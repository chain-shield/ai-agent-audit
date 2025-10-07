
## *MAIN TARGET CONTRACT* TO REVIEW

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

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

/**
 * @dev Dynamic roles are roles that are not hardcoded in the contract but are defined by the protocol
 * Members of this enum are treated as prefixes to the role generated using prefix and target contract address
 * e.g generateRole(ContractSpecificRoles.CURATOR_ROLE, address(this)) for FleetCommander, to generate the CURATOR_ROLE
 * for the curator of the FleetCommander contract
 */
enum ContractSpecificRoles {
    CURATOR_ROLE,
    KEEPER_ROLE,
    COMMANDER_ROLE
}

/**
 * @title IProtocolAccessManager
 * @notice Defines system roles and provides role based remote-access control for
 *         contracts that inherit from ProtocolAccessManaged contract
 */
interface IProtocolAccessManager {
    /**
     * @notice Grants the Governor role to a given account
     *
     * @param account The account to which the Governor role will be granted
     */
    function grantGovernorRole(address account) external;

    /**
     * @notice Revokes the Governor role from a given account
     *
     * @param account The account from which the Governor role will be revoked
     */
    function revokeGovernorRole(address account) external;

    /**
     * @notice Grants the Super Keeper role to a given account
     *
     * @param account The account to which the Super Keeper role will be granted
     */
    function grantSuperKeeperRole(address account) external;

    /**
     * @notice Revokes the Super Keeper role from a given account
     *
     * @param account The account from which the Super Keeper role will be revoked
     */
    function revokeSuperKeeperRole(address account) external;

    /**
     * @dev Generates a unique role identifier based on the role name and target contract address
     * @param roleName The name of the role (from ContractSpecificRoles enum)
     * @param roleTargetContract The address of the contract the role is for
     * @return bytes32 The generated role identifier
     * @custom:internal-logic
     * - Combines the roleName and roleTargetContract using abi.encodePacked
     * - Applies keccak256 hash function to generate a unique bytes32 identifier
     * @custom:effects
     * - Does not modify any state, pure function
     * @custom:security-considerations
     * - Ensures unique role identifiers for different contracts
     * - Relies on the uniqueness of contract addresses and role names
     */
    function generateRole(
        ContractSpecificRoles roleName,
        address roleTargetContract
    ) external pure returns (bytes32);

    /**
     * @notice Grants a contract specific role to a given account
     * @param roleName The name of the role to grant
     * @param roleTargetContract The address of the contract to grant the role for
     * @param account The account to which the role will be granted
     */
    function grantContractSpecificRole(
        ContractSpecificRoles roleName,
        address roleTargetContract,
        address account
    ) external;

    /**
     * @notice Revokes a contract specific role from a given account
     * @param roleName The name of the role to revoke
     * @param roleTargetContract The address of the contract to revoke the role for
     * @param account The account from which the role will be revoked
     */
    function revokeContractSpecificRole(
        ContractSpecificRoles roleName,
        address roleTargetContract,
        address account
    ) external;

    /**
     * @notice Grants the Curator role to a given account
     * @param fleetCommanderAddress The address of the fleet commander to grant the role for
     * @param account The account to which the role will be granted
     */
    function grantCuratorRole(
        address fleetCommanderAddress,
        address account
    ) external;

    /**
     * @notice Revokes the Curator role from a given account
     * @param fleetCommanderAddress The address of the fleet commander to revoke the role for
     * @param account The account from which the role will be revoked
     */
    function revokeCuratorRole(
        address fleetCommanderAddress,
        address account
    ) external;

    /**
     * @notice Grants the Keeper role to a given account
     * @param fleetCommanderAddress The address of the fleet commander to grant the role for
     * @param account The account to which the role will be granted
     */
    function grantKeeperRole(
        address fleetCommanderAddress,
        address account
    ) external;

    /**
     * @notice Revokes the Keeper role from a given account
     * @param fleetCommanderAddress The address of the fleet commander to revoke the role for
     * @param account The account from which the role will be revoked
     */
    function revokeKeeperRole(
        address fleetCommanderAddress,
        address account
    ) external;

    /**
     * @notice Grants the Commander role for a specific Ark
     * @param arkAddress Address of the Ark contract
     * @param account Address to grant the Commander role to
     */
    function grantCommanderRole(address arkAddress, address account) external;

