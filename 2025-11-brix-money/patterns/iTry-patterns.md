## Verified Patterns Found: 19

## Verified Patterns Found in following Categories:

- StorageCollisionOrSelectorClash
- UnboundedLoops
- AccessControlOrAuthByPass
- StandardViolation
- UpgradeAuthBypass
- BeaconOrFactoryAuthorityDrift
- PermitFrontRun
- AccountingInvariantViolation
- UnsafeAssembyTypeCasts
- ConfigFootgun



## Summary of Patterns

ConfigFootgun: `rescueTokens` permanently reverts for native token in `WHITELIST_ENABLED` state

Infinite loop in batch role management due to uint8 overflow

Missing UUPS Implementation Invalidator Upgradeability

Admin Self-DoS on iTry Rescue in Whitelist Mode

Potential for multiple minters violates strict backing invariant

renounceRole disabled preventing compromised role self-revocation

Missing UUPS implementation locks protocol upgradeability

Permit signature front-running causes DoS

Batch Operation Failure via Loop Iterator Overflow

iTry contract missing UUPS upgradeability implementation

Missing storage gap in SingleAdminAccessControlUpgradeable limits future upgrades

Missing UUPS Implementation Prevents Upgrades

Missing UUPSUpgradeable inheritance and authorization prevents upgrades

Missing storage gap in SingleAdminAccessControlUpgradeable risks storage collision

Admin Recovery Functions Blocked by FULLY_DISABLED State

Burn Functionality Disabled for Whitelisted Users

Blacklisted Minter can still mint tokens

AccessControlOrAuthByPass: Whitelist bypass during Minting and Redistribution

Blacklisted Minter role can still mint and redeem tokens

## Patterns



 ### Issue Type: ConfigFootgun

 ### Relevant Function/Location: iTry.rescueTokens

 ### Title
ConfigFootgun: `rescueTokens` permanently reverts for native token in `WHITELIST_ENABLED` state
 ### Description/Code Snippet
The `rescueTokens` function allows the admin to recover tokens sent to the contract by mistake. When rescuing the `iTry` token itself, it calls `transfer`. If `transferState` is set to `WHITELIST_ENABLED`, `_beforeTokenTransfer` requires `msg.sender`, `from`, and `to` to have the `WHITELISTED_ROLE`. Since `msg.sender` and `from` are `address(this)` (the contract itself) during a self-transfer, the rescue will fail unless the contract address is explicitly whitelisted, which is non-standard configuration. This creates a state where funds are unrecoverable.
 ### Static Signals
safeTransfer(to, amount), transferState == WHITELIST_ENABLED, hasRole(WHITELISTED_ROLE, msg.sender)
 ### Assets at Risk
iTry tokens held by contract
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: iTry.addBlacklistAddress

 ### Title
Infinite loop in batch role management due to uint8 overflow
 ### Description/Code Snippet
The functions `addBlacklistAddress`, `removeBlacklistAddress`, `addWhitelistAddress`, and `removeWhitelistAddress` declare the loop counter `i` as a `uint8`. If the `users` array length is 256 or greater, `i` will overflow from 255 to 0, causing the condition `i < users.length` to remain true indefinitely. This results in an infinite loop that consumes all gas and reverts, preventing the batch processing of more than 255 addresses.
 ### Static Signals
for (uint8 i = 0; i < users.length; i++)
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: UpgradeAuthBypass

 ### Relevant Function/Location: iTry.class definition

 ### Title
Missing UUPS Implementation Invalidator Upgradeability
 ### Description/Code Snippet
The documentation states iTRY is UUPS-upgradeable, but the `iTry` contract does not inherit `UUPSUpgradeable` nor implement the `_authorizeUpgrade` function required for UUPS proxies. This discrepancy means any attempt to upgrade the contract via a UUPS proxy will revert (missing function selector), effectively making the contract immutable and unable to accept security fixes or improvements.
 ### Static Signals
Missing UUPSUpgradeable inheritance, Missing _authorizeUpgrade function, Contract claimed to be upgradeable
 ### Assets at Risk
Protocol Upgradeability
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: iTry.rescueTokens

 ### Title
