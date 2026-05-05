
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.27 || 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-strict-inequalities

import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import { IGraphPayments } from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import { IPaymentsEscrow } from "@graphprotocol/interfaces/contracts/horizon/IPaymentsEscrow.sol";

import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import { MulticallUpgradeable } from "@openzeppelin/contracts-upgradeable/utils/MulticallUpgradeable.sol";
import { TokenUtils } from "@graphprotocol/contracts/contracts/utils/TokenUtils.sol";

import { GraphDirectory } from "../utilities/GraphDirectory.sol";

/**
 * @title PaymentsEscrow contract
 * @author Edge & Node
 * @dev Implements the {IPaymentsEscrow} interface
 * @notice This contract is part of the Graph Horizon payments protocol. It holds the funds (GRT)
 * for payments made through the payments protocol for services provided
 * via a Graph Horizon data service.
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
contract PaymentsEscrow is Initializable, MulticallUpgradeable, GraphDirectory, IPaymentsEscrow {
    using TokenUtils for IGraphToken;

    /// @notice The maximum thawing period (in seconds) for both escrow withdrawal and collector revocation
    /// @dev This is a precautionary measure to avoid inadvertedly locking funds for too long
    uint256 public constant MAX_WAIT_PERIOD = 90 days;

    /// @notice Thawing period in seconds for escrow funds withdrawal
    uint256 public immutable WITHDRAW_ESCROW_THAWING_PERIOD;

    /// @notice Escrow account details for payer-collector-receiver tuples
    mapping(address payer => mapping(address collector => mapping(address receiver => IPaymentsEscrow.EscrowAccount escrowAccount)))
        public escrowAccounts;

    // forge-lint: disable-next-item(unwrapped-modifier-logic)
    /**
     * @notice Modifier to prevent function execution when contract is paused
     * @dev Reverts if the controller indicates the contract is paused
     */
    modifier notPaused() {
        require(!_graphController().paused(), PaymentsEscrowIsPaused());
        _;
    }

    /**
     * @notice Construct the PaymentsEscrow contract
     * @param controller The address of the controller
     * @param withdrawEscrowThawingPeriod Thawing period in seconds for escrow funds withdrawal
     */
    constructor(address controller, uint256 withdrawEscrowThawingPeriod) GraphDirectory(controller) {
        require(
            withdrawEscrowThawingPeriod <= MAX_WAIT_PERIOD,
            PaymentsEscrowThawingPeriodTooLong(withdrawEscrowThawingPeriod, MAX_WAIT_PERIOD)
        );

        WITHDRAW_ESCROW_THAWING_PERIOD = withdrawEscrowThawingPeriod;
        _disableInitializers();
    }

    /// @inheritdoc IPaymentsEscrow
    function initialize() external initializer {
        __Multicall_init();
    }

    /// @inheritdoc IPaymentsEscrow
    function deposit(address collector, address receiver, uint256 tokens) external override notPaused {
        _deposit(msg.sender, collector, receiver, tokens);
    }

    /// @inheritdoc IPaymentsEscrow
    function depositTo(address payer, address collector, address receiver, uint256 tokens) external override notPaused {
        _deposit(payer, collector, receiver, tokens);
    }

    /// @inheritdoc IPaymentsEscrow
    function thaw(address collector, address receiver, uint256 tokens) external override notPaused {
        require(tokens > 0, PaymentsEscrowInvalidZeroTokens());

        EscrowAccount storage account = escrowAccounts[msg.sender][collector][receiver];
        require(account.balance >= tokens, PaymentsEscrowInsufficientBalance(account.balance, tokens));

        account.tokensThawing = tokens;
        account.thawEndTimestamp = block.timestamp + WITHDRAW_ESCROW_THAWING_PERIOD;

        emit Thaw(msg.sender, collector, receiver, tokens, account.thawEndTimestamp);
    }

    /// @inheritdoc IPaymentsEscrow
    function cancelThaw(address collector, address receiver) external override notPaused {
        EscrowAccount storage account = escrowAccounts[msg.sender][collector][receiver];
        require(account.tokensThawing != 0, PaymentsEscrowNotThawing());

        uint256 tokensThawing = account.tokensThawing;
        uint256 thawEndTimestamp = account.thawEndTimestamp;
        account.tokensThawing = 0;
        account.thawEndTimestamp = 0;

        emit CancelThaw(msg.sender, collector, receiver, tokensThawing, thawEndTimestamp);
    }

    /// @inheritdoc IPaymentsEscrow
    function withdraw(address collector, address receiver) external override notPaused {
        EscrowAccount storage account = escrowAccounts[msg.sender][collector][receiver];
        require(account.thawEndTimestamp != 0, PaymentsEscrowNotThawing());
        require(
            account.thawEndTimestamp < block.timestamp,
            PaymentsEscrowStillThawing(block.timestamp, account.thawEndTimestamp)
        );

        // Amount is the minimum between the amount being thawed and the actual balance
        uint256 tokens = account.tokensThawing > account.balance ? account.balance : account.tokensThawing;

        account.balance -= tokens;
        account.tokensThawing = 0;
        account.thawEndTimestamp = 0;
        _graphToken().pushTokens(msg.sender, tokens);
        emit Withdraw(msg.sender, collector, receiver, tokens);
    }

    /// @inheritdoc IPaymentsEscrow
    function collect(
        IGraphPayments.PaymentTypes paymentType,
        address payer,
        address receiver,
        uint256 tokens,
        address dataService,
        uint256 dataServiceCut,
        address receiverDestination
    ) external override notPaused {
        // Check if there are enough funds in the escrow account
        EscrowAccount storage account = escrowAccounts[payer][msg.sender][receiver];
        require(account.balance >= tokens, PaymentsEscrowInsufficientBalance(account.balance, tokens));

        // Reduce amount from account balance
        account.balance -= tokens;

        // Cap tokensThawing to the new balance to keep state consistent
        if (account.tokensThawing > account.balance) {
            account.tokensThawing = account.balance;
            if (account.tokensThawing == 0) {
                account.thawEndTimestamp = 0;
            }
        }

        uint256 escrowBalanceBefore = _graphToken().balanceOf(address(this));

        _graphToken().approve(address(_graphPayments()), tokens);
        _graphPayments().collect(paymentType, receiver, tokens, dataService, dataServiceCut, receiverDestination);

        // Verify that the escrow balance is consistent with the collected tokens
        uint256 escrowBalanceAfter = _graphToken().balanceOf(address(this));
        require(
            escrowBalanceBefore == tokens + escrowBalanceAfter,
            PaymentsEscrowInconsistentCollection(escrowBalanceBefore, escrowBalanceAfter, tokens)
        );

        emit EscrowCollected(paymentType, payer, msg.sender, receiver, tokens, receiverDestination);
    }

    /// @inheritdoc IPaymentsEscrow
    function getBalance(address payer, address collector, address receiver) external view override returns (uint256) {
        EscrowAccount storage account = escrowAccounts[payer][collector][receiver];
        return account.balance > account.tokensThawing ? account.balance - account.tokensThawing : 0;
    }

    /**
     * @notice Deposits funds into the escrow for a payer-collector-receiver tuple, where
     * the payer is the transaction caller.
     * @param _payer The address of the payer
     * @param _collector The address of the collector
     * @param _receiver The address of the receiver
     * @param _tokens The amount of tokens to deposit
     */
    function _deposit(address _payer, address _collector, address _receiver, uint256 _tokens) private {
        escrowAccounts[_payer][_collector][_receiver].balance += _tokens;
        _graphToken().pullTokens(msg.sender, _tokens);
        emit Deposit(_payer, _collector, _receiver, _tokens);
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


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

