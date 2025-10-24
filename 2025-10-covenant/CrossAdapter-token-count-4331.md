
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import {BaseAdapter} from "./BaseAdapter.sol";
import {IPriceOracle} from "../../interfaces/IPriceOracle.sol";
import {ScaleUtils} from "@euler-price-oracle/lib/ScaleUtils.sol";
import {Errors} from "../lib/Errors.sol";

/// @title CrossAdapter
/// @author Covenant Labs
/// @notice PriceOracle that chains two PriceOracles.
/// @dev For example, CrossAdapter can price wstETH/USD by querying a wstETH/stETH oracle and a stETH/USD oracle.
/// @notice This is a very close copy to the Errors contract in the euler-price-oracle library, adapted for Covenant under GPL.
contract CrossAdapter is BaseAdapter {
    string public constant name = "CrossAdapter";
    /// @notice The address of the base asset.
    address public immutable base;
    /// @notice The address of the cross/through asset.
    address public immutable cross;
    /// @notice The address of the quote asset.
    address public immutable quote;
    /// @notice The oracle that resolves base/cross and cross/base.
    /// @dev The oracle MUST be bidirectional.
    address public immutable oracleBaseCross;
    /// @notice The oracle that resolves quote/cross and cross/quote.
    /// @dev The oracle MUST be bidirectional.
    address public immutable oracleCrossQuote;

    /// @notice Deploy a CrossAdapter.
    /// @param _base The address of the base asset.
    /// @param _cross The address of the cross/through asset.
    /// @param _quote The address of the quote asset.
    /// @param _oracleBaseCross The oracle that resolves base/cross and cross/base.
    /// @param _oracleCrossQuote The oracle that resolves quote/cross and cross/quote.
    /// @dev Both cross oracles MUST be bidirectional.
    /// @dev Does not support bid/ask pricing.
    constructor(address _base, address _cross, address _quote, address _oracleBaseCross, address _oracleCrossQuote) {
        base = _base;
        cross = _cross;
        quote = _quote;
        oracleBaseCross = _oracleBaseCross;
        oracleCrossQuote = _oracleCrossQuote;
    }

    /// @notice Get a quote by chaining the cross oracles.
    /// @dev For the inverse direction it calculates quote/cross * cross/base.
    /// For the forward direction it calculates base/cross * cross/quote.
    /// @param inAmount The amount of `base` to convert.
    /// @param _base The token that is being priced.
    /// @param _quote The token that is the unit of account.
    /// @return The converted amount by chaining the cross oracles.
    function _getQuote(uint256 inAmount, address _base, address _quote) internal view override returns (uint256) {
        bool inverse = ScaleUtils.getDirectionOrRevert(_base, base, _quote, quote);

        if (inverse) {
            inAmount = IPriceOracle(oracleCrossQuote).getQuote(inAmount, quote, cross);
            return IPriceOracle(oracleBaseCross).getQuote(inAmount, cross, base);
        } else {
            inAmount = IPriceOracle(oracleBaseCross).getQuote(inAmount, base, cross);
            return IPriceOracle(oracleCrossQuote).getQuote(inAmount, cross, quote);
        }
    }

    /// @notice Get a quote preview by chaining the cross oracles.
    /// @dev For the inverse direction it calculates quote/cross * cross/base.
    /// For the forward direction it calculates base/cross * cross/quote.
    /// @param inAmount The amount of `base` to convert.
    /// @param _base The token that is being priced.
    /// @param _quote The token that is the unit of account.
    /// @return The converted amount by chaining the cross oracles.
    function _previewGetQuote(
        uint256 inAmount,
        address _base,
        address _quote
    ) internal view override returns (uint256) {
        bool inverse = ScaleUtils.getDirectionOrRevert(_base, base, _quote, quote);

        if (inverse) {
            inAmount = IPriceOracle(oracleCrossQuote).previewGetQuote(inAmount, quote, cross);
            return IPriceOracle(oracleBaseCross).previewGetQuote(inAmount, cross, base);
        } else {
            inAmount = IPriceOracle(oracleBaseCross).previewGetQuote(inAmount, base, cross);
            return IPriceOracle(oracleCrossQuote).previewGetQuote(inAmount, cross, quote);
        }
    }

    /// @notice Updates price feeds for pull type oracles
    function _updatePriceFeeds(address _base, address _quote, bytes calldata updateData) internal override {
        // decode updateData
        if (updateData.length == 0) {
            // no price feed to update
            // revert if there was a fee payment
            if (msg.value > 0) revert Errors.PriceOracle_IncorrectPayment();
        } else {
            bytes[] memory crossUpdateData = abi.decode(updateData, (bytes[]));
            if (crossUpdateData.length != 2) revert Errors.PriceOracle_InvalidUpdateData();

            // read expected fee payment
            // @dev - note updateData encoding, where base feed data comes first
            uint128 baseFee = IPriceOracle(oracleBaseCross).getUpdateFee(_base, cross, crossUpdateData[0]);
            uint128 quoteFee = IPriceOracle(oracleCrossQuote).getUpdateFee(_quote, cross, crossUpdateData[1]);

            if (msg.value != (baseFee + quoteFee)) revert Errors.PriceOracle_IncorrectPayment();

            // @dev - note updateData encoding, where base feed data comes first
            IPriceOracle(oracleBaseCross).updatePriceFeeds{value: baseFee}(_base, cross, crossUpdateData[0]);
            IPriceOracle(oracleCrossQuote).updatePriceFeeds{value: quoteFee}(_quote, cross, crossUpdateData[1]);
        }
    }

    /// @notice Gets update fee for pull oracles
    function _getUpdateFee(
        address _base,
        address _quote,
        bytes calldata updateData
    ) internal view override returns (uint128 updateFee) {
        // decode updateData
        if (updateData.length == 0) {
            return 0;
        } else {
            bytes[] memory crossUpdateData = abi.decode(updateData, (bytes[]));
            if (crossUpdateData.length != 2) revert Errors.PriceOracle_InvalidUpdateData();

            // read expected fee payment
            // @dev - note updateData encoding, where base feed data comes first
            uint128 baseFee = IPriceOracle(oracleBaseCross).getUpdateFee(_base, cross, crossUpdateData[0]);
            uint128 quoteFee = IPriceOracle(oracleCrossQuote).getUpdateFee(_quote, cross, crossUpdateData[1]);

            return baseFee + quoteFee;
        }
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: AGPL-3.0
pragma solidity >=0.8.0;

/**
 * @title IPriceOracle
 * @author Covenant Labs
 * @notice Defines the the core interface for Covenant oracles.
 * @notice Extends the oracle interface of Euler Labs, https://github.com/euler-xyz/euler-price-oracle/
    to include pricePreviews and priceUpdates/getUpdateFee for pull oracles
 * @notice All functions return a value.  if bid/ask price not implemented, then getQuotes returns bid = ask = getQuote()
 **/

interface IPriceOracle {
    /// @notice Get the name of the oracle.
    /// @return The name of the oracle.
    function name() external view returns (string memory);

    /// @notice One-sided price: How much quote token you would get for inAmount of base token, assuming no price spread.
    /// @param inAmount The amount of `base` to convert.
    /// @param base The token that is being priced.
    /// @param quote The token that is the unit of account.
    /// @return outAmount The amount of `quote` that is equivalent to `inAmount` of `base`.
    function getQuote(uint256 inAmount, address base, address quote) external view returns (uint256 outAmount);

    /// @notice Two-sided price: How much quote token you would get/spend for selling/buying inAmount of base token.
    /// @param inAmount The amount of `base` to convert.
    /// @param base The token that is being priced.
    /// @param quote The token that is the unit of account.
    /// @return bidOutAmount The amount of `quote` you would get for selling `inAmount` of `base`.
    /// @return askOutAmount The amount of `quote` you would spend for buying `inAmount` of `base`.
    function getQuotes(
        uint256 inAmount,
        address base,
        address quote
    ) external view returns (uint256 bidOutAmount, uint256 askOutAmount);

    /// @notice priceUpdate for pulled pricing (e.g., Pyth, Redstone, Chainlink datastreams)
    /// @notice allows pushing pricing to be verified on-chain. Function is payable to receive required payment.
    /// @param base The token that is being priced (use here for routing purposes).
    /// @param quote The token that is the unit of account (use here for routing purposes).
    /// @param updateData Update data package (contains price and other info to be verified onchain)
    function updatePriceFeeds(address base, address quote, bytes calldata updateData) external payable;

    /// @notice Returns the required fee to update an oracle price.
    /// @param base The token that is being priced (use here for routing purposes).
    /// @param quote The token that is the unit of account (use here for routing purposes).
    /// @param updateData Array of price update data.
    /// @return updateFee The required fee in Wei.
    function getUpdateFee(
        address base,
        address quote,
        bytes calldata updateData
    ) external view returns (uint128 updateFee);

    /// @notice Preview of getQuote, with a longer lookback window to avoid quote blocking
    /// @notice One-sided price: How much quote token you would get for inAmount of base token, assuming no price spread.
    /// @param inAmount The amount of `base` to convert.
    /// @param base The token that is being priced.
    /// @param quote The token that is the unit of account.
    /// @return outAmount The amount of `quote` that is equivalent to `inAmount` of `base`.
    function previewGetQuote(uint256 inAmount, address base, address quote) external view returns (uint256 outAmount);

    /// @notice Preview of getQuotes, with a longer lookback window to avoid quote blocking
    /// @notice Two-sided price: How much quote token you would get/spend for selling/buying inAmount of base token.
    /// @param inAmount The amount of `base` to convert.
    /// @param base The token that is being priced.
    /// @param quote The token that is the unit of account.
    /// @return bidOutAmount The amount of `quote` you would get for selling `inAmount` of `base`.
    /// @return askOutAmount The amount of `quote` you would spend for buying `inAmount` of `base`.
    function previewGetQuotes(
        uint256 inAmount,
        address base,
        address quote
    ) external view returns (uint256 bidOutAmount, uint256 askOutAmount);
}

// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.30;

library Errors {
    error E_ZeroAmount(); // 0xaf4935be
    error E_ZeroAddress(); // 0x459f5f6e
    error E_CrossedLimit(); // 0xb8775684
    error E_InsufficientAmount(); // 0x9950c184
    error E_IncorrectMarketAsset(); // 0x76ba676e
    error E_EqualSwapAssets(); // 0x213a0fd0
    error E_MarketLocked(); // 0x8a7ede1f
    error E_MarketPaused(); // 0xed9c479e
    error E_MarketNonExistent(); // 0xe2f65643
    error E_LEXimplementationNotAuthorized(); // 0xb4ee3f59
    error E_CuratorNotAuthorized(); // 0x808c99be
    error E_MarketAlreadyExists(); // 0x601494a7
    error E_IncorrectMarketParams(); // 0x7f6cd0c7
    error E_Unauthorized(); // 0x08e2ce17
    error E_IncorrectPayment(); // 0xa705df45
    error E_ProtocolFeeTooHigh(); // 0xc886fec7
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import {BaseAdapter as EulerBaseAdapter, IERC20} from "@euler-price-oracle/adapter/BaseAdapter.sol";
import {ICovenantPriceOracle} from "../interfaces/ICovenantPriceOracle.sol";
import {Errors} from "../lib/Errors.sol";

/// @title BaseAdapter
/// @author Covenant Labs
/// @notice Abstract adapter with virtual bid/ask pricing.
/// @notice This extends the Euler BaseAdapter and adds the ICovenantPriceOracle interface.
abstract contract BaseAdapter is EulerBaseAdapter, ICovenantPriceOracle {
    /// @inheritdoc ICovenantPriceOracle
    function previewGetQuote(uint256 inAmount, address base, address quote) external view returns (uint256) {
        return _previewGetQuote(inAmount, base, quote);
    }

    /// @inheritdoc ICovenantPriceOracle
    /// @dev Does not support true bid/ask pricing.
    function previewGetQuotes(uint256 inAmount, address base, address quote) external view returns (uint256, uint256) {
        uint256 outAmount = _previewGetQuote(inAmount, base, quote);
        return (outAmount, outAmount);
    }

    /// @inheritdoc ICovenantPriceOracle
    function updatePriceFeeds(address base, address quote, bytes calldata updateData) external payable {
        _updatePriceFeeds(base, quote, updateData);
    }

    /// @inheritdoc ICovenantPriceOracle
    function getUpdateFee(address base, address quote, bytes calldata updateData) external view returns (uint128) {
        return _getUpdateFee(base, quote, updateData);
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Internal functions to be overridden in the inheriting contract

    /// @notice Return the preview quote for the given price query.
    /// @dev Must be overridden in the inheriting contract.
    function _previewGetQuote(uint256 inAmount, address base, address quote) internal view virtual returns (uint256) {
        // Unless overridden, return the live quote.
        return _getQuote(inAmount, base, quote);
    }

    /// @notice Updates price feeds for pull type oracles
    /// @dev Must be overridden in the inheriting contract.
    function _updatePriceFeeds(address base, address quote, bytes calldata updateData) internal virtual {
        // Unless overridden, do not accept any value and return.
        if (msg.value > 0) revert Errors.PriceOracle_IncorrectPayment();
    }

    /// @notice Calculates fee when updating price feed for pull type oracles
    /// @dev Must be overridden in the inheriting contract.
    function _getUpdateFee(
        address base,
        address quote,
        bytes calldata updateData
    ) internal view virtual returns (uint128) {
        // Unless overridden, return 0.
        return 0;
    }
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {Errors} from "./Errors.sol";

type Scale is uint256;

/// @title ScaleUtils
/// @custom:security-contact security@euler.xyz
/// @author Euler Labs (https://www.eulerlabs.com/)
/// @notice Utilities for handling decimal conversion of unit price feeds.
library ScaleUtils {
    uint256 internal constant PRICE_SCALE_MASK = 0x00000000000000000000000000000000ffffffffffffffffffffffffffffffff;
    /// @notice The maximum allowed exponent for Scale components.
    /// @dev 38 is the largest integer exponent of 10 that fits in 128 bits.
    uint256 internal constant MAX_EXPONENT = 38;

    /// @notice Create a `Scale` by packing 2 powers of 10.
    /// @dev Upper 128 bits occupied by 10^feedExponent.
    /// Lower 128 bits occupied by 10^priceExponent.
    /// @param priceExponent The power for `priceScale = 10**priceExponent`.
    /// @param feedExponent The power for `feedScale = 10**feedExponent`.
    /// @return The two scale factors packed in `Scale`.
    function from(uint8 priceExponent, uint8 feedExponent) internal pure returns (Scale) {
        if (priceExponent > MAX_EXPONENT || feedExponent > MAX_EXPONENT) {
            revert Errors.PriceOracle_Overflow();
        }
        return Scale.wrap((10 ** feedExponent << 128) | 10 ** priceExponent);
    }

    /// @notice Calculate the direction of pricing, or revert if no match.
    /// @param givenBase The base asset supplied by the caller.
    /// @param base The base asset in the price oracle adapter.
    /// @param givenQuote The quote asset supplied by the caller.
    /// @param quote The quote asset in the price oracle adapter.
    /// @return False if base/quote, true if quote/base else revert.
    function getDirectionOrRevert(address givenBase, address base, address givenQuote, address quote)
        internal
        pure
        returns (bool)
    {
        if (givenBase == base && givenQuote == quote) return false;
        if (givenBase == quote && givenQuote == base) return true;
        revert Errors.PriceOracle_NotSupported(givenBase, givenQuote);
    }

    /// @notice Calculate the scale factors for converting a unit price.
    /// @param baseDecimals The decimals of the base asset.
    /// @param quoteDecimals The decimals of the quote asset.
    /// @param feedDecimals The decimals of the feed, already incorporated into the price.
    /// @return The scale factors used for price conversions.
    function calcScale(uint8 baseDecimals, uint8 quoteDecimals, uint8 feedDecimals) internal pure returns (Scale) {
        return from(quoteDecimals, feedDecimals + baseDecimals);
    }

    /// @notice Convert the price by applying scale factors.
    /// @param inAmount The amount of `base` to convert.
    /// @param unitPrice The unit price reported by the feed.
    /// @param scale The scale factors returned by `calcScale`.
    /// @param inverse Whether to price base/quote or quote/base.
    /// @return The resulting outAmount.
    function calcOutAmount(uint256 inAmount, uint256 unitPrice, Scale scale, bool inverse)
        internal
        pure
        returns (uint256)
    {
        uint256 priceScale = Scale.unwrap(scale) & PRICE_SCALE_MASK;
        uint256 feedScale = Scale.unwrap(scale) >> 128;
        if (inverse) {
            // (inAmount * feedScale) / (priceScale * unitPrice)
            return FixedPointMathLib.fullMulDiv(inAmount, feedScale, priceScale * unitPrice);
        } else {
            // (inAmount * priceScale * unitPrice) / feedScale
            return FixedPointMathLib.fullMulDiv(inAmount, priceScale * unitPrice, feedScale);
        }
    }
}


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

