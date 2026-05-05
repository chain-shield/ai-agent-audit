
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.27 || 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable function-max-lines

import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import { IGraphPayments } from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import { IHorizonStakingTypes } from "@graphprotocol/interfaces/contracts/horizon/internal/IHorizonStakingTypes.sol";

import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import { MulticallUpgradeable } from "@openzeppelin/contracts-upgradeable/utils/MulticallUpgradeable.sol";
import { TokenUtils } from "@graphprotocol/contracts/contracts/utils/TokenUtils.sol";
import { PPMMath } from "../libraries/PPMMath.sol";

import { GraphDirectory } from "../utilities/GraphDirectory.sol";

/**
 * @title GraphPayments contract
 * @author Edge & Node
 * @notice This contract is part of the Graph Horizon payments protocol. It's designed
 * to pull funds (GRT) from the {PaymentsEscrow} and distribute them according to a
 * set of pre established rules.
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
contract GraphPayments is Initializable, MulticallUpgradeable, GraphDirectory, IGraphPayments {
    using TokenUtils for IGraphToken;
    using PPMMath for uint256;

    /// @notice Protocol payment cut in PPM
    uint256 public immutable PROTOCOL_PAYMENT_CUT;

    /**
     * @notice Constructor for the {GraphPayments} contract
     * @dev This contract is upgradeable however we still use the constructor to set
     * a few immutable variables.
     * @param controller The address of the Graph controller
     * @param protocolPaymentCut The protocol tax in PPM
     */
    constructor(address controller, uint256 protocolPaymentCut) GraphDirectory(controller) {
        require(PPMMath.isValidPPM(protocolPaymentCut), GraphPaymentsInvalidCut(protocolPaymentCut));
        PROTOCOL_PAYMENT_CUT = protocolPaymentCut;
        _disableInitializers();
    }

    /// @inheritdoc IGraphPayments
    function initialize() external initializer {
        __Multicall_init();
    }

    /// @inheritdoc IGraphPayments
    function collect(
        IGraphPayments.PaymentTypes paymentType,
        address receiver,
        uint256 tokens,
        address dataService,
        uint256 dataServiceCut,
        address receiverDestination
    ) external {
        require(PPMMath.isValidPPM(dataServiceCut), GraphPaymentsInvalidCut(dataServiceCut));

        // Pull tokens from the sender
        _graphToken().pullTokens(msg.sender, tokens);

        // Calculate token amounts for each party
        // Order matters: protocol -> data service -> delegators -> receiver
        // Note the substractions should not underflow as we are only deducting a percentage of the remainder
        uint256 tokensRemaining = tokens;

        uint256 tokensProtocol = tokensRemaining.mulPPMRoundUp(PROTOCOL_PAYMENT_CUT);
        tokensRemaining = tokensRemaining - tokensProtocol;

        uint256 tokensDataService = tokensRemaining.mulPPMRoundUp(dataServiceCut);
        tokensRemaining = tokensRemaining - tokensDataService;

        uint256 tokensDelegationPool = 0;
        IHorizonStakingTypes.DelegationPool memory pool = _graphStaking().getDelegationPool(receiver, dataService);
        if (pool.shares > 0) {
            tokensDelegationPool = tokensRemaining.mulPPMRoundUp(
                _graphStaking().getDelegationFeeCut(receiver, dataService, paymentType)
            );
            tokensRemaining = tokensRemaining - tokensDelegationPool;
        }

        // Pay all parties
        _graphToken().burnTokens(tokensProtocol);

        _graphToken().pushTokens(dataService, tokensDataService);

        if (tokensDelegationPool > 0) {
            _graphToken().approve(address(_graphStaking()), tokensDelegationPool);
            _graphStaking().addToDelegationPool(receiver, dataService, tokensDelegationPool);
        }

        if (tokensRemaining > 0) {
            if (receiverDestination == address(0)) {
                _graphToken().approve(address(_graphStaking()), tokensRemaining);
                _graphStaking().stakeTo(receiver, tokensRemaining);
            } else {
                _graphToken().pushTokens(receiverDestination, tokensRemaining);
            }
        }

        emit GraphPaymentCollected(
            paymentType,
            msg.sender,
            receiver,
            dataService,
            tokens,
            tokensProtocol,
            tokensDataService,
            tokensDelegationPool,
            tokensRemaining,
            receiverDestination
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

