
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

// import {Initializable} from "@solady/utils/Initializable.sol";
import {OwnableRoles} from "@solady/auth/OwnableRoles.sol";
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";
import {SafeTransferLib} from "@solady/utils/SafeTransferLib.sol";

import {IDistributor, IGTELaunchpadV2Pair} from "./interfaces/IDistributor.sol";
import "./libraries/RewardsTracker.sol";

/// @notice This contract receives tokens from
contract Distributor is OwnableRoles, IDistributor {
    using SafeTransferLib for address;

    event TotalPendingRewardsIncreased(address indexed asset, uint256 amount);
    event TotalPendingRewardsDecreased(address indexed asset, uint256 amount);

    error Initialized();
    error RewardsExist();
    error RewardsDoNotExist();
    error ClaimAmountExceedsTotalPendingRewards();
    error NoSharesToIncentivize();
    error SkimOverflow();

    uint256 public constant ADMIN_ROLE = _ROLE_0;

    bool private initialized;
    address public launchpad;

    /// @dev metadata to recover donations while preserving pending rewards
    mapping(address => uint256) public totalPendingRewards;

    constructor() {
        _initializeOwner(msg.sender);
    }

    /// @dev There is no init check anywhere else because this contract can't be used until the launchpad address is set
    function initialize(address _launchpad) public onlyOwner {
        if (initialized) revert Initialized();
        launchpad = _launchpad;
        initialized = true;
    }

    modifier onlyLaunchpad() {
        if (msg.sender != launchpad) revert Unauthorized();
        _;
    }

    function skimExcessRewards(address asset, uint256 amount) external onlyOwnerOrRoles(ADMIN_ROLE) {
        if (amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]) revert SkimOverflow();

        asset.safeTransfer(msg.sender, amount);
    }

    function getRewardsPoolData(address launchAsset) external view returns (RewardPoolDataMemory memory) {
        return RewardsTrackerStorage.getRewardPool(launchAsset).getRewardsPoolData();
    }

    function getUserData(address launchAsset, address account) external view returns (UserRewardData memory) {
        return RewardsTrackerStorage.getRewardPool(launchAsset).getUserData(account);
    }

    function getUserDataForTokens(address[] calldata launchAssets, address account)
        external
        view
        returns (UserRewardData[] memory)
    {
        UserRewardData[] memory data = new UserRewardData[](launchAssets.length);

        for (uint256 i = 0; i < launchAssets.length; i++) {
            data[i] = RewardsTrackerStorage.getRewardPool(launchAssets[i]).getUserData(account);
        }
        return data;
    }

    function getPendingRewards(address launchAsset, address account)
        external
        view
        returns (uint256 pendingBase, uint256 pendingQuote)
    {
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);

        return rs.getPendingRewards(account);
    }

    function endRewards(IGTELaunchpadV2Pair pair) external onlyLaunchpad {
        pair.endRewardsAccrual();
    }

    /// @notice Initializes the rewards pair from the launchpad
    /// @dev Neither the launchAsset, nor the quoteAsset can be the baseAsset of an existing reward pool
    function createRewardsPair(address launchAsset, address quoteAsset) external onlyLaunchpad {
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);
        RewardPoolData storage rsq = RewardsTrackerStorage.getRewardPool(quoteAsset);

        // Sanity check in case the admin makes the quote asset of launchpad an existing asset
        if (rs.quoteAsset != address(0) || rsq.quoteAsset != address(0)) revert RewardsExist();

        rs.initializePair(launchAsset, quoteAsset);
    }

    /// @notice Allows rewards to be added to a pool regardless of token order
    /// @dev Pools can only be created once per asset combo, regardless of the order of the assets
    /// Additionally, anyone can add rewards as incentive, even while a pair is still bonding
    function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external {
        (address launchAsset, address quoteAsset, uint128 launchAssetAmount, uint128 quoteAssetAmount) =
            (token0, token1, amount0, amount1);
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(token0);

        if (rs.quoteAsset == address(0)) {
            rs = RewardsTrackerStorage.getRewardPool(token1);

            if (rs.quoteAsset == address(0)) revert RewardsDoNotExist();

            (launchAsset, quoteAsset, launchAssetAmount, quoteAssetAmount) = (token1, token0, amount1, amount0);
        }

        if (rs.totalShares == 0) revert NoSharesToIncentivize();

        if (launchAssetAmount > 0) {
            rs.addBaseRewards(launchAsset, launchAssetAmount);
            _increaseTotalPending(launchAsset, launchAssetAmount);
            launchAsset.safeTransferFrom(msg.sender, address(this), uint256(launchAssetAmount));
        }

        if (quoteAssetAmount > 0) {
            rs.addQuoteRewards(launchAsset, quoteAsset, quoteAssetAmount);
            _increaseTotalPending(quoteAsset, quoteAssetAmount);
            quoteAsset.safeTransferFrom(msg.sender, address(this), uint256(quoteAssetAmount));
        }
    }

    /// @dev This can only be called while `launchAsset` is bonding, so we dont need to check if the pool exists or is still active
    function increaseStake(address launchAsset, address account, uint96 shares)
        external
        onlyLaunchpad
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);

        (baseAmount, quoteAmount) = rs.stake(account, uint96(shares));
        _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
    }

    /// @dev This can only be called while `launchAsset` still shares to remove from bonders, so it cannot be called after the pool has been deactivated
    function decreaseStake(address launchAsset, address account, uint96 shares)
        external
        onlyLaunchpad
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);

        (baseAmount, quoteAmount) = rs.unstake(account, uint96(shares));
        _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
    }

    /// @dev This can be called even after a pool has been deactivated, as accounts may still have pending rewards
    function claimRewards(address launchAsset) external returns (uint256 baseAmount, uint256 quoteAmount) {
        RewardPoolData storage rs = RewardsTrackerStorage.getRewardPool(launchAsset);

        (baseAmount, quoteAmount) = rs.claim(msg.sender);

        _distributeAssets(launchAsset, baseAmount, rs.quoteAsset, quoteAmount);
    }

    function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
        if (baseAmount > 0) {
            _decreaseTotalPending(base, baseAmount);
            base.safeTransfer(msg.sender, baseAmount);
        }

        if (quoteAmount > 0) {
            _decreaseTotalPending(quote, quoteAmount);
            quote.safeTransfer(msg.sender, quoteAmount);
        }
    }

    function _increaseTotalPending(address asset, uint256 amount) internal {
        unchecked {
            totalPendingRewards[asset] += amount;
        }

        emit TotalPendingRewardsIncreased(asset, amount);
    }

    function _decreaseTotalPending(address asset, uint256 amount) internal {
        uint256 currTotal = totalPendingRewards[asset];

        if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();

        unchecked {
            totalPendingRewards[asset] -= amount;
        }

        emit TotalPendingRewardsDecreased(asset, amount);
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import "@gte-univ2-core/interfaces/IUniswapV2Pair.sol";
import "@gte-univ2-core/UniswapV2ERC20.sol";
import "@gte-univ2-core/libraries/Math.sol";
import "@gte-univ2-core/libraries/UQ112x112.sol";
import "@gte-univ2-core/interfaces/IERC20.sol";
import "@gte-univ2-core/interfaces/IUniswapV2Factory.sol";
import "@gte-univ2-core/interfaces/IUniswapV2Callee.sol";

import "../interfaces/IDistributor.sol";
import "./interfaces/IGTELaunchpadV2Pair.sol";

// import "forge-std/Console.sol";

contract GTELaunchpadV2Pair is IUniswapV2Pair, IGTELaunchpadV2Pair, UniswapV2ERC20 {
    using SafeMath for uint256;
    using UQ112x112 for uint224;

    event RewardsPoolDeactivated();
    event LaunchpadFeesAccrued(uint112 fee0, uint112 fee1);
    event LaunchpadFeesCollected(uint256 collected0, uint256 collected1);
    event LaunchpadFeesLastAccrued(uint112 fee0, uint112 fee1);

    uint256 public constant REWARDS_FEE_SHARE = 1;
    uint256 public constant MINIMUM_LIQUIDITY = 10 ** 3;
    bytes4 private constant TRANSFER_SELECTOR = bytes4(keccak256(bytes("transfer(address,uint256)")));
    bytes4 private constant APPROVE_SELECTOR = bytes4(keccak256(bytes("approve(address,uint256)")));

    // If the launchpad is non 0, then this pool will accumulate a share of swap fees that can be claimed
    address public launchpadLp;
    address public launchpadFeeDistributor;

    address public factory;
    address public token0;
    address public token1;

    uint112 private reserve0; // uses single storage slot, accessible via getReserves
    uint112 private reserve1; // uses single storage slot, accessible via getReserves
    uint32 private blockTimestampLast; // uses single storage slot, accessible via getReserves

    uint256 public price0CumulativeLast;
    uint256 public price1CumulativeLast;
    uint256 public kLast; // reserve0 * reserve1, as of immediately after the most recent liquidity event

    uint112 public accruedLaunchpadFee0;
    uint112 public accruedLaunchpadFee1;

    uint256 public rewardsPoolActive = 1;

    uint256 private unlocked = 1;

    modifier lock() {
        if (unlocked != 1) revert("UniswapV2: LOCKED");
        unlocked = 0;
        _;
        unlocked = 1;
    }

    function getReserves() public view returns (uint112 _reserve0, uint112 _reserve1, uint32 _blockTimestampLast) {
        _reserve0 = reserve0;
        _reserve1 = reserve1;
        _blockTimestampLast = blockTimestampLast;
    }

    function getAccruedLaunchpadFees() public view returns (uint112, uint112, uint32) {
        return (accruedLaunchpadFee0, accruedLaunchpadFee1, blockTimestampLast);
    }

    function _safeTransfer(address token, address to, uint256 value) private {
        (bool success, bytes memory data) = token.call(abi.encodeWithSelector(TRANSFER_SELECTOR, to, value));
        if (!success || !(data.length == 0 || abi.decode(data, (bool)))) revert("UniswapV2: TRANSFER_FAILED");
    }

    function _safeApprove(address token, address to, uint256 value) private {
        (bool success, bytes memory data) = token.call(abi.encodeWithSelector(APPROVE_SELECTOR, to, value));
        if (!success || !(data.length == 0 || abi.decode(data, (bool)))) revert("UniswapV2: APPROVAL_FAILED");
    }

    constructor() {
        factory = msg.sender;
    }

    // called once by the factory at time of deployment
    function initialize(address _token0, address _token1, address _launchpadLp, address _launchpadFeeDistributor)
        external
    {
        if (msg.sender != factory) revert("UniswapV2: FORBIDDEN"); // sufficient check
        token0 = _token0;
        token1 = _token1;
        launchpadLp = _launchpadLp;
        launchpadFeeDistributor = _launchpadFeeDistributor;
        rewardsPoolActive = 1;
    }

    function endRewardsAccrual() external {
        if (msg.sender != launchpadFeeDistributor) revert("GTEUniV2: FORBIDDEN");

        // There are no more shares, so prevent distribution and accrual of any remaining rewards
        delete accruedLaunchpadFee0;
        delete accruedLaunchpadFee1;
        delete rewardsPoolActive;

        _update(
            IERC20(token0).balanceOf(address(this)),
            IERC20(token1).balanceOf(address(this)),
            reserve0,
            reserve1,
            uint112(0),
            uint112(0)
        );

        emit RewardsPoolDeactivated();
    }

    // update reserves and, on the first call per block, price accumulators
    function _update(
        uint256 balance0,
        uint256 balance1,
        uint112 _reserve0,
        uint112 _reserve1,
        uint112 newLaunchpadFee0,
        uint112 newLaunchpadFee1
    ) private {
        if (balance0 > type(uint112).max || balance1 > type(uint112).max) revert("UniswapV2: OVERFLOW");

        // New accrued fees must AT LEAST equal existing undistributed fees so that the Sync can be accurate
        uint112 totalLaunchpadFee0 = accruedLaunchpadFee0 + newLaunchpadFee0;
        uint112 totalLaunchpadFee1 = accruedLaunchpadFee1 + newLaunchpadFee1;

        uint32 blockTimestamp = uint32(block.timestamp % 2 ** 32);
        uint32 timeElapsed = blockTimestamp - blockTimestampLast; // overflow is desired
        if (timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0) {
            // * never overflows, and + overflow is desired
            price0CumulativeLast += uint256(UQ112x112.encode(_reserve1).uqdiv(_reserve0)) * timeElapsed;
            price1CumulativeLast += uint256(UQ112x112.encode(_reserve0).uqdiv(_reserve1)) * timeElapsed;

            if (launchpadFeeDistributor > address(0)) {
                if (totalLaunchpadFee0 | totalLaunchpadFee1 > 0) {
                    delete accruedLaunchpadFee0;
                    delete accruedLaunchpadFee1;
                    _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1);
                }
            }
        } else if (launchpadFeeDistributor > address(0) && newLaunchpadFee0 | newLaunchpadFee1 > 0) {
            accruedLaunchpadFee0 = totalLaunchpadFee0;
            accruedLaunchpadFee1 = totalLaunchpadFee1;
            emit LaunchpadFeesAccrued(newLaunchpadFee0, newLaunchpadFee1);
        }

        // Balances contain both accrued and new launchpad fees earned this tx
        // as balance is called before any fee distributions, so subtract the total
        reserve0 = _reserve0 = uint112(balance0) - totalLaunchpadFee0;
        reserve1 = _reserve1 = uint112(balance1) - totalLaunchpadFee1;

        blockTimestampLast = blockTimestamp;
        emit Sync(_reserve0, _reserve1);
    }

    // if fee is on, mint liquidity equivalent to 1/6th of the growth in sqrt(k)
    function _mintFee(uint112 _reserve0, uint112 _reserve1) private returns (bool feeOn) {
        address feeTo = IUniswapV2Factory(factory).feeTo();
        feeOn = feeTo != address(0);
        uint256 _kLast = kLast; // gas savings
        if (feeOn) {
            if (_kLast != 0) {
                uint256 rootK = Math.sqrt(uint256(_reserve0).mul(_reserve1));
                uint256 rootKLast = Math.sqrt(_kLast);
                if (rootK > rootKLast) {
                    uint256 numerator = totalSupply.mul(rootK.sub(rootKLast));
                    uint256 denominator = rootK.mul(5).add(rootKLast);
                    uint256 liquidity = numerator / denominator;
                    if (liquidity > 0) _mint(feeTo, liquidity);
                }
            }
        } else if (_kLast != 0) {
            kLast = 0;
        }
    }

    // this low-level function should be called from a contract which performs important safety checks
    function mint(address to) external lock returns (uint256 liquidity) {
        (uint112 _reserve0, uint112 _reserve1,) = getReserves(); // gas savings
        uint256 balance0 = IERC20(token0).balanceOf(address(this));
        uint256 balance1 = IERC20(token1).balanceOf(address(this));
        uint256 amount0 = balance0.sub(_reserve0);
        uint256 amount1 = balance1.sub(_reserve1);

        bool feeOn = _mintFee(_reserve0, _reserve1);
        uint256 _totalSupply = totalSupply; // gas savings, must be defined here since totalSupply can update in _mintFee
        if (_totalSupply == 0) {
            liquidity = Math.sqrt(amount0.mul(amount1)).sub(MINIMUM_LIQUIDITY);
            _mint(address(0), MINIMUM_LIQUIDITY); // permanently lock the first MINIMUM_LIQUIDITY tokens
        } else {
            liquidity = Math.min(amount0.mul(_totalSupply) / _reserve0, amount1.mul(_totalSupply) / _reserve1);
        }
        if (liquidity == 0) revert("UniswapV2: INSUFFICIENT_LIQUIDITY_MINTED");
        _mint(to, liquidity);

        _update(balance0, balance1, _reserve0, _reserve1, uint112(0), uint112(0));
        if (feeOn) kLast = uint256(reserve0).mul(reserve1); // reserve0 and reserve1 are up-to-date
        emit Mint(msg.sender, amount0, amount1);
    }

    // this low-level function should be called from a contract which performs important safety checks
    function burn(address to) external lock returns (uint256 amount0, uint256 amount1) {
        (uint112 _reserve0, uint112 _reserve1,) = getReserves(); // gas savings
        address _token0 = token0; // gas savings
        address _token1 = token1; // gas savings
        uint256 balance0 = IERC20(_token0).balanceOf(address(this));
        uint256 balance1 = IERC20(_token1).balanceOf(address(this));
        uint256 liquidity = balanceOf[address(this)];

        bool feeOn = _mintFee(_reserve0, _reserve1);
        uint256 _totalSupply = totalSupply; // gas savings, must be defined here since totalSupply can update in _mintFee
        amount0 = liquidity.mul(balance0) / _totalSupply; // using balances ensures pro-rata distribution
        amount1 = liquidity.mul(balance1) / _totalSupply; // using balances ensures pro-rata distribution
        if (amount0 == 0 || amount1 == 0) revert("UniswapV2: INSUFFICIENT_LIQUIDITY_BURNED");
        _burn(address(this), liquidity);
        _safeTransfer(_token0, to, amount0);
        _safeTransfer(_token1, to, amount1);
        balance0 = IERC20(_token0).balanceOf(address(this));
        balance1 = IERC20(_token1).balanceOf(address(this));

        _update(balance0, balance1, _reserve0, _reserve1, uint112(0), uint112(0));
        if (feeOn) kLast = uint256(reserve0).mul(reserve1); // reserve0 and reserve1 are up-to-date
        emit Burn(msg.sender, amount0, amount1, to);
    }

    // this low-level function should be called from a contract which performs important safety checks
    function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external lock {
        if (amount0Out == 0 && amount1Out == 0) revert("UniswapV2: INSUFFICIENT_OUTPUT_AMOUNT");
        (uint112 _reserve0, uint112 _reserve1,) = getReserves(); // gas savings
        if (amount0Out >= _reserve0 || amount1Out >= _reserve1) revert("UniswapV2: INSUFFICIENT_LIQUIDITY");

        uint256 balance0;
        uint256 balance1;
        {
            // scope for _token{0,1}, avoids stack too deep errors
            address _token0 = token0;
            address _token1 = token1;
            if (to == _token0 || to == _token1) revert("UniswapV2: INVALID_TO");
            if (amount0Out > 0) _safeTransfer(_token0, to, amount0Out); // optimistically transfer tokens
            if (amount1Out > 0) _safeTransfer(_token1, to, amount1Out); // optimistically transfer tokens
            if (data.length > 0) IUniswapV2Callee(to).uniswapV2Call(msg.sender, amount0Out, amount1Out, data);
            balance0 = IERC20(_token0).balanceOf(address(this));
            balance1 = IERC20(_token1).balanceOf(address(this));
        }
        uint256 amount0In = balance0 > _reserve0 - amount0Out ? balance0 - (_reserve0 - amount0Out) : 0;
        uint256 amount1In = balance1 > _reserve1 - amount1Out ? balance1 - (_reserve1 - amount1Out) : 0;
        if (amount0In == 0 && amount1In == 0) revert("UniswapV2: INSUFFICIENT_INPUT_AMOUNT");

        {
            // scope for reserve{0,1}Adjusted and launchpadFee{0,1}, avoids stack too deep errors
            uint256 balance0Adjusted = balance0.mul(1000).sub(amount0In.mul(3));
            uint256 balance1Adjusted = balance1.mul(1000).sub(amount1In.mul(3));

            if (balance0Adjusted.mul(balance1Adjusted) < uint256(_reserve0).mul(_reserve1).mul(1000 ** 2)) {
                revert("UniswapV2: K");
            }

            (uint112 launchpadFee0, uint112 launchpadFee1) = launchpadFeeDistributor > address(0)
                && rewardsPoolActive > 0 ? _getLaunchpadFees(amount0In, amount1In) : (uint112(0), uint112(0));

            _update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
        }

        emit Swap(msg.sender, amount0In, amount1In, amount0Out, amount1Out, to);
    }

    function _getLaunchpadFees(uint256 amount0In, uint256 amount1In)
        internal
        view
        returns (uint112 fee0, uint112 fee1)
    {
        // Only swap fees that represent the launchpad's share of the LP supply should accrue towards unclaimed launchpad fees.
        // MINIMUM_LIQUIDITY is permanently locked to address(0) when liquidity is first added by the launchpad.
        // Therefore, we add it back here so that the fee share math represents the whole of *initial* liquidity added,
        // not just the launchpad's slightly lower actual lp balance
        uint256 totalLpBal = this.totalSupply();
        uint256 launchpadLpBal = this.balanceOf(launchpadLp) + MINIMUM_LIQUIDITY;

        if (amount0In > 0) fee0 = uint112(amount0In.mul(REWARDS_FEE_SHARE).mul(launchpadLpBal) / (totalLpBal * 1000));
        if (amount1In > 0) fee1 = uint112(amount1In.mul(REWARDS_FEE_SHARE).mul(launchpadLpBal) / (totalLpBal * 1000));

        return (fee0, fee1);
    }

    function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
        if ((fee0 | fee1) > 0) {
            address _token0 = token0;
            address _token1 = token1;
            address distributor = launchpadFeeDistributor;

            // Since only pairs created by the launchpad can accrue fee tracking, the tokens are trusted
            if (fee0 > 0) _safeApprove(_token0, distributor, uint256(fee0));
            if (fee1 > 0) _safeApprove(_token1, distributor, uint256(fee1));

            IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));

            emit LaunchpadFeesCollected(fee0, fee1);
        }
    }

    // force balances to match reserves
    function skim(address to) external lock {
        address _token0 = token0; // gas savings
        address _token1 = token1; // gas savings
        _safeTransfer(_token0, to, IERC20(_token0).balanceOf(address(this)).sub(reserve0 + accruedLaunchpadFee0));
        _safeTransfer(_token1, to, IERC20(_token1).balanceOf(address(this)).sub(reserve1 + accruedLaunchpadFee1));
    }

    // force reserves to match balances
    function sync() external lock {
        _update(
            IERC20(token0).balanceOf(address(this)),
            IERC20(token1).balanceOf(address(this)),
            reserve0,
            reserve1,
            uint112(0),
            uint112(0)
        );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

struct UserRewardData {
    uint96 shares; // User's current share count (up to ~7.9e28)
    uint96 baseRewardDebt; // Used to calculate base token rewards owed
    uint96 quoteRewardDebt; // Used to calculate quote token rewards owed
}

struct RewardPoolData {
    // SLOT 0 //
    uint96 totalShares; // Sum of all user shares
    address quoteAsset; // Secondary reward token
    // SLOT 1 //
    uint128 pendingBaseRewards;
    uint128 pendingQuoteRewards;
    // SLOT 2 //
    uint256 accBaseRewardPerShare; // Accumulated base rewards per share, scaled by 1e12
    uint256 accQuoteRewardPerShare; // Accumulated quote rewards per share, scaled by 1e12
    // SLOT 3 //
    mapping(address => UserRewardData) userRewards; // User-specific reward data
}

struct RewardPoolDataMemory {
    uint96 totalShares; // Sum of all user shares
    address quoteAsset; // Secondary reward token
    uint128 pendingBaseRewards;
    uint128 pendingQuoteRewards; //
    uint256 accBaseRewardPerShare;
    uint256 accQuoteRewardPerShare; // Accumulated quote rewards per share, scaled by 1e12
}

using RewardsTrackerLib for RewardPoolData global;
/**
 * @title RewardsLibrary
 * @dev Library with internal functions for pro rata reward distribution
 */

library RewardsTrackerLib {
    /// @dev sig: 0x9511e79574c9aa195c27c3455b60ba70c9a6efbcfc431ae68b8a3cb4d3764f6c
    event PairRewardsInitialized(address indexed baseAsset, address indexed quoteAsset);
    /// @dev sig: 0x2cbe0649bcb43ba4ace580eeeb0c95a516dec93862fe4cc4e7e60528575cec67
    event BaseRewardsAdded(address indexed baseAsset, uint256 amount);
    /// @dev sig: 0x28590542f9792ca8533cd1beac50e724892009d1f19ed17351f264be124d3293
    event QuoteRewardsAdded(address indexed baseAsset, address indexed quoteAsset, uint256 amount);

    /// @dev sig: 0xe3e46b04
    error ZeroShareStake();
    /// @dev sig: 0xe331bd04
    error ZeroShareClaim();
    /// @dev sig: 0x39996567
    error InsufficientShares();

    // Scale factor used for fixed-point math
    uint128 public constant PRECISION_FACTOR = 1e12;

    function getQuoteAsset(RewardPoolData storage self) internal view returns (address) {
        return self.quoteAsset;
    }

    function getUserData(RewardPoolData storage self, address account) internal view returns (UserRewardData memory) {
        return self.userRewards[account];
    }

    function getRewardsPoolData(RewardPoolData storage self) internal view returns (RewardPoolDataMemory memory pm) {
        pm = RewardPoolDataMemory({
            quoteAsset: self.quoteAsset,
            totalShares: self.totalShares,
            pendingBaseRewards: self.pendingBaseRewards,
            pendingQuoteRewards: self.pendingQuoteRewards,
            accBaseRewardPerShare: self.accBaseRewardPerShare,
            accQuoteRewardPerShare: self.accQuoteRewardPerShare
        });
    }

    function initializePair(RewardPoolData storage self, address baseAsset, address quoteAsset) internal {
        self.quoteAsset = quoteAsset;
        emit PairRewardsInitialized(baseAsset, quoteAsset);
    }

    function addBaseRewards(RewardPoolData storage self, address baseAsset, uint128 amount) internal {
        self.pendingBaseRewards += amount;
        emit BaseRewardsAdded(baseAsset, amount);
    }

    function addQuoteRewards(RewardPoolData storage self, address baseAsset, address quoteAsset, uint128 amount)
        internal
    {
        self.pendingQuoteRewards += amount;
        emit QuoteRewardsAdded(baseAsset, quoteAsset, amount);
    }

    function stake(RewardPoolData storage self, address user, uint96 newShares)
        internal
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        if (newShares == 0) revert ZeroShareStake();

        (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();

        UserRewardData storage userData = self.userRewards[user];

        uint256 existingShares = uint96(userData.shares);

        // Calculate pending rewards before updating shares
        if (existingShares > 0) {
            baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
            quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;
        }

        // Update user shares
        userData.shares += newShares;
        self.totalShares += newShares;

        // Update reward debts
        userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
        userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));
    }

    function unstake(RewardPoolData storage self, address user, uint96 removeShares)
        internal
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();

        UserRewardData storage userData = self.userRewards[user];

        if (removeShares == 0) revert ZeroShareStake();

        uint256 existingShares = uint256(userData.shares);
        if (existingShares < removeShares) revert InsufficientShares();

        // Calculate pending rewards before updating shares
        baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
        quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;

        // Update user shares
        userData.shares -= removeShares;
        self.totalShares -= removeShares;

        // Update reward debts
        userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
        userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));
    }

    function claim(RewardPoolData storage self, address user)
        internal
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        UserRewardData storage userData = self.userRewards[user];
        uint256 shares = uint256(userData.shares);

        if (shares == 0) revert ZeroShareClaim();

        (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();

        // Calculate pending rewards
        uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
        uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);

        baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
        quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);

        // Update reward debts
        userData.baseRewardDebt = uint96(totalAccBaseRewards);
        userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
    }

    function getPendingRewards(RewardPoolData storage self, address user)
        internal
        view
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = getAccRewardsPerShare(self);

        UserRewardData storage userData = self.userRewards[user];
        uint256 shares = uint256(userData.shares);

        // Unstaking claims pending rewards, so if no shares, then no pending
        if (shares == 0) return (0, 0);

        baseAmount = totalAccRewards(shares, accBaseRewardsPerShare) - uint128(userData.baseRewardDebt);
        quoteAmount = totalAccRewards(shares, accQuoteRewardsPerShare) - uint128(userData.quoteRewardDebt);
    }

    function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
        return (shares * accRewardsPerShare) / PRECISION_FACTOR;
    }

    /// @dev Applies the new accrued rewards per share to the rewards state
    function update(RewardPoolData storage self)
        internal
        returns (uint256 newAccBaseRewardsPerShare, uint256 newAccQuoteRewardsPerShare)
    {
        (newAccBaseRewardsPerShare, newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self);

        if (self.pendingBaseRewards > 0) {
            self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
            delete self.pendingBaseRewards;
        }

        if (self.pendingQuoteRewards > 0) {
            self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
            delete self.pendingQuoteRewards;
        }
    }

    /// @dev Gets the new accrued rewards per share without updating rewards state
    function getAccRewardsPerShare(RewardPoolData storage self)
        internal
        view
        returns (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare)
    {
        uint96 totalShares = self.totalShares;
        if (totalShares == 0) return (self.accBaseRewardPerShare, self.accQuoteRewardPerShare);

        accBaseRewardsPerShare = self.accBaseRewardPerShare;
        accQuoteRewardsPerShare = self.accQuoteRewardPerShare;

        if (self.pendingBaseRewards > 0) {
            accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
        }

        if (self.pendingQuoteRewards > 0) {
            accQuoteRewardsPerShare += ((self.pendingQuoteRewards * PRECISION_FACTOR) / uint128(totalShares));
        }
    }
}

