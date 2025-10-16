
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity =0.7.6;

import "../interfaces/ICLPool.sol";
import "../interfaces/fees/IDynamicFeeModule.sol";
import "../libraries/FullMath.sol";

contract DynamicSwapFeeModule is IDynamicFeeModule {
    struct DynamicFeeConfig {
        uint24 baseFee;
        uint24 feeCap;
        uint64 scalingFactor; // K
    }

    uint32 public constant MIN_SECONDS_AGO = 2; // it must be set to the block time
    uint32 public constant MAX_SECONDS_AGO = 65535 * MIN_SECONDS_AGO; // 65535 is the maximum number of slots available in the oracle

    uint256 public constant MAX_BASE_FEE = 500_000; // 50% - for launch anti-MEV protection
    uint256 public constant MAX_DISCOUNT = 500_000; // 50%
    // Override to indicate there is custom 0% fee - as a 0 value in the customFee mapping indicates
    // that no custom fee rate has been set
    uint256 public constant ZERO_FEE_INDICATOR = 420;

    uint256 public constant MAX_SCALING_FACTOR = 1e18;
    uint256 public constant SCALING_PRECISION = 1e6;
    uint256 public constant MAX_FEE_CAP = 500_000; // 50% - for launch anti-MEV protection

    /// @inheritdoc IDynamicFeeModule
    uint256 public override defaultScalingFactor; // default K
    /// @inheritdoc IDynamicFeeModule
    uint256 public override defaultFeeCap;
    /// @inheritdoc IDynamicFeeModule
    uint32 public override secondsAgo = 600; // 10 minutes

    /// @inheritdoc IFeeModule
    ICLFactory public immutable override factory;
    /// @inheritdoc IDynamicFeeModule
    mapping(address => uint24) public override discounted;
    /// @inheritdoc IDynamicFeeModule
    mapping(address => DynamicFeeConfig) public override dynamicFeeConfig;

    constructor(
        address _factory,
        uint256 _defaultScalingFactor,
        uint256 _defaultFeeCap,
        address[] memory _pools,
        uint24[] memory _fees
    ) {
        require(_defaultScalingFactor <= MAX_SCALING_FACTOR, "ISF");
        require(_defaultFeeCap <= MAX_FEE_CAP, "MFC");

        factory = ICLFactory(_factory);
        defaultScalingFactor = _defaultScalingFactor;
        defaultFeeCap = _defaultFeeCap;
        _bulkUpdateFees(ICLFactory(_factory), _pools, _fees);

        emit DefaultScalingFactorSet({defaultScalingFactor: _defaultScalingFactor});
        emit DefaultFeeCapSet({defaultFeeCap: _defaultFeeCap});
    }

    modifier onlySwapFeeManager() {
        require(msg.sender == factory.swapFeeManager(), "NFM");
        _;
    }

    /// @inheritdoc ICustomFeeModule
    function customFee(address _pool) external view override returns (uint24) {
        return dynamicFeeConfig[_pool].baseFee;
    }

    /// @inheritdoc ICustomFeeModule
    function setCustomFee(address _pool, uint24 _fee) external override onlySwapFeeManager {
        require(_fee <= MAX_BASE_FEE || _fee == ZERO_FEE_INDICATOR, "MBF");
        require(factory.isPool(_pool));

        dynamicFeeConfig[_pool].baseFee = _fee;
        emit SetCustomFee({pool: _pool, fee: _fee});
    }

    /// @inheritdoc IDynamicFeeModule
    function setDefaultScalingFactor(uint256 _defaultScalingFactor) external override onlySwapFeeManager {
        require(_defaultScalingFactor <= MAX_SCALING_FACTOR, "ISF");

        defaultScalingFactor = _defaultScalingFactor;
        emit DefaultScalingFactorSet({defaultScalingFactor: _defaultScalingFactor});
    }

    /// @inheritdoc IDynamicFeeModule
    function setDefaultFeeCap(uint256 _defaultFeeCap) external override onlySwapFeeManager {
        require(_defaultFeeCap <= MAX_FEE_CAP, "MFC");

        defaultFeeCap = _defaultFeeCap;
        emit DefaultFeeCapSet({defaultFeeCap: _defaultFeeCap});
    }

    /// @inheritdoc IDynamicFeeModule
    function setScalingFactor(address _pool, uint64 _scalingFactor) external override onlySwapFeeManager {
        require(factory.isPool(_pool));
        require(dynamicFeeConfig[_pool].feeCap != 0 && _scalingFactor <= MAX_SCALING_FACTOR, "ISF");

        dynamicFeeConfig[_pool].scalingFactor = _scalingFactor;
        emit ScalingFactorSet({pool: _pool, scalingFactor: _scalingFactor});
    }

    /// @inheritdoc IDynamicFeeModule
    function setFeeCap(address _pool, uint24 _feeCap) external override onlySwapFeeManager {
        require(factory.isPool(_pool));
        require(_feeCap > 0, "FC0");
        require(_feeCap <= MAX_FEE_CAP, "MFC");

        dynamicFeeConfig[_pool].feeCap = _feeCap;
        emit FeeCapSet({pool: _pool, feeCap: _feeCap});
    }

    /// @inheritdoc IDynamicFeeModule
    function resetDynamicFee(address _pool) external override onlySwapFeeManager {
        require(factory.isPool(_pool));

        delete dynamicFeeConfig[_pool].feeCap;
        delete dynamicFeeConfig[_pool].scalingFactor;
        emit DynamicFeeReset({pool: _pool});
    }

    /// @inheritdoc IDynamicFeeModule
    function setSecondsAgo(uint32 _secondsAgo) external override onlySwapFeeManager {
        require(_secondsAgo >= MIN_SECONDS_AGO && _secondsAgo < MAX_SECONDS_AGO, "ISA");

        secondsAgo = _secondsAgo;
        emit SecondsAgoSet({secondsAgo: _secondsAgo});
    }

    /// @inheritdoc IDynamicFeeModule
    function registerDiscounted(address _discountReceiver, uint24 _discount) external override onlySwapFeeManager {
        require(_discount <= MAX_DISCOUNT, "MDC");

        discounted[_discountReceiver] = _discount;
        emit DiscountedRegistered({discountReceiver: _discountReceiver, discount: _discount});
    }

    /// @inheritdoc IDynamicFeeModule
    function deregisterDiscounted(address _discountOver) external override onlySwapFeeManager {
        delete discounted[_discountOver];
        emit DiscountedDeregistered({discountOver: _discountOver});
    }

    /// @inheritdoc IDynamicFeeModule
    function bulkUpdateFees(address[] calldata _pools, uint24[] calldata _fees) external override onlySwapFeeManager {
        _bulkUpdateFees(factory, _pools, _fees);
    }

    /// @inheritdoc IFeeModule
    function getFee(address _pool) external view override returns (uint24) {
        DynamicFeeConfig storage dfc = dynamicFeeConfig[_pool];
        uint256 baseFee = dfc.baseFee;
        uint256 scalingFactor = dfc.scalingFactor;
        uint256 feeCap = dfc.feeCap;

        if (baseFee == ZERO_FEE_INDICATOR) return 0;
        baseFee = baseFee != 0 ? baseFee : factory.tickSpacingToFee(ICLPool(_pool).tickSpacing());

        if (scalingFactor == 0) {
            scalingFactor = defaultScalingFactor;
            feeCap = defaultFeeCap;
        }
        uint256 totalFee = baseFee + _getDynamicFee(_pool, scalingFactor);
        totalFee = totalFee < feeCap ? totalFee : feeCap;

        // apply discount if any
        if (discounted[tx.origin] > 0) {
            uint256 discount = FullMath.mulDivRoundingUp(totalFee, discounted[tx.origin], 1_000_000);
            totalFee = totalFee - discount;
        }

        return uint24(totalFee);
    }

    function _getDynamicFee(address _pool, uint256 _scalingFactor) internal view returns (uint256) {
        (, int24 currentTick,, uint16 observationCardinality,,) = ICLPool(_pool).slot0();
        uint32 _secondsAgo = secondsAgo;

        if (observationCardinality < _secondsAgo / MIN_SECONDS_AGO) return 0;

        uint32[] memory sa = new uint32[](2);
        sa[0] = _secondsAgo;
        // sa[1] = 0; default is 0

        int24 twAvgTick;
        try ICLPool(_pool).observe(sa) returns (int56[] memory tickCumulatives, uint160[] memory) {
            twAvgTick = int24((tickCumulatives[1] - tickCumulatives[0]) / _secondsAgo);
        } catch {
            return 0;
        }

        int24 tickDelta = currentTick - twAvgTick;
        uint24 absTickDelta = tickDelta < 0 ? uint24(-tickDelta) : uint24(tickDelta);
        return absTickDelta * _scalingFactor / SCALING_PRECISION;
    }

    function _bulkUpdateFees(ICLFactory _factory, address[] memory _pools, uint24[] memory _fees) internal {
        uint256 poolsLength = _pools.length;
        require(poolsLength == _fees.length, "LMM");

        uint24 fee;
        address pool;
        for (uint256 i = 0; i < poolsLength; i++) {
            fee = _fees[i];
            require(fee <= MAX_BASE_FEE || fee == ZERO_FEE_INDICATOR, "MBF");
            pool = _pools[i];
            require(_factory.isPool(pool));
            dynamicFeeConfig[pool].baseFee = fee;
            emit SetCustomFee({pool: pool, fee: fee});
        }
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity >=0.4.0 <0.8.0;

/// @title Contains 512-bit math functions
/// @notice Facilitates multiplication and division that can have overflow of an intermediate value without any loss of precision
/// @dev Handles "phantom overflow" i.e., allows multiplication and division where an intermediate value overflows 256 bits
library FullMath {
    /// @notice Calculates floor(a×b÷denominator) with full precision. Throws if result overflows a uint256 or denominator == 0
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @param denominator The divisor
    /// @return result The 256-bit result
    /// @dev Credit to Remco Bloemen under MIT license https://xn--2-umb.com/21/muldiv
    function mulDiv(uint256 a, uint256 b, uint256 denominator) internal pure returns (uint256 result) {
        // 512-bit multiply [prod1 prod0] = a * b
        // Compute the product mod 2**256 and mod 2**256 - 1
        // then use the Chinese Remainder Theorem to reconstruct
        // the 512 bit result. The result is stored in two 256
        // variables such that product = prod1 * 2**256 + prod0
        uint256 prod0; // Least significant 256 bits of the product
        uint256 prod1; // Most significant 256 bits of the product
        assembly {
            let mm := mulmod(a, b, not(0))
            prod0 := mul(a, b)
            prod1 := sub(sub(mm, prod0), lt(mm, prod0))
        }

        // Handle non-overflow cases, 256 by 256 division
        if (prod1 == 0) {
            require(denominator > 0);
            assembly {
                result := div(prod0, denominator)
            }
            return result;
        }

        // Make sure the result is less than 2**256.
        // Also prevents denominator == 0
        require(denominator > prod1);

        ///////////////////////////////////////////////
        // 512 by 256 division.
        ///////////////////////////////////////////////

        // Make division exact by subtracting the remainder from [prod1 prod0]
        // Compute remainder using mulmod
        uint256 remainder;
        assembly {
            remainder := mulmod(a, b, denominator)
        }
        // Subtract 256 bit number from 512 bit number
        assembly {
            prod1 := sub(prod1, gt(remainder, prod0))
            prod0 := sub(prod0, remainder)
        }

        // Factor powers of two out of denominator
        // Compute largest power of two divisor of denominator.
        // Always >= 1.
        uint256 twos = -denominator & denominator;
        // Divide denominator by power of two
        assembly {
            denominator := div(denominator, twos)
        }

        // Divide [prod1 prod0] by the factors of two
        assembly {
            prod0 := div(prod0, twos)
        }
        // Shift in bits from prod1 into prod0. For this we need
        // to flip `twos` such that it is 2**256 / twos.
        // If twos is zero, then it becomes one
        assembly {
            twos := add(div(sub(0, twos), twos), 1)
        }
        prod0 |= prod1 * twos;

        // Invert denominator mod 2**256
        // Now that denominator is an odd number, it has an inverse
        // modulo 2**256 such that denominator * inv = 1 mod 2**256.
        // Compute the inverse by starting with a seed that is correct
        // correct for four bits. That is, denominator * inv = 1 mod 2**4
        uint256 inv = (3 * denominator) ^ 2;
        // Now use Newton-Raphson iteration to improve the precision.
        // Thanks to Hensel's lifting lemma, this also works in modular
        // arithmetic, doubling the correct bits in each step.
        inv *= 2 - denominator * inv; // inverse mod 2**8
        inv *= 2 - denominator * inv; // inverse mod 2**16
        inv *= 2 - denominator * inv; // inverse mod 2**32
        inv *= 2 - denominator * inv; // inverse mod 2**64
        inv *= 2 - denominator * inv; // inverse mod 2**128
        inv *= 2 - denominator * inv; // inverse mod 2**256

        // Because the division is now exact we can divide by multiplying
        // with the modular inverse of denominator. This will give us the
        // correct result modulo 2**256. Since the precoditions guarantee
        // that the outcome is less than 2**256, this is the final result.
        // We don't need to compute the high bits of the result and prod1
        // is no longer required.
        result = prod0 * inv;
        return result;
    }

    /// @notice Calculates ceil(a×b÷denominator) with full precision. Throws if result overflows a uint256 or denominator == 0
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @param denominator The divisor
    /// @return result The 256-bit result
    function mulDivRoundingUp(uint256 a, uint256 b, uint256 denominator) internal pure returns (uint256 result) {
        result = mulDiv(a, b, denominator);
        if (mulmod(a, b, denominator) > 0) {
            require(result < type(uint256).max);
            result++;
        }
    }
}

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

// SPDX-License-Identifier: MIT
pragma solidity =0.7.6;
pragma abicoder v2;

import {IVotingEscrow} from "contracts/core/interfaces/IVotingEscrow.sol";
import {IFactoryRegistry} from "contracts/core/interfaces/IFactoryRegistry.sol";

interface IVoter {
    function ve() external view returns (IVotingEscrow);

    function vote(uint256 _tokenId, address[] calldata _poolVote, uint256[] calldata _weights) external;

    function gauges(address _pool) external view returns (address);

    function gaugeToFees(address _gauge) external view returns (address);

    function gaugeToBribes(address _gauge) external view returns (address);

    function createGauge(address _poolFactory, address _pool) external returns (address);

    function distribute(address gauge) external;

    function factoryRegistry() external view returns (IFactoryRegistry);

    /// @dev Utility to distribute to gauges of pools in array.
    /// @param _gauges Array of gauges to distribute to.
    function distribute(address[] memory _gauges) external;

    function isAlive(address _gauge) external view returns (bool);

    function killGauge(address _gauge) external;

    function emergencyCouncil() external view returns (address);

    /// @notice Claim emissions from gauges.
    /// @param _gauges Array of gauges to collect emissions from.
    function claimRewards(address[] memory _gauges) external;

    /// @notice Claim fees for a given NFT.
    /// @dev Utility to help batch fee claims.
    /// @param _fees    Array of FeesVotingReward contracts to collect from.
    /// @param _tokens  Array of tokens that are used as fees.
    /// @param _tokenId Id of veNFT that you wish to claim fees for.
    function claimFees(address[] memory _fees, address[][] memory _tokens, uint256 _tokenId) external;
}

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

// SPDX-License-Identifier: MIT
pragma solidity =0.7.6;

interface IFactoryRegistry {
    function approve(address poolFactory, address votingRewardsFactory, address gaugeFactory) external;

    function isPoolFactoryApproved(address poolFactory) external returns (bool);

    function factoriesToPoolFactory(address poolFactory)
        external
        returns (address votingRewardsFactory, address gaugeFactory);
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

// SPDX-License-Identifier: MIT
pragma solidity =0.7.6;

import "./ICustomFeeModule.sol";

interface IDynamicFeeModule is ICustomFeeModule {
    event ScalingFactorSet(address indexed pool, uint256 indexed scalingFactor);
    event FeeCapSet(address indexed pool, uint256 indexed feeCap);
    event DynamicFeeReset(address indexed pool);
    event DefaultScalingFactorSet(uint256 indexed defaultScalingFactor);
    event DefaultFeeCapSet(uint256 indexed defaultFeeCap);
    event DiscountedRegistered(address indexed discountReceiver, uint24 indexed discount);
    event DiscountedDeregistered(address indexed discountOver);
    event SecondsAgoSet(uint32 indexed secondsAgo);

    /// @notice Returns the dynamic fee config for the pool
    /// @param _pool The pool address
    /// @return The baseFee, feeCap and scalingFactor set for the pool
    function dynamicFeeConfig(address _pool) external view returns (uint24, uint24, uint64);

    /// @notice The current default scaling factor
    function defaultScalingFactor() external view returns (uint256);

    /// @notice The current default fee cap
    function defaultFeeCap() external view returns (uint256);

    /// @notice The amount of time used to calculate price change
    function secondsAgo() external view returns (uint32);

    /// @notice Returns the discount in fees
    /// @param _sender The address to check if it's discounted
    /// @return The amount of discount the _sender eligible for
    function discounted(address _sender) external view returns (uint24);

    /// @notice Sets the new default scaling factor
    /// @dev Must be called by the current fee manager
    /// @param _defaultScalingFactor The new default scaling factor for dynamic fees
    function setDefaultScalingFactor(uint256 _defaultScalingFactor) external;

    /// @notice Sets the new default fee cap
    /// @dev Must be called by the current fee manager
    /// @param _defaultFeeCap The new default fee cap for dynamic fees
    function setDefaultFeeCap(uint256 _defaultFeeCap) external;

    /// @notice Sets the new scaling factor on the passed pool
    /// @dev Must be called by the current fee manager
    /// @dev Pool must exist
    /// @dev Must set feeCap first
    /// @param _pool The pool address
    /// @param _scalingFactor The new scaling factor for dynamic fees
    function setScalingFactor(address _pool, uint64 _scalingFactor) external;

    /// @notice Sets the new fee cap on the passed pool
    /// @dev Must be called by the current fee manager
    /// @dev Pool must exist
    /// @param _pool The pool address
    /// @param _feeCap The new fee cap for dynamic fees
    function setFeeCap(address _pool, uint24 _feeCap) external;

    /// @notice Resets the dynamic fee for a given pool
    /// @dev Must be called by the current fee manager
    /// @dev Pool must exist
    /// @param _pool The address of the pool for which the dynamic fee is being reset
    function resetDynamicFee(address _pool) external;

    /// @notice Sets the new secondsAgo
    /// @dev Must be called by the current fee manager
    /// @param _secondsAgo The new secondsAgo for price change calculation
    function setSecondsAgo(uint32 _secondsAgo) external;

    /// @notice Registers a new address to receive fee discount
    /// @dev Must be called by the current fee manager
    /// @param _discountReceiver Address to register for fee discount
    /// @param _discount The amount of discount in basis points
    function registerDiscounted(address _discountReceiver, uint24 _discount) external;

    /// @notice Deregisters address to receive fee discount
    /// @dev Must be called by the current fee manager
    /// @param _discountOver Address to deregister from fee discount
    function deregisterDiscounted(address _discountOver) external;

    /// @notice Bulk updates the fee for the passed in pools
    /// @dev Must be called by the current fee manager
    /// @param _pools The pool addresses which are going to be updated (must be a valid pool)
    /// @param _fees The fees to be set on the pools
    function bulkUpdateFees(address[] calldata _pools, uint24[] calldata _fees) external;
}


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

