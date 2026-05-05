
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;
pragma abicoder v2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-strict-inequalities

import { SafeMath } from "@openzeppelin/contracts/math/SafeMath.sol";
import { StakingV4Storage } from "./StakingStorage.sol";
import { IStakingExtension } from "@graphprotocol/interfaces/contracts/contracts/staking/IStakingExtension.sol";
import { TokenUtils } from "../utils/TokenUtils.sol";
import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import { GraphUpgradeable } from "../upgrades/GraphUpgradeable.sol";
import { IStakes } from "@graphprotocol/interfaces/contracts/contracts/staking/libs/IStakes.sol";
import { Stakes } from "./libs/Stakes.sol";
import { IStakingData } from "@graphprotocol/interfaces/contracts/contracts/staking/IStakingData.sol";
import { MathUtils } from "./libs/MathUtils.sol";

/**
 * @title StakingExtension contract
 * @author Edge & Node
 * @notice This contract provides the logic to manage delegations and other Staking
 * extension features (e.g. storage getters). It is meant to be called through delegatecall from the
 * Staking contract, and is only kept separate to keep the Staking contract size
 * within limits.
 */
contract StakingExtension is StakingV4Storage, GraphUpgradeable, IStakingExtension {
    using SafeMath for uint256;
    using Stakes for IStakes.Indexer;

    /// @dev 100% in parts per million
    uint32 private constant MAX_PPM = 1000000;
    /// @dev Minimum amount of tokens that can be delegated
    uint256 private constant MINIMUM_DELEGATION = 1e18;

    /**
     * @dev Check if the caller is the slasher.
     */
    modifier onlySlasher() {
        require(__slashers[msg.sender] == true, "!slasher");
        _;
    }

    /**
     * @notice Initialize the StakingExtension contract
     * @dev This function is meant to be delegatecalled from the Staking contract's
     * initialize() function, so it uses the same access control check to ensure it is
     * being called by the Staking implementation as part of the proxy upgrade process.
     * @param _delegationUnbondingPeriod Delegation unbonding period in blocks
     * @param _cooldownBlocks Deprecated parameter (no longer used)
     * @param _delegationRatio Delegation capacity multiplier (e.g. 10 means 10x the indexer stake)
     * @param _delegationTaxPercentage Percentage of delegated tokens to burn as delegation tax, expressed in parts per million
     */
    function initialize(
        uint32 _delegationUnbondingPeriod,
        // solhint-disable-next-line no-unused-vars
        uint32 _cooldownBlocks, // deprecated
        uint32 _delegationRatio,
        uint32 _delegationTaxPercentage
    ) external onlyImpl {
        _setDelegationUnbondingPeriod(_delegationUnbondingPeriod);
        _setDelegationRatio(_delegationRatio);
        _setDelegationTaxPercentage(_delegationTaxPercentage);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function setDelegationTaxPercentage(uint32 _percentage) external override onlyGovernor {
        _setDelegationTaxPercentage(_percentage);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function setDelegationRatio(uint32 _delegationRatio) external override onlyGovernor {
        _setDelegationRatio(_delegationRatio);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function setDelegationUnbondingPeriod(uint32 _delegationUnbondingPeriod) external override onlyGovernor {
        _setDelegationUnbondingPeriod(_delegationUnbondingPeriod);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function setSlasher(address _slasher, bool _allowed) external override onlyGovernor {
        require(_slasher != address(0), "!slasher");
        __slashers[_slasher] = _allowed;
        emit SlasherUpdate(msg.sender, _slasher, _allowed);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function delegate(address _indexer, uint256 _tokens) external override notPartialPaused returns (uint256) {
        address delegator = msg.sender;

        // Transfer tokens to delegate to this contract
        TokenUtils.pullTokens(graphToken(), delegator, _tokens);

        // Update state
        return _delegate(delegator, _indexer, _tokens);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function undelegate(address _indexer, uint256 _shares) external override notPartialPaused returns (uint256) {
        return _undelegate(msg.sender, _indexer, _shares);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function withdrawDelegated(address _indexer, address _newIndexer) external override notPaused returns (uint256) {
        return _withdrawDelegated(msg.sender, _indexer, _newIndexer);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function slash(
        address _indexer,
        uint256 _tokens,
        uint256 _reward,
        address _beneficiary
    ) external override onlySlasher notPartialPaused {
        IStakes.Indexer storage indexerStake = __stakes[_indexer];

        // Only able to slash a non-zero number of tokens
        require(_tokens > 0, "!tokens");

        // Rewards comes from tokens slashed balance
        require(_tokens >= _reward, "rewards>slash");

        // Cannot slash stake of an indexer without any or enough stake
        require(indexerStake.tokensStaked > 0, "!stake");
        require(_tokens <= indexerStake.tokensStaked, "slash>stake");

        // Validate beneficiary of slashed tokens
        require(_beneficiary != address(0), "!beneficiary");

        // Slashing more tokens than freely available (over allocation condition)
        // Unlock locked tokens to avoid the indexer to withdraw them
        if (_tokens > indexerStake.tokensAvailable() && indexerStake.tokensLocked > 0) {
            uint256 tokensOverAllocated = _tokens.sub(indexerStake.tokensAvailable());
            uint256 tokensToUnlock = MathUtils.min(tokensOverAllocated, indexerStake.tokensLocked);
            indexerStake.unlockTokens(tokensToUnlock);
        }

        // Remove tokens to slash from the stake
        indexerStake.release(_tokens);

        // -- Interactions --

        IGraphToken graphToken = graphToken();

        // Set apart the reward for the beneficiary and burn remaining slashed stake
        TokenUtils.burnTokens(graphToken, _tokens.sub(_reward));

        // Give the beneficiary a reward for slashing
        TokenUtils.pushTokens(graphToken, _beneficiary, _reward);

        emit StakeSlashed(_indexer, _tokens, _reward, _beneficiary);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function getDelegation(address _indexer, address _delegator) external view override returns (Delegation memory) {
        return __delegationPools[_indexer].delegators[_delegator];
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function delegationRatio() external view override returns (uint32) {
        return __delegationRatio;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function delegationUnbondingPeriod() external view override returns (uint32) {
        return __delegationUnbondingPeriod;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function delegationTaxPercentage() external view override returns (uint32) {
        return __delegationTaxPercentage;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function delegationPools(address _indexer) external view override returns (DelegationPoolReturn memory) {
        DelegationPool storage pool = __delegationPools[_indexer];
        return
            DelegationPoolReturn(
                0, // Blocks to wait before updating parameters (deprecated)
                pool.indexingRewardCut, // in PPM
                pool.queryFeeCut, // in PPM
                pool.updatedAtBlock, // Block when the pool was last updated
                pool.tokens, // Total tokens as pool reserves
                pool.shares // Total shares minted in the pool
            );
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function rewardsDestination(address _indexer) external view override returns (address) {
        return __rewardsDestination[_indexer];
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function operatorAuth(address _indexer, address _maybeOperator) external view override returns (bool) {
        return __operatorAuth[_indexer][_maybeOperator];
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function subgraphAllocations(bytes32 _subgraphDeploymentId) external view override returns (uint256) {
        return __subgraphAllocations[_subgraphDeploymentId];
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function slashers(address _maybeSlasher) external view override returns (bool) {
        return __slashers[_maybeSlasher];
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function minimumIndexerStake() external view override returns (uint256) {
        return __minimumIndexerStake;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function thawingPeriod() external view override returns (uint32) {
        return __thawingPeriod;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function curationPercentage() external view override returns (uint32) {
        return __curationPercentage;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function protocolPercentage() external view override returns (uint32) {
        return __protocolPercentage;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function maxAllocationEpochs() external view override returns (uint32) {
        return __maxAllocationEpochs;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function alphaNumerator() external view override returns (uint32) {
        return __alphaNumerator;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function alphaDenominator() external view override returns (uint32) {
        return __alphaDenominator;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function lambdaNumerator() external view override returns (uint32) {
        return __lambdaNumerator;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function lambdaDenominator() external view override returns (uint32) {
        return __lambdaDenominator;
    }

    /**
     * @notice Getter for stakes[_indexer]:
     * gets the stake information for an indexer as an IStakes.Indexer struct.
     * @param _indexer Indexer address for which to query the stake information
     * @return Stake information for the specified indexer, as an IStakes.Indexer struct
     * @inheritdoc IStakingExtension
     */
    function stakes(address _indexer) external view override returns (IStakes.Indexer memory) {
        return __stakes[_indexer];
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function allocations(address _allocationID) external view override returns (IStakingData.Allocation memory) {
        return __allocations[_allocationID];
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function isDelegator(address _indexer, address _delegator) public view override returns (bool) {
        return __delegationPools[_indexer].delegators[_delegator].shares > 0;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function getWithdraweableDelegatedTokens(Delegation memory _delegation) public view override returns (uint256) {
        // There must be locked tokens and period passed
        uint256 currentEpoch = epochManager().currentEpoch();
        if (_delegation.tokensLockedUntil > 0 && currentEpoch >= _delegation.tokensLockedUntil) {
            return _delegation.tokensLocked;
        }
        return 0;
    }

    /**
     * @notice Internal: Set a delegation tax percentage to burn when delegated funds are deposited.
     * @param _percentage Percentage of delegated tokens to burn as delegation tax
     */
    function _setDelegationTaxPercentage(uint32 _percentage) private {
        // Must be within 0% to 100% (inclusive)
        require(_percentage <= MAX_PPM, ">percentage");
        __delegationTaxPercentage = _percentage;
        emit ParameterUpdated("delegationTaxPercentage");
    }

    /**
     * @notice Internal: Set the delegation ratio.
     * If set to 10 it means the indexer can use up to 10x the indexer staked amount
     * from their delegated tokens
     * @param _delegationRatio Delegation capacity multiplier
     */
    function _setDelegationRatio(uint32 _delegationRatio) private {
        __delegationRatio = _delegationRatio;
        emit ParameterUpdated("delegationRatio");
    }

    /**
     * @notice Internal: Set the period for undelegation of stake from indexer.
     * @param _delegationUnbondingPeriod Period in epochs to wait for token withdrawals after undelegating
     */
    function _setDelegationUnbondingPeriod(uint32 _delegationUnbondingPeriod) private {
        require(_delegationUnbondingPeriod > 0, "!delegationUnbondingPeriod");
        __delegationUnbondingPeriod = _delegationUnbondingPeriod;
        emit ParameterUpdated("delegationUnbondingPeriod");
    }

    /**
     * @notice Delegate tokens to an indexer.
     * @param _delegator Address of the delegator
     * @param _indexer Address of the indexer to delegate tokens to
     * @param _tokens Amount of tokens to delegate
     * @return Amount of shares issued of the delegation pool
     */
    function _delegate(address _delegator, address _indexer, uint256 _tokens) private returns (uint256) {
        // Only allow delegations over a minimum, to prevent rounding attacks
        require(_tokens >= MINIMUM_DELEGATION, "!minimum-delegation");
        // Only delegate to non-empty address
        require(_indexer != address(0), "!indexer");
        // Only delegate to staked indexer
        require(__stakes[_indexer].tokensStaked > 0, "!stake");

        // Get the delegation pool of the indexer
        DelegationPool storage pool = __delegationPools[_indexer];
        Delegation storage delegation = pool.delegators[_delegator];

        // Collect delegation tax
        uint256 delegationTax = _collectTax(graphToken(), _tokens, __delegationTaxPercentage);
        uint256 delegatedTokens = _tokens.sub(delegationTax);

        // Calculate shares to issue
        uint256 shares = (pool.tokens == 0) ? delegatedTokens : delegatedTokens.mul(pool.shares).div(pool.tokens);
        require(shares > 0, "!shares");

        // Update the delegation pool
        pool.tokens = pool.tokens.add(delegatedTokens);
        pool.shares = pool.shares.add(shares);

        // Update the individual delegation
        delegation.shares = delegation.shares.add(shares);

        emit StakeDelegated(_indexer, _delegator, delegatedTokens, shares);

        return shares;
    }

    /**
     * @notice Undelegate tokens from an indexer.
     * @param _delegator Address of the delegator
     * @param _indexer Address of the indexer where tokens had been delegated
     * @param _shares Amount of shares to return and undelegate tokens
     * @return Amount of tokens returned for the shares of the delegation pool
     */
    function _undelegate(address _delegator, address _indexer, uint256 _shares) private returns (uint256) {
        // Can only undelegate a non-zero amount of shares
        require(_shares > 0, "!shares");

        // Get the delegation pool of the indexer
        DelegationPool storage pool = __delegationPools[_indexer];
        Delegation storage delegation = pool.delegators[_delegator];

        // Delegator need to have enough shares in the pool to undelegate
        require(delegation.shares >= _shares, "!shares-avail");

        // Withdraw tokens if available
        if (getWithdraweableDelegatedTokens(delegation) > 0) {
            _withdrawDelegated(_delegator, _indexer, address(0));
        }

        uint256 poolTokens = pool.tokens;
        uint256 poolShares = pool.shares;

        // Calculate tokens to get in exchange for the shares
        uint256 tokens = _shares.mul(poolTokens).div(poolShares);

        // Update the delegation pool
        poolTokens = poolTokens.sub(tokens);
        poolShares = poolShares.sub(_shares);
        pool.tokens = poolTokens;
        pool.shares = poolShares;

        // Update the delegation
        delegation.shares = delegation.shares.sub(_shares);
        // Enforce more than the minimum delegation is left,
        // to prevent rounding attacks
        if (delegation.shares > 0) {
            uint256 remainingDelegation = delegation.shares.mul(poolTokens).div(poolShares);
            require(remainingDelegation >= MINIMUM_DELEGATION, "!minimum-delegation");
        }
        delegation.tokensLocked = delegation.tokensLocked.add(tokens);
        delegation.tokensLockedUntil = epochManager().currentEpoch().add(__delegationUnbondingPeriod);

        emit StakeDelegatedLocked(_indexer, _delegator, tokens, _shares, delegation.tokensLockedUntil);

        return tokens;
    }

    /**
     * @notice Withdraw delegated tokens once the unbonding period has passed.
     * @param _delegator Delegator that is withdrawing tokens
     * @param _indexer Withdraw available tokens delegated to indexer
     * @param _delegateToIndexer Re-delegate to indexer address if non-zero, withdraw if zero address
     * @return Amount of tokens withdrawn or re-delegated
     */
    function _withdrawDelegated(
        address _delegator,
        address _indexer,
        address _delegateToIndexer
    ) private returns (uint256) {
        // Get the delegation pool of the indexer
        DelegationPool storage pool = __delegationPools[_indexer];
        Delegation storage delegation = pool.delegators[_delegator];

        // Validation
        uint256 tokensToWithdraw = getWithdraweableDelegatedTokens(delegation);
        require(tokensToWithdraw > 0, "!tokens");

        // Reset lock
        delegation.tokensLocked = 0;
        delegation.tokensLockedUntil = 0;

        emit StakeDelegatedWithdrawn(_indexer, _delegator, tokensToWithdraw);

        // -- Interactions --

        if (_delegateToIndexer != address(0)) {
            // Re-delegate tokens to a new indexer
            _delegate(_delegator, _delegateToIndexer, tokensToWithdraw);
        } else {
            // Return tokens to the delegator
            TokenUtils.pushTokens(graphToken(), _delegator, tokensToWithdraw);
        }

        return tokensToWithdraw;
    }

    /**
     * @notice Collect tax to burn for an amount of tokens.
     * @param _graphToken Token to burn
     * @param _tokens Total tokens received used to calculate the amount of tax to collect
     * @param _percentage Percentage of tokens to burn as tax
     * @return Amount of tax charged
     */
    function _collectTax(IGraphToken _graphToken, uint256 _tokens, uint256 _percentage) private returns (uint256) {
        uint256 tax = uint256(_percentage).mul(_tokens).div(MAX_PPM);
        TokenUtils.burnTokens(_graphToken, tax); // Burn tax if any
        return tax;
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity 0.8.27 || 0.8.33;

import { GraphDirectory } from "../../utilities/GraphDirectory.sol";

/* solhint-disable var-name-mixedcase */

/**
 * @title Graph Managed contract
 * @author Edge & Node
 * @notice The Managed contract provides an interface to interact with the Controller
 * @dev For Graph Horizon this contract is mostly a shell that uses {GraphDirectory}, however since the {HorizonStaking}
 * contract uses it we need to preserve the storage layout.
 * Inspired by Livepeer: https://github.com/livepeer/protocol/blob/streamflow/contracts/Controller.sol
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
abstract contract Managed is GraphDirectory {
    // -- State --

    // forge-lint: disable-next-item(mixed-case-variable)
    /// @notice Controller that manages this contract
    address private __DEPRECATED_controller;

    // forge-lint: disable-next-item(mixed-case-variable)
    /// @dev Cache for the addresses of the contracts retrieved from the controller
    mapping(bytes32 contractName => address contractAddress) private __DEPRECATED_addressCache;

    // forge-lint: disable-next-item(mixed-case-variable)
    /// @dev Gap for future storage variables
    uint256[10] private __gap;

    /**
     * @notice Thrown when a protected function is called and the contract is paused.
     */
    error ManagedIsPaused();

    /**
     * @notice Thrown when a the caller is not the expected controller address.
     */
    error ManagedOnlyController();

    /**
     * @notice Thrown when a the caller is not the governor.
     */
    error ManagedOnlyGovernor();

    // forge-lint: disable-next-item(unwrapped-modifier-logic)
    /**
     * @dev Revert if the controller is paused
     */
    modifier notPaused() {
        require(!_graphController().paused(), ManagedIsPaused());
        _;
    }

    // forge-lint: disable-next-item(unwrapped-modifier-logic)
    /**
     * @dev Revert if the caller is not the governor
     */
    modifier onlyGovernor() {
        require(msg.sender == _graphController().getGovernor(), ManagedOnlyGovernor());
        _;
    }

    /**
     * @notice Initialize the contract
     * @param controller_ The address of the Graph controller contract
     */
    constructor(address controller_) GraphDirectory(controller_) {}
}

// SPDX-License-Identifier: GPL-2.0-or-later
// solhint-disable one-contract-per-file

pragma solidity ^0.7.6;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable one-contract-per-file, max-states-count
// solhint-disable named-parameters-mapping

import { Managed } from "../governance/Managed.sol";

import { IStakingData } from "@graphprotocol/interfaces/contracts/contracts/staking/IStakingData.sol";
import { IStakes } from "@graphprotocol/interfaces/contracts/contracts/staking/libs/IStakes.sol";

/**
 * @title StakingV1Storage
 * @author Edge & Node
 * @notice This contract holds all the storage variables for the Staking contract, version 1
 * @dev Note that we use a double underscore prefix for variable names; this prefix identifies
 * variables that used to be public but are now internal, getters can be found on StakingExtension.sol.
 */
contract StakingV1Storage is Managed {
    // -- Staking --

    /// @dev Minimum amount of tokens an indexer needs to stake
    uint256 internal __minimumIndexerStake;

    /// @dev Time in blocks to unstake
    uint32 internal __thawingPeriod; // in blocks

    /// @dev Percentage of fees going to curators
    /// Parts per million. (Allows for 4 decimal points, 999,999 = 99.9999%)
    uint32 internal __curationPercentage;

    /// @dev Percentage of fees burned as protocol fee
    /// Parts per million. (Allows for 4 decimal points, 999,999 = 99.9999%)
    uint32 internal __protocolPercentage;

    /// @dev Period for allocation to be finalized
    uint32 private __DEPRECATED_channelDisputeEpochs; // solhint-disable-line var-name-mixedcase

    /// @dev Maximum allocation time
    uint32 internal __maxAllocationEpochs;

    /// @dev Rebate alpha numerator
    // Originally used for Cobb-Douglas rebates, now used for exponential rebates
    uint32 internal __alphaNumerator;

    /// @dev Rebate alpha denominator
    // Originally used for Cobb-Douglas rebates, now used for exponential rebates
    uint32 internal __alphaDenominator;

    /// @dev Indexer stakes : indexer => Stake
    mapping(address => IStakes.Indexer) internal __stakes;

    /// @dev Allocations : allocationID => Allocation
    mapping(address => IStakingData.Allocation) internal __allocations;

    /// @dev Subgraph Allocations: subgraphDeploymentID => tokens
    mapping(bytes32 => uint256) internal __subgraphAllocations;

    /// @dev Deprecated rebate pools mapping (no longer used)
    mapping(uint256 => uint256) private __DEPRECATED_rebates; // solhint-disable-line var-name-mixedcase

    // -- Slashing --

    /// @dev List of addresses allowed to slash stakes
    mapping(address => bool) internal __slashers;

    // -- Delegation --

    /// @dev Set the delegation capacity multiplier defined by the delegation ratio
    /// If delegation ratio is 100, and an Indexer has staked 5 GRT,
    /// then they can use up to 500 GRT from the delegated stake
    uint32 internal __delegationRatio;

    /// @dev Time in blocks an indexer needs to wait to change delegation parameters (deprecated)
    uint32 internal __DEPRECATED_delegationParametersCooldown; // solhint-disable-line var-name-mixedcase

    /// @dev Time in epochs a delegator needs to wait to withdraw delegated stake
    uint32 internal __delegationUnbondingPeriod; // in epochs

    /// @dev Percentage of tokens to tax a delegation deposit
    /// Parts per million. (Allows for 4 decimal points, 999,999 = 99.9999%)
    uint32 internal __delegationTaxPercentage;

    /// @dev Delegation pools : indexer => DelegationPool
    mapping(address => IStakingData.DelegationPool) internal __delegationPools;

    // -- Operators --

    /// @dev Operator auth : indexer => operator => is authorized
    mapping(address => mapping(address => bool)) internal __operatorAuth;

    // -- Asset Holders --

    /// @dev DEPRECATED: Allowed AssetHolders: assetHolder => is allowed
    mapping(address => bool) private __DEPRECATED_assetHolders; // solhint-disable-line var-name-mixedcase
}

/**
 * @title StakingV2Storage
 * @author Edge & Node
 * @notice This contract holds all the storage variables for the Staking contract, version 2
 * @dev Note that we use a double underscore prefix for variable names; this prefix identifies
 * variables that used to be public but are now internal, getters can be found on StakingExtension.sol.
 */
contract StakingV2Storage is StakingV1Storage {
    /// @dev Destination of accrued rewards : beneficiary => rewards destination
    mapping(address => address) internal __rewardsDestination;
}

/**
 * @title StakingV3Storage
 * @author Edge & Node
 * @notice This contract holds all the storage variables for the base Staking contract, version 3.
 */
contract StakingV3Storage is StakingV2Storage {
    /// @dev Address of the counterpart Staking contract on L1/L2
    address internal counterpartStakingAddress;
    /// @dev Address of the StakingExtension implementation
    address internal extensionImpl;
}

/**
 * @title StakingV4Storage
 * @author Edge & Node
 * @notice This contract holds all the storage variables for the base Staking contract, version 4.
 * @dev Note that it includes a storage gap - if adding future versions, make sure to move the gap
 * to the new version and reduce the size of the gap accordingly.
 */
contract StakingV4Storage is StakingV3Storage {
    /// @dev Numerator for the lambda parameter in exponential rebate calculations
    uint32 internal __lambdaNumerator;
    /// @dev Denominator for the lambda parameter in exponential rebate calculations
    uint32 internal __lambdaDenominator;

    /// @dev Gap to allow adding variables in future upgrades (since L1Staking and L2Staking can have their own storage as well)
    uint256[50] private __gap;
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity 0.8.27 || 0.8.33;

import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import { IHorizonStaking } from "@graphprotocol/interfaces/contracts/horizon/IHorizonStaking.sol";
import { IGraphPayments } from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import { IPaymentsEscrow } from "@graphprotocol/interfaces/contracts/horizon/IPaymentsEscrow.sol";

import { IController } from "@graphprotocol/interfaces/contracts/contracts/governance/IController.sol";
import { IEpochManager } from "@graphprotocol/interfaces/contracts/contracts/epochs/IEpochManager.sol";
import { IRewardsManager } from "@graphprotocol/interfaces/contracts/contracts/rewards/IRewardsManager.sol";
import { ITokenGateway } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/ITokenGateway.sol";
import { IGraphProxyAdmin } from "@graphprotocol/interfaces/contracts/contracts/upgrades/IGraphProxyAdmin.sol";

import { ICuration } from "@graphprotocol/interfaces/contracts/contracts/curation/ICuration.sol";

/**
 * @title GraphDirectory contract
 * @author Edge & Node
 * @notice This contract is meant to be inherited by other contracts that
 * need to keep track of the addresses in Graph Horizon contracts.
 * It fetches the addresses from the Controller supplied during construction,
 * and uses immutable variables to minimize gas costs.
 */
abstract contract GraphDirectory {
    // -- Graph Horizon contracts --

    /// @notice The Graph Token contract address
    IGraphToken private immutable GRAPH_TOKEN;

    /// @notice The Horizon Staking contract address
    IHorizonStaking private immutable GRAPH_STAKING;

    /// @notice The Graph Payments contract address
    IGraphPayments private immutable GRAPH_PAYMENTS;

    /// @notice The Payments Escrow contract address
    IPaymentsEscrow private immutable GRAPH_PAYMENTS_ESCROW;

    // -- Graph periphery contracts --

    /// @notice The Graph Controller contract address
    IController private immutable GRAPH_CONTROLLER;

    /// @notice The Epoch Manager contract address
    IEpochManager private immutable GRAPH_EPOCH_MANAGER;

    /// @notice The Rewards Manager contract address
    IRewardsManager private immutable GRAPH_REWARDS_MANAGER;

    /// @notice The Token Gateway contract address
    ITokenGateway private immutable GRAPH_TOKEN_GATEWAY;

    /// @notice The Graph Proxy Admin contract address
    IGraphProxyAdmin private immutable GRAPH_PROXY_ADMIN;

    // -- Legacy Graph contracts --
    // These are required for backwards compatibility on HorizonStakingExtension
    // TRANSITION PERIOD: remove these once HorizonStakingExtension is removed

    /// @notice The Curation contract address
    ICuration private immutable GRAPH_CURATION;

    /**
     * @notice Emitted when the GraphDirectory is initialized
     * @param graphToken The Graph Token contract address
     * @param graphStaking The Horizon Staking contract address
     * @param graphPayments The Graph Payments contract address
     * @param graphEscrow The Payments Escrow contract address
     * @param graphController The Graph Controller contract address
     * @param graphEpochManager The Epoch Manager contract address
     * @param graphRewardsManager The Rewards Manager contract address
     * @param graphTokenGateway The Token Gateway contract address
     * @param graphProxyAdmin The Graph Proxy Admin contract address
     * @param graphCuration The Curation contract address
     */
    event GraphDirectoryInitialized(
        address indexed graphToken,
        address indexed graphStaking,
        address graphPayments,
        address graphEscrow,
        address indexed graphController,
        address graphEpochManager,
        address graphRewardsManager,
        address graphTokenGateway,
        address graphProxyAdmin,
        address graphCuration
    );

    /**
     * @notice Thrown when either the controller is the zero address or a contract address is not found
     * on the controller
     * @param contractName The name of the contract that was not found, or the controller
     */
    error GraphDirectoryInvalidZeroAddress(bytes contractName);

    /**
     * @notice Constructor for the GraphDirectory contract
     * @dev Requirements:
     * - `controller` cannot be zero address
     *
     * Emits a {GraphDirectoryInitialized} event
     *
     * @param controller The address of the Graph Controller contract.
     */
    constructor(address controller) {
        require(controller != address(0), GraphDirectoryInvalidZeroAddress("Controller"));

        GRAPH_CONTROLLER = IController(controller);
        GRAPH_TOKEN = IGraphToken(_getContractFromController("GraphToken"));
        GRAPH_STAKING = IHorizonStaking(_getContractFromController("Staking"));
        GRAPH_PAYMENTS = IGraphPayments(_getContractFromController("GraphPayments"));
        GRAPH_PAYMENTS_ESCROW = IPaymentsEscrow(_getContractFromController("PaymentsEscrow"));
        GRAPH_EPOCH_MANAGER = IEpochManager(_getContractFromController("EpochManager"));
        GRAPH_REWARDS_MANAGER = IRewardsManager(_getContractFromController("RewardsManager"));
        GRAPH_TOKEN_GATEWAY = ITokenGateway(_getContractFromController("GraphTokenGateway"));
        GRAPH_PROXY_ADMIN = IGraphProxyAdmin(_getContractFromController("GraphProxyAdmin"));
        GRAPH_CURATION = ICuration(_getContractFromController("Curation"));

        emit GraphDirectoryInitialized(
            address(GRAPH_TOKEN),
            address(GRAPH_STAKING),
            address(GRAPH_PAYMENTS),
            address(GRAPH_PAYMENTS_ESCROW),
            address(GRAPH_CONTROLLER),
            address(GRAPH_EPOCH_MANAGER),
            address(GRAPH_REWARDS_MANAGER),
            address(GRAPH_TOKEN_GATEWAY),
            address(GRAPH_PROXY_ADMIN),
            address(GRAPH_CURATION)
        );
    }

    /**
     * @notice Get the Graph Token contract
     * @return The Graph Token contract
     */
    function _graphToken() internal view returns (IGraphToken) {
        return GRAPH_TOKEN;
    }

    /**
     * @notice Get the Horizon Staking contract
     * @return The Horizon Staking contract
     */
    function _graphStaking() internal view returns (IHorizonStaking) {
        return GRAPH_STAKING;
    }

    /**
     * @notice Get the Graph Payments contract
     * @return The Graph Payments contract
     */
    function _graphPayments() internal view returns (IGraphPayments) {
        return GRAPH_PAYMENTS;
    }

    /**
     * @notice Get the Payments Escrow contract
     * @return The Payments Escrow contract
     */
    function _graphPaymentsEscrow() internal view returns (IPaymentsEscrow) {
        return GRAPH_PAYMENTS_ESCROW;
    }

    /**
     * @notice Get the Graph Controller contract
     * @return The Graph Controller contract
     */
    function _graphController() internal view returns (IController) {
        return GRAPH_CONTROLLER;
    }

    /**
     * @notice Get the Epoch Manager contract
     * @return The Epoch Manager contract
     */
    function _graphEpochManager() internal view returns (IEpochManager) {
        return GRAPH_EPOCH_MANAGER;
    }

    /**
     * @notice Get the Rewards Manager contract
     * @return The Rewards Manager contract address
     */
    function _graphRewardsManager() internal view returns (IRewardsManager) {
        return GRAPH_REWARDS_MANAGER;
    }

    /**
     * @notice Get the Graph Token Gateway contract
     * @return The Graph Token Gateway contract
     */
    function _graphTokenGateway() internal view returns (ITokenGateway) {
        return GRAPH_TOKEN_GATEWAY;
    }

    /**
     * @notice Get the Graph Proxy Admin contract
     * @return The Graph Proxy Admin contract
     */
    function _graphProxyAdmin() internal view returns (IGraphProxyAdmin) {
        return GRAPH_PROXY_ADMIN;
    }

    /**
     * @notice Get the Curation contract
     * @return The Curation contract
     */
    function _graphCuration() internal view returns (ICuration) {
        return GRAPH_CURATION;
    }

    /**
     * @notice Get a contract address from the controller
     * @dev Requirements:
     * - The `_contractName` must be registered in the controller
     * @param _contractName The name of the contract to fetch from the controller
     * @return The address of the contract
     */
    function _getContractFromController(bytes memory _contractName) private view returns (address) {
        address contractAddress = GRAPH_CONTROLLER.getContractProxy(keccak256(_contractName));
        require(contractAddress != address(0), GraphDirectoryInvalidZeroAddress(_contractName));
        return contractAddress;
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

/* solhint-disable gas-custom-errors */ // Cannot use custom errors with 0.7.6

import { IGraphProxy } from "@graphprotocol/interfaces/contracts/contracts/upgrades/IGraphProxy.sol";

/**
 * @title Graph Upgradeable
 * @author Edge & Node
 * @notice This contract is intended to be inherited from upgradeable contracts.
 */
abstract contract GraphUpgradeable {
    /**
     * @dev Storage slot with the address of the current implementation.
     * This is the keccak-256 hash of "eip1967.proxy.implementation" subtracted by 1, and is
     * validated in the constructor.
     */
    bytes32 internal constant IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

    /**
     * @dev Check if the caller is the proxy admin.
     * @param _proxy The proxy contract to check admin for
     */
    modifier onlyProxyAdmin(IGraphProxy _proxy) {
        require(msg.sender == _proxy.admin(), "Caller must be the proxy admin");
        _;
    }

    /**
     * @dev Check if the caller is the implementation.
     */
    modifier onlyImpl() {
        require(msg.sender == _implementation(), "Only implementation");
        _;
    }

    /**
     * @notice Returns the current implementation.
     * @return impl Address of the current implementation
     */
    function _implementation() internal view returns (address impl) {
        bytes32 slot = IMPLEMENTATION_SLOT;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            impl := sload(slot)
        }
    }

    /**
     * @notice Accept to be an implementation of proxy.
     * @param _proxy Proxy to accept
     */
    function acceptProxy(IGraphProxy _proxy) external onlyProxyAdmin(_proxy) {
        _proxy.acceptUpgrade();
    }

    /**
     * @notice Accept to be an implementation of proxy and then call a function from the new
     * implementation as specified by `_data`, which should be an encoded function call. This is
     * useful to initialize new storage variables in the proxied contract.
     * @param _proxy Proxy to accept
     * @param _data Calldata for the initialization function call (including selector)
     */
    function acceptProxyAndCall(IGraphProxy _proxy, bytes calldata _data) external onlyProxyAdmin(_proxy) {
        _proxy.acceptUpgradeAndCall(_data);
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec

import { SafeMath } from "@openzeppelin/contracts/math/SafeMath.sol";

/**
 * @title A collection of data structures and functions to manage the Indexer Stake state.
 *        Used for low-level state changes, require() conditions should be evaluated
 *        at the caller function scope.
 */
library Stakes {
    using SafeMath for uint256;
    using Stakes for Stakes.Indexer;

    struct Indexer {
        uint256 tokensStaked; // Tokens on the indexer stake (staked by the indexer)
        uint256 tokensAllocated; // Tokens used in allocations
        uint256 tokensLocked; // Tokens locked for withdrawal subject to thawing period
        uint256 tokensLockedUntil; // Block when locked tokens can be withdrawn
    }

    /**
     * @dev Deposit tokens to the indexer stake.
     * @param stake Stake data
     * @param _tokens Amount of tokens to deposit
     */
    function deposit(Stakes.Indexer storage stake, uint256 _tokens) internal {
        stake.tokensStaked = stake.tokensStaked.add(_tokens);
    }

    /**
     * @dev Release tokens from the indexer stake.
     * @param stake Stake data
     * @param _tokens Amount of tokens to release
     */
    function release(Stakes.Indexer storage stake, uint256 _tokens) internal {
        stake.tokensStaked = stake.tokensStaked.sub(_tokens);
    }

    /**
     * @dev Allocate tokens from the main stack to a SubgraphDeployment.
     * @param stake Stake data
     * @param _tokens Amount of tokens to allocate
     */
    function allocate(Stakes.Indexer storage stake, uint256 _tokens) internal {
        stake.tokensAllocated = stake.tokensAllocated.add(_tokens);
    }

    /**
     * @dev Unallocate tokens from a SubgraphDeployment back to the main stack.
     * @param stake Stake data
     * @param _tokens Amount of tokens to unallocate
     */
    function unallocate(Stakes.Indexer storage stake, uint256 _tokens) internal {
        stake.tokensAllocated = stake.tokensAllocated.sub(_tokens);
    }

    /**
     * @dev Lock tokens until a thawing period pass.
     * @param stake Stake data
     * @param _tokens Amount of tokens to unstake
     * @param _period Period in blocks that need to pass before withdrawal
     */
    function lockTokens(Stakes.Indexer storage stake, uint256 _tokens, uint256 _period) internal {
        // Take into account period averaging for multiple unstake requests
        uint256 lockingPeriod = _period;
        if (stake.tokensLocked > 0) {
            lockingPeriod = stake.getLockingPeriod(_tokens, _period);
        }

        // Update balances
        stake.tokensLocked = stake.tokensLocked.add(_tokens);
        stake.tokensLockedUntil = block.number.add(lockingPeriod);
    }

    /**
     * @dev Unlock tokens.
     * @param stake Stake data
     * @param _tokens Amount of tokens to unkock
     */
    function unlockTokens(Stakes.Indexer storage stake, uint256 _tokens) internal {
        stake.tokensLocked = stake.tokensLocked.sub(_tokens);
        if (stake.tokensLocked == 0) {
            stake.tokensLockedUntil = 0;
        }
    }

    /**
     * @dev Take all tokens out from the locked stake for withdrawal.
     * @param stake Stake data
     * @return Amount of tokens being withdrawn
     */
    function withdrawTokens(Stakes.Indexer storage stake) internal returns (uint256) {
        // Calculate tokens that can be released
        uint256 tokensToWithdraw = stake.tokensWithdrawable();

        if (tokensToWithdraw > 0) {
            // Reset locked tokens
            stake.unlockTokens(tokensToWithdraw);

            // Decrease indexer stake
            stake.release(tokensToWithdraw);
        }

        return tokensToWithdraw;
    }

    /**
     * @dev Get the locking period of the tokens to unstake.
     * If already unstaked before calculate the weighted average.
     * @param stake Stake data
     * @param _tokens Amount of tokens to unstake
     * @param _thawingPeriod Period in blocks that need to pass before withdrawal
     * @return True if staked
     */
    function getLockingPeriod(
        Stakes.Indexer memory stake,
        uint256 _tokens,
        uint256 _thawingPeriod
    ) internal view returns (uint256) {
        uint256 blockNum = block.number;
        uint256 periodA = (stake.tokensLockedUntil > blockNum) ? stake.tokensLockedUntil.sub(blockNum) : 0;
        uint256 periodB = _thawingPeriod;
        uint256 stakeA = stake.tokensLocked;
        uint256 stakeB = _tokens;
        return periodA.mul(stakeA).add(periodB.mul(stakeB)).div(stakeA.add(stakeB));
    }

    /**
     * @dev Return true if there are tokens staked by the Indexer.
     * @param stake Stake data
     * @return True if staked
     */
    function hasTokens(Stakes.Indexer memory stake) internal pure returns (bool) {
        return stake.tokensStaked > 0;
    }

    /**
     * @dev Return the amount of tokens used in allocations and locked for withdrawal.
     * @param stake Stake data
     * @return Token amount
     */
    function tokensUsed(Stakes.Indexer memory stake) internal pure returns (uint256) {
        return stake.tokensAllocated.add(stake.tokensLocked);
    }

    /**
     * @dev Return the amount of tokens staked not considering the ones that are already going
     * through the thawing period or are ready for withdrawal. We call it secure stake because
     * it is not subject to change by a withdraw call from the indexer.
     * @param stake Stake data
     * @return Token amount
     */
    function tokensSecureStake(Stakes.Indexer memory stake) internal pure returns (uint256) {
        return stake.tokensStaked.sub(stake.tokensLocked);
    }

    /**
     * @dev Tokens free balance on the indexer stake that can be used for any purpose.
     * Any token that is allocated cannot be used as well as tokens that are going through the
     * thawing period or are withdrawable
     * Calc: tokensStaked - tokensAllocated - tokensLocked
     * @param stake Stake data
     * @return Token amount
     */
    function tokensAvailable(Stakes.Indexer memory stake) internal pure returns (uint256) {
        return stake.tokensAvailableWithDelegation(0);
    }

    /**
     * @dev Tokens free balance on the indexer stake that can be used for allocations.
     * This function accepts a parameter for extra delegated capacity that takes into
     * account delegated tokens
     * @param stake Stake data
     * @param _delegatedCapacity Amount of tokens used from delegators to calculate availability
     * @return Token amount
     */
    function tokensAvailableWithDelegation(
        Stakes.Indexer memory stake,
        uint256 _delegatedCapacity
    ) internal pure returns (uint256) {
        uint256 tokensCapacity = stake.tokensStaked.add(_delegatedCapacity);
        uint256 _tokensUsed = stake.tokensUsed();
        // If more tokens are used than the current capacity, the indexer is overallocated.
        // This means the indexer doesn't have available capacity to create new allocations.
        // We can reach this state when the indexer has funds allocated and then any
        // of these conditions happen:
        // - The delegationCapacity ratio is reduced.
        // - The indexer stake is slashed.
        // - A delegator removes enough stake.
        if (_tokensUsed > tokensCapacity) {
            // Indexer stake is over allocated: return 0 to avoid stake to be used until
            // the overallocation is restored by staking more tokens, unallocating tokens
            // or using more delegated funds
            return 0;
        }
        return tokensCapacity.sub(_tokensUsed);
    }

    /**
     * @dev Tokens available for withdrawal after thawing period.
     * @param stake Stake data
     * @return Token amount
     */
    function tokensWithdrawable(Stakes.Indexer memory stake) internal view returns (uint256) {
        // No tokens to withdraw before locking period
        if (stake.tokensLockedUntil == 0 || block.number < stake.tokensLockedUntil) {
            return 0;
        }
        return stake.tokensLocked;
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-strict-inequalities

pragma solidity 0.8.27 || 0.8.33;

/**
 * @title MathUtils Library
 * @author Edge & Node
 * @notice A collection of functions to perform math operations
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
library MathUtils {
    /**
     * @notice Calculates the weighted average of two values pondering each of these
     * values based on configured weights
     * @dev The contribution of each value N is
     * weightN/(weightA + weightB). The calculation rounds up to ensure the result
     * is always equal or greater than the smallest of the two values.
     * @param valueA The amount for value A
     * @param weightA The weight to use for value A
     * @param valueB The amount for value B
     * @param weightB The weight to use for value B
     * @return The weighted average result
     */
    function weightedAverageRoundingUp(
        uint256 valueA,
        uint256 weightA,
        uint256 valueB,
        uint256 weightB
    ) internal pure returns (uint256) {
        return ((valueA * weightA) + (valueB * weightB) + (weightA + weightB - 1)) / (weightA + weightB);
    }

    /**
     * @notice Returns the minimum of two numbers
     * @param x The first number
     * @param y The second number
     * @return The minimum of the two numbers
     */
    function min(uint256 x, uint256 y) internal pure returns (uint256) {
        return x <= y ? x : y;
    }

    /**
     * @notice Returns the difference between two numbers or zero if negative
     * @param x The first number
     * @param y The second number
     * @return The difference between the two numbers or zero if negative
     */
    function diffOrZero(uint256 x, uint256 y) internal pure returns (uint256) {
        return (x > y) ? x - y : 0;
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

/* solhint-disable gas-custom-errors */ // Cannot use custom errors with 0.7.6

import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";

/**
 * @title TokenUtils library
 * @author Edge & Node
 * @notice This library contains utility functions for handling tokens (transfers and burns).
 * It is specifically adapted for the GraphToken, so does not need to handle edge cases
 * for other tokens.
 */
library TokenUtils {
    /**
     * @notice Pull tokens from an address to this contract.
     * @param _graphToken Token to transfer
     * @param _from Address sending the tokens
     * @param _amount Amount of tokens to transfer
     */
    function pullTokens(IGraphToken _graphToken, address _from, uint256 _amount) internal {
        if (_amount > 0) {
            require(_graphToken.transferFrom(_from, address(this), _amount), "!transfer");
        }
    }

    /**
     * @notice Push tokens from this contract to a receiving address.
     * @param _graphToken Token to transfer
     * @param _to Address receiving the tokens
     * @param _amount Amount of tokens to transfer
     */
    function pushTokens(IGraphToken _graphToken, address _to, uint256 _amount) internal {
        if (_amount > 0) {
            require(_graphToken.transfer(_to, _amount), "!transfer");
        }
    }

    /**
     * @notice Burn tokens held by this contract.
     * @param _graphToken Token to burn
     * @param _amount Amount of tokens to burn
     */
    function burnTokens(IGraphToken _graphToken, uint256 _amount) internal {
        if (_amount > 0) {
            _graphToken.burn(_amount);
        }
    }
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

