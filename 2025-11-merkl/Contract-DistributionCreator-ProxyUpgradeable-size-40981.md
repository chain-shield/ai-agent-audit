
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: BUSL-1.1

pragma solidity ^0.8.17;

import { ReentrancyGuardUpgradeable } from "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

import { UUPSHelper } from "./utils/UUPSHelper.sol";
import { IAccessControlManager } from "./interfaces/IAccessControlManager.sol";
import { Errors } from "./utils/Errors.sol";
import { CampaignParameters } from "./struct/CampaignParameters.sol";
import { DistributionParameters } from "./struct/DistributionParameters.sol";
import { RewardTokenAmounts } from "./struct/RewardTokenAmounts.sol";

/// @title DistributionCreator
/// @author Merkl SAS
/// @notice Manages the creation and administration of reward distribution campaigns through the Merkl system
/// @dev This contract serves as the primary interface for campaign creators and provides helper functions for APIs built on Merkl
/// @dev Deprecated variables are maintained in storage for upgrade compatibility
//solhint-disable
contract DistributionCreator is UUPSHelper, ReentrancyGuardUpgradeable {
    using SafeERC20 for IERC20;

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                 CONSTANTS / VARIABLES                                              
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    /// @notice Duration of one hour in seconds
    uint32 public constant HOUR = 3600;

    /// @notice Base denominator for fee calculations (represents 100%)
    uint256 public constant BASE_9 = 1e9;

    /// @notice Chain ID where this contract is deployed
    uint256 public immutable CHAIN_ID = block.chainid;

    /// @notice `AccessControlManager` contract handling access control
    IAccessControlManager public accessControlManager;

    /// @notice Address of the Distributor contract that distributes rewards to users
    address public distributor;

    /// @notice Address that receives protocol fees from campaign creation
    address public feeRecipient;

    /// @notice Default fee rate (in base 10^9) applied when creating a campaign
    uint256 public defaultFees;

    /// @notice Terms and conditions message that users must acknowledge before creating campaigns
    string public message;

    /// @notice Keccak256 hash of the conditions that users must accept before creating campaigns
    /// @dev The message may be a link to the full terms hosted offchain
    bytes32 public messageHash;

    /// @notice Deprecated - kept for storage layout compatibility
    DistributionParameters[] public distributionList;

    /// @notice Maps an address to its fee rebate percentage
    mapping(address => uint256) public feeRebate;

    /// @notice Deprecated - kept for storage layout compatibility
    mapping(address => uint256) public isWhitelistedToken;

    /// @notice Deprecated - kept for storage layout compatibility
    mapping(address => uint256) public _nonces;

    /// @notice Maps user addresses to the hash of the last terms and conditions they accepted
    /// @dev The name includes 'signature' for legacy reasons, reflecting the original requirement for users to sign conditions
    mapping(address => bytes32) public userSignatures;

    /// @notice Maps user addresses to their whitelist status for signature requirements
    /// @dev The name includes 'signature' for legacy reasons, reflecting the original requirement for users to sign conditions
    mapping(address => uint256) public userSignatureWhitelist;

    /// @notice Maps each reward token to its minimum required amount per epoch for campaign validity
    /// @dev A value of 0 indicates the token is not whitelisted and cannot be used as a reward
    mapping(address => uint256) public rewardTokenMinAmounts;

    /// @notice Array of all reward tokens that have been whitelisted at any point
    address[] public rewardTokens;

    /// @notice Array of all campaigns ever created in the contract (past, current, and future)
    /// @dev This list can grow unbounded, but is only accessed by view functions
    CampaignParameters[] public campaignList;

    /// @notice Maps a campaign ID to its index in the campaign list plus one (0 = does not exist)
    mapping(bytes32 => uint256) internal _campaignLookup;

    /// @notice Maps campaign types to their specific fee rates, overriding the default fee
    mapping(uint32 => uint256) public campaignSpecificFees;

    /// @notice Maps campaign IDs to override parameters that modify the original campaign
    mapping(bytes32 => CampaignParameters) public campaignOverrides;

    /// @notice Maps campaign IDs to timestamps when overrides were applied
    mapping(bytes32 => uint256[]) public campaignOverridesTimestamp;

    /// @notice Maps campaign IDs to reward reallocations (from address -> to address)
    mapping(bytes32 => mapping(address => address)) public campaignReallocation;

    /// @notice Maps campaign IDs to lists of addresses whose rewards have been reallocated
    mapping(bytes32 => address[]) public campaignListReallocation;

    /// @notice Maps creator addresses to their predeposited token balances for each reward token
    mapping(address => mapping(address => uint256)) public creatorBalance;

    /// @notice Maps creator addresses to operator approvals for spending predeposited tokens
    /// @dev creator => operator => rewardToken => allowance amount
    mapping(address => mapping(address => mapping(address => uint256))) public creatorAllowance;

    /// @notice Maps manager addresses to authorized campaign operators who can manage campaigns on their behalf
    mapping(address => mapping(address => uint256)) public campaignOperators;

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                        EVENTS                                                      
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    event CreatorAllowanceUpdated(address indexed user, address indexed operator, address indexed token, uint256 amount);
    event CreatorBalanceUpdated(address indexed user, address indexed token, uint256 amount);
    event DistributorUpdated(address indexed _distributor);
    event FeeRebateUpdated(address indexed user, uint256 userFeeRebate);
    event FeeRecipientUpdated(address indexed _feeRecipient);
    event FeesSet(uint256 _fees);
    event CampaignOperatorToggled(address indexed user, address indexed operator, bool isWhitelisted);
    event CampaignOverride(bytes32 _campaignId, CampaignParameters campaign);
    event CampaignReallocation(bytes32 _campaignId, address[] indexed from, address indexed to);
    event CampaignSpecificFeesSet(uint32 campaignType, uint256 _fees);
    event MessageUpdated(bytes32 _messageHash);
    event NewCampaign(CampaignParameters campaign);
    event RewardTokenMinimumAmountUpdated(address indexed token, uint256 amount);
    event UserSigningWhitelistToggled(address indexed user, uint256 toggleStatus);

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                       MODIFIERS                                                    
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    /// @notice Restricts function access to addresses with governor or guardian role
    modifier onlyGovernorOrGuardian() {
        if (!accessControlManager.isGovernorOrGuardian(msg.sender)) revert Errors.NotGovernorOrGuardian();
        _;
    }

    /// @notice Restricts function access to addresses with governor role only
    modifier onlyGovernor() {
        if (!accessControlManager.isGovernor(msg.sender)) revert Errors.NotGovernor();
        _;
    }

    /// @notice Ensures the caller has accepted the current terms or is whitelisted for this
    /// @dev Checks both msg.sender and tx.origin for signature or whitelist status
    modifier hasSigned() {
        if (
            userSignatureWhitelist[msg.sender] == 0 &&
            userSignatureWhitelist[tx.origin] == 0 &&
            userSignatures[msg.sender] != messageHash &&
            userSignatures[tx.origin] != messageHash
        ) revert Errors.NotSigned();
        _;
    }

    /// @notice Restricts function access to the specified user or any governor
    /// @param user The user address allowed to call the function
    modifier onlyUserOrGovernor(address user) {
        if (user != msg.sender && !accessControlManager.isGovernor(msg.sender)) revert Errors.NotAllowed();
        _;
    }

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                      CONSTRUCTOR                                                   
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    /// @notice Initializes the contract with access control, distributor, and default fees
    /// @param _accessControlManager Address of the access control manager contract
    /// @param _distributor Address of the Distributor contract
    /// @param _fees Default fee rate in base 10^9 (must be less than BASE_9)
    function initialize(IAccessControlManager _accessControlManager, address _distributor, uint256 _fees) external initializer {
        if (address(_accessControlManager) == address(0) || _distributor == address(0)) revert Errors.ZeroAddress();
        if (_fees >= BASE_9) revert Errors.InvalidParam();
        distributor = _distributor;
        accessControlManager = _accessControlManager;
        defaultFees = _fees;
    }

    constructor() initializer {}

    /// @inheritdoc UUPSHelper
    function _authorizeUpgrade(address) internal view override onlyGovernorUpgrader(accessControlManager) {}

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                 USER FACING FUNCTIONS                                              
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    /// @notice Creates a new reward distribution campaign
    /// @param newCampaign Parameters defining the campaign structure and rewards
    /// @return campaignId Unique identifier for the newly created campaign
    /// @dev Campaigns with invalid formatting may not be processed by the reward engine, potentially losing rewards
    /// @dev Reward tokens must be whitelisted and amounts must exceed the token-specific minimum threshold
    /// @dev Reverts if the sender has not accepted the terms and conditions via acceptConditions() or signature
    function createCampaign(CampaignParameters memory newCampaign) external nonReentrant hasSigned returns (bytes32) {
        return _createCampaign(newCampaign);
    }

    /// @notice Creates multiple reward distribution campaigns in a single transaction
    /// @param campaigns Array of campaign parameters to create
    /// @return Array of campaign IDs for all newly created campaigns
    function createCampaigns(CampaignParameters[] memory campaigns) external nonReentrant hasSigned returns (bytes32[] memory) {
        uint256 campaignsLength = campaigns.length;
        bytes32[] memory campaignIds = new bytes32[](campaignsLength);
        for (uint256 i; i < campaignsLength; ) {
            campaignIds[i] = _createCampaign(campaigns[i]);
            unchecked {
                ++i;
            }
        }
        return campaignIds;
    }

    /// @notice Allows a user to accept Merkl's terms and conditions to enable campaign creation
    /// @dev If the conditions change (through setMessage), users must accept again the new terms
    /// @dev If the messageHash is not set, it means that there are no conditions to accept
    function acceptConditions() external {
        userSignatures[msg.sender] = messageHash;
    }

    /// @notice Updates parameters of an existing campaign while preserving core immutable fields
    /// @param _campaignId ID of the campaign to override
    /// @param newCampaign New campaign parameters (some fields will be ignored or validated)
    /// @dev Cannot change rewardToken, amount, or creator address
    /// @dev Can only update startTimestamp if the campaign has not yet started
    /// @dev New end time (startTimestamp + duration) must be in the future
    /// @dev The Merkl engine validates override correctness; invalid overrides are ignored
    /// @dev In the case of an invalid override, the campaign may not be processed and fees may still be taken by the Merkl engine
    function overrideCampaign(bytes32 _campaignId, CampaignParameters memory newCampaign) external {
        CampaignParameters memory _campaign = campaign(_campaignId);
        _isValidOperator(_campaign.creator);
        if (
            newCampaign.rewardToken != _campaign.rewardToken ||
            newCampaign.amount != _campaign.amount ||
            (newCampaign.startTimestamp != _campaign.startTimestamp && block.timestamp > _campaign.startTimestamp) || // Allow to update startTimestamp before campaign start
            // End timestamp should be in the future
            newCampaign.duration + _campaign.startTimestamp <= block.timestamp
        ) revert Errors.InvalidOverride();

        newCampaign.campaignId = _campaignId;
        // The manager address cannot be changed
        newCampaign.creator = _campaign.creator;
        campaignOverrides[_campaignId] = newCampaign;
        campaignOverridesTimestamp[_campaignId].push(block.timestamp);
        emit CampaignOverride(_campaignId, newCampaign);
    }

    /// @notice Reallocates unclaimed rewards from specific addresses to a new recipient after campaign ends
    /// @param _campaignId ID of the completed campaign to reallocate from
    /// @param froms Array of addresses whose unclaimed rewards should be reallocated
    /// @param to Address that will receive the reallocated rewards
    /// @dev Can only be called after the campaign has ended (startTimestamp + duration has passed)
    /// @dev Reallocation validity is determined by the Merkl engine; invalid reallocations are ignored
    function reallocateCampaignRewards(bytes32 _campaignId, address[] memory froms, address to) external {
        CampaignParameters memory _campaign = campaign(_campaignId);
        _isValidOperator(_campaign.creator);
        if (block.timestamp < _campaign.startTimestamp + _campaign.duration) revert Errors.InvalidReallocation();

        uint256 fromsLength = froms.length;
        for (uint256 i; i < fromsLength; ) {
            campaignReallocation[_campaignId][froms[i]] = to;
            campaignListReallocation[_campaignId].push(froms[i]);
            unchecked {
                ++i;
            }
        }
        emit CampaignReallocation(_campaignId, froms, to);
    }

    /// @notice Increases a user's predeposited token balance for campaign funding
    /// @param user Address whose balance will be increased
    /// @param rewardToken Token to deposit
    /// @param amount Amount to deposit
    /// @dev When called by a governor, the user must have sent tokens to the contract beforehand
    /// @dev Can be used to deposit on behalf of another user
    /// @dev WARNING: Do not use with any non strictly standard ERC20 (like rebasing tokens) as they will cause accounting issues
    function increaseTokenBalance(address user, address rewardToken, uint256 amount) external {
        if (!accessControlManager.isGovernor(msg.sender)) IERC20(rewardToken).safeTransferFrom(msg.sender, address(this), amount);
        _updateBalance(user, rewardToken, creatorBalance[user][rewardToken] + amount);
    }

    /// @notice Decreases a user's predeposited token balance and transfers tokens out
    /// @param user Address whose balance will be decreased
    /// @param rewardToken Token to withdraw
    /// @param to Address that will receive the withdrawn tokens
    /// @param amount Amount to withdraw
    /// @dev Only callable by the user themselves or a governor
    function decreaseTokenBalance(address user, address rewardToken, address to, uint256 amount) external onlyUserOrGovernor(user) {
        _updateBalance(user, rewardToken, creatorBalance[user][rewardToken] - amount);
        IERC20(rewardToken).safeTransfer(to, amount);
    }

    /// @notice Increases an operator's allowance to spend a user's predeposited tokens
    /// @param user User granting the allowance
    /// @param operator Operator receiving spending permission
    /// @param rewardToken Token for which allowance is granted
    /// @param amount Amount to increase the allowance by
    /// @dev Only callable by the user themselves or a governor
    function increaseTokenAllowance(address user, address operator, address rewardToken, uint256 amount) external onlyUserOrGovernor(user) {
        _updateAllowance(user, operator, rewardToken, creatorAllowance[user][operator][rewardToken] + amount);
    }

    /// @notice Decreases an operator's allowance to spend a user's predeposited tokens
    /// @param user User reducing the allowance
    /// @param operator Operator whose allowance is being reduced
    /// @param rewardToken Token for which allowance is reduced
    /// @param amount Amount to decrease the allowance by
    /// @dev Only callable by the user themselves or a governor
    function decreaseTokenAllowance(address user, address operator, address rewardToken, uint256 amount) external onlyUserOrGovernor(user) {
        _updateAllowance(user, operator, rewardToken, creatorAllowance[user][operator][rewardToken] - amount);
    }

    /// @notice Toggles an operator's authorization to create and manage campaigns on behalf of a user
    /// @param user User granting or revoking operator access
    /// @param operator Operator whose authorization is being toggled
    /// @dev Only callable by the user themselves or a governor
    /// @dev Toggles between authorized (1) and unauthorized (0)
    function toggleCampaignOperator(address user, address operator) external onlyUserOrGovernor(user) {
        uint256 currentStatus = campaignOperators[user][operator];
        campaignOperators[user][operator] = 1 - currentStatus;
        emit CampaignOperatorToggled(user, operator, currentStatus == 0);
    }

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                        GETTERS                                                     
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    /// @notice Returns the array index of a campaign in the campaign list
    /// @param _campaignId ID of the campaign to look up
    /// @return Zero-based index of the campaign in the campaignList array
    /// @dev Reverts if the campaign does not exist
    function campaignLookup(bytes32 _campaignId) public view returns (uint256) {
        uint256 index = _campaignLookup[_campaignId];
        if (index == 0) revert Errors.CampaignDoesNotExist();
        return index - 1;
    }

    /// @notice Returns the original parameters of a campaign
    /// @param _campaignId ID of the campaign to retrieve
    /// @return Campaign parameters as originally created
    /// @dev Returns original parameters even if the campaign has been overridden
    function campaign(bytes32 _campaignId) public view returns (CampaignParameters memory) {
        return campaignList[campaignLookup(_campaignId)];
    }

    /// @notice Computes the unique campaign ID for a given set of campaign parameters
    /// @param campaignData Campaign parameters to hash
    /// @return Unique campaign ID derived from hashing key parameters
    /// @dev Campaign ID is computed as keccak256 of creator, rewardToken, campaignType, startTimestamp, duration, and campaignData
    function campaignId(CampaignParameters memory campaignData) public view returns (bytes32) {
        return
            bytes32(
                keccak256(
                    abi.encodePacked(
                        CHAIN_ID,
                        campaignData.creator,
                        campaignData.rewardToken,
                        campaignData.campaignType,
                        campaignData.startTimestamp,
                        campaignData.duration,
                        campaignData.campaignData
                    )
                )
            );
    }

    /// @notice Returns all whitelisted reward tokens and their minimum required amounts
    /// @return Array of reward tokens with their minimum amounts per epoch
    /// @dev Not optimized for onchain queries; intended for off-chain/API use
    function getValidRewardTokens() external view returns (RewardTokenAmounts[] memory) {
        (RewardTokenAmounts[] memory validRewardTokens, ) = _getValidRewardTokens(0, type(uint32).max);
        return validRewardTokens;
    }

    /// @notice Returns a paginated list of whitelisted reward tokens
    /// @param skip Number of tokens to skip
    /// @param first Maximum number of tokens to return
    /// @return Array of reward tokens and total count
    /// @dev Not optimized for onchain queries; intended for off-chain/API use
    function getValidRewardTokens(uint32 skip, uint32 first) external view returns (RewardTokenAmounts[] memory, uint256) {
        return _getValidRewardTokens(skip, first);
    }

    /// @notice Returns all timestamps when a campaign was overridden
    /// @param _campaignId ID of the campaign
    /// @return Array of block timestamps when overrides occurred
    function getCampaignOverridesTimestamp(bytes32 _campaignId) external view returns (uint256[] memory) {
        return campaignOverridesTimestamp[_campaignId];
    }

    /// @notice Returns all addresses from which rewards were reallocated for a campaign
    /// @param _campaignId ID of the campaign
    /// @return Array of addresses that had rewards reallocated away from them
    function getCampaignListReallocation(bytes32 _campaignId) external view returns (address[] memory) {
        return campaignListReallocation[_campaignId];
    }

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                 GOVERNANCE FUNCTIONS                                               
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    /// @notice Updates the Distributor contract address that receives and distributes rewards
    /// @param _distributor New Distributor contract address
    /// @dev Only callable by governor
    function setNewDistributor(address _distributor) external onlyGovernor {
        if (_distributor == address(0)) revert Errors.InvalidParam();
        distributor = _distributor;
        emit DistributorUpdated(_distributor);
    }

    /// @notice Withdraws accumulated protocol fees to a specified address
    /// @param tokens Array of token addresses to withdraw fees from
    /// @param to Address that will receive the withdrawn fees
    /// @dev Only callable by governor
    /// @dev Transfers the entire balance of each token held by the contract
    function recoverFees(IERC20[] calldata tokens, address to) external onlyGovernor {
        uint256 tokensLength = tokens.length;
        for (uint256 i; i < tokensLength; ) {
            tokens[i].safeTransfer(to, tokens[i].balanceOf(address(this)));
            unchecked {
                ++i;
            }
        }
    }

    /// @notice Updates the address that receives protocol fees from campaign creation
    /// @param _feeRecipient New fee recipient address
    /// @dev Only callable by governor
    function setFeeRecipient(address _feeRecipient) external onlyGovernor {
        feeRecipient = _feeRecipient;
        emit FeeRecipientUpdated(_feeRecipient);
    }

    /// @notice Updates the terms and conditions that users must accept before creating campaigns
    /// @param _message New terms and conditions message text
    /// @dev Only callable by governor or guardian
    /// @dev Automatically computes and stores the keccak256 hash
    /// @dev The message may be a link to the full terms hosted offchain
    function setMessage(string memory _message) external onlyGovernorOrGuardian {
        message = _message;
        bytes32 _messageHash = ECDSA.toEthSignedMessageHash(bytes(_message));
        messageHash = _messageHash;
        emit MessageUpdated(_messageHash);
    }

    /// @notice Updates the default fee rate applied to campaign creation
    /// @param _defaultFees New default fee rate in base 10^9
    /// @dev Only callable by governor or guardian
    /// @dev Fee rate must be less than BASE_9 (100%)
    function setFees(uint256 _defaultFees) external onlyGovernorOrGuardian {
        if (_defaultFees >= BASE_9) revert Errors.InvalidParam();
        defaultFees = _defaultFees;
        emit FeesSet(_defaultFees);
    }

    /// @notice Sets campaign-type-specific fee rates that override the default fee
    /// @param campaignType Type identifier for the campaign
    /// @param _fees Fee rate for this campaign type in base 10^9
    /// @dev Only callable by governor or guardian
    /// @dev Set fee to 1 to effectively waive fees for a campaign type
    /// @dev Fee rate must be less than BASE_9 (100%)
    function setCampaignFees(uint32 campaignType, uint256 _fees) external onlyGovernorOrGuardian {
        if (_fees >= BASE_9) revert Errors.InvalidParam();
        campaignSpecificFees[campaignType] = _fees;
        emit CampaignSpecificFeesSet(campaignType, _fees);
    }

    /// @notice Sets a fee rebate for a specific user
    /// @param user User address receiving the fee rebate
    /// @param userFeeRebate Rebate amount in base 10^9
    /// @dev Only callable by governor or guardian
    function setUserFeeRebate(address user, uint256 userFeeRebate) external onlyGovernorOrGuardian {
        feeRebate[user] = userFeeRebate;
        emit FeeRebateUpdated(user, userFeeRebate);
    }

    /// @notice Toggles whether a user must sign the terms message before creating campaigns
    /// @param user User address whose whitelist status is being toggled
    /// @dev Only callable by governor or guardian
    /// @dev Whitelisted users (status = 1) can create campaigns without accepting Merkl terms
    function toggleSigningWhitelist(address user) external onlyGovernorOrGuardian {
        uint256 whitelistStatus = 1 - userSignatureWhitelist[user];
        userSignatureWhitelist[user] = whitelistStatus;
        emit UserSigningWhitelistToggled(user, whitelistStatus);
    }

    /// @notice Configures minimum reward amounts per epoch for whitelisted tokens
    /// @param tokens Array of reward token addresses
    /// @param amounts Array of minimum amounts (0 = remove from whitelist, >0 = add/update)
    /// @dev Only callable by governor or guardian
    /// @dev Setting amount to 0 effectively removes the token from the whitelist
    /// @dev Prevents duplicate entries when adding previously removed tokens
    function setRewardTokenMinAmounts(address[] calldata tokens, uint256[] calldata amounts) external onlyGovernorOrGuardian {
        uint256 tokensLength = tokens.length;
        if (tokensLength != amounts.length) revert Errors.InvalidLengths();
        for (uint256 i; i < tokensLength; ) {
            uint256 amount = amounts[i];
            // Basic logic check to make sure there are no duplicates in the `rewardTokens` table. If a token is
            // removed then re-added, it will appear as a duplicate in the list
            if (amount != 0 && rewardTokenMinAmounts[tokens[i]] == 0) rewardTokens.push(tokens[i]);
            rewardTokenMinAmounts[tokens[i]] = amount;
            emit RewardTokenMinimumAmountUpdated(tokens[i], amount);
            unchecked {
                ++i;
            }
        }
    }

    /*//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                                                       INTERNAL                                                     
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////*/

    /// @notice Internal function to create a new campaign with validation and fee processing
    /// @param newCampaign Campaign parameters to create
    /// @return Unique campaign ID of the created campaign
    /// @dev Validates campaign duration, reward token whitelist status, and minimum reward amounts
    /// @dev Computes and deducts protocol fees from the campaign amount
    /// @dev Reverts if campaign already exists or validation fails
    function _createCampaign(CampaignParameters memory newCampaign) internal returns (bytes32) {
        uint256 rewardTokenMinAmount = rewardTokenMinAmounts[newCampaign.rewardToken];
        // if the campaign doesn't last at least one hour
        if (newCampaign.duration < HOUR) revert Errors.CampaignDurationBelowHour();
        // if the reward token is not whitelisted as an incentive token
        if (rewardTokenMinAmount == 0) revert Errors.CampaignRewardTokenNotWhitelisted();
        // if the amount distributed is too small with respect to what is allowed
        if ((newCampaign.amount * HOUR) / newCampaign.duration < rewardTokenMinAmount) revert Errors.CampaignRewardTooLow();
        // Computing fees and pulling tokens
        uint256 campaignAmountMinusFees = _computeFees(newCampaign.campaignType, newCampaign.amount);
        if (newCampaign.creator == address(0)) newCampaign.creator = msg.sender;
        _pullTokens(newCampaign.creator, newCampaign.rewardToken, newCampaign.amount, campaignAmountMinusFees);
        newCampaign.amount = campaignAmountMinusFees;
        newCampaign.campaignId = campaignId(newCampaign);

        if (_campaignLookup[newCampaign.campaignId] != 0) revert Errors.CampaignAlreadyExists();
        _campaignLookup[newCampaign.campaignId] = campaignList.length + 1;
        campaignList.push(newCampaign);
        emit NewCampaign(newCampaign);

        return newCampaign.campaignId;
    }

    /// @notice Validates that the caller is authorized to manage campaigns for the specified manager
    /// @param manager Address of the campaign manager
    /// @dev Reverts if msg.sender is not the manager and not an authorized operator
    function _isValidOperator(address manager) internal view {
        if (manager != msg.sender && campaignOperators[manager][msg.sender] == 0) {
            revert Errors.OperatorNotAllowed();
        }
    }

    /// @notice Updates an operator's allowance to spend a user's predeposited tokens
    /// @param user User granting the allowance
    /// @param operator Operator receiving the allowance
    /// @param rewardToken Token for which allowance is being set
    /// @param newAllowance New allowance amount
    function _updateAllowance(address user, address operator, address rewardToken, uint256 newAllowance) internal {
        creatorAllowance[user][operator][rewardToken] = newAllowance;
        emit CreatorAllowanceUpdated(user, operator, rewardToken, newAllowance);
    }

    /// @notice Updates a user's predeposited token balance
    /// @param user User whose balance is being updated
    /// @param rewardToken Token whose balance is being updated
    /// @param newBalance New balance amount
    function _updateBalance(address user, address rewardToken, uint256 newBalance) internal {
        creatorBalance[user][rewardToken] = newBalance;
        emit CreatorBalanceUpdated(user, rewardToken, newBalance);
    }

    /// @notice Transfers reward tokens from creator's balance or msg.sender to the distributor
    /// @param creator Address of the campaign creator
    /// @param rewardToken Token being transferred
    /// @param campaignAmount Total amount including fees
    /// @param campaignAmountMinusFees Net amount after fees to send to distributor
    /// @dev Attempts to use predeposited balance first, checking operator allowance if applicable
    /// @dev Falls back to direct transfer from msg.sender if insufficient predeposited balance
    /// @dev Sends fees to feeRecipient (or this contract if feeRecipient is zero address)
    function _pullTokens(address creator, address rewardToken, uint256 campaignAmount, uint256 campaignAmountMinusFees) internal {
        uint256 fees = campaignAmount - campaignAmountMinusFees;
        address _feeRecipient;
        if (fees > 0) {
            _feeRecipient = feeRecipient;
            _feeRecipient = _feeRecipient == address(0) ? address(this) : _feeRecipient;
        }
        uint256 userBalance = creatorBalance[creator][rewardToken];
        if (userBalance >= campaignAmount) {
            if (msg.sender != creator) {
                uint256 senderAllowance = creatorAllowance[creator][msg.sender][rewardToken];
                if (senderAllowance >= campaignAmount) {
                    _updateAllowance(creator, msg.sender, rewardToken, senderAllowance - campaignAmount);
                } else {
                    if (fees > 0) IERC20(rewardToken).safeTransferFrom(msg.sender, _feeRecipient, fees);
                    IERC20(rewardToken).safeTransferFrom(msg.sender, distributor, campaignAmountMinusFees);
                    return;
                }
            }
            _updateBalance(creator, rewardToken, userBalance - campaignAmount);
            if (fees > 0 && _feeRecipient != address(this)) IERC20(rewardToken).safeTransfer(_feeRecipient, fees);
            IERC20(rewardToken).safeTransfer(distributor, campaignAmountMinusFees);
        } else {
            if (fees > 0) IERC20(rewardToken).safeTransferFrom(msg.sender, _feeRecipient, fees);
            IERC20(rewardToken).safeTransferFrom(msg.sender, distributor, campaignAmountMinusFees);
        }
    }

    /// @notice Calculates the net campaign amount after deducting applicable fees
    /// @param campaignType Type of campaign for fee calculation
    /// @param distributionAmount Gross distribution amount before fees
    /// @return distributionAmountMinusFees Net amount after fees are deducted
    /// @dev Uses campaign-specific fees if set, otherwise uses default fees
    /// @dev Campaign-specific fee of 1 is treated as 0 (fee waiver)
    /// @dev Applies fee rebates to msg.sender (not creator)
    function _computeFees(uint32 campaignType, uint256 distributionAmount) internal view returns (uint256 distributionAmountMinusFees) {
        uint256 baseFeesValue = campaignSpecificFees[campaignType];
        if (baseFeesValue == 1) baseFeesValue = 0;
        else if (baseFeesValue == 0) baseFeesValue = defaultFees;
        // Fee rebates are applied to the msg.sender and not to the creator of the campaign
        uint256 _fees = (baseFeesValue * (BASE_9 - feeRebate[msg.sender])) / BASE_9;
        distributionAmountMinusFees = distributionAmount;
        if (_fees != 0) {
            distributionAmountMinusFees = (distributionAmount * (BASE_9 - _fees)) / BASE_9;
        }
    }

    /// @notice Builds a paginated list of whitelisted reward tokens with their minimum amounts
    /// @param skip Number of tokens to skip in the iteration
    /// @param first Maximum number of tokens to return
    /// @return Array of valid reward tokens and the index where iteration stopped
    /// @dev Only includes tokens with non-zero minimum amounts (active whitelist entries)
    /// @dev Uses assembly to resize the return array to actual length
    function _getValidRewardTokens(uint32 skip, uint32 first) internal view returns (RewardTokenAmounts[] memory, uint256) {
        uint256 length;
        uint256 rewardTokenListLength = rewardTokens.length;
        uint256 returnSize = first > rewardTokenListLength ? rewardTokenListLength : first;
        RewardTokenAmounts[] memory validRewardTokens = new RewardTokenAmounts[](returnSize);
        uint32 i = skip;
        while (i < rewardTokenListLength) {
            address token = rewardTokens[i];
            uint256 minAmount = rewardTokenMinAmounts[token];
            if (minAmount > 0) {
                validRewardTokens[length] = RewardTokenAmounts(token, minAmount);
                length += 1;
            }
            unchecked {
                ++i;
            }
            if (length == returnSize) break;
        }
        assembly {
            mstore(validRewardTokens, length)
        }
        return (validRewardTokens, i);
    }

    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[28] private __gap;
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

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v4.9.0) (security/ReentrancyGuard.sol)

