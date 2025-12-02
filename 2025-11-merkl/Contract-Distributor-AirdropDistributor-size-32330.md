
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: BUSL-1.1

pragma solidity ^0.8.17;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { SafeCast } from "@openzeppelin/contracts/utils/math/SafeCast.sol";

import { UUPSHelper } from "./utils/UUPSHelper.sol";
import { IAccessControlManager } from "./interfaces/IAccessControlManager.sol";
import { Errors } from "./utils/Errors.sol";
import { IClaimRecipient } from "./interfaces/IClaimRecipient.sol";

struct MerkleTree {
    /// @notice Root of a Merkle tree whose leaves are `(address user, address token, uint amount)`
    /// representing the cumulative amount of tokens earned by each user
    /// @dev The Merkle tree contains only monotonically increasing amounts: if a user previously claimed 1 token,
    /// subsequent tree updates should show amounts x > 1 for that user
    bytes32 merkleRoot;
    /// @dev Deprecated: this used to be the IPFS hash of the complete tree data
    bytes32 ipfsHash;
}

struct Claim {
    /// @notice Cumulative amount claimed by the user for this token
    uint208 amount;
    /// @notice Timestamp of the last claim
    uint48 timestamp;
    /// @notice Merkle root that was active when the last claim occurred
    bytes32 merkleRoot;
}