/**
 * @title RewardsStorage
 * @dev Storage library for rewards distribution using EIP-1967 pattern
 */
library RewardsTrackerStorage {
    bytes32 internal constant LAUNCH_ASSET_TO_REWARDS_SLOT =
        keccak256(abi.encode(uint256(keccak256("rewardsTrackerPool.self.slot")) - 1)) & ~bytes32(uint256(0xff));

    function rewardPoolSlot(address baseAsset) private pure returns (bytes32) {
        return keccak256(abi.encodePacked(baseAsset, LAUNCH_ASSET_TO_REWARDS_SLOT));
    }

    function getRewardPool(address baseAsset) internal pure returns (RewardPoolData storage p) {
        bytes32 slot = rewardPoolSlot(baseAsset);
        assembly {
            p.slot := slot
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import {Ownable} from "./Ownable.sol";

/// @notice Simple single owner and multiroles authorization mixin.
/// @author Solady (https://github.com/vectorized/solady/blob/main/src/auth/OwnableRoles.sol)
///
/// @dev Note:
/// This implementation does NOT auto-initialize the owner to `msg.sender`.
/// You MUST call the `_initializeOwner` in the constructor / initializer.
///
/// While the ownable portion follows
/// [EIP-173](https://eips.ethereum.org/EIPS/eip-173) for compatibility,
/// the nomenclature for the 2-step ownership handover may be unique to this codebase.
abstract contract OwnableRoles is Ownable {
    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                           EVENTS                           */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The `user`'s roles is updated to `roles`.
    /// Each bit of `roles` represents whether the role is set.
    event RolesUpdated(address indexed user, uint256 indexed roles);

    /// @dev `keccak256(bytes("RolesUpdated(address,uint256)"))`.
    uint256 private constant _ROLES_UPDATED_EVENT_SIGNATURE =
        0x715ad5ce61fc9595c7b415289d59cf203f23a94fa06f04af7e489a0a76e1fe26;

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                          STORAGE                           */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The role slot of `user` is given by:
    /// ```
    ///     mstore(0x00, or(shl(96, user), _ROLE_SLOT_SEED))
    ///     let roleSlot := keccak256(0x00, 0x20)
    /// ```
    /// This automatically ignores the upper bits of the `user` in case
    /// they are not clean, as well as keep the `keccak256` under 32-bytes.
    ///
    /// Note: This is equivalent to `uint32(bytes4(keccak256("_OWNER_SLOT_NOT")))`.
    uint256 private constant _ROLE_SLOT_SEED = 0x8b78c6d8;

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                     INTERNAL FUNCTIONS                     */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Overwrite the roles directly without authorization guard.
    function _setRoles(address user, uint256 roles) internal virtual {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(0x0c, _ROLE_SLOT_SEED)
            mstore(0x00, user)
            // Store the new value.
            sstore(keccak256(0x0c, 0x20), roles)
            // Emit the {RolesUpdated} event.
            log3(0, 0, _ROLES_UPDATED_EVENT_SIGNATURE, shr(96, mload(0x0c)), roles)
        }
    }

    /// @dev Updates the roles directly without authorization guard.
    /// If `on` is true, each set bit of `roles` will be turned on,
    /// otherwise, each set bit of `roles` will be turned off.
    function _updateRoles(address user, uint256 roles, bool on) internal virtual {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(0x0c, _ROLE_SLOT_SEED)
            mstore(0x00, user)
            let roleSlot := keccak256(0x0c, 0x20)
            // Load the current value.
            let current := sload(roleSlot)
            // Compute the updated roles if `on` is true.
            let updated := or(current, roles)
            // Compute the updated roles if `on` is false.
            // Use `and` to compute the intersection of `current` and `roles`,
            // `xor` it with `current` to flip the bits in the intersection.
            if iszero(on) { updated := xor(current, and(current, roles)) }
            // Then, store the new value.
            sstore(roleSlot, updated)
            // Emit the {RolesUpdated} event.
            log3(0, 0, _ROLES_UPDATED_EVENT_SIGNATURE, shr(96, mload(0x0c)), updated)
        }
    }

    /// @dev Grants the roles directly without authorization guard.
    /// Each bit of `roles` represents the role to turn on.
    function _grantRoles(address user, uint256 roles) internal virtual {
        _updateRoles(user, roles, true);
    }

    /// @dev Removes the roles directly without authorization guard.
    /// Each bit of `roles` represents the role to turn off.
    function _removeRoles(address user, uint256 roles) internal virtual {
        _updateRoles(user, roles, false);
    }

    /// @dev Throws if the sender does not have any of the `roles`.
    function _checkRoles(uint256 roles) internal view virtual {
        /// @solidity memory-safe-assembly
        assembly {
            // Compute the role slot.
            mstore(0x0c, _ROLE_SLOT_SEED)
            mstore(0x00, caller())
            // Load the stored value, and if the `and` intersection
            // of the value and `roles` is zero, revert.
            if iszero(and(sload(keccak256(0x0c, 0x20)), roles)) {
                mstore(0x00, 0x82b42900) // `Unauthorized()`.
                revert(0x1c, 0x04)
            }
        }
    }

    /// @dev Throws if the sender is not the owner,
    /// and does not have any of the `roles`.
    /// Checks for ownership first, then lazily checks for roles.
    function _checkOwnerOrRoles(uint256 roles) internal view virtual {
        /// @solidity memory-safe-assembly
        assembly {
            // If the caller is not the stored owner.
            // Note: `_ROLE_SLOT_SEED` is equal to `_OWNER_SLOT_NOT`.
            if iszero(eq(caller(), sload(not(_ROLE_SLOT_SEED)))) {
                // Compute the role slot.
                mstore(0x0c, _ROLE_SLOT_SEED)
                mstore(0x00, caller())
                // Load the stored value, and if the `and` intersection
                // of the value and `roles` is zero, revert.
                if iszero(and(sload(keccak256(0x0c, 0x20)), roles)) {
                    mstore(0x00, 0x82b42900) // `Unauthorized()`.
                    revert(0x1c, 0x04)
                }
            }
        }
    }

    /// @dev Throws if the sender does not have any of the `roles`,
    /// and is not the owner.
    /// Checks for roles first, then lazily checks for ownership.
    function _checkRolesOrOwner(uint256 roles) internal view virtual {
        /// @solidity memory-safe-assembly
        assembly {
            // Compute the role slot.
            mstore(0x0c, _ROLE_SLOT_SEED)
            mstore(0x00, caller())
            // Load the stored value, and if the `and` intersection
            // of the value and `roles` is zero, revert.
            if iszero(and(sload(keccak256(0x0c, 0x20)), roles)) {
                // If the caller is not the stored owner.
                // Note: `_ROLE_SLOT_SEED` is equal to `_OWNER_SLOT_NOT`.
                if iszero(eq(caller(), sload(not(_ROLE_SLOT_SEED)))) {
                    mstore(0x00, 0x82b42900) // `Unauthorized()`.
                    revert(0x1c, 0x04)
                }
            }
        }
    }

    /// @dev Convenience function to return a `roles` bitmap from an array of `ordinals`.
    /// This is meant for frontends like Etherscan, and is therefore not fully optimized.
    /// Not recommended to be called on-chain.
    /// Made internal to conserve bytecode. Wrap it in a public function if needed.
    function _rolesFromOrdinals(uint8[] memory ordinals) internal pure returns (uint256 roles) {
        /// @solidity memory-safe-assembly
        assembly {
            for { let i := shl(5, mload(ordinals)) } i { i := sub(i, 0x20) } {
                // We don't need to mask the values of `ordinals`, as Solidity
                // cleans dirty upper bits when storing variables into memory.
                roles := or(shl(mload(add(ordinals, i)), 1), roles)
            }
        }
    }

    /// @dev Convenience function to return an array of `ordinals` from the `roles` bitmap.
    /// This is meant for frontends like Etherscan, and is therefore not fully optimized.
    /// Not recommended to be called on-chain.
    /// Made internal to conserve bytecode. Wrap it in a public function if needed.
    function _ordinalsFromRoles(uint256 roles) internal pure returns (uint8[] memory ordinals) {
        /// @solidity memory-safe-assembly
        assembly {
            // Grab the pointer to the free memory.
            ordinals := mload(0x40)
            let ptr := add(ordinals, 0x20)
            let o := 0
            // The absence of lookup tables, De Bruijn, etc., here is intentional for
            // smaller bytecode, as this function is not meant to be called on-chain.
            for { let t := roles } 1 {} {
                mstore(ptr, o)
                // `shr` 5 is equivalent to multiplying by 0x20.
                // Push back into the ordinals array if the bit is set.
                ptr := add(ptr, shl(5, and(t, 1)))
                o := add(o, 1)
                t := shr(o, roles)
                if iszero(t) { break }
            }
            // Store the length of `ordinals`.
            mstore(ordinals, shr(5, sub(ptr, add(ordinals, 0x20))))
            // Allocate the memory.
            mstore(0x40, ptr)
        }
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                  PUBLIC UPDATE FUNCTIONS                   */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Allows the owner to grant `user` `roles`.
    /// If the `user` already has a role, then it will be an no-op for the role.
    function grantRoles(address user, uint256 roles) public payable virtual onlyOwner {
        _grantRoles(user, roles);
    }

    /// @dev Allows the owner to remove `user` `roles`.
    /// If the `user` does not have a role, then it will be an no-op for the role.
    function revokeRoles(address user, uint256 roles) public payable virtual onlyOwner {
        _removeRoles(user, roles);
    }

    /// @dev Allow the caller to remove their own roles.
    /// If the caller does not have a role, then it will be an no-op for the role.
    function renounceRoles(uint256 roles) public payable virtual {
        _removeRoles(msg.sender, roles);
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                   PUBLIC READ FUNCTIONS                    */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Returns the roles of `user`.
    function rolesOf(address user) public view virtual returns (uint256 roles) {
        /// @solidity memory-safe-assembly
        assembly {
            // Compute the role slot.
            mstore(0x0c, _ROLE_SLOT_SEED)
            mstore(0x00, user)
            // Load the stored value.
            roles := sload(keccak256(0x0c, 0x20))
        }
    }

    /// @dev Returns whether `user` has any of `roles`.
    function hasAnyRole(address user, uint256 roles) public view virtual returns (bool) {
        return rolesOf(user) & roles != 0;
    }

    /// @dev Returns whether `user` has all of `roles`.
    function hasAllRoles(address user, uint256 roles) public view virtual returns (bool) {
        return rolesOf(user) & roles == roles;
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                         MODIFIERS                          */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Marks a function as only callable by an account with `roles`.
    modifier onlyRoles(uint256 roles) virtual {
        _checkRoles(roles);
        _;
    }

    /// @dev Marks a function as only callable by the owner or by an account
    /// with `roles`. Checks for ownership first, then lazily checks for roles.
    modifier onlyOwnerOrRoles(uint256 roles) virtual {
        _checkOwnerOrRoles(roles);
        _;
    }

    /// @dev Marks a function as only callable by an account with `roles`
    /// or the owner. Checks for roles first, then lazily checks for ownership.
    modifier onlyRolesOrOwner(uint256 roles) virtual {
        _checkRolesOrOwner(roles);
        _;
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                       ROLE CONSTANTS                       */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    // IYKYK

    uint256 internal constant _ROLE_0 = 1 << 0;
    uint256 internal constant _ROLE_1 = 1 << 1;
    uint256 internal constant _ROLE_2 = 1 << 2;
    uint256 internal constant _ROLE_3 = 1 << 3;
    uint256 internal constant _ROLE_4 = 1 << 4;
    uint256 internal constant _ROLE_5 = 1 << 5;
    uint256 internal constant _ROLE_6 = 1 << 6;
    uint256 internal constant _ROLE_7 = 1 << 7;
    uint256 internal constant _ROLE_8 = 1 << 8;
    uint256 internal constant _ROLE_9 = 1 << 9;
    uint256 internal constant _ROLE_10 = 1 << 10;
    uint256 internal constant _ROLE_11 = 1 << 11;
    uint256 internal constant _ROLE_12 = 1 << 12;
    uint256 internal constant _ROLE_13 = 1 << 13;
    uint256 internal constant _ROLE_14 = 1 << 14;
    uint256 internal constant _ROLE_15 = 1 << 15;
    uint256 internal constant _ROLE_16 = 1 << 16;
    uint256 internal constant _ROLE_17 = 1 << 17;
    uint256 internal constant _ROLE_18 = 1 << 18;
    uint256 internal constant _ROLE_19 = 1 << 19;
    uint256 internal constant _ROLE_20 = 1 << 20;
    uint256 internal constant _ROLE_21 = 1 << 21;
    uint256 internal constant _ROLE_22 = 1 << 22;
    uint256 internal constant _ROLE_23 = 1 << 23;
    uint256 internal constant _ROLE_24 = 1 << 24;
    uint256 internal constant _ROLE_25 = 1 << 25;
    uint256 internal constant _ROLE_26 = 1 << 26;
    uint256 internal constant _ROLE_27 = 1 << 27;
    uint256 internal constant _ROLE_28 = 1 << 28;
    uint256 internal constant _ROLE_29 = 1 << 29;
    uint256 internal constant _ROLE_30 = 1 << 30;
    uint256 internal constant _ROLE_31 = 1 << 31;
    uint256 internal constant _ROLE_32 = 1 << 32;
    uint256 internal constant _ROLE_33 = 1 << 33;
    uint256 internal constant _ROLE_34 = 1 << 34;
    uint256 internal constant _ROLE_35 = 1 << 35;
    uint256 internal constant _ROLE_36 = 1 << 36;
    uint256 internal constant _ROLE_37 = 1 << 37;
    uint256 internal constant _ROLE_38 = 1 << 38;
    uint256 internal constant _ROLE_39 = 1 << 39;
    uint256 internal constant _ROLE_40 = 1 << 40;
    uint256 internal constant _ROLE_41 = 1 << 41;
    uint256 internal constant _ROLE_42 = 1 << 42;
    uint256 internal constant _ROLE_43 = 1 << 43;
    uint256 internal constant _ROLE_44 = 1 << 44;
    uint256 internal constant _ROLE_45 = 1 << 45;
    uint256 internal constant _ROLE_46 = 1 << 46;
    uint256 internal constant _ROLE_47 = 1 << 47;
    uint256 internal constant _ROLE_48 = 1 << 48;
    uint256 internal constant _ROLE_49 = 1 << 49;
    uint256 internal constant _ROLE_50 = 1 << 50;
    uint256 internal constant _ROLE_51 = 1 << 51;
    uint256 internal constant _ROLE_52 = 1 << 52;
    uint256 internal constant _ROLE_53 = 1 << 53;
    uint256 internal constant _ROLE_54 = 1 << 54;
    uint256 internal constant _ROLE_55 = 1 << 55;
    uint256 internal constant _ROLE_56 = 1 << 56;
    uint256 internal constant _ROLE_57 = 1 << 57;
    uint256 internal constant _ROLE_58 = 1 << 58;
    uint256 internal constant _ROLE_59 = 1 << 59;
    uint256 internal constant _ROLE_60 = 1 << 60;
    uint256 internal constant _ROLE_61 = 1 << 61;
    uint256 internal constant _ROLE_62 = 1 << 62;
    uint256 internal constant _ROLE_63 = 1 << 63;
    uint256 internal constant _ROLE_64 = 1 << 64;
    uint256 internal constant _ROLE_65 = 1 << 65;
    uint256 internal constant _ROLE_66 = 1 << 66;
    uint256 internal constant _ROLE_67 = 1 << 67;
    uint256 internal constant _ROLE_68 = 1 << 68;
    uint256 internal constant _ROLE_69 = 1 << 69;
    uint256 internal constant _ROLE_70 = 1 << 70;
    uint256 internal constant _ROLE_71 = 1 << 71;
    uint256 internal constant _ROLE_72 = 1 << 72;
    uint256 internal constant _ROLE_73 = 1 << 73;
    uint256 internal constant _ROLE_74 = 1 << 74;
    uint256 internal constant _ROLE_75 = 1 << 75;
    uint256 internal constant _ROLE_76 = 1 << 76;
    uint256 internal constant _ROLE_77 = 1 << 77;
    uint256 internal constant _ROLE_78 = 1 << 78;
    uint256 internal constant _ROLE_79 = 1 << 79;
    uint256 internal constant _ROLE_80 = 1 << 80;
    uint256 internal constant _ROLE_81 = 1 << 81;
    uint256 internal constant _ROLE_82 = 1 << 82;
    uint256 internal constant _ROLE_83 = 1 << 83;
    uint256 internal constant _ROLE_84 = 1 << 84;
    uint256 internal constant _ROLE_85 = 1 << 85;
    uint256 internal constant _ROLE_86 = 1 << 86;
    uint256 internal constant _ROLE_87 = 1 << 87;
    uint256 internal constant _ROLE_88 = 1 << 88;
    uint256 internal constant _ROLE_89 = 1 << 89;
    uint256 internal constant _ROLE_90 = 1 << 90;
    uint256 internal constant _ROLE_91 = 1 << 91;
    uint256 internal constant _ROLE_92 = 1 << 92;
    uint256 internal constant _ROLE_93 = 1 << 93;
    uint256 internal constant _ROLE_94 = 1 << 94;
    uint256 internal constant _ROLE_95 = 1 << 95;
    uint256 internal constant _ROLE_96 = 1 << 96;
    uint256 internal constant _ROLE_97 = 1 << 97;
    uint256 internal constant _ROLE_98 = 1 << 98;
    uint256 internal constant _ROLE_99 = 1 << 99;
    uint256 internal constant _ROLE_100 = 1 << 100;
    uint256 internal constant _ROLE_101 = 1 << 101;
    uint256 internal constant _ROLE_102 = 1 << 102;
    uint256 internal constant _ROLE_103 = 1 << 103;
    uint256 internal constant _ROLE_104 = 1 << 104;
    uint256 internal constant _ROLE_105 = 1 << 105;
    uint256 internal constant _ROLE_106 = 1 << 106;
    uint256 internal constant _ROLE_107 = 1 << 107;
    uint256 internal constant _ROLE_108 = 1 << 108;
    uint256 internal constant _ROLE_109 = 1 << 109;
    uint256 internal constant _ROLE_110 = 1 << 110;
    uint256 internal constant _ROLE_111 = 1 << 111;
    uint256 internal constant _ROLE_112 = 1 << 112;
    uint256 internal constant _ROLE_113 = 1 << 113;
    uint256 internal constant _ROLE_114 = 1 << 114;
    uint256 internal constant _ROLE_115 = 1 << 115;
    uint256 internal constant _ROLE_116 = 1 << 116;
    uint256 internal constant _ROLE_117 = 1 << 117;
    uint256 internal constant _ROLE_118 = 1 << 118;
    uint256 internal constant _ROLE_119 = 1 << 119;
    uint256 internal constant _ROLE_120 = 1 << 120;
    uint256 internal constant _ROLE_121 = 1 << 121;
    uint256 internal constant _ROLE_122 = 1 << 122;
    uint256 internal constant _ROLE_123 = 1 << 123;
    uint256 internal constant _ROLE_124 = 1 << 124;
    uint256 internal constant _ROLE_125 = 1 << 125;
    uint256 internal constant _ROLE_126 = 1 << 126;
    uint256 internal constant _ROLE_127 = 1 << 127;
    uint256 internal constant _ROLE_128 = 1 << 128;
    uint256 internal constant _ROLE_129 = 1 << 129;
    uint256 internal constant _ROLE_130 = 1 << 130;
    uint256 internal constant _ROLE_131 = 1 << 131;
    uint256 internal constant _ROLE_132 = 1 << 132;
    uint256 internal constant _ROLE_133 = 1 << 133;
    uint256 internal constant _ROLE_134 = 1 << 134;
    uint256 internal constant _ROLE_135 = 1 << 135;
    uint256 internal constant _ROLE_136 = 1 << 136;
    uint256 internal constant _ROLE_137 = 1 << 137;
    uint256 internal constant _ROLE_138 = 1 << 138;
    uint256 internal constant _ROLE_139 = 1 << 139;
    uint256 internal constant _ROLE_140 = 1 << 140;
    uint256 internal constant _ROLE_141 = 1 << 141;
    uint256 internal constant _ROLE_142 = 1 << 142;
    uint256 internal constant _ROLE_143 = 1 << 143;
    uint256 internal constant _ROLE_144 = 1 << 144;
    uint256 internal constant _ROLE_145 = 1 << 145;
    uint256 internal constant _ROLE_146 = 1 << 146;
    uint256 internal constant _ROLE_147 = 1 << 147;
    uint256 internal constant _ROLE_148 = 1 << 148;
    uint256 internal constant _ROLE_149 = 1 << 149;
    uint256 internal constant _ROLE_150 = 1 << 150;
    uint256 internal constant _ROLE_151 = 1 << 151;
    uint256 internal constant _ROLE_152 = 1 << 152;
    uint256 internal constant _ROLE_153 = 1 << 153;
    uint256 internal constant _ROLE_154 = 1 << 154;
    uint256 internal constant _ROLE_155 = 1 << 155;
    uint256 internal constant _ROLE_156 = 1 << 156;
    uint256 internal constant _ROLE_157 = 1 << 157;
    uint256 internal constant _ROLE_158 = 1 << 158;
    uint256 internal constant _ROLE_159 = 1 << 159;
    uint256 internal constant _ROLE_160 = 1 << 160;
    uint256 internal constant _ROLE_161 = 1 << 161;
    uint256 internal constant _ROLE_162 = 1 << 162;
    uint256 internal constant _ROLE_163 = 1 << 163;
    uint256 internal constant _ROLE_164 = 1 << 164;
    uint256 internal constant _ROLE_165 = 1 << 165;
    uint256 internal constant _ROLE_166 = 1 << 166;
    uint256 internal constant _ROLE_167 = 1 << 167;
    uint256 internal constant _ROLE_168 = 1 << 168;
    uint256 internal constant _ROLE_169 = 1 << 169;
    uint256 internal constant _ROLE_170 = 1 << 170;
    uint256 internal constant _ROLE_171 = 1 << 171;
    uint256 internal constant _ROLE_172 = 1 << 172;
    uint256 internal constant _ROLE_173 = 1 << 173;
    uint256 internal constant _ROLE_174 = 1 << 174;
    uint256 internal constant _ROLE_175 = 1 << 175;
    uint256 internal constant _ROLE_176 = 1 << 176;
    uint256 internal constant _ROLE_177 = 1 << 177;
    uint256 internal constant _ROLE_178 = 1 << 178;
    uint256 internal constant _ROLE_179 = 1 << 179;
    uint256 internal constant _ROLE_180 = 1 << 180;
    uint256 internal constant _ROLE_181 = 1 << 181;
    uint256 internal constant _ROLE_182 = 1 << 182;
    uint256 internal constant _ROLE_183 = 1 << 183;
    uint256 internal constant _ROLE_184 = 1 << 184;
    uint256 internal constant _ROLE_185 = 1 << 185;
    uint256 internal constant _ROLE_186 = 1 << 186;
    uint256 internal constant _ROLE_187 = 1 << 187;
    uint256 internal constant _ROLE_188 = 1 << 188;
    uint256 internal constant _ROLE_189 = 1 << 189;
    uint256 internal constant _ROLE_190 = 1 << 190;
    uint256 internal constant _ROLE_191 = 1 << 191;
    uint256 internal constant _ROLE_192 = 1 << 192;
    uint256 internal constant _ROLE_193 = 1 << 193;
    uint256 internal constant _ROLE_194 = 1 << 194;
    uint256 internal constant _ROLE_195 = 1 << 195;
    uint256 internal constant _ROLE_196 = 1 << 196;
    uint256 internal constant _ROLE_197 = 1 << 197;
    uint256 internal constant _ROLE_198 = 1 << 198;
    uint256 internal constant _ROLE_199 = 1 << 199;
    uint256 internal constant _ROLE_200 = 1 << 200;
    uint256 internal constant _ROLE_201 = 1 << 201;
    uint256 internal constant _ROLE_202 = 1 << 202;
    uint256 internal constant _ROLE_203 = 1 << 203;
    uint256 internal constant _ROLE_204 = 1 << 204;
    uint256 internal constant _ROLE_205 = 1 << 205;
    uint256 internal constant _ROLE_206 = 1 << 206;
    uint256 internal constant _ROLE_207 = 1 << 207;
    uint256 internal constant _ROLE_208 = 1 << 208;
    uint256 internal constant _ROLE_209 = 1 << 209;
    uint256 internal constant _ROLE_210 = 1 << 210;
    uint256 internal constant _ROLE_211 = 1 << 211;
    uint256 internal constant _ROLE_212 = 1 << 212;
    uint256 internal constant _ROLE_213 = 1 << 213;
    uint256 internal constant _ROLE_214 = 1 << 214;
    uint256 internal constant _ROLE_215 = 1 << 215;
    uint256 internal constant _ROLE_216 = 1 << 216;
    uint256 internal constant _ROLE_217 = 1 << 217;
    uint256 internal constant _ROLE_218 = 1 << 218;
    uint256 internal constant _ROLE_219 = 1 << 219;
    uint256 internal constant _ROLE_220 = 1 << 220;
    uint256 internal constant _ROLE_221 = 1 << 221;
    uint256 internal constant _ROLE_222 = 1 << 222;
    uint256 internal constant _ROLE_223 = 1 << 223;
    uint256 internal constant _ROLE_224 = 1 << 224;
    uint256 internal constant _ROLE_225 = 1 << 225;
    uint256 internal constant _ROLE_226 = 1 << 226;
    uint256 internal constant _ROLE_227 = 1 << 227;
    uint256 internal constant _ROLE_228 = 1 << 228;
    uint256 internal constant _ROLE_229 = 1 << 229;
    uint256 internal constant _ROLE_230 = 1 << 230;
    uint256 internal constant _ROLE_231 = 1 << 231;
    uint256 internal constant _ROLE_232 = 1 << 232;
    uint256 internal constant _ROLE_233 = 1 << 233;
    uint256 internal constant _ROLE_234 = 1 << 234;
    uint256 internal constant _ROLE_235 = 1 << 235;
    uint256 internal constant _ROLE_236 = 1 << 236;
    uint256 internal constant _ROLE_237 = 1 << 237;
    uint256 internal constant _ROLE_238 = 1 << 238;
    uint256 internal constant _ROLE_239 = 1 << 239;
    uint256 internal constant _ROLE_240 = 1 << 240;
    uint256 internal constant _ROLE_241 = 1 << 241;
    uint256 internal constant _ROLE_242 = 1 << 242;
    uint256 internal constant _ROLE_243 = 1 << 243;
    uint256 internal constant _ROLE_244 = 1 << 244;
    uint256 internal constant _ROLE_245 = 1 << 245;
    uint256 internal constant _ROLE_246 = 1 << 246;
    uint256 internal constant _ROLE_247 = 1 << 247;
    uint256 internal constant _ROLE_248 = 1 << 248;
    uint256 internal constant _ROLE_249 = 1 << 249;
    uint256 internal constant _ROLE_250 = 1 << 250;
    uint256 internal constant _ROLE_251 = 1 << 251;
    uint256 internal constant _ROLE_252 = 1 << 252;
    uint256 internal constant _ROLE_253 = 1 << 253;
    uint256 internal constant _ROLE_254 = 1 << 254;
    uint256 internal constant _ROLE_255 = 1 << 255;
}

pragma solidity 0.8.27;

import {IGTELaunchpadV2Pair} from "../uniswap/interfaces/IGTELaunchpadV2Pair.sol";

import {UserRewardData, RewardPoolDataMemory} from "../libraries/RewardsTracker.sol";

interface IDistributor {
    function getUserData(address launchAsset, address account) external view returns (UserRewardData memory);
    function getUserDataForTokens(address[] calldata launchAssets, address account)
        external
        view
        returns (UserRewardData[] memory);
    function increaseStake(address launchAsset, address account, uint96 shares)
        external
        returns (uint256 baseAmount, uint256 quoteAmount);
    function decreaseStake(address launchAsset, address account, uint96 shares)
        external
        returns (uint256 baseAmount, uint256 quoteAmount);
    function claimRewards(address launchAsset) external returns (uint256 baseAmount, uint256 quoteAmount);
    function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external;
    function createRewardsPair(address launchAsset, address quoteToken) external;

    function endRewards(IGTELaunchpadV2Pair pair) external;
}

pragma solidity 0.8.27;

import "./interfaces/IUniswapV2ERC20.sol";
import "./libraries/SafeMath.sol";

contract UniswapV2ERC20 is IUniswapV2ERC20 {
    using SafeMath for uint256;

    string public constant name = "Uniswap V2";
    string public constant symbol = "UNI-V2";
    uint8 public constant decimals = 18;
    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    bytes32 public DOMAIN_SEPARATOR;
    // keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");
    bytes32 public constant PERMIT_TYPEHASH = 0x6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9;
    mapping(address => uint256) public nonces;

    constructor() {
        uint256 chainId;
        assembly {
            chainId := chainid()
        }
        DOMAIN_SEPARATOR = keccak256(
            abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256(bytes(name)),
                keccak256(bytes("1")),
                chainId,
                address(this)
            )
        );
    }

    function _mint(address to, uint256 value) internal {
        totalSupply = totalSupply.add(value);
        balanceOf[to] = balanceOf[to].add(value);
        emit Transfer(address(0), to, value);
    }

    function _burn(address from, uint256 value) internal {
        balanceOf[from] = balanceOf[from].sub(value);
        totalSupply = totalSupply.sub(value);
        emit Transfer(from, address(0), value);
    }

    function _approve(address owner, address spender, uint256 value) private {
        allowance[owner][spender] = value;
        emit Approval(owner, spender, value);
    }

    function _transfer(address from, address to, uint256 value) private {
        balanceOf[from] = balanceOf[from].sub(value);
        balanceOf[to] = balanceOf[to].add(value);
        emit Transfer(from, to, value);
    }

    function approve(address spender, uint256 value) external returns (bool) {
        _approve(msg.sender, spender, value);
        return true;
    }

    function transfer(address to, uint256 value) external returns (bool) {
        _transfer(msg.sender, to, value);
        return true;
    }

    function transferFrom(address from, address to, uint256 value) external returns (bool) {
        if (allowance[from][msg.sender] != type(uint256).max) {
            allowance[from][msg.sender] = allowance[from][msg.sender].sub(value);
        }
        _transfer(from, to, value);
        return true;
    }

    function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s)
        external
    {
        require(deadline >= block.timestamp, "UniswapV2: EXPIRED");
        bytes32 digest = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(abi.encode(PERMIT_TYPEHASH, owner, spender, value, nonces[owner]++, deadline))
            )
        );
        address recoveredAddress = ecrecover(digest, v, r, s);
        require(recoveredAddress != address(0) && recoveredAddress == owner, "UniswapV2: INVALID_SIGNATURE");
        _approve(owner, spender, value);
    }
}

pragma solidity 0.8.27;

interface IUniswapV2Pair {
    event Mint(address indexed sender, uint256 amount0, uint256 amount1);
    event Burn(address indexed sender, uint256 amount0, uint256 amount1, address indexed to);
    event Swap(
        address indexed sender,
        uint256 amount0In,
        uint256 amount1In,
        uint256 amount0Out,
        uint256 amount1Out,
        address indexed to
    );
    event Sync(uint112 reserve0, uint112 reserve1);

    function MINIMUM_LIQUIDITY() external pure returns (uint256);
    function factory() external view returns (address);
    function token0() external view returns (address);
    function token1() external view returns (address);
    function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
    function price0CumulativeLast() external view returns (uint256);
    function price1CumulativeLast() external view returns (uint256);
    function kLast() external view returns (uint256);

    function mint(address to) external returns (uint256 liquidity);
    function burn(address to) external returns (uint256 amount0, uint256 amount1);
    function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external;
    function skim(address to) external;
    function sync() external;

    function initialize(address, address, address, address) external;
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

interface IGTELaunchpadV2Pair {
    function rewardsPoolActive() external view returns (uint256);
    function accruedLaunchpadFee0() external view returns (uint112);
    function accruedLaunchpadFee1() external view returns (uint112);
    function launchpadLp() external view returns (address);
    function launchpadFeeDistributor() external view returns (address);
    function REWARDS_FEE_SHARE() external view returns (uint256);

    function endRewardsAccrual() external;
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

struct UserRewardData {
    uint96 shares; // User's current share count (up to ~7.9e28)
    uint96 baseRewardDebt; // Used to calculate base token rewards owed
    uint96 quoteRewardDebt; // Used to calculate quote token rewards owed
}

struct RewardPoolData {
    // SLOT 0 //
    uint96 totalShares; // Sum of all user shares
    address quoteAsset; // Secondary reward token
    // SLOT 1 //
    uint128 pendingBaseRewards;
    uint128 pendingQuoteRewards;
    // SLOT 2 //
    uint256 accBaseRewardPerShare; // Accumulated base rewards per share, scaled by 1e12
    uint256 accQuoteRewardPerShare; // Accumulated quote rewards per share, scaled by 1e12
    // SLOT 3 //
    mapping(address => UserRewardData) userRewards; // User-specific reward data
}

struct RewardPoolDataMemory {
    uint96 totalShares; // Sum of all user shares
    address quoteAsset; // Secondary reward token
    uint128 pendingBaseRewards;
    uint128 pendingQuoteRewards; //
    uint256 accBaseRewardPerShare;
    uint256 accQuoteRewardPerShare; // Accumulated quote rewards per share, scaled by 1e12
}

using RewardsTrackerLib for RewardPoolData global;
/**
 * @title RewardsLibrary
 * @dev Library with internal functions for pro rata reward distribution
 */

library RewardsTrackerLib {
    /// @dev sig: 0x9511e79574c9aa195c27c3455b60ba70c9a6efbcfc431ae68b8a3cb4d3764f6c
    event PairRewardsInitialized(address indexed baseAsset, address indexed quoteAsset);
    /// @dev sig: 0x2cbe0649bcb43ba4ace580eeeb0c95a516dec93862fe4cc4e7e60528575cec67
    event BaseRewardsAdded(address indexed baseAsset, uint256 amount);
    /// @dev sig: 0x28590542f9792ca8533cd1beac50e724892009d1f19ed17351f264be124d3293
    event QuoteRewardsAdded(address indexed baseAsset, address indexed quoteAsset, uint256 amount);

    /// @dev sig: 0xe3e46b04
    error ZeroShareStake();
    /// @dev sig: 0xe331bd04
    error ZeroShareClaim();
    /// @dev sig: 0x39996567
    error InsufficientShares();

    // Scale factor used for fixed-point math
    uint128 public constant PRECISION_FACTOR = 1e12;

    function getQuoteAsset(RewardPoolData storage self) internal view returns (address) {
        return self.quoteAsset;
    }

    function getUserData(RewardPoolData storage self, address account) internal view returns (UserRewardData memory) {
        return self.userRewards[account];
    }

    function getRewardsPoolData(RewardPoolData storage self) internal view returns (RewardPoolDataMemory memory pm) {
        pm = RewardPoolDataMemory({
            quoteAsset: self.quoteAsset,
            totalShares: self.totalShares,
            pendingBaseRewards: self.pendingBaseRewards,
            pendingQuoteRewards: self.pendingQuoteRewards,
            accBaseRewardPerShare: self.accBaseRewardPerShare,
            accQuoteRewardPerShare: self.accQuoteRewardPerShare
        });
    }

    function initializePair(RewardPoolData storage self, address baseAsset, address quoteAsset) internal {
        self.quoteAsset = quoteAsset;
        emit PairRewardsInitialized(baseAsset, quoteAsset);
    }

    function addBaseRewards(RewardPoolData storage self, address baseAsset, uint128 amount) internal {
        self.pendingBaseRewards += amount;
        emit BaseRewardsAdded(baseAsset, amount);
    }

    function addQuoteRewards(RewardPoolData storage self, address baseAsset, address quoteAsset, uint128 amount)
        internal
    {
        self.pendingQuoteRewards += amount;
        emit QuoteRewardsAdded(baseAsset, quoteAsset, amount);
    }

    function stake(RewardPoolData storage self, address user, uint96 newShares)
        internal
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        if (newShares == 0) revert ZeroShareStake();

        (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();

        UserRewardData storage userData = self.userRewards[user];

        uint256 existingShares = uint96(userData.shares);

        // Calculate pending rewards before updating shares
        if (existingShares > 0) {
            baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
            quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;
        }

        // Update user shares
        userData.shares += newShares;
        self.totalShares += newShares;

        // Update reward debts
        userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
        userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));
    }

    function unstake(RewardPoolData storage self, address user, uint96 removeShares)
        internal
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();

        UserRewardData storage userData = self.userRewards[user];

        if (removeShares == 0) revert ZeroShareStake();

        uint256 existingShares = uint256(userData.shares);
        if (existingShares < removeShares) revert InsufficientShares();

        // Calculate pending rewards before updating shares
        baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
        quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;

        // Update user shares
        userData.shares -= removeShares;
        self.totalShares -= removeShares;

        // Update reward debts
        userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
        userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));
    }

    function claim(RewardPoolData storage self, address user)
        internal
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        UserRewardData storage userData = self.userRewards[user];
        uint256 shares = uint256(userData.shares);

        if (shares == 0) revert ZeroShareClaim();

        (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();

        // Calculate pending rewards
        uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
        uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);

        baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
        quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);

        // Update reward debts
        userData.baseRewardDebt = uint96(totalAccBaseRewards);
        userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
    }

    function getPendingRewards(RewardPoolData storage self, address user)
        internal
        view
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = getAccRewardsPerShare(self);

        UserRewardData storage userData = self.userRewards[user];
        uint256 shares = uint256(userData.shares);

        // Unstaking claims pending rewards, so if no shares, then no pending
        if (shares == 0) return (0, 0);

        baseAmount = totalAccRewards(shares, accBaseRewardsPerShare) - uint128(userData.baseRewardDebt);
        quoteAmount = totalAccRewards(shares, accQuoteRewardsPerShare) - uint128(userData.quoteRewardDebt);
    }

    function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
        return (shares * accRewardsPerShare) / PRECISION_FACTOR;
    }

    /// @dev Applies the new accrued rewards per share to the rewards state
    function update(RewardPoolData storage self)
        internal
        returns (uint256 newAccBaseRewardsPerShare, uint256 newAccQuoteRewardsPerShare)
    {
        (newAccBaseRewardsPerShare, newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self);

        if (self.pendingBaseRewards > 0) {
            self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
            delete self.pendingBaseRewards;
        }

        if (self.pendingQuoteRewards > 0) {
            self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
            delete self.pendingQuoteRewards;
        }
    }

    /// @dev Gets the new accrued rewards per share without updating rewards state
    function getAccRewardsPerShare(RewardPoolData storage self)
        internal
        view
        returns (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare)
    {
        uint96 totalShares = self.totalShares;
        if (totalShares == 0) return (self.accBaseRewardPerShare, self.accQuoteRewardPerShare);

        accBaseRewardsPerShare = self.accBaseRewardPerShare;
        accQuoteRewardsPerShare = self.accQuoteRewardPerShare;

        if (self.pendingBaseRewards > 0) {
            accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
        }

        if (self.pendingQuoteRewards > 0) {
            accQuoteRewardsPerShare += ((self.pendingQuoteRewards * PRECISION_FACTOR) / uint128(totalShares));
        }
    }
}

