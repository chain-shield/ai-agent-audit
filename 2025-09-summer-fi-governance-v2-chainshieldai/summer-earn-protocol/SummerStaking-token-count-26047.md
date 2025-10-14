
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IStakedSummerToken} from "../interfaces/IStakedSummerToken.sol";
import {StakingRewardsManagerBase} from "@summerfi/rewards-contracts/contracts/StakingRewardsManagerBase.sol";
import {WrappedStakingToken} from "./WrappedStakingToken.sol";
import {Constants} from "@summerfi/constants/Constants.sol";
import {ConfigurationManaged} from "@summerfi/earn-protocol-contracts/contracts/ConfigurationManaged.sol";
import {UD60x18, ud60x18, convert} from "@prb/math/src/UD60x18.sol";
import {IStakingRewardsManagerBase} from "@summerfi/rewards-contracts/interfaces/IStakingRewardsManagerBase.sol";
import {ISummerStaking} from "../interfaces/ISummerStaking.sol";
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";

/**
 * @title SummerStaking
 * @notice Enhanced staking with lockups (0–3y), weighted rewards, and bucket caps. Users stake SUMR and receive xSUMR
 *         1:1 while rewards accrue on weighted balances computed via a quadratic time multiplier.
 *
 * @dev Architecture and invariants:
 *      - Index 0 of each portfolio aggregates all no-lockup stake; positions with lockup > 0 occupy indices > 0.
 *      - Weighted supply drives rewards accounting: `totalSupply` in base manager is the weighted sum.
 *      - Early unstake penalty: fixed 2% if remaining < FIXED_PENALTY_PERIOD, else linear up to 20% at 3 years.
 *      - Token flows: stake pulls SUMR, wraps internally, and mints xSUMR; unstake burns xSUMR, unwraps, and splits
 *        penalty to treasury.
 *      - Access control: governor manages bucket caps and penalty enablement; xSUMR roles managed on the token.
 *      - Reentrancy: public mutating entrypoints are nonReentrant and updateRewards for correct accounting.
 *
 *      Buckets & Caps (capacity control):
 *      - Each possible lockup duration maps to a discrete Bucket enum via `_findBucket(_lockupPeriod)`:
 *          • NoLockup:        0 seconds (min=0, max=0)
 *          • ShortTerm:       [1 second, 14 days]
 *          • TwoWeeksToThreeMonths: (14 days, 90 days]
 *          • ThreeToSixMonths:      (90 days, 180 days]
 *          • SixToTwelveMonths:     (180 days, 365 days]
 *          • OneToTwoYears:         (365 days, 730 days]
 *          • TwoToThreeYears:       (730 days, 1095 days]
 *
 *      - Bucket caps throttle the total raw SUMR that can be staked in each bucket. They are applied on the
 *        unweighted amount (plain token units), not the weighted amount used for rewards accounting.
 *        Cap semantics:
 *          • cap == 0                → bucket is disabled (any positive stake reverts with Staking_BucketCapExceeded)
 *          • cap == type(uint256).max → bucket is unlimited
 *          • 0 < cap < max           → currentRawStaked + amount must be <= cap
 */
