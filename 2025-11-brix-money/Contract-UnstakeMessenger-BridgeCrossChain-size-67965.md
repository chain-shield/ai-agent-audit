
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import {OAppSender, MessagingFee, MessagingReceipt} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oapp/OAppSender.sol";
import {OAppCore} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oapp/OAppCore.sol";
import {OAppOptionsType3} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oapp/libs/OAppOptionsType3.sol";
import {OptionsBuilder} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oapp/libs/OptionsBuilder.sol";
import {MessagingParams} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "./interfaces/IUnstakeMessenger.sol";

/**
 * @title UnstakeMessenger
 * @notice User-facing contract on spoke chains for initiating crosschain unstaking operations
 *
 * @dev Part of the iTRY crosschain unstaking system. Key responsibilities:
 *      - Fee quoting: Provides accurate spoke→hub message fees via two quote functions
 *      - Fee validation: Ensures caller sends sufficient native tokens for round-trip messaging
 *      - Native value forwarding: Embeds returnTripAllocation in LayerZero options for hub execution
 *      - Security: Enforces msg.sender as user address (prevents authorization spoofing)
 *      - Refund handling: Returns excess msg.value to user after message dispatch
 *
 * @dev User Flow:
 *      1. Client queries wiTryVaultComposer.quoteUnstakeReturn() on hub to get hub→spoke return fee
 *      2. Client queries quoteUnstakeWithBuffer() (recommended) or quoteUnstakeWithReturnValue() (exact)
 *         - quoteUnstakeWithBuffer(): Applies feeBufferBPS safety margin (e.g., 10%) for gas fluctuations
 *         - quoteUnstakeWithReturnValue(): Returns exact fee without buffer
 *      3. User calls unstake(returnTripAllocation) with quoted total as msg.value
 *      4. Contract calculates spoke→hub fee with embedded returnTripAllocation
 *      5. Contract validates msg.value ≥ total fee, dispatches message, refunds excess to user
 *      6. Hub receives returnTripAllocation as native value for return trip execution
 *
 * @dev Fee Architecture:
 *      - Single msg.value payment covers both message directions (spoke→hub + hub→spoke)
 *      - returnTripAllocation is fixed parameter (not calculated from msg.value remainder)
 *      - Spoke→hub fee = LayerZero messaging cost + returnTripAllocation embedded in options
 *      - Contract embeds returnTripAllocation via addExecutorLzReceiveOption (native value forwarding)
 *      - Hub receives exact returnTripAllocation for return message; hub refunds excess to wiTryVaultComposer
 *      - Spoke refunds any msg.value excess (buffer) to user immediately after dispatch
 *
 * @dev Configuration:
 *      - feeBufferBPS: Recommended safety buffer (500-5000 BPS = 5-50%), adjustable by owner
 *      - hubEid: Immutable hub chain endpoint ID, set at deployment
 *      - peers[hubEid]: Trusted wiTryVaultComposer address on hub (bytes32), set via setPeer()
 */
