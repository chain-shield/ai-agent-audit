# 2025 09 summer fi governance v2 chainshieldai/summer earn protocol - Findings Report
## Commit hash: 19703a7179a349b357f85b37b16307634b17f151

##Findings by Pattern


 **Derived From** : totalSupply() == sum(Transfer events where from == address(0)).amount - sum(Transfer events where to == address(0)).amount

[M-1]. transferFrom(to=address(0)) bypasses burnFrom authorization; any approved spender can burn victim xSUMR and lock unstake - INVALID

 **Derived From** : Guardian expiry not enforced in access modifiers (expired guardians keep privileges)

[M-3]. Expired guardians can bypass authorization in ProtocolAccessManaged.onlyGuardian/onlyGuardianOrGovernor and still invoke emergency actions - *VALID MEDIUM*

 **Derived From** : Unstake forwards released tokens from escrow balance, causing DoS if wallet pays beneficiary

[M-13]. Permissionless release causes permanent DoS in SummerVestingWalletsEscrow._unstakeFromFactory by draining escrow accounting - *VALID MEDIUM*




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


## Suggested Mitigation
Do not transfer releasedWhileStaked from the escrow unless the escrow actually holds those tokens. Robust options:
- Minimal fix: Replace the unconditional transfer with a bounded payout to avoid revert: amountToPay = min(releasedWhileStaked, SUMMER_TOKEN.balanceOf(address(this))); if (amountToPay > 0) transfer it. This prevents DoS even if the escrow is unfunded.
- Preferred fix: Stop paying from the escrow entirely. On unstake, do not attempt SUMR transfers from the escrow. Instead, rely on the vesting wallet’s own release() to deliver vested tokens to its beneficiary (typically the user). Optionally, call vestingWallet.release(SUMMER_TOKEN) before or after transferring ownership (release is permissionless) to flush any remaining vested tokens directly to the beneficiary.
- If the intended design requires released tokens to accrue to the escrow during staking, enforce that assumption: at stake time validate the vesting wallet beneficiary equals the escrow; on unstake, call release(SUMMER_TOKEN) while the escrow is still owner to pull the tokens, then forward escrow-held balance to the user. Finally, transfer ownership back. In all cases, never make unstake depend on the escrow having a positive SUMR balance.



