
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

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

/// @notice Arithmetic library with operations for fixed-point numbers.
/// @author Solady (https://github.com/vectorized/solady/blob/main/src/utils/FixedPointMathLib.sol)
/// @author Modified from Solmate (https://github.com/transmissions11/solmate/blob/main/src/utils/FixedPointMathLib.sol)
library FixedPointMathLib {
    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                       CUSTOM ERRORS                        */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The operation failed, as the output exceeds the maximum value of uint256.
    error ExpOverflow();

    /// @dev The operation failed, as the output exceeds the maximum value of uint256.
    error FactorialOverflow();

    /// @dev The operation failed, due to an overflow.
    error RPowOverflow();

    /// @dev The mantissa is too big to fit.
    error MantissaOverflow();

    /// @dev The operation failed, due to an multiplication overflow.
    error MulWadFailed();

    /// @dev The operation failed, due to an multiplication overflow.
    error SMulWadFailed();

    /// @dev The operation failed, either due to a multiplication overflow, or a division by a zero.
    error DivWadFailed();

    /// @dev The operation failed, either due to a multiplication overflow, or a division by a zero.
    error SDivWadFailed();

    /// @dev The operation failed, either due to a multiplication overflow, or a division by a zero.
    error MulDivFailed();

    /// @dev The division failed, as the denominator is zero.
    error DivFailed();

    /// @dev The full precision multiply-divide operation failed, either due
    /// to the result being larger than 256 bits, or a division by a zero.
    error FullMulDivFailed();

    /// @dev The output is undefined, as the input is less-than-or-equal to zero.
    error LnWadUndefined();

    /// @dev The input outside the acceptable domain.
    error OutOfDomain();

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                         CONSTANTS                          */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The scalar of ETH and most ERC20s.
    uint256 internal constant WAD = 1e18;

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*              SIMPLIFIED FIXED POINT OPERATIONS             */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Equivalent to `(x * y) / WAD` rounded down.
    function mulWad(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            // Equivalent to `require(y == 0 || x <= type(uint256).max / y)`.
            if gt(x, div(not(0), y)) {
                if y {
                    mstore(0x00, 0xbac65e5b) // `MulWadFailed()`.
                    revert(0x1c, 0x04)
                }
            }
            z := div(mul(x, y), WAD)
        }
    }

    /// @dev Equivalent to `(x * y) / WAD` rounded down.
    function sMulWad(int256 x, int256 y) internal pure returns (int256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := mul(x, y)
            // Equivalent to `require((x == 0 || z / x == y) && !(x == -1 && y == type(int256).min))`.
            if iszero(gt(or(iszero(x), eq(sdiv(z, x), y)), lt(not(x), eq(y, shl(255, 1))))) {
                mstore(0x00, 0xedcd4dd4) // `SMulWadFailed()`.
                revert(0x1c, 0x04)
            }
            z := sdiv(z, WAD)
        }
    }

    /// @dev Equivalent to `(x * y) / WAD` rounded down, but without overflow checks.
    function rawMulWad(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := div(mul(x, y), WAD)
        }
    }

    /// @dev Equivalent to `(x * y) / WAD` rounded down, but without overflow checks.
    function rawSMulWad(int256 x, int256 y) internal pure returns (int256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := sdiv(mul(x, y), WAD)
        }
    }

    /// @dev Equivalent to `(x * y) / WAD` rounded up.
    function mulWadUp(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := mul(x, y)
            // Equivalent to `require(y == 0 || x <= type(uint256).max / y)`.
            if iszero(eq(div(z, y), x)) {
                if y {
                    mstore(0x00, 0xbac65e5b) // `MulWadFailed()`.
                    revert(0x1c, 0x04)
                }
            }
            z := add(iszero(iszero(mod(z, WAD))), div(z, WAD))
        }
    }

    /// @dev Equivalent to `(x * y) / WAD` rounded up, but without overflow checks.
    function rawMulWadUp(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := add(iszero(iszero(mod(mul(x, y), WAD))), div(mul(x, y), WAD))
        }
    }

    /// @dev Equivalent to `(x * WAD) / y` rounded down.
    function divWad(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            // Equivalent to `require(y != 0 && x <= type(uint256).max / WAD)`.
            if iszero(mul(y, lt(x, add(1, div(not(0), WAD))))) {
                mstore(0x00, 0x7c5f487d) // `DivWadFailed()`.
                revert(0x1c, 0x04)
            }
            z := div(mul(x, WAD), y)
        }
    }

    /// @dev Equivalent to `(x * WAD) / y` rounded down.
    function sDivWad(int256 x, int256 y) internal pure returns (int256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := mul(x, WAD)
            // Equivalent to `require(y != 0 && ((x * WAD) / WAD == x))`.
            if iszero(mul(y, eq(sdiv(z, WAD), x))) {
                mstore(0x00, 0x5c43740d) // `SDivWadFailed()`.
                revert(0x1c, 0x04)
            }
            z := sdiv(z, y)
        }
    }

    /// @dev Equivalent to `(x * WAD) / y` rounded down, but without overflow and divide by zero checks.
    function rawDivWad(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := div(mul(x, WAD), y)
        }
    }

    /// @dev Equivalent to `(x * WAD) / y` rounded down, but without overflow and divide by zero checks.
    function rawSDivWad(int256 x, int256 y) internal pure returns (int256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := sdiv(mul(x, WAD), y)
        }
    }

    /// @dev Equivalent to `(x * WAD) / y` rounded up.
    function divWadUp(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            // Equivalent to `require(y != 0 && x <= type(uint256).max / WAD)`.
            if iszero(mul(y, lt(x, add(1, div(not(0), WAD))))) {
                mstore(0x00, 0x7c5f487d) // `DivWadFailed()`.
                revert(0x1c, 0x04)
            }
            z := add(iszero(iszero(mod(mul(x, WAD), y))), div(mul(x, WAD), y))
        }
    }

    /// @dev Equivalent to `(x * WAD) / y` rounded up, but without overflow and divide by zero checks.
    function rawDivWadUp(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := add(iszero(iszero(mod(mul(x, WAD), y))), div(mul(x, WAD), y))
        }
    }

    /// @dev Equivalent to `x` to the power of `y`.
    /// because `x ** y = (e ** ln(x)) ** y = e ** (ln(x) * y)`.
    /// Note: This function is an approximation.
    function powWad(int256 x, int256 y) internal pure returns (int256) {
        // Using `ln(x)` means `x` must be greater than 0.
        return expWad((lnWad(x) * y) / int256(WAD));
    }

    /// @dev Returns `exp(x)`, denominated in `WAD`.
    /// Credit to Remco Bloemen under MIT license: https://2π.com/22/exp-ln
    /// Note: This function is an approximation. Monotonically increasing.
    function expWad(int256 x) internal pure returns (int256 r) {
        unchecked {
            // When the result is less than 0.5 we return zero.
            // This happens when `x <= (log(1e-18) * 1e18) ~ -4.15e19`.
            if (x <= -41446531673892822313) return r;

            /// @solidity memory-safe-assembly
            assembly {
                // When the result is greater than `(2**255 - 1) / 1e18` we can not represent it as
                // an int. This happens when `x >= floor(log((2**255 - 1) / 1e18) * 1e18) ≈ 135`.
                if iszero(slt(x, 135305999368893231589)) {
                    mstore(0x00, 0xa37bfec9) // `ExpOverflow()`.
                    revert(0x1c, 0x04)
                }
            }

            // `x` is now in the range `(-42, 136) * 1e18`. Convert to `(-42, 136) * 2**96`
            // for more intermediate precision and a binary basis. This base conversion
            // is a multiplication by 1e18 / 2**96 = 5**18 / 2**78.
            x = (x << 78) / 5 ** 18;

            // Reduce range of x to (-½ ln 2, ½ ln 2) * 2**96 by factoring out powers
            // of two such that exp(x) = exp(x') * 2**k, where k is an integer.
            // Solving this gives k = round(x / log(2)) and x' = x - k * log(2).
            int256 k = ((x << 96) / 54916777467707473351141471128 + 2 ** 95) >> 96;
            x = x - k * 54916777467707473351141471128;

            // `k` is in the range `[-61, 195]`.

            // Evaluate using a (6, 7)-term rational approximation.
            // `p` is made monic, we'll multiply by a scale factor later.
            int256 y = x + 1346386616545796478920950773328;
            y = ((y * x) >> 96) + 57155421227552351082224309758442;
            int256 p = y + x - 94201549194550492254356042504812;
            p = ((p * y) >> 96) + 28719021644029726153956944680412240;
            p = p * x + (4385272521454847904659076985693276 << 96);

            // We leave `p` in `2**192` basis so we don't need to scale it back up for the division.
            int256 q = x - 2855989394907223263936484059900;
            q = ((q * x) >> 96) + 50020603652535783019961831881945;
            q = ((q * x) >> 96) - 533845033583426703283633433725380;
            q = ((q * x) >> 96) + 3604857256930695427073651918091429;
            q = ((q * x) >> 96) - 14423608567350463180887372962807573;
            q = ((q * x) >> 96) + 26449188498355588339934803723976023;

            /// @solidity memory-safe-assembly
            assembly {
                // Div in assembly because solidity adds a zero check despite the unchecked.
                // The q polynomial won't have zeros in the domain as all its roots are complex.
                // No scaling is necessary because p is already `2**96` too large.
                r := sdiv(p, q)
            }

            // r should be in the range `(0.09, 0.25) * 2**96`.

            // We now need to multiply r by:
            // - The scale factor `s ≈ 6.031367120`.
            // - The `2**k` factor from the range reduction.
            // - The `1e18 / 2**96` factor for base conversion.
            // We do this all at once, with an intermediate result in `2**213`
            // basis, so the final right shift is always by a positive amount.
            r = int256(
                (uint256(r) * 3822833074963236453042738258902158003155416615667) >> uint256(195 - k)
            );
        }
    }

    /// @dev Returns `ln(x)`, denominated in `WAD`.
    /// Credit to Remco Bloemen under MIT license: https://2π.com/22/exp-ln
    /// Note: This function is an approximation. Monotonically increasing.
    function lnWad(int256 x) internal pure returns (int256 r) {
        /// @solidity memory-safe-assembly
        assembly {
            // We want to convert `x` from `10**18` fixed point to `2**96` fixed point.
            // We do this by multiplying by `2**96 / 10**18`. But since
            // `ln(x * C) = ln(x) + ln(C)`, we can simply do nothing here
            // and add `ln(2**96 / 10**18)` at the end.

            // Compute `k = log2(x) - 96`, `r = 159 - k = 255 - log2(x) = 255 ^ log2(x)`.
            r := shl(7, lt(0xffffffffffffffffffffffffffffffff, x))
            r := or(r, shl(6, lt(0xffffffffffffffff, shr(r, x))))
            r := or(r, shl(5, lt(0xffffffff, shr(r, x))))
            r := or(r, shl(4, lt(0xffff, shr(r, x))))
            r := or(r, shl(3, lt(0xff, shr(r, x))))
            // We place the check here for more optimal stack operations.
            if iszero(sgt(x, 0)) {
                mstore(0x00, 0x1615e638) // `LnWadUndefined()`.
                revert(0x1c, 0x04)
            }
            // forgefmt: disable-next-item
            r := xor(r, byte(and(0x1f, shr(shr(r, x), 0x8421084210842108cc6318c6db6d54be)),
                0xf8f9f9faf9fdfafbf9fdfcfdfafbfcfef9fafdfafcfcfbfefafafcfbffffffff))

            // Reduce range of x to (1, 2) * 2**96
            // ln(2^k * x) = k * ln(2) + ln(x)
            x := shr(159, shl(r, x))

            // Evaluate using a (8, 8)-term rational approximation.
            // `p` is made monic, we will multiply by a scale factor later.
            // forgefmt: disable-next-item
            let p := sub( // This heavily nested expression is to avoid stack-too-deep for via-ir.
                sar(96, mul(add(43456485725739037958740375743393,
                sar(96, mul(add(24828157081833163892658089445524,
                sar(96, mul(add(3273285459638523848632254066296,
                    x), x))), x))), x)), 11111509109440967052023855526967)
            p := sub(sar(96, mul(p, x)), 45023709667254063763336534515857)
            p := sub(sar(96, mul(p, x)), 14706773417378608786704636184526)
            p := sub(mul(p, x), shl(96, 795164235651350426258249787498))
            // We leave `p` in `2**192` basis so we don't need to scale it back up for the division.

            // `q` is monic by convention.
            let q := add(5573035233440673466300451813936, x)
            q := add(71694874799317883764090561454958, sar(96, mul(x, q)))
            q := add(283447036172924575727196451306956, sar(96, mul(x, q)))
            q := add(401686690394027663651624208769553, sar(96, mul(x, q)))
            q := add(204048457590392012362485061816622, sar(96, mul(x, q)))
            q := add(31853899698501571402653359427138, sar(96, mul(x, q)))
            q := add(909429971244387300277376558375, sar(96, mul(x, q)))

            // `p / q` is in the range `(0, 0.125) * 2**96`.

            // Finalization, we need to:
            // - Multiply by the scale factor `s = 5.549…`.
            // - Add `ln(2**96 / 10**18)`.
            // - Add `k * ln(2)`.
            // - Multiply by `10**18 / 2**96 = 5**18 >> 78`.

            // The q polynomial is known not to have zeros in the domain.
            // No scaling required because p is already `2**96` too large.
            p := sdiv(p, q)
            // Multiply by the scaling factor: `s * 5**18 * 2**96`, base is now `5**18 * 2**192`.
            p := mul(1677202110996718588342820967067443963516166, p)
            // Add `ln(2) * k * 5**18 * 2**192`.
            // forgefmt: disable-next-item
            p := add(mul(16597577552685614221487285958193947469193820559219878177908093499208371, sub(159, r)), p)
            // Add `ln(2**96 / 10**18) * 5**18 * 2**192`.
            p := add(600920179829731861736702779321621459595472258049074101567377883020018308, p)
            // Base conversion: mul `2**18 / 2**192`.
            r := sar(174, p)
        }
    }

    /// @dev Returns `W_0(x)`, denominated in `WAD`.
    /// See: https://en.wikipedia.org/wiki/Lambert_W_function
    /// a.k.a. Product log function. This is an approximation of the principal branch.
    /// Note: This function is an approximation. Monotonically increasing.
    function lambertW0Wad(int256 x) internal pure returns (int256 w) {
        // forgefmt: disable-next-item
        unchecked {
            if ((w = x) <= -367879441171442322) revert OutOfDomain(); // `x` less than `-1/e`.
            (int256 wad, int256 p) = (int256(WAD), x);
            uint256 c; // Whether we need to avoid catastrophic cancellation.
            uint256 i = 4; // Number of iterations.
            if (w <= 0x1ffffffffffff) {
                if (-0x4000000000000 <= w) {
                    i = 1; // Inputs near zero only take one step to converge.
                } else if (w <= -0x3ffffffffffffff) {
                    i = 32; // Inputs near `-1/e` take very long to converge.
                }
            } else if (uint256(w >> 63) == uint256(0)) {
                /// @solidity memory-safe-assembly
                assembly {
                    // Inline log2 for more performance, since the range is small.
                    let v := shr(49, w)
                    let l := shl(3, lt(0xff, v))
                    l := add(or(l, byte(and(0x1f, shr(shr(l, v), 0x8421084210842108cc6318c6db6d54be)),
                        0x0706060506020504060203020504030106050205030304010505030400000000)), 49)
                    w := sdiv(shl(l, 7), byte(sub(l, 31), 0x0303030303030303040506080c13))
                    c := gt(l, 60)
                    i := add(2, add(gt(l, 53), c))
                }
            } else {
                int256 ll = lnWad(w = lnWad(w));
                /// @solidity memory-safe-assembly
                assembly {
                    // `w = ln(x) - ln(ln(x)) + b * ln(ln(x)) / ln(x)`.
                    w := add(sdiv(mul(ll, 1023715080943847266), w), sub(w, ll))
                    i := add(3, iszero(shr(68, x)))
                    c := iszero(shr(143, x))
                }
                if (c == uint256(0)) {
                    do { // If `x` is big, use Newton's so that intermediate values won't overflow.
                        int256 e = expWad(w);
                        /// @solidity memory-safe-assembly
                        assembly {
                            let t := mul(w, div(e, wad))
                            w := sub(w, sdiv(sub(t, x), div(add(e, t), wad)))
                        }
                        if (p <= w) break;
                        p = w;
                    } while (--i != uint256(0));
                    /// @solidity memory-safe-assembly
                    assembly {
                        w := sub(w, sgt(w, 2))
                    }
                    return w;
                }
            }
            do { // Otherwise, use Halley's for faster convergence.
                int256 e = expWad(w);
                /// @solidity memory-safe-assembly
                assembly {
                    let t := add(w, wad)
                    let s := sub(mul(w, e), mul(x, wad))
                    w := sub(w, sdiv(mul(s, wad), sub(mul(e, t), sdiv(mul(add(t, wad), s), add(t, t)))))
                }
                if (p <= w) break;
                p = w;
            } while (--i != c);
            /// @solidity memory-safe-assembly
            assembly {
                w := sub(w, sgt(w, 2))
            }
            // For certain ranges of `x`, we'll use the quadratic-rate recursive formula of
            // R. Iacono and J.P. Boyd for the last iteration, to avoid catastrophic cancellation.
            if (c == uint256(0)) return w;
            int256 t = w | 1;
            /// @solidity memory-safe-assembly
            assembly {
                x := sdiv(mul(x, wad), t)
            }
            x = (t * (wad + lnWad(x)));
            /// @solidity memory-safe-assembly
            assembly {
                w := sdiv(x, add(wad, t))
            }
        }
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                  GENERAL NUMBER UTILITIES                  */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Returns `a * b == x * y`, with full precision.
    function fullMulEq(uint256 a, uint256 b, uint256 x, uint256 y)
        internal
        pure
        returns (bool result)
    {
        /// @solidity memory-safe-assembly
        assembly {
            result := and(eq(mul(a, b), mul(x, y)), eq(mulmod(x, y, not(0)), mulmod(a, b, not(0))))
        }
    }

    /// @dev Calculates `floor(x * y / d)` with full precision.
    /// Throws if result overflows a uint256 or when `d` is zero.
    /// Credit to Remco Bloemen under MIT license: https://2π.com/21/muldiv
    function fullMulDiv(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            // 512-bit multiply `[p1 p0] = x * y`.
            // Compute the product mod `2**256` and mod `2**256 - 1`
            // then use the Chinese Remainder Theorem to reconstruct
            // the 512 bit result. The result is stored in two 256
            // variables such that `product = p1 * 2**256 + p0`.

            // Temporarily use `z` as `p0` to save gas.
            z := mul(x, y) // Lower 256 bits of `x * y`.
            for {} 1 {} {
                // If overflows.
                if iszero(mul(or(iszero(x), eq(div(z, x), y)), d)) {
                    let mm := mulmod(x, y, not(0))
                    let p1 := sub(mm, add(z, lt(mm, z))) // Upper 256 bits of `x * y`.

                    /*------------------- 512 by 256 division --------------------*/

                    // Make division exact by subtracting the remainder from `[p1 p0]`.
                    let r := mulmod(x, y, d) // Compute remainder using mulmod.
                    let t := and(d, sub(0, d)) // The least significant bit of `d`. `t >= 1`.
                    // Make sure `z` is less than `2**256`. Also prevents `d == 0`.
                    // Placing the check here seems to give more optimal stack operations.
                    if iszero(gt(d, p1)) {
                        mstore(0x00, 0xae47f702) // `FullMulDivFailed()`.
                        revert(0x1c, 0x04)
                    }
                    d := div(d, t) // Divide `d` by `t`, which is a power of two.
                    // Invert `d mod 2**256`
                    // Now that `d` is an odd number, it has an inverse
                    // modulo `2**256` such that `d * inv = 1 mod 2**256`.
                    // Compute the inverse by starting with a seed that is correct
                    // correct for four bits. That is, `d * inv = 1 mod 2**4`.
                    let inv := xor(2, mul(3, d))
                    // Now use Newton-Raphson iteration to improve the precision.
                    // Thanks to Hensel's lifting lemma, this also works in modular
                    // arithmetic, doubling the correct bits in each step.
                    inv := mul(inv, sub(2, mul(d, inv))) // inverse mod 2**8
                    inv := mul(inv, sub(2, mul(d, inv))) // inverse mod 2**16
                    inv := mul(inv, sub(2, mul(d, inv))) // inverse mod 2**32
                    inv := mul(inv, sub(2, mul(d, inv))) // inverse mod 2**64
                    inv := mul(inv, sub(2, mul(d, inv))) // inverse mod 2**128
                    z :=
                        mul(
                            // Divide [p1 p0] by the factors of two.
                            // Shift in bits from `p1` into `p0`. For this we need
                            // to flip `t` such that it is `2**256 / t`.
                            or(mul(sub(p1, gt(r, z)), add(div(sub(0, t), t), 1)), div(sub(z, r), t)),
                            mul(sub(2, mul(d, inv)), inv) // inverse mod 2**256
                        )
                    break
                }
                z := div(z, d)
                break
            }
        }
    }

    /// @dev Calculates `floor(x * y / d)` with full precision.
    /// Behavior is undefined if `d` is zero or the final result cannot fit in 256 bits.
    /// Performs the full 512 bit calculation regardless.
    function fullMulDivUnchecked(uint256 x, uint256 y, uint256 d)
        internal
        pure
        returns (uint256 z)
    {
        /// @solidity memory-safe-assembly
        assembly {
            z := mul(x, y)
            let mm := mulmod(x, y, not(0))
            let p1 := sub(mm, add(z, lt(mm, z)))
            let t := and(d, sub(0, d))
            let r := mulmod(x, y, d)
            d := div(d, t)
            let inv := xor(2, mul(3, d))
            inv := mul(inv, sub(2, mul(d, inv)))
            inv := mul(inv, sub(2, mul(d, inv)))
            inv := mul(inv, sub(2, mul(d, inv)))
            inv := mul(inv, sub(2, mul(d, inv)))
            inv := mul(inv, sub(2, mul(d, inv)))
            z :=
                mul(
                    or(mul(sub(p1, gt(r, z)), add(div(sub(0, t), t), 1)), div(sub(z, r), t)),
                    mul(sub(2, mul(d, inv)), inv)
                )
        }
    }

    /// @dev Calculates `floor(x * y / d)` with full precision, rounded up.
    /// Throws if result overflows a uint256 or when `d` is zero.
    /// Credit to Uniswap-v3-core under MIT license:
    /// https://github.com/Uniswap/v3-core/blob/main/contracts/libraries/FullMath.sol
    function fullMulDivUp(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
        z = fullMulDiv(x, y, d);
        /// @solidity memory-safe-assembly
        assembly {
            if mulmod(x, y, d) {
                z := add(z, 1)
                if iszero(z) {
                    mstore(0x00, 0xae47f702) // `FullMulDivFailed()`.
                    revert(0x1c, 0x04)
                }
            }
        }
    }

    /// @dev Calculates `floor(x * y / 2 ** n)` with full precision.
    /// Throws if result overflows a uint256.
    /// Credit to Philogy under MIT license:
    /// https://github.com/SorellaLabs/angstrom/blob/main/contracts/src/libraries/X128MathLib.sol
    function fullMulDivN(uint256 x, uint256 y, uint8 n) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            // Temporarily use `z` as `p0` to save gas.
            z := mul(x, y) // Lower 256 bits of `x * y`. We'll call this `z`.
            for {} 1 {} {
                if iszero(or(iszero(x), eq(div(z, x), y))) {
                    let k := and(n, 0xff) // `n`, cleaned.
                    let mm := mulmod(x, y, not(0))
                    let p1 := sub(mm, add(z, lt(mm, z))) // Upper 256 bits of `x * y`.
                    //         |      p1     |      z     |
                    // Before: | p1_0 ¦ p1_1 | z_0  ¦ z_1 |
                    // Final:  |   0  ¦ p1_0 | p1_1 ¦ z_0 |
                    // Check that final `z` doesn't overflow by checking that p1_0 = 0.
                    if iszero(shr(k, p1)) {
                        z := add(shl(sub(256, k), p1), shr(k, z))
                        break
                    }
                    mstore(0x00, 0xae47f702) // `FullMulDivFailed()`.
                    revert(0x1c, 0x04)
                }
                z := shr(and(n, 0xff), z)
                break
            }
        }
    }

    /// @dev Returns `floor(x * y / d)`.
    /// Reverts if `x * y` overflows, or `d` is zero.
    function mulDiv(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := mul(x, y)
            // Equivalent to `require(d != 0 && (y == 0 || x <= type(uint256).max / y))`.
            if iszero(mul(or(iszero(x), eq(div(z, x), y)), d)) {
                mstore(0x00, 0xad251c27) // `MulDivFailed()`.
                revert(0x1c, 0x04)
            }
            z := div(z, d)
        }
    }

    /// @dev Returns `ceil(x * y / d)`.
    /// Reverts if `x * y` overflows, or `d` is zero.
    function mulDivUp(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := mul(x, y)
            // Equivalent to `require(d != 0 && (y == 0 || x <= type(uint256).max / y))`.
            if iszero(mul(or(iszero(x), eq(div(z, x), y)), d)) {
                mstore(0x00, 0xad251c27) // `MulDivFailed()`.
                revert(0x1c, 0x04)
            }
            z := add(iszero(iszero(mod(z, d))), div(z, d))
        }
    }

    /// @dev Returns `x`, the modular multiplicative inverse of `a`, such that `(a * x) % n == 1`.
    function invMod(uint256 a, uint256 n) internal pure returns (uint256 x) {
        /// @solidity memory-safe-assembly
        assembly {
            let g := n
            let r := mod(a, n)
            for { let y := 1 } 1 {} {
                let q := div(g, r)
                let t := g
                g := r
                r := sub(t, mul(r, q))
                let u := x
                x := y
                y := sub(u, mul(y, q))
                if iszero(r) { break }
            }
            x := mul(eq(g, 1), add(x, mul(slt(x, 0), n)))
        }
    }

    /// @dev Returns `ceil(x / d)`.
    /// Reverts if `d` is zero.
    function divUp(uint256 x, uint256 d) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            if iszero(d) {
                mstore(0x00, 0x65244e4e) // `DivFailed()`.
                revert(0x1c, 0x04)
            }
            z := add(iszero(iszero(mod(x, d))), div(x, d))
        }
    }

    /// @dev Returns `max(0, x - y)`. Alias for `saturatingSub`.
    function zeroFloorSub(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := mul(gt(x, y), sub(x, y))
        }
    }

    /// @dev Returns `max(0, x - y)`.
    function saturatingSub(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := mul(gt(x, y), sub(x, y))
        }
    }

    /// @dev Returns `min(2 ** 256 - 1, x + y)`.
    function saturatingAdd(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := or(sub(0, lt(add(x, y), x)), add(x, y))
        }
    }

    /// @dev Returns `min(2 ** 256 - 1, x * y)`.
    function saturatingMul(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := or(sub(or(iszero(x), eq(div(mul(x, y), x), y)), 1), mul(x, y))
        }
    }

    /// @dev Returns `condition ? x : y`, without branching.
    function ternary(bool condition, uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := xor(x, mul(xor(x, y), iszero(condition)))
        }
    }

    /// @dev Returns `condition ? x : y`, without branching.
    function ternary(bool condition, bytes32 x, bytes32 y) internal pure returns (bytes32 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := xor(x, mul(xor(x, y), iszero(condition)))
        }
    }

    /// @dev Returns `condition ? x : y`, without branching.
    function ternary(bool condition, address x, address y) internal pure returns (address z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := xor(x, mul(xor(x, y), iszero(condition)))
        }
    }

    /// @dev Returns `x != 0 ? x : y`, without branching.
    function coalesce(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := or(x, mul(y, iszero(x)))
        }
    }

    /// @dev Returns `x != bytes32(0) ? x : y`, without branching.
    function coalesce(bytes32 x, bytes32 y) internal pure returns (bytes32 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := or(x, mul(y, iszero(x)))
        }
    }

    /// @dev Returns `x != address(0) ? x : y`, without branching.
    function coalesce(address x, address y) internal pure returns (address z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := or(x, mul(y, iszero(shl(96, x))))
        }
    }

    /// @dev Exponentiate `x` to `y` by squaring, denominated in base `b`.
    /// Reverts if the computation overflows.
    function rpow(uint256 x, uint256 y, uint256 b) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := mul(b, iszero(y)) // `0 ** 0 = 1`. Otherwise, `0 ** n = 0`.
            if x {
                z := xor(b, mul(xor(b, x), and(y, 1))) // `z = isEven(y) ? scale : x`
                let half := shr(1, b) // Divide `b` by 2.
                // Divide `y` by 2 every iteration.
                for { y := shr(1, y) } y { y := shr(1, y) } {
                    let xx := mul(x, x) // Store x squared.
                    let xxRound := add(xx, half) // Round to the nearest number.
                    // Revert if `xx + half` overflowed, or if `x ** 2` overflows.
                    if or(lt(xxRound, xx), shr(128, x)) {
                        mstore(0x00, 0x49f7642b) // `RPowOverflow()`.
                        revert(0x1c, 0x04)
                    }
                    x := div(xxRound, b) // Set `x` to scaled `xxRound`.
                    // If `y` is odd:
                    if and(y, 1) {
                        let zx := mul(z, x) // Compute `z * x`.
                        let zxRound := add(zx, half) // Round to the nearest number.
                        // If `z * x` overflowed or `zx + half` overflowed:
                        if or(xor(div(zx, x), z), lt(zxRound, zx)) {
                            // Revert if `x` is non-zero.
                            if x {
                                mstore(0x00, 0x49f7642b) // `RPowOverflow()`.
                                revert(0x1c, 0x04)
                            }
                        }
                        z := div(zxRound, b) // Return properly scaled `zxRound`.
                    }
                }
            }
        }
    }

    /// @dev Returns the square root of `x`, rounded down.
    function sqrt(uint256 x) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            // `floor(sqrt(2**15)) = 181`. `sqrt(2**15) - 181 = 2.84`.
            z := 181 // The "correct" value is 1, but this saves a multiplication later.

            // This segment is to get a reasonable initial estimate for the Babylonian method. With a bad
            // start, the correct # of bits increases ~linearly each iteration instead of ~quadratically.

            // Let `y = x / 2**r`. We check `y >= 2**(k + 8)`
            // but shift right by `k` bits to ensure that if `x >= 256`, then `y >= 256`.
            let r := shl(7, lt(0xffffffffffffffffffffffffffffffffff, x))
            r := or(r, shl(6, lt(0xffffffffffffffffff, shr(r, x))))
            r := or(r, shl(5, lt(0xffffffffff, shr(r, x))))
            r := or(r, shl(4, lt(0xffffff, shr(r, x))))
            z := shl(shr(1, r), z)

            // Goal was to get `z*z*y` within a small factor of `x`. More iterations could
            // get y in a tighter range. Currently, we will have y in `[256, 256*(2**16))`.
            // We ensured `y >= 256` so that the relative difference between `y` and `y+1` is small.
            // That's not possible if `x < 256` but we can just verify those cases exhaustively.

            // Now, `z*z*y <= x < z*z*(y+1)`, and `y <= 2**(16+8)`, and either `y >= 256`, or `x < 256`.
            // Correctness can be checked exhaustively for `x < 256`, so we assume `y >= 256`.
            // Then `z*sqrt(y)` is within `sqrt(257)/sqrt(256)` of `sqrt(x)`, or about 20bps.

            // For `s` in the range `[1/256, 256]`, the estimate `f(s) = (181/1024) * (s+1)`
            // is in the range `(1/2.84 * sqrt(s), 2.84 * sqrt(s))`,
            // with largest error when `s = 1` and when `s = 256` or `1/256`.

            // Since `y` is in `[256, 256*(2**16))`, let `a = y/65536`, so that `a` is in `[1/256, 256)`.
            // Then we can estimate `sqrt(y)` using
            // `sqrt(65536) * 181/1024 * (a + 1) = 181/4 * (y + 65536)/65536 = 181 * (y + 65536)/2**18`.

            // There is no overflow risk here since `y < 2**136` after the first branch above.
            z := shr(18, mul(z, add(shr(r, x), 65536))) // A `mul()` is saved from starting `z` at 181.

            // Given the worst case multiplicative error of 2.84 above, 7 iterations should be enough.
            z := shr(1, add(z, div(x, z)))
            z := shr(1, add(z, div(x, z)))
            z := shr(1, add(z, div(x, z)))
            z := shr(1, add(z, div(x, z)))
            z := shr(1, add(z, div(x, z)))
            z := shr(1, add(z, div(x, z)))
            z := shr(1, add(z, div(x, z)))

            // If `x+1` is a perfect square, the Babylonian method cycles between
            // `floor(sqrt(x))` and `ceil(sqrt(x))`. This statement ensures we return floor.
            // See: https://en.wikipedia.org/wiki/Integer_square_root#Using_only_integer_division
            z := sub(z, lt(div(x, z), z))
        }
    }

    /// @dev Returns the cube root of `x`, rounded down.
    /// Credit to bout3fiddy and pcaversaccio under AGPLv3 license:
    /// https://github.com/pcaversaccio/snekmate/blob/main/src/snekmate/utils/math.vy
    /// Formally verified by xuwinnie:
    /// https://github.com/vectorized/solady/blob/main/audits/xuwinnie-solady-cbrt-proof.pdf
    function cbrt(uint256 x) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            let r := shl(7, lt(0xffffffffffffffffffffffffffffffff, x))
            r := or(r, shl(6, lt(0xffffffffffffffff, shr(r, x))))
            r := or(r, shl(5, lt(0xffffffff, shr(r, x))))
            r := or(r, shl(4, lt(0xffff, shr(r, x))))
            r := or(r, shl(3, lt(0xff, shr(r, x))))
            // Makeshift lookup table to nudge the approximate log2 result.
            z := div(shl(div(r, 3), shl(lt(0xf, shr(r, x)), 0xf)), xor(7, mod(r, 3)))
            // Newton-Raphson's.
            z := div(add(add(div(x, mul(z, z)), z), z), 3)
            z := div(add(add(div(x, mul(z, z)), z), z), 3)
            z := div(add(add(div(x, mul(z, z)), z), z), 3)
            z := div(add(add(div(x, mul(z, z)), z), z), 3)
            z := div(add(add(div(x, mul(z, z)), z), z), 3)
            z := div(add(add(div(x, mul(z, z)), z), z), 3)
            z := div(add(add(div(x, mul(z, z)), z), z), 3)
            // Round down.
            z := sub(z, lt(div(x, mul(z, z)), z))
        }
    }

    /// @dev Returns the square root of `x`, denominated in `WAD`, rounded down.
    function sqrtWad(uint256 x) internal pure returns (uint256 z) {
        unchecked {
            if (x <= type(uint256).max / 10 ** 18) return sqrt(x * 10 ** 18);
            z = (1 + sqrt(x)) * 10 ** 9;
            z = (fullMulDivUnchecked(x, 10 ** 18, z) + z) >> 1;
        }
        /// @solidity memory-safe-assembly
        assembly {
            z := sub(z, gt(999999999999999999, sub(mulmod(z, z, x), 1))) // Round down.
        }
    }

    /// @dev Returns the cube root of `x`, denominated in `WAD`, rounded down.
    /// Formally verified by xuwinnie:
    /// https://github.com/vectorized/solady/blob/main/audits/xuwinnie-solady-cbrt-proof.pdf
    function cbrtWad(uint256 x) internal pure returns (uint256 z) {
        unchecked {
            if (x <= type(uint256).max / 10 ** 36) return cbrt(x * 10 ** 36);
            z = (1 + cbrt(x)) * 10 ** 12;
            z = (fullMulDivUnchecked(x, 10 ** 36, z * z) + z + z) / 3;
        }
        /// @solidity memory-safe-assembly
        assembly {
            let p := x
            for {} 1 {} {
                if iszero(shr(229, p)) {
                    if iszero(shr(199, p)) {
                        p := mul(p, 100000000000000000) // 10 ** 17.
                        break
                    }
                    p := mul(p, 100000000) // 10 ** 8.
                    break
                }
                if iszero(shr(249, p)) { p := mul(p, 100) }
                break
            }
            let t := mulmod(mul(z, z), z, p)
            z := sub(z, gt(lt(t, shr(1, p)), iszero(t))) // Round down.
        }
    }

    /// @dev Returns the factorial of `x`.
    function factorial(uint256 x) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := 1
            if iszero(lt(x, 58)) {
                mstore(0x00, 0xaba0f2a2) // `FactorialOverflow()`.
                revert(0x1c, 0x04)
            }
            for {} x { x := sub(x, 1) } { z := mul(z, x) }
        }
    }

    /// @dev Returns the log2 of `x`.
    /// Equivalent to computing the index of the most significant bit (MSB) of `x`.
    /// Returns 0 if `x` is zero.
    function log2(uint256 x) internal pure returns (uint256 r) {
        /// @solidity memory-safe-assembly
        assembly {
            r := shl(7, lt(0xffffffffffffffffffffffffffffffff, x))
            r := or(r, shl(6, lt(0xffffffffffffffff, shr(r, x))))
            r := or(r, shl(5, lt(0xffffffff, shr(r, x))))
            r := or(r, shl(4, lt(0xffff, shr(r, x))))
            r := or(r, shl(3, lt(0xff, shr(r, x))))
            // forgefmt: disable-next-item
            r := or(r, byte(and(0x1f, shr(shr(r, x), 0x8421084210842108cc6318c6db6d54be)),
                0x0706060506020504060203020504030106050205030304010505030400000000))
        }
    }

    /// @dev Returns the log2 of `x`, rounded up.
    /// Returns 0 if `x` is zero.
    function log2Up(uint256 x) internal pure returns (uint256 r) {
        r = log2(x);
        /// @solidity memory-safe-assembly
        assembly {
            r := add(r, lt(shl(r, 1), x))
        }
    }

    /// @dev Returns the log10 of `x`.
    /// Returns 0 if `x` is zero.
    function log10(uint256 x) internal pure returns (uint256 r) {
        /// @solidity memory-safe-assembly
        assembly {
            if iszero(lt(x, 100000000000000000000000000000000000000)) {
                x := div(x, 100000000000000000000000000000000000000)
                r := 38
            }
            if iszero(lt(x, 100000000000000000000)) {
                x := div(x, 100000000000000000000)
                r := add(r, 20)
            }
            if iszero(lt(x, 10000000000)) {
                x := div(x, 10000000000)
                r := add(r, 10)
            }
            if iszero(lt(x, 100000)) {
                x := div(x, 100000)
                r := add(r, 5)
            }
            r := add(r, add(gt(x, 9), add(gt(x, 99), add(gt(x, 999), gt(x, 9999)))))
        }
    }

    /// @dev Returns the log10 of `x`, rounded up.
    /// Returns 0 if `x` is zero.
    function log10Up(uint256 x) internal pure returns (uint256 r) {
        r = log10(x);
        /// @solidity memory-safe-assembly
        assembly {
            r := add(r, lt(exp(10, r), x))
        }
    }

    /// @dev Returns the log256 of `x`.
    /// Returns 0 if `x` is zero.
    function log256(uint256 x) internal pure returns (uint256 r) {
        /// @solidity memory-safe-assembly
        assembly {
            r := shl(7, lt(0xffffffffffffffffffffffffffffffff, x))
            r := or(r, shl(6, lt(0xffffffffffffffff, shr(r, x))))
            r := or(r, shl(5, lt(0xffffffff, shr(r, x))))
            r := or(r, shl(4, lt(0xffff, shr(r, x))))
            r := or(shr(3, r), lt(0xff, shr(r, x)))
        }
    }

    /// @dev Returns the log256 of `x`, rounded up.
    /// Returns 0 if `x` is zero.
    function log256Up(uint256 x) internal pure returns (uint256 r) {
        r = log256(x);
        /// @solidity memory-safe-assembly
        assembly {
            r := add(r, lt(shl(shl(3, r), 1), x))
        }
    }

    /// @dev Returns the scientific notation format `mantissa * 10 ** exponent` of `x`.
    /// Useful for compressing prices (e.g. using 25 bit mantissa and 7 bit exponent).
    function sci(uint256 x) internal pure returns (uint256 mantissa, uint256 exponent) {
        /// @solidity memory-safe-assembly
        assembly {
            mantissa := x
            if mantissa {
                if iszero(mod(mantissa, 1000000000000000000000000000000000)) {
                    mantissa := div(mantissa, 1000000000000000000000000000000000)
                    exponent := 33
                }
                if iszero(mod(mantissa, 10000000000000000000)) {
                    mantissa := div(mantissa, 10000000000000000000)
                    exponent := add(exponent, 19)
                }
                if iszero(mod(mantissa, 1000000000000)) {
                    mantissa := div(mantissa, 1000000000000)
                    exponent := add(exponent, 12)
                }
                if iszero(mod(mantissa, 1000000)) {
                    mantissa := div(mantissa, 1000000)
                    exponent := add(exponent, 6)
                }
                if iszero(mod(mantissa, 10000)) {
                    mantissa := div(mantissa, 10000)
                    exponent := add(exponent, 4)
                }
                if iszero(mod(mantissa, 100)) {
                    mantissa := div(mantissa, 100)
                    exponent := add(exponent, 2)
                }
                if iszero(mod(mantissa, 10)) {
                    mantissa := div(mantissa, 10)
                    exponent := add(exponent, 1)
                }
            }
        }
    }

    /// @dev Convenience function for packing `x` into a smaller number using `sci`.
    /// The `mantissa` will be in bits [7..255] (the upper 249 bits).
    /// The `exponent` will be in bits [0..6] (the lower 7 bits).
    /// Use `SafeCastLib` to safely ensure that the `packed` number is small
    /// enough to fit in the desired unsigned integer type:
    /// ```
    ///     uint32 packed = SafeCastLib.toUint32(FixedPointMathLib.packSci(777 ether));
    /// ```
    function packSci(uint256 x) internal pure returns (uint256 packed) {
        (x, packed) = sci(x); // Reuse for `mantissa` and `exponent`.
        /// @solidity memory-safe-assembly
        assembly {
            if shr(249, x) {
                mstore(0x00, 0xce30380c) // `MantissaOverflow()`.
                revert(0x1c, 0x04)
            }
            packed := or(shl(7, x), packed)
        }
    }

    /// @dev Convenience function for unpacking a packed number from `packSci`.
    function unpackSci(uint256 packed) internal pure returns (uint256 unpacked) {
        unchecked {
            unpacked = (packed >> 7) * 10 ** (packed & 0x7f);
        }
    }

    /// @dev Returns the average of `x` and `y`. Rounds towards zero.
    function avg(uint256 x, uint256 y) internal pure returns (uint256 z) {
        unchecked {
            z = (x & y) + ((x ^ y) >> 1);
        }
    }

    /// @dev Returns the average of `x` and `y`. Rounds towards negative infinity.
    function avg(int256 x, int256 y) internal pure returns (int256 z) {
        unchecked {
            z = (x >> 1) + (y >> 1) + (x & y & 1);
        }
    }

    /// @dev Returns the absolute value of `x`.
    function abs(int256 x) internal pure returns (uint256 z) {
        unchecked {
            z = (uint256(x) + uint256(x >> 255)) ^ uint256(x >> 255);
        }
    }

    /// @dev Returns the absolute distance between `x` and `y`.
    function dist(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := add(xor(sub(0, gt(x, y)), sub(y, x)), gt(x, y))
        }
    }

    /// @dev Returns the absolute distance between `x` and `y`.
    function dist(int256 x, int256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := add(xor(sub(0, sgt(x, y)), sub(y, x)), sgt(x, y))
        }
    }

    /// @dev Returns the minimum of `x` and `y`.
    function min(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := xor(x, mul(xor(x, y), lt(y, x)))
        }
    }

    /// @dev Returns the minimum of `x` and `y`.
    function min(int256 x, int256 y) internal pure returns (int256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := xor(x, mul(xor(x, y), slt(y, x)))
        }
    }

    /// @dev Returns the maximum of `x` and `y`.
    function max(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := xor(x, mul(xor(x, y), gt(y, x)))
        }
    }

    /// @dev Returns the maximum of `x` and `y`.
    function max(int256 x, int256 y) internal pure returns (int256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := xor(x, mul(xor(x, y), sgt(y, x)))
        }
    }

    /// @dev Returns `x`, bounded to `minValue` and `maxValue`.
    function clamp(uint256 x, uint256 minValue, uint256 maxValue)
        internal
        pure
        returns (uint256 z)
    {
        /// @solidity memory-safe-assembly
        assembly {
            z := xor(x, mul(xor(x, minValue), gt(minValue, x)))
            z := xor(z, mul(xor(z, maxValue), lt(maxValue, z)))
        }
    }

    /// @dev Returns `x`, bounded to `minValue` and `maxValue`.
    function clamp(int256 x, int256 minValue, int256 maxValue) internal pure returns (int256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := xor(x, mul(xor(x, minValue), sgt(minValue, x)))
            z := xor(z, mul(xor(z, maxValue), slt(maxValue, z)))
        }
    }

    /// @dev Returns greatest common divisor of `x` and `y`.
    function gcd(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            for { z := x } y {} {
                let t := y
                y := mod(z, y)
                z := t
            }
        }
    }

    /// @dev Returns `a + (b - a) * (t - begin) / (end - begin)`,
    /// with `t` clamped between `begin` and `end` (inclusive).
    /// Agnostic to the order of (`a`, `b`) and (`end`, `begin`).
    /// If `begins == end`, returns `t <= begin ? a : b`.
    function lerp(uint256 a, uint256 b, uint256 t, uint256 begin, uint256 end)
        internal
        pure
        returns (uint256)
    {
        if (begin > end) (t, begin, end) = (~t, ~begin, ~end);
        if (t <= begin) return a;
        if (t >= end) return b;
        unchecked {
            if (b >= a) return a + fullMulDiv(b - a, t - begin, end - begin);
            return a - fullMulDiv(a - b, t - begin, end - begin);
        }
    }

    /// @dev Returns `a + (b - a) * (t - begin) / (end - begin)`.
    /// with `t` clamped between `begin` and `end` (inclusive).
    /// Agnostic to the order of (`a`, `b`) and (`end`, `begin`).
    /// If `begins == end`, returns `t <= begin ? a : b`.
    function lerp(int256 a, int256 b, int256 t, int256 begin, int256 end)
        internal
        pure
        returns (int256)
    {
        if (begin > end) (t, begin, end) = (~t, ~begin, ~end);
        if (t <= begin) return a;
        if (t >= end) return b;
        // forgefmt: disable-next-item
        unchecked {
            if (b >= a) return int256(uint256(a) + fullMulDiv(uint256(b - a),
                uint256(t - begin), uint256(end - begin)));
            return int256(uint256(a) - fullMulDiv(uint256(a - b),
                uint256(t - begin), uint256(end - begin)));
        }
    }

    /// @dev Returns if `x` is an even number. Some people may need this.
    function isEven(uint256 x) internal pure returns (bool) {
        return x & uint256(1) == uint256(0);
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                   RAW NUMBER OPERATIONS                    */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Returns `x + y`, without checking for overflow.
    function rawAdd(uint256 x, uint256 y) internal pure returns (uint256 z) {
        unchecked {
            z = x + y;
        }
    }

    /// @dev Returns `x + y`, without checking for overflow.
    function rawAdd(int256 x, int256 y) internal pure returns (int256 z) {
        unchecked {
            z = x + y;
        }
    }

    /// @dev Returns `x - y`, without checking for underflow.
    function rawSub(uint256 x, uint256 y) internal pure returns (uint256 z) {
        unchecked {
            z = x - y;
        }
    }

    /// @dev Returns `x - y`, without checking for underflow.
    function rawSub(int256 x, int256 y) internal pure returns (int256 z) {
        unchecked {
            z = x - y;
        }
    }

    /// @dev Returns `x * y`, without checking for overflow.
    function rawMul(uint256 x, uint256 y) internal pure returns (uint256 z) {
        unchecked {
            z = x * y;
        }
    }

    /// @dev Returns `x * y`, without checking for overflow.
    function rawMul(int256 x, int256 y) internal pure returns (int256 z) {
        unchecked {
            z = x * y;
        }
    }

    /// @dev Returns `x / y`, returning 0 if `y` is zero.
    function rawDiv(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := div(x, y)
        }
    }

    /// @dev Returns `x / y`, returning 0 if `y` is zero.
    function rawSDiv(int256 x, int256 y) internal pure returns (int256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := sdiv(x, y)
        }
    }

    /// @dev Returns `x % y`, returning 0 if `y` is zero.
    function rawMod(uint256 x, uint256 y) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := mod(x, y)
        }
    }

    /// @dev Returns `x % y`, returning 0 if `y` is zero.
    function rawSMod(int256 x, int256 y) internal pure returns (int256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := smod(x, y)
        }
    }

    /// @dev Returns `(x + y) % d`, return 0 if `d` if zero.
    function rawAddMod(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := addmod(x, y, d)
        }
    }

    /// @dev Returns `(x * y) % d`, return 0 if `d` if zero.
    function rawMulMod(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := mulmod(x, y, d)
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

