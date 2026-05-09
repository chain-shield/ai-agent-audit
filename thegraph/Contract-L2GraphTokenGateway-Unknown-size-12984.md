
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;
pragma abicoder v2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-indexed-events

import { ReentrancyGuardUpgradeable } from "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";
import { SafeMathUpgradeable } from "@openzeppelin/contracts-upgradeable/math/SafeMathUpgradeable.sol";

import { L2ArbitrumMessenger } from "../../arbitrum/L2ArbitrumMessenger.sol";
import { AddressAliasHelper } from "../../arbitrum/AddressAliasHelper.sol";
import { ITokenGateway } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/ITokenGateway.sol";
import { Managed } from "../../governance/Managed.sol";
import { GraphTokenGateway } from "../../gateway/GraphTokenGateway.sol";
import { ICallhookReceiver } from "@graphprotocol/interfaces/contracts/contracts/gateway/ICallhookReceiver.sol";
import { L2GraphToken } from "../token/L2GraphToken.sol";

/**
 * @title L2 Graph Token Gateway Contract
 * @author Edge & Node
 * @notice Provides the L2 side of the Ethereum-Arbitrum GRT bridge. Receives GRT from the L1 chain
 * and mints them on the L2 side. Sends GRT back to L1 by burning them on the L2 side.
 * Based on Offchain Labs' reference implementation and Livepeer's arbitrum-lpt-bridge
 * (See: https://github.com/OffchainLabs/arbitrum/tree/master/packages/arb-bridge-peripherals/contracts/tokenbridge
 * and https://github.com/livepeer/arbitrum-lpt-bridge)
 */
