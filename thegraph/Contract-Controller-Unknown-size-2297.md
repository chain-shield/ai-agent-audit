
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-indexed-events, gas-small-strings
// solhint-disable named-parameters-mapping

/* solhint-disable gas-custom-errors */ // Cannot use custom errors with 0.7.6

import { IController } from "@graphprotocol/interfaces/contracts/contracts/governance/IController.sol";
import { IManaged } from "@graphprotocol/interfaces/contracts/contracts/governance/IManaged.sol";
import { Governed } from "./Governed.sol";
import { Pausable } from "./Pausable.sol";

/**
 * @title Graph Controller contract
 * @author Edge & Node
 * @notice Controller is a registry of contracts for convenience. Inspired by Livepeer:
 * https://github.com/livepeer/protocol/blob/streamflow/contracts/Controller.sol
 */
contract Controller is Governed, Pausable, IController {
    /// @dev Track contract ids to contract proxy address
    mapping(bytes32 => address) private _registry;

    /**
     * @notice Emitted when the proxy address for a protocol contract has been set
     * @param id Contract identifier
     * @param contractAddress Address of the contract proxy
     */
    event SetContractProxy(bytes32 indexed id, address contractAddress);

    /**
     * @notice Controller contract constructor.
     */
    constructor() {
        Governed._initialize(msg.sender);

        _setPaused(true);
    }

    /**
     * @dev Check if the caller is the governor or pause guardian.
     */
    modifier onlyGovernorOrGuardian() {
        require(msg.sender == governor || msg.sender == pauseGuardian, "Only Governor or Guardian can call");
        _;
    }

    /**
     * @inheritdoc IController
     */
    function getGovernor() external view override returns (address) {
        return governor;
    }

    // -- Registry --

    /**
     * @inheritdoc IController
     */
    function setContractProxy(bytes32 _id, address _contractAddress) external override onlyGovernor {
        require(_contractAddress != address(0), "Contract address must be set");
        _registry[_id] = _contractAddress;
        emit SetContractProxy(_id, _contractAddress);
    }

    /**
     * @inheritdoc IController
     */
    function unsetContractProxy(bytes32 _id) external override onlyGovernor {
        _registry[_id] = address(0);
        emit SetContractProxy(_id, address(0));
    }

    /**
     * @inheritdoc IController
     */
    function getContractProxy(bytes32 _id) external view override returns (address) {
        return _registry[_id];
    }

    /**
     * @inheritdoc IController
     */
    function updateController(bytes32 _id, address _controller) external override onlyGovernor {
        require(_controller != address(0), "Controller must be set");
        return IManaged(_registry[_id]).setController(_controller);
    }

    // -- Pausing --

    /**
     * @notice Change the partial paused state of the contract
     * Partial pause is intended as a partial pause of the protocol
     * @param _toPartialPause True if the contracts should be (partially) paused, false otherwise
     */
    function setPartialPaused(bool _toPartialPause) external override onlyGovernorOrGuardian {
        _setPartialPaused(_toPartialPause);
    }

    /**
     * @inheritdoc IController
     * @dev Full pause most of protocol functions
     */
    function setPaused(bool _toPause) external override onlyGovernorOrGuardian {
        _setPaused(_toPause);
    }

    /**
     * @inheritdoc IController
     */
    function setPauseGuardian(address _newPauseGuardian) external override onlyGovernor {
        require(_newPauseGuardian != address(0), "PauseGuardian must be set");
        _setPauseGuardian(_newPauseGuardian);
    }

    /**
     * @inheritdoc IController
     */
    function paused() external view override returns (bool) {
        return _paused;
    }

    /**
     * @inheritdoc IController
     */
    function partialPaused() external view override returns (bool) {
        return _partialPaused;
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
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

