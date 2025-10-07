
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {ISummerGovernorV2} from "../interfaces/ISummerGovernorV2.sol";
import {IProtocolAccessManager} from "@summerfi/access-contracts/interfaces/IProtocolAccessManager.sol";
import {IGovernor} from "@openzeppelin/contracts/governance/IGovernor.sol";
import {IERC6372} from "@openzeppelin/contracts/interfaces/IERC6372.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";

import {MessagingFee, OApp, Origin} from "@layerzerolabs/oapp-evm/contracts/oapp/OApp.sol";

import {Governor, GovernorVotes} from "@openzeppelin/contracts/governance/extensions/GovernorVotes.sol";
import {GovernorCountingSimple} from "@openzeppelin/contracts/governance/extensions/GovernorCountingSimple.sol";
import {GovernorSettings} from "@openzeppelin/contracts/governance/extensions/GovernorSettings.sol";
import {GovernorTimelockControl} from "@openzeppelin/contracts/governance/extensions/GovernorTimelockControl.sol";
import {GovernorVotesQuorumFraction} from "@openzeppelin/contracts/governance/extensions/GovernorVotesQuorumFraction.sol";

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/*
 * @title SummerGovernorV2
 * @dev Governance V2 with hub-and-satellite architecture. Voting happens on the hub with xSUMR; finalized proposals
 *      can be sent cross-chain to satellites via LayerZero for queuing and timed execution. Guardianship and proposal
 *      thresholds are enforced; ETH receive is hardened to only accept funds from LayerZero endpoint or timelock.
 */
