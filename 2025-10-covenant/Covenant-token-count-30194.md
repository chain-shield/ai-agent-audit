
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

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.1) (utils/Context.sol)

pragma solidity ^0.8.20;

/**
 * @dev Provides information about the current execution context, including the
 * sender of the transaction and its data. While these are generally available
 * via msg.sender and msg.data, they should not be accessed in such a direct
 * manner, since when dealing with meta-transactions the account sending and
 * paying for execution may not be the actual sender (as far as an application
 * is concerned).
 *
 * This contract is only required for intermediate, library-like contracts.
 */
abstract contract Context {
    function _msgSender() internal view virtual returns (address) {
        return msg.sender;
    }

    function _msgData() internal view virtual returns (bytes calldata) {
        return msg.data;
    }

    function _contextSuffixLength() internal view virtual returns (uint256) {
        return 0;
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.0) (access/Ownable.sol)

pragma solidity ^0.8.20;

import {Context} from "../utils/Context.sol";

/**
 * @dev Contract module which provides a basic access control mechanism, where
 * there is an account (an owner) that can be granted exclusive access to
 * specific functions.
 *
 * The initial owner is set to the address provided by the deployer. This can
 * later be changed with {transferOwnership}.
 *
 * This module is used through inheritance. It will make available the modifier
 * `onlyOwner`, which can be applied to your functions to restrict their use to
 * the owner.
 */
abstract contract Ownable is Context {
    address private _owner;

    /**
     * @dev The caller account is not authorized to perform an operation.
     */
    error OwnableUnauthorizedAccount(address account);

    /**
     * @dev The owner is not a valid owner account. (eg. `address(0)`)
     */
    error OwnableInvalidOwner(address owner);

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    /**
     * @dev Initializes the contract setting the address provided by the deployer as the initial owner.
     */
    constructor(address initialOwner) {
        if (initialOwner == address(0)) {
            revert OwnableInvalidOwner(address(0));
        }
        _transferOwnership(initialOwner);
    }

    /**
     * @dev Throws if called by any account other than the owner.
     */
    modifier onlyOwner() {
        _checkOwner();
        _;
    }

    /**
     * @dev Returns the address of the current owner.
     */
    function owner() public view virtual returns (address) {
        return _owner;
    }

    /**
     * @dev Throws if the sender is not the owner.
     */
    function _checkOwner() internal view virtual {
        if (owner() != _msgSender()) {
            revert OwnableUnauthorizedAccount(_msgSender());
        }
    }

    /**
     * @dev Leaves the contract without owner. It will not be possible to call
     * `onlyOwner` functions. Can only be called by the current owner.
     *
     * NOTE: Renouncing ownership will leave the contract without an owner,
     * thereby disabling any functionality that is only available to the owner.
     */
    function renounceOwnership() public virtual onlyOwner {
        _transferOwnership(address(0));
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    function transferOwnership(address newOwner) public virtual onlyOwner {
        if (newOwner == address(0)) {
            revert OwnableInvalidOwner(address(0));
        }
        _transferOwnership(newOwner);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Internal function without access restriction.
     */
    function _transferOwnership(address newOwner) internal virtual {
        address oldOwner = _owner;
        _owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
    }
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

