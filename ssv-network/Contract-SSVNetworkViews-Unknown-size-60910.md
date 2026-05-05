
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import "./interfaces/ISSVViews.sol";
import "./libraries/ClusterLib.sol";
import "./libraries/OperatorLib.sol";
import "./libraries/ProtocolLib.sol";
import {MAX_DELEGATION_SLOTS} from "./libraries/storage/SSVStorageStaking.sol";

import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";

contract SSVNetworkViews is UUPSUpgradeable, Ownable2StepUpgradeable, ISSVViews {
    using ClusterLib for Cluster;
    using OperatorLib for Operator;

    ISSVViews public ssvNetwork;

    // @dev reserve storage space for future new state variables in base contract
    // slither-disable-next-line shadowing-state
    uint256[50] private __gap;

    function _authorizeUpgrade(address) internal override onlyOwner {}

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(ISSVViews ssvNetwork_) external initializer onlyProxy {
        __UUPSUpgradeable_init();
        __Ownable2Step_init();
        ssvNetwork = ssvNetwork_;
    }

    /*************************************/
    /* Validator External View Functions */
    /*************************************/

    function getValidator(address clusterOwner, bytes calldata publicKey) external view override returns (bool) {
        return ssvNetwork.getValidator(clusterOwner, publicKey);
    }

    /************************************/
    /* Operator External View Functions */
    /************************************/

    function getOperatorFee(uint64 operatorId) external view override returns (uint256) {
        return ssvNetwork.getOperatorFee(operatorId);
    }

    function getOperatorFeeSSV(uint64 operatorId) external view override returns (uint256) {
        return ssvNetwork.getOperatorFeeSSV(operatorId);
    }

    function getOperatorDeclaredFee(uint64 operatorId) external view override returns (OperatorDeclaredFeeData memory) {
        return ssvNetwork.getOperatorDeclaredFee(operatorId);
    }

    function getOperatorById(
        uint64 operatorId
    ) external view override returns (OperatorData memory) {
        return ssvNetwork.getOperatorById(operatorId);
    }

    function getOperatorByIdSSV(
        uint64 operatorId
    ) external view override returns (OperatorData memory) {
        return ssvNetwork.getOperatorByIdSSV(operatorId);
    }

    function getWhitelistedOperators(
        uint64[] calldata operatorIds,
        address whitelistedAddress
    ) external view override returns (uint64[] memory whitelistedOperatorIds) {
        return ssvNetwork.getWhitelistedOperators(operatorIds, whitelistedAddress);
    }

    function isWhitelistingContract(address contractAddress) external view override returns (bool) {
        return ssvNetwork.isWhitelistingContract(contractAddress);
    }

    function isAddressWhitelistedInWhitelistingContract(
        address addressToCheck,
        uint256 operatorId,
        address whitelistingContract
    ) external view override returns (bool isWhitelisted) {
        return ssvNetwork.isAddressWhitelistedInWhitelistingContract(addressToCheck, operatorId, whitelistingContract);
    }

    /***********************************/
    /* Cluster External View Functions */
    /***********************************/

    function isLiquidatable(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view override returns (bool) {
        return ssvNetwork.isLiquidatable(clusterOwner, operatorIds, cluster);
    }

    function isLiquidatableSSV(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view override returns (bool) {
        return ssvNetwork.isLiquidatableSSV(clusterOwner, operatorIds, cluster);
    }

    function isLiquidated(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view override returns (bool) {
        return ssvNetwork.isLiquidated(clusterOwner, operatorIds, cluster);
    }

    function getBurnRate(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view override returns (uint256) {
        return ssvNetwork.getBurnRate(clusterOwner, operatorIds, cluster);
    }

    function getBurnRateSSV(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view override returns (uint256) {
        return ssvNetwork.getBurnRateSSV(clusterOwner, operatorIds, cluster);
    }

    /***********************************/
    /* Balance External View Functions */
    /***********************************/

    function getOperatorEarnings(uint64 id) external view override returns (uint256) {
        return ssvNetwork.getOperatorEarnings(id);
    }

    function getOperatorEarningsSSV(uint64 id) external view override returns (uint256) {
        return ssvNetwork.getOperatorEarningsSSV(id);
    }

    function getBalance(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view override returns (uint256 balance) {
        return ssvNetwork.getBalance(clusterOwner, operatorIds, cluster);
    }

    function getBalanceSSV(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view override returns (uint256 balance) {
        return ssvNetwork.getBalanceSSV(clusterOwner, operatorIds, cluster);
    }

    function getEffectiveBalance(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view returns (uint32 effectiveBalance) {
        return ssvNetwork.getEffectiveBalance(clusterOwner, operatorIds, cluster);
    }

    /*******************************/
    /* DAO External View Functions */
    /*******************************/

    function getNetworkFee() external view override returns (uint256) {
        return ssvNetwork.getNetworkFee();
    }

    function getNetworkEarnings() external view override returns (uint256) {
        return ssvNetwork.getNetworkEarnings();
    }

    function getNetworkFeeSSV() external view override returns (uint256) {
        return ssvNetwork.getNetworkFeeSSV();
    }

    function getNetworkEarningsSSV() external view override returns (uint256) {
        return ssvNetwork.getNetworkEarningsSSV();
    }

    function getOperatorFeeIncreaseLimit() external view override returns (uint64) {
        return ssvNetwork.getOperatorFeeIncreaseLimit();
    }

    function getMaximumOperatorFee() external view override returns (uint256) {
        return ssvNetwork.getMaximumOperatorFee();
    }

    function getMaximumOperatorFeeSSV() external view override returns (uint256) {
        return ssvNetwork.getMaximumOperatorFeeSSV();
    }

    function getMinimumOperatorEthFee() external view override returns (uint256) {
        return ssvNetwork.getMinimumOperatorEthFee();
    }

    function getOperatorFeePeriods() external view override returns (OperatorFeePeriodsData memory) {
        return ssvNetwork.getOperatorFeePeriods();
    }

    function getLiquidationThresholdPeriod() external view override returns (uint64) {
        return ssvNetwork.getLiquidationThresholdPeriod();
    }

    function getLiquidationThresholdPeriodSSV() external view override returns (uint64) {
        return ssvNetwork.getLiquidationThresholdPeriodSSV();
    }

    function getMinimumLiquidationCollateral() external view override returns (uint256) {
        return ssvNetwork.getMinimumLiquidationCollateral();
    }

    function getMinimumLiquidationCollateralSSV() external view override returns (uint256) {
        return ssvNetwork.getMinimumLiquidationCollateralSSV();
    }

    function getValidatorsPerOperatorLimit() external view override returns (uint32) {
        return ssvNetwork.getValidatorsPerOperatorLimit();
    }

    function getNetworkValidatorsCount() external view override returns (uint32) {
        return ssvNetwork.getNetworkValidatorsCount();
    }

    function getClusterAssetType(address owner, uint64[] calldata operatorIds) external view override returns (uint8) {
        return ssvNetwork.getClusterAssetType(owner, operatorIds);
    }

    function cooldownDuration() external view override returns (uint256) {
        return ssvNetwork.cooldownDuration();
    }

    function totalStaked() external view override returns (uint256) {
        return ssvNetwork.totalStaked();
    }

    function stakedBalanceOf(address user) external view override returns (uint256) {
        return ssvNetwork.stakedBalanceOf(user);
    }

    function pendingUnstake(address user) external view override returns (UnstakeRequestsData[] memory) {
        return ssvNetwork.pendingUnstake(user);
    }

    function accEthPerShare() external view override returns (uint256) {
        return ssvNetwork.accEthPerShare();
    }

    function stakingEthPoolBalance() external view override returns (uint256) {
        return ssvNetwork.stakingEthPoolBalance();
    }

    function previewClaimableEth(address user) external view override returns (uint256) {
        return ssvNetwork.previewClaimableEth(user);
    }

    function getOracle(uint32 oracleId) external view override returns (address) {
        return ssvNetwork.getOracle(oracleId);
    }

    function getOracleWeight(uint32 oracleId) external view override returns (uint256) {
        return ssvNetwork.getOracleWeight(oracleId);
    }

    function getActiveOracleIds() external view override returns (uint32[MAX_DELEGATION_SLOTS] memory) {
        return ssvNetwork.getActiveOracleIds();
    }

    function getQuorumBps() external view override returns (uint16) {
        return ssvNetwork.getQuorumBps();
    }

    function getCommittedRoot(uint64 blockNum) external view override returns (bytes32) {
        return ssvNetwork.getCommittedRoot(blockNum);
    }

    function getVersion() external view override returns (string memory) {
        return ssvNetwork.getVersion();
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {PackedSSV, PackedETH, DEDUCTED_DIGITS, ETH_DEDUCTED_DIGITS} from "./SSVCoreTypes.sol";
import {ISSVNetworkCore} from "../interfaces/ISSVNetworkCore.sol";

library PackingLib {

    function _pack(uint256 value, uint256 scale) internal pure returns (uint64) {
        if (value > uint256(type(uint64).max) * scale) revert ISSVNetworkCore.MaxValueExceeded();
        if (value % scale != 0) revert ISSVNetworkCore.MaxPrecisionExceeded();

        return uint64(value / scale);
    }

    function _unpack(uint64 raw, uint256 scale) internal pure returns (uint256) {
        return uint256(raw) * scale;
    }
}

library PackedSSVLib {
    function pack(uint256 value) internal pure returns (PackedSSV) {
        return PackedSSV.wrap(PackingLib._pack(value, DEDUCTED_DIGITS));
    }

    function unpack(PackedSSV packed) internal pure returns (uint256) {
        return PackingLib._unpack(PackedSSV.unwrap(packed), DEDUCTED_DIGITS);
    }

    function raw(PackedSSV packed) internal pure returns (uint64) {
        return PackedSSV.unwrap(packed);
    }

    function eq(PackedSSV a, PackedSSV b) internal pure returns (bool) {
        return PackedSSV.unwrap(a) == PackedSSV.unwrap(b);
    }

    function neq(PackedSSV a, PackedSSV b) internal pure returns (bool) {
        return PackedSSV.unwrap(a) != PackedSSV.unwrap(b);
    }

    function gt(PackedSSV a, PackedSSV b) internal pure returns (bool) {
        return PackedSSV.unwrap(a) > PackedSSV.unwrap(b);
    }

    function lt(PackedSSV a, PackedSSV b) internal pure returns (bool) {
        return PackedSSV.unwrap(a) < PackedSSV.unwrap(b);
    }

    function add(PackedSSV a, PackedSSV b) internal pure returns (PackedSSV) {
        return PackedSSV.wrap(PackedSSV.unwrap(a) + PackedSSV.unwrap(b));
    }
    
    function sub(PackedSSV a, PackedSSV b) internal pure returns (PackedSSV) {
        return PackedSSV.wrap(PackedSSV.unwrap(a) - PackedSSV.unwrap(b));
    }
}

library PackedETHLib {
    function pack(uint256 value) internal pure returns (PackedETH) {
        return PackedETH.wrap(PackingLib._pack(value, ETH_DEDUCTED_DIGITS));
    }

    function unpack(PackedETH packed) internal pure returns (uint256) {
        return PackingLib._unpack(PackedETH.unwrap(packed), ETH_DEDUCTED_DIGITS);
    }

    function raw(PackedETH packed) internal pure returns (uint64) {
        return PackedETH.unwrap(packed);
    }

    function eq(PackedETH a, PackedETH b) internal pure returns (bool) {
        return PackedETH.unwrap(a) == PackedETH.unwrap(b);
    }

    function neq(PackedETH a, PackedETH b) internal pure returns (bool) {
        return PackedETH.unwrap(a) != PackedETH.unwrap(b);
    }

    function gt(PackedETH a, PackedETH b) internal pure returns (bool) {
        return PackedETH.unwrap(a) > PackedETH.unwrap(b);
    }

    function gte(PackedETH a, PackedETH b) internal pure returns (bool) {
        return PackedETH.unwrap(a) >= PackedETH.unwrap(b);
    }

    function lt(PackedETH a, PackedETH b) internal pure returns (bool) {
        return PackedETH.unwrap(a) < PackedETH.unwrap(b);
    }

    function lte(PackedETH a, PackedETH b) internal pure returns (bool) {
        return PackedETH.unwrap(a) <= PackedETH.unwrap(b);
    }

    function add(PackedETH a, PackedETH b) internal pure returns (PackedETH) {
        return PackedETH.wrap(PackedETH.unwrap(a) + PackedETH.unwrap(b));
    }

    function sub(PackedETH a, PackedETH b) internal pure returns (PackedETH) {
        return PackedETH.wrap(PackedETH.unwrap(a) - PackedETH.unwrap(b));
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {PackedETH} from "../SSVCoreTypes.sol";

uint256 constant MAX_DELEGATION_SLOTS = 4;

struct UnstakeRequest {
    /// @notice Amount of cSSV burned and pending to be withdrawn as SSV
    uint192 amount;
    /// @notice Timestamp after which the pending unstake can be withdrawn
    uint64 unlockTime;
}

struct StorageStaking {
    /// @notice Unstake cooldown duration in seconds
    uint64 cooldownDuration;
    /// @notice Total ETH-denominated rewards (shrunk) allocated to the staking pool
    PackedETH stakingEthPoolBalance;
    /// @notice Global accumulated ETH rewards per cSSV token (scaled by PRECISION)
    uint128 accEthPerShare;

    /// @notice Per-user reward index used to track their last settled accEthPerShare
    mapping(address => uint256) userIndex;
    /// @notice Accumulated but unclaimed ETH rewards for each user (in wei)
    mapping(address => uint256) accrued;

    /// @notice Oracle registry: stable ID => oracle address
    mapping(uint32 => address) oracles;
    /// @notice Reverse lookup: oracle address => oracle ID (0 if not registered)
    mapping(address => uint32) oracleIdOf;
    /// @notice Default oracle IDs to use for new delegations (equal split)
    uint32[MAX_DELEGATION_SLOTS] defaultOracleIds;
    /// @notice Quorum threshold in basis points (e.g. 7000 = 70%)
    uint16 quorumBps;
    /// @notice The mapping of address to their unstake requests
    mapping(address => UnstakeRequest[]) withdrawalRequests;
}

library SSVStorageStaking {
    uint256 private constant SSV_STORAGE_POSITION = uint256(keccak256("ssv.network.storage.staking")) - 1;

    function load() internal pure returns (StorageStaking storage ss) {
        uint256 position = SSV_STORAGE_POSITION;
        assembly {
            ss.slot := position
        }
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {ISSVNetworkCore} from "../interfaces/ISSVNetworkCore.sol";
import {StorageData} from "./storage/SSVStorage.sol";
import {StorageProtocol} from "./storage/SSVStorageProtocol.sol";
import {SSVStorageEB, StorageEB} from "./storage/SSVStorageEB.sol";
import {OperatorLib} from "./OperatorLib.sol";
import {ProtocolLib} from "./ProtocolLib.sol";
import {PackedSSV, PackedETH, VERSION_SSV, VERSION_ETH, ETH_DEDUCTED_DIGITS, DEFAULT_EB_PER_VALIDATOR, BPS_DENOMINATOR} from "../libraries/SSVCoreTypes.sol";
import {PackedSSVLib, PackedETHLib} from "../libraries/SSVPackedLib.sol";

/**
 * @title SSV Cluster Library
 * @author SSV Labs
 * @notice Library functions for managing SSV clusters including balance updates, liquidation checks, validations and data operations
 */
library ClusterLib {
    using ProtocolLib for StorageProtocol;

    function updateBalanceSSV(
        ISSVNetworkCore.Cluster memory cluster,
        uint64 newIndex,
        uint64 currentNetworkFeeIndex
    ) internal pure {
        uint64 networkFee = uint64(currentNetworkFeeIndex - cluster.networkFeeIndex) * cluster.validatorCount;
        PackedSSV usage = PackedSSV.wrap((newIndex - cluster.index) * cluster.validatorCount + networkFee);
        cluster.balance = PackedSSVLib.unpack(usage) > cluster.balance ? 0 : cluster.balance - PackedSSVLib.unpack(usage);
    }

    /**
     * @notice Checks if cluster is liquidatable based on balance thresholds
     * @param cluster Cluster data
     * @param burnRate Cluster burn rate
     * @param networkFee Network fee
     * @param minimumBlocksBeforeLiquidation Minimum blocks before liquidation
     * @param minimumLiquidationCollateral Minimum collateral for liquidation
     * @return liquidatable True if cluster can be liquidated
     */
    function isLiquidatable(
        ISSVNetworkCore.Cluster memory cluster,
        uint64 burnRate,
        uint64 networkFee,
        uint64 minimumBlocksBeforeLiquidation,
        PackedSSV minimumLiquidationCollateral
    ) internal pure returns (bool liquidatable) {
        if (cluster.validatorCount != 0) {
            if (cluster.balance < PackedSSVLib.unpack(minimumLiquidationCollateral)) return true;
            uint64 liquidationThreshold = minimumBlocksBeforeLiquidation *
                (burnRate + networkFee) *
                cluster.validatorCount;

            return cluster.balance < PackedSSVLib.unpack(PackedSSV.wrap(liquidationThreshold));
        }
    }

    /**
     * @notice Checks if cluster is liquidatable using effective balance
     * @param cluster Cluster data
     * @param clusterId Cluster ID
     * @param burnRate Cluster burn rate
     * @param networkFee Network fee
     * @param minimumBlocksBeforeLiquidation Minimum blocks before liquidation
     * @param minimumLiquidationCollateral Minimum collateral for liquidation
     * @return liquidatable True if cluster can be liquidated
     */
    function isLiquidatableWithEB(
        ISSVNetworkCore.Cluster memory cluster,
        bytes32 clusterId,
        uint64 burnRate,
        uint64 networkFee,
        uint64 minimumBlocksBeforeLiquidation,
        PackedETH minimumLiquidationCollateral
    ) internal view returns (bool liquidatable) {
        if (cluster.validatorCount == 0) return false;
        if (cluster.balance < PackedETHLib.unpack(minimumLiquidationCollateral)) return true;

        uint64 vUnits = getVUnits(clusterId, cluster.validatorCount);
        uint128 units = vUnits;
        uint128 rate = burnRate + networkFee;
        uint256 thresholdUnits = (uint256(minimumBlocksBeforeLiquidation) * rate * units) / BPS_DENOMINATOR;
        uint256 liquidationThreshold = thresholdUnits * ETH_DEDUCTED_DIGITS;
        return cluster.balance < liquidationThreshold;
    }

    /**
     * @notice Checks if cluster is liquidatable using provided vUnits
     * @param cluster Cluster data
     * @param vUnits cluster VUnits
     * @param burnRate Cluster burn rate
     * @param networkFee Network fee
     * @param minimumBlocksBeforeLiquidation Minimum blocks before liquidation
     * @param minimumLiquidationCollateral Minimum collateral for liquidation
     * @return liquidatable True if cluster can be liquidated
     */
    function isLiquidatableWithVUnits(
        ISSVNetworkCore.Cluster memory cluster,
        uint64 vUnits,
        uint64 burnRate,
        uint64 networkFee,
        uint64 minimumBlocksBeforeLiquidation,
        PackedETH minimumLiquidationCollateral
    ) internal pure returns (bool liquidatable) {
        if (cluster.validatorCount == 0) return false;
        if (cluster.balance < PackedETHLib.unpack(minimumLiquidationCollateral)) return true;

        uint128 units = vUnits;
        uint128 rate = burnRate + networkFee;
        uint256 thresholdUnits = (uint256(minimumBlocksBeforeLiquidation) * rate * units) / BPS_DENOMINATOR;
        uint256 liquidationThreshold = thresholdUnits * ETH_DEDUCTED_DIGITS;
        return cluster.balance < liquidationThreshold;
    }

    /**
     * @notice Validates that cluster is not liquidated
     * @param cluster Cluster data
     */
    function validateClusterIsNotLiquidated(ISSVNetworkCore.Cluster memory cluster) internal pure {
        if (!cluster.active) revert ISSVNetworkCore.ClusterIsLiquidated();
    }

    /**
     * @notice Validates and hashes cluster data
     * @param cluster Cluster data
     * @param owner Cluster owner
     * @param operatorIds Operator IDs
     * @param s Storage data
     * @return hashedCluster Hashed cluster ID
     * @return version Cluster version
     */
    function validateHashedCluster(
        ISSVNetworkCore.Cluster memory cluster,
        address owner,
        uint64[] memory operatorIds,
        StorageData storage s
    ) internal view returns (bytes32 hashedCluster, uint8 version) {
        hashedCluster = keccak256(abi.encodePacked(owner, operatorIds));
        bytes32 hashedClusterData = hashClusterData(cluster);

        (bytes32 clusterData, uint8 detectedVersion) = getClusterData(hashedCluster, s);
        if (clusterData == bytes32(0)) {
            revert ISSVNetworkCore.ClusterDoesNotExist();
        } else if (clusterData != hashedClusterData) {
            revert ISSVNetworkCore.IncorrectClusterState();
        }

        return (hashedCluster, detectedVersion);
    }

    /**
     * @notice Updates ETH cluster data with new indexes
     * @param cluster Cluster data
     * @param clusterIndex New cluster index
     * @param currentNetworkFeeIndex Current network fee index
     */
    function updateClusterData(
        ISSVNetworkCore.Cluster memory cluster,
        bytes32 hashedCluster,
        uint64 clusterIndex,
        uint64 currentNetworkFeeIndex
    ) internal view {
        updateBalanceWithEB(cluster, hashedCluster, clusterIndex, currentNetworkFeeIndex);
        cluster.index = clusterIndex;
        cluster.networkFeeIndex = currentNetworkFeeIndex;
    }

    /**
     * @notice Hashes cluster data
     * @param cluster Cluster data
     * @return Hashed cluster data
     */
    function hashClusterData(ISSVNetworkCore.Cluster memory cluster) internal pure returns (bytes32) {
        return
            keccak256(
                abi.encodePacked(
                    cluster.validatorCount,
                    cluster.networkFeeIndex,
                    cluster.index,
                    cluster.balance,
                    cluster.active
                )
            );
    }

    /**
     * @notice Validates cluster state for registration
     * @param cluster Cluster data
     * @param owner Cluster owner
     * @param operatorIds Operator IDs
     * @param s Storage data
     * @return hashedCluster Hashed cluster ID
     */
    function validateClusterOnRegistration(
        ISSVNetworkCore.Cluster memory cluster,
        address owner,
        uint64[] memory operatorIds,
        StorageData storage s
    ) internal view returns (bytes32 hashedCluster) {
        hashedCluster = keccak256(abi.encodePacked(owner, operatorIds));

        bytes32 clusterData = s.ethClusters[hashedCluster];
        bytes32 clusterDataSSV = s.clusters[hashedCluster];

        if (clusterData == bytes32(0) && clusterDataSSV!= bytes32(0)) {
            revert ISSVNetworkCore.IncorrectClusterVersion();
        }

        if (clusterData == bytes32(0)) {
            if (
                cluster.validatorCount != 0 ||
                cluster.networkFeeIndex != 0 ||
                cluster.index != 0 ||
                cluster.balance != 0 ||
                !cluster.active
            ) {
                revert ISSVNetworkCore.IncorrectClusterState();
            }
        } else if (clusterData != hashClusterData(cluster)) {
            revert ISSVNetworkCore.IncorrectClusterState();
        } else {
            validateClusterIsNotLiquidated(cluster);
        }
    }

    /**
     * @notice Updates cluster on registration
     * @param cluster Cluster data
     * @param operatorIds Operator IDs
     * @param hashedCluster Hashed cluster ID
     * @param validatorCountDelta Change in validator count
     * @param s Storage data
     * @param sp Storage protocol
     */
    function updateClusterOnRegistration(
        ISSVNetworkCore.Cluster memory cluster,
        uint64[] memory operatorIds,
        bytes32 hashedCluster,
        uint32 validatorCountDelta,
        StorageData storage s,
        StorageProtocol storage sp
    ) internal {
        (uint64 clusterIndex, uint64 burnRate) = OperatorLib.updateClusterOperatorsOnRegistration(
            operatorIds,
            validatorCountDelta,
            s,
            sp
        );

        updateClusterData(cluster, hashedCluster, clusterIndex, sp.currentNetworkFeeIndex());

        sp.updateDAO(true, validatorCountDelta);

        cluster.validatorCount += validatorCountDelta;

        {
            StorageEB storage seb = SSVStorageEB.load();
            uint64 storedVUnits = seb.clusterEB[hashedCluster].vUnits;
            uint64 projectedVUnits = storedVUnits > 0
                ? storedVUnits + uint64(validatorCountDelta) * BPS_DENOMINATOR
                : uint64(cluster.validatorCount) * BPS_DENOMINATOR;

            if (
                isLiquidatableWithVUnits(
                    cluster,
                    projectedVUnits,
                    burnRate,
                    PackedETH.unwrap(sp.ethNetworkFee),
                    sp.minimumBlocksBeforeLiquidation,
                    sp.minimumLiquidationCollateral
                )
            ) {
                revert ISSVNetworkCore.InsufficientBalance();
            }
        }

        s.ethClusters[hashedCluster] = hashClusterData(cluster);
    }

    /**
     * @notice Gets VUnits for cluster
     * @param clusterId Cluster ID
     * @param validatorCount Validator count
     * @return vUnits cluster VUnits
     */
    function getVUnits(bytes32 clusterId, uint32 validatorCount) internal view returns (uint64) {
        StorageEB storage seb = SSVStorageEB.load();
        uint64 vUnits = seb.clusterEB[clusterId].vUnits;

        if (vUnits == 0) {
            // Before any EB is set for this cluster, approximate EB as 32 ETH per validator.
            // To preserve legacy accounting, we treat each validator as 1 logical vUnit (32 ETH),
            // scaled by BPS_DENOMINATOR for fixed-point arithmetic.
            return uint64(validatorCount) * BPS_DENOMINATOR;
        }

        return vUnits;
    }

    /**
     * @notice Updates cluster balance using effective balance
     * @param cluster Cluster data
     * @param clusterId Cluster ID
     * @param newIndex New operator index
     * @param currentNetworkFeeIndex Current network fee index
     */
    function updateBalanceWithEB(
        ISSVNetworkCore.Cluster memory cluster,
        bytes32 clusterId,
        uint64 newIndex,
        uint64 currentNetworkFeeIndex
    ) internal view {
        uint64 vUnits = getVUnits(clusterId, cluster.validatorCount);
        uint128 units = vUnits;
        uint128 idxNet = currentNetworkFeeIndex - cluster.networkFeeIndex;
        uint128 idxOp = newIndex - cluster.index;

        uint128 networkFeeUnits = (idxNet * units) / BPS_DENOMINATOR;
        uint128 usageUnits = (idxOp * units) / BPS_DENOMINATOR + networkFeeUnits;
        uint256 usage = uint256(usageUnits) * ETH_DEDUCTED_DIGITS;
        cluster.balance = usage > cluster.balance ? 0 : cluster.balance - usage;
    }

    /**
     * @notice Validates cluster version and throws error if version is not the expected one
     * @param clusterVersion Detected version
     * @param expectedVersion Expected version
     */
    function validateClusterVersion(uint8 clusterVersion, uint8 expectedVersion) internal pure {
        if (clusterVersion != expectedVersion) revert ISSVNetworkCore.IncorrectClusterVersion();
    }

    /**
     * @notice Gets cluster data from storage
     * @param hashedCluster Hashed cluster ID
     * @param s Storage data
     * @return clusterData Hashed cluster data
     * @return version Cluster version
     */
    function getClusterData(
        bytes32 hashedCluster,
        StorageData storage s
    ) internal view returns (bytes32 clusterData, uint8 version) {
        clusterData = s.ethClusters[hashedCluster];
        bytes32 clusterDataSSV = s.clusters[hashedCluster];

        if (clusterData != bytes32(0) && clusterDataSSV != bytes32(0)) {
            revert ISSVNetworkCore.IncorrectClusterState();
        }

        if (clusterData != bytes32(0)) {
            return (clusterData, VERSION_ETH);
        }

        if (clusterDataSSV != bytes32(0)) {
            return (clusterDataSSV, VERSION_SSV);
        }

        revert ISSVNetworkCore.ClusterDoesNotExist();
    }

    /**
     * @notice Converts effective balance to v units using ceiling division
     * @param effectiveBalance Effective balance in ETH
     * @return vUnits v units scaled by precision
     */
    function ebToVUnits(uint32 effectiveBalance) internal pure returns (uint64) {
        uint256 vUnits = uint256(effectiveBalance) * BPS_DENOMINATOR;
        uint256 vUnitsPerValidator = DEFAULT_EB_PER_VALIDATOR / 1 ether;
        
        return uint64(vUnits == 0 ? 0 : (vUnits - 1) / vUnitsPerValidator + 1);
    }

    /**
     * @notice Converts v units to effective balance using floor division
     * @param vUnits v units scaled by precision
     * @return effectiveBalance Effective balance in ETH
     */
    function vUnitsToEB(uint64 vUnits) internal pure returns (uint32) {
        return uint32((uint256(vUnits) * (DEFAULT_EB_PER_VALIDATOR / 1 ether)) / BPS_DENOMINATOR);
    }
}
// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v4.9.0) (access/Ownable.sol)

pragma solidity ^0.8.0;

import "../utils/ContextUpgradeable.sol";
import {Initializable} from "../proxy/utils/Initializable.sol";

/**
 * @dev Contract module which provides a basic access control mechanism, where
 * there is an account (an owner) that can be granted exclusive access to
 * specific functions.
 *
 * By default, the owner account will be the one that deploys the contract. This
 * can later be changed with {transferOwnership}.
 *
 * This module is used through inheritance. It will make available the modifier
 * `onlyOwner`, which can be applied to your functions to restrict their use to
 * the owner.
 */
abstract contract OwnableUpgradeable is Initializable, ContextUpgradeable {
    address private _owner;

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    /**
     * @dev Initializes the contract setting the deployer as the initial owner.
     */
    function __Ownable_init() internal onlyInitializing {
        __Ownable_init_unchained();
    }

    function __Ownable_init_unchained() internal onlyInitializing {
        _transferOwnership(_msgSender());
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
        require(owner() == _msgSender(), "Ownable: caller is not the owner");
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
        require(newOwner != address(0), "Ownable: new owner is the zero address");
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

    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[49] private __gap;
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {ISSVViews} from "../interfaces/ISSVViews.sol";
import {ISSVWhitelistingContract} from "../interfaces/external/ISSVWhitelistingContract.sol";
import {ICSSVToken} from "../interfaces/ICSSVToken.sol";
import {ClusterLib} from "../libraries/ClusterLib.sol";
import {OperatorLib} from "../libraries/OperatorLib.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {ProtocolLib} from "../libraries/ProtocolLib.sol";
import {PackedSSV, PackedETH, VERSION_ETH, VERSION_SSV, DEFAULT_OPERATOR_ETH_FEE, PRECISION, BPS_DENOMINATOR} from "../libraries/SSVCoreTypes.sol";
import {PackedSSVLib, PackedETHLib} from "../libraries/SSVPackedLib.sol";
import {SSVStorage, StorageData} from "../libraries/storage/SSVStorage.sol";
import {SSVStorageProtocol, StorageProtocol} from "../libraries/storage/SSVStorageProtocol.sol";
import {SSVStorageEB, StorageEB} from "../libraries/storage/SSVStorageEB.sol";
import {MAX_DELEGATION_SLOTS, SSVStorageStaking, StorageStaking, UnstakeRequest} from "../libraries/storage/SSVStorageStaking.sol";

contract SSVViews is ISSVViews {
    using ClusterLib for Cluster;
    using OperatorLib for Operator;
    using ProtocolLib for StorageProtocol;
    using PackedETHLib for PackedETH;
    using PackedSSVLib for PackedSSV;

    address public immutable CSSV_ADDRESS;

    constructor(address _cssv) {
        CSSV_ADDRESS = _cssv;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getValidator(address clusterOwner, bytes calldata publicKey) external view override returns (bool) {
        bytes32 validatorData = SSVStorage.load().validatorPKs[keccak256(abi.encodePacked(publicKey, clusterOwner))];

        if (validatorData == bytes32(0)) return false;
        bytes32 activeFlag = validatorData & bytes32(uint256(1)); // Retrieve LSB of stored value

        return activeFlag == bytes32(uint256(1));
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getOperatorFee(uint64 operatorId) external view override returns (uint256) {
        Operator storage operator = SSVStorage.load().operators[operatorId];
        if (operator.ethSnapshot.block != 0) {
            return PackedETHLib.unpack(operator.ethFee);
        } else if (PackedSSV.unwrap(operator.fee) != 0) {
            return DEFAULT_OPERATOR_ETH_FEE;
        }
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getOperatorFeeSSV(uint64 operatorId) external view override returns (uint256) {
        return PackedSSVLib.unpack(SSVStorage.load().operators[operatorId].fee);
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getOperatorDeclaredFee(uint64 operatorId) external view override returns (OperatorDeclaredFeeData memory) {
        StorageData storage s = SSVStorage.load();
        OperatorFeeChangeRequest memory opFeeChangeRequest = s.operatorFeeChangeRequests[operatorId];

        bool isETHOperator = s.operators[operatorId].ethSnapshot.block != 0;

        return OperatorDeclaredFeeData(
            opFeeChangeRequest.approvalBeginTime != 0,
            isETHOperator ? PackedETHLib.unpack(PackedETH.wrap(opFeeChangeRequest.fee)) : PackedSSVLib.unpack(PackedSSV.wrap(opFeeChangeRequest.fee)),
            opFeeChangeRequest.approvalBeginTime,
            opFeeChangeRequest.approvalEndTime
        );
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getOperatorById(
        uint64 operatorId
    ) external view override returns (OperatorData memory op)
    {
        Operator storage operator = SSVStorage.load().operators[operatorId];

        op.owner = operator.owner;
        if (operator.ethSnapshot.block != 0) {
            op.fee = PackedETHLib.unpack(operator.ethFee);
        } else if (PackedSSV.unwrap(operator.fee) != 0) {
            op.fee = DEFAULT_OPERATOR_ETH_FEE;
        }

        op.validatorCount = operator.ethValidatorCount;
        op.whitelistedAddress = SSVStorage.load().operatorsWhitelist[operatorId];
        op.isPrivate = operator.whitelisted;
        op.isActive = operator.ethSnapshot.block != 0 || operator.snapshot.block != 0;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getOperatorByIdSSV(
        uint64 operatorId
    ) external view override returns (OperatorData memory op)
    {
        Operator storage operator = SSVStorage.load().operators[operatorId];

        op.owner = operator.owner;
        op.fee = PackedSSVLib.unpack(operator.fee);
        op.validatorCount = operator.validatorCount;
        op.whitelistedAddress = SSVStorage.load().operatorsWhitelist[operatorId];
        op.isPrivate = operator.whitelisted;
        op.isActive = operator.ethSnapshot.block != 0 || operator.snapshot.block != 0;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getWhitelistedOperators(
        uint64[] calldata operatorIds,
        address addressToCheck
    ) external view override returns (uint64[] memory whitelistedOperatorIds) {
        uint256 operatorsLength = operatorIds.length;
        if (operatorsLength == 0 || addressToCheck == address(0)) return whitelistedOperatorIds;

        StorageData storage s = SSVStorage.load();

        uint256 internalCount;

        // Check whitelisting address for each operator using the internal SSV whitelisting module
        (uint256[] memory masks, uint256 startBlockIndex) = OperatorLib.generateBlockMasks(operatorIds, false, s);
        uint64[] memory internalWhitelistedOperatorIds = new uint64[](operatorsLength);

        uint256 endBlockIndex = startBlockIndex + masks.length;
        // Check whitelisting status for each mask
        for (uint256 blockIndex = startBlockIndex; blockIndex < endBlockIndex; ++blockIndex) {
            uint256 mask = masks[blockIndex - startBlockIndex];
            // Only check blocks that have operator IDs
            if (mask != 0) {
                uint256 whitelistedMask = s.addressWhitelistedForOperators[addressToCheck][blockIndex];

                // This will give the matching whitelisted operators
                uint256 matchedMask = whitelistedMask & mask;

                // Now we need to extract operator IDs from matchedMask
                uint256 blockPointer = blockIndex << 8;
                for (uint256 bit; bit < 256; ++bit) {
                    if (matchedMask & (1 << bit) != 0) {
                        internalWhitelistedOperatorIds[internalCount++] = uint64(blockPointer + bit);
                        if (internalCount == operatorsLength) {
                            return internalWhitelistedOperatorIds; // Early termination
                        }
                    }
                }
            }
        }

        // Resize internalWhitelistedOperatorIds to the actual number of whitelisted operators
        assembly {
            mstore(internalWhitelistedOperatorIds, internalCount)
        }

        // Check if pending operators use an external whitelisting contract and check whitelistedAddress using it
        whitelistedOperatorIds = new uint64[](operatorsLength);
        uint256 internalWhitelistIndex;
        uint256 count;

        for (uint256 operatorIndex; operatorIndex < operatorsLength; ++operatorIndex) {
            uint64 operatorId = operatorIds[operatorIndex];

            // Check if operatorId is already in internalWhitelistedOperatorIds
            if (
                internalWhitelistIndex < internalCount &&
                operatorId == internalWhitelistedOperatorIds[internalWhitelistIndex]
            ) {
                whitelistedOperatorIds[count++] = operatorId;
                ++internalWhitelistIndex;
            } else {
                address whitelistedAddress = s.operatorsWhitelist[operatorId];

                // Legacy address whitelists (EOAs or generic contracts)
                if (
                    whitelistedAddress == addressToCheck ||
                    (OperatorLib.isWhitelistingContract(whitelistedAddress) &&
                        ISSVWhitelistingContract(whitelistedAddress).isWhitelisted(addressToCheck, operatorId))
                ) {
                    whitelistedOperatorIds[count++] = operatorId;
                }
            }
        }

        // Resize whitelistedOperatorIds to the actual number of whitelisted operators
        assembly {
            mstore(whitelistedOperatorIds, count)
        }
    }

    /**
     * @inheritdoc ISSVViews
     */
    function isWhitelistingContract(address contractAddress) external view override returns (bool) {
        return OperatorLib.isWhitelistingContract(contractAddress);
    }

    /**
     * @inheritdoc ISSVViews
     */
    function isAddressWhitelistedInWhitelistingContract(
        address addressToCheck,
        uint256 operatorId,
        address whitelistingContract
    ) external view override returns (bool) {
        if (!OperatorLib.isWhitelistingContract(whitelistingContract) || addressToCheck == address(0)) return false;
        return ISSVWhitelistingContract(whitelistingContract).isWhitelisted(addressToCheck, operatorId);
    }

    /**
     * @inheritdoc ISSVViews
     */
    function isLiquidatable(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view override returns (bool) {
        StorageData storage s = SSVStorage.load();
        (bytes32 hashedCluster, uint8 version) = cluster.validateHashedCluster(clusterOwner, operatorIds, s);
        ClusterLib.validateClusterVersion(version, VERSION_ETH);

        if (!cluster.active) {
            return false;
        }

        uint64 clusterIndex;
        uint64 burnRate;
        uint256 operatorsLength = operatorIds.length;
        for (uint256 i; i < operatorsLength; ++i) {
            Operator memory operator = s.operators[operatorIds[i]];
            clusterIndex += operator.ethSnapshot.index + (uint64(block.number) - operator.ethSnapshot.block) * PackedETH.unwrap(operator.ethFee);
            burnRate += PackedETH.unwrap(operator.ethFee);
        }

        StorageProtocol storage sp = SSVStorageProtocol.load();

        cluster.updateBalanceWithEB(hashedCluster, clusterIndex, sp.currentNetworkFeeIndex());
        return
            cluster.isLiquidatableWithEB(
                hashedCluster,
                burnRate,
                PackedETH.unwrap(sp.ethNetworkFee),
                sp.minimumBlocksBeforeLiquidation,
                sp.minimumLiquidationCollateral
            );
    }

    /**
     * @inheritdoc ISSVViews
     */
    function isLiquidatableSSV(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view override returns (bool) {
        StorageData storage s = SSVStorage.load();
        (, uint8 version) = cluster.validateHashedCluster(clusterOwner, operatorIds, s);
        ClusterLib.validateClusterVersion(version, VERSION_SSV);

        if (!cluster.active) {
            return false;
        }

        uint64 clusterIndex;
        uint64 burnRate;
        uint256 operatorsLength = operatorIds.length;
        for (uint256 i; i < operatorsLength; ++i) {
            Operator memory operator = s.operators[operatorIds[i]];
            clusterIndex += operator.snapshot.index + (uint64(block.number) - operator.snapshot.block) * PackedSSV.unwrap(operator.fee);
            burnRate += PackedSSV.unwrap(operator.fee);
        }

        StorageProtocol storage sp = SSVStorageProtocol.load();

        cluster.updateBalanceSSV(clusterIndex, sp.currentNetworkFeeIndexSSV());
        return
            cluster.isLiquidatable(
                burnRate,
                PackedSSV.unwrap(sp.networkFee),
                sp.minimumBlocksBeforeLiquidationSSV,
                sp.minimumLiquidationCollateralSSV
            );
    }

    /**
     * @inheritdoc ISSVViews
     */
    function isLiquidated(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view override returns (bool) {
        cluster.validateHashedCluster(clusterOwner, operatorIds, SSVStorage.load());
        return !cluster.active;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getBurnRate(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view override returns (uint256) {
        StorageData storage s = SSVStorage.load();
        (bytes32 hashedCluster, uint8 version) = cluster.validateHashedCluster(
            clusterOwner,
            operatorIds,
            s
        );
        ClusterLib.validateClusterVersion(version, VERSION_ETH);

        PackedETH operatorsFee;
        uint256 len = operatorIds.length;
        for (uint256 i; i < len; ++i) {
            Operator memory op = s.operators[operatorIds[i]];
            if (op.owner != address(0)) {
                operatorsFee = operatorsFee.add(op.ethFee);
            }
        }

        PackedETH networkFee = SSVStorageProtocol.load().ethNetworkFee;

        uint64 vUnits = SSVStorageEB.load().clusterEB[hashedCluster].vUnits;
        if (vUnits == 0) {
            vUnits = uint64(cluster.validatorCount) * BPS_DENOMINATOR;
        }

        return (PackedETHLib.unpack(networkFee.add(operatorsFee)) * uint256(vUnits)) / BPS_DENOMINATOR;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getBurnRateSSV(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view override returns (uint256) {
        StorageData storage s = SSVStorage.load();
        (, uint8 version) = cluster.validateHashedCluster(clusterOwner, operatorIds, s);
        ClusterLib.validateClusterVersion(version, VERSION_SSV);

        PackedSSV aggregateFee;
        uint256 operatorsLength = operatorIds.length;
        for (uint256 i; i < operatorsLength; ++i) {
            Operator memory operator = s.operators[operatorIds[i]];
            if (operator.owner != address(0)) {
                aggregateFee = aggregateFee.add(operator.fee);
            }
        }

        uint128 burnRate = PackedSSV.unwrap(aggregateFee.add(SSVStorageProtocol.load().networkFee)) * cluster.validatorCount;
        return PackedSSVLib.unpack(PackedSSV.wrap(uint64(burnRate)));
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getOperatorEarnings(uint64 id) external view override returns (uint256) {
        Operator memory operator = SSVStorage.load().operators[id];

        operator.updateSnapshot(id);
        return PackedETHLib.unpack(operator.ethSnapshot.balance);
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getOperatorEarningsSSV(uint64 id) external view override returns (uint256) {
        Operator memory operator = SSVStorage.load().operators[id];

        operator.updateSnapshotSSV();
        return PackedSSVLib.unpack(operator.snapshot.balance);
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getBalance(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view override returns (uint256 balance) {
        StorageData storage s = SSVStorage.load();
        (bytes32 hashedCluster, uint8 version) = cluster.validateHashedCluster(clusterOwner, operatorIds, s);
        ClusterLib.validateClusterVersion(version, VERSION_ETH);
        cluster.validateClusterIsNotLiquidated();

        uint64 clusterIndex;
        uint256 operatorsLength = operatorIds.length;
        for (uint256 i; i < operatorsLength; ++i) {
            Operator memory operator = s.operators[operatorIds[i]];
            clusterIndex += operator.ethSnapshot.index + (uint64(block.number) - operator.ethSnapshot.block) * PackedETH.unwrap(operator.ethFee);
        }

        StorageProtocol storage sp = SSVStorageProtocol.load();
        cluster.updateBalanceWithEB(hashedCluster, clusterIndex, sp.currentNetworkFeeIndex());
        balance = cluster.balance;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getBalanceSSV(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view override returns (uint256 balance) {
        StorageData storage s = SSVStorage.load();
        (, uint8 version) = cluster.validateHashedCluster(clusterOwner, operatorIds, s);
        ClusterLib.validateClusterVersion(version, VERSION_SSV);
        cluster.validateClusterIsNotLiquidated();

        uint64 clusterIndex;
        uint256 operatorsLength = operatorIds.length;
        for (uint256 i; i < operatorsLength; ++i) {
            Operator memory operator = s.operators[operatorIds[i]];
            clusterIndex += operator.snapshot.index + (uint64(block.number) - operator.snapshot.block) * PackedSSV.unwrap(operator.fee);
        }

        cluster.updateBalanceSSV(clusterIndex, SSVStorageProtocol.load().currentNetworkFeeIndexSSV());
        balance = cluster.balance;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getEffectiveBalance(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view returns (uint32 effectiveBalance) {
        StorageData storage s = SSVStorage.load();
        (bytes32 hashedCluster, ) = cluster.validateHashedCluster(clusterOwner, operatorIds, s);
        cluster.validateClusterIsNotLiquidated();

        StorageEB storage seb = SSVStorageEB.load();
        uint64 vUnits = seb.clusterEB[hashedCluster].vUnits;

        if (vUnits == 0) {
            vUnits = cluster.validatorCount * BPS_DENOMINATOR;
        }

        return ClusterLib.vUnitsToEB(vUnits);
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getClusterAssetType(address clusterOwner, uint64[] calldata operatorIds) external view override returns (uint8) {
        StorageData storage s = SSVStorage.load();
        bytes32 hashedCluster = keccak256(abi.encodePacked(clusterOwner, operatorIds));

        if (s.ethClusters[hashedCluster] != bytes32(0)) {
            return VERSION_ETH;
        }
        if (s.clusters[hashedCluster] != bytes32(0)) {
            return VERSION_SSV;
        }

        revert ClusterDoesNotExist();
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getNetworkFee() external view override returns (uint256) {
        return PackedETHLib.unpack(SSVStorageProtocol.load().ethNetworkFee);
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getNetworkFeeSSV() external view override returns (uint256) {
        return PackedSSVLib.unpack(SSVStorageProtocol.load().networkFee);
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getNetworkEarnings() external view override returns (uint256) {
        return PackedETHLib.unpack(SSVStorageProtocol.load().networkTotalEarnings());
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getNetworkEarningsSSV() external view override returns (uint256) {
        return PackedSSVLib.unpack(SSVStorageProtocol.load().networkTotalEarningsSSV());
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getOperatorFeeIncreaseLimit() external view override returns (uint64) {
        return SSVStorageProtocol.load().operatorMaxFeeIncrease;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getMaximumOperatorFee() external view override returns (uint256) {
        return SSVStorageProtocol.load().operatorMaxFee.unpack();
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getMaximumOperatorFeeSSV() external view override returns (uint256) {
        return SSVStorageProtocol.load().operatorMaxFeeSSV;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getMinimumOperatorEthFee() external view override returns (uint256) {
        return SSVStorageProtocol.load().minimumOperatorEthFee.unpack();
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getOperatorFeePeriods() external view override returns (OperatorFeePeriodsData memory) {
        StorageProtocol storage sp = SSVStorageProtocol.load();
        return OperatorFeePeriodsData(
            sp.declareOperatorFeePeriod,
            sp.executeOperatorFeePeriod
        );
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getLiquidationThresholdPeriod() external view override returns (uint64) {
        return SSVStorageProtocol.load().minimumBlocksBeforeLiquidation;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getLiquidationThresholdPeriodSSV() external view override returns (uint64) {
        return SSVStorageProtocol.load().minimumBlocksBeforeLiquidationSSV;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getMinimumLiquidationCollateral() external view override returns (uint256) {
        return PackedETHLib.unpack(SSVStorageProtocol.load().minimumLiquidationCollateral);
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getMinimumLiquidationCollateralSSV() external view override returns (uint256) {
        return PackedSSVLib.unpack(SSVStorageProtocol.load().minimumLiquidationCollateralSSV);
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getValidatorsPerOperatorLimit() external view override returns (uint32) {
        return SSVStorageProtocol.load().validatorsPerOperatorLimit;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getNetworkValidatorsCount() external view override returns (uint32) {
        return SSVStorageProtocol.load().ethDaoValidatorCount;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function cooldownDuration() external view override returns (uint256) {
        return SSVStorageStaking.load().cooldownDuration;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function totalStaked() external view override returns (uint256) {
        return ICSSVToken(CSSV_ADDRESS).totalSupply();
    }

    /**
     * @inheritdoc ISSVViews
     */
    function stakedBalanceOf(address user) external view override returns (uint256) {
        return ICSSVToken(CSSV_ADDRESS).balanceOf(user);
    }

    /**
     * @inheritdoc ISSVViews
     */
    function pendingUnstake(address user) external view override returns (UnstakeRequestsData[] memory data) {
        StorageStaking storage s = SSVStorageStaking.load();
        UnstakeRequest[] storage requests = s.withdrawalRequests[user];

        uint256 len = requests.length;
        data = new UnstakeRequestsData[](len);

        for (uint256 i = 0; i < len; i++) {
            data[i] = UnstakeRequestsData({
                amount: requests[i].amount,
                unlockTime: requests[i].unlockTime
            });
        }
    }

    /**
     * @inheritdoc ISSVViews
     */
    function accEthPerShare() external view override returns (uint256) {
        return SSVStorageStaking.load().accEthPerShare;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function stakingEthPoolBalance() external view override returns (uint256) {
        return SSVStorageStaking.load().stakingEthPoolBalance.unpack();
    }

    /**
     * @inheritdoc ISSVViews
     */
    function previewClaimableEth(address user) external view override returns (uint256) {
        StorageStaking storage s = SSVStorageStaking.load();
        uint256 idx = _previewAccEthPerShare(s);
        uint256 bal = ICSSVToken(CSSV_ADDRESS).balanceOf(user);
        uint256 delta = idx - s.userIndex[user];
        uint256 pending = (bal * delta) / PRECISION;
        return s.accrued[user] + pending;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getOracle(uint32 oracleId) external view override returns (address) {
        return SSVStorageStaking.load().oracles[oracleId];
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getOracleWeight(uint32 oracleId) external view override returns (uint256) {
        uint256 staked = ICSSVToken(CSSV_ADDRESS).totalSupply();
        return staked / SSVStorageStaking.load().defaultOracleIds.length;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getActiveOracleIds() external view override returns (uint32[MAX_DELEGATION_SLOTS] memory) {
        return SSVStorageStaking.load().defaultOracleIds;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getQuorumBps() external view override returns (uint16) {
        return SSVStorageStaking.load().quorumBps;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getCommittedRoot(uint64 blockNum) external view override returns (bytes32) {
        return SSVStorageEB.load().ebRoots[blockNum];
    }

    function _previewAccEthPerShare(StorageStaking storage s) internal view returns (uint256) {
        StorageProtocol storage sp = SSVStorageProtocol.load();
        PackedETH current = sp.networkTotalEarnings();

        uint256 idx = s.accEthPerShare;
        PackedETH previous = s.stakingEthPoolBalance;

        uint256 totalStaked_ = ICSSVToken(CSSV_ADDRESS).totalSupply();

        if (current.lte(previous) || totalStaked_ == 0) {
            return idx;
        }

        PackedETH packedNewFees = current.sub(previous);
        uint256 newFeesWei = PackedETHLib.unpack(packedNewFees);
        return idx + (newFeesWei * PRECISION) / totalStaked_;
    }

    /**
     * @inheritdoc ISSVViews
     */
    function getVersion() external pure override returns (string memory) {
        return CoreLib.getVersion();
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {ISSVNetworkCore} from "../interfaces/ISSVNetworkCore.sol";
import {ISSVWhitelistingContract} from "../interfaces/external/ISSVWhitelistingContract.sol";
import {StorageData} from "./storage/SSVStorage.sol";
import {StorageProtocol} from "./storage/SSVStorageProtocol.sol";
import {PackedETH, PackedSSV, DEFAULT_OPERATOR_ETH_FEE, PACKED_ETH_ZERO, PACKED_SSV_ZERO, BPS_DENOMINATOR, _safeUint64} from "../libraries/SSVCoreTypes.sol";
import {PackedETHLib, PackedSSVLib} from "../libraries/SSVPackedLib.sol";
import {StorageEB, SSVStorageEB} from "./storage/SSVStorageEB.sol";
import {ERC165Checker} from "@openzeppelin/contracts/utils/introspection/ERC165Checker.sol";
import {ISSVOperators} from "../interfaces/ISSVOperators.sol";

/**
 * @title SSV Operator Library
 * @author SSV Labs
 * @notice Library functions for managing SSV operators including snapshot updates, cluster operations, whitelists and validations
 */
library OperatorLib {
    using PackedETHLib for PackedETH;
    using PackedSSVLib for PackedSSV;

    /**
     * @notice Updates SSV operator snapshot
     * @param operator Operator data
     */
    function updateSnapshotSSV(ISSVNetworkCore.Operator memory operator) internal view {
        uint64 blockDiffFee = (uint32(block.number) - operator.snapshot.block) * PackedSSV.unwrap(operator.fee);

        operator.snapshot.index += blockDiffFee;
        operator.snapshot.balance = operator.snapshot.balance.add(PackedSSV.wrap(blockDiffFee * operator.validatorCount));
        operator.snapshot.block = uint32(block.number);
    }

    /**
     * @notice Updates stored SSV operator snapshot
     * @param operator Operator storage reference
     */
    function updateSnapshotStSSV(ISSVNetworkCore.Operator storage operator) internal {
        uint64 blockDiffFee = (uint32(block.number) - operator.snapshot.block) * PackedSSV.unwrap(operator.fee);

        operator.snapshot.index += blockDiffFee;
        operator.snapshot.balance = operator.snapshot.balance.add(PackedSSV.wrap(blockDiffFee * operator.validatorCount));
        operator.snapshot.block = uint32(block.number);
    }

    /**
     * @notice Updates stored ETH operator snapshot
     * @param operator Operator storage reference
     * @param operatorId Operator ID
     */
    function updateSnapshotSt(
        ISSVNetworkCore.Operator storage operator,
        uint64 operatorId
    ) internal {
        StorageEB storage seb = SSVStorageEB.load();
        uint32 currentBlock = uint32(block.number);
        uint64 blockDiffEthFee = (currentBlock - operator.ethSnapshot.block) * PackedETH.unwrap(operator.ethFee);

        // Deviation-only model: effectiveVUnits = baseline + storedDeviation
        // storedDeviation = operatorEthVUnits (only non-default EB contributions)
        // baseline = ethValidatorCount * BPS_DENOMINATOR
        uint64 storedDeviation = seb.operatorEthVUnits[operatorId];
        uint64 effectiveVUnits = storedDeviation + (uint64(operator.ethValidatorCount) * BPS_DENOMINATOR);

        operator.ethSnapshot.index += blockDiffEthFee;
        if (effectiveVUnits != 0 && blockDiffEthFee != 0) {
            uint128 delta = (uint128(blockDiffEthFee) * uint128(effectiveVUnits)) / BPS_DENOMINATOR;
            operator.ethSnapshot.balance = operator.ethSnapshot.balance.add(PackedETH.wrap(_safeUint64(delta)));
        }
        operator.ethSnapshot.block = currentBlock;
    }

    /**
     * @notice Updates ETH operator snapshot
     * @param operator Operator data
     * @param operatorId Operator ID
     */
    function updateSnapshot(
        ISSVNetworkCore.Operator memory operator,
        uint64 operatorId
    ) internal view {
        StorageEB storage seb = SSVStorageEB.load();
        uint32 currentBlock = uint32(block.number);
        uint64 blockDiffEthFee = (currentBlock - operator.ethSnapshot.block) * PackedETH.unwrap(operator.ethFee);

        // Deviation-only model: effectiveVUnits = baseline + storedDeviation
        uint64 storedDeviation = seb.operatorEthVUnits[operatorId];
        uint64 effectiveVUnits = storedDeviation + (uint64(operator.ethValidatorCount) * BPS_DENOMINATOR);

        operator.ethSnapshot.index += blockDiffEthFee;
        if (effectiveVUnits != 0 && blockDiffEthFee != 0) {
            uint128 delta = (uint128(blockDiffEthFee) * uint128(effectiveVUnits)) / BPS_DENOMINATOR;
            operator.ethSnapshot.balance = operator.ethSnapshot.balance.add(PackedETH.wrap(_safeUint64(delta)));
        }
        operator.ethSnapshot.block = currentBlock;
    }

    /**
     * @notice Returns default ETH fee for operators
     * @return Default ETH fee
     */
    function defaultOperatorEthFee() internal pure returns (PackedETH) {
        return PackedETHLib.pack(DEFAULT_OPERATOR_ETH_FEE);
    }

    /**
     * @notice Checks operator owner
     * @param operator Operator storage reference
     */
    function checkOwner(ISSVNetworkCore.Operator storage operator) internal view {
        if (operator.snapshot.block == 0 && operator.ethSnapshot.block == 0) {
            revert ISSVNetworkCore.OperatorDoesNotExist();
        }
        if (operator.owner != msg.sender) revert ISSVNetworkCore.CallerNotOwnerWithData(msg.sender, operator.owner);
    }

    /**
     * @notice Ensures ETH defaults for operator
     * @param operator Operator storage reference
     */
    function ensureETHDefaults(ISSVNetworkCore.Operator storage operator, uint64 operatorId) internal {
        if (operator.ethSnapshot.block == 0) {
            operator.ethSnapshot.block = uint32(block.number);
            operator.ethSnapshot.balance = PACKED_ETH_ZERO;

            if (operator.ethFee.eq(PACKED_ETH_ZERO) && operator.fee.neq(PACKED_SSV_ZERO)) {
                operator.ethFee = defaultOperatorEthFee();
                emit ISSVOperators.OperatorFeeExecuted(operator.owner, operatorId, block.number, DEFAULT_OPERATOR_ETH_FEE);
            }
        }
        // we don't want to revert here because this will block the migration flow
    }

    /**
     * @notice Validates operator state for validator registration
     * @param operator Operator storage reference
     */
    function ensureOperatorExist(ISSVNetworkCore.Operator storage operator) internal view {
        if (operator.owner == address(0) ||
            (operator.ethSnapshot.block == 0 && operator.snapshot.block == 0)) {
            revert ISSVNetworkCore.OperatorDoesNotExist();
        }
    }

    /**
     * @notice Updates cluster operators on registration
     * @param operatorIds Operator IDs
     * @param deltaValidatorCount Validator count delta
     * @param s Storage data
     * @param sp Storage protocol
     * @return cumulativeIndex Cumulative index
     * @return cumulativeFee Cumulative fee
     */
    function updateClusterOperatorsOnRegistration(
        uint64[] memory operatorIds,
        uint32 deltaValidatorCount,
        StorageData storage s,
        StorageProtocol storage sp
    ) internal returns (uint64 cumulativeIndex, uint64 cumulativeFee) {
        uint256 operatorsLength = operatorIds.length;

        uint256 blockIndex;
        uint256 lastBlockIndex = ~uint256(0); // Use an invalid block index as the initial value
        uint256 currentWhitelistedMask;

        for (uint256 i; i < operatorsLength; ++i) {
            uint64 operatorId = operatorIds[i];

            if (i + 1 < operatorsLength) {
                if (operatorId > operatorIds[i + 1]) {
                    revert ISSVNetworkCore.UnsortedOperatorsList();
                } else if (operatorId == operatorIds[i + 1]) {
                    revert ISSVNetworkCore.OperatorsListNotUnique();
                }
            }
            ISSVNetworkCore.Operator storage operatorSt = s.operators[operatorId];
            ensureOperatorExist(operatorSt);

            ensureETHDefaults(operatorSt, operatorId);
            ISSVNetworkCore.Operator memory operator = operatorSt;
            // check if the pending operator is whitelisted (must be backward compatible)
            if (operator.whitelisted) {
                // Handle bitmap-based whitelisting
                blockIndex = operatorId >> 8;
                if (blockIndex != lastBlockIndex) {
                    currentWhitelistedMask = s.addressWhitelistedForOperators[msg.sender][blockIndex];
                    lastBlockIndex = blockIndex;
                }

                // if msg.sender is not whitelisted via bitmap, check for legacy whitelist/whitelisting contract
                if (currentWhitelistedMask & (1 << (operatorId & 0xFF)) == 0) {
                    address whitelistedAddress = s.operatorsWhitelist[operatorId];
                    if (whitelistedAddress == address(0)) {
                        // msg.sender is not whitelisted via bitmap or legacy whitelist/whitelisting contract
                        revert ISSVNetworkCore.CallerNotWhitelistedWithData(operatorId);
                    }
                    // Legacy address & whitelisting contract check
                    if (whitelistedAddress != msg.sender) {
                        // Check if whitelistedAddress is a valid whitelisting contract and if msg.sender is whitelisted by it
                        // For non-whitelisting contracts, check if msg.sender is whitelisted (EOAs or generic contracts)
                        if (
                            !OperatorLib.isWhitelistingContract(whitelistedAddress) ||
                            !ISSVWhitelistingContract(whitelistedAddress).isWhitelisted(msg.sender, operatorId)
                        ) {
                            revert ISSVNetworkCore.CallerNotWhitelistedWithData(operatorId);
                        }
                    }
                }
            }

            updateSnapshot(operator, operatorId);
            if ((operator.ethValidatorCount += deltaValidatorCount) > sp.validatorsPerOperatorLimit) {
                revert ISSVNetworkCore.ExceedValidatorLimitWithData(operatorId);
            }
            cumulativeFee += PackedETH.unwrap(operator.ethFee);
            cumulativeIndex += operator.ethSnapshot.index;

            s.operators[operatorId] = operator;
        }
    }

    /**
     * @notice Updates ETH cluster operators
     * @param operatorIds Operator IDs
     * @param increaseValidatorCount Increase flag
     * @param deltaValidatorCount Validator count delta
     * @param s Storage data
     * @param sp Storage protocol
     * @return cumulativeIndex Cumulative index
     * @return cumulativeFee Cumulative fee
     */
    function updateClusterOperators(
        uint64[] memory operatorIds,
        bool increaseValidatorCount,
        uint32 deltaValidatorCount,
        StorageData storage s,
        StorageProtocol storage sp
    ) internal returns (uint64 cumulativeIndex, uint64 cumulativeFee) {
        uint256 operatorsLength = operatorIds.length;
        for (uint256 i; i < operatorsLength; ++i) {
            uint64 operatorId = operatorIds[i];
            ISSVNetworkCore.Operator storage operator = s.operators[operatorId];

            // only update active operators (block != 0)
            // removed operators have block == 0 and contribute their preserved index
            if (operator.ethSnapshot.block != 0) {
                updateSnapshotSt(operator, operatorId);

                if (increaseValidatorCount) {
                    if ((operator.ethValidatorCount += deltaValidatorCount) > sp.validatorsPerOperatorLimit) {
                        revert ISSVNetworkCore.ExceedValidatorLimitWithData(operatorId);
                    }
                } else {
                    operator.ethValidatorCount -= deltaValidatorCount;
                }

                cumulativeFee += PackedETH.unwrap(operator.ethFee);
            }
            cumulativeIndex += operator.ethSnapshot.index;
        }
    }

    /**
     * @notice Updates cluster operators on reactivation
     * @param operatorIds Operator IDs
     * @param deltaValidatorCount Validator count delta
     * @param clusterDeviation Cluster deviation
     * @param s Storage data
     * @param sp Storage protocol
     * @param seb Storage EB
     * @return cumulativeIndex Cumulative index
     * @return cumulativeFee Cumulative fee
     */
    function updateClusterOperatorsOnReactivation(
        uint64[] memory operatorIds,
        uint32 deltaValidatorCount,
        uint64 clusterDeviation,
        StorageData storage s,
        StorageProtocol storage sp,
        StorageEB storage seb
    ) internal returns (uint64 cumulativeIndex, uint64 cumulativeFee) {
        uint256 operatorsLength = operatorIds.length;
        uint32 currentBlock = uint32(block.number);
        bool hasDeviation = sp.daoTotalEthVUnits != uint64(sp.ethDaoValidatorCount) * BPS_DENOMINATOR;

        for (uint256 i; i < operatorsLength; ++i) {
            uint64 operatorId = operatorIds[i];
            ISSVNetworkCore.Operator storage operator = s.operators[operatorId];

            if (operator.ethSnapshot.block != 0) {
                uint64 blockDiffEthFee = (currentBlock - operator.ethSnapshot.block) * PackedETH.unwrap(operator.ethFee);

                if (blockDiffEthFee != 0) {
                    operator.ethSnapshot.index += blockDiffEthFee;
                    uint64 effectiveVUnits;

                    if (hasDeviation) {
                        uint64 storedDeviation = seb.operatorEthVUnits[operatorId];
                        effectiveVUnits = storedDeviation + (uint64(operator.ethValidatorCount) * BPS_DENOMINATOR);
                    } else {
                        effectiveVUnits = uint64(operator.ethValidatorCount) * BPS_DENOMINATOR;
                    }

                    if (effectiveVUnits != 0) {
                        uint128 delta = (uint128(blockDiffEthFee) * uint128(effectiveVUnits)) / BPS_DENOMINATOR;
                        operator.ethSnapshot.balance = operator.ethSnapshot.balance.add(PackedETH.wrap(_safeUint64(delta)));
                    }
                }
                operator.ethSnapshot.block = currentBlock;

                if (clusterDeviation != 0) {
                    if (hasDeviation) {
                        uint64 storedDeviation = seb.operatorEthVUnits[operatorId];
                        seb.operatorEthVUnits[operatorId] = storedDeviation + clusterDeviation;
                    } else {
                        seb.operatorEthVUnits[operatorId] = clusterDeviation;
                    }
                }

                operator.ethValidatorCount += deltaValidatorCount;
                if (operator.ethValidatorCount > sp.validatorsPerOperatorLimit) {
                    revert ISSVNetworkCore.ExceedValidatorLimitWithData(operatorId);
                }

                cumulativeFee += PackedETH.unwrap(operator.ethFee);
            }
            cumulativeIndex += operator.ethSnapshot.index;
        }
    }

    /**
     * @notice Updates cluster operators on migration
     * @param operatorIds Operator IDs
     * @param validatorCount Validator count
     * @param s Storage data
     * @param sp Storage protocol
     * @param isClusterLiquidated Liquidated flag
     * @return cumulativeIndexSSV Cumulative index SSV
     * @return cumulativeIndexETH Cumulative index ETH
     * @return cumulativeFeeETH Cumulative fee ETH
     */
    function updateClusterOperatorsMigration(
        uint64[] memory operatorIds,
        uint32 validatorCount,
        StorageData storage s,
        StorageProtocol storage sp,
        bool isClusterLiquidated
    ) internal returns (uint64 cumulativeIndexSSV, uint64 cumulativeIndexETH, uint64 cumulativeFeeETH) {
        uint256 operatorsLength = operatorIds.length;
        for (uint256 i; i < operatorsLength; ++i) {
            uint64 operatorId = operatorIds[i];
            ISSVNetworkCore.Operator storage operator = s.operators[operatorId];

            if (operator.snapshot.block != 0) {
                updateSnapshotStSSV(operator);
                if (!isClusterLiquidated) {
                    operator.validatorCount -= validatorCount;
                }
            }
            cumulativeIndexSSV += operator.snapshot.index;

            // Removed operators (both blocks == 0) contribute their frozen index
            // but are not mutated (no validator count or fee changes)
            if (operator.snapshot.block != 0 || operator.ethSnapshot.block != 0) {
                if (operator.ethSnapshot.block == 0) {
                    // first-time ETH usage or migration
                    ensureETHDefaults(operator, operatorId);
                } else {
                    // already ETH operator
                    updateSnapshotSt(operator, operatorId);
                }

                // update ETH validator count for both new ETH-initialized and existing ETH-initialized operators
                if ((operator.ethValidatorCount += validatorCount) > sp.validatorsPerOperatorLimit) {
                    revert ISSVNetworkCore.ExceedValidatorLimitWithData(operatorId);
                }

                cumulativeFeeETH += PackedETH.unwrap(operator.ethFee);
            }
            cumulativeIndexETH += operator.ethSnapshot.index;
        }
    }

    /**
     * @notice Updates SSV cluster operators
     * @param operatorIds Operator IDs
     * @param increaseValidatorCount Increase flag
     * @param deltaValidatorCount Validator count delta
     * @param s Storage data
     * @param sp Storage protocol
     * @return cumulativeIndex Cumulative index
     * @return cumulativeFee Cumulative fee
     */
    function updateClusterOperatorsSSV(
        uint64[] memory operatorIds,
        bool increaseValidatorCount,
        uint32 deltaValidatorCount,
        StorageData storage s,
        StorageProtocol storage sp
    ) internal returns (uint64 cumulativeIndex, uint64 cumulativeFee) {
        uint256 operatorsLength = operatorIds.length;

        for (uint256 i; i < operatorsLength; ++i) {
            uint64 operatorId = operatorIds[i];

            ISSVNetworkCore.Operator storage operator = s.operators[operatorId];

            if (operator.snapshot.block != 0) {
                updateSnapshotStSSV(operator);
                if (!increaseValidatorCount) {
                    operator.validatorCount -= deltaValidatorCount;
                } else if ((operator.validatorCount += deltaValidatorCount) > sp.validatorsPerOperatorLimit) {
                    revert ISSVNetworkCore.ExceedValidatorLimitWithData(operatorId);
                }

                cumulativeFee += PackedSSV.unwrap(operator.fee);
            }

            cumulativeIndex += operator.snapshot.index;
        }
    }

    /**
     * @notice Updates multiple whitelists
     * @param whitelistAddresses Whitelist addresses
     * @param operatorIds Operator IDs
     * @param registerAddresses Register flag
     * @param s Storage data
     */
    function updateMultipleWhitelists(
        address[] calldata whitelistAddresses,
        uint64[] calldata operatorIds,
        bool registerAddresses,
        StorageData storage s
    ) internal {
        uint256 addressesLength = whitelistAddresses.length;
        if (addressesLength == 0) revert ISSVNetworkCore.InvalidWhitelistAddressesLength();

        checkOperatorsLength(operatorIds);

        // create the max number of masks that will be updated
        (uint256[] memory masks, uint256 startBlockIndex) = generateBlockMasks(operatorIds, true, s);
        uint256 endBlockIndex = startBlockIndex + masks.length;

        for (uint256 i; i < addressesLength; ++i) {
            address whitelistAddress = whitelistAddresses[i];
            checkZeroAddress(whitelistAddress);

            // If whitelistAddress is a custom contract, reverts only when registering addresses
            if (registerAddresses && isWhitelistingContract(whitelistAddress))
                revert ISSVNetworkCore.AddressIsWhitelistingContract(whitelistAddress);

            for (uint256 blockIndex = startBlockIndex; blockIndex < endBlockIndex; ++blockIndex) {
                // only update storage for updated masks
                uint256 mask = masks[blockIndex - startBlockIndex];
                if (mask != 0) {
                    if (registerAddresses) {
                        s.addressWhitelistedForOperators[whitelistAddress][blockIndex] |= mask;
                    } else {
                        s.addressWhitelistedForOperators[whitelistAddress][blockIndex] &= ~mask;
                    }
                }
            }
        }
    }

    /**
     * @notice Generates block masks for operators
     * @param operatorIds Operator IDs
     * @param checkOperatorsOwnership Ownership check flag
     * @param s Storage data
     * @return masks Block masks
     * @return startBlockIndex Start block index
     */
    function generateBlockMasks(
        uint64[] calldata operatorIds,
        bool checkOperatorsOwnership,
        StorageData storage s
    ) internal view returns (uint256[] memory masks, uint256 startBlockIndex) {
        uint256 operatorsLength = operatorIds.length;
        startBlockIndex = operatorIds[0] >> 8;

        // Create the masks array from startBlockIndex to the last block index
        masks = new uint256[]((operatorIds[operatorsLength - 1] >> 8) - startBlockIndex + 1);

        uint64 currentOperatorId;
        uint64 prevOperatorId;

        for (uint256 i; i < operatorsLength; ++i) {
            currentOperatorId = operatorIds[i];

            if (checkOperatorsOwnership) {
                checkOwner(s.operators[currentOperatorId]);
            }

            if (i > 0 && currentOperatorId <= prevOperatorId) {
                if (currentOperatorId == prevOperatorId) {
                    revert ISSVNetworkCore.OperatorsListNotUnique();
                }
                revert ISSVNetworkCore.UnsortedOperatorsList();
            }

            (uint256 blockIndex, uint256 bitPosition) = getBitmapIndexes(currentOperatorId);

            masks[blockIndex - startBlockIndex] |= (1 << bitPosition);
            prevOperatorId = currentOperatorId;
        }
    }

    /**
     * @notice Updates operator privacy status
     * @param operatorIds Operator IDs
     * @param setPrivate Private flag
     * @param s Storage data
     */
    function updatePrivacyStatus(uint64[] calldata operatorIds, bool setPrivate, StorageData storage s) internal {
        uint256 operatorsLength = checkOperatorsLength(operatorIds);

        ISSVNetworkCore.Operator storage operator;
        for (uint256 i; i < operatorsLength; ++i) {
            uint64 operatorId = operatorIds[i];
            operator = s.operators[operatorId];
            checkOwner(operator);

            operator.whitelisted = setPrivate;
        }
    }

    /**
     * @notice Gets bitmap indexes for operator
     * @param operatorId Operator ID
     * @return blockIndex Block index
     * @return bitPosition Bit position
     */
    function getBitmapIndexes(uint64 operatorId) internal pure returns (uint256 blockIndex, uint256 bitPosition) {
        blockIndex = operatorId >> 8; // Equivalent to operatorId / 256
        bitPosition = operatorId & 0xFF; // Equivalent to operatorId % 256
    }

    /**
     * @notice Checks for zero address
     * @param whitelistAddress Address to check
     */
    function checkZeroAddress(address whitelistAddress) internal pure {
        if (whitelistAddress == address(0)) revert ISSVNetworkCore.ZeroAddressNotAllowed();
    }

    /**
     * @notice Checks operator IDs length
     * @param operatorIds Operator IDs
     * @return operatorsLength Length
     */
    function checkOperatorsLength(uint64[] calldata operatorIds) internal pure returns (uint256 operatorsLength) {
        operatorsLength = operatorIds.length;
        if (operatorsLength == 0) revert ISSVNetworkCore.InvalidOperatorIdsLength();
    }

    /**
     * @notice Checks if address is whitelisting contract
     * @param whitelistingContract Contract address
     * @return True if whitelisting contract
     */
    function isWhitelistingContract(address whitelistingContract) internal view returns (bool) {
        return ERC165Checker.supportsInterface(whitelistingContract, type(ISSVWhitelistingContract).interfaceId);
    }
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
// OpenZeppelin Contracts (last updated v4.9.0) (proxy/utils/UUPSUpgradeable.sol)

pragma solidity ^0.8.0;

import "../../interfaces/draft-IERC1822Upgradeable.sol";
import "../ERC1967/ERC1967UpgradeUpgradeable.sol";
import {Initializable} from "./Initializable.sol";

/**
 * @dev An upgradeability mechanism designed for UUPS proxies. The functions included here can perform an upgrade of an
 * {ERC1967Proxy}, when this contract is set as the implementation behind such a proxy.
 *
 * A security mechanism ensures that an upgrade does not turn off upgradeability accidentally, although this risk is
 * reinstated if the upgrade retains upgradeability but removes the security mechanism, e.g. by replacing
 * `UUPSUpgradeable` with a custom implementation of upgrades.
 *
 * The {_authorizeUpgrade} function must be overridden to include access restriction to the upgrade mechanism.
 *
 * _Available since v4.1._
 */
abstract contract UUPSUpgradeable is Initializable, IERC1822ProxiableUpgradeable, ERC1967UpgradeUpgradeable {
    /// @custom:oz-upgrades-unsafe-allow state-variable-immutable state-variable-assignment
    address private immutable __self = address(this);

    /**
     * @dev Check that the execution is being performed through a delegatecall call and that the execution context is
     * a proxy contract with an implementation (as defined in ERC1967) pointing to self. This should only be the case
     * for UUPS and transparent proxies that are using the current contract as their implementation. Execution of a
     * function through ERC1167 minimal proxies (clones) would not normally pass this test, but is not guaranteed to
     * fail.
     */
    modifier onlyProxy() {
        require(address(this) != __self, "Function must be called through delegatecall");
        require(_getImplementation() == __self, "Function must be called through active proxy");
        _;
    }

    /**
     * @dev Check that the execution is not being performed through a delegate call. This allows a function to be
     * callable on the implementing contract but not through proxies.
     */
    modifier notDelegated() {
        require(address(this) == __self, "UUPSUpgradeable: must not be called through delegatecall");
        _;
    }

    function __UUPSUpgradeable_init() internal onlyInitializing {
    }

    function __UUPSUpgradeable_init_unchained() internal onlyInitializing {
    }
    /**
     * @dev Implementation of the ERC1822 {proxiableUUID} function. This returns the storage slot used by the
     * implementation. It is used to validate the implementation's compatibility when performing an upgrade.
     *
     * IMPORTANT: A proxy pointing at a proxiable contract should not be considered proxiable itself, because this risks
     * bricking a proxy that upgrades to it, by delegating to itself until out of gas. Thus it is critical that this
     * function revert if invoked through a proxy. This is guaranteed by the `notDelegated` modifier.
     */
    function proxiableUUID() external view virtual override notDelegated returns (bytes32) {
        return _IMPLEMENTATION_SLOT;
    }

    /**
     * @dev Upgrade the implementation of the proxy to `newImplementation`.
     *
     * Calls {_authorizeUpgrade}.
     *
     * Emits an {Upgraded} event.
     *
     * @custom:oz-upgrades-unsafe-allow-reachable delegatecall
     */
    function upgradeTo(address newImplementation) public virtual onlyProxy {
        _authorizeUpgrade(newImplementation);
        _upgradeToAndCallUUPS(newImplementation, new bytes(0), false);
    }

    /**
     * @dev Upgrade the implementation of the proxy to `newImplementation`, and subsequently execute the function call
     * encoded in `data`.
     *
     * Calls {_authorizeUpgrade}.
     *
     * Emits an {Upgraded} event.
     *
     * @custom:oz-upgrades-unsafe-allow-reachable delegatecall
     */
    function upgradeToAndCall(address newImplementation, bytes memory data) public payable virtual onlyProxy {
        _authorizeUpgrade(newImplementation);
        _upgradeToAndCallUUPS(newImplementation, data, true);
    }

    /**
     * @dev Function that should revert when `msg.sender` is not authorized to upgrade the contract. Called by
     * {upgradeTo} and {upgradeToAndCall}.
     *
     * Normally, this function will use an xref:access.adoc[access control] modifier such as {Ownable-onlyOwner}.
     *
     * ```solidity
     * function _authorizeUpgrade(address) internal override onlyOwner {}
     * ```
     */
    function _authorizeUpgrade(address newImplementation) internal virtual;

    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[50] private __gap;
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.20;

/**
 * @title SSV Whitelisting Contract Interface
 * @author SSV Labs
 */
interface ISSVWhitelistingContract {
    /**
     * @notice Checks if the caller is whitelisted
     * @param account The account that is being checked for whitelisting
     * @param operatorId The SSV Operator Id which is being checked
     */
    function isWhitelisted(address account, uint256 operatorId) external view returns (bool);
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {ISSVNetworkCore} from "../interfaces/ISSVNetworkCore.sol";
import {PackedSSV, PackedETH, BPS_DENOMINATOR, _safeUint64} from "../libraries/SSVCoreTypes.sol";
import {PackedSSVLib, PackedETHLib} from "../libraries/SSVPackedLib.sol";
import {StorageProtocol} from "./storage/SSVStorageProtocol.sol";

/**
 * @title SSV Protocol Library
 * @author SSV Labs
 * @notice Library functions for managing SSV protocol including network fees, DAO earnings and validator updates
 */
library ProtocolLib {
    using PackedETHLib for PackedETH;

    /**
     * @notice Returns current network fee index
     * @param sp Storage protocol
     * @return Current network fee index
     */
    function currentNetworkFeeIndex(StorageProtocol storage sp) internal view returns (uint64) {
        return sp.ethNetworkFeeIndex + uint64(block.number - sp.ethNetworkFeeIndexBlockNumber) * PackedETH.unwrap(sp.ethNetworkFee);
    }

    /**
     * @notice Returns current SSV network fee index
     * @param sp Storage protocol
     * @return Current SSV network fee index
     */
    function currentNetworkFeeIndexSSV(StorageProtocol storage sp) internal view returns (uint64) {
        return sp.networkFeeIndex + uint64(block.number - sp.networkFeeIndexBlockNumber) * PackedSSV.unwrap(sp.networkFee);
    }

    /**
     * @notice Updates ETH network fee
     * @param sp Storage protocol
     * @param fee New fee
     */
    function updateNetworkFee(StorageProtocol storage sp, uint256 fee) internal {
        updateDAOEarnings(sp);

        sp.ethNetworkFeeIndex = currentNetworkFeeIndex(sp);
        sp.ethNetworkFeeIndexBlockNumber = uint32(block.number);
        sp.ethNetworkFee = PackedETHLib.pack(fee);
    }

    /**
     * @notice Updates SSV network fee
     * @param sp Storage protocol
     * @param fee New fee
     */
    function updateNetworkFeeSSV(StorageProtocol storage sp, uint256 fee) internal {
        updateDAOEarningsSSV(sp);

        sp.networkFeeIndex = currentNetworkFeeIndexSSV(sp);
        sp.networkFeeIndexBlockNumber = uint32(block.number);
        sp.networkFee = PackedSSVLib.pack(fee);
    }

    /**
     * @notice Updates DAO earnings
     * @param sp Storage protocol
     */
    function updateDAOEarnings(StorageProtocol storage sp) internal {
        sp.ethDaoBalance = networkTotalEarnings(sp);
        sp.ethDaoIndexBlockNumber = uint32(block.number);
    }

    /**
     * @notice Updates SSV DAO earnings
     * @param sp Storage protocol
     */
    function updateDAOEarningsSSV(StorageProtocol storage sp) internal {
        sp.daoBalance = networkTotalEarningsSSV(sp);
        sp.daoIndexBlockNumber = uint32(block.number);
    }

    /**
     * @notice Returns total network earnings
     * @param sp Storage protocol
     * @return Total earnings
     */
    function networkTotalEarnings(StorageProtocol storage sp) internal view returns (PackedETH) {
        uint128 units = sp.daoTotalEthVUnits;
        uint128 idx = uint64(block.number) - sp.ethDaoIndexBlockNumber;

        uint128 earningsUnits = (idx * PackedETH.unwrap(sp.ethNetworkFee) * units) / BPS_DENOMINATOR;
        return sp.ethDaoBalance.add(PackedETH.wrap(_safeUint64(earningsUnits)));
    }

    /**
     * @notice Returns total SSV network earnings
     * @param sp Storage protocol
     * @return Total earnings
     */
    function networkTotalEarningsSSV(StorageProtocol storage sp) internal view returns (PackedSSV) {
        return PackedSSV.wrap(PackedSSV.unwrap(sp.daoBalance) + (uint64(block.number) - sp.daoIndexBlockNumber) * PackedSSV.unwrap(sp.networkFee) * sp.daoValidatorCount);
    }

    /**
     * @notice Updates DAO validator count
     * @param sp Storage protocol
     * @param increaseValidatorCount Increase flag
     * @param deltaValidatorCount Validator count delta
     */
    function updateDAO(StorageProtocol storage sp, bool increaseValidatorCount, uint32 deltaValidatorCount) internal {
        updateDAOEarnings(sp);
        uint64 vUnitsDelta = uint64(deltaValidatorCount) * BPS_DENOMINATOR;
        if (!increaseValidatorCount) {
            sp.ethDaoValidatorCount -= deltaValidatorCount;
            sp.daoTotalEthVUnits -= vUnitsDelta;
        } else {
            if ((sp.ethDaoValidatorCount += deltaValidatorCount) > type(uint32).max) {
                revert ISSVNetworkCore.MaxValueExceeded();
            } 
            sp.daoTotalEthVUnits += vUnitsDelta;
        }
    }

    /**
     * @notice Updates SSV DAO validator count
     * @param sp Storage protocol
     * @param increaseValidatorCount Increase flag
     * @param deltaValidatorCount Validator count delta
     */
    function updateDAOSSV(StorageProtocol storage sp, bool increaseValidatorCount, uint32 deltaValidatorCount) internal {
        updateDAOEarningsSSV(sp);
        if (!increaseValidatorCount) {
            sp.daoValidatorCount -= deltaValidatorCount;
        } else if ((sp.daoValidatorCount += deltaValidatorCount) > type(uint32).max) {
            revert ISSVNetworkCore.MaxValueExceeded();
        }
    }

    /**
     * @notice Updates DAO ETH v units
     * @param sp Storage protocol
     * @param oldVUnits Old v units
     * @param newVUnits New v units
     */
    function updateDAOEthVUnits(StorageProtocol storage sp, uint64 oldVUnits, uint64 newVUnits) internal {
        updateDAOEarnings(sp);  // Settle ETH earnings first

        if (newVUnits > oldVUnits) {
            sp.daoTotalEthVUnits += newVUnits - oldVUnits;
        } else {
            sp.daoTotalEthVUnits -= oldVUnits - newVUnits;
        }
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v4.9.0) (access/Ownable2Step.sol)

pragma solidity ^0.8.0;

import "./OwnableUpgradeable.sol";
import {Initializable} from "../proxy/utils/Initializable.sol";

/**
 * @dev Contract module which provides access control mechanism, where
 * there is an account (an owner) that can be granted exclusive access to
 * specific functions.
 *
 * By default, the owner account will be the one that deploys the contract. This
 * can later be changed with {transferOwnership} and {acceptOwnership}.
 *
 * This module is used through inheritance. It will make available all functions
 * from parent (Ownable).
 */
abstract contract Ownable2StepUpgradeable is Initializable, OwnableUpgradeable {
    address private _pendingOwner;

    event OwnershipTransferStarted(address indexed previousOwner, address indexed newOwner);

    function __Ownable2Step_init() internal onlyInitializing {
        __Ownable_init_unchained();
    }

    function __Ownable2Step_init_unchained() internal onlyInitializing {
    }
    /**
     * @dev Returns the address of the pending owner.
     */
    function pendingOwner() public view virtual returns (address) {
        return _pendingOwner;
    }

    /**
     * @dev Starts the ownership transfer of the contract to a new account. Replaces the pending transfer if there is one.
     * Can only be called by the current owner.
     */
    function transferOwnership(address newOwner) public virtual override onlyOwner {
        _pendingOwner = newOwner;
        emit OwnershipTransferStarted(owner(), newOwner);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`) and deletes any pending owner.
     * Internal function without access restriction.
     */
    function _transferOwnership(address newOwner) internal virtual override {
        delete _pendingOwner;
        super._transferOwnership(newOwner);
    }

    /**
     * @dev The new owner accepts the ownership transfer.
     */
    function acceptOwnership() public virtual {
        address sender = _msgSender();
        require(pendingOwner() == sender, "Ownable2Step: caller is not the new owner");
        _transferOwnership(sender);
    }

    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[49] private __gap;
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {ISSVNetworkCore} from "../../interfaces/ISSVNetworkCore.sol";
import {Counters} from "@openzeppelin/contracts/utils/Counters.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

enum SSVModules {
    SSV_OPERATORS,
    SSV_CLUSTERS,
    SSV_DAO,
    SSV_VIEWS,
    SSV_OPERATORS_WHITELIST,
    SSV_STAKING,
    SSV_VALIDATORS
}

/// @title SSV Network Storage Data
/// @notice Represents all operational state required by the SSV Network
struct StorageData {
    /// @notice Maps each validator's public key to its hashed representation of: operator Ids used by the validator and active / inactive flag (uses LSB)
    mapping(bytes32 => bytes32) validatorPKs;
    /// @notice Maps each cluster's bytes32 identifier to its hashed representation of ISSVNetworkCore.Cluster
    mapping(bytes32 => bytes32) clusters;
    /// @notice Maps each operator's public key to its corresponding ID
    mapping(bytes32 => uint64) operatorsPKs;
    /// @notice Maps each SSVModules' module to its corresponding contract address
    mapping(SSVModules => address) ssvContracts;
    /// @notice Operators' whitelist: Maps each operator's ID to a whitelisting contract
    mapping(uint64 => address) operatorsWhitelist;
    /// @notice Maps each operator's ID to its corresponding operator fee change request data
    mapping(uint64 => ISSVNetworkCore.OperatorFeeChangeRequest) operatorFeeChangeRequests;
    /// @notice Maps each operator's ID to its corresponding operator data
    mapping(uint64 => ISSVNetworkCore.Operator) operators;
    /// @notice The SSV token used within the network (fees, rewards)
    IERC20 token;
    /// @notice Counter keeping track of the last Operator ID issued
    Counters.Counter lastOperatorId;
    /// @notice Operators' whitelist: Maps each whitelisted address to a list of operators
    /// @notice that are whitelisted for that address using bitmaps
    /// @dev The nested mapping's key represents a uint256 slot to handle more than 256 operators per address
    mapping(address => mapping(uint256 => uint256)) addressWhitelistedForOperators;
    /// @notice Maps each cluster's bytes32 identifier to its hashed representation of ISSVNetworkCore.Cluster for eth
    mapping(bytes32 => bytes32) ethClusters;
}

library SSVStorage {
    uint256 private constant SSV_STORAGE_POSITION = uint256(keccak256("ssv.network.storage.main")) - 1;

    function load() internal pure returns (StorageData storage sd) {
        uint256 position = SSV_STORAGE_POSITION;
        assembly {
            sd.slot := position
        }
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {ISSVNetworkCore} from "../interfaces/ISSVNetworkCore.sol";
import {SSVModules, SSVStorage} from "./storage/SSVStorage.sol";

/**
 * @title SSV Core Library
 * @author SSV Labs
 * @notice Library with core utility functions for SSV network including transfers, contract checks and module upgrades
 */
library CoreLib {
    /**
     * @dev Emitted when a module is upgraded
     * @param moduleId The module ID
     * @param moduleAddress The new module address
     */
    event ModuleUpgraded(SSVModules indexed moduleId, address moduleAddress);

    /**
     * @notice Returns the contract version
     * @return Version string
     */
    function getVersion() internal pure returns (string memory) {
        return "v2.0.0";
    }

    /**
     * @notice Transfers ETH to recipient
     * @param to Recipient address
     * @param amount Amount to transfer
     */
    function transferBalance(address to, uint256 amount) internal {
        (bool success, ) = payable(to).call{value: amount}("");
        if(!success){
            revert ISSVNetworkCore.ETHTransferFailed();
        }
    }

    /**
     * @notice Transfers tokens to recipient
     * @param to Recipient address
     * @param amount Amount to transfer
     */
    function transferTokenBalance(address to, uint256 amount) internal {
        if (!SSVStorage.load().token.transfer(to, amount)) {
            revert ISSVNetworkCore.TokenTransferFailed();
        }
    }

    /**
     * @dev Returns true if `account` is a contract.
     *
     * [IMPORTANT]
     * ====
     * It is unsafe to assume that an address for which this function returns
     * false is an externally-owned account (EOA) and not a contract.
     *
     * Among others, `isContract` will return false for the following
     * types of addresses:
     *
     *  - an externally-owned account
     *  - a contract in construction
     *  - an address where a contract will be created
     *  - an address where a contract lived, but was destroyed
     * ====
     */
    function isContract(address account) internal view returns (bool) {
        if (account == address(0)) {
            return false;
        }
        // This method relies on extcodesize, which returns 0 for contracts in
        // construction, since the code is only stored at the end of the
        // constructor execution.

        uint256 size;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            size := extcodesize(account)
        }
        return size > 0;
    }

    /**
     * @notice Sets contract address for a module
     * @param moduleId Module ID
     * @param moduleAddress New module address
     */
    function setModuleContract(SSVModules moduleId, address moduleAddress) internal {
        if (!isContract(moduleAddress)) revert ISSVNetworkCore.TargetModuleDoesNotExistWithData(uint8(moduleId));

        SSVStorage.load().ssvContracts[moduleId] = moduleAddress;
        emit ModuleUpgraded(moduleId, moduleAddress);
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

struct ClusterEBSnapshot {
    uint64 vUnits;
    uint64 lastRootBlockNum;
    uint64 lastUpdateBlock;
}

struct StorageEB {
    /// @notice Maps block to EB roots
    mapping(uint64 => bytes32) ebRoots;
    /// @notice Maps cluster ID to EB snapshot
    mapping(bytes32 => ClusterEBSnapshot) clusterEB;
    /// @notice Maps operator ID to ETH vUnits
    mapping(uint64 => uint64) operatorEthVUnits;
    /// @notice Latest block number where EB was committed
    uint64 latestCommittedBlock;
    /// @notice Minimum blocks between updates
    uint32 minBlocksBetweenUpdates;
    /// @notice Counts root commitments (accumulated weight) per commitment key (encoded root and block)
    mapping(bytes32 => uint256) rootCommitments;
    /// @notice Tracks if an oracle ID has voted for a specific commitment key
    mapping(bytes32 => mapping(uint32 => bool)) hasVoted;
    /// @notice Frozen voting supply (truncated to oracle-count divisibility) at the first vote of each commitment round
    mapping(bytes32 => uint256) roundFrozenSupply;
}

library SSVStorageEB {
    uint256 private constant SSV_STORAGE_POSITION = uint256(keccak256("ssv.network.storage.eb")) - 1;

    function load() internal pure returns (StorageEB storage seb) {
        uint256 position = SSV_STORAGE_POSITION;
        assembly {
            seb.slot := position
        }
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {PackedSSV, PackedETH} from "../SSVCoreTypes.sol";

/// @title SSV Network Storage Protocol
/// @notice Represents the operational settings and parameters required by the SSV Network
struct StorageProtocol {
    /// @notice The block number when the network fee index was last updated
    uint32 networkFeeIndexBlockNumber;
    /// @notice The count of validators governed by the DAO
    uint32 daoValidatorCount;
    /// @notice The block number when the DAO index was last updated
    uint32 daoIndexBlockNumber;
    /// @notice The maximum limit of validators per operator
    uint32 validatorsPerOperatorLimit;
    /// @notice The current network fee value
    PackedSSV networkFee;
    /// @notice The current network fee index value
    uint64 networkFeeIndex;
    /// @notice The current balance of the DAO
    PackedSSV daoBalance;
    /// @notice The minimum number of blocks before a liquidation event can be triggered for SSV cluster
    uint64 minimumBlocksBeforeLiquidationSSV;
    /// @notice The minimum collateral required for liquidation of SSV clusters
    PackedSSV minimumLiquidationCollateralSSV;
    /// @notice The period in which an operator can declare a fee change
    uint64 declareOperatorFeePeriod;
    /// @notice The period in which an operator fee change can be executed
    uint64 executeOperatorFeePeriod;
    /// @notice The maximum increase in operator fee that is allowed (percentage)
    uint64 operatorMaxFeeIncrease;
    /// @notice The maximum value in operator fee that is allowed (SSV)
    uint64 operatorMaxFeeSSV;

    // ETH 
    /// @notice The block number when the network fee index was last updated for eth
    uint32 ethNetworkFeeIndexBlockNumber;
    /// @notice The count of validators governed by the DAO for eth clusters
    uint32 ethDaoValidatorCount;
    /// @notice The block number when the DAO index was last updated for eth
    uint32 ethDaoIndexBlockNumber;
    /// @notice The current network fee value for eth clusters
    PackedETH ethNetworkFee;
    /// @notice The current network fee index value for eth clusters
    uint64 ethNetworkFeeIndex;
    /// @notice The current balance of the DAO for eth clusters
    PackedETH ethDaoBalance;
    /// @notice The minimum collateral required for liquidation
    PackedETH minimumLiquidationCollateral;
    /// @notice The minimum number of blocks before a liquidation event can be triggered
    uint64 minimumBlocksBeforeLiquidation;
    /// @notice The maximum value in operator fee that is allowed (ETH)
    PackedETH operatorMaxFee;

    // EB
    /// @notice The current total ETH vUnits
    uint64 daoTotalEthVUnits;
    /// @notice The minimum operator ETH fee (DAO-governed)
    PackedETH minimumOperatorEthFee;
}

library SSVStorageProtocol {
    uint256 private constant SSV_STORAGE_POSITION = uint256(keccak256("ssv.network.storage.protocol")) - 1;

    function load() internal pure returns (StorageProtocol storage sd) {
        uint256 position = SSV_STORAGE_POSITION;
        assembly {
            sd.slot := position
        }
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v4.9.4) (utils/Context.sol)

pragma solidity ^0.8.0;
import {Initializable} from "../proxy/utils/Initializable.sol";

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
abstract contract ContextUpgradeable is Initializable {
    function __Context_init() internal onlyInitializing {
    }

    function __Context_init_unchained() internal onlyInitializing {
    }
    function _msgSender() internal view virtual returns (address) {
        return msg.sender;
    }

    function _msgData() internal view virtual returns (bytes calldata) {
        return msg.data;
    }

    function _contextSuffixLength() internal view virtual returns (uint256) {
        return 0;
    }

    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[50] private __gap;
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.20;

import {ISSVNetworkCore} from "./ISSVNetworkCore.sol";
import {MAX_DELEGATION_SLOTS} from "../libraries/storage/SSVStorageStaking.sol";

/**
 * @title SSV Views Types Interface
 * @author SSV Labs
 * @notice Interface providing strict data types to be used as return values in SSV Views getters
 */
interface ISSVViewsTypes {
    /// @notice Contains data about a declared (pending) operator fee change
    struct OperatorDeclaredFeeData {
        /// @dev Whether the operator has an active fee declaration
        bool isFeeDeclared;
        /// @dev The fee value that was declared
        uint256 fee;
        /// @dev Timestamp when the approval window for this declaration begins
        uint64 approvalBeginTime;
        /// @dev Timestamp when the approval window for this declaration ends
        uint64 approvalEndTime;
    }

    /// @notice Contains core information about an operator
    struct OperatorData {
        /// @dev The address that owns and manages the operator
        address owner;
        /// @dev The current fee charged by the operator
        uint256 fee;
        /// @dev The number of validators currently registered to this operator
        uint32 validatorCount;
        /// @dev The address whitelisted for this operator
        address whitelistedAddress;
        /// @dev Whether the operator is private
        bool isPrivate;
        /// @dev Whether the operator is currently active
        bool isActive;
    }

    /// @notice Contains the time periods used for operator fee change workflow
    struct OperatorFeePeriodsData {
        /// @dev Duration (in seconds) of the declaration period
        uint64 declarePeriod;
        /// @dev Duration (in seconds) of the approval/execution period
        uint64 executePeriod;
    }

    /// @notice Represents a single pending unstake request
    struct UnstakeRequestsData {
        /// @dev The amount of SSV requested to be unstaked
        uint256 amount;
        /// @dev Timestamp after which the unstaked amount becomes withdrawable
        uint256 unlockTime;
    }
}

/**
 * @title SSV Views Interface
 * @author SSV Labs
 * @notice Interface providing view functions to retrieve network state, operator data, validator status, cluster information, fees, and staking details
 */
interface ISSVViews is ISSVNetworkCore, ISSVViewsTypes {
    /**
     * @notice Returns whether a validator is active
     * @param owner Owner of the validator
     * @param publicKey Validator public key
     * @return active True if validator exists and is active
     */
    function getValidator(address owner, bytes calldata publicKey) external view returns (bool);

    /**
     * @notice Returns the current ETH fee of an operator
     * @param operatorId The operator ID
     * @return fee Current operator fee in ETH
     */
    function getOperatorFee(uint64 operatorId) external view returns (uint256 fee);

    /**
     * @notice Returns the legacy SSV fee of an operator
     * @param operatorId The operator ID
     * @return fee Current operator fee in SSV
     */
    function getOperatorFeeSSV(uint64 operatorId) external view returns (uint256 fee);

    /**
     * @notice Gets the declared operator fee
     * @param operatorId The ID of the operator
     * @return data Declaration data
     */
    function getOperatorDeclaredFee(
        uint64 operatorId
    ) external view returns (OperatorDeclaredFeeData memory);

    /**
     * @notice Gets operator details by ID
     * @param operatorId The ID of the operator
     * @return The struct with operator details
     */
    function getOperatorById(
        uint64 operatorId
    )
        external
        view
        returns (OperatorData memory);

    /**
     * @notice Gets legacy SSV operator details by ID
     * @param operatorId The ID of the operator
     * @return The struct with operator details
     */
    function getOperatorByIdSSV(
        uint64 operatorId
    )
        external
        view
        returns (OperatorData memory);

    /**
     * @notice Returns which operators have the given address whitelisted
     * @param operatorIds List of operator IDs to check
     * @param whitelistedAddress Address to check
     * @return whitelistedOperatorIds List of operators where address is whitelisted
     */
    function getWhitelistedOperators(
        uint64[] calldata operatorIds,
        address whitelistedAddress
    ) external view returns (uint64[] memory whitelistedOperatorIds);

    /**
     * @notice Checks if an address is a valid whitelisting contract
     * @param contractAddress Address to check
     * @return isWhitelistingContract True if address implements ISSVWhitelistingContract
     */
    function isWhitelistingContract(address contractAddress) external view returns (bool);

    /**
     * @notice Checks if an address is whitelisted in a specific whitelisting contract
     * @param addressToCheck Address to verify
     * @param operatorId Operator ID (usage depends on contract implementation)
     * @param whitelistingContract Whitelisting contract address
     * @return isWhitelisted Whether the address is whitelisted
     */
    function isAddressWhitelistedInWhitelistingContract(
        address addressToCheck,
        uint256 operatorId,
        address whitelistingContract
    ) external view returns (bool isWhitelisted);

    /**
     * @notice Checks if a cluster is eligible for liquidation
     * @param owner Cluster owner
     * @param operatorIds Operator IDs in the cluster
     * @param cluster Cluster data
     * @return isLiquidatable True if cluster can be liquidated
     */
    function isLiquidatable(
        address owner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view returns (bool isLiquidatable);

    /**
     * @notice Checks if a legacy SSV cluster is eligible for liquidation
     * @param owner Cluster owner
     * @param operatorIds Operator IDs in the cluster
     * @param cluster Cluster data
     * @return isLiquidatable True if cluster can be liquidated
     */
    function isLiquidatableSSV(
        address owner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view returns (bool isLiquidatable);

    /**
     * @notice Checks if a cluster is already liquidated
     * @param owner Cluster owner
     * @param operatorIds Operator IDs in the cluster
     * @param cluster Cluster data
     * @return isLiquidated True if cluster is liquidated
     */
    function isLiquidated(
        address owner,
        uint64[] memory operatorIds,
        Cluster memory cluster
    ) external view returns (bool isLiquidated);

    /**
     * @notice Returns the current burn rate of a cluster
     * @param owner Cluster owner
     * @param operatorIds Operator IDs in the cluster
     * @param cluster Cluster data
     * @return burnRate Current burn rate in SSV per block
     */
    function getBurnRate(
        address owner,
        uint64[] memory operatorIds,
        Cluster memory cluster
    ) external view returns (uint256 burnRate);

    /**
     * @notice Returns the burn rate of a legacy SSV cluster
     * @param owner Cluster owner
     * @param operatorIds Operator IDs in the cluster
     * @param cluster Cluster data
     * @return burnRate Current burn rate in SSV per block
     */
    function getBurnRateSSV(
        address owner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view returns (uint256 burnRate);

    /**
     * @notice Returns accumulated operator earnings (ETH)
     * @param operatorId The operator ID
     * @return earnings Total ETH earnings
     */
    function getOperatorEarnings(uint64 operatorId) external view returns (uint256 earnings);

    /**
     * @notice Returns accumulated operator earnings (legacy SSV)
     * @param operatorId The operator ID
     * @return earnings Total SSV earnings
     */
    function getOperatorEarningsSSV(uint64 operatorId) external view returns (uint256 earnings);

    /**
     * @notice Returns the balance of a cluster
     * @param owner Cluster owner
     * @param operatorIds Operator IDs in the cluster
     * @param cluster Cluster data
     * @return balance Cluster balance in ETH
     */
    function getBalance(
        address owner,
        uint64[] memory operatorIds,
        Cluster memory cluster
    ) external view returns (uint256 balance);

    /**
     * @notice Returns the balance of a legacy SSV cluster
     * @param owner Cluster owner
     * @param operatorIds Operator IDs in the cluster
     * @param cluster Cluster data
     * @return balance Cluster balance in SSV
     */
    function getBalanceSSV(
        address owner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view returns (uint256 balance);

    /**
     * @notice Returns the effective balance of a cluster
     * @param owner Cluster owner
     * @param operatorIds Operator IDs in the cluster
     * @param cluster Cluster data
     * @return effectiveBalance Effective balance
     */
    function getEffectiveBalance(
        address owner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external view returns (uint32 effectiveBalance);

    /**
     * @notice Returns the asset type/version of a cluster
     * @param owner Cluster owner
     * @param operatorIds Operator IDs in the cluster
     * @return version Cluster version (ETH or SSV)
     */
    function getClusterAssetType(
        address owner,
        uint64[] calldata operatorIds
    ) external view returns (uint8 version);

    /**
     * @notice Returns the current network fee
     * @return networkFee Current network fee in ETH
     */
    function getNetworkFee() external view returns (uint256 networkFee);

    /**
     * @notice Returns the total network earnings
     * @return networkEarnings Total network earnings in ETH
     */
    function getNetworkEarnings() external view returns (uint256 networkEarnings);

    /**
     * @notice Returns the legacy network fee (SSV)
     * @return networkFee Current network fee in SSV
     */
    function getNetworkFeeSSV() external view returns (uint256 networkFee);

    /**
     * @notice Returns the legacy network earnings (SSV)
     * @return networkEarnings Total network earnings in SSV
     */
    function getNetworkEarningsSSV() external view returns (uint256 networkEarnings);

    /**
     * @notice Returns the maximum allowed operator fee increase percentage
     * @return Maximum fee increase limit
     */
    function getOperatorFeeIncreaseLimit() external view returns (uint64);

    /**
     * @notice Returns the maximum allowed operator fee (ETH)
     * @return Maximum operator fee
     */
    function getMaximumOperatorFee() external view returns (uint256);

    /**
     * @notice Returns the maximum allowed operator fee (SSV)
     * @return Maximum operator fee
     */
    function getMaximumOperatorFeeSSV() external view returns (uint256);

    /**
     * @notice Returns the minimum operator ETH fee set by DAO
     * @return Minimum operator fee in ETH
     */
    function getMinimumOperatorEthFee() external view returns (uint256);

    /**
     * @notice Returns the declaration and execution periods for operator fee changes
     * @return The struct with operator fee periods
     */
    function getOperatorFeePeriods() external view returns (OperatorFeePeriodsData memory);

    /**
     * @notice Returns the liquidation threshold period (ETH)
     * @return blocks Number of blocks
     */
    function getLiquidationThresholdPeriod() external view returns (uint64 blocks);

    /**
     * @notice Returns the liquidation threshold period (SSV)
     * @return blocks Number of blocks
     */
    function getLiquidationThresholdPeriodSSV() external view returns (uint64 blocks);

    /**
     * @notice Returns the minimum liquidation collateral
     * @return amount Minimum collateral in SSV
     */
    function getMinimumLiquidationCollateral() external view returns (uint256 amount);

    /**
     * @notice Returns the minimum liquidation collateral (SSV)
     * @return amount Minimum collateral in SSV
     */
    function getMinimumLiquidationCollateralSSV() external view returns (uint256 amount);

    /**
     * @notice Returns the maximum number of validators per operator
     * @return validators Maximum validators allowed
     */
    function getValidatorsPerOperatorLimit() external view returns (uint32 validators);

    /**
     * @notice Returns total number of registered validators in the network
     * @return validatorsCount Total validator count
     */
    function getNetworkValidatorsCount() external view returns (uint32 validatorsCount);

    /**
     * @notice Returns the unstaking cooldown duration
     * @return Cooldown period in seconds
     */
    function cooldownDuration() external view returns (uint256);

    /**
     * @notice Returns total SSV tokens currently staked
     * @return Total staked amount
     */
    function totalStaked() external view returns (uint256);

    /**
     * @notice Returns the staked balance of a user
     * @param user User address
     * @return Staked balance
     */
    function stakedBalanceOf(address user) external view returns (uint256);

    /**
     * @notice Returns pending unstake requests for a user
     * @param user User address
     * @return Array of pending amounts and unstake requests
     */
    function pendingUnstake(address user) external view returns (UnstakeRequestsData[] memory);

    /**
     * @notice Returns current accumulated ETH per share
     * @return Accumulated ETH per share
     */
    function accEthPerShare() external view returns (uint256);

    /**
     * @notice Returns current ETH balance in the staking pool
     * @return ETH pool balance
     */
    function stakingEthPoolBalance() external view returns (uint256);

    /**
     * @notice Returns claimable ETH rewards for a user
     * @param user User address
     * @return Claimable ETH amount
     */
    function previewClaimableEth(address user) external view returns (uint256);

    /**
     * @notice Returns oracle address by ID
     * @param oracleId Oracle ID
     * @return Oracle address
     */
    function getOracle(uint32 oracleId) external view returns (address);

    /**
     * @notice Returns weight of a specific oracle
     * @param oracleId Oracle ID
     * @return Oracle weight
     */
    function getOracleWeight(uint32 oracleId) external view returns (uint256);

    /**
     * @notice Returns currently active oracle IDs
     * @return Array of active oracle IDs
     */
    function getActiveOracleIds() external view returns (uint32[MAX_DELEGATION_SLOTS] memory);

    /**
     * @notice Returns the required quorum in basis points
     * @return Quorum in bps
     */
    function getQuorumBps() external view returns (uint16);

    /**
     * @notice Returns the committed merkle root for a given block
     * @param blockNum Block number
     * @return merkleRoot Committed merkle root
     */
    function getCommittedRoot(uint64 blockNum) external view returns (bytes32 merkleRoot);

    /**
     * @notice Returns the current contract version
     * @return Contract version string
     */
    function getVersion() external view returns (string memory);
}
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.20;

import {ISSVNetworkCore} from "./ISSVNetworkCore.sol";

/**
 * @title SSV Operators Interface
 * @author SSV Labs
 * @notice Interface for managing SSV operators including registration, fee management, earnings withdrawal and privacy settings
 */
interface ISSVOperators is ISSVNetworkCore {
    /**
     * @dev Emitted when a new operator is registered
     * @param operatorId The ID assigned to the new operator
     * @param owner The address that owns and can collect fees from this operator
     * @param publicKey The operator's public key used for encrypting validator shares
     * @param fee The fee set for this operator
     */
    event OperatorAdded(
        uint64 indexed operatorId,
        address indexed owner,
        bytes publicKey,
        uint256 fee
    );

    /**
     * @dev Emitted when an operator is removed
     * @param operatorId The ID of the removed operator
     */
    event OperatorRemoved(uint64 indexed operatorId);

    /**
     * @dev Emitted when an operator fee is declared
     * @param owner The owner of the operator
     * @param operatorId The ID of the operator
     * @param blockNumber The block number when the declaration was made
     * @param fee The proposed fee value
     */
    event OperatorFeeDeclared(
        address indexed owner,
        uint64 indexed operatorId,
        uint256 blockNumber,
        uint256 fee
    );

    /**
     * @dev Emitted when a declared operator fee is cancelled
     * @param owner The owner of the operator
     * @param operatorId The ID of the operator
     */
    event OperatorFeeDeclarationCancelled(
        address indexed owner,
        uint64 indexed operatorId
    );

    /**
     * @dev Emitted when a declared operator fee is executed
     * @param owner The owner of the operator
     * @param operatorId The ID of the operator
     * @param blockNumber The block number from which the new fee applies
     * @param fee The new active fee value
     */
    event OperatorFeeExecuted(
        address indexed owner,
        uint64 indexed operatorId,
        uint256 blockNumber,
        uint256 fee
    );

    /**
     * @dev Emitted when operator ETH earnings are withdrawn
     * @param owner The owner of the operator
     * @param operatorId The ID of the operator
     * @param value The amount withdrawn
     */
    event OperatorWithdrawn(
        address indexed owner,
        uint64 indexed operatorId,
        uint256 value
    );

    /**
     * @dev Emitted when operator legacy SSV earnings are withdrawn
     * @param owner The owner of the operator
     * @param operatorId The ID of the operator
     * @param value The amount withdrawn
     */
    event OperatorWithdrawnSSV(
        address indexed owner,
        uint64 indexed operatorId,
        uint256 value
    );

    /**
     * @dev Emitted when an operator changes privacy status
     * @param operatorIds The IDs of the affected operators
     * @param toPrivate True = set to private, False = set to public
     */
    event OperatorPrivacyStatusUpdated(uint64[] operatorIds, bool toPrivate);

    /**
     * @dev Emitted when an operator's whitelist address is updated
     * @param operatorId The ID of the operator
     * @param whitelisted The new whitelisted address
     */
    event OperatorWhitelistUpdated(uint64 indexed operatorId, address whitelisted);

    /**
     * @dev Emitted when operator fee recipient address is updated
     * @param owner The owner of the operator
     * @param recipientAddress The new fee recipient address
     */
    event FeeRecipientAddressUpdated(address indexed owner, address recipientAddress);

    /**
     * @notice Registers a new operator
     * @param publicKey The public key of the operator
     * @param fee The operator's fee (in ETH)
     * @param setPrivate Flag indicating whether the operator should be private
     * @return operatorId The newly assigned operator ID
     */
    function registerOperator(
        bytes calldata publicKey,
        uint256 fee,
        bool setPrivate
    ) external returns (uint64);

    /**
     * @notice Removes an existing ETH operator
     * @param operatorId The ID of the operator to remove
     */
    function removeOperator(uint64 operatorId) external;

    /**
     * @notice Declares a new fee for the operator
     * @param operatorId The ID of the operator
     * @param fee The new fee value to propose (in SSV units)
     */
    function declareOperatorFee(uint64 operatorId, uint256 fee) external;

    /**
     * @notice Executes a previously declared operator fee
     * @param operatorId The ID of the operator
     */
    function executeOperatorFee(uint64 operatorId) external;

    /**
     * @notice Cancels a previously declared (but not yet executed) operator fee
     * @param operatorId The ID of the operator
     */
    function cancelDeclaredOperatorFee(uint64 operatorId) external;

    /**
     * @notice Reduces the operator's fee (can only decrease)
     * @param operatorId The ID of the operator
     * @param fee The new (lower) fee value
     */
    function reduceOperatorFee(uint64 operatorId, uint256 fee) external;

    /**
     * @notice Withdraws a specified amount of operator earnings in ETH (post-migration)
     * @param operatorId The ID of the operator
     * @param ethAmount The amount of ETH to withdraw
     */
    function withdrawOperatorEarnings(uint64 operatorId, uint256 ethAmount) external;

    /**
     * @notice Withdraws all available operator earnings in ETH (post-migration)
     * @param operatorId The ID of the operator
     */
    function withdrawAllOperatorEarnings(uint64 operatorId) external;

    /**
     * @notice Withdraws all available operator earnings (both ETH and legacy SSV) in one call
     * @param operatorId The ID of the operator
     */
    function withdrawAllVersionOperatorEarnings(uint64 operatorId) external;

    /**
     * @notice Withdraws a specified amount of legacy SSV operator earnings (pre-migration)
     * @param operatorId The ID of the operator
     * @param tokenAmount The amount of SSV tokens to withdraw
     */
    function withdrawOperatorEarningsSSV(uint64 operatorId, uint256 tokenAmount) external;

    /**
     * @notice Withdraws all available legacy SSV operator earnings (pre-migration)
     * @param operatorId The ID of the operator
     */
    function withdrawAllOperatorEarningsSSV(uint64 operatorId) external;

    /**
     * @notice Sets multiple operators as private without checking whitelist addresses
     * @param operatorIds Array of operator IDs to set as private
     */
    function setOperatorsPrivateUnchecked(uint64[] calldata operatorIds) external;

    /**
     * @notice Sets multiple operators as public while keeping existing whitelist addresses
     * @param operatorIds Array of operator IDs to set as public
     */
    function setOperatorsPublicUnchecked(uint64[] calldata operatorIds) external;
}
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

type PackedSSV is uint64;
type PackedETH is uint64;

PackedETH constant PACKED_ETH_ZERO = PackedETH.wrap(0);
PackedSSV constant PACKED_SSV_ZERO = PackedSSV.wrap(0);

uint8 constant VERSION_SSV = 0;
uint8 constant VERSION_ETH = 1;
uint8 constant VERSION_UNDEFINED = type(uint8).max;

uint64 constant BPS_DENOMINATOR = 10_000;
uint256 constant DEFAULT_OPERATOR_ETH_FEE = 1778_800_000;
uint256 constant PRECISION = 1e18;

uint256 constant DEDUCTED_DIGITS = 10_000_000;
uint256 constant ETH_DEDUCTED_DIGITS = 100_000;

uint256 constant DEFAULT_EB_PER_VALIDATOR = 32 ether;
uint256 constant MAX_EB_PER_VALIDATOR = 2048 ether;

error SafeCastOverflow();

function _safeUint64(uint128 value) pure returns (uint64) {
    if (value > type(uint64).max) revert SafeCastOverflow();
    return uint64(value);
}


// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.20;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title CSSV Token Interface
 * @author SSV Labs
 */
interface ICSSVToken is IERC20 {
    /**
     * @dev Mints a specified amount of tokens to an address
     * @param to The address that will receive the minted tokens
     * @param amount The amount of tokens to mint
     */
    function mint(address to, uint256 amount) external;

    /**
     * @dev Burns a specified amount of tokens from an address
     * @param from The address from which tokens will be burned
     * @param amount The amount of tokens to burn
     */
    function burn(address from, uint256 amount) external;
}
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

type PackedSSV is uint64;
type PackedETH is uint64;

PackedETH constant PACKED_ETH_ZERO = PackedETH.wrap(0);
PackedSSV constant PACKED_SSV_ZERO = PackedSSV.wrap(0);

uint8 constant VERSION_SSV = 0;
uint8 constant VERSION_ETH = 1;
uint8 constant VERSION_UNDEFINED = type(uint8).max;

uint64 constant BPS_DENOMINATOR = 10_000;
uint256 constant DEFAULT_OPERATOR_ETH_FEE = 1778_800_000;
uint256 constant PRECISION = 1e18;

uint256 constant DEDUCTED_DIGITS = 10_000_000;
uint256 constant ETH_DEDUCTED_DIGITS = 100_000;

uint256 constant DEFAULT_EB_PER_VALIDATOR = 32 ether;
uint256 constant MAX_EB_PER_VALIDATOR = 2048 ether;

error SafeCastOverflow();

function _safeUint64(uint128 value) pure returns (uint64) {
    if (value > type(uint64).max) revert SafeCastOverflow();
    return uint64(value);
}


// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

type PackedSSV is uint64;
type PackedETH is uint64;

PackedETH constant PACKED_ETH_ZERO = PackedETH.wrap(0);
PackedSSV constant PACKED_SSV_ZERO = PackedSSV.wrap(0);

uint8 constant VERSION_SSV = 0;
uint8 constant VERSION_ETH = 1;
uint8 constant VERSION_UNDEFINED = type(uint8).max;

uint64 constant BPS_DENOMINATOR = 10_000;
uint256 constant DEFAULT_OPERATOR_ETH_FEE = 1778_800_000;
uint256 constant PRECISION = 1e18;

uint256 constant DEDUCTED_DIGITS = 10_000_000;
uint256 constant ETH_DEDUCTED_DIGITS = 100_000;

uint256 constant DEFAULT_EB_PER_VALIDATOR = 32 ether;
uint256 constant MAX_EB_PER_VALIDATOR = 2048 ether;

error SafeCastOverflow();

function _safeUint64(uint128 value) pure returns (uint64) {
    if (value > type(uint64).max) revert SafeCastOverflow();
    return uint64(value);
}


// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.20;

import {PackedSSV, PackedETH} from "../libraries/SSVCoreTypes.sol";

interface ISSVNetworkCore {
    /// @notice Represents a snapshot of an SSV operator's or a SSV DAO's state at a certain block
    struct Snapshot {
        /// @dev The block number when the snapshot was taken
        uint32 block;
        /// @dev The last index calculated by the formula index += (currentBlock - block) * fee
        uint64 index;
        /// @dev Total accumulated earnings calculated by the formula accumulated + lastIndex * validatorCount
        PackedSSV balance;
    }

    /// @notice Represents a snapshot of an operator's or a DAO's state at a certain block
    struct EthSnapshot {
        /// @dev The block number when the snapshot was taken
        uint32 block;
        /// @dev The last index calculated by the formula index += (currentBlock - block) * fee
        uint64 index;
        /// @dev Total accumulated earnings calculated by the formula accumulated + lastIndex * validatorCount
        PackedETH balance;
    }

    /// @notice Represents an SSV operator
    struct Operator {
        /// @dev The number of validators associated with this operator
        uint32 validatorCount;
        /// @dev The fee charged by the operator, set to zero for private operators and cannot be increased once set
        PackedSSV fee;
        /// @dev The address of the operator's owner
        address owner;
        /// @dev private flag for this operator
        bool whitelisted;
        /// @dev The state snapshot of the operator
        Snapshot snapshot;
        
        /// @dev The number of validators associated with this operator in eth
        uint32 ethValidatorCount;
        /// @dev The fee charged by the operator in eth, set to zero for private operators and cannot be increased once set
        PackedETH ethFee;
        /// @dev The state snapshot of the operator for eth
        EthSnapshot ethSnapshot;
    }

    /// @notice Represents a request to change an operator's fee
    struct OperatorFeeChangeRequest {
        /// @dev The new fee proposed by the operator
        uint64 fee;
        /// @dev The time when the approval period for the fee change begins
        uint64 approvalBeginTime;
        /// @dev The time when the approval period for the fee change ends
        uint64 approvalEndTime;
    }

    /// @notice Represents a cluster of validators
    struct Cluster {
        /// @dev The number of validators in the cluster
        uint32 validatorCount;
        /// @dev The index of network fees related to this cluster
        uint64 networkFeeIndex;
        /// @dev The last index calculated for the cluster
        uint64 index;
        /// @dev Flag indicating whether the cluster is active
        bool active;
        /// @dev The balance of the cluster
        uint256 balance;
    }

    /**
     * @dev Thrown when the caller is not the owner of the called entity (operator, cluster)
     */
    error CallerNotOwnerWithData(address caller, address owner); // 0x8907fc65

    /**
     * @dev Thrown when the caller is trying to create a cluster with an operator who did not whitelist the caller
     */
    error CallerNotWhitelistedWithData(uint64 operatorId); // 0xb7f529fe

    /**
     * @dev Thrown when trying to use a fee that is below a minimum allowed
     */
    error FeeTooLow(); // 0x732f9413

    /**
     * @dev Thrown when trying to increase the fee above the allowed limit
     */
    error FeeExceedsIncreaseLimit(); // 0x958065d9

    /**
     * @dev Thrown when trying executee a fee without declaration
     */
    error NoFeeDeclared(); // 0x1d226c30

    /**
     * @dev Thrown when trying to execute fee change outside approval timeframe
     */
    error ApprovalNotWithinTimeframe(); // 0x97e4b518

    /**
     * @dev Thrown when operator does not exist
     */
    error OperatorDoesNotExist(); // 0x961e3e8c

    /**
     * @dev Thrown when balance required to perform an action is insufficient
     */
    error InsufficientBalance(); // 0xf4d678b8

    /**
     * @dev Thrown when validator does not exist
     */
    error ValidatorDoesNotExist(); // 0xe51315d2

    /**
     * @dev Thrown when cluster is not liquidatable
     */
    error ClusterNotLiquidatable(); // 0x60300a8d

    /**
     * @dev Thrown when public key length is invalid
     */
    error InvalidPublicKeyLength(); // 0x637297a4

    /**
     * @dev Thrown when operator IDs length is invalid (allowed only 4, 7, 10 and 13)
     */
    error InvalidOperatorIdsLength(); // 0x38186224

    /**
     * @dev Thrown when trying to reactive active cluster
     */
    error ClusterAlreadyEnabled(); // 0x3babafd2

    /**
     * @dev Thrown when trying to interact with a liquidated cluster
     */
    error ClusterIsLiquidated(); // 0x95a0cf33

    /**
     * @dev Thrown when cluster does not exist
     */
    error ClusterDoesNotExist(); // 0x25d92f88

    /**
     * @dev Thrown when the provided data is incorrect
     */
    error IncorrectClusterState(); // 0x12e04c87

    /**
     * @dev Thrown when operators list is unsorted
     */
    error UnsortedOperatorsList(); // 0xdd020e25

    /**
     * @dev Thrown when new block period is below minimum
     */
    error NewBlockPeriodIsBelowMinimum(); // 0x6e6c9cac

    /**
     * @dev Thrown when registering a validator, but validator limit is exceeded
     */
    error ExceedValidatorLimitWithData(uint64 operatorId); // 0x639f5851

    /**
     * @dev Thrown when token transfer fails
     */
    error TokenTransferFailed(); // 0x045c4b02

    /**
     * @dev Thrown when trying to change fee to the same value
     */
    error SameFeeChangeNotAllowed(); // 0xc81272f8

    /**
     * @dev Thrown when trying to increase fee of a free operator
     */
    error FeeIncreaseNotAllowed(); // 0x410a2b6c

    /**
     * @dev Thrown when operators list is not unique and has duplicates
     */
    error OperatorsListNotUnique(); // 0xa5a1ff5d

    /**
     * @dev Thrown when operator with the same public key already exists
     */
    error OperatorAlreadyExists(); // 0x289c9494

    /**
     * @dev Thrown when target module does not exist
     */
    error TargetModuleDoesNotExistWithData(uint8 moduleId); // 0x208bb85d

    /**
     * @dev Thrown when maximum value is exceeded for the target type
     */
    error MaxValueExceeded(); // 0x91aa3017

    /**
     * @dev Thrown when precision is exceeded (e.g., division with remainder)
     */
    error MaxPrecisionExceeded(); // 0x24756546

    /**
     * @dev Thrown when the provided fee is too high
     */
    error FeeTooHigh(); // 0xcd4e6167

    /**
     * @dev Thrown when public keys and shares arrays length mismatch
     */
    error PublicKeysSharesLengthMismatch(); // 0x9ad467b8

    /**
     * @dev Thrown when validator state is incorrect
     */
    error IncorrectValidatorStateWithData(bytes publicKey); // 0x89307938

    /**
     * @dev Thrown when trying to register a validator that is already registered
     */
    error ValidatorAlreadyRegistered(bytes publicKey, address owner); // 0x75106a26

    /**
     * @dev Thrown when public keys list is empty
     */
    error EmptyPublicKeysList(); // 0xdf83e679

    /**
     * @dev Thrown when address is a whitelisting contract
     */
    error AddressIsWhitelistingContract(address contractAddress); // 0x71cadba7

    /**
     * @dev Thrown when whitelisting contract is invalid
     */
    error InvalidWhitelistingContract(address contractAddress); // 0x886e6a03

    /**
     * @dev Thrown when whitelist addresses length is invalid
     */
    error InvalidWhitelistAddressesLength(); // 0xcbb362dc

    /**
     * @dev Thrown when trying to use zero address
     */
    error ZeroAddressNotAllowed(); // 0x8579befe

    /**
     * @dev Thrown when operator version is incorrect
     */
    error IncorrectOperatorVersion(uint8 operatorVersion); // 0xf222e863

    /**
     * @dev Thrown when cluster version is incorrect
     */
    error IncorrectClusterVersion(); // 0xf6749746

    /**
     * @dev Thrown when ETH transfer fails
     */
    error ETHTransferFailed(); // 0xb12d13eb

    /**
     * @dev Thrown when legacy operator fee declaration (before migration) is invalid for current configuration
     */
    error LegacyOperatorFeeDeclarationInvalid(); // 0x9e593e76

    /**
     * @dev Thrown when the provided block number is stale
     */
    error StaleBlockNumber(); // 0x305c3e93

    /**
     * @dev Thrown when commiting a block number that is in future
     */
    error FutureBlockNumber(); // 0x252f8a0e

    /**
     * @dev Thrown when the merkle for a specific block root was not found
     */
    error RootNotFound(); // 0x3033b0ff

    /**
     * @dev Thrown when eb update is happening too frequent
     */
    error UpdateTooFrequent(); // 0x53f7a6ee

    /**
     * @dev Thrown when eb update is stale
     */
    error StaleUpdate(); // 0x666a2814

    /**
     * @dev Thrown when eb update does not use latest committed root block
     */
    error MustUseLatestRoot();

    /**
     * @dev Thrown when the merkle proof is invalid
     */
    error InvalidProof(); // 0x09bde339

    /**
     * @dev Thrown when EB exceeds maximum allowed
     */
    error EBExceedsMaximum(); // 0xf5ca7cb9

    /**
     * @dev Thrown when EB is below minimum
     */
    error EBBelowMinimum(); // 0x9fecdce5

    /**
     * @dev Thrown when no cSSV supply exists for root voting
     */
    error ZeroCSSVSupply();

    /**
     * @dev Thrown when cSSV supply exists but truncates to zero oracle weight
     */
    error InsufficientCSSVSupply();

    /**
     * @dev Thrown when the caller is not cSSV token
     */
    error NotCSSV(); // 0x1598959e

    /**
     * @dev Thrown when trying to use zero address
     */
    error ZeroAddress(); // 0xd92e233d

    /**
     * @dev Thrown when trying to configure a quorum higher than 100%
     */
    error InvalidQuorum(); // 0xd1735779

    /**
     * @dev Thrown when trying to configure operator fee increase limit above 100%
     */
    error InvalidOperatorFeeIncreaseLimit(); // 0x602d89dd

    /**
     * @dev Thrown when trying to configure inconsistent operator fee bounds
     */
    error InvalidOperatorFeeRange(); // 0x44b0758c

    /**
     * @dev Thrown when amount is zero
     */
    error ZeroAmount(); // 0x1f2a2005

    /**
     * @dev Thrown when token is invalid
     */
    error InvalidToken(); // 0xc1ab6dc1

    /**
     * @dev Thrown when user has nothing to claim
     */
    error NothingToClaim(); // 0x969bf728

    /**
     * @dev Thrown when user has nothing to withdraw
     */
    error NothingToWithdraw(); // 0xd0d04f60

    /**
     * @dev Thrown when unstake amount exceeds staked balance
     */
    error UnstakeAmountExceedsBalance(); // 0x02a19f57

    /**
     * @dev Thrown when stake amount is less than minimum allowed
     */
    error StakeTooLow(); // 0x1cc3b37b

    /**
     * @dev Thrown when the caller is not an oracle
     */
    error NotOracle(); // 0x1bc2178f

    /**
     * @dev Thrown when the oracle already voted for this root
     */
    error AlreadyVoted(); // 0x7c9a1cf9

    /**
     * @dev Thrown when oracle is already assigned with the selected address
     */
    error OracleAlreadyAssigned(); // 0xa97938cb

    /**
     * @dev Thrown when attempting to replace an oracle with the same address
     */
    error SameOracleAddressNotAllowed(); // 0xe991f7e9

    /**
     * @dev Thrown when oracleId exceeds the maximum allowed oracle slots
     */
    error InvalidOracleId();

    /**
     * @dev Thrown when the maximum unstake requests amount reached
     */
    error MaxRequestsAmountReached(); // 0xee0e82ff


    // legacy errors
    error ValidatorAlreadyExists(); // 0x8d09a73e
    error ValidatorAlreadyExistsWithData(bytes publicKey); // 0x388e7999
    error IncorrectValidatorState(); // 0x2feda3c1
    error ExceedValidatorLimit(uint64 operatorId); // 0x6df5ab76
    error CallerNotOwner(); // 0x5cd83192
    error TargetModuleDoesNotExist(); // 0x8f9195fb
    error CallerNotWhitelisted(); // 0x8c6e5d71

}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {PackedETH} from "../SSVCoreTypes.sol";

uint256 constant MAX_DELEGATION_SLOTS = 4;

struct UnstakeRequest {
    /// @notice Amount of cSSV burned and pending to be withdrawn as SSV
    uint192 amount;
    /// @notice Timestamp after which the pending unstake can be withdrawn
    uint64 unlockTime;
}

struct StorageStaking {
    /// @notice Unstake cooldown duration in seconds
    uint64 cooldownDuration;
    /// @notice Total ETH-denominated rewards (shrunk) allocated to the staking pool
    PackedETH stakingEthPoolBalance;
    /// @notice Global accumulated ETH rewards per cSSV token (scaled by PRECISION)
    uint128 accEthPerShare;

    /// @notice Per-user reward index used to track their last settled accEthPerShare
    mapping(address => uint256) userIndex;
    /// @notice Accumulated but unclaimed ETH rewards for each user (in wei)
    mapping(address => uint256) accrued;

    /// @notice Oracle registry: stable ID => oracle address
    mapping(uint32 => address) oracles;
    /// @notice Reverse lookup: oracle address => oracle ID (0 if not registered)
    mapping(address => uint32) oracleIdOf;
    /// @notice Default oracle IDs to use for new delegations (equal split)
    uint32[MAX_DELEGATION_SLOTS] defaultOracleIds;
    /// @notice Quorum threshold in basis points (e.g. 7000 = 70%)
    uint16 quorumBps;
    /// @notice The mapping of address to their unstake requests
    mapping(address => UnstakeRequest[]) withdrawalRequests;
}

library SSVStorageStaking {
    uint256 private constant SSV_STORAGE_POSITION = uint256(keccak256("ssv.network.storage.staking")) - 1;

    function load() internal pure returns (StorageStaking storage ss) {
        uint256 position = SSV_STORAGE_POSITION;
        assembly {
            ss.slot := position
        }
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {PackedETH} from "../SSVCoreTypes.sol";

uint256 constant MAX_DELEGATION_SLOTS = 4;

struct UnstakeRequest {
    /// @notice Amount of cSSV burned and pending to be withdrawn as SSV
    uint192 amount;
    /// @notice Timestamp after which the pending unstake can be withdrawn
    uint64 unlockTime;
}

struct StorageStaking {
    /// @notice Unstake cooldown duration in seconds
    uint64 cooldownDuration;
    /// @notice Total ETH-denominated rewards (shrunk) allocated to the staking pool
    PackedETH stakingEthPoolBalance;
    /// @notice Global accumulated ETH rewards per cSSV token (scaled by PRECISION)
    uint128 accEthPerShare;

    /// @notice Per-user reward index used to track their last settled accEthPerShare
    mapping(address => uint256) userIndex;
    /// @notice Accumulated but unclaimed ETH rewards for each user (in wei)
    mapping(address => uint256) accrued;

    /// @notice Oracle registry: stable ID => oracle address
    mapping(uint32 => address) oracles;
    /// @notice Reverse lookup: oracle address => oracle ID (0 if not registered)
    mapping(address => uint32) oracleIdOf;
    /// @notice Default oracle IDs to use for new delegations (equal split)
    uint32[MAX_DELEGATION_SLOTS] defaultOracleIds;
    /// @notice Quorum threshold in basis points (e.g. 7000 = 70%)
    uint16 quorumBps;
    /// @notice The mapping of address to their unstake requests
    mapping(address => UnstakeRequest[]) withdrawalRequests;
}

library SSVStorageStaking {
    uint256 private constant SSV_STORAGE_POSITION = uint256(keccak256("ssv.network.storage.staking")) - 1;

    function load() internal pure returns (StorageStaking storage ss) {
        uint256 position = SSV_STORAGE_POSITION;
        assembly {
            ss.slot := position
        }
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

type PackedSSV is uint64;
type PackedETH is uint64;

PackedETH constant PACKED_ETH_ZERO = PackedETH.wrap(0);
PackedSSV constant PACKED_SSV_ZERO = PackedSSV.wrap(0);

uint8 constant VERSION_SSV = 0;
uint8 constant VERSION_ETH = 1;
uint8 constant VERSION_UNDEFINED = type(uint8).max;

uint64 constant BPS_DENOMINATOR = 10_000;
uint256 constant DEFAULT_OPERATOR_ETH_FEE = 1778_800_000;
uint256 constant PRECISION = 1e18;

uint256 constant DEDUCTED_DIGITS = 10_000_000;
uint256 constant ETH_DEDUCTED_DIGITS = 100_000;

uint256 constant DEFAULT_EB_PER_VALIDATOR = 32 ether;
uint256 constant MAX_EB_PER_VALIDATOR = 2048 ether;

error SafeCastOverflow();

function _safeUint64(uint128 value) pure returns (uint64) {
    if (value > type(uint64).max) revert SafeCastOverflow();
    return uint64(value);
}


// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

type PackedSSV is uint64;
type PackedETH is uint64;

PackedETH constant PACKED_ETH_ZERO = PackedETH.wrap(0);
PackedSSV constant PACKED_SSV_ZERO = PackedSSV.wrap(0);

uint8 constant VERSION_SSV = 0;
uint8 constant VERSION_ETH = 1;
uint8 constant VERSION_UNDEFINED = type(uint8).max;

uint64 constant BPS_DENOMINATOR = 10_000;
uint256 constant DEFAULT_OPERATOR_ETH_FEE = 1778_800_000;
uint256 constant PRECISION = 1e18;

uint256 constant DEDUCTED_DIGITS = 10_000_000;
uint256 constant ETH_DEDUCTED_DIGITS = 100_000;

uint256 constant DEFAULT_EB_PER_VALIDATOR = 32 ether;
uint256 constant MAX_EB_PER_VALIDATOR = 2048 ether;

error SafeCastOverflow();

function _safeUint64(uint128 value) pure returns (uint64) {
    if (value > type(uint64).max) revert SafeCastOverflow();
    return uint64(value);
}


// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

type PackedSSV is uint64;
type PackedETH is uint64;

PackedETH constant PACKED_ETH_ZERO = PackedETH.wrap(0);
PackedSSV constant PACKED_SSV_ZERO = PackedSSV.wrap(0);

uint8 constant VERSION_SSV = 0;
uint8 constant VERSION_ETH = 1;
uint8 constant VERSION_UNDEFINED = type(uint8).max;

uint64 constant BPS_DENOMINATOR = 10_000;
uint256 constant DEFAULT_OPERATOR_ETH_FEE = 1778_800_000;
uint256 constant PRECISION = 1e18;

uint256 constant DEDUCTED_DIGITS = 10_000_000;
uint256 constant ETH_DEDUCTED_DIGITS = 100_000;

uint256 constant DEFAULT_EB_PER_VALIDATOR = 32 ether;
uint256 constant MAX_EB_PER_VALIDATOR = 2048 ether;

error SafeCastOverflow();

function _safeUint64(uint128 value) pure returns (uint64) {
    if (value > type(uint64).max) revert SafeCastOverflow();
    return uint64(value);
}



## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.20;

import {ISSVNetworkCore} from "./ISSVNetworkCore.sol";

/**
 * @title SSV Clusters Interface
 * @author SSV Labs
 * @dev Interface for managing SSV clusters, including migration, liquidation, reactivation, deposits, withdrawals, and balance updates
 */
interface ISSVClusters is ISSVNetworkCore {
    /// @dev Context structure for updating cluster balances
    struct UpdateCtx {
        /// @dev The owner of the cluster
        address clusterOwner;
        /// @dev The unique identifier for the cluster
        bytes32 clusterId;
        /// @dev The block number for the update
        uint64 blockNum;
        /// @dev The effective balance of the cluster
        uint32 effectiveBalance;
        /// @dev The merkle proof for validation
        bytes32[] merkleProof;
        /// @dev The version of the cluster
        uint8 version;
    }

    /**
     * @dev Emitted when a cluster is liquidated
     * @param owner The owner of the liquidated cluster
     * @param operatorIds The operator IDs managing the cluster
     * @param cluster The liquidated cluster data
     */
    event ClusterLiquidated(address indexed owner, uint64[] operatorIds, Cluster cluster);

    /**
     * @dev Emitted when a cluster is reactivated
     * @param owner The owner of the reactivated cluster
     * @param operatorIds The operator IDs managing the cluster
     * @param cluster The reactivated cluster data
     */
    event ClusterReactivated(address indexed owner, uint64[] operatorIds, Cluster cluster);

    /**
     * @dev Emitted when a legacy SSV cluster is migrated to ETH
     * @param owner The owner of the migrated cluster
     * @param operatorIds The operator IDs managing the cluster
     * @param ethDeposited The amount of ETH supplied during migration
     * @param ssvRefunded The amount of SSV tokens refunded to the owner
     * @param effectiveBalance Cluster effective balance in wei
     * @param cluster The migrated cluster data (ETH version)
     */
    event ClusterMigratedToETH(
        address indexed owner,
        uint64[] operatorIds,
        uint256 ethDeposited,
        uint256 ssvRefunded,
        uint32 effectiveBalance,
        Cluster cluster
    );

    /**
     * @dev Emitted when tokens are withdrawn from a cluster
     * @param owner The owner of the cluster
     * @param operatorIds The operator IDs managing the cluster
     * @param value The amount of tokens withdrawn
     * @param cluster The cluster from which tokens were withdrawn
     */
    event ClusterWithdrawn(address indexed owner, uint64[] operatorIds, uint256 value, Cluster cluster);

    /**
     * @dev Emitted when tokens are deposited into a cluster
     * @param owner The owner of the cluster
     * @param operatorIds The operator IDs managing the cluster
     * @param value The amount of ETH deposited
     * @param cluster The cluster into which ETH was deposited
     */
    event ClusterDeposited(address indexed owner, uint64[] operatorIds, uint256 value, Cluster cluster);

    /**
     * @dev Emitted when a cluster's balance is updated
     * @param owner The owner of the cluster
     * @param operatorIds The operator IDs managing the cluster
     * @param blockNum The block number of the update
     * @param effectiveBalance The new effective balance
     * @param cluster The updated cluster data
     */
    event ClusterBalanceUpdated(
        address indexed owner,
        uint64[] operatorIds,
        uint64 indexed blockNum,
        uint32 effectiveBalance,
        ISSVNetworkCore.Cluster cluster
    );


    /**
     * @notice Migrates an SSV cluster to ETH, returning any SSV balance and accepting ETH top-up
     * @param operatorIds Array of IDs of operators managing the cluster
     * @param cluster Cluster data to migrate
     */
    function migrateClusterToETH(uint64[] calldata operatorIds, Cluster memory cluster) external payable;

    /**
     * @notice Liquidates a cluster
     * @param owner The owner of the cluster
     * @param operatorIds Array of IDs of operators managing the cluster
     * @param cluster Cluster to be liquidated
     */
    function liquidate(address owner, uint64[] memory operatorIds, Cluster memory cluster) external;

    /**
     * @notice Liquidates a cluster using SSV
     * @param owner The owner of the cluster
     * @param operatorIds Array of IDs of operators managing the cluster
     * @param cluster Cluster to be liquidated
     */
    function liquidateSSV(address owner, uint64[] memory operatorIds, Cluster memory cluster) external;

    /**
     * @notice Reactivates a cluster
     * @param operatorIds Array of IDs of operators managing the cluster
     * @param cluster Cluster to be reactivated
     */
    function reactivate(uint64[] memory operatorIds, Cluster memory cluster) external payable;

    /**
     * @notice Deposits ETH into a cluster
     * @param owner The owner of the cluster
     * @param operatorIds Array of IDs of operators managing the cluster
     * @param cluster Cluster to which the deposit will be made
     */
    function deposit(
        address owner,
        uint64[] memory operatorIds,
        Cluster memory cluster
    ) external payable;

    /**
     * @notice Withdraws ETH from a cluster
     * @param operatorIds Array of IDs of operators managing the cluster
     * @param tokenAmount Amount of SSV tokens to be withdrawn
     * @param cluster Cluster where the withdrawal will be made
     */
    function withdraw(uint64[] memory operatorIds, uint256 tokenAmount, Cluster memory cluster) external;

    /**
     * @notice Updates the balance of a cluster
     * @param blockNum The block number for the update
     * @param clusterOwner The owner of the cluster
     * @param operatorIds Array of operator IDs
     * @param cluster The cluster data.
     * @param effectiveBalance The new effective balance
     * @param merkleProof The merkle proof for validation
     */
    function updateClusterBalance(
        uint64 blockNum,
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster,
        uint32 effectiveBalance,
        bytes32[] calldata merkleProof
    ) external;
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.20;

import {ISSVNetworkCore} from "./ISSVNetworkCore.sol";

/**
 * @title SSV Staking Interface
 * @author SSV Labs
 * @notice Interface for SSV staking operations including staking tokens, requesting unstakes, withdrawing, claiming rewards and managing fees
 */
interface ISSVStaking is ISSVNetworkCore {
    /**
     * @dev Emitted when SSV tokens are staked
     * @param user The user who staked tokens
     * @param amount The amount of SSV staked
     */
    event Staked(address indexed user, uint256 amount);

    /**
     * @dev Emitted when an unstake is requested
     * @param user The user requesting the unstake
     * @param amount The amount of cSSV burned (matches SSV amount)
     * @param unlockTime When the SSV can be withdrawn
     */
    event UnstakeRequested(address indexed user, uint256 amount, uint256 unlockTime);

    /**
     * @dev Emitted when unstaked SSV is withdrawn
     * @param user The user withdrawing
     * @param amount The amount of SSV withdrawn
     */
    event UnstakedWithdrawn(address indexed user, uint256 amount);

    /**
     * @dev Emitted when fees are synced within the protocol
     * @param newFeesWei New fees amount in wei
     * @param accEthPerShare Updated accumulated ETH per share
     */
    event FeesSynced(uint256 newFeesWei, uint256 accEthPerShare);

    /**
     * @dev Emitted when a user's rewards are settled
     * @param user The user's address
     * @param pending Pending rewards for this settlement
     * @param accrued Total accrued rewards
     * @param userIndex User's reward index after settlement
     */
    event RewardsSettled(address indexed user, uint256 pending, uint256 accrued, uint256 userIndex);

    /**
     * @dev Emitted when ETH rewards are claimed
     * @param user The user claiming
     * @param amount The ETH amount claimed
     */
    event RewardsClaimed(address indexed user, uint256 amount);

    /**
     * @dev Emitted when ERC20 tokens are rescued
     * @param token The token rescued
     * @param to The recipient
     * @param amount The amount rescued
     */
    event ERC20Rescued(address indexed token, address indexed to, uint256 amount);

    /**
     * @notice Updates the global ETH reward index from protocol storage
     */
    function syncFees() external;

    /**
     * @notice Stakes SSV tokens to mint cSSV and earn ETH rewards
     * @param amount Amount of SSV to stake
     */
    function stake(uint256 amount) external;

    /**
     * @notice Requests to unstake SSV by burning cSSV
     * @notice Starts cooldown period
     * @param amount Amount of cSSV to burn (1:1 with SSV)
     */
    function requestUnstake(uint256 amount) external;

    /**
     * @notice Withdraws unlocked SSV after cooldown
     */
    function withdrawUnlocked() external;

    /**
     * @notice Claims earned ETH rewards
     */
    function claimEthRewards() external;

    /**
     * @notice Rescues stuck ERC20 tokens (not SSV or cSSV)
     * @param token Token address
     * @param to Recipient
     * @param amount Amount to rescue
     */
    function rescueERC20(address token, address to, uint256 amount) external;

    /**
     * @dev Hook for cSSV transfers
     * @dev Updates rewards for sender and receiver
     * @param from Sender
     * @param to Recipient
     * @param amount cSSV amount
     */
    function onCSSVTransfer(address from, address to, uint256 amount) external;
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {ISSVOperatorsWhitelist} from "../interfaces/ISSVOperatorsWhitelist.sol";
import {ISSVWhitelistingContract} from "../interfaces/external/ISSVWhitelistingContract.sol";
import {StorageData, SSVStorage} from "../libraries/storage/SSVStorage.sol";
import {OperatorLib} from "../libraries/OperatorLib.sol";

contract SSVOperatorsWhitelist is ISSVOperatorsWhitelist {
    using OperatorLib for Operator;

    /**
     * @inheritdoc ISSVOperatorsWhitelist
     */
    function setOperatorsWhitelists(
        uint64[] calldata operatorIds,
        address[] calldata whitelistAddresses
    ) external override {
        OperatorLib.updateMultipleWhitelists(whitelistAddresses, operatorIds, true, SSVStorage.load());
        emit OperatorMultipleWhitelistUpdated(operatorIds, whitelistAddresses);
    }

    /**
     * @inheritdoc ISSVOperatorsWhitelist
     */
    function removeOperatorsWhitelists(
        uint64[] calldata operatorIds,
        address[] calldata whitelistAddresses
    ) external override {
        OperatorLib.updateMultipleWhitelists(whitelistAddresses, operatorIds, false, SSVStorage.load());
        emit OperatorMultipleWhitelistRemoved(operatorIds, whitelistAddresses);
    }

    /**
     * @inheritdoc ISSVOperatorsWhitelist
     */
    function setOperatorsWhitelistingContract(
        uint64[] calldata operatorIds,
        ISSVWhitelistingContract whitelistingContract
    ) external {
        // Reverts also when whitelistingContract == address(0)
        if (!OperatorLib.isWhitelistingContract(address(whitelistingContract)))
            revert InvalidWhitelistingContract(address(whitelistingContract));

        uint256 operatorsLength = OperatorLib.checkOperatorsLength(operatorIds);

        StorageData storage s = SSVStorage.load();
        Operator storage operator;

        for (uint256 i; i < operatorsLength; ++i) {
            uint64 operatorId = operatorIds[i];

            operator = s.operators[operatorId];
            operator.checkOwner();

            address currentWhitelisted = s.operatorsWhitelist[operatorId];

            // operator already whitelisted?
            // if EOA or generic contract, move it to SSV whitelisting module
            if (currentWhitelisted != address(0) && !OperatorLib.isWhitelistingContract(currentWhitelisted)) {
                (uint256 blockIndex, uint256 bitPosition) = OperatorLib.getBitmapIndexes(operatorId);

                s.addressWhitelistedForOperators[currentWhitelisted][blockIndex] |= (1 << bitPosition);
            }

            s.operatorsWhitelist[operatorId] = address(whitelistingContract);
        }

        emit OperatorWhitelistingContractUpdated(operatorIds, address(whitelistingContract));
    }

    /**
     * @inheritdoc ISSVOperatorsWhitelist
     */
    function removeOperatorsWhitelistingContract(uint64[] calldata operatorIds) external {
        uint256 operatorsLength = OperatorLib.checkOperatorsLength(operatorIds);

        StorageData storage s = SSVStorage.load();
        Operator storage operator;

        for (uint256 i; i < operatorsLength; ++i) {
            uint64 operatorId = operatorIds[i];
            operator = s.operators[operatorId];

            operator.checkOwner();

            s.operatorsWhitelist[operatorId] = address(0);
        }

        emit OperatorWhitelistingContractUpdated(operatorIds, address(0));
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {ISSVOperators} from "../interfaces/ISSVOperators.sol";
import {PackedSSV, PackedETH, VERSION_ETH, VERSION_SSV, PACKED_ETH_ZERO, PACKED_SSV_ZERO, BPS_DENOMINATOR} from "../libraries/SSVCoreTypes.sol";
import {PackedSSVLib, PackedETHLib} from "../libraries/SSVPackedLib.sol";
import {SSVStorage, StorageData} from "../libraries/storage/SSVStorage.sol";
import {SSVStorageProtocol, StorageProtocol} from "../libraries/storage/SSVStorageProtocol.sol";
import {OperatorLib} from "../libraries/OperatorLib.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {SSVStorageEB, StorageEB} from "../libraries/storage/SSVStorageEB.sol";
import {SSVReentrancyGuard} from "../abstract/SSVReentrancyGuard.sol";

import {Counters} from "@openzeppelin/contracts/utils/Counters.sol";

contract SSVOperators is ISSVOperators, SSVReentrancyGuard {
    uint256 public immutable UPGRADE_TIMESTAMP;

    using Counters for Counters.Counter;
    using OperatorLib for Operator;
    using PackedETHLib for PackedETH;
    using PackedSSVLib for PackedSSV;

    constructor(uint256 upgradeTimestamp) {
        UPGRADE_TIMESTAMP = upgradeTimestamp;
    }

    /**
     * @inheritdoc ISSVOperators
     */
    function registerOperator(
        bytes calldata publicKey,
        uint256 fee,
        bool setPrivate
    ) external override returns (uint64 id) {
        StorageProtocol storage sp = SSVStorageProtocol.load();

        if (fee != 0 && fee < PackedETHLib.unpack(sp.minimumOperatorEthFee)) {
            revert FeeTooLow();
        }
        if (fee > PackedETHLib.unpack(sp.operatorMaxFee)) {
            revert FeeTooHigh();
        }

        StorageData storage s = SSVStorage.load();

        bytes32 hashedPk = keccak256(publicKey);
        if (s.operatorsPKs[hashedPk] != 0) revert OperatorAlreadyExists();

        s.lastOperatorId.increment();
        id = uint64(s.lastOperatorId.current());
        Operator storage op = s.operators[id];

        op.owner = msg.sender;
        op.whitelisted = setPrivate;
        op.ethFee = PackedETHLib.pack(fee);

        op.ethSnapshot.block = uint32(block.number);
        s.operatorsPKs[hashedPk] = id;

        uint64[] memory operatorIds = new uint64[](1);
        operatorIds[0] = id;

        emit OperatorAdded(id, msg.sender, publicKey, fee);
        emit OperatorPrivacyStatusUpdated(operatorIds, setPrivate);
    }

    /**
     * @inheritdoc ISSVOperators
     */
    function removeOperator(uint64 operatorId) external override nonReentrant {
        StorageData storage s = SSVStorage.load();
        Operator storage operator = s.operators[operatorId];
        StorageEB storage seb = SSVStorageEB.load();

        operator.checkOwner();

        PackedETH currentBalanceETH = PACKED_ETH_ZERO;
        PackedSSV currentBalanceSSV = PACKED_SSV_ZERO;

        if (operator.snapshot.block != 0) {
            OperatorLib.updateSnapshotStSSV(operator);
            currentBalanceSSV = operator.snapshot.balance;
        }

        if (operator.ethSnapshot.block != 0) {
            OperatorLib.updateSnapshotSt(operator, operatorId);
            currentBalanceETH = operator.ethSnapshot.balance;
        }

        _resetOperatorState(operator);

        delete seb.operatorEthVUnits[operatorId];
        delete s.operatorFeeChangeRequests[operatorId];
        delete s.operatorsWhitelist[operatorId];

        if (PackedETHLib.raw(currentBalanceETH) > 0) {
            _transferOperatorBalanceUnsafe(operatorId, PackedETHLib.unpack(currentBalanceETH));
        }
        if (PackedSSVLib.raw(currentBalanceSSV) > 0) {
            _transferOperatorTokenBalanceUnsafe(operatorId, PackedSSVLib.unpack(currentBalanceSSV));
        }
        emit OperatorRemoved(operatorId);
    }

    /**
     * @inheritdoc ISSVOperators
     */
    function declareOperatorFee(uint64 operatorId, uint256 fee) external override {
        StorageData storage s = SSVStorage.load();
        s.operators[operatorId].checkOwner();

        StorageProtocol storage sp = SSVStorageProtocol.load();

        if (fee != 0 && fee < PackedETHLib.unpack(sp.minimumOperatorEthFee)) revert FeeTooLow();
        if (fee > PackedETHLib.unpack(sp.operatorMaxFee)) revert FeeTooHigh();
        if (s.operators[operatorId].ethSnapshot.block == 0) {
            s.operators[operatorId].ensureETHDefaults(operatorId);
        }
        PackedSSV operatorSSVFee = s.operators[operatorId].fee;
        PackedETH operatorFee = s.operators[operatorId].ethFee;
        PackedETH shrunkFee = PackedETHLib.pack(fee);

        if (operatorFee.eq(shrunkFee)) {
            revert SameFeeChangeNotAllowed();
        } else if (shrunkFee.raw() != 0 && operatorFee.raw() == 0 && operatorSSVFee.raw() == 0) {
            revert FeeIncreaseNotAllowed();
        }

        // @dev 100%  =  10000, 10% = 1000 - using 10000 to represent 2 digit precision
        uint64 maxAllowedFee = (operatorFee.raw() * (BPS_DENOMINATOR + sp.operatorMaxFeeIncrease) + BPS_DENOMINATOR - 1) / BPS_DENOMINATOR;

        if (shrunkFee.raw() > maxAllowedFee) revert FeeExceedsIncreaseLimit();

        s.operatorFeeChangeRequests[operatorId] = OperatorFeeChangeRequest(
            PackedETH.unwrap(shrunkFee),
            uint64(block.timestamp) + sp.declareOperatorFeePeriod,
            uint64(block.timestamp) + sp.declareOperatorFeePeriod + sp.executeOperatorFeePeriod
        );
        emit OperatorFeeDeclared(msg.sender, operatorId, block.number, fee);
    }

    /**
     * @inheritdoc ISSVOperators
     */
    function executeOperatorFee(uint64 operatorId) external override {
        StorageData storage s = SSVStorage.load();
        s.operators[operatorId].checkOwner();

        OperatorFeeChangeRequest memory feeChangeRequest = s.operatorFeeChangeRequests[operatorId];

        if (feeChangeRequest.approvalBeginTime == 0) revert NoFeeDeclared();

        if (feeChangeRequest.approvalBeginTime <= UPGRADE_TIMESTAMP) {
            revert LegacyOperatorFeeDeclarationInvalid();
        }

        if (
            block.timestamp < feeChangeRequest.approvalBeginTime || block.timestamp > feeChangeRequest.approvalEndTime
        ) {
            revert ApprovalNotWithinTimeframe();
        }

        StorageProtocol storage sp = SSVStorageProtocol.load();
        if (PackedETH.wrap(feeChangeRequest.fee).gt(sp.operatorMaxFee)) revert FeeTooHigh();
        if (feeChangeRequest.fee != 0 && feeChangeRequest.fee < PackedETH.unwrap(sp.minimumOperatorEthFee)) revert FeeTooLow();

        Operator storage operator = s.operators[operatorId];
        OperatorLib.updateSnapshotSt(operator, operatorId);
        operator.ethFee = PackedETH.wrap(feeChangeRequest.fee);

        delete s.operatorFeeChangeRequests[operatorId];

        emit OperatorFeeExecuted(msg.sender, operatorId, block.number, PackedETHLib.unpack(PackedETH.wrap(feeChangeRequest.fee)));
    }

    /**
     * @inheritdoc ISSVOperators
     */
    function cancelDeclaredOperatorFee(uint64 operatorId) external override {
        StorageData storage s = SSVStorage.load();
        s.operators[operatorId].checkOwner();

        if (s.operatorFeeChangeRequests[operatorId].approvalBeginTime == 0) revert NoFeeDeclared();

        delete s.operatorFeeChangeRequests[operatorId];

        emit OperatorFeeDeclarationCancelled(msg.sender, operatorId);
    }

    /**
     * @inheritdoc ISSVOperators
     */
    function reduceOperatorFee(uint64 operatorId, uint256 fee) external override {
        StorageData storage s = SSVStorage.load();
        s.operators[operatorId].checkOwner();

        if (fee != 0 && fee < PackedETHLib.unpack(SSVStorageProtocol.load().minimumOperatorEthFee)) revert FeeTooLow();

        if (s.operators[operatorId].ethSnapshot.block == 0) {
            s.operators[operatorId].ensureETHDefaults(operatorId);
        }

        Operator memory operator = s.operators[operatorId]; 

        PackedETH shrunkAmount = PackedETHLib.pack(fee);
        if (shrunkAmount.gte(operator.ethFee)) revert FeeIncreaseNotAllowed();

        operator.updateSnapshot(operatorId);
        operator.ethFee = shrunkAmount;
        s.operators[operatorId] = operator;

        delete s.operatorFeeChangeRequests[operatorId];

        emit OperatorFeeExecuted(msg.sender, operatorId, block.number, fee);
    }

    /**
     * @inheritdoc ISSVOperators
     */
    function setOperatorsPrivateUnchecked(uint64[] calldata operatorIds) external override {
        OperatorLib.updatePrivacyStatus(operatorIds, true, SSVStorage.load());
        emit OperatorPrivacyStatusUpdated(operatorIds, true);
    }

    /**
     * @inheritdoc ISSVOperators
     */
    function setOperatorsPublicUnchecked(uint64[] calldata operatorIds) external override {
        OperatorLib.updatePrivacyStatus(operatorIds, false, SSVStorage.load());
        emit OperatorPrivacyStatusUpdated(operatorIds, false);
    }

    /**
     * @inheritdoc ISSVOperators
     */
    function withdrawOperatorEarnings(uint64 operatorId, uint256 amount) external override nonReentrant {
        _withdrawOperatorEarnings(operatorId, amount, VERSION_ETH);
    }

    /**
     * @inheritdoc ISSVOperators
     */
    function withdrawAllOperatorEarnings(uint64 operatorId) external override nonReentrant {
        _withdrawOperatorEarnings(operatorId, 0, VERSION_ETH);
    }

    /**
     * @inheritdoc ISSVOperators
     */
    function withdrawAllVersionOperatorEarnings(uint64 operatorId) external override nonReentrant {
        StorageData storage s = SSVStorage.load();
        Operator storage operator = s.operators[operatorId];
        operator.checkOwner();

        PackedETH ethBalance = PACKED_ETH_ZERO;
        PackedSSV ssvBalance = PACKED_SSV_ZERO;

        if (operator.snapshot.block != 0) {
            OperatorLib.updateSnapshotStSSV(operator);
            ssvBalance = operator.snapshot.balance;
            operator.snapshot.balance = PACKED_SSV_ZERO;
        }

        if (operator.ethSnapshot.block != 0) {
            OperatorLib.updateSnapshotSt(operator, operatorId);
            ethBalance = operator.ethSnapshot.balance;
            operator.ethSnapshot.balance = PACKED_ETH_ZERO;
        }

        if (PackedETHLib.raw(ethBalance) > 0) {
            _transferOperatorBalanceUnsafe(operatorId, PackedETHLib.unpack(ethBalance));
        }
        if (PackedSSVLib.raw(ssvBalance) > 0) {
            _transferOperatorTokenBalanceUnsafe(operatorId, PackedSSVLib.unpack(ssvBalance));
        }
    }

    /**
     * @inheritdoc ISSVOperators
     */
    function withdrawOperatorEarningsSSV(uint64 operatorId, uint256 amount) external override nonReentrant {
        _withdrawOperatorEarnings(operatorId, amount, VERSION_SSV);
    }

    /**
     * @inheritdoc ISSVOperators
     */
    function withdrawAllOperatorEarningsSSV(uint64 operatorId) external override nonReentrant {
        _withdrawOperatorEarnings(operatorId, 0, VERSION_SSV);
    }

    // private functions
    function _withdrawOperatorEarnings(
        uint64 operatorId,
        uint256 amount,
        uint8 version
    ) private {
        StorageData storage s = SSVStorage.load();
        Operator storage operator = s.operators[operatorId];

        operator.checkOwner();

        if (version == VERSION_ETH) {
            if (operator.ethSnapshot.block == 0) revert InsufficientBalance();
            
            PackedETH shrunkWithdrawn;
            PackedETH shrunkAmount = PackedETHLib.pack(amount);
            OperatorLib.updateSnapshotSt(operator, operatorId);

            PackedETH balance = operator.ethSnapshot.balance;

            if (amount == 0) {
                if (PackedETHLib.raw(balance) == 0) revert InsufficientBalance();
                shrunkWithdrawn = balance;
            } else {
                if (PackedETHLib.raw(balance) < PackedETHLib.raw(shrunkAmount)) revert InsufficientBalance();
                shrunkWithdrawn = shrunkAmount;
            }

            operator.ethSnapshot.balance = balance.sub(shrunkWithdrawn);
            _transferOperatorBalanceUnsafe(operatorId, PackedETHLib.unpack(shrunkWithdrawn));

        } else if (version == VERSION_SSV) {
            if (operator.snapshot.block == 0) revert InsufficientBalance();

            PackedSSV shrunkWithdrawn;
            PackedSSV shrunkAmount = PackedSSVLib.pack(amount);
            OperatorLib.updateSnapshotStSSV(operator);

            PackedSSV balance = operator.snapshot.balance;

            if (amount == 0) {
                if (PackedSSVLib.raw(balance) == 0) revert InsufficientBalance();
                shrunkWithdrawn = balance;
            } else {
                if (PackedSSVLib.raw(balance) < PackedSSVLib.raw(shrunkAmount)) revert InsufficientBalance();
                shrunkWithdrawn = shrunkAmount;
            }

            operator.snapshot.balance = balance.sub(shrunkWithdrawn);
            _transferOperatorTokenBalanceUnsafe(operatorId, PackedSSVLib.unpack(shrunkWithdrawn));

        } else {
            revert IncorrectOperatorVersion(version);
        }
    }

    function _resetOperatorState(Operator storage operator) private returns (Operator memory) {
        operator.ethSnapshot.block = 0;
        operator.ethSnapshot.balance = PACKED_ETH_ZERO;
        operator.ethFee = PACKED_ETH_ZERO;
        operator.snapshot.block = 0;
        operator.snapshot.balance = PACKED_SSV_ZERO;
        operator.fee = PACKED_SSV_ZERO;
        operator.ethValidatorCount = 0;
        operator.validatorCount = 0;
        
        return operator;
    }

    function _transferOperatorBalanceUnsafe(uint64 operatorId, uint256 amount) private {
        CoreLib.transferBalance(payable(msg.sender), amount);
        emit OperatorWithdrawn(msg.sender, operatorId, amount);
    }

    function _transferOperatorTokenBalanceUnsafe(uint64 operatorId, uint256 amount) private {
        CoreLib.transferTokenBalance(msg.sender, amount);
        emit OperatorWithdrawnSSV(msg.sender, operatorId, amount);
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

import {ISSVStaking} from "../interfaces/ISSVStaking.sol";
import {ICSSVToken} from "../interfaces/ICSSVToken.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {ProtocolLib} from "../libraries/ProtocolLib.sol";
import {SSVStorage} from "../libraries/storage/SSVStorage.sol";
import {SSVStorageStaking, StorageStaking, UnstakeRequest} from "../libraries/storage/SSVStorageStaking.sol";
import {SSVStorageProtocol, StorageProtocol} from "../libraries/storage/SSVStorageProtocol.sol";
import {SSVReentrancyGuard} from "../abstract/SSVReentrancyGuard.sol";
import {PackedETH} from "../libraries/SSVCoreTypes.sol";
import {PackedETHLib} from "../libraries/SSVPackedLib.sol";
import {PRECISION, ETH_DEDUCTED_DIGITS} from "../libraries/SSVCoreTypes.sol";

contract SSVStaking is ISSVStaking, SSVReentrancyGuard {
    using SafeERC20 for IERC20;
    using ProtocolLib for StorageProtocol;
    using PackedETHLib for PackedETH;

    uint64 private constant MINIMAL_STAKING_AMOUNT = 1_000_000_000;
    uint256 private constant MAX_PENDING_REQUESTS = 2000;

    address public immutable CSSV_ADDRESS;

    constructor(address _cssv) {
        CSSV_ADDRESS = _cssv;
    }

    /**
     * @inheritdoc ISSVStaking
     */
    function syncFees() external nonReentrant {
        _syncFees(SSVStorageStaking.load());
    }

    /**
     * @inheritdoc ISSVStaking
     */
    function stake(uint256 amount) external nonReentrant {
        if (amount < MINIMAL_STAKING_AMOUNT) {
            revert StakeTooLow();
        }

        StorageStaking storage s = SSVStorageStaking.load();

        _syncFees(s);
        _settle(msg.sender, s);

        if (!SSVStorage.load().token.transferFrom(msg.sender, address(this), amount)) {
            revert TokenTransferFailed();
        }

        ICSSVToken(CSSV_ADDRESS).mint(msg.sender, amount);

        emit Staked(msg.sender, amount);
    }

    /**
     * @inheritdoc ISSVStaking
     */
    function requestUnstake(uint256 amount) external nonReentrant {
        if (amount == 0) {
            revert ZeroAmount();
        }

        StorageStaking storage s = SSVStorageStaking.load();

        _syncFees(s);

        uint256 bal = ICSSVToken(CSSV_ADDRESS).balanceOf(msg.sender);
        _settleWithBalance(msg.sender, bal, s);

        if (amount > bal) {
            revert UnstakeAmountExceedsBalance();
        }

        UnstakeRequest[] storage requests = s.withdrawalRequests[msg.sender];

        if (requests.length == MAX_PENDING_REQUESTS) {
            revert MaxRequestsAmountReached();
        }

        uint64 unlockTime = uint64(block.timestamp + s.cooldownDuration);
        requests.push(UnstakeRequest({amount: uint192(amount), unlockTime: unlockTime}));

        ICSSVToken(CSSV_ADDRESS).burn(msg.sender, amount);

        emit UnstakeRequested(msg.sender, amount, unlockTime);
    }

    /**
     * @inheritdoc ISSVStaking
     */
    function withdrawUnlocked() external nonReentrant {
        StorageStaking storage s = SSVStorageStaking.load();
        uint256 amount = calculateTotalUnfrozenBalance(s);
        if (amount == 0) revert NothingToWithdraw();

        if (!SSVStorage.load().token.transfer(msg.sender, amount)) {
            revert TokenTransferFailed();
        }

        emit UnstakedWithdrawn(msg.sender, amount);
    }

    /**
     * @inheritdoc ISSVStaking
     */
    function claimEthRewards() external nonReentrant {
        StorageStaking storage s = SSVStorageStaking.load();

        _syncFees(s);
        _settle(msg.sender, s);

        uint256 claimable = s.accrued[msg.sender];
        if (claimable == 0) revert NothingToClaim();

        uint256 payout = claimable - (claimable % ETH_DEDUCTED_DIGITS);
        uint256 userBalance = ICSSVToken(CSSV_ADDRESS).balanceOf(msg.sender);
        if (payout == 0) {
            if (userBalance == 0) {
                s.accrued[msg.sender] = 0;
                emit RewardsClaimed(msg.sender, 0);
                return;
            }
            revert NothingToClaim();
        }

        PackedETH packedPayout = PackedETHLib.pack(payout);

        StorageProtocol storage sp = SSVStorageProtocol.load();

        if (packedPayout.gt(s.stakingEthPoolBalance)) {
            revert InsufficientBalance();
        }
        if (packedPayout.gt(sp.ethDaoBalance))   {
            revert InsufficientBalance();
        }

        uint256 remainder = claimable - payout;
        s.accrued[msg.sender] = (remainder != 0 && userBalance == 0) ? 0 : remainder;
        s.stakingEthPoolBalance = s.stakingEthPoolBalance.sub(packedPayout);
        sp.ethDaoBalance = sp.ethDaoBalance.sub(packedPayout);

        CoreLib.transferBalance(msg.sender, payout);
        emit RewardsClaimed(msg.sender, payout);
    }

    /**
     * @inheritdoc ISSVStaking
     */
    function rescueERC20(address token, address to, uint256 amount) external nonReentrant {
        if (token == address(0) || to == address(0)) revert ZeroAddress();
        if (token == address(SSVStorage.load().token) || token == CSSV_ADDRESS) {
            revert InvalidToken();
        }
        if (amount == 0) {
            revert ZeroAmount();
        }

        IERC20(token).safeTransfer(to, amount);

        emit ERC20Rescued(token, to, amount);
    }

    /**
     * @inheritdoc ISSVStaking
     */
    function onCSSVTransfer(address from, address to, uint256 amount) external virtual {
        if (msg.sender != CSSV_ADDRESS) revert NotCSSV();

        StorageStaking storage s = SSVStorageStaking.load();

        _syncFees(s);
        _settle(from, s);
        _settle(to, s);
    }

    function _syncFees(StorageStaking storage s) internal {
        StorageProtocol storage sp = SSVStorageProtocol.load();

        PackedETH current = sp.networkTotalEarnings();
        sp.ethDaoBalance = current;
        sp.ethDaoIndexBlockNumber = uint32(block.number);

        PackedETH previous = s.stakingEthPoolBalance;
        if (current.lte(previous)) {
            s.stakingEthPoolBalance = current;
            return;
        }

        PackedETH packedNewFees = current.sub(previous);
        uint256 newFeesWei;

        uint256 totalStaked = ICSSVToken(CSSV_ADDRESS).totalSupply();
        if (totalStaked != 0) {
            newFeesWei = PackedETHLib.unpack(packedNewFees);
            s.accEthPerShare += uint128((newFeesWei * PRECISION) / totalStaked);
        }

        s.stakingEthPoolBalance = current;
        emit FeesSynced(newFeesWei, s.accEthPerShare);
    }

    function _settle(address user, StorageStaking storage s) internal {
        uint256 bal = ICSSVToken(CSSV_ADDRESS).balanceOf(user);
        _settleWithBalance(user, bal, s);
    }

    function _settleWithBalance(address user, uint256 bal, StorageStaking storage s) internal {
        uint256 idx = s.accEthPerShare;
        uint256 userIdx = s.userIndex[user];

        uint256 pending;
        if (bal != 0 && idx != userIdx) {
            pending = (bal * (idx - userIdx)) / PRECISION;
            if (pending != 0) {
                s.accrued[user] += pending;
            }
        }

        s.userIndex[user] = idx;
        emit RewardsSettled(user, pending, s.accrued[user], idx);
    }

    function calculateTotalUnfrozenBalance(StorageStaking storage s) internal returns (uint256) {
        UnstakeRequest[] storage requests = s.withdrawalRequests[msg.sender];
        uint256 total = 0;
        uint256 i = 0;

        while (i < requests.length) {
            if (requests[i].unlockTime <= block.timestamp) {
                total += requests[i].amount;
                requests[i] = requests[requests.length - 1];
                requests.pop();
            } else {
                i++;
            }
        }
        return total;
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {ISSVValidators} from "../interfaces/ISSVValidators.sol";
import {ClusterLib} from "../libraries/ClusterLib.sol";
import {OperatorLib} from "../libraries/OperatorLib.sol";
import {ProtocolLib} from "../libraries/ProtocolLib.sol";
import {ValidatorLib} from "../libraries/ValidatorLib.sol";
import {VERSION_ETH, VERSION_SSV, BPS_DENOMINATOR} from "../libraries/SSVCoreTypes.sol";
import {SSVStorage, StorageData} from "../libraries/storage/SSVStorage.sol";
import {SSVStorageProtocol, StorageProtocol} from "../libraries/storage/SSVStorageProtocol.sol";
import {
    SSVStorageEB,
    StorageEB,
    ClusterEBSnapshot
} from "../libraries/storage/SSVStorageEB.sol";

contract SSVValidators is ISSVValidators {
    using ClusterLib for Cluster;
    using OperatorLib for Operator;
    using ProtocolLib for StorageProtocol;

    /**
     * @inheritdoc ISSVValidators
     */
    function registerValidator(
        bytes calldata publicKey,
        uint64[] memory operatorIds,
        bytes calldata sharesData,
        Cluster memory cluster
    ) external payable override {
        bytes[] memory publicKeys = new bytes[](1);
        publicKeys[0] = publicKey;

        bytes[] memory shares = new bytes[](1);
        shares[0] = sharesData;

        _bulkRegisterValidator(msg.sender, msg.value, publicKeys, operatorIds, shares, cluster);
    }

    /**
     * @inheritdoc ISSVValidators
     */
    function bulkRegisterValidator(
        bytes[] memory publicKeys,
        uint64[] memory operatorIds,
        bytes[] calldata sharesData,
        Cluster memory cluster
    ) external payable override {
        _bulkRegisterValidator(msg.sender, msg.value, publicKeys, operatorIds, sharesData, cluster);
    }

    /**
     * @inheritdoc ISSVValidators
     */
    function removeValidator(
        bytes calldata publicKey,
        uint64[] memory operatorIds,
        Cluster memory cluster
    ) external override {
        bytes[] memory publicKeys = new bytes[](1);
        publicKeys[0] = publicKey;

        _bulkRemoveValidator(msg.sender, publicKeys, operatorIds, cluster);
    }

    /**
     * @inheritdoc ISSVValidators
     */
    function bulkRemoveValidator(
        bytes[] calldata publicKeys,
        uint64[] memory operatorIds,
        Cluster memory cluster
    ) external override {
        _bulkRemoveValidator(msg.sender, publicKeys, operatorIds, cluster);
    }

    /**
     * @inheritdoc ISSVValidators
     */
    function exitValidator(bytes calldata publicKey, uint64[] calldata operatorIds) external override {
        StorageData storage s = SSVStorage.load();
        _validateExistingValidator(publicKey, msg.sender, ValidatorLib.hashOperatorIds(operatorIds), s);

        emit ValidatorExited(msg.sender, operatorIds, publicKey);
    }

    /**
     * @inheritdoc ISSVValidators
     */
    function bulkExitValidator(bytes[] calldata publicKeys, uint64[] calldata operatorIds) external override {
        if (publicKeys.length == 0) {
            revert ValidatorDoesNotExist();
        }
        StorageData storage s = SSVStorage.load();
        bytes32 hashedOperatorIds = ValidatorLib.hashOperatorIds(operatorIds);

        for (uint i; i < publicKeys.length; ++i) {
            _validateExistingValidator(publicKeys[i], msg.sender, hashedOperatorIds, s);

            emit ValidatorExited(msg.sender, operatorIds, publicKeys[i]);
        }
    }

    function _bulkRegisterValidator(
        address owner,
        uint256 value,
        bytes[] memory publicKeys,
        uint64[] memory operatorIds,
        bytes[] memory sharesData,
        Cluster memory cluster
    ) internal virtual {
        uint256 validatorsLength = publicKeys.length;

        if (validatorsLength == 0) revert EmptyPublicKeysList();
        if (validatorsLength != sharesData.length) revert PublicKeysSharesLengthMismatch();

        StorageData storage s = SSVStorage.load();
        StorageProtocol storage sp = SSVStorageProtocol.load();

        ValidatorLib.validateOperatorsLength(operatorIds);

        for (uint i; i < validatorsLength; ++i) {
            ValidatorLib.registerPublicKey(publicKeys[i], operatorIds, owner, s);
        }
        bytes32 hashedCluster = cluster.validateClusterOnRegistration(owner, operatorIds, s);

        cluster.balance += value;

        cluster.updateClusterOnRegistration(operatorIds, hashedCluster, uint32(validatorsLength), s, sp);

        {
            // Deviation-only model: baseline comes from ethValidatorCount (already updated above)
            // Only update ebSnapshot.vUnits for clusters with explicit EB tracking
            // Do NOT update operatorEthVUnits here - deviation unchanged on registration
            StorageEB storage seb = SSVStorageEB.load();
            ClusterEBSnapshot storage ebSnapshot = seb.clusterEB[hashedCluster];
            if (ebSnapshot.vUnits > 0) {
                // Cluster has explicit EB tracking - add baseline for new validators
                ebSnapshot.vUnits += uint64(validatorsLength) * BPS_DENOMINATOR;
            }
            // operatorEthVUnits NOT updated: deviation doesn't change on registration
        }

        for (uint i; i < validatorsLength; ++i) {
            bytes memory pk = publicKeys[i];
            bytes memory sh = sharesData[i];

            emit ValidatorAdded(owner, operatorIds, pk, sh, cluster);
        }
    }

    function _bulkRemoveValidator(
        address owner,
        bytes[] memory publicKeys,
        uint64[] memory operatorIds,
        Cluster memory cluster
    ) internal virtual {
        uint256 validatorsLength = publicKeys.length;

        if (validatorsLength == 0) {
            revert ValidatorDoesNotExist();
        }
        StorageData storage s = SSVStorage.load();

        (bytes32 hashedCluster, uint8 version) = cluster.validateHashedCluster(owner, operatorIds, s);
        bytes32 hashedOperatorIds = ValidatorLib.hashOperatorIds(operatorIds);

        uint32 validatorsRemoved;

        for (uint i; i < validatorsLength; ++i) {
            bytes32 hashedValidator = _validateExistingValidator(publicKeys[i], owner, hashedOperatorIds, s);

            delete s.validatorPKs[hashedValidator];
            validatorsRemoved++;
        }

        if (version == VERSION_ETH) {
            if (cluster.active) {
                StorageProtocol storage sp = SSVStorageProtocol.load();
                // slither-disable-next-line unused-return
                (uint64 clusterIndex, ) = OperatorLib.updateClusterOperators(
                    operatorIds,
                    false,
                    validatorsRemoved,
                    s,
                    sp
                );

                cluster.updateClusterData(hashedCluster, clusterIndex, sp.currentNetworkFeeIndex());

                sp.updateDAO(false, validatorsRemoved);
            }

            cluster.validatorCount -= validatorsRemoved;

            {
                // Deviation-only model: baseline removed via ethValidatorCount (already updated above)
                // Do NOT subtract baseline from operatorEthVUnits
                // Only handle deviation cleanup for explicit EB clusters
                StorageEB storage seb = SSVStorageEB.load();
                ClusterEBSnapshot storage ebSnapshot = seb.clusterEB[hashedCluster];
                
                if (ebSnapshot.vUnits > 0) {
                    // Cluster has explicit EB tracking - subtract baseline from snapshot
                    uint64 deltaClusterVUnits = uint64(validatorsRemoved) * BPS_DENOMINATOR;
                    ebSnapshot.vUnits -= deltaClusterVUnits;
                    
                    // When cluster becomes empty, clean up any remaining deviation
                    if (cluster.validatorCount == 0) {
                        uint64 remainingVUnits = ebSnapshot.vUnits;
                        if (remainingVUnits > 0 && cluster.active) {
                            // remainingVUnits is pure deviation (no baseline left since validatorCount=0)
                            // Skip for liquidated clusters: deviation already cleaned up in _executeLiquidation
                            uint256 operatorsLength = operatorIds.length;
                            for (uint256 i; i < operatorsLength; ++i) {
                                if (s.operators[operatorIds[i]].ethSnapshot.block == 0) continue;
                                seb.operatorEthVUnits[operatorIds[i]] -= remainingVUnits;
                            }
                            StorageProtocol storage sp = SSVStorageProtocol.load();
                            sp.updateDAOEthVUnits(remainingVUnits, 0);
                        }
                        ebSnapshot.vUnits = 0;
                    }
                }
                // For implicit clusters (ebSnapshot.vUnits == 0): nothing to do
                // Baseline removal handled via ethValidatorCount decrement
            }

            s.ethClusters[hashedCluster] = cluster.hashClusterData();
        } else if (version == VERSION_SSV) {
            if (cluster.active) {
                StorageProtocol storage sp = SSVStorageProtocol.load();
                // slither-disable-next-line unused-return
                (uint64 clusterIndex, ) = OperatorLib.updateClusterOperatorsSSV(
                    operatorIds,
                    false,
                    validatorsRemoved,
                    s,
                    sp
                );
                uint64 currentNetworkFeeIndexSSV = sp.currentNetworkFeeIndexSSV();
                cluster.updateBalanceSSV(clusterIndex, currentNetworkFeeIndexSSV);
                cluster.index = clusterIndex;
                cluster.networkFeeIndex = currentNetworkFeeIndexSSV;
                sp.updateDAOSSV(false, validatorsRemoved);
            }

            cluster.validatorCount -= validatorsRemoved;
            s.clusters[hashedCluster] = cluster.hashClusterData();
        } else {
            revert IncorrectClusterVersion();
        }

        for (uint i; i < validatorsLength; ++i) {
            emit ValidatorRemoved(owner, operatorIds, publicKeys[i], cluster);
        }
    }

    function _validateExistingValidator(
        bytes memory publicKey,
        address owner,
        bytes32 hashedOperatorIds,
        StorageData storage s
    ) internal view returns (bytes32 hashedValidator) {
        hashedValidator = keccak256(abi.encodePacked(publicKey, owner));
        bytes32 validatorData = s.validatorPKs[hashedValidator];
        if (validatorData == bytes32(0)) {
            revert ValidatorDoesNotExist();
        }
        if (!ValidatorLib.validateCorrectState(validatorData, hashedOperatorIds)) {
            revert IncorrectValidatorStateWithData(publicKey);
        }
    }

}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {ISSVDAO} from "../interfaces/ISSVDAO.sol";
import {ProtocolLib} from "../libraries/ProtocolLib.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {PackedSSV, PackedETH, BPS_DENOMINATOR} from "../libraries/SSVCoreTypes.sol";
import {PackedSSVLib, PackedETHLib} from "../libraries/SSVPackedLib.sol";
import {SSVStorageProtocol, StorageProtocol} from "../libraries/storage/SSVStorageProtocol.sol";
import {SSVStorageEB, StorageEB} from "../libraries/storage/SSVStorageEB.sol";
import {ICSSVToken} from "../interfaces/ICSSVToken.sol";
import {SSVStorageStaking, StorageStaking, MAX_DELEGATION_SLOTS} from "../libraries/storage/SSVStorageStaking.sol";
import {SSVReentrancyGuard} from "../abstract/SSVReentrancyGuard.sol";

contract SSVDAO is ISSVDAO, SSVReentrancyGuard {
    using ProtocolLib for StorageProtocol;
    using PackedSSVLib for PackedSSV;

    uint64 private constant MINIMAL_LIQUIDATION_THRESHOLD = 21_480;
    address public immutable CSSV_ADDRESS;

    constructor(address _cssv) {
        CSSV_ADDRESS = _cssv;
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function updateNetworkFee(uint256 fee) external override {
        StorageProtocol storage sp = SSVStorageProtocol.load();
        PackedETH previousFee = sp.ethNetworkFee;

        sp.updateNetworkFee(fee);
        emit NetworkFeeUpdated(PackedETHLib.unpack(previousFee), fee);
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function updateNetworkFeeSSV(uint256 fee) external override {
        StorageProtocol storage sp = SSVStorageProtocol.load();
        PackedSSV previousFee = sp.networkFee;

        sp.updateNetworkFeeSSV(fee);
        emit NetworkFeeUpdatedSSV(PackedSSVLib.unpack(previousFee), fee);
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function withdrawNetworkSSVEarnings(uint256 amount) external override nonReentrant {
        StorageProtocol storage sp = SSVStorageProtocol.load();

        PackedSSV shrunkAmount = PackedSSVLib.pack(amount);

        PackedSSV networkBalance = sp.networkTotalEarningsSSV();

        if (shrunkAmount.gt(networkBalance)) {
            revert InsufficientBalance();
        }

        sp.daoBalance = networkBalance.sub(shrunkAmount);
        sp.daoIndexBlockNumber = uint32(block.number);

        CoreLib.transferTokenBalance(msg.sender, amount);

        emit NetworkEarningsWithdrawn(amount, msg.sender);
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function updateOperatorFeeIncreaseLimit(uint64 percentage) external override {
        if (percentage > BPS_DENOMINATOR) {
            revert InvalidOperatorFeeIncreaseLimit();
        }

        SSVStorageProtocol.load().operatorMaxFeeIncrease = percentage;
        emit OperatorFeeIncreaseLimitUpdated(percentage);
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function updateDeclareOperatorFeePeriod(uint64 timeInSeconds) external override {
        SSVStorageProtocol.load().declareOperatorFeePeriod = timeInSeconds;
        emit DeclareOperatorFeePeriodUpdated(timeInSeconds);
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function updateExecuteOperatorFeePeriod(uint64 timeInSeconds) external override {
        SSVStorageProtocol.load().executeOperatorFeePeriod = timeInSeconds;
        emit ExecuteOperatorFeePeriodUpdated(timeInSeconds);
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function updateLiquidationThresholdPeriod(uint64 blocks) external override {
        if (blocks < MINIMAL_LIQUIDATION_THRESHOLD) {
            revert NewBlockPeriodIsBelowMinimum();
        }

        SSVStorageProtocol.load().minimumBlocksBeforeLiquidation = blocks;
        emit LiquidationThresholdPeriodUpdated(blocks);
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function updateLiquidationThresholdPeriodSSV(uint64 blocks) external {
        if (blocks < MINIMAL_LIQUIDATION_THRESHOLD) {
            revert NewBlockPeriodIsBelowMinimum();
        }

        SSVStorageProtocol.load().minimumBlocksBeforeLiquidationSSV = blocks;
        emit LiquidationThresholdPeriodSSVUpdated(blocks);
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function updateMinimumLiquidationCollateral(uint256 amount) external override {
        SSVStorageProtocol.load().minimumLiquidationCollateral = PackedETHLib.pack(amount);
        emit MinimumLiquidationCollateralUpdated(amount);
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function updateMinimumLiquidationCollateralSSV(uint256 amount) external {
        SSVStorageProtocol.load().minimumLiquidationCollateralSSV = PackedSSVLib.pack(amount);
        emit MinimumLiquidationCollateralSSVUpdated(amount);
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function updateMaximumOperatorFee(uint256 maxFee) external override {
        StorageProtocol storage sp = SSVStorageProtocol.load();
        if (maxFee < PackedETHLib.unpack(sp.minimumOperatorEthFee)) {
            revert InvalidOperatorFeeRange();
        }

        sp.operatorMaxFee = PackedETHLib.pack(maxFee);
        emit OperatorMaximumFeeUpdated(maxFee);
    }


    /**
     * @inheritdoc ISSVDAO
     */
    function updateMinimumOperatorEthFee(uint256 minFee) external override {
        StorageProtocol storage sp = SSVStorageProtocol.load();
        if (minFee > PackedETHLib.unpack(sp.operatorMaxFee)) {
            revert InvalidOperatorFeeRange();
        }

        sp.minimumOperatorEthFee = PackedETHLib.pack(minFee);
        emit MinimumOperatorEthFeeUpdated(minFee);
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function commitRoot(bytes32 merkleRoot, uint64 blockNum) external override {
        StorageEB storage seb = SSVStorageEB.load();
        StorageStaking storage s = SSVStorageStaking.load();

        uint32 oracleId = s.oracleIdOf[msg.sender];
        if (oracleId == 0) revert NotOracle();

        // Enforce monotonicity - new block must be greater than last
        if (blockNum <= seb.latestCommittedBlock) {
            revert StaleBlockNumber();
        }

        // Ensure block is not in the future
        if (blockNum > block.number) {
            revert FutureBlockNumber();
        }

        // block and root combined to keep block-root proposal tied together
        bytes32 commitmentKey = keccak256(abi.encodePacked(blockNum, merkleRoot));

        if (seb.hasVoted[commitmentKey][oracleId]) revert AlreadyVoted();
        seb.hasVoted[commitmentKey][oracleId] = true;

        uint256 oracleCount = s.defaultOracleIds.length;
        uint256 totalStaked = seb.roundFrozenSupply[commitmentKey];
        if (totalStaked == 0) {
            uint256 rawSupply = ICSSVToken(CSSV_ADDRESS).totalSupply();
            if (rawSupply == 0) revert ZeroCSSVSupply();

            totalStaked = rawSupply - (rawSupply % oracleCount);
            if (totalStaked == 0) revert InsufficientCSSVSupply();
            seb.roundFrozenSupply[commitmentKey] = totalStaked;
        }

        uint256 weight = totalStaked / oracleCount;
        seb.rootCommitments[commitmentKey] += weight;

        uint256 accumulatedWeight = seb.rootCommitments[commitmentKey];

        uint256 threshold = (totalStaked * s.quorumBps) / BPS_DENOMINATOR;

        emit WeightedRootProposed(merkleRoot, blockNum, accumulatedWeight, threshold, oracleId, msg.sender);

        if (accumulatedWeight >= threshold) {
            seb.ebRoots[blockNum] = merkleRoot;
            seb.latestCommittedBlock = blockNum;

            delete seb.rootCommitments[commitmentKey];
            delete seb.roundFrozenSupply[commitmentKey];
            // Do not delete hasVoted to prevent re-voting if same key is somehow reused

            emit RootCommitted(merkleRoot, blockNum);
        }
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function replaceOracle(uint32 oracleId, address newOracle) external override {
        StorageStaking storage s = SSVStorageStaking.load();
        if (oracleId == 0 || oracleId > MAX_DELEGATION_SLOTS) revert InvalidOracleId();
        if (newOracle == address(0)) revert ZeroAddress();

        address oldOracle = s.oracles[oracleId];
        if (oldOracle == newOracle) {
            revert SameOracleAddressNotAllowed();
        }

        // Clear reverse mapping for old oracle if existed
        if (oldOracle != address(0)) {
            s.oracleIdOf[oldOracle] = 0;
        }

        // Ensure newOracle is not already assigned to another ID
        uint32 existing = s.oracleIdOf[newOracle];
        if (existing != 0 && existing != oracleId) revert OracleAlreadyAssigned();

        s.oracles[oracleId] = newOracle;
        s.oracleIdOf[newOracle] = oracleId;

        emit OracleReplaced(oracleId, oldOracle, newOracle);
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function updateQuorumBps(uint16 quorum) external override {
        if (quorum == 0 || quorum > BPS_DENOMINATOR) {
            revert InvalidQuorum();
        }
        SSVStorageStaking.load().quorumBps = quorum;
        emit QuorumUpdated(quorum);
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function updateUnstakeCooldownDuration(uint64 duration) external override {
        SSVStorageStaking.load().cooldownDuration = duration;
        emit CooldownDurationUpdated(duration);
    }

    /**
     * @inheritdoc ISSVDAO
     */
    function updateMinBlocksBetweenUpdates(uint32 blocks) external override {
        SSVStorageEB.load().minBlocksBetweenUpdates = blocks;
        emit MinBlocksBetweenUpdatesUpdated(blocks);
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.20;

import {ISSVNetworkCore} from "./ISSVNetworkCore.sol";

/**
 * @title SSV Validators Interface
 * @author SSV Labs
 * @notice Interface for managing validators in the SSV network including registration, removal and exit operations
 */
interface ISSVValidators is ISSVNetworkCore {
    /**
     * @dev Emitted when a validator is added
     * @param owner The owner of the validator (and cluster)
     * @param operatorIds The operator IDs managing the validator
     * @param publicKey The validator's public key
     * @param shares The shares data
     * @param cluster The cluster data
     */
    event ValidatorAdded(
        address indexed owner,
        uint64[] operatorIds,
        bytes publicKey,
        bytes shares,
        Cluster cluster
    );

    /**
     * @dev Emitted when a validator is removed
     * @param owner The owner of the validator
     * @param operatorIds The operator IDs managing the validator
     * @param publicKey The validator's public key
     * @param cluster The cluster data
     */
    event ValidatorRemoved(
        address indexed owner,
        uint64[] operatorIds,
        bytes publicKey,
        Cluster cluster
    );

    /**
     * @dev Emitted when a validator exits
     * @param owner The owner of the validator
     * @param operatorIds The operator IDs managing the validator
     * @param publicKey The validator's public key
     */
    event ValidatorExited(
        address indexed owner,
        uint64[] operatorIds,
        bytes publicKey
    );

    /**
     * @notice Registers a new validator
     * @param publicKey Validator public key
     * @param operatorIds Operator IDs managing the validator
     * @param sharesData Encrypted shares data
     * @param cluster Cluster data
     */
    function registerValidator(
        bytes calldata publicKey,
        uint64[] memory operatorIds,
        bytes calldata sharesData,
        Cluster memory cluster
    ) external payable;

    /**
     * @notice Registers multiple new validators
     * @param publicKeys Array of validator public keys
     * @param operatorIds Operator IDs managing the validators
     * @param sharesData Array of encrypted shares data
     * @param cluster Cluster data
     */
    function bulkRegisterValidator(
        bytes[] calldata publicKeys,
        uint64[] memory operatorIds,
        bytes[] calldata sharesData,
        Cluster memory cluster
    ) external payable;

    /**
     * @notice Removes an existing validator
     * @param publicKey Validator public key
     * @param operatorIds Operator IDs managing the validator
     * @param cluster Cluster data
     */
    function removeValidator(
        bytes calldata publicKey,
        uint64[] memory operatorIds,
        Cluster memory cluster
    ) external;

    /**
     * @notice Removes multiple existing validators from the same cluster
     * @notice Reverts on duplicates or non-existent validators
     * @param publicKeys Array of validator public keys
     * @param operatorIds Operator IDs managing the validators
     * @param cluster Cluster data
     */
    function bulkRemoveValidator(
        bytes[] calldata publicKeys,
        uint64[] memory operatorIds,
        Cluster memory cluster
    ) external;

    /**
     * @notice Initiates exit for a validator
     * @param publicKey Validator public key
     * @param operatorIds Operator IDs managing the validator
     */
    function exitValidator(
        bytes calldata publicKey,
        uint64[] calldata operatorIds
    ) external;

    /**
     * @notice Initiates exit for multiple validators
     * @param publicKeys Array of validator public keys
     * @param operatorIds Operator IDs managing the validators
     */
    function bulkExitValidator(
        bytes[] calldata publicKeys,
        uint64[] calldata operatorIds
    ) external;
}
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/introspection/ERC165.sol";
import {ISSVWhitelistingContract} from "../interfaces/external/ISSVWhitelistingContract.sol";

contract BasicWhitelisting is ISSVWhitelistingContract, ERC165, Ownable {
    mapping(address => bool) private whitelisted;

    event AddressWhitelisted(address indexed account);
    event AddressRemovedFromWhitelist(address indexed account);

    function addWhitelistedAddress(address account) external onlyOwner {
        whitelisted[account] = true;
        emit AddressWhitelisted(account);
    }

    function removeWhitelistedAddress(address account) external onlyOwner {
        whitelisted[account] = false;
        emit AddressRemovedFromWhitelist(account);
    }

    function isWhitelisted(address account, uint256) external view override returns (bool) {
        return whitelisted[account];
    }

    function supportsInterface(bytes4 interfaceId) public view virtual override returns (bool) {
        return interfaceId == type(ISSVWhitelistingContract).interfaceId || super.supportsInterface(interfaceId);
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import "../../SSVNetwork.sol";
import {MAX_DELEGATION_SLOTS} from "../../libraries/storage/SSVStorageStaking.sol";

contract SSVNetworkSSVStakingUpgrade is SSVNetwork {
    /// @notice One-time initializer for the SSV Staking upgrade
    /// @param cooldownDuration Unstake cooldown duration in seconds (e.g. 604800 for 7 days)
    /// @param defaultOracleIds Default oracle IDs for new delegations
    /// @param quorumBps Oracle quorum in basis points
    function initializeSSVStaking(
        uint64 cooldownDuration,
        uint32[MAX_DELEGATION_SLOTS] memory defaultOracleIds,
        uint16 quorumBps
    ) external onlyOwner reinitializer(3) {
        if (quorumBps == 0 || quorumBps > 10_000) revert InvalidQuorum();

        // save staking storage updates
        StorageStaking storage s = SSVStorageStaking.load();
        s.cooldownDuration = cooldownDuration;
        s.defaultOracleIds = defaultOracleIds;
        s.quorumBps = quorumBps;

        emit CooldownDurationUpdated(cooldownDuration);
        emit QuorumUpdated(quorumBps);
        emit SSVNetworkUpgradeBlock("v2.0.0", block.number);
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import "./interfaces/ISSVNetwork.sol";

import "./interfaces/ISSVClusters.sol";
import "./interfaces/ISSVValidators.sol";
import "./interfaces/ISSVOperators.sol";
import "./interfaces/ISSVOperatorsWhitelist.sol";
import "./interfaces/ISSVDAO.sol";
import "./interfaces/ISSVViews.sol";
import "./interfaces/ISSVStaking.sol";
import "./interfaces/external/ISSVWhitelistingContract.sol";

import {PackedETHLib} from "./libraries/SSVPackedLib.sol";
import {CoreLib} from "./libraries/CoreLib.sol";
import {StorageProtocol, SSVStorageProtocol} from "./libraries/storage/SSVStorageProtocol.sol";
import {StorageData, SSVModules} from "./libraries/storage/SSVStorage.sol";
import {SSVStorageStaking, StorageStaking} from "./libraries/storage/SSVStorageStaking.sol";

import "./SSVProxy.sol";

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";

contract SSVNetwork is
    UUPSUpgradeable,
    Ownable2StepUpgradeable,
    ISSVNetwork,
    ISSVOperators,
    ISSVOperatorsWhitelist,
    ISSVClusters,
    ISSVValidators,
    ISSVDAO,
    ISSVStaking,
    SSVProxy
{
    /****************/
    /* Initializers */
    /****************/

    function initialize(
        IERC20 token_,
        ISSVOperators ssvOperators_,
        ISSVClusters ssvClusters_,
        ISSVDAO ssvDAO_,
        ISSVViews ssvViews_,
        NetworkInitParams calldata params
    ) external override initializer onlyProxy {
        __UUPSUpgradeable_init();
        __Ownable2Step_init();
        __SSVNetwork_init_unchained(
            token_,
            ssvOperators_,
            ssvClusters_,
            ssvDAO_,
            ssvViews_,
            params
        );
    }

    function __SSVNetwork_init_unchained(
        IERC20 token_,
        ISSVOperators ssvOperators_,
        ISSVClusters ssvClusters_,
        ISSVDAO ssvDAO_,
        ISSVViews ssvViews_,
        NetworkInitParams calldata params
    ) internal onlyInitializing {
        StorageData storage s = SSVStorage.load();
        StorageProtocol storage sp = SSVStorageProtocol.load();
        StorageStaking storage ss = SSVStorageStaking.load();
        s.token = token_;
        s.ssvContracts[SSVModules.SSV_OPERATORS] = address(ssvOperators_);
        s.ssvContracts[SSVModules.SSV_CLUSTERS] = address(ssvClusters_);
        s.ssvContracts[SSVModules.SSV_DAO] = address(ssvDAO_);
        s.ssvContracts[SSVModules.SSV_VIEWS] = address(ssvViews_);
        sp.minimumBlocksBeforeLiquidation = params.minimumBlocksBeforeLiquidation;
        sp.minimumLiquidationCollateral = PackedETHLib.pack(params.minimumLiquidationCollateral);
        sp.validatorsPerOperatorLimit = params.validatorsPerOperatorLimit;
        sp.declareOperatorFeePeriod = params.declareOperatorFeePeriod;
        sp.executeOperatorFeePeriod = params.executeOperatorFeePeriod;
        sp.operatorMaxFeeIncrease = params.operatorMaxFeeIncrease;
        ss.defaultOracleIds = params.defaultOracleIds;
        ss.quorumBps = params.quorumBps;
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /*****************/
    /* UUPS required */
    /*****************/

    function _authorizeUpgrade(address) internal override onlyOwner {}

    /*********************/
    /* Fallback function */
    /*********************/
    fallback() external {
        // Delegates the call to the address of the SSV Views module
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_VIEWS]);
    }

    /*******************************/
    /* Operator External Functions */
    /*******************************/

    function registerOperator(
        bytes calldata publicKey,
        uint256 fee,
        bool setPrivate
    ) external override returns (uint64 id) {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS]);
    }

    function removeOperator(uint64 operatorId) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS]);
    }

    function setOperatorsWhitelists(
        uint64[] calldata operatorIds,
        address[] calldata whitelistAddresses
    ) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS_WHITELIST]);
    }

    function removeOperatorsWhitelists(
        uint64[] calldata operatorIds,
        address[] calldata whitelistAddresses
    ) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS_WHITELIST]);
    }

    function setOperatorsWhitelistingContract(
        uint64[] calldata operatorIds,
        ISSVWhitelistingContract whitelistingContract
    ) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS_WHITELIST]);
    }

    function setOperatorsPrivateUnchecked(uint64[] calldata operatorIds) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS]);
    }

    function setOperatorsPublicUnchecked(uint64[] calldata operatorIds) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS]);
    }

    function removeOperatorsWhitelistingContract(uint64[] calldata operatorIds) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS_WHITELIST]);
    }

    function declareOperatorFee(uint64 operatorId, uint256 fee) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS]);
    }

    function executeOperatorFee(uint64 operatorId) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS]);
    }

    function cancelDeclaredOperatorFee(uint64 operatorId) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS]);
    }

    function reduceOperatorFee(uint64 operatorId, uint256 fee) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS]);
    }

    function withdrawOperatorEarnings(uint64 operatorId, uint256 amount) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS]);
    }

    function withdrawAllOperatorEarnings(uint64 operatorId) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS]);
    }

    function withdrawAllVersionOperatorEarnings(uint64 operatorId) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS]);
    }

    function withdrawOperatorEarningsSSV(uint64 operatorId, uint256 amount) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS]);
    }

    function withdrawAllOperatorEarningsSSV(uint64 operatorId) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_OPERATORS]);
    }

    /*******************************/
    /* Address External Functions */
    /*******************************/

    function setFeeRecipientAddress(address recipientAddress) external override {
        emit FeeRecipientAddressUpdated(msg.sender, recipientAddress);
    }

    /*******************************/
    /* Staking External Functions */
    /*******************************/

    function syncFees() external {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_STAKING]);
    }

    function stake(uint256 amount) external {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_STAKING]);
    }

    function requestUnstake(uint256 amount) external {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_STAKING]);
    }

    function withdrawUnlocked() external {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_STAKING]);
    }

    function claimEthRewards() external {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_STAKING]);
    }

    function rescueERC20(address token, address to, uint256 amount) external onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_STAKING]);
    }

    function onCSSVTransfer(address from, address to, uint256 amount) external {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_STAKING]);
    }

    /*******************************/
    /* Validator External Functions */
    /*******************************/

    function registerValidator(
        bytes calldata publicKey,
        uint64[] calldata operatorIds,
        bytes calldata sharesData,
        ISSVNetworkCore.Cluster memory cluster
    ) external payable override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_VALIDATORS]);
    }

    function bulkRegisterValidator(
        bytes[] calldata publicKeys,
        uint64[] calldata operatorIds,
        bytes[] calldata sharesData,
        ISSVNetworkCore.Cluster memory cluster
    ) external payable override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_VALIDATORS]);
    }

    function removeValidator(
        bytes calldata publicKey,
        uint64[] calldata operatorIds,
        ISSVNetworkCore.Cluster memory cluster
    ) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_VALIDATORS]);
    }

    function bulkRemoveValidator(
        bytes[] calldata publicKeys,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_VALIDATORS]);
    }

    function liquidate(
        address clusterOwner,
        uint64[] calldata operatorIds,
        ISSVNetworkCore.Cluster memory cluster
    ) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_CLUSTERS]);
    }

    function liquidateSSV(
        address clusterOwner,
        uint64[] calldata operatorIds,
        ISSVNetworkCore.Cluster memory cluster
    ) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_CLUSTERS]);
    }

    function reactivate(
        uint64[] calldata operatorIds,
        ISSVNetworkCore.Cluster memory cluster
    ) external payable override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_CLUSTERS]);
    }

    function deposit(
        address clusterOwner,
        uint64[] calldata operatorIds,
        ISSVNetworkCore.Cluster memory cluster
    ) external payable override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_CLUSTERS]);
    }

    function withdraw(
        uint64[] calldata operatorIds,
        uint256 amount,
        ISSVNetworkCore.Cluster memory cluster
    ) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_CLUSTERS]);
    }

    function updateClusterBalance(
        uint64 blockNum,
        address clusterOwner,
        uint64[] calldata operatorIds,
        ISSVNetworkCore.Cluster memory cluster,
        uint32 effectiveBalance,
        bytes32[] calldata merkleProof
    ) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_CLUSTERS]);
    }

    function migrateClusterToETH(
        uint64[] calldata operatorIds,
        ISSVNetworkCore.Cluster memory cluster
    ) external payable override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_CLUSTERS]);
    }

    function exitValidator(bytes calldata publicKey, uint64[] calldata operatorIds) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_VALIDATORS]);
    }

    function bulkExitValidator(bytes[] calldata publicKeys, uint64[] calldata operatorIds) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_VALIDATORS]);
    }

    function updateNetworkFee(uint256 fee) external override onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function updateNetworkFeeSSV(uint256 fee) external override onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function withdrawNetworkSSVEarnings(uint256 amount) external override onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function updateOperatorFeeIncreaseLimit(uint64 percentage) external override onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function updateDeclareOperatorFeePeriod(uint64 timeInSeconds) external override onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function updateExecuteOperatorFeePeriod(uint64 timeInSeconds) external override onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function updateLiquidationThresholdPeriod(uint64 blocks) external override onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function updateLiquidationThresholdPeriodSSV(uint64 blocks) external onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function updateMinimumLiquidationCollateral(uint256 amount) external override onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function updateMinimumLiquidationCollateralSSV(uint256 amount) external onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function updateMaximumOperatorFee(uint256 maxFee) external override onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function updateMinimumOperatorEthFee(uint256 minFee) external override onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function commitRoot(bytes32 merkleRoot, uint64 blockNum) external override {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function updateUnstakeCooldownDuration(uint64 duration) external onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function updateMinBlocksBetweenUpdates(uint32 blocks) external override onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function replaceOracle(uint32 oracleId, address newOracle) external override onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function updateQuorumBps(uint16 quorum) external override onlyOwner {
        _delegate(SSVStorage.load().ssvContracts[SSVModules.SSV_DAO]);
    }

    function getVersion() external pure override returns (string memory version) {
        return CoreLib.getVersion();
    }

    /*******************************/
    /* Upgrade Modules Function    */
    /*******************************/
    function updateModule(SSVModules moduleId, address moduleAddress) external onlyOwner {
        CoreLib.setModuleContract(moduleId, moduleAddress);
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.20;

import {ISSVNetworkCore} from "./ISSVNetworkCore.sol";
import {ISSVWhitelistingContract} from "./external/ISSVWhitelistingContract.sol";

/**
 * @title SSV Operators Whitelist Interface
 * @author SSV Labs
 * @notice Interface for managing whitelists for SSV operators including setting and removing whitelisted addresses and contracts
 */
interface ISSVOperatorsWhitelist is ISSVNetworkCore {
    /**
     * @dev Emitted when multiple addresses are added to whitelists for operators
     * @param operatorIds The IDs of the affected operators
     * @param whitelistAddresses The addresses added to the whitelists
     */
    event OperatorMultipleWhitelistUpdated(uint64[] operatorIds, address[] whitelistAddresses);

    /**
     * @dev Emitted when multiple addresses are removed from whitelists for operators
     * @param operatorIds The IDs of the affected operators
     * @param whitelistAddresses The addresses removed from the whitelists
     */
    event OperatorMultipleWhitelistRemoved(uint64[] operatorIds, address[] whitelistAddresses);

    /**
     * @dev Emitted when the whitelisting contract is updated for operators
     * @param operatorIds The IDs of the affected operators
     * @param whitelistingContract The new whitelisting contract address
     */
    event OperatorWhitelistingContractUpdated(uint64[] operatorIds, address whitelistingContract);

    /**
     * @notice Sets whitelisted addresses (EOAs or contracts) for multiple operators
     * @notice Updates do not affect existing validators
     * @notice Only new registrations use the updated whitelist
     * @param operatorIds Array of operator IDs to update
     * @param whitelistAddresses Array of addresses to whitelist
     */
    function setOperatorsWhitelists(
        uint64[] calldata operatorIds,
        address[] calldata whitelistAddresses
    ) external;

    /**
     * @notice Removes whitelisted addresses from multiple operators
     * @param operatorIds Array of operator IDs to update
     * @param whitelistAddresses Array of addresses to remove
     */
    function removeOperatorsWhitelists(
        uint64[] calldata operatorIds,
        address[] calldata whitelistAddresses
    ) external;

    /**
     * @notice Sets a whitelisting contract for multiple operators
     * @param operatorIds Array of operator IDs to update
     * @param whitelistingContract The whitelisting contract address
     */
    function setOperatorsWhitelistingContract(
        uint64[] calldata operatorIds,
        ISSVWhitelistingContract whitelistingContract
    ) external;

    /**
     * @notice Removes the whitelisting contract from multiple operators
     * @param operatorIds Array of operator IDs to update
     */
    function removeOperatorsWhitelistingContract(uint64[] calldata operatorIds) external;
}
// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.20;

import {ISSVNetworkCore} from "./ISSVNetworkCore.sol";

/**
 * @title SSV DAO Interface
 * @author SSV Labs
 * @dev Interface for DAO operations in the SSV network, including fee updates, period adjustments, and other governance functions
 */
interface ISSVDAO is ISSVNetworkCore {

    /**
     * @dev Emitted when the operator fee increase limit is updated
     * @param value The new limit value
     */
    event OperatorFeeIncreaseLimitUpdated(uint64 value);

    /**
     * @dev Emitted when the declare operator fee period is updated
     * @param value The new period value in seconds
     */
    event DeclareOperatorFeePeriodUpdated(uint64 value);

    /**
     * @dev Emitted when the execute operator fee period is updated
     * @param value The new period value in seconds
     */
    event ExecuteOperatorFeePeriodUpdated(uint64 value);

    /**
     * @dev Emitted when the liquidation threshold period is updated
     * @param value The new threshold in blocks
     */
    event LiquidationThresholdPeriodUpdated(uint64 value);

    /**
     * @dev Emitted when the SSV liquidation threshold period is updated
     * @param value The new threshold in blocks
     */
    event LiquidationThresholdPeriodSSVUpdated(uint64 value);

    /**
     * @dev Emitted when the minimum liquidation collateral is updated
     * @param value The new collateral amount
     */
    event MinimumLiquidationCollateralUpdated(uint256 value);

    /**
     * @dev Emitted when the SSV minimum liquidation collateral is updated
     * @param value The new collateral amount
     */
    event MinimumLiquidationCollateralSSVUpdated(uint256 value);

    /**
     * @dev Emitted when the network fee is updated
     * @param oldFee The previous fee
     * @param newFee The new fee
     */
    event NetworkFeeUpdated(uint256 oldFee, uint256 newFee);

    /**
     * @dev Emitted when the SSV network fee is updated
     * @param oldFee The previous fee
     * @param newFee The new fee
     */
    event NetworkFeeUpdatedSSV(uint256 oldFee, uint256 newFee);

    /**
     * @dev Emitted when network earnings are withdrawn
     * @param value The amount withdrawn
     * @param recipient The address receiving the funds
     */
    event NetworkEarningsWithdrawn(uint256 value, address recipient);

    /**
     * @dev Emitted when the maximum operator fee is updated
     * @param maxFee The new maximum fee
     */
    event OperatorMaximumFeeUpdated(uint256 maxFee);

    /**
     * @dev Emitted when the minimum operator ETH fee is updated
     * @param minFee The new minimum fee
     */
    event MinimumOperatorEthFeeUpdated(uint256 minFee);

    /**
     * @dev Emitted when an EB Merkle root is committed for a given block
     * @param merkleRoot The committed Merkle root
     * @param blockNum The block number the root corresponds to
     */
    event RootCommitted(bytes32 indexed merkleRoot, uint64 indexed blockNum);

    /**
     * @dev Emitted when the unstake cooldown duration is updated
     * @param newCooldownDuration The new duration in seconds
     */
    event CooldownDurationUpdated(uint64 newCooldownDuration);

    /**
     * @dev Emitted when a weighted root is proposed
     * @param merkleRoot The proposed Merkle root
     * @param blockNum The block number
     * @param accumulatedWeight The accumulated weight
     * @param quorum The quorum value
     * @param oracleId The oracle ID
     * @param oracle The oracle address
     */
    event WeightedRootProposed(bytes32 indexed merkleRoot, uint64 indexed blockNum, uint256 accumulatedWeight, uint256 quorum, uint32 oracleId, address oracle);

    /**
     * @dev Emitted when an oracle is replaced
     * @param oracleId The oracle ID
     * @param oldOracle The old oracle address
     * @param newOracle The new oracle address
     */
    event OracleReplaced(uint32 indexed oracleId, address indexed oldOracle, address indexed newOracle);

    /**
     * @dev Emitted when the quorum is updated
     * @param newQuorum The new quorum value
     */
    event QuorumUpdated(uint16 newQuorum);

    /**
     * @dev Emitted when the minimum block interval between EB updates is updated
     * @param newMinBlocksBetweenUpdates The new minimum block interval
     */
    event MinBlocksBetweenUpdatesUpdated(uint32 newMinBlocksBetweenUpdates);

    /**
     * @notice Updates the network fee (ETH post-migration)
     * @param fee The new network fee (ETH) to be set
     */
    function updateNetworkFee(uint256 fee) external;

    /**
     * @notice Updates the legacy network fee (SSV pre-migration)
     * @param fee The new network fee (SSV) to be set
     */
    function updateNetworkFeeSSV(uint256 fee) external;

    /**
     * @notice Withdraws legacy network earnings (SSV pre-migration)
     * @param amount The amount (SSV) to be withdrawn
     */
    function withdrawNetworkSSVEarnings(uint256 amount) external;

    /**
     * @notice Updates the limit on the percentage increase in operator fees
     * @param percentage The new percentage limit
     */
    function updateOperatorFeeIncreaseLimit(uint64 percentage) external;

    /**
     * @notice Updates the period for declaring operator fees
     * @param timeInSeconds The new period in seconds
     */
    function updateDeclareOperatorFeePeriod(uint64 timeInSeconds) external;

    /**
     * @notice Updates the period for executing operator fees
     * @param timeInSeconds The new period in seconds
     */
    function updateExecuteOperatorFeePeriod(uint64 timeInSeconds) external;

    /**
     * @notice Updates the liquidation threshold period
     * @param blocks The new liquidation threshold in blocks
     */
    function updateLiquidationThresholdPeriod(uint64 blocks) external;

    /**
     * @notice Updates the SSV liquidation threshold period
     * @param blocks The new liquidation threshold in blocks
     */
    function updateLiquidationThresholdPeriodSSV(uint64 blocks) external;

    /**
     * @notice Updates the minimum collateral required to prevent liquidation
     * @param amount The new minimum collateral amount
     */
    function updateMinimumLiquidationCollateral(uint256 amount) external;

    /**
     * @notice Updates the SSV minimum collateral required to prevent liquidation
     * @param amount The new minimum collateral amount (SSV)
     */
    function updateMinimumLiquidationCollateralSSV(uint256 amount) external;

    /**
     * @notice Updates the maximum fee an operator that uses SSV token can set
     * @param maxFee The new maximum fee
     */
    function updateMaximumOperatorFee(uint256 maxFee) external;

    /**
     * @notice Updates the minimum operator ETH fee
     * @param minFee The new minimum fee (ETH)
     */
    function updateMinimumOperatorEthFee(uint256 minFee) external;

    /**
     * @notice Commit Merkle root of all cluster EBs
     * @param merkleRoot Root of Merkle tree containing all cluster EBs
     * @param blockNum Block number when oracle computed this data (must be finalized and strictly increasing)
     */
    function commitRoot(bytes32 merkleRoot, uint64 blockNum) external;

    /**
     * @notice Sets the unstake cooldown duration
     * @param duration The new duration in seconds
     */
    function updateUnstakeCooldownDuration(uint64 duration) external;

    /**
     * @notice Sets the minimum block interval between EB updates for the same cluster
     * @param blocks The new minimum interval in blocks (must be non-zero)
     */
    function updateMinBlocksBetweenUpdates(uint32 blocks) external;

    /**
     * @notice Replace oracle address at a stable oracle ID
     * @param oracleId Stable oracle ID to update
     * @param newOracle New oracle address
     */
    function replaceOracle(uint32 oracleId, address newOracle) external;

    /**
     * @notice Sets the quorum BPS
     * @param quorum The new quorum value
     */
    function updateQuorumBps(uint16 quorum) external;
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.24;

import {ISSVClusters} from "../interfaces/ISSVClusters.sol";
import {ClusterLib} from "../libraries/ClusterLib.sol";
import {OperatorLib} from "../libraries/OperatorLib.sol";
import {ProtocolLib} from "../libraries/ProtocolLib.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {PackedSSV, PackedETH, VERSION_ETH, VERSION_SSV, ETH_DEDUCTED_DIGITS, DEFAULT_EB_PER_VALIDATOR, MAX_EB_PER_VALIDATOR, BPS_DENOMINATOR} from "../libraries/SSVCoreTypes.sol";
import {SSVStorage, StorageData} from "../libraries/storage/SSVStorage.sol";
import {SSVStorageProtocol, StorageProtocol} from "../libraries/storage/SSVStorageProtocol.sol";
import {
    SSVStorageEB,
    StorageEB,
    ClusterEBSnapshot
} from "../libraries/storage/SSVStorageEB.sol";


import {MerkleProof} from "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";
import {SSVReentrancyGuard} from "../abstract/SSVReentrancyGuard.sol";
import {ISSVOperators} from "../interfaces/ISSVOperators.sol";

contract SSVClusters is ISSVClusters, SSVReentrancyGuard {
    using ClusterLib for Cluster;
    using OperatorLib for Operator;
    using ProtocolLib for StorageProtocol;

    /**
     * @inheritdoc ISSVClusters
     */
    function liquidate(address clusterOwner, uint64[] calldata operatorIds, Cluster memory cluster) external override nonReentrant {
        StorageData storage s = SSVStorage.load();

        (bytes32 hashedCluster, uint8 version) = cluster.validateHashedCluster(clusterOwner, operatorIds, s);
        ClusterLib.validateClusterVersion(version, VERSION_ETH);
        cluster.validateClusterIsNotLiquidated();

        StorageProtocol storage sp = SSVStorageProtocol.load();
        StorageEB storage seb = SSVStorageEB.load();

        (uint64 clusterIndex, uint64 burnRate) = OperatorLib.updateClusterOperators(
            operatorIds,
            false,
            cluster.validatorCount,
            s,
            sp
        );

        cluster.updateClusterData(hashedCluster, clusterIndex, sp.currentNetworkFeeIndex());

        if (
            clusterOwner != msg.sender &&
            !cluster.isLiquidatableWithEB(
                hashedCluster,
                burnRate,
                PackedETH.unwrap(sp.ethNetworkFee),
                sp.minimumBlocksBeforeLiquidation,
                sp.minimumLiquidationCollateral
            )
        ) {
            revert ClusterNotLiquidatable();
        }

        _executeLiquidation(clusterOwner, msg.sender, hashedCluster, operatorIds, cluster, s, sp, seb);
    }

    /**
     * @inheritdoc ISSVClusters
     */
    function liquidateSSV(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external override nonReentrant {
        StorageData storage s = SSVStorage.load();

        (bytes32 hashedCluster, uint8 version) = cluster.validateHashedCluster(clusterOwner, operatorIds, s);
        ClusterLib.validateClusterVersion(version, VERSION_SSV);
        cluster.validateClusterIsNotLiquidated();

        StorageProtocol storage sp = SSVStorageProtocol.load();

        (uint64 clusterIndex, uint64 burnRate) = OperatorLib.updateClusterOperatorsSSV(
            operatorIds,
            false,
            cluster.validatorCount,
            s,
            sp
        );

        cluster.updateBalanceSSV(clusterIndex, sp.currentNetworkFeeIndexSSV());

        uint256 balanceLiquidatable;

        if (
            clusterOwner != msg.sender &&
            !cluster.isLiquidatable(
                burnRate,
                PackedSSV.unwrap(sp.networkFee),
                sp.minimumBlocksBeforeLiquidationSSV,
                sp.minimumLiquidationCollateralSSV
            )
        ) {
            revert ClusterNotLiquidatable();
        }

        sp.updateDAOSSV(false, cluster.validatorCount);

        if (cluster.balance != 0) {
            balanceLiquidatable = cluster.balance;
            cluster.balance = 0;
        }
        cluster.index = 0;
        cluster.networkFeeIndex = 0;
        cluster.active = false;

        s.clusters[hashedCluster] = cluster.hashClusterData();

        if (balanceLiquidatable != 0) {
            CoreLib.transferTokenBalance(msg.sender, balanceLiquidatable);
        }

        emit ClusterLiquidated(clusterOwner, operatorIds, cluster);
    }

    /**
     * @inheritdoc ISSVClusters
     */
    function reactivate(
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external payable override nonReentrant {
        StorageData storage s = SSVStorage.load();

        (bytes32 hashedCluster, uint8 version) = cluster.validateHashedCluster(msg.sender, operatorIds, s);
        ClusterLib.validateClusterVersion(version, VERSION_ETH);
        if (cluster.active) revert ClusterAlreadyEnabled();

        StorageProtocol storage sp = SSVStorageProtocol.load();
        StorageEB storage seb = SSVStorageEB.load();

        uint64 vUnitsCluster = seb.clusterEB[hashedCluster].vUnits;
        uint64 baselineVUnits = uint64(cluster.validatorCount) * BPS_DENOMINATOR;
        uint64 effectiveVUnits = vUnitsCluster > 0 ? vUnitsCluster : baselineVUnits;
        uint64 clusterDeviation = vUnitsCluster > baselineVUnits ? vUnitsCluster - baselineVUnits : 0;

        (uint64 clusterIndex, uint64 burnRate) = OperatorLib.updateClusterOperatorsOnReactivation(
            operatorIds,
            cluster.validatorCount,
            clusterDeviation,
            s,
            sp,
            seb
        );

        cluster.balance += msg.value;
        cluster.active = true;
        cluster.index = clusterIndex;
        cluster.networkFeeIndex = sp.currentNetworkFeeIndex();

        if (
            cluster.isLiquidatableWithVUnits(
                effectiveVUnits,
                burnRate,
                PackedETH.unwrap(sp.ethNetworkFee),
                sp.minimumBlocksBeforeLiquidation,
                sp.minimumLiquidationCollateral
            )
        ) {
            revert InsufficientBalance();
        }

        sp.updateDAO(true, cluster.validatorCount);
        if (clusterDeviation > 0) {
            sp.daoTotalEthVUnits += clusterDeviation;
        }

        s.ethClusters[hashedCluster] = cluster.hashClusterData();

        emit ClusterReactivated(msg.sender, operatorIds, cluster);
    }

    /**
     * @inheritdoc ISSVClusters
     */
    function deposit(
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster
    ) external payable override {
        StorageData storage s = SSVStorage.load();

        (bytes32 hashedCluster, uint8 version) = cluster.validateHashedCluster(clusterOwner, operatorIds, s);
        ClusterLib.validateClusterVersion(version, VERSION_ETH);

        cluster.balance += msg.value;

        s.ethClusters[hashedCluster] = cluster.hashClusterData();

        emit ClusterDeposited(clusterOwner, operatorIds, msg.value, cluster);
    }

    /**
     * @inheritdoc ISSVClusters
     */
    function withdraw(uint64[] calldata operatorIds, uint256 amount, Cluster memory cluster) external override nonReentrant {
        StorageData storage s = SSVStorage.load();

        (bytes32 hashedCluster, uint8 version) = cluster.validateHashedCluster(msg.sender, operatorIds, s);
        ClusterLib.validateClusterVersion(version, VERSION_ETH);

        StorageProtocol storage sp = SSVStorageProtocol.load();

        uint64 burnRate;
        if (cluster.active) {
            uint64 clusterIndex;
            {
                uint256 operatorsLength = operatorIds.length;
                for (uint256 i; i < operatorsLength; ++i) {
                    Operator storage operator = s.operators[operatorIds[i]];
                    clusterIndex +=
                        operator.ethSnapshot.index +
                        (uint64(block.number) - operator.ethSnapshot.block) *
                        PackedETH.unwrap(operator.ethFee);
                    burnRate += PackedETH.unwrap(operator.ethFee);
                }
            }

            cluster.updateClusterData(hashedCluster, clusterIndex, sp.currentNetworkFeeIndex());
        }
        if (cluster.balance < amount) revert InsufficientBalance();

        cluster.balance -= amount;

        if (
            cluster.active &&
            cluster.validatorCount != 0 &&
            cluster.isLiquidatableWithEB(
                hashedCluster,
                burnRate,
                PackedETH.unwrap(sp.ethNetworkFee),
                sp.minimumBlocksBeforeLiquidation,
                sp.minimumLiquidationCollateral
            )
        ) {
            revert InsufficientBalance();
        }

        s.ethClusters[hashedCluster] = cluster.hashClusterData();

        CoreLib.transferBalance(msg.sender, amount);

        emit ClusterWithdrawn(msg.sender, operatorIds, amount, cluster);
    }

    /**
     * @inheritdoc ISSVClusters
     */
    function migrateClusterToETH(uint64[] calldata operatorIds, Cluster memory cluster) external payable override {
        StorageData storage s = SSVStorage.load();
        StorageProtocol storage sp = SSVStorageProtocol.load();

        (bytes32 hashedCluster, uint8 version) = cluster.validateHashedCluster(msg.sender, operatorIds, s);
        ClusterLib.validateClusterVersion(version, VERSION_SSV);
        bool isLiquidated = !cluster.active; // A liquidated SSV cluster already had its SSV counts removed

        // compute cluster data using ETH fields
        (uint64 clusterIndexSSV, uint64 clusterIndexETH, uint64 burnRateETH) = OperatorLib.updateClusterOperatorsMigration(
            operatorIds,
            cluster.validatorCount,
            s,
            sp,
            isLiquidated
        );

        cluster.updateBalanceSSV(clusterIndexSSV, sp.currentNetworkFeeIndexSSV());
        uint256 ssvClusterBalance = cluster.balance;

        cluster.balance = msg.value;
        cluster.active = true;
        cluster.index = clusterIndexETH;
        cluster.networkFeeIndex = sp.currentNetworkFeeIndex();

        if (!isLiquidated) {
            sp.updateDAOSSV(false, cluster.validatorCount);
        }
        sp.updateDAO(true, cluster.validatorCount);

        if (
            cluster.isLiquidatableWithEB(
                hashedCluster,
                burnRateETH,
                PackedETH.unwrap(sp.ethNetworkFee),
                sp.minimumBlocksBeforeLiquidation,
                sp.minimumLiquidationCollateral
            )
        ) {
            revert InsufficientBalance();
        }

        s.ethClusters[hashedCluster] = cluster.hashClusterData();
        delete s.clusters[hashedCluster];

        StorageEB storage seb = SSVStorageEB.load();
        ClusterEBSnapshot storage ebSnapshot = seb.clusterEB[hashedCluster];

        // Deviation-only model: baseline added via ethValidatorCount (in updateClusterOperatorsMigration above)
        // Only add deviation if cluster has explicit EB tracking
        uint64 vUnitsCluster = ebSnapshot.vUnits;
        if (vUnitsCluster > 0) {
            uint64 baseline = uint64(cluster.validatorCount) * BPS_DENOMINATOR;
            
            // DAO deviation accounting
            if (vUnitsCluster > baseline) {
                uint64 deviation = vUnitsCluster - baseline;
                sp.daoTotalEthVUnits += deviation;
                
                // Operator deviation accounting
                uint256 n = operatorIds.length;
                for (uint256 i; i < n; ++i) {
                    if (s.operators[operatorIds[i]].ethSnapshot.block == 0) continue;
                    seb.operatorEthVUnits[operatorIds[i]] += deviation;
                }
            }
            // Note: EB floor is 32 ETH, so vUnitsCluster >= baseline always
            // If vUnitsCluster == baseline, deviation is 0, nothing to add
        }
        // For implicit clusters (vUnitsCluster == 0): no deviation to add

        // For event emission, compute effective balance
        uint64 effectiveVUnits = vUnitsCluster > 0 
            ? vUnitsCluster 
            : uint64(cluster.validatorCount) * BPS_DENOMINATOR;
        uint32 effectiveBalance = ClusterLib.vUnitsToEB(effectiveVUnits);

        if (ssvClusterBalance != 0) {
            CoreLib.transferTokenBalance(msg.sender, ssvClusterBalance);
        }

        emit ClusterMigratedToETH(msg.sender, operatorIds, msg.value, ssvClusterBalance, effectiveBalance, cluster);
        if (isLiquidated) {
            emit ClusterReactivated(msg.sender, operatorIds, cluster);
        }
    }

    /**
     * @inheritdoc ISSVClusters
     */
    function updateClusterBalance(
        uint64 blockNum,
        address clusterOwner,
        uint64[] calldata operatorIds,
        Cluster memory cluster,
        uint32 effectiveBalance,
        bytes32[] calldata merkleProof
    ) external override nonReentrant {
        UpdateCtx memory ctx;
        StorageData storage s = SSVStorage.load();

        (ctx.clusterId, ctx.version) = cluster.validateHashedCluster(clusterOwner, operatorIds, s);
        ctx.clusterOwner = clusterOwner;
        ctx.blockNum = blockNum;
        ctx.effectiveBalance = effectiveBalance;
        ctx.merkleProof = merkleProof;

        _updateClusterBalanceInternal(operatorIds, cluster, ctx);
    }

    function _updateClusterBalanceInternal(
        uint64[] calldata operatorIds,
        Cluster memory cluster,
        UpdateCtx memory ctx
    ) internal {
        StorageData storage s = SSVStorage.load();
        StorageProtocol storage sp = SSVStorageProtocol.load();
        StorageEB storage seb = SSVStorageEB.load();

        bytes32 clusterId = ctx.clusterId;

        _verifyEBRoots(ctx, seb);
        _verifyEBUpdateFrequency(clusterId, seb);
        _verifyEBStaleness(ctx, clusterId, seb);
        _verifyMerkleProof(ctx, seb);
        _verifyEBLimits(ctx, cluster);

        uint64 newVUnits = ClusterLib.ebToVUnits(ctx.effectiveBalance);

        if (ctx.version == VERSION_ETH) {
            // ETH clusters: full accounting flow
            uint64 storedVUnits = seb.clusterEB[clusterId].vUnits;
            if (storedVUnits == 0) {
                storedVUnits = uint64(cluster.validatorCount) * BPS_DENOMINATOR;
            }

            uint64 burnRate;
            if (cluster.active) {
                burnRate = _applyClusterFeeUpdates(operatorIds, cluster, storedVUnits, s, sp);
            }

            // Apply new vUnits BEFORE liquidation check so auto-liquidation
            if (cluster.active && newVUnits != storedVUnits) {
                _updateOperatorVUnits(operatorIds, seb, storedVUnits, newVUnits);
                sp.updateDAOEthVUnits(storedVUnits, newVUnits);
            }
            _updateEBSnapshot(seb, clusterId, ctx.blockNum, newVUnits);

            bool liquidated = _liquidateAfterEBUpdateIfNeeded(cluster, clusterId, ctx.clusterOwner, operatorIds, burnRate, s, sp, seb);

            if (!liquidated && cluster.active) {
                s.ethClusters[clusterId] = cluster.hashClusterData();
            }
        } else {
            // SSV clusters: only update EB snapshot (preparing for future migration)
            _updateEBSnapshot(seb, clusterId, ctx.blockNum, newVUnits);
        }
        
        emit ClusterBalanceUpdated(ctx.clusterOwner, operatorIds, ctx.blockNum, ctx.effectiveBalance, cluster);
    }

    function _verifyEBRoots(UpdateCtx memory ctx, StorageEB storage seb) internal view {
        if (seb.ebRoots[ctx.blockNum] == bytes32(0)) {
            revert RootNotFound();
        }
    }

    function _verifyEBUpdateFrequency(bytes32 clusterId, StorageEB storage seb) internal view {
        ClusterEBSnapshot storage ebSnapshot = seb.clusterEB[clusterId];
        if (
            ebSnapshot.lastUpdateBlock != 0 && block.number < ebSnapshot.lastUpdateBlock + seb.minBlocksBetweenUpdates
        ) {
            revert UpdateTooFrequent();
        }
    }

    function _verifyEBStaleness(UpdateCtx memory ctx, bytes32 clusterId, StorageEB storage seb) internal view {
        if (ctx.blockNum != seb.latestCommittedBlock) {
            revert MustUseLatestRoot();
        }

        ClusterEBSnapshot storage ebSnapshot = seb.clusterEB[clusterId];
        if (ebSnapshot.lastRootBlockNum != 0 && ctx.blockNum <= ebSnapshot.lastRootBlockNum) {
            revert StaleUpdate();
        }
    }

    function _verifyMerkleProof(UpdateCtx memory ctx, StorageEB storage seb) internal view {
        bytes32 root = seb.ebRoots[ctx.blockNum];

        if (!MerkleProof.verify(ctx.merkleProof, root, keccak256(abi.encodePacked(keccak256(abi.encode(ctx.clusterId, ctx.effectiveBalance)))))) {
            revert InvalidProof();
        }
    }

    function _verifyEBLimits(UpdateCtx memory ctx, Cluster memory cluster) internal pure {
        if (ctx.effectiveBalance > uint256(cluster.validatorCount) * (MAX_EB_PER_VALIDATOR / 1 ether)) {
            revert EBExceedsMaximum();
        } else if (ctx.effectiveBalance < uint256(cluster.validatorCount) * (DEFAULT_EB_PER_VALIDATOR / 1 ether)) {
            revert EBBelowMinimum();
        }
    }

    function _applyClusterFeeUpdates(
        uint64[] calldata operatorIds,
        Cluster memory cluster,
        uint64 oldVUnits,
        StorageData storage s,
        StorageProtocol storage sp
    ) internal returns (uint64 burnRate) {
        // ETH path: use ethSnapshot, ethFee, ethNetworkFeeIndex
        (uint64 clusterIndex, uint64 cumulativeFee) = OperatorLib.updateClusterOperators(operatorIds, false, 0, s, sp);
        uint64 currentNetworkFeeIndex = sp.currentNetworkFeeIndex();

        // Calculate fee deltas BEFORE updating indexes
        uint128 units = oldVUnits;
        uint128 idxNet = currentNetworkFeeIndex - cluster.networkFeeIndex;
        uint128 idxOp = clusterIndex - cluster.index;

        uint128 networkFeeUnits = (idxNet * units) / BPS_DENOMINATOR;
        uint128 operatorFeeUnits = (idxOp * units) / BPS_DENOMINATOR;
        uint256 totalFees = (uint256(networkFeeUnits) + uint256(operatorFeeUnits)) * ETH_DEDUCTED_DIGITS;

        // Update indexes
        cluster.index = clusterIndex;
        cluster.networkFeeIndex = currentNetworkFeeIndex;

        if (cluster.balance >= totalFees) {
            cluster.balance -= totalFees;
        } else {
            cluster.balance = 0;
        }

        return cumulativeFee;
    }

    function _updateOperatorVUnits(
        uint64[] calldata operatorIds,
        StorageEB storage seb,
        uint64 storedVUnits,
        uint64 newVUnits
    ) internal {
        // Caller must ensure newVUnits != storedVUnits
        bool deltaPositive = newVUnits > storedVUnits;
        uint64 deltaAbs = deltaPositive ? newVUnits - storedVUnits : storedVUnits - newVUnits;

        StorageData storage s = SSVStorage.load();
        uint256 operatorsLength = operatorIds.length;
        for (uint256 i; i < operatorsLength; ++i) {
            uint64 operatorId = operatorIds[i];
            if (s.operators[operatorId].ethSnapshot.block == 0) continue;
            if (deltaPositive) seb.operatorEthVUnits[operatorId] += deltaAbs;
            else seb.operatorEthVUnits[operatorId] -= deltaAbs;
        }
    }

    function _updateEBSnapshot(StorageEB storage seb, bytes32 clusterId, uint64 blockNum, uint64 newVUnits) internal {
        ClusterEBSnapshot storage ebSnapshot = seb.clusterEB[clusterId];
        ebSnapshot.vUnits = newVUnits;
        ebSnapshot.lastRootBlockNum = blockNum;
        ebSnapshot.lastUpdateBlock = uint64(block.number);
    }

    function _liquidateAfterEBUpdateIfNeeded(
        Cluster memory cluster,
        bytes32 clusterId,
        address clusterOwner,
        uint64[] calldata operatorIds,
        uint64 burnRate,
        StorageData storage s,
        StorageProtocol storage sp,
        StorageEB storage seb
    ) internal returns (bool liquidated) {
        if (!cluster.active || cluster.validatorCount == 0) return false;

        if (cluster.isLiquidatableWithEB(
            clusterId,
            burnRate,
            PackedETH.unwrap(sp.ethNetworkFee),
            sp.minimumBlocksBeforeLiquidation,
            sp.minimumLiquidationCollateral
        )) {

            for (uint256 i; i < operatorIds.length; ++i) {
                ISSVOperators.Operator storage op = s.operators[operatorIds[i]];
                if (op.ethSnapshot.block != 0) {
                    op.ethValidatorCount -= cluster.validatorCount;
                }
            }

            _executeLiquidation(clusterOwner, msg.sender, clusterId, operatorIds, cluster, s, sp, seb);
            return true;
        }
        return false;
    }

    function _executeLiquidation(
        address clusterOwner,
        address liquidator,
        bytes32 clusterId,
        uint64[] calldata operatorIds,
        Cluster memory cluster,
        StorageData storage s,
        StorageProtocol storage sp,
        StorageEB storage seb
    ) internal {
        sp.updateDAO(false, cluster.validatorCount);

        ClusterEBSnapshot storage ebSnapshot = seb.clusterEB[clusterId];
        uint64 vUnitsCluster = ebSnapshot.vUnits;
        
        // Deviation-only model: only subtract deviation from operatorEthVUnits
        // Baseline is removed via ethValidatorCount decrement (in updateClusterOperators above)
        if (vUnitsCluster > 0) {
            uint64 baselineVUnits = uint64(cluster.validatorCount) * BPS_DENOMINATOR;

            // DAO deviation accounting
            if (vUnitsCluster != baselineVUnits) {
                bool moreThanBaseline = vUnitsCluster > baselineVUnits;
                uint64 deviation = moreThanBaseline ? vUnitsCluster - baselineVUnits : baselineVUnits - vUnitsCluster;

                if (deviation != 0) {
                    if (moreThanBaseline) sp.daoTotalEthVUnits -= deviation;
                    else sp.daoTotalEthVUnits += deviation;
                }
                
                // Operator deviation accounting: only subtract deviation, not baseline
                // Note: EB floor is 32 ETH, so vUnitsCluster >= baselineVUnits always
                // But we handle both cases for safety
                uint256 n = operatorIds.length;
                for (uint256 i; i < n; ++i) {
                    if (s.operators[operatorIds[i]].ethSnapshot.block == 0) continue;
                    if (moreThanBaseline) {
                        seb.operatorEthVUnits[operatorIds[i]] -= deviation;
                    } else {
                        seb.operatorEthVUnits[operatorIds[i]] += deviation;
                    }
                }
            }
            // If vUnitsCluster == baselineVUnits, deviation is 0, nothing to update
            
        }
        // For implicit clusters (vUnitsCluster == 0): no deviation to remove

        uint256 balanceLiquidatable = cluster.balance;
        cluster.balance = 0;
        cluster.active = false;
        cluster.index = 0;
        cluster.networkFeeIndex = 0;

        s.ethClusters[clusterId] = cluster.hashClusterData();

        if (balanceLiquidatable > 0) {
            CoreLib.transferBalance(liquidator, balanceLiquidatable);
        }

        emit ClusterLiquidated(clusterOwner, operatorIds, cluster);
    }
}


## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