contract SummerStaking is
    StakingRewardsManagerBase,
    ConfigurationManaged,
    ISummerStaking
{
    using SafeERC20 for IStakedSummerToken;
    using SafeERC20 for IERC20;
    using SafeERC20 for IERC20;

    // ============ IMMUTABLE STATE ============

    IERC20 public immutable SUMMER_TOKEN;
    IStakedSummerToken public immutable STAKED_SUMMER_TOKEN;
    WrappedStakingToken public immutable WRAPPED_SUMMER_TOKEN;

    // ============ CONSTANTS ============

    uint256 public constant MAX_LOCKUP_PERIOD = 3 * 365 days;
    uint256 public constant MAX_AMOUNT_OF_STAKES = 1000;
    uint256 public constant MIN_PENALTY_PERCENTAGE = 0.02e18; // 2%
    uint256 public constant MAX_PENALTY_PERCENTAGE = 0.2e18; // 20%
    uint256 public constant FIXED_PENALTY_PERIOD = 110 days;

    uint256 public constant WEIGHTED_STAKE_BASE = Constants.WAD; // 1 in 60.18 fixed-point
    uint256 public constant WEIGHTED_STAKE_COEFFICIENT = 700; // 7e-16 * 1e18 in 60.18 fixed-point

    uint256 public constant NO_LOCKUP_INDEX = 0;
    uint256 public constant BUCKET_SHORT_TERM_MIN = 1;
    uint256 public constant BUCKET_SHORT_TERM_MAX = 14 days;
    uint256 public constant BUCKET_TWO_WEEKS_TO_THREE_MONTHS_MAX = 90 days;
    uint256 public constant BUCKET_THREE_TO_SIX_MAX = 180 days;
    uint256 public constant BUCKET_SIX_TO_TWELVE_MAX = 365 days;
    uint256 public constant BUCKET_ONE_TO_TWO_MAX = 2 * 365 days;
    uint256 public constant BUCKET_TWO_TO_THREE_MAX = MAX_LOCKUP_PERIOD;

    // ============ STORAGE ============

    mapping(address owner => UserStake[] stakes) public stakesByOwner;

    mapping(address owner => uint256 weightedBalance) public weightedBalances;
    mapping(Bucket bucketId => BucketData bucketData) public bucketData;
    bool public penaltyEnabled = true;

    // ============ CONSTRUCTOR ============

    constructor(
        address _protocolAccessManager,
        address _configurationManager,
        address _summerToken,
        address _stakedSummerToken
    )
        StakingRewardsManagerBase(_protocolAccessManager)
        ConfigurationManaged(_configurationManager)
    {
        if (_summerToken == address(0)) {
            revert Staking_InvalidAddress(
                "Summer token address cannot be zero"
            );
        }
        if (_stakedSummerToken == address(0)) {
            revert Staking_InvalidAddress(
                "StakedSummerToken address cannot be zero"
            );
        }

        SUMMER_TOKEN = IERC20(_summerToken);
        STAKED_SUMMER_TOKEN = IStakedSummerToken(_stakedSummerToken);
        WRAPPED_SUMMER_TOKEN = new WrappedStakingToken(_summerToken);
        stakingToken = _summerToken;
    }

    // ============ EXTERNAL FUNCTIONS - STAKING ============

    ///  @inheritdoc ISummerStaking
    function stakeLockup(
        uint256 _amount,
        uint256 _lockupPeriod
    ) external nonReentrant {
        _stakeLockup(_msgSender(), _msgSender(), _amount, _lockupPeriod);
    }

    ///  @inheritdoc ISummerStaking
    function stakeLockupOnBehalf(
        address _receiver,
        uint256 _amount,
        uint256 _lockupPeriod
    ) external nonReentrant {
        _stakeLockup(_msgSender(), _receiver, _amount, _lockupPeriod);
    }

    // ============ EXTERNAL FUNCTIONS - UNSTAKING ============

    ///  @inheritdoc ISummerStaking
    function unstakeLockup(
        uint256 _stakeIndex,
        uint256 _amount
    ) external virtual updateReward(_msgSender()) nonReentrant {
        // Validate amount and availability before reading stake
        if (_amount == 0) revert Staking_InvalidAmount("Amount cannot be zero");
        if (_amount > _balances[_msgSender()])
            revert Staking_InsufficientBalance();
        UserStake[] storage stakes = stakesByOwner[_msgSender()];
        if (_stakeIndex >= stakes.length)
            revert Staking_InvalidStakeIndex("Stake index out of bounds");

        // Copy stake to memory for mutation and proportional computations
        UserStake memory processedStake = stakes[_stakeIndex];
        if (processedStake.amount < _amount)
            revert Staking_InvalidStakeIndex(
                "Stake amount is less than unstake amount"
            );

        // Compute penalty and the weighted share we need to remove proportionally
        uint256 unstakePenalty = calculatePenalty(
            _msgSender(),
            _amount,
            _stakeIndex
        );
        // No overflow for any realistic amounts (e.g., 1e9 total supply with 18 decimals has >20 orders
        // of magnitude headroom). Proportional division truncates on partial unstakes; the final full exit
        // clears the remaining weighted exactly (no residual).
        uint256 weightedAmountToRemove = (processedStake.weightedAmount *
            _amount) / processedStake.amount;

        // Mutate local stake and persist back to storage (or pop if fully consumed and not index 0)
        processedStake.amount -= _amount;
        processedStake.weightedAmount -= weightedAmountToRemove;

        _updateBalancesOnUnstake(_msgSender(), _amount, weightedAmountToRemove);
        _subtractFromBucketTotal(processedStake.lockupPeriod, _amount);

        if (processedStake.amount == 0 && !_isNoLockupStakeIndex(_stakeIndex)) {
            _removeStake(stakes, _stakeIndex);
        } else {
            stakes[_stakeIndex] = processedStake;
        }

        // Perform token movements, including penalty routing to treasury if applicable
        _handleTokenTransfersOnUnstake(_msgSender(), _amount, unstakePenalty);

        emit UnstakedWithPenalty(
            _msgSender(),
            _stakeIndex,
            _amount,
            unstakePenalty,
            _amount - unstakePenalty
        );
        emit Unstaked(_msgSender(), _msgSender(), _amount);
    }

    // ============ EXTERNAL FUNCTIONS - ADMIN ============
    ///  @inheritdoc ISummerStaking
    function updateLockupBucketCap(
        Bucket _bucket,
        uint256 _newCap
    ) external onlyGovernor {
        // Governor may set to 0 (disabled) or max (no cap). Intermediate caps throttle bucket utilization.
        bucketData[_bucket].cap = _newCap;
        emit LockupBucketUpdated(_bucket, _newCap);
    }

    ///  @inheritdoc ISummerStaking
    function updatePenaltyEnabled(bool _penaltyEnabled) external onlyGovernor {
        // Toggling penalties is a risk lever; when disabled, early exits incur no treasury fee
        penaltyEnabled = _penaltyEnabled;
        emit PenaltyEnabledUpdated(_penaltyEnabled);
    }

    ///  @inheritdoc ISummerStaking
    function rescueToken(address _token, address _to) external onlyGovernor {
        if (_token == address(WRAPPED_SUMMER_TOKEN)) {
            revert Staking_InvalidAddress("Cannot rescue wrapped summer token");
        }
        // Sweep entire token balance to the target; used for emergency recovery only
        IERC20(_token).safeTransfer(
            _to,
            IERC20(_token).balanceOf(address(this))
        );
    }

    // ============ EXTERNAL VIEW FUNCTIONS - STAKE INFORMATION ============

    function getUserStakesCount(address _user) external view returns (uint256) {
        return stakesByOwner[_user].length;
    }

    ///  @inheritdoc ISummerStaking
    function getUserStake(
        address _user,
        uint256 _index
    )
        external
        view
        returns (
            uint256 amount,
            uint256 weightedAmount,
            uint256 lockupEndTime,
            uint256 lockupPeriod
        )
    {
        // Return zeroed tuple if index out-of-bounds
        UserStake[] storage stakes = stakesByOwner[_user];
        if (_index < stakes.length) {
            amount = stakes[_index].amount;
            weightedAmount = stakes[_index].weightedAmount;
            lockupEndTime = stakes[_index].lockupEndTime;
            lockupPeriod = stakes[_index].lockupPeriod;
        }
    }

    ///  @inheritdoc ISummerStaking
    function weightedBalanceOf(
        address account
    ) external view virtual returns (uint256) {
        return weightedBalances[account];
    }

    // ============ EXTERNAL VIEW FUNCTIONS - BUCKET INFORMATION ============

    ///  @inheritdoc ISummerStaking
    function getBucketTotalStaked(
        Bucket _bucket
    ) external view returns (uint256) {
        return bucketData[_bucket].staked;
    }

    ///  @inheritdoc ISummerStaking
    function getBucketDetails(
        Bucket _bucket
    )
        external
        view
        returns (
            uint256 cap,
            uint256 staked,
            uint256 minLockupPeriod,
            uint256 maxLockupPeriod
        )
    {
        (cap, staked, minLockupPeriod, maxLockupPeriod) = _getBucketDetails(
            _bucket
        );
    }

    ///  @inheritdoc ISummerStaking
    function getAllBucketInfo()
        external
        view
        returns (
            Bucket[] memory buckets,
            uint256[] memory caps,
            uint256[] memory stakedAmounts,
            uint256[] memory minPeriods,
            uint256[] memory maxPeriods
        )
    {
        buckets = new Bucket[](7);
        caps = new uint256[](7);
        stakedAmounts = new uint256[](7);
        minPeriods = new uint256[](7);
        maxPeriods = new uint256[](7);

        buckets[0] = Bucket.NoLockup;
        buckets[1] = Bucket.ShortTerm;
        buckets[2] = Bucket.TwoWeeksToThreeMonths;
        buckets[3] = Bucket.ThreeToSixMonths;
        buckets[4] = Bucket.SixToTwelveMonths;
        buckets[5] = Bucket.OneToTwoYears;
        buckets[6] = Bucket.TwoToThreeYears;

        for (uint256 i = 0; i < 7; i++) {
            (
                caps[i],
                stakedAmounts[i],
                minPeriods[i],
                maxPeriods[i]
            ) = _getBucketDetails(buckets[i]);
        }
    }

    // ============ EXTERNAL VIEW FUNCTIONS - PENALTY CALCULATIONS ============

    ///  @inheritdoc ISummerStaking
    function calculatePenaltyPercentage(
        address _user,
        uint256 _stakeIndex
    ) public view returns (uint256) {
        // If penalties are globally disabled, early exits are free
        if (!penaltyEnabled) {
            return 0;
        }
        // Load stake; caller must pass a valid index (public functions ensure bounds)
        UserStake storage userStake = stakesByOwner[_user][_stakeIndex];

        // No penalty if lockup has already expired
        if (block.timestamp >= userStake.lockupEndTime) {
            return 0;
        }

        // Near-expiry fixed penalty floor to avoid cliff at zero
        uint256 timeRemaining = userStake.lockupEndTime - block.timestamp;
        if (timeRemaining < FIXED_PENALTY_PERIOD) {
            return MIN_PENALTY_PERCENTAGE;
        }
        // Linear ramp to MAX_PENALTY_PERCENTAGE at MAX_LOCKUP_PERIOD
        return (timeRemaining * MAX_PENALTY_PERCENTAGE) / MAX_LOCKUP_PERIOD;
    }

    ///  @inheritdoc ISummerStaking
    function calculatePenalty(
        address _user,
        uint256 _amount,
        uint256 _stakeIndex
    ) public view returns (uint256) {
        uint256 penaltyPercentage = calculatePenaltyPercentage(
            _user,
            _stakeIndex
        );
        return (penaltyPercentage * _amount) / Constants.WAD;
    }

    // ============ EXTERNAL PURE FUNCTIONS - WEIGHTED STAKE CALCULATIONS ============

    ///  @inheritdoc ISummerStaking
    function calculateWeightedStake(
        uint256 _amount,
        uint256 _lockupPeriod
    ) public pure returns (uint256) {
        return _calculateWeightedStake(_amount, _lockupPeriod);
    }

    // ============ PUBLIC OVERRIDE FUNCTIONS - REWARDS ============

    ///  @inheritdoc IStakingRewardsManagerBase
    function earned(
        address account,
        address rewardToken
    )
        public
        view
        override(StakingRewardsManagerBase, IStakingRewardsManagerBase)
        returns (uint256)
    {
        uint256 weightedBalance = weightedBalances[account];
        if (weightedBalance == 0) {
            return rewards[rewardToken][account];
        }

        return
            (weightedBalance *
                (rewardPerToken(rewardToken) -
                    userRewardPerTokenPaid[rewardToken][account])) /
            Constants.WAD +
            rewards[rewardToken][account];
    }

    // ============ PUBLIC OVERRIDE FUNCTIONS - DISABLED FUNCTIONS ============
    ///  @inheritdoc IStakingRewardsManagerBase
    function stakeOnBehalfOf(
        address,
        uint256
    ) external pure override(IStakingRewardsManagerBase) {
        revert StakeOnBehalfOfNotSupported();
    }

    ///  @inheritdoc IStakingRewardsManagerBase
    function unstakeAndWithdrawOnBehalfOf(
        address,
        uint256,
        bool
    ) external pure override(IStakingRewardsManagerBase) {
        revert UnstakeOnBehalfOfNotSupported();
    }

    ///  @inheritdoc IStakingRewardsManagerBase
    function stake(
        uint256
    )
        external
        virtual
        override(StakingRewardsManagerBase, IStakingRewardsManagerBase)
    {
        revert Staking_DirectStakeNotAllowed("Use stakeLockup instead");
    }

    ///  @inheritdoc IStakingRewardsManagerBase
    function unstake(
        uint256
    )
        external
        virtual
        override(StakingRewardsManagerBase, IStakingRewardsManagerBase)
    {
        revert Staking_DirectUnstakeNotAllowed("Use unstakeLockup instead");
    }

    ///  @inheritdoc IStakingRewardsManagerBase
    function exit()
        external
        pure
        override(StakingRewardsManagerBase, IStakingRewardsManagerBase)
    {
        revert Staking_DirectUnstakeNotAllowed("Use unstakeLockup instead");
    }

    // ============ INTERNAL FUNCTIONS - STAKING LOGIC ============

    /**
     * @notice Internal staking entrypoint that validates inputs, computes weighted amounts, updates accounting
     *         and emits stake events.
     * @dev Emits both `Staked` (from base manager) and `StakedWithLockup` (extended) events. Creates or updates
     *      the no-lockup aggregate stake at index 0 when `_lockupPeriod == 0`, otherwise appends a new stake.
     *      Reverts on invalid inputs, exceeding caps, or reaching per-user stake limit. Updates both raw and
     *      weighted balances and bucket totals, and performs token transfers/minting.
     * @param _from Address providing SUMR tokens (debited via transferFrom)
     * @param _receiver Address that receives the stake and xSUMR
     * @param _amount SUMR amount to stake (must be > 0)
     * @param _lockupPeriod Lockup duration in seconds (0..MAX_LOCKUP_PERIOD)
     * @custom:reverts Staking_InvalidAddress if `_from` or `_receiver` is zero
     * @custom:reverts Staking_InvalidAmount if `_amount == 0`
     * @custom:reverts Staking_InvalidLockupPeriod if `_lockupPeriod > MAX_LOCKUP_PERIOD`
     * @custom:reverts Staking_MaxStakesReached if user already has `MAX_AMOUNT_OF_STAKES`
     * @custom:reverts Staking_BucketCapExceeded if staking would exceed bucket cap
     */
    function _stakeLockup(
        address _from,
        address _receiver,
        uint256 _amount,
        uint256 _lockupPeriod
    ) internal updateReward(_receiver) {
        // Validate addresses and parameters up-front to fail fast
        if (_receiver == address(0))
            revert Staking_InvalidAddress("Target address cannot be zero");
        if (_from == address(0))
            revert Staking_InvalidAddress("Sender address cannot be zero");
        if (_amount == 0) revert Staking_InvalidAmount("Amount cannot be zero");
        if (_lockupPeriod > MAX_LOCKUP_PERIOD) {
            revert Staking_InvalidLockupPeriod(
                "Lockup period cannot exceed 3 years"
            );
        }
        // Enforce per-portfolio stake count bound and bucket caps on raw amount
        if (stakesByOwner[_receiver].length >= MAX_AMOUNT_OF_STAKES) {
            revert Staking_MaxStakesReached();
        }
        if (_wouldExceedBucketCap(_lockupPeriod, _amount)) {
            revert Staking_BucketCapExceeded();
        }

        // Precompute weighted amount: amount * (1 + k * t^2) in UD60x18 fixed-point
        uint256 weightedAmount = _calculateWeightedStake(
            _amount,
            _lockupPeriod
        );
        UserStake[] storage _stakePortfolio = _ensurePortfolio(_receiver);

        uint256 _stakeIndex;
        if (_lockupPeriod == 0) {
            // Aggregate no-lockup at index 0 to save storage slots and simplify exits
            UserStake storage noLockupStake = _noLockupStake(_stakePortfolio);
            noLockupStake.amount += _amount;
            noLockupStake.weightedAmount += weightedAmount;
            noLockupStake.lockupEndTime = block.timestamp;
            _stakeIndex = NO_LOCKUP_INDEX;
        } else {
            // Append an independent lockup position
            _stakePortfolio.push(
                UserStake({
                    amount: _amount,
                    weightedAmount: weightedAmount,
                    lockupEndTime: block.timestamp + _lockupPeriod,
                    lockupPeriod: _lockupPeriod
                })
            );
            _stakeIndex = _stakePortfolio.length - 1;
        }
        // Update balances and bucket totals, then move tokens and mint xSUMR
        _updateBalancesOnStake(_receiver, _amount, weightedAmount);
        _addToBucketTotal(_lockupPeriod, _amount);
        _handleTokenTransfersOnStake(_from, _receiver, _amount);

        emit Staked(_from, _receiver, _amount);
        emit StakedWithLockup(
            _receiver,
            _stakeIndex,
            _amount,
            _lockupPeriod,
            weightedAmount
        );
    }

    // ============ INTERNAL FUNCTIONS - BUCKET MANAGEMENT ============
    /**
     * @notice Resolves the `Bucket` for a given lockup period.
     * @param _lockupPeriod Lockup duration in seconds
     * @return bucket The resolved bucket enum
     * @custom:reverts Staking_InvalidLockupPeriod if `_lockupPeriod` exceeds maximum allowed
     */
    function _findBucket(uint256 _lockupPeriod) internal pure returns (Bucket) {
        // Map the lockup duration to a discrete risk bucket; 0 is a dedicated no-lockup bucket
        if (_lockupPeriod == 0) return Bucket.NoLockup;
        if (_lockupPeriod <= BUCKET_SHORT_TERM_MAX) return Bucket.ShortTerm;
        if (_lockupPeriod <= BUCKET_TWO_WEEKS_TO_THREE_MONTHS_MAX)
            return Bucket.TwoWeeksToThreeMonths;
        if (_lockupPeriod <= BUCKET_THREE_TO_SIX_MAX)
            return Bucket.ThreeToSixMonths;
        if (_lockupPeriod <= BUCKET_SIX_TO_TWELVE_MAX)
            return Bucket.SixToTwelveMonths;
        if (_lockupPeriod <= BUCKET_ONE_TO_TWO_MAX) return Bucket.OneToTwoYears;
        if (_lockupPeriod <= BUCKET_TWO_TO_THREE_MAX)
            return Bucket.TwoToThreeYears;
        revert Staking_InvalidLockupPeriod(
            "Lockup period exceeds maximum allowed"
        );
    }
    /**
     * @notice Increases the total staked amount for the bucket of the given lockup period.
     * @param _lockupPeriod Lockup duration in seconds
     * @param _amount Raw amount to add to the bucket total
     */
    function _addToBucketTotal(
        uint256 _lockupPeriod,
        uint256 _amount
    ) internal {
        // Increase current bucket raw staked total; used for cap enforcement and telemetry
        Bucket bucket = _findBucket(_lockupPeriod);
        bucketData[bucket].staked += _amount;
    }
    /**
     * @notice Decreases the total staked amount for the bucket of the given lockup period.
     * @param _lockupPeriod Lockup duration in seconds
     * @param _amount Raw amount to subtract from the bucket total
     */
    function _subtractFromBucketTotal(
        uint256 _lockupPeriod,
        uint256 _amount
    ) internal {
        // Decrease current bucket raw staked total on exits (or partial exits)
        Bucket bucket = _findBucket(_lockupPeriod);
        bucketData[bucket].staked -= _amount;
    }
    /**
     * @notice Returns cap, current staked total, and min/max lockup bounds for a bucket.
     * @param _bucket The bucket to query
     * @return cap The staking cap for the bucket (0 = disabled, max = unlimited)
     * @return staked The current total raw amount staked in the bucket
     * @return minLockupPeriod Minimum lockup period in seconds for the bucket
     * @return maxLockupPeriod Maximum lockup period in seconds for the bucket
     */
    function _getBucketDetails(
        Bucket _bucket
    )
        internal
        view
        returns (
            uint256 cap,
            uint256 staked,
            uint256 minLockupPeriod,
            uint256 maxLockupPeriod
        )
    {
        cap = bucketData[_bucket].cap;
        staked = bucketData[_bucket].staked;

        if (_bucket == Bucket.NoLockup) {
            minLockupPeriod = 0;
            maxLockupPeriod = 0;
        } else if (_bucket == Bucket.ShortTerm) {
            minLockupPeriod = BUCKET_SHORT_TERM_MIN;
            maxLockupPeriod = BUCKET_SHORT_TERM_MAX;
        } else if (_bucket == Bucket.TwoWeeksToThreeMonths) {
            minLockupPeriod = BUCKET_SHORT_TERM_MAX + 1;
            maxLockupPeriod = BUCKET_TWO_WEEKS_TO_THREE_MONTHS_MAX;
        } else if (_bucket == Bucket.ThreeToSixMonths) {
            minLockupPeriod = BUCKET_TWO_WEEKS_TO_THREE_MONTHS_MAX + 1;
            maxLockupPeriod = BUCKET_THREE_TO_SIX_MAX;
        } else if (_bucket == Bucket.SixToTwelveMonths) {
            minLockupPeriod = BUCKET_THREE_TO_SIX_MAX + 1;
            maxLockupPeriod = BUCKET_SIX_TO_TWELVE_MAX;
        } else if (_bucket == Bucket.OneToTwoYears) {
            minLockupPeriod = BUCKET_SIX_TO_TWELVE_MAX + 1;
            maxLockupPeriod = BUCKET_ONE_TO_TWO_MAX;
        } else if (_bucket == Bucket.TwoToThreeYears) {
            minLockupPeriod = BUCKET_ONE_TO_TWO_MAX + 1;
            maxLockupPeriod = BUCKET_TWO_TO_THREE_MAX;
        }
    }
    /**
     * @notice Checks whether staking `_amount` with `_lockupPeriod` would exceed the bucket cap.
     * @param _lockupPeriod Lockup duration in seconds
     * @param _amount Raw amount to test
     * @return wouldExceed True if current bucket total + amount would exceed cap
     */
    function _wouldExceedBucketCap(
        uint256 _lockupPeriod,
        uint256 _amount
    ) internal view returns (bool) {
        // Compute `current + amount > cap` with cap==0 treated as disabled (always exceed)
        Bucket bucket = _findBucket(_lockupPeriod);
        uint256 currentBucketTotal = bucketData[bucket].staked;
        uint256 bucketCap = bucketData[bucket].cap;
        return (currentBucketTotal + _amount) > bucketCap;
    }

    // ============ INTERNAL FUNCTIONS - WEIGHTED STAKE CALCULATIONS ============
    /**
     * @notice Calculates the weighted stake used for rewards accounting.
     * @dev Uses 60.18 fixed-point arithmetic: weighted = amount * (WEIGHTED_STAKE_BASE + WEIGHTED_STAKE_COEFFICIENT * t^2)
     *      where t is `_lockupPeriod` seconds. Constants: BASE=1e18, COEFFICIENT=700 (i.e., 7e-16 scaled to 60.18).
     * @param _amount Raw stake amount
     * @param _lockupPeriod Lockup duration in seconds
     * @return weightedAmount The weighted amount applied to rewards `totalSupply`
     */
    function _calculateWeightedStake(
        uint256 _amount,
        uint256 _lockupPeriod
    ) internal pure returns (uint256) {
        // Convert lockup seconds to UD60x18 and square for quadratic multiplier
        UD60x18 time = convert(_lockupPeriod);
        UD60x18 timeSquared = time.mul(time);

        // multiplier = BASE + COEFFICIENT * t^2 (scaled math); then multiply by raw amount
        UD60x18 multiplier = ud60x18(WEIGHTED_STAKE_COEFFICIENT)
            .mul(timeSquared)
            .add(ud60x18(WEIGHTED_STAKE_BASE));

        return ud60x18(_amount).mul(multiplier).unwrap();
    }

    // ============ INTERNAL FUNCTIONS - TOKEN TRANSFERS ============
    /**
     * @notice Handles token flows for stake: pull SUMR from `from`, wrap into `WRAPPED_SUMMER_TOKEN`, mint xSUMR to `receiver`.
     * @dev Uses SafeERC20 for transfers and forceApprove to set allowance for the wrapper.
     * @param from Source address providing SUMR via `transferFrom`
     * @param receiver Recipient of newly minted xSUMR
     * @param amount Amount of SUMR to move and mint 1:1 as xSUMR
     */
    function _handleTokenTransfersOnStake(
        address from,
        address receiver,
        uint amount
    ) internal {
        // Pull SUMR from the staker and approve the wrapper for exact amount
        SUMMER_TOKEN.safeTransferFrom(from, address(this), amount);
        SUMMER_TOKEN.forceApprove(address(WRAPPED_SUMMER_TOKEN), amount);
        // Wrap into internal accounting token and mint xSUMR 1:1 to the receiver
        WRAPPED_SUMMER_TOKEN.depositFor(address(this), amount);
        STAKED_SUMMER_TOKEN.mint(receiver, amount);
    }
    /**
     * @notice Handles token flows for unstake: withdraw wrapped SUMR,
     * @notice send net to `receiver`, penalty to `treasury`, and burn xSUMR.
     * @dev If `unstakePenalty == 0`, withdraw directly to receiver; otherwise withdraw to this contract, split net and penalty.
     * @param receiver Receiver of the unstaked SUMR net of penalty
     * @param amount Amount of SUMR being unstaked
     * @param unstakePenalty Penalty amount in SUMR sent to `treasury()`
     */
    function _handleTokenTransfersOnUnstake(
        address receiver,
        uint amount,
        uint unstakePenalty
    ) internal {
        if (unstakePenalty > 0) {
            // Withdraw wrapped SUMR to this contract, then split between user and treasury
            WRAPPED_SUMMER_TOKEN.withdrawTo(address(this), amount);
            SUMMER_TOKEN.safeTransfer(receiver, amount - unstakePenalty);
            SUMMER_TOKEN.safeTransfer(treasury(), unstakePenalty);
        } else {
            // Gas-optimal path: withdraw directly to the receiver if no penalty is due
            WRAPPED_SUMMER_TOKEN.withdrawTo(receiver, amount);
        }
        // Burn xSUMR from the receiver to maintain 1:1 accounting with SUMR backing
        STAKED_SUMMER_TOKEN.burnFrom(receiver, amount);
    }

    // ============ INTERNAL FUNCTIONS - BALANCE MANAGEMENT ============
    /**
     * @notice Updates raw and weighted balances and weighted total supply during stake.
     * @param _receiver Receiver whose balances are increased
     * @param _amount Raw amount added
     * @param _weightedAmount Weighted amount added to rewards `totalSupply`
     */
    function _updateBalancesOnStake(
        address _receiver,
        uint256 _amount,
        uint256 _weightedAmount
    ) internal {
        // Raw SUMR staking balance (used for xSUMR mint/burn) and weighted balance for rewards
        _balances[_receiver] += _amount;
        weightedBalances[_receiver] += _weightedAmount;
        totalSupply += _weightedAmount;
    }
    /**
     * @notice Updates raw and weighted balances and weighted total supply during unstake.
     * @param _receiver Receiver whose balances are decreased
     * @param _amount Raw amount removed
     * @param _weightedAmount Weighted amount removed from rewards `totalSupply`
     */
    function _updateBalancesOnUnstake(
        address _receiver,
        uint256 _amount,
        uint256 _weightedAmount
    ) internal {
        // Mirror updates from stake but in reverse; maintain total weighted supply invariant
        _balances[_receiver] -= _amount;
        weightedBalances[_receiver] -= _weightedAmount;
        totalSupply -= _weightedAmount;
    }

    /**
     * @notice Removes a stake at `_index` from a user's portfolio using swap-and-pop.
     * @dev Assumes caller validated `_index` bounds. This is only used for non-aggregate stakes (index > 0).
     * @param _stakes The stakes to remove from
     * @param _index The index to remove (0-based)
     */
    function _removeStake(
        UserStake[] storage _stakes,
        uint256 _index
    ) internal {
        _stakes[_index] = _stakes[_stakes.length - 1];
        _stakes.pop();
    }
    // ============ INTERNAL FUNCTIONS - INDEX HELPERS ============
    /**
     * @notice Check if the index is the no lockup index
     * @param index The index to check
     * @return True if the index is the no lockup index, false otherwise
     */
    function _isNoLockupStakeIndex(uint256 index) internal pure returns (bool) {
        return index == NO_LOCKUP_INDEX;
    }
    /**
     * @notice Get the no lockup stake for a given portfolio
     * @param portfolio The portfolio to get the no lockup stake for
     * @return The no lockup stake for the portfolio
     */
    function _noLockupStake(
        UserStake[] storage portfolio
    ) internal view returns (UserStake storage) {
        return portfolio[NO_LOCKUP_INDEX];
    }

    // ============ INTERNAL - ID HELPERS ============

    /**
     * @notice Ensure a portfolio for a given owner address
     * @param owner The address to ensure an id for
     * @return The portfolio for the address
     */
    function _ensurePortfolio(
        address owner
    ) internal returns (UserStake[] storage) {
        if (stakesByOwner[owner].length == 0) {
            stakesByOwner[owner].push(
                UserStake({
                    amount: 0,
                    weightedAmount: 0,
                    lockupEndTime: block.timestamp,
                    lockupPeriod: 0
                })
            );
        }
        return stakesByOwner[owner];
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {IConfigurationManagerErrors} from "../errors/IConfigurationManagerErrors.sol";
import {IConfigurationManagerEvents} from "../events/IConfigurationManagerEvents.sol";
import {ConfigurationManagerParams} from "../types/ConfigurationManagerTypes.sol";

/**
 * @title IConfigurationManager
 * @notice Interface for the ConfigurationManager contract, which manages system-wide parameters
 * @dev This interface defines the getters and setters for system-wide parameters
 */

interface IConfigurationManager is
    IConfigurationManagerEvents,
    IConfigurationManagerErrors
{
    /**
     * @notice Initialize the configuration with the given parameters
     * @param params The parameters to initialize the configuration with
     * @dev Can only be called by the governor
     */
    function initializeConfiguration(
        ConfigurationManagerParams memory params
    ) external;

    /**
     * @notice Get the address of the Raft contract
     * @return The address of the Raft contract
     * @dev This is where rewards and farmed tokens are sent for processing
     */
    function raft() external view returns (address);

    /**
     * @notice Get the current tip jar address
     * @return The current tip jar address
     * @dev This is the contract that owns tips and is responsible for
     *     dispensing them to claimants
     */
    function tipJar() external view returns (address);

    /**
     * @notice Get the current treasury address
     * @return The current treasury address
     *       @dev This is the contract that owns the treasury and is responsible for
     *      dispensing funds to the protocol's operations
     */
    function treasury() external view returns (address);

    /**
     * @notice Get the address of theHarbor command
     * @return The address of theHarbor command
     * @dev This is the contract that's the registry of all Fleet Commanders
     */
    function harborCommand() external view returns (address);

    /**
     * @notice Get the address of the Fleet Commander Rewards Manager Factory contract
     * @return The address of the Fleet Commander Rewards Manager Factory contract
     */
    function fleetCommanderRewardsManagerFactory()
        external
        view
        returns (address);

    /**
     * @notice Set a new address for the Raft contract
     * @param newRaft The new address for the Raft contract
     * @dev Can only be called by the governor
     */
    function setRaft(address newRaft) external;

    /**
     * @notice Set a new tip ar address
     * @param newTipJar The address of the new tip jar
     * @dev Can only be called by the governor
     */
    function setTipJar(address newTipJar) external;

    /**
     * @notice Set a new treasury address
     * @param newTreasury The address of the new treasury
     * @dev Can only be called by the governor
     */
    function setTreasury(address newTreasury) external;

    /**
     * @notice Set a new harbor command address
     * @param newHarborCommand The address of the new harbor command
     * @dev Can only be called by the governor
     */
    function setHarborCommand(address newHarborCommand) external;

    /**
     * @notice Set a new fleet commander rewards manager factory address
     * @param newFleetCommanderRewardsManagerFactory The address of the new fleet commander rewards manager factory
     * @dev Can only be called by the governor
     */
    function setFleetCommanderRewardsManagerFactory(
        address newFleetCommanderRewardsManagerFactory
    ) external;
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

/**
 * @title StakingRewardsManager
 * @notice Contract for managing staking rewards with multiple reward tokens in the Summer protocol
 * @dev Implements IStakingRewards interface and inherits from ReentrancyGuardTransient and ProtocolAccessManaged
 * @dev Inspired by Synthetix's StakingRewards contract:
 * https://github.com/Synthetixio/synthetix/blob/v2.101.3/contracts/StakingRewards.sol
 */
import {IStakingRewardsManagerBase} from "../interfaces/IStakingRewardsManagerBase.sol";
import {ProtocolAccessManaged} from "@summerfi/access-contracts/contracts/ProtocolAccessManaged.sol";
import {ReentrancyGuardTransient} from "@summerfi/dependencies/openzeppelin-next/ReentrancyGuardTransient.sol";
import {IERC20, SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/interfaces/IERC20Metadata.sol";
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {Constants} from "@summerfi/constants/Constants.sol";
import {ERC20Wrapper} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Wrapper.sol";

/**
 * @title StakingRewards
 * @notice Contract for managing staking rewards with multiple reward tokens in the Summer protocol
 * @dev Implements IStakingRewards interface and inherits from ReentrancyGuardTransient and ProtocolAccessManaged
 */
abstract contract StakingRewardsManagerBase is
    IStakingRewardsManagerBase,
    ReentrancyGuardTransient,
    ProtocolAccessManaged
{
    using SafeERC20 for IERC20;
    using EnumerableSet for EnumerableSet.AddressSet;

    struct RewardData {
        uint256 periodFinish;
        uint256 rewardRate;
        uint256 rewardsDuration;
        uint256 lastUpdateTime;
        uint256 rewardPerTokenStored;
    }

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /* @notice List of all reward tokens supported by this contract */
    EnumerableSet.AddressSet internal _rewardTokensList;
    /* @notice The token that users stake to earn rewards */
    address public immutable stakingToken;

    /* @notice Mapping of reward token to its reward distribution data */
    mapping(address rewardToken => RewardData data) public rewardData;
    /* @notice Tracks the last reward per token paid to each user for each reward token */
    mapping(address rewardToken => mapping(address account => uint256 rewardPerTokenPaid))
        public userRewardPerTokenPaid;
    /* @notice Tracks the unclaimed rewards for each user for each reward token */
    mapping(address rewardToken => mapping(address account => uint256 rewardAmount))
        public rewards;

    /* @notice Total amount of tokens staked in the contract */
    uint256 public totalSupply;
    mapping(address account => uint256 balance) internal _balances;

    uint256 private constant MAX_REWARD_DURATION = 360 days; // 1 year

    /*//////////////////////////////////////////////////////////////
                                MODIFIERS
    //////////////////////////////////////////////////////////////*/

    modifier updateReward(address account) virtual {
        _updateReward(account);
        _;
    }

    /*//////////////////////////////////////////////////////////////
                                CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initializes the StakingRewards contract
     * @param accessManager The address of the access manager
     */
    constructor(address accessManager) ProtocolAccessManaged(accessManager) {}

    /*//////////////////////////////////////////////////////////////
                                VIEWS
    //////////////////////////////////////////////////////////////*/

    /// @inheritdoc IStakingRewardsManagerBase
    function rewardTokens(
        uint256 index
    ) external view override returns (address) {
        if (index >= _rewardTokensList.length()) revert IndexOutOfBounds();
        address rewardTokenAddress = _rewardTokensList.at(index);
        return rewardTokenAddress;
    }

    /// @inheritdoc IStakingRewardsManagerBase
    function rewardTokensLength() external view returns (uint256) {
        return _rewardTokensList.length();
    }

    /// @inheritdoc IStakingRewardsManagerBase
    function balanceOf(address account) public view virtual returns (uint256) {
        return _balances[account];
    }

    /// @inheritdoc IStakingRewardsManagerBase
    function lastTimeRewardApplicable(
        address rewardToken
    ) public view returns (uint256) {
        return
            block.timestamp < rewardData[rewardToken].periodFinish
                ? block.timestamp
                : rewardData[rewardToken].periodFinish;
    }

    /// @inheritdoc IStakingRewardsManagerBase
    function rewardPerToken(address rewardToken) public view returns (uint256) {
        if (totalSupply == 0) {
            return rewardData[rewardToken].rewardPerTokenStored;
        }
        return
            rewardData[rewardToken].rewardPerTokenStored +
            ((lastTimeRewardApplicable(rewardToken) -
                rewardData[rewardToken].lastUpdateTime) *
                rewardData[rewardToken].rewardRate) /
            totalSupply;
    }

    /// @inheritdoc IStakingRewardsManagerBase
    function earned(
        address account,
        address rewardToken
    ) public view virtual returns (uint256) {
        return _earned(account, rewardToken);
    }

    /// @inheritdoc IStakingRewardsManagerBase
    function getRewardForDuration(
        address rewardToken
    ) external view returns (uint256) {
        RewardData storage data = rewardData[rewardToken];
        if (block.timestamp >= data.periodFinish) {
            return (data.rewardRate * data.rewardsDuration) / Constants.WAD;
        }
        // For active periods, calculate remaining rewards plus any new rewards
        uint256 remaining = data.periodFinish - block.timestamp;
        return (data.rewardRate * remaining) / Constants.WAD;
    }

    /// @inheritdoc IStakingRewardsManagerBase
    function isRewardToken(address rewardToken) external view returns (bool) {
        return _isRewardToken(rewardToken);
    }

    /*//////////////////////////////////////////////////////////////
                            MUTATIVE FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @inheritdoc IStakingRewardsManagerBase
    function stake(uint256 amount) external virtual updateReward(_msgSender()) {
        _stake(_msgSender(), _msgSender(), amount);
    }

    /// @inheritdoc IStakingRewardsManagerBase
    function unstake(
        uint256 amount
    ) external virtual updateReward(_msgSender()) {
        _unstake(_msgSender(), _msgSender(), amount);
    }

    /// @inheritdoc IStakingRewardsManagerBase
    function getReward() public virtual nonReentrant {
        uint256 rewardTokenCount = _rewardTokensList.length();
        for (uint256 i = 0; i < rewardTokenCount; i++) {
            address rewardTokenAddress = _rewardTokensList.at(i);
            _getReward(_msgSender(), rewardTokenAddress);
        }
    }

    /// @inheritdoc IStakingRewardsManagerBase
    function getReward(address rewardToken) public virtual nonReentrant {
        if (!_isRewardToken(rewardToken)) revert RewardTokenDoesNotExist();
        _getReward(_msgSender(), rewardToken);
    }

    /// @inheritdoc IStakingRewardsManagerBase
    function exit() external virtual {
        getReward();
        _unstake(_msgSender(), _msgSender(), _balances[_msgSender()]);
    }

    /// @notice Claims rewards for a specific account
    /// @param account The address to claim rewards for
    function getRewardFor(address account) public virtual nonReentrant {
        uint256 rewardTokenCount = _rewardTokensList.length();
        for (uint256 i = 0; i < rewardTokenCount; i++) {
            address rewardTokenAddress = _rewardTokensList.at(i);
            _getReward(account, rewardTokenAddress);
        }
    }

    /// @notice Claims rewards for a specific account and specific reward token
    /// @param account The address to claim rewards for
    /// @param rewardToken The address of the reward token to claim
    function getRewardFor(
        address account,
        address rewardToken
    ) public virtual nonReentrant {
        if (!_isRewardToken(rewardToken)) revert RewardTokenDoesNotExist();
        _getReward(account, rewardToken);
    }

    /*//////////////////////////////////////////////////////////////
                            RESTRICTED FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @inheritdoc IStakingRewardsManagerBase
    function notifyRewardAmount(
        address rewardToken,
        uint256 reward,
        uint256 newRewardsDuration
    ) external virtual onlyGovernor updateReward(address(0)) {
        _notifyRewardAmount(rewardToken, reward, newRewardsDuration);
    }

    /// @inheritdoc IStakingRewardsManagerBase
    function setRewardsDuration(
        address rewardToken,
        uint256 _rewardsDuration
    ) external onlyGovernor {
        if (!_isRewardToken(rewardToken)) {
            revert RewardTokenDoesNotExist();
        }
        if (_rewardsDuration == 0) {
            revert RewardsDurationCannotBeZero();
        }
        if (_rewardsDuration > MAX_REWARD_DURATION) {
            revert RewardsDurationTooLong();
        }

        RewardData storage data = rewardData[rewardToken];
        if (block.timestamp <= data.periodFinish) {
            revert RewardPeriodNotComplete();
        }
        data.rewardsDuration = _rewardsDuration;
        emit RewardsDurationUpdated(address(rewardToken), _rewardsDuration);
    }

    /// @notice Removes a reward token from the list of reward tokens
    /// @param rewardToken The address of the reward token to remove
    function removeRewardToken(address rewardToken) external onlyGovernor {
        if (!_isRewardToken(rewardToken)) {
            revert RewardTokenDoesNotExist();
        }

        if (block.timestamp <= rewardData[rewardToken].periodFinish) {
            revert RewardPeriodNotComplete();
        }

        // Check if all tokens have been claimed, allowing a small dust balance
        uint256 remainingBalance = IERC20(rewardToken).balanceOf(address(this));
        uint256 dustThreshold;

        try IERC20Metadata(address(rewardToken)).decimals() returns (
            uint8 decimals
        ) {
            // For tokens with 4 or fewer decimals, use a minimum threshold of 1
            // For tokens with more decimals, use 0.01% of 1 token
            if (decimals <= 4) {
                dustThreshold = 1;
            } else {
                dustThreshold = 10 ** (decimals - 4); // 0.01% of 1 token
            }
        } catch {
            dustThreshold = 1e14; // Default threshold for tokens without decimals
        }

        if (remainingBalance > dustThreshold) {
            revert RewardTokenStillHasBalance(remainingBalance);
        }

        // Remove the token from the rewardTokens map
        bool success = _rewardTokensList.remove(address(rewardToken));
        if (!success) revert RewardTokenDoesNotExist();

        emit RewardTokenRemoved(address(rewardToken));
    }

    /*//////////////////////////////////////////////////////////////
                            INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    function _isRewardToken(address rewardToken) internal view returns (bool) {
        return _rewardTokensList.contains(rewardToken);
    }

    function _stake(
        address staker,
        address receiver,
        uint256 amount
    ) internal virtual {
        if (receiver == address(0)) revert CannotStakeToZeroAddress();
        if (amount == 0) revert CannotStakeZero();
        if (address(stakingToken) == address(0)) {
            revert StakingTokenNotInitialized();
        }
        totalSupply += amount;
        _balances[receiver] += amount;
        IERC20(stakingToken).safeTransferFrom(staker, address(this), amount);
        emit Staked(staker, receiver, amount);
    }

    function _unstake(
        address staker,
        address receiver,
        uint256 amount
    ) internal virtual {
        if (amount == 0) revert CannotUnstakeZero();
        totalSupply -= amount;
        _balances[staker] -= amount;
        IERC20(stakingToken).safeTransfer(receiver, amount);
        emit Unstaked(staker, receiver, amount);
    }

    /*
     * @notice Internal function to calculate earned rewards for an account
     * @param account The address to calculate earnings for
     * @param rewardToken The reward token to calculate earnings for
     * @return The amount of reward tokens earned
     */
    function _earned(
        address account,
        address rewardToken
    ) internal view returns (uint256) {
        return
            (_balances[account] *
                (rewardPerToken(rewardToken) -
                    userRewardPerTokenPaid[rewardToken][account])) /
            Constants.WAD +
            rewards[rewardToken][account];
    }

    function _updateReward(address account) internal {
        uint256 rewardTokenCount = _rewardTokensList.length();
        for (uint256 i = 0; i < rewardTokenCount; i++) {
            address rewardTokenAddress = _rewardTokensList.at(i);
            RewardData storage rewardTokenData = rewardData[rewardTokenAddress];
            rewardTokenData.rewardPerTokenStored = rewardPerToken(
                rewardTokenAddress
            );
            rewardTokenData.lastUpdateTime = lastTimeRewardApplicable(
                rewardTokenAddress
            );
            if (account != address(0)) {
                rewards[rewardTokenAddress][account] = earned(
                    account,
                    rewardTokenAddress
                );
                userRewardPerTokenPaid[rewardTokenAddress][
                    account
                ] = rewardTokenData.rewardPerTokenStored;
            }
        }
    }

    /**
     * @notice Internal function to claim rewards for an account for a specific token
     * @param account The address to claim rewards for
     * @param rewardTokenAddress The address of the reward token to claim
     * @dev rewards go straight to the user's wallet
     */
    function _getReward(
        address account,
        address rewardTokenAddress
    ) internal virtual updateReward(account) {
        uint256 reward = rewards[rewardTokenAddress][account];
        if (reward > 0) {
            rewards[rewardTokenAddress][account] = 0;
            IERC20(rewardTokenAddress).safeTransfer(account, reward);
            emit RewardPaid(account, rewardTokenAddress, reward);
        }
    }

    /**
     * @dev Internal implementation of notifyRewardAmount
     * @param rewardToken The token to distribute as rewards
     * @param reward The amount of reward tokens to distribute
     * @param newRewardsDuration The duration for new reward tokens (only used for first time)
     */
    function _notifyRewardAmount(
        address rewardToken,
        uint256 reward,
        uint256 newRewardsDuration
    ) internal {
        RewardData storage rewardTokenData = rewardData[rewardToken];
        if (newRewardsDuration == 0) {
            revert RewardsDurationCannotBeZero();
        }

        if (newRewardsDuration > MAX_REWARD_DURATION) {
            revert RewardsDurationTooLong();
        }

        // For existing reward tokens, check if current period is complete
        if (_isRewardToken(rewardToken)) {
            if (newRewardsDuration != rewardTokenData.rewardsDuration) {
                revert CannotChangeRewardsDuration();
            }
        } else {
            // First time setup for new reward token
            bool success = _rewardTokensList.add(rewardToken);
            if (!success) revert RewardTokenAlreadyExists();

            rewardTokenData.rewardsDuration = newRewardsDuration;
            emit RewardTokenAdded(rewardToken, rewardTokenData.rewardsDuration);
        }

        // Transfer exact amount needed for new rewards
        IERC20(rewardToken).safeTransferFrom(msg.sender, address(this), reward);

        // Calculate new reward rate
        rewardTokenData.rewardRate =
            (reward * Constants.WAD) /
            rewardTokenData.rewardsDuration;
        rewardTokenData.lastUpdateTime = block.timestamp;
        rewardTokenData.periodFinish =
            block.timestamp +
            rewardTokenData.rewardsDuration;

        emit RewardAdded(address(rewardToken), reward);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

library Constants {
    // WAD: Common unit, stands for "18 decimals"
    uint256 public constant WAD = 1e18;

    // RAY: Higher precision unit, "27 decimals"
    uint256 public constant RAY = 1e27;

    // Conversion factor from WAD to RAY
    uint256 public constant WAD_TO_RAY = 1e9;

    // Number of seconds in a day
    uint256 public constant SECONDS_PER_DAY = 1 days;

    // Number of seconds in a year (assuming 365 days)
    uint256 public constant SECONDS_PER_YEAR = 365 days;

    // Maximum value for uint256
    uint256 public constant MAX_UINT256 = type(uint256).max;

    // AAVE V3 POOL CONFIG DATA MASK

    uint256 internal constant ACTIVE_MASK =
        0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFFFFFFFFFF;
    uint256 internal constant FROZEN_MASK =
        0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFDFFFFFFFFFFFFFF;
    uint256 internal constant PAUSED_MASK =
        0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFFFFFFFFFFF;
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

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {IConfigurationManager} from "./IConfigurationManager.sol";

/**
 * @title IConfigurationManaged
 * @notice Interface for contracts that need to read from the ConfigurationManager
 * @dev This interface defines the standard methods for accessing configuration values
 *      from the ConfigurationManager. It should be implemented by contracts that
 *      need to read these configurations.
 */
interface IConfigurationManaged {
    /**
     * @notice Gets the address of the ConfigurationManager contract
     * @return The address of the ConfigurationManager contract
     */
    function configurationManager()
        external
        view
        returns (IConfigurationManager);

    /**
     * @notice Gets the address of the Raft contract
     * @return The address of the Raft contract
     */
    function raft() external view returns (address);

    /**
     * @notice Gets the address of the TipJar contract
     * @return The address of the TipJar contract
     */
    function tipJar() external view returns (address);

    /**
     * @notice Gets the address of the Treasury contract
     * @return The address of the Treasury contract
     */
    function treasury() external view returns (address);

    /**
     * @notice Gets the address of the HarborCommand contract
     * @return The address of the HarborCommand contract
     */
    function harborCommand() external view returns (address);

    /**
     * @notice Gets the address of the Fleet Commander Rewards Manager Factory contract
     * @return The address of the Fleet Commander Rewards Manager Factory contract
     */
    function fleetCommanderRewardsManagerFactory()
        external
        view
        returns (address);

    error ConfigurationManagerZeroAddress();
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

/**
 * @title IConfigurationManagerErrors
 * @dev This file contains custom error definitions for the ConfigurationManager contract.
 * @notice These custom errors provide more gas-efficient and informative error handling
 * compared to traditional require statements with string messages.
 */
interface IConfigurationManagerErrors {
    /**
     * @notice Thrown when an operation is attempted with a zero address where a non-zero address is required.
     */
    error ZeroAddress();
    /**
     * @notice Thrown when ConfigurationManager was already initialized.
     */
    error ConfigurationManagerAlreadyInitialized();

    /**
     * @notice Thrown when the Raft address is not set.
     */
    error RaftNotSet();

    /**
     * @notice Thrown when the TipJar address is not set.
     */
    error TipJarNotSet();

    /**
     * @notice Thrown when the Treasury address is not set.
     */
    error TreasuryNotSet();

    /**
     * @notice Thrown when constructor address is set to the zero address.
     */
    error AddressZero();

    /**
     * @notice Thrown when the HarborCommand address is not set.
     */
    error HarborCommandNotSet();
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

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {ERC20Wrapper} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Wrapper.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/**
 * @title WrappedStakingToken
 * @notice A simple wrapper for the staking token that inherits from ERC20Wrapper
 * @dev This contract is used by GovernanceRewardsManager to wrap staking tokens when they are used as rewards
 */
contract WrappedStakingToken is ERC20Wrapper {
    constructor(
        address underlyingToken
    )
        ERC20(string.concat("Wrapped ", "Summer"), string.concat("w", "SUMR"))
        ERC20Wrapper(IERC20(underlyingToken))
    {}
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

/**
 * @title IConfigurationManagerEvents
 * @notice Interface for events emitted by the Configuration Manager
 */
interface IConfigurationManagerEvents {
    /**
     * @notice Emitted when the Raft address is updated
     * @param newRaft The address of the new Raft
     */
    event RaftUpdated(address oldRaft, address newRaft);

    /**
     * @notice Emitted when the tip jar address is updated
     * @param newTipJar The address of the new tip jar
     */
    event TipJarUpdated(address oldTipJar, address newTipJar);

    /**
     * @notice Emitted when the tip rate is updated
     * @param newTipRate The new tip rate value
     */
    event TipRateUpdated(uint8 oldTipRate, uint8 newTipRate);

    /**
     * @notice Emitted when the Treasury address is updated
     * @param newTreasury The address of the new Treasury
     */
    event TreasuryUpdated(address oldTreasury, address newTreasury);

    /**
     * @notice Emitted when the Harbor Command address is updated
     * @param oldHarborCommand The address of the old Harbor Command
     * @param newHarborCommand The address of the new Harbor Command
     */
    event HarborCommandUpdated(
        address oldHarborCommand,
        address newHarborCommand
    );

    /**
     * @notice Emitted when the Fleet Commander Rewards Manager Factory address is updated
     * @param oldFleetCommanderRewardsManagerFactory The address of the old Fleet Commander Rewards Manager Factory
     * @param newFleetCommanderRewardsManagerFactory The address of the new Fleet Commander Rewards Manager Factory
     */
    event FleetCommanderRewardsManagerFactoryUpdated(
        address oldFleetCommanderRewardsManagerFactory,
        address newFleetCommanderRewardsManagerFactory
    );
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IStakingRewardsManagerBaseErrors} from "./IStakingRewardsManagerBaseErrors.sol";

/* @title IStakingRewardsManagerBase
 * @notice Interface for the Staking Rewards Manager contract
 * @dev Manages staking and distribution of multiple reward tokens
 */
interface IStakingRewardsManagerBase is IStakingRewardsManagerBaseErrors {
    // Views

    /* @notice Get the total amount of staked tokens
     * @return The total supply of staked tokens
     */
    function totalSupply() external view returns (uint256);

    /* @notice Get the staked balance of a specific account
     * @param account The address of the account to check
     * @return The staked balance of the account
     */
    function balanceOf(address account) external view returns (uint256);

    /* @notice Get the last time the reward was applicable for a specific reward token
     * @param rewardToken The address of the reward token
     * @return The timestamp of the last applicable reward time
     */
    function lastTimeRewardApplicable(
        address rewardToken
    ) external view returns (uint256);

    /* @notice Get the reward per token for a specific reward token
     * @param rewardToken The address of the reward token
     * @return The reward amount per staked token (WAD-scaled)
     * @dev Returns a WAD-scaled value (1e18) to maintain precision in calculations
     * @dev This value represents: (rewardRate * timeElapsed * WAD) / totalSupply
     */
    function rewardPerToken(
        address rewardToken
    ) external view returns (uint256);

    /* @notice Calculate the earned reward for an account and a specific reward token
     * @param account The address of the account
     * @param rewardToken The address of the reward token
     * @return The amount of reward tokens earned (not WAD-scaled)
     * @dev Calculated as: (balance * (rewardPerToken - userRewardPerTokenPaid)) / WAD + rewards
     */
    function earned(
        address account,
        address rewardToken
    ) external view returns (uint256);

    /* @notice Get the reward for the entire duration for a specific reward token
     * @param rewardToken The address of the reward token
     * @return The total reward amount for the duration (not WAD-scaled)
     * @dev Calculated as: (rewardRate * rewardsDuration) / WAD
     */
    function getRewardForDuration(
        address rewardToken
    ) external view returns (uint256);

    /* @notice Get the address of the staking token
     * @return The address of the staking token
     */
    function stakingToken() external view returns (address);

    /* @notice Get the reward token at a specific index
     * @param index The index of the reward token
     * @return The address of the reward token
     * @dev Reverts with IndexOutOfBounds if index >= rewardTokensLength()
     */
    function rewardTokens(uint256 index) external view returns (address);

    /* @notice Get the total number of reward tokens
     * @return The length of the reward tokens list
     */
    function rewardTokensLength() external view returns (uint256);

    /* @notice Check if a token is in the list of reward tokens
     * @param rewardToken The address to check
     * @return bool True if the token is a reward token, false otherwise
     */
    function isRewardToken(address rewardToken) external view returns (bool);

    // Mutative functions

    /* @notice Stake tokens for an account
     * @param amount The amount of tokens to stake
     */
    function stake(uint256 amount) external;

    /* @notice Stake tokens for an account on behalf of another account
     * @param receiver The address of the account to stake for
     * @param amount The amount of tokens to stake
     */
    function stakeOnBehalfOf(address receiver, uint256 amount) external;

    /* @notice Unstake staked tokens on behalf of another account
     * @param owner The address of the account to unstake from
     * @param amount The amount of tokens to unstake
     * @param claimRewards Whether to claim rewards before unstaking
     */
    function unstakeAndWithdrawOnBehalfOf(
        address owner,
        uint256 amount,
        bool claimRewards
    ) external;

    /* @notice Unstake staked tokens
     * @param amount The amount of tokens to unstake
     */
    function unstake(uint256 amount) external;

    /* @notice Claim accumulated rewards for all reward tokens */
    function getReward() external;

    /* @notice Claim accumulated rewards for a specific reward token
     * @param rewardToken The address of the reward token to claim
     */
    function getReward(address rewardToken) external;

    /* @notice Withdraw all staked tokens and claim rewards */
    function exit() external;

    // Admin functions

    /* @notice Notify the contract about new reward amount
     * @param rewardToken The address of the reward token
     * @param reward The amount of new reward (not WAD-scaled)
     * @param newRewardsDuration The duration for rewards distribution (only used when adding a new reward token)
     * @dev Internally sets rewardRate as (reward * WAD) / duration to maintain precision
     */
    function notifyRewardAmount(
        address rewardToken,
        uint256 reward,
        uint256 newRewardsDuration
    ) external;

    /* @notice Set the duration for rewards distribution
     * @param rewardToken The address of the reward token
     * @param _rewardsDuration The new duration for rewards
     */
    function setRewardsDuration(
        address rewardToken,
        uint256 _rewardsDuration
    ) external;

    /* @notice Removes a reward token from the list of reward tokens
     * @dev Can only be called by governor
     * @dev Can only be called after reward period is complete
     * @dev Can only be called if remaining balance is below dust threshold
     * @param rewardToken The address of the reward token to remove
     */
    function removeRewardToken(address rewardToken) external;

    // Events

    /* @notice Emitted when a new reward is added
     * @param rewardToken The address of the reward token
     * @param reward The amount of reward added
     */
    event RewardAdded(address indexed rewardToken, uint256 reward);

    /* @notice Emitted when tokens are staked
     * @param staker The address that provided the tokens for staking
     * @param receiver The address whose staking balance was updated
     * @param amount The amount of tokens added to the staking position
     */
    event Staked(
        address indexed staker,
        address indexed receiver,
        uint256 amount
    );

    /* @notice Emitted when tokens are unstaked
     * @param staker The address whose tokens were unstaked
     * @param receiver The address receiving the unstaked tokens
     * @param amount The amount of tokens unstaked
     */
    event Unstaked(
        address indexed staker,
        address indexed receiver,
        uint256 amount
    );

    /* @notice Emitted when tokens are withdrawn
     * @param user The address of the user that withdrew
     * @param amount The amount of tokens withdrawn
     */
    event Withdrawn(address indexed user, uint256 amount);

    /* @notice Emitted when rewards are paid out
     * @param user The address of the user receiving the reward
     * @param rewardToken The address of the reward token
     * @param reward The amount of reward paid
     */
    event RewardPaid(
        address indexed user,
        address indexed rewardToken,
        uint256 reward
    );

    /* @notice Emitted when the rewards duration is updated
     * @param rewardToken The address of the reward token
     * @param newDuration The new duration for rewards
     */
    event RewardsDurationUpdated(
        address indexed rewardToken,
        uint256 newDuration
    );

    /* @notice Emitted when a new reward token is added
     * @param rewardToken The address of the new reward token
     * @param rewardsDuration The duration for the new reward token
     */
    event RewardTokenAdded(address rewardToken, uint256 rewardsDuration);

    /* @notice Emitted when a reward token is removed
     * @param rewardToken The address of the reward token
     */
    event RewardTokenRemoved(address rewardToken);

    /* @notice Claims rewards for a specific account
     * @param account The address to claim rewards for
     */
    function getRewardFor(address account) external;

    /* @notice Claims rewards for a specific account and specific reward token
     * @param account The address to claim rewards for
     * @param rewardToken The address of the reward token to claim
     */
    function getRewardFor(address account, address rewardToken) external;
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {IStakingRewardsManagerBase} from "@summerfi/rewards-contracts/interfaces/IStakingRewardsManagerBase.sol";

/**
 * @title ISummerStaking
 * @notice Interface for the staking module used by Governance v2. Users stake SUMR with optional lockups (0–3y),
 *         receiving non-transferable xSUMR 1:1 for governance while rewards are accounted on the weighted balance.
 * @dev Design highlights and invariants:
 *      - One aggregated NoLockup stake is always at index 0 for a portfolio (created lazily on first stake).
 *      - Lockups > 0 live at indices > 0. A user may have up to MAX_AMOUNT_OF_STAKES (see implementation constant).
 *      - Rewards are distributed on weighted balances; base `totalSupply` in the rewards manager tracks weighted sum.
 *      - Bucket caps are governor-controlled and can be 0 (disabled) or type(uint256).max (uncapped).
 *      - Early unstake penalties are forwarded to protocol treasury; no burn of SUMR occurs.
 *      - xSUMR mint/burn is delegated to approved staking modules; governance manages module roles on xSUMR.
 * @author Summer.fi Protocol
 */
interface ISummerStaking is IStakingRewardsManagerBase {
    // ============ ENUMS ============

    /**
     * @notice Lockup buckets categorizing stakes by lockup period duration
     * @dev Each bucket has configurable caps and different lockup period ranges
     */
    enum Bucket {
        NoLockup, // 0 days - immediate withdrawal with no lockup
        ShortTerm, // 1second - 14 days - disabled by default (cap = 0)
        TwoWeeksToThreeMonths, // <90 days - 2 weeks-3 months lockup period
        ThreeToSixMonths, // <180 days - 3-6 months lockup period
        SixToTwelveMonths, // <365 days - 6-12 month lockup period
        OneToTwoYears, // <730 days - 1-2 years lockup period
        TwoToThreeYears // <1095 days - 2-3 years lockup period
    }

    // ============ STRUCTS ============
    /**
     * @notice Structure representing a bucket's data
     * @param cap The maximum amount that can be staked in this bucket
     * @param staked The current total amount staked in this bucket
     */
    struct BucketData {
        uint256 cap;
        uint256 staked;
    }
    /**
     * @dev User stake element stored per portfolio. Implementation reserves index 0 for the no-lockup aggregate.
     *      For indices > 0, each element corresponds to an independent lockup position with its own end time.
     *      `weightedAmount` is precomputed to avoid recalculating the quadratic multiplier on every read.
     */
    /**
     * @notice Structure representing a user's individual stake with lockup details
     * @param amount The actual amount of tokens staked
     * @param weightedAmount The weighted amount used for reward calculations (amount * multiplier)
     * @param lockupEndTime Timestamp when the lockup period ends
     * @param lockupPeriod Original lockup period duration in seconds
     */
    struct UserStake {
        uint256 amount;
        uint256 weightedAmount;
        uint256 lockupEndTime;
        uint256 lockupPeriod;
    }

    // ============ STAKING FUNCTIONS ============

    /**
     * @notice Stake SUMMER tokens with a specified lockup period
     * @param _amount The amount of SUMMER tokens to stake (must be > 0)
     * @param _lockupPeriod The lockup period in seconds (0 to 3 years max)
     * @dev Creates a new stake with weighted amount calculated based on lockup period
     * @dev Transfers SUMMER tokens from caller and mints equivalent xSUMR tokens
     * @dev Weighted amount formula: amount * (1 + 7e-16 * lockupPeriod^2) using 60.18 fixed-point
     * @dev Emits StakedWithLockup and Staked events
     * @dev Reverts if amount is 0, lockup period exceeds max, or bucket cap exceeded
     */
    function stakeLockup(uint256 _amount, uint256 _lockupPeriod) external;

    /**
     * @notice Stake SUMMER tokens with lockup period on behalf of another address
     * @param _receiver The address that will receive the stake and xSUMR tokens
     * @param _amount The amount of SUMMER tokens to stake (must be > 0)
     * @param _lockupPeriod The lockup period in seconds (0 to 3 years max)
     * @dev SUMMER tokens are transferred from caller, but stake and xSUMR go to receiver
     * @dev Useful for protocol-level staking or delegation scenarios
     * @dev Same validation and mechanics as stakeLockup()
     */
    function stakeLockupOnBehalf(
        address _receiver,
        uint256 _amount,
        uint256 _lockupPeriod
    ) external;

    // ============ UNSTAKING FUNCTIONS ============

    /**
     * @notice Unstake tokens from a specific stake position with penalty calculation
     * @param _stakeIndex The index of the stake to unstake from (0-based)
     * @param _amount The amount of tokens to unstake (must be > 0 and <= stake amount)
     * @dev can only be called by the wallet that owns the stake, there is no onBehalf version due to the penalty
     * @dev Applies penalty for early withdrawal based on remaining lockup time
     * @dev Penalty formula: penalty% = 2% if remaining time < 110 days otherwise (timeRemaining / maxLockupPeriod) * 20%
     * @dev Examples:
     *      - 3yr lockup, immediate unstake: 20% penalty
     *      - 3yr lockup, unstake after 1.5yr: 10% penalty
     *      - 1yr lockup, immediate unstake: ~6.67% penalty
     *      - After lockup ends: 0% penalty
     *      - 0 lockup, immediate unstake: 0% penalty
     *      - any lockup time, <110 days remaining: 2% penalty
     * @dev Penalties are sent to protocol treasury
     * @dev Emits UnstakedWithPenalty and Unstaked events
     * @dev Reverts if amount is 0, stake index invalid, or insufficient balance
     */
    function unstakeLockup(uint256 _stakeIndex, uint256 _amount) external;

    // ============ VIEW FUNCTIONS - STAKE INFORMATION ============

    /**
     * @notice Get the number of stakes for a specific user
     * @param _user The address to check stake count for
     * @return The total number of stakes (max 1000 per user)
     */
    function getUserStakesCount(address _user) external view returns (uint256);

    /**
     * @notice Get detailed information about a specific user stake
     * @param _user The address of the stake owner
     * @param _index The index of the stake to query (0-based)
     * @return amount The actual staked token amount
     * @return weightedAmount The weighted amount used for reward calculations
     * @return lockupEndTime Timestamp when lockup period ends
     * @return lockupPeriod Original lockup duration in seconds
     * @dev Returns zeros if index is out of bounds
     */
    function getUserStake(
        address _user,
        uint256 _index
    )
        external
        view
        returns (
            uint256 amount,
            uint256 weightedAmount,
            uint256 lockupEndTime,
            uint256 lockupPeriod
        );

    /**
     * @notice Get the weighted balance of an account for reward calculations
     * @param account The address to check weighted balance for
     * @return The total weighted balance (sum of all weighted stakes)
     * @dev This is the balance used for reward distribution calculations
     * @dev Different from balanceOf() which returns actual staked amount
     */
    function weightedBalanceOf(address account) external view returns (uint256);

    // ============ VIEW FUNCTIONS - PENALTY CALCULATIONS ============

    /**
     * @notice Calculate the penalty percentage for early unstaking
     * @param _user The address of the stake owner
     * @param _stakeIndex The index of the stake to calculate penalty for
     * @return The penalty percentage in WAD format (18 decimals)
     * @dev Returns 0 if lockup period has ended
     * @dev Formula: (timeRemaining / maxLockupPeriod) * maxPenalty
     * @dev maxPenalty = 2% floor < 110 days, else 20% (0.2e18), maxLockupPeriod = 3 years
     * @dev penaltyDisabled == true then 0
     */
    function calculatePenaltyPercentage(
        address _user,
        uint256 _stakeIndex
    ) external view returns (uint256);

    /**
     * @notice Calculate the penalty amount for unstaking a specific amount
     * @param _user The address of the stake owner
     * @param _amount The amount of tokens to unstake
     * @param _stakeIndex The index of the stake
     * @return The penalty amount in token units
     * @dev Penalty = (penaltyPercentage * amount) / WAD
     */
    function calculatePenalty(
        address _user,
        uint256 _amount,
        uint256 _stakeIndex
    ) external view returns (uint256);

    // ============ VIEW FUNCTIONS - WEIGHTED STAKE CALCULATIONS ============

    /**
     * @notice Calculate the weighted stake amount for a given amount and lockup period
     * @param _amount The base amount to stake
     * @param _lockupPeriod The lockup period in seconds
     * @return The weighted stake amount for reward calculations
     * @dev Formula: amount * (1 + 7e-16 * lockupPeriod^2) using 60.18 fixed-point
     * @dev Longer lockups result in higher weighted amounts and more rewards
     * @dev For 0 lockup period, multiplier is 1.0 (no boost)
     */
    function calculateWeightedStake(
        uint256 _amount,
        uint256 _lockupPeriod
    ) external pure returns (uint256);

    // ============ VIEW FUNCTIONS - BUCKET MANAGEMENT ============

    /**
     * @notice Get the total staked amount for a specific lockup bucket
     * @param _bucket The bucket to check (NoLockup, ShortTerm, etc.)
     * @return The total amount staked in this bucket across all users
     */
    function getBucketTotalStaked(
        Bucket _bucket
    ) external view returns (uint256);

    /**
     * @notice Get comprehensive details about a specific lockup bucket
     * @param _bucket The bucket to query
     * @return cap The maximum amount that can be staked in this bucket
     * @return staked The current total amount staked in this bucket
     * @return minLockupPeriod The minimum lockup period for this bucket
     * @return maxLockupPeriod The maximum lockup period for this bucket
     * @dev cap = 0 means bucket is disabled, type(uint256).max means no cap
     */
    function getBucketDetails(
        Bucket _bucket
    )
        external
        view
        returns (
            uint256 cap,
            uint256 staked,
            uint256 minLockupPeriod,
            uint256 maxLockupPeriod
        );

    /**
     * @notice Get information about all lockup buckets at once
     * @return buckets Array of all bucket enums
     * @return caps Array of bucket caps (0 = disabled, max = no cap)
     * @return stakedAmounts Array of current staked amounts per bucket
     * @return minPeriods Array of minimum lockup periods per bucket
     * @return maxPeriods Array of maximum lockup periods per bucket
     * @dev Arrays are ordered: [NoLockup, ShortTerm, TwoWeeksToThreeMonths, ThreeToSixMonths, SixToTwelveMonths, OneToTwoYears, TwoToThreeYears]
     */
    function getAllBucketInfo()
        external
        view
        returns (
            Bucket[] memory buckets,
            uint256[] memory caps,
            uint256[] memory stakedAmounts,
            uint256[] memory minPeriods,
            uint256[] memory maxPeriods
        );

    // ============ ADMIN FUNCTIONS ============

    /**
     * @notice Update the staking cap for a specific lockup bucket
     * @param _bucket The bucket to update
     * @param _newCap The new cap amount (0 = disabled, type(uint256).max = no cap)
     * @dev Only callable by protocol governor
     * @dev Used to manage protocol risk and control staking distribution
     * @dev Emits LockupBucketUpdated event
     */
    function updateLockupBucketCap(Bucket _bucket, uint256 _newCap) external;

    /**
     * @notice Update the penalty enabled status
     * @param _penaltyEnabled The new penalty enabled status
     * @dev Only callable by protocol governor
     * @dev Used to manage protocol risk and control staking distribution
     * @dev Emits PenaltyEnabledUpdated event
     */
    function updatePenaltyEnabled(bool _penaltyEnabled) external;

    /**
     * @notice Rescues a token and transfers it to the new owner
     * @param _token The address of the token to rescue
     * @param _to The address of the new owner
     * @dev Only callable by protocol governor
     * @dev Used to rescue tokens in case of emergency
     */
    function rescueToken(address _token, address _to) external;

    // ============ EVENTS ============

    /**
     * @notice Emitted when tokens are staked with a lockup period
     * @param receiver The address that staked the tokens
     * @param stakeIndex The index of the stake that was staked
     * @param amount The amount of tokens staked
     * @param lockupPeriod The lockup period in seconds
     * @param weightedAmount The weighted amount calculated for rewards
     */
    event StakedWithLockup(
        address indexed receiver,
        uint256 indexed stakeIndex,
        uint256 amount,
        uint256 lockupPeriod,
        uint256 weightedAmount
    );

    /**
     * @notice Emitted when tokens are unstaked with a penalty applied
     * @param receiver The owner of the stake that unstaked the tokens
     * @param stakeIndex The index of the stake that was unstaked
     * @param unstakedAmount The gross amount unstaked before penalty
     * @param penalty The penalty amount sent to treasury
     * @param returnAmount The net amount returned to user (unstakedAmount - penalty)
     */
    event UnstakedWithPenalty(
        address indexed receiver,
        uint256 indexed stakeIndex,
        uint256 unstakedAmount,
        uint256 penalty,
        uint256 returnAmount
    );

    /**
     * @notice Emitted when a lockup bucket cap is updated
     * @param bucket The bucket that was updated
     * @param cap The new cap amount (0 = disabled, max = unlimited)
     */
    event LockupBucketUpdated(Bucket indexed bucket, uint256 cap);

    /**
     * @notice Emitted when the penalty enabled status is updated
     * @param penaltyEnabled The new penalty enabled status
     */
    event PenaltyEnabledUpdated(bool penaltyEnabled);

    // ============ ERRORS ============

    /**
     * @notice Thrown when trying to use an invalid address (zero address)
     */
    error Staking_InvalidAddress(string message);

    /**
     * @notice Thrown when trying to use direct stake function instead of stakeLockup
     */
    error Staking_DirectStakeNotAllowed(string message);

    /**
     * @notice Thrown when trying to use direct unstake function instead of unstakeLockup
     */
    error Staking_DirectUnstakeNotAllowed(string message);

    /**
     * @notice Thrown when lockup period is invalid (too long, ended, etc.)
     */
    error Staking_InvalidLockupPeriod(string message);

    /**
     * @notice Thrown when stake index is invalid or out of bounds
     */
    error Staking_InvalidStakeIndex(string message);

    /**
     * @notice Thrown when trying to unstake more than available balance
     */
    error Staking_InsufficientBalance();

    /**
     * @notice Thrown when trying to use stakeOnBehalfOf (not supported)
     */
    error StakeOnBehalfOfNotSupported();

    /**
     * @notice Thrown when trying to use unstakeAndWithdrawOnBehalfOf (not supported)
     */
    error UnstakeOnBehalfOfNotSupported();

    /**
     * @notice Thrown when user has reached the maximum number of stakes allowed
     */
    error Staking_MaxStakesReached();

    /**
     * @notice Thrown when trying to stake amount that would exceed bucket cap
     */
    error Staking_BucketCapExceeded();

    /**
     * @notice Thrown when trying to stake/unstake amount that is invalid
     */
    error Staking_InvalidAmount(string message);
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {IConfigurationManaged} from "../interfaces/IConfigurationManaged.sol";
import {IConfigurationManager} from "../interfaces/IConfigurationManager.sol";

/**
 * @title ConfigurationManaged
 * @notice Base contract for contracts that need to read from the ConfigurationManager
 * @custom:see IConfigurationManaged
 */
abstract contract ConfigurationManaged is IConfigurationManaged {
    IConfigurationManager public immutable configurationManager;

    /**
     * @notice Constructs the ConfigurationManaged contract
     * @param _configurationManager The address of the ConfigurationManager contract
     */
    constructor(address _configurationManager) {
        if (_configurationManager == address(0)) {
            revert ConfigurationManagerZeroAddress();
        }
        configurationManager = IConfigurationManager(_configurationManager);
    }

    /// @inheritdoc IConfigurationManaged
    function raft() public view virtual returns (address) {
        return configurationManager.raft();
    }

    /// @inheritdoc IConfigurationManaged
    function tipJar() public view virtual returns (address) {
        return configurationManager.tipJar();
    }

    /// @inheritdoc IConfigurationManaged
    function treasury() public view virtual returns (address) {
        return configurationManager.treasury();
    }

    /// @inheritdoc IConfigurationManaged
    function harborCommand() public view virtual returns (address) {
        return configurationManager.harborCommand();
    }

    /// @inheritdoc IConfigurationManaged
    function fleetCommanderRewardsManagerFactory()
        public
        view
        virtual
        returns (address)
    {
        return configurationManager.fleetCommanderRewardsManagerFactory();
    }
}

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

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;

import {StorageSlot} from "./StorageSlot.sol";

/**
 * @dev Variant of {ReentrancyGuard} that uses transient storage.
 *
 * NOTE: This variant only works on networks where EIP-1153 is available.
 *
 * _Available since v5.1._
 */
abstract contract ReentrancyGuardTransient {
    using StorageSlot for *;

    // keccak256(abi.encode(uint256(keccak256("openzeppelin.storage.ReentrancyGuard")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant REENTRANCY_GUARD_STORAGE =
        0x9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f00;

    /**
     * @dev Unauthorized reentrant call.
     */
    error ReentrancyGuardReentrantCall();

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
        // On the first call to nonReentrant, _status will be NOT_ENTERED
        if (_reentrancyGuardEntered()) {
            revert ReentrancyGuardReentrantCall();
        }

        // Any calls to nonReentrant after this point will fail
        REENTRANCY_GUARD_STORAGE.asBoolean().tstore(true);
    }

    function _nonReentrantAfter() private {
        REENTRANCY_GUARD_STORAGE.asBoolean().tstore(false);
    }

    /**
     * @dev Returns true if the reentrancy guard is currently set to "entered", which indicates there is a
     * `nonReentrant` function in the call stack.
     */
    function _reentrancyGuardEntered() internal view returns (bool) {
        return REENTRANCY_GUARD_STORAGE.asBoolean().tload();
    }
}


END OF SUPPORTING CONTRACTS AND INTERFACES
