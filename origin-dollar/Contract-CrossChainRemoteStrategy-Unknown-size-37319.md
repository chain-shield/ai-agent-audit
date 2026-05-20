
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

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
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
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

import { Governable } from "./Governable.sol";

contract Strategizable is Governable {
    event StrategistUpdated(address _address);

    // Address of strategist
    address public strategistAddr;

    // For future use
    uint256[50] private __gap;

    /**
     * @dev Verifies that the caller is either Governor or Strategist.
     */
    modifier onlyGovernorOrStrategist() virtual {
        require(
            msg.sender == strategistAddr || isGovernor(),
            "Caller is not the Strategist or Governor"
        );
        _;
    }

    /**
     * @dev Set address of Strategist
     * @param _address Address of Strategist
     */
    function setStrategistAddr(address _address) external onlyGovernor {
        _setStrategistAddr(_address);
    }

    /**
     * @dev Set address of Strategist
     * @param _address Address of Strategist
     */
    function _setStrategistAddr(address _address) internal {
        strategistAddr = _address;
        emit StrategistUpdated(_address);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title OUSD Token Contract
 * @dev ERC20 compatible contract for OUSD
 * @dev Implements an elastic supply
 * @author Origin Protocol Inc
 */
import { IVault } from "../interfaces/IVault.sol";
import { Governable } from "../governance/Governable.sol";
import { SafeCast } from "@openzeppelin/contracts/utils/math/SafeCast.sol";

contract OUSD is Governable {
    using SafeCast for int256;
    using SafeCast for uint256;

    /// @dev Event triggered when the supply changes
    /// @param totalSupply Updated token total supply
    /// @param rebasingCredits Updated token rebasing credits
    /// @param rebasingCreditsPerToken Updated token rebasing credits per token
    event TotalSupplyUpdatedHighres(
        uint256 totalSupply,
        uint256 rebasingCredits,
        uint256 rebasingCreditsPerToken
    );
    /// @dev Event triggered when an account opts in for rebasing
    /// @param account Address of the account
    event AccountRebasingEnabled(address account);
    /// @dev Event triggered when an account opts out of rebasing
    /// @param account Address of the account
    event AccountRebasingDisabled(address account);
    /// @dev Emitted when `value` tokens are moved from one account `from` to
    ///      another `to`.
    /// @param from Address of the account tokens are moved from
    /// @param to Address of the account tokens are moved to
    /// @param value Amount of tokens transferred
    event Transfer(address indexed from, address indexed to, uint256 value);
    /// @dev Emitted when the allowance of a `spender` for an `owner` is set by
    ///      a call to {approve}. `value` is the new allowance.
    /// @param owner Address of the owner approving allowance
    /// @param spender Address of the spender allowance is granted to
    /// @param value Amount of tokens spender can transfer
    event Approval(
        address indexed owner,
        address indexed spender,
        uint256 value
    );
    /// @dev Yield resulting from {changeSupply} that a `source` account would
    ///      receive is directed to `target` account.
    /// @param source Address of the source forwarding the yield
    /// @param target Address of the target receiving the yield
    event YieldDelegated(address source, address target);
    /// @dev Yield delegation from `source` account to the `target` account is
    ///      suspended.
    /// @param source Address of the source suspending yield forwarding
    /// @param target Address of the target no longer receiving yield from `source`
    ///        account
    event YieldUndelegated(address source, address target);

    enum RebaseOptions {
        NotSet,
        StdNonRebasing,
        StdRebasing,
        YieldDelegationSource,
        YieldDelegationTarget
    }

    uint256[154] private _gap; // Slots to align with deployed contract
    uint256 private constant MAX_SUPPLY = type(uint128).max;
    /// @dev The amount of tokens in existence
    uint256 public totalSupply;
    mapping(address => mapping(address => uint256)) private allowances;
    /// @dev The vault with privileges to execute {mint}, {burn}
    ///     and {changeSupply}
    address public vaultAddress;
    mapping(address => uint256) internal creditBalances;
    // the 2 storage variables below need trailing underscores to not name collide with public functions
    uint256 private rebasingCredits_; // Sum of all rebasing credits (creditBalances for rebasing accounts)
    uint256 private rebasingCreditsPerToken_;
    /// @dev The amount of tokens that are not rebasing - receiving yield
    uint256 public nonRebasingSupply;
    mapping(address => uint256) internal alternativeCreditsPerToken;
    /// @dev A map of all addresses and their respective RebaseOptions
    mapping(address => RebaseOptions) public rebaseState;
    mapping(address => uint256) private __deprecated_isUpgraded;
    /// @dev A map of addresses that have yields forwarded to. This is an
    ///      inverse mapping of {yieldFrom}
    /// Key Account forwarding yield
    /// Value Account receiving yield
    mapping(address => address) public yieldTo;
    /// @dev A map of addresses that are receiving the yield. This is an
    ///      inverse mapping of {yieldTo}
    /// Key Account receiving yield
    /// Value Account forwarding yield
    mapping(address => address) public yieldFrom;

    uint256 private constant RESOLUTION_INCREASE = 1e9;
    uint256[34] private __gap; // including below gap totals up to 200

    /// @dev Verifies that the caller is the Governor or Strategist.
    modifier onlyGovernorOrStrategist() {
        require(
            isGovernor() || msg.sender == IVault(vaultAddress).strategistAddr(),
            "Caller is not the Strategist or Governor"
        );
        _;
    }

    /// @dev Initializes the contract and sets necessary variables.
    /// @param _vaultAddress Address of the vault contract
    /// @param _initialCreditsPerToken The starting rebasing credits per token.
    function initialize(address _vaultAddress, uint256 _initialCreditsPerToken)
        external
        onlyGovernor
    {
        require(_vaultAddress != address(0), "Zero vault address");
        require(vaultAddress == address(0), "Already initialized");

        rebasingCreditsPerToken_ = _initialCreditsPerToken;
        vaultAddress = _vaultAddress;
    }

    /// @dev Returns the symbol of the token, a shorter version
    ///      of the name.
    function symbol() external pure virtual returns (string memory) {
        return "OUSD";
    }

    /// @dev Returns the name of the token.
    function name() external pure virtual returns (string memory) {
        return "Origin Dollar";
    }

    /// @dev Returns the number of decimals used to get its user representation.
    function decimals() external pure virtual returns (uint8) {
        return 18;
    }

    /**
     * @dev Verifies that the caller is the Vault contract
     */
    modifier onlyVault() {
        require(vaultAddress == msg.sender, "Caller is not the Vault");
        _;
    }

    /**
     * @return High resolution rebasingCreditsPerToken
     */
    function rebasingCreditsPerTokenHighres() external view returns (uint256) {
        return rebasingCreditsPerToken_;
    }

    /**
     * @return Low resolution rebasingCreditsPerToken
     */
    function rebasingCreditsPerToken() external view returns (uint256) {
        return rebasingCreditsPerToken_ / RESOLUTION_INCREASE;
    }

    /**
     * @return High resolution total number of rebasing credits
     */
    function rebasingCreditsHighres() external view returns (uint256) {
        return rebasingCredits_;
    }

    /**
     * @return Low resolution total number of rebasing credits
     */
    function rebasingCredits() external view returns (uint256) {
        return rebasingCredits_ / RESOLUTION_INCREASE;
    }

    /**
     * @notice Gets the balance of the specified address.
     * @param _account Address to query the balance of.
     * @return A uint256 representing the amount of base units owned by the
     *         specified address.
     */
    function balanceOf(address _account) public view returns (uint256) {
        RebaseOptions state = rebaseState[_account];
        if (state == RebaseOptions.YieldDelegationSource) {
            // Saves a slot read when transferring to or from a yield delegating source
            // since we know creditBalances equals the balance.
            return creditBalances[_account];
        }
        uint256 baseBalance = (creditBalances[_account] * 1e18) /
            _creditsPerToken(_account);
        if (state == RebaseOptions.YieldDelegationTarget) {
            // creditBalances of yieldFrom accounts equals token balances
            return baseBalance - creditBalances[yieldFrom[_account]];
        }
        return baseBalance;
    }

    /**
     * @notice Gets the credits balance of the specified address.
     * @dev Backwards compatible with old low res credits per token.
     * @param _account The address to query the balance of.
     * @return (uint256, uint256) Credit balance and credits per token of the
     *         address
     */
    function creditsBalanceOf(address _account)
        external
        view
        returns (uint256, uint256)
    {
        uint256 cpt = _creditsPerToken(_account);
        if (cpt == 1e27) {
            // For a period before the resolution upgrade, we created all new
            // contract accounts at high resolution. Since they are not changing
            // as a result of this upgrade, we will return their true values
            return (creditBalances[_account], cpt);
        } else {
            return (
                creditBalances[_account] / RESOLUTION_INCREASE,
                cpt / RESOLUTION_INCREASE
            );
        }
    }

    /**
     * @notice Gets the credits balance of the specified address.
     * @param _account The address to query the balance of.
     * @return (uint256, uint256, bool) Credit balance, credits per token of the
     *         address, and isUpgraded
     */
    function creditsBalanceOfHighres(address _account)
        external
        view
        returns (
            uint256,
            uint256,
            bool
        )
    {
        return (
            creditBalances[_account],
            _creditsPerToken(_account),
            true // all accounts have their resolution "upgraded"
        );
    }

    // Backwards compatible view
    function nonRebasingCreditsPerToken(address _account)
        external
        view
        returns (uint256)
    {
        return alternativeCreditsPerToken[_account];
    }

    /**
     * @notice Transfer tokens to a specified address.
     * @param _to the address to transfer to.
     * @param _value the amount to be transferred.
     * @return true on success.
     */
    function transfer(address _to, uint256 _value) external returns (bool) {
        require(_to != address(0), "Transfer to zero address");

        _executeTransfer(msg.sender, _to, _value);

        emit Transfer(msg.sender, _to, _value);
        return true;
    }

    /**
     * @notice Transfer tokens from one address to another.
     * @param _from The address you want to send tokens from.
     * @param _to The address you want to transfer to.
     * @param _value The amount of tokens to be transferred.
     * @return true on success.
     */
    function transferFrom(
        address _from,
        address _to,
        uint256 _value
    ) external returns (bool) {
        require(_to != address(0), "Transfer to zero address");
        uint256 userAllowance = allowances[_from][msg.sender];
        require(_value <= userAllowance, "Allowance exceeded");

        unchecked {
            allowances[_from][msg.sender] = userAllowance - _value;
        }

        _executeTransfer(_from, _to, _value);

        emit Transfer(_from, _to, _value);
        return true;
    }

    function _executeTransfer(
        address _from,
        address _to,
        uint256 _value
    ) internal {
        (
            int256 fromRebasingCreditsDiff,
            int256 fromNonRebasingSupplyDiff
        ) = _adjustAccount(_from, -_value.toInt256());
        (
            int256 toRebasingCreditsDiff,
            int256 toNonRebasingSupplyDiff
        ) = _adjustAccount(_to, _value.toInt256());

        _adjustGlobals(
            fromRebasingCreditsDiff + toRebasingCreditsDiff,
            fromNonRebasingSupplyDiff + toNonRebasingSupplyDiff
        );
    }

    function _adjustAccount(address _account, int256 _balanceChange)
        internal
        returns (int256 rebasingCreditsDiff, int256 nonRebasingSupplyDiff)
    {
        RebaseOptions state = rebaseState[_account];
        int256 currentBalance = balanceOf(_account).toInt256();
        if (currentBalance + _balanceChange < 0) {
            revert("Transfer amount exceeds balance");
        }
        uint256 newBalance = (currentBalance + _balanceChange).toUint256();

        if (state == RebaseOptions.YieldDelegationSource) {
            address target = yieldTo[_account];
            uint256 targetOldBalance = balanceOf(target);
            uint256 targetNewCredits = _balanceToRebasingCredits(
                targetOldBalance + newBalance
            );
            rebasingCreditsDiff =
                targetNewCredits.toInt256() -
                creditBalances[target].toInt256();

            creditBalances[_account] = newBalance;
            creditBalances[target] = targetNewCredits;
        } else if (state == RebaseOptions.YieldDelegationTarget) {
            uint256 newCredits = _balanceToRebasingCredits(
                newBalance + creditBalances[yieldFrom[_account]]
            );
            rebasingCreditsDiff =
                newCredits.toInt256() -
                creditBalances[_account].toInt256();
            creditBalances[_account] = newCredits;
        } else {
            _autoMigrate(_account);
            uint256 alternativeCreditsPerTokenMem = alternativeCreditsPerToken[
                _account
            ];
            if (alternativeCreditsPerTokenMem > 0) {
                nonRebasingSupplyDiff = _balanceChange;
                if (alternativeCreditsPerTokenMem != 1e18) {
                    alternativeCreditsPerToken[_account] = 1e18;
                }
                creditBalances[_account] = newBalance;
            } else {
                uint256 newCredits = _balanceToRebasingCredits(newBalance);
                rebasingCreditsDiff =
                    newCredits.toInt256() -
                    creditBalances[_account].toInt256();
                creditBalances[_account] = newCredits;
            }
        }
    }

    function _adjustGlobals(
        int256 _rebasingCreditsDiff,
        int256 _nonRebasingSupplyDiff
    ) internal {
        if (_rebasingCreditsDiff != 0) {
            rebasingCredits_ = (rebasingCredits_.toInt256() +
                _rebasingCreditsDiff).toUint256();
        }
        if (_nonRebasingSupplyDiff != 0) {
            nonRebasingSupply = (nonRebasingSupply.toInt256() +
                _nonRebasingSupplyDiff).toUint256();
        }
    }

    /**
     * @notice Function to check the amount of tokens that _owner has allowed
     *      to `_spender`.
     * @param _owner The address which owns the funds.
     * @param _spender The address which will spend the funds.
     * @return The number of tokens still available for the _spender.
     */
    function allowance(address _owner, address _spender)
        external
        view
        returns (uint256)
    {
        return allowances[_owner][_spender];
    }

    /**
     * @notice Approve the passed address to spend the specified amount of
     *      tokens on behalf of msg.sender.
     * @param _spender The address which will spend the funds.
     * @param _value The amount of tokens to be spent.
     * @return true on success.
     */
    function approve(address _spender, uint256 _value) external returns (bool) {
        allowances[msg.sender][_spender] = _value;
        emit Approval(msg.sender, _spender, _value);
        return true;
    }

    /**
     * @notice Creates `_amount` tokens and assigns them to `_account`,
     *     increasing the total supply.
     */
    function mint(address _account, uint256 _amount) external onlyVault {
        require(_account != address(0), "Mint to the zero address");

        // Account
        (
            int256 toRebasingCreditsDiff,
            int256 toNonRebasingSupplyDiff
        ) = _adjustAccount(_account, _amount.toInt256());
        // Globals
        _adjustGlobals(toRebasingCreditsDiff, toNonRebasingSupplyDiff);
        totalSupply = totalSupply + _amount;

        require(totalSupply < MAX_SUPPLY, "Max supply");
        emit Transfer(address(0), _account, _amount);
    }

    /**
     * @notice Destroys `_amount` tokens from `_account`,
     *     reducing the total supply.
     */
    function burn(address _account, uint256 _amount) external onlyVault {
        require(_account != address(0), "Burn from the zero address");
        if (_amount == 0) {
            return;
        }

        // Account
        (
            int256 toRebasingCreditsDiff,
            int256 toNonRebasingSupplyDiff
        ) = _adjustAccount(_account, -_amount.toInt256());
        // Globals
        _adjustGlobals(toRebasingCreditsDiff, toNonRebasingSupplyDiff);
        totalSupply = totalSupply - _amount;

        emit Transfer(_account, address(0), _amount);
    }

    /**
     * @dev Get the credits per token for an account. Returns a fixed amount
     *      if the account is non-rebasing.
     * @param _account Address of the account.
     */
    function _creditsPerToken(address _account)
        internal
        view
        returns (uint256)
    {
        uint256 alternativeCreditsPerTokenMem = alternativeCreditsPerToken[
            _account
        ];
        if (alternativeCreditsPerTokenMem != 0) {
            return alternativeCreditsPerTokenMem;
        } else {
            return rebasingCreditsPerToken_;
        }
    }

    /**
     * @dev Auto migrate contracts to be non rebasing,
     *     unless they have opted into yield.
     * @param _account Address of the account.
     */
    function _autoMigrate(address _account) internal {
        uint256 codeLen = _account.code.length;
        bool isEOA = (codeLen == 0) ||
            (codeLen == 23 && bytes3(_account.code) == 0xef0100);
        // In previous code versions, contracts would not have had their
        // rebaseState[_account] set to RebaseOptions.NonRebasing when migrated
        // therefore we check the actual accounting used on the account as well.
        if (
            (!isEOA) &&
            rebaseState[_account] == RebaseOptions.NotSet &&
            alternativeCreditsPerToken[_account] == 0
        ) {
            _rebaseOptOut(_account);
        }
    }

    /**
     * @dev Calculates credits from contract's global rebasingCreditsPerToken_, and
     *      also balance that corresponds to those credits. The latter is important
     *      when adjusting the contract's global nonRebasingSupply to circumvent any
     *      possible rounding errors.
     *
     * @param _balance Balance of the account.
     */
    function _balanceToRebasingCredits(uint256 _balance)
        internal
        view
        returns (uint256 rebasingCredits)
    {
        // Rounds up, because we need to ensure that accounts always have
        // at least the balance that they should have.
        // Note this should always be used on an absolute account value,
        // not on a possibly negative diff, because then the rounding would be wrong.
        return ((_balance) * rebasingCreditsPerToken_ + 1e18 - 1) / 1e18;
    }

    /**
     * @notice The calling account will start receiving yield after a successful call.
     * @param _account Address of the account.
     */
    function governanceRebaseOptIn(address _account) external onlyGovernor {
        require(_account != address(0), "Zero address not allowed");
        _rebaseOptIn(_account);
    }

    /**
     * @notice The calling account will start receiving yield after a successful call.
     */
    function rebaseOptIn() external {
        _rebaseOptIn(msg.sender);
    }

    function _rebaseOptIn(address _account) internal {
        uint256 balance = balanceOf(_account);

        // prettier-ignore
        require(
            alternativeCreditsPerToken[_account] > 0 ||
                // Accounts may explicitly `rebaseOptIn` regardless of
                // accounting if they have a 0 balance.
                creditBalances[_account] == 0
            ,
            "Account must be non-rebasing"
        );
        RebaseOptions state = rebaseState[_account];
        // prettier-ignore
        require(
            state == RebaseOptions.StdNonRebasing ||
                state == RebaseOptions.NotSet,
            "Only standard non-rebasing accounts can opt in"
        );

        uint256 newCredits = _balanceToRebasingCredits(balance);

        // Account
        rebaseState[_account] = RebaseOptions.StdRebasing;
        alternativeCreditsPerToken[_account] = 0;
        creditBalances[_account] = newCredits;
        // Globals
        _adjustGlobals(newCredits.toInt256(), -balance.toInt256());

        emit AccountRebasingEnabled(_account);
    }

    /**
     * @notice The calling account will no longer receive yield
     */
    function rebaseOptOut() external {
        _rebaseOptOut(msg.sender);
    }

    function _rebaseOptOut(address _account) internal {
        require(
            alternativeCreditsPerToken[_account] == 0,
            "Account must be rebasing"
        );
        RebaseOptions state = rebaseState[_account];
        require(
            state == RebaseOptions.StdRebasing || state == RebaseOptions.NotSet,
            "Only standard rebasing accounts can opt out"
        );

        uint256 oldCredits = creditBalances[_account];
        uint256 balance = balanceOf(_account);

        // Account
        rebaseState[_account] = RebaseOptions.StdNonRebasing;
        alternativeCreditsPerToken[_account] = 1e18;
        creditBalances[_account] = balance;
        // Globals
        _adjustGlobals(-oldCredits.toInt256(), balance.toInt256());

        emit AccountRebasingDisabled(_account);
    }

    /**
     * @notice Distribute yield to users. This changes the exchange rate
     *  between "credits" and OUSD tokens to change rebasing user's balances.
     * @param _newTotalSupply New total supply of OUSD.
     */
    function changeSupply(uint256 _newTotalSupply) external onlyVault {
        require(totalSupply > 0, "Cannot increase 0 supply");

        if (totalSupply == _newTotalSupply) {
            emit TotalSupplyUpdatedHighres(
                totalSupply,
                rebasingCredits_,
                rebasingCreditsPerToken_
            );
            return;
        }

        totalSupply = _newTotalSupply > MAX_SUPPLY
            ? MAX_SUPPLY
            : _newTotalSupply;

        uint256 rebasingSupply = totalSupply - nonRebasingSupply;
        // round up in the favour of the protocol
        rebasingCreditsPerToken_ =
            (rebasingCredits_ * 1e18 + rebasingSupply - 1) /
            rebasingSupply;

        require(rebasingCreditsPerToken_ > 0, "Invalid change in supply");

        emit TotalSupplyUpdatedHighres(
            totalSupply,
            rebasingCredits_,
            rebasingCreditsPerToken_
        );
    }

    /*
     * @notice Send the yield from one account to another account.
     *         Each account keeps its own balances.
     */
    function delegateYield(address _from, address _to)
        external
        onlyGovernorOrStrategist
    {
        require(_from != address(0), "Zero from address not allowed");
        require(_to != address(0), "Zero to address not allowed");

        require(_from != _to, "Cannot delegate to self");
        require(
            yieldFrom[_to] == address(0) &&
                yieldTo[_to] == address(0) &&
                yieldFrom[_from] == address(0) &&
                yieldTo[_from] == address(0),
            "Blocked by existing yield delegation"
        );
        RebaseOptions stateFrom = rebaseState[_from];
        RebaseOptions stateTo = rebaseState[_to];

        require(
            stateFrom == RebaseOptions.NotSet ||
                stateFrom == RebaseOptions.StdNonRebasing ||
                stateFrom == RebaseOptions.StdRebasing,
            "Invalid rebaseState from"
        );

        require(
            stateTo == RebaseOptions.NotSet ||
                stateTo == RebaseOptions.StdNonRebasing ||
                stateTo == RebaseOptions.StdRebasing,
            "Invalid rebaseState to"
        );

        if (alternativeCreditsPerToken[_from] == 0) {
            _rebaseOptOut(_from);
        }
        if (alternativeCreditsPerToken[_to] > 0) {
            _rebaseOptIn(_to);
        }

        uint256 fromBalance = balanceOf(_from);
        uint256 toBalance = balanceOf(_to);
        uint256 oldToCredits = creditBalances[_to];
        uint256 newToCredits = _balanceToRebasingCredits(
            fromBalance + toBalance
        );

        // Set up the bidirectional links
        yieldTo[_from] = _to;
        yieldFrom[_to] = _from;

        // Local
        rebaseState[_from] = RebaseOptions.YieldDelegationSource;
        alternativeCreditsPerToken[_from] = 1e18;
        creditBalances[_from] = fromBalance;
        rebaseState[_to] = RebaseOptions.YieldDelegationTarget;
        creditBalances[_to] = newToCredits;

        // Global
        int256 creditsChange = newToCredits.toInt256() -
            oldToCredits.toInt256();
        _adjustGlobals(creditsChange, -(fromBalance).toInt256());
        emit YieldDelegated(_from, _to);
    }

    /*
     * @notice Stop sending the yield from one account to another account.
     */
    function undelegateYield(address _from) external onlyGovernorOrStrategist {
        // Require a delegation, which will also ensure a valid delegation
        require(yieldTo[_from] != address(0), "Zero address not allowed");

        address to = yieldTo[_from];
        uint256 fromBalance = balanceOf(_from);
        uint256 toBalance = balanceOf(to);
        uint256 oldToCredits = creditBalances[to];
        uint256 newToCredits = _balanceToRebasingCredits(toBalance);

        // Remove the bidirectional links
        yieldFrom[to] = address(0);
        yieldTo[_from] = address(0);

        // Local
        rebaseState[_from] = RebaseOptions.StdNonRebasing;
        // alternativeCreditsPerToken[from] already 1e18 from `delegateYield()`
        creditBalances[_from] = fromBalance;
        rebaseState[to] = RebaseOptions.StdRebasing;
        // alternativeCreditsPerToken[to] already 0 from `delegateYield()`
        creditBalances[to] = newToCredits;

        // Global
        int256 creditsChange = newToCredits.toInt256() -
            oldToCredits.toInt256();
        _adjustGlobals(creditsChange, fromBalance.toInt256());
        emit YieldUndelegated(_from, to);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { IERC20 } from "../utils/InitializableAbstractStrategy.sol";
import { IERC4626 } from "../../lib/openzeppelin/interfaces/IERC4626.sol";
import { IVaultV2 } from "../interfaces/morpho/IVaultV2.sol";
import { IMorphoV2Adapter } from "../interfaces/morpho/IMorphoV2Adapter.sol";

library MorphoV2VaultUtils {
    error IncompatibleAdapter(address adapter);

    /**
     * @notice Return maximum amount that can be safely withdrawn from a Morpho V2 vault.
     * @dev Available liquidity is:
     *      1) asset balance parked on Morpho V2 vault contract
     *      2) additional liquidity from the active adapter if it resolves to a Morpho V1 vault
     *         and, when provided, matches the expected adapter
     */
    function maxWithdrawableAssets(address platformAddress, address assetToken)
        internal
        view
        returns (uint256 availableAssetLiquidity)
    {
        availableAssetLiquidity = IERC20(assetToken).balanceOf(platformAddress);

        address liquidityAdapter = IVaultV2(platformAddress).liquidityAdapter();
        // this is a sufficient check to ensure the adapter is Morpho V1
        try IMorphoV2Adapter(liquidityAdapter).morphoVaultV1() returns (
            address underlyingVault
        ) {
            availableAssetLiquidity += IERC4626(underlyingVault).maxWithdraw(
                liquidityAdapter
            );
        } catch {
            revert IncompatibleAdapter(liquidityAdapter);
        }
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

/**
 * @title Base contract for vault strategies.
 * @author Origin Protocol Inc
 */
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

import { Initializable } from "../utils/Initializable.sol";
import { Governable } from "../governance/Governable.sol";
import { IVault } from "../interfaces/IVault.sol";

abstract contract InitializableAbstractStrategy is Initializable, Governable {
    using SafeERC20 for IERC20;

    event PTokenAdded(address indexed _asset, address _pToken);
    event PTokenRemoved(address indexed _asset, address _pToken);
    event Deposit(address indexed _asset, address _pToken, uint256 _amount);
    event Withdrawal(address indexed _asset, address _pToken, uint256 _amount);
    event RewardTokenCollected(
        address recipient,
        address rewardToken,
        uint256 amount
    );
    event RewardTokenAddressesUpdated(
        address[] _oldAddresses,
        address[] _newAddresses
    );
    event HarvesterAddressesUpdated(
        address _oldHarvesterAddress,
        address _newHarvesterAddress
    );

    /// @notice Address of the underlying platform
    address public immutable platformAddress;
    /// @notice Address of the OToken vault
    address public immutable vaultAddress;

    /// @dev Replaced with an immutable variable
    // slither-disable-next-line constable-states
    address private _deprecated_platformAddress;

    /// @dev Replaced with an immutable
    // slither-disable-next-line constable-states
    address private _deprecated_vaultAddress;

    /// @notice asset => pToken (Platform Specific Token Address)
    mapping(address => address) public assetToPToken;

    /// @notice Full list of all assets supported by the strategy
    address[] internal assetsMapped;

    // Deprecated: Reward token address
    // slither-disable-next-line constable-states
    address private _deprecated_rewardTokenAddress;

    // Deprecated: now resides in Harvester's rewardTokenConfigs
    // slither-disable-next-line constable-states
    uint256 private _deprecated_rewardLiquidationThreshold;

    /// @notice Address of the Harvester contract allowed to collect reward tokens
    address public harvesterAddress;

    /// @notice Address of the reward tokens. eg CRV, BAL, CVX, AURA
    address[] public rewardTokenAddresses;

    /* Reserved for future expansion. Used to be 100 storage slots
     * and has decreased to accommodate:
     * - harvesterAddress
     * - rewardTokenAddresses
     */
    int256[98] private _reserved;

    struct BaseStrategyConfig {
        address platformAddress; // Address of the underlying platform
        address vaultAddress; // Address of the OToken's Vault
    }

    /**
     * @dev Verifies that the caller is the Governor or Strategist.
     */
    modifier onlyGovernorOrStrategist() virtual {
        require(
            isGovernor() || msg.sender == IVault(vaultAddress).strategistAddr(),
            "Caller is not the Strategist or Governor"
        );
        _;
    }

    /**
     * @param _config The platform and OToken vault addresses
     */
    constructor(BaseStrategyConfig memory _config) {
        platformAddress = _config.platformAddress;
        vaultAddress = _config.vaultAddress;
    }

    /**
     * @dev Internal initialize function, to set up initial internal state
     * @param _rewardTokenAddresses Address of reward token for platform
     * @param _assets Addresses of initial supported assets
     * @param _pTokens Platform Token corresponding addresses
     */
    function _initialize(
        address[] memory _rewardTokenAddresses,
        address[] memory _assets,
        address[] memory _pTokens
    ) internal {
        rewardTokenAddresses = _rewardTokenAddresses;

        uint256 assetCount = _assets.length;
        require(assetCount == _pTokens.length, "Invalid input arrays");
        for (uint256 i = 0; i < assetCount; ++i) {
            _setPTokenAddress(_assets[i], _pTokens[i]);
        }
    }

    /**
     * @notice Collect accumulated reward token and send to Vault.
     *         No-ops when the harvester address is not set.
     */
    function collectRewardTokens()
        external
        virtual
        onlyHarvesterOrStrategist
        nonReentrant
    {
        if (harvesterAddress == address(0)) {
            return;
        }
        _collectRewardTokens();
    }

    /**
     * @dev Default implementation that transfers reward tokens to the Harvester.
     * Implementing strategies need to add custom logic to collect the rewards.
     */
    function _collectRewardTokens() internal virtual {
        if (harvesterAddress == address(0)) {
            return;
        }
        uint256 rewardTokenCount = rewardTokenAddresses.length;
        for (uint256 i = 0; i < rewardTokenCount; ++i) {
            IERC20 rewardToken = IERC20(rewardTokenAddresses[i]);
            uint256 balance = rewardToken.balanceOf(address(this));
            if (balance > 0) {
                emit RewardTokenCollected(
                    harvesterAddress,
                    address(rewardToken),
                    balance
                );
                rewardToken.safeTransfer(harvesterAddress, balance);
            }
        }
    }

    /**
     * @dev Verifies that the caller is the Vault.
     */
    modifier onlyVault() {
        require(msg.sender == vaultAddress, "Caller is not the Vault");
        _;
    }

    /**
     * @dev Verifies that the caller is the Harvester or Strategist.
     */
    modifier onlyHarvesterOrStrategist() {
        require(
            msg.sender == harvesterAddress ||
                msg.sender == IVault(vaultAddress).strategistAddr(),
            "Caller is not the Harvester or Strategist"
        );
        _;
    }

    /**
     * @dev Verifies that the caller is the Vault or Governor.
     */
    modifier onlyVaultOrGovernor() {
        require(
            msg.sender == vaultAddress || msg.sender == governor(),
            "Caller is not the Vault or Governor"
        );
        _;
    }

    /**
     * @dev Verifies that the caller is the Vault, Governor, or Strategist.
     */
    modifier onlyVaultOrGovernorOrStrategist() {
        require(
            msg.sender == vaultAddress ||
                msg.sender == governor() ||
                msg.sender == IVault(vaultAddress).strategistAddr(),
            "Caller is not the Vault, Governor, or Strategist"
        );
        _;
    }

    /**
     * @notice Set the reward token addresses. Any old addresses will be overwritten.
     * @param _rewardTokenAddresses Array of reward token addresses
     */
    function setRewardTokenAddresses(address[] calldata _rewardTokenAddresses)
        external
        onlyGovernor
    {
        uint256 rewardTokenCount = _rewardTokenAddresses.length;
        for (uint256 i = 0; i < rewardTokenCount; ++i) {
            require(
                _rewardTokenAddresses[i] != address(0),
                "Can not set an empty address as a reward token"
            );
        }

        emit RewardTokenAddressesUpdated(
            rewardTokenAddresses,
            _rewardTokenAddresses
        );
        rewardTokenAddresses = _rewardTokenAddresses;
    }

    /**
     * @notice Get the reward token addresses.
     * @return address[] the reward token addresses.
     */
    function getRewardTokenAddresses()
        external
        view
        returns (address[] memory)
    {
        return rewardTokenAddresses;
    }

    /**
     * @notice Provide support for asset by passing its pToken address.
     *      This method can only be called by the system Governor
     * @param _asset    Address for the asset
     * @param _pToken   Address for the corresponding platform token
     */
    function setPTokenAddress(address _asset, address _pToken)
        external
        virtual
        onlyGovernor
    {
        _setPTokenAddress(_asset, _pToken);
    }

    /**
     * @notice Remove a supported asset by passing its index.
     *      This method can only be called by the system Governor
     * @param _assetIndex Index of the asset to be removed
     */
    function removePToken(uint256 _assetIndex) external virtual onlyGovernor {
        require(_assetIndex < assetsMapped.length, "Invalid index");
        address asset = assetsMapped[_assetIndex];
        address pToken = assetToPToken[asset];

        if (_assetIndex < assetsMapped.length - 1) {
            assetsMapped[_assetIndex] = assetsMapped[assetsMapped.length - 1];
        }
        assetsMapped.pop();
        assetToPToken[asset] = address(0);

        emit PTokenRemoved(asset, pToken);
    }

    /**
     * @notice Provide support for asset by passing its pToken address.
     *      Add to internal mappings and execute the platform specific,
     * abstract method `_abstractSetPToken`
     * @param _asset    Address for the asset
     * @param _pToken   Address for the corresponding platform token
     */
    function _setPTokenAddress(address _asset, address _pToken) internal {
        require(assetToPToken[_asset] == address(0), "pToken already set");
        require(
            _asset != address(0) && _pToken != address(0),
            "Invalid addresses"
        );

        assetToPToken[_asset] = _pToken;
        assetsMapped.push(_asset);

        emit PTokenAdded(_asset, _pToken);

        _abstractSetPToken(_asset, _pToken);
    }

    /**
     * @notice Transfer token to governor. Intended for recovering tokens stuck in
     *      strategy contracts, i.e. mistaken sends.
     * @param _asset Address for the asset
     * @param _amount Amount of the asset to transfer
     */
    function transferToken(address _asset, uint256 _amount)
        public
        virtual
        onlyGovernor
    {
        require(!supportsAsset(_asset), "Cannot transfer supported asset");
        IERC20(_asset).safeTransfer(governor(), _amount);
    }

    /**
     * @notice Set the Harvester contract that can collect rewards.
     * @param _harvesterAddress Address of the harvester contract.
     */
    function setHarvesterAddress(address _harvesterAddress)
        external
        onlyGovernorOrStrategist
    {
        emit HarvesterAddressesUpdated(harvesterAddress, _harvesterAddress);
        harvesterAddress = _harvesterAddress;
    }

    /***************************************
                 Abstract
    ****************************************/

    function _abstractSetPToken(address _asset, address _pToken)
        internal
        virtual;

    function safeApproveAllTokens() external virtual;

    /**
     * @notice Deposit an amount of assets into the platform
     * @param _asset               Address for the asset
     * @param _amount              Units of asset to deposit
     */
    function deposit(address _asset, uint256 _amount) external virtual;

    /**
     * @notice Deposit all supported assets in this strategy contract to the platform
     */
    function depositAll() external virtual;

    /**
     * @notice Withdraw an `amount` of assets from the platform and
     * send to the `_recipient`.
     * @param _recipient         Address to which the asset should be sent
     * @param _asset             Address of the asset
     * @param _amount            Units of asset to withdraw
     */
    function withdraw(
        address _recipient,
        address _asset,
        uint256 _amount
    ) external virtual;

    /**
     * @notice Withdraw all supported assets from platform and
     * sends to the OToken's Vault.
     */
    function withdrawAll() external virtual;

    /**
     * @notice Get the total asset value held in the platform.
     *      This includes any interest that was generated since depositing.
     * @param _asset      Address of the asset
     * @return balance    Total value of the asset in the platform
     */
    function checkBalance(address _asset)
        external
        view
        virtual
        returns (uint256 balance);

    /**
     * @notice Check if an asset is supported.
     * @param _asset    Address of the asset
     * @return bool     Whether asset is supported
     */
    function supportsAsset(address _asset) public view virtual returns (bool);
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

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title Generalized 4626 Strategy
 * @notice Investment strategy for ERC-4626 Tokenized Vaults
 * @dev This strategy should not be used for the Morpho V2 Vaults as those are not
 *      completley ERC-4626 compliant - they don't implement the maxWithdraw() and
 *      maxRedeem() functions and rather return 0 when any of them is called.
 * @author Origin Protocol Inc
 */
import { IERC4626 } from "../../lib/openzeppelin/interfaces/IERC4626.sol";
import { IERC20, InitializableAbstractStrategy } from "../utils/InitializableAbstractStrategy.sol";
import { IDistributor } from "../interfaces/IMerkl.sol";

contract Generalized4626Strategy is InitializableAbstractStrategy {
    /// @notice The address of the Merkle Distributor contract.
    IDistributor public constant merkleDistributor =
        IDistributor(0x3Ef3D8bA38EBe18DB133cEc108f4D14CE00Dd9Ae);

    /// @dev Replaced with an immutable variable
    // slither-disable-next-line constable-states
    address private _deprecate_shareToken;
    /// @dev Replaced with an immutable variable
    // slither-disable-next-line constable-states
    address private _deprecate_assetToken;

    IERC20 public immutable shareToken;
    IERC20 public immutable assetToken;

    // For future use
    uint256[50] private __gap;

    event ClaimedRewards(address indexed token, uint256 amount);

    /**
     * @param _baseConfig Base strategy config with platformAddress (ERC-4626 Vault contract), eg sfrxETH or sDAI,
     * and vaultAddress (OToken Vault contract), eg VaultProxy or OETHVaultProxy
     * @param _assetToken Address of the ERC-4626 asset token. eg frxETH or DAI
     */
    constructor(BaseStrategyConfig memory _baseConfig, address _assetToken)
        InitializableAbstractStrategy(_baseConfig)
    {
        shareToken = IERC20(_baseConfig.platformAddress);
        assetToken = IERC20(_assetToken);
    }

    function initialize() external virtual onlyGovernor initializer {
        address[] memory rewardTokens = new address[](0);
        address[] memory assets = new address[](1);
        address[] memory pTokens = new address[](1);

        assets[0] = address(assetToken);
        pTokens[0] = address(platformAddress);

        InitializableAbstractStrategy._initialize(
            rewardTokens,
            assets,
            pTokens
        );
    }

    /**
     * @dev Deposit assets by converting them to shares
     * @param _asset Address of asset to deposit
     * @param _amount Amount of asset to deposit
     */
    function deposit(address _asset, uint256 _amount)
        external
        virtual
        override
        onlyVault
        nonReentrant
    {
        _deposit(_asset, _amount);
    }

    /**
     * @dev Deposit assets by converting them to shares
     * @param _asset Address of asset to deposit
     * @param _amount Amount of asset to deposit
     */
    function _deposit(address _asset, uint256 _amount) internal virtual {
        require(_amount > 0, "Must deposit something");
        require(_asset == address(assetToken), "Unexpected asset address");

        // slither-disable-next-line unused-return
        IERC4626(platformAddress).deposit(_amount, address(this));
        emit Deposit(_asset, address(shareToken), _amount);
    }

    /**
     * @dev Deposit the entire balance of assetToken to gain shareToken
     */
    function depositAll() external virtual override onlyVault nonReentrant {
        uint256 balance = assetToken.balanceOf(address(this));
        if (balance > 0) {
            _deposit(address(assetToken), balance);
        }
    }

    /**
     * @dev Withdraw asset by burning shares
     * @param _recipient Address to receive withdrawn asset
     * @param _asset Address of asset to withdraw
     * @param _amount Amount of asset to withdraw
     */
    function withdraw(
        address _recipient,
        address _asset,
        uint256 _amount
    ) external virtual override onlyVault nonReentrant {
        _withdraw(_recipient, _asset, _amount);
    }

    function _withdraw(
        address _recipient,
        address _asset,
        uint256 _amount
    ) internal virtual {
        require(_amount > 0, "Must withdraw something");
        require(_recipient != address(0), "Must specify recipient");
        require(_asset == address(assetToken), "Unexpected asset address");

        // slither-disable-next-line unused-return
        IERC4626(platformAddress).withdraw(_amount, _recipient, address(this));
        emit Withdrawal(_asset, address(shareToken), _amount);
    }

    /**
     * @dev Internal method to respond to the addition of new asset / share tokens
     */
    function _abstractSetPToken(address, address) internal virtual override {
        _approveBase();
    }

    /**
     * @dev Remove all assets from platform and send them to Vault contract.
     */
    function withdrawAll()
        external
        virtual
        override
        onlyVaultOrGovernor
        nonReentrant
    {
        // @dev Don't use for Morpho V2 Vaults as below line will return 0
        uint256 sharesToRedeem = IERC4626(platformAddress).maxRedeem(
            address(this)
        );

        uint256 assetAmount = 0;
        if (sharesToRedeem > 0) {
            assetAmount = IERC4626(platformAddress).redeem(
                sharesToRedeem,
                vaultAddress,
                address(this)
            );
            emit Withdrawal(
                address(assetToken),
                address(shareToken),
                assetAmount
            );
        }
    }

    /**
     * @notice Get the total asset value held in the platform
     * @param _asset      Address of the asset
     * @return balance    Total value of the asset in the platform
     */
    function checkBalance(address _asset)
        public
        view
        virtual
        override
        returns (uint256 balance)
    {
        require(_asset == address(assetToken), "Unexpected asset address");
        /* We are intentionally not counting the amount of assetToken parked on the
         * contract toward the checkBalance. The deposit and withdraw functions
         * should not result in assetToken being unused and owned by this strategy
         * contract.
         */
        IERC4626 platform = IERC4626(platformAddress);
        return platform.previewRedeem(platform.balanceOf(address(this)));
    }

    /**
     * @notice Governor approves the ERC-4626 Tokenized Vault to spend the asset.
     */
    function safeApproveAllTokens() external override onlyGovernor {
        _approveBase();
    }

    function _approveBase() internal virtual {
        // Approval the asset to be transferred to the ERC-4626 Tokenized Vault.
        // Used by the ERC-4626 deposit() and mint() functions
        // slither-disable-next-line unused-return
        assetToken.approve(platformAddress, type(uint256).max);
    }

    /**
     * @dev Returns bool indicating whether asset is supported by strategy
     * @param _asset Address of the asset
     */
    function supportsAsset(address _asset)
        public
        view
        virtual
        override
        returns (bool)
    {
        return _asset == address(assetToken);
    }

    /**
     * @notice is not supported for this strategy as the asset and
     * ERC-4626 Tokenized Vault are set at deploy time.
     * @dev If the ERC-4626 Tokenized Vault needed to be changed, a new
     * contract would need to be deployed and the proxy updated.
     */
    function setPTokenAddress(address, address) external override onlyGovernor {
        revert("unsupported function");
    }

    /**
     * @notice is not supported for this strategy as the asset and
     * ERC-4626 Tokenized Vault are set at deploy time.
     * @dev If the ERC-4626 Tokenized Vault needed to be changed, a new
     * contract would need to be deployed and the proxy updated.
     */
    function removePToken(uint256) external override onlyGovernor {
        revert("unsupported function");
    }

    /// @notice Claim tokens from the Merkle Distributor
    /// @param token The address of the token to claim.
    /// @param amount The amount of tokens to claim.
    /// @param proof The Merkle proof to validate the claim.
    function merkleClaim(
        address token,
        uint256 amount,
        bytes32[] calldata proof
    ) external {
        address[] memory users = new address[](1);
        users[0] = address(this);

        address[] memory tokens = new address[](1);
        tokens[0] = token;

        uint256[] memory amounts = new uint256[](1);
        amounts[0] = amount;

        bytes32[][] memory proofs = new bytes32[][](1);
        proofs[0] = proof;

        merkleDistributor.claim(users, tokens, amounts, proofs);

        emit ClaimedRewards(token, amount);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title OToken VaultStorage contract
 * @notice The VaultStorage contract defines the storage for the Vault contracts
 * @author Origin Protocol Inc
 */

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { Address } from "@openzeppelin/contracts/utils/Address.sol";

import { IStrategy } from "../interfaces/IStrategy.sol";
import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { Governable } from "../governance/Governable.sol";
import { OUSD } from "../token/OUSD.sol";
import { Initializable } from "../utils/Initializable.sol";
import "../utils/Helpers.sol";

abstract contract VaultStorage is Initializable, Governable {
    using SafeERC20 for IERC20;

    event AssetAllocated(address _asset, address _strategy, uint256 _amount);
    event StrategyApproved(address _addr);
    event StrategyRemoved(address _addr);
    event Mint(address _addr, uint256 _value);
    event Redeem(address _addr, uint256 _value);
    event CapitalPaused();
    event CapitalUnpaused();
    event DefaultStrategyUpdated(address _strategy);
    event RebasePaused();
    event RebaseUnpaused();
    event VaultBufferUpdated(uint256 _vaultBuffer);
    event AllocateThresholdUpdated(uint256 _threshold);
    event StrategistUpdated(address _address);
    event MaxSupplyDiffChanged(uint256 maxSupplyDiff);
    event YieldDistribution(address _to, uint256 _yield, uint256 _fee);
    event TrusteeFeeBpsChanged(uint256 _basis);
    event TrusteeAddressChanged(address _address);
    event StrategyAddedToMintWhitelist(address indexed strategy);
    event StrategyRemovedFromMintWhitelist(address indexed strategy);
    event RebasePerSecondMaxChanged(uint256 rebaseRatePerSecond);
    event DripDurationChanged(uint256 dripDuration);
    event OperatorUpdated(address newOperator);
    event WithdrawalRequested(
        address indexed _withdrawer,
        uint256 indexed _requestId,
        uint256 _amount,
        uint256 _queued
    );
    event WithdrawalClaimed(
        address indexed _withdrawer,
        uint256 indexed _requestId,
        uint256 _amount
    );
    event WithdrawalClaimable(uint256 _claimable, uint256 _newClaimable);
    event WithdrawalClaimDelayUpdated(uint256 _newDelay);

    // Since we are proxy, all state should be uninitalized.
    // Since this storage contract does not have logic directly on it
    // we should not be checking for to see if these variables can be constant.
    // slither-disable-start uninitialized-state
    // slither-disable-start constable-states

    /// @dev mapping of supported vault assets to their configuration
    uint256 private _deprecated_assets;
    /// @dev list of all assets supported by the vault.
    address[] private _deprecated_allAssets;

    // Strategies approved for use by the Vault
    struct Strategy {
        bool isSupported;
        uint256 _deprecated; // Deprecated storage slot
    }
    /// @dev mapping of strategy contracts to their configuration
    mapping(address => Strategy) public strategies;
    /// @dev list of all vault strategies
    address[] internal allStrategies;

    /// @notice Address of the Oracle price provider contract
    address private _deprecated_priceProvider;
    /// @notice pause rebasing if true
    bool public rebasePaused;
    /// @notice pause operations that change the OToken supply.
    /// eg mint, redeem, allocate, mint/burn for strategy
    bool public capitalPaused;
    /// @notice Redemption fee in basis points. eg 50 = 0.5%
    uint256 private _deprecated_redeemFeeBps;
    /// @notice Percentage of assets to keep in Vault to handle (most) withdrawals. 100% = 1e18.
    uint256 public vaultBuffer;
    /// @notice OToken mints over this amount automatically allocate funds. 18 decimals.
    uint256 public autoAllocateThreshold;
    /// @dev Deprecated. Was the auto-rebase trigger threshold for mint/redeem.
    ///      Storage slot retained for proxy compatibility; no longer read or written.
    uint256 internal __deprecatedRebaseThreshold;

    /// @dev Address of the OToken token. eg OUSD or OETH.
    OUSD public oToken;

    /// @dev Address of the contract responsible for post rebase syncs with AMMs
    address private _deprecated_rebaseHooksAddr = address(0);

    /// @dev Deprecated: Address of Uniswap
    address private _deprecated_uniswapAddr = address(0);

    /// @notice Address of the Strategist
    address public strategistAddr = address(0);

    /// @notice Mapping of asset address to the Strategy that they should automatically
    // be allocated to
    uint256 private _deprecated_assetDefaultStrategies;

    /// @notice Max difference between total supply and total value of assets. 18 decimals.
    uint256 public maxSupplyDiff;

    /// @notice Trustee contract that can collect a percentage of yield
    address public trusteeAddress;

    /// @notice Amount of yield collected in basis points. eg 2000 = 20%
    uint256 public trusteeFeeBps;

    /// @dev Deprecated: Tokens that should be swapped for stablecoins
    address[] private _deprecated_swapTokens;

    /// @notice Metapool strategy that is allowed to mint/burn OTokens without changing collateral

    address private _deprecated_ousdMetaStrategy;

    /// @notice How much OTokens are currently minted by the strategy
    int256 private _deprecated_netOusdMintedForStrategy;

    /// @notice How much net total OTokens are allowed to be minted by all strategies
    uint256 private _deprecated_netOusdMintForStrategyThreshold;

    uint256 private _deprecated_swapConfig;

    // List of strategies that can mint oTokens directly
    // Used in OETHBaseVaultCore
    mapping(address => bool) public isMintWhitelistedStrategy;

    /// @notice Address of the Dripper contract that streams harvested rewards to the Vault
    /// @dev The vault is proxied so needs to be set with setDripper against the proxy contract.
    address private _deprecated_dripper;

    /// Withdrawal Queue Storage /////

    struct WithdrawalQueueMetadata {
        // cumulative total of all withdrawal requests included the ones that have already been claimed
        uint128 queued;
        // cumulative total of all the requests that can be claimed including the ones that have already been claimed
        uint128 claimable;
        // total of all the requests that have been claimed
        uint128 claimed;
        // index of the next withdrawal request starting at 0
        uint128 nextWithdrawalIndex;
    }

    /// @notice Global metadata for the withdrawal queue including:
    /// queued - cumulative total of all withdrawal requests included the ones that have already been claimed
    /// claimable - cumulative total of all the requests that can be claimed including the ones already claimed
    /// claimed - total of all the requests that have been claimed
    /// nextWithdrawalIndex - index of the next withdrawal request starting at 0
    WithdrawalQueueMetadata public withdrawalQueueMetadata;

    struct WithdrawalRequest {
        address withdrawer;
        bool claimed;
        uint40 timestamp; // timestamp of the withdrawal request
        // Amount of oTokens to redeem. eg OETH
        uint128 amount;
        // cumulative total of all withdrawal requests including this one.
        // this request can be claimed when this queued amount is less than or equal to the queue's claimable amount.
        uint128 queued;
    }

    /// @notice Mapping of withdrawal request indices to the user withdrawal request data
    mapping(uint256 => WithdrawalRequest) public withdrawalRequests;

    /// @notice Sets a minimum delay that is required to elapse between
    ///     requesting async withdrawals and claiming the request.
    ///     When set to 0 async withdrawals are disabled.
    uint256 public withdrawalClaimDelay;

    /// @notice Time in seconds that the vault last rebased yield.
    uint64 public lastRebase;

    /// @notice Automatic rebase yield calculations. In seconds. Set to 0 or 1 to disable.
    uint64 public dripDuration;

    /// @notice max rebase percentage per second
    ///   Can be used to set maximum yield of the protocol,
    ///   spreading out yield over time
    uint64 public rebasePerSecondMax;

    /// @notice target rebase rate limit, based on past rates and funds available.
    uint64 public rebasePerSecondTarget;

    uint256 internal constant MAX_REBASE = 0.02 ether;
    uint256 internal constant MAX_REBASE_PER_SECOND =
        uint256(0.05 ether) / 1 days;

    /// @notice Default strategy for asset
    address public defaultStrategy;

    /// @notice Address authorized to call `rebase()` directly. The Governor
    ///         and Strategist are always allowed in addition to this address.
    address public operatorAddr;

    // For future use
    uint256[41] private __gap;

    /// @notice Index of WETH asset in allAssets array
    /// Legacy OETHVaultCore code, relocated here for vault consistency.
    uint256 private _deprecated_wethAssetIndex;

    /// @dev Address of the asset (eg. WETH or USDC)
    address public immutable asset;
    uint8 internal immutable assetDecimals;

    // slither-disable-end constable-states
    // slither-disable-end uninitialized-state

    constructor(address _asset) {
        uint8 _decimals = IERC20Metadata(_asset).decimals();
        require(_decimals <= 18, "invalid asset decimals");
        asset = _asset;
        assetDecimals = _decimals;
    }

    /// @notice Deprecated: use `oToken()` instead.
    function oUSD() external view returns (OUSD) {
        return oToken;
    }
}

// SPDX-License-Identifier: MIT

// solhint-disable-next-line compiler-version
pragma solidity >=0.4.24 <0.8.0;

import "../utils/Address.sol";

/**
 * @dev This is a base contract to aid in writing upgradeable contracts, or any kind of contract that will be deployed
 * behind a proxy. Since a proxied contract can't have a constructor, it's common to move constructor logic to an
 * external initializer function, usually called `initialize`. It then becomes necessary to protect this initializer
 * function so it can only be called once. The {initializer} modifier provided by this contract will have this effect.
 *
 * TIP: To avoid leaving the proxy in an uninitialized state, the initializer function should be called as early as
 * possible by providing the encoded function call as the `_data` argument to {UpgradeableProxy-constructor}.
 *
 * CAUTION: When used with inheritance, manual care must be taken to not invoke a parent initializer twice, or to ensure
 * that all initializers are idempotent. This is not verified automatically as constructors are by Solidity.
 */
abstract contract Initializable {

    /**
     * @dev Indicates that the contract has been initialized.
     */
    bool private _initialized;

    /**
     * @dev Indicates that the contract is in the process of being initialized.
     */
    bool private _initializing;

    /**
     * @dev Modifier to protect an initializer function from being invoked twice.
     */
    modifier initializer() {
        require(_initializing || _isConstructor() || !_initialized, "Initializable: contract is already initialized");

        bool isTopLevelCall = !_initializing;
        if (isTopLevelCall) {
            _initializing = true;
            _initialized = true;
        }

        _;

        if (isTopLevelCall) {
            _initializing = false;
        }
    }

    /// @dev Returns true if and only if the function is running in the constructor
    function _isConstructor() private view returns (bool) {
        return !Address.isContract(address(this));
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

pragma solidity ^0.8.0;

import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IERC4626 is IERC20, IERC20Metadata {
    event Deposit(address indexed caller, address indexed owner, uint256 assets, uint256 shares);

    event Withdraw(
        address indexed caller,
        address indexed receiver,
        address indexed owner,
        uint256 assets,
        uint256 shares
    );

    /**
     * @dev Returns the address of the underlying token used for the Vault for accounting, depositing, and withdrawing.
     *
     * - MUST be an ERC-20 token contract.
     * - MUST NOT revert.
     */
    function asset() external view returns (address assetTokenAddress);

    /**
     * @dev Returns the total amount of the underlying asset that is “managed” by Vault.
     *
     * - SHOULD include any compounding that occurs from yield.
     * - MUST be inclusive of any fees that are charged against assets in the Vault.
     * - MUST NOT revert.
     */
    function totalAssets() external view returns (uint256 totalManagedAssets);

    /**
     * @dev Returns the amount of shares that the Vault would exchange for the amount of assets provided, in an ideal
     * scenario where all the conditions are met.
     *
     * - MUST NOT be inclusive of any fees that are charged against assets in the Vault.
     * - MUST NOT show any variations depending on the caller.
     * - MUST NOT reflect slippage or other on-chain conditions, when performing the actual exchange.
     * - MUST NOT revert.
     *
     * NOTE: This calculation MAY NOT reflect the “per-user” price-per-share, and instead should reflect the
     * “average-user’s” price-per-share, meaning what the average user should expect to see when exchanging to and
     * from.
     */
    function convertToShares(uint256 assets) external view returns (uint256 shares);

    /**
     * @dev Returns the amount of assets that the Vault would exchange for the amount of shares provided, in an ideal
     * scenario where all the conditions are met.
     *
     * - MUST NOT be inclusive of any fees that are charged against assets in the Vault.
     * - MUST NOT show any variations depending on the caller.
     * - MUST NOT reflect slippage or other on-chain conditions, when performing the actual exchange.
     * - MUST NOT revert.
     *
     * NOTE: This calculation MAY NOT reflect the “per-user” price-per-share, and instead should reflect the
     * “average-user’s” price-per-share, meaning what the average user should expect to see when exchanging to and
     * from.
     */
    function convertToAssets(uint256 shares) external view returns (uint256 assets);

    /**
     * @dev Returns the maximum amount of the underlying asset that can be deposited into the Vault for the receiver,
     * through a deposit call.
     *
     * - MUST return a limited value if receiver is subject to some deposit limit.
     * - MUST return 2 ** 256 - 1 if there is no limit on the maximum amount of assets that may be deposited.
     * - MUST NOT revert.
     */
    function maxDeposit(address receiver) external view returns (uint256 maxAssets);

    /**
     * @dev Allows an on-chain or off-chain user to simulate the effects of their deposit at the current block, given
     * current on-chain conditions.
     *
     * - MUST return as close to and no more than the exact amount of Vault shares that would be minted in a deposit
     *   call in the same transaction. I.e. deposit should return the same or more shares as previewDeposit if called
     *   in the same transaction.
     * - MUST NOT account for deposit limits like those returned from maxDeposit and should always act as though the
     *   deposit would be accepted, regardless if the user has enough tokens approved, etc.
     * - MUST be inclusive of deposit fees. Integrators should be aware of the existence of deposit fees.
     * - MUST NOT revert.
     *
     * NOTE: any unfavorable discrepancy between convertToShares and previewDeposit SHOULD be considered slippage in
     * share price or some other type of condition, meaning the depositor will lose assets by depositing.
     */
    function previewDeposit(uint256 assets) external view returns (uint256 shares);

    /**
     * @dev Mints shares Vault shares to receiver by depositing exactly amount of underlying tokens.
     *
     * - MUST emit the Deposit event.
     * - MAY support an additional flow in which the underlying tokens are owned by the Vault contract before the
     *   deposit execution, and are accounted for during deposit.
     * - MUST revert if all of assets cannot be deposited (due to deposit limit being reached, slippage, the user not
     *   approving enough underlying tokens to the Vault contract, etc).
     *
     * NOTE: most implementations will require pre-approval of the Vault with the Vault’s underlying asset token.
     */
    function deposit(uint256 assets, address receiver) external returns (uint256 shares);

    /**
     * @dev Returns the maximum amount of the Vault shares that can be minted for the receiver, through a mint call.
     * - MUST return a limited value if receiver is subject to some mint limit.
     * - MUST return 2 ** 256 - 1 if there is no limit on the maximum amount of shares that may be minted.
     * - MUST NOT revert.
     */
    function maxMint(address receiver) external view returns (uint256 maxShares);

    /**
     * @dev Allows an on-chain or off-chain user to simulate the effects of their mint at the current block, given
     * current on-chain conditions.
     *
     * - MUST return as close to and no fewer than the exact amount of assets that would be deposited in a mint call
     *   in the same transaction. I.e. mint should return the same or fewer assets as previewMint if called in the
     *   same transaction.
     * - MUST NOT account for mint limits like those returned from maxMint and should always act as though the mint
     *   would be accepted, regardless if the user has enough tokens approved, etc.
     * - MUST be inclusive of deposit fees. Integrators should be aware of the existence of deposit fees.
     * - MUST NOT revert.
     *
     * NOTE: any unfavorable discrepancy between convertToAssets and previewMint SHOULD be considered slippage in
     * share price or some other type of condition, meaning the depositor will lose assets by minting.
     */
    function previewMint(uint256 shares) external view returns (uint256 assets);

    /**
     * @dev Mints exactly shares Vault shares to receiver by depositing amount of underlying tokens.
     *
     * - MUST emit the Deposit event.
     * - MAY support an additional flow in which the underlying tokens are owned by the Vault contract before the mint
     *   execution, and are accounted for during mint.
     * - MUST revert if all of shares cannot be minted (due to deposit limit being reached, slippage, the user not
     *   approving enough underlying tokens to the Vault contract, etc).
     *
     * NOTE: most implementations will require pre-approval of the Vault with the Vault’s underlying asset token.
     */
    function mint(uint256 shares, address receiver) external returns (uint256 assets);

    /**
     * @dev Returns the maximum amount of the underlying asset that can be withdrawn from the owner balance in the
     * Vault, through a withdraw call.
     *
     * - MUST return a limited value if owner is subject to some withdrawal limit or timelock.
     * - MUST NOT revert.
     */
    function maxWithdraw(address owner) external view returns (uint256 maxAssets);

    /**
     * @dev Allows an on-chain or off-chain user to simulate the effects of their withdrawal at the current block,
     * given current on-chain conditions.
     *
     * - MUST return as close to and no fewer than the exact amount of Vault shares that would be burned in a withdraw
     *   call in the same transaction. I.e. withdraw should return the same or fewer shares as previewWithdraw if
     *   called
     *   in the same transaction.
     * - MUST NOT account for withdrawal limits like those returned from maxWithdraw and should always act as though
     *   the withdrawal would be accepted, regardless if the user has enough shares, etc.
     * - MUST be inclusive of withdrawal fees. Integrators should be aware of the existence of withdrawal fees.
     * - MUST NOT revert.
     *
     * NOTE: any unfavorable discrepancy between convertToShares and previewWithdraw SHOULD be considered slippage in
     * share price or some other type of condition, meaning the depositor will lose assets by depositing.
     */
    function previewWithdraw(uint256 assets) external view returns (uint256 shares);

    /**
     * @dev Burns shares from owner and sends exactly assets of underlying tokens to receiver.
     *
     * - MUST emit the Withdraw event.
     * - MAY support an additional flow in which the underlying tokens are owned by the Vault contract before the
     *   withdraw execution, and are accounted for during withdraw.
     * - MUST revert if all of assets cannot be withdrawn (due to withdrawal limit being reached, slippage, the owner
     *   not having enough shares, etc).
     *
     * Note that some implementations will require pre-requesting to the Vault before a withdrawal may be performed.
     * Those methods should be performed separately.
     */
    function withdraw(
        uint256 assets,
        address receiver,
        address owner
    ) external returns (uint256 shares);

    /**
     * @dev Returns the maximum amount of Vault shares that can be redeemed from the owner balance in the Vault,
     * through a redeem call.
     *
     * - MUST return a limited value if owner is subject to some withdrawal limit or timelock.
     * - MUST return balanceOf(owner) if owner is not subject to any withdrawal limit or timelock.
     * - MUST NOT revert.
     */
    function maxRedeem(address owner) external view returns (uint256 maxShares);

    /**
     * @dev Allows an on-chain or off-chain user to simulate the effects of their redeemption at the current block,
     * given current on-chain conditions.
     *
     * - MUST return as close to and no more than the exact amount of assets that would be withdrawn in a redeem call
     *   in the same transaction. I.e. redeem should return the same or more assets as previewRedeem if called in the
     *   same transaction.
     * - MUST NOT account for redemption limits like those returned from maxRedeem and should always act as though the
     *   redemption would be accepted, regardless if the user has enough shares, etc.
     * - MUST be inclusive of withdrawal fees. Integrators should be aware of the existence of withdrawal fees.
     * - MUST NOT revert.
     *
     * NOTE: any unfavorable discrepancy between convertToAssets and previewRedeem SHOULD be considered slippage in
     * share price or some other type of condition, meaning the depositor will lose assets by redeeming.
     */
    function previewRedeem(uint256 shares) external view returns (uint256 assets);

    /**
     * @dev Burns exactly shares from owner and sends assets of underlying tokens to receiver.
     *
     * - MUST emit the Withdraw event.
     * - MAY support an additional flow in which the underlying tokens are owned by the Vault contract before the
     *   redeem execution, and are accounted for during redeem.
     * - MUST revert if all of shares cannot be redeemed (due to withdrawal limit being reached, slippage, the owner
     *   not having enough shares, etc).
     *
     * NOTE: some implementations will require pre-requesting to the Vault before a withdrawal may be performed.
     * Those methods should be performed separately.
     */
    function redeem(
        uint256 shares,
        address receiver,
        address owner
    ) external returns (uint256 assets);
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IBasicToken {
    function symbol() external view returns (string memory);

    function decimals() external view returns (uint8);
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

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

interface IDistributor {
    event Claimed(address indexed user, address indexed token, uint256 amount);

    function claim(
        address[] calldata users,
        address[] calldata tokens,
        uint256[] calldata amounts,
        bytes32[][] calldata proofs
    ) external;
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { IERC4626 } from "../../../lib/openzeppelin/interfaces/IERC4626.sol";

interface IVaultV2 is IERC4626 {
    function liquidityAdapter() external view returns (address);
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

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

interface IMorphoV2Adapter {
    // address of the underlying vault
    function morphoVaultV1() external view returns (address);

    // address of the parent Morpho V2 vault
    function parentVault() external view returns (address);
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title Platform interface to integrate with lending platform like Compound, AAVE etc.
 */
interface IStrategy {
    /**
     * @dev Deposit the given asset to platform
     * @param _asset asset address
     * @param _amount Amount to deposit
     */
    function deposit(address _asset, uint256 _amount) external;

    /**
     * @dev Deposit the entire balance of all supported assets in the Strategy
     *      to the platform
     */
    function depositAll() external;

    /**
     * @dev Withdraw given asset from Lending platform
     */
    function withdraw(
        address _recipient,
        address _asset,
        uint256 _amount
    ) external;

    /**
     * @dev Liquidate all assets in strategy and return them to Vault.
     */
    function withdrawAll() external;

    /**
     * @dev Returns the current balance of the given asset.
     */
    function checkBalance(address _asset)
        external
        view
        returns (uint256 balance);

    /**
     * @dev Returns bool indicating whether strategy supports asset.
     */
    function supportsAsset(address _asset) external view returns (bool);

    /**
     * @dev Collect reward tokens from the Strategy.
     */
    function collectRewardTokens() external;

    /**
     * @dev The address array of the reward tokens for the Strategy.
     */
    function getRewardTokenAddresses() external view returns (address[] memory);

    function harvesterAddress() external view returns (address);

    function transferToken(address token, uint256 amount) external;

    function setRewardTokenAddresses(address[] calldata _rewardTokenAddresses)
        external;
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { VaultStorage } from "../vault/VaultStorage.sol";

interface IVault {
    // slither-disable-start constable-states

    event AssetAllocated(address _asset, address _strategy, uint256 _amount);
    event StrategyApproved(address _addr);
    event StrategyRemoved(address _addr);
    event Mint(address _addr, uint256 _value);
    event Redeem(address _addr, uint256 _value);
    event CapitalPaused();
    event CapitalUnpaused();
    event DefaultStrategyUpdated(address _strategy);
    event RebasePaused();
    event RebaseUnpaused();
    event VaultBufferUpdated(uint256 _vaultBuffer);
    event AllocateThresholdUpdated(uint256 _threshold);
    event StrategistUpdated(address _address);
    event MaxSupplyDiffChanged(uint256 maxSupplyDiff);
    event YieldDistribution(address _to, uint256 _yield, uint256 _fee);
    event TrusteeFeeBpsChanged(uint256 _basis);
    event TrusteeAddressChanged(address _address);
    event StrategyAddedToMintWhitelist(address indexed strategy);
    event StrategyRemovedFromMintWhitelist(address indexed strategy);
    event RebasePerSecondMaxChanged(uint256 rebaseRatePerSecond);
    event DripDurationChanged(uint256 dripDuration);
    event WithdrawalRequested(
        address indexed _withdrawer,
        uint256 indexed _requestId,
        uint256 _amount,
        uint256 _queued
    );
    event WithdrawalClaimed(
        address indexed _withdrawer,
        uint256 indexed _requestId,
        uint256 _amount
    );
    event WithdrawalClaimable(uint256 _claimable, uint256 _newClaimable);
    event WithdrawalClaimDelayUpdated(uint256 _newDelay);

    // Governable.sol
    function transferGovernance(address _newGovernor) external;

    function claimGovernance() external;

    function governor() external view returns (address);

    // VaultAdmin.sol
    function setVaultBuffer(uint256 _vaultBuffer) external;

    function vaultBuffer() external view returns (uint256);

    function setAutoAllocateThreshold(uint256 _threshold) external;

    function autoAllocateThreshold() external view returns (uint256);

    function setStrategistAddr(address _address) external;

    function strategistAddr() external view returns (address);

    function setOperatorAddr(address _operator) external;

    function operatorAddr() external view returns (address);

    function setMaxSupplyDiff(uint256 _maxSupplyDiff) external;

    function maxSupplyDiff() external view returns (uint256);

    function setTrusteeAddress(address _address) external;

    function trusteeAddress() external view returns (address);

    function setTrusteeFeeBps(uint256 _basis) external;

    function trusteeFeeBps() external view returns (uint256);

    function approveStrategy(address _addr) external;

    function removeStrategy(address _addr) external;

    function setDefaultStrategy(address _strategy) external;

    function defaultStrategy() external view returns (address);

    function pauseRebase() external;

    function unpauseRebase() external;

    function rebasePaused() external view returns (bool);

    function pauseCapital() external;

    function unpauseCapital() external;

    function capitalPaused() external view returns (bool);

    function transferToken(address _asset, uint256 _amount) external;

    function withdrawAllFromStrategy(address _strategyAddr) external;

    function withdrawAllFromStrategies() external;

    function withdrawFromStrategy(
        address _strategyFromAddress,
        address[] calldata _assets,
        uint256[] calldata _amounts
    ) external;

    function depositToStrategy(
        address _strategyToAddress,
        address[] calldata _assets,
        uint256[] calldata _amounts
    ) external;

    // VaultCore.sol
    function mint(uint256 _amount) external;

    function mintForStrategy(uint256 _amount) external;

    function burnForStrategy(uint256 _amount) external;

    function allocate() external;

    function rebase() external;

    function totalValue() external view returns (uint256 value);

    function checkBalance(address _asset) external view returns (uint256);

    function getAssetCount() external view returns (uint256);

    function getAllAssets() external view returns (address[] memory);

    function getStrategyCount() external view returns (uint256);

    function getAllStrategies() external view returns (address[] memory);

    function strategies(address _addr)
        external
        view
        returns (VaultStorage.Strategy memory);

    /// @notice Deprecated: use `asset()` instead.
    function isSupportedAsset(address _asset) external view returns (bool);

    function asset() external view returns (address);

    function oToken() external view returns (address);

    function initialize(address) external;

    function addWithdrawalQueueLiquidity() external;

    function requestWithdrawal(uint256 _amount)
        external
        returns (uint256 requestId, uint256 queued);

    function claimWithdrawal(uint256 requestId)
        external
        returns (uint256 amount);

    function claimWithdrawals(uint256[] memory requestIds)
        external
        returns (uint256[] memory amounts, uint256 totalAmount);

    function withdrawalQueueMetadata()
        external
        view
        returns (VaultStorage.WithdrawalQueueMetadata memory);

    function withdrawalRequests(uint256 requestId)
        external
        view
        returns (VaultStorage.WithdrawalRequest memory);

    function addStrategyToMintWhitelist(address strategyAddr) external;

    function removeStrategyFromMintWhitelist(address strategyAddr) external;

    function isMintWhitelistedStrategy(address strategyAddr)
        external
        view
        returns (bool);

    function withdrawalClaimDelay() external view returns (uint256);

    function setWithdrawalClaimDelay(uint256 newDelay) external;

    function lastRebase() external view returns (uint64);

    function dripDuration() external view returns (uint64);

    function setDripDuration(uint256 _dripDuration) external;

    function rebasePerSecondMax() external view returns (uint64);

    function setRebaseRateMax(uint256 yearlyApr) external;

    function rebasePerSecondTarget() external view returns (uint64);

    function previewYield() external view returns (uint256 yield);

    // slither-disable-end constable-states
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

import { IVault } from "./IVault.sol";

interface IMockVault is IVault {
    function outstandingWithdrawalsAmount() external view returns (uint256);

    function wethAvailable() external view returns (uint256);
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


## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

