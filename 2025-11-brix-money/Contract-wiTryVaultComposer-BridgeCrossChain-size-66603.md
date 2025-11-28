
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {VaultComposerSync} from "./libraries/VaultComposerSync.sol";
import {SendParam, MessagingFee, IOFT} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oft/interfaces/IOFT.sol";
import {OFTComposeMsgCodec} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oft/libs/OFTComposeMsgCodec.sol";
import {OptionsBuilder} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oapp/libs/OptionsBuilder.sol";
import {IStakediTryCrosschain} from "../interfaces/IStakediTryCrosschain.sol";
import {IwiTryVaultComposer} from "./interfaces/IwiTryVaultComposer.sol";
import {OApp} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oapp/OApp.sol";
import {Origin} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oapp/interfaces/IOAppReceiver.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {StakediTryCrosschain} from "../StakediTryCrosschain.sol";
import {IUnstakeMessenger} from "./interfaces/IUnstakeMessenger.sol";

/**
 * @title wiTryVaultComposer - Async Cooldown Vault Composer
 * @author Inverter Network
 * @notice wiTryVaultComposer that supports deposit-and-send and async cooldown-based redemption
 * @dev Extends VaultComposerSync with custom redemption logic for StakediTryCrosschain vault
 *      Deposits are instant and shares can be sent cross-chain immediately
 *      Redemptions require a cooldown period before claiming assets
 *      OApp inheritance allows direct LayerZero messages for unstake operations
 */
