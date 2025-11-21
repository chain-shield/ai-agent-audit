
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {SafeTransferLib} from "@solady/utils/SafeTransferLib.sol";

import {PackedFeeRates, PackedFeeRatesLib} from "./PackedFeeRatesLib.sol";
import {FeeTier} from "./Enums.sol";

struct FeeManager {
    mapping(address account => FeeTier) accountFeeTier;
    PackedFeeRates takerFeeRates;
    PackedFeeRates makerFeeRates;
}

using FeeManagerLib for FeeManager global;

library FeeManagerLib {
    using FixedPointMathLib for uint256;

    uint256 constant FEE_SCALING = 10_000_000;

    function setAccountFeeTier(FeeManager storage self, address account, FeeTier feeTier) internal {
        self.accountFeeTier[account] = feeTier;
    }

    function setTakerFeeRates(FeeManager storage self, uint16[] memory takerFeeRates) internal {
        self.takerFeeRates = PackedFeeRatesLib.packFeeRates(takerFeeRates);
    }

    function setMakerFeeRates(FeeManager storage self, uint16[] memory makerFeeRates) internal {
        self.makerFeeRates = PackedFeeRatesLib.packFeeRates(makerFeeRates);
    }

    function getTakerFee(FeeManager storage self, address account, uint256 amount) internal view returns (uint256) {
        if (amount == 0) return 0;

        uint16 feeRate = self.getTakerFeeRate(account);
        return amount.fullMulDiv(feeRate, FEE_SCALING);
    }

    function getMakerFee(FeeManager storage self, address account, uint256 amount) internal view returns (uint256) {
        if (amount == 0) return 0;

        uint16 feeRate = self.getMakerFeeRate(account);
        return amount.fullMulDiv(feeRate, FEE_SCALING);
    }

    function getTakerFeeRate(FeeManager storage self, address account) internal view returns (uint16 feeRate) {
        return self.takerFeeRates.getFeeAt(uint256(self.accountFeeTier[account]));
    }

    function getMakerFeeRate(FeeManager storage self, address account) internal view returns (uint16 feeRate) {
        return self.makerFeeRates.getFeeAt(uint256(self.accountFeeTier[account]));
    }

    function getAccountFeeTier(FeeManager storage self, address account) internal view returns (FeeTier tier) {
        return self.accountFeeTier[account];
    }

    function getAccountTakerFeeRate(FeeManager storage self, address account) internal view returns (uint16 feeRate) {
        return self.takerFeeRates.getFeeAt(uint256(self.accountFeeTier[account]));
    }

    function getAccountMakerFeeRate(FeeManager storage self, address account) internal view returns (uint16 feeRate) {
        return self.makerFeeRates.getFeeAt(uint256(self.accountFeeTier[account]));
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {EventNonceLib as FeeDataEventNonce} from "contracts/utils/types/EventNonce.sol";

import {SafeTransferLib} from "@solady/utils/SafeTransferLib.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";

type PackedFeeRates is uint256;

using PackedFeeRatesLib for PackedFeeRates global;

library PackedFeeRatesLib {
    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ERRORS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0x39bdbb10
    error FeeTiersExceedsMax();
    /// @dev sig: 0x8e516923
    error FeeTierIndexOutOfBounds();

    uint256 private constant U16_PER_WORD = 16;

    function packFeeRates(uint16[] memory fees) internal pure returns (PackedFeeRates) {
        if (fees.length > U16_PER_WORD) revert FeeTiersExceedsMax();

        uint256 packedValue = 0;
        for (uint256 i; i < fees.length; i++) {
            packedValue = packedValue | (uint256(fees[i]) << (i * U16_PER_WORD));
        }

        return PackedFeeRates.wrap(packedValue);
    }

    function getFeeAt(PackedFeeRates fees, uint256 index) internal pure returns (uint16) {
        if (index >= 15) revert FeeTierIndexOutOfBounds();

        uint256 shiftBits = index * U16_PER_WORD;

        return uint16((PackedFeeRates.unwrap(fees) >> shiftBits) & 0xFFFF);
    }
}

enum FeeTiers {
    ZERO,
    ONE,
    TWO
}

struct FeeData {
    mapping(address token => uint256) totalFees;
    mapping(address token => uint256) unclaimedFees;
    mapping(address account => FeeTiers) accountFeeTier;
}

using FeeDataLib for FeeData global;

/// @custom:storage-location erc7201:FeeDataStorage
library FeeDataStorageLib {
    bytes32 constant FEE_DATA_STORAGE_POSITION =
        keccak256(abi.encode(uint256(keccak256("FeeDataStorage")) - 1)) & ~bytes32(uint256(0xff));

    /// @dev Gets the storage slot of the FeeData struct
    // slither-disable-next-line uninitialized-storage
    function getFeeDataStorage() internal pure returns (FeeData storage self) {
        bytes32 position = FEE_DATA_STORAGE_POSITION;

        // slither-disable-next-line assembly
        assembly {
            self.slot := position
        }
    }
}

library FeeDataLib {
    using PackedFeeRatesLib for PackedFeeRates;
    using FixedPointMathLib for uint256;
    using SafeTransferLib for address;

    /// @dev sig: 0x2227733fc4c8a9034cb58087dcf6995128b9c0233b038b03366aaf30c92b92d6
    event FeesClaimed(uint256 indexed eventNonce, address indexed token, uint256 fee);
    /// @dev sig: 0xfaa858b3dfeba08d811f5f70b037ea5cb20192ab57f696df5a74a281ef22751b
    event AccountFeeTierUpdated(uint256 indexed eventNonce, address indexed account, FeeTiers newTier);
    /// @dev sig: 0x91865da290f8efd7332deaf04dfb3d8fdcf887d7d5d9e55b2bd72c932c939b32
    event FeesAccrued(uint256 indexed eventNonce, address indexed token, uint256 amount);

    uint256 constant FEE_SCALING = 10_000_000;

    /// @dev Returns the taker fee for a given amount and account
    function getTakerFee(FeeData storage self, PackedFeeRates takerRates, address account, uint256 amount)
        internal
        view
        returns (uint256)
    {
        if (amount == 0) return 0;

        uint16 feeRate = takerRates.getFeeAt(uint256(self.accountFeeTier[account]));
        return amount.fullMulDiv(feeRate, FEE_SCALING);
    }

    /// @dev Returns the maker fee for a given amount and account
    function getMakerFee(FeeData storage self, PackedFeeRates makerRates, address account, uint256 amount)
        internal
        view
        returns (uint256)
    {
        if (amount == 0) return 0;

        uint16 feeRate = makerRates.getFeeAt(uint256(self.accountFeeTier[account]));
        return amount.fullMulDiv(feeRate, FEE_SCALING);
    }

    /// @dev Returns the fee tier for a given account
    function getAccountFeeTier(FeeData storage self, address account) internal view returns (FeeTiers tier) {
        return self.accountFeeTier[account];
    }

    /// @dev Sets the fee tier for a given account
    function setAccountFeeTier(FeeData storage self, address account, FeeTiers feeTier) internal {
        self.accountFeeTier[account] = feeTier;

        emit AccountFeeTierUpdated(FeeDataEventNonce.inc(), account, feeTier);
    }

    /// @dev Accrues fees for a given token
    function accrueFee(FeeData storage self, address token, uint256 amount) internal {
        self.totalFees[token] += amount;
        self.unclaimedFees[token] += amount;

        emit FeesAccrued(FeeDataEventNonce.inc(), token, amount);
    }

    /// @dev Claims fees for a given token
    function claimFees(FeeData storage self, address token) internal returns (uint256 fees) {
        fees = self.unclaimedFees[token];
        delete self.unclaimedFees[token];

        emit FeesClaimed(FeeDataEventNonce.inc(), token, fees);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

/**
 * @title EventNonce
 * @notice Shared event nonce management for tracking event ordering offchain
 * @dev Uses ERC-7201 specifically for shared access across a contract's inheritance graph
 */
struct EventNonceStorage {
    uint256 eventNonce;
}

/// @custom:storage-location erc7201:EventNonceStorage
library EventNonceLib {
    bytes32 constant EVENT_NONCE_STORAGE_POSITION =
        keccak256(abi.encode(uint256(keccak256("EventNonceStorage")) - 1)) & ~bytes32(uint256(0xff));

    // slither-disable-next-line uninitialized-storage
    function getEventNonceStorage() internal pure returns (EventNonceStorage storage ds) {
        bytes32 position = EVENT_NONCE_STORAGE_POSITION;

        // slither-disable-next-line assembly
        assembly {
            ds.slot := position
        }
    }

    /// @notice Increments and returns the event nonce
    /// @return The new event nonce value
    function inc() internal returns (uint256) {
        EventNonceStorage storage ds = getEventNonceStorage();
        return ++ds.eventNonce;
    }

    /// @notice Gets the current event nonce without incrementing
    /// @return The current event nonce value
    function getCurrentNonce() internal view returns (uint256) {
        EventNonceStorage storage ds = getEventNonceStorage();
        return ds.eventNonce;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

enum Side {
    BUY,
    SELL
}

enum TiF {
    // MAKER
    GTC, // good-till-cancelled
    MOC, // maker-or-cancel (post-only)
    // TAKER
    FOK, // fill-or-kill
    IOC // immediate-or-cancel

}

enum Status {
    NULL,
    INACTIVE,
    ACTIVE,
    DELISTED
}

enum FeeTier {
    ZERO,
    ONE,
    TWO
}

enum BookType {
    STANDARD,
    BACKSTOP
}

enum TradeType {
    TAKER,
    MAKER,
    LIQUIDATOR,
    LIQUIDATEE,
    DELEVERAGE_MAKER,
    DELEVERAGE_TAKER,
    DELIST
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

type PackedFeeRates is uint256;

using PackedFeeRatesLib for PackedFeeRates global;

library PackedFeeRatesLib {
    /// @dev sig: 0x08498ba1
    error TooManyFeeTiers();
    /// @dev sig: 0x4e23d035
    error IndexOutOfBounds();

    function packFeeRates(uint16[] memory fees) internal pure returns (PackedFeeRates) {
        if (fees.length > 15) revert TooManyFeeTiers();

        uint256 packedValue;
        for (uint256 i; i < fees.length; i++) {
            packedValue = packedValue | (uint256(fees[i]) << (i * 16));
        }

        return PackedFeeRates.wrap(packedValue);
    }

    function getFeeAt(PackedFeeRates fees, uint256 index) internal pure returns (uint16) {
        if (index > 15) revert IndexOutOfBounds();

        uint256 shiftBits = index * 16;

        return uint16((PackedFeeRates.unwrap(fees) >> shiftBits) & 0xFFFF);
    }
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