contract SummerGovernorV2 is
    ISummerGovernorV2,
    GovernorTimelockControl,
    GovernorSettings,
    GovernorCountingSimple,
    GovernorVotesQuorumFraction,
    OApp
{
    /*//////////////////////////////////////////////////////////////
                                CONSTANTS
    //////////////////////////////////////////////////////////////*/

    uint256 public constant MIN_PROPOSAL_THRESHOLD = 1000e18; // 1,000 Tokens
    uint256 public constant MAX_PROPOSAL_THRESHOLD = 100000e18; // 100,000 Tokens
    uint32 public immutable HUB_CHAIN_ID;

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    address public immutable ACCESS_MANAGER;

    /*//////////////////////////////////////////////////////////////
                                MODIFIERS
    //////////////////////////////////////////////////////////////*/

    /**
     * @dev Modifier to restrict certain functions to only be called on the hub chain.
     * This ensures that governance actions like proposing, executing, and canceling can only happen
     * on the designated hub chain, while other chains act as spokes that can only receive and execute
     * proposals that have been approved on the hub.
     */
    modifier onlyHubChain() {
        if (block.chainid != HUB_CHAIN_ID) {
            revert SummerGovernorNotHubChain(block.chainid, HUB_CHAIN_ID);
        }
        _;
    }

    /**
     * @dev Modifier to restrict certain functions to only be called on satellite chains (non-hub chains).
     * This ensures that certain operations can only happen on spoke chains that receive and execute
     * proposals from the hub chain.
     */
    modifier onlySatelliteChain() {
        if (block.chainid == HUB_CHAIN_ID) {
            revert SummerGovernorCannotExecuteOnHubChain();
        }
        _;
    }

    /*//////////////////////////////////////////////////////////////
                                CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    constructor(
        GovernorParams memory params
    )
        Governor("SummerGovernorV2")
        GovernorSettings(
            params.votingDelay,
            params.votingPeriod,
            params.proposalThreshold
        )
        GovernorVotes(params.token)
        GovernorVotesQuorumFraction(params.quorumFraction)
        GovernorTimelockControl(params.timelock)
        OApp(params.endpoint, address(params.initialOwner))
        Ownable(address(params.initialOwner))
    {
        ACCESS_MANAGER = params.accessManager;
        _validateProposalThreshold(params.proposalThreshold);
        HUB_CHAIN_ID = params.hubChainId;
    }

    /*//////////////////////////////////////////////////////////////
                        CROSS-CHAIN MESSAGING FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISummerGovernorV2
    function sendProposalToTargetChain(
        uint32 _dstEid,
        address[] memory _dstTargets,
        uint256[] memory _dstValues,
        bytes[] memory _dstCalldatas,
        bytes32 _dstDescriptionHash,
        bytes calldata _options
    ) external onlyGovernance onlyHubChain {
        // Restrict cross-chain dispatch to governance flow and hub chain
        _sendProposalToTargetChain(
            _dstEid,
            _dstTargets,
            _dstValues,
            _dstCalldatas,
            _dstDescriptionHash,
            _options
        );
    }

    /**
     * @notice Encodes and sends a proposal to a target chain via LayerZero.
     * @dev Computes `dstProposalId = hashProposal(targets, values, calldatas, descriptionHash)` on the destination chain.
     *      Quotes messaging fees and sends using native token. Emits `ProposalSentCrossChain`.
     *      Hub-only and governance-only.
     * @param _dstEid Destination LayerZero endpoint ID
     * @param _dstTargets Target addresses on destination chain
     * @param _dstValues ETH values for each call
     * @param _dstCalldatas Calldata payloads for each call
     * @param _dstDescriptionHash EIP-712 compatible description hash
     * @param _options LayerZero executor options
     */
    function _sendProposalToTargetChain(
        uint32 _dstEid,
        address[] memory _dstTargets,
        uint256[] memory _dstValues,
        bytes[] memory _dstCalldatas,
        bytes32 _dstDescriptionHash,
        bytes calldata _options
    ) internal {
        // Compute proposalId as it will be known on the destination chain
        uint256 dstProposalId = hashProposal(
            _dstTargets,
            _dstValues,
            _dstCalldatas,
            _dstDescriptionHash
        );

        // Prepare payload for LayerZero transport
        bytes memory payload = abi.encode(
            dstProposalId,
            _dstTargets,
            _dstValues,
            _dstCalldatas,
            _dstDescriptionHash
        );

        // Quote and pay native execution fee in the contract's native balance
        MessagingFee memory fee = _quote(_dstEid, payload, _options, false);

        _lzSend(
            _dstEid,
            payload,
            _options,
            MessagingFee(fee.nativeFee, 0),
            payable(address(this))
        );

        emit ProposalSentCrossChain(dstProposalId, _dstEid);
    }

    /**
     * @dev Receive function to allow the contract to receive ETH from LayerZero
     * @dev Accepts ETH only from LayerZero endpoint or the timelock executor.
     * @dev Prevents accidental or malicious direct funding by other addresses.
     */
    receive() external payable override {
        // Allow deposits from LayerZero endpoint or timelock
        if (msg.sender != address(endpoint) && msg.sender != timelock()) {
            revert GovernorDisabledDeposit();
        }
    }

    /**
     * @notice Queues a received cross-chain proposal into the local timelock.
     * @dev Satellite-only. Returns proposalId and emits `ProposalQueued` with eta.
     * @param proposalId Proposal identifier (must match destination chain hash)
     * @param targets Target addresses for queued operations
     * @param values ETH values for queued operations
     * @param calldatas Calldata for queued operations
     * @param descriptionHash Description hash
     */
    function _queueCrossChainProposal(
        uint256 proposalId,
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) internal onlySatelliteChain returns (uint256) {
        // Satellite-only queueing of received operations into the local timelock controller
        uint48 eta = _queueOperations(
            proposalId,
            targets,
            values,
            calldatas,
            descriptionHash
        );

        emit ProposalQueued(proposalId, uint256(eta));

        return proposalId;
    }

    /**
     * @notice LayerZero receive hook: decodes cross-chain proposal and queues it locally.
     * @dev Emits `ProposalReceivedCrossChain`. Trust boundary is the LayerZero endpoint configuration.
     * @param _origin Origin metadata (contains srcEid)
     * @param payload ABI-encoded (proposalId, targets[], values[], calldatas[], descriptionHash)
     */
    function _lzReceive(
        Origin calldata _origin,
        bytes32,
        bytes calldata payload,
        address,
        bytes calldata
    ) internal override {
        // Decode the cross-chain proposal payload
        (
            uint256 proposalId,
            address[] memory targets,
            uint256[] memory values,
            bytes[] memory calldatas,
            bytes32 descriptionHash
        ) = abi.decode(
                payload,
                (uint256, address[], uint256[], bytes[], bytes32)
            );

        emit ProposalReceivedCrossChain(proposalId, _origin.srcEid);

        // Queue operations for later execution per local timelock settings
        _queueCrossChainProposal(
            proposalId,
            targets,
            values,
            calldatas,
            descriptionHash
        );
    }

    /*//////////////////////////////////////////////////////////////
                            GOVERNANCE FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISummerGovernorV2
    function castVote(
        uint256 proposalId,
        uint8 support
    )
        public
        override(ISummerGovernorV2, Governor)
        onlyHubChain
        returns (uint256)
    {
        // Vote is hub-only; OZ Governor handles voting power checks via token snapshots
        address voter = _msgSender();
        return _castVote(proposalId, voter, support, "");
    }

    /// @inheritdoc ISummerGovernorV2
    function propose(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        string memory description
    )
        public
        override(Governor, ISummerGovernorV2)
        onlyHubChain
        returns (uint256)
    {
        // Enforce proposer threshold, with guardian override via access manager
        address proposer = _msgSender();
        uint256 proposerVotes = getVotes(proposer, block.timestamp - 1);

        if (
            proposerVotes < proposalThreshold() && !isActiveGuardian(proposer)
        ) {
            revert SummerGovernorProposerBelowThresholdAndNotGuardian(
                proposer,
                proposerVotes,
                proposalThreshold()
            );
        }

        return _propose(targets, values, calldatas, description, proposer);
    }

    /// @inheritdoc ISummerGovernorV2
    function execute(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    )
        public
        payable
        override(Governor, ISummerGovernorV2)
        onlyHubChain
        returns (uint256)
    {
        // Timelock-controlled execution path from OZ GovernorTimelockControl
        return super.execute(targets, values, calldatas, descriptionHash);
    }

    /// @inheritdoc ISummerGovernorV2
    function cancel(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    )
        public
        override(Governor, ISummerGovernorV2)
        onlyHubChain
        returns (uint256)
    {
        // Anyone may cancel if proposer is below threshold; guardians may have extra cancellation powers
        uint256 proposalId = hashProposal(
            targets,
            values,
            calldatas,
            descriptionHash
        );
        address proposer = proposalProposer(proposalId);
        if (
            _msgSender() != proposer &&
            getVotes(proposer, block.timestamp - 1) >= proposalThreshold() &&
            !isActiveGuardian(_msgSender())
        ) {
            revert SummerGovernorUnauthorizedCancellation(
                _msgSender(),
                proposer,
                getVotes(proposer, block.timestamp - 1),
                proposalThreshold()
            );
        }

        return _cancel(targets, values, calldatas, descriptionHash);
    }

    /*//////////////////////////////////////////////////////////////
                        WHITELIST MANAGEMENT FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @inheritdoc ISummerGovernorV2
    function isActiveGuardian(address account) public view returns (bool) {
        // Delegate guardian status to the shared ProtocolAccessManager
        return IProtocolAccessManager(ACCESS_MANAGER).isActiveGuardian(account);
    }

    /*//////////////////////////////////////////////////////////////
                            INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Pays LayerZero native execution fee.
     * @dev Reverts if contract native balance is insufficient. Caller must ensure funding.
     * @param _nativeFee Quoted native fee amount
     * @return nativeFee Echoed native fee amount
     */
    function _payNative(
        uint256 _nativeFee
    ) internal view override returns (uint256 nativeFee) {
        // Ensure the contract is pre-funded to cover executor fee; avoids trapping proposals mid-flight
        if (address(this).balance < _nativeFee) {
            revert NotEnoughNative(address(this).balance);
        }
        return _nativeFee;
    }

    /**
     * @dev Internal function to validate the proposal threshold
     * @param thresholdToValidate The threshold value to validate against min/max bounds
     */
    function _validateProposalThreshold(
        uint256 thresholdToValidate
    ) internal pure {
        if (
            thresholdToValidate < MIN_PROPOSAL_THRESHOLD ||
            thresholdToValidate > MAX_PROPOSAL_THRESHOLD
        ) {
            revert SummerGovernorInvalidProposalThreshold(
                thresholdToValidate,
                MIN_PROPOSAL_THRESHOLD,
                MAX_PROPOSAL_THRESHOLD
            );
        }
    }

    /*//////////////////////////////////////////////////////////////
                            OVERRIDE FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /*
     * @dev Overrides the internal cancellation function to use the timelocked version
     * @param targets The addresses of the contracts to call
     * @param values The ETH values to send with the calls
     * @param calldatas The call data for each contract call
     * @param descriptionHash The hash of the proposal description
     * @return The ID of the cancelled proposal
     */
    function _cancel(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) internal override(Governor, GovernorTimelockControl) returns (uint256) {
        return
            GovernorTimelockControl._cancel(
                targets,
                values,
                calldatas,
                descriptionHash
            );
    }

    /**
     * @dev Returns the address of the executor (timelock).
     * @return The address of the executor.
     */
    function _executor()
        internal
        view
        override(Governor, GovernorTimelockControl)
        returns (address)
    {
        return GovernorTimelockControl._executor();
    }

    /**
     * @dev Returns the current proposal threshold.
     * @return The current proposal threshold.
     */
    function proposalThreshold()
        public
        view
        override(Governor, GovernorSettings, IGovernor)
        returns (uint256)
    {
        return GovernorSettings.proposalThreshold();
    }

    /**
     * @dev Returns the state of a proposal.
     * @param proposalId The ID of the proposal.
     * @return The current state of the proposal.
     */
    function state(
        uint256 proposalId
    )
        public
        view
        override(Governor, GovernorTimelockControl, IGovernor)
        returns (ProposalState)
    {
        return GovernorTimelockControl.state(proposalId);
    }

    /**
     * @dev Checks if the contract supports an interface.
     * @param interfaceId The interface identifier.
     * @return True if the contract supports the interface, false otherwise.
     */
    function supportsInterface(
        bytes4 interfaceId
    ) public view override(Governor, IERC165) returns (bool) {
        return super.supportsInterface(interfaceId);
    }

    /**
     * @dev Internal function to execute proposal operations.
     * @param proposalId The ID of the proposal.
     * @param targets The addresses of the contracts to call.
     * @param values The ETH values to send with the calls.
     * @param calldatas The call data for each contract call.
     * @param descriptionHash The hash of the proposal description.
     */
    function _executeOperations(
        uint256 proposalId,
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) internal override(Governor, GovernorTimelockControl) {
        GovernorTimelockControl._executeOperations(
            proposalId,
            targets,
            values,
            calldatas,
            descriptionHash
        );
    }

    /**
     * @dev Internal function to queue proposal operations.
     * @param proposalId The ID of the proposal.
     * @param targets The addresses of the contracts to call.
     * @param values The ETH values to send with the calls.
     * @param calldatas The call data for each contract call.
     * @param descriptionHash The hash of the proposal description.
     * @return The timestamp at which the proposal will be executable.
     */
    function _queueOperations(
        uint256 proposalId,
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) internal override(Governor, GovernorTimelockControl) returns (uint48) {
        return
            GovernorTimelockControl._queueOperations(
                proposalId,
                targets,
                values,
                calldatas,
                descriptionHash
            );
    }

    /**
     * @dev Checks if a proposal needs queuing.
     * @param proposalId The ID of the proposal.
     * @return True if the proposal needs queuing, false otherwise.
     */
    function proposalNeedsQueuing(
        uint256 proposalId
    )
        public
        view
        override(Governor, GovernorTimelockControl, IGovernor)
        returns (bool)
    {
        return super.proposalNeedsQueuing(proposalId);
    }

    /**
     * @dev Returns the clock mode used by the contract.
     * @return A string describing the clock mode.
     */
    function CLOCK_MODE()
        public
        view
        override(Governor, GovernorVotes, IERC6372)
        returns (string memory)
    {
        return super.CLOCK_MODE();
    }

    /**
     * @dev Returns the current clock value used by the contract.
     * @return The current clock value.
     */
    function clock()
        public
        view
        override(Governor, GovernorVotes, IERC6372)
        returns (uint48)
    {
        return super.clock();
    }

    /**
     * @dev Calculates the quorum for a specific timepoint.
     * @param timepoint The timepoint to calculate the quorum for.
     * @return The quorum value.
     */
    function quorum(
        uint256 timepoint
    )
        public
        view
        override(Governor, GovernorVotesQuorumFraction, IGovernor)
        returns (uint256)
    {
        return super.quorum(timepoint);
    }

    /**
     * @dev Returns the current voting delay.
     * @return The current voting delay
     */
    function votingDelay()
        public
        view
        override(Governor, GovernorSettings, IGovernor)
        returns (uint256)
    {
        return super.votingDelay();
    }

    /**
     * @dev Returns the current voting period.
     * @return The current voting period
     */
    function votingPeriod()
        public
        view
        override(Governor, GovernorSettings, IGovernor)
        returns (uint256)
    {
        return super.votingPeriod();
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.0) (governance/Governor.sol)

pragma solidity ^0.8.20;

import {IERC721Receiver} from "../token/ERC721/IERC721Receiver.sol";
import {IERC1155Receiver} from "../token/ERC1155/IERC1155Receiver.sol";
import {EIP712} from "../utils/cryptography/EIP712.sol";
import {SignatureChecker} from "../utils/cryptography/SignatureChecker.sol";
import {IERC165, ERC165} from "../utils/introspection/ERC165.sol";
import {SafeCast} from "../utils/math/SafeCast.sol";
import {DoubleEndedQueue} from "../utils/structs/DoubleEndedQueue.sol";
import {Address} from "../utils/Address.sol";
import {Context} from "../utils/Context.sol";
import {Nonces} from "../utils/Nonces.sol";
import {IGovernor, IERC6372} from "./IGovernor.sol";

/**
 * @dev Core of the governance system, designed to be extended through various modules.
 *
 * This contract is abstract and requires several functions to be implemented in various modules:
 *
 * - A counting module must implement {quorum}, {_quorumReached}, {_voteSucceeded} and {_countVote}
 * - A voting module must implement {_getVotes}
 * - Additionally, {votingPeriod} must also be implemented
 */
abstract contract Governor is Context, ERC165, EIP712, Nonces, IGovernor, IERC721Receiver, IERC1155Receiver {
    using DoubleEndedQueue for DoubleEndedQueue.Bytes32Deque;

    bytes32 public constant BALLOT_TYPEHASH =
        keccak256("Ballot(uint256 proposalId,uint8 support,address voter,uint256 nonce)");
    bytes32 public constant EXTENDED_BALLOT_TYPEHASH =
        keccak256(
            "ExtendedBallot(uint256 proposalId,uint8 support,address voter,uint256 nonce,string reason,bytes params)"
        );

    struct ProposalCore {
        address proposer;
        uint48 voteStart;
        uint32 voteDuration;
        bool executed;
        bool canceled;
        uint48 etaSeconds;
    }

    bytes32 private constant ALL_PROPOSAL_STATES_BITMAP = bytes32((2 ** (uint8(type(ProposalState).max) + 1)) - 1);
    string private _name;

    mapping(uint256 proposalId => ProposalCore) private _proposals;

    // This queue keeps track of the governor operating on itself. Calls to functions protected by the {onlyGovernance}
    // modifier needs to be whitelisted in this queue. Whitelisting is set in {execute}, consumed by the
    // {onlyGovernance} modifier and eventually reset after {_executeOperations} completes. This ensures that the
    // execution of {onlyGovernance} protected calls can only be achieved through successful proposals.
    DoubleEndedQueue.Bytes32Deque private _governanceCall;

    /**
     * @dev Restricts a function so it can only be executed through governance proposals. For example, governance
     * parameter setters in {GovernorSettings} are protected using this modifier.
     *
     * The governance executing address may be different from the Governor's own address, for example it could be a
     * timelock. This can be customized by modules by overriding {_executor}. The executor is only able to invoke these
     * functions during the execution of the governor's {execute} function, and not under any other circumstances. Thus,
     * for example, additional timelock proposers are not able to change governance parameters without going through the
     * governance protocol (since v4.6).
     */
    modifier onlyGovernance() {
        _checkGovernance();
        _;
    }

    /**
     * @dev Sets the value for {name} and {version}
     */
    constructor(string memory name_) EIP712(name_, version()) {
        _name = name_;
    }

    /**
     * @dev Function to receive ETH that will be handled by the governor (disabled if executor is a third party contract)
     */
    receive() external payable virtual {
        if (_executor() != address(this)) {
            revert GovernorDisabledDeposit();
        }
    }

    /**
     * @dev See {IERC165-supportsInterface}.
     */
    function supportsInterface(bytes4 interfaceId) public view virtual override(IERC165, ERC165) returns (bool) {
        return
            interfaceId == type(IGovernor).interfaceId ||
            interfaceId == type(IERC1155Receiver).interfaceId ||
            super.supportsInterface(interfaceId);
    }

    /**
     * @dev See {IGovernor-name}.
     */
    function name() public view virtual returns (string memory) {
        return _name;
    }

    /**
     * @dev See {IGovernor-version}.
     */
    function version() public view virtual returns (string memory) {
        return "1";
    }

    /**
     * @dev See {IGovernor-hashProposal}.
     *
     * The proposal id is produced by hashing the ABI encoded `targets` array, the `values` array, the `calldatas` array
     * and the descriptionHash (bytes32 which itself is the keccak256 hash of the description string). This proposal id
     * can be produced from the proposal data which is part of the {ProposalCreated} event. It can even be computed in
     * advance, before the proposal is submitted.
     *
     * Note that the chainId and the governor address are not part of the proposal id computation. Consequently, the
     * same proposal (with same operation and same description) will have the same id if submitted on multiple governors
     * across multiple networks. This also means that in order to execute the same operation twice (on the same
     * governor) the proposer will have to change the description in order to avoid proposal id conflicts.
     */
    function hashProposal(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) public pure virtual returns (uint256) {
        return uint256(keccak256(abi.encode(targets, values, calldatas, descriptionHash)));
    }

    /**
     * @dev See {IGovernor-state}.
     */
    function state(uint256 proposalId) public view virtual returns (ProposalState) {
        // We read the struct fields into the stack at once so Solidity emits a single SLOAD
        ProposalCore storage proposal = _proposals[proposalId];
        bool proposalExecuted = proposal.executed;
        bool proposalCanceled = proposal.canceled;

        if (proposalExecuted) {
            return ProposalState.Executed;
        }

        if (proposalCanceled) {
            return ProposalState.Canceled;
        }

        uint256 snapshot = proposalSnapshot(proposalId);

        if (snapshot == 0) {
            revert GovernorNonexistentProposal(proposalId);
        }

        uint256 currentTimepoint = clock();

        if (snapshot >= currentTimepoint) {
            return ProposalState.Pending;
        }

        uint256 deadline = proposalDeadline(proposalId);

        if (deadline >= currentTimepoint) {
            return ProposalState.Active;
        } else if (!_quorumReached(proposalId) || !_voteSucceeded(proposalId)) {
            return ProposalState.Defeated;
        } else if (proposalEta(proposalId) == 0) {
            return ProposalState.Succeeded;
        } else {
            return ProposalState.Queued;
        }
    }

    /**
     * @dev See {IGovernor-proposalThreshold}.
     */
    function proposalThreshold() public view virtual returns (uint256) {
        return 0;
    }

    /**
     * @dev See {IGovernor-proposalSnapshot}.
     */
    function proposalSnapshot(uint256 proposalId) public view virtual returns (uint256) {
        return _proposals[proposalId].voteStart;
    }

    /**
     * @dev See {IGovernor-proposalDeadline}.
     */
    function proposalDeadline(uint256 proposalId) public view virtual returns (uint256) {
        return _proposals[proposalId].voteStart + _proposals[proposalId].voteDuration;
    }

    /**
     * @dev See {IGovernor-proposalProposer}.
     */
    function proposalProposer(uint256 proposalId) public view virtual returns (address) {
        return _proposals[proposalId].proposer;
    }

    /**
     * @dev See {IGovernor-proposalEta}.
     */
    function proposalEta(uint256 proposalId) public view virtual returns (uint256) {
        return _proposals[proposalId].etaSeconds;
    }

    /**
     * @dev See {IGovernor-proposalNeedsQueuing}.
     */
    function proposalNeedsQueuing(uint256) public view virtual returns (bool) {
        return false;
    }

    /**
     * @dev Reverts if the `msg.sender` is not the executor. In case the executor is not this contract
     * itself, the function reverts if `msg.data` is not whitelisted as a result of an {execute}
     * operation. See {onlyGovernance}.
     */
    function _checkGovernance() internal virtual {
        if (_executor() != _msgSender()) {
            revert GovernorOnlyExecutor(_msgSender());
        }
        if (_executor() != address(this)) {
            bytes32 msgDataHash = keccak256(_msgData());
            // loop until popping the expected operation - throw if deque is empty (operation not authorized)
            while (_governanceCall.popFront() != msgDataHash) {}
        }
    }

    /**
     * @dev Amount of votes already cast passes the threshold limit.
     */
    function _quorumReached(uint256 proposalId) internal view virtual returns (bool);

    /**
     * @dev Is the proposal successful or not.
     */
    function _voteSucceeded(uint256 proposalId) internal view virtual returns (bool);

    /**
     * @dev Get the voting weight of `account` at a specific `timepoint`, for a vote as described by `params`.
     */
    function _getVotes(address account, uint256 timepoint, bytes memory params) internal view virtual returns (uint256);

    /**
     * @dev Register a vote for `proposalId` by `account` with a given `support`, voting `weight` and voting `params`.
     *
     * Note: Support is generic and can represent various things depending on the voting system used.
     */
    function _countVote(
        uint256 proposalId,
        address account,
        uint8 support,
        uint256 totalWeight,
        bytes memory params
    ) internal virtual returns (uint256);

    /**
     * @dev Default additional encoded parameters used by castVote methods that don't include them
     *
     * Note: Should be overridden by specific implementations to use an appropriate value, the
     * meaning of the additional params, in the context of that implementation
     */
    function _defaultParams() internal view virtual returns (bytes memory) {
        return "";
    }

    /**
     * @dev See {IGovernor-propose}. This function has opt-in frontrunning protection, described in {_isValidDescriptionForProposer}.
     */
    function propose(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        string memory description
    ) public virtual returns (uint256) {
        address proposer = _msgSender();

        // check description restriction
        if (!_isValidDescriptionForProposer(proposer, description)) {
            revert GovernorRestrictedProposer(proposer);
        }

        // check proposal threshold
        uint256 votesThreshold = proposalThreshold();
        if (votesThreshold > 0) {
            uint256 proposerVotes = getVotes(proposer, clock() - 1);
            if (proposerVotes < votesThreshold) {
                revert GovernorInsufficientProposerVotes(proposer, proposerVotes, votesThreshold);
            }
        }

        return _propose(targets, values, calldatas, description, proposer);
    }

    /**
     * @dev Internal propose mechanism. Can be overridden to add more logic on proposal creation.
     *
     * Emits a {IGovernor-ProposalCreated} event.
     */
    function _propose(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        string memory description,
        address proposer
    ) internal virtual returns (uint256 proposalId) {
        proposalId = hashProposal(targets, values, calldatas, keccak256(bytes(description)));

        if (targets.length != values.length || targets.length != calldatas.length || targets.length == 0) {
            revert GovernorInvalidProposalLength(targets.length, calldatas.length, values.length);
        }
        if (_proposals[proposalId].voteStart != 0) {
            revert GovernorUnexpectedProposalState(proposalId, state(proposalId), bytes32(0));
        }

        uint256 snapshot = clock() + votingDelay();
        uint256 duration = votingPeriod();

        ProposalCore storage proposal = _proposals[proposalId];
        proposal.proposer = proposer;
        proposal.voteStart = SafeCast.toUint48(snapshot);
        proposal.voteDuration = SafeCast.toUint32(duration);

        emit ProposalCreated(
            proposalId,
            proposer,
            targets,
            values,
            new string[](targets.length),
            calldatas,
            snapshot,
            snapshot + duration,
            description
        );

        // Using a named return variable to avoid stack too deep errors
    }

    /**
     * @dev See {IGovernor-queue}.
     */
    function queue(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) public virtual returns (uint256) {
        uint256 proposalId = hashProposal(targets, values, calldatas, descriptionHash);

        _validateStateBitmap(proposalId, _encodeStateBitmap(ProposalState.Succeeded));

        uint48 etaSeconds = _queueOperations(proposalId, targets, values, calldatas, descriptionHash);

        if (etaSeconds != 0) {
            _proposals[proposalId].etaSeconds = etaSeconds;
            emit ProposalQueued(proposalId, etaSeconds);
        } else {
            revert GovernorQueueNotImplemented();
        }

        return proposalId;
    }

    /**
     * @dev Internal queuing mechanism. Can be overridden (without a super call) to modify the way queuing is
     * performed (for example adding a vault/timelock).
     *
     * This is empty by default, and must be overridden to implement queuing.
     *
     * This function returns a timestamp that describes the expected ETA for execution. If the returned value is 0
     * (which is the default value), the core will consider queueing did not succeed, and the public {queue} function
     * will revert.
     *
     * NOTE: Calling this function directly will NOT check the current state of the proposal, or emit the
     * `ProposalQueued` event. Queuing a proposal should be done using {queue}.
     */
    function _queueOperations(
        uint256 /*proposalId*/,
        address[] memory /*targets*/,
        uint256[] memory /*values*/,
        bytes[] memory /*calldatas*/,
        bytes32 /*descriptionHash*/
    ) internal virtual returns (uint48) {
        return 0;
    }

    /**
     * @dev See {IGovernor-execute}.
     */
    function execute(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) public payable virtual returns (uint256) {
        uint256 proposalId = hashProposal(targets, values, calldatas, descriptionHash);

        _validateStateBitmap(
            proposalId,
            _encodeStateBitmap(ProposalState.Succeeded) | _encodeStateBitmap(ProposalState.Queued)
        );

        // mark as executed before calls to avoid reentrancy
        _proposals[proposalId].executed = true;

        // before execute: register governance call in queue.
        if (_executor() != address(this)) {
            for (uint256 i = 0; i < targets.length; ++i) {
                if (targets[i] == address(this)) {
                    _governanceCall.pushBack(keccak256(calldatas[i]));
                }
            }
        }

        _executeOperations(proposalId, targets, values, calldatas, descriptionHash);

        // after execute: cleanup governance call queue.
        if (_executor() != address(this) && !_governanceCall.empty()) {
            _governanceCall.clear();
        }

        emit ProposalExecuted(proposalId);

        return proposalId;
    }

    /**
     * @dev Internal execution mechanism. Can be overridden (without a super call) to modify the way execution is
     * performed (for example adding a vault/timelock).
     *
     * NOTE: Calling this function directly will NOT check the current state of the proposal, set the executed flag to
     * true or emit the `ProposalExecuted` event. Executing a proposal should be done using {execute} or {_execute}.
     */
    function _executeOperations(
        uint256 /* proposalId */,
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 /*descriptionHash*/
    ) internal virtual {
        for (uint256 i = 0; i < targets.length; ++i) {
            (bool success, bytes memory returndata) = targets[i].call{value: values[i]}(calldatas[i]);
            Address.verifyCallResult(success, returndata);
        }
    }

    /**
     * @dev See {IGovernor-cancel}.
     */
    function cancel(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) public virtual returns (uint256) {
        // The proposalId will be recomputed in the `_cancel` call further down. However we need the value before we
        // do the internal call, because we need to check the proposal state BEFORE the internal `_cancel` call
        // changes it. The `hashProposal` duplication has a cost that is limited, and that we accept.
        uint256 proposalId = hashProposal(targets, values, calldatas, descriptionHash);

        // public cancel restrictions (on top of existing _cancel restrictions).
        _validateStateBitmap(proposalId, _encodeStateBitmap(ProposalState.Pending));
        if (_msgSender() != proposalProposer(proposalId)) {
            revert GovernorOnlyProposer(_msgSender());
        }

        return _cancel(targets, values, calldatas, descriptionHash);
    }

    /**
     * @dev Internal cancel mechanism with minimal restrictions. A proposal can be cancelled in any state other than
     * Canceled, Expired, or Executed. Once cancelled a proposal can't be re-submitted.
     *
     * Emits a {IGovernor-ProposalCanceled} event.
     */
    function _cancel(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) internal virtual returns (uint256) {
        uint256 proposalId = hashProposal(targets, values, calldatas, descriptionHash);

        _validateStateBitmap(
            proposalId,
            ALL_PROPOSAL_STATES_BITMAP ^
                _encodeStateBitmap(ProposalState.Canceled) ^
                _encodeStateBitmap(ProposalState.Expired) ^
                _encodeStateBitmap(ProposalState.Executed)
        );

        _proposals[proposalId].canceled = true;
        emit ProposalCanceled(proposalId);

        return proposalId;
    }

    /**
     * @dev See {IGovernor-getVotes}.
     */
    function getVotes(address account, uint256 timepoint) public view virtual returns (uint256) {
        return _getVotes(account, timepoint, _defaultParams());
    }

    /**
     * @dev See {IGovernor-getVotesWithParams}.
     */
    function getVotesWithParams(
        address account,
        uint256 timepoint,
        bytes memory params
    ) public view virtual returns (uint256) {
        return _getVotes(account, timepoint, params);
    }

    /**
     * @dev See {IGovernor-castVote}.
     */
    function castVote(uint256 proposalId, uint8 support) public virtual returns (uint256) {
        address voter = _msgSender();
        return _castVote(proposalId, voter, support, "");
    }

    /**
     * @dev See {IGovernor-castVoteWithReason}.
     */
    function castVoteWithReason(
        uint256 proposalId,
        uint8 support,
        string calldata reason
    ) public virtual returns (uint256) {
        address voter = _msgSender();
        return _castVote(proposalId, voter, support, reason);
    }

    /**
     * @dev See {IGovernor-castVoteWithReasonAndParams}.
     */
    function castVoteWithReasonAndParams(
        uint256 proposalId,
        uint8 support,
        string calldata reason,
        bytes memory params
    ) public virtual returns (uint256) {
        address voter = _msgSender();
        return _castVote(proposalId, voter, support, reason, params);
    }

    /**
     * @dev See {IGovernor-castVoteBySig}.
     */
    function castVoteBySig(
        uint256 proposalId,
        uint8 support,
        address voter,
        bytes memory signature
    ) public virtual returns (uint256) {
        bool valid = SignatureChecker.isValidSignatureNow(
            voter,
            _hashTypedDataV4(keccak256(abi.encode(BALLOT_TYPEHASH, proposalId, support, voter, _useNonce(voter)))),
            signature
        );

        if (!valid) {
            revert GovernorInvalidSignature(voter);
        }

        return _castVote(proposalId, voter, support, "");
    }

    /**
     * @dev See {IGovernor-castVoteWithReasonAndParamsBySig}.
     */
    function castVoteWithReasonAndParamsBySig(
        uint256 proposalId,
        uint8 support,
        address voter,
        string calldata reason,
        bytes memory params,
        bytes memory signature
    ) public virtual returns (uint256) {
        bool valid = SignatureChecker.isValidSignatureNow(
            voter,
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        EXTENDED_BALLOT_TYPEHASH,
                        proposalId,
                        support,
                        voter,
                        _useNonce(voter),
                        keccak256(bytes(reason)),
                        keccak256(params)
                    )
                )
            ),
            signature
        );

        if (!valid) {
            revert GovernorInvalidSignature(voter);
        }

        return _castVote(proposalId, voter, support, reason, params);
    }

    /**
     * @dev Internal vote casting mechanism: Check that the vote is pending, that it has not been cast yet, retrieve
     * voting weight using {IGovernor-getVotes} and call the {_countVote} internal function. Uses the _defaultParams().
     *
     * Emits a {IGovernor-VoteCast} event.
     */
    function _castVote(
        uint256 proposalId,
        address account,
        uint8 support,
        string memory reason
    ) internal virtual returns (uint256) {
        return _castVote(proposalId, account, support, reason, _defaultParams());
    }

    /**
     * @dev Internal vote casting mechanism: Check that the vote is pending, that it has not been cast yet, retrieve
     * voting weight using {IGovernor-getVotes} and call the {_countVote} internal function.
     *
     * Emits a {IGovernor-VoteCast} event.
     */
    function _castVote(
        uint256 proposalId,
        address account,
        uint8 support,
        string memory reason,
        bytes memory params
    ) internal virtual returns (uint256) {
        _validateStateBitmap(proposalId, _encodeStateBitmap(ProposalState.Active));

        uint256 totalWeight = _getVotes(account, proposalSnapshot(proposalId), params);
        uint256 votedWeight = _countVote(proposalId, account, support, totalWeight, params);

        if (params.length == 0) {
            emit VoteCast(account, proposalId, support, votedWeight, reason);
        } else {
            emit VoteCastWithParams(account, proposalId, support, votedWeight, reason, params);
        }

        return votedWeight;
    }

    /**
     * @dev Relays a transaction or function call to an arbitrary target. In cases where the governance executor
     * is some contract other than the governor itself, like when using a timelock, this function can be invoked
     * in a governance proposal to recover tokens or Ether that was sent to the governor contract by mistake.
     * Note that if the executor is simply the governor itself, use of `relay` is redundant.
     */
    function relay(address target, uint256 value, bytes calldata data) external payable virtual onlyGovernance {
        (bool success, bytes memory returndata) = target.call{value: value}(data);
        Address.verifyCallResult(success, returndata);
    }

    /**
     * @dev Address through which the governor executes action. Will be overloaded by module that execute actions
     * through another contract such as a timelock.
     */
    function _executor() internal view virtual returns (address) {
        return address(this);
    }

    /**
     * @dev See {IERC721Receiver-onERC721Received}.
     * Receiving tokens is disabled if the governance executor is other than the governor itself (eg. when using with a timelock).
     */
    function onERC721Received(address, address, uint256, bytes memory) public virtual returns (bytes4) {
        if (_executor() != address(this)) {
            revert GovernorDisabledDeposit();
        }
        return this.onERC721Received.selector;
    }

    /**
     * @dev See {IERC1155Receiver-onERC1155Received}.
     * Receiving tokens is disabled if the governance executor is other than the governor itself (eg. when using with a timelock).
     */
    function onERC1155Received(address, address, uint256, uint256, bytes memory) public virtual returns (bytes4) {
        if (_executor() != address(this)) {
            revert GovernorDisabledDeposit();
        }
        return this.onERC1155Received.selector;
    }

    /**
     * @dev See {IERC1155Receiver-onERC1155BatchReceived}.
     * Receiving tokens is disabled if the governance executor is other than the governor itself (eg. when using with a timelock).
     */
    function onERC1155BatchReceived(
        address,
        address,
        uint256[] memory,
        uint256[] memory,
        bytes memory
    ) public virtual returns (bytes4) {
        if (_executor() != address(this)) {
            revert GovernorDisabledDeposit();
        }
        return this.onERC1155BatchReceived.selector;
    }

    /**
     * @dev Encodes a `ProposalState` into a `bytes32` representation where each bit enabled corresponds to
     * the underlying position in the `ProposalState` enum. For example:
     *
     * 0x000...10000
     *   ^^^^^^------ ...
     *         ^----- Succeeded
     *          ^---- Defeated
     *           ^--- Canceled
     *            ^-- Active
     *             ^- Pending
     */
    function _encodeStateBitmap(ProposalState proposalState) internal pure returns (bytes32) {
        return bytes32(1 << uint8(proposalState));
    }

    /**
     * @dev Check that the current state of a proposal matches the requirements described by the `allowedStates` bitmap.
     * This bitmap should be built using `_encodeStateBitmap`.
     *
     * If requirements are not met, reverts with a {GovernorUnexpectedProposalState} error.
     */
    function _validateStateBitmap(uint256 proposalId, bytes32 allowedStates) private view returns (ProposalState) {
        ProposalState currentState = state(proposalId);
        if (_encodeStateBitmap(currentState) & allowedStates == bytes32(0)) {
            revert GovernorUnexpectedProposalState(proposalId, currentState, allowedStates);
        }
        return currentState;
    }

    /*
     * @dev Check if the proposer is authorized to submit a proposal with the given description.
     *
     * If the proposal description ends with `#proposer=0x???`, where `0x???` is an address written as a hex string
     * (case insensitive), then the submission of this proposal will only be authorized to said address.
     *
     * This is used for frontrunning protection. By adding this pattern at the end of their proposal, one can ensure
     * that no other address can submit the same proposal. An attacker would have to either remove or change that part,
     * which would result in a different proposal id.
     *
     * If the description does not match this pattern, it is unrestricted and anyone can submit it. This includes:
     * - If the `0x???` part is not a valid hex string.
     * - If the `0x???` part is a valid hex string, but does not contain exactly 40 hex digits.
     * - If it ends with the expected suffix followed by newlines or other whitespace.
     * - If it ends with some other similar suffix, e.g. `#other=abc`.
     * - If it does not end with any such suffix.
     */
    function _isValidDescriptionForProposer(
        address proposer,
        string memory description
    ) internal view virtual returns (bool) {
        uint256 len = bytes(description).length;

        // Length is too short to contain a valid proposer suffix
        if (len < 52) {
            return true;
        }

        // Extract what would be the `#proposer=0x` marker beginning the suffix
        bytes12 marker;
        assembly ("memory-safe") {
            // - Start of the string contents in memory = description + 32
            // - First character of the marker = len - 52
            //   - Length of "#proposer=0x0000000000000000000000000000000000000000" = 52
            // - We read the memory word starting at the first character of the marker:
            //   - (description + 32) + (len - 52) = description + (len - 20)
            // - Note: Solidity will ignore anything past the first 12 bytes
            marker := mload(add(description, sub(len, 20)))
        }

        // If the marker is not found, there is no proposer suffix to check
        if (marker != bytes12("#proposer=0x")) {
            return true;
        }

        // Parse the 40 characters following the marker as uint160
        uint160 recovered = 0;
        for (uint256 i = len - 40; i < len; ++i) {
            (bool isHex, uint8 value) = _tryHexToUint(bytes(description)[i]);
            // If any of the characters is not a hex digit, ignore the suffix entirely
            if (!isHex) {
                return true;
            }
            recovered = (recovered << 4) | value;
        }

        return recovered == uint160(proposer);
    }

    /**
     * @dev Try to parse a character from a string as a hex value. Returns `(true, value)` if the char is in
     * `[0-9a-fA-F]` and `(false, 0)` otherwise. Value is guaranteed to be in the range `0 <= value < 16`
     */
    function _tryHexToUint(bytes1 char) private pure returns (bool isHex, uint8 value) {
        uint8 c = uint8(char);
        unchecked {
            // Case 0-9
            if (47 < c && c < 58) {
                return (true, c - 48);
            }
            // Case A-F
            else if (64 < c && c < 71) {
                return (true, c - 55);
            }
            // Case a-f
            else if (96 < c && c < 103) {
                return (true, c - 87);
            }
            // Else: not a hex char
            else {
                return (false, 0);
            }
        }
    }

    /**
     * @inheritdoc IERC6372
     */
    function clock() public view virtual returns (uint48);

    /**
     * @inheritdoc IERC6372
     */
    // solhint-disable-next-line func-name-mixedcase
    function CLOCK_MODE() public view virtual returns (string memory);

    /**
     * @inheritdoc IGovernor
     */
    function votingDelay() public view virtual returns (uint256);

    /**
     * @inheritdoc IGovernor
     */
    function votingPeriod() public view virtual returns (uint256);

    /**
     * @inheritdoc IGovernor
     */
    function quorum(uint256 timepoint) public view virtual returns (uint256);
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.0) (governance/extensions/GovernorTimelockControl.sol)