/**
 * @title RewardsStorage
 * @dev Storage library for rewards distribution using EIP-1967 pattern
 */
library RewardsTrackerStorage {
    bytes32 internal constant LAUNCH_ASSET_TO_REWARDS_SLOT =
        keccak256(abi.encode(uint256(keccak256("rewardsTrackerPool.self.slot")) - 1)) & ~bytes32(uint256(0xff));

    function rewardPoolSlot(address baseAsset) private pure returns (bytes32) {
        return keccak256(abi.encodePacked(baseAsset, LAUNCH_ASSET_TO_REWARDS_SLOT));
    }

    function getRewardPool(address baseAsset) internal pure returns (RewardPoolData storage p) {
        bytes32 slot = rewardPoolSlot(baseAsset);
        assembly {
            p.slot := slot
        }
    }
}

pragma solidity 0.8.27;

import {IGTELaunchpadV2Pair} from "../uniswap/interfaces/IGTELaunchpadV2Pair.sol";

import {UserRewardData, RewardPoolDataMemory} from "../libraries/RewardsTracker.sol";

interface IDistributor {
    function getUserData(address launchAsset, address account) external view returns (UserRewardData memory);
    function getUserDataForTokens(address[] calldata launchAssets, address account)
        external
        view
        returns (UserRewardData[] memory);
    function increaseStake(address launchAsset, address account, uint96 shares)
        external
        returns (uint256 baseAmount, uint256 quoteAmount);
    function decreaseStake(address launchAsset, address account, uint96 shares)
        external
        returns (uint256 baseAmount, uint256 quoteAmount);
    function claimRewards(address launchAsset) external returns (uint256 baseAmount, uint256 quoteAmount);
    function addRewards(address token0, address token1, uint128 amount0, uint128 amount1) external;
    function createRewardsPair(address launchAsset, address quoteToken) external;

