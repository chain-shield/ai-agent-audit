
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.30;

import {ICovenant, MarketId, MarketParams, MarketState, SwapParams, RedeemParams, MintParams, SynthTokens, TokenPrices, AssetType} from "./interfaces/ICovenant.sol";
import {ILiquidExchangeModel} from "./interfaces/ILiquidExchangeModel.sol";
import {IERC20} from "@openzeppelin/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/token/ERC20/utils/SafeERC20.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/access/Ownable2Step.sol";
import {NoDelegateCall} from "./libraries/NoDelegateCall.sol";
import {ValidationLogic} from "./libraries/ValidationLogic.sol";
import {MarketParamsLib} from "./libraries/MarketParams.sol";
import {MulticallLib} from "./libraries/MultiCall.sol";
import {Errors} from "./libraries/Errors.sol";
import {Events} from "./libraries/Events.sol";

/**
 * @title Covenant contract
 * @author Covenant Labs
 **/

contract Covenant is ICovenant, NoDelegateCall, Ownable2Step {
    using MarketParamsLib for MarketParams;
    using SafeERC20 for IERC20;

    /// @inheritdoc ICovenant
    string public constant name = "Covenant V1.0";

    /////////////////////////////////////////////////////////////////////////////////////////////
    // Storage

    // Map of Liquid market params (marketID to data)
    mapping(MarketId marketId => MarketParams) internal idToMarketParams;

    // Map of Liquid base asset states (marketID to data)
    mapping(MarketId marketId => MarketState) internal marketState;

    // Map of valid Lending Exchanges
    mapping(address lex => bool) public isLexEnabled;

    // Map of valid curators
    mapping(address curator => bool) public isCuratorEnabled;

    // Default protocol fee
    uint32 private _defaultProtocolFee;

    // Default address authorized to pause markets
    address private _defaultPauseAddress;

    // Global flag to check if the contract is in a multicall
    bool private _isMulticall;

    /////////////////////////////////////////////////////////////////////////////////////////////
    // Modifiers

    uint8 constant STATE_UNINITIALIZED = 0;
    uint8 constant STATE_UNLOCKED = 1;
    uint8 constant STATE_LOCKED = 2;
    uint8 constant STATE_PAUSED = 3;

    /// @dev Mutually exclusive reentrancy protection into each Market.
    /// This method also prevents entrance to a function before the market is initialized.
    modifier lock(MarketId marketId) {
        {
            uint8 statusFlag = marketState[marketId].statusFlag;
            if (statusFlag != STATE_UNLOCKED) {
                if (statusFlag == STATE_LOCKED) revert Errors.E_MarketLocked();
                else if (statusFlag == STATE_PAUSED) revert Errors.E_MarketPaused();
                else revert Errors.E_MarketNonExistent();
            }
        }
        marketState[marketId].statusFlag = STATE_LOCKED;
        _;
        marketState[marketId].statusFlag = STATE_UNLOCKED;
    }

    // Prevent read only reentrancy
    // @dev - allows read operations on paused markets
    modifier lockView(MarketId marketId) {
        {
            uint8 statusFlag = marketState[marketId].statusFlag;
            if (statusFlag != STATE_UNLOCKED && statusFlag != STATE_PAUSED) {
                if (statusFlag == STATE_LOCKED) revert Errors.E_MarketLocked();
                else revert Errors.E_MarketNonExistent();
            }
        }
        _;
    }

    ////////////////////////////////////////////////////////////////////////////////////////
    // Constructor
    constructor(address initialOwner) Ownable(initialOwner) {
        _defaultPauseAddress = initialOwner;
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Getters

    /// @inheritdoc ICovenant
    function getIdToMarketParams(MarketId marketId) external view returns (MarketParams memory) {
        return idToMarketParams[marketId];
    }

    /// @inheritdoc ICovenant
    function getMarketState(MarketId marketId) external view returns (MarketState memory) {
        return marketState[marketId];
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // OnlyOwner functions

    /// @inheritdoc ICovenant
    function setEnabledLEX(address lex, bool isEnabled) external onlyOwner noDelegateCall {
        isLexEnabled[lex] = isEnabled;
        emit Events.UpdateEnabledLEX(lex, isEnabled);
    }

    /// @inheritdoc ICovenant
    function setEnabledCurator(address curator, bool isEnabled) external onlyOwner noDelegateCall {
        isCuratorEnabled[curator] = isEnabled;
        emit Events.UpdateEnabledOracle(curator, isEnabled);
    }

    /// @inheritdoc ICovenant
    function setDefaultFee(uint32 newFee) external onlyOwner noDelegateCall {
        // Validate protocol fee
        ValidationLogic.checkProtocolFee(newFee);

        uint32 oldFee = _defaultProtocolFee;
        _defaultProtocolFee = newFee;
        emit Events.UpdateDefaultProtocolFee(oldFee, newFee);
    }

    /// @inheritdoc ICovenant
    function setMarketProtocolFee(
        MarketId marketId,
        MarketParams calldata marketParams,
        bytes calldata data,
        uint256 msgValue,
        uint32 newFee
    ) external payable onlyOwner noDelegateCall lock(marketId) {
        // Validate protocol fee
        ValidationLogic.checkProtocolFee(newFee);

        // update market state beforehand to accrue any fees at current rate till now
        _updateState(marketId, marketParams, data, msgValue);

        uint32 oldFee = ILiquidExchangeModel(marketParams.lex).getProtocolFee(marketId);
        ILiquidExchangeModel(marketParams.lex).setMarketProtocolFee(marketId, newFee);
        emit Events.UpdateMarketProtocolFee(marketId, oldFee, newFee);
    }

    /// @inheritdoc ICovenant
    function collectProtocolFee(
        MarketId marketId,
        address recipient,
        uint128 amountRequested
    ) external onlyOwner noDelegateCall lock(marketId) {
        if (amountRequested == 0) revert Errors.E_ZeroAmount();
        if (recipient == address(0)) revert Errors.E_ZeroAddress();
        if (recipient == address(this)) revert Errors.E_Unauthorized();

        // update state
        uint128 accruedFees = marketState[marketId].protocolFeeGrowth;
        address baseToken = idToMarketParams[marketId].baseToken;
        if (amountRequested > accruedFees) amountRequested = accruedFees;
        unchecked {
            marketState[marketId].protocolFeeGrowth = accruedFees - amountRequested;
        }

        // transfer
        IERC20(baseToken).safeTransfer(recipient, amountRequested);

        // emit
        emit Events.CollectProtocolFee(marketId, recipient, baseToken, amountRequested);
    }

    /// @inheritdoc ICovenant
    function setMarketPause(MarketId marketId, bool isPaused) external noDelegateCall lockView(marketId) {
        // only authorized pause address can pause / unpause market
        if (_msgSender() != marketState[marketId].authorizedPauseAddress) revert Errors.E_Unauthorized();

        // @dev - lockView only allows operations on existing and unlocked markets
        marketState[marketId].statusFlag = isPaused ? STATE_PAUSED : STATE_UNLOCKED;
        emit Events.MarketPaused(marketId, isPaused);
    }

    /// @inheritdoc ICovenant
    function setDefaultPauseAddress(address newPauseAddress) external onlyOwner noDelegateCall {
        address oldPauseAddress = _defaultPauseAddress;
        _defaultPauseAddress = newPauseAddress;
        emit Events.UpdateDefaultPauseAddress(oldPauseAddress, newPauseAddress);
    }

    /// @inheritdoc ICovenant
    function setMarketPauseAddress(
        MarketId marketId,
        address newPauseAddress
    ) external noDelegateCall lockView(marketId) {
        // only owner or authorized pause address can set pause address for a market
        if (_msgSender() != owner() && _msgSender() != marketState[marketId].authorizedPauseAddress)
            revert Errors.E_Unauthorized();

        address oldPauseAddress = marketState[marketId].authorizedPauseAddress;
        marketState[marketId].authorizedPauseAddress = newPauseAddress;
        emit Events.UpdateMarketPauseAddress(marketId, oldPauseAddress, newPauseAddress);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // External Functions (mostly covering internal function and implementing a lock per market)

    /// @inheritdoc ICovenant
    function createMarket(
        MarketParams calldata marketParams,
        bytes calldata initData
    ) external noDelegateCall returns (MarketId) {
        // Calculate marketId
        MarketId marketId = marketParams.id();

        // validate marketParams
        ValidationLogic.checkMarketParams(marketParams, idToMarketParams[marketId], isLexEnabled, isCuratorEnabled);

        // initialize LEX for market
        (SynthTokens memory synthTokens, bytes memory lexData) = ILiquidExchangeModel(marketParams.lex).initMarket(
            marketId,
            marketParams,
            _defaultProtocolFee,
            initData
        );

        idToMarketParams[marketId] = marketParams;
        marketState[marketId].authorizedPauseAddress = _defaultPauseAddress;
        marketState[marketId].statusFlag = STATE_UNLOCKED; // unlock market to enable its us

        // emit new market creation event
        emit Events.CreateMarket(marketId, marketParams, synthTokens, initData, lexData);

        return marketId;
    }

    /// @inheritdoc ICovenant
    /// @notice - reverts if market is locked or non-existent
    function mint(
        MintParams calldata mintParams
    ) external payable override noDelegateCall lock(mintParams.marketId) returns (uint256, uint256) {
        // Caches
        MarketState storage ms = marketState[mintParams.marketId];
        MarketParams calldata mp = mintParams.marketParams;
        uint256 localBaseSupply = ms.baseSupply;

        // check payment
        _checkPayment(mintParams.msgValue);

        // Validate mint params
        ValidationLogic.checkMintParams(mintParams);

        // Mint synthTokens through LEX. @dev - accrues debt interest, mints aTokens/zTokens.
        // @dev - only passes msgValue to LEX (to enable multicall).  Does not check for overdeposit.
        (
            uint256 aTokenAmountOut,
            uint256 zTokenAmountOut,
            uint128 protocolFees,
            TokenPrices memory tokenPrices
        ) = ILiquidExchangeModel(mp.lex).mint{value: mintParams.msgValue}(mintParams, _msgSender(), localBaseSupply);

        // Validate mint amounts
        ValidationLogic.checkMintOutputs(mintParams, aTokenAmountOut, zTokenAmountOut);

        // emit mint event
        emit Events.Mint(
            mintParams.marketId,
            mintParams.baseAmountIn,
            _msgSender(),
            mintParams.to,
            aTokenAmountOut,
            zTokenAmountOut,
            protocolFees,
            tokenPrices
        );

        // Update market state (storage)
        ms.baseSupply = (localBaseSupply + mintParams.baseAmountIn) - protocolFees;
        if (protocolFees > 0) ms.protocolFeeGrowth += protocolFees;

        // Transfer base asset in
        IERC20(mp.baseToken).safeTransferFrom(_msgSender(), address(this), mintParams.baseAmountIn);

        return (aTokenAmountOut, zTokenAmountOut);
    }

    /// @inheritdoc ICovenant
    /// @notice - reverts if market is locked or non-existent
    function redeem(
        RedeemParams calldata redeemParams
    ) external payable override noDelegateCall lock(redeemParams.marketId) returns (uint256) {
        // Cache pointers
        MarketState storage ms = marketState[redeemParams.marketId];
        MarketParams calldata mp = redeemParams.marketParams;
        uint256 localBaseSupply = ms.baseSupply;

        // check payment
        _checkPayment(redeemParams.msgValue);

        // check redeemParams
        ValidationLogic.checkRedeemParams(redeemParams);

        // Redeem synthTokens through LEX. @dev - accrues debt interest, burns aTokens/zTokens.
        // @dev - only passes msgValue to LEX (to enable multicall).  Does not check for overdeposit.
        (uint256 amountOut, uint128 protocolFees, TokenPrices memory tokenPrices) = ILiquidExchangeModel(mp.lex).redeem{
            value: redeemParams.msgValue
        }(redeemParams, _msgSender(), localBaseSupply);

        // Validate redeem amounts
        ValidationLogic.checkRedeemOutputs(redeemParams, localBaseSupply, amountOut);

        // emit event
        emit Events.Redeem(
            redeemParams.marketId,
            redeemParams.aTokenAmountIn,
            redeemParams.zTokenAmountIn,
            _msgSender(),
            redeemParams.to,
            amountOut,
            protocolFees,
            tokenPrices
        );

        // Update market state (storage)
        ms.baseSupply = localBaseSupply - amountOut - protocolFees;
        if (protocolFees > 0) ms.protocolFeeGrowth += protocolFees;

        // Transfer base asset out
        IERC20(mp.baseToken).safeTransfer(redeemParams.to, amountOut);

        // return
        return amountOut;
    }

    /// @inheritdoc ICovenant
    /// @notice - reverts if market is locked or non-existent
    /// @dev if assetIn or assetOut is the base token, then swap will perform a mint / redeem in addition to a swap
    function swap(
        SwapParams calldata swapParams
    ) external payable override noDelegateCall lock(swapParams.marketId) returns (uint256) {
        // Caches
        MarketState storage ms = marketState[swapParams.marketId];
        MarketParams calldata mp = swapParams.marketParams;
        uint256 localBaseSupply = ms.baseSupply;

        // check payment
        _checkPayment(swapParams.msgValue);

        // check swapParams
        ValidationLogic.checkSwapParams(swapParams, localBaseSupply);

        // Swap synthTokens through LEX. @dev - accrues debt interest, burns/mints aTokens/zTokens when needed
        // @dev - only passes msgValue to LEX (to enable multicall).  Does not check for overdeposit.
        (uint256 amountCalculated, uint128 protocolFees, TokenPrices memory tokenPrices) = ILiquidExchangeModel(mp.lex)
            .swap{value: swapParams.msgValue}(swapParams, _msgSender(), localBaseSupply);

        // Validate swap amounts
        ValidationLogic.checkSwapOutputs(swapParams, localBaseSupply, amountCalculated, protocolFees);

        // emit event
        emit Events.Swap(
            swapParams.marketId,
            swapParams.assetIn,
            swapParams.assetOut,
            swapParams.isExactIn ? swapParams.amountSpecified : amountCalculated,
            swapParams.isExactIn ? amountCalculated : swapParams.amountSpecified,
            _msgSender(),
            swapParams.to,
            protocolFees,
            tokenPrices
        );

        // Update market state (storage) and transfer if swap involves base tokens
        if (protocolFees > 0) ms.protocolFeeGrowth += protocolFees;
        if (swapParams.assetIn == AssetType.BASE) {
            uint256 amount = swapParams.isExactIn ? swapParams.amountSpecified : amountCalculated;
            // update state
            ms.baseSupply = localBaseSupply + amount - protocolFees;
            // transfer
            IERC20(mp.baseToken).safeTransferFrom(_msgSender(), address(this), amount);
        } else if (swapParams.assetOut == AssetType.BASE) {
            uint256 amount = swapParams.isExactIn ? amountCalculated : swapParams.amountSpecified;
            // update state
            ms.baseSupply = localBaseSupply - amount - protocolFees;
            // transfer
            IERC20(mp.baseToken).safeTransfer(swapParams.to, amount);
        } else if (protocolFees > 0) {
            ms.baseSupply = localBaseSupply - protocolFees;
        }

        return amountCalculated;
    }

    /// @inheritdoc ICovenant
    function updateState(
        MarketId marketId,
        MarketParams calldata marketParams,
        bytes calldata data,
        uint256 msgValue
    ) external payable noDelegateCall lock(marketId) {
        // update state
        _updateState(marketId, marketParams, data, msgValue);
    }

    /// @inheritdoc ICovenant
    function previewMint(
        MintParams calldata mintParams
    )
        external
        view
        override
        noDelegateCall
        lockView(mintParams.marketId)
        returns (
            uint256 aTokenAmountOut,
            uint256 zTokenAmountOut,
            uint128 protocolFees,
            uint128 oracleUpdateFee,
            TokenPrices memory tokenPrices
        )
    {
        // Caches
        MarketState storage ms = marketState[mintParams.marketId];
        MarketParams calldata mp = mintParams.marketParams;
        uint256 localBaseSupply = ms.baseSupply;

        // Validate mint params
        ValidationLogic.checkMintParams(mintParams);

        // Mint synthTokens through LEX. @dev - accrues debt interest, mints aTokens/zTokens.
        (aTokenAmountOut, zTokenAmountOut, protocolFees, oracleUpdateFee, tokenPrices) = ILiquidExchangeModel(mp.lex)
            .quoteMint(mintParams, _msgSender(), localBaseSupply);

        // Validate mint amounts
        ValidationLogic.checkMintOutputs(mintParams, aTokenAmountOut, zTokenAmountOut);
    }

    /// @inheritdoc ICovenant
    function previewRedeem(
        RedeemParams calldata redeemParams
    )
        external
        view
        override
        noDelegateCall
        lockView(redeemParams.marketId)
        returns (uint256 amountOut, uint128 protocolFees, uint128 oracleUpdateFee, TokenPrices memory tokenPrices)
    {
        // Cache pointers
        MarketState storage ms = marketState[redeemParams.marketId];
        MarketParams calldata mp = redeemParams.marketParams;
        uint256 localBaseSupply = ms.baseSupply;

        // check redeemParams
        ValidationLogic.checkRedeemParams(redeemParams);

        // Redeem synthTokens through LEX. @dev - accrues debt interest, burns aTokens/zTokens.
        (amountOut, protocolFees, oracleUpdateFee, tokenPrices) = ILiquidExchangeModel(mp.lex).quoteRedeem(
            redeemParams,
            _msgSender(),
            localBaseSupply
        );

        // Validate redeem amounts
        ValidationLogic.checkRedeemOutputs(redeemParams, localBaseSupply, amountOut);
    }

    /// @inheritdoc ICovenant
    function previewSwap(
        SwapParams calldata swapParams
    )
        external
        view
        override
        noDelegateCall
        lockView(swapParams.marketId)
        returns (
            uint256 amountCalculated,
            uint128 protocolFees,
            uint128 oracleUpdateFee,
            TokenPrices memory tokenPrices
        )
    {
        // Caches
        MarketState storage ms = marketState[swapParams.marketId];
        MarketParams calldata mp = swapParams.marketParams;
        uint256 localBaseSupply = ms.baseSupply;

        // check swapParams
        ValidationLogic.checkSwapParams(swapParams, localBaseSupply);

        // Swap synthTokens through LEX. @dev - accrues debt interest, burns/mints aTokens/zTokens when needed
        (amountCalculated, protocolFees, oracleUpdateFee, tokenPrices) = ILiquidExchangeModel(mp.lex).quoteSwap(
            swapParams,
            _msgSender(),
            localBaseSupply
        );

        // Validate swap amounts
        ValidationLogic.checkSwapOutputs(swapParams, localBaseSupply, amountCalculated, protocolFees);
    }

    /// @inheritdoc ICovenant
    function multicall(bytes[] calldata data) external payable returns (bytes[] memory results) {
        // @dev - runs in the context of this call, and hence inherits msg.value
        // @dev - does not check whether msg.value is enough for tx to succeed
        // if not enough, it reverts.  If msg.value higher than needed, it also reverts (does not allow excess deposit)
        // @dev - code does not guard against reentrancy overdeposits.
        // @dev - code looks to protect common users from overdeposits, not malicious donations....
        uint256 balanceBeforeCall = address(this).balance - msg.value;

        // perform multicall
        _isMulticall = true;
        results = MulticallLib.multicall(data);
        _isMulticall = false;

        // validate balance (do not allow over or under deposit)
        if (address(this).balance != balanceBeforeCall) revert Errors.E_IncorrectPayment();

        return results;
    }

    //////////////////////////////////////////////////////////////////////////
    // private

    function _updateState(
        MarketId marketId,
        MarketParams calldata marketParams,
        bytes calldata data,
        uint256 msgValue
    ) private {
        // check payment
        _checkPayment(msgValue);

        // check update params
        ValidationLogic.checkUpdateParams(marketId, marketParams);

        // update LEX state (pass msgValue to LEX)
        uint128 protocolFees = ILiquidExchangeModel(marketParams.lex).updateState{value: msgValue}(
            marketId,
            marketParams,
            marketState[marketId].baseSupply,
            data
        );

        // Update market state (storage)
        if (protocolFees > 0) {
            marketState[marketId].protocolFeeGrowth += protocolFees;
            marketState[marketId].baseSupply -= protocolFees;
        }
    }

    function _checkPayment(uint256 msgValue) private {
        // check for overdeposit (if not multicall)
        // @dev - allows user to underdeposit (will revert if not enough balance in contract)
        if (msg.value != msgValue && !_isMulticall) revert Errors.E_IncorrectPayment();
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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

contract NoDelegateCall {
    address private immutable originalAddress;
    error E_DelegateCallNotAllowed();

    constructor() {
        originalAddress = address(this);
    }

    modifier noDelegateCall() {
        if (address(this) != originalAddress) revert E_DelegateCallNotAllowed();
        _;
    }
}

// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.30;

import {MarketId, MarketParams, SynthTokens, TokenPrices, AssetType} from "../interfaces/ICovenant.sol";

library Events {
    /**
     * @dev Emitted on market creation
     * @param marketId the market ID
     * @param marketParams the market params
     * @param initData additional data passed to LEX during initialization
     * @param lexData additional data returned by LEX during initialization (ABI encoded)
     **/
    event CreateMarket(
        MarketId indexed marketId,
        MarketParams marketParams,
        SynthTokens synthTokens,
        bytes initData,
        bytes lexData
    );

    /**
     * @dev Emitted on mint
     * @notice Event incorporates minimal price information (from which all prices can be derived)
     * @param marketId the market indicating the aTokens / zTokens to mint given baseToken (there could be more than one market for the same baseToken)
     * @param baseAmountIn the amount of base token to deposit (and against which to mint a and z tokens)
     * @param sender the supplier of base Tokens
     * @param receiver the receiver of aTokens and zTokens
     * @param aTokenAmountOut amount of aToken minted
     * @param zTokenAmountOut amount of zToken minted
     * @param tokenPrices Prices, in WADS, of baseToken, aToken and zToken after action (denominated in Quote tokens)
     **/
    event Mint(
        MarketId indexed marketId,
        uint256 baseAmountIn,
        address indexed sender,
        address indexed receiver,
        uint256 aTokenAmountOut,
        uint256 zTokenAmountOut,
        uint128 protocolFees,
        TokenPrices tokenPrices
    );

    /**
     * @dev Emitted on redeem
     * @param marketId the market for which the aTokens / zTokens will be redeemed for baseToken
     * @param aTokenAmountIn the aTokenAmount being redeemed / burned (exact in)
     * @param zTokenAmountIn the zTokenAmount being redeemed / burned (exact in)
     * @param sender the supplier of a and z tokens
     * @param receiver the receiver of base tokens
     * @param amountOut amount of base tokens sent out to receiver
     * @param tokenPrices Prices, in WADS, of baseToken, aToken and zToken after action (denominated in Quote tokens)
     **/
    event Redeem(
        MarketId indexed marketId,
        uint256 aTokenAmountIn,
        uint256 zTokenAmountIn,
        address indexed sender,
        address indexed receiver,
        uint256 amountOut,
        uint128 protocolFees,
        TokenPrices tokenPrices
    );

    /**
     * @dev Emitted on swap
     * @param marketId the market in which the swap is executed
     * @param assetIn type of token swapped in
     * @param assetOut type of token swapped out
     * @param amountIn amount of tokenIn received and burned by market during swap
     * @param amountOut amount of tokenOut minted and sent by market during swap
     * @param sender the supplier of tokenIn
     * @param receiver the receiver of tokenOut
     * @param tokenPrices Prices, in WADS, of baseToken, aToken and zToken after action (denominated in Quote tokens)
     **/
    event Swap(
        MarketId indexed marketId,
        AssetType assetIn,
        AssetType assetOut,
        uint256 amountIn,
        uint256 amountOut,
        address indexed sender,
        address indexed receiver,
        uint128 protocolFees,
        TokenPrices tokenPrices
    );

    /**
     * @dev Emitted on LEX update
     * @param LEXImplementationAddress address of lex logic implementation
     * @param isEnabled whether the address is a valid implementation for new markets
     **/
    event UpdateEnabledLEX(address indexed LEXImplementationAddress, bool isEnabled);

    /**
     * @dev Emitted on Oracle update
     * @param oracle address of oracle
     * @param isEnabled whether the address is a valid oracle for new markets
     **/
    event UpdateEnabledOracle(address indexed oracle, bool isEnabled);

    /**
     * @dev Emitted on update of default protocol fee
     * @param oldDefaultFee old default fee
     * @param newDefaultFee new default fee
     **/
    event UpdateDefaultProtocolFee(uint32 oldDefaultFee, uint32 newDefaultFee);

    /**
     * @dev Emitted on update of a market protocol fee
     * @param marketId market being updated
     * @param oldMarketFee old default fee
     * @param newMarketFee new default fee
     **/
    event UpdateMarketProtocolFee(MarketId indexed marketId, uint32 oldMarketFee, uint32 newMarketFee);

    /**
     * @dev Emitted when protocol fees are collected
     * @param marketId market from which fees are being collected
     * @param recipient recipient of collected fees
     * @param asset asset in which fees are denominated
     * @param amount amount collected
     **/
    event CollectProtocolFee(MarketId indexed marketId, address recipient, address asset, uint128 amount);

    /**
     * @dev Emitted when protocol is paused or unpaused
     * @param marketId market from which fees are being collected
     * @param isPaused whether the market is paused
     **/
    event MarketPaused(MarketId indexed marketId, bool isPaused);

    /**
     * @dev Emitted when default pause address is updated
     * @param oldDefaultPauseAddress old default pause address
     * @param newDefaultPauseAddress new default pause address
     **/
    event UpdateDefaultPauseAddress(address oldDefaultPauseAddress, address newDefaultPauseAddress);

    /**
     * @dev Emitted when authorized pause address for a market is updated
     * @param marketId market from which pause address is being updated
     * @param oldPauseAddress old authorized pause address
     * @param newPauseAddress new authorized pause address
     **/
    event UpdateMarketPauseAddress(MarketId indexed marketId, address oldPauseAddress, address newPauseAddress);
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

import {MarketId, MarketParams} from "../interfaces/ICovenant.sol";

/// @title MarketParams Library
/// @author Covenant Labs
/// @notice Library to convert a market to its id.
library MarketParamsLib {
    /// @notice Returns the id of the market `marketParams`.
    function id(MarketParams calldata p) internal pure returns (MarketId marketParamsId) {
        return
            MarketId.wrap(
                bytes20(uint160(uint256(keccak256(abi.encodePacked(p.baseToken, p.quoteToken, p.curator, p.lex)))))
            );
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.3.0) (utils/MultiCall.sol)

// CovenantLabs - modified to make contract payable
// @dev - use with caution, all calls will get same msg.value and should not be relied on
// @dev - cannot be used by ERC2771Contexts

pragma solidity ^0.8.30;

import {Address} from "@openzeppelin/utils/Address.sol";
import {Context} from "@openzeppelin/utils/Context.sol";

/**
 * @dev Provides a function to batch together multiple calls in a single external call.
 *
 * Consider any assumption about calldata validation performed by the sender may be violated if it's not especially
 * careful about sending transactions invoking {multicall}. For example, a relay address that filters function
 * selectors won't filter calls nested within a {multicall} operation.
 *
 */
library MulticallLib {
    /**
     * @dev Receives and executes a batch of function calls on this contract.
     * @custom:oz-upgrades-unsafe-allow-reachable delegatecall
     */
    function multicall(bytes[] calldata data) internal returns (bytes[] memory results) {
        results = new bytes[](data.length);
        for (uint256 i = 0; i < data.length; i++) {
            results[i] = Address.functionDelegateCall(address(this), data[i]);
        }
        return results;
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.30;

import {Errors} from "./Errors.sol";
import {MarketId, MarketParams, SwapParams, MintParams, RedeemParams, AssetType} from "../interfaces/ICovenant.sol";
import {MarketParamsLib} from "./MarketParams.sol";
import {UtilsLib} from "./Utils.sol";

/**
 * @title ValidationLogic library
 * @author Covenant Labs
 * @notice Implements functions to validate the different actions of the protocol
 */
library ValidationLogic {
    using MarketParamsLib for MarketParams;

    function checkUpdateParams(MarketId marketId, MarketParams calldata marketParams) internal pure {
        // check marketParams
        if (MarketId.unwrap(marketId) != MarketId.unwrap(marketParams.id())) revert Errors.E_IncorrectMarketParams();
    }

    function checkMintParams(MintParams calldata mintParams) internal pure {
        // check marketParams
        if (MarketId.unwrap(mintParams.marketId) != MarketId.unwrap(mintParams.marketParams.id()))
            revert Errors.E_IncorrectMarketParams();

        // check mintParams
        if (mintParams.baseAmountIn == 0) revert Errors.E_ZeroAmount();
        if (mintParams.to == address(0)) revert Errors.E_ZeroAddress();
    }

    function checkMintOutputs(
        MintParams calldata mintParams,
        uint256 aTokenAmountOut,
        uint256 zTokenAmountOut
    ) internal pure {
        if (aTokenAmountOut < mintParams.minATokenAmountOut) revert Errors.E_CrossedLimit();
        if (zTokenAmountOut < mintParams.minZTokenAmountOut) revert Errors.E_CrossedLimit();
        if (zTokenAmountOut == 0 && aTokenAmountOut == 0) revert Errors.E_InsufficientAmount();
    }

    function checkRedeemParams(RedeemParams calldata redeemParams) internal pure {
        // check marketParams
        if (MarketId.unwrap(redeemParams.marketId) != MarketId.unwrap(redeemParams.marketParams.id()))
            revert Errors.E_IncorrectMarketParams();

        // check redeemParams
        if (redeemParams.aTokenAmountIn == 0 && redeemParams.zTokenAmountIn == 0) revert Errors.E_ZeroAmount();
        if (redeemParams.to == address(0)) revert Errors.E_ZeroAddress();
    }

    function checkRedeemOutputs(
        RedeemParams calldata redeemParams,
        uint256 baseSupply,
        uint256 amountOut
    ) internal pure {
        if (amountOut < redeemParams.minAmountOut) revert Errors.E_CrossedLimit();
        if (amountOut > baseSupply) revert Errors.E_InsufficientAmount();
        if (amountOut == 0) revert Errors.E_InsufficientAmount();
    }

    function checkSwapParams(SwapParams calldata swapParams, uint256 baseSupply) internal pure {
        // check marketParams
        if (MarketId.unwrap(swapParams.marketId) != MarketId.unwrap(swapParams.marketParams.id()))
            revert Errors.E_IncorrectMarketParams();

        // check swapParams
        if (swapParams.amountSpecified == 0) revert Errors.E_ZeroAmount();
        if (swapParams.to == address(0)) revert Errors.E_ZeroAddress();
        if (swapParams.assetOut == swapParams.assetIn) revert Errors.E_EqualSwapAssets();
        if (
            (uint8(swapParams.assetOut) >= uint8(AssetType.COUNT)) ||
            (uint8(swapParams.assetIn) >= uint8(AssetType.COUNT))
        ) revert Errors.E_IncorrectMarketAsset();

        // check if requesting more base tokens than available
        if (
            !swapParams.isExactIn &&
            (swapParams.assetOut == AssetType.BASE) &&
            (swapParams.amountSpecified > baseSupply)
        ) revert Errors.E_InsufficientAmount();
    }

    function checkSwapOutputs(
        SwapParams calldata swapParams,
        uint256 baseSupply,
        uint256 amountCalculated,
        uint256 protocolFees
    ) internal pure {
        if (amountCalculated == 0) {
            if (!swapParams.isExactIn) revert Errors.E_InsufficientAmount();
            // Do not allow 0 input if this is an exactOut swap
            else if (swapParams.assetIn == AssetType.BASE) revert Errors.E_InsufficientAmount(); //  Do not allow zero out swaps with BASE token as input
            // @dev - the above conditions allow exactIn swaps where a Synth token is donated in, but no Base tokens come out (0 output)
            // This is to allow donation of valueless synth dust to the Covenant Protocol.
        }

        // check amounts do not surpass swapParam limits
        if (swapParams.isExactIn) {
            if (amountCalculated < swapParams.amountLimit) revert Errors.E_CrossedLimit(); // check minimum limit is coming out
        } else {
            if (amountCalculated > swapParams.amountLimit) revert Errors.E_CrossedLimit(); // check less than max limit is coming in
        }

        // if Base asset out check base supply limits
        if (
            (swapParams.assetOut == AssetType.BASE) &&
            (((swapParams.isExactIn ? amountCalculated : swapParams.amountSpecified) + protocolFees) > baseSupply)
        ) revert Errors.E_InsufficientAmount();
    }

    function checkMarketParams(
        MarketParams calldata marketParams,
        MarketParams storage storageMarketParams,
        mapping(address LEXimplementation => bool) storage validLEX,
        mapping(address Oracle => bool) storage validCurator
    ) internal view {
        // check whether lex is enabled
        if (!validLEX[address(marketParams.lex)]) revert Errors.E_LEXimplementationNotAuthorized();

        // check whether curator is enabled
        if (!validCurator[address(marketParams.curator)]) revert Errors.E_CuratorNotAuthorized();

        // check whether market already exists
        if (address(storageMarketParams.baseToken) != address(0)) revert Errors.E_MarketAlreadyExists();
    }

    function checkProtocolFee(uint32 protocolFee) internal pure {
        // split out fees
        (uint16 yieldFee, uint16 tvlFee) = UtilsLib.decodeFee(protocolFee);

        if (yieldFee > 3000) revert Errors.E_ProtocolFeeTooHigh(); // 30% max of yield as additional fee
        if (tvlFee > 500) revert Errors.E_ProtocolFeeTooHigh(); // 5% max of tvl as yearly fee
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


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {Covenant} from "../src/Covenant.sol";
import {SynthToken} from "../src/synths/SynthToken.sol";

contract DeployCovenant is Script {
    function run() external {
        vm.startBroadcast();

        Covenant covenantLiquidCore = new Covenant(msg.sender);
        console.log("Covenant Liquid address %s", address(covenantLiquidCore));

        vm.stopBroadcast();
    }
}

