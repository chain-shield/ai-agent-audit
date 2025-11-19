
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {OwnableRoles} from "@solady/auth/OwnableRoles.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {DynamicArrayLib} from "@solady/utils/DynamicArrayLib.sol";
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";
import {EnumerableSetLib} from "@solady/utils/EnumerableSetLib.sol";
import {Initializable} from "@solady/utils/Initializable.sol";
import {SignatureCheckerLib} from "@solady/utils/SignatureCheckerLib.sol";

import {Status, Side, TradeType, FeeTier, BookType} from "../types/Enums.sol";
import {
    MarketParams,
    PlaceOrderResult,
    PositionUpdateResult,
    LiquidateData,
    BackstopLiquidateData,
    TradeExecutedData,
    DeleveragePair,
    LiquidatorData,
    Condition,
    OIDelta,
    SignData,
    FundingPaymentResult,
    LiquidateeSettleData,
    Account
} from "../types/Structs.sol";
import {Constants} from "../types/Constants.sol";

import {BackstopLiquidatorDataLib} from "../types/BackstopLiquidatorDataLib.sol";
import {StorageLib} from "../types/StorageLib.sol";
import {CLOBLib} from "../types/CLOBLib.sol";
import {ClearingHouse, ClearingHouseLib} from "../types/ClearingHouse.sol";
import {Market, MarketSettings} from "../types/Market.sol";
import {FundingRateSettings} from "../types/FundingRateEngine.sol";
import {Position} from "../types/Position.sol";
import {Book, BookSettings} from "../types/Book.sol";