contract UnstakeMessenger is OAppSender, OAppOptionsType3, ReentrancyGuard, IUnstakeMessenger {
    using OptionsBuilder for bytes;
    using SafeERC20 for IERC20;

    // ============ Constants ============

    /// @notice Message type identifier for unstake operations
    uint16 public constant MSG_TYPE_UNSTAKE = 1;

    /// @notice Basis points denominator for percentage calculations
    /// @dev Standard DeFi convention: 10000 BPS = 100%
    uint256 public constant BPS_DENOMINATOR = 10000;

    /// @notice Gas limit for lzReceive on hub chain
    /// @dev Must be non-zero to satisfy LayerZero executor requirements
    uint128 internal constant LZ_RECEIVE_GAS = 350000;

    // ============ State Variables ============

    /// @notice Fee buffer in basis points to protect against gas price fluctuations
    /// @dev Configurable to adapt to different network conditions
    /// @dev Default 10% (1000 bps) follows LayerZero team guidance
    uint256 public feeBufferBPS = 1000;

    /// @notice Endpoint ID of the hub chain (immutable after deployment)
    uint32 public immutable hubEid;

    // ============ Constructor ============

    /**
     * @notice Initialize UnstakeMessenger contract
     * @param _endpoint LayerZero endpoint address on this chain
     * @param _owner Contract owner address
     * @param _hubEid Endpoint ID of the hub chain
     */
    constructor(address _endpoint, address _owner, uint32 _hubEid) OAppCore(_endpoint, _owner) {
        if (_hubEid == 0) revert HubNotConfigured();
        hubEid = _hubEid;
    }

    // ============ External Functions ============

    /**
     * @notice Initiate crosschain unstaking operation
     * @dev User address is always msg.sender (prevents authorization spoofing)
     *      Encodes msg.sender as user in UnstakeMessage
     *      Validates hub peer is configured
     *      Uses fixed returnTripAllocation to avoid circular fee dependency
     *      Protected against reentrancy with nonReentrant modifier
     *
     * @param returnTripAllocation Exact native value to forward to hub for return trip (in wei)
     *        This should be the result of calling  wiTryVaultComposer.quoteUnstakeReturn()
     *
     * @return guid Unique identifier for this LayerZero message
     *
     * @dev Usage:
     *      1. Call wiTryVaultComposer.quoteUnstakeReturn(user, amount, spokeDstEid) on hub
     *      2. Call quoteUnstakeWithReturnValue(returnTripAllocation) or quoteUnstakeWithBuffer(returnTripAllocation) on spoke
     *      3. Call unstake{value: quotedTotal}(returnTripAllocation)
     */
    function unstake(uint256 returnTripAllocation) external payable nonReentrant returns (bytes32 guid) {
        // Validate hub peer configured
        bytes32 hubPeer = peers[hubEid];
        if (hubPeer == bytes32(0)) revert HubNotConfigured();

        // Validate returnTripAllocation
        if (returnTripAllocation == 0) revert InvalidReturnTripAllocation();

        // Build return trip options (valid TYPE_3 header)
        bytes memory extraOptions = OptionsBuilder.newOptions();

        // Encode UnstakeMessage with msg.sender as user (prevents spoofing)
        UnstakeMessage memory message = UnstakeMessage({user: msg.sender, extraOptions: extraOptions});
        bytes memory payload = abi.encode(MSG_TYPE_UNSTAKE, message);

        // Build options WITH native value forwarding for return trip execution
        // casting to 'uint128' is safe because returnTripAllocation value will be less than 2^128
        // forge-lint: disable-next-line(unsafe-typecast)
        bytes memory callerOptions =
            OptionsBuilder.newOptions().addExecutorLzReceiveOption(LZ_RECEIVE_GAS, uint128(returnTripAllocation));
        bytes memory options = _combineOptions(hubEid, MSG_TYPE_UNSTAKE, callerOptions);

        // Quote with native drop included (single quote with fixed returnTripAllocation)
        MessagingFee memory fee = _quote(hubEid, payload, options, false);

        // Validate caller sent enough
        if (msg.value < fee.nativeFee) {
            revert InsufficientFee(fee.nativeFee, msg.value);
        }

        // Automatic refund to msg.sender
        MessagingReceipt memory receipt = _lzSend(
            hubEid,
            payload,
            options,
            fee,
            payable(msg.sender) // Refund excess to user
        );
        guid = receipt.guid;

        emit UnstakeRequested(msg.sender, hubEid, fee.nativeFee, msg.value - fee.nativeFee, guid);

        return guid;
    }

    /**
     * @notice Quote exact fee for unstake WITH specified return trip allocation
     * @dev Returns the precise spoke→hub message fee with embedded native value forwarding.
     *      The contract will forward exactly returnTripValue to hub for the return trip.
     *
     * @dev Usage pattern:
     *      1. Call wiTryVaultComposer.quoteUnstakeReturn() to get hub→spoke return fee
     *      2. Call this function with that fee as returnTripValue
     *      3. Send the returned nativeFee as msg.value to unstake()
     *
     * @param returnTripValue Exact amount in wei to forward to hub for return trip
     * @return nativeFee Total spoke→hub message fee WITH returnTripValue embedded
     * @return lzTokenFee Fee in LayerZero token (typically 0)
     */
    function quoteUnstakeWithReturnValue(uint256 returnTripValue)
        external
        view
        returns (uint256 nativeFee, uint256 lzTokenFee)
    {
        // Build dummy UnstakeMessage for quoting
        UnstakeMessage memory dummyMessage =
            UnstakeMessage({user: address(0), extraOptions: OptionsBuilder.newOptions()});

        bytes memory payload = abi.encode(MSG_TYPE_UNSTAKE, dummyMessage);

        // Build options WITH specified native value
        // This matches what unstake() will do when msg.value = total
        bytes memory callerOptions = OptionsBuilder.newOptions().addExecutorLzReceiveOption(LZ_RECEIVE_GAS, uint128(returnTripValue));
        bytes memory options = _combineOptions(hubEid, MSG_TYPE_UNSTAKE, callerOptions);

        // Quote with native value included
        MessagingFee memory fee = _quote(hubEid, payload, options, false);

        return (fee.nativeFee, fee.lzTokenFee);
    }

    /**
     * @notice Quote recommended fee for unstake WITH buffer applied
     * @dev Recommended for standard integrations. Automatically applies feeBufferBPS
     *      safety buffer to protect against gas price fluctuations between quote and execution.
     *
     * @dev Usage pattern:
     *      1. Call wiTryVaultComposer.quoteUnstakeReturn() to get hub→spoke return fee
     *      2. Call this function with that fee as returnTripValue
     *      3. Send the returned recommendedFee as msg.value to unstake()
     *      4. Contract uses exact fees; excess buffer refunded to user
     *
     * @param returnTripValue Exact amount in wei to forward to hub for return trip
     * @return recommendedFee Recommended total fee with buffer applied
     * @return lzTokenFee Fee in LayerZero token (typically 0)
     */
    function quoteUnstakeWithBuffer(uint256 returnTripValue)
        external
        view
        returns (uint256 recommendedFee, uint256 lzTokenFee)
    {
        // Get exact fee with return trip value
        (uint256 nativeFee, uint256 lzTokenFeeExact) = this.quoteUnstakeWithReturnValue(returnTripValue);

        // Apply buffer: fee * (10000 + bufferBPS) / 10000
        // Example: 1 ETH fee with 1000 BPS (10%) = 1.1 ETH recommended
        recommendedFee = (nativeFee * (BPS_DENOMINATOR + feeBufferBPS)) / BPS_DENOMINATOR;
        lzTokenFee = lzTokenFeeExact;

        return (recommendedFee, lzTokenFee);
    }

    /**
     * @notice Set trusted peer for crosschain messaging
     * @dev Overrides OAppCore to restrict configuration to hub chain only
     *      Only owner can set peer
     *      Reverts if eid does not equal hubEid
     *      Reverts if peer is zero address
     * @param eid Endpoint ID (must equal hubEid)
     * @param peer Peer address on remote chain (bytes32 format)
     */
    function setPeer(uint32 eid, bytes32 peer) public override(OAppCore, IUnstakeMessenger) onlyOwner {
        require(eid == hubEid, "UnstakeMessenger: Invalid endpoint");
        require(peer != bytes32(0), "UnstakeMessenger: Invalid peer");

        super.setPeer(eid, peer);
    }

    /**
     * @notice Get the configured hub peer address
     * @return peer Hub peer address in bytes32 format
     */
    function getHubPeer() external view returns (bytes32 peer) {
        return peers[hubEid];
    }

    /**
     * @notice Update fee buffer percentage
     * @dev Only owner can adjust buffer to respond to network conditions
     * @param newBufferBPS New buffer in basis points (e.g., 1000 = 10%)
     */
    function setFeeBufferBPS(uint256 newBufferBPS) external onlyOwner {
        require(newBufferBPS >= 500, "Buffer too low (min 5%)");
        require(newBufferBPS <= 5000, "Buffer too high (max 50%)");

        uint256 oldBuffer = feeBufferBPS;
        feeBufferBPS = newBufferBPS;

        emit FeeBufferUpdated(oldBuffer, newBufferBPS);
    }

    /**
     * @notice Allow contract to receive ETH refunds from LayerZero
     * @dev Required because endpoint.send() refunds to address(this)
     */
    receive() external payable {
        // Accept LayerZero refunds silently
        // Owner can rescue if needed
    }

    /**
     * @notice Rescue tokens accidentally sent to this contract
     * @dev Only callable by owner. Can rescue both ERC20 tokens and native ETH
     *      Use address(0) for rescuing ETH
     * @param token The token address to rescue (use address(0) for ETH)
     * @param to The address to send rescued tokens to
     * @param amount The amount to rescue
     */
    function rescueToken(address token, address to, uint256 amount) external onlyOwner nonReentrant {
        if (to == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();

        if (token == address(0)) {
            // Rescue ETH
            (bool success,) = to.call{value: amount}("");
            if (!success) revert TransferFailed();
        } else {
            // Rescue ERC20 tokens
            IERC20(token).safeTransfer(to, amount);
        }

        emit TokenRescued(token, to, amount);
    }

    /**
     * @notice Internal helper to combine options (works with memory instead of calldata)
     * @dev Wraps the public combineOptions function to work with memory bytes
     * @param _eid Destination endpoint ID
     * @param _msgType Message type
     * @param _extraOptions Extra options in memory
     * @return Combined options bytes
     */
    function _combineOptions(uint32 _eid, uint16 _msgType, bytes memory _extraOptions)
        internal
        view
        returns (bytes memory)
    {
        bytes memory enforced = enforcedOptions[_eid][_msgType];

        // No enforced options, return extra options
        if (enforced.length == 0) {
            return _extraOptions;
        }

        // No extra options, return enforced options
        if (_extraOptions.length <= 2) {
            return enforced;
        }

        // Combine: enforced options + extra options (skip TYPE_3 header from extra)
        return bytes.concat(enforced, _slice(_extraOptions, 2, _extraOptions.length - 2));
    }

    /**
     * @notice Slice bytes array
     * @param _bytes Bytes to slice
     * @param _start Start index
     * @param _length Length to slice
     * @return Sliced bytes
     */
    function _slice(bytes memory _bytes, uint256 _start, uint256 _length) internal pure returns (bytes memory) {
        bytes memory result = new bytes(_length);
        for (uint256 i = 0; i < _length; i++) {
            result[i] = _bytes[_start + i];
        }
        return result;
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import { SafeERC20, IERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { MessagingParams, MessagingFee, MessagingReceipt } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import { OAppCore } from "./OAppCore.sol";

/**
 * @title OAppSender
 * @dev Abstract contract implementing the OAppSender functionality for sending messages to a LayerZero endpoint.
 */
abstract contract OAppSender is OAppCore {
    using SafeERC20 for IERC20;

    // Custom error messages
    error NotEnoughNative(uint256 msgValue);
    error LzTokenUnavailable();

    // @dev The version of the OAppSender implementation.
    // @dev Version is bumped when changes are made to this contract.
    uint64 internal constant SENDER_VERSION = 1;

    /**
     * @notice Retrieves the OApp version information.
     * @return senderVersion The version of the OAppSender.sol contract.
     * @return receiverVersion The version of the OAppReceiver.sol contract.
     *
     * @dev Providing 0 as the default for OAppReceiver version. Indicates that the OAppReceiver is not implemented.
     * ie. this is a SEND only OApp.
     * @dev If the OApp uses both OAppSender and OAppReceiver, then this needs to be override returning the correct versions
     */
    function oAppVersion() public view virtual returns (uint64 senderVersion, uint64 receiverVersion) {
        return (SENDER_VERSION, 0);
    }

    /**
     * @dev Internal function to interact with the LayerZero EndpointV2.quote() for fee calculation.
     * @param _dstEid The destination endpoint ID.
     * @param _message The message payload.
     * @param _options Additional options for the message.
     * @param _payInLzToken Flag indicating whether to pay the fee in LZ tokens.
     * @return fee The calculated MessagingFee for the message.
     *      - nativeFee: The native fee for the message.
     *      - lzTokenFee: The LZ token fee for the message.
     */
    function _quote(
        uint32 _dstEid,
        bytes memory _message,
        bytes memory _options,
        bool _payInLzToken
    ) internal view virtual returns (MessagingFee memory fee) {
        return
            endpoint.quote(
                MessagingParams(_dstEid, _getPeerOrRevert(_dstEid), _message, _options, _payInLzToken),
                address(this)
            );
    }

    /**
     * @dev Internal function to interact with the LayerZero EndpointV2.send() for sending a message.
     * @param _dstEid The destination endpoint ID.
     * @param _message The message payload.
     * @param _options Additional options for the message.
     * @param _fee The calculated LayerZero fee for the message.
     *      - nativeFee: The native fee.
     *      - lzTokenFee: The lzToken fee.
     * @param _refundAddress The address to receive any excess fee values sent to the endpoint.
     * @return receipt The receipt for the sent message.
     *      - guid: The unique identifier for the sent message.
     *      - nonce: The nonce of the sent message.
     *      - fee: The LayerZero fee incurred for the message.
     */
    function _lzSend(
        uint32 _dstEid,
        bytes memory _message,
        bytes memory _options,
        MessagingFee memory _fee,
        address _refundAddress
    ) internal virtual returns (MessagingReceipt memory receipt) {
        // @dev Push corresponding fees to the endpoint, any excess is sent back to the _refundAddress from the endpoint.
        uint256 messageValue = _payNative(_fee.nativeFee);
        if (_fee.lzTokenFee > 0) _payLzToken(_fee.lzTokenFee);

        return
            // solhint-disable-next-line check-send-result
            endpoint.send{ value: messageValue }(
                MessagingParams(_dstEid, _getPeerOrRevert(_dstEid), _message, _options, _fee.lzTokenFee > 0),
                _refundAddress
            );
    }

    /**
     * @dev Internal function to pay the native fee associated with the message.
     * @param _nativeFee The native fee to be paid.
     * @return nativeFee The amount of native currency paid.
     *
     * @dev If the OApp needs to initiate MULTIPLE LayerZero messages in a single transaction,
     * this will need to be overridden because msg.value would contain multiple lzFees.
     * @dev Should be overridden in the event the LayerZero endpoint requires a different native currency.
     * @dev Some EVMs use an ERC20 as a method for paying transactions/gasFees.
     * @dev The endpoint is EITHER/OR, ie. it will NOT support both types of native payment at a time.
     */
    function _payNative(uint256 _nativeFee) internal virtual returns (uint256 nativeFee) {
        if (msg.value != _nativeFee) revert NotEnoughNative(msg.value);
        return _nativeFee;
    }

    /**
     * @dev Internal function to pay the LZ token fee associated with the message.
     * @param _lzTokenFee The LZ token fee to be paid.
     *
     * @dev If the caller is trying to pay in the specified lzToken, then the lzTokenFee is passed to the endpoint.
     * @dev Any excess sent, is passed back to the specified _refundAddress in the _lzSend().
     */
    function _payLzToken(uint256 _lzTokenFee) internal virtual {
        // @dev Cannot cache the token because it is not immutable in the endpoint.
        address lzToken = endpoint.lzToken();
        if (lzToken == address(0)) revert LzTokenUnavailable();

        // Pay LZ token fee by sending tokens to the endpoint.
        IERC20(lzToken).safeTransferFrom(msg.sender, address(endpoint), _lzTokenFee);
    }
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import "@layerzerolabs/lz-evm-protocol-v2/contracts/libs/CalldataBytesLib.sol";

library ExecutorOptions {
    using CalldataBytesLib for bytes;

    uint8 internal constant WORKER_ID = 1;

    uint8 internal constant OPTION_TYPE_LZRECEIVE = 1;
    uint8 internal constant OPTION_TYPE_NATIVE_DROP = 2;
    uint8 internal constant OPTION_TYPE_LZCOMPOSE = 3;
    uint8 internal constant OPTION_TYPE_ORDERED_EXECUTION = 4;
    uint8 internal constant OPTION_TYPE_LZREAD = 5;

    error Executor_InvalidLzReceiveOption();
    error Executor_InvalidNativeDropOption();
    error Executor_InvalidLzComposeOption();
    error Executor_InvalidLzReadOption();

    /// @dev decode the next executor option from the options starting from the specified cursor
    /// @param _options [executor_id][executor_option][executor_id][executor_option]...
    ///        executor_option = [option_size][option_type][option]
    ///        option_size = len(option_type) + len(option)
    ///        executor_id: uint8, option_size: uint16, option_type: uint8, option: bytes
    /// @param _cursor the cursor to start decoding from
    /// @return optionType the type of the option
    /// @return option the option of the executor
    /// @return cursor the cursor to start decoding the next executor option
    function nextExecutorOption(
        bytes calldata _options,
        uint256 _cursor
    ) internal pure returns (uint8 optionType, bytes calldata option, uint256 cursor) {
        unchecked {
            // skip worker id
            cursor = _cursor + 1;

            // read option size
            uint16 size = _options.toU16(cursor);
            cursor += 2;

            // read option type
            optionType = _options.toU8(cursor);

            // startCursor and endCursor are used to slice the option from _options
            uint256 startCursor = cursor + 1; // skip option type
            uint256 endCursor = cursor + size;
            option = _options[startCursor:endCursor];
            cursor += size;
        }
    }

    function decodeLzReceiveOption(bytes calldata _option) internal pure returns (uint128 gas, uint128 value) {
        if (_option.length != 16 && _option.length != 32) revert Executor_InvalidLzReceiveOption();
        gas = _option.toU128(0);
        value = _option.length == 32 ? _option.toU128(16) : 0;
    }

    function decodeNativeDropOption(bytes calldata _option) internal pure returns (uint128 amount, bytes32 receiver) {
        if (_option.length != 48) revert Executor_InvalidNativeDropOption();
        amount = _option.toU128(0);
        receiver = _option.toB32(16);
    }

    function decodeLzComposeOption(
        bytes calldata _option
    ) internal pure returns (uint16 index, uint128 gas, uint128 value) {
        if (_option.length != 18 && _option.length != 34) revert Executor_InvalidLzComposeOption();
        index = _option.toU16(0);
        gas = _option.toU128(2);
        value = _option.length == 34 ? _option.toU128(18) : 0;
    }

    function decodeLzReadOption(
        bytes calldata _option
    ) internal pure returns (uint128 gas, uint32 calldataSize, uint128 value) {
        if (_option.length != 20 && _option.length != 36) revert Executor_InvalidLzReadOption();
        gas = _option.toU128(0);
        calldataSize = _option.toU32(16);
        value = _option.length == 36 ? _option.toU128(20) : 0;
    }

    function encodeLzReceiveOption(uint128 _gas, uint128 _value) internal pure returns (bytes memory) {
        return _value == 0 ? abi.encodePacked(_gas) : abi.encodePacked(_gas, _value);
    }

    function encodeNativeDropOption(uint128 _amount, bytes32 _receiver) internal pure returns (bytes memory) {
        return abi.encodePacked(_amount, _receiver);
    }

    function encodeLzComposeOption(uint16 _index, uint128 _gas, uint128 _value) internal pure returns (bytes memory) {
        return _value == 0 ? abi.encodePacked(_index, _gas) : abi.encodePacked(_index, _gas, _value);
    }

    function encodeLzReadOption(
        uint128 _gas,
        uint32 _calldataSize,
        uint128 _value
    ) internal pure returns (bytes memory) {
        return _value == 0 ? abi.encodePacked(_gas, _calldataSize) : abi.encodePacked(_gas, _calldataSize, _value);
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { IOAppCore, ILayerZeroEndpointV2 } from "./interfaces/IOAppCore.sol";

/**
 * @title OAppCore
 * @dev Abstract contract implementing the IOAppCore interface with basic OApp configurations.
 */
abstract contract OAppCore is IOAppCore, Ownable {
    // The LayerZero endpoint associated with the given OApp
    ILayerZeroEndpointV2 public immutable endpoint;

    // Mapping to store peers associated with corresponding endpoints
    mapping(uint32 eid => bytes32 peer) public peers;

    /**
     * @dev Constructor to initialize the OAppCore with the provided endpoint and delegate.
     * @param _endpoint The address of the LOCAL Layer Zero endpoint.
     * @param _delegate The delegate capable of making OApp configurations inside of the endpoint.
     *
     * @dev The delegate typically should be set as the owner of the contract.
     */
    constructor(address _endpoint, address _delegate) {
        endpoint = ILayerZeroEndpointV2(_endpoint);

        if (_delegate == address(0)) revert InvalidDelegate();
        endpoint.setDelegate(_delegate);
    }

    /**
     * @notice Sets the peer address (OApp instance) for a corresponding endpoint.
     * @param _eid The endpoint ID.
     * @param _peer The address of the peer to be associated with the corresponding endpoint.
     *
     * @dev Only the owner/admin of the OApp can call this function.
     * @dev Indicates that the peer is trusted to send LayerZero messages to this OApp.
     * @dev Set this to bytes32(0) to remove the peer address.
     * @dev Peer is a bytes32 to accommodate non-evm chains.
     */
    function setPeer(uint32 _eid, bytes32 _peer) public virtual onlyOwner {
        _setPeer(_eid, _peer);
    }

    /**
     * @notice Sets the peer address (OApp instance) for a corresponding endpoint.
     * @param _eid The endpoint ID.
     * @param _peer The address of the peer to be associated with the corresponding endpoint.
     *
     * @dev Indicates that the peer is trusted to send LayerZero messages to this OApp.
     * @dev Set this to bytes32(0) to remove the peer address.
     * @dev Peer is a bytes32 to accommodate non-evm chains.
     */
    function _setPeer(uint32 _eid, bytes32 _peer) internal virtual {
        peers[_eid] = _peer;
        emit PeerSet(_eid, _peer);
    }

    /**
     * @notice Internal function to get the peer address associated with a specific endpoint; reverts if NOT set.
     * ie. the peer is set to bytes32(0).
     * @param _eid The endpoint ID.
     * @return peer The address of the peer associated with the specified endpoint.
     */
    function _getPeerOrRevert(uint32 _eid) internal view virtual returns (bytes32) {
        bytes32 peer = peers[_eid];
        if (peer == bytes32(0)) revert NoPeer(_eid);
        return peer;
    }

    /**
     * @notice Sets the delegate address for the OApp.
     * @param _delegate The address of the delegate to be set.
     *
     * @dev Only the owner/admin of the OApp can call this function.
     * @dev Provides the ability for a delegate to set configs, on behalf of the OApp, directly on the Endpoint contract.
     */
    function setDelegate(address _delegate) public onlyOwner {
        endpoint.setDelegate(_delegate);
    }
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { ReentrancyGuardUpgradeable } from "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import { Proxied } from "hardhat-deploy/solc_0.8/proxy/Proxied.sol";

import { Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import { PacketV1Codec } from "@layerzerolabs/lz-evm-protocol-v2/contracts/messagelib/libs/PacketV1Codec.sol";

import { IUltraLightNode301 } from "./uln/uln301/interfaces/IUltraLightNode301.sol";
import { IExecutor } from "./interfaces/IExecutor.sol";
import { IExecutorFeeLib } from "./interfaces/IExecutorFeeLib.sol";
import { WorkerUpgradeable } from "./upgradeable/WorkerUpgradeable.sol";

interface ILayerZeroEndpointV2 {
    function eid() external view returns (uint32);

    function lzReceive(
        Origin calldata _origin,
        address _receiver,
        bytes32 _guid,
        bytes calldata _message,
        bytes calldata _extraData
    ) external payable;

    function lzReceiveAlert(
        Origin calldata _origin,
        address _receiver,
        bytes32 _guid,
        uint256 _gas,
        uint256 _value,
        bytes calldata _message,
        bytes calldata _extraData,
        bytes calldata _reason
    ) external;

    function lzCompose(
        address _from,
        address _to,
        bytes32 _guid,
        uint16 _index,
        bytes calldata _message,
        bytes calldata _extraData
    ) external payable;

    function lzComposeAlert(
        address _from,
        address _to,
        bytes32 _guid,
        uint16 _index,
        uint256 _gas,
        uint256 _value,
        bytes calldata _message,
        bytes calldata _extraData,
        bytes calldata _reason
    ) external;
}

contract Executor is WorkerUpgradeable, ReentrancyGuardUpgradeable, Proxied, IExecutor {
    using PacketV1Codec for bytes;

    mapping(uint32 dstEid => DstConfig) public dstConfig;

    // endpoint v2
    address public endpoint;
    uint32 public localEidV2;

    // endpoint v1
    address public receiveUln301;

    function initialize(
        address _endpoint,
        address _receiveUln301,
        address[] memory _messageLibs,
        address _priceFeed,
        address _roleAdmin,
        address[] memory _admins
    ) external proxied initializer {
        __ReentrancyGuard_init();
        __Worker_init(_messageLibs, _priceFeed, 12000, _roleAdmin, _admins);
        endpoint = _endpoint;
        localEidV2 = ILayerZeroEndpointV2(_endpoint).eid();
        receiveUln301 = _receiveUln301;
    }

    function onUpgrade(address _receiveUln301) external proxied {
        receiveUln301 = _receiveUln301;
    }

    // --- Admin ---
    function setDstConfig(DstConfigParam[] memory _params) external onlyRole(ADMIN_ROLE) {
        for (uint256 i = 0; i < _params.length; i++) {
            DstConfigParam memory param = _params[i];
            dstConfig[param.dstEid] = DstConfig(
                param.lzReceiveBaseGas,
                param.multiplierBps,
                param.floorMarginUSD,
                param.nativeCap,
                param.lzComposeBaseGas
            );
        }
        emit DstConfigSet(_params);
    }

    function nativeDrop(
        Origin calldata _origin,
        uint32 _dstEid,
        address _oapp,
        NativeDropParams[] calldata _nativeDropParams,
        uint256 _nativeDropGasLimit
    ) external payable onlyRole(ADMIN_ROLE) nonReentrant {
        _nativeDrop(_origin, _dstEid, _oapp, _nativeDropParams, _nativeDropGasLimit);
    }

    function nativeDropAndExecute301(
        Origin calldata _origin,
        NativeDropParams[] calldata _nativeDropParams,
        uint256 _nativeDropGasLimit,
        bytes calldata _packet,
        uint256 _gasLimit
    ) external payable onlyRole(ADMIN_ROLE) nonReentrant {
        _nativeDrop(_origin, _packet.dstEid(), _packet.receiverB20(), _nativeDropParams, _nativeDropGasLimit);
        IUltraLightNode301(receiveUln301).commitVerification(_packet, _gasLimit);
    }

    function execute301(bytes calldata _packet, uint256 _gasLimit) external onlyRole(ADMIN_ROLE) nonReentrant {
        IUltraLightNode301(receiveUln301).commitVerification(_packet, _gasLimit);
    }

    function execute302(ExecutionParams calldata _executionParams) external payable onlyRole(ADMIN_ROLE) nonReentrant {
        try
            ILayerZeroEndpointV2(endpoint).lzReceive{ value: msg.value, gas: _executionParams.gasLimit }(
                _executionParams.origin,
                _executionParams.receiver,
                _executionParams.guid,
                _executionParams.message,
                _executionParams.extraData
            )
        {
            // do nothing
        } catch (bytes memory reason) {
            ILayerZeroEndpointV2(endpoint).lzReceiveAlert(
                _executionParams.origin,
                _executionParams.receiver,
                _executionParams.guid,
                _executionParams.gasLimit,
                msg.value,
                _executionParams.message,
                _executionParams.extraData,
                reason
            );
        }
    }

    function compose302(
        address _from,
        address _to,
        bytes32 _guid,
        uint16 _index,
        bytes calldata _message,
        bytes calldata _extraData,
        uint256 _gasLimit
    ) external payable onlyRole(ADMIN_ROLE) nonReentrant {
        try
            ILayerZeroEndpointV2(endpoint).lzCompose{ value: msg.value, gas: _gasLimit }(
                _from,
                _to,
                _guid,
                _index,
                _message,
                _extraData
            )
        {
            // do nothing
        } catch (bytes memory reason) {
            ILayerZeroEndpointV2(endpoint).lzComposeAlert(
                _from,
                _to,
                _guid,
                _index,
                _gasLimit,
                msg.value,
                _message,
                _extraData,
                reason
            );
        }
    }

    function nativeDropAndExecute302(
        NativeDropParams[] calldata _nativeDropParams,
        uint256 _nativeDropGasLimit,
        ExecutionParams calldata _executionParams
    ) external payable onlyRole(ADMIN_ROLE) nonReentrant {
        uint256 spent = _nativeDrop(
            _executionParams.origin,
            localEidV2,
            _executionParams.receiver,
            _nativeDropParams,
            _nativeDropGasLimit
        );

        uint256 value = msg.value - spent;
        try
            ILayerZeroEndpointV2(endpoint).lzReceive{ value: value, gas: _executionParams.gasLimit }(
                _executionParams.origin,
                _executionParams.receiver,
                _executionParams.guid,
                _executionParams.message,
                _executionParams.extraData
            )
        {
            // do nothing
        } catch (bytes memory reason) {
            ILayerZeroEndpointV2(endpoint).lzReceiveAlert(
                _executionParams.origin,
                _executionParams.receiver,
                _executionParams.guid,
                _executionParams.gasLimit,
                value,
                _executionParams.message,
                _executionParams.extraData,
                reason
            );
        }
    }

    // --- Message Lib ---
    function assignJob(
        uint32 _dstEid,
        address _sender,
        uint256 _calldataSize,
        bytes calldata _options
    ) external onlyRole(MESSAGE_LIB_ROLE) onlyAcl(_sender) whenNotPaused returns (uint256 fee) {
        IExecutorFeeLib.FeeParams memory params = IExecutorFeeLib.FeeParams(
            priceFeed,
            _dstEid,
            _sender,
            _calldataSize,
            defaultMultiplierBps
        );
        fee = IExecutorFeeLib(workerFeeLib).getFeeOnSend(params, dstConfig[_dstEid], _options);
    }

    // assignJob for CmdLib
    function assignJob(
        address _sender,
        bytes calldata _options
    ) external onlyRole(MESSAGE_LIB_ROLE) onlyAcl(_sender) whenNotPaused returns (uint256 fee) {
        IExecutorFeeLib.FeeParamsForRead memory params = IExecutorFeeLib.FeeParamsForRead(
            priceFeed,
            _sender,
            defaultMultiplierBps
        );
        fee = IExecutorFeeLib(workerFeeLib).getFeeOnSend(params, dstConfig[localEidV2], _options);
    }

    // --- Only ACL ---
    function getFee(
        uint32 _dstEid,
        address _sender,
        uint256 _calldataSize,
        bytes calldata _options
    ) external view onlyAcl(_sender) whenNotPaused returns (uint256 fee) {
        IExecutorFeeLib.FeeParams memory params = IExecutorFeeLib.FeeParams(
            priceFeed,
            _dstEid,
            _sender,
            _calldataSize,
            defaultMultiplierBps
        );
        fee = IExecutorFeeLib(workerFeeLib).getFee(params, dstConfig[_dstEid], _options);
    }

    function getFee(
        address _sender,
        bytes calldata _options
    ) external view onlyAcl(_sender) whenNotPaused returns (uint256 fee) {
        IExecutorFeeLib.FeeParamsForRead memory params = IExecutorFeeLib.FeeParamsForRead(
            priceFeed,
            _sender,
            defaultMultiplierBps
        );
        fee = IExecutorFeeLib(workerFeeLib).getFee(params, dstConfig[localEidV2], _options);
    }

    function _nativeDrop(
        Origin calldata _origin,
        uint32 _dstEid,
        address _oapp,
        NativeDropParams[] calldata _nativeDropParams,
        uint256 _nativeDropGasLimit
    ) internal returns (uint256 spent) {
        bool[] memory success = new bool[](_nativeDropParams.length);
        for (uint256 i = 0; i < _nativeDropParams.length; i++) {
            NativeDropParams memory param = _nativeDropParams[i];

            (bool sent, ) = param.receiver.call{ value: param.amount, gas: _nativeDropGasLimit }("");

            success[i] = sent;
            spent += param.amount;
        }
        emit NativeDropApplied(_origin, _dstEid, _oapp, _nativeDropParams, success);
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import { BytesLib } from "solidity-bytes-utils/contracts/BytesLib.sol";
import { SafeCast } from "@openzeppelin/contracts/utils/math/SafeCast.sol";

import { ExecutorOptions } from "@layerzerolabs/lz-evm-protocol-v2/contracts/messagelib/libs/ExecutorOptions.sol";
import { DVNOptions } from "@layerzerolabs/lz-evm-messagelib-v2/contracts/uln/libs/DVNOptions.sol";

/**
 * @title OptionsBuilder
 * @dev Library for building and encoding various message options.
 */
library OptionsBuilder {
    using SafeCast for uint256;
    using BytesLib for bytes;

    // Constants for options types
    uint16 internal constant TYPE_1 = 1; // legacy options type 1
    uint16 internal constant TYPE_2 = 2; // legacy options type 2
    uint16 internal constant TYPE_3 = 3;

    // Custom error message
    error InvalidSize(uint256 max, uint256 actual);
    error InvalidOptionType(uint16 optionType);

    // Modifier to ensure only options of type 3 are used
    modifier onlyType3(bytes memory _options) {
        if (_options.toUint16(0) != TYPE_3) revert InvalidOptionType(_options.toUint16(0));
        _;
    }

    /**
     * @dev Creates a new options container with type 3.
     * @return options The newly created options container.
     */
    function newOptions() internal pure returns (bytes memory) {
        return abi.encodePacked(TYPE_3);
    }

    /**
     * @dev Adds an executor LZ receive option to the existing options.
     * @param _options The existing options container.
     * @param _gas The gasLimit used on the lzReceive() function in the OApp.
     * @param _value The msg.value passed to the lzReceive() function in the OApp.
     * @return options The updated options container.
     *
     * @dev When multiples of this option are added, they are summed by the executor
     * eg. if (_gas: 200k, and _value: 1 ether) AND (_gas: 100k, _value: 0.5 ether) are sent in an option to the LayerZeroEndpoint,
     * that becomes (300k, 1.5 ether) when the message is executed on the remote lzReceive() function.
     */
    function addExecutorLzReceiveOption(
        bytes memory _options,
        uint128 _gas,
        uint128 _value
    ) internal pure onlyType3(_options) returns (bytes memory) {
        bytes memory option = ExecutorOptions.encodeLzReceiveOption(_gas, _value);
        return addExecutorOption(_options, ExecutorOptions.OPTION_TYPE_LZRECEIVE, option);
    }

    /**
     * @dev Adds an executor native drop option to the existing options.
     * @param _options The existing options container.
     * @param _amount The amount for the native value that is airdropped to the 'receiver'.
     * @param _receiver The receiver address for the native drop option.
     * @return options The updated options container.
     *
     * @dev When multiples of this option are added, they are summed by the executor on the remote chain.
     */
    function addExecutorNativeDropOption(
        bytes memory _options,
        uint128 _amount,
        bytes32 _receiver
    ) internal pure onlyType3(_options) returns (bytes memory) {
        bytes memory option = ExecutorOptions.encodeNativeDropOption(_amount, _receiver);
        return addExecutorOption(_options, ExecutorOptions.OPTION_TYPE_NATIVE_DROP, option);
    }

    /**
     * @dev Adds an executor LZ compose option to the existing options.
     * @param _options The existing options container.
     * @param _index The index for the lzCompose() function call.
     * @param _gas The gasLimit for the lzCompose() function call.
     * @param _value The msg.value for the lzCompose() function call.
     * @return options The updated options container.
     *
     * @dev When multiples of this option are added, they are summed PER index by the executor on the remote chain.
     * @dev If the OApp sends N lzCompose calls on the remote, you must provide N incremented indexes starting with 0.
     * ie. When your remote OApp composes (N = 3) messages, you must set this option for index 0,1,2
     */
    function addExecutorLzComposeOption(
        bytes memory _options,
        uint16 _index,
        uint128 _gas,
        uint128 _value
    ) internal pure onlyType3(_options) returns (bytes memory) {
        bytes memory option = ExecutorOptions.encodeLzComposeOption(_index, _gas, _value);
        return addExecutorOption(_options, ExecutorOptions.OPTION_TYPE_LZCOMPOSE, option);
    }

    /**
     * @dev Adds an executor ordered execution option to the existing options.
     * @param _options The existing options container.
     * @return options The updated options container.
     */
    function addExecutorOrderedExecutionOption(
        bytes memory _options
    ) internal pure onlyType3(_options) returns (bytes memory) {
        return addExecutorOption(_options, ExecutorOptions.OPTION_TYPE_ORDERED_EXECUTION, bytes(""));
    }

    /**
     * @dev Adds a DVN pre-crime option to the existing options.
     * @param _options The existing options container.
     * @param _dvnIdx The DVN index for the pre-crime option.
     * @return options The updated options container.
     */
    function addDVNPreCrimeOption(
        bytes memory _options,
        uint8 _dvnIdx
    ) internal pure onlyType3(_options) returns (bytes memory) {
        return addDVNOption(_options, _dvnIdx, DVNOptions.OPTION_TYPE_PRECRIME, bytes(""));
    }

    /**
     * @dev Adds an executor option to the existing options.
     * @param _options The existing options container.
     * @param _optionType The type of the executor option.
     * @param _option The encoded data for the executor option.
     * @return options The updated options container.
     */
    function addExecutorOption(
        bytes memory _options,
        uint8 _optionType,
        bytes memory _option
    ) internal pure onlyType3(_options) returns (bytes memory) {
        return
            abi.encodePacked(
                _options,
                ExecutorOptions.WORKER_ID,
                _option.length.toUint16() + 1, // +1 for optionType
                _optionType,
                _option
            );
    }

    /**
     * @dev Adds a DVN option to the existing options.
     * @param _options The existing options container.
     * @param _dvnIdx The DVN index for the DVN option.
     * @param _optionType The type of the DVN option.
     * @param _option The encoded data for the DVN option.
     * @return options The updated options container.
     */
    function addDVNOption(
        bytes memory _options,
        uint8 _dvnIdx,
        uint8 _optionType,
        bytes memory _option
    ) internal pure onlyType3(_options) returns (bytes memory) {
        return
            abi.encodePacked(
                _options,
                DVNOptions.WORKER_ID,
                _option.length.toUint16() + 2, // +2 for optionType and dvnIdx
                _dvnIdx,
                _optionType,
                _option
            );
    }

    /**
     * @dev Encodes legacy options of type 1.
     * @param _executionGas The gasLimit value passed to lzReceive().
     * @return legacyOptions The encoded legacy options.
     */
    function encodeLegacyOptionsType1(uint256 _executionGas) internal pure returns (bytes memory) {
        if (_executionGas > type(uint128).max) revert InvalidSize(type(uint128).max, _executionGas);
        return abi.encodePacked(TYPE_1, _executionGas);
    }

    /**
     * @dev Encodes legacy options of type 2.
     * @param _executionGas The gasLimit value passed to lzReceive().
     * @param _nativeForDst The amount of native air dropped to the receiver.
     * @param _receiver The _nativeForDst receiver address.
     * @return legacyOptions The encoded legacy options of type 2.
     */
    function encodeLegacyOptionsType2(
        uint256 _executionGas,
        uint256 _nativeForDst,
        bytes memory _receiver // @dev Use bytes instead of bytes32 in legacy type 2 for _receiver.
    ) internal pure returns (bytes memory) {
        if (_executionGas > type(uint128).max) revert InvalidSize(type(uint128).max, _executionGas);
        if (_nativeForDst > type(uint128).max) revert InvalidSize(type(uint128).max, _nativeForDst);
        if (_receiver.length > 32) revert InvalidSize(32, _receiver.length);
        return abi.encodePacked(TYPE_2, _executionGas, _nativeForDst, _receiver);
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { IOAppOptionsType3, EnforcedOptionParam } from "../interfaces/IOAppOptionsType3.sol";

/**
 * @title OAppOptionsType3
 * @dev Abstract contract implementing the IOAppOptionsType3 interface with type 3 options.
 */
abstract contract OAppOptionsType3 is IOAppOptionsType3, Ownable {
    uint16 internal constant OPTION_TYPE_3 = 3;

    // @dev The "msgType" should be defined in the child contract.
    mapping(uint32 eid => mapping(uint16 msgType => bytes enforcedOption)) public enforcedOptions;

    /**
     * @dev Sets the enforced options for specific endpoint and message type combinations.
     * @param _enforcedOptions An array of EnforcedOptionParam structures specifying enforced options.
     *
     * @dev Only the owner/admin of the OApp can call this function.
     * @dev Provides a way for the OApp to enforce things like paying for PreCrime, AND/OR minimum dst lzReceive gas amounts etc.
     * @dev These enforced options can vary as the potential options/execution on the remote may differ as per the msgType.
     * eg. Amount of lzReceive() gas necessary to deliver a lzCompose() message adds overhead you dont want to pay
     * if you are only making a standard LayerZero message ie. lzReceive() WITHOUT sendCompose().
     */
    function setEnforcedOptions(EnforcedOptionParam[] calldata _enforcedOptions) public virtual onlyOwner {
        _setEnforcedOptions(_enforcedOptions);
    }

    /**
     * @dev Sets the enforced options for specific endpoint and message type combinations.
     * @param _enforcedOptions An array of EnforcedOptionParam structures specifying enforced options.
     *
     * @dev Provides a way for the OApp to enforce things like paying for PreCrime, AND/OR minimum dst lzReceive gas amounts etc.
     * @dev These enforced options can vary as the potential options/execution on the remote may differ as per the msgType.
     * eg. Amount of lzReceive() gas necessary to deliver a lzCompose() message adds overhead you dont want to pay
     * if you are only making a standard LayerZero message ie. lzReceive() WITHOUT sendCompose().
     */
    function _setEnforcedOptions(EnforcedOptionParam[] memory _enforcedOptions) internal virtual {
        for (uint256 i = 0; i < _enforcedOptions.length; i++) {
            // @dev Enforced options are only available for optionType 3, as type 1 and 2 dont support combining.
            _assertOptionsType3(_enforcedOptions[i].options);
            enforcedOptions[_enforcedOptions[i].eid][_enforcedOptions[i].msgType] = _enforcedOptions[i].options;
        }

        emit EnforcedOptionSet(_enforcedOptions);
    }

    /**
     * @notice Combines options for a given endpoint and message type.
     * @param _eid The endpoint ID.
     * @param _msgType The OAPP message type.
     * @param _extraOptions Additional options passed by the caller.
     * @return options The combination of caller specified options AND enforced options.
     *
     * @dev If there is an enforced lzReceive option:
     * - {gasLimit: 200k, msg.value: 1 ether} AND a caller supplies a lzReceive option: {gasLimit: 100k, msg.value: 0.5 ether}
     * - The resulting options will be {gasLimit: 300k, msg.value: 1.5 ether} when the message is executed on the remote lzReceive() function.
     * @dev This presence of duplicated options is handled off-chain in the verifier/executor.
     */
    function combineOptions(
        uint32 _eid,
        uint16 _msgType,
        bytes calldata _extraOptions
    ) public view virtual returns (bytes memory) {
        bytes memory enforced = enforcedOptions[_eid][_msgType];

        // No enforced options, pass whatever the caller supplied, even if it's empty or legacy type 1/2 options.
        if (enforced.length == 0) return _extraOptions;

        // No caller options, return enforced
        if (_extraOptions.length == 0) return enforced;

        // @dev If caller provided _extraOptions, must be type 3 as its the ONLY type that can be combined.
        if (_extraOptions.length >= 2) {
            _assertOptionsType3(_extraOptions);
            // @dev Remove the first 2 bytes containing the type from the _extraOptions and combine with enforced.
            return bytes.concat(enforced, _extraOptions[2:]);
        }

        // No valid set of options was found.
        revert InvalidOptions(_extraOptions);
    }

    /**
     * @dev Internal function to assert that options are of type 3.
     * @param _options The options to be checked.
     */
    function _assertOptionsType3(bytes memory _options) internal pure virtual {
        uint16 optionsType;
        assembly {
            optionsType := mload(add(_options, 2))
        }
        if (optionsType != OPTION_TYPE_3) revert InvalidOptions(_options);
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v4.9.0) (security/ReentrancyGuard.sol)

pragma solidity ^0.8.0;

/**
 * @dev Contract module that helps prevent reentrant calls to a function.
 *
 * Inheriting from `ReentrancyGuard` will make the {nonReentrant} modifier
 * available, which can be applied to functions to make sure there are no nested
 * (reentrant) calls to them.
 *
 * Note that because there is a single `nonReentrant` guard, functions marked as
 * `nonReentrant` may not call one another. This can be worked around by making
 * those functions `private`, and then adding `external` `nonReentrant` entry
 * points to them.
 *
 * TIP: If you would like to learn more about reentrancy and alternative ways
 * to protect against it, check out our blog post
 * https://blog.openzeppelin.com/reentrancy-after-istanbul/[Reentrancy After Istanbul].
 */
abstract contract ReentrancyGuard {
    // Booleans are more expensive than uint256 or any type that takes up a full
    // word because each write operation emits an extra SLOAD to first read the
    // slot's contents, replace the bits taken up by the boolean, and then write
    // back. This is the compiler's defense against contract upgrades and
    // pointer aliasing, and it cannot be disabled.

    // The values being non-zero value makes deployment a bit more expensive,
    // but in exchange the refund on every call to nonReentrant will be lower in
    // amount. Since refunds are capped to a percentage of the total
    // transaction's gas, it is best to keep them low in cases like this one, to
    // increase the likelihood of the full refund coming into effect.
    uint256 private constant _NOT_ENTERED = 1;
    uint256 private constant _ENTERED = 2;

    uint256 private _status;

    constructor() {
        _status = _NOT_ENTERED;
    }

    /**
     * @dev Prevents a contract from calling itself, directly or indirectly.
     * Calling a `nonReentrant` function from another `nonReentrant`
     * function is not supported. It is possible to prevent this from happening
     * by making the `nonReentrant` function external, and making it call a
     * `private` function that does the actual work.
     */
    modifier nonReentrant() {
        _nonReentrantBefore();
        _;
        _nonReentrantAfter();
    }

    function _nonReentrantBefore() private {
        // On the first call to nonReentrant, _status will be _NOT_ENTERED
        require(_status != _ENTERED, "ReentrancyGuard: reentrant call");

        // Any calls to nonReentrant after this point will fail
        _status = _ENTERED;
    }

    function _nonReentrantAfter() private {
        // By storing the original value once again, a refund is triggered (see
        // https://eips.ethereum.org/EIPS/eip-2200)
        _status = _NOT_ENTERED;
    }

    /**
     * @dev Returns true if the reentrancy guard is currently set to "entered", which indicates there is a
     * `nonReentrant` function in the call stack.
     */
    function _reentrancyGuardEntered() internal view returns (bool) {
        return _status == _ENTERED;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title IUnstakeMessenger
 * @notice Interface for the UnstakeMessenger contract on spoke chains
 * @dev Enables users to request crosschain unstaking from spoke to hub
 */
interface IUnstakeMessenger {
    // ============ Structs ============

    /**
     * @notice Unstake message payload sent to hub chain
     * @param user Address that should receive the unstaked iTRY on spoke chain
     * @param extraOptions LayerZero gas options for the return trip (Hub → Spoke)
     */
    struct UnstakeMessage {
        address user;
        bytes extraOptions;
    }

    // ============ Events ============

    /**
     * @notice Emitted when a crosschain unstake request is sent
     * @param user Address requesting the unstake
     * @param hubEid Destination endpoint ID (hub chain)
     * @param totalFee LayerZero fee paid (actual fee used)
     * @param excessRefunded Amount refunded to user (buffer excess)
     * @param guid LayerZero message GUID
     */
    event UnstakeRequested(
        address indexed user, uint32 indexed hubEid, uint256 totalFee, uint256 excessRefunded, bytes32 guid
    );

    /**
     * @notice Emitted when fee buffer is updated
     * @param oldBufferBPS Previous buffer value
     * @param newBufferBPS New buffer value
     */
    event FeeBufferUpdated(uint256 oldBufferBPS, uint256 newBufferBPS);

    /**
     * @notice Emitted when tokens are rescued from the contract
     * @param token The address of the token rescued (address(0) for ETH)
     * @param to The address receiving the rescued tokens
     * @param amount The amount rescued
     */
    event TokenRescued(address indexed token, address indexed to, uint256 amount);

    // ============ Errors ============

    /**
     * @notice Thrown when insufficient fee is provided for LayerZero message
     * @param required Required fee amount
     * @param provided Provided fee amount
     */
    error InsufficientFee(uint256 required, uint256 provided);

    /**
     * @notice Thrown when hub endpoint ID is not configured
     */
    error HubNotConfigured();

    /**
     * @notice Thrown when returnTripAllocation is zero
     */
    error InvalidReturnTripAllocation();

    /**
     * @notice Thrown when a zero amount is provided where it's not allowed
     */
    error ZeroAmount();

    /**
     * @notice Thrown when a transfer fails
     */
    error TransferFailed();

    /**
     * @notice Thrown when an invalid zero address is provided
     */
    error ZeroAddress();

    // ============ External Functions ============

    /**
     * @notice Request crosschain unstake from spoke to hub
     * @dev Sends LayerZero message to wiTryVaultComposer on hub chain
     * @dev User must have completed cooldown on hub chain or transaction will revert
     * @dev msg.value must cover LayerZero fees for both legs (Spoke→Hub and Hub→Spoke)
     * @param returnTripAllocation Exact native value to forward to hub for return trip (in wei)
     *        This should be the result of calling wiTryVaultComposer.quoteUnstakeReturn()
     * @return guid LayerZero message GUID for tracking
     */
    function unstake(uint256 returnTripAllocation) external payable returns (bytes32 guid);

    /**
     * @notice Quote exact fee for unstake WITH specified return trip allocation
     * @param returnTripValue Exact amount in wei to forward to hub for return trip
     * @return nativeFee Total spoke→hub message fee WITH returnTripValue embedded
     * @return lzTokenFee Fee in LayerZero token (typically 0)
     */
    function quoteUnstakeWithReturnValue(uint256 returnTripValue)
        external
        view
        returns (uint256 nativeFee, uint256 lzTokenFee);

    /**
     * @notice Quote recommended fee for unstake WITH buffer applied
     * @dev Automatically applies feeBufferBPS safety buffer to protect against gas fluctuations
     * @param returnTripValue Exact amount in wei to forward to hub for return trip
     * @return recommendedFee Recommended total fee with buffer applied
     * @return lzTokenFee Fee in LayerZero token (typically 0)
     */
    function quoteUnstakeWithBuffer(uint256 returnTripValue)
        external
        view
        returns (uint256 recommendedFee, uint256 lzTokenFee);

    /**
     * @notice Set the peer contract address on the hub chain
     * @param hubEid Hub chain endpoint ID
     * @param peer wiTryVaultComposer address on hub (bytes32 encoded)
     */
    function setPeer(uint32 hubEid, bytes32 peer) external;

    /**
     * @notice Get the configured peer for the hub chain
     * @return peer Peer contract address (bytes32 encoded)
     */
    function getHubPeer() external view returns (bytes32 peer);

    // ============ View Functions ============

    /**
     * @notice Get the hub chain endpoint ID
     * @return Hub chain EID
     */
    function hubEid() external view returns (uint32);

    /**
     * @notice Get the current fee buffer in basis points
     * @return Fee buffer in BPS (e.g., 1000 = 10%)
     */
    function feeBufferBPS() external view returns (uint256);

    // ============ Admin Functions ============

    /**
     * @notice Update fee buffer percentage
     * @param newBufferBPS New buffer in basis points (e.g., 1000 = 10%)
     */
    function setFeeBufferBPS(uint256 newBufferBPS) external;

    /**
     * @notice Rescue tokens accidentally sent to this contract
     * @dev Only callable by owner. Can rescue both ERC20 tokens and native ETH
     *      Use address(0) for rescuing ETH
     * @param token The token address to rescue (use address(0) for ETH)
     * @param to The address to send rescued tokens to
     * @param amount The amount to rescue
     */
    function rescueToken(address token, address to, uint256 amount) external;
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import { ILayerZeroEndpointV2 } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";

/**
 * @title IOAppCore
 */
interface IOAppCore {
    // Custom error messages
    error OnlyPeer(uint32 eid, bytes32 sender);
    error NoPeer(uint32 eid);
    error InvalidEndpointCall();
    error InvalidDelegate();

    // Event emitted when a peer (OApp) is set for a corresponding endpoint
    event PeerSet(uint32 eid, bytes32 peer);

    /**
     * @notice Retrieves the OApp version information.
     * @return senderVersion The version of the OAppSender.sol contract.
     * @return receiverVersion The version of the OAppReceiver.sol contract.
     */
    function oAppVersion() external view returns (uint64 senderVersion, uint64 receiverVersion);

    /**
     * @notice Retrieves the LayerZero endpoint associated with the OApp.
     * @return iEndpoint The LayerZero endpoint as an interface.
     */
    function endpoint() external view returns (ILayerZeroEndpointV2 iEndpoint);

    /**
     * @notice Retrieves the peer (OApp) associated with a corresponding endpoint.
     * @param _eid The endpoint ID.
     * @return peer The peer address (OApp instance) associated with the corresponding endpoint.
     */
    function peers(uint32 _eid) external view returns (bytes32 peer);

    /**
     * @notice Sets the peer address (OApp instance) for a corresponding endpoint.
     * @param _eid The endpoint ID.
     * @param _peer The address of the peer to be associated with the corresponding endpoint.
     */
    function setPeer(uint32 _eid, bytes32 _peer) external;

    /**
     * @notice Sets the delegate address for the OApp Core.
     * @param _delegate The address of the delegate to be set.
     */
    function setDelegate(address _delegate) external;
}

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

interface IUltraLightNode301 {
    function commitVerification(bytes calldata _packet, uint256 _gasLimit) external;
}

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

import { Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";

import { IWorker } from "./IWorker.sol";
import { ILayerZeroExecutor } from "./ILayerZeroExecutor.sol";
import { ILayerZeroReadExecutor } from "./ILayerZeroReadExecutor.sol";

interface IExecutor is IWorker, ILayerZeroExecutor, ILayerZeroReadExecutor {
    struct DstConfigParam {
        uint32 dstEid;
        uint64 lzReceiveBaseGas;
        uint64 lzComposeBaseGas;
        uint16 multiplierBps;
        uint128 floorMarginUSD;
        uint128 nativeCap;
    }

    struct DstConfig {
        uint64 lzReceiveBaseGas;
        uint16 multiplierBps;
        uint128 floorMarginUSD; // uses priceFeed PRICE_RATIO_DENOMINATOR
        uint128 nativeCap;
        uint64 lzComposeBaseGas;
    }

    struct ExecutionParams {
        address receiver;
        Origin origin;
        bytes32 guid;
        bytes message;
        bytes extraData;
        uint256 gasLimit;
    }

    struct NativeDropParams {
        address receiver;
        uint256 amount;
    }

    event DstConfigSet(DstConfigParam[] params);
    event NativeDropApplied(Origin origin, uint32 dstEid, address oapp, NativeDropParams[] params, bool[] success);

    function dstConfig(uint32 _dstEid) external view returns (uint64, uint16, uint128, uint128, uint64);
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { PausableUpgradeable } from "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";
import { AccessControlUpgradeable } from "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import { ISendLib } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ISendLib.sol";
import { Transfer } from "@layerzerolabs/lz-evm-protocol-v2/contracts/libs/Transfer.sol";

import { IWorker } from "../interfaces/IWorker.sol";

abstract contract WorkerUpgradeable is Initializable, AccessControlUpgradeable, PausableUpgradeable, IWorker {
    bytes32 internal constant MESSAGE_LIB_ROLE = keccak256("MESSAGE_LIB_ROLE");
    bytes32 internal constant ALLOWLIST = keccak256("ALLOWLIST");
    bytes32 internal constant DENYLIST = keccak256("DENYLIST");
    bytes32 internal constant ADMIN_ROLE = keccak256("ADMIN_ROLE");

    address public workerFeeLib;

    uint64 public allowlistSize;
    uint16 public defaultMultiplierBps;
    address public priceFeed;

    mapping(uint32 eid => uint8[] optionTypes) internal supportedOptionTypes;

    /// @param _messageLibs array of message lib addresses that are granted the MESSAGE_LIB_ROLE
    /// @param _priceFeed price feed address
    /// @param _defaultMultiplierBps default multiplier for worker fee
    /// @param _roleAdmin address that is granted the DEFAULT_ADMIN_ROLE (can grant and revoke all roles)
    /// @param _admins array of admin addresses that are granted the ADMIN_ROLE
    function __Worker_init(
        address[] memory _messageLibs,
        address _priceFeed,
        uint16 _defaultMultiplierBps,
        address _roleAdmin,
        address[] memory _admins
    ) internal onlyInitializing {
        __Context_init_unchained();
        __AccessControl_init_unchained();
        __Pausable_init_unchained();
        __Worker_init_unchained(_messageLibs, _priceFeed, _defaultMultiplierBps, _roleAdmin, _admins);
    }

    function __Worker_init_unchained(
        address[] memory _messageLibs,
        address _priceFeed,
        uint16 _defaultMultiplierBps,
        address _roleAdmin,
        address[] memory _admins
    ) internal onlyInitializing {
        defaultMultiplierBps = _defaultMultiplierBps;
        priceFeed = _priceFeed;

        if (_roleAdmin != address(0x0)) {
            _grantRole(DEFAULT_ADMIN_ROLE, _roleAdmin); // _roleAdmin can grant and revoke all roles
        }

        for (uint256 i = 0; i < _messageLibs.length; ++i) {
            _grantRole(MESSAGE_LIB_ROLE, _messageLibs[i]);
        }

        for (uint256 i = 0; i < _admins.length; ++i) {
            _grantRole(ADMIN_ROLE, _admins[i]);
        }
    }

    // ========================= Modifier =========================

    modifier onlyAcl(address _sender) {
        if (!hasAcl(_sender)) {
            revert Worker_NotAllowed();
        }
        _;
    }

    /// @dev Access control list using allowlist and denylist
    /// @dev 1) if one address is in the denylist -> deny
    /// @dev 2) else if address in the allowlist OR allowlist is empty (allows everyone)-> allow
    /// @dev 3) else deny
    /// @param _sender address to check
    function hasAcl(address _sender) public view returns (bool) {
        if (hasRole(DENYLIST, _sender)) {
            return false;
        } else if (allowlistSize == 0 || hasRole(ALLOWLIST, _sender)) {
            return true;
        } else {
            return false;
        }
    }

    // ========================= OnyDefaultAdmin =========================

    /// @dev flag to pause execution of workers (if used with whenNotPaused modifier)
    /// @param _paused true to pause, false to unpause
    function setPaused(bool _paused) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (_paused) {
            _pause();
        } else {
            _unpause();
        }
    }

    // ========================= OnlyAdmin =========================

    /// @param _priceFeed price feed address
    function setPriceFeed(address _priceFeed) external onlyRole(ADMIN_ROLE) {
        priceFeed = _priceFeed;
        emit SetPriceFeed(_priceFeed);
    }

    /// @param _workerFeeLib worker fee lib address
    function setWorkerFeeLib(address _workerFeeLib) external onlyRole(ADMIN_ROLE) {
        workerFeeLib = _workerFeeLib;
        emit SetWorkerLib(_workerFeeLib);
    }

    /// @param _multiplierBps default multiplier for worker fee
    function setDefaultMultiplierBps(uint16 _multiplierBps) external onlyRole(ADMIN_ROLE) {
        defaultMultiplierBps = _multiplierBps;
        emit SetDefaultMultiplierBps(_multiplierBps);
    }

    /// @dev supports withdrawing fee from ULN301, ULN302 and more
    /// @param _lib message lib address
    /// @param _to address to withdraw fee to
    /// @param _amount amount to withdraw
    function withdrawFee(address _lib, address _to, uint256 _amount) external onlyRole(ADMIN_ROLE) {
        if (!hasRole(MESSAGE_LIB_ROLE, _lib)) revert Worker_OnlyMessageLib();
        ISendLib(_lib).withdrawFee(_to, _amount);
        emit Withdraw(_lib, _to, _amount);
    }

    /// @dev supports withdrawing token from the contract
    /// @param _token token address
    /// @param _to address to withdraw token to
    /// @param _amount amount to withdraw
    function withdrawToken(address _token, address _to, uint256 _amount) external onlyRole(ADMIN_ROLE) {
        // transfers native if _token is address(0x0)
        Transfer.nativeOrToken(_token, _to, _amount);
    }

    function setSupportedOptionTypes(uint32 _eid, uint8[] calldata _optionTypes) external onlyRole(ADMIN_ROLE) {
        supportedOptionTypes[_eid] = _optionTypes;
    }

    // ========================= View Functions =========================
    function getSupportedOptionTypes(uint32 _eid) external view returns (uint8[] memory) {
        return supportedOptionTypes[_eid];
    }

    // ========================= Internal Functions =========================

    /// @dev overrides AccessControl to allow for counting of allowlistSize
    /// @param _role role to grant
    /// @param _account address to grant role to
    function _grantRole(bytes32 _role, address _account) internal override {
        if (_role == ALLOWLIST && !hasRole(_role, _account)) {
            ++allowlistSize;
        }
        super._grantRole(_role, _account);
    }

    /// @dev overrides AccessControl to allow for counting of allowlistSize
    /// @param _role role to revoke
    /// @param _account address to revoke role from
    function _revokeRole(bytes32 _role, address _account) internal override {
        if (_role == ALLOWLIST && hasRole(_role, _account)) {
            --allowlistSize;
        }
        super._revokeRole(_role, _account);
    }

    /// @dev overrides AccessControl to disable renouncing of roles
    function renounceRole(bytes32 /*role*/, address /*account*/) public pure override {
        revert Worker_RoleRenouncingDisabled();
    }

    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[47] private __gap;
}

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

interface ILayerZeroReadExecutor {
    // @notice query price and assign jobs at the same time
    // @param _sender - the source sending contract address. executors may apply price discrimination to senders
    // @param _options - optional parameters for extra service plugins, e.g. sending dust tokens at the destination chain
    function assignJob(address _sender, bytes calldata _options) external returns (uint256 fee);

    // @notice query the executor price for executing the payload on this chain
    // @param _sender - the source sending contract address. executors may apply price discrimination to senders
    // @param _options - optional parameters for extra service plugins, e.g. sending dust tokens
    function getFee(address _sender, bytes calldata _options) external view returns (uint256 fee);
}

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

interface IWorker {
    event SetWorkerLib(address workerLib);
    event SetPriceFeed(address priceFeed);
    event SetDefaultMultiplierBps(uint16 multiplierBps);
    event SetSupportedOptionTypes(uint32 dstEid, uint8[] optionTypes);
    event Withdraw(address lib, address to, uint256 amount);

    error Worker_NotAllowed();
    error Worker_OnlyMessageLib();
    error Worker_RoleRenouncingDisabled();

    function setPriceFeed(address _priceFeed) external;

    function priceFeed() external view returns (address);

    function setDefaultMultiplierBps(uint16 _multiplierBps) external;

    function defaultMultiplierBps() external view returns (uint16);

    function withdrawFee(address _lib, address _to, uint256 _amount) external;

    function setSupportedOptionTypes(uint32 _eid, uint8[] calldata _optionTypes) external;

    function getSupportedOptionTypes(uint32 _eid) external view returns (uint8[] memory);
}

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

interface IWorker {
    event SetWorkerLib(address workerLib);
    event SetPriceFeed(address priceFeed);
    event SetDefaultMultiplierBps(uint16 multiplierBps);
    event SetSupportedOptionTypes(uint32 dstEid, uint8[] optionTypes);
    event Withdraw(address lib, address to, uint256 amount);

    error Worker_NotAllowed();
    error Worker_OnlyMessageLib();
    error Worker_RoleRenouncingDisabled();

    function setPriceFeed(address _priceFeed) external;

    function priceFeed() external view returns (address);

    function setDefaultMultiplierBps(uint16 _multiplierBps) external;

    function defaultMultiplierBps() external view returns (uint16);

    function withdrawFee(address _lib, address _to, uint256 _amount) external;

    function setSupportedOptionTypes(uint32 _eid, uint8[] calldata _optionTypes) external;

    function getSupportedOptionTypes(uint32 _eid) external view returns (uint8[] memory);
}

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

interface ILayerZeroExecutor {
    // @notice query price and assign jobs at the same time
    // @param _dstEid - the destination endpoint identifier
    // @param _sender - the source sending contract address. executors may apply price discrimination to senders
    // @param _calldataSize - dynamic data size of message + caller params
    // @param _options - optional parameters for extra service plugins, e.g. sending dust tokens at the destination chain
    function assignJob(
        uint32 _dstEid,
        address _sender,
        uint256 _calldataSize,
        bytes calldata _options
    ) external returns (uint256 price);

    // @notice query the executor price for relaying the payload and its proof to the destination chain
    // @param _dstEid - the destination endpoint identifier
    // @param _sender - the source sending contract address. executors may apply price discrimination to senders
    // @param _calldataSize - dynamic data size of message + caller params
    // @param _options - optional parameters for extra service plugins, e.g. sending dust tokens at the destination chain
    function getFee(
        uint32 _dstEid,
        address _sender,
        uint256 _calldataSize,
        bytes calldata _options
    ) external view returns (uint256 price);
}

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

import { IExecutor } from "./IExecutor.sol";

interface IExecutorFeeLib {
    struct FeeParams {
        address priceFeed;
        uint32 dstEid;
        address sender;
        uint256 calldataSize;
        uint16 defaultMultiplierBps;
    }

    struct FeeParamsForRead {
        address priceFeed;
        address sender;
        uint16 defaultMultiplierBps;
    }

    error Executor_NoOptions();
    error Executor_NativeAmountExceedsCap(uint256 amount, uint256 cap);
    error Executor_UnsupportedOptionType(uint8 optionType);
    error Executor_InvalidExecutorOptions(uint256 cursor);
    error Executor_ZeroLzReceiveGasProvided();
    error Executor_ZeroLzComposeGasProvided();
    error Executor_ZeroCalldataSizeProvided();
    error Executor_EidNotSupported(uint32 eid);

    function getFeeOnSend(
        FeeParams calldata _params,
        IExecutor.DstConfig calldata _dstConfig,
        bytes calldata _options
    ) external returns (uint256 fee);

    function getFee(
        FeeParams calldata _params,
        IExecutor.DstConfig calldata _dstConfig,
        bytes calldata _options
    ) external view returns (uint256 fee);

    function getFeeOnSend(
        FeeParamsForRead calldata _params,
        IExecutor.DstConfig calldata _dstConfig,
        bytes calldata _options
    ) external returns (uint256 fee);

    function getFee(
        FeeParamsForRead calldata _params,
        IExecutor.DstConfig calldata _dstConfig,
        bytes calldata _options
    ) external view returns (uint256 fee);

    function version() external view returns (uint64 major, uint8 minor);
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

import { MessagingFee } from "./ILayerZeroEndpointV2.sol";
import { IMessageLib } from "./IMessageLib.sol";

struct Packet {
    uint64 nonce;
    uint32 srcEid;
    address sender;
    uint32 dstEid;
    bytes32 receiver;
    bytes32 guid;
    bytes message;
}

interface ISendLib is IMessageLib {
    function send(
        Packet calldata _packet,
        bytes calldata _options,
        bool _payInLzToken
    ) external returns (MessagingFee memory, bytes memory encodedPacket);

    function quote(
        Packet calldata _packet,
        bytes calldata _options,
        bool _payInLzToken
    ) external view returns (MessagingFee memory);

    function setTreasury(address _treasury) external;

    function withdrawFee(address _to, uint256 _amount) external;

    function withdrawLzTokenFee(address _lzToken, address _to, uint256 _amount) external;
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { Pausable } from "@openzeppelin/contracts/security/Pausable.sol";
import { AccessControl } from "@openzeppelin/contracts/access/AccessControl.sol";

import { ISendLib } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ISendLib.sol";
import { Transfer } from "@layerzerolabs/lz-evm-protocol-v2/contracts/libs/Transfer.sol";

import { IWorker } from "./interfaces/IWorker.sol";

abstract contract Worker is AccessControl, Pausable, IWorker {
    bytes32 internal constant MESSAGE_LIB_ROLE = keccak256("MESSAGE_LIB_ROLE");
    bytes32 internal constant ALLOWLIST = keccak256("ALLOWLIST");
    bytes32 internal constant DENYLIST = keccak256("DENYLIST");
    bytes32 internal constant ADMIN_ROLE = keccak256("ADMIN_ROLE");

    address public workerFeeLib;

    uint64 public allowlistSize;
    uint16 public defaultMultiplierBps;
    address public priceFeed;

    mapping(uint32 eid => uint8[] optionTypes) internal supportedOptionTypes;

    // ========================= Constructor =========================

    /// @param _messageLibs array of message lib addresses that are granted the MESSAGE_LIB_ROLE
    /// @param _priceFeed price feed address
    /// @param _defaultMultiplierBps default multiplier for worker fee
    /// @param _roleAdmin address that is granted the DEFAULT_ADMIN_ROLE (can grant and revoke all roles)
    /// @param _admins array of admin addresses that are granted the ADMIN_ROLE
    constructor(
        address[] memory _messageLibs,
        address _priceFeed,
        uint16 _defaultMultiplierBps,
        address _roleAdmin,
        address[] memory _admins
    ) {
        defaultMultiplierBps = _defaultMultiplierBps;
        priceFeed = _priceFeed;

        if (_roleAdmin != address(0x0)) {
            _grantRole(DEFAULT_ADMIN_ROLE, _roleAdmin); // _roleAdmin can grant and revoke all roles
        }

        for (uint256 i = 0; i < _messageLibs.length; ++i) {
            _grantRole(MESSAGE_LIB_ROLE, _messageLibs[i]);
        }

        for (uint256 i = 0; i < _admins.length; ++i) {
            _grantRole(ADMIN_ROLE, _admins[i]);
        }
    }

    // ========================= Modifier =========================

    modifier onlyAcl(address _sender) {
        if (!hasAcl(_sender)) {
            revert Worker_NotAllowed();
        }
        _;
    }

    /// @dev Access control list using allowlist and denylist
    /// @dev 1) if one address is in the denylist -> deny
    /// @dev 2) else if address in the allowlist OR allowlist is empty (allows everyone)-> allow
    /// @dev 3) else deny
    /// @param _sender address to check
    function hasAcl(address _sender) public view returns (bool) {
        if (hasRole(DENYLIST, _sender)) {
            return false;
        } else if (allowlistSize == 0 || hasRole(ALLOWLIST, _sender)) {
            return true;
        } else {
            return false;
        }
    }

    // ========================= OnyDefaultAdmin =========================

    /// @dev flag to pause execution of workers (if used with whenNotPaused modifier)
    /// @param _paused true to pause, false to unpause
    function setPaused(bool _paused) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (_paused) {
            _pause();
        } else {
            _unpause();
        }
    }

    // ========================= OnlyAdmin =========================

    /// @param _priceFeed price feed address
    function setPriceFeed(address _priceFeed) external onlyRole(ADMIN_ROLE) {
        priceFeed = _priceFeed;
        emit SetPriceFeed(_priceFeed);
    }

    /// @param _workerFeeLib worker fee lib address
    function setWorkerFeeLib(address _workerFeeLib) external onlyRole(ADMIN_ROLE) {
        workerFeeLib = _workerFeeLib;
        emit SetWorkerLib(_workerFeeLib);
    }

    /// @param _multiplierBps default multiplier for worker fee
    function setDefaultMultiplierBps(uint16 _multiplierBps) external onlyRole(ADMIN_ROLE) {
        defaultMultiplierBps = _multiplierBps;
        emit SetDefaultMultiplierBps(_multiplierBps);
    }

    /// @dev supports withdrawing fee from ULN301, ULN302 and more
    /// @param _lib message lib address
    /// @param _to address to withdraw fee to
    /// @param _amount amount to withdraw
    function withdrawFee(address _lib, address _to, uint256 _amount) external onlyRole(ADMIN_ROLE) {
        if (!hasRole(MESSAGE_LIB_ROLE, _lib)) revert Worker_OnlyMessageLib();
        ISendLib(_lib).withdrawFee(_to, _amount);
        emit Withdraw(_lib, _to, _amount);
    }

    /// @dev supports withdrawing token from the contract
    /// @param _token token address
    /// @param _to address to withdraw token to
    /// @param _amount amount to withdraw
    function withdrawToken(address _token, address _to, uint256 _amount) external onlyRole(ADMIN_ROLE) {
        // transfers native if _token is address(0x0)
        Transfer.nativeOrToken(_token, _to, _amount);
    }

    function setSupportedOptionTypes(uint32 _eid, uint8[] calldata _optionTypes) external onlyRole(ADMIN_ROLE) {
        supportedOptionTypes[_eid] = _optionTypes;
    }

    // ========================= View Functions =========================
    function getSupportedOptionTypes(uint32 _eid) external view returns (uint8[] memory) {
        return supportedOptionTypes[_eid];
    }

    // ========================= Internal Functions =========================

    /// @dev overrides AccessControl to allow for counting of allowlistSize
    /// @param _role role to grant
    /// @param _account address to grant role to
    function _grantRole(bytes32 _role, address _account) internal override {
        if (_role == ALLOWLIST && !hasRole(_role, _account)) {
            ++allowlistSize;
        }
        super._grantRole(_role, _account);
    }

    /// @dev overrides AccessControl to allow for counting of allowlistSize
    /// @param _role role to revoke
    /// @param _account address to revoke role from
    function _revokeRole(bytes32 _role, address _account) internal override {
        if (_role == ALLOWLIST && hasRole(_role, _account)) {
            --allowlistSize;
        }
        super._revokeRole(_role, _account);
    }

    /// @dev overrides AccessControl to disable renouncing of roles
    function renounceRole(bytes32 /*role*/, address /*account*/) public pure override {
        revert Worker_RoleRenouncingDisabled();
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Permit.sol";

/**
 * @title IiTryToken
 * @notice Interface for the iTRY token contract
 * @dev This interface defines the functions that the iTryIssuer contract needs to interact with the iTRY token
 */
interface IiTryToken is IERC20, IERC20Permit, IERC20Metadata {
    /**
     * @notice Mint new iTRY tokens
     * @param to The address to receive the minted tokens
     * @param amount The amount of tokens to mint
     */
    function mint(address to, uint256 amount) external;

    /**
     * @notice Burn iTRY tokens from a specific address
     * @param from The address whose tokens will be burned
     * @param amount The amount of tokens to burn
     */
    function burnFrom(address from, uint256 amount) external;
}

// SPDX-License-Identifier: GPL-3.0

pragma solidity 0.8.20;

import {IStakediTryCooldown} from "./IStakediTryCooldown.sol";

/**
 * @title IStakediTryFastRedeem
 * @notice Interface for fast redemption functionality
 * @dev Extends IStakediTryCooldown with fast redemption capabilities that allow users to bypass
 *      the cooldown period by paying a fee
 */
interface IStakediTryFastRedeem is IStakediTryCooldown {
    // Events //
    /// @notice Event emitted when fast redeem is enabled/disabled
    event FastRedeemEnabledUpdated(bool enabled);
    /// @notice Event emitted when fast redeem fee is updated
    event FastRedeemFeeUpdated(uint16 previousFee, uint16 newFee);
    /// @notice Event emitted when fast redeem treasury is updated
    event FastRedeemTreasuryUpdated(address previousTreasury, address newTreasury);
    /// @notice Event emitted when a fast redemption occurs
    event FastRedeemed(
        address indexed owner, address indexed receiver, uint256 shares, uint256 assets, uint256 feeAssets
    );

    // Errors //
    /// @notice Error emitted when fast redeem is disabled
    error FastRedeemDisabled();
    /// @notice Error emitted when fast redeem fee exceeds maximum or is below minimum
    error InvalidFastRedeemFee();

    /**
     * @notice Fast redeem shares for immediate withdrawal with a fee
     * @param shares Amount of shares to redeem
     * @param receiver Address to receive the net assets
     * @param owner Address that owns the shares being redeemed
     * @return assets Net assets received by the receiver (after fee)
     */
    function fastRedeem(uint256 shares, address receiver, address owner) external returns (uint256 assets);

    /**
     * @notice Fast withdraw assets for immediate withdrawal with a fee
     * @param assets Amount of assets to withdraw (gross, before fee)
     * @param receiver Address to receive the net assets
     * @param owner Address that owns the shares being burned
     * @return shares Total shares burned
     */
    function fastWithdraw(uint256 assets, address receiver, address owner) external returns (uint256 shares);

    /**
     * @notice Enable or disable fast redemption feature
     * @param enabled True to enable, false to disable
     */
    function setFastRedeemEnabled(bool enabled) external;

    /**
     * @notice Set the fast redemption fee in basis points
     * @param feeInBPS Fee in basis points (e.g., 500 = 5%)
     */
    function setFastRedeemFee(uint16 feeInBPS) external;

    /**
     * @notice Set the treasury address that receives fast redemption fees
     * @param treasury Address of the treasury
     */
    function setFastRedeemTreasury(address treasury) external;

    /**
     * @notice Get the current fast redemption treasury address
     * @return address Treasury address
     */
    function fastRedeemTreasury() external view returns (address);

    /**
     * @notice Get the current fast redemption fee in basis points
     * @return uint16 Fee in basis points
     */
    function fastRedeemFeeInBPS() external view returns (uint16);

    /**
     * @notice Check if fast redemption is currently enabled
     * @return bool True if enabled
     */
    function fastRedeemEnabled() external view returns (bool);
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.17;

import "../mocks/RedstoneConsumerNumericMock.sol";
import "@openzeppelin/contracts/utils/Context.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @dev Implementation of the {IERC20} interface.
 *
 * This implementation is agnostic to the way tokens are created. This means
 * that a supply mechanism has to be added in a derived contract using {_mint}.
 * For a generic mechanism see {ERC20PresetMinterPauser}.
 *
 * TIP: For a detailed writeup see our guide
 * https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How
 * to implement supply mechanisms].
 *
 * We have followed general OpenZeppelin guidelines: functions revert instead
 * of returning `false` on failure. This behavior is nonetheless conventional
 * and does not conflict with the expectations of ERC20 applications.
 *
 * Additionally, an {Approval} event is emitted on calls to {transferFrom}.
 * This allows applications to reconstruct the allowance for all accounts just
 * by listening to said events. Other implementations of the EIP may not emit
 * these events, as it isn't required by the specification.
 *
 * Finally, the non-standard {decreaseAllowance} and {increaseAllowance}
 * functions have been added to mitigate the well-known issues around setting
 * allowances. See {IERC20-approve}.
 */
contract ERC20Initializable is Context, IERC20, IERC20Metadata {
  mapping(address => uint256) private _balances;

  mapping(address => mapping(address => uint256)) private _allowances;

  uint256 private _totalSupply;

  string private _name;
  string private _symbol;

  function initialize(string memory name_, string memory symbol_) internal {
    _name = name_;
    _symbol = symbol_;
  }

  /**
   * @dev Returns the name of the token.
   */
  function name() public view virtual override returns (string memory) {
    return _name;
  }

  /**
   * @dev Returns the symbol of the token, usually a shorter version of the
   * name.
   */
  function symbol() public view virtual override returns (string memory) {
    return _symbol;
  }

  /**
   * @dev Returns the number of decimals used to get its user representation.
   * For example, if `decimals` equals `2`, a balance of `505` tokens should
   * be displayed to a user as `5,05` (`505 / 10 ** 2`).
   *
   * Tokens usually opt for a value of 18, imitating the relationship between
   * Ether and Wei. This is the value {ERC20} uses, unless this function is
   * overridden;
   *
   * NOTE: This information is only used for _display_ purposes: it in
   * no way affects any of the arithmetic of the contract, including
   * {IERC20-balanceOf} and {IERC20-transfer}.
   */
  function decimals() public view virtual override returns (uint8) {
    return 18;
  }

  /**
   * @dev See {IERC20-totalSupply}.
   */
  function totalSupply() public view virtual override returns (uint256) {
    return _totalSupply;
  }

  /**
   * @dev See {IERC20-balanceOf}.
   */
  function balanceOf(address account) public view virtual override returns (uint256) {
    return _balances[account];
  }

  /**
   * @dev See {IERC20-transfer}.
   *
   * Requirements:
   *
   * - `recipient` cannot be the zero address.
   * - the caller must have a balance of at least `amount`.
   */
  function transfer(address recipient, uint256 amount) public virtual override returns (bool) {
    _transfer(_msgSender(), recipient, amount);
    return true;
  }

  /**
   * @dev See {IERC20-allowance}.
   */
  function allowance(address owner, address spender)
    public
    view
    virtual
    override
    returns (uint256)
  {
    return _allowances[owner][spender];
  }

  /**
   * @dev See {IERC20-approve}.
   *
   * Requirements:
   *
   * - `spender` cannot be the zero address.
   */
  function approve(address spender, uint256 amount) public virtual override returns (bool) {
    _approve(_msgSender(), spender, amount);
    return true;
  }

  /**
   * @dev See {IERC20-transferFrom}.
   *
   * Emits an {Approval} event indicating the updated allowance. This is not
   * required by the EIP. See the note at the beginning of {ERC20}.
   *
   * Requirements:
   *
   * - `sender` and `recipient` cannot be the zero address.
   * - `sender` must have a balance of at least `amount`.
   * - the caller must have allowance for ``sender``'s tokens of at least
   * `amount`.
   */
  function transferFrom(
    address sender,
    address recipient,
    uint256 amount
  ) public virtual override returns (bool) {
    _transfer(sender, recipient, amount);

    uint256 currentAllowance = _allowances[sender][_msgSender()];
    require(currentAllowance >= amount, "ERC20: transfer amount exceeds allowance");
    _approve(sender, _msgSender(), currentAllowance - amount);

    return true;
  }

  /**
   * @dev Atomically increases the allowance granted to `spender` by the caller.
   *
   * This is an alternative to {approve} that can be used as a mitigation for
   * problems described in {IERC20-approve}.
   *
   * Emits an {Approval} event indicating the updated allowance.
   *
   * Requirements:
   *
   * - `spender` cannot be the zero address.
   */
  function increaseAllowance(address spender, uint256 addedValue) public virtual returns (bool) {
    _approve(_msgSender(), spender, _allowances[_msgSender()][spender] + addedValue);
    return true;
  }

  /**
   * @dev Atomically decreases the allowance granted to `spender` by the caller.
   *
   * This is an alternative to {approve} that can be used as a mitigation for
   * problems described in {IERC20-approve}.
   *
   * Emits an {Approval} event indicating the updated allowance.
   *
   * Requirements:
   *
   * - `spender` cannot be the zero address.
   * - `spender` must have allowance for the caller of at least
   * `subtractedValue`.
   */
  function decreaseAllowance(address spender, uint256 subtractedValue)
    public
    virtual
    returns (bool)
  {
    uint256 currentAllowance = _allowances[_msgSender()][spender];
    require(currentAllowance >= subtractedValue, "ERC20: decreased allowance below zero");
    _approve(_msgSender(), spender, currentAllowance - subtractedValue);

    return true;
  }

  /**
   * @dev Moves tokens `amount` from `sender` to `recipient`.
   *
   * This is internal function is equivalent to {transfer}, and can be used to
   * e.g. implement automatic token fees, slashing mechanisms, etc.
   *
   * Emits a {Transfer} event.
   *
   * Requirements:
   *
   * - `sender` cannot be the zero address.
   * - `recipient` cannot be the zero address.
   * - `sender` must have a balance of at least `amount`.
   */
  function _transfer(
    address sender,
    address recipient,
    uint256 amount
  ) internal virtual {
    require(sender != address(0), "ERC20: transfer from the zero address");
    require(recipient != address(0), "ERC20: transfer to the zero address");

    _beforeTokenTransfer(sender, recipient, amount);

    uint256 senderBalance = _balances[sender];
    require(senderBalance >= amount, "ERC20: transfer amount exceeds balance");
    _balances[sender] = senderBalance - amount;
    _balances[recipient] += amount;

    emit Transfer(sender, recipient, amount);
  }

  /** @dev Creates `amount` tokens and assigns them to `account`, increasing
   * the total supply.
   *
   * Emits a {Transfer} event with `from` set to the zero address.
   *
   * Requirements:
   *
   * - `to` cannot be the zero address.
   */
  function _mint(address account, uint256 amount) internal virtual {
    require(account != address(0), "ERC20: mint to the zero address");

    _beforeTokenTransfer(address(0), account, amount);

    _totalSupply += amount;
    _balances[account] += amount;
    emit Transfer(address(0), account, amount);
  }

  /**
   * @dev Destroys `amount` tokens from `account`, reducing the
   * total supply.
   *
   * Emits a {Transfer} event with `to` set to the zero address.
   *
   * Requirements:
   *
   * - `account` cannot be the zero address.
   * - `account` must have at least `amount` tokens.
   */
  function _burn(address account, uint256 amount) internal virtual {
    require(account != address(0), "ERC20: burn from the zero address");

    _beforeTokenTransfer(account, address(0), amount);

    uint256 accountBalance = _balances[account];
    require(accountBalance >= amount, "ERC20: burn amount exceeds balance");
    _balances[account] = accountBalance - amount;
    _totalSupply -= amount;

    emit Transfer(account, address(0), amount);
  }

  /**
   * @dev Sets `amount` as the allowance of `spender` over the `owner` s tokens.
   *
   * This internal function is equivalent to `approve`, and can be used to
   * e.g. set automatic allowances for certain subsystems, etc.
   *
   * Emits an {Approval} event.
   *
   * Requirements:
   *
   * - `owner` cannot be the zero address.
   * - `spender` cannot be the zero address.
   */
  function _approve(
    address owner,
    address spender,
    uint256 amount
  ) internal virtual {
    require(owner != address(0), "ERC20: approve from the zero address");
    require(spender != address(0), "ERC20: approve to the zero address");

    _allowances[owner][spender] = amount;
    emit Approval(owner, spender, amount);
  }

  /**
   * @dev Hook that is called before any transfer of tokens. This includes
   * minting and burning.
   *
   * Calling conditions:
   *
   * - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
   * will be to transferred to `to`.
   * - when `from` is zero, `amount` tokens will be minted for `to`.
   * - when `to` is zero, `amount` of ``from``'s tokens will be burned.
   * - `from` and `to` are never both zero.
   *
   * To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
   */
  function _beforeTokenTransfer(
    address from,
    address to,
    uint256 amount
  ) internal virtual {}
}

contract SampleSyntheticToken is ERC20Initializable, Ownable, RedstoneConsumerNumericMock {
  bool private initialized;
  bytes32 public asset;
  address public broker;

  uint256 public constant MAX_SOLVENCY = 2**256 - 1;
  bytes32 public constant COLLATERAL_TOKEN = "ETH";
  uint256 public constant SOLVENCY_PRECISION = 1000; // 100%, 1 unit = 0.1%
  uint256 public constant MIN_SOLVENCY = 1200; // 120%, 1 unit = 0.1%
  uint256 public constant LIQUIDATION_BONUS = 50; // 5%, 1 unit = 0.1%

  mapping(address => uint256) public collateral;
  mapping(address => uint256) public debt;

  function initialize(
    bytes32 asset_,
    string memory name_,
    string memory symbol_
  ) external {
    require(!initialized);

    super.initialize(name_, symbol_);

    asset = asset_;

    initialized = true;
  }

  /**
   * @dev Mints koTokens increasing user's debt
   */
  function mint(uint256 amount) external payable remainsSolvent {
    super._mint(msg.sender, amount);
    debt[msg.sender] += amount;
    addCollateral();
  }

  /**
   * @dev Burns koTokens to reduce user debt
   */
  function burn(uint256 amount) external {
    require(debt[msg.sender] >= amount, "Cannot burn more than minted");
    debt[msg.sender] -= amount;
    super._burn(msg.sender, amount);
  }

  /**
   * @dev Adds collateral to user account
   * It could be done to increase the solvency ratio
   */
  function addCollateral() public payable virtual {
    collateral[msg.sender] += msg.value;
    emit CollateralAdded(msg.sender, msg.value, block.timestamp);
  }

  /**
   * @dev Removes outstanding collateral by paying out funds to depositor
   * The account must remain solvent after the operation
   */
  function removeCollateral(uint256 amount) external virtual remainsSolvent {
    require(collateral[msg.sender] >= amount, "Cannot remove more collateral than deposited");
    collateral[msg.sender] -= amount;
    payable(msg.sender).transfer(amount);
    emit CollateralRemoved(msg.sender, amount, block.timestamp);
  }

  /**
   * @dev Collateral amount expressed in ETH
   */
  function collateralOf(address account) public view virtual returns (uint256) {
    return collateral[account];
  }

  /**
   * @dev Collateral value expressed in USD
   */
  function collateralValueOf(address account) public view returns (uint256) {
    return collateralOf(account) * getOracleNumericValueFromTxMsg(COLLATERAL_TOKEN);
  }

  /**
   * @dev Debt of the account (number of koTokens minted)
   */
  function debtOf(address account) public view returns (uint256) {
    return debt[account];
  }

  /**
   * @dev Debt of the account expressed in USD
   */
  function debtValueOf(address account) public view returns (uint256) {
    return debt[account] * getOracleNumericValueFromTxMsg(asset);
  }

  /**
   * @dev A ratio between the value of collateral and debt of the account
   * It's expressed in permills - 0.1% (ratio 1 to 1 is equal to 1000 units)
   * To leave a margin for price movement and liquidation it must remain safely abot 1000
   */
  function solvencyOf(address account) public view returns (uint256) {
    if (debtValueOf(account) == 0) {
      return MAX_SOLVENCY;
    } else {
      return (collateralValueOf(account) * SOLVENCY_PRECISION) / debtValueOf(account);
    }
  }

  /**
   * @dev Value of komodo tokens held by given account at the current market price
   */
  function balanceValueOf(address account) public view returns (uint256) {
    return balanceOf(account) * getOracleNumericValueFromTxMsg(asset);
  }

  /**
   * @dev Total value of all minted komodo tokens at the current market price
   */
  function totalValue() public view returns (uint256) {
    return totalSupply() * getOracleNumericValueFromTxMsg(asset);
  }

  function liquidate(address account, uint256 amount) public {
    require(solvencyOf(account) < MIN_SOLVENCY, "Cannot liquidate a solvent account");
    this.transferFrom(msg.sender, account, amount);
    super._burn(account, amount);
    debt[account] -= amount;

    // Liquidator reward
    uint256 collateralRepayment = (amount * getOracleNumericValueFromTxMsg(asset)) /
      getOracleNumericValueFromTxMsg(COLLATERAL_TOKEN);
    uint256 bonus = (collateralRepayment * LIQUIDATION_BONUS) / SOLVENCY_PRECISION;

    uint256 repaymentWithBonus = collateralRepayment + bonus;
    collateral[account] -= repaymentWithBonus;
    payable(msg.sender).transfer(repaymentWithBonus);

    require(solvencyOf(account) >= MIN_SOLVENCY, "Account must be solvent after liquidation");
  }

  modifier remainsSolvent() {
    _;
    require(solvencyOf(msg.sender) >= MIN_SOLVENCY, "The account must remain solvent");
  }

  // EVENTS
  event CollateralAdded(address account, uint256 val, uint256 time);
  event CollateralRemoved(address account, uint256 val, uint256 time);
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { ERC165 } from "@openzeppelin/contracts/utils/introspection/ERC165.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

import { IMessageLib, MessageLibType } from "../interfaces/IMessageLib.sol";
import { Packet } from "../interfaces/ISendLib.sol";
import { ILayerZeroEndpointV2, MessagingFee, Origin } from "../interfaces/ILayerZeroEndpointV2.sol";
import { Errors } from "../libs/Errors.sol";
import { PacketV1Codec } from "./libs/PacketV1Codec.sol";
import { Transfer } from "../libs/Transfer.sol";

contract SimpleMessageLib is Ownable, ERC165 {
    using SafeERC20 for IERC20;
    using PacketV1Codec for bytes;

    address public immutable endpoint;
    address public immutable treasury;
    uint32 public immutable localEid;
    uint8 public constant PACKET_VERSION = 1;

    address public whitelistCaller;

    uint256 public lzTokenFee;
    uint256 public nativeFee;

    bytes public defaultOption;

    error OnlyEndpoint();
    error OnlyWhitelistCaller();
    error InvalidEndpoint(address expected, address actual);
    error ToIsAddressZero();
    error LzTokenIsAddressZero();
    error TransferFailed();

    // only the endpoint can call SEND() and setConfig()
    modifier onlyEndpoint() {
        if (endpoint != msg.sender) {
            revert OnlyEndpoint();
        }
        _;
    }

    constructor(address _endpoint, address _treasury) {
        endpoint = _endpoint;
        treasury = _treasury;
        localEid = ILayerZeroEndpointV2(_endpoint).eid();
        lzTokenFee = 99;
        nativeFee = 100;
        //        defaultOption = Options.encodeLegacyOptionsType1(200000);
    }

    function supportsInterface(bytes4 interfaceId) public view override returns (bool) {
        return interfaceId == type(IMessageLib).interfaceId || super.supportsInterface(interfaceId);
    }

    // no validation logic at all
    function validatePacket(bytes calldata packetBytes) external {
        if (whitelistCaller != address(0x0) && msg.sender != whitelistCaller) {
            revert OnlyWhitelistCaller();
        }
        Origin memory origin = Origin(packetBytes.srcEid(), packetBytes.sender(), packetBytes.nonce());
        ILayerZeroEndpointV2(endpoint).verify(origin, packetBytes.receiverB20(), keccak256(packetBytes.payload()));
    }

    // ------------------ onlyEndpoint ------------------
    function send(
        Packet calldata _packet,
        bytes memory _options,
        bool _payInLzToken
    ) external onlyEndpoint returns (MessagingFee memory fee, bytes memory encodedPacket, bytes memory options) {
        encodedPacket = PacketV1Codec.encode(_packet);

        options = _options.length == 0 ? defaultOption : _options;
        _handleMessagingParamsHook(encodedPacket, options);

        fee = MessagingFee(nativeFee, _payInLzToken ? lzTokenFee : 0);
    }

    // ------------------ onlyOwner ------------------
    function setDefaultOption(bytes memory _defaultOption) external onlyOwner {
        defaultOption = _defaultOption;
    }

    function setMessagingFee(uint256 _nativeFee, uint256 _lzTokenFee) external onlyOwner {
        nativeFee = _nativeFee;
        lzTokenFee = _lzTokenFee;
    }

    function setWhitelistCaller(address _whitelistCaller) external onlyOwner {
        whitelistCaller = _whitelistCaller;
    }

    function withdrawFee(address _to, uint256 _amount) external onlyOwner {
        if (_to == address(0x0)) {
            revert ToIsAddressZero();
        }

        address altTokenAddr = ILayerZeroEndpointV2(endpoint).nativeToken();

        // transfers native if altTokenAddr == address(0x0)
        Transfer.nativeOrToken(altTokenAddr, _to, _amount);
    }

    function withdrawLzTokenFee(address _to, uint256 _amount) external onlyOwner {
        if (_to == address(0x0)) {
            revert ToIsAddressZero();
        }
        address lzToken = ILayerZeroEndpointV2(endpoint).lzToken();
        if (lzToken == address(0x0)) {
            revert LzTokenIsAddressZero();
        }
        IERC20(lzToken).safeTransfer(_to, _amount);
    }

    // ------------------ View ------------------
    function quote(
        Packet calldata /*_packet*/,
        bytes calldata /*_options*/,
        bool _payInLzToken
    ) external view returns (MessagingFee memory) {
        return MessagingFee(nativeFee, _payInLzToken ? lzTokenFee : 0);
    }

    function isSupportedEid(uint32) external pure returns (bool) {
        return true;
    }

    function version() external pure returns (uint64 major, uint8 minor, uint8 endpointVersion) {
        return (0, 0, 2);
    }

    function messageLibType() external pure returns (MessageLibType) {
        return MessageLibType.SendAndReceive;
    }

    // ------------------ Internal ------------------
    function _handleMessagingParamsHook(bytes memory _encodedPacket, bytes memory _options) internal virtual {}

    fallback() external payable {
        revert Errors.LZ_NotImplemented();
    }

    receive() external payable {}
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { Transfer } from "@layerzerolabs/lz-evm-protocol-v2/contracts/libs/Transfer.sol";
import { ISendLib } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ISendLib.sol";

import { ILayerZeroDVN } from "../../interfaces/ILayerZeroDVN.sol";
import { Worker } from "../../../Worker.sol";
import { DVNAdapterMessageCodec } from "./libs/DVNAdapterMessageCodec.sol";

interface ISendLibBase {
    function fees(address _worker) external view returns (uint256);
}

interface IReceiveUln {
    function verify(bytes calldata _packetHeader, bytes32 _payloadHash, uint64 _confirmations) external;
}

struct ReceiveLibParam {
    address sendLib;
    uint32 dstEid;
    bytes32 receiveLib;
}

/// @title SendDVNAdapterBase
/// @notice base contract for DVN adapters
/// @dev limitations:
///  - doesn't accept alt token
///  - doesn't respect block confirmations
abstract contract DVNAdapterBase is Worker, ILayerZeroDVN {
    // --- Errors ---
    error DVNAdapter_InsufficientBalance(uint256 actual, uint256 requested);
    error DVNAdapter_NotImplemented();
    error DVNAdapter_MissingRecieveLib(address sendLib, uint32 dstEid);

    event ReceiveLibsSet(ReceiveLibParam[] params);

    /// @dev on change of application config, dvn adapters will not perform any additional verification
    /// @dev to avoid messages from being stuck, all verifications from adapters will be done with the maximum possible confirmations
    uint64 internal constant MAX_CONFIRMATIONS = type(uint64).max;

    /// @dev receive lib to call verify() on at destination
    mapping(address sendLib => mapping(uint32 dstEid => bytes32 receiveLib)) public receiveLibs;

    constructor(
        address _roleAdmin,
        address[] memory _admins,
        uint16 _defaultMultiplierBps
    ) Worker(new address[](0), address(0x0), _defaultMultiplierBps, _roleAdmin, _admins) {}

    // ========================= OnlyAdmin =========================
    /// @notice sets receive lib for destination chains
    /// @dev DEFAULT_ADMIN_ROLE can set MESSAGE_LIB_ROLE for sendLibs and use below function to set receiveLibs
    function setReceiveLibs(ReceiveLibParam[] calldata _params) external onlyRole(DEFAULT_ADMIN_ROLE) {
        for (uint256 i = 0; i < _params.length; i++) {
            ReceiveLibParam calldata param = _params[i];
            receiveLibs[param.sendLib][param.dstEid] = param.receiveLib;
        }

        emit ReceiveLibsSet(_params);
    }

    // ========================= Internal =========================
    function _getAndAssertReceiveLib(address _sendLib, uint32 _dstEid) internal view returns (bytes32 lib) {
        lib = receiveLibs[_sendLib][_dstEid];
        if (lib == bytes32(0)) revert DVNAdapter_MissingRecieveLib(_sendLib, _dstEid);
    }

    function _encode(
        bytes32 _receiveLib,
        bytes memory _packetHeader,
        bytes32 _payloadHash
    ) internal pure returns (bytes memory) {
        return DVNAdapterMessageCodec.encode(_receiveLib, _packetHeader, _payloadHash);
    }

    function _encodeEmpty() internal pure returns (bytes memory) {
        return
            DVNAdapterMessageCodec.encode(bytes32(0), new bytes(DVNAdapterMessageCodec.PACKET_HEADER_SIZE), bytes32(0));
    }

    function _decodeAndVerify(uint32 _srcEid, bytes calldata _payload) internal {
        require((DVNAdapterMessageCodec.srcEid(_payload) % 30000) == _srcEid, "DVNAdapterBase: invalid srcEid");

        (address receiveLib, bytes memory packetHeader, bytes32 payloadHash) = DVNAdapterMessageCodec.decode(_payload);

        IReceiveUln(receiveLib).verify(packetHeader, payloadHash, MAX_CONFIRMATIONS);
    }

    function _withdrawFeeFromSendLib(address _sendLib, address _to) internal {
        uint256 fee = ISendLibBase(_sendLib).fees(address(this));
        if (fee > 0) {
            ISendLib(_sendLib).withdrawFee(_to, fee);
            emit Withdraw(_sendLib, _to, fee);
        }
    }

    function _assertBalanceAndWithdrawFee(address _sendLib, uint256 _messageFee) internal {
        uint256 balance = address(this).balance;
        if (balance < _messageFee) {
            // withdraw all fees from the sendLib if balance is insufficient
            _withdrawFeeFromSendLib(_sendLib, address(this));

            // check balance again
            balance = address(this).balance;
            // revert if balance is still insufficient, need to transfer more funds manually to the adapter
            if (balance < _messageFee) revert DVNAdapter_InsufficientBalance(balance, _messageFee);
        }
    }

    /// @dev to receive refund
    receive() external payable {}
}

// SPDX-License-Identifier: MIT

pragma solidity 0.8.20;

import {IStakediTry} from "./IStakediTry.sol";

struct UserCooldown {
    uint104 cooldownEnd;
    uint152 underlyingAmount;
}

interface IStakediTryCooldown is IStakediTry {
    // Events //
    /// @notice Event emitted when cooldown duration updates
    event CooldownDurationUpdated(uint24 previousDuration, uint24 newDuration);

    // Errors //
    /// @notice Error emitted when the shares amount to redeem is greater than the shares balance of the owner
    error ExcessiveRedeemAmount();
    /// @notice Error emitted when the shares amount to withdraw is greater than the shares balance of the owner
    error ExcessiveWithdrawAmount();
    /// @notice Error emitted when cooldown value is invalid
    error InvalidCooldown();

    function cooldownAssets(uint256 assets) external returns (uint256 shares);

    function cooldownShares(uint256 shares) external returns (uint256 assets);

    function unstake(address receiver) external;

    function setCooldownDuration(uint24 duration) external;
}

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

import { IERC165 } from "@openzeppelin/contracts/utils/introspection/IERC165.sol";

import { SetConfigParam } from "./IMessageLibManager.sol";

enum MessageLibType {
    Send,
    Receive,
    SendAndReceive
}

interface IMessageLib is IERC165 {
    function setConfig(address _oapp, SetConfigParam[] calldata _config) external;

    function getConfig(uint32 _eid, address _oapp, uint32 _configType) external view returns (bytes memory config);

    function isSupportedEid(uint32 _eid) external view returns (bool);

    // message libs of same major version are compatible
    function version() external view returns (uint64 major, uint8 minor, uint8 endpointVersion);

    function messageLibType() external view returns (MessageLibType);
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {StakediTryV2} from "./StakediTryCooldown.sol";
import {IStakediTryFastRedeem} from "./interfaces/IStakediTryFastRedeem.sol";

/**
 * @title StakediTryFastRedeem
 * @notice Extends StakediTryV2 with fast redemption functionality
 * @dev Allows users to bypass the cooldown period by paying a fee that goes to the treasury.
 *      This provides liquidity to users who need immediate access to their funds while
 *      maintaining the protocol's stability through fee collection.
 */
contract StakediTryFastRedeem is StakediTryV2, IStakediTryFastRedeem {
    using SafeERC20 for IERC20;

    /// @notice Basis points denominator for fee calculations (100% = 10000 basis points)
    uint256 private constant BASIS_POINTS = 10000;

    /// @notice Fast redemption configuration
    address public fastRedeemTreasury;
    uint16 public fastRedeemFeeInBPS;
    bool public fastRedeemEnabled;
    uint16 public constant MIN_FAST_REDEEM_FEE = 1; // 0.01% minimum fee (1 basis point)
    uint16 public constant MAX_FAST_REDEEM_FEE = 2000; // 20% maximum fee

    /// @notice ensure fast redeem is enabled
    modifier ensureFastRedeemEnabled() {
        if (!fastRedeemEnabled) revert FastRedeemDisabled();
        _;
    }

    /**
     * @notice Constructor for StakediTryFastRedeem contract
     * @param _asset The address of the iTry token
     * @param initialRewarder The address of the initial rewarder
     * @param owner The address of the admin role
     * @param _fastRedeemTreasury The address that will receive fast redemption fees
     */
    constructor(IERC20 _asset, address initialRewarder, address owner, address _fastRedeemTreasury)
        StakediTryV2(_asset, initialRewarder, owner)
    {
        if (_fastRedeemTreasury == address(0)) revert InvalidZeroAddress();

        fastRedeemTreasury = _fastRedeemTreasury;
        fastRedeemEnabled = false;
        fastRedeemFeeInBPS = MAX_FAST_REDEEM_FEE; // Start at maximum fee (20%)
    }

    /* ------------- EXTERNAL ------------- */

    /**
     * @inheritdoc IStakediTryFastRedeem
     */
    function fastRedeem(uint256 shares, address receiver, address owner)
        external
        ensureCooldownOn
        ensureFastRedeemEnabled
        returns (uint256 assets)
    {
        if (shares > maxRedeem(owner)) revert ExcessiveRedeemAmount();

        uint256 totalAssets = previewRedeem(shares);
        uint256 feeAssets = _redeemWithFee(shares, totalAssets, receiver, owner);

        emit FastRedeemed(owner, receiver, shares, totalAssets, feeAssets);

        return totalAssets - feeAssets;
    }

    /**
     * @inheritdoc IStakediTryFastRedeem
     */
    function fastWithdraw(uint256 assets, address receiver, address owner)
        external
        ensureCooldownOn
        ensureFastRedeemEnabled
        returns (uint256 shares)
    {
        if (assets > maxWithdraw(owner)) revert ExcessiveWithdrawAmount();

        uint256 totalShares = previewWithdraw(assets);
        uint256 feeAssets = _redeemWithFee(totalShares, assets, receiver, owner);

        emit FastRedeemed(owner, receiver, totalShares, assets, feeAssets);

        return totalShares;
    }

    /**
     * @inheritdoc IStakediTryFastRedeem
     */
    function setFastRedeemEnabled(bool enabled) external onlyRole(DEFAULT_ADMIN_ROLE) {
        fastRedeemEnabled = enabled;
        emit FastRedeemEnabledUpdated(enabled);
    }

    /**
     * @inheritdoc IStakediTryFastRedeem
     */
    function setFastRedeemFee(uint16 feeInBPS) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (feeInBPS < MIN_FAST_REDEEM_FEE || feeInBPS > MAX_FAST_REDEEM_FEE) {
            revert InvalidFastRedeemFee();
        }

        uint16 previousFee = fastRedeemFeeInBPS;
        fastRedeemFeeInBPS = feeInBPS;
        emit FastRedeemFeeUpdated(previousFee, feeInBPS);
    }

    /**
     * @inheritdoc IStakediTryFastRedeem
     */
    function setFastRedeemTreasury(address treasury) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (treasury == address(0)) revert InvalidZeroAddress();

        address previousTreasury = fastRedeemTreasury;
        fastRedeemTreasury = treasury;
        emit FastRedeemTreasuryUpdated(previousTreasury, treasury);
    }

    /* ------------- INTERNAL ------------- */

    /**
     * @notice Internal helper to perform redemption with fee split
     * @dev Handles the logic of splitting redemption into treasury fee and net user amount.
     *      Follows ERC4626 naming: "redeem" burns shares to withdraw assets.
     *      Reverts if the fee rounds down to zero.
     *      MIN_SHARES validation happens automatically in _withdraw() calls.
     * @param shares Total shares to burn from owner's balance
     * @param assets Total assets being withdrawn (gross amount before fee deduction)
     * @param receiver Address to receive the net assets (after fee is deducted)
     * @param owner Address that owns the shares being burned (must have approved caller if caller != owner)
     * @return feeAssets Amount of assets sent to treasury as fee
     */
    function _redeemWithFee(uint256 shares, uint256 assets, address receiver, address owner)
        internal
        returns (uint256 feeAssets)
    {
        feeAssets = (assets * fastRedeemFeeInBPS) / BASIS_POINTS;

        // Enforce that fast redemption always has a cost
        if (feeAssets == 0) revert InvalidAmount();

        uint256 feeShares = previewWithdraw(feeAssets);
        uint256 netShares = shares - feeShares;
        uint256 netAssets = assets - feeAssets;

        // Withdraw fee portion to treasury
        _withdraw(_msgSender(), fastRedeemTreasury, owner, feeAssets, feeShares);

        // Withdraw net portion to receiver
        _withdraw(_msgSender(), receiver, owner, netAssets, netShares);
    }
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { AxelarExecutable } from "@axelar-network/axelar-gmp-sdk-solidity/contracts/executable/AxelarExecutable.sol";
import { ISendLib } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ISendLib.sol";

import { DVNAdapterBase } from "../DVNAdapterBase.sol";
import { IAxelarDVNAdapter } from "../../../interfaces/adapters/IAxelarDVNAdapter.sol";
import { IAxelarDVNAdapterFeeLib } from "../../../interfaces/adapters/IAxelarDVNAdapterFeeLib.sol";

interface ISendLibBase {
    function fees(address _worker) external view returns (uint256);
}

/// @title AxelarDVNAdapter
/// @dev How Axelar DVN Adapter works:
///  1. Estimate gas fee off-chain using the Axelar SDK.
///     refer to https://docs.axelar.dev/dev/axelarjs-sdk/axelar-query-api#estimategasfee
///  2. Pay gas fee by calling `payNativeGasForContractCall` on the AxelarGasService contract.
///     refer to https://docs.axelar.dev/dev/general-message-passing/gas-services/pay-gas#paynativegasforcontractcall
///  3. Send message by calling `callContract` on the AxelarGateway contract.
///     refer to https://docs.axelar.dev/dev/general-message-passing/gmp-messages#call-a-contract-on-chain-b-from-chain-a
///  4. Refund surplus gas fee asynchronously.
///     refer to https://docs.axelar.dev/dev/general-message-passing/gas-services/refund
/// @dev Recovery:
///  1. If not enough gas fee is paid, the message will be hangup on source chain and can `add gas` to retry.
///     refer to https://docs.axelar.dev/dev/general-message-passing/recovery#increase-gas-payment-to-the-gas-receiver-on-the-source-chain
///  2. If the message is not executed on the destination chain, you can manually retry by calling `execute` on the `ReceiveAxelarDVNAdapter` contract.
///     refer to https://docs.axelar.dev/dev/general-message-passing/recovery#manually-execute-a-transfer
/// @dev As the Gas is estimated off-chain, we need to update the gas fee periodically on-chain by calling `setNativeGasFee` with the new fee.
contract AxelarDVNAdapter is DVNAdapterBase, AxelarExecutable, IAxelarDVNAdapter {
    mapping(string srcChainName => SrcConfig) public srcConfig; // by chain name
    mapping(uint32 dstEid => DstConfig) public dstConfig; // by dstEid

    // set default multiplier to 2.5x
    constructor(
        address[] memory _admins,
        address _gateway
    ) AxelarExecutable(_gateway) DVNAdapterBase(msg.sender, _admins, 10000) {}

    // ========================= OnlyAdmin =========================
    function setDstConfig(DstConfigParam[] calldata _params) external onlyRole(ADMIN_ROLE) {
        for (uint256 i = 0; i < _params.length; i++) {
            DstConfigParam calldata param = _params[i];

            uint32 eid = param.eid % 30000;

            // set once per chainName
            // only one adapter per dvn that services both endpoint v1 and v2
            // we standardize the eid stored here with mod 30000
            if (bytes(dstConfig[eid].chainName).length == 0) {
                dstConfig[eid].chainName = param.chainName;
                dstConfig[eid].peer = param.peer;
                srcConfig[param.chainName].eid = eid;
                srcConfig[param.chainName].peer = param.peer;
            }

            dstConfig[eid].multiplierBps = param.multiplierBps;
            dstConfig[eid].nativeGasFee = param.nativeGasFee;
        }

        emit DstConfigSet(_params);
    }

    /// @notice sets message fee in native gas for destination chains.
    /// @dev Axelar does not support quoting fee on-chain. Instead, the fee needs to be obtained off-chain by querying through the Axelar SDK.
    /// @dev The fee may change over time when token prices change, requiring admins to monitor and make necessary updates to reflect the actual fee.
    /// @dev Adding a buffer to the required fee is advisable. Any surplus fee will be refunded asynchronously if it exceeds the necessary amount.
    /// https://docs.axelar.dev/dev/general-message-passing/gas-services/pay-gas
    /// https://github.com/axelarnetwork/axelarjs/blob/070c8fe061f1082e79772fdc5c4675c0237bbba2/packages/api/src/axelar-query/isomorphic.ts#L54
    /// https://github.com/axelarnetwork/axelar-cgp-solidity/blob/d4536599321774927bf9716178a9e360f8e0efac/contracts/gas-service/AxelarGasService.sol#L403
    function setNativeGasFee(NativeGasFeeParam[] calldata _params) external onlyRole(ADMIN_ROLE) {
        for (uint256 i = 0; i < _params.length; i++) {
            NativeGasFeeParam calldata param = _params[i];
            dstConfig[param.dstEid % 30000].nativeGasFee = param.nativeGasFee;
        }
        emit NativeGasFeeSet(_params);
    }

    // ========================= OnlyWorkerFeeLib =========================
    function withdrawToFeeLib(address _sendLib) external {
        if (msg.sender != workerFeeLib) revert AxelarDVNAdapter_OnlyWorkerFeeLib();

        _withdrawFeeFromSendLib(_sendLib, workerFeeLib);
    }

    // ========================= OnlyMessageLib =========================
    function assignJob(
        AssignJobParam calldata _param,
        bytes calldata _options
    ) external payable override onlyAcl(_param.sender) returns (uint totalFee) {
        bytes32 receiveLib = _getAndAssertReceiveLib(msg.sender, _param.dstEid);

        IAxelarDVNAdapterFeeLib.Param memory feeLibParam = IAxelarDVNAdapterFeeLib.Param({
            dstEid: _param.dstEid,
            confirmations: _param.confirmations,
            sender: _param.sender,
            defaultMultiplierBps: defaultMultiplierBps
        });
        DstConfig memory config = dstConfig[_param.dstEid % 30000];

        bytes memory payload = _encode(receiveLib, _param.packetHeader, _param.payloadHash);

        totalFee = IAxelarDVNAdapterFeeLib(workerFeeLib).getFeeOnSend(
            feeLibParam,
            config,
            payload,
            _options,
            msg.sender
        );

        gateway.callContract(config.chainName, config.peer, payload);
    }

    // ========================= View =========================
    function getFee(
        uint32 _dstEid,
        uint64 _confirmations,
        address _sender,
        bytes calldata _options
    ) external view override returns (uint256 totalFee) {
        IAxelarDVNAdapterFeeLib.Param memory feeLibParam = IAxelarDVNAdapterFeeLib.Param(
            _dstEid,
            _confirmations,
            _sender,
            defaultMultiplierBps
        );

        totalFee = IAxelarDVNAdapterFeeLib(workerFeeLib).getFee(feeLibParam, dstConfig[_dstEid % 30000], _options);
    }

    // ========================= Internal =========================
    function _execute(
        string calldata _sourceChain,
        string calldata _sourceAddress,
        bytes calldata _payload
    ) internal override {
        SrcConfig memory config = srcConfig[_sourceChain];

        // assert peer is the same as the source chain
        _assertPeer(_sourceChain, _sourceAddress, config.peer);

        _decodeAndVerify(config.eid, _payload);
    }

    function _assertPeer(string memory _sourceChain, string memory _sourceAddress, string memory peer) private pure {
        if (keccak256(bytes(_sourceAddress)) != keccak256(bytes(peer))) {
            revert AxelarDVNAdapter_UntrustedPeer(_sourceChain, _sourceAddress);
        }
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Pausable} from "@openzeppelin/contracts/security/Pausable.sol";
import {Initializable} from "@openzeppelin/contracts/proxy/utils/Initializable.sol";

contract DLFToken is Initializable, ERC20, Ownable, Pausable {
    mapping(address => bool) private _isBlacklisted;

    constructor(address owner) ERC20("Digital Liquiditiy Fund Token Mock", "DLF") {
        _transferOwnership(owner);
        _mint(owner, 1000e18); // Test mint
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    function _beforeTokenTransfer(address from, address to, uint256 amount) internal override whenNotPaused {
        require(!_isBlacklisted[from], "ERC20: sender is blacklisted");
        require(!_isBlacklisted[to], "ERC20: recipient is blacklisted");
        super._beforeTokenTransfer(from, to, amount);
    }

    function blacklist(address account) public onlyOwner {
        require(account != address(0), "Blacklist: account is the zero address");
        _isBlacklisted[account] = true;
    }

    function unblacklist(address account) public onlyOwner {
        require(account != address(0), "Blacklist: account is the zero address");
        _isBlacklisted[account] = false;
    }

    function isBlacklisted(address account) public view returns (bool) {
        return _isBlacklisted[account];
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) public onlyOwner {
        _burn(from, amount);
    }
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { ILayerZeroUltraLightNodeV2 } from "@layerzerolabs/lz-evm-v1-0.7/contracts/interfaces/ILayerZeroUltraLightNodeV2.sol";

import { Worker } from "../../Worker.sol";
import { MultiSig } from "./MultiSig.sol";
import { ReadLib1002 } from "../readlib/ReadLib1002.sol";
import { IDVN } from "../interfaces/IDVN.sol";
import { IDVNFeeLib } from "../interfaces/IDVNFeeLib.sol";
import { IReceiveUlnE2 } from "../interfaces/IReceiveUlnE2.sol";

struct ExecuteParam {
    uint32 vid;
    address target;
    bytes callData;
    uint256 expiration;
    bytes signatures;
}

contract DVN is Worker, MultiSig, IDVN {
    // to uniquely identify this DVN instance
    // set to endpoint v1 eid if available OR endpoint v2 eid % 30_000
    uint32 public immutable vid;
    uint32 public immutable localEidV2; // endpoint-v2 only, for read call

    mapping(uint32 dstEid => DstConfig) public dstConfig;
    mapping(bytes32 executableHash => bool used) public usedHashes;

    error DVN_OnlySelf();
    error DVN_InvalidRole(bytes32 role);
    error DVN_InstructionExpired();
    error DVN_InvalidTarget(address target);
    error DVN_InvalidVid(uint32 vid);
    error DVN_InvalidSignatures();
    error DVN_DuplicatedHash(bytes32 executableHash);

    event VerifySignaturesFailed(uint256 idx);
    event ExecuteFailed(uint256 _index, bytes _data);
    event HashAlreadyUsed(ExecuteParam param, bytes32 _hash);
    // same as DVNFeePaid, but for ULNv2
    event VerifierFeePaid(uint256 fee);

    // ========================= Constructor =========================

    /// @dev DVN doesn't have a roleAdmin (address(0x0))
    /// @dev Supports all of ULNv2, ULN301, ULN302 and more
    /// @param _localEidV2 local endpoint-v2 eid
    /// @param _vid unique identifier for this DVN instance
    /// @param _messageLibs array of message lib addresses that are granted the MESSAGE_LIB_ROLE
    /// @param _priceFeed price feed address
    /// @param _signers array of signer addresses for multisig
    /// @param _quorum quorum for multisig
    /// @param _admins array of admin addresses that are granted the ADMIN_ROLE
    constructor(
        uint32 _localEidV2,
        uint32 _vid,
        address[] memory _messageLibs,
        address _priceFeed,
        address[] memory _signers,
        uint64 _quorum,
        address[] memory _admins
    ) Worker(_messageLibs, _priceFeed, 12000, address(0x0), _admins) MultiSig(_signers, _quorum) {
        vid = _vid;
        localEidV2 = _localEidV2;
    }

    // ========================= Modifier =========================

    /// @dev depending on role, restrict access to only self or admin
    /// @dev ALLOWLIST, DENYLIST, MESSAGE_LIB_ROLE can only be granted/revoked by self
    /// @dev ADMIN_ROLE can only be granted/revoked by admin
    /// @dev reverts if not one of the above roles
    /// @param _role role to check
    modifier onlySelfOrAdmin(bytes32 _role) {
        if (_role == ALLOWLIST || _role == DENYLIST || _role == MESSAGE_LIB_ROLE) {
            // self required
            if (address(this) != msg.sender) {
                revert DVN_OnlySelf();
            }
        } else if (_role == ADMIN_ROLE) {
            // admin required
            _checkRole(ADMIN_ROLE);
        } else {
            revert DVN_InvalidRole(_role);
        }
        _;
    }

    modifier onlySelf() {
        if (address(this) != msg.sender) {
            revert DVN_OnlySelf();
        }
        _;
    }

    // ========================= OnlySelf =========================

    /// @dev set signers for multisig
    /// @dev function sig 0x31cb6105
    /// @param _signer signer address
    /// @param _active true to add, false to remove
    function setSigner(address _signer, bool _active) external onlySelf {
        _setSigner(_signer, _active);
    }

    /// @dev set quorum for multisig
    /// @dev function sig 0x8585c945
    /// @param _quorum to set
    function setQuorum(uint64 _quorum) external onlySelf {
        _setQuorum(_quorum);
    }

    // ========================= OnlySelf / OnlyAdmin =========================

    /// @dev overrides AccessControl to allow self/admin to grant role'
    /// @dev function sig 0x2f2ff15d
    /// @param _role role to grant
    /// @param _account account to grant role to
    function grantRole(bytes32 _role, address _account) public override onlySelfOrAdmin(_role) {
        _grantRole(_role, _account);
    }

    /// @dev overrides AccessControl to allow self/admin to revoke role
    /// @dev function sig 0xd547741f
    /// @param _role role to revoke
    /// @param _account account to revoke role from
    function revokeRole(bytes32 _role, address _account) public override onlySelfOrAdmin(_role) {
        _revokeRole(_role, _account);
    }

    // ========================= OnlyQuorum =========================

    /// @notice function for quorum to change admin without going through execute function
    /// @dev calldata in the case is abi.encode new admin address
    function quorumChangeAdmin(ExecuteParam calldata _param) external {
        if (_param.expiration <= block.timestamp) {
            revert DVN_InstructionExpired();
        }
        if (_param.target != address(this)) {
            revert DVN_InvalidTarget(_param.target);
        }
        if (_param.vid != vid) {
            revert DVN_InvalidVid(_param.vid);
        }

        // generate and validate hash
        bytes32 hash = hashCallData(_param.vid, _param.target, _param.callData, _param.expiration);
        (bool sigsValid, ) = verifySignatures(hash, _param.signatures);
        if (!sigsValid) {
            revert DVN_InvalidSignatures();
        }
        if (usedHashes[hash]) {
            revert DVN_DuplicatedHash(hash);
        }

        usedHashes[hash] = true;
        _grantRole(ADMIN_ROLE, abi.decode(_param.callData, (address)));
    }

    // ========================= OnlyAdmin =========================

    /// @param _params array of DstConfigParam
    function setDstConfig(DstConfigParam[] calldata _params) external onlyRole(ADMIN_ROLE) {
        for (uint256 i = 0; i < _params.length; ++i) {
            DstConfigParam calldata param = _params[i];
            dstConfig[param.dstEid] = DstConfig(param.gas, param.multiplierBps, param.floorMarginUSD);
        }
        emit SetDstConfig(_params);
    }

    /// @dev takes a list of instructions and executes them in order
    /// @dev if any of the instructions fail, it will emit an error event and continue to execute the rest of the instructions
    /// @param _params array of ExecuteParam, includes target, callData, expiration, signatures
    function execute(ExecuteParam[] calldata _params) external onlyRole(ADMIN_ROLE) {
        for (uint256 i = 0; i < _params.length; ++i) {
            ExecuteParam calldata param = _params[i];
            // 1. skip if invalid vid
            if (param.vid != vid) {
                continue;
            }

            // 2. skip if expired
            if (param.expiration <= block.timestamp) {
                continue;
            }

            // generate and validate hash
            bytes32 hash = hashCallData(param.vid, param.target, param.callData, param.expiration);

            // 3. check signatures
            (bool sigsValid, ) = verifySignatures(hash, param.signatures);
            if (!sigsValid) {
                emit VerifySignaturesFailed(i);
                continue;
            }

            // 4. should check hash
            bool shouldCheckHash = _shouldCheckHash(bytes4(param.callData));
            if (shouldCheckHash) {
                if (usedHashes[hash]) {
                    emit HashAlreadyUsed(param, hash);
                    continue;
                } else {
                    usedHashes[hash] = true; // prevent reentry and replay attack
                }
            }

            (bool success, bytes memory rtnData) = param.target.call(param.callData);
            if (!success) {
                if (shouldCheckHash) {
                    // need to unset the usedHash otherwise it cant be used
                    usedHashes[hash] = false;
                }
                // emit an event in any case
                emit ExecuteFailed(i, rtnData);
            }
        }
    }

    /// @dev to support ULNv2
    /// @dev the withdrawFee function for ULN30X is built in the Worker contract
    /// @param _lib message lib address
    /// @param _to address to withdraw to
    /// @param _amount amount to withdraw
    function withdrawFeeFromUlnV2(address _lib, address payable _to, uint256 _amount) external onlyRole(ADMIN_ROLE) {
        if (!hasRole(MESSAGE_LIB_ROLE, _lib)) {
            revert Worker_OnlyMessageLib();
        }
        ILayerZeroUltraLightNodeV2(_lib).withdrawNative(_to, _amount);
    }

    // ========================= OnlyMessageLib =========================

    /// @dev for ULN301, ULN302 and more to assign job
    /// @dev dvn network can reject job from _sender by adding/removing them from allowlist/denylist
    /// @param _param assign job param
    /// @param _options dvn options
    function assignJob(
        AssignJobParam calldata _param,
        bytes calldata _options
    ) external payable onlyRole(MESSAGE_LIB_ROLE) onlyAcl(_param.sender) returns (uint256 totalFee) {
        IDVNFeeLib.FeeParams memory feeParams = IDVNFeeLib.FeeParams(
            priceFeed,
            _param.dstEid,
            _param.confirmations,
            _param.sender,
            quorum,
            defaultMultiplierBps
        );
        totalFee = IDVNFeeLib(workerFeeLib).getFeeOnSend(feeParams, dstConfig[_param.dstEid], _options);
    }

    /// @dev to support ULNv2
    /// @dev dvn network can reject job from _sender by adding/removing them from allowlist/denylist
    /// @param _dstEid destination EndpointId
    /// @param //_outboundProofType outbound proof type
    /// @param _confirmations block confirmations
    /// @param _sender message sender address
    function assignJob(
        uint16 _dstEid,
        uint16 /*_outboundProofType*/,
        uint64 _confirmations,
        address _sender
    ) external onlyRole(MESSAGE_LIB_ROLE) onlyAcl(_sender) returns (uint256 totalFee) {
        IDVNFeeLib.FeeParams memory params = IDVNFeeLib.FeeParams(
            priceFeed,
            _dstEid,
            _confirmations,
            _sender,
            quorum,
            defaultMultiplierBps
        );
        // ULNV2 does not have dvn options
        totalFee = IDVNFeeLib(workerFeeLib).getFeeOnSend(params, dstConfig[_dstEid], bytes(""));
        emit VerifierFeePaid(totalFee);
    }

    /// @dev to support CmdLib
    // @param _packetHeader - version + nonce + path
    // @param _cmd - the command to be executed to obtain the payload
    // @param _options - options
    function assignJob(
        address _sender,
        bytes calldata /*_packetHeader*/,
        bytes calldata _cmd,
        bytes calldata _options
    ) external payable onlyRole(MESSAGE_LIB_ROLE) onlyAcl(_sender) returns (uint256 fee) {
        IDVNFeeLib.FeeParamsForRead memory feeParams = IDVNFeeLib.FeeParamsForRead(
            priceFeed,
            _sender,
            quorum,
            defaultMultiplierBps
        );
        fee = IDVNFeeLib(workerFeeLib).getFeeOnSend(feeParams, dstConfig[localEidV2], _cmd, _options);
    }

    // ========================= View =========================

    /// @dev getFee can revert if _sender doesn't pass ACL
    /// @param _dstEid destination EndpointId
    /// @param _confirmations block confirmations
    /// @param _sender message sender address
    /// @param _options dvn options
    /// @return fee fee in native amount
    function getFee(
        uint32 _dstEid,
        uint64 _confirmations,
        address _sender,
        bytes calldata _options
    ) external view onlyAcl(_sender) returns (uint256 fee) {
        IDVNFeeLib.FeeParams memory params = IDVNFeeLib.FeeParams(
            priceFeed,
            _dstEid,
            _confirmations,
            _sender,
            quorum,
            defaultMultiplierBps
        );
        fee = IDVNFeeLib(workerFeeLib).getFee(params, dstConfig[_dstEid], _options);
    }

    /// @dev to support ULNv2
    /// @dev getFee can revert if _sender doesn't pass ACL
    /// @param _dstEid destination EndpointId
    /// @param //_outboundProofType outbound proof type
    /// @param _confirmations block confirmations
    /// @param _sender message sender address
    function getFee(
        uint16 _dstEid,
        uint16 /*_outboundProofType*/,
        uint64 _confirmations,
        address _sender
    ) public view onlyAcl(_sender) returns (uint256 fee) {
        IDVNFeeLib.FeeParams memory params = IDVNFeeLib.FeeParams(
            priceFeed,
            _dstEid,
            _confirmations,
            _sender,
            quorum,
            defaultMultiplierBps
        );
        fee = IDVNFeeLib(workerFeeLib).getFee(params, dstConfig[_dstEid], bytes(""));
    }

    /// @dev to support CmdLib
    // @param _packetHeader - version + nonce + path
    // @param _cmd - the command to be executed to obtain the payload
    // @param _options - options
    function getFee(
        address _sender,
        bytes calldata /*_packetHeader*/,
        bytes calldata _cmd,
        bytes calldata _options
    ) external view onlyAcl(_sender) returns (uint256 fee) {
        IDVNFeeLib.FeeParamsForRead memory feeParams = IDVNFeeLib.FeeParamsForRead(
            priceFeed,
            _sender,
            quorum,
            defaultMultiplierBps
        );
        fee = IDVNFeeLib(workerFeeLib).getFee(feeParams, dstConfig[localEidV2], _cmd, _options);
    }

    /// @param _target target address
    /// @param _callData call data
    /// @param _expiration expiration timestamp
    /// @return hash of above
    function hashCallData(
        uint32 _vid,
        address _target,
        bytes calldata _callData,
        uint256 _expiration
    ) public pure returns (bytes32) {
        return keccak256(abi.encodePacked(_vid, _target, _expiration, _callData));
    }

    // ========================= Internal =========================

    /// @dev to save gas, we don't check hash for some functions (where replaying won't change the state)
    /// @dev for example, some administrative functions like changing signers, the contract should check hash to double spending
    /// @dev should ensure that all onlySelf functions have unique functionSig
    /// @param _functionSig function signature
    /// @return true if should check hash
    function _shouldCheckHash(bytes4 _functionSig) internal pure returns (bool) {
        // never check for these selectors to save gas
        return
            _functionSig != IReceiveUlnE2.verify.selector && // 0x0223536e, replaying won't change the state
            _functionSig != ReadLib1002.verify.selector && // 0xab750e75, replaying won't change the state
            _functionSig != ILayerZeroUltraLightNodeV2.updateHash.selector; // 0x704316e5, replaying will be revert at uln
    }
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { ERC165 } from "@openzeppelin/contracts/utils/introspection/ERC165.sol";

import { IMessageLib, MessageLibType } from "../interfaces/IMessageLib.sol";
import { Errors } from "../libs/Errors.sol";

contract BlockedMessageLib is ERC165 {
    function supportsInterface(bytes4 interfaceId) public view override returns (bool) {
        return interfaceId == type(IMessageLib).interfaceId || super.supportsInterface(interfaceId);
    }

    function version() external pure returns (uint64 major, uint8 minor, uint8 endpointVersion) {
        return (type(uint64).max, type(uint8).max, 2);
    }

    function messageLibType() external pure returns (MessageLibType) {
        return MessageLibType.SendAndReceive;
    }

    function isSupportedEid(uint32) external pure returns (bool) {
        return true;
    }

    fallback() external {
        revert Errors.LZ_NotImplemented();
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {SingleAdminAccessControl} from "../utils/SingleAdminAccessControl.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import {IiTryIssuer} from "./interfaces/IiTryIssuer.sol";
import {IiTryToken} from "../token/iTRY/interfaces/IiTryToken.sol";
import {IFastAccessVault} from "./interfaces/IFastAccessVault.sol";
import {FastAccessVault} from "./FastAccessVault.sol";
import {IOracle} from "./periphery/IOracle.sol";
import {IYieldProcessor} from "./periphery/IYieldProcessor.sol";
import {CommonErrors} from "./periphery/CommonErrors.sol";

/**
 * @title iTryIssuer
 * @author Inverter Network
 * @notice Central issuer contract for iTRY stablecoins, managing minting, redemption, and yield distribution
 * @dev This contract acts as the controller for iTRY token supply, handling:
 *      - Minting iTRY against DLF collateral
 *      - Redeeming iTRY for DLF (from buffer vault or custodian)
 *      - Processing accumulated yield from NAV appreciation
 *      - Managing fees and whitelisted users
 *      - Coordinating between liquidity vault and custodian for collateral management
 *
 *
 *      The contract uses a role-based access control system with the following roles:
 *      - DEFAULT_ADMIN_ROLE: Can manage all roles and contract parameters
 *      - WHITELIST_MANAGER_ROLE: Can add/remove users from whitelist
 *      - YIELD_DISTRIBUTOR_ROLE: Can process and distribute yield
 *      - INTEGRATION_MANAGER_ROLE: Can set integration contract addresses
 *      - WHITELISTED_USER_ROLE: Can mint and redeem iTRY tokens
 *
 * @custom:security-contact security@inverter.network
 */

contract iTryIssuer is IiTryIssuer, SingleAdminAccessControl, ReentrancyGuard {
    using SafeERC20 for IERC20;

    // ============================================
    // Constants
    // ============================================

    /// @notice Maximum mint fee in basis points (100% = 10000 BPS)
    uint256 public constant MAX_MINT_FEE_BPS = 9999;

    /// @notice Maximum redeem fee in basis points (100% = 10000 BPS)
    uint256 public constant MAX_REDEEM_FEE_BPS = 9999;

    // ============================================
    // State Variables - Dependencies
    // ============================================

    /// @notice The iTRY token contract
    IiTryToken public immutable iTryToken;

    /// @notice The DLF collateral token contract
    IERC20 public immutable collateralToken;

    /// @notice The fast access liquidity vault for quick redemptions
    IFastAccessVault public immutable liquidityVault;

    /// @notice The oracle providing NAV price for DLF/iTRY conversion
    IOracle public oracle;

    /// @notice The custodian address for off-chain collateral management
    address public custodian;

    /// @notice The yield processor contract for distributing accumulated yield
    IYieldProcessor public yieldReceiver;

    // ============================================
    // State Variables - Fee Configuration
    // ============================================

    /// @notice Address receiving protocol fees
    address public treasury;

    /// @notice Mint fee in basis points (1 BPS = 0.01%)
    uint256 public mintFeeInBPS;

    /// @notice Redemption fee in basis points (1 BPS = 0.01%)
    uint256 public redemptionFeeInBPS;

    // ============================================
    // State Variables - Accounting
    // ============================================

    /// @notice Total amount of iTRY tokens currently in circulation
    uint256 private _totalIssuedITry;

    /// @notice Total amount of DLF collateral held under custody (vault + custodian)
    uint256 private _totalDLFUnderCustody;

    // ============================================
    // Access Control Roles
    // ============================================

    /// @notice Role that can manage the whitelist of users
    bytes32 private constant _WHITELIST_MANAGER_ROLE = keccak256("WHITELIST_MANAGER_ROLE");

    /// @notice Role that can mint new yield and distribute it
    bytes32 private constant _YIELD_DISTRIBUTOR_ROLE = keccak256("YIELD_DISTRIBUTOR_ROLE");

    /// @notice Role that can set the addresses of integration contracts
    bytes32 private constant _INTEGRATION_MANAGER_ROLE = keccak256("INTEGRATION_MANAGER_ROLE");

    /// @notice Role for whitelisted users who can mint and redeem iTRY
    bytes32 private constant _WHITELISTED_USER_ROLE = keccak256("WHITELISTED_USER_ROLE");

    // ============================================
    // Constructor
    // ============================================

    /**
     * @notice Initializes the iTryIssuer contract with all necessary dependencies
     * @param _iTryToken Address of the iTRY token contract
     * @param _collateralToken Address of the DLF collateral token
     * @param _oracle Address of the NAV price oracle
     * @param _treasury Address to receive protocol fees
     * @param _yieldReceiver Address of the yield processor contract
     * @param _custodian Address of the custodian integration
     * @param _initialAdmin Address to receive all initial admin roles
     * @param _initialIssued Initial amount of iTRY already issued (for migration scenarios)
     * @param _initialDLFUnderCustody Initial amount of DLF collateral under custody (for migration scenarios)
     * @param _vaultTargetPercentageBPS Target buffer percentage for FastAccessVault in basis points
     * @param _vaultMinimumBalance Minimum balance for FastAccessVault to maintain
     */
    constructor(
        address _iTryToken,
        address _collateralToken,
        address _oracle,
        address _treasury,
        address _yieldReceiver,
        address _custodian,
        address _initialAdmin,
        uint256 _initialIssued,
        uint256 _initialDLFUnderCustody,
        uint256 _vaultTargetPercentageBPS,
        uint256 _vaultMinimumBalance
    ) {
        if (_initialAdmin == address(0)) revert CommonErrors.ZeroAddress();
        if (_iTryToken == address(0)) revert CommonErrors.ZeroAddress();
        if (_collateralToken == address(0)) revert CommonErrors.ZeroAddress();

        // Deploy FastAccessVault internally with this contract as the issuer
        liquidityVault = IFastAccessVault(
            address(
                new FastAccessVault(
                    _collateralToken,
                    address(this), // Issuer is this contract
                    _custodian,
                    _vaultTargetPercentageBPS,
                    _vaultMinimumBalance,
                    _initialAdmin // Admin for vault ownership
                )
            )
        );

        iTryToken = IiTryToken(_iTryToken);
        collateralToken = IERC20(_collateralToken);
        _setOracle(_oracle);
        _setTreasury(_treasury);
        _setYieldReceiver(_yieldReceiver);
        _setCustodian(_custodian);

        // Set initial fees to 0
        redemptionFeeInBPS = 0;
        mintFeeInBPS = 0;

        // Set initial issued amount and collateral under custody
        _totalIssuedITry = _initialIssued;
        _totalDLFUnderCustody = _initialDLFUnderCustody;

        // Initial role setup - grant all roles to initial admin
        _grantRole(DEFAULT_ADMIN_ROLE, _initialAdmin);
        _grantRole(_WHITELIST_MANAGER_ROLE, _initialAdmin);
        _grantRole(_YIELD_DISTRIBUTOR_ROLE, _initialAdmin);
        _grantRole(_INTEGRATION_MANAGER_ROLE, _initialAdmin);

        // Note: The iTRY token admin should call addMinter(address(this)) to grant this contract the MINTER_CONTRACT role
    }

    // ============================================
    // View Functions - Minting Preview
    // ============================================

    /// @inheritdoc IiTryIssuer
    function previewMint(uint256 dlfAmount) external view returns (uint256 iTRYAmount) {
        if (dlfAmount == 0) revert CommonErrors.ZeroAmount();

        uint256 navPrice = oracle.price();
        uint256 netDlfAmount = dlfAmount;

        netDlfAmount = dlfAmount - _calculateMintFee(dlfAmount);

        // Calculate iTRY amount: netDlfAmount * navPrice / 1e18
        iTRYAmount = netDlfAmount * navPrice / 1e18;
    }

    // ============================================
    // View Functions - Redemption Preview
    // ============================================

    /// @inheritdoc IiTryIssuer
    function previewRedeem(uint256 iTRYAmount) external view returns (uint256 dlfAmount) {
        if (iTRYAmount == 0) revert CommonErrors.ZeroAmount();

        uint256 navPrice = oracle.price();

        // Calculate gross DLF amount: iTRYAmount * 1e18 / navPrice
        uint256 grossDlfAmount = iTRYAmount * 1e18 / navPrice;

        // Account for redemption fee if configured
        if (redemptionFeeInBPS > 0) {
            dlfAmount = grossDlfAmount - _calculateRedemptionFee(grossDlfAmount);
        } else {
            dlfAmount = grossDlfAmount;
        }

        return dlfAmount;
    }

    // ============================================
    // View Functions - Yield Preview
    // ============================================

    /// @inheritdoc IiTryIssuer
    function previewAccumulatedYield() external view returns (uint256) {
        uint256 navPrice = oracle.price();

        // Calculate total collateral value: _totalDLFUnderCustody * currentNAVPrice / 1e18
        uint256 currentCollateralValue = _totalDLFUnderCustody * navPrice / 1e18;
        if (currentCollateralValue <= _totalIssuedITry) {
            return 0;
        }
        return currentCollateralValue - _totalIssuedITry;
    }

    // ============================================
    // View Functions - Accounting
    // ============================================

    /// @inheritdoc IiTryIssuer
    function getTotalIssuedITry() external view returns (uint256) {
        return _totalIssuedITry;
    }

    /// @inheritdoc IiTryIssuer
    function getCollateralUnderCustody() external view returns (uint256) {
        return _totalDLFUnderCustody;
    }

    /// @inheritdoc IiTryIssuer
    function isWhitelistedUser(address user) external view returns (bool) {
        return hasRole(_WHITELISTED_USER_ROLE, user);
    }

    // ============================================
    // State-Changing Functions - Minting
    // ============================================

    /// @inheritdoc IiTryIssuer
    function mintITRY(uint256 dlfAmount, uint256 minAmountOut) external returns (uint256 iTRYAmount) {
        return mintFor(msg.sender, dlfAmount, minAmountOut);
    }

    /// @inheritdoc IiTryIssuer
    function mintFor(address recipient, uint256 dlfAmount, uint256 minAmountOut)
        public
        onlyRole(_WHITELISTED_USER_ROLE)
        nonReentrant
        returns (uint256 iTRYAmount)
    {
        // Validate recipient address
        if (recipient == address(0)) revert CommonErrors.ZeroAddress();

        // Validate dlfAmount > 0
        if (dlfAmount == 0) revert CommonErrors.ZeroAmount();

        // Get NAV price from oracle
        uint256 navPrice = oracle.price();
        if (navPrice == 0) revert InvalidNAVPrice(navPrice);

        uint256 feeAmount = _calculateMintFee(dlfAmount);
        uint256 netDlfAmount = feeAmount > 0 ? (dlfAmount - feeAmount) : dlfAmount;

        // Calculate iTRY amount: netDlfAmount * navPrice / 1e18
        iTRYAmount = netDlfAmount * navPrice / 1e18;

        if (iTRYAmount == 0) revert CommonErrors.ZeroAmount();

        // Check if output meets minimum requirement
        if (iTRYAmount < minAmountOut) {
            revert OutputBelowMinimum(iTRYAmount, minAmountOut);
        }

        // Transfer collateral into vault BEFORE minting (CEI pattern)
        _transferIntoVault(msg.sender, netDlfAmount, feeAmount);

        _mint(recipient, iTRYAmount);

        // Emit event
        emit ITRYIssued(recipient, netDlfAmount, iTRYAmount, navPrice, mintFeeInBPS);
    }

    // ============================================
    // State-Changing Functions - Redemption
    // ============================================

    /// @inheritdoc IiTryIssuer
    function redeemITRY(uint256 iTRYAmount, uint256 minAmountOut) external returns (bool fromBuffer) {
        return redeemFor(msg.sender, iTRYAmount, minAmountOut);
    }

    /// @inheritdoc IiTryIssuer
    function redeemFor(address recipient, uint256 iTRYAmount, uint256 minAmountOut)
        public
        onlyRole(_WHITELISTED_USER_ROLE)
        nonReentrant
        returns (bool fromBuffer)
    {
        // Validate recipient address
        if (recipient == address(0)) revert CommonErrors.ZeroAddress();

        // Validate iTRYAmount > 0
        if (iTRYAmount == 0) revert CommonErrors.ZeroAmount();

        if (iTRYAmount > _totalIssuedITry) {
            revert AmountExceedsITryIssuance(iTRYAmount, _totalIssuedITry);
        }

        // Get NAV price from oracle
        uint256 navPrice = oracle.price();
        if (navPrice == 0) revert InvalidNAVPrice(navPrice);

        // Calculate gross DLF amount: iTRYAmount * 1e18 / navPrice
        uint256 grossDlfAmount = iTRYAmount * 1e18 / navPrice;

        if (grossDlfAmount == 0) revert CommonErrors.ZeroAmount();

        uint256 feeAmount = _calculateRedemptionFee(grossDlfAmount);
        uint256 netDlfAmount = grossDlfAmount - feeAmount;

        // Check if output meets minimum requirement
        if (netDlfAmount < minAmountOut) {
            revert OutputBelowMinimum(netDlfAmount, minAmountOut);
        }

        _burn(msg.sender, iTRYAmount);

        // Check if buffer pool has enough DLF balance
        uint256 bufferBalance = liquidityVault.getAvailableBalance();

        if (bufferBalance >= grossDlfAmount) {
            // Buffer has enough - serve from buffer
            _redeemFromVault(recipient, netDlfAmount, feeAmount);

            fromBuffer = true;
        } else {
            // Buffer insufficient - serve from custodian
            _redeemFromCustodian(recipient, netDlfAmount, feeAmount);

            fromBuffer = false;
        }

        // Emit redemption event
        emit ITRYRedeemed(recipient, iTRYAmount, netDlfAmount, fromBuffer, redemptionFeeInBPS);
    }

 /// @inheritdoc IiTryIssuer
    function burnExcessITry(uint256 iTRYAmount)
        public
        onlyRole(DEFAULT_ADMIN_ROLE)
        nonReentrant
    {
        // Validate iTRYAmount > 0
        if (iTRYAmount == 0) revert CommonErrors.ZeroAmount();

        if (iTRYAmount > _totalIssuedITry) {
            revert AmountExceedsITryIssuance(iTRYAmount, _totalIssuedITry);
        }

        _burn(msg.sender, iTRYAmount);


        // Emit redemption event
        emit excessITryRemoved(iTRYAmount, _totalIssuedITry);
    }


    // ============================================
    // State-Changing Functions - Yield Management
    // ============================================

    /// @inheritdoc IiTryIssuer
    function processAccumulatedYield() external onlyRole(_YIELD_DISTRIBUTOR_ROLE) returns (uint256 newYield) {
        // Get current NAV price
        uint256 navPrice = oracle.price();
        if (navPrice == 0) revert InvalidNAVPrice(navPrice);

        // Calculate total collateral value: totalDLFUnderCustody * currentNAVPrice / 1e18
        uint256 currentCollateralValue = _totalDLFUnderCustody * navPrice / 1e18;

        // Calculate yield: currentCollateralValue - _totalIssuedITry
        if (currentCollateralValue <= _totalIssuedITry) {
            revert NoYieldAvailable(currentCollateralValue, _totalIssuedITry);
        }
        newYield = currentCollateralValue - _totalIssuedITry;

        // Mint yield amount to yieldReceiver contract
        _mint(address(yieldReceiver), newYield);

        // Notify yield distributor of received yield
        yieldReceiver.processNewYield(newYield);

        // Emit event
        emit YieldDistributed(newYield, address(yieldReceiver), currentCollateralValue);
    }

    // ============================================
    // Admin Functions - Fee Management
    // ============================================

    /**
     * @notice Set the redemption fee rate
     * @dev Only callable by DEFAULT_ADMIN_ROLE
     * @param newRedemptionFeeInBPS The new redemption fee in basis points (1 BPS = 0.01%)
     */
    function setRedemptionFeeInBPS(uint256 newRedemptionFeeInBPS) external onlyRole(DEFAULT_ADMIN_ROLE) {
        _validateFeeBPS(newRedemptionFeeInBPS, MAX_REDEEM_FEE_BPS);
        uint256 oldFee = redemptionFeeInBPS;
        redemptionFeeInBPS = newRedemptionFeeInBPS;
        emit RedemptionFeeUpdated(oldFee, newRedemptionFeeInBPS);
    }

    /**
     * @notice Set the mint fee rate
     * @dev Only callable by DEFAULT_ADMIN_ROLE
     * @param newMintFeeInBPS The new mint fee in basis points (1 BPS = 0.01%)
     */
    function setMintFeeInBPS(uint256 newMintFeeInBPS) external onlyRole(DEFAULT_ADMIN_ROLE) {
        _validateFeeBPS(newMintFeeInBPS, MAX_MINT_FEE_BPS);
        uint256 oldFee = mintFeeInBPS;
        mintFeeInBPS = newMintFeeInBPS;
        emit MintFeeUpdated(oldFee, newMintFeeInBPS);
    }

    // ============================================
    // Admin Functions - Integration Management
    // ============================================

    /**
     * @notice Set the address of the oracle contract
     * @dev Only callable by _INTEGRATION_MANAGER_ROLE
     * @param newOracle The address of the new oracle contract
     */
    function setOracle(address newOracle) external onlyRole(_INTEGRATION_MANAGER_ROLE) {
        _setOracle(newOracle);
    }

    /**
     * @notice Set the address of the custodian
     * @dev Only callable by _INTEGRATION_MANAGER_ROLE
     * @param newCustodian The address of the new custodian
     */
    function setCustodian(address newCustodian) external onlyRole(_INTEGRATION_MANAGER_ROLE) {
        _setCustodian(newCustodian);
    }

    /**
     * @notice Set the address of the yield receiver contract
     * @dev Only callable by _INTEGRATION_MANAGER_ROLE
     * @param newYieldReceiver The address of the new yield receiver contract
     */
    function setYieldReceiver(address newYieldReceiver) external onlyRole(_INTEGRATION_MANAGER_ROLE) {
        _setYieldReceiver(newYieldReceiver);
    }

    /**
     * @notice Internal function to set the address of the oracle contract
     * @param newOracle The address of the new oracle contract
     */
    function _setOracle(address newOracle) internal {
        if (newOracle == address(0)) revert CommonErrors.ZeroAddress();
        address oldOracle = address(oracle);
        oracle = IOracle(newOracle);
        emit OracleUpdated(oldOracle, newOracle);
    }

    /**
     * @notice Internal function to set the address of the custodian
     * @param newCustodian The address of the new custodian
     */
    function _setCustodian(address newCustodian) internal {
        if (newCustodian == address(0)) revert CommonErrors.ZeroAddress();
        address oldCustodian = custodian;
        custodian = newCustodian;
        emit CustodianUpdated(oldCustodian, newCustodian);
    }

    /**
     * @notice Internal function to set the address of the yield receiver contract
     * @param newYieldReceiver The address of the new yield receiver contract
     */
    function _setYieldReceiver(address newYieldReceiver) internal {
        if (newYieldReceiver == address(0)) revert CommonErrors.ZeroAddress();
        address oldYieldReceiver = address(yieldReceiver);
        yieldReceiver = IYieldProcessor(newYieldReceiver);
        emit YieldReceiverUpdated(oldYieldReceiver, newYieldReceiver);
    }

    /**
     * @notice Set the address of the treasury
     * @dev Only callable by _INTEGRATION_MANAGER_ROLE
     * @param newTreasury The address of the new treasury
     */
    function setTreasury(address newTreasury) external onlyRole(_INTEGRATION_MANAGER_ROLE) {
        _setTreasury(newTreasury);
    }

    /**
     * @notice Internal function to set the address of the treasury
     * @param newTreasury The address of the new treasury
     */
    function _setTreasury(address newTreasury) internal {
        if (newTreasury == address(0)) revert CommonErrors.ZeroAddress();
        address oldTreasury = treasury;
        treasury = newTreasury;
        emit TreasuryUpdated(oldTreasury, newTreasury);
    }

    /**
     * @notice Validate that fee percentage is within acceptable range
     * @dev Internal function to consolidate BPS validation logic (DRY principle)
     * @param bps The fee percentage in basis points to validate
     * @param maxBps The maximum allowed fee percentage in basis points
     */
    function _validateFeeBPS(uint256 bps, uint256 maxBps) internal pure {
        if (bps > maxBps) revert FeeTooHigh(bps, maxBps);
    }

    // ============================================
    // Admin Functions - Whitelist Management
    // ============================================

    /**
     * @notice Add an address to the whitelist, allowing them to mint and redeem iTRY
     * @dev Only callable by _WHITELIST_MANAGER_ROLE
     * @param target The address to whitelist
     */
    function addToWhitelist(address target) external onlyRole(_WHITELIST_MANAGER_ROLE) {
        _grantRole(_WHITELISTED_USER_ROLE, target);
    }

    /**
     * @notice Remove an address from the whitelist, preventing them from minting and redeeming iTRY
     * @dev Only callable by _WHITELIST_MANAGER_ROLE
     * @param target The address to remove from whitelist
     */
    function removeFromWhitelist(address target) external onlyRole(_WHITELIST_MANAGER_ROLE) {
        _revokeRole(_WHITELISTED_USER_ROLE, target);
    }

    // ============================================
    // Internal Functions - Token Operations
    // ============================================

    /**
     * @notice Internal function to mint iTRY tokens
     * @dev Updates total issued accounting and calls the iTRY token mint function
     * @param receiver The address to receive the minted tokens
     * @param amount The amount of iTRY tokens to mint
     */
    function _mint(address receiver, uint256 amount) internal {
        _totalIssuedITry += amount;
        iTryToken.mint(receiver, amount);
    }

    /**
     * @notice Internal function to burn iTRY tokens
     * @dev Updates total issued accounting and calls the iTRY token burn function
     * @param from The address whose tokens will be burned
     * @param amount The amount of iTRY tokens to burn
     */
    function _burn(address from, uint256 amount) internal {
        // Burn user's iTRY tokens
        _totalIssuedITry -= amount;
        iTryToken.burnFrom(from, amount);
    }

    // ============================================
    // Internal Functions - Collateral Management
    // ============================================

    /**
     * @notice Internal function to transfer DLF collateral from user to vault and fees to treasury
     * @dev Updates total DLF under custody and transfers tokens
     * @param from The address providing the DLF tokens
     * @param dlfAmount The net amount of DLF to transfer to the vault (after fees)
     * @param feeAmount The fee amount to transfer to treasury
     */
    function _transferIntoVault(address from, uint256 dlfAmount, uint256 feeAmount) internal {
        _totalDLFUnderCustody += dlfAmount;
        // Transfer net DLF amount to buffer pool
        if (!collateralToken.transferFrom(from, address(liquidityVault), dlfAmount)) {
            revert CommonErrors.TransferFailed();
        }

        if (feeAmount > 0) {
            // Transfer fee to treasury
            if (!collateralToken.transferFrom(from, treasury, feeAmount)) {
                revert CommonErrors.TransferFailed();
            }
            emit FeeProcessed(from, treasury, feeAmount);
        }
    }

    /**
     * @notice Internal function to process redemption from the buffer vault
     * @dev Updates total DLF under custody and instructs vault to transfer tokens
     * @param receiver The address to receive the DLF tokens
     * @param receiveAmount The net amount of DLF to transfer (after fees)
     * @param feeAmount The fee amount to transfer to treasury
     */
    function _redeemFromVault(address receiver, uint256 receiveAmount, uint256 feeAmount) internal {
        _totalDLFUnderCustody -= (receiveAmount + feeAmount);

        liquidityVault.processTransfer(receiver, receiveAmount);

        if (feeAmount > 0) {
            liquidityVault.processTransfer(treasury, feeAmount);
        }
    }

    /**
     * @notice Internal function to process redemption via custodian transfer request
     * @dev Emits events for off-chain custodian to process transfers manually
     * @param receiver The address to receive the DLF tokens
     * @param receiveAmount The net amount of DLF to transfer (after fees)
     * @param feeAmount The fee amount to transfer to treasury
     */
    function _redeemFromCustodian(address receiver, uint256 receiveAmount, uint256 feeAmount) internal {
        _totalDLFUnderCustody -= (receiveAmount + feeAmount);

        // Signal that fast access vault needs top-up from custodian
        uint256 topUpAmount = receiveAmount + feeAmount;
        emit FastAccessVaultTopUpRequested(topUpAmount);

        if (feeAmount > 0) {
            // Emit event for off-chain custodian to process
            emit CustodianTransferRequested(treasury, feeAmount);
        }

        // Emit event for off-chain custodian to process
        emit CustodianTransferRequested(receiver, receiveAmount);
    }

    // ============================================
    // Internal Functions - Fee Calculations
    // ============================================

    /**
     * @notice Calculate the mint fee for a given amount
     * @dev Fee = amount * mintFeeInBPS / 10000
     * @param amount The amount to calculate fee on
     * @return feeAmount The calculated fee amount
     */
    function _calculateMintFee(uint256 amount) internal view returns (uint256 feeAmount) {
        // Account for mint fee if configured
        if (mintFeeInBPS > 0) {
            feeAmount = amount * mintFeeInBPS / 10000;
            return feeAmount == 0 ? 1 : feeAmount; // avoid round-down to zero
        } else {
            return 0;
        }
    }

    /**
     * @notice Calculate the redemption fee for a given amount
     * @dev Fee = amount * redemptionFeeInBPS / 10000
     * @param amount The amount to calculate fee on
     * @return feeAmount The calculated fee amount
     */
    function _calculateRedemptionFee(uint256 amount) internal view returns (uint256) {
        // Account for redemption fee if configured
        if (redemptionFeeInBPS == 0) {
            return 0;
        }

        uint256 feeAmount = amount * redemptionFeeInBPS / 10000;
        return feeAmount == 0 ? 1 : feeAmount; // avoid round-down to zero
    }
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { IRouterClient } from "@chainlink/contracts-ccip/src/v0.8/ccip/interfaces/IRouterClient.sol";
import { IAny2EVMMessageReceiver } from "@chainlink/contracts-ccip/src/v0.8/ccip/interfaces/IAny2EVMMessageReceiver.sol";
import { Client } from "@chainlink/contracts-ccip/src/v0.8/ccip/libraries/Client.sol";

import { DVNAdapterBase } from "../DVNAdapterBase.sol";
import { ICCIPDVNAdapter } from "../../../interfaces/adapters/ICCIPDVNAdapter.sol";
import { ICCIPDVNAdapterFeeLib } from "../../../interfaces/adapters/ICCIPDVNAdapterFeeLib.sol";

/// @title CCIPDVNAdapter
/// @dev How CCIP DVN Adapter works:
/// 1. Estimate gas cost for the message on-chain by calling `getFee` on the Router contract.
///     refer to https://docs.chain.link/ccip/api-reference/i-router-client#getfee
/// 2. Call `ccipSend` on the Router contract to send the message.
///     refer to https://docs.chain.link/ccip/api-reference/i-router-client#ccipsend
/// @dev Recovery:
/// 1. If not enough gas paid, the message will be failed to execute on the destination chain, you can manually retry by calling `manuallyExecute` on the `OffRamp` contract.
///     refer to https://github.com/smartcontractkit/ccip/blob/ccip-develop/contracts/src/v0.8/ccip/offRamp/EVM2EVMOffRamp.sol#L222
contract CCIPDVNAdapter is DVNAdapterBase, IAny2EVMMessageReceiver, ICCIPDVNAdapter {
    address private constant NATIVE_GAS_TOKEN_ADDRESS = address(0);

    IRouterClient public immutable router;

    mapping(uint32 dstEid => DstConfig) public dstConfig;
    mapping(uint64 srcChainSelector => SrcConfig) public srcConfig;

    constructor(address[] memory _admins, address _router) DVNAdapterBase(msg.sender, _admins, 12000) {
        router = IRouterClient(_router);
    }

    // ========================= OnlyAdmin =========================
    /// @notice sets configuration for destination chains
    /// @param _params array of chain configurations
    function setDstConfig(DstConfigParam[] calldata _params) external onlyRole(ADMIN_ROLE) {
        for (uint256 i = 0; i < _params.length; i++) {
            DstConfigParam calldata param = _params[i];

            uint32 eid = param.eid % 30000;

            // set once per chainSelector
            // only one adapter per dvn that services both endpoint v1 and v2
            // we standardize the eid stored here with mod 30000
            if (dstConfig[eid].chainSelector == 0) {
                dstConfig[eid].chainSelector = param.chainSelector;
                dstConfig[eid].peer = param.peer;
                srcConfig[param.chainSelector].eid = eid;
                srcConfig[param.chainSelector].peer = param.peer;
            }

            dstConfig[eid].multiplierBps = param.multiplierBps;
            dstConfig[eid].gas = param.gas;
        }

        emit DstConfigSet(_params);
    }

    // ========================= OnlyMessageLib =========================
    function assignJob(
        AssignJobParam calldata _param,
        bytes calldata _options
    ) external payable override onlyAcl(_param.sender) returns (uint256 totalFee) {
        bytes32 receiveLib = _getAndAssertReceiveLib(msg.sender, _param.dstEid);

        ICCIPDVNAdapterFeeLib.Param memory feeLibParam = ICCIPDVNAdapterFeeLib.Param(
            _param.dstEid,
            _param.confirmations,
            _param.sender,
            defaultMultiplierBps
        );

        DstConfig memory config = dstConfig[_param.dstEid % 30000];

        bytes memory data = _encode(receiveLib, _param.packetHeader, _param.payloadHash);
        Client.EVM2AnyMessage memory message = _createCCIPMessage(data, config.peer, config.gas);

        IRouterClient ccipRouter = router;
        uint256 ccipFee;
        (ccipFee, totalFee) = ICCIPDVNAdapterFeeLib(workerFeeLib).getFeeOnSend(
            feeLibParam,
            config,
            message,
            _options,
            ccipRouter
        );

        _assertBalanceAndWithdrawFee(msg.sender, ccipFee);

        ccipRouter.ccipSend{ value: ccipFee }(config.chainSelector, message);
    }

    // ========================= OnlyRouter =========================
    function ccipReceive(Client.Any2EVMMessage calldata _message) external {
        if (msg.sender != address(router)) revert CCIPDVNAdapter_InvalidRouter(msg.sender);

        SrcConfig memory config = srcConfig[_message.sourceChainSelector];

        _assertPeer(_message.sourceChainSelector, _message.sender, config.peer);

        _decodeAndVerify(config.eid, _message.data);
    }

    // ========================= View =========================
    function getFee(
        uint32 _dstEid,
        uint64 _confirmations,
        address _sender,
        bytes calldata _options
    ) external view override onlyAcl(_sender) returns (uint256 totalFee) {
        ICCIPDVNAdapterFeeLib.Param memory feeLibParam = ICCIPDVNAdapterFeeLib.Param(
            _dstEid,
            _confirmations,
            _sender,
            defaultMultiplierBps
        );

        DstConfig memory config = dstConfig[_dstEid % 30000];

        bytes memory data = _encodeEmpty();
        Client.EVM2AnyMessage memory message = _createCCIPMessage(data, config.peer, config.gas);

        totalFee = ICCIPDVNAdapterFeeLib(workerFeeLib).getFee(feeLibParam, config, message, _options, router);
    }

    function supportsInterface(bytes4 interfaceId) public view virtual override returns (bool) {
        return interfaceId == type(IAny2EVMMessageReceiver).interfaceId || super.supportsInterface(interfaceId);
    }

    // ========================= Internal =========================
    function _createCCIPMessage(
        bytes memory _data,
        bytes memory _receiver,
        uint256 _gas
    ) private pure returns (Client.EVM2AnyMessage memory message) {
        message = Client.EVM2AnyMessage({
            receiver: _receiver,
            data: _data,
            tokenAmounts: new Client.EVMTokenAmount[](0), // Empty array indicating no tokens are being sent
            extraArgs: Client._argsToBytes(Client.EVMExtraArgsV1({ gasLimit: _gas, strict: false })),
            feeToken: NATIVE_GAS_TOKEN_ADDRESS
        });
    }

    function _assertPeer(uint64 _sourceChainSelector, bytes memory _sourceAddress, bytes memory peer) private pure {
        if (keccak256(_sourceAddress) != keccak256(peer)) {
            revert CCIPDVNAdapter_UntrustedPeer(_sourceChainSelector, _sourceAddress);
        }
    }
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { IERC165 } from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import { ERC165 } from "@openzeppelin/contracts/utils/introspection/ERC165.sol";

import { ILayerZeroEndpointV2, Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import { IMessageLib, MessageLibType } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/IMessageLib.sol";
import { PacketV1Codec } from "@layerzerolabs/lz-evm-protocol-v2/contracts/messagelib/libs/PacketV1Codec.sol";

import { MessageLibBase } from "./MessageLibBase.sol";

/// @dev receive-side message library base contract on endpoint v2.
/// it does not have the complication as the one of endpoint v1, such as nonce, executor whitelist, etc.
abstract contract ReceiveLibBaseE2 is MessageLibBase, ERC165, IMessageLib {
    using PacketV1Codec for bytes;

    constructor(address _endpoint) MessageLibBase(_endpoint, ILayerZeroEndpointV2(_endpoint).eid()) {}

    function supportsInterface(bytes4 _interfaceId) public view virtual override(ERC165, IERC165) returns (bool) {
        return _interfaceId == type(IMessageLib).interfaceId || super.supportsInterface(_interfaceId);
    }

    function messageLibType() external pure virtual override returns (MessageLibType) {
        return MessageLibType.Receive;
    }
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

/* solhint-disable private-vars-leading-underscore */

import {ERC4626} from "@openzeppelin/contracts/token/ERC20/extensions/ERC4626.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import {ERC20Permit, ERC20, IERC20} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import {SingleAdminAccessControl} from "../../utils/SingleAdminAccessControl.sol";
import {IStakediTry} from "./interfaces/IStakediTry.sol";

/**
 * @title StakediTry
 * @notice The StakediTry contract allows users to stake iTry tokens and earn a portion of protocol LST and perpetual yield that is allocated
 * to stakers by the Ethena DAO governance voted yield distribution algorithm.  The algorithm seeks to balance the stability of the protocol by funding
 * the protocol's insurance fund, DAO activities, and rewarding stakers with a portion of the protocol's yield.
 */
contract StakediTry is SingleAdminAccessControl, ReentrancyGuard, ERC20Permit, ERC4626, IStakediTry {
    using SafeERC20 for IERC20;

    /* ------------- CONSTANTS ------------- */
    /// @notice The role that is allowed to distribute rewards to this contract
    bytes32 private constant REWARDER_ROLE = keccak256("REWARDER_ROLE");
    /// @notice The role that is allowed to blacklist and un-blacklist addresses
    bytes32 private constant BLACKLIST_MANAGER_ROLE = keccak256("BLACKLIST_MANAGER_ROLE");
    /// @notice The role which prevents an address to stake
    bytes32 private constant SOFT_RESTRICTED_STAKER_ROLE = keccak256("SOFT_RESTRICTED_STAKER_ROLE");
    /// @notice The role which prevents an address to transfer, stake, or unstake. The owner of the contract can redirect address staking balance if an address is in full restricting mode.
    bytes32 private constant FULL_RESTRICTED_STAKER_ROLE = keccak256("FULL_RESTRICTED_STAKER_ROLE");
    /// @notice Minimum non-zero shares amount to prevent donation attack
    uint256 private constant MIN_SHARES = 1 ether;
    /// @notice Minimum allowed vesting period (1 hour)
    uint256 private constant MIN_VESTING_PERIOD = 1 hours;
    /// @notice Maximum allowed vesting period (30 days)
    uint256 private constant MAX_VESTING_PERIOD = 30 days;

    /* ------------- STATE VARIABLES ------------- */

    /// @notice The amount of the last asset distribution from the controller contract into this
    /// contract + any unvested remainder at that time
    uint256 public vestingAmount;

    /// @notice The timestamp of the last asset distribution from the controller contract into this contract
    uint256 public lastDistributionTimestamp;

    /// @notice The vesting period of lastDistributionAmount over which it increasingly becomes available to stakers
    uint256 private vestingPeriod;

    /* ------------- MODIFIERS ------------- */

    /// @notice ensure input amount nonzero
    modifier notZero(uint256 amount) {
        if (amount == 0) revert InvalidAmount();
        _;
    }

    /// @notice ensures blacklist target is not owner
    modifier notOwner(address target) {
        if (target == owner()) revert CantBlacklistOwner();
        _;
    }

    /* ------------- CONSTRUCTOR ------------- */

    /**
     * @notice Constructor for StakediTry contract.
     * @param _asset The address of the iTry token.
     * @param _initialRewarder The address of the initial rewarder.
     * @param _owner The address of the admin role.
     *
     */
    constructor(IERC20 _asset, address _initialRewarder, address _owner)
        ERC20("wiTRY", "wiTRY")
        ERC4626(_asset)
        ERC20Permit("wiTRY")
    {
        if (_owner == address(0) || _initialRewarder == address(0) || address(_asset) == address(0)) {
            revert InvalidZeroAddress();
        }

        vestingPeriod = MIN_VESTING_PERIOD;

        _grantRole(REWARDER_ROLE, _initialRewarder);
        _grantRole(DEFAULT_ADMIN_ROLE, _owner);
    }

    /* ------------- EXTERNAL ------------- */

    /**
     * @notice Allows the owner to update the vesting period.
     * @dev Can only be called when there are no unvested rewards to avoid disrupting active vesting.
     * @param _vestingPeriod The new vesting period (must be between MIN_VESTING_PERIOD and MAX_VESTING_PERIOD).
     */
    function setVestingPeriod(uint256 _vestingPeriod) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (_vestingPeriod < MIN_VESTING_PERIOD || _vestingPeriod > MAX_VESTING_PERIOD) {
            revert InvalidVestingPeriod();
        }
        if (getUnvestedAmount() > 0) {
            revert StillVesting();
        }

        uint256 oldVestingPeriod = vestingPeriod;
        vestingPeriod = _vestingPeriod;

        emit VestingPeriodUpdated(oldVestingPeriod, _vestingPeriod);
    }

    /**
     * @notice Allows the owner to transfer rewards from the controller contract into this contract.
     * @param amount The amount of rewards to transfer.
     */
    function transferInRewards(uint256 amount) external nonReentrant onlyRole(REWARDER_ROLE) notZero(amount) {
        _updateVestingAmount(amount);
        // transfer assets from rewarder to this contract
        IERC20(asset()).safeTransferFrom(msg.sender, address(this), amount);

        emit RewardsReceived(amount);
    }

    /**
     * @notice Allows the owner (DEFAULT_ADMIN_ROLE) and blacklist managers to blacklist addresses.
     * @param target The address to blacklist.
     * @param isFullBlacklisting Soft or full blacklisting level.
     */
    function addToBlacklist(address target, bool isFullBlacklisting)
        external
        onlyRole(BLACKLIST_MANAGER_ROLE)
        notOwner(target)
    {
        bytes32 role = isFullBlacklisting ? FULL_RESTRICTED_STAKER_ROLE : SOFT_RESTRICTED_STAKER_ROLE;
        _grantRole(role, target);
    }

    /**
     * @notice Allows the owner (DEFAULT_ADMIN_ROLE) and blacklist managers to un-blacklist addresses.
     * @param target The address to un-blacklist.
     * @param isFullBlacklisting Soft or full blacklisting level.
     */
    function removeFromBlacklist(address target, bool isFullBlacklisting) external onlyRole(BLACKLIST_MANAGER_ROLE) {
        bytes32 role = isFullBlacklisting ? FULL_RESTRICTED_STAKER_ROLE : SOFT_RESTRICTED_STAKER_ROLE;
        _revokeRole(role, target);
    }

    /**
     * @notice Allows the owner to rescue tokens accidentally sent to the contract.
     * Note that the owner cannot rescue iTry tokens because they functionally sit here
     * and belong to stakers but can rescue staked iTry as they should never actually
     * sit in this contract and a staker may well transfer them here by accident.
     * @param token The token to be rescued.
     * @param amount The amount of tokens to be rescued.
     * @param to Where to send rescued tokens
     */
    function rescueTokens(address token, uint256 amount, address to)
        external
        nonReentrant
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        if (address(token) == asset()) revert InvalidToken();
        IERC20(token).safeTransfer(to, amount);
    }

    /**
     * @dev Burns the full restricted user amount and mints to the desired owner address.
     * @param from The address to burn the entire balance, with the FULL_RESTRICTED_STAKER_ROLE
     * @param to The address to mint the entire balance of "from" parameter.
     */
    function redistributeLockedAmount(address from, address to) external nonReentrant onlyRole(DEFAULT_ADMIN_ROLE) {
        if (hasRole(FULL_RESTRICTED_STAKER_ROLE, from) && !hasRole(FULL_RESTRICTED_STAKER_ROLE, to)) {
            uint256 amountToDistribute = balanceOf(from);
            uint256 iTryToVest = previewRedeem(amountToDistribute);
            _burn(from, amountToDistribute);
            _checkMinShares();
            // to address of address(0) enables burning
            if (to == address(0)) {
                _updateVestingAmount(iTryToVest);
            } else {
                _mint(to, amountToDistribute);
            }

            emit LockedAmountRedistributed(from, to, amountToDistribute);
        } else {
            revert OperationNotAllowed();
        }
    }

    /* ------------- PUBLIC ------------- */

    /**
     * @notice Returns the amount of iTry tokens that are vested in the contract.
     */
    function totalAssets() public view override returns (uint256) {
        return IERC20(asset()).balanceOf(address(this)) - getUnvestedAmount();
    }

    /**
     * @notice Returns the amount of iTry tokens that are unvested in the contract.
     */
    function getUnvestedAmount() public view returns (uint256) {
        uint256 timeSinceLastDistribution = block.timestamp - lastDistributionTimestamp;

        if (timeSinceLastDistribution >= vestingPeriod) {
            return 0;
        }

        uint256 deltaT;
        unchecked {
            deltaT = (vestingPeriod - timeSinceLastDistribution);
        }
        return (deltaT * vestingAmount) / vestingPeriod;
    }

    /// @dev Necessary because both ERC20 (from ERC20Permit) and ERC4626 declare decimals()
    function decimals() public pure override(ERC4626, ERC20) returns (uint8) {
        return 18;
    }

    /**
     * @notice Returns the current vesting period.
     */
    function getVestingPeriod() public view returns (uint256) {
        return vestingPeriod;
    }

    /* ------------- INTERNAL ------------- */

    /// @notice ensures a small non-zero amount of shares does not remain, exposing to donation attack
    function _checkMinShares() internal view {
        uint256 _totalSupply = totalSupply();
        if (_totalSupply > 0 && _totalSupply < MIN_SHARES) revert MinSharesViolation();
    }

    /**
     * @dev Deposit/mint common workflow.
     * @param caller sender of assets
     * @param receiver where to send shares
     * @param assets assets to deposit
     * @param shares shares to mint
     */
    function _deposit(address caller, address receiver, uint256 assets, uint256 shares)
        internal
        override
        nonReentrant
        notZero(assets)
        notZero(shares)
    {
        if (hasRole(SOFT_RESTRICTED_STAKER_ROLE, caller) || hasRole(SOFT_RESTRICTED_STAKER_ROLE, receiver)) {
            revert OperationNotAllowed();
        }
        super._deposit(caller, receiver, assets, shares);
        _checkMinShares();
    }

    /**
     * @dev Withdraw/redeem common workflow.
     * @param caller tx sender
     * @param receiver where to send assets
     * @param _owner where to burn shares from
     * @param assets asset amount to transfer out
     * @param shares shares to burn
     */
    function _withdraw(address caller, address receiver, address _owner, uint256 assets, uint256 shares)
        internal
        override
        nonReentrant
        notZero(assets)
        notZero(shares)
    {
        if (
            hasRole(FULL_RESTRICTED_STAKER_ROLE, caller) || hasRole(FULL_RESTRICTED_STAKER_ROLE, receiver)
                || hasRole(FULL_RESTRICTED_STAKER_ROLE, _owner)
        ) {
            revert OperationNotAllowed();
        }

        super._withdraw(caller, receiver, _owner, assets, shares);
        _checkMinShares();
    }

    function _updateVestingAmount(uint256 newVestingAmount) internal {
        if (getUnvestedAmount() > 0) revert StillVesting();

        vestingAmount = newVestingAmount;
        lastDistributionTimestamp = block.timestamp;
    }

    /**
     * @dev Hook that is called before any transfer of tokens. This includes
     * minting and burning. Disables transfers from or to of addresses with the FULL_RESTRICTED_STAKER_ROLE role.
     */

    function _beforeTokenTransfer(address from, address to, uint256) internal virtual override {
        if (hasRole(FULL_RESTRICTED_STAKER_ROLE, from) && to != address(0)) {
            revert OperationNotAllowed();
        }
        if (hasRole(FULL_RESTRICTED_STAKER_ROLE, to)) {
            revert OperationNotAllowed();
        }
    }

    /**
     * @dev Remove renounce role access from AccessControl, to prevent users to resign roles.
     */
    function renounceRole(bytes32, address) public virtual override {
        revert OperationNotAllowed();
    }
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

/* solhint-disable var-name-mixedcase */

import {StakediTry, SafeERC20, IERC20} from "./StakediTry.sol";
import {IStakediTryCooldown, UserCooldown} from "./interfaces/IStakediTryCooldown.sol";
import {iTrySilo} from "./iTrySilo.sol";

/**
 * @title StakediTryV2
 * @notice The StakediTryV2 contract allows users to stake iTry tokens and earn a portion of protocol LST and perpetual yield that is allocated
 * to stakers by the Ethena DAO governance voted yield distribution algorithm.  The algorithm seeks to balance the stability of the protocol by funding
 * the protocol's insurance fund, DAO activities, and rewarding stakers with a portion of the protocol's yield.
 * @dev If cooldown duration is set to zero, the StakediTryV2 behavior changes to follow ERC4626 standard and disables cooldownShares and cooldownAssets methods. If cooldown duration is greater than zero, the ERC4626 withdrawal and redeem functions are disabled, breaking the ERC4626 standard, and enabling the cooldownShares and the cooldownAssets functions.
 */
contract StakediTryV2 is IStakediTryCooldown, StakediTry {
    using SafeERC20 for IERC20;

    mapping(address => UserCooldown) public cooldowns;

    iTrySilo public immutable silo;

    uint24 public constant MAX_COOLDOWN_DURATION = 90 days;

    uint24 public cooldownDuration;

    /// @notice ensure cooldownDuration is zero
    modifier ensureCooldownOff() {
        if (cooldownDuration != 0) revert OperationNotAllowed();
        _;
    }

    /// @notice ensure cooldownDuration is gt 0
    modifier ensureCooldownOn() {
        if (cooldownDuration == 0) revert OperationNotAllowed();
        _;
    }

    /// @notice Constructor for StakediTryV2 contract.
    /// @param _asset The address of the iTry token.
    /// @param initialRewarder The address of the initial rewarder.
    /// @param _owner The address of the admin role.
    constructor(IERC20 _asset, address initialRewarder, address _owner) StakediTry(_asset, initialRewarder, _owner) {
        silo = new iTrySilo(address(this), address(_asset));
        cooldownDuration = MAX_COOLDOWN_DURATION;
    }

    /* ------------- EXTERNAL ------------- */

    /**
     * @dev See {IERC4626-withdraw}.
     */
    function withdraw(uint256 assets, address receiver, address _owner)
        public
        virtual
        override
        ensureCooldownOff
        returns (uint256)
    {
        return super.withdraw(assets, receiver, _owner);
    }

    /**
     * @dev See {IERC4626-redeem}.
     */
    function redeem(uint256 shares, address receiver, address _owner)
        public
        virtual
        override
        ensureCooldownOff
        returns (uint256)
    {
        return super.redeem(shares, receiver, _owner);
    }

    /// @notice Claim the staking amount after the cooldown has finished. The address can only retire the full amount of assets.
    /// @dev unstake can be called after cooldown have been set to 0, to let accounts to be able to claim remaining assets locked at Silo
    /// @param receiver Address to send the assets by the staker
    function unstake(address receiver) external {
        UserCooldown storage userCooldown = cooldowns[msg.sender];
        uint256 assets = userCooldown.underlyingAmount;

        if (block.timestamp >= userCooldown.cooldownEnd || cooldownDuration == 0) {
            userCooldown.cooldownEnd = 0;
            userCooldown.underlyingAmount = 0;

            silo.withdraw(receiver, assets);
        } else {
            revert InvalidCooldown();
        }
    }

    /// @notice redeem assets and starts a cooldown to claim the converted underlying asset
    /// @param assets assets to redeem
    function cooldownAssets(uint256 assets) external ensureCooldownOn returns (uint256 shares) {
        if (assets > maxWithdraw(msg.sender)) revert ExcessiveWithdrawAmount();

        shares = previewWithdraw(assets);

        cooldowns[msg.sender].cooldownEnd = uint104(block.timestamp) + cooldownDuration;
        cooldowns[msg.sender].underlyingAmount += uint152(assets);

        _withdraw(msg.sender, address(silo), msg.sender, assets, shares);
    }

    /// @notice redeem shares into assets and starts a cooldown to claim the converted underlying asset
    /// @param shares shares to redeem
    function cooldownShares(uint256 shares) external ensureCooldownOn returns (uint256 assets) {
        if (shares > maxRedeem(msg.sender)) revert ExcessiveRedeemAmount();

        assets = previewRedeem(shares);

        cooldowns[msg.sender].cooldownEnd = uint104(block.timestamp) + cooldownDuration;
        cooldowns[msg.sender].underlyingAmount += uint152(assets);

        _withdraw(msg.sender, address(silo), msg.sender, assets, shares);
    }

    /// @notice Set cooldown duration. If cooldown duration is set to zero, the StakediTryV2 behavior changes to follow ERC4626 standard and disables cooldownShares and cooldownAssets methods. If cooldown duration is greater than zero, the ERC4626 withdrawal and redeem functions are disabled, breaking the ERC4626 standard, and enabling the cooldownShares and the cooldownAssets functions.
    /// @param duration Duration of the cooldown
    function setCooldownDuration(uint24 duration) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (duration > MAX_COOLDOWN_DURATION) {
            revert InvalidCooldown();
        }

        uint24 previousDuration = cooldownDuration;
        cooldownDuration = duration;
        emit CooldownDurationUpdated(previousDuration, cooldownDuration);
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { IOFT, OFTCore } from "./OFTCore.sol";

/**
 * @title OFT Contract
 * @dev OFT is an ERC-20 token that extends the functionality of the OFTCore contract.
 */
abstract contract OFT is OFTCore, ERC20 {
    /**
     * @dev Constructor for the OFT contract.
     * @param _name The name of the OFT.
     * @param _symbol The symbol of the OFT.
     * @param _lzEndpoint The LayerZero endpoint address.
     * @param _delegate The delegate capable of making OApp configurations inside of the endpoint.
     */
    constructor(
        string memory _name,
        string memory _symbol,
        address _lzEndpoint,
        address _delegate
    ) ERC20(_name, _symbol) OFTCore(decimals(), _lzEndpoint, _delegate) {}

    /**
     * @dev Retrieves the address of the underlying ERC20 implementation.
     * @return The address of the OFT token.
     *
     * @dev In the case of OFT, address(this) and erc20 are the same contract.
     */
    function token() public view returns (address) {
        return address(this);
    }

    /**
     * @notice Indicates whether the OFT contract requires approval of the 'token()' to send.
     * @return requiresApproval Needs approval of the underlying token implementation.
     *
     * @dev In the case of OFT where the contract IS the token, approval is NOT required.
     */
    function approvalRequired() external pure virtual returns (bool) {
        return false;
    }

    /**
     * @dev Burns tokens from the sender's specified balance.
     * @param _from The address to debit the tokens from.
     * @param _amountLD The amount of tokens to send in local decimals.
     * @param _minAmountLD The minimum amount to send in local decimals.
     * @param _dstEid The destination chain ID.
     * @return amountSentLD The amount sent in local decimals.
     * @return amountReceivedLD The amount received in local decimals on the remote.
     */
    function _debit(
        address _from,
        uint256 _amountLD,
        uint256 _minAmountLD,
        uint32 _dstEid
    ) internal virtual override returns (uint256 amountSentLD, uint256 amountReceivedLD) {
        (amountSentLD, amountReceivedLD) = _debitView(_amountLD, _minAmountLD, _dstEid);

        // @dev In NON-default OFT, amountSentLD could be 100, with a 10% fee, the amountReceivedLD amount is 90,
        // therefore amountSentLD CAN differ from amountReceivedLD.

        // @dev Default OFT burns on src.
        _burn(_from, amountSentLD);
    }

    /**
     * @dev Credits tokens to the specified address.
     * @param _to The address to credit the tokens to.
     * @param _amountLD The amount of tokens to credit in local decimals.
     * @dev _srcEid The source chain ID.
     * @return amountReceivedLD The amount of tokens ACTUALLY received in local decimals.
     */
    function _credit(
        address _to,
        uint256 _amountLD,
        uint32 /*_srcEid*/
    ) internal virtual override returns (uint256 amountReceivedLD) {
        if (_to == address(0x0)) _to = address(0xdead); // _mint(...) does not support address(0x0)
        // @dev Default OFT mints on dst.
        _mint(_to, _amountLD);
        // @dev In the case of NON-default OFT, the _amountLD MIGHT not be == amountReceivedLD.
        return _amountLD;
    }
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {IERC5313} from "@openzeppelin/contracts/interfaces/IERC5313.sol";
import {ISingleAdminAccessControl} from "./ISingleAdminAccessControl.sol";

/**
 * @title SingleAdminAccessControl
 * @notice SingleAdminAccessControl is a contract that provides a single admin role
 * @notice This contract is a simplified alternative to OpenZeppelin's AccessControlDefaultAdminRules
 */
abstract contract SingleAdminAccessControl is IERC5313, ISingleAdminAccessControl, AccessControl {
    address private _currentDefaultAdmin;
    address private _pendingDefaultAdmin;

    modifier notAdmin(bytes32 role) {
        if (role == DEFAULT_ADMIN_ROLE) revert InvalidAdminChange();
        _;
    }

    /// @notice Transfer the admin role to a new address
    /// @notice This can ONLY be executed by the current admin
    /// @param newAdmin address
    function transferAdmin(address newAdmin) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (newAdmin == msg.sender) revert InvalidAdminChange();
        _pendingDefaultAdmin = newAdmin;
        emit AdminTransferRequested(_currentDefaultAdmin, newAdmin);
    }

    function acceptAdmin() external {
        if (msg.sender != _pendingDefaultAdmin) revert NotPendingAdmin();
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    /// @notice grant a role
    /// @notice can only be executed by the current single admin
    /// @notice admin role cannot be granted externally
    /// @param role bytes32
    /// @param account address
    function grantRole(bytes32 role, address account) public override onlyRole(DEFAULT_ADMIN_ROLE) notAdmin(role) {
        _grantRole(role, account);
    }

    /// @notice revoke a role
    /// @notice can only be executed by the current admin
    /// @notice admin role cannot be revoked
    /// @param role bytes32
    /// @param account address
    function revokeRole(bytes32 role, address account) public override onlyRole(DEFAULT_ADMIN_ROLE) notAdmin(role) {
        _revokeRole(role, account);
    }

    /// @notice renounce the role of msg.sender
    /// @notice admin role cannot be renounced
    /// @param role bytes32
    /// @param account address
    function renounceRole(bytes32 role, address account) public virtual override notAdmin(role) {
        super.renounceRole(role, account);
    }

    /**
     * @dev See {IERC5313-owner}.
     */
    function owner() public view virtual returns (address) {
        return _currentDefaultAdmin;
    }

    /**
     * @notice no way to change admin without removing old admin first
     */
    function _grantRole(bytes32 role, address account) internal override {
        if (role == DEFAULT_ADMIN_ROLE) {
            emit AdminTransferred(_currentDefaultAdmin, account);
            _revokeRole(DEFAULT_ADMIN_ROLE, _currentDefaultAdmin);
            _currentDefaultAdmin = account;
            delete _pendingDefaultAdmin;
        }
        super._grantRole(role, account);
    }
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { ERC165, IERC165 } from "@openzeppelin/contracts/utils/introspection/ERC165.sol";

import { ILayerZeroEndpointV2, MessagingFee, Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import { IMessageLib, MessageLibType, SetConfigParam } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/IMessageLib.sol";
import { ISendLib, Packet } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ISendLib.sol";
import { PacketV1Codec } from "@layerzerolabs/lz-evm-protocol-v2/contracts/messagelib/libs/PacketV1Codec.sol";
import { Transfer } from "@layerzerolabs/lz-evm-protocol-v2/contracts/libs/Transfer.sol";
import { AddressCast } from "@layerzerolabs/lz-evm-protocol-v2/contracts/libs/AddressCast.sol";

import { ILayerZeroReadExecutor } from "../../interfaces/ILayerZeroReadExecutor.sol";
import { ILayerZeroReadDVN } from "../interfaces/ILayerZeroReadDVN.sol";
import { ILayerZeroTreasury } from "../../interfaces/ILayerZeroTreasury.sol";

import { UlnOptions } from "../libs/UlnOptions.sol";
import { DVNOptions } from "../libs/DVNOptions.sol";
import { SafeCall } from "../../libs/SafeCall.sol";

import { MessageLibBase } from "../../MessageLibBase.sol";
import { ReadLibBase, ReadLibConfig } from "./ReadLibBase.sol";

contract ReadLib1002 is ISendLib, ERC165, ReadLibBase, MessageLibBase {
    using PacketV1Codec for bytes;
    using SafeCall for address;

    uint32 internal constant CONFIG_TYPE_READ_LID_CONFIG = 1;

    uint16 internal constant TREASURY_MAX_COPY = 32;

    uint256 internal immutable treasuryGasLimit;

    mapping(address oapp => mapping(uint32 eid => mapping(uint64 nonce => bytes32 cmdHash))) public cmdHashLookup;
    mapping(bytes32 headerHash => mapping(bytes32 cmdHash => mapping(address dvn => bytes32 payloadHash)))
        public hashLookup;

    // accumulated fees for workers and treasury
    mapping(address worker => uint256 fee) public fees;
    uint256 internal treasuryNativeFeeCap;
    address internal treasury;

    event PayloadVerified(address dvn, bytes header, bytes32 cmdHash, bytes32 payloadHash);
    event ExecutorFeePaid(address executor, uint256 fee);
    event DVNFeePaid(address[] requiredDVNs, address[] optionalDVNs, uint256[] fees);
    event NativeFeeWithdrawn(address worker, address receiver, uint256 amount);
    event LzTokenFeeWithdrawn(address lzToken, address receiver, uint256 amount);
    event TreasurySet(address treasury);
    event TreasuryNativeFeeCapSet(uint256 newTreasuryNativeFeeCap);

    error LZ_RL_InvalidReceiver();
    error LZ_RL_InvalidPacketHeader();
    error LZ_RL_InvalidCmdHash();
    error LZ_RL_InvalidPacketVersion();
    error LZ_RL_InvalidEid();
    error LZ_RL_Verifying();
    error LZ_RL_InvalidConfigType(uint32 configType);
    error LZ_RL_InvalidAmount(uint256 requested, uint256 available);
    error LZ_RL_NotTreasury();
    error LZ_RL_CannotWithdrawAltToken();

    constructor(
        address _endpoint,
        uint256 _treasuryGasLimit,
        uint256 _treasuryGasForFeeCap
    ) MessageLibBase(_endpoint, ILayerZeroEndpointV2(_endpoint).eid()) {
        treasuryGasLimit = _treasuryGasLimit;
        treasuryNativeFeeCap = _treasuryGasForFeeCap;
    }

    function supportsInterface(bytes4 _interfaceId) public view override(ERC165, IERC165) returns (bool) {
        return
            _interfaceId == type(IMessageLib).interfaceId ||
            _interfaceId == type(ISendLib).interfaceId ||
            super.supportsInterface(_interfaceId);
    }

    // ============================ OnlyOwner ===================================

    function setTreasury(address _treasury) external onlyOwner {
        treasury = _treasury;
        emit TreasurySet(_treasury);
    }

    /// @dev the new value can not be greater than the old value, i.e. down only
    function setTreasuryNativeFeeCap(uint256 _newTreasuryNativeFeeCap) external onlyOwner {
        // assert the new value is no greater than the old value
        if (_newTreasuryNativeFeeCap > treasuryNativeFeeCap)
            revert LZ_RL_InvalidAmount(_newTreasuryNativeFeeCap, treasuryNativeFeeCap);
        treasuryNativeFeeCap = _newTreasuryNativeFeeCap;
        emit TreasuryNativeFeeCapSet(_newTreasuryNativeFeeCap);
    }

    // ============================ OnlyEndpoint ===================================

    function send(
        Packet calldata _packet,
        bytes calldata _options,
        bool _payInLzToken
    ) external onlyEndpoint returns (MessagingFee memory, bytes memory) {
        // the receiver must be the same as the sender
        if (AddressCast.toBytes32(_packet.sender) != _packet.receiver) revert LZ_RL_InvalidReceiver();

        // pay worker and treasury
        (bytes memory encodedPacket, uint256 totalNativeFee) = _payWorkers(_packet, _options);
        (uint256 treasuryNativeFee, uint256 lzTokenFee) = _payTreasury(
            _packet.sender,
            _packet.dstEid,
            totalNativeFee,
            _payInLzToken
        );
        totalNativeFee += treasuryNativeFee;

        // store the cmdHash for verification in order to prevent reorg attack
        cmdHashLookup[_packet.sender][_packet.dstEid][_packet.nonce] = keccak256(_packet.message);

        return (MessagingFee(totalNativeFee, lzTokenFee), encodedPacket);
    }

    function setConfig(address _oapp, SetConfigParam[] calldata _params) external onlyEndpoint {
        for (uint256 i = 0; i < _params.length; i++) {
            SetConfigParam calldata param = _params[i];
            _assertSupportedEid(param.eid);
            if (param.configType == CONFIG_TYPE_READ_LID_CONFIG) {
                _setReadLibConfig(param.eid, _oapp, abi.decode(param.config, (ReadLibConfig)));
            } else {
                revert LZ_RL_InvalidConfigType(param.configType);
            }
        }
    }

    // ============================ External ===================================
    /// @dev The verification will be done in the same chain where the packet is sent.
    /// @dev dont need to check endpoint verifiable here to save gas, as it will reverts if not verifiable.
    /// @param _packetHeader - the srcEid should be the localEid and the dstEid should be the channel id.
    ///        The original packet header in PacketSent event should be processed to flip the srcEid and dstEid.
    function commitVerification(bytes calldata _packetHeader, bytes32 _cmdHash, bytes32 _payloadHash) external {
        // assert packet header is of right size 81
        if (_packetHeader.length != 81) revert LZ_RL_InvalidPacketHeader();
        // assert packet header version is the same
        if (_packetHeader.version() != PacketV1Codec.PACKET_VERSION) revert LZ_RL_InvalidPacketVersion();
        // assert the packet is for this endpoint
        if (_packetHeader.dstEid() != localEid) revert LZ_RL_InvalidEid();

        // cache these values to save gas
        address receiver = _packetHeader.receiverB20();
        uint32 srcEid = _packetHeader.srcEid(); // channel id
        uint64 nonce = _packetHeader.nonce();

        // reorg protection. to allow reverification, the cmdHash cant be removed
        if (cmdHashLookup[receiver][srcEid][nonce] != _cmdHash) revert LZ_RL_InvalidCmdHash();

        ReadLibConfig memory config = getReadLibConfig(receiver, srcEid);
        _verifyAndReclaimStorage(config, keccak256(_packetHeader), _cmdHash, _payloadHash);

        // endpoint will revert if nonce <= lazyInboundNonce
        Origin memory origin = Origin(srcEid, _packetHeader.sender(), nonce);
        ILayerZeroEndpointV2(endpoint).verify(origin, receiver, _payloadHash);
    }

    /// @dev DVN verifies the payload with the packet header and command hash
    /// @param _packetHeader - the packet header is needed for event only, which can be conveniently for off-chain to track the packet state.
    function verify(bytes calldata _packetHeader, bytes32 _cmdHash, bytes32 _payloadHash) external {
        hashLookup[keccak256(_packetHeader)][_cmdHash][msg.sender] = _payloadHash;
        emit PayloadVerified(msg.sender, _packetHeader, _cmdHash, _payloadHash);
    }

    function withdrawFee(address _to, uint256 _amount) external {
        uint256 fee = fees[msg.sender];
        if (_amount > fee) revert LZ_RL_InvalidAmount(_amount, fee);
        unchecked {
            fees[msg.sender] = fee - _amount;
        }

        // transfers native if nativeToken == address(0x0)
        address nativeToken = ILayerZeroEndpointV2(endpoint).nativeToken();
        Transfer.nativeOrToken(nativeToken, _to, _amount);
        emit NativeFeeWithdrawn(msg.sender, _to, _amount);
    }

    // ============================ Treasury ===================================

    /// @dev _lzToken is a user-supplied value because lzToken might change in the endpoint before all lzToken can be taken out
    function withdrawLzTokenFee(address _lzToken, address _to, uint256 _amount) external {
        if (msg.sender != treasury) revert LZ_RL_NotTreasury();

        // lz token cannot be the same as the native token
        if (ILayerZeroEndpointV2(endpoint).nativeToken() == _lzToken) revert LZ_RL_CannotWithdrawAltToken();

        Transfer.token(_lzToken, _to, _amount);

        emit LzTokenFeeWithdrawn(_lzToken, _to, _amount);
    }

    // ============================ View ===================================

    function quote(
        Packet calldata _packet,
        bytes calldata _options,
        bool _payInLzToken
    ) external view returns (MessagingFee memory) {
        // split workers options
        (bytes memory executorOptions, bytes memory dvnOptions) = UlnOptions.decode(_options);

        address sender = _packet.sender;
        uint32 dstEid = _packet.dstEid;

        // quote the executor and dvns
        ReadLibConfig memory config = getReadLibConfig(sender, dstEid);
        uint256 nativeFee = _quoteDVNs(
            config,
            sender,
            PacketV1Codec.encodePacketHeader(_packet),
            _packet.message,
            dvnOptions
        );
        nativeFee += ILayerZeroReadExecutor(config.executor).getFee(sender, executorOptions);

        // quote treasury
        (uint256 treasuryNativeFee, uint256 lzTokenFee) = _quoteTreasury(sender, dstEid, nativeFee, _payInLzToken);
        nativeFee += treasuryNativeFee;

        return MessagingFee(nativeFee, lzTokenFee);
    }

    function verifiable(
        ReadLibConfig calldata _config,
        bytes32 _headerHash,
        bytes32 _cmdHash,
        bytes32 _payloadHash
    ) external view returns (bool) {
        return _checkVerifiable(_config, _headerHash, _cmdHash, _payloadHash);
    }

    function getConfig(uint32 _eid, address _oapp, uint32 _configType) external view returns (bytes memory) {
        if (_configType == CONFIG_TYPE_READ_LID_CONFIG) {
            return abi.encode(getReadLibConfig(_oapp, _eid));
        } else {
            revert LZ_RL_InvalidConfigType(_configType);
        }
    }

    function getTreasuryAndNativeFeeCap() external view returns (address, uint256) {
        return (treasury, treasuryNativeFeeCap);
    }

    function isSupportedEid(uint32 _eid) external view returns (bool) {
        return _isSupportedEid(_eid);
    }

    function messageLibType() external pure returns (MessageLibType) {
        return MessageLibType.SendAndReceive;
    }

    function version() external pure returns (uint64 major, uint8 minor, uint8 endpointVersion) {
        return (10, 0, 2);
    }

    // ============================ Internal ===================================

    /// 1/ handle executor
    /// 2/ handle other workers
    function _payWorkers(
        Packet calldata _packet,
        bytes calldata _options
    ) internal returns (bytes memory encodedPacket, uint256 totalNativeFee) {
        // split workers options
        (bytes memory executorOptions, bytes memory dvnOptions) = UlnOptions.decode(_options);

        // handle executor
        ReadLibConfig memory config = getReadLibConfig(_packet.sender, _packet.dstEid);
        totalNativeFee = _payExecutor(config.executor, _packet.sender, executorOptions);

        // handle dvns
        (uint256 dvnFee, bytes memory packetBytes) = _payDVNs(config, _packet, dvnOptions);
        totalNativeFee += dvnFee;

        encodedPacket = packetBytes;
    }

    function _payDVNs(
        ReadLibConfig memory _config,
        Packet calldata _packet,
        bytes memory _options
    ) internal returns (uint256 totalFee, bytes memory encodedPacket) {
        bytes memory packetHeader = PacketV1Codec.encodePacketHeader(_packet);
        bytes memory payload = PacketV1Codec.encodePayload(_packet);

        uint256[] memory dvnFees;
        (totalFee, dvnFees) = _assignDVNJobs(_config, _packet.sender, packetHeader, _packet.message, _options);

        encodedPacket = abi.encodePacked(packetHeader, payload);
        emit DVNFeePaid(_config.requiredDVNs, _config.optionalDVNs, dvnFees);
    }

    function _assignDVNJobs(
        ReadLibConfig memory _config,
        address _sender,
        bytes memory _packetHeader,
        bytes calldata _cmd,
        bytes memory _options
    ) internal returns (uint256 totalFee, uint256[] memory dvnFees) {
        (bytes[] memory optionsArray, uint8[] memory dvnIds) = DVNOptions.groupDVNOptionsByIdx(_options);

        uint8 dvnsLength = _config.requiredDVNCount + _config.optionalDVNCount;
        dvnFees = new uint256[](dvnsLength);
        for (uint8 i = 0; i < dvnsLength; ++i) {
            address dvn = i < _config.requiredDVNCount
                ? _config.requiredDVNs[i]
                : _config.optionalDVNs[i - _config.requiredDVNCount];

            bytes memory options = "";
            for (uint256 j = 0; j < dvnIds.length; ++j) {
                if (dvnIds[j] == i) {
                    options = optionsArray[j];
                    break;
                }
            }

            dvnFees[i] = ILayerZeroReadDVN(dvn).assignJob(_sender, _packetHeader, _cmd, options);
            if (dvnFees[i] > 0) {
                fees[dvn] += dvnFees[i];
                totalFee += dvnFees[i];
            }
        }
    }

    function _quoteDVNs(
        ReadLibConfig memory _config,
        address _sender,
        bytes memory _packetHeader,
        bytes calldata _cmd,
        bytes memory _options
    ) internal view returns (uint256 totalFee) {
        (bytes[] memory optionsArray, uint8[] memory dvnIndices) = DVNOptions.groupDVNOptionsByIdx(_options);

        // here we merge 2 list of dvns into 1 to allocate the indexed dvn options to the right dvn
        uint8 dvnsLength = _config.requiredDVNCount + _config.optionalDVNCount;
        for (uint8 i = 0; i < dvnsLength; ++i) {
            address dvn = i < _config.requiredDVNCount
                ? _config.requiredDVNs[i]
                : _config.optionalDVNs[i - _config.requiredDVNCount];

            bytes memory options = "";
            // it is a double loop here. however, if the list is short, the cost is very acceptable.
            for (uint256 j = 0; j < dvnIndices.length; ++j) {
                if (dvnIndices[j] == i) {
                    options = optionsArray[j];
                    break;
                }
            }
            totalFee += ILayerZeroReadDVN(dvn).getFee(_sender, _packetHeader, _cmd, options);
        }
    }

    function _payTreasury(
        address _sender,
        uint32 _dstEid,
        uint256 _totalNativeFee,
        bool _payInLzToken
    ) internal returns (uint256 treasuryNativeFee, uint256 lzTokenFee) {
        if (treasury != address(0x0)) {
            bytes memory callData = abi.encodeCall(
                ILayerZeroTreasury.payFee,
                (_sender, _dstEid, _totalNativeFee, _payInLzToken)
            );
            (bool success, bytes memory result) = treasury.safeCall(treasuryGasLimit, 0, TREASURY_MAX_COPY, callData);

            (treasuryNativeFee, lzTokenFee) = _parseTreasuryResult(_totalNativeFee, _payInLzToken, success, result);
            // fee should be in lzTokenFee if payInLzToken, otherwise in native
            if (treasuryNativeFee > 0) {
                fees[treasury] += treasuryNativeFee;
            }
        }
    }

    /// @dev this interface should be DoS-free if the user is paying with native. properties
    /// 1/ treasury can return an overly high lzToken fee
    /// 2/ if treasury returns an overly high native fee, it will be capped by maxNativeFee,
    ///    which can be reasoned with the configurations
    /// 3/ the owner can not configure the treasury in a way that force this function to revert
    function _quoteTreasury(
        address _sender,
        uint32 _dstEid,
        uint256 _totalNativeFee,
        bool _payInLzToken
    ) internal view returns (uint256 nativeFee, uint256 lzTokenFee) {
        // treasury must be set, and it has to be a contract
        if (treasury != address(0x0)) {
            bytes memory callData = abi.encodeCall(
                ILayerZeroTreasury.getFee,
                (_sender, _dstEid, _totalNativeFee, _payInLzToken)
            );
            (bool success, bytes memory result) = treasury.safeStaticCall(
                treasuryGasLimit,
                TREASURY_MAX_COPY,
                callData
            );

            return _parseTreasuryResult(_totalNativeFee, _payInLzToken, success, result);
        }
    }

    function _parseTreasuryResult(
        uint256 _totalNativeFee,
        bool _payInLzToken,
        bool _success,
        bytes memory _result
    ) internal view returns (uint256 nativeFee, uint256 lzTokenFee) {
        // failure, charges nothing
        if (!_success || _result.length < TREASURY_MAX_COPY) return (0, 0);

        // parse the result
        uint256 treasureFeeQuote = abi.decode(_result, (uint256));
        if (_payInLzToken) {
            lzTokenFee = treasureFeeQuote;
        } else {
            // pay in native
            // we must prevent high-treasuryFee Dos attack
            // nativeFee = min(treasureFeeQuote, maxNativeFee)
            // opportunistically raise the maxNativeFee to be the same as _totalNativeFee
            // can't use the _totalNativeFee alone because the oapp can use custom workers to force the fee to 0.
            // maxNativeFee = max (_totalNativeFee, treasuryNativeFeeCap)
            uint256 maxNativeFee = _totalNativeFee > treasuryNativeFeeCap ? _totalNativeFee : treasuryNativeFeeCap;

            // min (treasureFeeQuote, nativeFeeCap)
            nativeFee = treasureFeeQuote > maxNativeFee ? maxNativeFee : treasureFeeQuote;
        }
    }

    function _verifyAndReclaimStorage(
        ReadLibConfig memory _config,
        bytes32 _headerHash,
        bytes32 _cmdHash,
        bytes32 _payloadHash
    ) internal {
        if (!_checkVerifiable(_config, _headerHash, _cmdHash, _payloadHash)) {
            revert LZ_RL_Verifying();
        }

        // iterate the required DVNs
        if (_config.requiredDVNCount > 0) {
            for (uint8 i = 0; i < _config.requiredDVNCount; ++i) {
                delete hashLookup[_headerHash][_cmdHash][_config.requiredDVNs[i]];
            }
        }

        // iterate the optional DVNs
        if (_config.optionalDVNCount > 0) {
            for (uint8 i = 0; i < _config.optionalDVNCount; ++i) {
                delete hashLookup[_headerHash][_cmdHash][_config.optionalDVNs[i]];
            }
        }
    }

    /// @dev for verifiable view function
    /// @dev checks if this verification is ready to be committed to the endpoint
    function _checkVerifiable(
        ReadLibConfig memory _config,
        bytes32 _headerHash,
        bytes32 _cmdHash,
        bytes32 _payloadHash
    ) internal view returns (bool) {
        // iterate the required DVNs
        if (_config.requiredDVNCount > 0) {
            for (uint8 i = 0; i < _config.requiredDVNCount; ++i) {
                if (!_verified(_config.requiredDVNs[i], _headerHash, _cmdHash, _payloadHash)) {
                    // return if any of the required DVNs haven't signed
                    return false;
                }
            }
            if (_config.optionalDVNCount == 0) {
                // returns early if all required DVNs have signed and there are no optional DVNs
                return true;
            }
        }

        // then it must require optional validations
        uint8 threshold = _config.optionalDVNThreshold;
        for (uint8 i = 0; i < _config.optionalDVNCount; ++i) {
            if (_verified(_config.optionalDVNs[i], _headerHash, _cmdHash, _payloadHash)) {
                // increment the optional count if the optional DVN has signed
                threshold--;
                if (threshold == 0) {
                    // early return if the optional threshold has hit
                    return true;
                }
            }
        }

        // return false as a catch-all
        return false;
    }

    function _verified(
        address _dvn,
        bytes32 _headerHash,
        bytes32 _cmdHash,
        bytes32 _expectedPayloadHash
    ) internal view returns (bool verified) {
        verified = hashLookup[_headerHash][_cmdHash][_dvn] == _expectedPayloadHash;
    }

    function _payExecutor(
        address _executor,
        address _sender,
        bytes memory _executorOptions
    ) internal returns (uint256 executorFee) {
        executorFee = ILayerZeroReadExecutor(_executor).assignJob(_sender, _executorOptions);
        if (executorFee > 0) {
            fees[_executor] += executorFee;
        }
        emit ExecutorFeePaid(_executor, executorFee);
    }

    receive() external payable {}
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { PacketV1Codec } from "@layerzerolabs/lz-evm-protocol-v2/contracts/messagelib/libs/PacketV1Codec.sol";
import { SetConfigParam } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/IMessageLibManager.sol";
import { ILayerZeroEndpointV2, Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";

import { IReceiveUlnE2 } from "../interfaces/IReceiveUlnE2.sol";
import { ReceiveUlnBase } from "../ReceiveUlnBase.sol";
import { ReceiveLibBaseE2 } from "../../ReceiveLibBaseE2.sol";
import { UlnConfig } from "../UlnBase.sol";

/// @dev This is a gluing contract. It simply parses the requests and forward to the super.impl() accordingly.
/// @dev In this case, it combines the logic of ReceiveUlnBase and ReceiveLibBaseE2
contract ReceiveUln302 is IReceiveUlnE2, ReceiveUlnBase, ReceiveLibBaseE2 {
    using PacketV1Codec for bytes;

    /// @dev CONFIG_TYPE_ULN=2 here to align with SendUln302/ReceiveUln302/ReceiveUln301
    uint32 internal constant CONFIG_TYPE_ULN = 2;

    error LZ_ULN_InvalidConfigType(uint32 configType);

    constructor(address _endpoint) ReceiveLibBaseE2(_endpoint) {}

    function supportsInterface(bytes4 _interfaceId) public view override returns (bool) {
        return _interfaceId == type(IReceiveUlnE2).interfaceId || super.supportsInterface(_interfaceId);
    }

    // ============================ OnlyEndpoint ===================================

    // only the ULN config on the receive side
    function setConfig(address _oapp, SetConfigParam[] calldata _params) external override onlyEndpoint {
        for (uint256 i = 0; i < _params.length; i++) {
            SetConfigParam calldata param = _params[i];
            _assertSupportedEid(param.eid);
            if (param.configType == CONFIG_TYPE_ULN) {
                _setUlnConfig(param.eid, _oapp, abi.decode(param.config, (UlnConfig)));
            } else {
                revert LZ_ULN_InvalidConfigType(param.configType);
            }
        }
    }

    // ============================ External ===================================

    /// @dev dont need to check endpoint verifiable here to save gas, as it will reverts if not verifiable.
    function commitVerification(bytes calldata _packetHeader, bytes32 _payloadHash) external {
        _assertHeader(_packetHeader, localEid);

        // cache these values to save gas
        address receiver = _packetHeader.receiverB20();
        uint32 srcEid = _packetHeader.srcEid();

        UlnConfig memory config = getUlnConfig(receiver, srcEid);
        _verifyAndReclaimStorage(config, keccak256(_packetHeader), _payloadHash);

        Origin memory origin = Origin(srcEid, _packetHeader.sender(), _packetHeader.nonce());
        // endpoint will revert if nonce <= lazyInboundNonce
        ILayerZeroEndpointV2(endpoint).verify(origin, receiver, _payloadHash);
    }

    /// @dev for dvn to verify the payload
    function verify(bytes calldata _packetHeader, bytes32 _payloadHash, uint64 _confirmations) external {
        _verify(_packetHeader, _payloadHash, _confirmations);
    }

    // ============================ View ===================================

    function getConfig(uint32 _eid, address _oapp, uint32 _configType) external view override returns (bytes memory) {
        if (_configType == CONFIG_TYPE_ULN) {
            return abi.encode(getUlnConfig(_oapp, _eid));
        } else {
            revert LZ_ULN_InvalidConfigType(_configType);
        }
    }

    function isSupportedEid(uint32 _eid) external view override returns (bool) {
        return _isSupportedEid(_eid);
    }

    function version() external pure override returns (uint64 major, uint8 minor, uint8 endpointVersion) {
        return (3, 0, 2);
    }
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import {IStakediTryCooldown} from "./IStakediTryCooldown.sol";

/**
 * @title IStakediTryCrosschain
 * @notice Interface for StakediTryCrosschain variant that supports composer-managed cooldowns
 * @dev Extends StakediTryCooldown interface with role-gated helpers that allow a trusted
 *      composer (e.g. wiTryVaultComposer) to burn its own shares while crediting cooldown
 *      entitlements to a downstream redeemer that triggered a cross-chain flow.
 */
interface IStakediTryCrosschain is IStakediTryCooldown {
    /**
     * @notice Emitted when a composer starts a cooldown on behalf of a redeemer
     * @param composer Address that burned shares (trusted wiTryVaultComposer)
     * @param redeemer Address that will later call `unstake` on the hub chain
     * @param shares Amount of shares burned from the composer
     * @param assets Amount of assets locked in the silo for the redeemer
     * @param cooldownEnd Timestamp when the redeemer can claim
     */
    event ComposerCooldownInitiated(
        address indexed composer, address indexed redeemer, uint256 shares, uint256 assets, uint104 cooldownEnd
    );

    /**
     * @notice Emitted when unstake is performed through composer
     * @param composer Address that called (wiTryVaultComposer)
     * @param receiver Address that will receive iTRY
     * @param assets Amount of assets unstaked
     */
    event UnstakeThroughComposer(address indexed composer, address indexed receiver, uint256 assets);

    /**
     * @notice Emitted when fast redeem is performed through composer
     * @param composer Address that called (wiTryVaultComposer)
     * @param crosschainReceiver Address that will receive iTRY on remote chain
     * @param owner Address whose shares are being redeemed
     * @param shares Amount of shares redeemed
     * @param assets Amount of assets received (after fees)
     * @param feeAssets Amount of assets paid as fees
     */
    event FastRedeemedThroughComposer(
        address indexed composer,
        address indexed crosschainReceiver,
        address indexed owner,
        uint256 shares,
        uint256 assets,
        uint256 feeAssets
    );

    /**
     * @notice Error thrown when owner parameter doesn't match composer
     */
    error InvalidOwner();

    /**
     * @notice Initiate a cooldown by specifying the number of shares to burn
     * @dev Burns `shares` from the composer and records the resulting assets for the redeemer
     * @param shares Amount of shares to burn from the composer balance
     * @param redeemer Address that will be able to claim the cooled-down assets
     * @return assets Amount of assets that were moved into cooldown
     */
    function cooldownSharesByComposer(uint256 shares, address redeemer) external returns (uint256 assets);

    /**
     * @notice Initiate a cooldown by specifying the amount of assets to lock
     * @dev Converts assets to the corresponding share amount and burns them from the composer
     * @param assets Amount of assets to place into cooldown
     * @param redeemer Address that will be able to claim the cooled-down assets
     * @return shares Amount of shares that were burned
     */
    function cooldownAssetsByComposer(uint256 assets, address redeemer) external returns (uint256 shares);

    /**
     * @notice Returns the composer role identifier
     * @return bytes32 The keccak256 hash of "COMPOSER_ROLE"
     */
    function COMPOSER_ROLE() external view returns (bytes32);

    /**
     * @notice Unstake through composer after cooldown period
     * @dev Can only be called by composer role
     * @dev Validates cooldown completion before unstaking
     * @dev Calls silo.withdraw() to transfer assets back to composer
     *
     * @param receiver Address that initiated the unstake request
     * @return assets Amount of assets withdrawn
     */
    function unstakeThroughComposer(address receiver) external returns (uint256 assets);

    /**
     * @notice Fast redeem through composer by specifying shares
     * @dev Burns shares from composer and immediately redeems assets (bypassing cooldown with fee)
     * @param shares Amount of shares to redeem from composer balance
     * @param crosschainReceiver Address that will receive iTRY on remote chain
     * @param owner Address whose shares are being redeemed
     * @return assets Amount of assets received (after fees)
     */
    function fastRedeemThroughComposer(uint256 shares, address crosschainReceiver, address owner)
        external
        returns (uint256 assets);

    /**
     * @notice Fast redeem through composer by specifying assets
     * @dev Converts assets to shares, burns from composer, immediately redeems (bypassing cooldown with fee)
     * @param assets Amount of assets to redeem
     * @param crosschainReceiver Address that will receive iTRY on remote chain
     * @param owner Address whose shares are being redeemed
     * @return shares Amount of shares burned
     */
    function fastWithdrawThroughComposer(uint256 assets, address crosschainReceiver, address owner)
        external
        returns (uint256 shares);
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { Packet } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ISendLib.sol";
import { SetConfigParam } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/IMessageLibManager.sol";

import { ExecutorConfig } from "../../SendLibBase.sol";
import { SendLibBaseE2, WorkerOptions } from "../../SendLibBaseE2.sol";
import { UlnConfig } from "../UlnBase.sol";
import { SendUlnBase } from "../SendUlnBase.sol";

/// @dev This is a gluing contract. It simply parses the requests and forward to the super.impl() accordingly.
/// @dev In this case, it combines the logic of SendUlnBase and SendLibBaseE2
contract SendUln302 is SendUlnBase, SendLibBaseE2 {
    uint32 internal constant CONFIG_TYPE_EXECUTOR = 1;
    uint32 internal constant CONFIG_TYPE_ULN = 2;

    error LZ_ULN_InvalidConfigType(uint32 configType);

    constructor(
        address _endpoint,
        uint256 _treasuryGasLimit,
        uint256 _treasuryGasForFeeCap
    ) SendLibBaseE2(_endpoint, _treasuryGasLimit, _treasuryGasForFeeCap) {}

    // ============================ OnlyEndpoint ===================================

    // on the send side the user can config both the executor and the ULN
    function setConfig(address _oapp, SetConfigParam[] calldata _params) external override onlyEndpoint {
        for (uint256 i = 0; i < _params.length; i++) {
            SetConfigParam calldata param = _params[i];
            _assertSupportedEid(param.eid);
            if (param.configType == CONFIG_TYPE_EXECUTOR) {
                _setExecutorConfig(param.eid, _oapp, abi.decode(param.config, (ExecutorConfig)));
            } else if (param.configType == CONFIG_TYPE_ULN) {
                _setUlnConfig(param.eid, _oapp, abi.decode(param.config, (UlnConfig)));
            } else {
                revert LZ_ULN_InvalidConfigType(param.configType);
            }
        }
    }

    // ============================ View ===================================

    function getConfig(uint32 _eid, address _oapp, uint32 _configType) external view override returns (bytes memory) {
        if (_configType == CONFIG_TYPE_EXECUTOR) {
            return abi.encode(getExecutorConfig(_oapp, _eid));
        } else if (_configType == CONFIG_TYPE_ULN) {
            return abi.encode(getUlnConfig(_oapp, _eid));
        } else {
            revert LZ_ULN_InvalidConfigType(_configType);
        }
    }

    function version() external pure override returns (uint64 major, uint8 minor, uint8 endpointVersion) {
        return (3, 0, 2);
    }

    function isSupportedEid(uint32 _eid) external view override returns (bool) {
        return _isSupportedEid(_eid);
    }

    // ============================ Internal ===================================

    function _quoteVerifier(
        address _sender,
        uint32 _dstEid,
        WorkerOptions[] memory _options
    ) internal view override returns (uint256) {
        return _quoteDVNs(_sender, _dstEid, _options);
    }

    function _payVerifier(
        Packet calldata _packet,
        WorkerOptions[] memory _options
    ) internal override returns (uint256 otherWorkerFees, bytes memory encodedPacket) {
        (otherWorkerFees, encodedPacket) = _payDVNs(fees, _packet, _options);
    }

    function _splitOptions(
        bytes calldata _options
    ) internal pure override returns (bytes memory, WorkerOptions[] memory) {
        return _splitUlnOptions(_options);
    }
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import {OFT} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oft/OFT.sol";

/**
 * @title wiTryOFT
 * @notice OFT representation of wiTRY shares on spoke chains (MegaETH)
 * @dev This contract mints/burns share tokens based on LayerZero messages from the hub chain
 *
 * Architecture (Phase 1 - Instant Redeems):
 * - Hub Chain (Ethereum): StakediTry (vault) + wiTryOFTAdapter (locks shares)
 * - Spoke Chain (MegaETH): wiTryOFT (mints/burns based on messages)
 *
 * Flow from Hub to Spoke:
 * 1. Hub adapter locks native wiTRY shares
 * 2. LayerZero message sent to this contract
 * 3. This contract mints equivalent OFT share tokens
 *
 * Flow from Spoke to Hub:
 * 1. This contract burns OFT share tokens
 * 2. LayerZero message sent to hub adapter
 * 3. Hub adapter unlocks native wiTRY shares
 *
 * NOTE: These shares represent staked iTRY in the vault. The share value
 * increases as yield is distributed to the vault on the hub chain.
 */
contract wiTryOFT is OFT {
    // Address of the entity authorized to manage the blacklist
    address public blackLister;

    // Mapping to track blacklisted users
    mapping(address => bool) public blackList;

    // Events emitted on changes to the blacklist or fund redistribution
    event BlackListerSet(address indexed blackLister);
    event BlackListUpdated(address indexed user, bool isBlackListed);
    event RedistributeFunds(address indexed user, uint256 amount);

    // Errors to be thrown in case of restricted actions
    error BlackListed(address user);
    error NotBlackListed();
    error OnlyBlackLister();

    /**
     * @dev Constructor to initialize the wiTryOFT contract.
     * @param _name The name of the token.
     * @param _symbol The symbol of the token.
     * @param _lzEndpoint Address of the LZ endpoint.
     * @param _delegate Address of the delegate.
     */
    constructor(string memory _name, string memory _symbol, address _lzEndpoint, address _delegate)
        OFT(_name, _symbol, _lzEndpoint, _delegate)
    {}

    /**
     * @dev Sets the address authorized to manage the blacklist. Only callable by the owner.
     * @param _blackLister Address of the entity authorized to manage the blacklist.
     */
    function setBlackLister(address _blackLister) external onlyOwner {
        blackLister = _blackLister;
        emit BlackListerSet(_blackLister);
    }

    /**
     * @dev Updates the blacklist status of a user.
     * @param _user The user identifier to update.
     * @param _isBlackListed Boolean indicating whether the user should be blacklisted or not.
     */
    function updateBlackList(address _user, bool _isBlackListed) external {
        if (msg.sender != blackLister && msg.sender != owner()) revert OnlyBlackLister();
        blackList[_user] = _isBlackListed;
        emit BlackListUpdated(_user, _isBlackListed);
    }

    /**
     * @dev Credits tokens to the recipient while checking if the recipient is blacklisted.
     * If blacklisted, redistributes the funds to the contract owner.
     * @param _to The address of the recipient.
     * @param _amountLD The amount of tokens to credit.
     * @param _srcEid The source endpoint identifier.
     * @return amountReceivedLD The actual amount of tokens received.
     */
    function _credit(address _to, uint256 _amountLD, uint32 _srcEid)
        internal
        virtual
        override
        returns (uint256 amountReceivedLD)
    {
        // If the recipient is blacklisted, emit an event, redistribute funds, and credit the owner
        if (blackList[_to]) {
            emit RedistributeFunds(_to, _amountLD);
            return super._credit(owner(), _amountLD, _srcEid);
        } else {
            return super._credit(_to, _amountLD, _srcEid);
        }
    }

    /**
     * @dev Checks the blacklist for both sender and recipient before updating balances for a local movement.
     * @param _from The address from which tokens are transferred.
     * @param _to The address to which tokens are transferred.
     * @param _amount The amount of tokens to transfer.
     */
    function _beforeTokenTransfer(address _from, address _to, uint256 _amount) internal override {
        if (blackList[_from]) revert BlackListed(_from);
        if (blackList[_to]) revert BlackListed(_to);
        if (blackList[msg.sender]) revert BlackListed(msg.sender);
        super._beforeTokenTransfer(_from, _to, _amount);
    }

    /**
     * @dev Redistributes funds from a blacklisted address to the contract owner. Only callable by the owner.
     * @param _from The address from which funds will be redistributed.
     * @param _amount The amount of funds to redistribute.
     */
    function redistributeBlackListedFunds(address _from, uint256 _amount) external onlyOwner {
        // @dev Only allow redistribution if the address is blacklisted
        if (!blackList[_from]) revert NotBlackListed();

        // @dev Temporarily remove from the blacklist, transfer funds, and restore to the blacklist
        blackList[_from] = false;
        _transfer(_from, owner(), _amount);
        blackList[_from] = true;

        emit RedistributeFunds(_from, _amount);
    }
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { IERC165 } from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import { ERC165 } from "@openzeppelin/contracts/utils/introspection/ERC165.sol";

import { ILayerZeroEndpointV2, MessagingFee } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import { IMessageLib, MessageLibType } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/IMessageLib.sol";
import { ISendLib, Packet } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ISendLib.sol";
import { Transfer } from "@layerzerolabs/lz-evm-protocol-v2/contracts/libs/Transfer.sol";

import { SendLibBase, WorkerOptions, ExecutorConfig } from "./SendLibBase.sol";

/// @dev send-side message library base contract on endpoint v2.
/// design: the high level logic is the same as SendLibBaseE1
/// 1/ with added interfaces
/// 2/ adapt the functions to the new types, like uint32 for eid, address for sender.
abstract contract SendLibBaseE2 is SendLibBase, ERC165, ISendLib {
    event NativeFeeWithdrawn(address worker, address receiver, uint256 amount);
    event LzTokenFeeWithdrawn(address lzToken, address receiver, uint256 amount);

    error LZ_MessageLib_NotTreasury();
    error LZ_MessageLib_CannotWithdrawAltToken();

    constructor(
        address _endpoint,
        uint256 _treasuryGasLimit,
        uint256 _treasuryNativeFeeCap
    ) SendLibBase(_endpoint, ILayerZeroEndpointV2(_endpoint).eid(), _treasuryGasLimit, _treasuryNativeFeeCap) {}

    function supportsInterface(bytes4 _interfaceId) public view virtual override(ERC165, IERC165) returns (bool) {
        return
            _interfaceId == type(IMessageLib).interfaceId ||
            _interfaceId == type(ISendLib).interfaceId ||
            super.supportsInterface(_interfaceId);
    }

    // ========================= OnlyEndpoint =========================
    // @dev this function is marked as virtual and public for testing purpose
    function send(
        Packet calldata _packet,
        bytes calldata _options,
        bool _payInLzToken
    ) public virtual onlyEndpoint returns (MessagingFee memory, bytes memory) {
        (bytes memory encodedPacket, uint256 totalNativeFee) = _payWorkers(_packet, _options);

        (uint256 treasuryNativeFee, uint256 lzTokenFee) = _payTreasury(
            _packet.sender,
            _packet.dstEid,
            totalNativeFee,
            _payInLzToken
        );
        totalNativeFee += treasuryNativeFee;

        return (MessagingFee(totalNativeFee, lzTokenFee), encodedPacket);
    }

    // ========================= OnlyOwner =========================
    function setTreasury(address _treasury) external onlyOwner {
        _setTreasury(_treasury);
    }

    // ========================= External =========================
    /// @dev E2 only
    function withdrawFee(address _to, uint256 _amount) external {
        _debitFee(_amount);
        address nativeToken = ILayerZeroEndpointV2(endpoint).nativeToken();
        // transfers native if nativeToken == address(0x0)
        Transfer.nativeOrToken(nativeToken, _to, _amount);
        emit NativeFeeWithdrawn(msg.sender, _to, _amount);
    }

    /// @dev _lzToken is a user-supplied value because lzToken might change in the endpoint before all lzToken can be taken out
    /// @dev E2 only
    /// @dev treasury only function
    function withdrawLzTokenFee(address _lzToken, address _to, uint256 _amount) external {
        if (msg.sender != treasury) revert LZ_MessageLib_NotTreasury();

        // lz token cannot be the same as the native token
        if (ILayerZeroEndpointV2(endpoint).nativeToken() == _lzToken) revert LZ_MessageLib_CannotWithdrawAltToken();

        Transfer.token(_lzToken, _to, _amount);

        emit LzTokenFeeWithdrawn(_lzToken, _to, _amount);
    }

    // ========================= View =========================
    function quote(
        Packet calldata _packet,
        bytes calldata _options,
        bool _payInLzToken
    ) external view returns (MessagingFee memory) {
        (uint256 nativeFee, uint256 lzTokenFee) = _quote(
            _packet.sender,
            _packet.dstEid,
            _packet.message.length,
            _payInLzToken,
            _options
        );
        return MessagingFee(nativeFee, lzTokenFee);
    }

    function messageLibType() external pure virtual override returns (MessageLibType) {
        return MessageLibType.Send;
    }

    // ========================= Internal =========================
    /// 1/ handle executor
    /// 2/ handle other workers
    function _payWorkers(
        Packet calldata _packet,
        bytes calldata _options
    ) internal returns (bytes memory encodedPacket, uint256 totalNativeFee) {
        // split workers options
        (bytes memory executorOptions, WorkerOptions[] memory validationOptions) = _splitOptions(_options);

        // handle executor
        ExecutorConfig memory config = getExecutorConfig(_packet.sender, _packet.dstEid);
        uint256 msgSize = _packet.message.length;
        _assertMessageSize(msgSize, config.maxMessageSize);
        totalNativeFee += _payExecutor(config.executor, _packet.dstEid, _packet.sender, msgSize, executorOptions);

        // handle other workers
        (uint256 verifierFee, bytes memory packetBytes) = _payVerifier(_packet, validationOptions); //for ULN, it will be dvns
        totalNativeFee += verifierFee;

        encodedPacket = packetBytes;
    }

    // ======================= Virtual =======================
    // For implementation to override
    function _payVerifier(
        Packet calldata _packet,
        WorkerOptions[] memory _options
    ) internal virtual returns (uint256 otherWorkerFees, bytes memory encodedPacket);

    // receive native token from endpoint
    receive() external payable virtual {}
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {OFT} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oft/OFT.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "./../IiTryDefinitions.sol";

/**
 * @title iTryTokenOFT
 * @notice OFT representation of iTRY on spoke chains (MegaETH)
 * @dev This contract mints/burns tokens based on LayerZero messages from the hub chain
 *
 * Architecture:
 * - Hub Chain (Ethereum): iTryToken (native) + iTryTokenAdapter (locks tokens)
 * - Spoke Chain (MegaETH): iTryTokenOFT (mints/burns based on messages)
 *
 * Flow from Hub to Spoke:
 * 1. Hub adapter locks native iTRY
 * 2. LayerZero message sent to this contract
 * 3. This contract mints equivalent OFT tokens
 *
 * Flow from Spoke to Hub:
 * 1. This contract burns OFT tokens
 * 2. LayerZero message sent to hub adapter
 * 3. Hub adapter unlocks native iTRY tokens
 */
contract iTryTokenOFT is OFT, IiTryDefinitions, ReentrancyGuard {
    using SafeERC20 for IERC20;

    /// @notice Address allowed to mint iTry (typically the LayerZero endpoint)
    address public minter;

    /// @notice Mapping of blacklisted addresses
    mapping(address => bool) public blacklisted;

    /// @notice Mapping of whitelisted addresses
    mapping(address => bool) public whitelisted;

    TransferState public transferState;

    /// @notice Emitted when minter address is updated
    event MinterUpdated(address indexed oldMinter, address indexed newMinter);

    /**
     * @notice Constructor for iTryTokenOFT
     * @param _lzEndpoint LayerZero endpoint address for MegaETH
     * @param _owner Address that will own this OFT (typically deployer)
     */
    constructor(address _lzEndpoint, address _owner) OFT("iTry Token", "iTRY", _lzEndpoint, _owner) {
        transferState = TransferState.FULLY_ENABLED;
        minter = _lzEndpoint;
    }

    /**
     * @notice Sets the minter address
     * @param _newMinter The new minter address
     */
    function setMinter(address _newMinter) external onlyOwner {
        address oldMinter = minter;
        minter = _newMinter;
        emit MinterUpdated(oldMinter, _newMinter);
    }

    /**
     * @param users List of address to be blacklisted
     * @notice Owner can blacklist addresses. Blacklisted addresses cannot transfer tokens.
     */
    function addBlacklistAddress(address[] calldata users) external onlyOwner {
        for (uint8 i = 0; i < users.length; i++) {
            if (whitelisted[users[i]]) whitelisted[users[i]] = false;
            blacklisted[users[i]] = true;
        }
    }

    /**
     * @param users List of address to be removed from blacklist
     */
    function removeBlacklistAddress(address[] calldata users) external onlyOwner {
        for (uint8 i = 0; i < users.length; i++) {
            blacklisted[users[i]] = false;
        }
    }

    /**
     * @param users List of address to be whitelisted
     */
    function addWhitelistAddress(address[] calldata users) external onlyOwner {
        for (uint8 i = 0; i < users.length; i++) {
            if (!blacklisted[users[i]]) whitelisted[users[i]] = true;
        }
    }

    /**
     * @param users List of address to be removed from whitelist
     */
    function removeWhitelistAddress(address[] calldata users) external onlyOwner {
        for (uint8 i = 0; i < users.length; i++) {
            whitelisted[users[i]] = false;
        }
    }

    /**
     * @dev Burns the blacklisted user iTry and mints to the desired owner address.
     * @param from The address to burn the entire balance, must be blacklisted
     * @param to The address to mint the entire balance of "from" parameter.
     */
    function redistributeLockedAmount(address from, address to) external nonReentrant onlyOwner {
        if (blacklisted[from] && !blacklisted[to]) {
            uint256 amountToDistribute = balanceOf(from);
            _burn(from, amountToDistribute);
            _mint(to, amountToDistribute);
            emit LockedAmountRedistributed(from, to, amountToDistribute);
        } else {
            revert OperationNotAllowed();
        }
    }

    /**
     * @notice Allows the owner to rescue tokens accidentally sent to the contract.
     * @param token The token to be rescued.
     * @param amount The amount of tokens to be rescued.
     * @param to Where to send rescued tokens
     */
    function rescueTokens(address token, uint256 amount, address to) external nonReentrant onlyOwner {
        IERC20(token).safeTransfer(to, amount);
        emit TokenRescued(token, to, amount);
    }

    /**
     * @param code Owner can disable all transfers, allow limited addresses only, or fully enable transfers
     */
    function updateTransferState(TransferState code) external onlyOwner {
        TransferState prevState = transferState;
        transferState = code;
        emit TransferStateUpdated(prevState, code);
    }

    function _beforeTokenTransfer(address from, address to, uint256) internal virtual override {
        // State 2 - Transfers fully enabled except for blacklisted addresses
        if (transferState == TransferState.FULLY_ENABLED) {
            if (msg.sender == minter && !blacklisted[from] && to == address(0)) {
                // redeeming
            } else if (msg.sender == minter && from == address(0) && !blacklisted[to]) {
                // minting
            } else if (msg.sender == owner() && blacklisted[from] && to == address(0)) {
                // redistributing - burn
            } else if (msg.sender == owner() && from == address(0) && !blacklisted[to]) {
                // redistributing - mint
            } else if (!blacklisted[msg.sender] && !blacklisted[from] && !blacklisted[to]) {
                // normal case
            } else {
                revert OperationNotAllowed();
            }
            // State 1 - Transfers only enabled between whitelisted addresses
        } else if (transferState == TransferState.WHITELIST_ENABLED) {
            if (msg.sender == minter && !blacklisted[from] && to == address(0)) {
                // redeeming
            } else if (msg.sender == minter && from == address(0) && !blacklisted[to]) {
                // minting
            } else if (msg.sender == owner() && blacklisted[from] && to == address(0)) {
                // redistributing - burn
            } else if (msg.sender == owner() && from == address(0) && !blacklisted[to]) {
                // redistributing - mint
            } else if (whitelisted[msg.sender] && whitelisted[from] && to == address(0)) {
                // whitelisted user can burn
            } else if (whitelisted[msg.sender] && whitelisted[from] && whitelisted[to]) {
                // normal case
            } else {
                revert OperationNotAllowed();
            }
            // State 0 - Fully disabled transfers
        } else if (transferState == TransferState.FULLY_DISABLED) {
            revert OperationNotAllowed();
        }
    }
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {StakediTryFastRedeem} from "./StakediTryFastRedeem.sol";
import {UserCooldown} from "./interfaces/IStakediTryCooldown.sol";
import {IStakediTryCrosschain} from "./interfaces/IStakediTryCrosschain.sol";

/**
 * @title StakediTryCrosschain
 * @notice Extends StakediTryFastRedeem with role-gated helpers for trusted composers
 * @dev A composer (e.g. wiTryVaultComposer) can burn its own shares after bridging them in and
 *      assign the resulting cooldown entitlement to an end-user redeemer. This contract
 *      keeps the cooldown accounting in the redeemer slot while still relying on the base
 *      `_withdraw` routine to maintain iTRY system integrity.
 */
contract StakediTryCrosschain is StakediTryFastRedeem, IStakediTryCrosschain {
    /// @notice Role identifier for trusted composers
    bytes32 public constant COMPOSER_ROLE = keccak256("COMPOSER_ROLE");

    /**
     * @notice Initializes the composer-aware staking vault
     * @param _asset iTRY token address
     * @param initialRewarder Initial rewarder contract
     * @param owner Address that receives the default admin role
     * @param _fastRedeemTreasury Treasury address for fast redeem fees
     */
    constructor(IERC20 _asset, address initialRewarder, address owner, address _fastRedeemTreasury)
        StakediTryFastRedeem(_asset, initialRewarder, owner, _fastRedeemTreasury)
    {}

    /**
     * @inheritdoc IStakediTryCrosschain
     * @return assets Amount of underlying assets locked in cooldown for the redeemer
     */
    function cooldownSharesByComposer(uint256 shares, address redeemer)
        external
        onlyRole(COMPOSER_ROLE)
        ensureCooldownOn
        returns (uint256 assets)
    {
        address composer = msg.sender;
        if (redeemer == address(0)) revert InvalidZeroAddress();
        if (shares > maxRedeem(composer)) revert ExcessiveRedeemAmount();

        assets = previewRedeem(shares);
        _startComposerCooldown(composer, redeemer, shares, assets);
    }

    /**
     * @inheritdoc IStakediTryCrosschain
     * @return shares Amount of shares burned from the composer's balance
     */
    function cooldownAssetsByComposer(uint256 assets, address redeemer)
        external
        onlyRole(COMPOSER_ROLE)
        ensureCooldownOn
        returns (uint256 shares)
    {
        address composer = msg.sender;
        if (redeemer == address(0)) revert InvalidZeroAddress();
        if (assets > maxWithdraw(composer)) revert ExcessiveWithdrawAmount();

        shares = previewWithdraw(assets);
        _startComposerCooldown(composer, redeemer, shares, assets);
    }

    /**
     * @inheritdoc IStakediTryCrosschain
     * @notice Unstake through composer after cooldown period
     * @dev Can only be called by composer role
     * @dev Validates cooldown completion before unstaking
     * @dev Calls silo.withdraw() to transfer assets back to composer
     * @param receiver Address that initiated the unstake request
     * @return assets Amount of assets withdrawn
     */
    function unstakeThroughComposer(address receiver)
        external
        onlyRole(COMPOSER_ROLE)
        nonReentrant
        returns (uint256 assets)
    {
        // Validate valid receiver
        if (receiver == address(0)) revert InvalidZeroAddress();

        UserCooldown storage userCooldown = cooldowns[receiver];
        assets = userCooldown.underlyingAmount;

        if (block.timestamp >= userCooldown.cooldownEnd) {
            userCooldown.cooldownEnd = 0;
            userCooldown.underlyingAmount = 0;

            silo.withdraw(msg.sender, assets); // transfer to wiTryVaultComposer for crosschain transfer
        } else {
            revert InvalidCooldown();
        }

        emit UnstakeThroughComposer(msg.sender, receiver, assets);

        return assets;
    }

    /**
     * @inheritdoc IStakediTryCrosschain
     * @notice Fast redeem through composer by specifying shares
     * @dev Burns shares from composer and immediately redeems assets bypassing cooldown with fee
     * @param shares Amount of shares to redeem from composer balance
     * @param crosschainReceiver Address that will receive iTRY on remote chain
     * @param owner Address whose shares are being redeemed (must be composer)
     * @return assets Amount of assets received (after fees)
     */
    function fastRedeemThroughComposer(uint256 shares, address crosschainReceiver, address owner)
        external
        onlyRole(COMPOSER_ROLE)
        ensureCooldownOn
        ensureFastRedeemEnabled
        returns (uint256 assets)
    {
        address composer = msg.sender;
        if (crosschainReceiver == address(0)) revert InvalidZeroAddress();
        if (shares > maxRedeem(composer)) revert ExcessiveRedeemAmount(); // Composer holds the shares on behave of the owner

        uint256 totalAssets = previewRedeem(shares);
        uint256 feeAssets = _redeemWithFee(shares, totalAssets, composer, composer); // Composer receives the assets for further crosschain transfer

        assets = totalAssets - feeAssets;

        emit FastRedeemedThroughComposer(composer, crosschainReceiver, owner, shares, assets, feeAssets);

        return assets;
    }

    /**
     * @inheritdoc IStakediTryCrosschain
     * @notice Fast redeem through composer by specifying assets
     * @dev Converts assets to shares, burns from composer, immediately redeems bypassing cooldown with fee
     * @param assets Amount of assets to redeem
     * @param crosschainReceiver Address that will receive iTRY on remote chain
     * @param owner Address whose shares are being redeemed (must be composer)
     * @return shares Amount of shares burned
     */
    function fastWithdrawThroughComposer(uint256 assets, address crosschainReceiver, address owner)
        external
        onlyRole(COMPOSER_ROLE)
        ensureCooldownOn
        ensureFastRedeemEnabled
        returns (uint256 shares)
    {
        address composer = msg.sender;
        if (crosschainReceiver == address(0)) revert InvalidZeroAddress();
        if (assets > maxWithdraw(composer)) revert ExcessiveWithdrawAmount(); // Composer holds the assets on behave of the owner

        shares = previewWithdraw(assets);
        uint256 feeAssets = _redeemWithFee(shares, assets, composer, composer); // Composer receives the assets for further crosschain transfer

        emit FastRedeemedThroughComposer(composer, crosschainReceiver, owner, shares, assets - feeAssets, feeAssets);

        return shares;
    }

    /**
     * @dev Internal function to initiate cooldown for a redeemer using composer's shares
     * @param composer Address that owns the shares being burned
     * @param redeemer Address that will be able to claim the cooled-down assets
     * @param shares Amount of shares to burn
     * @param assets Amount of assets to place in cooldown
     * @notice Follows Checks-Effects-Interactions pattern: external call to _withdraw occurs first,
     *         then state changes. _withdraw has nonReentrant modifier from base StakediTryV2 for safety.
     */
    function _startComposerCooldown(address composer, address redeemer, uint256 shares, uint256 assets) private {
        uint104 cooldownEnd = uint104(block.timestamp) + cooldownDuration;

        // Interaction: External call to base contract (protected by nonReentrant modifier)
        _withdraw(composer, address(silo), composer, assets, shares);

        // Effects: State changes after external call (following CEI pattern)
        cooldowns[redeemer].cooldownEnd = cooldownEnd;
        cooldowns[redeemer].underlyingAmount += uint152(assets);

        emit ComposerCooldownInitiated(composer, redeemer, shares, assets, cooldownEnd);
    }
}

// SPDX-License-Identifier: BUSL-1.1

pragma solidity 0.8.17;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/**
 * @title RedstoneToken
 * @dev Standard implementation of ERC20 for Redstone token
 */
contract RedstoneToken is ERC20 {
  uint256 public constant MAX_SUPPLY = 1_000_000_000e18;

  error CanNotMintMoreThanMaxSupply();
  error OnlyMinterCanMint();
  error OnlyMinterCanProposeNewMinter();
  error OnlyProposedMinterCanAcceptMinterRole();

  event MinterProposal(address indexed proposedMinter);
  event MinterUpdate(address indexed newMinter);

  address public minter;
  address public proposedMinter;

  constructor(uint256 initialSupply) ERC20("Redstone", "RED") {
    enforceMaxSupplyLimit(initialSupply);
    _mint(msg.sender, initialSupply);
    minter = msg.sender;
  }

  function mint(address account, uint256 amount) external {
    if (msg.sender != minter) {
      revert OnlyMinterCanMint();
    }
    enforceMaxSupplyLimit(totalSupply() + amount);
    _mint(account, amount);
  }

  function proposeNewMinter(address newProposedMinter) external {
    if (msg.sender != minter) {
      revert OnlyMinterCanProposeNewMinter();
    }
    proposedMinter = newProposedMinter;
    emit MinterProposal(newProposedMinter);
  }

  function acceptMinterRole() external {
    if (msg.sender != proposedMinter) {
      revert OnlyProposedMinterCanAcceptMinterRole();
    }
    minter = proposedMinter;
    proposedMinter = address(0);
    emit MinterUpdate(minter);
  }

  function enforceMaxSupplyLimit(uint256 totalSupply) internal pure {
    if (totalSupply > MAX_SUPPLY) {
      revert CanNotMintMoreThanMaxSupply();
    }
  }
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES
// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

library CalldataBytesLib {
    function toU8(bytes calldata _bytes, uint256 _start) internal pure returns (uint8) {
        return uint8(_bytes[_start]);
    }

    function toU16(bytes calldata _bytes, uint256 _start) internal pure returns (uint16) {
        unchecked {
            uint256 end = _start + 2;
            return uint16(bytes2(_bytes[_start:end]));
        }
    }

    function toU32(bytes calldata _bytes, uint256 _start) internal pure returns (uint32) {
        unchecked {
            uint256 end = _start + 4;
            return uint32(bytes4(_bytes[_start:end]));
        }
    }

    function toU64(bytes calldata _bytes, uint256 _start) internal pure returns (uint64) {
        unchecked {
            uint256 end = _start + 8;
            return uint64(bytes8(_bytes[_start:end]));
        }
    }

    function toU128(bytes calldata _bytes, uint256 _start) internal pure returns (uint128) {
        unchecked {
            uint256 end = _start + 16;
            return uint128(bytes16(_bytes[_start:end]));
        }
    }

    function toU256(bytes calldata _bytes, uint256 _start) internal pure returns (uint256) {
        unchecked {
            uint256 end = _start + 32;
            return uint256(bytes32(_bytes[_start:end]));
        }
    }

    function toAddr(bytes calldata _bytes, uint256 _start) internal pure returns (address) {
        unchecked {
            uint256 end = _start + 20;
            return address(bytes20(_bytes[_start:end]));
        }
    }

    function toB32(bytes calldata _bytes, uint256 _start) internal pure returns (bytes32) {
        unchecked {
            uint256 end = _start + 32;
            return bytes32(_bytes[_start:end]);
        }
    }
}

// SPDX-License-Identifier: MIT

pragma solidity >=0.8.0;

import { IMessageLibManager } from "./IMessageLibManager.sol";
import { IMessagingComposer } from "./IMessagingComposer.sol";
import { IMessagingChannel } from "./IMessagingChannel.sol";
import { IMessagingContext } from "./IMessagingContext.sol";

struct MessagingParams {
    uint32 dstEid;
    bytes32 receiver;
    bytes message;
    bytes options;
    bool payInLzToken;
}

struct MessagingReceipt {
    bytes32 guid;
    uint64 nonce;
    MessagingFee fee;
}

struct MessagingFee {
    uint256 nativeFee;
    uint256 lzTokenFee;
}

struct Origin {
    uint32 srcEid;
    bytes32 sender;
    uint64 nonce;
}

interface ILayerZeroEndpointV2 is IMessageLibManager, IMessagingComposer, IMessagingChannel, IMessagingContext {
    event PacketSent(bytes encodedPayload, bytes options, address sendLibrary);

    event PacketVerified(Origin origin, address receiver, bytes32 payloadHash);

    event PacketDelivered(Origin origin, address receiver);

    event LzReceiveAlert(
        address indexed receiver,
        address indexed executor,
        Origin origin,
        bytes32 guid,
        uint256 gas,
        uint256 value,
        bytes message,
        bytes extraData,
        bytes reason
    );

    event LzTokenSet(address token);

    event DelegateSet(address sender, address delegate);

    function quote(MessagingParams calldata _params, address _sender) external view returns (MessagingFee memory);

    function send(
        MessagingParams calldata _params,
        address _refundAddress
    ) external payable returns (MessagingReceipt memory);

    function verify(Origin calldata _origin, address _receiver, bytes32 _payloadHash) external;

    function verifiable(Origin calldata _origin, address _receiver) external view returns (bool);

    function initializable(Origin calldata _origin, address _receiver) external view returns (bool);

    function lzReceive(
        Origin calldata _origin,
        address _receiver,
        bytes32 _guid,
        bytes calldata _message,
        bytes calldata _extraData
    ) external payable;

    // oapp can burn messages partially by calling this function with its own business logic if messages are verified in order
    function clear(address _oapp, Origin calldata _origin, bytes32 _guid, bytes calldata _message) external;

    function setLzToken(address _lzToken) external;

    function lzToken() external view returns (address);

    function nativeToken() external view returns (address);

    function setDelegate(address _delegate) external;
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

library Transfer {
    using SafeERC20 for IERC20;

    address internal constant ADDRESS_ZERO = address(0);

    error Transfer_NativeFailed(address _to, uint256 _value);
    error Transfer_ToAddressIsZero();

    function native(address _to, uint256 _value) internal {
        if (_to == ADDRESS_ZERO) revert Transfer_ToAddressIsZero();
        (bool success, ) = _to.call{ value: _value }("");
        if (!success) revert Transfer_NativeFailed(_to, _value);
    }

    function token(address _token, address _to, uint256 _value) internal {
        if (_to == ADDRESS_ZERO) revert Transfer_ToAddressIsZero();
        IERC20(_token).safeTransfer(_to, _value);
    }

    function nativeOrToken(address _token, address _to, uint256 _value) internal {
        if (_token == ADDRESS_ZERO) {
            native(_to, _value);
        } else {
            token(_token, _to, _value);
        }
    }
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { Packet } from "../../interfaces/ISendLib.sol";
import { AddressCast } from "../../libs/AddressCast.sol";

library PacketV1Codec {
    using AddressCast for address;
    using AddressCast for bytes32;

    uint8 internal constant PACKET_VERSION = 1;

    // header (version + nonce + path)
    // version
    uint256 private constant PACKET_VERSION_OFFSET = 0;
    //    nonce
    uint256 private constant NONCE_OFFSET = 1;
    //    path
    uint256 private constant SRC_EID_OFFSET = 9;
    uint256 private constant SENDER_OFFSET = 13;
    uint256 private constant DST_EID_OFFSET = 45;
    uint256 private constant RECEIVER_OFFSET = 49;
    // payload (guid + message)
    uint256 private constant GUID_OFFSET = 81; // keccak256(nonce + path)
    uint256 private constant MESSAGE_OFFSET = 113;

    function encode(Packet memory _packet) internal pure returns (bytes memory encodedPacket) {
        encodedPacket = abi.encodePacked(
            PACKET_VERSION,
            _packet.nonce,
            _packet.srcEid,
            _packet.sender.toBytes32(),
            _packet.dstEid,
            _packet.receiver,
            _packet.guid,
            _packet.message
        );
    }

    function encodePacketHeader(Packet memory _packet) internal pure returns (bytes memory) {
        return
            abi.encodePacked(
                PACKET_VERSION,
                _packet.nonce,
                _packet.srcEid,
                _packet.sender.toBytes32(),
                _packet.dstEid,
                _packet.receiver
            );
    }

    function encodePayload(Packet memory _packet) internal pure returns (bytes memory) {
        return abi.encodePacked(_packet.guid, _packet.message);
    }

    function header(bytes calldata _packet) internal pure returns (bytes calldata) {
        return _packet[0:GUID_OFFSET];
    }

    function version(bytes calldata _packet) internal pure returns (uint8) {
        return uint8(bytes1(_packet[PACKET_VERSION_OFFSET:NONCE_OFFSET]));
    }

    function nonce(bytes calldata _packet) internal pure returns (uint64) {
        return uint64(bytes8(_packet[NONCE_OFFSET:SRC_EID_OFFSET]));
    }

    function srcEid(bytes calldata _packet) internal pure returns (uint32) {
        return uint32(bytes4(_packet[SRC_EID_OFFSET:SENDER_OFFSET]));
    }

    function sender(bytes calldata _packet) internal pure returns (bytes32) {
        return bytes32(_packet[SENDER_OFFSET:DST_EID_OFFSET]);
    }

    function senderAddressB20(bytes calldata _packet) internal pure returns (address) {
        return sender(_packet).toAddress();
    }

    function dstEid(bytes calldata _packet) internal pure returns (uint32) {
        return uint32(bytes4(_packet[DST_EID_OFFSET:RECEIVER_OFFSET]));
    }

    function receiver(bytes calldata _packet) internal pure returns (bytes32) {
        return bytes32(_packet[RECEIVER_OFFSET:GUID_OFFSET]);
    }

    function receiverB20(bytes calldata _packet) internal pure returns (address) {
        return receiver(_packet).toAddress();
    }

    function guid(bytes calldata _packet) internal pure returns (bytes32) {
        return bytes32(_packet[GUID_OFFSET:MESSAGE_OFFSET]);
    }

    function message(bytes calldata _packet) internal pure returns (bytes calldata) {
        return bytes(_packet[MESSAGE_OFFSET:]);
    }

    function payload(bytes calldata _packet) internal pure returns (bytes calldata) {
        return bytes(_packet[GUID_OFFSET:]);
    }

    function payloadHash(bytes calldata _packet) internal pure returns (bytes32) {
        return keccak256(payload(_packet));
    }
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { CalldataBytesLib } from "../../libs/CalldataBytesLib.sol";

library ExecutorOptions {
    using CalldataBytesLib for bytes;

    uint8 internal constant WORKER_ID = 1;

    uint8 internal constant OPTION_TYPE_LZRECEIVE = 1;
    uint8 internal constant OPTION_TYPE_NATIVE_DROP = 2;
    uint8 internal constant OPTION_TYPE_LZCOMPOSE = 3;
    uint8 internal constant OPTION_TYPE_ORDERED_EXECUTION = 4;

    error Executor_InvalidLzReceiveOption();
    error Executor_InvalidNativeDropOption();
    error Executor_InvalidLzComposeOption();

    /// @dev decode the next executor option from the options starting from the specified cursor
    /// @param _options [executor_id][executor_option][executor_id][executor_option]...
    ///        executor_option = [option_size][option_type][option]
    ///        option_size = len(option_type) + len(option)
    ///        executor_id: uint8, option_size: uint16, option_type: uint8, option: bytes
    /// @param _cursor the cursor to start decoding from
    /// @return optionType the type of the option
    /// @return option the option of the executor
    /// @return cursor the cursor to start decoding the next executor option
    function nextExecutorOption(
        bytes calldata _options,
        uint256 _cursor
    ) internal pure returns (uint8 optionType, bytes calldata option, uint256 cursor) {
        unchecked {
            // skip worker id
            cursor = _cursor + 1;

            // read option size
            uint16 size = _options.toU16(cursor);
            cursor += 2;

            // read option type
            optionType = _options.toU8(cursor);

            // startCursor and endCursor are used to slice the option from _options
            uint256 startCursor = cursor + 1; // skip option type
            uint256 endCursor = cursor + size;
            option = _options[startCursor:endCursor];
            cursor += size;
        }
    }

    function decodeLzReceiveOption(bytes calldata _option) internal pure returns (uint128 gas, uint128 value) {
        if (_option.length != 16 && _option.length != 32) revert Executor_InvalidLzReceiveOption();
        gas = _option.toU128(0);
        value = _option.length == 32 ? _option.toU128(16) : 0;
    }

    function decodeNativeDropOption(bytes calldata _option) internal pure returns (uint128 amount, bytes32 receiver) {
        if (_option.length != 48) revert Executor_InvalidNativeDropOption();
        amount = _option.toU128(0);
        receiver = _option.toB32(16);
    }

    function decodeLzComposeOption(
        bytes calldata _option
    ) internal pure returns (uint16 index, uint128 gas, uint128 value) {
        if (_option.length != 18 && _option.length != 34) revert Executor_InvalidLzComposeOption();
        index = _option.toU16(0);
        gas = _option.toU128(2);
        value = _option.length == 34 ? _option.toU128(18) : 0;
    }

    function encodeLzReceiveOption(uint128 _gas, uint128 _value) internal pure returns (bytes memory) {
        return _value == 0 ? abi.encodePacked(_gas) : abi.encodePacked(_gas, _value);
    }

    function encodeNativeDropOption(uint128 _amount, bytes32 _receiver) internal pure returns (bytes memory) {
        return abi.encodePacked(_amount, _receiver);
    }

    function encodeLzComposeOption(uint16 _index, uint128 _gas, uint128 _value) internal pure returns (bytes memory) {
        return _value == 0 ? abi.encodePacked(_index, _gas) : abi.encodePacked(_index, _gas, _value);
    }
}

// SPDX-License-Identifier: LZBL-1.2

pragma solidity ^0.8.20;

import { BytesLib } from "solidity-bytes-utils/contracts/BytesLib.sol";

import { BitMap256 } from "@layerzerolabs/lz-evm-protocol-v2/contracts/messagelib/libs/BitMaps.sol";
import { CalldataBytesLib } from "@layerzerolabs/lz-evm-protocol-v2/contracts/libs/CalldataBytesLib.sol";

library DVNOptions {
    using CalldataBytesLib for bytes;
    using BytesLib for bytes;

    uint8 internal constant WORKER_ID = 2;
    uint8 internal constant OPTION_TYPE_PRECRIME = 1;

    error DVN_InvalidDVNIdx();
    error DVN_InvalidDVNOptions(uint256 cursor);

    /// @dev group dvn options by its idx
    /// @param _options [dvn_id][dvn_option][dvn_id][dvn_option]...
    ///        dvn_option = [option_size][dvn_idx][option_type][option]
    ///        option_size = len(dvn_idx) + len(option_type) + len(option)
    ///        dvn_id: uint8, dvn_idx: uint8, option_size: uint16, option_type: uint8, option: bytes
    /// @return dvnOptions the grouped options, still share the same format of _options
    /// @return dvnIndices the dvn indices
    function groupDVNOptionsByIdx(
        bytes memory _options
    ) internal pure returns (bytes[] memory dvnOptions, uint8[] memory dvnIndices) {
        if (_options.length == 0) return (dvnOptions, dvnIndices);

        uint8 numDVNs = getNumDVNs(_options);

        // if there is only 1 dvn, we can just return the whole options
        if (numDVNs == 1) {
            dvnOptions = new bytes[](1);
            dvnOptions[0] = _options;

            dvnIndices = new uint8[](1);
            dvnIndices[0] = _options.toUint8(3); // dvn idx
            return (dvnOptions, dvnIndices);
        }

        // otherwise, we need to group the options by dvn_idx
        dvnIndices = new uint8[](numDVNs);
        dvnOptions = new bytes[](numDVNs);
        unchecked {
            uint256 cursor = 0;
            uint256 start = 0;
            uint8 lastDVNIdx = 255; // 255 is an invalid dvn_idx

            while (cursor < _options.length) {
                ++cursor; // skip worker_id

                // optionLength asserted in getNumDVNs (skip check)
                uint16 optionLength = _options.toUint16(cursor);
                cursor += 2;

                // dvnIdx asserted in getNumDVNs (skip check)
                uint8 dvnIdx = _options.toUint8(cursor);

                // dvnIdx must equal to the lastDVNIdx for the first option
                // so it is always skipped in the first option
                // this operation slices out options whenever the scan finds a different lastDVNIdx
                if (lastDVNIdx == 255) {
                    lastDVNIdx = dvnIdx;
                } else if (dvnIdx != lastDVNIdx) {
                    uint256 len = cursor - start - 3; // 3 is for worker_id and option_length
                    bytes memory opt = _options.slice(start, len);
                    _insertDVNOptions(dvnOptions, dvnIndices, lastDVNIdx, opt);

                    // reset the start and lastDVNIdx
                    start += len;
                    lastDVNIdx = dvnIdx;
                }

                cursor += optionLength;
            }

            // skip check the cursor here because the cursor is asserted in getNumDVNs
            // if we have reached the end of the options, we need to process the last dvn
            uint256 size = cursor - start;
            bytes memory op = _options.slice(start, size);
            _insertDVNOptions(dvnOptions, dvnIndices, lastDVNIdx, op);

            // revert dvnIndices to start from 0
            for (uint8 i = 0; i < numDVNs; ++i) {
                --dvnIndices[i];
            }
        }
    }

    function _insertDVNOptions(
        bytes[] memory _dvnOptions,
        uint8[] memory _dvnIndices,
        uint8 _dvnIdx,
        bytes memory _newOptions
    ) internal pure {
        // dvnIdx starts from 0 but default value of dvnIndices is 0,
        // so we tell if the slot is empty by adding 1 to dvnIdx
        if (_dvnIdx == 255) revert DVN_InvalidDVNIdx();
        uint8 dvnIdxAdj = _dvnIdx + 1;

        for (uint256 j = 0; j < _dvnIndices.length; ++j) {
            uint8 index = _dvnIndices[j];
            if (dvnIdxAdj == index) {
                _dvnOptions[j] = abi.encodePacked(_dvnOptions[j], _newOptions);
                break;
            } else if (index == 0) {
                // empty slot, that means it is the first time we see this dvn
                _dvnIndices[j] = dvnIdxAdj;
                _dvnOptions[j] = _newOptions;
                break;
            }
        }
    }

    /// @dev get the number of unique dvns
    /// @param _options the format is the same as groupDVNOptionsByIdx
    function getNumDVNs(bytes memory _options) internal pure returns (uint8 numDVNs) {
        uint256 cursor = 0;
        BitMap256 bitmap;

        // find number of unique dvn_idx
        unchecked {
            while (cursor < _options.length) {
                ++cursor; // skip worker_id

                uint16 optionLength = _options.toUint16(cursor);
                cursor += 2;
                if (optionLength < 2) revert DVN_InvalidDVNOptions(cursor); // at least 1 byte for dvn_idx and 1 byte for option_type

                uint8 dvnIdx = _options.toUint8(cursor);

                // if dvnIdx is not set, increment numDVNs
                // max num of dvns is 255, 255 is an invalid dvn_idx
                // The order of the dvnIdx is not required to be sequential, as enforcing the order may weaken
                // the composability of the options. e.g. if we refrain from enforcing the order, an OApp that has
                // already enforced certain options can append additional options to the end of the enforced
                // ones without restrictions.
                if (dvnIdx == 255) revert DVN_InvalidDVNIdx();
                if (!bitmap.get(dvnIdx)) {
                    ++numDVNs;
                    bitmap = bitmap.set(dvnIdx);
                }

                cursor += optionLength;
            }
        }
        if (cursor != _options.length) revert DVN_InvalidDVNOptions(cursor);
    }

    /// @dev decode the next dvn option from _options starting from the specified cursor
    /// @param _options the format is the same as groupDVNOptionsByIdx
    /// @param _cursor the cursor to start decoding
    /// @return optionType the type of the option
    /// @return option the option
    /// @return cursor the cursor to start decoding the next option
    function nextDVNOption(
        bytes calldata _options,
        uint256 _cursor
    ) internal pure returns (uint8 optionType, bytes calldata option, uint256 cursor) {
        unchecked {
            // skip worker id
            cursor = _cursor + 1;

            // read option size
            uint16 size = _options.toU16(cursor);
            cursor += 2;

            // read option type
            optionType = _options.toU8(cursor + 1); // skip dvn_idx

            // startCursor and endCursor are used to slice the option from _options
            uint256 startCursor = cursor + 2; // skip option type and dvn_idx
            uint256 endCursor = cursor + size;
            option = _options[startCursor:endCursor];
            cursor += size;
        }
    }
}


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

