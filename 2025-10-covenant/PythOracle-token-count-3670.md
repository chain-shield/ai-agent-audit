
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

