
## *MAIN TARGET CONTRACT* TO REVIEW

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

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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

import {IOperatorPanel} from "./interfaces/IOperatorPanel.sol";
import {EventNonceLib as OperatorEventNonce} from "./types/EventNonce.sol";

// @todo rename "spot" to "account"
enum SpotOperatorRoles {
    ADMIN,
    PLACE_ORDER,
    SPOT_DEPOSIT,
    SPOT_WITHDRAW,
    PERP_TO_SPOT_DEPOSIT,
    LAUNCHPAD_FILL
}

enum PerpsOperatorRoles {
    ADMIN,
    PLACE_ORDER,
    SET_LEVERAGE,
    DEPOSIT_MARGIN,
    WITHDRAW_MARGIN,
    DEPOSIT_ACCOUNT,
    WITHDRAW_ACCOUNT,
    SPOT_TO_PERP_DEPOSIT
}

struct OperatorStorage {
    mapping(address account => mapping(address operator => uint256)) operatorRoleApprovals;
}

using OperatorStorageLib for OperatorStorage global;

/// @custom:storage-location erc7201:OperatorStorage
library OperatorStorageLib {
    bytes32 constant OPERATOR_STORAGE_POSITION =
        keccak256(abi.encode(uint256(keccak256("OperatorStorage")) - 1)) & ~bytes32(uint256(0xff));

    /// @dev Gets the storage slot of the storage struct for the contract calling this library function
    // slither-disable-next-line uninitialized-storage
    function getOperatorStorage() internal pure returns (OperatorStorage storage self) {
        bytes32 position = OPERATOR_STORAGE_POSITION;

        // slither-disable-next-line assembly
        assembly {
            self.slot := position
        }
    }
}

