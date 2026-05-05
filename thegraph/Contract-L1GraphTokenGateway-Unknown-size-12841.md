
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;
pragma abicoder v2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-indexed-events, gas-strict-inequalities
// solhint-disable named-parameters-mapping

import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/Initializable.sol";
import { AddressUpgradeable } from "@openzeppelin/contracts-upgradeable/utils/AddressUpgradeable.sol";
import { SafeMathUpgradeable } from "@openzeppelin/contracts-upgradeable/math/SafeMathUpgradeable.sol";

import { L1ArbitrumMessenger } from "../arbitrum/L1ArbitrumMessenger.sol";
import { IBridge } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/IBridge.sol";
import { IInbox } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/IInbox.sol";
import { IOutbox } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/IOutbox.sol";
import { ITokenGateway } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/ITokenGateway.sol";
import { Managed } from "../governance/Managed.sol";
import { GraphTokenGateway } from "./GraphTokenGateway.sol";
import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";

/**
 * @title L1 Graph Token Gateway Contract
 * @author Edge & Node
 * @notice Provides the L1 side of the Ethereum-Arbitrum GRT bridge. Sends GRT to the L2 chain
 * by escrowing them and sending a message to the L2 gateway, and receives tokens from L2 by
 * releasing them from escrow.
 * Based on Offchain Labs' reference implementation and Livepeer's arbitrum-lpt-bridge
 * (See: https://github.com/OffchainLabs/arbitrum/tree/master/packages/arb-bridge-peripherals/contracts/tokenbridge
 * and https://github.com/livepeer/arbitrum-lpt-bridge)
 */
