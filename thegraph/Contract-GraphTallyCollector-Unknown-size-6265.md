
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.27 || 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-small-strings
// solhint-disable gas-strict-inequalities
// solhint-disable function-max-lines
// forge-lint: disable-start(mixed-case-function, mixed-case-variable)

import { IGraphPayments } from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import { IGraphTallyCollector } from "@graphprotocol/interfaces/contracts/horizon/IGraphTallyCollector.sol";
import { IPaymentsCollector } from "@graphprotocol/interfaces/contracts/horizon/IPaymentsCollector.sol";

import { Authorizable } from "../../utilities/Authorizable.sol";
import { EIP712 } from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import { PPMMath } from "../../libraries/PPMMath.sol";

import { GraphDirectory } from "../../utilities/GraphDirectory.sol";
import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

/**
 * @title GraphTallyCollector contract
 * @author Edge & Node
 * @dev Implements the {IGraphTallyCollector}, {IPaymentCollector} and {IAuthorizable} interfaces.
 * @notice A payments collector contract that can be used to collect payments using a GraphTally RAV (Receipt Aggregate Voucher).
 * @dev Note that the contract expects the RAV aggregate value to be monotonically increasing, each successive RAV for the same
 * (data service-payer-receiver) tuple should have a value greater than the previous one. The contract will keep track of the tokens
 * already collected and calculate the difference to collect.
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
contract GraphTallyCollector is EIP712, GraphDirectory, Authorizable, IGraphTallyCollector {
    using PPMMath for uint256;

    /// @notice The EIP712 typehash for the ReceiptAggregateVoucher struct
    bytes32 private constant EIP712_RAV_TYPEHASH =
        keccak256(
            "ReceiptAggregateVoucher(bytes32 collectionId,address payer,address serviceProvider,address dataService,uint64 timestampNs,uint128 valueAggregate,bytes metadata)"
        );

    /// @notice Tracks the amount of tokens already collected by a data service from a payer to a receiver.
    /// @dev The collectionId provides a secondary key for grouping payment tracking if needed. Data services that do not require
    /// grouping can use the same collectionId for all payments (0x00 or some other default value).
    mapping(address dataService => mapping(bytes32 collectionId => mapping(address receiver => mapping(address payer => uint256 tokens))))
        public tokensCollected;

    /**
     * @notice Constructs a new instance of the GraphTallyCollector contract.
     * @param eip712Name The name of the EIP712 domain.
     * @param eip712Version The version of the EIP712 domain.
     * @param controller The address of the Graph controller.
     * @param revokeSignerThawingPeriod The duration (in seconds) in which a signer is thawing before they can be revoked.
     */
    constructor(
        string memory eip712Name,
        string memory eip712Version,
        address controller,
        uint256 revokeSignerThawingPeriod
    ) EIP712(eip712Name, eip712Version) GraphDirectory(controller) Authorizable(revokeSignerThawingPeriod) {}

    /**
     * @notice See {IGraphPayments.collect}.
     * @dev Requirements:
     * - Caller must be the data service the RAV was issued to.
     * - Signer of the RAV must be authorized to sign for the payer.
     * - Service provider must have an active provision with the data service to collect payments.
     * @notice REVERT: This function may revert if ECDSA.recover fails, check ECDSA library for details.
     * @param paymentType The payment type to collect
     * @param data Additional data required for the payment collection. Encoded as follows:
     * - SignedRAV `signedRAV`: The signed RAV
     * - uint256 `dataServiceCut`: The data service cut in PPM
     * - address `receiverDestination`: The address where the receiver's payment should be sent.
     * @return The amount of tokens collected
     */
    /// @inheritdoc IPaymentsCollector
    function collect(IGraphPayments.PaymentTypes paymentType, bytes calldata data) external override returns (uint256) {
        return _collect(paymentType, data, 0);
    }

    /// @inheritdoc IGraphTallyCollector
    function collect(
        IGraphPayments.PaymentTypes paymentType,
        bytes calldata data,
        uint256 tokensToCollect
    ) external override returns (uint256) {
        return _collect(paymentType, data, tokensToCollect);
    }

    /// @inheritdoc IGraphTallyCollector
    function recoverRAVSigner(SignedRAV calldata signedRAV) external view override returns (address) {
        return _recoverRAVSigner(signedRAV);
    }

    /// @inheritdoc IGraphTallyCollector
    function encodeRAV(ReceiptAggregateVoucher calldata rav) external view returns (bytes32) {
        return _encodeRAV(rav);
    }

    /**
     * @notice See {IPaymentsCollector.collect}
     * This variant adds the ability to partially collect a RAV by specifying the amount of tokens to collect.
     * @param _paymentType The payment type to collect
     * @param _data Additional data required for the payment collection
     * @param _tokensToCollect The amount of tokens to collect. If 0, all tokens from the RAV will be collected.
     * @return The amount of tokens collected
     */
    function _collect(
        IGraphPayments.PaymentTypes _paymentType,
        bytes calldata _data,
        uint256 _tokensToCollect
    ) private returns (uint256) {
        (SignedRAV memory signedRAV, uint256 dataServiceCut, address receiverDestination) = abi.decode(
            _data,
            (SignedRAV, uint256, address)
        );

        // Ensure caller is the RAV data service
        require(
            signedRAV.rav.dataService == msg.sender,
            GraphTallyCollectorCallerNotDataService(msg.sender, signedRAV.rav.dataService)
        );

        // Ensure RAV signer is authorized for the payer
        _requireAuthorizedSigner(signedRAV);

        bytes32 collectionId = signedRAV.rav.collectionId;
        address dataService = signedRAV.rav.dataService;
        address receiver = signedRAV.rav.serviceProvider;

        // Check the service provider has an active provision with the data service
        // This prevents an attack where the payer can deny the service provider from collecting payments
        // by using a signer as data service to syphon off the tokens in the escrow to an account they control
        {
            uint256 tokensAvailable = _graphStaking().getProviderTokensAvailable(
                signedRAV.rav.serviceProvider,
                signedRAV.rav.dataService
            );
            require(tokensAvailable > 0, GraphTallyCollectorUnauthorizedDataService(signedRAV.rav.dataService));
        }

        uint256 tokensToCollect = 0;
        {
            uint256 tokensRAV = signedRAV.rav.valueAggregate;
            uint256 tokensAlreadyCollected = tokensCollected[dataService][collectionId][receiver][signedRAV.rav.payer];
            require(
                tokensRAV > tokensAlreadyCollected,
                GraphTallyCollectorInconsistentRAVTokens(tokensRAV, tokensAlreadyCollected)
            );

            if (_tokensToCollect == 0) {
                tokensToCollect = tokensRAV - tokensAlreadyCollected;
            } else {
                require(
                    _tokensToCollect <= tokensRAV - tokensAlreadyCollected,
                    GraphTallyCollectorInvalidTokensToCollectAmount(
                        _tokensToCollect,
                        tokensRAV - tokensAlreadyCollected
                    )
                );
                tokensToCollect = _tokensToCollect;
            }
        }

        if (tokensToCollect > 0) {
            tokensCollected[dataService][collectionId][receiver][signedRAV.rav.payer] += tokensToCollect;
            _graphPaymentsEscrow().collect(
                _paymentType,
                signedRAV.rav.payer,
                receiver,
                tokensToCollect,
                dataService,
                dataServiceCut,
                receiverDestination
            );
        }

        emit PaymentCollected(_paymentType, collectionId, signedRAV.rav.payer, receiver, dataService, tokensToCollect);

        // This event is emitted to allow reconstructing RAV history with onchain data.
        emit RAVCollected(
            collectionId,
            signedRAV.rav.payer,
            receiver,
            dataService,
            signedRAV.rav.timestampNs,
            signedRAV.rav.valueAggregate,
            signedRAV.rav.metadata,
            signedRAV.signature
        );

        return tokensToCollect;
    }

    /**
     * @notice Recovers the signer address of a signed ReceiptAggregateVoucher (RAV)
     * @param _signedRAV The SignedRAV containing the RAV and its signature
     * @return The address of the signer
     */
    function _recoverRAVSigner(SignedRAV memory _signedRAV) private view returns (address) {
        bytes32 messageHash = _encodeRAV(_signedRAV.rav);
        return ECDSA.recover(messageHash, _signedRAV.signature);
    }

    /**
     * @notice Computes the hash of a ReceiptAggregateVoucher (RAV)
     * @param _rav The RAV for which to compute the hash
     * @return The hash of the RAV
     */
    function _encodeRAV(ReceiptAggregateVoucher memory _rav) private view returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        EIP712_RAV_TYPEHASH,
                        _rav.collectionId,
                        _rav.payer,
                        _rav.serviceProvider,
                        _rav.dataService,
                        _rav.timestampNs,
                        _rav.valueAggregate,
                        keccak256(_rav.metadata)
                    )
                )
            );
    }

    /**
     * @notice Reverts if the RAV signer is not authorized by the payer
     * @param _signedRAV The signed RAV
     */
    function _requireAuthorizedSigner(SignedRAV memory _signedRAV) private view {
        require(
            _isAuthorized(_signedRAV.rav.payer, _recoverRAVSigner(_signedRAV)),
            GraphTallyCollectorInvalidRAVSigner()
        );
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity 0.8.27 || 0.8.33;

import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import { IHorizonStaking } from "@graphprotocol/interfaces/contracts/horizon/IHorizonStaking.sol";
import { IGraphPayments } from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import { IPaymentsEscrow } from "@graphprotocol/interfaces/contracts/horizon/IPaymentsEscrow.sol";

import { IController } from "@graphprotocol/interfaces/contracts/contracts/governance/IController.sol";
import { IEpochManager } from "@graphprotocol/interfaces/contracts/contracts/epochs/IEpochManager.sol";
import { IRewardsManager } from "@graphprotocol/interfaces/contracts/contracts/rewards/IRewardsManager.sol";
import { ITokenGateway } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/ITokenGateway.sol";
import { IGraphProxyAdmin } from "@graphprotocol/interfaces/contracts/contracts/upgrades/IGraphProxyAdmin.sol";

import { ICuration } from "@graphprotocol/interfaces/contracts/contracts/curation/ICuration.sol";

/**
 * @title GraphDirectory contract
 * @author Edge & Node
 * @notice This contract is meant to be inherited by other contracts that
 * need to keep track of the addresses in Graph Horizon contracts.
 * It fetches the addresses from the Controller supplied during construction,
 * and uses immutable variables to minimize gas costs.
 */
abstract contract GraphDirectory {
    // -- Graph Horizon contracts --

    /// @notice The Graph Token contract address
    IGraphToken private immutable GRAPH_TOKEN;

    /// @notice The Horizon Staking contract address
    IHorizonStaking private immutable GRAPH_STAKING;

    /// @notice The Graph Payments contract address
    IGraphPayments private immutable GRAPH_PAYMENTS;

    /// @notice The Payments Escrow contract address
    IPaymentsEscrow private immutable GRAPH_PAYMENTS_ESCROW;

    // -- Graph periphery contracts --

    /// @notice The Graph Controller contract address
    IController private immutable GRAPH_CONTROLLER;

    /// @notice The Epoch Manager contract address
    IEpochManager private immutable GRAPH_EPOCH_MANAGER;

    /// @notice The Rewards Manager contract address
    IRewardsManager private immutable GRAPH_REWARDS_MANAGER;

    /// @notice The Token Gateway contract address
    ITokenGateway private immutable GRAPH_TOKEN_GATEWAY;

    /// @notice The Graph Proxy Admin contract address
    IGraphProxyAdmin private immutable GRAPH_PROXY_ADMIN;

    // -- Legacy Graph contracts --
    // These are required for backwards compatibility on HorizonStakingExtension
    // TRANSITION PERIOD: remove these once HorizonStakingExtension is removed

    /// @notice The Curation contract address
    ICuration private immutable GRAPH_CURATION;

    /**
     * @notice Emitted when the GraphDirectory is initialized
     * @param graphToken The Graph Token contract address
     * @param graphStaking The Horizon Staking contract address
     * @param graphPayments The Graph Payments contract address
     * @param graphEscrow The Payments Escrow contract address
     * @param graphController The Graph Controller contract address
     * @param graphEpochManager The Epoch Manager contract address
     * @param graphRewardsManager The Rewards Manager contract address
     * @param graphTokenGateway The Token Gateway contract address
     * @param graphProxyAdmin The Graph Proxy Admin contract address
     * @param graphCuration The Curation contract address
     */
    event GraphDirectoryInitialized(
        address indexed graphToken,
        address indexed graphStaking,
        address graphPayments,
        address graphEscrow,
        address indexed graphController,
        address graphEpochManager,
        address graphRewardsManager,
        address graphTokenGateway,
        address graphProxyAdmin,
        address graphCuration
    );

    /**
     * @notice Thrown when either the controller is the zero address or a contract address is not found
     * on the controller
     * @param contractName The name of the contract that was not found, or the controller
     */
    error GraphDirectoryInvalidZeroAddress(bytes contractName);

    /**
     * @notice Constructor for the GraphDirectory contract
     * @dev Requirements:
     * - `controller` cannot be zero address
     *
     * Emits a {GraphDirectoryInitialized} event
     *
     * @param controller The address of the Graph Controller contract.
     */
    constructor(address controller) {
        require(controller != address(0), GraphDirectoryInvalidZeroAddress("Controller"));

        GRAPH_CONTROLLER = IController(controller);
        GRAPH_TOKEN = IGraphToken(_getContractFromController("GraphToken"));
        GRAPH_STAKING = IHorizonStaking(_getContractFromController("Staking"));
        GRAPH_PAYMENTS = IGraphPayments(_getContractFromController("GraphPayments"));
        GRAPH_PAYMENTS_ESCROW = IPaymentsEscrow(_getContractFromController("PaymentsEscrow"));
        GRAPH_EPOCH_MANAGER = IEpochManager(_getContractFromController("EpochManager"));
        GRAPH_REWARDS_MANAGER = IRewardsManager(_getContractFromController("RewardsManager"));
        GRAPH_TOKEN_GATEWAY = ITokenGateway(_getContractFromController("GraphTokenGateway"));
        GRAPH_PROXY_ADMIN = IGraphProxyAdmin(_getContractFromController("GraphProxyAdmin"));
        GRAPH_CURATION = ICuration(_getContractFromController("Curation"));

        emit GraphDirectoryInitialized(
            address(GRAPH_TOKEN),
            address(GRAPH_STAKING),
            address(GRAPH_PAYMENTS),
            address(GRAPH_PAYMENTS_ESCROW),
            address(GRAPH_CONTROLLER),
            address(GRAPH_EPOCH_MANAGER),
            address(GRAPH_REWARDS_MANAGER),
            address(GRAPH_TOKEN_GATEWAY),
            address(GRAPH_PROXY_ADMIN),
            address(GRAPH_CURATION)
        );
    }

    /**
     * @notice Get the Graph Token contract
     * @return The Graph Token contract
     */
    function _graphToken() internal view returns (IGraphToken) {
        return GRAPH_TOKEN;
    }

    /**
     * @notice Get the Horizon Staking contract
     * @return The Horizon Staking contract
     */
    function _graphStaking() internal view returns (IHorizonStaking) {
        return GRAPH_STAKING;
    }

    /**
     * @notice Get the Graph Payments contract
     * @return The Graph Payments contract
     */
    function _graphPayments() internal view returns (IGraphPayments) {
        return GRAPH_PAYMENTS;
    }

    /**
     * @notice Get the Payments Escrow contract
     * @return The Payments Escrow contract
     */
    function _graphPaymentsEscrow() internal view returns (IPaymentsEscrow) {
        return GRAPH_PAYMENTS_ESCROW;
    }

    /**
     * @notice Get the Graph Controller contract
     * @return The Graph Controller contract
     */
    function _graphController() internal view returns (IController) {
        return GRAPH_CONTROLLER;
    }

    /**
     * @notice Get the Epoch Manager contract
     * @return The Epoch Manager contract
     */
    function _graphEpochManager() internal view returns (IEpochManager) {
        return GRAPH_EPOCH_MANAGER;
    }

    /**
     * @notice Get the Rewards Manager contract
     * @return The Rewards Manager contract address
     */
    function _graphRewardsManager() internal view returns (IRewardsManager) {
        return GRAPH_REWARDS_MANAGER;
    }

    /**
     * @notice Get the Graph Token Gateway contract
     * @return The Graph Token Gateway contract
     */
    function _graphTokenGateway() internal view returns (ITokenGateway) {
        return GRAPH_TOKEN_GATEWAY;
    }

    /**
     * @notice Get the Graph Proxy Admin contract
     * @return The Graph Proxy Admin contract
     */
    function _graphProxyAdmin() internal view returns (IGraphProxyAdmin) {
        return GRAPH_PROXY_ADMIN;
    }

    /**
     * @notice Get the Curation contract
     * @return The Curation contract
     */
    function _graphCuration() internal view returns (ICuration) {
        return GRAPH_CURATION;
    }

    /**
     * @notice Get a contract address from the controller
     * @dev Requirements:
     * - The `_contractName` must be registered in the controller
     * @param _contractName The name of the contract to fetch from the controller
     * @return The address of the contract
     */
    function _getContractFromController(bytes memory _contractName) private view returns (address) {
        address contractAddress = GRAPH_CONTROLLER.getContractProxy(keccak256(_contractName));
        require(contractAddress != address(0), GraphDirectoryInvalidZeroAddress(_contractName));
        return contractAddress;
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.27 || 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-strict-inequalities

import { IAuthorizable } from "@graphprotocol/interfaces/contracts/horizon/IAuthorizable.sol";

import { ECDSA } from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import { MessageHashUtils } from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

/**
 * @title Authorizable contract
 * @author Edge & Node
 * @dev Implements the {IAuthorizable} interface.
 * @notice A mechanism to authorize signers to sign messages on behalf of an authorizer.
 * Signers cannot be reused for different authorizers.
 * @dev Contract uses "authorizeSignerProof" as the domain for signer proofs.
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
abstract contract Authorizable is IAuthorizable {
    /// @notice The duration (in seconds) for which an authorization is thawing before it can be revoked
    uint256 public immutable REVOKE_AUTHORIZATION_THAWING_PERIOD;

    /// @notice Authorization details for authorizer-signer pairs
    mapping(address signer => Authorization authorization) public authorizations;

    /**
     * @dev Revert if the caller has not authorized the signer
     * @param signer The address of the signer
     */
    modifier onlyAuthorized(address signer) {
        _requireAuthorized(msg.sender, signer);
        _;
    }

    /**
     * @notice Constructs a new instance of the Authorizable contract.
     * @param revokeAuthorizationThawingPeriod The duration (in seconds) for which an authorization is thawing before it can be revoked.
     */
    constructor(uint256 revokeAuthorizationThawingPeriod) {
        REVOKE_AUTHORIZATION_THAWING_PERIOD = revokeAuthorizationThawingPeriod;
    }

    /// @inheritdoc IAuthorizable
    function authorizeSigner(address signer, uint256 proofDeadline, bytes calldata proof) external {
        require(
            authorizations[signer].authorizer == address(0),
            AuthorizableSignerAlreadyAuthorized(
                authorizations[signer].authorizer,
                signer,
                authorizations[signer].revoked
            )
        );
        _verifyAuthorizationProof(proof, proofDeadline, signer);
        authorizations[signer].authorizer = msg.sender;
        emit SignerAuthorized(msg.sender, signer);
    }

    /// @inheritdoc IAuthorizable
    function thawSigner(address signer) external onlyAuthorized(signer) {
        authorizations[signer].thawEndTimestamp = block.timestamp + REVOKE_AUTHORIZATION_THAWING_PERIOD;
        emit SignerThawing(msg.sender, signer, authorizations[signer].thawEndTimestamp);
    }

    /// @inheritdoc IAuthorizable
    function cancelThawSigner(address signer) external onlyAuthorized(signer) {
        require(authorizations[signer].thawEndTimestamp > 0, AuthorizableSignerNotThawing(signer));
        uint256 thawEnd = authorizations[signer].thawEndTimestamp;
        authorizations[signer].thawEndTimestamp = 0;
        emit SignerThawCanceled(msg.sender, signer, thawEnd);
    }

    /// @inheritdoc IAuthorizable
    function revokeAuthorizedSigner(address signer) external onlyAuthorized(signer) {
        uint256 thawEndTimestamp = authorizations[signer].thawEndTimestamp;
        require(thawEndTimestamp > 0, AuthorizableSignerNotThawing(signer));
        require(thawEndTimestamp <= block.timestamp, AuthorizableSignerStillThawing(block.timestamp, thawEndTimestamp));
        authorizations[signer].revoked = true;
        emit SignerRevoked(msg.sender, signer);
    }

    /// @inheritdoc IAuthorizable
    function getThawEnd(address signer) external view returns (uint256) {
        return authorizations[signer].thawEndTimestamp;
    }

    /// @inheritdoc IAuthorizable
    function isAuthorized(address authorizer, address signer) external view returns (bool) {
        return _isAuthorized(authorizer, signer);
    }

    /**
     * @notice Returns true if the signer is authorized by the authorizer
     * @param _authorizer The address of the authorizer
     * @param _signer The address of the signer
     * @return true if the signer is authorized by the authorizer, false otherwise
     */
    function _isAuthorized(address _authorizer, address _signer) internal view returns (bool) {
        return (_authorizer != address(0) &&
            authorizations[_signer].authorizer == _authorizer &&
            !authorizations[_signer].revoked);
    }

    /**
     * @notice Reverts if the authorizer has not authorized the signer
     * @param _authorizer The address of the authorizer
     * @param _signer The address of the signer
     */
    function _requireAuthorized(address _authorizer, address _signer) internal view {
        require(_isAuthorized(_authorizer, _signer), AuthorizableSignerNotAuthorized(_authorizer, _signer));
    }

    /**
     * @notice Verify the authorization proof provided by the authorizer
     * @param _proof The proof provided by the authorizer
     * @param _proofDeadline The deadline by which the proof must be verified
     * @param _signer The authorization recipient
     */
    function _verifyAuthorizationProof(bytes calldata _proof, uint256 _proofDeadline, address _signer) private view {
        // Check that the proofDeadline has not passed
        require(
            _proofDeadline > block.timestamp,
            AuthorizableInvalidSignerProofDeadline(_proofDeadline, block.timestamp)
        );

        // Generate the message hash
        // forge-lint: disable-next-item(asm-keccak256)
        bytes32 messageHash = keccak256(
            abi.encodePacked(block.chainid, address(this), "authorizeSignerProof", _proofDeadline, msg.sender)
        );

        // Generate the allegedly signed digest
        bytes32 digest = MessageHashUtils.toEthSignedMessageHash(messageHash);

        // Verify that the recovered signer matches the to be authorized signer
        require(ECDSA.recover(digest, _proof) == _signer, AuthorizableInvalidSignerProof());
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.27 || 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-strict-inequalities
// forge-lint: disable-start(mixed-case-function)

/**
 * @title PPMMath library
 * @author Edge & Node
 * @notice A library for handling calculations with parts per million (PPM) amounts.
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
library PPMMath {
    /// @notice Maximum value (100%) in parts per million (PPM).
    uint256 internal constant MAX_PPM = 1_000_000;

    /**
     * @notice Thrown when a value is expected to be in PPM but is not.
     * @param value The value that is not in PPM.
     */
    error PPMMathInvalidPPM(uint256 value);

    /**
     * @notice Thrown when no value in a multiplication is in PPM.
     * @param a The first value in the multiplication.
     * @param b The second value in the multiplication.
     */
    error PPMMathInvalidMulPPM(uint256 a, uint256 b);

    /**
     * @notice Multiplies two values, one of which must be in PPM.
     * @param a The first value.
     * @param b The second value.
     * @return The result of the multiplication.
     */
    function mulPPM(uint256 a, uint256 b) internal pure returns (uint256) {
        require(isValidPPM(a) || isValidPPM(b), PPMMathInvalidMulPPM(a, b));
        return (a * b) / MAX_PPM;
    }

    /**
     * @notice Multiplies two values, the second one must be in PPM, and rounds up the result.
     * @dev requirements:
     * - The second value must be in PPM.
     * @param a The first value.
     * @param b The second value.
     * @return The result of the multiplication.
     */
    function mulPPMRoundUp(uint256 a, uint256 b) internal pure returns (uint256) {
        require(isValidPPM(b), PPMMathInvalidPPM(b));
        return a - mulPPM(a, MAX_PPM - b);
    }

    /**
     * @notice Checks if a value is in PPM.
     * @dev A valid PPM value is between 0 and MAX_PPM.
     * @param value The value to check.
     * @return true if the value is in PPM, false otherwise.
     */
    function isValidPPM(uint256 value) internal pure returns (bool) {
        return value <= MAX_PPM;
    }
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