Admin Self-DoS on iTry Rescue in Whitelist Mode
 ### Description/Code Snippet
In `TransferState.WHITELIST_ENABLED`, the `_beforeTokenTransfer` hook requires the sender to have `WHITELISTED_ROLE`. When `rescueTokens` is called to rescue `iTry` tokens held by the contract itself, the sender is `address(this)`. Since `address(this)` is not whitelisted by default during initialization, this rescue operation will revert, trapping funds until the admin explicitly whitelists the contract address.
 ### Static Signals
_beforeTokenTransfer checks hasRole(WHITELISTED_ROLE, msg.sender), rescueTokens calls transfer from address(this)
 ### Assets at Risk
Stuck iTry tokens
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: ConfigFootgun

 ### Relevant Function/Location: iTry.addMinter

 ### Title
Potential for multiple minters violates strict backing invariant
 ### Description/Code Snippet
The `addMinter` function allows the admin to grant the `MINTER_CONTRACT` role to additional addresses without removing existing ones. The system design and comments imply a single minter (the `iTryIssuer`) is responsible for maintaining 1:1 backing with DLF. If a second minter is added (accidentally or for testing) and not removed, it can mint iTRY tokens that are not tracked by the Issuer's backing accounting, breaking the solvency invariant.
 ### Static Signals
_grantRole(MINTER_CONTRACT, minterContract), no limit on role members
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: iTry.renounceRole

 ### Title
renounceRole disabled preventing compromised role self-revocation
 ### Description/Code Snippet
The contract overrides `renounceRole` to unconditionally revert with `OperationNotAllowed`. While this prevents accidental admin resignation, it also prevents other privileged roles (like `MINTER_CONTRACT` or `BLACKLIST_MANAGER_ROLE`) from renouncing their own roles. If an operational role account is compromised, it cannot self-revoke to mitigate damage, violating the principle of defense-in-depth and deviating from the standard AccessControl specification.
 ### Static Signals
revert OperationNotAllowed()
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: BeaconOrFactoryAuthorityDrift

 ### Relevant Function/Location: iTry.iTry (contract definition)

 ### Title
Missing UUPS implementation locks protocol upgradeability
 ### Description/Code Snippet
The protocol documentation states `iTry` is UUPS-upgradeable. However, the contract does not inherit `UUPSUpgradeable` nor implement the `_authorizeUpgrade` function. If deployed behind a standard UUPS ERC1967Proxy, the proxy will delegate upgrade calls to the implementation, which will fail as the function does not exist. This results in the protocol being permanently immutable and unable to be upgraded, violating the intended design and preventing security patches.
 ### Static Signals
upgrade path controlled by different admin than core protocol, no codehash/impl allowlist; no immutability on critical addresses
 ### Assets at Risk
future funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: PermitFrontRun

 ### Relevant Function/Location: iTry.permit

 ### Title
Permit signature front-running causes DoS
 ### Description/Code Snippet
The contract inherits `ERC20PermitUpgradeable` which allows spending approval via signatures. These signatures are susceptible to front-running where an attacker observes a pending transaction with a valid `permit` signature and submits it first. The attacker's transaction consumes the nonce, causing the victim's transaction (often bundling `permit` and `transferFrom`) to revert. This allows griefing and Denial of Service against users interacting with the token.
 ### Static Signals
accepts expired or zero-deadline permits, nonces not incremented on failure
 ### Assets at Risk
user transactions (gas)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeAssembyTypeCasts

 ### Relevant Function/Location: iTry.addBlacklistAddress

 ### Title
Batch Operation Failure via Loop Iterator Overflow
 ### Description/Code Snippet
The `addBlacklistAddress`, `removeBlacklistAddress`, and `add/removeWhitelistAddress` functions iterate using a `uint8 i` counter. If the input array `users` has a length greater than 255, the `i++` operation will overflow and revert (Solidity 0.8+). This creates a denial of service for batch operations, which may be critical during an emergency response to blacklist many attacker addresses.
 ### Static Signals
for (uint8 i = 0; i < users.length; i++)
 ### Assets at Risk
Operational Security
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: BeaconOrFactoryAuthorityDrift

 ### Relevant Function/Location: iTry.N/A

 ### Title