contract L1GraphTokenGateway is Initializable, GraphTokenGateway, L1ArbitrumMessenger {
    using SafeMathUpgradeable for uint256;

    /// @notice Address of the Graph Token contract on L2
    address public l2GRT;
    /// @notice Address of the Arbitrum Inbox
    address public inbox;
    /// @notice Address of the Arbitrum Gateway Router on L1
    address public l1Router;
    /// @notice Address of the L2GraphTokenGateway on L2 that is the counterpart of this gateway
    address public l2Counterpart;
    /// @notice Address of the BridgeEscrow contract that holds the GRT in the bridge
    address public escrow;
    /// @notice Addresses for which this mapping is true are allowed to send callhooks in outbound transfers
    mapping(address => bool) public callhookAllowlist;
    /// @notice Total amount minted from L2
    uint256 public totalMintedFromL2;
    /// @notice Accumulated allowance for tokens minted from L2 at lastL2MintAllowanceUpdateBlock
    uint256 public accumulatedL2MintAllowanceSnapshot;
    /// @notice Block at which new L2 allowance starts accumulating
    uint256 public lastL2MintAllowanceUpdateBlock;
    /// @notice New L2 mint allowance per block
    uint256 public l2MintAllowancePerBlock;

    /**
     * @notice Emitted when an outbound transfer is initiated, i.e. tokens are deposited from L1 to L2
     * @param l1Token Address of the L1 token being transferred
     * @param from Address sending the tokens on L1
     * @param to Address receiving the tokens on L2
     * @param sequenceNumber Sequence number of the retryable ticket
     * @param amount Amount of tokens transferred
     */
    event DepositInitiated(
        address l1Token,
        address indexed from,
        address indexed to,
        uint256 indexed sequenceNumber,
        uint256 amount
    );

    /**
     * @notice Emitted when an incoming transfer is finalized, i.e tokens are withdrawn from L2 to L1
     * @param l1Token Address of the L1 token being transferred
     * @param from Address sending the tokens on L2
     * @param to Address receiving the tokens on L1
     * @param exitNum Exit number (always 0 for this contract)
     * @param amount Amount of tokens transferred
     */
    event WithdrawalFinalized(
        address l1Token,
        address indexed from,
        address indexed to,
        uint256 indexed exitNum,
        uint256 amount
    );

    /**
     * @notice Emitted when the Arbitrum Inbox and Gateway Router addresses have been updated
     * @param inbox Address of the Arbitrum Inbox
     * @param l1Router Address of the L1 Gateway Router
     */
    event ArbitrumAddressesSet(address inbox, address l1Router);

    /**
     * @notice Emitted when the L2 GRT address has been updated
     * @param l2GRT Address of the L2 GRT contract
     */
    event L2TokenAddressSet(address l2GRT);

    /**
     * @notice Emitted when the counterpart L2GraphTokenGateway address has been updated
     * @param l2Counterpart Address of the L2 counterpart gateway
     */
    event L2CounterpartAddressSet(address l2Counterpart);
    /**
     * @notice Emitted when the escrow address has been updated
     * @param escrow Address of the escrow contract
     */
    event EscrowAddressSet(address escrow);

    /**
     * @notice Emitted when an address is added to the callhook allowlist
     * @param newAllowlisted Address added to the allowlist
     */
    event AddedToCallhookAllowlist(address newAllowlisted);

    /**
     * @notice Emitted when an address is removed from the callhook allowlist
     * @param notAllowlisted Address removed from the allowlist
     */
    event RemovedFromCallhookAllowlist(address notAllowlisted);

    /**
     * @notice Emitted when the L2 mint allowance per block is updated
     * @param accumulatedL2MintAllowanceSnapshot Accumulated allowance snapshot at update block
     * @param l2MintAllowancePerBlock New allowance per block
     * @param lastL2MintAllowanceUpdateBlock Block number when allowance was updated
     */
    event L2MintAllowanceUpdated(
        uint256 accumulatedL2MintAllowanceSnapshot,
        uint256 l2MintAllowancePerBlock,
        uint256 lastL2MintAllowanceUpdateBlock
    );

    /**
     * @notice Emitted when tokens are minted due to an incoming transfer from L2
     * @param amount Amount of tokens minted
     */
    event TokensMintedFromL2(uint256 amount);

    /**
     * @dev Allows a function to be called only by the gateway's L2 counterpart.
     * The message will actually come from the Arbitrum Bridge, but the Outbox
     * can tell us who the sender from L2 is.
     */
    modifier onlyL2Counterpart() {
        require(inbox != address(0), "INBOX_NOT_SET");
        require(l2Counterpart != address(0), "L2_COUNTERPART_NOT_SET");

        // a message coming from the counterpart gateway was executed by the bridge
        IBridge bridge = IInbox(inbox).bridge();
        require(msg.sender == address(bridge), "NOT_FROM_BRIDGE");

        // and the outbox reports that the L2 address of the sender is the counterpart gateway
        address l2ToL1Sender = IOutbox(bridge.activeOutbox()).l2ToL1Sender();
        require(l2ToL1Sender == l2Counterpart, "ONLY_COUNTERPART_GATEWAY");
        _;
    }

    /**
     * @notice Initialize the L1GraphTokenGateway contract.
     * @dev The contract will be paused.
     * Note some parameters have to be set separately as they are generally
     * not expected to be available at initialization time:
     * - inbox  and l1Router using setArbitrumAddresses
     * - l2GRT using setL2TokenAddress
     * - l2Counterpart using setL2CounterpartAddress
     * - escrow using setEscrowAddress
     * - allowlisted callhook callers using addToCallhookAllowlist
     * - pauseGuardian using setPauseGuardian
     * @param _controller Address of the Controller that manages this contract
     */
    function initialize(address _controller) external onlyImpl initializer {
        Managed._initialize(_controller);
        _paused = true;
    }

    /**
     * @notice Sets the addresses for L1 contracts provided by Arbitrum
     * @param _inbox Address of the Inbox that is part of the Arbitrum Bridge
     * @param _l1Router Address of the Gateway Router
     */
    function setArbitrumAddresses(address _inbox, address _l1Router) external onlyGovernor {
        require(_inbox != address(0), "INVALID_INBOX");
        require(_l1Router != address(0), "INVALID_L1_ROUTER");
        require(!callhookAllowlist[_l1Router], "ROUTER_CANT_BE_ALLOWLISTED");
        require(AddressUpgradeable.isContract(_inbox), "INBOX_MUST_BE_CONTRACT");
        require(AddressUpgradeable.isContract(_l1Router), "ROUTER_MUST_BE_CONTRACT");
        inbox = _inbox;
        l1Router = _l1Router;
        emit ArbitrumAddressesSet(_inbox, _l1Router);
    }

    /**
     * @notice Sets the address of the L2 Graph Token
     * @param _l2GRT Address of the GRT contract on L2
     */
    function setL2TokenAddress(address _l2GRT) external onlyGovernor {
        require(_l2GRT != address(0), "INVALID_L2_GRT");
        l2GRT = _l2GRT;
        emit L2TokenAddressSet(_l2GRT);
    }

    /**
     * @notice Sets the address of the counterpart gateway on L2
     * @param _l2Counterpart Address of the corresponding L2GraphTokenGateway on Arbitrum
     */
    function setL2CounterpartAddress(address _l2Counterpart) external onlyGovernor {
        require(_l2Counterpart != address(0), "INVALID_L2_COUNTERPART");
        l2Counterpart = _l2Counterpart;
        emit L2CounterpartAddressSet(_l2Counterpart);
    }

    /**
     * @notice Sets the address of the escrow contract on L1
     * @param _escrow Address of the BridgeEscrow
     */
    function setEscrowAddress(address _escrow) external onlyGovernor {
        require(_escrow != address(0), "INVALID_ESCROW");
        require(AddressUpgradeable.isContract(_escrow), "MUST_BE_CONTRACT");
        escrow = _escrow;
        emit EscrowAddressSet(_escrow);
    }

    /**
     * @notice Adds an address to the callhook allowlist.
     * This address will be allowed to include callhooks when transferring tokens.
     * @param _newAllowlisted Address to add to the allowlist
     */
    function addToCallhookAllowlist(address _newAllowlisted) external onlyGovernor {
        require(_newAllowlisted != address(0), "INVALID_ADDRESS");
        require(_newAllowlisted != l1Router, "CANT_ALLOW_ROUTER");
        require(AddressUpgradeable.isContract(_newAllowlisted), "MUST_BE_CONTRACT");
        require(!callhookAllowlist[_newAllowlisted], "ALREADY_ALLOWLISTED");
        callhookAllowlist[_newAllowlisted] = true;
        emit AddedToCallhookAllowlist(_newAllowlisted);
    }

    /**
     * @notice Removes an address from the callhook allowlist.
     * This address will no longer be allowed to include callhooks when transferring tokens.
     * @param _notAllowlisted Address to remove from the allowlist
     */
    function removeFromCallhookAllowlist(address _notAllowlisted) external onlyGovernor {
        require(_notAllowlisted != address(0), "INVALID_ADDRESS");
        require(callhookAllowlist[_notAllowlisted], "NOT_ALLOWLISTED");
        callhookAllowlist[_notAllowlisted] = false;
        emit RemovedFromCallhookAllowlist(_notAllowlisted);
    }

    /**
     * @notice Updates the L2 mint allowance per block
     * It is meant to be called _after_ the issuancePerBlock is updated in L2.
     * The caller should provide the new issuance per block and the block at which it was updated,
     * the function will automatically compute the values so that the bridge's mint allowance
     * correctly tracks the maximum rewards minted in L2.
     * @param _l2IssuancePerBlock New issuancePerBlock that has been set in L2
     * @param _updateBlockNum L1 Block number at which issuancePerBlock was updated in L2
     */
    function updateL2MintAllowance(uint256 _l2IssuancePerBlock, uint256 _updateBlockNum) external onlyGovernor {
        require(_updateBlockNum < block.number, "BLOCK_MUST_BE_PAST");
        require(_updateBlockNum > lastL2MintAllowanceUpdateBlock, "BLOCK_MUST_BE_INCREMENTING");
        accumulatedL2MintAllowanceSnapshot = accumulatedL2MintAllowanceAtBlock(_updateBlockNum);
        lastL2MintAllowanceUpdateBlock = _updateBlockNum;
        l2MintAllowancePerBlock = _l2IssuancePerBlock;
        emit L2MintAllowanceUpdated(
            accumulatedL2MintAllowanceSnapshot,
            l2MintAllowancePerBlock,
            lastL2MintAllowanceUpdateBlock
        );
    }

    /**
     * @notice Manually sets the parameters used to compute the L2 mint allowance
     * The use of this function is not recommended, use updateL2MintAllowance instead;
     * this one is only meant to be used as a backup recovery if a previous call to
     * updateL2MintAllowance was done with incorrect values.
     * @param _accumulatedL2MintAllowanceSnapshot Accumulated L2 mint allowance at L1 block _lastL2MintAllowanceUpdateBlock
     * @param _l2MintAllowancePerBlock L2 issuance per block since block number _lastL2MintAllowanceUpdateBlock
     * @param _lastL2MintAllowanceUpdateBlock L1 Block number at which issuancePerBlock was last updated in L2
     */
    function setL2MintAllowanceParametersManual(
        uint256 _accumulatedL2MintAllowanceSnapshot,
        uint256 _l2MintAllowancePerBlock,
        uint256 _lastL2MintAllowanceUpdateBlock
    ) external onlyGovernor {
        require(_lastL2MintAllowanceUpdateBlock < block.number, "BLOCK_MUST_BE_PAST");
        accumulatedL2MintAllowanceSnapshot = _accumulatedL2MintAllowanceSnapshot;
        l2MintAllowancePerBlock = _l2MintAllowancePerBlock;
        lastL2MintAllowanceUpdateBlock = _lastL2MintAllowanceUpdateBlock;
        emit L2MintAllowanceUpdated(
            accumulatedL2MintAllowanceSnapshot,
            l2MintAllowancePerBlock,
            lastL2MintAllowanceUpdateBlock
        );
    }

    /**
     * @inheritdoc ITokenGateway
     * @dev maxGas and gasPriceBid must be set using Arbitrum's NodeInterface.estimateRetryableTicket method.
     * Also note that allowlisted senders (some protocol contracts) can include additional calldata
     * for a callhook to be executed on the L2 side when the tokens are received. In this case, the L2 transaction
     * can revert if the callhook reverts, potentially locking the tokens on the bridge if the callhook
     * never succeeds. This requires extra care when adding contracts to the allowlist, but is necessary to ensure that
     * the tickets can be retried in the case of a temporary failure, and to ensure the atomicity of callhooks
     * with token transfers.
     */
    function outboundTransfer(
        address _l1Token,
        address _to,
        uint256 _amount,
        uint256 _maxGas,
        uint256 _gasPriceBid,
        bytes calldata _data
    ) external payable override notPaused returns (bytes memory) {
        IGraphToken token = graphToken();
        require(_amount != 0, "INVALID_ZERO_AMOUNT");
        require(_l1Token == address(token), "TOKEN_NOT_GRT");
        require(_to != address(0), "INVALID_DESTINATION");

        // nested scopes to avoid stack too deep errors
        address from;
        uint256 seqNum;
        {
            uint256 maxSubmissionCost;
            bytes memory outboundCalldata;
            {
                bytes memory extraData;
                (from, maxSubmissionCost, extraData) = _parseOutboundData(_data);
                require(extraData.length == 0 || callhookAllowlist[msg.sender] == true, "CALL_HOOK_DATA_NOT_ALLOWED");
                require(maxSubmissionCost != 0, "NO_SUBMISSION_COST");
                outboundCalldata = getOutboundCalldata(_l1Token, from, _to, _amount, extraData);
            }
            {
                L2GasParams memory gasParams = L2GasParams(maxSubmissionCost, _maxGas, _gasPriceBid);
                // transfer tokens to escrow
                token.transferFrom(from, escrow, _amount);
                seqNum = sendTxToL2(inbox, l2Counterpart, from, msg.value, 0, gasParams, outboundCalldata);
            }
        }
        emit DepositInitiated(_l1Token, from, _to, seqNum, _amount);

        return abi.encode(seqNum);
    }

    /**
     * @inheritdoc ITokenGateway
     * @dev can only accept transactions coming from the L2 GRT Gateway.
     * The last parameter is unused but kept for compatibility with Arbitrum gateways,
     * and the encoded exitNum is assumed to be 0.
     */
    function finalizeInboundTransfer(
        address _l1Token,
        address _from,
        address _to,
        uint256 _amount,
        bytes calldata // _data, contains exitNum, unused by this contract
    ) external payable override notPaused onlyL2Counterpart {
        IGraphToken token = graphToken();
        require(_l1Token == address(token), "TOKEN_NOT_GRT");

        uint256 escrowBalance = token.balanceOf(escrow);
        if (_amount > escrowBalance) {
            // This will revert if trying to mint more than allowed
            _mintFromL2(_amount.sub(escrowBalance));
        }
        token.transferFrom(escrow, _to, _amount);

        emit WithdrawalFinalized(_l1Token, _from, _to, 0, _amount);
    }

    /**
     * @inheritdoc ITokenGateway
     * @dev In our case, this would only work for GRT.
     */
    function calculateL2TokenAddress(address _l1ERC20) external view override returns (address) {
        IGraphToken token = graphToken();
        if (_l1ERC20 != address(token)) {
            return address(0);
        }
        return l2GRT;
    }

    /**
     * @notice Get the address of the L2GraphTokenGateway
     * @dev This is added for compatibility with the Arbitrum Router's
     * gateway registration process.
     * @return Address of the L2 gateway connected to this gateway
     */
    function counterpartGateway() external view returns (address) {
        return l2Counterpart;
    }

    /**
     * @notice Creates calldata required to create a retryable ticket
     * @dev encodes the target function with its params which
     * will be called on L2 when the retryable ticket is redeemed
     * @param _l1Token Address of the Graph token contract on L1
     * @param _from Address on L1 from which we're transferring tokens
     * @param _to Address on L2 to which we're transferring tokens
     * @param _amount Amount of GRT to transfer
     * @param _data Additional call data for the L2 transaction, which must be empty unless the caller is allowlisted
     * @return Encoded calldata (including function selector) for the L2 transaction
     */
    function getOutboundCalldata(
        address _l1Token,
        address _from,
        address _to,
        uint256 _amount,
        bytes memory _data
    ) public pure returns (bytes memory) {
        return
            abi.encodeWithSelector(
                ITokenGateway.finalizeInboundTransfer.selector,
                _l1Token,
                _from,
                _to,
                _amount,
                _data
            );
    }

    /// @inheritdoc GraphTokenGateway
    // solhint-disable-next-line use-natspec
    function _checksBeforeUnpause() internal view override {
        require(inbox != address(0), "INBOX_NOT_SET");
        require(l1Router != address(0), "ROUTER_NOT_SET");
        require(l2Counterpart != address(0), "L2_COUNTERPART_NOT_SET");
        require(escrow != address(0), "ESCROW_NOT_SET");
    }

    /**
     * @notice Decodes calldata required for transfer of tokens to L2
     * @dev Data must include maxSubmissionCost, extraData can be left empty. When the router
     * sends an outbound message, data also contains the from address.
     * @param _data encoded callhook data
     * @return Sender of the tx
     * @return Base ether value required to keep retryable ticket alive
     * @return Additional data sent to L2
     */
    function _parseOutboundData(bytes calldata _data) private view returns (address, uint256, bytes memory) {
        address from;
        uint256 maxSubmissionCost;
        bytes memory extraData;
        if (msg.sender == l1Router) {
            // Data encoded by the Gateway Router includes the sender address
            (from, extraData) = abi.decode(_data, (address, bytes));
        } else {
            from = msg.sender;
            extraData = _data;
        }
        // User-encoded data contains the max retryable ticket submission cost
        // and additional L2 calldata
        (maxSubmissionCost, extraData) = abi.decode(extraData, (uint256, bytes));
        return (from, maxSubmissionCost, extraData);
    }

    /**
     * @notice Get the accumulated L2 mint allowance at a particular block number
     * @param _blockNum Block at which allowance will be computed
     * @return The accumulated GRT amount that can be minted from L2 at the specified block
     */
    function accumulatedL2MintAllowanceAtBlock(uint256 _blockNum) public view returns (uint256) {
        require(_blockNum >= lastL2MintAllowanceUpdateBlock, "INVALID_BLOCK_FOR_MINT_ALLOWANCE");
        return
            accumulatedL2MintAllowanceSnapshot.add(
                l2MintAllowancePerBlock.mul(_blockNum.sub(lastL2MintAllowanceUpdateBlock))
            );
    }

    /**
     * @notice Mint new L1 tokens coming  from L2
     * This will check if the amount to mint is within the L2's mint allowance, and revert otherwise.
     * The tokens will be sent to the bridge escrow (from where they will then be sent to the destinatary
     * of the current inbound transfer).
     * @param _amount Number of tokens to mint
     */
    function _mintFromL2(uint256 _amount) internal {
        // If we're trying to mint more than allowed, something's gone terribly wrong
        // (either the L2 issuance is wrong, or the Arbitrum bridge has been compromised)
        require(_l2MintAmountAllowed(_amount), "INVALID_L2_MINT_AMOUNT");
        totalMintedFromL2 = totalMintedFromL2.add(_amount);
        graphToken().mint(escrow, _amount);
        emit TokensMintedFromL2(_amount);
    }

    /**
     * @notice Check if minting a certain amount of tokens from L2 is within allowance
     * @param _amount Number of tokens that would be minted
     * @return true if minting those tokens is allowed, or false if it would be over allowance
     */
    function _l2MintAmountAllowed(uint256 _amount) internal view returns (bool) {
        return (totalMintedFromL2.add(_amount) <= accumulatedL2MintAllowanceAtBlock(block.number));
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity 0.8.27 || 0.8.33;

import { GraphDirectory } from "../../utilities/GraphDirectory.sol";

/* solhint-disable var-name-mixedcase */

/**
 * @title Graph Managed contract
 * @author Edge & Node
 * @notice The Managed contract provides an interface to interact with the Controller
 * @dev For Graph Horizon this contract is mostly a shell that uses {GraphDirectory}, however since the {HorizonStaking}
 * contract uses it we need to preserve the storage layout.
 * Inspired by Livepeer: https://github.com/livepeer/protocol/blob/streamflow/contracts/Controller.sol
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
abstract contract Managed is GraphDirectory {
    // -- State --

    // forge-lint: disable-next-item(mixed-case-variable)
    /// @notice Controller that manages this contract
    address private __DEPRECATED_controller;

    // forge-lint: disable-next-item(mixed-case-variable)
    /// @dev Cache for the addresses of the contracts retrieved from the controller
    mapping(bytes32 contractName => address contractAddress) private __DEPRECATED_addressCache;

    // forge-lint: disable-next-item(mixed-case-variable)
    /// @dev Gap for future storage variables
    uint256[10] private __gap;

    /**
     * @notice Thrown when a protected function is called and the contract is paused.
     */
    error ManagedIsPaused();

    /**
     * @notice Thrown when a the caller is not the expected controller address.
     */
    error ManagedOnlyController();

    /**
     * @notice Thrown when a the caller is not the governor.
     */
    error ManagedOnlyGovernor();

    // forge-lint: disable-next-item(unwrapped-modifier-logic)
    /**
     * @dev Revert if the controller is paused
     */
    modifier notPaused() {
        require(!_graphController().paused(), ManagedIsPaused());
        _;
    }

    // forge-lint: disable-next-item(unwrapped-modifier-logic)
    /**
     * @dev Revert if the caller is not the governor
     */
    modifier onlyGovernor() {
        require(msg.sender == _graphController().getGovernor(), ManagedOnlyGovernor());
        _;
    }

    /**
     * @notice Initialize the contract
     * @param controller_ The address of the Graph controller contract
     */
    constructor(address controller_) GraphDirectory(controller_) {}
}

// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0 <0.8.0;

/**
 * @dev Wrappers over Solidity's arithmetic operations with added overflow
 * checks.
 *
 * Arithmetic operations in Solidity wrap on overflow. This can easily result
 * in bugs, because programmers usually assume that an overflow raises an
 * error, which is the standard behavior in high level programming languages.
 * `SafeMath` restores this intuition by reverting the transaction when an
 * operation overflows.
 *
 * Using this library instead of the unchecked operations eliminates an entire
 * class of bugs, so it's recommended to use it always.
 */
library SafeMathUpgradeable {
    /**
     * @dev Returns the addition of two unsigned integers, with an overflow flag.
     *
     * _Available since v3.4._
     */
    function tryAdd(uint256 a, uint256 b) internal pure returns (bool, uint256) {
        uint256 c = a + b;
        if (c < a) return (false, 0);
        return (true, c);
    }

    /**
     * @dev Returns the substraction of two unsigned integers, with an overflow flag.
     *
     * _Available since v3.4._
     */
    function trySub(uint256 a, uint256 b) internal pure returns (bool, uint256) {
        if (b > a) return (false, 0);
        return (true, a - b);
    }

    /**
     * @dev Returns the multiplication of two unsigned integers, with an overflow flag.
     *
     * _Available since v3.4._
     */
    function tryMul(uint256 a, uint256 b) internal pure returns (bool, uint256) {
        // Gas optimization: this is cheaper than requiring 'a' not being zero, but the
        // benefit is lost if 'b' is also tested.
        // See: https://github.com/OpenZeppelin/openzeppelin-contracts/pull/522
        if (a == 0) return (true, 0);
        uint256 c = a * b;
        if (c / a != b) return (false, 0);
        return (true, c);
    }

    /**
     * @dev Returns the division of two unsigned integers, with a division by zero flag.
     *
     * _Available since v3.4._
     */
    function tryDiv(uint256 a, uint256 b) internal pure returns (bool, uint256) {
        if (b == 0) return (false, 0);
        return (true, a / b);
    }

    /**
     * @dev Returns the remainder of dividing two unsigned integers, with a division by zero flag.
     *
     * _Available since v3.4._
     */
    function tryMod(uint256 a, uint256 b) internal pure returns (bool, uint256) {
        if (b == 0) return (false, 0);
        return (true, a % b);
    }

    /**
     * @dev Returns the addition of two unsigned integers, reverting on
     * overflow.
     *
     * Counterpart to Solidity's `+` operator.
     *
     * Requirements:
     *
     * - Addition cannot overflow.
     */
    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        uint256 c = a + b;
        require(c >= a, "SafeMath: addition overflow");
        return c;
    }

    /**
     * @dev Returns the subtraction of two unsigned integers, reverting on
     * overflow (when the result is negative).
     *
     * Counterpart to Solidity's `-` operator.
     *
     * Requirements:
     *
     * - Subtraction cannot overflow.
     */
    function sub(uint256 a, uint256 b) internal pure returns (uint256) {
        require(b <= a, "SafeMath: subtraction overflow");
        return a - b;
    }

    /**
     * @dev Returns the multiplication of two unsigned integers, reverting on
     * overflow.
     *
     * Counterpart to Solidity's `*` operator.
     *
     * Requirements:
     *
     * - Multiplication cannot overflow.
     */
    function mul(uint256 a, uint256 b) internal pure returns (uint256) {
        if (a == 0) return 0;
        uint256 c = a * b;
        require(c / a == b, "SafeMath: multiplication overflow");
        return c;
    }

    /**
     * @dev Returns the integer division of two unsigned integers, reverting on
     * division by zero. The result is rounded towards zero.
     *
     * Counterpart to Solidity's `/` operator. Note: this function uses a
     * `revert` opcode (which leaves remaining gas untouched) while Solidity
     * uses an invalid opcode to revert (consuming all remaining gas).
     *
     * Requirements:
     *
     * - The divisor cannot be zero.
     */
    function div(uint256 a, uint256 b) internal pure returns (uint256) {
        require(b > 0, "SafeMath: division by zero");
        return a / b;
    }

    /**
     * @dev Returns the remainder of dividing two unsigned integers. (unsigned integer modulo),
     * reverting when dividing by zero.
     *
     * Counterpart to Solidity's `%` operator. This function uses a `revert`
     * opcode (which leaves remaining gas untouched) while Solidity uses an
     * invalid opcode to revert (consuming all remaining gas).
     *
     * Requirements:
     *
     * - The divisor cannot be zero.
     */
    function mod(uint256 a, uint256 b) internal pure returns (uint256) {
        require(b > 0, "SafeMath: modulo by zero");
        return a % b;
    }

    /**
     * @dev Returns the subtraction of two unsigned integers, reverting with custom message on
     * overflow (when the result is negative).
     *
     * CAUTION: This function is deprecated because it requires allocating memory for the error
     * message unnecessarily. For custom revert reasons use {trySub}.
     *
     * Counterpart to Solidity's `-` operator.
     *
     * Requirements:
     *
     * - Subtraction cannot overflow.
     */
    function sub(uint256 a, uint256 b, string memory errorMessage) internal pure returns (uint256) {
        require(b <= a, errorMessage);
        return a - b;
    }

    /**
     * @dev Returns the integer division of two unsigned integers, reverting with custom message on
     * division by zero. The result is rounded towards zero.
     *
     * CAUTION: This function is deprecated because it requires allocating memory for the error
     * message unnecessarily. For custom revert reasons use {tryDiv}.
     *
     * Counterpart to Solidity's `/` operator. Note: this function uses a
     * `revert` opcode (which leaves remaining gas untouched) while Solidity
     * uses an invalid opcode to revert (consuming all remaining gas).
     *
     * Requirements:
     *
     * - The divisor cannot be zero.
     */
    function div(uint256 a, uint256 b, string memory errorMessage) internal pure returns (uint256) {
        require(b > 0, errorMessage);
        return a / b;
    }

    /**
     * @dev Returns the remainder of dividing two unsigned integers. (unsigned integer modulo),
     * reverting with custom message when dividing by zero.
     *
     * CAUTION: This function is deprecated because it requires allocating memory for the error
     * message unnecessarily. For custom revert reasons use {tryMod}.
     *
     * Counterpart to Solidity's `%` operator. This function uses a `revert`
     * opcode (which leaves remaining gas untouched) while Solidity uses an
     * invalid opcode to revert (consuming all remaining gas).
     *
     * Requirements:
     *
     * - The divisor cannot be zero.
     */
    function mod(uint256 a, uint256 b, string memory errorMessage) internal pure returns (uint256) {
        require(b > 0, errorMessage);
        return a % b;
    }
}

