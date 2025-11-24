
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity ^0.8.30;

import {ICore, PoolKey, PositionId, CallPoints} from "../interfaces/ICore.sol";
import {IMEVCapture} from "../interfaces/extensions/IMEVCapture.sol";
import {IExtension} from "../interfaces/ICore.sol";
import {BaseExtension} from "../base/BaseExtension.sol";
import {BaseForwardee} from "../base/BaseForwardee.sol";
import {amountBeforeFee, computeFee} from "../math/fee.sol";
import {ExposedStorage} from "../base/ExposedStorage.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {ExposedStorageLib} from "../libraries/ExposedStorageLib.sol";
import {CoreStorageLayout} from "../libraries/CoreStorageLayout.sol";
import {PoolState} from "../types/poolState.sol";
import {MEVCapturePoolState, createMEVCapturePoolState} from "../types/mevCapturePoolState.sol";
import {SwapParameters} from "../types/swapParameters.sol";
import {PoolId} from "../types/poolId.sol";
import {Locker} from "../types/locker.sol";
import {StorageSlot} from "../types/storageSlot.sol";
import {PoolBalanceUpdate, createPoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";

function mevCaptureCallPoints() pure returns (CallPoints memory) {
    return CallPoints({
        // to store the initial tick
        beforeInitializePool: true,
        afterInitializePool: false,
        // so that we can prevent swaps that are not made via forward
        beforeSwap: true,
        afterSwap: false,
        // in order to accumulate any collected fees
        beforeUpdatePosition: true,
        afterUpdatePosition: false,
        // in order to accumulate any collected fees
        beforeCollectFees: true,
        afterCollectFees: false
    });
}

/// @notice Charges additional fees based on the relative size of the priority fee
contract MEVCapture is IMEVCapture, BaseExtension, BaseForwardee, ExposedStorage {
    using CoreLib for *;
    using ExposedStorageLib for *;

    constructor(ICore core) BaseExtension(core) BaseForwardee(core) {}

    function getPoolState(PoolId poolId) private view returns (MEVCapturePoolState state) {
        assembly ("memory-safe") {
            state := sload(poolId)
        }
    }

    function setPoolState(PoolId poolId, MEVCapturePoolState state) private {
        assembly ("memory-safe") {
            sstore(poolId, state)
        }
    }

    function getCallPoints() internal pure override returns (CallPoints memory) {
        return mevCaptureCallPoints();
    }

    function beforeInitializePool(address, PoolKey memory poolKey, int32 tick)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (poolKey.config.isStableswap()) {
            revert ConcentratedLiquidityPoolsOnly();
        }
        if (poolKey.config.fee() == 0) {
            // nothing to multiply == no-op extension
            revert NonzeroFeesOnly();
        }

        setPoolState({
            poolId: poolKey.toPoolId(),
            state: createMEVCapturePoolState({_lastUpdateTime: uint32(block.timestamp), _tickLast: tick})
        });
    }

    /// @notice We only allow swapping via forward to this extension
    function beforeSwap(Locker, PoolKey memory, SwapParameters) external pure override(BaseExtension, IExtension) {
        revert SwapMustHappenThroughForward();
    }

    // Allows users to collect pending fees before the first swap in the block happens
    function beforeCollectFees(Locker, PoolKey memory poolKey, PositionId)
        external
        override(BaseExtension, IExtension)
    {
        accumulatePoolFees(poolKey);
    }

    /// Prevents new liquidity from collecting on fees
    function beforeUpdatePosition(Locker, PoolKey memory poolKey, PositionId, int128)
        external
        override(BaseExtension, IExtension)
    {
        accumulatePoolFees(poolKey);
    }

    /// @inheritdoc IMEVCapture
    function accumulatePoolFees(PoolKey memory poolKey) public {
        PoolId poolId = poolKey.toPoolId();
        MEVCapturePoolState state = getPoolState(poolId);

        // the only thing we lock for is accumulating fees when the pool has not been updated in this block
        if (state.lastUpdateTime() != uint32(block.timestamp)) {
            address target = address(CORE);
            assembly ("memory-safe") {
                let o := mload(0x40)
                mstore(o, shl(224, 0xf83d08ba))
                mcopy(add(o, 4), poolKey, 96)
                mstore(add(o, 100), poolId)

                // If the call failed, pass through the revert
                if iszero(call(gas(), target, 0, o, 132, 0, 0)) {
                    returndatacopy(o, 0, returndatasize())
                    revert(o, returndatasize())
                }
            }
        }
    }

    function locked_6416899205(uint256) external onlyCore {
        PoolKey memory poolKey;
        PoolId poolId;
        assembly ("memory-safe") {
            // copy the poolkey out of calldata
            calldatacopy(poolKey, 36, 96)
            poolId := calldataload(132)
        }

        (int32 tick, uint128 fees0, uint128 fees1) = loadCoreState(poolId, poolKey.token0, poolKey.token1);

        if (fees0 != 0 || fees1 != 0) {
            CORE.accumulateAsFees(poolKey, fees0, fees1);
            unchecked {
                CORE.updateSavedBalances(
                    poolKey.token0,
                    poolKey.token1,
                    PoolId.unwrap(poolId),
                    -int256(uint256(fees0)),
                    -int256(uint256(fees1))
                );
            }
        }

        setPoolState({
            poolId: poolId,
            state: createMEVCapturePoolState({_lastUpdateTime: uint32(block.timestamp), _tickLast: tick})
        });
    }

    function loadCoreState(PoolId poolId, address token0, address token1)
        private
        view
        returns (int32 tick, uint128 fees0, uint128 fees1)
    {
        StorageSlot stateSlot = CoreStorageLayout.poolStateSlot(poolId);
        StorageSlot feesSlot = CoreStorageLayout.savedBalancesSlot(address(this), token0, token1, PoolId.unwrap(poolId));

        (bytes32 v0, bytes32 v1) = CORE.sload(stateSlot, feesSlot);
        tick = PoolState.wrap(v0).tick();

        assembly ("memory-safe") {
            fees0 := shr(128, v1)
            fees0 := sub(fees0, gt(fees0, 0))

            fees1 := shr(128, shl(128, v1))
            fees1 := sub(fees1, gt(fees1, 0))
        }
    }

    function handleForwardData(Locker, bytes memory data) internal override returns (bytes memory result) {
        unchecked {
            (PoolKey memory poolKey, SwapParameters params) = abi.decode(data, (PoolKey, SwapParameters));

            PoolId poolId = poolKey.toPoolId();
            MEVCapturePoolState state = getPoolState(poolId);
            uint32 lastUpdateTime = state.lastUpdateTime();
            int32 tickLast = state.tickLast();

            uint32 currentTime = uint32(block.timestamp);

            int256 saveDelta0;
            int256 saveDelta1;

            if (lastUpdateTime != currentTime) {
                (int32 tick, uint128 fees0, uint128 fees1) =
                    loadCoreState({poolId: poolId, token0: poolKey.token0, token1: poolKey.token1});

                if (fees0 != 0 || fees1 != 0) {
                    CORE.accumulateAsFees(poolKey, fees0, fees1);
                    // never overflows int256 container
                    saveDelta0 -= int256(uint256(fees0));
                    saveDelta1 -= int256(uint256(fees1));
                }

                tickLast = tick;
                setPoolState({
                    poolId: poolId,
                    state: createMEVCapturePoolState({_lastUpdateTime: currentTime, _tickLast: tickLast})
                });
            }

            (PoolBalanceUpdate balanceUpdate, PoolState stateAfter) = CORE.swap(0, poolKey, params);

            // however many tick spacings were crossed is the fee multiplier
            uint256 feeMultiplierX64 =
                (FixedPointMathLib.abs(stateAfter.tick() - tickLast) << 64) / poolKey.config.concentratedTickSpacing();
            uint64 poolFee = poolKey.config.fee();
            uint64 additionalFee = uint64(FixedPointMathLib.min(type(uint64).max, (feeMultiplierX64 * poolFee) >> 64));

            if (additionalFee != 0) {
                if (params.isExactOut()) {
                    // take an additional fee from the calculated input amount equal to the `additionalFee - poolFee`
                    if (balanceUpdate.delta0() > 0) {
                        uint128 inputAmount = uint128(uint256(int256(balanceUpdate.delta0())));
                        // first remove the fee to get the original input amount before we compute the additional fee
                        inputAmount -= computeFee(inputAmount, poolFee);
                        int128 fee = SafeCastLib.toInt128(amountBeforeFee(inputAmount, additionalFee) - inputAmount);

                        saveDelta0 += fee;
                        balanceUpdate = createPoolBalanceUpdate(balanceUpdate.delta0() + fee, balanceUpdate.delta1());
                    } else if (balanceUpdate.delta1() > 0) {
                        uint128 inputAmount = uint128(uint256(int256(balanceUpdate.delta1())));
                        // first remove the fee to get the original input amount before we compute the additional fee
                        inputAmount -= computeFee(inputAmount, poolFee);
                        int128 fee = SafeCastLib.toInt128(amountBeforeFee(inputAmount, additionalFee) - inputAmount);

                        saveDelta1 += fee;
                        balanceUpdate = createPoolBalanceUpdate(balanceUpdate.delta0(), balanceUpdate.delta1() + fee);
                    }
                } else {
                    if (balanceUpdate.delta0() < 0) {
                        uint128 outputAmount = uint128(uint256(-int256(balanceUpdate.delta0())));
                        int128 fee = SafeCastLib.toInt128(computeFee(outputAmount, additionalFee));

                        saveDelta0 += fee;
                        balanceUpdate = createPoolBalanceUpdate(balanceUpdate.delta0() + fee, balanceUpdate.delta1());
                    } else if (balanceUpdate.delta1() < 0) {
                        uint128 outputAmount = uint128(uint256(-int256(balanceUpdate.delta1())));
                        int128 fee = SafeCastLib.toInt128(computeFee(outputAmount, additionalFee));

                        saveDelta1 += fee;
                        balanceUpdate = createPoolBalanceUpdate(balanceUpdate.delta0(), balanceUpdate.delta1() + fee);
                    }
                }
            }

            if (saveDelta0 != 0 || saveDelta1 != 0) {
                CORE.updateSavedBalances(poolKey.token0, poolKey.token1, PoolId.unwrap(poolId), saveDelta0, saveDelta1);
            }

            result = abi.encode(balanceUpdate, stateAfter);
        }
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {IExposedStorage} from "../interfaces/IExposedStorage.sol";
import {StorageSlot} from "../types/storageSlot.sol";

/// @dev This library includes some helper functions for calling IExposedStorage#sload and IExposedStorage#tload.
library ExposedStorageLib {
    function sload(IExposedStorage target, bytes32 slot) internal view returns (bytes32 result) {
        assembly ("memory-safe") {
            mstore(0, shl(224, 0x380eb4e0))
            mstore(4, slot)

            if iszero(staticcall(gas(), target, 0, 36, 0, 32)) { revert(0, 0) }

            result := mload(0)
        }
    }

    function sload(IExposedStorage target, bytes32 slot0, bytes32 slot1)
        internal
        view
        returns (bytes32 result0, bytes32 result1)
    {
        assembly ("memory-safe") {
            let o := mload(0x40)
            mstore(o, shl(224, 0x380eb4e0))
            mstore(add(o, 4), slot0)
            mstore(add(o, 36), slot1)

            if iszero(staticcall(gas(), target, o, 68, o, 64)) { revert(0, 0) }

            result0 := mload(o)
            result1 := mload(add(o, 32))
        }
    }

    function sload(IExposedStorage target, bytes32 slot0, bytes32 slot1, bytes32 slot2)
        internal
        view
        returns (bytes32 result0, bytes32 result1, bytes32 result2)
    {
        assembly ("memory-safe") {
            let o := mload(0x40)
            mstore(o, shl(224, 0x380eb4e0))
            mstore(add(o, 4), slot0)
            mstore(add(o, 36), slot1)
            mstore(add(o, 68), slot2)

            if iszero(staticcall(gas(), target, o, 100, o, 96)) { revert(0, 0) }

            result0 := mload(o)
            result1 := mload(add(o, 32))
            result2 := mload(add(o, 64))
        }
    }

    function tload(IExposedStorage target, bytes32 slot) internal view returns (bytes32 result) {
        assembly ("memory-safe") {
            mstore(0, shl(224, 0xed832830))
            mstore(4, slot)

            if iszero(staticcall(gas(), target, 0, 36, 0, 32)) { revert(0, 0) }

            result := mload(0)
        }
    }

    // Overloads for StorageSlot type

    function sload(IExposedStorage target, StorageSlot slot) internal view returns (bytes32 result) {
        return sload(target, StorageSlot.unwrap(slot));
    }

    function sload(IExposedStorage target, StorageSlot slot0, StorageSlot slot1)
        internal
        view
        returns (bytes32 result0, bytes32 result1)
    {
        return sload(target, StorageSlot.unwrap(slot0), StorageSlot.unwrap(slot1));
    }

    function sload(IExposedStorage target, StorageSlot slot0, StorageSlot slot1, StorageSlot slot2)
        internal
        view
        returns (bytes32 result0, bytes32 result1, bytes32 result2)
    {
        return sload(target, StorageSlot.unwrap(slot0), StorageSlot.unwrap(slot1), StorageSlot.unwrap(slot2));
    }

    function tload(IExposedStorage target, StorageSlot slot) internal view returns (bytes32 result) {
        return tload(target, StorageSlot.unwrap(slot));
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {IForwardee, IFlashAccountant} from "../interfaces/IFlashAccountant.sol";
import {Locker} from "../types/locker.sol";

/// @title Base Forwardee
/// @notice Abstract base contract for contracts that need to receive forwarded calls from the flash accountant
/// @dev Provides the forwarding mechanism and delegates actual data handling to implementing contracts
abstract contract BaseForwardee is IForwardee {
    /// @notice Thrown when a function is called by an address other than the accountant
    error BaseForwardeeAccountantOnly();

    /// @notice The flash accountant contract that can forward calls to this contract
    IFlashAccountant private immutable ACCOUNTANT;

    /// @notice Constructs the BaseForwardee with a flash accountant
    /// @param _accountant The flash accountant contract that will forward calls
    constructor(IFlashAccountant _accountant) {
        ACCOUNTANT = _accountant;
    }

    /// CALLBACK HANDLERS

    /// @inheritdoc IForwardee
    /// @dev Extracts the forwarded data from calldata and delegates to handleForwardData
    /// The first 68 bytes of calldata contain the function selector (4 bytes), id (32 bytes), and originalLocker (32 bytes)
    /// All remaining calldata is treated as the forwarded data
    /// Return data from handleForwardData is returned exactly as is, with no additional encoding or decoding
    /// Reverts are also bubbled up
    function forwarded_2374103877(Locker original) external {
        if (msg.sender != address(ACCOUNTANT)) revert BaseForwardeeAccountantOnly();

        bytes memory data = msg.data[36:];

        bytes memory result = handleForwardData(original, data);

        assembly ("memory-safe") {
            // raw return whatever the handler sent
            return(add(result, 32), mload(result))
        }
    }

    /// INTERNAL FUNCTIONS

    /// @notice Handles the execution of forwarded data
    /// @dev Must be implemented by derived contracts to define forwarding behavior
    /// @param original The original locker that called forward
    /// @param data The forwarded data to process
    /// @return result The result of processing the forwarded data
    function handleForwardData(Locker original, bytes memory data) internal virtual returns (bytes memory result);
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {ICore, IExtension} from "../interfaces/ICore.sol";
import {CallPoints} from "../types/callPoints.sol";
import {PoolKey} from "../types/poolKey.sol";
import {PositionId} from "../types/positionId.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";
import {PoolState} from "../types/poolState.sol";
import {SwapParameters} from "../types/swapParameters.sol";
import {UsesCore} from "./UsesCore.sol";
import {Locker} from "../types/locker.sol";
import {PoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";

/// @title Base Extension
/// @notice Abstract base contract for creating extensions to the Ekubo Protocol
/// @dev Extensions can hook into various pool operations to add custom functionality
///      Derived contracts must implement getCallPoints() and the specific hook functions they want to use
abstract contract BaseExtension is IExtension, UsesCore {
    /// @notice Thrown when a call point is not implemented by the extension
    error CallPointNotImplemented();

    /// @notice Constructs the BaseExtension and optionally registers it with the core
    /// @param core The core contract instance
    constructor(ICore core) UsesCore(core) {
        if (_registerInConstructor()) core.registerExtension(getCallPoints());
    }

    /// @notice Determines whether the extension should register itself in the constructor
    /// @dev Can be overridden by derived contracts to control registration timing
    /// @return True if the extension should register in the constructor
    function _registerInConstructor() internal pure virtual returns (bool) {
        return true;
    }

    /// @notice Returns the call points configuration for this extension
    /// @dev Must be implemented by derived contracts to specify which hooks they use
    /// @return The call points configuration
    function getCallPoints() internal virtual returns (CallPoints memory);

    /// @inheritdoc IExtension
    function beforeInitializePool(address, PoolKey calldata, int32) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function afterInitializePool(address, PoolKey calldata, int32, SqrtRatio) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function beforeUpdatePosition(Locker, PoolKey memory, PositionId, int128) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function afterUpdatePosition(Locker, PoolKey memory, PositionId, int128, PoolBalanceUpdate, PoolState)
        external
        virtual
    {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function beforeSwap(Locker, PoolKey memory, SwapParameters) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function afterSwap(Locker, PoolKey memory, SwapParameters, PoolBalanceUpdate, PoolState) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function beforeCollectFees(Locker, PoolKey memory, PositionId) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function afterCollectFees(Locker, PoolKey memory, PositionId, uint128, uint128) external virtual {
        revert CallPointNotImplemented();
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {IExposedStorage} from "../interfaces/IExposedStorage.sol";

/// @title ExposedStorage
/// @notice Abstract contract that implements storage exposure functionality
/// @dev This contract provides the implementation for the IExposedStorage interface,
///      allowing inheriting contracts to expose their storage slots via view functions.
///      Uses inline assembly for efficient storage access.
abstract contract ExposedStorage is IExposedStorage {
    /// @inheritdoc IExposedStorage
    /// @dev Uses inline assembly to efficiently read multiple storage slots specified in calldata.
    ///      Each slot value is loaded using the SLOAD opcode and stored in memory.
    function sload() external view {
        assembly ("memory-safe") {
            for { let i := 4 } lt(i, calldatasize()) { i := add(i, 32) } { mstore(sub(i, 4), sload(calldataload(i))) }
            return(0, sub(calldatasize(), 4))
        }
    }

    /// @inheritdoc IExposedStorage
    /// @dev Uses inline assembly to efficiently read multiple transient storage slots specified in calldata.
    ///      Each slot value is loaded using the TLOAD opcode and stored in memory.
    function tload() external view {
        assembly ("memory-safe") {
            for { let i := 4 } lt(i, calldatasize()) { i := add(i, 32) } { mstore(sub(i, 4), tload(calldataload(i))) }
            return(0, sub(calldatasize(), 4))
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {ICore} from "../interfaces/ICore.sol";

/// @title Uses Core
/// @notice Abstract base contract for contracts that need to interact with the Ekubo Protocol core
/// @dev Provides core contract reference and access control functionality
abstract contract UsesCore {
    /// @notice Thrown when a function restricted to the core contract is called by another address
    error CoreOnly();

    /// @notice The core contract instance that this contract interacts with
    ICore internal immutable CORE;

    /// @notice Constructs the UsesCore contract with a core contract reference
    /// @param _core The core contract instance to use
    constructor(ICore _core) {
        CORE = _core;
    }

    /// @notice Restricts function access to only the core contract
    /// @dev Reverts with CoreOnly if called by any address other than the core contract
    modifier onlyCore() {
        if (msg.sender != address(CORE)) revert CoreOnly();
        _;
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {ICore} from "../interfaces/ICore.sol";
import {CoreStorageLayout} from "./CoreStorageLayout.sol";
import {ExposedStorageLib} from "./ExposedStorageLib.sol";
import {FeesPerLiquidity} from "../types/feesPerLiquidity.sol";
import {Position} from "../types/position.sol";
import {PoolState} from "../types/poolState.sol";
import {PositionId} from "../types/positionId.sol";
import {PoolId} from "../types/poolId.sol";
import {PoolKey} from "../types/poolKey.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";
import {SwapParameters, createSwapParameters} from "../types/swapParameters.sol";
import {PoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";
import {StorageSlot} from "../types/storageSlot.sol";

/// @title Core Library
/// @notice Library providing common storage getters for external contracts
/// @dev These functions access Core contract storage directly for gas efficiency
library CoreLib {
    using ExposedStorageLib for *;

    /// @notice Checks if an extension is registered with the core contract
    /// @param core The core contract instance
    /// @param extension The extension address to check
    /// @return registered True if the extension is registered
    function isExtensionRegistered(ICore core, address extension) internal view returns (bool registered) {
        registered = uint256(core.sload(CoreStorageLayout.isExtensionRegisteredSlot(extension))) != 0;
    }

    /// @notice Gets the current state of a pool
    /// @param core The core contract instance
    /// @param poolId The unique identifier for the pool
    /// @return state The current state of the pool
    function poolState(ICore core, PoolId poolId) internal view returns (PoolState state) {
        state = PoolState.wrap(core.sload(CoreStorageLayout.poolStateSlot(poolId)));
    }

    /// @notice Gets the current global fees per liquidity for a pool
    /// @param core The core contract instance
    /// @param poolId The unique identifier for the pool
    /// @return feesPerLiquidity The current global fees per liquidtiy of the pool
    function getPoolFeesPerLiquidity(ICore core, PoolId poolId)
        internal
        view
        returns (FeesPerLiquidity memory feesPerLiquidity)
    {
        StorageSlot fplFirstSlot = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId);
        (bytes32 value0, bytes32 value1) = core.sload(fplFirstSlot, fplFirstSlot.next());

        feesPerLiquidity.value0 = uint256(value0);
        feesPerLiquidity.value1 = uint256(value1);
    }

    /// @notice Gets position data for a specific position in a pool
    /// @param core The core contract instance
    /// @param poolId The unique identifier for the pool
    /// @param positionId The unique identifier for the position
    /// @return position The position data including liquidity, extraData, and fees
    function poolPositions(ICore core, PoolId poolId, address owner, PositionId positionId)
        internal
        view
        returns (Position memory position)
    {
        StorageSlot firstSlot = CoreStorageLayout.poolPositionsSlot(poolId, owner, positionId);
        (bytes32 v0, bytes32 v1, bytes32 v2) = core.sload(firstSlot, firstSlot.add(1), firstSlot.add(2));

        assembly ("memory-safe") {
            mstore(position, shl(128, v0))
            mstore(add(position, 0x20), shr(128, v0))
        }

        position.feesPerLiquidityInsideLast = FeesPerLiquidity(uint256(v1), uint256(v2));
    }

    /// @notice Gets saved balances for a specific owner and token pair
    /// @param core The core contract instance
    /// @param owner The owner of the saved balances
    /// @param token0 The first token address
    /// @param token1 The second token address
    /// @param salt The salt used for the saved balance key
    /// @return savedBalance0 The saved balance of token0
    /// @return savedBalance1 The saved balance of token1
    function savedBalances(ICore core, address owner, address token0, address token1, bytes32 salt)
        internal
        view
        returns (uint128 savedBalance0, uint128 savedBalance1)
    {
        uint256 value = uint256(core.sload(CoreStorageLayout.savedBalancesSlot(owner, token0, token1, salt)));

        savedBalance0 = uint128(value >> 128);
        savedBalance1 = uint128(value);
    }

    /// @notice Gets tick information for a specific tick in a pool
    /// @param core The core contract instance
    /// @param poolId The unique identifier for the pool
    /// @param tick The tick to query
    /// @return liquidityDelta The liquidity change when crossing this tick
    /// @return liquidityNet The net liquidity above this tick
    function poolTicks(ICore core, PoolId poolId, int32 tick)
        internal
        view
        returns (int128 liquidityDelta, uint128 liquidityNet)
    {
        bytes32 data = core.sload(CoreStorageLayout.poolTicksSlot(poolId, tick));

        // takes only least significant 128 bits
        liquidityDelta = int128(uint128(uint256(data)));
        // takes only most significant 128 bits
        liquidityNet = uint128(bytes16(data));
    }

    /// @notice Executes a swap against the core contract using assembly optimization
    /// @dev Uses assembly to make direct call to core contract for gas efficiency
    /// @param core The core contract instance
    /// @param value Native token value to send with the swap
    /// @param poolKey Pool key identifying the pool
    /// @param params The swap parameters to use
    /// @return balanceUpdate Change to the pool balances that resulted from the swap
    /// @return stateAfter The pool state after the swap
    function swap(ICore core, uint256 value, PoolKey memory poolKey, SwapParameters params)
        internal
        returns (PoolBalanceUpdate balanceUpdate, PoolState stateAfter)
    {
        assembly ("memory-safe") {
            let free := mload(0x40)

            // the function selector of swap is 0
            mstore(free, 0)

            // Copy PoolKey
            mcopy(add(free, 4), poolKey, 96)

            // Add SwapParameters
            mstore(add(free, 100), params)

            if iszero(call(gas(), core, value, free, 132, free, 64)) {
                returndatacopy(free, 0, returndatasize())
                revert(free, returndatasize())
            }

            // Extract return values - balanceUpdate is packed (delta1 << 128 | delta0)
            balanceUpdate := mload(free)
            stateAfter := mload(add(free, 32))
        }
    }
}

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

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {Locker} from "../types/locker.sol";

interface ILocker {
    function locked_6416899205(uint256 id) external;
}

interface IForwardee {
    function forwarded_2374103877(Locker original) external;
}

/// @title IFlashAccountant
/// @notice Interface for flash loan accounting functionality using transient storage
/// @dev This interface manages debt tracking for flash loans, allowing users to borrow tokens temporarily
///      and ensuring all debts are settled before the transaction completes. Uses transient storage
///      for gas-efficient temporary state management within a single transaction.
interface IFlashAccountant {
    /// @notice Thrown when an operation is made that affects debts without an active lock
    error NotLocked();
    /// @notice Thrown when a method is called by an address other than the current locker
    error LockerOnly();
    /// @notice Thrown when the lock callback returns without clearing all the debts either via withdrawing from or paying to the accountant
    error DebtsNotZeroed(uint256 id);
    /// @notice Thrown if the contract receives a payment that exceeds type(uint128).max
    error PaymentOverflow();
    /// @notice Thrown if the contract receives a call to updateDebt that has a data length != 20
    error UpdateDebtMessageLength();

    /// @notice Creates a lock context and calls back to the caller's locked function
    /// @dev The entrypoint for all operations on the core contract. Any data passed after the
    ///      function signature is passed through back to the caller after the locked function
    ///      signature and data, with no additional encoding. Any data returned from ILocker#locked
    ///      is also returned from this function exactly as is. Reverts are bubbled up.
    ///      Ensures all debts are zeroed before completing the lock.
    function lock() external;

    /// @notice Forwards the lock context to another actor, allowing them to act on the original locker's debt
    /// @dev Temporarily changes the locker to the forwarded address for the duration of the forwarded call.
    ///      Any additional calldata is passed through to the forwardee with no additional encoding.
    ///      Any data returned from IForwardee#forwarded is returned exactly as is. Reverts are bubbled up.
    /// @param to The address to forward the lock context to
    function forward(address to) external;

    /// @notice Initiates a payment operation by recording current token balances
    /// @dev To make a payment to core, you must first call startPayments with all the tokens you'd like to send.
    ///      All the tokens that will be paid must be ABI-encoded immediately after the 4 byte function selector.
    ///      This function stores the current balance + 1 for each token to distinguish between zero balance
    ///      and uninitialized state. Returns the current balances of all specified tokens as ABI-encoded
    ///      raw bytes via assembly (no explicit Solidity return type).
    function startPayments() external;

    /// @notice Completes a payment operation by calculating and crediting token payments
    /// @dev After tokens have been transferred, call completePayments to be credited for the tokens
    ///      that have been paid to core. The credit goes to the current locker. Compares current
    ///      balances with those recorded in startPayments to determine payment amounts.
    ///      The computed payments are applied to the current locker's debt.
    ///      Returns packed uint128 payment amounts (16 bytes each) in the same order as the tokens.
    function completePayments() external;

    /// @notice Withdraws tokens from the accountant to recipients using packed calldata
    /// @dev The contract must be locked, as it tracks withdrawn amounts against the current locker's debt.
    ///      Calldata format: each withdrawal is 56 bytes: token (20) + recipient (20) + amount (16)
    ///      For native tokens, uses the NATIVE_TOKEN_ADDRESS constant and transfers ETH directly.
    function withdraw() external;

    /// @notice Updates debt for the current locker and for the token at the calling address
    /// @dev This is for deeply-integrated tokens that allow flash operations via the accountant.
    ///      The calling address is treated as the token address.
    /// @dev The debt change argument is an int128 encoded immediately after the selector.
    /// @dev The calldata length must be exactly 20 bytes in order to avoid this being called unintentionally.
    function updateDebt() external;

    /// @notice Receives ETH payments and credits them against the current locker's native token debt
    /// @dev This contract can receive ETH as a payment. The received amount is credited as a negative
    ///      debt change for the native token. Note: because we use msg.value here, this contract can
    ///      never be multicallable, i.e. it should never expose the ability to delegatecall itself
    ///      more than once in a single call.
    receive() external payable;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @title IExposedStorage
/// @notice Interface for exposing contract storage via view methods
/// @dev This interface provides a way to access specific pieces of state in the inheriting contract.
///      It serves as a workaround in the absence of EIP-2330 (https://eips.ethereum.org/EIPS/eip-2330)
///      which would provide native support for exposing contract storage.
interface IExposedStorage {
    /// @notice Loads storage slots from the contract's persistent storage
    /// @dev Reads each 32-byte slot specified in the calldata (after the function selector) from storage
    ///      and returns all the loaded values concatenated together.
    function sload() external view;

    /// @notice Loads storage slots from the contract's transient storage
    /// @dev Reads each 32-byte slot specified in the calldata (after the function selector) from transient storage
    ///      and returns all the loaded values concatenated together. Transient storage is cleared at the end
    ///      of each transaction.
    function tload() external view;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolKey} from "../../types/poolKey.sol";
import {ILocker, IForwardee} from "../IFlashAccountant.sol";
import {IExtension} from "../ICore.sol";
import {IExposedStorage} from "../IExposedStorage.sol";

/// @title MEV Capture Interface
/// @notice Interface for the Ekubo MEV Capture Extension
/// @dev Extension that charges additional fees based on the relative size of the priority fee and tick movement during swaps
interface IMEVCapture is IExposedStorage, ILocker, IForwardee, IExtension {
    /// @notice Thrown when trying to use MEV capture on a full-range-only pool
    /// @dev MEV capture only works with concentrated liquidity pools that have discrete tick spacing
    error ConcentratedLiquidityPoolsOnly();

    /// @notice Thrown when trying to use MEV capture on a pool with zero fees
    /// @dev MEV capture multiplies the base fee, so a zero fee would result in no additional fees
    error NonzeroFeesOnly();

    /// @notice Thrown when attempting to swap directly without using the forward mechanism
    /// @dev All swaps must go through the forward mechanism to ensure proper MEV fee calculation
    error SwapMustHappenThroughForward();

    /// @notice Accumulates any pending pool fees from past blocks
    /// @dev This function can be called by anyone to trigger fee accumulation for a pool
    /// @dev Fees are accumulated when the pool hasn't been updated in the current block
    /// @param poolKey The pool key identifying the pool to accumulate fees for
    function accumulatePoolFees(PoolKey memory poolKey) external;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {CallPoints} from "../types/callPoints.sol";
import {PoolKey} from "../types/poolKey.sol";
import {PositionId} from "../types/positionId.sol";
import {FeesPerLiquidity} from "../types/feesPerLiquidity.sol";
import {IExposedStorage} from "../interfaces/IExposedStorage.sol";
import {IFlashAccountant} from "../interfaces/IFlashAccountant.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";
import {PoolState} from "../types/poolState.sol";
import {SwapParameters} from "../types/swapParameters.sol";
import {PoolId} from "../types/poolId.sol";
import {Locker} from "../types/locker.sol";
import {PoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";

/// @title Extension Interface
/// @notice Interface for pool extensions that can hook into core operations
/// @dev Extensions must register with the core contract and implement these hooks
interface IExtension {
    /// @notice Called before a pool is initialized
    /// @param caller Address that initiated the pool initialization
    /// @param key Pool key identifying the pool
    /// @param tick Initial tick for the pool
    function beforeInitializePool(address caller, PoolKey calldata key, int32 tick) external;

    /// @notice Called after a pool is initialized
    /// @param caller Address that initiated the pool initialization
    /// @param key Pool key identifying the pool
    /// @param tick Initial tick for the pool
    /// @param sqrtRatio Initial sqrt price ratio for the pool
    function afterInitializePool(address caller, PoolKey calldata key, int32 tick, SqrtRatio sqrtRatio) external;

    /// @notice Called before a position is updated
    /// @param locker The current holder of the lock performing the position update
    /// @param poolKey Pool key identifying the pool
    /// @param positionId The key of the position that is being updated
    /// @param liquidityDelta The change in liquidity that is being requested for the position
    function beforeUpdatePosition(Locker locker, PoolKey memory poolKey, PositionId positionId, int128 liquidityDelta)
        external;

    /// @notice Called after a position is updated
    /// @param locker The current holder of the lock performing the position update
    /// @param poolKey Pool key identifying the pool
    /// @param positionId The key of the position that was updated
    /// @param liquidityDelta Change in liquidity of the specified position key range
    /// @param balanceUpdate Change in token balances of the pool (delta0 and delta1)
    function afterUpdatePosition(
        Locker locker,
        PoolKey memory poolKey,
        PositionId positionId,
        int128 liquidityDelta,
        PoolBalanceUpdate balanceUpdate,
        PoolState stateAfter
    ) external;

    /// @notice Called before a swap is executed
    /// @param locker The current holder of the lock performing the swap
    /// @param poolKey Pool key identifying the pool
    /// @param params Swap parameters containing amount, isToken1, sqrtRatioLimit, and skipAhead
    function beforeSwap(Locker locker, PoolKey memory poolKey, SwapParameters params) external;

    /// @notice Called after a swap is executed
    /// @param locker The current holder of the lock performing the swap
    /// @param poolKey Pool key identifying the pool
    /// @param params Swap parameters containing amount, isToken1, sqrtRatioLimit, and skipAhead
    /// @param balanceUpdate Change in token balances (delta0 and delta1)
    /// @param stateAfter The pool state after the swap
    function afterSwap(
        Locker locker,
        PoolKey memory poolKey,
        SwapParameters params,
        PoolBalanceUpdate balanceUpdate,
        PoolState stateAfter
    ) external;

    /// @notice Called before fees are collected from a position
    /// @param locker The current holder of the lock collecting fees
    /// @param poolKey Pool key identifying the pool
    /// @param positionId The key of the position for which fees will be collected
    function beforeCollectFees(Locker locker, PoolKey memory poolKey, PositionId positionId) external;

    /// @notice Called after fees are collected from a position
    /// @param locker The current holder of the lock collecting fees
    /// @param poolKey Pool key identifying the pool
    /// @param positionId The key of the position for which fees were collected
    /// @param amount0 Amount of token0 fees collected
    /// @param amount1 Amount of token1 fees collected
    function afterCollectFees(
        Locker locker,
        PoolKey memory poolKey,
        PositionId positionId,
        uint128 amount0,
        uint128 amount1
    ) external;
}

/// @title Core Interface
/// @notice Main interface for the Ekubo Protocol core contract
/// @dev Inherits from IFlashAccountant and IExposedStorage for additional functionality
interface ICore is IFlashAccountant, IExposedStorage {
    /// @notice Emitted when an extension is registered
    /// @param extension Address of the registered extension
    event ExtensionRegistered(address extension);

    /// @notice Emitted when a pool is initialized
    /// @param poolId Unique identifier for the pool
    /// @param poolKey Pool key containing token addresses and configuration
    /// @param tick Initial tick for the pool
    /// @param sqrtRatio Initial sqrt price ratio for the pool
    event PoolInitialized(PoolId poolId, PoolKey poolKey, int32 tick, SqrtRatio sqrtRatio);

    /// @notice Emitted when a position is updated
    /// @param locker The locker that is updating the position
    /// @param poolId Unique identifier for the pool
    /// @param positionId Identifier of the position specifying a salt and the bounds
    /// @param liquidityDelta The change in liquidity for the specified pool and position keys
    /// @param balanceUpdate Change in token balances (delta0 and delta1)
    event PositionUpdated(
        address locker,
        PoolId poolId,
        PositionId positionId,
        int128 liquidityDelta,
        PoolBalanceUpdate balanceUpdate,
        PoolState stateAfter
    );

    /// @notice Emitted when fees are collected from a position
    /// @param locker The locker that is collecting fees
    /// @param poolId Unique identifier for the pool
    /// @param positionId Identifier of the position specifying a salt and the bounds
    /// @param amount0 Amount of token0 fees collected
    /// @param amount1 Amount of token1 fees collected
    event PositionFeesCollected(address locker, PoolId poolId, PositionId positionId, uint128 amount0, uint128 amount1);

    /// @notice Emitted when fees are accumulated to a pool
    /// @param poolId Unique identifier for the pool
    /// @param amount0 Amount of token0 fees accumulated
    /// @param amount1 Amount of token1 fees accumulated
    /// @dev Note locker is ommitted because it's always the extension of the pool associated with poolId
    event FeesAccumulated(PoolId poolId, uint128 amount0, uint128 amount1);

    /// @notice Thrown when extension registration fails due to invalid call points
    error FailedRegisterInvalidCallPoints();

    /// @notice Thrown when trying to register an already registered extension
    error ExtensionAlreadyRegistered();

    /// @notice Thrown when saved balance operations would cause overflow
    error SavedBalanceOverflow();

    /// @notice Thrown when trying to initialize an already initialized pool
    error PoolAlreadyInitialized();

    /// @notice Thrown when trying to use an unregistered extension
    error ExtensionNotRegistered();

    /// @notice Thrown when trying to operate on an uninitialized pool
    error PoolNotInitialized();

    /// @notice Thrown when sqrt ratio limit is out of valid range
    error SqrtRatioLimitOutOfRange();

    /// @notice Thrown when sqrt ratio limit parameter given to swap is not a valid sqrt ratio
    error InvalidSqrtRatioLimit();

    /// @notice Thrown when the sqrt ratio limit is in the wrong direction of the current price
    error SqrtRatioLimitWrongDirection();

    /// @notice Thrown when saved balance tokens are not properly sorted
    error SavedBalanceTokensNotSorted();

    /// @notice Thrown when a position update would cause liquidityNet on a tick to exceed the maximum allowed
    /// @param tick The tick that would exceed the limit
    /// @param liquidityNet The resulting liquidityNet that exceeds the limit
    /// @param maxLiquidityPerTick The maximum allowed liquidity per tick
    error MaxLiquidityPerTickExceeded(int32 tick, uint128 liquidityNet, uint128 maxLiquidityPerTick);

    /// @notice Registers an extension with the core contract
    /// @dev Extensions must call this function to become registered. The call points are validated against the caller address
    /// @param expectedCallPoints Call points configuration for the extension
    function registerExtension(CallPoints memory expectedCallPoints) external;

    /// @notice Initializes a new pool with the given tick
    /// @dev Sets the initial price for a new pool in terms of tick
    /// @param poolKey Pool key identifying the pool to initialize
    /// @param tick Initial tick for the pool
    /// @return sqrtRatio Initial sqrt price ratio for the pool
    function initializePool(PoolKey memory poolKey, int32 tick) external returns (SqrtRatio sqrtRatio);

    /// @notice Finds the previous initialized tick
    /// @param poolId Unique identifier for the pool
    /// @param fromTick Starting tick to search from
    /// @param tickSpacing Tick spacing for the pool
    /// @param skipAhead Number of ticks to skip for gas optimization
    /// @return tick The previous initialized tick
    /// @return isInitialized Whether the tick is initialized
    function prevInitializedTick(PoolId poolId, int32 fromTick, uint32 tickSpacing, uint256 skipAhead)
        external
        view
        returns (int32 tick, bool isInitialized);

    /// @notice Finds the next initialized tick
    /// @param poolId Unique identifier for the pool
    /// @param fromTick Starting tick to search from
    /// @param tickSpacing Tick spacing for the pool
    /// @param skipAhead Number of ticks to skip for gas optimization
    /// @return tick The next initialized tick
    /// @return isInitialized Whether the tick is initialized
    function nextInitializedTick(PoolId poolId, int32 fromTick, uint32 tickSpacing, uint256 skipAhead)
        external
        view
        returns (int32 tick, bool isInitialized);

    /// @notice Updates saved balances for later use
    /// @dev The saved balances are stored in a single slot. The resulting saved balance must fit within a uint128 container
    /// @param token0 Address of the first token (must be < token1)
    /// @param token1 Address of the second token (must be > token0)
    /// @param salt Unique identifier for the saved balance
    /// @param delta0 Change in token0 balance (positive for saving, negative for loading)
    /// @param delta1 Change in token1 balance (positive for saving, negative for loading)
    function updateSavedBalances(address token0, address token1, bytes32 salt, int256 delta0, int256 delta1)
        external
        payable;

    /// @notice Returns the accumulated fees per liquidity inside the given bounds
    /// @dev The reason this getter is exposed is that it requires conditional SLOADs for maximum efficiency
    /// @param poolId The ID of the pool to fetch the fees per liquidity inside
    /// @param tickLower Lower bound of the price range to get the snapshot
    /// @param tickLower Upper bound of the price range to get the snapshot
    /// @return feesPerLiquidity Accumulated fees per liquidity inside the bounds
    function getPoolFeesPerLiquidityInside(PoolId poolId, int32 tickLower, int32 tickUpper)
        external
        view
        returns (FeesPerLiquidity memory feesPerLiquidity);

    /// @notice Accumulates tokens as fees for a pool
    /// @dev Only callable by the extension of the specified pool key. The current locker must be the extension.
    /// The extension must call this function within a lock callback
    /// @param poolKey Pool key identifying the pool
    /// @param amount0 Amount of token0 to accumulate as fees
    /// @param amount1 Amount of token1 to accumulate as fees
    function accumulateAsFees(PoolKey memory poolKey, uint128 amount0, uint128 amount1) external payable;

    /// @notice Updates a liquidity position and sets extra data
    /// @param poolKey Pool key identifying the pool
    /// @param positionId The key of the position to update
    /// @param liquidityDelta The change in liquidity
    /// @return balanceUpdate Change in token balances (delta0 and delta1)
    function updatePosition(PoolKey memory poolKey, PositionId positionId, int128 liquidityDelta)
        external
        payable
        returns (PoolBalanceUpdate balanceUpdate);

    /// @notice Sets extra data for the position
    /// @param poolId ID of the pool for which the position exists
    /// @param positionId The key of the position to set extra data at
    /// @param extraData The data to set on the position
    function setExtraData(PoolId poolId, PositionId positionId, bytes16 extraData) external;

    /// @notice Collects accumulated fees from a position
    /// @param poolKey Pool key identifying the pool
    /// @param positionId The key of the position for which to collect fees
    /// @return amount0 Amount of token0 fees collected
    /// @return amount1 Amount of token1 fees collected
    function collectFees(PoolKey memory poolKey, PositionId positionId)
        external
        returns (uint128 amount0, uint128 amount1);

    /// @notice Executes a swap against a pool
    /// @dev Function name is mined to have a zero function selector for gas efficiency
    function swap_6269342730() external payable;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

struct CallPoints {
    bool beforeInitializePool;
    bool afterInitializePool;
    bool beforeSwap;
    bool afterSwap;
    bool beforeUpdatePosition;
    bool afterUpdatePosition;
    bool beforeCollectFees;
    bool afterCollectFees;
}

using {eq, isValid, toUint8} for CallPoints global;

function eq(CallPoints memory a, CallPoints memory b) pure returns (bool) {
    return (a.beforeInitializePool == b.beforeInitializePool && a.afterInitializePool == b.afterInitializePool
            && a.beforeSwap == b.beforeSwap && a.afterSwap == b.afterSwap
            && a.beforeUpdatePosition == b.beforeUpdatePosition && a.afterUpdatePosition == b.afterUpdatePosition
            && a.beforeCollectFees == b.beforeCollectFees && a.afterCollectFees == b.afterCollectFees);
}

function isValid(CallPoints memory a) pure returns (bool) {
    return (a.beforeInitializePool || a.afterInitializePool || a.beforeSwap || a.afterSwap || a.beforeUpdatePosition
            || a.afterUpdatePosition || a.beforeCollectFees || a.afterCollectFees);
}

function toUint8(CallPoints memory callPoints) pure returns (uint8 b) {
    assembly ("memory-safe") {
        b := add(
            add(
                add(
                    add(
                        add(
                            add(
                                add(mload(callPoints), mul(128, mload(add(callPoints, 32)))),
                                mul(64, mload(add(callPoints, 64)))
                            ),
                            mul(32, mload(add(callPoints, 96)))
                        ),
                        mul(16, mload(add(callPoints, 128)))
                    ),
                    mul(8, mload(add(callPoints, 160)))
                ),
                mul(4, mload(add(callPoints, 192)))
            ),
            mul(2, mload(add(callPoints, 224)))
        )
    }
}

function addressToCallPoints(address a) pure returns (CallPoints memory result) {
    result = byteToCallPoints(uint8(uint160(a) >> 152));
}

function byteToCallPoints(uint8 b) pure returns (CallPoints memory result) {
    // note the order of bytes does not match the struct order of elements because we are matching the cairo implementation
    // which for legacy reasons has the fields in this order
    result = CallPoints({
        beforeInitializePool: (b & 1) != 0,
        afterInitializePool: (b & 128) != 0,
        beforeSwap: (b & 64) != 0,
        afterSwap: (b & 32) != 0,
        beforeUpdatePosition: (b & 16) != 0,
        afterUpdatePosition: (b & 8) != 0,
        beforeCollectFees: (b & 4) != 0,
        afterCollectFees: (b & 2) != 0
    });
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a pool
/// @dev Wraps bytes32 to provide type safety for pool identifiers
type PoolId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a pool
/// @dev Wraps bytes32 to provide type safety for pool identifiers
type PoolId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type Locker is bytes32;

using {id, addr, parse} for Locker global;

function id(Locker locker) pure returns (uint256 v) {
    assembly ("memory-safe") {
        v := sub(shr(160, locker), 1)
    }
}

function addr(Locker locker) pure returns (address v) {
    assembly ("memory-safe") {
        v := shr(96, shl(96, locker))
    }
}

function parse(Locker locker) pure returns (uint256 lockerId, address lockerAddr) {
    assembly ("memory-safe") {
        lockerId := sub(shr(160, locker), 1)
        lockerAddr := shr(96, shl(96, locker))
    }
}

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

// The total fees per liquidity for each token.
// Since these are always read together we put them in a struct, even though they cannot be packed.
struct FeesPerLiquidity {
    uint256 value0;
    uint256 value1;
}

using {sub} for FeesPerLiquidity global;

function sub(FeesPerLiquidity memory a, FeesPerLiquidity memory b) pure returns (FeesPerLiquidity memory result) {
    assembly ("memory-safe") {
        mstore(result, sub(mload(a), mload(b)))
        mstore(add(result, 32), sub(mload(add(a, 32)), mload(add(b, 32))))
    }
}

function feesPerLiquidityFromAmounts(uint128 amount0, uint128 amount1, uint128 liquidity)
    pure
    returns (FeesPerLiquidity memory result)
{
    assembly ("memory-safe") {
        mstore(result, div(shl(128, amount0), liquidity))
        mstore(add(result, 32), div(shl(128, amount1), liquidity))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type PoolBalanceUpdate is bytes32;

using {delta0, delta1} for PoolBalanceUpdate global;

function delta0(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, shr(128, update))
    }
}

function delta1(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, update)
    }
}

function createPoolBalanceUpdate(int128 _delta0, int128 _delta1) pure returns (PoolBalanceUpdate update) {
    assembly ("memory-safe") {
        // update = (delta0 << 128) | delta1
        update := or(shl(128, _delta0), and(_delta1, 0xffffffffffffffffffffffffffffffff))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a pool
/// @dev Wraps bytes32 to provide type safety for pool identifiers
type PoolId is bytes32;

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

// Returns the fee to charge based on the amount, which is the fee (a 0.64 number) times the
// amount, rounded up
function computeFee(uint128 amount, uint64 fee) pure returns (uint128 result) {
    assembly ("memory-safe") {
        result := shr(64, add(mul(amount, fee), 0xffffffffffffffff))
    }
}

error AmountBeforeFeeOverflow();

// Returns the amount before the fee is applied, which is the amount minus the fee, rounded up
function amountBeforeFee(uint128 afterFee, uint64 fee) pure returns (uint128 result) {
    assembly ("memory-safe") {
        let v := shl(64, afterFee)
        let d := sub(0x10000000000000000, fee)
        result := add(iszero(iszero(mod(v, d))), div(v, d))
        if shr(128, result) {
            mstore(0, 0x0d88f526)
            revert(0x1c, 0x04)
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio} from "./sqrtRatio.sol";

type PoolState is bytes32;

using {sqrtRatio, tick, liquidity, isInitialized, parse} for PoolState global;

function sqrtRatio(PoolState state) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, state)
    }
}

function tick(PoolState state) pure returns (int32 t) {
    assembly ("memory-safe") {
        t := signextend(3, shr(128, state))
    }
}

function liquidity(PoolState state) pure returns (uint128 l) {
    assembly ("memory-safe") {
        l := shr(128, shl(128, state))
    }
}

function isInitialized(PoolState state) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := iszero(iszero(state))
    }
}

function parse(PoolState state) pure returns (SqrtRatio r, int32 t, uint128 l) {
    assembly ("memory-safe") {
        r := shr(160, state)
        t := signextend(3, shr(128, state))
        l := shr(128, shl(128, state))
    }
}

function createPoolState(SqrtRatio _sqrtRatio, int32 _tick, uint128 _liquidity) pure returns (PoolState s) {
    assembly ("memory-safe") {
        // s = (sqrtRatio << 160) | (_tick << 128) | liquidity
        s := or(shl(160, _sqrtRatio), or(shl(128, and(_tick, 0xFFFFFFFF)), shr(128, shl(128, _liquidity))))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type Locker is bytes32;

using {id, addr, parse} for Locker global;

function id(Locker locker) pure returns (uint256 v) {
    assembly ("memory-safe") {
        v := sub(shr(160, locker), 1)
    }
}

function addr(Locker locker) pure returns (address v) {
    assembly ("memory-safe") {
        v := shr(96, shl(96, locker))
    }
}

function parse(Locker locker) pure returns (uint256 lockerId, address lockerAddr) {
    assembly ("memory-safe") {
        lockerId := sub(shr(160, locker), 1)
        lockerAddr := shr(96, shl(96, locker))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio, MIN_SQRT_RATIO_RAW, MAX_SQRT_RATIO_RAW} from "./sqrtRatio.sol";

type SwapParameters is bytes32;

using {
    sqrtRatioLimit,
    amount,
    isToken1,
    skipAhead,
    isExactOut,
    isPriceIncreasing,
    withDefaultSqrtRatioLimit
} for SwapParameters global;

function sqrtRatioLimit(SwapParameters params) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, params)
    }
}

function amount(SwapParameters params) pure returns (int128 a) {
    assembly ("memory-safe") {
        a := signextend(15, shr(32, params))
    }
}

function isToken1(SwapParameters params) pure returns (bool t) {
    assembly ("memory-safe") {
        t := and(shr(31, params), 1)
    }
}

function skipAhead(SwapParameters params) pure returns (uint256 s) {
    assembly ("memory-safe") {
        s := and(params, 0x7fffffff)
    }
}

function createSwapParameters(SqrtRatio _sqrtRatioLimit, int128 _amount, bool _isToken1, uint256 _skipAhead)
    pure
    returns (SwapParameters p)
{
    assembly ("memory-safe") {
        // p = (sqrtRatioLimit << 160) | (amount << 32) | (isToken1 << 31) | skipAhead
        // Mask each field to ensure dirty bits don't interfere
        // For isToken1, use iszero(iszero()) to convert any non-zero value to 1
        p := or(
            shl(160, _sqrtRatioLimit),
            or(
                shl(32, and(_amount, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)),
                or(shl(31, iszero(iszero(_isToken1))), and(_skipAhead, 0x7fffffff))
            )
        )
    }
}

function isExactOut(SwapParameters params) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := and(shr(159, params), 1)
    }
}

function isPriceIncreasing(SwapParameters params) pure returns (bool yes) {
    bool _isExactOut = params.isExactOut();
    bool _isToken1 = params.isToken1();
    assembly ("memory-safe") {
        yes := xor(_isExactOut, _isToken1)
    }
}

function withDefaultSqrtRatioLimit(SwapParameters params) pure returns (SwapParameters updated) {
    bool increasing = params.isPriceIncreasing();
    assembly ("memory-safe") {
        let replace := iszero(shr(160, params))
        let orMask :=
            shl(160, mul(replace, or(mul(increasing, MAX_SQRT_RATIO_RAW), mul(iszero(increasing), MIN_SQRT_RATIO_RAW))))
        updated := or(orMask, params)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolId} from "./poolId.sol";
import {PoolConfig} from "./poolConfig.sol";

using {toPoolId, validate} for PoolKey global;

/// @notice Unique identifier for a pool containing token addresses and configuration
/// @dev Each pool has its own state associated with this key
struct PoolKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing extension, fee, and tick spacing
    PoolConfig config;
}

/// @notice Thrown when tokens are not properly sorted (token0 >= token1)
error TokensMustBeSorted();

/// @notice Validates that a pool key is valid
/// @dev Checks that tokens are sorted and the config is valid
/// @param key The pool key to validate
function validate(PoolKey memory key) pure {
    if (key.token0 >= key.token1) revert TokensMustBeSorted();
    key.config.validate();
}

/// @notice Converts a pool key to a unique pool ID
/// @param key The pool key
/// @return result The unique pool ID (hash of the pool key)
function toPoolId(PoolKey memory key) pure returns (PoolId result) {
    assembly ("memory-safe") {
        // it's already copied into memory
        result := keccak256(key, 96)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type MEVCapturePoolState is bytes32;

using {lastUpdateTime, tickLast} for MEVCapturePoolState global;

function lastUpdateTime(MEVCapturePoolState state) pure returns (uint32 v) {
    assembly ("memory-safe") {
        v := shr(224, state)
    }
}

function tickLast(MEVCapturePoolState state) pure returns (int32 v) {
    assembly ("memory-safe") {
        v := signextend(3, state)
    }
}

function createMEVCapturePoolState(uint32 _lastUpdateTime, int32 _tickLast) pure returns (MEVCapturePoolState s) {
    assembly ("memory-safe") {
        // s = (lastUpdateTime << 224) | tickLast
        s := or(shl(224, _lastUpdateTime), and(_tickLast, 0xffffffff))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio, MIN_SQRT_RATIO_RAW, MAX_SQRT_RATIO_RAW} from "./sqrtRatio.sol";

type SwapParameters is bytes32;

using {
    sqrtRatioLimit,
    amount,
    isToken1,
    skipAhead,
    isExactOut,
    isPriceIncreasing,
    withDefaultSqrtRatioLimit
} for SwapParameters global;

function sqrtRatioLimit(SwapParameters params) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, params)
    }
}

function amount(SwapParameters params) pure returns (int128 a) {
    assembly ("memory-safe") {
        a := signextend(15, shr(32, params))
    }
}

function isToken1(SwapParameters params) pure returns (bool t) {
    assembly ("memory-safe") {
        t := and(shr(31, params), 1)
    }
}

function skipAhead(SwapParameters params) pure returns (uint256 s) {
    assembly ("memory-safe") {
        s := and(params, 0x7fffffff)
    }
}

function createSwapParameters(SqrtRatio _sqrtRatioLimit, int128 _amount, bool _isToken1, uint256 _skipAhead)
    pure
    returns (SwapParameters p)
{
    assembly ("memory-safe") {
        // p = (sqrtRatioLimit << 160) | (amount << 32) | (isToken1 << 31) | skipAhead
        // Mask each field to ensure dirty bits don't interfere
        // For isToken1, use iszero(iszero()) to convert any non-zero value to 1
        p := or(
            shl(160, _sqrtRatioLimit),
            or(
                shl(32, and(_amount, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)),
                or(shl(31, iszero(iszero(_isToken1))), and(_skipAhead, 0x7fffffff))
            )
        )
    }
}

function isExactOut(SwapParameters params) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := and(shr(159, params), 1)
    }
}

function isPriceIncreasing(SwapParameters params) pure returns (bool yes) {
    bool _isExactOut = params.isExactOut();
    bool _isToken1 = params.isToken1();
    assembly ("memory-safe") {
        yes := xor(_isExactOut, _isToken1)
    }
}

function withDefaultSqrtRatioLimit(SwapParameters params) pure returns (SwapParameters updated) {
    bool increasing = params.isPriceIncreasing();
    assembly ("memory-safe") {
        let replace := iszero(shr(160, params))
        let orMask :=
            shl(160, mul(replace, or(mul(increasing, MAX_SQRT_RATIO_RAW), mul(iszero(increasing), MIN_SQRT_RATIO_RAW))))
        updated := or(orMask, params)
    }
}

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

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type PoolBalanceUpdate is bytes32;

using {delta0, delta1} for PoolBalanceUpdate global;

function delta0(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, shr(128, update))
    }
}

function delta1(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, update)
    }
}

function createPoolBalanceUpdate(int128 _delta0, int128 _delta1) pure returns (PoolBalanceUpdate update) {
    assembly ("memory-safe") {
        // update = (delta0 << 128) | delta1
        update := or(shl(128, _delta0), and(_delta1, 0xffffffffffffffffffffffffffffffff))
    }
}

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

// A dynamic fixed point number (a la floating point) that stores a shifting 94 bit view of the underlying fixed point value,
//  based on the most significant bits (mantissa)
// If the most significant 2 bits are 11, it represents a 64.30
// If the most significant 2 bits are 10, it represents a 32.62 number
// If the most significant 2 bits are 01, it represents a 0.94 number
// If the most significant 2 bits are 00, it represents a 0.126 number that is always less than 2**-32

type SqrtRatio is uint96;

uint96 constant MIN_SQRT_RATIO_RAW = 4611797791050542631;
SqrtRatio constant MIN_SQRT_RATIO = SqrtRatio.wrap(MIN_SQRT_RATIO_RAW);
uint96 constant MAX_SQRT_RATIO_RAW = 79227682466138141934206691491;
SqrtRatio constant MAX_SQRT_RATIO = SqrtRatio.wrap(MAX_SQRT_RATIO_RAW);

uint96 constant TWO_POW_95 = 0x800000000000000000000000;
uint96 constant TWO_POW_94 = 0x400000000000000000000000;
uint96 constant TWO_POW_62 = 0x4000000000000000;
uint96 constant TWO_POW_62_MINUS_ONE = 0x3fffffffffffffff;
uint96 constant BIT_MASK = 0xc00000000000000000000000; // TWO_POW_95 | TWO_POW_94

SqrtRatio constant ONE = SqrtRatio.wrap((TWO_POW_95) + (1 << 62));

using {
    toFixed,
    isValid,
    ge as >=,
    le as <=,
    lt as <,
    gt as >,
    eq as ==,
    neq as !=,
    isZero,
    min,
    max
} for SqrtRatio global;

function isValid(SqrtRatio sqrtRatio) pure returns (bool r) {
    assembly ("memory-safe") {
        r := and(
            // greater than or equal to TWO_POW_62, i.e. the whole number portion is nonzero
            gt(and(sqrtRatio, not(BIT_MASK)), TWO_POW_62_MINUS_ONE),
            // and between min/max sqrt ratio
            and(iszero(lt(sqrtRatio, MIN_SQRT_RATIO_RAW)), iszero(gt(sqrtRatio, MAX_SQRT_RATIO_RAW)))
        )
    }
}

error ValueOverflowsSqrtRatioContainer();

// If passing a value greater than this constant with roundUp = true, toSqrtRatio will overflow
// For roundUp = false, the constant is type(uint192).max
uint256 constant MAX_FIXED_VALUE_ROUND_UP =
    0x1000000000000000000000000000000000000000000000000 - 0x4000000000000000000000000;

// Converts a 64.128 value into the compact SqrtRatio representation
function toSqrtRatio(uint256 sqrtRatio, bool roundUp) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        function compute(sr, ru) -> v {
            // rup = 0x00...00 when false, 0xff...ff when true
            let rup := sub(0, ru)

            // Region: < 2**96  (shift = 2)
            let addmask := and(0x3, rup) // (1<<s)-1 if ru
            if lt(add(sr, addmask), shl(96, 1)) {
                v := shr(2, add(sr, addmask))
                leave
            }

            // Region: < 2**128 (shift = 34)  + set bit 94
            addmask := and(0x3ffffffff, rup)
            if lt(add(sr, addmask), shl(128, 1)) {
                v := or(shl(94, 1), shr(34, add(sr, addmask)))
                leave
            }

            // Region: < 2**160 (shift = 66)  + set bit 95
            addmask := and(0x3ffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(160, 1)) {
                v := or(shl(95, 1), shr(66, add(sr, addmask)))
                leave
            }

            // Region: < 2**192 (shift = 98)  + set bits 95|94
            addmask := and(0x3ffffffffffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(192, 1)) {
                v := or(shl(94, 3), shr(98, add(sr, addmask))) // 3<<94 == bit95|bit94
                leave
            }

            // cast sig "ValueOverflowsSqrtRatioContainer()"
            mstore(0, shl(224, 0xa10459f4))
            revert(0, 4)
        }
        r := compute(sqrtRatio, roundUp)
    }
}

// Returns the 64.128 representation of the given sqrt ratio
function toFixed(SqrtRatio sqrtRatio) pure returns (uint256 r) {
    assembly ("memory-safe") {
        r := shl(add(2, shr(89, and(sqrtRatio, BIT_MASK))), and(sqrtRatio, not(BIT_MASK)))
    }
}

// The below operators assume that the SqrtRatio is valid, i.e. SqrtRatio#isValid returns true

function lt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) < SqrtRatio.unwrap(b);
}

function gt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) > SqrtRatio.unwrap(b);
}

function le(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) <= SqrtRatio.unwrap(b);
}

function ge(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) >= SqrtRatio.unwrap(b);
}

