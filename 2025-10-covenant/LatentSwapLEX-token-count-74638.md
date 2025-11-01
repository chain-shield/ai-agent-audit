
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.30;

import {ILatentSwapLEX, LexState, LexConfig, AssetType, MintParams, RedeemParams, SwapParams, MarketId, MarketParams, TokenPrices, SynthTokens, LexParams} from "./interfaces/ILatentSwapLEX.sol";
import {ILiquidExchangeModel} from "../../interfaces/ILiquidExchangeModel.sol";
import {ISynthToken, IERC20} from "../../interfaces/ISynthToken.sol";
import {IPriceOracle} from "../../interfaces/IPriceOracle.sol";
import {ITokenData} from "./interfaces/ITokenData.sol";
import {TokenData} from "./libraries/TokenData.sol";
import {FixedPoint} from "./libraries/FixedPoint.sol";
import {LSErrors} from "./libraries/LSErrors.sol";
import {LatentSwapLogic} from "./libraries/LatentSwapLogic.sol";
import {SynthToken} from "../../synths/SynthToken.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/access/Ownable2Step.sol";

/**
 * @title Latent Swap LEX
 * @author Covenant Labs
 **/

/**
 * @dev Emitted when default noCapLimit is set for a token
 * @param token the token address
 * @param oldDefaultNoCapLimit the old default noCapLimit for markets with this quote token
 * @param newDefaultNoCapLimit the new default noCapLimit for markets with this quote token
 **/
event SetDefaultNoCapLimit(address indexed token, uint8 oldDefaultNoCapLimit, uint8 newDefaultNoCapLimit);
event SetMarketNoCapLimit(MarketId indexed marketId, uint8 oldNoCapLimit, uint8 newNoCapLimit);