// SPDX-License-Identifier: Apache-2.0

/*
 * Copyright 2020, Offchain Labs, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * Originally copied from:
 * https://github.com/OffchainLabs/arbitrum/tree/e3a6307ad8a2dc2cad35728a2a9908cfd8dd8ef9/packages/arb-bridge-peripherals
 *
 * MODIFIED from Offchain Labs' implementation:
 * - Changed solidity version to 0.7.6 (pablo@edgeandnode.com)
 *
 */

pragma solidity ^0.7.6;

import { IInbox } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/IInbox.sol";
import { IOutbox } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/IOutbox.sol";
import { IBridge } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/IBridge.sol";

/**
 * @title L1 Arbitrum Messenger
 * @author Edge & Node
 * @notice L1 utility contract to assist with L1 <=> L2 interactions
 * @dev this is an abstract contract instead of library so the functions can be easily overridden when testing
 */
abstract contract L1ArbitrumMessenger {
    /**
     * @notice Emitted when a transaction is sent to L2
     * @param _from Address sending the transaction
     * @param _to Address receiving the transaction on L2
     * @param _seqNum Sequence number of the retryable ticket
     * @param _data Transaction data
     */
    event TxToL2(address indexed _from, address indexed _to, uint256 indexed _seqNum, bytes _data);

    /**
     * @dev Parameters for L2 gas configuration
     * @param _maxSubmissionCost Maximum cost for submitting the transaction
     * @param _maxGas Maximum gas for the L2 transaction
     * @param _gasPriceBid Gas price bid for the L2 transaction
     */
    struct L2GasParams {
        uint256 _maxSubmissionCost;
        uint256 _maxGas;
        uint256 _gasPriceBid;
    }

    /**
     * @notice Send a transaction to L2 using gas parameters struct
     * @param _inbox Address of the inbox contract
     * @param _to Destination address on L2
     * @param _user Address that will be credited as the sender
     * @param _l1CallValue ETH value to send with the L1 transaction
     * @param _l2CallValue ETH value to send with the L2 transaction
     * @param _l2GasParams Gas parameters for the L2 transaction
     * @param _data Calldata for the L2 transaction
     * @return Sequence number of the retryable ticket
     */
    function sendTxToL2(
        address _inbox,
        address _to,
        address _user,
        uint256 _l1CallValue,
        uint256 _l2CallValue,
        L2GasParams memory _l2GasParams,
        bytes memory _data
    ) internal virtual returns (uint256) {
        // alternative function entry point when struggling with the stack size
        return
            sendTxToL2(
                _inbox,
                _to,
                _user,
                _l1CallValue,
                _l2CallValue,
                _l2GasParams._maxSubmissionCost,
                _l2GasParams._maxGas,
                _l2GasParams._gasPriceBid,
                _data
            );
    }

    /**
     * @notice Send a transaction to L2 with individual gas parameters
     * @param _inbox Address of the inbox contract
     * @param _to Destination address on L2
     * @param _user Address that will be credited as the sender
     * @param _l1CallValue ETH value to send with the L1 transaction
     * @param _l2CallValue ETH value to send with the L2 transaction
     * @param _maxSubmissionCost Maximum cost for submitting the transaction
     * @param _maxGas Maximum gas for the L2 transaction
     * @param _gasPriceBid Gas price bid for the L2 transaction
     * @param _data Calldata for the L2 transaction
     * @return Sequence number of the retryable ticket
     */
    function sendTxToL2(
        address _inbox,
        address _to,
        address _user,
        uint256 _l1CallValue,
        uint256 _l2CallValue,
        uint256 _maxSubmissionCost,
        uint256 _maxGas,
        uint256 _gasPriceBid,
        bytes memory _data
    ) internal virtual returns (uint256) {
        uint256 seqNum = IInbox(_inbox).createRetryableTicket{ value: _l1CallValue }(
            _to,
            _l2CallValue,
            _maxSubmissionCost,
            _user,
            _user,
            _maxGas,
            _gasPriceBid,
            _data
        );
        emit TxToL2(_user, _to, seqNum, _data);
        return seqNum;
    }

    /**
     * @notice Get the bridge contract from an inbox
     * @param _inbox Address of the inbox contract
     * @return Bridge contract interface
     */
    function getBridge(address _inbox) internal view virtual returns (IBridge) {
        return IInbox(_inbox).bridge();
    }

    /**
     * @notice Get the L2 to L1 sender address from the outbox
     * @dev the l2ToL1Sender behaves as the tx.origin, the msg.sender should be validated to protect against reentrancies
     * @param _inbox Address of the inbox contract
     * @return Address of the L2 to L1 sender
     */
    function getL2ToL1Sender(address _inbox) internal view virtual returns (address) {
        IOutbox outbox = IOutbox(getBridge(_inbox).activeOutbox());
        address l2ToL1Sender = outbox.l2ToL1Sender();

        require(l2ToL1Sender != address(0), "NO_SENDER");
        return l2ToL1Sender;
    }
}

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

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;

