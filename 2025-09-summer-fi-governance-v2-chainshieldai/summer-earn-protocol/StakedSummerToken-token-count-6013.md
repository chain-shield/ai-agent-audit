
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC20Burnable} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import {ERC20Pausable} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol";
import {ERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import {ERC20Votes} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol";
import {Nonces} from "@openzeppelin/contracts/utils/Nonces.sol";
import {ProtocolAccessManaged} from "@summerfi/access-contracts/contracts/ProtocolAccessManaged.sol";
import {IStakedSummerToken} from "../interfaces/IStakedSummerToken.sol";

/**
 * @title StakedSummerToken (xSUMR)
 * @notice Non-transferable staked representation of SUMR used for governance and rewards accounting.
 * @dev Key properties:
 *      - Minting/Burning controlled by governance-authorized staking modules
 *      - Direct transfers disabled; only mint (from address(0)) and burn (to address(0)) allowed
 *      - Pausable by guardian/governor for emergency response
 *      - Integrates ERC20Permit and ERC20Votes for signatures and governance snapshots
 *
 * Access control model:
 *      - Governor can add/remove staking modules, which grants MINTER and BURNER roles
 *      - Only modules with MINTER_ROLE can mint
 *      - burnFrom requires either the token owner or an address with BURNER_ROLE plus standard allowance
 */
contract StakedSummerToken is
    IStakedSummerToken,
    ERC20Burnable,
    ERC20Pausable,
    ProtocolAccessManaged,
    AccessControl,
    ERC20Permit,
    ERC20Votes
{
    // ============ ROLES ============
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");

    // ============ CONSTRUCTOR ============
    constructor(
        address _protocolAccessManager
    )
        ERC20("StakedSummerToken", "xSUMR")
        ERC20Permit("StakedSummerToken")
        ProtocolAccessManaged(_protocolAccessManager)
    {}

    // ============ GOVERNANCE ============

    /// @inheritdoc IStakedSummerToken
    function addStakingModule(address _stakingModule) external onlyGovernor {
        if (_stakingModule == address(0)) {
            revert xSumr_InvalidStakingModule(
                "Staking module address cannot be zero"
            );
        }
        // Authorize staking module to participate in mint/burn flows
        _grantRole(MINTER_ROLE, _stakingModule);
        _grantRole(BURNER_ROLE, _stakingModule);

        emit StakingModuleAdded(_stakingModule);
    }

    /// @inheritdoc IStakedSummerToken
    function removeStakingModule(address _stakingModule) external onlyGovernor {
        // Fully deauthorize staking module by revoking both roles
        _revokeRole(MINTER_ROLE, _stakingModule);
        _revokeRole(BURNER_ROLE, _stakingModule);
        emit StakingModuleRemoved(_stakingModule);
    }

    /// @inheritdoc IStakedSummerToken
    function pause() external onlyGuardianOrGovernor {
        _pause();
    }

    /// @inheritdoc IStakedSummerToken
    function unpause() external onlyGuardianOrGovernor {
        _unpause();
    }

    // ============ MINT / BURN API ============

    /// @inheritdoc IStakedSummerToken
    function mint(address to, uint256 amount) external onlyRole(MINTER_ROLE) {
        // Only authorized staking modules are permitted to mint xSUMR
        _mint(to, amount);
    }

    /// @inheritdoc IStakedSummerToken
    function burn(
        uint256 amount
    ) public override(ERC20Burnable, IStakedSummerToken) {
        super.burn(amount);
    }

    /// @inheritdoc IStakedSummerToken
    function burnFrom(
        address from,
        uint256 amount
    ) public override(ERC20Burnable, IStakedSummerToken) {
        if (!_canBurnFrom(from, msg.sender)) {
            revert xSumr__NotAuthorized();
        }
        // Honor allowance semantics when `msg.sender != from` via ERC20Burnable
        super.burnFrom(from, amount);
    }

    // ============ ERC6372 / ERC20Votes ============
    /// @notice Returns the current clock in seconds, used by ERC20Votes for timestamp-based checkpoints.
    function clock() public view override returns (uint48) {
        return uint48(block.timestamp);
    }

    // solhint-disable-next-line func-name-mixedcase
    /// @notice Returns the clock mode string as required by ERC-6372.
    function CLOCK_MODE() public pure override returns (string memory) {
        return "mode=timestamp";
    }

    // The following functions are overrides required by Solidity.
    function _update(
        address from,
        address to,
        uint256 value
    ) internal override(ERC20, ERC20Pausable, ERC20Votes) {
        if (!_canTransfer(from, to)) {
            revert xSumr_TransferNotAllowed();
        }
        // Run pausable and votes hooks (checkpoints, etc.)
        super._update(from, to, value);
    }

    function nonces(
        address owner
    ) public view override(ERC20Permit, Nonces) returns (uint256) {
        return super.nonces(owner);
    }

    // ============ ROLE MANAGEMENT (GOVERNOR) ============

    /// @inheritdoc IStakedSummerToken
    function grantMinterRole(address _minter) external onlyGovernor {
        _grantRole(MINTER_ROLE, _minter);
    }

    /// @inheritdoc IStakedSummerToken
    function revokeMinterRole(address _minter) external onlyGovernor {
        _revokeRole(MINTER_ROLE, _minter);
    }

    /**
     * @dev Overrides the grantRole function from AccessControl to disable direct role granting.
     * @notice This function always reverts with a DirectGrantIsDisabled error.
     */
    function grantRole(bytes32, address) public view override {
        revert DirectGrantIsDisabled(msg.sender);
    }

    /**
     * @dev Overrides the revokeRole function from AccessControl to disable direct role revoking.
     * @notice This function always reverts with a DirectRevokeIsDisabled error.
     */
    function revokeRole(bytes32, address) public view override {
        revert DirectRevokeIsDisabled(msg.sender);
    }

    // ============ INTERNAL HELPERS ============

    /**
     * @dev Only allow mint (from == address(0)) and burn (to == address(0)) movements. Block user-to-user transfers.
     * @notice All staking module interactions are based on `mint()` and `burnFrom()`;
     * transfers between users are disallowed.
     * @param from The address to transfer tokens from.
     * @param to The address to transfer tokens to.
     * @return bool True if the transfer is allowed, false otherwise.
     */
    function _canTransfer(
        address from,
        address to
    ) internal pure returns (bool) {
        return from == address(0) || to == address(0);
    }

    /**
     * @notice Allows `burnFrom` only if `spender` burns its own tokens or holds `BURNER_ROLE`.
     * @dev Even with `BURNER_ROLE`, standard ERC20 allowance rules apply.
     * @param from The address to burn tokens from.
     * @param spender The address to check for `BURNER_ROLE`.
     * @return bool True if the burn is allowed, false otherwise.
     */
    function _canBurnFrom(
        address from,
        address spender
    ) internal view returns (bool) {
        return spender == from || hasRole(BURNER_ROLE, spender);
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: PARENT AND CALLED CONTRACTS
// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {IAccessControlErrors} from "../interfaces/IAccessControlErrors.sol";
import {ContractSpecificRoles, IProtocolAccessManager} from "../interfaces/IProtocolAccessManager.sol";
import {ProtocolAccessManager} from "./ProtocolAccessManager.sol";

import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";

/**
 * @title ProtocolAccessManaged
 * @notice This contract provides role-based access control functionality for protocol contracts
 * by interfacing with a central ProtocolAccessManager.
 *
 * @dev This contract is meant to be inherited by other protocol contracts that need
 * role-based access control. It provides modifiers and utilities to check various roles.
 *
 * The contract supports several key roles through modifiers:
 * 1. GOVERNOR_ROLE: System-wide administrators
 * 2. KEEPER_ROLE: Routine maintenance operators (contract-specific)
 * 3. SUPER_KEEPER_ROLE: Advanced maintenance operators (global)
 * 4. CURATOR_ROLE: Fleet-specific managers
 * 5. GUARDIAN_ROLE: Emergency response operators
 * 6. DECAY_CONTROLLER_ROLE: Specific role for decay management
 * 7. ADMIRALS_QUARTERS_ROLE: Specific role for admirals quarters bundler contract
 *
 * Usage:
 * - Inherit from this contract to gain access to role-checking modifiers
 * - Use modifiers like onlyGovernor, onlyKeeper, etc. to protect functions
 * - Access the internal _accessManager to perform custom role checks
 *
 * Security Considerations:
 * - The contract validates the access manager address during construction
 * - All role checks are performed against the immutable access manager instance
 * - Contract-specific roles are generated using the contract's address to prevent conflicts
 */
contract ProtocolAccessManaged is IAccessControlErrors, Context {
    /*//////////////////////////////////////////////////////////////
                                CONSTANTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Role identifier for protocol governors - highest privilege level with admin capabilities
    bytes32 public constant GOVERNOR_ROLE = keccak256("GOVERNOR_ROLE");

    /// @notice Role identifier for super keepers who can globally perform fleet maintanence roles
    bytes32 public constant SUPER_KEEPER_ROLE = keccak256("SUPER_KEEPER_ROLE");

    /**
     * @notice Role identifier for protocol guardians
     * @dev Guardians have emergency powers across multiple protocol components:
     * - Can pause/unpause Fleet operations for security
     * - Can pause/unpause TipJar operations
     * - Can cancel governance proposals on SummerGovernor even if they don't meet normal cancellation requirements
     * - Can cancel TipJar proposals
     *
     * The guardian role serves as an emergency backstop to protect the protocol, but with less
     * privilege than governors.
     */
    bytes32 public constant GUARDIAN_ROLE = keccak256("GUARDIAN_ROLE");

    /**
     * @notice Role identifier for decay controller
     * @dev This role allows the decay controller to manage the decay of user voting power
     */
    bytes32 public constant DECAY_CONTROLLER_ROLE =
        keccak256("DECAY_CONTROLLER_ROLE");

    /**
     * @notice Role identifier for admirals quarters bundler contract
     * @dev This role allows Admirals Quarters to unstake and withdraw assets from fleets, on behalf of users
     * @dev Withdrawn tokens go straight to users wallet, lowering the risk of manipulation if the role is compromised
     */
    bytes32 public constant ADMIRALS_QUARTERS_ROLE =
        keccak256("ADMIRALS_QUARTERS_ROLE");

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice The ProtocolAccessManager instance used for access control
    ProtocolAccessManager internal immutable _accessManager;

    /*//////////////////////////////////////////////////////////////
                                CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initializes the ProtocolAccessManaged contract
     * @param accessManager Address of the ProtocolAccessManager contract
     * @dev Validates the provided accessManager address and initializes the _accessManager
     */
    constructor(address accessManager) {
        if (accessManager == address(0)) {
            revert InvalidAccessManagerAddress(address(0));
        }

        if (
            !IERC165(accessManager).supportsInterface(
                type(IProtocolAccessManager).interfaceId
            )
        ) {
            revert InvalidAccessManagerAddress(accessManager);
        }

        _accessManager = ProtocolAccessManager(accessManager);
    }

    /*//////////////////////////////////////////////////////////////
                                MODIFIERS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Modifier to restrict access to governors only
     *
     * @dev Modifier to check that the caller has the Governor role
     * @custom:internal-logic
     * - Checks if the caller has the GOVERNOR_ROLE in the access manager
     * @custom:effects
     * - Reverts if the caller doesn't have the GOVERNOR_ROLE
     * - Allows the function to proceed if the caller has the role
     * @custom:security-considerations
     * - Ensures that only authorized governors can access critical functions
     * - Relies on the correct setup of the access manager
     */
    modifier onlyGovernor() {
        if (!_accessManager.hasRole(GOVERNOR_ROLE, msg.sender)) {
            revert CallerIsNotGovernor(msg.sender);
        }
        _;
    }

    /**
     * @notice Modifier to restrict access to keepers only
     * @dev Modifier to check that the caller has the Keeper role
     * @custom:internal-logic
     * - Checks if the caller has either the contract-specific KEEPER_ROLE or the SUPER_KEEPER_ROLE
     * @custom:effects
     * - Reverts if the caller doesn't have either of the required roles
     * - Allows the function to proceed if the caller has one of the roles
     * @custom:security-considerations
     * - Ensures that only authorized keepers can access maintenance functions
     * - Allows for both contract-specific and super keepers
     * @custom:gas-considerations
     * - Performs two role checks, which may impact gas usage
     */
    modifier onlyKeeper() {
        if (
            !_accessManager.hasRole(
                generateRole(ContractSpecificRoles.KEEPER_ROLE, address(this)),
                msg.sender
            ) && !_accessManager.hasRole(SUPER_KEEPER_ROLE, msg.sender)
        ) {
            revert CallerIsNotKeeper(msg.sender);
        }
        _;
    }

    /**
     * @notice Modifier to restrict access to super keepers only
     * @dev Modifier to check that the caller has the Super Keeper role
     * @custom:internal-logic
     * - Checks if the caller has the SUPER_KEEPER_ROLE in the access manager
     * @custom:effects
     * - Reverts if the caller doesn't have the SUPER_KEEPER_ROLE
     * - Allows the function to proceed if the caller has the role
     * @custom:security-considerations
     * - Ensures that only authorized super keepers can access advanced maintenance functions
     * - Relies on the correct setup of the access manager
     */
    modifier onlySuperKeeper() {
        if (!_accessManager.hasRole(SUPER_KEEPER_ROLE, msg.sender)) {
            revert CallerIsNotSuperKeeper(msg.sender);
        }
        _;
    }

    /**
     * @notice Modifier to restrict access to curators only
     * @param fleetAddress The address of the fleet to check the curator role for
     * @dev Checks if the caller has the contract-specific CURATOR_ROLE
     */
    modifier onlyCurator(address fleetAddress) {
        if (
            fleetAddress == address(0) ||
            !_accessManager.hasRole(
                generateRole(ContractSpecificRoles.CURATOR_ROLE, fleetAddress),
                msg.sender
            )
        ) {
            revert CallerIsNotCurator(msg.sender);
        }
        _;
    }

    /**
     * @notice Modifier to restrict access to guardians only
     * @dev Modifier to check that the caller has the Guardian role
     * @custom:internal-logic
     * - Checks if the caller has the GUARDIAN_ROLE in the access manager
     * @custom:effects
     * - Reverts if the caller doesn't have the GUARDIAN_ROLE
     * - Allows the function to proceed if the caller has the role
     * @custom:security-considerations
     * - Ensures that only authorized guardians can access emergency functions
     * - Relies on the correct setup of the access manager
     */
    modifier onlyGuardian() {
        if (!_accessManager.hasRole(GUARDIAN_ROLE, msg.sender)) {
            revert CallerIsNotGuardian(msg.sender);
        }
        _;
    }

    /**
     * @notice Modifier to restrict access to either guardians or governors
     * @dev Modifier to check that the caller has either the Guardian or Governor role
     * @custom:internal-logic
     * - Checks if the caller has either the GUARDIAN_ROLE or the GOVERNOR_ROLE
     * @custom:effects
     * - Reverts if the caller doesn't have either of the required roles
     * - Allows the function to proceed if the caller has one of the roles
     * @custom:security-considerations
     * - Ensures that only authorized guardians or governors can access certain functions
     * - Provides flexibility for functions that can be accessed by either role
     * @custom:gas-considerations
     * - Performs two role checks, which may impact gas usage
     */
    modifier onlyGuardianOrGovernor() {
        if (
            !_accessManager.hasRole(GUARDIAN_ROLE, msg.sender) &&
            !_accessManager.hasRole(GOVERNOR_ROLE, msg.sender)
        ) {
            revert CallerIsNotGuardianOrGovernor(msg.sender);
        }
        _;
    }

    /**
     * @notice Modifier to restrict access to decay controllers only
     */
    modifier onlyDecayController() {
        if (!_accessManager.hasRole(DECAY_CONTROLLER_ROLE, msg.sender)) {
            revert CallerIsNotDecayController(msg.sender);
        }
        _;
    }

    /**
     * @notice Modifier to restrict access to foundation only
     * @dev Modifier to check that the caller has the Foundation role
     * @custom:security-considerations
     * - Ensures that only the Foundation can access vesting and related functions
     * - Relies on the correct setup of the access manager
     */
    modifier onlyFoundation() {
        if (
            !_accessManager.hasRole(
                _accessManager.FOUNDATION_ROLE(),
                msg.sender
            )
        ) {
            revert CallerIsNotFoundation(msg.sender);
        }
        _;
    }

    /*//////////////////////////////////////////////////////////////
                            PUBLIC FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Generates a role identifier for a specific contract and role
     * @param roleName The name of the role
     * @param roleTargetContract The address of the contract the role is for
     * @return The generated role identifier
     * @dev This function is used to create unique role identifiers for contract-specific roles
     */
    function generateRole(
        ContractSpecificRoles roleName,
        address roleTargetContract
    ) public pure returns (bytes32) {
        return keccak256(abi.encodePacked(roleName, roleTargetContract));
    }

    /**
     * @notice Checks if an account has the Admirals Quarters role
     * @param account The address to check
     * @return bool True if the account has the Admirals Quarters role
     */
    function hasAdmiralsQuartersRole(
        address account
    ) public view returns (bool) {
        return _accessManager.hasRole(ADMIRALS_QUARTERS_ROLE, account);
    }

    /*//////////////////////////////////////////////////////////////
                            INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Helper function to check if an address has the Governor role
     * @param account The address to check
     * @return bool True if the address has the Governor role
     */
    function _isGovernor(address account) internal view returns (bool) {
        return _accessManager.hasRole(GOVERNOR_ROLE, account);
    }

    function _isDecayController(address account) internal view returns (bool) {
        return _accessManager.hasRole(DECAY_CONTROLLER_ROLE, account);
    }

    /**
     * @notice Helper function to check if an address has the Foundation role
     * @param account The address to check
     * @return bool True if the address has the Foundation role
     */
    function _isFoundation(address account) internal view returns (bool) {
        return
            _accessManager.hasRole(_accessManager.FOUNDATION_ROLE(), account);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title IStakedSummerToken
 * @notice Interface for xSUMR, the non-transferable staked representation of SUMR used for governance and rewards.
 * @dev xSUMR is mint/burn controlled by approved staking modules. Direct transfers between users are disabled; only
 *      minting (from address(0)) and burning (to address(0)) are permitted movements. Implementations SHOULD enforce
 *      role-based access control for minting and restricted burning, and MAY expose pause controls via governance.
 *
 * Rationale and Invariants:
 * - Transfer path is intentionally restricted to mint/burn to avoid bypassing staking logic and snapshots.
 * - Multiple staking modules may be authorized concurrently; governance manages their lifecycle.
 * - `burnFrom` authorization requires either the owner or an address with BURNER role; allowance rules still apply.
 */
interface IStakedSummerToken is IERC20 {
    // =============================
    //            EVENTS
    // =============================

    /**
     * @notice Emitted when a staking module is granted mint/burn permissions on xSUMR.
     * @param stakingModule Address of the staking module added.
     */
    event StakingModuleAdded(address indexed stakingModule);

    /**
     * @notice Emitted when a staking module has its mint/burn permissions revoked on xSUMR.
     * @param stakingModule Address of the staking module removed.
     */
    event StakingModuleRemoved(address indexed stakingModule);

    // =============================
    //            ERRORS
    // =============================

    /**
     * @notice Thrown when a zero address or otherwise invalid staking module is supplied.
     * @param message Details about the invalid staking module input.
     */
    error xSumr_InvalidStakingModule(string message);

    /**
     * @notice Thrown when a caller attempts an operation without the required authorization.
     */
    error xSumr__NotAuthorized();

    /**
     * @notice Thrown when a forbidden token transfer is attempted (only mint/burn flows are allowed).
     */
    error xSumr_TransferNotAllowed();

    // =============================
    //          GOVERNANCE
    // =============================

    /**
     * @notice Adds a staking module with mint and burn permissions.
     * @dev Access restricted to governance in the implementing contract.
     * @param _stakingModule The staking module to authorize.
     *        Must be a non-zero address and expected to integrate with staking flows.
     * @custom:reverts xSumr_InvalidStakingModule If `_stakingModule` is the zero address.
     * @custom:emits StakingModuleAdded Emitted upon successful addition.
     */
    function addStakingModule(address _stakingModule) external;

    /**
     * @notice Removes a staking module with mint and burn permissions.
     * @dev Access restricted to governance in the implementing contract.
     * @param _stakingModule The staking module to remove.
     *        Must be a non-zero address and previously authorized.
     * @custom:reverts xSumr_InvalidStakingModule If `_stakingModule` is the zero address.
     * @custom:emits StakingModuleRemoved Emitted upon successful removal.
     */
    function removeStakingModule(address _stakingModule) external;

    /**
     * @notice Grants MINTER_ROLE to a specified address. Governor-only.
     * @dev Intended for emergency recovery scenarios (e.g., user burned xSUMR prematurely
     *      and needs redemption support). Normal mint authorization should be managed via
     *      `addStakingModule`.
     * @param _minter Address to grant MINTER_ROLE to.
     */
    function grantMinterRole(address _minter) external;

    /**
     * @notice Revokes MINTER_ROLE from a specified address. Governor-only.
     * @dev Intended for emergency recovery scenarios. Normal flow uses `removeStakingModule` for module revocation.
     * @param _minter Address to revoke MINTER_ROLE from.
     */
    function revokeMinterRole(address _minter) external;

    /**
     * @notice  Pauses token operations that honor pausability (e.g., burns).
     * @dev Callable by guardian or governor. While paused, `mint`, `burn` and `burnFrom` are blocked by ERC20Pausable.
     */
    function pause() external;

    /**
     * @notice Unpauses token operations.
     * @dev Callable by guardian or governor. Restores normal `mint`/`burn`/`burnFrom` behavior.
     */
    function unpause() external;

    // =============================
    //        MINT / BURN API
    // =============================

    /**
     * @notice Mints xSUMR to a recipient address.
     * @dev Access is expected to be restricted to authorized staking modules.
     * @param _to Recipient address for newly minted xSUMR.
     * @param _amount Amount of xSUMR to mint (1:1 to staked SUMR backing in typical flows).
     */
    function mint(address _to, uint256 _amount) external;

    /**
     * @notice Burns caller's xSUMR balance.
     * @dev Access: Token owner. Used for self-burn flows like unstaking where the owner directly initiates the burn.
     * @param _amount Amount of xSUMR to burn from the caller's balance.
     */
    function burn(uint256 _amount) external;

    /**
     * @notice Burns xSUMR from a specified address using module authorization and/or allowance
     * @dev Implementations SHOULD allow either the token owner or an authorized burner module to execute.
     * @param _from Address from which tokens will be burned.
     * @param _amount Amount of xSUMR to burn.
     * @dev Implementations SHOULD enforce either owner self-burn or burner-role authorization plus allowance.
     */
    function burnFrom(address _from, uint256 _amount) external;
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.1) (utils/Context.sol)

pragma solidity ^0.8.20;

/**
 * @dev Provides information about the current execution context, including the
 * sender of the transaction and its data. While these are generally available
 * via msg.sender and msg.data, they should not be accessed in such a direct
 * manner, since when dealing with meta-transactions the account sending and
 * paying for execution may not be the actual sender (as far as an application
 * is concerned).
 *
 * This contract is only required for intermediate, library-like contracts.
 */
abstract contract Context {
    function _msgSender() internal view virtual returns (address) {
        return msg.sender;
    }

    function _msgData() internal view virtual returns (bytes calldata) {
        return msg.data;
    }

    function _contextSuffixLength() internal view virtual returns (uint256) {
        return 0;
    }
}


END OF SUPPORTING CONTRACTS AND INTERFACES