    /**
     * @notice Revokes the Commander role for a specific Ark
     * @param arkAddress Address of the Ark contract
     * @param account Address to revoke the Commander role from
     */
    function revokeCommanderRole(address arkAddress, address account) external;

    /**
     * @notice Revokes a contract specific role from the caller
     * @param roleName The name of the role to revoke
     * @param roleTargetContract The address of the contract to revoke the role for
     */
    function selfRevokeContractSpecificRole(
        ContractSpecificRoles roleName,
        address roleTargetContract
    ) external;

    /**
     * @notice Grants the Guardian role to a given account
     *
     * @param account The account to which the Guardian role will be granted
     */
    function grantGuardianRole(address account) external;

    /**
     * @notice Revokes the Guardian role from a given account
     *
     * @param account The account from which the Guardian role will be revoked
     */
    function revokeGuardianRole(address account) external;

    /**
     * @notice Grants the Decay Controller role to a given account
     * @param account The account to which the Decay Controller role will be granted
     */
    function grantDecayControllerRole(address account) external;

    /**
     * @notice Revokes the Decay Controller role from a given account
     * @param account The account from which the Decay Controller role will be revoked
     */
    function revokeDecayControllerRole(address account) external;

    /**
     * @notice Grants the ADMIRALS_QUARTERS_ROLE to an address
     * @param account The address to grant the role to
     */
    function grantAdmiralsQuartersRole(address account) external;

    /**
     * @notice Revokes the ADMIRALS_QUARTERS_ROLE from an address
     * @param account The address to revoke the role from
     */
    function revokeAdmiralsQuartersRole(address account) external;

    /*//////////////////////////////////////////////////////////////
                            ROLE CONSTANTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Role identifier for the Governor role
    function GOVERNOR_ROLE() external pure returns (bytes32);

    /// @notice Role identifier for the Guardian role
    function GUARDIAN_ROLE() external pure returns (bytes32);

    /// @notice Role identifier for the Super Keeper role
    function SUPER_KEEPER_ROLE() external pure returns (bytes32);

    /// @notice Role identifier for the Decay Controller role
    function DECAY_CONTROLLER_ROLE() external pure returns (bytes32);

    /// @notice Role identifier for the Admirals Quarters role
    function ADMIRALS_QUARTERS_ROLE() external pure returns (bytes32);

    /// @notice Role identifier for the Foundation, responsible for managing vesting wallets and related operations
    function FOUNDATION_ROLE() external pure returns (bytes32);

    /**
     * @notice Checks if an account has a specific role
     * @param role The role identifier to check
     * @param account The account to check the role for
     * @return bool True if the account has the role, false otherwise
     */
    function hasRole(
        bytes32 role,
        address account
    ) external view returns (bool);

    /*//////////////////////////////////////////////////////////////
                                EVENTS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Emitted when a guardian's expiration is set
     * @param account The address of the guardian
     * @param expiration The timestamp until which the guardian powers are valid
     */
    event GuardianExpirationSet(address indexed account, uint256 expiration);

    /*//////////////////////////////////////////////////////////////
                            GUARDIAN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Checks if an account is an active guardian (has role and not expired)
     * @param account Address to check
     * @return bool True if account is an active guardian
     */
    function isActiveGuardian(address account) external view returns (bool);

    /**
     * @notice Sets the expiration timestamp for a guardian
     * @param account Guardian address
     * @param expiration Timestamp when guardian powers expire
     */
    function setGuardianExpiration(
        address account,
        uint256 expiration
    ) external;

    /**
     * @notice Gets the expiration timestamp for a guardian
     * @param account Guardian address
     * @return uint256 Timestamp when guardian powers expire
     */
    function guardianExpirations(
        address account
    ) external view returns (uint256);

    /**
     * @notice Gets the expiration timestamp for a guardian
     * @param account Guardian address
     * @return expiration Timestamp when guardian powers expire
     */
    function getGuardianExpiration(
        address account
    ) external view returns (uint256 expiration);

    /**
     * @notice Emitted when an invalid guardian expiry period is set
     * @param expiryPeriod The expiry period that was set
     * @param minExpiryPeriod The minimum allowed expiry period
     * @param maxExpiryPeriod The maximum allowed expiry period
     */
    error InvalidGuardianExpiryPeriod(
        uint256 expiryPeriod,
        uint256 minExpiryPeriod,
        uint256 maxExpiryPeriod
    );

