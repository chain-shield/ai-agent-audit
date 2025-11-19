
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {ClearingHouse} from "./ClearingHouse.sol";
import {InsuranceFund} from "./InsuranceFund.sol";
import {CollateralManager} from "./CollateralManager.sol";
import {FeeManager} from "./FeeManager.sol";

import {Market, MarketSettings, MarketMetadata} from "./Market.sol";
import {FundingRateEngine, FundingRateSettings} from "./FundingRateEngine.sol";

import {Book, BookConfig, BookSettings, BookMetadata} from "./Book.sol";
import {BookType} from "./Enums.sol";

// @todo change get to load & take loads out of ClearingHouseLib

library StorageLib {
    /// erc7201('ClearingHouse')
    bytes32 constant CLEARING_HOUSE_SLOT = 0x82401ef06211501256a876d252aaf61e7132ccc51e18716e2d709a0d4272e700;
    /// erc7201('InsuranceFund')
    bytes32 constant INSURANCE_FUND_SLOT = 0xbfd5935e9ce192860479583c8f68d8f0281e1b205c9b51903c37fd1663caf700;
    /// erc7201('CollateralManager')
    bytes32 constant COLLATERAL_MANAGER_SLOT = 0x61b9ccef1e220863792471c905db5592dea4de72f361956c0ad957095e951f00;
    /// erc7201('FeeManager')
    bytes32 constant FEE_MANAGER_SLOT = 0x342baed097735cb285ac1652589d9be5e07986ffa1048894c329a3e87d336000;

    /// erc7201('MarketSettings')
    bytes32 constant MARKET_SETTINGS_SLOT = 0xabab056a6b37dca48028a49dc141d38e864363077235e1ceedd891a9da3d5700;
    /// erc7201('MarketMetadata')
    bytes32 constant MARKET_METADATA_SLOT = 0x924d635e09fb0ed4d506fa4757253ad18d1012b6778f35eaca050f36795c0e00;
    /// erc7201('FundingRateEngine')
    bytes32 constant FUNDING_RATE_ENGINE_SLOT = 0x617f70bdcfb1b30f7368b905448126d45e4211d49d45d0b890adb64417867a00;
    /// erc7201('FundingRateSettings')
    bytes32 constant FUNDING_RATE_SETTINGS_SLOT = 0x2d9df79ce2a04bace979c8e7822d5d58e0eba86f9b6d650a53d036070e79e300;

    /// erc7201('Book')
    bytes32 constant PERP_CLOB_SLOT = 0xa57a5c98162987d0c55c599afa286778f3124669c2f7ee0229f5fa9d51839700;
    /// erc7201('BookConfig')
    bytes32 constant BOOK_CONFIG_SLOT = 0x9664b91c31ceff59d9f1ffab6c8af23eb35df7e5770fbcfcb63ce9d0c5f3d600;
    /// erc7201('BookSettings')
    bytes32 constant BOOK_SETTINGS_SLOT = 0xfd97e8e280d3f806a8f248b702ebf7f7d42962451433a0b02180a11a7d773b00;
    /// erc7201('BookMetadata')
    bytes32 constant BOOK_METADATA_SLOT = 0x96ac35e14db2dbf70714b88de0d8321e86e7af2b879d88c55718ded69e62bf00;

    /// erc7201('EventNonce')
    bytes32 constant EVENT_NONCE_SLOT = 0x00f57b92438c2add21322de9585c2e64b6631becda92262d6e63a910f44abd00;

    /*//////////////////////////////////////////////////////////////
                             CLEARINGHOUSE
    //////////////////////////////////////////////////////////////*/

    function loadClearingHouse() internal pure returns (ClearingHouse storage ch) {
        bytes32 slot = CLEARING_HOUSE_SLOT;

        assembly {
            ch.slot := slot
        }
    }

    function loadInsuranceFund() internal pure returns (InsuranceFund storage insuranceFund) {
        bytes32 slot = INSURANCE_FUND_SLOT;

        assembly {
            insuranceFund.slot := slot
        }
    }

    function loadCollateralManager() internal pure returns (CollateralManager storage collateralManager) {
        bytes32 slot = COLLATERAL_MANAGER_SLOT;

        assembly {
            collateralManager.slot := slot
        }
    }

    function loadFeeManager() internal pure returns (FeeManager storage feeManager) {
        bytes32 slot = FEE_MANAGER_SLOT;

        assembly {
            feeManager.slot := slot
        }
    }

    /*//////////////////////////////////////////////////////////////
                                 MARKET
    //////////////////////////////////////////////////////////////*/

    function loadMarket(bytes32 asset) internal view returns (Market storage market) {
        return loadClearingHouse().market[asset];
    }

    function loadMarketSettings(bytes32 asset) internal pure returns (MarketSettings storage marketSettings) {
        bytes32 slot = keccak256(abi.encode(asset, MARKET_SETTINGS_SLOT));

        assembly {
            marketSettings.slot := slot
        }
    }

    function loadMarketMetadata(bytes32 asset) internal pure returns (MarketMetadata storage marketMetadata) {
        bytes32 slot = keccak256(abi.encode(asset, MARKET_METADATA_SLOT));

        assembly {
            marketMetadata.slot := slot
        }
    }

    function loadFundingRateEngine(bytes32 asset) internal pure returns (FundingRateEngine storage fundingRateEngine) {
        bytes32 slot = keccak256(abi.encode(asset, FUNDING_RATE_ENGINE_SLOT));

        assembly {
            fundingRateEngine.slot := slot
        }
    }

    function loadFundingRateSettings(bytes32 asset) internal pure returns (FundingRateSettings storage settings) {
        bytes32 slot = keccak256(abi.encode(asset, FUNDING_RATE_SETTINGS_SLOT));

        assembly {
            settings.slot := slot
        }
    }

    /*//////////////////////////////////////////////////////////////
                                  BOOK
    //////////////////////////////////////////////////////////////*/

    function loadBook(bytes32 asset) internal pure returns (Book storage ds) {
        bytes32 assetSlot = keccak256(abi.encode(uint256(keccak256(abi.encode(asset))) - 1)) & ~bytes32(uint256(0xff));

        // note: simulates a PerpBook => asset mapping
        bytes32 slot = keccak256(abi.encode(BookType.STANDARD, assetSlot, PERP_CLOB_SLOT));

        assembly {
            ds.slot := slot
        }
    }

    function loadBackstopBook(bytes32 asset) internal pure returns (Book storage ds) {
        bytes32 assetSlot = keccak256(abi.encode(uint256(keccak256(abi.encode(asset))) - 1)) & ~bytes32(uint256(0xff));

        // note: simulates a PerpBook => asset mapping
        bytes32 slot = keccak256(abi.encode(BookType.BACKSTOP, assetSlot, PERP_CLOB_SLOT));

        assembly {
            ds.slot := slot
        }
    }

    function loadBook(bytes32 asset, BookType bookType) internal pure returns (Book storage ds) {
        bytes32 assetSlot = keccak256(abi.encode(uint256(keccak256(abi.encode(asset))) - 1)) & ~bytes32(uint256(0xff));

        // asset => book type => book mapping
        bytes32 slot = keccak256(abi.encode(bookType, assetSlot, PERP_CLOB_SLOT));

        assembly {
            ds.slot := slot
        }
    }

    function loadBookConfig(bytes32 asset) internal pure returns (BookConfig storage bookConfig) {
        bytes32 slot = keccak256(abi.encode(asset, BOOK_CONFIG_SLOT));

        assembly {
            bookConfig.slot := slot
        }
    }

    function loadBookSettings(bytes32 asset) internal pure returns (BookSettings storage bookSettings) {
        bytes32 slot = keccak256(abi.encode(asset, BOOK_SETTINGS_SLOT));

        assembly {
            bookSettings.slot := slot
        }
    }

    /*//////////////////////////////////////////////////////////////
                                 NONCE
    //////////////////////////////////////////////////////////////*/

    function incNonce() internal returns (uint256 n) {
        bytes32 slot = EVENT_NONCE_SLOT;

        assembly {
            n := add(sload(slot), 1)
            sstore(slot, n)
        }
    }

    function loadNonce() internal view returns (uint256 n) {
        bytes32 slot = EVENT_NONCE_SLOT;

        assembly {
            n := sload(slot)
        }
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";

struct PriceHistory {
    PriceSnapshot[] snapshots;
}

struct PriceSnapshot {
    uint256 price;
    int256 basisSpread;
    uint256 timestamp;
}

using PriceHistoryLib for PriceHistory global;

library PriceHistoryLib {
    using FixedPointMathLib for uint256;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                SETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice snapshots
    function snapshot(PriceHistory storage history, uint256 price) internal {
        uint256 length = history.snapshots.length;

        if (length > 0 && history.snapshots[length - 1].timestamp == block.timestamp) {
            history.snapshots[length - 1].price = price;
        } else {
            history.snapshots.push(PriceSnapshot(price, 0, block.timestamp));
        }
    }

    function snapshotBasisSpread(PriceHistory storage history, int256 basisSpread) internal {
        history.snapshots.push(PriceSnapshot(0, basisSpread, 0));
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function latest(PriceHistory storage history) internal view returns (uint256) {
        uint256 length = history.snapshots.length;

        if (length == 0) return 0;

        return history.snapshots[length - 1].price;
    }

    function twap(PriceHistory storage history, uint256 twapInterval) internal view returns (uint256) {
        uint256 idx = history.snapshots.length;

        if (idx == 0) return 0;

        PriceSnapshot memory currentSnapshot = history.snapshots[--idx];

        if (idx == 0) return currentSnapshot.price;

        uint256 targetTime = block.timestamp - twapInterval;
        uint256 timePeriod = block.timestamp - currentSnapshot.timestamp;
        uint256 elapsedTime = timePeriod;
        uint256 weightedPrice = currentSnapshot.price * timePeriod;
        uint256 previousTime = currentSnapshot.timestamp;

        while (currentSnapshot.timestamp > targetTime) {
            // history is too short
            if (idx == 0) break;

            currentSnapshot = history.snapshots[--idx];

            if (currentSnapshot.timestamp < targetTime) {
                // if snapshot is before target time, bound the time period
                elapsedTime += timePeriod = previousTime - targetTime;
            } else {
                elapsedTime += timePeriod = previousTime - currentSnapshot.timestamp;
            }

            weightedPrice += currentSnapshot.price * timePeriod;
            previousTime = currentSnapshot.timestamp;
        }

        return weightedPrice / elapsedTime;
    }

    /// @notice returns ema of basis spread
    function ema(PriceHistory storage history, uint256 period) internal view returns (int256) {
        uint256 n = history.snapshots.length;
        if (n == 0 || period == 0) return 0;

        // only consider up to `period` most recent entries
        uint256 count = period <= n ? period : n;
        uint256 start = n - count;

        int256 k = (2 * 1e18) / (int256(count) + 1);

        // initialize EMA using the first value in the slice (scaled)
        int256 _ema = history.snapshots[start].basisSpread * 1e18;

        // apply EMA formula over the remaining `count - 1` entries
        for (uint256 i = start + 1; i < n; i++) {
            int256 pWad = history.snapshots[i].basisSpread * 1e18;
            _ema = (pWad * k + _ema * (1e18 - k)) / 1e18;
        }

        // Return unscaled EMA value
        return _ema / 1e18;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {SafeTransferLib} from "@solady/utils/SafeTransferLib.sol";

import {Constants} from "./Constants.sol";

struct InsuranceFund {
    uint256 balance;
}

using InsuranceFundLib for InsuranceFund global;

library InsuranceFundLib {
    using SafeTransferLib for address;

    address constant USDC = Constants.USDC;

    event InsuranceFundWithdrawal(address indexed account, uint256 amount);
    event InsuranceFundDeposit(address indexed account, uint256 amount);

    error InsufficientInsuranceFundBalance();

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               INSURANCE
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function pay(InsuranceFund storage self, uint256 amount) internal {
        if (amount == 0) return;
        assembly {
            let currentBalance := sload(self.slot)
            let newBalance := add(currentBalance, amount)
            sstore(self.slot, newBalance)
        }
    }

    function claim(InsuranceFund storage self, uint256 amount) internal {
        if (amount == 0) return;
        if (self.balance < amount) revert InsufficientInsuranceFundBalance();
        assembly {
            let currentBalance := sload(self.slot)
            let newBalance := sub(currentBalance, amount)
            sstore(self.slot, newBalance)
        }
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                 ADMIN
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function withdraw(InsuranceFund storage self, uint256 amount) internal {
        if (self.balance < amount) revert InsufficientInsuranceFundBalance();
        assembly {
            let currentBalance := sload(self.slot)
            let newBalance := sub(currentBalance, amount)
            sstore(self.slot, newBalance)
        }
        USDC.safeTransfer(msg.sender, amount);
        emit InsuranceFundWithdrawal(msg.sender, amount);
    }

    function deposit(InsuranceFund storage self, uint256 amount) internal {
        assembly {
            let currentBalance := sload(self.slot)
            let newBalance := add(currentBalance, amount)
            sstore(self.slot, newBalance)
        }
        USDC.safeTransferFrom(msg.sender, address(this), amount);
        emit InsuranceFundDeposit(msg.sender, amount);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getBalance(
        InsuranceFund storage self
    ) internal view returns (uint256) {
        return self.balance;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Side, TiF, Status, TradeType, BookType} from "./Enums.sol";
import {Position} from "./Position.sol";

/*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        MARKET CREATION
▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

struct MarketParams {
    uint256 maxOpenLeverage; // 1e18 = 1x
    uint256 maintenanceMarginRatio; // 0.5e18 = 50%
    uint256 liquidationFeeRate; // .01e18 = 1%
    uint256 divergenceCap; // 0.1e18 = trades can occur at max 10% price from mark
    uint256 reduceOnlyCap; // max number of reduce only orders per subaccount
    uint256 partialLiquidationThreshold; // 20_000e18 = positions worth $20k and over will be partially liquidated
    uint256 partialLiquidationRate; // 0.2e18 = 20% of position will be liquidated on partial liquidation
    bool crossMarginEnabled; // true if there can be more than 1 position open per subaccount
    uint256 fundingInterval;
    uint256 resetInterval;
    uint256 resetIterations;
    uint256 innerClamp;
    uint256 outerClamp;
    int256 interestRate;
    uint256 maxNumOrders; // max number of orders per book
    uint8 maxLimitsPerTx; // max number of limit orders per transaction
    uint256 minLimitOrderAmountInBase; // minimum amount in base for limit orders
    uint256 tickSize; // 0.01e18 = 1 cent
    uint256 lotSize;
    uint256 initialPrice; // initial price of the market in quote token
}

/*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            ORDER POST
▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

struct PlaceOrderArgs {
    // account
    uint256 subaccount;
    // metadata
    bytes32 asset;
    Side side;
    // price
    uint256 limitPrice; // if 0, market order (system internally sets 0 ask or +inf bid)
    // size
    uint256 amount;
    bool baseDenominated; // true: amount in base; false: amount in quote
    // time / execution
    TiF tif; // time in force
    uint32 expiryTime; // optional auto-cancel time (only for GTC, MOC)
    // custom id tag
    uint96 clientOrderId;
    bool reduceOnly; // true if order is reduce-only
}

struct AmendLimitOrderArgs {
    bytes32 asset;
    uint256 subaccount;
    uint256 orderId;
    uint256 baseAmount;
    uint256 price;
    uint32 expiryTime;
    Side side;
    bool reduceOnly;
}

struct Condition {
    uint256 triggerPrice;
    bool stopLoss;
}

struct SignData {
    bytes sig;
    uint256 nonce;
    uint256 expiry;
}

/*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        EXTERNAL RESULT
▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

struct PlaceOrderResult {
    uint256 orderId;
    uint256 basePosted; // base posted on the book
    uint256 quoteTraded;
    uint256 baseTraded;
}

/*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        INTERNAL HELPERS
▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

struct MakerFillResult {
    bytes32 asset;
    BookType bookType;
    uint256 orderId;
    address maker;
    uint256 subaccount;
    Side side;
    uint256 quoteAmountTraded;
    uint256 baseAmountTraded;
    bool reduceOnly;
}

struct PositionUpdateResult {
    int256 marginDelta;
    int256 rpnl;
    bool sideClose;
    OIDelta oiDelta;
}

struct __TradeData__ {
    uint256 baseTraded;
    uint256 quoteTraded;
    uint256 filledAmount;
}

struct FundingPaymentResult {
    int256 fundingPayment;
    int256 marginDelta;
    uint256 debt;
}

struct TradeExecutedData {
    bytes32 asset;
    address account;
    uint256 subaccount;
    Side side;
    uint256 quoteTraded;
    uint256 baseTraded;
    Position position;
    int256 margin;
    int256 rpnl;
    uint256 fee;
    TradeType tradeType;
}

struct LiquidateData {
    uint256 fee;
    int256 rpnl;
    int256 marginDelta;
    uint256 debt;
}

struct BackstopLiquidateData {
    int256 rpnl;
    int256 marginDelta;
    uint256 debt;
}

struct MakerSettleData {
    address account;
    uint256 subaccount;
    int256 marginDelta;
    int256 collateralDelta;
    uint256 debt;
    uint256 makerFee;
    bool close;
}

struct LiquidateeSettleData {
    address account;
    uint256 subaccount;
    int256 marginDelta;
    uint256 debt;
    uint256 fee;
    bool fullLiquidation;
}

struct LiquidatorData {
    address liquidator;
    uint256 volume; // in quote
}

struct TakerSettleData {
    address account;
    uint256 subaccount;
    int256 marginDelta;
    int256 collateralDelta;
    uint256 debt;
    uint256 takerFee;
    bool close;
}

struct Account {
    address account;
    uint256 subaccount;
}

struct DeleveragePair {
    Account maker; // the underwater account in a deleverage
    Account taker; // the in profit account in a deleverage
}

struct OIDelta {
    int256 long;
    int256 short;
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";

import {RedBlackTree} from "../../clob/types/RedBlackTree.sol";
import {__TradeData__} from "./Structs.sol";
import {BookType, Side} from "./Enums.sol";
import {Order, OrderLib, OrderId, OrderIdLib} from "./Order.sol";
import {ClearingHouseLib} from "./ClearingHouse.sol";
import {StorageLib} from "./StorageLib.sol";

uint256 constant MIN_LIMIT_PRICE = 1;
uint256 constant MIN_FILL_ORDER_AMOUNT_BASE = 1;
uint256 constant MIN_MIN_LIMIT_ORDER_AMOUNT_BASE = 10;

struct BookConfig {
    bytes32 asset;
    uint256 lotSize;
    BookType bookType;
}

struct BookSettings {
    uint256 maxNumOrders;
    uint8 maxLimitsPerTx;
    uint256 minLimitOrderAmountInBase;
    uint256 tickSize;
}

struct BookMetadata {
    uint96 orderIdCounter;
    uint256 numBids;
    uint256 numAsks;
    uint256 baseOI;
    uint256 quoteOI;
}

struct Limit {
    uint64 numOrders;
    OrderId headOrder;
    OrderId tailOrder;
}

struct Book {
    BookConfig config;
    BookMetadata metadata;
    RedBlackTree bidTree; // header
    RedBlackTree askTree; // header
    mapping(OrderId => Order) orders;
    mapping(uint256 price => Limit) bidLimits; // header
    mapping(uint256 price => Limit) askLimits; // header
}

using BookLib for Book global;

library BookLib {
    using OrderIdLib for uint256;
    using FixedPointMathLib for uint256;

    error OrderPriceOutOfBounds();
    error LimitPriceOutOfBounds();
    error LimitOrderAmountNotOnLotSize();
    error LimitOrderAmountOutOfBounds();
    error NoOrdersAtLimit();
    error LimitsPlacedExceedsMaxThisTx();
    error InvalidMaxLimitsPerTx();
    error InvalidMinLimitOrderAmountInBase();
    error OrderIdInUse();

    bytes32 constant MAX_LIMIT_ALLOWLIST =
        keccak256(abi.encode(uint256(keccak256("MAX_LIMIT_ALLOWLIST")) - 1)) & ~bytes32(uint256(0xff));

    bytes32 constant TRANSIENT_LIMITS_PLACED =
        keccak256(abi.encode(uint256(keccak256("TRANSIENT_LIMITS_PLACED")) - 1)) & ~bytes32(uint256(0xff));

    // ASSERTIONS //

    function exists(Book storage self) internal view returns (bool) {
        return self.config.asset != bytes32(0);
    }

    function assertLimitPriceInBounds(Book storage self, uint256 price) internal view {
        uint256 tickSize = StorageLib.loadBookSettings(self.config.asset).tickSize;

        if (price == 0 || price % tickSize != 0) revert LimitPriceOutOfBounds();
    }

    function assertPriceInBounds(Book storage self, uint256 price) internal view {
        // zero price is ok for market orders
        if (price % StorageLib.loadBookSettings(self.config.asset).tickSize != 0) revert OrderPriceOutOfBounds();
    }

    function assertOrdersAtLimit(Book storage self, uint256 price, Side side) internal view {
        if (self.getLimit(price, side).numOrders == 0) revert NoOrdersAtLimit();
    }

    function assertLimitOrderAmountInBounds(Book storage self, uint256 orderAmountInBase) internal view {
        if (orderAmountInBase < StorageLib.loadBookSettings(self.config.asset).minLimitOrderAmountInBase) {
            revert LimitOrderAmountOutOfBounds();
        }
        if (orderAmountInBase % self.config.lotSize != 0) revert LimitOrderAmountNotOnLotSize();
    }

    function assertUnusedOrderId(Book storage self, uint256 orderId) internal view {
        if (self.orders[orderId.wrap()].owner != address(0)) revert OrderIdInUse();
    }

    // GETTERS //

    /// @dev Returns the highest bid price
    function getBestBid(Book storage self) internal view returns (uint256) {
        return self.bidTree.maximum();
    }

    /// @dev Returns the lowest ask price
    function getBestAsk(Book storage self) internal view returns (uint256) {
        return self.askTree.minimum();
    }

    /// @dev Returns the lowest bid price
    function getMinBidPrice(Book storage self) internal view returns (uint256) {
        return self.bidTree.minimum();
    }

    /// @dev Returns the highest ask price
    function getMaxAskPrice(Book storage self) internal view returns (uint256) {
        return self.askTree.maximum();
    }

    function getMaxLimitExempt(address who) internal view returns (bool allowed) {
        bytes32 slot = keccak256(abi.encode(MAX_LIMIT_ALLOWLIST, who));

        // slither-disable-next-line assembly
        assembly {
            allowed := sload(slot)
        }
    }

    function getLimit(Book storage self, uint256 price, Side side) internal view returns (Limit storage) {
        return side == Side.BUY ? self.bidLimits[price] : self.askLimits[price];
    }

    function getNextBiggestPrice(Book storage self, uint256 price, Side side) internal view returns (uint256) {
        return side == Side.BUY ? self.bidTree.getNextBiggest(price) : self.askTree.getNextBiggest(price);
    }

    function getNextSmallestPrice(Book storage self, uint256 price, Side side) internal view returns (uint256) {
        return side == Side.BUY ? self.bidTree.getNextSmallest(price) : self.askTree.getNextSmallest(price);
    }

    function getTradedAmounts(
        Book storage self,
        uint256 makerBase,
        uint256 takerAmount,
        uint256 price,
        bool baseDenominated
    ) internal view returns (__TradeData__ memory tradeData) {
        uint256 lotSize = self.config.lotSize;

        uint256 takerBase = baseDenominated ? takerAmount : takerAmount.fullMulDiv(1e18, price);

        takerBase -= tradeData.baseTraded = makerBase.min(takerBase) / lotSize * lotSize;
        tradeData.quoteTraded = tradeData.baseTraded.fullMulDiv(price, 1e18);

        if (takerBase < lotSize) {
            // filledAmount is only used to decrease the taker order amount — doesn't represent traded position
            // this prevents FOK orders from reverting on dust from lots & rounding errors when converting
            // quote -> base -> quote in quote denominated orders
            tradeData.filledAmount = takerAmount;
        } else {
            tradeData.filledAmount = baseDenominated ? tradeData.baseTraded : tradeData.quoteTraded;
        }
    }

    function boundToLots(Book storage self, uint256 baseAmount) internal view returns (uint256) {
        uint256 lotSize = self.config.lotSize;

        return baseAmount / lotSize * lotSize;
    }

    function getPostableBaseAmount(Book storage self, uint256 baseAmount)
        internal
        view
        returns (uint256 postableBaseAmount)
    {
        postableBaseAmount = self.boundToLots(baseAmount);

        if (postableBaseAmount < StorageLib.loadBookSettings(self.config.asset).minLimitOrderAmountInBase) return 0;
    }

    function quoteBidInBase(Book storage self, uint256 baseAmount)
        internal
        view
        returns (uint256 quoteAmount, uint256 baseUsed)
    {
        uint256 bestAsk = self.getBestAsk();

        uint256 quoteFromLimit;
        uint256 baseFromLimit;
        while (baseAmount > 0) {
            if (bestAsk == type(uint256).max) break;

            (quoteFromLimit, baseFromLimit) = _getQuoteLimit(self, self.askLimits[bestAsk], bestAsk, baseAmount);

            quoteAmount += quoteFromLimit;
            baseUsed += baseFromLimit;
            baseAmount -= baseFromLimit;
            bestAsk = self.getNextBiggestPrice(bestAsk, Side.SELL);
        }
    }

    function quoteBidInQuote(Book storage self, uint256 quoteAmount)
        internal
        view
        returns (uint256 baseAmount, uint256 quoteUsed)
    {
        uint256 bestAsk = self.getBestAsk();

        uint256 baseFromLimit;
        uint256 quoteFromLimit;
        while (quoteAmount > 0) {
            if (bestAsk == type(uint256).max) break;

            (baseFromLimit, quoteFromLimit) = _getBaseLimit(self, self.askLimits[bestAsk], bestAsk, quoteAmount);

            baseAmount += baseFromLimit;
            quoteUsed += quoteFromLimit;
            quoteAmount -= quoteFromLimit;
            bestAsk = self.getNextBiggestPrice(bestAsk, Side.SELL);
        }
    }

    function quoteAskInBase(Book storage self, uint256 baseAmount)
        internal
        view
        returns (uint256 quoteAmount, uint256 baseUsed)
    {
        uint256 bestBid = self.getBestBid();

        uint256 quoteFromLimit;
        uint256 baseFromLimit;
        while (baseAmount > 0) {
            if (bestBid == 0) break;

            (quoteFromLimit, baseFromLimit) = _getQuoteLimit(self, self.bidLimits[bestBid], bestBid, baseAmount);

            quoteAmount += quoteFromLimit;
            baseUsed += baseFromLimit;
            baseAmount -= baseFromLimit;
            bestBid = self.getNextSmallestPrice(bestBid, Side.BUY);
        }
    }

    function quoteAskInQuote(Book storage self, uint256 quoteAmount)
        internal
        view
        returns (uint256 baseAmount, uint256 quoteUsed)
    {
        uint256 bestBid = self.getBestBid();

        uint256 baseFromLimit;
        uint256 quoteFromLimit;
        while (quoteAmount > 0) {
            if (bestBid == 0) break;

            (baseFromLimit, quoteFromLimit) = _getBaseLimit(self, self.bidLimits[bestBid], bestBid, quoteAmount);

            baseAmount += baseFromLimit;
            quoteUsed += quoteFromLimit;
            quoteAmount -= quoteFromLimit;
            bestBid = self.getNextSmallestPrice(bestBid, Side.BUY);
        }
    }

    function getNextOrders(Book storage self, OrderId startOrderId, uint256 numOrders)
        internal
        view
        returns (Order[] memory)
    {
        Order storage currentOrder = self.orders[startOrderId];
        currentOrder.assertExists();

        uint256 count = 0;
        Order[] memory orders = new Order[](numOrders);

        while (count < numOrders && !currentOrder.isNull()) {
            orders[count] = currentOrder;
            count++;

            if (currentOrder.nextOrderId.unwrap() != 0) {
                currentOrder = self.orders[currentOrder.nextOrderId];
            } else {
                uint256 nextPrice = self.getNextBiggestPrice(currentOrder.price, currentOrder.side);

                if (nextPrice == 0) break;

                Limit storage nextLimit = self.getLimit(nextPrice, currentOrder.side);

                currentOrder = self.orders[nextLimit.headOrder];
            }
        }

        return orders;
    }

    function toOrderId(Book storage self, address account, uint96 clientOrderId) internal returns (uint256 orderId) {
        if (clientOrderId == 0) return self.incrementOrderId();

        orderId = OrderIdLib.getOrderId(account, clientOrderId);

        self.assertUnusedOrderId(orderId);
    }

    /// @dev returns incremented orderId
    function incrementOrderId(Book storage self) internal returns (uint256) {
        return ++self.metadata.orderIdCounter;
    }

    function setMaxLimitExempt(address who, bool toggle) internal {
        bytes32 slot = keccak256(abi.encode(MAX_LIMIT_ALLOWLIST, who));

        // slither-disable-next-line assembly
        assembly {
            sstore(slot, toggle)
        }
    }

    function setMaxLimitsPerTx(Book storage self, uint8 newMax) internal {
        if (newMax == 0) revert InvalidMaxLimitsPerTx();

        StorageLib.loadBookSettings(self.config.asset).maxLimitsPerTx = newMax;
    }

    function setMinLimitOrderAmountInBase(Book storage self, uint256 newLimitOrderAmountInBase) internal {
        if (newLimitOrderAmountInBase < MIN_MIN_LIMIT_ORDER_AMOUNT_BASE) revert InvalidMinLimitOrderAmountInBase();

        StorageLib.loadBookSettings(self.config.asset).minLimitOrderAmountInBase = newLimitOrderAmountInBase;
    }

    function _getTransientLimitsPlaced() private view returns (uint8 limitsPlaced) {
        bytes32 slot = TRANSIENT_LIMITS_PLACED;

        // This solidity version does not support the `transient` identifier
        // slither-disable-next-line assembly
        assembly {
            limitsPlaced := tload(slot)
        }
    }

    function incrementLimitsPlaced(Book storage self, address account) internal {
        uint8 limitsPlaced = _getTransientLimitsPlaced();

        if (limitsPlaced == StorageLib.loadBookSettings(self.config.asset).maxLimitsPerTx) {
            if (getMaxLimitExempt(account)) return;
            revert LimitsPlacedExceedsMaxThisTx();
        }

        bytes32 slot = TRANSIENT_LIMITS_PLACED;

        // This solidity version does not support the `transient` identifier
        // slither-disable-next-line assembly
        assembly {
            tstore(slot, add(limitsPlaced, 1))
        }
    }

    function addOrderToBook(Book storage self, Order memory order) internal {
        if (order.reduceOnly) {
            StorageLib.loadMarket(self.config.asset).linkReduceOnlyOrder(
                order.owner, order.subaccount, order.id.unwrap(), self.config.bookType
            );
        }

        Limit storage limit = _updateBookPostOrder(self, order);
        _updateLimitPostOrder(self, limit, order);

        self.orders[order.id] = order;
    }

    function removeOrderFromBook(Book storage self, Order memory order) internal {
        if (order.reduceOnly) {
            StorageLib.loadMarket(self.config.asset).unlinkReduceOnlyOrder(
                order.owner, order.subaccount, order.id.unwrap(), self.config.bookType
            );
        }

        _updateLimitRemoveOrder(self, order);
        _updateBookRemoveOrder(self, order);
    }

    function _updateBookPostOrder(Book storage self, Order memory order) private returns (Limit storage limit) {
        if (order.side == Side.BUY) {
            limit = self.bidLimits[order.price];
            if (limit.numOrders == 0) self.bidTree.insert(order.price);
            self.metadata.numBids++;
            self.metadata.quoteOI += order.amount.fullMulDiv(order.price, 1e18);
        } else {
            limit = self.askLimits[order.price];
            if (limit.numOrders == 0) self.askTree.insert(order.price);
            self.metadata.numAsks++;
            self.metadata.baseOI += order.amount;
        }
    }

    function _updateLimitPostOrder(Book storage self, Limit storage limit, Order memory order) private {
        limit.numOrders++;

        if (limit.headOrder.unwrap() == 0) {
            limit.headOrder = order.id;
            limit.tailOrder = order.id;
        } else {
            Order storage tailOrder = self.orders[limit.tailOrder];
            tailOrder.nextOrderId = order.id;
            order.prevOrderId = tailOrder.id;
            limit.tailOrder = order.id;
        }
    }

    function _updateBookRemoveOrder(Book storage self, Order memory order) private {
        if (order.side == Side.BUY) {
            self.metadata.numBids--;

            self.metadata.quoteOI -= order.amount.fullMulDiv(order.price, 1e18);
        } else {
            self.metadata.numAsks--;

            self.metadata.baseOI -= order.amount;
        }

        delete self.orders[order.id];
    }

    function _updateLimitRemoveOrder(Book storage self, Order memory order) private {
        Limit storage limit = order.side == Side.BUY ? self.bidLimits[order.price] : self.askLimits[order.price];

        if (limit.numOrders == 1) {
            if (order.side == Side.BUY) {
                delete self.bidLimits[order.price];
                self.bidTree.remove(order.price);
            } else {
                delete self.askLimits[order.price];
                self.askTree.remove(order.price);
            }
            return;
        }

        limit.numOrders--;

        if (order.prevOrderId.unwrap() != 0) self.orders[order.prevOrderId].nextOrderId = order.nextOrderId;
        else limit.headOrder = order.nextOrderId;

        if (order.nextOrderId.unwrap() != 0) self.orders[order.nextOrderId].prevOrderId = order.prevOrderId;
        else limit.tailOrder = order.prevOrderId;
    }

    function _getQuoteLimit(Book storage self, Limit storage limit, uint256 price, uint256 baseAmount)
        private
        view
        returns (uint256 quoteAmount, uint256 baseUsed)
    {
        uint256 numOrders = limit.numOrders;
        OrderId orderId = limit.headOrder;

        uint256 fillAmount;
        for (uint256 i; i < numOrders; ++i) {
            if (baseAmount == 0) break;
            if (orderId.unwrap() == 0) break;

            fillAmount = self.orders[orderId].amount.min(baseAmount);

            quoteAmount += fillAmount.fullMulDiv(price, 1e18);
            baseAmount -= fillAmount;
            baseUsed += fillAmount;

            orderId = self.orders[orderId].nextOrderId;
        }
    }

    function _getBaseLimit(Book storage self, Limit storage limit, uint256 price, uint256 quoteAmount)
        private
        view
        returns (uint256 baseAmount, uint256 quoteUsed)
    {
        uint256 numOrders = limit.numOrders;
        OrderId orderId = limit.headOrder;

        uint256 fillAmount;
        for (uint256 i; i < numOrders; ++i) {
            if (quoteAmount == 0) break;
            if (orderId.unwrap() == 0) break;

            fillAmount = self.orders[orderId].amount.min(quoteAmount.fullMulDiv(1e18, price));

            baseAmount += fillAmount;
            quoteUsed += fillAmount.fullMulDiv(price, 1e18);
            quoteAmount -= fillAmount.fullMulDiv(price, 1e18);

            orderId = self.orders[orderId].nextOrderId;
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";
import {DynamicArrayLib} from "@solady/utils/DynamicArrayLib.sol";

import {IGTL} from "../interfaces/IGTL.sol";

import {Constants} from "../types/Constants.sol";
import {BookType, TiF} from "../types/Enums.sol";
import {
    PlaceOrderArgs, PlaceOrderResult, __TradeData__, AmendLimitOrderArgs, MakerFillResult
} from "../types/Structs.sol";

import {StorageLib} from "../types/StorageLib.sol";
import {Order, OrderLib, OrderId, OrderIdLib, Side} from "../types/Order.sol";
import {Book, BookLib, Limit, BookConfig, BookSettings} from "../types/Book.sol";
import {ClearingHouse, ClearingHouseLib} from "../types/ClearingHouse.sol";

library CLOBLib {
    using OrderLib for *;
    using OrderLib for uint256;
    using OrderIdLib for *;
    using FixedPointMathLib for *;
    using SafeCastLib for uint256;
    using DynamicArrayLib for uint256[];

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                 EVENTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    event CancelFailed(
        bytes32 indexed asset, uint256 indexed orderId, address indexed owner, BookType bookType, uint256 nonce
    );

    event OrderProcessed(
        bytes32 indexed asset,
        address indexed account,
        uint256 subaccount,
        uint256 indexed orderId,
        uint256 amountSubmitted,
        bool baseDenominated,
        TiF tif,
        uint32 expiryTime,
        uint256 limitPrice,
        Side side,
        bool reduceOnly,
        uint256 basePosted,
        uint256 quoteTraded,
        uint256 baseTraded,
        BookType bookType,
        uint256 nonce
    );

    event OrderCanceled(
        bytes32 indexed asset,
        uint256 indexed orderId,
        address indexed owner,
        uint256 subaccount,
        uint256 collateralRefunded,
        BookType bookType,
        uint256 nonce
    );

    event OrderAmended(
        bytes32 indexed asset,
        uint256 indexed orderId,
        Order newOrder,
        int256 collateralDelta,
        BookType bookType,
        uint256 nonce
    );

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                 ERRORS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0x32cc7236
    error NotFactory();
    /// @dev sig: 0x87e393a7
    error FOKNotFilled();
    /// @dev sig: 0xdf7d9ed2
    error UnauthorizedReduce();
    // @dev sig: 0x60ab4840
    error UnauthorizedAmend();
    /// @dev sig: 0x45bb6073
    error UnauthorizedCancel();
    /// @dev sig: 0x3154078e
    error OrderAlreadyExpired();
    /// @dev sig: 0xadaa5d56
    error ReduceAmountOutOfBounds();
    /// @dev sig: 0x3d104567
    error InvalidAccountOrOperator();
    /// @dev sig: 0x52409ba3
    error PostOnlyOrderWouldBeFilled();
    /// @dev sig: 0x315ff5e5
    error MaxOrdersInBookPostNotCompetitive();
    /// @dev sig: 0x4b22649a
    error InvalidAmend();
    error IncorrectSubaccount();
    error ZeroOrder();
    error ZeroAmount();
    error InvalidMakerPrice();
    error InvalidOrderArgs();

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                     CONSTRUCTOR AND INITIALIZATION
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function init(bytes32 asset, BookSettings memory bookSettings, uint256 lotSize) internal {
        Book storage ds = _getStorage(asset, BookType.STANDARD);
        BookSettings storage dsSettings = StorageLib.loadBookSettings(asset);

        dsSettings.maxNumOrders = bookSettings.maxNumOrders;
        dsSettings.maxLimitsPerTx = bookSettings.maxLimitsPerTx;
        dsSettings.minLimitOrderAmountInBase = bookSettings.minLimitOrderAmountInBase;
        dsSettings.tickSize = bookSettings.tickSize;

        ds.config.asset = asset;
        ds.config.bookType = BookType.STANDARD;
        ds.config.lotSize = lotSize;

        ds = _getStorage(asset, BookType.BACKSTOP);

        ds.config.asset = asset;
        ds.config.bookType = BookType.BACKSTOP;
        ds.config.lotSize = lotSize;
    }

    address public constant GTL = Constants.GTL;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                           EXTERNAL WRITES
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function placeOrder(address account, PlaceOrderArgs memory args, BookType bookType)
        internal
        returns (PlaceOrderResult memory result)
    {
        Book storage ds = _getStorage(args.asset, bookType);

        // time in force below 1 is maker
        if (uint8(args.tif) <= 1 && args.limitPrice == 0) revert InvalidMakerPrice();
        if (args.amount == 0) revert ZeroAmount();
        ds.assertPriceInBounds(args.limitPrice);
        if (args.expiryTime.isExpired()) revert OrderAlreadyExpired();

        uint256 orderId = ds.toOrderId(account, args.clientOrderId);
        Order memory newOrder = args.toOrder(orderId, account);

        if (args.side == Side.BUY) result = _processBuyOrder(ds, newOrder, args);
        else result = _processSellOrder(ds, newOrder, args);

        if (result.baseTraded + result.quoteTraded + result.basePosted == 0) revert ZeroOrder();

        if (!args.reduceOnly && result.basePosted > 0) {
            _updateOrderbookNotional(
                args.asset, account, args.subaccount, result.basePosted.fullMulDiv(newOrder.price, 1e18).toInt256()
            );
        }

        _emitOrderProcessed(account, args, result, bookType);
    }

    /// @notice Amends an existing order for `account`
    function amend(address account, AmendLimitOrderArgs calldata args, BookType bookType)
        internal
        returns (int256 collateralDelta)
    {
        Book storage ds = _getStorage(args.asset, bookType);
        Order storage order = ds.orders[args.orderId.wrap()];

        if (order.id.unwrap() == 0) revert OrderLib.OrderNotFound();
        if (order.owner != account) revert UnauthorizedAmend();
        if (order.subaccount != args.subaccount) revert IncorrectSubaccount();

        ds.assertLimitPriceInBounds(args.price);
        ds.assertLimitOrderAmountInBounds(args.baseAmount);

        int256 notionalDelta;
        (notionalDelta, collateralDelta) = _processAmend(ds, order, args);

        _updateOrderbookNotional(args.asset, account, args.subaccount, notionalDelta);

        emit OrderAmended(args.asset, args.orderId, order, collateralDelta, ds.config.bookType, StorageLib.incNonce());
    }

    function cancel(bytes32 asset, address account, uint256 subaccount, uint256[] memory orderIds, BookType bookType)
        internal
        returns (uint256 collateralRefunded)
    {
        Book storage ds = _getStorage(asset, bookType);

        collateralRefunded = _executeCancel(ds, account, subaccount, orderIds);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                             INTERNAL LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _processBuyOrder(Book storage ds, Order memory newOrder, PlaceOrderArgs memory args)
        internal
        returns (PlaceOrderResult memory result)
    {
        result.orderId = newOrder.id.unwrap();

        (result.quoteTraded, result.baseTraded) = _executeBuyOrder(ds, newOrder, args.tif, args.baseDenominated);

        // if maker order
        if (uint8(args.tif) <= 1) result.basePosted = newOrder.amount;
    }

    function _processSellOrder(Book storage ds, Order memory newOrder, PlaceOrderArgs memory args)
        internal
        returns (PlaceOrderResult memory result)
    {
        result.orderId = newOrder.id.unwrap();

        (result.quoteTraded, result.baseTraded) = _executeSellOrder(ds, newOrder, args.tif, args.baseDenominated);

        // if maker order
        if (uint8(args.tif) <= 1) result.basePosted = newOrder.amount;
    }

    function _executeBuyOrder(Book storage ds, Order memory newOrder, TiF tif, bool baseDenominated)
        internal
        returns (uint256 quoteSent, uint256 baseReceived)
    {
        // if price crosses the book
        if (ds.getBestAsk() <= newOrder.price) {
            if (tif == TiF.MOC) revert PostOnlyOrderWouldBeFilled();
            (quoteSent, baseReceived) = _matchIncomingBid(ds, newOrder, baseDenominated);
        }

        if (tif == TiF.FOK && newOrder.amount > 0) revert FOKNotFilled();

        // if taker order
        if (uint8(tif) > 1) return (quoteSent, baseReceived);

        // Max limits per tx is enforced on the caller to allow for whitelisted operators
        // to implement their own max limit logic.
        ds.incrementLimitsPlaced(msg.sender);

        // convert quote denominated to base
        if (!baseDenominated) newOrder.amount = newOrder.amount.fullMulDiv(1e18, newOrder.price);

        /// bound to lots and above min size
        newOrder.amount = ds.getPostableBaseAmount(newOrder.amount);

        if (newOrder.amount == 0) return (quoteSent, baseReceived);

        if (ds.metadata.numBids < StorageLib.loadBookSettings(ds.config.asset).maxNumOrders) {
            // Book has room for new order
            ds.addOrderToBook(newOrder);
        } else if (
            ds.metadata.numBids == StorageLib.loadBookSettings(ds.config.asset).maxNumOrders
                && newOrder.price > ds.getMinBidPrice()
        ) {
            // The max orders are filled, but this order is more competitive
            Order storage removeOrder = ds.orders[ds.bidLimits[ds.getMinBidPrice()].tailOrder];

            _removeUnfillableOrder(ds, removeOrder);

            ds.addOrderToBook(newOrder);
        } else {
            delete newOrder.amount;
        }
    }

    function _executeSellOrder(Book storage ds, Order memory newOrder, TiF tif, bool baseDenominated)
        internal
        returns (uint256 quoteReceived, uint256 baseSent)
    {
        // if price crosses the book
        if (ds.getBestBid() >= newOrder.price) {
            if (tif == TiF.MOC) revert PostOnlyOrderWouldBeFilled();
            (quoteReceived, baseSent) = _matchIncomingAsk(ds, newOrder, baseDenominated);
        }

        if (tif == TiF.FOK && newOrder.amount > 0) revert FOKNotFilled();

        // if taker order
        if (uint8(tif) > 1) return (quoteReceived, baseSent);

        // Max limits per tx is enforced on the caller to allow for whitelisted operators
        // to implement their own max limit logic.
        ds.incrementLimitsPlaced(msg.sender);

        // convert quote denominated to base
        if (!baseDenominated) newOrder.amount = newOrder.amount.fullMulDiv(1e18, newOrder.price);

        // bound to lots and above min size
        newOrder.amount = ds.getPostableBaseAmount(newOrder.amount);

        if (newOrder.amount == 0) return (quoteReceived, baseSent);

        if (ds.metadata.numAsks < StorageLib.loadBookSettings(ds.config.asset).maxNumOrders) {
            ds.addOrderToBook(newOrder);
        } else if (
            ds.metadata.numAsks == StorageLib.loadBookSettings(ds.config.asset).maxNumOrders
                && newOrder.price < ds.getMaxAskPrice()
        ) {
            Order storage removeOrder = ds.orders[ds.askLimits[ds.getMaxAskPrice()].tailOrder];

            _removeUnfillableOrder(ds, removeOrder);

            ds.addOrderToBook(newOrder);
        } else {
            delete newOrder.amount;
        }
    }

    function _removeUnfillableOrder(Book storage ds, Order storage order) internal {
        uint256 quoteTokenAmount = order.amount.fullMulDiv(order.price, 1e18);
        uint256 orderId = order.id.unwrap();
        bytes32 asset = ds.config.asset;
        address owner = order.owner;
        uint256 subaccount = order.subaccount;

        uint256 collateralRefund;
        if (!order.reduceOnly) {
            collateralRefund = quoteTokenAmount.fullMulDiv(1e18, _getLeverage(asset, owner, subaccount));

            _updateOrderbookNotional(asset, owner, subaccount, -quoteTokenAmount.toInt256());
        }

        emit OrderCanceled(
            asset, orderId, owner, subaccount, collateralRefund, ds.config.bookType, StorageLib.incNonce()
        );

        StorageLib.loadCollateralManager().creditAccount(owner, collateralRefund);

        ds.removeOrderFromBook(order);
    }

    /// @notice Match incoming bid order to best asks
    function _matchIncomingBid(Book storage ds, Order memory incomingOrder, bool baseDenominated)
        internal
        returns (uint256 quoteSent, uint256 baseReceived)
    {
        uint256 bestAsk = ds.getBestAsk();
        uint256 maxAsk = StorageLib.loadMarket(ds.config.asset).getMaxDivergingAskPrice();

        while (bestAsk <= incomingOrder.price && incomingOrder.amount > 0) {
            if (bestAsk == type(uint256).max) break;
            if (bestAsk > maxAsk) break;

            Limit storage limit = ds.askLimits[bestAsk];
            Order storage bestAskOrder = ds.orders[limit.headOrder];

            if (bestAskOrder.isExpired()) {
                _removeUnfillableOrder(ds, bestAskOrder);
                bestAsk = ds.getBestAsk();
                continue;
            }

            __TradeData__ memory data = _matchIncomingOrder(ds, bestAskOrder, incomingOrder, baseDenominated);

            incomingOrder.amount -= data.filledAmount;

            baseReceived += data.baseTraded;
            quoteSent += data.quoteTraded;

            if (limit.numOrders == 0) bestAsk = ds.getBestAsk();
        }
    }

    function _matchIncomingAsk(Book storage ds, Order memory incomingOrder, bool baseDenominated)
        internal
        returns (uint256 totalQuoteTokenReceived, uint256 totalBaseTokenSent)
    {
        uint256 bestBid = ds.getBestBid();
        uint256 minBid = StorageLib.loadMarket(ds.config.asset).getMaxDivergingBidPrice();

        while (bestBid >= incomingOrder.price && incomingOrder.amount > 0) {
            if (bestBid == 0) break;
            if (bestBid < minBid) break;

            Limit storage limit = ds.bidLimits[bestBid];
            Order storage bestBidOrder = ds.orders[limit.headOrder];

            if (bestBidOrder.isExpired()) {
                _removeUnfillableOrder(ds, bestBidOrder);
                bestBid = ds.getBestBid();
                continue;
            }

            __TradeData__ memory data = _matchIncomingOrder(ds, bestBidOrder, incomingOrder, baseDenominated);

            incomingOrder.amount -= data.filledAmount;

            totalQuoteTokenReceived += data.quoteTraded;
            totalBaseTokenSent += data.baseTraded;

            if (limit.numOrders == 0) bestBid = ds.getBestBid();
        }
    }

    function _matchIncomingOrder(
        Book storage ds,
        Order storage matchedOrder,
        Order memory incomingOrder,
        bool baseDenominated // true if incomingOrder is in base, false if in quote
    ) internal returns (__TradeData__ memory tradeData) {
        address matchedOwner = matchedOrder.owner;

        if (incomingOrder.owner == matchedOwner) {
            _removeUnfillableOrder(ds, matchedOrder);
            return tradeData;
        }

        if (matchedOrder.reduceOnly) _boundReduceOnlyOrder(ds, matchedOrder);

        tradeData = ds.getTradedAmounts({
            makerBase: matchedOrder.amount,
            takerAmount: incomingOrder.amount,
            price: matchedOrder.price,
            baseDenominated: baseDenominated
        });

        if (tradeData.baseTraded == 0) return tradeData;

        bool orderRemoved = tradeData.baseTraded == matchedOrder.amount;

        // handle maker fill
        bool unfillable = StorageLib.loadClearingHouse().processMakerFill(
            MakerFillResult({
                asset: ds.config.asset,
                bookType: ds.config.bookType,
                orderId: matchedOrder.id.unwrap(),
                maker: matchedOwner,
                subaccount: matchedOrder.subaccount,
                side: matchedOrder.side,
                quoteAmountTraded: tradeData.quoteTraded,
                baseAmountTraded: tradeData.baseTraded,
                reduceOnly: matchedOrder.reduceOnly
            })
        );

        if (unfillable) {
            _removeUnfillableOrder(ds, matchedOrder);
            return __TradeData__(0, 0, 0);
        } else if (!orderRemoved) {
            if (incomingOrder.side == Side.BUY) ds.metadata.baseOI -= tradeData.baseTraded;
            else ds.metadata.quoteOI -= tradeData.quoteTraded;
        }

        if (!matchedOrder.reduceOnly) {
            _updateOrderbookNotional(
                ds.config.asset, matchedOwner, matchedOrder.subaccount, -tradeData.quoteTraded.toInt256()
            );
        }

        if (orderRemoved) ds.removeOrderFromBook(matchedOrder);
        else matchedOrder.amount -= tradeData.baseTraded;
    }

    function _executeCancel(Book storage ds, address account, uint256 subaccount, uint256[] memory orderIds)
        internal
        returns (uint256 totalCollateralRefunded)
    {
        bytes32 asset = ds.config.asset;
        BookType bookType = ds.config.bookType;
        uint256 numOrders = orderIds.length;

        uint256 orderId;
        for (uint256 i; i < numOrders; ++i) {
            orderId = orderIds[i];
            Order storage order = ds.orders[orderId.wrap()];

            // This loads the whole order
            if (order.isNull()) {
                emit CancelFailed(asset, orderId, account, bookType, StorageLib.incNonce());
                continue; // Order may have been matched
            } else if (order.owner != account) {
                revert UnauthorizedCancel();
            } else if (order.subaccount != subaccount) {
                revert IncorrectSubaccount();
            }

            uint256 collateralRefunded;
            if (!order.reduceOnly) {
                uint256 quoteAmount = order.amount.fullMulDiv(order.price, 1e18);
                collateralRefunded = quoteAmount.fullMulDiv(1e18, _getLeverage(asset, account, subaccount));
                _updateOrderbookNotional(asset, account, subaccount, -quoteAmount.toInt256());
            }

            emit OrderCanceled(asset, orderId, account, subaccount, collateralRefunded, bookType, StorageLib.incNonce());

            totalCollateralRefunded += collateralRefunded;

            ds.removeOrderFromBook(order);
        }
    }

    function _boundReduceOnlyOrder(Book storage ds, Order storage order) internal {
        uint256 positionAmount = StorageLib.loadMarket(ds.config.asset).position[order.owner][order.subaccount].amount;

        if (positionAmount < order.amount) {
            uint256 reduceAmount = order.amount - positionAmount;

            order.amount = positionAmount;

            if (order.side == Side.BUY) ds.metadata.quoteOI -= reduceAmount.fullMulDiv(order.price, 1e18);
            else ds.metadata.baseOI -= reduceAmount;
        }
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                          INTERNAL AMEND LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Performs the amending of an order
    function _processAmend(Book storage ds, Order storage order, AmendLimitOrderArgs calldata args)
        internal
        returns (int256 notionalDelta, int256 collateralDelta)
    {
        if (
            args.expiryTime.isExpired()
                || args.baseAmount < StorageLib.loadBookSettings(ds.config.asset).minLimitOrderAmountInBase
        ) {
            revert InvalidAmend();
        } else if (order.side != args.side || order.price != args.price) {
            // change place in book
            return _executeAmendNewOrder(ds, order, args);
        } else {
            // change amount
            return _executeAmendAmount(ds, order, args);
        }
    }

    /// @dev Performs the removal and replacement of an amended order with a new price or side
    function _executeAmendNewOrder(Book storage ds, Order storage order, AmendLimitOrderArgs calldata args)
        internal
        returns (int256 notionalDelta, int256 collateralDelta)
    {
        Order memory newOrder = args.toOrder(order);

        uint256 leverage = _getLeverage(args.asset, newOrder.owner, newOrder.subaccount);

        if (!order.reduceOnly) {
            uint256 orderNotional = order.amount.fullMulDiv(order.price, 1e18);

            notionalDelta -= orderNotional.toInt256();
            collateralDelta -= orderNotional.fullMulDiv(1e18, leverage).toInt256();
        }

        ds.removeOrderFromBook(order);

        // amends restricted to post-only
        if (args.side == Side.BUY) _executeBuyOrder(ds, newOrder, TiF.MOC, true);
        else _executeSellOrder(ds, newOrder, TiF.MOC, true);

        if (!newOrder.reduceOnly) {
            uint256 orderNotional = newOrder.amount.fullMulDiv(newOrder.price, 1e18);

            notionalDelta += orderNotional.toInt256();
            collateralDelta += orderNotional.fullMulDiv(1e18, leverage).toInt256();
        }
    }

    /// @dev Performs the updating of an amended order with a new amount
    function _executeAmendAmount(Book storage ds, Order storage order, AmendLimitOrderArgs calldata args)
        internal
        returns (int256 notionalDelta, int256 collateralDelta)
    {
        uint256 price = order.price;
        uint256 newQuoteAmount = args.baseAmount.fullMulDiv(price, 1e18);
        uint256 oldQuoteAmount = order.amount.fullMulDiv(price, 1e18);
        uint256 leverage = _getLeverage(args.asset, order.owner, args.subaccount);

        if (!order.reduceOnly) {
            uint256 orderNotional = order.amount.fullMulDiv(order.price, 1e18);
            notionalDelta -= orderNotional.toInt256();
            collateralDelta -= orderNotional.fullMulDiv(1e18, leverage).toInt256();
        }

        if (!args.reduceOnly) {
            uint256 newOrderNotional = args.baseAmount.fullMulDiv(price, 1e18);
            notionalDelta += newOrderNotional.toInt256();
            collateralDelta += newOrderNotional.fullMulDiv(1e18, leverage).toInt256();
        }

        if (order.side == Side.BUY) {
            int256 quoteDelta = oldQuoteAmount.toInt256() - newQuoteAmount.toInt256();

            ds.metadata.quoteOI = (ds.metadata.quoteOI.toInt256() - quoteDelta).abs();
        } else {
            int256 baseDelta = order.amount.toInt256() - args.baseAmount.toInt256();

            ds.metadata.baseOI = (ds.metadata.baseOI.toInt256() - baseDelta).abs();
        }

        if (order.reduceOnly != args.reduceOnly) {
            if (args.reduceOnly) {
                StorageLib.loadMarket(ds.config.asset).linkReduceOnlyOrder(
                    order.owner, order.subaccount, args.orderId, ds.config.bookType
                );
            } else {
                StorageLib.loadMarket(ds.config.asset).unlinkReduceOnlyOrder(
                    order.owner, order.subaccount, args.orderId, ds.config.bookType
                );
            }
        }

        order.amount = args.baseAmount;
        order.reduceOnly = args.reduceOnly;
        order.expiryTime = args.expiryTime;
    }

    function _emitOrderProcessed(
        address account,
        PlaceOrderArgs memory args,
        PlaceOrderResult memory result,
        BookType bookType
    ) internal {
        emit OrderProcessed({
            asset: args.asset,
            account: account,
            subaccount: args.subaccount,
            orderId: result.orderId,
            amountSubmitted: args.amount,
            baseDenominated: args.baseDenominated,
            tif: args.tif,
            expiryTime: args.expiryTime,
            limitPrice: args.limitPrice,
            side: args.side,
            reduceOnly: args.reduceOnly,
            basePosted: result.basePosted,
            quoteTraded: result.quoteTraded,
            baseTraded: result.baseTraded,
            bookType: bookType,
            nonce: StorageLib.incNonce()
        });
    }

    function _updateOrderbookNotional(bytes32 asset, address account, uint256 subaccount, int256 amount) private {
        StorageLib.loadMarket(asset).updateOrderbookNotional(account, subaccount, amount);
    }

    function _getLeverage(bytes32 asset, address account, uint256 subaccount) internal view returns (uint256) {
        return StorageLib.loadMarket(asset).getPositionLeverage(account, subaccount);
    }

    function _div(int256 a, int256 b) private pure returns (int256) {
        uint256 result = a.abs().fullMulDiv(1e18, b.abs());
        return a < 0 != b < 0 ? -result.toInt256() : result.toInt256();
    }

    function _getStorage(bytes32 asset, BookType bookType) internal pure returns (Book storage) {
        return StorageLib.loadBook(asset, bookType);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {PlaceOrderArgs, AmendLimitOrderArgs} from "./Structs.sol";
import {Side} from "./Enums.sol";

type OrderId is uint256;

using OrderIdLib for OrderId global;

library OrderIdLib {
    error UintExceedsOrderIdSize();

    function getOrderId(address account, uint96 id) internal pure returns (uint256) {
        return uint256(bytes32(abi.encodePacked(account, id)));
    }

    // @todo rename to toOrderId
    function wrap(uint256 id) internal pure returns (OrderId) {
        return OrderId.wrap(id);
    }

    function unwrap(OrderId id) internal pure returns (uint256) {
        return OrderId.unwrap(id);
    }
}

uint256 constant NULL_ORDER_ID = 0;
uint32 constant NULL_TIMESTAMP = 0;

struct Order {
    // SLOT 0 //
    Side side;
    uint32 expiryTime;
    OrderId id;
    OrderId prevOrderId;
    OrderId nextOrderId;
    // SLOT 1 //
    address owner;
    // SLOT 2 //
    uint256 price;
    // SLOT 3 //
    uint256 amount;
    // SLOT 4 //
    uint256 subaccount;
    // SLOT 5 //
    bool reduceOnly;
}

using OrderLib for Order global;

library OrderLib {
    using OrderIdLib for uint256;

    error OrderNotFound();

    function toOrder(PlaceOrderArgs memory args, uint256 orderId, address owner)
        internal
        pure
        returns (Order memory order)
    {
        order.side = args.side;
        order.expiryTime = args.expiryTime;
        order.id = orderId.wrap();
        order.owner = owner;
        order.amount = args.amount;
        order.price = args.limitPrice;
        order.subaccount = args.subaccount;
        order.reduceOnly = args.reduceOnly;

        // zero price == max slippage
        if (order.price == 0 && order.side == Side.BUY) order.price = type(uint256).max; // set to max for buy orders
    }

    function toOrder(AmendLimitOrderArgs calldata args, Order storage currentOrder)
        internal
        view
        returns (Order memory newOrder)
    {
        newOrder.owner = currentOrder.owner;
        newOrder.id = currentOrder.id;
        newOrder.side = args.side;
        newOrder.price = args.price;
        newOrder.amount = args.baseAmount;
        newOrder.reduceOnly = args.reduceOnly;
        newOrder.subaccount = currentOrder.subaccount;
        newOrder.expiryTime = args.expiryTime;
    }

    function isExpired(Order memory self) internal view returns (bool) {
        // slither-disable-next-line timestamp
        return self.expiryTime != NULL_TIMESTAMP && self.expiryTime < block.timestamp;
    }

    function isExpired(uint256 expiryTime) internal view returns (bool) {
        // slither-disable-next-line timestamp
        return expiryTime != NULL_TIMESTAMP && expiryTime < block.timestamp;
    }

    // @todo this reads the whole order into memory
    function isNull(Order memory self) internal pure returns (bool) {
        return self.id.unwrap() == NULL_ORDER_ID;
    }

    // @todo this reads the whole order into memory
    function assertExists(Order memory self) internal pure {
        if (self.isNull()) revert OrderNotFound();
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

interface IGTL {
    function orderUpdated(int256 marginDelta) external;
    function addSubaccount(uint256 subaccount) external;
    function removeSubaccount(uint256 subaccount) external;
}

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

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {EnumerableSetLib} from "@solady/utils/EnumerableSetLib.sol";
import {DynamicArrayLib} from "@solady/utils/DynamicArrayLib.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";

import {IGTL} from "../interfaces/IGTL.sol";

import {Constants} from "./Constants.sol";
import {Side, Status, BookType, TradeType} from "./Enums.sol";
import {
    PlaceOrderArgs,
    PlaceOrderResult,
    MakerFillResult,
    PositionUpdateResult,
    FundingPaymentResult,
    OIDelta,
    TradeExecutedData,
    MakerSettleData,
    TakerSettleData
} from "./Structs.sol";

import {BackstopLiquidatorDataLib} from "./BackstopLiquidatorDataLib.sol";

import {StorageLib} from "./StorageLib.sol";

import {Market, MarketLib} from "./Market.sol";
import {Book} from "./Book.sol";
import {InsuranceFund} from "./InsuranceFund.sol";
import {CollateralManager} from "./CollateralManager.sol";
import {FeeManager} from "./FeeManager.sol";
import {Position} from "./Position.sol";

struct ClearingHouse {
    bool active;
    mapping(bytes32 asset => Market) market;
    mapping(address account => mapping(uint256 subaccount => EnumerableSetLib.Bytes32Set)) assets;
    mapping(address account => mapping(address operator => bool)) approvedOperator;
    mapping(address liquidator => uint256) liquidatorPoints;
    mapping(address account => mapping(uint256 nonce => bool)) nonceUsed;
}

using ClearingHouseLib for ClearingHouse global;

// @todo review: for maker: tests on refund for reversing & refund on less margin needed to open

library ClearingHouseLib {
    using FixedPointMathLib for *;
    using SafeCastLib for *;
    using EnumerableSetLib for EnumerableSetLib.Bytes32Set;
    using DynamicArrayLib for *;

    error CrossMarginIsDisabled();
    error Liquidatable();
    error NotLiquidatable();
    error MarginRequirementUnmet();

    struct __ProcessMakerFillCache__ {
        DynamicArrayLib.DynamicArray assets;
        Position[] positions;
        int256 fundingPayment;
        uint256 orderValue;
        PositionUpdateResult positionResult;
        int256 margin;
        bool isNewPosition;
        uint256 fee;
    }

    struct __ProcessTakerFillCache__ {
        DynamicArrayLib.DynamicArray assets;
        Position[] positions;
        PositionUpdateResult positionResult;
        int256 fundingPayment;
        int256 margin;
        uint256 takerFee;
    }

    struct __RebalanceCollateralCache__ {
        uint256 intendedMargin;
        int256 upnl;
        int256 equity;
        int256 overCollateralization;
    }

    struct __FillParams__ {
        bytes32 asset;
        address account;
        uint256 subaccount;
        Side side;
        uint256 quoteAmount;
        uint256 baseAmount;
        uint256 collateralPosted; // Only used for limit orders
    }

    struct __LiquidatableCheckCache__ {
        int256 upnl;
        uint256 minMargin;
        int256 totalUpnl;
        uint256 totalMinMargin;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                              ORDER PLACE
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function placeOrder(ClearingHouse storage self, address account, PlaceOrderArgs calldata args, BookType bookType)
        internal
        returns (PlaceOrderResult memory orderResult)
    {
        Market storage market = self.market[args.asset];

        orderResult = market.placeOrder(account, args, bookType);

        uint256 collateralPosted;
        if (orderResult.basePosted > 0 && !args.reduceOnly) {
            collateralPosted = _getCollateral(
                orderResult.basePosted, args.limitPrice, market.getPositionLeverage(account, args.subaccount)
            );
        }

        if (orderResult.baseTraded == 0) {
            StorageLib.loadCollateralManager().handleCollateralDelta({
                account: account,
                collateralDelta: collateralPosted.toInt256()
            });

            return orderResult;
        }

        _processTakerFill(
            self,
            __FillParams__({
                asset: args.asset,
                account: account,
                subaccount: args.subaccount,
                side: args.side,
                quoteAmount: orderResult.quoteTraded,
                baseAmount: orderResult.baseTraded,
                collateralPosted: collateralPosted
            })
        );
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                              MAKER FILL
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice processes a maker fill during CLOBLib._matchIncomingOrder()
    /// @dev if unfillable, must return true while emitting no events and saving nothing to storage
    /// @dev must not revert
    function processMakerFill(ClearingHouse storage self, MakerFillResult memory makerResult)
        internal
        returns (bool unfillable)
    {
        __ProcessMakerFillCache__ memory cache;

        // load assets
        cache.assets = self.getAssets(makerResult.maker, makerResult.subaccount);

        // check if new position
        cache.isNewPosition = !cache.assets.contains(makerResult.asset);

        // if new position, check if asset can be added to account
        // if not, return true to indicate unfillable
        if (cache.isNewPosition) {
            if (!_assetCanBeAddedToAccount(cache.assets, makerResult.asset)) return true;
            // add asset to account
            cache.assets.p(makerResult.asset);
        }

        // load positions
        cache.positions =
            _getPositions(self, cache.assets, makerResult.maker, makerResult.subaccount, cache.isNewPosition);

        // get funding payment & update position.lastCumulativeFunding
        cache.fundingPayment = realizeFundingPayment(cache.assets, cache.positions);

        // get index of traded position
        uint256 positionIdx = cache.assets.indexOf(makerResult.asset);

        // process the trade
        cache.positionResult = cache.positions[positionIdx].processTrade({
            side: makerResult.side,
            quoteTraded: makerResult.quoteAmountTraded,
            baseTraded: makerResult.baseAmountTraded
        });

        cache.fee = makerResult.bookType == BookType.STANDARD
            ? StorageLib.loadFeeManager().getMakerFee(makerResult.maker, makerResult.quoteAmountTraded)
            : 0;

        cache.margin = StorageLib.loadCollateralManager().getMarginBalance(makerResult.maker, makerResult.subaccount);

        // settle rpnl on margin
        cache.margin += cache.positionResult.rpnl - cache.fundingPayment - cache.fee.toInt256();

        // rebalance account
        (cache.margin, cache.positionResult.marginDelta) = self.rebalanceAccount({
            assets: cache.assets,
            positions: cache.positions,
            margin: cache.margin,
            marginDelta: cache.positionResult.marginDelta
        });

        cache.orderValue = makerResult.reduceOnly
            ? 0
            : makerResult.quoteAmountTraded.fullMulDiv(1e18, cache.positions[positionIdx].leverage);

        // check liquidatability
        if (self.isLiquidatable(cache.assets, cache.positions, cache.margin, BookType.STANDARD)) return true;

        if (makerResult.bookType == BookType.BACKSTOP) {
            BackstopLiquidatorDataLib.addLiquidatorVolume(makerResult.maker, makerResult.quoteAmountTraded);
        }

        StorageLib.loadInsuranceFund().pay(cache.fee);

        // settle fill & subtract margin posted from amount owed
        StorageLib.loadCollateralManager().settleFill({
            account: makerResult.maker,
            subaccount: makerResult.subaccount,
            margin: cache.margin,
            marginDelta: cache.positionResult.marginDelta - cache.orderValue.toInt256()
        });

        // unlink reduce only order from account so storage isn't deleted before this function returns to CLOBLib
        if (cache.positionResult.sideClose && makerResult.reduceOnly) {
            self.market[makerResult.asset].unlinkReduceOnlyOrder(
                makerResult.maker, makerResult.subaccount, makerResult.orderId, makerResult.bookType
            );
        }

        self.updateAccount({
            account: makerResult.maker,
            subaccount: makerResult.subaccount,
            assets: cache.assets,
            positions: cache.positions,
            tradedAsset: makerResult.asset,
            positionIdx: positionIdx,
            oiDelta: cache.positionResult.oiDelta,
            sideClose: cache.positionResult.sideClose
        });
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               HELPERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function setAssets(
        ClearingHouse storage self,
        address account,
        uint256 subaccount,
        uint256 newLength,
        bytes32 asset
    ) internal {
        uint256 oldLength = self.assets[account][subaccount].length();

        if (oldLength == newLength) return;

        if (oldLength < newLength) self.assets[account][subaccount].add(asset);
        else self.assets[account][subaccount].remove(asset);

        if (account == Constants.GTL) {
            if (oldLength == 0) IGTL(Constants.GTL).addSubaccount(subaccount);
            else if (newLength == 0) IGTL(Constants.GTL).removeSubaccount(subaccount);
        }
    }

    function setPositions(
        ClearingHouse storage self,
        bytes32 tradedAsset,
        address account,
        uint256 subaccount,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions
    ) internal {
        uint256 length = assets.length();

        for (uint256 i; i < length; ++i) {
            if (assets.getBytes32(i) == tradedAsset) {
                self.market[assets.getBytes32(i)].setPosition(account, subaccount, positions[i]);
            } else {
                self.market[assets.getBytes32(i)].position[account][subaccount].lastCumulativeFunding =
                    positions[i].lastCumulativeFunding;
            }
        }
    }

    function rebalanceAccount(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions,
        int256 margin,
        int256 marginDelta
    ) internal view returns (int256 finalMargin, int256 finalMarginDelta) {
        if (marginDelta >= 0) {
            return self.rebalanceOpen({assets: assets, positions: positions, margin: margin, marginDelta: marginDelta});
        } else {
            return self.rebalanceClose({assets: assets, positions: positions, margin: margin, marginDelta: marginDelta});
        }
    }

    function rebalanceOpen(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions,
        int256 margin,
        int256 marginDelta
    ) internal view returns (int256 finalMargin, int256 finalMarginDelta) {
        (uint256 intendedMargin, int256 upnl) = _getIntendedMarginAndUpnl(self, assets, positions);

        int256 equity = margin + upnl;

        // finalMarginDelta = MIN(marginDelta, MAX(intendedMargin - equity, 0))
        // marginDelta on an open is (openedNotional / leverage)
        finalMarginDelta = marginDelta.min((intendedMargin.toInt256() - equity).max(0));

        finalMargin = margin + finalMarginDelta;
    }

    /// @notice on close accounts should receive MAX(closed open notional / leverage, amount left over after meeting intended margin)
    ///         meaning closed margin subsidizes -pnl
    function rebalanceClose(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions,
        int256 margin,
        int256 marginDelta
    ) internal view returns (int256 finalMargin, int256 finalMarginDelta) {
        (uint256 intendedMargin, int256 upnl) = _getIntendedMarginAndUpnl(self, assets, positions);

        // full close
        if (intendedMargin == 0) {
            if (margin < 0) return (margin, 0);
            else return (0, -margin);
        }

        int256 equity = margin + upnl;

        // finalMarginDelta = MAX(marginDelta, MIN(intendedMargin - equity, 0))
        // marginDelta on a decrease is -(closedOpenNotional / leverage), where
        // closedOpenNotional = position.openNotional * closedAmount / position.amount
        finalMarginDelta = marginDelta.max((intendedMargin.toInt256() - equity).min(0));

        finalMargin = margin + finalMarginDelta;
    }

    function updateAccount(
        ClearingHouse storage self,
        address account,
        uint256 subaccount,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions,
        bytes32 tradedAsset,
        uint256 positionIdx,
        OIDelta memory oiDelta,
        bool sideClose
    ) internal {
        self.setPositions(tradedAsset, account, subaccount, assets, positions);

        if (positions[positionIdx].amount == 0) _movePop(assets, tradedAsset);

        self.setAssets(account, subaccount, assets.length(), tradedAsset);

        MarketLib.updateOI(tradedAsset, oiDelta);

        if (sideClose) self.market[tradedAsset].cancelCloseOrders(account, subaccount);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getAssets(ClearingHouse storage self, address account, uint256 subaccount)
        internal
        view
        returns (DynamicArrayLib.DynamicArray memory assets)
    {
        return self.assets[account][subaccount].values().wrap();
    }

    function getAccount(ClearingHouse storage self, address account, uint256 subaccount)
        internal
        view
        returns (DynamicArrayLib.DynamicArray memory assets, Position[] memory positions)
    {
        assets = self.assets[account][subaccount].values().wrap();
        positions = _getPositions(self, assets, account, subaccount, false);
    }

    function getAccountAndMargin(ClearingHouse storage self, address account, uint256 subaccount)
        internal
        view
        returns (DynamicArrayLib.DynamicArray memory assets, Position[] memory positions, int256 margin)
    {
        (assets, positions) = self.getAccount(account, subaccount);
        margin = StorageLib.loadCollateralManager().getMarginBalance(account, subaccount);
    }

    function getUpnl(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions
    ) internal view returns (int256 upnl) {
        uint256 length = assets.length();

        for (uint256 i; i < length; ++i) {
            upnl += self.market[assets.getBytes32(i)].getUpnl(positions[i]);
        }
    }

    function getIntendedMargin(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions
    ) internal view returns (uint256 intendedMargin) {
        uint256 length = assets.length();

        for (uint256 i; i < length; ++i) {
            intendedMargin += self.market[assets.getBytes32(i)].getIntendedMargin(positions[i]);
        }
    }

    /// @notice returns margin prorated based on the asset's notional value
    function getProratedMargin(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions,
        bytes32 asset,
        int256 margin
    ) internal view returns (int256 proratedMargin) {
        uint256 length = assets.length();

        uint256 notional;
        uint256 assetNotional;
        uint256 totalNotional;
        for (uint256 i; i < length; ++i) {
            notional = self.market[assets.getBytes32(i)].getNotionalValue(positions[i]);
            totalNotional += notional;

            if (assets.getBytes32(i) == asset) assetNotional = notional;
        }

        return _prorateMargin(margin, assetNotional, totalNotional);
    }

    function realizeFundingPayment(DynamicArrayLib.DynamicArray memory assets, Position[] memory positions)
        internal
        view
        returns (int256 fundingPayment)
    {
        uint256 length = assets.length();

        for (uint256 i; i < length; ++i) {
            fundingPayment += MarketLib.realizeFundingPayment(assets.getBytes32(i), positions[i]);
        }
    }

    function getNotionalAccountValue(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions
    ) internal view returns (uint256 totalNotional) {
        uint256 length = assets.length();

        for (uint256 i; i < length; ++i) {
            totalNotional += self.market[assets.getBytes32(i)].getNotionalValue(positions[i]);
        }
    }

    function getFundingPayment(ClearingHouse storage self, address account, uint256 subaccount)
        internal
        view
        returns (int256 fundingPayment)
    {
        bytes32[] memory assets = self.assets[account][subaccount].values();

        for (uint256 i; i < assets.length; ++i) {
            fundingPayment += self.market[assets[i]].getFundingPayment(account, subaccount);
        }
    }

    function isLiquidatable(ClearingHouse storage self, address account, uint256 subaccount, BookType bookType)
        internal
        view
        returns (bool liquidatable)
    {
        (DynamicArrayLib.DynamicArray memory assets, Position[] memory positions, int256 margin) =
            self.getAccountAndMargin(account, subaccount);

        int256 fundingPayment = self.getFundingPayment(account, subaccount);

        return self.isLiquidatable(assets, positions, margin - fundingPayment, bookType);
    }

    function isLiquidatable(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions,
        int256 margin,
        BookType bookType
    ) internal view returns (bool liquidatable) {
        __LiquidatableCheckCache__ memory cache;
        for (uint256 i; i < assets.length(); ++i) {
            (cache.upnl, cache.minMargin) =
                self.market[assets.getBytes32(i)].getUpnlAndMinMargin(positions[i], bookType);

            cache.totalUpnl += cache.upnl;
            cache.totalMinMargin += cache.minMargin;
        }

        // account close w/ bad debt
        if (cache.totalMinMargin == 0 && margin < 0) return true;

        return (margin + cache.totalUpnl) < cache.totalMinMargin.toInt256();
    }

    function isOpenMarginRequirementMet(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions,
        int256 margin
    ) internal view returns (bool met) {
        uint256 minOpenMargin;
        int256 upnl;
        for (uint256 i; i < positions.length; ++i) {
            minOpenMargin += self.market[assets.getBytes32(i)].getMinOpenMargin(positions[i].amount);
            upnl += self.market[assets.getBytes32(i)].getUpnl(positions[i]);
        }

        return margin + upnl >= minOpenMargin.toInt256();
    }

    function hasBadDebt(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions,
        int256 margin
    ) internal view returns (bool badDebt) {
        int256 upnl;
        for (uint256 i; i < positions.length; ++i) {
            upnl += self.market[assets.getBytes32(i)].getUpnl(positions[i]);
        }

        return margin + upnl < 0;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            PRIVATE HELPERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _getCollateral(uint256 baseAmount, uint256 price, uint256 leverage)
        private
        pure
        returns (uint256 collateral)
    {
        collateral = baseAmount.fullMulDiv(price, 1e18).fullMulDiv(1e18, leverage);
    }

    function _isClosing(uint256 positionAmount, bool isLong, Side side) internal pure returns (bool closing) {
        if (positionAmount == 0) return false;

        if (isLong) return side == Side.SELL;
        else return side == Side.BUY;
    }

    function _prorateMargin(int256 margin, uint256 assetNotional, uint256 totalNotional)
        internal
        pure
        returns (int256 proratedMargin)
    {
        if (totalNotional == 0) return 0;

        proratedMargin = margin.abs().fullMulDiv(assetNotional, totalNotional).toInt256();

        if (margin < 0) proratedMargin = -proratedMargin;
    }

    function _processTakerFill(ClearingHouse storage self, __FillParams__ memory params) internal {
        __ProcessTakerFillCache__ memory cache;

        // load assets
        cache.assets = self.assets[params.account][params.subaccount].values().wrap();

        // check if new position
        bool isNewPosition = !cache.assets.contains(params.asset);

        // if new position, check if asset can be added to account
        // if not, revert
        if (isNewPosition) {
            if (!_assetCanBeAddedToAccount(cache.assets, params.asset)) revert CrossMarginIsDisabled();
            // add asset to account
            cache.assets.p(params.asset);
        }

        // load positions
        cache.positions = _getPositions(self, cache.assets, params.account, params.subaccount, isNewPosition);

        // get funding payment
        cache.fundingPayment = realizeFundingPayment(cache.assets, cache.positions);

        // get index of traded position
        uint256 positionIdx = cache.assets.indexOf(params.asset);

        // process the trade
        cache.positionResult = cache.positions[positionIdx].processTrade({
            side: params.side,
            quoteTraded: params.quoteAmount,
            baseTraded: params.baseAmount
        });

        cache.takerFee = StorageLib.loadFeeManager().getTakerFee(params.account, params.quoteAmount);

        cache.margin = StorageLib.loadCollateralManager().getMarginBalance(params.account, params.subaccount);

        // settle rpnl on margin
        cache.margin += cache.positionResult.rpnl - cache.fundingPayment - cache.takerFee.toInt256();

        // rebalance account
        (cache.margin, cache.positionResult.marginDelta) = self.rebalanceAccount({
            assets: cache.assets,
            positions: cache.positions,
            margin: cache.margin,
            marginDelta: cache.positionResult.marginDelta
        });

        // check liquidatability
        self.assertNotLiquidatable(cache.assets, cache.positions, cache.margin);

        StorageLib.loadInsuranceFund().pay(cache.takerFee);

        StorageLib.loadCollateralManager().settleFill(
            params.account,
            params.subaccount,
            cache.margin,
            cache.positionResult.marginDelta + params.collateralPosted.toInt256()
        );

        self.updateAccount({
            account: params.account,
            subaccount: params.subaccount,
            assets: cache.assets,
            positions: cache.positions,
            tradedAsset: params.asset,
            positionIdx: positionIdx,
            oiDelta: cache.positionResult.oiDelta,
            sideClose: cache.positionResult.sideClose
        });
    }

    function _getIntendedMarginAndUpnl(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions
    ) internal view returns (uint256 totalIntendedMargin, int256 totalUpnl) {
        uint256 length = assets.length();

        uint256 intendedMargin;
        int256 upnl;
        for (uint256 i; i < length; ++i) {
            (intendedMargin, upnl) = self.market[assets.getBytes32(i)].getIntendedMarginAndUpnl(positions[i]);

            totalIntendedMargin += intendedMargin;
            totalUpnl += upnl;
        }
    }

    function _getPositions(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        address account,
        uint256 subaccount,
        bool newPosition
    ) internal view returns (Position[] memory positions) {
        uint256 length = assets.length();

        if (length == 0) return positions;

        positions = new Position[](length);

        for (uint256 i; i < length - 1; ++i) {
            positions[i] = self.market[assets.getBytes32(i)].getPosition(account, subaccount);
        }

        if (newPosition) {
            positions[length - 1].leverage =
                self.market[assets.getBytes32(length - 1)].getPositionLeverage(account, subaccount);
        } else {
            positions[length - 1] = self.market[assets.getBytes32(length - 1)].getPosition(account, subaccount);
        }
    }

    function _assetCanBeAddedToAccount(DynamicArrayLib.DynamicArray memory assets, bytes32 asset)
        private
        view
        returns (bool canBeAdded)
    {
        uint256 numPositions = assets.length();

        // check incoming asset
        if (numPositions == 0) return true;
        if (assets.contains(asset)) return true;
        if (!StorageLib.loadMarketSettings(asset).crossMarginEnabled) return false;

        // check existing assets
        for (uint256 i; i < numPositions; ++i) {
            if (!StorageLib.loadMarketSettings(assets.getBytes32(i)).crossMarginEnabled) return false;
        }

        return true;
    }

    function _getDeltas(Side side, uint256 quoteTraded, uint256 baseTraded)
        private
        pure
        returns (int256 quoteDelta, int256 baseDelta)
    {
        if (side == Side.BUY) {
            quoteDelta = -quoteTraded.toInt256();
            baseDelta = baseTraded.toInt256();
        } else {
            quoteDelta = quoteTraded.toInt256();
            baseDelta = -baseTraded.toInt256();
        }
    }

    function _movePop(DynamicArrayLib.DynamicArray memory array, bytes32 asset) private pure {
        uint256 index = array.indexOf(asset);

        if (index == type(uint256).max) return;

        array.set(index, asset);
        array.pop();
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               ASSERTIONS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function assertNotLiquidatable(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions,
        int256 margin
    ) internal view {
        if (self.isLiquidatable(assets, positions, margin, BookType.STANDARD)) revert Liquidatable();
    }

    function assertLiquidatable(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions,
        int256 margin,
        BookType bookType
    ) internal view {
        if (!self.isLiquidatable(assets, positions, margin, bookType)) revert NotLiquidatable();
    }

    function assertPostWithdrawalMarginRequired(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions,
        int256 margin
    ) internal view {
        if (margin < 0) revert MarginRequirementUnmet();

        (uint256 intendedMargin, int256 upnl) = _getIntendedMarginAndUpnl(self, assets, positions);
        uint256 totalNotional = self.getNotionalAccountValue(assets, positions);

        intendedMargin = intendedMargin.max(totalNotional / 10);

        if (margin + upnl < intendedMargin.toInt256()) revert MarginRequirementUnmet();
    }

    /// @notice asserts min open margin requirement is met after margin updates
    function assertOpenMarginRequired(
        ClearingHouse storage self,
        DynamicArrayLib.DynamicArray memory assets,
        Position[] memory positions,
        int256 margin
    ) internal view {
        if (!self.isOpenMarginRequirementMet(assets, positions, margin)) revert MarginRequirementUnmet();
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {DynamicArrayLib} from "@solady/utils/DynamicArrayLib.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";

import {Status, BookType, TiF, Side} from "./Enums.sol";
import {
    PlaceOrderArgs,
    PlaceOrderResult,
    AmendLimitOrderArgs,
    Condition,
    FundingPaymentResult,
    LiquidateData,
    BackstopLiquidateData,
    OIDelta
} from "./Structs.sol";

import {CLOBLib} from "./CLOBLib.sol";
import {StorageLib} from "./StorageLib.sol";

import {Position} from "./Position.sol";
import {FundingRateEngine, FundingRateSettings} from "./FundingRateEngine.sol";
import {PriceHistory} from "./PriceHistory.sol";

struct MarketSettings {
    Status status;
    bool crossMarginEnabled; // true if there can be more than 1 position open per subaccount
    uint256 maxOpenLeverage;
    uint256 maintenanceMarginRatio;
    uint256 liquidationFeeRate;
    uint256 divergenceCap;
    uint256 reduceOnlyCap;
    uint256 partialLiquidationThreshold; // position min position value to partial liquidate
    uint256 partialLiquidationRate; // percentage of position to partially liquidate
}

struct MarketMetadata {
    uint256 longOI;
    uint256 shortOI;
    PriceHistory markPriceHistory;
    PriceHistory indexPriceHistory;
    PriceHistory impactPriceHistory;
    PriceHistory basisSpreadHistory;
}

struct Market {
    bytes32 asset;
    uint256 markPrice;
    mapping(address account => mapping(uint256 subaccount => Position)) position;
    mapping(address account => mapping(uint256 subaccount => uint256[])) reduceOnlyOrders;
    mapping(address account => mapping(uint256 subaccount => uint256[])) reduceOnlyOrdersBackstopBook;
    mapping(address account => mapping(uint256 subaccount => uint256)) orderbookNotional;
}

using MarketLib for Market global;
using MarketLib for MarketSettings global;

library MarketLib {
    using FixedPointMathLib for *;
    using SafeCastLib for *;
    using DynamicArrayLib for uint256[];

    event PositionLiquidated(
        bytes32 asset,
        address indexed account,
        uint256 indexed subaccount,
        int256 quoteDelta,
        int256 baseDelta,
        int256 rpnl,
        Position position,
        BookType liquidationType,
        uint256 nonce
    );

    event FundingSettled(bytes32 indexed asset, int256 funding, int256 cumulativeFunding, uint256 openInterest, uint256 nonce);

    event MarkPriceUpdated(bytes32 indexed asset, uint256 markPrice, uint256 p1, uint256 p2, uint256 p3, uint256 nonce);

    error MarketInactive();
    error InvalidReduceOnlyDenomination();
    error MaxLeverageExceeded();
    error LeverageInvalid();
    error InvalidBackstopOrder();
    error ZeroTrade();
    error ZeroOrder();
    error NotReduceOnly();
    error ReduceOnlyCapExceeded();
    error BackstopOrderNotPostOnly();
    error PartialBackstopLiquidation();
    error InvalidDeleveragePair();

    modifier onlyActiveMarket(bytes32 asset) {
        assertActive(asset);
        _;
    }

    function init(
        Market storage self,
        bytes32 asset,
        MarketSettings memory marketSettings,
        FundingRateSettings memory fundingSettings,
        uint256 initialPrice
    ) internal {
        self.asset = asset;
        self.markPrice = initialPrice;

        StorageLib.loadMarketSettings(asset).init(marketSettings);
        StorageLib.loadFundingRateSettings(asset).init(fundingSettings);
        StorageLib.loadFundingRateEngine(asset).lastFundingTime = block.timestamp;
    }

    function init(MarketSettings storage settings, MarketSettings memory initSettings) internal {
        settings.status = initSettings.status;
        settings.crossMarginEnabled = initSettings.crossMarginEnabled;
        settings.maxOpenLeverage = initSettings.maxOpenLeverage;
        settings.maintenanceMarginRatio = initSettings.maintenanceMarginRatio;
        settings.liquidationFeeRate = initSettings.liquidationFeeRate;
        settings.divergenceCap = initSettings.divergenceCap;
        settings.reduceOnlyCap = initSettings.reduceOnlyCap;
        settings.partialLiquidationThreshold = initSettings.partialLiquidationThreshold;
        settings.partialLiquidationRate = initSettings.partialLiquidationRate;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                             STANDARD BOOK
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function placeOrder(Market storage self, address account, PlaceOrderArgs calldata args, BookType bookType)
        internal
        onlyActiveMarket(args.asset)
        returns (PlaceOrderResult memory result)
    {
        // sanity check: non liquidation taker order can't be placed on the backstop book
        if (bookType == BookType.BACKSTOP && args.tif != TiF.MOC) revert InvalidBackstopOrder();

        if (args.reduceOnly) {
            _validateReduceOnlyOrder({
                self: self,
                account: account,
                subaccount: args.subaccount,
                orderAmount: args.amount,
                side: args.side,
                baseDenominated: args.baseDenominated
            });
        }

        return CLOBLib.placeOrder(account, args, bookType);
    }

    function amendLimitOrder(Market storage self, address account, AmendLimitOrderArgs calldata args, BookType bookType)
        internal
        onlyActiveMarket(args.asset)
        returns (int256 collateralDelta)
    {
        if (args.reduceOnly) _validateReduceOnlyOrder(self, account, args.subaccount, args.baseAmount, args.side, true);

        return CLOBLib.amend(account, args, bookType);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                              LIQUIDATIONS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function liquidate(
        Market storage self,
        address account,
        uint256 subaccount,
        Side side,
        uint256 amount,
        BookType bookType
    ) internal returns (PlaceOrderResult memory result) {
        if (bookType == BookType.STANDARD) amount = _getLiquidationAmount(self, amount);

        result = CLOBLib.placeOrder(
            account,
            PlaceOrderArgs({
                subaccount: subaccount,
                asset: self.asset,
                side: side,
                limitPrice: 0, // max slippage
                amount: amount,
                baseDenominated: true,
                tif: TiF.IOC,
                expiryTime: 0,
                clientOrderId: 0,
                reduceOnly: true
            }),
            bookType
        );
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                              SETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function settleFunding(Market storage self) internal {
        bytes32 asset = self.asset;

        FundingRateEngine storage fundingRateEngine = StorageLib.loadFundingRateEngine(asset);
        MarketMetadata storage metadata = StorageLib.loadMarketMetadata(asset);

        uint256 interval = fundingRateEngine.getTimeSinceLastFunding();

        (int256 funding, int256 cumulativeFunding) = fundingRateEngine.settleFunding({
            asset: asset,
            markTwap: metadata.markPriceHistory.twap(interval),
            indexTwap: metadata.indexPriceHistory.twap(interval)
        });

        emit FundingSettled({
            asset: asset,
            funding: funding,
            cumulativeFunding: cumulativeFunding,
            openInterest: metadata.longOI,
            nonce: StorageLib.incNonce()
        });
    }

    function setMarkPrice(Market storage self, uint256 indexPrice) internal returns (uint256 markPrice) {
        MarketMetadata storage metadata = StorageLib.loadMarketMetadata(self.asset);

        _cacheBasisSpread(self, indexPrice);
        _cacheImpactPrice(self);

        uint256 p1 = self.getFundingRateComponent(indexPrice);
        uint256 p2 = (indexPrice.toInt256() + self.getBasisSpreadEMA()).toUint256();
        uint256 p3 = self.getImpactPriceTwap();

        self.markPrice = markPrice = _getMedian(p1, p2, p3);

        metadata.markPriceHistory.snapshot(markPrice);
        metadata.indexPriceHistory.snapshot(indexPrice);

        emit MarkPriceUpdated({
            asset: self.asset,
            markPrice: markPrice,
            p1: p1,
            p2: p2,
            p3: p3,
            nonce: StorageLib.incNonce()
        });
    }

    function realizeFundingPayment(bytes32 asset, Position memory position)
        internal
        view
        returns (int256 fundingPayment)
    {
        return position.realizeFundingPayment(StorageLib.loadFundingRateEngine(asset).getCumulativeFunding());
    }

    function updateOI(bytes32 asset, OIDelta memory oiDelta) internal {
        MarketMetadata storage metadata = StorageLib.loadMarketMetadata(asset);

        if (oiDelta.long > 0) metadata.longOI += oiDelta.long.abs();
        else if (oiDelta.long < 0) metadata.longOI -= oiDelta.long.abs();

        if (oiDelta.short > 0) metadata.shortOI += oiDelta.short.abs();
        else if (oiDelta.short < 0) metadata.shortOI -= oiDelta.short.abs();
    }

    function setPosition(Market storage self, address account, uint256 subaccount, Position memory position) internal {
        self.position[account][subaccount] = position;
    }

    function cancelCloseOrders(Market storage self, address account, uint256 subaccount) internal {
        _cancelReduceOnlyOrdersStandard(self, account, subaccount);
        _cancelReduceOnlyOrdersBackstop(self, account, subaccount);
    }

    function linkReduceOnlyOrder(
        Market storage self,
        address account,
        uint256 subaccount,
        uint256 orderId,
        BookType bookType
    ) internal {
        if (bookType == BookType.STANDARD) _linkReduceOnlyStandard(self, account, subaccount, orderId);
        else _linkReduceOnlyBackstop(self, account, subaccount, orderId);
    }

    function unlinkReduceOnlyOrder(
        Market storage self,
        address account,
        uint256 subaccount,
        uint256 orderId,
        BookType bookType
    ) internal {
        if (bookType == BookType.STANDARD) _unlinkIdFromArray(self.reduceOnlyOrders[account][subaccount], orderId);
        else _unlinkIdFromArray(self.reduceOnlyOrdersBackstopBook[account][subaccount], orderId);
    }

    function updateOrderbookNotional(Market storage self, address account, uint256 subaccount, int256 notionalDelta)
        internal
    {
        if (notionalDelta > 0) self.orderbookNotional[account][subaccount] += notionalDelta.abs();
        else if (notionalDelta < 0) self.orderbookNotional[account][subaccount] -= notionalDelta.abs();
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getUpnl(Market storage self, address account, uint256 subaccount) internal view returns (int256 upnl) {
        Position storage position = self.position[account][subaccount];

        uint256 currentNotional = position.amount.fullMulDiv(self.markPrice, 1e18);

        return _calcUpnl(position.isLong, position.openNotional, currentNotional);
    }

    function getUpnlAndMinMargin(Market storage self, Position memory position, BookType bookType)
        internal
        view
        returns (int256 upnl, uint256 minMargin)
    {
        if (position.amount == 0) return (0, 0);

        uint256 currentNotional = position.amount.fullMulDiv(self.markPrice, 1e18);

        upnl = _calcUpnl(position.isLong, position.openNotional, currentNotional);
        minMargin = currentNotional.fullMulDiv(self.getMinMarginRatio(bookType), 1e18);
    }

    function getNotionalValue(Market storage self, Position memory position) internal view returns (uint256 notional) {
        return position.amount.fullMulDiv(self.markPrice, 1e18);
    }

    function getIntendedMargin(Market storage self, Position memory position)
        internal
        view
        returns (uint256 intendedMargin)
    {
        if (position.amount == 0) return 0;

        uint256 currentNotional = position.amount.fullMulDiv(self.markPrice, 1e18);

        intendedMargin = currentNotional.fullMulDiv(1e18, position.leverage);
    }

    function getMinOpenMargin(Market storage self, uint256 positionAmount)
        internal
        view
        returns (uint256 minOpenMargin)
    {
        uint256 positionNotional = positionAmount.fullMulDiv(self.markPrice, 1e18);

        minOpenMargin = positionNotional.fullMulDiv(1e18, StorageLib.loadMarketSettings(self.asset).maxOpenLeverage);
    }

    function getIntendedMarginAndUpnl(Market storage self, Position memory position)
        internal
        view
        returns (uint256 intendedMargin, int256 upnl)
    {
        if (position.amount == 0) return (0, 0);

        uint256 currentNotional = position.amount.fullMulDiv(self.markPrice, 1e18);

        upnl = _calcUpnl(position.isLong, position.openNotional, currentNotional);
        intendedMargin = currentNotional.fullMulDiv(1e18, position.leverage);
    }

    function getUpnl(Market storage self, Position memory position) internal view returns (int256 upnl) {
        uint256 currentNotional = position.amount.fullMulDiv(self.markPrice, 1e18);

        return _calcUpnl(position.isLong, position.openNotional, currentNotional);
    }

    function getPosition(Market storage self, address account, uint256 subaccount)
        internal
        view
        returns (Position memory position)
    {
        position = self.position[account][subaccount];

        if (position.leverage == 0) position.leverage = 1e18; // default leverage
    }

    function getPositionLeverage(Market storage self, address account, uint256 subaccount)
        internal
        view
        returns (uint256 leverage)
    {
        leverage = self.position[account][subaccount].leverage;

        if (leverage == 0) leverage = 1e18; // default leverage
    }

    function getMaintenanceMargin(Market storage self, uint256 positionAmount) internal view returns (uint256) {
        return positionAmount.fullMulDiv(self.markPrice, 1e18).fullMulDiv(
            StorageLib.loadMarketSettings(self.asset).maintenanceMarginRatio, 1e18
        );
    }

    function getFundingPayment(Market storage self, address account, uint256 subaccount)
        internal
        view
        returns (int256 fundingPayment)
    {
        Position storage position = self.position[account][subaccount];
        return position.realizeFundingPayment(StorageLib.loadFundingRateEngine(self.asset).getCumulativeFunding());
    }

    function getMaxDivergingBidPrice(Market storage self) internal view returns (uint256) {
        uint256 mark = self.markPrice;
        uint256 maxDivergence = mark.fullMulDiv(StorageLib.loadMarketSettings(self.asset).divergenceCap, 1e18);

        return mark - maxDivergence;
    }

    function getMaxDivergingAskPrice(Market storage self) internal view returns (uint256) {
        uint256 mark = self.markPrice;
        uint256 maxDivergence = mark.fullMulDiv(StorageLib.loadMarketSettings(self.asset).divergenceCap, 1e18);

        return mark + maxDivergence;
    }

    function getImpactPrice(Market storage self, uint256 impactNotional) internal view returns (uint256 impactPrice) {
        (uint256 baseAmount, uint256 quoteUsed) = StorageLib.loadBook(self.asset).quoteBidInQuote(impactNotional);

        if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, type(uint256).max);

        uint256 impactBid = baseAmount == 0 ? 0 : impactNotional.fullMulDiv(1e18, baseAmount);

        (baseAmount, quoteUsed) = StorageLib.loadBook(self.asset).quoteAskInQuote(impactNotional);

        if (impactNotional > quoteUsed) baseAmount += (impactNotional - quoteUsed).fullMulDiv(1e18, 1);

        uint256 impactAsk = impactNotional.fullMulDiv(1e18, baseAmount);

        return (impactBid + impactAsk) / 2;
    }

    function getMidPrice(Market storage self) internal view returns (uint256 midPrice) {
        bytes32 asset = self.asset;

        uint256 bestBid = StorageLib.loadBook(asset).getBestBid();
        uint256 bestAsk = StorageLib.loadBook(asset).getBestAsk();

        if (bestAsk == type(uint256).max || bestBid == 0) return 0;

        return (bestBid + bestAsk) / 2;
    }

    function getFundingRateComponent(Market storage self, uint256 indexPrice)
        internal
        view
        returns (uint256 fundingRateComponent)
    {
        bytes32 asset = self.asset;
        FundingRateEngine storage fundingRateEngine = StorageLib.loadFundingRateEngine(asset);

        return indexPrice.fullMulDiv(
            1e18
                + fundingRateEngine.fundingRate.abs().fullMulDiv(
                    block.timestamp - fundingRateEngine.lastFundingTime, fundingRateEngine.getFundingInterval(asset)
                ),
            1e18
        );
    }

    function getBasisSpreadEMA(Market storage self) internal view returns (int256 basisSpreadEMA) {
        return StorageLib.loadMarketMetadata(self.asset).basisSpreadHistory.ema(15 minutes);
    }

    function getImpactPriceTwap(Market storage self) internal view returns (uint256 impactPriceTwap) {
        bytes32 asset = self.asset;
        return StorageLib.loadMarketMetadata(asset).impactPriceHistory.twap(
            StorageLib.loadFundingRateEngine(asset).getFundingInterval(asset)
        );
    }

    /// @dev The subaccount's margin that is locked up in makes
    function getOrderBookCollateral(Market storage self, address account, uint256 subaccount)
        internal
        view
        returns (uint256 orderbookCollateral)
    {
        return
            self.orderbookNotional[account][subaccount].fullMulDiv(1e18, self.getPositionLeverage(account, subaccount));
    }

    function getMinMarginRatio(Market storage self, BookType liquidationType) internal view returns (uint256) {
        uint256 denominator = liquidationType == BookType.STANDARD ? 1 : 3;

        return StorageLib.loadMarketSettings(self.asset).maintenanceMarginRatio / denominator;
    }

    function isTPSLConditionMet(Market storage self, Side side, Condition calldata condition)
        internal
        view
        returns (bool met)
    {
        uint256 mark = self.markPrice;

        if (side == Side.SELL) {
            if (condition.stopLoss ? mark <= condition.triggerPrice : mark >= condition.triggerPrice) return true;
        } else {
            if (condition.stopLoss ? mark >= condition.triggerPrice : mark <= condition.triggerPrice) return true;
        }
    }

    function exists(Market storage self) internal view returns (bool) {
        return self.asset != bytes32(0);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               ASSERTIONS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function assertMaxLeverage(bytes32 asset, uint256 leverage) internal view {
        if (leverage < 1e18) revert LeverageInvalid(); // leverage must be at least 1x
        if (leverage > StorageLib.loadMarketSettings(asset).maxOpenLeverage) revert MaxLeverageExceeded();
    }

    function assertActive(bytes32 asset) internal view {
        if (StorageLib.loadMarketSettings(asset).status != Status.ACTIVE) revert MarketInactive();
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               HELPERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _validateReduceOnlyOrder(
        Market storage self,
        address account,
        uint256 subaccount,
        uint256 orderAmount,
        Side side,
        bool baseDenominated
    ) internal view {
        // not possible to validate reduce only on quote denominated orders
        if (!baseDenominated) revert InvalidReduceOnlyDenomination();

        Position storage position = self.position[account][subaccount];

        if (position.amount < orderAmount) revert NotReduceOnly();

        if (side == Side.BUY && position.isLong) revert NotReduceOnly();
        else if (side == Side.SELL && !position.isLong) revert NotReduceOnly();
    }

    function _linkReduceOnlyStandard(Market storage self, address account, uint256 subaccount, uint256 orderId)
        private
    {
        if (
            self.reduceOnlyOrders[account][subaccount].length >= StorageLib.loadMarketSettings(self.asset).reduceOnlyCap
        ) revert ReduceOnlyCapExceeded();

        if (self.reduceOnlyOrders[account][subaccount].contains(orderId)) return;
        self.reduceOnlyOrders[account][subaccount].push(orderId);
    }

    function _linkReduceOnlyBackstop(Market storage self, address account, uint256 subaccount, uint256 orderId)
        private
    {
        if (
            self.reduceOnlyOrdersBackstopBook[account][subaccount].length
                >= StorageLib.loadMarketSettings(self.asset).reduceOnlyCap
        ) revert ReduceOnlyCapExceeded();

        if (self.reduceOnlyOrdersBackstopBook[account][subaccount].contains(orderId)) return;
        self.reduceOnlyOrdersBackstopBook[account][subaccount].push(orderId);
    }

    function _getLiquidationAmount(Market storage self, uint256 positionAmount) internal view returns (uint256) {
        if (
            positionAmount.fullMulDiv(self.markPrice, 1e18)
                < StorageLib.loadMarketSettings(self.asset).partialLiquidationThreshold
        ) return positionAmount;

        return positionAmount.fullMulDiv(StorageLib.loadMarketSettings(self.asset).partialLiquidationRate, 1e18);
    }

    function _cancelReduceOnlyOrdersStandard(Market storage self, address account, uint256 subaccount) private {
        uint256[] memory orderIds = self.reduceOnlyOrders[account][subaccount];

        if (orderIds.length == 0) return;

        CLOBLib.cancel(self.asset, account, subaccount, orderIds, BookType.STANDARD);

        delete self.reduceOnlyOrders[account][subaccount];
    }

    function _cancelReduceOnlyOrdersBackstop(Market storage self, address account, uint256 subaccount) private {
        uint256[] memory orderIds = self.reduceOnlyOrdersBackstopBook[account][subaccount];

        if (orderIds.length == 0) return;

        CLOBLib.cancel(self.asset, account, subaccount, orderIds, BookType.BACKSTOP);

        delete self.reduceOnlyOrdersBackstopBook[account][subaccount];
    }

    function _calcUpnl(bool isLong, uint256 openNotional, uint256 currentNotional) private pure returns (int256 upnl) {
        if (isLong) upnl = currentNotional.toInt256() - openNotional.toInt256();
        else upnl = openNotional.toInt256() - currentNotional.toInt256();
    }

    function _unlinkIdFromArray(uint256[] storage ids, uint256 id) private {
        uint256 index = ids.indexOf(id);

        if (index == type(uint256).max) return;

        ids[index] = ids[ids.length - 1];

        ids.pop();
    }

    function _isInProfit(uint256 markPrice, uint256 openPrice, bool isLong) private pure returns (bool inProfit) {
        if (isLong) return markPrice >= openPrice;
        else return markPrice <= openPrice;
    }

    function _cacheBasisSpread(Market storage self, uint256 indexPrice) private {
        uint256 midPrice = self.getMidPrice();

        if (midPrice == 0) return; // no valid mid price available

        int256 basisSpread = midPrice.toInt256() - indexPrice.toInt256();

        StorageLib.loadMarketMetadata(self.asset).basisSpreadHistory.snapshotBasisSpread(basisSpread);
    }

    function _cacheImpactPrice(Market storage self) private returns (uint256 impactPrice) {
        // impact notional is 500 * max leverage
        uint256 impactNotional =
            uint256(500e18).fullMulDiv(StorageLib.loadMarketSettings(self.asset).maxOpenLeverage, 1e18);

        impactPrice = self.getImpactPrice(impactNotional);

        StorageLib.loadMarketMetadata(self.asset).impactPriceHistory.snapshot(impactPrice);
    }

    function _getMedian(uint256 a, uint256 b, uint256 c) private pure returns (uint256 median) {
        uint256 maxAB = a.max(b);
        uint256 minAB = a.min(b);

        return minAB.max(maxAB.min(c));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

library Constants {
    // ADDRESSES
    address constant USDC = 0xE9b6e75C243B6100ffcb1c66e8f78F96FeeA727F;
    address constant GTL = 0x037eDa3aDB1198021A9b2e88C22B464fD38db3f3;
    // ROLES
    uint256 constant ADMIN_ROLE = 1 << 7;
    uint256 constant KEEPER_ROLE = 1 << 6;
    uint256 constant LIQUIDATOR_ROLE = 1 << 5;
    uint256 constant BACKSTOP_LIQUIDATOR_ROLE = 1 << 4;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {LiquidatorData} from "./Structs.sol";

library BackstopLiquidatorDataLib {
    /// erc7201('TransientLiquidators')
    bytes32 constant TRANSIENT_LIQUIDATORS_SLOT = 0x4241b72dd798242510fb56f3af1b11b473993219eaff939db6095ef4a72ad900;
    /// erc7201('TransientVolume')
    bytes32 constant TRANSIENT_VOLUME_SLOT = 0xddbbbd6c2145904e746c66ce13af468e7d054181c91f1b7fbae06864da072000;

    function addLiquidatorVolume(address liquidator, uint256 volume) internal {
        bytes32 slot = keccak256(abi.encode(TRANSIENT_VOLUME_SLOT, liquidator));

        bool exists;
        assembly ("memory-safe") {
            exists := iszero(iszero(tload(slot)))

            if iszero(exists) { tstore(slot, 1) }

            let totalVolume := tload(add(slot, 1))

            tstore(add(slot, 1), add(totalVolume, volume))
        }

        if (!exists) _addLiquidator(liquidator);
    }

    function getLiquidatorDataAndClearStorage() internal returns (LiquidatorData[] memory liquidatorData) {
        address[] memory liquidators = _getLiquidatorsAndClear();

        uint256 length = liquidators.length;

        liquidatorData = new LiquidatorData[](length);

        uint256 volume;
        for (uint256 i; i < length; i++) {
            volume = _getVolumeAndClear(liquidators[i]);

            liquidatorData[i] = LiquidatorData({liquidator: liquidators[i], volume: volume});
        }
    }

    function _addLiquidator(address liquidator) internal {
        bytes32 slot = TRANSIENT_LIQUIDATORS_SLOT;

        assembly ("memory-safe") {
            let len := tload(slot)

            mstore(0x00, slot)

            let dataSlot := keccak256(0x00, 0x20)

            tstore(add(dataSlot, len), liquidator)
            tstore(slot, add(len, 1))
        }
    }

    function _getLiquidatorsAndClear() internal returns (address[] memory liquidators) {
        bytes32 slot = TRANSIENT_LIQUIDATORS_SLOT;

        assembly ("memory-safe") {
            let len := tload(slot)

            liquidators := mload(0x40)

            mstore(liquidators, len)

            mstore(0x00, slot)

            let dataSlot := keccak256(0x00, 0x20)
            let memPointer := add(liquidators, 0x20)

            for { let i := 0 } lt(i, len) { i := add(i, 1) } {
                mstore(add(memPointer, mul(i, 0x20)), tload(add(dataSlot, i)))
                tstore(add(dataSlot, i), 0) // clear maker
            }

            mstore(0x40, add(memPointer, mul(len, 0x20)))
            tstore(slot, 0) // clear length
        }
    }

    function _getVolumeAndClear(address liquidator) internal returns (uint256 volume) {
        bytes32 slot = keccak256(abi.encode(TRANSIENT_VOLUME_SLOT, liquidator));

        assembly ("memory-safe") {
            volume := tload(add(slot, 1))

            tstore(slot, 0)
            tstore(add(slot, 1), 0)
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";

import {Side} from "./Enums.sol";
import {FundingPaymentResult, PositionUpdateResult, OIDelta} from "./Structs.sol";

struct Position {
    bool isLong;
    uint256 amount;
    uint256 openNotional;
    uint256 leverage;
    int256 lastCumulativeFunding;
}

using PositionLib for Position global;

library PositionLib {
    using FixedPointMathLib for *;
    using SafeCastLib for uint256;

    struct __CloseCache__ {
        uint256 closeSize;
        uint256 closedOpenNotional;
        uint256 currentNotional;
        uint256 marginRemoved;
        int256 remainingMargin;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                          POSITION MANAGEMENT
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function processTrade(Position memory self, Side side, uint256 quoteTraded, uint256 baseTraded)
        internal
        pure
        returns (PositionUpdateResult memory result)
    {
        bool openLong = side == Side.BUY && (self.isLong || self.amount == 0);
        bool openShort = side == Side.SELL && (!self.isLong || self.amount == 0);

        if (openLong || openShort) {
            result.marginDelta = _open(self, side, quoteTraded, baseTraded);

            if (side == Side.BUY) result.oiDelta.long += baseTraded.toInt256();
            else result.oiDelta.short += baseTraded.toInt256();
        } else {
            result = _close(self, side, quoteTraded, baseTraded);
        }
    }

    function realizeFundingPayment(Position memory self, int256 cumulativeFunding)
        internal
        pure
        returns (int256 fundingPayment)
    {
        if (self.lastCumulativeFunding == cumulativeFunding) return 0;

        fundingPayment = _getFundingPayment({
            amount: self.isLong ? self.amount.toInt256() : -self.amount.toInt256(),
            lastCumulativePremiumFunding: self.lastCumulativeFunding,
            cumulativePremiumFunding: cumulativeFunding
        });

        self.lastCumulativeFunding = cumulativeFunding;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               FILL LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _open(Position memory self, Side side, uint256 quoteTraded, uint256 baseTraded)
        private
        pure
        returns (int256 marginDelta)
    {
        if (self.leverage == 0) self.leverage = 1e18; // default leverage

        self.isLong = side == Side.BUY;

        self.amount += baseTraded;
        self.openNotional += quoteTraded;

        marginDelta = quoteTraded.fullMulDiv(1e18, self.leverage).toInt256();
    }

    /// @dev covers decrease, close, reverse open
    function _close(Position memory self, Side side, uint256 quoteTraded, uint256 baseTraded)
        private
        pure
        returns (PositionUpdateResult memory result)
    {
        __CloseCache__ memory cache;

        cache.closeSize = self.amount.min(baseTraded);

        // pro rate quote amounts by close
        cache.closedOpenNotional = self.openNotional.fullMulDiv(cache.closeSize, self.amount);
        cache.currentNotional = quoteTraded.fullMulDiv(cache.closeSize, baseTraded);

        result.rpnl = _pnl(self.isLong, cache.closedOpenNotional, cache.currentNotional);
        result.marginDelta = -cache.closedOpenNotional.fullMulDiv(1e18, self.leverage).toInt256();

        self.openNotional -= cache.closedOpenNotional;
        self.amount -= cache.closeSize;

        quoteTraded -= cache.currentNotional;
        baseTraded -= cache.closeSize;

        if (self.isLong) result.oiDelta.long = -cache.closeSize.toInt256();
        else result.oiDelta.short = -cache.closeSize.toInt256();

        if (result.sideClose = self.amount == 0) {
            // reverse open
            if (baseTraded > 0) {
                result.marginDelta = _open(self, side, quoteTraded, baseTraded);

                if (self.isLong) result.oiDelta.long += baseTraded.toInt256();
                else result.oiDelta.short += baseTraded.toInt256();
            } else {
                // full close, set to defaults
                delete self.lastCumulativeFunding;
                delete self.isLong;
            }
        }
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                HELPERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _getFundingPayment(int256 amount, int256 lastCumulativePremiumFunding, int256 cumulativePremiumFunding)
        private
        pure
        returns (int256)
    {
        if (amount == 0) return 0;

        return _mul(amount, cumulativePremiumFunding - lastCumulativePremiumFunding);
    }

    /// @dev wrapper for fullMulDiv to handle int256
    function _mul(int256 amt, int256 fundingDelta) private pure returns (int256) {
        uint256 result = amt.abs().fullMulDiv(fundingDelta.abs(), 1e18);
        return amt < 0 != fundingDelta < 0 ? -result.toInt256() : result.toInt256();
    }

    function _pnl(bool isLong, uint256 openNotional, uint256 currentNotional) private pure returns (int256 pnl) {
        if (isLong) pnl = currentNotional.toInt256() - openNotional.toInt256();
        else pnl = openNotional.toInt256() - currentNotional.toInt256();
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {RedBlackTreeLib} from "@solady/utils/RedBlackTreeLib.sol";

uint256 constant MIN = 0;
uint256 constant MAX = type(uint256).max;

struct RedBlackTree {
    RedBlackTreeLib.Tree tree;
}

using BookRedBlackTreeLib for RedBlackTree global;

library BookRedBlackTreeLib {
    /// @dev sig: 0x2b72e905
    error NodeKeyInvalid();

    function size(RedBlackTree storage tree) internal view returns (uint256) {
        return RedBlackTreeLib.size(tree.tree);
    }

    /// @dev Returns the minimum value in the tree, or type(uint256).max if the tree is empty
    function minimum(RedBlackTree storage tree) internal view returns (uint256) {
        bytes32 result = RedBlackTreeLib.first(tree.tree);

        if (result == bytes32(0)) return type(uint256).max;

        return RedBlackTreeLib.value(result);
    }

    /// @dev Returns the maximum value in the tree, or type(uint256).min if the tree is empty
    function maximum(RedBlackTree storage tree) internal view returns (uint256) {
        bytes32 result = RedBlackTreeLib.last(tree.tree);

        if (result == bytes32(0)) return type(uint256).min;

        return RedBlackTreeLib.value(result);
    }

    function contains(RedBlackTree storage tree, uint256 nodeKey) internal view returns (bool) {
        return RedBlackTreeLib.exists(tree.tree, nodeKey);
    }

    /// @dev Returns the nearest key greater than `nodeKey`, checking if nodeKey exists.
    /// @dev If nodeKey is the maximum, returns MIN.
    function getNextBiggest(RedBlackTree storage tree, uint256 nodeKey) internal view returns (uint256) {
        if (nodeKey == tree.maximum()) return MAX;
        if (nodeKey == uint256(type(uint256).max)) revert NodeKeyInvalid();

        bytes32 result = RedBlackTreeLib.nearestAfter(tree.tree, nodeKey + 1);
        return RedBlackTreeLib.value(result);
    }

    /// @dev Returns the nearest key less than `nodeKey`, checking if nodeKey exists.
    /// @dev If nodeKey is the minimum, returns MAX.
    function getNextSmallest(RedBlackTree storage tree, uint256 nodeKey) internal view returns (uint256) {
        if (nodeKey == tree.minimum()) return MIN;
        if (nodeKey == 0) revert NodeKeyInvalid();

        bytes32 result = RedBlackTreeLib.nearestBefore(tree.tree, nodeKey - 1);
        return RedBlackTreeLib.value(result);
    }

    function insert(RedBlackTree storage tree, uint256 nodeKey) internal {
        RedBlackTreeLib.insert(tree.tree, nodeKey);
    }

    function remove(RedBlackTree storage tree, uint256 nodeKey) internal {
        RedBlackTreeLib.remove(tree.tree, nodeKey);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {SafeTransferLib} from "@solady/utils/SafeTransferLib.sol";
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";

import {MakerSettleData, TakerSettleData, LiquidateeSettleData} from "./Structs.sol";
import {Constants} from "./Constants.sol";

struct CollateralManager {
    mapping(address account => mapping(uint256 subaccount => int256)) margin;
    mapping(address account => uint256) freeCollateral; // collateral not tied to any subaccount
}

using CollateralManagerLib for CollateralManager global;

library CollateralManagerLib {
    using SafeTransferLib for address;
    using SafeCastLib for uint256;
    using FixedPointMathLib for *;

    address constant USDC = Constants.USDC;

    event Deposit(address indexed account, uint256 amount);
    event Withdraw(address indexed account, uint256 amount);

    error InsufficientBalance();
    error BadDebt();

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                           DEPOSIT / WITHDRAW
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function depositFreeCollateral(CollateralManager storage self, address from, address to, uint256 amount) internal {
        USDC.safeTransferFrom(from, address(this), amount);
        self.creditAccount(to, amount);
        emit Deposit(to, amount);
    }

    function withdrawFreeCollateral(CollateralManager storage self, address account, uint256 amount) internal {
        self.debitAccount(account, amount);
        USDC.safeTransfer(account, amount);
        emit Withdraw(account, amount);
    }

    function depositFromSpot(CollateralManager storage self, address account, uint256 amount) internal {
        self.creditAccount(account, amount);
        emit Deposit(account, amount);
    }

    function withdrawToSpot(CollateralManager storage self, address account, uint256 amount, address accountManager)
        internal
    {
        self.debitAccount(account, amount);
        USDC.safeTransfer(accountManager, amount);
        emit Withdraw(account, amount);
    }

    function settleMarginUpdate(
        CollateralManager storage self,
        address account,
        uint256 subaccount,
        int256 marginDelta,
        int256 fundingPayment
    ) internal returns (int256 remainingMargin) {
        remainingMargin = self.margin[account][subaccount] += marginDelta - fundingPayment;

        self.handleCollateralDelta(account, marginDelta);
    }

    function settleNewLeverage(
        CollateralManager storage self,
        address account,
        uint256 subaccount,
        int256 collateralDeltaFromBook,
        int256 newMargin,
        int256 fundingPayment
    ) internal returns (int256 collateralDelta) {
        int256 currentMargin = self.margin[account][subaccount] - fundingPayment;

        int256 collateralDeltaFromPosition = newMargin - currentMargin;

        collateralDelta = collateralDeltaFromPosition + collateralDeltaFromBook;

        self.handleCollateralDelta(account, collateralDelta);

        self.margin[account][subaccount] = newMargin;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                 TAKER
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function settleFill(
        CollateralManager storage self,
        address account,
        uint256 subaccount,
        int256 margin,
        int256 marginDelta
    ) internal {
        self.margin[account][subaccount] = margin;

        self.handleCollateralDelta(account, marginDelta);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               ACCOUNT
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function creditAccount(CollateralManager storage self, address account, uint256 amount) internal {
        self.freeCollateral[account] += amount;
    }

    function debitAccount(CollateralManager storage self, address account, uint256 amount) internal {
        if (self.freeCollateral[account] < amount) revert InsufficientBalance();
        self.freeCollateral[account] -= amount;
    }

    function handleCollateralDelta(CollateralManager storage self, address account, int256 collateralDelta) internal {
        if (collateralDelta > 0) self.debitAccount(account, collateralDelta.abs());
        else if (collateralDelta < 0) self.creditAccount(account, collateralDelta.abs());
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getFreeCollateralBalance(CollateralManager storage self, address account)
        internal
        view
        returns (uint256)
    {
        return self.freeCollateral[account];
    }

    function getMarginBalance(CollateralManager storage self, address account, uint256 subaccount)
        internal
        view
        returns (int256)
    {
        return self.margin[account][subaccount];
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";

import {StorageLib} from "./StorageLib.sol";

struct FundingRateSettings {
    uint256 fundingInterval;
    uint256 resetInterval;
    uint256 resetIterations;
    uint256 innerClamp;
    uint256 outerClamp;
    int256 interestRate;
}

struct FundingRateEngine {
    int256 fundingRate;
    int256 cumulativeFundingIndex;
    uint256 lastFundingTime;
    uint256 resetIterationsLeft;
}

using FundingLib for FundingRateEngine global;
using FundingLib for FundingRateSettings global;

library FundingLib {
    using FixedPointMathLib for *;
    using SafeCastLib for uint256;

    error FundingIntervalNotElapsed();

    function init(FundingRateSettings storage settings, FundingRateSettings memory initSettings) internal {
        settings.fundingInterval = initSettings.fundingInterval;
        settings.resetInterval = initSettings.resetInterval;
        settings.resetIterations = initSettings.resetIterations;
        settings.innerClamp = initSettings.innerClamp;
        settings.outerClamp = initSettings.outerClamp;
        settings.interestRate = initSettings.interestRate;
    }

    function settleFunding(FundingRateEngine storage self, bytes32 asset, uint256 markTwap, uint256 indexTwap)
        internal
        returns (int256 fundingIndex, int256 cumulativeFundingIndex)
    {
        FundingRateSettings storage settings = StorageLib.loadFundingRateSettings(asset);

        self.assertFundingIntervalElapsed(asset);

        self.lastFundingTime = block.timestamp;

        int256 fundingRate;
        (fundingIndex, fundingRate) = _calcFundingIndex({
            self: self,
            settings: settings,
            markTwap: markTwap.toInt256(),
            indexTwap: indexTwap.toInt256()
        });

        cumulativeFundingIndex = self.cumulativeFundingIndex += fundingIndex;
        self.fundingRate = fundingRate;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getFundingInterval(FundingRateEngine storage self, bytes32 asset) internal view returns (uint256) {
        FundingRateSettings storage settings = StorageLib.loadFundingRateSettings(asset);

        return self.resetIterationsLeft == 0 ? settings.fundingInterval : settings.resetInterval;
    }

    function getCumulativeFunding(FundingRateEngine storage self) internal view returns (int256) {
        return self.cumulativeFundingIndex;
    }

    function getTimeSinceLastFunding(FundingRateEngine storage self) internal view returns (uint256) {
        return block.timestamp - self.lastFundingTime;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               ASSERTIONS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function assertFundingIntervalElapsed(FundingRateEngine storage self, bytes32 asset) internal view {
        uint256 elapsedTime = self.getTimeSinceLastFunding();
        uint256 interval = self.getFundingInterval(asset);

        if (interval > elapsedTime) revert FundingIntervalNotElapsed();
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            PRIVATE HELPERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _calcFundingIndex(
        FundingRateEngine storage self,
        FundingRateSettings storage settings,
        int256 markTwap,
        int256 indexTwap
    ) private returns (int256 fundingIndex, int256 fundingRate) {
        int256 innerClamp = settings.innerClamp.toInt256();
        int256 outerClamp = settings.outerClamp.toInt256();

        int256 premium = _div(markTwap - indexTwap, indexTwap);

        int256 rawFunding = premium + (settings.interestRate - premium).clamp(-innerClamp, innerClamp);

        fundingRate = rawFunding.clamp(-outerClamp, outerClamp);

        if (fundingRate != rawFunding) self.resetIterationsLeft = settings.resetIterations;
        else if (self.resetIterationsLeft > 0) --self.resetIterationsLeft;

        fundingIndex = _mul(fundingRate, indexTwap);
    }

    // @dev wrapper for fullMulDiv to handle int256
    function _div(int256 a, int256 b) private pure returns (int256) {
        uint256 result = a.abs().fullMulDiv(1e18, b.abs());
        return a < 0 != b < 0 ? -result.toInt256() : result.toInt256();
    }

    function _mul(int256 a, int256 b) private pure returns (int256) {
        uint256 result = a.abs().fullMulDiv(b.abs(), 1e18);
        return a < 0 != b < 0 ? -result.toInt256() : result.toInt256();
    }
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