function eq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) == SqrtRatio.unwrap(b);
}

function neq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) != SqrtRatio.unwrap(b);
}

function isZero(SqrtRatio a) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(a)
    }
}

function max(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), gt(b, a)))
    }
}

function min(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), lt(b, a)))
    }
}

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

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolId} from "./poolId.sol";
import {PoolConfig} from "./poolConfig.sol";

using {toPoolId, validate} for PoolKey global;

/// @notice Unique identifier for a pool containing token addresses and configuration
/// @dev Each pool has its own state associated with this key
struct PoolKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing extension, fee, and tick spacing
    PoolConfig config;
}

/// @notice Thrown when tokens are not properly sorted (token0 >= token1)
error TokensMustBeSorted();

/// @notice Validates that a pool key is valid
/// @dev Checks that tokens are sorted and the config is valid
/// @param key The pool key to validate
function validate(PoolKey memory key) pure {
    if (key.token0 >= key.token1) revert TokensMustBeSorted();
    key.config.validate();
}

/// @notice Converts a pool key to a unique pool ID
/// @param key The pool key
/// @return result The unique pool ID (hash of the pool key)
function toPoolId(PoolKey memory key) pure returns (PoolId result) {
    assembly ("memory-safe") {
        // it's already copied into memory
        result := keccak256(key, 96)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type PoolBalanceUpdate is bytes32;

using {delta0, delta1} for PoolBalanceUpdate global;

function delta0(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, shr(128, update))
    }
}

function delta1(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, update)
    }
}

function createPoolBalanceUpdate(int128 _delta0, int128 _delta1) pure returns (PoolBalanceUpdate update) {
    assembly ("memory-safe") {
        // update = (delta0 << 128) | delta1
        update := or(shl(128, _delta0), and(_delta1, 0xffffffffffffffffffffffffffffffff))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio} from "./sqrtRatio.sol";

type PoolState is bytes32;

using {sqrtRatio, tick, liquidity, isInitialized, parse} for PoolState global;

function sqrtRatio(PoolState state) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, state)
    }
}

function tick(PoolState state) pure returns (int32 t) {
    assembly ("memory-safe") {
        t := signextend(3, shr(128, state))
    }
}

function liquidity(PoolState state) pure returns (uint128 l) {
    assembly ("memory-safe") {
        l := shr(128, shl(128, state))
    }
}

function isInitialized(PoolState state) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := iszero(iszero(state))
    }
}

function parse(PoolState state) pure returns (SqrtRatio r, int32 t, uint128 l) {
    assembly ("memory-safe") {
        r := shr(160, state)
        t := signextend(3, shr(128, state))
        l := shr(128, shl(128, state))
    }
}

function createPoolState(SqrtRatio _sqrtRatio, int32 _tick, uint128 _liquidity) pure returns (PoolState s) {
    assembly ("memory-safe") {
        // s = (sqrtRatio << 160) | (_tick << 128) | liquidity
        s := or(shl(160, _sqrtRatio), or(shl(128, and(_tick, 0xFFFFFFFF)), shr(128, shl(128, _liquidity))))
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

import {SqrtRatio, MIN_SQRT_RATIO_RAW, MAX_SQRT_RATIO_RAW} from "./sqrtRatio.sol";

type SwapParameters is bytes32;

using {
    sqrtRatioLimit,
    amount,
    isToken1,
    skipAhead,
    isExactOut,
    isPriceIncreasing,
    withDefaultSqrtRatioLimit
} for SwapParameters global;

function sqrtRatioLimit(SwapParameters params) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, params)
    }
}

function amount(SwapParameters params) pure returns (int128 a) {
    assembly ("memory-safe") {
        a := signextend(15, shr(32, params))
    }
}

function isToken1(SwapParameters params) pure returns (bool t) {
    assembly ("memory-safe") {
        t := and(shr(31, params), 1)
    }
}

function skipAhead(SwapParameters params) pure returns (uint256 s) {
    assembly ("memory-safe") {
        s := and(params, 0x7fffffff)
    }
}

