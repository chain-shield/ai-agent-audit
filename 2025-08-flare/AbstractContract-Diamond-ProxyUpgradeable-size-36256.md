
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/******************************************************************************\
* Author: Nick Mudge <nick@perfectabstractions.com> (https://twitter.com/mudgen)
* EIP-2535 Diamonds: https://eips.ethereum.org/EIPS/eip-2535
*
* Implementation of a diamond.
/******************************************************************************/

import { LibDiamond } from "../library/LibDiamond.sol";

// When no function exists for function called
error FunctionNotFound(bytes4 _functionSelector);

// solhint-disable no-inline-assembly
abstract contract Diamond {
    // Find facet for function that is called and execute the
    // function if a facet is found and return any value.
    // solhint-disable-next-line no-complex-fallback
    fallback() external payable {
        LibDiamond.DiamondStorage storage ds;
        bytes32 position = LibDiamond.DIAMOND_STORAGE_POSITION;
        // get diamond storage
        assembly {
            ds.slot := position
        }
        // get facet from function selector
        address facet = ds.facetAddressAndSelectorPosition[msg.sig].facetAddress;
        if(facet == address(0)) {
            revert FunctionNotFound(msg.sig);
        }
        // Execute external function from facet using delegatecall and return any value.
        assembly {
            // copy function selector and any arguments
            calldatacopy(0, 0, calldatasize())
             // execute function call using the facet
            let result := delegatecall(gas(), facet, 0, calldatasize(), 0, 0)
            // get any return value
            returndatacopy(0, 0, returndatasize())
            // return any return value or error back to the caller
            switch result
                case 0 {
                    revert(0, returndatasize())
                }
                default {
                    return(0, returndatasize())
                }
        }
    }

    receive() external payable {}
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/******************************************************************************\
* Author: Nick Mudge <nick@perfectabstractions.com> (https://twitter.com/mudgen)
* EIP-2535 Diamonds: https://eips.ethereum.org/EIPS/eip-2535
/******************************************************************************/

interface IDiamond {
    enum FacetCutAction {Add, Replace, Remove}
    // Add=0, Replace=1, Remove=2

    struct FacetCut {
        address facetAddress;
        FacetCutAction action;
        bytes4[] functionSelectors;
    }

    event DiamondCut(FacetCut[] _diamondCut, address _init, bytes _calldata);
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/******************************************************************************\
* Author: Nick Mudge <nick@perfectabstractions.com> (https://twitter.com/mudgen)
* EIP-2535 Diamonds: https://eips.ethereum.org/EIPS/eip-2535
/******************************************************************************/
import { IDiamond } from "../interfaces/IDiamond.sol";
import { IDiamondCut } from "../interfaces/IDiamondCut.sol";

// Remember to add the loupe functions from DiamondLoupeFacet to the diamond.
// The loupe functions are required by the EIP2535 Diamonds standard

error NoSelectorsGivenToAdd();
error NotContractOwner(address _user, address _contractOwner);
error NoSelectorsProvidedForFacetForCut(address _facetAddress);
error CannotAddSelectorsToZeroAddress(bytes4[] _selectors);
error NoBytecodeAtAddress(address _contractAddress, string _message);
error IncorrectFacetCutAction(uint8 _action);
error CannotAddFunctionToDiamondThatAlreadyExists(bytes4 _selector);
error CannotReplaceFunctionsFromFacetWithZeroAddress(bytes4[] _selectors);
error CannotReplaceImmutableFunction(bytes4 _selector);
error CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(bytes4 _selector);
error CannotReplaceFunctionThatDoesNotExists(bytes4 _selector);
error RemoveFacetAddressMustBeZeroAddress(address _facetAddress);
error CannotRemoveFunctionThatDoesNotExist(bytes4 _selector);
error CannotRemoveImmutableFunction(bytes4 _selector);
error InitializationFunctionReverted(address _initializationContractAddress, bytes _calldata);

// solhint-disable no-inline-assembly
library LibDiamond {
    bytes32 internal constant DIAMOND_STORAGE_POSITION = keccak256("diamond.standard.diamond.storage");

    struct FacetAddressAndSelectorPosition {
        address facetAddress;
        uint16 selectorPosition;
    }

    struct DiamondStorage {
        // function selector => facet address and selector position in selectors array
        mapping(bytes4 => FacetAddressAndSelectorPosition) facetAddressAndSelectorPosition;
        bytes4[] selectors;
        mapping(bytes4 => bool) supportedInterfaces;
        // owner of the contract
        address contractOwner;
    }

    function diamondStorage() internal pure returns (DiamondStorage storage ds) {
        bytes32 position = DIAMOND_STORAGE_POSITION;
        assembly {
            ds.slot := position
        }
    }

    // Internal function version of diamondCut
    function diamondCut(
        IDiamondCut.FacetCut[] memory _diamondCut,
        address _init,
        bytes memory _calldata
    ) internal {
        for (uint256 facetIndex; facetIndex < _diamondCut.length; facetIndex++) {
            bytes4[] memory functionSelectors = _diamondCut[facetIndex].functionSelectors;
            address facetAddress = _diamondCut[facetIndex].facetAddress;
            if(functionSelectors.length == 0) {
                revert NoSelectorsProvidedForFacetForCut(facetAddress);
            }
            IDiamondCut.FacetCutAction action = _diamondCut[facetIndex].action;
            if (action == IDiamond.FacetCutAction.Add) {
                addFunctions(facetAddress, functionSelectors);
            } else if (action == IDiamond.FacetCutAction.Replace) {
                replaceFunctions(facetAddress, functionSelectors);
            } else if (action == IDiamond.FacetCutAction.Remove) {
                removeFunctions(facetAddress, functionSelectors);
            } else {
                revert IncorrectFacetCutAction(uint8(action));
            }
        }
        emit IDiamond.DiamondCut(_diamondCut, _init, _calldata);
        initializeDiamondCut(_init, _calldata);
    }

    function addFunctions(address _facetAddress, bytes4[] memory _functionSelectors) internal {
        if(_facetAddress == address(0)) {
            revert CannotAddSelectorsToZeroAddress(_functionSelectors);
        }
        DiamondStorage storage ds = diamondStorage();
        uint16 selectorCount = uint16(ds.selectors.length);
        enforceHasContractCode(_facetAddress, "LibDiamondCut: Add facet has no code");
        for (uint256 selectorIndex; selectorIndex < _functionSelectors.length; selectorIndex++) {
            bytes4 selector = _functionSelectors[selectorIndex];
            address oldFacetAddress = ds.facetAddressAndSelectorPosition[selector].facetAddress;
            if(oldFacetAddress != address(0)) {
                revert CannotAddFunctionToDiamondThatAlreadyExists(selector);
            }
            ds.facetAddressAndSelectorPosition[selector] =
                FacetAddressAndSelectorPosition(_facetAddress, selectorCount);
            ds.selectors.push(selector);
            selectorCount++;
        }
    }

    function replaceFunctions(address _facetAddress, bytes4[] memory _functionSelectors) internal {
        DiamondStorage storage ds = diamondStorage();
        if(_facetAddress == address(0)) {
            revert CannotReplaceFunctionsFromFacetWithZeroAddress(_functionSelectors);
        }
        enforceHasContractCode(_facetAddress, "LibDiamondCut: Replace facet has no code");
        for (uint256 selectorIndex; selectorIndex < _functionSelectors.length; selectorIndex++) {
            bytes4 selector = _functionSelectors[selectorIndex];
            address oldFacetAddress = ds.facetAddressAndSelectorPosition[selector].facetAddress;
            // can't replace immutable functions -- functions defined directly in the diamond in this case
            if(oldFacetAddress == address(this)) {
                revert CannotReplaceImmutableFunction(selector);
            }
            if(oldFacetAddress == _facetAddress) {
                revert CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(selector);
            }
            if(oldFacetAddress == address(0)) {
                revert CannotReplaceFunctionThatDoesNotExists(selector);
            }
            // replace old facet address
            ds.facetAddressAndSelectorPosition[selector].facetAddress = _facetAddress;
        }
    }

    function removeFunctions(address _facetAddress, bytes4[] memory _functionSelectors) internal {
        DiamondStorage storage ds = diamondStorage();
        uint256 selectorCount = ds.selectors.length;
        if(_facetAddress != address(0)) {
            revert RemoveFacetAddressMustBeZeroAddress(_facetAddress);
        }
        for (uint256 selectorIndex; selectorIndex < _functionSelectors.length; selectorIndex++) {
            bytes4 selector = _functionSelectors[selectorIndex];
            FacetAddressAndSelectorPosition memory oldFacetAddressAndSelectorPosition =
                ds.facetAddressAndSelectorPosition[selector];
            if(oldFacetAddressAndSelectorPosition.facetAddress == address(0)) {
                revert CannotRemoveFunctionThatDoesNotExist(selector);
            }

            // can't remove immutable functions -- functions defined directly in the diamond
            if(oldFacetAddressAndSelectorPosition.facetAddress == address(this)) {
                revert CannotRemoveImmutableFunction(selector);
            }
            // replace selector with last selector
            selectorCount--;
            if (oldFacetAddressAndSelectorPosition.selectorPosition != selectorCount) {
                bytes4 lastSelector = ds.selectors[selectorCount];
                ds.selectors[oldFacetAddressAndSelectorPosition.selectorPosition] = lastSelector;
                ds.facetAddressAndSelectorPosition[lastSelector].selectorPosition =
                    oldFacetAddressAndSelectorPosition.selectorPosition;
            }
            // delete last selector
            ds.selectors.pop();
            delete ds.facetAddressAndSelectorPosition[selector];
        }
    }

    function initializeDiamondCut(address _init, bytes memory _calldata) internal {
        if (_init == address(0)) {
            return;
        }
        enforceHasContractCode(_init, "LibDiamondCut: _init address has no code");
        // solhint-disable-next-line avoid-low-level-calls
        (bool success, bytes memory error) = _init.delegatecall(_calldata);
        if (!success) {
            if (error.length > 0) {
                // bubble up error
                /// @solidity memory-safe-assembly
                assembly {
                    let returndata_size := mload(error)
                    revert(add(32, error), returndata_size)
                }
            } else {
                revert InitializationFunctionReverted(_init, _calldata);
            }
        }
    }

    function enforceHasContractCode(address _contract, string memory _errorMessage) internal view {
        uint256 contractSize;
        assembly {
            contractSize := extcodesize(_contract)
        }
        if(contractSize == 0) {
            revert NoBytecodeAtAddress(_contract, _errorMessage);
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/******************************************************************************\
* Author: Nick Mudge <nick@perfectabstractions.com> (https://twitter.com/mudgen)
* EIP-2535 Diamonds: https://eips.ethereum.org/EIPS/eip-2535
/******************************************************************************/

import { IDiamond } from "./IDiamond.sol";

interface IDiamondCut is IDiamond {

    /// @notice Add/replace/remove any number of functions and optionally execute
    ///         a function with delegatecall
    /// @param _diamondCut Contains the facet addresses and function selectors
    /// @param _init The address of the contract or facet to execute _calldata
    /// @param _calldata A function call, including function selector and arguments
    ///                  _calldata is executed with delegatecall on _init
    function diamondCut(
        FacetCut[] calldata _diamondCut,
        address _init,
        bytes calldata _calldata
    ) external;
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";
import {IGoverned} from "../interfaces/IGoverned.sol";

/**
 * @title Governed Base
 * @notice This abstract base class defines behaviors for a governed contract.
 * @dev This class is abstract so that specific behaviors can be defined for the constructor.
 *   Contracts should not be left ungoverned, but not all contract will have a constructor
 *   (for example those pre-defined in genesis).
 * @dev This version is compatible with both Flare (where governance settings is in genesis at the address
 *   0x1000000000000000000000000000000000000007) and Songbird (where governance settings is a deployed contract).
 * @dev It also uses diamond storage for state, so it is safer tp use in diamond structures or proxies.
 **/
abstract contract GovernedBase is IGoverned {
    struct GovernedState {
        IGovernanceSettings governanceSettings;
        bool initialised;
        bool productionMode;
        bool executing;
        address initialGovernance;
        mapping(bytes32 encodedCallHash => uint256 allowedAfterTimestamp) timelockedCalls;
    }

    modifier onlyGovernance {
        if (_timeToExecute()) {
            _beforeExecute();
            _;
        } else {
            _recordTimelockedCall(msg.data, 0);
        }
    }

    modifier onlyGovernanceWithTimelockAtLeast(uint256 _minimumTimelock) {
        if (_timeToExecute()) {
            _beforeExecute();
            _;
        } else {
            _recordTimelockedCall(msg.data, _minimumTimelock);
        }
    }

    modifier onlyImmediateGovernance {
        _checkOnlyGovernance();
        _;
    }

    // solhint-disable-next-line no-empty-blocks
    constructor() {
    }

    /**
     * @notice Execute the timelocked governance calls once the timelock period expires.
     * @dev Only executor can call this method.
     * @param _encodedCall ABI encoded call data (signature and parameters).
     */
    function executeGovernanceCall(bytes calldata _encodedCall) external override {
        GovernedState storage state = _governedState();
        require(isExecutor(msg.sender), OnlyExecutor());
        bytes32 encodedCallHash = keccak256(_encodedCall);
        uint256 allowedAfterTimestamp = state.timelockedCalls[encodedCallHash];
        require(allowedAfterTimestamp != 0, TimelockInvalidSelector());
        require(block.timestamp >= allowedAfterTimestamp, TimelockNotAllowedYet());
        delete state.timelockedCalls[encodedCallHash];
        state.executing = true;
        //solhint-disable-next-line avoid-low-level-calls
        (bool success,) = address(this).call(_encodedCall);
        state.executing = false;
        emit TimelockedGovernanceCallExecuted(encodedCallHash);
        _passReturnOrRevert(success);
    }

    /**
     * Cancel a timelocked governance call before it has been executed.
     * @dev Only governance can call this method.
     * @param _encodedCall ABI encoded call data (signature and parameters).
     */
    function cancelGovernanceCall(bytes calldata _encodedCall) external override onlyImmediateGovernance {
        GovernedState storage state = _governedState();
        bytes32 encodedCallHash = keccak256(_encodedCall);
        require(state.timelockedCalls[encodedCallHash] != 0, TimelockInvalidSelector());
        emit TimelockedGovernanceCallCanceled(encodedCallHash);
        delete state.timelockedCalls[encodedCallHash];
    }

    /**
     * Enter the production mode after all the initial governance settings have been set.
     * This enables timelocks and the governance is afterwards obtained by calling
     * governanceSettings.getGovernanceAddress().
     */
    function switchToProductionMode() external onlyImmediateGovernance {
        GovernedState storage state = _governedState();
        require(!state.productionMode, AlreadyInProductionMode());
        state.initialGovernance = address(0);
        state.productionMode = true;
        emit GovernedProductionModeEntered(address(state.governanceSettings));
    }

    /**
     * @notice Initialize the governance address if not first initialized.
     */
    function initialise(IGovernanceSettings _governanceSettings, address _initialGovernance) internal virtual {
        GovernedState storage state = _governedState();
        require(state.initialised == false, GovernedAlreadyInitialized());
        require(address(_governanceSettings) != address(0), GovernedAddressZero());
        require(_initialGovernance != address(0), GovernedAddressZero());
        state.initialised = true;
        state.governanceSettings = _governanceSettings;
        state.initialGovernance = _initialGovernance;
        emit GovernanceInitialised(_initialGovernance);
    }

    /**
     * Returns the governance settings contract address.
     */
    function governanceSettings() public view returns (IGovernanceSettings) {
        return _governedState().governanceSettings;
    }

    /**
     * True after switching to production mode (see `switchToProductionMode()`).
     */
    function productionMode() public view returns (bool) {
        return _governedState().productionMode;
    }

    /**
     * Returns the current effective governance address.
     */
    function governance() public view returns (address) {
        GovernedState storage state = _governedState();
        return state.productionMode ? state.governanceSettings.getGovernanceAddress() : state.initialGovernance;
    }

    /**
     * Check if an address is one of the executors defined in governanceSettings.
     */
    function isExecutor(address _address) public view returns (bool) {
        GovernedState storage state = _governedState();
        return state.initialised && state.governanceSettings.isExecutor(_address);
    }

    function _beforeExecute() private {
        GovernedState storage state = _governedState();
        if (state.executing) {
            // can only be run from executeGovernanceCall(), where we check that only executor can call
            // make sure nothing else gets executed, even in case of reentrancy
            assert(msg.sender == address(this));
            state.executing = false;
        } else {
            // must be called with: productionMode=false
            // must check governance in this case
            _checkOnlyGovernance();
        }
    }

    function _recordTimelockedCall(bytes calldata _encodedCall, uint256 _minimumTimelock) private {
        GovernedState storage state = _governedState();
        _checkOnlyGovernance();
        bytes32 encodedCallHash = keccak256(_encodedCall);
        uint256 timelock = state.governanceSettings.getTimelock();
        if (timelock < _minimumTimelock) {
            timelock = _minimumTimelock;
        }
        uint256 allowedAt = block.timestamp + timelock;
        state.timelockedCalls[encodedCallHash] = allowedAt;
        emit GovernanceCallTimelocked(_encodedCall, encodedCallHash, allowedAt);
    }

    function _timeToExecute() private view returns (bool) {
        GovernedState storage state = _governedState();
        return state.executing || !state.productionMode;
    }

    function _checkOnlyGovernance() private view {
        require(msg.sender == governance(), OnlyGovernance());
    }

    function _governedState() private pure returns (GovernedState storage _state) {
        bytes32 position = keccak256("fasset.GovernedBase.GovernedState");
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _state.slot := position
        }
    }

    function _passReturnOrRevert(bool _success) private pure {
        // pass exact return or revert data - needs to be done in assembly
        //solhint-disable-next-line no-inline-assembly
        assembly {
            let size := returndatasize()
            let ptr := mload(0x40)
            mstore(0x40, add(ptr, size))
            returndatacopy(ptr, 0, size)
            if _success {
                return(ptr, size)
            }
            revert(ptr, size)
        }
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/******************************************************************************\
* Author: Nick Mudge <nick@perfectabstractions.com> (https://twitter.com/mudgen)
* EIP-2535 Diamonds: https://eips.ethereum.org/EIPS/eip-2535
/******************************************************************************/

import { Globals } from "../library/Globals.sol";
import { IDiamondCut } from "../../diamond/interfaces/IDiamondCut.sol";
import { LibDiamond } from "../../diamond/library/LibDiamond.sol";
import { GovernedProxyImplementation } from "../../governance/implementation/GovernedProxyImplementation.sol";

// DiamondCutFacet that also respects diamondCutMinTimelockSeconds setting.

// Remember to add the loupe functions from DiamondLoupeFacet to the diamond.
// The loupe functions are required by the EIP2535 Diamonds standard

contract AssetManagerDiamondCutFacet is IDiamondCut, GovernedProxyImplementation {
    /// @notice Add/replace/remove any number of functions and optionally execute
    ///         a function with delegatecall
    /// @param _diamondCut Contains the facet addresses and function selectors
    /// @param _init The address of the contract or facet to execute _calldata
    /// @param _calldata A function call, including function selector and arguments
    ///                  _calldata is executed with delegatecall on _init
    function diamondCut(
        FacetCut[] calldata _diamondCut,
        address _init,
        bytes calldata _calldata
    )
        external override
        onlyGovernanceWithTimelockAtLeast(Globals.getSettings().diamondCutMinTimelockSeconds)
    {
        LibDiamond.diamondCut(_diamondCut, _init, _calldata);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import { GovernedBase } from "./GovernedBase.sol";
import { IGovernanceSettings } from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";


/**
 * @title Governed
 * @dev For deployed, governed contracts, enforce non-zero addresses at create time.
 **/
abstract contract Governed is GovernedBase {
    constructor(IGovernanceSettings _governanceSettings, address _initialGovernance) {
        initialise(_governanceSettings, _initialGovernance);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IDiamondCut} from "../../diamond/interfaces/IDiamondCut.sol";
import {IGoverned} from "../../governance/interfaces/IGoverned.sol";
import {IAssetManager} from "../../userInterfaces/IAssetManager.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {IISettingsManagement} from "./IISettingsManagement.sol";
import {CollateralType} from "../../userInterfaces/data/CollateralType.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";


/**
 * Asset Manager methods used internally in AgentVault, CollateralPool and AssetManagerController.
 */
interface IIAssetManager is IAssetManager, IGoverned, IDiamondCut, IISettingsManagement {
    ////////////////////////////////////////////////////////////////////////////////////
    // Settings update

    /**
     * When `attached` is true, asset manager has been added to the asset manager controller.
     * Even though the asset manager controller address is set at the construction time, the manager may not
     * be able to be added to the controller immediately because the method addAssetManager must be called
     * by the governance multisig (with timelock). During this time it is impossible to verify through the
     * controller that the asset manager is legit.
     * Therefore creating agents and minting is disabled until the asset manager controller notifies
     * the asset manager that it has been added.
     * The `attached` can be set to false when the retired asset manager is removed from the controller.
     * NOTE: this method will be called automatically when the asset manager is added to a controller
     *      and cannot be called directly.
     */
    function attachController(bool attached) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Emergency pause

    /**
     * Trigger pause of most operations.
     */
    function emergencyPause(bool _byGovernance, uint256 _duration)
        external;

    /**
     * Reset total duration of 3rd party pauses, so that they can trigger pause again.
     * Otherwise, the total duration is automatically reset emergencyPauseDurationResetAfterSeconds after last pause.
     */
    function resetEmergencyPauseTotalDuration()
        external;

    /**
     * Emergency pause details, useful for monitors.
     */
    function emergencyPauseDetails()
        external view
        returns (uint256 _pausedUntil, uint256 _totalPauseDuration, bool _pausedByGovernance);

    ////////////////////////////////////////////////////////////////////////////////////
    // Emergency transfer pause

    /**
     * Trigger pause of most operations.
     */
    function emergencyPauseTransfers(bool _byGovernance, uint256 _duration)
        external;

    /**
     * Reset total duration of 3rd party pauses, so that they can trigger pause again.
     * Otherwise, the total duration is automatically reset emergencyPauseDurationResetAfterSeconds after last pause.
     */
    function resetEmergencyPauseTransfersTotalDuration()
        external;

    /**
     * Emergency pause details, useful for monitors.
     */
    function emergencyPauseTransfersDetails()
        external view
        returns (uint256 _pausedUntil, uint256 _totalPauseDuration, bool _pausedByGovernance);

    ////////////////////////////////////////////////////////////////////////////////////
    // Upgrade

    /**
     * When asset manager is paused, no new minting can be made.
     * All other operations continue normally.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function pauseMinting() external;

    /**
     * Minting can continue.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function unpauseMinting() external;

    /**
     * When agent vault, collateral pool or collateral pool token factory is upgraded, new agent vaults
     * automatically get the new implementation from the factory. The existing vaults can be batch updated
     * by this method.
     * Parameters `_start` and `_end` allow limiting the upgrades to a selection of all agents, to avoid
     * breaking the block gas limit.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     * @param _start the start index of the list of agent vaults (in getAllAgents()) to upgrade
     * @param _end the end index (exclusive) of the list of agent vaults to upgrade;
     *  can be larger then the number of agents, if gas is not an issue
     */
    function upgradeAgentVaultsAndPools(
        uint256 _start,
        uint256 _end
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Collateral type management

    /**
     * Add new vault collateral type (new token type and initial collateral ratios).
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function addCollateralType(
        CollateralType.Data calldata _data
    ) external;

    /**
     * Update collateral ratios for collateral type identified by `_collateralClass` and `_token`.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function setCollateralRatiosForToken(
        CollateralType.Class _collateralClass,
        IERC20 _token,
        uint256 _minCollateralRatioBIPS,
        uint256 _safetyMinCollateralRatioBIPS
    ) external;

    /**
     * Deprecate collateral type identified by `_collateralClass` and `_token`.
     * After `_invalidationTimeSec` the collateral will become invalid and all the agents
     * that still use it as collateral will be liquidated.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function deprecateCollateralType(
        CollateralType.Class _collateralClass,
        IERC20 _token,
        uint256 _invalidationTimeSec
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Collateral pool redemptions

    /**
     * Create a redemption from a single agent. Used in self-close exit from the collateral pool.
     * NOTE: only collateral pool can call this method.
     */
    function redeemFromAgent(
        address _agentVault,
        address _receiver,
        uint256 _amountUBA,
        string memory _receiverUnderlyingAddress,
        address payable _executor
    ) external payable;

    /**
     * Burn fassets from  a single agent and get paid in vault collateral by the agent.
     * Price is FTSO price, multiplied by factor buyFAssetByAgentFactorBIPS (set by agent).
     * Used in self-close exit from the collateral pool when requested or when self-close amount is less than 1 lot.
     * NOTE: only collateral pool can call this method.
     */
    function redeemFromAgentInCollateral(
        address _agentVault,
        address _receiver,
        uint256 _amountUBA
    ) external;

    /**
     * To avoid unlimited work, the maximum number of redemption tickets closed in redemption, self close
     * or liquidation is limited. This means that a single redemption/self close/liquidation is limited.
     * This function calculates the maximum single redemption amount.
     */
    function maxRedemptionFromAgent(address _agentVault)
        external view
        returns (uint256);

    ////////////////////////////////////////////////////////////////////////////////////
    // Functions, used by agent vault during collateral deposit/withdraw

    /**
     * Called by AgentVault when agent calls `withdraw()`.
     * NOTE: may only be called from an agent vault, not from an EOA address.
     * @param _valueNATWei the withdrawn amount
     */
    function beforeCollateralWithdrawal(
        IERC20 _token,
        uint256 _valueNATWei
    ) external;

    /**
     * Called by AgentVault when there was a deposit.
     * May pull agent out of liquidation.
     * NOTE: may only be called from an agent vault or collateral pool, not from an EOA address.
     */
    function updateCollateral(
        address _agentVault,
        IERC20 _token
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // View functions used internally by agent vault and collateral pool.

    /**
     * Get current WNat contract set in the asset manager.
     * Used internally by agent vault and collateral pool.
     * @return WNat contract
     */
    function getWNat()
        external view
        returns (IWNat);

    /**
     * Returns price of asset (UBA) in NAT Wei as a fraction.
     * Used internally by collateral pool.
     */
    function assetPriceNatWei()
        external view
        returns (uint256 _multiplier, uint256 _divisor);

    /**
     * Returns the number of f-assets that the agent's pool identified by `_agentVault` is backing.
     * This is the same as the number of f-assets the agent is backing, but excluding
     * f-assets being redeemed by pool self-close redemptions.
     * Used internally by collateral pool.
     */
    function getFAssetsBackedByPool(address _agentVault)
        external view
        returns (uint256);

    /**
     * Returns the duration for which the collateral pool tokens are timelocked after minting.
     * Timelocking is done to battle sandwich attacks aimed at stealing newly deposited f-asset
     * fees from the pool.
     */
    function getCollateralPoolTokenTimelockSeconds()
        external view
        returns (uint256);

    /**
     * Check if `_token` is either vault collateral token for `_agentVault` or the pool token.
     * These types of tokens cannot be simply transferred from the agent vault, but can only be
     * withdrawn after announcement if they are not backing any f-assets.
     * Used internally by agent vault.
     */
    function isLockedVaultToken(address _agentVault, IERC20 _token)
        external view
        returns (bool);

    /**
     * Check if `_token` is any of the vault collateral tokens (including already invalidated).
     */
    function isVaultCollateralToken(IERC20 _token)
        external view
        returns (bool);

    /**
     * True if `_address` is either work or management address of the owner of the agent identified by `_agentVault`.
     * Used internally by agent vault.
     */
    function isAgentVaultOwner(address _agentVault, address _address)
        external view
        returns (bool);

    /**
     * Return the work address for the given management address.
     */
    function getWorkAddress(address _managementAddress)
        external view
        returns (address);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {CoreVaultClient} from "../library/CoreVaultClient.sol";
import {IICoreVaultManager} from "../../coreVaultManager/interfaces/IICoreVaultManager.sol";
import {LibDiamond} from "../../diamond/library/LibDiamond.sol";
import {GovernedProxyImplementation} from "../../governance/implementation/GovernedProxyImplementation.sol";
import {ICoreVaultClientSettings} from "../../userInterfaces/ICoreVaultClientSettings.sol";
import {IAssetManager} from "../../userInterfaces/IAssetManager.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {ICoreVaultClient} from "../../userInterfaces/ICoreVaultClient.sol";
import {SafePct} from "../../utils/library/SafePct.sol";


contract CoreVaultClientSettingsFacet is AssetManagerBase, GovernedProxyImplementation, ICoreVaultClientSettings {
    using SafeCast for uint256;

    error WrongAssetManager();
    error CannotDisable();
    error DiamondNotInitialized();
    error AlreadyInitialized();
    error BipsValueTooHigh();

    // prevent initialization of implementation contract
    constructor() {
        CoreVaultClient.getState().initialized = true;
    }

    function initCoreVaultFacet(
        IICoreVaultManager _coreVaultManager,
        address payable _nativeAddress,
        uint256 _transferTimeExtensionSeconds,
        uint256 _redemptionFeeBIPS,
        uint256 _minimumAmountLeftBIPS,
        uint256 _minimumRedeemLots
    )
        external
    {
        updateInterfacesAtCoreVaultDeploy();
        // init settings
        require(_redemptionFeeBIPS <= SafePct.MAX_BIPS, BipsValueTooHigh());
        require(_minimumAmountLeftBIPS <= SafePct.MAX_BIPS, BipsValueTooHigh());
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        require(!state.initialized, AlreadyInitialized());
        state.initialized = true;
        state.coreVaultManager = _coreVaultManager;
        state.nativeAddress = _nativeAddress;
        state.transferTimeExtensionSeconds = _transferTimeExtensionSeconds.toUint64();
        state.redemptionFeeBIPS = _redemptionFeeBIPS.toUint16();
        state.minimumAmountLeftBIPS = _minimumAmountLeftBIPS.toUint16();
        state.minimumRedeemLots = _minimumRedeemLots.toUint64();
    }

    function updateInterfacesAtCoreVaultDeploy()
        public
    {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        require(ds.supportedInterfaces[type(IERC165).interfaceId], DiamondNotInitialized());
        // IAssetManager has new methods (at CoreVaultClient deploy on Songbird)
        ds.supportedInterfaces[type(IAssetManager).interfaceId] = true;
        // Core Vault interfaces added
        ds.supportedInterfaces[type(ICoreVaultClient).interfaceId] = true;
        ds.supportedInterfaces[type(ICoreVaultClientSettings).interfaceId] = true;
    }

    ///////////////////////////////////////////////////////////////////////////////////
    // Settings

    function setCoreVaultManager(
        address _coreVaultManager
    )
        external
        onlyGovernance
    {
        // core vault cannot be disabled once it has been enabled (it can be disabled initially
        // in initCoreVaultFacet method, for chains where core vault is not supported)
        require(_coreVaultManager != address(0), CannotDisable());
        IICoreVaultManager coreVaultManager = IICoreVaultManager(_coreVaultManager);
        require(coreVaultManager.assetManager() == address(this), WrongAssetManager());
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        state.coreVaultManager = coreVaultManager;
        emit IAssetManagerEvents.ContractChanged("coreVaultManager", _coreVaultManager);
    }

    function setCoreVaultNativeAddress(
        address payable _nativeAddress
    )
        external
        onlyImmediateGovernance
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        state.nativeAddress = _nativeAddress;
        // not really a contract, but works for any address - event name is a bit unfortunate
        // but we don't want to change it now to keep backward compatibility
        emit IAssetManagerEvents.ContractChanged("coreVaultNativeAddress", _nativeAddress);
    }

    function setCoreVaultTransferTimeExtensionSeconds(
        uint256 _transferTimeExtensionSeconds
    )
        external
        onlyImmediateGovernance
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        state.transferTimeExtensionSeconds = _transferTimeExtensionSeconds.toUint64();
        emit IAssetManagerEvents.SettingChanged("coreVaultTransferTimeExtensionSeconds",
            _transferTimeExtensionSeconds);
    }

    function setCoreVaultRedemptionFeeBIPS(
        uint256 _redemptionFeeBIPS
    )
        external
        onlyImmediateGovernance
    {
        require(_redemptionFeeBIPS <= SafePct.MAX_BIPS, BipsValueTooHigh());
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        state.redemptionFeeBIPS = _redemptionFeeBIPS.toUint16();
        emit IAssetManagerEvents.SettingChanged("coreVaultRedemptionFeeBIPS", _redemptionFeeBIPS);
    }

    function setCoreVaultMinimumAmountLeftBIPS(
        uint256 _minimumAmountLeftBIPS
    )
        external
        onlyImmediateGovernance
    {
        require(_minimumAmountLeftBIPS <= SafePct.MAX_BIPS, BipsValueTooHigh());
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        state.minimumAmountLeftBIPS = _minimumAmountLeftBIPS.toUint16();
        emit IAssetManagerEvents.SettingChanged("coreVaultMinimumAmountLeftBIPS", _minimumAmountLeftBIPS);
    }

    function setCoreVaultMinimumRedeemLots(
        uint256 _minimumRedeemLots
    )
        external
        onlyImmediateGovernance
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        state.minimumRedeemLots = _minimumRedeemLots.toUint64();
        emit IAssetManagerEvents.SettingChanged("coreVaultMinimumRedeemLots", _minimumRedeemLots);
    }

    function getCoreVaultManager()
        external view
        returns (address)
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        return address(state.coreVaultManager);
    }

    function getCoreVaultNativeAddress()
        external view
        returns (address)
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        return state.nativeAddress;
    }

    function getCoreVaultTransferTimeExtensionSeconds()
        external view
        returns (uint256)
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        return state.transferTimeExtensionSeconds;
    }

    function getCoreVaultRedemptionFeeBIPS()
        external view
        returns (uint256)
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        return state.redemptionFeeBIPS;
    }

    function getCoreVaultMinimumAmountLeftBIPS()
        external view
        returns (uint256)
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        return state.minimumAmountLeftBIPS;
    }

    function getCoreVaultMinimumRedeemLots()
        external view
        returns (uint256)
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        return state.minimumRedeemLots;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {IAssetManagerController} from "../../userInterfaces/IAssetManagerController.sol";
import {IAddressUpdatable} from "../../flareSmartContracts/interfaces/IAddressUpdatable.sol";
import {IUUPSUpgradeable} from "../../utils/interfaces/IUUPSUpgradeable.sol";
import {IIAssetManager} from "../../assetManager/interfaces/IIAssetManager.sol";
import {IGoverned} from "../../governance/interfaces/IGoverned.sol";
import {CollateralType} from "../../userInterfaces/data/CollateralType.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";



interface IIAssetManagerController is
    IERC165,
    IAssetManagerController,
    IGoverned,
    IAddressUpdatable,
    IUUPSUpgradeable
{
    /**
     * New address in case this controller was replaced.
     * Note: this code contains no checks that replacedBy==0, because when replaced,
     * all calls to AssetManager's updateSettings/pause will fail anyway
     * since they will arrive from wrong controller address.
     */
    function replacedBy() external view returns (address);

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Manage list of asset managers

    /**
     * Add an asset manager to this controller. The asset manager controller address in the settings of the
     * asset manager must match this. This method automatically marks the asset manager as attached.
     */
    function addAssetManager(IIAssetManager _assetManager)
        external;

    /**
     * Remove an asset manager from this controller, if it is attached to this controller.
     * The asset manager won't be attached any more, so it will be unusable.
     */
    function removeAssetManager(IIAssetManager _assetManager)
        external;

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Setters

    function setAgentOwnerRegistry(IIAssetManager[] memory _assetManagers, address _value)
        external;

    function setAgentVaultFactory(IIAssetManager[] memory _assetManagers, address _value)
        external;

    function setCollateralPoolFactory(IIAssetManager[] memory _assetManagers, address _value)
        external;

    function setCollateralPoolTokenFactory(IIAssetManager[] memory _assetManagers, address _value)
        external;

    function upgradeAgentVaultsAndPools(IIAssetManager[] memory _assetManagers, uint256 _start, uint256 _end)
        external;

    function setPriceReader(IIAssetManager[] memory _assetManagers, address _value)
        external;

    function setFdcVerification(IIAssetManager[] memory _assetManagers, address _value)
        external;

    function setCleanerContract(IIAssetManager[] memory _assetManagers, address _value)
        external;

    function setCleanupBlockNumberManager(IIAssetManager[] memory _assetManagers, address _value)
        external;

    // if callData is not empty, it is abi encoded call to init function in the new proxy implementation
    function upgradeFAssetImplementation(
        IIAssetManager[] memory _assetManagers,
        address _implementation,
        bytes memory _callData
    ) external;

    function setMinUpdateRepeatTimeSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setLotSizeAmg(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setTimeForPayment(
        IIAssetManager[] memory _assetManagers,
        uint256 _underlyingBlocks,
        uint256 _underlyingSeconds
    ) external;

    function setPaymentChallengeReward(
        IIAssetManager[] memory _assetManagers,
        uint256 _rewardVaultCollateralWei,
        uint256 _rewardBIPS
    ) external;

    function setMaxTrustedPriceAgeSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setCollateralReservationFeeBips(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setRedemptionFeeBips(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setRedemptionDefaultFactorVaultCollateralBIPS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setConfirmationByOthersAfterSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setConfirmationByOthersRewardUSD5(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setMaxRedeemedTickets(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setWithdrawalOrDestroyWaitMinSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setAttestationWindowSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setAverageBlockTimeMS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setMintingPoolHoldingsRequiredBIPS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setMintingCapAmg(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setTokenInvalidationTimeMinSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setVaultCollateralBuyForFlareFactorBIPS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setAgentExitAvailableTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setAgentFeeChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setAgentMintingCRChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setPoolExitCRChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setAgentTimelockedOperationWindowSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setCollateralPoolTokenTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setLiquidationStepSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setLiquidationPaymentFactors(
        IIAssetManager[] memory _assetManagers,
        uint256[] memory _paymentFactors,
        uint256[] memory _vaultCollateralFactors
    ) external;

    function setRedemptionPaymentExtensionSeconds(
        IIAssetManager[] memory _assetManagers,
        uint256 _value
    ) external;

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Collateral tokens

    function addCollateralType(
        IIAssetManager[] memory _assetManagers,
        CollateralType.Data calldata _data
    ) external;

    function setCollateralRatiosForToken(
        IIAssetManager[] memory _assetManagers,
        CollateralType.Class _class,
        IERC20 _token,
        uint256 _minCollateralRatioBIPS,
        uint256 _safetyMinCollateralRatioBIPS
    ) external;

    function deprecateCollateralType(
        IIAssetManager[] memory _assetManagers,
        CollateralType.Class _class,
        IERC20 _token,
        uint256 _invalidationTimeSec
    ) external;

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Upgrade (second phase)

    /**
     * When asset manager is paused, no new minting can be made.
     * All other operations continue normally.
     */
    function pauseMinting(IIAssetManager[] calldata _assetManagers)
        external;

    /**
     * Minting can continue.
     */
    function unpauseMinting(IIAssetManager[] calldata _assetManagers)
        external;

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Update contracts

    /**
     * Can be called to update address updater managed contracts if there are too many asset managers
     * to update in one block. In such a case, running AddressUpdater.updateContractAddresses will fail
     * and there will be no way to update contracts. This method allow the update to only change some
     * of the asset managers.
     */
    function updateContracts(IIAssetManager[] calldata _assetManagers)
        external;

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Emergency pause

    function emergencyPause(IIAssetManager[] memory _assetManagers, uint256 _duration)
        external;

    function emergencyPauseTransfers(IIAssetManager[] memory _assetManagers, uint256 _duration)
        external;

    function resetEmergencyPauseTotalDuration(IIAssetManager[] memory _assetManagers)
        external;

    function addEmergencyPauseSender(address _address)
        external;

    function removeEmergencyPauseSender(address _address)
        external;

    function setMaxEmergencyPauseDurationSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setEmergencyPauseDurationResetAfterSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {GovernedUUPSProxyImplementation} from "../../governance/implementation/GovernedUUPSProxyImplementation.sol";
import {AddressUpdatable} from "../../flareSmartContracts/implementation/AddressUpdatable.sol";
import {IICoreVaultManager} from "../interfaces/IICoreVaultManager.sol";
import {IFdcVerification, IPayment} from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";
import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";
import {GovernedBase} from "../../governance/implementation/GovernedBase.sol";
import {IIAddressUpdatable}
    from "@flarenetwork/flare-periphery-contracts/flare/addressUpdater/interfaces/IIAddressUpdatable.sol";

// import is needed for @inheritdoc
import {ICoreVaultManager} from "../../userInterfaces/ICoreVaultManager.sol"; // solhint-disable-line no-unused-import

//solhint-disable-next-line max-states-count
contract CoreVaultManager is
    GovernedUUPSProxyImplementation,
    AddressUpdatable,
    IICoreVaultManager,
    IERC165
{
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSet for EnumerableSet.Bytes32Set;

    /// asset manager address
    address public assetManager;
    /// chain id
    bytes32 public chainId;
    /// custodian address
    string public custodianAddress;
    /// core vault address hash
    bytes32 public coreVaultAddressHash;
    /// core vault address
    string public coreVaultAddress;
    /// next sequence number for core vault instructions
    uint256 public nextSequenceNumber;

    /// FDC verification contract
    IFdcVerification public fdcVerification;
    /// confirmed payments
    mapping(bytes32 transactionId => bool) public confirmedPayments;

    EnumerableSet.Bytes32Set private preimageHashes;
    Escrow[] private escrows;
    mapping(bytes32 preimageHash => uint256 escrowIndex) private preimageHashToEscrowIndex; // 1-based index

    /// index of a next preimage hash to be used for escrow
    uint256 public nextUnusedPreimageHashIndex;
    /// index of a next unprocessed escrow
    uint256 public nextUnprocessedEscrowIndex;

    uint256 private nextTransferRequestId;

    // NOTE: There is at most one cancelableTransferRequest per agent (an agent must cancel previous
    // return request before starting a new one). The number of agents cannot increase arbitrarily,
    // as agents that are allowed return are controlled by the governance. The total number will always be < ~10.
    // Therefore loops over cancelableTransferRequests are actually bounded and will not run out of gas.
    uint256[] private cancelableTransferRequests;

    // NOTE: The nonCancelableTransferRequests correspond to requests for direct core vault redemption.
    // The addresses to which the redemptions can be made are controlled by governance and the requests
    // to the same address get merged, so there will always be a limited number of requests (< ~10).
    // Therefore loops over nonCancelableTransferRequests are actually bounded and will not run out of gas.
    uint256[] private nonCancelableTransferRequests;

    mapping(uint256 transferRequestId => TransferRequest) private transferRequestById;

    // there will probably be no more than 10 destination addresses set in the system at any time
    string[] private allowedDestinationAddresses;
    mapping(string allowedDestinationAddress => uint256) private allowedDestinationAddressIndex; // 1-based index
    EnumerableSet.AddressSet private triggeringAccounts;
    EnumerableSet.AddressSet private emergencyPauseSenders;

    // settings
    /// escrow end time during a day in seconds (UTC time)
    uint128 private escrowEndTimeSeconds;
    /// amount to be escrowed
    uint128 private escrowAmount;
    /// minimal amount left in the core vault after escrowing
    uint128 private minimalAmount;
    /// fee
    uint128 private fee;

    /// available funds in the core vault
    uint128 public availableFunds;
    /// escrowed funds
    uint128 public escrowedFunds;
    /// cancelable transfer requests amount
    uint128 private cancelableTransferRequestsAmount;
    /// non-cancelable transfer requests amount
    uint128 private nonCancelableTransferRequestsAmount;

    /// paused state
    bool public paused;

    modifier onlyAssetManager() {
        _checkOnlyAssetManager();
        _;
    }

    modifier notPaused() {
        _checkNotPaused();
        _;
    }

    constructor()
        GovernedUUPSProxyImplementation()
        AddressUpdatable(address(0))
    {
    }

    /**
     * Proxyable initialization method. Can be called only once, from the proxy constructor
     * (single call is assured by GovernedBase.initialise).
     */
    function initialize(
        IGovernanceSettings _governanceSettings,
        address _initialGovernance,
        address _addressUpdater,
        address _assetManager,
        bytes32 _chainId,
        string memory _custodianAddress,
        string memory _coreVaultAddress,
        uint256 _nextSequenceNumber
    )
        external
    {
        require(_assetManager != address(0), InvalidAddress());
        require(_chainId != bytes32(0), InvalidChain());
        require(bytes(_custodianAddress).length > 0, InvalidAddress());
        require(bytes(_coreVaultAddress).length > 0, InvalidAddress());

        GovernedBase.initialise(_governanceSettings, _initialGovernance);
        AddressUpdatable.setAddressUpdaterValue(_addressUpdater);

        assetManager = _assetManager;
        chainId = _chainId;
        custodianAddress = _custodianAddress;
        coreVaultAddressHash = keccak256(bytes(_coreVaultAddress));
        coreVaultAddress = _coreVaultAddress;
        nextSequenceNumber = _nextSequenceNumber;
        emit CustodianAddressUpdated(_custodianAddress);
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function confirmPayment(
        IPayment.Proof calldata _proof
    )
        external
    {
        require(_proof.data.responseBody.status == 0, PaymentFailed()); // 0 = payment success
        require(_proof.data.sourceId == chainId, InvalidChain());
        require(fdcVerification.verifyPayment(_proof), PaymentNotProven());
        require(_proof.data.responseBody.receivingAddressHash == coreVaultAddressHash, NotCoreVault());
        require(_proof.data.responseBody.receivedAmount > 0, InvalidAmount());
        if (!confirmedPayments[_proof.data.requestBody.transactionId]) {
            uint128 receivedAmount = uint128(uint256(_proof.data.responseBody.receivedAmount));
            confirmedPayments[_proof.data.requestBody.transactionId] = true;
            availableFunds += receivedAmount;
            emit PaymentConfirmed(
                _proof.data.requestBody.transactionId,
                _proof.data.responseBody.standardPaymentReference,
                receivedAmount
            );
        }
    }

    /**
     * @inheritdoc IICoreVaultManager
     */
    function requestTransferFromCoreVault(
        string memory _destinationAddress,
        bytes32 _paymentReference,
        uint128 _amount,
        bool _cancelable
    )
        external
        onlyAssetManager notPaused
        returns (bytes32)
    {
        require(_amount > 0, AmountZero());
        require(allowedDestinationAddressIndex[_destinationAddress] != 0, DestinationNotAllowed());
        bytes32 destinationAddressHash = keccak256(bytes(_destinationAddress));
        bool newTransferRequest = false;
        if (_cancelable) {
            // only one cancelable request per destination address
            for (uint256 i = 0; i < cancelableTransferRequests.length; i++) {
                TransferRequest storage req = transferRequestById[cancelableTransferRequests[i]];
                require(keccak256(bytes(req.destinationAddress)) != destinationAddressHash, RequestExists());
            }
            cancelableTransferRequestsAmount += _amount;
            cancelableTransferRequests.push(nextTransferRequestId);
            newTransferRequest = true;
        } else {
            uint256 index = 0;
            while (index < nonCancelableTransferRequests.length) {
                TransferRequest storage req = transferRequestById[nonCancelableTransferRequests[index]];
                if (keccak256(bytes(req.destinationAddress)) == destinationAddressHash) {
                    // add the amount to the existing request
                    req.amount += _amount;
                    _paymentReference = req.paymentReference;   // use the old payment reference when merged
                    break;
                }
                index++;
            }
            nonCancelableTransferRequestsAmount += _amount;
            // if the request does not exist, add a new one
            if (index == nonCancelableTransferRequests.length) {
                nonCancelableTransferRequests.push(nextTransferRequestId);
                newTransferRequest = true;
            }
        }

        uint256 requestsAmount = totalRequestAmountWithFee();
        require(requestsAmount <= availableFunds + escrowedFunds, InsufficientFunds());

        if (newTransferRequest) {
            transferRequestById[nextTransferRequestId++] = TransferRequest({
                destinationAddress: _destinationAddress,
                paymentReference: _paymentReference,
                amount: _amount
            });
        }
        emit TransferRequested(_destinationAddress, _paymentReference, _amount, _cancelable);
        return _paymentReference;
    }

    /**
     * @inheritdoc IICoreVaultManager
     */
    function cancelTransferRequestFromCoreVault(
        string memory _destinationAddress
    )
        external
        onlyAssetManager
    {
        bytes32 destinationAddressHash = keccak256(bytes(_destinationAddress));
        uint256 index = 0;
        while (index < cancelableTransferRequests.length) {
            string memory destAddress = transferRequestById[cancelableTransferRequests[index]].destinationAddress;
            if (keccak256(bytes(destAddress)) == destinationAddressHash) {
                break;
            }
            index++;
        }
        require (index < cancelableTransferRequests.length, NotFound());
        uint256 transferRequestId = cancelableTransferRequests[index];
        TransferRequest storage req = transferRequestById[transferRequestId];
        uint128 amount = req.amount;
        cancelableTransferRequestsAmount -= amount;
        emit TransferRequestCanceled(_destinationAddress, req.paymentReference, amount);

        // remove the transfer request - keep the order
        while (index < cancelableTransferRequests.length - 1) { // length > 0
            cancelableTransferRequests[index] = cancelableTransferRequests[index + 1]; // shift left
            index++;
        }
        cancelableTransferRequests.pop(); // remove the last element
        delete transferRequestById[transferRequestId];
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function processEscrows(uint256 _maxCount) external returns (bool) {
        return _processEscrows(_maxCount);
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function triggerInstructions() external notPaused returns (uint256 _numberOfInstructions) {
        require(triggeringAccounts.contains(msg.sender), NotAuthorized());
        _processEscrows(type(uint256).max); // process all escrows
        uint128 availableFundsTmp = availableFunds;
        uint256 sequenceNumberTmp = nextSequenceNumber;

        // process cancelable transfer requests
        uint128 feeTmp = fee;
        require(feeTmp > 0, FeeZero());
        uint256 index = 0;
        uint256 length = cancelableTransferRequests.length;
        uint128 amountTmp = cancelableTransferRequestsAmount;
        while (index < length) {
            uint256 transferRequestId = cancelableTransferRequests[index];
            if (availableFundsTmp >= transferRequestById[transferRequestId].amount + feeTmp) {
                TransferRequest memory req = transferRequestById[transferRequestId];
                availableFundsTmp -= (req.amount + feeTmp);
                amountTmp -= req.amount;
                emit PaymentInstructions(
                    sequenceNumberTmp++,
                    coreVaultAddress,
                    req.destinationAddress,
                    req.amount,
                    feeTmp,
                    req.paymentReference
                );
                _numberOfInstructions++;
                // remove the transfer request - keep the order
                for (uint256 i = index; i < length - 1; i++) { // length > 0
                    cancelableTransferRequests[i] = cancelableTransferRequests[i + 1]; // shift left
                }
                cancelableTransferRequests.pop(); // remove the last element
                delete transferRequestById[transferRequestId];
                length--;
            } else {
                index++;
            }
        }
        cancelableTransferRequestsAmount = amountTmp;

        // process non-cancelable transfer requests
        index = 0;
        length = nonCancelableTransferRequests.length;
        amountTmp = nonCancelableTransferRequestsAmount;
        while (index < length) {
            uint256 transferRequestId = nonCancelableTransferRequests[index];
            if (availableFundsTmp >= transferRequestById[transferRequestId].amount + feeTmp) {
                TransferRequest memory req = transferRequestById[transferRequestId];
                availableFundsTmp -= (req.amount + feeTmp);
                amountTmp -= req.amount;
                emit PaymentInstructions(
                    sequenceNumberTmp++,
                    coreVaultAddress,
                    req.destinationAddress,
                    req.amount,
                    feeTmp,
                    req.paymentReference
                );
                _numberOfInstructions++;
                // remove the transfer request - keep the order
                for (uint256 i = index; i < length - 1; i++) { // length > 0
                    nonCancelableTransferRequests[i] = nonCancelableTransferRequests[i + 1]; // shift left
                }
                nonCancelableTransferRequests.pop(); // remove the last element
                delete transferRequestById[transferRequestId];
                length--;
            } else {
                index++;
            }
        }
        nonCancelableTransferRequestsAmount = amountTmp;

        uint128 escrowAmountTmp = escrowAmount;
        if (escrowAmountTmp == 0 || length > 0 || cancelableTransferRequests.length > 0) {
            // update the state but skip creating new escrows
            availableFunds = availableFundsTmp;
            nextSequenceNumber = sequenceNumberTmp;
            return _numberOfInstructions;
        }

        // create escrows
        uint256 preimageHashIndexTmp = nextUnusedPreimageHashIndex;
        uint256 minFundsToTriggerEscrow = minimalAmount + escrowAmountTmp + feeTmp;
        length = preimageHashes.length();
        amountTmp = escrowedFunds;
        if (availableFundsTmp >= minFundsToTriggerEscrow && preimageHashIndexTmp < length) {
            uint64 escrowEndTimestamp = _getNextEscrowEndTimestamp();
            while (availableFundsTmp >= minFundsToTriggerEscrow && preimageHashIndexTmp < length) {
                availableFundsTmp -= (escrowAmountTmp + feeTmp);
                amountTmp += escrowAmountTmp;
                bytes32 preimageHash = preimageHashes.at(preimageHashIndexTmp++);
                Escrow memory escrow = Escrow({
                    preimageHash: preimageHash,
                    amount: escrowAmountTmp,
                    expiryTs: escrowEndTimestamp,
                    finished: false
                });
                escrows.push(escrow);
                preimageHashToEscrowIndex[preimageHash] = escrows.length;
                emit EscrowInstructions(
                    sequenceNumberTmp++,
                    preimageHash,
                    coreVaultAddress,
                    custodianAddress,
                    escrowAmountTmp,
                    feeTmp,
                    escrowEndTimestamp
                );
                _numberOfInstructions++;
                // next escrow end timestamp
                escrowEndTimestamp += 1 days;
            }
            nextUnusedPreimageHashIndex = preimageHashIndexTmp;
        }

        // update the state
        availableFunds = availableFundsTmp;
        nextSequenceNumber = sequenceNumberTmp;
        escrowedFunds = amountTmp;
    }

    /**
     * Adds allowed destination addresses.
     * @param _allowedDestinationAddresses List of allowed destination addresses to add.
     * NOTE: may only be called by the governance.
     */
    function addAllowedDestinationAddresses(
        string[] calldata _allowedDestinationAddresses
    )
        external
        onlyGovernance
    {
        for (uint256 i = 0; i < _allowedDestinationAddresses.length; i++) {
            require(bytes(_allowedDestinationAddresses[i]).length > 0, InvalidAddress());
            if (allowedDestinationAddressIndex[_allowedDestinationAddresses[i]] != 0) {
                continue;
            }
            allowedDestinationAddresses.push(_allowedDestinationAddresses[i]);
            allowedDestinationAddressIndex[_allowedDestinationAddresses[i]] = allowedDestinationAddresses.length;
            emit AllowedDestinationAddressAdded(_allowedDestinationAddresses[i]);
        }
    }

    /**
     * Removes allowed destination addresses.
     * @param _allowedDestinationAddresses List of allowed destination addresses to remove.
     * NOTE: may only be called by the governance.
     */
    function removeAllowedDestinationAddresses(
        string[] calldata _allowedDestinationAddresses
    )
        external
        onlyGovernance
    {
        for (uint256 i = 0; i < _allowedDestinationAddresses.length; i++) {
            uint256 index = allowedDestinationAddressIndex[_allowedDestinationAddresses[i]];
            if (index == 0) {
                continue;
            }
            uint256 length = allowedDestinationAddresses.length;
            if (index < length) {
                string memory addressToMove = allowedDestinationAddresses[length - 1];
                allowedDestinationAddresses[index - 1] = addressToMove;
                allowedDestinationAddressIndex[addressToMove] = index;
            }
            allowedDestinationAddresses.pop();
            delete allowedDestinationAddressIndex[_allowedDestinationAddresses[i]];
            emit AllowedDestinationAddressRemoved(_allowedDestinationAddresses[i]);
        }
    }

    /**
     * Adds the triggering accounts.
     * @param _triggeringAccounts List of triggering accounts to add.
     * NOTE: may only be called by the governance.
     */
    function addTriggeringAccounts(
        address[] calldata _triggeringAccounts
    )
        external
        onlyGovernance
    {
        for (uint256 i = 0; i < _triggeringAccounts.length; i++) {
            if (triggeringAccounts.add(_triggeringAccounts[i])) {
                emit TriggeringAccountAdded(_triggeringAccounts[i]);
            }
        }
    }

    /**
     * Removes the triggering accounts.
     * @param _triggeringAccounts List of triggering accounts to remove.
     * NOTE: may only be called by the governance.
     */
    function removeTriggeringAccounts(
        address[] calldata _triggeringAccounts
    )
        external
        onlyGovernance
    {
        for (uint256 i = 0; i < _triggeringAccounts.length; i++) {
            if (triggeringAccounts.remove(_triggeringAccounts[i])) {
                emit TriggeringAccountRemoved(_triggeringAccounts[i]);
            }
        }
    }

    /**
     * Updates the custodian address.
     * @param _custodianAddress Custodian address.
     * NOTE: may only be called by the governance.
     */
    function updateCustodianAddress(
        string calldata _custodianAddress
    )
        external
        onlyGovernance
    {
        require(bytes(_custodianAddress).length > 0, InvalidAddress());
        custodianAddress = _custodianAddress;
        emit CustodianAddressUpdated(_custodianAddress);
    }

    /**
     * Updates the settings.
     * @param _escrowEndTimeSeconds Escrow end time in seconds.
     * @param _escrowAmount Escrow amount (setting to 0 will disable escrows).
     * @param _minimalAmount Minimal amount left in the core vault after escrow.
     * @param _fee Fee.
     * NOTE: may only be called by the governance.
     */
    function updateSettings(
        uint128 _escrowEndTimeSeconds,
        uint128 _escrowAmount,
        uint128 _minimalAmount,
        uint128 _fee
    )
        external
        onlyGovernance
    {
        require(_escrowEndTimeSeconds < 1 days, InvalidEndTime());
        require(_fee > 0, FeeZero());
        escrowEndTimeSeconds = _escrowEndTimeSeconds;
        escrowAmount = _escrowAmount;
        minimalAmount = _minimalAmount;
        fee = _fee;
        emit SettingsUpdated(_escrowEndTimeSeconds, _escrowAmount, _minimalAmount, _fee);
    }

    /**
     * Adds preimage hashes.
     * @param _preimageHashes List of preimage hashes.
     * NOTE: may only be called by the governance.
     */
    function addPreimageHashes(
        bytes32[] calldata _preimageHashes
    )
        external
        onlyImmediateGovernance
    {
        for (uint256 i = 0; i < _preimageHashes.length; i++) {
            require(_preimageHashes[i] != bytes32(0) && preimageHashes.add(_preimageHashes[i]),
                InvalidPreimageHash());
            emit PreimageHashAdded(_preimageHashes[i]);
        }
    }

    /**
     * Remove last unused preimage hashes.
     * @param _maxCount Maximum number of preimage hashes to remove.
     * NOTE: may only be called by the governance.
     */
    function removeUnusedPreimageHashes(
        uint256 _maxCount
    )
        external
        onlyImmediateGovernance
    {
        uint256 index = preimageHashes.length();
        while (_maxCount > 0 && index > nextUnusedPreimageHashIndex) {
            bytes32 preimageHash = preimageHashes.at(--index);
            preimageHashes.remove(preimageHash);
            _maxCount--;
            emit UnusedPreimageHashRemoved(preimageHash);
        }
    }

    /**
     * Sets escrows as finished.
     * @param _preimageHashes List of preimage hashes.
     * NOTE: may only be called by the governance.
     */
    function setEscrowsFinished(
        bytes32[] calldata _preimageHashes
    )
        external
        onlyImmediateGovernance
    {
        uint128 availableFundsTmp = availableFunds;
        uint128 escrowedFundsTmp = escrowedFunds;
        for (uint256 i = 0; i < _preimageHashes.length; i++) {
            uint256 escrowIndex = preimageHashToEscrowIndex[_preimageHashes[i]];
            Escrow storage escrow = _getEscrow(escrowIndex);
            require(!escrow.finished, EscrowAlreadyFinished());
            escrow.finished = true;
            if (escrowIndex <= nextUnprocessedEscrowIndex) {
                availableFundsTmp -= escrow.amount;
            } else {
                escrowedFundsTmp -= escrow.amount;
            }
            emit EscrowFinished(_preimageHashes[i], escrow.amount);
        }
        availableFunds = availableFundsTmp;
        escrowedFunds = escrowedFundsTmp;
    }

    /**
     * Adds emergency pause senders.
     * @param _addresses List of emergency pause senders to add.
     * NOTE: may only be called by the governance.
     */
    function addEmergencyPauseSenders(address[] calldata _addresses)
        external
        onlyImmediateGovernance
    {
        for (uint256 i = 0; i < _addresses.length; i++) {
            if(emergencyPauseSenders.add(_addresses[i])) {
                emit EmergencyPauseSenderAdded(_addresses[i]);
            }
        }
    }

    /**
     * Removes emergency pause senders.
     * @param _addresses List of emergency pause senders to remove.
     * NOTE: may only be called by the governance.
     */
    function removeEmergencyPauseSenders(address[] calldata _addresses)
        external
        onlyImmediateGovernance
    {
        for (uint256 i = 0; i < _addresses.length; i++) {
            if (emergencyPauseSenders.remove(_addresses[i])) {
                emit EmergencyPauseSenderRemoved(_addresses[i]);
            }
        }
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function pause() external {
        require(msg.sender == governance() || emergencyPauseSenders.contains(msg.sender), NotAuthorized());
        paused = true;
        emit Paused();
    }

    /**
     * Unpauses the contract. New transfer requests and instructions can be triggered.
     * NOTE: may only be called by the governance.
     */
    function unpause() external onlyImmediateGovernance {
        paused = false;
        emit Unpaused();
    }

    /**
     * Triggers custom instructions, which are not related to payment or escrow but increases the sequence number.
     * @param _instructionsHash Hash of the instructions send off-chain.
     * NOTE: may only be called by the governance.
     */
    function triggerCustomInstructions(bytes32 _instructionsHash) external onlyImmediateGovernance {
        emit CustomInstructions(
            nextSequenceNumber++,
            coreVaultAddress,
            _instructionsHash
        );
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getSettings()
        external view
        returns (
            uint128 _escrowEndTimeSeconds,
            uint128 _escrowAmount,
            uint128 _minimalAmount,
            uint128 _fee
        )
    {
        return (escrowEndTimeSeconds, escrowAmount, minimalAmount, fee);
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getAllowedDestinationAddresses() external view returns (string[] memory) {
        return allowedDestinationAddresses;
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function isDestinationAddressAllowed(string memory _address) external view returns (bool) {
        return allowedDestinationAddressIndex[_address] > 0;
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getTriggeringAccounts() external view returns (address[] memory) {
        return triggeringAccounts.values();
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getUnprocessedEscrows() external view returns (Escrow[] memory _unprocessedEscrows) {
        uint256 length = escrows.length - nextUnprocessedEscrowIndex;
        _unprocessedEscrows = new Escrow[](length);
        for (uint256 i = 0; i < length; i++) {
            _unprocessedEscrows[i] = escrows[nextUnprocessedEscrowIndex + i];
        }
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getEscrowsCount() external view returns (uint256) {
        return escrows.length;
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getEscrowByIndex(uint256 _index) external view returns (Escrow memory) {
        return escrows[_index];
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getEscrowByPreimageHash(bytes32 _preimageHash) external view returns (Escrow memory) {
        uint256 index = preimageHashToEscrowIndex[_preimageHash];
        return _getEscrow(index);
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getUnusedPreimageHashes() external view returns (bytes32[] memory) {
        uint256 length = preimageHashes.length() - nextUnusedPreimageHashIndex;
        bytes32[] memory unusedPreimageHashes = new bytes32[](length);
        for (uint256 i = 0; i < length; i++) {
            unusedPreimageHashes[i] = preimageHashes.at(nextUnusedPreimageHashIndex + i);
        }
        return unusedPreimageHashes;
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getPreimageHashesCount() external view returns (uint256) {
        return preimageHashes.length();
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getPreimageHash(uint256 _index) external view returns (bytes32) {
        return preimageHashes.at(_index);
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getCancelableTransferRequests() external view returns (TransferRequest[] memory _transferRequests) {
        _transferRequests = new TransferRequest[](cancelableTransferRequests.length);
        for (uint256 i = 0; i < cancelableTransferRequests.length; i++) {
            _transferRequests[i] = transferRequestById[cancelableTransferRequests[i]];
        }
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getNonCancelableTransferRequests() external view returns (TransferRequest[] memory _transferRequests) {
        _transferRequests = new TransferRequest[](nonCancelableTransferRequests.length);
        for (uint256 i = 0; i < nonCancelableTransferRequests.length; i++) {
            _transferRequests[i] = transferRequestById[nonCancelableTransferRequests[i]];
        }
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function totalRequestAmountWithFee() public view returns (uint256) {
        return nonCancelableTransferRequestsAmount + cancelableTransferRequestsAmount +
            (cancelableTransferRequests.length + nonCancelableTransferRequests.length) * fee;
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getEmergencyPauseSenders() external view returns (address[] memory) {
        return emergencyPauseSenders.values();
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // ERC 165

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        external pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(IIAddressUpdatable).interfaceId
            || _interfaceId == type(IICoreVaultManager).interfaceId;
    }

    /**
     * @inheritdoc AddressUpdatable
     */
    function _updateContractAddresses(
        bytes32[] memory _contractNameHashes,
        address[] memory _contractAddresses
    )
        internal override
    {
        fdcVerification = IFdcVerification(
            _getContractAddress(_contractNameHashes, _contractAddresses, "FdcVerification"));
    }

    /**
     * Processes the escrows.
     * @param _maxCount Maximum number of escrows to process.
     * @return _allProcessed True if all escrows were processed, false otherwise.
     */
    function _processEscrows(uint256 _maxCount) internal returns (bool _allProcessed) {
        uint128 availableFundsTmp = availableFunds;
        uint128 escrowedFundsTmp = escrowedFunds;
        // process all expired or finished escrows
        uint256 index = nextUnprocessedEscrowIndex;
        while (_maxCount > 0 && index < escrows.length &&
            (escrows[index].expiryTs <= block.timestamp || escrows[index].finished))
        {
            if (!escrows[index].finished) {
                // if the escrow is not finished, add the amount to the available funds
                Escrow storage escrow = escrows[index];
                uint128 amount = escrow.amount;
                availableFundsTmp += amount;
                escrowedFundsTmp -= amount;
                emit EscrowExpired(escrow.preimageHash, amount);
            }
            index++;
            _maxCount--;
        }
        // update the state
        nextUnprocessedEscrowIndex = index;
        availableFunds = availableFundsTmp;
        escrowedFunds = escrowedFundsTmp;

        _allProcessed = _maxCount > 0 || index == escrows.length ||
            (escrows[index].expiryTs > block.timestamp && !escrows[index].finished);
        if (!_allProcessed) {
            emit NotAllEscrowsProcessed();
        }
    }

    /**
     * Gets the escrow by index.
     * @param _index Escrow index (1-based).
     * @return Escrow.
     */
    function _getEscrow(uint256 _index) internal view returns (Escrow storage) {
        require(_index != 0, NotFound());
        return escrows[_index - 1];
    }

    /**
     * Gets the next escrow end timestamp.
     * @return Next escrow end timestamp.
     */
    function _getNextEscrowEndTimestamp() internal view returns (uint64) {
        uint256 escrowEndTimestamp = 0;
        // find the last unfinished escrow
        for (uint256 i = escrows.length; i > nextUnprocessedEscrowIndex; i--) {
            if (!escrows[i - 1].finished) {
                escrowEndTimestamp = escrows[i - 1].expiryTs;
                break;
            }
        }
        escrowEndTimestamp = Math.max(escrowEndTimestamp, block.timestamp);
        escrowEndTimestamp += 1 days;
        // slither-disable-next-line weak-prng
        escrowEndTimestamp = escrowEndTimestamp - (escrowEndTimestamp % 1 days) + escrowEndTimeSeconds;
        if (escrowEndTimestamp <= block.timestamp + 12 hours) { // less than 12 hours from now, move to the next day
            escrowEndTimestamp += 1 days;
        }
        return uint64(escrowEndTimestamp);
    }

    /**
     * Checks if the caller is the asset manager.
     */
    function _checkOnlyAssetManager() internal view {
        require(msg.sender == assetManager, OnlyAssetManager());
    }

    /**
     * Checks if the contract is not paused.
     */
    function _checkNotPaused() internal view {
        require(!paused, ContractPaused());
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {ReentrancyGuard} from "../../openzeppelin/security/ReentrancyGuard.sol";
import {CollateralTypes} from "../library/CollateralTypes.sol";
import {SettingsInitializer} from "../library/SettingsInitializer.sol";
import {IIAssetManager} from "../../assetManager/interfaces/IIAssetManager.sol";
import {IDiamondCut} from "../../diamond/interfaces/IDiamondCut.sol";
import {IDiamondLoupe} from "../../diamond/interfaces/IDiamondLoupe.sol";
import {LibDiamond} from "../../diamond/library/LibDiamond.sol";
import {IGoverned} from "../../governance/interfaces/IGoverned.sol";
import {GovernedBase} from "../../governance/implementation/GovernedBase.sol";
import {GovernedProxyImplementation} from "../../governance/implementation/GovernedProxyImplementation.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {CollateralType} from "../../userInterfaces/data/CollateralType.sol";
import {IAgentPing} from "../../userInterfaces/IAgentPing.sol";
import {IAssetManager} from "../../userInterfaces/IAssetManager.sol";


contract AssetManagerInit is GovernedProxyImplementation, ReentrancyGuard {
    error NotInitialized();

    function init(
        IGovernanceSettings _governanceSettings,
        address _initialGovernance,
        AssetManagerSettings.Data memory _settings,
        CollateralType.Data[] memory _initialCollateralTypes
    )
        external
    {
        GovernedBase.initialise(_governanceSettings, _initialGovernance);
        ReentrancyGuard.initializeReentrancyGuard();
        SettingsInitializer.validateAndSet(_settings);
        CollateralTypes.initialize(_initialCollateralTypes);
        _initIERC165();
    }

    /**
     * If a diamond cut adds methods to one of the declared interfaces, it should call this method in initialization.
     * In this way ERC165 identifiers for both old and new version of interface will be marked as supported,
     * which is correct since the new interface should be backward compatible with the old one.
     */
    function upgradeERC165Identifiers() external {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        require(ds.supportedInterfaces[type(IERC165).interfaceId], NotInitialized());
        ds.supportedInterfaces[type(IGoverned).interfaceId] = true;
        ds.supportedInterfaces[type(IAssetManager).interfaceId] = true;
        ds.supportedInterfaces[type(IIAssetManager).interfaceId] = true;
        ds.supportedInterfaces[type(IAgentPing).interfaceId] = true;
    }

    function _initIERC165() private {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        ds.supportedInterfaces[type(IERC165).interfaceId] = true;
        ds.supportedInterfaces[type(IDiamondLoupe).interfaceId] = true;
        ds.supportedInterfaces[type(IDiamondCut).interfaceId] = true;
        ds.supportedInterfaces[type(IGoverned).interfaceId] = true;
        ds.supportedInterfaces[type(IAssetManager).interfaceId] = true;
        ds.supportedInterfaces[type(IIAssetManager).interfaceId] = true;
        ds.supportedInterfaces[type(IAgentPing).interfaceId] = true;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IAgentOwnerRegistry} from "../../userInterfaces/IAgentOwnerRegistry.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {GovernedUUPSProxyImplementation} from "../../governance/implementation/GovernedUUPSProxyImplementation.sol";
import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";


contract AgentOwnerRegistry is GovernedUUPSProxyImplementation, IERC165, IAgentOwnerRegistry {

    event ManagerChanged(address manager);

    error AddressZero();
    error OnlyGovernanceOrManager();

    /**
     * When nonzero, this is the address that can perform whitelisting operations
     * instead of the governance.
     */
    address public manager;

    mapping(address => bool) private whitelist;

    mapping(address => address) private workToMgmtAddress;
    mapping(address => address) private mgmtToWorkAddress;

    mapping(address => string) private agentName;
    mapping(address => string) private agentDescription;
    mapping(address => string) private agentIconUrl;
    mapping(address => string) private agentTouUrl;

    modifier onlyGovernanceOrManager {
        require(msg.sender == manager || msg.sender == governance(), OnlyGovernanceOrManager());
        _;
    }

    function initialize(IGovernanceSettings _governanceSettings, address _initialGovernance) external {
        initialise(_governanceSettings, _initialGovernance);    // also marks as initialized
    }

    function revokeAddress(address _address) external onlyGovernanceOrManager {
        _removeAddressFromWhitelist(_address);
    }

    function setManager(address _manager) external onlyGovernance {
        manager = _manager;
        emit ManagerChanged(_manager);
    }

    /**
     * Add agent to the whitelist and set data for agent presentation.
     * If the agent is already whitelisted, only updates agent presentation data.
     * @param _managementAddress the agent owner's address
     * @param _name agent owner's name
     * @param _description agent owner's description
     * @param _iconUrl url of the agent owner's icon image; governance or manager should check it is in correct format
     *      and size and it is on a server where it cannot change or be deleted
     * @param _touUrl url of the agent's page with terms of use; similar considerations apply as for icon url
     */
    function whitelistAndDescribeAgent(
        address _managementAddress,
        string memory _name,
        string memory _description,
        string memory _iconUrl,
        string memory _touUrl
    )
        external
        onlyGovernanceOrManager
    {
        _addAddressToWhitelist(_managementAddress);
        _setAgentData(_managementAddress, _name, _description, _iconUrl, _touUrl);
    }

    /**
     * Associate a work address with the agent owner's management address.
     * Every owner (management address) can have only one work address, so as soon as the new one is set, the old
     * one stops working.
     * NOTE: May only be called by an agent on the allowed agent list and only from the management address.
     */
    function setWorkAddress(address _ownerWorkAddress)
        external
    {
        require(isWhitelisted(msg.sender), AgentNotWhitelisted());
        require(_ownerWorkAddress == address(0) || workToMgmtAddress[_ownerWorkAddress] == address(0),
               WorkAddressInUse());
        // delete old work to management mapping
        address oldWorkAddress = mgmtToWorkAddress[msg.sender];
        if (oldWorkAddress != address(0)) {
            workToMgmtAddress[oldWorkAddress] = address(0);
        }
        // create a new bidirectional mapping
        mgmtToWorkAddress[msg.sender] = _ownerWorkAddress;
        if (_ownerWorkAddress != address(0)) {
            workToMgmtAddress[_ownerWorkAddress] = msg.sender;
        }
        emit WorkAddressChanged(msg.sender, oldWorkAddress, _ownerWorkAddress);
    }

    /**
     * Set agent owner's name.
     * @param _managementAddress agent owner's management address
     * @param _name new agent owner's name
     */
    function setAgentName(address _managementAddress, string memory _name)
        external
        onlyGovernanceOrManager
    {
        agentName[_managementAddress] = _name;
        _emitDataChanged(_managementAddress);
    }

    /**
     * Set agent owner's description.
     * @param _managementAddress agent owner's management address
     * @param _description new agent owner's description
     */
    function setAgentDescription(address _managementAddress, string memory _description)
        external
        onlyGovernanceOrManager
    {
        agentDescription[_managementAddress] = _description;
        _emitDataChanged(_managementAddress);
    }

    /**
     * Set url of the agent owner's icon.
     * @param _managementAddress agent owner's management address
     * @param _iconUrl new url of the agent owner's icon
     */
    function setAgentIconUrl(address _managementAddress, string memory _iconUrl)
        external
        onlyGovernanceOrManager
    {
        agentIconUrl[_managementAddress] = _iconUrl;
        _emitDataChanged(_managementAddress);
    }

    /**
     * Set url of the agent's page with terms of use.
     * @param _managementAddress agent owner's management address
     * @param _touUrl new url of the agent's page with terms of use
     */
    function setAgentTermsOfUseUrl(address _managementAddress, string memory _touUrl)
        external
        onlyGovernanceOrManager
    {
        agentTouUrl[_managementAddress] = _touUrl;
        _emitDataChanged(_managementAddress);
    }

    /**
     * Return agent owner's name.
     * @param _managementAddress agent owner's management address
     */
    function getAgentName(address _managementAddress)
        external view override
        returns (string memory)
    {
        return agentName[_managementAddress];
    }

    /**
     * Return agent owner's description.
     * @param _managementAddress agent owner's management address
     */
    function getAgentDescription(address _managementAddress)
        external view override
        returns (string memory)
    {
        return agentDescription[_managementAddress];
    }

    /**
     * Return url of the agent owner's icon.
     * @param _managementAddress agent owner's management address
     */
    function getAgentIconUrl(address _managementAddress)
        external view override
        returns (string memory)
    {
        return agentIconUrl[_managementAddress];
    }

    /**
     * Return url of the agent's page with terms of use.
     * @param _managementAddress agent owner's management address
     */
    function getAgentTermsOfUseUrl(address _managementAddress)
        external view override
        returns (string memory)
    {
        return agentTouUrl[_managementAddress];
    }

    /**
     * Get the (unique) work address for the given management address.
     */
    function getWorkAddress(address _managementAddress)
        external view override
        returns (address)
    {
        return mgmtToWorkAddress[_managementAddress];
    }

    /**
     * Get the (unique) management address for the given work address.
     */
    function getManagementAddress(address _workAddress)
        external view override
        returns (address)
    {
        return workToMgmtAddress[_workAddress];
    }

    function isWhitelisted(address _address) public view override returns (bool) {
        return whitelist[_address];
    }

    function _addAddressToWhitelist(address _address) internal {
        require(_address != address(0), AddressZero());
        if (whitelist[_address]) return;
        whitelist[_address] = true;
        emit Whitelisted(_address);
    }

    function _removeAddressFromWhitelist(address _address) internal {
        if (!whitelist[_address]) return;
        delete whitelist[_address];
        emit WhitelistingRevoked(_address);
    }

    function _setAgentData(
        address _managementAddress,
        string memory _name,
        string memory _description,
        string memory _iconUrl,
        string memory _touUrl
    ) private {
        agentName[_managementAddress] = _name;
        agentDescription[_managementAddress] = _description;
        agentIconUrl[_managementAddress] = _iconUrl;
        agentTouUrl[_managementAddress] = _touUrl;
        emit AgentDataChanged(_managementAddress, _name, _description, _iconUrl, _touUrl);
    }

    function _emitDataChanged(address _managementAddress) private {
        emit AgentDataChanged(_managementAddress,
            agentName[_managementAddress],
            agentDescription[_managementAddress],
            agentIconUrl[_managementAddress],
            agentTouUrl[_managementAddress]);
    }

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        public pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(IAgentOwnerRegistry).interfaceId;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import { UUPSUpgradeable } from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import { GovernedProxyImplementation } from "./GovernedProxyImplementation.sol";
import { IUUPSUpgradeable } from "../../utils/interfaces/IUUPSUpgradeable.sol";

/**
 * Implementation of UUPS proxy that uses Flare governance with timelock.
 **/
abstract contract GovernedUUPSProxyImplementation is
    UUPSUpgradeable,
    GovernedProxyImplementation,
    IUUPSUpgradeable
{
    constructor()
        GovernedProxyImplementation()
    {}

    /**
     * See UUPSUpgradeable.upgradeTo
     */
    function upgradeTo(address newImplementation)
        public override (IUUPSUpgradeable, UUPSUpgradeable)
        onlyGovernance
        onlyProxy
    {
        _upgradeToAndCallUUPS(newImplementation, new bytes(0), false);
    }

    /**
     * See UUPSUpgradeable.upgradeToAndCall
     */
    function upgradeToAndCall(address newImplementation, bytes memory data)
        public payable override (IUUPSUpgradeable, UUPSUpgradeable)
        onlyGovernance
        onlyProxy
    {
        _upgradeToAndCallUUPS(newImplementation, data, true);
    }

    /**
     * Unused. Only present to satisfy UUPSUpgradeable requirement.
     * The real check is in onlyGovernance modifier on upgradeTo and upgradeToAndCall.
     */
    function _authorizeUpgrade(address  /* _newImplementation */)
        internal pure override
    {
        assert(false);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {IISettingsManagement} from "../interfaces/IISettingsManagement.sol";
import {CollateralTypes} from "../library/CollateralTypes.sol";
import {Globals} from "../library/Globals.sol";
import {SettingsUpdater} from "../library/SettingsUpdater.sol";
import {SettingsValidators} from "../library/SettingsValidators.sol";
import {IIFAsset} from "../../fassetToken/interfaces/IIFAsset.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {CollateralType} from "../../userInterfaces/data/CollateralType.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {IUpgradableProxy} from "../../utils/interfaces/IUpgradableProxy.sol";
import {SafePct} from "../../utils/library/SafePct.sol";


contract SettingsManagementFacet is AssetManagerBase, IAssetManagerEvents, IISettingsManagement {
    using SafeCast for uint256;
    using SafePct for uint256;

    error InvalidAddress();
    error CannotBeZero();
    error IncreaseTooBig();
    error DecreaseTooBig();
    error ValueTooSmall();
    error ValueTooBig();
    error FeeIncreaseTooBig();
    error FeeDecreaseTooBig();
    error LotSizeIncreaseTooBig();
    error LotSizeDecreaseTooBig();
    error LotSizeBiggerThanMintingCap();
    error BipsValueTooHigh();
    error BipsValueTooLow();
    error MustBeAtLeastTwoHours();
    error WindowTooSmall();
    error ConfirmationTimeTooBig();

    struct UpdaterState {
        mapping (bytes4 => uint256) lastUpdate;
    }

    bytes32 internal constant UPDATES_STATE_POSITION = keccak256("fasset.AssetManager.UpdaterState");

    modifier rateLimited() {
        SettingsUpdater.checkEnoughTimeSinceLastUpdate();
        _;
    }

    function updateSystemContracts(address _controller, IWNat _wNat)
        external
        onlyAssetManagerController
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // update assetManagerController
        if (settings.assetManagerController != _controller) {
            settings.assetManagerController = _controller;
            emit ContractChanged("assetManagerController", address(_controller));
        }
        // update wNat
        IWNat oldWNat = Globals.getWNat();
        if (oldWNat != _wNat) {
            CollateralType.Data memory data = CollateralTypes.getInfo(CollateralType.Class.POOL, oldWNat);
            data.validUntil = 0;
            data.token = _wNat;
            CollateralTypes.setPoolWNatCollateralType(data);
            emit ContractChanged("wNat", address(_wNat));
        }
    }

    function setAgentOwnerRegistry(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value != address(0), InvalidAddress());
        // update
        settings.agentOwnerRegistry = _value;
        emit ContractChanged("agentOwnerRegistry", _value);
    }

    function setAgentVaultFactory(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value != address(0), InvalidAddress());
        // update
        settings.agentVaultFactory = _value;
        emit ContractChanged("agentVaultFactory", _value);
    }

    function setCollateralPoolFactory(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value != address(0), InvalidAddress());
        // update
        settings.collateralPoolFactory = _value;
        emit ContractChanged("collateralPoolFactory", _value);
    }

    function setCollateralPoolTokenFactory(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value != address(0), InvalidAddress());
        // update
        settings.collateralPoolTokenFactory = _value;
        emit ContractChanged("collateralPoolTokenFactory", _value);
    }

    function setPriceReader(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value != address(0), InvalidAddress());
        // update
        settings.priceReader = _value;
        emit ContractChanged("priceReader", _value);
    }

    function setFdcVerification(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value != address(0), InvalidAddress());
        // update
        settings.fdcVerification = _value;
        emit IAssetManagerEvents.ContractChanged("fdcVerification", _value);
    }

    function setCleanerContract(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        IIFAsset fAsset = Globals.getFAsset();
        // validate
        // update
        fAsset.setCleanerContract(_value);
        emit ContractChanged("cleanerContract", _value);
    }

    function setCleanupBlockNumberManager(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        IIFAsset fAsset = Globals.getFAsset();
        // validate
        // update
        fAsset.setCleanupBlockNumberManager(_value);
        emit ContractChanged("cleanupBlockNumberManager", _value);
    }

    function upgradeFAssetImplementation(address _value, bytes memory callData)
        external
        onlyAssetManagerController
        rateLimited
    {
        IUpgradableProxy fAssetProxy = IUpgradableProxy(address(Globals.getFAsset()));
        // validate
        require(_value != address(0), InvalidAddress());
        // update
        if (callData.length > 0) {
            fAssetProxy.upgradeToAndCall(_value, callData);
        } else {
            fAssetProxy.upgradeTo(_value);
        }
        emit ContractChanged("fAsset", _value);
    }

    function setTimeForPayment(uint256 _underlyingBlocks, uint256 _underlyingSeconds)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_underlyingSeconds > 0, CannotBeZero());
        require(_underlyingBlocks > 0, CannotBeZero());
        SettingsValidators.validateTimeForPayment(_underlyingBlocks, _underlyingSeconds, settings.averageBlockTimeMS);
        // update
        settings.underlyingBlocksForPayment = _underlyingBlocks.toUint64();
        settings.underlyingSecondsForPayment = _underlyingSeconds.toUint64();
        emit SettingChanged("underlyingBlocksForPayment", _underlyingBlocks);
        emit SettingChanged("underlyingSecondsForPayment", _underlyingSeconds);
    }

    function setPaymentChallengeReward(uint256 _rewardNATWei, uint256 _rewardBIPS)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_rewardNATWei <= (settings.paymentChallengeRewardUSD5 * 4) + 100 ether, IncreaseTooBig());
        require(_rewardNATWei >= (settings.paymentChallengeRewardUSD5) / 4, DecreaseTooBig());
        require(_rewardBIPS <= (settings.paymentChallengeRewardBIPS * 4) + 100, IncreaseTooBig());
        require(_rewardBIPS >= (settings.paymentChallengeRewardBIPS) / 4, DecreaseTooBig());
        // update
        settings.paymentChallengeRewardUSD5 = _rewardNATWei.toUint128();
        settings.paymentChallengeRewardBIPS = _rewardBIPS.toUint16();
        emit SettingChanged("paymentChallengeRewardUSD5", _rewardNATWei);
        emit SettingChanged("paymentChallengeRewardBIPS", _rewardBIPS);
    }

    function setMinUpdateRepeatTimeSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        // update
        settings.minUpdateRepeatTimeSeconds = _value.toUint64();
        emit SettingChanged("minUpdateRepeatTimeSeconds", _value);
    }

    function setLotSizeAmg(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        // huge lot size increase is very dangerous, because it breaks redemption
        // (converts all tickets to dust)
        require(_value > 0, CannotBeZero());
        require(_value <= settings.lotSizeAMG * 10, LotSizeIncreaseTooBig());
        require(_value >= settings.lotSizeAMG / 10, LotSizeDecreaseTooBig());
        require(settings.mintingCapAMG == 0 || settings.mintingCapAMG >= _value,
            LotSizeBiggerThanMintingCap());
        // update
        settings.lotSizeAMG = _value.toUint64();
        emit SettingChanged("lotSizeAMG", _value);
    }

    function setMaxTrustedPriceAgeSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= settings.maxTrustedPriceAgeSeconds * 2, FeeIncreaseTooBig());
        require(_value >= settings.maxTrustedPriceAgeSeconds / 2, FeeDecreaseTooBig());
        // update
        settings.maxTrustedPriceAgeSeconds = _value.toUint64();
        emit SettingChanged("maxTrustedPriceAgeSeconds", _value);
    }

    function setCollateralReservationFeeBips(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= SafePct.MAX_BIPS, BipsValueTooHigh());
        require(_value <= settings.collateralReservationFeeBIPS * 4, FeeIncreaseTooBig());
        require(_value >= settings.collateralReservationFeeBIPS / 4, FeeDecreaseTooBig());
        // update
        settings.collateralReservationFeeBIPS = _value.toUint16();
        emit SettingChanged("collateralReservationFeeBIPS", _value);
    }

    function setRedemptionFeeBips(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= SafePct.MAX_BIPS, BipsValueTooHigh());
        require(_value <= settings.redemptionFeeBIPS * 4, FeeIncreaseTooBig());
        require(_value >= settings.redemptionFeeBIPS / 4, FeeDecreaseTooBig());
        // update
        settings.redemptionFeeBIPS = _value.toUint16();
        emit SettingChanged("redemptionFeeBIPS", _value);
    }

    function setRedemptionDefaultFactorVaultCollateralBIPS(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > SafePct.MAX_BIPS,
            BipsValueTooLow());
        require(_value <= uint256(settings.redemptionDefaultFactorVaultCollateralBIPS).mulBips(12000) + 1000,
            FeeIncreaseTooBig());
        require(_value >= uint256(settings.redemptionDefaultFactorVaultCollateralBIPS).mulBips(8333),
            FeeDecreaseTooBig());
        // update
        settings.redemptionDefaultFactorVaultCollateralBIPS = _value.toUint32();
        emit SettingChanged("redemptionDefaultFactorVaultCollateralBIPS", _value);
    }

    function setConfirmationByOthersAfterSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value >= 2 hours, MustBeAtLeastTwoHours());
        // update
        settings.confirmationByOthersAfterSeconds = _value.toUint64();
        emit SettingChanged("confirmationByOthersAfterSeconds", _value);
    }

    function setConfirmationByOthersRewardUSD5(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= settings.confirmationByOthersRewardUSD5 * 4, FeeIncreaseTooBig());
        require(_value >= settings.confirmationByOthersRewardUSD5 / 4, FeeDecreaseTooBig());
        // update
        settings.confirmationByOthersRewardUSD5 = _value.toUint128();
        emit SettingChanged("confirmationByOthersRewardUSD5", _value);
    }

    function setMaxRedeemedTickets(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= settings.maxRedeemedTickets * 2, IncreaseTooBig());
        require(_value >= settings.maxRedeemedTickets / 4, DecreaseTooBig());
        // update
        settings.maxRedeemedTickets = _value.toUint16();
        emit SettingChanged("maxRedeemedTickets", _value);
    }

    function setWithdrawalOrDestroyWaitMinSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        // making this _value small doesn't present huge danger, so we don't limit decrease
        require(_value > 0, CannotBeZero());
        require(_value <= settings.withdrawalWaitMinSeconds + 10 minutes, IncreaseTooBig());
        // update
        settings.withdrawalWaitMinSeconds = _value.toUint64();
        emit SettingChanged("withdrawalWaitMinSeconds", _value);
    }

    function setAttestationWindowSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value >= 1 days, WindowTooSmall());
        // update
        settings.attestationWindowSeconds = _value.toUint64();
        emit SettingChanged("attestationWindowSeconds", _value);
    }

    function setAverageBlockTimeMS(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= settings.averageBlockTimeMS * 2, IncreaseTooBig());
        require(_value >= settings.averageBlockTimeMS / 2, DecreaseTooBig());
        // update
        settings.averageBlockTimeMS = _value.toUint32();
        emit SettingChanged("averageBlockTimeMS", _value);
    }

    function setMintingPoolHoldingsRequiredBIPS(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value <= settings.mintingPoolHoldingsRequiredBIPS * 4 + SafePct.MAX_BIPS, ValueTooBig());
        // update
        settings.mintingPoolHoldingsRequiredBIPS = _value.toUint32();
        emit SettingChanged("mintingPoolHoldingsRequiredBIPS", _value);
    }

    function setMintingCapAmg(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value == 0 || _value >= settings.lotSizeAMG, ValueTooSmall());
        // update
        settings.mintingCapAMG = _value.toUint64();
        emit SettingChanged("mintingCapAMG", _value);
    }

    function setTokenInvalidationTimeMinSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        // update
        settings.tokenInvalidationTimeMinSeconds = _value.toUint64();
        emit SettingChanged("tokenInvalidationTimeMinSeconds", _value);
    }

    function setVaultCollateralBuyForFlareFactorBIPS(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value >= SafePct.MAX_BIPS, ValueTooSmall());
        // update
        settings.vaultCollateralBuyForFlareFactorBIPS = _value.toUint32();
        emit SettingChanged("vaultCollateralBuyForFlareFactorBIPS", _value);
    }

    function setAgentExitAvailableTimelockSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value <= settings.agentExitAvailableTimelockSeconds * 4 + 1 weeks, ValueTooBig());
        // update
        settings.agentExitAvailableTimelockSeconds = _value.toUint64();
        emit SettingChanged("agentExitAvailableTimelockSeconds", _value);
    }

    function setAgentFeeChangeTimelockSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value <= settings.agentFeeChangeTimelockSeconds * 4 + 1 days, ValueTooBig());
        // update
        settings.agentFeeChangeTimelockSeconds = _value.toUint64();
        emit SettingChanged("agentFeeChangeTimelockSeconds", _value);
    }

    function setAgentMintingCRChangeTimelockSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value <= settings.agentMintingCRChangeTimelockSeconds * 4 + 1 days, ValueTooBig());
        // update
        settings.agentMintingCRChangeTimelockSeconds = _value.toUint64();
        emit SettingChanged("agentMintingCRChangeTimelockSeconds", _value);
    }

    function setPoolExitCRChangeTimelockSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value <= settings.poolExitCRChangeTimelockSeconds * 4 + 1 days, ValueTooBig());
        // update
        settings.poolExitCRChangeTimelockSeconds = _value.toUint64();
        emit SettingChanged("poolExitCRChangeTimelockSeconds", _value);
    }

    function setAgentTimelockedOperationWindowSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value >= 1 hours, ValueTooSmall());
        // update
        settings.agentTimelockedOperationWindowSeconds = _value.toUint64();
        emit SettingChanged("agentTimelockedOperationWindowSeconds", _value);
    }

    function setCollateralPoolTokenTimelockSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value >= 1 minutes, ValueTooSmall());
        // update
        settings.collateralPoolTokenTimelockSeconds = _value.toUint32();
        emit SettingChanged("collateralPoolTokenTimelockSeconds", _value);
    }

    function setLiquidationStepSeconds(uint256 _stepSeconds)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_stepSeconds > 0, CannotBeZero());
        require(_stepSeconds <= settings.liquidationStepSeconds * 2, IncreaseTooBig());
        require(_stepSeconds >= settings.liquidationStepSeconds / 2, DecreaseTooBig());
        // update
        settings.liquidationStepSeconds = _stepSeconds.toUint64();
        emit SettingChanged("liquidationStepSeconds", _stepSeconds);
    }

    function setLiquidationPaymentFactors(
        uint256[] memory _liquidationFactors,
        uint256[] memory _vaultCollateralFactors
    )
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        SettingsValidators.validateLiquidationFactors(_liquidationFactors, _vaultCollateralFactors);
        // update
        delete settings.liquidationCollateralFactorBIPS;
        delete settings.liquidationFactorVaultCollateralBIPS;
        for (uint256 i = 0; i < _liquidationFactors.length; i++) {
            settings.liquidationCollateralFactorBIPS.push(_liquidationFactors[i].toUint32());
            settings.liquidationFactorVaultCollateralBIPS.push(_vaultCollateralFactors[i].toUint32());
        }
        // emit events
        emit SettingArrayChanged("liquidationCollateralFactorBIPS", _liquidationFactors);
        emit SettingArrayChanged("liquidationFactorVaultCollateralBIPS", _vaultCollateralFactors);
    }

    function setMaxEmergencyPauseDurationSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= settings.maxEmergencyPauseDurationSeconds * 4 + 60, IncreaseTooBig());
        require(_value >= settings.maxEmergencyPauseDurationSeconds / 4, DecreaseTooBig());
        // update
        settings.maxEmergencyPauseDurationSeconds = _value.toUint64();
        // emit events
        emit SettingChanged("maxEmergencyPauseDurationSeconds", _value);
    }

    function setEmergencyPauseDurationResetAfterSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= settings.emergencyPauseDurationResetAfterSeconds * 4 + 3600, IncreaseTooBig());
        require(_value >= settings.emergencyPauseDurationResetAfterSeconds / 4, DecreaseTooBig());
        // update
        settings.emergencyPauseDurationResetAfterSeconds = _value.toUint64();
        // emit events
        emit SettingChanged("emergencyPauseDurationResetAfterSeconds", _value);
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import { IGovernanceSettings, GovernedBase } from "./GovernedBase.sol";