pragma solidity ^0.8.0;
import "../proxy/utils/Initializable.sol";

/**
 * @dev Contract module that helps prevent reentrant calls to a function.
 *
 * Inheriting from `ReentrancyGuard` will make the {nonReentrant} modifier
 * available, which can be applied to functions to make sure there are no nested
 * (reentrant) calls to them.
 *
 * Note that because there is a single `nonReentrant` guard, functions marked as
 * `nonReentrant` may not call one another. This can be worked around by making
 * those functions `private`, and then adding `external` `nonReentrant` entry
 * points to them.
 *
 * TIP: If you would like to learn more about reentrancy and alternative ways
 * to protect against it, check out our blog post
 * https://blog.openzeppelin.com/reentrancy-after-istanbul/[Reentrancy After Istanbul].
 */
abstract contract ReentrancyGuardUpgradeable is Initializable {
    // Booleans are more expensive than uint256 or any type that takes up a full
    // word because each write operation emits an extra SLOAD to first read the
    // slot's contents, replace the bits taken up by the boolean, and then write
    // back. This is the compiler's defense against contract upgrades and
    // pointer aliasing, and it cannot be disabled.

    // The values being non-zero value makes deployment a bit more expensive,
    // but in exchange the refund on every call to nonReentrant will be lower in
    // amount. Since refunds are capped to a percentage of the total
    // transaction's gas, it is best to keep them low in cases like this one, to
    // increase the likelihood of the full refund coming into effect.
    uint256 private constant _NOT_ENTERED = 1;
    uint256 private constant _ENTERED = 2;

    uint256 private _status;

    function __ReentrancyGuard_init() internal onlyInitializing {
        __ReentrancyGuard_init_unchained();
    }

    function __ReentrancyGuard_init_unchained() internal onlyInitializing {
        _status = _NOT_ENTERED;
    }

    /**
     * @dev Prevents a contract from calling itself, directly or indirectly.
     * Calling a `nonReentrant` function from another `nonReentrant`
     * function is not supported. It is possible to prevent this from happening
     * by making the `nonReentrant` function external, and making it call a
     * `private` function that does the actual work.
     */
    modifier nonReentrant() {
        _nonReentrantBefore();
        _;
        _nonReentrantAfter();
    }

    function _nonReentrantBefore() private {
        // On the first call to nonReentrant, _status will be _NOT_ENTERED
        require(_status != _ENTERED, "ReentrancyGuard: reentrant call");

        // Any calls to nonReentrant after this point will fail
        _status = _ENTERED;
    }

    function _nonReentrantAfter() private {
        // By storing the original value once again, a refund is triggered (see
        // https://eips.ethereum.org/EIPS/eip-2200)
        _status = _NOT_ENTERED;
    }

    /**
     * @dev Returns true if the reentrancy guard is currently set to "entered", which indicates there is a
     * `nonReentrant` function in the call stack.
     */
    function _reentrancyGuardEntered() internal view returns (bool) {
        return _status == _ENTERED;
    }

    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[49] private __gap;
}

// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.0;

struct DistributionParameters {
    // ID of the reward (populated once created). This can be left as a null bytes32 when creating distributions
    // on Merkl.
    bytes32 rewardId;
    // Address of the UniswapV3 pool that needs to be incentivized
    address uniV3Pool;
    // Address of the reward token for the incentives
    address rewardToken;
    // Amount of `rewardToken` to distribute across all the epochs
    // Amount distributed per epoch is `amount/numEpoch`
    uint256 amount;
    // List of all position wrappers to consider or not for this contract. Some wrappers like Gamma or Arrakis
    // are automatically detected and so there is no need to specify them here. Check out the docs to find out
    // which need to be specified and which are not automatically detected.
    address[] positionWrappers;
    // Type (blacklist==3, whitelist==0, ...) encoded as a `uint32` for each wrapper in the list above. Mapping between
    // wrapper types and their corresponding `uint32` value can be found in Merkl Docs
    uint32[] wrapperTypes;
    // In the incentivization formula, how much of the fees should go to holders of token0
    // in base 10**4
    uint32 propToken0;
    // Proportion for holding token1 (in base 10**4)
    uint32 propToken1;
    // Proportion for providing a useful liquidity (in base 10**4) that generates fees
    uint32 propFees;
    // Timestamp at which the incentivization should start. This is in the same units as `block.timestamp`.
    uint32 epochStart;
    // Amount of epochs for which incentivization should last. Epochs are expressed in hours here, so for a
    // campaign of 1 week `numEpoch` should for instance be 168.
    uint32 numEpoch;
    // Whether out of range liquidity should still be incentivized or not
    // This should be equal to 1 if out of range liquidity should still be incentivized
    // and 0 otherwise.
    uint32 isOutOfRangeIncentivized;
    // How much more addresses with a maximum boost can get with respect to addresses
    // which do not have a boost (in base 4). In the case of Curve where addresses get 2.5x more
    // this would be 25000.
    uint32 boostedReward;
    // Address of the token which dictates who gets boosted rewards or not. This is optional
    // and if the zero address is given no boost will be taken into account. In the case of Curve, this address
    // would for instance be the veBoostProxy address, or in other cases the veToken address.
    address boostingAddress;
    // Additional data passed when distributing rewards. This parameter may be used in case
    // the reward distribution script needs to look into other parameters beyond the ones above.
    // In most cases, when creating a campaign on Merkl, you can leave this as an empty bytes.
    bytes additionalData;
}

// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.0;

struct RewardTokenAmounts {
    address token;
    uint256 minimumAmountPerEpoch;
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

// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.0;

/// @notice Parameters defining a Merkl reward distribution campaign
struct CampaignParameters {
    // ========== POPULATED BY CONTRACT ==========

    /// @notice Unique identifier for the campaign
    /// @dev Can be left as bytes32(0) when creating a new campaign - will be computed by the contract
    bytes32 campaignId;
    // ========== CONFIGURED BY CREATOR ==========

    /// @notice Address of the campaign creator
    /// @dev If set to address(0), will be automatically set to msg.sender when the campaign is created
    address creator;
    /// @notice Token distributed as rewards to campaign participants
    address rewardToken;
    /// @notice Total amount of rewardToken to distribute over the entire campaign duration
    /// @dev Must meet the minimum amount requirement for the reward token
    uint256 amount;
    /// @notice Type identifier for the campaign structure and rules
    /// @dev Different types may have different campaignData encoding schemes
    uint32 campaignType;
    /// @notice Unix timestamp when reward distribution begins
    uint32 startTimestamp;
    /// @notice Total duration of the campaign in seconds
    /// @dev Must be a multiple of EPOCH_DURATION (3600 seconds / 1 hour)
    /// @dev Must be at least EPOCH_DURATION (1 hour minimum)
    uint32 duration;
    /// @notice Encoded campaign-specific parameters
    /// @dev Encoding structure depends on campaignType
    /// @dev May include pool addresses, reward distribution rules, whitelists, etc.
    bytes campaignData;
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
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


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.17;

import { console } from "forge-std/console.sol";

import { BaseScript } from "./utils/Base.s.sol";

import { console } from "forge-std/console.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { ITransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { SonicFragment } from "../contracts/partners/tokenWrappers/SonicFragment.sol";
import { DistributionCreator } from "../contracts/DistributionCreator.sol";
import { IAccessControlManager } from "../contracts/interfaces/IAccessControlManager.sol";
import { MockToken } from "../contracts/mock/MockToken.sol";

// forge script scripts/deploySonicFragment.s.sol:DeploySonicFragment --rpc-url sonic --sender 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701 --verify -vvvv --broadcast -i 1
contract DeploySonicFragment is BaseScript {
    function run() public broadcast {
        console.log("DEPLOYER_ADDRESS:", broadcaster);

        // Sonic address - to check
        IAccessControlManager manager = IAccessControlManager(0xa25c30044142d2fA243E7Fd3a6a9713117b3c396);
        address recipient = address(broadcaster);
        // TODO this is the wrapped Sonic address
        address sToken = address(0x039e2fB66102314Ce7b64Ce5Ce3E5183bc94aD38);
        uint256 totalSupply = 10_000_000 ether;
        string memory name = "TEST GEM";
        string memory symbol = "testGEM-S1";

        // Deploy implementation
        SonicFragment implementation = new SonicFragment(address(manager), recipient, sToken, totalSupply, name, symbol);
        console.log("SonicFragment deployed at:", address(implementation));
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.17;

import { console } from "forge-std/console.sol";
import { BaseScript } from "./utils/Base.s.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { ReferralRegistry } from "../contracts/ReferralRegistry.sol";
import { DistributionCreator } from "../contracts/DistributionCreator.sol";
import { IAccessControlManager } from "../contracts/interfaces/IAccessControlManager.sol";

contract DeployReferralRegistry is BaseScript {
    // forge script scripts/deployReferralRegistry.s.sol:DeployReferralRegistry --rpc-url avalanche --broadcast --verify -vvvv
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        uint256 feeSetup = 0;
        // uint32 cliffDuration = 1 weeks;
        DistributionCreator distributionCreator = DistributionCreator(0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd);
        address feeRecipient = distributionCreator.feeRecipient();
        IAccessControlManager accessControlManager = distributionCreator.accessControlManager();

        // Deploy implementation
        /*
        address implementation = address(new ReferralRegistry());
        console.log("ReferralRegistry Implementation:", implementation);

        // Deploy proxy
        ERC1967Proxy proxy = new ERC1967Proxy(implementation, "");
        console.log("ReferralRegistry Proxy:", address(proxy));

        // Initialize
        ReferralRegistry(payable(address(proxy))).initialize(accessControlManager, feeSetup, feeRecipient);

*/
        string memory key = "avant-referral";

        ReferralRegistry(payable(0x3FB2121208b40c7878089A78cc58f9b4D9D8b9F4)).addReferralKey(
            key,
            0,
            false,
            0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701,
            false,
            address(0)
        );

        vm.stopBroadcast();
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
import { PointToken } from "../contracts/partners/tokenWrappers/PointToken.sol";
import { DistributionCreator } from "../contracts/DistributionCreator.sol";
import { IAccessControlManager } from "../contracts/interfaces/IAccessControlManager.sol";

// Base contract with shared constants and utilities
contract PointTokenScript is BaseScript {
    // Common constants and utilities for PointToken scripts
}

// Deploy script
contract DeployPointToken is PointTokenScript {
    function run() external broadcast {
        // forge script scripts/PointToken.s.sol:DeployPointToken --rpc-url arbitrum --broadcast --verify -vvvv
        uint256 chainId = block.chainid;
        // MODIFY THESE VALUES TO SET YOUR DESIRED TOKEN PARAMETERS
        string memory name = "Stable Tracking";
        string memory symbol = "stbl-tracking";
        address minter = 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701;
        uint256 amount = 10_000_000_000 * 1e18;
        address creator = 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701;
        uint8 decimals = 18;

        address accessControlManager = address(DistributionCreator(0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd).accessControlManager());
        _run(name, symbol, minter, accessControlManager, amount, creator);
    }

    function _run(
        string memory name,
        string memory symbol,
        address minter,
        address accessControlManager,
        uint256 amount,
        address creator
    ) internal {
        console.log("DEPLOYER_ADDRESS:", broadcaster);

        // Deploy PointToken
        PointToken token = new PointToken(name, symbol, minter, accessControlManager);

        // Load the point token contract
        // PointToken token = PointToken(0xf9e03FfE6d23D37199CC4B29Dbe0224d8735d02C);
        console.log("Point token deployed at:", address(token));
        console.log("Name:", name);
        console.log("Symbol:", symbol);
        console.log("Decimals:", token.decimals());

        // Mint initial supply to deployer
        token.mint(minter, amount);

        // Whitelist the minter
        token.toggleWhitelistedRecipient(minter);
        console.log("Initial supply minted to deployer");

        // Whitelist the Merkl Contracts
        token.toggleWhitelistedRecipient(0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae);
        token.toggleWhitelistedRecipient(0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd);
        token.toggleWhitelistedRecipient(0xeaC6A75e19beB1283352d24c0311De865a867DAB);
        token.toggleWhitelistedRecipient(0x1A2039792b43C150d3bE02135978A5c3f4d874F4);
        token.transfer(0x1A2039792b43C150d3bE02135978A5c3f4d874F4, 1e10 * 1e18);

        console.log("Whitelisted recipients:");
        // transfer to the SAFE
        if (creator != minter) {
            console.log("Transferring initial supply to KAT SAFE:", creator);
            token.transfer(creator, amount);
        }

        console.log("Transferred initial supply to KAT SAFE");
    }
}

contract MintMorePointToken is PointTokenScript {
    // forge script scripts/PointToken.s.sol:MintMorePointToken --rpc-url hyperevm --broadcast --verify -vvvv
    function run() external broadcast {
        uint256 chainId = block.chainid;
        address recipient = 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701;
        address pointToken = 0x076C42Fe8E13253133738cC8674d85135137270D;
        _run(recipient, pointToken);
    }

    function _run(address recipient, address pointToken) internal {
        console.log("DEPLOYER_ADDRESS:", broadcaster);

        // Deploy PointToken
        PointToken token = PointToken(pointToken);

        // Mint initial supply to deployer
        token.mint(recipient, 1e12 * 1e18);
        console.log("Initial supply minted to deployer");
    }
}

contract WhitelistRecipient is PointTokenScript {
    // forge script scripts/PointToken.s.sol:WhitelistRecipient --rpc-url hyperevm --broadcast --verify -vvvv
    function run() external broadcast {
        uint256 chainId = block.chainid;
        // MODIFY THESE VALUES TO SET YOUR DESIRED TOKEN PARAMETERS
        address recipient = 0xABb29f9CCd2dD058A2DA6b6022f82F90Ae0CEc90;
        address pointToken = 0x49c7B39A2E01869d39548F232F9B1586DA8Ef9c2;
        _run(recipient, pointToken);
    }

    function _run(address recipient, address pointToken) internal {
        console.log("DEPLOYER_ADDRESS:", broadcaster);

        // Deploy PointToken
        PointToken token = PointToken(pointToken);

        // Mint initial supply to deployer
        token.toggleWhitelistedRecipient(recipient);
        console.log("Initial supply minted to deployer");
    }
}

// ToggleMinter script
contract ToggleMinter is PointTokenScript {
    function run(address minter, address pointTokenAddress) external {
        _run(minter, pointTokenAddress);
    }

    function run() external {
        // MODIFY THIS VALUE TO SET THE MINTER ADDRESS
        address minter = address(0);
        address pointTokenAddress = address(0);
        _run(minter, pointTokenAddress);
    }

    function _run(address _minter, address _pointTokenAddress) internal broadcast {
        uint256 chainId = block.chainid;

        PointToken(_pointTokenAddress).toggleMinter(_minter);

        console.log("Toggled minter status for:", _minter);
    }
}

// ToggleAllowedTransfers script
contract ToggleAllowedTransfers is PointTokenScript {
    function run(address pointTokenAddress) external {
        _run(pointTokenAddress);
    }

    function run() external {
        uint256 chainId = block.chainid;
        address pointTokenAddress = address(0);
        _run(pointTokenAddress);
    }

    function _run(address _pointTokenAddress) internal broadcast {
        PointToken(_pointTokenAddress).toggleAllowedTransfers();

        console.log("Toggled allowed transfers status");
    }
}

// ToggleWhitelistedRecipient script
contract ToggleWhitelistedRecipient is PointTokenScript {
    function run(address recipient, address pointTokenAddress) external {
        _run(recipient, pointTokenAddress);
    }

    function run() external {
        // MODIFY THIS VALUE TO SET THE RECIPIENT ADDRESS
        address recipient = address(0);
        address pointTokenAddress = address(0);
        _run(recipient, pointTokenAddress);
    }

    function _run(address _recipient, address _pointTokenAddress) internal broadcast {
        uint256 chainId = block.chainid;

        PointToken(_pointTokenAddress).toggleWhitelistedRecipient(_recipient);

        console.log("Toggled whitelisted recipient status for:", _recipient);
    }
}

// Mint script
contract Mint is PointTokenScript {
    function run(address account, uint256 amount, address pointTokenAddress) external {
        _run(account, amount, pointTokenAddress);
    }

    function run() external {
        // MODIFY THESE VALUES TO SET THE RECIPIENT AND AMOUNT
        address account = address(0);
        uint256 amount = 1000 * 10 ** 18; // 1000 tokens with 18 decimals
        address pointTokenAddress = address(0);
        _run(account, amount, pointTokenAddress);
    }

    function _run(address _account, uint256 _amount, address _pointTokenAddress) internal broadcast {
        uint256 chainId = block.chainid;

        PointToken(_pointTokenAddress).mint(_account, _amount);

        console.log("Minted %s tokens to %s", _amount, _account);
    }
}

// MintBatch script
contract MintBatch is PointTokenScript {
    function run(address[] calldata accounts, uint256[] calldata amounts, address pointTokenAddress) external {
        _run(accounts, amounts, pointTokenAddress);
    }

    function run() external {
        // MODIFY THESE VALUES TO SET THE RECIPIENTS AND AMOUNTS
        address[] memory accounts = new address[](2);
        accounts[0] = address(0x1);
        accounts[1] = address(0x2);

        uint256[] memory amounts = new uint256[](2);
        amounts[0] = 1000 * 10 ** 18; // 1000 tokens
        amounts[1] = 2000 * 10 ** 18; // 2000 tokens

        address pointTokenAddress = address(0);

        _run(accounts, amounts, pointTokenAddress);
    }

    function _run(address[] memory _accounts, uint256[] memory _amounts, address _pointTokenAddress) internal broadcast {
        require(_accounts.length == _amounts.length, "Arrays length mismatch");

        uint256 chainId = block.chainid;

        PointToken(_pointTokenAddress).mintBatch(_accounts, _amounts);

        console.log("Minted tokens in batch to %s accounts", _accounts.length);
    }
}

// Burn script
contract Burn is PointTokenScript {
    function run(address account, uint256 amount, address pointTokenAddress) external {
        _run(account, amount, pointTokenAddress);
    }

    function run() external {
        // MODIFY THESE VALUES TO SET THE ACCOUNT AND AMOUNT
        address account = address(0);
        uint256 amount = 1000 * 10 ** 18; // 1000 tokens with 18 decimals
        address pointTokenAddress = address(0);
        _run(account, amount, pointTokenAddress);
    }

    function _run(address _account, uint256 _amount, address _pointTokenAddress) internal broadcast {
        uint256 chainId = block.chainid;

        PointToken(_pointTokenAddress).burn(_account, _amount);

        console.log("Burned %s tokens from %s", _amount, _account);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.17;

import { console } from "forge-std/console.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { ITransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { IERC20Metadata } from "@openzeppelin/contracts/interfaces/IERC20Metadata.sol";

import { BaseScript } from "./utils/Base.s.sol";
import { DistributionCreator } from "../contracts/DistributionCreator.sol";
import { IAccessControlManager } from "../contracts/interfaces/IAccessControlManager.sol";
import { CampaignParameters } from "../contracts/struct/CampaignParameters.sol";
import { MockToken } from "../contracts/mock/MockToken.sol";

// Base contract with shared utilities
contract DistributionCreatorScript is BaseScript {
    struct CampaignInput {
        address creator;
        address rewardToken;
        uint256 amount;
        uint32 campaignType;
        uint32 startTimestamp;
        uint32 duration;
        bytes campaignData;
    }
}

// Deploy script
contract Deploy is DistributionCreatorScript {
    function run() external broadcast {
        uint256 chainId = block.chainid;
        console.log("DEPLOYER_ADDRESS:", broadcaster);

        // Read configuration from JSON
        address accessControlManager = address(0);
        address distributor = 0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae;
        uint256 defaultFees = 0.03 gwei; // 0.03 gwei

        // Deploy implementation
        DistributionCreator implementation = new DistributionCreator();
        console.log("DistributionCreator Implementation:", address(implementation));

        // Deploy proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), "");
        console.log("DistributionCreator Proxy:", address(proxy));

        // Initialize
        DistributionCreator(address(proxy)).initialize(IAccessControlManager(accessControlManager), distributor, defaultFees);
    }
}

contract DeployImplementation is DistributionCreatorScript {
    function run() external broadcast {
        uint256 chainId = block.chainid;
        console.log("DEPLOYER_ADDRESS:", broadcaster);

        // Deploy implementation
        DistributionCreator implementation = new DistributionCreator();
        console.log("DistributionCreator Implementation:", address(implementation));
    }
}

// SetNewDistributor script
contract SetNewDistributor is DistributionCreatorScript {
    function run() external {
        // MODIFY THIS VALUE TO SET YOUR DESIRED DISTRIBUTOR ADDRESS
        address distributor = address(0);
        _run(distributor);
    }

    function run(address distributor) external {
        _run(distributor);
    }

    function _run(address _distributor) internal broadcast {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

        DistributionCreator(creatorAddress).setNewDistributor(_distributor);

        console.log("New distributor set to:", _distributor);
    }
}

// SetFees script
contract SetFees is DistributionCreatorScript {
    function run() external {
        // MODIFY THIS VALUE TO SET YOUR DESIRED FEES
        uint256 fees = 0;
        _run(fees);
    }

    function run(uint256 fees) external {
        _run(fees);
    }

    function _run(uint256 _fees) internal broadcast {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

        DistributionCreator(creatorAddress).setFees(_fees);

        console.log("Default fees updated to:", _fees);
    }
}

// SetCampaignFees script
contract SetCampaignFees is DistributionCreatorScript {
    function run() external {
        // MODIFY THESE VALUES TO SET YOUR DESIRED CAMPAIGN TYPE AND FEES
        uint32 campaignType = 0;
        uint256 fees = 0;
        _run(campaignType, fees);
    }

    function run(uint32 campaignType, uint256 fees) external {
        _run(campaignType, fees);
    }

    function _run(uint32 _campaignType, uint256 _fees) internal broadcast {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

        DistributionCreator(creatorAddress).setCampaignFees(_campaignType, _fees);

        console.log("Campaign fees updated for type %s to: %s", _campaignType, _fees);
    }
}

// RecoverFees script
contract RecoverFees is DistributionCreatorScript {
    function run() external {
        // MODIFY THESE VALUES TO SET YOUR DESIRED TOKENS AND RECIPIENT
        IERC20[] memory tokens = new IERC20[](0);
        address to = address(0);
        _run(tokens, to);
    }

    function run(IERC20[] calldata tokens, address to) external {
        _run(tokens, to);
    }

    function _run(IERC20[] memory _tokens, address _to) internal broadcast {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

        DistributionCreator(creatorAddress).recoverFees(_tokens, _to);

        console.log("Fees recovered to:", _to);
    }
}

// SetUserFeeRebate script
contract SetUserFeeRebate is DistributionCreatorScript {
    // forge script scripts/DistributionCreator.s.sol:SetUserFeeRebate --rpc-url bsc --sender 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701 --broadcast --legacy
    function run() external {
        // MODIFY THESE VALUES TO SET YOUR DESIRED USER AND REBATE
        // 1_000_000_000 = 100%
        // 250_000_000 = 25%
        // 330_000_000  = 3
        address user = address(0x19674E9Af1A04DAf183F8E1A23E0afc2bc79A939);
        uint256 rebate = 1_000_000_000; // 100% 500000000
        _run(user, rebate);
    }

    function _run(address _user, uint256 _rebate) internal broadcast {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        DistributionCreator(creatorAddress).setUserFeeRebate(_user, _rebate);

        console.log("Fee rebate set to %s for user: %s", _rebate, _user);
    }
}

// SetRewardTokenMinAmounts script
contract SetRewardTokenMinAmounts is DistributionCreatorScript {
    // forge script scripts/DistributionCreator.s.sol:SetRewardTokenMinAmounts --rpc-url xdc --sender 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701 --broadcast --legacy
    function run() external {
        console.log("DEPLOYER_ADDRESS:", broadcaster);
        // MODIFY THESE VALUES TO SET YOUR DESIRED TOKENS AND AMOUNTS
        address[] memory tokens = new address[](1);
        uint256[] memory amounts = new uint256[](1);
        tokens[0] = 0xc3ef7ed4F97450Ae8dA2473068375788BdeB5c5c;
        amounts[0] = 1 ether / 10; // 0.1 tokens with 18 decimals
        _run(tokens, amounts);
    }

    function _run(address[] memory _tokens, uint256[] memory _amounts) internal broadcast {
        uint256 chainId = block.chainid;
        // address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

        DistributionCreator(creatorAddress).setRewardTokenMinAmounts(_tokens, _amounts);

        console.log("Minimum amounts updated for %s tokens", _tokens.length);
    }
}

// SetFeeRecipient script
contract SetFeeRecipient is DistributionCreatorScript {
    function run() external {
        // MODIFY THIS VALUE TO SET YOUR DESIRED RECIPIENT
        address recipient = address(0);
        _run(recipient);
    }

    function run(address recipient) external {
        _run(recipient);
    }

    function _run(address _recipient) internal broadcast {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

        DistributionCreator(creatorAddress).setFeeRecipient(_recipient);

        console.log("Fee recipient updated to:", _recipient);
    }
}

// SetMessage script
contract SetMessage is DistributionCreatorScript {
    function run() external {
        // MODIFY THIS VALUE TO SET YOUR DESIRED MESSAGE
        string memory message = "";
        _run(message);
    }

    function run(string calldata message) external {
        _run(message);
    }

    function _run(string memory _message) internal broadcast {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

        DistributionCreator(creatorAddress).setMessage(_message);

        console.log("Message updated to:", _message);
    }
}

// GetMessage script
contract GetMessage is DistributionCreatorScript {
    function run() external broadcast {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

        console.log("Creator address:", creatorAddress);
        string memory message = DistributionCreator(creatorAddress).message();

        console.log("Message is:", message);
    }
}

// ToggleSigningWhitelist script
contract ToggleSigningWhitelist is DistributionCreatorScript {
    function run() external {
        // MODIFY THIS VALUE TO SET YOUR DESIRED USER ADDRESS
        address user = address(0);
        _run(user);
    }

    function run(address user) external {
        _run(user);
    }

    function _run(address _user) internal broadcast {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

        DistributionCreator(creatorAddress).toggleSigningWhitelist(_user);

        console.log("Signing whitelist toggled for user:", _user);
    }
}

// AcceptConditions script
contract AcceptConditions is DistributionCreatorScript {
    function run() external broadcast {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

        DistributionCreator(creatorAddress).acceptConditions();

        console.log("Conditions accepted for:", broadcaster);
    }
}

// CreateCampaign script
// @notice Example usage for CreateCampaign:
// forge script scripts/DistributionCreator.s.sol:CreateCampaign \
// --rpc-url lisk \
// --sig "run((bytes32,address,address,uint256,uint32,uint32,uint32,bytes))" \
// "(\
// 0x0000000000000000000000000000000000000000000000000000000000000000,\
// 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701,\
// 0xE0688A2FE90d0f93F17f273235031062a210d691,\
// 2000000000000000000000,\
// 2,\
// 1732924800,\
// 604800,\
// 0x000000000000000000000000ec883424202a963af2a3e59bccaa0219e88ab9db00000000000000000000000000000000000000000000000000000000000007d00000000000000000000000000000000000000000000000000000000000000fa00000000000000000000000000000000000000000000000000000000000000fa0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000018000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000001c000000000000000000000000000000000000000000000000000000000000001e00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
// )"
contract CreateCampaign is DistributionCreatorScript {
    function run() external broadcast {
        bytes memory campaignData;
        // MODIFY THESE VALUES TO SET YOUR DESIRED CAMPAIGN PARAMETERS
        address rewardToken = address(0xB63B9f0eb4A6E6f191529D71d4D88cc8900Df2C9);

        CampaignParameters memory campaign = CampaignParameters({
            campaignId: bytes32(0),
            creator: address(0),
            rewardToken: rewardToken,
            amount: 9867825382083116891581,
            campaignType: 56,
            startTimestamp: 1752764400,
            duration: 385200,
            campaignData: hex"00000000000000000000000084bbc0be5a6f831a4e2c28a2f3b892c70acaa5b3000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000160000000000000000000000000000000000000000000000000000000000000018000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000001c000000000000000000000000000000000000000000000000000000000000001e000000000000000000000000000000000000000000000000000000000000000010000000000000000000000009fee01e948353e0897968a3ea955815aaa49f58d000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        });
        _run(campaign);
    }

    function run(CampaignParameters calldata campaign) external broadcast {
        _run(campaign);
    }

    function _run(CampaignParameters memory campaign) internal {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        IERC20(campaign.rewardToken).approve(creatorAddress, campaign.amount);
        bytes32 campaignId = DistributionCreator(creatorAddress).createCampaign(campaign);

        console.log("Campaign created with ID:", vm.toString(campaignId));
    }
}

// @notice Example usage for CreateCampaigns:
// forge script scripts/DistributionCreator.s.sol:CreateCampaigns \
// --rpc-url lisk \
// --sig "run((address,address,uint256,uint32,uint32,uint32,bytes)[])" \
// "[(\
// 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701,\
// 0xE0688A2FE90d0f93F17f273235031062a210d691,\
// 2000000000000000000000,\
// 2,\
// 1732924800,\
// 604800,\
// 0x000000000000000000000000ec883424202a963af2a3e59bccaa0219e88ab9db00000000000000000000000000000000000000000000000000000000000007d00000000000000000000000000000000000000000000000000000000000000fa00000000000000000000000000000000000000000000000000000000000000fa0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000018000000000000000000000000000000000000000000000000000000000000001a000000000000000000000000000000000000000000000000000000000000001c000000000000000000000000000000000000000000000000000000000000001e00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
// )]"
contract CreateCampaigns is DistributionCreatorScript {
    // MODIFY THESE VALUES TO SET YOUR DESIRED CAMPAIGN INPUTS
    mapping(uint256 => address[]) public targetTokens;
    uint256 distributionChain = 100;
    uint16[4] public chains = [1, 100, 8453, 59144];
    uint32 public campaignType = 22;
    uint32 public subCampaignType = 0;
    uint256 public tokenId = 0;
    address[] public whitelist = new address[](0);
    address[] public blacklist = new address[](0);
    bytes[] public hooks = new bytes[](0);
    string public apr = "1";
    bool public targetTokenPricing = true;
    bool public rewardTokenPricing = false;
    string public baseUrl = "https://app.hyperdrive.box/market/";
    address public rewardToken = 0x79385D4B4c531bBbDa25C4cFB749781Bd9E23039;
    uint32 startTimestamp = 1740787200;
    uint32 duration = 61 days;

    function run() external broadcast {
        // MODIFY THESE VALUES TO SET YOUR DESIRED CAMPAIGN INPUTS
        uint256 amount = 1e5 * 10 ** (IERC20Metadata(rewardToken).decimals());
        targetTokens[1] = [
            // 0xd7e470043241C10970953Bd8374ee6238e77D735
            0x324395D5d835F84a02A75Aa26814f6fD22F25698,
            0xca5dB9Bb25D09A9bF3b22360Be3763b5f2d13589,
            0xd41225855A5c5Ba1C672CcF4d72D1822a5686d30,
            0xA29A771683b4857bBd16e1e4f27D5B6bfF53209B,
            0x4c3054e51b46BE3191be9A05e73D73F1a2147854,
            0x158Ed87D7E529CFE274f3036ade49975Fb10f030,
            0xc8D47DE20F7053Cc02504600596A647A482Bbc46,
            0x7548c4F665402BAb3a4298B88527824B7b18Fe27,
            0xA4090183878d5B7b6Ad104863743dd7E58985321,
            0x8f2AC104e07d94488a1821E5A393351FCA9239aa,
            0x05b65FA90AD702e6Fd0C3Bd7c4c9C47BAB2BEa6b,
            0xf1232Dc21eADAf503D82f1e1361CfF2BBf40394D
        ];
        // targetTokens[100] = [
        //     0x2f840f1575EE77adAa43415Ac5953F7Db9F8C6ba,
        //     0xEe9BFf933aDD313C4289E98dA80fEfbF9d5Cd9Ba,
        //     0x9248f874AaA2c53AD9324d7A2D033ea133443874
        // ];
        targetTokens[8453] = [
            0x2a1ca35Ded36C531F77c614b5AAA0d4F86edbB06,
            0xFcdaF9A4A731C24ed2E1BFd6FA918d9CF7F50137,
            0x1243C06146ACa2D4Aaf8F9860F6D8d59d636d46C,
            0xceD9F810098f8329472AEFbaa1112534E96A5c7b,
            0x9bAdB6A21FbA04EE94fde3E85F7d170E90394c89,
            0xD9b66D9a819B36ECEfC26B043eF3B422d5A6123a,
            0xdd8E1B14A04cbdD98dfcAF3F0Db84A80Bfb8FC25
        ];
        targetTokens[59144] = [0xB56e0Bf37c4747AbbC3aA9B8084B0d9b9A336777, 0x1cB0E96C07910fee9a22607bb9228c73848903a3];

        CampaignInput[] memory inputs;
        uint256 countInputs = 0;
        {
            uint256 numberCampaigns = 0;
            for (uint256 i = 0; i < chains.length; i++) {
                numberCampaigns += targetTokens[chains[i]].length;
            }
            inputs = new CampaignInput[](numberCampaigns);
        }
        for (uint256 i = 0; i < chains.length; i++) {
            uint256 chainId = chains[i];
            string memory baseUrlChain = string.concat(baseUrl, vm.toString(chainId), "/");
            address[] memory tokens = targetTokens[chainId];
            for (uint256 j = 0; j < tokens.length; j++) {
                bytes memory campaignData = abi.encode(
                    tokens[j],
                    subCampaignType,
                    tokenId,
                    whitelist,
                    blacklist,
                    string.concat(baseUrlChain, vm.toString(tokens[j])),
                    hooks,
                    apr,
                    targetTokenPricing,
                    rewardTokenPricing
                );
                campaignData = abi.encode(uint32(chainId), campaignData);
                campaignData = abi.encodePacked(campaignData, hex"c0c0c0c0");
                inputs[countInputs++] = CampaignInput({
                    creator: address(0),
                    rewardToken: rewardToken,
                    amount: amount,
                    campaignType: campaignType,
                    startTimestamp: startTimestamp,
                    duration: duration,
                    campaignData: campaignData
                });
            }
        }

        _run(inputs);
    }

    function run(CampaignInput[] calldata inputs) external broadcast {
        _run(inputs);
    }

    function _run(CampaignInput[] memory inputs) internal {
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        DistributionCreator creator = DistributionCreator(creatorAddress);

        uint256 inputsLength = inputs.length;
        CampaignParameters[] memory campaigns = new CampaignParameters[](inputsLength);

        // Convert inputs to CampaignParameters, letting the contract compute campaignId
        for (uint256 i = 0; i < inputsLength; i++) {
            campaigns[i] = CampaignParameters({
                creator: inputs[i].creator,
                rewardToken: inputs[i].rewardToken,
                amount: inputs[i].amount,
                campaignType: inputs[i].campaignType,
                startTimestamp: inputs[i].startTimestamp,
                duration: inputs[i].duration,
                campaignData: inputs[i].campaignData,
                campaignId: bytes32(0) // Will be computed by the contract
            });
        }

        IERC20(rewardToken).approve(creatorAddress, inputs.length * inputs[0].amount);
        bytes32[] memory campaignIds = creator.createCampaigns(campaigns);

        console.log("Created %s campaigns:", inputsLength);
        for (uint256 i = 0; i < campaignIds.length; i++) {
            console.log("Campaign %s ID: %s", i, vm.toString(campaignIds[i]));
        }
    }
}

contract OverrideCampaign is DistributionCreatorScript {
    function run() external broadcast {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        // MODIFY THESE VALUES TO SET YOUR DESIRED CAMPAIGN PARAMETERS
        bytes32 campaignId = 0xf93a5b762bd5a2a3e6cf6dcb83cb54f70ab2de457e0dc4cbb4da29ba8b54e4ad;
        address targetToken = address(0x1337BedC9D22ecbe766dF105c9623922A27963EC);
        address[] memory whitelist = new address[](0);
        address[] memory blacklist = new address[](0);
        string memory url = "https://curve.fi/dex/#/xdai/pools/3pool/deposit";
        bytes[] memory forwarders = new bytes[](0);
        bytes[] memory hooks = new bytes[](0);
        // END

        CampaignParameters memory campaign = DistributionCreator(creatorAddress).campaign(campaignId);

        CampaignParameters memory overrideCampaign = CampaignParameters({
            campaignId: bytes32(campaign.campaignId),
            creator: address(0),
            rewardToken: address(0x65A1DfB54CDec9011688b1818A27A8C687e6B1ed),
            amount: campaign.amount,
            campaignType: 1,
            startTimestamp: uint32(campaign.startTimestamp),
            duration: 1.5 days,
            campaignData: abi.encode(targetToken, whitelist, blacklist, url, forwarders, hooks, hex"")
        });
        _run(overrideCampaign);
    }

    function run(CampaignParameters calldata campaign) external broadcast {
        _run(campaign);
    }

    function _run(CampaignParameters memory campaign) internal {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        IERC20(campaign.rewardToken).approve(creatorAddress, campaign.amount);
        DistributionCreator(creatorAddress).overrideCampaign(campaign.campaignId, campaign);

        console.log("Campaign created with ID:", vm.toString(campaign.campaignId));
    }
}

contract ReallocateCampaign is DistributionCreatorScript {
    function run() external broadcast {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        // MODIFY THESE VALUES TO SET YOUR DESIRED CAMPAIGN PARAMETERS
        bytes32 campaignId = 0x490af89ce201bb272809983117aa95ce4a6cfcbb178343076519fc80ec2ff408;
        address[] memory froms = new address[](2);
        froms[0] = 0xBA12222222228d8Ba445958a75a0704d566BF2C8;
        froms[1] = 0x53C9ACaB7D5f3078141D1178EeA782c7129D92C9;
        address to = 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701;
        // END

        _run(campaignId, froms, to);
    }

    function run(bytes32 campaignId, address[] memory froms, address to) external broadcast {
        _run(campaignId, froms, to);
    }

    function _run(bytes32 campaignId, address[] memory froms, address to) internal {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        DistributionCreator(creatorAddress).reallocateCampaignRewards(campaignId, froms, to);
    }
}

contract ReallocateCampaigns is DistributionCreatorScript {
    function run() external broadcast {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        // MODIFY THESE VALUES TO SET YOUR DESIRED CAMPAIGN PARAMETERS

        // MAINNET
        // now
        bytes32[] memory campaignIds = new bytes32[](2);
        campaignIds[0] = 0x20cc77d4b6138837753a6f37a6b6c5a7675d1bbf26b3e1f759b437e3bafbc8a5;
        campaignIds[1] = 0x552c13b139cb09bb00e486c9ff1b18bf31fa4af94efa92e953a5654922ef50ba;

        // // future
        // bytes32[] memory campaignIds = new bytes32[](4);
        // campaignIds[0] = 0x31daf1460445bd688f895014ee21d58a41dc31ccb1e3592c7c5af953194625b4;
        // campaignIds[1] = 0x989ba1dbf3430e313891984d176ffc79a9aa8410d14a2215156242d981b1dc48;
        // campaignIds[2] = 0x5ff68f84ce65b84ef0da170a44716a899e34b79b7d86ef98f4c8495c2b9a715e;
        // campaignIds[3] = 0x735de250ce1ca074972d15742fc9367f93f007c057c540d88bd42a6a77b7881c;

        // // BASE
        // // now
        // bytes32[] memory campaignIds = new bytes32[](4);
        // campaignIds[0] = 0x477d78ee82651afdca318485f877ff1923afac46e55a39705665e1da00aa4ab4;
        // campaignIds[1] = 0x0de8aad778761479b33f21d81ecffd505c0b3011c66af40fa2ec464e5899b707;
        // campaignIds[2] = 0x2340c7c2d3efe6ed0eb6e532f6707b692a58b2340ca31ac426544d5166f4a3d3;
        // campaignIds[3] = 0x600d210d7390c8a78dcaeb820e70e16b38c69ef00ca458ccfefe5abd9c9d4d9b;
        // campaignIds[4] = 0x18a6be06c3e3f870858ff229f7cb8fa02f9ec8a89586860ed2538bf1cc716a83;
        // campaignIds[5] = 0x6fcea5032087c0d5acf11ce21c0a0f9ad1a6532f0323bd8aac390babe40b764e;
        // campaignIds[6] = 0xe4c666e923ca13e830ed476f0cded9dd1dd6457c637fbc1ae4b2b473ebe9c211;

        // // future
        // bytes32[] memory campaignIds = new bytes32[](4);
        // campaignIds[0] = 0xb63933be5065bce6c9b2719f7afa76f3c41a992eddef74b1a321351967313ce7;
        // campaignIds[1] = 0x033ed946fb107cf928ffe5d1d4d57c649999769eeb38773443a50eb862caab19;
        // campaignIds[2] = 0x5716e04285cf59798cfc1dcc554a07bb2b00031feb9dd7e0fe01ca712fa3ae09;
        // campaignIds[3] = 0x0323d17f64608e1f402192ec920b4a9e0203ddfc262fdc4d0a233bc7c64c4f29;
        // campaignIds[4] = 0xb32cae66a15634a6b3e65eeb874f1d437288c06ab0604d6e21737e117efdb9bb;

        address[] memory froms = new address[](1);
        froms[0] = 0x000000DCeb71f3107909b1b748424349bfde5493;
        address to = 0x9a8FEe232DCF73060Af348a1B62Cdb0a19852d13;
        // END

        for (uint256 i = 0; i < campaignIds.length; i++) {
            _run(campaignIds[i], froms, to);
        }
    }

    function run(bytes32 campaignId, address[] memory froms, address to) external broadcast {
        _run(campaignId, froms, to);
    }

    function _run(bytes32 campaignId, address[] memory froms, address to) internal {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        DistributionCreator(creatorAddress).reallocateCampaignRewards(campaignId, froms, to);
    }
}

// CreateCampaign script
contract CreateCampaignTest is DistributionCreatorScript {
    function run() external {
        vm.createSelectFork(vm.envString("BASE_NODE_URI"));
        uint256 chainId = block.chainid;

        /// TODO: COMPLETE
        IERC20 rewardToken = IERC20(0xC011882d0f7672D8942e7fE2248C174eeD640c8f);
        uint256 amount = 100 ether;
        /// END

        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        DistributionCreator distributionCreator = DistributionCreator(creatorAddress);

        vm.startBroadcast(broadcaster);

        MockToken(address(rewardToken)).mint(broadcaster, amount);
        rewardToken.approve(address(distributionCreator), amount);

        uint32 startTimestamp = uint32(block.timestamp + 600);

        bytes32 campaignId = distributionCreator.createCampaign(
            CampaignParameters({
                campaignId: bytes32(0),
                creator: broadcaster,
                rewardToken: address(rewardToken),
                amount: amount,
                campaignType: 1,
                startTimestamp: startTimestamp,
                duration: 3600 * 24,
                campaignData: abi.encode(
                    0xbEEfa1aBfEbE621DF50ceaEF9f54FdB73648c92C,
                    new address[](0),
                    new address[](0),
                    "",
                    new bytes[](0),
                    new bytes[](0),
                    hex""
                )
            })
        );
        vm.stopBroadcast();

        CampaignParameters memory campaign = distributionCreator.campaign(campaignId);
        require(campaign.creator == broadcaster, "Invalid creator");
        require(campaign.rewardToken == address(rewardToken), "Invalid reward token");
        require(campaign.amount == (amount * (1e9 - distributionCreator.defaultFees())) / 1e9, "Invalid amount");
        require(campaign.campaignType == 1, "Invalid campaign type");
        require(campaign.startTimestamp == startTimestamp, "Invalid start timestamp");
        require(campaign.duration == 3600 * 24, "Invalid duration");

        console.log("Campaign created with ID:", vm.toString(campaignId));
    }
}

contract UpgradeAndBuildUpgradeToPayload is DistributionCreatorScript {
    function run() external broadcast {
        uint256 chainId = block.chainid;
        address distributionCreator = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

        address distributionCreatorImpl = address(new DistributionCreator());

        bytes memory payload = abi.encodeWithSelector(ITransparentUpgradeableProxy.upgradeTo.selector, distributionCreatorImpl);

        // TODO: Set safe address if needed
        address safe = address(0);

        _serializeJson(
            chainId,
            distributionCreator, // target address (the proxy)
            0, // value
            payload, // direct upgrade call
            Operation.Call, // standard call (not delegate)
            hex"", // signature
            safe // safe address
        );
    }
}

contract SetRewardTokenMinAmountsDistributor is DistributionCreatorScript {
    function run() external broadcast {
        // MODIFY THESE VALUES TO SET YOUR DESIRED CAMPAIGN PARAMETERS AND SIGNATURE
        address[] memory tokens = new address[](1);
        uint256[] memory minAmounts = new uint256[](1);
        tokens[0] = 0x79385D4B4c531bBbDa25C4cFB749781Bd9E23039;
        minAmounts[0] = 1e18;

        _run(tokens, minAmounts);
    }

    function run(address[] memory tokens, uint256[] memory minAmounts) external broadcast {
        _run(tokens, minAmounts);
    }

    function _run(address[] memory tokens, uint256[] memory minAmounts) internal {
        uint256 chainId = block.chainid;
        address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

        DistributionCreator(creatorAddress).setRewardTokenMinAmounts(tokens, minAmounts);
    }
}

// SetFeesMultichain script
contract SetFeesMultichain is DistributionCreatorScript {
    struct FailedChain {
        string network;
        uint256 chainId;
        string reason;
    }

    function run() external {
        // Set default fees to 0 and campaign fees to 3% (30,000,000 in 9 decimals)
        uint256 defaultFees = 0;
        uint256 campaignFees = 5000000; // 0.5%

        // Get all networks from foundry.toml
        string[2][] memory networks = vm.rpcUrls();

        // Track failed chains
        FailedChain[] memory failedChains = new FailedChain[](networks.length);
        uint256 failedCount = 0;
        uint256 successCount = 0;

        for (uint256 i = 0; i < networks.length; i++) {
            string memory network = networks[i][0];

            // Skip localhost and fork
            if (
                keccak256(abi.encodePacked(network)) == keccak256(abi.encodePacked("localhost")) ||
                keccak256(abi.encodePacked(network)) == keccak256(abi.encodePacked("fork")) ||
                keccak256(abi.encodePacked(network)) == keccak256(abi.encodePacked("zksync")) ||
                keccak256(abi.encodePacked(network)) == keccak256(abi.encodePacked("nibiru")) // No multisig deployed
            ) {
                continue;
            }

            // // Create fork for this network
            vm.createSelectFork(network);
            uint256 chainId = block.chainid;
            address creatorAddress = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

            // Create contract instances to reuse existing logic
            // SetFees setFeesContract = new SetFees();
            // SetCampaignFees setCampaignFeesContract = new SetCampaignFees();

            // Set default fees using existing contract
            // TODO: Instead of broadcasting, we should just write the transaction to the json file
            bytes memory setFeesPayload = abi.encodeWithSelector(DistributionCreator.setFees.selector, defaultFees);

            // console.log("Safe address: %s", safe);
            address safe = address(0);
            _serializeJson(
                chainId,
                creatorAddress, // target address (the DistributionCreator proxy)
                0, // value
                setFeesPayload, // setFees call
                Operation.Call, // standard call (not delegate)
                hex"", // signature
                safe // safe address
            );
            console.log("Default fees transaction serialized for %s (chain %s)", network, chainId);
            successCount++;

            // // Set campaign fees for each type using existing contract
            // if (DistributionCreator(creatorAddress).campaignSpecificFees(27) != campaignFees) {
            //     console.log("Setting campaign fees for type 27 to %s", campaignFees);
            //     try setCampaignFeesContract.run(27, campaignFees) {
            //         // Success
            //         console.log("Fees updated on %s (chain %s)", network, chainId);
            //         successCount++;
            //     } catch (bytes memory err) {
            //         console.log("Failed to set campaign fees for type %s on %s", 27, network);
            //         failedChains[failedCount] = FailedChain(network, chainId, string(err));
            //         failedCount++;
            //         vm.stopBroadcast();
            //     }
            // }
        }

        // Display summary
        console.log("");
        console.log("=== MULTICHAIN FEE SETTING SUMMARY ===");
        console.log("Successful chains: %s", successCount);
        console.log("Failed chains: %s", failedCount);

        if (failedCount > 0) {
            console.log("");
            console.log("Failed chains details:");
            for (uint256 i = 0; i < failedCount; i++) {
                console.log("- %s (Chain ID: %s) - Reason: %s", failedChains[i].network, failedChains[i].chainId, failedChains[i].reason);
            }
        }

        console.log("");
        console.log("Multichain fee setting completed!");
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.17;

import { console } from "forge-std/console.sol";

import { BaseScript } from "./utils/Base.s.sol";

import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { ITransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { PullTokenWrapperTransfer } from "../contracts/partners/tokenWrappers/PullTokenWrapperTransfer.sol";
import { DistributionCreator } from "../contracts/DistributionCreator.sol";
import { IAccessControlManager } from "../contracts/interfaces/IAccessControlManager.sol";
import { MockToken } from "../contracts/mock/MockToken.sol";

contract DeployPullTokenWrapperTransfer is BaseScript {
    // forge script scripts/deployPullTokenWrapperTransfer.s.sol --rpc-url katana --sender 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701 --broadcast --verify —verifier=blockscout   --verifier-url 'https://explorer.katanarpc.com/api/'
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Katana
        address underlying = 0x7F1f4b4b29f5058fA32CC7a97141b8D7e5ABDC2d;
        address distributionCreator = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        address minter = 0xb08AB4332AD871F89da24df4751968A61e58013c;
        // Keeping the same name and symbol as the original underlying token so it's invisible for users
        string memory name = "Katana Network Token (wrapped v2)";
        string memory symbol = "KAT";

        // Deploy implementation
        PullTokenWrapperTransfer implementation = new PullTokenWrapperTransfer();
        console.log("PullTokenWrapperTransfer Implementation:", address(implementation));

        // Deploy proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), "");
        console.log("PullTokenWrapperTransfer Proxy:", address(proxy));

        // Initialize
        PullTokenWrapperTransfer(address(proxy)).initialize(underlying, distributionCreator, minter, name, symbol);

        address[] memory tokens;
        tokens[0] = address(proxy);

        uint256[] memory amounts;
        amounts[0] = 1e18;
        DistributionCreator(distributionCreator).setRewardTokenMinAmounts(tokens, amounts); // Set the minimum amount for the token wrapper

        vm.stopBroadcast();
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.17;

import { console } from "forge-std/console.sol";

import { BaseScript } from "./utils/Base.s.sol";

import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { ITransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { PufferPointTokenWrapper } from "../contracts/partners/tokenWrappers/PufferPointTokenWrapper.sol";
import { DistributionCreator } from "../contracts/DistributionCreator.sol";
import { IAccessControlManager } from "../contracts/interfaces/IAccessControlManager.sol";
import { MockToken } from "../contracts/mock/MockToken.sol";

contract DeployPufferPointTokenWrapper is BaseScript {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        address underlying = 0x282A69142bac47855C3fbE1693FcC4bA3B4d5Ed6;
        uint32 cliffDuration = 500;
        // uint32 cliffDuration = 1 weeks;
        IAccessControlManager manager = IAccessControlManager(0x0E632a15EbCBa463151B5367B4fCF91313e389a6);
        address distributionCreator = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;

        // ARBITRUM TEST
        /*
        // aglaMerkl
        address underlying = 0xE0688A2FE90d0f93F17f273235031062a210d691;
        uint32 cliffDuration = 500;
        // uint32 cliffDuration = 1 weeks;
        IAccessControlManager manager = IAccessControlManager(0xA86CC1ae2D94C6ED2aB3bF68fB128c2825673267);
        address distributionCreator = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        */

        // Deploy implementation
        PufferPointTokenWrapper implementation = new PufferPointTokenWrapper();
        console.log("PufferPointTokenWrapper Implementation:", address(implementation));
        /*
        // Deploy proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), "");
        console.log("PufferPointTokenWrapper Proxy:", address(proxy));

        // Initialize
        PufferPointTokenWrapper(address(proxy)).initialize(underlying, cliffDuration, manager, distributionCreator);
        */
        vm.stopBroadcast();
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.17;

import { console } from "forge-std/console.sol";

import { BaseScript } from "./utils/Base.s.sol";

import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { ITransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";

import { PullTokenWrapperAllow } from "../contracts/partners/tokenWrappers/PullTokenWrapperAllow.sol";
import { PullTokenWrapperWithdraw } from "../contracts/partners/tokenWrappers/PullTokenWrapperWithdraw.sol";
import { DistributionCreator } from "../contracts/DistributionCreator.sol";
import { IAccessControlManager } from "../contracts/interfaces/IAccessControlManager.sol";
import { MockToken } from "../contracts/mock/MockToken.sol";

contract DeployPullTokenWrapper is BaseScript {
    // forge script scripts/deployPullTokenWrapper.s.sol --rpc-url plasma --sender 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701 --broadcast --verify
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        address distributionCreator = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        // ------------------------------------------------------------------------
        // TO EDIT
        address underlying = 0x5aA4bc74811D672DA5308019dA4779f673e60B47;
        address holder = 0xdef1FA4CEfe67365ba046a7C630D6B885298E210;

        // Need to choose the implementation type and if implementation needs to be deployed

        // address implementation = address(new PullTokenWrapperWithdraw());
        address implementation = address(new PullTokenWrapperAllow());
        // Ethereum implementation of PullTokenWrapperAllow
        // address implementation = 0x979a04fd2f3A6a2B3945A715e24b974323E93567;
        // Ethereum implementation of PullTokenWrapperWithdraw
        // address implementation = 0x721d37cf37e230E120a09adbBB7aAB0CF729AcA1

        // Keeping the same name and symbol as the original underlying token so it's invisible for users
        string memory name = string(abi.encodePacked(IERC20Metadata(underlying).name(), " (wrapped)"));
        string memory symbol = IERC20Metadata(underlying).symbol();

        // Names to override if deploying a PullTokenWrapperWithdraw implementation
        // name = "USDT0 (wrapped)";
        // symbol = "USDT0";

        // ------------------------------------------------------------------------

        console.log("PullTokenWrapper Implementation:", address(implementation));

        // Deploy proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), "");
        console.log("PullTokenWrapper Proxy:", address(proxy));

        // Initialize
        PullTokenWrapperAllow(address(proxy)).initialize(underlying, distributionCreator, holder, name, symbol);

        vm.stopBroadcast();
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.17;

import { console } from "forge-std/console.sol";

import { BaseScript } from "./utils/Base.s.sol";

import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { ITransparentUpgradeableProxy } from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";

import { TokenTGEWrapper } from "../contracts/partners/tokenWrappers/TokenTGEWrapper.sol";
import { DistributionCreator } from "../contracts/DistributionCreator.sol";
import { IAccessControlManager } from "../contracts/interfaces/IAccessControlManager.sol";
import { MockToken } from "../contracts/mock/MockToken.sol";

contract DeployTokenTGEWrapper is BaseScript {
    // forge script scripts/deployTokenTGEWrapper.s.sol --rpc-url bsc --sender 0xA9DdD91249DFdd450E81E1c56Ab60E1A62651701 --broadcast --verify
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        address distributionCreator = 0x8BB4C975Ff3c250e0ceEA271728547f3802B36Fd;
        // ------------------------------------------------------------------------
        // TO EDIT
        address underlying = 0x499D35eBE6cEe9B2Ac35Fd003fcBbeeB9CFc7B32;
        // ------------------------------------------------------------------------

        address implementation = address(new TokenTGEWrapper());

        console.log("Wrapper Implementation:", address(implementation));

        // Deploy proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), "");
        console.log("Wrapper Proxy:", address(proxy));
        // Initialize
        TokenTGEWrapper(address(proxy)).initialize(underlying, 1764068400, distributionCreator);

        vm.stopBroadcast();
    }
}

