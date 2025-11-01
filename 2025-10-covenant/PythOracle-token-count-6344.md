
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import {IPyth} from "@pyth/IPyth.sol";
import {PythStructs} from "@pyth/PythStructs.sol";
import {PythOracle as EulerPythOracle, ScaleUtils, Scale} from "@euler-price-oracle/adapter/pyth/PythOracle.sol";
import {Errors} from "../../lib/Errors.sol";
import {SafeCast} from "@openzeppelin/utils/math/SafeCast.sol";

/// @title PythOracle
/// @author Covenant Labs (Expands Euler Labs interface to include feed updates and getUpdateFee)
contract PythOracle is EulerPythOracle {
    using SafeCast for uint256;

    /// @notice Deploy a PythOracle.
    /// @param _pyth The address of the Pyth oracle proxy.
    /// @param _base The address of the base asset corresponding to the feed.
    /// @param _quote The address of the quote asset corresponding to the feed.
    /// @param _feedId The id of the feed in the Pyth network.
    /// @param _maxStaleness The maximum allowed age of the price.
    /// @param _maxConfWidth The maximum width of the confidence interval in basis points.
    /// @dev Note: A high confidence interval indicates market volatility or Pyth consensus instability.
    /// Consider a lower `_maxConfWidth` for highly-correlated pairs and a higher value for uncorrelated pairs.
    /// Pairs with few data sources and low liquidity are more prone to volatility spikes and consensus instability.
    constructor(
        address _pyth,
        address _base,
        address _quote,
        bytes32 _feedId,
        uint256 _maxStaleness,
        uint256 _maxConfWidth
    ) EulerPythOracle(_pyth, _base, _quote, _feedId, _maxStaleness, _maxConfWidth) {}

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Additional Covenant interface

    /// inheritdoc IPriceOracle
    /// @dev For chainlink push-based price feeds, the preview quote is the same as the live quote.
    function previewGetQuote(uint256 inAmount, address base, address quote) external view returns (uint256) {
        return _previewGetQuote(inAmount, base, quote);
    }

    /// inheritdoc IPriceOracle
    /// @dev Does not support true bid/ask pricing.
    /// @dev For chainlink push-based price feeds, the preview quote is the same as the live quote.
    function previewGetQuotes(uint256 inAmount, address base, address quote) external view returns (uint256, uint256) {
        uint256 outAmount = _previewGetQuote(inAmount, base, quote);
        return (outAmount, outAmount);
    }

    /// inheritdoc IPriceOracle
    function updatePriceFeeds(address, address, bytes calldata updateData) external payable {
        bytes[] memory priceUpdate = abi.decode(updateData, (bytes[]));
        uint fee = IPyth(pyth).getUpdateFee(priceUpdate);
        if (msg.value != fee) revert Errors.PriceOracle_IncorrectPayment();
        IPyth(pyth).updatePriceFeeds{value: fee}(priceUpdate);
    }

    /// inheritdoc IPriceOracle
    function getUpdateFee(address, address, bytes calldata updateData) external view returns (uint128) {
        return IPyth(pyth).getUpdateFee(abi.decode(updateData, (bytes[]))).toUint128();
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Internal functions below are the same as the Euler PythOracle, but with the following changes:
    // - Changed maxStaleness to MAX_STALENESS_UPPER_BOUND
    // - Changed fetchPriceStruct to previewFetchPriceStruct

    /// @notice Same code as _getQuote, but using previewFetchPriceStruct instead of fetchPriceStruct
    function _previewGetQuote(uint256 inAmount, address _base, address _quote) internal view returns (uint256) {
        bool inverse = ScaleUtils.getDirectionOrRevert(_base, base, _quote, quote);

        PythStructs.Price memory priceStruct = _previewFetchPriceStruct();

        uint256 price = uint256(uint64(priceStruct.price));
        int8 feedExponent = int8(baseDecimals) - int8(priceStruct.expo);

        Scale scale;
        if (feedExponent > 0) {
            scale = ScaleUtils.from(quoteDecimals, uint8(feedExponent));
        } else {
            scale = ScaleUtils.from(quoteDecimals + uint8(-feedExponent), 0);
        }
        return ScaleUtils.calcOutAmount(inAmount, price, scale, inverse);
    }

    /// @notice same code as _fetchPriceStruct, but using MAX_STALENESS_UPPER_BOUND instead of maxStaleness
    function _previewFetchPriceStruct() internal view returns (PythStructs.Price memory) {
        PythStructs.Price memory p = IPyth(pyth).getPriceUnsafe(feedId);

        if (p.publishTime < block.timestamp) {
            // Verify that the price is not too stale
            uint256 staleness = block.timestamp - p.publishTime;
            if (staleness > MAX_STALENESS_UPPER_BOUND) revert Errors.PriceOracle_InvalidAnswer(); // @dev Changed from maxStaleness to MAX_STALENESS_UPPER_BOUND
        } else {
            // Verify that the price is not too ahead
            uint256 aheadness = p.publishTime - block.timestamp;
            if (aheadness > MAX_AHEADNESS) revert Errors.PriceOracle_InvalidAnswer();
        }

        // Verify that the price is positive and within the confidence width.
        if (p.price <= 0 || p.conf > (uint64(p.price) * maxConfWidth) / BASIS_POINTS) {
            revert Errors.PriceOracle_InvalidAnswer();
        }

        // Verify that the price exponent is within bounds.
        if (p.expo < MIN_EXPONENT || p.expo > MAX_EXPONENT) {
            revert Errors.PriceOracle_InvalidAnswer();
        }
        return p;
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "./PythStructs.sol";
import "./IPythEvents.sol";

/**
 * @title Consume prices from the Pyth Network (https://pyth.network/).
 * @author Pyth Data Association
 * @notice **Deprecated** – this codebase will be removed on **1 August 2025**.
 *
 * @dev    Switch to the maintained package:
 *         `npm install @pythnetwork/pyth-sdk-solidity`
 *
 *         Migration guide:
 *         https://docs.pyth.network/price-feeds/use-real-time-data/evm
 *
 * @custom:deprecated Repository scheduled for deletion on 1 August 2025.
 *                    Use `@pythnetwork/pyth-sdk-solidity` instead.
 */
interface IPyth is IPythEvents {
    /// @notice Returns the period (in seconds) that a price feed is considered valid since its publish time
    function getValidTimePeriod() external view returns (uint validTimePeriod);

    /// @notice Returns the price and confidence interval.
    /// @dev Reverts if the price has not been updated within the last `getValidTimePeriod()` seconds.
    /// @param id The Pyth Price Feed ID of which to fetch the price and confidence interval.
    /// @return price - please read the documentation of PythStructs.Price to understand how to use this safely.
    function getPrice(
        bytes32 id
    ) external view returns (PythStructs.Price memory price);

    /// @notice Returns the exponentially-weighted moving average price and confidence interval.
    /// @dev Reverts if the EMA price is not available.
    /// @param id The Pyth Price Feed ID of which to fetch the EMA price and confidence interval.
    /// @return price - please read the documentation of PythStructs.Price to understand how to use this safely.
    function getEmaPrice(
        bytes32 id
    ) external view returns (PythStructs.Price memory price);

    /// @notice Returns the price of a price feed without any sanity checks.
    /// @dev This function returns the most recent price update in this contract without any recency checks.
    /// This function is unsafe as the returned price update may be arbitrarily far in the past.
    ///
    /// Users of this function should check the `publishTime` in the price to ensure that the returned price is
    /// sufficiently recent for their application. If you are considering using this function, it may be
    /// safer / easier to use either `getPrice` or `getPriceNoOlderThan`.
    /// @return price - please read the documentation of PythStructs.Price to understand how to use this safely.
    function getPriceUnsafe(
        bytes32 id
    ) external view returns (PythStructs.Price memory price);

    /// @notice Returns the price that is no older than `age` seconds of the current time.
    /// @dev This function is a sanity-checked version of `getPriceUnsafe` which is useful in
    /// applications that require a sufficiently-recent price. Reverts if the price wasn't updated sufficiently
    /// recently.
    /// @return price - please read the documentation of PythStructs.Price to understand how to use this safely.
    function getPriceNoOlderThan(
        bytes32 id,
        uint age
    ) external view returns (PythStructs.Price memory price);

    /// @notice Returns the exponentially-weighted moving average price of a price feed without any sanity checks.
    /// @dev This function returns the same price as `getEmaPrice` in the case where the price is available.
    /// However, if the price is not recent this function returns the latest available price.
    ///
    /// The returned price can be from arbitrarily far in the past; this function makes no guarantees that
    /// the returned price is recent or useful for any particular application.
    ///
    /// Users of this function should check the `publishTime` in the price to ensure that the returned price is
    /// sufficiently recent for their application. If you are considering using this function, it may be
    /// safer / easier to use either `getEmaPrice` or `getEmaPriceNoOlderThan`.
    /// @return price - please read the documentation of PythStructs.Price to understand how to use this safely.
    function getEmaPriceUnsafe(
        bytes32 id
    ) external view returns (PythStructs.Price memory price);

    /// @notice Returns the exponentially-weighted moving average price that is no older than `age` seconds
    /// of the current time.
    /// @dev This function is a sanity-checked version of `getEmaPriceUnsafe` which is useful in
    /// applications that require a sufficiently-recent price. Reverts if the price wasn't updated sufficiently
    /// recently.
    /// @return price - please read the documentation of PythStructs.Price to understand how to use this safely.
    function getEmaPriceNoOlderThan(
        bytes32 id,
        uint age
    ) external view returns (PythStructs.Price memory price);

    /// @notice Update price feeds with given update messages.
    /// This method requires the caller to pay a fee in wei; the required fee can be computed by calling
    /// `getUpdateFee` with the length of the `updateData` array.
    /// Prices will be updated if they are more recent than the current stored prices.
    /// The call will succeed even if the update is not the most recent.
    /// @dev Reverts if the transferred fee is not sufficient or the updateData is invalid.
    /// @param updateData Array of price update data.
    function updatePriceFeeds(bytes[] calldata updateData) external payable;

    /// @notice Wrapper around updatePriceFeeds that rejects fast if a price update is not necessary. A price update is
    /// necessary if the current on-chain publishTime is older than the given publishTime. It relies solely on the
    /// given `publishTimes` for the price feeds and does not read the actual price update publish time within `updateData`.
    ///
    /// This method requires the caller to pay a fee in wei; the required fee can be computed by calling
    /// `getUpdateFee` with the length of the `updateData` array.
    ///
    /// `priceIds` and `publishTimes` are two arrays with the same size that correspond to senders known publishTime
    /// of each priceId when calling this method. If all of price feeds within `priceIds` have updated and have
    /// a newer or equal publish time than the given publish time, it will reject the transaction to save gas.
    /// Otherwise, it calls updatePriceFeeds method to update the prices.
    ///
    /// @dev Reverts if update is not needed or the transferred fee is not sufficient or the updateData is invalid.
    /// @param updateData Array of price update data.
    /// @param priceIds Array of price ids.
    /// @param publishTimes Array of publishTimes. `publishTimes[i]` corresponds to known `publishTime` of `priceIds[i]`
    function updatePriceFeedsIfNecessary(
        bytes[] calldata updateData,
        bytes32[] calldata priceIds,
        uint64[] calldata publishTimes
    ) external payable;

    /// @notice Returns the required fee to update an array of price updates.
    /// @param updateData Array of price update data.
    /// @return feeAmount The required fee in Wei.
    function getUpdateFee(
        bytes[] calldata updateData
    ) external view returns (uint feeAmount);

    /// @notice Parse `updateData` and return price feeds of the given `priceIds` if they are all published
    /// within `minPublishTime` and `maxPublishTime`.
    ///
    /// You can use this method if you want to use a Pyth price at a fixed time and not the most recent price;
    /// otherwise, please consider using `updatePriceFeeds`. This method does not store the price updates on-chain.
    ///
    /// This method requires the caller to pay a fee in wei; the required fee can be computed by calling
    /// `getUpdateFee` with the length of the `updateData` array.
    ///
    ///
    /// @dev Reverts if the transferred fee is not sufficient or the updateData is invalid or there is
    /// no update for any of the given `priceIds` within the given time range.
    /// @param updateData Array of price update data.
    /// @param priceIds Array of price ids.
    /// @param minPublishTime minimum acceptable publishTime for the given `priceIds`.
    /// @param maxPublishTime maximum acceptable publishTime for the given `priceIds`.
    /// @return priceFeeds Array of the price feeds corresponding to the given `priceIds` (with the same order).
    function parsePriceFeedUpdates(
        bytes[] calldata updateData,
        bytes32[] calldata priceIds,
        uint64 minPublishTime,
        uint64 maxPublishTime
    ) external payable returns (PythStructs.PriceFeed[] memory priceFeeds);
}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

/**
 * @title IPythEvents contains the events that Pyth contract emits.
 * @notice **Deprecated** – this codebase will be removed on **1 August 2025**.
 *
 * @dev    Switch to the maintained package:
 *         `npm install @pythnetwork/pyth-sdk-solidity`
 *
 *         Migration guide:
 *         https://docs.pyth.network/price-feeds/use-real-time-data/evm
 *
 * @custom:deprecated Repository scheduled for deletion on 1 August 2025.
 *                    Use `@pythnetwork/pyth-sdk-solidity` instead.
 */
interface IPythEvents {
    /// @dev Emitted when the price feed with `id` has received a fresh update.
    /// @param id The Pyth Price Feed ID.
    /// @param publishTime Publish time of the given price update.
    /// @param price Price of the given price update.
    /// @param conf Confidence interval of the given price update.
    event PriceFeedUpdate(
        bytes32 indexed id,
        uint64 publishTime,
        int64 price,
        uint64 conf
    );

    /// @dev Emitted when a batch price update is processed successfully.
    /// @param chainId ID of the source chain that the batch price update comes from.
    /// @param sequenceNumber Sequence number of the batch price update.
    event BatchPriceFeedUpdate(uint16 chainId, uint64 sequenceNumber);
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

/// @title Errors
/// @author Covenant Labs
/// @notice Collects common errors in PriceOracles.
/// @notice This is a very close copy to the Errors contract in the euler-price-oracle library, adapted for Covenant under GPL.
library Errors {
    /// @notice The external feed returned an invalid answer.
    error PriceOracle_InvalidAnswer();
    /// @notice The configuration parameters for the PriceOracle are invalid.
    error PriceOracle_InvalidConfiguration();
    /// @notice The base/quote path is not supported.
    /// @param base The address of the base asset.
    /// @param quote The address of the quote asset.
    error PriceOracle_NotSupported(address base, address quote);
    /// @notice The quote cannot be completed due to overflow.
    error PriceOracle_Overflow();
    /// @notice The price is too stale.
    /// @param staleness The time elapsed since the price was updated.
    /// @param maxStaleness The maximum time elapsed since the last price update.
    error PriceOracle_TooStale(uint256 staleness, uint256 maxStaleness);
    /// @notice The method can only be called by the governor.
    error Governance_CallerNotGovernor();
    /// @notice There is an incorrect payment in the call.
    error PriceOracle_IncorrectPayment();
    /// @notice The update data is invalid.
    error PriceOracle_InvalidUpdateData();
    /// @notice The method is not implemented.
    error PriceOracle_NotImplemented();
}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

/**
 * @notice **Deprecated** – this codebase will be removed on **1 August 2025**.
 *
 * @dev    Switch to the maintained package:
 *         `npm install @pythnetwork/pyth-sdk-solidity`
 *
 *         Migration guide:
 *         https://docs.pyth.network/price-feeds/use-real-time-data/evm
 *
 * @custom:deprecated Repository scheduled for deletion on 1 August 2025.
 *                    Use `@pythnetwork/pyth-sdk-solidity` instead.
 */
contract PythStructs {
    // A price with a degree of uncertainty, represented as a price +- a confidence interval.
    //
    // The confidence interval roughly corresponds to the standard error of a normal distribution.
    // Both the price and confidence are stored in a fixed-point numeric representation,
    // `x * (10^expo)`, where `expo` is the exponent.
    //
    // Please refer to the documentation at https://docs.pyth.network/consumers/best-practices for how
    // to how this price safely.
    struct Price {
        // Price
        int64 price;
        // Confidence interval around the price
        uint64 conf;
        // Price exponent
        int32 expo;
        // Unix timestamp describing when the price was published
        uint publishTime;
    }

    // PriceFeed represents a current aggregate price from pyth publisher feeds.
    struct PriceFeed {
        // The price ID.
        bytes32 id;
        // Latest available price
        Price price;
        // Latest available exponentially-weighted moving average price
        Price emaPrice;
    }
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import {IPyth} from "@pyth/IPyth.sol";
import {PythStructs} from "@pyth/PythStructs.sol";
import {BaseAdapter, Errors, IPriceOracle} from "../BaseAdapter.sol";
import {ScaleUtils, Scale} from "../../lib/ScaleUtils.sol";

/// @title PythOracle
/// @custom:security-contact security@euler.xyz
/// @author Euler Labs (https://www.eulerlabs.com/)
/// @notice PriceOracle adapter for Pyth pull-based price feeds.
/// @dev Integration Note: Pyth is a pull-based oracle which requires price updates to be pushed by the user.
/// Before calling `getQuote*` dispatch a call `updatePriceFeeds` on the Pyth oracle proxy to refresh the price.
/// This is best done atomically via a multicall contract such as the Ethereum Vault Connector (EVC).
contract PythOracle is BaseAdapter {
    /// @notice The maximum length of time that a price can be in the future.
    uint256 internal constant MAX_AHEADNESS = 1 minutes;
    /// @notice The maximum permitted value for `maxStaleness`.
    uint256 internal constant MAX_STALENESS_UPPER_BOUND = 15 minutes;
    /// @notice The minimum permitted value for `maxConfWidth`.
    /// @dev Equal to 0.1%.
    uint256 internal constant MAX_CONF_WIDTH_LOWER_BOUND = 10;
    /// @notice The maximum permitted value for `maxConfWidth`.
    /// @dev Equal to 5%.
    uint256 internal constant MAX_CONF_WIDTH_UPPER_BOUND = 500;
    /// @dev The smallest PythStruct exponent that the oracle can handle.
    int256 internal constant MIN_EXPONENT = -20;
    /// @dev The largest PythStruct exponent that the oracle can handle.
    int256 internal constant MAX_EXPONENT = 12;
    /// @dev The denominator for basis points values (maxConfWidth).
    uint256 internal constant BASIS_POINTS = 10_000;
    /// @inheritdoc IPriceOracle
    string public constant name = "PythOracle";
    /// @notice The address of the Pyth oracle proxy.
    address public immutable pyth;
    /// @notice The address of the base asset corresponding to the feed.
    address public immutable base;
    /// @notice The address of the quote asset corresponding to the feed.
    address public immutable quote;
    /// @notice The id of the feed in the Pyth network.
    /// @dev See https://pyth.network/developers/price-feed-ids.
    bytes32 public immutable feedId;
    /// @notice The maximum allowed age of the price.
    uint256 public immutable maxStaleness;
    /// @notice The maximum allowed width of the confidence interval.
    /// @dev Note: this value is in basis points i.e. 500 = 5%.
    uint256 public immutable maxConfWidth;
    /// @dev Used for correcting for the decimals of base and quote.
    uint8 internal immutable baseDecimals;
    /// @dev Used for correcting for the decimals of base and quote.
    uint8 internal immutable quoteDecimals;

    /// @notice Deploy a PythOracle.
    /// @param _pyth The address of the Pyth oracle proxy.
    /// @param _base The address of the base asset corresponding to the feed.
    /// @param _quote The address of the quote asset corresponding to the feed.
    /// @param _feedId The id of the feed in the Pyth network.
    /// @param _maxStaleness The maximum allowed age of the price.
    /// @param _maxConfWidth The maximum width of the confidence interval in basis points.
    /// @dev Note: A high confidence interval indicates market volatility or Pyth consensus instability.
    /// Consider a lower `_maxConfWidth` for highly-correlated pairs and a higher value for uncorrelated pairs.
    /// Pairs with few data sources and low liquidity are more prone to volatility spikes and consensus instability.
    constructor(
        address _pyth,
        address _base,
        address _quote,
        bytes32 _feedId,
        uint256 _maxStaleness,
        uint256 _maxConfWidth
    ) {
        if (_maxStaleness > MAX_STALENESS_UPPER_BOUND) {
            revert Errors.PriceOracle_InvalidConfiguration();
        }
        if (_maxConfWidth < MAX_CONF_WIDTH_LOWER_BOUND || _maxConfWidth > MAX_CONF_WIDTH_UPPER_BOUND) {
            revert Errors.PriceOracle_InvalidConfiguration();
        }

        pyth = _pyth;
        base = _base;
        quote = _quote;
        feedId = _feedId;
        maxStaleness = _maxStaleness;
        maxConfWidth = _maxConfWidth;
        baseDecimals = _getDecimals(base);
        quoteDecimals = _getDecimals(quote);
    }

    /// @notice Fetch the latest Pyth price and transform it to a quote.
    /// @param inAmount The amount of `base` to convert.
    /// @param _base The token that is being priced.
    /// @param _quote The token that is the unit of account.
    /// @return The converted amount.
    function _getQuote(uint256 inAmount, address _base, address _quote) internal view override returns (uint256) {
        bool inverse = ScaleUtils.getDirectionOrRevert(_base, base, _quote, quote);

        PythStructs.Price memory priceStruct = _fetchPriceStruct();

        uint256 price = uint256(uint64(priceStruct.price));
        int8 feedExponent = int8(baseDecimals) - int8(priceStruct.expo);

        Scale scale;
        if (feedExponent > 0) {
            scale = ScaleUtils.from(quoteDecimals, uint8(feedExponent));
        } else {
            scale = ScaleUtils.from(quoteDecimals + uint8(-feedExponent), 0);
        }
        return ScaleUtils.calcOutAmount(inAmount, price, scale, inverse);
    }

    /// @notice Get the latest Pyth price and perform sanity checks.
    /// @dev Revert conditions: update timestamp is too stale or too ahead, price is negative or zero,
    /// confidence interval is too wide, exponent is too large or too small.
    /// @return The Pyth price struct without modification.
    function _fetchPriceStruct() internal view returns (PythStructs.Price memory) {
        PythStructs.Price memory p = IPyth(pyth).getPriceUnsafe(feedId);

        if (p.publishTime < block.timestamp) {
            // Verify that the price is not too stale
            uint256 staleness = block.timestamp - p.publishTime;
            if (staleness > maxStaleness) revert Errors.PriceOracle_InvalidAnswer();
        } else {
            // Verify that the price is not too ahead
            uint256 aheadness = p.publishTime - block.timestamp;
            if (aheadness > MAX_AHEADNESS) revert Errors.PriceOracle_InvalidAnswer();
        }

        // Verify that the price is positive and within the confidence width.
        if (p.price <= 0 || p.conf > uint64(p.price) * maxConfWidth / BASIS_POINTS) {
            revert Errors.PriceOracle_InvalidAnswer();
        }

        // Verify that the price exponent is within bounds.
        if (p.expo < MIN_EXPONENT || p.expo > MAX_EXPONENT) {
            revert Errors.PriceOracle_InvalidAnswer();
        }
        return p;
    }
}


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {PythOracle} from "../src/curators/oracles/pyth/PythOracle.sol";

// @dev - order needs to be alphabetical given forge-std constraints
struct PythConfig {
    address baseToken;
    bytes32 feedId;
    uint256 maxConfWidth;
    uint256 maxStaleness;
    string name;
    string oracleType;
    address pythAddress;
    address quoteToken;
}

contract DeployPythOracle is Script {
    function run() external {
        // Read pyth config JSON
        string memory jsonString = vm.readFile("./script/OracleConfig-Pyth.json"); //prettier-ignore
        bytes memory jsonData = vm.parseJson(jsonString);
        PythConfig memory config = abi.decode(jsonData, (PythConfig)); //prettier-ignore

        // Print parsing
        console.log("=== Pyth Oracle Configuration ===");
        console.log("Oracle Type:", config.oracleType);
        console.log("Name:", config.name);
        console.log("Base Token:", config.baseToken);
        console.log("Quote Token:", config.quoteToken);
        console.log("Pyth Address:", config.pythAddress);
        console.log("Feed ID:");
        console.logBytes32(config.feedId);
        console.log("Max Staleness:", config.maxStaleness);
        console.log("Max Conf Width:", config.maxConfWidth);

        vm.startBroadcast();

        // Deploy Pyth Oracle
        PythOracle pythOracle = new PythOracle(
            config.pythAddress,
            config.baseToken,
            config.quoteToken,
            config.feedId,
            config.maxStaleness,
            config.maxConfWidth
        );

        console.log("PythOracle deployed at:", address(pythOracle));

        vm.stopBroadcast();

        console.log("\n=== Deployment Summary ===");
        console.log("PythOracle:", address(pythOracle));
    }
}

