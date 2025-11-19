
## *MAIN TARGET CONTRACT* TO REVIEW

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

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IIFAsset} from "../../fassetToken/interfaces/IIFAsset.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {IAgentOwnerRegistry} from "../../userInterfaces/IAgentOwnerRegistry.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";


// global state helpers
library Globals {
    bytes32 internal constant ASSET_MANAGER_SETTINGS_POSITION = keccak256("fasset.AssetManager.Settings");

    function getSettings()
        internal pure
        returns (AssetManagerSettings.Data storage _settings)
    {
        bytes32 position = ASSET_MANAGER_SETTINGS_POSITION;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _settings.slot := position
        }
    }

    function getWNat()
        internal view
        returns (IWNat)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return IWNat(address(state.collateralTokens[state.poolCollateralIndex].token));
    }

    function getPoolCollateral()
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.collateralTokens[state.poolCollateralIndex];
    }

    function getFAsset()
        internal view
        returns (IIFAsset)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return IIFAsset(settings.fAsset);
    }

    function getAgentOwnerRegistry()
        internal view
        returns (IAgentOwnerRegistry)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return IAgentOwnerRegistry(settings.agentOwnerRegistry);
    }

    function getBurnAddress()
        internal view
        returns (address payable)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return settings.burnAddress;
    }
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
pragma solidity ^0.8.27;

