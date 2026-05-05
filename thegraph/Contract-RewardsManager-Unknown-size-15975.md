
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity 0.7.6;
pragma abicoder v2;

import { SafeMath } from "@openzeppelin/contracts/math/SafeMath.sol";
import { IERC165 } from "@openzeppelin/contracts/introspection/IERC165.sol";

import { GraphUpgradeable } from "../upgrades/GraphUpgradeable.sol";
import { Managed } from "../governance/Managed.sol";
import { MathUtils } from "../staking/libs/MathUtils.sol";

import { RewardsManagerV6Storage } from "./RewardsManagerStorage.sol";
import { IRewardsIssuer } from "@graphprotocol/interfaces/contracts/contracts/rewards/IRewardsIssuer.sol";
import { IRewardsManager } from "@graphprotocol/interfaces/contracts/contracts/rewards/IRewardsManager.sol";
import { IRewardsManagerDeprecated } from "@graphprotocol/interfaces/contracts/contracts/rewards/IRewardsManagerDeprecated.sol";
import { IIssuanceAllocationDistribution } from "@graphprotocol/interfaces/contracts/issuance/allocate/IIssuanceAllocationDistribution.sol";
import { IIssuanceTarget } from "@graphprotocol/interfaces/contracts/issuance/allocate/IIssuanceTarget.sol";
import { IRewardsEligibility } from "@graphprotocol/interfaces/contracts/issuance/eligibility/IRewardsEligibility.sol";
import { RewardsCondition } from "@graphprotocol/interfaces/contracts/contracts/rewards/RewardsCondition.sol";

/**
 * @title Rewards Manager Contract
 * @author Edge & Node
 * @notice Manages indexing rewards distribution using a two-level accumulation model:
 * signal → subgraph → allocation. See docs/RewardAccountingSafety.md for details.
 *
 * @dev Issuance source: `issuanceAllocator` if set, otherwise `issuancePerBlock` storage.
 * Getter functions (getAccRewardsPerSignal, getRewards, etc.) may overestimate until
 * takeRewards is called due to pending state updates.
 */
contract RewardsManager is
    GraphUpgradeable,
    IERC165,
    IRewardsManager,
    IIssuanceTarget,
    IRewardsManagerDeprecated,
    RewardsManagerV6Storage
{
    using SafeMath for uint256;

    /// @dev Fixed point scaling factor used for decimals in reward calculations
    uint256 private constant FIXED_POINT_SCALING_FACTOR = 1e18;

    // -- Modifiers --

    /**
     * @dev Modifier to restrict access to the subgraph availability oracle only
     */
    modifier onlySubgraphAvailabilityOracle() {
        // solhint-disable-next-line gas-small-strings
        require(msg.sender == address(subgraphAvailabilityOracle), "Caller must be the subgraph availability oracle");
        _;
    }

    /**
     * @notice Initialize this contract
     * @param _controller Address of the controller contract
     */
    function initialize(address _controller) external onlyImpl {
        Managed._initialize(_controller);
    }

    /**
     * @inheritdoc IERC165
     * @dev Implements ERC165 interface detection
     * Returns true if this contract implements the interface defined by interfaceId.
     * See: https://eips.ethereum.org/EIPS/eip-165
     */
    function supportsInterface(bytes4 interfaceId) public view virtual override returns (bool) {
        return
            interfaceId == type(IERC165).interfaceId ||
            interfaceId == type(IIssuanceTarget).interfaceId ||
            interfaceId == type(IRewardsManager).interfaceId;
    }

    // -- Config --

    /**
     * @inheritdoc IRewardsManagerDeprecated
     * @dev When an IssuanceAllocator is set, the effective issuance will be determined by the allocator,
     * but this local value can still be updated for cases when the allocator is later removed.
     *
     * The issuance is defined as a fixed amount of rewards per block in GRT.
     * Whenever this function is called in layer 2, the updateL2MintAllowance function
     * _must_ be called on the L1GraphTokenGateway in L1, to ensure the bridge can mint the
     * right amount of tokens.
     */
    function setIssuancePerBlock(uint256 _issuancePerBlock) external override onlyGovernor {
        _setIssuancePerBlock(_issuancePerBlock);
    }

    /**
     * @notice Sets the GRT issuance per block.
     * @dev The issuance is defined as a fixed amount of rewards per block in GRT.
     * @param _issuancePerBlock Issuance expressed in GRT per block (scaled by 1e18)
     */
    function _setIssuancePerBlock(uint256 _issuancePerBlock) private {
        // Called since `issuance per block` will change
        updateAccRewardsPerSignal();

        issuancePerBlock = _issuancePerBlock;
        emit ParameterUpdated("issuancePerBlock");
    }

    /**
     * @inheritdoc IRewardsManager
     */
    function setSubgraphAvailabilityOracle(address _subgraphAvailabilityOracle) external override onlyGovernor {
        subgraphAvailabilityOracle = _subgraphAvailabilityOracle;
        emit ParameterUpdated("subgraphAvailabilityOracle");
    }

    /**
     * @inheritdoc IRewardsManager
     * @dev Can be set to zero which means that this feature is not being used
     * @param _minimumSubgraphSignal Minimum signaled tokens
     *
     * IMPORTANT: This function does not update existing subgraphs. When subgraphs are later
     * updated, the current threshold is applied to ALL pending rewards since their last update,
     * regardless of historical threshold values.
     *
     * ## Rewards Accounting Issue
     *
     * - Threshold increase: Pending rewards on previously eligible subgraphs are reclaimed
     * - Threshold decrease: Previously ineligible subgraphs retroactively accumulate pending rewards
     *
     * ## Mitigation
     *
     * 1. Communicate the planned threshold change with a specific future date
     * 2. Wait - notice period allows participants to adjust signal if desired
     * 3. Identify affected subgraphs off-chain (those crossing the threshold)
     * 4. Call onSubgraphSignalUpdate() for all affected subgraphs to accumulate pending rewards
     *    under current eligibility rules
     * 5. Execute threshold change via this function (promptly after step 4, ideally same block)
     */
    function setMinimumSubgraphSignal(uint256 _minimumSubgraphSignal) external override {
        // Caller can be the SAO or the governor
        require(
            msg.sender == address(subgraphAvailabilityOracle) || msg.sender == controller.getGovernor(),
            "Not authorized"
        );
        minimumSubgraphSignal = _minimumSubgraphSignal;
        emit ParameterUpdated("minimumSubgraphSignal");
    }

    /**
     * @inheritdoc IRewardsManager
     */
    function setSubgraphService(address _subgraphService) external override onlyGovernor {
        address oldSubgraphService = address(subgraphService);
        subgraphService = IRewardsIssuer(_subgraphService);
        emit SubgraphServiceSet(oldSubgraphService, _subgraphService);
    }

    /**
     * @inheritdoc IIssuanceTarget
     * @dev This function facilitates upgrades by providing a standard way for targets
     * to change their allocator. Only the governor can call this function.
     * Note that the IssuanceAllocator can be set to the zero address to disable use of an allocator, and
     * use the local `issuancePerBlock` variable instead to control issuance.
     */
    function setIssuanceAllocator(address newIssuanceAllocator) external override onlyGovernor {
        if (address(issuanceAllocator) != newIssuanceAllocator) {
            // Update rewards calculation before changing the issuance allocator
            updateAccRewardsPerSignal();

            // Check that the contract supports the IIssuanceAllocationDistribution interface
            // Allow zero address to disable the allocator
            if (newIssuanceAllocator != address(0)) {
                // solhint-disable-next-line gas-small-strings
                require(
                    IERC165(newIssuanceAllocator).supportsInterface(type(IIssuanceAllocationDistribution).interfaceId),
                    "Contract does not support IIssuanceAllocationDistribution interface"
                );
            }

            address oldIssuanceAllocator = address(issuanceAllocator);
            issuanceAllocator = IIssuanceAllocationDistribution(newIssuanceAllocator);
            emit IssuanceAllocatorSet(oldIssuanceAllocator, newIssuanceAllocator);
        }
    }

    /**
     * @inheritdoc IIssuanceTarget
     * @dev Ensures that all reward calculations are up-to-date with the current block
     * before any allocation changes take effect.
     *
     * This function can be called by anyone to update the rewards calculation state.
     * The IssuanceAllocator calls this function before changing a target's allocation to ensure
     * all issuance is properly accounted for with the current issuance rate before applying an
     * issuance allocation change.
     */
    function beforeIssuanceAllocationChange() external override {
        // Update rewards calculation with the current issuance rate
        updateAccRewardsPerSignal();
    }

    /**
     * @inheritdoc IRewardsManager
     * @dev Note that the rewards eligibility oracle can be set to the zero address to disable use of an oracle, in
     * which case no indexers will be denied rewards due to eligibility.
     */
    function setRewardsEligibilityOracle(address newRewardsEligibilityOracle) external override onlyGovernor {
        if (address(rewardsEligibilityOracle) != newRewardsEligibilityOracle) {
            // Check that the contract supports the IRewardsEligibility interface
            // Allow zero address to disable the oracle
            if (newRewardsEligibilityOracle != address(0)) {
                // solhint-disable-next-line gas-small-strings
                require(
                    IERC165(newRewardsEligibilityOracle).supportsInterface(type(IRewardsEligibility).interfaceId),
                    "Contract does not support IRewardsEligibility interface"
                );
            }

            address oldRewardsEligibilityOracle = address(rewardsEligibilityOracle);
            rewardsEligibilityOracle = IRewardsEligibility(newRewardsEligibilityOracle);
            emit RewardsEligibilityOracleSet(oldRewardsEligibilityOracle, newRewardsEligibilityOracle);
        }
    }

    /**
     * @inheritdoc IRewardsManager
     * @dev bytes32(0) is reserved as an invalid reason to prevent accidental misconfiguration
     * and catch uninitialized reason identifiers.
     *
     * IMPORTANT: Changes take effect immediately and retroactively. All unclaimed rewards from
     * previous periods will be sent to the new reclaim address when they are eventually reclaimed,
     * regardless of which address was configured when the rewards were originally accrued.
     */
    function setReclaimAddress(bytes32 reason, address newAddress) external override onlyGovernor {
        // solhint-disable-next-line gas-small-strings
        require(reason != RewardsCondition.NONE, "Cannot set reclaim address for NONE");

        address oldAddress = reclaimAddresses[reason];

        if (oldAddress != newAddress) {
            reclaimAddresses[reason] = newAddress;
            emit ReclaimAddressSet(reason, oldAddress, newAddress);
        }
    }

    /**
     * @inheritdoc IRewardsManager
     */
    function setDefaultReclaimAddress(address newAddress) external override onlyGovernor {
        address oldAddress = defaultReclaimAddress;

        if (oldAddress != newAddress) {
            defaultReclaimAddress = newAddress;
            emit DefaultReclaimAddressSet(oldAddress, newAddress);
        }
    }

    // -- Denylist --

    /**
     * @inheritdoc IRewardsManager
     * @dev Can only be called by the subgraph availability oracle
     */
    function setDenied(bytes32 subgraphDeploymentId, bool deny) external override onlySubgraphAvailabilityOracle {
        _setDenied(subgraphDeploymentId, deny);
    }

    /**
     * @notice Sets the denied status for a subgraph.
     * @dev Idempotent: redundant calls skip the update but still call `onSubgraphAllocationUpdate`.
     * @param subgraphDeploymentId Subgraph deployment ID
     * @param deny True to deny rewards, false to allow
     */
    function _setDenied(bytes32 subgraphDeploymentId, bool deny) private {
        onSubgraphAllocationUpdate(subgraphDeploymentId);

        bool stateChange = deny == (denylist[subgraphDeploymentId] == 0);
        if (stateChange) {
            uint256 sinceBlock = deny ? block.number : 0;
            denylist[subgraphDeploymentId] = sinceBlock;
            emit RewardsDenylistUpdated(subgraphDeploymentId, sinceBlock);
        }
    }

    /// @inheritdoc IRewardsManager
    function isDenied(bytes32 _subgraphDeploymentID) public view override returns (bool) {
        return denylist[_subgraphDeploymentID] > 0;
    }

    // -- Getters --

    /**
     * @inheritdoc IRewardsManager
     */
    function getAllocatedIssuancePerBlock() public view override returns (uint256) {
        return
            address(issuanceAllocator) != address(0)
                ? issuanceAllocator.getTargetIssuancePerBlock(address(this)).selfIssuanceRate
                : issuancePerBlock;
    }

    /**
     * @inheritdoc IRewardsManager
     */
    function getRawIssuancePerBlock() external view override returns (uint256) {
        return issuancePerBlock;
    }

    /**
     * @inheritdoc IRewardsManager
     */
    function getIssuanceAllocator() external view override returns (IIssuanceAllocationDistribution) {
        return issuanceAllocator;
    }

    /**
     * @inheritdoc IRewardsManager
     */
    function getReclaimAddress(bytes32 reason) external view override returns (address) {
        return reclaimAddresses[reason];
    }

    /**
     * @inheritdoc IRewardsManager
     */
    function getDefaultReclaimAddress() external view override returns (address) {
        return defaultReclaimAddress;
    }

    /**
     * @inheritdoc IRewardsManager
     */
    function getRewardsEligibilityOracle() external view override returns (IRewardsEligibility) {
        return rewardsEligibilityOracle;
    }

    /// @inheritdoc IRewardsManager
    function getNewRewardsPerSignal() public view override returns (uint256 claimablePerSignal) {
        (claimablePerSignal, ) = _getNewRewardsPerSignal();
    }

    /**
     * @notice Calculate new rewards per signal since last update
     * @dev Formula: `x = r * t` where t = blocks since last update.
     * @return claimablePerSignal Rewards per signal (scaled by FIXED_POINT_SCALING_FACTOR)
     * @return unclaimableTokens Tokens not distributed due to zero signal
     */
    function _getNewRewardsPerSignal() private view returns (uint256 claimablePerSignal, uint256 unclaimableTokens) {
        // Calculate time steps
        uint256 t = block.number.sub(accRewardsPerSignalLastBlockUpdated);
        // Optimization to skip calculations if zero time steps elapsed
        if (t == 0) return (0, 0);

        uint256 rewardsIssuancePerBlock = getAllocatedIssuancePerBlock();

        if (rewardsIssuancePerBlock == 0) return (0, 0);

        uint256 x = rewardsIssuancePerBlock.mul(t);

        // Check signalled tokens
        uint256 signalledTokens = graphToken().balanceOf(address(curation()));
        if (signalledTokens == 0) return (0, x); // All unclaimable when no signal

        // Get the new issuance per signalled token
        // We multiply the decimals to keep the precision as fixed-point number
        return (x.mul(FIXED_POINT_SCALING_FACTOR).div(signalledTokens), 0);
    }

    /// @inheritdoc IRewardsManager
    function getAccRewardsPerSignal() public view override returns (uint256) {
        return accRewardsPerSignal.add(getNewRewardsPerSignal());
    }

    /**
     * @inheritdoc IRewardsManager
     * @dev Returns accumulated rewards for external callers.
     * New rewards are only included if the subgraph is claimable (neither denied nor below minimum signal).
     * Reclaim for non-claimable subgraphs is handled in `onSubgraphSignalUpdate()` and `onSubgraphAllocationUpdate()`.
     */
    function getAccRewardsForSubgraph(bytes32 _subgraphDeploymentID) public view override returns (uint256) {
        Subgraph storage subgraph = subgraphs[_subgraphDeploymentID];
        (uint256 newRewards, , bytes32 condition) = _getSubgraphRewardsState(_subgraphDeploymentID);
        return subgraph.accRewardsForSubgraph.add(condition == RewardsCondition.NONE ? newRewards : 0);
    }

    /**
     * @inheritdoc IRewardsManager
     * @dev New rewards are only included via `getAccRewardsForSubgraph` when subgraph is claimable.
     * Pre-existing stored rewards are always shown as distributable (preserved for when conditions clear).
     * Does not check indexer eligibility - that can change and doesn't affect reward accrual.
     */
    function getAccRewardsPerAllocatedToken(
        bytes32 _subgraphDeploymentID
    ) public view override returns (uint256, uint256) {
        Subgraph storage subgraph = subgraphs[_subgraphDeploymentID];

        // getAccRewardsForSubgraph already handles claimability: excludes new rewards when not claimable
        uint256 accRewardsForSubgraph = getAccRewardsForSubgraph(_subgraphDeploymentID);
        uint256 newRewardsForSubgraph = MathUtils.diffOrZero(
            accRewardsForSubgraph,
            subgraph.accRewardsForSubgraphSnapshot
        );

        // Get total allocated tokens across all issuers
        uint256 subgraphAllocatedTokens = _getSubgraphAllocatedTokens(_subgraphDeploymentID);

        if (subgraphAllocatedTokens == 0) {
            // No allocations to distribute to, return stored value (no pending updates possible)
            return (subgraph.accRewardsPerAllocatedToken, accRewardsForSubgraph);
        }

        uint256 newRewardsPerAllocatedToken = newRewardsForSubgraph.mul(FIXED_POINT_SCALING_FACTOR).div(
            subgraphAllocatedTokens
        );
        return (subgraph.accRewardsPerAllocatedToken.add(newRewardsPerAllocatedToken), accRewardsForSubgraph);
    }

    // -- Internal Helpers --

    /**
     * @notice Get subgraph rewards state including effective reclaim condition
     * @dev Determines claimability with priority: SUBGRAPH_DENIED > BELOW_MINIMUM_SIGNAL > NO_ALLOCATED_TOKENS > NONE
     * When multiple conditions apply, prefers conditions with configured reclaim addresses.
     * @param _subgraphDeploymentID Subgraph deployment
     * @return newRewards Rewards accumulated since last snapshot
     * @return subgraphAllocatedTokens Total tokens allocated to this subgraph
     * @return condition The effective condition for reclaim routing (NONE if claimable)
     */
    function _getSubgraphRewardsState(
        bytes32 _subgraphDeploymentID
    ) private view returns (uint256 newRewards, uint256 subgraphAllocatedTokens, bytes32 condition) {
        Subgraph storage subgraph = subgraphs[_subgraphDeploymentID];
        uint256 signalledTokens = curation().getCurationPoolTokens(_subgraphDeploymentID);
        uint256 accRewardsPerSignalDelta = getAccRewardsPerSignal().sub(subgraph.accRewardsPerSignalSnapshot);
        newRewards = accRewardsPerSignalDelta.mul(signalledTokens).div(FIXED_POINT_SCALING_FACTOR);
        subgraphAllocatedTokens = _getSubgraphAllocatedTokens(_subgraphDeploymentID);

        condition = isDenied(_subgraphDeploymentID) ? RewardsCondition.SUBGRAPH_DENIED : RewardsCondition.NONE;
        if (
            signalledTokens < minimumSubgraphSignal &&
            (condition == RewardsCondition.NONE || reclaimAddresses[condition] == address(0))
        ) condition = RewardsCondition.BELOW_MINIMUM_SIGNAL;
        if (
            subgraphAllocatedTokens == 0 &&
            (condition == RewardsCondition.NONE || reclaimAddresses[condition] == address(0))
        ) condition = RewardsCondition.NO_ALLOCATED_TOKENS;
    }

    /**
     * @notice Get total allocated tokens for a subgraph across all issuers
     * @param _subgraphDeploymentID Subgraph deployment
     * @return Total tokens allocated to this subgraph
     */
    function _getSubgraphAllocatedTokens(bytes32 _subgraphDeploymentID) private view returns (uint256) {
        uint256 subgraphAllocatedTokens = 0;
        address[2] memory rewardsIssuers = [address(staking()), address(subgraphService)];
        for (uint256 i = 0; i < rewardsIssuers.length; ++i) {
            if (rewardsIssuers[i] != address(0)) {
                subgraphAllocatedTokens += IRewardsIssuer(rewardsIssuers[i]).getSubgraphAllocatedTokens(
                    _subgraphDeploymentID
                );
            }
        }
        return subgraphAllocatedTokens;
    }

    // -- Updates --

    /**
     * @inheritdoc IRewardsManager
     * @dev Must be called before `issuancePerBlock` or `total signalled GRT` changes.
     * Called from the Curation contract on mint() and burn()
     *
     * ## Zero Signal Handling
     *
     * When total signalled tokens is zero, issuance for the period is reclaimed
     * (if NO_SIGNAL reclaim address is configured) rather than being lost.
     */
    function updateAccRewardsPerSignal() public override returns (uint256) {
        if (accRewardsPerSignalLastBlockUpdated == block.number) return accRewardsPerSignal;

        (uint256 claimablePerSignal, uint256 unclaimableTokens) = _getNewRewardsPerSignal();

        if (0 < unclaimableTokens)
            _reclaimRewards(RewardsCondition.NO_SIGNAL, unclaimableTokens, address(0), address(0), bytes32(0));

        uint256 newAccRewardsPerSignal = accRewardsPerSignal.add(claimablePerSignal);
        accRewardsPerSignal = newAccRewardsPerSignal;
        accRewardsPerSignalLastBlockUpdated = block.number;
        return newAccRewardsPerSignal;
    }

    /**
     * @notice Internal function that updates subgraph reward accumulators.
     * Shared logic for both signal and allocation update hooks.
     *
     * @param subgraph Storage pointer to the subgraph
     * @param _subgraphDeploymentID The subgraph deployment ID
     * @param accRewardsPerSignal Current global rewards per signal
     * @param accRewardsForSubgraph Current subgraph accumulated rewards
     * @param accRewardsPerAllocatedToken Current rewards per allocated token
     * @return newAccRewardsForSubgraph Updated subgraph accumulated rewards
     * @return newAccRewardsPerAllocatedToken Updated rewards per allocated token
     */
    function _updateSubgraphRewards(
        Subgraph storage subgraph,
        bytes32 _subgraphDeploymentID,
        uint256 accRewardsPerSignal,
        uint256 accRewardsForSubgraph,
        uint256 accRewardsPerAllocatedToken
    ) internal returns (uint256 newAccRewardsForSubgraph, uint256 newAccRewardsPerAllocatedToken) {
        (
            uint256 rewardsSinceSignalSnapshot,
            uint256 subgraphAllocatedTokens,
            bytes32 condition
        ) = _getSubgraphRewardsState(_subgraphDeploymentID);
        subgraph.accRewardsPerSignalSnapshot = accRewardsPerSignal;

        // Calculate undistributed: rewards accumulated but not yet distributed to allocations.
        // Will be just rewards since last snapshot for subgraphs that have had onSubgraphSignalUpdate or
        // onSubgraphAllocationUpdate called since upgrade;
        // can include non-zero (original) accRewardsForSubgraph - accRewardsForSubgraphSnapshot for
        // subgraphs that have not had either hook called since upgrade.
        uint256 undistributedRewards = accRewardsForSubgraph.sub(subgraph.accRewardsForSubgraphSnapshot).add(
            rewardsSinceSignalSnapshot
        );

        if (condition != RewardsCondition.NONE) {
            _reclaimRewards(condition, undistributedRewards, address(0), address(0), _subgraphDeploymentID);
            undistributedRewards = 0;
            newAccRewardsForSubgraph = accRewardsForSubgraph;
        } else {
            newAccRewardsForSubgraph = accRewardsForSubgraph.add(rewardsSinceSignalSnapshot);
            subgraph.accRewardsForSubgraph = newAccRewardsForSubgraph;
        }

        subgraph.accRewardsForSubgraphSnapshot = newAccRewardsForSubgraph;

        newAccRewardsPerAllocatedToken = accRewardsPerAllocatedToken;
        if (undistributedRewards != 0) {
            newAccRewardsPerAllocatedToken = accRewardsPerAllocatedToken.add(
                undistributedRewards.mul(FIXED_POINT_SCALING_FACTOR).div(subgraphAllocatedTokens)
            );
            subgraph.accRewardsPerAllocatedToken = newAccRewardsPerAllocatedToken;
        }
    }

    /**
     * @inheritdoc IRewardsManager
     * @dev Must be called before `signalled GRT` on a subgraph changes.
     * Hook called from the Curation contract on mint() and burn()
     *
     * ## Claimability Behavior
     *
     * When a subgraph is not claimable (denied, below minimum signal, or no allocations):
     * - Rewards are reclaimed immediately with the appropriate reason
     * - `accRewardsForSubgraph` is NOT updated (rewards go to reclaim, not accumulator)
     *
     * When claimable (not denied, above minimum signal, has allocations):
     * - Rewards are added to `accRewardsForSubgraph` for later distribution via `onSubgraphAllocationUpdate`
     */
    function onSubgraphSignalUpdate(
        bytes32 _subgraphDeploymentID
    ) external override returns (uint256 accRewardsForSubgraph) {
        // Called since `total signalled GRT` will change
        uint256 accRewardsPerSignal = updateAccRewardsPerSignal();

        Subgraph storage subgraph = subgraphs[_subgraphDeploymentID];
        accRewardsForSubgraph = subgraph.accRewardsForSubgraph;

        if (subgraph.accRewardsPerSignalSnapshot == accRewardsPerSignal) return accRewardsForSubgraph;

        (accRewardsForSubgraph, ) = _updateSubgraphRewards(
            subgraph,
            _subgraphDeploymentID,
            accRewardsPerSignal,
            accRewardsForSubgraph,
            subgraph.accRewardsPerAllocatedToken
        );
    }

    /**
     * @inheritdoc IRewardsManager
     * @dev Hook called from the Staking contract on allocate() and close()
     *
     * ## Claimability Behavior
     *
     * When a subgraph is not claimable (denied, below minimum signal, or no allocations):
     * - Rewards are reclaimed immediately with the appropriate reason
     * - `accRewardsForSubgraph` is NOT updated (rewards go to reclaim, not accumulator)
     * - `accRewardsPerAllocatedToken` does NOT increase
     *
     * When claimable (not denied, above minimum signal, has allocations):
     * - Rewards are added to `accRewardsForSubgraph`
     * - `accRewardsPerAllocatedToken` increases (rewards distributable to allocations)
     *
     * @return accRewardsPerAllocatedToken Current `accRewardsPerAllocatedToken`
     */
    function onSubgraphAllocationUpdate(
        bytes32 _subgraphDeploymentID
    ) public override returns (uint256 accRewardsPerAllocatedToken) {
        Subgraph storage subgraph = subgraphs[_subgraphDeploymentID];

        uint256 accRewardsPerSignal = updateAccRewardsPerSignal();
        uint256 accRewardsForSubgraph = subgraph.accRewardsForSubgraph;
        accRewardsPerAllocatedToken = subgraph.accRewardsPerAllocatedToken;

        // Return early to save gas if both snapshots are up-to-date
        if (
            subgraph.accRewardsPerSignalSnapshot == accRewardsPerSignal &&
            subgraph.accRewardsForSubgraphSnapshot == accRewardsForSubgraph
        ) return accRewardsPerAllocatedToken;

        (, accRewardsPerAllocatedToken) = _updateSubgraphRewards(
            subgraph,
            _subgraphDeploymentID,
            accRewardsPerSignal,
            accRewardsForSubgraph,
            accRewardsPerAllocatedToken
        );
    }

    /**
     * @inheritdoc IRewardsManager
     * @dev Reflects the gap between the subgraph accumulator and the allocation's snapshot, plus
     * stored pending rewards. During exclusion (denied, below minimum signal, no allocations), the
     * accumulator is frozen: new rewards are excluded but the existing gap remains claimable when
     * conditions clear. Does not check indexer eligibility - that is verified at claim time via
     * takeRewards().
     */
    function getRewards(address _rewardsIssuer, address _allocationID) external view override returns (uint256) {
        require(
            _rewardsIssuer == address(staking()) || _rewardsIssuer == address(subgraphService),
            "Not a rewards issuer"
        );

        (
            bool isActive,
            ,
            bytes32 subgraphDeploymentId,
            uint256 tokens,
            uint256 alloAccRewardsPerAllocatedToken,
            uint256 accRewardsPending
        ) = IRewardsIssuer(_rewardsIssuer).getAllocationData(_allocationID);

        if (!isActive) {
            return 0;
        }

        (uint256 accRewardsPerAllocatedToken, ) = getAccRewardsPerAllocatedToken(subgraphDeploymentId);
        return
            accRewardsPending.add(_calcRewards(tokens, alloAccRewardsPerAllocatedToken, accRewardsPerAllocatedToken));
    }

    /**
     * @notice Calculate rewards for a given accumulated rewards per allocated token
     * @param _tokens Tokens allocated
     * @param _accRewardsPerAllocatedToken Allocation accumulated rewards per token
     * @return Rewards amount
     */
    function calcRewards(
        uint256 _tokens,
        uint256 _accRewardsPerAllocatedToken
    ) external pure override returns (uint256) {
        return _accRewardsPerAllocatedToken.mul(_tokens).div(FIXED_POINT_SCALING_FACTOR);
    }

    /**
     * @notice Calculate current rewards for a given allocation.
     * @param _tokens Tokens allocated
     * @param _startAccRewardsPerAllocatedToken Allocation start accumulated rewards
     * @param _endAccRewardsPerAllocatedToken Allocation end accumulated rewards
     * @return Rewards amount
     */
    function _calcRewards(
        uint256 _tokens,
        uint256 _startAccRewardsPerAllocatedToken,
        uint256 _endAccRewardsPerAllocatedToken
    ) private pure returns (uint256) {
        uint256 newAccrued = _endAccRewardsPerAllocatedToken.sub(_startAccRewardsPerAllocatedToken);
        return newAccrued.mul(_tokens).div(FIXED_POINT_SCALING_FACTOR);
    }

    /**
     * @notice Calculate rewards for an allocation
     * @param rewardsIssuer Address of the rewards issuer calling the function
     * @param allocationID Address of the allocation
     * @return rewards Amount of rewards calculated
     * @return indexer Address of the indexer
     * @return subgraphDeploymentID Subgraph deployment ID
     */
    function _calcAllocationRewards(
        address rewardsIssuer,
        address allocationID
    ) private returns (uint256 rewards, address indexer, bytes32 subgraphDeploymentID) {
        (
            bool isActive,
            address _indexer,
            bytes32 _subgraphDeploymentID,
            uint256 tokens,
            uint256 accRewardsPerAllocatedToken,
            uint256 accRewardsPending
        ) = IRewardsIssuer(rewardsIssuer).getAllocationData(allocationID);

        uint256 updatedAccRewardsPerAllocatedToken = onSubgraphAllocationUpdate(_subgraphDeploymentID);

        rewards = isActive
            ? accRewardsPending.add(
                _calcRewards(tokens, accRewardsPerAllocatedToken, updatedAccRewardsPerAllocatedToken)
            )
            : 0;

        indexer = _indexer;
        subgraphDeploymentID = _subgraphDeploymentID;
    }

    /**
     * @notice Reclaim rewards to reason-specific address or default fallback
     * @param reason Reclaim reason identifier
     * @param rewards Amount of rewards to reclaim
     * @param indexer Address of the indexer
     * @param allocationId Address of the allocation
     * @param subgraphDeploymentId Subgraph deployment ID for the allocation
     * @return Amount reclaimed (0 if no target address configured)
     *
     * @dev ## Reclaim Priority
     *
     * 1. Try the reason-specific address
     * 2. If not configured, try defaultReclaimAddress
     * 3. If neither configured, rewards are dropped (not minted), returns 0
     */
    function _reclaimRewards(
        bytes32 reason,
        uint256 rewards,
        address indexer,
        address allocationId,
        bytes32 subgraphDeploymentId
    ) private returns (uint256) {
        if (rewards == 0) return 0;
        if (reason == RewardsCondition.NONE) return 0; // NONE cannot be used as reclaim reason

        address target = reclaimAddresses[reason];
        if (target == address(0)) target = defaultReclaimAddress;
        if (target == address(0)) return 0; // Dropped, not reclaimed

        graphToken().mint(target, rewards);
        emit RewardsReclaimed(reason, rewards, indexer, allocationId, subgraphDeploymentId);
        return rewards;
    }

    /**
     * @notice Check if rewards should be denied and attempt to reclaim them
     * @param rewards Amount of rewards to check
     * @param indexer Address of the indexer
     * @param allocationID Address of the allocation
     * @param subgraphDeploymentID Subgraph deployment ID for the allocation
     * @return denied True if rewards are denied (either reclaimed or dropped), false if they should be minted
     * @dev Emits denial events, then attempts reclaim.
     * Prefers subgraph denial over indexer ineligibility as reason when both apply.
     * First configured applicable reclaim address is used.
     * If rewards denied but no specific address is configured, the default reclaim address is used.
     * If no applicable reclaim address is configured, rewards are not minted.
     */
    function _deniedRewards(
        uint256 rewards,
        address indexer,
        address allocationID,
        bytes32 subgraphDeploymentID
    ) private returns (bool denied) {
        bool isDeniedSubgraph = isDenied(subgraphDeploymentID);
        bool isIneligible = address(rewardsEligibilityOracle) != address(0) &&
            !rewardsEligibilityOracle.isEligible(indexer);
        if (!isDeniedSubgraph && !isIneligible) return false;

        if (isDeniedSubgraph) emit RewardsDenied(indexer, allocationID);
        if (isIneligible) emit RewardsDeniedDueToEligibility(indexer, allocationID, rewards);

        bytes32 reason = isDeniedSubgraph ? RewardsCondition.SUBGRAPH_DENIED : RewardsCondition.NONE;
        if (isIneligible && (!isDeniedSubgraph || reclaimAddresses[reason] == address(0)))
            reason = RewardsCondition.INDEXER_INELIGIBLE;

        _reclaimRewards(reason, rewards, indexer, allocationID, subgraphDeploymentID);
        return true;
    }

    /**
     * @inheritdoc IRewardsManager
     * @dev This function can only be called by an authorized rewards issuer which are
     * the staking contract (for legacy allocations), and the subgraph service (for new allocations).
     * Mints 0 tokens if the allocation is not active.
     * @dev First successful reclaim wins - short-circuits on reclaim:
     * - If subgraph denied with reclaim address → reclaim to SUBGRAPH_DENIED address (eligibility NOT checked)
     * - If subgraph not denied OR denied without address, then check eligibility → reclaim to INDEXER_INELIGIBLE if configured
     * - Subsequent denial emitted only when earlier denial has no reclaim address
     * - Any denial without reclaim address drops rewards (no minting)
     */
    function takeRewards(address _allocationID) external override returns (uint256) {
        address rewardsIssuer = msg.sender;
        require(
            rewardsIssuer == address(staking()) || rewardsIssuer == address(subgraphService),
            "Caller must be a rewards issuer"
        );

        (uint256 rewards, address indexer, bytes32 subgraphDeploymentID) = _calcAllocationRewards(
            rewardsIssuer,
            _allocationID
        );

        if (rewards == 0) return 0;
        if (_deniedRewards(rewards, indexer, _allocationID, subgraphDeploymentID)) return 0;

        graphToken().mint(rewardsIssuer, rewards);
        emit HorizonRewardsAssigned(indexer, _allocationID, rewards);

        return rewards;
    }

    /**
     * @inheritdoc IRewardsManager
     * @dev bytes32(0) (NONE) cannot be used as a reclaim reason and will return 0.
     * Use specific RewardsCondition constants for reclaim reasons.
     */
    function reclaimRewards(bytes32 reason, address allocationID) external override returns (uint256) {
        address rewardsIssuer = msg.sender;
        require(rewardsIssuer == address(subgraphService), "Not a rewards issuer");

        (uint256 rewards, address indexer, bytes32 subgraphDeploymentID) = _calcAllocationRewards(
            rewardsIssuer,
            allocationID
        );

        return _reclaimRewards(reason, rewards, indexer, allocationID, subgraphDeploymentID);
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-indexed-events, gas-small-strings
// solhint-disable named-parameters-mapping

/* solhint-disable gas-custom-errors */ // Cannot use custom errors with 0.7.6

import { IController } from "@graphprotocol/interfaces/contracts/contracts/governance/IController.sol";
import { IManaged } from "@graphprotocol/interfaces/contracts/contracts/governance/IManaged.sol";
import { Governed } from "./Governed.sol";
import { Pausable } from "./Pausable.sol";

/**
 * @title Graph Controller contract
 * @author Edge & Node
 * @notice Controller is a registry of contracts for convenience. Inspired by Livepeer:
 * https://github.com/livepeer/protocol/blob/streamflow/contracts/Controller.sol
 */
contract Controller is Governed, Pausable, IController {
    /// @dev Track contract ids to contract proxy address
    mapping(bytes32 => address) private _registry;

    /**
     * @notice Emitted when the proxy address for a protocol contract has been set
     * @param id Contract identifier
     * @param contractAddress Address of the contract proxy
     */
    event SetContractProxy(bytes32 indexed id, address contractAddress);

    /**
     * @notice Controller contract constructor.
     */
    constructor() {
        Governed._initialize(msg.sender);

        _setPaused(true);
    }

    /**
     * @dev Check if the caller is the governor or pause guardian.
     */
    modifier onlyGovernorOrGuardian() {
        require(msg.sender == governor || msg.sender == pauseGuardian, "Only Governor or Guardian can call");
        _;
    }

    /**
     * @inheritdoc IController
     */
    function getGovernor() external view override returns (address) {
        return governor;
    }

    // -- Registry --

    /**
     * @inheritdoc IController
     */
    function setContractProxy(bytes32 _id, address _contractAddress) external override onlyGovernor {
        require(_contractAddress != address(0), "Contract address must be set");
        _registry[_id] = _contractAddress;
        emit SetContractProxy(_id, _contractAddress);
    }

    /**
     * @inheritdoc IController
     */
    function unsetContractProxy(bytes32 _id) external override onlyGovernor {
        _registry[_id] = address(0);
        emit SetContractProxy(_id, address(0));
    }

    /**
     * @inheritdoc IController
     */
    function getContractProxy(bytes32 _id) external view override returns (address) {
        return _registry[_id];
    }

    /**
     * @inheritdoc IController
     */
    function updateController(bytes32 _id, address _controller) external override onlyGovernor {
        require(_controller != address(0), "Controller must be set");
        return IManaged(_registry[_id]).setController(_controller);
    }

    // -- Pausing --

    /**
     * @notice Change the partial paused state of the contract
     * Partial pause is intended as a partial pause of the protocol
     * @param _toPartialPause True if the contracts should be (partially) paused, false otherwise
     */
    function setPartialPaused(bool _toPartialPause) external override onlyGovernorOrGuardian {
        _setPartialPaused(_toPartialPause);
    }

    /**
     * @inheritdoc IController
     * @dev Full pause most of protocol functions
     */
    function setPaused(bool _toPause) external override onlyGovernorOrGuardian {
        _setPaused(_toPause);
    }

    /**
     * @inheritdoc IController
     */
    function setPauseGuardian(address _newPauseGuardian) external override onlyGovernor {
        require(_newPauseGuardian != address(0), "PauseGuardian must be set");
        _setPauseGuardian(_newPauseGuardian);
    }

    /**
     * @inheritdoc IController
     */
    function paused() external view override returns (bool) {
        return _paused;
    }

    /**
     * @inheritdoc IController
     */
    function partialPaused() external view override returns (bool) {
        return _partialPaused;
    }
}

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

/* solhint-disable one-contract-per-file */

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable named-parameters-mapping

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

import { IIssuanceAllocationDistribution } from "@graphprotocol/interfaces/contracts/issuance/allocate/IIssuanceAllocationDistribution.sol";
import { IRewardsEligibility } from "@graphprotocol/interfaces/contracts/issuance/eligibility/IRewardsEligibility.sol";
import { IRewardsIssuer } from "@graphprotocol/interfaces/contracts/contracts/rewards/IRewardsIssuer.sol";
import { IRewardsManager } from "@graphprotocol/interfaces/contracts/contracts/rewards/IRewardsManager.sol";
import { IRewardsManagerDeprecated } from "@graphprotocol/interfaces/contracts/contracts/rewards/IRewardsManagerDeprecated.sol";
import { Managed } from "../governance/Managed.sol";

/**
 * @title RewardsManagerV1Storage
 * @author Edge & Node
 * @notice Storage layout for RewardsManager V1
 */
contract RewardsManagerV1Storage is Managed {
    // -- State --

    /// @dev Deprecated issuance rate variable (no longer used)
    uint256 private __DEPRECATED_issuanceRate; // solhint-disable-line var-name-mixedcase

    /// @notice Accumulated rewards per signal (fixed-point, scaled by 1e18)
    /// @dev Never decreases. Only increases via updateAccRewardsPerSignal().
    /// Represents the cumulative GRT rewards per signaled token since contract deployment.
    uint256 public accRewardsPerSignal;

    /// @notice Block number when accumulated rewards per signal was last updated
    /// @dev Used to calculate time delta for new reward accrual. Must be updated atomically
    /// with accRewardsPerSignal to maintain accounting consistency.
    uint256 public accRewardsPerSignalLastBlockUpdated;

    /// @notice Address of role allowed to deny rewards on subgraphs
    address public subgraphAvailabilityOracle;

    /// @notice Subgraph related rewards: subgraph deployment ID => subgraph rewards
    /// @dev Accumulation state tracked per subgraph.
    mapping(bytes32 => IRewardsManager.Subgraph) public subgraphs;

    /// @notice Subgraph denylist: subgraph deployment ID => block when added or zero (if not denied)
    /// @dev **Denial Semantics**:
    /// - Non-zero value: subgraph is denied since that block number
    /// - Zero value: subgraph is not denied
    /// - When denied: accRewardsPerAllocatedToken freezes (stops updating)
    /// - New rewards during denial are reclaimed (if reclaim address configured) or dropped
    mapping(bytes32 => uint256) public denylist;
}

/**
 * @title RewardsManagerV2Storage
 * @author Edge & Node
 * @notice Storage layout for RewardsManager V2
 */
contract RewardsManagerV2Storage is RewardsManagerV1Storage {
    /// @notice Minimum amount of signaled tokens on a subgraph required to accrue rewards
    uint256 public minimumSubgraphSignal;
}

/**
 * @title RewardsManagerV3Storage
 * @author Edge & Node
 * @notice Storage layout for RewardsManager V3
 */
contract RewardsManagerV3Storage is RewardsManagerV2Storage {
    /// @dev Deprecated token supply snapshot variable (no longer used)
    uint256 private __DEPRECATED_tokenSupplySnapshot; // solhint-disable-line var-name-mixedcase
}

/**
 * @title RewardsManagerV4Storage
 * @author Edge & Node
 * @notice Storage layout for RewardsManager V4
 */
abstract contract RewardsManagerV4Storage is IRewardsManagerDeprecated, RewardsManagerV3Storage {
    /// @notice GRT issued for indexer rewards per block
    /// @dev Only used when issuanceAllocator is zero address.
    uint256 public override issuancePerBlock;
}

/**
 * @title RewardsManagerV5Storage
 * @author Edge & Node
 * @notice Storage layout for RewardsManager V5
 */
abstract contract RewardsManagerV5Storage is IRewardsManager, RewardsManagerV4Storage {
    /// @notice Address of the subgraph service
    IRewardsIssuer public override subgraphService;
}

/**
 * @title RewardsManagerV6Storage
 * @author Edge & Node
 * @notice Storage layout for RewardsManager V6
 * Includes support for Rewards Eligibility Oracle, Issuance Allocator, and reclaim addresses.
 */
abstract contract RewardsManagerV6Storage is RewardsManagerV5Storage {
    /// @dev Address of the rewards eligibility oracle contract
    /// When set, indexers must pass eligibility check to claim rewards.
    /// Zero address disables eligibility checks.
    IRewardsEligibility internal rewardsEligibilityOracle;

    /// @dev Address of the issuance allocator
    /// When set, determines GRT issued per block. Zero address uses issuancePerBlock storage value.
    IIssuanceAllocationDistribution internal issuanceAllocator;

    /// @dev Mapping of reclaim reason identifiers to reclaim addresses
    /// @dev Uses bytes32 for extensibility. See RewardsCondition library for canonical reasons.
    /// **IMPORTANT**: Changes to reclaim addresses are retroactive. When an address is changed,
    /// ALL future reclaims for that reason go to the new address, regardless of when the
    /// rewards were originally accrued. Zero address means rewards are dropped (not minted).
    mapping(bytes32 => address) internal reclaimAddresses;
    /// @dev Default fallback address for reclaiming rewards when no reason-specific address is configured.
    /// Zero address means rewards are dropped (not minted) if no specific reclaim address matches.
    address internal defaultReclaimAddress;
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-indexed-events

/**
 * @title Pausable Contract
 * @author Edge & Node
 * @notice Abstract contract that provides pause functionality for protocol operations
 */
abstract contract Pausable {
    /**
     * @dev "Partial paused" pauses exit and enter functions for GRT, but not internal
     * functions, such as allocating
     */
    bool internal _partialPaused;
    /**
     * @dev Paused will pause all major protocol functions
     */
    bool internal _paused;

    /// @notice Timestamp for the last time the partial pause was set
    uint256 public lastPartialPauseTime;
    /// @notice Timestamp for the last time the full pause was set
    uint256 public lastPauseTime;

    /// @notice Pause guardian is a separate entity from the governor that can
    /// pause and unpause the protocol, fully or partially
    address public pauseGuardian;

    /**
     * @notice Emitted when the partial pause state changed
     * @param isPaused Whether the contract is partially paused
     */
    event PartialPauseChanged(bool isPaused);

    /**
     * @notice Emitted when the full pause state changed
     * @param isPaused Whether the contract is fully paused
     */
    event PauseChanged(bool isPaused);

    /**
     * @notice Emitted when the pause guardian is changed
     * @param oldPauseGuardian Address of the previous pause guardian
     * @param pauseGuardian Address of the new pause guardian
     */
    event NewPauseGuardian(address indexed oldPauseGuardian, address indexed pauseGuardian);

    /**
     * @notice Change the partial paused state of the contract
     * @param _toPartialPause New value for the partial pause state (true means the contracts will be partially paused)
     */
    function _setPartialPaused(bool _toPartialPause) internal {
        if (_toPartialPause == _partialPaused) {
            return;
        }
        _partialPaused = _toPartialPause;
        if (_partialPaused) {
            lastPartialPauseTime = block.timestamp;
        }
        emit PartialPauseChanged(_partialPaused);
    }

    /**
     * @notice Change the paused state of the contract
     * @param _toPause New value for the pause state (true means the contracts will be paused)
     */
    function _setPaused(bool _toPause) internal {
        if (_toPause == _paused) {
            return;
        }
        _paused = _toPause;
        if (_paused) {
            lastPauseTime = block.timestamp;
        }
        emit PauseChanged(_paused);
    }

    /**
     * @notice Change the Pause Guardian
     * @param newPauseGuardian The address of the new Pause Guardian
     */
    function _setPauseGuardian(address newPauseGuardian) internal {
        address oldPauseGuardian = pauseGuardian;
        pauseGuardian = newPauseGuardian;
        emit NewPauseGuardian(oldPauseGuardian, newPauseGuardian);
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

/* solhint-disable gas-custom-errors */ // Cannot use custom errors with 0.7.6

/**
 * @title Graph Governance contract
 * @author Edge & Node
 * @notice All contracts that will be owned by a Governor entity should extend this contract.
 */
abstract contract Governed {
    // -- State --

    /**
     * @notice Address of the governor
     */
    address public governor;
    /**
     * @notice Address of the new governor that is pending acceptance
     */
    address public pendingGovernor;

    // -- Events --

    /**
     * @notice Emitted when a new owner/governor has been set, but is pending acceptance
     * @param from Previous pending governor address
     * @param to New pending governor address
     */
    event NewPendingOwnership(address indexed from, address indexed to);

    /**
     * @notice Emitted when a new owner/governor has accepted their role
     * @param from Previous governor address
     * @param to New governor address
     */
    event NewOwnership(address indexed from, address indexed to);

    /**
     * @dev Check if the caller is the governor.
     */
    modifier onlyGovernor() {
        require(msg.sender == governor, "Only Governor can call");
        _;
    }

    /**
     * @notice Initialize the governor for this contract
     * @param _initGovernor Address of the governor
     */
    function _initialize(address _initGovernor) internal {
        governor = _initGovernor;
    }

    /**
     * @notice Admin function to begin change of governor. The `_newGovernor` must call
     * `acceptOwnership` to finalize the transfer.
     * @param _newGovernor Address of new `governor`
     */
    function transferOwnership(address _newGovernor) external onlyGovernor {
        require(_newGovernor != address(0), "Governor must be set");

        address oldPendingGovernor = pendingGovernor;
        pendingGovernor = _newGovernor;

        emit NewPendingOwnership(oldPendingGovernor, pendingGovernor);
    }

    /**
     * @notice Admin function for pending governor to accept role and update governor.
     * This function must called by the pending governor.
     */
    function acceptOwnership() external {
        address oldPendingGovernor = pendingGovernor;

        require(
            oldPendingGovernor != address(0) && msg.sender == oldPendingGovernor,
            "Caller must be pending governor"
        );

        address oldGovernor = governor;

        governor = oldPendingGovernor;
        pendingGovernor = address(0);

        emit NewOwnership(oldGovernor, governor);
        emit NewPendingOwnership(oldPendingGovernor, pendingGovernor);
    }
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


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
pragma experimental ABIEncoderV2;

interface IERC165 {
    /// @notice Query if a contract implements an interface
    /// @param interfaceId The interface identifier, as specified in ERC-165
    /// @dev Interface identification is specified in ERC-165. This function
    ///  uses less than 30,000 gas.
    /// @return `true` if the contract implements `interfaceID` and
    ///  `interfaceID` is not 0xffffffff, `false` otherwise
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;
pragma experimental ABIEncoderV2;

/******************************************************************************\
* Author: Nick Mudge <nick@perfectabstractions.com> (https://twitter.com/mudgen)
* EIP-2535 Diamond Standard: https://eips.ethereum.org/EIPS/eip-2535
/******************************************************************************/

import "../libraries/LibDiamond.sol";
import "../interfaces/IDiamondCut.sol";
import "../interfaces/IDiamondLoupe.sol";
import "../interfaces/IERC165.sol";

contract DiamondLoupeFacet is IDiamondLoupe, IERC165 {
    // Diamond Loupe Functions
    ////////////////////////////////////////////////////////////////////
    /// These functions are expected to be called frequently by tools.
    //
    // struct Facet {
    //     address facetAddress;
    //     bytes4[] functionSelectors;
    // }

    /// @notice Gets all facets and their selectors.
    /// @return facets_ Facet
    function facets() external override view returns (Facet[] memory facets_) {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        uint256 numFacets = ds.facetAddresses.length;
        facets_ = new Facet[](numFacets);
        for (uint256 i; i < numFacets; i++) {
            address facetAddress_ = ds.facetAddresses[i];
            facets_[i].facetAddress = facetAddress_;
            facets_[i].functionSelectors = ds.facetFunctionSelectors[facetAddress_].functionSelectors;
        }
    }

    /// @notice Gets all the function selectors provided by a facet.
    /// @param _facet The facet address.
    /// @return facetFunctionSelectors_
    function facetFunctionSelectors(address _facet) external override view returns (bytes4[] memory facetFunctionSelectors_) {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        facetFunctionSelectors_ = ds.facetFunctionSelectors[_facet].functionSelectors;
    }

    /// @notice Get all the facet addresses used by a diamond.
    /// @return facetAddresses_
    function facetAddresses() external override view returns (address[] memory facetAddresses_) {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        facetAddresses_ = ds.facetAddresses;
    }

    /// @notice Gets the facet that supports the given selector.
    /// @dev If facet is not found return address(0).
    /// @param _functionSelector The function selector.
    /// @return facetAddress_ The facet address.
    function facetAddress(bytes4 _functionSelector) external override view returns (address facetAddress_) {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        facetAddress_ = ds.selectorToFacetAndPosition[_functionSelector].facetAddress;
    }

    // This implements ERC-165.
    function supportsInterface(bytes4 _interfaceId) external override view returns (bool) {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        return ds.supportedInterfaces[_interfaceId];
    }
}


## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