/// @title Distributor
/// @notice Manages the distribution of Merkl rewards and allows users to claim their earned tokens
/// @dev Implements a Merkle tree-based reward distribution system with dispute resolution mechanism
/// @author Merkl SAS
contract Distributor is UUPSHelper {
    using SafeERC20 for IERC20;

    /// @notice Default epoch duration in seconds (1 hour)
    uint32 internal constant _EPOCH_DURATION = 3600;

    /// @notice Success message that must be returned by `IClaimRecipient.onClaim` callback
    bytes32 public constant CALLBACK_SUCCESS = keccak256("IClaimRecipient.onClaim");

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                       VARIABLES                                                    
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    /// @notice Current active Merkle tree containing claimable token data
    MerkleTree public tree;

    /// @notice Previous Merkle tree that was active before the last update
    /// @dev Used to revert to if the current tree is disputed and found invalid
    MerkleTree public lastTree;

    /// @notice Token required as a deposit to dispute a tree update
    IERC20 public disputeToken;

    /// @notice Access control manager contract handling role-based permissions
    IAccessControlManager public accessControlManager;

    /// @notice Address that created the current ongoing dispute
    /// @dev Non-zero value indicates there is an active dispute
    address public disputer;

    /// @notice Timestamp after which the current tree becomes effective and undisputable
    uint48 public endOfDisputePeriod;

    /// @notice Number of epochs (in EPOCH_DURATION units) to wait before a tree update becomes effective
    uint48 public disputePeriod;

    /// @notice Amount of disputeToken required to create a dispute
    uint256 public disputeAmount;

    /// @notice Tracks cumulative claimed amounts for each user and token
    /// @dev Maps user => token => Claim details (amount, timestamp, merkleRoot)
    mapping(address => mapping(address => Claim)) public claimed;

    /// @notice Trusted addresses authorized to update the Merkle root
    /// @dev 1 = trusted, 0 = not trusted
    mapping(address => uint256) public canUpdateMerkleRoot;

    /// @notice Deprecated - kept for storage layout compatibility
    mapping(address => uint256) public onlyOperatorCanClaim;

    /// @notice Authorization for operators to claim on behalf of users
    /// @dev Maps user => operator => authorization status (1 = authorized, 0 = not authorized)
    mapping(address => mapping(address => uint256)) public operators;

    /// @notice Whether contract upgradeability has been permanently disabled
    /// @dev 1 = upgrades disabled, 0 = upgrades allowed
    uint128 public upgradeabilityDeactivated;

    /// @notice Reentrancy guard status
    /// @dev 1 = not entered, 2 = entered
    uint96 private _status;

    /// @notice Custom epoch duration for dispute periods in seconds
    /// @dev If 0, defaults to _EPOCH_DURATION
    uint32 internal _epochDuration;

    /// @notice Custom recipient addresses for user claims per token
    /// @dev Maps user => token => recipient address (zero address = use default behavior)
    /// @dev Setting recipient for address(0) token sets the default recipient for all tokens
    mapping(address => mapping(address => address)) public claimRecipient;

    /// @notice Global operators authorized to claim specific tokens on behalf of any user
    /// @dev Maps operator => token => authorization (1 = authorized, 0 = not authorized)
    /// @dev Authorization for address(0) token allows claiming any token for any user
    mapping(address => mapping(address => uint256)) public mainOperators;

    uint256[35] private __gap;

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                        EVENTS                                                      
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    event Claimed(address indexed user, address indexed token, uint256 amount);
    event ClaimRecipientUpdated(address indexed user, address indexed token, address indexed recipient);
    event DisputeAmountUpdated(uint256 _disputeAmount);
    event Disputed(string reason);
    event DisputePeriodUpdated(uint48 _disputePeriod);
    event DisputeResolved(bool valid);
    event DisputeTokenUpdated(address indexed _disputeToken);
    event EpochDurationUpdated(uint32 newEpochDuration);
    event MainOperatorStatusUpdated(address indexed operator, address indexed token, bool isWhitelisted);
    event OperatorClaimingToggled(address indexed user, bool isEnabled);
    event OperatorToggled(address indexed user, address indexed operator, bool isWhitelisted);
    event Recovered(address indexed token, address indexed to, uint256 amount);
    event Revoked(); // With this event an indexer could maintain a table (timestamp, merkleRootUpdate)
    event TreeUpdated(bytes32 merkleRoot, bytes32 ipfsHash, uint48 endOfDisputePeriod);
    event TrustedToggled(address indexed eoa, bool trust);
    event UpgradeabilityRevoked();

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                       MODIFIERS                                                    
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    /// @notice Restricts function access to addresses with governor role only
    modifier onlyGovernor() {
        _onlyGovernor();
        _;
    }

    /// @notice Restricts function access to addresses with governor or guardian role
    modifier onlyGuardian() {
        _onlyGuardian();
        _;
    }

    /// @notice Ensures the contract is still upgradeable and caller has governor role
    /// @dev Reverts if upgradeability has been revoked or caller is not a governor
    modifier onlyUpgradeableInstance() {
        if (upgradeabilityDeactivated == 1) revert Errors.NotUpgradeable();
        else if (!accessControlManager.isGovernor(msg.sender)) revert Errors.NotGovernor();
        _;
    }

    /// @notice Prevents reentrancy attacks by locking the contract during execution
    /// @dev Uses a status flag that is set to 2 during execution and reset to 1 after
    modifier nonReentrant() {
        if (_status == 2) revert Errors.ReentrantCall();

        // Any calls to nonReentrant after this point will fail
        _status = 2;

        _;

        // By storing the original value once again, a refund is triggered (see
        // https://eips.ethereum.org/EIPS/eip-2200)
        _status = 1;
    }

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                      CONSTRUCTOR                                                   
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    constructor() initializer {}

    /// @notice Initializes the contract with access control manager
    /// @param _accessControlManager Address of the access control manager contract
    function initialize(IAccessControlManager _accessControlManager) external initializer {
        if (address(_accessControlManager) == address(0)) revert Errors.ZeroAddress();
        accessControlManager = _accessControlManager;
    }

    /// @inheritdoc UUPSHelper
    function _authorizeUpgrade(address) internal view override onlyUpgradeableInstance {}

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                    MAIN FUNCTIONS                                                  
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    /// @notice Claims rewards for a set of users based on Merkle proofs
    /// @param users Addresses claiming rewards (or being claimed for)
    /// @param tokens ERC20 tokens being claimed
    /// @param amounts Cumulative amounts earned (not incremental amounts)
    /// @param proofs Merkle proofs validating each claim
    /// @dev Users can only claim for themselves unless they've authorized an operator
    /// @dev Arrays must all have the same length
    function claim(address[] calldata users, address[] calldata tokens, uint256[] calldata amounts, bytes32[][] calldata proofs) external {
        address[] memory recipients = new address[](users.length);
        bytes[] memory datas = new bytes[](users.length);
        _claim(users, tokens, amounts, proofs, recipients, datas);
    }

    /// @notice Claims rewards with custom recipient addresses and callback data
    /// @param users Addresses claiming rewards (or being claimed for)
    /// @param tokens ERC20 tokens being claimed
    /// @param amounts Cumulative amounts earned (not incremental amounts)
    /// @param proofs Merkle proofs validating each claim
    /// @param recipients Custom recipient addresses for each claim (zero address = use default)
    /// @param datas Arbitrary data passed to recipient's onClaim callback (if recipient is a contract)
    /// @dev Only msg.sender claiming for themselves can override the recipient address
    /// @dev Non-zero recipient addresses override any previously set default recipients
    function claimWithRecipient(
        address[] calldata users,
        address[] calldata tokens,
        uint256[] calldata amounts,
        bytes32[][] calldata proofs,
        address[] calldata recipients,
        bytes[] memory datas
    ) external {
        _claim(users, tokens, amounts, proofs, recipients, datas);
    }

    /// @notice Returns the currently active Merkle root for claim verification
    /// @return The Merkle root that is currently valid for claims
    /// @dev Returns lastTree.merkleRoot if within dispute period or if there's an active dispute
    /// @dev Returns tree.merkleRoot if dispute period has passed and no active dispute
    function getMerkleRoot() public view returns (bytes32) {
        if (block.timestamp >= endOfDisputePeriod && disputer == address(0)) return tree.merkleRoot;
        else return lastTree.merkleRoot;
    }

    /// @notice Returns the epoch duration used for dispute period calculations
    /// @return epochDuration The current epoch duration in seconds
    /// @dev Returns custom _epochDuration if set, otherwise returns default _EPOCH_DURATION (3600 seconds)
    function getEpochDuration() public view returns (uint32 epochDuration) {
        epochDuration = _epochDuration;
        if (epochDuration == 0) epochDuration = _EPOCH_DURATION;
    }

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                 USER ADMIN FUNCTIONS                                               
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    /// @notice Toggles an operator's authorization to claim rewards on behalf of a user
    /// @param user User granting or revoking the authorization
    /// @param operator Operator address being authorized or deauthorized
    /// @dev When operator is address(0), it enables any address to claim for the user
    /// @dev Only the user themselves or governance can toggle operator status
    function toggleOperator(address user, address operator) external {
        if (user != msg.sender && !accessControlManager.isGovernorOrGuardian(msg.sender)) revert Errors.NotTrusted();
        uint256 oldValue = operators[user][operator];
        operators[user][operator] = 1 - oldValue;
        emit OperatorToggled(user, operator, oldValue == 0);
    }

    /// @notice Sets a custom recipient address for a user's token claims
    /// @param recipient Address that will receive claimed tokens (zero address = default to user)
    /// @param token Token for which to set the recipient (zero address = all tokens)
    /// @dev Users can override this recipient when calling claimWithRecipient
    /// @dev Setting recipient to address(0) removes the custom recipient
    function setClaimRecipient(address recipient, address token) external {
        _setClaimRecipient(msg.sender, recipient, token);
    }

    /// @notice Toggles a main operator's authorization to claim tokens on behalf of any user
    /// @param operator Operator whose status is being toggled
    /// @param token Token for which authorization applies (zero address = all tokens)
    /// @dev Only callable by guardian for an individual token or governor if it's for all tokens
    /// @dev Main operators can claim for any user without individual user authorization
    function toggleMainOperatorStatus(address operator, address token) external {
        if (token == address(0)) _onlyGovernor();
        else _onlyGuardian();
        uint256 oldValue = mainOperators[operator][token];
        mainOperators[operator][token] = 1 - oldValue;
        emit MainOperatorStatusUpdated(operator, token, oldValue == 0);
    }

    /// @notice Creates a dispute to freeze the current Merkle tree update
    /// @param reason Explanation for why the tree update is being disputed
    /// @dev Requires depositing disputeAmount of disputeToken as collateral
    /// @dev Can only dispute within disputePeriod after a tree update
    /// @dev Deposit is slashed if dispute is rejected, returned if dispute is valid
    function disputeTree(string memory reason) external {
        if (disputer != address(0)) revert Errors.UnresolvedDispute();
        if (block.timestamp >= endOfDisputePeriod) revert Errors.InvalidDispute();
        IERC20(disputeToken).safeTransferFrom(msg.sender, address(this), disputeAmount);
        disputer = msg.sender;
        emit Disputed(reason);
    }

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                 GOVERNANCE FUNCTIONS                                               
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    /// @notice Updates the active Merkle tree with new reward data
    /// @param _tree New Merkle tree containing updated reward information
    /// @dev Can only be called by trusted addresses or governor
    /// @dev Trusted addresses cannot update during an active dispute period to prevent circumventing disputes
    /// @dev Saves the current tree to lastTree before updating
    function updateTree(MerkleTree calldata _tree) external {
        if (
            disputer != address(0) ||
            // A trusted address cannot update a tree right after a precedent tree update otherwise it can de facto
            // validate a tree which has not passed the dispute period
            ((canUpdateMerkleRoot[msg.sender] != 1 || block.timestamp < endOfDisputePeriod) && !accessControlManager.isGovernor(msg.sender))
        ) revert Errors.NotTrusted();
        MerkleTree memory _lastTree = tree;
        tree = _tree;
        lastTree = _lastTree;

        uint48 _endOfPeriod = _endOfDisputePeriod(uint48(block.timestamp));
        endOfDisputePeriod = _endOfPeriod;
        emit TreeUpdated(_tree.merkleRoot, _tree.ipfsHash, _endOfPeriod);
    }

    /// @notice Toggles an address's authorization to update the Merkle tree
    /// @param trustAddress Address whose trusted status is being toggled
    /// @dev Only callable by governor
    /// @dev Trusted addresses can update trees but must wait for dispute periods
    function toggleTrusted(address trustAddress) external onlyGovernor {
        uint256 trustedStatus = 1 - canUpdateMerkleRoot[trustAddress];
        canUpdateMerkleRoot[trustAddress] = trustedStatus;
        emit TrustedToggled(trustAddress, trustedStatus == 1);
    }

    /// @notice Permanently disables contract upgradeability
    /// @dev Only callable by governor
    /// @dev This action is irreversible - use with extreme caution
    function revokeUpgradeability() external onlyGovernor {
        upgradeabilityDeactivated = 1;
        emit UpgradeabilityRevoked();
    }

    /// @notice Updates the epoch duration used for dispute period calculations
    /// @param epochDuration New epoch duration in seconds
    /// @dev Only callable by governor
    function setEpochDuration(uint32 epochDuration) external onlyGovernor {
        _epochDuration = epochDuration;
        emit EpochDurationUpdated(epochDuration);
    }

    /// @notice Resolves an ongoing dispute
    /// @param valid True if the dispute is valid (tree will be reverted), false if invalid (disputer loses deposit)
    /// @dev Only callable by governor
    /// @dev If valid: returns deposit to disputer and reverts to lastTree
    /// @dev If invalid: sends deposit to governor and extends dispute period
    function resolveDispute(bool valid) external onlyGovernor {
        if (disputer == address(0)) revert Errors.NoDispute();
        if (valid) {
            IERC20(disputeToken).safeTransfer(disputer, disputeAmount);
            // If a dispute is valid, the contract falls back to the last tree that was updated
            _revokeTree();
        } else {
            IERC20(disputeToken).safeTransfer(msg.sender, disputeAmount);
            endOfDisputePeriod = _endOfDisputePeriod(uint48(block.timestamp));
        }
        disputer = address(0);
        emit DisputeResolved(valid);
    }

    /// @notice Reverts to the previous Merkle tree immediately
    /// @dev Only callable by governor
    /// @dev Cannot be called if there's an active dispute (must resolve dispute first)
    function revokeTree() external onlyGovernor {
        if (disputer != address(0)) revert Errors.UnresolvedDispute();
        _revokeTree();
    }

    /// @notice Recovers ERC20 tokens accidentally sent to the contract
    /// @param tokenAddress Address of the token to recover
    /// @param to Address that will receive the recovered tokens
    /// @param amountToRecover Amount of tokens to recover
    /// @dev Only callable by governor
    function recoverERC20(address tokenAddress, address to, uint256 amountToRecover) external onlyGovernor {
        IERC20(tokenAddress).safeTransfer(to, amountToRecover);
        emit Recovered(tokenAddress, to, amountToRecover);
    }

    /// @notice Updates the dispute period duration
    /// @param _disputePeriod New dispute period in epoch units
    /// @dev Only callable by governor
    function setDisputePeriod(uint48 _disputePeriod) external onlyGovernor {
        disputePeriod = uint48(_disputePeriod);
        emit DisputePeriodUpdated(_disputePeriod);
    }

    /// @notice Updates the token required as collateral for disputes
    /// @param _disputeToken New dispute token address
    /// @dev Only callable by governor
    /// @dev Cannot be changed during an active dispute
    function setDisputeToken(IERC20 _disputeToken) external onlyGovernor {
        if (disputer != address(0)) revert Errors.UnresolvedDispute();
        disputeToken = _disputeToken;
        emit DisputeTokenUpdated(address(_disputeToken));
    }

    /// @notice Updates the amount of tokens required to create a dispute
    /// @param _disputeAmount New dispute amount
    /// @dev Only callable by governor
    /// @dev Cannot be changed during an active dispute
    function setDisputeAmount(uint256 _disputeAmount) external onlyGovernor {
        if (disputer != address(0)) revert Errors.UnresolvedDispute();
        disputeAmount = _disputeAmount;
        emit DisputeAmountUpdated(_disputeAmount);
    }

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                   INTERNAL HELPERS                                                 
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    /// @notice Internal implementation of reward claiming with full recipient and callback support
    /// @param users Addresses claiming rewards
    /// @param tokens Tokens being claimed
    /// @param amounts Cumulative earned amounts (not incremental)
    /// @param proofs Merkle proofs for validation
    /// @param recipients Custom recipient addresses (zero = use default)
    /// @param datas Callback data for recipients
    /// @dev Validates authorization, verifies proofs, updates claimed amounts, and transfers tokens
    /// @dev Attempts to call onClaim callback on recipient if data is provided
    function _claim(
        address[] calldata users,
        address[] calldata tokens,
        uint256[] calldata amounts,
        bytes32[][] calldata proofs,
        address[] memory recipients,
        bytes[] memory datas
    ) internal nonReentrant {
        uint256 usersLength = users.length;
        if (
            usersLength == 0 ||
            usersLength != tokens.length ||
            usersLength != amounts.length ||
            usersLength != proofs.length ||
            usersLength != recipients.length ||
            usersLength != datas.length
        ) revert Errors.InvalidLengths();

        for (uint256 i; i < usersLength; ) {
            address user = users[i];
            address token = tokens[i];
            uint256 amount = amounts[i];
            bytes memory data = datas[i];

            // Only approved operators can claim for `user`
            if (
                msg.sender != user &&
                tx.origin != user &&
                mainOperators[msg.sender][token] == 0 &&
                mainOperators[msg.sender][address(0)] == 0 &&
                operators[user][msg.sender] == 0 &&
                operators[user][address(0)] == 0 &&
                !accessControlManager.isGovernorOrGuardian(msg.sender)
            ) revert Errors.NotWhitelisted();

            // Verifying proof
            bytes32 leaf = keccak256(abi.encode(user, token, amount));
            if (!_verifyProof(leaf, proofs[i])) revert Errors.InvalidProof();

            // Closing reentrancy gate here
            uint256 toSend = amount - claimed[user][token].amount;
            claimed[user][token] = Claim(SafeCast.toUint208(amount), uint48(block.timestamp), getMerkleRoot());
            emit Claimed(user, token, toSend);

            address recipient = recipients[i];
            // Only `msg.sender` can set a different recipient for itself within the context of a call to claim
            // The recipient set in the context of the call to `claim` can override the default recipient set by the user
            if (msg.sender != user || recipient == address(0)) {
                address userSetRecipient = claimRecipient[user][token];
                if (userSetRecipient == address(0)) userSetRecipient = claimRecipient[user][address(0)];
                if (userSetRecipient == address(0)) recipient = user;
                else recipient = userSetRecipient;
            }

            if (toSend != 0) {
                IERC20(token).safeTransfer(recipient, toSend);
                if (data.length != 0) {
                    try IClaimRecipient(recipient).onClaim(user, token, amount, data) returns (bytes32 callbackSuccess) {
                        if (callbackSuccess != CALLBACK_SUCCESS) revert Errors.InvalidReturnMessage();
                    } catch {}
                }
            }
            unchecked {
                ++i;
            }
        }
    }

    /// @notice Reverts to the previous Merkle tree
    /// @dev Resets endOfDisputePeriod to 0 and emits both Revoked and TreeUpdated events
    function _revokeTree() internal {
        MerkleTree memory _tree = lastTree;
        endOfDisputePeriod = 0;
        tree = _tree;
        uint32 epochDuration = getEpochDuration();
        emit Revoked();
        emit TreeUpdated(
            _tree.merkleRoot,
            _tree.ipfsHash,
            (uint48(block.timestamp) / epochDuration) * (epochDuration) // Last hour
        );
    }

    /// @notice Calculates when a tree update's dispute period ends
    /// @param treeUpdate Timestamp when the tree was updated
    /// @return Timestamp when the dispute period ends and tree becomes effective
    /// @dev Rounds treeUpdate up to next epoch boundary, then adds disputePeriod epochs
    function _endOfDisputePeriod(uint48 treeUpdate) internal view returns (uint48) {
        uint32 epochDuration = getEpochDuration();
        return ((treeUpdate - 1) / epochDuration + 1 + disputePeriod) * (epochDuration);
    }

    /// @notice Verifies a Merkle proof against the current active root
    /// @param leaf Hashed leaf data representing the claim (user, token, amount)
    /// @param proof Array of sibling hashes forming the path from leaf to root
    /// @return True if the proof is valid, false otherwise
    /// @dev Uses standard Merkle tree verification with sorted concatenation
    function _verifyProof(bytes32 leaf, bytes32[] memory proof) internal view returns (bool) {
        bytes32 currentHash = leaf;
        uint256 proofLength = proof.length;
        for (uint256 i; i < proofLength; ) {
            if (currentHash < proof[i]) {
                currentHash = keccak256(abi.encode(currentHash, proof[i]));
            } else {
                currentHash = keccak256(abi.encode(proof[i], currentHash));
            }
            unchecked {
                ++i;
            }
        }
        bytes32 root = getMerkleRoot();
        if (root == bytes32(0)) revert Errors.InvalidUninitializedRoot();
        return currentHash == root;
    }

    /// @notice Internal implementation for setting a claim recipient
    /// @param user User for whom to set the recipient
    /// @param recipient Address that will receive claimed tokens
    /// @param token Token for which recipient is set (address(0) = all tokens)
    function _setClaimRecipient(address user, address recipient, address token) internal {
        claimRecipient[user][token] = recipient;
        emit ClaimRecipientUpdated(user, recipient, token);
    }

    /// @notice Ensures the caller has governor role
    function _onlyGovernor() internal view {
        if (!accessControlManager.isGovernor(msg.sender)) revert Errors.NotGovernor();
    }

    /// @notice Ensures the caller has guardian role
    function _onlyGuardian() internal view {
        if (!accessControlManager.isGovernorOrGuardian(msg.sender)) revert Errors.NotGovernorOrGuardian();
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.17;

import { AccessControlEnumerableUpgradeable } from "@openzeppelin/contracts-upgradeable/access/AccessControlEnumerableUpgradeable.sol";
import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import { IAccessControlManager } from "./interfaces/IAccessControlManager.sol";

/// @title AccessControlManager
/// @author Merkl SAS
/// @notice Manages role-based access control across all Merkl protocol contracts
/// @dev Implements a two-tier permission system with governor and guardian roles
/// @dev All governors automatically have guardian privileges
contract AccessControlManager is IAccessControlManager, Initializable, AccessControlEnumerableUpgradeable {
    /// @notice Role identifier for guardians (limited administrative privileges)
    bytes32 public constant GUARDIAN_ROLE = keccak256("GUARDIAN_ROLE");
    /// @notice Role identifier for governors (full administrative privileges)
    bytes32 public constant GOVERNOR_ROLE = keccak256("GOVERNOR_ROLE");

    // =============================== Events ======================================

    event AccessControlManagerUpdated(address indexed _accessControlManager);

    // =============================== Errors ======================================

    error InvalidAccessControlManager();
    error IncompatibleGovernorAndGuardian();
    error NotEnoughGovernorsLeft();
    error ZeroAddress();

    /// @notice Initializes the AccessControlManager with initial governor and guardian
    /// @param governor Address to be granted the governor role (full administrative privileges)
    /// @param guardian Address to be granted the guardian role (limited administrative privileges)
    /// @dev Governor and guardian must be different non-zero addresses
    /// @dev Governor automatically receives both GOVERNOR_ROLE and GUARDIAN_ROLE
    /// @dev Sets GOVERNOR_ROLE as the admin role for both GOVERNOR_ROLE and GUARDIAN_ROLE
    function initialize(address governor, address guardian) public initializer {
        if (governor == address(0) || guardian == address(0)) revert ZeroAddress();
        if (governor == guardian) revert IncompatibleGovernorAndGuardian();
        _setupRole(GOVERNOR_ROLE, governor);
        _setupRole(GUARDIAN_ROLE, guardian);
        _setupRole(GUARDIAN_ROLE, governor);
        _setRoleAdmin(GUARDIAN_ROLE, GOVERNOR_ROLE);
        _setRoleAdmin(GOVERNOR_ROLE, GOVERNOR_ROLE);
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() initializer {}

    // =========================== View Functions ==================================

    /// @inheritdoc IAccessControlManager
    function isGovernor(address admin) external view virtual returns (bool) {
        return hasRole(GOVERNOR_ROLE, admin);
    }

    /// @inheritdoc IAccessControlManager
    function isGovernorOrGuardian(address admin) external view returns (bool) {
        return hasRole(GUARDIAN_ROLE, admin);
    }

    // =========================== Governor Functions ==============================

    /// @notice Grants governor role to a new address
    /// @param governor Address to receive governor privileges
    /// @dev Must be called instead of grantRole to ensure the address receives both governor and guardian roles
    /// @dev Only existing governors can call this function
    function addGovernor(address governor) external {
        grantRole(GOVERNOR_ROLE, governor);
        grantRole(GUARDIAN_ROLE, governor);
    }

    /// @notice Revokes governor role from an address
    /// @param governor Address to lose governor privileges
    /// @dev Must be called instead of revokeRole to ensure both governor and guardian roles are removed
    /// @dev Cannot remove the last governor - at least one must remain
    /// @dev Only existing governors can call this function
    function removeGovernor(address governor) external {
        if (getRoleMemberCount(GOVERNOR_ROLE) <= 1) revert NotEnoughGovernorsLeft();
        revokeRole(GUARDIAN_ROLE, governor);
        revokeRole(GOVERNOR_ROLE, governor);
    }

    /// @notice Migrates to a new AccessControlManager contract
    /// @param _accessControlManager Address of the new AccessControlManager contract
    /// @dev Validates that all current governors are also governors in the new contract
    /// @dev After calling this, governance should also update all protocol contracts to use the new AccessControlManager
    /// @dev Only callable by existing governors
    function setAccessControlManager(IAccessControlManager _accessControlManager) external onlyRole(GOVERNOR_ROLE) {
        uint256 count = getRoleMemberCount(GOVERNOR_ROLE);
        bool success;
        for (uint256 i; i < count; ++i) {
            success = _accessControlManager.isGovernor(getRoleMember(GOVERNOR_ROLE, i));
            if (!success) break;
        }
        if (!success) revert InvalidAccessControlManager();
        emit AccessControlManagerUpdated(address(_accessControlManager));
    }
}

// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.17;

/// @title IClaimRecipient
/// @author Merkl SAS
/// @notice Interface for the `ClaimRecipient` contracts expected by the `Distributor` contract
interface IClaimRecipient {
    /// @notice Hook to call within contracts receiving token rewards on behalf of users
    function onClaim(address user, address token, uint256 amount, bytes memory data) external returns (bytes32);
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v4.9.0) (proxy/utils/Initializable.sol)

pragma solidity ^0.8.2;

import "../../utils/AddressUpgradeable.sol";

/**
 * @dev This is a base contract to aid in writing upgradeable contracts, or any kind of contract that will be deployed
 * behind a proxy. Since proxied contracts do not make use of a constructor, it's common to move constructor logic to an
 * external initializer function, usually called `initialize`. It then becomes necessary to protect this initializer
 * function so it can only be called once. The {initializer} modifier provided by this contract will have this effect.
 *
 * The initialization functions use a version number. Once a version number is used, it is consumed and cannot be
 * reused. This mechanism prevents re-execution of each "step" but allows the creation of new initialization steps in
 * case an upgrade adds a module that needs to be initialized.
 *
 * For example:
 *
 * [.hljs-theme-light.nopadding]
 * ```solidity
 * contract MyToken is ERC20Upgradeable {
 *     function initialize() initializer public {
 *         __ERC20_init("MyToken", "MTK");
 *     }
 * }
 *
 * contract MyTokenV2 is MyToken, ERC20PermitUpgradeable {
 *     function initializeV2() reinitializer(2) public {
 *         __ERC20Permit_init("MyToken");
 *     }
 * }
 * ```
 *
 * TIP: To avoid leaving the proxy in an uninitialized state, the initializer function should be called as early as
 * possible by providing the encoded function call as the `_data` argument to {ERC1967Proxy-constructor}.
 *
 * CAUTION: When used with inheritance, manual care must be taken to not invoke a parent initializer twice, or to ensure
 * that all initializers are idempotent. This is not verified automatically as constructors are by Solidity.
 *
 * [CAUTION]
 * ====
 * Avoid leaving a contract uninitialized.
 *
 * An uninitialized contract can be taken over by an attacker. This applies to both a proxy and its implementation
 * contract, which may impact the proxy. To prevent the implementation contract from being used, you should invoke
 * the {_disableInitializers} function in the constructor to automatically lock it when it is deployed:
 *
 * [.hljs-theme-light.nopadding]
 * ```
 * /// @custom:oz-upgrades-unsafe-allow constructor
 * constructor() {
 *     _disableInitializers();
 * }
 * ```
 * ====
 */
abstract contract Initializable {
    /**
     * @dev Indicates that the contract has been initialized.
     * @custom:oz-retyped-from bool
     */
    uint8 private _initialized;

    /**
     * @dev Indicates that the contract is in the process of being initialized.
     */
    bool private _initializing;

    /**
     * @dev Triggered when the contract has been initialized or reinitialized.
     */
    event Initialized(uint8 version);

    /**
     * @dev A modifier that defines a protected initializer function that can be invoked at most once. In its scope,
     * `onlyInitializing` functions can be used to initialize parent contracts.
     *
     * Similar to `reinitializer(1)`, except that functions marked with `initializer` can be nested in the context of a
     * constructor.
     *
     * Emits an {Initialized} event.
     */
    modifier initializer() {
        bool isTopLevelCall = !_initializing;
        require(
            (isTopLevelCall && _initialized < 1) || (!AddressUpgradeable.isContract(address(this)) && _initialized == 1),
            "Initializable: contract is already initialized"
        );
        _initialized = 1;
        if (isTopLevelCall) {
            _initializing = true;
        }
        _;
        if (isTopLevelCall) {
            _initializing = false;
            emit Initialized(1);
        }
    }

    /**
     * @dev A modifier that defines a protected reinitializer function that can be invoked at most once, and only if the
     * contract hasn't been initialized to a greater version before. In its scope, `onlyInitializing` functions can be
     * used to initialize parent contracts.
     *
     * A reinitializer may be used after the original initialization step. This is essential to configure modules that
     * are added through upgrades and that require initialization.
     *
     * When `version` is 1, this modifier is similar to `initializer`, except that functions marked with `reinitializer`
     * cannot be nested. If one is invoked in the context of another, execution will revert.
     *
     * Note that versions can jump in increments greater than 1; this implies that if multiple reinitializers coexist in
     * a contract, executing them in the right order is up to the developer or operator.
     *
     * WARNING: setting the version to 255 will prevent any future reinitialization.
     *
     * Emits an {Initialized} event.
     */
    modifier reinitializer(uint8 version) {
        require(!_initializing && _initialized < version, "Initializable: contract is already initialized");
        _initialized = version;
        _initializing = true;
        _;
        _initializing = false;
        emit Initialized(version);
    }

    /**
     * @dev Modifier to protect an initialization function so that it can only be invoked by functions with the
     * {initializer} and {reinitializer} modifiers, directly or indirectly.
     */
    modifier onlyInitializing() {
        require(_initializing, "Initializable: contract is not initializing");
        _;
    }

    /**
     * @dev Locks the contract, preventing any future reinitialization. This cannot be part of an initializer call.
     * Calling this in the constructor of a contract will prevent that contract from being initialized or reinitialized
     * to any version. It is recommended to use this to lock implementation contracts that are designed to be called
     * through proxies.
     *
     * Emits an {Initialized} event the first time it is successfully executed.
     */
    function _disableInitializers() internal virtual {
        require(!_initializing, "Initializable: contract is initializing");
        if (_initialized != type(uint8).max) {
            _initialized = type(uint8).max;
            emit Initialized(type(uint8).max);
        }
    }

    /**
     * @dev Returns the highest version that has been initialized. See {reinitializer}.
     */
    function _getInitializedVersion() internal view returns (uint8) {
        return _initialized;
    }

    /**
     * @dev Returns `true` if the contract is currently initializing. See {onlyInitializing}.
     */
    function _isInitializing() internal view returns (bool) {
        return _initializing;
    }
}

// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.17;

import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import { IAccessControlManager } from "../interfaces/IAccessControlManager.sol";
import { Errors } from "./Errors.sol";

/// @title UUPSHelper
/// @notice Helper contract for UUPSUpgradeable contracts where the upgradeability is controlled by a specific address
/// @author Merkl SAS
/// @dev The 0 address check in the modifier allows the use of these modifiers during initialization
abstract contract UUPSHelper is UUPSUpgradeable {
    modifier onlyGuardianUpgrader(IAccessControlManager _accessControlManager) {
        if (address(_accessControlManager) != address(0) && !_accessControlManager.isGovernorOrGuardian(msg.sender))
            revert Errors.NotGovernorOrGuardian();
        _;
    }

    modifier onlyGovernorUpgrader(IAccessControlManager _accessControlManager) {
        if (address(_accessControlManager) != address(0) && !_accessControlManager.isGovernor(msg.sender)) revert Errors.NotGovernor();
        _;
    }

    constructor() initializer {}

    /// @inheritdoc UUPSUpgradeable
    function _authorizeUpgrade(address newImplementation) internal virtual override {}
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v4.5.0) (access/AccessControlEnumerable.sol)

pragma solidity ^0.8.0;

import "./IAccessControlEnumerableUpgradeable.sol";
import "./AccessControlUpgradeable.sol";
import "../utils/structs/EnumerableSetUpgradeable.sol";
import "../proxy/utils/Initializable.sol";

/**
 * @dev Extension of {AccessControl} that allows enumerating the members of each role.
 */
abstract contract AccessControlEnumerableUpgradeable is Initializable, IAccessControlEnumerableUpgradeable, AccessControlUpgradeable {
    function __AccessControlEnumerable_init() internal onlyInitializing {
    }

    function __AccessControlEnumerable_init_unchained() internal onlyInitializing {
    }
    using EnumerableSetUpgradeable for EnumerableSetUpgradeable.AddressSet;

    mapping(bytes32 => EnumerableSetUpgradeable.AddressSet) private _roleMembers;

    /**
     * @dev See {IERC165-supportsInterface}.
     */
    function supportsInterface(bytes4 interfaceId) public view virtual override returns (bool) {
        return interfaceId == type(IAccessControlEnumerableUpgradeable).interfaceId || super.supportsInterface(interfaceId);
    }

    /**
     * @dev Returns one of the accounts that have `role`. `index` must be a
     * value between 0 and {getRoleMemberCount}, non-inclusive.
     *
     * Role bearers are not sorted in any particular way, and their ordering may
     * change at any point.
     *
     * WARNING: When using {getRoleMember} and {getRoleMemberCount}, make sure
     * you perform all queries on the same block. See the following
     * https://forum.openzeppelin.com/t/iterating-over-elements-on-enumerableset-in-openzeppelin-contracts/2296[forum post]
     * for more information.
     */
    function getRoleMember(bytes32 role, uint256 index) public view virtual override returns (address) {
        return _roleMembers[role].at(index);
    }

    /**
     * @dev Returns the number of accounts that have `role`. Can be used
     * together with {getRoleMember} to enumerate all bearers of a role.
     */
    function getRoleMemberCount(bytes32 role) public view virtual override returns (uint256) {
        return _roleMembers[role].length();
    }

    /**
     * @dev Overload {_grantRole} to track enumerable memberships
     */
    function _grantRole(bytes32 role, address account) internal virtual override {
        super._grantRole(role, account);
        _roleMembers[role].add(account);
    }

    /**
     * @dev Overload {_revokeRole} to track enumerable memberships
     */
    function _revokeRole(bytes32 role, address account) internal virtual override {
        super._revokeRole(role, account);
        _roleMembers[role].remove(account);
    }

    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[49] private __gap;
}

// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.17;

library Errors {
    error CampaignDoesNotExist();
    error CampaignAlreadyExists();
    error CampaignDurationBelowHour();
    error CampaignRewardTokenNotWhitelisted();
    error CampaignRewardTooLow();
    error CampaignShouldStartInFuture();
    error InvalidDispute();
    error InvalidLengths();
    error InvalidOverride();
    error InvalidParam();
    error InvalidParams();
    error InvalidProof();
    error InvalidUninitializedRoot();
    error InvalidReallocation();
    error InvalidReturnMessage();
    error InvalidReward();
    error InvalidSignature();
    error KeyAlreadyUsed();
    error NoDispute();
    error NoOverrideForCampaign();
    error NotAllowed();
    error NotEnoughAllowance();
    error NotEnoughBalance();
    error NotEnoughPayment();
    error NotGovernor();
    error NotGovernorOrGuardian();
    error NotSigned();
    error NotTrusted();
    error NotUpgradeable();
    error NotWhitelisted();
    error OperatorNotAllowed();
    error UnresolvedDispute();
    error ZeroAddress();
    error DisputeFundsTransferFailed();
    error EthNotAccepted();
    error ReentrantCall();
    error WithdrawalFailed();
    error InvalidClaim();
    error RefererNotSet();
}

// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.17;

/// @title IAccessControlManager
/// @author Merkl SAS
/// @notice Interface for the `AccessControlManager` contracts of Merkl contracts
interface IAccessControlManager {
    /// @notice Checks whether an address is governor
    /// @param admin Address to check
    /// @return Whether the address has the `GOVERNOR_ROLE` or not
    function isGovernor(address admin) external view returns (bool);

    /// @notice Checks whether an address is a governor or a guardian of a module
    /// @param admin Address to check
    /// @return Whether the address has the `GUARDIAN_ROLE` or not
    /// @dev Governance should make sure when adding a governor to also give this governor the guardian
    /// role by calling the `addGovernor` function
    function isGovernorOrGuardian(address admin) external view returns (bool);
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.7;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { IAccessControlManager } from "../../interfaces/IAccessControlManager.sol";
import { Errors } from "../../utils/Errors.sol";

/// @title PointToken
/// @author Merkl SAS
/// @notice Reference contract for points systems within Merkl
contract PointToken is ERC20 {
    mapping(address => bool) public minters;
    mapping(address => bool) public whitelistedRecipients;
    IAccessControlManager public accessControlManager;
    uint8 public allowedTransfers;

    constructor(string memory name_, string memory symbol_, address _minter, address _accessControlManager) ERC20(name_, symbol_) {
        if (_accessControlManager == address(0) || _minter == address(0)) revert Errors.ZeroAddress();
        accessControlManager = IAccessControlManager(_accessControlManager);
        minters[_minter] = true;
    }

    modifier onlyGovernorOrGuardian() {
        if (!accessControlManager.isGovernorOrGuardian(msg.sender)) revert Errors.NotGovernorOrGuardian();
        _;
    }

    modifier onlyGovernor() {
        if (!accessControlManager.isGovernor(msg.sender)) revert Errors.NotGovernor();
        _;
    }

    modifier onlyMinter() {
        if (!minters[msg.sender]) revert Errors.NotTrusted();
        _;
    }

    function mint(address account, uint256 amount) external onlyMinter {
        _mint(account, amount);
    }

    function burn(address account, uint256 amount) external onlyMinter {
        _burn(account, amount);
    }

    function mintBatch(address[] memory accounts, uint256[] memory amounts) external onlyMinter {
        uint256 length = accounts.length;
        for (uint256 i = 0; i < length; ++i) {
            _mint(accounts[i], amounts[i]);
        }
    }

    function toggleMinter(address minter) external onlyGovernor {
        minters[minter] = !minters[minter];
    }

    function toggleAllowedTransfers() external onlyGovernorOrGuardian {
        allowedTransfers = 1 - allowedTransfers;
    }

    function toggleWhitelistedRecipient(address recipient) external onlyGovernorOrGuardian {
        whitelistedRecipients[recipient] = !whitelistedRecipients[recipient];
    }

    function _beforeTokenTransfer(address from, address to, uint256) internal view override {
        if (allowedTransfers == 0 && from != address(0) && to != address(0) && !whitelistedRecipients[from] && !whitelistedRecipients[to])
            revert Errors.NotAllowed();
    }
}

// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.17;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";

import { IAccessControlManager } from "../../interfaces/IAccessControlManager.sol";
import { Errors } from "../../utils/Errors.sol";

/// @title SonicFragment
/// @notice Contract for Sonic fragments which can be converted upon activation into S tokens
/// @author Merkl SAS
contract SonicFragment is ERC20 {
    using SafeERC20 for IERC20;

    /// @notice Contract handling access control
    IAccessControlManager public immutable accessControlManager;
    /// @notice Address for the S token
    address public immutable sToken;

    /// @notice Amount of S tokens sent on the contract at the activation of redemption
    /// @dev Used to compute the exchange rate between fragments and S tokens
    uint128 public sTokenAmount;
    /// @notice Total supply of the contract
    /// @dev Needs to be stored to compute the exchange rate between fragments and sTokens
    uint120 public supply;
    /// @notice Whether redemption for S tokens has been activated or not
    uint8 public contractSettled;

    constructor(
        address _accessControlManager,
        address recipient,
        address _sToken,
        uint256 _totalSupply,
        string memory _name,
        string memory _symbol
    ) ERC20(_name, _symbol) {
        // Zero address check
        if (_sToken == address(0)) revert Errors.ZeroAddress();
        IAccessControlManager(_accessControlManager).isGovernor(msg.sender);
        sToken = _sToken;
        accessControlManager = IAccessControlManager(_accessControlManager);
        supply = uint120(_totalSupply);
        _mint(recipient, _totalSupply);
    }

    // ================================= MODIFIERS =================================

    /// @notice Checks whether the `msg.sender` has the governor role
    modifier onlyGovernor() {
        if (!accessControlManager.isGovernor(msg.sender)) revert Errors.NotAllowed();
        _;
    }

    /// @notice Activates the contract settlement and enables redemption of fragments into S
    /// @dev Can only be called once
    function settleContract(uint256 _sTokenAmount) external onlyGovernor {
        if (contractSettled > 0) revert Errors.NotAllowed();
        contractSettled = 1;
        IERC20(sToken).safeTransferFrom(msg.sender, address(this), sTokenAmount);
        sTokenAmount = uint128(_sTokenAmount);
    }

    /// @notice Recovers leftover tokens after sometime
    function recover(uint256 amount, address recipient) external onlyGovernor {
        IERC20(sToken).safeTransfer(recipient, amount);
        sTokenAmount = 0;
    }

    /// @notice Redeems fragments against S based on a predefined exchange rate
    function redeem(uint256 amount, address recipient) external returns (uint256 amountToSend) {
        uint128 _sTokenAmount = sTokenAmount;
        if (_sTokenAmount == 0) revert Errors.NotAllowed();
        _burn(msg.sender, amount);
        amountToSend = (amount * _sTokenAmount) / supply;
        IERC20(sToken).safeTransfer(recipient, amountToSend);
    }
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.17;

import { console } from "forge-std/console.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { ITransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { ProxyAdmin } from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";

import { ChainUtils, IMultiChainScript } from "./utils/ChainUtils.s.sol";
import { BaseScript } from "./utils/Base.s.sol";
import { Distributor, MerkleTree } from "../contracts/Distributor.sol";
import { IAccessControlManager } from "../contracts/interfaces/IAccessControlManager.sol";
import { AccessControlManager } from "../contracts/AccessControlManager.sol";

// Base contract with shared utilities
contract DistributorScript is BaseScript {}

// Deploy script
contract Deploy is DistributorScript {
    function run() external broadcast {
        uint256 chainId = block.chainid;
        console.log("DEPLOYER_ADDRESS:", broadcaster);
        // TODO
        address accessControlManager = address(0);

        // Deploy implementation
        Distributor implementation = new Distributor();
        console.log("Distributor Implementation:", address(implementation));

        // Deploy proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), "");
        console.log("Distributor Proxy:", address(proxy));

        // Initialize
        Distributor(address(proxy)).initialize(IAccessControlManager(accessControlManager));
    }
}

// UpdateTree script
contract UpdateTree is DistributorScript {
    function run() external broadcast {
        // MODIFY THESE VALUES TO SET YOUR DESIRED TREE PARAMETERS
        bytes32 merkleRoot = bytes32(0);
        bytes32 ipfsHash = bytes32(0);
        _run(merkleRoot, ipfsHash);
    }

    function run(bytes32 merkleRoot, bytes32 ipfsHash) external broadcast {
        _run(merkleRoot, ipfsHash);
    }

    function _run(bytes32 merkleRoot, bytes32 ipfsHash) internal {
        uint256 chainId = block.chainid;
        // TODO
        address distributorAddress = address(0);

        MerkleTree memory newTree = MerkleTree({ merkleRoot: merkleRoot, ipfsHash: ipfsHash });

        Distributor(distributorAddress).updateTree(newTree);

        console.log("Tree updated with root:", vm.toString(merkleRoot));
        console.log("IPFS Hash:", vm.toString(ipfsHash));
    }
}

// DisputeTree script
contract DisputeTree is DistributorScript {
    function run() external broadcast {
        // MODIFY THIS VALUE TO SET YOUR DESIRED REASON
        string memory reason = "reason";
        _run(reason);
    }

    function run(string calldata reason) external broadcast {
        _run(reason);
    }

    function _run(string memory reason) internal {
        uint256 chainId = block.chainid;
        address distributorAddress = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;

        // Get dispute token and amount from the distributor
        IERC20 disputeToken = Distributor(distributorAddress).disputeToken();
        uint256 disputeAmount = Distributor(distributorAddress).disputeAmount();

        // Check current allowance
        uint256 currentAllowance = disputeToken.allowance(broadcaster, distributorAddress);
        if (currentAllowance < disputeAmount) {
            disputeToken.approve(distributorAddress, disputeAmount);
        }

        Distributor(distributorAddress).disputeTree(reason);

        console.log("Tree disputed with reason:", reason);
    }
}

// ResolveDispute script
contract ResolveDispute is DistributorScript {
    function run() external broadcast {
        // MODIFY THIS VALUE TO SET YOUR DESIRED VALIDITY
        bool valid = false;
        _run(valid);
    }

    function run(bool valid) external broadcast {
        _run(valid);
    }

    function _run(bool valid) internal {
        uint256 chainId = block.chainid;
        address distributorAddress = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;

        Distributor(distributorAddress).resolveDispute(valid);

        console.log("Dispute resolved with validity:", valid);
    }
}

// RevokeTree script
contract RevokeTree is DistributorScript {
    function run() external broadcast {
        uint256 chainId = block.chainid;
        address distributorAddress = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;

        Distributor(distributorAddress).revokeTree();

        console.log("Tree revoked");
    }
}

// ToggleOperator script
contract ToggleOperator is DistributorScript {
    function run() external broadcast {
        // MODIFY THESE VALUES TO SET YOUR DESIRED USER AND OPERATOR
        // forge script scripts/Distributor.s.sol:ToggleOperator --rpc-url berachain --sender 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701 --broadcast -i 1
        address user = address(0xe2843F0148Ab7de33Ce85DE433850F5f68b46331);
        address operator = address(0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701);
        _run(user, operator);
    }

    function run(address user, address operator) external broadcast {
        _run(user, operator);
    }

    function _run(address user, address operator) internal {
        uint256 chainId = block.chainid;
        address distributorAddress = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;

        Distributor(distributorAddress).toggleOperator(user, operator);

        console.log("Toggled operator:", operator, "for user:", user);
    }
}

// RecoverERC20 script
contract RecoverERC20 is DistributorScript {
    function run() external broadcast {
        // MODIFY THESE VALUES TO SET YOUR DESIRED TOKEN, RECIPIENT AND AMOUNT
        address token = address(0);
        address to = address(0);
        uint256 amount = 0;
        _run(token, to, amount);
    }

    function run(address token, address to, uint256 amount) external broadcast {
        _run(token, to, amount);
    }

    function _run(address token, address to, uint256 amount) internal {
        uint256 chainId = block.chainid;
        address distributorAddress = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;

        Distributor(distributorAddress).recoverERC20(token, to, amount);

        console.log("Recovered %s of token %s to %s", amount, token, to);
    }
}

// SetDisputeToken script
contract SetDisputeToken is DistributorScript {
    function run() external broadcast {
        // MODIFY THIS VALUE TO SET YOUR DESIRED TOKEN
        IERC20 token = IERC20(address(0));
        _run(token);
    }

    function run(IERC20 token) external broadcast {
        _run(token);
    }

    function _run(IERC20 token) internal {
        uint256 chainId = block.chainid;
        address distributorAddress = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;

        Distributor(distributorAddress).setDisputeToken(token);

        console.log("Dispute token updated to:", address(token));
    }
}

// SetDisputeAmount script
contract SetDisputeAmount is DistributorScript {
    function run() external broadcast {
        // MODIFY THIS VALUE TO SET YOUR DESIRED AMOUNT
        uint256 amount = 0;
        _run(amount);
    }

    function run(uint256 amount) external broadcast {
        _run(amount);
    }

    function _run(uint256 amount) internal {
        uint256 chainId = block.chainid;
        address distributorAddress = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;

        Distributor(distributorAddress).setDisputeAmount(amount);

        console.log("Dispute amount updated to:", amount);
    }
}

// SetDisputePeriod script
contract SetDisputePeriod is DistributorScript {
    function run() external broadcast {
        // MODIFY THIS VALUE TO SET YOUR DESIRED PERIOD
        uint48 period = 0;
        _run(period);
    }

    function run(uint48 period) external broadcast {
        _run(period);
    }

    function _run(uint48 period) internal {
        uint256 chainId = block.chainid;
        address distributorAddress = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;

        Distributor(distributorAddress).setDisputePeriod(period);

        console.log("Dispute period updated to:", period);
    }
}

// ToggleTrusted script
contract ToggleTrusted is DistributorScript {
    function run() external broadcast {
        // MODIFY THIS VALUE TO SET YOUR DESIRED EOA
        address eoa = address(0);
        _run(eoa);
    }

    function run(address eoa) external broadcast {
        _run(eoa);
    }

    function _run(address eoa) internal {
        uint256 chainId = block.chainid;
        address distributorAddress = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;

        Distributor(distributorAddress).toggleTrusted(eoa);

        console.log("Toggled trusted status for:", eoa);
    }
}

// Claim script
contract Claim is DistributorScript {
    function run() external broadcast {
        // MODIFY THESE VALUES TO SET YOUR DESIRED CLAIM PARAMETERS
        address[] memory users = new address[](0);
        address[] memory tokens = new address[](0);
        uint256[] memory amounts = new uint256[](0);
        bytes32[][] memory proofs = new bytes32[][](0);
        _run(users, tokens, amounts, proofs);
    }

    function run(
        address[] calldata users,
        address[] calldata tokens,
        uint256[] calldata amounts,
        bytes32[][] calldata proofs
    ) external broadcast {
        _run(users, tokens, amounts, proofs);
    }

    function _run(address[] memory users, address[] memory tokens, uint256[] memory amounts, bytes32[][] memory proofs) internal {
        uint256 chainId = block.chainid;
        address distributorAddress = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;

        Distributor(distributorAddress).claim(users, tokens, amounts, proofs);

        console.log("Claimed rewards for", users.length, "users");
    }
}

contract BuildUpgradeToPayload is DistributorScript {
    function run() external broadcast {
        uint256 chainId = block.chainid;
        address distributor = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;

        address distributorImpl = address(new Distributor());

        bytes memory payload = abi.encodeWithSelector(ITransparentUpgradeableProxy.upgradeTo.selector, distributorImpl);

        address safe = address(0);
    }
}

contract DisputeCheck is DistributorScript, ChainUtils {
    function run() external broadcast {
        uint256 chainId = block.chainid;
        address _distributor = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;
        // TODO: replace
        address _core = address(0);
        address _multisig = address(0);
        address _proxyAdmin = address(0);

        // Now we can call verifyRegistryAddresses directly!
        verifyRegistryAddresses(_distributor, _core, _multisig, _proxyAdmin);

        uint256 disputeAmount = Distributor(_distributor).disputeAmount();
        address disputeToken = address(Distributor(_distributor).disputeToken());
        console.log("Dispute amount:", disputeAmount);
        console.log("Dispute token:", disputeToken);

        // deployCreateX();
        if (disputeAmount == 0 && disputeToken == address(0)) {
            disputeAmount = 100 * 10 ** 6;
            disputeToken = address(0x79A02482A880bCE3F13e09Da970dC34db4CD24d1); // worldchain 480

            bytes memory setDisputeTokenPayload = abi.encodeWithSelector(Distributor.setDisputeToken.selector, disputeToken);

            // console.log("Safe address: %s", safe);
            address safe = address(0);
            _serializeJson(
                chainId,
                _distributor, // target address (the DistributionCreator proxy)
                0, // value
                setDisputeTokenPayload, // setFees call
                Operation.Call, // standard call (not delegate)
                hex"", // signature
                safe // safe address
            );
            // Distributor(distributor).setDisputeAmount(disputeAmount);
            // Distributor(distributor).setDisputeToken(IERC20(disputeToken));
            // Distributor(distributor).setDisputePeriod(1);
        }
    }
}

// New MultiChain version of the existing DisputeCheck
contract DisputeCheckMultiChain is DistributorScript, ChainUtils {
    function run() external {
        // Run dispute check across all chains without using address(this)
        bytes memory emptyData = "";
        executeAcrossChains(address(0), emptyData, false);
    }

    // Override the executeScript function to run our dispute check logic
    function executeScript(
        address /* scriptContract */,
        bytes memory /* data */
    ) internal override returns (bool success, string memory errorReason) {
        new DisputeCheck().run();
        return (true, "");
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.17;

import { console } from "forge-std/console.sol";
import { UpgradeDeploymentBase } from "./utils/UpgradeDeploymentBase.s.sol";
import { DistributionCreator } from "../contracts/DistributionCreator.sol";
import { Distributor } from "../contracts/Distributor.sol";

/// @title DeployUpgradeImplementations
/// @notice Deploys new implementations of DistributionCreator and Distributor for upgrades
/// @dev This script deploys new implementation contracts across all chains and saves the addresses
///      to separate JSON files per chain for easy Gnosis Safe transaction drafting
contract DeployUpgradeImplementations is UpgradeDeploymentBase {
    // All supported chains from foundry.toml
    ChainConfig[] public chains;

    function setUp() public {
        // Initialize chain configurations from base
        ChainConfig[] memory configs = _getChainConfigs();
        for (uint256 i = 0; i < configs.length; i++) {
            chains.push(configs[i]);
        }
    }

    /// @notice Main deployment function
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);

        console.log("==========================================================");
        console.log("Deploying Upgrade Implementations");
        console.log("Deployer:", deployer);
        console.log("==========================================================");
        console.log("");

        // Deploy on all chains
        for (uint256 i = 0; i < chains.length; i++) {
            _deployOnChain(chains[i], deployerPrivateKey, deployer);
        }

        console.log("");
        console.log("==========================================================");
        console.log("Deployment Complete!");
        console.log("Check ./deployments/ folder for individual chain results");
        console.log("==========================================================");
    }

    /// @notice Public wrapper for _deployImplementations to enable try/catch
    function deployImplementationsWrapper() external returns (address, address) {
        return _deployImplementations();
    }

    /// @notice Deploy on a specific chain with error handling
    function _deployOnChain(ChainConfig memory chainConfig, uint256 privateKey, address deployer) internal {
        console.log("----------------------------------------------------------");
        console.log(string.concat("Chain: ", chainConfig.name));
        console.log("Chain ID:", chainConfig.chainId);

        // Fork the chain
        string memory rpcEnvVar = string.concat(_toUpperCase(chainConfig.name), "_NODE_URI");
        string memory rpcUrl;

        try vm.envString(rpcEnvVar) returns (string memory url) {
            rpcUrl = url;
        } catch {
            console.log("SKIPPED: RPC URL not configured");
            _saveDeploymentResult(
                DeploymentResult({
                    distributionCreatorImpl: address(0),
                    distributorImpl: address(0),
                    timestamp: block.timestamp,
                    chainId: chainConfig.chainId,
                    chainName: chainConfig.name,
                    deployer: deployer,
                    status: "SKIPPED",
                    error: "RPC URL not configured"
                })
            );
            console.log("");
            return;
        }

        // Create fork
        uint256 forkId;
        try vm.createFork(rpcUrl) returns (uint256 id) {
            forkId = id;
            vm.selectFork(forkId);
        } catch {
            console.log("ERROR: Failed to create fork");
            _saveDeploymentResult(
                DeploymentResult({
                    distributionCreatorImpl: address(0),
                    distributorImpl: address(0),
                    timestamp: block.timestamp,
                    chainId: chainConfig.chainId,
                    chainName: chainConfig.name,
                    deployer: deployer,
                    status: "ERROR",
                    error: "Failed to create fork - RPC may be down"
                })
            );
            console.log("");
            return;
        }

        // Verify chain ID matches
        if (block.chainid != chainConfig.chainId) {
            console.log("ERROR: Chain ID mismatch");
            console.log("Expected:", chainConfig.chainId);
            console.log("Got:", block.chainid);
            _saveDeploymentResult(
                DeploymentResult({
                    distributionCreatorImpl: address(0),
                    distributorImpl: address(0),
                    timestamp: block.timestamp,
                    chainId: chainConfig.chainId,
                    chainName: chainConfig.name,
                    deployer: deployer,
                    status: "ERROR",
                    error: "Chain ID mismatch"
                })
            );
            console.log("");
            return;
        }

        // Start broadcasting transactions
        vm.startBroadcast(privateKey);

        address distributionCreatorImpl;
        address distributorImpl;
        string memory errorMsg = "";

        // Deploy implementations using base contract function
        try this.deployImplementationsWrapper() returns (address dcImpl, address dImpl) {
            distributionCreatorImpl = dcImpl;
            distributorImpl = dImpl;
        } catch Error(string memory reason) {
            errorMsg = string.concat("Failed to deploy: ", reason);
            console.log("ERROR:", errorMsg);
        } catch (bytes memory lowLevelData) {
            errorMsg = "Failed to deploy: Low-level error";
            console.log("ERROR:", errorMsg);
            console.logBytes(lowLevelData);
        }

        vm.stopBroadcast();

        // Determine status
        string memory status;
        if (bytes(errorMsg).length > 0) {
            status = "ERROR";
        } else if (distributionCreatorImpl != address(0) && distributorImpl != address(0)) {
            status = "SUCCESS";
            console.log("SUCCESS: Both implementations deployed");

            // Verify contracts if not skipped
            if (!chainConfig.skipVerification) {
                _verifyContracts(chainConfig.name, distributionCreatorImpl, distributorImpl);
            }
        } else {
            status = "PARTIAL";
            errorMsg = "Some deployments failed";
        }

        // Save deployment result
        _saveDeploymentResult(
            DeploymentResult({
                distributionCreatorImpl: distributionCreatorImpl,
                distributorImpl: distributorImpl,
                timestamp: block.timestamp,
                chainId: chainConfig.chainId,
                chainName: chainConfig.name,
                deployer: deployer,
                status: status,
                error: errorMsg
            })
        );

        console.log("");
    }

    /// @notice Verify contracts on block explorer
    function _verifyContracts(string memory chainName, address distributionCreator, address distributor) internal {
        console.log("Verifying contracts...");

        // Verify DistributionCreator
        try vm.tryFfi(_buildVerifyCommand(chainName, distributionCreator, "DistributionCreator")) {
            console.log("DistributionCreator verified");
        } catch {
            console.log("Warning: DistributionCreator verification failed (run manually if needed)");
        }

        // Verify Distributor
        try vm.tryFfi(_buildVerifyCommand(chainName, distributor, "Distributor")) {
            console.log("Distributor verified");
        } catch {
            console.log("Warning: Distributor verification failed (run manually if needed)");
        }
    }

    /// @notice Build verification command
    function _buildVerifyCommand(
        string memory chainName,
        address contractAddress,
        string memory contractName
    ) internal pure returns (string[] memory) {
        string[] memory args = new string[](7);
        args[0] = "forge";
        args[1] = "verify-contract";
        args[2] = vm.toString(contractAddress);
        args[3] = string.concat("contracts/", contractName, ".sol:", contractName);
        args[4] = "--chain";
        args[5] = chainName;
        args[6] = "--watch";
        return args;
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.17;

import { console } from "forge-std/console.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { BaseScript } from "./utils/Base.s.sol";
import { Disputer } from "../contracts/Disputer.sol";
import { Distributor } from "../contracts/Distributor.sol";
import { MockToken } from "../contracts/mock/MockToken.sol";
import { TokensUtils } from "./utils/TokensUtils.sol";
import { CreateXConstants } from "./utils/CreateXConstants.sol";

// Base contract with shared constants and utilities
contract DisputerScript is BaseScript {
    address[] public DISPUTER_WHITELIST = [
        0xeA05F9001FbDeA6d4280879f283Ff9D0b282060e,
        0x0dd2Ea40A3561C309C03B96108e78d06E8A1a99B,
        0xF4c94b2FdC2efA4ad4b831f312E7eF74890705DA
    ];
}

// Deploy scrip
contract Deploy is DisputerScript {
    function run() external broadcast {
        uint256 chainId = block.chainid;
        console.log("DEPLOYER_ADDRESS:", broadcaster);

        // Read configuration from JSON
        // TODO: replace
        address angleLabs = address(0);
        address distributor = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;

        address disputer = address(
            new Disputer{ salt: vm.envBytes32("DEPLOY_SALT") }(broadcaster, DISPUTER_WHITELIST, Distributor(distributor))
        );
        Disputer(disputer).transferOwnership(angleLabs);

        console.log("Disputer deployed at:", disputer);
    }
}

// SetDistributor scrip
contract SetDistributor is DisputerScript {
    function run(Distributor newDistributor) external {
        _run(newDistributor);
    }

    function run() external {
        // MODIFY THIS VALUE TO SET YOUR DESIRED DISTRIBUTOR ADDRESS
        address distributorAddress = address(0);
        _run(Distributor(distributorAddress));
    }

    function _run(Distributor _newDistributor) internal broadcast {
        uint256 chainId = block.chainid;
        // TODO: replace
        address disputerAddress = address(0);

        Disputer(disputerAddress).setDistributor(_newDistributor);

        console.log("Distributor updated to:", address(_newDistributor));
    }
}

// AddToWhitelist scrip
contract AddToWhitelist is DisputerScript {
    function run(address account) external {
        _run(account);
    }

    function run() external {
        // MODIFY THIS VALUE TO SET THE ACCOUNT TO WHITELIST
        address account = address(0);
        _run(account);
    }

    function _run(address _account) internal broadcast {
        uint256 chainId = block.chainid;
        // TODO: replace
        address disputerAddress = address(0);

        Disputer(disputerAddress).addToWhitelist(_account);

        console.log("Address added to whitelist:", _account);
    }
}

// RemoveFromWhitelist scrip
contract RemoveFromWhitelist is DisputerScript {
    function run(address account) external {
        _run(account);
    }

    function run() external {
        // MODIFY THIS VALUE TO SET THE ACCOUNT TO REMOVE FROM WHITELIST
        address accountToRemove = address(0);
        _run(accountToRemove);
    }

    function _run(address _account) internal broadcast {
        uint256 chainId = block.chainid;
        // TODO: replace
        address disputerAddress = address(0);

        Disputer(disputerAddress).removeFromWhitelist(_account);
        console.log("Address removed from whitelist:", _account);
    }
}

// FundDisputerWhitelist script
contract FundDisputerWhitelist is DisputerScript {
    function run(uint256 amountToFund) external {
        _run(amountToFund);
    }

    function run() external {
        // MODIFY THIS VALUE TO SET THE FUNDING AMOUNT (in ether)
        uint256 amountToFund = 0.001 ether;
        _run(amountToFund);
    }

    function _run(uint256 _amountToFund) internal broadcast {
        console.log("Chain ID:", block.chainid);

        // Fund each whitelisted address
        for (uint256 i = 0; i < DISPUTER_WHITELIST.length; i++) {
            address recipient = DISPUTER_WHITELIST[i];
            console.log("Funding whitelist address:", recipient);

            // Transfer native token
            (bool success, ) = recipient.call{ value: _amountToFund }("");
            require(success, "Transfer failed");

            console.log("Funded with amount:", _amountToFund);
        }

        // Print summary
        console.log("\n=== Funding Summary ===");
        console.log("Amount per address:", _amountToFund);
        console.log("Number of addresses funded:", DISPUTER_WHITELIST.length);
    }
}

contract FundDisputer is DisputerScript {
    function run(uint256 amountToFund) external {
        _run(amountToFund);
    }

    function run() external {
        // MODIFY THIS VALUE TO SET THE FUNDING AMOUNT (in dispute tokens decimals)
        uint256 amountToFund = 100 * 10 ** 6; // i.e. 100 USDC -> 100 * 10 ** 6
        _run(amountToFund);
    }

    function _run(uint256 _amountToFund) internal broadcast {
        uint256 chainId = block.chainid;
        // TODO: replace
        address disputerAddress = address(0);

        IERC20 disputeToken = Disputer(disputerAddress).distributor().disputeToken();
        console.log("Transferring %s to %s", _amountToFund, disputerAddress);
        disputeToken.transfer(disputerAddress, _amountToFund);
    }
}

contract WithdrawFunds is DisputerScript {
    function run() external {
        // MODIFY THESE VALUES TO SET THE WITHDRAWAL PARAMETERS
        address asset = address(0); // Use address(0) for ETH, or token address for ERC20
        uint256 amountToWithdraw = 100 * 10 ** 6; // Adjust decimals according to asset
        address recipient = address(0); // Set the recipient address
        _run(asset, recipient, amountToWithdraw);
    }

    function run(address asset, address recipient, uint256 amountToWithdraw) external {
        _run(asset, recipient, amountToWithdraw);
    }

    function _run(address asset, address to, uint256 _amountToWithdraw) internal broadcast {
        uint256 chainId = block.chainid;
        // TODO: replace
        address disputerAddress = address(0);
        Disputer disputer = Disputer(disputerAddress);

        if (asset == address(0)) {
            // Withdraw ETH
            disputer.withdrawFunds(payable(to), _amountToWithdraw);
            console.log("Withdrew %s ETH to %s", _amountToWithdraw, to);
        } else {
            // Withdraw ERC20 token
            disputer.withdrawFunds(asset, to, _amountToWithdraw);
            console.log("Withdrew %s %s to %s", _amountToWithdraw, asset, to);
        }
    }
}

// ToggleDispute script
contract ToggleDispute is DisputerScript {
    function run(string memory reason) external {
        _run(reason);
    }

    function run() external {
        // MODIFY THIS VALUE TO SET THE DISPUTE REASON TO TOGGLE
        string memory reason = "test";
        _run(reason);
    }

    function _run(string memory _reason) internal broadcast {
        uint256 chainId = block.chainid;
        // TODO: replace
        address disputerAddress = address(0);
        Disputer(disputerAddress).toggleDispute(_reason);
        console.log("Toggled dispute for:", _reason);
    }
}

contract DeployWithCreate is DisputerScript, TokensUtils, CreateXConstants {
    function run(address disputeToken) external broadcast {
        uint256 chainId = block.chainid;
        address distributor = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;

        deployDisputer(distributor, disputeToken);
    }

    function deployDisputer(address distributor, address disputeToken) public returns (address) {
        console.log("\n=== Deploying Disputer ===");

        bytes32 salt = vm.envBytes32("DEPLOY_SALT");

        // Check if CREATEX contract is deployed
        address disputer;
        // if (CREATEX.code.length == 0) {
        address CREATE2_DEPLOYER = 0x4e59b44847b379578588920cA78FbF26c0B4956C;
        // if (CREATE2_DEPLOYER.code.length != 0) {

        //     // Deploy using the standard Deterministic CREATE2 deployer and a deterministic salt
        //     disputer = address(new Disputer{ salt: salt }(broadcaster, DISPUTER_WHITELIST, Distributor(distributor)));
        // } else {
        //     // Classic deployment if CREATE2 deployer is not deployed
        //     // disputer = address(new Disputer(broadcaster, DISPUTER_WHITELIST, Distributor(distributor)));
        // }
        // } else {
        // Deploy using CreateX
        // Create initialization bytecode
        bytes
            memory bytecode = hex"60806040523480156200001157600080fd5b506040516200137738038062001377833981016040819052620000349162000322565b6200003f33620001b3565b600180546001600160a01b0319166001600160a01b0383169081179091556040805163c748d26160e01b8152905163c748d261916004808201926020929091908290030181865afa15801562000099573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000bf91906200041f565b60405163095ea7b360e01b81526001600160a01b0383811660048301526000196024830152919091169063095ea7b3906044016020604051808303816000875af115801562000112573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000138919062000446565b50815160005b818110156200019d576001600260008684815181106200016257620001626200046a565b6020908102919091018101516001600160a01b03168252810191909152604001600020805460ff19169115159190911790556001016200013e565b50620001a98462000203565b5050505062000480565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6200020d62000286565b6001600160a01b038116620002785760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084015b60405180910390fd5b6200028381620001b3565b50565b6000546001600160a01b03163314620002e25760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016200026f565b565b6001600160a01b03811681146200028357600080fd5b80516200030781620002e4565b919050565b634e487b7160e01b600052604160045260246000fd5b6000806000606084860312156200033857600080fd5b83516200034581620002e4565b602085810151919450906001600160401b03808211156200036557600080fd5b818701915087601f8301126200037a57600080fd5b8151818111156200038f576200038f6200030c565b8060051b604051601f19603f83011681018181108582111715620003b757620003b76200030c565b60405291825284820192508381018501918a831115620003d657600080fd5b938501935b82851015620003ff57620003ef85620002fa565b84529385019392850192620003db565b8097505050505050506200041660408501620002fa565b90509250925092565b6000602082840312156200043257600080fd5b81516200043f81620002e4565b9392505050565b6000602082840312156200045957600080fd5b815180151581146200043f57600080fd5b634e487b7160e01b600052603260045260246000fd5b610ee780620004906000396000f3fe608060405234801561001057600080fd5b50600436106100c95760003560e01c80639b19251a11610081578063ca85e5d01161005b578063ca85e5d0146101bb578063e43252d7146101ce578063f2fde38b146101e157600080fd5b80639b19251a14610155578063bfe1092814610188578063c1075329146101a857600080fd5b806375619ab5116100b257806375619ab5146100eb5780638ab1d681146100fe5780638da5cb5b1461011157600080fd5b80631c20fadd146100ce578063715018a6146100e3575b600080fd5b6100e16100dc366004610c1d565b6101f4565b005b6100e161029b565b6100e16100f9366004610c5e565b6102af565b6100e161010c366004610c5e565b61055e565b60005473ffffffffffffffffffffffffffffffffffffffff165b60405173ffffffffffffffffffffffffffffffffffffffff90911681526020015b60405180910390f35b610178610163366004610c5e565b60026020526000908152604090205460ff1681565b604051901515815260200161014c565b60015461012b9073ffffffffffffffffffffffffffffffffffffffff1681565b6100e16101b6366004610c82565b6105b2565b6100e16101c9366004610cdd565b610659565b6100e16101dc366004610c5e565b6109f2565b6100e16101ef366004610c5e565b610a49565b6101fc610b05565b6040517fa9059cbb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff83811660048301526024820183905284169063a9059cbb906044016020604051808303816000875af1158015610271573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102959190610dac565b50505050565b6102a3610b05565b6102ad6000610b86565b565b6102b7610b05565b600160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663c748d2616040518163ffffffff1660e01b8152600401602060405180830381865afa158015610324573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103489190610dce565b6001546040517f095ea7b300000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff91821660048201526000602482015291169063095ea7b3906044016020604051808303816000875af11580156103c0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103e49190610dac565b50600180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff8316908117909155604080517fc748d261000000000000000000000000000000000000000000000000000000008152905163c748d261916004808201926020929091908290030181865afa15801561047c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104a09190610dce565b6040517f095ea7b300000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff83811660048301527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6024830152919091169063095ea7b3906044016020604051808303816000875af1158015610536573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061055a9190610dac565b5050565b610566610b05565b73ffffffffffffffffffffffffffffffffffffffff16600090815260026020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00169055565b6105ba610b05565b60008273ffffffffffffffffffffffffffffffffffffffff168260405160006040518083038185875af1925050503d8060008114610614576040519150601f19603f3d011682016040523d82523d6000602084013e610619565b606091505b5050905080610654576040517f27fcd9d100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505050565b3360009081526002602052604090205460ff166106a2576040517f584a793800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600154604080517fc748d261000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163c748d2619160048083019260209291908290030181865afa158015610712573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107369190610dce565b90506000600160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166309454ba36040518163ffffffff1660e01b8152600401602060405180830381865afa1580156107a7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107cb9190610deb565b6040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015290915060009073ffffffffffffffffffffffffffffffffffffffff8416906370a0823190602401602060405180830381865afa15801561083b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061085f9190610deb565b9050818110156109645773ffffffffffffffffffffffffffffffffffffffff83166323b872dd33306108918587610e04565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e086901b16815273ffffffffffffffffffffffffffffffffffffffff938416600482015292909116602483015260448201526064016020604051808303816000875af115801561090a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061092e9190610dac565b610964576040517fb1eb39bb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001546040517f2a25dd4100000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff90911690632a25dd41906109ba908790600401610e44565b600060405180830381600087803b1580156109d457600080fd5b505af11580156109e8573d6000803e3d6000fd5b5050505050505050565b6109fa610b05565b73ffffffffffffffffffffffffffffffffffffffff16600090815260026020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001179055565b610a51610b05565b73ffffffffffffffffffffffffffffffffffffffff8116610af9576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f646472657373000000000000000000000000000000000000000000000000000060648201526084015b60405180910390fd5b610b0281610b86565b50565b60005473ffffffffffffffffffffffffffffffffffffffff1633146102ad576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610af0565b6000805473ffffffffffffffffffffffffffffffffffffffff8381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b73ffffffffffffffffffffffffffffffffffffffff81168114610b0257600080fd5b600080600060608486031215610c3257600080fd5b8335610c3d81610bfb565b92506020840135610c4d81610bfb565b929592945050506040919091013590565b600060208284031215610c7057600080fd5b8135610c7b81610bfb565b9392505050565b60008060408385031215610c9557600080fd5b8235610ca081610bfb565b946020939093013593505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b600060208284031215610cef57600080fd5b813567ffffffffffffffff80821115610d0757600080fd5b818401915084601f830112610d1b57600080fd5b813581811115610d2d57610d2d610cae565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0908116603f01168101908382118183101715610d7357610d73610cae565b81604052828152876020848701011115610d8c57600080fd5b826020860160208301376000928101602001929092525095945050505050565b600060208284031215610dbe57600080fd5b81518015158114610c7b57600080fd5b600060208284031215610de057600080fd5b8151610c7b81610bfb565b600060208284031215610dfd57600080fd5b5051919050565b81810381811115610e3e577f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b92915050565b60006020808352835180602085015260005b81811015610e7257858101830151858201604001528201610e56565b5060006040828601015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f830116850101925050509291505056fea26469706673582212202c48aee5de4959ef518786fa36afffd4b4a0c2f4af9e25741d18037fdf60170b64736f6c63430008180033000000000000000000000000a9ddd91249dfdd450e81e1c56ab60e1a6265170100000000000000000000000000000000000000000000000000000000000000600000000000000000000000003ef3d8ba38ebe18db133cec108f4d14ce00dd9ae0000000000000000000000000000000000000000000000000000000000000003000000000000000000000000ea05f9001fbdea6d4280879f283ff9d0b282060e0000000000000000000000000dd2ea40a3561c309c03b96108e78d06e8a1a99b000000000000000000000000f4c94b2fdc2efa4ad4b831f312e7ef74890705da";

        // Deploy using the specified CREATE2 deployer and a deterministic salt
        bytes memory callData = abi.encodeWithSignature("deployCreate2(bytes32,bytes)", salt, bytecode);
        (bool success, bytes memory returnData) = CREATEX.call(callData);

        require(success, "CREATE2 deployment failed");
        disputer = address(uint160(uint256(bytes32(returnData))));

        console.log("Disputer:", disputer);

        // // Transfer ownership to multisig
        // console.log("Transferred Disputer ownership to multisig:", multisig);
        // Disputer(disputer).transferOwnership(multisig);

        // // Send dispute tokens to disputer
        // uint256 amount = 100 * 10 ** MockToken(disputeToken).decimals();
        // if (MockToken(disputeToken).balanceOf(broadcaster) >= amount) {
        //     transferERC20Tokens(disputer, 100 * 10 ** MockToken(disputeToken).decimals(), disputeToken);
        //     console.log("Sent dispute tokens to disputer:", 100 * 10 ** MockToken(disputeToken).decimals());
        // }

        return address(disputer);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.17;

import { Script } from "forge-std/Script.sol";
import { TransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { ProxyAdmin } from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { console } from "forge-std/console.sol";

import { CreateXConstants } from "./utils/CreateXConstants.sol";
import { TokensUtils } from "./utils/TokensUtils.sol";

import { AccessControlManager } from "../contracts/AccessControlManager.sol";
import { Disputer } from "../contracts/Disputer.sol";
import { Distributor } from "../contracts/Distributor.sol";
import { DistributionCreator } from "../contracts/DistributionCreator.sol";
import { IAccessControlManager } from "../contracts/interfaces/IAccessControlManager.sol";
import { MockToken } from "../contracts/mock/MockToken.sol";

// NOTE: Before running this script on a new chain, make sure to create the AngleLabs multisig and update the sdk with the new address
// Can be executed with (using zero address as default will fetch addresses from the sdk registry):
// forge script scripts/merklDeploy.s.sol \
//     --rpc-url $RPC_URL \
//     -vvvv
// Can be also be executed with (using zero address as default with addresses from the sdk registry):
// forge script scripts/merklDeploy.s.sol \
//     --sig "run(address,address)" \
//     "0x0000000000000000000000000000000000000000" "0x0000000000000000000000000000000000000000" \
//     --rpc-url $RPC_URL \
//     -vvvv
contract MainDeployScript is Script, TokensUtils, CreateXConstants {
    uint256 private DEPLOYER_PRIVATE_KEY;
    uint256 private MERKL_DEPLOYER_PRIVATE_KEY;

    // Constants and storage
    address public KEEPER = 0x435046800Fb9149eE65159721A92cB7d50a7534b;
    address public DUMPER = 0xeaC6A75e19beB1283352d24c0311De865a867DAB;
    address public TEMP_GOVERNOR = 0xb08AB4332AD871F89da24df4751968A61e58013c;
    address public GUARDIAN_ADDRESS = 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701; // also deployer v2
    address public EXPECTED_MERKL_DEPLOYER_ADDRESS = 0x9f76a95AA7535bb0893cf88A146396e00ed21A12;

    address[] public DISPUTER_WHITELIST = [
        0xeA05F9001FbDeA6d4280879f283Ff9D0b282060e,
        0x0dd2Ea40A3561C309C03B96108e78d06E8A1a99B,
        0xF4c94b2FdC2efA4ad4b831f312E7eF74890705DA
    ];

    uint256 public FUND_AMOUNT = 0.001 ether;

    address public ANGLE_LABS;
    address public DEPLOYER_ADDRESS;
    address public MERKL_DEPLOYER_ADDRESS;
    address public DISPUTE_TOKEN;

    struct DeploymentAddresses {
        address proxy;
        address implementation;
    }

    // NOTE: This function is used to automatically set the ANGLE_LABS and DISPUTE_TOKEN addresses from the sdk registry
    function run() external {
        DISPUTE_TOKEN = address(0);

        ANGLE_LABS = TEMP_GOVERNOR;

        _run(ANGLE_LABS, DISPUTE_TOKEN);
    }

    // NOTE: This function is used to manually set the ANGLE_LABS and DISPUTE_TOKEN addresses.
    // If angleLabs or disputeToken are set to the zero address, the script will try to fetch the addresses from the sdk registry
    function run(address angleLabs, address disputeToken) external {
        // Setup
        if (disputeToken != address(0)) {
            DISPUTE_TOKEN = disputeToken;
        } else {
            DISPUTE_TOKEN = address(0);
        }

        if (angleLabs != address(0)) {
            ANGLE_LABS = angleLabs;
        } else {
            ANGLE_LABS = TEMP_GOVERNOR;
        }

        _run(ANGLE_LABS, DISPUTE_TOKEN);
    }

    function _run(address angleLabs, address disputeToken) internal {
        ANGLE_LABS = angleLabs;
        DISPUTE_TOKEN = disputeToken;

        DEPLOYER_PRIVATE_KEY = vm.envUint("DEPLOYER_PRIVATE_KEY");
        MERKL_DEPLOYER_PRIVATE_KEY = vm.envUint("MERKL_DEPLOYER_PRIVATE_KEY");

        console.log("Chain ID:", block.chainid);
        console.log("ANGLE_LABS:", ANGLE_LABS);

        // Compute addresses from private keys
        DEPLOYER_ADDRESS = vm.addr(DEPLOYER_PRIVATE_KEY);
        MERKL_DEPLOYER_ADDRESS = vm.addr(MERKL_DEPLOYER_PRIVATE_KEY);
        console.log("DEPLOYER_ADDRESS:", DEPLOYER_ADDRESS);
        console.log("MERKL_DEPLOYER_ADDRESS:", MERKL_DEPLOYER_ADDRESS);
        console.log("DISPUTE TOKEN:", DISPUTE_TOKEN);

        if (DEPLOYER_ADDRESS == ANGLE_LABS) revert("ANGLE_LABS cannot be the deployer address");
        if (DEPLOYER_ADDRESS == EXPECTED_MERKL_DEPLOYER_ADDRESS) revert("DEPLOYER_ADDRESS cannot be the merkl deployer address"); // prevent from using the merkl deployer private key as deployer private key
        if (MERKL_DEPLOYER_ADDRESS != EXPECTED_MERKL_DEPLOYER_ADDRESS)
            revert("MERKL_DEPLOYER_ADDRESS is not the expected merkl deployer address"); // guarantee that MERKL_DEPLOYER_ADDRESS is the merkl deployer address

        // 1. Deploy using DEPLOYER_PRIVATE_KEY
        vm.startBroadcast(DEPLOYER_PRIVATE_KEY);

        // Transfer initial funds to required addresses
        transferInitialFunds();

        // Deploy ProxyAdmin
        address proxyAdmin = deployProxyAdmin();
        // Deploy AccessControlManager
        DeploymentAddresses memory accessControlManager = deployAccessControl(proxyAdmin);
        // Deploy AglaMerkl
        address aglaMerkl = deployAglaMerkl();

        vm.stopBroadcast();

        // 2. Deploy using MERKL_DEPLOYER_PRIVATE_KEY
        vm.startBroadcast(MERKL_DEPLOYER_PRIVATE_KEY);

        verifyMerklNonces();

        // Deploy Distributor
        DeploymentAddresses memory distributor = deployDistributor(accessControlManager.proxy);
        // Deploy DistributionCreator
        DeploymentAddresses memory creator = deployDistributionCreator(accessControlManager.proxy, distributor.proxy);

        vm.stopBroadcast();

        // 3. Set params and deploy Disputer using DEPLOYER_PRIVATE_KEY (make sure that the deployer is the GUARDIAN_ADDRESS)
        vm.startBroadcast(DEPLOYER_PRIVATE_KEY);

        // Set params and transfer ownership
        setDistributionCreatorParams(address(creator.proxy), aglaMerkl, DUMPER);
        setDistributorParams(address(distributor.proxy), DISPUTE_TOKEN, KEEPER);

        // Deploy Disputer
        address disputer = deployDisputer(distributor.proxy);

        // Revoke GOVENOR from DEPLOYER_ADDRESS if deployer is GUARDIAN_ADDRESS (keeping GUARDIAN role), else revoke both roles by calling removeGovernor
        if (DEPLOYER_ADDRESS == GUARDIAN_ADDRESS) {
            if (
                AccessControlManager(accessControlManager.proxy).getRoleMemberCount(
                    AccessControlManager(accessControlManager.proxy).GOVERNOR_ROLE()
                ) > 1
            ) {
                AccessControlManager(accessControlManager.proxy).revokeRole(
                    AccessControlManager(accessControlManager.proxy).GOVERNOR_ROLE(),
                    DEPLOYER_ADDRESS
                );
            } else {
                console.log("No governor to revoke, there must have been an error in the deployment");
            }
        } else {
            // removeGovernor already checks that there is at least one governor
            AccessControlManager(accessControlManager.proxy).removeGovernor(DEPLOYER_ADDRESS);
        }

        vm.stopBroadcast();

        // Print summary
        console.log("\n=== Deployment Summary ===");
        console.log("ProxyAdmin:");
        console.log("  - Address:", proxyAdmin);
        console.log("AccessControlManager:");
        console.log("  - Proxy:", accessControlManager.proxy);
        console.log("  - Implementation:", accessControlManager.implementation);
        console.log("Distributor:");
        console.log("  - Proxy:", distributor.proxy);
        console.log("  - Implementation:", distributor.implementation);
        console.log("DistributionCreator:");
        console.log("  - Proxy:", creator.proxy);
        console.log("  - Implementation:", creator.implementation);
        if (disputer != address(0)) {
            console.log("Disputer:");
            console.log("  - Address:", disputer);
        }
        console.log("AglaMerkl:");
        console.log("  - Address:", aglaMerkl);
    }
    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                   DEPLOY FUNCTIONS                                                 
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    function deployProxyAdmin() public returns (address) {
        console.log("\n=== Deploying ProxyAdmin ===");

        // Deploy ProxyAdmin
        ProxyAdmin proxyAdmin = new ProxyAdmin();
        console.log("ProxyAdmin:", address(proxyAdmin));

        // Transfer ownership
        proxyAdmin.transferOwnership(ANGLE_LABS);
        console.log("Transferred ProxyAdmin ownership to:", ANGLE_LABS);

        return address(proxyAdmin);
    }

    function deployAccessControl(address proxyAdmin) public returns (DeploymentAddresses memory) {
        console.log("\n=== Deploying AccessControlManager ===");

        // Deploy implementation
        AccessControlManager implementation = new AccessControlManager();
        console.log("AccessControlManager Implementation:", address(implementation));

        // Prepare initialization data
        bytes memory initData = abi.encodeCall(AccessControlManager.initialize, (DEPLOYER_ADDRESS, ANGLE_LABS));

        // Deploy proxy
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(address(implementation), address(proxyAdmin), initData);
        console.log("AccessControlManager Proxy:", address(proxy));

        AccessControlManager(address(proxy)).addGovernor(ANGLE_LABS);
        return DeploymentAddresses(address(proxy), address(implementation));
    }

    function deployDistributor(address accessControlManager) public returns (DeploymentAddresses memory) {
        console.log("\n=== Deploying Distributor ===");

        // Deploy implementation
        Distributor implementation = new Distributor();
        console.log("Distributor Implementation:", address(implementation));

        // Deploy proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), "");
        console.log("Distributor Proxy:", address(proxy));

        // Initialize
        Distributor(address(proxy)).initialize(IAccessControlManager(accessControlManager));

        return DeploymentAddresses(address(proxy), address(implementation));
    }

    function deployDistributionCreator(address accessControlManager, address distributor) public returns (DeploymentAddresses memory) {
        console.log("\n=== Deploying DistributionCreator ===");

        // Deploy implementation
        DistributionCreator implementation = new DistributionCreator();
        console.log("DistributionCreator Implementation:", address(implementation));

        // Deploy proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), "");
        console.log("DistributionCreator Proxy:", address(proxy));

        // Initialize
        DistributionCreator(address(proxy)).initialize(
            IAccessControlManager(accessControlManager),
            distributor,
            0.03 gwei // 0.03 gwei
        );

        return DeploymentAddresses(address(proxy), address(implementation));
    }

    function deployDisputer(address distributor) public returns (address) {
        console.log("\n=== Deploying Disputer ===");

        bytes32 salt = vm.envBytes32("DEPLOY_SALT");

        // Check if deployer is the guardian
        if (DEPLOYER_ADDRESS != GUARDIAN_ADDRESS) {
            console.log("Skipping Disputer deployment - deployer is not the guardian");
            return address(0);
        }

        // Check if dispute token is set
        if (address(Distributor(distributor).disputeToken()) == address(0)) {
            console.log("Skipping Disputer deployment - dispute token not set");
            return address(0);
        }

        // Check if CREATEX contract is deployed
        address disputer;
        if (CREATEX.code.length == 0) {
            address CREATE2_DEPLOYER = 0x4e59b44847b379578588920cA78FbF26c0B4956C;
            if (CREATE2_DEPLOYER.code.length != 0) {
                // Deploy using the standard Deterministic CREATE2 deployer and a deterministic salt
                disputer = address(new Disputer{ salt: salt }(DEPLOYER_ADDRESS, DISPUTER_WHITELIST, Distributor(distributor)));
            } else {
                // Classic deployment if CREATE2 deployer is not deployed
                disputer = address(new Disputer(DEPLOYER_ADDRESS, DISPUTER_WHITELIST, Distributor(distributor)));
            }
        } else {
            // Deploy using CreateX
            // Create initialization bytecode
            bytes
                memory bytecode = hex"60806040523480156200001157600080fd5b506040516200137738038062001377833981016040819052620000349162000322565b6200003f33620001b3565b600180546001600160a01b0319166001600160a01b0383169081179091556040805163c748d26160e01b8152905163c748d261916004808201926020929091908290030181865afa15801562000099573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000bf91906200041f565b60405163095ea7b360e01b81526001600160a01b0383811660048301526000196024830152919091169063095ea7b3906044016020604051808303816000875af115801562000112573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000138919062000446565b50815160005b818110156200019d576001600260008684815181106200016257620001626200046a565b6020908102919091018101516001600160a01b03168252810191909152604001600020805460ff19169115159190911790556001016200013e565b50620001a98462000203565b5050505062000480565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6200020d62000286565b6001600160a01b038116620002785760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084015b60405180910390fd5b6200028381620001b3565b50565b6000546001600160a01b03163314620002e25760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016200026f565b565b6001600160a01b03811681146200028357600080fd5b80516200030781620002e4565b919050565b634e487b7160e01b600052604160045260246000fd5b6000806000606084860312156200033857600080fd5b83516200034581620002e4565b602085810151919450906001600160401b03808211156200036557600080fd5b818701915087601f8301126200037a57600080fd5b8151818111156200038f576200038f6200030c565b8060051b604051601f19603f83011681018181108582111715620003b757620003b76200030c565b60405291825284820192508381018501918a831115620003d657600080fd5b938501935b82851015620003ff57620003ef85620002fa565b84529385019392850192620003db565b8097505050505050506200041660408501620002fa565b90509250925092565b6000602082840312156200043257600080fd5b81516200043f81620002e4565b9392505050565b6000602082840312156200045957600080fd5b815180151581146200043f57600080fd5b634e487b7160e01b600052603260045260246000fd5b610ee780620004906000396000f3fe608060405234801561001057600080fd5b50600436106100c95760003560e01c80639b19251a11610081578063ca85e5d01161005b578063ca85e5d0146101bb578063e43252d7146101ce578063f2fde38b146101e157600080fd5b80639b19251a14610155578063bfe1092814610188578063c1075329146101a857600080fd5b806375619ab5116100b257806375619ab5146100eb5780638ab1d681146100fe5780638da5cb5b1461011157600080fd5b80631c20fadd146100ce578063715018a6146100e3575b600080fd5b6100e16100dc366004610c1d565b6101f4565b005b6100e161029b565b6100e16100f9366004610c5e565b6102af565b6100e161010c366004610c5e565b61055e565b60005473ffffffffffffffffffffffffffffffffffffffff165b60405173ffffffffffffffffffffffffffffffffffffffff90911681526020015b60405180910390f35b610178610163366004610c5e565b60026020526000908152604090205460ff1681565b604051901515815260200161014c565b60015461012b9073ffffffffffffffffffffffffffffffffffffffff1681565b6100e16101b6366004610c82565b6105b2565b6100e16101c9366004610cdd565b610659565b6100e16101dc366004610c5e565b6109f2565b6100e16101ef366004610c5e565b610a49565b6101fc610b05565b6040517fa9059cbb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff83811660048301526024820183905284169063a9059cbb906044016020604051808303816000875af1158015610271573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102959190610dac565b50505050565b6102a3610b05565b6102ad6000610b86565b565b6102b7610b05565b600160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663c748d2616040518163ffffffff1660e01b8152600401602060405180830381865afa158015610324573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103489190610dce565b6001546040517f095ea7b300000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff91821660048201526000602482015291169063095ea7b3906044016020604051808303816000875af11580156103c0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103e49190610dac565b50600180547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff8316908117909155604080517fc748d261000000000000000000000000000000000000000000000000000000008152905163c748d261916004808201926020929091908290030181865afa15801561047c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104a09190610dce565b6040517f095ea7b300000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff83811660048301527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6024830152919091169063095ea7b3906044016020604051808303816000875af1158015610536573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061055a9190610dac565b5050565b610566610b05565b73ffffffffffffffffffffffffffffffffffffffff16600090815260026020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00169055565b6105ba610b05565b60008273ffffffffffffffffffffffffffffffffffffffff168260405160006040518083038185875af1925050503d8060008114610614576040519150601f19603f3d011682016040523d82523d6000602084013e610619565b606091505b5050905080610654576040517f27fcd9d100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505050565b3360009081526002602052604090205460ff166106a2576040517f584a793800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600154604080517fc748d261000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163c748d2619160048083019260209291908290030181865afa158015610712573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107369190610dce565b90506000600160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166309454ba36040518163ffffffff1660e01b8152600401602060405180830381865afa1580156107a7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107cb9190610deb565b6040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015290915060009073ffffffffffffffffffffffffffffffffffffffff8416906370a0823190602401602060405180830381865afa15801561083b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061085f9190610deb565b9050818110156109645773ffffffffffffffffffffffffffffffffffffffff83166323b872dd33306108918587610e04565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e086901b16815273ffffffffffffffffffffffffffffffffffffffff938416600482015292909116602483015260448201526064016020604051808303816000875af115801561090a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061092e9190610dac565b610964576040517fb1eb39bb00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001546040517f2a25dd4100000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff90911690632a25dd41906109ba908790600401610e44565b600060405180830381600087803b1580156109d457600080fd5b505af11580156109e8573d6000803e3d6000fd5b5050505050505050565b6109fa610b05565b73ffffffffffffffffffffffffffffffffffffffff16600090815260026020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00166001179055565b610a51610b05565b73ffffffffffffffffffffffffffffffffffffffff8116610af9576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f646472657373000000000000000000000000000000000000000000000000000060648201526084015b60405180910390fd5b610b0281610b86565b50565b60005473ffffffffffffffffffffffffffffffffffffffff1633146102ad576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610af0565b6000805473ffffffffffffffffffffffffffffffffffffffff8381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b73ffffffffffffffffffffffffffffffffffffffff81168114610b0257600080fd5b600080600060608486031215610c3257600080fd5b8335610c3d81610bfb565b92506020840135610c4d81610bfb565b929592945050506040919091013590565b600060208284031215610c7057600080fd5b8135610c7b81610bfb565b9392505050565b60008060408385031215610c9557600080fd5b8235610ca081610bfb565b946020939093013593505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b600060208284031215610cef57600080fd5b813567ffffffffffffffff80821115610d0757600080fd5b818401915084601f830112610d1b57600080fd5b813581811115610d2d57610d2d610cae565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0908116603f01168101908382118183101715610d7357610d73610cae565b81604052828152876020848701011115610d8c57600080fd5b826020860160208301376000928101602001929092525095945050505050565b600060208284031215610dbe57600080fd5b81518015158114610c7b57600080fd5b600060208284031215610de057600080fd5b8151610c7b81610bfb565b600060208284031215610dfd57600080fd5b5051919050565b81810381811115610e3e577f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b92915050565b60006020808352835180602085015260005b81811015610e7257858101830151858201604001528201610e56565b5060006040828601015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f830116850101925050509291505056fea26469706673582212202c48aee5de4959ef518786fa36afffd4b4a0c2f4af9e25741d18037fdf60170b64736f6c63430008180033000000000000000000000000a9ddd91249dfdd450e81e1c56ab60e1a6265170100000000000000000000000000000000000000000000000000000000000000600000000000000000000000003ef3d8ba38ebe18db133cec108f4d14ce00dd9ae0000000000000000000000000000000000000000000000000000000000000003000000000000000000000000ea05f9001fbdea6d4280879f283ff9d0b282060e0000000000000000000000000dd2ea40a3561c309c03b96108e78d06e8a1a99b000000000000000000000000f4c94b2fdc2efa4ad4b831f312e7ef74890705da";

            // Deploy using the specified CREATE2 deployer and a deterministic salt
            bytes memory callData = abi.encodeWithSignature("deployCreate2(bytes32,bytes)", salt, bytecode);
            (bool success, bytes memory returnData) = CREATEX.call(callData);

            require(success, "CREATE2 deployment failed");
            disputer = address(uint160(uint256(bytes32(returnData))));
        }

        console.log("Disputer:", disputer);

        // Transfer ownership to AngleLabs
        console.log("Transferred Disputer ownership to AngleLabs:", ANGLE_LABS);
        Disputer(disputer).transferOwnership(ANGLE_LABS);

        // Send dispute tokens to disputer
        uint256 amount = 200 * 10 ** MockToken(DISPUTE_TOKEN).decimals();
        if (MockToken(DISPUTE_TOKEN).balanceOf(DEPLOYER_ADDRESS) >= amount) {
            transferERC20Tokens(disputer, 200 * 10 ** MockToken(DISPUTE_TOKEN).decimals(), DISPUTE_TOKEN);
            console.log("Sent dispute tokens to disputer:", 200 * 10 ** MockToken(DISPUTE_TOKEN).decimals());
        }

        return address(disputer);
    }

    function deployAglaMerkl() public returns (address) {
        console.log("\n=== Deploying AglaMerkl ===");

        // Deploy MockToken
        MockToken token = new MockToken("aglaMerkl", "aglaMerkl", 6);

        console.log("AglaMerkl Token:", address(token));
        console.log("Minting tokens to deployer:", DEPLOYER_ADDRESS);
        // Mint tokens to deployer
        token.mint(DEPLOYER_ADDRESS, 1_000_000_000_000_000_000_000_000_000);

        return address(token);
    }

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                        SETTERS                                                     
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    function setDistributionCreatorParams(address _distributionCreator, address aglaMerkl, address dumper) public {
        console.log("\n=== Setting DistributionCreator params ===");

        DistributionCreator distributionCreator = DistributionCreator(_distributionCreator);

        // Set min amount to 1 for reward tokens
        uint256[] memory minAmounts = new uint256[](1);
        address[] memory tokens = new address[](1);
        minAmounts[0] = 1;
        tokens[0] = aglaMerkl;
        console.log("Setting reward token min amounts to 1 for:", aglaMerkl);
        distributionCreator.setRewardTokenMinAmounts(tokens, minAmounts);

        // Set keeper as fee recipient
        console.log("Setting dumper as fee recipient:", dumper);
        distributionCreator.setFeeRecipient(dumper);

        // Set campaign fees to 5% for airdrop campaigns
        console.log("Setting campaign fees to 5% for airdrop campaigns");
        distributionCreator.setCampaignFees(4, 5 * 1e6);

        // Set message
        console.log("Setting message");
        distributionCreator.setMessage(
            " 1. Merkl is experimental software provided as is, use it at your own discretion. There may notably be delays in the onchain Merkle root updates and there may be flaws in the script (or engine) or in the infrastructure used to update results onchain. In that regard, everyone can permissionlessly dispute the rewards which are posted onchain, and when creating a distribution, you are responsible for checking the results and eventually dispute them. 2. If you are specifying an invalid pool address or a pool from an AMM that is not marked as supported, your rewards will not be taken into account and you will not be able to recover them. 3. If you do not blacklist liquidity position managers or smart contract addresses holding LP tokens that are not natively supported by the Merkl system, or if you don't specify the addresses of the liquidity position managers that are not automatically handled by the system, then the script will not be able to take the specifities of these addresses into account, and it will reward them like a normal externally owned account would be. If these are smart contracts that do not support external rewards, then rewards that should be accruing to it will be lost. 4. If rewards sent through Merkl remain unclaimed for a period of more than 1 year after the end of the distribution (because they are meant for instance for smart contract addresses that cannot claim or deal with them), then we reserve the right to recover these rewards. 5. Fees apply to incentives deposited on Merkl, unless the pools incentivized contain a whitelisted token (e.g an Angle Protocol stablecoin). 6. By interacting with the Merkl smart contract to deposit an incentive for a pool, you are exposed to smart contract risk and to the offchain mechanism used to compute reward distribution. 7. If the rewards you are sending are too small in value, or if you are sending rewards using a token that is not approved for it, your rewards will not be handled by the script, and they may be lost. 8. If you mistakenly send too much rewards compared with what you wanted to send, you will not be able to call them back. You will also not be able to prematurely end a reward distribution once created. 9. The engine handling reward distribution for a pool may not look at all the swaps occurring on the pool during the time for which you are incentivizing, but just at a subset of it to gain in efficiency. Overall, if you distribute incentives using Merkl, it means that you are aware of how the engine works, of the approximations it makes and of the behaviors it may trigger (e.g. just in time liquidity). 10. Rewards corresponding to incentives distributed through Merkl do not compound block by block, but are regularly made available (through a Merkle root update) at a frequency which depends on the chain. "
        );
    }

    function setDistributorParams(address _distributor, address disputeToken, address keeper) public {
        console.log("\n=== Setting Distributor params ===");
        Distributor distributor = Distributor(_distributor);

        // Toggle trusted status for keeper
        console.log("Toggling trusted status for keeper:", keeper);
        distributor.toggleTrusted(keeper);

        // Set dispute token (DISPUTE_TOKEN if available, skip otherwise)
        console.log("Setting dispute token:", disputeToken);
        if (disputeToken != address(0)) distributor.setDisputeToken(IERC20(disputeToken));

        // Set dispute period
        console.log("Setting dispute period to 1");
        distributor.setDisputePeriod(1);

        // Set dispute amount to 100 tokens (18 decimals)
        string memory symbol = MockToken(disputeToken).symbol();
        uint8 decimals = MockToken(disputeToken).decimals();
        console.log("Token decimals:", decimals);
        if (
            keccak256(abi.encodePacked(symbol)) == keccak256(abi.encodePacked("EURA")) ||
            keccak256(abi.encodePacked(symbol)) == keccak256(abi.encodePacked("USDC")) ||
            keccak256(abi.encodePacked(symbol)) == keccak256(abi.encodePacked("USDT"))
        ) {
            console.log("Setting dispute amount to 100", symbol);
            distributor.setDisputeAmount(100 * 10 ** decimals);
        }
        if (keccak256(abi.encodePacked(symbol)) == keccak256(abi.encodePacked("WETH"))) {
            console.log("Setting dispute amount to 0.03", symbol);
            distributor.setDisputeAmount(3 * 10 ** (decimals - 2));
        }
    }

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                         UTILS                                                      
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    function verifyMerklNonces() public view {
        address EXPECTED_DISTRIBUTOR_IMPLEMENTATION_ADDRESS = 0x918261fa5Dd9C3b1358cA911792E9bDF3c5CCa35;
        address EXPECTED_DISTRIBUTOR_PROXY_ADDRESS = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;
        address EXPECTED_DISTRIBUTION_CREATOR_IMPLEMENTATION_ADDRESS = 0x7Db28175B63f154587BbB1Cae62D39Ea80A23383;
        address EXPECTED_DISTRIBUTION_CREATOR_PROXY_ADDRESS = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

        // deploy DISTRIBUTOR implementation nonce 0
        // deploy DISTRIBUTOR proxy nonce 1
        // initialize DISTRIBUTOR nonce 2
        // deploy DISTRIBUTION_CREATOR implementation nonce 3
        // deploy DISTRIBUTION_CREATOR proxy nonce 4
        // initialize DISTRIBUTION_CREATOR nonce 5
        if (EXPECTED_DISTRIBUTOR_IMPLEMENTATION_ADDRESS != vm.computeCreateAddress(MERKL_DEPLOYER_ADDRESS, 0))
            revert("DISTRIBUTOR_IMPLEMENTATION_ADDRESS_MISMATCH");
        if (EXPECTED_DISTRIBUTOR_PROXY_ADDRESS != vm.computeCreateAddress(MERKL_DEPLOYER_ADDRESS, 1))
            revert("DISTRIBUTOR_PROXY_ADDRESS_MISMATCH");
        if (EXPECTED_DISTRIBUTION_CREATOR_IMPLEMENTATION_ADDRESS != vm.computeCreateAddress(MERKL_DEPLOYER_ADDRESS, 3))
            revert("DISTRIBUTION_CREATOR_IMPLEMENTATION_ADDRESS_MISMATCH");
        if (EXPECTED_DISTRIBUTION_CREATOR_PROXY_ADDRESS != vm.computeCreateAddress(MERKL_DEPLOYER_ADDRESS, 4))
            revert("DISTRIBUTION_CREATOR_PROXY_ADDRESS_MISMATCH");
    }

    function transferInitialFunds() internal {
        console.log("\n=== Transferring initial funds ===");

        // Calculate total recipients including KEEPER, DUMPER, DISPUTER_WHITELIST
        uint256 transferLength = 3 + DISPUTER_WHITELIST.length;

        // Check deployer balance
        if (DEPLOYER_ADDRESS.balance < FUND_AMOUNT * transferLength) {
            revert(
                "DEPLOYER_ADDRESS does not have enough balance to transfer to KEEPER, DUMPER and DISPUTER_WHITELIST, please fund the deployer and check FUND_AMOUNT if needed"
            );
        }

        // Prepare recipient and amount arrays
        address[] memory recipients = new address[](transferLength);
        uint256[] memory amounts = new uint256[](transferLength);

        // Add KEEPER, DUMPER and MERKL_DEPLOYER_ADDRESS
        recipients[0] = KEEPER;
        recipients[1] = DUMPER;
        recipients[2] = MERKL_DEPLOYER_ADDRESS;
        amounts[0] = FUND_AMOUNT;
        amounts[1] = FUND_AMOUNT;
        amounts[2] = FUND_AMOUNT;

        // Add DISPUTER_WHITELIST
        for (uint256 i = 0; i < DISPUTER_WHITELIST.length; i++) {
            recipients[i + 3] = DISPUTER_WHITELIST[i];
            amounts[i + 3] = FUND_AMOUNT;
        }

        console.log("Transferring funds to required addresses:", FUND_AMOUNT);
        console.log("Total amount transferred:", FUND_AMOUNT * transferLength);
        transferNativeTokens(recipients, amounts);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.17;

import { console } from "forge-std/console.sol";

import { BaseScript } from "./utils/Base.s.sol";

import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { ITransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { Distributor } from "../contracts/Distributor.sol";
import { IAccessControlManager } from "../contracts/interfaces/IAccessControlManager.sol";
import { MockToken } from "../contracts/mock/MockToken.sol";

contract toggleOperatorBatch is BaseScript {
    // forge script scripts/toggleOperatorBatch.s.sol --rpc-url arbitrum --sender 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701 --broadcast
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        address distributor = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;

        address operator = 0x0E24b0F342F034446Ec814281AD1a7653cBd85e9;

        // Ethereum
        Distributor(distributor).toggleOperator(0xD5E56dCd97f51130798747edAfAa4A3f27995e54, operator);

        Distributor(distributor).toggleOperator(0x155F6eb8C52991e6c092C659a4c7E28293A74704, operator);
        Distributor(distributor).toggleOperator(0xd71Eb01698F7A6BE19D211De37D55cf2b8bD8528, operator);

        // Katana
        /*
        Distributor(distributor).toggleOperator(0xEA79C91540C7E884e6E0069Ce036E52f7BbB1194, operator);
        Distributor(distributor).toggleOperator(0x37a79Bfb9F645F8Ed0a9ead9c722710D8f47C431, operator);
        Distributor(distributor).toggleOperator(0x543CC24962b540430DD1121E83E8564770Da6810, operator);
        Distributor(distributor).toggleOperator(0x156C729C78076b7cd815D01Ca6967c00c5ac8D9C, operator);
        Distributor(distributor).toggleOperator(0xF7EDe5332c6b4A235be4aA3c019222CFe72e984F, operator);
        Distributor(distributor).toggleOperator(0xC1Ec6d26902949Bf6cbb0c9859dbEAD1E87FB243, operator);
        Distributor(distributor).toggleOperator(0x78EC25FBa1bAf6b7dc097Ebb8115A390A2a4Ee12, operator);
        Distributor(distributor).toggleOperator(0xD46dFDAA7cAA8739B0e3274e2C085dFFc8d4776A, operator);
        Distributor(distributor).toggleOperator(0x58B369aEC52DD904f70122cF72ed311f7AAe3bAc, operator);
        Distributor(distributor).toggleOperator(0x0a1937F0D7f15B9ADee5d96616f269a0C6749C6d, operator);
        */

        // Base
        /*
        Distributor(distributor).toggleOperator(0xEF34B4Dcb851385b8F3a8ff460C34aDAFD160802, operator);
        Distributor(distributor).toggleOperator(0xd5428B889621Eee8060fc105AA0AB0Fa2e344468, operator);
        Distributor(distributor).toggleOperator(0x985CC9c306Bfe075F7c67EC275fb0b80F0b21976, operator);
        Distributor(distributor).toggleOperator(0xB9acb02818BDDD3aC178fa51a0587101E54748B0, operator);
        Distributor(distributor).toggleOperator(0x7bc9E2D216B22611D9805C12E0C682391720752F, operator);
        Distributor(distributor).toggleOperator(0x953370e91B70897A0cECc97B58E99D7946C841dE, operator);
        Distributor(distributor).toggleOperator(0xF115C134c23C7A05FBD489A8bE3116EbF54B0D9f, operator);
        Distributor(distributor).toggleOperator(0xBDD79a7DF622E9d9e19a7d92Bc7ea212FA0D2F3E, operator);
        Distributor(distributor).toggleOperator(0xa72a60e6167E8fC5e523184911475c4B37B835E2, operator);
        */

        vm.stopBroadcast();
    }
}