library Redemption {
    enum Status {
        EMPTY,      // redemption request with this id doesn't exist
        ACTIVE,     // waiting for confirmation/default
        DEFAULTED,  // default called, failed or late payment can still be confirmed
        // final statuses - there can be no valid payment for this redemption anymore
        SUCCESSFUL, // successful payment confirmed
        FAILED,     // payment failed
        BLOCKED,    // payment blocked
        REJECTED    // redemption request rejected due to invalid redeemer's address
    }

    struct Request {
        bytes32 redeemerUnderlyingAddressHash;
        uint128 underlyingValueUBA;
        uint128 underlyingFeeUBA;
        uint64 firstUnderlyingBlock;
        uint64 lastUnderlyingBlock;
        uint64 lastUnderlyingTimestamp;
        uint64 valueAMG;
        address redeemer;
        uint64 timestamp;
        address agentVault;
        Redemption.Status status;
        bool poolSelfClose;
        address payable executor;
        uint64 executorFeeNatGWei;
        uint64 __rejectionTimestamp; // only storage placeholder
        uint64 __takeOverTimestamp; // only storage placeholder
        string redeemerUnderlyingAddressString;
        bool transferToCoreVault;
        uint16 poolFeeShareBIPS;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";


interface IFAsset is IERC20, IERC20Metadata {
    ////////////////////////////////////////////////////////////////////////////////////
    // System information

    /**
     * The name of the underlying asset.
     */
    function assetName() external view returns (string memory);

    /**
     * The symbol of the underlying asset.
     */
    function assetSymbol() external view returns (string memory);

    /**
     * Get the asset manager, corresponding to this fAsset.
     * fAssets and asset managers are in 1:1 correspondence.
     */
    function assetManager() external view returns (address);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {RedemptionQueue} from "./RedemptionQueue.sol";
import {PaymentConfirmations} from "./PaymentConfirmations.sol";
import {UnderlyingAddressOwnership} from "./UnderlyingAddressOwnership.sol";
import {CollateralReservation} from "./CollateralReservation.sol";
import {Redemption} from "./Redemption.sol";
import {CollateralTypeInt} from "./CollateralTypeInt.sol";


library AssetManagerState {
    struct State {
        // All collateral types, used for vault or pool.
        // Pool collateral (always WNat) has index 0.
        CollateralTypeInt.Data[] collateralTokens;

        // mapping((collateralClass, tokenAddress) => collateralTokens index + 1)
        mapping(bytes32 => uint256) collateralTokenIndex;

        // makes sure pool tokens have unique names and symbols
        mapping(string => bool) reservedPoolTokenSuffixes;

        // A list of all agents (for use by monitoring or challengers).
        // Type: array of agent vault addresses; when one is deleted, its position is filled with last
        address[] allAgents;

        // A list of all agents that are available for minting.
        // Type: array of agent vault addresses; when one is deleted, its position is filled with last
        address[] availableAgents;

        // Ownership of underlying source addresses is needed to prevent someone
        // overtaking the payer and presenting an underlying payment as his own.
        UnderlyingAddressOwnership.State underlyingAddressOwnership;

        // Type: mapping collateralReservationId => collateralReservation
        mapping(uint256 => CollateralReservation.Data) crts;

        // redemption queue
        RedemptionQueue.State redemptionQueue;

        // mapping redemptionRequest_id => request
        mapping(uint256 => Redemption.Request) redemptionRequests;

        // verified payment hashes; expire in 5 days
        PaymentConfirmations.State paymentConfirmations;

        // New ids (listed together to save storage); all must be incremented before assigning, so 0 means empty
        uint64 newCrtId;
        uint64 newRedemptionRequestId;
        uint64 newPaymentAnnouncementId;

        // Total collateral reservations (in underlying AMG units). Used by minting cap.
        uint64 totalReservedCollateralAMG;

        // Pool collateral is always wrapped NAT, but the wrapping contract may change.
        // In this case, new pool collateral token must be added and set as current.
        uint16 poolCollateralIndex;

        // Current block number and timestamp on the underlying chain
        uint64 currentUnderlyingBlock;
        uint64 currentUnderlyingBlockTimestamp;

        // The timestamp (on this network) when the underlying block was last updated
        uint64 currentUnderlyingBlockUpdatedAt;

        // If non-zero, minting is paused and has been paused at the time indicated by timestamp mintingPausedAt.
        // When asset manager is paused, no new mintings can be done, but redemptions still work.
        uint64 mintingPausedAt;

        // If non-zero, asset manager is paused and will be paused until the time indicated.
        // When asset manager is paused, all dangerous operations ar blocked (mintings, redemptions, etc.).
        // It is an extreme measure, which can be used in case there is a dangerous hole in the system.
        uint64 emergencyPausedUntil;

        // When emergency pause is not done by governance, the total allowed pause is limited.
        // So the caller must state the duration after which the pause will automatically end.
        // When total pauses exceed the max allowed length, pausing is only allowed by the governance.
        // An emergencyPause call by the governance optionally resets the total duration counter to 0.
        uint64 emergencyPausedTotalDuration;

        // When emergency pause was triggered by governance, only governance can unpause.
        bool emergencyPausedByGovernance;

        // If non-zero, asset manager is paused and will be paused until the time indicated.
        // When asset manager is paused, all dangerous operations ar blocked (mintings, redemptions, etc.).
        // It is an extreme measure, which can be used in case there is a dangerous hole in the system.
        uint64 transfersEmergencyPausedUntil;

        // When emergency pause is not done by governance, the total allowed pause is limited.
        // So the caller must state the duration after which the pause will automatically end.
        // When total pauses exceed the max allowed length, pausing is only allowed by the governance.
        // An emergencyPause call by the governance optionally resets the total duration counter to 0.
        uint64 transfersEmergencyPausedTotalDuration;

        // When emergency pause was triggered by governance, only governance can unpause.
        bool transfersEmergencyPausedByGovernance;

        // When true, asset manager has been added to the asset manager controller.
        // Even though the asset manager controller address is set at the construction time, the manager may not
        // be able to be added to the controller immediately because the method addAssetManager must be called
        // by the governance multisig (with timelock).
        // During this time it is impossible to verify through the controller that the asset manager is legit.
        // Therefore creating agents and minting is disabled until the asset manager controller notifies
        // the asset manager that it has been added.
        bool attached;
    }

    // diamond state access to state and settings

    bytes32 internal constant STATE_POSITION = keccak256("fasset.AssetManager.State");

    function get() internal pure returns (AssetManagerState.State storage _state) {
        // Only direct constants are allowed in inline assembly, so we assign it here
        bytes32 position = STATE_POSITION;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _state.slot := position
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;


library AssetManagerSettings {
    struct Data {
        // Required contracts.
        // Only used to verify that calls come from assetManagerController.
        // Type: AssetManagerController
        // changed via address updater
        address assetManagerController;

        // The f-asset contract managed by this asset manager.
        // Type: IIFAsset
        // immutable
        address fAsset;

        // Factory for creating new agent vaults.
        // Type: IIAgentVaultFactory
        // timelocked
        address agentVaultFactory;

        // Factory for creating new agent collateral pools.
        // Type: IICollateralPoolFactory
        // timelocked
        address collateralPoolFactory;

        // Factory for creating new agent collateral pool tokens.
        // Type: IICollateralPoolTokenFactory
        // timelocked
        address collateralPoolTokenFactory;

        // The suffix to pool token name and symbol that identifies new vault's collateral pool token.
        // When vault is created, the owner passes own suffix which will be appended to this.
        string poolTokenSuffix;

        // If set, the whitelist contains a list of accounts that can call public methods
        // (minting, redeeming, challenging, etc.)
        // This can be `address(0)`, in which case no whitelist checks are done.
        // Type: IWhitelist
        // timelocked
        address __whitelist; // only storage placeholder

        // If set, the owner address registry contains a list of allowed agent owner's
        // management addresses and mappings from management to work address.
        // Type: IAgentOwnerRegistry
        // timelocked
        address agentOwnerRegistry;

        // Attestation client verifies and decodes attestation proofs.
        // Type: IFdcVerification
        // changed via address updater
        address fdcVerification;

        // The address where burned NAT is sent.
        // immutable
        address payable burnAddress;

        // The contract that reads prices from FTSO system in an FTSO version independent way.
        // Type: IPriceReader
        // timelocked
        address priceReader;

        // Same as assetToken.decimals()
        // immutable
        uint8 assetDecimals;

        // Number of decimals of precision of minted amounts.
        // assetMintingGranularityUBA = 10 ** (assetDecimals - assetMintingDecimals)
        // immutable
        uint8 assetMintingDecimals;

        // Must match attestation data chainId.
        // immutable
        bytes32 chainId;

        // Average time between two successive blocks on the underlying chain, in milliseconds.
        // rate-limited
        uint32 averageBlockTimeMS;

        // The minimum amount of pool tokens the agent must hold to be able to mint.
        // To be able to mint, the NAT value of all backed fassets together with new ones times this percentage
        // must be smaller than the agent's pool tokens' amount converted to NAT.
        // rate-limited
        uint32 mintingPoolHoldingsRequiredBIPS;

        // Collateral reservation fee that must be paid by the minter.
        // Payment is in NAT, but is proportional to the value of assets to be minted.
        // rate-limited
        uint16 collateralReservationFeeBIPS;

        // Asset unit value (e.g. 1 BTC or 1 ETH) in UBA = 10 ** assetToken.decimals()
        // immutable
        uint64 assetUnitUBA;

        // The granularity in which lots are measured = the value of AMG (asset minting granularity) in UBA.
        // Can only be changed via redeploy of AssetManager.
        // AMG is used internally instead of UBA so that minted quantities fit into 64bits to reduce storage.
        // So assetMintingGranularityUBA should be set so that the max supply in AMG of this currency
        // in foreseeable time (say 100yr) cannot overflow 64 bits.
        // immutable
        uint64 assetMintingGranularityUBA;

        // Lot size in asset minting granularity. May change, which affects subsequent mintings and redemptions.
        // timelocked
        uint64 lotSizeAMG;

        // The percentage of minted f-assets that the agent must hold in his underlying address.
        uint16 __minUnderlyingBackingBIPS; // only storage placeholder

        // for some chains (e.g. Ethereum) we require that agent proves that underlying address is an EOA address
        // this must be done by presenting a payment proof from that address
        // immutable
        bool __requireEOAAddressProof; // only storage placeholder

        // Maximum minted amount of the f-asset.
        // rate-limited
        uint64 mintingCapAMG;

        // Number of underlying blocks that the minter or agent is allowed to pay underlying value.
        // If payment not reported in that time, minting/redemption can be challenged and default action triggered.
        // CAREFUL: Count starts from the current proved block height, so the minters and agents should
        // make sure that current block height is fresh, otherwise they might not have enough time for payment.
        // timelocked
        uint64 underlyingBlocksForPayment;

        // Minimum time to allow agent to pay for redemption or minter to pay for minting.
        // This is useful for fast chains, when there can be more than one block per second.
        // Redemption/minting payment failure can be called only after underlyingSecondsForPayment have elapsed
        // on underlying chain.
        // CAREFUL: Count starts from the current proved block timestamp, so the minters and agents should
        // make sure that current block timestamp is fresh, otherwise they might not have enough time for payment.
        // This is partially mitigated by adding local duration since the last block height update to
        // the current underlying block timestamp.
        // timelocked
        uint64 underlyingSecondsForPayment;

        // Redemption fee in underlying currency base amount (UBA).
        // rate-limited
        uint16 redemptionFeeBIPS;

        // On redemption underlying payment failure, redeemer is compensated with
        // redemption value recalculated in flare/sgb times redemption failure factor.
        // Expressed in BIPS, e.g. 12000 for factor of 1.2.
        // This is the part of factor paid from agent's vault collateral.
        // rate-limited
        uint32 redemptionDefaultFactorVaultCollateralBIPS;

        // This is the part of redemption factor paid from agent's pool collateral.
        // rate-limited
        uint32 __redemptionDefaultFactorPoolBIPS; // only storage placeholder

        // If the agent or redeemer becomes unresponsive, we still need payment or non-payment confirmations
        // to be presented eventually to properly track agent's underlying balance.
        // Therefore we allow anybody to confirm payments/non-payments this many seconds after request was made.
        // rate-limited
        uint64 confirmationByOthersAfterSeconds;

        // The user who makes abandoned redemption confirmations gets rewarded by the following amount.
        // rate-limited
        uint128 confirmationByOthersRewardUSD5;

        // To prevent unbounded work, the number of tickets redeemed in a single request is limited.
        // rate-limited
        // >= 1
        uint16 maxRedeemedTickets;

        // Challenge reward can be composed of two part - fixed and proportional (any of them can be zero).
        // This is the proportional part (in BIPS).
        // rate-limited
        uint16 paymentChallengeRewardBIPS;

        // Challenge reward can be composed of two part - fixed and proportional (any of them can be zero).
        // This is the fixed part (in vault collateral token wei).
        // rate-limited
        uint128 paymentChallengeRewardUSD5;

        // Agent has to announce any collateral withdrawal ar vault destroy and then wait for at least
        // withdrawalWaitMinSeconds. This prevents challenged agent to remove all collateral before
        // challenge can be proved.
        // rate-limited
        uint64 withdrawalWaitMinSeconds;

        // Maximum age that trusted price feed is valid.
        // Otherwise (if there were no trusted votes for that long) just use generic ftso price feed.
        // rate-limited
        uint64 maxTrustedPriceAgeSeconds;

        // Agent can remain in CCB for this much time, after that liquidation starts automatically.
        // rate-limited
        uint64 __ccbTimeSeconds; // only storage placeholder

        // Amount of seconds (typically 1 day) that the payment/non-payment proofs must be available.
        // This setting is used in `unstickMinting` and `finishRedemptionWithoutPayment` to prove that the time when
        // payment/non-payment could be proved has already passed.
        // rate-limited
        uint64 attestationWindowSeconds;

        // Minimum time after an update of a setting before the same setting can be updated again.
        // timelocked
        uint64 minUpdateRepeatTimeSeconds;

        // Ratio at which the agents can buy back their collateral when f-asset is terminated.
        // Typically a bit more than 1 to incentivize agents to buy f-assets and self-close instead.
        // immutable
        uint64 __buybackCollateralFactorBIPS; // only storage placeholder

        // Minimum time that has to pass between underlying withdrawal announcement and the confirmation.
        // Any value is ok, but higher values give more security against multiple announcement attack by a miner.
        // Shouldn't be much bigger than Flare data connector response time, so that payments can be confirmed without
        // extra wait. Should be smaller than confirmationByOthersAfterSeconds (e.g. less than 1 hour).
        // rate-limited
        uint64 __announcedUnderlyingConfirmationMinSeconds;

        // Minimum time from the moment token is deprecated to when it becomes invalid and agents still using
        // it as vault collateral get liquidated.
        // timelocked
        uint64 tokenInvalidationTimeMinSeconds;

        // On some rare occasions (stuck minting), the agent has to unlock collateral.
        // For this, part of collateral corresponding to FTSO asset value is burned and the rest is released.
        // However, we cannot burn typical vault collateral (stablecoins), so the agent must buy them for NAT
        // at FTSO price multiplied with this factor (should be a bit above 1) and then we burn the NATs.
        // timelocked
        uint32 vaultCollateralBuyForFlareFactorBIPS;

        // Amount of seconds that have to pass between available list exit announcement and execution.
        // rate-limited
        uint64 agentExitAvailableTimelockSeconds;

        // Amount of seconds that have to pass between agent fee and pool fee share change announcement and execution.
        // rate-limited
        uint64 agentFeeChangeTimelockSeconds;

        // Amount of seconds that have to pass between agent-set minting collateral ratio (vault or pool)
        // change announcement and execution.
        // rate-limited
        uint64 agentMintingCRChangeTimelockSeconds;

        // Amount of seconds that have to pass between agent-set settings for pool exit collateral ratio
        // change announcement and execution.
        // rate-limited
        uint64 poolExitCRChangeTimelockSeconds;

        // Amount of seconds that an agent is allowed to execute an update once it is allowed.
        // rate-limited
        uint64 agentTimelockedOperationWindowSeconds;

        // duration of the timelock for collateral pool tokens after minting
        uint32 collateralPoolTokenTimelockSeconds;

        // If there was no liquidator for the current liquidation offer,
        // go to the next step of liquidation after a certain period of time.
        // rate-limited
        uint64 liquidationStepSeconds;

        // Factor with which to multiply the asset price in native currency to obtain the payment
        // to the liquidator.
        // Expressed in BIPS, e.g. [12000, 16000, 20000] means that the liquidator will be paid 1.2, 1.6 and 2.0
        // times the market price of the liquidated assets after each `liquidationStepSeconds`.
        // Values in the array must increase and be greater than 100%.
        // rate-limited
        uint256[] liquidationCollateralFactorBIPS;

        // How much of the liquidation is paid in vault collateral.
        // The remainder will be paid in pool NAT collateral.
        uint256[] liquidationFactorVaultCollateralBIPS;

        // Minimum time that the system must wait before performing diamond cut.
        // The actual timelock is the maximum of this setting and GovernanceSettings.timelock.
        uint64 diamondCutMinTimelockSeconds;

        // The maximum total pause that can be triggered by non-governance (but governance allowed) caller.
        // The duration count can be reset by the governance.
        uint64 maxEmergencyPauseDurationSeconds;

        // The amount of time since last emergency pause after which the total pause duration counter
        // will reset automatically.
        uint64 emergencyPauseDurationResetAfterSeconds;

        // The amount of time after which the collateral reservation can be cancelled if the
        // handshake is not completed.
        // rate-limited
        uint64 __cancelCollateralReservationAfterSeconds; // only storage placeholder

        // The amount of collateral reservation fee returned to the minter in case of rejection or cancellation.
        // Expressed in BIPS, e.g. 9500 for factor of 0.95, max 10000 for factor of 1.0.
        // rate-limited
        uint16 __rejectOrCancelCollateralReservationReturnFactorBIPS; // only storage placeholder

        // Time window inside which the agent can reject the redemption request.
        // rate-limited
        uint64 __rejectRedemptionRequestWindowSeconds; // only storage placeholder

        // Time window inside which the agent can take over the redemption request from another agent
        // that has rejected it.
        // rate-limited
        uint64 __takeOverRedemptionRequestWindowSeconds; // only storage placeholder

        // On redemption rejection, without take over, redeemer is compensated with
        // redemption value recalculated in flare/sgb times redemption failure factor.
        // Expressed in BIPS, e.g. 12000 for factor of 1.2.
        // This is the part of factor paid from agent's vault collateral.
        // rate-limited
        uint32 __rejectedRedemptionDefaultFactorVaultCollateralBIPS; // only storage placeholder

        // This is the part of rejected redemption factor paid from agent's pool collateral.
        // rate-limited
        uint32 __rejectedRedemptionDefaultFactorPoolBIPS; // only storage placeholder
    }
}

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
pragma solidity >=0.7.6 <0.9;

import {IVPToken} from "@flarenetwork/flare-periphery-contracts/flare/IVPToken.sol";

/**
 * @title Wrapped Native token
 * @notice Accept native token deposits and mint ERC20 WNAT (wrapped native) tokens 1-1.
 */
interface IWNat is IVPToken {
    /**
     * @notice Deposit Native and mint wNat ERC20.
     */
    function deposit() external payable;

    /**
     * @notice Deposit Native from msg.sender and mints WNAT ERC20 to recipient address.
     * @param recipient An address to receive minted WNAT.
     */
    function depositTo(address recipient) external payable;

    /**
     * @notice Withdraw Native and burn WNAT ERC20.
     * @param amount The amount to withdraw.
     */
    function withdraw(uint256 amount) external;

    /**
     * @notice Withdraw WNAT from an owner and send native tokens to msg.sender given an allowance.
     * @param owner An address spending the Native tokens.
     * @param amount The amount to spend.
     *
     * Requirements:
     *
     * - `owner` must have a balance of at least `amount`.
     * - the caller must have allowance for `owners`'s tokens of at least
     * `amount`.
     */
    function withdrawFrom(address owner, uint256 amount) external;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {CollateralType} from "../../../userInterfaces/data/CollateralType.sol";

library CollateralTypeInt {
    struct Data {
        // The ERC20 token contract for this collateral type.
        // immutable
        IERC20 token;
        // The kind of collateral for this token.
        // immutable
        CollateralType.Class collateralClass;
        // Same as token.decimals(), when that exists.
        // immutable
        uint8 decimals;
        // If some token should not be used anymore as collateral, it has to be announced in advance and it
        // is still valid until this timestamp. After that time, the corresponding collateral is considered as
        // zero and the agents that haven't replaced it are liquidated.
        // When the invalidation has not been announced, this value is 0.
        uint64 validUntil;
        // When `true`, the FTSO with symbol `assetFtsoSymbol` returns asset price relative to this token
        // (such FTSO's will probably exist for major stablecoins).
        // When `false`, the FTSOs with symbols `assetFtsoSymbol` and `tokenFtsoSymbol` give asset and token
        // price relative to the same reference currency and the asset/token price is calculated as their ratio.
        // immutable
        bool directPricePair;
        // FTSO symbol for the asset, relative to this token or a reference currency
        // (it depends on the value of `directPricePair`).
        // immutable
        string assetFtsoSymbol;
        // FTSO symbol for this token in reference currency.
        // Used for asset/token price calculation when `directPricePair` is `false`.
        // Otherwise it is irrelevant to asset/token price calculation, but if it is nonempty,
        // it is still used in calculation of challenger and confirmation rewards
        // (otherwise we assume it approximates the value of USD and pay directly the USD amount in vault collateral).
        // immutable
        string tokenFtsoSymbol;
        // Minimum collateral ratio for healthy agents.
        // timelocked
        uint32 minCollateralRatioBIPS;
        // Minimum collateral ratio for agent in CCB (Collateral call band).
        // If the agent's collateral ratio is less than this, skip the CCB and go straight to liquidation.
        // A bit smaller than minCollateralRatioBIPS.
        // timelocked
        uint32 __ccbMinCollateralRatioBIPS; // only storage placeholder
        // Minimum collateral ratio required to get agent out of liquidation.
        // Will always be greater than minCollateralRatioBIPS.
        // timelocked
        uint32 safetyMinCollateralRatioBIPS;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IICleanable} from "@flarenetwork/flare-periphery-contracts/flare/token/interfaces/IICleanable.sol";
import {IFAsset} from "../../userInterfaces/IFAsset.sol";
import {IICheckPointable} from "./IICheckPointable.sol";


interface IIFAsset is IFAsset, IICheckPointable, IICleanable {
    /**
     * Mints `_amount` od fAsset.
     * Only the assetManager corresponding to this fAsset may call `mint()`.
     */
    function mint(address _owner, uint256 _amount) external;

    /**
     * Burns `_amount` od fAsset.
     * Only the assetManager corresponding to this fAsset may call `burn()`.
     */
    function burn(address _owner, uint256 _amount) external;

    /**
     * Set the contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    function setCleanupBlockNumberManager(address _cleanupBlockNumberManager) external;

    /**
     * The contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    function cleanupBlockNumberManager() external view returns (address);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;


library RedemptionQueue {
    struct Ticket {
        address agentVault;
        uint64 valueAMG;
        uint64 prev;
        uint64 next;
        uint64 prevForAgent;
        uint64 nextForAgent;
    }

    struct AgentQueue {
        uint64 firstTicketId;
        uint64 lastTicketId;
    }

    struct State {
        mapping(uint64 => Ticket) tickets;      // mapping redemption_id=>ticket
        mapping(address => AgentQueue) agents;  // mapping address=>dl-list
        uint64 firstTicketId;
        uint64 lastTicketId;
        uint64 newTicketId;       // increment before assigning to ticket (to avoid 0)
    }

    function createRedemptionTicket(
        State storage _state,
        address _agentVault,
        uint64 _valueAMG
    )
        internal
        returns (uint64)
    {
        AgentQueue storage agent = _state.agents[_agentVault];
        uint64 ticketId = ++_state.newTicketId;   // pre-increment - id can never be 0
        // insert new ticket to the last place in global and agent redemption queues
        _state.tickets[ticketId] = Ticket({
            agentVault: _agentVault,
            valueAMG: _valueAMG,
            prev: _state.lastTicketId,
            next: 0,
            prevForAgent: agent.lastTicketId,
            nextForAgent: 0
        });
        // update links in global redemption queue
        if (_state.firstTicketId == 0) {
            assert(_state.lastTicketId == 0);    // empty queue - first and last must be 0
            _state.firstTicketId = ticketId;
        } else {
            assert(_state.lastTicketId != 0);    // non-empty queue - first and last must be non-zero
            _state.tickets[_state.lastTicketId].next = ticketId;
        }
        _state.lastTicketId = ticketId;
        // update links in agent redemption queue
        if (agent.firstTicketId == 0) {
            assert(agent.lastTicketId == 0);    // empty queue - first and last must be 0
            agent.firstTicketId = ticketId;
        } else {
            assert(agent.lastTicketId != 0);    // non-empty queue - first and last must be non-zero
            _state.tickets[agent.lastTicketId].nextForAgent = ticketId;
        }
        agent.lastTicketId = ticketId;
        // return the new redemption ticket's id
        return ticketId;
    }

    function deleteRedemptionTicket(
        State storage _state,
        uint64 _ticketId
    )
        internal
    {
        Ticket storage ticket = _state.tickets[_ticketId];
        assert(ticket.agentVault != address(0));
        AgentQueue storage agent = _state.agents[ticket.agentVault];
        // unlink from global queue
        if (ticket.prev == 0) {
            assert(_ticketId == _state.firstTicketId);     // ticket is first in queue
            _state.firstTicketId = ticket.next;
        } else {
            assert(_ticketId != _state.firstTicketId);     // ticket is not first in queue
            _state.tickets[ticket.prev].next = ticket.next;
        }
        if (ticket.next == 0) {
            assert(_ticketId == _state.lastTicketId);     // ticket is last in queue
            _state.lastTicketId = ticket.prev;
        } else {
            assert(_ticketId != _state.lastTicketId);     // ticket is not last in queue
            _state.tickets[ticket.next].prev = ticket.prev;
        }
        // unlink from agent queue
        if (ticket.prevForAgent == 0) {
            assert(_ticketId == agent.firstTicketId);     // ticket is first in agent queue
            agent.firstTicketId = ticket.nextForAgent;
        } else {
            assert(_ticketId != agent.firstTicketId);     // ticket is not first in agent queue
            _state.tickets[ticket.prevForAgent].nextForAgent = ticket.nextForAgent;
        }
        if (ticket.nextForAgent == 0) {
            assert(_ticketId == agent.lastTicketId);     // ticket is last in agent queue
            agent.lastTicketId = ticket.prevForAgent;
        } else {
            assert(_ticketId != agent.lastTicketId);     // ticket is not last in agent queue
            _state.tickets[ticket.nextForAgent].prevForAgent = ticket.prevForAgent;
        }
        // delete storage
        delete _state.tickets[_ticketId];
    }

    function getTicket(State storage _state, uint64 _id) internal view returns (Ticket storage) {
        return _state.tickets[_id];
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;


library UnderlyingAddressOwnership {
    error InvalidAddressOwnershipProof();
    error EOAProofRequired();
    error AddressAlreadyClaimed();

    struct Ownership {
        address owner;

        // if not 0, there was a payment proof indicating this is externally owned account
        uint64 __underlyingBlockOfEOAProof; // only storage placeholder

        bool __provedEOA; // only storage placeholder
    }

    struct State {
        // mapping underlyingAddressHash => Ownership
        mapping (bytes32 => Ownership) ownership;
    }

    function claimAndTransfer(
        State storage _state,
        address _owner,
        bytes32 _underlyingAddressHash
    )
        internal
    {
        Ownership storage ownership = _state.ownership[_underlyingAddressHash];
        // check that currently unclaimed
        require(ownership.owner == address(0), AddressAlreadyClaimed());
        // set the new owner
        ownership.owner = _owner;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";


library CollateralType {
    enum Class {
        NONE,   // unused
        POOL,   // pool collateral type
        VAULT  // usable as vault collateral
    }

    // Collateral token is uniquely identified by the pair (collateralClass, token).
    struct Data {
        // The kind of collateral for this token.
        CollateralType.Class collateralClass;

        // The ERC20 token contract for this collateral type.
        IERC20 token;

        // Same as token.decimals(), when that exists.
        uint256 decimals;

        // Token invalidation time. Must be 0 on creation.
        uint256 validUntil;

        // When `true`, the FTSO with symbol `assetFtsoSymbol` returns asset price relative to this token
        // (such FTSO's will probably exist for major stablecoins).
        // When `false`, the FTSOs with symbols `assetFtsoSymbol` and `tokenFtsoSymbol` give asset and token
        // price relative to the same reference currency and the asset/token price is calculated as their ratio.
        bool directPricePair;

        // FTSO symbol for the asset, relative to this token or a reference currency
        // (it depends on the value of `directPricePair`).
        string assetFtsoSymbol;

        // FTSO symbol for this token in reference currency.
        // Used for asset/token price calculation when `directPricePair` is `false`.
        // Otherwise it is irrelevant to asset/token price calculation, but if it is nonempty,
        // it is still used in calculation of challenger and confirmation rewards
        // (otherwise we assume it approximates the value of USD and pay directly the USD amount in vault collateral).
        string tokenFtsoSymbol;

        // Minimum collateral ratio for healthy agents.
        uint256 minCollateralRatioBIPS;

        // Minimum collateral ratio required to get agent out of liquidation.
        // Will always be greater than minCollateralRatioBIPS.
        uint256 safetyMinCollateralRatioBIPS;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

/**
 * Agent owner management and work address management
 */
interface IAgentOwnerRegistry {

    event Whitelisted(address value);
    event WhitelistingRevoked(address value);

    /**
     * Agent owner's work address has been set.
     */
    event WorkAddressChanged(
        address indexed managementAddress,
        address prevWorkAddress,
        address workAddress);

    event AgentDataChanged(
        address indexed managementAddress,
        string name,
        string description,
        string iconUrl,
        string termsOfUseUrl);

    error AgentNotWhitelisted();
    error WorkAddressInUse();


    /**
     * Returns true if the address is whitelisted, false otherwise.
     * @param _address address to check
     */
    function isWhitelisted(address _address) external view returns (bool);

    /**
     * Return agent owner's name.
     * @param _managementAddress agent owner's management address
     */
    function getAgentName(address _managementAddress)
        external view
        returns (string memory);

    /**
     * Return agent owner's description.
     * @param _managementAddress agent owner's management address
     */
    function getAgentDescription(address _managementAddress)
        external view
        returns (string memory);

    /**
     * Return url of the agent owner's icon.
     * @param _managementAddress agent owner's management address
     */
    function getAgentIconUrl(address _managementAddress)
        external view
        returns (string memory);

    /**
     * Return url of the agent's page with terms of use.
     * @param _managementAddress agent owner's management address
     */
    function getAgentTermsOfUseUrl(address _managementAddress)
        external view
        returns (string memory);

    /**
     * Get the (unique) work address for the given management address.
     */
    function getWorkAddress(address _managementAddress)
        external view
        returns (address);

    /**
     * Get the (unique) management address for the given work address.
     */
    function getManagementAddress(address _workAddress)
        external view
        returns (address);
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;


interface IICheckPointable {
    /**
     * @notice Total amount of tokens at a specific `_blockNumber`.
     * @param _blockNumber The block number when the totalSupply is queried
     * @return The total amount of tokens at `_blockNumber`
     **/
    function totalSupplyAt(uint256 _blockNumber) external view returns(uint256);

    /**
     * @dev Queries the token balance of `_owner` at a specific `_blockNumber`.
     * @param _owner The address from which the balance will be retrieved.
     * @param _blockNumber The block number when the balance is queried.
     * @return The balance at `_blockNumber`.
     **/
    function balanceOfAt(address _owner, uint256 _blockNumber) external view returns (uint256);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;


library CollateralReservation {
    enum Status {
        ACTIVE,         // the minting process hasn't finished yet
        SUCCESSFUL,     // the payment has been confirmed and the FAssets minted
        DEFAULTED,      // the payment has defaulted and the agent received the collateral reservation fee
        EXPIRED         // the confirmation time has expired and the agent called unstickMinting
    }

    struct Data {
        uint64 valueAMG;
        uint64 firstUnderlyingBlock;
        uint64 lastUnderlyingBlock;
        uint64 lastUnderlyingTimestamp;
        uint128 underlyingFeeUBA;
        uint128 reservationFeeNatWei;
        address agentVault;
        uint16 poolFeeShareBIPS;
        address minter;
        CollateralReservation.Status status;
        address payable executor;
        uint64 executorFeeNatGWei;
        uint64 __handshakeStartTimestamp; // only storage placeholder
        bytes32 __sourceAddressesRoot; // only storage placeholder
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IPayment, IBalanceDecreasingTransaction}
    from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";


library PaymentConfirmations {
    error PaymentAlreadyConfirmed();

    struct State {
        // a store of payment hashes to prevent payment being used / challenged twice
        // structure: map of hash to the next hash in that day
        mapping(bytes32 => bytes32) verifiedPayments;
        // a linked list of payment hashes (one list per day) used for cleanup
        mapping(uint256 => bytes32) __verifiedPaymentsForDay; // only storage placeholder
        // first day number for which we are tracking verifications
        uint256 __verifiedPaymentsForDayStart; // only storage placeholder
    }

    /**
     * For payment transaction with non-unique payment reference (generated from address, not id),
     * we record `tx hash`, so that the same transaction can only be used once for payment.
     */
    function confirmIncomingPayment(
        State storage _state,
        IPayment.Proof calldata _payment
    )
        internal
    {
        _recordPaymentVerification(_state, _payment.data.requestBody.transactionId);
    }

    /**
     * For source decreasing transaction, we record `(source address, tx hash)` pair, since illegal
     * transactions on utxo chains can have multiple input addresses.
     */
    function confirmSourceDecreasingTransaction(
        State storage _state,
        IPayment.Proof calldata _payment
    )
        internal
    {
        bytes32 txKey = transactionKey(_payment.data.responseBody.sourceAddressHash,
            _payment.data.requestBody.transactionId);
        _recordPaymentVerification(_state, txKey);
    }

    /**
     * Check if source decreasing transaction was already confirmed.
     */
    function transactionConfirmed(
        State storage _state,
        IBalanceDecreasingTransaction.Proof calldata _transaction
    )
        internal view
        returns (bool)
    {
        bytes32 txKey = transactionKey(_transaction.data.responseBody.sourceAddressHash,
            _transaction.data.requestBody.transactionId);
        return _state.verifiedPayments[txKey] != 0;
    }

    // the same transaction hash could perform several underlying payments if it is smart contract
    // for now this is illegal, but might change for some smart contract chains
    // therefore the mapping key for transaction is always the combination of
    // underlying address (from which funds were removed) and transaction hash
    function transactionKey(bytes32 _underlyingSourceAddressHash, bytes32 _transactionHash)
        internal pure
        returns (bytes32)
    {
        return keccak256(abi.encode(_underlyingSourceAddressHash, _transactionHash));
    }

    function _recordPaymentVerification(
        State storage _state,
        bytes32 _txKey
    )
        private
    {
        require(_state.verifiedPayments[_txKey] == 0, PaymentAlreadyConfirmed());
        _state.verifiedPayments[_txKey] = _txKey; // any non-zero value is fine
    }
}

## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

interface IICleanable {
    /**
     * Set the contract that is allowed to call history cleaning methods.
     */
    function setCleanerContract(address _cleanerContract) external;

    /**
     * Set the cleanup block number.
     * Historic data for the blocks before `cleanupBlockNumber` can be erased,
     * history before that block should never be used since it can be inconsistent.
     * In particular, cleanup block number must be before current vote power block.
     * @param _blockNumber The new cleanup block number.
     */
    function setCleanupBlockNumber(uint256 _blockNumber) external;

    /**
     * Get the current cleanup block number.
     */
    function cleanupBlockNumber() external view returns (uint256);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IGovernanceVotePower} from "./IGovernanceVotePower.sol";
import {IVPContractEvents} from "./IVPContractEvents.sol";

interface IVPToken is IERC20 {
    /**
     * @notice Delegate by percentage `_bips` of voting power to `_to` from `msg.sender`.
     * @param _to The address of the recipient
     * @param _bips The percentage of voting power to be delegated expressed in basis points (1/100 of one percent).
     *   Not cumulative - every call resets the delegation value (and value of 0 undelegates `to`).
     **/
    function delegate(address _to, uint256 _bips) external;

    /**
     * @notice Undelegate all percentage delegations from the sender and then delegate corresponding
     *   `_bips` percentage of voting power from the sender to each member of `_delegatees`.
     * @param _delegatees The addresses of the new recipients.
     * @param _bips The percentages of voting power to be delegated expressed in basis points (1/100 of one percent).
     *   Total of all `_bips` values must be at most 10000.
     **/
    function batchDelegate(
        address[] memory _delegatees,
        uint256[] memory _bips
    ) external;

    /**
     * @notice Explicitly delegate `_amount` of voting power to `_to` from `msg.sender`.
     * @param _to The address of the recipient
     * @param _amount An explicit vote power amount to be delegated.
     *   Not cumulative - every call resets the delegation value (and value of 0 undelegates `to`).
     **/
    function delegateExplicit(address _to, uint _amount) external;

    /**
     * @notice Revoke all delegation from sender to `_who` at given block.
     *    Only affects the reads via `votePowerOfAtCached()` in the block `_blockNumber`.
     *    Block `_blockNumber` must be in the past.
     *    This method should be used only to prevent rogue delegate voting in the current voting block.
     *    To stop delegating use delegate/delegateExplicit with value of 0 or undelegateAll/undelegateAllExplicit.
     * @param _who Address of the delegatee
     * @param _blockNumber The block number at which to revoke delegation.
     */
    function revokeDelegationAt(address _who, uint _blockNumber) external;

    /**
     * @notice Undelegate all voting power for delegates of `msg.sender`
     *    Can only be used with percentage delegation.
     *    Does not reset delegation mode back to NOTSET.
     **/
    function undelegateAll() external;

    /**
     * @notice Undelegate all explicit vote power by amount delegates for `msg.sender`.
     *    Can only be used with explicit delegation.
     *    Does not reset delegation mode back to NOTSET.
     * @param _delegateAddresses Explicit delegation does not store delegatees' addresses,
     *   so the caller must supply them.
     * @return The amount still delegated (in case the list of delegates was incomplete).
     */
    function undelegateAllExplicit(
        address[] memory _delegateAddresses
    ) external returns (uint256);

    /**
     * @dev Should be compatible with ERC20 method
     */
    function name() external view returns (string memory);

    /**
     * @dev Should be compatible with ERC20 method
     */
    function symbol() external view returns (string memory);

    /**
     * @dev Should be compatible with ERC20 method
     */
    function decimals() external view returns (uint8);

    /**
     * @notice Total amount of tokens at a specific `_blockNumber`.
     * @param _blockNumber The block number when the totalSupply is queried
     * @return The total amount of tokens at `_blockNumber`
     **/
    function totalSupplyAt(uint _blockNumber) external view returns (uint256);

    /**
     * @dev Queries the token balance of `_owner` at a specific `_blockNumber`.
     * @param _owner The address from which the balance will be retrieved.
     * @param _blockNumber The block number when the balance is queried.
     * @return The balance at `_blockNumber`.
     **/
    function balanceOfAt(
        address _owner,
        uint _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the current total vote power.
     * @return The current total vote power (sum of all accounts' vote powers).
     */
    function totalVotePower() external view returns (uint256);

    /**
     * @notice Get the total vote power at block `_blockNumber`
     * @param _blockNumber The block number at which to fetch.
     * @return The total vote power at the block  (sum of all accounts' vote powers).
     */
    function totalVotePowerAt(
        uint _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the current vote power of `_owner`.
     * @param _owner The address to get voting power.
     * @return Current vote power of `_owner`.
     */
    function votePowerOf(address _owner) external view returns (uint256);

    /**
     * @notice Get the vote power of `_owner` at block `_blockNumber`
     * @param _owner The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_owner` at `_blockNumber`.
     */
    function votePowerOfAt(
        address _owner,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the vote power of `_owner` at block `_blockNumber`, ignoring revocation information (and cache).
     * @param _owner The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_owner` at `_blockNumber`. Result doesn't change if vote power is revoked.
     */
    function votePowerOfAtIgnoringRevocation(
        address _owner,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the delegation mode for '_who'. This mode determines whether vote power is
     *  allocated by percentage or by explicit value. Once the delegation mode is set,
     *  it never changes, even if all delegations are removed.
     * @param _who The address to get delegation mode.
     * @return delegation mode: 0 = NOTSET, 1 = PERCENTAGE, 2 = AMOUNT (i.e. explicit)
     */
    function delegationModeOf(address _who) external view returns (uint256);

    /**
     * @notice Get current delegated vote power `_from` delegator delegated `_to` delegatee.
     * @param _from Address of delegator
     * @param _to Address of delegatee
     * @return The delegated vote power.
     */
    function votePowerFromTo(
        address _from,
        address _to
    ) external view returns (uint256);

    /**
     * @notice Get delegated the vote power `_from` delegator delegated `_to` delegatee at `_blockNumber`.
     * @param _from Address of delegator
     * @param _to Address of delegatee
     * @param _blockNumber The block number at which to fetch.
     * @return The delegated vote power.
     */
    function votePowerFromToAt(
        address _from,
        address _to,
        uint _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Compute the current undelegated vote power of `_owner`
     * @param _owner The address to get undelegated voting power.
     * @return The unallocated vote power of `_owner`
     */
    function undelegatedVotePowerOf(
        address _owner
    ) external view returns (uint256);

    /**
     * @notice Get the undelegated vote power of `_owner` at given block.
     * @param _owner The address to get undelegated voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return The undelegated vote power of `_owner` (= owner's own balance minus all delegations from owner)
     */
    function undelegatedVotePowerOfAt(
        address _owner,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the vote power delegation `delegationAddresses`
     *  and `_bips` of `_who`. Returned in two separate positional arrays.
     * @param _who The address to get delegations.
     * @return _delegateAddresses Positional array of delegation addresses.
     * @return _bips Positional array of delegation percents specified in basis points (1/100 or 1 percent)
     * @return _count The number of delegates.
     * @return _delegationMode The mode of the delegation (NOTSET=0, PERCENTAGE=1, AMOUNT=2).
     */
    function delegatesOf(
        address _who
    )
        external
        view
        returns (
            address[] memory _delegateAddresses,
            uint256[] memory _bips,
            uint256 _count,
            uint256 _delegationMode
        );

    /**
     * @notice Get the vote power delegation `delegationAddresses`
     *  and `pcts` of `_who`. Returned in two separate positional arrays.
     * @param _who The address to get delegations.
     * @param _blockNumber The block for which we want to know the delegations.
     * @return _delegateAddresses Positional array of delegation addresses.
     * @return _bips Positional array of delegation percents specified in basis points (1/100 or 1 percent)
     * @return _count The number of delegates.
     * @return _delegationMode The mode of the delegation (NOTSET=0, PERCENTAGE=1, AMOUNT=2).
     */
    function delegatesOfAt(
        address _who,
        uint256 _blockNumber
    )
        external
        view
        returns (
            address[] memory _delegateAddresses,
            uint256[] memory _bips,
            uint256 _count,
            uint256 _delegationMode
        );

    /**
     * Returns VPContract used for readonly operations (view methods).
     * The only non-view method that might be called on it is `revokeDelegationAt`.
     *
     * @notice `readVotePowerContract` is almost always equal to `writeVotePowerContract`
     * except during upgrade from one VPContract to a new version (which should happen
     * rarely or never and will be anounced before).
     *
     * @notice You shouldn't call any methods on VPContract directly, all are exposed
     * via VPToken (and state changing methods are forbidden from direct calls).
     * This is the reason why this method returns `IVPContractEvents` - it should only be used
     * for listening to events (`Revoke` only).
     */
    function readVotePowerContract() external view returns (IVPContractEvents);

    /**
     * Returns VPContract used for state changing operations (non-view methods).
     * The only non-view method that might be called on it is `revokeDelegationAt`.
     *
     * @notice `writeVotePowerContract` is almost always equal to `readVotePowerContract`
     * except during upgrade from one VPContract to a new version (which should happen
     * rarely or never and will be anounced before). In the case of upgrade,
     * `writeVotePowerContract` will be replaced first to establish delegations, and
     * after some perio (e.g. after a reward epoch ends) `readVotePowerContract` will be set equal to it.
     *
     * @notice You shouldn't call any methods on VPContract directly, all are exposed
     * via VPToken (and state changing methods are forbidden from direct calls).
     * This is the reason why this method returns `IVPContractEvents` - it should only be used
     * for listening to events (`Delegate` and `Revoke` only).
     */
    function writeVotePowerContract() external view returns (IVPContractEvents);

    /**
     * When set, allows token owners to participate in governance voting
     * and delegate governance vote power.
     */
    function governanceVotePower() external view returns (IGovernanceVotePower);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "../../IVPToken.sol";
import "../../IVPContractEvents.sol";
import "./IICleanable.sol";

interface IIVPContract is IICleanable, IVPContractEvents {
    /**
     * Update vote powers when tokens are transfered.
     * Also update delegated vote powers for percentage delegation
     * and check for enough funds for explicit delegations.
     **/
    function updateAtTokenTransfer(
        address _from,
        address _to,
        uint256 _fromBalance,
        uint256 _toBalance,
        uint256 _amount
    ) external;

    /**
     * @notice Delegate `_bips` percentage of voting power to `_to` from `_from`
     * @param _from The address of the delegator
     * @param _to The address of the recipient
     * @param _balance The delegator's current balance
     * @param _bips The percentage of voting power to be delegated expressed in basis points (1/100 of one percent).
     *   Not cumulative - every call resets the delegation value (and value of 0 revokes delegation).
     **/
    function delegate(
        address _from,
        address _to,
        uint256 _balance,
        uint256 _bips
    ) external;

    /**
     * @notice Explicitly delegate `_amount` of voting power to `_to` from `msg.sender`.
     * @param _from The address of the delegator
     * @param _to The address of the recipient
     * @param _balance The delegator's current balance
     * @param _amount An explicit vote power amount to be delegated.
     *   Not cumulative - every call resets the delegation value (and value of 0 undelegates `to`).
     **/
    function delegateExplicit(
        address _from,
        address _to,
        uint256 _balance,
        uint _amount
    ) external;

    /**
     * @notice Revoke all delegation from sender to `_who` at given block.
     *    Only affects the reads via `votePowerOfAtCached()` in the block `_blockNumber`.
     *    Block `_blockNumber` must be in the past.
     *    This method should be used only to prevent rogue delegate voting in the current voting block.
     *    To stop delegating use delegate/delegateExplicit with value of 0 or undelegateAll/undelegateAllExplicit.
     * @param _from The address of the delegator
     * @param _who Address of the delegatee
     * @param _balance The delegator's current balance
     * @param _blockNumber The block number at which to revoke delegation.
     **/
    function revokeDelegationAt(
        address _from,
        address _who,
        uint256 _balance,
        uint _blockNumber
    ) external;

    /**
     * @notice Undelegate all voting power for delegates of `msg.sender`
     *    Can only be used with percentage delegation.
     *    Does not reset delegation mode back to NOTSET.
     * @param _from The address of the delegator
     **/
    function undelegateAll(address _from, uint256 _balance) external;

    /**
     * @notice Undelegate all explicit vote power by amount delegates for `msg.sender`.
     *    Can only be used with explicit delegation.
     *    Does not reset delegation mode back to NOTSET.
     * @param _from The address of the delegator
     * @param _delegateAddresses Explicit delegation does not store delegatees' addresses,
     *   so the caller must supply them.
     * @return The amount still delegated (in case the list of delegates was incomplete).
     */
    function undelegateAllExplicit(
        address _from,
        address[] memory _delegateAddresses
    ) external returns (uint256);

    /**
     * @notice Get the vote power of `_who` at block `_blockNumber`
     *   Reads/updates cache and upholds revocations.
     * @param _who The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_who` at `_blockNumber`.
     */
    function votePowerOfAtCached(
        address _who,
        uint256 _blockNumber
    ) external returns (uint256);

    /**
     * @notice Get the current vote power of `_who`.
     * @param _who The address to get voting power.
     * @return Current vote power of `_who`.
     */
    function votePowerOf(address _who) external view returns (uint256);

    /**
     * @notice Get the vote power of `_who` at block `_blockNumber`
     * @param _who The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_who` at `_blockNumber`.
     */
    function votePowerOfAt(
        address _who,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the vote power of `_who` at block `_blockNumber`, ignoring revocation information (and cache).
     * @param _who The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_who` at `_blockNumber`. Result doesn't change if vote power is revoked.
     */
    function votePowerOfAtIgnoringRevocation(
        address _who,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * Return vote powers for several addresses in a batch.
     * @param _owners The list of addresses to fetch vote power of.
     * @param _blockNumber The block number at which to fetch.
     * @return A list of vote powers.
     */
    function batchVotePowerOfAt(
        address[] memory _owners,
        uint256 _blockNumber
    ) external view returns (uint256[] memory);

    /**
     * @notice Get current delegated vote power `_from` delegator delegated `_to` delegatee.
     * @param _from Address of delegator
     * @param _to Address of delegatee
     * @param _balance The delegator's current balance
     * @return The delegated vote power.
     */
    function votePowerFromTo(
        address _from,
        address _to,
        uint256 _balance
    ) external view returns (uint256);

    /**
     * @notice Get delegated the vote power `_from` delegator delegated `_to` delegatee at `_blockNumber`.
     * @param _from Address of delegator
     * @param _to Address of delegatee
     * @param _balance The delegator's current balance
     * @param _blockNumber The block number at which to fetch.
     * @return The delegated vote power.
     */
    function votePowerFromToAt(
        address _from,
        address _to,
        uint256 _balance,
        uint _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Compute the current undelegated vote power of `_owner`
     * @param _owner The address to get undelegated voting power.
     * @param _balance Owner's current balance
     * @return The unallocated vote power of `_owner`
     */
    function undelegatedVotePowerOf(
        address _owner,
        uint256 _balance
    ) external view returns (uint256);

    /**
     * @notice Get the undelegated vote power of `_owner` at given block.
     * @param _owner The address to get undelegated voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return The undelegated vote power of `_owner` (= owner's own balance minus all delegations from owner)
     */
    function undelegatedVotePowerOfAt(
        address _owner,
        uint256 _balance,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the delegation mode for '_who'. This mode determines whether vote power is
     *  allocated by percentage or by explicit value.
     * @param _who The address to get delegation mode.
     * @return Delegation mode (NOTSET=0, PERCENTAGE=1, AMOUNT=2))
     */
    function delegationModeOf(address _who) external view returns (uint256);

    /**
     * @notice Get the vote power delegation `_delegateAddresses`
     *  and `pcts` of an `_owner`. Returned in two separate positional arrays.
     * @param _owner The address to get delegations.
     * @return _delegateAddresses Positional array of delegation addresses.
     * @return _bips Positional array of delegation percents specified in basis points (1/100 or 1 percent)
     * @return _count The number of delegates.
     * @return _delegationMode The mode of the delegation (NOTSET=0, PERCENTAGE=1, AMOUNT=2).
     */
    function delegatesOf(
        address _owner
    )
        external
        view
        returns (
            address[] memory _delegateAddresses,
            uint256[] memory _bips,
            uint256 _count,
            uint256 _delegationMode
        );

    /**
     * @notice Get the vote power delegation `delegationAddresses`
     *  and `pcts` of an `_owner`. Returned in two separate positional arrays.
     * @param _owner The address to get delegations.
     * @param _blockNumber The block for which we want to know the delegations.
     * @return _delegateAddresses Positional array of delegation addresses.
     * @return _bips Positional array of delegation percents specified in basis points (1/100 or 1 percent)
     * @return _count The number of delegates.
     * @return _delegationMode The mode of the delegation (NOTSET=0, PERCENTAGE=1, AMOUNT=2).
     */
    function delegatesOfAt(
        address _owner,
        uint256 _blockNumber
    )
        external
        view
        returns (
            address[] memory _delegateAddresses,
            uint256[] memory _bips,
            uint256 _count,
            uint256 _delegationMode
        );

    /**
     * The VPToken (or some other contract) that owns this VPContract.
     * All state changing methods may be called only from this address.
     * This is because original msg.sender is sent in `_from` parameter
     * and we must be sure that it cannot be faked by directly calling VPContract.
     * Owner token is also used in case of replacement to recover vote powers from balances.
     */
    function ownerToken() external view returns (IVPToken);

    /**
     * Return true if this IIVPContract is configured to be used as a replacement for other contract.
     * It means that vote powers are not necessarily correct at the initialization, therefore
     * every method that reads vote power must check whether it is initialized for that address and block.
     */
    function isReplacement() external view returns (bool);
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
pragma solidity >=0.7.6 <0.9;

import "../../IVPToken.sol";
import "../../IGovernanceVotePower.sol";
import "./IIVPContract.sol";
import "./IIGovernanceVotePower.sol";
import "./IICleanable.sol";

interface IIVPToken is IVPToken, IICleanable {
    /**
     * Set the contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    function setCleanupBlockNumberManager(
        address _cleanupBlockNumberManager
    ) external;

    /**
     * Sets new governance vote power contract that allows token owners to participate in governance voting
     * and delegate governance vote power.
     */
    function setGovernanceVotePower(
        IIGovernanceVotePower _governanceVotePower
    ) external;

    /**
     * @notice Get the total vote power at block `_blockNumber` using cache.
     *   It tries to read the cached value and if not found, reads the actual value and stores it in cache.
     *   Can only be used if `_blockNumber` is in the past, otherwise reverts.
     * @param _blockNumber The block number at which to fetch.
     * @return The total vote power at the block (sum of all accounts' vote powers).
     */
    function totalVotePowerAtCached(
        uint256 _blockNumber
    ) external returns (uint256);

    /**
     * @notice Get the vote power of `_owner` at block `_blockNumber` using cache.
     *   It tries to read the cached value and if not found, reads the actual value and stores it in cache.
     *   Can only be used if _blockNumber is in the past, otherwise reverts.
     * @param _owner The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_owner` at `_blockNumber`.
     */
    function votePowerOfAtCached(
        address _owner,
        uint256 _blockNumber
    ) external returns (uint256);

    /**
     * Return vote powers for several addresses in a batch.
     * @param _owners The list of addresses to fetch vote power of.
     * @param _blockNumber The block number at which to fetch.
     * @return A list of vote powers.
     */
    function batchVotePowerOfAt(
        address[] memory _owners,
        uint256 _blockNumber
    ) external view returns (uint256[] memory);
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

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Permit.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {IERC5267} from "@openzeppelin/contracts/interfaces/IERC5267.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {IIFAsset} from "../interfaces/IIFAsset.sol";
import {ERC20Permit} from "../../openzeppelin/token/ERC20Permit.sol";
import {CheckPointable} from "./CheckPointable.sol";
import {IAssetManager} from "../../userInterfaces/IAssetManager.sol";
import {IICleanable} from "@flarenetwork/flare-periphery-contracts/flare/token/interfaces/IICleanable.sol";
import {IFAsset} from "../../userInterfaces/IFAsset.sol";
import {IICheckPointable} from "../interfaces/IICheckPointable.sol";


contract FAsset is IIFAsset, IERC165, ERC20, CheckPointable, UUPSUpgradeable, ERC20Permit {
    error OnlyAssetManager();
    error AlreadyInitialized();
    error AlreadyUpgraded();
    error OnlyDeployer();
    error ZeroAssetManager();
    error CannotReplaceAssetManager();
    error OnlyCleanupBlockManager();
    error FAssetTerminated();
    error FAssetBalanceTooLow();
    error CannotTransferToSelf();
    error EmergencyPauseOfTransfersActive();

    /**
     * The name of the underlying asset.
     */
    string public override assetName;

    /**
     * The symbol of the underlying asset.
     */
    string public override assetSymbol;

    /**
     * The contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    address public cleanupBlockNumberManager;

    /**
     * Get the asset manager, corresponding to this fAsset.
     * fAssets and asset managers are in 1:1 correspondence.
     */
    address public override assetManager;

    uint64 private __terminatedAt; // only storage placeholder

    string private _name;
    string private _symbol;
    uint8 private _decimals;

    // the address that created this contract and is allowed to set initial settings
    address private _deployer;
    bool private _initialized;
    uint16 private _version;

    modifier onlyAssetManager() {
        require(msg.sender == assetManager, OnlyAssetManager());
        _;
    }

    constructor()
        ERC20("", "")
    {
        _initialized = true;
        _version = 1000;
    }

    function initialize(
        string memory name_,
        string memory symbol_,
        string memory assetName_,
        string memory assetSymbol_,
        uint8 decimals_
    )
        external
    {
        require(!_initialized, AlreadyInitialized());
        _initialized = true;
        _deployer = msg.sender;
        _name = name_;
        _symbol = symbol_;
        _decimals = decimals_;
        assetName = assetName_;
        assetSymbol = assetSymbol_;
        initializeV1r1();
    }

    function initializeV1r1() public {
        require(_version == 0, AlreadyUpgraded());
        _version = 1;
        initializeEIP712(_name, "1");
    }

    /**
     * Set asset manager contract this can be done only once and must be just after deploy
     * (otherwise nothing can be minted).
     */
    function setAssetManager(address _assetManager)
        external
    {
        require (msg.sender == _deployer, OnlyDeployer());
        require(_assetManager != address(0), ZeroAssetManager());
        require(assetManager == address(0), CannotReplaceAssetManager());
        assetManager = _assetManager;
    }

    /**
     * Mints `_amount` od fAsset.
     * Only the assetManager corresponding to this fAsset may call `mint()`.
     */
    function mint(address _owner, uint256 _amount)
        external override
        onlyAssetManager
    {
        _mint(_owner, _amount);
    }

    /**
     * Burns `_amount` od fAsset.
     * Only the assetManager corresponding to this fAsset may call `burn()`.
     */
    function burn(address _owner, uint256 _amount)
        external override
        onlyAssetManager
    {
        _burn(_owner, _amount);
    }

    /**
     * Returns the name of the token.
     */
    function name() public view virtual override(ERC20, IERC20Metadata) returns (string memory) {
        return _name;
    }

    /**
     * Returns the symbol of the token, usually a shorter version of the name.
     */
    function symbol() public view virtual override(ERC20, IERC20Metadata) returns (string memory) {
        return _symbol;
    }
    /**
     * Implements IERC20Metadata method and returns configurable number of decimals.
     */
    function decimals() public view virtual override(ERC20, IERC20Metadata) returns (uint8) {
        return _decimals;
    }

    /**
     * Set the cleanup block number.
     * Historic data for the blocks before `cleanupBlockNumber` can be erased,
     * history before that block should never be used since it can be inconsistent.
     * In particular, cleanup block number must be before current vote power block.
     * @param _blockNumber The new cleanup block number.
     */
    function setCleanupBlockNumber(uint256 _blockNumber)
        external override
    {
        require(msg.sender == cleanupBlockNumberManager, OnlyCleanupBlockManager());
        _setCleanupBlockNumber(_blockNumber);
    }

    /**
     * Get the current cleanup block number.
     */
    function cleanupBlockNumber()
        external view override
        returns (uint256)
    {
        return _cleanupBlockNumber();
    }

    /**
     * Set the contract that is allowed to call history cleaning methods.
     */
    function setCleanerContract(address _cleanerContract)
        external override
        onlyAssetManager
    {
        _setCleanerContract(_cleanerContract);
    }

    /**
     * Set the contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    function setCleanupBlockNumberManager(address _cleanupBlockNumberManager)
        external
        onlyAssetManager
    {
        cleanupBlockNumberManager = _cleanupBlockNumberManager;
    }

    function _beforeTokenTransfer(address _from, address _to, uint256 _amount)
        internal override
    {
        require(_from == address(0) || balanceOf(_from) >= _amount, FAssetBalanceTooLow());
        require(_from != _to, CannotTransferToSelf());
        // mint and redeem are allowed on transfer pause, but not transfer
        require(_from == address(0) || _to == address(0) || !IAssetManager(assetManager).transfersEmergencyPaused(),
            EmergencyPauseOfTransfersActive());
        // update balance history
        _updateBalanceHistoryAtTransfer(_from, _to, _amount);
    }

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        external pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(IERC20).interfaceId
            || _interfaceId == type(IERC20Metadata).interfaceId
            || _interfaceId == type(IERC5267).interfaceId
            || _interfaceId == type(IERC20Permit).interfaceId
            || _interfaceId == type(IICheckPointable).interfaceId
            || _interfaceId == type(IFAsset).interfaceId
            || _interfaceId == type(IIFAsset).interfaceId
            || _interfaceId == type(IICleanable).interfaceId;
    }

    // support for ERC20Permit
    function _approve(address _owner, address _spender, uint256 _amount)
        internal virtual override (ERC20, ERC20Permit)
    {
        ERC20._approve(_owner, _spender, _amount);
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // UUPS proxy upgrade

    function implementation() external view returns (address) {
        return _getImplementation();
    }

    /**
     * Upgrade calls can only arrive through asset manager.
     * See UUPSUpgradeable._authorizeUpgrade.
     */
    function _authorizeUpgrade(address /* _newImplementation */)
        internal virtual override
        onlyAssetManager
    { // solhint-disable-line no-empty-blocks
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IICheckPointable} from "../interfaces/IICheckPointable.sol";
import {CheckPointHistory} from "../library/CheckPointHistory.sol";
import {CheckPointsByAddress} from "../library/CheckPointsByAddress.sol";

/**
 * @title Check Pointable ERC20 Behavior
 * @notice ERC20 behavior which adds balance check point features.
 **/
abstract contract CheckPointable is IICheckPointable {
    error CheckPointableReadingFromCleanedupBlock();
    error OnlyCleanerContract();
    error CleanupBlockNumberMustNeverDecrease();
    error CleanupBlockMustBeInThePast();

    using CheckPointHistory for CheckPointHistory.CheckPointHistoryState;
    using CheckPointsByAddress for CheckPointsByAddress.CheckPointsByAddressState;

    // The number of history cleanup steps executed for every write operation.
    // It is more than 1 to make as certain as possible that all history gets cleaned eventually.
    uint256 private constant CLEANUP_COUNT = 2;

    // Private member variables
    CheckPointsByAddress.CheckPointsByAddressState private balanceHistory;
    CheckPointHistory.CheckPointHistoryState private totalSupply;

    // Historic data for the blocks before `cleanupBlockNumber` can be erased,
    // history before that block should never be used since it can be inconsistent.
    uint256 private cleanupBlockNumber;

    // Address of the contract that is allowed to call methods for history cleaning.
    address public cleanerContract;

    /**
     * Emitted when a total supply cache entry is created.
     * Allows history cleaners to track total supply cache cleanup opportunities off-chain.
     */
    event CreatedTotalSupplyCache(uint256 _blockNumber);

    // Most cleanup opportunities can be deduced from standard event
    // Transfer(from, to, amount):
    //   - balance history for `from` (if nonzero) and `to` (if nonzero)
    //   - total supply history when either `from` or `to` is zero

    modifier notBeforeCleanupBlock(uint256 _blockNumber) {
        require(_blockNumber >= cleanupBlockNumber, CheckPointableReadingFromCleanedupBlock());
        _;
    }

    modifier onlyCleaner {
        require(msg.sender == cleanerContract, OnlyCleanerContract());
        _;
    }

    /**
     * @dev Queries the token balance of `_owner` at a specific `_blockNumber`.
     * @param _owner The address from which the balance will be retrieved.
     * @param _blockNumber The block number when the balance is queried.
     * @return _balance The balance at `_blockNumber`.
     **/
    function balanceOfAt(address _owner, uint256 _blockNumber)
        public virtual view
        notBeforeCleanupBlock(_blockNumber)
        returns (uint256 _balance)
    {
        return balanceHistory.valueOfAt(_owner, _blockNumber);
    }

    /**
     * @notice Burn current token `amount` for `owner` of checkpoints at current block.
     * @param _owner The address of the owner to burn tokens.
     * @param _amount The amount to burn.
     */
    function _burnForAtNow(address _owner, uint256 _amount) internal virtual {
        uint256 newBalance = balanceOfAt(_owner, block.number) - _amount;
        balanceHistory.writeValue(_owner, newBalance);
        balanceHistory.cleanupOldCheckpoints(_owner, CLEANUP_COUNT, cleanupBlockNumber);
        totalSupply.writeValue(totalSupplyAt(block.number) - _amount);
        totalSupply.cleanupOldCheckpoints(CLEANUP_COUNT, cleanupBlockNumber);
    }

    /**
     * @notice Mint current token `amount` for `owner` of checkpoints at current block.
     * @param _owner The address of the owner to burn tokens.
     * @param _amount The amount to burn.
     */
    function _mintForAtNow(address _owner, uint256 _amount) internal virtual {
        uint256 newBalance = balanceOfAt(_owner, block.number) + _amount;
        balanceHistory.writeValue(_owner, newBalance);
        balanceHistory.cleanupOldCheckpoints(_owner, CLEANUP_COUNT, cleanupBlockNumber);
        totalSupply.writeValue(totalSupplyAt(block.number) + _amount);
        totalSupply.cleanupOldCheckpoints(CLEANUP_COUNT, cleanupBlockNumber);
    }

    /**
     * @notice Total amount of tokens at a specific `_blockNumber`.
     * @param _blockNumber The block number when the _totalSupply is queried
     * @return _totalSupply The total amount of tokens at `_blockNumber`
     **/
    function totalSupplyAt(uint256 _blockNumber)
        public virtual view
        notBeforeCleanupBlock(_blockNumber)
        returns(uint256 _totalSupply)
    {
        return totalSupply.valueAt(_blockNumber);
    }

    /**
     * @notice Transmit token `_amount` `_from` address `_to` address of checkpoints at current block.
     * @param _from The address of the sender.
     * @param _to The address of the receiver.
     * @param _amount The amount to transmit.
     */
    function _transmitAtNow(address _from, address _to, uint256 _amount) internal virtual {
        balanceHistory.transmit(_from, _to, _amount);
        balanceHistory.cleanupOldCheckpoints(_from, CLEANUP_COUNT, cleanupBlockNumber);
        balanceHistory.cleanupOldCheckpoints(_to, CLEANUP_COUNT, cleanupBlockNumber);
    }

    /**
     * Set the cleanup block number.
     */
    function _setCleanupBlockNumber(uint256 _blockNumber) internal {
        require(_blockNumber >= cleanupBlockNumber, CleanupBlockNumberMustNeverDecrease());
        require(_blockNumber < block.number, CleanupBlockMustBeInThePast());
        cleanupBlockNumber = _blockNumber;
    }

    /**
     * Get the cleanup block number.
     */
    function _cleanupBlockNumber() internal view returns (uint256) {
        return cleanupBlockNumber;
    }

    /**
     * @notice Update history at token transfer, the CheckPointable part of `_beforeTokenTransfer` hook.
     * @param _from The address of the sender.
     * @param _to The address of the receiver.
     * @param _amount The amount to transmit.
     */
    function _updateBalanceHistoryAtTransfer(address _from, address _to, uint256 _amount) internal virtual {
        if (_from == address(0)) {
            // mint checkpoint balance data for transferee
            _mintForAtNow(_to, _amount);
        } else if (_to == address(0)) {
            // burn checkpoint data for transferer
            _burnForAtNow(_from, _amount);
        } else {
            // transfer checkpoint balance data
            _transmitAtNow(_from, _to, _amount);
        }
    }

    // history cleanup methods

    /**
     * Set the contract that is allowed to call history cleaning methods.
     */
    function _setCleanerContract(address _cleanerContract) internal {
        cleanerContract = _cleanerContract;
    }

    /**
     * Delete balance checkpoints that expired (i.e. are before `cleanupBlockNumber`).
     * Method can only be called from the `cleanerContract` (which may be a proxy to external cleaners).
     * @param _owner balance owner account address
     * @param _count maximum number of checkpoints to delete
     * @return the number of checkpoints deleted
     */
    function balanceHistoryCleanup(address _owner, uint256 _count) external onlyCleaner returns (uint256) {
        return balanceHistory.cleanupOldCheckpoints(_owner, _count, cleanupBlockNumber);
    }

    /**
     * Delete total supply checkpoints that expired (i.e. are before `cleanupBlockNumber`).
     * Method can only be called from the `cleanerContract` (which may be a proxy to external cleaners).
     * @param _count maximum number of checkpoints to delete
     * @return the number of checkpoints deleted
     */
    function totalSupplyHistoryCleanup(uint256 _count) external onlyCleaner returns (uint256) {
        return totalSupply.cleanupOldCheckpoints(_count, cleanupBlockNumber);
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


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