iTry contract missing UUPS upgradeability implementation
 ### Description/Code Snippet
The protocol documentation states that the `iTry` token is UUPS-upgradeable. However, the `iTry.sol` contract does not inherit from `UUPSUpgradeable` nor does it implement the required `_authorizeUpgrade` function or `proxiableUUID`. If deployed behind a standard ERC1967/UUPS proxy, the proxy will either fail to deploy (due to missing UUID) or, if forced, will be permanently non-upgradeable because the implementation lacks the upgrade logic required by the UUPS pattern. This breaks the intended upgrade path for the core token.
 ### Static Signals
upgrade path controlled by different admin, no codehash/impl allowlist, missing _authorizeUpgrade
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: SingleAdminAccessControlUpgradeable.N/A

 ### Title
Missing storage gap in SingleAdminAccessControlUpgradeable limits future upgrades
 ### Description/Code Snippet
The `SingleAdminAccessControlUpgradeable` contract is intended for use in an upgradeable inheritance chain (as utilized by `iTry`). While it introduces new state variables (`_currentDefaultAdmin`, `_pendingDefaultAdmin`), it fails to declare a storage gap (e.g., `uint256[48] __gap`) at the end of the contract. Since `iTry` inherits this contract and immediately declares its own storage variable `transferState`, any future upgrade to `SingleAdminAccessControlUpgradeable` that adds a state variable will overwrite `transferState` in the `iTry` proxy storage, causing critical state corruption.
 ### Static Signals
inherits AccessControlUpgradeable, defines state variables, no __gap defined, inherited by upgradeable contract with storage
 ### Assets at Risk
transferState
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: UpgradeAuthBypass

 ### Relevant Function/Location: iTry.N/A

 ### Title
Missing UUPS Implementation Prevents Upgrades
 ### Description/Code Snippet
The protocol documentation and context explicitly state that the iTRY token is UUPS-upgradeable. However, the `iTry` contract fails to inherit from `UUPSUpgradeable` and does not implement the `_authorizeUpgrade` function required for UUPS proxies. In the UUPS pattern, the logic to upgrade the contract must reside in the implementation itself. Without this logic, the proxy cannot be upgraded to a new implementation, rendering the contract immutable and preventing the remediation of future vulnerabilities or the addition of features.
 ### Static Signals
no UUPSUpgradeable inheritance, missing _authorizeUpgrade function, missing upgradeTo function
 ### Assets at Risk
Protocol Upgradeability
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: iTry.N/A

 ### Title
Missing UUPSUpgradeable inheritance and authorization prevents upgrades
 ### Description/Code Snippet
The `iTry` contract is documented as UUPS-upgradeable and uses upgradeable libraries, but it fails to inherit `UUPSUpgradeable` or implement the `_authorizeUpgrade` function. If deployed behind a UUPS proxy (ERC1967Proxy), any attempt to upgrade the contract via `upgradeTo` or `upgradeToAndCall` will fail because the implementation logic is missing the required upgrade interface.
 ### Static Signals
contract iTry is ... (missing UUPSUpgradeable), no _authorizeUpgrade function
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StorageCollisionOrSelectorClash

 ### Relevant Function/Location: SingleAdminAccessControlUpgradeable.N/A

 ### Title
Missing storage gap in SingleAdminAccessControlUpgradeable risks storage collision
 ### Description/Code Snippet
The `SingleAdminAccessControlUpgradeable` abstract contract introduces new state variables (`_currentDefaultAdmin`, `_pendingDefaultAdmin`) but does not declare a storage gap (`__gap`) at the end. Since `iTry` inherits from this contract and adds its own state (`transferState`), any future upgrade to `SingleAdminAccessControlUpgradeable` that adds variables will shift the storage layout, colliding with and corrupting `iTry`'s state variables.
 ### Static Signals
manual state variables without __gap, upgradeable parent contract
 ### Assets at Risk
transferState
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: iTry._beforeTokenTransfer

 ### Title
Admin Recovery Functions Blocked by FULLY_DISABLED State
 ### Description/Code Snippet