contract wiTryVaultComposer is VaultComposerSync, IwiTryVaultComposer, OApp {
    using OFTComposeMsgCodec for bytes32;
    using OptionsBuilder for bytes;
    using SafeERC20 for IERC20;

    // ============ CONSTANTS ============

    /**
     * @notice Message type constant for unstake operations
     * @dev Used for routing direct _lzReceive messages from spoke chains
     * @dev Distinct from compose flow which uses oftCmd strings
     */
    uint16 public constant MSG_TYPE_UNSTAKE = 1;

    /**
     * @notice Initializes the wiTryVaultComposer contract
     * @param _vault The address of the StakediTryCrosschain vault contract
     * @param _assetOFT The address of the iTRY asset OFT contract
     * @param _shareOFT The address of the wiTRY share OFT contract
     * @param _endpoint The LayerZero endpoint address
     * @dev Initializes both VaultComposerSync and OApp functionality
     *      Endpoint must be passed explicitly to ensure OApp initialization happens correctly
     */
    constructor(address _vault, address _assetOFT, address _shareOFT, address _endpoint)
        VaultComposerSync(_vault, _assetOFT, _shareOFT)
        OApp(_endpoint, msg.sender)
    {}

    /**
     * @notice Handles composed cross-chain operations with oftCmd routing
     * @param _oftIn The OFT token received in lzReceive
     * @param _composeFrom The bytes32 identifier of the compose sender
     * @param _composeMsg The encoded message containing SendParam and minMsgValue
     * @param _amount The amount of tokens received
     */
    function handleCompose(address _oftIn, bytes32 _composeFrom, bytes memory _composeMsg, uint256 _amount)
        external
        payable
        override
    {
        if (msg.sender != address(this)) revert OnlySelf(msg.sender);

        (SendParam memory sendParam, uint256 minMsgValue) = abi.decode(_composeMsg, (SendParam, uint256));
        if (msg.value < minMsgValue) revert InsufficientMsgValue(minMsgValue, msg.value);

        if (_oftIn == ASSET_OFT) {
            _depositAndSend(_composeFrom, _amount, sendParam, address(this));
        } else if (_oftIn == SHARE_OFT) {
            if (keccak256(sendParam.oftCmd) == keccak256("INITIATE_COOLDOWN")) {
                _initiateCooldown(_composeFrom, _amount);
            } else if (keccak256(sendParam.oftCmd) == keccak256("FAST_REDEEM")) {
                _fastRedeem(_composeFrom, _amount, sendParam, address(this));
            } else {
                revert InitiateCooldownRequired();
            }
        } else {
            revert OnlyValidComposeCaller(_oftIn);
        }
    }

    /**
     * @notice Initiates async redemption via cooldown mechanism
     * @param _redeemer The bytes32 address of the redeemer
     * @param _shareAmount The number of shares to redeem
     */
    function _initiateCooldown(bytes32 _redeemer, uint256 _shareAmount) internal virtual {
        address redeemer = _redeemer.bytes32ToAddress();
        if (redeemer == address(0)) revert InvalidZeroAddress();
        uint256 assetAmount = IStakediTryCrosschain(address(VAULT)).cooldownSharesByComposer(_shareAmount, redeemer);
        emit CooldownInitiated(_redeemer, redeemer, _shareAmount, assetAmount);
    }


    /**
     * @notice Fast redeem shares for immediate withdrawal with a fee
     * @param _redeemer The bytes32 address of the redeemer
     * @param _shareAmount The number of shares to redeem
     * @param _sendParam The send parameters for the crosschain transfer
     * @param _refundAddress The address to receive the refund
     */
    function _fastRedeem(bytes32 _redeemer, uint256 _shareAmount, SendParam memory _sendParam, address _refundAddress) internal virtual {
         address redeemer = _redeemer.bytes32ToAddress();
        if (redeemer == address(0)) revert InvalidZeroAddress();

        uint256 assets = IStakediTryCrosschain(address(VAULT)).fastRedeemThroughComposer(_shareAmount, redeemer, redeemer); // redeemer is the owner and crosschain receiver

          if (assets == 0) {
            revert NoAssetsToRedeem();
        }

        _sendParam.amountLD = assets;
        _sendParam.minAmountLD = assets;

        _send(ASSET_OFT, _sendParam, _refundAddress);

        // Emit success event
        emit CrosschainFastRedeemProcessed(redeemer, _sendParam.dstEid, _shareAmount, assets);

    }

    /**
     * @notice Synchronous redemption is not supported
     * @dev Always reverts - use INITIATE_COOLDOWN command instead
     */
    function _redeemAndSend(
        bytes32,
        /*_redeemer*/
        uint256,
        /*_shareAmount*/
        SendParam memory,
        /*_sendParam*/
        address /*_refundAddress*/
    ) internal virtual override {
        revert SyncRedemptionNotSupported();
    }

    /**
     * @notice Override _refund to include valid extraOptions
     * @dev Ensures extraOptions contains valid TYPE_3 options for LayerZero
     * @param _oft The OFT contract address used for refunding
     * @param _message The original message that was sent
     * @param _amount The amount of tokens to refund
     * @param _refundAddress Address to receive the refund
     */
    function _refund(address _oft, bytes calldata _message, uint256 _amount, address _refundAddress)
        internal
        virtual
        override
    {
        SendParam memory refundSendParam;
        refundSendParam.dstEid = OFTComposeMsgCodec.srcEid(_message);
        refundSendParam.to = OFTComposeMsgCodec.composeFrom(_message);
        refundSendParam.amountLD = _amount;
        refundSendParam.extraOptions = OptionsBuilder.newOptions(); // Add valid TYPE_3 options header (0x0003)

        IOFT(_oft).send{value: msg.value}(refundSendParam, MessagingFee(msg.value, 0), _refundAddress);
    }

    // ============ OAPP INTEGRATION ============

    /**
     * @notice Allow contract to receive ETH for LayerZero operations and refunds
     * @dev Required for:
     *      1. Receiving LayerZero fee refunds from crosschain operations (hub→spoke dust)
     *      2. Accepting ETH for gas forwarding in crosschain unstaking
     *      3. Emergency recovery scenarios
     * @dev wiTryVaultComposer is the refund address for unstake operations to maintain
     *      clean separation between crosschain logic and staking vault logic
     * @dev Overrides parent VaultComposerSync's receive() function
     */
    receive() external payable override {}

    /**
     * @notice Rescue tokens accidentally sent to this contract
     * @dev Only callable by owner. Can rescue both ERC20 tokens and native ETH
     *      Use address(0) for rescuing ETH
     * @param token The token address to rescue (use address(0) for ETH)
     * @param to The address to send rescued tokens to
     * @param amount The amount to rescue
     */
    function rescueToken(address token, address to, uint256 amount) external onlyOwner nonReentrant {
        if (to == address(0)) revert InvalidZeroAddress();
        if (amount == 0) revert InvalidAmount();

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
     * @notice Receive crosschain messages from spoke chains
     * @dev Routes messages by type to appropriate handlers
     * @dev SECURITY: LayerZero OApp validates peers before calling _lzReceive()
     *      The authorization model relies on the spoke chain's UnstakeMessenger
     *      validating that only the token owner can initiate unstaking.
     * @param _origin Origin information (source chain and sender)
     * @param _guid Message GUID
     * @param _message Encoded message payload
     * @param _executor Executor address
     * @param _extraData Extra data from LayerZero
     */
    function _lzReceive(
        Origin calldata _origin,
        bytes32 _guid,
        bytes calldata _message,
        address _executor,
        bytes calldata _extraData
    ) internal override {
        // Note: LayerZero OApp handles peer validation before calling _lzReceive().
        // Peer validation is redundant here as the OApp base contract already ensures
        // messages only come from authorized peers configured via setPeer().

        // Decode and route message
        (uint16 msgType, IUnstakeMessenger.UnstakeMessage memory unstakeMsg) =
            abi.decode(_message, (uint16, IUnstakeMessenger.UnstakeMessage));

        if (msgType == MSG_TYPE_UNSTAKE) {
            _handleUnstake(_origin, _guid, unstakeMsg);
        } else {
            revert UnknownMessageType(msgType);
        }
    }

    /**
     * @notice Process unstake requests from spoke chains
     * @dev Validates user address and cooldown state, calls vault to unstake,
     *      and sends the unstaked iTRY assets back to the user on the spoke chain
     * @param _origin Origin information (source chain and sender)
     * @param _guid Message GUID
     * @param unstakeMsg Unstake message containing user address and extraOptions
     */
    function _handleUnstake(Origin calldata _origin, bytes32 _guid, IUnstakeMessenger.UnstakeMessage memory unstakeMsg)
        internal
        virtual
    {
        address user = unstakeMsg.user;

        // Validate user
        if (user == address(0)) revert InvalidZeroAddress();
        if (_origin.srcEid == 0) revert InvalidOrigin();

        // Call vault to unstake
        uint256 assets = IStakediTryCrosschain(address(VAULT)).unstakeThroughComposer(user);

        if (assets == 0) {
            revert NoAssetsToUnstake();
        }

        // Build send parameters and send assets back to spoke chain
        bytes memory options = OptionsBuilder.newOptions();

        SendParam memory _sendParam = SendParam({
            dstEid: _origin.srcEid,
            to: bytes32(uint256(uint160(user))),
            amountLD: assets,
            minAmountLD: assets,
            extraOptions: options,
            composeMsg: "",
            oftCmd: ""
        });

        _send(ASSET_OFT, _sendParam, address(this));

        // Emit success event
        emit CrosschainUnstakeProcessed(user, _origin.srcEid, assets, _guid);
    }

    /**
     * @notice Quote fee for unstake return leg (hub→spoke)
     * @dev Queries the adapter to get LayerZero fee for sending iTRY back to spoke chain
     *      Used by test scripts and potential frontend to calculate total round-trip cost
     * @dev Gas Consideration:
     * - Enforced options on adapter already include 200k gas
     * - Quote automatically includes gas cost
     * - No need to calculate vault execution gas (already happened on hub)
     *
     * @param to Address to receive iTRY on destination chain
     * @param amount Amount of iTRY to send
     * @param dstEid Destination endpoint ID (spoke chain)
     * @return nativeFee Fee in native token for hub→spoke leg
     * @return lzTokenFee Fee in LayerZero token (typically 0)
     */
    function quoteUnstakeReturn(address to, uint256 amount, uint32 dstEid)
        external
        view
        returns (uint256 nativeFee, uint256 lzTokenFee)
    {
        // Validate inputs
        if (to == address(0)) revert InvalidZeroAddress();
        if (amount == 0) revert InvalidAmount();
        if (dstEid == 0) revert InvalidDestination();

        // Build send parameters for vault composer's _send() function
        SendParam memory sendParam = SendParam({
            dstEid: dstEid,
            to: bytes32(uint256(uint160(to))),
            amountLD: amount,
            minAmountLD: amount,
            extraOptions: OptionsBuilder.newOptions(), // Adapter enforced options provide gas
            composeMsg: "",
            oftCmd: ""
        });

        // Quote the fee from the adapter
        // Note: This uses the adapter's quoteSend (OFT), not wiTryVaultComposer's _quote (OApp)
        // The adapter already has enforced options with 200k gas set
        MessagingFee memory fee = IOFT(ASSET_OFT).quoteSend(sendParam, false);

        return (fee.nativeFee, fee.lzTokenFee);
    }

    /**
     * @notice Quote fee for fast redeem return leg (spoke→hub)
     * @dev Queries the adapter to get LayerZero fee for sending iTRY back to hub chain
     *      Used by test scripts and potential frontend to calculate total round-trip cost
     * @param to Address to receive iTRY on hub chain
     * @param amount Amount of iTRY to send
     * @param dstEid Destination endpoint ID (hub chain)
     * @return nativeFee Fee in native token for spoke→hub leg
     * @return lzTokenFee Fee in LayerZero token (typically 0)
     */
    function quoteFastRedeemReturn(address to, uint256 amount, uint32 dstEid)
        external
        view
        returns (uint256 nativeFee, uint256 lzTokenFee)
    {
        // Validate inputs
        if (to == address(0)) revert InvalidZeroAddress();
        if (amount == 0) revert InvalidAmount();
        if (dstEid == 0) revert InvalidDestination();

        // Build send parameters for vault composer's _send() function
        SendParam memory sendParam = SendParam({
            dstEid: dstEid,
            to: bytes32(uint256(uint160(to))),
            amountLD: amount,
            minAmountLD: amount,
            extraOptions: OptionsBuilder.newOptions(), // Adapter enforced options provide gas
            composeMsg: "",
            oftCmd: ""
        });

        // Quote the fee from the adapter
        // Note: This uses the adapter's quoteSend (OFT), not wiTryVaultComposer's _quote (OApp)
        // The adapter already has enforced options with 200k gas set
        MessagingFee memory fee = IOFT(ASSET_OFT).quoteSend(sendParam, false);

        return (fee.nativeFee, fee.lzTokenFee);
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

library OFTComposeMsgCodec {
    // Offset constants for decoding composed messages
    uint8 private constant NONCE_OFFSET = 8;
    uint8 private constant SRC_EID_OFFSET = 12;
    uint8 private constant AMOUNT_LD_OFFSET = 44;
    uint8 private constant COMPOSE_FROM_OFFSET = 76;

    /**
     * @dev Encodes a OFT composed message.
     * @param _nonce The nonce value.
     * @param _srcEid The source endpoint ID.
     * @param _amountLD The amount in local decimals.
     * @param _composeMsg The composed message.
     * @return _msg The encoded Composed message.
     */
    function encode(
        uint64 _nonce,
        uint32 _srcEid,
        uint256 _amountLD,
        bytes memory _composeMsg // 0x[composeFrom][composeMsg]
    ) internal pure returns (bytes memory _msg) {
        _msg = abi.encodePacked(_nonce, _srcEid, _amountLD, _composeMsg);
    }

    /**
     * @dev Retrieves the nonce from the composed message.
     * @param _msg The message.
     * @return The nonce value.
     */
    function nonce(bytes calldata _msg) internal pure returns (uint64) {
        return uint64(bytes8(_msg[:NONCE_OFFSET]));
    }

    /**
     * @dev Retrieves the source endpoint ID from the composed message.
     * @param _msg The message.
     * @return The source endpoint ID.
     */
    function srcEid(bytes calldata _msg) internal pure returns (uint32) {
        return uint32(bytes4(_msg[NONCE_OFFSET:SRC_EID_OFFSET]));
    }

    /**
     * @dev Retrieves the amount in local decimals from the composed message.
     * @param _msg The message.
     * @return The amount in local decimals.
     */
    function amountLD(bytes calldata _msg) internal pure returns (uint256) {
        return uint256(bytes32(_msg[SRC_EID_OFFSET:AMOUNT_LD_OFFSET]));
    }

    /**
     * @dev Retrieves the composeFrom value from the composed message.
     * @param _msg The message.
     * @return The composeFrom value.
     */
    function composeFrom(bytes calldata _msg) internal pure returns (bytes32) {
        return bytes32(_msg[AMOUNT_LD_OFFSET:COMPOSE_FROM_OFFSET]);
    }

    /**
     * @dev Retrieves the composed message.
     * @param _msg The message.
     * @return The composed message.
     */
    function composeMsg(bytes calldata _msg) internal pure returns (bytes memory) {
        return _msg[COMPOSE_FROM_OFFSET:];
    }

    /**
     * @dev Converts an address to bytes32.
     * @param _addr The address to convert.
     * @return The bytes32 representation of the address.
     */
    function addressToBytes32(address _addr) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(_addr)));
    }

    /**
     * @dev Converts bytes32 to an address.
     * @param _b The bytes32 value to convert.
     * @return The address representation of bytes32.
     */
    function bytes32ToAddress(bytes32 _b) internal pure returns (address) {
        return address(uint160(uint256(_b)));
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

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import { MessagingReceipt, MessagingFee } from "../../oapp/OAppSender.sol";

/**
 * @dev Struct representing token parameters for the OFT send() operation.
 */
struct SendParam {
    uint32 dstEid; // Destination endpoint ID.
    bytes32 to; // Recipient address.
    uint256 amountLD; // Amount to send in local decimals.
    uint256 minAmountLD; // Minimum amount to send in local decimals.
    bytes extraOptions; // Additional options supplied by the caller to be used in the LayerZero message.
    bytes composeMsg; // The composed message for the send() operation.
    bytes oftCmd; // The OFT command to be executed, unused in default OFT implementations.
}

/**
 * @dev Struct representing OFT limit information.
 * @dev These amounts can change dynamically and are up the the specific oft implementation.
 */
struct OFTLimit {
    uint256 minAmountLD; // Minimum amount in local decimals that can be sent to the recipient.
    uint256 maxAmountLD; // Maximum amount in local decimals that can be sent to the recipient.
}

/**
 * @dev Struct representing OFT receipt information.
 */
struct OFTReceipt {
    uint256 amountSentLD; // Amount of tokens ACTUALLY debited from the sender in local decimals.
    // @dev In non-default implementations, the amountReceivedLD COULD differ from this value.
    uint256 amountReceivedLD; // Amount of tokens to be received on the remote side.
}

/**
 * @dev Struct representing OFT fee details.
 * @dev Future proof mechanism to provide a standardized way to communicate fees to things like a UI.
 */
struct OFTFeeDetail {
    int256 feeAmountLD; // Amount of the fee in local decimals.
    string description; // Description of the fee.
}

/**
 * @title IOFT
 * @dev Interface for the OftChain (OFT) token.
 * @dev Does not inherit ERC20 to accommodate usage by OFTAdapter as well.
 * @dev This specific interface ID is '0x02e49c2c'.
 */
interface IOFT {
    // Custom error messages
    error InvalidLocalDecimals();
    error SlippageExceeded(uint256 amountLD, uint256 minAmountLD);

    // Events
    event OFTSent(
        bytes32 indexed guid, // GUID of the OFT message.
        uint32 dstEid, // Destination Endpoint ID.
        address indexed fromAddress, // Address of the sender on the src chain.
        uint256 amountSentLD, // Amount of tokens sent in local decimals.
        uint256 amountReceivedLD // Amount of tokens received in local decimals.
    );
    event OFTReceived(
        bytes32 indexed guid, // GUID of the OFT message.
        uint32 srcEid, // Source Endpoint ID.
        address indexed toAddress, // Address of the recipient on the dst chain.
        uint256 amountReceivedLD // Amount of tokens received in local decimals.
    );

    /**
     * @notice Retrieves interfaceID and the version of the OFT.
     * @return interfaceId The interface ID.
     * @return version The version.
     *
     * @dev interfaceId: This specific interface ID is '0x02e49c2c'.
     * @dev version: Indicates a cross-chain compatible msg encoding with other OFTs.
     * @dev If a new feature is added to the OFT cross-chain msg encoding, the version will be incremented.
     * ie. localOFT version(x,1) CAN send messages to remoteOFT version(x,1)
     */
    function oftVersion() external view returns (bytes4 interfaceId, uint64 version);

    /**
     * @notice Retrieves the address of the token associated with the OFT.
     * @return token The address of the ERC20 token implementation.
     */
    function token() external view returns (address);

    /**
     * @notice Indicates whether the OFT contract requires approval of the 'token()' to send.
     * @return requiresApproval Needs approval of the underlying token implementation.
     *
     * @dev Allows things like wallet implementers to determine integration requirements,
     * without understanding the underlying token implementation.
     */
    function approvalRequired() external view returns (bool);

    /**
     * @notice Retrieves the shared decimals of the OFT.
     * @return sharedDecimals The shared decimals of the OFT.
     */
    function sharedDecimals() external view returns (uint8);

    /**
     * @notice Provides a quote for OFT-related operations.
     * @param _sendParam The parameters for the send operation.
     * @return limit The OFT limit information.
     * @return oftFeeDetails The details of OFT fees.
     * @return receipt The OFT receipt information.
     */
    function quoteOFT(
        SendParam calldata _sendParam
    ) external view returns (OFTLimit memory, OFTFeeDetail[] memory oftFeeDetails, OFTReceipt memory);

    /**
     * @notice Provides a quote for the send() operation.
     * @param _sendParam The parameters for the send() operation.
     * @param _payInLzToken Flag indicating whether the caller is paying in the LZ token.
     * @return fee The calculated LayerZero messaging fee from the send() operation.
     *
     * @dev MessagingFee: LayerZero msg fee
     *  - nativeFee: The native fee.
     *  - lzTokenFee: The lzToken fee.
     */
    function quoteSend(SendParam calldata _sendParam, bool _payInLzToken) external view returns (MessagingFee memory);

    /**
     * @notice Executes the send() operation.
     * @param _sendParam The parameters for the send operation.
     * @param _fee The fee information supplied by the caller.
     *      - nativeFee: The native fee.
     *      - lzTokenFee: The lzToken fee.
     * @param _refundAddress The address to receive any excess funds from fees etc. on the src.
     * @return receipt The LayerZero messaging receipt from the send() operation.
     * @return oftReceipt The OFT receipt information.
     *
     * @dev MessagingReceipt: LayerZero msg receipt
     *  - guid: The unique identifier for the sent message.
     *  - nonce: The nonce of the sent message.
     *  - fee: The LayerZero fee incurred for the message.
     */
    function send(
        SendParam calldata _sendParam,
        MessagingFee calldata _fee,
        address _refundAddress
    ) external payable returns (MessagingReceipt memory, OFTReceipt memory);
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts v4.4.1 (utils/Context.sol)

pragma solidity ^0.8.0;

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

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {SafeERC20, IERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {ERC4626, IERC4626} from "@openzeppelin/contracts/token/ERC20/extensions/ERC4626.sol";

import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";

import {IOFT, SendParam, MessagingFee} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oft/interfaces/IOFT.sol";
import {IOAppCore} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oapp/interfaces/IOAppCore.sol";
import {ILayerZeroEndpointV2} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import {OFTComposeMsgCodec} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oft/libs/OFTComposeMsgCodec.sol";

import {IVaultComposerSync} from "./IVaultComposerSync.sol";

/**
 * @title VaultComposerSync - Synchronous Vault Composer
 * @author LayerZero Labs (@shankars99, @TRileySchwarz)
 * @notice This contract is a composer that allows deposits and redemptions operations against a
 *         synchronous vault across different chains using LayerZero's OFT protocol.
 * @dev The contract is designed to handle deposits and redemptions of vault shares and assets,
 *      ensuring that the share and asset tokens are correctly managed and transferred across chains.
 *      It also includes slippage protection and refund mechanisms for failed transactions.
 * @dev Default refunds are enabled to EOA addresses only on the source.
 *         Custom refunds to contracts can be implemented by overriding the _refund function.
 * @dev Default vault interface is IERC4626 - [ERC4626](https://eips.ethereum.org/EIPS/eip-4626) compliant vaults.
 *      Custom vaults can be implemented by overriding the _deposit and _redeem functions.
 */
contract VaultComposerSync is IVaultComposerSync, ReentrancyGuard {
    using OFTComposeMsgCodec for bytes;
    using OFTComposeMsgCodec for bytes32;
    using SafeERC20 for IERC20;

    // NOTE: Custom error name to avoid conflict with OAppReceiver.OnlyEndpoint
    // when wiTryVaultComposer inherits from both VaultComposerSync and OApp
    error OnlyLzEndpoint(address caller);

    // NOTE: Added this error for compatibility without OZV5 upgrade. Resolve for Prod implementations
    /**
     * @dev Attempted to deposit more assets than the max amount for `receiver`.
     */
    error ERC4626ExceededMaxDeposit(address receiver, uint256 assets, uint256 max);

    /**
     * @dev Attempted to redeem more shares than the max amount for `receiver`.
     */
    error ERC4626ExceededMaxRedeem(address owner, uint256 shares, uint256 max);

    /// @dev Must be a synchronous vault - NO 2-step redemptions/deposit windows
    IERC4626 public immutable VAULT;

    address public immutable ASSET_OFT;
    address public immutable ASSET_ERC20;
    address public immutable SHARE_OFT;
    address public immutable SHARE_ERC20;

    address public immutable ENDPOINT;
    uint32 public immutable VAULT_EID;

    /**
     * @notice Initializes the VaultComposerSync contract with vault and OFT token addresses
     * @param _vault The address of the ERC4626 vault contract
     * @param _assetOFT The address of the asset OFT (Omnichain Fungible Token) contract
     * @param _shareOFT The address of the share OFT contract (must be an adapter)
     *
     * Requirements:
     * - Share token must be the vault itself
     * - Asset token must match the vault's underlying asset
     * - Share OFT must be an adapter (approvalRequired() returns true)
     */
    constructor(address _vault, address _assetOFT, address _shareOFT) {
        VAULT = IERC4626(_vault);

        ASSET_OFT = _assetOFT;
        ASSET_ERC20 = IOFT(ASSET_OFT).token();
        SHARE_OFT = _shareOFT;
        SHARE_ERC20 = IOFT(SHARE_OFT).token();

        ENDPOINT = address(IOAppCore(ASSET_OFT).endpoint());
        VAULT_EID = ILayerZeroEndpointV2(ENDPOINT).eid();

        if (SHARE_ERC20 != address(VAULT)) {
            revert ShareTokenNotVault(SHARE_ERC20, address(VAULT));
        }

        if (ASSET_ERC20 != address(VAULT.asset())) {
            revert AssetTokenNotVaultAsset(ASSET_ERC20, address(VAULT.asset()));
        }

        /// @dev ShareOFT must be an OFT adapter. We can infer this by checking 'approvalRequired()'.
        /// @dev burn() on tokens when a user sends changes totalSupply() which the asset:share ratio depends on.
        if (!IOFT(SHARE_OFT).approvalRequired()) revert ShareOFTNotAdapter(SHARE_OFT);

        /// @dev Approve the vault to spend the asset tokens held by this contract
        IERC20(ASSET_ERC20).approve(_vault, type(uint256).max);
        /// @dev Approving the vault for the share erc20 is not required when the vault is the share erc20
        // IERC20(SHARE_ERC20).approve(_vault, type(uint256).max);

        /// @dev Approve the share adapter with the share tokens held by this contract
        IERC20(SHARE_ERC20).approve(_shareOFT, type(uint256).max);
        /// @dev If the asset OFT is an adapter, approve it as well
        if (IOFT(_assetOFT).approvalRequired()) IERC20(ASSET_ERC20).approve(_assetOFT, type(uint256).max);
    }

    /**
     * @notice Handles LayerZero compose operations for vault transactions with automatic refund functionality
     * @dev This composer is designed to handle refunds to an EOA address and not a contract
     * @dev Any revert in handleCompose() causes a refund back to the src EXCEPT for InsufficientMsgValue
     * @param _composeSender The OFT contract address used for refunds, must be either ASSET_OFT or SHARE_OFT
     * @param _guid LayerZero's unique tx id (created on the source tx)
     * @param _message Decomposable bytes object into [composeHeader][composeMessage]
     */
    function lzCompose(
        address _composeSender, // The OFT used on refund, also the vaultIn token.
        bytes32 _guid,
        bytes calldata _message, // expected to contain a composeMessage = abi.encode(SendParam hopSendParam,uint256 minMsgValue)
        address,
        /*_executor*/
        bytes calldata /*_extraData*/
    )
        external
        payable
        virtual
        override
    {
        if (msg.sender != ENDPOINT) revert OnlyLzEndpoint(msg.sender);
        if (_composeSender != ASSET_OFT && _composeSender != SHARE_OFT) revert OnlyValidComposeCaller(_composeSender);

        bytes32 composeFrom = _message.composeFrom();
        uint256 amount = _message.amountLD();
        bytes memory composeMsg = _message.composeMsg();

        /// @dev try...catch to handle the compose operation. if it fails we refund the user
        try this.handleCompose{value: msg.value}(_composeSender, composeFrom, composeMsg, amount) {
            emit Sent(_guid);
        } catch (bytes memory _err) {
            /// @dev A revert where the msg.value passed is lower than the min expected msg.value is handled separately
            /// This is because it is possible to re-trigger from the endpoint the compose operation with the right msg.value
            if (bytes4(_err) == InsufficientMsgValue.selector) {
                assembly {
                    revert(add(32, _err), mload(_err))
                }
            }

            _refund(_composeSender, _message, amount, tx.origin);
            emit Refunded(_guid);
        }
    }

    /**
     * @notice Handles the compose operation for OFT (Omnichain Fungible Token) transactions
     * @dev This function can only be called by the contract itself (self-call restriction)
     *      Decodes the compose message to extract SendParam and minimum message value
     *      Routes to either deposit or redeem flow based on the input OFT token type
     * @param _oftIn The OFT token whose funds have been received in the lzReceive associated with this lzTx
     * @param _composeFrom The bytes32 identifier of the compose sender
     * @param _composeMsg The encoded message containing SendParam and minMsgValue
     * @param _amount The amount of tokens received in the lzReceive associated with this lzTx
     */
    function handleCompose(address _oftIn, bytes32 _composeFrom, bytes memory _composeMsg, uint256 _amount)
        external
        payable
        virtual
    {
        /// @dev Can only be called by self
        if (msg.sender != address(this)) revert OnlySelf(msg.sender);

        /// @dev SendParam defines how the composer will handle the user's funds
        /// @dev The minMsgValue is the minimum amount of msg.value that must be sent, failing to do so will revert and the transaction will be retained in the endpoint for future retries
        (SendParam memory sendParam, uint256 minMsgValue) = abi.decode(_composeMsg, (SendParam, uint256));
        if (msg.value < minMsgValue) revert InsufficientMsgValue(minMsgValue, msg.value);

        if (_oftIn == ASSET_OFT) {
            _depositAndSend(_composeFrom, _amount, sendParam, tx.origin);
        } else {
            _redeemAndSend(_composeFrom, _amount, sendParam, tx.origin);
        }
    }

    /**
     * @notice Deposits ERC20 assets from the caller into the vault and sends them to the recipient
     * @param _assetAmount The number of ERC20 tokens to deposit and send
     * @param _sendParam Parameters on how to send the shares to the recipient
     * @param _refundAddress Address to receive excess `msg.value`
     */
    function depositAndSend(uint256 _assetAmount, SendParam memory _sendParam, address _refundAddress)
        external
        payable
        virtual
        nonReentrant
    {
        IERC20(ASSET_ERC20).safeTransferFrom(msg.sender, address(this), _assetAmount);
        _depositAndSend(OFTComposeMsgCodec.addressToBytes32(msg.sender), _assetAmount, _sendParam, _refundAddress);
    }

    /**
     * @dev Internal function that deposits assets and sends shares to another chain
     * @param _depositor The depositor (bytes32 format to account for non-evm addresses)
     * @param _assetAmount The number of assets to deposit
     * @param _sendParam Parameter that defines how to send the shares
     * @param _refundAddress Address to receive excess payment of the LZ fees
     * @notice This function first deposits the assets to mint shares, validates the shares meet minimum slippage requirements,
     *         then sends the minted shares cross-chain using the OFT (Omnichain Fungible Token) protocol
     * @notice The _sendParam.amountLD is updated to the actual share amount minted, and minAmountLD is reset to 0 for the send operation
     */
    function _depositAndSend(
        bytes32 _depositor,
        uint256 _assetAmount,
        SendParam memory _sendParam,
        address _refundAddress
    ) internal virtual {
        uint256 shareAmount = _deposit(_depositor, _assetAmount);
        _assertSlippage(shareAmount, _sendParam.minAmountLD);

        _sendParam.amountLD = shareAmount;
        _sendParam.minAmountLD = 0;

        _send(SHARE_OFT, _sendParam, _refundAddress);
        emit Deposited(_depositor, _sendParam.to, _sendParam.dstEid, _assetAmount, shareAmount);
    }

    /**
     * @dev Internal function to deposit assets into the vault
     * @param _assetAmount The number of assets to deposit into the vault
     * @return shareAmount The number of shares received from the vault deposit
     * @notice This function is expected to be overridden by the inheriting contract to implement custom/nonERC4626 deposit logic
     */
    function _deposit(
        bytes32,
        /*_depositor*/
        uint256 _assetAmount
    )
        internal
        virtual
        returns (uint256 shareAmount)
    {
        shareAmount = VAULT.deposit(_assetAmount, address(this));
    }

    /**
     * @notice Redeems vault shares and sends the resulting assets to the user
     * @param _shareAmount The number of vault shares to redeem
     * @param _sendParam Parameter that defines how to send the assets
     * @param _refundAddress Address to receive excess payment of the LZ fees
     */
    function redeemAndSend(uint256 _shareAmount, SendParam memory _sendParam, address _refundAddress)
        external
        payable
        virtual
        nonReentrant
    {
        IERC20(SHARE_ERC20).safeTransferFrom(msg.sender, address(this), _shareAmount);
        _redeemAndSend(OFTComposeMsgCodec.addressToBytes32(msg.sender), _shareAmount, _sendParam, _refundAddress);
    }

    /**
     * @dev Internal function that redeems shares for assets and sends them cross-chain
     * @param _redeemer The address of the redeemer in bytes32 format
     * @param _shareAmount The number of shares to redeem
     * @param _sendParam Parameter that defines how to send the assets
     * @param _refundAddress Address to receive excess payment of the LZ fees
     * @notice This function first redeems the specified share amount for the underlying asset,
     *         validates the received amount against slippage protection, then initiates a cross-chain
     *         transfer of the redeemed assets using the OFT (Omnichain Fungible Token) protocol
     * @notice The minAmountLD in _sendParam is reset to 0 after slippage validation since the
     *         actual amount has already been verified
     */
    function _redeemAndSend(
        bytes32 _redeemer,
        uint256 _shareAmount,
        SendParam memory _sendParam,
        address _refundAddress
    ) internal virtual {
        uint256 assetAmount = _redeem(_redeemer, _shareAmount);
        _assertSlippage(assetAmount, _sendParam.minAmountLD);

        _sendParam.amountLD = assetAmount;
        _sendParam.minAmountLD = 0;

        _send(ASSET_OFT, _sendParam, _refundAddress);
        emit Redeemed(_redeemer, _sendParam.to, _sendParam.dstEid, _shareAmount, assetAmount);
    }

    /**
     * @dev Internal function to redeem shares from the vault
     * @param _shareAmount The number of shares to redeem from the vault
     * @return assetAmount The number of assets received from the vault redemption
     * @notice This function is expected to be overridden by the inheriting contract to implement custom/nonERC4626 redemption logic
     */
    function _redeem(
        bytes32,
        /*_redeemer*/
        uint256 _shareAmount
    )
        internal
        virtual
        returns (uint256 assetAmount)
    {
        assetAmount = VAULT.redeem(_shareAmount, address(this), address(this));
    }

    /**
     * @param _amountLD The amount of tokens to send
     * @param _minAmountLD The minimum amount of tokens that must be sent to avoid slippage
     * @notice This function checks if the amount sent is less than the minimum amount
     *         If it is, it reverts with SlippageExceeded error
     * @notice This function can be overridden to implement custom slippage logic
     */
    function _assertSlippage(uint256 _amountLD, uint256 _minAmountLD) internal view virtual {
        if (_amountLD < _minAmountLD) revert SlippageExceeded(_amountLD, _minAmountLD);
    }

    /**
     * @notice Quotes the send operation for the given OFT and SendParam
     * @dev Revert on slippage will be thrown by the OFT and not _assertSlippage
     * @param _from The "sender address" used for the quote
     * @param _targetOFT The OFT contract address to quote
     * @param _vaultInAmount The amount of tokens to send to the vault
     * @param _sendParam The parameters for the send operation
     * @return MessagingFee The estimated fee for the send operation
     * @dev This function can be overridden to implement custom quoting logic
     */
    function quoteSend(address _from, address _targetOFT, uint256 _vaultInAmount, SendParam memory _sendParam)
        external
        view
        virtual
        returns (MessagingFee memory)
    {
        /// @dev When quoting the asset OFT, the function input is shares and the SendParam.amountLD into quoteSend() should be assets (and vice versa)

        if (_targetOFT == ASSET_OFT) {
            uint256 maxRedeem = VAULT.maxRedeem(_from);
            if (_vaultInAmount > maxRedeem) {
                revert ERC4626ExceededMaxRedeem(_from, _vaultInAmount, maxRedeem);
            }

            _sendParam.amountLD = VAULT.previewRedeem(_vaultInAmount);
        } else {
            uint256 maxDeposit = VAULT.maxDeposit(_from);
            if (_vaultInAmount > maxDeposit) {
                revert ERC4626ExceededMaxDeposit(_from, _vaultInAmount, maxDeposit);
            }

            _sendParam.amountLD = VAULT.previewDeposit(_vaultInAmount);
        }
        return IOFT(_targetOFT).quoteSend(_sendParam, false);
    }

    /**
     * @dev Internal function that handles token transfer to the recipient
     * @dev If the destination eid is the same as the current eid, it transfers the tokens directly to the recipient
     * @dev If the destination eid is different, it sends a LayerZero cross-chain transaction
     * @param _oft The OFT contract address to use for sending
     * @param _sendParam The parameters for the send operation
     * @param _refundAddress Address to receive excess payment of the LZ fees
     */
    function _send(address _oft, SendParam memory _sendParam, address _refundAddress) internal {
        if (_sendParam.dstEid == VAULT_EID) {
            /// @dev Can do this because _oft is validated before this function is called
            address erc20 = _oft == ASSET_OFT ? ASSET_ERC20 : SHARE_ERC20;

            if (msg.value > 0) revert NoMsgValueExpected();
            IERC20(erc20).safeTransfer(_sendParam.to.bytes32ToAddress(), _sendParam.amountLD);
        } else {
            // crosschain send
            IOFT(_oft).send{value: msg.value}(_sendParam, MessagingFee(msg.value, 0), _refundAddress);
        }
    }

    /**
     * @dev Internal function to refund input tokens to sender on source during a failed transaction
     * @param _oft The OFT contract address used for refunding
     * @param _message The original message that was sent
     * @param _amount The amount of tokens to refund
     * @param _refundAddress Address to receive the refund
     */
    function _refund(address _oft, bytes calldata _message, uint256 _amount, address _refundAddress) internal virtual {
        /// @dev Extracted from the _message header. Will always be part of the _message since it is created by lzReceive
        SendParam memory refundSendParam;
        refundSendParam.dstEid = OFTComposeMsgCodec.srcEid(_message);
        refundSendParam.to = OFTComposeMsgCodec.composeFrom(_message);
        refundSendParam.amountLD = _amount;

        IOFT(_oft).send{value: msg.value}(refundSendParam, MessagingFee(msg.value, 0), _refundAddress);
    }

    /**
     * @notice Allow contract to receive ETH
     * @dev Virtual to allow child contracts to override if needed
     */
    receive() external payable virtual {}
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IiTrySiloDefinitions} from "./interfaces/IiTrySiloDefinitions.sol";

/**
 * @title iTrySilo
 * @notice The Silo allows to store iTry during the stake cooldown process.
 */
contract iTrySilo is IiTrySiloDefinitions {
    using SafeERC20 for IERC20;

    address immutable STAKING_VAULT;
    IERC20 immutable iTry;

    constructor(address _stakingVault, address _iTryToken) {
        STAKING_VAULT = _stakingVault;
        iTry = IERC20(_iTryToken);
    }

    modifier onlyStakingVault() {
        if (msg.sender != STAKING_VAULT) revert OnlyStakingVault();
        _;
    }

    function withdraw(address to, uint256 amount) external onlyStakingVault {
        iTry.transfer(to, amount);
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

/**
 * @title IwiTryVaultComposer
 * @notice Interface for wiTryVaultComposer contract with async redemption support
 * @dev Defines events and errors specific to wiTryVaultComposer async redemption functionality
 */
interface IwiTryVaultComposer {
    // ============================================================================
    // Events
    // ============================================================================

    /**
     * @notice Emitted when async redemption (cooldown) is initiated for a user
     * @param user The bytes32 identifier of the user (cross-chain compatible)
     * @param owner The address of the owner on the hub chain
     * @param shareAmount The number of shares being redeemed
     * @param assetAmount The number of assets locked in cooldown
     */
    event CooldownInitiated(bytes32 indexed user, address indexed owner, uint256 shareAmount, uint256 assetAmount);

    /**
     * @notice Emitted when crosschain unstake is processed successfully
     * @param user Address that requested unstake
     * @param srcEid Source chain endpoint ID
     * @param assets Amount of iTRY unstaked
     * @param guid Message GUID
     */
    event CrosschainUnstakeProcessed(address indexed user, uint32 indexed srcEid, uint256 assets, bytes32 guid);

    /**
     * @notice Emitted when tokens are rescued from the contract
     * @param token The address of the token rescued (address(0) for ETH)
     * @param to The address receiving the rescued tokens
     * @param amount The amount rescued
     */
    event TokenRescued(address indexed token, address indexed to, uint256 amount);

    /**
     * @notice Emitted when crosschain fast redeem is processed successfully
     * @param user Address that requested fast redeem
     * @param srcEid Source chain endpoint ID
     * @param shares Amount of shares redeemed
     * @param assets Amount of iTRY redeemed
     */
    event CrosschainFastRedeemProcessed(address indexed user, uint32 indexed srcEid, uint256 shares, uint256 assets);

    // ============================================================================
    // Errors
    // ============================================================================

    /**
     * @notice Error emitted when attempting to use synchronous redemption
     * @dev Synchronous redemption is not supported for vaults requiring cooldown periods
     *      Use INITIATE_COOLDOWN command in lzCompose instead
     */
    error SyncRedemptionNotSupported();

    /**
     * @notice Error emitted when attempting share operations without INITIATE_COOLDOWN command
     * @dev When sending shares to the wiTryVaultComposer via lzCompose, you must include
     *      oftCmd="INITIATE_COOLDOWN" in the SendParam to trigger async redemption
     */
    error InitiateCooldownRequired();

    /**
     * @notice Error emitted when an invalid origin is provided
     * @dev Thrown during validation of origin information
     */
    error InvalidOrigin();

    /**
     * @notice Error emitted when an invalid zero address is provided
     * @dev Thrown during validation of zero address
     */
    error InvalidZeroAddress();

    /**
     * @notice Thrown when an invalid token address is provided
     */
    error InvalidToken();

    /**
     * @notice Thrown when attempting to rescue with zero balance
     */
    error NoBalance();

    /**
     * @notice Thrown when ETH transfer fails
     */
    error TransferFailed();

    /**
     * @notice Thrown when an unknown message type is received
     * @param msgType The unknown message type
     */
    error UnknownMessageType(uint16 msgType);

    /**
     * @notice Thrown when there are no assets to unstake
     */
    error NoAssetsToUnstake();

    /**
     * @notice Thrown when an invalid amount is provided
     */
    error InvalidAmount();

    /**
     * @notice Thrown when an invalid destination endpoint ID is provided
     */
    error InvalidDestination();

    /**
     * @notice Thrown when there are no assets to redeem
     */
    error NoAssetsToRedeem();

    // ============================================================================
    // Functions
    // ============================================================================

    /**
     * @notice Quote fee for unstake return leg (hub→spoke)
     * @param to Address to receive iTRY on destination chain
     * @param amount Amount of iTRY to send
     * @param dstEid Destination endpoint ID (spoke chain)
     * @return nativeFee Fee in native token for hub→spoke leg
     * @return lzTokenFee Fee in LayerZero token (typically 0)
     */
    function quoteUnstakeReturn(address to, uint256 amount, uint32 dstEid)
        external
        view
        returns (uint256 nativeFee, uint256 lzTokenFee);

    /**
     * @notice Quote fee for fast redeem return leg (hub→spoke)
     * @dev Queries the adapter to get LayerZero fee for sending iTRY back to spoke chain
     * @param to Address to receive iTRY on spoke chain
     * @param amount Amount of iTRY to send
     * @param dstEid Destination endpoint ID (spoke chain)
     * @return nativeFee Fee in native token for hub→spoke leg
     * @return lzTokenFee Fee in LayerZero token (typically 0)
     */
    function quoteFastRedeemReturn(address to, uint256 amount, uint32 dstEid)
        external
        view
        returns (uint256 nativeFee, uint256 lzTokenFee);

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

// SPDX-License-Identifier: MIT

pragma solidity 0.8.20;

interface IStakediTry {
    // Events //
    /// @notice Event emitted when the rewards are received
    event RewardsReceived(uint256 amount);
    /// @notice Event emitted when the balance from an FULL_RESTRICTED_STAKER_ROLE user are redistributed
    event LockedAmountRedistributed(address indexed from, address indexed to, uint256 amount);
    /// @notice Event emitted when the vesting period is updated
    event VestingPeriodUpdated(uint256 indexed oldVestingPeriod, uint256 indexed newVestingPeriod);

    // Errors //
    /// @notice Error emitted shares or assets equal zero.
    error InvalidAmount();
    /// @notice Error emitted when owner attempts to rescue iTry tokens.
    error InvalidToken();
    /// @notice Error emitted when a small non-zero share amount remains, which risks donations attack
    error MinSharesViolation();
    /// @notice Error emitted when owner is not allowed to perform an operation
    error OperationNotAllowed();
    /// @notice Error emitted when there is still unvested amount
    error StillVesting();
    /// @notice Error emitted when owner or blacklist manager attempts to blacklist owner
    error CantBlacklistOwner();
    /// @notice Error emitted when the zero address is given
    error InvalidZeroAddress();
    /// @notice Error emitted when an invalid vesting period is provided
    error InvalidVestingPeriod();

    function transferInRewards(uint256 amount) external;

    function rescueTokens(address token, uint256 amount, address to) external;

    function getUnvestedAmount() external view returns (uint256);

    function setVestingPeriod(uint256 _vestingPeriod) external;

    function getVestingPeriod() external view returns (uint256);
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
pragma solidity ^0.8.20;

import {IERC4626} from "@openzeppelin/contracts/interfaces/IERC4626.sol";

import {IOAppComposer} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oapp/interfaces/IOAppComposer.sol";
import {SendParam, MessagingFee} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oft/interfaces/IOFT.sol";

interface IVaultComposerSync is IOAppComposer {
    /// ========================== EVENTS =====================================
    event Sent(bytes32 indexed guid); // 0x27b5aea9
    event Refunded(bytes32 indexed guid); // 0xfe509803

    event Deposited(bytes32 sender, bytes32 recipient, uint32 dstEid, uint256 assetAmt, uint256 shareAmt); // 0xa53b96f2
    event Redeemed(bytes32 sender, bytes32 recipient, uint32 dstEid, uint256 shareAmt, uint256 assetAmt); // 0x57e232f1

    /// ========================== Error Messages =====================================
    error ShareOFTNotAdapter(address shareOFT); // 0xfc1514ae
    error ShareTokenNotVault(address shareERC20, address vault); // 0x0e178ab6
    error AssetTokenNotVaultAsset(address assetERC20, address vaultAsset); // 0xba9d665f

    // NOTE: OnlyEndpoint(address) declared locally in VaultComposerSync
    // Cannot be in interface due to conflict with OAppReceiver when wiTryVaultComposer inherits OApp
    error OnlySelf(address caller); // 0xa19dbf00
    error OnlyValidComposeCaller(address caller); // 0x84fb3f0d

    error InsufficientMsgValue(uint256 expectedMsgValue, uint256 actualMsgValue); // 0x7cb769dc
    error NoMsgValueExpected(); // 0x7578d2bd

    error SlippageExceeded(uint256 amountLD, uint256 minAmountLD); // 0x71c4efed

    /// ========================== GLOBAL VARIABLE FUNCTIONS =====================================
    function VAULT() external view returns (IERC4626);

    function ASSET_OFT() external view returns (address);
    function ASSET_ERC20() external view returns (address);
    function SHARE_OFT() external view returns (address);
    function SHARE_ERC20() external view returns (address);

    function ENDPOINT() external view returns (address);
    function VAULT_EID() external view returns (uint32);

    /// ========================== Proxy OFT =====================================

    /**
     * @notice Deposits ERC20 assets from the caller into the vault and sends them to the recipient
     * @param assetAmount The number of ERC20 tokens to deposit and send
     * @param sendParam Parameters on how to send the shares to the recipient
     * @param refundAddress Address to receive excess `msg.value`
     */
    function depositAndSend(uint256 assetAmount, SendParam memory sendParam, address refundAddress) external payable;

    /**
     * @notice Redeems vault shares and sends the resulting assets to the user
     * @param shareAmount The number of vault shares to redeem
     * @param sendParam Parameter that defines how to send the assets
     * @param refundAddress Address to receive excess payment of the LZ fees
     */
    function redeemAndSend(uint256 shareAmount, SendParam memory sendParam, address refundAddress) external payable;

    /**
     * @notice Quotes the send operation for the given OFT and SendParam
     * @param from The "sender address" used for the quote
     * @param targetOft The OFT contract address to quote
     * @param vaultInAmount The amount of tokens to send to the vault
     * @param sendParam The parameters for the send operation
     * @return MessagingFee The estimated fee for the send operation
     * @dev This function can be overridden to implement custom quoting logic
     */
    function quoteSend(address from, address targetOft, uint256 vaultInAmount, SendParam memory sendParam)
        external
        view
        returns (MessagingFee memory);

    /// ========================== Receive =====================================
    receive() external payable;
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

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

interface IiTrySiloDefinitions {
    /// @notice Error emitted when the staking vault is not the caller
    error OnlyStakingVault();
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

// @dev Import the 'MessagingFee' and 'MessagingReceipt' so it's exposed to OApp implementers
// solhint-disable-next-line no-unused-import
import { OAppSender, MessagingFee, MessagingReceipt } from "./OAppSender.sol";
// @dev Import the 'Origin' so it's exposed to OApp implementers
// solhint-disable-next-line no-unused-import
import { OAppReceiver, Origin } from "./OAppReceiver.sol";
import { OAppCore } from "./OAppCore.sol";

/**
 * @title OApp
 * @dev Abstract contract serving as the base for OApp implementation, combining OAppSender and OAppReceiver functionality.
 */
abstract contract OApp is OAppSender, OAppReceiver {
    /**
     * @dev Constructor to initialize the OApp with the provided endpoint and owner.
     * @param _endpoint The address of the LOCAL LayerZero endpoint.
     * @param _delegate The delegate capable of making OApp configurations inside of the endpoint.
     */
    constructor(address _endpoint, address _delegate) OAppCore(_endpoint, _delegate) {}

    /**
     * @notice Retrieves the OApp version information.
     * @return senderVersion The version of the OAppSender.sol implementation.
     * @return receiverVersion The version of the OAppReceiver.sol implementation.
     */
    function oAppVersion()
        public
        pure
        virtual
        override(OAppSender, OAppReceiver)
        returns (uint64 senderVersion, uint64 receiverVersion)
    {
        return (SENDER_VERSION, RECEIVER_VERSION);
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

pragma solidity >=0.8.0;

interface IUltraLightNode301 {
    function commitVerification(bytes calldata _packet, uint256 _gasLimit) external;
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


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
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

import { ILayerZeroComposer } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroComposer.sol";

/**
 * @title IOAppComposer
 * @dev This interface defines the OApp Composer, allowing developers to inherit only the OApp package without the protocol.
 */
// solhint-disable-next-line no-empty-blocks
interface IOAppComposer is ILayerZeroComposer {}

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

pragma solidity ^0.8.20;

import { ILayerZeroEndpointV2, MessagingFee, MessagingReceipt, Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import { ReadCmdCodecV1, EVMCallComputeV1, EVMCallRequestV1 } from "../libs/ReadCmdCodecV1.sol";
import { IOAppComputer } from "../interfaces/IOAppComputer.sol";

import { OAppRead } from "../OAppRead.sol";

contract LzReadCounter is OAppRead, IOAppComputer {
    struct EvmReadRequest {
        uint16 appRequestLabel;
        uint32 targetEid;
        bool isBlockNum;
        uint64 blockNumOrTimestamp;
        uint16 confirmations;
        address to;
        uint256 countAddition; // addition to add to the count when reading
    }

    struct ComputeSetting {
        uint8 computeSetting;
        uint16 computeConfirmations;
        uint64 blockNumOrTimestamp;
        bool isBlockNum;
    }

    uint8 internal constant COMPUTE_SETTING_MAP_ONLY = 0;
    uint8 internal constant COMPUTE_SETTING_REDUCE_ONLY = 1;
    uint8 internal constant COMPUTE_SETTING_MAP_REDUCE = 2;
    uint8 internal constant COMPUTE_SETTING_NONE = 3;

    uint32 public immutable eid;
    uint256 public count;

    constructor(address _endpoint) OAppRead(_endpoint, msg.sender) {
        eid = ILayerZeroEndpointV2(_endpoint).eid();
    }

    // -------------------------------
    // Trigger Read
    function triggerRead(
        uint32 _channelId, // The read channel id
        uint16 _appLabel, // The cmd app label
        EvmReadRequest[] memory _requests,
        ComputeSetting memory _computeSetting,
        bytes calldata _options
    ) external payable returns (MessagingReceipt memory receipt) {
        bytes memory cmd = buildCmd(_appLabel, _requests, _computeSetting);
        count += 1; // increase the count, for pin block testing
        return _lzSend(_channelId, cmd, _options, MessagingFee(msg.value, 0), payable(msg.sender));
    }

    function clearCount() external {
        count = 0;
    }

    // -------------------------------
    // View
    function quote(
        uint32 _channelId,
        uint16 _appLabel,
        EvmReadRequest[] memory _requests,
        ComputeSetting memory _computeSetting,
        bytes calldata _options
    ) public view returns (uint256 nativeFee, uint256 lzTokenFee) {
        bytes memory cmd = buildCmd(_appLabel, _requests, _computeSetting);
        MessagingFee memory fee = _quote(_channelId, cmd, _options, false);
        return (fee.nativeFee, fee.lzTokenFee);
    }

    function buildCmd(
        uint16 appLabel,
        EvmReadRequest[] memory _readRequests,
        ComputeSetting memory _computeSetting
    ) public view returns (bytes memory) {
        require(_readRequests.length > 0, "LzReadCounter: empty requests");
        // build read requests
        EVMCallRequestV1[] memory readRequests = new EVMCallRequestV1[](_readRequests.length);
        for (uint256 i = 0; i < _readRequests.length; i++) {
            EvmReadRequest memory req = _readRequests[i];
            readRequests[i] = EVMCallRequestV1({
                appRequestLabel: req.appRequestLabel,
                targetEid: req.targetEid,
                isBlockNum: req.isBlockNum,
                blockNumOrTimestamp: req.blockNumOrTimestamp,
                confirmations: req.confirmations,
                to: req.to,
                callData: abi.encodeWithSelector(this.readCount.selector, req.countAddition)
            });
        }
        // build compute, on current contract
        require(_computeSetting.computeSetting <= COMPUTE_SETTING_NONE, "LzReadCounter: invalid compute type");
        EVMCallComputeV1 memory evmCompute = EVMCallComputeV1({
            computeSetting: _computeSetting.computeSetting,
            targetEid: _computeSetting.computeSetting == COMPUTE_SETTING_NONE ? 0 : eid, // 0(means no compute) for none, else use local eid
            isBlockNum: _computeSetting.isBlockNum,
            blockNumOrTimestamp: _computeSetting.blockNumOrTimestamp,
            confirmations: _computeSetting.computeConfirmations,
            to: address(this)
        });
        bytes memory cmd = ReadCmdCodecV1.encode(appLabel, readRequests, evmCompute);

        return cmd;
    }

    function readCount(uint256 countAddition) external view returns (uint256) {
        require(countAddition != 9, "LzReadCounter: invalid count addition"); // This is only for testing
        return count + countAddition;
    }

    function lzMap(bytes calldata _request, bytes calldata _response) external pure returns (bytes memory) {
        require(_response.length == 32, "LzReadCounter: invalid response length");
        uint16 requestId = ReadCmdCodecV1.decodeRequestV1AppRequestLabel(_request);
        uint256 countNum = abi.decode(_response, (uint256));
        return abi.encode(countNum + 100 + requestId * 1000); // map behavior
    }

    function lzReduce(bytes calldata _cmd, bytes[] calldata _responses) external pure returns (bytes memory) {
        uint256 total = 0;
        for (uint256 i = 0; i < _responses.length; i++) {
            require(_responses[i].length == 32, "LzReadCounter: invalid response length");
            uint256 countNum = abi.decode(_responses[i], (uint256));
            total += countNum;
        }
        total += 10000; // reduce behavior

        uint16 cmdAppLabel = ReadCmdCodecV1.decodeCmdAppLabel(_cmd);
        total += uint256(cmdAppLabel) * 100000; // cmdAppLabel behavior

        return abi.encode(total);
    }

    // -------------------------------
    function _lzReceive(
        Origin calldata /* _origin */,
        bytes32 /* _guid */,
        bytes calldata _message,
        address /*_executor*/,
        bytes calldata /*_extraData*/
    ) internal override {
        require(_message.length % 32 == 0, "LzReadCounter: invalid message length");
        uint256 total = 0;
        // loop read bytes32 of the message and decode it to uint256 then add it to the total
        for (uint256 i = 0; i < _message.length; i += 32) {
            total += abi.decode(_message[i:i + 32], (uint256));
        }
        // reset count if it's too large
        if (count >= 2 ** 128) {
            count = 0;
        }
        count += total;
    }

    // be able to receive ether
    receive() external payable virtual {}

    fallback() external payable {}
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

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import { ILayerZeroEndpointV2, MessagingFee, MessagingReceipt, Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";
import { ILayerZeroComposer } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroComposer.sol";

import { OApp } from "../OApp.sol";
import { OptionsBuilder } from "../libs/OptionsBuilder.sol";
import { OAppPreCrimeSimulator } from "../../precrime/OAppPreCrimeSimulator.sol";

library MsgCodec {
    uint8 internal constant VANILLA_TYPE = 1;
    uint8 internal constant COMPOSED_TYPE = 2;
    uint8 internal constant ABA_TYPE = 3;
    uint8 internal constant COMPOSED_ABA_TYPE = 4;

    uint8 internal constant MSG_TYPE_OFFSET = 0;
    uint8 internal constant SRC_EID_OFFSET = 1;
    uint8 internal constant VALUE_OFFSET = 5;

    function encode(uint8 _type, uint32 _srcEid) internal pure returns (bytes memory) {
        return abi.encodePacked(_type, _srcEid);
    }

    function encode(uint8 _type, uint32 _srcEid, uint256 _value) internal pure returns (bytes memory) {
        return abi.encodePacked(_type, _srcEid, _value);
    }

    function msgType(bytes calldata _message) internal pure returns (uint8) {
        return uint8(bytes1(_message[MSG_TYPE_OFFSET:SRC_EID_OFFSET]));
    }

    function srcEid(bytes calldata _message) internal pure returns (uint32) {
        return uint32(bytes4(_message[SRC_EID_OFFSET:VALUE_OFFSET]));
    }

    function value(bytes calldata _message) internal pure returns (uint256) {
        return uint256(bytes32(_message[VALUE_OFFSET:]));
    }
}

// @dev declared as abstract to provide backwards compatibility with Oz5/Oz4
abstract contract OmniCounterAbstract is ILayerZeroComposer, OApp, OAppPreCrimeSimulator {
    using MsgCodec for bytes;
    using OptionsBuilder for bytes;

    uint256 public count;
    uint256 public composedCount;

    address public admin;
    uint32 public eid;

    mapping(uint32 srcEid => mapping(bytes32 sender => uint64 nonce)) private maxReceivedNonce;
    bool private orderedNonce;

    // for global assertions
    mapping(uint32 srcEid => uint256 count) public inboundCount;
    mapping(uint32 dstEid => uint256 count) public outboundCount;

    constructor(address _endpoint, address _delegate) OApp(_endpoint, _delegate) {
        admin = msg.sender;
        eid = ILayerZeroEndpointV2(_endpoint).eid();
    }

    modifier onlyAdmin() {
        require(msg.sender == admin, "only admin");
        _;
    }

    // -------------------------------
    // Only Admin
    function setAdmin(address _admin) external onlyAdmin {
        admin = _admin;
    }

    function withdraw(address payable _to, uint256 _amount) external onlyAdmin {
        (bool success, ) = _to.call{ value: _amount }("");
        require(success, "OmniCounter: withdraw failed");
    }

    // -------------------------------
    // Send
    function increment(uint32 _eid, uint8 _type, bytes calldata _options) external payable {
        //        bytes memory options = combineOptions(_eid, _type, _options);
        _lzSend(_eid, MsgCodec.encode(_type, eid), _options, MessagingFee(msg.value, 0), payable(msg.sender));
        _incrementOutbound(_eid);
    }

    // this is a broken function to skip incrementing outbound count
    // so that preCrime will fail
    function brokenIncrement(uint32 _eid, uint8 _type, bytes calldata _options) external payable onlyAdmin {
        //        bytes memory options = combineOptions(_eid, _type, _options);
        _lzSend(_eid, MsgCodec.encode(_type, eid), _options, MessagingFee(msg.value, 0), payable(msg.sender));
        // _incrementOutbound(_eid); // mock method which intentionally does not increment outboundCount to cause a PreCrime Crime
    }

    function batchIncrement(
        uint32[] calldata _eids,
        uint8[] calldata _types,
        bytes[] calldata _options
    ) external payable {
        require(_eids.length == _options.length && _eids.length == _types.length, "OmniCounter: length mismatch");

        MessagingReceipt memory receipt;
        uint256 providedFee = msg.value;
        for (uint256 i = 0; i < _eids.length; i++) {
            address refundAddress = i == _eids.length - 1 ? msg.sender : address(this);
            uint32 dstEid = _eids[i];
            uint8 msgType = _types[i];
            //            bytes memory options = combineOptions(dstEid, msgType, _options[i]);
            receipt = _lzSend(
                dstEid,
                MsgCodec.encode(msgType, eid),
                _options[i],
                MessagingFee(providedFee, 0),
                payable(refundAddress)
            );
            _incrementOutbound(dstEid);
            providedFee -= receipt.fee.nativeFee;
        }
    }

    // -------------------------------
    // View
    function quote(
        uint32 _eid,
        uint8 _type,
        bytes calldata _options
    ) public view returns (uint256 nativeFee, uint256 lzTokenFee) {
        //        bytes memory options = combineOptions(_eid, _type, _options);
        MessagingFee memory fee = _quote(_eid, MsgCodec.encode(_type, eid), _options, false);
        return (fee.nativeFee, fee.lzTokenFee);
    }

    // @dev enables preCrime simulator
    // @dev routes the call down from the OAppPreCrimeSimulator, and up to the OApp
    function _lzReceiveSimulate(
        Origin calldata _origin,
        bytes32 _guid,
        bytes calldata _message,
        address _executor,
        bytes calldata _extraData
    ) internal virtual override {
        _lzReceive(_origin, _guid, _message, _executor, _extraData);
    }

    // -------------------------------
    function _lzReceive(
        Origin calldata _origin,
        bytes32 _guid,
        bytes calldata _message,
        address /*_executor*/,
        bytes calldata /*_extraData*/
    ) internal override {
        _acceptNonce(_origin.srcEid, _origin.sender, _origin.nonce);
        uint8 messageType = _message.msgType();

        if (messageType == MsgCodec.VANILLA_TYPE) {
            count++;

            //////////////////////////////// IMPORTANT //////////////////////////////////
            /// if you request for msg.value in the options, you should also encode it
            /// into your message and check the value received at destination (example below).
            /// if not, the executor could potentially provide less msg.value than you requested
            /// leading to unintended behavior. Another option is to assert the executor to be
            /// one that you trust.
            /////////////////////////////////////////////////////////////////////////////
            require(msg.value >= _message.value(), "OmniCounter: insufficient value");

            _incrementInbound(_origin.srcEid);
        } else if (messageType == MsgCodec.COMPOSED_TYPE || messageType == MsgCodec.COMPOSED_ABA_TYPE) {
            count++;
            _incrementInbound(_origin.srcEid);
            endpoint.sendCompose(address(this), _guid, 0, _message);
        } else if (messageType == MsgCodec.ABA_TYPE) {
            count++;
            _incrementInbound(_origin.srcEid);

            // send back to the sender
            _incrementOutbound(_origin.srcEid);
            bytes memory options = OptionsBuilder.newOptions().addExecutorLzReceiveOption(200000, 10);
            _lzSend(
                _origin.srcEid,
                MsgCodec.encode(MsgCodec.VANILLA_TYPE, eid, 10),
                options,
                MessagingFee(msg.value, 0),
                payable(address(this))
            );
        } else {
            revert("invalid message type");
        }
    }

    function _incrementInbound(uint32 _srcEid) internal {
        inboundCount[_srcEid]++;
    }

    function _incrementOutbound(uint32 _dstEid) internal {
        outboundCount[_dstEid]++;
    }

    function lzCompose(
        address _oApp,
        bytes32 /*_guid*/,
        bytes calldata _message,
        address,
        bytes calldata
    ) external payable override {
        require(_oApp == address(this), "!oApp");
        require(msg.sender == address(endpoint), "!endpoint");

        uint8 msgType = _message.msgType();
        if (msgType == MsgCodec.COMPOSED_TYPE) {
            composedCount += 1;
        } else if (msgType == MsgCodec.COMPOSED_ABA_TYPE) {
            composedCount += 1;

            uint32 srcEid = _message.srcEid();
            _incrementOutbound(srcEid);
            bytes memory options = OptionsBuilder.newOptions().addExecutorLzReceiveOption(200000, 0);
            _lzSend(
                srcEid,
                MsgCodec.encode(MsgCodec.VANILLA_TYPE, eid),
                options,
                MessagingFee(msg.value, 0),
                payable(address(this))
            );
        } else {
            revert("invalid message type");
        }
    }

    // -------------------------------
    // Ordered OApp
    // this demonstrates how to build an app that requires execution nonce ordering
    // normally an app should decide ordered or not on contract construction
    // this is just a demo
    function setOrderedNonce(bool _orderedNonce) external onlyOwner {
        orderedNonce = _orderedNonce;
    }

    function _acceptNonce(uint32 _srcEid, bytes32 _sender, uint64 _nonce) internal virtual {
        uint64 currentNonce = maxReceivedNonce[_srcEid][_sender];
        if (orderedNonce) {
            require(_nonce == currentNonce + 1, "OApp: invalid nonce");
        }
        // update the max nonce anyway. once the ordered mode is turned on, missing early nonces will be rejected
        if (_nonce > currentNonce) {
            maxReceivedNonce[_srcEid][_sender] = _nonce;
        }
    }

    function nextNonce(uint32 _srcEid, bytes32 _sender) public view virtual override returns (uint64) {
        if (orderedNonce) {
            return maxReceivedNonce[_srcEid][_sender] + 1;
        } else {
            return 0; // path nonce starts from 1. if 0 it means that there is no specific nonce enforcement
        }
    }

    // TODO should override oApp version with added ordered nonce increment
    // a governance function to skip nonce
    function skipInboundNonce(uint32 _srcEid, bytes32 _sender, uint64 _nonce) public virtual onlyOwner {
        endpoint.skip(address(this), _srcEid, _sender, _nonce);
        if (orderedNonce) {
            maxReceivedNonce[_srcEid][_sender]++;
        }
    }

    function isPeer(uint32 _eid, bytes32 _peer) public view override returns (bool) {
        return peers[_eid] == _peer;
    }

    // @dev Batch send requires overriding this function from OAppSender because the msg.value contains multiple fees
    function _payNative(uint256 _nativeFee) internal virtual override returns (uint256 nativeFee) {
        if (msg.value < _nativeFee) revert NotEnoughNative(msg.value);
        return _nativeFee;
    }

    // be able to receive ether
    receive() external payable virtual {}

    fallback() external payable {}
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import {OFTAdapter} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oft/OFTAdapter.sol";

/**
 * @title wiTryOFTAdapter
 * @notice OFT Adapter for wiTRY shares on hub chain (Ethereum Mainnet)
 * @dev Wraps the StakedUSDe share token to enable cross-chain transfers via LayerZero
 *
 * Architecture (Phase 1 - Instant Redeems):
 * - Hub Chain (Ethereum): StakedUSDe (ERC4626 vault) + wiTryOFTAdapter (locks shares)
 * - Spoke Chain (MegaETH): ShareOFT (mints/burns based on messages)
 *
 * Flow:
 * 1. User deposits iTRY into StakedUSDe vault → receives wiTRY shares
 * 2. User approves wiTryOFTAdapter to spend their wiTRY
 * 3. User calls send() on wiTryOFTAdapter
 * 4. Adapter locks wiTRY shares and sends LayerZero message
 * 5. ShareOFT mints equivalent shares on spoke chain
 *
 * IMPORTANT: This adapter uses lock/unlock pattern (not mint/burn) because
 * the share token's totalSupply must match the vault's accounting.
 * Burning shares would break the share-to-asset ratio in the ERC4626 vault.
 */
contract wiTryOFTAdapter is OFTAdapter {
    /**
     * @notice Constructor for wiTryOFTAdapter
     * @param _token Address of the wiTRY share token from StakedUSDe
     * @param _lzEndpoint LayerZero endpoint address for Ethereum Mainnet
     * @param _owner Address that will own this adapter (typically deployer)
     */
    constructor(address _token, address _lzEndpoint, address _owner) OFTAdapter(_token, _lzEndpoint, _owner) {}
}

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

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import { AddressCast } from "@layerzerolabs/lz-evm-protocol-v2/contracts/libs/AddressCast.sol";

import { OApp } from "./OApp.sol";

abstract contract OAppRead is OApp {

    constructor(address _endpoint, address _delegate) OApp(_endpoint, _delegate) {}

    // -------------------------------
    // Only Owner
    function setReadChannel(uint32 _channelId, bool _active) public virtual onlyOwner {
        _setPeer(_channelId, _active ? AddressCast.toBytes32(address(this)) : bytes32(0));
    }
}

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

// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {OFTAdapter} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oft/OFTAdapter.sol";

/**
 * @title iTryTokenAdapter
 * @notice OFT Adapter for existing iTRY token on hub chain (Ethereum Mainnet)
 * @dev Wraps the existing iTryToken to enable cross-chain transfers via LayerZero
 *
 * Architecture:
 * - Hub Chain (Ethereum): iTryToken (native) + iTryTokenAdapter (locks tokens)
 * - Spoke Chain (MegaETH): iTryTokenOFT (mints/burns based on messages)
 *
 * Flow:
 * 1. User approves iTryTokenAdapter to spend their iTRY
 * 2. User calls send() on iTryTokenAdapter
 * 3. Adapter locks iTRY and sends LayerZero message to spoke chain
 * 4. iTryTokenOFT mints equivalent amount on spoke chain
 */
contract iTryTokenOFTAdapter is OFTAdapter {
    /**
     * @notice Constructor for iTryTokenAdapter
     * @param _token Address of the existing iTryToken contract
     * @param _lzEndpoint LayerZero endpoint address for Ethereum Mainnet
     * @param _owner Address that will own this adapter (typically deployer)
     */
    constructor(address _token, address _lzEndpoint, address _owner) OFTAdapter(_token, _lzEndpoint, _owner) {}
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

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import { IERC20Metadata, IERC20 } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { IOFT, OFTCore } from "./OFTCore.sol";

/**
 * @title OFTAdapter Contract
 * @dev OFTAdapter is a contract that adapts an ERC-20 token to the OFT functionality.
 *
 * @dev For existing ERC20 tokens, this can be used to convert the token to crosschain compatibility.
 * @dev WARNING: ONLY 1 of these should exist for a given global mesh,
 * unless you make a NON-default implementation of OFT and needs to be done very carefully.
 * @dev WARNING: The default OFTAdapter implementation assumes LOSSLESS transfers, ie. 1 token in, 1 token out.
 * IF the 'innerToken' applies something like a transfer fee, the default will NOT work...
 * a pre/post balance check will need to be done to calculate the amountSentLD/amountReceivedLD.
 */
abstract contract OFTAdapter is OFTCore {
    using SafeERC20 for IERC20;

    IERC20 internal immutable innerToken;

    /**
     * @dev Constructor for the OFTAdapter contract.
     * @param _token The address of the ERC-20 token to be adapted.
     * @param _lzEndpoint The LayerZero endpoint address.
     * @param _delegate The delegate capable of making OApp configurations inside of the endpoint.
     */
    constructor(
        address _token,
        address _lzEndpoint,
        address _delegate
    ) OFTCore(IERC20Metadata(_token).decimals(), _lzEndpoint, _delegate) {
        innerToken = IERC20(_token);
    }

    /**
     * @dev Retrieves the address of the underlying ERC20 implementation.
     * @return The address of the adapted ERC-20 token.
     *
     * @dev In the case of OFTAdapter, address(this) and erc20 are NOT the same contract.
     */
    function token() public view returns (address) {
        return address(innerToken);
    }

    /**
     * @notice Indicates whether the OFT contract requires approval of the 'token()' to send.
     * @return requiresApproval Needs approval of the underlying token implementation.
     *
     * @dev In the case of default OFTAdapter, approval is required.
     * @dev In non-default OFTAdapter contracts with something like mint and burn privileges, it would NOT need approval.
     */
    function approvalRequired() external pure virtual returns (bool) {
        return true;
    }

    /**
     * @dev Burns tokens from the sender's specified balance, ie. pull method.
     * @param _from The address to debit from.
     * @param _amountLD The amount of tokens to send in local decimals.
     * @param _minAmountLD The minimum amount to send in local decimals.
     * @param _dstEid The destination chain ID.
     * @return amountSentLD The amount sent in local decimals.
     * @return amountReceivedLD The amount received in local decimals on the remote.
     *
     * @dev msg.sender will need to approve this _amountLD of tokens to be locked inside of the contract.
     * @dev WARNING: The default OFTAdapter implementation assumes LOSSLESS transfers, ie. 1 token in, 1 token out.
     * IF the 'innerToken' applies something like a transfer fee, the default will NOT work...
     * a pre/post balance check will need to be done to calculate the amountReceivedLD.
     */
    function _debit(
        address _from,
        uint256 _amountLD,
        uint256 _minAmountLD,
        uint32 _dstEid
    ) internal virtual override returns (uint256 amountSentLD, uint256 amountReceivedLD) {
        (amountSentLD, amountReceivedLD) = _debitView(_amountLD, _minAmountLD, _dstEid);
        // @dev Lock tokens by moving them into this contract from the caller.
        innerToken.safeTransferFrom(_from, address(this), amountSentLD);
    }

    /**
     * @dev Credits tokens to the specified address.
     * @param _to The address to credit the tokens to.
     * @param _amountLD The amount of tokens to credit in local decimals.
     * @dev _srcEid The source chain ID.
     * @return amountReceivedLD The amount of tokens ACTUALLY received in local decimals.
     *
     * @dev WARNING: The default OFTAdapter implementation assumes LOSSLESS transfers, ie. 1 token in, 1 token out.
     * IF the 'innerToken' applies something like a transfer fee, the default will NOT work...
     * a pre/post balance check will need to be done to calculate the amountReceivedLD.
     */
    function _credit(
        address _to,
        uint256 _amountLD,
        uint32 /*_srcEid*/
    ) internal virtual override returns (uint256 amountReceivedLD) {
        // @dev Unlock the tokens and transfer to the recipient.
        innerToken.safeTransfer(_to, _amountLD);
        // @dev In the case of NON-default OFTAdapter, the amountLD MIGHT not be == amountReceivedLD.
        return _amountLD;
    }
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

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

// @dev Oz5 implementation
// import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

import { OmniCounterAbstract, MsgCodec } from "./OmniCounterAbstract.sol";

contract OmniCounter is OmniCounterAbstract {
    // @dev Oz4 implementation
    constructor(address _endpoint, address _delegate) OmniCounterAbstract(_endpoint, _delegate) {}

    // @dev Oz5 implementation
    //    constructor(address _endpoint, address _delegate) OmniCounterAbstract(_endpoint, _delegate) Ownable(_delegate) {}
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

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import { IOAppReceiver, Origin } from "./interfaces/IOAppReceiver.sol";
import { OAppCore } from "./OAppCore.sol";

/**
 * @title OAppReceiver
 * @dev Abstract contract implementing the ILayerZeroReceiver interface and extending OAppCore for OApp receivers.
 */
abstract contract OAppReceiver is IOAppReceiver, OAppCore {
    // Custom error message for when the caller is not the registered endpoint/
    error OnlyEndpoint(address addr);

    // @dev The version of the OAppReceiver implementation.
    // @dev Version is bumped when changes are made to this contract.
    uint64 internal constant RECEIVER_VERSION = 2;

    /**
     * @notice Retrieves the OApp version information.
     * @return senderVersion The version of the OAppSender.sol contract.
     * @return receiverVersion The version of the OAppReceiver.sol contract.
     *
     * @dev Providing 0 as the default for OAppSender version. Indicates that the OAppSender is not implemented.
     * ie. this is a RECEIVE only OApp.
     * @dev If the OApp uses both OAppSender and OAppReceiver, then this needs to be override returning the correct versions.
     */
    function oAppVersion() public view virtual returns (uint64 senderVersion, uint64 receiverVersion) {
        return (0, RECEIVER_VERSION);
    }

    /**
     * @notice Indicates whether an address is an approved composeMsg sender to the Endpoint.
     * @dev _origin The origin information containing the source endpoint and sender address.
     *  - srcEid: The source chain endpoint ID.
     *  - sender: The sender address on the src chain.
     *  - nonce: The nonce of the message.
     * @dev _message The lzReceive payload.
     * @param _sender The sender address.
     * @return isSender Is a valid sender.
     *
     * @dev Applications can optionally choose to implement separate composeMsg senders that are NOT the bridging layer.
     * @dev The default sender IS the OAppReceiver implementer.
     */
    function isComposeMsgSender(
        Origin calldata /*_origin*/,
        bytes calldata /*_message*/,
        address _sender
    ) public view virtual returns (bool) {
        return _sender == address(this);
    }

    /**
     * @notice Checks if the path initialization is allowed based on the provided origin.
     * @param origin The origin information containing the source endpoint and sender address.
     * @return Whether the path has been initialized.
     *
     * @dev This indicates to the endpoint that the OApp has enabled msgs for this particular path to be received.
     * @dev This defaults to assuming if a peer has been set, its initialized.
     * Can be overridden by the OApp if there is other logic to determine this.
     */
    function allowInitializePath(Origin calldata origin) public view virtual returns (bool) {
        return peers[origin.srcEid] == origin.sender;
    }

    /**
     * @notice Retrieves the next nonce for a given source endpoint and sender address.
     * @dev _srcEid The source endpoint ID.
     * @dev _sender The sender address.
     * @return nonce The next nonce.
     *
     * @dev The path nonce starts from 1. If 0 is returned it means that there is NO nonce ordered enforcement.
     * @dev Is required by the off-chain executor to determine the OApp expects msg execution is ordered.
     * @dev This is also enforced by the OApp.
     * @dev By default this is NOT enabled. ie. nextNonce is hardcoded to return 0.
     */
    function nextNonce(uint32 /*_srcEid*/, bytes32 /*_sender*/) public view virtual returns (uint64 nonce) {
        return 0;
    }

    /**
     * @dev Entry point for receiving messages or packets from the endpoint.
     * @param _origin The origin information containing the source endpoint and sender address.
     *  - srcEid: The source chain endpoint ID.
     *  - sender: The sender address on the src chain.
     *  - nonce: The nonce of the message.
     * @param _guid The unique identifier for the received LayerZero message.
     * @param _message The payload of the received message.
     * @param _executor The address of the executor for the received message.
     * @param _extraData Additional arbitrary data provided by the corresponding executor.
     *
     * @dev Entry point for receiving msg/packet from the LayerZero endpoint.
     */
    function lzReceive(
        Origin calldata _origin,
        bytes32 _guid,
        bytes calldata _message,
        address _executor,
        bytes calldata _extraData
    ) public payable virtual {
        // Ensures that only the endpoint can attempt to lzReceive() messages to this OApp.
        if (address(endpoint) != msg.sender) revert OnlyEndpoint(msg.sender);

        // Ensure that the sender matches the expected peer for the source endpoint.
        if (_getPeerOrRevert(_origin.srcEid) != _origin.sender) revert OnlyPeer(_origin.srcEid, _origin.sender);

        // Call the internal OApp implementation of lzReceive.
        _lzReceive(_origin, _guid, _message, _executor, _extraData);
    }

    /**
     * @dev Internal function to implement lzReceive logic without needing to copy the basic parameter validation.
     */
    function _lzReceive(
        Origin calldata _origin,
        bytes32 _guid,
        bytes calldata _message,
        address _executor,
        bytes calldata _extraData
    ) internal virtual;
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import { OApp, Origin } from "../oapp/OApp.sol";
import { OAppOptionsType3 } from "../oapp/libs/OAppOptionsType3.sol";
import { IOAppMsgInspector } from "../oapp/interfaces/IOAppMsgInspector.sol";

import { OAppPreCrimeSimulator } from "../precrime/OAppPreCrimeSimulator.sol";

import { IOFT, SendParam, OFTLimit, OFTReceipt, OFTFeeDetail, MessagingReceipt, MessagingFee } from "./interfaces/IOFT.sol";
import { OFTMsgCodec } from "./libs/OFTMsgCodec.sol";
import { OFTComposeMsgCodec } from "./libs/OFTComposeMsgCodec.sol";

/**
 * @title OFTCore
 * @dev Abstract contract for the OftChain (OFT) token.
 */
abstract contract OFTCore is IOFT, OApp, OAppPreCrimeSimulator, OAppOptionsType3 {
    using OFTMsgCodec for bytes;
    using OFTMsgCodec for bytes32;

    // @notice Provides a conversion rate when swapping between denominations of SD and LD
    //      - shareDecimals == SD == shared Decimals
    //      - localDecimals == LD == local decimals
    // @dev Considers that tokens have different decimal amounts on various chains.
    // @dev eg.
    //  For a token
    //      - locally with 4 decimals --> 1.2345 => uint(12345)
    //      - remotely with 2 decimals --> 1.23 => uint(123)
    //      - The conversion rate would be 10 ** (4 - 2) = 100
    //  @dev If you want to send 1.2345 -> (uint 12345), you CANNOT represent that value on the remote,
    //  you can only display 1.23 -> uint(123).
    //  @dev To preserve the dust that would otherwise be lost on that conversion,
    //  we need to unify a denomination that can be represented on ALL chains inside of the OFT mesh
    uint256 public immutable decimalConversionRate;

    // @notice Msg types that are used to identify the various OFT operations.
    // @dev This can be extended in child contracts for non-default oft operations
    // @dev These values are used in things like combineOptions() in OAppOptionsType3.sol.
    uint16 public constant SEND = 1;
    uint16 public constant SEND_AND_CALL = 2;

    // Address of an optional contract to inspect both 'message' and 'options'
    address public msgInspector;
    event MsgInspectorSet(address inspector);

    /**
     * @dev Constructor.
     * @param _localDecimals The decimals of the token on the local chain (this chain).
     * @param _endpoint The address of the LayerZero endpoint.
     * @param _delegate The delegate capable of making OApp configurations inside of the endpoint.
     */
    constructor(uint8 _localDecimals, address _endpoint, address _delegate) OApp(_endpoint, _delegate) {
        if (_localDecimals < sharedDecimals()) revert InvalidLocalDecimals();
        decimalConversionRate = 10 ** (_localDecimals - sharedDecimals());
    }

    /**
     * @notice Retrieves interfaceID and the version of the OFT.
     * @return interfaceId The interface ID.
     * @return version The version.
     *
     * @dev interfaceId: This specific interface ID is '0x02e49c2c'.
     * @dev version: Indicates a cross-chain compatible msg encoding with other OFTs.
     * @dev If a new feature is added to the OFT cross-chain msg encoding, the version will be incremented.
     * ie. localOFT version(x,1) CAN send messages to remoteOFT version(x,1)
     */
    function oftVersion() external pure virtual returns (bytes4 interfaceId, uint64 version) {
        return (type(IOFT).interfaceId, 1);
    }

    /**
     * @dev Retrieves the shared decimals of the OFT.
     * @return The shared decimals of the OFT.
     *
     * @dev Sets an implicit cap on the amount of tokens, over uint64.max() will need some sort of outbound cap / totalSupply cap
     * Lowest common decimal denominator between chains.
     * Defaults to 6 decimal places to provide up to 18,446,744,073,709.551615 units (max uint64).
     * For tokens exceeding this totalSupply(), they will need to override the sharedDecimals function with something smaller.
     * ie. 4 sharedDecimals would be 1,844,674,407,370,955.1615
     */
    function sharedDecimals() public view virtual returns (uint8) {
        return 6;
    }

    /**
     * @dev Sets the message inspector address for the OFT.
     * @param _msgInspector The address of the message inspector.
     *
     * @dev This is an optional contract that can be used to inspect both 'message' and 'options'.
     * @dev Set it to address(0) to disable it, or set it to a contract address to enable it.
     */
    function setMsgInspector(address _msgInspector) public virtual onlyOwner {
        msgInspector = _msgInspector;
        emit MsgInspectorSet(_msgInspector);
    }

    /**
     * @notice Provides a quote for OFT-related operations.
     * @param _sendParam The parameters for the send operation.
     * @return oftLimit The OFT limit information.
     * @return oftFeeDetails The details of OFT fees.
     * @return oftReceipt The OFT receipt information.
     */
    function quoteOFT(
        SendParam calldata _sendParam
    )
        external
        view
        virtual
        returns (OFTLimit memory oftLimit, OFTFeeDetail[] memory oftFeeDetails, OFTReceipt memory oftReceipt)
    {
        uint256 minAmountLD = 0; // Unused in the default implementation.
        uint256 maxAmountLD = type(uint64).max; // Unused in the default implementation.
        oftLimit = OFTLimit(minAmountLD, maxAmountLD);

        // Unused in the default implementation; reserved for future complex fee details.
        oftFeeDetails = new OFTFeeDetail[](0);

        // @dev This is the same as the send() operation, but without the actual send.
        // - amountSentLD is the amount in local decimals that would be sent from the sender.
        // - amountReceivedLD is the amount in local decimals that will be credited to the recipient on the remote OFT instance.
        // @dev The amountSentLD MIGHT not equal the amount the user actually receives. HOWEVER, the default does.
        (uint256 amountSentLD, uint256 amountReceivedLD) = _debitView(
            _sendParam.amountLD,
            _sendParam.minAmountLD,
            _sendParam.dstEid
        );
        oftReceipt = OFTReceipt(amountSentLD, amountReceivedLD);
    }

    /**
     * @notice Provides a quote for the send() operation.
     * @param _sendParam The parameters for the send() operation.
     * @param _payInLzToken Flag indicating whether the caller is paying in the LZ token.
     * @return msgFee The calculated LayerZero messaging fee from the send() operation.
     *
     * @dev MessagingFee: LayerZero msg fee
     *  - nativeFee: The native fee.
     *  - lzTokenFee: The lzToken fee.
     */
    function quoteSend(
        SendParam calldata _sendParam,
        bool _payInLzToken
    ) external view virtual returns (MessagingFee memory msgFee) {
        // @dev mock the amount to receive, this is the same operation used in the send().
        // The quote is as similar as possible to the actual send() operation.
        (, uint256 amountReceivedLD) = _debitView(_sendParam.amountLD, _sendParam.minAmountLD, _sendParam.dstEid);

        // @dev Builds the options and OFT message to quote in the endpoint.
        (bytes memory message, bytes memory options) = _buildMsgAndOptions(_sendParam, amountReceivedLD);

        // @dev Calculates the LayerZero fee for the send() operation.
        return _quote(_sendParam.dstEid, message, options, _payInLzToken);
    }

    /**
     * @dev Executes the send operation.
     * @param _sendParam The parameters for the send operation.
     * @param _fee The calculated fee for the send() operation.
     *      - nativeFee: The native fee.
     *      - lzTokenFee: The lzToken fee.
     * @param _refundAddress The address to receive any excess funds.
     * @return msgReceipt The receipt for the send operation.
     * @return oftReceipt The OFT receipt information.
     *
     * @dev MessagingReceipt: LayerZero msg receipt
     *  - guid: The unique identifier for the sent message.
     *  - nonce: The nonce of the sent message.
     *  - fee: The LayerZero fee incurred for the message.
     */
    function send(
        SendParam calldata _sendParam,
        MessagingFee calldata _fee,
        address _refundAddress
    ) external payable virtual returns (MessagingReceipt memory msgReceipt, OFTReceipt memory oftReceipt) {
        // @dev Applies the token transfers regarding this send() operation.
        // - amountSentLD is the amount in local decimals that was ACTUALLY sent/debited from the sender.
        // - amountReceivedLD is the amount in local decimals that will be received/credited to the recipient on the remote OFT instance.
        (uint256 amountSentLD, uint256 amountReceivedLD) = _debit(
            msg.sender,
            _sendParam.amountLD,
            _sendParam.minAmountLD,
            _sendParam.dstEid
        );

        // @dev Builds the options and OFT message to quote in the endpoint.
        (bytes memory message, bytes memory options) = _buildMsgAndOptions(_sendParam, amountReceivedLD);

        // @dev Sends the message to the LayerZero endpoint and returns the LayerZero msg receipt.
        msgReceipt = _lzSend(_sendParam.dstEid, message, options, _fee, _refundAddress);
        // @dev Formulate the OFT receipt.
        oftReceipt = OFTReceipt(amountSentLD, amountReceivedLD);

        emit OFTSent(msgReceipt.guid, _sendParam.dstEid, msg.sender, amountSentLD, amountReceivedLD);
    }

    /**
     * @dev Internal function to build the message and options.
     * @param _sendParam The parameters for the send() operation.
     * @param _amountLD The amount in local decimals.
     * @return message The encoded message.
     * @return options The encoded options.
     */
    function _buildMsgAndOptions(
        SendParam calldata _sendParam,
        uint256 _amountLD
    ) internal view virtual returns (bytes memory message, bytes memory options) {
        bool hasCompose;
        // @dev This generated message has the msg.sender encoded into the payload so the remote knows who the caller is.
        (message, hasCompose) = OFTMsgCodec.encode(
            _sendParam.to,
            _toSD(_amountLD),
            // @dev Must be include a non empty bytes if you want to compose, EVEN if you dont need it on the remote.
            // EVEN if you dont require an arbitrary payload to be sent... eg. '0x01'
            _sendParam.composeMsg
        );
        // @dev Change the msg type depending if its composed or not.
        uint16 msgType = hasCompose ? SEND_AND_CALL : SEND;
        // @dev Combine the callers _extraOptions with the enforced options via the OAppOptionsType3.
        options = combineOptions(_sendParam.dstEid, msgType, _sendParam.extraOptions);

        // @dev Optionally inspect the message and options depending if the OApp owner has set a msg inspector.
        // @dev If it fails inspection, needs to revert in the implementation. ie. does not rely on return boolean
        if (msgInspector != address(0)) IOAppMsgInspector(msgInspector).inspect(message, options);
    }

    /**
     * @dev Internal function to handle the receive on the LayerZero endpoint.
     * @param _origin The origin information.
     *  - srcEid: The source chain endpoint ID.
     *  - sender: The sender address from the src chain.
     *  - nonce: The nonce of the LayerZero message.
     * @param _guid The unique identifier for the received LayerZero message.
     * @param _message The encoded message.
     * @dev _executor The address of the executor.
     * @dev _extraData Additional data.
     */
    function _lzReceive(
        Origin calldata _origin,
        bytes32 _guid,
        bytes calldata _message,
        address /*_executor*/, // @dev unused in the default implementation.
        bytes calldata /*_extraData*/ // @dev unused in the default implementation.
    ) internal virtual override {
        // @dev The src sending chain doesnt know the address length on this chain (potentially non-evm)
        // Thus everything is bytes32() encoded in flight.
        address toAddress = _message.sendTo().bytes32ToAddress();
        // @dev Credit the amountLD to the recipient and return the ACTUAL amount the recipient received in local decimals
        uint256 amountReceivedLD = _credit(toAddress, _toLD(_message.amountSD()), _origin.srcEid);

        if (_message.isComposed()) {
            // @dev Proprietary composeMsg format for the OFT.
            bytes memory composeMsg = OFTComposeMsgCodec.encode(
                _origin.nonce,
                _origin.srcEid,
                amountReceivedLD,
                _message.composeMsg()
            );

            // @dev Stores the lzCompose payload that will be executed in a separate tx.
            // Standardizes functionality for executing arbitrary contract invocation on some non-evm chains.
            // @dev The off-chain executor will listen and process the msg based on the src-chain-callers compose options passed.
            // @dev The index is used when a OApp needs to compose multiple msgs on lzReceive.
            // For default OFT implementation there is only 1 compose msg per lzReceive, thus its always 0.
            endpoint.sendCompose(toAddress, _guid, 0 /* the index of the composed message*/, composeMsg);
        }

        emit OFTReceived(_guid, _origin.srcEid, toAddress, amountReceivedLD);
    }

    /**
     * @dev Internal function to handle the OAppPreCrimeSimulator simulated receive.
     * @param _origin The origin information.
     *  - srcEid: The source chain endpoint ID.
     *  - sender: The sender address from the src chain.
     *  - nonce: The nonce of the LayerZero message.
     * @param _guid The unique identifier for the received LayerZero message.
     * @param _message The LayerZero message.
     * @param _executor The address of the off-chain executor.
     * @param _extraData Arbitrary data passed by the msg executor.
     *
     * @dev Enables the preCrime simulator to mock sending lzReceive() messages,
     * routes the msg down from the OAppPreCrimeSimulator, and back up to the OAppReceiver.
     */
    function _lzReceiveSimulate(
        Origin calldata _origin,
        bytes32 _guid,
        bytes calldata _message,
        address _executor,
        bytes calldata _extraData
    ) internal virtual override {
        _lzReceive(_origin, _guid, _message, _executor, _extraData);
    }

    /**
     * @dev Check if the peer is considered 'trusted' by the OApp.
     * @param _eid The endpoint ID to check.
     * @param _peer The peer to check.
     * @return Whether the peer passed is considered 'trusted' by the OApp.
     *
     * @dev Enables OAppPreCrimeSimulator to check whether a potential Inbound Packet is from a trusted source.
     */
    function isPeer(uint32 _eid, bytes32 _peer) public view virtual override returns (bool) {
        return peers[_eid] == _peer;
    }

    /**
     * @dev Internal function to remove dust from the given local decimal amount.
     * @param _amountLD The amount in local decimals.
     * @return amountLD The amount after removing dust.
     *
     * @dev Prevents the loss of dust when moving amounts between chains with different decimals.
     * @dev eg. uint(123) with a conversion rate of 100 becomes uint(100).
     */
    function _removeDust(uint256 _amountLD) internal view virtual returns (uint256 amountLD) {
        return (_amountLD / decimalConversionRate) * decimalConversionRate;
    }

    /**
     * @dev Internal function to convert an amount from shared decimals into local decimals.
     * @param _amountSD The amount in shared decimals.
     * @return amountLD The amount in local decimals.
     */
    function _toLD(uint64 _amountSD) internal view virtual returns (uint256 amountLD) {
        return _amountSD * decimalConversionRate;
    }

    /**
     * @dev Internal function to convert an amount from local decimals into shared decimals.
     * @param _amountLD The amount in local decimals.
     * @return amountSD The amount in shared decimals.
     */
    function _toSD(uint256 _amountLD) internal view virtual returns (uint64 amountSD) {
        return uint64(_amountLD / decimalConversionRate);
    }

    /**
     * @dev Internal function to mock the amount mutation from a OFT debit() operation.
     * @param _amountLD The amount to send in local decimals.
     * @param _minAmountLD The minimum amount to send in local decimals.
     * @dev _dstEid The destination endpoint ID.
     * @return amountSentLD The amount sent, in local decimals.
     * @return amountReceivedLD The amount to be received on the remote chain, in local decimals.
     *
     * @dev This is where things like fees would be calculated and deducted from the amount to be received on the remote.
     */
    function _debitView(
        uint256 _amountLD,
        uint256 _minAmountLD,
        uint32 /*_dstEid*/
    ) internal view virtual returns (uint256 amountSentLD, uint256 amountReceivedLD) {
        // @dev Remove the dust so nothing is lost on the conversion between chains with different decimals for the token.
        amountSentLD = _removeDust(_amountLD);
        // @dev The amount to send is the same as amount received in the default implementation.
        amountReceivedLD = amountSentLD;

        // @dev Check for slippage.
        if (amountReceivedLD < _minAmountLD) {
            revert SlippageExceeded(amountReceivedLD, _minAmountLD);
        }
    }

    /**
     * @dev Internal function to perform a debit operation.
     * @param _from The address to debit.
     * @param _amountLD The amount to send in local decimals.
     * @param _minAmountLD The minimum amount to send in local decimals.
     * @param _dstEid The destination endpoint ID.
     * @return amountSentLD The amount sent in local decimals.
     * @return amountReceivedLD The amount received in local decimals on the remote.
     *
     * @dev Defined here but are intended to be overriden depending on the OFT implementation.
     * @dev Depending on OFT implementation the _amountLD could differ from the amountReceivedLD.
     */
    function _debit(
        address _from,
        uint256 _amountLD,
        uint256 _minAmountLD,
        uint32 _dstEid
    ) internal virtual returns (uint256 amountSentLD, uint256 amountReceivedLD);

    /**
     * @dev Internal function to perform a credit operation.
     * @param _to The address to credit.
     * @param _amountLD The amount to credit in local decimals.
     * @param _srcEid The source endpoint ID.
     * @return amountReceivedLD The amount ACTUALLY received in local decimals.
     *
     * @dev Defined here but are intended to be overriden depending on the OFT implementation.
     * @dev Depending on OFT implementation the _amountLD could differ from the amountReceivedLD.
     */
    function _credit(
        address _to,
        uint256 _amountLD,
        uint32 _srcEid
    ) internal virtual returns (uint256 amountReceivedLD);
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES
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

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import { ILayerZeroReceiver, Origin } from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroReceiver.sol";

interface IOAppReceiver is ILayerZeroReceiver {
    /**
     * @notice Indicates whether an address is an approved composeMsg sender to the Endpoint.
     * @param _origin The origin information containing the source endpoint and sender address.
     *  - srcEid: The source chain endpoint ID.
     *  - sender: The sender address on the src chain.
     *  - nonce: The nonce of the message.
     * @param _message The lzReceive payload.
     * @param _sender The sender address.
     * @return isSender Is a valid sender.
     *
     * @dev Applications can optionally choose to implement a separate composeMsg sender that is NOT the bridging layer.
     * @dev The default sender IS the OAppReceiver implementer.
     */
    function isComposeMsgSender(
        Origin calldata _origin,
        bytes calldata _message,
        address _sender
    ) external view returns (bool isSender);
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


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {stdJson} from "forge-std/StdJson.sol";

/**
 * @title DeploymentRegistry
 * @notice Registry for storing and retrieving deployment addresses
 * @dev Allows modular deployment scripts to share contract addresses
 * @dev Stores addresses per chain ID to prevent overwrites between networks
 */
contract DeploymentRegistry is Script {
    using stdJson for string;

    // Registry path is now dynamic based on chain ID
    // Example: broadcast/deployment-addresses-31337.json (for Anvil)
    //          broadcast/deployment-addresses-11155111.json (for Sepolia)
    function _getRegistryPath() internal view returns (string memory) {
        return string(abi.encodePacked("broadcast/deployment-addresses-", vm.toString(block.chainid), ".json"));
    }

    struct DeploymentAddresses {
        address create2Factory;
        address oracle;
        address dlfToken;
        address itryToken;
        address custodian;
        address bufferPool;
        address staking;
        address yieldDistributor;
        address controller;
        address itryAdapter;
        address shareAdapter;
        address wiTryVaultComposer;
    }

    /**
     * @notice Save deployment addresses to JSON file (chain-specific)
     */
    function saveAddresses(DeploymentAddresses memory addrs) internal {
        string memory registryPath = _getRegistryPath();
        string memory json = "deployment";

        vm.serializeAddress(json, "create2Factory", addrs.create2Factory);
        vm.serializeAddress(json, "oracle", addrs.oracle);
        vm.serializeAddress(json, "dlfToken", addrs.dlfToken);
        vm.serializeAddress(json, "itryToken", addrs.itryToken);
        vm.serializeAddress(json, "custodian", addrs.custodian);
        vm.serializeAddress(json, "bufferPool", addrs.bufferPool);
        vm.serializeAddress(json, "staking", addrs.staking);
        vm.serializeAddress(json, "yieldDistributor", addrs.yieldDistributor);
        vm.serializeAddress(json, "controller", addrs.controller);
        vm.serializeAddress(json, "itryAdapter", addrs.itryAdapter);
        vm.serializeAddress(json, "shareAdapter", addrs.shareAdapter);
        string memory finalJson = vm.serializeAddress(json, "wiTryVaultComposer", addrs.wiTryVaultComposer);

        vm.writeJson(finalJson, registryPath);
        console2.log("Saved deployment addresses to:", registryPath);
        console2.log("Chain ID:", block.chainid);
    }

    /**
     * @notice Load deployment addresses from JSON file (chain-specific)
     */
    function loadAddresses() internal view returns (DeploymentAddresses memory) {
        string memory registryPath = _getRegistryPath();
        string memory json = vm.readFile(registryPath);

        return DeploymentAddresses({
            create2Factory: json.readAddress(".create2Factory"),
            oracle: json.readAddress(".oracle"),
            dlfToken: json.readAddress(".dlfToken"),
            itryToken: json.readAddress(".itryToken"),
            custodian: json.readAddress(".custodian"),
            bufferPool: json.readAddress(".bufferPool"),
            staking: json.readAddress(".staking"),
            yieldDistributor: json.readAddress(".yieldDistributor"),
            controller: json.readAddress(".controller"),
            itryAdapter: json.readAddress(".itryAdapter"),
            shareAdapter: json.readAddress(".shareAdapter"),
            wiTryVaultComposer: json.readAddress(".wiTryVaultComposer")
        });
    }

    /**
     * @notice Check if deployment registry exists (chain-specific)
     */
    function registryExists() internal view returns (bool) {
        string memory registryPath = _getRegistryPath();
        try vm.readFile(registryPath) returns (string memory) {
            return true;
        } catch {
            return false;
        }
    }

    /**
     * @notice Clear deployment registry (chain-specific)
     */
    function clearRegistry() internal {
        string memory registryPath = _getRegistryPath();
        try vm.removeFile(registryPath) {
            console2.log("Cleared deployment registry:", registryPath);
        } catch {
            console2.log("No registry to clear");
        }
    }
}