import { GraphUpgradeable } from "../upgrades/GraphUpgradeable.sol";
import { ITokenGateway } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/ITokenGateway.sol";
import { Pausable } from "../governance/Pausable.sol";
import { Managed } from "../governance/Managed.sol";

/**
 * @title L1/L2 Graph Token Gateway
 * @author Edge & Node
 * @notice This includes everything that's shared between the L1 and L2 sides of the bridge.
 */
abstract contract GraphTokenGateway is GraphUpgradeable, Pausable, Managed, ITokenGateway {
    /// @dev Storage gap added in case we need to add state variables to this contract
    uint256[50] private __gap;

    /**
     * @dev Check if the caller is the Controller's governor or this contract's pause guardian.
     */
    modifier onlyGovernorOrGuardian() {
        require(msg.sender == controller.getGovernor() || msg.sender == pauseGuardian, "Only Governor or Guardian");
        _;
    }

    /**
     * @notice Change the Pause Guardian for this contract
     * @param _newPauseGuardian The address of the new Pause Guardian
     */
    function setPauseGuardian(address _newPauseGuardian) external onlyGovernor {
        require(_newPauseGuardian != address(0), "PauseGuardian must be set");
        _setPauseGuardian(_newPauseGuardian);
    }

    /**
     * @notice Change the paused state of the contract
     * @param _newPaused New value for the pause state (true means the transfers will be paused)
     */
    function setPaused(bool _newPaused) external onlyGovernorOrGuardian {
        if (!_newPaused) {
            _checksBeforeUnpause();
        }
        _setPaused(_newPaused);
    }

    /**
     * @notice Getter to access paused state of this contract
     * @return True if the contract is paused, false otherwise
     */
    function paused() external view returns (bool) {
        return _paused;
    }

    /**
     * @notice Override the default pausing from Managed to allow pausing this
     * particular contract instead of pausing from the Controller.
     */
    function _notPaused() internal view override {
        require(!_paused, "Paused (contract)");
    }

    /**
     * @notice Runs state validation before unpausing, reverts if
     * something is not set properly
     */
    function _checksBeforeUnpause() internal view virtual;
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

/* solhint-disable gas-custom-errors */ // Cannot use custom errors with 0.7.6

import { IGraphProxy } from "@graphprotocol/interfaces/contracts/contracts/upgrades/IGraphProxy.sol";

/**
 * @title Graph Upgradeable
 * @author Edge & Node
 * @notice This contract is intended to be inherited from upgradeable contracts.
 */
abstract contract GraphUpgradeable {
    /**
     * @dev Storage slot with the address of the current implementation.
     * This is the keccak-256 hash of "eip1967.proxy.implementation" subtracted by 1, and is
     * validated in the constructor.
     */
    bytes32 internal constant IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

    /**
     * @dev Check if the caller is the proxy admin.
     * @param _proxy The proxy contract to check admin for
     */
    modifier onlyProxyAdmin(IGraphProxy _proxy) {
        require(msg.sender == _proxy.admin(), "Caller must be the proxy admin");
        _;
    }

    /**
     * @dev Check if the caller is the implementation.
     */
    modifier onlyImpl() {
        require(msg.sender == _implementation(), "Only implementation");
        _;
    }

    /**
     * @notice Returns the current implementation.
     * @return impl Address of the current implementation
     */
    function _implementation() internal view returns (address impl) {
        bytes32 slot = IMPLEMENTATION_SLOT;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            impl := sload(slot)
        }
    }

    /**
     * @notice Accept to be an implementation of proxy.
     * @param _proxy Proxy to accept
     */
    function acceptProxy(IGraphProxy _proxy) external onlyProxyAdmin(_proxy) {
        _proxy.acceptUpgrade();
    }

    /**
     * @notice Accept to be an implementation of proxy and then call a function from the new
     * implementation as specified by `_data`, which should be an encoded function call. This is
     * useful to initialize new storage variables in the proxied contract.
     * @param _proxy Proxy to accept
     * @param _data Calldata for the initialization function call (including selector)
     */
    function acceptProxyAndCall(IGraphProxy _proxy, bytes calldata _data) external onlyProxyAdmin(_proxy) {
        _proxy.acceptUpgradeAndCall(_data);
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-indexed-events

/**
 * @title Pausable Contract
 * @author Edge & Node
 * @notice Abstract contract that provides pause functionality for protocol operations
 */
abstract contract Pausable {
    /**
     * @dev "Partial paused" pauses exit and enter functions for GRT, but not internal
     * functions, such as allocating
     */
    bool internal _partialPaused;
    /**
     * @dev Paused will pause all major protocol functions
     */
    bool internal _paused;

    /// @notice Timestamp for the last time the partial pause was set
    uint256 public lastPartialPauseTime;
    /// @notice Timestamp for the last time the full pause was set
    uint256 public lastPauseTime;

    /// @notice Pause guardian is a separate entity from the governor that can
    /// pause and unpause the protocol, fully or partially
    address public pauseGuardian;

    /**
     * @notice Emitted when the partial pause state changed
     * @param isPaused Whether the contract is partially paused
     */
    event PartialPauseChanged(bool isPaused);

    /**
     * @notice Emitted when the full pause state changed
     * @param isPaused Whether the contract is fully paused
     */
    event PauseChanged(bool isPaused);

    /**
     * @notice Emitted when the pause guardian is changed
     * @param oldPauseGuardian Address of the previous pause guardian
     * @param pauseGuardian Address of the new pause guardian
     */
    event NewPauseGuardian(address indexed oldPauseGuardian, address indexed pauseGuardian);

    /**
     * @notice Change the partial paused state of the contract
     * @param _toPartialPause New value for the partial pause state (true means the contracts will be partially paused)
     */
    function _setPartialPaused(bool _toPartialPause) internal {
        if (_toPartialPause == _partialPaused) {
            return;
        }
        _partialPaused = _toPartialPause;
        if (_partialPaused) {
            lastPartialPauseTime = block.timestamp;
        }
        emit PartialPauseChanged(_partialPaused);
    }

    /**
     * @notice Change the paused state of the contract
     * @param _toPause New value for the pause state (true means the contracts will be paused)
     */
    function _setPaused(bool _toPause) internal {
        if (_toPause == _paused) {
            return;
        }
        _paused = _toPause;
        if (_paused) {
            lastPauseTime = block.timestamp;
        }
        emit PauseChanged(_paused);
    }

    /**
     * @notice Change the Pause Guardian
     * @param newPauseGuardian The address of the new Pause Guardian
     */
    function _setPauseGuardian(address newPauseGuardian) internal {
        address oldPauseGuardian = pauseGuardian;
        pauseGuardian = newPauseGuardian;
        emit NewPauseGuardian(oldPauseGuardian, newPauseGuardian);
    }
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

