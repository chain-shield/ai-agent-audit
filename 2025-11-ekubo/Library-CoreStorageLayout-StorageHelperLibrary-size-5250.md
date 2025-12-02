
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolId} from "../types/poolId.sol";
import {PositionId} from "../types/positionId.sol";
import {StorageSlot} from "../types/storageSlot.sol";

/// @title Core Storage Layout
/// @notice Library providing functions to compute the storage locations for the Core contract
/// @dev Core uses a custom storage layout to avoid keccak's where possible.
///      For certain storage values, the pool id is used as a base offset and
///      we allocate the following relative offsets (starting from the pool id) as:
///        0: pool state
///        [FPL_OFFSET, FPL_OFFSET + 1]: fees per liquidity
///        [TICKS_OFFSET + MIN_TICK, TICKS_OFFSET + MAX_TICK]: tick info
///        [FPL_OUTSIDE_OFFSET_VALUE0 + MIN_TICK, FPL_OUTSIDE_OFFSET_VALUE0 + MAX_TICK]: fees per liquidity outside (value0)
///        [FPL_OUTSIDE_OFFSET_VALUE0 + FPL_OUTSIDE_OFFSET_VALUE1 + MIN_TICK, FPL_OUTSIDE_OFFSET_VALUE0 + FPL_OUTSIDE_OFFSET_VALUE1 + MAX_TICK]: fees per liquidity outside (value1)
///        [BITMAPS_OFFSET + FIRST_BITMAP_WORD, BITMAPS_OFFSET + LAST_BITMAP_WORD]: tick bitmaps
library CoreStorageLayout {
    /// @dev Generated using: cast keccak "CoreStorageLayout#FPL_OFFSET"
    uint256 internal constant FPL_OFFSET = 0xb09b03866d96933565a9435bfb511c8ac5b2be454285ca331201452704799f72;
    /// @dev Generated using: cast keccak "CoreStorageLayout#TICKS_OFFSET"
    uint256 internal constant TICKS_OFFSET = 0x435a5eb89a296820174331cf5a3902d9fca683928d56726d8e7acd6efb28c568;
    /// @dev Generated using: cast keccak "CoreStorageLayout#FPL_OUTSIDE_OFFSET_VALUE0"
    uint256 internal constant FPL_OUTSIDE_OFFSET_VALUE0 =
        0x5695060fdb9cfea656f872ae4887221aff7dbfefc45eaf753e4e70cdfb5cd19c;
    /// @dev Generated using: cast keccak "CoreStorageLayout#FPL_OUTSIDE_OFFSET_VALUE1"
    uint256 internal constant FPL_OUTSIDE_OFFSET_VALUE1 =
        0x7a2a03fc08af3dae7869678617dc8abe8f15a3b719b37ba108dba879571f8b02;
    /// @dev Generated using: cast keccak "CoreStorageLayout#BITMAPS_OFFSET"
    uint256 internal constant BITMAPS_OFFSET = 0x3def450d0010a2fef515ce5eba4b363b5a0f42fdd4c53e1c737975db05a2e3a5;

    /// @notice Computes the storage slot containing information on whether an extension is registered
    /// @param extension The extension address to check
    /// @return slot The storage slot in the Core contract
    function isExtensionRegisteredSlot(address extension) internal pure returns (StorageSlot slot) {
        assembly ("memory-safe") {
            mstore(0, extension)
            mstore(32, 0)
            slot := keccak256(0, 64)
        }
    }

    /// @notice Computes the storage slot of the current state of a pool
    /// @param poolId The unique identifier for the pool
    /// @return slot The storage slot in the Core contract
    function poolStateSlot(PoolId poolId) internal pure returns (StorageSlot slot) {
        slot = StorageSlot.wrap(PoolId.unwrap(poolId));
    }

    /// @notice Computes the storage slots of the current global fees per liquidity of a pool
    /// @param poolId The unique identifier for the pool
    /// @return firstSlot The first of two consecutive storage slots in the Core contract
    function poolFeesPerLiquiditySlot(PoolId poolId) internal pure returns (StorageSlot firstSlot) {
        assembly ("memory-safe") {
            firstSlot := add(poolId, FPL_OFFSET)
        }
    }

    /// @notice Computes the storage slot of tick information for a specific tick in a pool
    /// @param poolId The unique identifier for the pool
    /// @param tick The tick to query
    /// @return slot The storage slot in the Core contract
    function poolTicksSlot(PoolId poolId, int32 tick) internal pure returns (StorageSlot slot) {
        assembly ("memory-safe") {
            slot := add(poolId, add(tick, TICKS_OFFSET))
        }
    }

    /// @notice Computes the storage slots of the outside fees of a pool for a given tick
    /// @param poolId The unique identifier for the pool
    /// @param tick The tick to query
    /// @return firstSlot The first storage slot in the Core contract
    /// @return secondSlot The second storage slot in the Core contract
    function poolTickFeesPerLiquidityOutsideSlot(PoolId poolId, int32 tick)
        internal
        pure
        returns (StorageSlot firstSlot, StorageSlot secondSlot)
    {
        assembly ("memory-safe") {
            firstSlot := add(poolId, add(FPL_OUTSIDE_OFFSET_VALUE0, tick))
            secondSlot := add(firstSlot, FPL_OUTSIDE_OFFSET_VALUE1)
        }
    }

    /// @notice Computes the first storage slot of the tick bitmaps for a specific pool
    /// @param poolId The unique identifier for the pool
    /// @return firstSlot The first storage slot in the Core contract
    function tickBitmapsSlot(PoolId poolId) internal pure returns (StorageSlot firstSlot) {
        assembly ("memory-safe") {
            firstSlot := add(poolId, BITMAPS_OFFSET)
        }
    }

    /// @notice Computes the first storage slot of the position data for a specific position in a pool
    /// @param poolId The unique identifier for the pool
    /// @param owner The position owner
    /// @param positionId The unique identifier for the position
    /// @return firstSlot The first of three consecutive storage slots in the Core contract
    function poolPositionsSlot(PoolId poolId, address owner, PositionId positionId)
        internal
        pure
        returns (StorageSlot firstSlot)
    {
        assembly ("memory-safe") {
            let free := mload(0x40)
            mstore(free, positionId)
            mstore(add(free, 0x20), poolId)
            mstore(add(free, 0x40), owner)
            mstore(0, keccak256(free, 0x60))
            mstore(32, 1)
            firstSlot := keccak256(0, 64)
        }
    }

    /// @notice Computes the storage slot for saved balances
    /// @param owner The owner of the saved balances
    /// @param token0 The first token address
    /// @param token1 The second token address
    /// @param salt The salt used for the saved balance key
    /// @return slot The storage slot in the Core contract
    function savedBalancesSlot(address owner, address token0, address token1, bytes32 salt)
        internal
        pure
        returns (StorageSlot slot)
    {
        assembly ("memory-safe") {
            let free := mload(0x40)
            mstore(free, owner)
            mstore(add(free, 0x20), token0)
            mstore(add(free, 0x40), token1)
            mstore(add(free, 0x60), salt)
            slot := keccak256(free, 128)
        }
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a pool
/// @dev Wraps bytes32 to provide type safety for pool identifiers
type PoolId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MIN_TICK, MAX_TICK} from "../math/constants.sol";
import {PoolConfig} from "./poolConfig.sol";

type PositionId is bytes32;

using {validate, salt, tickLower, tickUpper} for PositionId global;

function salt(PositionId positionId) pure returns (bytes24 v) {
    assembly ("memory-safe") {
        v := shl(64, shr(64, positionId))
    }
}

function tickLower(PositionId positionId) pure returns (int32 v) {
    assembly ("memory-safe") {
        // shift down, then signextend to 32 bits
        v := signextend(3, shr(32, positionId))
    }
}

function tickUpper(PositionId positionId) pure returns (int32 v) {
    assembly ("memory-safe") {
        // lowest 4 bytes, then signextend to 32 bits
        v := signextend(3, positionId)
    }
}

function createPositionId(bytes24 _salt, int32 _tickLower, int32 _tickUpper) pure returns (PositionId v) {
    assembly ("memory-safe") {
        // v = salt | (tickLower << 32) | tickUpper
        v := or(shl(64, shr(64, _salt)), or(shl(32, and(_tickLower, 0xFFFFFFFF)), and(_tickUpper, 0xFFFFFFFF)))
    }
}

/// @notice Thrown when the order of the position bounds is invalid, i.e. tickLower >= tickUpper
error BoundsOrder();
/// @notice Thrown when the bounds of the position are outside the pool's min/max tick range
error MinMaxBounds();
/// @notice Thrown when the ticks of the bounds do not align with tick spacing for concentrated pools
error BoundsTickSpacing();
/// @notice Thrown when stableswap pool positions are not at the min/max tick for the config
error StableswapMustBeFullRange();

function validate(PositionId positionId, PoolConfig config) pure {
    if (config.isConcentrated()) {
        if (positionId.tickLower() >= positionId.tickUpper()) revert BoundsOrder();
        if (positionId.tickLower() < MIN_TICK || positionId.tickUpper() > MAX_TICK) revert MinMaxBounds();
        int32 spacing = int32(config.concentratedTickSpacing());
        if (positionId.tickLower() % spacing != 0 || positionId.tickUpper() % spacing != 0) revert BoundsTickSpacing();
    } else {
        (int32 lower, int32 upper) = config.stableswapActiveLiquidityTickRange();
        // For stableswap pools, positions must be exactly min/max tick
        if (positionId.tickLower() != lower || positionId.tickUpper() != upper) revert StableswapMustBeFullRange();
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type StorageSlot is bytes32;

using {load, loadTwo, store, storeTwo, next, add, sub} for StorageSlot global;

function load(StorageSlot slot) view returns (bytes32 value) {
    assembly ("memory-safe") {
        value := sload(slot)
    }
}

function loadTwo(StorageSlot slot) view returns (bytes32 value0, bytes32 value1) {
    value0 = slot.load();
    value1 = slot.next().load();
}

function store(StorageSlot slot, bytes32 value) {
    assembly ("memory-safe") {
        sstore(slot, value)
    }
}

function storeTwo(StorageSlot slot, bytes32 value0, bytes32 value1) {
    slot.store(value0);
    slot.next().store(value1);
}

function next(StorageSlot slot) pure returns (StorageSlot nextSlot) {
    assembly ("memory-safe") {
        nextSlot := add(slot, 1)
    }
}

function add(StorageSlot slot, uint256 addend) pure returns (StorageSlot summedSlot) {
    assembly ("memory-safe") {
        summedSlot := add(slot, addend)
    }
}

function sub(StorageSlot slot, uint256 subtrahend) pure returns (StorageSlot differenceSlot) {
    assembly ("memory-safe") {
        differenceSlot := sub(slot, subtrahend)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// Protocol Constants
// Contains all constant values used throughout the Ekubo Protocol
// These constants define the boundaries and special values for the protocol's operation

// The minimum tick value supported by the protocol
// Corresponds to the minimum possible price ratio in the protocol
int32 constant MIN_TICK = -88722835;

// The maximum tick value supported by the protocol
// Corresponds to the maximum possible price ratio in the protocol
int32 constant MAX_TICK = 88722835;

// The maximum tick magnitude (absolute value of MAX_TICK)
// Used for validation and bounds checking in tick-related calculations
uint32 constant MAX_TICK_MAGNITUDE = uint32(MAX_TICK);

// The maximum allowed tick spacing for pools
// Defines the upper limit for tick spacing configuration in pool creation
uint32 constant MAX_TICK_SPACING = 698605;

// Address used to represent the native token (ETH) within the protocol
// Using address(0) allows the protocol to handle native ETH alongside ERC20 tokens
address constant NATIVE_TOKEN_ADDRESS = address(0);

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {MIN_TICK, MAX_TICK, MAX_TICK_SPACING} from "../math/constants.sol";

/// @notice Pool configuration packed into a single bytes32
/// @dev Contains extension address (20 bytes), fee (8 bytes), and pool type config (4 bytes)
/// Pool type config (32 bits):
///   - Bit 31: discriminator (1 = concentrated, 0 = stableswap)
///   - For concentrated (bit 31 = 1): bits 30-0 are tick spacing
///   - For stableswap (bit 31 = 0): bits 30-24 are amplification factor, bits 23-0 are center tick
type PoolConfig is bytes32;

using {
    fee,
    extension,
    isConcentrated,
    isStableswap,
    isFullRange,
    concentratedTickSpacing,
    stableswapAmplification,
    stableswapCenterTick,
    stableswapActiveLiquidityTickRange,
    stableswapLiquidityWidth,
    validate,
    concentratedMaxLiquidityPerTick
} for PoolConfig global;

/// @notice Extracts the fee from a pool config
/// @param config The pool config
/// @return r The fee
function fee(PoolConfig config) pure returns (uint64 r) {
    assembly ("memory-safe") {
        r := and(shr(32, config), 0xffffffffffffffff)
    }
}

/// @notice Extracts the extension address from a pool config
/// @param config The pool config
/// @return r The extension address
function extension(PoolConfig config) pure returns (address r) {
    assembly ("memory-safe") {
        r := shr(96, config)
    }
}

/// @notice Checks if this is a concentrated liquidity pool
/// @param config The pool config
/// @return r True if the pool is concentrated liquidity
function isConcentrated(PoolConfig config) pure returns (bool r) {
    r = !config.isStableswap();
}

/// @notice Extracts the tick spacing from a concentrated liquidity pool config
/// @dev Only valid for concentrated liquidity pools (isConcentrated() == true)
/// @param config The pool config
/// @return r The tick spacing
function concentratedTickSpacing(PoolConfig config) pure returns (uint32 r) {
    assembly ("memory-safe") {
        // Extract lower 31 bits (bits 30-0)
        r := and(config, 0x7fffffff)
    }
}

/// @notice Checks if this is a stableswap pool
/// @param config The pool config
/// @return r True if the pool is stableswap
function isStableswap(PoolConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        // = iff bit 31 is not set
        r := iszero(and(0x80000000, config))
    }
}

/// @notice Determines if this pool is full range (special case of stableswap with amplification=0 and center=0)
/// @dev Full range can be slightly optimized in that we don't need to compute the sqrt ratio at the tick boundaries
/// @param config The pool config
/// @return r True if the pool is full range
function isFullRange(PoolConfig config) pure returns (bool r) {
    assembly ("memory-safe") {
        // Full range when all 32 bits are 0 (discriminator=0, amplification=0, center=0)
        r := iszero(and(config, 0xffffffff))
    }
}

/// @notice Extracts the amplification factor from a stableswap pool config
/// @dev Only valid for stableswap pools (isStableswap() == true)
/// @param config The pool config
/// @return r The amplification factor (0-127)
function stableswapAmplification(PoolConfig config) pure returns (uint8 r) {
    assembly ("memory-safe") {
        // Extract bits 30-24
        r := and(shr(24, config), 0x7f)
    }
}

/// @notice Extracts the center tick from a stableswap pool config
/// @dev Only valid for stableswap pools (isStableswap() == true)
/// @dev The 24-bit center tick is scaled by 16 to get the actual tick
/// @param config The pool config
/// @return r The center tick
function stableswapCenterTick(PoolConfig config) pure returns (int32 r) {
    assembly ("memory-safe") {
        // Extract bits 23-0 and sign extend to 32 bits (24-bit signed integer)
        // Then multiply by 16, since the value does not have full precision
        r := mul(signextend(2, and(config, 0xffffff)), 16)
    }
}

/// @notice Returns the width of the liquidity range
function stableswapLiquidityWidth(PoolConfig config) pure returns (uint256 width) {
    uint8 amp = config.stableswapAmplification();

    assembly ("memory-safe") {
        width := shr(amp, MAX_TICK)
    }
}

/// @notice Computes the tick range where liquidity is active for stableswap pools
/// @param config The pool config
/// @return lower The lower tick of the bounds
/// @return upper The upper tick of the bounds
function stableswapActiveLiquidityTickRange(PoolConfig config) pure returns (int32 lower, int32 upper) {
    int32 center = config.stableswapCenterTick();
    uint256 width = config.stableswapLiquidityWidth();

    assembly ("memory-safe") {
        lower := sub(center, width)
        lower := add(lower, mul(sgt(MIN_TICK, lower), sub(MIN_TICK, lower)))

        upper := add(center, width)
        upper := sub(upper, mul(sgt(upper, MAX_TICK), sub(upper, MAX_TICK)))
    }
}

/// @notice Creates a PoolConfig for a concentrated liquidity pool
/// @param _fee The fee for the pool
/// @param _tickSpacing The tick spacing for the pool
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createConcentratedPoolConfig(uint64 _fee, uint32 _tickSpacing, address _extension)
    pure
    returns (PoolConfig c)
{
    assembly ("memory-safe") {
        // Set bit 31 to 1 for concentrated liquidity, then OR with tick spacing (bits 30-0)
        let typeConfig := or(0x80000000, and(_tickSpacing, 0x7fffffff))
        c := or(or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff))), typeConfig)
    }
}

/// @notice Creates a PoolConfig for a stableswap pool
/// @param _fee The fee for the pool
/// @param _amplification The amplification factor (0-127)
/// @param _centerTick The center tick (will be divided by 16 and stored as 24-bit value)
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createStableswapPoolConfig(uint64 _fee, uint8 _amplification, int32 _centerTick, address _extension)
    pure
    returns (PoolConfig c)
{
    assembly ("memory-safe") {
        // Divide center tick by 16 to get 24-bit representation
        let stableswapCenterTick24 := sdiv(_centerTick, 16)
        // Pack: bit 31 = 0 (stableswap), bits 30-24 = amplification, bits 23-0 = center tick
        let typeConfig := or(shl(24, and(_amplification, 0x7f)), and(stableswapCenterTick24, 0xffffff))
        c := or(or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff))), typeConfig)
    }
}