    function endRewards(IGTELaunchpadV2Pair pair) external;
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

struct UserRewardData {
    uint96 shares; // User's current share count (up to ~7.9e28)
    uint96 baseRewardDebt; // Used to calculate base token rewards owed
    uint96 quoteRewardDebt; // Used to calculate quote token rewards owed
}

struct RewardPoolData {
    // SLOT 0 //
    uint96 totalShares; // Sum of all user shares
    address quoteAsset; // Secondary reward token
    // SLOT 1 //
    uint128 pendingBaseRewards;
    uint128 pendingQuoteRewards;
    // SLOT 2 //
    uint256 accBaseRewardPerShare; // Accumulated base rewards per share, scaled by 1e12
    uint256 accQuoteRewardPerShare; // Accumulated quote rewards per share, scaled by 1e12
    // SLOT 3 //
    mapping(address => UserRewardData) userRewards; // User-specific reward data
}

struct RewardPoolDataMemory {
    uint96 totalShares; // Sum of all user shares
    address quoteAsset; // Secondary reward token
    uint128 pendingBaseRewards;
    uint128 pendingQuoteRewards; //
    uint256 accBaseRewardPerShare;
    uint256 accQuoteRewardPerShare; // Accumulated quote rewards per share, scaled by 1e12
}

using RewardsTrackerLib for RewardPoolData global;
/**
 * @title RewardsLibrary
 * @dev Library with internal functions for pro rata reward distribution
 */

library RewardsTrackerLib {
    /// @dev sig: 0x9511e79574c9aa195c27c3455b60ba70c9a6efbcfc431ae68b8a3cb4d3764f6c
    event PairRewardsInitialized(address indexed baseAsset, address indexed quoteAsset);
    /// @dev sig: 0x2cbe0649bcb43ba4ace580eeeb0c95a516dec93862fe4cc4e7e60528575cec67
    event BaseRewardsAdded(address indexed baseAsset, uint256 amount);
    /// @dev sig: 0x28590542f9792ca8533cd1beac50e724892009d1f19ed17351f264be124d3293
    event QuoteRewardsAdded(address indexed baseAsset, address indexed quoteAsset, uint256 amount);

    /// @dev sig: 0xe3e46b04
    error ZeroShareStake();
    /// @dev sig: 0xe331bd04
    error ZeroShareClaim();
    /// @dev sig: 0x39996567
    error InsufficientShares();

    // Scale factor used for fixed-point math
    uint128 public constant PRECISION_FACTOR = 1e12;

    function getQuoteAsset(RewardPoolData storage self) internal view returns (address) {
        return self.quoteAsset;
    }

    function getUserData(RewardPoolData storage self, address account) internal view returns (UserRewardData memory) {
        return self.userRewards[account];
    }

    function getRewardsPoolData(RewardPoolData storage self) internal view returns (RewardPoolDataMemory memory pm) {
        pm = RewardPoolDataMemory({
            quoteAsset: self.quoteAsset,
            totalShares: self.totalShares,
            pendingBaseRewards: self.pendingBaseRewards,
            pendingQuoteRewards: self.pendingQuoteRewards,
            accBaseRewardPerShare: self.accBaseRewardPerShare,
            accQuoteRewardPerShare: self.accQuoteRewardPerShare
        });
    }

    function initializePair(RewardPoolData storage self, address baseAsset, address quoteAsset) internal {
        self.quoteAsset = quoteAsset;
        emit PairRewardsInitialized(baseAsset, quoteAsset);
    }

    function addBaseRewards(RewardPoolData storage self, address baseAsset, uint128 amount) internal {
        self.pendingBaseRewards += amount;
        emit BaseRewardsAdded(baseAsset, amount);
    }

    function addQuoteRewards(RewardPoolData storage self, address baseAsset, address quoteAsset, uint128 amount)
        internal
    {
        self.pendingQuoteRewards += amount;
        emit QuoteRewardsAdded(baseAsset, quoteAsset, amount);
    }

    function stake(RewardPoolData storage self, address user, uint96 newShares)
        internal
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        if (newShares == 0) revert ZeroShareStake();

        (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();

        UserRewardData storage userData = self.userRewards[user];

        uint256 existingShares = uint96(userData.shares);

        // Calculate pending rewards before updating shares
        if (existingShares > 0) {
            baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
            quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;
        }

        // Update user shares
        userData.shares += newShares;
        self.totalShares += newShares;

        // Update reward debts
        userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares, accBaseRewardsPerShare));
        userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares, accQuoteRewardsPerShare));
    }

    function unstake(RewardPoolData storage self, address user, uint96 removeShares)
        internal
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();

        UserRewardData storage userData = self.userRewards[user];

        if (removeShares == 0) revert ZeroShareStake();

        uint256 existingShares = uint256(userData.shares);
        if (existingShares < removeShares) revert InsufficientShares();

        // Calculate pending rewards before updating shares
        baseAmount = totalAccRewards(existingShares, accBaseRewardsPerShare) - userData.baseRewardDebt;
        quoteAmount = totalAccRewards(existingShares, accQuoteRewardsPerShare) - userData.quoteRewardDebt;

        // Update user shares
        userData.shares -= removeShares;
        self.totalShares -= removeShares;

        // Update reward debts
        userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accBaseRewardsPerShare));
        userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares, accQuoteRewardsPerShare));
    }

    function claim(RewardPoolData storage self, address user)
        internal
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        UserRewardData storage userData = self.userRewards[user];
        uint256 shares = uint256(userData.shares);

        if (shares == 0) revert ZeroShareClaim();

        (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();

        // Calculate pending rewards
        uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
        uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);

        baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
        quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);

        // Update reward debts
        userData.baseRewardDebt = uint96(totalAccBaseRewards);
        userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
    }

    function getPendingRewards(RewardPoolData storage self, address user)
        internal
        view
        returns (uint256 baseAmount, uint256 quoteAmount)
    {
        (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = getAccRewardsPerShare(self);

        UserRewardData storage userData = self.userRewards[user];
        uint256 shares = uint256(userData.shares);

        // Unstaking claims pending rewards, so if no shares, then no pending
        if (shares == 0) return (0, 0);

        baseAmount = totalAccRewards(shares, accBaseRewardsPerShare) - uint128(userData.baseRewardDebt);
        quoteAmount = totalAccRewards(shares, accQuoteRewardsPerShare) - uint128(userData.quoteRewardDebt);
    }

    function totalAccRewards(uint256 shares, uint256 accRewardsPerShare) internal pure returns (uint256) {
        return (shares * accRewardsPerShare) / PRECISION_FACTOR;
    }

    /// @dev Applies the new accrued rewards per share to the rewards state
    function update(RewardPoolData storage self)
        internal
        returns (uint256 newAccBaseRewardsPerShare, uint256 newAccQuoteRewardsPerShare)
    {
        (newAccBaseRewardsPerShare, newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self);

        if (self.pendingBaseRewards > 0) {
            self.accBaseRewardPerShare = newAccBaseRewardsPerShare;
            delete self.pendingBaseRewards;
        }

        if (self.pendingQuoteRewards > 0) {
            self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare;
            delete self.pendingQuoteRewards;
        }
    }

    /// @dev Gets the new accrued rewards per share without updating rewards state
    function getAccRewardsPerShare(RewardPoolData storage self)
        internal
        view
        returns (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare)
    {
        uint96 totalShares = self.totalShares;
        if (totalShares == 0) return (self.accBaseRewardPerShare, self.accQuoteRewardPerShare);

        accBaseRewardsPerShare = self.accBaseRewardPerShare;
        accQuoteRewardsPerShare = self.accQuoteRewardPerShare;

        if (self.pendingBaseRewards > 0) {
            accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares));
        }

        if (self.pendingQuoteRewards > 0) {
            accQuoteRewardsPerShare += ((self.pendingQuoteRewards * PRECISION_FACTOR) / uint128(totalShares));
        }
    }
}

