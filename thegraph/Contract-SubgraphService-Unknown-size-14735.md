
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.33;

import { IGraphPayments } from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import { IGraphTallyCollector } from "@graphprotocol/interfaces/contracts/horizon/IGraphTallyCollector.sol";
import { IRewardsIssuer } from "@graphprotocol/interfaces/contracts/contracts/rewards/IRewardsIssuer.sol";
import { IDataService } from "@graphprotocol/interfaces/contracts/data-service/IDataService.sol";
import { ISubgraphService } from "@graphprotocol/interfaces/contracts/subgraph-service/ISubgraphService.sol";
import { IAllocation } from "@graphprotocol/interfaces/contracts/subgraph-service/internal/IAllocation.sol";
import { ILegacyAllocation } from "@graphprotocol/interfaces/contracts/subgraph-service/internal/ILegacyAllocation.sol";

import { OwnableUpgradeable } from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import { MulticallUpgradeable } from "@openzeppelin/contracts-upgradeable/utils/MulticallUpgradeable.sol";
import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import { DataServicePausableUpgradeable } from "@graphprotocol/horizon/contracts/data-service/extensions/DataServicePausableUpgradeable.sol";
import { DataService } from "@graphprotocol/horizon/contracts/data-service/DataService.sol";
import { DataServiceFees } from "@graphprotocol/horizon/contracts/data-service/extensions/DataServiceFees.sol";
import { Directory } from "./utilities/Directory.sol";
import { AllocationManager } from "./utilities/AllocationManager.sol";
import { SubgraphServiceV1Storage } from "./SubgraphServiceStorage.sol";

import { TokenUtils } from "@graphprotocol/contracts/contracts/utils/TokenUtils.sol";
import { PPMMath } from "@graphprotocol/horizon/contracts/libraries/PPMMath.sol";
import { Allocation } from "./libraries/Allocation.sol";