    /**
     * @notice Grants the Foundation role to a given account. The Foundation is responsible for
     * managing vesting wallets and related operations.
     * @param account The account to which the Foundation role will be granted
     */
    function grantFoundationRole(address account) external;

    /**
     * @notice Revokes the Foundation role from a given account
     * @param account The account from which the Foundation role will be revoked
     */
    function revokeFoundationRole(address account) external;
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {ContractSpecificRoles, IProtocolAccessManager} from "../interfaces/IProtocolAccessManager.sol";
import {LimitedAccessControl} from "./LimitedAccessControl.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

/**
 * @title ProtocolAccessManager
 * @notice This contract is the central authority for access control within the protocol.
 * It defines and manages various roles that govern different aspects of the system.
 *
 * @dev This contract extends LimitedAccessControl, which restricts direct role management.
 * Roles are typically assigned during deployment or through governance proposals.
 *
 * The contract defines four main roles:
 * 1. GOVERNOR_ROLE: System-wide administrators
 * 2. KEEPER_ROLE: Routine maintenance operators
 * 3. SUPER_KEEPER_ROLE: Advanced maintenance operators
 * 4. COMMANDER_ROLE: Managers of specific protocol components (Arks)
 * 5. ADMIRALS_QUARTERS_ROLE: Specific role for admirals quarters bundler contract
 * Role Hierarchy and Management:
 * - The GOVERNOR_ROLE is at the top of the hierarchy and can manage all other roles.
 * - Other roles cannot manage roles directly due to LimitedAccessControl restrictions.
 * - Role assignments are typically done through governance proposals or during initial setup.
 *
 * Usage in the System:
 * - Other contracts in the system inherit from ProtocolAccessManaged, which checks permissions
 *   against this ProtocolAccessManager.
 * - Critical functions in various contracts are protected by role-based modifiers
 *   (e.g., onlyGovernor, onlyKeeper, etc.) which query this contract for permissions.
 *
 * Security Considerations:
 * - The GOVERNOR_ROLE has significant power and should be managed carefully, potentially
 *   through a multi-sig wallet or governance contract.
 * - The SUPER_KEEPER_ROLE has elevated privileges and should be assigned judiciously.
 * - The COMMANDER_ROLE is not directly manageable through this contract but is used
 *   in other parts of the system for specific access control.
 */
contract ProtocolAccessManager is IProtocolAccessManager, LimitedAccessControl {
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

    /// @notice Minimum allowed guardian expiration period (7 days)
    uint256 public constant MIN_GUARDIAN_EXPIRY = 7 days;

    /// @notice Maximum allowed guardian expiration period (180 days)
    uint256 public constant MAX_GUARDIAN_EXPIRY = 180 days;

    /// @notice Role identifier for the Foundation which manages vesting wallets and related operations
    bytes32 public constant FOUNDATION_ROLE = keccak256("FOUNDATION_ROLE");

    /*//////////////////////////////////////////////////////////////
                                CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initializes the ProtocolAccessManager contract
     * @param governor Address of the initial governor
     * @dev Grants the governor address the GOVERNOR_ROLE
     */
    constructor(address governor) {
        _grantRole(GOVERNOR_ROLE, governor);
    }

    /**
     * @dev Modifier to check that the caller has the Governor role
     */
    modifier onlyGovernor() {
        if (!hasRole(GOVERNOR_ROLE, msg.sender)) {
            revert CallerIsNotGovernor(msg.sender);
        }
        _;
    }

    /*//////////////////////////////////////////////////////////////
                            PUBLIC FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Checks if the contract supports a given interface
     * @dev Overrides the supportsInterface function from AccessControl
     * @param interfaceId The interface identifier, as specified in ERC-165
     * @return bool True if the contract supports the interface, false otherwise
     *
     * This function supports:
     * - IProtocolAccessManager interface
     * - All interfaces supported by the parent AccessControl contract
     */
    function supportsInterface(
        bytes4 interfaceId
    ) public view override returns (bool) {
        return
            interfaceId == type(IProtocolAccessManager).interfaceId ||
            super.supportsInterface(interfaceId);
    }

    /// @inheritdoc IProtocolAccessManager
    function grantGovernorRole(address account) external onlyGovernor {
        _grantRole(GOVERNOR_ROLE, account);
    }

    /// @inheritdoc IProtocolAccessManager
    function revokeGovernorRole(address account) external onlyGovernor {
        _revokeRole(GOVERNOR_ROLE, account);
    }

    /*//////////////////////////////////////////////////////////////
                        EXTERNAL GOVERNOR FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @inheritdoc IProtocolAccessManager
    function grantSuperKeeperRole(address account) external onlyGovernor {
        _grantRole(SUPER_KEEPER_ROLE, account);
    }

    /// @inheritdoc IProtocolAccessManager
    function grantGuardianRole(address account) external onlyGovernor {
        _grantRole(GUARDIAN_ROLE, account);
    }

    /// @inheritdoc IProtocolAccessManager
    function revokeGuardianRole(address account) external onlyGovernor {
        _revokeRole(GUARDIAN_ROLE, account);
    }

    /// @inheritdoc IProtocolAccessManager
    function revokeSuperKeeperRole(address account) external onlyGovernor {
        _revokeRole(SUPER_KEEPER_ROLE, account);
    }

    /// @inheritdoc IProtocolAccessManager
    function grantContractSpecificRole(
        ContractSpecificRoles roleName,
        address roleTargetContract,
        address roleOwner
    ) public onlyGovernor {
        bytes32 role = generateRole(roleName, roleTargetContract);
        _grantRole(role, roleOwner);
    }

    /// @inheritdoc IProtocolAccessManager
    function revokeContractSpecificRole(
        ContractSpecificRoles roleName,
        address roleTargetContract,
        address roleOwner
    ) public onlyGovernor {
        bytes32 role = generateRole(roleName, roleTargetContract);
        _revokeRole(role, roleOwner);
    }

    /// @inheritdoc IProtocolAccessManager
    function grantCuratorRole(
        address fleetCommanderAddress,
        address account
    ) public onlyGovernor {
        grantContractSpecificRole(
            ContractSpecificRoles.CURATOR_ROLE,
            fleetCommanderAddress,
            account
        );
    }

    /// @inheritdoc IProtocolAccessManager
    function revokeCuratorRole(
        address fleetCommanderAddress,
        address account
    ) public onlyGovernor {
        revokeContractSpecificRole(
            ContractSpecificRoles.CURATOR_ROLE,
            fleetCommanderAddress,
            account
        );
    }

    /// @inheritdoc IProtocolAccessManager
    function grantKeeperRole(
        address fleetCommanderAddress,
        address account
    ) public onlyGovernor {
        grantContractSpecificRole(
            ContractSpecificRoles.KEEPER_ROLE,
            fleetCommanderAddress,
            account
        );
    }

    /// @inheritdoc IProtocolAccessManager
    function revokeKeeperRole(
        address fleetCommanderAddress,
        address account
    ) public onlyGovernor {
        revokeContractSpecificRole(
            ContractSpecificRoles.KEEPER_ROLE,
            fleetCommanderAddress,
            account
        );
    }

    /// @inheritdoc IProtocolAccessManager
    function grantCommanderRole(
        address arkAddress,
        address account
    ) public onlyGovernor {
        grantContractSpecificRole(
            ContractSpecificRoles.COMMANDER_ROLE,
            arkAddress,
            account
        );
    }

    /// @inheritdoc IProtocolAccessManager
    function revokeCommanderRole(
        address arkAddress,
        address account
    ) public onlyGovernor {
        revokeContractSpecificRole(
            ContractSpecificRoles.COMMANDER_ROLE,
            arkAddress,
            account
        );
    }

    /// @inheritdoc IProtocolAccessManager
    function grantDecayControllerRole(address account) public onlyGovernor {
        _grantRole(DECAY_CONTROLLER_ROLE, account);
    }

    /// @inheritdoc IProtocolAccessManager
    function revokeDecayControllerRole(address account) public onlyGovernor {
        _revokeRole(DECAY_CONTROLLER_ROLE, account);
    }

    /*//////////////////////////////////////////////////////////////
                            PUBLIC FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @inheritdoc IProtocolAccessManager
    function selfRevokeContractSpecificRole(
        ContractSpecificRoles roleName,
        address roleTargetContract
    ) public {
        bytes32 role = generateRole(roleName, roleTargetContract);
        if (!hasRole(role, msg.sender)) {
            revert CallerIsNotContractSpecificRole(msg.sender, role);
        }
        _revokeRole(role, msg.sender);
    }

    /// @inheritdoc IProtocolAccessManager
    function generateRole(
        ContractSpecificRoles roleName,
        address roleTargetContract
    ) public pure returns (bytes32) {
        return keccak256(abi.encodePacked(roleName, roleTargetContract));
    }

    /// @inheritdoc IProtocolAccessManager
    function grantAdmiralsQuartersRole(
        address account
    ) external onlyRole(GOVERNOR_ROLE) {
        _grantRole(ADMIRALS_QUARTERS_ROLE, account);
    }

    /// @inheritdoc IProtocolAccessManager
    function revokeAdmiralsQuartersRole(
        address account
    ) external onlyRole(GOVERNOR_ROLE) {
        _revokeRole(ADMIRALS_QUARTERS_ROLE, account);
    }

    mapping(address guardian => uint256 expirationTimestamp)
        public guardianExpirations;

    /**
     * @notice Checks if an account is an active guardian (has role and not expired)
     * @param account Address to check
     * @return bool True if account is an active guardian
     */
    function isActiveGuardian(address account) public view returns (bool) {
        return
            hasRole(GUARDIAN_ROLE, account) &&
            guardianExpirations[account] > block.timestamp;
    }

    /**
     * @notice Sets the expiration timestamp for a guardian
     * @param account Guardian address
     * @param expiration Timestamp when guardian powers expire
     * @dev The expiration period (time from now until expiration) must be between MIN_GUARDIAN_EXPIRY and MAX_GUARDIAN_EXPIRY
     * This ensures guardians can't be immediately removed (protecting against malicious proposals) while still
     * allowing for their eventual phase-out (protecting against malicious guardians)
     */
    function setGuardianExpiration(
        address account,
        uint256 expiration
    ) external onlyRole(GOVERNOR_ROLE) {
        if (!hasRole(GUARDIAN_ROLE, account)) {
            revert CallerIsNotGuardian(account);
        }

        uint256 expiryPeriod = expiration - block.timestamp;
        if (
            expiryPeriod < MIN_GUARDIAN_EXPIRY ||
            expiryPeriod > MAX_GUARDIAN_EXPIRY
        ) {
            revert InvalidGuardianExpiryPeriod(
                expiryPeriod,
                MIN_GUARDIAN_EXPIRY,
                MAX_GUARDIAN_EXPIRY
            );
        }

        guardianExpirations[account] = expiration;
        emit GuardianExpirationSet(account, expiration);
    }

    /**
     * @inheritdoc IProtocolAccessManager
     */
    function hasRole(
        bytes32 role,
        address account
    )
        public
        view
        virtual
        override(IProtocolAccessManager, AccessControl)
        returns (bool)
    {
        return super.hasRole(role, account);
    }

    /// @inheritdoc IProtocolAccessManager
    function getGuardianExpiration(
        address account
    ) external view returns (uint256 expiration) {
        if (!hasRole(GUARDIAN_ROLE, account)) {
            revert CallerIsNotGuardian(account);
        }
        return guardianExpirations[account];
    }

    /// @inheritdoc IProtocolAccessManager
    function grantFoundationRole(address account) external onlyGovernor {
        _grantRole(FOUNDATION_ROLE, account);
    }

    /// @inheritdoc IProtocolAccessManager
    function revokeFoundationRole(address account) external onlyGovernor {
        _revokeRole(FOUNDATION_ROLE, account);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {IAccessControlErrors} from "../interfaces/IAccessControlErrors.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

/**
 * @title LimitedAccessControl
 * @dev This contract extends OpenZeppelin's AccessControl, disabling direct role granting and revoking.
 * It's designed to be used as a base contract for more specific access control implementations.
 * @dev This contract overrides the grantRole and revokeRole functions from AccessControl to disable direct role
 * granting and revoking.
 * @dev It doesn't override the renounceRole function, so it can be used to renounce roles for compromised accounts.
 */
abstract contract LimitedAccessControl is AccessControl, IAccessControlErrors {
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
