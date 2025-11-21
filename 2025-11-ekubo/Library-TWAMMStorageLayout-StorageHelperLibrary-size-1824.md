
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolId} from "../types/poolId.sol";
import {OrderId} from "../types/orderId.sol";
import {StorageSlot} from "../types/storageSlot.sol";

/// @title TWAMM Storage Layout
/// @notice Library providing functions to compute the storage locations for the TWAMM contract
/// @dev TWAMM uses a custom storage layout to avoid keccak's where possible.
///      For certain storage values, the pool id is used as a base offset and
///      we allocate the following relative offsets (starting from the pool id) as:
///        0: pool state
///        [REWARD_RATES_OFFSET, REWARD_RATES_OFFSET + 1]: global reward rates
///        [TIME_BITMAPS_OFFSET, TIME_BITMAPS_OFFSET + type(uint52).max]: initialized times bitmaps
///        [TIME_INFOS_OFFSET, TIME_INFOS_OFFSET + type(uint64).max]: time infos
///        [REWARD_RATES_BEFORE_OFFSET, REWARD_RATES_BEFORE_OFFSET + 2 * type(uint64).max]: reward rates before time
library TWAMMStorageLayout {
    /// @dev Generated using: cast keccak "TWAMMStorageLayout#REWARD_RATES_OFFSET"
    uint256 internal constant REWARD_RATES_OFFSET = 0x6536a49ed1752ddb42ba94b6b00660382279a8d99d650d701d5d127e7a3bbd95;
    /// @dev Generated using: cast keccak "TWAMMStorageLayout#TIME_BITMAPS_OFFSET"
    uint256 internal constant TIME_BITMAPS_OFFSET = 0x07f3f693b68a1a1b1b3315d4b74217931d60e9dc7f1af4989f50e7ab31c8820e;
    /// @dev Generated using: cast keccak "TWAMMStorageLayout#TIME_INFOS_OFFSET"
    uint256 internal constant TIME_INFOS_OFFSET = 0x70db18ef1c685b7aa06d1ac5ea2d101c7261974df22a15951f768f92187043fb;
    /// @dev Generated using: cast keccak "TWAMMStorageLayout#REWARD_RATES_BEFORE_OFFSET"
    uint256 internal constant REWARD_RATES_BEFORE_OFFSET =
        0x6a7cb7181a18ced052a38531ee9ccb088f76cd0fb0c4475d55c480aebfae7b2b;
    /// @dev Generated using: cast keccak "TWAMMStorageLayout#ORDER_STATE_OFFSET"
    uint256 internal constant ORDER_STATE_OFFSET = 0xdc028e0b30217dc4c47f0ed37f8e3d64faf5fcf0199e7e05f83775072aa91e8d;

    /// @notice Computes the storage slot of the TWAMM pool state
    /// @param poolId The unique identifier for the pool
    /// @return slot The storage slot in the TWAMM contract
    function twammPoolStateSlot(PoolId poolId) internal pure returns (StorageSlot slot) {
        slot = StorageSlot.wrap(PoolId.unwrap(poolId));
    }

    /// @notice Computes the first storage slot of the reward rates of a pool
    /// @param poolId The unique identifier for the pool
    /// @return firstSlot The first of two consecutive storage slots in the TWAMM contract
    function poolRewardRatesSlot(PoolId poolId) internal pure returns (StorageSlot firstSlot) {
        assembly ("memory-safe") {
            firstSlot := add(poolId, REWARD_RATES_OFFSET)
        }
    }

    /// @notice Computes the storage slot of the first word of an initialized times bitmap for a given pool
    /// @param poolId The unique identifier for the pool
    /// @return firstSlot The first storage slot in the TWAMM contract
    function poolInitializedTimesBitmapSlot(PoolId poolId) internal pure returns (StorageSlot firstSlot) {
        assembly ("memory-safe") {
            firstSlot := add(poolId, TIME_BITMAPS_OFFSET)
        }
    }

    /// @notice Computes the storage slot of time info for a specific time
    /// @param poolId The unique identifier for the pool
    /// @param time The timestamp to query
    /// @return slot The storage slot in the TWAMM contract
    function poolTimeInfosSlot(PoolId poolId, uint256 time) internal pure returns (StorageSlot slot) {
        assembly ("memory-safe") {
            slot := add(poolId, add(TIME_INFOS_OFFSET, time))
        }
    }

    /// @notice Computes the storage slot of the pool reward rates before a given time
    /// @param poolId The unique identifier for the pool
    /// @param time The time to query
    /// @return firstSlot The first of two consecutive storage slots in the TWAMM contract
    function poolRewardRatesBeforeSlot(PoolId poolId, uint256 time) internal pure returns (StorageSlot firstSlot) {
        assembly ("memory-safe") {
            firstSlot := add(poolId, add(REWARD_RATES_BEFORE_OFFSET, mul(time, 2)))
        }
    }

    /// @notice Computes the storage slot of the order state, followed by the order reward rate snapshot for a specific order
    /// @param owner The order owner
    /// @param salt The salt used for the order
    /// @param orderId The unique identifier for the order
    /// @return slot The storage slot of the order state in the TWAMM contract, followed by the storage slot of the order reward rate snapshot
    function orderStateSlotFollowedByOrderRewardRateSnapshotSlot(address owner, bytes32 salt, OrderId orderId)
        internal
        pure
        returns (StorageSlot slot)
    {
        assembly ("memory-safe") {
            let free := mload(0x40)
            mstore(free, owner)
            mstore(add(free, 0x20), salt)
            mstore(add(free, 0x40), orderId)
            slot := add(keccak256(free, 96), ORDER_STATE_OFFSET)
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

/// @notice Unique identifier for a TWAMM order
/// @dev Wraps bytes32 to provide type safety for order identifiers
type OrderId is bytes32;

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


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