/**
 * @title RewardsStorage
 * @dev Storage library for rewards distribution using EIP-1967 pattern
 */
library RewardsTrackerStorage {
    bytes32 internal constant LAUNCH_ASSET_TO_REWARDS_SLOT =
        keccak256(abi.encode(uint256(keccak256("rewardsTrackerPool.self.slot")) - 1)) & ~bytes32(uint256(0xff));

    function rewardPoolSlot(address baseAsset) private pure returns (bytes32) {
        return keccak256(abi.encodePacked(baseAsset, LAUNCH_ASSET_TO_REWARDS_SLOT));
    }

    function getRewardPool(address baseAsset) internal pure returns (RewardPoolData storage p) {
        bytes32 slot = rewardPoolSlot(baseAsset);
        assembly {
            p.slot := slot
        }
    }
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES
pragma solidity 0.8.27;

interface IUniswapV2Callee {
    function uniswapV2Call(address sender, uint256 amount0, uint256 amount1, bytes calldata data) external;
}

pragma solidity 0.8.27;

// a library for handling binary fixed point numbers (https://en.wikipedia.org/wiki/Q_(number_format))

// range: [0, 2**112 - 1]
// resolution: 1 / 2**112

library UQ112x112 {
    uint224 constant Q112 = 2 ** 112;

    // encode a uint112 as a UQ112x112
    function encode(uint112 y) internal pure returns (uint224 z) {
        z = uint224(y) * Q112; // never overflows
    }

    // divide a UQ112x112 by a uint112, returning a UQ112x112
    function uqdiv(uint224 x, uint112 y) internal pure returns (uint224 z) {
        z = x / uint224(y);
    }
}

pragma solidity 0.8.27;

interface IERC20 {
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event Transfer(address indexed from, address indexed to, uint256 value);

    function name() external view returns (string memory);
    function symbol() external view returns (string memory);
    function decimals() external view returns (uint8);
    function totalSupply() external view returns (uint256);
    function balanceOf(address owner) external view returns (uint256);
    function allowance(address owner, address spender) external view returns (uint256);

    function approve(address spender, uint256 value) external returns (bool);
    function transfer(address to, uint256 value) external returns (bool);
    function transferFrom(address from, address to, uint256 value) external returns (bool);
}

pragma solidity 0.8.27;

interface IUniswapV2Factory {
    event PairCreated(address indexed token0, address indexed token1, address pair, uint256);

    function feeTo() external view returns (address);
    function feeToSetter() external view returns (address);

    function getPair(address tokenA, address tokenB) external view returns (address pair);
    function allPairs(uint256) external view returns (address pair);
    function allPairsLength() external view returns (uint256);

    function createPair(address tokenA, address tokenB) external returns (address pair);

    function setFeeTo(address) external;
    function setFeeToSetter(address) external;
}

pragma solidity 0.8.27;

// a library for performing various math operations

library Math {
    function min(uint256 x, uint256 y) internal pure returns (uint256 z) {
        z = x < y ? x : y;
    }

    // babylonian method (https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method)
    function sqrt(uint256 y) internal pure returns (uint256 z) {
        if (y > 3) {
            z = y;
            uint256 x = y / 2 + 1;
            while (x < z) {
                z = x;
                x = (y / x + x) / 2;
            }
        } else if (y != 0) {
            z = 1;
        }
    }
}


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