contract L2GraphTokenGateway is GraphTokenGateway, L2ArbitrumMessenger, ReentrancyGuardUpgradeable {
    using SafeMathUpgradeable for uint256;

    /// @notice Address of the Graph Token contract on L1
    address public l1GRT;
    /// @notice Address of the L1GraphTokenGateway that is the counterpart of this gateway on L1
    address public l1Counterpart;
    /// @notice Address of the Arbitrum Gateway Router on L2
    address public l2Router;

    /// @dev Calldata included in an outbound transfer, stored as a structure for convenience and stack depth
    /**
     * @dev Struct for outbound transfer calldata
     * @param from Address sending the tokens
     * @param extraData Additional data for the transfer
     */
    struct OutboundCalldata {
        address from;
        bytes extraData;
    }

    /**
     * @notice Emitted when an incoming transfer is finalized, i.e. tokens were deposited from L1 to L2
     * @param l1Token Address of the L1 token
     * @param from Address sending the tokens on L1
     * @param to Address receiving the tokens on L2
     * @param amount Amount of tokens transferred
     */
    event DepositFinalized(address indexed l1Token, address indexed from, address indexed to, uint256 amount);

    /**
     * @notice Emitted when an outbound transfer is initiated, i.e. tokens are being withdrawn from L2 back to L1
     * @param l1Token Address of the L1 token
     * @param from Address sending the tokens on L2
     * @param to Address receiving the tokens on L1
     * @param l2ToL1Id ID of the L2 to L1 message
     * @param exitNum Exit number (always 0 for this contract)
     * @param amount Amount of tokens transferred
     */
    event WithdrawalInitiated(
        address l1Token,
        address indexed from,
        address indexed to,
        uint256 indexed l2ToL1Id,
        uint256 exitNum,
        uint256 amount
    );

    /**
     * @notice Emitted when the Arbitrum Gateway Router address on L2 has been updated
     * @param l2Router Address of the L2 Gateway Router
     */
    event L2RouterSet(address l2Router);

    /**
     * @notice Emitted when the L1 Graph Token address has been updated
     * @param l1GRT Address of the L1 GRT contract
     */
    event L1TokenAddressSet(address l1GRT);

    /**
     * @notice Emitted when the address of the counterpart gateway on L1 has been updated
     * @param l1Counterpart Address of the L1 counterpart gateway
     */
    event L1CounterpartAddressSet(address l1Counterpart);

    /**
     * @dev Checks that the sender is the L2 alias of the counterpart
     * gateway on L1.
     */
    modifier onlyL1Counterpart() {
        require(msg.sender == AddressAliasHelper.applyL1ToL2Alias(l1Counterpart), "ONLY_COUNTERPART_GATEWAY");
        _;
    }

    /**
     * @notice Initialize the L2GraphTokenGateway contract.
     * @dev The contract will be paused.
     * Note some parameters have to be set separately as they are generally
     * not expected to be available at initialization time:
     * - l2Router using setL2Router
     * - l1GRT using setL1TokenAddress
     * - l1Counterpart using setL1CounterpartAddress
     * - pauseGuardian using setPauseGuardian
     * @param _controller Address of the Controller that manages this contract
     */
    function initialize(address _controller) external onlyImpl initializer {
        Managed._initialize(_controller);
        _paused = true;
        __ReentrancyGuard_init();
    }

    /**
     * @notice Sets the address of the Arbitrum Gateway Router on L2
     * @param _l2Router Address of the L2 Router (provided by Arbitrum)
     */
    function setL2Router(address _l2Router) external onlyGovernor {
        require(_l2Router != address(0), "INVALID_L2_ROUTER");
        l2Router = _l2Router;
        emit L2RouterSet(_l2Router);
    }

    /**
     * @notice Sets the address of the Graph Token on L1
     * @param _l1GRT L1 address of the Graph Token contract
     */
    function setL1TokenAddress(address _l1GRT) external onlyGovernor {
        require(_l1GRT != address(0), "INVALID_L1_GRT");
        l1GRT = _l1GRT;
        emit L1TokenAddressSet(_l1GRT);
    }

    /**
     * @notice Sets the address of the counterpart gateway on L1
     * @param _l1Counterpart Address of the L1GraphTokenGateway on L1
     */
    function setL1CounterpartAddress(address _l1Counterpart) external onlyGovernor {
        require(_l1Counterpart != address(0), "INVALID_L1_COUNTERPART");
        l1Counterpart = _l1Counterpart;
        emit L1CounterpartAddressSet(_l1Counterpart);
    }

    /**
     * @notice Burns L2 tokens and initiates a transfer to L1.
     * The tokens will be received on L1 only after the wait period (7 days) is over,
     * and will require an Outbox.executeTransaction to finalize.
     * @dev no additional callhook data is allowed
     * @param _l1Token L1 Address of GRT (needed for compatibility with Arbitrum Gateway Router)
     * @param _to Recipient address on L1
     * @param _amount Amount of tokens to burn
     * @param _data Contains sender and additional data to send to L1
     * @return ID of the withdraw tx
     */
    function outboundTransfer(
        address _l1Token,
        address _to,
        uint256 _amount,
        bytes calldata _data
    ) external returns (bytes memory) {
        return outboundTransfer(_l1Token, _to, _amount, 0, 0, _data);
    }

    /**
     * @inheritdoc ITokenGateway
     * @dev Only accepts transactions from the L1 GRT Gateway.
     * The function is payable for ITokenGateway compatibility, but msg.value must be zero.
     * Note that allowlisted senders (some protocol contracts) can include additional calldata
     * for a callhook to be executed on the L2 side when the tokens are received. In this case, the L2 transaction
     * can revert if the callhook reverts, potentially locking the tokens on the bridge if the callhook
     * never succeeds. This requires extra care when adding contracts to the allowlist, but is necessary to ensure that
     * the tickets can be retried in the case of a temporary failure, and to ensure the atomicity of callhooks
     * with token transfers.
     */
    function finalizeInboundTransfer(
        address _l1Token,
        address _from,
        address _to,
        uint256 _amount,
        bytes calldata _data
    ) external payable override nonReentrant notPaused onlyL1Counterpart {
        require(_l1Token == l1GRT, "TOKEN_NOT_GRT");
        require(msg.value == 0, "INVALID_NONZERO_VALUE");

        L2GraphToken(calculateL2TokenAddress(l1GRT)).bridgeMint(_to, _amount);

        if (_data.length > 0) {
            ICallhookReceiver(_to).onTokenTransfer(_from, _amount, _data);
        }

        emit DepositFinalized(_l1Token, _from, _to, _amount);
    }

    /**
     * @inheritdoc ITokenGateway
     * @dev Burns L2 tokens and initiates a transfer to L1.
     * The tokens will be available on L1 only after the wait period (7 days) is over,
     * and will require an Outbox.executeTransaction to finalize.
     * Note that the caller must previously allow the gateway to spend the specified amount of GRT.
     * No additional callhook data is allowed. The two unused params are needed
     * for compatibility with Arbitrum's gateway router.
     * The function is payable for ITokenGateway compatibility, but msg.value must be zero.
     */
    function outboundTransfer(
        address _l1Token,
        address _to,
        uint256 _amount,
        uint256, // unused on L2
        uint256, // unused on L2
        bytes calldata _data
    ) public payable override nonReentrant notPaused returns (bytes memory) {
        require(_l1Token == l1GRT, "TOKEN_NOT_GRT");
        require(_amount != 0, "INVALID_ZERO_AMOUNT");
        require(msg.value == 0, "INVALID_NONZERO_VALUE");
        require(_to != address(0), "INVALID_DESTINATION");

        OutboundCalldata memory outboundCalldata;

        (outboundCalldata.from, outboundCalldata.extraData) = _parseOutboundData(_data);
        require(outboundCalldata.extraData.length == 0, "CALL_HOOK_DATA_NOT_ALLOWED");

        // from needs to approve this contract to burn the amount first
        L2GraphToken(calculateL2TokenAddress(l1GRT)).bridgeBurn(outboundCalldata.from, _amount);

        uint256 id = sendTxToL1(
            0,
            outboundCalldata.from,
            l1Counterpart,
            getOutboundCalldata(_l1Token, outboundCalldata.from, _to, _amount, outboundCalldata.extraData)
        );

        // we don't need to track exitNums (b/c we have no fast exits) so we always use 0
        emit WithdrawalInitiated(_l1Token, outboundCalldata.from, _to, id, 0, _amount);

        return abi.encode(id);
    }

    /**
     * @inheritdoc ITokenGateway
     * @dev In our case, this would only work for GRT.
     */
    function calculateL2TokenAddress(address l1ERC20) public view override returns (address) {
        if (l1ERC20 != l1GRT) {
            return address(0);
        }
        return address(graphToken());
    }

    /**
     * @notice Creates calldata required to send tx to L1
     * @dev encodes the target function with its params which
     * will be called on L1 when the message is received on L1
     * @param _token Address of the token on L1
     * @param _from Address of the token sender on L2
     * @param _to Address to which we're sending tokens on L1
     * @param _amount Amount of GRT to transfer
     * @param _data Additional calldata for the transaction
     * @return Calldata for a transaction sent to L1
     */
    function getOutboundCalldata(
        address _token,
        address _from,
        address _to,
        uint256 _amount,
        bytes memory _data
    ) public pure returns (bytes memory) {
        return
            abi.encodeWithSelector(
                ITokenGateway.finalizeInboundTransfer.selector,
                _token,
                _from,
                _to,
                _amount,
                abi.encode(0, _data) // we don't need to track exitNums (b/c we have no fast exits) so we always use 0
            );
    }

    /// @inheritdoc GraphTokenGateway
    // solhint-disable-next-line use-natspec
    function _checksBeforeUnpause() internal view override {
        require(l2Router != address(0), "L2_ROUTER_NOT_SET");
        require(l1Counterpart != address(0), "L1_COUNTERPART_NOT_SET");
        require(l1GRT != address(0), "L1_GRT_NOT_SET");
    }

    /**
     * @notice Decodes calldata required for transfer of tokens to L1
     * @dev extraData can be left empty
     * @param _data Encoded callhook data
     * @return Sender of the tx
     * @return Any other data sent to L1
     */
    function _parseOutboundData(bytes calldata _data) private view returns (address, bytes memory) {
        address from;
        bytes memory extraData;
        if (msg.sender == l2Router) {
            (from, extraData) = abi.decode(_data, (address, bytes));
        } else {
            from = msg.sender;
            extraData = _data;
        }
        return (from, extraData);
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec

/**
 * @title ArbSys Mock Contract
 * @dev This is a mock implementation of the ArbSys precompiled contract used in Arbitrum
 * It's used for testing the L2GraphTokenGateway contract
 */
contract ArbSysMock {
    /**
     * @dev Emitted when a transaction is sent from L2 to L1
     * @param from Address sending the transaction on L2
     * @param to Address receiving the transaction on L1
     * @param id Unique identifier for the L2-to-L1 transaction
     * @param data Transaction data
     */
    event L2ToL1Tx(address indexed from, address indexed to, uint256 indexed id, bytes data);

    /**
     * @notice Send a transaction to L1
     * @param destination The address on L1 to send the transaction to
     * @param calldataForL1 The calldata for the transaction
     * @return A unique identifier for this L2-to-L1 transaction
     */
    function sendTxToL1(address destination, bytes calldata calldataForL1) external returns (uint256) {
        uint256 id = 1; // Always return 1 for testing
        emit L2ToL1Tx(msg.sender, destination, id, calldataForL1);
        return id;
    }
}

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

import { ArbSys } from "arbos-precompiles/arbos/builtin/ArbSys.sol";

/**
 * @title L2 Arbitrum Messenger
 * @author Edge & Node
 * @notice L2 utility contract to assist with L1 <=> L2 interactions
 * @dev this is an abstract contract instead of library so the functions can be easily overridden when testing
 */
abstract contract L2ArbitrumMessenger {
    /// @dev Address of the ArbSys precompile
    address internal constant ARB_SYS_ADDRESS = address(100);

    /**
     * @notice Emitted when a transaction is sent to L1
     * @param _from Address sending the transaction
     * @param _to Address receiving the transaction on L1
     * @param _id ID of the L2 to L1 message
     * @param _data Transaction data
     */
    event TxToL1(address indexed _from, address indexed _to, uint256 indexed _id, bytes _data);

    /**
     * @notice Send a transaction from L2 to L1
     * @param _l1CallValue ETH value to send with the L1 transaction
     * @param _from Address that is sending the transaction
     * @param _to Destination address on L1
     * @param _data Calldata for the L1 transaction
     * @return ID of the L2 to L1 message
     */
    function sendTxToL1(
        uint256 _l1CallValue,
        address _from,
        address _to,
        bytes memory _data
    ) internal virtual returns (uint256) {
        uint256 _id = ArbSys(ARB_SYS_ADDRESS).sendTxToL1{ value: _l1CallValue }(_to, _data);
        emit TxToL1(_from, _to, _id, _data);
        return _id;
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-indexed-events

import { GraphTokenUpgradeable } from "./GraphTokenUpgradeable.sol";
import { IArbToken } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/IArbToken.sol";

/**
 * @title L2 Graph Token Contract
 * @author Edge & Node
 * @notice Provides the L2 version of the GRT token, meant to be minted/burned
 * through the L2GraphTokenGateway.
 */
contract L2GraphToken is GraphTokenUpgradeable, IArbToken {
    /// @notice Address of the gateway (on L2) that is allowed to mint tokens
    address public gateway;
    /// @notice Address of the corresponding Graph Token contract on L1
    address public override l1Address;

    /**
     * @notice Emitted when the bridge / gateway has minted new tokens, i.e. tokens were transferred to L2
     * @param account Address that received the minted tokens
     * @param amount Amount of tokens minted
     */
    event BridgeMinted(address indexed account, uint256 amount);

    /**
     * @notice Emitted when the bridge / gateway has burned tokens, i.e. tokens were transferred back to L1
     * @param account Address from which tokens were burned
     * @param amount Amount of tokens burned
     */
    event BridgeBurned(address indexed account, uint256 amount);

    /**
     * @notice Emitted when the address of the gateway has been updated
     * @param gateway Address of the new gateway
     */
    event GatewaySet(address gateway);

    /**
     * @notice Emitted when the address of the Graph Token contract on L1 has been updated
     * @param l1Address Address of the L1 Graph Token contract
     */
    event L1AddressSet(address l1Address);

    /**
     * @dev Checks that the sender is the L2 gateway from the L1/L2 token bridge
     */
    modifier onlyGateway() {
        require(msg.sender == gateway, "NOT_GATEWAY");
        _;
    }

    /**
     * @notice L2 Graph Token Contract initializer.
     * @dev Note some parameters have to be set separately as they are generally
     * not expected to be available at initialization time:
     * - gateway using setGateway
     * - l1Address using setL1Address
     * @param _owner Governance address that owns this contract
     */
    function initialize(address _owner) external onlyImpl initializer {
        require(_owner != address(0), "Owner must be set");
        // Initial supply hard coded to 0 as tokens are only supposed
        // to be minted through the bridge.
        GraphTokenUpgradeable._initialize(_owner, 0);
    }

    /**
     * @notice Sets the address of the L2 gateway allowed to mint tokens
     * @param _gw Address for the L2GraphTokenGateway that will be allowed to mint tokens
     */
    function setGateway(address _gw) external onlyGovernor {
        require(_gw != address(0), "INVALID_GATEWAY");
        gateway = _gw;
        emit GatewaySet(_gw);
    }

    /**
     * @notice Sets the address of the counterpart token on L1
     * @param _addr Address for the GraphToken contract on L1
     */
    function setL1Address(address _addr) external onlyGovernor {
        require(_addr != address(0), "INVALID_L1_ADDRESS");
        l1Address = _addr;
        emit L1AddressSet(_addr);
    }

    /**
     * @inheritdoc IArbToken
     * @dev Only callable by the L2GraphTokenGateway when tokens are transferred to L2
     */
    function bridgeMint(address _account, uint256 _amount) external override onlyGateway {
        _mint(_account, _amount);
        emit BridgeMinted(_account, _amount);
    }

    /**
     * @inheritdoc IArbToken
     * @dev Only callable by the L2GraphTokenGateway when tokens are transferred back to L1
     */
    function bridgeBurn(address _account, uint256 _amount) external override onlyGateway {
        burnFrom(_account, _amount);
        emit BridgeBurned(_account, _amount);
    }
}

// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0 <0.8.0;
import "../proxy/Initializable.sol";

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
abstract contract ReentrancyGuardUpgradeable is Initializable {
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

    function __ReentrancyGuard_init() internal initializer {
        __ReentrancyGuard_init_unchained();
    }

    function __ReentrancyGuard_init_unchained() internal initializer {
        _status = _NOT_ENTERED;
    }

    /**
     * @dev Prevents a contract from calling itself, directly or indirectly.
     * Calling a `nonReentrant` function from another `nonReentrant`
     * function is not supported. It is possible to prevent this from happening
     * by making the `nonReentrant` function external, and make it call a
     * `private` function that does the actual work.
     */
    modifier nonReentrant() {
        // On the first call to nonReentrant, _notEntered will be true
        require(_status != _ENTERED, "ReentrancyGuard: reentrant call");

        // Any calls to nonReentrant after this point will fail
        _status = _ENTERED;

        _;

        // By storing the original value once again, a refund is triggered (see
        // https://eips.ethereum.org/EIPS/eip-2200)
        _status = _NOT_ENTERED;
    }
    uint256[49] private __gap;
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

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-increment-by-one, gas-small-strings, gas-strict-inequalities
// solhint-disable named-parameters-mapping

import { ERC20BurnableUpgradeable } from "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20BurnableUpgradeable.sol";
import { ECDSAUpgradeable } from "@openzeppelin/contracts-upgradeable/cryptography/ECDSAUpgradeable.sol";

import { GraphUpgradeable } from "../../upgrades/GraphUpgradeable.sol";
import { Governed } from "../../governance/Governed.sol";

/**
 * @title GraphTokenUpgradeable contract
 * @author Edge & Node
 * @notice This is the implementation of the ERC20 Graph Token.
 * The implementation exposes a permit() function to allow for a spender to send a signed message
 * and approve funds to a spender following EIP2612 to make integration with other contracts easier.
 *
 * The token is initially owned by the deployer address that can mint tokens to create the initial
 * distribution. For convenience, an initial supply can be passed in the constructor that will be
 * assigned to the deployer.
 *
 * The governor can add contracts allowed to mint indexing rewards.
 *
 * Note this is an exact copy of the original GraphToken contract, but using
 * initializer functions and upgradeable OpenZeppelin contracts instead of
 * the original's constructor + non-upgradeable approach.
 */
abstract contract GraphTokenUpgradeable is GraphUpgradeable, Governed, ERC20BurnableUpgradeable {
    // -- EIP712 --
    // https://github.com/ethereum/EIPs/blob/master/EIPS/eip-712.md#definition-of-domainseparator

    /// @dev Hash of the EIP-712 Domain type
    bytes32 private immutable DOMAIN_TYPE_HASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract,bytes32 salt)");
    /// @dev Hash of the EIP-712 Domain name
    bytes32 private immutable DOMAIN_NAME_HASH = keccak256("Graph Token");
    /// @dev Hash of the EIP-712 Domain version
    bytes32 private immutable DOMAIN_VERSION_HASH = keccak256("0");
    /// @dev EIP-712 Domain salt
    bytes32 private immutable DOMAIN_SALT = 0xe33842a7acd1d5a1d28f25a931703e5605152dc48d64dc4716efdae1f5659591; // Randomly generated salt
    /// @dev Hash of the EIP-712 permit type
    bytes32 private immutable PERMIT_TYPEHASH =
        keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");

    // -- State --

    /// @dev EIP-712 Domain separator
    bytes32 private DOMAIN_SEPARATOR; // solhint-disable-line var-name-mixedcase
    /// @dev Addresses for which this mapping is true are allowed to mint tokens
    mapping(address => bool) private _minters;
    /// @notice Nonces for permit signatures for each token holder
    mapping(address => uint256) public nonces;
    /// @dev Storage gap added in case we need to add state variables to this contract
    uint256[47] private __gap;

    // -- Events --

    /**
     * @notice Emitted when a new minter is added
     * @param account Address of the minter that was added
     */
    event MinterAdded(address indexed account);

    /**
     * @notice Emitted when a minter is removed
     * @param account Address of the minter that was removed
     */
    event MinterRemoved(address indexed account);

    /// @dev Reverts if the caller is not an authorized minter
    modifier onlyMinter() {
        require(isMinter(msg.sender), "Only minter can call");
        _;
    }

    /**
     * @notice Approve token allowance by validating a message signed by the holder.
     * @param _owner Address of the token holder
     * @param _spender Address of the approved spender
     * @param _value Amount of tokens to approve the spender
     * @param _deadline Expiration time of the signed permit (if zero, the permit will never expire, so use with caution)
     * @param _v Signature recovery id
     * @param _r Signature r value
     * @param _s Signature s value
     */
    function permit(
        address _owner,
        address _spender,
        uint256 _value,
        uint256 _deadline,
        uint8 _v,
        bytes32 _r,
        bytes32 _s
    ) external {
        require(_deadline == 0 || block.timestamp <= _deadline, "GRT: expired permit");
        bytes32 digest = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(abi.encode(PERMIT_TYPEHASH, _owner, _spender, _value, nonces[_owner], _deadline))
            )
        );

        address recoveredAddress = ECDSAUpgradeable.recover(digest, _v, _r, _s);
        require(_owner == recoveredAddress, "GRT: invalid permit");

        nonces[_owner] = nonces[_owner] + 1;
        _approve(_owner, _spender, _value);
    }

    /**
     * @notice Add a new minter.
     * @param _account Address of the minter
     */
    function addMinter(address _account) external onlyGovernor {
        require(_account != address(0), "INVALID_MINTER");
        _addMinter(_account);
    }

    /**
     * @notice Remove a minter.
     * @param _account Address of the minter
     */
    function removeMinter(address _account) external onlyGovernor {
        require(isMinter(_account), "NOT_A_MINTER");
        _removeMinter(_account);
    }

    /**
     * @notice Renounce being a minter.
     */
    function renounceMinter() external {
        require(isMinter(msg.sender), "NOT_A_MINTER");
        _removeMinter(msg.sender);
    }

    /**
     * @notice Mint new tokens.
     * @param _to Address to send the newly minted tokens
     * @param _amount Amount of tokens to mint
     */
    function mint(address _to, uint256 _amount) external onlyMinter {
        _mint(_to, _amount);
    }

    /**
     * @notice Return if the `_account` is a minter or not.
     * @param _account Address to check
     * @return True if the `_account` is minter
     */
    function isMinter(address _account) public view returns (bool) {
        return _minters[_account];
    }

    /**
     * @notice Graph Token Contract initializer.
     * @param _owner Owner of this contract, who will hold the initial supply and will be a minter
     * @param _initialSupply Initial supply of GRT
     */
    function _initialize(address _owner, uint256 _initialSupply) internal {
        __ERC20_init("Graph Token", "GRT");
        Governed._initialize(_owner);

        // The Governor has the initial supply of tokens
        _mint(_owner, _initialSupply);

        // The Governor is the default minter
        _addMinter(_owner);

        // EIP-712 domain separator
        DOMAIN_SEPARATOR = keccak256(
            abi.encode(
                DOMAIN_TYPE_HASH,
                DOMAIN_NAME_HASH,
                DOMAIN_VERSION_HASH,
                _getChainID(),
                address(this),
                DOMAIN_SALT
            )
        );
    }

    /**
     * @notice Add a new minter.
     * @param _account Address of the minter
     */
    function _addMinter(address _account) private {
        _minters[_account] = true;
        emit MinterAdded(_account);
    }

    /**
     * @notice Remove a minter.
     * @param _account Address of the minter
     */
    function _removeMinter(address _account) private {
        _minters[_account] = false;
        emit MinterRemoved(_account);
    }

    /**
     * @notice Get the running network chain ID.
     * @return The chain ID
     */
    function _getChainID() private pure returns (uint256) {
        uint256 id;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            id := chainid()
        }
        return id;
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

/* solhint-disable gas-custom-errors */ // Cannot use custom errors with 0.7.6

/**
 * @title Graph Governance contract
 * @author Edge & Node
 * @notice All contracts that will be owned by a Governor entity should extend this contract.
 */
abstract contract Governed {
    // -- State --

    /**
     * @notice Address of the governor
     */
    address public governor;
    /**
     * @notice Address of the new governor that is pending acceptance
     */
    address public pendingGovernor;

    // -- Events --

    /**
     * @notice Emitted when a new owner/governor has been set, but is pending acceptance
     * @param from Previous pending governor address
     * @param to New pending governor address
     */
    event NewPendingOwnership(address indexed from, address indexed to);

    /**
     * @notice Emitted when a new owner/governor has accepted their role
     * @param from Previous governor address
     * @param to New governor address
     */
    event NewOwnership(address indexed from, address indexed to);

    /**
     * @dev Check if the caller is the governor.
     */
    modifier onlyGovernor() {
        require(msg.sender == governor, "Only Governor can call");
        _;
    }

    /**
     * @notice Initialize the governor for this contract
     * @param _initGovernor Address of the governor
     */
    function _initialize(address _initGovernor) internal {
        governor = _initGovernor;
    }

    /**
     * @notice Admin function to begin change of governor. The `_newGovernor` must call
     * `acceptOwnership` to finalize the transfer.
     * @param _newGovernor Address of new `governor`
     */
    function transferOwnership(address _newGovernor) external onlyGovernor {
        require(_newGovernor != address(0), "Governor must be set");

        address oldPendingGovernor = pendingGovernor;
        pendingGovernor = _newGovernor;

        emit NewPendingOwnership(oldPendingGovernor, pendingGovernor);
    }

    /**
     * @notice Admin function for pending governor to accept role and update governor.
     * This function must called by the pending governor.
     */
    function acceptOwnership() external {
        address oldPendingGovernor = pendingGovernor;

        require(
            oldPendingGovernor != address(0) && msg.sender == oldPendingGovernor,
            "Caller must be pending governor"
        );

        address oldGovernor = governor;

        governor = oldPendingGovernor;
        pendingGovernor = address(0);

        emit NewOwnership(oldGovernor, governor);
        emit NewPendingOwnership(oldPendingGovernor, pendingGovernor);
    }
}

// SPDX-License-Identifier: Apache-2.0

/*
 * Copyright 2019-2021, Offchain Labs, Inc.
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
 * https://github.com/OffchainLabs/arbitrum/tree/84e64dee6ee82adbf8ec34fd4b86c207a61d9007/packages/arb-bridge-eth
 *
 * MODIFIED from Offchain Labs' implementation:
 * - Changed solidity version to 0.7.3 (pablo@edgeandnode.com)
 *
 */

pragma solidity ^0.7.3;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec

library AddressAliasHelper {
    // solhint-disable-next-line const-name-snakecase
    uint160 internal constant offset = uint160(0x1111000000000000000000000000000000001111);

    /// @notice Utility function that converts the address in the L1 that submitted a tx to
    /// the inbox to the msg.sender viewed in the L2
    /// @param l1Address the address in the L1 that triggered the tx to L2
    /// @return l2Address L2 address as viewed in msg.sender
    function applyL1ToL2Alias(address l1Address) internal pure returns (address l2Address) {
        l2Address = address(uint160(l1Address) + offset);
    }

    /// @notice Utility function that converts the msg.sender viewed in the L2 to the
    /// address in the L1 that submitted a tx to the inbox
    /// @param l2Address L2 address as viewed in msg.sender
    /// @return l1Address the address in the L1 that triggered the tx to L2
    function undoL1ToL2Alias(address l2Address) internal pure returns (address l1Address) {
        l1Address = address(uint160(l2Address) - offset);
    }
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