The `_beforeTokenTransfer` function enforces the `transferState`. When `transferState` is set to `FULLY_DISABLED`, the function immediately reverts with `OperationNotAllowed()` without checking for privileged roles (`DEFAULT_ADMIN_ROLE` or `MINTER_CONTRACT`). This means that during a full emergency pause, the Admin cannot execute critical recovery functions such as `redistributeLockedAmount` (to seize hacker funds) or `mint`/`burn`. The Admin is forced to re-enable transfers (potentially to `WHITELIST_ENABLED`) to perform these actions, which exposes the protocol to race conditions if the blacklist is not perfectly synchronized.
 ### Static Signals
transferState == TransferState.FULLY_DISABLED, revert OperationNotAllowed(), no role check in disabled branch
 ### Assets at Risk
Protocol Solvency, Stolen Funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: iTry._beforeTokenTransfer

 ### Title
Burn Functionality Disabled for Whitelisted Users
 ### Description/Code Snippet
In `WHITELIST_ENABLED` mode, `_beforeTokenTransfer` requires the `to` address to be whitelisted. Standard `burn` operations (inherited from `ERC20Burnable`) set `to` to `address(0)`. Since `address(0)` is typically not whitelisted, whitelisted users are unable to burn their tokens directly. While redemption typically goes through the Issuer (Minter), this restriction breaks the standard `ERC20Burnable` behavior users might expect, potentially trapping funds if the Minter is unavailable.
 ### Static Signals
hasRole(WHITELISTED_ROLE, to), to == address(0), transferState == WHITELIST_ENABLED
 ### Assets at Risk
User Funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTry._beforeTokenTransfer

 ### Title
Blacklisted Minter can still mint tokens
 ### Description/Code Snippet
The `_beforeTokenTransfer` function enforces transfer restrictions. For minting, it checks `if (hasRole(MINTER_CONTRACT, msg.sender) ...)` before checking the generic blacklist restriction `!hasRole(BLACKLISTED_ROLE, msg.sender)`. This creates a loophole where a `MINTER_CONTRACT` that has been added to the blacklist (e.g., by the `BLACKLIST_MANAGER_ROLE` during an emergency/compromise) can still successfully mint tokens. The Blacklist Manager cannot stop a compromised minter, violating the invariant that blacklisted addresses are restricted.
 ### Static Signals
check precedence favors privileged role over blacklist
 ### Assets at Risk
iTry supply
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: iTry._beforeTokenTransfer

 ### Title
AccessControlOrAuthByPass: Whitelist bypass during Minting and Redistribution
 ### Description/Code Snippet
In `WHITELIST_ENABLED` mode, the protocol intends to restrict token possession/transfer to whitelisted users. However, the `_beforeTokenTransfer` hook's logic for minting (called by `MINTER_CONTRACT`) and redistribution (called by `DEFAULT_ADMIN_ROLE`) only checks that the recipient is NOT blacklisted (`!hasRole(BLACKLISTED_ROLE, to)`). It fails to check if the recipient is whitelisted. This allows minting tokens to non-whitelisted users, who will subsequently be unable to transfer them (creating trapped funds) and violating the whitelist invariant.
 ### Static Signals
!hasRole(BLACKLISTED_ROLE, to), missing hasRole(WHITELISTED_ROLE, to)
 ### Assets at Risk
User funds (trapped)
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: iTry._beforeTokenTransfer

 ### Title
Blacklisted Minter role can still mint and redeem tokens
 ### Description/Code Snippet
In `_beforeTokenTransfer`, the logic checks `hasRole(MINTER_CONTRACT, msg.sender)` before checking if `msg.sender` is blacklisted. The `MINTER_CONTRACT` branches (for minting and redeeming) do not verify that `msg.sender` is NOT blacklisted, unlike the 'normal case' branch. This means if the `BLACKLIST_MANAGER` blacklists a compromised Minter address, the Minter can still continue to mint and burn tokens, rendering the blacklist ineffective against the Minter.
 ### Static Signals
role check overrides blacklist check, missing !hasRole(BLACKLISTED_ROLE, msg.sender) in privileged branch
 ### Assets at Risk
iTry Token Supply, DLF Collateral
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole

