
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title AbstractCCTPIntegrator
 * @author Origin Protocol Inc
 *
 * @dev Abstract contract that contains all the logic used to integrate with CCTP.
 */

import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { IERC20 } from "../../utils/InitializableAbstractStrategy.sol";

import { ICCTPTokenMessenger, ICCTPMessageTransmitter, IMessageHandlerV2 } from "../../interfaces/cctp/ICCTP.sol";

import { CrossChainStrategyHelper } from "./CrossChainStrategyHelper.sol";
import { Governable } from "../../governance/Governable.sol";
import { BytesHelper } from "../../utils/BytesHelper.sol";
import "../../utils/Helpers.sol";

abstract contract AbstractCCTPIntegrator is Governable, IMessageHandlerV2 {
    using SafeERC20 for IERC20;

    using BytesHelper for bytes;
    using CrossChainStrategyHelper for bytes;

    event LastTransferNonceUpdated(uint64 lastTransferNonce);
    event NonceProcessed(uint64 nonce);

    event CCTPMinFinalityThresholdSet(uint16 minFinalityThreshold);
    event CCTPFeePremiumBpsSet(uint16 feePremiumBps);
    event OperatorChanged(address operator);
    event TokensBridged(
        uint32 destinationDomain,
        address peerStrategy,
        address tokenAddress,
        uint256 tokenAmount,
        uint256 maxFee,
        uint32 minFinalityThreshold,
        bytes hookData
    );
    event MessageTransmitted(
        uint32 destinationDomain,
        address peerStrategy,
        uint32 minFinalityThreshold,
        bytes message
    );

    // Message body V2 fields
    // Ref: https://developers.circle.com/cctp/technical-guide#message-body
    // Ref: https://github.com/circlefin/evm-cctp-contracts/blob/master/src/messages/v2/BurnMessageV2.sol
    uint8 private constant BURN_MESSAGE_V2_VERSION_INDEX = 0;
    uint8 private constant BURN_MESSAGE_V2_BURN_TOKEN_INDEX = 4;
    uint8 private constant BURN_MESSAGE_V2_RECIPIENT_INDEX = 36;
    uint8 private constant BURN_MESSAGE_V2_AMOUNT_INDEX = 68;
    uint8 private constant BURN_MESSAGE_V2_MESSAGE_SENDER_INDEX = 100;
    uint8 private constant BURN_MESSAGE_V2_FEE_EXECUTED_INDEX = 164;
    uint8 private constant BURN_MESSAGE_V2_HOOK_DATA_INDEX = 228;

    /**
     * @notice  Max transfer threshold imposed by the CCTP
     *          Ref: https://developers.circle.com/cctp/evm-smart-contracts#depositforburn
     * @dev     10M USDC limit applies to both standard and fast transfer modes. The fast transfer mode has
     *          an additional limitation that is not present on-chain and Circle may alter that amount off-chain
     *          at their preference. The amount available for fast transfer can be queried here:
     *          https://iris-api.circle.com/v2/fastBurn/USDC/allowance .
     *          If a fast transfer token transaction has been issued and there is not enough allowance for it
     *          the off-chain Iris component will re-attempt the transaction and if it fails it will fallback
     *          to a standard transfer. Reference section 4.3 in the whitepaper:
     *          https://6778953.fs1.hubspotusercontent-na1.net/hubfs/6778953/PDFs/Whitepapers/CCTPV2_White_Paper.pdf
     */
    uint256 public constant MAX_TRANSFER_AMOUNT = 10_000_000 * 10**6; // 10M USDC

    /// @notice Minimum transfer amount to avoid zero or dust transfers
    uint256 public constant MIN_TRANSFER_AMOUNT = 10**6;

    // CCTP contracts
    // This implementation assumes that remote and local chains have these contracts
    // deployed on the same addresses.
    /// @notice CCTP message transmitter contract
    ICCTPMessageTransmitter public immutable cctpMessageTransmitter;
    /// @notice CCTP token messenger contract
    ICCTPTokenMessenger public immutable cctpTokenMessenger;

    /// @notice USDC address on local chain
    address public immutable usdcToken;

    /// @notice USDC address on remote chain
    address public immutable peerUsdcToken;

    /// @notice Domain ID of the chain from which messages are accepted
    uint32 public immutable peerDomainID;

    /// @notice Strategy address on other chain
    address public immutable peerStrategy;

    /**
     * @notice Minimum finality threshold
     *         Can be 1000 (safe, after 1 epoch) or 2000 (finalized, after 2 epochs).
     *         Ref: https://developers.circle.com/cctp/technical-guide#finality-thresholds
     * @dev    When configuring the contract for fast transfer we should check the available
     *         allowance of USDC that can be bridged using fast mode:
     *         wget https://iris-api.circle.com/v2/fastBurn/USDC/allowance
     */
    uint16 public minFinalityThreshold;

    /// @notice Fee premium in basis points
    uint16 public feePremiumBps;

    /// @notice Nonce of the last known deposit or withdrawal
    uint64 public lastTransferNonce;

    /// @notice Operator address: Can relay CCTP messages
    address public operator;

    /// @notice Mapping of processed nonces
    mapping(uint64 => bool) private nonceProcessed;

    // For future use
    uint256[48] private __gap;

    modifier onlyCCTPMessageTransmitter() {
        require(
            msg.sender == address(cctpMessageTransmitter),
            "Caller is not CCTP transmitter"
        );
        _;
    }

    modifier onlyOperator() {
        require(msg.sender == operator, "Caller is not the Operator");
        _;
    }

    /**
     * @notice Configuration for CCTP integration
     * @param cctpTokenMessenger Address of the CCTP token messenger contract
     * @param cctpMessageTransmitter Address of the CCTP message transmitter contract
     * @param peerDomainID Domain ID of the chain from which messages are accepted.
     *         0 for Ethereum, 6 for Base, etc.
     *         Ref: https://developers.circle.com/cctp/cctp-supported-blockchains
     * @param peerStrategy Address of the master or remote strategy on the other chain
     * @param usdcToken USDC address on local chain
     */
    struct CCTPIntegrationConfig {
        address cctpTokenMessenger;
        address cctpMessageTransmitter;
        uint32 peerDomainID;
        address peerStrategy;
        address usdcToken;
        address peerUsdcToken;
    }

    constructor(CCTPIntegrationConfig memory _config) {
        require(_config.usdcToken != address(0), "Invalid USDC address");
        require(
            _config.peerUsdcToken != address(0),
            "Invalid peer USDC address"
        );
        require(
            _config.cctpTokenMessenger != address(0),
            "Invalid CCTP config"
        );
        require(
            _config.cctpMessageTransmitter != address(0),
            "Invalid CCTP config"
        );
        require(
            _config.peerStrategy != address(0),
            "Invalid peer strategy address"
        );

        cctpMessageTransmitter = ICCTPMessageTransmitter(
            _config.cctpMessageTransmitter
        );
        cctpTokenMessenger = ICCTPTokenMessenger(_config.cctpTokenMessenger);

        // Domain ID of the chain from which messages are accepted
        peerDomainID = _config.peerDomainID;

        // Strategy address on other chain, should
        // always be same as the proxy of this strategy
        peerStrategy = _config.peerStrategy;

        // USDC address on local chain
        usdcToken = _config.usdcToken;

        // Just a sanity check to ensure the base token is USDC
        uint256 _usdcTokenDecimals = Helpers.getDecimals(_config.usdcToken);
        string memory _usdcTokenSymbol = Helpers.getSymbol(_config.usdcToken);
        require(_usdcTokenDecimals == 6, "Base token decimals must be 6");
        require(
            keccak256(abi.encodePacked(_usdcTokenSymbol)) ==
                keccak256(abi.encodePacked("USDC")),
            "Token symbol must be USDC"
        );

        // USDC address on remote chain
        peerUsdcToken = _config.peerUsdcToken;
    }

    /**
     * @dev Initialize the implementation contract
     * @param _operator Operator address
     * @param _minFinalityThreshold Minimum finality threshold
     * @param _feePremiumBps Fee premium in basis points
     */
    function _initialize(
        address _operator,
        uint16 _minFinalityThreshold,
        uint16 _feePremiumBps
    ) internal {
        _setOperator(_operator);
        _setMinFinalityThreshold(_minFinalityThreshold);
        _setFeePremiumBps(_feePremiumBps);

        // Nonce starts at 1, so assume nonce 0 as processed.
        // NOTE: This will cause the deposit/withdraw to fail if the
        // strategy is not initialized properly (which is expected).
        nonceProcessed[0] = true;
    }

    /***************************************
                    Settings
    ****************************************/
    /**
     * @dev Set the operator address
     * @param _operator Operator address
     */
    function setOperator(address _operator) external onlyGovernor {
        _setOperator(_operator);
    }

    /**
     * @dev Set the operator address
     * @param _operator Operator address
     */
    function _setOperator(address _operator) internal {
        operator = _operator;
        emit OperatorChanged(_operator);
    }

    /**
     * @dev Set the minimum finality threshold at which
     *      the message is considered to be finalized to relay.
     *      Only accepts a value of 1000 (Safe, after 1 epoch) or
     *      2000 (Finalized, after 2 epochs).
     * @param _minFinalityThreshold Minimum finality threshold
     */
    function setMinFinalityThreshold(uint16 _minFinalityThreshold)
        external
        onlyGovernor
    {
        _setMinFinalityThreshold(_minFinalityThreshold);
    }

    /**
     * @dev Set the minimum finality threshold
     * @param _minFinalityThreshold Minimum finality threshold
     */
    function _setMinFinalityThreshold(uint16 _minFinalityThreshold) internal {
        // 1000 for fast transfer and 2000 for standard transfer
        require(
            _minFinalityThreshold == 1000 || _minFinalityThreshold == 2000,
            "Invalid threshold"
        );

        minFinalityThreshold = _minFinalityThreshold;
        emit CCTPMinFinalityThresholdSet(_minFinalityThreshold);
    }

    /**
     * @dev Set the fee premium in basis points.
     *      Cannot be higher than 30% (3000 basis points).
     * @param _feePremiumBps Fee premium in basis points
     */
    function setFeePremiumBps(uint16 _feePremiumBps) external onlyGovernor {
        _setFeePremiumBps(_feePremiumBps);
    }

    /**
     * @dev Set the fee premium in basis points
     *      Cannot be higher than 30% (3000 basis points).
     *      Ref: https://developers.circle.com/cctp/technical-guide#fees
     * @param _feePremiumBps Fee premium in basis points
     */
    function _setFeePremiumBps(uint16 _feePremiumBps) internal {
        require(_feePremiumBps <= 3000, "Fee premium too high"); // 30%

        feePremiumBps = _feePremiumBps;
        emit CCTPFeePremiumBpsSet(_feePremiumBps);
    }

    /***************************************
             CCTP message handling
    ****************************************/

    /**
     * @dev Handles a finalized CCTP message
     * @param sourceDomain Source domain of the message
     * @param sender Sender of the message
     * @param finalityThresholdExecuted Fidelity threshold executed
     * @param messageBody Message body
     */
    function handleReceiveFinalizedMessage(
        uint32 sourceDomain,
        bytes32 sender,
        uint32 finalityThresholdExecuted,
        bytes memory messageBody
    ) external override onlyCCTPMessageTransmitter returns (bool) {
        // Make sure the finality threshold at execution is at least 2000
        require(
            finalityThresholdExecuted >= 2000,
            "Finality threshold too low"
        );

        return _handleReceivedMessage(sourceDomain, sender, messageBody);
    }

    /**
     * @dev Handles an unfinalized but safe CCTP message
     * @param sourceDomain Source domain of the message
     * @param sender Sender of the message
     * @param finalityThresholdExecuted Fidelity threshold executed
     * @param messageBody Message body
     */
    function handleReceiveUnfinalizedMessage(
        uint32 sourceDomain,
        bytes32 sender,
        uint32 finalityThresholdExecuted,
        bytes memory messageBody
    ) external override onlyCCTPMessageTransmitter returns (bool) {
        // Make sure the contract is configured to handle unfinalized messages
        require(
            minFinalityThreshold == 1000,
            "Unfinalized messages are not supported"
        );
        // Make sure the finality threshold at execution is at least 1000
        require(
            finalityThresholdExecuted >= 1000,
            "Finality threshold too low"
        );

        return _handleReceivedMessage(sourceDomain, sender, messageBody);
    }

    /**
     * @dev Handles a CCTP message
     * @param sourceDomain Source domain of the message
     * @param sender Sender of the message
     * @param messageBody Message body
     */
    function _handleReceivedMessage(
        uint32 sourceDomain,
        bytes32 sender,
        bytes memory messageBody
    ) internal returns (bool) {
        require(sourceDomain == peerDomainID, "Unknown Source Domain");

        // Extract address from bytes32 (CCTP stores addresses as right-padded bytes32)
        address senderAddress = address(uint160(uint256(sender)));
        require(senderAddress == peerStrategy, "Unknown Sender");

        _onMessageReceived(messageBody);

        return true;
    }

    /**
     * @dev Sends tokens to the peer strategy using CCTP Token Messenger
     * @param tokenAmount Amount of tokens to send
     * @param hookData Hook data
     */
    function _sendTokens(uint256 tokenAmount, bytes memory hookData)
        internal
        virtual
    {
        // CCTP has a maximum transfer amount of 10M USDC per tx
        require(tokenAmount <= MAX_TRANSFER_AMOUNT, "Token amount too high");

        // Approve only what needs to be transferred
        IERC20(usdcToken).safeApprove(address(cctpTokenMessenger), tokenAmount);

        // Compute the max fee to be paid.
        // Ref: https://developers.circle.com/cctp/evm-smart-contracts#getminfeeamount
        // The right way to compute fees would be to use CCTP's getMinFeeAmount function.
        // The issue is that the getMinFeeAmount is not present on v2.0 contracts, but is on
        // v2.1. Some of CCTP's deployed contracts are v2.0, some are v2.1.
        // We will only be using standard transfers and fee on those is 0 for now. If they
        // ever start implementing fee for standard transfers or if we decide to use fast
        // trasnfer, we can use feePremiumBps as a workaround.
        uint256 maxFee = feePremiumBps > 0
            ? (tokenAmount * feePremiumBps) / 10000
            : 0;

        // Send tokens to the peer strategy using CCTP Token Messenger
        cctpTokenMessenger.depositForBurnWithHook(
            tokenAmount,
            peerDomainID,
            bytes32(uint256(uint160(peerStrategy))),
            address(usdcToken),
            bytes32(uint256(uint160(peerStrategy))),
            maxFee,
            uint32(minFinalityThreshold),
            hookData
        );

        emit TokensBridged(
            peerDomainID,
            peerStrategy,
            usdcToken,
            tokenAmount,
            maxFee,
            uint32(minFinalityThreshold),
            hookData
        );
    }

    /**
     * @dev Sends a message to the peer strategy using CCTP Message Transmitter
     * @param message Payload of the message to send
     */
    function _sendMessage(bytes memory message) internal virtual {
        cctpMessageTransmitter.sendMessage(
            peerDomainID,
            bytes32(uint256(uint160(peerStrategy))),
            bytes32(uint256(uint160(peerStrategy))),
            uint32(minFinalityThreshold),
            message
        );

        emit MessageTransmitted(
            peerDomainID,
            peerStrategy,
            uint32(minFinalityThreshold),
            message
        );
    }

    /**
     * @dev Receives a message from the peer strategy on the other chain,
     *      does some basic checks and relays it to the local MessageTransmitterV2.
     *      If the message is a burn message, it will also handle the hook data
     *      and call the _onTokenReceived function.
     * @param message Payload of the message to send
     * @param attestation Attestation of the message
     */
    function relay(bytes memory message, bytes memory attestation)
        external
        onlyOperator
    {
        (
            uint32 version,
            uint32 sourceDomainID,
            address sender,
            address recipient,
            bytes memory messageBody
        ) = message.decodeMessageHeader();

        // Ensure that it's a CCTP message
        require(
            version == CrossChainStrategyHelper.CCTP_MESSAGE_VERSION,
            "Invalid CCTP message version"
        );

        // Ensure that the source domain is the peer domain
        require(sourceDomainID == peerDomainID, "Unknown Source Domain");

        // Ensure message body version
        version = messageBody.extractUint32(BURN_MESSAGE_V2_VERSION_INDEX);

        // NOTE: There's a possibility that the CCTP Token Messenger might
        // send other types of messages in future, not just the burn message.
        // If it ever comes to that, this shouldn't cause us any problems
        // because it has to still go through the followign checks:
        // - version check
        // - message body length check
        // - sender and recipient (which should be in the same slots and same as address(this))
        // - hook data handling (which will revert even if all the above checks pass)
        bool isBurnMessageV1 = sender == address(cctpTokenMessenger);

        if (isBurnMessageV1) {
            // Handle burn message
            require(
                version == 1 &&
                    messageBody.length >= BURN_MESSAGE_V2_HOOK_DATA_INDEX,
                "Invalid burn message"
            );

            // Ensure the burn token is USDC
            address burnToken = messageBody.extractAddress(
                BURN_MESSAGE_V2_BURN_TOKEN_INDEX
            );
            require(burnToken == peerUsdcToken, "Invalid burn token");

            // Address of caller of depositForBurn (or depositForBurnWithCaller) on source domain
            sender = messageBody.extractAddress(
                BURN_MESSAGE_V2_MESSAGE_SENDER_INDEX
            );

            recipient = messageBody.extractAddress(
                BURN_MESSAGE_V2_RECIPIENT_INDEX
            );
        } else {
            // We handle only Burn message or our custom messagee
            require(
                version == CrossChainStrategyHelper.ORIGIN_MESSAGE_VERSION,
                "Unsupported message version"
            );
        }

        // Ensure the recipient is this contract
        // Both sender and recipient should be deployed to same address on both chains.
        require(address(this) == recipient, "Unexpected recipient address");
        require(sender == peerStrategy, "Incorrect sender/recipient address");

        // Relay the message
        // This step also mints USDC and transfers it to the recipient wallet
        bool relaySuccess = cctpMessageTransmitter.receiveMessage(
            message,
            attestation
        );
        require(relaySuccess, "Receive message failed");

        if (isBurnMessageV1) {
            // Extract the hook data from the message body
            bytes memory hookData = messageBody.extractSlice(
                BURN_MESSAGE_V2_HOOK_DATA_INDEX,
                messageBody.length
            );

            // Extract the token amount from the message body
            uint256 tokenAmount = messageBody.extractUint256(
                BURN_MESSAGE_V2_AMOUNT_INDEX
            );

            // Extract the fee executed from the message body
            uint256 feeExecuted = messageBody.extractUint256(
                BURN_MESSAGE_V2_FEE_EXECUTED_INDEX
            );

            // Call the _onTokenReceived function
            _onTokenReceived(tokenAmount - feeExecuted, feeExecuted, hookData);
        }
    }

    /***************************************
                  Message utils
    ****************************************/

    /***************************************
                  Nonce Handling
    ****************************************/
    /**
     * @dev Checks if the last known transfer is pending.
     *      Nonce starts at 1, so 0 is disregarded.
     * @return True if a transfer is pending, false otherwise
     */
    function isTransferPending() public view returns (bool) {
        return !nonceProcessed[lastTransferNonce];
    }

    /**
     * @dev Checks if a given nonce is processed.
     *      Nonce starts at 1, so 0 is disregarded.
     * @param nonce Nonce to check
     * @return True if the nonce is processed, false otherwise
     */
    function isNonceProcessed(uint64 nonce) public view returns (bool) {
        return nonceProcessed[nonce];
    }

    /**
     * @dev Marks a given nonce as processed.
     *      Can only mark nonce as processed once. New nonce should
     *      always be greater than the last known nonce. Also updates
     *      the last known nonce.
     * @param nonce Nonce to mark as processed
     */
    function _markNonceAsProcessed(uint64 nonce) internal {
        uint64 lastNonce = lastTransferNonce;

        // Can only mark latest nonce as processed
        // Master strategy when receiving a message from the remote strategy
        // will have lastNone == nonce, as the nonce is increase at the start
        // of deposit / withdrawal flow.
        // Remote strategy will have lastNonce < nonce, as a new nonce initiated
        // from master will be greater than the last one.
        require(nonce >= lastNonce, "Nonce too low");
        // Can only mark nonce as processed once
        require(!nonceProcessed[nonce], "Nonce already processed");

        nonceProcessed[nonce] = true;
        emit NonceProcessed(nonce);

        if (nonce != lastNonce) {
            // Update last known nonce
            lastTransferNonce = nonce;
            emit LastTransferNonceUpdated(nonce);
        }
    }

    /**
     * @dev Gets the next nonce to use.
     *      Nonce starts at 1, so 0 is disregarded.
     *      Reverts if last nonce hasn't been processed yet.
     * @return Next nonce
     */
    function _getNextNonce() internal returns (uint64) {
        uint64 nonce = lastTransferNonce;

        require(nonceProcessed[nonce], "Pending token transfer");

        nonce = nonce + 1;
        lastTransferNonce = nonce;
        emit LastTransferNonceUpdated(nonce);

        return nonce;
    }

    /***************************************
             Inheritence overrides
    ****************************************/

    /**
     * @dev Called when the USDC is received from the CCTP
     * @param tokenAmount The actual amount of USDC received (amount sent - fee executed)
     * @param feeExecuted The fee executed
     * @param payload The payload of the message (hook data)
     */
    function _onTokenReceived(
        uint256 tokenAmount,
        uint256 feeExecuted,
        bytes memory payload
    ) internal virtual;

    /**
     * @dev Called when the message is received
     * @param payload The payload of the message
     */
    function _onMessageReceived(bytes memory payload) internal virtual;
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { IBasicToken } from "../interfaces/IBasicToken.sol";

library Helpers {
    /**
     * @notice Fetch the `symbol()` from an ERC20 token
     * @dev Grabs the `symbol()` from a contract
     * @param _token Address of the ERC20 token
     * @return string Symbol of the ERC20 token
     */
    function getSymbol(address _token) internal view returns (string memory) {
        string memory symbol = IBasicToken(_token).symbol();
        return symbol;
    }

    /**
     * @notice Fetch the `decimals()` from an ERC20 token
     * @dev Grabs the `decimals()` from a contract and fails if
     *      the decimal value does not live within a certain range
     * @param _token Address of the ERC20 token
     * @return uint256 Decimals of the ERC20 token
     */
    function getDecimals(address _token) internal view returns (uint256) {
        uint256 decimals = IBasicToken(_token).decimals();
        require(
            decimals >= 4 && decimals <= 18,
            "Token must have sufficient decimal places"
        );

        return decimals;
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title CrossChainStrategyHelper
 * @author Origin Protocol Inc
 * @dev This library is used to encode and decode the messages for the cross-chain strategy.
 *      It is used to ensure that the messages are valid and to get the message version and type.
 */

import { BytesHelper } from "../../utils/BytesHelper.sol";

library CrossChainStrategyHelper {
    using BytesHelper for bytes;

    uint32 public constant DEPOSIT_MESSAGE = 1;
    uint32 public constant WITHDRAW_MESSAGE = 2;
    uint32 public constant BALANCE_CHECK_MESSAGE = 3;

    uint32 public constant CCTP_MESSAGE_VERSION = 1;
    uint32 public constant ORIGIN_MESSAGE_VERSION = 1010;

    // CCTP Message Header fields
    // Ref: https://developers.circle.com/cctp/technical-guide#message-header
    uint8 private constant VERSION_INDEX = 0;
    uint8 private constant SOURCE_DOMAIN_INDEX = 4;
    uint8 private constant SENDER_INDEX = 44;
    uint8 private constant RECIPIENT_INDEX = 76;
    uint8 private constant MESSAGE_BODY_INDEX = 148;

    /**
     * @dev Get the message version from the message.
     *      It should always be 4 bytes long,
     *      starting from the 0th index.
     * @param message The message to get the version from
     * @return The message version
     */
    function getMessageVersion(bytes memory message)
        internal
        pure
        returns (uint32)
    {
        // uint32 bytes 0 to 4 is Origin message version
        // uint32 bytes 4 to 8 is Message type
        return message.extractUint32(0);
    }

    /**
     * @dev Get the message type from the message.
     *      It should always be 4 bytes long,
     *      starting from the 4th index.
     * @param message The message to get the type from
     * @return The message type
     */
    function getMessageType(bytes memory message)
        internal
        pure
        returns (uint32)
    {
        // uint32 bytes 0 to 4 is Origin message version
        // uint32 bytes 4 to 8 is Message type
        return message.extractUint32(4);
    }

    /**
     * @dev Verify the message version and type.
     *      The message version should be the same as the Origin message version,
     *      and the message type should be the same as the expected message type.
     * @param _message The message to verify
     * @param _type The expected message type
     */
    function verifyMessageVersionAndType(bytes memory _message, uint32 _type)
        internal
        pure
    {
        require(
            getMessageVersion(_message) == ORIGIN_MESSAGE_VERSION,
            "Invalid Origin Message Version"
        );
        require(getMessageType(_message) == _type, "Invalid Message type");
    }

    /**
     * @dev Get the message payload from the message.
     *      The payload starts at the 8th byte.
     * @param message The message to get the payload from
     * @return The message payload
     */
    function getMessagePayload(bytes memory message)
        internal
        pure
        returns (bytes memory)
    {
        // uint32 bytes 0 to 4 is Origin message version
        // uint32 bytes 4 to 8 is Message type
        // Payload starts at byte 8
        return message.extractSlice(8, message.length);
    }

    /**
     * @dev Encode the deposit message.
     *      The message version and type are always encoded in the message.
     * @param nonce The nonce of the deposit
     * @param depositAmount The amount of the deposit
     * @return The encoded deposit message
     */
    function encodeDepositMessage(uint64 nonce, uint256 depositAmount)
        internal
        pure
        returns (bytes memory)
    {
        return
            abi.encodePacked(
                ORIGIN_MESSAGE_VERSION,
                DEPOSIT_MESSAGE,
                abi.encode(nonce, depositAmount)
            );
    }

    /**
     * @dev Decode the deposit message.
     *      The message version and type are verified in the message.
     * @param message The message to decode
     * @return The nonce and the amount of the deposit
     */
    function decodeDepositMessage(bytes memory message)
        internal
        pure
        returns (uint64, uint256)
    {
        verifyMessageVersionAndType(message, DEPOSIT_MESSAGE);

        (uint64 nonce, uint256 depositAmount) = abi.decode(
            getMessagePayload(message),
            (uint64, uint256)
        );
        return (nonce, depositAmount);
    }

    /**
     * @dev Encode the withdrawal message.
     *      The message version and type are always encoded in the message.
     * @param nonce The nonce of the withdrawal
     * @param withdrawAmount The amount of the withdrawal
     * @return The encoded withdrawal message
     */
    function encodeWithdrawMessage(uint64 nonce, uint256 withdrawAmount)
        internal
        pure
        returns (bytes memory)
    {
        return
            abi.encodePacked(
                ORIGIN_MESSAGE_VERSION,
                WITHDRAW_MESSAGE,
                abi.encode(nonce, withdrawAmount)
            );
    }

    /**
     * @dev Decode the withdrawal message.
     *      The message version and type are verified in the message.
     * @param message The message to decode
     * @return The nonce and the amount of the withdrawal
     */
    function decodeWithdrawMessage(bytes memory message)
        internal
        pure
        returns (uint64, uint256)
    {
        verifyMessageVersionAndType(message, WITHDRAW_MESSAGE);

        (uint64 nonce, uint256 withdrawAmount) = abi.decode(
            getMessagePayload(message),
            (uint64, uint256)
        );
        return (nonce, withdrawAmount);
    }

    /**
     * @dev Encode the balance check message.
     *      The message version and type are always encoded in the message.
     * @param nonce The nonce of the balance check
     * @param balance The balance to check
     * @param transferConfirmation Indicates if the message is a transfer confirmation. This is true
     *                            when the message is a result of a deposit or a withdrawal.
     * @return The encoded balance check message
     */
    function encodeBalanceCheckMessage(
        uint64 nonce,
        uint256 balance,
        bool transferConfirmation,
        uint256 timestamp
    ) internal pure returns (bytes memory) {
        return
            abi.encodePacked(
                ORIGIN_MESSAGE_VERSION,
                BALANCE_CHECK_MESSAGE,
                abi.encode(nonce, balance, transferConfirmation, timestamp)
            );
    }

    /**
     * @dev Decode the balance check message.
     *      The message version and type are verified in the message.
     * @param message The message to decode
     * @return The nonce, the balance and indicates if the message is a transfer confirmation
     */
    function decodeBalanceCheckMessage(bytes memory message)
        internal
        pure
        returns (
            uint64,
            uint256,
            bool,
            uint256
        )
    {
        verifyMessageVersionAndType(message, BALANCE_CHECK_MESSAGE);

        (
            uint64 nonce,
            uint256 balance,
            bool transferConfirmation,
            uint256 timestamp
        ) = abi.decode(
                getMessagePayload(message),
                (uint64, uint256, bool, uint256)
            );
        return (nonce, balance, transferConfirmation, timestamp);
    }

    /**
     * @dev Decode the CCTP message header
     * @param message Message to decode
     * @return version Version of the message
     * @return sourceDomainID Source domain ID
     * @return sender Sender of the message
     * @return recipient Recipient of the message
     * @return messageBody Message body
     */
    function decodeMessageHeader(bytes memory message)
        internal
        pure
        returns (
            uint32 version,
            uint32 sourceDomainID,
            address sender,
            address recipient,
            bytes memory messageBody
        )
    {
        version = message.extractUint32(VERSION_INDEX);
        sourceDomainID = message.extractUint32(SOURCE_DOMAIN_INDEX);
        // Address of MessageTransmitterV2 caller on source domain
        sender = message.extractAddress(SENDER_INDEX);
        // Address to handle message body on destination domain
        recipient = message.extractAddress(RECIPIENT_INDEX);
        messageBody = message.extractSlice(MESSAGE_BODY_INDEX, message.length);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

uint256 constant UINT32_LENGTH = 4;
uint256 constant UINT64_LENGTH = 8;
uint256 constant UINT256_LENGTH = 32;
// Address is 20 bytes, but we expect the data to be padded with 0s to 32 bytes
uint256 constant ADDRESS_LENGTH = 32;

library BytesHelper {
    /**
     * @dev Extract a slice from bytes memory
     * @param data The bytes memory to slice
     * @param start The start index (inclusive)
     * @param end The end index (exclusive)
     * @return result A new bytes memory containing the slice
     */
    function extractSlice(
        bytes memory data,
        uint256 start,
        uint256 end
    ) internal pure returns (bytes memory) {
        require(end >= start, "Invalid slice range");
        require(end <= data.length, "Slice end exceeds data length");

        uint256 length = end - start;
        bytes memory result = new bytes(length);

        // Simple byte-by-byte copy
        for (uint256 i = 0; i < length; i++) {
            result[i] = data[start + i];
        }

        return result;
    }

    /**
     * @dev Decode a uint32 from a bytes memory
     * @param data The bytes memory to decode
     * @return uint32 The decoded uint32
     */
    function decodeUint32(bytes memory data) internal pure returns (uint32) {
        require(data.length == 4, "Invalid data length");
        return uint32(uint256(bytes32(data)) >> 224);
    }

    /**
     * @dev Extract a uint32 from a bytes memory
     * @param data The bytes memory to extract from
     * @param start The start index (inclusive)
     * @return uint32 The extracted uint32
     */
    function extractUint32(bytes memory data, uint256 start)
        internal
        pure
        returns (uint32)
    {
        return decodeUint32(extractSlice(data, start, start + UINT32_LENGTH));
    }

    /**
     * @dev Decode an address from a bytes memory.
     *      Expects the data to be padded with 0s to 32 bytes.
     * @param data The bytes memory to decode
     * @return address The decoded address
     */
    function decodeAddress(bytes memory data) internal pure returns (address) {
        // We expect the data to be padded with 0s, so length is 32 not 20
        require(data.length == 32, "Invalid data length");
        return abi.decode(data, (address));
    }

    /**
     * @dev Extract an address from a bytes memory
     * @param data The bytes memory to extract from
     * @param start The start index (inclusive)
     * @return address The extracted address
     */
    function extractAddress(bytes memory data, uint256 start)
        internal
        pure
        returns (address)
    {
        return decodeAddress(extractSlice(data, start, start + ADDRESS_LENGTH));
    }

    /**
     * @dev Decode a uint256 from a bytes memory
     * @param data The bytes memory to decode
     * @return uint256 The decoded uint256
     */
    function decodeUint256(bytes memory data) internal pure returns (uint256) {
        require(data.length == 32, "Invalid data length");
        return abi.decode(data, (uint256));
    }

    /**
     * @dev Extract a uint256 from a bytes memory
     * @param data The bytes memory to extract from
     * @param start The start index (inclusive)
     * @return uint256 The extracted uint256
     */
    function extractUint256(bytes memory data, uint256 start)
        internal
        pure
        returns (uint256)
    {
        return decodeUint256(extractSlice(data, start, start + UINT256_LENGTH));
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title Base for contracts that are managed by the Origin Protocol's Governor.
 * @dev Copy of the openzeppelin Ownable.sol contract with nomenclature change
 *      from owner to governor and renounce methods removed. Does not use
 *      Context.sol like Ownable.sol does for simplification.
 * @author Origin Protocol Inc
 */
abstract contract Governable {
    // Storage position of the owner and pendingOwner of the contract
    // keccak256("OUSD.governor");
    bytes32 private constant governorPosition =
        0x7bea13895fa79d2831e0a9e28edede30099005a50d652d8957cf8a607ee6ca4a;

    // keccak256("OUSD.pending.governor");
    bytes32 private constant pendingGovernorPosition =
        0x44c4d30b2eaad5130ad70c3ba6972730566f3e6359ab83e800d905c61b1c51db;

    // keccak256("OUSD.reentry.status");
    bytes32 private constant reentryStatusPosition =
        0x53bf423e48ed90e97d02ab0ebab13b2a235a6bfbe9c321847d5c175333ac4535;

    // See OpenZeppelin ReentrancyGuard implementation
    uint256 constant _NOT_ENTERED = 1;
    uint256 constant _ENTERED = 2;

    event PendingGovernorshipTransfer(
        address indexed previousGovernor,
        address indexed newGovernor
    );

    event GovernorshipTransferred(
        address indexed previousGovernor,
        address indexed newGovernor
    );

    /**
     * @notice Returns the address of the current Governor.
     */
    function governor() public view returns (address) {
        return _governor();
    }

    /**
     * @dev Returns the address of the current Governor.
     */
    function _governor() internal view returns (address governorOut) {
        bytes32 position = governorPosition;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            governorOut := sload(position)
        }
    }

    /**
     * @dev Returns the address of the pending Governor.
     */
    function _pendingGovernor()
        internal
        view
        returns (address pendingGovernor)
    {
        bytes32 position = pendingGovernorPosition;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            pendingGovernor := sload(position)
        }
    }

    /**
     * @dev Throws if called by any account other than the Governor.
     */
    modifier onlyGovernor() {
        require(isGovernor(), "Caller is not the Governor");
        _;
    }

    /**
     * @notice Returns true if the caller is the current Governor.
     */
    function isGovernor() public view returns (bool) {
        return msg.sender == _governor();
    }

    function _setGovernor(address newGovernor) internal {
        emit GovernorshipTransferred(_governor(), newGovernor);

        bytes32 position = governorPosition;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(position, newGovernor)
        }
    }

    /**
     * @dev Prevents a contract from calling itself, directly or indirectly.
     * Calling a `nonReentrant` function from another `nonReentrant`
     * function is not supported. It is possible to prevent this from happening
     * by making the `nonReentrant` function external, and make it call a
     * `private` function that does the actual work.
     */
    modifier nonReentrant() {
        bytes32 position = reentryStatusPosition;
        uint256 _reentry_status;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _reentry_status := sload(position)
        }

        // On the first call to nonReentrant, _notEntered will be true
        require(_reentry_status != _ENTERED, "Reentrant call");

        // Any calls to nonReentrant after this point will fail
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(position, _ENTERED)
        }

        _;

        // By storing the original value once again, a refund is triggered (see
        // https://eips.ethereum.org/EIPS/eip-2200)
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(position, _NOT_ENTERED)
        }
    }

    function _setPendingGovernor(address newGovernor) internal {
        bytes32 position = pendingGovernorPosition;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(position, newGovernor)
        }
    }

    /**
     * @notice Transfers Governance of the contract to a new account (`newGovernor`).
     * Can only be called by the current Governor. Must be claimed for this to complete
     * @param _newGovernor Address of the new Governor
     */
    function transferGovernance(address _newGovernor) external onlyGovernor {
        _setPendingGovernor(_newGovernor);
        emit PendingGovernorshipTransfer(_governor(), _newGovernor);
    }

    /**
     * @notice Claim Governance of the contract to a new account (`newGovernor`).
     * Can only be called by the new Governor.
     */
    function claimGovernance() external {
        require(
            msg.sender == _pendingGovernor(),
            "Only the pending Governor can complete the claim"
        );
        _changeGovernor(msg.sender);
    }

    /**
     * @dev Change Governance of the contract to a new account (`newGovernor`).
     * @param _newGovernor Address of the new Governor
     */
    function _changeGovernor(address _newGovernor) internal {
        require(_newGovernor != address(0), "New Governor is address(0)");
        _setGovernor(_newGovernor);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

interface ICCTPTokenMessenger {
    function depositForBurn(
        uint256 amount,
        uint32 destinationDomain,
        bytes32 mintRecipient,
        address burnToken,
        bytes32 destinationCaller,
        uint256 maxFee,
        uint32 minFinalityThreshold
    ) external;

    function depositForBurnWithHook(
        uint256 amount,
        uint32 destinationDomain,
        bytes32 mintRecipient,
        address burnToken,
        bytes32 destinationCaller,
        uint256 maxFee,
        uint32 minFinalityThreshold,
        bytes memory hookData
    ) external;

    function getMinFeeAmount(uint256 amount) external view returns (uint256);
}

interface ICCTPMessageTransmitter {
    function sendMessage(
        uint32 destinationDomain,
        bytes32 recipient,
        bytes32 destinationCaller,
        uint32 minFinalityThreshold,
        bytes memory messageBody
    ) external;

    function receiveMessage(bytes calldata message, bytes calldata attestation)
        external
        returns (bool);
}

interface IMessageHandlerV2 {
    /**
     * @notice Handles an incoming finalized message from an IReceiverV2
     * @dev Finalized messages have finality threshold values greater than or equal to 2000
     * @param sourceDomain The source domain of the message
     * @param sender The sender of the message
     * @param finalityThresholdExecuted the finality threshold at which the message was attested to
     * @param messageBody The raw bytes of the message body
     * @return success True, if successful; false, if not.
     */
    function handleReceiveFinalizedMessage(
        uint32 sourceDomain,
        bytes32 sender,
        uint32 finalityThresholdExecuted,
        bytes calldata messageBody
    ) external returns (bool);

    /**
     * @notice Handles an incoming unfinalized message from an IReceiverV2
     * @dev Unfinalized messages have finality threshold values less than 2000
     * @param sourceDomain The source domain of the message
     * @param sender The sender of the message
     * @param finalityThresholdExecuted The finality threshold at which the message was attested to
     * @param messageBody The raw bytes of the message body
     * @return success True, if successful; false, if not.
     */
    function handleReceiveUnfinalizedMessage(
        uint32 sourceDomain,
        bytes32 sender,
        uint32 finalityThresholdExecuted,
        bytes calldata messageBody
    ) external returns (bool);
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { IBasicToken } from "../interfaces/IBasicToken.sol";

library Helpers {
    /**
     * @notice Fetch the `symbol()` from an ERC20 token
     * @dev Grabs the `symbol()` from a contract
     * @param _token Address of the ERC20 token
     * @return string Symbol of the ERC20 token
     */
    function getSymbol(address _token) internal view returns (string memory) {
        string memory symbol = IBasicToken(_token).symbol();
        return symbol;
    }

    /**
     * @notice Fetch the `decimals()` from an ERC20 token
     * @dev Grabs the `decimals()` from a contract and fails if
     *      the decimal value does not live within a certain range
     * @param _token Address of the ERC20 token
     * @return uint256 Decimals of the ERC20 token
     */
    function getDecimals(address _token) internal view returns (uint256) {
        uint256 decimals = IBasicToken(_token).decimals();
        require(
            decimals >= 4 && decimals <= 18,
            "Token must have sufficient decimal places"
        );

        return decimals;
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.0;

/**
 * @dev Interface of the ERC20 standard as defined in the EIP.
 */
interface IERC20 {
    /**
     * @dev Returns the amount of tokens in existence.
     */
    function totalSupply() external view returns (uint256);

    /**
     * @dev Returns the amount of tokens owned by `account`.
     */
    function balanceOf(address account) external view returns (uint256);

    /**
     * @dev Moves `amount` tokens from the caller's account to `recipient`.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * Emits a {Transfer} event.
     */
    function transfer(address recipient, uint256 amount) external returns (bool);

    /**
     * @dev Returns the remaining number of tokens that `spender` will be
     * allowed to spend on behalf of `owner` through {transferFrom}. This is
     * zero by default.
     *
     * This value changes when {approve} or {transferFrom} are called.
     */
    function allowance(address owner, address spender) external view returns (uint256);

    /**
     * @dev Sets `amount` as the allowance of `spender` over the caller's tokens.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * IMPORTANT: Beware that changing an allowance with this method brings the risk
     * that someone may use both the old and the new allowance by unfortunate
     * transaction ordering. One possible solution to mitigate this race
     * condition is to first reduce the spender's allowance to 0 and set the
     * desired value afterwards:
     * https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
     *
     * Emits an {Approval} event.
     */
    function approve(address spender, uint256 amount) external returns (bool);

    /**
     * @dev Moves `amount` tokens from `sender` to `recipient` using the
     * allowance mechanism. `amount` is then deducted from the caller's
     * allowance.
     *
     * Returns a boolean value indicating whether the operation succeeded.
     *
     * Emits a {Transfer} event.
     */
    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);

    /**
     * @dev Emitted when `value` tokens are moved from one account (`from`) to
     * another (`to`).
     *
     * Note that `value` may be zero.
     */
    event Transfer(address indexed from, address indexed to, uint256 value);

    /**
     * @dev Emitted when the allowance of a `spender` for an `owner` is set by
     * a call to {approve}. `value` is the new allowance.
     */
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IBasicToken {
    function symbol() external view returns (string memory);

    function decimals() external view returns (uint8);
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 
// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { WOETH } from "./WOETH.sol";
import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/**
 * @title OETH Token Contract
 * @author Origin Protocol Inc
 */

contract WOETHBase is WOETH {
    constructor(ERC20 underlying_) WOETH(underlying_) {}

    function name() public view virtual override returns (string memory) {
        return "Wrapped Super OETH";
    }

    function symbol() public view virtual override returns (string memory) {
        return "wsuperOETHb";
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { IERC4626 } from "../../../lib/openzeppelin/interfaces/IERC4626.sol";

interface IVaultV2 is IERC4626 {
    function liquidityAdapter() external view returns (address);
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { WOETH } from "./WOETH.sol";

/**
 * @title Wrapped OUSD Token Contract
 * @author Origin Protocol Inc
 */
contract WrappedOusd is WOETH {
    constructor(ERC20 underlying_) WOETH(underlying_) {}

    function name()
        public
        view
        virtual
        override(WOETH)
        returns (string memory)
    {
        return "Wrapped OUSD";
    }

    function symbol()
        public
        view
        virtual
        override(WOETH)
        returns (string memory)
    {
        return "WOUSD";
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title OUSD Yearn V3 Master Strategy - the Mainnet part
 * @author Origin Protocol Inc
 *
 * @dev This strategy can only perform 1 deposit or withdrawal at a time. For that
 *      reason it shouldn't be configured as an asset default strategy.
 */

import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { IERC20, InitializableAbstractStrategy } from "../../utils/InitializableAbstractStrategy.sol";
import { AbstractCCTPIntegrator } from "./AbstractCCTPIntegrator.sol";
import { CrossChainStrategyHelper } from "./CrossChainStrategyHelper.sol";

contract CrossChainMasterStrategy is
    AbstractCCTPIntegrator,
    InitializableAbstractStrategy
{
    using SafeERC20 for IERC20;
    using CrossChainStrategyHelper for bytes;

    /**
     * @notice Remote strategy balance
     * @dev    The remote balance is cached and might not reflect the actual
     *         real-time balance of the remote strategy.
     */
    uint256 public remoteStrategyBalance;

    /// @notice Amount that's bridged due to a pending Deposit process
    ///         but with no acknowledgement from the remote strategy yet
    uint256 public pendingAmount;

    uint256 internal constant MAX_BALANCE_CHECK_AGE = 1 days;

    event RemoteStrategyBalanceUpdated(uint256 balance);
    event WithdrawRequested(address indexed asset, uint256 amount);
    event WithdrawAllSkipped();
    event BalanceCheckIgnored(uint64 nonce, uint256 timestamp, bool isTooOld);

    /**
     * @param _stratConfig The platform and OToken vault addresses
     */
    constructor(
        BaseStrategyConfig memory _stratConfig,
        CCTPIntegrationConfig memory _cctpConfig
    )
        InitializableAbstractStrategy(_stratConfig)
        AbstractCCTPIntegrator(_cctpConfig)
    {
        require(
            _stratConfig.platformAddress == address(0),
            "Invalid platform address"
        );
        require(
            _stratConfig.vaultAddress != address(0),
            "Invalid Vault address"
        );
    }

    /**
     * @dev Initialize the strategy implementation
     * @param _operator Address of the operator
     * @param _minFinalityThreshold Minimum finality threshold
     * @param _feePremiumBps Fee premium in basis points
     */
    function initialize(
        address _operator,
        uint16 _minFinalityThreshold,
        uint16 _feePremiumBps
    ) external virtual onlyGovernor initializer {
        _initialize(_operator, _minFinalityThreshold, _feePremiumBps);

        address[] memory rewardTokens = new address[](0);
        address[] memory assets = new address[](0);
        address[] memory pTokens = new address[](0);

        InitializableAbstractStrategy._initialize(
            rewardTokens,
            assets,
            pTokens
        );
    }

    /// @inheritdoc InitializableAbstractStrategy
    function deposit(address _asset, uint256 _amount)
        external
        override
        onlyVault
        nonReentrant
    {
        _deposit(_asset, _amount);
    }

    /// @inheritdoc InitializableAbstractStrategy
    function depositAll() external override onlyVault nonReentrant {
        uint256 balance = IERC20(usdcToken).balanceOf(address(this));
        // Deposit if balance is greater than 1 USDC
        if (balance >= MIN_TRANSFER_AMOUNT) {
            _deposit(usdcToken, balance);
        }
    }

    /// @inheritdoc InitializableAbstractStrategy
    function withdraw(
        address _recipient,
        address _asset,
        uint256 _amount
    ) external override onlyVault nonReentrant {
        require(_recipient == vaultAddress, "Only Vault can withdraw");
        _withdraw(_asset, _amount);
    }

    /// @inheritdoc InitializableAbstractStrategy
    function withdrawAll() external override onlyVaultOrGovernor nonReentrant {
        if (isTransferPending()) {
            // Do nothing if there is a pending transfer
            // Note: We never want withdrawAll to fail, so
            // emit an event to indicate that the withdrawal was skipped
            emit WithdrawAllSkipped();
            return;
        }

        // Withdraw everything in Remote strategy
        uint256 _remoteBalance = remoteStrategyBalance;
        if (_remoteBalance < MIN_TRANSFER_AMOUNT) {
            // Do nothing if there is less than 1 USDC in the Remote strategy
            return;
        }

        _withdraw(
            usdcToken,
            _remoteBalance > MAX_TRANSFER_AMOUNT
                ? MAX_TRANSFER_AMOUNT
                : _remoteBalance
        );
    }

    /**
     * @notice Check the balance of the strategy that includes
     *          the balance of the asset on this contract,
     *          the amount of the asset being bridged,
     *          and the balance reported by the Remote strategy.
     * @param _asset Address of the asset to check
     * @return balance Total balance of the asset
     */
    function checkBalance(address _asset)
        public
        view
        override
        returns (uint256 balance)
    {
        require(_asset == usdcToken, "Unsupported asset");

        // USDC balance on this contract
        // + USDC being bridged
        // + USDC cached in the corresponding Remote part of this contract
        return
            IERC20(usdcToken).balanceOf(address(this)) +
            pendingAmount +
            remoteStrategyBalance;
    }

    /// @inheritdoc InitializableAbstractStrategy
    function supportsAsset(address _asset) public view override returns (bool) {
        return _asset == usdcToken;
    }

    /// @inheritdoc InitializableAbstractStrategy
    function safeApproveAllTokens()
        external
        override
        onlyGovernor
        nonReentrant
    {}

    /// @inheritdoc InitializableAbstractStrategy
    function _abstractSetPToken(address, address) internal override {}

    /// @inheritdoc InitializableAbstractStrategy
    function collectRewardTokens()
        external
        override
        onlyHarvesterOrStrategist
        nonReentrant
    {}

    /// @inheritdoc AbstractCCTPIntegrator
    function _onMessageReceived(bytes memory payload) internal override {
        if (
            payload.getMessageType() ==
            CrossChainStrategyHelper.BALANCE_CHECK_MESSAGE
        ) {
            // Received when Remote strategy checks the balance
            _processBalanceCheckMessage(payload);
            return;
        }

        revert("Unknown message type");
    }

    /// @inheritdoc AbstractCCTPIntegrator
    function _onTokenReceived(
        uint256 tokenAmount,
        // solhint-disable-next-line no-unused-vars
        uint256 feeExecuted,
        bytes memory payload
    ) internal override {
        uint64 _nonce = lastTransferNonce;

        // Should be expecting an acknowledgement
        require(!isNonceProcessed(_nonce), "Nonce already processed");

        // Now relay to the regular flow
        // NOTE: Calling _onMessageReceived would mean that we are bypassing a
        // few checks that the regular flow does (like sourceDomainID check
        // and sender check in `handleReceiveFinalizedMessage`). However,
        // CCTPMessageRelayer relays the message first (which will go through
        // all the checks) and not update balance and then finally calls this
        // `_onTokenReceived` which will update the balance.
        // So, if any of the checks fail during the first no-balance-update flow,
        // this won't happen either, since the tx would revert.
        _onMessageReceived(payload);

        // Send any tokens in the contract to the Vault
        uint256 usdcBalance = IERC20(usdcToken).balanceOf(address(this));
        // Should always have enough tokens
        require(usdcBalance >= tokenAmount, "Insufficient balance");
        // Transfer all tokens to the Vault to not leave any dust
        IERC20(usdcToken).safeTransfer(vaultAddress, usdcBalance);

        // Emit withdrawal amount
        emit Withdrawal(usdcToken, usdcToken, usdcBalance);
    }

    /**
     * @dev Bridge and deposit asset into the remote strategy
     * @param _asset Address of the asset to deposit
     * @param depositAmount Amount of the asset to deposit
     */
    function _deposit(address _asset, uint256 depositAmount) internal virtual {
        require(_asset == usdcToken, "Unsupported asset");
        require(pendingAmount == 0, "Unexpected pending amount");
        // Deposit at least 1 USDC
        require(
            depositAmount >= MIN_TRANSFER_AMOUNT,
            "Deposit amount too small"
        );
        require(
            depositAmount <= MAX_TRANSFER_AMOUNT,
            "Deposit amount too high"
        );

        // Get the next nonce
        // Note: reverts if a transfer is pending
        uint64 nonce = _getNextNonce();

        // Set pending amount
        pendingAmount = depositAmount;

        // Build deposit message payload
        bytes memory message = CrossChainStrategyHelper.encodeDepositMessage(
            nonce,
            depositAmount
        );

        // Send deposit message to the remote strategy
        _sendTokens(depositAmount, message);

        // Emit deposit event
        emit Deposit(_asset, _asset, depositAmount);
    }

    /**
     * @dev Send a withdraw request to the remote strategy
     * @param _asset Address of the asset to withdraw
     * @param _amount Amount of the asset to withdraw
     */
    function _withdraw(address _asset, uint256 _amount) internal virtual {
        require(_asset == usdcToken, "Unsupported asset");
        // Withdraw at least 1 USDC
        require(_amount >= MIN_TRANSFER_AMOUNT, "Withdraw amount too small");
        require(
            _amount <= remoteStrategyBalance,
            "Withdraw amount exceeds remote strategy balance"
        );
        require(
            _amount <= MAX_TRANSFER_AMOUNT,
            "Withdraw amount exceeds max transfer amount"
        );

        // Get the next nonce
        // Note: reverts if a transfer is pending
        uint64 nonce = _getNextNonce();

        // Build and send withdrawal message with payload
        bytes memory message = CrossChainStrategyHelper.encodeWithdrawMessage(
            nonce,
            _amount
        );
        _sendMessage(message);

        // Emit WithdrawRequested event here,
        // Withdraw will be emitted in _onTokenReceived
        emit WithdrawRequested(usdcToken, _amount);
    }

    /**
     * @dev Process balance check:
     *  - Confirms a deposit to the remote strategy
     *  - Skips balance update if there's a pending withdrawal
     *  - Updates the remote strategy balance
     * @param message The message containing the nonce and balance
     */
    function _processBalanceCheckMessage(bytes memory message)
        internal
        virtual
    {
        // Decode the message
        // When transferConfirmation is true, it means that the message is a result of a deposit or a withdrawal
        // process.
        (
            uint64 nonce,
            uint256 balance,
            bool transferConfirmation,
            uint256 timestamp
        ) = message.decodeBalanceCheckMessage();
        // Get the last cached nonce
        uint64 _lastCachedNonce = lastTransferNonce;

        if (nonce != _lastCachedNonce) {
            // If nonce is not the last cached nonce, it is an outdated message
            // Ignore it
            return;
        }

        // A received message nonce not yet processed indicates there is a
        // deposit or withdrawal in progress.
        bool transferInProgress = isTransferPending();

        if (transferInProgress) {
            if (transferConfirmation) {
                // Apply the effects of the deposit / withdrawal completion
                _markNonceAsProcessed(nonce);
                pendingAmount = 0;
            } else {
                // A balanceCheck arrived that is not part of the deposit / withdrawal process
                // that has been generated on the Remote contract after the deposit / withdrawal which is
                // still pending. This can happen when the CCTP bridge delivers the messages out of order.
                // Ignore it, since the pending deposit / withdrawal must first be cofirmed.
                emit BalanceCheckIgnored(nonce, timestamp, false);
                return;
            }
        } else {
            if (block.timestamp > timestamp + MAX_BALANCE_CHECK_AGE) {
                // Balance check is too old, ignore it
                emit BalanceCheckIgnored(nonce, timestamp, true);
                return;
            }
        }

        // At this point update the strategy balance the balanceCheck message is either:
        // - a confirmation of a deposit / withdrawal
        // - a message that updates balances when no deposit / withdrawal is in progress
        remoteStrategyBalance = balance;
        emit RemoteStrategyBalanceUpdated(balance);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title CrossChainRemoteStrategy
 * @author Origin Protocol Inc
 *
 * @dev Part of the cross-chain strategy that lives on the remote chain.
 *      Handles deposits and withdrawals from the master strategy on peer chain
 *      and locally deposits the funds to a 4626 compatible vault.
 */

import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { Math } from "@openzeppelin/contracts/utils/math/Math.sol";
import { IERC20 } from "../../utils/InitializableAbstractStrategy.sol";
import { IERC4626 } from "../../../lib/openzeppelin/interfaces/IERC4626.sol";
import { IVaultV2 } from "../../interfaces/morpho/IVaultV2.sol";
import { Generalized4626Strategy } from "../Generalized4626Strategy.sol";
import { AbstractCCTPIntegrator } from "./AbstractCCTPIntegrator.sol";
import { CrossChainStrategyHelper } from "./CrossChainStrategyHelper.sol";
import { InitializableAbstractStrategy } from "../../utils/InitializableAbstractStrategy.sol";
import { Strategizable } from "../../governance/Strategizable.sol";
import { MorphoV2VaultUtils } from "../MorphoV2VaultUtils.sol";

contract CrossChainRemoteStrategy is
    AbstractCCTPIntegrator,
    Generalized4626Strategy,
    Strategizable
{
    using SafeERC20 for IERC20;
    using CrossChainStrategyHelper for bytes;

    event DepositUnderlyingFailed(string reason);
    event WithdrawalFailed(uint256 amountRequested, uint256 amountAvailable);
    event WithdrawUnderlyingFailed(string reason);

    modifier onlyOperatorOrStrategistOrGovernor() {
        require(
            msg.sender == operator ||
                msg.sender == strategistAddr ||
                isGovernor(),
            "Caller is not the Operator, Strategist or the Governor"
        );
        _;
    }

    modifier onlyGovernorOrStrategist()
        override(InitializableAbstractStrategy, Strategizable) {
        require(
            msg.sender == strategistAddr || isGovernor(),
            "Caller is not the Strategist or Governor"
        );
        _;
    }

    constructor(
        BaseStrategyConfig memory _baseConfig,
        CCTPIntegrationConfig memory _cctpConfig
    )
        AbstractCCTPIntegrator(_cctpConfig)
        Generalized4626Strategy(_baseConfig, _cctpConfig.usdcToken)
    {
        require(usdcToken == address(assetToken), "Token mismatch");
        require(
            _baseConfig.platformAddress != address(0),
            "Invalid platform address"
        );
        // Vault address must always be address(0) for the remote strategy
        require(
            _baseConfig.vaultAddress == address(0),
            "Invalid vault address"
        );
    }

    /**
     * @dev Initialize the strategy implementation
     * @param _strategist Address of the strategist
     * @param _operator Address of the operator
     * @param _minFinalityThreshold Minimum finality threshold
     * @param _feePremiumBps Fee premium in basis points
     */
    function initialize(
        address _strategist,
        address _operator,
        uint16 _minFinalityThreshold,
        uint16 _feePremiumBps
    ) external virtual onlyGovernor initializer {
        _initialize(_operator, _minFinalityThreshold, _feePremiumBps);
        _setStrategistAddr(_strategist);

        address[] memory rewardTokens = new address[](0);
        address[] memory assets = new address[](1);
        address[] memory pTokens = new address[](1);

        assets[0] = address(usdcToken);
        pTokens[0] = address(platformAddress);

        InitializableAbstractStrategy._initialize(
            rewardTokens,
            assets,
            pTokens
        );
    }

    /// @inheritdoc Generalized4626Strategy
    function deposit(address _asset, uint256 _amount)
        external
        virtual
        override
        onlyGovernorOrStrategist
        nonReentrant
    {
        _deposit(_asset, _amount);
    }

    /// @inheritdoc Generalized4626Strategy
    function depositAll()
        external
        virtual
        override
        onlyGovernorOrStrategist
        nonReentrant
    {
        _deposit(usdcToken, IERC20(usdcToken).balanceOf(address(this)));
    }

    /// @inheritdoc Generalized4626Strategy
    /// @dev Interface requires a recipient, but for compatibility it must be address(this).
    function withdraw(
        address _recipient,
        address _asset,
        uint256 _amount
    ) external virtual override onlyGovernorOrStrategist nonReentrant {
        _withdraw(_recipient, _asset, _amount);
    }

    /// @inheritdoc Generalized4626Strategy
    function withdrawAll()
        external
        virtual
        override
        onlyGovernorOrStrategist
        nonReentrant
    {
        IERC4626 platform = IERC4626(platformAddress);
        uint256 availableMorphoVault = MorphoV2VaultUtils.maxWithdrawableAssets(
            platformAddress,
            usdcToken
        );
        uint256 amountToWithdraw = Math.min(
            availableMorphoVault,
            platform.previewRedeem(platform.balanceOf(address(this)))
        );

        if (amountToWithdraw > 0) {
            _withdraw(address(this), usdcToken, amountToWithdraw);
        }
    }

    /// @inheritdoc AbstractCCTPIntegrator
    function _onMessageReceived(bytes memory payload) internal override {
        uint32 messageType = payload.getMessageType();
        if (messageType == CrossChainStrategyHelper.DEPOSIT_MESSAGE) {
            // Received when Master strategy sends tokens to the remote strategy
            // Do nothing because we receive acknowledgement with token transfer,
            // so _onTokenReceived will handle it
        } else if (messageType == CrossChainStrategyHelper.WITHDRAW_MESSAGE) {
            // Received when Master strategy requests a withdrawal
            _processWithdrawMessage(payload);
        } else {
            revert("Unknown message type");
        }
    }

    /**
     * @dev Process deposit message from peer strategy
     * @param tokenAmount Amount of tokens received
     * @param feeExecuted Fee executed
     * @param payload Payload of the message
     */
    function _processDepositMessage(
        // solhint-disable-next-line no-unused-vars
        uint256 tokenAmount,
        // solhint-disable-next-line no-unused-vars
        uint256 feeExecuted,
        bytes memory payload
    ) internal virtual {
        (uint64 nonce, ) = payload.decodeDepositMessage();

        // Replay protection is part of the _markNonceAsProcessed function
        _markNonceAsProcessed(nonce);

        // Deposit everything we got, not just what was bridged
        uint256 balance = IERC20(usdcToken).balanceOf(address(this));

        // Underlying call to deposit funds can fail. It mustn't affect the overall
        // flow as confirmation message should still be sent.
        if (balance >= MIN_TRANSFER_AMOUNT) {
            _deposit(usdcToken, balance);
        }

        // Send balance check message to the peer strategy
        bytes memory message = CrossChainStrategyHelper
            .encodeBalanceCheckMessage(
                lastTransferNonce,
                checkBalance(usdcToken),
                true,
                block.timestamp
            );
        _sendMessage(message);
    }

    /**
     * @dev Deposit assets by converting them to shares
     * @param _asset Address of asset to deposit
     * @param _amount Amount of asset to deposit
     */
    function _deposit(address _asset, uint256 _amount) internal override {
        // By design, this function should not revert. Otherwise, it'd
        // not be able to process messages and might freeze the contracts
        // state. However these two require statements would never fail
        // in every function invoking this. The same kind of checks should
        // be enforced in all the calling functions for these two and any
        // other require statements added to this function.
        require(_amount > 0, "Must deposit something");
        require(_asset == address(usdcToken), "Unexpected asset address");

        // This call can fail, and the failure doesn't need to bubble up to the _processDepositMessage function
        // as the flow is not affected by the failure.

        try IERC4626(platformAddress).deposit(_amount, address(this)) {
            emit Deposit(_asset, address(shareToken), _amount);
        } catch Error(string memory reason) {
            emit DepositUnderlyingFailed(
                string(abi.encodePacked("Deposit failed: ", reason))
            );
        } catch (bytes memory lowLevelData) {
            emit DepositUnderlyingFailed(
                string(
                    abi.encodePacked(
                        "Deposit failed: low-level call failed with data ",
                        lowLevelData
                    )
                )
            );
        }
    }

    /**
     * @dev Process withdrawal message from peer strategy
     * @param payload Payload of the message
     */
    function _processWithdrawMessage(bytes memory payload) internal virtual {
        (uint64 nonce, uint256 withdrawAmount) = payload
            .decodeWithdrawMessage();

        // Replay protection is part of the _markNonceAsProcessed function
        _markNonceAsProcessed(nonce);

        uint256 usdcBalance = IERC20(usdcToken).balanceOf(address(this));

        if (usdcBalance < withdrawAmount) {
            // Withdraw the missing funds from the remote strategy. This call can fail and
            // the failure doesn't bubble up to the _processWithdrawMessage function
            _withdraw(address(this), usdcToken, withdrawAmount - usdcBalance);

            // Update the possible increase in the balance on the contract.
            usdcBalance = IERC20(usdcToken).balanceOf(address(this));
        }

        // Check balance after withdrawal
        uint256 strategyBalance = checkBalance(usdcToken);

        // If there are some tokens to be sent AND the balance is sufficient
        // to satisfy the withdrawal request then send the funds to the peer strategy.
        // In case a direct withdraw(All) has previously been called
        // there is a possibility of USDC funds remaining on the contract.
        // A separate withdraw to extract or deposit to the Morpho vault needs to be
        // initiated from the peer Master strategy to utilise USDC funds.
        if (
            withdrawAmount >= MIN_TRANSFER_AMOUNT &&
            usdcBalance >= withdrawAmount
        ) {
            // The new balance on the contract needs to have USDC subtracted from it as
            // that will be withdrawn in the next step
            bytes memory message = CrossChainStrategyHelper
                .encodeBalanceCheckMessage(
                    lastTransferNonce,
                    strategyBalance - withdrawAmount,
                    true,
                    block.timestamp
                );
            _sendTokens(withdrawAmount, message);
        } else {
            // Contract either:
            // - only has small dust amount of USDC
            // - doesn't have sufficient funds to satisfy the withdrawal request
            // In both cases send the balance update message to the peer strategy.
            bytes memory message = CrossChainStrategyHelper
                .encodeBalanceCheckMessage(
                    lastTransferNonce,
                    strategyBalance,
                    true,
                    block.timestamp
                );
            _sendMessage(message);
            emit WithdrawalFailed(withdrawAmount, usdcBalance);
        }
    }

    /**
     * @dev Withdraw asset by burning shares
     * @param _recipient Address to receive withdrawn asset
     * @param _asset Address of asset to withdraw
     * @param _amount Amount of asset to withdraw
     */
    function _withdraw(
        address _recipient,
        address _asset,
        uint256 _amount
    ) internal override {
        require(_amount > 0, "Must withdraw something");
        require(_recipient == address(this), "Invalid recipient");
        require(_asset == address(usdcToken), "Unexpected asset address");

        // This call can fail, and the failure doesn't need to bubble up to the _processWithdrawMessage function
        // as the flow is not affected by the failure.
        try
            // slither-disable-next-line unused-return
            IERC4626(platformAddress).withdraw(
                _amount,
                address(this),
                address(this)
            )
        {
            emit Withdrawal(_asset, address(shareToken), _amount);
        } catch Error(string memory reason) {
            emit WithdrawUnderlyingFailed(
                string(abi.encodePacked("Withdrawal failed: ", reason))
            );
        } catch (bytes memory lowLevelData) {
            emit WithdrawUnderlyingFailed(
                string(
                    abi.encodePacked(
                        "Withdrawal failed: low-level call failed with data ",
                        lowLevelData
                    )
                )
            );
        }
    }

    /**
     * @dev Process token received message from peer strategy
     * @param tokenAmount Amount of tokens received
     * @param feeExecuted Fee executed
     * @param payload Payload of the message
     */
    function _onTokenReceived(
        uint256 tokenAmount,
        uint256 feeExecuted,
        bytes memory payload
    ) internal override {
        uint32 messageType = payload.getMessageType();

        require(
            messageType == CrossChainStrategyHelper.DEPOSIT_MESSAGE,
            "Invalid message type"
        );

        _processDepositMessage(tokenAmount, feeExecuted, payload);
    }

    /**
     * @dev Send balance update message to the peer strategy
     */
    function sendBalanceUpdate()
        external
        virtual
        onlyOperatorOrStrategistOrGovernor
    {
        uint256 balance = checkBalance(usdcToken);
        bytes memory message = CrossChainStrategyHelper
            .encodeBalanceCheckMessage(
                lastTransferNonce,
                balance,
                false,
                block.timestamp
            );
        _sendMessage(message);
    }

    /**
     * @notice Get the total asset value held in the platform and contract
     * @param _asset      Address of the asset
     * @return balance    Total value of the asset in the platform and contract
     */
    function checkBalance(address _asset)
        public
        view
        override
        returns (uint256)
    {
        require(_asset == usdcToken, "Unexpected asset address");
        /**
         * Balance of USDC on the contract is counted towards the total balance, since a deposit
         * to the Morpho V2 might fail and the USDC might remain on this contract as a result of a
         * bridged transfer.
         */
        uint256 balanceOnContract = IERC20(usdcToken).balanceOf(address(this));

        IERC4626 platform = IERC4626(platformAddress);
        return
            platform.previewRedeem(platform.balanceOf(address(this))) +
            balanceOnContract;
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { ERC4626 } from "../../lib/openzeppelin/contracts/token/ERC20/extensions/ERC4626.sol";
import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

import { Governable } from "../governance/Governable.sol";
import { Initializable } from "../utils/Initializable.sol";
import { OETH } from "./OETH.sol";

/**
 * @title Wrapped OETH Token Contract
 * @author Origin Protocol Inc
 *
 * @dev An important capability of this contract is that it isn't susceptible to changes of the
 * exchange rate of WOETH/OETH if/when someone sends the underlying asset (OETH) to the contract.
 * If OETH weren't rebasing this could be achieved by solely tracking the ERC20 transfers of the OETH
 * token on mint, deposit, redeem, withdraw. The issue is that OETH is rebasing and OETH balances
 * will change when the token rebases.
 * For that reason the contract logic checks the actual underlying OETH token balance only once
 * (either on a fresh contract creation or upgrade) and considering the WOETH supply and
 * rebasingCreditsPerToken calculates the _adjuster. Once the adjuster is calculated any donations
 * to the contract are ignored. The totalSupply (instead of querying OETH balance) works off of
 * adjuster the current WOETH supply and rebasingCreditsPerToken. This makes WOETH value accrual
 * completely follow OETH's value accrual.
 * WOETH is safe to use in lending markets as the VualtCore's _rebase contains safeguards preventing
 * any sudden large rebases.
 */

contract WOETH is ERC4626, Governable, Initializable {
    using SafeERC20 for IERC20;
    /* This is a 1e27 adjustment constant that expresses the difference in exchange rate between
     * OETH's rebase since inception (expressed with rebasingCreditsPerToken) and WOETH to OETH
     * conversion.
     *
     * If WOETH and OETH are deployed at the same time, the value of adjuster is a neutral 1e27
     */
    uint256 public adjuster;
    uint256[49] private __gap;

    // no need to set ERC20 name and symbol since they are overridden in WOETH & WOETHBase
    constructor(ERC20 underlying_) ERC20("", "") ERC4626(underlying_) {}

    /**
     * @notice Enable OETH rebasing for this contract
     */
    function initialize() external onlyGovernor initializer {
        OETH(address(asset())).rebaseOptIn();

        initialize2();
    }

    /**
     * @notice secondary initializer that newly deployed contracts will execute as part
     *         of primary initialize function and the existing contracts will have it called
     *         as a governance operation.
     */
    function initialize2() public onlyGovernor {
        require(adjuster == 0, "Initialize2 already called");

        if (totalSupply() == 0) {
            adjuster = 1e27;
        } else {
            adjuster =
                (rebasingCreditsPerTokenHighres() *
                    ERC20(asset()).balanceOf(address(this))) /
                totalSupply();
        }
    }

    function name()
        public
        view
        virtual
        override(ERC20, IERC20Metadata)
        returns (string memory)
    {
        return "Wrapped OETH";
    }

    function symbol()
        public
        view
        virtual
        override(ERC20, IERC20Metadata)
        returns (string memory)
    {
        return "wOETH";
    }

    /**
     * @notice Transfer token to governor. Intended for recovering tokens stuck in
     *      contract, i.e. mistaken sends. Cannot transfer OETH
     * @param asset_ Address for the asset
     * @param amount_ Amount of the asset to transfer
     */
    function transferToken(address asset_, uint256 amount_)
        external
        onlyGovernor
    {
        require(asset_ != address(asset()), "Cannot collect core asset");
        IERC20(asset_).safeTransfer(governor(), amount_);
    }

    /// @inheritdoc ERC4626
    function convertToShares(uint256 assets)
        public
        view
        virtual
        override
        returns (uint256 shares)
    {
        return (assets * rebasingCreditsPerTokenHighres()) / adjuster;
    }

    /// @inheritdoc ERC4626
    function convertToAssets(uint256 shares)
        public
        view
        virtual
        override
        returns (uint256 assets)
    {
        return (shares * adjuster) / rebasingCreditsPerTokenHighres();
    }

    /// @inheritdoc ERC4626
    function totalAssets() public view override returns (uint256) {
        return (totalSupply() * adjuster) / rebasingCreditsPerTokenHighres();
    }

    function rebasingCreditsPerTokenHighres() internal view returns (uint256) {
        return OETH(asset()).rebasingCreditsPerTokenHighres();
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { WOETH } from "./WOETH.sol";

/**
 * @title Wrapped Origin Sonic (wOS) token on Sonic
 * @author Origin Protocol Inc
 */
contract WOSonic is WOETH {
    constructor(ERC20 underlying_) WOETH(underlying_) {}

    function name()
        public
        view
        virtual
        override(WOETH)
        returns (string memory)
    {
        return "Wrapped OS";
    }

    function symbol()
        public
        view
        virtual
        override(WOETH)
        returns (string memory)
    {
        return "wOS";
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { WOETH } from "./WOETH.sol";
import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/**
 * @title wOETH (Plume) Token Contract
 * @author Origin Protocol Inc
 */

contract WOETHPlume is WOETH {
    constructor(ERC20 underlying_) WOETH(underlying_) {}

    function name() public view virtual override returns (string memory) {
        return "Wrapped Super OETH";
    }

    function symbol() public view virtual override returns (string memory) {
        return "wsuperOETHp";
    }
}


## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

