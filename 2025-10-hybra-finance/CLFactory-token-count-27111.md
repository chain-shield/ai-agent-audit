
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity =0.7.6;

import "./interfaces/ICLFactory.sol";
import "./interfaces/fees/IFeeModule.sol";

import "./interfaces/IGaugeManager.sol";
import "./interfaces/IFactoryRegistry.sol";
import "@openzeppelin/contracts/proxy/Clones.sol";
import "@nomad-xyz/src/ExcessivelySafeCall.sol";
import "./CLPool.sol";

/// @title Canonical CL factory
/// @notice Deploys CL pools and manages ownership and control over pool protocol fees
contract CLFactory is ICLFactory {
    using ExcessivelySafeCall for address;

    /// @inheritdoc ICLFactory
    IGaugeManager public override gaugeManager;
    /// @inheritdoc ICLFactory
    address public immutable override poolImplementation;
    /// @inheritdoc ICLFactory
    address public override owner;
    /// @inheritdoc ICLFactory
    address public override swapFeeManager;
    /// @inheritdoc ICLFactory
    address public override swapFeeModule;
    /// @inheritdoc ICLFactory
    address public override unstakedFeeManager;
    /// @inheritdoc ICLFactory
    address public override unstakedFeeModule;
    /// @inheritdoc ICLFactory
    uint24 public override defaultUnstakedFee;
    /// @inheritdoc ICLFactory

    address public override protocolFeeManager;
    /// @inheritdoc ICLFactory
    address public override protocolFeeModule;
    /// @inheritdoc ICLFactory
    uint24 public override defaultProtocolFee;

    mapping(int24 => uint24) public override tickSpacingToFee;
    /// @inheritdoc ICLFactory
    mapping(address => mapping(address => mapping(int24 => address))) public override getPool;
    /// @dev Used in VotingEscrow to determine if a contract is a valid pool
    mapping(address => bool) private _isPool;
    /// @inheritdoc ICLFactory
    address[] public override allPools;

    int24[] private _tickSpacings;

    constructor(address _poolImplementation) {
        owner = msg.sender;
        swapFeeManager = msg.sender;
        unstakedFeeManager = msg.sender;
        protocolFeeManager = msg.sender;
        poolImplementation = _poolImplementation;
        defaultUnstakedFee = 100_000;
        defaultProtocolFee = 250_000;
        emit OwnerChanged(address(0), msg.sender);
        emit SwapFeeManagerChanged(address(0), msg.sender);
        emit UnstakedFeeManagerChanged(address(0), msg.sender);
        emit DefaultUnstakedFeeChanged(0, 100_000);

        enableTickSpacing(1, 100);
        enableTickSpacing(50, 500);
        enableTickSpacing(100, 500);
        enableTickSpacing(200, 3_000);
        enableTickSpacing(2_000, 10_000);
    }

    function setGaugeManager(address _gaugeManager) external {
        require(msg.sender == owner);
        gaugeManager = IGaugeManager(_gaugeManager);
    }

    /// @inheritdoc ICLFactory
    function createPool(address tokenA, address tokenB, int24 tickSpacing, uint160 sqrtPriceX96)
        external
        override
        returns (address pool)
    {
        require(tokenA != tokenB);
        (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
        require(token0 != address(0));
        require(tickSpacingToFee[tickSpacing] != 0);
        require(getPool[token0][token1][tickSpacing] == address(0));
        pool = Clones.cloneDeterministic({
            master: poolImplementation,
            salt: keccak256(abi.encode(token0, token1, tickSpacing))
        });
        CLPool(pool).initialize({
            _factory: address(this),
            _token0: token0,
            _token1: token1,
            _tickSpacing: tickSpacing,
            _gaugeManager: address(gaugeManager),
            _sqrtPriceX96: sqrtPriceX96
        });
        allPools.push(pool);
        _isPool[pool] = true;
        getPool[token0][token1][tickSpacing] = pool;
        // populate mapping in the reverse direction, deliberate choice to avoid the cost of comparing addresses
        getPool[token1][token0][tickSpacing] = pool;
        emit PoolCreated(token0, token1, tickSpacing, pool);
    }

    /// @inheritdoc ICLFactory
    function setOwner(address _owner) external override {
        address cachedOwner = owner;
        require(msg.sender == cachedOwner);
        require(_owner != address(0));
        emit OwnerChanged(cachedOwner, _owner);
        owner = _owner;
    }

    /// @inheritdoc ICLFactory
    function setSwapFeeManager(address _swapFeeManager) external override {
        address cachedSwapFeeManager = swapFeeManager;
        require(msg.sender == cachedSwapFeeManager);
        require(_swapFeeManager != address(0));
        swapFeeManager = _swapFeeManager;
        emit SwapFeeManagerChanged(cachedSwapFeeManager, _swapFeeManager);
    }

    /// @inheritdoc ICLFactory
    function setUnstakedFeeManager(address _unstakedFeeManager) external override {
        address cachedUnstakedFeeManager = unstakedFeeManager;
        require(msg.sender == cachedUnstakedFeeManager);
        require(_unstakedFeeManager != address(0));
        unstakedFeeManager = _unstakedFeeManager;
        emit UnstakedFeeManagerChanged(cachedUnstakedFeeManager, _unstakedFeeManager);
    }

    /// @inheritdoc ICLFactory
    function setSwapFeeModule(address _swapFeeModule) external override {
        require(msg.sender == swapFeeManager);
        require(_swapFeeModule != address(0));
        address oldFeeModule = swapFeeModule;
        swapFeeModule = _swapFeeModule;
        emit SwapFeeModuleChanged(oldFeeModule, _swapFeeModule);
    }

    /// @inheritdoc ICLFactory
    function setUnstakedFeeModule(address _unstakedFeeModule) external override {
        require(msg.sender == unstakedFeeManager);
        require(_unstakedFeeModule != address(0));
        address oldFeeModule = unstakedFeeModule;
        unstakedFeeModule = _unstakedFeeModule;
        emit UnstakedFeeModuleChanged(oldFeeModule, _unstakedFeeModule);
    }

    /// @inheritdoc ICLFactory
    function setDefaultUnstakedFee(uint24 _defaultUnstakedFee) external override {
        require(msg.sender == unstakedFeeManager);
        require(_defaultUnstakedFee <= 500_000);
        uint24 oldUnstakedFee = defaultUnstakedFee;
        defaultUnstakedFee = _defaultUnstakedFee;
        emit DefaultUnstakedFeeChanged(oldUnstakedFee, _defaultUnstakedFee);
    }


    function setProtocolFeeModule(address _protocolFeeModule) external override {
        require(msg.sender == protocolFeeManager);
        require(_protocolFeeModule != address(0));
        protocolFeeModule = _protocolFeeModule;
    }

    function setProtocolFeeManager(address _protocolFeeManager) external override {
        require(msg.sender == protocolFeeManager);
        require(_protocolFeeManager != address(0));
        protocolFeeManager = _protocolFeeManager;
    }

    /// @inheritdoc ICLFactory
    function getSwapFee(address pool) external view override returns (uint24) {
        if (swapFeeModule != address(0)) {
            (bool success, bytes memory data) = swapFeeModule.excessivelySafeStaticCall(
                200_000, 32, abi.encodeWithSelector(IFeeModule.getFee.selector, pool)
            );
            if (success) {
                uint24 fee = abi.decode(data, (uint24));
                if (fee <= 100_000) {
                    return fee;
                }
            }
        }
        return tickSpacingToFee[CLPool(pool).tickSpacing()];
    }

    /// @inheritdoc ICLFactory
    function getUnstakedFee(address pool) external view override returns (uint24) {
        
        if (!gaugeManager.isGaugeAliveForPool(pool)) {
            return 0;
        }
        if (unstakedFeeModule != address(0)) {
            (bool success, bytes memory data) = unstakedFeeModule.excessivelySafeStaticCall(
                200_000, 32, abi.encodeWithSelector(IFeeModule.getFee.selector, pool)
            );
            if (success) {
                uint24 fee = abi.decode(data, (uint24));
                if (fee <= 1_000_000) {
                    return fee;
                }
            }
        }
        return defaultUnstakedFee;
    }

    function getProtocolFee(address pool) external view override returns (uint24) {
        // if the gauge is alive, return 0, protocol fee is only for inactive gauges
        if (gaugeManager.isGaugeAliveForPool(pool)) {
            return 0;
        }

        if (protocolFeeModule != address(0)) {
            (bool success, bytes memory data) = protocolFeeModule.excessivelySafeStaticCall(
                200_000, 32, abi.encodeWithSelector(IFeeModule.getFee.selector, pool)
            );
            if (success) {
                uint24 fee = abi.decode(data, (uint24));
                if (fee <= 500_000) {
                    return fee;
                }
            }
        }
        return defaultProtocolFee;
    }

    /// @inheritdoc ICLFactory
    function enableTickSpacing(int24 tickSpacing, uint24 fee) public override {
        require(msg.sender == owner);
        require(fee > 0 && fee <= 100_000);
        // tick spacing is capped at 16384 to prevent the situation where tickSpacing is so large that
        // TickBitmap#nextInitializedTickWithinOneWord overflows int24 container from a valid tick
        // 16384 ticks represents a >5x price change with ticks of 1 bips
        require(tickSpacing > 0 && tickSpacing < 16384);
        require(tickSpacingToFee[tickSpacing] == 0);

        tickSpacingToFee[tickSpacing] = fee;
        _tickSpacings.push(tickSpacing);
        emit TickSpacingEnabled(tickSpacing, fee);
    }

    function collectAllProtocolFees() external  {
        require(msg.sender == owner);

        for (uint256 i = 0; i < allPools.length; i++) {
            CLPool(allPools[i]).collectProtocolFees(msg.sender);
        }
    }

    function collectProtocolFees(address pool) external returns (uint128 amount0, uint128 amount1) {
        require(msg.sender == owner);
        (amount0, amount1) = CLPool(pool).collectProtocolFees(msg.sender);
    }

    /// @inheritdoc ICLFactory
    function tickSpacings() external view override returns (int24[] memory) {
        return _tickSpacings;
    }

    /// @inheritdoc ICLFactory
    function allPoolsLength() external view override returns (uint256) {
        return allPools.length;
    }

    /// @inheritdoc ICLFactory
    function isPool(address pool) external view override returns (bool) {
        return _isPool[pool];
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

interface IGaugeManager {
    
    struct FarmingParam {
        address farmingCenter;
        address algebraEternalFarming;
        address nfpm;
    }

    function isGaugeAliveForPool(address _pool) external view returns (bool);
    function gauges(address _pair) external view returns (address);
    function isGauge(address _gauge) external view returns (bool);
    function poolForGauge(address _gauge) external view returns (address);
}
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.5.0 <0.8.0;

/// @title Math library for computing sqrt prices from ticks and vice versa
/// @notice Computes sqrt price for ticks of size 1.0001, i.e. sqrt(1.0001^tick) as fixed point Q64.96 numbers. Supports
/// prices between 2**-128 and 2**128
library TickMath {
    /// @dev The minimum tick that may be passed to #getSqrtRatioAtTick computed from log base 1.0001 of 2**-128
    int24 internal constant MIN_TICK = -887272;
    /// @dev The maximum tick that may be passed to #getSqrtRatioAtTick computed from log base 1.0001 of 2**128
    int24 internal constant MAX_TICK = -MIN_TICK;

    /// @dev The minimum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MIN_TICK)
    uint160 internal constant MIN_SQRT_RATIO = 4295128739;
    /// @dev The maximum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MAX_TICK)
    uint160 internal constant MAX_SQRT_RATIO = 1461446703485210103287273052203988822378723970342;

    /// @notice Calculates sqrt(1.0001^tick) * 2^96
    /// @dev Throws if |tick| > max tick
    /// @param tick The input tick for the above formula
    /// @return sqrtPriceX96 A Fixed point Q64.96 number representing the sqrt of the ratio of the two assets (token1/token0)
    /// at the given tick
    function getSqrtRatioAtTick(int24 tick) internal pure returns (uint160 sqrtPriceX96) {
        uint256 absTick = tick < 0 ? uint256(-int256(tick)) : uint256(int256(tick));
        require(absTick <= uint256(MAX_TICK), "T");

        uint256 ratio = absTick & 0x1 != 0 ? 0xfffcb933bd6fad37aa2d162d1a594001 : 0x100000000000000000000000000000000;
        if (absTick & 0x2 != 0) ratio = (ratio * 0xfff97272373d413259a46990580e213a) >> 128;
        if (absTick & 0x4 != 0) ratio = (ratio * 0xfff2e50f5f656932ef12357cf3c7fdcc) >> 128;
        if (absTick & 0x8 != 0) ratio = (ratio * 0xffe5caca7e10e4e61c3624eaa0941cd0) >> 128;
        if (absTick & 0x10 != 0) ratio = (ratio * 0xffcb9843d60f6159c9db58835c926644) >> 128;
        if (absTick & 0x20 != 0) ratio = (ratio * 0xff973b41fa98c081472e6896dfb254c0) >> 128;
        if (absTick & 0x40 != 0) ratio = (ratio * 0xff2ea16466c96a3843ec78b326b52861) >> 128;
        if (absTick & 0x80 != 0) ratio = (ratio * 0xfe5dee046a99a2a811c461f1969c3053) >> 128;
        if (absTick & 0x100 != 0) ratio = (ratio * 0xfcbe86c7900a88aedcffc83b479aa3a4) >> 128;
        if (absTick & 0x200 != 0) ratio = (ratio * 0xf987a7253ac413176f2b074cf7815e54) >> 128;
        if (absTick & 0x400 != 0) ratio = (ratio * 0xf3392b0822b70005940c7a398e4b70f3) >> 128;
        if (absTick & 0x800 != 0) ratio = (ratio * 0xe7159475a2c29b7443b29c7fa6e889d9) >> 128;
        if (absTick & 0x1000 != 0) ratio = (ratio * 0xd097f3bdfd2022b8845ad8f792aa5825) >> 128;
        if (absTick & 0x2000 != 0) ratio = (ratio * 0xa9f746462d870fdf8a65dc1f90e061e5) >> 128;
        if (absTick & 0x4000 != 0) ratio = (ratio * 0x70d869a156d2a1b890bb3df62baf32f7) >> 128;
        if (absTick & 0x8000 != 0) ratio = (ratio * 0x31be135f97d08fd981231505542fcfa6) >> 128;
        if (absTick & 0x10000 != 0) ratio = (ratio * 0x9aa508b5b7a84e1c677de54f3e99bc9) >> 128;
        if (absTick & 0x20000 != 0) ratio = (ratio * 0x5d6af8dedb81196699c329225ee604) >> 128;
        if (absTick & 0x40000 != 0) ratio = (ratio * 0x2216e584f5fa1ea926041bedfe98) >> 128;
        if (absTick & 0x80000 != 0) ratio = (ratio * 0x48a170391f7dc42444e8fa2) >> 128;

        if (tick > 0) ratio = type(uint256).max / ratio;

        // this divides by 1<<32 rounding up to go from a Q128.128 to a Q128.96.
        // we then downcast because we know the result always fits within 160 bits due to our tick input constraint
        // we round up in the division so getTickAtSqrtRatio of the output price is always consistent
        sqrtPriceX96 = uint160((ratio >> 32) + (ratio % (1 << 32) == 0 ? 0 : 1));
    }

    /// @notice Calculates the greatest tick value such that getRatioAtTick(tick) <= ratio
    /// @dev Throws in case sqrtPriceX96 < MIN_SQRT_RATIO, as MIN_SQRT_RATIO is the lowest value getRatioAtTick may
    /// ever return.
    /// @param sqrtPriceX96 The sqrt ratio for which to compute the tick as a Q64.96
    /// @return tick The greatest tick for which the ratio is less than or equal to the input ratio
    function getTickAtSqrtRatio(uint160 sqrtPriceX96) internal pure returns (int24 tick) {
        // second inequality must be < because the price can never reach the price at the max tick
        require(sqrtPriceX96 >= MIN_SQRT_RATIO && sqrtPriceX96 < MAX_SQRT_RATIO, "R");
        uint256 ratio = uint256(sqrtPriceX96) << 32;

        uint256 r = ratio;
        uint256 msb = 0;

        assembly {
            let f := shl(7, gt(r, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF))
            msb := or(msb, f)
            r := shr(f, r)
        }
        assembly {
            let f := shl(6, gt(r, 0xFFFFFFFFFFFFFFFF))
            msb := or(msb, f)
            r := shr(f, r)
        }
        assembly {
            let f := shl(5, gt(r, 0xFFFFFFFF))
            msb := or(msb, f)
            r := shr(f, r)
        }
        assembly {
            let f := shl(4, gt(r, 0xFFFF))
            msb := or(msb, f)
            r := shr(f, r)
        }
        assembly {
            let f := shl(3, gt(r, 0xFF))
            msb := or(msb, f)
            r := shr(f, r)
        }
        assembly {
            let f := shl(2, gt(r, 0xF))
            msb := or(msb, f)
            r := shr(f, r)
        }
        assembly {
            let f := shl(1, gt(r, 0x3))
            msb := or(msb, f)
            r := shr(f, r)
        }
        assembly {
            let f := gt(r, 0x1)
            msb := or(msb, f)
        }

        if (msb >= 128) r = ratio >> (msb - 127);
        else r = ratio << (127 - msb);

        int256 log_2 = (int256(msb) - 128) << 64;

        assembly {
            r := shr(127, mul(r, r))
            let f := shr(128, r)
            log_2 := or(log_2, shl(63, f))
            r := shr(f, r)
        }
        assembly {
            r := shr(127, mul(r, r))
            let f := shr(128, r)
            log_2 := or(log_2, shl(62, f))
            r := shr(f, r)
        }
        assembly {
            r := shr(127, mul(r, r))
            let f := shr(128, r)
            log_2 := or(log_2, shl(61, f))
            r := shr(f, r)
        }
        assembly {
            r := shr(127, mul(r, r))
            let f := shr(128, r)
            log_2 := or(log_2, shl(60, f))
            r := shr(f, r)
        }
        assembly {
            r := shr(127, mul(r, r))
            let f := shr(128, r)
            log_2 := or(log_2, shl(59, f))
            r := shr(f, r)
        }
        assembly {
            r := shr(127, mul(r, r))
            let f := shr(128, r)
            log_2 := or(log_2, shl(58, f))
            r := shr(f, r)
        }
        assembly {
            r := shr(127, mul(r, r))
            let f := shr(128, r)
            log_2 := or(log_2, shl(57, f))
            r := shr(f, r)
        }
        assembly {
            r := shr(127, mul(r, r))
            let f := shr(128, r)
            log_2 := or(log_2, shl(56, f))
            r := shr(f, r)
        }
        assembly {
            r := shr(127, mul(r, r))
            let f := shr(128, r)
            log_2 := or(log_2, shl(55, f))
            r := shr(f, r)
        }
        assembly {
            r := shr(127, mul(r, r))
            let f := shr(128, r)
            log_2 := or(log_2, shl(54, f))
            r := shr(f, r)
        }
        assembly {
            r := shr(127, mul(r, r))
            let f := shr(128, r)
            log_2 := or(log_2, shl(53, f))
            r := shr(f, r)
        }
        assembly {
            r := shr(127, mul(r, r))
            let f := shr(128, r)
            log_2 := or(log_2, shl(52, f))
            r := shr(f, r)
        }
        assembly {
            r := shr(127, mul(r, r))
            let f := shr(128, r)
            log_2 := or(log_2, shl(51, f))
            r := shr(f, r)
        }
        assembly {
            r := shr(127, mul(r, r))
            let f := shr(128, r)
            log_2 := or(log_2, shl(50, f))
        }

        int256 log_sqrt10001 = log_2 * 255738958999603826347141; // 128.128 number

        int24 tickLow = int24((log_sqrt10001 - 3402992956809132418596140100660247210) >> 128);
        int24 tickHi = int24((log_sqrt10001 + 291339464771989622907027621153398088495) >> 128);

        tick = tickLow == tickHi ? tickLow : getSqrtRatioAtTick(tickHi) <= sqrtPriceX96 ? tickHi : tickLow;
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity =0.7.6;

import "./interfaces/ICLPool.sol";

import "./libraries/LowGasSafeMath.sol";
import "./libraries/SafeCast.sol";
import "./libraries/Tick.sol";
import "./libraries/TickBitmap.sol";
import "./libraries/Position.sol";
import "./libraries/Oracle.sol";

import "./libraries/FullMath.sol";
import "./libraries/FixedPoint128.sol";
import "./libraries/TransferHelper.sol";
import "./libraries/TickMath.sol";
import "./libraries/LiquidityMath.sol";
import "./libraries/SqrtPriceMath.sol";
import "./libraries/SwapMath.sol";

import "./interfaces/ICLFactory.sol";
import "./interfaces/IERC20Minimal.sol";
import "./interfaces/callback/ICLMintCallback.sol";
import "./interfaces/callback/ICLSwapCallback.sol";
import "./interfaces/callback/ICLFlashCallback.sol";

contract CLPool is ICLPool {
    using LowGasSafeMath for uint256;
    using LowGasSafeMath for int256;
    using SafeCast for uint256;
    using SafeCast for int256;
    using Tick for mapping(int24 => Tick.Info);
    using TickBitmap for mapping(int16 => uint256);
    using Position for mapping(bytes32 => Position.Info);
    using Position for Position.Info;
    using Oracle for Oracle.Observation[65535];

    /// @inheritdoc ICLPoolConstants
    address public override factory;
    /// @inheritdoc ICLPoolConstants
    address public override token0;
    /// @inheritdoc ICLPoolConstants
    address public override token1;
    /// @inheritdoc ICLPoolConstants
    address public override gauge;
    /// @inheritdoc ICLPoolConstants
    address public override nft;
    // address public override factoryRegistry;
    address internal  gaugeManager;

    struct Slot0 {
        // the current price
        uint160 sqrtPriceX96;
        // the current tick
        int24 tick;
        // the most-recently updated index of the observations array
        uint16 observationIndex;
        // the current maximum number of observations that are being stored
        uint16 observationCardinality;
        // the next maximum number of observations to store, triggered in observations.write
        uint16 observationCardinalityNext;
        // whether the pool is locked
        bool unlocked;
    }

    /// @inheritdoc ICLPoolState
    Slot0 public override slot0;

    /// @inheritdoc ICLPoolState
    uint256 public override feeGrowthGlobal0X128;
    /// @inheritdoc ICLPoolState
    uint256 public override feeGrowthGlobal1X128;

    /// @inheritdoc ICLPoolState
    uint256 public override rewardGrowthGlobalX128;

    // accumulated gauge fees in token0/token1 units
    struct GaugeFees {
        uint128 token0;
        uint128 token1;
    }

    struct ProtocolFees {
        uint128 token0;
        uint128 token1;
    }

    /// @inheritdoc ICLPoolState
    GaugeFees public override gaugeFees;

    ProtocolFees private protocolFees;

    /// @inheritdoc ICLPoolState
    uint256 public override rewardRate;
    /// @inheritdoc ICLPoolState
    uint256 public override rewardReserve;
    /// @inheritdoc ICLPoolState
    uint256 public override periodFinish;
    /// @inheritdoc ICLPoolState
    uint256 public override rollover;

    /// @inheritdoc ICLPoolState
    uint128 public override stakedLiquidity;
    /// @inheritdoc ICLPoolState
    uint32 public override lastUpdated;
    /// @inheritdoc ICLPoolConstants
    int24 public override tickSpacing;

    /// @inheritdoc ICLPoolState
    uint128 public override liquidity;
    /// @inheritdoc ICLPoolConstants
    uint128 public override maxLiquidityPerTick;

    /// @inheritdoc ICLPoolState
    mapping(int24 => Tick.Info) public override ticks;
    /// @inheritdoc ICLPoolState
    mapping(int16 => uint256) public override tickBitmap;
    /// @inheritdoc ICLPoolState
    mapping(bytes32 => Position.Info) public override positions;
    /// @inheritdoc ICLPoolState
    Oracle.Observation[65535] public override observations;

    /// @dev Mutually exclusive reentrancy protection into the pool to/from a method. This method also prevents entrance
    /// to a function before the pool is initialized. The reentrancy guard is required throughout the contract because
    /// we use balance checks to determine the payment status of interactions such as mint, swap and flash.
    modifier lock() {
        require(slot0.unlocked, "LK");
        slot0.unlocked = false;
        _;
        slot0.unlocked = true;
    }

    /// @dev Prevents calling a function from anyone except the gauge associated with this pool
    modifier onlyGauge() {
        require(msg.sender == gauge, "NG");
        _;
    }

    /// @dev Prevents calling a function from anyone except the nft manager
    modifier onlyNftManager() {
        require(msg.sender == nft, "NM");
        _;
    }

    /// @inheritdoc ICLPoolActions
    function initialize(
        address _factory,
        address _token0,
        address _token1,
        int24 _tickSpacing,
        address _gaugeManager,
        uint160 _sqrtPriceX96
    ) external override {
        require(factory == address(0) && _factory != address(0));
        factory = _factory;
        token0 = _token0;
        token1 = _token1;
        tickSpacing = _tickSpacing;
        gaugeManager = _gaugeManager;
        maxLiquidityPerTick = Tick.tickSpacingToMaxLiquidityPerTick(_tickSpacing);

        int24 tick = TickMath.getTickAtSqrtRatio(_sqrtPriceX96);

        (uint16 cardinality, uint16 cardinalityNext) = observations.initialize(_blockTimestamp());

        slot0 = Slot0({
            sqrtPriceX96: _sqrtPriceX96,
            tick: tick,
            observationIndex: 0,
            observationCardinality: cardinality,
            observationCardinalityNext: cardinalityNext,
            unlocked: true
        });

        emit Initialize(_sqrtPriceX96, tick);
    }

    function fee() public view override returns (uint24) {
        return ICLFactory(factory).getSwapFee(address(this));
    }

    function unstakedFee() public view override returns (uint24) {
        return ICLFactory(factory).getUnstakedFee(address(this));
    }

    function protocolFee() internal view  returns (uint24) {
        return ICLFactory(factory).getProtocolFee(address(this));
    }

    /// @dev Common checks for valid tick inputs.
    function checkTicks(int24 tickLower, int24 tickUpper) private pure {
        require(tickLower < tickUpper, "TLU");
        require(tickLower >= TickMath.MIN_TICK, "TLM");
        require(tickUpper <= TickMath.MAX_TICK, "TUM");
    }

    /// @dev Returns the block timestamp truncated to 32 bits, i.e. mod 2**32. This method is overridden in tests.
    function _blockTimestamp() internal view virtual returns (uint32) {
        return uint32(block.timestamp); // truncation is desired
    }

    /// @dev Get the pool's balance of token0
    /// @dev This function is gas optimized to avoid a redundant extcodesize check in addition to the returndatasize
    /// check
    function balance0() private view returns (uint256) {
        (bool success, bytes memory data) =
            token0.staticcall(abi.encodeWithSelector(IERC20Minimal.balanceOf.selector, address(this)));
        require(success && data.length >= 32);
        return abi.decode(data, (uint256));
    }

    /// @dev Get the pool's balance of token1
    /// @dev This function is gas optimized to avoid a redundant extcodesize check in addition to the returndatasize
    /// check
    function balance1() private view returns (uint256) {
        (bool success, bytes memory data) =
            token1.staticcall(abi.encodeWithSelector(IERC20Minimal.balanceOf.selector, address(this)));
        require(success && data.length >= 32);
        return abi.decode(data, (uint256));
    }

    /// @inheritdoc ICLPoolDerivedState
    function snapshotCumulativesInside(int24 tickLower, int24 tickUpper)
        external
        view
        override
        returns (int56 tickCumulativeInside, uint160 secondsPerLiquidityInsideX128, uint32 secondsInside)
    {
        checkTicks(tickLower, tickUpper);

        int56 tickCumulativeLower;
        int56 tickCumulativeUpper;
        uint160 secondsPerLiquidityOutsideLowerX128;
        uint160 secondsPerLiquidityOutsideUpperX128;
        uint32 secondsOutsideLower;
        uint32 secondsOutsideUpper;

        {
            Tick.Info storage lower = ticks[tickLower];
            Tick.Info storage upper = ticks[tickUpper];
            bool initializedLower;
            (tickCumulativeLower, secondsPerLiquidityOutsideLowerX128, secondsOutsideLower, initializedLower) = (
                lower.tickCumulativeOutside,
                lower.secondsPerLiquidityOutsideX128,
                lower.secondsOutside,
                lower.initialized
            );
            require(initializedLower);

            bool initializedUpper;
            (tickCumulativeUpper, secondsPerLiquidityOutsideUpperX128, secondsOutsideUpper, initializedUpper) = (
                upper.tickCumulativeOutside,
                upper.secondsPerLiquidityOutsideX128,
                upper.secondsOutside,
                upper.initialized
            );
            require(initializedUpper);
        }

        Slot0 memory _slot0 = slot0;

        if (_slot0.tick < tickLower) {
            return (
                tickCumulativeLower - tickCumulativeUpper,
                secondsPerLiquidityOutsideLowerX128 - secondsPerLiquidityOutsideUpperX128,
                secondsOutsideLower - secondsOutsideUpper
            );
        } else if (_slot0.tick < tickUpper) {
            uint32 time = _blockTimestamp();
            (int56 tickCumulative, uint160 secondsPerLiquidityCumulativeX128) = observations.observeSingle(
                time, 0, _slot0.tick, _slot0.observationIndex, liquidity, _slot0.observationCardinality
            );
            return (
                tickCumulative - tickCumulativeLower - tickCumulativeUpper,
                secondsPerLiquidityCumulativeX128 - secondsPerLiquidityOutsideLowerX128
                    - secondsPerLiquidityOutsideUpperX128,
                time - secondsOutsideLower - secondsOutsideUpper
            );
        } else {
            return (
                tickCumulativeUpper - tickCumulativeLower,
                secondsPerLiquidityOutsideUpperX128 - secondsPerLiquidityOutsideLowerX128,
                secondsOutsideUpper - secondsOutsideLower
            );
        }
    }

    /// @inheritdoc ICLPoolDerivedState
    function observe(uint32[] calldata secondsAgos)
        external
        view
        override
        returns (int56[] memory tickCumulatives, uint160[] memory secondsPerLiquidityCumulativeX128s)
    {
        return observations.observe(
            _blockTimestamp(), secondsAgos, slot0.tick, slot0.observationIndex, liquidity, slot0.observationCardinality
        );
    }

    /// @inheritdoc ICLPoolActions
    function increaseObservationCardinalityNext(uint16 observationCardinalityNext) external override lock {
        uint16 observationCardinalityNextOld = slot0.observationCardinalityNext; // for the event
        uint16 observationCardinalityNextNew =
            observations.grow(observationCardinalityNextOld, observationCardinalityNext);
        slot0.observationCardinalityNext = observationCardinalityNextNew;
        if (observationCardinalityNextOld != observationCardinalityNextNew) {
            emit IncreaseObservationCardinalityNext(observationCardinalityNextOld, observationCardinalityNextNew);
        }
    }

    struct ModifyPositionParams {
        // the address that owns the position
        address owner;
        // the lower and upper tick of the position
        int24 tickLower;
        int24 tickUpper;
        // any change in liquidity
        int128 liquidityDelta;
    }

    /// @dev Effect some changes to a position
    /// @param params the position details and the change to the position's liquidity to effect
    /// @return position a storage pointer referencing the position with the given owner and tick range
    /// @return amount0 the amount of token0 owed to the pool, negative if the pool should pay the recipient
    /// @return amount1 the amount of token1 owed to the pool, negative if the pool should pay the recipient
    function _modifyPosition(ModifyPositionParams memory params)
        private
        returns (Position.Info storage position, int256 amount0, int256 amount1)
    {
        checkTicks(params.tickLower, params.tickUpper);

        Slot0 memory _slot0 = slot0; // SLOAD for gas optimization

        position = _updatePosition(params.owner, params.tickLower, params.tickUpper, params.liquidityDelta, _slot0.tick);

        if (params.liquidityDelta != 0) {
            if (_slot0.tick < params.tickLower) {
                // current tick is below the passed range; liquidity can only become in range by crossing from left to
                // right, when we'll need _more_ token0 (it's becoming more valuable) so user must provide it
                amount0 = SqrtPriceMath.getAmount0Delta(
                    TickMath.getSqrtRatioAtTick(params.tickLower),
                    TickMath.getSqrtRatioAtTick(params.tickUpper),
                    params.liquidityDelta
                );
            } else if (_slot0.tick < params.tickUpper) {
                // current tick is inside the passed range
                uint128 liquidityBefore = liquidity; // SLOAD for gas optimization

                // write an oracle entry
                (slot0.observationIndex, slot0.observationCardinality) = observations.write(
                    _slot0.observationIndex,
                    _blockTimestamp(),
                    _slot0.tick,
                    liquidityBefore,
                    _slot0.observationCardinality,
                    _slot0.observationCardinalityNext
                );

                amount0 = SqrtPriceMath.getAmount0Delta(
                    _slot0.sqrtPriceX96, TickMath.getSqrtRatioAtTick(params.tickUpper), params.liquidityDelta
                );
                amount1 = SqrtPriceMath.getAmount1Delta(
                    TickMath.getSqrtRatioAtTick(params.tickLower), _slot0.sqrtPriceX96, params.liquidityDelta
                );

                liquidity = LiquidityMath.addDelta(liquidityBefore, params.liquidityDelta);
            } else {
                // current tick is above the passed range; liquidity can only become in range by crossing from right to
                // left, when we'll need _more_ token1 (it's becoming more valuable) so user must provide it
                amount1 = SqrtPriceMath.getAmount1Delta(
                    TickMath.getSqrtRatioAtTick(params.tickLower),
                    TickMath.getSqrtRatioAtTick(params.tickUpper),
                    params.liquidityDelta
                );
            }
        }
    }

    /// @dev Gets and updates a position with the given liquidity delta
    /// @param owner the owner of the position
    /// @param tickLower the lower tick of the position's tick range
    /// @param tickUpper the upper tick of the position's tick range
    /// @param tick the current tick, passed to avoid sloads
    function _updatePosition(address owner, int24 tickLower, int24 tickUpper, int128 liquidityDelta, int24 tick)
        private
        returns (Position.Info storage position)
    {
        position = positions.get(owner, tickLower, tickUpper);

        uint256 _feeGrowthGlobal0X128 = feeGrowthGlobal0X128; // SLOAD for gas optimization
        uint256 _feeGrowthGlobal1X128 = feeGrowthGlobal1X128; // SLOAD for gas optimization

        // if we need to update the ticks, do it
        bool flippedLower;
        bool flippedUpper;
        if (liquidityDelta != 0) {
            uint32 time = _blockTimestamp();
            (int56 tickCumulative, uint160 secondsPerLiquidityCumulativeX128) = observations.observeSingle(
                time, 0, slot0.tick, slot0.observationIndex, liquidity, slot0.observationCardinality
            );

            flippedLower = ticks.update(
                tickLower,
                tick,
                liquidityDelta,
                _feeGrowthGlobal0X128,
                _feeGrowthGlobal1X128,
                secondsPerLiquidityCumulativeX128,
                tickCumulative,
                time,
                false,
                maxLiquidityPerTick
            );
            flippedUpper = ticks.update(
                tickUpper,
                tick,
                liquidityDelta,
                _feeGrowthGlobal0X128,
                _feeGrowthGlobal1X128,
                secondsPerLiquidityCumulativeX128,
                tickCumulative,
                time,
                true,
                maxLiquidityPerTick
            );

            if (flippedLower) {
                tickBitmap.flipTick(tickLower, tickSpacing);
            }
            if (flippedUpper) {
                tickBitmap.flipTick(tickUpper, tickSpacing);
            }
        }

        (uint256 feeGrowthInside0X128, uint256 feeGrowthInside1X128) =
            ticks.getFeeGrowthInside(tickLower, tickUpper, tick, _feeGrowthGlobal0X128, _feeGrowthGlobal1X128);

        bool staked = (owner == gauge) && (owner != address(0));
        position.update(liquidityDelta, feeGrowthInside0X128, feeGrowthInside1X128, staked);

        // clear any tick data that is no longer needed
        if (liquidityDelta < 0) {
            if (flippedLower) {
                ticks.clear(tickLower);
            }
            if (flippedUpper) {
                ticks.clear(tickUpper);
            }
        }
    }

    /// @inheritdoc ICLPoolActions
    function mint(address recipient, int24 tickLower, int24 tickUpper, uint128 amount, bytes calldata data)
        external
        override
        lock
        returns (uint256 amount0, uint256 amount1)
    {
        require(amount > 0);
        (, int256 amount0Int, int256 amount1Int) = _modifyPosition(
            ModifyPositionParams({
                owner: recipient,
                tickLower: tickLower,
                tickUpper: tickUpper,
                liquidityDelta: int256(amount).toInt128()
            })
        );

        amount0 = uint256(amount0Int);
        amount1 = uint256(amount1Int);

        uint256 balance0Before;
        uint256 balance1Before;
        if (amount0 > 0) balance0Before = balance0();
        if (amount1 > 0) balance1Before = balance1();
        ICLMintCallback(msg.sender).uniswapV3MintCallback(amount0, amount1, data);
        if (amount0 > 0) require(balance0Before.add(amount0) <= balance0(), "M0");
        if (amount1 > 0) require(balance1Before.add(amount1) <= balance1(), "M1");

        emit Mint(msg.sender, recipient, tickLower, tickUpper, amount, amount0, amount1);
    }

    /// @inheritdoc ICLPoolActions
    function collect(
        address recipient,
        int24 tickLower,
        int24 tickUpper,
        uint128 amount0Requested,
        uint128 amount1Requested
    ) external override lock returns (uint128 amount0, uint128 amount1) {
        (amount0, amount1) = _collect({
            recipient: recipient,
            tickLower: tickLower,
            tickUpper: tickUpper,
            amount0Requested: amount0Requested,
            amount1Requested: amount1Requested,
            owner: msg.sender
        });
    }

    /// @inheritdoc ICLPoolActions
    function collect(
        address recipient,
        int24 tickLower,
        int24 tickUpper,
        uint128 amount0Requested,
        uint128 amount1Requested,
        address owner
    ) external override lock onlyNftManager returns (uint128 amount0, uint128 amount1) {
        (amount0, amount1) = _collect({
            recipient: recipient,
            tickLower: tickLower,
            tickUpper: tickUpper,
            amount0Requested: amount0Requested,
            amount1Requested: amount1Requested,
            owner: owner
        });
    }

    function _collect(
        address recipient,
        int24 tickLower,
        int24 tickUpper,
        uint128 amount0Requested,
        uint128 amount1Requested,
        address owner
    ) private returns (uint128 amount0, uint128 amount1) {
        // we don't need to checkTicks here, because invalid positions will never have non-zero tokensOwed{0,1}
        Position.Info storage position = positions.get(owner, tickLower, tickUpper);

        amount0 = amount0Requested > position.tokensOwed0 ? position.tokensOwed0 : amount0Requested;
        amount1 = amount1Requested > position.tokensOwed1 ? position.tokensOwed1 : amount1Requested;

        if (amount0 > 0) {
            position.tokensOwed0 -= amount0;
            TransferHelper.safeTransfer(token0, recipient, amount0);
        }
        if (amount1 > 0) {
            position.tokensOwed1 -= amount1;
            TransferHelper.safeTransfer(token1, recipient, amount1);
        }

        emit Collect(owner, recipient, tickLower, tickUpper, amount0, amount1);
    }

    /// @inheritdoc ICLPoolActions
    function burn(int24 tickLower, int24 tickUpper, uint128 amount)
        external
        override
        lock
        returns (uint256 amount0, uint256 amount1)
    {
        (amount0, amount1) = _burn({tickLower: tickLower, tickUpper: tickUpper, amount: amount, owner: msg.sender});
    }

    /// @inheritdoc ICLPoolActions
    function burn(int24 tickLower, int24 tickUpper, uint128 amount, address owner)
        external
        override
        lock
        onlyNftManager
        returns (uint256 amount0, uint256 amount1)
    {
        (amount0, amount1) = _burn({tickLower: tickLower, tickUpper: tickUpper, amount: amount, owner: owner});
    }

    function _burn(int24 tickLower, int24 tickUpper, uint128 amount, address owner)
        private
        returns (uint256 amount0, uint256 amount1)
    {
        (Position.Info storage position, int256 amount0Int, int256 amount1Int) = _modifyPosition(
            ModifyPositionParams({
                owner: owner,
                tickLower: tickLower,
                tickUpper: tickUpper,
                liquidityDelta: -int256(amount).toInt128()
            })
        );

        amount0 = uint256(-amount0Int);
        amount1 = uint256(-amount1Int);

        if (amount0 > 0 || amount1 > 0) {
            (position.tokensOwed0, position.tokensOwed1) =
                (position.tokensOwed0 + uint128(amount0), position.tokensOwed1 + uint128(amount1));
        }

        emit Burn(owner, tickLower, tickUpper, amount, amount0, amount1);
    }

    /// @inheritdoc ICLPoolActions
    function stake(int128 stakedLiquidityDelta, int24 tickLower, int24 tickUpper, bool positionUpdate)
        external
        override
        lock
        onlyGauge
    {
        int24 tick = slot0.tick;
        // Increase staked liquidity in the current tick
        if (tick >= tickLower && tick < tickUpper) {
            _updateRewardsGrowthGlobal();
            stakedLiquidity = LiquidityMath.addDelta(stakedLiquidity, stakedLiquidityDelta);
        }

        if (positionUpdate) {
            Position.Info storage nftPosition = positions.get(nft, tickLower, tickUpper);
            Position.Info storage gaugePosition = positions.get(gauge, tickLower, tickUpper);

            (uint256 feeGrowthInside0X128, uint256 feeGrowthInside1X128) =
                ticks.getFeeGrowthInside(tickLower, tickUpper, tick, feeGrowthGlobal0X128, feeGrowthGlobal1X128);

            // Assign the staked positions virtually to the gauge
            nftPosition.update(-stakedLiquidityDelta, feeGrowthInside0X128, feeGrowthInside1X128, false);
            gaugePosition.update(stakedLiquidityDelta, feeGrowthInside0X128, feeGrowthInside1X128, true);
        }

        // Update tick locations where staked liquidity needs to be added or subtracted
        // Only update ticks if current tick is initialized
        if (ticks[tickLower].initialized) ticks.updateStake(tickLower, stakedLiquidityDelta, false);
        if (ticks[tickUpper].initialized) ticks.updateStake(tickUpper, stakedLiquidityDelta, true);
    }

    struct SwapCache {
        // liquidity at the beginning of the swap
        uint128 liquidityStart;
        // staked liquidity at the beginning of the swap
        uint128 stakedLiquidityStart;
        // the timestamp of the current block
        uint32 blockTimestamp;
        // the current value of the tick accumulator, computed only if we cross an initialized tick
        int56 tickCumulative;
        // the current value of seconds per liquidity accumulator, computed only if we cross an initialized tick
        uint160 secondsPerLiquidityCumulativeX128;
        // whether we've computed and cached the above two accumulators
        bool computedLatestObservation;
    }

    // the top level state of the swap, the results of which are recorded in storage at the end
    struct SwapState {
        // the amount remaining to be swapped in/out of the input/output asset
        int256 amountSpecifiedRemaining;
        // the amount already swapped out/in of the output/input asset
        int256 amountCalculated;
        // current sqrt(price)
        uint160 sqrtPriceX96;
        // the tick associated with the current price
        int24 tick;
        // the fee associated with the pool
        uint24 fee;
        // wether we've updated the fees in the current swap
        bool hasUpdatedFees;
        // the global fee growth of the input token
        uint256 feeGrowthGlobalX128;
        // amount of input token paid as gauge fee
        uint128 gaugeFee;
        // amount of input token paid as protocol fee
        uint128 protocolFee;
        // the current liquidity in range
        uint128 liquidity;
        // the current staked liquidity in range
        uint128 stakedLiquidity;
    }

    struct StepComputations {
        // the price at the beginning of the step
        uint160 sqrtPriceStartX96;
        // the next tick to swap to from the current tick in the swap direction
        int24 tickNext;
        // whether tickNext is initialized or not
        bool initialized;
        // sqrt(price) for the next tick (1/0)
        uint160 sqrtPriceNextX96;
        // how much is being swapped in in this step
        uint256 amountIn;
        // how much is being swapped out
        uint256 amountOut;
        // how much fee is being paid in
        uint256 feeAmount;
    }

    /// @inheritdoc ICLPoolActions
    function swap(
        address recipient,
        bool zeroForOne,
        int256 amountSpecified,
        uint160 sqrtPriceLimitX96,
        bytes calldata data
    ) external override returns (int256 amount0, int256 amount1) {
        require(amountSpecified != 0, "AS");

        Slot0 memory slot0Start = slot0;

        require(slot0Start.unlocked, "LK");
        require(
            zeroForOne
                ? sqrtPriceLimitX96 < slot0Start.sqrtPriceX96 && sqrtPriceLimitX96 > TickMath.MIN_SQRT_RATIO
                : sqrtPriceLimitX96 > slot0Start.sqrtPriceX96 && sqrtPriceLimitX96 < TickMath.MAX_SQRT_RATIO,
            "SPL"
        );

        slot0.unlocked = false;

        SwapCache memory cache = SwapCache({
            liquidityStart: liquidity,
            stakedLiquidityStart: stakedLiquidity,
            blockTimestamp: _blockTimestamp(),
            secondsPerLiquidityCumulativeX128: 0,
            tickCumulative: 0,
            computedLatestObservation: false
        });

        bool exactInput = amountSpecified > 0;

        SwapState memory state = SwapState({
            amountSpecifiedRemaining: amountSpecified,
            amountCalculated: 0,
            sqrtPriceX96: slot0Start.sqrtPriceX96,
            tick: slot0Start.tick,
            fee: fee(),
            hasUpdatedFees: false,
            feeGrowthGlobalX128: zeroForOne ? feeGrowthGlobal0X128 : feeGrowthGlobal1X128,
            gaugeFee: 0,
            protocolFee: 0,
            liquidity: cache.liquidityStart,
            stakedLiquidity: cache.stakedLiquidityStart
        });

        // continue swapping as long as we haven't used the entire input/output and haven't reached the price limit
        while (state.amountSpecifiedRemaining != 0 && state.sqrtPriceX96 != sqrtPriceLimitX96) {
            StepComputations memory step;

            step.sqrtPriceStartX96 = state.sqrtPriceX96;

            (step.tickNext, step.initialized) =
                tickBitmap.nextInitializedTickWithinOneWord(state.tick, tickSpacing, zeroForOne);

            // ensure that we do not overshoot the min/max tick, as the tick bitmap is not aware of these bounds
            if (step.tickNext < TickMath.MIN_TICK) {
                step.tickNext = TickMath.MIN_TICK;
            } else if (step.tickNext > TickMath.MAX_TICK) {
                step.tickNext = TickMath.MAX_TICK;
            }

            // get the price for the next tick
            step.sqrtPriceNextX96 = TickMath.getSqrtRatioAtTick(step.tickNext);

            // compute values to swap to the target tick, price limit, or point where input/output amount is exhausted
            (state.sqrtPriceX96, step.amountIn, step.amountOut, step.feeAmount) = SwapMath.computeSwapStep(
                state.sqrtPriceX96,
                (zeroForOne ? step.sqrtPriceNextX96 < sqrtPriceLimitX96 : step.sqrtPriceNextX96 > sqrtPriceLimitX96)
                    ? sqrtPriceLimitX96
                    : step.sqrtPriceNextX96,
                state.liquidity,
                state.amountSpecifiedRemaining,
                state.fee
            );

            if (exactInput) {
                state.amountSpecifiedRemaining -= (step.amountIn + step.feeAmount).toInt256();
                state.amountCalculated = state.amountCalculated.sub(step.amountOut.toInt256());
            } else {
                state.amountSpecifiedRemaining += step.amountOut.toInt256();
                state.amountCalculated = state.amountCalculated.add((step.amountIn + step.feeAmount).toInt256());
            }

            // update global fee tracker and gauge fee
            if (state.liquidity > 0) {
                (uint256 _feeGrowthGlobalX128, uint256 _stakedFeeAmount, uint256 _protocolFeeAmount) =
                    calculateFees(step.feeAmount, state.liquidity, state.stakedLiquidity);

                state.feeGrowthGlobalX128 += _feeGrowthGlobalX128;
                state.gaugeFee += uint128(_stakedFeeAmount);
                state.protocolFee += uint128(_protocolFeeAmount);
            }

            // shift tick if we reached the next price
            if (state.sqrtPriceX96 == step.sqrtPriceNextX96) {
                // if the tick is initialized, run the tick transition
                if (step.initialized) {
                    // check for the placeholder value, which we replace with the actual value the first time the swap
                    // crosses an initialized tick
                    if (!cache.computedLatestObservation) {
                        (cache.tickCumulative, cache.secondsPerLiquidityCumulativeX128) = observations.observeSingle(
                            cache.blockTimestamp,
                            0,
                            slot0Start.tick,
                            slot0Start.observationIndex,
                            cache.liquidityStart,
                            slot0Start.observationCardinality
                        );
                        cache.computedLatestObservation = true;
                    }
                    if (!state.hasUpdatedFees) {
                        _updateRewardsGrowthGlobal();
                        state.hasUpdatedFees = true;
                    }
                    Tick.LiquidityNets memory nets = ticks.cross(
                        step.tickNext,
                        (zeroForOne ? state.feeGrowthGlobalX128 : feeGrowthGlobal0X128),
                        (zeroForOne ? feeGrowthGlobal1X128 : state.feeGrowthGlobalX128),
                        cache.secondsPerLiquidityCumulativeX128,
                        cache.tickCumulative,
                        cache.blockTimestamp,
                        rewardGrowthGlobalX128
                    );
                    // if we're moving leftward, we interpret liquidityNet & stakedLiquidityNet as the opposite sign
                    // safe because liquidityNet & stakedLiquidityNet cannot be type(int128).min
                    if (zeroForOne) {
                        nets.liquidityNet = -nets.liquidityNet;
                        nets.stakedLiquidityNet = -nets.stakedLiquidityNet;
                    }

                    state.liquidity = LiquidityMath.addDelta(state.liquidity, nets.liquidityNet);
                    state.stakedLiquidity = LiquidityMath.addDelta(state.stakedLiquidity, nets.stakedLiquidityNet);
                }

                state.tick = zeroForOne ? step.tickNext - 1 : step.tickNext;
            } else if (state.sqrtPriceX96 != step.sqrtPriceStartX96) {
                // recompute unless we're on a lower tick boundary (i.e. already transitioned ticks), and haven't moved
                state.tick = TickMath.getTickAtSqrtRatio(state.sqrtPriceX96);
            }
        }

        // update tick and write an oracle entry if the tick change
        if (state.tick != slot0Start.tick) {
            (uint16 observationIndex, uint16 observationCardinality) = observations.write(
                slot0Start.observationIndex,
                cache.blockTimestamp,
                slot0Start.tick,
                cache.liquidityStart,
                slot0Start.observationCardinality,
                slot0Start.observationCardinalityNext
            );
            (slot0.sqrtPriceX96, slot0.tick, slot0.observationIndex, slot0.observationCardinality) =
                (state.sqrtPriceX96, state.tick, observationIndex, observationCardinality);
        } else {
            // otherwise just update the price
            slot0.sqrtPriceX96 = state.sqrtPriceX96;
        }

        // update liquidity and stakedLiquidity if it changed
        if (cache.liquidityStart != state.liquidity) liquidity = state.liquidity;
        if (cache.stakedLiquidityStart != state.stakedLiquidity) stakedLiquidity = state.stakedLiquidity;

        // update fee growth global and, if necessary, gauge fees
        // overflow is acceptable, protocol has to withdraw before it hits type(uint128).max fees
        if (zeroForOne) {
            feeGrowthGlobal0X128 = state.feeGrowthGlobalX128;
            if (state.gaugeFee > 0) gaugeFees.token0 += state.gaugeFee;
            if (state.protocolFee > 0) protocolFees.token0 += state.protocolFee;
        } else {
            feeGrowthGlobal1X128 = state.feeGrowthGlobalX128;
            if (state.gaugeFee > 0) gaugeFees.token1 += state.gaugeFee;
            if (state.protocolFee > 0) protocolFees.token1 += state.protocolFee;
        }

        (amount0, amount1) = zeroForOne == exactInput
            ? (amountSpecified - state.amountSpecifiedRemaining, state.amountCalculated)
            : (state.amountCalculated, amountSpecified - state.amountSpecifiedRemaining);

        // do the transfers and collect payment
        if (zeroForOne) {
            if (amount1 < 0) TransferHelper.safeTransfer(token1, recipient, uint256(-amount1));

            uint256 balance0Before = balance0();
            ICLSwapCallback(msg.sender).uniswapV3SwapCallback(amount0, amount1, data);
            require(balance0Before.add(uint256(amount0)) <= balance0(), "IIA");
        } else {
            if (amount0 < 0) TransferHelper.safeTransfer(token0, recipient, uint256(-amount0));

            uint256 balance1Before = balance1();
            ICLSwapCallback(msg.sender).uniswapV3SwapCallback(amount0, amount1, data);
            require(balance1Before.add(uint256(amount1)) <= balance1(), "IIA");
        }

        emit Swap(msg.sender, recipient, amount0, amount1, state.sqrtPriceX96, state.liquidity, state.tick);
        slot0.unlocked = true;
    }

    /// @inheritdoc ICLPoolActions
    function flash(address recipient, uint256 amount0, uint256 amount1, bytes calldata data) external override lock {
        uint128 _liquidity = liquidity;
        require(_liquidity > 0, "L");

        uint256 fee0 = FullMath.mulDivRoundingUp(amount0, fee(), 1e6);
        uint256 fee1 = FullMath.mulDivRoundingUp(amount1, fee(), 1e6);
        uint256 balance0Before = balance0();
        uint256 balance1Before = balance1();

        if (amount0 > 0) TransferHelper.safeTransfer(token0, recipient, amount0);
        if (amount1 > 0) TransferHelper.safeTransfer(token1, recipient, amount1);

        ICLFlashCallback(msg.sender).uniswapV3FlashCallback(fee0, fee1, data);

        uint256 balance0After = balance0();
        uint256 balance1After = balance1();

        require(balance0Before.add(fee0) <= balance0After, "F0");
        require(balance1Before.add(fee1) <= balance1After, "F1");

        // sub is safe because we know balanceAfter is gt balanceBefore by at least fee
        uint256 paid0 = balance0After - balance0Before;
        uint256 paid1 = balance1After - balance1Before;

        if (paid0 > 0) {
            (uint256 feeGrowthGlobalX128, uint256 stakedFeeAmount, uint256 protocolFeeAmount) = calculateFees(paid0, _liquidity, stakedLiquidity);

            if (feeGrowthGlobalX128 > 0) feeGrowthGlobal0X128 += feeGrowthGlobalX128;
            if (uint128(stakedFeeAmount) > 0) gaugeFees.token0 += uint128(stakedFeeAmount);
            if (uint128(protocolFeeAmount) > 0) protocolFees.token0 += uint128(protocolFeeAmount);
        }
        if (paid1 > 0) {
            (uint256 feeGrowthGlobalX128, uint256 stakedFeeAmount, uint256 protocolFeeAmount) = calculateFees(paid1, _liquidity, stakedLiquidity);

            if (feeGrowthGlobalX128 > 0) feeGrowthGlobal1X128 += feeGrowthGlobalX128;
            if (uint128(stakedFeeAmount) > 0) gaugeFees.token1 += uint128(stakedFeeAmount);
            if (uint128(protocolFeeAmount) > 0) protocolFees.token1 += uint128(protocolFeeAmount);
        }
        emit Flash(msg.sender, recipient, amount0, amount1, paid0, paid1);
    }

    /// @inheritdoc ICLPoolState
    function getRewardGrowthInside(int24 tickLower, int24 tickUpper, uint256 _rewardGrowthGlobalX128)
        external
        view
        override
        returns (uint256 rewardGrowthInside)
    {
        checkTicks(tickLower, tickUpper);
        if (_rewardGrowthGlobalX128 == 0) _rewardGrowthGlobalX128 = rewardGrowthGlobalX128;

        return ticks.getRewardGrowthInside(tickLower, tickUpper, slot0.tick, _rewardGrowthGlobalX128);
    }

    /// @inheritdoc ICLPoolActions
    function updateRewardsGrowthGlobal() external override lock onlyGauge {
        _updateRewardsGrowthGlobal();
    }

    /// @dev timeDelta != 0 handles case when function is called twice in the same block.
    /// @dev stakedLiquidity > 0 handles case when depositing staked liquidity and there is no liquidity staked yet,
    /// @dev or when notifying rewards when there is no liquidity stake
    function _updateRewardsGrowthGlobal() internal {
        uint32 timestamp = _blockTimestamp();
        uint256 _lastUpdated = lastUpdated;
        uint256 timeDelta = timestamp - _lastUpdated; // skip if second call in same block

        if (timeDelta != 0) {
            if (rewardReserve > 0) {
                uint256 reward = rewardRate * timeDelta;
                if (reward > rewardReserve) reward = rewardReserve;
                rewardReserve -= reward;
                if (stakedLiquidity > 0) {
                    rewardGrowthGlobalX128 += FullMath.mulDiv(reward, FixedPoint128.Q128, stakedLiquidity);
                } else {
                    rollover += reward;
                }
            }
            lastUpdated = timestamp;
        }
    }

    /// @inheritdoc ICLPoolActions
    function syncReward(uint256 _rewardRate, uint256 _rewardReserve, uint256 _periodFinish)
        external
        override
        lock
        onlyGauge
    {
        rewardRate = _rewardRate;
        rewardReserve = _rewardReserve;
        periodFinish = _periodFinish;
        delete rollover;
    }

    /// @notice Calculates the fees owed to staked liquidity, then calculates fee levied on unstaked liquidity
    /// @param feeAmount Total fees
    /// @param _liquidity Current liquidity in active tick
    /// @param _stakedLiquidity Current staked liquidity in active tick
    /// @return unstakedFeeAmount Fee amount for unstaked LPs after accounting for staked liquidity contribution and unstaked fee
    /// @return stakedFeeAmount Fee amount for staked LPs consisting of staked liquidity contribution and unstaked fee
    function splitFees(uint256 feeAmount, uint128 _liquidity, uint128 _stakedLiquidity)
        internal
        view
        returns (uint256 unstakedFeeAmount, uint256 stakedFeeAmount, uint256 protocolFeeAmount)
    {
        stakedFeeAmount = FullMath.mulDivRoundingUp(feeAmount, _stakedLiquidity, _liquidity);
        (unstakedFeeAmount, stakedFeeAmount, protocolFeeAmount) = applyUnstakedFees(feeAmount - stakedFeeAmount, stakedFeeAmount);
    }

    /// @notice Calculates fee for levied on unstaked liquidity only
    /// @param _unstakedFeeAmount Fee amount for unstaked LPs net of staked liquidity contribution
    /// @param _stakedFeeAmount Fee amount for staked LPs consisting of staked liquidity contribution
    /// @return unstakedFeeAmount Fee amount for unstaked LPs after accounting for staked liquidity contribution and unstaked fee
    /// @return stakedFeeAmount Fee amount for staked LPs consisting of staked liquidity contribution and unstaked fee
    function applyUnstakedFees(uint256 _unstakedFeeAmount, uint256 _stakedFeeAmount)
        internal
        view
        returns (uint256 unstakedFeeAmount, uint256 stakedFeeAmount, uint256 protocolFeeAmount)
    {
        uint24 _protocolFeeConfig = protocolFee();
        if (_protocolFeeConfig > 0) {
            uint256 _protocolFee = FullMath.mulDivRoundingUp(_unstakedFeeAmount, _protocolFeeConfig, 1_000_000);
            unstakedFeeAmount = _unstakedFeeAmount - _protocolFee;
            stakedFeeAmount = _stakedFeeAmount;
            protocolFeeAmount = _protocolFee;
        } else {
            uint256 _stakedFee = FullMath.mulDivRoundingUp(_unstakedFeeAmount, unstakedFee(), 1_000_000);
            unstakedFeeAmount = _unstakedFeeAmount - _stakedFee;
            stakedFeeAmount = _stakedFeeAmount + _stakedFee;
        }
    }

    // calculates the fee growths for unstaked liquidity and returns it with the staked fee amount
    function calculateFees(uint256 feeAmount, uint128 _liquidity, uint128 _stakedLiquidity)
        internal
        view
        returns (uint256 feeGrowthGlobalX128, uint256 stakedFeeAmount, uint256 protocolFeeAmount)
    {
        // if there is only staked liquidity
        if (_liquidity == _stakedLiquidity) {
            stakedFeeAmount = feeAmount;
        }
        // if there is only unstaked liquidity
        else if (_stakedLiquidity == 0) {
            (uint256 unstakedFeeAmount, uint256 _stakedFeeAmount, uint256 _protocolFeeAmount) = applyUnstakedFees(feeAmount, 0 );
            feeGrowthGlobalX128 = FullMath.mulDiv(unstakedFeeAmount, FixedPoint128.Q128, _liquidity);
            stakedFeeAmount = _stakedFeeAmount;
            protocolFeeAmount = _protocolFeeAmount;
        }
        // if there are staked and unstaked liquidities
        else {
            (uint256 unstakedFeeAmount, uint256 _stakedFeeAmount, uint256 _protocolFeeAmount) = splitFees(feeAmount, _liquidity, _stakedLiquidity);
            feeGrowthGlobalX128 = FullMath.mulDiv(unstakedFeeAmount, FixedPoint128.Q128, _liquidity - _stakedLiquidity);
            stakedFeeAmount = _stakedFeeAmount;
            protocolFeeAmount = _protocolFeeAmount;
        }
    }

    /// @inheritdoc ICLPoolOwnerActions
    function collectFees() external override lock onlyGauge returns (uint128 amount0, uint128 amount1) {
        amount0 = gaugeFees.token0;
        amount1 = gaugeFees.token1;
        if (amount0 > 1) {
            // ensure that the slot is not cleared, for gas savings
            gaugeFees.token0 = 1;
            TransferHelper.safeTransfer(token0, msg.sender, --amount0);
        }
        if (amount1 > 1) {
            // ensure that the slot is not cleared, for gas savings
            gaugeFees.token1 = 1;
            TransferHelper.safeTransfer(token1, msg.sender, --amount1);
        }

        emit CollectFees(msg.sender, amount0, amount1);
    }

    function collectProtocolFees(address recipient) external  lock  returns (uint128 amount0, uint128 amount1) {
        require(msg.sender == factory);
        amount0 = protocolFees.token0;
        amount1 = protocolFees.token1;
        if (amount0 > 0) {
            protocolFees.token0 = 0;
            TransferHelper.safeTransfer(token0, recipient, --amount0);
        }
        if (amount1 > 0) {
            protocolFees.token1 = 0;
            TransferHelper.safeTransfer(token1, recipient, --amount1);
        }
        // emit CollectProtocolFees(recipient, amount0, amount1);
    }

    /// @inheritdoc ICLPoolActions
    function setGaugeAndPositionManager(address _gauge, address _nft) external override lock {
        require( msg.sender == gaugeManager && gauge == address(0) );
        gauge = _gauge;
        nft = _nft;
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.5.0 <0.8.0;

/// @title Oracle
/// @notice Provides price and liquidity data useful for a wide variety of system designs
/// @dev Instances of stored oracle data, "observations", are collected in the oracle array
/// Every pool is initialized with an oracle array length of 1. Anyone can pay the SSTOREs to increase the
/// maximum length of the oracle array. New slots will be added when the array is fully populated.
/// Observations are overwritten when the full length of the oracle array is populated.
/// The most recent observation is available, independent of the length of the oracle array, by passing 0 to observe()
library Oracle {
    struct Observation {
        // the block timestamp of the observation
        uint32 blockTimestamp;
        // the tick accumulator, i.e. tick * time elapsed since the pool was first initialized
        int56 tickCumulative;
        // the seconds per liquidity, i.e. seconds elapsed / max(1, liquidity) since the pool was first initialized
        uint160 secondsPerLiquidityCumulativeX128;
        // whether or not the observation is initialized
        bool initialized;
    }

    /// @notice Transforms a previous observation into a new observation, given the passage of time and the current tick and liquidity values
    /// @dev blockTimestamp _must_ be chronologically equal to or greater than last.blockTimestamp, safe for 0 or 1 overflows
    /// @param last The specified observation to be transformed
    /// @param blockTimestamp The timestamp of the new observation
    /// @param tick The active tick at the time of the new observation
    /// @param liquidity The total in-range liquidity at the time of the new observation
    /// @return Observation The newly populated observation
    function transform(Observation memory last, uint32 blockTimestamp, int24 tick, uint128 liquidity)
        private
        pure
        returns (Observation memory)
    {
        uint32 delta = blockTimestamp - last.blockTimestamp;
        return Observation({
            blockTimestamp: blockTimestamp,
            tickCumulative: last.tickCumulative + int56(tick) * delta,
            secondsPerLiquidityCumulativeX128: last.secondsPerLiquidityCumulativeX128
                + ((uint160(delta) << 128) / (liquidity > 0 ? liquidity : 1)),
            initialized: true
        });
    }

    /// @notice Initialize the oracle array by writing the first slot. Called once for the lifecycle of the observations array
    /// @param self The stored oracle array
    /// @param time The time of the oracle initialization, via block.timestamp truncated to uint32
    /// @return cardinality The number of populated elements in the oracle array
    /// @return cardinalityNext The new length of the oracle array, independent of population
    function initialize(Observation[65535] storage self, uint32 time)
        internal
        returns (uint16 cardinality, uint16 cardinalityNext)
    {
        self[0] = Observation({
            blockTimestamp: time,
            tickCumulative: 0,
            secondsPerLiquidityCumulativeX128: 0,
            initialized: true
        });
        return (1, 1);
    }

    /// @notice Writes an oracle observation to the array
    /// @dev Writable at most once per block. Index represents the most recently written element. cardinality and index must be tracked externally.
    /// If the index is at the end of the allowable array length (according to cardinality), and the next cardinality
    /// is greater than the current one, cardinality may be increased. This restriction is created to preserve ordering.
    /// @param self The stored oracle array
    /// @param index The index of the observation that was most recently written to the observations array
    /// @param blockTimestamp The timestamp of the new observation
    /// @param tick The active tick at the time of the new observation
    /// @param liquidity The total in-range liquidity at the time of the new observation
    /// @param cardinality The number of populated elements in the oracle array
    /// @param cardinalityNext The new length of the oracle array, independent of population
    /// @return indexUpdated The new index of the most recently written element in the oracle array
    /// @return cardinalityUpdated The new cardinality of the oracle array
    function write(
        Observation[65535] storage self,
        uint16 index,
        uint32 blockTimestamp,
        int24 tick,
        uint128 liquidity,
        uint16 cardinality,
        uint16 cardinalityNext
    ) internal returns (uint16 indexUpdated, uint16 cardinalityUpdated) {
        Observation memory last = self[index];

        // early return if we've already written an observation this block
        if (last.blockTimestamp == blockTimestamp) return (index, cardinality);

        // if the conditions are right, we can bump the cardinality
        if (cardinalityNext > cardinality && index == (cardinality - 1)) {
            cardinalityUpdated = cardinalityNext;
        } else {
            cardinalityUpdated = cardinality;
        }

        indexUpdated = (index + 1) % cardinalityUpdated;
        self[indexUpdated] = transform(last, blockTimestamp, tick, liquidity);
    }

    /// @notice Prepares the oracle array to store up to `next` observations
    /// @param self The stored oracle array
    /// @param current The current next cardinality of the oracle array
    /// @param next The proposed next cardinality which will be populated in the oracle array
    /// @return next The next cardinality which will be populated in the oracle array
    function grow(Observation[65535] storage self, uint16 current, uint16 next) internal returns (uint16) {
        require(current > 0, "I");
        // no-op if the passed next value isn't greater than the current next value
        if (next <= current) return current;
        // store in each slot to prevent fresh SSTOREs in swaps
        // this data will not be used because the initialized boolean is still false
        for (uint16 i = current; i < next; i++) {
            self[i].blockTimestamp = 1;
        }
        return next;
    }

    /// @notice comparator for 32-bit timestamps
    /// @dev safe for 0 or 1 overflows, a and b _must_ be chronologically before or equal to time
    /// @param time A timestamp truncated to 32 bits
    /// @param a A comparison timestamp from which to determine the relative position of `time`
    /// @param b From which to determine the relative position of `time`
    /// @return bool Whether `a` is chronologically <= `b`
    function lte(uint32 time, uint32 a, uint32 b) private pure returns (bool) {
        // if there hasn't been overflow, no need to adjust
        if (a <= time && b <= time) return a <= b;

        uint256 aAdjusted = a > time ? a : a + 2 ** 32;
        uint256 bAdjusted = b > time ? b : b + 2 ** 32;

        return aAdjusted <= bAdjusted;
    }

    /// @notice Fetches the observations beforeOrAt and atOrAfter a target, i.e. where [beforeOrAt, atOrAfter] is satisfied.
    /// The result may be the same observation, or adjacent observations.
    /// @dev The answer must be contained in the array, used when the target is located within the stored observation
    /// boundaries: older than the most recent observation and younger, or the same age as, the oldest observation
    /// @param self The stored oracle array
    /// @param time The current block.timestamp
    /// @param target The timestamp at which the reserved observation should be for
    /// @param index The index of the observation that was most recently written to the observations array
    /// @param cardinality The number of populated elements in the oracle array
    /// @return beforeOrAt The observation recorded before, or at, the target
    /// @return atOrAfter The observation recorded at, or after, the target
    function binarySearch(Observation[65535] storage self, uint32 time, uint32 target, uint16 index, uint16 cardinality)
        private
        view
        returns (Observation memory beforeOrAt, Observation memory atOrAfter)
    {
        uint256 l = (index + 1) % cardinality; // oldest observation
        uint256 r = l + cardinality - 1; // newest observation
        uint256 i;
        while (true) {
            i = (l + r) / 2;

            beforeOrAt = self[i % cardinality];

            // we've landed on an uninitialized tick, keep searching higher (more recently)
            if (!beforeOrAt.initialized) {
                l = i + 1;
                continue;
            }

            atOrAfter = self[(i + 1) % cardinality];

            bool targetAtOrAfter = lte(time, beforeOrAt.blockTimestamp, target);

            // check if we've found the answer!
            if (targetAtOrAfter && lte(time, target, atOrAfter.blockTimestamp)) break;

            if (!targetAtOrAfter) r = i - 1;
            else l = i + 1;
        }
    }

    /// @notice Fetches the observations beforeOrAt and atOrAfter a given target, i.e. where [beforeOrAt, atOrAfter] is satisfied
    /// @dev Assumes there is at least 1 initialized observation.
    /// Used by observeSingle() to compute the counterfactual accumulator values as of a given block timestamp.
    /// @param self The stored oracle array
    /// @param time The current block.timestamp
    /// @param target The timestamp at which the reserved observation should be for
    /// @param tick The active tick at the time of the returned or simulated observation
    /// @param index The index of the observation that was most recently written to the observations array
    /// @param liquidity The total pool liquidity at the time of the call
    /// @param cardinality The number of populated elements in the oracle array
    /// @return beforeOrAt The observation which occurred at, or before, the given timestamp
    /// @return atOrAfter The observation which occurred at, or after, the given timestamp
    function getSurroundingObservations(
        Observation[65535] storage self,
        uint32 time,
        uint32 target,
        int24 tick,
        uint16 index,
        uint128 liquidity,
        uint16 cardinality
    ) private view returns (Observation memory beforeOrAt, Observation memory atOrAfter) {
        // optimistically set before to the newest observation
        beforeOrAt = self[index];

        // if the target is chronologically at or after the newest observation, we can early return
        if (lte(time, beforeOrAt.blockTimestamp, target)) {
            if (beforeOrAt.blockTimestamp == target) {
                // if newest observation equals target, we're in the same block, so we can ignore atOrAfter
                return (beforeOrAt, atOrAfter);
            } else {
                // otherwise, we need to transform
                return (beforeOrAt, transform(beforeOrAt, target, tick, liquidity));
            }
        }

        // now, set before to the oldest observation
        beforeOrAt = self[(index + 1) % cardinality];
        if (!beforeOrAt.initialized) beforeOrAt = self[0];

        // ensure that the target is chronologically at or after the oldest observation
        require(lte(time, beforeOrAt.blockTimestamp, target), "OLD");

        // if we've reached this point, we have to binary search
        return binarySearch(self, time, target, index, cardinality);
    }

    /// @dev Reverts if an observation at or before the desired observation timestamp does not exist.
    /// 0 may be passed as `secondsAgo' to return the current cumulative values.
    /// If called with a timestamp falling between two observations, returns the counterfactual accumulator values
    /// at exactly the timestamp between the two observations.
    /// @param self The stored oracle array
    /// @param time The current block timestamp
    /// @param secondsAgo The amount of time to look back, in seconds, at which point to return an observation
    /// @param tick The current tick
    /// @param index The index of the observation that was most recently written to the observations array
    /// @param liquidity The current in-range pool liquidity
    /// @param cardinality The number of populated elements in the oracle array
    /// @return tickCumulative The tick * time elapsed since the pool was first initialized, as of `secondsAgo`
    /// @return secondsPerLiquidityCumulativeX128 The time elapsed / max(1, liquidity) since the pool was first initialized, as of `secondsAgo`
    function observeSingle(
        Observation[65535] storage self,
        uint32 time,
        uint32 secondsAgo,
        int24 tick,
        uint16 index,
        uint128 liquidity,
        uint16 cardinality
    ) internal view returns (int56 tickCumulative, uint160 secondsPerLiquidityCumulativeX128) {
        if (secondsAgo == 0) {
            Observation memory last = self[index];
            if (last.blockTimestamp != time) last = transform(last, time, tick, liquidity);
            return (last.tickCumulative, last.secondsPerLiquidityCumulativeX128);
        }

        uint32 target = time - secondsAgo;

        (Observation memory beforeOrAt, Observation memory atOrAfter) =
            getSurroundingObservations(self, time, target, tick, index, liquidity, cardinality);

        if (target == beforeOrAt.blockTimestamp) {
            // we're at the left boundary
            return (beforeOrAt.tickCumulative, beforeOrAt.secondsPerLiquidityCumulativeX128);
        } else if (target == atOrAfter.blockTimestamp) {
            // we're at the right boundary
            return (atOrAfter.tickCumulative, atOrAfter.secondsPerLiquidityCumulativeX128);
        } else {
            // we're in the middle
            uint32 observationTimeDelta = atOrAfter.blockTimestamp - beforeOrAt.blockTimestamp;
            uint32 targetDelta = target - beforeOrAt.blockTimestamp;
            return (
                beforeOrAt.tickCumulative
                    + ((atOrAfter.tickCumulative - beforeOrAt.tickCumulative) / observationTimeDelta) * targetDelta,
                beforeOrAt.secondsPerLiquidityCumulativeX128
                    + uint160(
                        (
                            uint256(
                                atOrAfter.secondsPerLiquidityCumulativeX128 - beforeOrAt.secondsPerLiquidityCumulativeX128
                            ) * targetDelta
                        ) / observationTimeDelta
                    )
            );
        }
    }

    /// @notice Returns the accumulator values as of each time seconds ago from the given time in the array of `secondsAgos`
    /// @dev Reverts if `secondsAgos` > oldest observation
    /// @param self The stored oracle array
    /// @param time The current block.timestamp
    /// @param secondsAgos Each amount of time to look back, in seconds, at which point to return an observation
    /// @param tick The current tick
    /// @param index The index of the observation that was most recently written to the observations array
    /// @param liquidity The current in-range pool liquidity
    /// @param cardinality The number of populated elements in the oracle array
    /// @return tickCumulatives The tick * time elapsed since the pool was first initialized, as of each `secondsAgo`
    /// @return secondsPerLiquidityCumulativeX128s The cumulative seconds / max(1, liquidity) since the pool was first initialized, as of each `secondsAgo`
    function observe(
        Observation[65535] storage self,
        uint32 time,
        uint32[] memory secondsAgos,
        int24 tick,
        uint16 index,
        uint128 liquidity,
        uint16 cardinality
    ) internal view returns (int56[] memory tickCumulatives, uint160[] memory secondsPerLiquidityCumulativeX128s) {
        require(cardinality > 0, "I");

        tickCumulatives = new int56[](secondsAgos.length);
        secondsPerLiquidityCumulativeX128s = new uint160[](secondsAgos.length);
        for (uint256 i = 0; i < secondsAgos.length; i++) {
            (tickCumulatives[i], secondsPerLiquidityCumulativeX128s[i]) =
                observeSingle(self, time, secondsAgos[i], tick, index, liquidity, cardinality);
        }
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.5.0;

import {IVoter} from "contracts/core/interfaces/IVoter.sol";
import {IFactoryRegistry} from "contracts/core/interfaces/IFactoryRegistry.sol";
import {IGaugeManager} from "contracts/core/interfaces/IGaugeManager.sol";


/// @title The interface for the CL Factory
/// @notice The CL Factory facilitates creation of CL pools and control over the protocol fees
interface ICLFactory {
    /// @notice Emitted when the owner of the factory is changed
    /// @param oldOwner The owner before the owner was changed
    /// @param newOwner The owner after the owner was changed
    event OwnerChanged(address indexed oldOwner, address indexed newOwner);

    /// @notice Emitted when the swapFeeManager of the factory is changed
    /// @param oldFeeManager The swapFeeManager before the swapFeeManager was changed
    /// @param newFeeManager The swapFeeManager after the swapFeeManager was changed
    event SwapFeeManagerChanged(address indexed oldFeeManager, address indexed newFeeManager);

    /// @notice Emitted when the swapFeeModule of the factory is changed
    /// @param oldFeeModule The swapFeeModule before the swapFeeModule was changed
    /// @param newFeeModule The swapFeeModule after the swapFeeModule was changed
    event SwapFeeModuleChanged(address indexed oldFeeModule, address indexed newFeeModule);

    /// @notice Emitted when the unstakedFeeManager of the factory is changed
    /// @param oldFeeManager The unstakedFeeManager before the unstakedFeeManager was changed
    /// @param newFeeManager The unstakedFeeManager after the unstakedFeeManager was changed
    event UnstakedFeeManagerChanged(address indexed oldFeeManager, address indexed newFeeManager);

    /// @notice Emitted when the unstakedFeeModule of the factory is changed
    /// @param oldFeeModule The unstakedFeeModule before the unstakedFeeModule was changed
    /// @param newFeeModule The unstakedFeeModule after the unstakedFeeModule was changed
    event UnstakedFeeModuleChanged(address indexed oldFeeModule, address indexed newFeeModule);

    /// @notice Emitted when the defaultUnstakedFee of the factory is changed
    /// @param oldUnstakedFee The defaultUnstakedFee before the defaultUnstakedFee was changed
    /// @param newUnstakedFee The defaultUnstakedFee after the unstakedFeeModule was changed
    event DefaultUnstakedFeeChanged(uint24 indexed oldUnstakedFee, uint24 indexed newUnstakedFee);

    /// @notice Emitted when a pool is created
    /// @param token0 The first token of the pool by address sort order
    /// @param token1 The second token of the pool by address sort order
    /// @param tickSpacing The minimum number of ticks between initialized ticks
    /// @param pool The address of the created pool
    event PoolCreated(address indexed token0, address indexed token1, int24 indexed tickSpacing, address pool);

    /// @notice Emitted when a new tick spacing is enabled for pool creation via the factory
    /// @param tickSpacing The minimum number of ticks between initialized ticks for pools
    /// @param fee The default fee for a pool created with a given tickSpacing
    event TickSpacingEnabled(int24 indexed tickSpacing, uint24 indexed fee);



    /// @notice The address of the pool implementation contract used to deploy proxies / clones
    /// @return The address of the pool implementation contract
    function poolImplementation() external view returns (address);

    /// @notice Factory registry for valid pool / gauge / rewards factories
    /// @return The address of the factory registry

    function gaugeManager() external view returns (IGaugeManager);

    /// @notice Returns the current owner of the factory
    /// @dev Can be changed by the current owner via setOwner
    /// @return The address of the factory owner
    function owner() external view returns (address);

    /// @notice Returns the current swapFeeManager of the factory
    /// @dev Can be changed by the current swap fee manager via setSwapFeeManager
    /// @return The address of the factory swapFeeManager
    function swapFeeManager() external view returns (address);

    /// @notice Returns the current protocolFeeManager of the factory
    /// @dev Can be changed by the current protocol fee manager via setProtocolFeeManager
    /// @return The address of the factory protocolFeeManager
    function protocolFeeManager() external view returns (address);

    /// @notice Returns the current swapFeeModule of the factory
    /// @dev Can be changed by the current swap fee manager via setSwapFeeModule
    /// @return The address of the factory swapFeeModule
    function swapFeeModule() external view returns (address);

    /// @notice Returns the current unstakedFeeManager of the factory
    /// @dev Can be changed by the current unstaked fee manager via setUnstakedFeeManager
    /// @return The address of the factory unstakedFeeManager
    function unstakedFeeManager() external view returns (address);

    /// @notice Returns the current unstakedFeeModule of the factory
    /// @dev Can be changed by the current unstaked fee manager via setUnstakedFeeModule
    /// @return The address of the factory unstakedFeeModule
    function unstakedFeeModule() external view returns (address);


    function protocolFeeModule() external view returns (address);

    /// @notice Returns the current defaultUnstakedFee of the factory
    /// @dev Can be changed by the current unstaked fee manager via setDefaultUnstakedFee
    /// @return The default Unstaked Fee of the factory
    function defaultUnstakedFee() external view returns (uint24);


    function defaultProtocolFee() external view returns (uint24);

    /// @notice Returns a default fee for a tick spacing.
    /// @dev Use getFee for the most up to date fee for a given pool.
    /// A tick spacing can never be removed, so this value should be hard coded or cached in the calling context
    /// @param tickSpacing The enabled tick spacing. Returns 0 if not enabled
    /// @return fee The default fee for the given tick spacing
    function tickSpacingToFee(int24 tickSpacing) external view returns (uint24 fee);

    /// @notice Returns a list of enabled tick spacings. Used to iterate through pools created by the factory
    /// @dev Tick spacings cannot be removed. Tick spacings are not ordered
    /// @return List of enabled tick spacings
    function tickSpacings() external view returns (int24[] memory);

    /// @notice Returns the pool address for a given pair of tokens and a tick spacing, or address 0 if it does not exist
    /// @dev tokenA and tokenB may be passed in either token0/token1 or token1/token0 order
    /// @param tokenA The contract address of either token0 or token1
    /// @param tokenB The contract address of the other token
    /// @param tickSpacing The tick spacing of the pool
    /// @return pool The pool address
    function getPool(address tokenA, address tokenB, int24 tickSpacing) external view returns (address pool);

    /// @notice Return address of pool created by this factory given its `index`
    /// @param index Index of the pool
    /// @return The pool address in the given index
    function allPools(uint256 index) external view returns (address);

    /// @notice Returns the number of pools created from this factory
    /// @return Number of pools created from this factory
    function allPoolsLength() external view returns (uint256);

    /// @notice Used in VotingEscrow to determine if a contract is a valid pool of the factory
    /// @param pool The address of the pool to check
    /// @return Whether the pool is a valid pool of the factory
    function isPool(address pool) external view returns (bool);

    /// @notice Get swap & flash fee for a given pool. Accounts for default and dynamic fees
    /// @dev Swap & flash fee is denominated in pips. i.e. 1e-6
    /// @param pool The pool to get the swap & flash fee for
    /// @return The swap & flash fee for the given pool
    function getSwapFee(address pool) external view returns (uint24);

    /// @notice Get unstaked fee for a given pool. Accounts for default and dynamic fees
    /// @dev Unstaked fee is denominated in pips. i.e. 1e-6
    /// @param pool The pool to get the unstaked fee for
    /// @return The unstaked fee for the given pool
    function getUnstakedFee(address pool) external view returns (uint24);

    /// @notice Get protocol fee for a given pool. Accounts for default and dynamic fees
    /// @dev Protocol fee is denominated in pips. i.e. 1e-6
    /// @param pool The pool to get the protocol fee for
    /// @return The protocol fee for the given pool
    function getProtocolFee(address pool) external view returns (uint24);

    /// @notice Creates a pool for the given two tokens and fee
    /// @param tokenA One of the two tokens in the desired pool
    /// @param tokenB The other of the two tokens in the desired pool
    /// @param tickSpacing The desired tick spacing for the pool
    /// @param sqrtPriceX96 The initial sqrt price of the pool, as a Q64.96
    /// @dev tokenA and tokenB may be passed in either order: token0/token1 or token1/token0. The call will
    /// revert if the pool already exists, the tick spacing is invalid, or the token arguments are invalid
    /// @return pool The address of the newly created pool
    function createPool(address tokenA, address tokenB, int24 tickSpacing, uint160 sqrtPriceX96)
        external
        returns (address pool);

    /// @notice Updates the owner of the factory
    /// @dev Must be called by the current owner
    /// @param _owner The new owner of the factory
    function setOwner(address _owner) external;

    /// @notice Updates the swapFeeManager of the factory
    /// @dev Must be called by the current swap fee manager
    /// @param _swapFeeManager The new swapFeeManager of the factory
    function setSwapFeeManager(address _swapFeeManager) external;

    /// @notice Updates the swapFeeModule of the factory
    /// @dev Must be called by the current swap fee manager
    /// @param _swapFeeModule The new swapFeeModule of the factory
    function setSwapFeeModule(address _swapFeeModule) external;

    /// @notice Updates the unstakedFeeManager of the factory
    /// @dev Must be called by the current unstaked fee manager
    /// @param _unstakedFeeManager The new unstakedFeeManager of the factory
    function setUnstakedFeeManager(address _unstakedFeeManager) external;

    /// @notice Updates the unstakedFeeModule of the factory
    /// @dev Must be called by the current unstaked fee manager
    /// @param _unstakedFeeModule The new unstakedFeeModule of the factory
    function setUnstakedFeeModule(address _unstakedFeeModule) external;

    /// @notice Updates the protocolFeeManager of the factory
    /// @dev Must be called by the current protocol fee manager
    /// @param _protocolFeeManager The new protocolFeeManager of the factory
    function setProtocolFeeManager(address _protocolFeeManager) external;

    /// @notice Updates the protocolFeeModule of the factory
    /// @dev Must be called by the current protocol fee manager
    /// @param _protocolFeeModule The new protocolFeeModule of the factory
    function setProtocolFeeModule(address _protocolFeeModule) external;

    /// @notice Updates the defaultUnstakedFee of the factory
    /// @dev Must be called by the current unstaked fee manager
    /// @param _defaultUnstakedFee The new defaultUnstakedFee of the factory
    function setDefaultUnstakedFee(uint24 _defaultUnstakedFee) external;

    /// @notice Enables a certain tickSpacing
    /// @dev Tick spacings may never be removed once enabled
    /// @param tickSpacing The spacing between ticks to be enforced in the pool
    /// @param fee The default fee associated with a given tick spacing
    function enableTickSpacing(int24 tickSpacing, uint24 fee) external;
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.5.0 <0.8.0;

import "./LowGasSafeMath.sol";
import "./SafeCast.sol";

import "./TickMath.sol";
import "./LiquidityMath.sol";

/// @title Tick
/// @notice Contains functions for managing tick processes and relevant calculations
library Tick {
    using LowGasSafeMath for int256;
    using SafeCast for int256;

    // info stored for each initialized individual tick
    struct Info {
        // the total position liquidity that references this tick
        // includes both staked and unstaked liquidity
        uint128 liquidityGross;
        // amount of net liquidity added (subtracted) when tick is crossed from left to right (right to left)
        // includes both staked and unstaked liquidity
        int128 liquidityNet;
        // amount of net staked liquidity added (subtracted) when tick is crossed from left to right (right to left)
        int128 stakedLiquidityNet;
        // fee growth per unit of liquidity on the _other_ side of this tick (relative to the current tick)
        // only has relative meaning, not absolute — the value depends on when the tick is initialized
        uint256 feeGrowthOutside0X128;
        uint256 feeGrowthOutside1X128;
        // reward growth per unit of liquidity on the _other_ side of this tick (relative to the current tick)
        // only has relative meaning, not absolute — the value depends on when the tick is initialized
        uint256 rewardGrowthOutsideX128;
        // the cumulative tick value on the other side of the tick
        int56 tickCumulativeOutside;
        // the seconds per unit of liquidity on the _other_ side of this tick (relative to the current tick)
        // only has relative meaning, not absolute — the value depends on when the tick is initialized
        uint160 secondsPerLiquidityOutsideX128;
        // the seconds spent on the other side of the tick (relative to the current tick)
        // only has relative meaning, not absolute — the value depends on when the tick is initialized
        uint32 secondsOutside;
        // true iff the tick is initialized, i.e. the value is exactly equivalent to the expression liquidityGross != 0
        // these 8 bits are set to prevent fresh sstores when crossing newly initialized ticks
        bool initialized;
    }

    struct LiquidityNets {
        int128 liquidityNet;
        int128 stakedLiquidityNet;
    }

    /// @notice Derives max liquidity per tick from given tick spacing
    /// @dev Executed within the pool constructor
    /// @param tickSpacing The amount of required tick separation, realized in multiples of `tickSpacing`
    ///     e.g., a tickSpacing of 3 requires ticks to be initialized every 3rd tick i.e., ..., -6, -3, 0, 3, 6, ...
    /// @return The max liquidity per tick
    function tickSpacingToMaxLiquidityPerTick(int24 tickSpacing) internal pure returns (uint128) {
        int24 minTick = (TickMath.MIN_TICK / tickSpacing) * tickSpacing;
        int24 maxTick = (TickMath.MAX_TICK / tickSpacing) * tickSpacing;
        uint24 numTicks = uint24((maxTick - minTick) / tickSpacing) + 1;
        return type(uint128).max / numTicks;
    }

    /// @notice Retrieves fee growth data
    /// @param self The mapping containing all tick information for initialized ticks
    /// @param tickLower The lower tick boundary of the position
    /// @param tickUpper The upper tick boundary of the position
    /// @param tickCurrent The current tick
    /// @param feeGrowthGlobal0X128 The all-time global fee growth, per unit of liquidity, in token0
    /// @param feeGrowthGlobal1X128 The all-time global fee growth, per unit of liquidity, in token1
    /// @return feeGrowthInside0X128 The all-time fee growth in token0, per unit of liquidity, inside the position's tick boundaries
    /// @return feeGrowthInside1X128 The all-time fee growth in token1, per unit of liquidity, inside the position's tick boundaries
    function getFeeGrowthInside(
        mapping(int24 => Tick.Info) storage self,
        int24 tickLower,
        int24 tickUpper,
        int24 tickCurrent,
        uint256 feeGrowthGlobal0X128,
        uint256 feeGrowthGlobal1X128
    ) internal view returns (uint256 feeGrowthInside0X128, uint256 feeGrowthInside1X128) {
        Info storage lower = self[tickLower];
        Info storage upper = self[tickUpper];

        // calculate fee growth below
        uint256 feeGrowthBelow0X128;
        uint256 feeGrowthBelow1X128;
        if (tickCurrent >= tickLower) {
            feeGrowthBelow0X128 = lower.feeGrowthOutside0X128;
            feeGrowthBelow1X128 = lower.feeGrowthOutside1X128;
        } else {
            feeGrowthBelow0X128 = feeGrowthGlobal0X128 - lower.feeGrowthOutside0X128;
            feeGrowthBelow1X128 = feeGrowthGlobal1X128 - lower.feeGrowthOutside1X128;
        }

        // calculate fee growth above
        uint256 feeGrowthAbove0X128;
        uint256 feeGrowthAbove1X128;
        if (tickCurrent < tickUpper) {
            feeGrowthAbove0X128 = upper.feeGrowthOutside0X128;
            feeGrowthAbove1X128 = upper.feeGrowthOutside1X128;
        } else {
            feeGrowthAbove0X128 = feeGrowthGlobal0X128 - upper.feeGrowthOutside0X128;
            feeGrowthAbove1X128 = feeGrowthGlobal1X128 - upper.feeGrowthOutside1X128;
        }

        feeGrowthInside0X128 = feeGrowthGlobal0X128 - feeGrowthBelow0X128 - feeGrowthAbove0X128;
        feeGrowthInside1X128 = feeGrowthGlobal1X128 - feeGrowthBelow1X128 - feeGrowthAbove1X128;
    }

    function getRewardGrowthInside(
        mapping(int24 => Tick.Info) storage self,
        int24 tickLower,
        int24 tickUpper,
        int24 tickCurrent,
        uint256 rewardGrowthGlobalX128
    ) internal view returns (uint256 rewardGrowthInsideX128) {
        Info storage lower = self[tickLower];
        Info storage upper = self[tickUpper];

        // calculate reward growth below
        uint256 rewardGrowthBelowX128;
        if (tickCurrent >= tickLower) {
            rewardGrowthBelowX128 = lower.rewardGrowthOutsideX128;
        } else {
            rewardGrowthBelowX128 = rewardGrowthGlobalX128 - lower.rewardGrowthOutsideX128;
        }

        // calculate reward growth above
        uint256 rewardGrowthAboveX128;
        if (tickCurrent < tickUpper) {
            rewardGrowthAboveX128 = upper.rewardGrowthOutsideX128;
        } else {
            rewardGrowthAboveX128 = rewardGrowthGlobalX128 - upper.rewardGrowthOutsideX128;
        }

        rewardGrowthInsideX128 = rewardGrowthGlobalX128 - rewardGrowthBelowX128 - rewardGrowthAboveX128;
    }

    /// @notice Updates a tick and returns true if the tick was flipped from initialized to uninitialized, or vice versa
    /// @param self The mapping containing all tick information for initialized ticks
    /// @param tick The tick that will be updated
    /// @param tickCurrent The current tick
    /// @param liquidityDelta A new amount of liquidity to be added (subtracted) when tick is crossed from left to right (right to left)
    /// @param feeGrowthGlobal0X128 The all-time global fee growth, per unit of liquidity, in token0
    /// @param feeGrowthGlobal1X128 The all-time global fee growth, per unit of liquidity, in token1
    /// @param secondsPerLiquidityCumulativeX128 The all-time seconds per max(1, liquidity) of the pool
    /// @param tickCumulative The tick * time elapsed since the pool was first initialized
    /// @param time The current block timestamp cast to a uint32
    /// @param upper true for updating a position's upper tick, or false for updating a position's lower tick
    /// @param maxLiquidity The maximum liquidity allocation for a single tick
    /// @return flipped Whether the tick was flipped from initialized to uninitialized, or vice versa
    function update(
        mapping(int24 => Tick.Info) storage self,
        int24 tick,
        int24 tickCurrent,
        int128 liquidityDelta,
        uint256 feeGrowthGlobal0X128,
        uint256 feeGrowthGlobal1X128,
        uint160 secondsPerLiquidityCumulativeX128,
        int56 tickCumulative,
        uint32 time,
        bool upper,
        uint128 maxLiquidity
    ) internal returns (bool flipped) {
        Tick.Info storage info = self[tick];

        uint128 liquidityGrossBefore = info.liquidityGross;
        uint128 liquidityGrossAfter = LiquidityMath.addDelta(liquidityGrossBefore, liquidityDelta);

        require(liquidityGrossAfter <= maxLiquidity, "LO");

        flipped = (liquidityGrossAfter == 0) != (liquidityGrossBefore == 0);

        if (liquidityGrossBefore == 0) {
            // by convention, we assume that all growth before a tick was initialized happened _below_ the tick
            if (tick <= tickCurrent) {
                info.feeGrowthOutside0X128 = feeGrowthGlobal0X128;
                info.feeGrowthOutside1X128 = feeGrowthGlobal1X128;
                info.secondsPerLiquidityOutsideX128 = secondsPerLiquidityCumulativeX128;
                info.tickCumulativeOutside = tickCumulative;
                info.secondsOutside = time;
            }
            info.initialized = true;
        }

        info.liquidityGross = liquidityGrossAfter;

        // when the lower (upper) tick is crossed left to right (right to left), liquidity must be added (removed)
        info.liquidityNet = upper
            ? int256(info.liquidityNet).sub(liquidityDelta).toInt128()
            : int256(info.liquidityNet).add(liquidityDelta).toInt128();
    }

    /// @notice Updates the staked liquidity component of a tick. Assumes tick is already initialized with an existing position.
    /// @notice We reuse existing liquidity for staking, so there is no change in liquidity
    /// @param self The mapping containing all tick information for initialized ticks
    /// @param tick The tick that will be updated
    /// @param stakedLiquidityDelta The amount of staked liquidity to be added (subtracted) when tick is crossed from left to right (right to left)
    /// @param upper true for updating a position's upper tick, or false for updating a position's lower tick
    function updateStake(mapping(int24 => Tick.Info) storage self, int24 tick, int128 stakedLiquidityDelta, bool upper)
        internal
    {
        Tick.Info storage info = self[tick];
        // when the lower (upper) tick is crossed left to right (right to left), staked liquidity must be added (removed)
        info.stakedLiquidityNet = upper
            ? int256(info.stakedLiquidityNet).sub(stakedLiquidityDelta).toInt128()
            : int256(info.stakedLiquidityNet).add(stakedLiquidityDelta).toInt128();
    }

    /// @notice Clears tick data
    /// @param self The mapping containing all initialized tick information for initialized ticks
    /// @param tick The tick that will be cleared
    function clear(mapping(int24 => Tick.Info) storage self, int24 tick) internal {
        delete self[tick];
    }

    /// @notice Transitions to next tick as needed by price movement
    /// @param self The mapping containing all tick information for initialized ticks
    /// @param tick The destination tick of the transition
    /// @param feeGrowthGlobal0X128 The all-time global fee growth, per unit of liquidity, in token0
    /// @param feeGrowthGlobal1X128 The all-time global fee growth, per unit of liquidity, in token1
    /// @param secondsPerLiquidityCumulativeX128 The current seconds per liquidity
    /// @param tickCumulative The tick * time elapsed since the pool was first initialized
    /// @param time The current block.timestamp
    /// @param rewardGrowthGlobalX128 The all-time global reward growth, per unit of liquidity
    /// @return nets The amount of liquidity and staked liquidity added (subtracted) when tick is crossed from left to right (right to left)
    function cross(
        mapping(int24 => Tick.Info) storage self,
        int24 tick,
        uint256 feeGrowthGlobal0X128,
        uint256 feeGrowthGlobal1X128,
        uint160 secondsPerLiquidityCumulativeX128,
        int56 tickCumulative,
        uint32 time,
        uint256 rewardGrowthGlobalX128
    ) internal returns (LiquidityNets memory nets) {
        Tick.Info storage info = self[tick];
        info.feeGrowthOutside0X128 = feeGrowthGlobal0X128 - info.feeGrowthOutside0X128;
        info.feeGrowthOutside1X128 = feeGrowthGlobal1X128 - info.feeGrowthOutside1X128;
        info.rewardGrowthOutsideX128 = rewardGrowthGlobalX128 - info.rewardGrowthOutsideX128;
        info.secondsPerLiquidityOutsideX128 = secondsPerLiquidityCumulativeX128 - info.secondsPerLiquidityOutsideX128;
        info.tickCumulativeOutside = tickCumulative - info.tickCumulativeOutside;
        info.secondsOutside = time - info.secondsOutside;
        nets.liquidityNet = info.liquidityNet;
        nets.stakedLiquidityNet = info.stakedLiquidityNet;
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.5.0;

/// @title Callback for ICLPoolActions#swap
/// @notice Any contract that calls ICLPoolActions#swap must implement this interface
interface ICLSwapCallback {
    /// @notice Called to `msg.sender` after executing a swap via ICLPool#swap.
    /// @dev In the implementation you must pay the pool tokens owed for the swap.
    /// The caller of this method must be checked to be a CLPool deployed by the canonical CLFactory.
    /// amount0Delta and amount1Delta can both be 0 if no tokens were swapped.
    /// @param amount0Delta The amount of token0 that was sent (negative) or must be received (positive) by the pool by
    /// the end of the swap. If positive, the callback must send that amount of token0 to the pool.
    /// @param amount1Delta The amount of token1 that was sent (negative) or must be received (positive) by the pool by
    /// the end of the swap. If positive, the callback must send that amount of token1 to the pool.
    /// @param data Any data passed through by the caller via the ICLPoolActions#swap call
    function uniswapV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external;
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.5.0;

/// @title Callback for ICLPoolActions#flash
/// @notice Any contract that calls ICLPoolActions#flash must implement this interface
interface ICLFlashCallback {
    /// @notice Called to `msg.sender` after transferring to the recipient from ICLPool#flash.
    /// @dev In the implementation you must repay the pool the tokens sent by flash plus the computed fee amounts.
    /// The caller of this method must be checked to be a CLPool deployed by the canonical CLFactory.
    /// @param fee0 The fee amount in token0 due to the pool by the end of the flash
    /// @param fee1 The fee amount in token1 due to the pool by the end of the flash
    /// @param data Any data passed through by the caller via the ICLPoolActions#flash call
    function uniswapV3FlashCallback(uint256 fee0, uint256 fee1, bytes calldata data) external;
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.5.0;

import "./pool/ICLPoolConstants.sol";
import "./pool/ICLPoolState.sol";
import "./pool/ICLPoolDerivedState.sol";
import "./pool/ICLPoolActions.sol";
import "./pool/ICLPoolOwnerActions.sol";
import "./pool/ICLPoolEvents.sol";

/// @title The interface for a CL Pool
/// @notice A CL pool facilitates swapping and automated market making between any two assets that strictly conform
/// to the ERC20 specification
/// @dev The pool interface is broken up into many smaller pieces
interface ICLPool is
    ICLPoolConstants,
    ICLPoolState,
    ICLPoolDerivedState,
    ICLPoolActions,
    ICLPoolEvents,
    ICLPoolOwnerActions
{}


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

import "forge-std/Script.sol";
import "forge-std/console2.sol";

// Mocks
import {MockERC20} from "contracts/mocks/MockERC20.sol";
import {MockNative} from "contracts/mocks/MockNative.sol";

// CL Core
import {CLPool} from "contracts/core/CLPool.sol";
import {CLFactory} from "contracts/core/CLFactory.sol";

// CL NFT
import {NonfungibleTokenPositionDescriptor} from "contracts/periphery/NonfungibleTokenPositionDescriptor.sol";
import {NonfungiblePositionManager} from "contracts/periphery/NonfungiblePositionManager.sol";

// CL Fees
import {DynamicSwapFeeModule} from "contracts/core/fees/DynamicSwapFeeModule.sol";
import {CustomUnstakedFeeModule} from "contracts/core/fees/CustomUnstakedFeeModule.sol";
import {CustomProtocolFeeModule} from "contracts/core/fees/CustomProtocolFeeModule.sol";

// CL Periphery
import {QuoterV2} from "contracts/periphery/lens/QuoterV2.sol";
import {SwapRouter} from "contracts/periphery/SwapRouter.sol";

/**
 * @title FullCLDeployment
 * @notice Deploys all contracts necessary for the CL system
 * @dev This is meant to be used as a dependency to export the relevant CL
 *      contract bytecodes for use by the ve33 system. Configuration
 *      MUST be done by contracts that utilize the system
 */
contract FullCLDeployment is Script {
    // Deployer
    address public deployer;

    // Token addresses
    address public USDC;
    address public USDT;
    address public DAI;
    address public WETH;

    // Deployment addresses
    address public poolFactory;
    address public poolImplementation;
    address public nonfungiblePositionManager;
    address public nftPositionDescriptor;
    address public swapRouter;
    address public quoter;
    address public swapFeeModule;
    address public unstakedFeeModule;
    address public protocolFeeModule;

    function run() public virtual {
        console2.log("=== Deploying all CL contracts for ve33 Integration ===");

        // Store deployer of contacts
        deployer = msg.sender;

        // Deploy USDC, USDT, DAI, and WETH
        USDC = address(new MockERC20("USDC", "USDC", 6));
        USDT = address(new MockERC20("USDT", "USDT", 6));
        DAI = address(new MockERC20("DAI", "DAI", 18));
        WETH = address(new MockNative("WETH", "WETH"));

        // Deploy Pool Factory & Pool Implementation
        (poolFactory, poolImplementation) = _deployCLCore();

        // Deploy NFT contracts
        (nftPositionDescriptor, nonfungiblePositionManager) = _deployCLNFT();

        // Deploy Fee contracts
        (swapFeeModule, unstakedFeeModule, protocolFeeModule) = _deployCLFees();

        // Deploy Periphery contracts
        (quoter, swapRouter) = _deployCLPeriphery();

        console2.log("=== Deployment Complete ===");
        console2.log("");
        console2.log("Summary:");
        console2.log("- PoolFactory:", poolFactory);
        console2.log("- PoolImplementation:", poolImplementation);
        console2.log(
            "- NonfungiblePositionManager:",
            nonfungiblePositionManager
        );
        console2.log("- SwapRouter:", swapRouter);
        console2.log("- Quoter:", quoter);
        console2.log("");
    }

    function _deployCLCore() internal returns (address, address) {
        address poolImplementation_ = address(new CLPool());
        return (
            address(
                new CLFactory({
                    _poolImplementation: address(poolImplementation_)
                })
            ),
            poolImplementation_
        );
    }

    function _deployCLNFT() internal returns (address, address) {
        address nftDescriptor = address(
            new NonfungibleTokenPositionDescriptor({
                _WETH9: WETH,
                _nativeCurrencyLabelBytes: bytes32("ETH")
            })
        );
        return (
            nftDescriptor,
            address(
                new NonfungiblePositionManager({
                    _factory: poolFactory,
                    _WETH9: WETH,
                    _tokenDescriptor: nftDescriptor,
                    name: "Concentrated Liquidity Positions NFT",
                    symbol: "CL-POS"
                })
            )
        );
    }

    function _deployCLFees() internal returns (address, address, address) {
        // Prepare initial fee configuration
        address[] memory initialPools = new address[](0);
        uint24[] memory initialFees = new uint24[](0);

        return (
            address(
                new DynamicSwapFeeModule({
                    _factory: poolFactory,
                    _defaultScalingFactor: 10000, // 1 basis point per tick deviation
                    _defaultFeeCap: 10000, // 1% max total fee
                    _pools: initialPools,
                    _fees: initialFees
                })
            ),
            address(new CustomUnstakedFeeModule({_factory: poolFactory})),
            address(new CustomProtocolFeeModule({_factory: poolFactory}))
        );
    }

    function _deployCLPeriphery() internal returns (address, address) {
        return (
            address(new QuoterV2({_factory: poolFactory, _WETH9: WETH})),
            address(new SwapRouter({_factory: poolFactory, _WETH9: WETH}))
        );
    }
}