/// @notice Creates a PoolConfig for a full range pool (stableswap with amplification=0, center=0)
/// @param _fee The fee for the pool
/// @param _extension The extension address for the pool
/// @return c The packed configuration
function createFullRangePoolConfig(uint64 _fee, address _extension) pure returns (PoolConfig c) {
    assembly ("memory-safe") {
        // All 32 bits of type config are 0 (discriminator=0, amplification=0, center=0)
        c := or(shl(96, _extension), shl(32, and(_fee, 0xffffffffffffffff)))
    }
}

/// @notice Computes the maximum liquidity per tick for a given concentrated liquidity pool configuration.
/// @dev Only valid for concentrated liquidity pools. Stableswap pools don't use ticks.
/// @dev Calculated as type(uint128).max / (1 + (MAX_TICK_MAGNITUDE / tickSpacing) * 2)
/// @param config The concentrated liquidity pool configuration
/// @return maxLiquidity The maximum liquidity allowed to reference each tick
function concentratedMaxLiquidityPerTick(PoolConfig config) pure returns (uint128 maxLiquidity) {
    uint32 _tickSpacing = config.concentratedTickSpacing();

    assembly ("memory-safe") {
        // Calculate total number of usable ticks: 1 + (MAX_TICK_MAGNITUDE / tickSpacing) * 2
        // This represents all ticks from -MAX_TICK_MAGNITUDE to +MAX_TICK_MAGNITUDE, and tick 0
        let numTicks := add(1, mul(div(MAX_TICK, _tickSpacing), 2))

        maxLiquidity := div(sub(shl(128, 1), 1), numTicks)
    }
}

/// @notice Thrown when tick spacing exceeds the maximum allowed value
error InvalidTickSpacing();

/// @notice Thrown when amplification factor exceeds the maximum allowed value
error InvalidStableswapAmplification();

/// @notice Thrown when center tick is not between min and max tick
error InvalidCenterTick();

/// @notice Validates that a pool config is properly formatted
/// @param config The config to validate
function validate(PoolConfig config) pure {
    if (config.isConcentrated()) {
        if (config.concentratedTickSpacing() > MAX_TICK_SPACING || config.concentratedTickSpacing() == 0) {
            revert InvalidTickSpacing();
        }
    } else {
        // Stableswap pool: validate amplification factor <= 26
        if (config.stableswapAmplification() > 26) {
            revert InvalidStableswapAmplification();
        }
        int32 centerTick = config.stableswapCenterTick();
        if (centerTick < MIN_TICK || centerTick > MAX_TICK) {
            revert InvalidCenterTick();
        }
    }
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