contract LiquidatorPanel is OwnableRoles {
    using FixedPointMathLib for *;
    using SafeCastLib for *;
    using DynamicArrayLib for *;

    error MarketNotDelisted();
    error ProtocolInactive();
    error InvalidDeleveragePair();
    error InvalidLiquidation();
    error InvalidBackstopLiquidation();

    enum LiquidationType {
        LIQUIDATEE,
        BACKSTOP_LIQUIDATEE,
        DELIST,
        DELEVERAGE_MAKER, // maker is underwater
        DELEVERAGE_TAKER
    }

    event Liquidation( // negative is bad debt
        bytes32 indexed asset,
        address indexed account,
        uint256 indexed subaccount,
        int256 baseDelta,
        int256 quoteDelta,
        int256 rpnl,
        int256 margin,
        int256 fee,
        LiquidationType liquidationType,
        uint256 nonce
    );

    struct __LiquidateParams__ {
        address account;
        uint256 subaccount;
        bytes32 asset;
        Position position;
        Side side;
        BookType bookType;
    }

    struct __InternalLiquidateCache__ {
        DynamicArrayLib.DynamicArray assets;
        Position[] positions;
        int256 margin;
        uint256 positionIdx;
        Side side;
        PlaceOrderResult fillResult;
        PositionUpdateResult positionResult;
        uint256 maintenanceOrProratedMargin;
    }

    struct __DelistCache__ {
        DynamicArrayLib.DynamicArray assets;
        Position[] positions;
        int256 fundingPayment;
        Side side;
        uint256 baseTraded;
        uint256 quoteTraded;
        PositionUpdateResult positionResult;
        int256 margin;
        int256 fee;
    }

    struct __DeleveragePairCache__ {
        DynamicArrayLib.DynamicArray makerAssets;
        DynamicArrayLib.DynamicArray takerAssets;
        Position[] makerPositions;
        Position[] takerPositions;
        int256 makerFundingPayment;
        int256 takerFundingPayment;
        int256 makerMargin;
        int256 takerMargin;
        uint256 baseAmount;
        uint256 quoteAmount;
    }

    struct __DeleverageValidationParams__ {
        bytes32 asset;
        DynamicArrayLib.DynamicArray makerAssets;
        DynamicArrayLib.DynamicArray takerAssets;
        Position[] makerPositions;
        Position[] takerPositions;
        uint256 makerPositionIdx;
        uint256 takerPositionIdx;
        int256 makerMargin;
        int256 takerMargin;
    }

    struct __DeleverageParams__ {
        address account;
        uint256 subaccount;
        bytes32 asset;
        DynamicArrayLib.DynamicArray assets;
        Position[] positions;
        uint256 positionIdx;
        uint256 baseTraded;
        uint256 quoteTraded;
        int256 margin;
        LiquidationType deleverageType;
    }

    modifier onlyLiquidator() {
        _checkRolesOrOwner(Constants.ADMIN_ROLE | Constants.LIQUIDATOR_ROLE);
        _;
    }

    modifier onlyBackstopLiquidator() {
        _checkRolesOrOwner(Constants.ADMIN_ROLE | Constants.BACKSTOP_LIQUIDATOR_ROLE);
        _;
    }

    modifier onlyActiveProtocol() virtual {
        if (!StorageLib.loadClearingHouse().active) revert ProtocolInactive();
        _;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                              LIQUIDATIONS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function liquidate(bytes32 asset, address account, uint256 subaccount) external onlyLiquidator onlyActiveProtocol {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        __InternalLiquidateCache__ memory cache = _liquidate({
            clearingHouse: clearingHouse,
            asset: asset,
            account: account,
            subaccount: subaccount,
            bookType: BookType.STANDARD
        });

        int256 fee = StorageLib.loadMarketSettings(asset).liquidationFeeRate.fullMulDiv(
            cache.fillResult.quoteTraded, 1e18
        ).toInt256();

        // settle rpnl and fee on margin
        cache.margin += cache.positionResult.rpnl - fee;

        (cache.margin, cache.positionResult.marginDelta) = clearingHouse.rebalanceClose({
            assets: cache.assets,
            positions: cache.positions,
            margin: cache.margin,
            marginDelta: cache.positionResult.marginDelta
        });

        bool fullClose = cache.positions[cache.positionIdx].amount == 0 && cache.positions.length == 1;

        // account close above water
        if (fullClose && cache.positionResult.marginDelta < 0) {
            // below maintenance margin
            if (cache.positionResult.marginDelta.abs() < cache.maintenanceOrProratedMargin) {
                fee -= cache.positionResult.marginDelta;
                delete cache.positionResult.marginDelta;
            }
            // account close under water
        } else if (fullClose && cache.margin < 0) {
            fee += cache.margin;

            delete cache.margin;
        }

        _emitLiquidationEvent({
            asset: asset,
            account: account,
            subaccount: subaccount,
            side: cache.side,
            quoteTraded: cache.fillResult.quoteTraded,
            baseTraded: cache.fillResult.baseTraded,
            rpnl: cache.positionResult.rpnl,
            margin: cache.margin,
            fee: fee,
            liquidationType: LiquidationType.LIQUIDATEE
        });

        if (fee > 0) StorageLib.loadInsuranceFund().pay(fee.abs());
        else StorageLib.loadInsuranceFund().claim(fee.abs());

        StorageLib.loadCollateralManager().settleFill(
            account, subaccount, cache.margin, cache.positionResult.marginDelta
        );

        clearingHouse.updateAccount({
            account: account,
            subaccount: subaccount,
            assets: cache.assets,
            positions: cache.positions,
            tradedAsset: asset,
            positionIdx: cache.positionIdx,
            oiDelta: cache.positionResult.oiDelta,
            sideClose: cache.positionResult.sideClose
        });
    }

    function backstopLiquidate(bytes32 asset, address account, uint256 subaccount)
        external
        onlyBackstopLiquidator
        onlyActiveProtocol
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        __InternalLiquidateCache__ memory cache = _liquidate({
            clearingHouse: clearingHouse,
            asset: asset,
            account: account,
            subaccount: subaccount,
            bookType: BookType.BACKSTOP
        });

        int256 proratedMargin = cache.maintenanceOrProratedMargin.toInt256();

        cache.margin -= proratedMargin;

        proratedMargin += cache.positionResult.rpnl;

        if (proratedMargin < 0) {
            cache.margin += proratedMargin;
            delete proratedMargin;
        }

        int256 fee = _settleBackstopLiquidation(clearingHouse, asset, proratedMargin.toUint256()).toInt256();

        // realize bad debt if underwater and full close
        if (cache.margin < 0 && cache.positions.length == 1) {
            fee += cache.margin;
            delete cache.margin;
        }

        _emitLiquidationEvent({
            asset: asset,
            account: account,
            subaccount: subaccount,
            side: cache.side,
            quoteTraded: cache.fillResult.quoteTraded,
            baseTraded: cache.fillResult.baseTraded,
            rpnl: cache.positionResult.rpnl,
            margin: cache.margin,
            fee: fee,
            liquidationType: LiquidationType.BACKSTOP_LIQUIDATEE
        });

        if (fee > 0) StorageLib.loadInsuranceFund().pay(fee.abs());
        else StorageLib.loadInsuranceFund().claim(fee.abs());

        StorageLib.loadCollateralManager().settleFill(account, subaccount, cache.margin, 0);

        clearingHouse.updateAccount({
            account: account,
            subaccount: subaccount,
            assets: cache.assets,
            positions: cache.positions,
            tradedAsset: asset,
            positionIdx: cache.positionIdx,
            oiDelta: cache.positionResult.oiDelta,
            sideClose: cache.positionResult.sideClose
        });
    }

    function deleverage(bytes32 asset, DeleveragePair[] calldata pairs) external onlyLiquidator onlyActiveProtocol {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        for (uint256 i; i < pairs.length; ++i) {
            _deleveragePair(clearingHouse, asset, pairs[i]);
        }
    }

    function delistClose(bytes32 asset, Account[] calldata accounts) external onlyLiquidator onlyActiveProtocol {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();
        Market storage market = clearingHouse.market[asset];

        if (StorageLib.loadMarketSettings(asset).status != Status.DELISTED) revert MarketNotDelisted();

        for (uint256 i; i < accounts.length; ++i) {
            _delistClose(clearingHouse, market, asset, accounts[i]);
        }
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                HELPERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _delistClose(
        ClearingHouse storage clearingHouse,
        Market storage market,
        bytes32 asset,
        Account calldata account
    ) internal {
        __DelistCache__ memory cache;

        (cache.assets, cache.positions, cache.margin) =
            clearingHouse.getAccountAndMargin(account.account, account.subaccount);

        uint256 positionIdx = cache.assets.indexOf(asset);

        if (positionIdx == type(uint256).max) return;

        cache.margin -= ClearingHouseLib.realizeFundingPayment(cache.assets, cache.positions);

        cache.side = cache.positions[positionIdx].isLong ? Side.SELL : Side.BUY;
        cache.baseTraded = cache.positions[positionIdx].amount;
        cache.quoteTraded = cache.baseTraded.fullMulDiv(market.markPrice, 1e18);

        cache.positionResult = cache.positions[positionIdx].processTrade({
            side: cache.side,
            quoteTraded: cache.quoteTraded,
            baseTraded: cache.baseTraded
        });

        cache.fee = StorageLib.loadFeeManager().getTakerFee(account.account, cache.quoteTraded).toInt256();

        // settle rpnl and fee on margin
        cache.margin += cache.positionResult.rpnl - cache.fee;

        (cache.margin, cache.positionResult.marginDelta) = clearingHouse.rebalanceClose({
            assets: cache.assets,
            positions: cache.positions,
            margin: cache.margin,
            marginDelta: cache.positionResult.marginDelta
        });

        // bad debt realized
        if (cache.margin < 0 && cache.positions.length == 1) {
            cache.fee += cache.margin;
            delete cache.margin;
        }

        _emitLiquidationEvent({
            asset: asset,
            account: account.account,
            subaccount: account.subaccount,
            side: cache.side,
            quoteTraded: cache.quoteTraded,
            baseTraded: cache.baseTraded,
            rpnl: cache.positionResult.rpnl,
            margin: cache.margin,
            fee: cache.fee,
            liquidationType: LiquidationType.DELIST
        });

        if (cache.fee > 0) StorageLib.loadInsuranceFund().pay(cache.fee.abs());
        else StorageLib.loadInsuranceFund().claim(cache.fee.abs());

        StorageLib.loadCollateralManager().settleFill(
            account.account, account.subaccount, cache.margin, cache.positionResult.marginDelta
        );

        clearingHouse.updateAccount({
            account: account.account,
            subaccount: account.subaccount,
            assets: cache.assets,
            positions: cache.positions,
            tradedAsset: asset,
            positionIdx: positionIdx,
            oiDelta: cache.positionResult.oiDelta,
            sideClose: true
        });
    }

    function _deleveragePair(ClearingHouse storage clearingHouse, bytes32 asset, DeleveragePair calldata pair)
        internal
    {
        __DeleveragePairCache__ memory cache;

        // load accounts
        (cache.makerAssets, cache.makerPositions, cache.makerMargin) =
            clearingHouse.getAccountAndMargin(pair.maker.account, pair.maker.subaccount);
        (cache.takerAssets, cache.takerPositions, cache.takerMargin) =
            clearingHouse.getAccountAndMargin(pair.taker.account, pair.taker.subaccount);

        // realize funding payments
        cache.makerMargin -= ClearingHouseLib.realizeFundingPayment(cache.makerAssets, cache.makerPositions);
        cache.takerMargin -= ClearingHouseLib.realizeFundingPayment(cache.takerAssets, cache.takerPositions);

        uint256 makerPositionIdx = cache.makerAssets.indexOf(asset);
        uint256 takerPositionIdx = cache.takerAssets.indexOf(asset);

        // validate
        _validateDeleveragePair(
            clearingHouse,
            __DeleverageValidationParams__({
                asset: asset,
                makerAssets: cache.makerAssets,
                takerAssets: cache.takerAssets,
                makerPositions: cache.makerPositions,
                takerPositions: cache.takerPositions,
                makerPositionIdx: makerPositionIdx,
                takerPositionIdx: takerPositionIdx,
                makerMargin: cache.makerMargin,
                takerMargin: cache.takerMargin
            })
        );

        cache.baseAmount =
            cache.makerPositions[makerPositionIdx].amount.min(cache.takerPositions[takerPositionIdx].amount);

        // ADL trades at bankruptcy price of maker
        uint256 bankruptcyPrice = _getBankruptcyPrice({
            position: cache.makerPositions[makerPositionIdx],
            closeSize: cache.baseAmount,
            proratedMargin: clearingHouse.getProratedMargin(
                cache.makerAssets, cache.makerPositions, asset, cache.makerMargin
            )
        });

        cache.quoteAmount = cache.baseAmount.fullMulDiv(bankruptcyPrice, 1e18);

        _deleverage(
            clearingHouse,
            __DeleverageParams__({
                account: pair.maker.account,
                subaccount: pair.maker.subaccount,
                asset: asset,
                assets: cache.makerAssets,
                positions: cache.makerPositions,
                positionIdx: makerPositionIdx,
                baseTraded: cache.baseAmount,
                quoteTraded: cache.quoteAmount,
                margin: cache.makerMargin,
                deleverageType: LiquidationType.DELEVERAGE_MAKER
            })
        );

        _deleverage(
            clearingHouse,
            __DeleverageParams__({
                account: pair.taker.account,
                subaccount: pair.taker.subaccount,
                asset: asset,
                assets: cache.takerAssets,
                positions: cache.takerPositions,
                positionIdx: takerPositionIdx,
                baseTraded: cache.baseAmount,
                quoteTraded: cache.quoteAmount,
                margin: cache.takerMargin,
                deleverageType: LiquidationType.DELEVERAGE_TAKER
            })
        );
    }

    function _deleverage(ClearingHouse storage clearingHouse, __DeleverageParams__ memory params) internal {
        Side side = params.positions[params.positionIdx].isLong ? Side.SELL : Side.BUY;

        PositionUpdateResult memory result = params.positions[params.positionIdx].processTrade({
            side: side,
            quoteTraded: params.quoteTraded,
            baseTraded: params.baseTraded
        });

        // settle rpnl on margin
        params.margin += result.rpnl;

        (params.margin, result.marginDelta) = clearingHouse.rebalanceClose({
            assets: params.assets,
            positions: params.positions,
            margin: params.margin,
            marginDelta: 0
        });

        bool fullClose = params.positions[params.positionIdx].amount == 0 && params.positions.length == 1;

        // full close, underwater
        // outside of dust bad debt due to rounding error, this can only happen when the loss on a maker short is greater than openNotional
        // or when the bankruptcy price puts the taker into bad debt
        // in practice, neither of these should ever occur
        uint256 badDebt;
        if (fullClose && params.margin < 0) {
            badDebt = params.margin.abs();
            StorageLib.loadInsuranceFund().claim(badDebt);
            delete params.margin;
        }

        _emitLiquidationEvent({
            asset: params.asset,
            account: params.account,
            subaccount: params.subaccount,
            side: side,
            quoteTraded: params.quoteTraded,
            baseTraded: params.baseTraded,
            rpnl: result.rpnl,
            margin: params.margin,
            fee: -badDebt.toInt256(),
            liquidationType: params.deleverageType
        });

        StorageLib.loadCollateralManager().settleFill(
            params.account, params.subaccount, params.margin, result.marginDelta
        );

        clearingHouse.updateAccount({
            account: params.account,
            subaccount: params.subaccount,
            assets: params.assets,
            positions: params.positions,
            tradedAsset: params.asset,
            positionIdx: params.positionIdx,
            oiDelta: result.oiDelta,
            sideClose: result.sideClose
        });
    }

    function _getBankruptcyPrice(Position memory position, uint256 closeSize, int256 proratedMargin)
        internal
        pure
        returns (uint256 bankruptcyPrice)
    {
        // prorated margin again, based on amount closed
        if (proratedMargin > 0) proratedMargin = proratedMargin.abs().fullMulDiv(closeSize, position.amount).toInt256();
        else proratedMargin = -proratedMargin.abs().fullMulDiv(closeSize, position.amount).toInt256();

        uint256 openNotional = position.openNotional.fullMulDiv(closeSize, position.amount);

        int256 numerator;
        if (position.isLong) numerator = openNotional.toInt256() - proratedMargin;
        else numerator = openNotional.toInt256() + proratedMargin;

        if (numerator < 0) return 0;

        bankruptcyPrice = numerator.toUint256().fullMulDiv(1e18, closeSize);
    }

    function _validateDeleveragePair(ClearingHouse storage clearingHouse, __DeleverageValidationParams__ memory params)
        internal
        view
    {
        // assert positions exist
        if (params.makerPositionIdx.max(params.takerPositionIdx) == type(uint256).max) revert InvalidDeleveragePair();

        // assert maker is under water
        if (!clearingHouse.hasBadDebt(params.makerAssets, params.makerPositions, params.makerMargin)) {
            revert InvalidDeleveragePair();
        }

        // assert taker meets open margin requirement
        if (!clearingHouse.isOpenMarginRequirementMet(params.takerAssets, params.takerPositions, params.takerMargin)) {
            revert InvalidDeleveragePair();
        }

        // assert side
        if (
            params.makerPositions[params.makerPositionIdx].isLong
                == params.takerPositions[params.takerPositionIdx].isLong
        ) revert InvalidDeleveragePair();
    }

    function _setupAccountAndValidateLiquidation(
        ClearingHouse storage clearingHouse,
        address account,
        uint256 subaccount,
        bytes32 asset,
        BookType bookType
    )
        internal
        view
        returns (
            DynamicArrayLib.DynamicArray memory assets,
            Position[] memory positions,
            int256 margin,
            uint256 positionIdx
        )
    {
        (assets, positions, margin) = clearingHouse.getAccountAndMargin(account, subaccount);

        positionIdx = assets.indexOf(asset);

        if (positionIdx == type(uint256).max) revert InvalidLiquidation();

        margin -= ClearingHouseLib.realizeFundingPayment(assets, positions);

        clearingHouse.assertLiquidatable(assets, positions, margin, bookType);
    }

    function _liquidate(
        ClearingHouse storage clearingHouse,
        bytes32 asset,
        address account,
        uint256 subaccount,
        BookType bookType
    ) internal returns (__InternalLiquidateCache__ memory cache) {
        // load account, realize funding on margin, & validate liquidatability
        (cache.assets, cache.positions, cache.margin, cache.positionIdx) = _setupAccountAndValidateLiquidation({
            clearingHouse: clearingHouse,
            account: account,
            subaccount: subaccount,
            asset: asset,
            bookType: bookType
        });
        cache.side = cache.positions[cache.positionIdx].isLong ? Side.SELL : Side.BUY;

        // liquidate on book
        cache.fillResult = clearingHouse.market[asset].liquidate({
            account: account,
            subaccount: subaccount,
            side: cache.side,
            amount: cache.positions[cache.positionIdx].amount,
            bookType: bookType
        });

        // calc maintenance/prorated margin
        if (bookType == BookType.BACKSTOP) {
            int256 proratedMargin = clearingHouse.getProratedMargin(cache.assets, cache.positions, asset, cache.margin);

            cache.maintenanceOrProratedMargin = proratedMargin < 0 ? 0 : proratedMargin.toUint256();

            // if prorated margin is non-zero & partial liquidation, then prorate margin again based on amount closed
            if (
                cache.maintenanceOrProratedMargin > 0
                    && cache.positions[cache.positionIdx].amount > cache.fillResult.baseTraded
            ) {
                cache.maintenanceOrProratedMargin = cache.maintenanceOrProratedMargin.fullMulDiv(
                    cache.fillResult.baseTraded, cache.positions[cache.positionIdx].amount
                );
            }
        } else {
            cache.maintenanceOrProratedMargin =
                clearingHouse.market[asset].getMaintenanceMargin(cache.positions[cache.positionIdx].amount);
        }

        // process on position
        cache.positionResult = cache.positions[cache.positionIdx].processTrade({
            side: cache.side,
            quoteTraded: cache.fillResult.quoteTraded,
            baseTraded: cache.fillResult.baseTraded
        });
    }

    function _settleBackstopLiquidation(ClearingHouse storage clearingHouse, bytes32 asset, uint256 margin)
        internal
        returns (uint256 liquidationFee)
    {
        LiquidatorData[] memory data = BackstopLiquidatorDataLib.getLiquidatorDataAndClearStorage();

        if (margin == 0) return 0;

        liquidationFee = margin.fullMulDiv(StorageLib.loadMarketSettings(asset).liquidationFeeRate, 1e18);
        margin -= liquidationFee;

        uint256[] memory points = new uint256[](data.length);

        uint256 totalPoints;
        uint256 totalVolume;
        for (uint256 i; i < data.length; ++i) {
            points[i] = clearingHouse.liquidatorPoints[data[i].liquidator];
            totalPoints += points[i];
            totalVolume += data[i].volume;
        }

        uint256 pointShare;
        uint256 volumeShare;
        uint256 rate;
        uint256 fee;
        for (uint256 i; i < data.length; ++i) {
            pointShare = points[i].fullMulDiv(1e18, totalPoints);
            volumeShare = data[i].volume.fullMulDiv(1e18, totalVolume);

            rate = (pointShare + volumeShare) / 2;
            fee = margin.fullMulDiv(rate, 1e18);

            StorageLib.loadCollateralManager().creditAccount(data[i].liquidator, fee);
        }
    }

    function _emitLiquidationEvent(
        bytes32 asset,
        address account,
        uint256 subaccount,
        Side side,
        uint256 quoteTraded,
        uint256 baseTraded,
        int256 rpnl,
        int256 margin,
        int256 fee,
        LiquidationType liquidationType
    ) internal {
        int256 quoteDelta = side == Side.BUY ? -quoteTraded.toInt256() : quoteTraded.toInt256();
        int256 baseDelta = side == Side.BUY ? baseTraded.toInt256() : -baseTraded.toInt256();

        emit Liquidation({
            asset: asset,
            account: account,
            subaccount: subaccount,
            baseDelta: baseDelta,
            quoteDelta: quoteDelta,
            rpnl: rpnl,
            margin: margin,
            fee: fee,
            liquidationType: liquidationType,
            nonce: StorageLib.incNonce()
        });
    }

    function _balanceBadDebt(uint256 fee, uint256 badDebt)
        internal
        pure
        returns (uint256, /*fee*/ uint256 /*badDebt*/ )
    {
        if (fee > badDebt) return (fee - badDebt, 0);
        else return (0, badDebt - fee);
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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
pragma solidity ^0.8.4;

/// @notice Safe integer casting library that reverts on overflow.
/// @author Solady (https://github.com/vectorized/solady/blob/main/src/utils/SafeCastLib.sol)
/// @author Modified from OpenZeppelin (https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/math/SafeCast.sol)
/// @dev Optimized for runtime gas for very high number of optimizer runs (i.e. >= 1000000).
library SafeCastLib {
    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                       CUSTOM ERRORS                        */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    error Overflow();

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*          UNSIGNED INTEGER SAFE CASTING OPERATIONS          */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    function toUint8(uint256 x) internal pure returns (uint8) {
        if (x >= 1 << 8) _revertOverflow();
        return uint8(x);
    }

    function toUint16(uint256 x) internal pure returns (uint16) {
        if (x >= 1 << 16) _revertOverflow();
        return uint16(x);
    }

    function toUint24(uint256 x) internal pure returns (uint24) {
        if (x >= 1 << 24) _revertOverflow();
        return uint24(x);
    }

    function toUint32(uint256 x) internal pure returns (uint32) {
        if (x >= 1 << 32) _revertOverflow();
        return uint32(x);
    }

    function toUint40(uint256 x) internal pure returns (uint40) {
        if (x >= 1 << 40) _revertOverflow();
        return uint40(x);
    }

    function toUint48(uint256 x) internal pure returns (uint48) {
        if (x >= 1 << 48) _revertOverflow();
        return uint48(x);
    }

    function toUint56(uint256 x) internal pure returns (uint56) {
        if (x >= 1 << 56) _revertOverflow();
        return uint56(x);
    }

    function toUint64(uint256 x) internal pure returns (uint64) {
        if (x >= 1 << 64) _revertOverflow();
        return uint64(x);
    }

    function toUint72(uint256 x) internal pure returns (uint72) {
        if (x >= 1 << 72) _revertOverflow();
        return uint72(x);
    }

    function toUint80(uint256 x) internal pure returns (uint80) {
        if (x >= 1 << 80) _revertOverflow();
        return uint80(x);
    }

    function toUint88(uint256 x) internal pure returns (uint88) {
        if (x >= 1 << 88) _revertOverflow();
        return uint88(x);
    }

    function toUint96(uint256 x) internal pure returns (uint96) {
        if (x >= 1 << 96) _revertOverflow();
        return uint96(x);
    }

    function toUint104(uint256 x) internal pure returns (uint104) {
        if (x >= 1 << 104) _revertOverflow();
        return uint104(x);
    }

    function toUint112(uint256 x) internal pure returns (uint112) {
        if (x >= 1 << 112) _revertOverflow();
        return uint112(x);
    }

    function toUint120(uint256 x) internal pure returns (uint120) {
        if (x >= 1 << 120) _revertOverflow();
        return uint120(x);
    }

    function toUint128(uint256 x) internal pure returns (uint128) {
        if (x >= 1 << 128) _revertOverflow();
        return uint128(x);
    }

    function toUint136(uint256 x) internal pure returns (uint136) {
        if (x >= 1 << 136) _revertOverflow();
        return uint136(x);
    }

    function toUint144(uint256 x) internal pure returns (uint144) {
        if (x >= 1 << 144) _revertOverflow();
        return uint144(x);
    }

    function toUint152(uint256 x) internal pure returns (uint152) {
        if (x >= 1 << 152) _revertOverflow();
        return uint152(x);
    }

    function toUint160(uint256 x) internal pure returns (uint160) {
        if (x >= 1 << 160) _revertOverflow();
        return uint160(x);
    }

    function toUint168(uint256 x) internal pure returns (uint168) {
        if (x >= 1 << 168) _revertOverflow();
        return uint168(x);
    }

    function toUint176(uint256 x) internal pure returns (uint176) {
        if (x >= 1 << 176) _revertOverflow();
        return uint176(x);
    }

    function toUint184(uint256 x) internal pure returns (uint184) {
        if (x >= 1 << 184) _revertOverflow();
        return uint184(x);
    }

    function toUint192(uint256 x) internal pure returns (uint192) {
        if (x >= 1 << 192) _revertOverflow();
        return uint192(x);
    }

    function toUint200(uint256 x) internal pure returns (uint200) {
        if (x >= 1 << 200) _revertOverflow();
        return uint200(x);
    }

    function toUint208(uint256 x) internal pure returns (uint208) {
        if (x >= 1 << 208) _revertOverflow();
        return uint208(x);
    }

    function toUint216(uint256 x) internal pure returns (uint216) {
        if (x >= 1 << 216) _revertOverflow();
        return uint216(x);
    }

    function toUint224(uint256 x) internal pure returns (uint224) {
        if (x >= 1 << 224) _revertOverflow();
        return uint224(x);
    }

    function toUint232(uint256 x) internal pure returns (uint232) {
        if (x >= 1 << 232) _revertOverflow();
        return uint232(x);
    }

    function toUint240(uint256 x) internal pure returns (uint240) {
        if (x >= 1 << 240) _revertOverflow();
        return uint240(x);
    }

    function toUint248(uint256 x) internal pure returns (uint248) {
        if (x >= 1 << 248) _revertOverflow();
        return uint248(x);
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*           SIGNED INTEGER SAFE CASTING OPERATIONS           */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    function toInt8(int256 x) internal pure returns (int8) {
        unchecked {
            if (((1 << 7) + uint256(x)) >> 8 == uint256(0)) return int8(x);
            _revertOverflow();
        }
    }

    function toInt16(int256 x) internal pure returns (int16) {
        unchecked {
            if (((1 << 15) + uint256(x)) >> 16 == uint256(0)) return int16(x);
            _revertOverflow();
        }
    }

    function toInt24(int256 x) internal pure returns (int24) {
        unchecked {
            if (((1 << 23) + uint256(x)) >> 24 == uint256(0)) return int24(x);
            _revertOverflow();
        }
    }

    function toInt32(int256 x) internal pure returns (int32) {
        unchecked {
            if (((1 << 31) + uint256(x)) >> 32 == uint256(0)) return int32(x);
            _revertOverflow();
        }
    }

    function toInt40(int256 x) internal pure returns (int40) {
        unchecked {
            if (((1 << 39) + uint256(x)) >> 40 == uint256(0)) return int40(x);
            _revertOverflow();
        }
    }

    function toInt48(int256 x) internal pure returns (int48) {
        unchecked {
            if (((1 << 47) + uint256(x)) >> 48 == uint256(0)) return int48(x);
            _revertOverflow();
        }
    }

    function toInt56(int256 x) internal pure returns (int56) {
        unchecked {
            if (((1 << 55) + uint256(x)) >> 56 == uint256(0)) return int56(x);
            _revertOverflow();
        }
    }

    function toInt64(int256 x) internal pure returns (int64) {
        unchecked {
            if (((1 << 63) + uint256(x)) >> 64 == uint256(0)) return int64(x);
            _revertOverflow();
        }
    }

    function toInt72(int256 x) internal pure returns (int72) {
        unchecked {
            if (((1 << 71) + uint256(x)) >> 72 == uint256(0)) return int72(x);
            _revertOverflow();
        }
    }

    function toInt80(int256 x) internal pure returns (int80) {
        unchecked {
            if (((1 << 79) + uint256(x)) >> 80 == uint256(0)) return int80(x);
            _revertOverflow();
        }
    }

    function toInt88(int256 x) internal pure returns (int88) {
        unchecked {
            if (((1 << 87) + uint256(x)) >> 88 == uint256(0)) return int88(x);
            _revertOverflow();
        }
    }

    function toInt96(int256 x) internal pure returns (int96) {
        unchecked {
            if (((1 << 95) + uint256(x)) >> 96 == uint256(0)) return int96(x);
            _revertOverflow();
        }
    }

    function toInt104(int256 x) internal pure returns (int104) {
        unchecked {
            if (((1 << 103) + uint256(x)) >> 104 == uint256(0)) return int104(x);
            _revertOverflow();
        }
    }

    function toInt112(int256 x) internal pure returns (int112) {
        unchecked {
            if (((1 << 111) + uint256(x)) >> 112 == uint256(0)) return int112(x);
            _revertOverflow();
        }
    }

    function toInt120(int256 x) internal pure returns (int120) {
        unchecked {
            if (((1 << 119) + uint256(x)) >> 120 == uint256(0)) return int120(x);
            _revertOverflow();
        }
    }

    function toInt128(int256 x) internal pure returns (int128) {
        unchecked {
            if (((1 << 127) + uint256(x)) >> 128 == uint256(0)) return int128(x);
            _revertOverflow();
        }
    }

    function toInt136(int256 x) internal pure returns (int136) {
        unchecked {
            if (((1 << 135) + uint256(x)) >> 136 == uint256(0)) return int136(x);
            _revertOverflow();
        }
    }

    function toInt144(int256 x) internal pure returns (int144) {
        unchecked {
            if (((1 << 143) + uint256(x)) >> 144 == uint256(0)) return int144(x);
            _revertOverflow();
        }
    }

    function toInt152(int256 x) internal pure returns (int152) {
        unchecked {
            if (((1 << 151) + uint256(x)) >> 152 == uint256(0)) return int152(x);
            _revertOverflow();
        }
    }

    function toInt160(int256 x) internal pure returns (int160) {
        unchecked {
            if (((1 << 159) + uint256(x)) >> 160 == uint256(0)) return int160(x);
            _revertOverflow();
        }
    }

    function toInt168(int256 x) internal pure returns (int168) {
        unchecked {
            if (((1 << 167) + uint256(x)) >> 168 == uint256(0)) return int168(x);
            _revertOverflow();
        }
    }

    function toInt176(int256 x) internal pure returns (int176) {
        unchecked {
            if (((1 << 175) + uint256(x)) >> 176 == uint256(0)) return int176(x);
            _revertOverflow();
        }
    }

    function toInt184(int256 x) internal pure returns (int184) {
        unchecked {
            if (((1 << 183) + uint256(x)) >> 184 == uint256(0)) return int184(x);
            _revertOverflow();
        }
    }

    function toInt192(int256 x) internal pure returns (int192) {
        unchecked {
            if (((1 << 191) + uint256(x)) >> 192 == uint256(0)) return int192(x);
            _revertOverflow();
        }
    }

    function toInt200(int256 x) internal pure returns (int200) {
        unchecked {
            if (((1 << 199) + uint256(x)) >> 200 == uint256(0)) return int200(x);
            _revertOverflow();
        }
    }

    function toInt208(int256 x) internal pure returns (int208) {
        unchecked {
            if (((1 << 207) + uint256(x)) >> 208 == uint256(0)) return int208(x);
            _revertOverflow();
        }
    }

    function toInt216(int256 x) internal pure returns (int216) {
        unchecked {
            if (((1 << 215) + uint256(x)) >> 216 == uint256(0)) return int216(x);
            _revertOverflow();
        }
    }

    function toInt224(int256 x) internal pure returns (int224) {
        unchecked {
            if (((1 << 223) + uint256(x)) >> 224 == uint256(0)) return int224(x);
            _revertOverflow();
        }
    }

    function toInt232(int256 x) internal pure returns (int232) {
        unchecked {
            if (((1 << 231) + uint256(x)) >> 232 == uint256(0)) return int232(x);
            _revertOverflow();
        }
    }

    function toInt240(int256 x) internal pure returns (int240) {
        unchecked {
            if (((1 << 239) + uint256(x)) >> 240 == uint256(0)) return int240(x);
            _revertOverflow();
        }
    }

    function toInt248(int256 x) internal pure returns (int248) {
        unchecked {
            if (((1 << 247) + uint256(x)) >> 248 == uint256(0)) return int248(x);
            _revertOverflow();
        }
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*               OTHER SAFE CASTING OPERATIONS                */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    function toInt8(uint256 x) internal pure returns (int8) {
        if (x >= 1 << 7) _revertOverflow();
        return int8(int256(x));
    }

    function toInt16(uint256 x) internal pure returns (int16) {
        if (x >= 1 << 15) _revertOverflow();
        return int16(int256(x));
    }

    function toInt24(uint256 x) internal pure returns (int24) {
        if (x >= 1 << 23) _revertOverflow();
        return int24(int256(x));
    }

    function toInt32(uint256 x) internal pure returns (int32) {
        if (x >= 1 << 31) _revertOverflow();
        return int32(int256(x));
    }

    function toInt40(uint256 x) internal pure returns (int40) {
        if (x >= 1 << 39) _revertOverflow();
        return int40(int256(x));
    }

    function toInt48(uint256 x) internal pure returns (int48) {
        if (x >= 1 << 47) _revertOverflow();
        return int48(int256(x));
    }

    function toInt56(uint256 x) internal pure returns (int56) {
        if (x >= 1 << 55) _revertOverflow();
        return int56(int256(x));
    }

    function toInt64(uint256 x) internal pure returns (int64) {
        if (x >= 1 << 63) _revertOverflow();
        return int64(int256(x));
    }

    function toInt72(uint256 x) internal pure returns (int72) {
        if (x >= 1 << 71) _revertOverflow();
        return int72(int256(x));
    }

    function toInt80(uint256 x) internal pure returns (int80) {
        if (x >= 1 << 79) _revertOverflow();
        return int80(int256(x));
    }

    function toInt88(uint256 x) internal pure returns (int88) {
        if (x >= 1 << 87) _revertOverflow();
        return int88(int256(x));
    }

    function toInt96(uint256 x) internal pure returns (int96) {
        if (x >= 1 << 95) _revertOverflow();
        return int96(int256(x));
    }

    function toInt104(uint256 x) internal pure returns (int104) {
        if (x >= 1 << 103) _revertOverflow();
        return int104(int256(x));
    }

    function toInt112(uint256 x) internal pure returns (int112) {
        if (x >= 1 << 111) _revertOverflow();
        return int112(int256(x));
    }

    function toInt120(uint256 x) internal pure returns (int120) {
        if (x >= 1 << 119) _revertOverflow();
        return int120(int256(x));
    }

    function toInt128(uint256 x) internal pure returns (int128) {
        if (x >= 1 << 127) _revertOverflow();
        return int128(int256(x));
    }

    function toInt136(uint256 x) internal pure returns (int136) {
        if (x >= 1 << 135) _revertOverflow();
        return int136(int256(x));
    }

    function toInt144(uint256 x) internal pure returns (int144) {
        if (x >= 1 << 143) _revertOverflow();
        return int144(int256(x));
    }

    function toInt152(uint256 x) internal pure returns (int152) {
        if (x >= 1 << 151) _revertOverflow();
        return int152(int256(x));
    }

    function toInt160(uint256 x) internal pure returns (int160) {
        if (x >= 1 << 159) _revertOverflow();
        return int160(int256(x));
    }

    function toInt168(uint256 x) internal pure returns (int168) {
        if (x >= 1 << 167) _revertOverflow();
        return int168(int256(x));
    }

    function toInt176(uint256 x) internal pure returns (int176) {
        if (x >= 1 << 175) _revertOverflow();
        return int176(int256(x));
    }

    function toInt184(uint256 x) internal pure returns (int184) {
        if (x >= 1 << 183) _revertOverflow();
        return int184(int256(x));
    }

    function toInt192(uint256 x) internal pure returns (int192) {
        if (x >= 1 << 191) _revertOverflow();
        return int192(int256(x));
    }

    function toInt200(uint256 x) internal pure returns (int200) {
        if (x >= 1 << 199) _revertOverflow();
        return int200(int256(x));
    }

    function toInt208(uint256 x) internal pure returns (int208) {
        if (x >= 1 << 207) _revertOverflow();
        return int208(int256(x));
    }

    function toInt216(uint256 x) internal pure returns (int216) {
        if (x >= 1 << 215) _revertOverflow();
        return int216(int256(x));
    }

    function toInt224(uint256 x) internal pure returns (int224) {
        if (x >= 1 << 223) _revertOverflow();
        return int224(int256(x));
    }

    function toInt232(uint256 x) internal pure returns (int232) {
        if (x >= 1 << 231) _revertOverflow();
        return int232(int256(x));
    }

    function toInt240(uint256 x) internal pure returns (int240) {
        if (x >= 1 << 239) _revertOverflow();
        return int240(int256(x));
    }

    function toInt248(uint256 x) internal pure returns (int248) {
        if (x >= 1 << 247) _revertOverflow();
        return int248(int256(x));
    }

    function toInt256(uint256 x) internal pure returns (int256) {
        if (int256(x) >= 0) return int256(x);
        _revertOverflow();
    }

    function toUint256(int256 x) internal pure returns (uint256) {
        if (x >= 0) return uint256(x);
        _revertOverflow();
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                      PRIVATE HELPERS                       */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    function _revertOverflow() private pure {
        /// @solidity memory-safe-assembly
        assembly {
            // Store the function selector of `Overflow()`.
            mstore(0x00, 0x35278d12)
            // Revert with (offset, size).
            revert(0x1c, 0x04)
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

// This file is auto-generated.

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                          STRUCTS                           */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

/// @dev Type to represent a dynamic array in memory.
/// You can directly assign to `data`, and the `p` function will
/// take care of the memory allocation.
struct DynamicArray {
    uint256[] data;
}

using DynamicArrayLib for DynamicArray global;

/// @notice Library for memory arrays with automatic capacity resizing.
/// @author Solady (https://github.com/vectorized/solady/blob/main/src/utils/g/DynamicArrayLib.sol)
library DynamicArrayLib {
    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                         CONSTANTS                          */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The constant returned when the element is not found in the array.
    uint256 internal constant NOT_FOUND = type(uint256).max;

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                  UINT256 ARRAY OPERATIONS                  */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    // Low level minimalist uint256 array operations.
    // If you don't need syntax sugar, it's recommended to use these.
    // Some of these functions returns the same array for function chaining.
    // `e.g. `array.set(0, 1).set(1, 2)`.

    /// @dev Returns a uint256 array with `n` elements. The elements are not zeroized.
    function malloc(uint256 n) internal pure returns (uint256[] memory result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := or(sub(0, shr(32, n)), mload(0x40))
            mstore(result, n)
            mstore(0x40, add(add(result, 0x20), shl(5, n)))
        }
    }

    /// @dev Zeroizes all the elements of `a`.
    function zeroize(uint256[] memory a) internal pure returns (uint256[] memory result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := a
            codecopy(add(result, 0x20), codesize(), shl(5, mload(result)))
        }
    }

    /// @dev Returns the element at `a[i]`, without bounds checking.
    function get(uint256[] memory a, uint256 i) internal pure returns (uint256 result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(add(add(a, 0x20), shl(5, i)))
        }
    }

    /// @dev Returns the element at `a[i]`, without bounds checking.
    function getUint256(uint256[] memory a, uint256 i) internal pure returns (uint256 result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(add(add(a, 0x20), shl(5, i)))
        }
    }

    /// @dev Returns the element at `a[i]`, without bounds checking.
    function getAddress(uint256[] memory a, uint256 i) internal pure returns (address result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(add(add(a, 0x20), shl(5, i)))
        }
    }

    /// @dev Returns the element at `a[i]`, without bounds checking.
    function getBool(uint256[] memory a, uint256 i) internal pure returns (bool result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(add(add(a, 0x20), shl(5, i)))
        }
    }

    /// @dev Returns the element at `a[i]`, without bounds checking.
    function getBytes32(uint256[] memory a, uint256 i) internal pure returns (bytes32 result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(add(add(a, 0x20), shl(5, i)))
        }
    }

    /// @dev Sets `a.data[i]` to `data`, without bounds checking.
    function set(uint256[] memory a, uint256 i, uint256 data)
        internal
        pure
        returns (uint256[] memory result)
    {
        result = a;
        /// @solidity memory-safe-assembly
        assembly {
            mstore(add(add(result, 0x20), shl(5, i)), data)
        }
    }

    /// @dev Sets `a.data[i]` to `data`, without bounds checking.
    function set(uint256[] memory a, uint256 i, address data)
        internal
        pure
        returns (uint256[] memory result)
    {
        result = a;
        /// @solidity memory-safe-assembly
        assembly {
            mstore(add(add(result, 0x20), shl(5, i)), shr(96, shl(96, data)))
        }
    }

    /// @dev Sets `a.data[i]` to `data`, without bounds checking.
    function set(uint256[] memory a, uint256 i, bool data)
        internal
        pure
        returns (uint256[] memory result)
    {
        result = a;
        /// @solidity memory-safe-assembly
        assembly {
            mstore(add(add(result, 0x20), shl(5, i)), iszero(iszero(data)))
        }
    }

    /// @dev Sets `a.data[i]` to `data`, without bounds checking.
    function set(uint256[] memory a, uint256 i, bytes32 data)
        internal
        pure
        returns (uint256[] memory result)
    {
        result = a;
        /// @solidity memory-safe-assembly
        assembly {
            mstore(add(add(result, 0x20), shl(5, i)), data)
        }
    }

    /// @dev Casts `a` to `address[]`.
    function asAddressArray(uint256[] memory a) internal pure returns (address[] memory result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := a
        }
    }

    /// @dev Casts `a` to `bool[]`.
    function asBoolArray(uint256[] memory a) internal pure returns (bool[] memory result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := a
        }
    }

    /// @dev Casts `a` to `bytes32[]`.
    function asBytes32Array(uint256[] memory a) internal pure returns (bytes32[] memory result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := a
        }
    }

    /// @dev Casts `a` to `uint256[]`.
    function toUint256Array(address[] memory a) internal pure returns (uint256[] memory result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := a
        }
    }

    /// @dev Casts `a` to `uint256[]`.
    function toUint256Array(bool[] memory a) internal pure returns (uint256[] memory result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := a
        }
    }

    /// @dev Casts `a` to `uint256[]`.
    function toUint256Array(bytes32[] memory a) internal pure returns (uint256[] memory result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := a
        }
    }

    /// @dev Reduces the size of `a` to `n`.
    /// If `n` is greater than the size of `a`, this will be a no-op.
    function truncate(uint256[] memory a, uint256 n)
        internal
        pure
        returns (uint256[] memory result)
    {
        /// @solidity memory-safe-assembly
        assembly {
            result := a
            mstore(mul(lt(n, mload(result)), result), n)
        }
    }

    /// @dev Clears the array and attempts to free the memory if possible.
    function free(uint256[] memory a) internal pure returns (uint256[] memory result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := a
            let n := mload(result)
            mstore(shl(6, lt(iszero(n), eq(add(shl(5, add(1, n)), result), mload(0x40)))), result)
            mstore(result, 0)
        }
    }

    /// @dev Equivalent to `keccak256(abi.encodePacked(a))`.
    function hash(uint256[] memory a) internal pure returns (bytes32 result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := keccak256(add(a, 0x20), shl(5, mload(a)))
        }
    }

    /// @dev Returns a copy of `a` sliced from `start` to `end` (exclusive).
    function slice(uint256[] memory a, uint256 start, uint256 end)
        internal
        pure
        returns (uint256[] memory result)
    {
        /// @solidity memory-safe-assembly
        assembly {
            let arrayLen := mload(a)
            if iszero(gt(arrayLen, end)) { end := arrayLen }
            if iszero(gt(arrayLen, start)) { start := arrayLen }
            if lt(start, end) {
                result := mload(0x40)
                let resultLen := sub(end, start)
                mstore(result, resultLen)
                a := add(a, shl(5, start))
                // Copy the `a` one word at a time, backwards.
                let o := shl(5, resultLen)
                mstore(0x40, add(add(result, o), 0x20)) // Allocate memory.
                for {} 1 {} {
                    mstore(add(result, o), mload(add(a, o)))
                    o := sub(o, 0x20)
                    if iszero(o) { break }
                }
            }
        }
    }

    /// @dev Returns if `needle` is in `a`.
    function contains(uint256[] memory a, uint256 needle) internal pure returns (bool) {
        return ~indexOf(a, needle, 0) != 0;
    }

    /// @dev Returns the first index of `needle`, scanning forward from `from`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function indexOf(uint256[] memory a, uint256 needle, uint256 from)
        internal
        pure
        returns (uint256 result)
    {
        /// @solidity memory-safe-assembly
        assembly {
            result := not(0)
            if lt(from, mload(a)) {
                let o := add(a, shl(5, from))
                let end := add(shl(5, add(1, mload(a))), a)
                let c := mload(end) // Cache the word after the array.
                for { mstore(end, needle) } 1 {} {
                    o := add(o, 0x20)
                    if eq(mload(o), needle) { break }
                }
                mstore(end, c) // Restore the word after the array.
                if iszero(eq(o, end)) { result := shr(5, sub(o, add(0x20, a))) }
            }
        }
    }

    /// @dev Returns the first index of `needle`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function indexOf(uint256[] memory a, uint256 needle) internal pure returns (uint256 result) {
        result = indexOf(a, needle, 0);
    }

    /// @dev Returns the last index of `needle`, scanning backwards from `from`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function lastIndexOf(uint256[] memory a, uint256 needle, uint256 from)
        internal
        pure
        returns (uint256 result)
    {
        /// @solidity memory-safe-assembly
        assembly {
            result := not(0)
            let n := mload(a)
            if n {
                if iszero(lt(from, n)) { from := sub(n, 1) }
                let o := add(shl(5, add(2, from)), a)
                for { mstore(a, needle) } 1 {} {
                    o := sub(o, 0x20)
                    if eq(mload(o), needle) { break }
                }
                mstore(a, n) // Restore the length.
                if iszero(eq(o, a)) { result := shr(5, sub(o, add(0x20, a))) }
            }
        }
    }

    /// @dev Returns the first index of `needle`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function lastIndexOf(uint256[] memory a, uint256 needle)
        internal
        pure
        returns (uint256 result)
    {
        result = lastIndexOf(a, needle, NOT_FOUND);
    }

    /// @dev Directly returns `a` without copying.
    function directReturn(uint256[] memory a) internal pure {
        assembly {
            let retStart := sub(a, 0x20)
            mstore(retStart, 0x20)
            return(retStart, add(0x40, shl(5, mload(a))))
        }
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                  DYNAMIC ARRAY OPERATIONS                  */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    // Some of these functions returns the same array for function chaining.
    // `e.g. `a.p("1").p("2")`.

    /// @dev Shorthand for `a.data.length`.
    function length(DynamicArray memory a) internal pure returns (uint256) {
        return a.data.length;
    }

    /// @dev Wraps `a` in a dynamic array struct.
    function wrap(uint256[] memory a) internal pure returns (DynamicArray memory result) {
        result.data = a;
    }

    /// @dev Wraps `a` in a dynamic array struct.
    function wrap(address[] memory a) internal pure returns (DynamicArray memory result) {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(result, a)
        }
    }

    /// @dev Wraps `a` in a dynamic array struct.
    function wrap(bool[] memory a) internal pure returns (DynamicArray memory result) {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(result, a)
        }
    }

    /// @dev Wraps `a` in a dynamic array struct.
    function wrap(bytes32[] memory a) internal pure returns (DynamicArray memory result) {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(result, a)
        }
    }

    /// @dev Clears the array without deallocating the memory.
    function clear(DynamicArray memory a) internal pure returns (DynamicArray memory result) {
        _deallocate(result);
        result = a;
        /// @solidity memory-safe-assembly
        assembly {
            mstore(mload(result), 0)
        }
    }

    /// @dev Clears the array and attempts to free the memory if possible.
    function free(DynamicArray memory a) internal pure returns (DynamicArray memory result) {
        _deallocate(result);
        result = a;
        /// @solidity memory-safe-assembly
        assembly {
            let arrData := mload(result)
            if iszero(eq(arrData, 0x60)) {
                let prime := 8188386068317523
                let cap := mload(sub(arrData, 0x20))
                // Extract `cap`, initializing it to zero if it is not a multiple of `prime`.
                cap := mul(div(cap, prime), iszero(mod(cap, prime)))
                // If `cap` is non-zero and the memory is contiguous, we can free it.
                if lt(iszero(cap), eq(mload(0x40), add(arrData, add(0x20, cap)))) {
                    mstore(0x40, sub(arrData, 0x20))
                }
                mstore(result, 0x60)
            }
        }
    }

    /// @dev Resizes the array to contain `n` elements. New elements will be zeroized.
    function resize(DynamicArray memory a, uint256 n)
        internal
        pure
        returns (DynamicArray memory result)
    {
        _deallocate(result);
        result = a;
        reserve(result, n);
        /// @solidity memory-safe-assembly
        assembly {
            let arrData := mload(result)
            let arrLen := mload(arrData)
            if iszero(lt(n, arrLen)) {
                codecopy(add(arrData, shl(5, add(1, arrLen))), codesize(), shl(5, sub(n, arrLen)))
            }
            mstore(arrData, n)
        }
    }

    /// @dev Increases the size of `a` to `n`.
    /// If `n` is less than the size of `a`, this will be a no-op.
    /// This method does not zeroize any newly created elements.
    function expand(DynamicArray memory a, uint256 n)
        internal
        pure
        returns (DynamicArray memory result)
    {
        _deallocate(result);
        result = a;
        if (n >= a.data.length) {
            reserve(result, n);
            /// @solidity memory-safe-assembly
            assembly {
                mstore(mload(result), n)
            }
        }
    }

    /// @dev Reduces the size of `a` to `n`.
    /// If `n` is greater than the size of `a`, this will be a no-op.
    function truncate(DynamicArray memory a, uint256 n)
        internal
        pure
        returns (DynamicArray memory result)
    {
        _deallocate(result);
        result = a;
        /// @solidity memory-safe-assembly
        assembly {
            mstore(mul(lt(n, mload(mload(result))), mload(result)), n)
        }
    }

    /// @dev Reserves at least `minimum` amount of contiguous memory.
    function reserve(DynamicArray memory a, uint256 minimum)
        internal
        pure
        returns (DynamicArray memory result)
    {
        _deallocate(result);
        result = a;
        /// @solidity memory-safe-assembly
        assembly {
            if iszero(lt(minimum, 0xffffffff)) { invalid() } // For extra safety.
            for { let arrData := mload(a) } 1 {} {
                // Some random prime number to multiply `cap`, so that
                // we know that the `cap` is for a dynamic array.
                // Selected to be larger than any memory pointer realistically.
                let prime := 8188386068317523
                // Special case for `arrData` pointing to zero pointer.
                if eq(arrData, 0x60) {
                    let newCap := shl(5, add(1, minimum))
                    let capSlot := mload(0x40)
                    mstore(capSlot, mul(prime, newCap)) // Store the capacity.
                    let newArrData := add(0x20, capSlot)
                    mstore(newArrData, 0) // Store the length.
                    mstore(0x40, add(newArrData, add(0x20, newCap))) // Allocate memory.
                    mstore(a, newArrData)
                    break
                }
                let w := not(0x1f)
                let cap := mload(add(arrData, w)) // `mload(sub(arrData, w))`.
                // Extract `cap`, initializing it to zero if it is not a multiple of `prime`.
                cap := mul(div(cap, prime), iszero(mod(cap, prime)))
                let newCap := shl(5, minimum)
                // If we don't need to grow the memory.
                if iszero(and(gt(minimum, mload(arrData)), gt(newCap, cap))) { break }
                // If the memory is contiguous, we can simply expand it.
                if eq(mload(0x40), add(arrData, add(0x20, cap))) {
                    mstore(add(arrData, w), mul(prime, newCap)) // Store the capacity.
                    mstore(0x40, add(arrData, add(0x20, newCap))) // Expand the memory allocation.
                    break
                }
                let capSlot := mload(0x40)
                let newArrData := add(capSlot, 0x20)
                mstore(0x40, add(newArrData, add(0x20, newCap))) // Reallocate the memory.
                mstore(a, newArrData) // Store the `newArrData`.
                // Copy `arrData` one word at a time, backwards.
                for { let o := add(0x20, shl(5, mload(arrData))) } 1 {} {
                    mstore(add(newArrData, o), mload(add(arrData, o)))
                    o := add(o, w) // `sub(o, 0x20)`.
                    if iszero(o) { break }
                }
                mstore(capSlot, mul(prime, newCap)) // Store the capacity.
                mstore(newArrData, mload(arrData)) // Store the length.
                break
            }
        }
    }

    /// @dev Appends `data` to `a`.
    function p(DynamicArray memory a, uint256 data)
        internal
        pure
        returns (DynamicArray memory result)
    {
        _deallocate(result);
        result = a;
        /// @solidity memory-safe-assembly
        assembly {
            let arrData := mload(a)
            let newArrLen := add(mload(arrData), 1)
            let newArrBytesLen := shl(5, newArrLen)
            // Some random prime number to multiply `cap`, so that
            // we know that the `cap` is for a dynamic array.
            // Selected to be larger than any memory pointer realistically.
            let prime := 8188386068317523
            let cap := mload(sub(arrData, 0x20))
            // Extract `cap`, initializing it to zero if it is not a multiple of `prime`.
            cap := mul(div(cap, prime), iszero(mod(cap, prime)))

            // Expand / Reallocate memory if required.
            // Note that we need to allocate an extra word for the length.
            for {} iszero(lt(newArrBytesLen, cap)) {} {
                // Approximately more than double the capacity to ensure more than enough space.
                let newCap := add(cap, or(cap, newArrBytesLen))
                // If the memory is contiguous, we can simply expand it.
                if iszero(or(xor(mload(0x40), add(arrData, add(0x20, cap))), eq(arrData, 0x60))) {
                    mstore(sub(arrData, 0x20), mul(prime, newCap)) // Store the capacity.
                    mstore(0x40, add(arrData, add(0x20, newCap))) // Expand the memory allocation.
                    break
                }
                // Set the `newArrData` to point to the word after `cap`.
                let newArrData := add(mload(0x40), 0x20)
                mstore(0x40, add(newArrData, add(0x20, newCap))) // Reallocate the memory.
                mstore(a, newArrData) // Store the `newArrData`.
                let w := not(0x1f)
                // Copy `arrData` one word at a time, backwards.
                for { let o := newArrBytesLen } 1 {} {
                    mstore(add(newArrData, o), mload(add(arrData, o)))
                    o := add(o, w) // `sub(o, 0x20)`.
                    if iszero(o) { break }
                }
                mstore(add(newArrData, w), mul(prime, newCap)) // Store the memory.
                arrData := newArrData // Assign `newArrData` to `arrData`.
                break
            }
            mstore(add(arrData, newArrBytesLen), data) // Append `data`.
            mstore(arrData, newArrLen) // Store the length.
        }
    }

    /// @dev Appends `data` to `a`.
    function p(DynamicArray memory a, address data)
        internal
        pure
        returns (DynamicArray memory result)
    {
        _deallocate(result);
        result = p(a, uint256(uint160(data)));
    }

    /// @dev Appends `data` to `a`.
    function p(DynamicArray memory a, bool data)
        internal
        pure
        returns (DynamicArray memory result)
    {
        _deallocate(result);
        result = p(a, _toUint(data));
    }

    /// @dev Appends `data` to `a`.
    function p(DynamicArray memory a, bytes32 data)
        internal
        pure
        returns (DynamicArray memory result)
    {
        _deallocate(result);
        result = p(a, uint256(data));
    }

    /// @dev Shorthand for returning an empty array.
    function p() internal pure returns (DynamicArray memory result) {}

    /// @dev Shorthand for `p(p(), data)`.
    function p(uint256 data) internal pure returns (DynamicArray memory result) {
        p(result, uint256(data));
    }

    /// @dev Shorthand for `p(p(), data)`.
    function p(address data) internal pure returns (DynamicArray memory result) {
        p(result, uint256(uint160(data)));
    }

    /// @dev Shorthand for `p(p(), data)`.
    function p(bool data) internal pure returns (DynamicArray memory result) {
        p(result, _toUint(data));
    }

    /// @dev Shorthand for `p(p(), data)`.
    function p(bytes32 data) internal pure returns (DynamicArray memory result) {
        p(result, uint256(data));
    }

    /// @dev Removes and returns the last element of `a`.
    /// Returns 0 and does not pop anything if the array is empty.
    function pop(DynamicArray memory a) internal pure returns (uint256 result) {
        /// @solidity memory-safe-assembly
        assembly {
            let o := mload(a)
            let n := mload(o)
            result := mload(add(o, shl(5, n)))
            mstore(o, sub(n, iszero(iszero(n))))
        }
    }

    /// @dev Removes and returns the last element of `a`.
    /// Returns 0 and does not pop anything if the array is empty.
    function popUint256(DynamicArray memory a) internal pure returns (uint256 result) {
        /// @solidity memory-safe-assembly
        assembly {
            let o := mload(a)
            let n := mload(o)
            result := mload(add(o, shl(5, n)))
            mstore(o, sub(n, iszero(iszero(n))))
        }
    }

    /// @dev Removes and returns the last element of `a`.
    /// Returns 0 and does not pop anything if the array is empty.
    function popAddress(DynamicArray memory a) internal pure returns (address result) {
        /// @solidity memory-safe-assembly
        assembly {
            let o := mload(a)
            let n := mload(o)
            result := mload(add(o, shl(5, n)))
            mstore(o, sub(n, iszero(iszero(n))))
        }
    }

    /// @dev Removes and returns the last element of `a`.
    /// Returns 0 and does not pop anything if the array is empty.
    function popBool(DynamicArray memory a) internal pure returns (bool result) {
        /// @solidity memory-safe-assembly
        assembly {
            let o := mload(a)
            let n := mload(o)
            result := mload(add(o, shl(5, n)))
            mstore(o, sub(n, iszero(iszero(n))))
        }
    }

    /// @dev Removes and returns the last element of `a`.
    /// Returns 0 and does not pop anything if the array is empty.
    function popBytes32(DynamicArray memory a) internal pure returns (bytes32 result) {
        /// @solidity memory-safe-assembly
        assembly {
            let o := mload(a)
            let n := mload(o)
            result := mload(add(o, shl(5, n)))
            mstore(o, sub(n, iszero(iszero(n))))
        }
    }

    /// @dev Returns the element at `a.data[i]`, without bounds checking.
    function get(DynamicArray memory a, uint256 i) internal pure returns (uint256 result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(add(add(mload(a), 0x20), shl(5, i)))
        }
    }

    /// @dev Returns the element at `a.data[i]`, without bounds checking.
    function getUint256(DynamicArray memory a, uint256 i) internal pure returns (uint256 result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(add(add(mload(a), 0x20), shl(5, i)))
        }
    }

    /// @dev Returns the element at `a.data[i]`, without bounds checking.
    function getAddress(DynamicArray memory a, uint256 i) internal pure returns (address result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(add(add(mload(a), 0x20), shl(5, i)))
        }
    }

    /// @dev Returns the element at `a.data[i]`, without bounds checking.
    function getBool(DynamicArray memory a, uint256 i) internal pure returns (bool result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(add(add(mload(a), 0x20), shl(5, i)))
        }
    }

    /// @dev Returns the element at `a.data[i]`, without bounds checking.
    function getBytes32(DynamicArray memory a, uint256 i) internal pure returns (bytes32 result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(add(add(mload(a), 0x20), shl(5, i)))
        }
    }

    /// @dev Sets `a.data[i]` to `data`, without bounds checking.
    function set(DynamicArray memory a, uint256 i, uint256 data)
        internal
        pure
        returns (DynamicArray memory result)
    {
        _deallocate(result);
        result = a;
        /// @solidity memory-safe-assembly
        assembly {
            mstore(add(add(mload(result), 0x20), shl(5, i)), data)
        }
    }

    /// @dev Sets `a.data[i]` to `data`, without bounds checking.
    function set(DynamicArray memory a, uint256 i, address data)
        internal
        pure
        returns (DynamicArray memory result)
    {
        _deallocate(result);
        result = a;
        /// @solidity memory-safe-assembly
        assembly {
            mstore(add(add(mload(result), 0x20), shl(5, i)), shr(96, shl(96, data)))
        }
    }

    /// @dev Sets `a.data[i]` to `data`, without bounds checking.
    function set(DynamicArray memory a, uint256 i, bool data)
        internal
        pure
        returns (DynamicArray memory result)
    {
        _deallocate(result);
        result = a;
        /// @solidity memory-safe-assembly
        assembly {
            mstore(add(add(mload(result), 0x20), shl(5, i)), iszero(iszero(data)))
        }
    }

    /// @dev Sets `a.data[i]` to `data`, without bounds checking.
    function set(DynamicArray memory a, uint256 i, bytes32 data)
        internal
        pure
        returns (DynamicArray memory result)
    {
        _deallocate(result);
        result = a;
        /// @solidity memory-safe-assembly
        assembly {
            mstore(add(add(mload(result), 0x20), shl(5, i)), data)
        }
    }

    /// @dev Returns the underlying array as a `uint256[]`.
    function asUint256Array(DynamicArray memory a)
        internal
        pure
        returns (uint256[] memory result)
    {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(a)
        }
    }

    /// @dev Returns the underlying array as a `address[]`.
    function asAddressArray(DynamicArray memory a)
        internal
        pure
        returns (address[] memory result)
    {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(a)
        }
    }

    /// @dev Returns the underlying array as a `bool[]`.
    function asBoolArray(DynamicArray memory a) internal pure returns (bool[] memory result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(a)
        }
    }

    /// @dev Returns the underlying array as a `bytes32[]`.
    function asBytes32Array(DynamicArray memory a)
        internal
        pure
        returns (bytes32[] memory result)
    {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(a)
        }
    }

    /// @dev Returns a copy of `a` sliced from `start` to `end` (exclusive).
    function slice(DynamicArray memory a, uint256 start, uint256 end)
        internal
        pure
        returns (DynamicArray memory result)
    {
        result.data = slice(a.data, start, end);
    }

    /// @dev Returns a copy of `a` sliced from `start` to the end of the array.
    function slice(DynamicArray memory a, uint256 start)
        internal
        pure
        returns (DynamicArray memory result)
    {
        result.data = slice(a.data, start, type(uint256).max);
    }

    /// @dev Returns if `needle` is in `a`.
    function contains(DynamicArray memory a, uint256 needle) internal pure returns (bool) {
        return ~indexOf(a.data, needle, 0) != 0;
    }

    /// @dev Returns if `needle` is in `a`.
    function contains(DynamicArray memory a, address needle) internal pure returns (bool) {
        return ~indexOf(a.data, uint160(needle), 0) != 0;
    }

    /// @dev Returns if `needle` is in `a`.
    function contains(DynamicArray memory a, bytes32 needle) internal pure returns (bool) {
        return ~indexOf(a.data, uint256(needle), 0) != 0;
    }

    /// @dev Returns the first index of `needle`, scanning forward from `from`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function indexOf(DynamicArray memory a, uint256 needle, uint256 from)
        internal
        pure
        returns (uint256)
    {
        return indexOf(a.data, needle, from);
    }

    /// @dev Returns the first index of `needle`, scanning forward from `from`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function indexOf(DynamicArray memory a, address needle, uint256 from)
        internal
        pure
        returns (uint256)
    {
        return indexOf(a.data, uint160(needle), from);
    }

    /// @dev Returns the first index of `needle`, scanning forward from `from`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function indexOf(DynamicArray memory a, bytes32 needle, uint256 from)
        internal
        pure
        returns (uint256)
    {
        return indexOf(a.data, uint256(needle), from);
    }

    /// @dev Returns the first index of `needle`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function indexOf(DynamicArray memory a, uint256 needle) internal pure returns (uint256) {
        return indexOf(a.data, needle, 0);
    }

    /// @dev Returns the first index of `needle`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function indexOf(DynamicArray memory a, address needle) internal pure returns (uint256) {
        return indexOf(a.data, uint160(needle), 0);
    }

    /// @dev Returns the first index of `needle`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function indexOf(DynamicArray memory a, bytes32 needle) internal pure returns (uint256) {
        return indexOf(a.data, uint256(needle), 0);
    }

    /// @dev Returns the last index of `needle`, scanning backwards from `from`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function lastIndexOf(DynamicArray memory a, uint256 needle, uint256 from)
        internal
        pure
        returns (uint256)
    {
        return lastIndexOf(a.data, needle, from);
    }

    /// @dev Returns the last index of `needle`, scanning backwards from `from`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function lastIndexOf(DynamicArray memory a, address needle, uint256 from)
        internal
        pure
        returns (uint256)
    {
        return lastIndexOf(a.data, uint160(needle), from);
    }

    /// @dev Returns the last index of `needle`, scanning backwards from `from`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function lastIndexOf(DynamicArray memory a, bytes32 needle, uint256 from)
        internal
        pure
        returns (uint256)
    {
        return lastIndexOf(a.data, uint256(needle), from);
    }

    /// @dev Returns the last index of `needle`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function lastIndexOf(DynamicArray memory a, uint256 needle) internal pure returns (uint256) {
        return lastIndexOf(a.data, needle, NOT_FOUND);
    }

    /// @dev Returns the last index of `needle`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function lastIndexOf(DynamicArray memory a, address needle) internal pure returns (uint256) {
        return lastIndexOf(a.data, uint160(needle), NOT_FOUND);
    }

    /// @dev Returns the last index of `needle`.
    /// If `needle` is not in `a`, returns `NOT_FOUND`.
    function lastIndexOf(DynamicArray memory a, bytes32 needle) internal pure returns (uint256) {
        return lastIndexOf(a.data, uint256(needle), NOT_FOUND);
    }

    /// @dev Equivalent to `keccak256(abi.encodePacked(a.data))`.
    function hash(DynamicArray memory a) internal pure returns (bytes32 result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := keccak256(add(mload(a), 0x20), shl(5, mload(mload(a))))
        }
    }

    /// @dev Directly returns `a` without copying.
    function directReturn(DynamicArray memory a) internal pure {
        assembly {
            let arrData := mload(a)
            let retStart := sub(arrData, 0x20)
            mstore(retStart, 0x20)
            return(retStart, add(0x40, shl(5, mload(arrData))))
        }
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                      PRIVATE HELPERS                       */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Helper for deallocating a automatically allocated array pointer.
    function _deallocate(DynamicArray memory result) private pure {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(0x40, result) // Deallocate, as we have already allocated.
        }
    }

    /// @dev Casts the bool into a uint256.
    function _toUint(bool b) private pure returns (uint256 result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := iszero(iszero(b))
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {ERC4626} from "@solady/tokens/ERC4626.sol";
import {EnumerableSetLib} from "@solady/utils/EnumerableSetLib.sol";
import {DynamicArrayLib} from "@solady/utils/DynamicArrayLib.sol";
import {SafeTransferLib} from "@solady/utils/SafeTransferLib.sol";
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";
import {Initializable} from "@solady/utils/Initializable.sol";
import {OwnableRoles} from "@solady/auth/OwnableRoles.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";

import {IViewPort} from "./interfaces/IViewPort.sol";
import {IOperatorPanel} from "../utils/interfaces/IOperatorPanel.sol";
import {PerpsOperatorRoles} from "../utils/OperatorPanel.sol";

contract GTL is Initializable, ERC4626, OwnableRoles {
    using DynamicArrayLib for uint256[];
    using SafeTransferLib for address;
    using EnumerableSetLib for EnumerableSetLib.Uint256Set;
    using SafeCastLib for *;
    using FixedPointMathLib for *;

    event WithdrawalQueued(uint256 indexed id, address indexed account, uint256 shares);
    event WithdrawalCanceled(uint256 indexed id);
    event WithdrawalProcessed(uint256 indexed id, address indexed account, uint256 shares, uint256 assets);
    event AdminRoleGranted(address indexed account);
    event AdminRoleRevoked(address indexed account);

    error NotPerpManager();
    error InvalidOperator();
    error InsufficientWithdrawalsQueued();
    error NotAdmin();
    error Unused();
    error InsufficientWithdrawal();

    /// @dev The abi version of this impl so the indexer can handle event-changing upgrades
    uint256 public constant ABI_VERSION = 1;

    constructor(address _usdc, address _perpManager) {
        usdc = _usdc;
        perpManager = _perpManager;
        _disableInitializers();
    }

    function initialize(address _owner) external initializer {
        usdc.safeApprove(perpManager, type(uint256).max);
        _initializeOwner(_owner);
    }

    modifier onlyPerpManager() {
        if (msg.sender != perpManager) revert NotPerpManager();
        _;
    }

    modifier onlyAdmin() {
        _assertAdmin();
        _;
    }

    struct Withdrawal {
        address account;
        uint256 shares;
    }

    address public immutable usdc;
    address public immutable perpManager;

    uint256 public constant ADMIN_ROLE = _ROLE_0;

    EnumerableSetLib.Uint256Set private _subaccounts;

    uint256[] private _withdrawalQueue;

    uint256 private _withdrawalCounter;

    mapping(uint256 id => Withdrawal) internal _queuedWithdrawal;

    mapping(address account => uint256) internal _queuedShares;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               METADATA
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function name() public pure override returns (string memory) {
        return "GTE Liquidity Pool";
    }

    function symbol() public pure override returns (string memory) {
        return "GTL";
    }

    function asset() public view override returns (address) {
        return usdc;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                  LP
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function queueWithdrawal(uint256 shares) external returns (uint256 id) {
        if (shares == 0) revert InsufficientWithdrawal();
        if (_queuedShares[msg.sender] + shares > balanceOf(msg.sender)) revert InsufficientBalance();

        id = ++_withdrawalCounter;

        _queuedShares[msg.sender] += shares;
        _queuedWithdrawal[id] = Withdrawal(msg.sender, shares);
        _withdrawalQueue.push(id);

        emit WithdrawalQueued(id, msg.sender, shares);
    }

    function cancelWithdrawal(uint256 id) external {
        if (_queuedWithdrawal[id].account != msg.sender) revert NotPerpManager();

        _queuedShares[msg.sender] -= _queuedWithdrawal[id].shares;
        delete _queuedWithdrawal[id];

        _dequeue(id);

        emit WithdrawalCanceled(id);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ADMIN
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function processWithdrawals(uint256 num) external onlyAdmin {
        if (num > _withdrawalQueue.length) revert InsufficientWithdrawalsQueued();

        uint256 allocatedAssets = orderbookCollateral() + freeCollateralBalance() + totalAccountValue();

        uint256 id;
        Withdrawal memory withdrawal;
        uint256 assets;
        for (uint256 i; i < num; ++i) {
            id = _withdrawalQueue[i];
            withdrawal = _queuedWithdrawal[id];

            assets = _convertToAssets({shares: withdrawal.shares, allocatedAssets: allocatedAssets});

            delete  _queuedWithdrawal[id];

            _queuedShares[withdrawal.account] -= withdrawal.shares;

            _burn(withdrawal.account, withdrawal.shares);

            usdc.safeTransfer(withdrawal.account, assets);

            emit WithdrawalProcessed(id, withdrawal.account, withdrawal.shares, assets);
        }

        _dequeueBatch(num);
    }

    function grantAdminRole(address account) external onlyOwner {
        _grantRoles(account, ADMIN_ROLE);
        emit AdminRoleGranted(account);
    }

    function revokeAdminRole(address account) external onlyOwner {
        _removeRoles(account, ADMIN_ROLE);
        emit AdminRoleRevoked(account);
    }

    function approveOperator(address operator) external onlyOwner {
        if (!hasAllRoles(operator, ADMIN_ROLE)) revert InvalidOperator();

        IOperatorPanel(perpManager).approveOperator({
            account: address(this),
            operator: operator,
            roles: 1 << uint256(PerpsOperatorRoles.ADMIN)
        });
    }

    function disapproveOperator(address operator) external onlyOwner {
        IOperatorPanel(perpManager).disapproveOperator({
            account: address(this),
            operator: operator,
            roles: 1 << uint256(PerpsOperatorRoles.ADMIN)
        });
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                           PERP MANAGER HOOK
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function addSubaccount(uint256 subaccount) external onlyPerpManager {
        _subaccounts.add(subaccount);
    }

    function removeSubaccount(uint256 subaccount) external onlyPerpManager {
        _subaccounts.remove(subaccount);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function totalAssets() public view override returns (uint256) {
        return usdc.balanceOf(address(this)) + orderbookCollateral() + freeCollateralBalance() + totalAccountValue();
    }

    function totalAccountValue() public view returns (uint256 accountValue) {
        uint256[] memory subaccounts = _subaccounts.values();

        int256 subaccountValue;
        for (uint256 i; i < subaccounts.length; ++i) {
            subaccountValue = IViewPort(perpManager).getAccountValue(address(this), subaccounts[i]);
            if (subaccountValue > 0) accountValue += subaccountValue.abs();
        }
    }

    function orderbookCollateral() public view returns (uint256 collateral) {
        uint256[] memory subaccounts = _subaccounts.values();

        for (uint256 i; i < subaccounts.length; ++i) {
            collateral += IViewPort(perpManager).getOrderbookCollateral(address(this), subaccounts[i]);
        }
    }

    function freeCollateralBalance() public view returns (uint256) {
        return IViewPort(perpManager).getFreeCollateralBalance(address(this));
    }

    function getSubaccounts() external view returns (uint256[] memory) {
        return _subaccounts.values();
    }

    function getQueuedWithdrawal(uint256 id) external view returns (Withdrawal memory) {
        return _queuedWithdrawal[id];
    }

    function getQueuedShares(address account) external view returns (uint256) {
        return _queuedShares[account];
    }

    function getWithdrawalQueue() external view returns (uint256[] memory) {
        return _withdrawalQueue;
    }

    function hasAdminRole(address account) external view returns (bool) {
        return hasAllRoles(account, ADMIN_ROLE);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                           UNUSED 4626 LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function previewWithdraw(uint256) public pure override returns (uint256 assets) {
        return 0;
    }

    function maxWithdraw(address) public pure override returns (uint256) {
        return 0;
    }

    function maxRedeem(address) public pure override returns (uint256) {
        return 0;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                HELPERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _afterTokenTransfer(address from, address, uint256) internal view override {
        if (balanceOf(from) < _queuedShares[from]) revert InsufficientBalance();
    }

    function _dequeue(uint256 id) internal {
        uint256[] memory withdrawalQueue = _withdrawalQueue;
        uint256 length = withdrawalQueue.length;

        uint256[] memory newQueue = new uint256[](length - 1);

        uint256 idx;
        for (uint256 i; i < length; ++i) {
            if (withdrawalQueue[i] != id) newQueue[idx++] = withdrawalQueue[i];
        }

        _withdrawalQueue = newQueue;
    }

    function _dequeueBatch(uint256 num) internal {
        uint256[] memory withdrawalQueue = _withdrawalQueue;

        _withdrawalQueue = withdrawalQueue.slice(num, withdrawalQueue.length);
    }

    /// @dev copied from ERC4626, assuming virtual shares is true & _decimalOffset is 0
    function _convertToAssets(uint256 shares, uint256 allocatedAssets) public view virtual returns (uint256 assets) {
        return shares.fullMulDiv(usdc.balanceOf(address(this)) + allocatedAssets + 1, totalSupply() + 1);
    }

    function _assertAdmin() internal view {
        if (!hasAllRoles(msg.sender, ADMIN_ROLE) && msg.sender != owner()) revert NotAdmin();
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
pragma solidity ^0.8.4;

/// @notice Library for managing enumerable sets in storage.
/// @author Solady (https://github.com/vectorized/solady/blob/main/src/utils/EnumerableSetLib.sol)
///
/// @dev Note:
/// In many applications, the number of elements in an enumerable set is small.
/// This enumerable set implementation avoids storing the length and indices
/// for up to 3 elements. Once the length exceeds 3 for the first time, the length
/// and indices will be initialized. The amortized cost of adding elements is O(1).
///
/// The AddressSet implementation packs the length with the 0th entry.
library EnumerableSetLib {
    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                       CUSTOM ERRORS                        */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The index must be less than the length.
    error IndexOutOfBounds();

    /// @dev The value cannot be the zero sentinel.
    error ValueIsZeroSentinel();

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                         CONSTANTS                          */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev A sentinel value to denote the zero value in storage.
    /// No elements can be equal to this value.
    /// `uint72(bytes9(keccak256(bytes("_ZERO_SENTINEL"))))`.
    uint256 private constant _ZERO_SENTINEL = 0xfbb67fda52d4bfb8bf;

    /// @dev The storage layout is given by:
    /// ```
    ///     mstore(0x04, _ENUMERABLE_ADDRESS_SET_SLOT_SEED)
    ///     mstore(0x00, set.slot)
    ///     let rootSlot := keccak256(0x00, 0x24)
    ///     mstore(0x20, rootSlot)
    ///     mstore(0x00, shr(96, shl(96, value)))
    ///     let positionSlot := keccak256(0x00, 0x40)
    ///     let valueSlot := add(rootSlot, sload(positionSlot))
    ///     let valueInStorage := shr(96, sload(valueSlot))
    ///     let lazyLength := shr(160, shl(160, sload(rootSlot)))
    /// ```
    uint256 private constant _ENUMERABLE_ADDRESS_SET_SLOT_SEED = 0x978aab92;

    /// @dev The storage layout is given by:
    /// ```
    ///     mstore(0x04, _ENUMERABLE_WORD_SET_SLOT_SEED)
    ///     mstore(0x00, set.slot)
    ///     let rootSlot := keccak256(0x00, 0x24)
    ///     mstore(0x20, rootSlot)
    ///     mstore(0x00, value)
    ///     let positionSlot := keccak256(0x00, 0x40)
    ///     let valueSlot := add(rootSlot, sload(positionSlot))
    ///     let valueInStorage := sload(valueSlot)
    ///     let lazyLength := sload(not(rootSlot))
    /// ```
    uint256 private constant _ENUMERABLE_WORD_SET_SLOT_SEED = 0x18fb5864;

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                          STRUCTS                           */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev An enumerable address set in storage.
    struct AddressSet {
        uint256 _spacer;
    }

    /// @dev An enumerable bytes32 set in storage.
    struct Bytes32Set {
        uint256 _spacer;
    }

    /// @dev An enumerable uint256 set in storage.
    struct Uint256Set {
        uint256 _spacer;
    }

    /// @dev An enumerable int256 set in storage.
    struct Int256Set {
        uint256 _spacer;
    }

    /// @dev An enumerable uint8 set in storage. Useful for enums.
    struct Uint8Set {
        uint256 data;
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                     GETTERS / SETTERS                      */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Returns the number of elements in the set.
    function length(AddressSet storage set) internal view returns (uint256 result) {
        bytes32 rootSlot = _rootSlot(set);
        /// @solidity memory-safe-assembly
        assembly {
            let rootPacked := sload(rootSlot)
            let n := shr(160, shl(160, rootPacked))
            result := shr(1, n)
            for {} iszero(or(iszero(shr(96, rootPacked)), n)) {} {
                result := 1
                if iszero(sload(add(rootSlot, result))) { break }
                result := 2
                if iszero(sload(add(rootSlot, result))) { break }
                result := 3
                break
            }
        }
    }

    /// @dev Returns the number of elements in the set.
    function length(Bytes32Set storage set) internal view returns (uint256 result) {
        bytes32 rootSlot = _rootSlot(set);
        /// @solidity memory-safe-assembly
        assembly {
            let n := sload(not(rootSlot))
            result := shr(1, n)
            for {} iszero(n) {} {
                result := 0
                if iszero(sload(add(rootSlot, result))) { break }
                result := 1
                if iszero(sload(add(rootSlot, result))) { break }
                result := 2
                if iszero(sload(add(rootSlot, result))) { break }
                result := 3
                break
            }
        }
    }

    /// @dev Returns the number of elements in the set.
    function length(Uint256Set storage set) internal view returns (uint256 result) {
        result = length(_toBytes32Set(set));
    }

    /// @dev Returns the number of elements in the set.
    function length(Int256Set storage set) internal view returns (uint256 result) {
        result = length(_toBytes32Set(set));
    }

    /// @dev Returns the number of elements in the set.
    function length(Uint8Set storage set) internal view returns (uint256 result) {
        /// @solidity memory-safe-assembly
        assembly {
            for { let packed := sload(set.slot) } packed { result := add(1, result) } {
                packed := xor(packed, and(packed, add(1, not(packed))))
            }
        }
    }

    /// @dev Returns whether `value` is in the set.
    function contains(AddressSet storage set, address value) internal view returns (bool result) {
        bytes32 rootSlot = _rootSlot(set);
        /// @solidity memory-safe-assembly
        assembly {
            value := shr(96, shl(96, value))
            if eq(value, _ZERO_SENTINEL) {
                mstore(0x00, 0xf5a267f1) // `ValueIsZeroSentinel()`.
                revert(0x1c, 0x04)
            }
            if iszero(value) { value := _ZERO_SENTINEL }
            let rootPacked := sload(rootSlot)
            for {} 1 {} {
                if iszero(shr(160, shl(160, rootPacked))) {
                    result := 1
                    if eq(shr(96, rootPacked), value) { break }
                    if eq(shr(96, sload(add(rootSlot, 1))), value) { break }
                    if eq(shr(96, sload(add(rootSlot, 2))), value) { break }
                    result := 0
                    break
                }
                mstore(0x20, rootSlot)
                mstore(0x00, value)
                result := iszero(iszero(sload(keccak256(0x00, 0x40))))
                break
            }
        }
    }

    /// @dev Returns whether `value` is in the set.
    function contains(Bytes32Set storage set, bytes32 value) internal view returns (bool result) {
        bytes32 rootSlot = _rootSlot(set);
        /// @solidity memory-safe-assembly
        assembly {
            if eq(value, _ZERO_SENTINEL) {
                mstore(0x00, 0xf5a267f1) // `ValueIsZeroSentinel()`.
                revert(0x1c, 0x04)
            }
            if iszero(value) { value := _ZERO_SENTINEL }
            for {} 1 {} {
                if iszero(sload(not(rootSlot))) {
                    result := 1
                    if eq(sload(rootSlot), value) { break }
                    if eq(sload(add(rootSlot, 1)), value) { break }
                    if eq(sload(add(rootSlot, 2)), value) { break }
                    result := 0
                    break
                }
                mstore(0x20, rootSlot)
                mstore(0x00, value)
                result := iszero(iszero(sload(keccak256(0x00, 0x40))))
                break
            }
        }
    }

    /// @dev Returns whether `value` is in the set.
    function contains(Uint256Set storage set, uint256 value) internal view returns (bool result) {
        result = contains(_toBytes32Set(set), bytes32(value));
    }

    /// @dev Returns whether `value` is in the set.
    function contains(Int256Set storage set, int256 value) internal view returns (bool result) {
        result = contains(_toBytes32Set(set), bytes32(uint256(value)));
    }

    /// @dev Returns whether `value` is in the set.
    function contains(Uint8Set storage set, uint8 value) internal view returns (bool result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := and(1, shr(and(0xff, value), sload(set.slot)))
        }
    }

    /// @dev Adds `value` to the set. Returns whether `value` was not in the set.
    function add(AddressSet storage set, address value) internal returns (bool result) {
        bytes32 rootSlot = _rootSlot(set);
        /// @solidity memory-safe-assembly
        assembly {
            value := shr(96, shl(96, value))
            if eq(value, _ZERO_SENTINEL) {
                mstore(0x00, 0xf5a267f1) // `ValueIsZeroSentinel()`.
                revert(0x1c, 0x04)
            }
            if iszero(value) { value := _ZERO_SENTINEL }
            let rootPacked := sload(rootSlot)
            for { let n := shr(160, shl(160, rootPacked)) } 1 {} {
                mstore(0x20, rootSlot)
                if iszero(n) {
                    let v0 := shr(96, rootPacked)
                    if iszero(v0) {
                        sstore(rootSlot, shl(96, value))
                        result := 1
                        break
                    }
                    if eq(v0, value) { break }
                    let v1 := shr(96, sload(add(rootSlot, 1)))
                    if iszero(v1) {
                        sstore(add(rootSlot, 1), shl(96, value))
                        result := 1
                        break
                    }
                    if eq(v1, value) { break }
                    let v2 := shr(96, sload(add(rootSlot, 2)))
                    if iszero(v2) {
                        sstore(add(rootSlot, 2), shl(96, value))
                        result := 1
                        break
                    }
                    if eq(v2, value) { break }
                    mstore(0x00, v0)
                    sstore(keccak256(0x00, 0x40), 1)
                    mstore(0x00, v1)
                    sstore(keccak256(0x00, 0x40), 2)
                    mstore(0x00, v2)
                    sstore(keccak256(0x00, 0x40), 3)
                    rootPacked := or(rootPacked, 7)
                    n := 7
                }
                mstore(0x00, value)
                let p := keccak256(0x00, 0x40)
                if iszero(sload(p)) {
                    n := shr(1, n)
                    result := 1
                    sstore(p, add(1, n))
                    if iszero(n) {
                        sstore(rootSlot, or(3, shl(96, value)))
                        break
                    }
                    sstore(add(rootSlot, n), shl(96, value))
                    sstore(rootSlot, add(2, rootPacked))
                    break
                }
                break
            }
        }
    }

    /// @dev Adds `value` to the set. Returns whether `value` was not in the set.
    function add(Bytes32Set storage set, bytes32 value) internal returns (bool result) {
        bytes32 rootSlot = _rootSlot(set);
        /// @solidity memory-safe-assembly
        assembly {
            if eq(value, _ZERO_SENTINEL) {
                mstore(0x00, 0xf5a267f1) // `ValueIsZeroSentinel()`.
                revert(0x1c, 0x04)
            }
            if iszero(value) { value := _ZERO_SENTINEL }
            for { let n := sload(not(rootSlot)) } 1 {} {
                mstore(0x20, rootSlot)
                if iszero(n) {
                    let v0 := sload(rootSlot)
                    if iszero(v0) {
                        sstore(rootSlot, value)
                        result := 1
                        break
                    }
                    if eq(v0, value) { break }
                    let v1 := sload(add(rootSlot, 1))
                    if iszero(v1) {
                        sstore(add(rootSlot, 1), value)
                        result := 1
                        break
                    }
                    if eq(v1, value) { break }
                    let v2 := sload(add(rootSlot, 2))
                    if iszero(v2) {
                        sstore(add(rootSlot, 2), value)
                        result := 1
                        break
                    }
                    if eq(v2, value) { break }
                    mstore(0x00, v0)
                    sstore(keccak256(0x00, 0x40), 1)
                    mstore(0x00, v1)
                    sstore(keccak256(0x00, 0x40), 2)
                    mstore(0x00, v2)
                    sstore(keccak256(0x00, 0x40), 3)
                    n := 7
                }
                mstore(0x00, value)
                let p := keccak256(0x00, 0x40)
                if iszero(sload(p)) {
                    n := shr(1, n)
                    sstore(add(rootSlot, n), value)
                    sstore(p, add(1, n))
                    sstore(not(rootSlot), or(1, shl(1, add(1, n))))
                    result := 1
                    break
                }
                break
            }
        }
    }

    /// @dev Adds `value` to the set. Returns whether `value` was not in the set.
    function add(Uint256Set storage set, uint256 value) internal returns (bool result) {
        result = add(_toBytes32Set(set), bytes32(value));
    }

    /// @dev Adds `value` to the set. Returns whether `value` was not in the set.
    function add(Int256Set storage set, int256 value) internal returns (bool result) {
        result = add(_toBytes32Set(set), bytes32(uint256(value)));
    }

    /// @dev Adds `value` to the set. Returns whether `value` was not in the set.
    function add(Uint8Set storage set, uint8 value) internal returns (bool result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := sload(set.slot)
            let mask := shl(and(0xff, value), 1)
            sstore(set.slot, or(result, mask))
            result := iszero(and(result, mask))
        }
    }

    /// @dev Removes `value` from the set. Returns whether `value` was in the set.
    function remove(AddressSet storage set, address value) internal returns (bool result) {
        bytes32 rootSlot = _rootSlot(set);
        /// @solidity memory-safe-assembly
        assembly {
            value := shr(96, shl(96, value))
            if eq(value, _ZERO_SENTINEL) {
                mstore(0x00, 0xf5a267f1) // `ValueIsZeroSentinel()`.
                revert(0x1c, 0x04)
            }
            if iszero(value) { value := _ZERO_SENTINEL }
            let rootPacked := sload(rootSlot)
            for { let n := shr(160, shl(160, rootPacked)) } 1 {} {
                if iszero(n) {
                    result := 1
                    if eq(shr(96, rootPacked), value) {
                        sstore(rootSlot, sload(add(rootSlot, 1)))
                        sstore(add(rootSlot, 1), sload(add(rootSlot, 2)))
                        sstore(add(rootSlot, 2), 0)
                        break
                    }
                    if eq(shr(96, sload(add(rootSlot, 1))), value) {
                        sstore(add(rootSlot, 1), sload(add(rootSlot, 2)))
                        sstore(add(rootSlot, 2), 0)
                        break
                    }
                    if eq(shr(96, sload(add(rootSlot, 2))), value) {
                        sstore(add(rootSlot, 2), 0)
                        break
                    }
                    result := 0
                    break
                }
                mstore(0x20, rootSlot)
                mstore(0x00, value)
                let p := keccak256(0x00, 0x40)
                let position := sload(p)
                if iszero(position) { break }
                n := sub(shr(1, n), 1)
                if iszero(eq(sub(position, 1), n)) {
                    let lastValue := shr(96, sload(add(rootSlot, n)))
                    sstore(add(rootSlot, sub(position, 1)), shl(96, lastValue))
                    sstore(add(rootSlot, n), 0)
                    mstore(0x00, lastValue)
                    sstore(keccak256(0x00, 0x40), position)
                }
                sstore(rootSlot, or(shl(96, shr(96, sload(rootSlot))), or(shl(1, n), 1)))
                sstore(p, 0)
                result := 1
                break
            }
        }
    }

    /// @dev Removes `value` from the set. Returns whether `value` was in the set.
    function remove(Bytes32Set storage set, bytes32 value) internal returns (bool result) {
        bytes32 rootSlot = _rootSlot(set);
        /// @solidity memory-safe-assembly
        assembly {
            if eq(value, _ZERO_SENTINEL) {
                mstore(0x00, 0xf5a267f1) // `ValueIsZeroSentinel()`.
                revert(0x1c, 0x04)
            }
            if iszero(value) { value := _ZERO_SENTINEL }
            for { let n := sload(not(rootSlot)) } 1 {} {
                if iszero(n) {
                    result := 1
                    if eq(sload(rootSlot), value) {
                        sstore(rootSlot, sload(add(rootSlot, 1)))
                        sstore(add(rootSlot, 1), sload(add(rootSlot, 2)))
                        sstore(add(rootSlot, 2), 0)
                        break
                    }
                    if eq(sload(add(rootSlot, 1)), value) {
                        sstore(add(rootSlot, 1), sload(add(rootSlot, 2)))
                        sstore(add(rootSlot, 2), 0)
                        break
                    }
                    if eq(sload(add(rootSlot, 2)), value) {
                        sstore(add(rootSlot, 2), 0)
                        break
                    }
                    result := 0
                    break
                }
                mstore(0x20, rootSlot)
                mstore(0x00, value)
                let p := keccak256(0x00, 0x40)
                let position := sload(p)
                if iszero(position) { break }
                n := sub(shr(1, n), 1)
                if iszero(eq(sub(position, 1), n)) {
                    let lastValue := sload(add(rootSlot, n))
                    sstore(add(rootSlot, sub(position, 1)), lastValue)
                    sstore(add(rootSlot, n), 0)
                    mstore(0x00, lastValue)
                    sstore(keccak256(0x00, 0x40), position)
                }
                sstore(not(rootSlot), or(shl(1, n), 1))
                sstore(p, 0)
                result := 1
                break
            }
        }
    }

    /// @dev Removes `value` from the set. Returns whether `value` was in the set.
    function remove(Uint256Set storage set, uint256 value) internal returns (bool result) {
        result = remove(_toBytes32Set(set), bytes32(value));
    }

    /// @dev Removes `value` from the set. Returns whether `value` was in the set.
    function remove(Int256Set storage set, int256 value) internal returns (bool result) {
        result = remove(_toBytes32Set(set), bytes32(uint256(value)));
    }

    /// @dev Removes `value` from the set. Returns whether `value` was in the set.
    function remove(Uint8Set storage set, uint8 value) internal returns (bool result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := sload(set.slot)
            let mask := shl(and(0xff, value), 1)
            sstore(set.slot, and(result, not(mask)))
            result := iszero(iszero(and(result, mask)))
        }
    }

    /// @dev Returns all of the values in the set.
    /// Note: This can consume more gas than the block gas limit for large sets.
    function values(AddressSet storage set) internal view returns (address[] memory result) {
        bytes32 rootSlot = _rootSlot(set);
        /// @solidity memory-safe-assembly
        assembly {
            let zs := _ZERO_SENTINEL
            let rootPacked := sload(rootSlot)
            let n := shr(160, shl(160, rootPacked))
            result := mload(0x40)
            let o := add(0x20, result)
            let v := shr(96, rootPacked)
            mstore(o, mul(v, iszero(eq(v, zs))))
            for {} 1 {} {
                if iszero(n) {
                    if v {
                        n := 1
                        v := shr(96, sload(add(rootSlot, n)))
                        if v {
                            n := 2
                            mstore(add(o, 0x20), mul(v, iszero(eq(v, zs))))
                            v := shr(96, sload(add(rootSlot, n)))
                            if v {
                                n := 3
                                mstore(add(o, 0x40), mul(v, iszero(eq(v, zs))))
                            }
                        }
                    }
                    break
                }
                n := shr(1, n)
                for { let i := 1 } lt(i, n) { i := add(i, 1) } {
                    v := shr(96, sload(add(rootSlot, i)))
                    mstore(add(o, shl(5, i)), mul(v, iszero(eq(v, zs))))
                }
                break
            }
            mstore(result, n)
            mstore(0x40, add(o, shl(5, n)))
        }
    }

    /// @dev Returns all of the values in the set.
    /// Note: This can consume more gas than the block gas limit for large sets.
    function values(Bytes32Set storage set) internal view returns (bytes32[] memory result) {
        bytes32 rootSlot = _rootSlot(set);
        /// @solidity memory-safe-assembly
        assembly {
            let zs := _ZERO_SENTINEL
            let n := sload(not(rootSlot))
            result := mload(0x40)
            let o := add(0x20, result)
            for {} 1 {} {
                if iszero(n) {
                    let v := sload(rootSlot)
                    if v {
                        n := 1
                        mstore(o, mul(v, iszero(eq(v, zs))))
                        v := sload(add(rootSlot, n))
                        if v {
                            n := 2
                            mstore(add(o, 0x20), mul(v, iszero(eq(v, zs))))
                            v := sload(add(rootSlot, n))
                            if v {
                                n := 3
                                mstore(add(o, 0x40), mul(v, iszero(eq(v, zs))))
                            }
                        }
                    }
                    break
                }
                n := shr(1, n)
                for { let i := 0 } lt(i, n) { i := add(i, 1) } {
                    let v := sload(add(rootSlot, i))
                    mstore(add(o, shl(5, i)), mul(v, iszero(eq(v, zs))))
                }
                break
            }
            mstore(result, n)
            mstore(0x40, add(o, shl(5, n)))
        }
    }

    /// @dev Returns all of the values in the set.
    /// Note: This can consume more gas than the block gas limit for large sets.
    function values(Uint256Set storage set) internal view returns (uint256[] memory result) {
        result = _toUints(values(_toBytes32Set(set)));
    }

    /// @dev Returns all of the values in the set.
    /// Note: This can consume more gas than the block gas limit for large sets.
    function values(Int256Set storage set) internal view returns (int256[] memory result) {
        result = _toInts(values(_toBytes32Set(set)));
    }

    /// @dev Returns all of the values in the set.
    function values(Uint8Set storage set) internal view returns (uint8[] memory result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := mload(0x40)
            let ptr := add(result, 0x20)
            let o := 0
            for { let packed := sload(set.slot) } packed {} {
                if iszero(and(packed, 0xffff)) {
                    o := add(o, 16)
                    packed := shr(16, packed)
                    continue
                }
                mstore(ptr, o)
                ptr := add(ptr, shl(5, and(packed, 1)))
                o := add(o, 1)
                packed := shr(1, packed)
            }
            mstore(result, shr(5, sub(ptr, add(result, 0x20))))
            mstore(0x40, ptr)
        }
    }

    /// @dev Returns the element at index `i` in the set. Reverts if `i` is out-of-bounds.
    function at(AddressSet storage set, uint256 i) internal view returns (address result) {
        bytes32 rootSlot = _rootSlot(set);
        /// @solidity memory-safe-assembly
        assembly {
            result := shr(96, sload(add(rootSlot, i)))
            result := mul(result, iszero(eq(result, _ZERO_SENTINEL)))
        }
        if (i >= length(set)) revert IndexOutOfBounds();
    }

    /// @dev Returns the element at index `i` in the set. Reverts if `i` is out-of-bounds.
    function at(Bytes32Set storage set, uint256 i) internal view returns (bytes32 result) {
        result = _rootSlot(set);
        /// @solidity memory-safe-assembly
        assembly {
            result := sload(add(result, i))
            result := mul(result, iszero(eq(result, _ZERO_SENTINEL)))
        }
        if (i >= length(set)) revert IndexOutOfBounds();
    }

    /// @dev Returns the element at index `i` in the set. Reverts if `i` is out-of-bounds.
    function at(Uint256Set storage set, uint256 i) internal view returns (uint256 result) {
        result = uint256(at(_toBytes32Set(set), i));
    }

    /// @dev Returns the element at index `i` in the set. Reverts if `i` is out-of-bounds.
    function at(Int256Set storage set, uint256 i) internal view returns (int256 result) {
        result = int256(uint256(at(_toBytes32Set(set), i)));
    }

    /// @dev Returns the element at index `i` in the set. Reverts if `i` is out-of-bounds.
    function at(Uint8Set storage set, uint256 i) internal view returns (uint8 result) {
        /// @solidity memory-safe-assembly
        assembly {
            let packed := sload(set.slot)
            for {} 1 {
                mstore(0x00, 0x4e23d035) // `IndexOutOfBounds()`.
                revert(0x1c, 0x04)
            } {
                if iszero(lt(i, 256)) { continue }
                for { let j := 0 } iszero(eq(i, j)) {} {
                    packed := xor(packed, and(packed, add(1, not(packed))))
                    j := add(j, 1)
                }
                if iszero(packed) { continue }
                break
            }
            // Find first set subroutine, optimized for smaller bytecode size.
            let x := and(packed, add(1, not(packed)))
            let r := shl(7, iszero(iszero(shr(128, x))))
            r := or(r, shl(6, iszero(iszero(shr(64, shr(r, x))))))
            r := or(r, shl(5, lt(0xffffffff, shr(r, x))))
            // For the lower 5 bits of the result, use a De Bruijn lookup.
            // forgefmt: disable-next-item
            result := or(r, byte(and(div(0xd76453e0, shr(r, x)), 0x1f),
                0x001f0d1e100c1d070f090b19131c1706010e11080a1a141802121b1503160405))
        }
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                      PRIVATE HELPERS                       */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Returns the root slot.
    function _rootSlot(AddressSet storage s) private pure returns (bytes32 r) {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(0x04, _ENUMERABLE_ADDRESS_SET_SLOT_SEED)
            mstore(0x00, s.slot)
            r := keccak256(0x00, 0x24)
        }
    }

    /// @dev Returns the root slot.
    function _rootSlot(Bytes32Set storage s) private pure returns (bytes32 r) {
        /// @solidity memory-safe-assembly
        assembly {
            mstore(0x04, _ENUMERABLE_WORD_SET_SLOT_SEED)
            mstore(0x00, s.slot)
            r := keccak256(0x00, 0x24)
        }
    }

    /// @dev Casts to a Bytes32Set.
    function _toBytes32Set(Uint256Set storage s) private pure returns (Bytes32Set storage c) {
        /// @solidity memory-safe-assembly
        assembly {
            c.slot := s.slot
        }
    }

    /// @dev Casts to a Bytes32Set.
    function _toBytes32Set(Int256Set storage s) private pure returns (Bytes32Set storage c) {
        /// @solidity memory-safe-assembly
        assembly {
            c.slot := s.slot
        }
    }

    /// @dev Casts to a uint256 array.
    function _toUints(bytes32[] memory a) private pure returns (uint256[] memory c) {
        /// @solidity memory-safe-assembly
        assembly {
            c := a
        }
    }

    /// @dev Casts to a int256 array.
    function _toInts(bytes32[] memory a) private pure returns (int256[] memory c) {
        /// @solidity memory-safe-assembly
        assembly {
            c := a
        }
    }
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

import {ICLOB} from "../ICLOB.sol";

type OrderId is uint256;

using OrderIdLib for OrderId global;

library OrderIdLib {
    function getClientOrderId(address account, uint96 id) internal pure returns (uint256) {
        return uint256(bytes32(abi.encodePacked(account, id)));
    }

    function toOrderId(uint256 id) internal pure returns (OrderId) {
        return OrderId.wrap(id);
    }

    function unwrap(OrderId id) internal pure returns (uint256) {
        return uint256(OrderId.unwrap(id));
    }

    function isNull(OrderId id) internal pure returns (bool) {
        return id.unwrap() == NULL_ORDER_ID;
    }
}

uint256 constant NULL_ORDER_ID = 0;
uint32 constant NULL_TIMESTAMP = 0;

enum Side {
    BUY,
    SELL
}

struct Order {
    // SLOT 0 //
    Side side;
    uint32 cancelTimestamp;
    OrderId id;
    OrderId prevOrderId;
    OrderId nextOrderId;
    // SLOT 1 //
    address owner;
    // SLOT 2 //
    uint256 price;
    // SLOT 3 //
    uint256 amount; // denominated in base for limit & either token for fill
}

using OrderLib for Order global;

library OrderLib {
    using OrderIdLib for uint256;

    /// @dev sig: 0xd36d8965
    error OrderNotFound();
    /// @dev sig: 0x207d0854
    error MarketOrderCannotMake();
    /// @dev sig: 0x3228b943
    error TakerOrdersCannotExpire();
    /// @dev sig: 0x048fe9b3
    error MakerOrderExpired();
    /// @dev sig: 0x07928dcd
    error PostOnlyOrderMustBeBaseDenominated();

    /// @dev Generates and Order from place order args and verifies the args do not conflict with eachother
    function toOrderChecked(ICLOB.PlaceOrderArgs calldata args, uint256 orderId, address owner)
        internal
        view
        returns (Order memory order)
    {
        // Validate market order constraints
        if (args.limitPrice == 0 && uint8(args.tif) < 2) revert MarketOrderCannotMake();

        // Check expiry for GTC and MOC orders (TiF 0 and 1)
        if (uint8(args.tif) <= 1 && args.expiryTime > 0 && args.expiryTime < block.timestamp) {
            revert MakerOrderExpired();
        }

        if (args.expiryTime > 0 && uint8(args.tif) > 1) revert TakerOrdersCannotExpire();

        if (args.tif == ICLOB.TiF.MOC && !args.baseDenominated) revert PostOnlyOrderMustBeBaseDenominated();

        // Set order fields after validation
        if (args.limitPrice > 0) {
            // limit order
            order.price = args.limitPrice;
        } else {
            // market order, limitPrice = 0 | +inf
            order.price = args.side == Side.BUY ? type(uint256).max : 0;
        }

        order.id = orderId.toOrderId();
        order.side = args.side;
        order.owner = owner;
        order.amount = args.amount;
        order.cancelTimestamp = args.expiryTime;
    }

    /// @dev Checks whether an order is expired from an Order struct
    function isExpired(Order memory self) internal view returns (bool) {
        // slither-disable-next-line timestamp
        return self.cancelTimestamp != NULL_TIMESTAMP && self.cancelTimestamp < block.timestamp;
    }

    /// @dev Checks whether an order is expired from a timestamp
    function isExpired(uint256 cancelTimestamp) internal view returns (bool) {
        // slither-disable-next-line timestamp
        return cancelTimestamp != NULL_TIMESTAMP && cancelTimestamp < block.timestamp;
    }

    /// @dev Checks whether an order is null
    function isNull(Order storage self) internal view returns (bool) {
        return self.id.unwrap() == NULL_ORDER_ID;
    }

    /// @dev Asserts that an order exists
    function assertExists(Order storage self) internal view {
        if (self.isNull()) revert OrderNotFound();
    }
}

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

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {ICLOBManager} from "../ICLOBManager.sol";

import {RedBlackTree} from "./RedBlackTree.sol";
import {Side, Order, OrderLib, OrderId, OrderIdLib} from "./Order.sol";

import {EventNonceLib as BookEventNonce} from "contracts/utils/types/EventNonce.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";



import {RedBlackTree} from "./RedBlackTree.sol";
import {Side, Order, OrderLib, OrderId, OrderIdLib} from "./Order.sol";

import {EventNonceLib as BookEventNonce} from "contracts/utils/types/EventNonce.sol";

uint256 constant MIN_MIN_LIMIT_ORDER_AMOUNT_BASE = 100;

struct Limit {
    uint64 numOrders;
    OrderId headOrder;
    OrderId tailOrder;
}

struct Book {
    RedBlackTree bidTree;
    RedBlackTree askTree;
    mapping(OrderId => Order) orders;
    mapping(uint256 price => Limit) bidLimits;
    mapping(uint256 price => Limit) askLimits;
}

struct MarketConfig {
    address quoteToken;
    address baseToken;
    uint256 quoteSize;
    uint256 baseSize;
}

struct MarketSettings {
    bool status;
    uint8 maxLimitsPerTx;
    uint256 minLimitOrderAmountInBase;
    uint256 tickSize;
    uint256 lotSizeInBase;
}

struct MarketMetadata {
    uint96 orderIdCounter;
    uint256 numBids;
    uint256 numAsks;
    uint256 baseTokenOpenInterest;
    uint256 quoteTokenOpenInterest;
}

using BookLib for Book global;
using CLOBStorageLib for Book global;
using FixedPointMathLib for uint256;

// slither-disable-start unimplemented-functions
library BookLib {
    using OrderIdLib for uint256;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                EVENTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0xe4f5b5cce490cd2969d01f4e8d15a7ec5650b813f83bc427e602c826540052be
    event LimitOrderCreated(
        uint256 indexed eventNonce, OrderId indexed orderId, uint256 price, uint256 amount, Side side
    );

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ERRORS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0xb3a23067
    error OrderIdInUse();
    /// @dev sig: 0x78591828
    error LotSizeInvalid();
    /// @dev sig: 0x9d6417b2
    error LimitPriceInvalid();
    /// @dev sig: 0x40dd76ff
    error LimitsPlacedExceedsMax();
    /// @dev sig: 0x2090fe47
    error LimitOrderAmountInvalid();

    /// @dev This caches the global max limit whitelist status stored in the manager
    /// so that makers placing a large number of limits only incurs one call to the factory
    /// intentionally not cleared
    bytes32 constant TRANSIENT_MAX_LIMIT_ALLOWLIST =
        keccak256(abi.encode(uint256(keccak256("TRANSIENT_MAX_LIMIT_ALLOWLIST")) - 1)) & ~bytes32(uint256(0xff));

    /// @dev This is the counter for how many limits have been placed in a txn, intentionally not cleared
    bytes32 constant TRANSIENT_LIMITS_PLACED =
        keccak256(abi.encode(uint256(keccak256("TRANSIENT_LIMITS_PLACED")) - 1)) & ~bytes32(uint256(0xff));

    // ASSERTIONS //

    /// @dev Asserts that the limit price is a multiple of the tick size
    function assertLimitPriceInBounds(Book storage self, uint256 price) internal view {
        uint256 tickSize = self.settings().tickSize;

        if (price % tickSize > 0 || price == 0) revert LimitPriceInvalid();
    }

    function assertLotSizeCompliant(Book storage self, uint256 amount) internal view {
        if (amount % self.settings().lotSizeInBase > 0) revert LotSizeInvalid();
    }

    /// @dev Asserts that the make order amount is valid (>= min amount and lot size compliant)
    function assertMakeAmountInBounds(Book storage self, uint256 orderAmountInBase) internal view {
        if (orderAmountInBase < self.settings().minLimitOrderAmountInBase) revert LimitOrderAmountInvalid();
        if (orderAmountInBase % self.settings().lotSizeInBase != 0) revert LotSizeInvalid();
    }

    /// @dev Asserts that the order id is not in use
    function assertUnusedOrderId(Book storage self, uint256 orderId) internal view {
        if (self.orders[orderId.toOrderId()].owner > address(0)) revert OrderIdInUse();
    }

    // MUTABLE FUNCTIONS //

    /// @dev Stores if the caller can avoid the max limit whitelist locally
    function setMaxLimitExemptTransient(address who, bool toggle) internal {
        bytes32 slot = keccak256(abi.encode(who, TRANSIENT_MAX_LIMIT_ALLOWLIST));

        // slither-disable-next-line assembly
        assembly {
            tstore(slot, toggle)
        }
    }

    /// @dev Increments the number of limits placed this txn, reverts if max is exceeded and caller is now allowlisted
    function incrementLimitsPlaced(Book storage self, address factory, address account) internal {
        uint8 limitsPlaced = getTransientLimitsPlaced();

        if (limitsPlaced >= self.settings().maxLimitsPerTx && !isMaxLimitExempt(self, factory, account)) {
            revert LimitsPlacedExceedsMax();
        }

        bytes32 slot = TRANSIENT_LIMITS_PLACED;

        // slither-disable-next-line assembly
        assembly {
            tstore(slot, add(limitsPlaced, 1))
        }
    }

    /// @dev Creates and returns a new OrderId nonce
    function incrementOrderId(Book storage self) internal returns (uint256) {
        return (++self.metadata().orderIdCounter);
    }

    /// @dev Adds a limit order to the book
    function addOrderToBook(Book storage self, Order memory order) internal {
        Limit storage limit = _updateBookPostOrder(self, order);

        _updateLimitPostOrder(self, limit, order);
    }

    /// @dev Removes an order from the book
    function removeOrderFromBook(Book storage self, Order storage order) internal {
        _updateLimitRemoveOrder(self, order);
        _updateBookRemoveOrder(self, order);
    }

    // VIEW FUNCTIONS //

    function boundToLots(Book storage self, uint256 baseAmount) internal view returns (uint256) {
        uint256 lotSize = self.settings().lotSizeInBase;

        return baseAmount / lotSize * lotSize;
    }

    /// @dev Returns the max limit exempt status for an `account` (whether he's restricted to an amount of tx/block or not)
    function isMaxLimitExempt(Book storage self, address factory, address who) internal returns (bool allowed) {
        bytes32 slot = keccak256(abi.encode(who, TRANSIENT_MAX_LIMIT_ALLOWLIST));

        // slither-disable-next-line assembly
        assembly {
            allowed := tload(slot)
        }

        if (!allowed) {
            allowed = ICLOBManager(factory).getMaxLimitExempt(who);
            if (!allowed) return allowed;
            setMaxLimitExemptTransient(who, allowed);
            return allowed;
        }
    }

    /// @dev Returns the next orders for a given start order id and number of orders
    function getNextOrders(Book storage self, OrderId startOrderId, uint256 numOrders)
        internal
        view
        returns (Order[] memory orders)
    {
        Order storage currentOrder = self.orders[startOrderId];
        currentOrder.assertExists();

        uint256 count = 0;
        orders = new Order[](numOrders);

        while (count < numOrders && !currentOrder.isNull()) {
            orders[count] = currentOrder;
            count++;

            if (currentOrder.nextOrderId.unwrap() != 0) {
                currentOrder = self.orders[currentOrder.nextOrderId];
            } else {
                uint256 price = self.getNextBiggestPrice(currentOrder.price, currentOrder.side);

                if (price == 0) break;

                Limit storage nextLimit = self.getLimit(price, currentOrder.side);

                currentOrder = self.orders[nextLimit.headOrder];
            }
        }
    }

    function getOrdersPaginated(Book storage ds, Order memory startOrder, uint256 pageSize)
        internal
        view
        returns (Order[] memory result, Order memory nextOrder)
    {
        Order[] memory orders = new Order[](pageSize);
        nextOrder = startOrder;
        uint256 counter;

        while (counter < pageSize) {
            if (nextOrder.id.unwrap() == 0) break;
            orders[counter] = nextOrder;
            if (nextOrder.nextOrderId.unwrap() == 0) {
                nextOrder = nextOrder.side == Side.BUY
                    ? ds.orders[ds.bidLimits[ds.getNextSmallestPrice(nextOrder.price, Side.BUY)].headOrder]
                    : ds.orders[ds.askLimits[ds.getNextBiggestPrice(nextOrder.price, Side.SELL)].headOrder];
            } else {
                nextOrder = ds.orders[nextOrder.nextOrderId];
            }
            counter++;
        }

        assembly {
            result := orders
            mstore(mul(lt(counter, mload(result)), result), counter)
        }

        return (result, nextOrder);
    }

    function getBaseQuanta(Book storage self) internal view returns (uint256) {
        MarketSettings storage marketSettings = self.settings();

        return marketSettings.lotSizeInBase.fullMulDiv(marketSettings.tickSize, self.config().baseSize);
    }

    // PURE FUNCTIONS //

    /// @dev Returns the number of limit orders placed this transaction
    function getTransientLimitsPlaced() internal view returns (uint8 limitsPlaced) {
        bytes32 slot = TRANSIENT_LIMITS_PLACED;

        // This solidity version does not support the `transient` identifier
        // slither-disable-next-line assembly
        assembly {
            limitsPlaced := tload(slot)
        }
    }

    // PRIVATE FUNCTIONS //

    function _updateBookPostOrder(Book storage self, Order memory order) private returns (Limit storage limit) {
        if (order.side == Side.BUY) {
            limit = self.bidLimits[order.price];
            if (limit.numOrders == 0) self.bidTree.insert(order.price);
            self.metadata().numBids++;
            self.metadata().quoteTokenOpenInterest += self.getQuoteTokenAmount(order.price, order.amount);
        } else {
            limit = self.askLimits[order.price];
            if (limit.numOrders == 0) self.askTree.insert(order.price);
            self.metadata().numAsks++;
            self.metadata().baseTokenOpenInterest += order.amount;
        }

        self.orders[order.id] = order;
    }

    function _updateLimitPostOrder(Book storage self, Limit storage limit, Order memory order) private {
        limit.numOrders++;

        if (limit.headOrder.isNull()) {
            limit.headOrder = order.id;
            limit.tailOrder = order.id;
        } else {
            Order storage tailOrder = self.orders[limit.tailOrder];
            tailOrder.nextOrderId = order.id;
            self.orders[order.id].prevOrderId = tailOrder.id;
            limit.tailOrder = order.id;
        }

        emit LimitOrderCreated(BookEventNonce.inc(), order.id, order.price, order.amount, order.side);
    }

    function _updateBookRemoveOrder(Book storage self, Order storage order) private {
        if (order.side == Side.BUY) {
            self.metadata().numBids--;

            self.metadata().quoteTokenOpenInterest -= self.getQuoteTokenAmount(order.price, order.amount);
        } else {
            self.metadata().numAsks--;

            self.metadata().baseTokenOpenInterest -= order.amount;
        }

        delete self.orders[order.id];
    }

    function _updateLimitRemoveOrder(Book storage self, Order storage order) private {
        uint256 price = order.price;

        Limit storage limit = order.side == Side.BUY ? self.bidLimits[price] : self.askLimits[price];

        if (limit.numOrders == 1) {
            if (order.side == Side.BUY) {
                delete self.bidLimits[price];
                self.bidTree.remove(price);
            } else {
                delete self.askLimits[price];
                self.askTree.remove(price);
            }
            return;
        }

        limit.numOrders--;

        OrderId prev = order.prevOrderId;
        OrderId next = order.nextOrderId;

        if (!prev.isNull()) self.orders[prev].nextOrderId = next;
        else limit.headOrder = next;

        if (!next.isNull()) self.orders[next].prevOrderId = prev;
        else limit.tailOrder = prev;
    }
}

/// @custom:storage-location erc7201:CLOBStorage
library CLOBStorageLib {

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                EVENTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0xdf07ebd269c613b8a3f2d3a9b3763bfed22597dc93ca6f40caf8773ebabf7d50
    event TickSizeUpdated(uint256 indexed eventNonce, uint256 indexed newTickSize);
    /// @dev sig: 0x1c8841f14ca7c4f639d9207829e05ea911febfd6609afc496f63efb5819f51f0
    event LotSizeInBaseUpdated(uint256 indexed eventNonce, uint256 indexed newLotSizeInBase);
    /// @dev sig: 0x1f4e491a4e8eba2c859a70417419f56aa296c496af7e1eccd17c5f2ee93aa36b
    event MaxLimitOrdersPerTxUpdated(uint256 indexed eventNonce, uint256 indexed newMaxLimits);
    /// @dev sig: 0xba6e3f8f80a920a3d4235f1df6df25a19c03bc81803cc4791feaee0aa6e548d3
    event MinLimitOrderAmountInBaseUpdated(uint256 indexed eventNonce, uint256 indexed newMinLimitOrderAmountInBase);

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ERRORS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0x2cd8344a
    error NewLotSizeInvalid();
    /// @dev sig: 0xd35bd829
    error NewTickSizeInvalid();
    /// @dev sig: 0xd78d4cbe
    error NewMaxLimitsPerTxInvalid();
    /// @dev sig: 0x4e63c1c2
    error NewMinLimitOrderAmountInvalid();

    bytes32 constant CLOB_STORAGE_POSITION =
        keccak256(abi.encode(uint256(keccak256("CLOBStorage")) - 1)) & ~bytes32(uint256(0xff));

    bytes32 constant MARKET_CONFIG_STORAGE_POSITION =
        keccak256(abi.encode(uint256(keccak256("MarketConfigStorage")) - 1)) & ~bytes32(uint256(0xff));

    bytes32 constant MARKET_SETTINGS_STORAGE_POSITION =
        keccak256(abi.encode(uint256(keccak256("MarketSettingsStorage")) - 1)) & ~bytes32(uint256(0xff));

    bytes32 constant MARKET_METADATA_STORAGE_POSITION =
        keccak256(abi.encode(uint256(keccak256("MarketMetadataStorage")) - 1)) & ~bytes32(uint256(0xff));

    /// @dev These functions expose the 3 book data structs as phantom fields
    /// while allowing their storage to be independent in case of updates

    function settings(Book storage) internal pure returns (MarketSettings storage) {
        return _getMarketSettingsStorage();
    }

    function config(Book storage) internal pure returns (MarketConfig storage) {
        return _getMarketConfigStorage();
    }

    function metadata(Book storage) internal pure returns (MarketMetadata storage) {
        return _getMarketMetadataStorage();
    }

    // slither-disable-next-line uninitialized-storage
    function _getCLOBStorage() internal pure returns (Book storage self) {
        bytes32 slot = CLOB_STORAGE_POSITION;

        // slither-disable-next-line assembly
        assembly {
            self.slot := slot
        }
    }

    // slither-disable-next-line uninitialized-storage
    function _getMarketConfigStorage() internal pure returns (MarketConfig storage self) {
        bytes32 slot = MARKET_CONFIG_STORAGE_POSITION;

        // slither-disable-next-line assembly
        assembly {
            self.slot := slot
        }
    }

    // slither-disable-next-line uninitialized-storage
    function _getMarketSettingsStorage() internal pure returns (MarketSettings storage self) {
        bytes32 slot = MARKET_SETTINGS_STORAGE_POSITION;

        // slither-disable-next-line assembly
        assembly {
            self.slot := slot
        }
    }

    // slither-disable-next-line uninitialized-storage
    function _getMarketMetadataStorage() internal pure returns (MarketMetadata storage self) {
        bytes32 slot = MARKET_METADATA_STORAGE_POSITION;

        // slither-disable-next-line assembly
        assembly {
            self.slot := slot
        }
    }

    /// @dev Returns the highest bid price
    function getBestBidPrice(Book storage self) internal view returns (uint256) {
        return self.bidTree.maximum();
    }

    /// @dev Returns the lowest ask price
    function getBestAskPrice(Book storage self) internal view returns (uint256) {
        return self.askTree.minimum();
    }

    /// @dev Returns the lowest bid price
    function getWorstBidPrice(Book storage self) internal view returns (uint256) {
        return self.bidTree.minimum();
    }

    /// @dev Returns the highest ask price
    function getWorstAskPrice(Book storage self) internal view returns (uint256) {
        return self.askTree.maximum();
    }

    /// @dev Returns the limit for a given price and side
    function getLimit(Book storage self, uint256 price, Side side) internal view returns (Limit storage) {
        return side == Side.BUY ? self.bidLimits[price] : self.askLimits[price];
    }

    /// @dev Returns the next biggest price for a given price and side
    function getNextBiggestPrice(Book storage self, uint256 price, Side side) internal view returns (uint256) {
        return side == Side.BUY ? self.bidTree.getNextBiggest(price) : self.askTree.getNextBiggest(price);
    }

    /// @dev Returns the next smallest price for a given price and side
    function getNextSmallestPrice(Book storage self, uint256 price, Side side) internal view returns (uint256) {
        return side == Side.BUY ? self.bidTree.getNextSmallest(price) : self.askTree.getNextSmallest(price);
    }

    /// @dev Returns the base token amount for a given price and quote amount
    function getBaseTokenAmount(Book storage self, uint256 price, uint256 quoteAmount)
        internal
        view
        returns (uint256)
    {
        return quoteAmount * self.config().baseSize / price;
    }

    /// @dev Returns the quote token amount for a given price and base amount
    function getQuoteTokenAmount(Book storage self, uint256 price, uint256 baseAmount)
        internal
        view
        returns (uint256 quoteAmount)
    {
        return baseAmount * price / self.config().baseSize;
    }

    function setMaxLimitsPerTx(Book storage self, uint8 newMaxLimits) internal {
        if (newMaxLimits == 0) revert NewMaxLimitsPerTxInvalid();

        self.settings().maxLimitsPerTx = newMaxLimits;

        emit MaxLimitOrdersPerTxUpdated(BookEventNonce.inc(), newMaxLimits);
    }

    function setTickSize(Book storage self, uint256 newTickSize) internal {
        self.settings().tickSize = newTickSize;

        if (self.getBaseQuanta() == 0) revert NewTickSizeInvalid();

        emit TickSizeUpdated(BookEventNonce.inc(), newTickSize);
    }

    function setMinLimitOrderAmountInBase(Book storage self, uint256 newMinLimitOrderAmountInBase) internal {
        if (newMinLimitOrderAmountInBase < self.settings().lotSizeInBase) revert NewMinLimitOrderAmountInvalid();

        self.settings().minLimitOrderAmountInBase = newMinLimitOrderAmountInBase;

        emit MinLimitOrderAmountInBaseUpdated(BookEventNonce.inc(), newMinLimitOrderAmountInBase);
    }

    function setLotSizeInBase(Book storage self, uint256 newLotSizeInBase) internal {
        self.settings().lotSizeInBase = newLotSizeInBase;

        if (self.settings().minLimitOrderAmountInBase < newLotSizeInBase) revert NewLotSizeInvalid();
        if (self.getBaseQuanta() == 0) revert NewLotSizeInvalid();

        emit LotSizeInBaseUpdated(BookEventNonce.inc(), newLotSizeInBase);
    }

    /// @dev Initializes the market config and setting
    function init(Book storage self, MarketConfig memory marketConfig, MarketSettings memory marketSettings) internal {
        MarketConfig storage cs = self.config();
        MarketSettings storage ss = self.settings();

        cs.quoteToken = marketConfig.quoteToken;
        cs.baseToken = marketConfig.baseToken;
        cs.quoteSize = marketConfig.quoteSize;
        cs.baseSize = marketConfig.baseSize;

        ss.status = marketSettings.status;
        ss.maxLimitsPerTx = marketSettings.maxLimitsPerTx;
        ss.minLimitOrderAmountInBase = marketSettings.minLimitOrderAmountInBase;
        ss.tickSize = marketSettings.tickSize;
        ss.lotSizeInBase = marketSettings.lotSizeInBase;
    }
}
// slither-disable-end unimplemented-functions

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

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

/// @notice Initializable mixin for the upgradeable contracts.
/// @author Solady (https://github.com/vectorized/solady/blob/main/src/utils/Initializable.sol)
/// @author Modified from OpenZeppelin (https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/contracts/proxy/utils/Initializable.sol)
abstract contract Initializable {
    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                       CUSTOM ERRORS                        */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The contract is already initialized.
    error InvalidInitialization();

    /// @dev The contract is not initializing.
    error NotInitializing();

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                           EVENTS                           */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Triggered when the contract has been initialized.
    event Initialized(uint64 version);

    /// @dev `keccak256(bytes("Initialized(uint64)"))`.
    bytes32 private constant _INTIALIZED_EVENT_SIGNATURE =
        0xc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2;

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                          STORAGE                           */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The default initializable slot is given by:
    /// `bytes32(~uint256(uint32(bytes4(keccak256("_INITIALIZABLE_SLOT")))))`.
    ///
    /// Bits Layout:
    /// - [0]     `initializing`
    /// - [1..64] `initializedVersion`
    bytes32 private constant _INITIALIZABLE_SLOT =
        0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffbf601132;

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                         OPERATIONS                         */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Override to return a custom storage slot if required.
    function _initializableSlot() internal pure virtual returns (bytes32) {
        return _INITIALIZABLE_SLOT;
    }

    /// @dev Guards an initializer function so that it can be invoked at most once.
    ///
    /// You can guard a function with `onlyInitializing` such that it can be called
    /// through a function guarded with `initializer`.
    ///
    /// This is similar to `reinitializer(1)`, except that in the context of a constructor,
    /// an `initializer` guarded function can be invoked multiple times.
    /// This can be useful during testing and is not expected to be used in production.
    ///
    /// Emits an {Initialized} event.
    modifier initializer() virtual {
        bytes32 s = _initializableSlot();
        /// @solidity memory-safe-assembly
        assembly {
            let i := sload(s)
            // Set `initializing` to 1, `initializedVersion` to 1.
            sstore(s, 3)
            // If `!(initializing == 0 && initializedVersion == 0)`.
            if i {
                // If `!(address(this).code.length == 0 && initializedVersion == 1)`.
                if iszero(lt(extcodesize(address()), eq(shr(1, i), 1))) {
                    mstore(0x00, 0xf92ee8a9) // `InvalidInitialization()`.
                    revert(0x1c, 0x04)
                }
                s := shl(shl(255, i), s) // Skip initializing if `initializing == 1`.
            }
        }
        _;
        /// @solidity memory-safe-assembly
        assembly {
            if s {
                // Set `initializing` to 0, `initializedVersion` to 1.
                sstore(s, 2)
                // Emit the {Initialized} event.
                mstore(0x20, 1)
                log1(0x20, 0x20, _INTIALIZED_EVENT_SIGNATURE)
            }
        }
    }

    /// @dev Guards an reinitialzer function so that it can be invoked at most once.
    ///
    /// You can guard a function with `onlyInitializing` such that it can be called
    /// through a function guarded with `reinitializer`.
    ///
    /// Emits an {Initialized} event.
    modifier reinitializer(uint64 version) virtual {
        bytes32 s = _initializableSlot();
        /// @solidity memory-safe-assembly
        assembly {
            version := and(version, 0xffffffffffffffff) // Clean upper bits.
            let i := sload(s)
            // If `initializing == 1 || initializedVersion >= version`.
            if iszero(lt(and(i, 1), lt(shr(1, i), version))) {
                mstore(0x00, 0xf92ee8a9) // `InvalidInitialization()`.
                revert(0x1c, 0x04)
            }
            // Set `initializing` to 1, `initializedVersion` to `version`.
            sstore(s, or(1, shl(1, version)))
        }
        _;
        /// @solidity memory-safe-assembly
        assembly {
            // Set `initializing` to 0, `initializedVersion` to `version`.
            sstore(s, shl(1, version))
            // Emit the {Initialized} event.
            mstore(0x20, version)
            log1(0x20, 0x20, _INTIALIZED_EVENT_SIGNATURE)
        }
    }

    /// @dev Guards a function such that it can only be called in the scope
    /// of a function guarded with `initializer` or `reinitializer`.
    modifier onlyInitializing() virtual {
        _checkInitializing();
        _;
    }

    /// @dev Reverts if the contract is not initializing.
    function _checkInitializing() internal view virtual {
        bytes32 s = _initializableSlot();
        /// @solidity memory-safe-assembly
        assembly {
            if iszero(and(1, sload(s))) {
                mstore(0x00, 0xd7e6bcf8) // `NotInitializing()`.
                revert(0x1c, 0x04)
            }
        }
    }

    /// @dev Locks any future initializations by setting the initialized version to `2**64 - 1`.
    ///
    /// Calling this in the constructor will prevent the contract from being initialized
    /// or reinitialized. It is recommended to use this to lock implementation contracts
    /// that are designed to be called through proxies.
    ///
    /// Emits an {Initialized} event the first time it is successfully called.
    function _disableInitializers() internal virtual {
        bytes32 s = _initializableSlot();
        /// @solidity memory-safe-assembly
        assembly {
            let i := sload(s)
            if and(i, 1) {
                mstore(0x00, 0xf92ee8a9) // `InvalidInitialization()`.
                revert(0x1c, 0x04)
            }
            let uint64max := shr(192, s) // Computed to save bytecode.
            if iszero(eq(shr(1, i), uint64max)) {
                // Set `initializing` to 0, `initializedVersion` to `2**64 - 1`.
                sstore(s, shl(1, uint64max))
                // Emit the {Initialized} event.
                mstore(0x20, uint64max)
                log1(0x20, 0x20, _INTIALIZED_EVENT_SIGNATURE)
            }
        }
    }

    /// @dev Returns the highest version that has been initialized.
    function _getInitializedVersion() internal view virtual returns (uint64 version) {
        bytes32 s = _initializableSlot();
        /// @solidity memory-safe-assembly
        assembly {
            version := shr(1, sload(s))
        }
    }

    /// @dev Returns whether the contract is currently initializing.
    function _isInitializing() internal view virtual returns (bool result) {
        bytes32 s = _initializableSlot();
        /// @solidity memory-safe-assembly
        assembly {
            result := and(1, sload(s))
        }
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

interface IGTL {
    function orderUpdated(int256 marginDelta) external;
    function addSubaccount(uint256 subaccount) external;
    function removeSubaccount(uint256 subaccount) external;
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


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