pragma solidity ^0.8.20;

import {IGovernor, Governor} from "../Governor.sol";
import {TimelockController} from "../TimelockController.sol";
import {IERC165} from "../../interfaces/IERC165.sol";
import {SafeCast} from "../../utils/math/SafeCast.sol";

/**
 * @dev Extension of {Governor} that binds the execution process to an instance of {TimelockController}. This adds a
 * delay, enforced by the {TimelockController} to all successful proposal (in addition to the voting duration). The
 * {Governor} needs the proposer (and ideally the executor and canceller) roles for the {Governor} to work properly.
 *
 * Using this model means the proposal will be operated by the {TimelockController} and not by the {Governor}. Thus,
 * the assets and permissions must be attached to the {TimelockController}. Any asset sent to the {Governor} will be
 * inaccessible from a proposal, unless executed via {Governor-relay}.
 *
 * WARNING: Setting up the TimelockController to have additional proposers or cancellers besides the governor is very
 * risky, as it grants them the ability to: 1) execute operations as the timelock, and thus possibly performing
 * operations or accessing funds that are expected to only be accessible through a vote, and 2) block governance
 * proposals that have been approved by the voters, effectively executing a Denial of Service attack.
 */
abstract contract GovernorTimelockControl is Governor {
    TimelockController private _timelock;
    mapping(uint256 proposalId => bytes32) private _timelockIds;

    /**
     * @dev Emitted when the timelock controller used for proposal execution is modified.
     */
    event TimelockChange(address oldTimelock, address newTimelock);

    /**
     * @dev Set the timelock.
     */
    constructor(TimelockController timelockAddress) {
        _updateTimelock(timelockAddress);
    }

    /**
     * @dev Overridden version of the {Governor-state} function that considers the status reported by the timelock.
     */
    function state(uint256 proposalId) public view virtual override returns (ProposalState) {
        ProposalState currentState = super.state(proposalId);

        if (currentState != ProposalState.Queued) {
            return currentState;
        }

        bytes32 queueid = _timelockIds[proposalId];
        if (_timelock.isOperationPending(queueid)) {
            return ProposalState.Queued;
        } else if (_timelock.isOperationDone(queueid)) {
            // This can happen if the proposal is executed directly on the timelock.
            return ProposalState.Executed;
        } else {
            // This can happen if the proposal is canceled directly on the timelock.
            return ProposalState.Canceled;
        }
    }

    /**
     * @dev Public accessor to check the address of the timelock
     */
    function timelock() public view virtual returns (address) {
        return address(_timelock);
    }

    /**
     * @dev See {IGovernor-proposalNeedsQueuing}.
     */
    function proposalNeedsQueuing(uint256) public view virtual override returns (bool) {
        return true;
    }

    /**
     * @dev Function to queue a proposal to the timelock.
     */
    function _queueOperations(
        uint256 proposalId,
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) internal virtual override returns (uint48) {
        uint256 delay = _timelock.getMinDelay();

        bytes32 salt = _timelockSalt(descriptionHash);
        _timelockIds[proposalId] = _timelock.hashOperationBatch(targets, values, calldatas, 0, salt);
        _timelock.scheduleBatch(targets, values, calldatas, 0, salt, delay);

        return SafeCast.toUint48(block.timestamp + delay);
    }

    /**
     * @dev Overridden version of the {Governor-_executeOperations} function that runs the already queued proposal
     * through the timelock.
     */
    function _executeOperations(
        uint256 proposalId,
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) internal virtual override {
        // execute
        _timelock.executeBatch{value: msg.value}(targets, values, calldatas, 0, _timelockSalt(descriptionHash));
        // cleanup for refund
        delete _timelockIds[proposalId];
    }

    /**
     * @dev Overridden version of the {Governor-_cancel} function to cancel the timelocked proposal if it has already
     * been queued.
     */
    // This function can reenter through the external call to the timelock, but we assume the timelock is trusted and
    // well behaved (according to TimelockController) and this will not happen.
    // slither-disable-next-line reentrancy-no-eth
    function _cancel(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) internal virtual override returns (uint256) {
        uint256 proposalId = super._cancel(targets, values, calldatas, descriptionHash);

        bytes32 timelockId = _timelockIds[proposalId];
        if (timelockId != 0) {
            // cancel
            _timelock.cancel(timelockId);
            // cleanup
            delete _timelockIds[proposalId];
        }

        return proposalId;
    }

    /**
     * @dev Address through which the governor executes action. In this case, the timelock.
     */
    function _executor() internal view virtual override returns (address) {
        return address(_timelock);
    }

    /**
     * @dev Public endpoint to update the underlying timelock instance. Restricted to the timelock itself, so updates
     * must be proposed, scheduled, and executed through governance proposals.
     *
     * CAUTION: It is not recommended to change the timelock while there are other queued governance proposals.
     */
    function updateTimelock(TimelockController newTimelock) external virtual onlyGovernance {
        _updateTimelock(newTimelock);
    }

    function _updateTimelock(TimelockController newTimelock) private {
        emit TimelockChange(address(_timelock), address(newTimelock));
        _timelock = newTimelock;
    }

    /**
     * @dev Computes the {TimelockController} operation salt.
     *
     * It is computed with the governor address itself to avoid collisions across governor instances using the
     * same timelock.
     */
    function _timelockSalt(bytes32 descriptionHash) private view returns (bytes32) {
        return bytes20(address(this)) ^ descriptionHash;
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {ISummerGovernorErrors} from "../errors/ISummerGovernorErrors.sol";
import {IGovernor} from "@openzeppelin/contracts/governance/IGovernor.sol";
import {SummerTimelockController} from "../contracts/SummerTimelockController.sol";
import {ISummerToken} from "./ISummerToken.sol";
import {IProtocolAccessManager} from "@summerfi/access-contracts/interfaces/IProtocolAccessManager.sol";
import {IVotes} from "@openzeppelin/contracts/governance/extensions/GovernorVotes.sol";
/**
 * @title ISummerGovernorV2 Interface
 * @notice Governance V2 hub-and-satellite interface extending OZ Governor. Voting occurs on the hub with xSUMR.
 *         Finalized proposals may be distributed cross-chain to satellites via LayerZero and queued for execution.
 * @dev Key behaviors:
 *      - Hub-only: propose, castVote, execute, cancel. Satellite-only: receive and queue cross-chain proposals.
 *      - ETH receive is restricted to LayerZero endpoint or timelock executors to avoid accidental funding.
 *      - Guardians (via ProtocolAccessManager) can propose below threshold and have scoped cancellation privileges.
 */
interface ISummerGovernorV2 is IGovernor, ISummerGovernorErrors {
    /*
     * @dev Struct for the governor parameters
     * @param token The token contract address
     * @param timelock The timelock controller contract address
     * @param accessManager The access manager contract address
     * @param votingDelay The voting delay in seconds
     * @param votingPeriod The voting period in seconds
     * @param proposalThreshold The proposal threshold in tokens
     * @param quorumFraction The quorum fraction
     * @param endpoint The LayerZero endpoint address
     * @param hubChainId The hub chain ID
     * @param initialOwner The initial owner of the contract
     */
    /**
     * @notice Deployment parameters for the governor.
     * @dev `proposalThreshold` is validated within [MIN_PROPOSAL_THRESHOLD; MAX_PROPOSAL_THRESHOLD].
     */
    struct GovernorParams {
        IVotes token;
        SummerTimelockController timelock;
        address accessManager;
        uint48 votingDelay;
        uint32 votingPeriod;
        uint256 proposalThreshold;
        uint256 quorumFraction;
        address endpoint;
        uint32 hubChainId;
        address initialOwner;
    }

    /**
     * @notice Emitted when a proposal is sent cross-chain to a destination endpoint.
     * @param proposalId Proposal ID on the destination chain (hash of destination data)
     * @param dstEid Destination LayerZero Endpoint ID
     */
    event ProposalSentCrossChain(
        uint256 indexed proposalId,
        uint32 indexed dstEid
    );

    /**
     * @notice Emitted when a cross-chain proposal is received via LayerZero.
     * @param proposalId Proposal ID computed from destination payload
     * @param srcEid Source LayerZero Endpoint ID
     */
    event ProposalReceivedCrossChain(
        uint256 indexed proposalId,
        uint32 indexed srcEid
    );

    /**
     * @notice Casts a vote for a proposal on the hub chain.
     * @param proposalId The proposal to vote on
     * @param support 0 = Against, 1 = For, 2 = Abstain
     * @return proposalId The proposal ID (echo)
     */
    function castVote(
        uint256 proposalId,
        uint8 support
    ) external returns (uint256);

    /**
     * @notice Creates a new proposal. Hub-only.
     * @param targets Call targets
     * @param values ETH values for each call
     * @param calldatas Calldata payloads
     * @param description Human-readable description
     * @return proposalId New proposal ID
     */
    function propose(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        string memory description
    ) external override(IGovernor) returns (uint256 proposalId);

    /**
     * @notice Executes a successful proposal from the hub chain. Timelock-enforced.
     * @param targets Call targets
     * @param values ETH values
     * @param calldatas Calldata payloads
     * @param descriptionHash EIP-712 description hash
     * @return proposalId Executed proposal ID
     */
    function execute(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) external payable override(IGovernor) returns (uint256 proposalId);

    /**
     * @notice Cancels a proposal. Hub-only. Guardians have scoped privileges.
     * @param targets Call targets
     * @param values ETH values
     * @param calldatas Calldata payloads
     * @param descriptionHash EIP-712 description hash
     * @return proposalId Cancelled proposal ID
     */
    function cancel(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) external override(IGovernor) returns (uint256 proposalId);

    /**
     * @notice Sends a finalized proposal to a satellite for queuing and execution via LayerZero.
     * @param _dstEid Destination Endpoint ID
     * @param _dstTargets Targets for destination chain
     * @param _dstValues ETH values for destination chain
     * @param _dstCalldatas Calldata payloads for destination chain
     * @param _dstDescriptionHash EIP-712 description hash for destination chain
     * @param _options LayerZero executor options
     */
    function sendProposalToTargetChain(
        uint32 _dstEid,
        address[] memory _dstTargets,
        uint256[] memory _dstValues,
        bytes[] memory _dstCalldatas,
        bytes32 _dstDescriptionHash,
        bytes calldata _options
    ) external;

    /**
     * @notice Returns whether an account is an active guardian according to the ProtocolAccessManager.
     * @param account Address to query
     * @return isGuardian True if the account is an active guardian
     */
    function isActiveGuardian(
        address account
    ) external view returns (bool isGuardian);
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

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.0) (governance/IGovernor.sol)

pragma solidity ^0.8.20;

import {IERC165} from "../interfaces/IERC165.sol";
import {IERC6372} from "../interfaces/IERC6372.sol";

/**
 * @dev Interface of the {Governor} core.
 *
 * NOTE: Event parameters lack the `indexed` keyword for compatibility with GovernorBravo events.
 * Making event parameters `indexed` affects how events are decoded, potentially breaking existing indexers.
 */
interface IGovernor is IERC165, IERC6372 {
    enum ProposalState {
        Pending,
        Active,
        Canceled,
        Defeated,
        Succeeded,
        Queued,
        Expired,
        Executed
    }

    /**
     * @dev Empty proposal or a mismatch between the parameters length for a proposal call.
     */
    error GovernorInvalidProposalLength(uint256 targets, uint256 calldatas, uint256 values);

    /**
     * @dev The vote was already cast.
     */
    error GovernorAlreadyCastVote(address voter);

    /**
     * @dev Token deposits are disabled in this contract.
     */
    error GovernorDisabledDeposit();

    /**
     * @dev The `account` is not a proposer.
     */
    error GovernorOnlyProposer(address account);

    /**
     * @dev The `account` is not the governance executor.
     */
    error GovernorOnlyExecutor(address account);

    /**
     * @dev The `proposalId` doesn't exist.
     */
    error GovernorNonexistentProposal(uint256 proposalId);

    /**
     * @dev The current state of a proposal is not the required for performing an operation.
     * The `expectedStates` is a bitmap with the bits enabled for each ProposalState enum position
     * counting from right to left.
     *
     * NOTE: If `expectedState` is `bytes32(0)`, the proposal is expected to not be in any state (i.e. not exist).
     * This is the case when a proposal that is expected to be unset is already initiated (the proposal is duplicated).
     *
     * See {Governor-_encodeStateBitmap}.
     */
    error GovernorUnexpectedProposalState(uint256 proposalId, ProposalState current, bytes32 expectedStates);

    /**
     * @dev The voting period set is not a valid period.
     */
    error GovernorInvalidVotingPeriod(uint256 votingPeriod);

    /**
     * @dev The `proposer` does not have the required votes to create a proposal.
     */
    error GovernorInsufficientProposerVotes(address proposer, uint256 votes, uint256 threshold);

    /**
     * @dev The `proposer` is not allowed to create a proposal.
     */
    error GovernorRestrictedProposer(address proposer);

    /**
     * @dev The vote type used is not valid for the corresponding counting module.
     */
    error GovernorInvalidVoteType();

    /**
     * @dev The provided params buffer is not supported by the counting module.
     */
    error GovernorInvalidVoteParams();

    /**
     * @dev Queue operation is not implemented for this governor. Execute should be called directly.
     */
    error GovernorQueueNotImplemented();

    /**
     * @dev The proposal hasn't been queued yet.
     */
    error GovernorNotQueuedProposal(uint256 proposalId);

    /**
     * @dev The proposal has already been queued.
     */
    error GovernorAlreadyQueuedProposal(uint256 proposalId);

    /**
     * @dev The provided signature is not valid for the expected `voter`.
     * If the `voter` is a contract, the signature is not valid using {IERC1271-isValidSignature}.
     */
    error GovernorInvalidSignature(address voter);

    /**
     * @dev Emitted when a proposal is created.
     */
    event ProposalCreated(
        uint256 proposalId,
        address proposer,
        address[] targets,
        uint256[] values,
        string[] signatures,
        bytes[] calldatas,
        uint256 voteStart,
        uint256 voteEnd,
        string description
    );

    /**
     * @dev Emitted when a proposal is queued.
     */
    event ProposalQueued(uint256 proposalId, uint256 etaSeconds);

    /**
     * @dev Emitted when a proposal is executed.
     */
    event ProposalExecuted(uint256 proposalId);

    /**
     * @dev Emitted when a proposal is canceled.
     */
    event ProposalCanceled(uint256 proposalId);

    /**
     * @dev Emitted when a vote is cast without params.
     *
     * Note: `support` values should be seen as buckets. Their interpretation depends on the voting module used.
     */
    event VoteCast(address indexed voter, uint256 proposalId, uint8 support, uint256 weight, string reason);

    /**
     * @dev Emitted when a vote is cast with params.
     *
     * Note: `support` values should be seen as buckets. Their interpretation depends on the voting module used.
     * `params` are additional encoded parameters. Their interpretation  also depends on the voting module used.
     */
    event VoteCastWithParams(
        address indexed voter,
        uint256 proposalId,
        uint8 support,
        uint256 weight,
        string reason,
        bytes params
    );

    /**
     * @notice module:core
     * @dev Name of the governor instance (used in building the EIP-712 domain separator).
     */
    function name() external view returns (string memory);

    /**
     * @notice module:core
     * @dev Version of the governor instance (used in building the EIP-712 domain separator). Default: "1"
     */
    function version() external view returns (string memory);

    /**
     * @notice module:voting
     * @dev A description of the possible `support` values for {castVote} and the way these votes are counted, meant to
     * be consumed by UIs to show correct vote options and interpret the results. The string is a URL-encoded sequence of
     * key-value pairs that each describe one aspect, for example `support=bravo&quorum=for,abstain`.
     *
     * There are 2 standard keys: `support` and `quorum`.
     *
     * - `support=bravo` refers to the vote options 0 = Against, 1 = For, 2 = Abstain, as in `GovernorBravo`.
     * - `quorum=bravo` means that only For votes are counted towards quorum.
     * - `quorum=for,abstain` means that both For and Abstain votes are counted towards quorum.
     *
     * If a counting module makes use of encoded `params`, it should  include this under a `params` key with a unique
     * name that describes the behavior. For example:
     *
     * - `params=fractional` might refer to a scheme where votes are divided fractionally between for/against/abstain.
     * - `params=erc721` might refer to a scheme where specific NFTs are delegated to vote.
     *
     * NOTE: The string can be decoded by the standard
     * https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams[`URLSearchParams`]
     * JavaScript class.
     */
    // solhint-disable-next-line func-name-mixedcase
    function COUNTING_MODE() external view returns (string memory);

    /**
     * @notice module:core
     * @dev Hashing function used to (re)build the proposal id from the proposal details..
     */
    function hashProposal(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) external pure returns (uint256);

    /**
     * @notice module:core
     * @dev Current state of a proposal, following Compound's convention
     */
    function state(uint256 proposalId) external view returns (ProposalState);

    /**
     * @notice module:core
     * @dev The number of votes required in order for a voter to become a proposer.
     */
    function proposalThreshold() external view returns (uint256);

    /**
     * @notice module:core
     * @dev Timepoint used to retrieve user's votes and quorum. If using block number (as per Compound's Comp), the
     * snapshot is performed at the end of this block. Hence, voting for this proposal starts at the beginning of the
     * following block.
     */
    function proposalSnapshot(uint256 proposalId) external view returns (uint256);

    /**
     * @notice module:core
     * @dev Timepoint at which votes close. If using block number, votes close at the end of this block, so it is
     * possible to cast a vote during this block.
     */
    function proposalDeadline(uint256 proposalId) external view returns (uint256);

    /**
     * @notice module:core
     * @dev The account that created a proposal.
     */
    function proposalProposer(uint256 proposalId) external view returns (address);

    /**
     * @notice module:core
     * @dev The time when a queued proposal becomes executable ("ETA"). Unlike {proposalSnapshot} and
     * {proposalDeadline}, this doesn't use the governor clock, and instead relies on the executor's clock which may be
     * different. In most cases this will be a timestamp.
     */
    function proposalEta(uint256 proposalId) external view returns (uint256);

    /**
     * @notice module:core
     * @dev Whether a proposal needs to be queued before execution.
     */
    function proposalNeedsQueuing(uint256 proposalId) external view returns (bool);

    /**
     * @notice module:user-config
     * @dev Delay, between the proposal is created and the vote starts. The unit this duration is expressed in depends
     * on the clock (see ERC-6372) this contract uses.
     *
     * This can be increased to leave time for users to buy voting power, or delegate it, before the voting of a
     * proposal starts.
     *
     * NOTE: While this interface returns a uint256, timepoints are stored as uint48 following the ERC-6372 clock type.
     * Consequently this value must fit in a uint48 (when added to the current clock). See {IERC6372-clock}.
     */
    function votingDelay() external view returns (uint256);

    /**
     * @notice module:user-config
     * @dev Delay between the vote start and vote end. The unit this duration is expressed in depends on the clock
     * (see ERC-6372) this contract uses.
     *
     * NOTE: The {votingDelay} can delay the start of the vote. This must be considered when setting the voting
     * duration compared to the voting delay.
     *
     * NOTE: This value is stored when the proposal is submitted so that possible changes to the value do not affect
     * proposals that have already been submitted. The type used to save it is a uint32. Consequently, while this
     * interface returns a uint256, the value it returns should fit in a uint32.
     */
    function votingPeriod() external view returns (uint256);

    /**
     * @notice module:user-config
     * @dev Minimum number of cast voted required for a proposal to be successful.
     *
     * NOTE: The `timepoint` parameter corresponds to the snapshot used for counting vote. This allows to scale the
     * quorum depending on values such as the totalSupply of a token at this timepoint (see {ERC20Votes}).
     */
    function quorum(uint256 timepoint) external view returns (uint256);

    /**
     * @notice module:reputation
     * @dev Voting power of an `account` at a specific `timepoint`.
     *
     * Note: this can be implemented in a number of ways, for example by reading the delegated balance from one (or
     * multiple), {ERC20Votes} tokens.
     */
    function getVotes(address account, uint256 timepoint) external view returns (uint256);

    /**
     * @notice module:reputation
     * @dev Voting power of an `account` at a specific `timepoint` given additional encoded parameters.
     */
    function getVotesWithParams(
        address account,
        uint256 timepoint,
        bytes memory params
    ) external view returns (uint256);

    /**
     * @notice module:voting
     * @dev Returns whether `account` has cast a vote on `proposalId`.
     */
    function hasVoted(uint256 proposalId, address account) external view returns (bool);

    /**
     * @dev Create a new proposal. Vote start after a delay specified by {IGovernor-votingDelay} and lasts for a
     * duration specified by {IGovernor-votingPeriod}.
     *
     * Emits a {ProposalCreated} event.
     *
     * NOTE: The state of the Governor and `targets` may change between the proposal creation and its execution.
     * This may be the result of third party actions on the targeted contracts, or other governor proposals.
     * For example, the balance of this contract could be updated or its access control permissions may be modified,
     * possibly compromising the proposal's ability to execute successfully (e.g. the governor doesn't have enough
     * value to cover a proposal with multiple transfers).
     */
    function propose(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        string memory description
    ) external returns (uint256 proposalId);

    /**
     * @dev Queue a proposal. Some governors require this step to be performed before execution can happen. If queuing
     * is not necessary, this function may revert.
     * Queuing a proposal requires the quorum to be reached, the vote to be successful, and the deadline to be reached.
     *
     * Emits a {ProposalQueued} event.
     */
    function queue(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) external returns (uint256 proposalId);

    /**
     * @dev Execute a successful proposal. This requires the quorum to be reached, the vote to be successful, and the
     * deadline to be reached. Depending on the governor it might also be required that the proposal was queued and
     * that some delay passed.
     *
     * Emits a {ProposalExecuted} event.
     *
     * NOTE: Some modules can modify the requirements for execution, for example by adding an additional timelock.
     */
    function execute(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) external payable returns (uint256 proposalId);

    /**
     * @dev Cancel a proposal. A proposal is cancellable by the proposer, but only while it is Pending state, i.e.
     * before the vote starts.
     *
     * Emits a {ProposalCanceled} event.
     */
    function cancel(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        bytes32 descriptionHash
    ) external returns (uint256 proposalId);

    /**
     * @dev Cast a vote
     *
     * Emits a {VoteCast} event.
     */
    function castVote(uint256 proposalId, uint8 support) external returns (uint256 balance);

    /**
     * @dev Cast a vote with a reason
     *
     * Emits a {VoteCast} event.
     */
    function castVoteWithReason(
        uint256 proposalId,
        uint8 support,
        string calldata reason
    ) external returns (uint256 balance);

    /**
     * @dev Cast a vote with a reason and additional encoded parameters
     *
     * Emits a {VoteCast} or {VoteCastWithParams} event depending on the length of params.
     */
    function castVoteWithReasonAndParams(
        uint256 proposalId,
        uint8 support,
        string calldata reason,
        bytes memory params
    ) external returns (uint256 balance);

    /**
     * @dev Cast a vote using the voter's signature, including ERC-1271 signature support.
     *
     * Emits a {VoteCast} event.
     */
    function castVoteBySig(
        uint256 proposalId,
        uint8 support,
        address voter,
        bytes memory signature
    ) external returns (uint256 balance);

    /**
     * @dev Cast a vote with a reason and additional encoded parameters using the voter's signature,
     * including ERC-1271 signature support.
     *
     * Emits a {VoteCast} or {VoteCastWithParams} event depending on the length of params.
     */
    function castVoteWithReasonAndParamsBySig(
        uint256 proposalId,
        uint8 support,
        address voter,
        string calldata reason,
        bytes memory params,
        bytes memory signature
    ) external returns (uint256 balance);
}

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
// OpenZeppelin Contracts (last updated v5.0.0) (governance/extensions/GovernorSettings.sol)

pragma solidity ^0.8.20;

import {Governor} from "../Governor.sol";

/**
 * @dev Extension of {Governor} for settings updatable through governance.
 */
abstract contract GovernorSettings is Governor {
    // amount of token
    uint256 private _proposalThreshold;
    // timepoint: limited to uint48 in core (same as clock() type)
    uint48 private _votingDelay;
    // duration: limited to uint32 in core
    uint32 private _votingPeriod;

    event VotingDelaySet(uint256 oldVotingDelay, uint256 newVotingDelay);
    event VotingPeriodSet(uint256 oldVotingPeriod, uint256 newVotingPeriod);
    event ProposalThresholdSet(uint256 oldProposalThreshold, uint256 newProposalThreshold);

    /**
     * @dev Initialize the governance parameters.
     */
    constructor(uint48 initialVotingDelay, uint32 initialVotingPeriod, uint256 initialProposalThreshold) {
        _setVotingDelay(initialVotingDelay);
        _setVotingPeriod(initialVotingPeriod);
        _setProposalThreshold(initialProposalThreshold);
    }

    /**
     * @dev See {IGovernor-votingDelay}.
     */
    function votingDelay() public view virtual override returns (uint256) {
        return _votingDelay;
    }

    /**
     * @dev See {IGovernor-votingPeriod}.
     */
    function votingPeriod() public view virtual override returns (uint256) {
        return _votingPeriod;
    }

    /**
     * @dev See {Governor-proposalThreshold}.
     */
    function proposalThreshold() public view virtual override returns (uint256) {
        return _proposalThreshold;
    }

    /**
     * @dev Update the voting delay. This operation can only be performed through a governance proposal.
     *
     * Emits a {VotingDelaySet} event.
     */
    function setVotingDelay(uint48 newVotingDelay) public virtual onlyGovernance {
        _setVotingDelay(newVotingDelay);
    }

    /**
     * @dev Update the voting period. This operation can only be performed through a governance proposal.
     *
     * Emits a {VotingPeriodSet} event.
     */
    function setVotingPeriod(uint32 newVotingPeriod) public virtual onlyGovernance {
        _setVotingPeriod(newVotingPeriod);
    }

    /**
     * @dev Update the proposal threshold. This operation can only be performed through a governance proposal.
     *
     * Emits a {ProposalThresholdSet} event.
     */
    function setProposalThreshold(uint256 newProposalThreshold) public virtual onlyGovernance {
        _setProposalThreshold(newProposalThreshold);
    }

    /**
     * @dev Internal setter for the voting delay.
     *
     * Emits a {VotingDelaySet} event.
     */
    function _setVotingDelay(uint48 newVotingDelay) internal virtual {
        emit VotingDelaySet(_votingDelay, newVotingDelay);
        _votingDelay = newVotingDelay;
    }

    /**
     * @dev Internal setter for the voting period.
     *
     * Emits a {VotingPeriodSet} event.
     */
    function _setVotingPeriod(uint32 newVotingPeriod) internal virtual {
        if (newVotingPeriod == 0) {
            revert GovernorInvalidVotingPeriod(0);
        }
        emit VotingPeriodSet(_votingPeriod, newVotingPeriod);
        _votingPeriod = newVotingPeriod;
    }

    /**
     * @dev Internal setter for the proposal threshold.
     *
     * Emits a {ProposalThresholdSet} event.
     */
    function _setProposalThreshold(uint256 newProposalThreshold) internal virtual {
        emit ProposalThresholdSet(_proposalThreshold, newProposalThreshold);
        _proposalThreshold = newProposalThreshold;
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.0) (utils/Nonces.sol)
pragma solidity ^0.8.20;

/**
 * @dev Provides tracking nonces for addresses. Nonces will only increment.
 */
abstract contract Nonces {
    /**
     * @dev The nonce used for an `account` is not the expected current nonce.
     */
    error InvalidAccountNonce(address account, uint256 currentNonce);

    mapping(address account => uint256) private _nonces;

    /**
     * @dev Returns the next unused nonce for an address.
     */
    function nonces(address owner) public view virtual returns (uint256) {
        return _nonces[owner];
    }

    /**
     * @dev Consumes a nonce.
     *
     * Returns the current value and increments nonce.
     */
    function _useNonce(address owner) internal virtual returns (uint256) {
        // For each account, the nonce has an initial value of 0, can only be incremented by one, and cannot be
        // decremented or reset. This guarantees that the nonce never overflows.
        unchecked {
            // It is important to do x++ and not ++x here.
            return _nonces[owner]++;
        }
    }

    /**
     * @dev Same as {_useNonce} but checking that `nonce` is the next valid for `owner`.
     */
    function _useCheckedNonce(address owner, uint256 nonce) internal virtual {
        uint256 current = _useNonce(owner);
        if (nonce != current) {
            revert InvalidAccountNonce(owner, current);
        }
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.0) (governance/extensions/GovernorVotesQuorumFraction.sol)

pragma solidity ^0.8.20;

import {GovernorVotes} from "./GovernorVotes.sol";
import {SafeCast} from "../../utils/math/SafeCast.sol";
import {Checkpoints} from "../../utils/structs/Checkpoints.sol";

/**
 * @dev Extension of {Governor} for voting weight extraction from an {ERC20Votes} token and a quorum expressed as a
 * fraction of the total supply.
 */
abstract contract GovernorVotesQuorumFraction is GovernorVotes {
    using Checkpoints for Checkpoints.Trace208;

    Checkpoints.Trace208 private _quorumNumeratorHistory;

    event QuorumNumeratorUpdated(uint256 oldQuorumNumerator, uint256 newQuorumNumerator);

    /**
     * @dev The quorum set is not a valid fraction.
     */
    error GovernorInvalidQuorumFraction(uint256 quorumNumerator, uint256 quorumDenominator);

    /**
     * @dev Initialize quorum as a fraction of the token's total supply.
     *
     * The fraction is specified as `numerator / denominator`. By default the denominator is 100, so quorum is
     * specified as a percent: a numerator of 10 corresponds to quorum being 10% of total supply. The denominator can be
     * customized by overriding {quorumDenominator}.
     */
    constructor(uint256 quorumNumeratorValue) {
        _updateQuorumNumerator(quorumNumeratorValue);
    }

    /**
     * @dev Returns the current quorum numerator. See {quorumDenominator}.
     */
    function quorumNumerator() public view virtual returns (uint256) {
        return _quorumNumeratorHistory.latest();
    }

    /**
     * @dev Returns the quorum numerator at a specific timepoint. See {quorumDenominator}.
     */
    function quorumNumerator(uint256 timepoint) public view virtual returns (uint256) {
        uint256 length = _quorumNumeratorHistory._checkpoints.length;

        // Optimistic search, check the latest checkpoint
        Checkpoints.Checkpoint208 storage latest = _quorumNumeratorHistory._checkpoints[length - 1];
        uint48 latestKey = latest._key;
        uint208 latestValue = latest._value;
        if (latestKey <= timepoint) {
            return latestValue;
        }

        // Otherwise, do the binary search
        return _quorumNumeratorHistory.upperLookupRecent(SafeCast.toUint48(timepoint));
    }

    /**
     * @dev Returns the quorum denominator. Defaults to 100, but may be overridden.
     */
    function quorumDenominator() public view virtual returns (uint256) {
        return 100;
    }

    /**
     * @dev Returns the quorum for a timepoint, in terms of number of votes: `supply * numerator / denominator`.
     */
    function quorum(uint256 timepoint) public view virtual override returns (uint256) {
        return (token().getPastTotalSupply(timepoint) * quorumNumerator(timepoint)) / quorumDenominator();
    }

    /**
     * @dev Changes the quorum numerator.
     *
     * Emits a {QuorumNumeratorUpdated} event.
     *
     * Requirements:
     *
     * - Must be called through a governance proposal.
     * - New numerator must be smaller or equal to the denominator.
     */
    function updateQuorumNumerator(uint256 newQuorumNumerator) external virtual onlyGovernance {
        _updateQuorumNumerator(newQuorumNumerator);
    }

    /**
     * @dev Changes the quorum numerator.
     *
     * Emits a {QuorumNumeratorUpdated} event.
     *
     * Requirements:
     *
     * - New numerator must be smaller or equal to the denominator.
     */
    function _updateQuorumNumerator(uint256 newQuorumNumerator) internal virtual {
        uint256 denominator = quorumDenominator();
        if (newQuorumNumerator > denominator) {
            revert GovernorInvalidQuorumFraction(newQuorumNumerator, denominator);
        }

        uint256 oldQuorumNumerator = quorumNumerator();
        _quorumNumeratorHistory.push(clock(), SafeCast.toUint208(newQuorumNumerator));

        emit QuorumNumeratorUpdated(oldQuorumNumerator, newQuorumNumerator);
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.0) (governance/extensions/GovernorCountingSimple.sol)

pragma solidity ^0.8.20;

import {Governor} from "../Governor.sol";

/**
 * @dev Extension of {Governor} for simple, 3 options, vote counting.
 */
abstract contract GovernorCountingSimple is Governor {
    /**
     * @dev Supported vote types. Matches Governor Bravo ordering.
     */
    enum VoteType {
        Against,
        For,
        Abstain
    }

    struct ProposalVote {
        uint256 againstVotes;
        uint256 forVotes;
        uint256 abstainVotes;
        mapping(address voter => bool) hasVoted;
    }

    mapping(uint256 proposalId => ProposalVote) private _proposalVotes;

    /**
     * @dev See {IGovernor-COUNTING_MODE}.
     */
    // solhint-disable-next-line func-name-mixedcase
    function COUNTING_MODE() public pure virtual override returns (string memory) {
        return "support=bravo&quorum=for,abstain";
    }

    /**
     * @dev See {IGovernor-hasVoted}.
     */
    function hasVoted(uint256 proposalId, address account) public view virtual override returns (bool) {
        return _proposalVotes[proposalId].hasVoted[account];
    }

    /**
     * @dev Accessor to the internal vote counts.
     */
    function proposalVotes(
        uint256 proposalId
    ) public view virtual returns (uint256 againstVotes, uint256 forVotes, uint256 abstainVotes) {
        ProposalVote storage proposalVote = _proposalVotes[proposalId];
        return (proposalVote.againstVotes, proposalVote.forVotes, proposalVote.abstainVotes);
    }

    /**
     * @dev See {Governor-_quorumReached}.
     */
    function _quorumReached(uint256 proposalId) internal view virtual override returns (bool) {
        ProposalVote storage proposalVote = _proposalVotes[proposalId];

        return quorum(proposalSnapshot(proposalId)) <= proposalVote.forVotes + proposalVote.abstainVotes;
    }

    /**
     * @dev See {Governor-_voteSucceeded}. In this module, the forVotes must be strictly over the againstVotes.
     */
    function _voteSucceeded(uint256 proposalId) internal view virtual override returns (bool) {
        ProposalVote storage proposalVote = _proposalVotes[proposalId];

        return proposalVote.forVotes > proposalVote.againstVotes;
    }

    /**
     * @dev See {Governor-_countVote}. In this module, the support follows the `VoteType` enum (from Governor Bravo).
     */
    function _countVote(
        uint256 proposalId,
        address account,
        uint8 support,
        uint256 totalWeight,
        bytes memory // params
    ) internal virtual override returns (uint256) {
        ProposalVote storage proposalVote = _proposalVotes[proposalId];

        if (proposalVote.hasVoted[account]) {
            revert GovernorAlreadyCastVote(account);
        }
        proposalVote.hasVoted[account] = true;

        if (support == uint8(VoteType.Against)) {
            proposalVote.againstVotes += totalWeight;
        } else if (support == uint8(VoteType.For)) {
            proposalVote.forVotes += totalWeight;
        } else if (support == uint8(VoteType.Abstain)) {
            proposalVote.abstainVotes += totalWeight;
        } else {
            revert GovernorInvalidVoteType();
        }

        return totalWeight;
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.0) (access/Ownable.sol)

pragma solidity ^0.8.20;

import {Context} from "../utils/Context.sol";

/**
 * @dev Contract module which provides a basic access control mechanism, where
 * there is an account (an owner) that can be granted exclusive access to
 * specific functions.
 *
 * The initial owner is set to the address provided by the deployer. This can
 * later be changed with {transferOwnership}.
 *
 * This module is used through inheritance. It will make available the modifier
 * `onlyOwner`, which can be applied to your functions to restrict their use to
 * the owner.
 */
abstract contract Ownable is Context {
    address private _owner;

    /**
     * @dev The caller account is not authorized to perform an operation.
     */
    error OwnableUnauthorizedAccount(address account);

    /**
     * @dev The owner is not a valid owner account. (eg. `address(0)`)
     */
    error OwnableInvalidOwner(address owner);

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    /**
     * @dev Initializes the contract setting the address provided by the deployer as the initial owner.
     */
    constructor(address initialOwner) {
        if (initialOwner == address(0)) {
            revert OwnableInvalidOwner(address(0));
        }
        _transferOwnership(initialOwner);
    }

    /**
     * @dev Throws if called by any account other than the owner.
     */
    modifier onlyOwner() {
        _checkOwner();
        _;
    }

    /**
     * @dev Returns the address of the current owner.
     */
    function owner() public view virtual returns (address) {
        return _owner;
    }

    /**
     * @dev Throws if the sender is not the owner.
     */
    function _checkOwner() internal view virtual {
        if (owner() != _msgSender()) {
            revert OwnableUnauthorizedAccount(_msgSender());
        }
    }

    /**
     * @dev Leaves the contract without owner. It will not be possible to call
     * `onlyOwner` functions. Can only be called by the current owner.
     *
     * NOTE: Renouncing ownership will leave the contract without an owner,
     * thereby disabling any functionality that is only available to the owner.
     */
    function renounceOwnership() public virtual onlyOwner {
        _transferOwnership(address(0));
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    function transferOwnership(address newOwner) public virtual onlyOwner {
        if (newOwner == address(0)) {
            revert OwnableInvalidOwner(address(0));
        }
        _transferOwnership(newOwner);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Internal function without access restriction.
     */
    function _transferOwnership(address newOwner) internal virtual {
        address oldOwner = _owner;
        _owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
    }
}


END OF SUPPORTING CONTRACTS AND INTERFACES