/**
 * Base class for proxy implementations or diamond facets that expose governed methods -
 * prevents initialization of the implementation/facet as contract (to avoid selfdestruct by attackers).
 *
 * The GovernedBase.initialise can later be called only through a proxy. It should be
 * called through proxy constructor or in diamond cut initializer.
 **/
abstract contract GovernedProxyImplementation is GovernedBase {
    address private constant EMPTY_ADDRESS = 0x0000000000000000000000000000000000001111;

    // Mark as initialised and set governance to an invalid address.
    constructor() {
        initialise(IGovernanceSettings(EMPTY_ADDRESS), EMPTY_ADDRESS);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {MerkleProof} from "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";
import {IRelay} from "@flarenetwork/flare-periphery-contracts/flare/IRelay.sol";
import {GovernedUUPSProxyImplementation} from "../../governance/implementation/GovernedUUPSProxyImplementation.sol";
import {AddressUpdatable} from "../../flareSmartContracts/implementation/AddressUpdatable.sol";
import {IPriceReader} from "../../ftso/interfaces/IPriceReader.sol";
import {IPricePublisher} from "../interfaces/IPricePublisher.sol";
import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";


contract FtsoV2PriceStore is
    GovernedUUPSProxyImplementation,
    IPriceReader,
    IPricePublisher,
    IERC165,
    AddressUpdatable
{
    using MerkleProof for bytes32[];

    uint256 internal constant MAX_BIPS = 1e4;

    struct PriceStore {
        uint32 votingRoundId;
        uint32 value;
        int8 decimals;

        uint32 trustedVotingRoundId;
        uint32 trustedValue;
        int8 trustedDecimals;
        uint8 numberOfSubmits;
    }

    error InvalidStartTime();
    error VotingEpochDurationTooShort();
    error WrongNumberOfProofs();
    error PricesAlreadyPublished();
    error SubmissionWindowNotClosed();
    error VotingRoundIdMismatch();
    error FeedIdMismatch();
    error ValueMustBeNonNegative();
    error MerkleProofInvalid();
    error OnlyTrustedProvider();
    error AllPricesMustBeProvided();
    error SubmissionWindowClosed();
    error AlreadySubmitted();
    error DecimalsMismatch();
    error LengthMismatch();
    error MaxSpreadTooBig();
    error TooManyTrustedProviders();
    error ThresholdTooHigh();
    error SymbolNotSupported();

    /// Timestamp when the first voting epoch started, in seconds since UNIX epoch.
    uint64 public firstVotingRoundStartTs;
    /// Duration of voting epochs, in seconds.
    uint64 public votingEpochDurationSeconds;
    /// Duration of a window for submitting trusted prices, in seconds.
    uint64 public submitTrustedPricesWindowSeconds;
    /// The FTSO protocol id.
    uint8 public ftsoProtocolId;

    /// The list of required feed ids to be published.
    bytes21[] internal feedIds;
    /// Mapping from symbol to feed id - used for price lookups (backwards compatibility).
    mapping(string symbol => bytes21 feedId) internal symbolToFeedId;
    /// Mapping from feed id to symbol - used for list of supported symbols.
    mapping(bytes21 feedId => string symbol) internal feedIdToSymbol;
    /// Mapping from feed id to price store which holds the latest published FTSO scaling price and trusted price.
    mapping(bytes21 feedId => PriceStore) internal latestPrices;
    /// Mapping from feed id to submitted trusted prices for the given voting round.
    mapping(bytes21 feedId => mapping (uint32 votingRoundId => bytes)) internal submittedTrustedPrices;
    /// Mapping from trusted provider to the last submitted voting epoch id.
    mapping(address trustedProvider => uint256 lastVotingEpochId) internal lastVotingEpochIdByProvider;

    /// The list of trusted providers.
    address[] internal trustedProviders;
    mapping(address trustedProvider => bool isTrustedProvider) internal trustedProvidersMap;
    /// Trusted providers threshold for calculating the median price.
    uint8 public trustedProvidersThreshold;
    /// The maximum spread between the median price and the nearby trusted prices in BIPS in order to update the price.
    uint16 public maxSpreadBIPS;

    /// The Relay contract.
    IRelay public relay;
    /// The last published voting round id.
    uint32 public lastPublishedVotingRoundId;

    event PricesPublished(uint32 indexed votingRoundId);

    constructor()
        GovernedUUPSProxyImplementation()   // marks as initialized
        AddressUpdatable(address(0))
    {}

    function initialize(
        IGovernanceSettings _governanceSettings,
        address _initialGovernance,
        address _addressUpdater,
        uint64 _firstVotingRoundStartTs,
        uint8 _votingEpochDurationSeconds,
        uint8 _ftsoProtocolId
    )
        external
    {
        require(_firstVotingRoundStartTs + _votingEpochDurationSeconds <= block.timestamp, InvalidStartTime());
        require(_votingEpochDurationSeconds > 1, VotingEpochDurationTooShort()); // 90 s

        initialise(_governanceSettings, _initialGovernance);    // also marks as initialized
        setAddressUpdaterValue(_addressUpdater);
        firstVotingRoundStartTs = _firstVotingRoundStartTs;
        votingEpochDurationSeconds = _votingEpochDurationSeconds;
        submitTrustedPricesWindowSeconds = _votingEpochDurationSeconds / 2; // 45 s
        ftsoProtocolId = _ftsoProtocolId;
        lastPublishedVotingRoundId = _getPreviousVotingEpochId();
    }

    /**
     * @inheritdoc IPricePublisher
     */
    function publishPrices(FeedWithProof[] calldata _proofs) external {
        uint32 votingRoundId = 0;
        require(_proofs.length == feedIds.length, WrongNumberOfProofs());
        for (uint256 i = 0; i < _proofs.length; i++) {
            FeedWithProof calldata proof = _proofs[i];
            Feed calldata feed = proof.body;
            if (i == 0) {
                votingRoundId = feed.votingRoundId;
                require(votingRoundId > lastPublishedVotingRoundId, PricesAlreadyPublished());
                require(_getEndTimestamp(votingRoundId) + submitTrustedPricesWindowSeconds <= block.timestamp,
                    SubmissionWindowNotClosed());
                // update last published voting round id
                lastPublishedVotingRoundId = votingRoundId;
                // emit event
                emit PricesPublished(votingRoundId);
            } else {
                require(feed.votingRoundId == votingRoundId, VotingRoundIdMismatch());
            }
            bytes21 feedId = feedIds[i];
            require(feed.id == feedId, FeedIdMismatch());
            require(feed.value >= 0, ValueMustBeNonNegative());

            bytes32 feedHash = keccak256(abi.encode(feed));
            bytes32 merkleRoot = relay.merkleRoots(ftsoProtocolId, votingRoundId);
            require(proof.proof.verifyCalldata(merkleRoot, feedHash), MerkleProofInvalid());

            PriceStore storage priceStore = latestPrices[feedId];
            priceStore.votingRoundId = feed.votingRoundId;
            priceStore.value = uint32(feed.value);
            priceStore.decimals = feed.decimals;

            // calculate trusted prices for the same voting round
            bytes memory trustedPrices = submittedTrustedPrices[feedId][votingRoundId];
            if (trustedPrices.length > 0 && trustedPrices.length >= 4 * trustedProvidersThreshold) {
                // calculate median price
                (uint256 medianPrice, bool priceOk) = _calculateMedian(trustedPrices);
                if (priceOk) {
                    // store the median price
                    priceStore.trustedVotingRoundId = votingRoundId;
                    priceStore.trustedValue = uint32(medianPrice);
                    priceStore.numberOfSubmits = uint8(trustedPrices.length / 4);
                }
                // delete submitted trusted prices
                delete submittedTrustedPrices[feedId][votingRoundId];
            }
        }
    }

    /**
     * @inheritdoc IPricePublisher
     * @dev The function can be called by trusted providers only.
     */
    function submitTrustedPrices(uint32 _votingRoundId, TrustedProviderFeed[] calldata _feeds) external {
        require(trustedProvidersMap[msg.sender], OnlyTrustedProvider());
        require(_feeds.length == feedIds.length, AllPricesMustBeProvided());
        uint32 previousVotingEpochId = _getPreviousVotingEpochId();
        require(_votingRoundId == previousVotingEpochId, VotingRoundIdMismatch());
        // end of previous voting epoch = start of current voting epoch
        uint256 startTimestamp = _getEndTimestamp(previousVotingEpochId);
        uint256 endTimestamp = startTimestamp + submitTrustedPricesWindowSeconds;
        require(block.timestamp >= startTimestamp && block.timestamp < endTimestamp, SubmissionWindowClosed());
        require(lastVotingEpochIdByProvider[msg.sender] < previousVotingEpochId, AlreadySubmitted());
        // mark the trusted provider submission
        lastVotingEpochIdByProvider[msg.sender] = previousVotingEpochId;

        for (uint256 i = 0; i < _feeds.length; i++) {
            TrustedProviderFeed calldata feed = _feeds[i];
            bytes21 feedId = feedIds[i];
            require(feed.id == feedId, FeedIdMismatch());
            require(feed.decimals == latestPrices[feedId].trustedDecimals, DecimalsMismatch());
            submittedTrustedPrices[feedId][previousVotingEpochId] =
                bytes.concat(submittedTrustedPrices[feedId][previousVotingEpochId], bytes4(feed.value));
        }
    }

    /**
     * Updates the settings.
     * @param _feedIds The list of feed ids.
     * @param _symbols The list of symbols.
     * @param _trustedDecimals The list of trusted decimals.
     * @param _maxSpreadBIPS The maximum spread between the median price and the nearby trusted prices in BIPS.
     * @dev Can only be called by the governance.
     */
    function updateSettings(
        bytes21[] calldata _feedIds,
        string[] calldata _symbols,
        int8[] calldata _trustedDecimals,
        uint16 _maxSpreadBIPS
    )
        external onlyGovernance
    {
        require(_feedIds.length == _symbols.length && _feedIds.length == _trustedDecimals.length, LengthMismatch());
        require(_maxSpreadBIPS <= MAX_BIPS, MaxSpreadTooBig());
        maxSpreadBIPS = _maxSpreadBIPS;
        feedIds = _feedIds;
        for (uint256 i = 0; i < _feedIds.length; i++) {
            bytes21 feedId = _feedIds[i];
            symbolToFeedId[_symbols[i]] = feedId;
            feedIdToSymbol[feedId] = _symbols[i];
            PriceStore storage latestPrice = latestPrices[feedId];
            if (latestPrice.trustedDecimals != _trustedDecimals[i]) {
                latestPrice.trustedDecimals = _trustedDecimals[i];
                latestPrice.trustedValue = 0;
                latestPrice.trustedVotingRoundId = 0;
                // delete all submitted trusted prices for the symbol
                for (uint32 j = lastPublishedVotingRoundId + 1; j <= _getPreviousVotingEpochId(); j++) {
                    delete submittedTrustedPrices[feedId][j];
                }
            }
        }
    }

    /**
     * Sets the trusted providers.
     * @param _trustedProviders The list of trusted providers.
     * @param _trustedProvidersThreshold The trusted providers threshold for calculating the median price.
     * @dev Can only be called by the governance.
     */
    function setTrustedProviders(
        address[] calldata _trustedProviders,
        uint8 _trustedProvidersThreshold
    )
        external onlyGovernance
    {
        require(_trustedProviders.length < 2**8, TooManyTrustedProviders());
        require(_trustedProviders.length >= _trustedProvidersThreshold, ThresholdTooHigh());
        trustedProvidersThreshold = _trustedProvidersThreshold;
        // reset all trusted providers
        for (uint256 i = 0; i < trustedProviders.length; i++) {
            trustedProvidersMap[trustedProviders[i]] = false;
        }
        // set new trusted providers
        trustedProviders = _trustedProviders;
        for (uint256 i = 0; i < _trustedProviders.length; i++) {
            trustedProvidersMap[_trustedProviders[i]] = true;
        }
    }

    /**
     * @inheritdoc IPriceReader
     */
    function getPrice(string memory _symbol)
        external view
        returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
    {
        bytes21 feedId = symbolToFeedId[_symbol];
        require(feedId != bytes21(0), SymbolNotSupported());
        PriceStore storage feed = latestPrices[feedId];
        _price = feed.value;
        _timestamp = _getEndTimestamp(feed.votingRoundId);
        int256 decimals = feed.decimals; // int8
        if (decimals < 0) {
            _priceDecimals = 0;
            _price *= 10 ** uint256(-decimals);
        } else {
            _priceDecimals = uint256(decimals);
        }
    }

    /**
     * @inheritdoc IPriceReader
     */
    function getPriceFromTrustedProviders(string memory _symbol)
        external view
        returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
    {
        bytes21 feedId = symbolToFeedId[_symbol];
        require(feedId != bytes21(0), SymbolNotSupported());
        PriceStore storage feed = latestPrices[feedId];
        (_price, _timestamp, _priceDecimals) = _getPriceFromTrustedProviders(feed);
    }

    /**
     * @inheritdoc IPriceReader
     */
    function getPriceFromTrustedProvidersWithQuality(string memory _symbol)
        external view
        returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals, uint8 _numberOfSubmits)
    {
        bytes21 feedId = symbolToFeedId[_symbol];
        require(feedId != bytes21(0), SymbolNotSupported());
        PriceStore storage feed = latestPrices[feedId];
        (_price, _timestamp, _priceDecimals) = _getPriceFromTrustedProviders(feed);
        _numberOfSubmits = feed.numberOfSubmits;
    }

    /**
     * @inheritdoc IPricePublisher
     */
    function getFeedIds() external view returns (bytes21[] memory) {
        return feedIds;
    }

    /**
     * @inheritdoc IPricePublisher
     */
    function getFeedIdsWithDecimals() external view returns (bytes21[] memory _feedIds, int8[] memory _decimals) {
        _feedIds = feedIds;
        _decimals = new int8[](_feedIds.length);
        for (uint256 i = 0; i < _feedIds.length; i++) {
            _decimals[i] = latestPrices[_feedIds[i]].trustedDecimals;
        }
    }

    /**
     * @inheritdoc IPricePublisher
     */
    function getSymbols() external view returns (string[] memory _symbols) {
        _symbols = new string[](feedIds.length);
        for (uint256 i = 0; i < feedIds.length; i++) {
            _symbols[i] = feedIdToSymbol[feedIds[i]];
        }
    }

    /**
     * @inheritdoc IPricePublisher
     */
    function getFeedId(string memory _symbol) external view returns (bytes21) {
        return symbolToFeedId[_symbol];
    }

    /**
     * @inheritdoc IPricePublisher
     */
    function getTrustedProviders() external view returns (address[] memory) {
        return trustedProviders;
    }

    /**
     * @notice virtual method that a contract extending AddressUpdatable must implement
     */
    function _updateContractAddresses(
        bytes32[] memory _contractNameHashes,
        address[] memory _contractAddresses
    )
        internal override
    {
        relay = IRelay(_getContractAddress(_contractNameHashes, _contractAddresses, "Relay"));
    }

    /**
     * Returns the previous voting epoch id.
     */
    function _getPreviousVotingEpochId() internal view returns(uint32) {
        return uint32((block.timestamp - firstVotingRoundStartTs) / votingEpochDurationSeconds) - 1;
    }

    /**
     * Returns the end timestamp for the given voting epoch id.
     */
    function _getEndTimestamp(uint256 _votingEpochId) internal view returns(uint256) {
        return firstVotingRoundStartTs + (_votingEpochId + 1) * votingEpochDurationSeconds;
    }

    /**
     * Returns price data from trusted providers.
     */
    function _getPriceFromTrustedProviders(PriceStore storage _feed)
        internal view
        returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
    {
        _price = _feed.trustedValue;
        _timestamp = _getEndTimestamp(_feed.trustedVotingRoundId);
        int256 decimals = _feed.trustedDecimals; // int8
        if (decimals < 0) {
            _priceDecimals = 0;
            _price *= 10 ** uint256(-decimals);
        } else {
            _priceDecimals = uint256(decimals);
        }
    }

    /**
     * @notice Calculates the simple median price (using insertion sort) - sorts original array
     * @param _prices positional array of prices to be sorted
     * @return _medianPrice median price
     * @return _priceOk true if the median price is within the spread
     */
    function _calculateMedian(bytes memory _prices) internal view returns (uint256 _medianPrice, bool _priceOk) {
        uint256 length = _prices.length;
        assert(length > 0 && length % 4 == 0);
        length /= 4;
        uint256[] memory prices = new uint256[](length);
        for (uint256 i = 0; i < length; i++) {
            bytes memory price = new bytes(4);
            for (uint256 j = 0; j < 4; j++) {
                price[j] = _prices[i * 4 + j];
            }
            prices[i] = uint32(bytes4(price));
        }

        for (uint256 i = 1; i < length; i++) {
            // price to sort next
            uint256 currentPrice = prices[i];

            // shift bigger prices right
            uint256 j = i;
            while (j > 0 && prices[j - 1] > currentPrice) {
                prices[j] = prices[j - 1];
                j--; // no underflow
            }
            // insert
            prices[j] = currentPrice;
        }

        uint256 spread = 0;
        uint256 middleIndex = length / 2;
        if (length % 2 == 1) {
            _medianPrice = prices[middleIndex];
            if (length >= 3) {
                spread = (prices[middleIndex + 1] - prices[middleIndex - 1]) / 2;
            }
        } else {
            // if median is "in the middle", take the average price of the two consecutive prices
            _medianPrice = (prices[middleIndex - 1] + prices[middleIndex]) / 2;
            spread = prices[middleIndex] - prices[middleIndex - 1];
        }
        // check if spread is within the limit
        _priceOk = spread <= maxSpreadBIPS * _medianPrice / MAX_BIPS; // no overflow
    }

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        external pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(IPriceReader).interfaceId
            || _interfaceId == type(IPricePublisher).interfaceId;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {IIAddressUpdater}
    from "@flarenetwork/flare-periphery-contracts/flare/addressUpdater/interfaces/IIAddressUpdater.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {IISettingsManagement} from "../../assetManager/interfaces/IISettingsManagement.sol";
import {IIAssetManagerController} from "../interfaces/IIAssetManagerController.sol";
import {GovernedProxyImplementation} from "../../governance/implementation/GovernedProxyImplementation.sol";
import {AddressUpdatable} from "../../flareSmartContracts/implementation/AddressUpdatable.sol";
import {IIAssetManager} from "../../assetManager/interfaces/IIAssetManager.sol";
import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";
import {IAssetManager} from "../../userInterfaces/IAssetManager.sol";
import {IUUPSUpgradeable} from "../../utils/interfaces/IUUPSUpgradeable.sol";
import {CollateralType} from "../../userInterfaces/data/CollateralType.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IGoverned} from "../../governance/interfaces/IGoverned.sol";
import {IAssetManagerController} from "../../userInterfaces/IAssetManagerController.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {IIAddressUpdatable}
    from "@flarenetwork/flare-periphery-contracts/flare/addressUpdater/interfaces/IIAddressUpdatable.sol";
import {IAddressUpdatable} from "../../flareSmartContracts/interfaces/IAddressUpdatable.sol";
import {IRedemptionTimeExtension} from "../../userInterfaces/IRedemptionTimeExtension.sol";
import {GovernedBase} from "../../governance/implementation/GovernedBase.sol";

contract AssetManagerController is
    UUPSUpgradeable,
    GovernedProxyImplementation,
    AddressUpdatable,
    IIAssetManagerController
{
    using EnumerableSet for EnumerableSet.AddressSet;

    error AssetManagerNotManaged();
    error OnlyGovernanceOrEmergencyPauseSenders();
    error AddressZero();

    /**
     * New address in case this controller was replaced.
     * Note: this code contains no checks that replacedBy==0, because when replaced,
     * all calls to AssetManager's updateSettings/pause will fail anyway
     * since they will arrive from wrong controller address.
     */
    address public replacedBy;

    mapping(address => uint256) private assetManagerIndex;
    IIAssetManager[] private assetManagers;

    EnumerableSet.AddressSet private emergencyPauseSenders;

    constructor()
        GovernedProxyImplementation()
        AddressUpdatable(address(0))
    {
    }

    /**
     * Proxyable initialization method. Can be called only once, from the proxy constructor
     * (single call is assured by GovernedBase.initialise).
     */
    function initialize(
        IGovernanceSettings _governanceSettings,
        address _initialGovernance,
        address _addressUpdater
    )
        external
    {
        GovernedBase.initialise(_governanceSettings, _initialGovernance);
        AddressUpdatable.setAddressUpdaterValue(_addressUpdater);
    }

    /**
     * Add an asset manager to this controller. The asset manager controller address in the settings of the
     * asset manager must match this. This method automatically marks the asset manager as attached.
     */
    function addAssetManager(IIAssetManager _assetManager)
        external
        onlyGovernance
    {
        if (assetManagerIndex[address(_assetManager)] != 0) return;
        assetManagers.push(_assetManager);
        assetManagerIndex[address(_assetManager)] = assetManagers.length;  // 1+index, so that 0 means empty
        // have to check, otherwise it fails when the controller is replaced
        if (_assetManager.assetManagerController() == address(this)) {
            _assetManager.attachController(true);
        }
    }

    /**
     * Remove an asset manager from this controller, if it is attached to this controller.
     * The asset manager won't be attached any more, so it will be unusable.
     */
    function removeAssetManager(IIAssetManager _assetManager)
        external
        onlyGovernance
    {
        uint256 position = assetManagerIndex[address(_assetManager)];
        if (position == 0) return;
        uint256 index = position - 1;   // the real index, can be 0
        uint256 lastIndex = assetManagers.length - 1;
        if (index < lastIndex) {
            assetManagers[index] = assetManagers[lastIndex];
            assetManagerIndex[address(assetManagers[index])] = index + 1;
        }
        assetManagers.pop();
        assetManagerIndex[address(_assetManager)] = 0;
        // have to check, otherwise it fails when the controller is replaced
        if (_assetManager.assetManagerController() == address(this)) {
            _assetManager.attachController(false);
        }
    }

    /**
     * Return the list of all asset managers managed by this controller.
     */
    function getAssetManagers()
        external view
        returns (IAssetManager[] memory _assetManagers)
    {
        uint256 length = assetManagers.length;
        _assetManagers = new IAssetManager[](length);
        for (uint256 i = 0; i < length; i++) {
            _assetManagers[i] = assetManagers[i];
        }
    }

    /**
     * Check whether the asset manager is managed by this controller.
     * @param _assetManager an asset manager address
     */
    function assetManagerExists(address _assetManager)
        external view
        returns (bool)
    {
        return assetManagerIndex[_assetManager] != 0;
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // UUPS Proxy

    /**
     * See UUPSUpgradeable.upgradeTo
     */
    function upgradeTo(address newImplementation)
        public override (IUUPSUpgradeable, UUPSUpgradeable)
        onlyGovernance
        onlyProxy
    {
        _upgradeToAndCallUUPS(newImplementation, new bytes(0), false);
    }

    /**
     * See UUPSUpgradeable.upgradeToAndCall
     */
    function upgradeToAndCall(address newImplementation, bytes memory data)
        public payable override (IUUPSUpgradeable, UUPSUpgradeable)
        onlyGovernance
        onlyProxy
    {
        _upgradeToAndCallUUPS(newImplementation, data, true);
    }

    /**
     * Unused. Only present to satisfy UUPSUpgradeable requirement.
     * The real check is in onlyGovernance modifier on upgradeTo and upgradeToAndCall.
     */
    function _authorizeUpgrade(address /* _newImplementation */)
        internal pure override
    {
        assert(false);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Setters

    function setAgentOwnerRegistry(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAgentOwnerRegistry.selector, _value);
    }

    function setAgentVaultFactory(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAgentVaultFactory.selector, _value);
    }

    function setCollateralPoolFactory(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setCollateralPoolFactory.selector, _value);
    }

    function setCollateralPoolTokenFactory(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setCollateralPoolTokenFactory.selector, _value);
    }

    function upgradeAgentVaultsAndPools(IIAssetManager[] memory _assetManagers, uint256 _start, uint256 _end)
        external
        onlyImmediateGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.upgradeAgentVaultsAndPools, (_start, _end)));
    }

    function setPriceReader(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setPriceReader.selector, _value);
    }

    function setFdcVerification(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setFdcVerification.selector, _value);
    }

    function setCleanerContract(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setCleanerContract.selector, _value);
    }

    function setCleanupBlockNumberManager(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setCleanupBlockNumberManager.selector, _value);
    }

    // if callData is not empty, it is abi encoded call to init function in the new proxy implementation
    function upgradeFAssetImplementation(
        IIAssetManager[] memory _assetManagers,
        address _implementation,
        bytes memory _callData
    )
        external
        onlyGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IISettingsManagement.upgradeFAssetImplementation, (_implementation, _callData)));
    }

    function setMinUpdateRepeatTimeSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setMinUpdateRepeatTimeSeconds.selector, _value);
    }

    function setLotSizeAmg(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setLotSizeAmg.selector, _value);
    }

    function setTimeForPayment(
        IIAssetManager[] memory _assetManagers,
        uint256 _underlyingBlocks,
        uint256 _underlyingSeconds
    )
        external
        onlyGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IISettingsManagement.setTimeForPayment, (_underlyingBlocks, _underlyingSeconds)));
    }

    function setPaymentChallengeReward(
        IIAssetManager[] memory _assetManagers,
        uint256 _rewardVaultCollateralWei,
        uint256 _rewardBIPS
    )
        external
        onlyImmediateGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IISettingsManagement.setPaymentChallengeReward, (_rewardVaultCollateralWei, _rewardBIPS)));
    }

    function setMaxTrustedPriceAgeSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setMaxTrustedPriceAgeSeconds.selector, _value);
    }

    function setCollateralReservationFeeBips(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setCollateralReservationFeeBips.selector, _value);
    }

    function setRedemptionFeeBips(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setRedemptionFeeBips.selector, _value);
    }

    function setRedemptionDefaultFactorVaultCollateralBIPS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setRedemptionDefaultFactorVaultCollateralBIPS.selector, _value);
    }

    function setConfirmationByOthersAfterSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setConfirmationByOthersAfterSeconds.selector, _value);
    }

    function setConfirmationByOthersRewardUSD5(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setConfirmationByOthersRewardUSD5.selector, _value);
    }

    function setMaxRedeemedTickets(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setMaxRedeemedTickets.selector, _value);
    }

    function setWithdrawalOrDestroyWaitMinSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setWithdrawalOrDestroyWaitMinSeconds.selector, _value);
    }

    function setAttestationWindowSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAttestationWindowSeconds.selector, _value);
    }

    function setAverageBlockTimeMS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAverageBlockTimeMS.selector, _value);
    }

    function setMintingPoolHoldingsRequiredBIPS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setMintingPoolHoldingsRequiredBIPS.selector, _value);
    }

    function setMintingCapAmg(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setMintingCapAmg.selector, _value);
    }

    function setTokenInvalidationTimeMinSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setTokenInvalidationTimeMinSeconds.selector, _value);
    }

    function setVaultCollateralBuyForFlareFactorBIPS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setVaultCollateralBuyForFlareFactorBIPS.selector, _value);
    }

    function setAgentExitAvailableTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAgentExitAvailableTimelockSeconds.selector, _value);
    }

    function setAgentFeeChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAgentFeeChangeTimelockSeconds.selector, _value);
    }

    function setAgentMintingCRChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAgentMintingCRChangeTimelockSeconds.selector, _value);
    }

    function setPoolExitCRChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setPoolExitCRChangeTimelockSeconds.selector, _value);
    }

    function setAgentTimelockedOperationWindowSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAgentTimelockedOperationWindowSeconds.selector, _value);
    }

    function setCollateralPoolTokenTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setCollateralPoolTokenTimelockSeconds.selector, _value);
    }

    function setLiquidationStepSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setLiquidationStepSeconds.selector, _value);
    }

    function setLiquidationPaymentFactors(
        IIAssetManager[] memory _assetManagers,
        uint256[] memory _paymentFactors,
        uint256[] memory _vaultCollateralFactors
    )
        external
        onlyGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IISettingsManagement.setLiquidationPaymentFactors,
                (_paymentFactors, _vaultCollateralFactors)));
    }

    function setRedemptionPaymentExtensionSeconds(
        IIAssetManager[] memory _assetManagers,
        uint256 _value
    )
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IRedemptionTimeExtension.setRedemptionPaymentExtensionSeconds.selector, _value);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Collateral tokens

    function addCollateralType(
        IIAssetManager[] memory _assetManagers,
        CollateralType.Data calldata _data
    )
        external
        onlyImmediateGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.addCollateralType, (_data)));
    }

    function setCollateralRatiosForToken(
        IIAssetManager[] memory _assetManagers,
        CollateralType.Class _class,
        IERC20 _token,
        uint256 _minCollateralRatioBIPS,
        uint256 _safetyMinCollateralRatioBIPS
    )
        external
        onlyGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.setCollateralRatiosForToken,
                (_class, _token, _minCollateralRatioBIPS, _safetyMinCollateralRatioBIPS)));
    }

    function deprecateCollateralType(
        IIAssetManager[] memory _assetManagers,
        CollateralType.Class _class,
        IERC20 _token,
        uint256 _invalidationTimeSec
    )
        external
        onlyImmediateGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.deprecateCollateralType, (_class, _token, _invalidationTimeSec)));
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Upgrade (second phase)

    /**
     * When asset manager is paused, no new minting can be made.
     * All other operations continue normally.
     */
    function pauseMinting(IIAssetManager[] calldata _assetManagers)
        external
        onlyImmediateGovernance
    {
        _callOnManagers(_assetManagers, abi.encodeCall(IIAssetManager.pauseMinting, ()));
    }

    /**
     * Minting can continue.
     */
    function unpauseMinting(IIAssetManager[] calldata _assetManagers)
        external
        onlyImmediateGovernance
    {
        _callOnManagers(_assetManagers, abi.encodeCall(IIAssetManager.unpauseMinting, ()));
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // ERC 165

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        external pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(IAddressUpdatable).interfaceId
            || _interfaceId == type(IIAddressUpdatable).interfaceId
            || _interfaceId == type(IAssetManagerController).interfaceId
            || _interfaceId == type(IIAssetManagerController).interfaceId
            || _interfaceId == type(IGoverned).interfaceId;
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Update contracts

    /**
     * Can be called to update address updater managed contracts if there are too many asset managers
     * to update in one block. In such a case, running AddressUpdater.updateContractAddresses will fail
     * and there will be no way to update contracts. This method allow the update to only change some
     * of the asset managers.
     */
    function updateContracts(IIAssetManager[] calldata _assetManagers)
        external
    {
        // read contract addresses
        IIAddressUpdater addressUpdater = IIAddressUpdater(getAddressUpdater());
        address newAddressUpdater = addressUpdater.getContractAddress("AddressUpdater");
        address assetManagerController = addressUpdater.getContractAddress("AssetManagerController");
        address wNat = addressUpdater.getContractAddress("WNat");
        require(newAddressUpdater != address(0) && assetManagerController != address(0) && wNat != address(0),
            AddressZero());
        _updateContracts(_assetManagers, newAddressUpdater, assetManagerController, wNat);
    }

    // called by AddressUpdater.update or AddressUpdater.updateContractAddresses
    function _updateContractAddresses(
        bytes32[] memory _contractNameHashes,
        address[] memory _contractAddresses
    )
        internal override
    {
        address addressUpdater =
            _getContractAddress(_contractNameHashes, _contractAddresses, "AddressUpdater");
        address assetManagerController =
            _getContractAddress(_contractNameHashes, _contractAddresses, "AssetManagerController");
        address wNat =
            _getContractAddress(_contractNameHashes, _contractAddresses, "WNat");
        _updateContracts(assetManagers, addressUpdater, assetManagerController, wNat);
    }

    function _updateContracts(
        IIAssetManager[] memory _assetManagers,
        address addressUpdater,
        address assetManagerController,
        address wNat
    )
        private
    {
        // update address updater if necessary
        if (addressUpdater != getAddressUpdater()) {
            setAddressUpdaterValue(addressUpdater);
        }
        // update contracts on asset managers
        _callOnManagers(_assetManagers,
            abi.encodeCall(IISettingsManagement.updateSystemContracts,
                (assetManagerController, IWNat(wNat))));
        // if this controller was replaced, set forwarding address
        if (assetManagerController != address(this)) {
            replacedBy = assetManagerController;
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Emergency pause

    function emergencyPause(IIAssetManager[] memory _assetManagers, uint256 _duration)
        external
    {
        bool byGovernance = msg.sender == governance();
        require(byGovernance || emergencyPauseSenders.contains(msg.sender),
            OnlyGovernanceOrEmergencyPauseSenders());
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.emergencyPause, (byGovernance, _duration)));
    }

    function emergencyPauseTransfers(IIAssetManager[] memory _assetManagers, uint256 _duration)
        external
    {
        bool byGovernance = msg.sender == governance();
        require(byGovernance || emergencyPauseSenders.contains(msg.sender),
            OnlyGovernanceOrEmergencyPauseSenders());
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.emergencyPauseTransfers, (byGovernance, _duration)));
    }

    function resetEmergencyPauseTotalDuration(IIAssetManager[] memory _assetManagers)
        external
        onlyImmediateGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.resetEmergencyPauseTotalDuration, ()));
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.resetEmergencyPauseTransfersTotalDuration, ()));
    }

    function addEmergencyPauseSender(address _address)
        external
        onlyImmediateGovernance
    {
        emergencyPauseSenders.add(_address);
    }

    function removeEmergencyPauseSender(address _address)
        external
        onlyImmediateGovernance
    {
        emergencyPauseSenders.remove(_address);
    }

    function setMaxEmergencyPauseDurationSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setMaxEmergencyPauseDurationSeconds.selector, _value);
    }

    function setEmergencyPauseDurationResetAfterSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setEmergencyPauseDurationResetAfterSeconds.selector, _value);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Helpers

    function _setValueOnManagers(IIAssetManager[] memory _assetManagers, bytes4 _selector, address _value) private {
        _callOnManagers(_assetManagers, abi.encodeWithSelector(_selector, (_value)));
    }

    function _setValueOnManagers(IIAssetManager[] memory _assetManagers, bytes4 _selector, uint256 _value) private {
        _callOnManagers(_assetManagers, abi.encodeWithSelector(_selector, (_value)));
    }

    function _callOnManagers(IIAssetManager[] memory _assetManagers, bytes memory _calldata) private {
        for (uint256 i = 0; i < _assetManagers.length; i++) {
            address assetManager = address(_assetManagers[i]);
            require(assetManagerIndex[assetManager] != 0, AssetManagerNotManaged());
            Address.functionCall(assetManager, _calldata);
        }
    }
}

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