function createSwapParameters(SqrtRatio _sqrtRatioLimit, int128 _amount, bool _isToken1, uint256 _skipAhead)
    pure
    returns (SwapParameters p)
{
    assembly ("memory-safe") {
        // p = (sqrtRatioLimit << 160) | (amount << 32) | (isToken1 << 31) | skipAhead
        // Mask each field to ensure dirty bits don't interfere
        // For isToken1, use iszero(iszero()) to convert any non-zero value to 1
        p := or(
            shl(160, _sqrtRatioLimit),
            or(
                shl(32, and(_amount, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)),
                or(shl(31, iszero(iszero(_isToken1))), and(_skipAhead, 0x7fffffff))
            )
        )
    }
}

function isExactOut(SwapParameters params) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := and(shr(159, params), 1)
    }
}

function isPriceIncreasing(SwapParameters params) pure returns (bool yes) {
    bool _isExactOut = params.isExactOut();
    bool _isToken1 = params.isToken1();
    assembly ("memory-safe") {
        yes := xor(_isExactOut, _isToken1)
    }
}

function withDefaultSqrtRatioLimit(SwapParameters params) pure returns (SwapParameters updated) {
    bool increasing = params.isPriceIncreasing();
    assembly ("memory-safe") {
        let replace := iszero(shr(160, params))
        let orMask :=
            shl(160, mul(replace, or(mul(increasing, MAX_SQRT_RATIO_RAW), mul(iszero(increasing), MIN_SQRT_RATIO_RAW))))
        updated := or(orMask, params)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type Locker is bytes32;

using {id, addr, parse} for Locker global;

function id(Locker locker) pure returns (uint256 v) {
    assembly ("memory-safe") {
        v := sub(shr(160, locker), 1)
    }
}

function addr(Locker locker) pure returns (address v) {
    assembly ("memory-safe") {
        v := shr(96, shl(96, locker))
    }
}

function parse(Locker locker) pure returns (uint256 lockerId, address lockerAddr) {
    assembly ("memory-safe") {
        lockerId := sub(shr(160, locker), 1)
        lockerAddr := shr(96, shl(96, locker))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

type PoolBalanceUpdate is bytes32;

using {delta0, delta1} for PoolBalanceUpdate global;

function delta0(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, shr(128, update))
    }
}

function delta1(PoolBalanceUpdate update) pure returns (int128 v) {
    assembly ("memory-safe") {
        v := signextend(15, update)
    }
}

function createPoolBalanceUpdate(int128 _delta0, int128 _delta1) pure returns (PoolBalanceUpdate update) {
    assembly ("memory-safe") {
        // update = (delta0 << 128) | delta1
        update := or(shl(128, _delta0), and(_delta1, 0xffffffffffffffffffffffffffffffff))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio} from "./sqrtRatio.sol";

type PoolState is bytes32;

using {sqrtRatio, tick, liquidity, isInitialized, parse} for PoolState global;

function sqrtRatio(PoolState state) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, state)
    }
}

function tick(PoolState state) pure returns (int32 t) {
    assembly ("memory-safe") {
        t := signextend(3, shr(128, state))
    }
}

function liquidity(PoolState state) pure returns (uint128 l) {
    assembly ("memory-safe") {
        l := shr(128, shl(128, state))
    }
}

function isInitialized(PoolState state) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := iszero(iszero(state))
    }
}

function parse(PoolState state) pure returns (SqrtRatio r, int32 t, uint128 l) {
    assembly ("memory-safe") {
        r := shr(160, state)
        t := signextend(3, shr(128, state))
        l := shr(128, shl(128, state))
    }
}

function createPoolState(SqrtRatio _sqrtRatio, int32 _tick, uint128 _liquidity) pure returns (PoolState s) {
    assembly ("memory-safe") {
        // s = (sqrtRatio << 160) | (_tick << 128) | liquidity
        s := or(shl(160, _sqrtRatio), or(shl(128, and(_tick, 0xFFFFFFFF)), shr(128, shl(128, _liquidity))))
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

// A dynamic fixed point number (a la floating point) that stores a shifting 94 bit view of the underlying fixed point value,
//  based on the most significant bits (mantissa)
// If the most significant 2 bits are 11, it represents a 64.30
// If the most significant 2 bits are 10, it represents a 32.62 number
// If the most significant 2 bits are 01, it represents a 0.94 number
// If the most significant 2 bits are 00, it represents a 0.126 number that is always less than 2**-32

type SqrtRatio is uint96;

uint96 constant MIN_SQRT_RATIO_RAW = 4611797791050542631;
SqrtRatio constant MIN_SQRT_RATIO = SqrtRatio.wrap(MIN_SQRT_RATIO_RAW);
uint96 constant MAX_SQRT_RATIO_RAW = 79227682466138141934206691491;
SqrtRatio constant MAX_SQRT_RATIO = SqrtRatio.wrap(MAX_SQRT_RATIO_RAW);

uint96 constant TWO_POW_95 = 0x800000000000000000000000;
uint96 constant TWO_POW_94 = 0x400000000000000000000000;
uint96 constant TWO_POW_62 = 0x4000000000000000;
uint96 constant TWO_POW_62_MINUS_ONE = 0x3fffffffffffffff;
uint96 constant BIT_MASK = 0xc00000000000000000000000; // TWO_POW_95 | TWO_POW_94

SqrtRatio constant ONE = SqrtRatio.wrap((TWO_POW_95) + (1 << 62));

using {
    toFixed,
    isValid,
    ge as >=,
    le as <=,
    lt as <,
    gt as >,
    eq as ==,
    neq as !=,
    isZero,
    min,
    max
} for SqrtRatio global;

function isValid(SqrtRatio sqrtRatio) pure returns (bool r) {
    assembly ("memory-safe") {
        r := and(
            // greater than or equal to TWO_POW_62, i.e. the whole number portion is nonzero
            gt(and(sqrtRatio, not(BIT_MASK)), TWO_POW_62_MINUS_ONE),
            // and between min/max sqrt ratio
            and(iszero(lt(sqrtRatio, MIN_SQRT_RATIO_RAW)), iszero(gt(sqrtRatio, MAX_SQRT_RATIO_RAW)))
        )
    }
}

error ValueOverflowsSqrtRatioContainer();

// If passing a value greater than this constant with roundUp = true, toSqrtRatio will overflow
// For roundUp = false, the constant is type(uint192).max
uint256 constant MAX_FIXED_VALUE_ROUND_UP =
    0x1000000000000000000000000000000000000000000000000 - 0x4000000000000000000000000;

// Converts a 64.128 value into the compact SqrtRatio representation
function toSqrtRatio(uint256 sqrtRatio, bool roundUp) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        function compute(sr, ru) -> v {
            // rup = 0x00...00 when false, 0xff...ff when true
            let rup := sub(0, ru)

            // Region: < 2**96  (shift = 2)
            let addmask := and(0x3, rup) // (1<<s)-1 if ru
            if lt(add(sr, addmask), shl(96, 1)) {
                v := shr(2, add(sr, addmask))
                leave
            }

            // Region: < 2**128 (shift = 34)  + set bit 94
            addmask := and(0x3ffffffff, rup)
            if lt(add(sr, addmask), shl(128, 1)) {
                v := or(shl(94, 1), shr(34, add(sr, addmask)))
                leave
            }

            // Region: < 2**160 (shift = 66)  + set bit 95
            addmask := and(0x3ffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(160, 1)) {
                v := or(shl(95, 1), shr(66, add(sr, addmask)))
                leave
            }

            // Region: < 2**192 (shift = 98)  + set bits 95|94
            addmask := and(0x3ffffffffffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(192, 1)) {
                v := or(shl(94, 3), shr(98, add(sr, addmask))) // 3<<94 == bit95|bit94
                leave
            }

            // cast sig "ValueOverflowsSqrtRatioContainer()"
            mstore(0, shl(224, 0xa10459f4))
            revert(0, 4)
        }
        r := compute(sqrtRatio, roundUp)
    }
}

// Returns the 64.128 representation of the given sqrt ratio
function toFixed(SqrtRatio sqrtRatio) pure returns (uint256 r) {
    assembly ("memory-safe") {
        r := shl(add(2, shr(89, and(sqrtRatio, BIT_MASK))), and(sqrtRatio, not(BIT_MASK)))
    }
}

// The below operators assume that the SqrtRatio is valid, i.e. SqrtRatio#isValid returns true

function lt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) < SqrtRatio.unwrap(b);
}

function gt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) > SqrtRatio.unwrap(b);
}

function le(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) <= SqrtRatio.unwrap(b);
}

function ge(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) >= SqrtRatio.unwrap(b);
}

function eq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) == SqrtRatio.unwrap(b);
}

function neq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) != SqrtRatio.unwrap(b);
}

function isZero(SqrtRatio a) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(a)
    }
}

function max(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), gt(b, a)))
    }
}

function min(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), lt(b, a)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolId} from "./poolId.sol";
import {PoolConfig} from "./poolConfig.sol";

using {toPoolId, validate} for PoolKey global;

/// @notice Unique identifier for a pool containing token addresses and configuration
/// @dev Each pool has its own state associated with this key
struct PoolKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing extension, fee, and tick spacing
    PoolConfig config;
}

/// @notice Thrown when tokens are not properly sorted (token0 >= token1)
error TokensMustBeSorted();

/// @notice Validates that a pool key is valid
/// @dev Checks that tokens are sorted and the config is valid
/// @param key The pool key to validate
function validate(PoolKey memory key) pure {
    if (key.token0 >= key.token1) revert TokensMustBeSorted();
    key.config.validate();
}

/// @notice Converts a pool key to a unique pool ID
/// @param key The pool key
/// @return result The unique pool ID (hash of the pool key)
function toPoolId(PoolKey memory key) pure returns (PoolId result) {
    assembly ("memory-safe") {
        // it's already copied into memory
        result := keccak256(key, 96)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {FeesPerLiquidity} from "./feesPerLiquidity.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";

// Position Type
// Represents a liquidity position in a pool
// Contains the position's liquidity amount and fee tracking information

/// @notice A liquidity position in a pool
/// @dev Tracks both the liquidity amount and the last known fees per liquidity for fee calculation
struct Position {
    /// @notice Extra data that can be set by the owner of a position
    bytes16 extraData;
    /// @notice Amount of liquidity in the position
    uint128 liquidity;
    /// @notice Snapshot of fees per liquidity when the position was last updated
    /// @dev Used to calculate fees owed to the position holder
    FeesPerLiquidity feesPerLiquidityInsideLast;
}

using {fees} for Position global;

/// @notice Calculates the fees owed to a position
/// @dev Returns the fee amounts of token0 and token1 owed to a position based on the given fees per liquidity inside snapshot
///      Note: if the computed fees overflow the uint128 type, it will return only the lower 128 bits. It is assumed that accumulated
///      fees will never exceed type(uint128).max.
/// @param position The position to calculate fees for
/// @param feesPerLiquidityInside Current fees per liquidity inside the position's bounds
/// @return Amount of token0 fees owed
/// @return Amount of token1 fees owed
function fees(Position memory position, FeesPerLiquidity memory feesPerLiquidityInside)
    pure
    returns (uint128, uint128)
{
    uint128 liquidity;
    uint256 difference0;
    uint256 difference1;
    assembly ("memory-safe") {
        liquidity := mload(add(position, 0x20))
        // feesPerLiquidityInsideLast is now at offset 0x40 due to extraData field
        let positionFpl := mload(add(position, 0x40))
        difference0 := sub(mload(feesPerLiquidityInside), mload(positionFpl))
        difference1 := sub(mload(add(feesPerLiquidityInside, 0x20)), mload(add(positionFpl, 0x20)))
    }

    return (
        uint128(FixedPointMathLib.fullMulDivN(difference0, liquidity, 128)),
        uint128(FixedPointMathLib.fullMulDivN(difference1, liquidity, 128))
    );
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// A dynamic fixed point number (a la floating point) that stores a shifting 94 bit view of the underlying fixed point value,
//  based on the most significant bits (mantissa)
// If the most significant 2 bits are 11, it represents a 64.30
// If the most significant 2 bits are 10, it represents a 32.62 number
// If the most significant 2 bits are 01, it represents a 0.94 number
// If the most significant 2 bits are 00, it represents a 0.126 number that is always less than 2**-32

type SqrtRatio is uint96;

uint96 constant MIN_SQRT_RATIO_RAW = 4611797791050542631;
SqrtRatio constant MIN_SQRT_RATIO = SqrtRatio.wrap(MIN_SQRT_RATIO_RAW);
uint96 constant MAX_SQRT_RATIO_RAW = 79227682466138141934206691491;
SqrtRatio constant MAX_SQRT_RATIO = SqrtRatio.wrap(MAX_SQRT_RATIO_RAW);

uint96 constant TWO_POW_95 = 0x800000000000000000000000;
uint96 constant TWO_POW_94 = 0x400000000000000000000000;
uint96 constant TWO_POW_62 = 0x4000000000000000;
uint96 constant TWO_POW_62_MINUS_ONE = 0x3fffffffffffffff;
uint96 constant BIT_MASK = 0xc00000000000000000000000; // TWO_POW_95 | TWO_POW_94

SqrtRatio constant ONE = SqrtRatio.wrap((TWO_POW_95) + (1 << 62));

using {
    toFixed,
    isValid,
    ge as >=,
    le as <=,
    lt as <,
    gt as >,
    eq as ==,
    neq as !=,
    isZero,
    min,
    max
} for SqrtRatio global;

function isValid(SqrtRatio sqrtRatio) pure returns (bool r) {
    assembly ("memory-safe") {
        r := and(
            // greater than or equal to TWO_POW_62, i.e. the whole number portion is nonzero
            gt(and(sqrtRatio, not(BIT_MASK)), TWO_POW_62_MINUS_ONE),
            // and between min/max sqrt ratio
            and(iszero(lt(sqrtRatio, MIN_SQRT_RATIO_RAW)), iszero(gt(sqrtRatio, MAX_SQRT_RATIO_RAW)))
        )
    }
}

error ValueOverflowsSqrtRatioContainer();

// If passing a value greater than this constant with roundUp = true, toSqrtRatio will overflow
// For roundUp = false, the constant is type(uint192).max
uint256 constant MAX_FIXED_VALUE_ROUND_UP =
    0x1000000000000000000000000000000000000000000000000 - 0x4000000000000000000000000;

// Converts a 64.128 value into the compact SqrtRatio representation
function toSqrtRatio(uint256 sqrtRatio, bool roundUp) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        function compute(sr, ru) -> v {
            // rup = 0x00...00 when false, 0xff...ff when true
            let rup := sub(0, ru)

            // Region: < 2**96  (shift = 2)
            let addmask := and(0x3, rup) // (1<<s)-1 if ru
            if lt(add(sr, addmask), shl(96, 1)) {
                v := shr(2, add(sr, addmask))
                leave
            }

            // Region: < 2**128 (shift = 34)  + set bit 94
            addmask := and(0x3ffffffff, rup)
            if lt(add(sr, addmask), shl(128, 1)) {
                v := or(shl(94, 1), shr(34, add(sr, addmask)))
                leave
            }

            // Region: < 2**160 (shift = 66)  + set bit 95
            addmask := and(0x3ffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(160, 1)) {
                v := or(shl(95, 1), shr(66, add(sr, addmask)))
                leave
            }

            // Region: < 2**192 (shift = 98)  + set bits 95|94
            addmask := and(0x3ffffffffffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(192, 1)) {
                v := or(shl(94, 3), shr(98, add(sr, addmask))) // 3<<94 == bit95|bit94
                leave
            }

            // cast sig "ValueOverflowsSqrtRatioContainer()"
            mstore(0, shl(224, 0xa10459f4))
            revert(0, 4)
        }
        r := compute(sqrtRatio, roundUp)
    }
}

// Returns the 64.128 representation of the given sqrt ratio
function toFixed(SqrtRatio sqrtRatio) pure returns (uint256 r) {
    assembly ("memory-safe") {
        r := shl(add(2, shr(89, and(sqrtRatio, BIT_MASK))), and(sqrtRatio, not(BIT_MASK)))
    }
}

// The below operators assume that the SqrtRatio is valid, i.e. SqrtRatio#isValid returns true

function lt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) < SqrtRatio.unwrap(b);
}

function gt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) > SqrtRatio.unwrap(b);
}

function le(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) <= SqrtRatio.unwrap(b);
}

function ge(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) >= SqrtRatio.unwrap(b);
}

function eq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) == SqrtRatio.unwrap(b);
}

function neq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) != SqrtRatio.unwrap(b);
}

function isZero(SqrtRatio a) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(a)
    }
}

function max(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), gt(b, a)))
    }
}

function min(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), lt(b, a)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio, MIN_SQRT_RATIO_RAW, MAX_SQRT_RATIO_RAW} from "./sqrtRatio.sol";

type SwapParameters is bytes32;

using {
    sqrtRatioLimit,
    amount,
    isToken1,
    skipAhead,
    isExactOut,
    isPriceIncreasing,
    withDefaultSqrtRatioLimit
} for SwapParameters global;

function sqrtRatioLimit(SwapParameters params) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, params)
    }
}

function amount(SwapParameters params) pure returns (int128 a) {
    assembly ("memory-safe") {
        a := signextend(15, shr(32, params))
    }
}

function isToken1(SwapParameters params) pure returns (bool t) {
    assembly ("memory-safe") {
        t := and(shr(31, params), 1)
    }
}

function skipAhead(SwapParameters params) pure returns (uint256 s) {
    assembly ("memory-safe") {
        s := and(params, 0x7fffffff)
    }
}

function createSwapParameters(SqrtRatio _sqrtRatioLimit, int128 _amount, bool _isToken1, uint256 _skipAhead)
    pure
    returns (SwapParameters p)
{
    assembly ("memory-safe") {
        // p = (sqrtRatioLimit << 160) | (amount << 32) | (isToken1 << 31) | skipAhead
        // Mask each field to ensure dirty bits don't interfere
        // For isToken1, use iszero(iszero()) to convert any non-zero value to 1
        p := or(
            shl(160, _sqrtRatioLimit),
            or(
                shl(32, and(_amount, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)),
                or(shl(31, iszero(iszero(_isToken1))), and(_skipAhead, 0x7fffffff))
            )
        )
    }
}

function isExactOut(SwapParameters params) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := and(shr(159, params), 1)
    }
}

function isPriceIncreasing(SwapParameters params) pure returns (bool yes) {
    bool _isExactOut = params.isExactOut();
    bool _isToken1 = params.isToken1();
    assembly ("memory-safe") {
        yes := xor(_isExactOut, _isToken1)
    }
}

function withDefaultSqrtRatioLimit(SwapParameters params) pure returns (SwapParameters updated) {
    bool increasing = params.isPriceIncreasing();
    assembly ("memory-safe") {
        let replace := iszero(shr(160, params))
        let orMask :=
            shl(160, mul(replace, or(mul(increasing, MAX_SQRT_RATIO_RAW), mul(iszero(increasing), MIN_SQRT_RATIO_RAW))))
        updated := or(orMask, params)
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolId} from "./poolId.sol";
import {PoolConfig} from "./poolConfig.sol";

using {toPoolId, validate} for PoolKey global;

/// @notice Unique identifier for a pool containing token addresses and configuration
/// @dev Each pool has its own state associated with this key
struct PoolKey {
    /// @notice Address of token0 (must be < token1)
    address token0;
    /// @notice Address of token1 (must be > token0)
    address token1;
    /// @notice Packed configuration containing extension, fee, and tick spacing
    PoolConfig config;
}

/// @notice Thrown when tokens are not properly sorted (token0 >= token1)
error TokensMustBeSorted();

/// @notice Validates that a pool key is valid
/// @dev Checks that tokens are sorted and the config is valid
/// @param key The pool key to validate
function validate(PoolKey memory key) pure {
    if (key.token0 >= key.token1) revert TokensMustBeSorted();
    key.config.validate();
}

/// @notice Converts a pool key to a unique pool ID
/// @param key The pool key
/// @return result The unique pool ID (hash of the pool key)
function toPoolId(PoolKey memory key) pure returns (PoolId result) {
    assembly ("memory-safe") {
        // it's already copied into memory
        result := keccak256(key, 96)
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

struct CallPoints {
    bool beforeInitializePool;
    bool afterInitializePool;
    bool beforeSwap;
    bool afterSwap;
    bool beforeUpdatePosition;
    bool afterUpdatePosition;
    bool beforeCollectFees;
    bool afterCollectFees;
}

using {eq, isValid, toUint8} for CallPoints global;

function eq(CallPoints memory a, CallPoints memory b) pure returns (bool) {
    return (a.beforeInitializePool == b.beforeInitializePool && a.afterInitializePool == b.afterInitializePool
            && a.beforeSwap == b.beforeSwap && a.afterSwap == b.afterSwap
            && a.beforeUpdatePosition == b.beforeUpdatePosition && a.afterUpdatePosition == b.afterUpdatePosition
            && a.beforeCollectFees == b.beforeCollectFees && a.afterCollectFees == b.afterCollectFees);
}

function isValid(CallPoints memory a) pure returns (bool) {
    return (a.beforeInitializePool || a.afterInitializePool || a.beforeSwap || a.afterSwap || a.beforeUpdatePosition
            || a.afterUpdatePosition || a.beforeCollectFees || a.afterCollectFees);
}

function toUint8(CallPoints memory callPoints) pure returns (uint8 b) {
    assembly ("memory-safe") {
        b := add(
            add(
                add(
                    add(
                        add(
                            add(
                                add(mload(callPoints), mul(128, mload(add(callPoints, 32)))),
                                mul(64, mload(add(callPoints, 64)))
                            ),
                            mul(32, mload(add(callPoints, 96)))
                        ),
                        mul(16, mload(add(callPoints, 128)))
                    ),
                    mul(8, mload(add(callPoints, 160)))
                ),
                mul(4, mload(add(callPoints, 192)))
            ),
            mul(2, mload(add(callPoints, 224)))
        )
    }
}

function addressToCallPoints(address a) pure returns (CallPoints memory result) {
    result = byteToCallPoints(uint8(uint160(a) >> 152));
}

function byteToCallPoints(uint8 b) pure returns (CallPoints memory result) {
    // note the order of bytes does not match the struct order of elements because we are matching the cairo implementation
    // which for legacy reasons has the fields in this order
    result = CallPoints({
        beforeInitializePool: (b & 1) != 0,
        afterInitializePool: (b & 128) != 0,
        beforeSwap: (b & 64) != 0,
        afterSwap: (b & 32) != 0,
        beforeUpdatePosition: (b & 16) != 0,
        afterUpdatePosition: (b & 8) != 0,
        beforeCollectFees: (b & 4) != 0,
        afterCollectFees: (b & 2) != 0
    });
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// The total fees per liquidity for each token.
// Since these are always read together we put them in a struct, even though they cannot be packed.
struct FeesPerLiquidity {
    uint256 value0;
    uint256 value1;
}

using {sub} for FeesPerLiquidity global;

function sub(FeesPerLiquidity memory a, FeesPerLiquidity memory b) pure returns (FeesPerLiquidity memory result) {
    assembly ("memory-safe") {
        mstore(result, sub(mload(a), mload(b)))
        mstore(add(result, 32), sub(mload(add(a, 32)), mload(add(b, 32))))
    }
}

function feesPerLiquidityFromAmounts(uint128 amount0, uint128 amount1, uint128 liquidity)
    pure
    returns (FeesPerLiquidity memory result)
{
    assembly ("memory-safe") {
        mstore(result, div(shl(128, amount0), liquidity))
        mstore(add(result, 32), div(shl(128, amount1), liquidity))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

// A dynamic fixed point number (a la floating point) that stores a shifting 94 bit view of the underlying fixed point value,
//  based on the most significant bits (mantissa)
// If the most significant 2 bits are 11, it represents a 64.30
// If the most significant 2 bits are 10, it represents a 32.62 number
// If the most significant 2 bits are 01, it represents a 0.94 number
// If the most significant 2 bits are 00, it represents a 0.126 number that is always less than 2**-32

type SqrtRatio is uint96;

uint96 constant MIN_SQRT_RATIO_RAW = 4611797791050542631;
SqrtRatio constant MIN_SQRT_RATIO = SqrtRatio.wrap(MIN_SQRT_RATIO_RAW);
uint96 constant MAX_SQRT_RATIO_RAW = 79227682466138141934206691491;
SqrtRatio constant MAX_SQRT_RATIO = SqrtRatio.wrap(MAX_SQRT_RATIO_RAW);

uint96 constant TWO_POW_95 = 0x800000000000000000000000;
uint96 constant TWO_POW_94 = 0x400000000000000000000000;
uint96 constant TWO_POW_62 = 0x4000000000000000;
uint96 constant TWO_POW_62_MINUS_ONE = 0x3fffffffffffffff;
uint96 constant BIT_MASK = 0xc00000000000000000000000; // TWO_POW_95 | TWO_POW_94

SqrtRatio constant ONE = SqrtRatio.wrap((TWO_POW_95) + (1 << 62));

using {
    toFixed,
    isValid,
    ge as >=,
    le as <=,
    lt as <,
    gt as >,
    eq as ==,
    neq as !=,
    isZero,
    min,
    max
} for SqrtRatio global;

function isValid(SqrtRatio sqrtRatio) pure returns (bool r) {
    assembly ("memory-safe") {
        r := and(
            // greater than or equal to TWO_POW_62, i.e. the whole number portion is nonzero
            gt(and(sqrtRatio, not(BIT_MASK)), TWO_POW_62_MINUS_ONE),
            // and between min/max sqrt ratio
            and(iszero(lt(sqrtRatio, MIN_SQRT_RATIO_RAW)), iszero(gt(sqrtRatio, MAX_SQRT_RATIO_RAW)))
        )
    }
}

error ValueOverflowsSqrtRatioContainer();

// If passing a value greater than this constant with roundUp = true, toSqrtRatio will overflow
// For roundUp = false, the constant is type(uint192).max
uint256 constant MAX_FIXED_VALUE_ROUND_UP =
    0x1000000000000000000000000000000000000000000000000 - 0x4000000000000000000000000;

// Converts a 64.128 value into the compact SqrtRatio representation
function toSqrtRatio(uint256 sqrtRatio, bool roundUp) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        function compute(sr, ru) -> v {
            // rup = 0x00...00 when false, 0xff...ff when true
            let rup := sub(0, ru)

            // Region: < 2**96  (shift = 2)
            let addmask := and(0x3, rup) // (1<<s)-1 if ru
            if lt(add(sr, addmask), shl(96, 1)) {
                v := shr(2, add(sr, addmask))
                leave
            }

            // Region: < 2**128 (shift = 34)  + set bit 94
            addmask := and(0x3ffffffff, rup)
            if lt(add(sr, addmask), shl(128, 1)) {
                v := or(shl(94, 1), shr(34, add(sr, addmask)))
                leave
            }

            // Region: < 2**160 (shift = 66)  + set bit 95
            addmask := and(0x3ffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(160, 1)) {
                v := or(shl(95, 1), shr(66, add(sr, addmask)))
                leave
            }

            // Region: < 2**192 (shift = 98)  + set bits 95|94
            addmask := and(0x3ffffffffffffffffffffffff, rup)
            if lt(add(sr, addmask), shl(192, 1)) {
                v := or(shl(94, 3), shr(98, add(sr, addmask))) // 3<<94 == bit95|bit94
                leave
            }

            // cast sig "ValueOverflowsSqrtRatioContainer()"
            mstore(0, shl(224, 0xa10459f4))
            revert(0, 4)
        }
        r := compute(sqrtRatio, roundUp)
    }
}

// Returns the 64.128 representation of the given sqrt ratio
function toFixed(SqrtRatio sqrtRatio) pure returns (uint256 r) {
    assembly ("memory-safe") {
        r := shl(add(2, shr(89, and(sqrtRatio, BIT_MASK))), and(sqrtRatio, not(BIT_MASK)))
    }
}

// The below operators assume that the SqrtRatio is valid, i.e. SqrtRatio#isValid returns true

function lt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) < SqrtRatio.unwrap(b);
}

function gt(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) > SqrtRatio.unwrap(b);
}

function le(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) <= SqrtRatio.unwrap(b);
}

function ge(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) >= SqrtRatio.unwrap(b);
}

function eq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) == SqrtRatio.unwrap(b);
}

function neq(SqrtRatio a, SqrtRatio b) pure returns (bool r) {
    r = SqrtRatio.unwrap(a) != SqrtRatio.unwrap(b);
}

function isZero(SqrtRatio a) pure returns (bool r) {
    assembly ("memory-safe") {
        r := iszero(a)
    }
}

function max(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), gt(b, a)))
    }
}

function min(SqrtRatio a, SqrtRatio b) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := xor(a, mul(xor(a, b), lt(b, a)))
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @notice Unique identifier for a pool
/// @dev Wraps bytes32 to provide type safety for pool identifiers
type PoolId is bytes32;

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SqrtRatio} from "./sqrtRatio.sol";

type PoolState is bytes32;

using {sqrtRatio, tick, liquidity, isInitialized, parse} for PoolState global;

function sqrtRatio(PoolState state) pure returns (SqrtRatio r) {
    assembly ("memory-safe") {
        r := shr(160, state)
    }
}

function tick(PoolState state) pure returns (int32 t) {
    assembly ("memory-safe") {
        t := signextend(3, shr(128, state))
    }
}

function liquidity(PoolState state) pure returns (uint128 l) {
    assembly ("memory-safe") {
        l := shr(128, shl(128, state))
    }
}

function isInitialized(PoolState state) pure returns (bool yes) {
    assembly ("memory-safe") {
        yes := iszero(iszero(state))
    }
}

function parse(PoolState state) pure returns (SqrtRatio r, int32 t, uint128 l) {
    assembly ("memory-safe") {
        r := shr(160, state)
        t := signextend(3, shr(128, state))
        l := shr(128, shl(128, state))
    }
}

