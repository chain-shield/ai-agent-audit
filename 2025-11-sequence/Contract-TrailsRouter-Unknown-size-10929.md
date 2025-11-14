
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IDelegatedExtension} from "wallet-contracts-v3/modules/interfaces/IDelegatedExtension.sol";
import {Tstorish} from "tstorish/Tstorish.sol";
import {DelegatecallGuard} from "./guards/DelegatecallGuard.sol";
import {IMulticall3} from "./interfaces/IMulticall3.sol";
import {ITrailsRouter} from "./interfaces/ITrailsRouter.sol";
import {TrailsSentinelLib} from "./libraries/TrailsSentinelLib.sol";

/// @title TrailsRouter
/// @author Miguel Mota, Shun Kakinoki
/// @notice Consolidated router for Trails operations including multicall routing, balance injection, and token sweeping
/// @dev Must be delegatecalled via the Sequence delegated extension module to access wallet storage/balances.
contract TrailsRouter is IDelegatedExtension, ITrailsRouter, DelegatecallGuard, Tstorish {
    // -------------------------------------------------------------------------
    // Libraries
    // -------------------------------------------------------------------------
    using SafeERC20 for IERC20;

    // -------------------------------------------------------------------------
    // Immutable Variables
    // -------------------------------------------------------------------------

    address public immutable MULTICALL3 = 0xcA11bde05977b3631167028862bE2a173976CA11;

    // -------------------------------------------------------------------------
    // Errors
    // -------------------------------------------------------------------------

    error NativeTransferFailed();
    error InvalidDelegatedSelector(bytes4 selector);
    error InvalidFunctionSelector(bytes4 selector);
    error AllowFailureMustBeFalse(uint256 callIndex);
    error SuccessSentinelNotSet();
    error NoEthSent();
    error NoTokensToPull();
    error InsufficientEth(uint256 required, uint256 received);
    error NoTokensToSweep();
    error NoEthAvailable();
    error AmountOffsetOutOfBounds();
    error PlaceholderMismatch();
    error TargetCallFailed(bytes revertData);

    // -------------------------------------------------------------------------
    // Receive ETH
    // -------------------------------------------------------------------------

    /// @notice Allow direct native token transfers when contract is used standalone.
    receive() external payable {}

    // -------------------------------------------------------------------------
    // Multicall3 Router Functions
    // -------------------------------------------------------------------------

    /// @inheritdoc ITrailsRouter
    function execute(bytes calldata data) public payable returns (IMulticall3.Result[] memory returnResults) {
        _validateRouterCall(data);
        (bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
        if (!success) revert TargetCallFailed(returnData);
        return abi.decode(returnData, (IMulticall3.Result[]));
    }

    /// @inheritdoc ITrailsRouter
    function pullAndExecute(address token, bytes calldata data)
        public
        payable
        returns (IMulticall3.Result[] memory returnResults)
    {
        uint256 amount;
        if (token == address(0)) {
            if (msg.value == 0) revert NoEthSent();
            amount = msg.value;
        } else {
            amount = _getBalance(token, msg.sender);
            if (amount == 0) revert NoTokensToPull();
        }

        return pullAmountAndExecute(token, amount, data);
    }

    /// @inheritdoc ITrailsRouter
    function pullAmountAndExecute(address token, uint256 amount, bytes calldata data)
        public
        payable
        returns (IMulticall3.Result[] memory returnResults)
    {
        _validateRouterCall(data);
        if (token == address(0)) {
            if (msg.value < amount) revert InsufficientEth(amount, msg.value);
        } else {
            _safeTransferFrom(token, msg.sender, address(this), amount);
        }

        (bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
        if (!success) revert TargetCallFailed(returnData);
        return abi.decode(returnData, (IMulticall3.Result[]));
    }

    // -------------------------------------------------------------------------
    // Balance Injection Functions
    // -------------------------------------------------------------------------

    /// @inheritdoc ITrailsRouter
    function injectSweepAndCall(
        address token,
        address target,
        bytes calldata callData,
        uint256 amountOffset,
        bytes32 placeholder
    ) external payable {
        uint256 callerBalance;

        if (token == address(0)) {
            callerBalance = msg.value;
            if (callerBalance == 0) revert NoEthSent();
        } else {
            callerBalance = _getBalance(token, msg.sender);
            if (callerBalance == 0) revert NoTokensToSweep();
            _safeTransferFrom(token, msg.sender, address(this), callerBalance);
        }

        _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
    }

    /// @inheritdoc ITrailsRouter
    function injectAndCall(
        address token,
        address target,
        bytes calldata callData,
        uint256 amountOffset,
        bytes32 placeholder
    ) public payable {
        uint256 callerBalance = _getSelfBalance(token);
        if (callerBalance == 0) {
            if (token == address(0)) {
                revert NoEthAvailable();
            } else {
                revert NoTokensToSweep();
            }
        }

        _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
    }

    // -------------------------------------------------------------------------
    // Token Sweeper Functions
    // -------------------------------------------------------------------------

    /// @inheritdoc ITrailsRouter
    function sweep(address _token, address _recipient) public payable onlyDelegatecall {
        uint256 amount = _getSelfBalance(_token);
        if (amount > 0) {
            if (_token == address(0)) {
                _transferNative(_recipient, amount);
            } else {
                _transferERC20(_token, _recipient, amount);
            }
            emit Sweep(_token, _recipient, amount);
        }
    }

    /// @inheritdoc ITrailsRouter
    function refundAndSweep(address _token, address _refundRecipient, uint256 _refundAmount, address _sweepRecipient)
        public
        payable
        onlyDelegatecall
    {
        uint256 current = _getSelfBalance(_token);

        uint256 actualRefund = _refundAmount > current ? current : _refundAmount;
        if (actualRefund != _refundAmount) {
            emit ActualRefund(_token, _refundRecipient, _refundAmount, actualRefund);
        }
        if (actualRefund > 0) {
            if (_token == address(0)) {
                _transferNative(_refundRecipient, actualRefund);
            } else {
                _transferERC20(_token, _refundRecipient, actualRefund);
            }
            emit Refund(_token, _refundRecipient, actualRefund);
        }

        uint256 remaining = _getSelfBalance(_token);
        if (remaining > 0) {
            if (_token == address(0)) {
                _transferNative(_sweepRecipient, remaining);
            } else {
                _transferERC20(_token, _sweepRecipient, remaining);
            }
            emit Sweep(_token, _sweepRecipient, remaining);
        }
        emit RefundAndSweep(_token, _refundRecipient, _refundAmount, _sweepRecipient, actualRefund, remaining);
    }

    /// @inheritdoc ITrailsRouter
    function validateOpHashAndSweep(bytes32 opHash, address _token, address _recipient)
        public
        payable
        onlyDelegatecall
    {
        uint256 slot = TrailsSentinelLib.successSlot(opHash);
        if (_getTstorish(slot) != TrailsSentinelLib.SUCCESS_VALUE) {
            revert SuccessSentinelNotSet();
        }
        sweep(_token, _recipient);
    }

    // -------------------------------------------------------------------------
    // Sequence Delegated Extension Entry Point
    // -------------------------------------------------------------------------

    /// @inheritdoc IDelegatedExtension
    function handleSequenceDelegateCall(
        bytes32 _opHash,
        uint256, /* _startingGas */
        uint256, /* _index */
        uint256, /* _numCalls */
        uint256, /* _space */
        bytes calldata _data
    )
        external
        override(IDelegatedExtension, ITrailsRouter)
        onlyDelegatecall
    {
        bytes4 selector;
        if (_data.length >= 4) {
            selector = bytes4(_data[0:4]);
        }

        // Balance Injection selectors
        if (selector == this.injectAndCall.selector) {
            (address token, address target, bytes memory callData, uint256 amountOffset, bytes32 placeholder) =
                abi.decode(_data[4:], (address, address, bytes, uint256, bytes32));
            _injectAndCallDelegated(token, target, callData, amountOffset, placeholder);
            return;
        }

        // Token Sweeper selectors
        if (selector == this.sweep.selector) {
            (address token, address recipient) = abi.decode(_data[4:], (address, address));
            sweep(token, recipient);
            return;
        }

        if (selector == this.refundAndSweep.selector) {
            (address token, address refundRecipient, uint256 refundAmount, address sweepRecipient) =
                abi.decode(_data[4:], (address, address, uint256, address));
            refundAndSweep(token, refundRecipient, refundAmount, sweepRecipient);
            return;
        }

        if (selector == this.validateOpHashAndSweep.selector) {
            (, address token, address recipient) = abi.decode(_data[4:], (bytes32, address, address));
            validateOpHashAndSweep(_opHash, token, recipient);
            return;
        }

        revert InvalidDelegatedSelector(selector);
    }

    // -------------------------------------------------------------------------
    // Internal Helpers
    // -------------------------------------------------------------------------

    /// forge-lint: disable-next-line(mixed-case-function)
    function _safeTransferFrom(address token, address from, address to, uint256 amount) internal {
        IERC20 erc20 = IERC20(token);
        SafeERC20.safeTransferFrom(erc20, from, to, amount);
    }

    /// forge-lint: disable-next-line(mixed-case-function)
    function _transferNative(address _to, uint256 _amount) internal {
        (bool success,) = payable(_to).call{value: _amount}("");
        if (!success) revert NativeTransferFailed();
    }

    /// forge-lint: disable-next-line(mixed-case-function)
    function _transferERC20(address _token, address _to, uint256 _amount) internal {
        IERC20 erc20 = IERC20(_token);
        SafeERC20.safeTransfer(erc20, _to, _amount);
    }

    /// forge-lint: disable-next-line(mixed-case-function)
    function _getBalance(address token, address account) internal view returns (uint256) {
        return token == address(0) ? account.balance : IERC20(token).balanceOf(account);
    }

    /// forge-lint: disable-next-line(mixed-case-function)
    function _getSelfBalance(address token) internal view returns (uint256) {
        return _getBalance(token, address(this));
    }

    /// forge-lint: disable-next-line(mixed-case-function)
    function _nativeBalance() internal view returns (uint256) {
        return address(this).balance;
    }

    /// forge-lint: disable-next-line(mixed-case-function)
    function _erc20Balance(address _token) internal view returns (uint256) {
        return IERC20(_token).balanceOf(address(this));
    }

    /// forge-lint: disable-next-line(mixed-case-function)
    function _erc20BalanceOf(address _token, address _account) internal view returns (uint256) {
        return IERC20(_token).balanceOf(_account);
    }

    /// forge-lint: disable-next-line(mixed-case-function)
    function _injectAndCallDelegated(
        address token,
        address target,
        bytes memory callData,
        uint256 amountOffset,
        bytes32 placeholder
    ) internal {
        uint256 callerBalance = _getSelfBalance(token);
        if (callerBalance == 0) {
            if (token == address(0)) {
                revert NoEthAvailable();
            } else {
                revert NoTokensToSweep();
            }
        }

        _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
    }

    /// forge-lint: disable-next-line(mixed-case-function)
    function _injectAndExecuteCall(
        address token,
        address target,
        bytes memory callData,
        uint256 amountOffset,
        bytes32 placeholder,
        uint256 callerBalance
    ) internal {
        // Replace placeholder with actual balance if needed
        bool shouldReplace = (amountOffset != 0 || placeholder != bytes32(0));

        if (shouldReplace) {
            if (callData.length < amountOffset + 32) revert AmountOffsetOutOfBounds();

            bytes32 found;
            assembly {
                found := mload(add(add(callData, 32), amountOffset))
            }
            if (found != placeholder) revert PlaceholderMismatch();

            assembly {
                mstore(add(add(callData, 32), amountOffset), callerBalance)
            }
        }

        // Execute call based on token type
        if (token == address(0)) {
            (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
            emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
            if (!success) revert TargetCallFailed(result);
        } else {
            IERC20 erc20 = IERC20(token);
            SafeERC20.forceApprove(erc20, target, callerBalance);

            (bool success, bytes memory result) = target.call(callData);
            emit BalanceInjectorCall(token, target, placeholder, callerBalance, amountOffset, success, result);
            if (!success) revert TargetCallFailed(result);
        }
    }

    /// forge-lint: disable-next-line(mixed-case-function)
    function _validateRouterCall(bytes memory callData) internal pure {
        // Extract function selector
        if (callData.length < 4) revert InvalidFunctionSelector(bytes4(0));

        bytes4 selector;
        assembly {
            selector := mload(add(callData, 32))
        }

        // Only allow `aggregate3Value` calls (0x174dea71)
        if (selector != 0x174dea71) {
            revert InvalidFunctionSelector(selector);
        }

        // Decode and validate the Call3Value[] array to ensure allowFailure=false for all calls
        IMulticall3.Call3Value[] memory calls = abi.decode(_sliceCallData(callData, 4), (IMulticall3.Call3Value[]));

        // Iterate through all calls and verify allowFailure is false
        for (uint256 i = 0; i < calls.length; i++) {
            if (calls[i].allowFailure) {
                revert AllowFailureMustBeFalse(i);
            }
        }
    }

    /// forge-lint: disable-next-line(mixed-case-function)
    function _sliceCallData(bytes memory data, uint256 start) internal pure returns (bytes memory) {
        bytes memory result = new bytes(data.length - start);
        for (uint256 i = 0; i < result.length; i++) {
            result[i] = data[start + i];
        }
        return result;
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

// -------------------------------------------------------------------------
// Library
// -------------------------------------------------------------------------
library TrailsSentinelLib {
    // -------------------------------------------------------------------------
    // Constants
    // -------------------------------------------------------------------------
    bytes32 public constant SENTINEL_NAMESPACE = keccak256("org.sequence.trails.router.sentinel");
    uint256 public constant SUCCESS_VALUE = uint256(1);

    // -------------------------------------------------------------------------
    // Storage Slot Helpers
    // -------------------------------------------------------------------------
    function successSlot(bytes32 opHash) internal pure returns (uint256 result) {
        // return keccak256(abi.encode(SENTINEL_NAMESPACE, opHash));
        bytes32 namespace = SENTINEL_NAMESPACE;
        assembly {
            mstore(0x00, namespace)
            mstore(0x20, opHash)
            result := keccak256(0x00, 0x40)
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @notice Abstract contract providing a reusable delegatecall-only guard.
abstract contract DelegatecallGuard {
    // -------------------------------------------------------------------------
    // Errors
    // -------------------------------------------------------------------------

    /// @dev Error thrown when a function expected to be delegatecalled is invoked directly
    error NotDelegateCall();

    // -------------------------------------------------------------------------
    // Immutable Variables
    // -------------------------------------------------------------------------

    /// @dev Cached address of this contract to detect delegatecall context
    address internal immutable _SELF = address(this);

    // -------------------------------------------------------------------------
    // Modifiers
    // -------------------------------------------------------------------------

    /// @dev Modifier restricting functions to only be executed via delegatecall
    modifier onlyDelegatecall() {
        _onlyDelegatecall();
        _;
    }

    // -------------------------------------------------------------------------
    // Internal Functions
    // -------------------------------------------------------------------------

    /// @dev Internal check enforcing delegatecall context
    function _onlyDelegatecall() internal view {
        if (address(this) == _SELF) revert NotDelegateCall();
    }
}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

/// @title IDelegatedExtension
/// @author Agustin Aguilar
/// @notice Interface for the delegated extension module
interface IDelegatedExtension {

  /// @notice Handle a sequence delegate call
  /// @param _opHash The operation hash
  /// @param _startingGas The starting gas
  /// @param _index The index
  /// @param _numCalls The number of calls
  /// @param _space The space
  /// @param _data The data
  function handleSequenceDelegateCall(
    bytes32 _opHash,
    uint256 _startingGas,
    uint256 _index,
    uint256 _numCalls,
    uint256 _space,
    bytes calldata _data
  ) external;

}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IDelegatedExtension} from "wallet-contracts-v3/modules/interfaces/IDelegatedExtension.sol";
import {IMulticall3} from "./IMulticall3.sol";

/// @title ITrailsRouter
/// @notice Interface describing the delegate-call router utilities exposed to Sequence wallets.
interface ITrailsRouter is IDelegatedExtension {
    // ---------------------------------------------------------------------
    // Events
    // ---------------------------------------------------------------------

    event BalanceInjectorCall(
        address indexed token,
        address indexed target,
        bytes32 placeholder,
        uint256 amountReplaced,
        uint256 amountOffset,
        bool success,
        bytes result
    );
    event Refund(address indexed token, address indexed recipient, uint256 amount);
    event Sweep(address indexed token, address indexed recipient, uint256 amount);
    event RefundAndSweep(
        address indexed token,
        address indexed refundRecipient,
        uint256 refundAmount,
        address indexed sweepRecipient,
        uint256 actualRefund,
        uint256 remaining
    );
    event ActualRefund(address indexed token, address indexed recipient, uint256 expected, uint256 actual);

    // ---------------------------------------------------------------------
    // Multicall Operations
    // ---------------------------------------------------------------------

    /// @notice Delegates to Multicall3 to preserve msg.sender context.
    /// @dev Delegates to Multicall3 to preserve msg.sender context.
    /// @param data The data to execute.
    /// @return returnResults The result of the execution.
    function execute(bytes calldata data) external payable returns (IMulticall3.Result[] memory returnResults);

    /// @notice Pull ERC20 from msg.sender, then delegatecall into Multicall3.
    /// @dev Requires prior approval to this router.
    /// @param token The ERC20 token to pull, or address(0) for ETH.
    /// @param data The calldata for Multicall3.
    /// @return returnResults The result of the execution.
    function pullAndExecute(address token, bytes calldata data)
        external
        payable
        returns (IMulticall3.Result[] memory returnResults);

    /// @notice Pull specific amount of ERC20 from msg.sender, then delegatecall into Multicall3.
    /// @dev Requires prior approval to this router.
    /// @param token The ERC20 token to pull, or address(0) for ETH.
    /// @param amount The amount to pull.
    /// @param data The calldata for Multicall3.
    /// @return returnResults The result of the execution.
    function pullAmountAndExecute(address token, uint256 amount, bytes calldata data)
        external
        payable
        returns (IMulticall3.Result[] memory returnResults);

    // ---------------------------------------------------------------------
    // Balance Injection
    // ---------------------------------------------------------------------

    /// @notice Sweeps tokens from msg.sender and calls target with modified calldata.
    /// @dev For regular calls (not delegatecall). Transfers tokens from msg.sender to this contract first.
    /// @param token The ERC-20 token to sweep, or address(0) for ETH.
    /// @param target The address to call with modified calldata.
    /// @param callData The original calldata (must include a 32-byte placeholder).
    /// @param amountOffset The byte offset in calldata where the placeholder is located.
    /// @param placeholder The 32-byte placeholder that will be replaced with balance.
    function injectSweepAndCall(
        address token,
        address target,
        bytes calldata callData,
        uint256 amountOffset,
        bytes32 placeholder
    ) external payable;

    /// @notice Injects balance and calls target (for delegatecall context).
    /// @dev For delegatecalls from Sequence wallets. Reads balance from address(this).
    /// @param token The ERC-20 token to sweep, or address(0) for ETH.
    /// @param target The address to call with modified calldata.
    /// @param callData The original calldata (must include a 32-byte placeholder).
    /// @param amountOffset The byte offset in calldata where the placeholder is located.
    /// @param placeholder The 32-byte placeholder that will be replaced with balance.
    function injectAndCall(
        address token,
        address target,
        bytes calldata callData,
        uint256 amountOffset,
        bytes32 placeholder
    ) external payable;

    /// @notice Validates that the success sentinel for an opHash is set, then sweeps tokens.
    /// @dev For delegatecall context. Used to ensure prior operation succeeded.
    /// @param opHash The operation hash to validate.
    /// @param token The token to sweep.
    /// @param recipient The recipient of the sweep.
    function validateOpHashAndSweep(bytes32 opHash, address token, address recipient) external payable;

    // ---------------------------------------------------------------------
    // Sweeper
    // ---------------------------------------------------------------------

    /// @notice Approves the sweeper if ERC20, then sweeps the entire balance to recipient.
    /// @dev For delegatecall context. Approval is set for `SELF` on the wallet.
    /// @param token The address of the token to sweep. Use address(0) for the native token.
    /// @param recipient The address to send the swept tokens to.
    function sweep(address token, address recipient) external payable;

    /// @notice Refunds up to `_refundAmount` to `_refundRecipient`, then sweeps any remaining balance to `_sweepRecipient`.
    /// @dev For delegatecall context.
    /// @param token The token address to operate on. Use address(0) for native.
    /// @param refundRecipient Address receiving the refund portion.
    /// @param refundAmount Maximum amount to refund.
    /// @param sweepRecipient Address receiving the remaining balance.
    function refundAndSweep(address token, address refundRecipient, uint256 refundAmount, address sweepRecipient)
        external
        payable;

    // ---------------------------------------------------------------------
    // Delegate Entry
    // ---------------------------------------------------------------------

    /// @inheritdoc IDelegatedExtension
    function handleSequenceDelegateCall(
        bytes32 opHash,
        uint256 startingGas,
        uint256 index,
        uint256 numCalls,
        uint256 space,
        bytes calldata data
    ) external;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Tstorish {
    // Declare a storage variable indicating if TSTORE support has been
    // activated post-deployment.
    bool private _tstoreSupport;

    /*
     * ------------------------------------------------------------------------+
     * Opcode      | Mnemonic         | Stack              | Memory            |
     * ------------------------------------------------------------------------|
     * 60 0x02     | PUSH1 0x02       | 0x02               |                   |
     * 60 0x1e     | PUSH1 0x1e       | 0x1e 0x02          |                   |
     * 61 0x3d5c   | PUSH2 0x3d5c     | 0x3d5c 0x1e 0x02   |                   |
     * 3d          | RETURNDATASIZE   | 0 0x3d5c 0x1e 0x02 |                   |
     *                                                                         |
     * :: store deployed bytecode in memory: (3d) RETURNDATASIZE (5c) TLOAD :: |
     * 52          | MSTORE           | 0x1e 0x02          | [0..0x20): 0x3d5c |
     * f3          | RETURN           |                    | [0..0x20): 0x3d5c |
     * ------------------------------------------------------------------------+
     */
    uint256 constant _TLOAD_TEST_PAYLOAD = 0x6002_601e_613d5c_3d_52_f3;
    uint256 constant _TLOAD_TEST_PAYLOAD_LENGTH = 0x0a;
    uint256 constant _TLOAD_TEST_PAYLOAD_OFFSET = 0x16;

    // Declare an immutable variable to store the tstore test contract address.
    address private immutable _tloadTestContract;

    // Declare an immutable variable to store the initial TSTORE support status.
    bool private immutable _tstoreInitialSupport;

    // Declare an immutable function type variable for the _setTstorish function
    // based on chain support for tstore at time of deployment.
    function(uint256,uint256) internal immutable _setTstorish;

    // Declare an immutable function type variable for the _getTstorish function
    // based on chain support for tstore at time of deployment.
    function(uint256) view returns (uint256) internal immutable _getTstorish;

    // Declare an immutable function type variable for the _clearTstorish function
    // based on chain support for tstore at time of deployment.
    function(uint256) internal immutable _clearTstorish;

    // Declare a few custom revert error types.
    error TStoreAlreadyActivated();
    error TStoreNotSupported();
    error TloadTestContractDeploymentFailed();
    error OnlyDirectCalls();

    /**
     * @dev Determine TSTORE availability during deployment. This involves
     *      attempting to deploy a contract that utilizes TLOAD as part of the
     *      contract construction bytecode, and configuring initial support for
     *      using TSTORE in place of SSTORE based on the result.
     */
    constructor() {
        // Deploy the contract testing TLOAD support and store the address.
        address tloadTestContract = _prepareTloadTest();

        // Ensure the deployment was successful.
        if (tloadTestContract == address(0)) {
            revert TloadTestContractDeploymentFailed();
        }

        // Determine if TSTORE is supported.
        bool tstoreInitialSupport = _testTload(tloadTestContract);

        if (tstoreInitialSupport) {
            // If TSTORE is supported, set functions to their versions that use
            // tstore/tload directly without support checks.
            _setTstorish = _setTstore;
            _getTstorish = _getTstore;
            _clearTstorish = _clearTstore;
        } else {
            // If TSTORE is not supported, set functions to their versions that 
            // fallback to sstore/sload until _tstoreSupport is true.
            _setTstorish = _setTstorishWithSstoreFallback;
            _getTstorish = _getTstorishWithSloadFallback;
            _clearTstorish = _clearTstorishWithSstoreFallback;
        }

        _tstoreInitialSupport = tstoreInitialSupport;

        // Set the address of the deployed TLOAD test contract as an immutable.
        _tloadTestContract = tloadTestContract;
    }

    /**
     * @dev External function to activate TSTORE usage. Does not need to be
     *      called if TSTORE is supported from deployment, and only needs to be
     *      called once. Reverts if TSTORE has already been activated or if the
     *      opcode is not available. Note that this must be called directly from
     *      an externally-owned account to avoid potential reentrancy issues.
     */
    function __activateTstore() external {
        // Ensure this function is triggered from an externally-owned account.
        if (msg.sender != tx.origin) {
            revert OnlyDirectCalls();
        }

        // Determine if TSTORE can potentially be activated.
        if (_tstoreInitialSupport || _tstoreSupport) {
            revert TStoreAlreadyActivated();
        }

        // Determine if TSTORE can be activated and revert if not.
        if (!_testTload(_tloadTestContract)) {
            revert TStoreNotSupported();
        }

        // Mark TSTORE as activated.
        _tstoreSupport = true;
    }

    /**
     * @dev Private function to set a TSTORISH value. Assigned to _setTstorish 
     *      internal function variable at construction if chain has tstore support.
     *
     * @param storageSlot The slot to write the TSTORISH value to.
     * @param value       The value to write to the given storage slot.
     */
    function _setTstore(uint256 storageSlot, uint256 value) private {
        assembly {
            tstore(storageSlot, value)
        }
    }

    /**
     * @dev Private function to set a TSTORISH value with sstore fallback. 
     *      Assigned to _setTstorish internal function variable at construction
     *      if chain does not have tstore support.
     *
     * @param storageSlot The slot to write the TSTORISH value to.
     * @param value       The value to write to the given storage slot.
     */
    function _setTstorishWithSstoreFallback(uint256 storageSlot, uint256 value) private {
        if (_tstoreSupport) {
            assembly {
                tstore(storageSlot, value)
            }
        } else {
            assembly {
                sstore(storageSlot, value)
            }
        }
    }

    /**
     * @dev Private function to read a TSTORISH value. Assigned to _getTstorish
     *      internal function variable at construction if chain has tstore support.
     *
     * @param storageSlot The slot to read the TSTORISH value from.
     *
     * @return value The TSTORISH value at the given storage slot.
     */
    function _getTstore(
        uint256 storageSlot
    ) private view returns (uint256 value) {
        assembly {
            value := tload(storageSlot)
        }
    }

    /**
     * @dev Private function to read a TSTORISH value with sload fallback. 
     *      Assigned to _getTstorish internal function variable at construction
     *      if chain does not have tstore support.
     *
     * @param storageSlot The slot to read the TSTORISH value from.
     *
     * @return value The TSTORISH value at the given storage slot.
     */
    function _getTstorishWithSloadFallback(
        uint256 storageSlot
    ) private view returns (uint256 value) {
        if (_tstoreSupport) {
            assembly {
                value := tload(storageSlot)
            }
        } else {
            assembly {
                value := sload(storageSlot)
            }
        }
    }

    /**
     * @dev Private function to clear a TSTORISH value. Assigned to _clearTstorish internal 
     *      function variable at construction if chain has tstore support.
     *
     * @param storageSlot The slot to clear the TSTORISH value for.
     */
    function _clearTstore(uint256 storageSlot) private {
        assembly {
            tstore(storageSlot, 0)
        }
    }

    /**
     * @dev Private function to clear a TSTORISH value with sstore fallback. 
     *      Assigned to _clearTstorish internal function variable at construction
     *      if chain does not have tstore support.
     *
     * @param storageSlot The slot to clear the TSTORISH value for.
     */
    function _clearTstorishWithSstoreFallback(uint256 storageSlot) private {
        if (_tstoreSupport) {
            assembly {
                tstore(storageSlot, 0)
            }
        } else {
            assembly {
                sstore(storageSlot, 0)
            }
        }
    }

    /**
     * @dev Private function to deploy a test contract that utilizes TLOAD as
     *      part of its fallback logic.
     */
    function _prepareTloadTest() private returns (address contractAddress) {
        // Utilize assembly to deploy a contract testing TLOAD support.
        assembly {
            // Write the contract deployment code payload to scratch space.
            mstore(0, _TLOAD_TEST_PAYLOAD)

            // Deploy the contract.
            contractAddress := create(
                0,
                _TLOAD_TEST_PAYLOAD_OFFSET,
                _TLOAD_TEST_PAYLOAD_LENGTH
            )
        }
    }

    /**
     * @dev Private view function to determine if TSTORE/TLOAD are supported by
     *      the current EVM implementation by attempting to call the test
     *      contract, which utilizes TLOAD as part of its fallback logic.
     */
    function _testTload(
        address tloadTestContract
    ) private view returns (bool ok) {
        // Call the test contract, which will perform a TLOAD test. If the call
        // does not revert, then TLOAD/TSTORE is supported. Do not forward all
        // available gas, as all forwarded gas will be consumed on revert.
        (ok, ) = tloadTestContract.staticcall{ gas: gasleft() / 10 }("");
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title IMulticall3
/// @notice Minimal subset of Multicall3 used by Trails router.
/// @dev Matches the canonical implementation deployed at `0xcA11bde05977b3631167028862bE2a173976CA11`.
interface IMulticall3 {
    // -------------------------------------------------------------------------
    // Structs
    // -------------------------------------------------------------------------

    struct Call3 {
        address target;
        bool allowFailure;
        bytes callData;
    }

    struct Call3Value {
        address target;
        bool allowFailure;
        uint256 value;
        bytes callData;
    }

    struct Result {
        bool success;
        bytes returnData;
    }

    // -------------------------------------------------------------------------
    // Functions
    // -------------------------------------------------------------------------

    function aggregate3(Call3[] calldata calls) external payable returns (Result[] memory returnData);

    function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory returnData);
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { WithdrawablePeriphery } from "../Helpers/WithdrawablePeriphery.sol";
import { InvalidConfig } from "../Errors/GenericErrors.sol";

/// @title IStETH
/// @notice External interface for Lido's stETH contract which supports wrapping and unwrapping wstETH
interface IStETH is IERC20 {
    /// @notice Unwraps wstETH into stETH
    /// @param amount The amount of wstETH to unwrap
    function wrap(uint256 amount) external returns (uint256 unwrappedAmount);

    /// @notice Wraps stETH into wstETH
    /// @param amount The amount of stETH to wrap
    function unwrap(uint256 amount) external returns (uint256 wrappedAmount);
}

/// @title LidoWrapper
/// @author LI.FI (https://li.fi)
/// @notice Wraps and unwraps Lido’s wstETH and stETH tokens
/// @dev Be aware that Lido's L2 `wrap`/`unwrap` naming is reversed from the typical expectation.
/// @dev Any stETH or wstETH tokens sent directly to the contract can be irrecoverably swept by MEV bots
/// @custom:version 1.0.0
contract LidoWrapper is WithdrawablePeriphery {
    uint256 public constant ETH_CHAIN_ID = 1;

    /// @notice Reference to the L2 stETH contract
    IStETH public immutable ST_ETH;

    /// @notice Address of the wstETH token contract
    address public immutable WST_ETH_ADDRESS;

    error ContractNotYetReadyForMainnet();

    /// @notice Constructor
    /// @param _stETHAddress The address of the stETH token on L2
    /// @param _wstETHAddress The address of the bridged wstETH token on L2
    /// @param _owner The address of the contract owner
    constructor(
        address _stETHAddress,
        address _wstETHAddress,
        address _owner
    ) WithdrawablePeriphery(_owner) {
        if (
            _stETHAddress == address(0) ||
            _wstETHAddress == address(0) ||
            _owner == address(0)
        ) revert InvalidConfig();

        ST_ETH = IStETH(_stETHAddress);
        WST_ETH_ADDRESS = _wstETHAddress;

        // the wrap/unwrap functions are different on mainnet
        if (block.chainid == ETH_CHAIN_ID)
            revert ContractNotYetReadyForMainnet();

        // Approve stETH contract to pull wstETH from this contract
        IERC20(WST_ETH_ADDRESS).approve(address(ST_ETH), type(uint256).max);
    }

    /// @notice Wraps stETH into wstETH
    /// @dev Transfers `_amount` stETH from caller, unwraps it via the stETH contract (which yields wstETH),
    ///      and returns wstETH to the caller.
    /// @param _amount The amount of stETH to wrap into wstETH
    function wrapStETHToWstETH(
        uint256 _amount
    ) external returns (uint256 wrappedAmount) {
        // Pull stETH from sender
        IERC20(address(ST_ETH)).transferFrom(
            msg.sender,
            address(this),
            _amount
        );

        // Call `unwrap` on stETH contract to get wstETH (naming is inverted) with full stETH contract balance
        // This contract is designed to not hold funds so sending full balance is not a problem
        uint256 stETHBalance = IERC20(address(ST_ETH)).balanceOf(
            address(this)
        );
        wrappedAmount = ST_ETH.unwrap(stETHBalance);

        // Transfer resulting wstETH to sender
        IERC20(WST_ETH_ADDRESS).transfer(msg.sender, wrappedAmount);

        // we are not emitting an event since the Lido contracts already emit events
    }

    /// @notice Unwraps wstETH into stETH
    /// @dev Transfers `_amount` wstETH from caller, wraps it via stETH contract (yielding stETH),
    ///      and returns stETH to the caller.
    /// @param _amount The amount of wstETH to unwrap into stETH
    function unwrapWstETHToStETH(
        uint256 _amount
    ) external returns (uint256 unwrappedAmount) {
        // Pull wstETH from sender
        IERC20(WST_ETH_ADDRESS).transferFrom(
            msg.sender,
            address(this),
            _amount
        );

        // Call `wrap` on stETH contract to get stETH (again, inverted naming)
        unwrappedAmount = ST_ETH.wrap(_amount);

        // Transfer resulting stETH to sender
        IERC20(address(ST_ETH)).transfer(msg.sender, unwrappedAmount);

        // we are not emitting an event since the Lido contracts already emit events
    }
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";
import {Deploy as TrailsRouterShimDeploy} from "script/TrailsRouterShim.s.sol";
import {TrailsRouterShim} from "src/TrailsRouterShim.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {Create2Utils} from "../utils/Create2Utils.sol";

// -----------------------------------------------------------------------------
// Test Contract
// -----------------------------------------------------------------------------

contract TrailsRouterShimDeploymentTest is Test {
    // -------------------------------------------------------------------------
    // Test State Variables
    // -------------------------------------------------------------------------

    TrailsRouterShimDeploy internal _deployScript;
    address internal _deployer;
    uint256 internal _deployerPk;
    string internal _deployerPkStr;

    // -------------------------------------------------------------------------
    // Pure Functions
    // -------------------------------------------------------------------------

    // Expected predetermined addresses (calculated using CREATE2)
    function expectedRouterAddress() internal pure returns (address payable) {
        return Create2Utils.calculateCreate2Address(type(TrailsRouter).creationCode, Create2Utils.standardSalt());
    }

    function expectedShimAddress() internal pure returns (address payable) {
        address routerAddr = expectedRouterAddress();
        bytes memory shimInitCode = abi.encodePacked(type(TrailsRouterShim).creationCode, abi.encode(routerAddr));
        return Create2Utils.calculateCreate2Address(shimInitCode, Create2Utils.standardSalt());
    }

    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------

    function setUp() public {
        _deployScript = new TrailsRouterShimDeploy();
        _deployerPk = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80; // anvil default key
        _deployerPkStr = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        _deployer = vm.addr(_deployerPk);
        vm.deal(_deployer, 100 ether);
    }

    // -------------------------------------------------------------------------
    // Test Functions
    // -------------------------------------------------------------------------

    function test_DeployRouterShim_Success() public {
        vm.setEnv("PRIVATE_KEY", _deployerPkStr);

        vm.recordLogs();
        _deployScript.run();

        // Get the actual router address from the deployment script
        address deployedRouterAddr = _deployScript.routerAddress();

        // Verify TrailsRouter was deployed
        assertEq(deployedRouterAddr.code.length > 0, true, "TrailsRouter should be deployed");

        // Verify TrailsRouterShim was deployed at the expected address
        address payable expectedShimAddr = expectedShimAddress();
        assertEq(expectedShimAddr.code.length > 0, true, "TrailsRouterShim should be deployed at expected address");

        // Verify the shim's router address is correctly set
        TrailsRouterShim shim = TrailsRouterShim(expectedShimAddr);
        assertEq(address(shim.ROUTER()), deployedRouterAddr, "Shim should have correct router address");
    }

    function test_DeployRouterShim_SameAddress() public {
        vm.setEnv("PRIVATE_KEY", _deployerPkStr);

        // First deployment
        vm.recordLogs();
        _deployScript.run();

        // Get the actual router address from the deployment script
        address deployedRouterAddr = _deployScript.routerAddress();

        // Verify first deployment addresses
        assertEq(deployedRouterAddr.code.length > 0, true, "First deployment: TrailsRouter deployed");
        address payable expectedShimAddr = expectedShimAddress();
        assertEq(expectedShimAddr.code.length > 0, true, "First deployment: TrailsRouterShim deployed");

        // Re-set the PRIVATE_KEY for second deployment
        vm.setEnv("PRIVATE_KEY", _deployerPkStr);

        // Second deployment should result in the same address (deterministic)
        vm.recordLogs();
        _deployScript.run();

        // Verify second deployment still has contracts at same addresses
        assertEq(deployedRouterAddr.code.length > 0, true, "Second deployment: TrailsRouter still deployed");
        assertEq(expectedShimAddr.code.length > 0, true, "Second deployment: TrailsRouterShim still deployed");

        // Both deployments should succeed without reverting
    }

    function test_DeployedContract_HasCorrectConfiguration() public {
        vm.setEnv("PRIVATE_KEY", _deployerPkStr);

        // Deploy the script
        _deployScript.run();

        // Get references to deployed contracts
        address deployedRouterAddr = _deployScript.routerAddress();
        address payable expectedShimAddr = expectedShimAddress();
        TrailsRouterShim shim = TrailsRouterShim(expectedShimAddr);
        TrailsRouter router = TrailsRouter(payable(deployedRouterAddr));

        // Verify the router address is set correctly in the shim
        assertEq(address(shim.ROUTER()), deployedRouterAddr, "Shim should have correct router address set");

        // Verify router is properly initialized (basic smoke test)
        assertEq(address(router).code.length > 0, true, "Router should have code");

        // Test that the shim can access its router (basic functionality test)
        // This tests that the immutable is correctly set and accessible
        address routerFromShim = address(shim.ROUTER());
        assertEq(routerFromShim, deployedRouterAddr, "Shim should be able to access its router address");
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";
import {Deploy as TrailsRouterDeploy} from "script/TrailsRouter.s.sol";
import {TrailsRouter} from "src/TrailsRouter.sol";
import {Create2Utils} from "../utils/Create2Utils.sol";

// -----------------------------------------------------------------------------
// Test Contract
// -----------------------------------------------------------------------------

contract TrailsRouterDeploymentTest is Test {
    // -------------------------------------------------------------------------
    // Test State Variables
    // -------------------------------------------------------------------------

    TrailsRouterDeploy internal _deployScript;
    address internal _deployer;
    uint256 internal _deployerPk;
    string internal _deployerPkStr;

    // -------------------------------------------------------------------------
    // Pure Functions
    // -------------------------------------------------------------------------

    // Expected predetermined address (calculated using CREATE2)
    function expectedRouterAddress() internal pure returns (address payable) {
        return Create2Utils.calculateCreate2Address(type(TrailsRouter).creationCode, Create2Utils.standardSalt());
    }

    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------

    function setUp() public {
        _deployScript = new TrailsRouterDeploy();
        _deployerPk = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80; // anvil default key
        _deployerPkStr = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        _deployer = vm.addr(_deployerPk);
        vm.deal(_deployer, 100 ether);
    }

    // -------------------------------------------------------------------------
    // Test Functions
    // -------------------------------------------------------------------------

    function test_DeployTrailsRouter_Success() public {
        vm.setEnv("PRIVATE_KEY", _deployerPkStr);

        vm.recordLogs();
        _deployScript.run();

        // Verify TrailsRouter was deployed at the expected address
        address payable expectedAddr = expectedRouterAddress();
        assertEq(expectedAddr.code.length > 0, true, "TrailsRouter should be deployed");

        // Verify the deployed contract is functional
        TrailsRouter router = TrailsRouter(expectedAddr);
        assertEq(address(router).code.length > 0, true, "Router should have code");
    }

    function test_DeployTrailsRouter_SameAddress() public {
        vm.setEnv("PRIVATE_KEY", _deployerPkStr);

        // First deployment
        vm.recordLogs();
        _deployScript.run();

        // Verify first deployment address
        address payable expectedAddr = expectedRouterAddress();
        assertEq(expectedAddr.code.length > 0, true, "First deployment: TrailsRouter deployed");

        // Re-set the PRIVATE_KEY for second deployment
        vm.setEnv("PRIVATE_KEY", _deployerPkStr);

        // Second deployment should result in the same address (deterministic)
        vm.recordLogs();
        _deployScript.run();

        // Verify second deployment still has contract at same address
        assertEq(expectedAddr.code.length > 0, true, "Second deployment: TrailsRouter still deployed");
    }

    function test_DeployedRouter_HasCorrectConfiguration() public {
        vm.setEnv("PRIVATE_KEY", _deployerPkStr);

        // Deploy the script
        _deployScript.run();

        // Get reference to deployed contract
        address payable expectedAddr = expectedRouterAddress();
        TrailsRouter router = TrailsRouter(expectedAddr);

        // Verify contract is deployed and functional
        assertEq(address(router).code.length > 0, true, "Router should have code");

        // Test basic functionality - router should be able to receive calls
        // This is a smoke test to ensure the contract is properly deployed
        (bool success,) = address(router).call("");
        assertEq(success, true, "Router should accept basic calls");
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {SingletonDeployer, console} from "erc2470-libs/script/SingletonDeployer.s.sol";
import {TrailsRouter} from "../src/TrailsRouter.sol";

contract Deploy is SingletonDeployer {
    // -------------------------------------------------------------------------
    // Run
    // -------------------------------------------------------------------------

    function run() external {
        uint256 pk = vm.envUint("PRIVATE_KEY");
        address deployerAddress = vm.addr(pk);
        console.log("Deployer Address:", deployerAddress);

        address router = deployRouter(pk);
        console.log("TrailsRouter deployed at:", router);
    }

    // -------------------------------------------------------------------------
    // Deploy Router
    // -------------------------------------------------------------------------

    function deployRouter(uint256 pk) public returns (address) {
        bytes32 salt = bytes32(0);

        // Deploy TrailsRouter
        bytes memory initCode = type(TrailsRouter).creationCode;
        address router = _deployIfNotAlready("TrailsRouter", initCode, salt, pk);

        return router;
    }
}