abstract contract OperatorPanel is IOperatorPanel {
    /// @dev sig: 0xb816c81e0d2e75687754a9cb3111541c16ab454792482bf1dd02093f2203f353
    event OperatorApproved(
        uint256 indexed eventNonce, address indexed account, address indexed operator, uint256 newRoles
    );
    /// @dev sig: 0x1145ef8300109b8668d5581d376603c552d28f5aaefa3ca8fb7524286a41a7ae
    event OperatorDisapproved(
        uint256 indexed eventNonce, address indexed account, address indexed operator, uint256 removedRoles
    );

    /// @dev sig: 0x732ea322
    error OperatorDoesNotHaveRole();
    /// @dev sig: 0xe9a05878
    error OperatorChangeUnauthorized();

    address public immutable operatorHub;

    constructor(address operatorHub_) {
        operatorHub = operatorHub_;
    }

    modifier onlySenderOrOperatorHub(address account) {
        if (msg.sender != account && msg.sender != operatorHub) revert OperatorChangeUnauthorized();
        _;
    }

    function _getOperatorStorage() internal pure returns (OperatorStorage storage self) {
        return OperatorStorageLib.getOperatorStorage();
    }

    function getOperatorRoleApprovals(address account, address operator) external view returns (uint256) {
        return _getOperatorStorage().operatorRoleApprovals[account][operator];
    }

    function approveOperator(address account, address operator, uint256 roles)
        external
        onlySenderOrOperatorHub(account)
    {
        OperatorStorage storage self = _getOperatorStorage();

        uint256 approvedRoles = self.operatorRoleApprovals[account][operator];
        self.operatorRoleApprovals[account][operator] = approvedRoles | roles;

        emit OperatorApproved(OperatorEventNonce.inc(), account, operator, roles);
    }

    function disapproveOperator(address account, address operator, uint256 roles)
        external
        onlySenderOrOperatorHub(account)
    {
        OperatorStorage storage self = _getOperatorStorage();

        uint256 approvedRoles = self.operatorRoleApprovals[account][operator];
        self.operatorRoleApprovals[account][operator] = approvedRoles & (~roles);

        emit OperatorDisapproved(OperatorEventNonce.inc(), account, operator, roles);
    }

    function getOperatorEventNonce() external view returns (uint256) {
        return OperatorEventNonce.getCurrentNonce();
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
        self.balance += amount;
    }

    function claim(InsuranceFund storage self, uint256 amount) internal {
        if (amount == 0) return;
        if (self.balance < amount) revert InsufficientInsuranceFundBalance();
        self.balance -= amount;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                 ADMIN
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function withdraw(InsuranceFund storage self, uint256 amount) internal {
        if (self.balance < amount) revert InsufficientInsuranceFundBalance();
        self.balance -= amount;
        USDC.safeTransfer(msg.sender, amount);
        emit InsuranceFundWithdrawal(msg.sender, amount);
    }

    function deposit(InsuranceFund storage self, uint256 amount) internal {
        self.balance += amount;
        USDC.safeTransferFrom(msg.sender, address(this), amount);
        emit InsuranceFundDeposit(msg.sender, amount);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getBalance(InsuranceFund storage self) internal view returns (uint256) {
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

import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {DynamicArrayLib} from "@solady/utils/DynamicArrayLib.sol";
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";

import {AdminPanel} from "./modules/AdminPanel.sol";
import {LiquidatorPanel} from "./modules/LiquidatorPanel.sol";
import {ViewPort} from "./modules/ViewPort.sol";

import {ClearingHouse, ClearingHouseLib} from "./types/ClearingHouse.sol";
import {Market, MarketLib} from "./types/Market.sol";
import {Position} from "./types/Position.sol";
import {StorageLib} from "./types/StorageLib.sol";
import {CLOBLib} from "./types/CLOBLib.sol";

import {Side, TiF, BookType, TradeType} from "./types/Enums.sol";
import {PlaceOrderArgs, PlaceOrderResult, AmendLimitOrderArgs} from "./types/Structs.sol";

import {IAccountManager} from "..//account-manager/IAccountManager.sol";

import {OperatorHelperLib} from "../utils/types/OperatorHelperLib.sol";
import {OperatorPanel, OperatorStorage, OperatorStorageLib, PerpsOperatorRoles} from "../utils/OperatorPanel.sol";

/// CONCURRENCY TODO ///
// @todo make nonces market-specific
// @todo isolate insurance payments, claims, and balance per market (will have to also make liquidations per market)
contract PerpManager is AdminPanel, LiquidatorPanel, ViewPort, OperatorPanel {
    using OperatorHelperLib for OperatorStorage;
    using FixedPointMathLib for uint256;
    using SafeCastLib for uint256;

    event PositionLeverageSet(
        bytes32 indexed asset,
        address indexed account,
        uint256 indexed subaccount,
        uint256 newLeverage,
        int256 collateralDelta,
        int256 newMargin,
        uint256 nonce
    );

    event MarginAdded(
        address indexed account, uint256 indexed subaccount, uint256 amount, int256 newMargin, uint256 nonce
    );
    event MarginRemoved(
        address indexed account, uint256 indexed subaccount, uint256 amount, int256 newMargin, uint256 nonce
    );

    error RemainingMarginInsufficient();
    error InvalidDeposit();
    error InvalidWithdraw();
    error NotAccountManager();
    error InvalidBackstopLimitOrder();

    constructor(address _accountManager, address _operatorHub) OperatorPanel(_operatorHub) {
        accountManager = IAccountManager(_accountManager);
        _disableInitializers();
    }

    IAccountManager immutable accountManager;

    struct __UpdateLeverageCache__ {
        DynamicArrayLib.DynamicArray assets;
        Position[] positions;
        int256 fundingPayment;
        uint256 currentLeverage;
        uint256 orderbookNotional;
        uint256 newOrderbookMargin;
        uint256 currentOrderbookMargin;
        int256 collateralDeltaFromBook;
        uint256 newMargin;
    }

    struct __MarginUpdateCache__ {
        DynamicArrayLib.DynamicArray assets;
        Position[] positions;
        int256 fundingPayment;
        uint256 intendedMargin;
    }

    modifier onlySenderOrOperator(address account, PerpsOperatorRoles requiredRole) {
        OperatorStorageLib.getOperatorStorage().onlySenderOrOperator(account, requiredRole);
        _;
    }

    modifier onlyActiveProtocol() override (AdminPanel, LiquidatorPanel) {
        if (!StorageLib.loadClearingHouse().active) revert ProtocolNotActive();
        _;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            FREE COLLATERAL
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function deposit(address account, uint256 amount)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.DEPOSIT_ACCOUNT)
    {
        StorageLib.loadCollateralManager().depositFreeCollateral(account, account, amount);
    }

    function withdraw(address account, uint256 amount)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.WITHDRAW_ACCOUNT)
    {
        StorageLib.loadCollateralManager().withdrawFreeCollateral(account, amount);
    }

    function depositTo(address account, uint256 amount) external {
        StorageLib.loadCollateralManager().depositFreeCollateral({
            from: msg.sender,
            to: account,
            amount: amount
        });
    }

    function depositFromSpot(address account, uint256 amount)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.SPOT_TO_PERP_DEPOSIT)
    {
        accountManager.withdrawToPerps(account, amount);
        StorageLib.loadCollateralManager().depositFromSpot(account, amount);
    }

    function withdrawToSpot(address account, uint256 amount) external {
        if (msg.sender != address(accountManager)) revert NotAccountManager();
        StorageLib.loadCollateralManager().withdrawToSpot(account, amount, address(accountManager));
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                 MARGIN
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function addMargin(address account, uint256 subaccount, uint256 amount)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.DEPOSIT_MARGIN)
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        __MarginUpdateCache__ memory cache;

        // load account
        (cache.assets, cache.positions) = clearingHouse.getAccount(account, subaccount);

        if (amount == 0) revert InvalidDeposit();
        if (cache.positions.length == 0) revert InvalidDeposit();

        // realize funding payment
        cache.fundingPayment = ClearingHouseLib.realizeFundingPayment(cache.assets, cache.positions);

        // settle margin update
        int256 remainingMargin = StorageLib.loadCollateralManager().settleMarginUpdate({
            account: account,
            subaccount: subaccount,
            marginDelta: amount.toInt256(),
            fundingPayment: cache.fundingPayment
        });

        // assert not liquidatable
        clearingHouse.assertNotLiquidatable({assets: cache.assets, positions: cache.positions, margin: remainingMargin});

        // set position update (note: this will just be the new position.lastCumulativeFunding)
        clearingHouse.setPositions({
            tradedAsset: "",
            account: account,
            subaccount: subaccount,
            assets: cache.assets,
            positions: cache.positions
        });

        emit MarginAdded(account, subaccount, amount, remainingMargin, StorageLib.incNonce());
    }

    function removeMargin(address account, uint256 subaccount, uint256 amount)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.WITHDRAW_MARGIN)
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        __MarginUpdateCache__ memory cache;

        // load account
        (cache.assets, cache.positions) = clearingHouse.getAccount(account, subaccount);

        if (amount == 0) revert InvalidWithdraw();
        if (cache.positions.length == 0) revert InvalidWithdraw();

        // realize funding payment
        cache.fundingPayment = ClearingHouseLib.realizeFundingPayment(cache.assets, cache.positions);

        // settle margin update
        int256 remainingMargin = StorageLib.loadCollateralManager().settleMarginUpdate({
            account: account,
            subaccount: subaccount,
            marginDelta: -amount.toInt256(),
            fundingPayment: cache.fundingPayment
        });

        // assert post withdraw margin requirement (margin + upnl) >= max(intendedMargin, totalNotional / 10)
        // where intendedMargin is the sum of notional / leverage for open positions
        clearingHouse.assertPostWithdrawalMarginRequired({
            assets: cache.assets,
            positions: cache.positions,
            margin: remainingMargin
        });

        // set position update (note: this will just be the new position.lastCumulativeFunding)
        clearingHouse.setPositions({
            tradedAsset: "",
            account: account,
            subaccount: subaccount,
            assets: cache.assets,
            positions: cache.positions
        });

        emit MarginRemoved(account, subaccount, amount, remainingMargin, StorageLib.incNonce());
    }

    function setPositionLeverage(bytes32 asset, address account, uint256 subaccount, uint256 newLeverage)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.SET_LEVERAGE)
        returns (int256 collateralDelta)
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();
        Market storage market = clearingHouse.market[asset];

        MarketLib.assertActive(asset);
        MarketLib.assertMaxLeverage(asset, newLeverage);

        __UpdateLeverageCache__ memory cache;

        // handle collateral delta for book oi
        cache.currentLeverage = market.getPositionLeverage(account, subaccount);
        cache.orderbookNotional = market.orderbookNotional[account][subaccount];

        cache.newOrderbookMargin = cache.orderbookNotional.fullMulDiv(1e18, newLeverage);
        cache.currentOrderbookMargin = cache.orderbookNotional.fullMulDiv(1e18, cache.currentLeverage);

        cache.collateralDeltaFromBook = cache.newOrderbookMargin.toInt256() - cache.currentOrderbookMargin.toInt256();

        // set new leverage before loading account
        market.position[account][subaccount].leverage = newLeverage;

        // empty position
        if (market.position[account][subaccount].amount == 0) {
            StorageLib.loadCollateralManager().handleCollateralDelta({
                account: account,
                collateralDelta: cache.collateralDeltaFromBook
            });

            // margin doesn't change on leverage update for empty positions
            int256 margin = StorageLib.loadCollateralManager().getMarginBalance(account, subaccount);

            emit PositionLeverageSet(
                asset, account, subaccount, newLeverage, cache.collateralDeltaFromBook, margin, StorageLib.incNonce()
            );

            return cache.collateralDeltaFromBook;
        }

        // load account
        (cache.assets, cache.positions) = clearingHouse.getAccount(account, subaccount);

        // realize funding payment
        cache.fundingPayment = ClearingHouseLib.realizeFundingPayment(cache.assets, cache.positions);

        cache.newMargin = clearingHouse.getIntendedMargin(cache.assets, cache.positions);

        // assert open margin requirement met
        clearingHouse.assertOpenMarginRequired({
            assets: cache.assets,
            positions: cache.positions,
            margin: cache.newMargin.toInt256()
        });

        clearingHouse.setPositions({
            tradedAsset: "",
            account: account,
            subaccount: subaccount,
            assets: cache.assets,
            positions: cache.positions
        });

        // settle delta between new and prev margin & new and prev orderbook collateral
        collateralDelta = StorageLib.loadCollateralManager().settleNewLeverage({
            account: account,
            subaccount: subaccount,
            collateralDeltaFromBook: cache.collateralDeltaFromBook,
            newMargin: cache.newMargin.toInt256(),
            fundingPayment: cache.fundingPayment
        });

        emit PositionLeverageSet(
            asset, account, subaccount, newLeverage, collateralDelta, cache.newMargin.toInt256(), StorageLib.incNonce()
        );
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                              ORDER PLACE
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function placeOrder(address account, PlaceOrderArgs calldata args)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.PLACE_ORDER)
        onlyActiveProtocol
        returns (PlaceOrderResult memory result)
    {
        return StorageLib.loadClearingHouse().placeOrder(account, args, BookType.STANDARD);
    }

    function postLimitOrderBackstop(address account, PlaceOrderArgs calldata args)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.PLACE_ORDER)
        onlyActiveProtocol
        returns (PlaceOrderResult memory result)
    {
        if (args.tif != TiF.MOC) revert InvalidBackstopLimitOrder();

        return StorageLib.loadClearingHouse().placeOrder(account, args, BookType.BACKSTOP);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                          ORDER AMEND / CANCEL
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function amendLimitOrder(address account, AmendLimitOrderArgs calldata args)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.PLACE_ORDER)
        onlyActiveProtocol
        returns (int256 collateralDelta)
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        collateralDelta = clearingHouse.market[args.asset].amendLimitOrder(account, args, BookType.STANDARD);

        StorageLib.loadCollateralManager().handleCollateralDelta({account: account, collateralDelta: collateralDelta});
    }

    function cancelLimitOrders(bytes32 asset, address account, uint256 subaccount, uint256[] calldata orderIds)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.PLACE_ORDER)
        onlyActiveProtocol
        returns (uint256 refund)
    {
        refund = CLOBLib.cancel(asset, account, subaccount, orderIds, BookType.STANDARD);

        StorageLib.loadCollateralManager().handleCollateralDelta({account: account, collateralDelta: -refund.toInt256()});
    }

    function amendLimitOrderBackstop(address account, AmendLimitOrderArgs calldata args)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.PLACE_ORDER)
        onlyActiveProtocol
        returns (int256 collateralDelta)
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        collateralDelta = clearingHouse.market[args.asset].amendLimitOrder(account, args, BookType.BACKSTOP);

        StorageLib.loadCollateralManager().handleCollateralDelta({account: account, collateralDelta: collateralDelta});
    }

    function cancelLimitOrdersBackstop(bytes32 asset, address account, uint256 subaccount, uint256[] calldata orderIds)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.PLACE_ORDER)
        onlyActiveProtocol
        returns (uint256 refund)
    {
        refund = CLOBLib.cancel(asset, account, subaccount, orderIds, BookType.BACKSTOP);

        StorageLib.loadCollateralManager().handleCollateralDelta({account: account, collateralDelta: -refund.toInt256()});
    }

    function cancelConditionalOrders(address account, uint256[] calldata nonces)
        external
        onlySenderOrOperator(account, PerpsOperatorRoles.PLACE_ORDER)
        onlyActiveProtocol
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        for (uint256 i; i < nonces.length; i++) {
            clearingHouse.nonceUsed[account][nonces[i]] = true;
        }
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               HELPER
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _getCollateral(uint256 baseAmount, uint256 price, uint256 leverage)
        private
        pure
        returns (uint256 collateral)
    {
        collateral = baseAmount.fullMulDiv(price, 1e18).fullMulDiv(1e18, leverage);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {IOperatorPanel} from "../interfaces/IOperatorPanel.sol";
import {SpotOperatorRoles, PerpsOperatorRoles, OperatorStorage} from "../OperatorPanel.sol";

library OperatorHelperLib {
    /// @dev sig: 0x732ea322
    error OperatorDoesNotHaveRole();

    function assertHasRole(uint256 rolesPacked, uint8 role) internal pure {
        if (rolesPacked & 1 << role == 0 && rolesPacked & 1 == 0) revert OperatorDoesNotHaveRole();
    }

    /// @dev Performs operator check with both operator and router bypass
    function onlySenderOrOperator(
        IOperatorPanel operator,
        address gteRouter,
        address account,
        SpotOperatorRoles requiredRole
    ) internal view {
        if (msg.sender == account || msg.sender == gteRouter) return;

        uint256 rolesPacked = operator.getOperatorRoleApprovals(account, msg.sender);
        assertHasRole(rolesPacked, uint8(requiredRole));
    }

    /// @dev Performs operator check with just operator
    function onlySenderOrOperator(IOperatorPanel operator, address account, SpotOperatorRoles requiredRole)
        internal
        view
    {
        if (msg.sender == account) return;

        uint256 rolesPacked = operator.getOperatorRoleApprovals(account, msg.sender);
        assertHasRole(rolesPacked, uint8(requiredRole));
    }

    /// @dev Performs spot operator check with storage directly (for contracts inheriting Operator)
    function onlySenderOrOperator(
        OperatorStorage storage self,
        address gteRouter,
        address account,
        SpotOperatorRoles requiredRole
    ) internal view {
        if (msg.sender == account || msg.sender == gteRouter) return;

        uint256 rolesPacked = self.operatorRoleApprovals[account][msg.sender];
        assertHasRole(rolesPacked, uint8(requiredRole));
    }

    /// @dev Performs perps operator check with storage directly (for contracts inheriting Operator)
    function onlySenderOrOperator(OperatorStorage storage self, address account, PerpsOperatorRoles requiredRole)
        internal
        view
    {
        if (msg.sender == account) return;

        uint256 rolesPacked = self.operatorRoleApprovals[account][msg.sender];
        assertHasRole(rolesPacked, uint8(requiredRole));
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
pragma solidity ^0.8.27;

import {ICLOB} from "../ICLOB.sol";

struct MakerCredit {
    address maker;
    uint256 quoteAmount;
    uint256 baseAmount;
}

// slither-disable-start assembly
library TransientMakerData {
    bytes32 constant TRANSIENT_MAKERS_POSITION =
        keccak256(abi.encode(uint256(keccak256("TransientMakers")) - 1)) & ~bytes32(uint256(0xff));
    bytes32 constant TRANSIENT_CREDITS_POSITION =
        keccak256(abi.encode(uint256(keccak256("TransientCredits")) - 1)) & ~bytes32(uint256(0xff));

    /// @dev sig: 0xe47ec074
    error ArithmeticOverflow();

    /// @dev Adds a quote token to the transient maker data
    function addQuoteToken(address maker, uint256 quoteAmount) internal {
        bytes32 slot = keccak256(abi.encode(TRANSIENT_CREDITS_POSITION, maker));
        bytes4 err = ArithmeticOverflow.selector;

        bool exists;
        assembly ("memory-safe") {
            exists := iszero(iszero(tload(slot)))

            if iszero(exists) { tstore(slot, 1) }

            let balSlot := add(slot, 1)

            let oldVal := tload(balSlot)
            let newVal := add(oldVal, quoteAmount)

            if lt(newVal, oldVal) {
                mstore(0x00, err)
                revert(0x00, 0x04)
            }

            tstore(balSlot, newVal)
        }

        if (!exists) _addMaker(maker);
    }

    /// @dev Adds a base token to the transient maker data
    function addBaseToken(address maker, uint256 baseAmount) internal {
        bytes32 slot = keccak256(abi.encode(TRANSIENT_CREDITS_POSITION, maker));
        bytes4 err = ArithmeticOverflow.selector;

        bool exists;
        assembly ("memory-safe") {
            exists := iszero(iszero(tload(slot)))

            if iszero(exists) { tstore(slot, 1) }

            let balSlot := add(slot, 2)

            let oldVal := tload(balSlot)
            let newVal := add(oldVal, baseAmount)

            if lt(newVal, oldVal) {
                mstore(0x00, err)
                revert(0x00, 0x04)
            }

            tstore(balSlot, newVal)
        }

        if (!exists) _addMaker(maker);
    }

    /// @dev Gets the maker credits and clears the storage
    function getMakerCreditsAndClearStorage() internal returns (MakerCredit[] memory makerCredits) {
        address[] memory makers = _getMakersAndClear();

        uint256 length = makers.length;

        makerCredits = new MakerCredit[](length);

        uint256 quoteAmount;
        uint256 baseAmount;

        for (uint256 i; i < length; i++) {
            (quoteAmount, baseAmount) = _getBalancesAndClear(makers[i]);

            makerCredits[i] = MakerCredit({maker: makers[i], quoteAmount: quoteAmount, baseAmount: baseAmount});
        }
    }

    function _addMaker(address maker) internal {
        bytes32 slot = TRANSIENT_MAKERS_POSITION;

        assembly ("memory-safe") {
            let len := tload(slot)

            mstore(0x00, slot)

            let dataSlot := keccak256(0x00, 0x20)

            tstore(add(dataSlot, len), maker)
            tstore(slot, add(len, 1))
        }
    }

    function _getMakersAndClear() internal returns (address[] memory makers) {
        bytes32 slot = TRANSIENT_MAKERS_POSITION;

        assembly ("memory-safe") {
            let len := tload(slot)

            makers := mload(0x40)

            mstore(makers, len)

            mstore(0x00, slot)

            let dataSlot := keccak256(0x00, 0x20)
            let memPointer := add(makers, 0x20)

            for { let i := 0 } lt(i, len) { i := add(i, 1) } {
                mstore(add(memPointer, mul(i, 0x20)), tload(add(dataSlot, i)))
                tstore(add(dataSlot, i), 0) // clear maker
            }

            mstore(0x40, add(memPointer, mul(len, 0x20))) // idk the purpose of this tbh
            tstore(slot, 0) // clear length
        }
    }

    function _getBalancesAndClear(address maker) internal returns (uint256 quoteAmount, uint256 baseAmount) {
        bytes32 slot = keccak256(abi.encode(TRANSIENT_CREDITS_POSITION, maker));

        assembly ("memory-safe") {
            let quote := add(slot, 1)
            let base := add(slot, 2)
            let instant := 0
            let account := 1

            quoteAmount := tload(quote)
            baseAmount := tload(base)

            tstore(slot, 0)
            tstore(quote, 0)
            tstore(base, 0)
        }
    }
}
// slither-disable-end assembly

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

import {OwnableRoles} from "@solady/auth/OwnableRoles.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";
import {Initializable} from "@solady/utils/Initializable.sol";
import {SignatureCheckerLib} from "@solady/utils/SignatureCheckerLib.sol";

import {Status, Side, TradeType, FeeTier, BookType} from "../types/Enums.sol";
import {
    MarketParams,
    PlaceOrderArgs,
    PlaceOrderResult,
    PositionUpdateResult,
    LiquidateData,
    BackstopLiquidateData,
    TradeExecutedData,
    DeleveragePair,
    LiquidatorData,
    Condition,
    SignData,
    FundingPaymentResult,
    LiquidateeSettleData,
    Account
} from "../types/Structs.sol";
import {Constants} from "../types/Constants.sol";

import {BackstopLiquidatorDataLib} from "../types/BackstopLiquidatorDataLib.sol";
import {StorageLib} from "../types/StorageLib.sol";
import {CLOBLib} from "../types/CLOBLib.sol";
import {ClearingHouse, StorageLib} from "../types/ClearingHouse.sol";
import {FeeManager} from "../types/FeeManager.sol";
import {Market, MarketSettings, MarketMetadata} from "../types/Market.sol";
import {FundingRateSettings} from "../types/FundingRateEngine.sol";
import {Position} from "../types/Position.sol";
import {Book, BookSettings} from "../types/Book.sol";

abstract contract AdminPanel is OwnableRoles, Initializable {
    using FixedPointMathLib for *;
    using SafeCastLib for *;
    using SignatureCheckerLib for address;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                EVENTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    event MarketCreated(
        bytes32 indexed asset,
        MarketSettings marketSettings,
        FundingRateSettings fundingSettings,
        BookSettings bookSettings,
        uint256 lotSize,
        uint256 initialPrice,
        uint256 nonce
    );

    event FeeTierUpdated(address indexed account, FeeTier indexed feeTier, uint256 nonce);

    event ProtocolActivated(uint256 nonce);
    event ProtocolDeactivated(uint256 nonce);
    event TakerFeeRatesUpdated(uint16[] takerFeeRates, uint256 nonce);
    event MakerFeeRatesUpdated(uint16[] makerFeeRates, uint256 nonce);

    event MarketStatusUpdated(bytes32 indexed asset, Status status, uint256 nonce);
    event CrossMarginEnabled(bytes32 indexed asset, uint256 nonce);
    event CrossMarginDisabled(bytes32 indexed asset, uint256 nonce);
    event MaxLeverageUpdated(bytes32 indexed asset, uint256 maxOpenLeverage, uint256 nonce);
    event MaintenanceMarginRatioUpdated(bytes32 indexed asset, uint256 maintenanceMarginRatio, uint256 nonce);
    event LiquidationFeeRateUpdated(bytes32 indexed asset, uint256 liquidationFeeRate, uint256 nonce);
    event FundingIntervalUpdated(bytes32 indexed asset, uint256 fundingInterval, uint256 resetInterval, uint256 nonce);
    event ResetIterationsUpdated(bytes32 indexed asset, uint256 resetIterations, uint256 nonce);
    event FundingClampsUpdated(bytes32 indexed asset, uint256 innerClamp, uint256 outerClamp, uint256 nonce);
    event InterestRateUpdated(bytes32 indexed asset, int256 interestRate, uint256 nonce);
    event DivergenceCapUpdated(bytes32 indexed asset, uint256 divergenceCap, uint256 nonce);
    event ReduceOnlyCapUpdated(bytes32 indexed asset, uint256 reduceOnlyCap, uint256 nonce);
    event PartialLiquidationThresholdUpdated(bytes32 indexed asset, uint256 partialLiquidationThreshold, uint256 nonce);
    event PartialLiquidationRateUpdated(bytes32 indexed asset, uint256 partialLiquidationRate, uint256 nonce);
    event MaxNumOrdersUpdated(bytes32 indexed asset, uint256 maxNumOrders, uint256 nonce);
    event MaxLimitsPerTxUpdated(bytes32 indexed asset, uint8 maxLimitsPerTx, uint256 nonce);
    event MinLimitOrderAmountInBaseUpdated(bytes32 indexed asset, uint256 minLimitOrderAmountInBase, uint256 nonce);
    event TickSizeUpdated(bytes32 indexed asset, uint256 tickSize, uint256 nonce);

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ERRORS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    error MarketNotFound();
    error BookNotFound();
    error InvalidNonce();
    error ConditionNotMet();
    error OrderExpired();
    error InvalidSignature();
    error MarketAlreadyInitialized();
    error ProtocolNotActive();
    error ProtocolAlreadyActive();
    error ProtocolAlreadyInactive();
    error CannotActivateMarket();
    error CannotDeactivateMarket();
    error CannotDelistMarket();
    error CannotRelistMarket();
    error CrossMarginAlreadyEnabled();
    error CrossMarginAlreadyDisabled();
    error InvalidSettings();

    modifier onlyAdmin() {
        _checkRolesOrOwner(Constants.ADMIN_ROLE);
        _;
    }

    modifier onlyActiveProtocol() virtual {
        if (!StorageLib.loadClearingHouse().active) revert ProtocolNotActive();
        _;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                             INITIALIZATION
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function initialize(address owner_, uint16[] calldata takerFees, uint16[] calldata makerFees)
        external
        initializer
    {
        FeeManager storage feeManager = StorageLib.loadFeeManager();

        _assertNonZero(uint160(owner_));
        _assertNonZero(takerFees.length);
        _assertNonZero(makerFees.length);
        _assertNonZero(takerFees[0]);
        _assertNonZero(makerFees[0]);

        _initializeOwner(owner_);

        feeManager.setTakerFeeRates(takerFees);
        feeManager.setMakerFeeRates(makerFees);

        emit TakerFeeRatesUpdated(takerFees, StorageLib.incNonce());
        emit MakerFeeRatesUpdated(makerFees, StorageLib.incNonce());
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            MARKET CREATION
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function createMarket(bytes32 asset, MarketParams calldata params) external onlyAdmin {
        Market storage market = StorageLib.loadMarket(asset);

        if (market.exists()) revert MarketAlreadyInitialized();

        _validateLeverage(params.maxOpenLeverage);
        _validateMaintenanceMarginRatio(params.maintenanceMarginRatio, params.maxOpenLeverage);
        _assertDecimal(params.liquidationFeeRate);
        _validateFundingInterval(params.fundingInterval, params.resetInterval);
        _assertNonZero(params.resetIterations);
        _assertDecimal(params.interestRate.abs());
        _assertDecimal(params.divergenceCap);
        _assertDecimal(params.partialLiquidationRate);
        _assertNonZero(params.partialLiquidationThreshold);
        _validateMinBookValue(params.lotSize, params.tickSize);
        _validateConform(params.minLimitOrderAmountInBase, params.lotSize);
        _assertNonZero(params.maxLimitsPerTx);

        MarketSettings memory marketSettings = MarketSettings({
            status: Status.INACTIVE,
            maxOpenLeverage: params.maxOpenLeverage,
            maintenanceMarginRatio: params.maintenanceMarginRatio,
            liquidationFeeRate: params.liquidationFeeRate,
            divergenceCap: params.divergenceCap,
            reduceOnlyCap: params.reduceOnlyCap,
            partialLiquidationThreshold: params.partialLiquidationThreshold,
            partialLiquidationRate: params.partialLiquidationRate,
            crossMarginEnabled: params.crossMarginEnabled
        });

        FundingRateSettings memory fundingSettings = FundingRateSettings({
            fundingInterval: params.fundingInterval,
            resetInterval: params.resetInterval,
            resetIterations: params.resetIterations,
            innerClamp: params.innerClamp,
            outerClamp: params.outerClamp,
            interestRate: params.interestRate
        });

        BookSettings memory bookSettings = BookSettings({
            maxNumOrders: params.maxNumOrders,
            maxLimitsPerTx: params.maxLimitsPerTx,
            minLimitOrderAmountInBase: params.minLimitOrderAmountInBase,
            tickSize: params.tickSize
        });

        market.init({
            asset: asset,
            marketSettings: marketSettings,
            fundingSettings: fundingSettings,
            initialPrice: params.initialPrice
        });

        CLOBLib.init(asset, bookSettings, params.lotSize);

        emit MarketCreated(
            asset, marketSettings, fundingSettings, bookSettings, params.initialPrice, params.lotSize, StorageLib.incNonce()
        );
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                              MAINTENANCE
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function setMarkPrice(bytes32 asset, uint256 indexPrice) external onlyAdmin {
        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        market.setMarkPrice(indexPrice);
    }

    function settleFunding(bytes32 asset) external onlyAdmin {
        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        market.settleFunding();
    }

    function setFeeTiers(address[] calldata accounts, FeeTier[] calldata feeTiers) external onlyAdmin {
        if (accounts.length != feeTiers.length) revert InvalidSettings();

        address account;
        FeeTier feeTier;
        for (uint256 i; i < accounts.length; ++i) {
            account = accounts[i];
            feeTier = feeTiers[i];

            StorageLib.loadFeeManager().setAccountFeeTier(account, feeTier);

            emit FeeTierUpdated(account, feeTier, StorageLib.incNonce());
        }
    }

    function setLiquidatorPoints(address account, uint256 points) external onlyAdmin {
        StorageLib.loadClearingHouse().liquidatorPoints[account] = points;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                 AUTH
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function grantAdmin(address account) external onlyOwner {
        _grantRoles(account, Constants.ADMIN_ROLE);
    }

    function revokeAdmin(address account) external onlyOwner {
        _removeRoles(account, Constants.ADMIN_ROLE);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                             INSURANCE FUND
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function insuranceFundDeposit(uint256 amount) external onlyOwner {
        StorageLib.loadInsuranceFund().deposit(amount);
    }

    function insuranceFundWithdraw(uint256 amount) external onlyOwner {
        StorageLib.loadInsuranceFund().withdraw(amount);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                           CONDITIONAL ORDERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function placeTwapOrder(address account, PlaceOrderArgs calldata args, SignData calldata signData)
        external
        onlyAdmin
        onlyActiveProtocol
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        _validateSig(clearingHouse, account, abi.encode(args), signData);

        clearingHouse.nonceUsed[account][signData.nonce] = true;

        clearingHouse.placeOrder(account, args, BookType.STANDARD);
    }

    function placeTPSLOrder(
        address account,
        PlaceOrderArgs calldata args,
        Condition calldata condition,
        SignData calldata signData
    ) external onlyAdmin onlyActiveProtocol {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        _validateSig(clearingHouse, account, abi.encode(args, condition), signData);

        clearingHouse.nonceUsed[account][signData.nonce] = true;

        if (!clearingHouse.market[args.asset].isTPSLConditionMet(args.side, condition)) revert ConditionNotMet();

        clearingHouse.placeOrder(account, args, BookType.STANDARD);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                           PROTOCOL SETTINGS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function setTakerFeeRates(uint16[] calldata takerFeeRates) external onlyAdmin {
        StorageLib.loadFeeManager().setTakerFeeRates(takerFeeRates);

        emit TakerFeeRatesUpdated(takerFeeRates, StorageLib.incNonce());
    }

    function setMakerFeeRates(uint16[] calldata makerFeeRates) external onlyAdmin {
        StorageLib.loadFeeManager().setMakerFeeRates(makerFeeRates);

        emit MakerFeeRatesUpdated(makerFeeRates, StorageLib.incNonce());
    }

    function activateProtocol() external onlyAdmin {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        if (clearingHouse.active) revert ProtocolAlreadyActive();

        clearingHouse.active = true;

        emit ProtocolActivated(StorageLib.incNonce());
    }

    function deactivateProtocol() external onlyAdmin {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        if (!clearingHouse.active) revert ProtocolAlreadyInactive();

        clearingHouse.active = false;

        emit ProtocolDeactivated(StorageLib.incNonce());
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            MARKET SETTINGS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice actives market of 'asset'
    /// @dev reverts if market is not INACTIVE
    function activateMarket(bytes32 asset) external onlyAdmin {
        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        if (StorageLib.loadMarketSettings(asset).status != Status.INACTIVE) revert CannotActivateMarket();

        StorageLib.loadMarketSettings(asset).status = Status.ACTIVE;

        emit MarketStatusUpdated(asset, Status.ACTIVE, StorageLib.incNonce());
    }

    /// @notice deactivates market of 'asset'
    /// @dev reverts if market is not ACTIVE
    function deactivateMarket(bytes32 asset) external onlyAdmin {
        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        if (StorageLib.loadMarketSettings(asset).status != Status.ACTIVE) revert CannotDeactivateMarket();

        StorageLib.loadMarketSettings(asset).status = Status.INACTIVE;

        emit MarketStatusUpdated(asset, Status.INACTIVE, StorageLib.incNonce());
    }

    /// @notice delists market of 'asset'
    /// @dev reverts if market is not INACTIVE
    function delistMarket(bytes32 asset) external onlyAdmin {
        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        if (StorageLib.loadMarketSettings(asset).status != Status.INACTIVE) revert CannotDelistMarket();

        StorageLib.loadMarketSettings(asset).status = Status.DELISTED;

        emit MarketStatusUpdated(asset, Status.DELISTED, StorageLib.incNonce());
    }

    /// @notice relists market of 'asset'
    /// @dev reverts if market is not DELISTED
    /// @dev reverts position oi and book oi are not cleared
    function relistMarket(bytes32 asset) external onlyAdmin {
        Market storage market = StorageLib.loadMarket(asset);
        MarketMetadata storage marketMetadata = StorageLib.loadMarketMetadata(asset);
        Book storage book = StorageLib.loadBook(asset);

        if (!market.exists()) revert MarketNotFound();
        if (!book.exists()) revert BookNotFound();

        if (StorageLib.loadMarketSettings(asset).status != Status.DELISTED) revert CannotRelistMarket();

        // book oi must be cleared
        if (book.metadata.baseOI + book.metadata.quoteOI > 0) revert CannotRelistMarket();

        // position oi must be cleared
        if (marketMetadata.longOI + marketMetadata.shortOI > 0) revert CannotRelistMarket();

        StorageLib.loadMarketSettings(asset).status = Status.INACTIVE;

        emit MarketStatusUpdated(asset, Status.INACTIVE, StorageLib.incNonce());
    }

    function enableCrossMargin(bytes32 asset) external onlyAdmin {
        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        if (StorageLib.loadMarketSettings(asset).crossMarginEnabled) revert CrossMarginAlreadyEnabled();

        StorageLib.loadMarketSettings(asset).crossMarginEnabled = true;

        emit CrossMarginEnabled(asset, StorageLib.incNonce());
    }

    function disableCrossMargin(bytes32 asset) external onlyAdmin {
        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        if (!StorageLib.loadMarketSettings(asset).crossMarginEnabled) revert CrossMarginAlreadyDisabled();

        StorageLib.loadMarketSettings(asset).crossMarginEnabled = false;

        emit CrossMarginDisabled(asset, StorageLib.incNonce());
    }

    /// @notice sets the maximum leverage for a market of 'asset'
    /// @param maxOpenLeverage the maximum leverage to set, must be between 1x and 100x (1e18 to 100e18)
    function setMaxLeverage(bytes32 asset, uint256 maxOpenLeverage) external onlyAdmin {
        _validateLeverage(maxOpenLeverage);

        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        StorageLib.loadMarketSettings(asset).maxOpenLeverage = maxOpenLeverage;

        emit MaxLeverageUpdated(asset, maxOpenLeverage, StorageLib.incNonce());
    }

    function setMinMarginRatio(bytes32 asset, uint256 maintenanceMarginRatio) external onlyAdmin {
        Market storage market = StorageLib.loadMarket(asset);
        MarketSettings storage settings = StorageLib.loadMarketSettings(asset);

        if (!market.exists()) revert MarketNotFound();

        _validateMaintenanceMarginRatio(maintenanceMarginRatio, settings.maxOpenLeverage);

        settings.maintenanceMarginRatio = maintenanceMarginRatio;

        emit MaintenanceMarginRatioUpdated(asset, maintenanceMarginRatio, StorageLib.incNonce());
    }

    /// @notice sets the liquidation fee rate for a market of 'asset'
    function setLiquidationFeeRate(bytes32 asset, uint256 liquidationFeeRate) external onlyAdmin {
        _assertDecimal(liquidationFeeRate);

        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        StorageLib.loadMarketSettings(asset).liquidationFeeRate = liquidationFeeRate;

        emit LiquidationFeeRateUpdated(asset, liquidationFeeRate, StorageLib.incNonce());
    }

    /// @notice sets max divergence between tradable price & mark
    /// @param divergenceCap e.g. .5 for 50% divergence from mark
    function setDivergenceCap(bytes32 asset, uint256 divergenceCap) external onlyAdmin {
        _assertDecimal(divergenceCap);

        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        StorageLib.loadMarketSettings(asset).divergenceCap = divergenceCap;

        emit DivergenceCapUpdated(asset, divergenceCap, StorageLib.incNonce());
    }

    /// @notice sets max number of reduce-only orders that can be placed
    function setReduceOnlyCap(bytes32 asset, uint256 reduceOnlyCap) external onlyAdmin {
        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        StorageLib.loadMarketSettings(asset).reduceOnlyCap = reduceOnlyCap;

        emit ReduceOnlyCapUpdated(asset, reduceOnlyCap, StorageLib.incNonce());
    }

    /// @notice sets the value threshold where a position must be partially liquidated
    /// @param partialLiquidationThreshold value in quote
    function setPartialLiquidationThreshold(bytes32 asset, uint256 partialLiquidationThreshold) external onlyAdmin {
        _assertNonZero(partialLiquidationThreshold);

        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        StorageLib.loadMarketSettings(asset).partialLiquidationThreshold = partialLiquidationThreshold;

        emit PartialLiquidationThresholdUpdated(asset, partialLiquidationThreshold, StorageLib.incNonce());
    }

    /// @notice sets the rate at which a position is partially liquidated
    /// @param partialLiquidationRate percentage of position to liquidate, e.g. 0.1 for 10%
    function setPartialLiquidationRate(bytes32 asset, uint256 partialLiquidationRate) external onlyAdmin {
        _assertDecimal(partialLiquidationRate);

        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        StorageLib.loadMarketSettings(asset).partialLiquidationRate = partialLiquidationRate;

        emit PartialLiquidationRateUpdated(asset, partialLiquidationRate, StorageLib.incNonce());
    }

    /// @notice sets the funding interval for a market of 'asset'
    function setFundingInterval(bytes32 asset, uint256 fundingInterval, uint256 resetInterval) external onlyAdmin {
        _assertNonZero(fundingInterval);

        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        FundingRateSettings storage settings = StorageLib.loadFundingRateSettings(asset);

        _validateFundingInterval(fundingInterval, resetInterval);

        settings.fundingInterval = fundingInterval;
        settings.resetInterval = resetInterval;

        emit FundingIntervalUpdated(asset, fundingInterval, resetInterval, StorageLib.incNonce());
    }

    function setResetIterations(bytes32 asset, uint256 resetIterations) external onlyAdmin {
        _assertNonZero(resetIterations);

        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        StorageLib.loadFundingRateSettings(asset).resetIterations = resetIterations;

        emit ResetIterationsUpdated(asset, resetIterations, StorageLib.incNonce());
    }

    /// @notice sets clamp range for funding rate
    function setFundingClamps(bytes32 asset, uint256 innerClamp, uint256 outerClamp) external onlyAdmin {
        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        FundingRateSettings storage settings = StorageLib.loadFundingRateSettings(asset);

        settings.innerClamp = innerClamp;
        settings.outerClamp = outerClamp;

        emit FundingClampsUpdated(asset, innerClamp, outerClamp, StorageLib.incNonce());
    }

    function setInterestRate(bytes32 asset, int256 interestRate) external onlyAdmin {
        _assertDecimal(interestRate.abs());

        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        StorageLib.loadFundingRateSettings(asset).interestRate = interestRate;

        emit InterestRateUpdated(asset, interestRate, StorageLib.incNonce());
    }

    function setMaxNumOrders(bytes32 asset, uint256 maxNumOrders) external onlyAdmin {
        _assertNonZero(maxNumOrders);

        Book storage book = StorageLib.loadBook(asset);

        if (!book.exists()) revert BookNotFound();

        StorageLib.loadBookSettings(asset).maxNumOrders = maxNumOrders;

        emit MaxNumOrdersUpdated(asset, maxNumOrders, StorageLib.incNonce());
    }

    function setMaxLimitsPerTx(bytes32 asset, uint8 maxLimitsPerTx) external onlyAdmin {
        _assertNonZero(maxLimitsPerTx);

        Book storage book = StorageLib.loadBook(asset);

        if (!book.exists()) revert BookNotFound();

        StorageLib.loadBookSettings(asset).maxLimitsPerTx = maxLimitsPerTx;

        emit MaxLimitsPerTxUpdated(asset, maxLimitsPerTx, StorageLib.incNonce());
    }

    function setMinLimitOrderAmountInBase(bytes32 asset, uint256 minLimitOrderAmountInBase) external onlyAdmin {
        Book storage book = StorageLib.loadBook(asset);

        if (!book.exists()) revert BookNotFound();

        _validateConform(minLimitOrderAmountInBase, book.config.lotSize);

        StorageLib.loadBookSettings(asset).minLimitOrderAmountInBase = minLimitOrderAmountInBase;

        emit MinLimitOrderAmountInBaseUpdated(asset, minLimitOrderAmountInBase, StorageLib.incNonce());
    }

    function setTickSize(bytes32 asset, uint256 tickSize) external onlyAdmin {
        Book storage book = StorageLib.loadBook(asset);

        if (!book.exists()) revert BookNotFound();

        _validateMinBookValue(StorageLib.loadBookSettings(asset).minLimitOrderAmountInBase, tickSize);
        _validateMinBookValue(book.config.lotSize, tickSize);

        StorageLib.loadBookSettings(asset).tickSize = tickSize;

        emit TickSizeUpdated(asset, tickSize, StorageLib.incNonce());
    }

    function setMarketSettings(bytes32 asset, MarketSettings calldata settings) external onlyAdmin {
        Market storage market = StorageLib.loadMarket(asset);
        MarketSettings storage marketSettings = StorageLib.loadMarketSettings(asset);

        if (!market.exists()) revert MarketNotFound();

        if (marketSettings.status != settings.status) revert InvalidSettings();

        _validateLeverage(settings.maxOpenLeverage);
        _validateMaintenanceMarginRatio(settings.maintenanceMarginRatio, settings.maxOpenLeverage);
        _assertDecimal(settings.liquidationFeeRate);
        _assertDecimal(settings.divergenceCap);
        _assertNonZero(settings.partialLiquidationThreshold);
        _assertDecimal(settings.partialLiquidationRate);

        marketSettings.init(settings);

        if (settings.crossMarginEnabled) emit CrossMarginEnabled(asset, StorageLib.incNonce());
        else emit CrossMarginDisabled(asset, StorageLib.incNonce());

        emit MaxLeverageUpdated(asset, settings.maxOpenLeverage, StorageLib.incNonce());
        emit MaintenanceMarginRatioUpdated(asset, settings.maintenanceMarginRatio, StorageLib.incNonce());
        emit LiquidationFeeRateUpdated(asset, settings.liquidationFeeRate, StorageLib.incNonce());
        emit DivergenceCapUpdated(asset, settings.divergenceCap, StorageLib.incNonce());
        emit ReduceOnlyCapUpdated(asset, settings.reduceOnlyCap, StorageLib.incNonce());
        emit PartialLiquidationThresholdUpdated(asset, settings.partialLiquidationThreshold, StorageLib.incNonce());
        emit PartialLiquidationRateUpdated(asset, settings.partialLiquidationRate, StorageLib.incNonce());
    }

    function setFundingRateSettings(bytes32 asset, FundingRateSettings calldata settings) external onlyAdmin {
        Market storage market = StorageLib.loadMarket(asset);

        if (!market.exists()) revert MarketNotFound();

        _validateFundingInterval(settings.fundingInterval, settings.resetInterval);
        _assertNonZero(settings.resetIterations);
        _assertDecimal(settings.interestRate.abs());

        StorageLib.loadFundingRateSettings(asset).init(settings);

        emit FundingIntervalUpdated(asset, settings.fundingInterval, settings.resetInterval, StorageLib.incNonce());
        emit ResetIterationsUpdated(asset, settings.resetIterations, StorageLib.incNonce());
        emit FundingClampsUpdated(asset, settings.innerClamp, settings.outerClamp, StorageLib.incNonce());
        emit InterestRateUpdated(asset, settings.interestRate, StorageLib.incNonce());
    }

    function setBookSettings(bytes32 asset, BookSettings calldata settings) external onlyAdmin {
        _validateMinBookValue(settings.minLimitOrderAmountInBase, settings.tickSize);
        _assertNonZero(settings.maxLimitsPerTx);
        _assertNonZero(settings.maxNumOrders);

        if (!StorageLib.loadBook(asset).exists()) revert BookNotFound();

        BookSettings storage bookSettings = StorageLib.loadBookSettings(asset);

        bookSettings.maxNumOrders = settings.maxNumOrders;
        bookSettings.maxLimitsPerTx = settings.maxLimitsPerTx;
        bookSettings.minLimitOrderAmountInBase = settings.minLimitOrderAmountInBase;
        bookSettings.tickSize = settings.tickSize;

        emit MaxNumOrdersUpdated(asset, settings.maxNumOrders, StorageLib.incNonce());
        emit MaxLimitsPerTxUpdated(asset, settings.maxLimitsPerTx, StorageLib.incNonce());
        emit MinLimitOrderAmountInBaseUpdated(asset, settings.minLimitOrderAmountInBase, StorageLib.incNonce());
        emit TickSizeUpdated(asset, settings.tickSize, StorageLib.incNonce());
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                HELPERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _validateSig(
        ClearingHouse storage clearingHouse,
        address signer,
        bytes memory order,
        SignData calldata signData
    ) internal view {
        if (clearingHouse.nonceUsed[signer][signData.nonce]) revert InvalidNonce();
        if (signData.expiry < block.timestamp) revert OrderExpired();

        bytes32 hash = keccak256(bytes.concat(order, abi.encode(signData.expiry, signData.nonce)));

        if (!signer.isValidSignatureNowCalldata(hash, signData.sig)) revert InvalidSignature();
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                              ASSERTIONS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _validateMinBookValue(uint256 minLimitOrderAmountInBase, uint256 tickSize) private pure {
        if (minLimitOrderAmountInBase == 0 || tickSize == 0) revert InvalidSettings();
        _assertNonZero(minLimitOrderAmountInBase.fullMulDiv(tickSize, 1e18));
    }

    function _validateConform(uint256 value, uint256 standard) private pure {
        _assertNonZero(value);
        if (value % standard != 0) revert InvalidSettings();
    }

    function _validateLeverage(uint256 maxOpenLeverage) private pure {
        if (maxOpenLeverage < 1e18 || maxOpenLeverage > 100e18) revert InvalidSettings();
    }

    function _validateFundingInterval(uint256 fundingInterval, uint256 resetInterval) private pure {
        _assertNonZero(fundingInterval);
        _assertNonZero(resetInterval);
        if (fundingInterval < resetInterval) revert InvalidSettings();
    }

    function _validateMaintenanceMarginRatio(uint256 maintenanceMarginRatio, uint256 maxLeverage) private pure {
        _assertDecimal(maintenanceMarginRatio);
        if (1e18.fullMulDiv(1e18, maintenanceMarginRatio) < maxLeverage) revert InvalidSettings();
    }

    function _assertNonZero(uint256 value) private pure {
        if (value == 0) revert InvalidSettings();
    }

    function _assertDecimal(uint256 decimal) private pure {
        if (decimal == 0) revert InvalidSettings();
        if (decimal > 1e18) revert InvalidSettings();
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IViewPort} from "./IViewPort.sol";

interface IPerpManager is IViewPort {
    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                             PERP MANAGER
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function depositFreeCollateral(address account, uint256 amount) external;

    function withdrawFreeCollateral(address account, uint256 amount) external;

    function depositFromSpot(address account, uint256 amount) external;

    function withdrawToSpot(address account, uint256 amount) external;
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

// Local types, libs, and interfaces
import {ICLOB} from "./ICLOB.sol";
import {ICLOBManager} from "./ICLOBManager.sol";
import {IAccountManager} from "../account-manager/IAccountManager.sol";
import {CLOBStorageLib} from "./types/Book.sol";
import {TransientMakerData, MakerCredit} from "./types/TransientMakerData.sol";
import {Order, OrderLib, OrderId, OrderIdLib, Side} from "./types/Order.sol";
import {Book, BookLib, Limit, MarketConfig, MarketSettings} from "./types/Book.sol";

// Internal package types, libs, and interfaces
import {IOperatorPanel} from "contracts/utils/interfaces/IOperatorPanel.sol";
import {SpotOperatorRoles} from "contracts/utils/OperatorPanel.sol";
import {OperatorHelperLib} from "contracts/utils/types/OperatorHelperLib.sol";
import {EventNonceLib as CLOBEventNonce} from "contracts/utils/types/EventNonce.sol";

// Solady and OZ imports
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin-contracts-upgradeable/access/Ownable2StepUpgradeable.sol";

/**
 * @title CLOB
 * Main spot market contract for trading asset pairs on an orderbook
 */
contract CLOB is ICLOB, Ownable2StepUpgradeable {
    using OrderLib for *;
    using OrderIdLib for uint256;
    using OrderIdLib for address;
    using FixedPointMathLib for uint256;
    using SafeCastLib for uint256;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                EVENTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0xc0208cc462e0f7d7b2329363da41c40e123ba2c9db4b8b03a183140d67ad1c60
    event CancelFailed(uint256 indexed eventNonce, uint256 orderId, address owner);

    /// @dev sig: 0xacb8106c549e32473004de43588b1bd716fc82873c60790caab04149f2cb9466
    event OrderCanceled(
        uint256 indexed eventNonce,
        uint256 indexed orderId,
        address indexed owner,
        uint256 quoteTokenRefunded,
        uint256 baseTokenRefunded,
        CancelType context
    );

    /// @dev sig: 0x06956ad87855e4ad9efb290bad3c7ef7a8c7cff5e28b5926b570b492c45b9c37
    event OrderAmended(
        uint256 indexed eventNonce, Order preAmend, AmendArgs args, int256 quoteTokenDelta, int256 baseTokenDelta
    );

    /// @dev sig: 0x76a9cd4a6124a3883e613ae4146376b48db63cfd526306751587a148642fce56
    event OrderProcessed(
        uint256 indexed eventNonce,
        address indexed account,
        uint256 indexed orderId,
        ICLOB.TiF tif,
        uint256 limitPrice,
        uint256 basePosted,
        int256 quoteDelta,
        int256 baseDelta,
        uint256 takerFee
    );

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ERRORS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0x175e7f45
    error ZeroAmend();
    /// @dev sig: 0xb82df155
    error ZeroOrder();
    /// @dev sig: 0x91b373e1
    error AmendInvalid();
    /// @dev sig: 0xc56873ba
    error OrderExpired();
    /// @dev sig: 0xd8a00083
    error ZeroCostTrade();
    /// @dev sig: 0xf1a5cd31
    error FOKOrderNotFilled();
    /// @dev sig: 0xba2ea531
    error AmendUnauthorized();
    /// @dev sig: 0xf99412b1
    error CancelUnauthorized();
    /// @dev sig: 0xd268c85f
    error ManagerUnauthorized();
    /// @dev sig: 0xd093feb7
    error FactoryUnauthorized();
    /// @dev sig: 0x3e27eb6d
    error PostOnlyOrderWouldFill();
    /// @dev sig: 0xb134397c
    error AmendNonPostOnlyInvalid();
    /// @dev sig: 0x315ff5e5
    error MaxOrdersInBookPostNotCompetitive();

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                CONSTANTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/


    /// @dev The abi version of this impl so the indexer can handle event-changing upgrades
    uint256 public constant ABI_VERSION = 1;

    /// @dev The global router address available to all CLOBs that can bypass the operator check
    address public immutable gteRouter;
    /// @dev The operator contract for role-based access control (same as accountManager)
    IOperatorPanel public immutable operator;
    /// @dev The factory that created this contract and controls its settings as well as processing maker settlement
    ICLOBManager public immutable factory;
    /// @dev The account manager contract for direct balance operations (and operator checks)
    IAccountManager public immutable accountManager;
    /// @dev Maximum number of maker orders allowed per side of the order book
    /// before the least competitive orders get bumped
    uint256 public immutable maxNumOrdersPerSide;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                MODIFIERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    modifier onlySenderOrOperator(address account, SpotOperatorRoles requiredRole) {
        OperatorHelperLib.onlySenderOrOperator(operator, gteRouter, account, requiredRole);
        _;
    }

    modifier onlyManager() {
        if (msg.sender != address(factory)) revert ManagerUnauthorized();
        _;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                    CONSTRUCTOR AND INITIALIZATION
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor(address _factory, address _gteRouter, address _accountManager, uint256 _maxNumOrdersPerSide) {
        factory = ICLOBManager(_factory);
        gteRouter = _gteRouter;
        operator = IOperatorPanel(_accountManager);
        accountManager = IAccountManager(_accountManager);
        maxNumOrdersPerSide = _maxNumOrdersPerSide;
        _disableInitializers();
    }

    /// @notice Initializes the `marketConfig`, `marketSettings`, and `initialOwner` of the market
    function initialize(MarketConfig memory marketConfig, MarketSettings memory marketSettings, address initialOwner)
        external
        initializer
    {
        __CLOB_init(marketConfig, marketSettings, initialOwner);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            EXTERNAL GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Gets base token
    function getBaseToken() external view returns (address) {
        return _getStorage().config().baseToken;
    }

    /// @notice Gets quote token
    function getQuoteToken() external view returns (address) {
        return _getStorage().config().quoteToken;
    }

    /// @notice Gets the base token amount equivalent to `quoteAmount` at a given `price`
    /// @dev This price does not have to be within tick size
    function getBaseTokenAmount(uint256 price, uint256 quoteAmount) external view returns (uint256) {
        return _getStorage().getBaseTokenAmount(price, quoteAmount);
    }

    /// @notice Gets the quote token amount equivalent to `baseAmount` at a given `price`
    /// @dev This price dos not have to be within tick size
    function getQuoteTokenAmount(uint256 price, uint256 baseAmount) external view returns (uint256) {
        return _getStorage().getQuoteTokenAmount(price, baseAmount);
    }

    /// @notice Gets the market config
    function getMarketConfig() external view returns (MarketConfig memory) {
        return _getStorage().config();
    }

    /// @notice Gets the market settings
    function getMarketSettings() external view returns (MarketSettings memory) {
        return _getStorage().settings();
    }

    /// @notice Gets tick size
    function getTickSize() external view returns (uint256) {
        return _getStorage().settings().tickSize;
    }

    /// @notice Gets lot size in base
    function getLotSizeInBase() external view returns (uint256) {
        return _getStorage().settings().lotSizeInBase;
    }

    /// @notice Gets quote and base open interest
    function getOpenInterest() external view returns (uint256 quoteOi, uint256 baseOi) {
        return (_getStorage().metadata().quoteTokenOpenInterest, _getStorage().metadata().baseTokenOpenInterest);
    }

    /// @notice Gets an order in the book from its id
    function getOrder(uint256 orderId) external view returns (Order memory) {
        return _getStorage().orders[orderId.toOrderId()];
    }

    /// @notice Gets top of book as price (max bid and min ask)
    function getTOB() external view returns (uint256 maxBid, uint256 minAsk) {
        return (_getStorage().getBestBidPrice(), _getStorage().getBestAskPrice());
    }

    /// @notice Gets the bid or ask Limit at a price depending on `side`
    function getLimit(uint256 price, Side side) external view returns (Limit memory) {
        return _getStorage().getLimit(price, side);
    }

    /// @notice Gets total bid limit orders in the book
    function getNumBids() external view returns (uint256) {
        return _getStorage().metadata().numBids;
    }

    /// @notice Gets total ask limit orders in the book
    function getNumAsks() external view returns (uint256) {
        return _getStorage().metadata().numAsks;
    }

    /// @notice Gets a list of orders, starting at an orderId
    function getNextOrders(uint256 startOrderId, uint256 numOrders) external view returns (Order[] memory) {
        return _getStorage().getNextOrders(startOrderId.toOrderId(), numOrders);
    }

    /// @notice Gets the next populated higher price limit to `price` on a side of the book
    function getNextBiggestPrice(uint256 price, Side side) external view returns (uint256) {
        return _getStorage().getNextBiggestPrice(price, side);
    }

    /// @notice Gets the next populated lower price limit to `price` on a side of the book
    function getNextSmallestPrice(uint256 price, Side side) external view returns (uint256) {
        return _getStorage().getNextSmallestPrice(price, side);
    }

    /// @notice Gets the next order id (nonce) that will be used upon placing an order
    /// @dev Placing both limit and fill orders increment the next orderId
    function getNextOrderId() external view returns (uint256) {
        return (_getStorage().metadata().orderIdCounter + 1);
    }

    /// @notice Gets the current event nonce
    function getEventNonce() external view returns (uint256) {
        return CLOBEventNonce.getCurrentNonce();
    }

    /// @notice Gets `pageSize` of orders from TOB down from a `startPrice` and on a given `side` of the book
    function getOrdersPaginated(uint256 startPrice, Side side, uint256 pageSize)
        external
        view
        returns (Order[] memory result, Order memory nextOrder)
    {
        Book storage ds = _getStorage();

        nextOrder = side == Side.BUY
            ? ds.orders[ds.bidLimits[startPrice].headOrder]
            : ds.orders[ds.askLimits[startPrice].headOrder];

        return ds.getOrdersPaginated(nextOrder, pageSize);
    }

    /// @notice Gets `pageSize` of orders from TOB down, starting at `startOrderId`
    function getOrdersPaginated(OrderId startOrderId, uint256 pageSize)
        external
        view
        returns (Order[] memory result, Order memory nextOrder)
    {
        Book storage ds = _getStorage();
        nextOrder = ds.orders[startOrderId];

        return ds.getOrdersPaginated(nextOrder, pageSize);
    }

    function getBaseQuanta() external view returns (uint256) {
        return _getStorage().getBaseQuanta();
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            AUTH-ONLY SETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Sets the new max limits per txn
    function setMaxLimitsPerTx(uint8 newMaxLimits) external onlyManager {
        _getStorage().setMaxLimitsPerTx(newMaxLimits);
    }

    /// @notice Sets the tick size of the book
    /// @dev New orders' limit prices % tickSize must be 0
    function setTickSize(uint256 tickSize) external onlyManager {
        _getStorage().setTickSize(tickSize);
    }

    /// @notice Sets the minimum amount an order (in base) must be to be placed on the book
    /// @dev Reducing an order below this amount will cause the order to get cancelled
    function setMinLimitOrderAmountInBase(uint256 newMinLimitOrderAmountInBase) external onlyManager {
        _getStorage().setMinLimitOrderAmountInBase(newMinLimitOrderAmountInBase);
    }

    /// @notice Sets the lot size in base for standardized trade sizes
    /// @dev Orders must be multiples of lot size. Setting to 0 disables lot size restrictions
    function setLotSizeInBase(uint256 newLotSizeInBase) external onlyManager {
        _getStorage().setLotSizeInBase(newLotSizeInBase);
    }

    /// @notice Clears out expired orders from one side of the book
    /// @dev Cancels must be on a single side bc settlement only treats one side (either base or quote)
    /// as a refund and the other side as a fill that incurs trading fees
    function adminCancelExpiredOrders(OrderId[] calldata ids, Side side) external onlyManager returns (bool[] memory) {
        bool[] memory removed = new bool[](ids.length);

        Book storage ds = _getStorage();

        for (uint256 i = 0; i < ids.length; i++) {
            Order storage o = ds.orders[ids[i]];

            if (!o.isExpired() || o.side != side) continue;

            removed[i] = true;
            side == Side.BUY ? _removeExpiredBid(ds, o) : _removeExpiredAsk(ds, o);
        }

        // Virtual taker side is opposite of cancelled orders' side to ensure refunds aren't charged fees
        Side virtualTakerSide = side == Side.BUY ? Side.SELL : Side.BUY;
        _settleIncomingOrder({
            ds: ds,
            account: address(0),
            side: virtualTakerSide,
            quoteTokenAmount: 0,
            baseTokenAmount: 0
        });

        return removed;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        EXTERNAL ORDER PLACEMENT
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function placeOrder(address account, ICLOB.PlaceOrderArgs calldata args)
        external
        onlySenderOrOperator(account, SpotOperatorRoles.PLACE_ORDER)
        returns (ICLOB.PlaceOrderResult memory)
    {
        Book storage ds = _getStorage();

        // inc order nonce regardless if the order is a pure take, or a custom id is used
        uint256 orderId = ds.incrementOrderId();

        if (args.clientOrderId > 0) {
            orderId = account.getClientOrderId(args.clientOrderId);
            ds.assertUnusedOrderId(orderId);
        }

        Order memory newOrder = args.toOrderChecked(orderId, account);

        // Fires an {OrderProcessed} event at the end of either sub-routines
        if (args.side == Side.BUY) return _processBid(ds, account, newOrder, args);
        else return _processAsk(ds, account, newOrder, args);
    }

    /// @notice Amends an existing order for `account`
    function amend(address account, AmendArgs calldata args)
        external
        onlySenderOrOperator(account, SpotOperatorRoles.PLACE_ORDER)
        returns (int256 quoteDelta, int256 baseDelta)
    {
        Book storage ds = _getStorage();
        Order storage order = ds.orders[args.orderId.toOrderId()];

        if (order.id.unwrap() == 0) revert OrderLib.OrderNotFound();
        if (order.owner != account) revert AmendUnauthorized();

        ds.assertLimitPriceInBounds(args.price);
        ds.assertMakeAmountInBounds(args.amountInBase);

        if (args.cancelTimestamp.isExpired()) revert AmendInvalid();

        // Update order
        (quoteDelta, baseDelta) = _processAmend(ds, order, args);
    }

    /// @notice Cancels a list of orders for `account`
    function cancel(address account, CancelArgs memory args)
        external
        onlySenderOrOperator(account, SpotOperatorRoles.PLACE_ORDER)
        returns (uint256, uint256)
    {
        Book storage ds = _getStorage();
        (address quoteToken, address baseToken) = (ds.config().quoteToken, ds.config().baseToken);

        (uint256 totalQuoteTokenRefunded, uint256 totalBaseTokenRefunded) = _executeCancel(ds, account, args);

        if (totalBaseTokenRefunded > 0) accountManager.creditAccount(account, baseToken, totalBaseTokenRefunded);
        if (totalQuoteTokenRefunded > 0) accountManager.creditAccount(account, quoteToken, totalQuoteTokenRefunded);

        return (totalQuoteTokenRefunded, totalBaseTokenRefunded);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            INTERNAL FILL LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Performs matching and settlement for a bid order
    function _processBid(Book storage ds, address account, Order memory newOrder, ICLOB.PlaceOrderArgs calldata args)
        internal
        returns (ICLOB.PlaceOrderResult memory res)
    {
        (uint256 postAmount, uint256 totalQuoteSent, uint256 totalBaseReceived) =
            _executeBid(ds, newOrder, args.tif, args.baseDenominated);

        if (postAmount + totalQuoteSent + totalBaseReceived == 0) revert ZeroOrder();

        if (totalBaseReceived != totalQuoteSent && (totalBaseReceived == 0 || totalQuoteSent == 0)) {
            revert ZeroCostTrade();
        }

        uint256 takerFee = _settleIncomingOrder(ds, account, Side.BUY, totalQuoteSent + postAmount, totalBaseReceived);

        // Populate result struct
        res.account = account;
        res.orderId = newOrder.id.unwrap();
        res.quoteTokenAmountTraded = -int256(totalQuoteSent);
        res.baseTokenAmountTraded = int256(totalBaseReceived);
        res.takerFee = takerFee;

        // Sets the base posted of the remainder of the order that was a make
        // Order amount is always converted to base when posting
        if (uint8(args.tif) <= 1) res.basePosted = newOrder.amount;

        // Set whether this was a market order (limitPrice = max) or limit order
        res.wasMarketOrder = (args.limitPrice == type(uint256).max);

        emit OrderProcessed({
            eventNonce: CLOBEventNonce.inc(),
            account: account,
            orderId: res.orderId,
            tif: args.tif,
            limitPrice: args.limitPrice,
            basePosted: res.basePosted,
            quoteDelta: res.quoteTokenAmountTraded,
            baseDelta: res.baseTokenAmountTraded,
            takerFee: takerFee
        });
    }

    function _processAsk(Book storage ds, address account, Order memory newOrder, ICLOB.PlaceOrderArgs calldata args)
        internal
        returns (ICLOB.PlaceOrderResult memory res)
    {
        (uint256 postAmount, uint256 totalQuoteReceived, uint256 totalBaseSent) =
            _executeAsk(ds, newOrder, args.tif, args.baseDenominated);

        if (postAmount + totalQuoteReceived + totalBaseSent == 0) revert ZeroOrder();

        if (totalBaseSent != totalQuoteReceived && (totalBaseSent == 0 || totalQuoteReceived == 0)) {
            revert ZeroCostTrade();
        }

        uint256 takerFee = _settleIncomingOrder(ds, account, Side.SELL, totalQuoteReceived, totalBaseSent + postAmount);

        // Populate result struct
        res.account = account;
        res.orderId = newOrder.id.unwrap();
        res.quoteTokenAmountTraded = int256(totalQuoteReceived);
        res.baseTokenAmountTraded = -int256(totalBaseSent);
        res.takerFee = takerFee;

        // Sets the base posted of the remainder of the order that was a make
        // Order amount is always converted to base when posting
        if (uint8(args.tif) <= 1) res.basePosted = newOrder.amount;

        // Set whether this was a market order (limitPrice = 0) or limit order
        res.wasMarketOrder = (args.limitPrice == 0);

        emit OrderProcessed({
            eventNonce: CLOBEventNonce.inc(),
            account: account,
            orderId: res.orderId,
            tif: args.tif,
            limitPrice: args.limitPrice,
            basePosted: res.basePosted,
            quoteDelta: res.quoteTokenAmountTraded,
            baseDelta: res.baseTokenAmountTraded,
            takerFee: takerFee
        });
    }

    /// @dev Performs the core matching and placement of a bid order into the book
    function _executeBid(Book storage ds, Order memory newOrder, ICLOB.TiF tif, bool baseDenominated)
        internal
        returns (uint256 postAmount, uint256 totalQuoteSent, uint256 totalBaseReceived)
    {
        // Attempt to fill any of the incoming order that's overlapping into asks
        if (ds.getBestAskPrice() <= newOrder.price) {
            if (tif == ICLOB.TiF.MOC) revert PostOnlyOrderWouldFill();
            (totalQuoteSent, totalBaseReceived) = _matchIncomingBid(ds, newOrder, baseDenominated);
        }

        if (tif == ICLOB.TiF.FOK && newOrder.amount > 0) revert FOKOrderNotFilled();

        bool isTake = false;
        (isTake, newOrder.amount) = _getTakeOrPostAmount(
            ds, tif, newOrder.amount, totalQuoteSent | totalBaseReceived > 0, baseDenominated, newOrder.price
        );

        // The order was a TAKE only either due to TIF settings,
        // or because a partially filled GTC had insufficient remaining amount in base
        if (isTake) return (0, totalQuoteSent, totalBaseReceived);

        // Validate price and amount bounds
        ds.assertLimitPriceInBounds(newOrder.price);

        // // Enforce per-tx max limit placements (unless exempt) and increment counter
        ds.incrementLimitsPlaced(address(factory), msg.sender);

        // The book is full, pop the least competitive order (or revert if incoming is the least competitive)
        if (ds.metadata().numBids == maxNumOrdersPerSide) {
            uint256 minBidPrice = ds.getWorstBidPrice();
            if (newOrder.price <= minBidPrice) revert MaxOrdersInBookPostNotCompetitive();

            _removeNonCompetitiveOrder(ds, ds.orders[ds.bidLimits[minBidPrice].tailOrder]);
        }

        ds.addOrderToBook(newOrder);
        postAmount = ds.getQuoteTokenAmount(newOrder.price, newOrder.amount);

        return (postAmount, totalQuoteSent, totalBaseReceived);
    }

    function _executeAsk(Book storage ds, Order memory newOrder, ICLOB.TiF tif, bool baseDenominated)
        internal
        returns (uint256 postAmount, uint256 totalQuoteReceived, uint256 totalBaseSent)
    {
        // Attempt to fill any of the incoming order that's overlapping into bids
        if (ds.getBestBidPrice() >= newOrder.price) {
            if (tif == ICLOB.TiF.MOC) revert PostOnlyOrderWouldFill();
            (totalQuoteReceived, totalBaseSent) = _matchIncomingAsk(ds, newOrder, baseDenominated);
        }

        if (tif == ICLOB.TiF.FOK && newOrder.amount > 0) revert FOKOrderNotFilled();

        bool isTake = false;
        (isTake, newOrder.amount) = _getTakeOrPostAmount(
            ds, tif, newOrder.amount, totalQuoteReceived | totalBaseSent > 0, baseDenominated, newOrder.price
        );

        // The order was a TAKE only either due to TIF settings,
        // or because a partially filled GTC had insufficient remaining amount in base
        if (isTake) return (0, totalQuoteReceived, totalBaseSent);

        // Validate price and amount bounds
        ds.assertLimitPriceInBounds(newOrder.price);

        // Enforce per-tx max limit placements (unless exempt) and increment counter
        ds.incrementLimitsPlaced(address(factory), msg.sender);

        // The book is full, pop the least competitive order (or revert if incoming is the least competitive)
        if (ds.metadata().numAsks == maxNumOrdersPerSide) {
            uint256 maxAskPrice = ds.getWorstAskPrice();
            if (newOrder.price >= maxAskPrice) revert MaxOrdersInBookPostNotCompetitive();

            _removeNonCompetitiveOrder(ds, ds.orders[ds.askLimits[maxAskPrice].tailOrder]);
        }

        ds.addOrderToBook(newOrder);
        postAmount = newOrder.amount;

        return (postAmount, totalQuoteReceived, totalBaseSent);
    }

    function _getTakeOrPostAmount(
        Book storage ds,
        ICLOB.TiF tif,
        uint256 remainingAmount,
        bool matchOccurred,
        bool baseDenominated,
        uint256 limitPrice
    ) internal view returns (bool isTake, uint256 postAmount) {
        // Order is explicitly a TAKE
        if (tif == ICLOB.TiF.FOK || tif == ICLOB.TiF.IOC) return (true, 0);

        // Order amount must be in base and conform to current lot size before posting
        remainingAmount = baseDenominated
            ? ds.boundToLots(remainingAmount)
            : ds.boundToLots(ds.getBaseTokenAmount(remainingAmount, limitPrice));

        // There is not enough order amount to post
        if (remainingAmount < ds.settings().minLimitOrderAmountInBase) {
            // If a GTC had any take, and the remaining amount is invalid,
            // this is permissible as a full take instead of reverting
            if (tif == ICLOB.TiF.GTC && matchOccurred) return (true, 0);

            // The make-only order violates minimum amount
            revert BookLib.LimitOrderAmountInvalid();
        }

        return (false, remainingAmount);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        INTERNAL AMEND LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Performs the amending of an order
    function _processAmend(Book storage ds, Order storage order, AmendArgs calldata args)
        internal
        returns (int256 quoteTokenDelta, int256 baseTokenDelta)
    {
        Order memory preAmend = order;
        address maker = preAmend.owner;

        if (args.cancelTimestamp.isExpired() || args.amountInBase < ds.settings().minLimitOrderAmountInBase) {
            revert AmendInvalid();
        }

        // Check lot size compliance after other validations
        ds.assertLotSizeCompliant(args.amountInBase);

        if (order.side != args.side || order.price != args.price) {
            // change place in book
            (quoteTokenDelta, baseTokenDelta) = _executeAmendNewOrder(ds, order, args);
        } else if (order.amount != args.amountInBase) {
            // change amount
            (quoteTokenDelta, baseTokenDelta) =
                _executeAmendAmount(ds, order, args.amountInBase, uint32(args.cancelTimestamp));
        } else if (args.cancelTimestamp != order.cancelTimestamp) {
            order.cancelTimestamp = uint32(args.cancelTimestamp);
        } else {
            revert ZeroAmend();
        }

        emit OrderAmended(CLOBEventNonce.inc(), preAmend, args, quoteTokenDelta, baseTokenDelta);

        _settleAmend(ds, maker, quoteTokenDelta, baseTokenDelta);
    }

    /// @dev Performs the removal and replacement of an amended order with a new price or side
    function _executeAmendNewOrder(Book storage ds, Order storage order, AmendArgs calldata args)
        internal
        returns (int256 quoteTokenDelta, int256 baseTokenDelta)
    {
        Order memory newOrder;

        newOrder.owner = order.owner;
        newOrder.id = order.id;
        newOrder.side = args.side;
        newOrder.price = args.price;
        newOrder.amount = args.amountInBase;
        newOrder.cancelTimestamp = uint32(args.cancelTimestamp);

        if (order.side == Side.BUY) quoteTokenDelta = ds.getQuoteTokenAmount(order.price, order.amount).toInt256();
        else baseTokenDelta = order.amount.toInt256();

        ds.removeOrderFromBook(order);

        uint256 postAmount;
        if (args.side == Side.BUY) {
            (postAmount,,) = _executeBid(ds, newOrder, ICLOB.TiF.MOC, true);

            quoteTokenDelta -= postAmount.toInt256();
        } else {
            (postAmount,,) = _executeAsk(ds, newOrder, ICLOB.TiF.MOC, true);

            baseTokenDelta -= postAmount.toInt256();
        }
    }

    /// @dev Performs the updating of an amended order with a new amount
    function _executeAmendAmount(Book storage ds, Order storage order, uint256 amount, uint32 cancelTimestamp)
        internal
        returns (int256 quoteTokenDelta, int256 baseTokenDelta)
    {
        if (order.side == Side.BUY) {
            int256 oldAmountInQuote = ds.getQuoteTokenAmount(order.price, order.amount).toInt256();
            int256 newAmountInQuote = ds.getQuoteTokenAmount(order.price, amount).toInt256();

            quoteTokenDelta = oldAmountInQuote - newAmountInQuote;

            ds.metadata().quoteTokenOpenInterest =
                uint256(ds.metadata().quoteTokenOpenInterest.toInt256() - quoteTokenDelta);
        } else {
            baseTokenDelta = order.amount.toInt256() - amount.toInt256();

            ds.metadata().baseTokenOpenInterest =
                uint256(ds.metadata().baseTokenOpenInterest.toInt256() - baseTokenDelta);
        }

        order.amount = amount;
        order.cancelTimestamp = cancelTimestamp;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        INTERNAL MATCHING LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Internal struct to prevent blowing stack
    struct __MatchData__ {
        uint256 matchedAmount;
        uint256 baseDelta;
        uint256 quoteDelta;
    }

    /// @dev Match incoming bid order to best asks
    function _matchIncomingBid(Book storage ds, Order memory incomingOrder, bool amountIsBase)
        internal
        returns (uint256 totalQuoteSent, uint256 totalBaseReceived)
    {
        uint256 bestAskPrice = ds.getBestAskPrice();

        while (bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0) {
            Limit storage limit = ds.askLimits[bestAskPrice];
            Order storage bestAskOrder = ds.orders[limit.headOrder];

            if (bestAskOrder.isExpired()) {
                _removeExpiredAsk(ds, bestAskOrder);
                bestAskPrice = ds.getBestAskPrice();
                continue;
            }

            // slither-disable-next-line uninitialized-local
            __MatchData__ memory currMatch =
                _matchIncomingOrder(ds, bestAskOrder, incomingOrder, bestAskPrice, amountIsBase);

            incomingOrder.amount -= currMatch.matchedAmount;

            totalQuoteSent += currMatch.quoteDelta;
            totalBaseReceived += currMatch.baseDelta;

            bestAskPrice = ds.getBestAskPrice();
        }
    }

    /// @dev Match incoming ask order to best bids
    function _matchIncomingAsk(Book storage ds, Order memory incomingOrder, bool amountIsBase)
        internal
        returns (uint256 totalQuoteReceived, uint256 totalBaseSent)
    {
        uint256 bestBidPrice = ds.getBestBidPrice();

        while (bestBidPrice >= incomingOrder.price && incomingOrder.amount > 0) {
            Limit storage limit = ds.bidLimits[bestBidPrice];
            Order storage bestBidOrder = ds.orders[limit.headOrder];

            if (bestBidOrder.isExpired()) {
                _removeExpiredBid(ds, bestBidOrder);
                bestBidPrice = ds.getBestBidPrice();
                continue;
            }

            // slither-disable-next-line uninitialized-local
            __MatchData__ memory currMatch =
                _matchIncomingOrder(ds, bestBidOrder, incomingOrder, bestBidPrice, amountIsBase);

            incomingOrder.amount -= currMatch.matchedAmount;

            totalQuoteReceived += currMatch.quoteDelta;
            totalBaseSent += currMatch.baseDelta;

            bestBidPrice = ds.getBestBidPrice();
        }
    }

    function _boundMakerToLotSize(Book storage ds, Order storage order, uint256 lotSize) internal {
        uint256 remainder = order.amount % lotSize;
        if (remainder == 0) return;

        if (remainder == order.amount) {
            if (order.side == Side.BUY) _removeExpiredBid(ds, order);
            else _removeExpiredAsk(ds, order);
            return;
        }

        if (order.side == Side.BUY) {
            uint256 quoteTokenAmount = ds.getQuoteTokenAmount(order.price, remainder);
            TransientMakerData.addQuoteToken(order.owner, quoteTokenAmount);

            ds.metadata().quoteTokenOpenInterest -= quoteTokenAmount;
        } else {
            TransientMakerData.addBaseToken(order.owner, remainder);
            ds.metadata().baseTokenOpenInterest -= remainder;
        }
        order.amount -= remainder;
    }

    /// @dev Matches an incoming order to its next counterparty order, crediting the maker and removing the counterparty order if fully filled
    function _matchIncomingOrder(
        Book storage ds,
        Order storage makerOrder,
        Order memory takerOrder,
        uint256 matchedPrice,
        bool amountIsBase
    ) internal returns (__MatchData__ memory matchData) {
        uint256 lotSize = ds.settings().lotSizeInBase;

        _boundMakerToLotSize(ds, makerOrder, lotSize);
        uint256 matchedBase = makerOrder.amount;

        if (amountIsBase) {
            // denominated in base
            matchData.baseDelta = (matchedBase.min(takerOrder.amount) / lotSize) * lotSize;
            matchData.quoteDelta = ds.getQuoteTokenAmount(matchedPrice, matchData.baseDelta);
            matchData.matchedAmount = matchData.baseDelta != matchedBase ? takerOrder.amount : matchData.baseDelta;
        } else {
            // denominated in quote
            matchData.baseDelta =
                (matchedBase.min(ds.getBaseTokenAmount(matchedPrice, takerOrder.amount)) / lotSize) * lotSize;
            matchData.quoteDelta = ds.getQuoteTokenAmount(matchedPrice, matchData.baseDelta);
            matchData.matchedAmount = matchData.baseDelta != matchedBase ? takerOrder.amount : matchData.quoteDelta;
        }

        // Early return if no tradeable amount due to lot size constraints (dust)
        if (matchData.baseDelta == 0) return matchData;

        bool orderRemoved = matchData.baseDelta == matchedBase;

        // Handle token accounting for maker.
        if (takerOrder.side == Side.BUY) {
            TransientMakerData.addQuoteToken(makerOrder.owner, matchData.quoteDelta);

            if (!orderRemoved) ds.metadata().baseTokenOpenInterest -= matchData.baseDelta;
        } else {
            TransientMakerData.addBaseToken(makerOrder.owner, matchData.baseDelta);

            if (!orderRemoved) ds.metadata().quoteTokenOpenInterest -= matchData.quoteDelta;
        }

        if (orderRemoved) ds.removeOrderFromBook(makerOrder);
        else makerOrder.amount -= matchData.baseDelta;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        INTERNAL EXPIRY LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Removes an expired ask, adding the order's amount to settlement data as a base refund
    function _removeExpiredAsk(Book storage ds, Order storage order) internal {
        uint256 baseTokenAmount = order.amount;

        // We can add the refund to maker fills because both cancelled asks and filled bids are credited in baseTokens
        TransientMakerData.addBaseToken(order.owner, baseTokenAmount);

        ds.removeOrderFromBook(order);
    }

    /// @dev Removes an expired bid, adding the order's amount to settlement as a quote refund
    function _removeExpiredBid(Book storage ds, Order storage order) internal {
        uint256 quoteTokenAmount = ds.getQuoteTokenAmount(order.price, order.amount);

        // We can add the refund to maker fills because both cancelled bids and filled asks are credited in quoteTokens
        TransientMakerData.addQuoteToken(order.owner, quoteTokenAmount);

        ds.removeOrderFromBook(order);
    }

    /// @notice Removes the least competitive order from the book
    function _removeNonCompetitiveOrder(Book storage ds, Order storage order) internal {
        uint256 quoteRefunded;
        uint256 baseRefunded;
        if (order.side == Side.BUY) {
            quoteRefunded = ds.getQuoteTokenAmount(order.price, order.amount);
            accountManager.creditAccountNoEvent(order.owner, address(ds.config().quoteToken), quoteRefunded);
        } else {
            baseRefunded = order.amount;
            accountManager.creditAccountNoEvent(order.owner, address(ds.config().baseToken), baseRefunded);
        }

        emit OrderCanceled(
            CLOBEventNonce.inc(),
            order.id.unwrap(),
            order.owner,
            quoteRefunded,
            baseRefunded,
            CancelType.NON_COMPETITIVE
        );

        ds.removeOrderFromBook(order);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        INTERNAL CANCEL LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Performs the cancellation of an account's orders
    function _executeCancel(Book storage ds, address account, CancelArgs memory args)
        internal
        returns (uint256 totalQuoteTokenRefunded, uint256 totalBaseTokenRefunded)
    {
        uint256 numOrders = args.orderIds.length;
        for (uint256 i = 0; i < numOrders; i++) {
            uint256 orderId = args.orderIds[i];
            Order storage order = ds.orders[orderId.toOrderId()];

            if (order.isNull()) {
                emit CancelFailed(CLOBEventNonce.inc(), orderId, account);
                continue; // Order may have been matched
            } else if (order.owner != account) {
                revert CancelUnauthorized();
            }

            uint256 quoteTokenRefunded = 0;
            uint256 baseTokenRefunded = 0;

            if (order.side == Side.BUY) {
                quoteTokenRefunded = ds.getQuoteTokenAmount(order.price, order.amount);
                totalQuoteTokenRefunded += quoteTokenRefunded;
            } else {
                baseTokenRefunded = order.amount;
                totalBaseTokenRefunded += baseTokenRefunded;
            }

            ds.removeOrderFromBook(order);

            uint256 eventNonce = CLOBEventNonce.inc();
            emit OrderCanceled(eventNonce, orderId, account, quoteTokenRefunded, baseTokenRefunded, CancelType.USER);
        }
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                        INTERNAL SETTLEMENT LOGIC
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Settles token accounting in the factory for the incoming trade
    function _settleIncomingOrder(
        Book storage ds,
        address account,
        Side side,
        uint256 quoteTokenAmount,
        uint256 baseTokenAmount
    ) internal returns (uint256 takerFee) {
        SettleParams memory settleParams;

        (settleParams.quoteToken, settleParams.baseToken) = (ds.config().quoteToken, ds.config().baseToken);

        settleParams.taker = account;
        settleParams.side = side;

        settleParams.takerQuoteAmount = quoteTokenAmount;
        settleParams.takerBaseAmount = baseTokenAmount;

        settleParams.makerCredits = TransientMakerData.getMakerCreditsAndClearStorage();

        return accountManager.settleIncomingOrder(settleParams);
    }

    /// @dev Settles the token deltas in the factory from an amend
    function _settleAmend(Book storage ds, address maker, int256 quoteTokenDelta, int256 baseTokenDelta) internal {
        if (quoteTokenDelta > 0) {
            accountManager.creditAccount(maker, address(ds.config().quoteToken), uint256(quoteTokenDelta));
        } else if (quoteTokenDelta < 0) {
            accountManager.debitAccount(maker, address(ds.config().quoteToken), uint256(-quoteTokenDelta));
        }

        if (baseTokenDelta > 0) {
            accountManager.creditAccount(maker, address(ds.config().baseToken), uint256(baseTokenDelta));
        } else if (baseTokenDelta < 0) {
            accountManager.debitAccount(maker, address(ds.config().baseToken), uint256(-baseTokenDelta));
        }
    }

    // This naming reflects OZ initializer naming
    // slither-disable-next-line naming-convention
    function __CLOB_init(MarketConfig memory marketConfig, MarketSettings memory marketSettings, address initialOwner)
        internal
    {
        __Ownable_init(initialOwner);
        CLOBStorageLib.init(_getStorage(), marketConfig, marketSettings);
    }

    /// @dev Helper to assign the storage slot to the Book struct
    function _getStorage() internal pure returns (Book storage) {
        return CLOBStorageLib._getCLOBStorage();
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

// Local types, libs, contracts, and interfaces
import {CLOB, ICLOB} from "./CLOB.sol";
import {Side, OrderId} from "./types/Order.sol";
import {MakerCredit} from "./types/TransientMakerData.sol";
import {ICLOBManager, ConfigParams, SettingsParams} from "./ICLOBManager.sol";
import {FeeTiers} from "./types/FeeData.sol";
import {CLOBStorageLib, MarketConfig, MarketSettings, MIN_MIN_LIMIT_ORDER_AMOUNT_BASE} from "./types/Book.sol";

// Internal package libs and interfaces
import {IAccountManager} from "../account-manager/IAccountManager.sol";
import {EventNonceLib as CLOBEventNonce} from "contracts/utils/types/EventNonce.sol";

// Solady and OZ imports
import {Initializable} from "@solady/utils/Initializable.sol";
import {SafeTransferLib} from "@solady/utils/SafeTransferLib.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {OwnableRoles as CLOBAdminOwnableRoles} from "@solady/auth/OwnableRoles.sol";
import {IERC20Metadata} from "@openzeppelin/token/ERC20/extensions/IERC20Metadata.sol";
import {BeaconProxy, IBeacon} from "@openzeppelin/proxy/beacon/BeaconProxy.sol";

struct CLOBManagerStorage {
    mapping(address clob => bool) isCLOB;
    mapping(bytes32 tokenPairHash => address) clob;
    mapping(address account => bool) maxLimitWhitelist;
}

using CLOBManagerStorageLib for CLOBManagerStorage global;

/// @custom:storage-location erc7201:CLOBManagerStorage
library CLOBManagerStorageLib {
    bytes32 constant CLOB_MANAGER_STORAGE_POSITION =
        keccak256(abi.encode(uint256(keccak256("CLOBManagerStorage")) - 1)) & ~bytes32(uint256(0xff));

    /// @dev Gets the storage slot of the storage struct for the contract calling this library function
    // slither-disable-next-line uninitialized-storage
    function getCLOBManagerStorage() internal pure returns (CLOBManagerStorage storage self) {
        bytes32 position = CLOB_MANAGER_STORAGE_POSITION;

        // slither-disable-next-line assembly
        assembly {
            self.slot := position
        }
    }
}

/**
 * @title CLOBManager
 * @notice Main contract that handles CLOB admin functionality and fee calculations
 */
contract CLOBManager is ICLOBManager, CLOBAdminOwnableRoles, Initializable {
    using FixedPointMathLib for uint256;
    using SafeTransferLib for address;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ERRORS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0x1e4f7d8c
    error InvalidPair();
    /// @dev sig: 0x8fc6f59b
    error MarketExists();
    /// @dev sig: 0xe591f33d
    error InvalidSettings();
    /// @dev sig: 0x1eb00b06
    error InvalidTokenAddress();
    /// @dev sig: 0x353f2237
    error AdminPanelArrayLengthsInvalid();
    /// @dev sig: 0xf9f68635
    error MarketUnauthorized();
    /// @dev sig: 0x6fbe54bd
    error InvalidBeaconAddress();
    /// @dev sig: 0x19ae8c78
    error CLOBBeaconMustHaveRouter();

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                EVENTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    event MarketCreated(
        uint256 indexed eventNonce,
        address indexed creator,
        address indexed baseToken,
        address quoteToken,
        address market,
        uint8 quoteDecimals,
        uint8 baseDecimals,
        ConfigParams config,
        SettingsParams settings
    );

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                CONSTANTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev The abi version of this impl so the indexer can handle event-changing upgrades
    uint256 public constant ABI_VERSION = 1;

    /// @dev Create and call markets to edit their settings
    uint256 public constant MARKET_MANAGER = 1;
    /// @dev Sets users' fee tiers in this contract
    uint256 public constant FEE_TIER_SETTER = 1 << 1;
    /// @dev Whitelists addresses to bypass the markets' max limits per txn
    uint256 public constant MAX_LIMIT_WHITELISTER = 1 << 2;
    /// @dev Clears expired orders from markets
    uint256 public constant EXPIRED_ORDER_CLEARER = 1 << 3;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            IMMUTABLE STATE
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev The beacon proxy containing the logic implementation all clobs' storage use
    address public immutable beacon;
    /// @dev The external AccountManager contract
    IAccountManager public immutable accountManager;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            CONSTRUCTOR
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    constructor(address _beacon, address _accountManager) {
        if (_beacon == address(0)) revert InvalidBeaconAddress();
        beacon = _beacon;
        accountManager = IAccountManager(_accountManager);
        _disableInitializers();
    }

    /// @dev Initializes the contract following ERC1967Factory pattern
    function initialize(address _owner) external initializer {
        _initializeOwner(_owner);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            EXTERNAL GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Gets the market address for a given `tokenA` and `tokenB`
    function getMarketAddress(address tokenA, address tokenB) external view returns (address marketAddress) {
        return _getStorage().clob[_getTokenHash(tokenA, tokenB)];
    }

    /// @notice Gets if `market` is a clob created by this factory
    function isMarket(address market) external view returns (bool) {
        return _getStorage().isCLOB[market];
    }

    /// @notice Gets whether an account is exempt from max limits
    function getMaxLimitExempt(address account) external view returns (bool) {
        return _getStorage().maxLimitWhitelist[account];
    }

    /// @notice Gets the current event nonce
    function getEventNonce() external view returns (uint256) {
        return CLOBEventNonce.getCurrentNonce();
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            ADMIN FUNCTIONS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Creates a new market for `quoteToken` and `baseToken` using beacon proxy
    function createMarket(address baseToken, address quoteToken, SettingsParams calldata settings)
        external
        virtual
        onlyOwnerOrRoles(MARKET_MANAGER)
        returns (address marketAddress)
    {
        _assertValidTokenPair(quoteToken, baseToken);

        uint8 quoteDecimals = IERC20Metadata(quoteToken).decimals();
        uint8 baseDecimals = IERC20Metadata(baseToken).decimals();

        ConfigParams memory config;

        config.quoteToken = quoteToken;
        config.baseToken = baseToken;
        config.quoteSize = 10 ** quoteDecimals;
        config.baseSize = 10 ** baseDecimals;

        _assertValidSettings(settings, config.baseSize);

        CLOBManagerStorage storage self = _getStorage();

        bytes32 tokenPairHash = _getTokenHash(quoteToken, baseToken);

        if (self.clob[tokenPairHash] > address(0)) revert MarketExists();

        bytes memory initData = abi.encodeWithSelector(
            CLOB.initialize.selector,
            MarketConfig({
                quoteToken: config.quoteToken,
                baseToken: config.baseToken,
                quoteSize: config.quoteSize,
                baseSize: config.baseSize
            }),
            MarketSettings({
                status: true,
                maxLimitsPerTx: settings.maxLimitsPerTx,
                minLimitOrderAmountInBase: settings.minLimitOrderAmountInBase,
                tickSize: settings.tickSize,
                lotSizeInBase: settings.lotSizeInBase
            }),
            settings.owner
        );

        // Beacon is immutable and itself non upgradeable
        marketAddress = address(new BeaconProxy(beacon, initData));

        self.isCLOB[marketAddress] = true;
        self.clob[tokenPairHash] = marketAddress;

        // Register the market in AccountManager
        accountManager.registerMarket(marketAddress);

        _emitMarketCreated(msg.sender, marketAddress, quoteDecimals, baseDecimals, config, settings);
    }

    /// @notice Sets the tick size for a market
    function setTickSize(ICLOB market, uint256 newTickSize) external onlyOwnerOrRoles(MARKET_MANAGER) {
        market.setTickSize(newTickSize);
    }

    /// @notice Sets the lot size for a market
    function setLotSizeInBase(ICLOB market, uint256 newLotSize) external onlyOwnerOrRoles(MARKET_MANAGER) {
        market.setLotSizeInBase(newLotSize);
    }

    /// @notice Sets the min limit order amount in base for a market
    function setMinLimitOrderAmountInBase(ICLOB market, uint256 newMinLimitOrderAmountInBase)
        external
        onlyOwnerOrRoles(MARKET_MANAGER)
    {
        market.setMinLimitOrderAmountInBase(newMinLimitOrderAmountInBase);
    }

    /// @notice Clears out expired orders from one side of a market
    function adminCancelExpiredOrders(ICLOB market, OrderId[] calldata ids, Side side)
        external
        onlyOwnerOrRoles(EXPIRED_ORDER_CLEARER)
    {
        market.adminCancelExpiredOrders(ids, side);
    }

    /// @notice Sets fee tiers for accounts
    function setAccountFeeTiers(address[] calldata accounts, FeeTiers[] calldata feeTiers)
        external
        onlyOwnerOrRoles(FEE_TIER_SETTER)
    {
        accountManager.setSpotAccountFeeTiers(accounts, feeTiers);
    }

    /// @notice Sets max limit exemptions for accounts
    function setMaxLimitsExempt(address[] calldata accounts, bool[] calldata toggles)
        external
        onlyOwnerOrRoles(MAX_LIMIT_WHITELISTER)
    {
        if (accounts.length != toggles.length) revert AdminPanelArrayLengthsInvalid();

        CLOBManagerStorage storage self = _getStorage();
        for (uint256 i = 0; i < accounts.length; i++) {
            self.maxLimitWhitelist[accounts[i]] = toggles[i];
        }
    }

    /// @notice Sets the max limits per tx for a market
    function setMaxLimitsPerTx(ICLOB market, uint8 newMaxLimits) external onlyOwnerOrRoles(MARKET_MANAGER) {
        market.setMaxLimitsPerTx(newMaxLimits);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            INTERNAL ASSERTIONS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Checks config and settings params are within correct bounds
    function _assertValidSettings(SettingsParams calldata settings, uint256 baseSize) internal pure {
        if (settings.maxLimitsPerTx == 0) revert InvalidSettings();
        if (settings.minLimitOrderAmountInBase < MIN_MIN_LIMIT_ORDER_AMOUNT_BASE) revert InvalidSettings();
        if (settings.minLimitOrderAmountInBase < settings.lotSizeInBase) revert InvalidSettings();
        if (settings.tickSize.fullMulDiv(settings.lotSizeInBase, baseSize) == 0) revert InvalidSettings();
    }

    /// @dev Performs sanity checks on the addresses passed to make it slightly more difficult to deploy a broken market
    function _assertValidTokenPair(address quoteToken, address baseToken) internal pure {
        if (quoteToken == baseToken) revert InvalidPair();
        if (quoteToken == address(0)) revert InvalidTokenAddress();
        if (baseToken == address(0)) revert InvalidTokenAddress();
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            PRIVATE HELPERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Event helper that prevents stack from blowing without IR
    function _emitMarketCreated(
        address creator,
        address marketAddress,
        uint8 quoteDecimals,
        uint8 baseDecimals,
        ConfigParams memory config,
        SettingsParams calldata settings
    ) internal {
        emit MarketCreated(
            CLOBEventNonce.inc(),
            creator,
            config.baseToken,
            config.quoteToken,
            marketAddress,
            quoteDecimals,
            baseDecimals,
            config,
            settings
        );
    }

    /// @dev Gets the token hash which can be used as a UID for a market
    function _getTokenHash(address tokenA, address tokenB) internal pure returns (bytes32) {
        (tokenA, tokenB) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);

        return keccak256(abi.encodePacked(tokenA, tokenB));
    }

    /// @dev Helper to set the storage slot of the storage struct for this contract
    function _getStorage() internal pure returns (CLOBManagerStorage storage ds) {
        return CLOBManagerStorageLib.getCLOBManagerStorage();
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {OwnableRoles} from "@solady/auth/OwnableRoles.sol";
import {EnumerableSetLib} from "@solady/utils/EnumerableSetLib.sol";
import {DynamicArrayLib} from "@solady/utils/DynamicArrayLib.sol";
import {SafeCastLib} from "@solady/utils/SafeCastLib.sol";

import {Status, BookType, Side} from "../types/Enums.sol";
import {PackedFeeRates} from "../types/PackedFeeRatesLib.sol";
import {Constants} from "../types/Constants.sol";
import {StorageLib} from "../types/StorageLib.sol";

import {IViewPort} from "../interfaces/IViewPort.sol";

import {ClearingHouse} from "../types/ClearingHouse.sol";
import {Market, MarketMetadata} from "../types/Market.sol";
import {FundingRateSettings} from "../types/FundingRateEngine.sol";
import {Book} from "../types/Book.sol";
import {Position} from "../types/Position.sol";
import {Order, OrderIdLib} from "../types/Order.sol";

abstract contract ViewPort is IViewPort, OwnableRoles {
    using EnumerableSetLib for EnumerableSetLib.Bytes32Set;
    using DynamicArrayLib for DynamicArrayLib.DynamicArray;
    using SafeCastLib for uint256;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ACCOUNT
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getPosition(bytes32 asset, address account, uint256 subaccount)
        external
        view
        returns (Position memory position)
    {
        return StorageLib.loadMarket(asset).getPosition(account, subaccount);
    }

    function getPositionLeverage(bytes32 asset, address account, uint256 subaccount)
        external
        view
        returns (uint256 leverage)
    {
        return StorageLib.loadMarket(asset).getPositionLeverage(account, subaccount);
    }

    function getAssets(address account, uint256 subaccount) external view returns (bytes32[] memory positions) {
        return StorageLib.loadClearingHouse().assets[account][subaccount].values();
    }

    function getReduceOnlyOrders(bytes32 asset, address account, uint256 subaccount)
        external
        view
        returns (uint256[] memory orderIds)
    {
        return StorageLib.loadMarket(asset).reduceOnlyOrders[account][subaccount];
    }

    function getMarginBalance(address account, uint256 subaccount) external view returns (int256 marginBalance) {
        return StorageLib.loadCollateralManager().getMarginBalance(account, subaccount);
    }

    function getFreeCollateralBalance(address account) external view returns (uint256 collateralBalance) {
        return StorageLib.loadCollateralManager().getFreeCollateralBalance(account);
    }

    function getPendingFundingPayment(address account, uint256 subaccount)
        external
        view
        returns (int256 pendingFundingPayment)
    {
        return StorageLib.loadClearingHouse().getFundingPayment(account, subaccount);
    }

    function getOrderbookNotional(bytes32 asset, address account, uint256 subaccount)
        external
        view
        returns (uint256 orderbookNotional)
    {
        return StorageLib.loadMarket(asset).orderbookNotional[account][subaccount];
    }

    function getOrderbookCollateral(address account, uint256 subaccount)
        external
        view
        returns (uint256 orderbookCollateral)
    {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();
        bytes32[] memory assets = clearingHouse.assets[account][subaccount].values();

        for (uint256 i; i < assets.length; ++i) {
            orderbookCollateral += clearingHouse.market[assets[i]].getOrderBookCollateral(account, subaccount);
        }
    }

    function getAccountValue(address account, uint256 subaccount) external view returns (int256 accountValue) {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        (DynamicArrayLib.DynamicArray memory assets, Position[] memory positions, int256 margin) =
            clearingHouse.getAccountAndMargin(account, subaccount);

        int256 fundingPayment = clearingHouse.getFundingPayment(account, subaccount);
        int256 upnl = clearingHouse.getUpnl(assets, positions);

        return margin - fundingPayment + upnl;
    }

    function isLiquidatable(address account, uint256 subaccount) external view returns (bool liquidatable) {
        return StorageLib.loadClearingHouse().isLiquidatable(account, subaccount, BookType.STANDARD);
    }

    function isLiquidatableBackstop(address account, uint256 subaccount) external view returns (bool liquidatable) {
        return StorageLib.loadClearingHouse().isLiquidatable(account, subaccount, BookType.BACKSTOP);
    }

    function getMaintenanceMargin(bytes32 asset, uint256 positionAmount)
        external
        view
        returns (uint256 maintenanceMargin)
    {
        return StorageLib.loadMarket(asset).getMaintenanceMargin(positionAmount);
    }

    function getIntendedMarginAndUpnl(bytes32 asset, Position memory position)
        external
        view
        returns (uint256 intendedMargin, int256 upnl)
    {
        return StorageLib.loadMarket(asset).getIntendedMarginAndUpnl(position);
    }

    function getNextEmptySubaccount(address account) external view returns (uint256 subaccount) {
        ClearingHouse storage clearingHouse = StorageLib.loadClearingHouse();

        for (uint256 i = 1; i < type(uint256).max; ++i) {
            if (clearingHouse.assets[account][i].length() == 0) return i;
        }
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ORDERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getLimitOrder(bytes32 asset, uint256 orderId) external view returns (Order memory order) {
        return StorageLib.loadBook(asset).orders[OrderIdLib.wrap(orderId)];
    }

    function getLimitOrderBackstop(bytes32 asset, uint256 orderId) external view returns (Order memory order) {
        return StorageLib.loadBackstopBook(asset).orders[OrderIdLib.wrap(orderId)];
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               PROTOCOL
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getCollateralAsset() external pure returns (address) {
        return Constants.USDC;
    }

    function getTakerFeeRates() external view returns (PackedFeeRates) {
        return StorageLib.loadFeeManager().takerFeeRates;
    }

    function getMakerFeeRates() external view returns (PackedFeeRates) {
        return StorageLib.loadFeeManager().makerFeeRates;
    }

    function getInsuranceFundBalance() external view returns (uint256 insuranceFundBalance) {
        return StorageLib.loadInsuranceFund().balance;
    }

    function isAdmin(address account) external view returns (bool) {
        return hasAllRoles(account, 7);
    }

    function getNonce() external view returns (uint256 nonce) {
        return StorageLib.loadNonce();
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                              MARKET DATA
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getMarkPrice(bytes32 asset) external view returns (uint256 markPrice) {
        return StorageLib.loadMarket(asset).markPrice;
    }

    function getIndexPrice(bytes32 asset) external view returns (uint256 indexPrice) {
        return StorageLib.loadMarketMetadata(asset).indexPriceHistory.latest();
    }

    function getFundingRate(bytes32 asset) external view returns (int256 fundingRate) {
        return StorageLib.loadFundingRateEngine(asset).fundingRate;
    }

    function getCumulativeFunding(bytes32 asset) external view returns (int256 cumulativeFunding) {
        return StorageLib.loadFundingRateEngine(asset).cumulativeFundingIndex;
    }

    function getLastFundingTime(bytes32 asset) external view returns (uint256 lastFundingTime) {
        return StorageLib.loadFundingRateEngine(asset).lastFundingTime;
    }

    function getCurrentFundingInterval(bytes32 asset) external view returns (uint256) {
        return StorageLib.loadFundingRateEngine(asset).getFundingInterval(asset);
    }

    function getOpenInterest(bytes32 asset) external view returns (uint256 longOi, uint256 shortOi) {
        MarketMetadata storage metadata = StorageLib.loadMarketMetadata(asset);

        longOi = metadata.longOI;
        shortOi = metadata.shortOI;
    }

    function getOpenInterestBook(bytes32 asset) external view returns (uint256 baseOi, uint256 quoteOi) {
        Book storage book = StorageLib.loadBook(asset);

        baseOi = book.metadata.baseOI;
        quoteOi = book.metadata.quoteOI;
    }

    function getOpenInterestBackstopBook(bytes32 asset) external view returns (uint256 baseOi, uint256 quoteOi) {
        Book storage book = StorageLib.loadBackstopBook(asset);

        baseOi = book.metadata.baseOI;
        quoteOi = book.metadata.quoteOI;
    }

    function getNumBids(bytes32 asset) external view returns (uint256 numBids) {
        return StorageLib.loadBook(asset).metadata.numBids;
    }

    function getNumBidsBackstop(bytes32 asset) external view returns (uint256 numBids) {
        return StorageLib.loadBackstopBook(asset).metadata.numBids;
    }

    function getNumAsks(bytes32 asset) external view returns (uint256 numAsks) {
        return StorageLib.loadBook(asset).metadata.numAsks;
    }

    function getNumAsksBackstop(bytes32 asset) external view returns (uint256 numAsks) {
        return StorageLib.loadBackstopBook(asset).metadata.numAsks;
    }

    function getNextOrderId(bytes32 asset) external view returns (uint96 orderIdCounter) {
        return StorageLib.loadBook(asset).metadata.orderIdCounter + 1;
    }

    function getNextOrderIdBackstop(bytes32 asset) external view returns (uint96 orderIdCounter) {
        return StorageLib.loadBackstopBook(asset).metadata.orderIdCounter + 1;
    }

    function getMidPrice(bytes32 asset) external view returns (uint256 midPrice) {
        return StorageLib.loadMarket(asset).getMidPrice();
    }

    function quoteBookInBase(bytes32 asset, uint256 baseAmount, Side side)
        external
        view
        returns (uint256 quoteAmount, uint256 baseUsed)
    {
        if (side == Side.BUY) return StorageLib.loadBook(asset).quoteBidInBase(baseAmount);
        else return StorageLib.loadBook(asset).quoteAskInBase(baseAmount);
    }

    function quoteBookInQuote(bytes32 asset, uint256 quoteAmount, Side side)
        external
        view
        returns (uint256 baseAmount, uint256 quoteUsed)
    {
        if (side == Side.BUY) return StorageLib.loadBook(asset).quoteBidInQuote(quoteAmount);
        else return StorageLib.loadBook(asset).quoteAskInQuote(quoteAmount);
    }

    function quoteBackstopBookInBase(bytes32 asset, uint256 baseAmount, Side side)
        external
        view
        returns (uint256 quoteAmount, uint256 baseUsed)
    {
        if (side == Side.BUY) return StorageLib.loadBackstopBook(asset).quoteBidInBase(baseAmount);
        else return StorageLib.loadBackstopBook(asset).quoteAskInBase(baseAmount);
    }

    function quoteBackstopBookInQuote(bytes32 asset, uint256 quoteAmount, Side side)
        external
        view
        returns (uint256 baseAmount, uint256 quoteUsed)
    {
        if (side == Side.BUY) return StorageLib.loadBackstopBook(asset).quoteBidInQuote(quoteAmount);
        else return StorageLib.loadBackstopBook(asset).quoteAskInQuote(quoteAmount);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            MARKET SETTINGS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getMarketStatus(bytes32 asset) external view returns (Status status) {
        return StorageLib.loadMarketSettings(asset).status;
    }

    function isCrossMarginEnabled(bytes32 asset) external view returns (bool crossMarginEnabled) {
        return StorageLib.loadMarketSettings(asset).crossMarginEnabled;
    }

    function getMaxLeverage(bytes32 asset) external view returns (uint256 maxLeverage) {
        return StorageLib.loadMarketSettings(asset).maxOpenLeverage;
    }

    function getMinMarginRatio(bytes32 asset) external view returns (uint256 minMarginRatio) {
        return StorageLib.loadMarket(asset).getMinMarginRatio(BookType.STANDARD);
    }

    function getMinMarginRatioBackstop(bytes32 asset) external view returns (uint256 minMarginRatio) {
        return StorageLib.loadMarket(asset).getMinMarginRatio(BookType.BACKSTOP);
    }

    function getLiquidationFeeRate(bytes32 asset) external view returns (uint256 liquidationFeeRate) {
        return StorageLib.loadMarketSettings(asset).liquidationFeeRate;
    }

    function getDivergenceCap(bytes32 asset) external view returns (uint256 divergenceCap) {
        return StorageLib.loadMarketSettings(asset).divergenceCap;
    }

    function getReduceOnlyCap(bytes32 asset) external view returns (uint256 reduceOnlyCap) {
        return StorageLib.loadMarketSettings(asset).reduceOnlyCap;
    }

    function getPartialLiquidationThreshold(bytes32 asset) external view returns (uint256 threshold) {
        return StorageLib.loadMarketSettings(asset).partialLiquidationThreshold;
    }

    function getPartialLiquidationRate(bytes32 asset) external view returns (uint256 rate) {
        return StorageLib.loadMarketSettings(asset).partialLiquidationRate;
    }

    function getFundingInterval(bytes32 asset) external view returns (uint256 fundingInterval) {
        return StorageLib.loadFundingRateSettings(asset).fundingInterval;
    }

    function getResetInterval(bytes32 asset) external view returns (uint256 resetInterval) {
        return StorageLib.loadFundingRateSettings(asset).resetInterval;
    }

    function getResetIterations(bytes32 asset) external view returns (uint256 resetIterations) {
        return StorageLib.loadFundingRateSettings(asset).resetIterations;
    }

    function getInterestRate(bytes32 asset) external view returns (int256 interestRate) {
        return StorageLib.loadFundingRateSettings(asset).interestRate;
    }

    function getFundingClamps(bytes32 asset) external view returns (uint256 innerClamp, uint256 outerClamp) {
        FundingRateSettings storage settings = StorageLib.loadFundingRateSettings(asset);
        return (settings.innerClamp, settings.outerClamp);
    }

    function getMaxNumOrders(bytes32 asset) external view returns (uint256 maxNumOrders) {
        return StorageLib.loadBookSettings(asset).maxNumOrders;
    }

    function getMaxLimitsPerTx(bytes32 asset) external view returns (uint8 maxLimitsPerTx) {
        return StorageLib.loadBookSettings(asset).maxLimitsPerTx;
    }

    function getMinLimitOrderAmountInBase(bytes32 asset) external view returns (uint256 minLimitOrderAmountInBase) {
        return StorageLib.loadBookSettings(asset).minLimitOrderAmountInBase;
    }

    function getTickSize(bytes32 asset) external view returns (uint256 tickSize) {
        return StorageLib.loadBookSettings(asset).tickSize;
    }

    function getLotSize(bytes32 asset) external view returns (uint256 lotSize) {
        return StorageLib.loadBook(asset).config.lotSize;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {IAccountManager} from "./IAccountManager.sol";
import {ICLOB} from "../clob/ICLOB.sol";
import {MakerCredit} from "../clob/types/TransientMakerData.sol";
import {IPerpManager} from "../perps/interfaces/IPerpManager.sol";
import {Side} from "../clob/types/Order.sol";
import {OperatorHelperLib} from "../utils/types/OperatorHelperLib.sol";
import {EventNonceLib as AccountEventNonce} from "../utils/types/EventNonce.sol";
import {Initializable} from "@solady/utils/Initializable.sol";
import {OwnableRoles} from "@solady/auth/OwnableRoles.sol";
import {SafeTransferLib} from "@solady/utils/SafeTransferLib.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {OperatorPanel, SpotOperatorRoles} from "../utils/OperatorPanel.sol";
import {
    FeeData,
    FeeDataLib,
    FeeDataStorageLib,
    PackedFeeRates,
    PackedFeeRatesLib,
    FeeTiers
} from "../clob/types/FeeData.sol";

struct AccountManagerStorage {
    mapping(address market => bool) isMarket;
    mapping(address account => mapping(address asset => uint256)) accountTokenBalances;
}

/**
 * @title AccountManager
 * @notice Handles account balances, deposits, withdrawals, for GTE spot as well as inheriting Operator
 */
contract AccountManager is IAccountManager, OperatorPanel, Initializable, OwnableRoles {
    using SafeTransferLib for address;
    using FixedPointMathLib for uint256;
    using PackedFeeRatesLib for PackedFeeRates;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                EVENTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0x07796b317344e6f18fa32ed89b6074ad66549cee7fb7b8c3e9f1c42c496f1c5c
    event MarketRegistered(uint256 indexed eventNonce, address indexed market);
    /// @dev sig: 0x1ae35cf838a52070167575d4dedf6631cc160136bee10eeca1575d2e3cc8a075
    event AccountDebited(uint256 indexed eventNonce, address indexed account, address indexed token, uint256 amount);
    /// @dev sig: 0x074f9f8975d437bea257b7e6abcfb4b45312683f7f8f120dde3faae76f783b58
    event AccountCredited(uint256 indexed eventNonce, address indexed account, address indexed token, uint256 amount);

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ERRORS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev sig: 0x00b8f216
    error BalanceInsufficient();
    /// @dev sig: 0x467cb8b4
    error GTERouterUnauthorized();
    /// @dev sig: 0x30eee8ba
    error CLOBManagerUnauthorized();
    /// @dev sig: 0x9d1c9c18
    error MarketUnauthorized();
    /// @dev sig: 0x38422dcd
    error UnmatchingArrayLengths();
    error NotPerpManager();

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            IMMUTABLE STATE
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    uint256 public constant FEE_COLLECTOR = 1;

    /// @dev The global router address that can bypass the operator check
    address public immutable gteRouter;
    /// @dev The CLOBManager address that can call settlement functions
    address public immutable clobManager;
    /// @dev Packed spot maker fee rates for all tiers
    PackedFeeRates public immutable spotMakerFeeRates;
    /// @dev Packed spot taker fee rates for all tiers
    PackedFeeRates public immutable spotTakerFeeRates;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                MODIFIERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @dev Ensures msg.sender is a registered market
    modifier onlyMarket() {
        if (!_getAccountStorage().isMarket[msg.sender]) revert MarketUnauthorized();
        _;
    }

    /// @dev Ensures msg.sender is the router
    modifier onlyGTERouter() {
        if (msg.sender != gteRouter) revert GTERouterUnauthorized();
        _;
    }

    /// @dev Ensures msg.sender is the CLOBManager
    modifier onlyCLOBManager() {
        if (msg.sender != clobManager) revert CLOBManagerUnauthorized();
        _;
    }

    /// @dev Ensures that if an account is not the msg.sender, both that account and the owner have approved msg.sender
    modifier onlySenderOrOperator(address account, SpotOperatorRoles requiredRole) {
        OperatorHelperLib.onlySenderOrOperator(_getOperatorStorage(), gteRouter, account, requiredRole);
        _;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                CONSTRUCTOR
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    constructor(
        address _gteRouter,
        address _clobManager,
        address _operatorHub,
        uint16[] memory _spotMakerFees,
        uint16[] memory _spotTakerFees,
        address _perpManager
    ) OperatorPanel(_operatorHub) {
        gteRouter = _gteRouter;
        clobManager = _clobManager;
        spotMakerFeeRates = PackedFeeRatesLib.packFeeRates(_spotMakerFees);
        spotTakerFeeRates = PackedFeeRatesLib.packFeeRates(_spotTakerFees);
        perpManager = IPerpManager(_perpManager);
        _disableInitializers();
    }

    /// @dev Initializes the contract
    function initialize(address _owner) external initializer {
        _initializeOwner(_owner);
    }

    IPerpManager immutable perpManager;

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            EXTERNAL GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Gets an `account`'s balance of `token`
    function getAccountBalance(address account, address token) external view returns (uint256) {
        return _getAccountStorage().accountTokenBalances[account][token];
    }

    /// @notice Gets the current event nonce
    function getEventNonce() external view returns (uint256) {
        return AccountEventNonce.getCurrentNonce();
    }

    /// @notice Gets the total fees collected for a token
    function getTotalFees(address token) external view returns (uint256) {
        return FeeDataStorageLib.getFeeDataStorage().totalFees[token];
    }

    /// @notice Gets the unclaimed fees for a token
    function getUnclaimedFees(address token) external view returns (uint256) {
        return FeeDataStorageLib.getFeeDataStorage().unclaimedFees[token];
    }

    /// @notice Gets the fee tier for an account
    function getFeeTier(address account) external view returns (FeeTiers) {
        return FeeDataStorageLib.getFeeDataStorage().getAccountFeeTier(account);
    }

    /// @notice Gets the spot taker fee rate for a given fee tier
    function getSpotTakerFeeRateForTier(FeeTiers tier) external view returns (uint256) {
        return spotTakerFeeRates.getFeeAt(uint256(tier));
    }

    /// @notice Gets the spot maker fee rate for a given fee tier
    function getSpotMakerFeeRateForTier(FeeTiers tier) external view returns (uint256) {
        return spotMakerFeeRates.getFeeAt(uint256(tier));
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ACCOUNTS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Deposits via transfer from the account
    function deposit(address account, address token, uint256 amount)
        external
        virtual
        onlySenderOrOperator(account, SpotOperatorRoles.SPOT_DEPOSIT)
    {
        _creditAccount(_getAccountStorage(), account, token, amount);
        token.safeTransferFrom(account, address(this), amount);
    }

    function depositTo(address account, address token, uint256 amount)
        external
    {
        _creditAccount(_getAccountStorage(), account, token, amount);
        token.safeTransferFrom(msg.sender, address(this), amount);
    }

    function depositFromPerps(address account, uint256 amount)
        external
        onlySenderOrOperator(account, SpotOperatorRoles.PERP_TO_SPOT_DEPOSIT)
    {
        perpManager.withdrawToSpot(account, amount);
        _creditAccount(_getAccountStorage(), account, perpManager.getCollateralAsset(), amount);
    }

    /// @notice Deposits via transfer from the router
    function depositFromRouter(address account, address token, uint256 amount) external onlyGTERouter {
        _creditAccount(_getAccountStorage(), account, token, amount);
        token.safeTransferFrom(gteRouter, address(this), amount);
    }

    /// @notice Withdraws to account
    function withdraw(address account, address token, uint256 amount)
        external
        virtual
        onlySenderOrOperator(account, SpotOperatorRoles.SPOT_WITHDRAW)
    {
        _debitAccount(_getAccountStorage(), account, token, amount);
        token.safeTransfer(account, amount);
    }

    function withdrawToPerps(address account, uint256 amount) external {
        if (msg.sender != address(perpManager)) revert NotPerpManager();

        address token = perpManager.getCollateralAsset();

        _debitAccount(_getAccountStorage(), account, token, amount);
        token.safeTransfer(address(perpManager), amount);
    }

    /// @notice Withdraws from account to router
    function withdrawToRouter(address account, address token, uint256 amount) external onlyGTERouter {
        _debitAccount(_getAccountStorage(), account, token, amount);
        token.safeTransfer(gteRouter, amount);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ADMIN
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice Registers a market address, can only be called by CLOBManager
    function registerMarket(address market) external onlyCLOBManager {
        _getAccountStorage().isMarket[market] = true;
        emit MarketRegistered(AccountEventNonce.inc(), market);
    }

    /// @notice Collects accrued fees for a token and transfers to recipient
    function collectFees(address token, address feeRecipient)
        external
        virtual
        onlyOwnerOrRoles(FEE_COLLECTOR)
        returns (uint256 fee)
    {
        FeeData storage feeData = FeeDataStorageLib.getFeeDataStorage();
        fee = feeData.claimFees(token);

        if (fee > 0) {
            // Transfer fees directly from contract balance to recipient
            token.safeTransfer(feeRecipient, fee);
        }
    }

    /// @notice Sets the spot fee tier for a single account, can only be called by CLOBManager
    function setSpotAccountFeeTier(address account, FeeTiers feeTier) external virtual onlyCLOBManager {
        FeeData storage feeData = FeeDataStorageLib.getFeeDataStorage();
        feeData.setAccountFeeTier(account, feeTier);
    }

    /// @notice Sets the spot fee tiers for multiple accounts, can only be called by CLOBManager
    function setSpotAccountFeeTiers(address[] calldata accounts, FeeTiers[] calldata feeTiers)
        external
        virtual
        onlyCLOBManager
    {
        if (accounts.length != feeTiers.length) revert UnmatchingArrayLengths();

        FeeData storage feeData = FeeDataStorageLib.getFeeDataStorage();
        for (uint256 i = 0; i < accounts.length; i++) {
            feeData.setAccountFeeTier(accounts[i], feeTiers[i]);
        }
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                SETTLEMENT
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    /// @notice The hook for markets to perform account settlement after a fill, including fee calculations
    function settleIncomingOrder(ICLOB.SettleParams calldata params)
        external
        virtual
        onlyMarket
        returns (uint256 takerFee)
    {
        AccountManagerStorage storage self = _getAccountStorage();
        FeeData storage feeData = FeeDataStorageLib.getFeeDataStorage();

        // Credit taker less fee
        address takerFeeToken;
        if (params.side == Side.BUY) {
            takerFee = feeData.getTakerFee(spotTakerFeeRates, params.taker, params.takerBaseAmount);
            takerFeeToken = params.baseToken;

            // Taker settlement
            _debitAccount(self, params.taker, params.quoteToken, params.takerQuoteAmount);
            _creditAccount(self, params.taker, params.baseToken, params.takerBaseAmount - takerFee);
        } else {
            takerFee = feeData.getTakerFee(spotTakerFeeRates, params.taker, params.takerQuoteAmount);
            takerFeeToken = params.quoteToken;

            // Taker settlement
            _debitAccount(self, params.taker, params.baseToken, params.takerBaseAmount);
            _creditAccount(self, params.taker, params.quoteToken, params.takerQuoteAmount - takerFee);
        }

        // Accrue taker fee
        if (takerFee > 0) feeData.accrueFee(takerFeeToken, takerFee);

        // Process maker settlement and fees
        uint256 currMakerFee = 0;
        uint256 totalQuoteMakerFee = 0;
        uint256 totalBaseMakerFee = 0;

        for (uint256 i; i < params.makerCredits.length; ++i) {
            MakerCredit memory credit = params.makerCredits[i];

            // Calculate fees only for the matching side
            if (params.side == Side.BUY && credit.quoteAmount > 0) {
                currMakerFee = feeData.getMakerFee(spotMakerFeeRates, credit.maker, credit.quoteAmount);
                credit.quoteAmount -= currMakerFee;
                totalQuoteMakerFee += currMakerFee;
            } else if (params.side == Side.SELL && credit.baseAmount > 0) {
                currMakerFee = feeData.getMakerFee(spotMakerFeeRates, credit.maker, credit.baseAmount);
                credit.baseAmount -= currMakerFee;
                totalBaseMakerFee += currMakerFee;
            }

            // Credit both base and quote amounts if any (not just fills less fee, but also expiry and non-competitive refunds)
            if (credit.baseAmount > 0) _creditAccountNoEvent(self, credit.maker, params.baseToken, credit.baseAmount);

            if (credit.quoteAmount > 0) {
                _creditAccountNoEvent(self, credit.maker, params.quoteToken, credit.quoteAmount);
            }
        }

        // Accrue total collected maker fees
        if (totalBaseMakerFee > 0) feeData.accrueFee(params.baseToken, totalBaseMakerFee);
        if (totalQuoteMakerFee > 0) feeData.accrueFee(params.quoteToken, totalQuoteMakerFee);
    }

    /// @notice Credits account, called by markets for amends/cancels
    function creditAccount(address account, address token, uint256 amount) external virtual onlyMarket {
        _creditAccount(_getAccountStorage(), account, token, amount);
    }

    /// @notice Credits account without event, called by markets for non-competitive order removal
    function creditAccountNoEvent(address account, address token, uint256 amount) external virtual onlyMarket {
        _creditAccountNoEvent(_getAccountStorage(), account, token, amount);
    }

    /// @notice Debits account, called by markets for amends
    function debitAccount(address account, address token, uint256 amount) external virtual onlyMarket {
        _debitAccount(_getAccountStorage(), account, token, amount);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            INTERNAL HELPERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function _creditAccount(AccountManagerStorage storage self, address account, address token, uint256 amount)
        internal
    {
        unchecked {
            self.accountTokenBalances[account][token] += amount;
        }
        emit AccountCredited(AccountEventNonce.inc(), account, token, amount);
    }

    function _creditAccountNoEvent(AccountManagerStorage storage self, address account, address token, uint256 amount)
        internal
    {
        unchecked {
            self.accountTokenBalances[account][token] += amount;
        }
    }

    function _debitAccount(AccountManagerStorage storage self, address account, address token, uint256 amount)
        internal
    {
        if (self.accountTokenBalances[account][token] < amount) revert BalanceInsufficient();

        unchecked {
            self.accountTokenBalances[account][token] -= amount;
        }
        emit AccountDebited(AccountEventNonce.inc(), account, token, amount);
    }

    /// @dev Helper to set the storage slot of the storage struct for this contract
    function _getAccountStorage() internal pure returns (AccountManagerStorage storage ds) {
        return AccountManagerStorageLib.getAccountManagerStorage();
    }
}

using AccountManagerStorageLib for AccountManagerStorage global;

/// @custom:storage-location erc7201:AccountManagerStorage
library AccountManagerStorageLib {
    bytes32 constant ACCOUNT_MANAGER_STORAGE_POSITION =
        keccak256(abi.encode(uint256(keccak256("AccountManagerStorage")) - 1)) & ~bytes32(uint256(0xff));

    /// @dev Gets the storage slot of the storage struct for the contract calling this library function
    // slither-disable-next-line uninitialized-storage
    function getAccountManagerStorage() internal pure returns (AccountManagerStorage storage self) {
        bytes32 position = ACCOUNT_MANAGER_STORAGE_POSITION;

        // slither-disable-next-line assembly
        assembly {
            self.slot := position
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {ICLOB, MakerCredit} from "../clob/ICLOB.sol";
import {FeeTiers} from "../clob/types/FeeData.sol";

/**
 * @title IAccountManager
 * @notice Interface defining account management functions
 */
interface IAccountManager {
    // Getters
    function getAccountBalance(address account, address token) external view returns (uint256);
    function getEventNonce() external view returns (uint256);
    function getTotalFees(address token) external view returns (uint256);
    function getUnclaimedFees(address token) external view returns (uint256);
    function getFeeTier(address account) external view returns (FeeTiers);
    function getSpotTakerFeeRateForTier(FeeTiers tier) external view returns (uint256);
    function getSpotMakerFeeRateForTier(FeeTiers tier) external view returns (uint256);

    // OperatorPanel functions (inherited from OperatorPanel.sol)

    // Accounts
    function deposit(address account, address token, uint256 amount) external;
    function withdraw(address account, address token, uint256 amount) external;
    function depositFromPerps(address account, uint256 amount) external;
    function withdrawToPerps(address account, uint256 amount) external;
    function depositFromRouter(address account, address token, uint256 amount) external;
    function withdrawToRouter(address account, address token, uint256 amount) external;

    // Admin called during market creation by CLOBManager
    function registerMarket(address market) external;

    // Settlement called by markets directly
    function settleIncomingOrder(ICLOB.SettleParams calldata params) external returns (uint256 takerFee);

    // Fee collection and management
    function collectFees(address token, address feeRecipient) external returns (uint256 fee);
    function setSpotAccountFeeTier(address account, FeeTiers feeTier) external;
    function setSpotAccountFeeTiers(address[] calldata accounts, FeeTiers[] calldata feeTiers) external;

    // Direct market operations called by CLOB (market) contracts
    function creditAccount(address account, address token, uint256 amount) external;
    function creditAccountNoEvent(address account, address token, uint256 amount) external;
    function debitAccount(address account, address token, uint256 amount) external;
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
pragma solidity ^0.8.27;

interface IOperatorPanel {
    function approveOperator(address account, address operator, uint256 roles) external;
    function disapproveOperator(address account, address operator, uint256 roles) external;
    function getOperatorRoleApprovals(address account, address operator) external view returns (uint256);
    function getOperatorEventNonce() external view returns (uint256);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Status, Side} from "../types/Enums.sol";

import {Position} from "../types/Position.sol";
import {Order} from "../types/Order.sol";
import {PackedFeeRates} from "../types/PackedFeeRatesLib.sol";

interface IViewPort {
    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ACCOUNT
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getPosition(bytes32 asset, address account, uint256 subaccount) external view returns (Position memory);

    function getAssets(address account, uint256 subaccount) external view returns (bytes32[] memory);

    function getMarginBalance(address account, uint256 subaccount) external view returns (int256);

    function getFreeCollateralBalance(address account) external view returns (uint256);

    function getOrderbookCollateral(address account, uint256 subaccount)
        external
        view
        returns (uint256 orderbookCollateral);

    function getPositionLeverage(bytes32 asset, address account, uint256 subaccount)
        external
        view
        returns (uint256 leverage);

    function getAccountValue(address account, uint256 subaccount) external view returns (int256);

    function isLiquidatable(address account, uint256 subaccount) external view returns (bool);

    function isLiquidatableBackstop(address account, uint256 subaccount) external view returns (bool);

    function getReduceOnlyOrders(bytes32 asset, address account, uint256 subaccount)
        external
        view
        returns (uint256[] memory orderIds);

    function getPendingFundingPayment(address account, uint256 subaccount) external view returns (int256);

    function getOrderbookNotional(bytes32 asset, address account, uint256 subaccount) external view returns (uint256);

    function getMaintenanceMargin(bytes32 asset, uint256 positionAmount) external view returns (uint256);

    function getIntendedMarginAndUpnl(bytes32 asset, Position memory position)
        external
        view
        returns (uint256, int256);

    function getNextEmptySubaccount(address account) external view returns (uint256 subaccount);

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                ORDERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getLimitOrder(bytes32 asset, uint256 orderId) external view returns (Order memory);

    function getLimitOrderBackstop(bytes32 asset, uint256 orderId) external view returns (Order memory);

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                               PROTOCOL
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getCollateralAsset() external pure returns (address);

    function getTakerFeeRates() external view returns (PackedFeeRates);

    function getMakerFeeRates() external view returns (PackedFeeRates);

    function getInsuranceFundBalance() external view returns (uint256);

    function isAdmin(address account) external view returns (bool);

    function getNonce() external view returns (uint256);

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                              MARKET DATA
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getMarkPrice(bytes32 asset) external view returns (uint256);

    function getIndexPrice(bytes32 asset) external view returns (uint256);

    function getFundingRate(bytes32 asset) external view returns (int256);

    function getCumulativeFunding(bytes32 asset) external view returns (int256);

    function getLastFundingTime(bytes32 asset) external view returns (uint256);

    function getOpenInterest(bytes32 asset) external view returns (uint256 longOi, uint256 shortOi);

    function getOpenInterestBook(bytes32 asset) external view returns (uint256 baseOi, uint256 quoteOi);

    function getOpenInterestBackstopBook(bytes32 asset) external view returns (uint256 baseOi, uint256 quoteOi);

    function getNumBids(bytes32 asset) external view returns (uint256);

    function getNumBidsBackstop(bytes32 asset) external view returns (uint256);

    function getNumAsks(bytes32 asset) external view returns (uint256);

    function getNumAsksBackstop(bytes32 asset) external view returns (uint256);

    function getNextOrderId(bytes32 asset) external view returns (uint96);

    function getNextOrderIdBackstop(bytes32 asset) external view returns (uint96);

    function getMidPrice(bytes32 asset) external view returns (uint256);

    function quoteBookInBase(bytes32 asset, uint256 baseAmount, Side side)
        external
        view
        returns (uint256 quoteAmount, uint256 baseUsed);

    function quoteBookInQuote(bytes32 asset, uint256 quoteAmount, Side side)
        external
        view
        returns (uint256 baseAmount, uint256 quoteUsed);

    function quoteBackstopBookInBase(bytes32 asset, uint256 baseAmount, Side side)
        external
        view
        returns (uint256 quoteAmount, uint256 baseUsed);

    function quoteBackstopBookInQuote(bytes32 asset, uint256 quoteAmount, Side side)
        external
        view
        returns (uint256 baseAmount, uint256 quoteUsed);

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                            MARKET SETTINGS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getMarketStatus(bytes32 asset) external view returns (Status);

    function isCrossMarginEnabled(bytes32 asset) external view returns (bool);

    function getMaxLeverage(bytes32 asset) external view returns (uint256);

    function getMinMarginRatio(bytes32 asset) external view returns (uint256);

    function getMinMarginRatioBackstop(bytes32 asset) external view returns (uint256);

    function getLiquidationFeeRate(bytes32 asset) external view returns (uint256);

    function getDivergenceCap(bytes32 asset) external view returns (uint256);

    function getReduceOnlyCap(bytes32 asset) external view returns (uint256);

    function getPartialLiquidationThreshold(bytes32 asset) external view returns (uint256);

    function getPartialLiquidationRate(bytes32 asset) external view returns (uint256);

    function getCurrentFundingInterval(bytes32 asset) external view returns (uint256);

    function getFundingInterval(bytes32 asset) external view returns (uint256);

    function getResetInterval(bytes32 asset) external view returns (uint256);

    function getResetIterations(bytes32 asset) external view returns (uint256);

    function getInterestRate(bytes32 asset) external view returns (int256);

    function getFundingClamps(bytes32 asset) external view returns (uint256 innerClamp, uint256 outerClamp);

    function getMaxNumOrders(bytes32 asset) external view returns (uint256);

    function getMaxLimitsPerTx(bytes32 asset) external view returns (uint8);

    function getMinLimitOrderAmountInBase(bytes32 asset) external view returns (uint256);

    function getTickSize(bytes32 asset) external view returns (uint256);

    function getLotSize(bytes32 asset) external view returns (uint256);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IAccountManager} from "../account-manager/IAccountManager.sol";
import {FeeTiers} from "./types/FeeData.sol";
import {ICLOB} from "./ICLOB.sol";
import {Side, OrderId} from "./types/Order.sol";
import {MakerCredit} from "./types/TransientMakerData.sol";

struct ConfigParams {
    address quoteToken;
    address baseToken;
    uint256 quoteSize;
    uint256 baseSize;
}

struct SettingsParams {
    address owner;
    uint8 maxLimitsPerTx;
    uint256 minLimitOrderAmountInBase;
    uint256 tickSize;
    uint256 lotSizeInBase;
}

interface ICLOBManager {
    // Basic getters from ICLOBAdminPanel
    function beacon() external view returns (address);
    function getMarketAddress(address quoteToken, address baseToken) external view returns (address);
    function isMarket(address market) external view returns (bool);

    // Market creation and management from ICLOBAdminPanel
    function createMarket(address baseToken, address quoteToken, SettingsParams calldata settings)
        external
        returns (address marketAddress);

    // Limit management getters
    function getMaxLimitExempt(address account) external view returns (bool);

    // Admin settings
    function setMaxLimitsPerTx(ICLOB market, uint8 newMaxLimits) external;
    function setTickSize(ICLOB market, uint256 newTickSize) external;
    function setLotSizeInBase(ICLOB market, uint256 newLotSize) external;
    function setMinLimitOrderAmountInBase(ICLOB market, uint256 newMinLimitOrderAmountInBase) external;
    function adminCancelExpiredOrders(ICLOB market, OrderId[] calldata ids, Side side) external;
    function setAccountFeeTiers(address[] calldata accounts, FeeTiers[] calldata feeTiers) external;
    function setMaxLimitsExempt(address[] calldata accounts, bool[] calldata toggles) external;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Side, Order, OrderId} from "./types/Order.sol";
import {MarketConfig, MarketSettings, Limit} from "./types/Book.sol";
import {MakerCredit} from "./types/TransientMakerData.sol";
import {ICLOBManager} from "./ICLOBManager.sol";

interface ICLOB {
    struct SettleParams {
        Side side;
        address taker;
        uint256 takerBaseAmount;
        uint256 takerQuoteAmount;
        address baseToken;
        address quoteToken;
        MakerCredit[] makerCredits;
    }

    enum TiF {
        // MAKER
        GTC, // good-till-cancelled
        MOC, // maker-or-cancel (post-only)
        // TAKER-ONLY
        FOK, // fill-or-kill
        IOC // immediate-or-cancel

    }

    struct PlaceOrderArgs {
        // metadata
        Side side; // bid / ask
        uint96 clientOrderId; // Optional user-defined id for makes
        // time / execution
        TiF tif; // time in force
        uint32 expiryTime; // optional auto-cancel time (only for GTC, MOC)
        // price
        uint256 limitPrice; // if 0, market order
        // size
        uint256 amount;
        bool baseDenominated; // which asset the amount denominates
    }

    struct PlaceOrderResult {
        address account;
        uint256 orderId;
        uint256 basePosted; // amount posted in base (for maker orders)
        int256 quoteTokenAmountTraded; // negative if outgoing, positive if incoming
        int256 baseTokenAmountTraded; // negative if outgoing, positive if incoming
        uint256 takerFee;
        bool wasMarketOrder; // true if market order (limitPrice = 0), false if limit order
    }

    enum CancelType {
        USER,
        EXPIRY,
        NON_COMPETITIVE
    }

    struct AmendArgs {
        uint256 orderId;
        uint256 amountInBase;
        uint256 price;
        uint32 cancelTimestamp;
        Side side;
    }

    struct CancelArgs {
        uint256[] orderIds;
    }

    function placeOrder(address account, PlaceOrderArgs calldata args) external returns (PlaceOrderResult memory);

    function amend(address account, AmendArgs memory args) external returns (int256 quoteDelta, int256 baseDelta);

    function cancel(address account, CancelArgs memory args) external returns (uint256, uint256); // quoteToken refunded, baseToken refunded

    // Token Amount Calculators
    function getQuoteTokenAmount(uint256 price, uint256 amountInBaseLots) external view returns (uint256);

    function getBaseTokenAmount(uint256 price, uint256 amountInBaseLots) external view returns (uint256);

    // Getters

    function maxNumOrdersPerSide() external view returns (uint256);

    function gteRouter() external view returns (address);

    function getQuoteToken() external view returns (address);

    function getBaseToken() external view returns (address);

    function getMarketConfig() external view returns (MarketConfig memory);

    function getTickSize() external view returns (uint256);

    function getLotSizeInBase() external view returns (uint256);

    function getOpenInterest() external view returns (uint256, uint256);

    function getOrder(uint256 orderId) external view returns (Order memory);

    function getTOB() external view returns (uint256, uint256);

    function getLimit(uint256 price, Side side) external view returns (Limit memory);

    function getNumBids() external view returns (uint256);

    function getNumAsks() external view returns (uint256);

    function getNextBiggestPrice(uint256 price, Side side) external view returns (uint256);

    function getNextSmallestPrice(uint256 price, Side side) external view returns (uint256);

    function getNextOrders(uint256 startOrderId, uint256 numOrders) external view returns (Order[] memory);

    function getNextOrderId() external view returns (uint256);

    function factory() external view returns (ICLOBManager);

    function getOrdersPaginated(uint256 startPrice, Side side, uint256 pageSize)
        external
        view
        returns (Order[] memory result, Order memory nextOrder);

    function getOrdersPaginated(OrderId startOrderId, uint256 pageSize)
        external
        view
        returns (Order[] memory result, Order memory nextOrder);

    function setLotSizeInBase(uint256 newLotSizeInBase) external;
    function setMaxLimitsPerTx(uint8 newMaxLimits) external;
    function setTickSize(uint256 newTickSize) external;
    function setMinLimitOrderAmountInBase(uint256 newMinLimitOrderAmountInBase) external;

    function adminCancelExpiredOrders(OrderId[] calldata ids, Side side) external returns (bool[] memory);
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
        self.balance += amount;
    }

    function claim(InsuranceFund storage self, uint256 amount) internal {
        if (amount == 0) return;
        if (self.balance < amount) revert InsufficientInsuranceFundBalance();
        self.balance -= amount;
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                 ADMIN
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function withdraw(InsuranceFund storage self, uint256 amount) internal {
        if (self.balance < amount) revert InsufficientInsuranceFundBalance();
        self.balance -= amount;
        USDC.safeTransfer(msg.sender, amount);
        emit InsuranceFundWithdrawal(msg.sender, amount);
    }

    function deposit(InsuranceFund storage self, uint256 amount) internal {
        self.balance += amount;
        USDC.safeTransferFrom(msg.sender, address(this), amount);
        emit InsuranceFundDeposit(msg.sender, amount);
    }

    /*▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
                                GETTERS
    ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀*/

    function getBalance(InsuranceFund storage self) internal view returns (uint256) {
        return self.balance;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {ICLOB} from "../ICLOB.sol";

struct MakerCredit {
    address maker;
    uint256 quoteAmount;
    uint256 baseAmount;
}

// slither-disable-start assembly
library TransientMakerData {
    bytes32 constant TRANSIENT_MAKERS_POSITION =
        keccak256(abi.encode(uint256(keccak256("TransientMakers")) - 1)) & ~bytes32(uint256(0xff));
    bytes32 constant TRANSIENT_CREDITS_POSITION =
        keccak256(abi.encode(uint256(keccak256("TransientCredits")) - 1)) & ~bytes32(uint256(0xff));

    /// @dev sig: 0xe47ec074
    error ArithmeticOverflow();

    /// @dev Adds a quote token to the transient maker data
    function addQuoteToken(address maker, uint256 quoteAmount) internal {
        bytes32 slot = keccak256(abi.encode(TRANSIENT_CREDITS_POSITION, maker));
        bytes4 err = ArithmeticOverflow.selector;

        bool exists;
        assembly ("memory-safe") {
            exists := iszero(iszero(tload(slot)))

            if iszero(exists) { tstore(slot, 1) }

            let balSlot := add(slot, 1)

            let oldVal := tload(balSlot)
            let newVal := add(oldVal, quoteAmount)

            if lt(newVal, oldVal) {
                mstore(0x00, err)
                revert(0x00, 0x04)
            }

            tstore(balSlot, newVal)
        }

        if (!exists) _addMaker(maker);
    }

    /// @dev Adds a base token to the transient maker data
    function addBaseToken(address maker, uint256 baseAmount) internal {
        bytes32 slot = keccak256(abi.encode(TRANSIENT_CREDITS_POSITION, maker));
        bytes4 err = ArithmeticOverflow.selector;

        bool exists;
        assembly ("memory-safe") {
            exists := iszero(iszero(tload(slot)))

            if iszero(exists) { tstore(slot, 1) }

            let balSlot := add(slot, 2)

            let oldVal := tload(balSlot)
            let newVal := add(oldVal, baseAmount)

            if lt(newVal, oldVal) {
                mstore(0x00, err)
                revert(0x00, 0x04)
            }

            tstore(balSlot, newVal)
        }

        if (!exists) _addMaker(maker);
    }

    /// @dev Gets the maker credits and clears the storage
    function getMakerCreditsAndClearStorage() internal returns (MakerCredit[] memory makerCredits) {
        address[] memory makers = _getMakersAndClear();

        uint256 length = makers.length;

        makerCredits = new MakerCredit[](length);

        uint256 quoteAmount;
        uint256 baseAmount;

        for (uint256 i; i < length; i++) {
            (quoteAmount, baseAmount) = _getBalancesAndClear(makers[i]);

            makerCredits[i] = MakerCredit({maker: makers[i], quoteAmount: quoteAmount, baseAmount: baseAmount});
        }
    }

    function _addMaker(address maker) internal {
        bytes32 slot = TRANSIENT_MAKERS_POSITION;

        assembly ("memory-safe") {
            let len := tload(slot)

            mstore(0x00, slot)

            let dataSlot := keccak256(0x00, 0x20)

            tstore(add(dataSlot, len), maker)
            tstore(slot, add(len, 1))
        }
    }

    function _getMakersAndClear() internal returns (address[] memory makers) {
        bytes32 slot = TRANSIENT_MAKERS_POSITION;

        assembly ("memory-safe") {
            let len := tload(slot)

            makers := mload(0x40)

            mstore(makers, len)

            mstore(0x00, slot)

            let dataSlot := keccak256(0x00, 0x20)
            let memPointer := add(makers, 0x20)

            for { let i := 0 } lt(i, len) { i := add(i, 1) } {
                mstore(add(memPointer, mul(i, 0x20)), tload(add(dataSlot, i)))
                tstore(add(dataSlot, i), 0) // clear maker
            }

            mstore(0x40, add(memPointer, mul(len, 0x20))) // idk the purpose of this tbh
            tstore(slot, 0) // clear length
        }
    }

    function _getBalancesAndClear(address maker) internal returns (uint256 quoteAmount, uint256 baseAmount) {
        bytes32 slot = keccak256(abi.encode(TRANSIENT_CREDITS_POSITION, maker));

        assembly ("memory-safe") {
            let quote := add(slot, 1)
            let base := add(slot, 2)
            let instant := 0
            let account := 1

            quoteAmount := tload(quote)
            baseAmount := tload(base)

            tstore(slot, 0)
            tstore(quote, 0)
            tstore(base, 0)
        }
    }
}
// slither-disable-end assembly

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

import {IOperatorPanel} from "./interfaces/IOperatorPanel.sol";
import {EventNonceLib as OperatorEventNonce} from "./types/EventNonce.sol";

// @todo rename "spot" to "account"
enum SpotOperatorRoles {
    ADMIN,
    PLACE_ORDER,
    SPOT_DEPOSIT,
    SPOT_WITHDRAW,
    PERP_TO_SPOT_DEPOSIT,
    LAUNCHPAD_FILL
}

enum PerpsOperatorRoles {
    ADMIN,
    PLACE_ORDER,
    SET_LEVERAGE,
    DEPOSIT_MARGIN,
    WITHDRAW_MARGIN,
    DEPOSIT_ACCOUNT,
    WITHDRAW_ACCOUNT,
    SPOT_TO_PERP_DEPOSIT
}

struct OperatorStorage {
    mapping(address account => mapping(address operator => uint256)) operatorRoleApprovals;
}

using OperatorStorageLib for OperatorStorage global;

/// @custom:storage-location erc7201:OperatorStorage
library OperatorStorageLib {
    bytes32 constant OPERATOR_STORAGE_POSITION =
        keccak256(abi.encode(uint256(keccak256("OperatorStorage")) - 1)) & ~bytes32(uint256(0xff));

    /// @dev Gets the storage slot of the storage struct for the contract calling this library function
    // slither-disable-next-line uninitialized-storage
    function getOperatorStorage() internal pure returns (OperatorStorage storage self) {
        bytes32 position = OPERATOR_STORAGE_POSITION;

        // slither-disable-next-line assembly
        assembly {
            self.slot := position
        }
    }
}

abstract contract OperatorPanel is IOperatorPanel {
    /// @dev sig: 0xb816c81e0d2e75687754a9cb3111541c16ab454792482bf1dd02093f2203f353
    event OperatorApproved(
        uint256 indexed eventNonce, address indexed account, address indexed operator, uint256 newRoles
    );
    /// @dev sig: 0x1145ef8300109b8668d5581d376603c552d28f5aaefa3ca8fb7524286a41a7ae
    event OperatorDisapproved(
        uint256 indexed eventNonce, address indexed account, address indexed operator, uint256 removedRoles
    );

    /// @dev sig: 0x732ea322
    error OperatorDoesNotHaveRole();
    /// @dev sig: 0xe9a05878
    error OperatorChangeUnauthorized();

    address public immutable operatorHub;

    constructor(address operatorHub_) {
        operatorHub = operatorHub_;
    }

    modifier onlySenderOrOperatorHub(address account) {
        if (msg.sender != account && msg.sender != operatorHub) revert OperatorChangeUnauthorized();
        _;
    }

    function _getOperatorStorage() internal pure returns (OperatorStorage storage self) {
        return OperatorStorageLib.getOperatorStorage();
    }

    function getOperatorRoleApprovals(address account, address operator) external view returns (uint256) {
        return _getOperatorStorage().operatorRoleApprovals[account][operator];
    }

    function approveOperator(address account, address operator, uint256 roles)
        external
        onlySenderOrOperatorHub(account)
    {
        OperatorStorage storage self = _getOperatorStorage();

        uint256 approvedRoles = self.operatorRoleApprovals[account][operator];
        self.operatorRoleApprovals[account][operator] = approvedRoles | roles;

        emit OperatorApproved(OperatorEventNonce.inc(), account, operator, roles);
    }

    function disapproveOperator(address account, address operator, uint256 roles)
        external
        onlySenderOrOperatorHub(account)
    {
        OperatorStorage storage self = _getOperatorStorage();

        uint256 approvedRoles = self.operatorRoleApprovals[account][operator];
        self.operatorRoleApprovals[account][operator] = approvedRoles & (~roles);

        emit OperatorDisapproved(OperatorEventNonce.inc(), account, operator, roles);
    }

    function getOperatorEventNonce() external view returns (uint256) {
        return OperatorEventNonce.getCurrentNonce();
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


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