function createPoolState(SqrtRatio _sqrtRatio, int32 _tick, uint128 _liquidity) pure returns (PoolState s) {
    assembly ("memory-safe") {
        // s = (sqrtRatio << 160) | (_tick << 128) | liquidity
        s := or(shl(160, _sqrtRatio), or(shl(128, and(_tick, 0xFFFFFFFF)), shr(128, shl(128, _liquidity))))
    }
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {CallPoints} from "../types/callPoints.sol";
import {PoolKey, PoolConfig} from "../types/poolKey.sol";
import {PositionId} from "../types/positionId.sol";
import {ICore, IExtension} from "../interfaces/ICore.sol";
import {IOracle} from "../interfaces/extensions/IOracle.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {ExposedStorage} from "../base/ExposedStorage.sol";
import {BaseExtension} from "../base/BaseExtension.sol";
import {NATIVE_TOKEN_ADDRESS} from "../math/constants.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {Snapshot, createSnapshot} from "../types/snapshot.sol";
import {Counts, createCounts} from "../types/counts.sol";
import {Observation, createObservation} from "../types/observation.sol";
import {PoolId} from "../types/poolId.sol";
import {PoolState} from "../types/poolState.sol";
import {SwapParameters} from "../types/swapParameters.sol";
import {Locker} from "../types/locker.sol";

/// @notice Returns the call points configuration for the Oracle extension
/// @dev Specifies which hooks the Oracle needs to capture price and liquidity data
/// @return The call points configuration for Oracle functionality
function oracleCallPoints() pure returns (CallPoints memory) {
    return CallPoints({
        beforeInitializePool: true,
        afterInitializePool: false,
        beforeUpdatePosition: true,
        afterUpdatePosition: false,
        beforeSwap: true,
        afterSwap: false,
        beforeCollectFees: false,
        afterCollectFees: false
    });
}

/// @notice Converts a logical index to a storage index for circular snapshot array
/// @dev Because the snapshots array is circular, the storage index of the most recently written snapshot can be any value in [0,c.count).
///      To simplify the code, we operate on the logical indices, rather than the storage indices.
///      For logical indices, the most recently written value is always at logicalIndex = c.count-1 and the earliest snapshot is always at logicalIndex = 0.
/// @param index The index of the most recently written snapshot
/// @param count The total number of snapshots that have been written
/// @param logicalIndex The index of the snapshot for which to compute the storage index
/// @return The storage index corresponding to the logical index
function logicalIndexToStorageIndex(uint256 index, uint256 count, uint256 logicalIndex) pure returns (uint256) {
    // We assume index < count and logicalIndex < count
    unchecked {
        return (index + 1 + logicalIndex) % count;
    }
}

/// @title Ekubo Oracle Extension
/// @author Moody Salem <moody@ekubo.org>
/// @notice Records price and liquidity into accumulators enabling a separate contract to compute a manipulation resistant average price and liquidity
contract Oracle is IOracle, ExposedStorage, BaseExtension {
    using CoreLib for ICore;

    constructor(ICore core) BaseExtension(core) {}

    /// @notice Emits a snapshot event for off-chain indexing
    /// @dev Uses assembly for gas-efficient event emission
    /// @param token The token address for the snapshot
    /// @param snapshot The snapshot that was just written
    function _emitSnapshotEvent(address token, Snapshot snapshot) private {
        unchecked {
            assembly ("memory-safe") {
                mstore(0, shl(96, token))
                mstore(20, snapshot)
                log0(0, 52)
            }
        }
    }

    /// @inheritdoc IOracle
    function getPoolKey(address token) public view returns (PoolKey memory) {
        PoolConfig config;
        assembly ("memory-safe") {
            config := shl(96, address())
        }
        return PoolKey({token0: NATIVE_TOKEN_ADDRESS, token1: token, config: config});
    }

    /// @notice Returns the call points configuration for this extension
    /// @dev Overrides the base implementation to return Oracle-specific call points
    /// @return The call points configuration
    function getCallPoints() internal pure override returns (CallPoints memory) {
        return oracleCallPoints();
    }

    /// @notice Inserts a new snapshot if enough time has passed since the last one
    /// @dev Only inserts if block.timestamp > lastTimestamp to avoid duplicate snapshots
    /// @param poolId The unique identifier for the pool
    /// @param token The token address for the oracle data
    function maybeInsertSnapshot(PoolId poolId, address token) private {
        unchecked {
            Counts c;
            assembly ("memory-safe") {
                c := sload(token)
            }

            uint32 timePassed = uint32(block.timestamp) - c.lastTimestamp();
            if (timePassed == 0) return;

            uint32 index = c.index();

            // we know count is always g.t. 0 in the places this is called
            Snapshot last;
            assembly ("memory-safe") {
                last := sload(or(shl(32, token), index))
            }

            PoolState state = CORE.poolState(poolId);

            uint128 liquidity = state.liquidity();
            uint256 nonZeroLiquidity;
            assembly ("memory-safe") {
                nonZeroLiquidity := add(liquidity, iszero(liquidity))
            }

            Snapshot snapshot = createSnapshot({
                _timestamp: uint32(block.timestamp),
                _secondsPerLiquidityCumulative: last.secondsPerLiquidityCumulative()
                    + uint160(FixedPointMathLib.rawDiv(uint256(timePassed) << 128, nonZeroLiquidity)),
                _tickCumulative: last.tickCumulative() + int64(uint64(timePassed)) * state.tick()
            });

            uint32 count = c.count();
            uint32 capacity = c.capacity();

            bool isLastIndex = index == count - 1;
            bool incrementCount = isLastIndex && capacity > count;

            if (incrementCount) count++;
            index = (index + 1) % count;
            uint32 lastTimestamp = uint32(block.timestamp);

            c = createCounts({_index: index, _count: count, _capacity: capacity, _lastTimestamp: lastTimestamp});
            assembly ("memory-safe") {
                sstore(token, c)
                sstore(or(shl(32, token), index), snapshot)
            }

            _emitSnapshotEvent(token, snapshot);
        }
    }

    /// @notice Called before a pool is initialized to set up Oracle tracking
    /// @dev Validates pool configuration and initializes the first snapshot
    function beforeInitializePool(address, PoolKey calldata key, int32)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (key.token0 != NATIVE_TOKEN_ADDRESS) revert PairsWithNativeTokenOnly();
        if (key.config.fee() != 0) revert FeeMustBeZero();
        if (!key.config.isFullRange()) revert FullRangePoolOnly();

        address token = key.token1;

        // in case expandCapacity is called before the pool is initialized:
        //  remember we have the capacity since the snapshot storage has been initialized
        uint32 lastTimestamp = uint32(block.timestamp);

        Counts c;
        assembly ("memory-safe") {
            c := sload(token)
        }

        c = createCounts({
            _index: 0,
            _count: 1,
            _capacity: uint32(FixedPointMathLib.max(1, c.capacity())),
            _lastTimestamp: lastTimestamp
        });

        Snapshot snapshot =
            createSnapshot({_timestamp: lastTimestamp, _secondsPerLiquidityCumulative: 0, _tickCumulative: 0});

        assembly ("memory-safe") {
            sstore(token, c)
            sstore(shl(32, token), snapshot)
        }

        _emitSnapshotEvent(token, snapshot);
    }

    /// @notice Called before a position is updated to capture price/liquidity snapshot
    /// @dev Inserts a new snapshot if liquidity is changing
    function beforeUpdatePosition(Locker, PoolKey memory poolKey, PositionId, int128 liquidityDelta)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (liquidityDelta != 0) {
            maybeInsertSnapshot(poolKey.toPoolId(), poolKey.token1);
        }
    }

    /// @notice Called before a swap to capture price/liquidity snapshot
    /// @dev Inserts a new snapshot if a swap is occurring
    function beforeSwap(Locker, PoolKey memory poolKey, SwapParameters params)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (params.amount() != 0) {
            maybeInsertSnapshot(poolKey.toPoolId(), poolKey.token1);
        }
    }

    /// @inheritdoc IOracle
    function expandCapacity(address token, uint32 minCapacity) external returns (uint32 capacity) {
        Counts c;
        assembly ("memory-safe") {
            c := sload(token)
        }

        if (c.capacity() < minCapacity) {
            for (uint256 i = c.capacity(); i < minCapacity; i++) {
                assembly ("memory-safe") {
                    // Simply initialize the slot, it will be overwritten when the index is reached
                    sstore(or(shl(32, token), i), 1)
                }
            }
            c = createCounts({
                _index: c.index(), _count: c.count(), _capacity: minCapacity, _lastTimestamp: c.lastTimestamp()
            });
            assembly ("memory-safe") {
                sstore(token, c)
            }
        }

        capacity = c.capacity();
    }

    /// @notice Searches for the latest snapshot with timestamp <= time within a logical range
    /// @dev Searches the logical range [min, maxExclusive) for the latest snapshot with timestamp <= time.
    ///      See logicalIndexToStorageIndex for an explanation of logical indices.
    ///      We make the assumption that all snapshots for the token were written within (2**32 - 1) seconds of the current block timestamp
    /// @param c The counts containing metadata about the snapshots array
    /// @param token The token address to search snapshots for
    /// @param time The target timestamp to search for
    /// @param logicalMin The minimum logical index to search from
    /// @param logicalMaxExclusive The maximum logical index to search to (exclusive)
    /// @return logicalIndex The logical index of the found snapshot
    /// @return snapshot The snapshot data at the found index
    function searchRangeForPrevious(
        Counts c,
        address token,
        uint256 time,
        uint256 logicalMin,
        uint256 logicalMaxExclusive
    ) private view returns (uint256 logicalIndex, Snapshot snapshot) {
        unchecked {
            if (logicalMin >= logicalMaxExclusive) {
                revert NoPreviousSnapshotExists(token, time);
            }

            uint32 current = uint32(block.timestamp);
            uint32 targetDiff = current - uint32(time);

            uint256 left = logicalMin;
            uint256 right = logicalMaxExclusive - 1;
            while (left < right) {
                uint256 mid = (left + right + 1) >> 1;
                uint256 storageIndex = logicalIndexToStorageIndex(c.index(), c.count(), mid);
                Snapshot midSnapshot;
                assembly ("memory-safe") {
                    midSnapshot := sload(or(shl(32, token), storageIndex))
                }
                if (current - midSnapshot.timestamp() >= targetDiff) {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }

            uint256 resultIndex = logicalIndexToStorageIndex(c.index(), c.count(), left);
            assembly ("memory-safe") {
                snapshot := sload(or(shl(32, token), resultIndex))
            }
            if (current - snapshot.timestamp() < targetDiff) {
                revert NoPreviousSnapshotExists(token, time);
            }
            return (left, snapshot);
        }
    }

    /// @inheritdoc IOracle
    function findPreviousSnapshot(address token, uint256 time)
        public
        view
        returns (uint256 count, uint256 logicalIndex, Snapshot snapshot)
    {
        if (time > block.timestamp) revert FutureTime();

        Counts c;
        assembly ("memory-safe") {
            c := sload(token)
        }
        count = c.count();
        (logicalIndex, snapshot) = searchRangeForPrevious(c, token, time, 0, count);
    }

    /// @notice Computes cumulative values at a given time by extrapolating from a previous snapshot
    /// @dev Uses linear interpolation between snapshots or current pool state for extrapolation
    /// @param c The counts containing metadata about the snapshots array
    /// @param token The token address to extrapolate for
    /// @param atTime The timestamp to extrapolate to
    /// @param logicalIndex The logical index of the base snapshot
    /// @param snapshot The base snapshot to extrapolate from
    /// @return secondsPerLiquidityCumulative The extrapolated seconds per liquidity cumulative
    /// @return tickCumulative The extrapolated tick cumulative
    function extrapolateSnapshotInternal(
        Counts c,
        address token,
        uint256 atTime,
        uint256 logicalIndex,
        Snapshot snapshot
    ) private view returns (uint160 secondsPerLiquidityCumulative, int64 tickCumulative) {
        unchecked {
            secondsPerLiquidityCumulative = snapshot.secondsPerLiquidityCumulative();
            tickCumulative = snapshot.tickCumulative();
            uint32 timePassed = uint32(atTime) - snapshot.timestamp();
            if (timePassed != 0) {
                if (logicalIndex == c.count() - 1) {
                    // Use current pool state.
                    PoolId poolId = getPoolKey(token).toPoolId();
                    PoolState state = CORE.poolState(poolId);

                    tickCumulative += int64(state.tick()) * int64(uint64(timePassed));
                    secondsPerLiquidityCumulative += uint160(
                        FixedPointMathLib.rawDiv(
                            uint256(timePassed) << 128, FixedPointMathLib.max(1, state.liquidity())
                        )
                    );
                } else {
                    // Use the next snapshot.
                    uint256 logicalIndexNext = logicalIndexToStorageIndex(c.index(), c.count(), logicalIndex + 1);
                    Snapshot next;
                    assembly ("memory-safe") {
                        next := sload(or(shl(32, token), logicalIndexNext))
                    }

                    uint32 timestampDifference = next.timestamp() - snapshot.timestamp();

                    tickCumulative += int64(
                        FixedPointMathLib.rawSDiv(
                            int256(uint256(timePassed)) * (next.tickCumulative() - snapshot.tickCumulative()),
                            int256(uint256(timestampDifference))
                        )
                    );
                    secondsPerLiquidityCumulative += uint160(
                        (uint256(timePassed)
                                * (next.secondsPerLiquidityCumulative() - snapshot.secondsPerLiquidityCumulative()))
                            / timestampDifference
                    );
                }
            }
        }
    }

    /// @inheritdoc IOracle
    function extrapolateSnapshot(address token, uint256 atTime)
        public
        view
        returns (uint160 secondsPerLiquidityCumulative, int64 tickCumulative)
    {
        if (atTime > block.timestamp) revert FutureTime();

        Counts c;
        assembly ("memory-safe") {
            c := sload(token)
        }
        (uint256 logicalIndex, Snapshot snapshot) = searchRangeForPrevious(c, token, atTime, 0, c.count());
        (secondsPerLiquidityCumulative, tickCumulative) =
            extrapolateSnapshotInternal(c, token, atTime, logicalIndex, snapshot);
    }

    /// @inheritdoc IOracle
    function getExtrapolatedSnapshotsForSortedTimestamps(address token, uint256[] memory timestamps)
        public
        view
        returns (Observation[] memory observations)
    {
        unchecked {
            if (timestamps.length == 0) revert ZeroTimestampsProvided();
            uint256 startTime = timestamps[0];
            uint256 endTime = timestamps[timestamps.length - 1];
            if (endTime < startTime) revert EndTimeLessThanStartTime();

            Counts c;
            assembly ("memory-safe") {
                c := sload(token)
            }
            (uint256 indexFirst,) = searchRangeForPrevious(c, token, startTime, 0, c.count());
            (uint256 indexLast,) = searchRangeForPrevious(c, token, endTime, indexFirst, c.count());

            observations = new Observation[](timestamps.length);
            uint256 lastTimestamp;
            for (uint256 i = 0; i < timestamps.length; i++) {
                uint256 timestamp = timestamps[i];

                if (timestamp < lastTimestamp) {
                    revert TimestampsNotSorted();
                } else if (timestamp > block.timestamp) {
                    revert FutureTime();
                }

                (uint256 logicalIndex, Snapshot snapshot) =
                    searchRangeForPrevious(c, token, timestamp, indexFirst, indexLast + 1);
                (uint160 spcCumulative, int64 tcCumulative) =
                    extrapolateSnapshotInternal(c, token, timestamp, logicalIndex, snapshot);
                observations[i] = createObservation(spcCumulative, tcCumulative);
                indexFirst = logicalIndex;
                lastTimestamp = timestamp;
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {BasePositions} from "./base/BasePositions.sol";
import {ICore} from "./interfaces/ICore.sol";
import {PoolKey} from "./types/poolKey.sol";
import {computeFee} from "./math/fee.sol";

/// @title Ekubo Protocol Positions
/// @author Moody Salem <moody@ekubo.org>
/// @notice Tracks liquidity positions in Ekubo Protocol as NFTs
/// @dev Manages liquidity positions, fee collection, and protocol fees
contract Positions is BasePositions {
    /// @notice Protocol fee rate for swaps (as a fraction of 2^64)
    uint64 public immutable SWAP_PROTOCOL_FEE_X64;

    /// @notice Denominator for withdrawal protocol fee calculation
    uint64 public immutable WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR;

    /// @notice Constructs the Positions contract
    /// @param core The core contract instance
    /// @param owner The owner of the contract (for access control)
    /// @param _swapProtocolFeeX64 Protocol fee rate for swaps
    /// @param _withdrawalProtocolFeeDenominator Denominator for withdrawal protocol fee
    constructor(ICore core, address owner, uint64 _swapProtocolFeeX64, uint64 _withdrawalProtocolFeeDenominator)
        BasePositions(core, owner)
    {
        SWAP_PROTOCOL_FEE_X64 = _swapProtocolFeeX64;
        WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR = _withdrawalProtocolFeeDenominator;
    }

    /// @notice Handles protocol fee collection during fee collection
    /// @dev Implements the abstract method from BasePositions
    /// @param amount0 The amount of token0 fees collected before protocol fee deduction
    /// @param amount1 The amount of token1 fees collected before protocol fee deduction
    /// @return protocolFee0 The amount of token0 protocol fees to collect
    /// @return protocolFee1 The amount of token1 protocol fees to collect
    function _computeSwapProtocolFees(PoolKey memory, uint128 amount0, uint128 amount1)
        internal
        view
        override
        returns (uint128 protocolFee0, uint128 protocolFee1)
    {
        if (SWAP_PROTOCOL_FEE_X64 != 0) {
            protocolFee0 = computeFee(amount0, SWAP_PROTOCOL_FEE_X64);
            protocolFee1 = computeFee(amount1, SWAP_PROTOCOL_FEE_X64);
        }
    }

    /// @notice Handles protocol fee collection during liquidity withdrawal
    /// @dev Implements the abstract method from BasePositions
    /// @param poolKey The pool key for the position
    /// @param amount0 The amount of token0 being withdrawn before protocol fee deduction
    /// @param amount1 The amount of token1 being withdrawn before protocol fee deduction
    /// @return protocolFee0 The amount of token0 protocol fees to collect
    /// @return protocolFee1 The amount of token1 protocol fees to collect
    function _computeWithdrawalProtocolFees(PoolKey memory poolKey, uint128 amount0, uint128 amount1)
        internal
        view
        override
        returns (uint128 protocolFee0, uint128 protocolFee1)
    {
        uint64 fee = poolKey.config.fee();
        if (fee != 0 && WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR != 0) {
            protocolFee0 = computeFee(amount0, fee / WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR);
            protocolFee1 = computeFee(amount1, fee / WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR);
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {Router} from "./Router.sol";
import {ICore, PoolKey} from "./interfaces/ICore.sol";
import {PoolState} from "./types/poolState.sol";
import {PoolBalanceUpdate} from "./types/poolBalanceUpdate.sol";
import {SwapParameters} from "./types/swapParameters.sol";
import {FlashAccountantLib} from "./libraries/FlashAccountantLib.sol";
import {CoreLib} from "./libraries/CoreLib.sol";

/// @title Ekubo MEV Capture Router
/// @author Moody Salem <moody@ekubo.org>
/// @notice Enables swapping and quoting against pools in Ekubo Protocol including the MEV capture extension pools
contract MEVCaptureRouter is Router {
    using FlashAccountantLib for *;
    using CoreLib for *;

    address public immutable MEV_CAPTURE;

    constructor(ICore core, address _mevCapture) Router(core) {
        MEV_CAPTURE = _mevCapture;
    }

    function _swap(uint256 value, PoolKey memory poolKey, SwapParameters params)
        internal
        override
        returns (PoolBalanceUpdate balanceUpdate, PoolState stateAfter)
    {
        if (poolKey.config.extension() != MEV_CAPTURE) {
            (balanceUpdate, stateAfter) = CORE.swap(value, poolKey, params.withDefaultSqrtRatioLimit());
        } else {
            (balanceUpdate, stateAfter) = abi.decode(
                CORE.forward(MEV_CAPTURE, abi.encode(poolKey, params.withDefaultSqrtRatioLimit())),
                (PoolBalanceUpdate, PoolState)
            );
            if (value != 0) {
                SafeTransferLib.safeTransferETH(address(CORE), value);
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {IOrders} from "./IOrders.sol";
import {BuybacksState} from "../types/buybacksState.sol";
import {IExposedStorage} from "./IExposedStorage.sol";

/// @title Revenue Buybacks Interface
/// @notice Interface for automated revenue buyback orders using TWAMM (Time-Weighted Average Market Maker)
/// @dev Defines the interface for managing buyback orders for protocol revenue
interface IRevenueBuybacks is IExposedStorage {
    /// @notice Thrown when minimum order duration exceeds target order duration
    /// @dev This would prevent order creation since the condition order duration >= min order duration would not always be met
    error MinOrderDurationGreaterThanTargetOrderDuration();

    /// @notice Thrown when minimum order duration is set to zero
    /// @dev Orders cannot have zero duration, so this prevents invalid configurations
    error MinOrderDurationMustBeGreaterThanZero();

    /// @notice Thrown when roll is called and a token is not configured
    error TokenNotConfigured(address token);

    /// @notice Emitted when a token's buyback configuration is updated
    /// @param token The token being configured for buybacks
    /// @param state The state after configuring the token
    event Configured(address token, BuybacksState state);

    /// @notice The Orders contract used to create and manage TWAMM orders
    /// @dev All buyback orders are created through this contract
    function ORDERS() external view returns (IOrders);

    /// @notice The NFT token ID that represents all buyback orders created by this contract
    /// @dev A single NFT is minted and reused for all buyback orders to simplify management
    function NFT_ID() external view returns (uint256);

    /// @notice The token that is purchased with collected revenue
    /// @dev This is typically the protocol's governance or utility token
    function BUY_TOKEN() external view returns (address);

    /// @notice Approves the Orders contract to spend unlimited amounts of a token
    /// @dev Must be called at least once for each revenue token before creating buyback orders
    /// @param token The token to approve for spending by the Orders contract
    function approveMax(address token) external;

    /// @notice Withdraws leftover tokens from the contract (only callable by owner)
    /// @dev Used to recover tokens that may be stuck in the contract or to withdraw excess funds
    /// @param token The address of the token to withdraw
    /// @param amount The amount of tokens to withdraw
    function take(address token, uint256 amount) external;

    /// @notice Collects the proceeds from a completed buyback order
    /// @dev Can be called by anyone at any time to collect proceeds from orders that have finished
    /// @param token The revenue token that was sold in the order
    /// @param fee The fee tier of the pool where the order was executed
    /// @param endTime The end time of the order to collect proceeds from
    /// @return proceeds The amount of buyToken received from the completed order
    function collect(address token, uint64 fee, uint64 endTime) external returns (uint128 proceeds);

    /// @notice Creates a new buyback order or extends an existing one with available revenue
    /// @dev Can be called by anyone to trigger the creation of buyback orders using collected revenue
    /// This function will either extend the current order (if conditions are met) or create a new order
    /// @param token The revenue token to use for creating the buyback order
    /// @return endTime The end time of the order that was created or extended
    /// @return saleRate The sale rate of the order (amount of token sold per second)
    function roll(address token) external returns (uint64 endTime, uint112 saleRate);

    /// @notice Configures buyback parameters for a revenue token (only callable by owner)
    /// @dev Sets the timing and fee parameters for automated buyback order creation
    /// @param token The revenue token to configure
    /// @param targetOrderDuration The target duration for new orders (in seconds)
    /// @param minOrderDuration The minimum duration threshold for creating new orders (in seconds)
    /// @param fee The fee tier for the buyback pool
    function configure(address token, uint32 targetOrderDuration, uint32 minOrderDuration, uint64 fee) external;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {Ownable} from "solady/auth/Ownable.sol";
import {Multicallable} from "solady/utils/Multicallable.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {nextValidTime} from "./math/time.sol";
import {IOrders} from "./interfaces/IOrders.sol";
import {IRevenueBuybacks} from "./interfaces/IRevenueBuybacks.sol";
import {BuybacksState, createBuybacksState} from "./types/buybacksState.sol";
import {OrderKey} from "./types/orderKey.sol";
import {createOrderConfig} from "./types/orderConfig.sol";
import {ExposedStorage} from "./base/ExposedStorage.sol";
import {NATIVE_TOKEN_ADDRESS} from "./math/constants.sol";

/// @title Revenue Buybacks
/// @author Moody Salem <moody@ekubo.org>
/// @notice Creates automated revenue buyback orders using TWAMM (Time-Weighted Average Market Maker)
/// @dev Final contract that manages the creation and execution of buyback orders for protocol revenue
/// This contract automatically creates TWAMM orders to buy back a specified token using collected revenue
contract RevenueBuybacks is IRevenueBuybacks, ExposedStorage, Ownable, Multicallable {
    /// @notice The Orders contract used to create and manage TWAMM orders
    /// @dev All buyback orders are created through this contract
    IOrders public immutable ORDERS;

    /// @notice The NFT token ID that represents all buyback orders created by this contract
    /// @dev A single NFT is minted and reused for all buyback orders to simplify management
    uint256 public immutable NFT_ID;

    /// @notice The token that is purchased with collected revenue
    /// @dev This is typically the protocol's governance or utility token
    address public immutable BUY_TOKEN;

    /// @notice Constructs the RevenueBuybacks contract
    /// @param owner The address that will own this contract and have administrative privileges
    /// @param _orders The Orders contract instance for creating TWAMM orders
    /// @param _buyToken The token that will be purchased with collected revenue
    constructor(address owner, IOrders _orders, address _buyToken) {
        _initializeOwner(owner);
        ORDERS = _orders;
        BUY_TOKEN = _buyToken;
        NFT_ID = ORDERS.mint();
    }

    /// @notice Approves the Orders contract to spend unlimited amounts of a token
    /// @dev Must be called at least once for each revenue token before creating buyback orders
    /// @param token The token to approve for spending by the Orders contract
    function approveMax(address token) external {
        SafeTransferLib.safeApproveWithRetry(token, address(ORDERS), type(uint256).max);
    }

    /// @notice Withdraws leftover tokens from the contract (only callable by owner)
    /// @dev Used to recover tokens that may be stuck in the contract
    /// @param token The address of the token to withdraw
    /// @param amount The amount of tokens to withdraw
    function take(address token, uint256 amount) external onlyOwner {
        // Transfer to msg.sender since only the owner can call this function
        SafeTransferLib.safeTransfer(token, msg.sender, amount);
    }

    /// @notice Withdraws native tokens held by this contract
    /// @dev Used to recover native tokens that may be stuck in the contract
    /// @param amount The amount of native tokens to withdraw
    function takeNative(uint256 amount) external onlyOwner {
        // Transfer to msg.sender since only the owner can call this function
        SafeTransferLib.safeTransferETH(msg.sender, amount);
    }

    /// @notice Collects the proceeds from a completed buyback order
    /// @dev Can be called by anyone at any time to collect proceeds from orders that have finished
    /// @param token The revenue token that was sold in the order
    /// @param fee The fee tier of the pool where the order was executed
    /// @param endTime The end time of the order to collect proceeds from
    /// @return proceeds The amount of buyToken received from the completed order
    function collect(address token, uint64 fee, uint64 endTime) external returns (uint128 proceeds) {
        proceeds = ORDERS.collectProceeds(NFT_ID, _createOrderKey(token, fee, 0, endTime), owner());
    }

    /// @notice Allows the contract to receive ETH revenue
    /// @dev Required to accept ETH payments when ETH is used as a revenue token
    receive() external payable {}

    /// @notice Creates a new buyback order or extends an existing one with available revenue
    /// @dev Can be called by anyone to trigger the creation of buyback orders using collected revenue
    /// This function will either extend the current order (if conditions are met) or create a new order
    /// @param token The revenue token to use for creating the buyback order, or NATIVE_TOKEN_ADDRESS
    /// @return endTime The end time of the order that was created or extended
    /// @return saleRate The sale rate of the order (amount of token sold per second)
    function roll(address token) public returns (uint64 endTime, uint112 saleRate) {
        unchecked {
            BuybacksState state;
            assembly ("memory-safe") {
                state := sload(token)
            }

            if (!state.isConfigured()) {
                revert TokenNotConfigured(token);
            }

            // minOrderDuration == 0 indicates the token is not configured
            bool isEth = token == NATIVE_TOKEN_ADDRESS;
            uint256 amountToSpend = isEth ? address(this).balance : SafeTransferLib.balanceOf(token, address(this));

            uint32 timeRemaining = state.lastEndTime() - uint32(block.timestamp);
            // if the fee changed, or the amount of time exceeds the min order duration
            // note the time remaining can underflow if the last order has ended. in this case time remaining will be greater than min order duration,
            // but also greater than last order duration, so it will not be re-used.
            if (
                state.fee() == state.lastFee() && timeRemaining >= state.minOrderDuration()
                    && timeRemaining <= state.lastOrderDuration()
            ) {
                // handles overflow
                endTime = uint64(block.timestamp + timeRemaining);
            } else {
                endTime =
                    uint64(nextValidTime(block.timestamp, block.timestamp + uint256(state.targetOrderDuration()) - 1));

                state = createBuybacksState({
                    _targetOrderDuration: state.targetOrderDuration(),
                    _minOrderDuration: state.minOrderDuration(),
                    _fee: state.fee(),
                    _lastEndTime: uint32(endTime),
                    _lastOrderDuration: uint32(endTime - block.timestamp),
                    _lastFee: state.fee()
                });

                assembly ("memory-safe") {
                    sstore(token, state)
                }
            }

            if (amountToSpend != 0) {
                saleRate = ORDERS.increaseSellAmount{value: isEth ? amountToSpend : 0}(
                    NFT_ID, _createOrderKey(token, state.fee(), 0, endTime), uint128(amountToSpend), type(uint112).max
                );
            }
        }
    }

    /// @notice Configures buyback parameters for a revenue token (only callable by owner)
    /// @dev Sets the timing and fee parameters for automated buyback order creation
    /// @param token The revenue token to configure
    /// @param targetOrderDuration The target duration for new orders (in seconds)
    /// @param minOrderDuration The minimum duration threshold for creating new orders (in seconds)
    /// @param fee The fee tier for the buyback pool
    function configure(address token, uint32 targetOrderDuration, uint32 minOrderDuration, uint64 fee)
        external
        onlyOwner
    {
        if (minOrderDuration > targetOrderDuration) revert MinOrderDurationGreaterThanTargetOrderDuration();
        if (minOrderDuration == 0 && targetOrderDuration != 0) {
            revert MinOrderDurationMustBeGreaterThanZero();
        }

        BuybacksState state;
        assembly ("memory-safe") {
            state := sload(token)
        }
        state = createBuybacksState({
            _targetOrderDuration: targetOrderDuration,
            _minOrderDuration: minOrderDuration,
            _fee: fee,
            _lastEndTime: state.lastEndTime(),
            _lastOrderDuration: state.lastOrderDuration(),
            _lastFee: state.lastFee()
        });
        assembly ("memory-safe") {
            sstore(token, state)
        }

        emit Configured(token, state);
    }

    function _createOrderKey(address token, uint64 fee, uint64 startTime, uint64 endTime)
        internal
        view
        returns (OrderKey memory key)
    {
        bool isToken1 = token > BUY_TOKEN;
        address buyToken = BUY_TOKEN;
        assembly ("memory-safe") {
            mstore(add(key, mul(isToken1, 32)), token)
            mstore(add(key, mul(iszero(isToken1), 32)), buyToken)
        }

        key.config = createOrderConfig({_fee: fee, _isToken1: isToken1, _startTime: startTime, _endTime: endTime});
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";

import {BaseForwardee} from "./base/BaseForwardee.sol";
import {UsesCore} from "./base/UsesCore.sol";
import {ICore} from "./interfaces/ICore.sol";
import {toDate, toQuarter} from "./libraries/TimeDescriptor.sol";
import {CoreLib} from "./libraries/CoreLib.sol";
import {FlashAccountantLib} from "./libraries/FlashAccountantLib.sol";
import {Locker} from "./types/locker.sol";

/// @title Time-locked Token Wrapper
/// @author Ekubo Protocol
/// @notice Wraps tokens that can only be unwrapped after a specific unlock time
/// @dev Wrapping and unwrapping happens via Ekubo Core#forward. Implements full ERC20 functionality
contract TokenWrapper is UsesCore, IERC20, BaseForwardee {
    using CoreLib for *;
    using FlashAccountantLib for *;

    /// @notice Thrown when trying to unwrap the token before the token has unlocked
    error TooEarly();

    /// @notice Thrown when attempting to transfer an amount greater than the balance
    error InsufficientBalance();

    /// @notice Thrown when calling transferFrom with an insufficient allowance
    error InsufficientAllowance();

    /// @notice The underlying token that is wrapped by this contract
    IERC20 public immutable UNDERLYING_TOKEN;

    /// @notice The timestamp after which the token may be unwrapped
    uint256 public immutable UNLOCK_TIME;

    /// @notice Constructs a new TokenWrapper
    /// @param core The Ekubo Core contract
    /// @param _underlyingToken The token to be wrapped
    /// @param _unlockTime The timestamp after which tokens can be unwrapped
    constructor(ICore core, IERC20 _underlyingToken, uint256 _unlockTime) UsesCore(core) BaseForwardee(core) {
        UNDERLYING_TOKEN = _underlyingToken;
        UNLOCK_TIME = _unlockTime;
    }

    /// @inheritdoc IERC20
    mapping(address owner => mapping(address spender => uint256)) public override allowance;

    /// @notice Mapping of account balances (not public because we use coreBalance for Core)
    /// @dev Private mapping to track individual account balances
    mapping(address account => uint256) private _balanceOf;

    /// @notice Transient balance for the Core contract
    /// @dev Core never actually holds a real balance of this token, we just use this transient balance to enable low cost payments to core
    uint256 private transient coreBalance;

    /// @inheritdoc IERC20
    /// @dev Returns the transient balance for Core contract, otherwise returns stored balance
    function balanceOf(address account) external view returns (uint256) {
        if (account == address(CORE)) return coreBalance;
        return _balanceOf[account];
    }

    /// @inheritdoc IERC20
    /// @dev Total supply is tracked in Core's saved balances
    function totalSupply() external view override returns (uint256) {
        (uint128 supply,) = CORE.savedBalances({
            owner: address(this),
            token0: address(UNDERLYING_TOKEN),
            token1: address(type(uint160).max),
            salt: bytes32(0)
        });

        return supply;
    }

    /// @inheritdoc IERC20
    /// @dev Combines underlying token name with unlock date
    function name() external view returns (string memory) {
        return string.concat(UNDERLYING_TOKEN.name(), " ", toDate(UNLOCK_TIME));
    }

    /// @inheritdoc IERC20
    /// @dev Combines "g" prefix with underlying token symbol and quarter
    function symbol() external view returns (string memory) {
        return string.concat("g", UNDERLYING_TOKEN.symbol(), "-", toQuarter(UNLOCK_TIME));
    }

    /// @inheritdoc IERC20
    function decimals() external view returns (uint8) {
        return UNDERLYING_TOKEN.decimals();
    }

    /// @inheritdoc IERC20
    function transfer(address to, uint256 amount) external returns (bool) {
        // note we do not need to check that core balance is sufficient as the sender
        // even if the caller gets core to withdraw to itself, as part of a payment, it will net to 0 with the Core#withdraw call
        if (msg.sender != address(CORE)) {
            uint256 balance = _balanceOf[msg.sender];
            if (balance < amount) {
                revert InsufficientBalance();
            }
            // since we already checked balance >= amount
            unchecked {
                _balanceOf[msg.sender] = balance - amount;
            }
        }
        if (to == address(CORE)) {
            coreBalance += amount;
        } else if (to != address(0)) {
            // we save storage writes on burn by checking to != address(0)
            _balanceOf[to] += amount;
        }
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    /// @inheritdoc IERC20
    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    /// @inheritdoc IERC20
    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowanceCurrent = allowance[from][msg.sender];
        if (allowanceCurrent != type(uint256).max) {
            if (allowanceCurrent < amount) revert InsufficientAllowance();
            // since we already checked allowanceCurrent >= amount
            unchecked {
                allowance[from][msg.sender] = allowanceCurrent - amount;
            }
        }

        // we know `from` at this point will never be address(core) for amount > 0, since Core will never give an allowance to any address

        uint256 balance = _balanceOf[from];
        if (balance < amount) {
            revert InsufficientBalance();
        }
        // since we already checked balance >= amount
        unchecked {
            _balanceOf[from] = balance - amount;
        }

        if (to == address(CORE)) {
            coreBalance += amount;
        } else {
            _balanceOf[to] += amount;
        }
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    /// @notice Handles wrap/unwrap operations forwarded from Core
    /// @dev Encode (int256 delta) in the forwarded data, where a positive amount means wrapping and a negative amount means unwrapping
    /// For wrap: the specified amount of this wrapper token will be credited to the locker and the same amount of underlying will be debited
    /// For unwrap: the specified amount of the underlying will be credited to the locker and the same amount of this wrapper token will be debited, iff block.timestamp > unlockTime and at least that much token has been wrapped
    /// @param data Encoded int256 delta (positive for wrap, negative for unwrap)
    /// @return Empty bytes (no return data needed)
    function handleForwardData(Locker, bytes memory data) internal override returns (bytes memory) {
        (int256 amount) = abi.decode(data, (int256));

        // unwrap
        if (amount < 0) {
            if (block.timestamp < UNLOCK_TIME) revert TooEarly();
        }

        CORE.updateSavedBalances({
            token0: address(UNDERLYING_TOKEN),
            token1: address(type(uint160).max),
            salt: bytes32(0),
            delta0: amount,
            delta1: 0
        });

        CORE.updateDebt(SafeCastLib.toInt128(-amount));

        return bytes("");
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {ILocker, IForwardee} from "../IFlashAccountant.sol";
import {IExtension} from "../ICore.sol";
import {IExposedStorage} from "../IExposedStorage.sol";
import {PoolKey} from "../../types/poolKey.sol";
import {OrderKey} from "../../types/orderKey.sol";
import {OrderConfig} from "../../types/orderConfig.sol";
import {PoolId} from "../../types/poolId.sol";

/// @title TWAMM Interface
/// @notice Interface for the Ekubo TWAMM Extension
/// @dev Extension for Ekubo Protocol that enables creation of DCA orders that are executed over time
interface ITWAMM is IExposedStorage, IExtension, ILocker, IForwardee {
    /// @notice Emitted when an order is updated
    /// @param owner Address of the order owner
    /// @param salt Unique salt for the order
    /// @param orderKey Order key identifying the order
    /// @param saleRateDelta Change in sale rate applied
    event OrderUpdated(address owner, bytes32 salt, OrderKey orderKey, int112 saleRateDelta);

    /// @notice Emitted when proceeds are withdrawn from an order
    /// @param owner Address of the order owner
    /// @param salt Unique salt for the order
    /// @param orderKey Order key identifying the order
    /// @param amount Amount of tokens withdrawn
    event OrderProceedsWithdrawn(address owner, bytes32 salt, OrderKey orderKey, uint128 amount);

    /// @notice Thrown when the number of orders at a time would overflow
    error TimeNumOrdersOverflow();

    /// @notice Thrown when tick spacing is not the maximum allowed value
    error FullRangePoolOnly();

    /// @notice Thrown when trying to modify an order that has already ended
    error OrderAlreadyEnded();

    /// @notice Thrown when order timestamps are invalid
    error InvalidTimestamps();

    /// @notice Thrown when sale rate delta exceeds maximum allowed value
    error MaxSaleRateDeltaPerTime();

    /// @notice Thrown when trying to operate on an uninitialized pool
    error PoolNotInitialized();

    /// @notice Gets the reward rate inside a time range for a specific token
    /// @dev Used to calculate how much of the buy token an order has earned
    /// @param poolId Unique identifier for the pool
    /// @param config The order config that is being checked
    /// @return result The reward rate inside the specified range
    function getRewardRateInside(PoolId poolId, OrderConfig config) external view returns (uint256 result);

    /// @notice Locks core and executes virtual orders for the given pool key
    /// @dev The pool key must use this extension, which is checked in the locked callback
    /// @param poolKey Pool key identifying the pool
    function lockAndExecuteVirtualOrders(PoolKey memory poolKey) external;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PayableMulticallable} from "./base/PayableMulticallable.sol";
import {BaseLocker} from "./base/BaseLocker.sol";
import {UsesCore} from "./base/UsesCore.sol";
import {ICore} from "./interfaces/ICore.sol";
import {PoolKey} from "./types/poolKey.sol";
import {NATIVE_TOKEN_ADDRESS} from "./math/constants.sol";
import {SqrtRatio} from "./types/sqrtRatio.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {FlashAccountantLib} from "./libraries/FlashAccountantLib.sol";
import {CoreLib} from "./libraries/CoreLib.sol";
import {PoolState} from "./types/poolState.sol";
import {PoolBalanceUpdate, createPoolBalanceUpdate} from "./types/poolBalanceUpdate.sol";
import {SwapParameters, createSwapParameters} from "./types/swapParameters.sol";

/// @notice Represents a single hop in a multi-hop swap route
/// @dev Contains pool information and swap parameters for one step
struct RouteNode {
    /// @notice Pool key identifying the pool for this hop
    PoolKey poolKey;
    /// @notice Price limit for this hop (0 for no limit)
    SqrtRatio sqrtRatioLimit;
    /// @notice Number of ticks to skip ahead for gas optimization
    uint256 skipAhead;
}

/// @notice Represents a token and amount pair
/// @dev Used to specify input/output tokens and amounts in swaps
struct TokenAmount {
    /// @notice Address of the token
    address token;
    /// @notice Amount of the token (positive or negative)
    int128 amount;
}

/// @notice Represents a multi-hop swap with route and initial token amount
/// @dev Contains the complete path and starting point for a swap
struct Swap {
    /// @notice Array of route nodes defining the swap path
    RouteNode[] route;
    /// @notice Initial token and amount for the swap
    TokenAmount tokenAmount;
}

/// @title Ekubo Protocol Router
/// @author Moody Salem <moody@ekubo.org>
/// @notice Enables swapping and quoting against pools in Ekubo Protocol
/// @dev Provides high-level swap functionality including single-hop, multi-hop, and batch swaps
contract Router is UsesCore, PayableMulticallable, BaseLocker {
    using FlashAccountantLib for *;
    using CoreLib for *;

    uint256 private constant CALL_TYPE_SINGLE_SWAP = 0;
    uint256 private constant CALL_TYPE_MULTIHOP_SWAP = 1;
    uint256 private constant CALL_TYPE_MULTI_MULTIHOP_SWAP = 3; // == 1 | 2
    uint256 private constant CALL_TYPE_QUOTE = 4;

    /// @notice Thrown when a swap doesn't consume the full input amount
    error PartialSwapsDisallowed();

    /// @notice Thrown when the calculated amount doesn't meet the minimum threshold
    /// @param expectedAmount The minimum expected amount
    /// @param calculatedAmount The actual calculated amount
    error SlippageCheckFailed(int256 expectedAmount, int256 calculatedAmount);

    /// @notice Thrown when tokens don't match across multiple swaps
    /// @param index The index of the mismatched swap
    error TokensMismatch(uint256 index);

    /// @notice Constructs the Router contract
    /// @param core The core contract instance
    constructor(ICore core) BaseLocker(core) UsesCore(core) {}

    /// @notice Internal function to execute a swap against the core contract
    /// @dev Virtual function that can be overridden by derived contracts
    /// @param value Native token value to send with the swap
    /// @param poolKey Pool key identifying the pool
    /// @param params The parameters of the swap to make
    /// @return balanceUpdate Change in token0 and token1 balances of the pool
    function _swap(uint256 value, PoolKey memory poolKey, SwapParameters params)
        internal
        virtual
        returns (PoolBalanceUpdate balanceUpdate, PoolState stateAfter)
    {
        (balanceUpdate, stateAfter) = CORE.swap(value, poolKey, params.withDefaultSqrtRatioLimit());
    }

    function handleLockData(uint256, bytes memory data) internal override returns (bytes memory result) {
        uint256 callType = abi.decode(data, (uint256));

        if (callType == CALL_TYPE_SINGLE_SWAP) {
            // swap
            (
                ,
                address swapper,
                PoolKey memory poolKey,
                SwapParameters params,
                int256 calculatedAmountThreshold,
                address recipient
            ) = abi.decode(data, (uint256, address, PoolKey, SwapParameters, int256, address));

            unchecked {
                uint256 value = FixedPointMathLib.ternary(
                    !params.isToken1() && !params.isExactOut() && poolKey.token0 == NATIVE_TOKEN_ADDRESS,
                    uint128(params.amount()),
                    0
                );

                bool increasing = params.isPriceIncreasing();

                (PoolBalanceUpdate balanceUpdate,) = _swap(value, poolKey, params);

                int128 amountCalculated = params.isToken1() ? -balanceUpdate.delta0() : -balanceUpdate.delta1();
                if (amountCalculated < calculatedAmountThreshold) {
                    revert SlippageCheckFailed(calculatedAmountThreshold, amountCalculated);
                }

                if (increasing) {
                    if (balanceUpdate.delta0() != 0) {
                        ACCOUNTANT.withdraw(poolKey.token0, recipient, uint128(-balanceUpdate.delta0()));
                    }
                    if (balanceUpdate.delta1() != 0) {
                        ACCOUNTANT.payFrom(swapper, poolKey.token1, uint128(balanceUpdate.delta1()));
                    }
                } else {
                    if (balanceUpdate.delta1() != 0) {
                        ACCOUNTANT.withdraw(poolKey.token1, recipient, uint128(-balanceUpdate.delta1()));
                    }

                    if (balanceUpdate.delta0() != 0) {
                        if (poolKey.token0 == NATIVE_TOKEN_ADDRESS) {
                            int256 valueDifference = int256(value) - int256(balanceUpdate.delta0());

                            // refund the overpaid ETH to the swapper
                            if (valueDifference > 0) {
                                ACCOUNTANT.withdraw(NATIVE_TOKEN_ADDRESS, swapper, uint128(uint256(valueDifference)));
                            } else if (valueDifference < 0) {
                                SafeTransferLib.safeTransferETH(address(ACCOUNTANT), uint128(uint256(-valueDifference)));
                            }
                        } else {
                            ACCOUNTANT.payFrom(swapper, poolKey.token0, uint128(balanceUpdate.delta0()));
                        }
                    }
                }

                result = abi.encode(balanceUpdate);
            }
        } else if ((callType & CALL_TYPE_MULTIHOP_SWAP) != 0) {
            address swapper;
            Swap[] memory swaps;
            int256 calculatedAmountThreshold;

            if (callType == CALL_TYPE_MULTIHOP_SWAP) {
                Swap memory s;
                // multihopSwap
                (, swapper, s, calculatedAmountThreshold) = abi.decode(data, (uint256, address, Swap, int256));

                swaps = new Swap[](1);
                swaps[0] = s;
            } else {
                // multiMultihopSwap
                (, swapper, swaps, calculatedAmountThreshold) = abi.decode(data, (uint256, address, Swap[], int256));
            }

            PoolBalanceUpdate[][] memory results = new PoolBalanceUpdate[][](swaps.length);

            unchecked {
                int256 totalCalculated;
                int256 totalSpecified;
                address specifiedToken;
                address calculatedToken;

                for (uint256 i = 0; i < swaps.length; i++) {
                    Swap memory s = swaps[i];
                    results[i] = new PoolBalanceUpdate[](s.route.length);

                    TokenAmount memory tokenAmount = s.tokenAmount;
                    totalSpecified += tokenAmount.amount;

                    for (uint256 j = 0; j < s.route.length; j++) {
                        RouteNode memory node = s.route[j];

                        bool isToken1 = tokenAmount.token == node.poolKey.token1;
                        require(isToken1 || tokenAmount.token == node.poolKey.token0);

                        (PoolBalanceUpdate update,) = _swap(
                            0,
                            node.poolKey,
                            createSwapParameters({
                                _amount: tokenAmount.amount,
                                _isToken1: isToken1,
                                _sqrtRatioLimit: node.sqrtRatioLimit,
                                _skipAhead: node.skipAhead
                            })
                        );
                        results[i][j] = update;

                        if (isToken1) {
                            if (update.delta1() != tokenAmount.amount) revert PartialSwapsDisallowed();
                            tokenAmount = TokenAmount({token: node.poolKey.token0, amount: -update.delta0()});
                        } else {
                            if (update.delta0() != tokenAmount.amount) revert PartialSwapsDisallowed();
                            tokenAmount = TokenAmount({token: node.poolKey.token1, amount: -update.delta1()});
                        }
                    }

                    totalCalculated += tokenAmount.amount;

                    if (i == 0) {
                        specifiedToken = s.tokenAmount.token;
                        calculatedToken = tokenAmount.token;
                    } else {
                        if (specifiedToken != s.tokenAmount.token || calculatedToken != tokenAmount.token) {
                            revert TokensMismatch(i);
                        }
                    }
                }

                if (totalCalculated < calculatedAmountThreshold) {
                    revert SlippageCheckFailed(calculatedAmountThreshold, totalCalculated);
                }

                if (totalSpecified < 0) {
                    ACCOUNTANT.withdraw(specifiedToken, swapper, uint128(uint256(-totalSpecified)));
                } else if (totalSpecified > 0) {
                    if (specifiedToken == NATIVE_TOKEN_ADDRESS) {
                        SafeTransferLib.safeTransferETH(address(ACCOUNTANT), uint128(uint256(totalSpecified)));
                    } else {
                        ACCOUNTANT.payFrom(swapper, specifiedToken, uint128(uint256(totalSpecified)));
                    }
                }

                if (totalCalculated > 0) {
                    ACCOUNTANT.withdraw(calculatedToken, swapper, uint128(uint256(totalCalculated)));
                } else if (totalCalculated < 0) {
                    if (calculatedToken == NATIVE_TOKEN_ADDRESS) {
                        SafeTransferLib.safeTransferETH(address(ACCOUNTANT), uint128(uint256(-totalCalculated)));
                    } else {
                        ACCOUNTANT.payFrom(swapper, calculatedToken, uint128(uint256(-totalCalculated)));
                    }
                }
            }

            if (callType == CALL_TYPE_MULTIHOP_SWAP) {
                result = abi.encode(results[0]);
            } else {
                result = abi.encode(results);
            }
        } else if (callType == CALL_TYPE_QUOTE) {
            (, PoolKey memory poolKey, SwapParameters params) = abi.decode(data, (uint256, PoolKey, SwapParameters));

            (PoolBalanceUpdate balanceUpdate, PoolState stateAfter) = _swap(0, poolKey, params);

            revert QuoteReturnValue(balanceUpdate, stateAfter);
        }
    }

    /// @notice Executes a single-hop swap with a specified recipient
    /// @param poolKey Pool key identifying the pool to swap against
    /// @param params The swap parameters to execute
    /// @param calculatedAmountThreshold Minimum amount to receive (for slippage protection)
    /// @return balanceUpdate Change in token0 and token1 balance of the pool
    function swap(PoolKey memory poolKey, SwapParameters params, int256 calculatedAmountThreshold)
        public
        payable
        returns (PoolBalanceUpdate balanceUpdate)
    {
        balanceUpdate = swap(poolKey, params, calculatedAmountThreshold, msg.sender);
    }

    /// @notice Executes a single-hop swap with a specified recipient
    /// @param poolKey Pool key identifying the pool to swap against
    /// @param params The swap parameters to execute
    /// @param calculatedAmountThreshold Minimum amount to receive (for slippage protection)
    /// @param recipient Address to receive the output tokens
    /// @return balanceUpdate Change in token0 and token1 balance of the pool
    function swap(PoolKey memory poolKey, SwapParameters params, int256 calculatedAmountThreshold, address recipient)
        public
        payable
        returns (PoolBalanceUpdate balanceUpdate)
    {
        (balanceUpdate) = abi.decode(
            lock(abi.encode(CALL_TYPE_SINGLE_SWAP, msg.sender, poolKey, params, calculatedAmountThreshold, recipient)),
            (PoolBalanceUpdate)
        );
    }

    /// @notice Executes a single-hop swap with a specified recipient
    /// @param poolKey Pool key identifying the pool to swap against
    /// @param isToken1 True if swapping token1, false if swapping token0
    /// @param amount Amount to swap (positive for exact input, negative for exact output)
    /// @param sqrtRatioLimit Price limit for the swap (0 for no limit)
    /// @param skipAhead Number of ticks to skip ahead for gas optimization
    /// @param calculatedAmountThreshold Minimum amount to receive (for slippage protection)
    /// @param recipient Address to receive the output tokens
    /// @return balanceUpdate Change in token0 and token1 balance of the pool
    function swap(
        PoolKey memory poolKey,
        bool isToken1,
        int128 amount,
        SqrtRatio sqrtRatioLimit,
        uint256 skipAhead,
        int256 calculatedAmountThreshold,
        address recipient
    ) public payable returns (PoolBalanceUpdate balanceUpdate) {
        balanceUpdate = swap(
            poolKey,
            createSwapParameters({
                _isToken1: isToken1, _amount: amount, _sqrtRatioLimit: sqrtRatioLimit, _skipAhead: skipAhead
            }),
            calculatedAmountThreshold,
            recipient
        );
    }

    /// @notice Executes a single-hop swap with msg.sender as recipient
    /// @param poolKey Pool key identifying the pool to swap against
    /// @param isToken1 True if swapping token1, false if swapping token0
    /// @param amount Amount to swap (positive for exact input, negative for exact output)
    /// @param sqrtRatioLimit Price limit for the swap (0 for no limit)
    /// @param skipAhead Number of ticks to skip ahead for gas optimization
    /// @param calculatedAmountThreshold Minimum amount to receive (for slippage protection)
    /// @return balanceUpdate Change in token0 and token1 balance of the pool
    function swap(
        PoolKey memory poolKey,
        bool isToken1,
        int128 amount,
        SqrtRatio sqrtRatioLimit,
        uint256 skipAhead,
        int256 calculatedAmountThreshold
    ) external payable returns (PoolBalanceUpdate balanceUpdate) {
        balanceUpdate = swap(
            poolKey, isToken1, amount, sqrtRatioLimit, skipAhead, calculatedAmountThreshold, msg.sender
        );
    }

    /// @notice Executes a single-hop swap with no slippage protection
    /// @param poolKey Pool key identifying the pool to swap against
    /// @param isToken1 True if swapping token1, false if swapping token0
    /// @param amount Amount to swap (positive for exact input, negative for exact output)
    /// @param sqrtRatioLimit Price limit for the swap (0 for no limit)
    /// @param skipAhead Number of ticks to skip ahead for gas optimization
    /// @return balanceUpdate Change in token0 and token1 balance of the pool
    function swap(PoolKey memory poolKey, bool isToken1, int128 amount, SqrtRatio sqrtRatioLimit, uint256 skipAhead)
        external
        payable
        returns (PoolBalanceUpdate balanceUpdate)
    {
        balanceUpdate = swap(poolKey, isToken1, amount, sqrtRatioLimit, skipAhead, type(int256).min, msg.sender);
    }

    /// @notice Executes a single-hop swap using RouteNode and TokenAmount structs
    /// @param node Route node containing pool and swap parameters
    /// @param tokenAmount Token and amount to swap
    /// @param calculatedAmountThreshold Minimum amount to receive (for slippage protection)
    /// @return balanceUpdate Change in token0 and token1 balance of the pool
    function swap(RouteNode memory node, TokenAmount memory tokenAmount, int256 calculatedAmountThreshold)
        public
        payable
        returns (PoolBalanceUpdate balanceUpdate)
    {
        balanceUpdate = swap(
            node.poolKey,
            node.poolKey.token1 == tokenAmount.token,
            tokenAmount.amount,
            node.sqrtRatioLimit,
            node.skipAhead,
            calculatedAmountThreshold,
            msg.sender
        );
    }

    /// @notice Executes a multi-hop swap through multiple pools
    /// @param s Swap struct containing the route and initial token amount
    /// @param calculatedAmountThreshold Minimum final amount to receive (for slippage protection)
    /// @return result Array of deltas for each hop in the swap
    function multihopSwap(Swap memory s, int256 calculatedAmountThreshold)
        external
        payable
        returns (PoolBalanceUpdate[] memory result)
    {
        result = abi.decode(
            lock(abi.encode(CALL_TYPE_MULTIHOP_SWAP, msg.sender, s, calculatedAmountThreshold)), (PoolBalanceUpdate[])
        );
    }

    /// @notice Executes multiple multi-hop swaps in a single transaction
    /// @param swaps Array of swap structs, each containing a route and initial token amount
    /// @param calculatedAmountThreshold Minimum total final amount to receive (for slippage protection)
    /// @return results Array of delta arrays, one for each swap
    function multiMultihopSwap(Swap[] memory swaps, int256 calculatedAmountThreshold)
        external
        payable
        returns (PoolBalanceUpdate[][] memory results)
    {
        results = abi.decode(
            lock(abi.encode(CALL_TYPE_MULTI_MULTIHOP_SWAP, msg.sender, swaps, calculatedAmountThreshold)),
            (PoolBalanceUpdate[][])
        );
    }

    /// @notice Error used to return quote values from the quote function
    /// @param balanceUpdate Change in token0 and token1 balance of the pool
    /// @param poolState The state after the swap
    error QuoteReturnValue(PoolBalanceUpdate balanceUpdate, PoolState poolState);

    /// @notice Quotes the result of a swap without executing it
    /// @dev Uses a revert-based mechanism to return the quote without state changes
    /// @param poolKey Pool key identifying the pool to quote against
    /// @param isToken1 True if swapping token1, false if swapping token0
    /// @param amount Amount to swap (positive for exact input, negative for exact output)
    /// @param sqrtRatioLimit Price limit for the swap (0 for no limit)
    /// @param skipAhead Number of ticks to skip ahead for gas optimization
    /// @return balanceUpdate The change in pool balances resulting from the swap
    /// @return stateAfter The state of the pool after the swap is complete
    function quote(PoolKey memory poolKey, bool isToken1, int128 amount, SqrtRatio sqrtRatioLimit, uint256 skipAhead)
        external
        returns (PoolBalanceUpdate balanceUpdate, PoolState stateAfter)
    {
        bytes memory revertData = lockAndExpectRevert(
            abi.encode(
                CALL_TYPE_QUOTE,
                poolKey,
                createSwapParameters({
                    _isToken1: isToken1, _amount: amount, _sqrtRatioLimit: sqrtRatioLimit, _skipAhead: skipAhead
                })
            )
        );

        // check that the sig matches the error data

        bytes4 sig;
        assembly ("memory-safe") {
            sig := mload(add(revertData, 32))
        }
        if (sig == QuoteReturnValue.selector && revertData.length == 68) {
            assembly ("memory-safe") {
                balanceUpdate := mload(add(revertData, 36))
                stateAfter := mload(add(revertData, 68))
            }
        } else {
            assembly ("memory-safe") {
                revert(add(revertData, 32), mload(revertData))
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolKey} from "../../types/poolKey.sol";
import {IExtension} from "../ICore.sol";
import {IExposedStorage} from "../IExposedStorage.sol";
import {Snapshot} from "../../types/snapshot.sol";
import {Observation} from "../../types/observation.sol";

/// @title Oracle Interface
/// @notice Interface for the Ekubo Oracle Extension
/// @dev Records price and liquidity into accumulators enabling a separate contract to compute a manipulation resistant average price and liquidity
interface IOracle is IExposedStorage, IExtension {
    /// @notice Thrown when trying to create a pool that doesn't pair with the native token
    error PairsWithNativeTokenOnly();

    /// @notice Thrown when the pool fee is not zero
    error FeeMustBeZero();

    /// @notice Thrown when the tick spacing is not the maximum allowed value
    error FullRangePoolOnly();

    /// @notice Thrown when querying data for a future timestamp
    error FutureTime();

    /// @notice Thrown when no previous snapshot exists for the given time
    /// @param token The token address
    /// @param time The requested timestamp
    error NoPreviousSnapshotExists(address token, uint256 time);

    /// @notice Thrown when the end time is less than the start time
    error EndTimeLessThanStartTime();

    /// @notice Thrown when timestamps are not provided in sorted order
    error TimestampsNotSorted();

    /// @notice Thrown when zero timestamps are provided to a function that requires at least one
    error ZeroTimestampsProvided();

    /// @notice Gets the pool key for the given token
    /// @dev The only allowed pool key for the given token pairs with the native token
    /// @param token The token address
    /// @return The pool key for the token paired with native token
    function getPoolKey(address token) external view returns (PoolKey memory);

    /// @notice Expands the capacity of the list of snapshots for the given token
    /// @param token The token address
    /// @param minCapacity The minimum capacity required
    /// @return capacity The actual capacity after expansion
    function expandCapacity(address token, uint32 minCapacity) external returns (uint32 capacity);

    /// @notice Finds the snapshot with the greatest timestamp ≤ the given time
    /// @param token The token address
    /// @param time The target timestamp
    /// @return count The total number of snapshots for the token
    /// @return logicalIndex The logical index of the found snapshot
    /// @return snapshot The snapshot data
    function findPreviousSnapshot(address token, uint256 time)
        external
        view
        returns (uint256 count, uint256 logicalIndex, Snapshot snapshot);

    /// @notice Returns cumulative snapshot values at a specific time
    /// @dev Extrapolates values from the most recent snapshot before the given time
    /// @param token The token address
    /// @param atTime The target timestamp
    /// @return secondsPerLiquidityCumulative The cumulative seconds per liquidity at the target time
    /// @return tickCumulative The cumulative tick value at the target time
    function extrapolateSnapshot(address token, uint256 atTime)
        external
        view
        returns (uint160 secondsPerLiquidityCumulative, int64 tickCumulative);

    /// @notice Returns extrapolated snapshots at each of the provided sorted timestamps
    /// @dev Efficiently computes observations for multiple timestamps in a single call
    /// @param token The token address
    /// @param timestamps Array of timestamps in ascending order
    /// @return observations Array of observations corresponding to each timestamp
    function getExtrapolatedSnapshotsForSortedTimestamps(address token, uint256[] memory timestamps)
        external
        view
        returns (Observation[] memory observations);
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {BaseLocker} from "./base/BaseLocker.sol";
import {UsesCore} from "./base/UsesCore.sol";
import {ICore} from "./interfaces/ICore.sol";
import {IOrders} from "./interfaces/IOrders.sol";
import {PayableMulticallable} from "./base/PayableMulticallable.sol";
import {TWAMMLib} from "./libraries/TWAMMLib.sol";
import {ITWAMM} from "./interfaces/extensions/ITWAMM.sol";
import {OrderKey} from "./types/orderKey.sol";
import {computeSaleRate} from "./math/twamm.sol";
import {BaseNonfungibleToken} from "./base/BaseNonfungibleToken.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {NATIVE_TOKEN_ADDRESS} from "./math/constants.sol";
import {FlashAccountantLib} from "./libraries/FlashAccountantLib.sol";

/// @title Ekubo Protocol Orders
/// @author Moody Salem <moody@ekubo.org>
/// @notice Tracks TWAMM (Time-Weighted Average Market Maker) orders in Ekubo Protocol as NFTs
/// @dev Manages long-term orders that execute over time through the TWAMM extension
contract Orders is IOrders, UsesCore, PayableMulticallable, BaseLocker, BaseNonfungibleToken {
    using TWAMMLib for *;
    using FlashAccountantLib for *;

    uint256 private constant CALL_TYPE_CHANGE_SALE_RATE = 0;
    uint256 private constant CALL_TYPE_COLLECT_PROCEEDS = 1;

    /// @notice The TWAMM extension contract that handles order execution
    ITWAMM public immutable TWAMM_EXTENSION;

    /// @notice Constructs the Orders contract
    /// @param core The core contract instance
    /// @param _twamm The TWAMM extension contract
    /// @param owner The owner of the contract (for access control)
    constructor(ICore core, ITWAMM _twamm, address owner) BaseNonfungibleToken(owner) BaseLocker(core) UsesCore(core) {
        TWAMM_EXTENSION = _twamm;
    }

    /// @inheritdoc IOrders
    function mintAndIncreaseSellAmount(OrderKey memory orderKey, uint112 amount, uint112 maxSaleRate)
        public
        payable
        returns (uint256 id, uint112 saleRate)
    {
        id = mint();
        saleRate = increaseSellAmount(id, orderKey, amount, maxSaleRate);
    }

    /// @inheritdoc IOrders
    function increaseSellAmount(uint256 id, OrderKey memory orderKey, uint128 amount, uint112 maxSaleRate)
        public
        payable
        authorizedForNft(id)
        returns (uint112 saleRate)
    {
        uint256 realStart = FixedPointMathLib.max(block.timestamp, orderKey.config.startTime());

        unchecked {
            if (orderKey.config.endTime() <= realStart) {
                revert OrderAlreadyEnded();
            }

            saleRate = uint112(computeSaleRate(amount, uint32(orderKey.config.endTime() - realStart)));

            if (saleRate > maxSaleRate) {
                revert MaxSaleRateExceeded();
            }
        }

        lock(abi.encode(CALL_TYPE_CHANGE_SALE_RATE, msg.sender, id, orderKey, saleRate));
    }

    /// @inheritdoc IOrders
    function decreaseSaleRate(uint256 id, OrderKey memory orderKey, uint112 saleRateDecrease, address recipient)
        public
        payable
        authorizedForNft(id)
        returns (uint112 refund)
    {
        refund = uint112(
            uint256(
                -abi.decode(
                    lock(
                        abi.encode(
                            CALL_TYPE_CHANGE_SALE_RATE, recipient, id, orderKey, -int256(uint256(saleRateDecrease))
                        )
                    ),
                    (int256)
                )
            )
        );
    }

    /// @inheritdoc IOrders
    function decreaseSaleRate(uint256 id, OrderKey memory orderKey, uint112 saleRateDecrease)
        external
        payable
        returns (uint112 refund)
    {
        refund = decreaseSaleRate(id, orderKey, saleRateDecrease, msg.sender);
    }

    /// @inheritdoc IOrders
    function collectProceeds(uint256 id, OrderKey memory orderKey, address recipient)
        public
        payable
        authorizedForNft(id)
        returns (uint128 proceeds)
    {
        proceeds = abi.decode(lock(abi.encode(CALL_TYPE_COLLECT_PROCEEDS, id, orderKey, recipient)), (uint128));
    }

    /// @inheritdoc IOrders
    function collectProceeds(uint256 id, OrderKey memory orderKey) external payable returns (uint128 proceeds) {
        proceeds = collectProceeds(id, orderKey, msg.sender);
    }

    /// @inheritdoc IOrders
    function executeVirtualOrdersAndGetCurrentOrderInfo(uint256 id, OrderKey memory orderKey)
        external
        returns (uint112 saleRate, uint256 amountSold, uint256 remainingSellAmount, uint128 purchasedAmount)
    {
        (saleRate, amountSold, remainingSellAmount, purchasedAmount) =
            TWAMM_EXTENSION.executeVirtualOrdersAndGetCurrentOrderInfo(address(this), bytes32(id), orderKey);
    }

    /// @notice Handles lock callback data for order operations
    /// @dev Internal function that processes different types of order operations
    /// @param data Encoded operation data
    /// @return result Encoded result data
    function handleLockData(uint256, bytes memory data) internal override returns (bytes memory result) {
        uint256 callType = abi.decode(data, (uint256));

        if (callType == CALL_TYPE_CHANGE_SALE_RATE) {
            (, address recipientOrPayer, uint256 id, OrderKey memory orderKey, int256 saleRateDelta) =
                abi.decode(data, (uint256, address, uint256, OrderKey, int256));

            int256 amount =
                CORE.updateSaleRate(TWAMM_EXTENSION, bytes32(id), orderKey, SafeCastLib.toInt112(saleRateDelta));

            if (amount != 0) {
                address sellToken = orderKey.sellToken();
                if (saleRateDelta > 0) {
                    if (sellToken == NATIVE_TOKEN_ADDRESS) {
                        SafeTransferLib.safeTransferETH(address(ACCOUNTANT), uint256(amount));
                    } else {
                        ACCOUNTANT.payFrom(recipientOrPayer, sellToken, uint256(amount));
                    }
                } else {
                    unchecked {
                        // we know amount will never exceed the uint128 type because of limitations on sale rate (fixed point 80.32) and duration (uint32)
                        ACCOUNTANT.withdraw(sellToken, recipientOrPayer, uint128(uint256(-amount)));
                    }
                }
            }

            result = abi.encode(amount);
        } else if (callType == CALL_TYPE_COLLECT_PROCEEDS) {
            (, uint256 id, OrderKey memory orderKey, address recipient) =
                abi.decode(data, (uint256, uint256, OrderKey, address));

            uint128 proceeds = CORE.collectProceeds(TWAMM_EXTENSION, bytes32(id), orderKey);

            if (proceeds != 0) {
                ACCOUNTANT.withdraw(orderKey.buyToken(), recipient, proceeds);
            }

            result = abi.encode(proceeds);
        } else {
            revert();
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {ILocker, IFlashAccountant} from "../interfaces/IFlashAccountant.sol";

/// @title Base Locker
/// @notice Abstract base contract for contracts that need to interact with the flash accountant
/// @dev Provides locking functionality and token transfer utilities
abstract contract BaseLocker is ILocker {
    /// @notice Thrown when a function is called by an address other than the accountant
    error BaseLockerAccountantOnly();

    /// @notice The flash accountant contract that manages locks and token transfers
    IFlashAccountant internal immutable ACCOUNTANT;

    /// @notice Constructs the BaseLocker with a flash accountant
    /// @param _accountant The flash accountant contract
    constructor(IFlashAccountant _accountant) {
        ACCOUNTANT = _accountant;
    }

    /// CALLBACK HANDLERS

    /// @inheritdoc ILocker
    function locked_6416899205(uint256 id) external {
        if (msg.sender != address(ACCOUNTANT)) revert BaseLockerAccountantOnly();

        bytes memory data = msg.data[36:];

        bytes memory result = handleLockData(id, data);

        assembly ("memory-safe") {
            // raw return whatever the handler sent
            return(add(result, 32), mload(result))
        }
    }

    /// INTERNAL FUNCTIONS

    /// @notice Acquires a lock and executes the provided data
    /// @dev Internal function that calls the accountant's lock function
    /// @param data The data to execute within the lock
    /// @return result The result of the lock execution
    function lock(bytes memory data) internal returns (bytes memory result) {
        address target = address(ACCOUNTANT);

        assembly ("memory-safe") {
            // We will store result where the free memory pointer is now, ...
            result := mload(0x40)

            // But first use it to store the calldata

            // Selector of lock()
            mstore(result, shl(224, 0xf83d08ba))

            // We only copy the data, not the length, because the length is read from the calldata size
            let len := mload(data)
            mcopy(add(result, 4), add(data, 32), len)

            // If the call failed, pass through the revert
            if iszero(call(gas(), target, 0, result, add(len, 4), 0, 0)) {
                returndatacopy(result, 0, returndatasize())
                revert(result, returndatasize())
            }

            // Copy the entire return data into the space where the result is pointing
            mstore(result, returndatasize())
            returndatacopy(add(result, 32), 0, returndatasize())

            // Update the free memory pointer to be after the end of the data, aligned to the next 32 byte word
            mstore(0x40, and(add(add(result, add(32, returndatasize())), 31), not(31)))
        }
    }

    /// @notice Thrown when a lock was expected to revert but didn't
    error ExpectedRevertWithinLock();

    /// @notice Acquires a lock expecting it to revert and returns the revert data
    /// @dev Used for quote functions that use reverts to return data
    /// @param data The data to execute within the lock
    /// @return result The revert data from the lock execution
    function lockAndExpectRevert(bytes memory data) internal returns (bytes memory result) {
        address target = address(ACCOUNTANT);

        assembly ("memory-safe") {
            // We will store result where the free memory pointer is now, ...
            result := mload(0x40)

            // But first use it to store the calldata

            // Selector of lock()
            mstore(result, shl(224, 0xf83d08ba))

            // We only copy the data, not the length, because the length is read from the calldata size
            let len := mload(data)
            mcopy(add(result, 4), add(data, 32), len)

            // If the call succeeded, revert with ExpectedRevertWithinLock.selector
            if call(gas(), target, 0, result, add(len, 4), 0, 0) {
                mstore(0, shl(224, 0x4c816e2b))
                revert(0, 4)
            }

            // Copy the entire revert data into the space where the result is pointing
            mstore(result, returndatasize())
            returndatacopy(add(result, 32), 0, returndatasize())

            // Update the free memory pointer to be after the end of the data, aligned to the next 32 byte word
            mstore(0x40, and(add(add(result, add(32, returndatasize())), 31), not(31)))
        }
    }

    /// @notice Handles the execution of lock data
    /// @dev Must be implemented by derived contracts to define lock behavior
    /// @param id The lock ID
    /// @param data The data to process within the lock
    /// @return result The result of processing the lock data
    function handleLockData(uint256 id, bytes memory data) internal virtual returns (bytes memory result);
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {NATIVE_TOKEN_ADDRESS} from "../math/constants.sol";
import {IFlashAccountant} from "../interfaces/IFlashAccountant.sol";
import {Locker} from "../types/locker.sol";

/// @title FlashAccountant
/// @notice Abstract contract that provides flash loan accounting functionality using transient storage
/// @dev This contract manages debt tracking for flash loans, allowing users to borrow tokens temporarily
///      and ensuring all debts are settled before the transaction completes. Uses transient storage
///      for gas-efficient temporary state management within a single transaction.
abstract contract FlashAccountant is IFlashAccountant {
    // These offsets are selected so that they do not accidentally overlap with any other base contract's use of transient storage

    /// @dev Transient storage slot for tracking the current locker ID and address
    /// @dev The stored ID is kept as id + 1 to facilitate the NotLocked check (zero means unlocked)
    /// @dev Generated using: cast keccak "FlashAccountant#CURRENT_LOCKER_SLOT"
    uint256 private constant _CURRENT_LOCKER_SLOT = 0x07cc7f5195d862f505d6b095c82f92e00cfc1766f5bca4383c28dc5fca1555fd;

    /// @dev Transient storage offset for tracking token debts for each locker
    /// @dev Generated using: cast keccak "FlashAccountant#_DEBT_LOCKER_TOKEN_ADDRESS_OFFSET"
    uint256 private constant _DEBT_LOCKER_TOKEN_ADDRESS_OFFSET =
        0x753dfe4b4dfb3ff6c11bbf6a97f3c094e91c003ce904a55cc5662fbad220f599;

    /// @dev Transient storage offset for tracking the count of tokens with non-zero debt for each locker
    /// @dev Generated using: cast keccak "FlashAccountant#NONZERO_DEBT_COUNT_OFFSET"
    uint256 private constant _NONZERO_DEBT_COUNT_OFFSET =
        0x7772acfd7e0f66ebb20a058830296c3dc1301b111d23348e1c961d324223190d;

    /// @dev Transient storage offset for tracking token balances during payment operations
    /// @dev Generated using: cast keccak "FlashAccountant#_PAYMENT_TOKEN_ADDRESS_OFFSET"
    uint256 private constant _PAYMENT_TOKEN_ADDRESS_OFFSET =
        0x6747da56dbd05b26a7ecd2a0106781585141cf07098ad54c0e049e4e86dccb8c;

    /// @notice Gets the current locker information from transient storage
    /// @dev Reverts with NotLocked() if no lock is currently active
    /// @return locker The current locker containing both id and address
    function _getLocker() internal view returns (Locker locker) {
        assembly ("memory-safe") {
            locker := tload(_CURRENT_LOCKER_SLOT)

            if iszero(locker) {
                // cast sig "NotLocked()"
                mstore(0, shl(224, 0x1834e265))
                revert(0, 4)
            }
        }
    }

    /// @notice Gets the current locker information and ensures the caller is the locker
    /// @dev Reverts with LockerOnly() if the caller is not the current locker
    /// @return locker The current locker containing both id and address (address must be msg.sender)
    function _requireLocker() internal view returns (Locker locker) {
        locker = _getLocker();
        if (locker.addr() != msg.sender) revert LockerOnly();
    }

    /// @notice Updates the debt tracking for a specific locker and token
    /// @dev We assume debtChange cannot exceed a 128 bits value, even though it uses a int256 container.
    ///      This must be enforced at the places it is called for this contract's safety.
    ///      Negative values erase debt, positive values add debt.
    ///      Updates the non-zero debt count when debt transitions between zero and non-zero states.
    /// @param id The locker ID to update debt for
    /// @param token The token address to update debt for
    /// @param debtChange The change in debt (negative to reduce, positive to increase)
    function _accountDebt(uint256 id, address token, int256 debtChange) internal {
        assembly ("memory-safe") {
            let deltaSlot := add(_DEBT_LOCKER_TOKEN_ADDRESS_OFFSET, add(shl(160, id), token))
            let current := tload(deltaSlot)

            // we know this never overflows because debtChange is only ever derived from 128 bit values in inheriting contracts
            let next := add(current, debtChange)

            let countChange := sub(iszero(current), iszero(next))

            if countChange {
                let nzdCountSlot := add(id, _NONZERO_DEBT_COUNT_OFFSET)
                tstore(nzdCountSlot, add(tload(nzdCountSlot), countChange))
            }

            tstore(deltaSlot, next)
        }
    }

    /// @notice Updates the debt tracking for a specific locker and pair of tokens in a single operation
    /// @dev Optimized version that updates both tokens' debts and performs a single tload/tstore on the non-zero debt count.
    ///      Individual token debt slots are still updated separately, but the non-zero debt count is only loaded/stored once.
    ///      We assume debtChange values cannot exceed 128 bits. This must be enforced at the places it is called for this contract's safety.
    ///      Negative values erase debt, positive values add debt.
    /// @param id The locker ID to update debt for
    /// @param tokenA The first token address
    /// @param tokenB The second token address
    /// @param debtChangeA The change in debt for tokenA (negative to reduce, positive to increase)
    /// @param debtChangeB The change in debt for tokenB (negative to reduce, positive to increase)
    function _updatePairDebt(uint256 id, address tokenA, address tokenB, int256 debtChangeA, int256 debtChangeB)
        internal
    {
        assembly ("memory-safe") {
            let nzdCountChange := 0

            // Update token0 debt if there's a change
            if debtChangeA {
                let deltaSlotA := add(_DEBT_LOCKER_TOKEN_ADDRESS_OFFSET, add(shl(160, id), tokenA))
                let currentA := tload(deltaSlotA)
                let nextA := add(currentA, debtChangeA)

                nzdCountChange := sub(iszero(currentA), iszero(nextA))

                tstore(deltaSlotA, nextA)
            }

            if debtChangeB {
                let deltaSlotB := add(_DEBT_LOCKER_TOKEN_ADDRESS_OFFSET, add(shl(160, id), tokenB))
                let currentB := tload(deltaSlotB)
                let nextB := add(currentB, debtChangeB)

                nzdCountChange := add(nzdCountChange, sub(iszero(currentB), iszero(nextB)))

                tstore(deltaSlotB, nextB)
            }

            // Update non-zero debt count only if it changed
            if nzdCountChange {
                let nzdCountSlot := add(id, _NONZERO_DEBT_COUNT_OFFSET)
                tstore(nzdCountSlot, add(tload(nzdCountSlot), nzdCountChange))
            }
        }
    }

    /// @inheritdoc IFlashAccountant
    function updateDebt() external {
        if (msg.data.length != 20) {
            revert UpdateDebtMessageLength();
        }

        uint256 id = _getLocker().id();
        int256 delta;
        assembly ("memory-safe") {
            delta := signextend(15, shr(128, calldataload(4)))
        }
        _accountDebt(id, msg.sender, delta);
    }

    /// @inheritdoc IFlashAccountant
    function lock() external {
        assembly ("memory-safe") {
            let current := tload(_CURRENT_LOCKER_SLOT)

            let id := shr(160, current)

            // store the count
            tstore(_CURRENT_LOCKER_SLOT, or(shl(160, add(id, 1)), caller()))

            let free := mload(0x40)
            // Prepare call to locked_(uint256) -> selector 0
            mstore(free, 0)
            mstore(add(free, 4), id) // ID argument

            calldatacopy(add(free, 36), 4, sub(calldatasize(), 4))

            // Call the original caller with the packed data
            let success := call(gas(), caller(), 0, free, add(calldatasize(), 32), 0, 0)

            // Pass through the error on failure
            if iszero(success) {
                returndatacopy(free, 0, returndatasize())
                revert(free, returndatasize())
            }

            // Undo the "locker" state changes
            tstore(_CURRENT_LOCKER_SLOT, current)

            // Check if something is nonzero
            let nonzeroDebtCount := tload(add(_NONZERO_DEBT_COUNT_OFFSET, id))
            if nonzeroDebtCount {
                // cast sig "DebtsNotZeroed(uint256)"
                mstore(0x00, 0x9731ba37)
                mstore(0x20, id)
                revert(0x1c, 0x24)
            }

            // Directly return whatever the subcall returned
            returndatacopy(free, 0, returndatasize())
            return(free, returndatasize())
        }
    }

    /// @inheritdoc IFlashAccountant
    function forward(address to) external {
        Locker locker = _requireLocker();

        // update this lock's locker to the forwarded address for the duration of the forwarded
        // call, meaning only the forwarded address can update state
        assembly ("memory-safe") {
            tstore(_CURRENT_LOCKER_SLOT, or(shl(160, shr(160, locker)), to))

            let free := mload(0x40)

            // Prepare call to forwarded_2374103877(bytes32) -> selector 0x01
            mstore(free, shl(224, 1))
            mstore(add(free, 4), locker)

            calldatacopy(add(free, 36), 36, sub(calldatasize(), 36))

            // Call the forwardee with the packed data
            let success := call(gas(), to, 0, free, calldatasize(), 0, 0)

            // Pass through the error on failure
            if iszero(success) {
                returndatacopy(free, 0, returndatasize())
                revert(free, returndatasize())
            }

            tstore(_CURRENT_LOCKER_SLOT, locker)

            // Directly return whatever the subcall returned
            returndatacopy(free, 0, returndatasize())
            return(free, returndatasize())
        }
    }

    /// @inheritdoc IFlashAccountant
    function startPayments() external {
        assembly ("memory-safe") {
            // 0-52 are used for the balanceOf calldata
            mstore(20, address()) // Store the `account` argument.
            mstore(0, 0x70a08231000000000000000000000000) // `balanceOf(address)`.

            let free := mload(0x40)

            for { let i := 4 } lt(i, calldatasize()) { i := add(i, 32) } {
                // clean upper 96 bits of the token argument at i
                let token := shr(96, shl(96, calldataload(i)))

                let returnLocation := add(free, sub(i, 4))

                let success := staticcall(gas(), token, 0x10, 0x24, returnLocation, 0x20)

                let tokenBalance :=
                    mul(
                        mload(returnLocation),
                        and(
                            gt(returndatasize(), 0x1f), // At least 32 bytes returned.
                            success
                        )
                    )

                tstore(add(_PAYMENT_TOKEN_ADDRESS_OFFSET, token), add(tokenBalance, success))
            }

            return(free, sub(calldatasize(), 4))
        }
    }

    /// @inheritdoc IFlashAccountant
    function completePayments() external {
        uint256 id = _getLocker().id();

        assembly ("memory-safe") {
            let paymentAmounts := mload(0x40)
            let nzdCountChange := 0

            for { let i := 4 } lt(i, calldatasize()) { i := add(i, 32) } {
                let token := shr(96, shl(96, calldataload(i)))

                let offset := add(_PAYMENT_TOKEN_ADDRESS_OFFSET, token)
                let lastBalance := tload(offset)
                tstore(offset, 0)

                mstore(20, address()) // Store the `account` argument.
                mstore(0, 0x70a08231000000000000000000000000) // `balanceOf(address)`.

                let currentBalance :=
                    mul( // The arguments of `mul` are evaluated from right to left.
                        mload(0),
                        and( // The arguments of `and` are evaluated from right to left.
                            gt(returndatasize(), 0x1f), // At least 32 bytes returned.
                            staticcall(gas(), token, 0x10, 0x24, 0, 0x20)
                        )
                    )

                let payment :=
                    mul(
                        and(gt(lastBalance, 0), not(lt(currentBalance, lastBalance))),
                        sub(currentBalance, sub(lastBalance, 1))
                    )

                // We never expect tokens to have this much total supply
                if shr(128, payment) {
                    // cast sig "PaymentOverflow()"
                    mstore(0x00, 0x9cac58ca)
                    revert(0x1c, 4)
                }

                mstore(add(paymentAmounts, mul(16, div(i, 32))), shl(128, payment))

                if payment {
                    let deltaSlot := add(_DEBT_LOCKER_TOKEN_ADDRESS_OFFSET, add(shl(160, id), token))
                    let current := tload(deltaSlot)

                    // never overflows because of the payment overflow check that bounds payment to 128 bits
                    let next := sub(current, payment)

                    nzdCountChange := add(nzdCountChange, sub(iszero(current), iszero(next)))

                    tstore(deltaSlot, next)
                }
            }

            // Update nzdCountSlot only once if there were any changes
            if nzdCountChange {
                let nzdCountSlot := add(id, _NONZERO_DEBT_COUNT_OFFSET)
                tstore(nzdCountSlot, add(tload(nzdCountSlot), nzdCountChange))
            }

            return(paymentAmounts, mul(16, div(calldatasize(), 32)))
        }
    }

    /// @inheritdoc IFlashAccountant
    function withdraw() external {
        uint256 id = _requireLocker().id();

        assembly ("memory-safe") {
            let nzdCountChange := 0

            // Process each withdrawal entry
            for { let i := 4 } lt(i, calldatasize()) { i := add(i, 56) } {
                let token := shr(96, calldataload(i))
                let recipient := shr(96, calldataload(add(i, 20)))
                let amount := shr(128, calldataload(add(i, 40)))

                if amount {
                    // Update debt tracking without updating nzdCountSlot yet
                    let deltaSlot := add(_DEBT_LOCKER_TOKEN_ADDRESS_OFFSET, add(shl(160, id), token))
                    let current := tload(deltaSlot)
                    let next := add(current, amount)

                    nzdCountChange := add(nzdCountChange, sub(iszero(current), iszero(next)))

                    tstore(deltaSlot, next)

                    // Perform the transfer of the withdrawn asset
                    // Note that these calls can re-enter and even relock with the same ID
                    // However the nzdCountChange is always applied as a delta at the end, meaning we load the latest value before updating it,
                    // so it's safe from re-entry
                    switch token
                    case 0 {
                        let success := call(gas(), recipient, amount, 0, 0, 0, 0)
                        if iszero(success) {
                            // cast sig "ETHTransferFailed()"
                            mstore(0x00, 0xb12d13eb)
                            revert(0x1c, 4)
                        }
                    }
                    default {
                        mstore(0x14, recipient)
                        mstore(0x34, amount)
                        mstore(0x00, 0xa9059cbb000000000000000000000000)
                        let success := call(gas(), token, 0, 0x10, 0x44, 0x00, 0x20)
                        if iszero(and(eq(mload(0x00), 1), success)) {
                            if iszero(lt(or(iszero(extcodesize(token)), returndatasize()), success)) {
                                mstore(0x00, 0x90b8ec18) // `TransferFailed()`.
                                revert(0x1c, 0x04)
                            }
                        }
                    }
                }
            }

            // Update nzdCountSlot only once if there were any changes
            if nzdCountChange {
                let nzdCountSlot := add(id, _NONZERO_DEBT_COUNT_OFFSET)
                tstore(nzdCountSlot, add(tload(nzdCountSlot), nzdCountChange))
            }

            // we return from assembly so as to prevent solidity from accessing the free memory pointer after we have written into it
            return(0, 0)
        }
    }

    /// @inheritdoc IFlashAccountant
    receive() external payable {
        uint256 id = _getLocker().id();

        // Note because we use msg.value here, this contract can never be multicallable, i.e. it should never expose the ability
        //      to delegatecall itself more than once in a single call
        unchecked {
            // We assume msg.value will never exceed type(uint128).max, so this should never cause an overflow/underflow of debt
            _accountDebt(id, NATIVE_TOKEN_ADDRESS, -int256(msg.value));
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {CallPoints} from "../types/callPoints.sol";
import {PoolKey} from "../types/poolKey.sol";
import {PositionId} from "../types/positionId.sol";
import {SqrtRatio, MIN_SQRT_RATIO, MAX_SQRT_RATIO} from "../types/sqrtRatio.sol";
import {ICore, IExtension} from "../interfaces/ICore.sol";
import {ITWAMM} from "../interfaces/extensions/ITWAMM.sol";
import {TimeInfo, createTimeInfo} from "../types/timeInfo.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {ExposedStorage} from "../base/ExposedStorage.sol";
import {TWAMMStorageLayout} from "../libraries/TWAMMStorageLayout.sol";
import {StorageSlot} from "../types/storageSlot.sol";
import {BaseExtension} from "../base/BaseExtension.sol";
import {BaseForwardee} from "../base/BaseForwardee.sol";
import {PoolState} from "../types/poolState.sol";
import {PoolBalanceUpdate, createPoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";
import {TwammPoolState, createTwammPoolState} from "../types/twammPoolState.sol";
import {OrderKey} from "../types/orderKey.sol";
import {OrderConfig} from "../types/orderConfig.sol";
import {OrderState, createOrderState} from "../types/orderState.sol";
import {searchForNextInitializedTime, flipTime} from "../math/timeBitmap.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {FeesPerLiquidity} from "../types/feesPerLiquidity.sol";
import {computeFee} from "../math/fee.sol";
import {
    computeNextSqrtRatio,
    computeAmountFromSaleRate,
    computeRewardAmount,
    addSaleRateDelta
} from "../math/twamm.sol";
import {isTimeValid, MAX_ABS_VALUE_SALE_RATE_DELTA} from "../math/time.sol";
import {PoolId} from "../types/poolId.sol";
import {OrderId} from "../types/orderId.sol";
import {SwapParameters, createSwapParameters} from "../types/swapParameters.sol";
import {Locker} from "../types/locker.sol";
import {LibBit} from "solady/utils/LibBit.sol";

/// @notice Returns the call points configuration for the TWAMM extension
/// @dev Specifies which hooks TWAMM needs to execute virtual orders and manage DCA functionality
/// @return The call points configuration for TWAMM functionality
function twammCallPoints() pure returns (CallPoints memory) {
    return CallPoints({
        beforeInitializePool: false,
        afterInitializePool: true,
        beforeUpdatePosition: true,
        afterUpdatePosition: false,
        beforeSwap: true,
        afterSwap: false,
        beforeCollectFees: true,
        afterCollectFees: false
    });
}

/// @title Ekubo TWAMM (Time-Weighted Average Market Maker)
/// @author Moody Salem <moody@ekubo.org>
/// @notice Extension for Ekubo Protocol that enables creation of DCA (Dollar Cost Averaging) orders that are executed over time
/// @dev Implements virtual order execution that spreads trades over time periods to reduce price impact and provide better execution
contract TWAMM is ITWAMM, ExposedStorage, BaseExtension, BaseForwardee {
    using CoreLib for *;

    constructor(ICore core) BaseExtension(core) BaseForwardee(core) {}

    /// @notice Emits an event for virtual order execution
    /// @dev Emits an event for the virtual order execution. Assumes that saleRateToken0 and saleRateToken1 are <= type(uint112).max
    /// @param poolId The unique identifier for the pool
    /// @param saleRateToken0 The sale rate for token0 orders
    /// @param saleRateToken1 The sale rate for token1 orders
    function _emitVirtualOrdersExecuted(PoolId poolId, uint256 saleRateToken0, uint256 saleRateToken1) internal {
        assembly ("memory-safe") {
            // by writing it backwards, we overwrite only the empty bits with each subsequent write
            // 28-60, only 46-60 can be non-zero
            mstore(28, saleRateToken1)
            // 14-46, only 32-46 can be non-zero
            mstore(14, saleRateToken0)
            mstore(0, poolId)

            log0(0, 60)
        }
    }

    /// @inheritdoc ITWAMM
    function getRewardRateInside(PoolId poolId, OrderConfig config) public view returns (uint256 result) {
        if (block.timestamp >= config.endTime()) {
            uint256 offset = LibBit.rawToUint(!config.isToken1());
            uint256 rewardRateStart =
                uint256(TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, config.startTime()).add(offset).load());

            uint256 rewardRateEnd =
                uint256(TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, config.endTime()).add(offset).load());

            unchecked {
                result = rewardRateEnd - rewardRateStart;
            }
        } else if (block.timestamp > config.startTime()) {
            uint256 offset = LibBit.rawToUint(!config.isToken1());

            //  note that we check gt because if it's equal to start time, then the reward rate inside is necessarily 0
            uint256 rewardRateStart =
                uint256(TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, config.startTime()).add(offset).load());
            uint256 rewardRateCurrent = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).add(offset).load());

            unchecked {
                result = rewardRateCurrent - rewardRateStart;
            }
        } else {
            // less than or equal to start time
            // returns 0
        }
    }

    /// @notice Safely adds a change to a sale rate delta with overflow protection
    /// @dev Ensures the resulting sale rate delta doesn't exceed the maximum allowed value
    /// @param saleRateDelta The current sale rate delta
    /// @param saleRateDeltaChange The change to apply to the sale rate delta
    /// @return saleRateDeltaNext The new sale rate delta after applying the change
    function _addConstrainSaleRateDelta(int112 saleRateDelta, int256 saleRateDeltaChange)
        internal
        pure
        returns (int112 saleRateDeltaNext)
    {
        int256 result = int256(saleRateDelta) + saleRateDeltaChange;

        // checked addition, no overflow of int112 type
        if (FixedPointMathLib.abs(result) > MAX_ABS_VALUE_SALE_RATE_DELTA) {
            revert MaxSaleRateDeltaPerTime();
        }

        // we know cast is safe because abs(result) is less than MAX_ABS_VALUE_SALE_RATE_DELTA which fits in a int112
        saleRateDeltaNext = int112(result);
    }

    /// @notice Updates time-specific information for TWAMM orders
    /// @dev Manages the sale rate deltas and order counts for a specific time point
    /// @param poolId The unique identifier for the pool
    /// @param time The timestamp to update
    /// @param saleRateDelta The change in sale rate for this time
    /// @param isToken1 True if updating token1 sale rate, false for token0
    /// @param numOrdersChange The change in number of orders referencing this time
    function _updateTime(PoolId poolId, uint256 time, int256 saleRateDelta, bool isToken1, int256 numOrdersChange)
        internal
    {
        TimeInfo timeInfo = TimeInfo.wrap(TWAMMStorageLayout.poolTimeInfosSlot(poolId, time).load());
        (uint32 numOrders, int112 saleRateDeltaToken0, int112 saleRateDeltaToken1) = timeInfo.parse();

        // note we assume this will never overflow, since it would require 2**32 separate orders to be placed
        uint32 numOrdersNext;
        assembly ("memory-safe") {
            numOrdersNext := add(numOrders, numOrdersChange)
            if gt(numOrdersNext, 0xffffffff) {
                // cast sig "TimeNumOrdersOverflow()"
                mstore(0, shl(224, 0x6916a952))
                revert(0, 4)
            }
        }

        bool flip = (numOrders == 0) != (numOrdersNext == 0);

        // write the poolRewardRatesBefore[poolId][time] = (1,1) if any orders still reference the time, or write (0,0) otherwise
        // we assume `_updateTime` is being called only for times that are greater than block.timestamp, i.e. have not been crossed yet
        // this reduces the cost of crossing that timestamp to a warm write instead of a cold write
        if (flip) {
            bytes32 zeroNumOrders = bytes32(LibBit.rawToUint(numOrders == 0));

            TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, time).storeTwo(zeroNumOrders, zeroNumOrders);

            flipTime(TWAMMStorageLayout.poolInitializedTimesBitmapSlot(poolId), time);
        }

        if (isToken1) {
            saleRateDeltaToken1 = _addConstrainSaleRateDelta(saleRateDeltaToken1, saleRateDelta);
        } else {
            saleRateDeltaToken0 = _addConstrainSaleRateDelta(saleRateDeltaToken0, saleRateDelta);
        }

        TWAMMStorageLayout.poolTimeInfosSlot(poolId, time)
            .store(TimeInfo.unwrap(createTimeInfo(numOrdersNext, saleRateDeltaToken0, saleRateDeltaToken1)));
    }

    /// @notice Returns the call points configuration for this extension
    /// @dev Overrides the base implementation to return TWAMM-specific call points
    /// @return The call points configuration
    function getCallPoints() internal pure override returns (CallPoints memory) {
        return twammCallPoints();
    }

    ///////////////////////// Callbacks /////////////////////////

    function handleForwardData(Locker original, bytes memory data) internal override returns (bytes memory result) {
        unchecked {
            uint256 callType = abi.decode(data, (uint256));
            address owner = original.addr();

            if (callType == 0) {
                (, bytes32 salt, OrderKey memory orderKey, int112 saleRateDelta) =
                    abi.decode(data, (uint256, bytes32, OrderKey, int112));

                (uint64 startTime, uint64 endTime) = (orderKey.config.startTime(), orderKey.config.endTime());

                if (endTime <= block.timestamp) revert OrderAlreadyEnded();

                if (
                    !isTimeValid(block.timestamp, startTime) || !isTimeValid(block.timestamp, endTime)
                        || startTime >= endTime
                ) {
                    revert InvalidTimestamps();
                }

                PoolKey memory poolKey = orderKey.toPoolKey(address(this));
                PoolId poolId = poolKey.toPoolId();
                _executeVirtualOrdersFromWithinLock(poolKey, poolId);

                OrderId orderId = orderKey.toOrderId();

                StorageSlot orderStateSlot =
                    TWAMMStorageLayout.orderStateSlotFollowedByOrderRewardRateSnapshotSlot(owner, salt, orderId);

                StorageSlot orderRewardRateSnapshotSlot = orderStateSlot.next();

                OrderState order = OrderState.wrap(orderStateSlot.load());
                uint256 rewardRateSnapshot = uint256(orderRewardRateSnapshotSlot.load());

                uint256 rewardRateInside = getRewardRateInside(poolId, orderKey.config);

                (uint32 lastUpdateTime, uint112 saleRate, uint112 amountSold) = order.parse();

                uint256 purchasedAmount = computeRewardAmount(rewardRateInside - rewardRateSnapshot, saleRate);

                uint256 saleRateNext = addSaleRateDelta(saleRate, saleRateDelta);

                uint256 rewardRateSnapshotAdjusted;
                int256 numOrdersChange;
                assembly ("memory-safe") {
                    rewardRateSnapshotAdjusted := mul(
                        sub(rewardRateInside, div(shl(128, purchasedAmount), saleRateNext)),
                        // if saleRateNext is zero, write 0 for the reward rate snapshot adjusted
                        iszero(iszero(saleRateNext))
                    )

                    // if current is zero, and next is zero, then 1-1 = 0
                    // if current is nonzero, and next is nonzero, then 0-0 = 0
                    // if current is zero, and next is nonzero, then we get 1-0 = 1
                    // if current is nonzero, and next is zero, then we get 0-1 = -1 = (type(uint256).max)
                    numOrdersChange := sub(iszero(saleRate), iszero(saleRateNext))
                }

                orderStateSlot.store(
                    OrderState.unwrap(
                        createOrderState({
                            _lastUpdateTime: uint32(block.timestamp),
                            _saleRate: uint112(saleRateNext),
                            _amountSold: uint112(
                                amountSold
                                    + computeAmountFromSaleRate({
                                        saleRate: saleRate,
                                        duration: FixedPointMathLib.min(
                                            uint32(block.timestamp) - lastUpdateTime,
                                            uint32(uint64(block.timestamp) - startTime)
                                        ),
                                        roundUp: false
                                    })
                            )
                        })
                    )
                );
                orderRewardRateSnapshotSlot.store(bytes32(rewardRateSnapshotAdjusted));

                bool isToken1 = orderKey.config.isToken1();

                if (block.timestamp < startTime) {
                    _updateTime(poolId, startTime, saleRateDelta, isToken1, numOrdersChange);
                    _updateTime(poolId, endTime, -int256(saleRateDelta), isToken1, numOrdersChange);
                } else {
                    // we know block.timestamp < orderKey.endTime because we validate that first
                    // and we know the order is active, so we have to apply its delta to the current pool state
                    StorageSlot currentStateSlot = TWAMMStorageLayout.twammPoolStateSlot(poolId);
                    TwammPoolState currentState = TwammPoolState.wrap(currentStateSlot.load());
                    (uint32 lastTime, uint112 rate0, uint112 rate1) = currentState.parse();

                    if (isToken1) {
                        currentState = createTwammPoolState({
                            _lastVirtualOrderExecutionTime: lastTime,
                            _saleRateToken0: rate0,
                            _saleRateToken1: uint112(addSaleRateDelta(rate1, saleRateDelta))
                        });
                    } else {
                        currentState = createTwammPoolState({
                            _lastVirtualOrderExecutionTime: lastTime,
                            _saleRateToken0: uint112(addSaleRateDelta(rate0, saleRateDelta)),
                            _saleRateToken1: rate1
                        });
                    }

                    currentStateSlot.store(TwammPoolState.unwrap(currentState));

                    // only update the end time
                    _updateTime(poolId, endTime, -int256(saleRateDelta), isToken1, numOrdersChange);
                }

                // we know this will fit in a uint32 because otherwise isValidTime would fail for the end time
                uint256 durationRemaining = endTime - FixedPointMathLib.max(block.timestamp, startTime);

                // the amount required for executing at the next sale rate for the remaining duration of the order
                uint256 amountRequired =
                    computeAmountFromSaleRate({saleRate: saleRateNext, duration: durationRemaining, roundUp: true});

                // subtract the remaining sell amount to get the delta
                int256 amountDelta;

                uint256 remainingSellAmount =
                    computeAmountFromSaleRate({saleRate: saleRate, duration: durationRemaining, roundUp: true});

                assembly ("memory-safe") {
                    amountDelta := sub(amountRequired, remainingSellAmount)
                }

                // user is withdrawing tokens, so they need to pay a fee to the liquidity providers
                if (amountDelta < 0) {
                    // negation and downcast will never overflow, since max sale rate times max duration is at most type(uint112).max
                    uint128 fee = computeFee(uint128(uint256(-amountDelta)), poolKey.config.fee());
                    if (isToken1) {
                        CORE.accumulateAsFees(poolKey, 0, fee);
                        CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), 0, amountDelta);
                    } else {
                        CORE.accumulateAsFees(poolKey, fee, 0);
                        CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), amountDelta, 0);
                    }

                    amountDelta += int128(fee);
                } else {
                    if (isToken1) {
                        CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), 0, amountDelta);
                    } else {
                        CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), amountDelta, 0);
                    }
                }

                emit OrderUpdated(owner, salt, orderKey, saleRateDelta);

                result = abi.encode(amountDelta);
            } else if (callType == 1) {
                (, bytes32 salt, OrderKey memory orderKey) = abi.decode(data, (uint256, bytes32, OrderKey));

                PoolKey memory poolKey = orderKey.toPoolKey(address(this));
                PoolId poolId = poolKey.toPoolId();
                _executeVirtualOrdersFromWithinLock(poolKey, poolId);

                OrderId orderId = orderKey.toOrderId();

                StorageSlot orderStateSlot =
                    TWAMMStorageLayout.orderStateSlotFollowedByOrderRewardRateSnapshotSlot(owner, salt, orderId);

                StorageSlot orderRewardRateSnapshotSlot = orderStateSlot.next();

                OrderState order = OrderState.wrap(orderStateSlot.load());
                uint256 rewardRateSnapshot = uint256(orderRewardRateSnapshotSlot.load());

                uint256 rewardRateInside = getRewardRateInside(poolId, orderKey.config);

                uint256 purchasedAmount = computeRewardAmount(rewardRateInside - rewardRateSnapshot, order.saleRate());

                orderRewardRateSnapshotSlot.store(bytes32(rewardRateInside));

                if (purchasedAmount != 0) {
                    if (orderKey.config.isToken1()) {
                        CORE.updateSavedBalances(
                            poolKey.token0, poolKey.token1, bytes32(0), -int256(purchasedAmount), 0
                        );
                    } else {
                        CORE.updateSavedBalances(
                            poolKey.token0, poolKey.token1, bytes32(0), 0, -int256(purchasedAmount)
                        );
                    }
                }

                emit OrderProceedsWithdrawn(original.addr(), salt, orderKey, uint128(purchasedAmount));

                result = abi.encode(purchasedAmount);
            } else {
                revert();
            }
        }
    }

    function _executeVirtualOrdersFromWithinLock(PoolKey memory poolKey, PoolId poolId) internal {
        unchecked {
            StorageSlot stateSlot = TWAMMStorageLayout.twammPoolStateSlot(poolId);
            TwammPoolState state = TwammPoolState.wrap(stateSlot.load());

            // we only conditionally load this if the state is coincidentally zero,
            // in order to not lock the pool if state is 0 but the pool _is_ initialized
            // this can only happen iff a pool has zero sale rates **and** an execution of virtual orders
            // happens on the uint32 boundary
            if (TwammPoolState.unwrap(state) == bytes32(0)) {
                if (poolKey.config.extension() != address(this) || !CORE.poolState(poolId).isInitialized()) {
                    revert PoolNotInitialized();
                }
            }

            uint256 realLastVirtualOrderExecutionTime = state.realLastVirtualOrderExecutionTime();

            // no-op if already executed in this block
            if (realLastVirtualOrderExecutionTime != block.timestamp) {
                // initialize the values that are handled once per execution
                FeesPerLiquidity memory rewardRates;

                // 0 = not loaded & not updated, 1 = loaded & not updated, 2 = loaded & updated
                uint256 rewardRate0Access;
                uint256 rewardRate1Access;

                int256 saveDelta0;
                int256 saveDelta1;
                PoolState corePoolState;
                uint256 time = realLastVirtualOrderExecutionTime;

                while (time != block.timestamp) {
                    StorageSlot initializedTimesBitmapSlot = TWAMMStorageLayout.poolInitializedTimesBitmapSlot(poolId);

                    (uint256 nextTime, bool initialized) = searchForNextInitializedTime({
                        slot: initializedTimesBitmapSlot,
                        lastVirtualOrderExecutionTime: realLastVirtualOrderExecutionTime,
                        fromTime: time,
                        untilTime: block.timestamp
                    });

                    // it is assumed that this will never return a value greater than type(uint32).max
                    uint256 timeElapsed = nextTime - time;

                    uint256 amount0 = computeAmountFromSaleRate({
                        saleRate: state.saleRateToken0(), duration: timeElapsed, roundUp: false
                    });

                    uint256 amount1 = computeAmountFromSaleRate({
                        saleRate: state.saleRateToken1(), duration: timeElapsed, roundUp: false
                    });

                    int256 rewardDelta0;
                    int256 rewardDelta1;
                    // if both sale rates are non-zero but amounts are zero, we will end up doing the math for no reason since we swap 0
                    if (amount0 != 0 && amount1 != 0) {
                        if (!corePoolState.isInitialized()) {
                            corePoolState = CORE.poolState(poolId);
                        }
                        SqrtRatio sqrtRatioNext = computeNextSqrtRatio({
                            sqrtRatio: corePoolState.sqrtRatio(),
                            liquidity: corePoolState.liquidity(),
                            saleRateToken0: state.saleRateToken0(),
                            saleRateToken1: state.saleRateToken1(),
                            timeElapsed: timeElapsed,
                            fee: poolKey.config.fee()
                        });

                        PoolBalanceUpdate swapBalanceUpdate;
                        if (sqrtRatioNext > corePoolState.sqrtRatio()) {
                            (swapBalanceUpdate, corePoolState) = CORE.swap(
                                0,
                                poolKey,
                                createSwapParameters({
                                    _sqrtRatioLimit: sqrtRatioNext,
                                    _amount: int128(uint128(amount1)),
                                    _isToken1: true,
                                    _skipAhead: 0
                                })
                            );
                        } else if (sqrtRatioNext < corePoolState.sqrtRatio()) {
                            (swapBalanceUpdate, corePoolState) = CORE.swap(
                                0,
                                poolKey,
                                createSwapParameters({
                                    _sqrtRatioLimit: sqrtRatioNext,
                                    _amount: int128(uint128(amount0)),
                                    _isToken1: false,
                                    _skipAhead: 0
                                })
                            );
                        }

                        saveDelta0 -= swapBalanceUpdate.delta0();
                        saveDelta1 -= swapBalanceUpdate.delta1();

                        // this cannot overflow or underflow because swapDelta0 is constrained to int128,
                        // and amounts computed from uint112 sale rates cannot exceed uint112.max
                        rewardDelta0 = swapBalanceUpdate.delta0() - int256(uint256(amount0));
                        rewardDelta1 = swapBalanceUpdate.delta1() - int256(uint256(amount1));
                    } else if (amount0 != 0 || amount1 != 0) {
                        PoolBalanceUpdate swapBalanceUpdate;
                        if (amount0 != 0) {
                            (swapBalanceUpdate, corePoolState) = CORE.swap(
                                0,
                                poolKey,
                                createSwapParameters({
                                    _sqrtRatioLimit: MIN_SQRT_RATIO,
                                    _amount: int128(uint128(amount0)),
                                    _isToken1: false,
                                    _skipAhead: 0
                                })
                            );
                        } else {
                            (swapBalanceUpdate, corePoolState) = CORE.swap(
                                0,
                                poolKey,
                                createSwapParameters({
                                    _sqrtRatioLimit: MAX_SQRT_RATIO,
                                    _amount: int128(uint128(amount1)),
                                    _isToken1: true,
                                    _skipAhead: 0
                                })
                            );
                        }

                        (rewardDelta0, rewardDelta1) = (swapBalanceUpdate.delta0(), swapBalanceUpdate.delta1());
                        saveDelta0 -= rewardDelta0;
                        saveDelta1 -= rewardDelta1;
                    }

                    if (rewardDelta0 < 0) {
                        if (rewardRate0Access == 0) {
                            rewardRates.value0 = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).load());
                        }
                        rewardRate0Access = 2;
                        rewardRates.value0 += FixedPointMathLib.rawDiv(
                            uint256(-rewardDelta0) << 128, state.saleRateToken1()
                        );
                    }

                    if (rewardDelta1 < 0) {
                        if (rewardRate1Access == 0) {
                            rewardRates.value1 = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).next().load());
                        }
                        rewardRate1Access = 2;
                        rewardRates.value1 += FixedPointMathLib.rawDiv(
                            uint256(-rewardDelta1) << 128, state.saleRateToken0()
                        );
                    }

                    if (initialized) {
                        if (rewardRate0Access == 0) {
                            rewardRates.value0 = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).load());
                            rewardRate0Access = 1;
                        }
                        if (rewardRate1Access == 0) {
                            rewardRates.value1 = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).next().load());
                            rewardRate1Access = 1;
                        }

                        TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, nextTime)
                            .storeTwo(bytes32(rewardRates.value0), bytes32(rewardRates.value1));

                        StorageSlot timeInfoSlot = TWAMMStorageLayout.poolTimeInfosSlot(poolId, nextTime);
                        (, int112 saleRateDeltaToken0, int112 saleRateDeltaToken1) =
                            TimeInfo.wrap(timeInfoSlot.load()).parse();

                        state = createTwammPoolState({
                            _lastVirtualOrderExecutionTime: uint32(nextTime),
                            _saleRateToken0: uint112(addSaleRateDelta(state.saleRateToken0(), saleRateDeltaToken0)),
                            _saleRateToken1: uint112(addSaleRateDelta(state.saleRateToken1(), saleRateDeltaToken1))
                        });

                        // this time is _consumed_, will never be crossed again, so we delete the info we no longer need.
                        // this helps reduce the cost of executing virtual orders.
                        timeInfoSlot.store(0);

                        flipTime(initializedTimesBitmapSlot, nextTime);
                    } else {
                        state = createTwammPoolState({
                            _lastVirtualOrderExecutionTime: uint32(nextTime),
                            _saleRateToken0: state.saleRateToken0(),
                            _saleRateToken1: state.saleRateToken1()
                        });
                    }

                    time = nextTime;
                }

                if (saveDelta0 != 0 || saveDelta1 != 0) {
                    CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), saveDelta0, saveDelta1);
                }

                if (rewardRate0Access == 2) {
                    TWAMMStorageLayout.poolRewardRatesSlot(poolId).store(bytes32(rewardRates.value0));
                }
                if (rewardRate1Access == 2) {
                    TWAMMStorageLayout.poolRewardRatesSlot(poolId).next().store(bytes32(rewardRates.value1));
                }

                stateSlot.store(TwammPoolState.unwrap(state));

                _emitVirtualOrdersExecuted(poolId, state.saleRateToken0(), state.saleRateToken1());
            }
        }
    }

    // Executes virtual orders for the specified initialized pool key. Protected because it is only called by core.
    function locked_6416899205(uint256) external override onlyCore {
        PoolKey memory poolKey;
        assembly ("memory-safe") {
            // copy the poolkey out of calldata at the solidity-allocated address
            calldatacopy(poolKey, 36, 96)
        }
        _executeVirtualOrdersFromWithinLock(poolKey, poolKey.toPoolId());
    }

    /// @inheritdoc ITWAMM
    function lockAndExecuteVirtualOrders(PoolKey memory poolKey) public {
        // the only thing we lock for is executing virtual orders, so all we need to encode is the pool key
        // so we call lock on the core contract with the pool key after it
        address target = address(CORE);
        assembly ("memory-safe") {
            let o := mload(0x40)
            mstore(o, shl(224, 0xf83d08ba))
            mcopy(add(o, 4), poolKey, 96)

            // If the call failed, pass through the revert
            if iszero(call(gas(), target, 0, o, 100, 0, 0)) {
                returndatacopy(o, 0, returndatasize())
                revert(o, returndatasize())
            }
        }
    }

    ///////////////////////// Extension call points /////////////////////////

    // This method must be protected because it sets state directly
    function afterInitializePool(address, PoolKey memory key, int32, SqrtRatio)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (!key.config.isFullRange()) revert FullRangePoolOnly();

        PoolId poolId = key.toPoolId();

        TWAMMStorageLayout.twammPoolStateSlot(poolId)
            .store(
                TwammPoolState.unwrap(
                    createTwammPoolState({
                        _lastVirtualOrderExecutionTime: uint32(block.timestamp), _saleRateToken0: 0, _saleRateToken1: 0
                    })
                )
            );

        _emitVirtualOrdersExecuted({poolId: poolId, saleRateToken0: 0, saleRateToken1: 0});
    }

    // Since anyone can call the method `#lockAndExecuteVirtualOrders`, the method is not protected
    function beforeSwap(Locker, PoolKey memory poolKey, SwapParameters) external override(BaseExtension, IExtension) {
        lockAndExecuteVirtualOrders(poolKey);
    }

    // Since anyone can call the method `#lockAndExecuteVirtualOrders`, the method is not protected
    function beforeUpdatePosition(Locker, PoolKey memory poolKey, PositionId, int128)
        external
        override(BaseExtension, IExtension)
    {
        lockAndExecuteVirtualOrders(poolKey);
    }

    // Since anyone can call the method `#lockAndExecuteVirtualOrders`, the method is not protected
    function beforeCollectFees(Locker, PoolKey memory poolKey, PositionId)
        external
        override(BaseExtension, IExtension)
    {
        lockAndExecuteVirtualOrders(poolKey);
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {CallPoints, addressToCallPoints} from "./types/callPoints.sol";
import {PoolKey} from "./types/poolKey.sol";
import {PoolConfig} from "./types/poolConfig.sol";
import {PositionId} from "./types/positionId.sol";
import {FeesPerLiquidity, feesPerLiquidityFromAmounts} from "./types/feesPerLiquidity.sol";
import {Position} from "./types/position.sol";
import {tickToSqrtRatio, sqrtRatioToTick} from "./math/ticks.sol";
import {CoreStorageLayout} from "./libraries/CoreStorageLayout.sol";
import {ExtensionCallPointsLib} from "./libraries/ExtensionCallPointsLib.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";
import {ExposedStorage} from "./base/ExposedStorage.sol";
import {liquidityDeltaToAmountDelta, addLiquidityDelta} from "./math/liquidity.sol";
import {findNextInitializedTick, findPrevInitializedTick, flipTick} from "./math/tickBitmap.sol";
import {ICore, IExtension} from "./interfaces/ICore.sol";
import {FlashAccountant} from "./base/FlashAccountant.sol";
import {MIN_TICK, MAX_TICK, NATIVE_TOKEN_ADDRESS} from "./math/constants.sol";
import {MIN_SQRT_RATIO, MAX_SQRT_RATIO, SqrtRatio} from "./types/sqrtRatio.sol";
import {PoolState, createPoolState} from "./types/poolState.sol";
import {SwapParameters} from "./types/swapParameters.sol";
import {TickInfo, createTickInfo} from "./types/tickInfo.sol";
import {PoolBalanceUpdate, createPoolBalanceUpdate} from "./types/poolBalanceUpdate.sol";
import {PoolId} from "./types/poolId.sol";
import {Locker} from "./types/locker.sol";
import {computeFee, amountBeforeFee} from "./math/fee.sol";
import {nextSqrtRatioFromAmount0, nextSqrtRatioFromAmount1} from "./math/sqrtRatio.sol";
import {
    amount0Delta,
    amount1Delta,
    amount0DeltaSorted,
    amount1DeltaSorted,
    sortAndConvertToFixedSqrtRatios
} from "./math/delta.sol";
import {StorageSlot} from "./types/storageSlot.sol";
import {LibBit} from "solady/utils/LibBit.sol";

/// @title Ekubo Protocol Core
/// @author Moody Salem <moody@ekubo.org>
/// @notice Singleton contract holding all tokens and containing all possible operations in Ekubo Protocol
/// @dev Implements the core AMM functionality including pools, positions, swaps, and fee collection
/// @dev Note this code is under the terms of the Ekubo DAO Shared Revenue License 1.0.
/// @dev The full terms of the license can be found at the contenthash specified at ekubo-license-v1.eth.
contract Core is ICore, FlashAccountant, ExposedStorage {
    using ExtensionCallPointsLib for *;

    /// @inheritdoc ICore
    function registerExtension(CallPoints memory expectedCallPoints) external {
        CallPoints memory computed = addressToCallPoints(msg.sender);
        if (!computed.eq(expectedCallPoints) || !computed.isValid()) {
            revert FailedRegisterInvalidCallPoints();
        }
        StorageSlot isExtensionRegisteredSlot = CoreStorageLayout.isExtensionRegisteredSlot(msg.sender);
        if (isExtensionRegisteredSlot.load() != bytes32(0)) revert ExtensionAlreadyRegistered();

        isExtensionRegisteredSlot.store(bytes32(LibBit.rawToUint(true)));

        emit ExtensionRegistered(msg.sender);
    }

    function readPoolState(PoolId poolId) internal view returns (PoolState state) {
        state = PoolState.wrap(CoreStorageLayout.poolStateSlot(poolId).load());
    }

    function writePoolState(PoolId poolId, PoolState state) internal {
        CoreStorageLayout.poolStateSlot(poolId).store(PoolState.unwrap(state));
    }

    /// @inheritdoc ICore
    function initializePool(PoolKey memory poolKey, int32 tick) external returns (SqrtRatio sqrtRatio) {
        poolKey.validate();

        address extension = poolKey.config.extension();
        if (extension != address(0)) {
            StorageSlot isExtensionRegisteredSlot = CoreStorageLayout.isExtensionRegisteredSlot(extension);

            if (isExtensionRegisteredSlot.load() == bytes32(0)) {
                revert ExtensionNotRegistered();
            }

            IExtension(extension).maybeCallBeforeInitializePool(msg.sender, poolKey, tick);
        }

        PoolId poolId = poolKey.toPoolId();
        PoolState state = readPoolState(poolId);
        if (state.isInitialized()) revert PoolAlreadyInitialized();

        sqrtRatio = tickToSqrtRatio(tick);
        writePoolState(poolId, createPoolState({_sqrtRatio: sqrtRatio, _tick: tick, _liquidity: 0}));

        // initialize these slots so the first swap or deposit on the pool is the same cost as any other swap
        StorageSlot fplSlot0 = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId);
        fplSlot0.store(bytes32(uint256(1)));
        fplSlot0.next().store(bytes32(uint256(1)));

        emit PoolInitialized(poolId, poolKey, tick, sqrtRatio);

        IExtension(extension).maybeCallAfterInitializePool(msg.sender, poolKey, tick, sqrtRatio);
    }

    /// @inheritdoc ICore
    function prevInitializedTick(PoolId poolId, int32 fromTick, uint32 tickSpacing, uint256 skipAhead)
        external
        view
        returns (int32 tick, bool isInitialized)
    {
        (tick, isInitialized) =
            findPrevInitializedTick(CoreStorageLayout.tickBitmapsSlot(poolId), fromTick, tickSpacing, skipAhead);
    }

    /// @inheritdoc ICore
    function nextInitializedTick(PoolId poolId, int32 fromTick, uint32 tickSpacing, uint256 skipAhead)
        external
        view
        returns (int32 tick, bool isInitialized)
    {
        (tick, isInitialized) =
            findNextInitializedTick(CoreStorageLayout.tickBitmapsSlot(poolId), fromTick, tickSpacing, skipAhead);
    }

    /// @inheritdoc ICore
    function updateSavedBalances(
        address token0,
        address token1,
        bytes32,
        // positive is saving, negative is loading
        int256 delta0,
        int256 delta1
    )
        external
        payable
    {
        if (token0 >= token1) revert SavedBalanceTokensNotSorted();

        (uint256 id, address lockerAddr) = _requireLocker().parse();

        assembly ("memory-safe") {
            function addDelta(u, i) -> result {
                // full‐width sum mod 2^256
                let sum := add(u, i)
                // 1 if i<0 else 0
                let sign := shr(255, i)
                // if sum > type(uint128).max || (i>=0 && sum<u) || (i<0 && sum>u) ⇒ 256-bit wrap or underflow
                if or(shr(128, sum), or(and(iszero(sign), lt(sum, u)), and(sign, gt(sum, u)))) {
                    mstore(0x00, 0x1293d6fa) // `SavedBalanceOverflow()`
                    revert(0x1c, 0x04)
                }
                result := sum
            }

            // we can cheaply calldatacopy the arguments into memory, hence no call to CoreStorageLayout#savedBalancesSlot
            let free := mload(0x40)
            mstore(free, lockerAddr)
            // copy the first 3 arguments in the same order
            calldatacopy(add(free, 0x20), 4, 96)
            let slot := keccak256(free, 128)
            let balances := sload(slot)

            let b0 := shr(128, balances)
            let b1 := shr(128, shl(128, balances))

            let b0Next := addDelta(b0, delta0)
            let b1Next := addDelta(b1, delta1)

            sstore(slot, add(shl(128, b0Next), b1Next))
        }

        _updatePairDebtWithNative(id, token0, token1, delta0, delta1);
    }

    /// @notice Returns the pool fees per liquidity inside the given bounds
    /// @dev Internal function that calculates fees per liquidity within position bounds
    /// @param poolId Unique identifier for the pool
    /// @param tick Current tick of the pool
    /// @param tickLower Lower tick of the price range to get the snapshot of
    /// @param tickLower Upper tick of the price range to get the snapshot of
    /// @return feesPerLiquidityInside Accumulated fees per liquidity snapshot inside the bounds. Note this is a relative value.
    function _getPoolFeesPerLiquidityInside(PoolId poolId, int32 tick, int32 tickLower, int32 tickUpper)
        internal
        view
        returns (FeesPerLiquidity memory feesPerLiquidityInside)
    {
        uint256 lower0;
        uint256 lower1;
        uint256 upper0;
        uint256 upper1;
        {
            (StorageSlot l0, StorageSlot l1) = CoreStorageLayout.poolTickFeesPerLiquidityOutsideSlot(poolId, tickLower);
            (lower0, lower1) = (uint256(l0.load()), uint256(l1.load()));

            (StorageSlot u0, StorageSlot u1) = CoreStorageLayout.poolTickFeesPerLiquidityOutsideSlot(poolId, tickUpper);
            (upper0, upper1) = (uint256(u0.load()), uint256(u1.load()));
        }

        unchecked {
            if (tick < tickLower) {
                feesPerLiquidityInside.value0 = lower0 - upper0;
                feesPerLiquidityInside.value1 = lower1 - upper1;
            } else if (tick < tickUpper) {
                uint256 global0;
                uint256 global1;
                {
                    (bytes32 g0, bytes32 g1) = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).loadTwo();
                    (global0, global1) = (uint256(g0), uint256(g1));
                }

                feesPerLiquidityInside.value0 = global0 - upper0 - lower0;
                feesPerLiquidityInside.value1 = global1 - upper1 - lower1;
            } else {
                feesPerLiquidityInside.value0 = upper0 - lower0;
                feesPerLiquidityInside.value1 = upper1 - lower1;
            }
        }
    }

    /// @inheritdoc ICore
    function getPoolFeesPerLiquidityInside(PoolId poolId, int32 tickLower, int32 tickUpper)
        external
        view
        returns (FeesPerLiquidity memory)
    {
        return _getPoolFeesPerLiquidityInside(poolId, readPoolState(poolId).tick(), tickLower, tickUpper);
    }

    /// @inheritdoc ICore
    function accumulateAsFees(PoolKey memory poolKey, uint128 _amount0, uint128 _amount1) external payable {
        (uint256 id, address lockerAddr) = _requireLocker().parse();
        require(lockerAddr == poolKey.config.extension());

        PoolId poolId = poolKey.toPoolId();

        uint256 amount0;
        uint256 amount1;
        assembly ("memory-safe") {
            amount0 := _amount0
            amount1 := _amount1
        }

        // Note we do not check pool is initialized. If the extension calls this for a pool that does not exist,
        //  the fees are simply burned since liquidity is 0.

        if (amount0 != 0 || amount1 != 0) {
            uint256 liquidity;
            {
                uint128 _liquidity = readPoolState(poolId).liquidity();
                assembly ("memory-safe") {
                    liquidity := _liquidity
                }
            }

            unchecked {
                if (liquidity != 0) {
                    StorageSlot slot0 = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId);

                    if (amount0 != 0) {
                        slot0.store(
                            bytes32(uint256(slot0.load()) + FixedPointMathLib.rawDiv(amount0 << 128, liquidity))
                        );
                    }
                    if (amount1 != 0) {
                        StorageSlot slot1 = slot0.next();
                        slot1.store(
                            bytes32(uint256(slot1.load()) + FixedPointMathLib.rawDiv(amount1 << 128, liquidity))
                        );
                    }
                }
            }
        }

        // whether the fees are actually accounted to any position, the caller owes the debt
        _updatePairDebtWithNative(id, poolKey.token0, poolKey.token1, int256(amount0), int256(amount1));

        emit FeesAccumulated(poolId, _amount0, _amount1);
    }

    /// @notice Updates tick information when liquidity is added or removed
    /// @dev Private function that handles tick initialization and liquidity tracking
    /// @param poolId Unique identifier for the pool
    /// @param tick Tick to update
    /// @param poolConfig Pool configuration containing tick spacing
    /// @param liquidityDelta Change in liquidity
    /// @param isUpper Whether this is the upper bound of a position
    function _updateTick(PoolId poolId, int32 tick, PoolConfig poolConfig, int128 liquidityDelta, bool isUpper)
        private
    {
        StorageSlot tickInfoSlot = CoreStorageLayout.poolTicksSlot(poolId, tick);

        (int128 currentLiquidityDelta, uint128 currentLiquidityNet) = TickInfo.wrap(tickInfoSlot.load()).parse();
        uint128 liquidityNetNext = addLiquidityDelta(currentLiquidityNet, liquidityDelta);
        // this is checked math
        int128 liquidityDeltaNext =
            isUpper ? currentLiquidityDelta - liquidityDelta : currentLiquidityDelta + liquidityDelta;

        // Check that liquidityNet doesn't exceed max liquidity per tick
        uint128 maxLiquidity = poolConfig.concentratedMaxLiquidityPerTick();
        if (liquidityNetNext > maxLiquidity) {
            revert MaxLiquidityPerTickExceeded(tick, liquidityNetNext, maxLiquidity);
        }

        if ((currentLiquidityNet == 0) != (liquidityNetNext == 0)) {
            flipTick(CoreStorageLayout.tickBitmapsSlot(poolId), tick, poolConfig.concentratedTickSpacing());

            (StorageSlot fplSlot0, StorageSlot fplSlot1) =
                CoreStorageLayout.poolTickFeesPerLiquidityOutsideSlot(poolId, tick);

            bytes32 v;
            assembly ("memory-safe") {
                v := gt(liquidityNetNext, 0)
            }

            // initialize the storage slots for the fees per liquidity outside to non-zero so tick crossing is cheaper
            fplSlot0.store(v);
            fplSlot1.store(v);
        }

        tickInfoSlot.store(TickInfo.unwrap(createTickInfo(liquidityDeltaNext, liquidityNetNext)));
    }

    /// @notice Updates debt for a token pair, handling native token payments for token0
    /// @dev Optimized version that updates both tokens' debts in a single operation when possible.
    ///      Assumes token0 < token1 (tokens are sorted).
    /// @param id Lock ID for debt tracking
    /// @param token0 Address of token0 (must be < token1)
    /// @param token1 Address of token1 (must be > token0)
    /// @param debtChange0 Change in debt amount for token0
    /// @param debtChange1 Change in debt amount for token1
    function _updatePairDebtWithNative(
        uint256 id,
        address token0,
        address token1,
        int256 debtChange0,
        int256 debtChange1
    ) private {
        if (msg.value == 0) {
            // No native token payment included in the call, so use optimized pair update
            _updatePairDebt(id, token0, token1, debtChange0, debtChange1);
        } else {
            if (token0 == NATIVE_TOKEN_ADDRESS) {
                unchecked {
                    // token0 is native, so we can still use pair update with adjusted debtChange0
                    // Subtraction is safe because debtChange0 and msg.value are both bounded by int128/uint128
                    _updatePairDebt(id, token0, token1, debtChange0 - int256(msg.value), debtChange1);
                }
            } else {
                // token0 is not native, and since token0 < token1, token1 cannot be native either
                // Update the token0, token1 debt and then update native token debt separately
                unchecked {
                    _updatePairDebt(id, token0, token1, debtChange0, debtChange1);
                    _accountDebt(id, NATIVE_TOKEN_ADDRESS, -int256(msg.value));
                }
            }
        }
    }

    /// @inheritdoc ICore
    function updatePosition(PoolKey memory poolKey, PositionId positionId, int128 liquidityDelta)
        external
        payable
        returns (PoolBalanceUpdate balanceUpdate)
    {
        positionId.validate(poolKey.config);

        Locker locker = _requireLocker();

        IExtension(poolKey.config.extension())
            .maybeCallBeforeUpdatePosition(locker, poolKey, positionId, liquidityDelta);

        PoolId poolId = poolKey.toPoolId();
        PoolState state = readPoolState(poolId);
        if (!state.isInitialized()) revert PoolNotInitialized();

        if (liquidityDelta != 0) {
            (SqrtRatio sqrtRatioLower, SqrtRatio sqrtRatioUpper) =
                (tickToSqrtRatio(positionId.tickLower()), tickToSqrtRatio(positionId.tickUpper()));

            (int128 delta0, int128 delta1) =
                liquidityDeltaToAmountDelta(state.sqrtRatio(), liquidityDelta, sqrtRatioLower, sqrtRatioUpper);

            StorageSlot positionSlot = CoreStorageLayout.poolPositionsSlot(poolId, locker.addr(), positionId);
            Position storage position;
            assembly ("memory-safe") {
                position.slot := positionSlot
            }

            uint128 liquidityNext = addLiquidityDelta(position.liquidity, liquidityDelta);

            FeesPerLiquidity memory feesPerLiquidityInside;

            if (poolKey.config.isConcentrated()) {
                // the position is fully withdrawn
                if (liquidityNext == 0) {
                    // we need to fetch it before the tick fees per liquidity outside is deleted
                    feesPerLiquidityInside = _getPoolFeesPerLiquidityInside(
                        poolId, state.tick(), positionId.tickLower(), positionId.tickUpper()
                    );
                }

                _updateTick(poolId, positionId.tickLower(), poolKey.config, liquidityDelta, false);
                _updateTick(poolId, positionId.tickUpper(), poolKey.config, liquidityDelta, true);

                if (liquidityNext != 0) {
                    feesPerLiquidityInside = _getPoolFeesPerLiquidityInside(
                        poolId, state.tick(), positionId.tickLower(), positionId.tickUpper()
                    );
                }

                if (state.tick() >= positionId.tickLower() && state.tick() < positionId.tickUpper()) {
                    state = createPoolState({
                        _sqrtRatio: state.sqrtRatio(),
                        _tick: state.tick(),
                        _liquidity: addLiquidityDelta(state.liquidity(), liquidityDelta)
                    });
                    writePoolState(poolId, state);
                }
            } else {
                // we store the active liquidity in the liquidity slot for stableswap pools
                state = createPoolState({
                    _sqrtRatio: state.sqrtRatio(),
                    _tick: state.tick(),
                    _liquidity: addLiquidityDelta(state.liquidity(), liquidityDelta)
                });
                writePoolState(poolId, state);
                StorageSlot fplFirstSlot = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId);
                feesPerLiquidityInside.value0 = uint256(fplFirstSlot.load());
                feesPerLiquidityInside.value1 = uint256(fplFirstSlot.next().load());
            }

            if (liquidityNext == 0) {
                position.liquidity = 0;
                position.feesPerLiquidityInsideLast = FeesPerLiquidity(0, 0);
            } else {
                (uint128 fees0, uint128 fees1) = position.fees(feesPerLiquidityInside);
                position.liquidity = liquidityNext;
                position.feesPerLiquidityInsideLast =
                    feesPerLiquidityInside.sub(feesPerLiquidityFromAmounts(fees0, fees1, liquidityNext));
            }

            _updatePairDebtWithNative(locker.id(), poolKey.token0, poolKey.token1, delta0, delta1);

            balanceUpdate = createPoolBalanceUpdate(delta0, delta1);
            emit PositionUpdated(locker.addr(), poolId, positionId, liquidityDelta, balanceUpdate, state);
        }

        IExtension(poolKey.config.extension())
            .maybeCallAfterUpdatePosition(locker, poolKey, positionId, liquidityDelta, balanceUpdate, state);
    }

    /// @inheritdoc ICore
    function setExtraData(PoolId poolId, PositionId positionId, bytes16 _extraData) external {
        StorageSlot firstSlot = CoreStorageLayout.poolPositionsSlot(poolId, msg.sender, positionId);

        bytes32 extraData;
        assembly ("memory-safe") {
            extraData := _extraData
        }

        firstSlot.store(((firstSlot.load() >> 128) << 128) | (extraData >> 128));
    }

    /// @inheritdoc ICore
    function collectFees(PoolKey memory poolKey, PositionId positionId)
        external
        returns (uint128 amount0, uint128 amount1)
    {
        Locker locker = _requireLocker();

        IExtension(poolKey.config.extension()).maybeCallBeforeCollectFees(locker, poolKey, positionId);

        PoolId poolId = poolKey.toPoolId();

        Position storage position;
        StorageSlot positionSlot = CoreStorageLayout.poolPositionsSlot(poolId, locker.addr(), positionId);
        assembly ("memory-safe") {
            position.slot := positionSlot
        }

        FeesPerLiquidity memory feesPerLiquidityInside;
        if (poolKey.config.isStableswap()) {
            // Stableswap pools: use global fees per liquidity
            StorageSlot fplFirstSlot = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId);
            feesPerLiquidityInside.value0 = uint256(fplFirstSlot.load());
            feesPerLiquidityInside.value1 = uint256(fplFirstSlot.next().load());
        } else {
            // Concentrated pools: calculate fees per liquidity inside the position bounds
            feesPerLiquidityInside = _getPoolFeesPerLiquidityInside(
                poolId, readPoolState(poolId).tick(), positionId.tickLower(), positionId.tickUpper()
            );
        }

        (amount0, amount1) = position.fees(feesPerLiquidityInside);

        position.feesPerLiquidityInsideLast = feesPerLiquidityInside;

        _updatePairDebt(
            locker.id(), poolKey.token0, poolKey.token1, -int256(uint256(amount0)), -int256(uint256(amount1))
        );

        emit PositionFeesCollected(locker.addr(), poolId, positionId, amount0, amount1);

        IExtension(poolKey.config.extension()).maybeCallAfterCollectFees(locker, poolKey, positionId, amount0, amount1);
    }

    /// @inheritdoc ICore
    function swap_6269342730() external payable {
        unchecked {
            PoolKey memory poolKey;
            address token0;
            address token1;
            PoolConfig config;

            SwapParameters params;

            assembly ("memory-safe") {
                token0 := calldataload(4)
                token1 := calldataload(36)
                config := calldataload(68)
                params := calldataload(100)
                calldatacopy(poolKey, 4, 96)
            }

            SqrtRatio sqrtRatioLimit = params.sqrtRatioLimit();
            if (!sqrtRatioLimit.isValid()) revert InvalidSqrtRatioLimit();

            Locker locker = _requireLocker();

            IExtension(config.extension()).maybeCallBeforeSwap(locker, poolKey, params);

            PoolId poolId = poolKey.toPoolId();

            PoolState stateAfter = readPoolState(poolId);

            if (!stateAfter.isInitialized()) revert PoolNotInitialized();

            int256 amountRemaining = params.amount();

            PoolBalanceUpdate balanceUpdate;

            // 0 swap amount or sqrt ratio limit == sqrt ratio is no-op
            if (amountRemaining != 0 && stateAfter.sqrtRatio() != sqrtRatioLimit) {
                (SqrtRatio sqrtRatio, int32 tick, uint128 liquidity) = stateAfter.parse();

                bool isToken1 = params.isToken1();

                bool isExactOut = amountRemaining < 0;
                bool increasing;
                assembly ("memory-safe") {
                    increasing := xor(isToken1, isExactOut)
                }

                if ((sqrtRatioLimit < sqrtRatio) == increasing) {
                    revert SqrtRatioLimitWrongDirection();
                }

                int256 calculatedAmount;

                // fees per liquidity only for the input token
                uint256 inputTokenFeesPerLiquidity;

                // 0 = not loaded & not updated, 1 = loaded & not updated, 2 = loaded & updated
                uint256 feesAccessed;

                while (true) {
                    int32 nextTick;
                    bool isInitialized;
                    SqrtRatio nextTickSqrtRatio;

                    // For stableswap pools, determine active liquidity for this step
                    uint128 stepLiquidity = liquidity;

                    if (config.isStableswap()) {
                        if (config.isFullRange()) {
                            // special case since we don't need to compute min/max tick sqrt ratio
                            (nextTick, nextTickSqrtRatio) =
                                increasing ? (MAX_TICK, MAX_SQRT_RATIO) : (MIN_TICK, MIN_SQRT_RATIO);
                        } else {
                            (int32 lower, int32 upper) = config.stableswapActiveLiquidityTickRange();

                            bool inRange;
                            assembly ("memory-safe") {
                                inRange := and(slt(tick, upper), iszero(slt(tick, lower)))
                            }
                            if (inRange) {
                                nextTick = increasing ? upper : lower;
                                nextTickSqrtRatio = tickToSqrtRatio(nextTick);
                            } else {
                                if (tick < lower) {
                                    (nextTick, nextTickSqrtRatio) =
                                        increasing ? (lower, tickToSqrtRatio(lower)) : (MIN_TICK, MIN_SQRT_RATIO);
                                } else {
                                    // tick >= upper implied
                                    (nextTick, nextTickSqrtRatio) =
                                        increasing ? (MAX_TICK, MAX_SQRT_RATIO) : (upper, tickToSqrtRatio(upper));
                                }
                                stepLiquidity = 0;
                            }
                        }
                    } else {
                        // concentrated liquidity pools use the tick bitmaps
                        (nextTick, isInitialized) = increasing
                            ? findNextInitializedTick(
                                CoreStorageLayout.tickBitmapsSlot(poolId),
                                tick,
                                config.concentratedTickSpacing(),
                                params.skipAhead()
                            )
                            : findPrevInitializedTick(
                                CoreStorageLayout.tickBitmapsSlot(poolId),
                                tick,
                                config.concentratedTickSpacing(),
                                params.skipAhead()
                            );

                        nextTickSqrtRatio = tickToSqrtRatio(nextTick);
                    }

                    SqrtRatio limitedNextSqrtRatio =
                        increasing ? nextTickSqrtRatio.min(sqrtRatioLimit) : nextTickSqrtRatio.max(sqrtRatioLimit);

                    SqrtRatio sqrtRatioNext;

                    if (stepLiquidity == 0) {
                        // if the pool is empty, the swap will always move all the way to the limit price
                        sqrtRatioNext = limitedNextSqrtRatio;
                    } else {
                        // this amount is what moves the price
                        int128 priceImpactAmount;
                        if (isExactOut) {
                            assembly ("memory-safe") {
                                priceImpactAmount := amountRemaining
                            }
                        } else {
                            uint128 amountU128;
                            assembly ("memory-safe") {
                                // cast is safe because amountRemaining is g.t. 0 and fits in int128
                                amountU128 := amountRemaining
                            }
                            uint128 feeAmount = computeFee(amountU128, config.fee());
                            assembly ("memory-safe") {
                                // feeAmount will never exceed amountRemaining since fee is < 100%
                                priceImpactAmount := sub(amountRemaining, feeAmount)
                            }
                        }

                        SqrtRatio sqrtRatioNextFromAmount = isToken1
                            ? nextSqrtRatioFromAmount1(sqrtRatio, stepLiquidity, priceImpactAmount)
                            : nextSqrtRatioFromAmount0(sqrtRatio, stepLiquidity, priceImpactAmount);

                        bool hitLimit;
                        assembly ("memory-safe") {
                            // Branchless limit check: (increasing && next > limit) || (!increasing && next < limit)
                            let exceedsUp := and(increasing, gt(sqrtRatioNextFromAmount, limitedNextSqrtRatio))
                            let exceedsDown :=
                                and(iszero(increasing), lt(sqrtRatioNextFromAmount, limitedNextSqrtRatio))
                            hitLimit := or(exceedsUp, exceedsDown)
                        }

                        // the change in fees per liquidity for this step of the iteration
                        uint256 stepFeesPerLiquidity;

                        if (hitLimit) {
                            (uint256 sqrtRatioLower, uint256 sqrtRatioUpper) =
                                sortAndConvertToFixedSqrtRatios(limitedNextSqrtRatio, sqrtRatio);
                            (uint128 limitSpecifiedAmountDelta, uint128 limitCalculatedAmountDelta) = isToken1
                                ? (
                                    amount1DeltaSorted(sqrtRatioLower, sqrtRatioUpper, stepLiquidity, !isExactOut),
                                    amount0DeltaSorted(sqrtRatioLower, sqrtRatioUpper, stepLiquidity, isExactOut)
                                )
                                : (
                                    amount0DeltaSorted(sqrtRatioLower, sqrtRatioUpper, stepLiquidity, !isExactOut),
                                    amount1DeltaSorted(sqrtRatioLower, sqrtRatioUpper, stepLiquidity, isExactOut)
                                );

                            if (isExactOut) {
                                uint128 beforeFee = amountBeforeFee(limitCalculatedAmountDelta, config.fee());
                                assembly ("memory-safe") {
                                    calculatedAmount := add(calculatedAmount, beforeFee)
                                    amountRemaining := add(amountRemaining, limitSpecifiedAmountDelta)
                                    stepFeesPerLiquidity := div(
                                        shl(128, sub(beforeFee, limitCalculatedAmountDelta)),
                                        stepLiquidity
                                    )
                                }
                            } else {
                                uint128 beforeFee = amountBeforeFee(limitSpecifiedAmountDelta, config.fee());
                                assembly ("memory-safe") {
                                    calculatedAmount := sub(calculatedAmount, limitCalculatedAmountDelta)
                                    amountRemaining := sub(amountRemaining, beforeFee)
                                    stepFeesPerLiquidity := div(
                                        shl(128, sub(beforeFee, limitSpecifiedAmountDelta)),
                                        stepLiquidity
                                    )
                                }
                            }

                            sqrtRatioNext = limitedNextSqrtRatio;
                        } else if (sqrtRatioNextFromAmount != sqrtRatio) {
                            uint128 calculatedAmountWithoutFee = isToken1
                                ? amount0Delta(sqrtRatioNextFromAmount, sqrtRatio, stepLiquidity, isExactOut)
                                : amount1Delta(sqrtRatioNextFromAmount, sqrtRatio, stepLiquidity, isExactOut);

                            if (isExactOut) {
                                uint128 includingFee = amountBeforeFee(calculatedAmountWithoutFee, config.fee());
                                assembly ("memory-safe") {
                                    calculatedAmount := add(calculatedAmount, includingFee)
                                    stepFeesPerLiquidity := div(
                                        shl(128, sub(includingFee, calculatedAmountWithoutFee)),
                                        stepLiquidity
                                    )
                                }
                            } else {
                                assembly ("memory-safe") {
                                    calculatedAmount := sub(calculatedAmount, calculatedAmountWithoutFee)
                                    stepFeesPerLiquidity := div(
                                        shl(128, sub(amountRemaining, priceImpactAmount)),
                                        stepLiquidity
                                    )
                                }
                            }

                            amountRemaining = 0;
                            sqrtRatioNext = sqrtRatioNextFromAmount;
                        } else {
                            // for an exact output swap, the price should always move since we have to round away from the current price
                            assert(!isExactOut);

                            // consume the entire input amount as fees since the price did not move
                            assembly ("memory-safe") {
                                stepFeesPerLiquidity := div(shl(128, amountRemaining), stepLiquidity)
                            }
                            amountRemaining = 0;
                            sqrtRatioNext = sqrtRatio;
                        }

                        // only if fees per liquidity was updated in this swap iteration
                        if (stepFeesPerLiquidity != 0) {
                            if (feesAccessed == 0) {
                                // this loads only the input token fees per liquidity
                                inputTokenFeesPerLiquidity = uint256(
                                    CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).add(LibBit.rawToUint(increasing))
                                        .load()
                                ) + stepFeesPerLiquidity;
                            } else {
                                inputTokenFeesPerLiquidity += stepFeesPerLiquidity;
                            }

                            feesAccessed = 2;
                        }
                    }

                    if (sqrtRatioNext == nextTickSqrtRatio) {
                        sqrtRatio = sqrtRatioNext;
                        assembly ("memory-safe") {
                            // no overflow danger because nextTick is always inside the valid tick bounds
                            tick := sub(nextTick, iszero(increasing))
                        }

                        if (isInitialized) {
                            bytes32 tickValue = CoreStorageLayout.poolTicksSlot(poolId, nextTick).load();
                            assembly ("memory-safe") {
                                // if increasing, we add the liquidity delta, otherwise we subtract it
                                let liquidityDelta :=
                                    mul(signextend(15, tickValue), sub(increasing, iszero(increasing)))
                                liquidity := add(liquidity, liquidityDelta)
                            }

                            (StorageSlot tickFplFirstSlot, StorageSlot tickFplSecondSlot) =
                                CoreStorageLayout.poolTickFeesPerLiquidityOutsideSlot(poolId, nextTick);

                            if (feesAccessed == 0) {
                                inputTokenFeesPerLiquidity = uint256(
                                    CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).add(LibBit.rawToUint(increasing))
                                        .load()
                                );
                                feesAccessed = 1;
                            }

                            uint256 globalFeesPerLiquidityOther = uint256(
                                CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).add(LibBit.rawToUint(!increasing))
                                    .load()
                            );

                            // if increasing, it means the pool is receiving token1 so the input fees per liquidity is token1
                            if (increasing) {
                                tickFplFirstSlot.store(
                                    bytes32(globalFeesPerLiquidityOther - uint256(tickFplFirstSlot.load()))
                                );
                                tickFplSecondSlot.store(
                                    bytes32(inputTokenFeesPerLiquidity - uint256(tickFplSecondSlot.load()))
                                );
                            } else {
                                tickFplFirstSlot.store(
                                    bytes32(inputTokenFeesPerLiquidity - uint256(tickFplFirstSlot.load()))
                                );
                                tickFplSecondSlot.store(
                                    bytes32(globalFeesPerLiquidityOther - uint256(tickFplSecondSlot.load()))
                                );
                            }
                        }
                    } else if (sqrtRatio != sqrtRatioNext) {
                        sqrtRatio = sqrtRatioNext;
                        tick = sqrtRatioToTick(sqrtRatio);
                    }

                    if (amountRemaining == 0 || sqrtRatio == sqrtRatioLimit) {
                        break;
                    }
                }

                int128 calculatedAmountDelta =
                    SafeCastLib.toInt128(FixedPointMathLib.max(type(int128).min, calculatedAmount));

                int128 specifiedAmountDelta;
                int128 specifiedAmount = params.amount();
                assembly ("memory-safe") {
                    specifiedAmountDelta := sub(specifiedAmount, amountRemaining)
                }

                balanceUpdate = isToken1
                    ? createPoolBalanceUpdate(calculatedAmountDelta, specifiedAmountDelta)
                    : createPoolBalanceUpdate(specifiedAmountDelta, calculatedAmountDelta);

                stateAfter = createPoolState({_sqrtRatio: sqrtRatio, _tick: tick, _liquidity: liquidity});

                writePoolState(poolId, stateAfter);

                if (feesAccessed == 2) {
                    // this stores only the input token fees per liquidity
                    CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).add(LibBit.rawToUint(increasing))
                        .store(bytes32(inputTokenFeesPerLiquidity));
                }

                _updatePairDebtWithNative(locker.id(), token0, token1, balanceUpdate.delta0(), balanceUpdate.delta1());

                assembly ("memory-safe") {
                    let o := mload(0x40)
                    mstore(o, shl(96, locker))
                    mstore(add(o, 20), poolId)
                    mstore(add(o, 52), balanceUpdate)
                    mstore(add(o, 84), stateAfter)
                    log0(o, 116)
                }
            }

            IExtension(config.extension()).maybeCallAfterSwap(locker, poolKey, params, balanceUpdate, stateAfter);

            assembly ("memory-safe") {
                mstore(0, balanceUpdate)
                mstore(32, stateAfter)
                return(0, 64)
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {Bitmap} from "./types/bitmap.sol";
import {DropKey} from "./types/dropKey.sol";
import {ClaimKey} from "./types/claimKey.sol";
import {DropState} from "./types/dropState.sol";
import {StorageSlot} from "./types/storageSlot.sol";
import {IIncentives} from "./interfaces/IIncentives.sol";
import {IncentivesLib} from "./libraries/IncentivesLib.sol";
import {ExposedStorage} from "./base/ExposedStorage.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {Multicallable} from "solady/utils/Multicallable.sol";
import {MerkleProofLib} from "solady/utils/MerkleProofLib.sol";

/// @author Moody Salem
/// @notice A singleton contract for making many airdrops
contract Incentives is IIncentives, ExposedStorage, Multicallable {
    /// @inheritdoc IIncentives
    function fund(DropKey memory key, uint128 minimum) external override returns (uint128 fundedAmount) {
        bytes32 id = key.toDropId();

        // Load drop state from storage slot: drop id
        DropState dropState;
        assembly ("memory-safe") {
            dropState := sload(id)
        }

        uint128 currentFunded = dropState.funded();
        if (currentFunded < minimum) {
            fundedAmount = minimum - currentFunded;
            dropState = dropState.setFunded(minimum);

            // Store updated drop state
            assembly ("memory-safe") {
                sstore(id, dropState)
            }

            SafeTransferLib.safeTransferFrom(key.token, msg.sender, address(this), fundedAmount);
            emit Funded(key, minimum);
        }
    }

    /// @inheritdoc IIncentives
    function refund(DropKey memory key) external override returns (uint128 refundAmount) {
        if (msg.sender != key.owner) {
            revert DropOwnerOnly();
        }

        bytes32 id = key.toDropId();

        // Load drop state from storage slot: drop id
        DropState dropState;
        assembly ("memory-safe") {
            dropState := sload(id)
        }

        refundAmount = dropState.getRemaining();
        if (refundAmount > 0) {
            // Set funded amount to claimed amount (no remaining funds)
            dropState = dropState.setFunded(dropState.claimed());

            // Store updated drop state
            assembly ("memory-safe") {
                sstore(id, dropState)
            }

            SafeTransferLib.safeTransfer(key.token, key.owner, refundAmount);
        }
        emit Refunded(key, refundAmount);
    }

    /// @inheritdoc IIncentives
    function claim(DropKey memory key, ClaimKey memory c, bytes32[] calldata proof) external override {
        bytes32 id = key.toDropId();

        // Check that it is not claimed
        (uint256 word, uint8 bit) = IncentivesLib.claimIndexToStorageIndex(c.index);
        StorageSlot bitmapSlot;
        unchecked {
            bitmapSlot = StorageSlot.wrap(bytes32(uint256(id) + 1 + word));
        }
        Bitmap bitmap = Bitmap.wrap(uint256(bitmapSlot.load()));
        if (bitmap.isSet(bit)) revert AlreadyClaimed();

        // Check the proof is valid
        bytes32 leaf = c.toClaimId();
        if (!MerkleProofLib.verify(proof, key.root, leaf)) revert InvalidProof();

        // Load drop state from storage slot: drop id
        DropState dropState;
        assembly ("memory-safe") {
            dropState := sload(id)
        }

        // Check sufficient funds
        uint128 remaining = dropState.getRemaining();
        if (remaining < c.amount) {
            revert InsufficientFunds();
        }

        // Update claimed amount
        dropState = dropState.setClaimed(dropState.claimed() + c.amount);

        // Store updated drop state
        assembly ("memory-safe") {
            sstore(id, dropState)
        }

        // Update claimed bitmap
        bitmap = bitmap.toggle(bit);
        assembly ("memory-safe") {
            sstore(bitmapSlot, bitmap)
        }

        SafeTransferLib.safeTransfer(key.token, c.account, c.amount);
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {BaseLocker} from "./BaseLocker.sol";
import {UsesCore} from "./UsesCore.sol";
import {ICore} from "../interfaces/ICore.sol";
import {IPositions} from "../interfaces/IPositions.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {FlashAccountantLib} from "../libraries/FlashAccountantLib.sol";
import {PoolKey} from "../types/poolKey.sol";
import {PositionId, createPositionId} from "../types/positionId.sol";
import {FeesPerLiquidity} from "../types/feesPerLiquidity.sol";
import {Position} from "../types/position.sol";
import {tickToSqrtRatio} from "../math/ticks.sol";
import {maxLiquidity, liquidityDeltaToAmountDelta} from "../math/liquidity.sol";
import {PayableMulticallable} from "./PayableMulticallable.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {BaseNonfungibleToken} from "./BaseNonfungibleToken.sol";
import {NATIVE_TOKEN_ADDRESS} from "../math/constants.sol";
import {PoolId} from "../types/poolId.sol";
import {PoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";

/// @title Base Positions Contract
/// @author Moody Salem <moody@ekubo.org>
/// @notice Abstract base contract for tracking liquidity positions in Ekubo Protocol as NFTs
/// @dev Provides core position management functionality with abstract protocol fee collection methods
abstract contract BasePositions is IPositions, UsesCore, PayableMulticallable, BaseLocker, BaseNonfungibleToken {
    using CoreLib for *;
    using FlashAccountantLib for *;

    /// @notice Constructs the BasePositions contract
    /// @param core The core contract instance
    /// @param owner The owner of the contract (for access control)
    constructor(ICore core, address owner) BaseNonfungibleToken(owner) BaseLocker(core) UsesCore(core) {}

    uint256 private constant CALL_TYPE_DEPOSIT = 0;
    uint256 private constant CALL_TYPE_WITHDRAW = 1;
    uint256 private constant CALL_TYPE_WITHDRAW_PROTOCOL_FEES = 2;

    /// @inheritdoc IPositions
    function getPositionFeesAndLiquidity(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper)
        external
        view
        returns (uint128 liquidity, uint128 principal0, uint128 principal1, uint128 fees0, uint128 fees1)
    {
        PoolId poolId = poolKey.toPoolId();
        SqrtRatio sqrtRatio = CORE.poolState(poolId).sqrtRatio();
        PositionId positionId =
            createPositionId({_salt: bytes24(uint192(id)), _tickLower: tickLower, _tickUpper: tickUpper});
        Position memory position = CORE.poolPositions(poolId, address(this), positionId);

        liquidity = position.liquidity;

        // the sqrt ratio may be 0 (because the pool is uninitialized) but this is
        // fine since amount0Delta isn't called with it in this case
        (int128 delta0, int128 delta1) = liquidityDeltaToAmountDelta(
            sqrtRatio, -SafeCastLib.toInt128(position.liquidity), tickToSqrtRatio(tickLower), tickToSqrtRatio(tickUpper)
        );

        (principal0, principal1) = (uint128(-delta0), uint128(-delta1));

        FeesPerLiquidity memory feesPerLiquidityInside = poolKey.config.isFullRange()
            ? CORE.getPoolFeesPerLiquidity(poolId)
            : CORE.getPoolFeesPerLiquidityInside(poolId, tickLower, tickUpper);
        (fees0, fees1) = position.fees(feesPerLiquidityInside);
    }

    /// @inheritdoc IPositions
    function deposit(
        uint256 id,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) public payable authorizedForNft(id) returns (uint128 liquidity, uint128 amount0, uint128 amount1) {
        SqrtRatio sqrtRatio = CORE.poolState(poolKey.toPoolId()).sqrtRatio();

        liquidity =
            maxLiquidity(sqrtRatio, tickToSqrtRatio(tickLower), tickToSqrtRatio(tickUpper), maxAmount0, maxAmount1);

        if (liquidity < minLiquidity) {
            revert DepositFailedDueToSlippage(liquidity, minLiquidity);
        }

        if (liquidity > uint128(type(int128).max)) {
            revert DepositOverflow();
        }

        (amount0, amount1) = abi.decode(
            lock(abi.encode(CALL_TYPE_DEPOSIT, msg.sender, id, poolKey, tickLower, tickUpper, liquidity)),
            (uint128, uint128)
        );
    }

    /// @inheritdoc IPositions
    function collectFees(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper)
        public
        payable
        authorizedForNft(id)
        returns (uint128 amount0, uint128 amount1)
    {
        (amount0, amount1) = collectFees(id, poolKey, tickLower, tickUpper, msg.sender);
    }

    /// @inheritdoc IPositions
    function collectFees(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper, address recipient)
        public
        payable
        authorizedForNft(id)
        returns (uint128 amount0, uint128 amount1)
    {
        (amount0, amount1) = withdraw(id, poolKey, tickLower, tickUpper, 0, recipient, true);
    }

    /// @inheritdoc IPositions
    function withdraw(
        uint256 id,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 liquidity,
        address recipient,
        bool withFees
    ) public payable authorizedForNft(id) returns (uint128 amount0, uint128 amount1) {
        (amount0, amount1) = abi.decode(
            lock(abi.encode(CALL_TYPE_WITHDRAW, id, poolKey, tickLower, tickUpper, liquidity, recipient, withFees)),
            (uint128, uint128)
        );
    }

    /// @inheritdoc IPositions
    function withdraw(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper, uint128 liquidity)
        public
        payable
        returns (uint128 amount0, uint128 amount1)
    {
        (amount0, amount1) = withdraw(id, poolKey, tickLower, tickUpper, liquidity, address(msg.sender), true);
    }

    /// @inheritdoc IPositions
    function maybeInitializePool(PoolKey memory poolKey, int32 tick)
        external
        payable
        returns (bool initialized, SqrtRatio sqrtRatio)
    {
        // the before update position hook shouldn't be taken into account here
        sqrtRatio = CORE.poolState(poolKey.toPoolId()).sqrtRatio();
        if (sqrtRatio.isZero()) {
            initialized = true;
            sqrtRatio = CORE.initializePool(poolKey, tick);
        }
    }

    /// @inheritdoc IPositions
    function mintAndDeposit(
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) external payable returns (uint256 id, uint128 liquidity, uint128 amount0, uint128 amount1) {
        id = mint();
        (liquidity, amount0, amount1) = deposit(id, poolKey, tickLower, tickUpper, maxAmount0, maxAmount1, minLiquidity);
    }

    /// @inheritdoc IPositions
    function mintAndDepositWithSalt(
        bytes32 salt,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) external payable returns (uint256 id, uint128 liquidity, uint128 amount0, uint128 amount1) {
        id = mint(salt);
        (liquidity, amount0, amount1) = deposit(id, poolKey, tickLower, tickUpper, maxAmount0, maxAmount1, minLiquidity);
    }

    /// @inheritdoc IPositions
    function withdrawProtocolFees(address token0, address token1, uint128 amount0, uint128 amount1, address recipient)
        external
        payable
        onlyOwner
    {
        lock(abi.encode(CALL_TYPE_WITHDRAW_PROTOCOL_FEES, token0, token1, amount0, amount1, recipient));
    }

    /// @inheritdoc IPositions
    function getProtocolFees(address token0, address token1) external view returns (uint128 amount0, uint128 amount1) {
        (amount0, amount1) = CORE.savedBalances(address(this), token0, token1, bytes32(0));
    }

    /// @notice Handles protocol fee collection during fee collection
    /// @dev Abstract method that must be implemented by concrete contracts
    /// @param poolKey The pool key for the position
    /// @param amount0 The amount of token0 fees collected before protocol fee deduction
    /// @param amount1 The amount of token1 fees collected before protocol fee deduction
    /// @return protocolFee0 The amount of token0 protocol fees to collect
    /// @return protocolFee1 The amount of token1 protocol fees to collect
    function _computeSwapProtocolFees(PoolKey memory poolKey, uint128 amount0, uint128 amount1)
        internal
        view
        virtual
        returns (uint128 protocolFee0, uint128 protocolFee1);

    /// @notice Handles protocol fee collection during liquidity withdrawal
    /// @dev Abstract method that must be implemented by concrete contracts
    /// @param poolKey The pool key for the position
    /// @param amount0 The amount of token0 being withdrawn before protocol fee deduction
    /// @param amount1 The amount of token1 being withdrawn before protocol fee deduction
    /// @return protocolFee0 The amount of token0 protocol fees to collect
    /// @return protocolFee1 The amount of token1 protocol fees to collect
    function _computeWithdrawalProtocolFees(PoolKey memory poolKey, uint128 amount0, uint128 amount1)
        internal
        view
        virtual
        returns (uint128 protocolFee0, uint128 protocolFee1);

    /// @notice Handles lock callback data for position operations
    /// @dev Internal function that processes different types of position operations
    /// @param data Encoded operation data
    /// @return result Encoded result data
    function handleLockData(uint256, bytes memory data) internal override returns (bytes memory result) {
        uint256 callType = abi.decode(data, (uint256));

        if (callType == CALL_TYPE_DEPOSIT) {
            (
                ,
                address caller,
                uint256 id,
                PoolKey memory poolKey,
                int32 tickLower,
                int32 tickUpper,
                uint128 liquidity
            ) = abi.decode(data, (uint256, address, uint256, PoolKey, int32, int32, uint128));

            PoolBalanceUpdate balanceUpdate = CORE.updatePosition(
                poolKey,
                createPositionId({_salt: bytes24(uint192(id)), _tickLower: tickLower, _tickUpper: tickUpper}),
                int128(liquidity)
            );

            uint128 amount0 = uint128(balanceUpdate.delta0());
            uint128 amount1 = uint128(balanceUpdate.delta1());

            // Use multi-token payment for ERC20-only pools, fall back to individual payments for native token pools
            if (poolKey.token0 != NATIVE_TOKEN_ADDRESS) {
                ACCOUNTANT.payTwoFrom(caller, poolKey.token0, poolKey.token1, amount0, amount1);
            } else {
                if (amount0 != 0) {
                    SafeTransferLib.safeTransferETH(address(ACCOUNTANT), amount0);
                }
                if (amount1 != 0) {
                    ACCOUNTANT.payFrom(caller, poolKey.token1, amount1);
                }
            }

            result = abi.encode(amount0, amount1);
        } else if (callType == CALL_TYPE_WITHDRAW) {
            (
                ,
                uint256 id,
                PoolKey memory poolKey,
                int32 tickLower,
                int32 tickUpper,
                uint128 liquidity,
                address recipient,
                bool withFees
            ) = abi.decode(data, (uint256, uint256, PoolKey, int32, int32, uint128, address, bool));

            if (liquidity > uint128(type(int128).max)) revert WithdrawOverflow();

            uint128 amount0;
            uint128 amount1;

            // collect first in case we are withdrawing the entire amount
            if (withFees) {
                (amount0, amount1) = CORE.collectFees(
                    poolKey,
                    createPositionId({_salt: bytes24(uint192(id)), _tickLower: tickLower, _tickUpper: tickUpper})
                );

                // Collect swap protocol fees
                (uint128 swapProtocolFee0, uint128 swapProtocolFee1) =
                    _computeSwapProtocolFees(poolKey, amount0, amount1);

                if (swapProtocolFee0 != 0 || swapProtocolFee1 != 0) {
                    CORE.updateSavedBalances(
                        poolKey.token0, poolKey.token1, bytes32(0), int128(swapProtocolFee0), int128(swapProtocolFee1)
                    );

                    amount0 -= swapProtocolFee0;
                    amount1 -= swapProtocolFee1;
                }
            }

            if (liquidity != 0) {
                PoolBalanceUpdate balanceUpdate = CORE.updatePosition(
                    poolKey,
                    createPositionId({_salt: bytes24(uint192(id)), _tickLower: tickLower, _tickUpper: tickUpper}),
                    -int128(liquidity)
                );

                uint128 withdrawnAmount0 = uint128(-balanceUpdate.delta0());
                uint128 withdrawnAmount1 = uint128(-balanceUpdate.delta1());

                // Collect withdrawal protocol fees
                (uint128 withdrawalFee0, uint128 withdrawalFee1) =
                    _computeWithdrawalProtocolFees(poolKey, withdrawnAmount0, withdrawnAmount1);

                if (withdrawalFee0 != 0 || withdrawalFee1 != 0) {
                    // we know cast won't overflow because delta0 and delta1 were originally int128
                    CORE.updateSavedBalances(
                        poolKey.token0, poolKey.token1, bytes32(0), int128(withdrawalFee0), int128(withdrawalFee1)
                    );
                }

                amount0 += withdrawnAmount0 - withdrawalFee0;
                amount1 += withdrawnAmount1 - withdrawalFee1;
            }

            ACCOUNTANT.withdrawTwo(poolKey.token0, poolKey.token1, recipient, amount0, amount1);

            result = abi.encode(amount0, amount1);
        } else if (callType == CALL_TYPE_WITHDRAW_PROTOCOL_FEES) {
            (, address token0, address token1, uint128 amount0, uint128 amount1, address recipient) =
                abi.decode(data, (uint256, address, address, uint128, uint128, address));

            CORE.updateSavedBalances(token0, token1, bytes32(0), -int256(uint256(amount0)), -int256(uint256(amount1)));
            ACCOUNTANT.withdrawTwo(token0, token1, recipient, amount0, amount1);
        } else {
            // Will never actually happen
            revert();
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {DropKey} from "../types/dropKey.sol";
import {ClaimKey} from "../types/claimKey.sol";
import {IExposedStorage} from "./IExposedStorage.sol";

/// @title Incentives Interface
/// @notice Interface for the Incentives contract that manages airdrops
/// @dev Inherits from IExposedStorage to allow direct storage access
interface IIncentives is IExposedStorage {
    /// @notice Emitted when a drop is funded
    /// @param key The drop key that was funded
    /// @param amountNext The new total funded amount
    event Funded(DropKey key, uint128 amountNext);

    /// @notice Emitted when a drop is refunded
    /// @param key The drop key that was refunded
    /// @param refundAmount The amount that was refunded
    event Refunded(DropKey key, uint128 refundAmount);

    /// @notice Thrown if the claim has already happened for this drop
    error AlreadyClaimed();

    /// @notice Thrown if the merkle proof does not correspond to the root
    error InvalidProof();

    /// @notice Thrown if the drop is not sufficiently funded for the claim
    error InsufficientFunds();

    /// @notice Only the drop owner may call this function
    error DropOwnerOnly();

    /// @notice Funds a drop to a minimum amount
    /// @param key The drop key to fund
    /// @param minimum The minimum amount to fund to
    /// @return fundedAmount The amount that was actually funded
    function fund(DropKey memory key, uint128 minimum) external returns (uint128 fundedAmount);

    /// @notice Refunds the remaining amount from a drop to the owner
    /// @param key The drop key to refund
    /// @return refundAmount The amount that was refunded
    function refund(DropKey memory key) external returns (uint128 refundAmount);

    /// @notice Claims tokens from a drop using a merkle proof
    /// @param key The drop key to claim from
    /// @param c The claim details
    /// @param proof The merkle proof for the claim
    function claim(DropKey memory key, ClaimKey memory c, bytes32[] calldata proof) external;
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