contract LatentSwapLEX is ILatentSwapLEX, TokenData, Ownable2Step {
    /// @inheritdoc ILiquidExchangeModel
    string public constant name = "LatentSwap V1.0"; // LEX name

    /////////////////////////////////////////////////////////////////////////////////////////////
    // Constants and immutables
    uint16 constant MAX_LIMIT_LTV = 9999; // 99.99% max limit LTV, above which aTokens cannot be minted.
    uint104 constant MAX_SQRTPRICE = uint104(8 * FixedPoint.Q96); // 64 max DEX price
    uint104 constant MIN_SQRTPRICE = uint104(FixedPoint.Q96 / 32); // 0.001 min DEX price
    uint104 constant MIN_SQRTPRICE_RATIO = uint104((1004 * FixedPoint.Q96) / 1000); // (1.0001)^80 MIN price width = 1.004 MIN sqrt price ratio.  Given this,  max market concentration is max 2^8. ie, Liquidity <= (BaseTokenValue * 2^8)
    int64 constant MAX_LN_RATE_BIAS = 405465108108164000; // ln(1.5) in WADs -> 50% max rate bias
    int64 constant MIN_LN_RATE_BIAS = -223143551314209704; // ln(0.8) in WADs -> -20% min rate bias
    uint32 constant MIN_DURATION = 1 days; // 1 day (in seconds)
    uint8 constant DEBT = 0; // used for indexing into supplyAmounts and dexAmounts
    uint8 constant LVRG = 1; // used for indexing into supplyAmounts and dexAmounts

    // Immutables
    address internal immutable _covenantCore;
    int64 internal immutable _initLnRateBias; // ln of initial rate bias (WADs)
    uint160 internal immutable _edgeSqrtPriceX96_B; // high edge of concentrated liquidity
    uint160 internal immutable _edgeSqrtPriceX96_A; // low edge of concentrated liquidity
    uint160 internal immutable _limHighSqrtPriceX96; // from which _highLTV can be derived (no aToken sales, no zToken buys)
    uint160 internal immutable _limMaxSqrtPriceX96; // from which _maxLTV can be derived (same as _highLTV && no aToken buys)
    uint32 internal immutable _debtDuration; // perpetual duration of debt, in seconds (max 100 years)
    uint8 internal immutable _swapFee; // BPS fee when swapping tokens.  Max of 2.55% swap fee

    // pre-calculated values
    uint256 internal immutable _targetXvsL; // pre-calculated static value, X96 precision

    /////////////////////////////////////////////////////////////////////////////////////////////
    // Storage

    // Map of Lex market states (marketId to data)
    mapping(MarketId marketId => LexState) internal lexState;

    // Map of Lex market configs (marketId to data)
    mapping(MarketId marketId => LexConfig) internal lexConfig;

    // Map of default NoCapLimit overrides for specific token addresses
    mapping(address token => uint8) internal tokenNoCapLimit;

    /////////////////////////////////////////////////////////////////////////////////////////////
    // Modifiers
    modifier onlyCovenantCore() {
        if (_covenantCore != _msgSender()) revert LSErrors.E_LEX_OnlyCovenantCanCall();
        _;
    }

    /////////////////////////////////////////////////////////////////////////////////////////////
    // Constructor

    constructor(
        address initialOwner_,
        address covenantCore_,
        uint160 edgeHighSqrtPriceX96_,
        uint160 edgeLowSqrtPriceX96_,
        uint160 limHighSqrtPriceX96_,
        uint160 limMaxSqrtPriceX96_,
        int64 initLnRateBias_,
        uint32 debtDuration_,
        uint8 swapFee_
    ) Ownable(initialOwner_) {
        // checks
        if (covenantCore_ == address(0)) revert LSErrors.E_LEX_ZeroAddress();

        // Check correct price ordering
        if (edgeHighSqrtPriceX96_ < limMaxSqrtPriceX96_) revert LSErrors.E_LEX_IncorrectInitializationPrice();
        if (limMaxSqrtPriceX96_ <= limHighSqrtPriceX96_) revert LSErrors.E_LEX_IncorrectInitializationPrice();
        if (limHighSqrtPriceX96_ <= FixedPoint.Q96) revert LSErrors.E_LEX_IncorrectInitializationPrice();
        if (FixedPoint.Q96 < edgeLowSqrtPriceX96_) revert LSErrors.E_LEX_IncorrectInitializationPrice();
        if (initLnRateBias_ > MAX_LN_RATE_BIAS || initLnRateBias_ < MIN_LN_RATE_BIAS)
            revert LSErrors.E_LEX_IncorrectInitializationLnRateBias();
        if (debtDuration_ < MIN_DURATION) revert LSErrors.E_LEX_IncorrectInitializationDuration();

        // check vs hardcoded limits
        if (edgeHighSqrtPriceX96_ > MAX_SQRTPRICE) revert LSErrors.E_LEX_IncorrectInitializationPrice();
        if (edgeLowSqrtPriceX96_ < MIN_SQRTPRICE) revert LSErrors.E_LEX_IncorrectInitializationPrice();
        if (((edgeHighSqrtPriceX96_ * FixedPoint.Q96) / edgeLowSqrtPriceX96_) < MIN_SQRTPRICE_RATIO)
            revert LSErrors.E_LEX_IncorrectInitializationPrice();

        // calculate maxLTV and target_dXdL_X96
        (uint256 maxLTV, uint256 target_dXdL_X96) = LatentSwapLogic.computeMaxLTVandTargetdXdL(
            edgeLowSqrtPriceX96_,
            edgeHighSqrtPriceX96_,
            limMaxSqrtPriceX96_
        );

        // check limMax <= MAX_LIMIT_LTV
        if (maxLTV > MAX_LIMIT_LTV) revert LSErrors.E_LEX_IncorrectInitializationPrice();

        // set implementation immutables
        _covenantCore = covenantCore_;
        _edgeSqrtPriceX96_B = edgeHighSqrtPriceX96_;
        _edgeSqrtPriceX96_A = edgeLowSqrtPriceX96_;
        _limHighSqrtPriceX96 = limHighSqrtPriceX96_;
        _limMaxSqrtPriceX96 = limMaxSqrtPriceX96_;
        _initLnRateBias = initLnRateBias_;
        _debtDuration = debtDuration_;
        _swapFee = swapFee_;

        // Pre-calculate static values
        // This is equivalent to dX/dL at the target price of 1
        _targetXvsL = target_dXdL_X96;
    }

    /////////////////////////////////////////////////////////////////////////////////////////////
    // OnlyOwner functions

    /// @inheritdoc ILatentSwapLEX
    function setDefaultNoCapLimit(address token, uint8 newDefaultMintRedeemNoCap) external onlyOwner {
        // @dev - if defaultMintRedeemNoCap == 0, new markets will use 1 baseToken as the MintRedeemNoCap as the default
        uint8 oldDefaulNoCapLimit = tokenNoCapLimit[token];
        tokenNoCapLimit[token] = newDefaultMintRedeemNoCap;
        emit SetDefaultNoCapLimit(token, oldDefaulNoCapLimit, newDefaultMintRedeemNoCap);
    }

    /// @inheritdoc ILatentSwapLEX
    function setMarketNoCapLimit(MarketId marketId, uint8 newNoCapLimit) external onlyOwner {
        if (lexConfig[marketId].aToken == address(0)) revert LSErrors.E_LEX_MarketDoesNotExist();
        uint8 oldNoCapLimit = lexConfig[marketId].noCapLimit;
        lexConfig[marketId].noCapLimit = newNoCapLimit;
        emit SetMarketNoCapLimit(marketId, oldNoCapLimit, newNoCapLimit);
    }

    function setQuoteTokenDecimalsOverrideForNewMarkets(address asset, uint8 newDecimals) external onlyOwner {
        _updateAssetDecimals(asset, newDecimals);
    }

    function setQuoteTokenSymbolOverrideForNewMarkets(address asset, string calldata newSymbol) external onlyOwner {
        _updateAssetSymbol(asset, newSymbol);
    }

    /////////////////////////////////////////////////////////////////////////////////////////////
    // ILiquidExchangeModel Getters

    /// @inheritdoc ILiquidExchangeModel
    function getProtocolFee(MarketId marketId) external view returns (uint32) {
        return lexConfig[marketId].protocolFee;
    }

    /// @inheritdoc ILiquidExchangeModel
    function getSynthTokens(MarketId marketId) external view returns (SynthTokens memory synthTokens) {
        synthTokens.aToken = lexConfig[marketId].aToken;
        synthTokens.zToken = lexConfig[marketId].zToken;
    }

    /////////////////////////////////////////////////////////////////////////////////////////////
    // ILatentSwapLEX Getters

    /// @inheritdoc ILatentSwapLEX
    function getLexParams() external view returns (LexParams memory) {
        return _lexParams();
    }

    /// @inheritdoc ILatentSwapLEX
    function getLexState(MarketId marketId) external view returns (LexState memory) {
        return lexState[marketId];
    }

    /// @inheritdoc ILatentSwapLEX
    function getLexConfig(MarketId marketId) external view returns (LexConfig memory) {
        return lexConfig[marketId];
    }

    /////////////////////////////////////////////////////////////////////////////////////////////
    // CovenantCore actions

    /// @inheritdoc ILiquidExchangeModel
    function initMarket(
        MarketId marketId,
        MarketParams calldata marketParams,
        uint32 protocolFee,
        bytes calldata initData
    ) external onlyCovenantCore returns (SynthTokens memory synthTokens, bytes memory lexData) {
        if (lexConfig[marketId].aToken != address(0)) revert LSErrors.E_LEX_AlreadyInitialized();

        LatentSwapLogic.MarketInitInfo memory info = LatentSwapLogic.getInitMarketInfo(
            marketParams,
            _debtDuration,
            _edgeSqrtPriceX96_A,
            _edgeSqrtPriceX96_B,
            tokenNoCapLimit[marketParams.baseToken],
            _assetDecimals(marketParams.quoteToken),
            _assetSymbol(marketParams.quoteToken)
        );

        // Create new leverage synth token
        // e.g., Symbol: ETHx2.USDT  Name: ETH x2 Leverage Coin (USDT/3M)
        synthTokens.aToken = address(
            new SynthToken(
                _covenantCore,
                address(this),
                marketId,
                IERC20(marketParams.baseToken),
                AssetType.LEVERAGE,
                info.aTokenName,
                info.aTokenSymbol,
                info.synthDecimals
            )
        );

        // Create new debt synth token
        // e.g., Symbol: USDT.bETH  Name: USDT ETH-backed Margin Coin (x2/3M)
        synthTokens.zToken = address(
            new SynthToken(
                _covenantCore,
                address(this),
                marketId,
                IERC20(marketParams.baseToken),
                AssetType.DEBT,
                info.zTokenName,
                info.zTokenSymbol,
                info.synthDecimals
            )
        );

        // Read oracle (current market price) - revert on error
        // @dev this can revert if the Oracle does not return a price, or if
        // price * (1/_targetXvsL) * (10 ^ (vars.synthDecimals - vars.quoteDecimals)) is too small
        // given oracle price being too low given other market parameters
        (uint256 currentBasePrice, ) = LatentSwapLogic.readBasePriceAndCalculateLiqRatio(
            marketParams,
            _targetXvsL,
            int8(info.quoteDecimals) - int8(info.synthDecimals),
            false
        );

        // Initialize lex config
        lexConfig[marketId] = LexConfig({
            aToken: synthTokens.aToken,
            zToken: synthTokens.zToken,
            protocolFee: protocolFee,
            noCapLimit: info.noCapLimit,
            scaleDecimals: int8(info.quoteDecimals) - int8(info.synthDecimals),
            adaptive: false
        });

        // Initialize lex state
        lexState[marketId] = LexState({
            lastBaseTokenPrice: currentBasePrice,
            lastDebtNotionalPrice: FixedPoint.WAD, //Notice: Upon market initialization, 1 zToken = 1 quoteToken in value
            lastLnRateBias: _initLnRateBias,
            lastETWAPBaseSupply: 0,
            lastSqrtPriceX96: uint160(FixedPoint.Q96), //Notice: Upon market initialization, market is at target LTV
            lastUpdateTimestamp: uint96(block.timestamp)
        });
    }

    /// @inheritdoc ILiquidExchangeModel
    function setMarketProtocolFee(MarketId marketId, uint32 newFee) external onlyCovenantCore {
        lexConfig[marketId].protocolFee = newFee;
    }

    /// @inheritdoc ILiquidExchangeModel
    // @Notice. When depositing baseTokens, how many aTokens and zTokens should be minted?
    // Target is not to have price impact from this operation, so minted amounts are all proportional
    // @dev - sender address not used (left empty)
    function mint(
        MintParams calldata mintParams,
        address,
        uint256 baseTokenSupply
    )
        external
        payable
        onlyCovenantCore
        returns (uint256 aTokenAmountOut, uint256 zTokenAmountOut, uint128 protocolFees, TokenPrices memory tokenPrices)
    {
        // Update oracle price if necessary (external call and storage write)
        _updateOraclePrice(mintParams.marketParams, mintParams.data);

        ///////////////////////////////
        // Mint logic
        LatentSwapLogic.LexFullState memory currentState;
        (currentState, tokenPrices, aTokenAmountOut, zTokenAmountOut) = LatentSwapLogic.mintLogic(
            mintParams,
            _lexParams(),
            lexConfig[mintParams.marketId],
            lexState[mintParams.marketId],
            baseTokenSupply,
            false
        );

        ///////////////////////////////
        // Write changes (storage writes)

        // Mint aTokens / zTokens (storage write)
        ISynthToken(currentState.lexConfig.aToken).lexMint(mintParams.to, aTokenAmountOut);
        ISynthToken(currentState.lexConfig.zToken).lexMint(mintParams.to, zTokenAmountOut);

        // Update lex state (storage write)
        lexState[mintParams.marketId] = currentState.lexState;

        return (aTokenAmountOut, zTokenAmountOut, currentState.accruedProtocolFee, tokenPrices);
    }

    /// @inheritdoc ILiquidExchangeModel
    // @Notice. When redeeming baseTokens, we might require to swap aTokens for zTokens before doing a balanced redeem operation
    // We calculate amount of baseTokens redeemed using stableMath logic.
    function redeem(
        RedeemParams calldata redeemParams,
        address sender,
        uint256 baseTokenSupply
    )
        external
        payable
        onlyCovenantCore
        returns (uint256 amountOut, uint128 protocolFees, TokenPrices memory tokenPrices)
    {
        // Update oracle price if necessary (external call and storage write) and check for overdeposit
        _updateOraclePrice(redeemParams.marketParams, redeemParams.data);

        ///////////////////////////////
        // Redeem logic
        LatentSwapLogic.LexFullState memory currentState;
        (currentState, tokenPrices, amountOut) = LatentSwapLogic.redeemLogic(
            redeemParams,
            _lexParams(),
            lexConfig[redeemParams.marketId],
            lexState[redeemParams.marketId],
            baseTokenSupply,
            false
        );

        ///////////////////////////////
        // Write changes (storage writes)

        // Burn aTokens / zTokens (storage write)
        ISynthToken(currentState.lexConfig.aToken).lexBurn(sender, redeemParams.aTokenAmountIn);
        ISynthToken(currentState.lexConfig.zToken).lexBurn(sender, redeemParams.zTokenAmountIn);

        // Update lex state (storage write)
        lexState[redeemParams.marketId] = currentState.lexState;

        return (amountOut, currentState.accruedProtocolFee, tokenPrices);
    }

    /// @inheritdoc ILiquidExchangeModel
    function swap(
        SwapParams calldata swapParams,
        address sender,
        uint256 baseTokenSupply
    )
        external
        payable
        onlyCovenantCore
        returns (uint256 amountCalculated, uint128 protocolFees, TokenPrices memory tokenPrices)
    {
        // Update oracle price if necessary (external call and storage write) and check for overdeposit
        _updateOraclePrice(swapParams.marketParams, swapParams.data);

        ///////////////////////////////
        // Swap logic
        LatentSwapLogic.LexFullState memory currentState;
        (currentState, tokenPrices, amountCalculated) = LatentSwapLogic.swapLogic(
            swapParams,
            _lexParams(),
            lexConfig[swapParams.marketId],
            lexState[swapParams.marketId],
            baseTokenSupply,
            false
        );

        ///////////////////////////////
        // Write changes (storage writes)

        // Burn aTokens / zTokens coming in (storage write in trusted external call)
        if (swapParams.assetIn != AssetType.BASE)
            ISynthToken(
                (swapParams.assetIn == AssetType.DEBT) ? currentState.lexConfig.zToken : currentState.lexConfig.aToken
            ).lexBurn(sender, (swapParams.isExactIn) ? swapParams.amountSpecified : amountCalculated);
        // Mint aTokens / zTokens going out (storage write in trusted external call)
        if (swapParams.assetOut != AssetType.BASE)
            ISynthToken(
                (swapParams.assetOut == AssetType.DEBT) ? currentState.lexConfig.zToken : currentState.lexConfig.aToken
            ).lexMint(swapParams.to, (swapParams.isExactIn) ? amountCalculated : swapParams.amountSpecified);

        // Update lex state (storage write)
        lexState[swapParams.marketId] = currentState.lexState;

        return (amountCalculated, currentState.accruedProtocolFee, tokenPrices);
    }

    /// @inheritdoc ILiquidExchangeModel
    function updateState(
        MarketId marketId,
        MarketParams calldata marketParams,
        uint256 baseTokenSupply,
        bytes calldata data
    ) external payable onlyCovenantCore returns (uint128 protocolFees) {
        // Update oracle price if necessary (external call and storage write) and check for overdeposit
        _updateOraclePrice(marketParams, data);

        // Calculate market state (storage read)
        LatentSwapLogic.LexFullState memory currentState = LatentSwapLogic.calculateMarketState(
            marketParams,
            _lexParams(),
            lexConfig[marketId],
            lexState[marketId],
            baseTokenSupply,
            false
        );

        // Update lex state (storage write)
        lexState[marketId] = currentState.lexState;

        return currentState.accruedProtocolFee;
    }

    /////////////////////////////////////////////////////////////////////////////////////////////
    // Quotes

    /// @inheritdoc ILiquidExchangeModel
    function quoteMint(
        MintParams calldata mintParams,
        address,
        uint256 baseTokenSupply
    )
        external
        view
        returns (
            uint256 aTokenAmountOut,
            uint256 zTokenAmountOut,
            uint128 protocolFees,
            uint128 oracleUpdateFee,
            TokenPrices memory tokenPrices
        )
    {
        oracleUpdateFee = _getOracleUpdateFee(mintParams.marketParams, mintParams.data); // Get oracle update fee, if any
        LatentSwapLogic.LexFullState memory currentState;
        (currentState, tokenPrices, aTokenAmountOut, zTokenAmountOut) = LatentSwapLogic.mintLogic(
            mintParams,
            _lexParams(),
            lexConfig[mintParams.marketId],
            lexState[mintParams.marketId],
            baseTokenSupply,
            true
        );
        return (aTokenAmountOut, zTokenAmountOut, currentState.accruedProtocolFee, oracleUpdateFee, tokenPrices);
    }

    /// @inheritdoc ILiquidExchangeModel
    function quoteRedeem(
        RedeemParams calldata redeemParams,
        address,
        uint256 baseTokenSupply
    )
        external
        view
        returns (uint256 amountOut, uint128 protocolFees, uint128 oracleUpdateFee, TokenPrices memory tokenPrices)
    {
        oracleUpdateFee = _getOracleUpdateFee(redeemParams.marketParams, redeemParams.data); // Get oracle update fee, if any
        LatentSwapLogic.LexFullState memory currentState;
        (currentState, tokenPrices, amountOut) = LatentSwapLogic.redeemLogic(
            redeemParams,
            _lexParams(),
            lexConfig[redeemParams.marketId],
            lexState[redeemParams.marketId],
            baseTokenSupply,
            true
        );
        return (amountOut, currentState.accruedProtocolFee, oracleUpdateFee, tokenPrices);
    }

    /// @inheritdoc ILiquidExchangeModel
    function quoteSwap(
        SwapParams calldata swapParams,
        address,
        uint256 baseTokenSupply
    )
        external
        view
        returns (
            uint256 amountCalculated,
            uint128 protocolFees,
            uint128 oracleUpdateFee,
            TokenPrices memory tokenPrices
        )
    {
        oracleUpdateFee = _getOracleUpdateFee(swapParams.marketParams, swapParams.data); // Get oracle update fee, if any
        LatentSwapLogic.LexFullState memory currentState;
        (currentState, tokenPrices, amountCalculated) = LatentSwapLogic.swapLogic(
            swapParams,
            _lexParams(),
            lexConfig[swapParams.marketId],
            lexState[swapParams.marketId],
            baseTokenSupply,
            true
        );
        return (amountCalculated, currentState.accruedProtocolFee, oracleUpdateFee, tokenPrices);
    }

    /////////////////////////////////////////////////////////////////////////////////////////////
    // Internal functions

    function _lexParams() internal view returns (LexParams memory lexParams) {
        return
            LexParams({
                covenantCore: _covenantCore,
                initLnRateBias: _initLnRateBias, // Init rate bias (in LN terms, WADs)
                edgeSqrtPriceX96_B: _edgeSqrtPriceX96_B, // high edge of concentrated liquidity
                edgeSqrtPriceX96_A: _edgeSqrtPriceX96_A, // low edge of concentrated liquidity
                limHighSqrtPriceX96: _limHighSqrtPriceX96, // from which _highLTV can be derived (no aToken sales, no zToken buys)
                limMaxSqrtPriceX96: _limMaxSqrtPriceX96, // from which _maxLTV can be derived (same as _highLTV && no aToken buys)
                debtDuration: _debtDuration, // perpetual duration of debt, in seconds (max 100 years)
                swapFee: _swapFee, // BPS fee when swapping tokens.  Max of 2.55% swap fee
                targetXvsL: _targetXvsL // pre-calculated liquidity concentration
            });
    }

    /////////////////////////////////////////////////////////////////////////////////////////////
    // Pull Oracle functions

    // updatePrices if there is data
    function _updateOraclePrice(MarketParams calldata marketParams, bytes calldata data) internal {
        // send data package and msgValue to Oracle, if data was sent
        if (data.length > 0)
            IPriceOracle(marketParams.curator).updatePriceFeeds{value: msg.value}(
                marketParams.baseToken,
                marketParams.quoteToken,
                data
            );
        else if (msg.value > 0) revert LSErrors.E_LEX_Overdeposit();
    }

    function _getOracleUpdateFee(
        MarketParams calldata marketParams,
        bytes calldata data
    ) internal view returns (uint128 oracleFee) {
        return
            (data.length > 0)
                ? IPriceOracle(marketParams.curator).getUpdateFee(marketParams.baseToken, marketParams.quoteToken, data)
                : 0;
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import {SafeMetadata, IERC20} from "../../../libraries/SafeMetadata.sol";
import {ITokenData} from "../interfaces/ITokenData.sol";

/// @title TokenData
/// @author Covenant Labs
/// @notice sets symbol, decimals and name overrides for a token
/// @dev each item can be set independently, and will override existing ERC20 values for the respecitve token
/// @dev if both symbol and decimals are overriden, a quote token need not be an actual ERC20
/// @dev this gives the flexibility to use currency ISO addresses and symbols for quote tokens.
/// @dev Oracles can use ERC-7535, ISO 4217 or other conventions to represent non-ERC20 assets as addresses.
/// @dev e.g., EIP7528 would set address = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE", symbol = "ETH", decimals = 18.
abstract contract TokenData is ITokenData {
    using SafeMetadata for IERC20;

    // mapping of decimal overrides for specific assets
    mapping(address => uint8) _decimals;

    // mapping of symbol overrides for specific assets
    mapping(address => string) _symbol;

    // mapping of name overrides for specific assets
    mapping(address => string) _name;

    error TokenData_InvalidDecimals();

    event SetTokenDecimals(address indexed token, uint8 oldDecimals, uint8 newDecimals);
    event SetTokenSymbol(address indexed token, string oldSymbol, string newSymbol);
    event SetTokenName(address indexed token, string oldName, string newName);

    function assetDecimals(address asset) public view returns (uint8 decimals_) {
        return _assetDecimals(asset);
    }

    function assetSymbol(address asset) public view returns (string memory symbol_) {
        return _assetSymbol(asset);
    }

    function assetName(address asset) public view returns (string memory name_) {
        return _assetName(asset);
    }

    //////////////////////////////////////////////////////////////////////////

    // @dev - Stored decimals are an override.
    // if stored decimals is 0, try and get ERC20 decimals
    function _assetDecimals(address asset) internal view returns (uint8 decimals_) {
        decimals_ = _decimals[asset]; //check if there is an override for this asset
        if (decimals_ > 0) return decimals_;
        else {
            // try and read from asset itself
            bool success;
            (success, decimals_) = IERC20(asset).tryGetDecimals();
            return success ? decimals_ : 18;
        }
    }

    // @dev - Stored symbol are an override.
    // @dev - if stored symbol is "", try and get ERC20 symbol
    function _assetSymbol(address asset) internal view returns (string memory symbol_) {
        symbol_ = _symbol[asset]; //check if there is an override for this asset
        if (bytes(symbol_).length > 0) return symbol_;
        else {
            // try and read from asset itself
            bool success;
            (success, symbol_) = IERC20(asset).tryGetSymbol();
            return success ? symbol_ : "";
        }
    }

    // @dev - Stored name are an override.
    // @dev - if stored name is "", try and get ERC20 name
    function _assetName(address asset) internal view returns (string memory name_) {
        name_ = _name[asset]; //check if there is an override for this asset
        if (bytes(name_).length > 0) return name_;
        else {
            // try and read from asset itself
            bool success;
            (success, name_) = IERC20(asset).tryGetName();
            return success ? name_ : "";
        }
    }

    // internal functions.  These should be exposed with the appropriate access modifiers
    // @dev - if newDecimals = 0, then _assetDecimals will try and get ERC20 decimals
    function _updateAssetDecimals(address asset, uint8 newDecimals) internal {
        if (newDecimals > 18) revert TokenData_InvalidDecimals();
        uint8 oldDecimals = _assetDecimals(asset); // get old decimals
        _decimals[asset] = newDecimals;
        emit SetTokenDecimals(asset, oldDecimals, newDecimals);
    }

    // internal functions.  These should be exposed with the appropriate access modifiers
    // @dev - if newSymbol = "", then _assetSymbol will try and get ERC20 symbol
    function _updateAssetSymbol(address asset, string calldata newSymbol) internal {
        string memory oldSymbol = _assetSymbol(asset); // get old symbol
        _symbol[asset] = newSymbol;
        emit SetTokenSymbol(asset, oldSymbol, newSymbol);
    }

    // internal functions.  These should be exposed with the appropriate access modifiers
    // @dev - if newName = "", then _assetNAme will try and get ERC20 name
    function _updateAssetName(address asset, string calldata newName) internal {
        string memory oldName = _assetName(asset); // get old symbol
        _name[asset] = newName;
        emit SetTokenName(asset, oldName, newName);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.30;

import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {Math} from "@openzeppelin/utils/math/Math.sol";
import {SaturatingMath} from "./SaturatingMath.sol";
import {FixedPoint} from "./FixedPoint.sol";

/**
 * @title DebtMath library
 * @author Covenant Labs
 * @notice Provides approximations for Perpetual Debt calculations
 */
library DebtMath {
    using SaturatingMath for uint256;
    using FixedPointMathLib for int256;
    using Math for uint256;

    uint256 internal constant SECONDS_PER_YEAR = 365 days;

    /**
     * @notice calculates interest update factor given perpetual debt duration, debt notional price and elapsed time.
     * @param _amount amount on which interest is being applied
     * @param _duration effective duration of the debt (in seconds)
     * @param _discountPrice discount price (vs debt notional) in WADs
     * @param _elapsedTime time over which to accrue interest (in seconds)
     * @param _lnRateBias additional market rate bias (that is not determined by price), in WADs
     * @return updatedAmount_  the updated amount given debt interest rate and elapsed time
     **/
    function accrueInterest(
        uint256 _amount,
        uint256 _duration,
        uint256 _discountPrice,
        uint256 _elapsedTime,
        int256 _lnRateBias
    ) internal pure returns (uint256 updatedAmount_) {
        // Calculate rate = - ln(price) + lnRateBias
        // and then updates amount.
        return accrueInterestLnRate(_amount, _lnRateBias - int256(_discountPrice).lnWad(), _elapsedTime, _duration);
    }

    /**
     * @notice updates amount given duration, lnRate and elapsed time.
     * @dev interest accrual saturates.  ie, calculation will not revert,
     * and instead updatedAmount will be >0 and <=type(uint256).max
     * @param _amount amount to be update given duration, lnRate and elapsed time.
     * @param _duration effective duration of the debt (in seconds)
     * @param _lnRate lnRate in WADs (lnRate < 1 is a negative interest rate)
     * @param _elapsedTime time over which to accrue interest (in seconds)
     * @return updatedAmount_  the updated amount given debt interest rate and elapsed time
     **/
    function accrueInterestLnRate(
        uint256 _amount,
        int256 _lnRate,
        uint256 _elapsedTime,
        uint256 _duration
    ) internal pure returns (uint256 updatedAmount_) {
        uint256 updateFactor = calculateApproxExponentialUpdate(
            uint256((_lnRate >= 0) ? _lnRate : -_lnRate),
            _elapsedTime,
            _duration
        );

        if (_lnRate >= 0) {
            return _amount.saturatingMulDiv(updateFactor, FixedPoint.RAY);
        } else {
            // @dev - when lnRate < 0, we calculate exp(x), but then divide _amount by that updatefactor.
            // given e(-x) = 1 / e(x). Amount is never allowed to get to 0 from interest accrual.
            updatedAmount_ = _amount.mulDiv(FixedPoint.RAY, updateFactor);
            if (updatedAmount_ == 0 && _amount > 0) updatedAmount_ = 1;
        }
    }

    /**
     * @notice Calculates approximation of exp(lnRate * timeDelta / duration) for small values of rate * timeDelta / duration
     * @dev rate * timeDelta / duration is considered small, given timeDelta << duration, and rangebound rate
     * @dev A taylor expansion is used to calculate exp(rate * timeDelta / duration), and output will alwas be <= to the exact calculation.
     * @dev Below calculation does not overflow, even in extremes.  e.g, max lnRAte
     * @dev below does not overflow for reasonable extremes.  E.g, duration of 1 year (in seconds), time elapsed of 10,000 years, lnRate = 7.9 RAYS (= 250000% daily rate)
     * @param _lnRate logaritmic rate. -ln(price) in WADs
     * @param _timeDelta time over which to accrue interest (in seconds)
     * @param _duration effective duration of the debt (in seconds)
     * @return updateMultiplier_ the update multiplier (in RAYs) with which to update an amount
     **/
    function calculateApproxExponentialUpdate(
        uint256 _lnRate,
        uint256 _timeDelta,
        uint256 _duration
    ) internal pure returns (uint256 updateMultiplier_) {
        // @dev- for extreme cases (e.g., daily 10000% interest rate over 10000 years, with duration = 1 day),
        // both _lnRate and _timeDelta are expected to be < uint96.max, and the below
        // calculation not to revert.

        // approximation for exp(lnRate * timeDelta / duration)
        uint256 rate1 = (_lnRate * _timeDelta * FixedPoint.WAD_RAY_RATIO) / _duration;
        uint256 rate2 = rate1.mulDiv(rate1, 2 * FixedPoint.RAY);
        uint256 rate3 = rate2.mulDiv(rate1, 3 * FixedPoint.RAY);
        return FixedPoint.RAY + rate1 + rate2 + rate3;
    }

    // returns linear update multiplier (ray units)
    // assumes rate in BPS for a yearly duration
    // @dev - output value saturates at type(uint256).max
    function calculateLinearAccrual(
        uint256 _value,
        uint256 _rateBPS,
        uint256 _timeDelta
    ) internal pure returns (uint256 accrualValue_) {
        // @dev - Even for extreme rate and timeDelta cases, _rate expected to be < type(uint160).max
        // and _timeDelta < type(uint96).max.  Given this, below does not revert for any
        // _value <= type(uint256).max.

        return _value.saturatingMulDiv(_rateBPS * _timeDelta, SECONDS_PER_YEAR * FixedPoint.PERCENTAGE_FACTOR);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.30;

/// @title Utils Library
/// @author Covenant Labs
/// @notice Library to convert a market to its id.
library UtilsLib {
    function encodeFee(uint16 yieldFee, uint16 tvlFee) internal pure returns (uint32 protocolFee) {
        return ((uint32(yieldFee) << 16) | uint32(tvlFee));
    }

    function decodeFee(uint32 protocolFee) internal pure returns (uint16 yieldFee, uint16 tvlFee) {
        yieldFee = uint16(protocolFee >> 16);
        tvlFee = uint16(protocolFee & 0xFFFF);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {IERC20Metadata} from "@openzeppelin/token/ERC20/extensions/IERC20Metadata.sol";

library SafeMetadata {
    /**
     * @dev Attempts to fetch the asset name as a string. A return value of false indicates that the attempt failed in some way.
     */
    function tryGetName(IERC20 token) internal view returns (bool ok, string memory out) {
        return _tryStringOrBytes32(address(token), IERC20Metadata.name.selector);
    }

    /**
     * @dev Attempts to fetch the asset symbol as a string. A return value of false indicates that the attempt failed in some way.
     */
    function tryGetSymbol(IERC20 token) internal view returns (bool ok, string memory out) {
        return _tryStringOrBytes32(address(token), IERC20Metadata.symbol.selector);
    }

    /**
     * @dev Attempts to fetch the asset decimals. A return value of false indicates that the attempt failed in some way.
     */
    function tryGetDecimals(IERC20 token) internal view returns (bool ok, uint8 assetDecimals) {
        (bool success, bytes memory encodedDecimals) = address(token).staticcall(
            abi.encodeCall(IERC20Metadata.decimals, ())
        );
        if (success && encodedDecimals.length >= 32) {
            uint256 returnedDecimals = abi.decode(encodedDecimals, (uint256));
            if (returnedDecimals <= type(uint8).max) {
                return (true, uint8(returnedDecimals));
            }
        }
        return (false, 0);
    }

    function _tryStringOrBytes32(address token, bytes4 selector) private view returns (bool ok, string memory out) {
        // Enforce read-only
        (bool success, bytes memory data) = token.staticcall(abi.encodeWithSelector(selector));
        if (!success) return (false, "");

        // Try standard (string).  Reverts on malformed data.
        if (data.length >= 64) return (true, abi.decode(data, (string)));

        // Fallback: bytes32 (older tokens)
        if (data.length == 32) {
            bytes32 raw = abi.decode(data, (bytes32));
            return (true, _bytes32ToString(raw));
        }

        // Anything else: treat as failure
        return (false, "");
    }

    // separate to allow try/catch
    function _decodeString(bytes memory data) internal pure returns (string memory s) {
        return abi.decode(data, (string));
    }

    function _bytes32ToString(bytes32 x) private pure returns (string memory) {
        uint256 len = 32;
        while (len > 0 && x[len - 1] == 0) {
            unchecked {
                len--;
            }
        }
        bytes memory out = new bytes(len);
        for (uint256 i = 0; i < len; ++i) out[i] = x[i];
        return string(out);
    }
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

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.30;

import {AssetType} from "../../../interfaces/ILiquidExchangeModel.sol";
import {Math} from "@openzeppelin/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/utils/math/SafeCast.sol";
import {FixedPoint} from "./FixedPoint.sol";
import {SqrtPriceMath} from "./SqrtPriceMath.sol";
import {Uint512} from "./Uint512.sol";

/**
 * @title Latent Math
 * @author Covenant Labs
 * @dev Library containing all the DEX functions for the exchange of liquidity vs leverage value and debt value.
 **/

library LatentMath {
    using Math for uint256;
    using SafeCast for uint256;
    using SafeCast for bool;

    struct ComputeLiquidityVars {
        uint256 pDiffX96;
        uint256 betaX96;
        uint256 b2X192_0;
        uint256 b2X192_1;
        uint256 qX192_0;
        uint256 qX192_1;
        uint256 dX192_0;
        uint256 dX192_1;
    }

    /**
     * @notice Computes the liquidity invariant given the current balances.
     * @dev - For covenant market limts (max 90% > LTV >= 50%, 1.6 > Pb/Pa > 1.004, price = 1 at target LTV),
     * we find aTokenAmount and zTokenAmount should be < 2^152 to ensure no overflows and liquidity < 2^160.
     * @param sqrtRatioX96_A low market price edge (price (token1/token0) when token0 = 0)
     * @param sqrtRatioX96_B high market price edge ( price (token1/token0) when token1 = 0)
     * @param zTokenAmount amount of zTokens in market
     * @param aTokenAmount amount of aTokens in market
     * @return liquidity The calculated liquidity of the pool
     */
    function computeLiquidity(
        uint160 sqrtRatioX96_A,
        uint160 sqrtRatioX96_B,
        uint256 zTokenAmount,
        uint256 aTokenAmount
    ) internal pure returns (uint160) {
        /**********************************************************************************************
        // invariant                                                                                 //
        // L = invariant                               (L/PA - BX)(L.PB - BY) = L^2                  //
        // PA = sqrt(price_0)                                                                        // 
        // PB = sqrt(price_1)                                                                        //
        // BX = balance of coin X                                                                    //
        // BY = balance of coin Y          `                                                         //
        // @dev - reverts on overflow                                                                //
        //                                                                                           //
        // Computes L using square root solution                                                     //
        //    L = beta + sqrt(beta^2 - q)                                                            //
        // where                                                                                     //
        //    beta = (Y + X*sqrt(p_a)*sqrt(p_b))/(2*(sqrt(p_b)-sqrt(p_a)))                           //
        //    q = X*Y*sqrt(p_a)/(sqrt(p_b)-sqrt(p_a))                                                //
        //                                                                                           //
        //                                                                                           //
        **********************************************************************************************/
        ComputeLiquidityVars memory vars;
        // @dev - For covenant market limts (max LTV = 90%, 1.6 > Pb/Pa > 1.004),
        // we find that BaseTokenValue < Liquidity < 2^8 * BaseTokenValue.
        // So BaseTokenValue < 2^152 to ensure Liquidity < 2^160 across markets.
        // In addition, given market concavity, aTokenAmount < BaseTokenValue, and zTokenAmount < BaseTokenValue.
        // At any point within the market limits.

        // @dev - assumes sqrtPrice_B > sqrtPrice_A (otherwise reverts)
        vars.pDiffX96 = (sqrtRatioX96_B - sqrtRatioX96_A);

        // beta with X96 precision
        // b = (Y + X*sqrt(p_a)*sqrt(p_b))/(2*(sqrt(p_b)-sqrt(p_a)))
        // @dev - For covenant market limts (max LTV = 90%, Pb/Pa < 1.004),
        // we have 1/(2*(sqrt(p_b)-sqrt(p_a))) < 2^8
        // For markets with LTV > 50%, we also have sqrtRatioX96_A * sqrtRatioX96_B <= FixedPoint.Q192
        // so, combined, FixedPoint.Q192 / (2 * vars.pDiffX96) < 2^8 * FixedPoint.Q96 < 2^102
        // Thus, aTokenAmount and zTokenAmount < 2^154 for betaX96 to fit in uint256.
        // (but should be further constrained to < 2^152 given previous comments)
        vars.betaX96 =
            Math.mulDiv(aTokenAmount, FixedPoint.Q192, vars.pDiffX96 << 1) +
            Math.mulDiv(zTokenAmount * sqrtRatioX96_A, sqrtRatioX96_B, vars.pDiffX96 << 1);

        // beta^2 with X192 precision (512)
        (vars.b2X192_1, vars.b2X192_0) = Math.mul512(vars.betaX96, vars.betaX96);

        // q = X*Y*sqrt(p_a)/(sqrt(p_b)-sqrt(p_a)), with X192 precision (512 bits)
        // biggest aTokenAmount <= biggest zTokenAmount for markets with LTV >= 50%
        // Liquidity =< aTokenAmount + zTokenAmount (== when currentPrice == 1 only).
        // Given market constraints, sqrtA / (sqrtB - sqrtA) < 2^8.  Thus, aTokenAmount < 2^152 so as to not overflow.
        // zTokenAmount can be < 2^160 in calculation below.
        (vars.qX192_1, vars.qX192_0) = Math.mul512(
            Math.mulDiv(
                aTokenAmount,
                uint256(sqrtRatioX96_A) << FixedPoint.RESOLUTION,
                vars.pDiffX96,
                Math.Rounding.Ceil
            ),
            zTokenAmount * FixedPoint.Q96
        );

        // add (beta^2 - q) if it is > 0. otherwise disrgard this term. (this could happen due to rounding for lower balances)
        // check beta^2 > q for 512 bit numbers.
        if ((vars.qX192_1 == vars.b2X192_1 || vars.qX192_0 < vars.b2X192_0) || vars.qX192_1 < vars.b2X192_1) {
            // calculate difference.
            (vars.dX192_0, vars.dX192_1) = Uint512.sub512x512(vars.b2X192_0, vars.b2X192_1, vars.qX192_0, vars.qX192_1);
            // add sqrt of difference
            vars.betaX96 += Uint512.sqrt512(vars.dX192_0, vars.dX192_1);
        }

        // @dev Return does not require toUint160() SafeCast given FixedPoint.RESOLUTION shiftRight.
        return uint160(vars.betaX96 >> FixedPoint.RESOLUTION);
    }

    /**
     * @notice Computes the required swap amounts
     * @dev Does not verify price limits
     * @dev - For large liquidity amounts, and low tokenIn amounts, the output will be zero
     * @dev - Reverts for currentLiquidity == 0, currentSqrtRatioX96 == 0
     * @param currentLiquidity The current liquidity
     * @param currentSqrtRatioX96 The current market price
     * @param tokenSpecified The index of the token that is specified for the swap
     * @param amountSpecified The exact amount specified for the swap
     * @param isExactIn Whether the amounts specified are for the token coming into the swap or not.
     * @return amountCalculated The calculated amount (amountOut if isExactIn, or amountIn if !IsExactIn)
     * @return nextSqrtRatioX96 The next market price after swap
     */
    function computeSwap(
        uint160 currentLiquidity,
        uint160 currentSqrtRatioX96,
        AssetType tokenSpecified,
        uint256 amountSpecified,
        bool isExactIn
    ) internal pure returns (uint256 amountCalculated, uint160 nextSqrtRatioX96) {
        // In latent swaps, when an amount is coming in, it 'decreases' the balance of that token
        // The amount out comes from an 'increase' in balance
        if (tokenSpecified == AssetType.DEBT) {
            // For exactIn, round to make sure we do not pass the target price. Given price is going down, round up.
            // For !exactIn, round to make sure we pass the target price. Given price is going up, round up.
            nextSqrtRatioX96 = SqrtPriceMath.getNextSqrtPriceFromAmount0(
                currentSqrtRatioX96,
                currentLiquidity,
                amountSpecified,
                isExactIn,
                Math.Rounding.Ceil
            );

            // round down is exactIn, up otherwise
            amountCalculated = SqrtPriceMath.getAmount1Delta(
                currentSqrtRatioX96,
                nextSqrtRatioX96,
                currentLiquidity,
                isExactIn ? Math.Rounding.Floor : Math.Rounding.Ceil
            );
        } else if (tokenSpecified == AssetType.LEVERAGE) {
            // For exactIn, round to make sure we do not pass the target price. Given price is going up, round down.
            // For !exactIn, round to make sure we pass the target price. Given price is going down, round down.
            nextSqrtRatioX96 = SqrtPriceMath.getNextSqrtPriceFromAmount1(
                currentSqrtRatioX96,
                currentLiquidity,
                amountSpecified,
                isExactIn,
                Math.Rounding.Floor
            );

            // round down is exactIn, up otherwise
            amountCalculated = SqrtPriceMath.getAmount0Delta(
                currentSqrtRatioX96,
                nextSqrtRatioX96,
                currentLiquidity,
                isExactIn ? Math.Rounding.Floor : Math.Rounding.Ceil
            );
        } else revert();
    }

    /**
     * @notice Calculate amount of a + z Tokens minted when a fixed amount of liquidity is added
     * @dev Does not change current market price
     * @dev Round calculated balance down.
     * @param currentSqrtRatioX96 current market price
     * @param  edgeSqrtRatioX96_A low market price edge
     * @param  edgeSqrtRatioX96_B high market price edge
     * @param liquidityIn Liquidity being added to the market
     * @return zTokenAmount zTokenAmount minted
     * @return aTokenAmount aTokenAmount minted
     **/
    function computeMint(
        uint160 currentSqrtRatioX96,
        uint160 edgeSqrtRatioX96_A,
        uint160 edgeSqrtRatioX96_B,
        uint160 liquidityIn
    ) internal pure returns (uint256 zTokenAmount, uint256 aTokenAmount) {
        zTokenAmount = SqrtPriceMath.getAmount0Delta(
            currentSqrtRatioX96,
            edgeSqrtRatioX96_A,
            liquidityIn,
            Math.Rounding.Floor
        );
        aTokenAmount = SqrtPriceMath.getAmount1Delta(
            edgeSqrtRatioX96_B,
            currentSqrtRatioX96,
            liquidityIn,
            Math.Rounding.Floor
        );
    }

    /**
     * @notice Computes liquidity out given exact zToken and aToken amounts
     * @dev - Estimates liquidity out, with higher error the bigger % of liquidity being redeemed given market size
     * @dev - Keeps nextSqrtRatioX96 within bounds
     * @param currentLiquidity The current liquidity
     * @param currentSqrtRatioX96 The current market price
     * @param edgeSqrtRatioX96_A low market price edge
     * @param edgeSqrtRatioX96_B high market price edge
     * @param zTokenAmtIn The exact amount of zToken specified for the redeem
     * @param zTokenAmtIn The exact amount of zToken specified for the redeem
     * @return liquidityOut The liquidity amount calculated for redeem
     * @return nextSqrtRatioX96 The next market price after swap
     */
    function computeRedeem(
        uint160 currentLiquidity,
        uint160 currentSqrtRatioX96,
        uint160 edgeSqrtRatioX96_A,
        uint160 edgeSqrtRatioX96_B,
        uint256 zTokenAmtIn,
        uint256 aTokenAmtIn
    ) internal pure returns (uint160 liquidityOut, uint160 nextSqrtRatioX96) {
        // Calculate current market dex amounts
        // @dev - given getMarketStateFromLiquidityAndDebt(),
        // calculated aDexAmount == actual aDexAmount in circulation
        // calculated zDexAmount <= actual zDexAmount in circulation
        // (given current market price)

        (uint256 zDexAmount, uint256 aDexAmount) = computeMint(
            currentSqrtRatioX96,
            edgeSqrtRatioX96_A,
            edgeSqrtRatioX96_B,
            currentLiquidity
        );

        if (zTokenAmtIn >= zDexAmount && aTokenAmtIn >= aDexAmount) {
            //Full burn
            return (currentLiquidity, currentSqrtRatioX96);
        } else {
            // Calculate remaining zToken and aToken amounts after redeem (add 1 to overestimate)
            uint256 remZamt = (zTokenAmtIn < zDexAmount) ? zDexAmount - zTokenAmtIn + 1 : 0;
            uint256 remAamt = (aTokenAmtIn < aDexAmount) ? aDexAmount - aTokenAmtIn + 1 : 0;

            // Calculate remaining liquidity (add 1 to force rounding up)
            uint256 remLiq = (uint256(computeLiquidity(edgeSqrtRatioX96_A, edgeSqrtRatioX96_B, remZamt, remAamt)) + 1);

            // set max remLiq as currentLiquidity (no need to safeCast remLiq after this)
            if (remLiq > uint256(currentLiquidity)) remLiq = currentLiquidity;
            liquidityOut = currentLiquidity - uint160(remLiq);
            nextSqrtRatioX96 = (liquidityOut == 0)
                ? currentSqrtRatioX96
                : SqrtPriceMath.getNextSqrtPriceFromAmount0(
                    edgeSqrtRatioX96_A,
                    uint160(remLiq),
                    remZamt,
                    false,
                    Math.Rounding.Ceil
                );
            if (nextSqrtRatioX96 > edgeSqrtRatioX96_B) nextSqrtRatioX96 = edgeSqrtRatioX96_B;
            return (liquidityOut, nextSqrtRatioX96);
        }
    }

    /**
     * @notice Calculate the derivative of liquidity vs a given token (at tokenType), given all current balances.
     * @param currentSqrtRatioX96 The current sqrtRatio of the market
     * @param  edgeSqrtRatioX96_A low market price edge (price (token1/token0) when token0 = 0)
     * @param  edgeSqrtRatioX96_B high market price edge ( price (token1/token0) when token1 = 0)
     * @param tokenType the token balance we are calculating
     * @return ratioX96 The derivative (spot price) of token vs liquidity, with X96 precision
     * @dev the inverse derivatives are as follows
     * dX/dL = 1/sqrt(Pa) - 2/sqrt(P) + sqrt(Pb)/P
     * dY/dL = sqrt(Pb) - 2sqrt(P)+ P/sqrt(Pa)
     * where Pa and Pb are the edge prices, and P is the current market spot price.
     */
    function get_XvsL(
        uint160 currentSqrtRatioX96,
        uint160 edgeSqrtRatioX96_A,
        uint160 edgeSqrtRatioX96_B,
        AssetType tokenType
    ) internal pure returns (uint256 ratioX96) {
        if (tokenType == AssetType.DEBT) {
            // calculates inverse derivative with Q96 resolution
            ratioX96 =
                Math.mulDiv(edgeSqrtRatioX96_B, FixedPoint.Q192, currentSqrtRatioX96) /
                currentSqrtRatioX96 +
                FixedPoint.Q192 /
                uint256(edgeSqrtRatioX96_A) -
                (FixedPoint.Q192 << 1) /
                currentSqrtRatioX96;
        } else if (tokenType == AssetType.LEVERAGE) {
            ratioX96 =
                Math.mulDiv(currentSqrtRatioX96, currentSqrtRatioX96, edgeSqrtRatioX96_A) +
                uint256(edgeSqrtRatioX96_B) -
                (uint256(currentSqrtRatioX96) << 1);
        } else revert();
    }

    /**
     * @notice Functionality used in LatentSwapLEX to calculate aDexAmount + marketSqrt price given zDexAmount + market Liquidity
     * @dev - functionality isolated into this function for testing purposes, to ensure updateMarket + redeem functionality are aligned
     * @param edgeSqrtRatioX96_A low market price edge
     * @param edgeSqrtRatioX96_B high market price edge
     * @param liquidity The current market liquidity
     * @param zDexAmount The exact amount of zToken specified for the redeem
     * @return aDexAmount The derived aDexAmount
     * @return currentSqrtPriceX96 The derived marketSqrtPriceX96
     */
    function getMarketStateFromLiquidityAndDebt(
        uint160 edgeSqrtRatioX96_A,
        uint160 edgeSqrtRatioX96_B,
        uint160 liquidity,
        uint256 zDexAmount
    ) internal pure returns (uint256 aDexAmount, uint160 currentSqrtPriceX96) {
        // calculate market price given liquidity and zDexAmount
        // Round up current price (implicitly rounds up debt value, rounds down aDexAmount in next calculation)
        currentSqrtPriceX96 = SqrtPriceMath.getNextSqrtPriceFromAmount0(
            edgeSqrtRatioX96_A,
            liquidity,
            zDexAmount,
            false,
            Math.Rounding.Ceil
        );

        // calculate aSynthAmount given current price and market liquidity
        // Round down aDexAmount
        aDexAmount = SqrtPriceMath.getAmount1Delta(
            edgeSqrtRatioX96_B,
            currentSqrtPriceX96,
            liquidity,
            Math.Rounding.Floor
        );
    }

    /**
     * @notice Calculates marginal value of L vs debt when market price == 1
     * @param edgeSqrtRatioX96_A low market price edge
     * @param edgeSqrtRatioX96_B high market price edge
     * @return targetXvsL XvsL when market on target (price == 1), with X96 precision
     */
    function targetXvsL(uint160 edgeSqrtRatioX96_A, uint160 edgeSqrtRatioX96_B) internal pure returns (uint256) {
        return get_XvsL(uint160(FixedPoint.Q96), edgeSqrtRatioX96_A, edgeSqrtRatioX96_B, AssetType.DEBT);
    }

    /**
     * @notice Calculates LTV given current market price and market edges.
     * @param edgeSqrtRatioX96_A low market price edge
     * @param edgeSqrtRatioX96_B high market price edge
     * @param currentSqrtRatioX96 current market price
     * @return LTV where 10000 = 100%
     */
    function computeLTV(
        uint160 edgeSqrtRatioX96_A,
        uint160 edgeSqrtRatioX96_B,
        uint160 currentSqrtRatioX96
    ) internal pure returns (uint256) {
        /**********************************************************************************************
        // At extremes, all X or all Y markets have equivalent value                                 //
        // (equal to value of collateral)                                                            //
        //                                                                                           //
        // All X coin Amount0 = Pb - Pa / Pb / Pa                                                    //
        // All Y coin Amount1 = Pb - Pa                                                              //
        //                                                                                           //
        // Where                                                                                     //
        // Pa = sqrt(price_0)                                                                        //
        // Pb = sqrt(price_1)                                                                        //
        // Pc = sqrt(price_current)                                                                  //
        //                                                                                           //
        //                                                                                           //
        // Thus, from a value perspective, if Vx = Amount0, then Vy = Amount1 * Pb * Pa              //
        //                                                                                           //
        // We define LTV = Vx / (Vy + Vx)                                                            //
        //               = Amount0 / (Amount0 + Amount1 * Pb * Pa)                                   //
        //                                                                                           //
        // Solving this (using Amount0 = 1 / Pc - 1 / Pa, and Amount 1 = Pb - Pc (see SqrtPriceMath) //
        //                                                                                           //
        // LTV = (Pc - Pa).Pb / [(Pc - Pa).Pb + (Pb - Pc).Pc]                                        //
        //                                                                                           //
        //                                                                                           //
        **********************************************************************************************/

        // @dev - does not revert for
        // edgeSqrtRatioX96_B <= (2^32) * FixedPoint.Q96
        // and 0 < edgeSqrtRatioX96_A <= currentSqrtRatioX96 <= edgeSqrtRatioX96_B
        // given edgeSqrtRatioX96_B ^ 2 <= 2^256
        uint256 calc1 = uint256(edgeSqrtRatioX96_B) * uint256(currentSqrtRatioX96 - edgeSqrtRatioX96_A);
        uint256 calc2 = uint256(currentSqrtRatioX96) * uint256(edgeSqrtRatioX96_B - currentSqrtRatioX96);

        return Math.mulDiv(FixedPoint.PERCENTAGE_FACTOR, calc1, calc1 + calc2);
    }

    /**
     * @notice Returns the maximum debt amount (in dex units) given liquidity and limit prices
     * @param edgeSqrtRatioX96_A low market price edge
     * @param edgeSqrtRatioX96_B high market price edge
     * @param liquidity current market liquidity
     * @return maxDebt The maximum debt amount
     */
    function computeMaxDebt(
        uint160 edgeSqrtRatioX96_A,
        uint160 edgeSqrtRatioX96_B,
        uint160 liquidity
    ) internal pure returns (uint256) {
        return SqrtPriceMath.getAmount0Delta(edgeSqrtRatioX96_B, edgeSqrtRatioX96_A, liquidity, Math.Rounding.Floor);
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

//SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.30;

import {TokenPrices, AssetType} from "../../../interfaces/ILiquidExchangeModel.sol";
import {LexConfig, LexState, MintParams, RedeemParams, SwapParams, MarketId, MarketParams, LexParams} from "../interfaces/ILatentSwapLEX.sol";
import {Math} from "@openzeppelin/utils/math/Math.sol";
import {LSErrors} from "./LSErrors.sol";
import {LatentMath} from "./LatentMath.sol";
import {FixedPoint} from "./FixedPoint.sol";
import {DebtMath} from "./DebtMath.sol";
import {UtilsLib} from "../../../libraries/Utils.sol";
import {PercentageMath} from "@aave/libraries/math/PercentageMath.sol";
import {SaturatingMath} from "./SaturatingMath.sol";
import {IPriceOracle} from "../../../interfaces/IPriceOracle.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {SafeCast} from "@openzeppelin/utils/math/SafeCast.sol";
import {SafeMetadata} from "../../../libraries/SafeMetadata.sol";
import {Strings} from "@openzeppelin/utils/Strings.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";

/**
 * @title Latent Swap Logic
 * @author Covenant Labs
 **/

library LatentSwapLogic {
    using SafeCast for uint256;
    using SafeCast for bool;
    using Math for uint256;
    using SaturatingMath for uint256;
    using PercentageMath for uint256;
    using SafeMetadata for IERC20;

    uint8 constant MAX_MINT_FACTOR_CAP = 1; // x1 max liquidity mint as a % of total market liquidity (measured through ETWAP). ie, limits amount that can be minted in ETWAP_MIN_HALF_LIFE minutes
    uint8 constant MAX_REDEEM_FACTOR_CAP = 2; // 1/4 max liquidity burn as a % of total market liquidity (measured through ETWAP). ie, limits amount that can be burned in ETWAP_MIN_HALF_LIFE minutes
    uint256 constant MAX_SYNTH_MINT_CAP = uint256(1) << 242; // do not allow minting more than 2^242 in aTokens and zTokens.  This allows for further market appreciation in value (and protects from percentageMul overflows)
    uint16 constant ETWAP_MIN_HALF_LIFE = 30 minutes; // Minimum half life of ETWAP calculations (ETWAP can have a longer halflife if market has infrequent updates)
    uint96 constant LN2 = 693147180559945331; // ln(2) in WADs.  Used for half-life calculations
    uint32 constant MIN_LIQRATIOX96 = 1e9; // Minimum Price * LiqRatio for market.  Prices under 1 wei (if in WADs) will revert.
    uint40 constant NEG_WORKOUT_LN_RATE = 116323331638; // -1% daily workout rate (when undercollateralized or above MAX_LIMIT_LTV). Expressed as lnRate per second in WADs.  Workout rate is negative, but here expressed as positive.
    uint8 constant DEBT = 0; // used for indexing into supplyAmounts and dexAmounts
    uint8 constant LVRG = 1; // used for indexing into supplyAmounts and dexAmounts

    // Market state - for cache + state calculated values
    struct LexFullState {
        LexState lexState;
        LexConfig lexConfig;
        uint256 baseTokenSupply;
        uint256[2] supplyAmounts;
        uint256[2] dexAmounts;
        uint256[3] dexAmountsScaled;
        uint256[3] synthAmountsScaled;
        uint256 liquidityRatioX96;
        uint160 liquidity;
        uint96 accruedProtocolFee;
        bool underCollateralized;
    }

    // Market init info struct
    struct MarketInitInfo {
        string baseName;
        string baseSymbol;
        string quoteSymbol;
        string aTokenName;
        string aTokenSymbol;
        string zTokenName;
        string zTokenSymbol;
        uint8 synthDecimals;
        uint8 quoteDecimals;
        uint8 noCapLimit;
        string levStr;
        string durStr;
    }

    ///////////////////////////////////////////////////////////////////////////////
    // External library functions
    ///////////////////////////////////////////////////////////////////////////////

    function mintLogic(
        MintParams calldata mintParams,
        LexParams memory lexParams,
        LexConfig storage lexConfig,
        LexState storage lexState,
        uint256 baseTokenSupply,
        bool isPreview
    )
        external
        view
        returns (
            LexFullState memory currentState,
            TokenPrices memory tokenPrices,
            uint256 aTokenAmountOut,
            uint256 zTokenAmountOut
        )
    {
        ///////////////////////////////
        // Calculate market state (storage read)
        currentState = _calculateMarketState(
            mintParams.marketParams,
            lexParams,
            lexConfig,
            lexState,
            baseTokenSupply,
            isPreview
        );

        ///////////////////////////////
        // Calculate mint
        (aTokenAmountOut, zTokenAmountOut) = _calcMint(lexParams, currentState, mintParams.baseAmountIn);

        /////////////////////////////////////
        // Calculate token prices (post action)
        tokenPrices = _calculateTokenPrices(lexParams, currentState);
    }

    function redeemLogic(
        RedeemParams calldata redeemParams,
        LexParams memory lexParams,
        LexConfig storage lexConfig,
        LexState storage lexState,
        uint256 baseTokenSupply,
        bool isPreview
    ) external view returns (LexFullState memory currentState, TokenPrices memory tokenPrices, uint256 amountOut) {
        ///////////////////////////////
        // Calculate market state (storage read)
        currentState = _calculateMarketState(
            redeemParams.marketParams,
            lexParams,
            lexConfig,
            lexState,
            baseTokenSupply,
            isPreview
        );

        ///////////////////////////////
        // Calculate redeem
        (amountOut, currentState.lexState.lastSqrtPriceX96) = _calcRedeem(
            lexParams,
            currentState,
            redeemParams.aTokenAmountIn,
            redeemParams.zTokenAmountIn
        );

        /////////////////////////////////////
        // Calculate token prices (post action)
        // @dev - prices are miscalculated if a market is fully redeemed, and are stated pre-action instead.
        tokenPrices = _calculateTokenPrices(lexParams, currentState);
    }

    function swapLogic(
        SwapParams calldata swapParams,
        LexParams memory lexParams,
        LexConfig storage lexConfig,
        LexState storage lexState,
        uint256 baseTokenSupply,
        bool isPreview
    )
        external
        view
        returns (LexFullState memory currentState, TokenPrices memory tokenPrices, uint256 amountCalculated)
    {
        ///////////////////////////////
        // Calculate market state (storage read)
        currentState = _calculateMarketState(
            swapParams.marketParams,
            lexParams,
            lexConfig,
            lexState,
            baseTokenSupply,
            isPreview
        ); ///////////////////////////////

        // Calculate swap
        (amountCalculated, currentState.lexState.lastSqrtPriceX96) = _calcSwap(
            lexParams,
            currentState,
            swapParams.amountSpecified,
            swapParams.assetIn,
            swapParams.assetOut,
            swapParams.isExactIn
        );

        /////////////////////////////////////
        // Calculate token prices (post action)
        // @dev - prices are miscalculated if a market is fully redeemed, and are stated pre-action instead.
        tokenPrices = _calculateTokenPrices(lexParams, currentState);
    }

    // Retrieve baseToken price
    // @dev - baseTokenPrice is the value of 10^18 baseTokens, in quoteTokens, irrespective of actual # of decimal precision the baseToken has
    function readBasePriceAndCalculateLiqRatio(
        MarketParams calldata marketParams,
        uint256 liquidityConcentrationX96,
        int8 scaleDecimals,
        bool isPreview
    ) external view returns (uint256 price, uint256 liqRatioX96) {
        return _readBasePriceAndCalculateLiqRatio(marketParams, liquidityConcentrationX96, scaleDecimals, isPreview);
    }

    function calculateMarketState(
        MarketParams calldata marketParams,
        LexParams memory lexParams,
        LexConfig storage lexConfig,
        LexState storage lexState,
        uint256 baseTokenSupply,
        bool isPreview
    ) external view returns (LexFullState memory marketState) {
        return _calculateMarketState(marketParams, lexParams, lexConfig, lexState, baseTokenSupply, isPreview);
    }

    function calcRatio(
        LexParams memory lexParams,
        LexFullState memory marketState,
        AssetType base,
        AssetType quote
    ) external pure returns (uint256 price) {
        return _calcRatio(lexParams, marketState, base, quote);
    }

    function getDebtPriceDiscount(
        uint160 edgeSqrtPriceX96_A,
        uint160 edgeSqrtPriceX96_B,
        uint160 currentSqrtPriceX96,
        uint256 target_dXdL_X96
    ) external pure returns (uint256 currentPriceDiscount) {
        return _getDebtPriceDiscount(edgeSqrtPriceX96_A, edgeSqrtPriceX96_B, currentSqrtPriceX96, target_dXdL_X96);
    }

    // @dev - externalizing computations used during LatentSwapLEX contract creation
    function computeMaxLTVandTargetdXdL(
        uint160 edgeSqrtPriceX96_A,
        uint160 edgeSqrtPriceX96_B,
        uint160 limMaxSqrtPriceX96
    ) external pure returns (uint256 maxLTV, uint256 target_dXdL_X96) {
        maxLTV = LatentMath.computeLTV(edgeSqrtPriceX96_A, edgeSqrtPriceX96_B, limMaxSqrtPriceX96);
        target_dXdL_X96 = LatentMath.targetXvsL(edgeSqrtPriceX96_A, edgeSqrtPriceX96_B);
    }

    // @dev - externalizing computations used during market initialization
    function getInitMarketInfo(
        MarketParams calldata marketParams,
        uint256 debtDuration,
        uint160 edgeSqrtPriceX96_A,
        uint160 edgeSqrtPriceX96_B,
        uint8 baseTokenNoCapLimit,
        uint8 quoteDecimals,
        string memory quoteSymbol
    ) external view returns (MarketInitInfo memory info) {
        bool success;

        // Get base asset name and symbol (cannot be overriden)
        (success, info.baseName) = IERC20(marketParams.baseToken).tryGetName();
        if (!success) revert LSErrors.E_LEX_BaseAssetNotERC20();
        (success, info.baseSymbol) = IERC20(marketParams.baseToken).tryGetSymbol();
        if (!success) revert LSErrors.E_LEX_BaseAssetNotERC20();

        // Get quote symbol with overrides
        info.quoteSymbol = quoteSymbol; // read
        if (bytes(info.quoteSymbol).length == 0) revert LSErrors.E_LEX_QuoteAssetHasNoSymbol();

        // Get synthDecimals decimals (== quote decimals with overrides)
        info.synthDecimals = quoteDecimals;
        if (info.synthDecimals == 0) info.synthDecimals = 18;

        // Get actual quote decimals (as would be returned by IPriceOracle)
        (success, info.quoteDecimals) = IERC20(marketParams.quoteToken).tryGetDecimals();
        if (!success) info.quoteDecimals = 18;

        // Get base Token noCap decimals
        info.noCapLimit = baseTokenNoCapLimit;
        if (info.noCapLimit == 0) {
            // default threshold above which mint / redeem cap are applied is equivalent to ~1 baseToken
            (bool success2, uint8 baseDecimals) = IERC20(marketParams.baseToken).tryGetDecimals();
            if (!success2)
                info.noCapLimit = uint8(60); // 18 Decimals -> 60 noCapLimit
            else info.noCapLimit = (FixedPointMathLib.log2(10 ** baseDecimals) + 1).toUint8();
        }

        // Calculate target leverage
        // leverage factor = 1/(1-targetLTV).  e.g., 50% -> 2, 80% -> 5, 90% -> 10, 95% -> 20
        info.levStr = Strings.toString(
            (FixedPoint.PERCENTAGE_FACTOR + 1) /
                (FixedPoint.PERCENTAGE_FACTOR -
                    LatentMath.computeLTV(edgeSqrtPriceX96_A, edgeSqrtPriceX96_B, uint160(FixedPoint.Q96)))
        );

        // Calculate duration in months / years
        // e.g., 1M, 3M, 6M, 1Y, 2Y, 5Y
        // @dev - simplified approach rounds down.
        uint256 months = debtDuration / (30 days);
        if (months < 12) info.durStr = string.concat(Strings.toString(months), "M");
        else info.durStr = string.concat(Strings.toString(debtDuration / (365 days)), "Y");

        info.aTokenName = string.concat(info.baseName," x",info.levStr," Leverage Coin (",info.quoteSymbol,"/",info.durStr,")"); // prettier-ignore
        info.aTokenSymbol = string.concat(info.baseSymbol, "x", info.levStr, ".", info.quoteSymbol);
        info.zTokenName = string.concat(info.quoteSymbol," Yield Coin - backed by ",info.baseSymbol," (x",info.levStr,"/",info.durStr,")"); // prettier-ignore
        info.zTokenSymbol = string.concat(info.quoteSymbol, ".b.", info.baseSymbol);
    }

    ///////////////////////////////////////////////////////////////////////////////
    // Internal functions
    ///////////////////////////////////////////////////////////////////////////////

    function _calcMint(
        LexParams memory lexParams,
        LexFullState memory marketState,
        uint256 baseTokenAmountIn
    ) internal pure returns (uint256 aTokenAmountOut, uint256 zTokenAmountOut) {
        ///////////////////////////////
        // Validate inputs

        // Check if undercollateralized (reverts if so)
        _checkUnderCollateralized(marketState);

        // Check LTV before mint (reverts if LTV above MAX_LIMIT_LTV)
        // @dev - minting aTokens when above MAX_LIMIT_LTV is blocked to avoid excessive aToken dilution
        // in high LTV or undercollateralized markets.
        _checkLTV(marketState.lexState.lastSqrtPriceX96, lexParams.limMaxSqrtPriceX96);

        // Check if mint amount is too large given market size
        _checkMintCap(
            marketState.baseTokenSupply,
            marketState.lexState.lastETWAPBaseSupply,
            baseTokenAmountIn,
            marketState.lexConfig.noCapLimit
        );

        ///////////////////////////////
        // Calculate mint
        // Calculate liquidity coming in (round down)
        uint160 liquidityIn = _synthToDex(marketState, baseTokenAmountIn, AssetType.BASE, Math.Rounding.Floor)
            .toUint160();
        // Calculate dex amounts that should be minted

        (uint256 zDexToMint, uint256 aDexToMint) = LatentMath.computeMint(
            marketState.lexState.lastSqrtPriceX96,
            lexParams.edgeSqrtPriceX96_A,
            lexParams.edgeSqrtPriceX96_B,
            liquidityIn
        );

        // Calculate actual zTokens / aTokens to mint (round down).
        // @dev - Use debt ratio for leverage token if this is the first liquidity in
        // (ratio not important for leverage token, so keep in targetLTV range)
        zTokenAmountOut = _dexToSynth(marketState, zDexToMint, AssetType.DEBT, Math.Rounding.Floor);
        aTokenAmountOut = _dexToSynth(marketState, aDexToMint, AssetType.LEVERAGE, Math.Rounding.Floor);

        ///////////////////////////////
        // Validate outputs

        // ensure we are not miniting more than MAX_SYNTH_MINT_CAP for zToken and aToken
        // Block minting while still allowing market value to appreciate.
        _checkSynthMintCap(marketState.supplyAmounts[DEBT], zTokenAmountOut);
        _checkSynthMintCap(marketState.supplyAmounts[LVRG], aTokenAmountOut);

        ///////////////////////////////
        // Charge fee by reducing out amount
        if (lexParams.swapFee > 0) {
            zTokenAmountOut = zTokenAmountOut.percentMul(FixedPoint.PERCENTAGE_FACTOR - lexParams.swapFee);
            aTokenAmountOut = aTokenAmountOut.percentMul(FixedPoint.PERCENTAGE_FACTOR - lexParams.swapFee);
        }
    }

    function _calcRedeem(
        LexParams memory lexParams,
        LexFullState memory marketState,
        uint256 aTokenAmountIn,
        uint256 zTokenAmountIn
    ) internal pure returns (uint256 amountOut, uint160 nextSqrtPriceX96) {
        // store for output validation
        uint160 beforeSqrtPriceX96 = marketState.lexState.lastSqrtPriceX96;

        ///////////////////////////////
        // Validate inputs
        if (aTokenAmountIn > marketState.supplyAmounts[LVRG]) revert LSErrors.E_LEX_InsufficientTokens();
        if (zTokenAmountIn > marketState.supplyAmounts[DEBT]) revert LSErrors.E_LEX_InsufficientTokens();

        // Validate liquidity
        // @dev - testing baseTokenSupply, because logic below allows for removal of baseTokenSupply
        // even if the market's calculated liquidity = 0.  In these situations, we do not go through
        // the latentSwap invariant, but instead assume all baseTokens belong to zToken holders (if any),
        // or otherwise to aToken holders, and redeem actionas are done proportioanl to holdings.
        if (marketState.baseTokenSupply == 0) revert LSErrors.E_LEX_ZeroLiquidity();

        ///////////////////////////////
        // Calculate redeem

        // Check for a full redeem
        if (aTokenAmountIn == marketState.supplyAmounts[LVRG] && zTokenAmountIn == marketState.supplyAmounts[DEBT]) {
            // @dev - when full redeem, no swap fee nor redeemCap checks, set to target price
            // @dev - we acknowledge that there is a footgun risk or someone preemptin a full redeem to capture fees
            // ie, redeem fees would not be captured by the attacker.  This can be mitigated by redeeming in two steps if
            // the last user feels the fees are worth it...
            // @dev - it is possible to redeem dust baseToken amounts even when liquidity == 0
            amountOut = marketState.baseTokenSupply;
            nextSqrtPriceX96 = uint160(FixedPoint.Q96);
        } else {
            if (marketState.underCollateralized || marketState.liquidity == 0) {
                if (marketState.supplyAmounts[DEBT] > 0) {
                    // if market is undercollateralized, only allow zToken -> base redeems, and make these proportional
                    // @dev - it is possible to redeem undercollateralized markets even if baseSupply > 0 but liquidity == 0
                    if (aTokenAmountIn > 0) revert LSErrors.E_LEX_ActionNotAllowedUnderCollateralized();

                    // Proportional redeem
                    // @dev - we do not go through the _synthToDex -> LatentMath -> _dexToSynth pathway
                    // to calculate amounts given we are at the market extreme where debt tokens are the full owners of base tokens
                    // and amounts can be just calculated proportional to market amounts
                    // @notice - amount out is a floor, but we can do a full redeem if zTokenAmountIn == marketState.supplyAmounts[DEBT]
                    amountOut = marketState.baseTokenSupply.mulDiv(zTokenAmountIn, marketState.supplyAmounts[DEBT]);

                    // Calc next price across the following three states:
                    // 1 - zTokens still in market, so remains undercollateralized and nextSqrtPrice = edgeSqrtPriceX96_B
                    // 1 - No baseTokens or zTokens left, but aTokens still in market -> nextSqrtPrice = edgeSqrtPriceX96_A
                    // 2 - fully empy market -> nextSqrtPrice = 1
                    nextSqrtPriceX96 = (amountOut < marketState.baseTokenSupply)
                        ? lexParams.edgeSqrtPriceX96_B
                        : (marketState.supplyAmounts[LVRG] > 0)
                            ? lexParams.edgeSqrtPriceX96_A
                            : uint160(FixedPoint.Q96);
                } else {
                    ////////////////////////////////////
                    // Allow proprional redeeming of baseTokens with leverage tokens, given no zTokens in the market
                    amountOut = marketState.baseTokenSupply.mulDiv(aTokenAmountIn, marketState.supplyAmounts[LVRG]);
                    nextSqrtPriceX96 = (amountOut < marketState.baseTokenSupply)
                        ? lexParams.edgeSqrtPriceX96_A
                        : uint160(FixedPoint.Q96);
                }
                // No redeem fees charged in undercollateralized or  zero liquiditymarket

                ///////////////////////////////////////
                // Validate output
                // Check if redeem amount is too large given market size, even when undercollateralized
                _checkRedeemCap(
                    marketState.baseTokenSupply,
                    marketState.lexState.lastETWAPBaseSupply,
                    amountOut,
                    marketState.lexConfig.noCapLimit
                );
            } else {
                uint256 zTokenDexIn = _synthToDex(marketState, zTokenAmountIn, AssetType.DEBT, Math.Rounding.Floor);
                uint256 aTokenDexIn = _synthToDex(marketState, aTokenAmountIn, AssetType.LEVERAGE, Math.Rounding.Floor);

                // Calculate liquidity out given tokens in
                uint160 liquidityOut;
                (liquidityOut, nextSqrtPriceX96) = LatentMath.computeRedeem(
                    marketState.liquidity,
                    marketState.lexState.lastSqrtPriceX96,
                    lexParams.edgeSqrtPriceX96_A,
                    lexParams.edgeSqrtPriceX96_B,
                    zTokenDexIn,
                    aTokenDexIn
                );

                // Calculate baseToken amount out given liquidity out
                // Round down amount out
                amountOut = _dexToSynth(marketState, liquidityOut, AssetType.BASE, Math.Rounding.Floor);

                if (amountOut >= marketState.baseTokenSupply || liquidityOut >= marketState.liquidity) {
                    // @dev - this is close to a full redeem, but there is some aToken or zToken dust left in the market
                    // @dev - when full redeem, no swap fee nor redeemCap checks, set to target price
                    // @dev - we acknowledge that there is a footgun risk or someone preemptin a full redeem to capture fees
                    // ie, redeem fees would not be captured by the attacker.  This can be mitigated by redeeming in two steps if
                    // the last user feels the fees are worth it...
                    // @dev - it is possible to redeem dust baseToken amounts even when liquidity == 0
                    // @dev - some dust aTokens or zTokens might be left in the market (valueless).
                    // @dev - We ackownolded that it could be argued that fees should not be skipped in this case.
                    // If we charged fees, a majority holder could redeeming here in various steps to avoid paying dust holders
                    // We acknowledge this fact by just not charging fees in this scenario when only dust holders are left.
                    amountOut = marketState.baseTokenSupply;
                    nextSqrtPriceX96 = uint160(FixedPoint.Q96);
                } else {
                    ///////////////////////////////////////
                    // Validate output
                    // Check if redeem amount is too large given market size
                    _checkRedeemCap(
                        marketState.baseTokenSupply,
                        marketState.lexState.lastETWAPBaseSupply,
                        amountOut,
                        marketState.lexConfig.noCapLimit
                    );

                    ////////////////////////////////////////
                    // Charge fee by reducing out amount (after checkRedeemCap)
                    if (lexParams.swapFee > 0)
                        amountOut = amountOut.percentMul(FixedPoint.PERCENTAGE_FACTOR - lexParams.swapFee);
                }
            }
        }

        // Additional validate outputs

        // Allow redeems that lower LTV (lower DEX price) or keep as is, and otherwise
        // check whether action pushes LTV past High limit (reverts if so)
        if (nextSqrtPriceX96 > beforeSqrtPriceX96) _checkLTV(nextSqrtPriceX96, lexParams.limHighSqrtPriceX96);

        // @dev - allow redeeming 0 base tokens (as a way to remove dust in markets if need be)
    }

    struct calcSwapVars {
        uint256 inputDexAmount;
        uint256 calcDexAmount;
        uint256 aDexTokenAmount;
        uint256 zDexTokenAmount;
        uint256 newDexTokenAmount;
        uint160 liquidityNew;
        AssetType fixedSynth;
    }

    function _calcSwap(
        LexParams memory lexParams,
        LexFullState memory marketState,
        uint256 swapAmount,
        AssetType assetIn,
        AssetType assetOut,
        bool isExactIn
    ) internal pure returns (uint256 calcAmount, uint160 nextSqrtPriceX96) {
        ///////////////////////////////
        // Validate inputs
        require(assetIn != assetOut);

        // check market liquidity
        // @dev - it can happen that the market has liquidity == 0, even if baseTokenSupply > 0 and aTokens or zTokens > 0.
        // However, the market is not operational (more liquidity needs to be added)
        if (marketState.liquidity == 0) revert LSErrors.E_LEX_ZeroLiquidity();

        // Undercollateralized checks
        // If undercollateralized, only allow zToken to baseToken swaps  (ie redeem zTokens only)
        if ((assetIn != AssetType.DEBT) || (assetOut != AssetType.BASE)) _checkUnderCollateralized(marketState);

        // Do not allow buying aTokens if market above MAX_LIMIT_LTV
        // @dev - markets above MAX_LIMIT_LTV return to a lower LTV through negative funding (zTokens paying aTokens), collateral price appreciation, or zToken -> base swaps
        if (assetOut == AssetType.LEVERAGE)
            _checkLTV(marketState.lexState.lastSqrtPriceX96, lexParams.limMaxSqrtPriceX96);

        ///////////////////////////////
        // Calculate swap

        calcSwapVars memory vars;

        if ((assetIn != AssetType.BASE) && (assetOut != AssetType.BASE)) {
            // case1: synth for synth swap
            vars.fixedSynth = isExactIn ? assetIn : assetOut;
            vars.inputDexAmount = _synthToDex(
                marketState,
                swapAmount,
                vars.fixedSynth,
                isExactIn ? Math.Rounding.Floor : Math.Rounding.Ceil
            );

            (vars.calcDexAmount, nextSqrtPriceX96) = LatentMath.computeSwap(
                marketState.liquidity,
                marketState.lexState.lastSqrtPriceX96,
                vars.fixedSynth,
                vars.inputDexAmount,
                isExactIn
            );

            // Convert internal DEX output to synth amounts
            calcAmount = _dexToSynth(
                marketState,
                vars.calcDexAmount,
                isExactIn ? assetOut : assetIn,
                isExactIn ? Math.Rounding.Floor : Math.Rounding.Ceil
            );

            ////////////////////////////////////////////////////////
            // Validate swap amounts
            // Do not allow ExactOut > 0 if InputAmount ends being 0
            if (!isExactIn && calcAmount == 0) revert LSErrors.E_LEX_InsufficientAmount();

            // ensure we are not miniting more than MAX_SYNTH_MINT_CAP for zToken and aToken
            // Block minting while still allowing market value to appreciate.
            _checkSynthMintCap(
                marketState.supplyAmounts[assetOut == AssetType.DEBT ? DEBT : LVRG],
                isExactIn ? calcAmount : swapAmount
            );
        } else if ((assetIn == AssetType.BASE) && isExactIn) {
            // case2: base token is being swapped with an exact amount in

            // @dev - round down for all conditions
            vars.liquidityNew = _synthToDex(marketState, swapAmount, AssetType.BASE, Math.Rounding.Floor).toUint160();

            // Mint tokens given liquidity in
            (vars.zDexTokenAmount, vars.aDexTokenAmount) = LatentMath.computeMint(
                marketState.lexState.lastSqrtPriceX96,
                lexParams.edgeSqrtPriceX96_A,
                lexParams.edgeSqrtPriceX96_B,
                vars.liquidityNew
            );

            // update liquidity
            marketState.liquidity += vars.liquidityNew;

            // swap assetOut
            (vars.newDexTokenAmount, nextSqrtPriceX96) = LatentMath.computeSwap(
                marketState.liquidity,
                marketState.lexState.lastSqrtPriceX96,
                assetOut == AssetType.DEBT ? AssetType.LEVERAGE : AssetType.DEBT,
                assetOut == AssetType.DEBT ? vars.aDexTokenAmount : vars.zDexTokenAmount,
                true
            );

            // add to swap output the original mint amount of assetOut
            vars.newDexTokenAmount += Math.ternary(
                assetOut == AssetType.LEVERAGE,
                vars.aDexTokenAmount,
                vars.zDexTokenAmount
            );

            calcAmount = _dexToSynth(marketState, vars.newDexTokenAmount, assetOut, Math.Rounding.Floor);

            ////////////////////////////////////////////////////////
            // Validate mintCap if baseToken is being swapped in
            _checkMintCap(
                marketState.baseTokenSupply,
                marketState.lexState.lastETWAPBaseSupply,
                swapAmount,
                marketState.lexConfig.noCapLimit
            );

            ////////////////////////////////////////////////////////
            // Validate synth mint cap
            _checkSynthMintCap(marketState.supplyAmounts[assetOut == AssetType.DEBT ? DEBT : LVRG], calcAmount);
        } else if ((assetOut == AssetType.BASE) && isExactIn) {
            // case3: base token is being swapped out, with exact synth amount in
            // @dev - equivalent to redeeming swapAmount of assetIn
            (calcAmount, nextSqrtPriceX96) = _calcRedeem(
                lexParams,
                marketState,
                (assetIn == AssetType.LEVERAGE) ? swapAmount : 0,
                (assetIn == AssetType.DEBT) ? swapAmount : 0
            );
        } else {
            revert LSErrors.E_LEX_OperationNotAllowed();
        }

        // Charge fee by reducing out amount (or increasing in amount)
        // @dev - Base out swaps were already charged when calling _calcRedeem
        if (lexParams.swapFee > 0 && (assetOut != AssetType.BASE))
            calcAmount = isExactIn
                ? calcAmount.percentMul(FixedPoint.PERCENTAGE_FACTOR - lexParams.swapFee)
                : calcAmount.percentDiv(FixedPoint.PERCENTAGE_FACTOR - lexParams.swapFee);

        ///////////////////////////////
        // Additional validate outputs

        // Do not allow actions that increase LTV (and end past High LTV limits)
        // e.g. aToken sales, or zToken buys if it takes market past High LTV limits
        // @dev - however, allow actions that make LTV better
        if (nextSqrtPriceX96 > marketState.lexState.lastSqrtPriceX96)
            _checkLTV(nextSqrtPriceX96, lexParams.limHighSqrtPriceX96);

        // Validate enough tokens in the market
        if (
            (assetIn != AssetType.BASE) &&
            ((isExactIn ? swapAmount : calcAmount) > marketState.supplyAmounts[(assetIn == AssetType.DEBT) ? 0 : 1])
        ) revert LSErrors.E_LEX_InsufficientTokens();

        // Validate nextSqrtPriceX96 did not pass lower bound (upper bound already checked above)
        // @dev - this can happen at the extreme of LTV -> 0% (no debt)
        if (nextSqrtPriceX96 < lexParams.edgeSqrtPriceX96_A) revert LSErrors.E_LEX_OperationNotAllowed();
    }

    // /// @dev - returns ratio (price) in WADs
    function _calcRatio(
        LexParams memory lexParams,
        LexFullState memory marketState,
        AssetType base,
        AssetType quote
    ) internal pure returns (uint256 price) {
        uint256 dexPrice;

        // Calculate price for converting one dex asset to another
        if (base != AssetType.BASE && quote != AssetType.BASE) {
            dexPrice = (base == AssetType.DEBT)
                ? Math.mulDiv(
                    marketState.lexState.lastSqrtPriceX96 * FixedPoint.WAD,
                    marketState.lexState.lastSqrtPriceX96,
                    FixedPoint.Q192
                )
                : ((FixedPoint.Q192 * FixedPoint.WAD) / marketState.lexState.lastSqrtPriceX96) /
                    marketState.lexState.lastSqrtPriceX96;
        } else {
            AssetType synthAsset = (base == AssetType.BASE) ? quote : base;
            // calculates BASE vs Synth price
            dexPrice = LatentMath.get_XvsL(
                marketState.lexState.lastSqrtPriceX96,
                lexParams.edgeSqrtPriceX96_A,
                lexParams.edgeSqrtPriceX96_B,
                synthAsset
            );
            if (quote == AssetType.BASE) dexPrice = (FixedPoint.WAD << FixedPoint.RESOLUTION) / dexPrice;
            else dexPrice = (FixedPoint.WAD * dexPrice) / FixedPoint.Q96;
        }

        // When calculating price between tokens, we must convert from dex prices to synth prices
        // ie dexToken1 = currentMarketPrice*dexToken0
        // => Token1amount * dexTokenRatio[LVRG] = currentMarketPrice * Token0amount * dexTokenRatio[DEBT]
        // => Token1amount / Token0amount = currentMarketPrice * dexTokenRatio[DEBT] / dexTokenRatio[LVRG]
        price = _dexToSynth(
            marketState,
            _synthToDex(marketState, dexPrice, base, Math.Rounding.Ceil),
            quote,
            Math.Rounding.Ceil
        );
    }

    // Local variables for _calculateMarketState to avoid stack too deep
    struct CalcMarketStateVars {
        uint256 elapsedTime;
        uint256 spotPriceDiscount;
        int256 spotLnRateBias;
        uint256 newDebtNotionalPrice;
        uint16 yieldFee;
        uint16 tvlFee;
        uint256 feeX96;
        uint256 yieldInBaseUnits;
        uint256 fee;
        uint256 invUpdateFactor;
        uint256 maxDebtValue;
    }

    // calculates current market state given market params, used by all other internal functions
    function _calculateMarketState(
        MarketParams calldata marketParams,
        LexParams memory lexParams,
        LexConfig storage lexConfig,
        LexState storage lexState,
        uint256 baseTokenSupply,
        bool isPreview
    ) internal view returns (LexFullState memory marketState) {
        CalcMarketStateVars memory vars;

        ////////////////////////////////////////////////////////////////////////////////
        // Read and cache variables
        marketState.lexState = lexState;
        marketState.lexConfig = lexConfig;
        marketState.baseTokenSupply = baseTokenSupply;

        ////////////////////////////////////////////////////////////////////////////////
        // Read external values
        // read synth token supplies (external call to trusted protocol)
        marketState.supplyAmounts[DEBT] = IERC20(marketState.lexConfig.zToken).totalSupply();
        marketState.supplyAmounts[LVRG] = IERC20(marketState.lexConfig.aToken).totalSupply();

        // get current baseToken market price from oracle and calculate liquidity ratio
        (marketState.lexState.lastBaseTokenPrice, marketState.liquidityRatioX96) = _readBasePriceAndCalculateLiqRatio(
            marketParams,
            lexParams.targetXvsL,
            marketState.lexConfig.scaleDecimals,
            isPreview
        );
        ////////////////////////////////////////////////////////////////////////////////
        // Execute time based accruals
        // update accrued yield + calc fees according to current blocktime + last market prices
        // update interest if the current block timestamp is greater than the last update timestamp
        if (block.timestamp > marketState.lexState.lastUpdateTimestamp) {
            vars.elapsedTime = block.timestamp - marketState.lexState.lastUpdateTimestamp;
            marketState.lexState.lastUpdateTimestamp = uint96(block.timestamp);

            // get debt price discount given last dex price. This is done on purpose, to accrue interest
            // based on past values (and time accrued with those values) vs current spot oracle and market values
            vars.spotPriceDiscount = _getDebtPriceDiscount(
                lexParams.edgeSqrtPriceX96_A,
                lexParams.edgeSqrtPriceX96_B,
                marketState.lexState.lastSqrtPriceX96,
                lexParams.targetXvsL
            );

            // if market price > _limMaxSqrtPriceX96 (ie, aToken selling is locked given high LTV)
            // this means market is undercollateralized or close to undercollateralized.  We thus activate a slowly increasing workout rate.
            // which creates ever higher positive interest rates as we get closer to the edge of the market
            // which also constitutes the underCollateralization event edge.
            // @dev - sqrtPrices have a max of uint104, so _squareUnsafe will not overflow.
            // @dev - WORKOUT_LN_RATE is negative
            vars.spotLnRateBias =
                int256(marketState.lexState.lastLnRateBias) -
                int256(
                    (marketState.lexState.lastSqrtPriceX96 <= lexParams.limMaxSqrtPriceX96)
                        ? 0
                        : Math.mulDiv(
                            uint256(lexParams.debtDuration) * NEG_WORKOUT_LN_RATE,
                            _squareUnsafe(marketState.lexState.lastSqrtPriceX96 - lexParams.limMaxSqrtPriceX96),
                            _squareUnsafe(lexParams.edgeSqrtPriceX96_B - lexParams.limMaxSqrtPriceX96)
                        )
                );

            // accrue interest (update cached debtNotionalPrice)
            // @dev limits elapsedTime for the update to a full duration interval.
            // @dev this limits interest accrual for inactive markets,
            // but ensures interest accrual does not overshoot given accrueInterest approximations
            // The maximum interest accrual below (in this update) is 1/price
            // (e.g., if price is 0.95 in a 12 month duration market, and market gets updated in 24 months... the update is still only 5.2%)
            vars.newDebtNotionalPrice = DebtMath.accrueInterest(
                marketState.lexState.lastDebtNotionalPrice,
                lexParams.debtDuration,
                vars.spotPriceDiscount,
                (vars.elapsedTime > lexParams.debtDuration) ? lexParams.debtDuration : vars.elapsedTime,
                vars.spotLnRateBias
            );

            // accrue fees
            // @dev - fee accruel linear instead of geometric to simplify math
            // large timesteps lead to underaccrual of protocol fees
            // @dev - do not accrue fees if LTV > MAX_LIMIT_LTV (high LTV or undercollateralized market)
            if (
                (marketState.lexConfig.protocolFee > 0) &&
                (marketState.lexState.lastSqrtPriceX96 <= lexParams.limMaxSqrtPriceX96)
            ) {
                // split out fees
                (vars.yieldFee, vars.tvlFee) = UtilsLib.decodeFee(marketState.lexConfig.protocolFee);

                vars.feeX96 = 0;
                if (vars.tvlFee > 0) {
                    // @dev - increase resolution of calculation to X96 to account for small fees, baseTokenSupplies or elapsedTime
                    vars.feeX96 = DebtMath.calculateLinearAccrual(
                        baseTokenSupply,
                        uint256(vars.tvlFee) << FixedPoint.RESOLUTION,
                        vars.elapsedTime
                    );
                }
                if ((vars.yieldFee > 0) && (vars.newDebtNotionalPrice > marketState.lexState.lastDebtNotionalPrice)) {
                    // calculate estimate of yield accrued in base units
                    // @dev - uses LTV instead of spot prices to lower gas cost
                    // this uses the avg market price instead of spot price for conversion,
                    // which leads to a higher yield estimation in low LTV environments vs high LTV environments
                    // this behavior is acceptable given gas savings.
                    vars.yieldInBaseUnits = marketState.baseTokenSupply.mulDiv(
                        (vars.newDebtNotionalPrice - marketState.lexState.lastDebtNotionalPrice) *
                            LatentMath.computeLTV(
                                lexParams.edgeSqrtPriceX96_A,
                                lexParams.edgeSqrtPriceX96_B,
                                marketState.lexState.lastSqrtPriceX96
                            ),
                        marketState.lexState.lastDebtNotionalPrice * FixedPoint.PERCENTAGE_FACTOR
                    );

                    vars.feeX96 += vars.yieldInBaseUnits.mulDiv(
                        uint256(vars.yieldFee) << FixedPoint.RESOLUTION,
                        FixedPoint.PERCENTAGE_FACTOR
                    );
                }

                vars.fee = vars.feeX96 / FixedPoint.Q96;

                // Probabilistically (best effort) add +1 fee depending on remainder for small fees (FIX from Pashov Audit)
                // @dev - this allows to probabilistically collect fees for low baseTokenSupplies, small fees, or small elapseTime
                // @dev -  We use the probabilistic approach to lower gas cost (avoid an SSTORE),
                // and only for fees < 100 given we are ok with a < 1% underaccrual of fees.
                if (vars.fee < 100) {
                    if (
                        (vars.feeX96 % FixedPoint.Q96) >
                        (uint256(
                            keccak256(
                                abi.encodePacked(
                                    uint32(block.prevrandao),
                                    uint32(block.timestamp),
                                    uint128(marketState.baseTokenSupply),
                                    uint64(marketState.lexState.lastSqrtPriceX96)
                                )
                            )
                        ) >> 160)
                    ) vars.fee += 1;
                } else if (vars.fee > type(uint96).max) vars.fee = type(uint96).max;
                if (vars.fee > (baseTokenSupply / 8)) vars.fee = baseTokenSupply / 8; // @dev set max update of 12.5% of baseTokenSupply

                unchecked {
                    marketState.accruedProtocolFee = uint96(vars.fee); //fits in uint96 given previous checks
                    marketState.baseTokenSupply -= vars.fee; // @dev - remove fee from baseTokenSupply for all calculations going fwd
                }
            }
            marketState.lexState.lastDebtNotionalPrice = vars.newDebtNotionalPrice;

            /////////////////////////////////////
            // Exponential TWAP
            // @dev - stores an exponential moving avg of market baseSupply

            // Calculate update factor with an approximate ETWAP_HALF_LIFE.
            vars.invUpdateFactor = DebtMath.calculateApproxExponentialUpdate(
                LN2,
                vars.elapsedTime,
                ETWAP_MIN_HALF_LIFE
            );

            marketState.lexState.lastETWAPBaseSupply =
                marketState.lexState.lastETWAPBaseSupply.mulDiv(FixedPoint.RAY, vars.invUpdateFactor) +
                marketState.baseTokenSupply.mulDiv(
                    FixedPoint.RAY - (FixedPoint.RAY * FixedPoint.RAY) / vars.invUpdateFactor,
                    FixedPoint.RAY
                );
        }

        //////////////////////////////////////////////////////////////////////////////////////
        // Calculate parameters for synth -> dex -> synth transforms (for BASE + DEBT only)
        // @dev - these scaled amounts seek to ensure liquidity < 2^152.
        // so, if liquidity is too big, I would make liquidityScaled ~ 2^152 = liquidity * X96 / divScaleFactorX96.
        // so, divScaleFactorX96 = liquidity * X96 / 2^152 = liquidityRatioX96 * BaseTokenSupply / 2^152;
        // @dev - saturates instead of reverting, meaning liquidity could still be > 2^152 even after applying this scaling.
        // This would only happen in markets where the oracle price * baseTokenSupply itself overflows, and thus an unlikely scenario.
        // if so, it will revert later when calculating liquidity.
        // @dev - if baseTokenSupply < X96, then use X96.  We assume all viable markets can price (without overflow a minimum of X96 base tokens)
        uint256 divScaleFactorX96 = marketState.liquidityRatioX96.saturatingMulShr(
            marketState.baseTokenSupply.max(FixedPoint.Q96),
            152
        );
        bool scaledLiquidity = divScaleFactorX96 > FixedPoint.Q96; // scaling only applies if divScaleFactorX96 > X96

        // if divScaleFactorX96 < X96, then liquidity already < 2^152.  liquidity = liquidityRatioX96 * BaseTokenSupply / X96
        // if divScaleFactorX96 > X96, then liquidityScaled = 2^152 = liquidityRatioX96 * BaseTokenSupply / divScaleFactorX96
        marketState.dexAmountsScaled[uint8(AssetType.BASE)] = marketState.liquidityRatioX96;
        marketState.synthAmountsScaled[uint8(AssetType.BASE)] = Math.ternary(
            scaledLiquidity,
            divScaleFactorX96,
            FixedPoint.Q96
        );

        // check that (notionalPrice * debtSupply * X96 / WAD / synthAmountScaled[Base]) does not overflow.
        // we have to use the synthAmountScaled[Base] across all assets to make the market consistent.
        // dex amount cannot be bigger than maxDebt * X96 / synthAmountScaled[Base] < notionalPrice * debtSupply * X96 / WAD / synthAmountScaled[Base]
        // thus, if maxDebt / notionalPrice < debtSupply / WAD, then market is undercollateralized.
        marketState.dexAmountsScaled[uint8(AssetType.DEBT)] = (scaledLiquidity &&
            marketState.lexState.lastDebtNotionalPrice < FixedPoint.Q160)
            ? (divScaleFactorX96 < FixedPoint.Q192)
                ? marketState.lexState.lastDebtNotionalPrice * FixedPoint.Q96
                : marketState.lexState.lastDebtNotionalPrice.mulDiv(FixedPoint.Q96, FixedPoint.WAD)
            : marketState.lexState.lastDebtNotionalPrice;

        marketState.synthAmountsScaled[uint8(AssetType.DEBT)] = scaledLiquidity
            ? (marketState.lexState.lastDebtNotionalPrice < FixedPoint.Q160)
                ? (divScaleFactorX96 < FixedPoint.Q192)
                    ? FixedPoint.WAD * divScaleFactorX96
                    : divScaleFactorX96
                : FixedPoint.WAD.mulDiv(divScaleFactorX96, FixedPoint.Q96)
            : FixedPoint.WAD;

        ////////////////////////////////////////////////////////////////////////////////
        // Calculate liquidity from baseTokenSupply
        marketState.liquidity = _synthToDex(
            marketState,
            marketState.baseTokenSupply,
            AssetType.BASE,
            Math.Rounding.Floor
        ).toUint160();

        ////////////////////////////////////////////////////////////////////////////////
        // calculate values if market has liquidity
        if (marketState.liquidity > 0) {
            // Calculate debt balanced value from zTokenSupply
            marketState.dexAmounts[DEBT] = _synthToDex(
                marketState,
                marketState.supplyAmounts[DEBT],
                AssetType.DEBT,
                Math.Rounding.Floor
            );

            // Check max value for debt, given availablie liquidity in market.
            vars.maxDebtValue = LatentMath.computeMaxDebt(
                lexParams.edgeSqrtPriceX96_A,
                lexParams.edgeSqrtPriceX96_B,
                marketState.liquidity
            );

            // if true, then system is undercollateralized (ie, debt notional value is above liquidity value)
            // if so, reduce debt value to be system liquidity value
            if (marketState.dexAmounts[DEBT] > vars.maxDebtValue) {
                marketState.lexState.lastSqrtPriceX96 = lexParams.edgeSqrtPriceX96_B;
                marketState.dexAmounts[DEBT] = vars.maxDebtValue;
                marketState.dexAmounts[LVRG] = 0;
                marketState.underCollateralized = true;
            } else {
                // calculate market price and aDexAmount given liquidity and zDexAmount
                // @dev - it could happen that marketState.dexAmounts[LVRG] == 0,
                // even if Liquidity > 0 and marketState.dexAmounts[DEBT] < maxDebtValue.
                (marketState.dexAmounts[LVRG], marketState.lexState.lastSqrtPriceX96) = LatentMath
                    .getMarketStateFromLiquidityAndDebt(
                        lexParams.edgeSqrtPriceX96_A,
                        lexParams.edgeSqrtPriceX96_B,
                        marketState.liquidity,
                        marketState.dexAmounts[DEBT]
                    );
            }
        } else {
            // If liquidity == 0, go through following scenarios to set price depending on whether
            // there is any debt or leverage tokens in the market, or whether it is a fully empty market.
            // @dev - any dust aTokens that might be left in the market is valueless.
            // @dev - any dust zTokens left might have some value (if baseTokenSupply > 0) but very small (under < 10-15 in quote tokens for most market setups)
            // but this undercollateralized state would not be resolved via workout accrual and might block the market,
            // so we consider them valueless as well.
            // Thus, we set the market price to uint160(FixedPoint.Q96) - uninitialized.
            marketState.dexAmounts[LVRG] = 0;
            marketState.dexAmounts[DEBT] = 0;
            marketState.lexState.lastSqrtPriceX96 = (marketState.supplyAmounts[LVRG] > 0 &&
                marketState.supplyAmounts[DEBT] == 0)
                ? lexParams.edgeSqrtPriceX96_A
                : uint160(FixedPoint.Q96);
        }

        //////////////////////////////////////////////////////////////////////////////////////
        // Calculate parameters for synth -> dex -> synth transforms (for LVRG only)
        // if no supply for leverage token, assume ratio == 1
        // if dexAmounts[LVRG] = 0 while supplyAmounts[LVRG] > 0, we will set dexAmountScaled[LVRG] = 1
        //  To ensure _synthToDex and _DexToSynth ratios work (and allow for LTV and cap checks)
        // This might happen when liquidity > 0 but very close to being undercollateralized (without triggering the flag),
        // or in markets where liquidity = 0 with leftover dust.
        marketState.dexAmountsScaled[uint8(AssetType.LEVERAGE)] = Math.ternary(
            marketState.supplyAmounts[LVRG] > 0,
            Math.max(marketState.dexAmounts[LVRG], 1),
            1
        );
        marketState.synthAmountsScaled[uint8(AssetType.LEVERAGE)] = Math.ternary(
            marketState.supplyAmounts[LVRG] > 0,
            marketState.supplyAmounts[LVRG],
            1
        );

        return marketState;
    }

    // @dev - returns price discount in WADs
    // ie. currentNotionalPrice = WAD indicates a price of 1 (ie, debt is trading at par)
    function _getDebtPriceDiscount(
        uint160 edgeSqrtPriceX96_A,
        uint160 edgeSqrtPriceX96_B,
        uint160 currentSqrtPriceX96,
        uint256 target_dXdL_X96
    ) internal pure returns (uint256 currentPriceDiscount) {
        // @dev - in undercollateralized case, will correctly price discount such that
        // the interest rate corresponds to a market that is 100% debt and 0% leverage,
        // ie, the max interest rate of the market
        uint256 current_dXdL_X96 = LatentMath.get_XvsL(
            currentSqrtPriceX96,
            edgeSqrtPriceX96_A,
            edgeSqrtPriceX96_B,
            AssetType.DEBT
        );

        return FixedPoint.WAD.mulDiv(target_dXdL_X96, current_dXdL_X96);
    }

    // checks market LTV and reverts if beyond bounds
    // @dev - perform after action that should be prohibited if LTVs are off limits
    function _checkLTV(uint160 nextSqrtPriceX96, uint160 sqrtPriceLimitX96) internal pure {
        if (nextSqrtPriceX96 > sqrtPriceLimitX96) revert LSErrors.E_LEX_ActionNotAllowedGivenLTVlimit();
    }

    // checks and reverts if in undercollateralized state
    function _checkUnderCollateralized(LexFullState memory marketState) internal pure {
        if (marketState.underCollateralized) revert LSErrors.E_LEX_ActionNotAllowedUnderCollateralized();
    }

    function _checkMintCap(
        uint256 marketBaseTokenSupply,
        uint256 eTWAPBaseTokenSupply,
        uint256 mintAmount,
        uint8 noCapLimit
    ) internal pure {
        // if marketBaseTokenSupply <= AMOUNT_NO_CAP, then we can mint upto 2^96.
        // ie, for small markets we can mint up 2^96 in one go (we assume markets can price correctly this amount of base tokens)
        // But for bigger markets, mintAmount <= marketBaseTokenSupply << (MAX_MINT_FACTOR_CAP - 1)
        // and marketBaseTokenSupply + mintAmount <= eTWAPBaseTokenSupply << MAX_MINT_FACTOR_CAP
        // For MaxMintFactorCap = 1, this means mintAmount <= marketBaseTokenSupply (ie, we can double the market size in one call)
        // as long as marketBaseTokenSupply + mintAmount <= eTWAPBaseTokenSupply << MAX_MINT_FACTOR_CAP.
        // in practice, a user can mint upto  and are approx. ~MAX_MINT_FACTOR_CAP/2 can be minted every 1hr
        // (this is based on ETWAP_MIN_HALF_LIFE and MAX_REDEEM_FACTOR_CAP values)
        unchecked {
            if (marketBaseTokenSupply <= (1 << noCapLimit)) {
                if ((mintAmount > FixedPoint.Q96) && (mintAmount > marketBaseTokenSupply)) {
                    revert LSErrors.E_LEX_MintCapExceeded();
                }
            } else if (
                (mintAmount > marketBaseTokenSupply) ||
                ((type(uint256).max - mintAmount) < marketBaseTokenSupply) ||
                ((eTWAPBaseTokenSupply < (type(uint256).max >> MAX_MINT_FACTOR_CAP)) &&
                    ((marketBaseTokenSupply + mintAmount) > (eTWAPBaseTokenSupply << MAX_MINT_FACTOR_CAP)))
            ) {
                revert LSErrors.E_LEX_MintCapExceeded();
            }
        }
    }

    function _checkRedeemCap(
        uint256 marketBaseTokenSupply,
        uint256 eTWAPBaseTokenSupply,
        uint256 redeemAmount,
        uint8 noCapLimit
    ) internal pure {
        // OK to redeem any amount if marketBaseTokenSupply < 10^noCapLimit
        // ie, for small markets there is no limit on how much can be redeemed.
        // But for bigger markets, approx. ~MAX_REDEEM_FACTOR_CAP/2 can be redeemed every 1hr
        // (this is based on ETWAP_MIN_HALF_LIFE and MAX_REDEEM_FACTOR_CAP values)
        unchecked {
            if (
                (marketBaseTokenSupply > (1 << noCapLimit)) &&
                (marketBaseTokenSupply > redeemAmount) &&
                ((marketBaseTokenSupply - redeemAmount) <
                    (eTWAPBaseTokenSupply - (eTWAPBaseTokenSupply >> MAX_REDEEM_FACTOR_CAP)))
            ) revert LSErrors.E_LEX_RedeemCapExceeded();
        }
    }

    function _checkSynthMintCap(uint256 synthSupplyAmount, uint256 mintAmount) internal pure {
        // ensure we are not miniting more than MAX_SYNTH_MINT_CAP for zToken and aToken
        // Block minting while still allowing market value to appreciate.
        if ((mintAmount > MAX_SYNTH_MINT_CAP) || ((MAX_SYNTH_MINT_CAP - mintAmount) < synthSupplyAmount))
            revert LSErrors.E_LEX_MarketSizeLimitExceeded();
    }

    // ------------------ Synth to Dex conversions ---------

    function _synthToDex(
        LexFullState memory marketState,
        uint256 synthAmount,
        AssetType assetType,
        Math.Rounding rounding
    ) internal pure returns (uint256 dexAmount) {
        return
            synthAmount.mulDiv(
                marketState.dexAmountsScaled[uint8(assetType)],
                marketState.synthAmountsScaled[uint8(assetType)],
                rounding
            );
    }

    function _dexToSynth(
        LexFullState memory marketState,
        uint256 dexAmount,
        AssetType assetType,
        Math.Rounding rounding
    ) internal pure returns (uint256 synthAmount) {
        // @dev - using saturatingMulDiv avoids overflows that are then captured downstream.
        // Specifically:
        // 1) in ExactIn cases with synth outputs (e.g., mint, swap base -> synth, swap synth -> synth),
        // if we saturate the amount of synth output, this is expected to be caught in the _checkSynthMintCap
        // check.
        // 2) in the ExactIn synth -> base swap, this would not saturate given we don't control the baseTokens in existence.
        // 3) in the ExactOut case with synth inputs (e.g. swap synth -> synth), it would seem saturating the amount of synth coming in
        // might create an issue (given more synths would be expected to come in given the synths going out).  however , we sould
        // recall we are saturating to the max synth tokens in existence 2^256-1, and thus we are removing all of one type of synth and
        // making the market be 100% the other type of synth.  In this situation, the market is correct.  We are eitehr 100% equity
        // LatentSwap DEX will correctly price the leverage token.  Or we are 100% debt (and the transaction will revert given LTV limits).
        return
            dexAmount.saturatingMulDiv(
                marketState.synthAmountsScaled[uint8(assetType)],
                marketState.dexAmountsScaled[uint8(assetType)],
                rounding
            );
    }

    // Retrieve baseToken price
    // @dev - baseTokenPrice is the value of 10^18 baseTokens, in quoteTokens, irrespective of actual # of decimal precision the baseToken has
    function _readBasePriceAndCalculateLiqRatio(
        MarketParams calldata marketParams,
        uint256 targetXvsL,
        int8 scaleDecimals,
        bool isPreview
    ) internal view returns (uint256 price, uint256 liqRatioX96) {
        // targetXvsL is also the liquidity concentration of the market, and used here when calculating the liquidityRatio
        // scaleDecimals ensures that final 'value' is in synth decimals (and not quote decimals)
        uint256 scaledLiquidityConcentrationX96 = (scaleDecimals > 0)
            ? FixedPoint.Q192 / (targetXvsL * (10 ** uint8(scaleDecimals)))
            : (FixedPoint.Q192 * (10 ** uint8(-scaleDecimals))) / targetXvsL;

        // liqRatioX96 represents the ratio transforming base token amounts to a concentrated value denominated liquidity
        // @dev - getQuote is such that it returns the # of quote tokens, given # of base tokens coming in (irrespective of actual decimal representation).
        liqRatioX96 = (isPreview)
            ? IPriceOracle(marketParams.curator).previewGetQuote(
                scaledLiquidityConcentrationX96,
                marketParams.baseToken,
                marketParams.quoteToken
            )
            : liqRatioX96 = IPriceOracle(marketParams.curator).getQuote(
            scaledLiquidityConcentrationX96,
            marketParams.baseToken,
            marketParams.quoteToken
        );

        if (liqRatioX96 < MIN_LIQRATIOX96) revert LSErrors.E_LEX_OraclePriceTooLowForMarket();

        // calculate price
        // @dev - price is the # of quote tokens given 10^18 base tokens ,
        // irrespective of actual # of decimal precision that baseToken or quoteToken has
        price = FixedPoint.WAD.mulDiv(liqRatioX96, scaledLiquidityConcentrationX96);
    }

    function _calculateTokenPrices(
        LexParams memory lexParams,
        LexFullState memory marketState
    ) internal pure returns (TokenPrices memory tokenPrices) {
        // @notice - all prices are # of quote tokens received for 10^18 of base, leverage, or debt tokens
        //(irrespective of actual decimal precision of each token type).
        tokenPrices.baseTokenPrice = marketState.lexState.lastBaseTokenPrice;

        // if market is undercollateralized and has leverage tokens, then leverage value is zero.
        // otherwise, calculate price of leverage token  given baseTokenPrice and leverage price in the Covenant market
        tokenPrices.aTokenPrice = (marketState.underCollateralized && marketState.dexAmounts[LVRG] > 0)
            ? 0
            : tokenPrices.baseTokenPrice.mulDiv(
                _calcRatio(lexParams, marketState, AssetType.LEVERAGE, AssetType.BASE),
                FixedPoint.WAD
            );

        // if market is undercollateralized and has debt, all base value is owned by debt.
        // otherwise, calculate price of debt token  given baseTokenPrice and debt price in the Covenant market
        tokenPrices.zTokenPrice = (marketState.underCollateralized && marketState.dexAmounts[DEBT] > 0)
            ? tokenPrices.baseTokenPrice.mulDiv(marketState.baseTokenSupply, marketState.supplyAmounts[DEBT])
            : tokenPrices.baseTokenPrice.mulDiv(
                _calcRatio(lexParams, marketState, AssetType.DEBT, AssetType.BASE),
                FixedPoint.WAD
            );
    }

    function _squareUnsafe(uint256 value) internal pure returns (uint256) {
        unchecked {
            return value * value;
        }
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.3.0) (utils/Strings.sol)

pragma solidity ^0.8.20;

import {Math} from "./math/Math.sol";
import {SafeCast} from "./math/SafeCast.sol";
import {SignedMath} from "./math/SignedMath.sol";

/**
 * @dev String operations.
 */
library Strings {
    using SafeCast for *;

    bytes16 private constant HEX_DIGITS = "0123456789abcdef";
    uint8 private constant ADDRESS_LENGTH = 20;
    uint256 private constant SPECIAL_CHARS_LOOKUP =
        (1 << 0x08) | // backspace
            (1 << 0x09) | // tab
            (1 << 0x0a) | // newline
            (1 << 0x0c) | // form feed
            (1 << 0x0d) | // carriage return
            (1 << 0x22) | // double quote
            (1 << 0x5c); // backslash

    /**
     * @dev The `value` string doesn't fit in the specified `length`.
     */
    error StringsInsufficientHexLength(uint256 value, uint256 length);

    /**
     * @dev The string being parsed contains characters that are not in scope of the given base.
     */
    error StringsInvalidChar();

    /**
     * @dev The string being parsed is not a properly formatted address.
     */
    error StringsInvalidAddressFormat();

    /**
     * @dev Converts a `uint256` to its ASCII `string` decimal representation.
     */
    function toString(uint256 value) internal pure returns (string memory) {
        unchecked {
            uint256 length = Math.log10(value) + 1;
            string memory buffer = new string(length);
            uint256 ptr;
            assembly ("memory-safe") {
                ptr := add(buffer, add(32, length))
            }
            while (true) {
                ptr--;
                assembly ("memory-safe") {
                    mstore8(ptr, byte(mod(value, 10), HEX_DIGITS))
                }
                value /= 10;
                if (value == 0) break;
            }
            return buffer;
        }
    }

    /**
     * @dev Converts a `int256` to its ASCII `string` decimal representation.
     */
    function toStringSigned(int256 value) internal pure returns (string memory) {
        return string.concat(value < 0 ? "-" : "", toString(SignedMath.abs(value)));
    }

    /**
     * @dev Converts a `uint256` to its ASCII `string` hexadecimal representation.
     */
    function toHexString(uint256 value) internal pure returns (string memory) {
        unchecked {
            return toHexString(value, Math.log256(value) + 1);
        }
    }

    /**
     * @dev Converts a `uint256` to its ASCII `string` hexadecimal representation with fixed length.
     */
    function toHexString(uint256 value, uint256 length) internal pure returns (string memory) {
        uint256 localValue = value;
        bytes memory buffer = new bytes(2 * length + 2);
        buffer[0] = "0";
        buffer[1] = "x";
        for (uint256 i = 2 * length + 1; i > 1; --i) {
            buffer[i] = HEX_DIGITS[localValue & 0xf];
            localValue >>= 4;
        }
        if (localValue != 0) {
            revert StringsInsufficientHexLength(value, length);
        }
        return string(buffer);
    }

    /**
     * @dev Converts an `address` with fixed length of 20 bytes to its not checksummed ASCII `string` hexadecimal
     * representation.
     */
    function toHexString(address addr) internal pure returns (string memory) {
        return toHexString(uint256(uint160(addr)), ADDRESS_LENGTH);
    }

    /**
     * @dev Converts an `address` with fixed length of 20 bytes to its checksummed ASCII `string` hexadecimal
     * representation, according to EIP-55.
     */
    function toChecksumHexString(address addr) internal pure returns (string memory) {
        bytes memory buffer = bytes(toHexString(addr));

        // hash the hex part of buffer (skip length + 2 bytes, length 40)
        uint256 hashValue;
        assembly ("memory-safe") {
            hashValue := shr(96, keccak256(add(buffer, 0x22), 40))
        }

        for (uint256 i = 41; i > 1; --i) {
            // possible values for buffer[i] are 48 (0) to 57 (9) and 97 (a) to 102 (f)
            if (hashValue & 0xf > 7 && uint8(buffer[i]) > 96) {
                // case shift by xoring with 0x20
                buffer[i] ^= 0x20;
            }
            hashValue >>= 4;
        }
        return string(buffer);
    }

    /**
     * @dev Returns true if the two strings are equal.
     */
    function equal(string memory a, string memory b) internal pure returns (bool) {
        return bytes(a).length == bytes(b).length && keccak256(bytes(a)) == keccak256(bytes(b));
    }

    /**
     * @dev Parse a decimal string and returns the value as a `uint256`.
     *
     * Requirements:
     * - The string must be formatted as `[0-9]*`
     * - The result must fit into an `uint256` type
     */
    function parseUint(string memory input) internal pure returns (uint256) {
        return parseUint(input, 0, bytes(input).length);
    }

    /**
     * @dev Variant of {parseUint-string} that parses a substring of `input` located between position `begin` (included) and
     * `end` (excluded).
     *
     * Requirements:
     * - The substring must be formatted as `[0-9]*`
     * - The result must fit into an `uint256` type
     */
    function parseUint(string memory input, uint256 begin, uint256 end) internal pure returns (uint256) {
        (bool success, uint256 value) = tryParseUint(input, begin, end);
        if (!success) revert StringsInvalidChar();
        return value;
    }

    /**
     * @dev Variant of {parseUint-string} that returns false if the parsing fails because of an invalid character.
     *
     * NOTE: This function will revert if the result does not fit in a `uint256`.
     */
    function tryParseUint(string memory input) internal pure returns (bool success, uint256 value) {
        return _tryParseUintUncheckedBounds(input, 0, bytes(input).length);
    }

    /**
     * @dev Variant of {parseUint-string-uint256-uint256} that returns false if the parsing fails because of an invalid
     * character.
     *
     * NOTE: This function will revert if the result does not fit in a `uint256`.
     */
    function tryParseUint(
        string memory input,
        uint256 begin,
        uint256 end
    ) internal pure returns (bool success, uint256 value) {
        if (end > bytes(input).length || begin > end) return (false, 0);
        return _tryParseUintUncheckedBounds(input, begin, end);
    }

    /**
     * @dev Implementation of {tryParseUint-string-uint256-uint256} that does not check bounds. Caller should make sure that
     * `begin <= end <= input.length`. Other inputs would result in undefined behavior.
     */
    function _tryParseUintUncheckedBounds(
        string memory input,
        uint256 begin,
        uint256 end
    ) private pure returns (bool success, uint256 value) {
        bytes memory buffer = bytes(input);

        uint256 result = 0;
        for (uint256 i = begin; i < end; ++i) {
            uint8 chr = _tryParseChr(bytes1(_unsafeReadBytesOffset(buffer, i)));
            if (chr > 9) return (false, 0);
            result *= 10;
            result += chr;
        }
        return (true, result);
    }

    /**
     * @dev Parse a decimal string and returns the value as a `int256`.
     *
     * Requirements:
     * - The string must be formatted as `[-+]?[0-9]*`
     * - The result must fit in an `int256` type.
     */
    function parseInt(string memory input) internal pure returns (int256) {
        return parseInt(input, 0, bytes(input).length);
    }

    /**
     * @dev Variant of {parseInt-string} that parses a substring of `input` located between position `begin` (included) and
     * `end` (excluded).
     *
     * Requirements:
     * - The substring must be formatted as `[-+]?[0-9]*`
     * - The result must fit in an `int256` type.
     */
    function parseInt(string memory input, uint256 begin, uint256 end) internal pure returns (int256) {
        (bool success, int256 value) = tryParseInt(input, begin, end);
        if (!success) revert StringsInvalidChar();
        return value;
    }

    /**
     * @dev Variant of {parseInt-string} that returns false if the parsing fails because of an invalid character or if
     * the result does not fit in a `int256`.
     *
     * NOTE: This function will revert if the absolute value of the result does not fit in a `uint256`.
     */
    function tryParseInt(string memory input) internal pure returns (bool success, int256 value) {
        return _tryParseIntUncheckedBounds(input, 0, bytes(input).length);
    }

    uint256 private constant ABS_MIN_INT256 = 2 ** 255;

    /**
     * @dev Variant of {parseInt-string-uint256-uint256} that returns false if the parsing fails because of an invalid
     * character or if the result does not fit in a `int256`.
     *
     * NOTE: This function will revert if the absolute value of the result does not fit in a `uint256`.
     */
    function tryParseInt(
        string memory input,
        uint256 begin,
        uint256 end
    ) internal pure returns (bool success, int256 value) {
        if (end > bytes(input).length || begin > end) return (false, 0);
        return _tryParseIntUncheckedBounds(input, begin, end);
    }

    /**
     * @dev Implementation of {tryParseInt-string-uint256-uint256} that does not check bounds. Caller should make sure that
     * `begin <= end <= input.length`. Other inputs would result in undefined behavior.
     */
    function _tryParseIntUncheckedBounds(
        string memory input,
        uint256 begin,
        uint256 end
    ) private pure returns (bool success, int256 value) {
        bytes memory buffer = bytes(input);

        // Check presence of a negative sign.
        bytes1 sign = begin == end ? bytes1(0) : bytes1(_unsafeReadBytesOffset(buffer, begin)); // don't do out-of-bound (possibly unsafe) read if sub-string is empty
        bool positiveSign = sign == bytes1("+");
        bool negativeSign = sign == bytes1("-");
        uint256 offset = (positiveSign || negativeSign).toUint();

        (bool absSuccess, uint256 absValue) = tryParseUint(input, begin + offset, end);

        if (absSuccess && absValue < ABS_MIN_INT256) {
            return (true, negativeSign ? -int256(absValue) : int256(absValue));
        } else if (absSuccess && negativeSign && absValue == ABS_MIN_INT256) {
            return (true, type(int256).min);
        } else return (false, 0);
    }

    /**
     * @dev Parse a hexadecimal string (with or without "0x" prefix), and returns the value as a `uint256`.
     *
     * Requirements:
     * - The string must be formatted as `(0x)?[0-9a-fA-F]*`
     * - The result must fit in an `uint256` type.
     */
    function parseHexUint(string memory input) internal pure returns (uint256) {
        return parseHexUint(input, 0, bytes(input).length);
    }

    /**
     * @dev Variant of {parseHexUint-string} that parses a substring of `input` located between position `begin` (included) and
     * `end` (excluded).
     *
     * Requirements:
     * - The substring must be formatted as `(0x)?[0-9a-fA-F]*`
     * - The result must fit in an `uint256` type.
     */
    function parseHexUint(string memory input, uint256 begin, uint256 end) internal pure returns (uint256) {
        (bool success, uint256 value) = tryParseHexUint(input, begin, end);
        if (!success) revert StringsInvalidChar();
        return value;
    }

    /**
     * @dev Variant of {parseHexUint-string} that returns false if the parsing fails because of an invalid character.
     *
     * NOTE: This function will revert if the result does not fit in a `uint256`.
     */
    function tryParseHexUint(string memory input) internal pure returns (bool success, uint256 value) {
        return _tryParseHexUintUncheckedBounds(input, 0, bytes(input).length);
    }

    /**
     * @dev Variant of {parseHexUint-string-uint256-uint256} that returns false if the parsing fails because of an
     * invalid character.
     *
     * NOTE: This function will revert if the result does not fit in a `uint256`.
     */
    function tryParseHexUint(
        string memory input,
        uint256 begin,
        uint256 end
    ) internal pure returns (bool success, uint256 value) {
        if (end > bytes(input).length || begin > end) return (false, 0);
        return _tryParseHexUintUncheckedBounds(input, begin, end);
    }

    /**
     * @dev Implementation of {tryParseHexUint-string-uint256-uint256} that does not check bounds. Caller should make sure that
     * `begin <= end <= input.length`. Other inputs would result in undefined behavior.
     */
    function _tryParseHexUintUncheckedBounds(
        string memory input,
        uint256 begin,
        uint256 end
    ) private pure returns (bool success, uint256 value) {
        bytes memory buffer = bytes(input);

        // skip 0x prefix if present
        bool hasPrefix = (end > begin + 1) && bytes2(_unsafeReadBytesOffset(buffer, begin)) == bytes2("0x"); // don't do out-of-bound (possibly unsafe) read if sub-string is empty
        uint256 offset = hasPrefix.toUint() * 2;

        uint256 result = 0;
        for (uint256 i = begin + offset; i < end; ++i) {
            uint8 chr = _tryParseChr(bytes1(_unsafeReadBytesOffset(buffer, i)));
            if (chr > 15) return (false, 0);
            result *= 16;
            unchecked {
                // Multiplying by 16 is equivalent to a shift of 4 bits (with additional overflow check).
                // This guarantees that adding a value < 16 will not cause an overflow, hence the unchecked.
                result += chr;
            }
        }
        return (true, result);
    }

    /**
     * @dev Parse a hexadecimal string (with or without "0x" prefix), and returns the value as an `address`.
     *
     * Requirements:
     * - The string must be formatted as `(0x)?[0-9a-fA-F]{40}`
     */
    function parseAddress(string memory input) internal pure returns (address) {
        return parseAddress(input, 0, bytes(input).length);
    }

    /**
     * @dev Variant of {parseAddress-string} that parses a substring of `input` located between position `begin` (included) and
     * `end` (excluded).
     *
     * Requirements:
     * - The substring must be formatted as `(0x)?[0-9a-fA-F]{40}`
     */
    function parseAddress(string memory input, uint256 begin, uint256 end) internal pure returns (address) {
        (bool success, address value) = tryParseAddress(input, begin, end);
        if (!success) revert StringsInvalidAddressFormat();
        return value;
    }

    /**
     * @dev Variant of {parseAddress-string} that returns false if the parsing fails because the input is not a properly
     * formatted address. See {parseAddress-string} requirements.
     */
    function tryParseAddress(string memory input) internal pure returns (bool success, address value) {
        return tryParseAddress(input, 0, bytes(input).length);
    }

    /**
     * @dev Variant of {parseAddress-string-uint256-uint256} that returns false if the parsing fails because input is not a properly
     * formatted address. See {parseAddress-string-uint256-uint256} requirements.
     */
    function tryParseAddress(
        string memory input,
        uint256 begin,
        uint256 end
    ) internal pure returns (bool success, address value) {
        if (end > bytes(input).length || begin > end) return (false, address(0));

        bool hasPrefix = (end > begin + 1) && bytes2(_unsafeReadBytesOffset(bytes(input), begin)) == bytes2("0x"); // don't do out-of-bound (possibly unsafe) read if sub-string is empty
        uint256 expectedLength = 40 + hasPrefix.toUint() * 2;

        // check that input is the correct length
        if (end - begin == expectedLength) {
            // length guarantees that this does not overflow, and value is at most type(uint160).max
            (bool s, uint256 v) = _tryParseHexUintUncheckedBounds(input, begin, end);
            return (s, address(uint160(v)));
        } else {
            return (false, address(0));
        }
    }

    function _tryParseChr(bytes1 chr) private pure returns (uint8) {
        uint8 value = uint8(chr);

        // Try to parse `chr`:
        // - Case 1: [0-9]
        // - Case 2: [a-f]
        // - Case 3: [A-F]
        // - otherwise not supported
        unchecked {
            if (value > 47 && value < 58) value -= 48;
            else if (value > 96 && value < 103) value -= 87;
            else if (value > 64 && value < 71) value -= 55;
            else return type(uint8).max;
        }

        return value;
    }

    /**
     * @dev Escape special characters in JSON strings. This can be useful to prevent JSON injection in NFT metadata.
     *
     * WARNING: This function should only be used in double quoted JSON strings. Single quotes are not escaped.
     *
     * NOTE: This function escapes all unicode characters, and not just the ones in ranges defined in section 2.5 of
     * RFC-4627 (U+0000 to U+001F, U+0022 and U+005C). ECMAScript's `JSON.parse` does recover escaped unicode
     * characters that are not in this range, but other tooling may provide different results.
     */
    function escapeJSON(string memory input) internal pure returns (string memory) {
        bytes memory buffer = bytes(input);
        bytes memory output = new bytes(2 * buffer.length); // worst case scenario
        uint256 outputLength = 0;

        for (uint256 i; i < buffer.length; ++i) {
            bytes1 char = bytes1(_unsafeReadBytesOffset(buffer, i));
            if (((SPECIAL_CHARS_LOOKUP & (1 << uint8(char))) != 0)) {
                output[outputLength++] = "\\";
                if (char == 0x08) output[outputLength++] = "b";
                else if (char == 0x09) output[outputLength++] = "t";
                else if (char == 0x0a) output[outputLength++] = "n";
                else if (char == 0x0c) output[outputLength++] = "f";
                else if (char == 0x0d) output[outputLength++] = "r";
                else if (char == 0x5c) output[outputLength++] = "\\";
                else if (char == 0x22) {
                    // solhint-disable-next-line quotes
                    output[outputLength++] = '"';
                }
            } else {
                output[outputLength++] = char;
            }
        }
        // write the actual length and deallocate unused memory
        assembly ("memory-safe") {
            mstore(output, outputLength)
            mstore(0x40, add(output, shl(5, shr(5, add(outputLength, 63)))))
        }

        return string(output);
    }

    /**
     * @dev Reads a bytes32 from a bytes array without bounds checking.
     *
     * NOTE: making this function internal would mean it could be used with memory unsafe offset, and marking the
     * assembly block as such would prevent some optimizations.
     */
    function _unsafeReadBytesOffset(bytes memory buffer, uint256 offset) private pure returns (bytes32 value) {
        // This is not memory safe in the general case, but all calls to this private function are within bounds.
        assembly ("memory-safe") {
            value := mload(add(buffer, add(0x20, offset)))
        }
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.30;

import {Math} from "@openzeppelin/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/utils/math/SafeCast.sol";

/**
 * @title SaturatingMath library
 * @author Covenant Labs
 * @notice Provides a saturating mulDiv operation
 */
library SaturatingMath {
    // Returns a saturating mulDiv operation
    // @dev - does not overflow, but instead returns type(uint256).max if so.
    function saturatingMulDiv(
        uint256 _numerator1,
        uint256 _numerator2,
        uint256 _denominator
    ) internal pure returns (uint256) {
        (uint256 high, uint256 low) = Math.mul512(_numerator1, _numerator2);

        // @dev - below follows the logic of Math.mulDiv, but saturates instead of reverting.
        if (high >= _denominator) {
            // returns type(uint256).max for all overflow and _denominator == 0 conditions
            return type(uint256).max;
        } else if (high == 0) {
            // @dev - execute 256 bit division here directly.
            // already checked for denominator == 0
            unchecked {
                return low / _denominator;
            }
        } else {
            // @dev - would be more efficient to do a 512 division here,
            // but OpenZeppelin does not have a separate (already audited) function.
            // So below recomputes Math.mul512 internally, and then performs the division.
            // Does not revert given checks above.
            return Math.mulDiv(_numerator1, _numerator2, _denominator);
        }
    }

    function saturatingMulDiv(
        uint256 x,
        uint256 y,
        uint256 denominator,
        Math.Rounding rounding
    ) internal pure returns (uint256 result) {
        result = saturatingMulDiv(x, y, denominator);
        return
            result +
            SafeCast.toUint(
                Math.unsignedRoundsUp(rounding) && mulmod(x, y, denominator) > 0 && result < type(uint256).max
            );
    }

    /**
     * @dev Calculates floor(x * y >> n) with full precision. saturates instead of reverting.
     * @dev Code copies @openzeppelin/utils/math/Math.sol:mulShr, but saturates instead of reverting.
     */
    function saturatingMulShr(uint256 x, uint256 y, uint8 n) internal pure returns (uint256 result) {
        unchecked {
            (uint256 high, uint256 low) = Math.mul512(x, y);
            if (high >= 1 << n) {
                return type(uint256).max; // @dev - saturates instead of reverting for overflow.
            }
            return (high << (256 - n)) | (low >> n);
        }
    }
}

// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.0;

import {MarketId, AssetType} from "../interfaces/ICovenant.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";

/**
 * @title ICovenant
 * @author Amorphous
 * @notice Defines the the core interface of Covenant Liquid markets.
 **/
interface ISynthToken is IERC20 {
    // Notice - gets CovenantCore associated with the SynthToken
    function getCovenantCore() external returns (address);

    // Notice - gets marketId associated with the SynthToken
    function getMarketId() external returns (MarketId);

    // Notice - gets synthType associated with the SynthToken
    function getSynthType() external returns (AssetType);

    /**
     * @dev Expose share mint functionality to Covenant Liquid
     */
    function lexMint(address account, uint256 value) external;

    /**
     * @dev Expose share redeem functionality to Covenant Liquid
     */
    function lexBurn(address account, uint256 value) external;
}

// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.0;

import {ILiquidExchangeModel, AssetType, MintParams, RedeemParams, SwapParams, MarketId, MarketParams, TokenPrices, SynthTokens} from "src/interfaces/ILiquidExchangeModel.sol";

struct LexState {
    uint256 lastDebtNotionalPrice; // WAD units
    uint256 lastBaseTokenPrice; // Last oracle read WAD units
    uint256 lastETWAPBaseSupply; // Tracks baseSupply for redeem cap
    uint160 lastSqrtPriceX96; // Last DEX price, X96 units
    uint96 lastUpdateTimestamp; // Timestamp in seconds
    int64 lastLnRateBias; // WAD units.
}

struct LexConfig {
    uint32 protocolFee; // Protocol fees in BPS units (uint16 tvlFee, uint16 yieldFee)
    address aToken;
    address zToken;
    uint8 noCapLimit; // Max liquidity mint / burn without a cap limit.  Limit = 2^noCapLimit
    int8 scaleDecimals; // Scale decimals (used for scaling the price from the oracle)
    bool adaptive; // Whether debtPriceDiscountBalanced is adaptive
}

struct LexParams {
    address covenantCore;
    int64 initLnRateBias;
    uint160 edgeSqrtPriceX96_B; // high edge of concentrated liquidity
    uint160 edgeSqrtPriceX96_A; // low edge of concentrated liquidity
    uint160 limHighSqrtPriceX96; // from which _highLTV can be derived (no aToken sales, no zToken buys)
    uint160 limMaxSqrtPriceX96; // from which _maxLTV can be derived (same as _highLTV && no aToken buys)
    uint32 debtDuration; // perpetual duration of debt, in seconds (max 100 years)
    uint8 swapFee; // BPS fee when swapping tokens.  Max of 2.55% swap fee
    uint256 targetXvsL; // pre-calculated static value
}

/**
 * @title ILatentSwapLEX
 * @author Covenant Labs
 * @notice Defines the interface for ILatentSwapLEX.sol
 **/
interface ILatentSwapLEX is ILiquidExchangeModel {
    ///////////////////////////////////////////////////////////////////////////////
    // Getters

    /// @notice LexParams (constructor) getter
    function getLexParams() external view returns (LexParams memory);

    /// @notice LexState getter
    function getLexState(MarketId marketId) external view returns (LexState memory);

    /// @notice LexConfig getter
    function getLexConfig(MarketId marketId) external view returns (LexConfig memory);

    ///////////////////////////////////////////////////////////////////////////////
    // Write functions (only Owner calls)

    /// @notice sets default noCapDecimals for a quote token
    /// @dev setting noCapLimit = 255 removes mint / redeem restriction for markets using this quoteToken
    /// @param token the quote token address
    /// @param newDefaultNoCapLimit the default noCapLimit for markets with this quote token
    function setDefaultNoCapLimit(address token, uint8 newDefaultNoCapLimit) external;

    /// @notice updates the noCapDecimals for a live market
    /// @dev this is useful when the market is live and the quote token is not an actual ERC20
    /// @dev setting noCapLimit = 255 removes mint / redeem restriction for the market
    /// @param marketId the market id
    /// @param newNoCapLimit the noCapLimit for the market (in power of 2).  Markets can mint and redeem baseTokens
    //  wihout mint and redeem caps if baseTokenSupply < 2^nowCapLimt.
    function setMarketNoCapLimit(MarketId marketId, uint8 newNoCapLimit) external;
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

// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.0;

import {ILiquidExchangeModel, AssetType, MintParams, RedeemParams, SwapParams, MarketId, MarketParams, TokenPrices, SynthTokens} from "src/interfaces/ILiquidExchangeModel.sol";

struct LexState {
    uint256 lastDebtNotionalPrice; // WAD units
    uint256 lastBaseTokenPrice; // Last oracle read WAD units
    uint256 lastETWAPBaseSupply; // Tracks baseSupply for redeem cap
    uint160 lastSqrtPriceX96; // Last DEX price, X96 units
    uint96 lastUpdateTimestamp; // Timestamp in seconds
    int64 lastLnRateBias; // WAD units.
}

struct LexConfig {
    uint32 protocolFee; // Protocol fees in BPS units (uint16 tvlFee, uint16 yieldFee)
    address aToken;
    address zToken;
    uint8 noCapLimit; // Max liquidity mint / burn without a cap limit.  Limit = 2^noCapLimit
    int8 scaleDecimals; // Scale decimals (used for scaling the price from the oracle)
    bool adaptive; // Whether debtPriceDiscountBalanced is adaptive
}

struct LexParams {
    address covenantCore;
    int64 initLnRateBias;
    uint160 edgeSqrtPriceX96_B; // high edge of concentrated liquidity
    uint160 edgeSqrtPriceX96_A; // low edge of concentrated liquidity
    uint160 limHighSqrtPriceX96; // from which _highLTV can be derived (no aToken sales, no zToken buys)
    uint160 limMaxSqrtPriceX96; // from which _maxLTV can be derived (same as _highLTV && no aToken buys)
    uint32 debtDuration; // perpetual duration of debt, in seconds (max 100 years)
    uint8 swapFee; // BPS fee when swapping tokens.  Max of 2.55% swap fee
    uint256 targetXvsL; // pre-calculated static value
}

/**
 * @title ILatentSwapLEX
 * @author Covenant Labs
 * @notice Defines the interface for ILatentSwapLEX.sol
 **/
interface ILatentSwapLEX is ILiquidExchangeModel {
    ///////////////////////////////////////////////////////////////////////////////
    // Getters

    /// @notice LexParams (constructor) getter
    function getLexParams() external view returns (LexParams memory);

    /// @notice LexState getter
    function getLexState(MarketId marketId) external view returns (LexState memory);

    /// @notice LexConfig getter
    function getLexConfig(MarketId marketId) external view returns (LexConfig memory);

    ///////////////////////////////////////////////////////////////////////////////
    // Write functions (only Owner calls)

    /// @notice sets default noCapDecimals for a quote token
    /// @dev setting noCapLimit = 255 removes mint / redeem restriction for markets using this quoteToken
    /// @param token the quote token address
    /// @param newDefaultNoCapLimit the default noCapLimit for markets with this quote token
    function setDefaultNoCapLimit(address token, uint8 newDefaultNoCapLimit) external;

    /// @notice updates the noCapDecimals for a live market
    /// @dev this is useful when the market is live and the quote token is not an actual ERC20
    /// @dev setting noCapLimit = 255 removes mint / redeem restriction for the market
    /// @param marketId the market id
    /// @param newNoCapLimit the noCapLimit for the market (in power of 2).  Markets can mint and redeem baseTokens
    //  wihout mint and redeem caps if baseTokenSupply < 2^nowCapLimt.
    function setMarketNoCapLimit(MarketId marketId, uint8 newNoCapLimit) external;
}

//SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.30;

import {TokenPrices, AssetType} from "../../../interfaces/ILiquidExchangeModel.sol";
import {LexConfig, LexState, MintParams, RedeemParams, SwapParams, MarketId, MarketParams, LexParams} from "../interfaces/ILatentSwapLEX.sol";
import {Math} from "@openzeppelin/utils/math/Math.sol";
import {LSErrors} from "./LSErrors.sol";
import {LatentMath} from "./LatentMath.sol";
import {FixedPoint} from "./FixedPoint.sol";
import {DebtMath} from "./DebtMath.sol";
import {UtilsLib} from "../../../libraries/Utils.sol";
import {PercentageMath} from "@aave/libraries/math/PercentageMath.sol";
import {SaturatingMath} from "./SaturatingMath.sol";
import {IPriceOracle} from "../../../interfaces/IPriceOracle.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {SafeCast} from "@openzeppelin/utils/math/SafeCast.sol";
import {SafeMetadata} from "../../../libraries/SafeMetadata.sol";
import {Strings} from "@openzeppelin/utils/Strings.sol";
import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";

/**
 * @title Latent Swap Logic
 * @author Covenant Labs
 **/

library LatentSwapLogic {
    using SafeCast for uint256;
    using SafeCast for bool;
    using Math for uint256;
    using SaturatingMath for uint256;
    using PercentageMath for uint256;
    using SafeMetadata for IERC20;

    uint8 constant MAX_MINT_FACTOR_CAP = 1; // x1 max liquidity mint as a % of total market liquidity (measured through ETWAP). ie, limits amount that can be minted in ETWAP_MIN_HALF_LIFE minutes
    uint8 constant MAX_REDEEM_FACTOR_CAP = 2; // 1/4 max liquidity burn as a % of total market liquidity (measured through ETWAP). ie, limits amount that can be burned in ETWAP_MIN_HALF_LIFE minutes
    uint256 constant MAX_SYNTH_MINT_CAP = uint256(1) << 242; // do not allow minting more than 2^242 in aTokens and zTokens.  This allows for further market appreciation in value (and protects from percentageMul overflows)
    uint16 constant ETWAP_MIN_HALF_LIFE = 30 minutes; // Minimum half life of ETWAP calculations (ETWAP can have a longer halflife if market has infrequent updates)
    uint96 constant LN2 = 693147180559945331; // ln(2) in WADs.  Used for half-life calculations
    uint32 constant MIN_LIQRATIOX96 = 1e9; // Minimum Price * LiqRatio for market.  Prices under 1 wei (if in WADs) will revert.
    uint40 constant NEG_WORKOUT_LN_RATE = 116323331638; // -1% daily workout rate (when undercollateralized or above MAX_LIMIT_LTV). Expressed as lnRate per second in WADs.  Workout rate is negative, but here expressed as positive.
    uint8 constant DEBT = 0; // used for indexing into supplyAmounts and dexAmounts
    uint8 constant LVRG = 1; // used for indexing into supplyAmounts and dexAmounts

    // Market state - for cache + state calculated values
    struct LexFullState {
        LexState lexState;
        LexConfig lexConfig;
        uint256 baseTokenSupply;
        uint256[2] supplyAmounts;
        uint256[2] dexAmounts;
        uint256[3] dexAmountsScaled;
        uint256[3] synthAmountsScaled;
        uint256 liquidityRatioX96;
        uint160 liquidity;
        uint96 accruedProtocolFee;
        bool underCollateralized;
    }

    // Market init info struct
    struct MarketInitInfo {
        string baseName;
        string baseSymbol;
        string quoteSymbol;
        string aTokenName;
        string aTokenSymbol;
        string zTokenName;
        string zTokenSymbol;
        uint8 synthDecimals;
        uint8 quoteDecimals;
        uint8 noCapLimit;
        string levStr;
        string durStr;
    }

    ///////////////////////////////////////////////////////////////////////////////
    // External library functions
    ///////////////////////////////////////////////////////////////////////////////

    function mintLogic(
        MintParams calldata mintParams,
        LexParams memory lexParams,
        LexConfig storage lexConfig,
        LexState storage lexState,
        uint256 baseTokenSupply,
        bool isPreview
    )
        external
        view
        returns (
            LexFullState memory currentState,
            TokenPrices memory tokenPrices,
            uint256 aTokenAmountOut,
            uint256 zTokenAmountOut
        )
    {
        ///////////////////////////////
        // Calculate market state (storage read)
        currentState = _calculateMarketState(
            mintParams.marketParams,
            lexParams,
            lexConfig,
            lexState,
            baseTokenSupply,
            isPreview
        );

        ///////////////////////////////
        // Calculate mint
        (aTokenAmountOut, zTokenAmountOut) = _calcMint(lexParams, currentState, mintParams.baseAmountIn);

        /////////////////////////////////////
        // Calculate token prices (post action)
        tokenPrices = _calculateTokenPrices(lexParams, currentState);
    }

    function redeemLogic(
        RedeemParams calldata redeemParams,
        LexParams memory lexParams,
        LexConfig storage lexConfig,
        LexState storage lexState,
        uint256 baseTokenSupply,
        bool isPreview
    ) external view returns (LexFullState memory currentState, TokenPrices memory tokenPrices, uint256 amountOut) {
        ///////////////////////////////
        // Calculate market state (storage read)
        currentState = _calculateMarketState(
            redeemParams.marketParams,
            lexParams,
            lexConfig,
            lexState,
            baseTokenSupply,
            isPreview
        );

        ///////////////////////////////
        // Calculate redeem
        (amountOut, currentState.lexState.lastSqrtPriceX96) = _calcRedeem(
            lexParams,
            currentState,
            redeemParams.aTokenAmountIn,
            redeemParams.zTokenAmountIn
        );

        /////////////////////////////////////
        // Calculate token prices (post action)
        // @dev - prices are miscalculated if a market is fully redeemed, and are stated pre-action instead.
        tokenPrices = _calculateTokenPrices(lexParams, currentState);
    }

    function swapLogic(
        SwapParams calldata swapParams,
        LexParams memory lexParams,
        LexConfig storage lexConfig,
        LexState storage lexState,
        uint256 baseTokenSupply,
        bool isPreview
    )
        external
        view
        returns (LexFullState memory currentState, TokenPrices memory tokenPrices, uint256 amountCalculated)
    {
        ///////////////////////////////
        // Calculate market state (storage read)
        currentState = _calculateMarketState(
            swapParams.marketParams,
            lexParams,
            lexConfig,
            lexState,
            baseTokenSupply,
            isPreview
        ); ///////////////////////////////

        // Calculate swap
        (amountCalculated, currentState.lexState.lastSqrtPriceX96) = _calcSwap(
            lexParams,
            currentState,
            swapParams.amountSpecified,
            swapParams.assetIn,
            swapParams.assetOut,
            swapParams.isExactIn
        );

        /////////////////////////////////////
        // Calculate token prices (post action)
        // @dev - prices are miscalculated if a market is fully redeemed, and are stated pre-action instead.
        tokenPrices = _calculateTokenPrices(lexParams, currentState);
    }

    // Retrieve baseToken price
    // @dev - baseTokenPrice is the value of 10^18 baseTokens, in quoteTokens, irrespective of actual # of decimal precision the baseToken has
    function readBasePriceAndCalculateLiqRatio(
        MarketParams calldata marketParams,
        uint256 liquidityConcentrationX96,
        int8 scaleDecimals,
        bool isPreview
    ) external view returns (uint256 price, uint256 liqRatioX96) {
        return _readBasePriceAndCalculateLiqRatio(marketParams, liquidityConcentrationX96, scaleDecimals, isPreview);
    }

    function calculateMarketState(
        MarketParams calldata marketParams,
        LexParams memory lexParams,
        LexConfig storage lexConfig,
        LexState storage lexState,
        uint256 baseTokenSupply,
        bool isPreview
    ) external view returns (LexFullState memory marketState) {
        return _calculateMarketState(marketParams, lexParams, lexConfig, lexState, baseTokenSupply, isPreview);
    }

    function calcRatio(
        LexParams memory lexParams,
        LexFullState memory marketState,
        AssetType base,
        AssetType quote
    ) external pure returns (uint256 price) {
        return _calcRatio(lexParams, marketState, base, quote);
    }

    function getDebtPriceDiscount(
        uint160 edgeSqrtPriceX96_A,
        uint160 edgeSqrtPriceX96_B,
        uint160 currentSqrtPriceX96,
        uint256 target_dXdL_X96
    ) external pure returns (uint256 currentPriceDiscount) {
        return _getDebtPriceDiscount(edgeSqrtPriceX96_A, edgeSqrtPriceX96_B, currentSqrtPriceX96, target_dXdL_X96);
    }

    // @dev - externalizing computations used during LatentSwapLEX contract creation
    function computeMaxLTVandTargetdXdL(
        uint160 edgeSqrtPriceX96_A,
        uint160 edgeSqrtPriceX96_B,
        uint160 limMaxSqrtPriceX96
    ) external pure returns (uint256 maxLTV, uint256 target_dXdL_X96) {
        maxLTV = LatentMath.computeLTV(edgeSqrtPriceX96_A, edgeSqrtPriceX96_B, limMaxSqrtPriceX96);
        target_dXdL_X96 = LatentMath.targetXvsL(edgeSqrtPriceX96_A, edgeSqrtPriceX96_B);
    }

    // @dev - externalizing computations used during market initialization
    function getInitMarketInfo(
        MarketParams calldata marketParams,
        uint256 debtDuration,
        uint160 edgeSqrtPriceX96_A,
        uint160 edgeSqrtPriceX96_B,
        uint8 baseTokenNoCapLimit,
        uint8 quoteDecimals,
        string memory quoteSymbol
    ) external view returns (MarketInitInfo memory info) {
        bool success;

        // Get base asset name and symbol (cannot be overriden)
        (success, info.baseName) = IERC20(marketParams.baseToken).tryGetName();
        if (!success) revert LSErrors.E_LEX_BaseAssetNotERC20();
        (success, info.baseSymbol) = IERC20(marketParams.baseToken).tryGetSymbol();
        if (!success) revert LSErrors.E_LEX_BaseAssetNotERC20();

        // Get quote symbol with overrides
        info.quoteSymbol = quoteSymbol; // read
        if (bytes(info.quoteSymbol).length == 0) revert LSErrors.E_LEX_QuoteAssetHasNoSymbol();

        // Get synthDecimals decimals (== quote decimals with overrides)
        info.synthDecimals = quoteDecimals;
        if (info.synthDecimals == 0) info.synthDecimals = 18;

        // Get actual quote decimals (as would be returned by IPriceOracle)
        (success, info.quoteDecimals) = IERC20(marketParams.quoteToken).tryGetDecimals();
        if (!success) info.quoteDecimals = 18;

        // Get base Token noCap decimals
        info.noCapLimit = baseTokenNoCapLimit;
        if (info.noCapLimit == 0) {
            // default threshold above which mint / redeem cap are applied is equivalent to ~1 baseToken
            (bool success2, uint8 baseDecimals) = IERC20(marketParams.baseToken).tryGetDecimals();
            if (!success2)
                info.noCapLimit = uint8(60); // 18 Decimals -> 60 noCapLimit
            else info.noCapLimit = (FixedPointMathLib.log2(10 ** baseDecimals) + 1).toUint8();
        }

        // Calculate target leverage
        // leverage factor = 1/(1-targetLTV).  e.g., 50% -> 2, 80% -> 5, 90% -> 10, 95% -> 20
        info.levStr = Strings.toString(
            (FixedPoint.PERCENTAGE_FACTOR + 1) /
                (FixedPoint.PERCENTAGE_FACTOR -
                    LatentMath.computeLTV(edgeSqrtPriceX96_A, edgeSqrtPriceX96_B, uint160(FixedPoint.Q96)))
        );

        // Calculate duration in months / years
        // e.g., 1M, 3M, 6M, 1Y, 2Y, 5Y
        // @dev - simplified approach rounds down.
        uint256 months = debtDuration / (30 days);
        if (months < 12) info.durStr = string.concat(Strings.toString(months), "M");
        else info.durStr = string.concat(Strings.toString(debtDuration / (365 days)), "Y");

        info.aTokenName = string.concat(info.baseName," x",info.levStr," Leverage Coin (",info.quoteSymbol,"/",info.durStr,")"); // prettier-ignore
        info.aTokenSymbol = string.concat(info.baseSymbol, "x", info.levStr, ".", info.quoteSymbol);
        info.zTokenName = string.concat(info.quoteSymbol," Yield Coin - backed by ",info.baseSymbol," (x",info.levStr,"/",info.durStr,")"); // prettier-ignore
        info.zTokenSymbol = string.concat(info.quoteSymbol, ".b.", info.baseSymbol);
    }

    ///////////////////////////////////////////////////////////////////////////////
    // Internal functions
    ///////////////////////////////////////////////////////////////////////////////

    function _calcMint(
        LexParams memory lexParams,
        LexFullState memory marketState,
        uint256 baseTokenAmountIn
    ) internal pure returns (uint256 aTokenAmountOut, uint256 zTokenAmountOut) {
        ///////////////////////////////
        // Validate inputs

        // Check if undercollateralized (reverts if so)
        _checkUnderCollateralized(marketState);

        // Check LTV before mint (reverts if LTV above MAX_LIMIT_LTV)
        // @dev - minting aTokens when above MAX_LIMIT_LTV is blocked to avoid excessive aToken dilution
        // in high LTV or undercollateralized markets.
        _checkLTV(marketState.lexState.lastSqrtPriceX96, lexParams.limMaxSqrtPriceX96);

        // Check if mint amount is too large given market size
        _checkMintCap(
            marketState.baseTokenSupply,
            marketState.lexState.lastETWAPBaseSupply,
            baseTokenAmountIn,
            marketState.lexConfig.noCapLimit
        );

        ///////////////////////////////
        // Calculate mint
        // Calculate liquidity coming in (round down)
        uint160 liquidityIn = _synthToDex(marketState, baseTokenAmountIn, AssetType.BASE, Math.Rounding.Floor)
            .toUint160();
        // Calculate dex amounts that should be minted

        (uint256 zDexToMint, uint256 aDexToMint) = LatentMath.computeMint(
            marketState.lexState.lastSqrtPriceX96,
            lexParams.edgeSqrtPriceX96_A,
            lexParams.edgeSqrtPriceX96_B,
            liquidityIn
        );

        // Calculate actual zTokens / aTokens to mint (round down).
        // @dev - Use debt ratio for leverage token if this is the first liquidity in
        // (ratio not important for leverage token, so keep in targetLTV range)
        zTokenAmountOut = _dexToSynth(marketState, zDexToMint, AssetType.DEBT, Math.Rounding.Floor);
        aTokenAmountOut = _dexToSynth(marketState, aDexToMint, AssetType.LEVERAGE, Math.Rounding.Floor);

        ///////////////////////////////
        // Validate outputs

        // ensure we are not miniting more than MAX_SYNTH_MINT_CAP for zToken and aToken
        // Block minting while still allowing market value to appreciate.
        _checkSynthMintCap(marketState.supplyAmounts[DEBT], zTokenAmountOut);
        _checkSynthMintCap(marketState.supplyAmounts[LVRG], aTokenAmountOut);

        ///////////////////////////////
        // Charge fee by reducing out amount
        if (lexParams.swapFee > 0) {
            zTokenAmountOut = zTokenAmountOut.percentMul(FixedPoint.PERCENTAGE_FACTOR - lexParams.swapFee);
            aTokenAmountOut = aTokenAmountOut.percentMul(FixedPoint.PERCENTAGE_FACTOR - lexParams.swapFee);
        }
    }

    function _calcRedeem(
        LexParams memory lexParams,
        LexFullState memory marketState,
        uint256 aTokenAmountIn,
        uint256 zTokenAmountIn
    ) internal pure returns (uint256 amountOut, uint160 nextSqrtPriceX96) {
        // store for output validation
        uint160 beforeSqrtPriceX96 = marketState.lexState.lastSqrtPriceX96;

        ///////////////////////////////
        // Validate inputs
        if (aTokenAmountIn > marketState.supplyAmounts[LVRG]) revert LSErrors.E_LEX_InsufficientTokens();
        if (zTokenAmountIn > marketState.supplyAmounts[DEBT]) revert LSErrors.E_LEX_InsufficientTokens();

        // Validate liquidity
        // @dev - testing baseTokenSupply, because logic below allows for removal of baseTokenSupply
        // even if the market's calculated liquidity = 0.  In these situations, we do not go through
        // the latentSwap invariant, but instead assume all baseTokens belong to zToken holders (if any),
        // or otherwise to aToken holders, and redeem actionas are done proportioanl to holdings.
        if (marketState.baseTokenSupply == 0) revert LSErrors.E_LEX_ZeroLiquidity();

        ///////////////////////////////
        // Calculate redeem

        // Check for a full redeem
        if (aTokenAmountIn == marketState.supplyAmounts[LVRG] && zTokenAmountIn == marketState.supplyAmounts[DEBT]) {
            // @dev - when full redeem, no swap fee nor redeemCap checks, set to target price
            // @dev - we acknowledge that there is a footgun risk or someone preemptin a full redeem to capture fees
            // ie, redeem fees would not be captured by the attacker.  This can be mitigated by redeeming in two steps if
            // the last user feels the fees are worth it...
            // @dev - it is possible to redeem dust baseToken amounts even when liquidity == 0
            amountOut = marketState.baseTokenSupply;
            nextSqrtPriceX96 = uint160(FixedPoint.Q96);
        } else {
            if (marketState.underCollateralized || marketState.liquidity == 0) {
                if (marketState.supplyAmounts[DEBT] > 0) {
                    // if market is undercollateralized, only allow zToken -> base redeems, and make these proportional
                    // @dev - it is possible to redeem undercollateralized markets even if baseSupply > 0 but liquidity == 0
                    if (aTokenAmountIn > 0) revert LSErrors.E_LEX_ActionNotAllowedUnderCollateralized();

                    // Proportional redeem
                    // @dev - we do not go through the _synthToDex -> LatentMath -> _dexToSynth pathway
                    // to calculate amounts given we are at the market extreme where debt tokens are the full owners of base tokens
                    // and amounts can be just calculated proportional to market amounts
                    // @notice - amount out is a floor, but we can do a full redeem if zTokenAmountIn == marketState.supplyAmounts[DEBT]
                    amountOut = marketState.baseTokenSupply.mulDiv(zTokenAmountIn, marketState.supplyAmounts[DEBT]);

                    // Calc next price across the following three states:
                    // 1 - zTokens still in market, so remains undercollateralized and nextSqrtPrice = edgeSqrtPriceX96_B
                    // 1 - No baseTokens or zTokens left, but aTokens still in market -> nextSqrtPrice = edgeSqrtPriceX96_A
                    // 2 - fully empy market -> nextSqrtPrice = 1
                    nextSqrtPriceX96 = (amountOut < marketState.baseTokenSupply)
                        ? lexParams.edgeSqrtPriceX96_B
                        : (marketState.supplyAmounts[LVRG] > 0)
                            ? lexParams.edgeSqrtPriceX96_A
                            : uint160(FixedPoint.Q96);
                } else {
                    ////////////////////////////////////
                    // Allow proprional redeeming of baseTokens with leverage tokens, given no zTokens in the market
                    amountOut = marketState.baseTokenSupply.mulDiv(aTokenAmountIn, marketState.supplyAmounts[LVRG]);
                    nextSqrtPriceX96 = (amountOut < marketState.baseTokenSupply)
                        ? lexParams.edgeSqrtPriceX96_A
                        : uint160(FixedPoint.Q96);
                }
                // No redeem fees charged in undercollateralized or  zero liquiditymarket

                ///////////////////////////////////////
                // Validate output
                // Check if redeem amount is too large given market size, even when undercollateralized
                _checkRedeemCap(
                    marketState.baseTokenSupply,
                    marketState.lexState.lastETWAPBaseSupply,
                    amountOut,
                    marketState.lexConfig.noCapLimit
                );
            } else {
                uint256 zTokenDexIn = _synthToDex(marketState, zTokenAmountIn, AssetType.DEBT, Math.Rounding.Floor);
                uint256 aTokenDexIn = _synthToDex(marketState, aTokenAmountIn, AssetType.LEVERAGE, Math.Rounding.Floor);

                // Calculate liquidity out given tokens in
                uint160 liquidityOut;
                (liquidityOut, nextSqrtPriceX96) = LatentMath.computeRedeem(
                    marketState.liquidity,
                    marketState.lexState.lastSqrtPriceX96,
                    lexParams.edgeSqrtPriceX96_A,
                    lexParams.edgeSqrtPriceX96_B,
                    zTokenDexIn,
                    aTokenDexIn
                );

                // Calculate baseToken amount out given liquidity out
                // Round down amount out
                amountOut = _dexToSynth(marketState, liquidityOut, AssetType.BASE, Math.Rounding.Floor);

                if (amountOut >= marketState.baseTokenSupply || liquidityOut >= marketState.liquidity) {
                    // @dev - this is close to a full redeem, but there is some aToken or zToken dust left in the market
                    // @dev - when full redeem, no swap fee nor redeemCap checks, set to target price
                    // @dev - we acknowledge that there is a footgun risk or someone preemptin a full redeem to capture fees
                    // ie, redeem fees would not be captured by the attacker.  This can be mitigated by redeeming in two steps if
                    // the last user feels the fees are worth it...
                    // @dev - it is possible to redeem dust baseToken amounts even when liquidity == 0
                    // @dev - some dust aTokens or zTokens might be left in the market (valueless).
                    // @dev - We ackownolded that it could be argued that fees should not be skipped in this case.
                    // If we charged fees, a majority holder could redeeming here in various steps to avoid paying dust holders
                    // We acknowledge this fact by just not charging fees in this scenario when only dust holders are left.
                    amountOut = marketState.baseTokenSupply;
                    nextSqrtPriceX96 = uint160(FixedPoint.Q96);
                } else {
                    ///////////////////////////////////////
                    // Validate output
                    // Check if redeem amount is too large given market size
                    _checkRedeemCap(
                        marketState.baseTokenSupply,
                        marketState.lexState.lastETWAPBaseSupply,
                        amountOut,
                        marketState.lexConfig.noCapLimit
                    );

                    ////////////////////////////////////////
                    // Charge fee by reducing out amount (after checkRedeemCap)
                    if (lexParams.swapFee > 0)
                        amountOut = amountOut.percentMul(FixedPoint.PERCENTAGE_FACTOR - lexParams.swapFee);
                }
            }
        }

        // Additional validate outputs

        // Allow redeems that lower LTV (lower DEX price) or keep as is, and otherwise
        // check whether action pushes LTV past High limit (reverts if so)
        if (nextSqrtPriceX96 > beforeSqrtPriceX96) _checkLTV(nextSqrtPriceX96, lexParams.limHighSqrtPriceX96);

        // @dev - allow redeeming 0 base tokens (as a way to remove dust in markets if need be)
    }

    struct calcSwapVars {
        uint256 inputDexAmount;
        uint256 calcDexAmount;
        uint256 aDexTokenAmount;
        uint256 zDexTokenAmount;
        uint256 newDexTokenAmount;
        uint160 liquidityNew;
        AssetType fixedSynth;
    }

    function _calcSwap(
        LexParams memory lexParams,
        LexFullState memory marketState,
        uint256 swapAmount,
        AssetType assetIn,
        AssetType assetOut,
        bool isExactIn
    ) internal pure returns (uint256 calcAmount, uint160 nextSqrtPriceX96) {
        ///////////////////////////////
        // Validate inputs
        require(assetIn != assetOut);

        // check market liquidity
        // @dev - it can happen that the market has liquidity == 0, even if baseTokenSupply > 0 and aTokens or zTokens > 0.
        // However, the market is not operational (more liquidity needs to be added)
        if (marketState.liquidity == 0) revert LSErrors.E_LEX_ZeroLiquidity();

        // Undercollateralized checks
        // If undercollateralized, only allow zToken to baseToken swaps  (ie redeem zTokens only)
        if ((assetIn != AssetType.DEBT) || (assetOut != AssetType.BASE)) _checkUnderCollateralized(marketState);

        // Do not allow buying aTokens if market above MAX_LIMIT_LTV
        // @dev - markets above MAX_LIMIT_LTV return to a lower LTV through negative funding (zTokens paying aTokens), collateral price appreciation, or zToken -> base swaps
        if (assetOut == AssetType.LEVERAGE)
            _checkLTV(marketState.lexState.lastSqrtPriceX96, lexParams.limMaxSqrtPriceX96);

        ///////////////////////////////
        // Calculate swap

        calcSwapVars memory vars;

        if ((assetIn != AssetType.BASE) && (assetOut != AssetType.BASE)) {
            // case1: synth for synth swap
            vars.fixedSynth = isExactIn ? assetIn : assetOut;
            vars.inputDexAmount = _synthToDex(
                marketState,
                swapAmount,
                vars.fixedSynth,
                isExactIn ? Math.Rounding.Floor : Math.Rounding.Ceil
            );

            (vars.calcDexAmount, nextSqrtPriceX96) = LatentMath.computeSwap(
                marketState.liquidity,
                marketState.lexState.lastSqrtPriceX96,
                vars.fixedSynth,
                vars.inputDexAmount,
                isExactIn
            );

            // Convert internal DEX output to synth amounts
            calcAmount = _dexToSynth(
                marketState,
                vars.calcDexAmount,
                isExactIn ? assetOut : assetIn,
                isExactIn ? Math.Rounding.Floor : Math.Rounding.Ceil
            );

            ////////////////////////////////////////////////////////
            // Validate swap amounts
            // Do not allow ExactOut > 0 if InputAmount ends being 0
            if (!isExactIn && calcAmount == 0) revert LSErrors.E_LEX_InsufficientAmount();

            // ensure we are not miniting more than MAX_SYNTH_MINT_CAP for zToken and aToken
            // Block minting while still allowing market value to appreciate.
            _checkSynthMintCap(
                marketState.supplyAmounts[assetOut == AssetType.DEBT ? DEBT : LVRG],
                isExactIn ? calcAmount : swapAmount
            );
        } else if ((assetIn == AssetType.BASE) && isExactIn) {
            // case2: base token is being swapped with an exact amount in

            // @dev - round down for all conditions
            vars.liquidityNew = _synthToDex(marketState, swapAmount, AssetType.BASE, Math.Rounding.Floor).toUint160();

            // Mint tokens given liquidity in
            (vars.zDexTokenAmount, vars.aDexTokenAmount) = LatentMath.computeMint(
                marketState.lexState.lastSqrtPriceX96,
                lexParams.edgeSqrtPriceX96_A,
                lexParams.edgeSqrtPriceX96_B,
                vars.liquidityNew
            );

            // update liquidity
            marketState.liquidity += vars.liquidityNew;

            // swap assetOut
            (vars.newDexTokenAmount, nextSqrtPriceX96) = LatentMath.computeSwap(
                marketState.liquidity,
                marketState.lexState.lastSqrtPriceX96,
                assetOut == AssetType.DEBT ? AssetType.LEVERAGE : AssetType.DEBT,
                assetOut == AssetType.DEBT ? vars.aDexTokenAmount : vars.zDexTokenAmount,
                true
            );

            // add to swap output the original mint amount of assetOut
            vars.newDexTokenAmount += Math.ternary(
                assetOut == AssetType.LEVERAGE,
                vars.aDexTokenAmount,
                vars.zDexTokenAmount
            );

            calcAmount = _dexToSynth(marketState, vars.newDexTokenAmount, assetOut, Math.Rounding.Floor);

            ////////////////////////////////////////////////////////
            // Validate mintCap if baseToken is being swapped in
            _checkMintCap(
                marketState.baseTokenSupply,
                marketState.lexState.lastETWAPBaseSupply,
                swapAmount,
                marketState.lexConfig.noCapLimit
            );

            ////////////////////////////////////////////////////////
            // Validate synth mint cap
            _checkSynthMintCap(marketState.supplyAmounts[assetOut == AssetType.DEBT ? DEBT : LVRG], calcAmount);
        } else if ((assetOut == AssetType.BASE) && isExactIn) {
            // case3: base token is being swapped out, with exact synth amount in
            // @dev - equivalent to redeeming swapAmount of assetIn
            (calcAmount, nextSqrtPriceX96) = _calcRedeem(
                lexParams,
                marketState,
                (assetIn == AssetType.LEVERAGE) ? swapAmount : 0,
                (assetIn == AssetType.DEBT) ? swapAmount : 0
            );
        } else {
            revert LSErrors.E_LEX_OperationNotAllowed();
        }

        // Charge fee by reducing out amount (or increasing in amount)
        // @dev - Base out swaps were already charged when calling _calcRedeem
        if (lexParams.swapFee > 0 && (assetOut != AssetType.BASE))
            calcAmount = isExactIn
                ? calcAmount.percentMul(FixedPoint.PERCENTAGE_FACTOR - lexParams.swapFee)
                : calcAmount.percentDiv(FixedPoint.PERCENTAGE_FACTOR - lexParams.swapFee);

        ///////////////////////////////
        // Additional validate outputs

        // Do not allow actions that increase LTV (and end past High LTV limits)
        // e.g. aToken sales, or zToken buys if it takes market past High LTV limits
        // @dev - however, allow actions that make LTV better
        if (nextSqrtPriceX96 > marketState.lexState.lastSqrtPriceX96)
            _checkLTV(nextSqrtPriceX96, lexParams.limHighSqrtPriceX96);

        // Validate enough tokens in the market
        if (
            (assetIn != AssetType.BASE) &&
            ((isExactIn ? swapAmount : calcAmount) > marketState.supplyAmounts[(assetIn == AssetType.DEBT) ? 0 : 1])
        ) revert LSErrors.E_LEX_InsufficientTokens();

        // Validate nextSqrtPriceX96 did not pass lower bound (upper bound already checked above)
        // @dev - this can happen at the extreme of LTV -> 0% (no debt)
        if (nextSqrtPriceX96 < lexParams.edgeSqrtPriceX96_A) revert LSErrors.E_LEX_OperationNotAllowed();
    }

    // /// @dev - returns ratio (price) in WADs
    function _calcRatio(
        LexParams memory lexParams,
        LexFullState memory marketState,
        AssetType base,
        AssetType quote
    ) internal pure returns (uint256 price) {
        uint256 dexPrice;

        // Calculate price for converting one dex asset to another
        if (base != AssetType.BASE && quote != AssetType.BASE) {
            dexPrice = (base == AssetType.DEBT)
                ? Math.mulDiv(
                    marketState.lexState.lastSqrtPriceX96 * FixedPoint.WAD,
                    marketState.lexState.lastSqrtPriceX96,
                    FixedPoint.Q192
                )
                : ((FixedPoint.Q192 * FixedPoint.WAD) / marketState.lexState.lastSqrtPriceX96) /
                    marketState.lexState.lastSqrtPriceX96;
        } else {
            AssetType synthAsset = (base == AssetType.BASE) ? quote : base;
            // calculates BASE vs Synth price
            dexPrice = LatentMath.get_XvsL(
                marketState.lexState.lastSqrtPriceX96,
                lexParams.edgeSqrtPriceX96_A,
                lexParams.edgeSqrtPriceX96_B,
                synthAsset
            );
            if (quote == AssetType.BASE) dexPrice = (FixedPoint.WAD << FixedPoint.RESOLUTION) / dexPrice;
            else dexPrice = (FixedPoint.WAD * dexPrice) / FixedPoint.Q96;
        }

        // When calculating price between tokens, we must convert from dex prices to synth prices
        // ie dexToken1 = currentMarketPrice*dexToken0
        // => Token1amount * dexTokenRatio[LVRG] = currentMarketPrice * Token0amount * dexTokenRatio[DEBT]
        // => Token1amount / Token0amount = currentMarketPrice * dexTokenRatio[DEBT] / dexTokenRatio[LVRG]
        price = _dexToSynth(
            marketState,
            _synthToDex(marketState, dexPrice, base, Math.Rounding.Ceil),
            quote,
            Math.Rounding.Ceil
        );
    }

    // Local variables for _calculateMarketState to avoid stack too deep
    struct CalcMarketStateVars {
        uint256 elapsedTime;
        uint256 spotPriceDiscount;
        int256 spotLnRateBias;
        uint256 newDebtNotionalPrice;
        uint16 yieldFee;
        uint16 tvlFee;
        uint256 feeX96;
        uint256 yieldInBaseUnits;
        uint256 fee;
        uint256 invUpdateFactor;
        uint256 maxDebtValue;
    }

    // calculates current market state given market params, used by all other internal functions
    function _calculateMarketState(
        MarketParams calldata marketParams,
        LexParams memory lexParams,
        LexConfig storage lexConfig,
        LexState storage lexState,
        uint256 baseTokenSupply,
        bool isPreview
    ) internal view returns (LexFullState memory marketState) {
        CalcMarketStateVars memory vars;

        ////////////////////////////////////////////////////////////////////////////////
        // Read and cache variables
        marketState.lexState = lexState;
        marketState.lexConfig = lexConfig;
        marketState.baseTokenSupply = baseTokenSupply;

        ////////////////////////////////////////////////////////////////////////////////
        // Read external values
        // read synth token supplies (external call to trusted protocol)
        marketState.supplyAmounts[DEBT] = IERC20(marketState.lexConfig.zToken).totalSupply();
        marketState.supplyAmounts[LVRG] = IERC20(marketState.lexConfig.aToken).totalSupply();

        // get current baseToken market price from oracle and calculate liquidity ratio
        (marketState.lexState.lastBaseTokenPrice, marketState.liquidityRatioX96) = _readBasePriceAndCalculateLiqRatio(
            marketParams,
            lexParams.targetXvsL,
            marketState.lexConfig.scaleDecimals,
            isPreview
        );
        ////////////////////////////////////////////////////////////////////////////////
        // Execute time based accruals
        // update accrued yield + calc fees according to current blocktime + last market prices
        // update interest if the current block timestamp is greater than the last update timestamp
        if (block.timestamp > marketState.lexState.lastUpdateTimestamp) {
            vars.elapsedTime = block.timestamp - marketState.lexState.lastUpdateTimestamp;
            marketState.lexState.lastUpdateTimestamp = uint96(block.timestamp);

            // get debt price discount given last dex price. This is done on purpose, to accrue interest
            // based on past values (and time accrued with those values) vs current spot oracle and market values
            vars.spotPriceDiscount = _getDebtPriceDiscount(
                lexParams.edgeSqrtPriceX96_A,
                lexParams.edgeSqrtPriceX96_B,
                marketState.lexState.lastSqrtPriceX96,
                lexParams.targetXvsL
            );

            // if market price > _limMaxSqrtPriceX96 (ie, aToken selling is locked given high LTV)
            // this means market is undercollateralized or close to undercollateralized.  We thus activate a slowly increasing workout rate.
            // which creates ever higher positive interest rates as we get closer to the edge of the market
            // which also constitutes the underCollateralization event edge.
            // @dev - sqrtPrices have a max of uint104, so _squareUnsafe will not overflow.
            // @dev - WORKOUT_LN_RATE is negative
            vars.spotLnRateBias =
                int256(marketState.lexState.lastLnRateBias) -
                int256(
                    (marketState.lexState.lastSqrtPriceX96 <= lexParams.limMaxSqrtPriceX96)
                        ? 0
                        : Math.mulDiv(
                            uint256(lexParams.debtDuration) * NEG_WORKOUT_LN_RATE,
                            _squareUnsafe(marketState.lexState.lastSqrtPriceX96 - lexParams.limMaxSqrtPriceX96),
                            _squareUnsafe(lexParams.edgeSqrtPriceX96_B - lexParams.limMaxSqrtPriceX96)
                        )
                );

            // accrue interest (update cached debtNotionalPrice)
            // @dev limits elapsedTime for the update to a full duration interval.
            // @dev this limits interest accrual for inactive markets,
            // but ensures interest accrual does not overshoot given accrueInterest approximations
            // The maximum interest accrual below (in this update) is 1/price
            // (e.g., if price is 0.95 in a 12 month duration market, and market gets updated in 24 months... the update is still only 5.2%)
            vars.newDebtNotionalPrice = DebtMath.accrueInterest(
                marketState.lexState.lastDebtNotionalPrice,
                lexParams.debtDuration,
                vars.spotPriceDiscount,
                (vars.elapsedTime > lexParams.debtDuration) ? lexParams.debtDuration : vars.elapsedTime,
                vars.spotLnRateBias
            );

            // accrue fees
            // @dev - fee accruel linear instead of geometric to simplify math
            // large timesteps lead to underaccrual of protocol fees
            // @dev - do not accrue fees if LTV > MAX_LIMIT_LTV (high LTV or undercollateralized market)
            if (
                (marketState.lexConfig.protocolFee > 0) &&
                (marketState.lexState.lastSqrtPriceX96 <= lexParams.limMaxSqrtPriceX96)
            ) {
                // split out fees
                (vars.yieldFee, vars.tvlFee) = UtilsLib.decodeFee(marketState.lexConfig.protocolFee);

                vars.feeX96 = 0;
                if (vars.tvlFee > 0) {
                    // @dev - increase resolution of calculation to X96 to account for small fees, baseTokenSupplies or elapsedTime
                    vars.feeX96 = DebtMath.calculateLinearAccrual(
                        baseTokenSupply,
                        uint256(vars.tvlFee) << FixedPoint.RESOLUTION,
                        vars.elapsedTime
                    );
                }
                if ((vars.yieldFee > 0) && (vars.newDebtNotionalPrice > marketState.lexState.lastDebtNotionalPrice)) {
                    // calculate estimate of yield accrued in base units
                    // @dev - uses LTV instead of spot prices to lower gas cost
                    // this uses the avg market price instead of spot price for conversion,
                    // which leads to a higher yield estimation in low LTV environments vs high LTV environments
                    // this behavior is acceptable given gas savings.
                    vars.yieldInBaseUnits = marketState.baseTokenSupply.mulDiv(
                        (vars.newDebtNotionalPrice - marketState.lexState.lastDebtNotionalPrice) *
                            LatentMath.computeLTV(
                                lexParams.edgeSqrtPriceX96_A,
                                lexParams.edgeSqrtPriceX96_B,
                                marketState.lexState.lastSqrtPriceX96
                            ),
                        marketState.lexState.lastDebtNotionalPrice * FixedPoint.PERCENTAGE_FACTOR
                    );

                    vars.feeX96 += vars.yieldInBaseUnits.mulDiv(
                        uint256(vars.yieldFee) << FixedPoint.RESOLUTION,
                        FixedPoint.PERCENTAGE_FACTOR
                    );
                }

                vars.fee = vars.feeX96 / FixedPoint.Q96;

                // Probabilistically (best effort) add +1 fee depending on remainder for small fees (FIX from Pashov Audit)
                // @dev - this allows to probabilistically collect fees for low baseTokenSupplies, small fees, or small elapseTime
                // @dev -  We use the probabilistic approach to lower gas cost (avoid an SSTORE),
                // and only for fees < 100 given we are ok with a < 1% underaccrual of fees.
                if (vars.fee < 100) {
                    if (
                        (vars.feeX96 % FixedPoint.Q96) >
                        (uint256(
                            keccak256(
                                abi.encodePacked(
                                    uint32(block.prevrandao),
                                    uint32(block.timestamp),
                                    uint128(marketState.baseTokenSupply),
                                    uint64(marketState.lexState.lastSqrtPriceX96)
                                )
                            )
                        ) >> 160)
                    ) vars.fee += 1;
                } else if (vars.fee > type(uint96).max) vars.fee = type(uint96).max;
                if (vars.fee > (baseTokenSupply / 8)) vars.fee = baseTokenSupply / 8; // @dev set max update of 12.5% of baseTokenSupply

                unchecked {
                    marketState.accruedProtocolFee = uint96(vars.fee); //fits in uint96 given previous checks
                    marketState.baseTokenSupply -= vars.fee; // @dev - remove fee from baseTokenSupply for all calculations going fwd
                }
            }
            marketState.lexState.lastDebtNotionalPrice = vars.newDebtNotionalPrice;

            /////////////////////////////////////
            // Exponential TWAP
            // @dev - stores an exponential moving avg of market baseSupply

            // Calculate update factor with an approximate ETWAP_HALF_LIFE.
            vars.invUpdateFactor = DebtMath.calculateApproxExponentialUpdate(
                LN2,
                vars.elapsedTime,
                ETWAP_MIN_HALF_LIFE
            );

            marketState.lexState.lastETWAPBaseSupply =
                marketState.lexState.lastETWAPBaseSupply.mulDiv(FixedPoint.RAY, vars.invUpdateFactor) +
                marketState.baseTokenSupply.mulDiv(
                    FixedPoint.RAY - (FixedPoint.RAY * FixedPoint.RAY) / vars.invUpdateFactor,
                    FixedPoint.RAY
                );
        }

        //////////////////////////////////////////////////////////////////////////////////////
        // Calculate parameters for synth -> dex -> synth transforms (for BASE + DEBT only)
        // @dev - these scaled amounts seek to ensure liquidity < 2^152.
        // so, if liquidity is too big, I would make liquidityScaled ~ 2^152 = liquidity * X96 / divScaleFactorX96.
        // so, divScaleFactorX96 = liquidity * X96 / 2^152 = liquidityRatioX96 * BaseTokenSupply / 2^152;
        // @dev - saturates instead of reverting, meaning liquidity could still be > 2^152 even after applying this scaling.
        // This would only happen in markets where the oracle price * baseTokenSupply itself overflows, and thus an unlikely scenario.
        // if so, it will revert later when calculating liquidity.
        // @dev - if baseTokenSupply < X96, then use X96.  We assume all viable markets can price (without overflow a minimum of X96 base tokens)
        uint256 divScaleFactorX96 = marketState.liquidityRatioX96.saturatingMulShr(
            marketState.baseTokenSupply.max(FixedPoint.Q96),
            152
        );
        bool scaledLiquidity = divScaleFactorX96 > FixedPoint.Q96; // scaling only applies if divScaleFactorX96 > X96

        // if divScaleFactorX96 < X96, then liquidity already < 2^152.  liquidity = liquidityRatioX96 * BaseTokenSupply / X96
        // if divScaleFactorX96 > X96, then liquidityScaled = 2^152 = liquidityRatioX96 * BaseTokenSupply / divScaleFactorX96
        marketState.dexAmountsScaled[uint8(AssetType.BASE)] = marketState.liquidityRatioX96;
        marketState.synthAmountsScaled[uint8(AssetType.BASE)] = Math.ternary(
            scaledLiquidity,
            divScaleFactorX96,
            FixedPoint.Q96
        );

        // check that (notionalPrice * debtSupply * X96 / WAD / synthAmountScaled[Base]) does not overflow.
        // we have to use the synthAmountScaled[Base] across all assets to make the market consistent.
        // dex amount cannot be bigger than maxDebt * X96 / synthAmountScaled[Base] < notionalPrice * debtSupply * X96 / WAD / synthAmountScaled[Base]
        // thus, if maxDebt / notionalPrice < debtSupply / WAD, then market is undercollateralized.
        marketState.dexAmountsScaled[uint8(AssetType.DEBT)] = (scaledLiquidity &&
            marketState.lexState.lastDebtNotionalPrice < FixedPoint.Q160)
            ? (divScaleFactorX96 < FixedPoint.Q192)
                ? marketState.lexState.lastDebtNotionalPrice * FixedPoint.Q96
                : marketState.lexState.lastDebtNotionalPrice.mulDiv(FixedPoint.Q96, FixedPoint.WAD)
            : marketState.lexState.lastDebtNotionalPrice;

        marketState.synthAmountsScaled[uint8(AssetType.DEBT)] = scaledLiquidity
            ? (marketState.lexState.lastDebtNotionalPrice < FixedPoint.Q160)
                ? (divScaleFactorX96 < FixedPoint.Q192)
                    ? FixedPoint.WAD * divScaleFactorX96
                    : divScaleFactorX96
                : FixedPoint.WAD.mulDiv(divScaleFactorX96, FixedPoint.Q96)
            : FixedPoint.WAD;

        ////////////////////////////////////////////////////////////////////////////////
        // Calculate liquidity from baseTokenSupply
        marketState.liquidity = _synthToDex(
            marketState,
            marketState.baseTokenSupply,
            AssetType.BASE,
            Math.Rounding.Floor
        ).toUint160();

        ////////////////////////////////////////////////////////////////////////////////
        // calculate values if market has liquidity
        if (marketState.liquidity > 0) {
            // Calculate debt balanced value from zTokenSupply
            marketState.dexAmounts[DEBT] = _synthToDex(
                marketState,
                marketState.supplyAmounts[DEBT],
                AssetType.DEBT,
                Math.Rounding.Floor
            );

            // Check max value for debt, given availablie liquidity in market.
            vars.maxDebtValue = LatentMath.computeMaxDebt(
                lexParams.edgeSqrtPriceX96_A,
                lexParams.edgeSqrtPriceX96_B,
                marketState.liquidity
            );

            // if true, then system is undercollateralized (ie, debt notional value is above liquidity value)
            // if so, reduce debt value to be system liquidity value
            if (marketState.dexAmounts[DEBT] > vars.maxDebtValue) {
                marketState.lexState.lastSqrtPriceX96 = lexParams.edgeSqrtPriceX96_B;
                marketState.dexAmounts[DEBT] = vars.maxDebtValue;
                marketState.dexAmounts[LVRG] = 0;
                marketState.underCollateralized = true;
            } else {
                // calculate market price and aDexAmount given liquidity and zDexAmount
                // @dev - it could happen that marketState.dexAmounts[LVRG] == 0,
                // even if Liquidity > 0 and marketState.dexAmounts[DEBT] < maxDebtValue.
                (marketState.dexAmounts[LVRG], marketState.lexState.lastSqrtPriceX96) = LatentMath
                    .getMarketStateFromLiquidityAndDebt(
                        lexParams.edgeSqrtPriceX96_A,
                        lexParams.edgeSqrtPriceX96_B,
                        marketState.liquidity,
                        marketState.dexAmounts[DEBT]
                    );
            }
        } else {
            // If liquidity == 0, go through following scenarios to set price depending on whether
            // there is any debt or leverage tokens in the market, or whether it is a fully empty market.
            // @dev - any dust aTokens that might be left in the market is valueless.
            // @dev - any dust zTokens left might have some value (if baseTokenSupply > 0) but very small (under < 10-15 in quote tokens for most market setups)
            // but this undercollateralized state would not be resolved via workout accrual and might block the market,
            // so we consider them valueless as well.
            // Thus, we set the market price to uint160(FixedPoint.Q96) - uninitialized.
            marketState.dexAmounts[LVRG] = 0;
            marketState.dexAmounts[DEBT] = 0;
            marketState.lexState.lastSqrtPriceX96 = (marketState.supplyAmounts[LVRG] > 0 &&
                marketState.supplyAmounts[DEBT] == 0)
                ? lexParams.edgeSqrtPriceX96_A
                : uint160(FixedPoint.Q96);
        }

        //////////////////////////////////////////////////////////////////////////////////////
        // Calculate parameters for synth -> dex -> synth transforms (for LVRG only)
        // if no supply for leverage token, assume ratio == 1
        // if dexAmounts[LVRG] = 0 while supplyAmounts[LVRG] > 0, we will set dexAmountScaled[LVRG] = 1
        //  To ensure _synthToDex and _DexToSynth ratios work (and allow for LTV and cap checks)
        // This might happen when liquidity > 0 but very close to being undercollateralized (without triggering the flag),
        // or in markets where liquidity = 0 with leftover dust.
        marketState.dexAmountsScaled[uint8(AssetType.LEVERAGE)] = Math.ternary(
            marketState.supplyAmounts[LVRG] > 0,
            Math.max(marketState.dexAmounts[LVRG], 1),
            1
        );
        marketState.synthAmountsScaled[uint8(AssetType.LEVERAGE)] = Math.ternary(
            marketState.supplyAmounts[LVRG] > 0,
            marketState.supplyAmounts[LVRG],
            1
        );

        return marketState;
    }

    // @dev - returns price discount in WADs
    // ie. currentNotionalPrice = WAD indicates a price of 1 (ie, debt is trading at par)
    function _getDebtPriceDiscount(
        uint160 edgeSqrtPriceX96_A,
        uint160 edgeSqrtPriceX96_B,
        uint160 currentSqrtPriceX96,
        uint256 target_dXdL_X96
    ) internal pure returns (uint256 currentPriceDiscount) {
        // @dev - in undercollateralized case, will correctly price discount such that
        // the interest rate corresponds to a market that is 100% debt and 0% leverage,
        // ie, the max interest rate of the market
        uint256 current_dXdL_X96 = LatentMath.get_XvsL(
            currentSqrtPriceX96,
            edgeSqrtPriceX96_A,
            edgeSqrtPriceX96_B,
            AssetType.DEBT
        );

        return FixedPoint.WAD.mulDiv(target_dXdL_X96, current_dXdL_X96);
    }

    // checks market LTV and reverts if beyond bounds
    // @dev - perform after action that should be prohibited if LTVs are off limits
    function _checkLTV(uint160 nextSqrtPriceX96, uint160 sqrtPriceLimitX96) internal pure {
        if (nextSqrtPriceX96 > sqrtPriceLimitX96) revert LSErrors.E_LEX_ActionNotAllowedGivenLTVlimit();
    }

    // checks and reverts if in undercollateralized state
    function _checkUnderCollateralized(LexFullState memory marketState) internal pure {
        if (marketState.underCollateralized) revert LSErrors.E_LEX_ActionNotAllowedUnderCollateralized();
    }

    function _checkMintCap(
        uint256 marketBaseTokenSupply,
        uint256 eTWAPBaseTokenSupply,
        uint256 mintAmount,
        uint8 noCapLimit
    ) internal pure {
        // if marketBaseTokenSupply <= AMOUNT_NO_CAP, then we can mint upto 2^96.
        // ie, for small markets we can mint up 2^96 in one go (we assume markets can price correctly this amount of base tokens)
        // But for bigger markets, mintAmount <= marketBaseTokenSupply << (MAX_MINT_FACTOR_CAP - 1)
        // and marketBaseTokenSupply + mintAmount <= eTWAPBaseTokenSupply << MAX_MINT_FACTOR_CAP
        // For MaxMintFactorCap = 1, this means mintAmount <= marketBaseTokenSupply (ie, we can double the market size in one call)
        // as long as marketBaseTokenSupply + mintAmount <= eTWAPBaseTokenSupply << MAX_MINT_FACTOR_CAP.
        // in practice, a user can mint upto  and are approx. ~MAX_MINT_FACTOR_CAP/2 can be minted every 1hr
        // (this is based on ETWAP_MIN_HALF_LIFE and MAX_REDEEM_FACTOR_CAP values)
        unchecked {
            if (marketBaseTokenSupply <= (1 << noCapLimit)) {
                if ((mintAmount > FixedPoint.Q96) && (mintAmount > marketBaseTokenSupply)) {
                    revert LSErrors.E_LEX_MintCapExceeded();
                }
            } else if (
                (mintAmount > marketBaseTokenSupply) ||
                ((type(uint256).max - mintAmount) < marketBaseTokenSupply) ||
                ((eTWAPBaseTokenSupply < (type(uint256).max >> MAX_MINT_FACTOR_CAP)) &&
                    ((marketBaseTokenSupply + mintAmount) > (eTWAPBaseTokenSupply << MAX_MINT_FACTOR_CAP)))
            ) {
                revert LSErrors.E_LEX_MintCapExceeded();
            }
        }
    }

    function _checkRedeemCap(
        uint256 marketBaseTokenSupply,
        uint256 eTWAPBaseTokenSupply,
        uint256 redeemAmount,
        uint8 noCapLimit
    ) internal pure {
        // OK to redeem any amount if marketBaseTokenSupply < 10^noCapLimit
        // ie, for small markets there is no limit on how much can be redeemed.
        // But for bigger markets, approx. ~MAX_REDEEM_FACTOR_CAP/2 can be redeemed every 1hr
        // (this is based on ETWAP_MIN_HALF_LIFE and MAX_REDEEM_FACTOR_CAP values)
        unchecked {
            if (
                (marketBaseTokenSupply > (1 << noCapLimit)) &&
                (marketBaseTokenSupply > redeemAmount) &&
                ((marketBaseTokenSupply - redeemAmount) <
                    (eTWAPBaseTokenSupply - (eTWAPBaseTokenSupply >> MAX_REDEEM_FACTOR_CAP)))
            ) revert LSErrors.E_LEX_RedeemCapExceeded();
        }
    }

    function _checkSynthMintCap(uint256 synthSupplyAmount, uint256 mintAmount) internal pure {
        // ensure we are not miniting more than MAX_SYNTH_MINT_CAP for zToken and aToken
        // Block minting while still allowing market value to appreciate.
        if ((mintAmount > MAX_SYNTH_MINT_CAP) || ((MAX_SYNTH_MINT_CAP - mintAmount) < synthSupplyAmount))
            revert LSErrors.E_LEX_MarketSizeLimitExceeded();
    }

    // ------------------ Synth to Dex conversions ---------

    function _synthToDex(
        LexFullState memory marketState,
        uint256 synthAmount,
        AssetType assetType,
        Math.Rounding rounding
    ) internal pure returns (uint256 dexAmount) {
        return
            synthAmount.mulDiv(
                marketState.dexAmountsScaled[uint8(assetType)],
                marketState.synthAmountsScaled[uint8(assetType)],
                rounding
            );
    }

    function _dexToSynth(
        LexFullState memory marketState,
        uint256 dexAmount,
        AssetType assetType,
        Math.Rounding rounding
    ) internal pure returns (uint256 synthAmount) {
        // @dev - using saturatingMulDiv avoids overflows that are then captured downstream.
        // Specifically:
        // 1) in ExactIn cases with synth outputs (e.g., mint, swap base -> synth, swap synth -> synth),
        // if we saturate the amount of synth output, this is expected to be caught in the _checkSynthMintCap
        // check.
        // 2) in the ExactIn synth -> base swap, this would not saturate given we don't control the baseTokens in existence.
        // 3) in the ExactOut case with synth inputs (e.g. swap synth -> synth), it would seem saturating the amount of synth coming in
        // might create an issue (given more synths would be expected to come in given the synths going out).  however , we sould
        // recall we are saturating to the max synth tokens in existence 2^256-1, and thus we are removing all of one type of synth and
        // making the market be 100% the other type of synth.  In this situation, the market is correct.  We are eitehr 100% equity
        // LatentSwap DEX will correctly price the leverage token.  Or we are 100% debt (and the transaction will revert given LTV limits).
        return
            dexAmount.saturatingMulDiv(
                marketState.synthAmountsScaled[uint8(assetType)],
                marketState.dexAmountsScaled[uint8(assetType)],
                rounding
            );
    }

    // Retrieve baseToken price
    // @dev - baseTokenPrice is the value of 10^18 baseTokens, in quoteTokens, irrespective of actual # of decimal precision the baseToken has
    function _readBasePriceAndCalculateLiqRatio(
        MarketParams calldata marketParams,
        uint256 targetXvsL,
        int8 scaleDecimals,
        bool isPreview
    ) internal view returns (uint256 price, uint256 liqRatioX96) {
        // targetXvsL is also the liquidity concentration of the market, and used here when calculating the liquidityRatio
        // scaleDecimals ensures that final 'value' is in synth decimals (and not quote decimals)
        uint256 scaledLiquidityConcentrationX96 = (scaleDecimals > 0)
            ? FixedPoint.Q192 / (targetXvsL * (10 ** uint8(scaleDecimals)))
            : (FixedPoint.Q192 * (10 ** uint8(-scaleDecimals))) / targetXvsL;

        // liqRatioX96 represents the ratio transforming base token amounts to a concentrated value denominated liquidity
        // @dev - getQuote is such that it returns the # of quote tokens, given # of base tokens coming in (irrespective of actual decimal representation).
        liqRatioX96 = (isPreview)
            ? IPriceOracle(marketParams.curator).previewGetQuote(
                scaledLiquidityConcentrationX96,
                marketParams.baseToken,
                marketParams.quoteToken
            )
            : liqRatioX96 = IPriceOracle(marketParams.curator).getQuote(
            scaledLiquidityConcentrationX96,
            marketParams.baseToken,
            marketParams.quoteToken
        );

        if (liqRatioX96 < MIN_LIQRATIOX96) revert LSErrors.E_LEX_OraclePriceTooLowForMarket();

        // calculate price
        // @dev - price is the # of quote tokens given 10^18 base tokens ,
        // irrespective of actual # of decimal precision that baseToken or quoteToken has
        price = FixedPoint.WAD.mulDiv(liqRatioX96, scaledLiquidityConcentrationX96);
    }

    function _calculateTokenPrices(
        LexParams memory lexParams,
        LexFullState memory marketState
    ) internal pure returns (TokenPrices memory tokenPrices) {
        // @notice - all prices are # of quote tokens received for 10^18 of base, leverage, or debt tokens
        //(irrespective of actual decimal precision of each token type).
        tokenPrices.baseTokenPrice = marketState.lexState.lastBaseTokenPrice;

        // if market is undercollateralized and has leverage tokens, then leverage value is zero.
        // otherwise, calculate price of leverage token  given baseTokenPrice and leverage price in the Covenant market
        tokenPrices.aTokenPrice = (marketState.underCollateralized && marketState.dexAmounts[LVRG] > 0)
            ? 0
            : tokenPrices.baseTokenPrice.mulDiv(
                _calcRatio(lexParams, marketState, AssetType.LEVERAGE, AssetType.BASE),
                FixedPoint.WAD
            );

        // if market is undercollateralized and has debt, all base value is owned by debt.
        // otherwise, calculate price of debt token  given baseTokenPrice and debt price in the Covenant market
        tokenPrices.zTokenPrice = (marketState.underCollateralized && marketState.dexAmounts[DEBT] > 0)
            ? tokenPrices.baseTokenPrice.mulDiv(marketState.baseTokenSupply, marketState.supplyAmounts[DEBT])
            : tokenPrices.baseTokenPrice.mulDiv(
                _calcRatio(lexParams, marketState, AssetType.DEBT, AssetType.BASE),
                FixedPoint.WAD
            );
    }

    function _squareUnsafe(uint256 value) internal pure returns (uint256) {
        unchecked {
            return value * value;
        }
    }
}

// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.0;

import {AssetType, MintParams, RedeemParams, SwapParams, MarketId, MarketParams, TokenPrices, SynthTokens} from "./ICovenant.sol";

/**
 * @title ILiquidExchangeModel
 * @author Covenant Labs
 * @notice Defines the the core interface of Liquid Exchange Models
 **/
interface ILiquidExchangeModel {
    ///////////////////////////////////////////////////////////////////////////////
    // Getters

    /// @notice ProtocolFee getter
    function getProtocolFee(MarketId marketId) external view returns (uint32);

    /// @notice SynthTokens getter
    function getSynthTokens(MarketId marketId) external view returns (SynthTokens memory);

    /// @notice LEX name getter
    function name() external view returns (string memory);

    ///////////////////////////////////////////////////////////////////////////////
    // Write functions (only Covenant calls)

    /// @notice sets protocol Fee for a given market
    function setMarketProtocolFee(MarketId marketId, uint32 newFee) external;

    /// @notice initializes LEX variables for a market
    function initMarket(
        MarketId marketId,
        MarketParams calldata marketParams,
        uint32 protocolFee,
        bytes memory initData
    ) external returns (SynthTokens memory, bytes memory);

    /**
     * @notice calculate Synth tokens to mint given baseLiquidityIn, and updates internal states.
     * @notice does not include fees
     * @param mintParams covenant mint parameters
     * @param baseTokenSupply total baseToken supply in the market
     * @param sender sender of tokens coming in
     * @return aTokenAmountOut amount of aToken to be minted given amountIn
     * @return zTokenAmountOut amount of zToken to be minted given amountIn
     * @return protocolFees calculated protocol fees to be charged
     * @return tokenPrices prices of baseToken, aToken and zToken (in quote tokens) after mint
     **/
    function mint(
        MintParams calldata mintParams,
        address sender,
        uint256 baseTokenSupply
    )
        external
        payable
        returns (
            uint256 aTokenAmountOut,
            uint256 zTokenAmountOut,
            uint128 protocolFees,
            TokenPrices memory tokenPrices
        );

    /**
     * @notice calculates base liquidity out, given synth tokens redeemed, and updates internal states
     * @notice does not include fees
     * @notice Treats amounts as exact input, and does not check for slippage
     * @param redeemParams covenant redeem parameters
     * @param sender sender of tokens coming in
     * @param baseTokenSupply total baseToken supply in the market
     * @return amountOut amount of base token being redeemed
     * @return protocolFees calculated protocol fees to be charged
     * @return tokenPrices prices of baseToken, aToken and zToken (in quote tokens) after redeem
     **/
    function redeem(
        RedeemParams calldata redeemParams,
        address sender,
        uint256 baseTokenSupply
    ) external payable returns (uint256 amountOut, uint128 protocolFees, TokenPrices memory tokenPrices);

    /**
     * @notice calculates swap between tokens (base or synths), and updates internal states
     * @notice does not include fees
     * @dev All parameters are given in raw token decimal encoding.
     * @param swapParams covenant swap parameters
     * @param sender sender of tokens coming in
     * @param baseTokenSupply total baseToken supply in the market
     * @return amountCalculated amount of liquidity swapped out / in, depending on whether swap is EXACT_IN / EXACT_OUT
     * @return protocolFees calculated protocol fees to be charged
     * @return tokenPrices prices of baseToken, aToken and zToken (in quote tokens) after swap
     **/
    function swap(
        SwapParams calldata swapParams,
        address sender,
        uint256 baseTokenSupply
    ) external payable returns (uint256 amountCalculated, uint128 protocolFees, TokenPrices memory tokenPrices);

    /**
     * @notice Updates market state (e.g., accrues debt fees and protocol fees)
     * @dev Calling mint / redeem / swap also updates internal states, but updateState allows a user to update the state without mint / redeem /swapping tokens
     * @param marketId market to update
     * @param marketParams marketParams of market to update
     * @param baseTokenSupply total baseToken supply in the market
     * @param data additional data to send to LEX
     * @return protocolFees calculated protocol fees to be charged
     **/
    function updateState(
        MarketId marketId,
        MarketParams calldata marketParams,
        uint256 baseTokenSupply,
        bytes calldata data
    ) external payable returns (uint128 protocolFees);

    ///////////////////////////////////////////////////////////////////////////////
    // Quote functions (do not update internal state)

    /**
     * @notice calculate Synth tokens to mint given baseLiquidityIn
     * @notice does not include fees
     * @param mintParams covenant mint parameters
     * @param baseTokenSupply total baseToken supply in the market
     * @param sender sender of tokens coming in
     * @return aTokenAmountOut amount of aToken to be minted given amountIn
     * @return zTokenAmountOut amount of zToken to be minted given amountIn
     * @return protocolFees calculated protocol fees to be charged
     * @return oracleUpdateFee fees to pay as msgValue when calling mint() given mintParams.data package, if any
     * @return tokenPrices prices of baseToken, aToken and zToken (in quote tokens) after mint
     **/
    function quoteMint(
        MintParams calldata mintParams,
        address sender,
        uint256 baseTokenSupply
    )
        external
        view
        returns (
            uint256 aTokenAmountOut,
            uint256 zTokenAmountOut,
            uint128 protocolFees,
            uint128 oracleUpdateFee,
            TokenPrices memory tokenPrices
        );

    /**
     * @notice calculates base liquidity out, given synth tokens redeemed
     * @notice does not include fees
     * @notice Treats amounts as exact input, and does not check for slippage
     * @param redeemParams covenant redeem parameters
     * @param sender sender of tokens coming in
     * @param baseTokenSupply total baseToken supply in the market
     * @return baseAmountOut base tokens that would come out
     * @return protocolFees calculated protocol fees to be charged
     * @return oracleUpdateFee fees to pay as msgValue when calling mint() given mintParams.data package, if any
     * @return tokenPrices prices of baseToken, aToken and zToken (in quote tokens) after redeem
     **/
    function quoteRedeem(
        RedeemParams calldata redeemParams,
        address sender,
        uint256 baseTokenSupply
    )
        external
        view
        returns (uint256 baseAmountOut, uint128 protocolFees, uint128 oracleUpdateFee, TokenPrices memory tokenPrices);
    /**
     * @notice calculates swap between tokens (base or synths)
     * @notice does not include fees
     * @dev All parameters are given in raw token decimal encoding.
     * @param swapParams covenant swap parameters
     * @param sender sender of tokens coming in
     * @param baseTokenSupply total baseToken supply in the market
     * @return amountCalculated amount of liquidity swapped out / in, depending on whether swap is EXACT_IN / EXACT_OUT
     * @return protocolFees calculated protocol fees to be charged
     * @return oracleUpdateFee fees to pay as msgValue when calling mint() given mintParams.data package, if any
     * @return tokenPrices prices of baseToken, aToken and zToken (in quote tokens) after swap
     **/
    function quoteSwap(
        SwapParams calldata swapParams,
        address sender,
        uint256 baseTokenSupply
    )
        external
        view
        returns (
            uint256 amountCalculated,
            uint128 protocolFees,
            uint128 oracleUpdateFee,
            TokenPrices memory tokenPrices
        );
}

// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.0;

import {IERC20} from "./ISynthToken.sol";
import {ILiquidExchangeModel} from "./ILiquidExchangeModel.sol";
import {Events} from "../libraries/Events.sol";
import {Errors} from "../libraries/Errors.sol";

type MarketId is bytes20;

// Parameters that uniquely defines a Covenant market
struct MarketParams {
    address baseToken;
    address quoteToken;
    address curator; // address of the oracle router
    address lex;
}

struct SynthTokens {
    address aToken;
    address zToken;
}

struct MarketState {
    uint256 baseSupply; // total base tokens for market
    uint128 protocolFeeGrowth; // cumulative fee accrued by protocol in base tokens (unclaimed)
    address authorizedPauseAddress; // address authorized to pause market
    uint8 statusFlag; // 0 = uninitialized, 1 = unlocked, 2 = locked, 3 = paused
}

struct SwapParams {
    MarketId marketId;
    MarketParams marketParams;
    AssetType assetIn;
    AssetType assetOut;
    address to;
    uint256 amountSpecified;
    uint256 amountLimit;
    bool isExactIn;
    bytes data;
    uint256 msgValue;
}

struct RedeemParams {
    MarketId marketId;
    MarketParams marketParams;
    uint256 aTokenAmountIn;
    uint256 zTokenAmountIn;
    address to;
    uint256 minAmountOut;
    bytes data;
    uint256 msgValue;
}

struct MintParams {
    MarketId marketId;
    MarketParams marketParams;
    uint256 baseAmountIn;
    address to;
    uint256 minATokenAmountOut;
    uint256 minZTokenAmountOut;
    bytes data;
    uint256 msgValue;
}

struct TokenPrices {
    uint256 baseTokenPrice;
    uint256 aTokenPrice;
    uint256 zTokenPrice;
}

enum AssetType {
    BASE, // index 0
    DEBT, // index 1
    LEVERAGE, // index 2
    COUNT // used to get the count of asset types
}

/**
 * @title ICovenant
 * @author Covenant Labs
 * @notice Defines the the core interface of Covenant Liquid markets.
 **/
interface ICovenant {
    /// @notice Covenant name getter
    function name() external view returns (string memory);

    /// @notice MarketParams getter
    function getIdToMarketParams(MarketId marketId) external view returns (MarketParams memory);

    /// @notice MarketState getter
    function getMarketState(MarketId marketId) external view returns (MarketState memory);

    /// @notice Whether the LEX is enabled.
    function isLexEnabled(address lex) external view returns (bool);

    /// @notice Whether the Curator (oracle router) is enabled.
    function isCuratorEnabled(address curator) external view returns (bool);

    /**
     * @notice creates a new Covenant Liquid market
     * @param marketParams market initialization parameters
     **/
    function createMarket(MarketParams calldata marketParams, bytes calldata initData) external returns (MarketId);

    /**
     * @notice mints aTokens and zTokens from base tokens.
     * @param mintParams mint parameters, as detailed below:
     * - marketId: the marketId
     * - marketParams: the marketParams (can be derived from Id by caller using getIdToMarketParams)
     * - baseAmountIn: the amount of base token to deposit (and against which to mint a and z tokens)
     * - to: the receiver of aTokens and zTokens
     * - minATokenAmountOut: minimum ATokens out
     * - minZTokenAmountOut: minimum Ztokens out
     * - data: additional data to send to LEX
     * - msgValue: msgValue to send to LEX if needed
     * @return aTokenAmountOut amount of aToken minted
     * @return zTokenAmountOut amount of zToken minted
     **/
    function mint(
        MintParams calldata mintParams
    ) external payable returns (uint256 aTokenAmountOut, uint256 zTokenAmountOut);

    /**
     * @notice Redeems aTokenAmount and zTokenAmount for base token.
     * @notice Treats amounts as exact input, and does not check for slippage
     * @dev This function send to LEX msgValue, but does not check whether msg.Value == msgValue (this is done to enable multicalls)
     * @dev This means that calling with msgValue > msg.Value will revert, and msgValue < msg.Value
     * @dev will leave excess value in the Covenant contract (which can be used by subsequent function calls or users)
     * @param redeemParams redeem parameters, as follows:
     * - marketId: the marketId
     * - marketParams: the marketParams (can be derived from Id by caller using getIdToMarketParams)
     * - aTokenAmountIn: the aTokenAmount being redeemed / burned (exact in)
     * - zTokenAmountIn: the zTokenAmount being redeemed / burned (exact in)
     * - to: the receiver of base tokens
     * - minAmountOut: the minimum amount of base token out (for slippage / MEV protection)
     * - data: additional data to send to LEX
     * - msgValue: msgValue to send to LEX if needed
     * @return baseAmountOut actual base tokens redeemed
     **/
    function redeem(RedeemParams calldata redeemParams) external payable returns (uint256 baseAmountOut);

    /**
     * @notice Executes a swap between any of the base, aToken, or zToken assets
     * @dev All parameters are given in raw token decimal encoding.
     * @dev function returns error if assets being swapped are not part of the same market
     * @dev swapping between aTokens / zTokens actually mints / burns tokens
     * @dev This function send to LEX msgValue, but does not check whether msg.Value == msgValue (this is done to enable multicalls)
     * @dev This means that calling with msgValue > msg.Value will revert, and msgValue < msg.Value
     * @dev will leave excess value in the Covenant contract (which can be used by subsequent function calls or users)
     * @param swapParams swap parameters
     * - marketId: the marketId
     * - marketParams: the marketParams (can be derived from Id by caller using getIdToMarketParams)
     * - assetIn: AssetType in
     * - assetOut: AssetType out
     * - to: the receiver of base tokens
     * - amountSpecified: swap amount specified (amount in, if isExactIn == true)
     * - amountLimit: swap reverts if less than amountLimit is return (if isExactIn), or more than amountLimit is expected as input (if !isExactIn)
     * - isExactIn: whether swap is exact in, or exact out
     * - data: additional data to send to LEX
     * - msgValue: msgValue to send to LEX if needed
     * @return amount amount of tokens swapped out / in, depending on whether swap isExactIn
     **/
    function swap(SwapParams calldata swapParams) external payable returns (uint256 amount);

    /**
     * @notice Updates market state (e.g., accrues debt fees and protocol fees)
     * @dev Calling mint / redeem / swap also updates internal states, but updateState allows a user to update the state without mint / redeem /swapping tokens
     * @dev This function send to LEX msgValue, but does not check whether msg.Value == msgValue (this is done to enable multicalls)
     * @dev This means that calling with msgValue > msg.Value will revert, and msgValue < msg.Value
     * @dev will leave excess value in the Covenant contract (which can be used by subsequent function calls or users)
     * @param marketId market to update
     * @param marketParams marketParams of market to update
     * @param data additional data to send to LEX
     * @param msgValue msgValue to send to LEX if needed
     **/
    function updateState(
        MarketId marketId,
        MarketParams calldata marketParams,
        bytes calldata data,
        uint256 msgValue
    ) external payable;

    /**
     * @notice previews mint of aTokens and zTokens from base tokens, without changing market state
     * @notice Treats amounts as exact input, runs validation logic as actual mint call
     * @param mintParams mint parameters, as detailed below:
     * - marketId: the marketId
     * - marketParams: the marketParams (can be derived from Id by caller using getIdToMarketParams)
     * - baseAmountIn: the amount of base token to deposit (and against which to mint a and z tokens)
     * - to: the receiver of aTokens and zTokens
     * - minATokenAmountOut: minimum ATokens out
     * - minZTokenAmountOut: minimum Ztokens out
     * @return aTokenAmountOut amount of aToken minted
     * @return zTokenAmountOut amount of zToken minted
     * @return protocolFees amount of fee charged by protocol in base tokens
     * @return oracleUpdateFee fees to pay as msgValue when calling mint() given mintParams.data package, if any
     * @return tokenPrices returns the dex prices after the action
     **/
    function previewMint(
        MintParams calldata mintParams
    )
        external
        view
        returns (
            uint256 aTokenAmountOut,
            uint256 zTokenAmountOut,
            uint128 protocolFees,
            uint128 oracleUpdateFee,
            TokenPrices memory tokenPrices
        );

    /**
     * @notice previews redeem of aTokenAmount and zTokenAmount for base token, without changing market state.
     * @notice Treats amounts as exact input, runs validation logic as actual redeem call
     * @param redeemParams redeem parameters, as follows:
     * - marketId: the marketId
     * - marketParams: the marketParams (can be derived from Id by caller using getIdToMarketParams)
     * - aTokenAmountIn: the aTokenAmount being redeemed / burned (exact in)
     * - zTokenAmountIn: the zTokenAmount being redeemed / burned (exact in)
     * - to: the receiver of base tokens
     * - minAmountOut: the minimum amount of base token out (for slippage / MEV protection)
     * @return amountOut actual base tokens redeemed
     * @return protocolFees amount of fee charged by protocol in base tokens
     * @return oracleUpdateFee fees to pay as msgValue when calling mint() given mintParams.data package, if any
     * @return tokenPrices returns the dex prices after the action
     **/
    function previewRedeem(
        RedeemParams calldata redeemParams
    )
        external
        view
        returns (uint256 amountOut, uint128 protocolFees, uint128 oracleUpdateFee, TokenPrices memory tokenPrices);

    /**
     * @notice Calculates output of a swap between any of the base, aToken, or zToken assets, without changing market
     * @notice Runs validation logic as actual swap call
     * @param swapParams swap parameters
     * - marketId: the marketId
     * - marketParams: the marketParams (can be derived from Id by caller using getIdToMarketParams)
     * - assetIn: AssetType in
     * - assetOut: AssetType out
     * - to: the receiver of base tokens
     * - amountSpecified: swap amount specified (amount in, if isExactIn == true)
     * - amountLimit: swap reverts if less than amountLimit is return (if isExactIn), or more than amountLimit is expected as input (if !isExactIn)
     * - isExactIn: whether swap is exact in, or exact out
     * @return amountCalc amount of tokens swapped out / in, depending on whether swap is EXACT_IN / EXACT_OUT
     * @return protocolFees amount of fee charged by protocol in base tokens
     * @return oracleUpdateFee fees to pay as msgValue when calling mint() given mintParams.data package, if any
     * @return tokenPrices returns the dex prices after the action
     **/
    function previewSwap(
        SwapParams calldata swapParams
    )
        external
        view
        returns (uint256 amountCalc, uint128 protocolFees, uint128 oracleUpdateFee, TokenPrices memory tokenPrices);

    /**
     * @notice Payable multicall
     * @notice Does not check msg.value received.  Instead, it uses any msgValues encoded in data and sends those onwards
     * @notice This means that calling multicall where sum(data(msgValues)) > msg.Value will revert, and
     * @notice sum(data(msgValues)) < msg.Value will leave excess value in the Covenant contract (which can be used by subsequent users)
     * @param data array of call data
     * @return results an array of return info
     */
    function multicall(bytes[] calldata data) external payable returns (bytes[] memory results);

    /////////////////////////////////////////////////////////////////////////////////
    // Restricted functions

    /// @notice Set valid LEX contracts (onlyOwner)
    /// @notice Disabling a LEX does not allow new markets with this LEX
    /// but does not invalidate already created markets
    function setEnabledLEX(address lex, bool isValid) external;

    /// @notice Set valid Curator (oracle router) contracts (onlyOwner)
    /// @notice Disabling a Curator does not allow new markets with this Curator
    /// but does not invalidate already created markets
    function setEnabledCurator(address curator, bool isValid) external;

    /// @notice Set default protocol fee (onlyOwner)
    function setDefaultFee(uint32 newFee) external;

    /// @notice Set protocol fee for a market (onlyOwner)
    function setMarketProtocolFee(
        MarketId marketId,
        MarketParams calldata marketParams,
        bytes calldata data,
        uint256 msgValue,
        uint32 newFee
    ) external payable;

    /// @notice Collect protocol fees for a market (onlyOwner)
    function collectProtocolFee(MarketId marketId, address recipient, uint128 amountRequested) external;

    /// @notice Pause a market (only authorized pause address)
    function setMarketPause(MarketId marketId, bool isPaused) external;

    /// @notice Set default pause address (onlyOwner)
    function setDefaultPauseAddress(address newPauseAddress) external;

    /// @notice Set pause address for a market (onlyOwner)
    function setMarketPauseAddress(MarketId marketId, address newPauseAddress) external;
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.30;

import {Math} from "@openzeppelin/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/utils/math/SafeCast.sol";

/**
 * @title SaturatingMath library
 * @author Covenant Labs
 * @notice Provides a saturating mulDiv operation
 */
library SaturatingMath {
    // Returns a saturating mulDiv operation
    // @dev - does not overflow, but instead returns type(uint256).max if so.
    function saturatingMulDiv(
        uint256 _numerator1,
        uint256 _numerator2,
        uint256 _denominator
    ) internal pure returns (uint256) {
        (uint256 high, uint256 low) = Math.mul512(_numerator1, _numerator2);

        // @dev - below follows the logic of Math.mulDiv, but saturates instead of reverting.
        if (high >= _denominator) {
            // returns type(uint256).max for all overflow and _denominator == 0 conditions
            return type(uint256).max;
        } else if (high == 0) {
            // @dev - execute 256 bit division here directly.
            // already checked for denominator == 0
            unchecked {
                return low / _denominator;
            }
        } else {
            // @dev - would be more efficient to do a 512 division here,
            // but OpenZeppelin does not have a separate (already audited) function.
            // So below recomputes Math.mul512 internally, and then performs the division.
            // Does not revert given checks above.
            return Math.mulDiv(_numerator1, _numerator2, _denominator);
        }
    }

    function saturatingMulDiv(
        uint256 x,
        uint256 y,
        uint256 denominator,
        Math.Rounding rounding
    ) internal pure returns (uint256 result) {
        result = saturatingMulDiv(x, y, denominator);
        return
            result +
            SafeCast.toUint(
                Math.unsignedRoundsUp(rounding) && mulmod(x, y, denominator) > 0 && result < type(uint256).max
            );
    }

    /**
     * @dev Calculates floor(x * y >> n) with full precision. saturates instead of reverting.
     * @dev Code copies @openzeppelin/utils/math/Math.sol:mulShr, but saturates instead of reverting.
     */
    function saturatingMulShr(uint256 x, uint256 y, uint8 n) internal pure returns (uint256 result) {
        unchecked {
            (uint256 high, uint256 low) = Math.mul512(x, y);
            if (high >= 1 << n) {
                return type(uint256).max; // @dev - saturates instead of reverting for overflow.
            }
            return (high << (256 - n)) | (low >> n);
        }
    }
}

// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.0;

import {AssetType, MintParams, RedeemParams, SwapParams, MarketId, MarketParams, TokenPrices, SynthTokens} from "./ICovenant.sol";

/**
 * @title ILiquidExchangeModel
 * @author Covenant Labs
 * @notice Defines the the core interface of Liquid Exchange Models
 **/
interface ILiquidExchangeModel {
    ///////////////////////////////////////////////////////////////////////////////
    // Getters

    /// @notice ProtocolFee getter
    function getProtocolFee(MarketId marketId) external view returns (uint32);

    /// @notice SynthTokens getter
    function getSynthTokens(MarketId marketId) external view returns (SynthTokens memory);

    /// @notice LEX name getter
    function name() external view returns (string memory);

    ///////////////////////////////////////////////////////////////////////////////
    // Write functions (only Covenant calls)

    /// @notice sets protocol Fee for a given market
    function setMarketProtocolFee(MarketId marketId, uint32 newFee) external;

    /// @notice initializes LEX variables for a market
    function initMarket(
        MarketId marketId,
        MarketParams calldata marketParams,
        uint32 protocolFee,
        bytes memory initData
    ) external returns (SynthTokens memory, bytes memory);

    /**
     * @notice calculate Synth tokens to mint given baseLiquidityIn, and updates internal states.
     * @notice does not include fees
     * @param mintParams covenant mint parameters
     * @param baseTokenSupply total baseToken supply in the market
     * @param sender sender of tokens coming in
     * @return aTokenAmountOut amount of aToken to be minted given amountIn
     * @return zTokenAmountOut amount of zToken to be minted given amountIn
     * @return protocolFees calculated protocol fees to be charged
     * @return tokenPrices prices of baseToken, aToken and zToken (in quote tokens) after mint
     **/
    function mint(
        MintParams calldata mintParams,
        address sender,
        uint256 baseTokenSupply
    )
        external
        payable
        returns (
            uint256 aTokenAmountOut,
            uint256 zTokenAmountOut,
            uint128 protocolFees,
            TokenPrices memory tokenPrices
        );

    /**
     * @notice calculates base liquidity out, given synth tokens redeemed, and updates internal states
     * @notice does not include fees
     * @notice Treats amounts as exact input, and does not check for slippage
     * @param redeemParams covenant redeem parameters
     * @param sender sender of tokens coming in
     * @param baseTokenSupply total baseToken supply in the market
     * @return amountOut amount of base token being redeemed
     * @return protocolFees calculated protocol fees to be charged
     * @return tokenPrices prices of baseToken, aToken and zToken (in quote tokens) after redeem
     **/
    function redeem(
        RedeemParams calldata redeemParams,
        address sender,
        uint256 baseTokenSupply
    ) external payable returns (uint256 amountOut, uint128 protocolFees, TokenPrices memory tokenPrices);

    /**
     * @notice calculates swap between tokens (base or synths), and updates internal states
     * @notice does not include fees
     * @dev All parameters are given in raw token decimal encoding.
     * @param swapParams covenant swap parameters
     * @param sender sender of tokens coming in
     * @param baseTokenSupply total baseToken supply in the market
     * @return amountCalculated amount of liquidity swapped out / in, depending on whether swap is EXACT_IN / EXACT_OUT
     * @return protocolFees calculated protocol fees to be charged
     * @return tokenPrices prices of baseToken, aToken and zToken (in quote tokens) after swap
     **/
    function swap(
        SwapParams calldata swapParams,
        address sender,
        uint256 baseTokenSupply
    ) external payable returns (uint256 amountCalculated, uint128 protocolFees, TokenPrices memory tokenPrices);

    /**
     * @notice Updates market state (e.g., accrues debt fees and protocol fees)
     * @dev Calling mint / redeem / swap also updates internal states, but updateState allows a user to update the state without mint / redeem /swapping tokens
     * @param marketId market to update
     * @param marketParams marketParams of market to update
     * @param baseTokenSupply total baseToken supply in the market
     * @param data additional data to send to LEX
     * @return protocolFees calculated protocol fees to be charged
     **/
    function updateState(
        MarketId marketId,
        MarketParams calldata marketParams,
        uint256 baseTokenSupply,
        bytes calldata data
    ) external payable returns (uint128 protocolFees);

    ///////////////////////////////////////////////////////////////////////////////
    // Quote functions (do not update internal state)

    /**
     * @notice calculate Synth tokens to mint given baseLiquidityIn
     * @notice does not include fees
     * @param mintParams covenant mint parameters
     * @param baseTokenSupply total baseToken supply in the market
     * @param sender sender of tokens coming in
     * @return aTokenAmountOut amount of aToken to be minted given amountIn
     * @return zTokenAmountOut amount of zToken to be minted given amountIn
     * @return protocolFees calculated protocol fees to be charged
     * @return oracleUpdateFee fees to pay as msgValue when calling mint() given mintParams.data package, if any
     * @return tokenPrices prices of baseToken, aToken and zToken (in quote tokens) after mint
     **/
    function quoteMint(
        MintParams calldata mintParams,
        address sender,
        uint256 baseTokenSupply
    )
        external
        view
        returns (
            uint256 aTokenAmountOut,
            uint256 zTokenAmountOut,
            uint128 protocolFees,
            uint128 oracleUpdateFee,
            TokenPrices memory tokenPrices
        );

    /**
     * @notice calculates base liquidity out, given synth tokens redeemed
     * @notice does not include fees
     * @notice Treats amounts as exact input, and does not check for slippage
     * @param redeemParams covenant redeem parameters
     * @param sender sender of tokens coming in
     * @param baseTokenSupply total baseToken supply in the market
     * @return baseAmountOut base tokens that would come out
     * @return protocolFees calculated protocol fees to be charged
     * @return oracleUpdateFee fees to pay as msgValue when calling mint() given mintParams.data package, if any
     * @return tokenPrices prices of baseToken, aToken and zToken (in quote tokens) after redeem
     **/
    function quoteRedeem(
        RedeemParams calldata redeemParams,
        address sender,
        uint256 baseTokenSupply
    )
        external
        view
        returns (uint256 baseAmountOut, uint128 protocolFees, uint128 oracleUpdateFee, TokenPrices memory tokenPrices);
    /**
     * @notice calculates swap between tokens (base or synths)
     * @notice does not include fees
     * @dev All parameters are given in raw token decimal encoding.
     * @param swapParams covenant swap parameters
     * @param sender sender of tokens coming in
     * @param baseTokenSupply total baseToken supply in the market
     * @return amountCalculated amount of liquidity swapped out / in, depending on whether swap is EXACT_IN / EXACT_OUT
     * @return protocolFees calculated protocol fees to be charged
     * @return oracleUpdateFee fees to pay as msgValue when calling mint() given mintParams.data package, if any
     * @return tokenPrices prices of baseToken, aToken and zToken (in quote tokens) after swap
     **/
    function quoteSwap(
        SwapParams calldata swapParams,
        address sender,
        uint256 baseTokenSupply
    )
        external
        view
        returns (
            uint256 amountCalculated,
            uint128 protocolFees,
            uint128 oracleUpdateFee,
            TokenPrices memory tokenPrices
        );
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.4.0;

/// @title FixedPoint
/// @notice A library for handling binary fixed point numbers, see https://en.wikipedia.org/wiki/Q_(number_format)
library FixedPoint {
    // Q96 Fixed Point Constants
    uint8 internal constant RESOLUTION = 96;
    uint256 internal constant Q96 = 0x1000000000000000000000000;
    uint256 internal constant Q128 = 0x100000000000000000000000000000000;
    uint256 internal constant Q160 = 0x0010000000000000000000000000000000000000000;
    uint256 internal constant Q192 = 0x1000000000000000000000000000000000000000000000000;

    // WAD Fixed Point Constants
    uint8 internal constant RESOLUTION_WAD = 18;
    uint256 internal constant WAD = 1e18;
    uint256 internal constant HALF_WAD = 0.5e18;

    // RAY Fixed Point Constants
    uint8 internal constant RESOLUTION_RAY = 27;
    uint256 internal constant RAY = 1e27;
    uint256 internal constant HALF_RAY = 0.5e27;
    uint256 internal constant WAD_RAY_RATIO = 1e9;

    // Perecentage Math Constants
    uint256 internal constant PERCENTAGE_FACTOR = 1e4;
    uint256 internal constant HALF_PERCENTAGE_FACTOR = 0.5e4;
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import {SafeMetadata, IERC20} from "../../../libraries/SafeMetadata.sol";
import {ITokenData} from "../interfaces/ITokenData.sol";

/// @title TokenData
/// @author Covenant Labs
/// @notice sets symbol, decimals and name overrides for a token
/// @dev each item can be set independently, and will override existing ERC20 values for the respecitve token
/// @dev if both symbol and decimals are overriden, a quote token need not be an actual ERC20
/// @dev this gives the flexibility to use currency ISO addresses and symbols for quote tokens.
/// @dev Oracles can use ERC-7535, ISO 4217 or other conventions to represent non-ERC20 assets as addresses.
/// @dev e.g., EIP7528 would set address = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE", symbol = "ETH", decimals = 18.
abstract contract TokenData is ITokenData {
    using SafeMetadata for IERC20;

    // mapping of decimal overrides for specific assets
    mapping(address => uint8) _decimals;

    // mapping of symbol overrides for specific assets
    mapping(address => string) _symbol;

    // mapping of name overrides for specific assets
    mapping(address => string) _name;

    error TokenData_InvalidDecimals();

    event SetTokenDecimals(address indexed token, uint8 oldDecimals, uint8 newDecimals);
    event SetTokenSymbol(address indexed token, string oldSymbol, string newSymbol);
    event SetTokenName(address indexed token, string oldName, string newName);

    function assetDecimals(address asset) public view returns (uint8 decimals_) {
        return _assetDecimals(asset);
    }

    function assetSymbol(address asset) public view returns (string memory symbol_) {
        return _assetSymbol(asset);
    }

    function assetName(address asset) public view returns (string memory name_) {
        return _assetName(asset);
    }

    //////////////////////////////////////////////////////////////////////////

    // @dev - Stored decimals are an override.
    // if stored decimals is 0, try and get ERC20 decimals
    function _assetDecimals(address asset) internal view returns (uint8 decimals_) {
        decimals_ = _decimals[asset]; //check if there is an override for this asset
        if (decimals_ > 0) return decimals_;
        else {
            // try and read from asset itself
            bool success;
            (success, decimals_) = IERC20(asset).tryGetDecimals();
            return success ? decimals_ : 18;
        }
    }

    // @dev - Stored symbol are an override.
    // @dev - if stored symbol is "", try and get ERC20 symbol
    function _assetSymbol(address asset) internal view returns (string memory symbol_) {
        symbol_ = _symbol[asset]; //check if there is an override for this asset
        if (bytes(symbol_).length > 0) return symbol_;
        else {
            // try and read from asset itself
            bool success;
            (success, symbol_) = IERC20(asset).tryGetSymbol();
            return success ? symbol_ : "";
        }
    }

    // @dev - Stored name are an override.
    // @dev - if stored name is "", try and get ERC20 name
    function _assetName(address asset) internal view returns (string memory name_) {
        name_ = _name[asset]; //check if there is an override for this asset
        if (bytes(name_).length > 0) return name_;
        else {
            // try and read from asset itself
            bool success;
            (success, name_) = IERC20(asset).tryGetName();
            return success ? name_ : "";
        }
    }

    // internal functions.  These should be exposed with the appropriate access modifiers
    // @dev - if newDecimals = 0, then _assetDecimals will try and get ERC20 decimals
    function _updateAssetDecimals(address asset, uint8 newDecimals) internal {
        if (newDecimals > 18) revert TokenData_InvalidDecimals();
        uint8 oldDecimals = _assetDecimals(asset); // get old decimals
        _decimals[asset] = newDecimals;
        emit SetTokenDecimals(asset, oldDecimals, newDecimals);
    }

    // internal functions.  These should be exposed with the appropriate access modifiers
    // @dev - if newSymbol = "", then _assetSymbol will try and get ERC20 symbol
    function _updateAssetSymbol(address asset, string calldata newSymbol) internal {
        string memory oldSymbol = _assetSymbol(asset); // get old symbol
        _symbol[asset] = newSymbol;
        emit SetTokenSymbol(asset, oldSymbol, newSymbol);
    }

    // internal functions.  These should be exposed with the appropriate access modifiers
    // @dev - if newName = "", then _assetNAme will try and get ERC20 name
    function _updateAssetName(address asset, string calldata newName) internal {
        string memory oldName = _assetName(asset); // get old symbol
        _name[asset] = newName;
        emit SetTokenName(asset, oldName, newName);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.30;

/// @title Utils Library
/// @author Covenant Labs
/// @notice Library to convert a market to its id.
library UtilsLib {
    function encodeFee(uint16 yieldFee, uint16 tvlFee) internal pure returns (uint32 protocolFee) {
        return ((uint32(yieldFee) << 16) | uint32(tvlFee));
    }

    function decodeFee(uint32 protocolFee) internal pure returns (uint16 yieldFee, uint16 tvlFee) {
        yieldFee = uint16(protocolFee >> 16);
        tvlFee = uint16(protocolFee & 0xFFFF);
    }
}

// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.30;

library LSErrors {
    error E_LEX_OnlyCovenantCanCall(); // 0x1150f470
    error E_LEX_ZeroAddress(); // 0xad5292ae
    error E_LEX_ZeroLiquidity(); // 0xb38d3cff
    error E_LEX_AlreadyInitialized(); // 0xbb304191
    error E_LEX_IncorrectInitializationPrice(); // 0xc868f36a
    error E_LEX_IncorrectInitializationLnRateBias(); // 0x4cf570ef
    error E_LEX_InsufficientTokens(); // 0x6cf03401
    error E_LEX_ActionNotAllowedGivenLTVlimit(); // 0x2ab66638
    error E_LEX_ActionNotAllowedUnderCollateralized(); // 0xabc1fa28
    error E_LEX_OperationNotAllowed(); // 0x82a547cd
    error E_LEX_RedeemCapExceeded(); // 0xef9b092a
    error E_LEX_MintCapExceeded(); // 0x6c8de6e1
    error E_LEX_OraclePriceTooLowForMarket(); // 0x417969ec
    error E_LEX_IncorrectInitializationDuration(); // 0x50560b48
    error E_LEX_BaseAssetNotERC20(); // 0x77e6fe18
    error E_LEX_QuoteAssetHasNoSymbol(); // 0xd5862a35
    error E_LEX_MarketDoesNotExist(); // 0xda47482e
    error E_LEX_Overdeposit(); // 0x94f85219
    error E_LEX_InsufficientAmount(); // 0xe869f1da
    error E_LEX_MarketSizeLimitExceeded(); // 0x23565a11
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.30;

import {Math} from "@openzeppelin/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/utils/math/SafeCast.sol";
import {FixedPoint} from "./FixedPoint.sol";

/// @title Functions based on Q64.96 sqrt price and liquidity
/// @notice Contains the math that uses square root of price as a Q64.96 and liquidity to compute deltas
library SqrtPriceMath {
    using SafeCast for uint256;

    /// @notice Gets the next sqrt price given a delta of token0
    /// The most precise formula for this is liquidity * sqrtPX96 / (liquidity +- amount * sqrtPX96),
    /// if this is impossible because of overflow, we calculate liquidity / (liquidity / sqrtPX96 +- amount).
    /// @param sqrtPX96 The starting price, i.e. before accounting for the token0 delta
    /// @param liquidity The amount of usable liquidity
    /// @param amount How much of token0 to add or remove from virtual reserves
    /// @param add Whether to add or remove the amount of token0
    /// @param rounding Whether to round result up or down
    /// @return The price after adding or removing amount, depending on add
    function getNextSqrtPriceFromAmount0(
        uint160 sqrtPX96,
        uint160 liquidity,
        uint256 amount,
        bool add,
        Math.Rounding rounding
    ) internal pure returns (uint160) {
        require(sqrtPX96 > 0);
        require(liquidity > 0);

        // we short circuit amount == 0 because the result is otherwise not guaranteed to equal the input price
        if (amount == 0) return sqrtPX96;
        uint256 numerator1 = uint256(liquidity) << FixedPoint.RESOLUTION;

        if (add) {
            unchecked {
                uint256 product;
                if ((product = amount * sqrtPX96) / amount == sqrtPX96) {
                    uint256 denominator = numerator1 + product;
                    if (denominator >= numerator1)
                        // always fits in 160 bits
                        return uint160(Math.mulDiv(numerator1, sqrtPX96, denominator, rounding));
                }
            }
            // denominator is checked for overflow
            uint256 denominator2 = (numerator1 / sqrtPX96) + amount;
            if (rounding == Math.Rounding.Ceil) return uint160(Math.ceilDiv(numerator1, denominator2));
            else return uint160(numerator1 / denominator2);
        } else {
            unchecked {
                uint256 product;
                // if the product overflows, we know the denominator underflows
                // in addition, we must check that the denominator does not underflow
                require((product = amount * sqrtPX96) / amount == sqrtPX96 && numerator1 > product);
                uint256 denominator = numerator1 - product;
                return Math.mulDiv(numerator1, sqrtPX96, denominator, rounding).toUint160();
            }
        }
    }

    /// @notice Gets the next sqrt price given a delta of token1
    /// The formula we compute is within <1 wei of the lossless version: sqrtPX96 +- amount / liquidity
    /// @param sqrtPX96 The starting price, i.e., before accounting for the token1 delta
    /// @param liquidity The amount of usable liquidity
    /// @param amount How much of token1 to add, or remove, from virtual reserves
    /// @param add Whether to add, or remove, the amount of token1
    /// @param rounding Whether to round result up or down
    /// @return The price after adding or removing `amount`
    function getNextSqrtPriceFromAmount1(
        uint160 sqrtPX96,
        uint160 liquidity,
        uint256 amount,
        bool add,
        Math.Rounding rounding
    ) internal pure returns (uint160) {
        require(sqrtPX96 > 0);
        require(liquidity > 0);

        // if we're adding (subtracting), rounding down requires rounding the quotient down (up)
        // in both cases, avoid a mulDiv for most inputs
        if (add) {
            uint256 quotient = (
                amount <= type(uint160).max
                    ? (
                        (rounding == Math.Rounding.Ceil)
                            ? Math.ceilDiv((amount << FixedPoint.RESOLUTION), liquidity)
                            : (amount << FixedPoint.RESOLUTION) / liquidity
                    )
                    : Math.mulDiv(amount, FixedPoint.Q96, liquidity, rounding)
            );

            return (uint256(sqrtPX96) + quotient).toUint160();
        } else {
            Math.Rounding invRounding = Math.Rounding(1 - uint8(rounding));
            uint256 quotient = (
                amount <= type(uint160).max
                    ? (invRounding == Math.Rounding.Ceil)
                        ? Math.ceilDiv(amount << FixedPoint.RESOLUTION, liquidity)
                        : ((amount << FixedPoint.RESOLUTION) / liquidity)
                    : Math.mulDiv(amount, FixedPoint.Q96, liquidity, invRounding)
            );

            require(sqrtPX96 > quotient);
            // always fits 160 bits
            unchecked {
                return uint160(sqrtPX96 - quotient);
            }
        }
    }

    /// @notice Gets the amount0 delta between two prices
    /// @dev Calculates liquidity / sqrt(lower) - liquidity / sqrt(upper),
    /// i.e. liquidity * (sqrt(upper) - sqrt(lower)) / (sqrt(upper) * sqrt(lower))
    /// @param sqrtRatioAX96 A sqrt price
    /// @param sqrtRatioBX96 Another sqrt price
    /// @param liquidity The amount of usable liquidity
    /// @param rounding Whether to round the amount up or down
    /// @return amount0 Amount of token0 required to cover a position of size liquidity between the two passed prices
    function getAmount0Delta(
        uint160 sqrtRatioAX96,
        uint160 sqrtRatioBX96,
        uint160 liquidity,
        Math.Rounding rounding
    ) internal pure returns (uint256 amount0) {
        unchecked {
            if (sqrtRatioAX96 > sqrtRatioBX96) (sqrtRatioAX96, sqrtRatioBX96) = (sqrtRatioBX96, sqrtRatioAX96);

            uint256 numerator1 = uint256(liquidity) << FixedPoint.RESOLUTION;
            uint256 numerator2 = sqrtRatioBX96 - sqrtRatioAX96;

            require(sqrtRatioAX96 > 0);

            uint256 numerator3 = Math.mulDiv(numerator1, numerator2, sqrtRatioBX96, rounding);
            return
                (rounding == Math.Rounding.Ceil) ? Math.ceilDiv(numerator3, sqrtRatioAX96) : numerator3 / sqrtRatioAX96;
        }
    }

    /// @notice Gets the amount1 delta between two prices
    /// @dev Calculates liquidity * (sqrt(upper) - sqrt(lower))
    /// @param sqrtRatioAX96 A sqrt price
    /// @param sqrtRatioBX96 Another sqrt price
    /// @param liquidity The amount of usable liquidity
    /// @param rounding Whether to round the amount up, or down
    /// @return amount1 Amount of token1 required to cover a position of size liquidity between the two passed prices
    function getAmount1Delta(
        uint160 sqrtRatioAX96,
        uint160 sqrtRatioBX96,
        uint160 liquidity,
        Math.Rounding rounding
    ) internal pure returns (uint256 amount1) {
        unchecked {
            if (sqrtRatioAX96 > sqrtRatioBX96) (sqrtRatioAX96, sqrtRatioBX96) = (sqrtRatioBX96, sqrtRatioAX96);
            return Math.mulDiv(liquidity, sqrtRatioBX96 - sqrtRatioAX96, FixedPoint.Q96, rounding);
        }
    }
}

// SPDX-License-Identifier: GPLv3
pragma solidity ^0.8.30;

import {Math} from "@openzeppelin/utils/math/Math.sol";

// Code developed by https://github.com/SimonSuckut/Solidity_Uint512/

library Uint512 {
    /// @notice Calculates the difference of two uint512 (a - b)
    /// @dev Does not revert on underflow (ie, does not revert if b > a)
    /// @param a0 A uint256 representing the lower bits of the minuend.
    /// @param a1 A uint256 representing the higher bits of the minuend.
    /// @param b0 A uint256 representing the lower bits of the subtrahend.
    /// @param b1 A uint256 representing the higher bits of the subtrahend.
    /// @return r0 The result as an uint512. r0 contains the lower bits.
    /// @return r1 The higher bits of the result.
    function sub512x512(uint256 a0, uint256 a1, uint256 b0, uint256 b1) public pure returns (uint256 r0, uint256 r1) {
        assembly {
            r0 := sub(a0, b0)
            r1 := sub(sub(a1, b1), lt(a0, b0))
        }
    }

    /// @notice Calculates the square root of a 512 bit unsigned integer, rounding down.
    /// @dev Uses the Karatsuba Square Root method. See https://hal.inria.fr/inria-00072854/document for details.
    /// @param a0 A uint256 representing the low bits of the input.
    /// @param a1 A uint256 representing the high bits of the input.
    /// @return s The square root as an uint256. Result has at most 256 bit.
    function sqrt512(uint256 a0, uint256 a1) public pure returns (uint256 s) {
        // A simple 256 bit square root is sufficient
        if (a1 == 0) return Math.sqrt(a0);

        // The used algorithm has the pre-condition a1 >= 2**254
        uint256 shift;

        assembly {
            let digits := mul(lt(a1, 0x100000000000000000000000000000000), 128)
            a1 := shl(digits, a1)
            shift := add(shift, digits)

            digits := mul(lt(a1, 0x1000000000000000000000000000000000000000000000000), 64)
            a1 := shl(digits, a1)
            shift := add(shift, digits)

            digits := mul(lt(a1, 0x100000000000000000000000000000000000000000000000000000000), 32)
            a1 := shl(digits, a1)
            shift := add(shift, digits)

            digits := mul(lt(a1, 0x1000000000000000000000000000000000000000000000000000000000000), 16)
            a1 := shl(digits, a1)
            shift := add(shift, digits)

            digits := mul(lt(a1, 0x100000000000000000000000000000000000000000000000000000000000000), 8)
            a1 := shl(digits, a1)
            shift := add(shift, digits)

            digits := mul(lt(a1, 0x1000000000000000000000000000000000000000000000000000000000000000), 4)
            a1 := shl(digits, a1)
            shift := add(shift, digits)

            digits := mul(lt(a1, 0x4000000000000000000000000000000000000000000000000000000000000000), 2)
            a1 := shl(digits, a1)
            shift := add(shift, digits)

            a1 := or(a1, shr(sub(256, shift), a0))
            a0 := shl(shift, a0)
        }

        uint256 sp = Math.sqrt(a1);
        uint256 rp = a1 - (sp * sp);

        uint256 nom;
        uint256 denom;
        uint256 u;
        uint256 q;

        assembly {
            nom := or(shl(128, rp), shr(128, a0))
            denom := shl(1, sp)
            q := div(nom, denom)
            u := mod(nom, denom)

            // The nominator can be bigger than 2**256. We know that rp < (sp+1) * (sp+1). As sp can be
            // at most floor(sqrt(2**256 - 1)) we can conclude that the nominator has at most 513 bits
            // set. An expensive 512x256 bit division can be avoided by treating the bit at position 513 manually
            let carry := shr(128, rp)
            let x := mul(carry, 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff)
            q := add(q, div(x, denom))
            u := add(u, add(carry, mod(x, denom)))
            q := add(q, div(u, denom))
            u := mod(u, denom)
        }

        unchecked {
            s = (sp << 128) + q;

            uint256 rl = ((u << 128) | (a0 & 0xffffffffffffffffffffffffffffffff));
            uint256 rr = q * q;

            if ((q >> 128) > (u >> 128) || (((q >> 128) == (u >> 128)) && rl < rr)) {
                s = s - 1;
            }

            return s >> (shift / 2);
        }
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.30;

import {AssetType} from "../../../interfaces/ILiquidExchangeModel.sol";
import {Math} from "@openzeppelin/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/utils/math/SafeCast.sol";
import {FixedPoint} from "./FixedPoint.sol";
import {SqrtPriceMath} from "./SqrtPriceMath.sol";
import {Uint512} from "./Uint512.sol";

/**
 * @title Latent Math
 * @author Covenant Labs
 * @dev Library containing all the DEX functions for the exchange of liquidity vs leverage value and debt value.
 **/

library LatentMath {
    using Math for uint256;
    using SafeCast for uint256;
    using SafeCast for bool;

    struct ComputeLiquidityVars {
        uint256 pDiffX96;
        uint256 betaX96;
        uint256 b2X192_0;
        uint256 b2X192_1;
        uint256 qX192_0;
        uint256 qX192_1;
        uint256 dX192_0;
        uint256 dX192_1;
    }

    /**
     * @notice Computes the liquidity invariant given the current balances.
     * @dev - For covenant market limts (max 90% > LTV >= 50%, 1.6 > Pb/Pa > 1.004, price = 1 at target LTV),
     * we find aTokenAmount and zTokenAmount should be < 2^152 to ensure no overflows and liquidity < 2^160.
     * @param sqrtRatioX96_A low market price edge (price (token1/token0) when token0 = 0)
     * @param sqrtRatioX96_B high market price edge ( price (token1/token0) when token1 = 0)
     * @param zTokenAmount amount of zTokens in market
     * @param aTokenAmount amount of aTokens in market
     * @return liquidity The calculated liquidity of the pool
     */
    function computeLiquidity(
        uint160 sqrtRatioX96_A,
        uint160 sqrtRatioX96_B,
        uint256 zTokenAmount,
        uint256 aTokenAmount
    ) internal pure returns (uint160) {
        /**********************************************************************************************
        // invariant                                                                                 //
        // L = invariant                               (L/PA - BX)(L.PB - BY) = L^2                  //
        // PA = sqrt(price_0)                                                                        // 
        // PB = sqrt(price_1)                                                                        //
        // BX = balance of coin X                                                                    //
        // BY = balance of coin Y          `                                                         //
        // @dev - reverts on overflow                                                                //
        //                                                                                           //
        // Computes L using square root solution                                                     //
        //    L = beta + sqrt(beta^2 - q)                                                            //
        // where                                                                                     //
        //    beta = (Y + X*sqrt(p_a)*sqrt(p_b))/(2*(sqrt(p_b)-sqrt(p_a)))                           //
        //    q = X*Y*sqrt(p_a)/(sqrt(p_b)-sqrt(p_a))                                                //
        //                                                                                           //
        //                                                                                           //
        **********************************************************************************************/
        ComputeLiquidityVars memory vars;
        // @dev - For covenant market limts (max LTV = 90%, 1.6 > Pb/Pa > 1.004),
        // we find that BaseTokenValue < Liquidity < 2^8 * BaseTokenValue.
        // So BaseTokenValue < 2^152 to ensure Liquidity < 2^160 across markets.
        // In addition, given market concavity, aTokenAmount < BaseTokenValue, and zTokenAmount < BaseTokenValue.
        // At any point within the market limits.

        // @dev - assumes sqrtPrice_B > sqrtPrice_A (otherwise reverts)
        vars.pDiffX96 = (sqrtRatioX96_B - sqrtRatioX96_A);

        // beta with X96 precision
        // b = (Y + X*sqrt(p_a)*sqrt(p_b))/(2*(sqrt(p_b)-sqrt(p_a)))
        // @dev - For covenant market limts (max LTV = 90%, Pb/Pa < 1.004),
        // we have 1/(2*(sqrt(p_b)-sqrt(p_a))) < 2^8
        // For markets with LTV > 50%, we also have sqrtRatioX96_A * sqrtRatioX96_B <= FixedPoint.Q192
        // so, combined, FixedPoint.Q192 / (2 * vars.pDiffX96) < 2^8 * FixedPoint.Q96 < 2^102
        // Thus, aTokenAmount and zTokenAmount < 2^154 for betaX96 to fit in uint256.
        // (but should be further constrained to < 2^152 given previous comments)
        vars.betaX96 =
            Math.mulDiv(aTokenAmount, FixedPoint.Q192, vars.pDiffX96 << 1) +
            Math.mulDiv(zTokenAmount * sqrtRatioX96_A, sqrtRatioX96_B, vars.pDiffX96 << 1);

        // beta^2 with X192 precision (512)
        (vars.b2X192_1, vars.b2X192_0) = Math.mul512(vars.betaX96, vars.betaX96);

        // q = X*Y*sqrt(p_a)/(sqrt(p_b)-sqrt(p_a)), with X192 precision (512 bits)
        // biggest aTokenAmount <= biggest zTokenAmount for markets with LTV >= 50%
        // Liquidity =< aTokenAmount + zTokenAmount (== when currentPrice == 1 only).
        // Given market constraints, sqrtA / (sqrtB - sqrtA) < 2^8.  Thus, aTokenAmount < 2^152 so as to not overflow.
        // zTokenAmount can be < 2^160 in calculation below.
        (vars.qX192_1, vars.qX192_0) = Math.mul512(
            Math.mulDiv(
                aTokenAmount,
                uint256(sqrtRatioX96_A) << FixedPoint.RESOLUTION,
                vars.pDiffX96,
                Math.Rounding.Ceil
            ),
            zTokenAmount * FixedPoint.Q96
        );

        // add (beta^2 - q) if it is > 0. otherwise disrgard this term. (this could happen due to rounding for lower balances)
        // check beta^2 > q for 512 bit numbers.
        if ((vars.qX192_1 == vars.b2X192_1 || vars.qX192_0 < vars.b2X192_0) || vars.qX192_1 < vars.b2X192_1) {
            // calculate difference.
            (vars.dX192_0, vars.dX192_1) = Uint512.sub512x512(vars.b2X192_0, vars.b2X192_1, vars.qX192_0, vars.qX192_1);
            // add sqrt of difference
            vars.betaX96 += Uint512.sqrt512(vars.dX192_0, vars.dX192_1);
        }

        // @dev Return does not require toUint160() SafeCast given FixedPoint.RESOLUTION shiftRight.
        return uint160(vars.betaX96 >> FixedPoint.RESOLUTION);
    }

    /**
     * @notice Computes the required swap amounts
     * @dev Does not verify price limits
     * @dev - For large liquidity amounts, and low tokenIn amounts, the output will be zero
     * @dev - Reverts for currentLiquidity == 0, currentSqrtRatioX96 == 0
     * @param currentLiquidity The current liquidity
     * @param currentSqrtRatioX96 The current market price
     * @param tokenSpecified The index of the token that is specified for the swap
     * @param amountSpecified The exact amount specified for the swap
     * @param isExactIn Whether the amounts specified are for the token coming into the swap or not.
     * @return amountCalculated The calculated amount (amountOut if isExactIn, or amountIn if !IsExactIn)
     * @return nextSqrtRatioX96 The next market price after swap
     */
    function computeSwap(
        uint160 currentLiquidity,
        uint160 currentSqrtRatioX96,
        AssetType tokenSpecified,
        uint256 amountSpecified,
        bool isExactIn
    ) internal pure returns (uint256 amountCalculated, uint160 nextSqrtRatioX96) {
        // In latent swaps, when an amount is coming in, it 'decreases' the balance of that token
        // The amount out comes from an 'increase' in balance
        if (tokenSpecified == AssetType.DEBT) {
            // For exactIn, round to make sure we do not pass the target price. Given price is going down, round up.
            // For !exactIn, round to make sure we pass the target price. Given price is going up, round up.
            nextSqrtRatioX96 = SqrtPriceMath.getNextSqrtPriceFromAmount0(
                currentSqrtRatioX96,
                currentLiquidity,
                amountSpecified,
                isExactIn,
                Math.Rounding.Ceil
            );

            // round down is exactIn, up otherwise
            amountCalculated = SqrtPriceMath.getAmount1Delta(
                currentSqrtRatioX96,
                nextSqrtRatioX96,
                currentLiquidity,
                isExactIn ? Math.Rounding.Floor : Math.Rounding.Ceil
            );
        } else if (tokenSpecified == AssetType.LEVERAGE) {
            // For exactIn, round to make sure we do not pass the target price. Given price is going up, round down.
            // For !exactIn, round to make sure we pass the target price. Given price is going down, round down.
            nextSqrtRatioX96 = SqrtPriceMath.getNextSqrtPriceFromAmount1(
                currentSqrtRatioX96,
                currentLiquidity,
                amountSpecified,
                isExactIn,
                Math.Rounding.Floor
            );

            // round down is exactIn, up otherwise
            amountCalculated = SqrtPriceMath.getAmount0Delta(
                currentSqrtRatioX96,
                nextSqrtRatioX96,
                currentLiquidity,
                isExactIn ? Math.Rounding.Floor : Math.Rounding.Ceil
            );
        } else revert();
    }

    /**
     * @notice Calculate amount of a + z Tokens minted when a fixed amount of liquidity is added
     * @dev Does not change current market price
     * @dev Round calculated balance down.
     * @param currentSqrtRatioX96 current market price
     * @param  edgeSqrtRatioX96_A low market price edge
     * @param  edgeSqrtRatioX96_B high market price edge
     * @param liquidityIn Liquidity being added to the market
     * @return zTokenAmount zTokenAmount minted
     * @return aTokenAmount aTokenAmount minted
     **/
    function computeMint(
        uint160 currentSqrtRatioX96,
        uint160 edgeSqrtRatioX96_A,
        uint160 edgeSqrtRatioX96_B,
        uint160 liquidityIn
    ) internal pure returns (uint256 zTokenAmount, uint256 aTokenAmount) {
        zTokenAmount = SqrtPriceMath.getAmount0Delta(
            currentSqrtRatioX96,
            edgeSqrtRatioX96_A,
            liquidityIn,
            Math.Rounding.Floor
        );
        aTokenAmount = SqrtPriceMath.getAmount1Delta(
            edgeSqrtRatioX96_B,
            currentSqrtRatioX96,
            liquidityIn,
            Math.Rounding.Floor
        );
    }

    /**
     * @notice Computes liquidity out given exact zToken and aToken amounts
     * @dev - Estimates liquidity out, with higher error the bigger % of liquidity being redeemed given market size
     * @dev - Keeps nextSqrtRatioX96 within bounds
     * @param currentLiquidity The current liquidity
     * @param currentSqrtRatioX96 The current market price
     * @param edgeSqrtRatioX96_A low market price edge
     * @param edgeSqrtRatioX96_B high market price edge
     * @param zTokenAmtIn The exact amount of zToken specified for the redeem
     * @param zTokenAmtIn The exact amount of zToken specified for the redeem
     * @return liquidityOut The liquidity amount calculated for redeem
     * @return nextSqrtRatioX96 The next market price after swap
     */
    function computeRedeem(
        uint160 currentLiquidity,
        uint160 currentSqrtRatioX96,
        uint160 edgeSqrtRatioX96_A,
        uint160 edgeSqrtRatioX96_B,
        uint256 zTokenAmtIn,
        uint256 aTokenAmtIn
    ) internal pure returns (uint160 liquidityOut, uint160 nextSqrtRatioX96) {
        // Calculate current market dex amounts
        // @dev - given getMarketStateFromLiquidityAndDebt(),
        // calculated aDexAmount == actual aDexAmount in circulation
        // calculated zDexAmount <= actual zDexAmount in circulation
        // (given current market price)

        (uint256 zDexAmount, uint256 aDexAmount) = computeMint(
            currentSqrtRatioX96,
            edgeSqrtRatioX96_A,
            edgeSqrtRatioX96_B,
            currentLiquidity
        );

        if (zTokenAmtIn >= zDexAmount && aTokenAmtIn >= aDexAmount) {
            //Full burn
            return (currentLiquidity, currentSqrtRatioX96);
        } else {
            // Calculate remaining zToken and aToken amounts after redeem (add 1 to overestimate)
            uint256 remZamt = (zTokenAmtIn < zDexAmount) ? zDexAmount - zTokenAmtIn + 1 : 0;
            uint256 remAamt = (aTokenAmtIn < aDexAmount) ? aDexAmount - aTokenAmtIn + 1 : 0;

            // Calculate remaining liquidity (add 1 to force rounding up)
            uint256 remLiq = (uint256(computeLiquidity(edgeSqrtRatioX96_A, edgeSqrtRatioX96_B, remZamt, remAamt)) + 1);

            // set max remLiq as currentLiquidity (no need to safeCast remLiq after this)
            if (remLiq > uint256(currentLiquidity)) remLiq = currentLiquidity;
            liquidityOut = currentLiquidity - uint160(remLiq);
            nextSqrtRatioX96 = (liquidityOut == 0)
                ? currentSqrtRatioX96
                : SqrtPriceMath.getNextSqrtPriceFromAmount0(
                    edgeSqrtRatioX96_A,
                    uint160(remLiq),
                    remZamt,
                    false,
                    Math.Rounding.Ceil
                );
            if (nextSqrtRatioX96 > edgeSqrtRatioX96_B) nextSqrtRatioX96 = edgeSqrtRatioX96_B;
            return (liquidityOut, nextSqrtRatioX96);
        }
    }

    /**
     * @notice Calculate the derivative of liquidity vs a given token (at tokenType), given all current balances.
     * @param currentSqrtRatioX96 The current sqrtRatio of the market
     * @param  edgeSqrtRatioX96_A low market price edge (price (token1/token0) when token0 = 0)
     * @param  edgeSqrtRatioX96_B high market price edge ( price (token1/token0) when token1 = 0)
     * @param tokenType the token balance we are calculating
     * @return ratioX96 The derivative (spot price) of token vs liquidity, with X96 precision
     * @dev the inverse derivatives are as follows
     * dX/dL = 1/sqrt(Pa) - 2/sqrt(P) + sqrt(Pb)/P
     * dY/dL = sqrt(Pb) - 2sqrt(P)+ P/sqrt(Pa)
     * where Pa and Pb are the edge prices, and P is the current market spot price.
     */
    function get_XvsL(
        uint160 currentSqrtRatioX96,
        uint160 edgeSqrtRatioX96_A,
        uint160 edgeSqrtRatioX96_B,
        AssetType tokenType
    ) internal pure returns (uint256 ratioX96) {
        if (tokenType == AssetType.DEBT) {
            // calculates inverse derivative with Q96 resolution
            ratioX96 =
                Math.mulDiv(edgeSqrtRatioX96_B, FixedPoint.Q192, currentSqrtRatioX96) /
                currentSqrtRatioX96 +
                FixedPoint.Q192 /
                uint256(edgeSqrtRatioX96_A) -
                (FixedPoint.Q192 << 1) /
                currentSqrtRatioX96;
        } else if (tokenType == AssetType.LEVERAGE) {
            ratioX96 =
                Math.mulDiv(currentSqrtRatioX96, currentSqrtRatioX96, edgeSqrtRatioX96_A) +
                uint256(edgeSqrtRatioX96_B) -
                (uint256(currentSqrtRatioX96) << 1);
        } else revert();
    }

    /**
     * @notice Functionality used in LatentSwapLEX to calculate aDexAmount + marketSqrt price given zDexAmount + market Liquidity
     * @dev - functionality isolated into this function for testing purposes, to ensure updateMarket + redeem functionality are aligned
     * @param edgeSqrtRatioX96_A low market price edge
     * @param edgeSqrtRatioX96_B high market price edge
     * @param liquidity The current market liquidity
     * @param zDexAmount The exact amount of zToken specified for the redeem
     * @return aDexAmount The derived aDexAmount
     * @return currentSqrtPriceX96 The derived marketSqrtPriceX96
     */
    function getMarketStateFromLiquidityAndDebt(
        uint160 edgeSqrtRatioX96_A,
        uint160 edgeSqrtRatioX96_B,
        uint160 liquidity,
        uint256 zDexAmount
    ) internal pure returns (uint256 aDexAmount, uint160 currentSqrtPriceX96) {
        // calculate market price given liquidity and zDexAmount
        // Round up current price (implicitly rounds up debt value, rounds down aDexAmount in next calculation)
        currentSqrtPriceX96 = SqrtPriceMath.getNextSqrtPriceFromAmount0(
            edgeSqrtRatioX96_A,
            liquidity,
            zDexAmount,
            false,
            Math.Rounding.Ceil
        );

        // calculate aSynthAmount given current price and market liquidity
        // Round down aDexAmount
        aDexAmount = SqrtPriceMath.getAmount1Delta(
            edgeSqrtRatioX96_B,
            currentSqrtPriceX96,
            liquidity,
            Math.Rounding.Floor
        );
    }

    /**
     * @notice Calculates marginal value of L vs debt when market price == 1
     * @param edgeSqrtRatioX96_A low market price edge
     * @param edgeSqrtRatioX96_B high market price edge
     * @return targetXvsL XvsL when market on target (price == 1), with X96 precision
     */
    function targetXvsL(uint160 edgeSqrtRatioX96_A, uint160 edgeSqrtRatioX96_B) internal pure returns (uint256) {
        return get_XvsL(uint160(FixedPoint.Q96), edgeSqrtRatioX96_A, edgeSqrtRatioX96_B, AssetType.DEBT);
    }

    /**
     * @notice Calculates LTV given current market price and market edges.
     * @param edgeSqrtRatioX96_A low market price edge
     * @param edgeSqrtRatioX96_B high market price edge
     * @param currentSqrtRatioX96 current market price
     * @return LTV where 10000 = 100%
     */
    function computeLTV(
        uint160 edgeSqrtRatioX96_A,
        uint160 edgeSqrtRatioX96_B,
        uint160 currentSqrtRatioX96
    ) internal pure returns (uint256) {
        /**********************************************************************************************
        // At extremes, all X or all Y markets have equivalent value                                 //
        // (equal to value of collateral)                                                            //
        //                                                                                           //
        // All X coin Amount0 = Pb - Pa / Pb / Pa                                                    //
        // All Y coin Amount1 = Pb - Pa                                                              //
        //                                                                                           //
        // Where                                                                                     //
        // Pa = sqrt(price_0)                                                                        //
        // Pb = sqrt(price_1)                                                                        //
        // Pc = sqrt(price_current)                                                                  //
        //                                                                                           //
        //                                                                                           //
        // Thus, from a value perspective, if Vx = Amount0, then Vy = Amount1 * Pb * Pa              //
        //                                                                                           //
        // We define LTV = Vx / (Vy + Vx)                                                            //
        //               = Amount0 / (Amount0 + Amount1 * Pb * Pa)                                   //
        //                                                                                           //
        // Solving this (using Amount0 = 1 / Pc - 1 / Pa, and Amount 1 = Pb - Pc (see SqrtPriceMath) //
        //                                                                                           //
        // LTV = (Pc - Pa).Pb / [(Pc - Pa).Pb + (Pb - Pc).Pc]                                        //
        //                                                                                           //
        //                                                                                           //
        **********************************************************************************************/

        // @dev - does not revert for
        // edgeSqrtRatioX96_B <= (2^32) * FixedPoint.Q96
        // and 0 < edgeSqrtRatioX96_A <= currentSqrtRatioX96 <= edgeSqrtRatioX96_B
        // given edgeSqrtRatioX96_B ^ 2 <= 2^256
        uint256 calc1 = uint256(edgeSqrtRatioX96_B) * uint256(currentSqrtRatioX96 - edgeSqrtRatioX96_A);
        uint256 calc2 = uint256(currentSqrtRatioX96) * uint256(edgeSqrtRatioX96_B - currentSqrtRatioX96);

        return Math.mulDiv(FixedPoint.PERCENTAGE_FACTOR, calc1, calc1 + calc2);
    }

    /**
     * @notice Returns the maximum debt amount (in dex units) given liquidity and limit prices
     * @param edgeSqrtRatioX96_A low market price edge
     * @param edgeSqrtRatioX96_B high market price edge
     * @param liquidity current market liquidity
     * @return maxDebt The maximum debt amount
     */
    function computeMaxDebt(
        uint160 edgeSqrtRatioX96_A,
        uint160 edgeSqrtRatioX96_B,
        uint160 liquidity
    ) internal pure returns (uint256) {
        return SqrtPriceMath.getAmount0Delta(edgeSqrtRatioX96_B, edgeSqrtRatioX96_A, liquidity, Math.Rounding.Floor);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {IERC20Metadata} from "@openzeppelin/token/ERC20/extensions/IERC20Metadata.sol";

library SafeMetadata {
    /**
     * @dev Attempts to fetch the asset name as a string. A return value of false indicates that the attempt failed in some way.
     */
    function tryGetName(IERC20 token) internal view returns (bool ok, string memory out) {
        return _tryStringOrBytes32(address(token), IERC20Metadata.name.selector);
    }

    /**
     * @dev Attempts to fetch the asset symbol as a string. A return value of false indicates that the attempt failed in some way.
     */
    function tryGetSymbol(IERC20 token) internal view returns (bool ok, string memory out) {
        return _tryStringOrBytes32(address(token), IERC20Metadata.symbol.selector);
    }

    /**
     * @dev Attempts to fetch the asset decimals. A return value of false indicates that the attempt failed in some way.
     */
    function tryGetDecimals(IERC20 token) internal view returns (bool ok, uint8 assetDecimals) {
        (bool success, bytes memory encodedDecimals) = address(token).staticcall(
            abi.encodeCall(IERC20Metadata.decimals, ())
        );
        if (success && encodedDecimals.length >= 32) {
            uint256 returnedDecimals = abi.decode(encodedDecimals, (uint256));
            if (returnedDecimals <= type(uint8).max) {
                return (true, uint8(returnedDecimals));
            }
        }
        return (false, 0);
    }

    function _tryStringOrBytes32(address token, bytes4 selector) private view returns (bool ok, string memory out) {
        // Enforce read-only
        (bool success, bytes memory data) = token.staticcall(abi.encodeWithSelector(selector));
        if (!success) return (false, "");

        // Try standard (string).  Reverts on malformed data.
        if (data.length >= 64) return (true, abi.decode(data, (string)));

        // Fallback: bytes32 (older tokens)
        if (data.length == 32) {
            bytes32 raw = abi.decode(data, (bytes32));
            return (true, _bytes32ToString(raw));
        }

        // Anything else: treat as failure
        return (false, "");
    }

    // separate to allow try/catch
    function _decodeString(bytes memory data) internal pure returns (string memory s) {
        return abi.decode(data, (string));
    }

    function _bytes32ToString(bytes32 x) private pure returns (string memory) {
        uint256 len = 32;
        while (len > 0 && x[len - 1] == 0) {
            unchecked {
                len--;
            }
        }
        bytes memory out = new bytes(len);
        for (uint256 i = 0; i < len; ++i) out[i] = x[i];
        return string(out);
    }
}

// SPDX-License-Identifier: AGPL-3.0
pragma solidity >=0.8.0;

/**
 * @title ITokenData
 * @author Covenant Labs
 * @notice Defines interface for symbol and decimal overrides
 **/

interface ITokenData {
    function assetDecimals(address asset) external view returns (uint8);
    function assetSymbol(address asset) external view returns (string memory);
    function assetName(address asset) external view returns (string memory);
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.30;

import {FixedPointMathLib} from "@solady/utils/FixedPointMathLib.sol";
import {Math} from "@openzeppelin/utils/math/Math.sol";
import {SaturatingMath} from "./SaturatingMath.sol";
import {FixedPoint} from "./FixedPoint.sol";

/**
 * @title DebtMath library
 * @author Covenant Labs
 * @notice Provides approximations for Perpetual Debt calculations
 */
library DebtMath {
    using SaturatingMath for uint256;
    using FixedPointMathLib for int256;
    using Math for uint256;

    uint256 internal constant SECONDS_PER_YEAR = 365 days;

    /**
     * @notice calculates interest update factor given perpetual debt duration, debt notional price and elapsed time.
     * @param _amount amount on which interest is being applied
     * @param _duration effective duration of the debt (in seconds)
     * @param _discountPrice discount price (vs debt notional) in WADs
     * @param _elapsedTime time over which to accrue interest (in seconds)
     * @param _lnRateBias additional market rate bias (that is not determined by price), in WADs
     * @return updatedAmount_  the updated amount given debt interest rate and elapsed time
     **/
    function accrueInterest(
        uint256 _amount,
        uint256 _duration,
        uint256 _discountPrice,
        uint256 _elapsedTime,
        int256 _lnRateBias
    ) internal pure returns (uint256 updatedAmount_) {
        // Calculate rate = - ln(price) + lnRateBias
        // and then updates amount.
        return accrueInterestLnRate(_amount, _lnRateBias - int256(_discountPrice).lnWad(), _elapsedTime, _duration);
    }

    /**
     * @notice updates amount given duration, lnRate and elapsed time.
     * @dev interest accrual saturates.  ie, calculation will not revert,
     * and instead updatedAmount will be >0 and <=type(uint256).max
     * @param _amount amount to be update given duration, lnRate and elapsed time.
     * @param _duration effective duration of the debt (in seconds)
     * @param _lnRate lnRate in WADs (lnRate < 1 is a negative interest rate)
     * @param _elapsedTime time over which to accrue interest (in seconds)
     * @return updatedAmount_  the updated amount given debt interest rate and elapsed time
     **/
    function accrueInterestLnRate(
        uint256 _amount,
        int256 _lnRate,
        uint256 _elapsedTime,
        uint256 _duration
    ) internal pure returns (uint256 updatedAmount_) {
        uint256 updateFactor = calculateApproxExponentialUpdate(
            uint256((_lnRate >= 0) ? _lnRate : -_lnRate),
            _elapsedTime,
            _duration
        );

        if (_lnRate >= 0) {
            return _amount.saturatingMulDiv(updateFactor, FixedPoint.RAY);
        } else {
            // @dev - when lnRate < 0, we calculate exp(x), but then divide _amount by that updatefactor.
            // given e(-x) = 1 / e(x). Amount is never allowed to get to 0 from interest accrual.
            updatedAmount_ = _amount.mulDiv(FixedPoint.RAY, updateFactor);
            if (updatedAmount_ == 0 && _amount > 0) updatedAmount_ = 1;
        }
    }

    /**
     * @notice Calculates approximation of exp(lnRate * timeDelta / duration) for small values of rate * timeDelta / duration
     * @dev rate * timeDelta / duration is considered small, given timeDelta << duration, and rangebound rate
     * @dev A taylor expansion is used to calculate exp(rate * timeDelta / duration), and output will alwas be <= to the exact calculation.
     * @dev Below calculation does not overflow, even in extremes.  e.g, max lnRAte
     * @dev below does not overflow for reasonable extremes.  E.g, duration of 1 year (in seconds), time elapsed of 10,000 years, lnRate = 7.9 RAYS (= 250000% daily rate)
     * @param _lnRate logaritmic rate. -ln(price) in WADs
     * @param _timeDelta time over which to accrue interest (in seconds)
     * @param _duration effective duration of the debt (in seconds)
     * @return updateMultiplier_ the update multiplier (in RAYs) with which to update an amount
     **/
    function calculateApproxExponentialUpdate(
        uint256 _lnRate,
        uint256 _timeDelta,
        uint256 _duration
    ) internal pure returns (uint256 updateMultiplier_) {
        // @dev- for extreme cases (e.g., daily 10000% interest rate over 10000 years, with duration = 1 day),
        // both _lnRate and _timeDelta are expected to be < uint96.max, and the below
        // calculation not to revert.

        // approximation for exp(lnRate * timeDelta / duration)
        uint256 rate1 = (_lnRate * _timeDelta * FixedPoint.WAD_RAY_RATIO) / _duration;
        uint256 rate2 = rate1.mulDiv(rate1, 2 * FixedPoint.RAY);
        uint256 rate3 = rate2.mulDiv(rate1, 3 * FixedPoint.RAY);
        return FixedPoint.RAY + rate1 + rate2 + rate3;
    }

    // returns linear update multiplier (ray units)
    // assumes rate in BPS for a yearly duration
    // @dev - output value saturates at type(uint256).max
    function calculateLinearAccrual(
        uint256 _value,
        uint256 _rateBPS,
        uint256 _timeDelta
    ) internal pure returns (uint256 accrualValue_) {
        // @dev - Even for extreme rate and timeDelta cases, _rate expected to be < type(uint160).max
        // and _timeDelta < type(uint96).max.  Given this, below does not revert for any
        // _value <= type(uint256).max.

        return _value.saturatingMulDiv(_rateBPS * _timeDelta, SECONDS_PER_YEAR * FixedPoint.PERCENTAGE_FACTOR);
    }
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


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES
// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title PercentageMath library
 * @author Aave
 * @notice Provides functions to perform percentage calculations
 * @dev Percentages are defined by default with 2 decimals of precision (100.00). The precision is indicated by PERCENTAGE_FACTOR
 * @dev Operations are rounded. If a value is >=.5, will be rounded up, otherwise rounded down.
 */
library PercentageMath {
  // Maximum percentage factor (100.00%)
  uint256 internal constant PERCENTAGE_FACTOR = 1e4;

  // Half percentage factor (50.00%)
  uint256 internal constant HALF_PERCENTAGE_FACTOR = 0.5e4;

  /**
   * @notice Executes a percentage multiplication
   * @dev assembly optimized for improved gas savings, see https://twitter.com/transmissions11/status/1451131036377571328
   * @param value The value of which the percentage needs to be calculated
   * @param percentage The percentage of the value to be calculated
   * @return result value percentmul percentage
   */
  function percentMul(uint256 value, uint256 percentage) internal pure returns (uint256 result) {
    // to avoid overflow, value <= (type(uint256).max - HALF_PERCENTAGE_FACTOR) / percentage
    assembly {
      if iszero(
        or(
          iszero(percentage),
          iszero(gt(value, div(sub(not(0), HALF_PERCENTAGE_FACTOR), percentage)))
        )
      ) {
        revert(0, 0)
      }

      result := div(add(mul(value, percentage), HALF_PERCENTAGE_FACTOR), PERCENTAGE_FACTOR)
    }
  }

  /**
   * @notice Executes a percentage division
   * @dev assembly optimized for improved gas savings, see https://twitter.com/transmissions11/status/1451131036377571328
   * @param value The value of which the percentage needs to be calculated
   * @param percentage The percentage of the value to be calculated
   * @return result value percentdiv percentage
   */
  function percentDiv(uint256 value, uint256 percentage) internal pure returns (uint256 result) {
    // to avoid overflow, value <= (type(uint256).max - halfPercentage) / PERCENTAGE_FACTOR
    assembly {
      if or(
        iszero(percentage),
        iszero(iszero(gt(value, div(sub(not(0), div(percentage, 2)), PERCENTAGE_FACTOR))))
      ) {
        revert(0, 0)
      }

      result := div(add(mul(value, PERCENTAGE_FACTOR), div(percentage, 2)), percentage)
    }
  }
}


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {ICovenant} from "../src/interfaces/ICovenant.sol";
import {LatentSwapLEX, ILatentSwapLEX} from "../src/lex/latentswap/LatentSwapLEX.sol";
import {LatentSwapLib} from "../src/periphery/libraries/LatentSwapLib.sol";
import {SafeCast} from "@openzeppelin/utils/math/SafeCast.sol";

// @dev - order needs to be alphabetical given forge-std constraints
struct CreateLEXParams {
    address covenantLiquid;
    uint256 debtDuration;
    uint256 highLTV;
    uint256 maxLTV;
    int256 rateBias;
    uint256 swapFee;
    uint256 targetLTV;
    uint256 yieldWidth;
}

struct runVars {
    string jsonString;
    bytes jsonData;
    CreateLEXParams LEXparams;
    uint160 maxMarketPriceX96;
    uint160 minMarketPriceX96;
    uint160 targetPriceX96;
    uint160 maxLimitPriceX96;
    uint160 highLimitPriceX96;
    uint160 edgeHighPriceX96;
    uint160 edgeLowPriceX96;
    address lexImplementation;
}

interface ILatentSwapLEXextended {
    function setQuoteTokenSymbolOverrideForNewMarkets(address token, string memory symbol) external;
    function setQuoteTokenDecimalsOverrideForNewMarkets(address token, uint8 decimals) external;
    function setDefaultNoCapLimit(address token, uint8 noCapLimit) external;
}

contract CastCreateLEX is Script {
    using SafeCast for uint256;
    using SafeCast for int256;

    function run() external {
        runVars memory vars;

        // Read market param JSON
        vars.jsonString = vm.readFile("./script/CreateLEXParams.json"); //prettier-ignore
        vars.jsonData = vm.parseJson(vars.jsonString);
        vars.LEXparams = abi.decode(vars.jsonData, (CreateLEXParams)); //prettier-ignore

        // Print parsing
        console.log("LEX Parameters");
        console.log("covenantLiquid", vars.LEXparams.covenantLiquid);
        console.log("targetLTV", vars.LEXparams.targetLTV);
        console.log("highLTV", vars.LEXparams.highLTV);
        console.log("maxLTV", vars.LEXparams.maxLTV);
        console.log("debtDuration", vars.LEXparams.debtDuration);
        console.log("swapFee", vars.LEXparams.swapFee);
        console.log("yieldWidth", vars.LEXparams.yieldWidth);
        console.log("rateBias", vars.LEXparams.rateBias);

        // convert config parameters to initialization parameters
        (vars.minMarketPriceX96, vars.maxMarketPriceX96) = LatentSwapLib.getMarketEdgePrices(
            vars.LEXparams.targetLTV.toUint32(),
            vars.LEXparams.yieldWidth
        );
        vars.maxLimitPriceX96 = LatentSwapLib.getSqrtPriceFromLTVX96(
            vars.minMarketPriceX96,
            vars.maxMarketPriceX96,
            vars.LEXparams.maxLTV.toUint32()
        );
        vars.highLimitPriceX96 = LatentSwapLib.getSqrtPriceFromLTVX96(
            vars.minMarketPriceX96,
            vars.maxMarketPriceX96,
            vars.LEXparams.highLTV.toUint32()
        );

        // Create market onchain
        vm.startBroadcast();

        // deploy lex implementation
        vars.lexImplementation = address(
            new LatentSwapLEX(
                msg.sender,
                vars.LEXparams.covenantLiquid,
                vars.maxMarketPriceX96,
                vars.minMarketPriceX96,
                vars.highLimitPriceX96,
                vars.maxLimitPriceX96,
                vars.LEXparams.rateBias.toInt64(),
                vars.LEXparams.debtDuration.toUint32(),
                vars.LEXparams.swapFee.toUint8()
            )
        );

        // authorize lex
        ICovenant(vars.LEXparams.covenantLiquid).setEnabledLEX(vars.lexImplementation, true);
        console.log("LEX implementation %s", vars.lexImplementation);

        // configure certain non-standard quote tokens

        // MON
        ILatentSwapLEXextended(vars.lexImplementation).setQuoteTokenSymbolOverrideForNewMarkets(address(0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE), "MON"); // prettier-ignore
        ILatentSwapLEXextended(vars.lexImplementation).setQuoteTokenDecimalsOverrideForNewMarkets(address(0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE), 18); // prettier-ignore
        ILatentSwapLEXextended(vars.lexImplementation).setDefaultNoCapLimit(
            address(0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE),
            70
        ); // 1200 MON limit

        // USD
        ILatentSwapLEXextended(vars.lexImplementation).setQuoteTokenSymbolOverrideForNewMarkets(address(0x0000000000000000000000000000000000000348), "USD"); // prettier-ignore
        ILatentSwapLEXextended(vars.lexImplementation).setQuoteTokenDecimalsOverrideForNewMarkets(address(0x0000000000000000000000000000000000000348), 6); // prettier-ignore
        ILatentSwapLEXextended(vars.lexImplementation).setDefaultNoCapLimit(
            address(0x0000000000000000000000000000000000000348),
            37
        ); // $130K limit

        vm.stopBroadcast();
    }
}