/**
 * @title SubgraphService contract
 * @author Edge & Node
 * @notice A data service contract for subgraph indexing and querying
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
contract SubgraphService is
    Initializable,
    OwnableUpgradeable,
    MulticallUpgradeable,
    DataService,
    DataServicePausableUpgradeable,
    DataServiceFees,
    Directory,
    AllocationManager,
    IRewardsIssuer,
    ISubgraphService,
    SubgraphServiceV1Storage
{
    using PPMMath for uint256;
    using Allocation for mapping(address => IAllocation.State);
    using Allocation for IAllocation.State;
    using TokenUtils for IGraphToken;

    /**
     * @notice Checks that an indexer is registered
     * @param indexer The address of the indexer
     */
    modifier onlyRegisteredIndexer(address indexer) {
        _checkRegisteredIndexer(indexer);
        _;
    }

    /**
     * @notice Constructor for the SubgraphService contract
     * @dev DataService and Directory constructors set a bunch of immutable variables
     * @param graphController The address of the Graph Controller contract
     * @param disputeManager The address of the DisputeManager contract
     * @param graphTallyCollector The address of the GraphTallyCollector contract
     * @param curation The address of the Curation contract
     */
    constructor(
        address graphController,
        address disputeManager,
        address graphTallyCollector,
        address curation
    ) DataService(graphController) Directory(address(this), disputeManager, graphTallyCollector, curation) {
        _disableInitializers();
    }

    /// @inheritdoc ISubgraphService
    function initialize(
        address owner,
        uint256 minimumProvisionTokens,
        uint32 maximumDelegationRatio,
        uint256 stakeToFeesRatio_
    ) external initializer {
        __Ownable_init(owner);
        __Multicall_init();
        __DataService_init();
        __DataServicePausable_init();
        __AllocationManager_init("SubgraphService", "1.0");

        _setProvisionTokensRange(minimumProvisionTokens, type(uint256).max);
        _setDelegationRatio(maximumDelegationRatio);
        _setStakeToFeesRatio(stakeToFeesRatio_);
    }

    /**
     * @notice
     * @dev Implements {IDataService.register}
     *
     * Requirements:
     * - The URL must not be empty
     * - The provision must be valid according to the subgraph service rules
     *
     * Emits a {ServiceProviderRegistered} event
     *
     * @param indexer The address of the indexer to register
     * @param data Encoded registration data:
     *  - string `url`: The URL of the indexer
     *  - string `geohash`: The geohash of the indexer
     *  - address `paymentsDestination`: The address where the indexer wants to receive payments.
     *    Use zero address for automatically restaking payments.
     */
    /// @inheritdoc IDataService
    function register(
        address indexer,
        bytes calldata data
    ) external override onlyAuthorizedForProvision(indexer) onlyValidProvision(indexer) whenNotPaused {
        (string memory url, string memory geohash, address paymentsDestination_) = abi.decode(
            data,
            (string, string, address)
        );

        require(bytes(url).length > 0, SubgraphServiceEmptyUrl());
        require(bytes(geohash).length > 0, SubgraphServiceEmptyGeohash());

        // Register the indexer
        indexers[indexer] = Indexer({ url: url, geoHash: geohash });
        _setPaymentsDestination(indexer, paymentsDestination_);

        emit ServiceProviderRegistered(indexer, data);
    }

    /**
     * @notice Accept staged parameters in the provision of a service provider
     * @dev Implements {IDataService-acceptProvisionPendingParameters}
     *
     * Requirements:
     * - The indexer must be registered
     * - Must have previously staged provision parameters, using {IHorizonStaking-setProvisionParameters}
     * - The new provision parameters must be valid according to the subgraph service rules
     *
     * Emits a {ProvisionPendingParametersAccepted} event
     *
     * @param indexer The address of the indexer to accept the provision for
     */
    /// @inheritdoc IDataService
    function acceptProvisionPendingParameters(
        address indexer,
        bytes calldata
    ) external override onlyAuthorizedForProvision(indexer) whenNotPaused {
        _acceptProvisionParameters(indexer);
        emit ProvisionPendingParametersAccepted(indexer);
    }

    /**
     * @notice Allocates tokens to subgraph deployment, manifesting the indexer's commitment to index it
     * @dev This is the equivalent of the `allocate` function in the legacy Staking contract.
     *
     * Requirements:
     * - The indexer must be registered
     * - The provision must be valid according to the subgraph service rules
     * - Allocation id cannot be zero
     * - Allocation id cannot be reused from the legacy staking contract
     * - The indexer must have enough available tokens to allocate
     *
     * The `allocationProof` is a 65-bytes Ethereum signed message of `keccak256(indexerAddress,allocationId)`.
     *
     * See {AllocationManager-allocate} for more details.
     *
     * Emits {ServiceStarted} and {AllocationCreated} events
     *
     * @param indexer The address of the indexer
     * @param data Encoded data:
     * - bytes32 `subgraphDeploymentId`: The id of the subgraph deployment
     * - uint256 `tokens`: The amount of tokens to allocate
     * - address `allocationId`: The id of the allocation
     * - bytes `allocationProof`: Signed proof of the allocation id address ownership
     */
    /// @inheritdoc IDataService
    function startService(
        address indexer,
        bytes calldata data
    )
        external
        override
        onlyAuthorizedForProvision(indexer)
        onlyValidProvision(indexer)
        onlyRegisteredIndexer(indexer)
        whenNotPaused
    {
        (bytes32 subgraphDeploymentId, uint256 tokens, address allocationId, bytes memory allocationProof) = abi.decode(
            data,
            (bytes32, uint256, address, bytes)
        );
        _allocate(indexer, allocationId, subgraphDeploymentId, tokens, allocationProof, _delegationRatio);
        emit ServiceStarted(indexer, data);
    }

    /**
     * @notice Close an allocation, indicating that the indexer has stopped indexing the subgraph deployment
     * @dev This is the equivalent of the `closeAllocation` function in the legacy Staking contract.
     * There are a few notable differences with the legacy function:
     * - allocations are nowlong lived. All service payments, including indexing rewards, should be collected periodically
     * without the need of closing the allocation. Allocations should only be closed when indexers want to reclaim the allocated
     * tokens for other purposes.
     * - No POI is required to close an allocation. Indexers should present POIs to collect indexing rewards using {collect}.
     *
     * Requirements:
     * - The indexer must be registered
     * - Allocation must exist and be open
     *
     * Emits {ServiceStopped} and {AllocationClosed} events
     *
     * @param indexer The address of the indexer
     * @param data Encoded data:
     * - address `allocationId`: The id of the allocation
     */
    /// @inheritdoc IDataService
    function stopService(
        address indexer,
        bytes calldata data
    ) external override onlyAuthorizedForProvision(indexer) onlyRegisteredIndexer(indexer) whenNotPaused {
        address allocationId = abi.decode(data, (address));
        require(
            _allocations.get(allocationId).indexer == indexer,
            SubgraphServiceAllocationNotAuthorized(indexer, allocationId)
        );
        _closeAllocation(allocationId, false);
        emit ServiceStopped(indexer, data);
    }

    /**
     * @notice Collects payment for the service provided by the indexer
     * Allows collecting different types of payments such as query fees and indexing rewards.
     * It uses Graph Horizon payments protocol to process payments.
     * Reverts if the payment type is not supported.
     * @dev This function is the equivalent of the `collect` function for query fees and the `closeAllocation` function
     * for indexing rewards in the legacy Staking contract.
     *
     * Requirements:
     * - The indexer must be registered
     * - The provision must be valid according to the subgraph service rules
     *
     * Emits a {ServicePaymentCollected} event. Emits payment type specific events.
     *
     * For query fees, see {SubgraphService-_collectQueryFees} for more details.
     * For indexing rewards, see {AllocationManager-_collectIndexingRewards} for more details.
     *
     * @param indexer The address of the indexer
     * @param paymentType The type of payment to collect as defined in {IGraphPayments}
     * @param data Encoded data:
     *    - For query fees:
     *      - IGraphTallyCollector.SignedRAV `signedRav`: The signed RAV
     *    - For indexing rewards:
     *      - address `allocationId`: The id of the allocation
     *      - bytes32 `poi`: The POI being presented
     *      - bytes `poiMetadata`: The metadata associated with the POI. See {AllocationManager-_collectIndexingRewards} for more details.
     */
    /// @inheritdoc IDataService
    function collect(
        address indexer,
        IGraphPayments.PaymentTypes paymentType,
        bytes calldata data
    )
        external
        override
        onlyAuthorizedForProvision(indexer)
        onlyValidProvision(indexer)
        onlyRegisteredIndexer(indexer)
        whenNotPaused
        returns (uint256)
    {
        uint256 paymentCollected = 0;

        if (paymentType == IGraphPayments.PaymentTypes.QueryFee) {
            paymentCollected = _collectQueryFees(indexer, data);
        } else if (paymentType == IGraphPayments.PaymentTypes.IndexingRewards) {
            paymentCollected = _collectIndexingRewards(indexer, data);
        } else {
            revert SubgraphServiceInvalidPaymentType(paymentType);
        }

        emit ServicePaymentCollected(indexer, paymentType, paymentCollected);
        return paymentCollected;
    }

    /**
     * @notice See {IHorizonStaking-slash} for more details.
     * @dev Slashing is delegated to the {DisputeManager} contract which is the only one that can call this
     * function.
     */
    /// @inheritdoc IDataService
    function slash(address indexer, bytes calldata data) external override onlyDisputeManager {
        (uint256 tokens, uint256 reward) = abi.decode(data, (uint256, uint256));
        _graphStaking().slash(indexer, tokens, reward, address(_disputeManager()));
        emit ServiceProviderSlashed(indexer, tokens);
    }

    /// @inheritdoc ISubgraphService
    function closeStaleAllocation(address allocationId) external override whenNotPaused {
        IAllocation.State memory allocation = _allocations.get(allocationId);
        require(allocation.isStale(maxPOIStaleness), SubgraphServiceCannotForceCloseAllocation(allocationId));
        require(!allocation.isAltruistic(), SubgraphServiceAllocationIsAltruistic(allocationId));
        _closeAllocation(allocationId, true);
    }

    /// @inheritdoc ISubgraphService
    function resizeAllocation(
        address indexer,
        address allocationId,
        uint256 tokens
    )
        external
        onlyAuthorizedForProvision(indexer)
        onlyValidProvision(indexer)
        onlyRegisteredIndexer(indexer)
        whenNotPaused
    {
        require(
            _allocations.get(allocationId).indexer == indexer,
            SubgraphServiceAllocationNotAuthorized(indexer, allocationId)
        );
        _resizeAllocation(allocationId, tokens, _delegationRatio);
    }

    /// @inheritdoc ISubgraphService
    function migrateLegacyAllocation(
        address indexer,
        address allocationId,
        bytes32 subgraphDeploymentId
    ) external override onlyOwner {
        _migrateLegacyAllocation(indexer, allocationId, subgraphDeploymentId);
    }

    /// @inheritdoc ISubgraphService
    function setPauseGuardian(address pauseGuardian, bool allowed) external override onlyOwner {
        _setPauseGuardian(pauseGuardian, allowed);
    }

    /// @inheritdoc ISubgraphService
    function setPaymentsDestination(address paymentsDestination_) external override {
        _setPaymentsDestination(msg.sender, paymentsDestination_);
    }

    /// @inheritdoc ISubgraphService
    function setMinimumProvisionTokens(uint256 minimumProvisionTokens) external override onlyOwner {
        _setProvisionTokensRange(minimumProvisionTokens, DEFAULT_MAX_PROVISION_TOKENS);
    }

    /// @inheritdoc ISubgraphService
    function setDelegationRatio(uint32 delegationRatio) external override onlyOwner {
        _setDelegationRatio(delegationRatio);
    }

    /// @inheritdoc ISubgraphService
    function setStakeToFeesRatio(uint256 stakeToFeesRatio_) external override onlyOwner {
        _setStakeToFeesRatio(stakeToFeesRatio_);
    }

    // forge-lint: disable-next-item(mixed-case-function)
    /// @inheritdoc ISubgraphService
    function setMaxPOIStaleness(uint256 maxPoiStaleness_) external override onlyOwner {
        _setMaxPoiStaleness(maxPoiStaleness_);
    }

    /// @inheritdoc ISubgraphService
    function setCurationCut(uint256 curationCut) external override onlyOwner {
        require(PPMMath.isValidPPM(curationCut), SubgraphServiceInvalidCurationCut(curationCut));
        curationFeesCut = curationCut;
        emit CurationCutSet(curationCut);
    }

    /// @inheritdoc ISubgraphService
    function getAllocation(address allocationId) external view override returns (IAllocation.State memory) {
        return _allocations[allocationId];
    }

    /// @inheritdoc IRewardsIssuer
    function getAllocationData(
        address allocationId
    ) external view override returns (bool, address, bytes32, uint256, uint256, uint256) {
        IAllocation.State memory allo = _allocations[allocationId];
        return (
            allo.isOpen(),
            allo.indexer,
            allo.subgraphDeploymentId,
            allo.tokens,
            allo.accRewardsPerAllocatedToken,
            allo.accRewardsPending
        );
    }

    /// @inheritdoc IRewardsIssuer
    function getSubgraphAllocatedTokens(bytes32 subgraphDeploymentId) external view override returns (uint256) {
        return _subgraphAllocatedTokens[subgraphDeploymentId];
    }

    /// @inheritdoc ISubgraphService
    function getLegacyAllocation(address allocationId) external view override returns (ILegacyAllocation.State memory) {
        return _legacyAllocations[allocationId];
    }

    /// @inheritdoc ISubgraphService
    function getDisputeManager() external view override returns (address) {
        return address(_disputeManager());
    }

    /// @inheritdoc ISubgraphService
    function getGraphTallyCollector() external view override returns (address) {
        return address(_graphTallyCollector());
    }

    /// @inheritdoc ISubgraphService
    function getCuration() external view override returns (address) {
        return address(_curation());
    }

    /// @inheritdoc ISubgraphService
    function encodeAllocationProof(address indexer, address allocationId) external view override returns (bytes32) {
        return _encodeAllocationProof(indexer, allocationId);
    }

    /// @inheritdoc ISubgraphService
    function isOverAllocated(address indexer) external view override returns (bool) {
        return _isOverAllocated(indexer, _delegationRatio);
    }

    /**
     * @notice Sets the payments destination for an indexer to receive payments
     * @dev Emits a {PaymentsDestinationSet} event
     * @param _indexer The address of the indexer
     * @param _paymentsDestination The address where payments should be sent
     */
    function _setPaymentsDestination(address _indexer, address _paymentsDestination) internal {
        paymentsDestination[_indexer] = _paymentsDestination;
        emit PaymentsDestinationSet(_indexer, _paymentsDestination);
    }

    // -- Data service parameter getters --
    /**
     * @notice Getter for the accepted thawing period range for provisions
     * The accepted range is just the dispute period defined by {DisputeManager-getDisputePeriod}
     * @dev This override ensures {ProvisionManager} uses the thawing period from the {DisputeManager}
     * @return The minimum thawing period - the dispute period
     * @return The maximum thawing period - the dispute period
     */
    function _getThawingPeriodRange() internal view override returns (uint64, uint64) {
        uint64 disputePeriod = _disputeManager().getDisputePeriod();
        return (disputePeriod, disputePeriod);
    }

    /**
     * @notice Getter for the accepted verifier cut range for provisions
     * @return The minimum verifier cut which is defined by the fisherman reward cut {DisputeManager-getFishermanRewardCut}
     * @return The maximum is 100% in PPM
     */
    function _getVerifierCutRange() internal view override returns (uint32, uint32) {
        return (_disputeManager().getFishermanRewardCut(), DEFAULT_MAX_VERIFIER_CUT);
    }

    /**
     * @notice Checks that an indexer is registered
     * @param indexer The address of the indexer
     */
    function _checkRegisteredIndexer(address indexer) private view {
        require(bytes(indexers[indexer].url).length > 0, SubgraphServiceIndexerNotRegistered(indexer));
    }

    /**
     * @notice Collect query fees
     * Stake equal to the amount being collected times the `stakeToFeesRatio` is locked into a stake claim.
     * This claim can be released at a later stage once expired.
     *
     * It's important to note that before collecting this function will attempt to release any expired stake claims.
     * This could lead to an out of gas error if there are too many expired claims. In that case, the indexer will need to
     * manually release the claims, see {IDataServiceFees-releaseStake}, before attempting to collect again.
     *
     * @dev This function is the equivalent of the legacy `collect` function for query fees.
     * @dev Uses the {GraphTallyCollector} to collect payment from Graph Horizon payments protocol.
     * Fees are distributed to service provider and delegators by {GraphPayments}, though curators
     * share is distributed by this function.
     *
     * Query fees can be collected on closed allocations.
     *
     * Requirements:
     * - Indexer must have enough available tokens to lock as economic security for fees
     *
     * Emits a {StakeClaimsReleased} event, and a {StakeClaimReleased} event for each claim released.
     * Emits a {StakeClaimLocked} event.
     * Emits a {QueryFeesCollected} event.
     *
     * @param _indexer The address of the indexer
     * @param _data Encoded data:
     *    - IGraphTallyCollector.SignedRAV `signedRav`: The signed RAV
     *    - uint256 `tokensToCollect`: The amount of tokens to collect. Allows partially collecting a RAV. If 0, the entire RAV will
     * be collected.
     * @return The amount of fees collected
     */
    // solhint-disable-next-line function-max-lines
    function _collectQueryFees(address _indexer, bytes calldata _data) private returns (uint256) {
        (IGraphTallyCollector.SignedRAV memory signedRav, uint256 tokensToCollect) = abi.decode(
            _data,
            (IGraphTallyCollector.SignedRAV, uint256)
        );
        require(
            signedRav.rav.serviceProvider == _indexer,
            SubgraphServiceIndexerMismatch(signedRav.rav.serviceProvider, _indexer)
        );

        // Check that collectionId (256 bits) is a valid address (160 bits)
        // collectionId is expected to be a zero padded address so it's safe to cast to uint160
        uint256 ravCollectionId = uint256(signedRav.rav.collectionId);
        // solhint-disable-next-line gas-strict-inequalities
        require(ravCollectionId <= type(uint160).max, SubgraphServiceInvalidCollectionId(signedRav.rav.collectionId));
        // forge-lint: disable-next-line(unsafe-typecast)
        address allocationId = address(uint160(ravCollectionId));
        IAllocation.State memory allocation = _allocations.get(allocationId);

        // Check RAV is consistent - RAV indexer must match the allocation's indexer
        require(allocation.indexer == _indexer, SubgraphServiceInvalidRAV(_indexer, allocation.indexer));
        bytes32 subgraphDeploymentId = allocation.subgraphDeploymentId;

        // release expired stake claims
        _releaseStake(_indexer, 0);

        // Collect from GraphPayments - only curators cut is sent back to the subgraph service
        uint256 tokensCollected;
        uint256 tokensCurators;
        {
            uint256 balanceBefore = _graphToken().balanceOf(address(this));

            tokensCollected = _graphTallyCollector().collect(
                IGraphPayments.PaymentTypes.QueryFee,
                _encodeGraphTallyData(signedRav, _curation().isCurated(subgraphDeploymentId) ? curationFeesCut : 0),
                tokensToCollect
            );

            uint256 balanceAfter = _graphToken().balanceOf(address(this));
            // solhint-disable-next-line gas-strict-inequalities
            require(balanceAfter >= balanceBefore, SubgraphServiceInconsistentCollection(balanceBefore, balanceAfter));
            tokensCurators = balanceAfter - balanceBefore;
        }

        if (tokensCollected > 0) {
            // lock stake as economic security for fees
            _lockStake(
                _indexer,
                tokensCollected * stakeToFeesRatio,
                block.timestamp + _disputeManager().getDisputePeriod()
            );

            if (tokensCurators > 0) {
                // curation collection changes subgraph signal so we take rewards snapshot
                _graphRewardsManager().onSubgraphSignalUpdate(subgraphDeploymentId);

                // Send GRT and bookkeep by calling collect()
                _graphToken().pushTokens(address(_curation()), tokensCurators);
                _curation().collect(subgraphDeploymentId, tokensCurators);
            }
        }

        emit QueryFeesCollected(
            _indexer,
            signedRav.rav.payer,
            allocationId,
            subgraphDeploymentId,
            tokensCollected,
            tokensCurators
        );
        return tokensCollected;
    }

    /**
     * @notice Collect indexing rewards
     * @param _indexer The address of the indexer
     * @param _data Encoded data:
     *    - address `allocationId`: The id of the allocation
     *    - bytes32 `poi`: The POI being presented
     *    - bytes `poiMetadata`: The metadata associated with the POI. See {AllocationManager-_presentPOI} for more details.
     * @return The amount of indexing rewards collected
     */
    function _collectIndexingRewards(address _indexer, bytes calldata _data) private returns (uint256) {
        (address allocationId, bytes32 poi_, bytes memory poiMetadata_) = abi.decode(_data, (address, bytes32, bytes));
        require(
            _allocations.get(allocationId).indexer == _indexer,
            SubgraphServiceAllocationNotAuthorized(_indexer, allocationId)
        );
        return _presentPoi(allocationId, poi_, poiMetadata_, _delegationRatio, paymentsDestination[_indexer]);
    }

    /**
     * @notice Set the stake to fees ratio.
     * @param _stakeToFeesRatio The stake to fees ratio
     */
    function _setStakeToFeesRatio(uint256 _stakeToFeesRatio) private {
        require(_stakeToFeesRatio != 0, SubgraphServiceInvalidZeroStakeToFeesRatio());
        stakeToFeesRatio = _stakeToFeesRatio;
        emit StakeToFeesRatioSet(_stakeToFeesRatio);
    }

    /**
     * @notice Encodes the data for the GraphTallyCollector
     * @dev The purpose of this function is just to avoid stack too deep errors
     * @param _signedRav The signed RAV
     * @param _curationCut The curation cut
     * @return The encoded data
     */
    function _encodeGraphTallyData(
        IGraphTallyCollector.SignedRAV memory _signedRav,
        uint256 _curationCut
    ) private view returns (bytes memory) {
        return abi.encode(_signedRav, _curationCut, paymentsDestination[_signedRav.rav.serviceProvider]);
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.33;

import { IGraphPayments } from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import { IHorizonStakingTypes } from "@graphprotocol/interfaces/contracts/horizon/internal/IHorizonStakingTypes.sol";
import { IAllocation } from "@graphprotocol/interfaces/contracts/subgraph-service/internal/IAllocation.sol";
import { IAllocationManager } from "@graphprotocol/interfaces/contracts/subgraph-service/internal/IAllocationManager.sol";
import { ILegacyAllocation } from "@graphprotocol/interfaces/contracts/subgraph-service/internal/ILegacyAllocation.sol";
import { RewardsCondition } from "@graphprotocol/interfaces/contracts/contracts/rewards/RewardsCondition.sol";

import { GraphDirectory } from "@graphprotocol/horizon/contracts/utilities/GraphDirectory.sol";
import { AllocationManagerV1Storage } from "./AllocationManagerStorage.sol";

import { TokenUtils } from "@graphprotocol/contracts/contracts/utils/TokenUtils.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { EIP712Upgradeable } from "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import { Allocation } from "../libraries/Allocation.sol";
import { LegacyAllocation } from "../libraries/LegacyAllocation.sol";
import { PPMMath } from "@graphprotocol/horizon/contracts/libraries/PPMMath.sol";
import { ProvisionTracker } from "@graphprotocol/horizon/contracts/data-service/libraries/ProvisionTracker.sol";

/**
 * @title AllocationManager contract
 * @author Edge & Node
 * @notice A helper contract implementing allocation lifecycle management
 * Allows opening, resizing, and closing allocations, as well as collecting indexing rewards by presenting a Proof
 * of Indexing (POI).
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
abstract contract AllocationManager is
    IAllocationManager,
    EIP712Upgradeable,
    GraphDirectory,
    AllocationManagerV1Storage
{
    using ProvisionTracker for mapping(address => uint256);
    using Allocation for mapping(address => IAllocation.State);
    using Allocation for IAllocation.State;
    using LegacyAllocation for mapping(address => ILegacyAllocation.State);
    using PPMMath for uint256;
    using TokenUtils for IGraphToken;

    ///@dev EIP712 typehash for allocation id proof
    bytes32 private constant EIP712_ALLOCATION_ID_PROOF_TYPEHASH =
        keccak256("AllocationIdProof(address indexer,address allocationId)");
    // solhint-disable-previous-line gas-small-strings

    // forge-lint: disable-next-item(mixed-case-function)
    /**
     * @notice Initializes the contract and parent contracts
     * @param _name The name to use for EIP712 domain separation
     * @param _version The version to use for EIP712 domain separation
     */
    function __AllocationManager_init(string memory _name, string memory _version) internal onlyInitializing {
        __EIP712_init(_name, _version);
        __AllocationManager_init_unchained();
    }

    // forge-lint: disable-next-item(mixed-case-function)
    /**
     * @notice Initializes the contract
     */
    function __AllocationManager_init_unchained() internal onlyInitializing {}

    /**
     * @notice Imports a legacy allocation id into the subgraph service
     * This is a governor only action that is required to prevent indexers from re-using allocation ids from the
     * legacy staking contract. It will revert with LegacyAllocationAlreadyMigrated if the allocation has already been migrated.
     * @param _indexer The address of the indexer
     * @param _allocationId The id of the allocation
     * @param _subgraphDeploymentId The id of the subgraph deployment
     */
    function _migrateLegacyAllocation(address _indexer, address _allocationId, bytes32 _subgraphDeploymentId) internal {
        _legacyAllocations.migrate(_indexer, _allocationId, _subgraphDeploymentId);
        emit LegacyAllocationMigrated(_indexer, _allocationId, _subgraphDeploymentId);
    }

    /**
     * @notice Create an allocation
     * @dev The `_allocationProof` is a 65-bytes Ethereum signed message of `keccak256(indexerAddress,allocationId)`
     *
     * Requirements:
     * - `_allocationId` must not be the zero address
     *
     * Emits a {AllocationCreated} event
     *
     * @param _indexer The address of the indexer
     * @param _allocationId The id of the allocation to be created
     * @param _subgraphDeploymentId The subgraph deployment Id
     * @param _tokens The amount of tokens to allocate
     * @param _allocationProof Signed proof of allocation id address ownership
     * @param _delegationRatio The delegation ratio to consider when locking tokens
     */
    function _allocate(
        address _indexer,
        address _allocationId,
        bytes32 _subgraphDeploymentId,
        uint256 _tokens,
        bytes memory _allocationProof,
        uint32 _delegationRatio
    ) internal {
        require(_allocationId != address(0), AllocationManagerInvalidZeroAllocationId());

        _verifyAllocationProof(_indexer, _allocationId, _allocationProof);

        // Ensure allocation id is not reused
        // need to check both subgraph service (on allocations.create()) and legacy allocations
        _legacyAllocations.revertIfExists(_graphStaking(), _allocationId);

        uint256 currentEpoch = _graphEpochManager().currentEpoch();
        IAllocation.State memory allocation = _allocations.create(
            _indexer,
            _allocationId,
            _subgraphDeploymentId,
            _tokens,
            _graphRewardsManager().onSubgraphAllocationUpdate(_subgraphDeploymentId),
            currentEpoch
        );

        // Check that the indexer has enough tokens available
        // Note that the delegation ratio ensures overdelegation cannot be used
        allocationProvisionTracker.lock(_graphStaking(), _indexer, _tokens, _delegationRatio);

        // Update total allocated tokens for the subgraph deployment
        _subgraphAllocatedTokens[allocation.subgraphDeploymentId] =
            _subgraphAllocatedTokens[allocation.subgraphDeploymentId] + allocation.tokens;

        emit AllocationCreated(_indexer, _allocationId, _subgraphDeploymentId, allocation.tokens, currentEpoch);
    }

    /**
     * @notice Present a POI to collect indexing rewards for an allocation
     * Mints indexing rewards using the {RewardsManager} and distributes them to the indexer and delegators.
     *
     * Requirements for indexing rewards:
     * - POI must be non-zero
     * - POI must not be stale (older than `maxPOIStaleness`)
     * - Allocation must be open for at least one epoch (returns early with 0 if too young)
     *
     * ## Reward Paths
     *
     * Rewards follow one of three paths based on allocation and POI state:
     *
     * **CLAIMED** (normal path): Valid POI, not stale, allocation mature, subgraph not denied
     * - Calls `takeRewards()` to mint tokens to this contract
     * - Distributes to indexer (stake or payments destination) and delegators
     * - Snapshots allocation to prevent double-counting
     *
     * **RECLAIMED** (redirect path): STALE_POI or ZERO_POI conditions
     * - Calls `reclaimRewards()` to mint tokens to configured reclaim address
     * - If no reclaim address configured, rewards are dropped (not minted)
     * - Snapshots allocation to prevent double-counting
     *
     * **DEFERRED** (early return): ALLOCATION_TOO_YOUNG or SUBGRAPH_DENIED conditions
     * - Returns 0 without calling take or reclaim
     * - Does NOT snapshot allocation (preserves rewards for later collection)
     * - Allows rewards to be claimed when condition clears
     *
     * ## Subgraph Denial (Soft Deny)
     *
     * When a subgraph is denied, this function implements "soft deny":
     * - Returns early without claiming or reclaiming
     * - Allocation state is preserved (pending rewards not cleared)
     * - Pre-denial rewards remain claimable after undeny
     * - Ongoing issuance during denial is reclaimed at RewardsManager level (hard deny)
     *
     * Note: Indexers should present POIs at least every `maxPOIStaleness` to avoid being locked out of rewards.
     * A zero POI can be presented if a valid one is unavailable, to prevent staleness and slashing.
     *
     * Note: Reclaim address changes in RewardsManager apply retroactively to all unclaimed rewards.
     *
     * Emits a {IndexingRewardsCollected} event.
     *
     * @param _allocationId The id of the allocation to collect rewards for
     * @param _poi The POI being presented
     * @param _poiMetadata Metadata associated with the POI, emitted as-is for off-chain components
     * @param _delegationRatio The delegation ratio to consider when locking tokens
     * @param _paymentsDestination The address where indexing rewards should be sent
     * @return rewardsCollected Indexing rewards collected
     */
    // solhint-disable-next-line function-max-lines
    function _presentPoi(
        address _allocationId,
        bytes32 _poi,
        bytes memory _poiMetadata,
        uint32 _delegationRatio,
        address _paymentsDestination
    ) internal returns (uint256 rewardsCollected) {
        IAllocation.State memory allocation = _allocations.get(_allocationId);
        require(allocation.isOpen(), AllocationManagerAllocationClosed(_allocationId));
        _allocations.presentPOI(_allocationId); // Always record POI presentation to prevent staleness

        uint256 currentEpoch = _graphEpochManager().currentEpoch();
        // Scoped for stack management
        {
            // Determine rewards condition
            bytes32 condition = RewardsCondition.NONE;
            if (allocation.isStale(maxPOIStaleness)) condition = RewardsCondition.STALE_POI;
            else if (_poi == bytes32(0))
                condition = RewardsCondition.ZERO_POI;
                // solhint-disable-next-line gas-strict-inequalities
            else if (currentEpoch <= allocation.createdAtEpoch) condition = RewardsCondition.ALLOCATION_TOO_YOUNG;
            else if (_graphRewardsManager().isDenied(allocation.subgraphDeploymentId))
                condition = RewardsCondition.SUBGRAPH_DENIED;

            emit POIPresented(
                allocation.indexer,
                _allocationId,
                allocation.subgraphDeploymentId,
                _poi,
                _poiMetadata,
                condition
            );

            // Early return skips the overallocation check intentionally to avoid loss of uncollected rewards
            if (condition == RewardsCondition.ALLOCATION_TOO_YOUNG || condition == RewardsCondition.SUBGRAPH_DENIED) {
                // Keep reward and reclaim accumulation current even if rewards are not collected
                _graphRewardsManager().onSubgraphAllocationUpdate(allocation.subgraphDeploymentId);

                return 0;
            }

            bool rewardsReclaimable = condition == RewardsCondition.STALE_POI || condition == RewardsCondition.ZERO_POI;
            if (rewardsReclaimable) _graphRewardsManager().reclaimRewards(condition, _allocationId);
            else rewardsCollected = _graphRewardsManager().takeRewards(_allocationId);
        }

        // Snapshot rewards to prevent accumulation for next POI, then clear pending
        _allocations.snapshotRewards(
            _allocationId,
            _graphRewardsManager().onSubgraphAllocationUpdate(allocation.subgraphDeploymentId)
        );
        _allocations.clearPendingRewards(_allocationId);

        // Scoped for stack management
        {
            (uint256 tokensIndexerRewards, uint256 tokensDelegationRewards) = _distributeIndexingRewards(
                allocation,
                rewardsCollected,
                _paymentsDestination
            );

            emit IndexingRewardsCollected(
                allocation.indexer,
                _allocationId,
                allocation.subgraphDeploymentId,
                rewardsCollected,
                tokensIndexerRewards,
                tokensDelegationRewards,
                _poi,
                _poiMetadata,
                currentEpoch
            );
        }

        if (_isOverAllocated(allocation.indexer, _delegationRatio)) _closeAllocation(_allocationId, true);
    }

    /**
     * @notice Resize an allocation
     * @dev Will lock or release tokens in the provision tracker depending on the new allocation size.
     * Rewards accrued but not issued before the resize will be accounted for as pending rewards,
     * unless the allocation is stale, in which case pending rewards are reclaimed.
     * These will be paid out when the indexer presents a POI.
     *
     * Requirements:
     * - `_indexer` must be the owner of the allocation
     * - Allocation must be open
     * - `_tokens` must be different from the current allocation size
     *
     * Emits a {AllocationResized} event.
     *
     * @param _allocationId The id of the allocation to be resized
     * @param _tokens The new amount of tokens to allocate
     * @param _delegationRatio The delegation ratio to consider when locking tokens
     */
    function _resizeAllocation(address _allocationId, uint256 _tokens, uint32 _delegationRatio) internal {
        IAllocation.State memory allocation = _allocations.get(_allocationId);
        require(allocation.isOpen(), AllocationManagerAllocationClosed(_allocationId));
        require(_tokens != allocation.tokens, AllocationManagerAllocationSameSize(_allocationId, _tokens));

        // Update provision tracker
        uint256 oldTokens = allocation.tokens;
        if (_tokens > oldTokens) {
            allocationProvisionTracker.lock(_graphStaking(), allocation.indexer, _tokens - oldTokens, _delegationRatio);
        } else {
            allocationProvisionTracker.release(allocation.indexer, oldTokens - _tokens);
        }

        // Calculate rewards that have been accrued since the last snapshot but not yet issued
        uint256 accRewardsPerAllocatedToken = _graphRewardsManager().onSubgraphAllocationUpdate(
            allocation.subgraphDeploymentId
        );
        uint256 accRewardsPerAllocatedTokenPending = !allocation.isAltruistic()
            ? accRewardsPerAllocatedToken - allocation.accRewardsPerAllocatedToken
            : 0;

        // Update the allocation
        _allocations[_allocationId].tokens = _tokens;
        _allocations[_allocationId].accRewardsPerAllocatedToken = accRewardsPerAllocatedToken;
        _allocations[_allocationId].accRewardsPending += _graphRewardsManager().calcRewards(
            oldTokens,
            accRewardsPerAllocatedTokenPending
        );

        // If allocation is stale, reclaim pending rewards defensively.
        // Stale allocations are not performing, so rewards should not accumulate.
        if (allocation.isStale(maxPOIStaleness)) {
            _graphRewardsManager().reclaimRewards(RewardsCondition.STALE_POI, _allocationId);
            _allocations.clearPendingRewards(_allocationId);
        }

        // Update total allocated tokens for the subgraph deployment
        if (_tokens > oldTokens) {
            _subgraphAllocatedTokens[allocation.subgraphDeploymentId] += (_tokens - oldTokens);
        } else {
            _subgraphAllocatedTokens[allocation.subgraphDeploymentId] -= (oldTokens - _tokens);
        }

        emit AllocationResized(allocation.indexer, _allocationId, allocation.subgraphDeploymentId, _tokens, oldTokens);
    }

    /**
     * @notice Close an allocation
     * Does not require presenting a POI, use {_collectIndexingRewards} to present a POI and collect rewards
     * @dev Allocations are long-lived. All service payments, including indexing rewards, should be collected
     * periodically without closing. Allocations should only be closed when indexers want to reclaim tokens.
     *
     * ## Reward Handling on Close
     *
     * Uncollected rewards are reclaimed with CLOSE_ALLOCATION reason:
     * - If reclaim address configured: tokens minted to that address
     * - If no reclaim address: rewards are dropped (not minted anywhere)
     *
     * ## Known Limitation
     *
     * `clearPendingRewards()` is only called when `0 < reclaimedRewards`. This means:
     * - If no reclaim address is configured, `accRewardsPending` may remain non-zero
     *
     * Emits a {AllocationClosed} event
     *
     * @param _allocationId The id of the allocation to be closed
     * @param _forceClosed Whether the allocation was force closed
     */
    function _closeAllocation(address _allocationId, bool _forceClosed) internal {
        IAllocation.State memory allocation = _allocations.get(_allocationId);

        // Reclaim uncollected rewards before closing
        uint256 reclaimedRewards = _graphRewardsManager().reclaimRewards(
            RewardsCondition.CLOSE_ALLOCATION,
            _allocationId
        );

        // Take rewards snapshot to prevent other allos from counting tokens from this allo
        _allocations.snapshotRewards(
            _allocationId,
            _graphRewardsManager().onSubgraphAllocationUpdate(allocation.subgraphDeploymentId)
        );

        // Clear pending rewards only if rewards were reclaimed. This marks them as consumed,
        // which could be useful for future logic that searches for unconsumed rewards.
        // Known limitation: This capture is incomplete due to other code paths (e.g., _presentPOI)
        // that clear pending even when rewards are not consumed.
        if (0 < reclaimedRewards) _allocations.clearPendingRewards(_allocationId);

        _allocations.close(_allocationId);
        allocationProvisionTracker.release(allocation.indexer, allocation.tokens);

        // Update total allocated tokens for the subgraph deployment
        _subgraphAllocatedTokens[allocation.subgraphDeploymentId] =
            _subgraphAllocatedTokens[allocation.subgraphDeploymentId] - allocation.tokens;

        emit AllocationClosed(
            allocation.indexer,
            _allocationId,
            allocation.subgraphDeploymentId,
            allocation.tokens,
            _forceClosed
        );
    }

    /**
     * @notice Sets the maximum amount of time, in seconds, allowed between presenting POIs to qualify for indexing rewards
     * @dev Emits a {MaxPOIStalenessSet} event
     * @param _maxPoiStaleness The max POI staleness in seconds
     */
    function _setMaxPoiStaleness(uint256 _maxPoiStaleness) internal {
        maxPOIStaleness = _maxPoiStaleness;
        emit MaxPOIStalenessSet(_maxPoiStaleness);
    }

    /**
     * @notice Encodes the allocation proof for EIP712 signing
     * @param _indexer The address of the indexer
     * @param _allocationId The id of the allocation
     * @return The encoded allocation proof
     */
    function _encodeAllocationProof(address _indexer, address _allocationId) internal view returns (bytes32) {
        return _hashTypedDataV4(keccak256(abi.encode(EIP712_ALLOCATION_ID_PROOF_TYPEHASH, _indexer, _allocationId)));
    }

    /**
     * @notice Checks if an allocation is over-allocated
     * @param _indexer The address of the indexer
     * @param _delegationRatio The delegation ratio to consider when locking tokens
     * @return True if the allocation is over-allocated, false otherwise
     */
    function _isOverAllocated(address _indexer, uint32 _delegationRatio) internal view returns (bool) {
        return !allocationProvisionTracker.check(_graphStaking(), _indexer, _delegationRatio);
    }

    /**
     * @notice Distributes indexing rewards to delegators and indexer
     * @param _allocation The allocation state
     * @param _rewardsCollected Total rewards to distribute
     * @param _paymentsDestination Where to send indexer rewards (0 = stake)
     * @return tokensIndexerRewards Amount sent to indexer
     * @return tokensDelegationRewards Amount sent to delegation pool
     */
    function _distributeIndexingRewards(
        IAllocation.State memory _allocation,
        uint256 _rewardsCollected,
        address _paymentsDestination
    ) private returns (uint256 tokensIndexerRewards, uint256 tokensDelegationRewards) {
        if (_rewardsCollected == 0) return (0, 0);

        // Calculate and distribute delegator share
        uint256 delegatorCut = _graphStaking().getDelegationFeeCut(
            _allocation.indexer,
            address(this),
            IGraphPayments.PaymentTypes.IndexingRewards
        );
        IHorizonStakingTypes.DelegationPool memory pool = _graphStaking().getDelegationPool(
            _allocation.indexer,
            address(this)
        );
        tokensDelegationRewards = pool.shares > 0 ? _rewardsCollected.mulPPM(delegatorCut) : 0;
        if (tokensDelegationRewards > 0) {
            _graphToken().approve(address(_graphStaking()), tokensDelegationRewards);
            _graphStaking().addToDelegationPool(_allocation.indexer, address(this), tokensDelegationRewards);
        }

        // Distribute indexer share
        tokensIndexerRewards = _rewardsCollected - tokensDelegationRewards;
        if (tokensIndexerRewards > 0) {
            if (_paymentsDestination == address(0)) {
                _graphToken().approve(address(_graphStaking()), tokensIndexerRewards);
                _graphStaking().stakeToProvision(_allocation.indexer, address(this), tokensIndexerRewards);
            } else {
                _graphToken().pushTokens(_paymentsDestination, tokensIndexerRewards);
            }
        }
    }

    /**
     * @notice Verifies ownership of an allocation id by verifying an EIP712 allocation proof
     * @dev Requirements:
     * - Signer must be the allocation id address
     * @param _indexer The address of the indexer
     * @param _allocationId The id of the allocation
     * @param _proof The EIP712 proof, an EIP712 signed message of (indexer,allocationId)
     */
    function _verifyAllocationProof(address _indexer, address _allocationId, bytes memory _proof) private view {
        address signer = ECDSA.recover(_encodeAllocationProof(_indexer, _allocationId), _proof);
        require(signer == _allocationId, AllocationManagerInvalidAllocationProof(signer, _allocationId));
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.33;

import { ISubgraphService } from "@graphprotocol/interfaces/contracts/subgraph-service/ISubgraphService.sol";

/**
 * @title SubgraphServiceStorage
 * @author Edge & Node
 * @notice This contract holds all the storage variables for the Subgraph Service contract
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
abstract contract SubgraphServiceV1Storage is ISubgraphService {
    /// @notice Service providers registered in the data service
    mapping(address indexer => ISubgraphService.Indexer details) public override indexers;

    ///@notice Multiplier for how many tokens back collected query fees
    uint256 public override stakeToFeesRatio;

    /// @notice The cut curators take from query fee payments. In PPM.
    uint256 public override curationFeesCut;

    /// @notice Destination of indexer payments
    mapping(address indexer => address destination) public override paymentsDestination;
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-indexed-events
// forge-lint: disable-start(unwrapped-modifier-logic)

import { IDisputeManager } from "@graphprotocol/interfaces/contracts/subgraph-service/IDisputeManager.sol";
import { ISubgraphService } from "@graphprotocol/interfaces/contracts/subgraph-service/ISubgraphService.sol";
import { IGraphTallyCollector } from "@graphprotocol/interfaces/contracts/horizon/IGraphTallyCollector.sol";
import { ICuration } from "@graphprotocol/interfaces/contracts/contracts/curation/ICuration.sol";

/**
 * @title Directory contract
 * @author Edge & Node
 * @notice This contract is meant to be inherited by {SubgraphService} contract
 * It contains the addresses of the contracts that the contract interacts with.
 * Uses immutable variables to minimize gas costs.
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
abstract contract Directory {
    /// @notice The Subgraph Service contract address
    ISubgraphService private immutable SUBGRAPH_SERVICE;

    /// @notice The Dispute Manager contract address
    IDisputeManager private immutable DISPUTE_MANAGER;

    /// @notice The Graph Tally Collector contract address
    /// @dev Required to collect payments via Graph Horizon payments protocol
    IGraphTallyCollector private immutable GRAPH_TALLY_COLLECTOR;

    /// @notice The Curation contract address
    /// @dev Required for curation fees distribution
    ICuration private immutable CURATION;

    /**
     * @notice Emitted when the Directory is initialized
     * @param subgraphService The Subgraph Service contract address
     * @param disputeManager The Dispute Manager contract address
     * @param graphTallyCollector The Graph Tally Collector contract address
     * @param curation The Curation contract address
     */
    event SubgraphServiceDirectoryInitialized(
        address subgraphService,
        address disputeManager,
        address graphTallyCollector,
        address curation
    );

    /**
     * @notice Thrown when the caller is not the Dispute Manager
     * @param caller The caller address
     * @param disputeManager The Dispute Manager address
     */
    error DirectoryNotDisputeManager(address caller, address disputeManager);

    /**
     * @notice Checks that the caller is the Dispute Manager
     */
    modifier onlyDisputeManager() {
        require(
            msg.sender == address(DISPUTE_MANAGER),
            DirectoryNotDisputeManager(msg.sender, address(DISPUTE_MANAGER))
        );
        _;
    }

    /**
     * @notice Constructor for the Directory contract
     * @param subgraphService The Subgraph Service contract address
     * @param disputeManager The Dispute Manager contract address
     * @param graphTallyCollector The Graph Tally Collector contract address
     * @param curation The Curation contract address
     */
    constructor(address subgraphService, address disputeManager, address graphTallyCollector, address curation) {
        SUBGRAPH_SERVICE = ISubgraphService(subgraphService);
        DISPUTE_MANAGER = IDisputeManager(disputeManager);
        GRAPH_TALLY_COLLECTOR = IGraphTallyCollector(graphTallyCollector);
        CURATION = ICuration(curation);

        emit SubgraphServiceDirectoryInitialized(subgraphService, disputeManager, graphTallyCollector, curation);
    }

    /**
     * @notice Returns the Subgraph Service contract address
     * @return The Subgraph Service contract
     */
    function _subgraphService() internal view returns (ISubgraphService) {
        return SUBGRAPH_SERVICE;
    }

    /**
     * @notice Returns the Dispute Manager contract address
     * @return The Dispute Manager contract
     */
    function _disputeManager() internal view returns (IDisputeManager) {
        return DISPUTE_MANAGER;
    }

    /**
     * @notice Returns the Graph Tally Collector contract address
     * @return The Graph Tally Collector contract
     */
    function _graphTallyCollector() internal view returns (IGraphTallyCollector) {
        return GRAPH_TALLY_COLLECTOR;
    }

    /**
     * @notice Returns the Curation contract address
     * @return The Curation contract
     */
    function _curation() internal view returns (ICuration) {
        return CURATION;
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.33;

import { IAllocation } from "@graphprotocol/interfaces/contracts/subgraph-service/internal/IAllocation.sol";
import { IAllocationManager } from "@graphprotocol/interfaces/contracts/subgraph-service/internal/IAllocationManager.sol";
import { ILegacyAllocation } from "@graphprotocol/interfaces/contracts/subgraph-service/internal/ILegacyAllocation.sol";

/**
 * @title AllocationManagerStorage
 * @author Edge & Node
 * @notice This contract holds all the storage variables for the Allocation Manager contract
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
abstract contract AllocationManagerV1Storage is IAllocationManager {
    /// @notice Allocation details
    mapping(address allocationId => IAllocation.State allocation) internal _allocations;

    /// @notice Legacy allocation details
    mapping(address allocationId => ILegacyAllocation.State allocation) internal _legacyAllocations;

    /// @notice Tracks allocated tokens per indexer
    mapping(address indexer => uint256 tokens) public override allocationProvisionTracker;

    // forge-lint: disable-next-item(mixed-case-variable)
    /// @notice Maximum amount of time, in seconds, allowed between presenting POIs to qualify for indexing rewards
    uint256 public override maxPOIStaleness;

    /// @notice Track total tokens allocated per subgraph deployment
    /// @dev Used to calculate indexing rewards
    mapping(bytes32 subgraphDeploymentId => uint256 tokens) internal _subgraphAllocatedTokens;

    // forge-lint: disable-next-item(mixed-case-variable)
    /// @dev Gap to allow adding variables in future upgrades
    uint256[50] private __gap;
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// forge-lint: disable-start(mixed-case-variable, mixed-case-function)

import { IAllocation } from "@graphprotocol/interfaces/contracts/subgraph-service/internal/IAllocation.sol";

import { Math } from "@openzeppelin/contracts/utils/math/Math.sol";

/**
 * @title Allocation library
 * @author Edge & Node
 * @notice A library to handle Allocations
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
library Allocation {
    using Allocation for IAllocation.State;

    /**
     * @notice Create a new allocation
     * @dev Requirements:
     * - The allocation must not exist
     * @param self The allocation list mapping
     * @param indexer The indexer that owns the allocation
     * @param allocationId The allocation id
     * @param subgraphDeploymentId The subgraph deployment id the allocation is for
     * @param tokens The number of tokens allocated
     * @param accRewardsPerAllocatedToken The initial accumulated rewards per allocated token
     * @param createdAtEpoch The epoch when the allocation was created
     * @return The allocation
     */
    function create(
        mapping(address => IAllocation.State) storage self,
        address indexer,
        address allocationId,
        bytes32 subgraphDeploymentId,
        uint256 tokens,
        uint256 accRewardsPerAllocatedToken,
        uint256 createdAtEpoch
    ) internal returns (IAllocation.State memory) {
        require(!self[allocationId].exists(), IAllocation.AllocationAlreadyExists(allocationId));

        IAllocation.State memory allocation = IAllocation.State({
            indexer: indexer,
            subgraphDeploymentId: subgraphDeploymentId,
            tokens: tokens,
            createdAt: block.timestamp,
            closedAt: 0,
            lastPOIPresentedAt: 0,
            accRewardsPerAllocatedToken: accRewardsPerAllocatedToken,
            accRewardsPending: 0,
            createdAtEpoch: createdAtEpoch
        });

        self[allocationId] = allocation;

        return allocation;
    }

    /**
     * @notice Present a POI for an allocation
     * @dev It only updates the last POI presented timestamp.
     * Requirements:
     * - The allocation must be open
     * @param self The allocation list mapping
     * @param allocationId The allocation id
     */
    function presentPOI(mapping(address => IAllocation.State) storage self, address allocationId) internal {
        IAllocation.State storage allocation = _get(self, allocationId);
        require(allocation.isOpen(), IAllocation.AllocationClosed(allocationId, allocation.closedAt));
        allocation.lastPOIPresentedAt = block.timestamp;
    }

    /**
     * @notice Update the accumulated rewards per allocated token for an allocation
     * @dev Requirements:
     * - The allocation must be open
     * @param self The allocation list mapping
     * @param allocationId The allocation id
     * @param accRewardsPerAllocatedToken The new accumulated rewards per allocated token
     */
    function snapshotRewards(
        mapping(address => IAllocation.State) storage self,
        address allocationId,
        uint256 accRewardsPerAllocatedToken
    ) internal {
        IAllocation.State storage allocation = _get(self, allocationId);
        require(allocation.isOpen(), IAllocation.AllocationClosed(allocationId, allocation.closedAt));
        allocation.accRewardsPerAllocatedToken = accRewardsPerAllocatedToken;
    }

    /**
     * @notice Update the accumulated rewards pending to be claimed for an allocation
     * @dev Requirements:
     * - The allocation must be open
     * @param self The allocation list mapping
     * @param allocationId The allocation id
     */
    function clearPendingRewards(mapping(address => IAllocation.State) storage self, address allocationId) internal {
        IAllocation.State storage allocation = _get(self, allocationId);
        require(allocation.isOpen(), IAllocation.AllocationClosed(allocationId, allocation.closedAt));
        allocation.accRewardsPending = 0;
    }

    /**
     * @notice Close an allocation
     * @dev Requirements:
     * - The allocation must be open
     * @param self The allocation list mapping
     * @param allocationId The allocation id
     */
    function close(mapping(address => IAllocation.State) storage self, address allocationId) internal {
        IAllocation.State storage allocation = _get(self, allocationId);
        require(allocation.isOpen(), IAllocation.AllocationClosed(allocationId, allocation.closedAt));
        allocation.closedAt = block.timestamp;
    }

    /**
     * @notice Get an allocation
     * @param self The allocation list mapping
     * @param allocationId The allocation id
     * @return The allocation
     */
    function get(
        mapping(address => IAllocation.State) storage self,
        address allocationId
    ) internal view returns (IAllocation.State memory) {
        return _get(self, allocationId);
    }

    /**
     * @notice Checks if an allocation is stale
     * @param self The allocation
     * @param staleThreshold The time in blocks to consider an allocation stale
     * @return True if the allocation is stale
     */
    function isStale(IAllocation.State memory self, uint256 staleThreshold) internal view returns (bool) {
        uint256 timeSinceLastPOI = block.timestamp - Math.max(self.createdAt, self.lastPOIPresentedAt);
        return self.isOpen() && timeSinceLastPOI > staleThreshold;
    }

    /**
     * @notice Checks if an allocation exists
     * @param self The allocation
     * @return True if the allocation exists
     */
    function exists(IAllocation.State memory self) internal pure returns (bool) {
        return self.createdAt != 0;
    }

    /**
     * @notice Checks if an allocation is open
     * @param self The allocation
     * @return True if the allocation is open
     */
    function isOpen(IAllocation.State memory self) internal pure returns (bool) {
        return self.exists() && self.closedAt == 0;
    }

    /**
     * @notice Checks if an allocation is alturistic
     * @param self The allocation
     * @return True if the allocation is alturistic
     */
    function isAltruistic(IAllocation.State memory self) internal pure returns (bool) {
        return self.exists() && self.tokens == 0;
    }

    /**
     * @notice Get the allocation for an allocation id
     * @dev Reverts if the allocation does not exist
     * @param self The allocation list mapping
     * @param allocationId The allocation id
     * @return The allocation
     */
    function _get(
        mapping(address => IAllocation.State) storage self,
        address allocationId
    ) private view returns (IAllocation.State storage) {
        IAllocation.State storage allocation = self[allocationId];
        require(allocation.exists(), IAllocation.AllocationDoesNotExist(allocationId));
        return allocation;
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.33;

import { IHorizonStaking } from "@graphprotocol/interfaces/contracts/horizon/IHorizonStaking.sol";
import { ILegacyAllocation } from "@graphprotocol/interfaces/contracts/subgraph-service/internal/ILegacyAllocation.sol";

/**
 * @title LegacyAllocation library
 * @author Edge & Node
 * @notice A library to handle legacy Allocations
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
library LegacyAllocation {
    using LegacyAllocation for ILegacyAllocation.State;

    /**
     * @notice Migrate a legacy allocation
     * @dev Requirements:
     * - The allocation must not have been previously migrated
     * @param self The legacy allocation list mapping
     * @param indexer The indexer that owns the allocation
     * @param allocationId The allocation id
     * @param subgraphDeploymentId The subgraph deployment id the allocation is for
     * @custom:error LegacyAllocationAlreadyMigrated if the allocation has already been migrated
     */
    function migrate(
        mapping(address => ILegacyAllocation.State) storage self,
        address indexer,
        address allocationId,
        bytes32 subgraphDeploymentId
    ) internal {
        require(!self[allocationId].exists(), ILegacyAllocation.LegacyAllocationAlreadyExists(allocationId));

        self[allocationId] = ILegacyAllocation.State({ indexer: indexer, subgraphDeploymentId: subgraphDeploymentId });
    }

    /**
     * @notice Get a legacy allocation
     * @param self The legacy allocation list mapping
     * @param allocationId The allocation id
     * @return The legacy allocation details
     */
    function get(
        mapping(address => ILegacyAllocation.State) storage self,
        address allocationId
    ) internal view returns (ILegacyAllocation.State memory) {
        return _get(self, allocationId);
    }

    /**
     * @notice Revert if a legacy allocation exists
     * @dev We first check the migrated mapping then the old staking contract.
     * @dev TRANSITION PERIOD: after the transition period when all the allocations are migrated we can
     * remove the call to the staking contract.
     * @param self The legacy allocation list mapping
     * @param graphStaking The Horizon Staking contract
     * @param allocationId The allocation id
     */
    function revertIfExists(
        mapping(address => ILegacyAllocation.State) storage self,
        IHorizonStaking graphStaking,
        address allocationId
    ) internal view {
        require(!self[allocationId].exists(), ILegacyAllocation.LegacyAllocationAlreadyExists(allocationId));
        require(
            !graphStaking.isAllocation(allocationId),
            ILegacyAllocation.LegacyAllocationAlreadyExists(allocationId)
        );
    }

    /**
     * @notice Check if a legacy allocation exists
     * @param self The legacy allocation
     * @return True if the allocation exists
     */
    function exists(ILegacyAllocation.State memory self) internal pure returns (bool) {
        return self.indexer != address(0);
    }

    /**
     * @notice Get a legacy allocation
     * @param self The legacy allocation list mapping
     * @param allocationId The allocation id
     * @return The legacy allocation details
     */
    function _get(
        mapping(address => ILegacyAllocation.State) storage self,
        address allocationId
    ) private view returns (ILegacyAllocation.State storage) {
        ILegacyAllocation.State storage allocation = self[allocationId];
        require(allocation.exists(), ILegacyAllocation.LegacyAllocationDoesNotExist(allocationId));
        return allocation;
    }
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

