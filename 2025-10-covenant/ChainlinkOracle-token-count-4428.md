
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

//import {IPriceOracle} from "../../../interfaces/IPriceOracle.sol";
import {ChainlinkOracle as EulerChainlinkOracle} from "@euler-price-oracle/adapter/chainlink/ChainlinkOracle.sol";
import {AggregatorV3Interface} from "@euler-price-oracle/adapter/chainlink/AggregatorV3Interface.sol";
import {ScaleUtils, Scale} from "@euler-price-oracle/lib/ScaleUtils.sol";
import {Errors} from "../../lib/Errors.sol";

/// @title ChainlinkOracle
/// @author Covenant Labs (expands Euler Labs interface, but does not change functionality))
/// @notice PriceOracle adapter for Chainlink push-based price feeds.
/// @dev Integration Note: `maxStaleness` is an immutable parameter set in the constructor.
/// If the aggregator's heartbeat changes, this adapter may exhibit unintended behavior.
contract ChainlinkOracle is EulerChainlinkOracle {
    /// @notice Deploy a ChainlinkOracle.
    /// @param _base The address of the base asset corresponding to the feed.
    /// @param _quote The address of the quote asset corresponding to the feed.
    /// @param _feed The address of the Chainlink price feed.
    /// @param _maxStaleness The maximum allowed age of the price.
    /// @dev Consider setting `_maxStaleness` to slightly more than the feed's heartbeat
    /// to account for possible network delays when the heartbeat is triggered.
    constructor(
        address _base,
        address _quote,
        address _feed,
        uint256 _maxStaleness
    ) EulerChainlinkOracle(_base, _quote, _feed, _maxStaleness) {}

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Additional Covenant interface

    /// inheritdoc IPriceOracle
    /// @dev For chainlink push-based price feeds, the preview quote is the same as the live quote.
    function previewGetQuote(uint256 inAmount, address base, address quote) external view returns (uint256) {
        return _getQuote(inAmount, base, quote);
    }

    /// inheritdoc IPriceOracle
    /// @dev Does not support true bid/ask pricing.
    /// @dev For chainlink push-based price feeds, the preview quote is the same as the live quote.
    function previewGetQuotes(uint256 inAmount, address base, address quote) external view returns (uint256, uint256) {
        uint256 outAmount = _getQuote(inAmount, base, quote);
        return (outAmount, outAmount);
    }

    /// inheritdoc IPriceOracle
    function updatePriceFeeds(address base, address quote, bytes calldata updateData) external payable {
        // Do not accept any value and return.
        if (msg.value > 0) revert Errors.PriceOracle_IncorrectPayment();
    }

    /// inheritdoc IPriceOracle
    function getUpdateFee(address base, address quote, bytes calldata updateData) external view returns (uint128) {
        // Return 0.
        return 0;
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES
// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

/// @title AggregatorV3Interface
/// @author smartcontractkit (https://github.com/smartcontractkit/chainlink/blob/e87b83cd78595c09061c199916c4bb9145e719b7/contracts/src/v0.8/shared/interfaces/AggregatorV3Interface.sol)
/// @notice Partial interface for Chainlink Data Feeds.
interface AggregatorV3Interface {
    /// @notice Returns the feed's decimals.
    /// @return The decimals of the feed.
    function decimals() external view returns (uint8);

    /// @notice Get data about the latest round.
    /// @return roundId The round ID from the aggregator for which the data was retrieved.
    /// @return answer The answer for the given round.
    /// @return startedAt The timestamp when the round was started.
    /// (Only some AggregatorV3Interface implementations return meaningful values)
    /// @return updatedAt The timestamp when the round last was updated (i.e. answer was last computed).
    /// @return answeredInRound is the round ID of the round in which the answer was computed.
    function latestRoundData()
        external
        view
        returns (uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound);
}

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

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import {BaseAdapter, Errors, IPriceOracle} from "../BaseAdapter.sol";
import {AggregatorV3Interface} from "./AggregatorV3Interface.sol";
import {ScaleUtils, Scale} from "../../lib/ScaleUtils.sol";

/// @title ChainlinkOracle
/// @custom:security-contact security@euler.xyz
/// @author Euler Labs (https://www.eulerlabs.com/)
/// @notice PriceOracle adapter for Chainlink push-based price feeds.
/// @dev Integration Note: `maxStaleness` is an immutable parameter set in the constructor.
/// If the aggregator's heartbeat changes, this adapter may exhibit unintended behavior.
contract ChainlinkOracle is BaseAdapter {
    /// @inheritdoc IPriceOracle
    string public constant name = "ChainlinkOracle";
    /// @notice The minimum permitted value for `maxStaleness`.
    uint256 internal constant MAX_STALENESS_LOWER_BOUND = 1 minutes;
    /// @notice The maximum permitted value for `maxStaleness`.
    uint256 internal constant MAX_STALENESS_UPPER_BOUND = 72 hours;
    /// @notice The address of the base asset corresponding to the feed.
    address public immutable base;
    /// @notice The address of the quote asset corresponding to the feed.
    address public immutable quote;
    /// @notice The address of the Chainlink price feed.
    /// @dev https://docs.chain.link/data-feeds/price-feeds/addresses
    address public immutable feed;
    /// @notice The maximum allowed age of the price.
    /// @dev Reverts if block.timestamp - updatedAt > maxStaleness.
    uint256 public immutable maxStaleness;
    /// @notice The scale factors used for decimal conversions.
    Scale internal immutable scale;

    /// @notice Deploy a ChainlinkOracle.
    /// @param _base The address of the base asset corresponding to the feed.
    /// @param _quote The address of the quote asset corresponding to the feed.
    /// @param _feed The address of the Chainlink price feed.
    /// @param _maxStaleness The maximum allowed age of the price.
    /// @dev Consider setting `_maxStaleness` to slightly more than the feed's heartbeat
    /// to account for possible network delays when the heartbeat is triggered.
    constructor(address _base, address _quote, address _feed, uint256 _maxStaleness) {
        if (_maxStaleness < MAX_STALENESS_LOWER_BOUND || _maxStaleness > MAX_STALENESS_UPPER_BOUND) {
            revert Errors.PriceOracle_InvalidConfiguration();
        }

        base = _base;
        quote = _quote;
        feed = _feed;
        maxStaleness = _maxStaleness;

        // The scale factor is used to correctly convert decimals.
        uint8 baseDecimals = _getDecimals(base);
        uint8 quoteDecimals = _getDecimals(quote);
        uint8 feedDecimals = AggregatorV3Interface(feed).decimals();
        scale = ScaleUtils.calcScale(baseDecimals, quoteDecimals, feedDecimals);
    }

    /// @notice Get the quote from the Chainlink feed.
    /// @param inAmount The amount of `base` to convert.
    /// @param _base The token that is being priced.
    /// @param _quote The token that is the unit of account.
    /// @return The converted amount using the Chainlink feed.
    function _getQuote(uint256 inAmount, address _base, address _quote) internal view override returns (uint256) {
        bool inverse = ScaleUtils.getDirectionOrRevert(_base, base, _quote, quote);

        (, int256 answer,, uint256 updatedAt,) = AggregatorV3Interface(feed).latestRoundData();
        if (answer <= 0) revert Errors.PriceOracle_InvalidAnswer();
        uint256 staleness = block.timestamp - updatedAt;
        if (staleness > maxStaleness) revert Errors.PriceOracle_TooStale(staleness, maxStaleness);

        uint256 price = uint256(answer);
        return ScaleUtils.calcOutAmount(inAmount, price, scale, inverse);
    }
}


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {ChainlinkOracle} from "../src/curators/oracles/chainlink/ChainlinkOracle.sol";

// @dev - order needs to be alphabetical given forge-std constraints
struct ChainlinkConfig {
    address baseToken;
    address feedAddress;
    uint256 maxStaleness;
    string name;
    string oracleType;
    address quoteToken;
}

contract DeployChainlinkOracle is Script {
    function run() external {
        // Read chainlink config JSON
        string memory jsonString = vm.readFile("./script/OracleConfig-Chainlink.json"); //prettier-ignore
        bytes memory jsonData = vm.parseJson(jsonString);
        ChainlinkConfig memory config = abi.decode(jsonData, (ChainlinkConfig)); //prettier-ignore

        // Print parsing
        console.log("=== Chainlink Oracle Configuration ===");
        console.log("Oracle Type:", config.oracleType);
        console.log("Name:", config.name);
        console.log("Base Token:", config.baseToken);
        console.log("Quote Token:", config.quoteToken);
        console.log("Feed Address:", config.feedAddress);
        console.log("Max Staleness:", config.maxStaleness);

        vm.startBroadcast();

        // Deploy Chainlink Oracle
        ChainlinkOracle chainlinkOracle = new ChainlinkOracle(
            config.baseToken,
            config.quoteToken,
            config.feedAddress,
            config.maxStaleness
        );

        console.log("ChainlinkOracle deployed at:", address(chainlinkOracle));

        vm.stopBroadcast();

        console.log("\n=== Deployment Summary ===");
        console.log("ChainlinkOracle:", address(chainlinkOracle));
    }
}

