
## *MAIN TARGET CONTRACT* TO REVIEW

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

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.1.0) (access/Ownable2Step.sol)

pragma solidity ^0.8.20;

import {Ownable} from "./Ownable.sol";

/**
 * @dev Contract module which provides access control mechanism, where
 * there is an account (an owner) that can be granted exclusive access to
 * specific functions.
 *
 * This extension of the {Ownable} contract includes a two-step mechanism to transfer
 * ownership, where the new owner must call {acceptOwnership} in order to replace the
 * old one. This can help prevent common mistakes, such as transfers of ownership to
 * incorrect accounts, or to contracts that are unable to interact with the
 * permission system.
 *
 * The initial owner is specified at deployment time in the constructor for `Ownable`. This
 * can later be changed with {transferOwnership} and {acceptOwnership}.
 *
 * This module is used through inheritance. It will make available all functions
 * from parent (Ownable).
 */
abstract contract Ownable2Step is Ownable {
    address private _pendingOwner;

    event OwnershipTransferStarted(address indexed previousOwner, address indexed newOwner);

    /**
     * @dev Returns the address of the pending owner.
     */
    function pendingOwner() public view virtual returns (address) {
        return _pendingOwner;
    }

    /**
     * @dev Starts the ownership transfer of the contract to a new account. Replaces the pending transfer if there is one.
     * Can only be called by the current owner.
     *
     * Setting `newOwner` to the zero address is allowed; this can be used to cancel an initiated ownership transfer.
     */
    function transferOwnership(address newOwner) public virtual override onlyOwner {
        _pendingOwner = newOwner;
        emit OwnershipTransferStarted(owner(), newOwner);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`) and deletes any pending owner.
     * Internal function without access restriction.
     */
    function _transferOwnership(address newOwner) internal virtual override {
        delete _pendingOwner;
        super._transferOwnership(newOwner);
    }

    /**
     * @dev The new owner accepts the ownership transfer.
     */
    function acceptOwnership() public virtual {
        address sender = _msgSender();
        if (pendingOwner() != sender) {
            revert OwnableUnauthorizedAccount(sender);
        }
        _transferOwnership(sender);
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


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {CovenantCurator} from "../src/curators/CovenantCurator.sol";

// @dev - order needs to be alphabetical given forge-std constraints
struct OraclePair {
    address base;
    address oracle;
    address quote;
}

struct RouterConfig {
    address fallbackOracle;
    OraclePair[] oraclePairs;
}

contract DeployCurator is Script {
    function run() external {
        // Read router config JSON
        string memory jsonString = vm.readFile("./script/OracleConfig-Router.json"); //prettier-ignore
        bytes memory jsonData = vm.parseJson(jsonString);
        RouterConfig memory config = abi.decode(jsonData, (RouterConfig)); //prettier-ignore

        // Print parsing
        console.log("=== Covenant Oracle Router Configuration ===");
        console.log("Fallback Oracle:", config.fallbackOracle);
        console.log("Number of Oracle Pairs:", config.oraclePairs.length);

        vm.startBroadcast();

        // Deploy Covenant Oracle Router
        CovenantCurator router = new CovenantCurator(msg.sender);
        console.log("CovenantCurator deployed at:", address(router));

        // set fallback oracle
        router.govSetFallbackOracle(config.fallbackOracle);
        console.log("Fallback Oracle set to:", config.fallbackOracle);

        // Register oracle pairs
        console.log("\n=== Registering Oracle Pairs ===");
        for (uint256 i = 0; i < config.oraclePairs.length; i++) {
            OraclePair memory pair = config.oraclePairs[i];
            console.log("Registering pair", i + 1, ":");
            console.log("  Base:", pair.base);
            console.log("  Quote:", pair.quote);
            console.log("  Oracle:", pair.oracle);

            router.govSetConfig(pair.base, pair.quote, pair.oracle);
            console.log("  Registered successfully");
        }

        vm.stopBroadcast();

        console.log("\n=== Deployment Summary ===");
        console.log("CovenantCurator:", address(router));
        console.log("Fallback Oracle:", config.fallbackOracle);
        console.log("Registered Oracle Pairs:", config.oraclePairs.length);
    }
}

