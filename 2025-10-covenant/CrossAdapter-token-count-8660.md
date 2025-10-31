
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

// SPDX-License-Identifier: AGPL-3.0
pragma solidity >=0.8.0;

/**
 * @title ICovenantPriceOracle
 * @author Covenant Labs
 * @notice Defines the expanded interface for Covenant oracles (vs Euler Labs interface as defined in Eulers IPriceOracle.sol)
 * @notice Follows closely oracle interface of Euler Labs, https://github.com/euler-xyz/euler-price-oracle/
 * @notice All functions return a value.  if bid/ask price not implemented, then getQuotes returns bid = ask = getQuote()
 **/

interface ICovenantPriceOracle {
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


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import {IERC4626} from "forge-std/interfaces/IERC4626.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/access/Ownable2Step.sol";
import {IPriceOracle} from "../interfaces/IPriceOracle.sol";
import {Errors} from "./lib/Errors.sol";

/// @title CovenantCurator
/// @author Covenant Labs
/// @notice Covenant Curator V1.0 is, among other things, an oracle router
/// @notice The contract enables the curator to decide on oracle and ERC4626 to authorize
/// @notice This is a very close copy to the oracle router contract in the euler-price-oracle library, adapted for Covenant under GPL.
/// but extends logic to include pricePreviews and priceUpdates/getUpdateFee for pull oracles
/// @notice All functions return a value.  if bid/ask price not implemented, then getQuotes returns bid = ask = getQuote()
/// @dev Integration Note: The router supports pricing via `convertToAssets` for trusted `resolvedVaults`.
/// By ERC4626 spec `convert*` ignores liquidity restrictions, fees, slippage and per-user restrictions.
/// Therefore the reported price may not be realizable through `redeem` or `withdraw`.
contract CovenantCurator is Ownable2Step, IPriceOracle {
    /// @inheritdoc IPriceOracle
    string public constant name = "CovenantCurator V1.0";
    /// @notice The PriceOracle to call if this router is not configured for base/quote.
    /// @dev If `address(0)` then there is no fallback.
    address public fallbackOracle;
    /// @notice ERC4626 vaults resolved using internal pricing (`convertToAssets`).
    mapping(address vault => address asset) public resolvedVaults;
    /// @notice PriceOracle configured per asset pair.
    /// @dev The keys are lexicographically sorted (asset0 < asset1).
    mapping(address asset0 => mapping(address asset1 => address oracle)) internal oracles;

    /// @notice Configure a PriceOracle to resolve an asset pair.
    /// @param asset0 The address first in lexicographic order.
    /// @param asset1 The address second in lexicographic order.
    /// @param oracle The address of the PriceOracle that resolves the pair.
    /// @dev If `oracle` is `address(0)` then the configuration was removed.
    /// The keys are lexicographically sorted (asset0 < asset1).
    event ConfigSet(address indexed asset0, address indexed asset1, address indexed oracle);
    /// @notice Set a PriceOracle as a fallback resolver.
    /// @param fallbackOracle The address of the PriceOracle that is called when base/quote is not configured.
    /// @dev If `fallbackOracle` is `address(0)` then there is no fallback resolver.
    event FallbackOracleSet(address indexed fallbackOracle);
    /// @notice Mark an ERC4626 vault to be resolved to its `asset` via its `convert*` methods.
    /// @param vault The address of the ERC4626 vault.
    /// @param asset The address of the vault's asset.
    /// @dev If `asset` is `address(0)` then the configuration was removed.
    event ResolvedVaultSet(address indexed vault, address indexed asset);

    /// @notice Deploy CovenantRouter.
    /// @param _governor The address of the governor.
    constructor(address _governor) Ownable(_governor) {
        if (_governor == address(0)) revert Errors.PriceOracle_InvalidConfiguration();
    }

    /// @notice Configure a PriceOracle to resolve base/quote and quote/base.
    /// @param base The address of the base token.
    /// @param quote The address of the quote token.
    /// @param oracle The address of the PriceOracle to resolve the pair.
    /// @dev Callable only by the governor.
    function govSetConfig(address base, address quote, address oracle) external onlyOwner {
        // This case is handled by `resolveOracle`.
        if (base == quote) revert Errors.PriceOracle_InvalidConfiguration();
        (address asset0, address asset1) = _sort(base, quote);
        oracles[asset0][asset1] = oracle;
        emit ConfigSet(asset0, asset1, oracle);
    }

    /// @notice Configure an ERC4626 vault to use internal pricing via `convert*` methods.
    /// @param vault The address of the ERC4626 vault.
    /// @param set True to configure the vault, false to clear the record.
    /// @dev Callable only by the governor. Vault must implement ERC4626.
    /// Note: Before configuring a vault verify that its `convertToAssets` is secure.
    function govSetResolvedVault(address vault, bool set) external onlyOwner {
        address asset = set ? IERC4626(vault).asset() : address(0);
        resolvedVaults[vault] = asset;
        emit ResolvedVaultSet(vault, asset);
    }

    /// @notice Set a PriceOracle as a fallback resolver.
    /// @param _fallbackOracle The address of the PriceOracle that is called when base/quote is not configured.
    /// @dev Callable only by the governor. `address(0)` removes the fallback.
    function govSetFallbackOracle(address _fallbackOracle) external onlyOwner {
        fallbackOracle = _fallbackOracle;
        emit FallbackOracleSet(_fallbackOracle);
    }

    /// @inheritdoc IPriceOracle
    function getQuote(uint256 inAmount, address base, address quote) external view returns (uint256) {
        address oracle;
        (inAmount, base, quote, oracle) = resolveOracle(inAmount, base, quote);
        if (base == quote) return inAmount;
        return IPriceOracle(oracle).getQuote(inAmount, base, quote);
    }

    /// @inheritdoc IPriceOracle
    function getQuotes(uint256 inAmount, address base, address quote) external view returns (uint256, uint256) {
        address oracle;
        (inAmount, base, quote, oracle) = resolveOracle(inAmount, base, quote);
        if (base == quote) return (inAmount, inAmount);
        return IPriceOracle(oracle).getQuotes(inAmount, base, quote);
    }

    /// @notice Get the PriceOracle configured for base/quote.
    /// @param base The address of the base token.
    /// @param quote The address of the quote token.
    /// @return The configured `PriceOracle` for the pair or `address(0)` if no oracle is configured.
    function getConfiguredOracle(address base, address quote) public view returns (address) {
        (address asset0, address asset1) = _sort(base, quote);
        return oracles[asset0][asset1];
    }

    /// @notice Resolve the PriceOracle to call for a given base/quote pair.
    /// @param inAmount The amount of `base` to convert.
    /// @param base The token that is being priced.
    /// @param quote The token that is the unit of account.
    /// @dev Implements the following resolution logic:
    /// 1. Check the base case: `base == quote` and terminate if true.
    /// 2. If a PriceOracle is configured for base/quote in the `oracles` mapping, return it.
    /// 3. If `base` is configured as a resolved ERC4626 vault, call `convertToAssets(inAmount)`
    /// and continue the recursion, substituting the ERC4626 `asset` for `base`.
    /// 4. As a last resort, return the fallback oracle or revert if it is not set.
    /// @return The resolved amount. This value may be different from the original `inAmount`
    /// if the resolution path included an ERC4626 vault present in `resolvedVaults`.
    /// @return The resolved base.
    /// @return The resolved quote.
    /// @return The resolved PriceOracle to call.
    function resolveOracle(
        uint256 inAmount,
        address base,
        address quote
    )
        public
        view
        returns (uint256, /* resolvedAmount */ address, /* base */ address, /* quote */ address /* oracle */)
    {
        // 1. Check the base case.
        if (base == quote) return (inAmount, base, quote, address(0));
        // 2. Check if there is a PriceOracle configured for base/quote.
        address oracle = getConfiguredOracle(base, quote);
        if (oracle != address(0)) return (inAmount, base, quote, oracle);
        // 3. Recursively resolve `base`.
        address baseAsset = resolvedVaults[base];
        if (baseAsset != address(0)) {
            inAmount = IERC4626(base).convertToAssets(inAmount);
            return resolveOracle(inAmount, baseAsset, quote);
        }
        // 4. Return the fallback or revert if not configured.
        oracle = fallbackOracle;
        if (oracle == address(0)) revert Errors.PriceOracle_NotSupported(base, quote);
        return (inAmount, base, quote, oracle);
    }

    /// @notice Lexicographically sort two addresses.
    /// @param assetA One of the assets in the pair.
    /// @param assetB The other asset in the pair.
    /// @return The address first in lexicographic order.
    /// @return The address second in lexicographic order.
    function _sort(address assetA, address assetB) internal pure returns (address, address) {
        return assetA < assetB ? (assetA, assetB) : (assetB, assetA);
    }

    /////////////////////////////////////////////////////////////////////////////////
    // Additional functions for Covenant
    /////////////////////////////////////////////////////////////////////////////////

    /// @inheritdoc IPriceOracle
    function previewGetQuote(uint256 inAmount, address base, address quote) external view returns (uint256) {
        address oracle;
        (inAmount, base, quote, oracle) = resolveOracle(inAmount, base, quote);
        if (base == quote) return inAmount;
        return IPriceOracle(oracle).previewGetQuote(inAmount, base, quote);
    }

    /// @inheritdoc IPriceOracle
    function previewGetQuotes(uint256 inAmount, address base, address quote) external view returns (uint256, uint256) {
        address oracle;
        (inAmount, base, quote, oracle) = resolveOracle(inAmount, base, quote);
        if (base == quote) return (inAmount, inAmount);
        return IPriceOracle(oracle).previewGetQuotes(inAmount, base, quote);
    }

    /// @inheritdoc IPriceOracle
    function updatePriceFeeds(address base, address quote, bytes calldata updateData) external payable {
        address oracle;
        (, base, quote, oracle) = resolveOracle(0, base, quote);
        if (base == quote) {
            if (msg.value > 0) revert Errors.PriceOracle_IncorrectPayment();
            return;
        }
        return IPriceOracle(oracle).updatePriceFeeds{value: msg.value}(base, quote, updateData);
    }

    /// @inheritdoc IPriceOracle
    function getUpdateFee(address base, address quote, bytes calldata updateData) external view returns (uint128) {
        address oracle;
        (, base, quote, oracle) = resolveOracle(0, base, quote);
        if (base == quote) return 0;
        return IPriceOracle(oracle).getUpdateFee(base, quote, updateData);
    }
}

pragma solidity ^0.8.30;

import {ISynthToken, IERC20, MarketId, AssetType} from "../interfaces/ISynthToken.sol";
import {ERC20} from "@openzeppelin/token/ERC20/ERC20.sol";

/**
 * @title Synthetic asset
 * @author Covenant Labs
 * @dev ERC20, closely integrated with CovenantCore
 */
contract SynthToken is ERC20, ISynthToken {
    /////////////////////////////////////////////////////////////////////////////////////////////
    // Errors
    error E_Synth_OnlyLEXCoreCanCall();

    /////////////////////////////////////////////////////////////////////////////////////////////
    // Modifiers
    modifier onlyLexCore() {
        if (_lexCore != _msgSender()) revert E_Synth_OnlyLEXCoreCanCall();
        _;
    }

    /////////////////////////////////////////////////////////////////////////////////////////////
    // Immutables

    address private immutable _covenantCore;
    address private immutable _lexCore; // autharized lex for mint/burn actions
    MarketId private immutable _marketId; // marketId associated with synth token
    AssetType private immutable _synthType; // type of synth token
    uint8 private immutable _decimals; // asset decimals

    ////////////////////////////////////////////////////////////////////////////////////////
    // Constructor
    constructor(
        address covenantCore_,
        address lexCore_,
        MarketId marketId_,
        IERC20 baseAsset_,
        AssetType synthType_,
        string memory name_,
        string memory symbol_,
        uint8 decimals_
    ) ERC20(name_, symbol_) {
        _covenantCore = covenantCore_;
        _lexCore = lexCore_;
        _marketId = marketId_;
        _synthType = synthType_;
        _decimals = decimals_;
    }

    ////////////////////////////////////////////////////////////////////////////////////////
    // ERC20 Overrides

    function decimals() public view override(ERC20) returns (uint8) {
        return _decimals;
    }

    ////////////////////////////////////////////////////////////////////////////////////////
    // Public Getters (non ERC20)

    function getCovenantCore() external view override returns (address) {
        return _covenantCore;
    }

    function getMarketId() external view override returns (MarketId) {
        return _marketId;
    }

    function getSynthType() external view override returns (AssetType) {
        return _synthType;
    }

    ////////////////////////////////////////////////////////////////////////////////////////
    // Covenant Liquid only functions (non ERC20)

    /**
     * @dev Expose share mint functionality to Covenant Liquid
     */
    function lexMint(address account, uint256 value) external onlyLexCore {
        _mint(account, value);
    }

    /**
     * @dev Expose share redeem functionality to Covenant Liquid
     */
    function lexBurn(address account, uint256 value) external onlyLexCore {
        _burn(account, value);
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

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {IPriceOracle} from "../interfaces/IPriceOracle.sol";
import {Errors} from "../lib/Errors.sol";

/// @title BaseAdapter
/// @custom:security-contact security@euler.xyz
/// @author Euler Labs (https://www.eulerlabs.com/)
/// @notice Abstract adapter with virtual bid/ask pricing.
abstract contract BaseAdapter is IPriceOracle {
    // @dev Addresses <= 0x00..00ffffffff are considered to have 18 decimals without dispatching a call.
    // This avoids collisions between ISO 4217 representations and (future) precompiles.
    uint256 internal constant ADDRESS_RESERVED_RANGE = 0xffffffff;

    /// @inheritdoc IPriceOracle
    function getQuote(uint256 inAmount, address base, address quote) external view returns (uint256) {
        return _getQuote(inAmount, base, quote);
    }

    /// @inheritdoc IPriceOracle
    /// @dev Does not support true bid/ask pricing.
    function getQuotes(uint256 inAmount, address base, address quote) external view returns (uint256, uint256) {
        uint256 outAmount = _getQuote(inAmount, base, quote);
        return (outAmount, outAmount);
    }

    /// @notice Determine the decimals of an asset.
    /// @param asset ERC20 token address or other asset.
    /// @dev Oracles can use ERC-7535, ISO 4217 or other conventions to represent non-ERC20 assets as addresses.
    /// Integrator Note: `_getDecimals` will return 18 if `asset` is:
    /// - any address <= 0x00000000000000000000000000000000ffffffff (4294967295)
    /// - an EOA or a to-be-deployed contract (which may implement `decimals()` after deployment).
    /// - a contract that does not implement `decimals()`.
    /// @return The decimals of the asset.
    function _getDecimals(address asset) internal view returns (uint8) {
        if (uint160(asset) <= ADDRESS_RESERVED_RANGE) return 18;
        (bool success, bytes memory data) = asset.staticcall(abi.encodeCall(IERC20.decimals, ()));
        return success && data.length == 32 ? abi.decode(data, (uint8)) : 18;
    }

    /// @notice Return the quote for the given price query.
    /// @dev Must be overridden in the inheriting contract.
    function _getQuote(uint256, address, address) internal view virtual returns (uint256);
}


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

