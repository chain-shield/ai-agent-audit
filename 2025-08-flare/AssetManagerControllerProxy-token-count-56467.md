
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";
import {AssetManagerController} from "./AssetManagerController.sol";


contract AssetManagerControllerProxy is ERC1967Proxy {
    constructor(
        address _implementationAddress,
        IGovernanceSettings _governanceSettings,
        address _initialGovernance,
        address _addressUpdater
    )
        ERC1967Proxy(_implementationAddress,
            abi.encodeCall(
                AssetManagerController.initialize,
                (_governanceSettings, _initialGovernance, _addressUpdater)
            )
        )
    {
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";


interface IISettingsManagement {
    function updateSystemContracts(address _controller, IWNat _wNat)
        external;

    function setAgentOwnerRegistry(address _value)
        external;

    function setAgentVaultFactory(address _value)
        external;

    function setCollateralPoolFactory(address _value)
        external;

    function setCollateralPoolTokenFactory(address _value)
        external;

    function setPriceReader(address _value)
        external;

    function setFdcVerification(address _value)
        external;

    function setCleanerContract(address _value)
        external;

    function setCleanupBlockNumberManager(address _value)
        external;

    function upgradeFAssetImplementation(address _value, bytes memory callData)
        external;

    function setTimeForPayment(uint256 _underlyingBlocks, uint256 _underlyingSeconds)
        external;

    function setPaymentChallengeReward(uint256 _rewardNATWei, uint256 _rewardBIPS)
        external;

    function setMinUpdateRepeatTimeSeconds(uint256 _value)
        external;

    function setLotSizeAmg(uint256 _value)
        external;

    function setMaxTrustedPriceAgeSeconds(uint256 _value)
        external;

    function setCollateralReservationFeeBips(uint256 _value)
        external;

    function setRedemptionFeeBips(uint256 _value)
        external;

    function setRedemptionDefaultFactorVaultCollateralBIPS(uint256 _value)
        external;

    function setConfirmationByOthersAfterSeconds(uint256 _value)
        external;

    function setConfirmationByOthersRewardUSD5(uint256 _value)
        external;

    function setMaxRedeemedTickets(uint256 _value)
        external;

    function setWithdrawalOrDestroyWaitMinSeconds(uint256 _value)
        external;

    function setAttestationWindowSeconds(uint256 _value)
        external;

    function setAverageBlockTimeMS(uint256 _value)
        external;

    function setMintingPoolHoldingsRequiredBIPS(uint256 _value)
        external;

    function setMintingCapAmg(uint256 _value)
        external;

    function setTokenInvalidationTimeMinSeconds(uint256 _value)
        external;

    function setVaultCollateralBuyForFlareFactorBIPS(uint256 _value)
        external;

    function setAgentExitAvailableTimelockSeconds(uint256 _value)
        external;

    function setAgentFeeChangeTimelockSeconds(uint256 _value)
        external;

    function setAgentMintingCRChangeTimelockSeconds(uint256 _value)
        external;

    function setPoolExitCRChangeTimelockSeconds(uint256 _value)
        external;

    function setAgentTimelockedOperationWindowSeconds(uint256 _value)
        external;

    function setCollateralPoolTokenTimelockSeconds(uint256 _value)
        external;

    function setLiquidationStepSeconds(uint256 _stepSeconds)
        external;

    function setLiquidationPaymentFactors(
        uint256[] memory _liquidationFactors,
        uint256[] memory _vaultCollateralFactors
    ) external;

    function setMaxEmergencyPauseDurationSeconds(uint256 _value)
        external;

    function setEmergencyPauseDurationResetAfterSeconds(uint256 _value)
        external;
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

import {IAssetManager} from "./IAssetManager.sol";


interface IAssetManagerController {
    /**
     * Return the list of all asset managers managed by this controller.
     */
    function getAssetManagers()
        external view
        returns (IAssetManager[] memory);

    /**
     * Check whether the asset manager is managed by this controller.
     * @param _assetManager an asset manager address
     */
    function assetManagerExists(address _assetManager)
        external view
        returns (bool);
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
pragma solidity >=0.7.6 <0.9;

interface IRedemptionTimeExtension {
    function setRedemptionPaymentExtensionSeconds(uint256 _value)
        external;

    function redemptionPaymentExtensionSeconds()
        external view
        returns (uint256);
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

import {IERC1967} from "@openzeppelin/contracts/interfaces/IERC1967.sol";


interface IUUPSUpgradeable is IERC1967 {
    /**
     * Upgrade proxy to new implementation.
     */
    function upgradeTo(address _newImplementation) external;

    /**
     * Upgrade proxy to new implementation and call an initialization method (via delegatecall).
     * @param _newImplementation the new implementation address
     * @param _initializeCall abi encoded call of some initialization method (as created by `abi.encodeCall`);
     *   if empty string is passed, no call is made
     */
    function upgradeToAndCall(address _newImplementation, bytes memory _initializeCall) external payable;
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IConfirmedBlockHeightExists, IPayment, IAddressValidity, IReferencedPaymentNonexistence,
        IBalanceDecreasingTransaction}
    from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {IDiamondLoupe} from "../diamond/interfaces/IDiamondLoupe.sol";
import {AssetManagerSettings} from "./data/AssetManagerSettings.sol";
import {CollateralType} from "./data/CollateralType.sol";
import {AgentInfo} from "./data/AgentInfo.sol";
import {AgentSettings} from "./data/AgentSettings.sol";
import {AvailableAgentInfo} from "./data/AvailableAgentInfo.sol";
import {RedemptionTicketInfo} from "./data/RedemptionTicketInfo.sol";
import {RedemptionRequestInfo} from "./data/RedemptionRequestInfo.sol";
import {CollateralReservationInfo} from "./data/CollateralReservationInfo.sol";
import {IAssetManagerEvents} from "./IAssetManagerEvents.sol";
import {IAgentPing} from "./IAgentPing.sol";
import {IRedemptionTimeExtension} from "./IRedemptionTimeExtension.sol";
import {ICoreVaultClient} from "./ICoreVaultClient.sol";
import {ICoreVaultClientSettings} from "./ICoreVaultClientSettings.sol";
import {IAgentAlwaysAllowedMinters} from "./IAgentAlwaysAllowedMinters.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";


/**
 * Asset manager publicly callable methods.
 */
interface IAssetManager is
    IERC165,
    IDiamondLoupe,
    IAssetManagerEvents,
    IAgentPing,
    IRedemptionTimeExtension,
    ICoreVaultClient,
    ICoreVaultClientSettings,
    IAgentAlwaysAllowedMinters
{
    ////////////////////////////////////////////////////////////////////////////////////
    // Basic system information

    /**
     * Get the asset manager controller, the only address that can change settings.
     * Asset manager must be attached to the asset manager controller in the system contract registry.
     */
    function assetManagerController()
        external view
        returns (address);

    /**
     * Get the f-asset contract managed by this asset manager instance.
     */
    function fAsset()
        external view
        returns (IERC20);

    /**
     * Get the price reader contract used by this asset manager instance.
     */
    function priceReader()
        external view
        returns (address);

    /**
     * Return lot size in UBA (underlying base amount - smallest amount on underlying chain, e.g. satoshi).
     */
    function lotSize()
        external view
        returns (uint256 _lotSizeUBA);

    /**
     * Return asset minting granularity - smallest unit of f-asset stored internally
     * within this asset manager instance.
     */
    function assetMintingGranularityUBA()
        external view
        returns (uint256);

    /**
     * Return asset minting decimals - the number of decimals of precision for minting.

     */
    function assetMintingDecimals()
        external view
        returns (uint256);

    ////////////////////////////////////////////////////////////////////////////////////
    // System settings

    /**
     * Get complete current settings.
     * @return the current settings
     */
    function getSettings()
        external view
        returns (AssetManagerSettings.Data memory);

    /**
     * When `controllerAttached` is true, asset manager has been added to the asset manager controller.
     * This is required for the asset manager to be operational (create agent and minting don't work otherwise).
     */
    function controllerAttached()
        external view
        returns (bool);

    ////////////////////////////////////////////////////////////////////////////////////
    // Emergency pause

    /**
     * If true, the system is in emergency pause mode and most operations (mint, redeem, liquidate) are disabled.
     */
    function emergencyPaused()
        external view
        returns (bool);

    /**
     * The time when emergency pause mode will end automatically.
     */
    function emergencyPausedUntil()
        external view
        returns (uint256);

    ////////////////////////////////////////////////////////////////////////////////////
    // Emergency pause transfers

    /**
     * If true, the system is in emergency pause mode and most operations (mint, redeem, liquidate) are disabled.
     */
    function transfersEmergencyPaused()
        external view
        returns (bool);

    /**
     * The time when emergency pause mode will end automatically.
     */
    function transfersEmergencyPausedUntil()
        external view
        returns (uint256);

    ////////////////////////////////////////////////////////////////////////////////////
    // Asset manager upgrading state

    /**
     * True if the asset manager is paused.
     * In the paused state, minting is disabled, but all other operations (e.g. redemptions, liquidation) still work.
     * Paused asset manager can be later unpaused.
     */
    function mintingPaused()
        external view
        returns (bool);

    ////////////////////////////////////////////////////////////////////////////////////
    // Timekeeping for underlying chain

    /**
     * Prove that a block with given number and timestamp exists and
     * update the current underlying block info if the provided data is higher.
     * This method should be called by minters before minting and by agent's regularly
     * to prevent current block being too outdated, which gives too short time for
     * minting or redemption payment.
     * NOTE: anybody can call.
     * @param _proof proof that a block with given number and timestamp exists
     */
    function updateCurrentBlock(
        IConfirmedBlockHeightExists.Proof calldata _proof
    ) external;

    /**
     * Get block number and timestamp of the current underlying block known to the f-asset system.
     * @return _blockNumber current underlying block number tracked by asset manager
     * @return _blockTimestamp current underlying block timestamp tracked by asset manager
     * @return _lastUpdateTs the timestamp on this chain when the current underlying block was last updated
     */
    function currentUnderlyingBlock()
        external view
        returns (uint256 _blockNumber, uint256 _blockTimestamp, uint256 _lastUpdateTs);

    ////////////////////////////////////////////////////////////////////////////////////
    // Available collateral types

    /**
     * Get collateral  information about a token.
     */
    function getCollateralType(CollateralType.Class _collateralClass, IERC20 _token)
        external view
        returns (CollateralType.Data memory);

    /**
     * Get the list of all available and deprecated tokens used for collateral.
     */
    function getCollateralTypes()
        external view
        returns (CollateralType.Data[] memory);

    ////////////////////////////////////////////////////////////////////////////////////
    // Agent create / destroy

    /**
     * Create an agent vault.
     * The agent will always be identified by `_agentVault` address.
     * (Externally, one account may own several agent vaults,
     *  but in fasset system, each agent vault acts as an independent agent.)
     * NOTE: may only be called by an agent on the allowed agent list.
     * Can be called from the management or the work agent owner address.
     * @return _agentVault new agent vault address
     */
    function createAgentVault(
        IAddressValidity.Proof calldata _addressProof,
        AgentSettings.Data calldata _settings
    ) external
        returns (address _agentVault);

    /**
     * Announce that the agent is going to be destroyed. At this time, the agent must not have any mintings
     * or collateral reservations and must not be on the available agents list.
     * NOTE: may only be called by the agent vault owner.
     * @return _destroyAllowedAt the timestamp at which the destroy can be executed
     */
    function announceDestroyAgent(
        address _agentVault
    ) external
        returns (uint256 _destroyAllowedAt);

    /**
     * Delete all agent data, self destruct agent vault and send remaining collateral to the `_recipient`.
     * Procedure for destroying agent:
     * - exit available agents list
     * - wait until all assets are redeemed or perform self-close
     * - announce destroy (and wait the required time)
     * - call destroyAgent()
     * NOTE: may only be called by the agent vault owner.
     * NOTE: the remaining funds from the vault will be transferred to the provided recipient.
     * @param _agentVault address of the agent's vault to destroy
     * @param _recipient address that receives the remaining funds and possible vault balance
     */
    function destroyAgent(
        address _agentVault,
        address payable _recipient
    ) external;

    /**
     * When agent vault, collateral pool or collateral pool token factory is upgraded, new agent vaults
     * automatically get the new implementation from the factory. But the existing agent vaults must
     * be upgraded by their owners using this method.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault address of the agent's vault; both vault, its corresponding pool, and
     *  its pool token will be upgraded to the newest implementations
     */
    function upgradeAgentVaultAndPool(
        address _agentVault
    ) external;

    /**
     * Check if the collateral pool token has been used already by some vault.
     * @param _suffix the suffix to check
     */
    function isPoolTokenSuffixReserved(
        string memory _suffix
    ) external view
        returns (bool);

    ////////////////////////////////////////////////////////////////////////////////////
    // Agent settings update

    /**
     * Due to the effect on the pool, all agent settings are timelocked.
     * This method announces a setting change. The change can be executed after the timelock expires.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     * @param _name setting name, same as for `getAgentSetting`
     * @return _updateAllowedAt the timestamp at which the update can be executed
     */
    function announceAgentSettingUpdate(
        address _agentVault,
        string memory _name,
        uint256 _value
    ) external
        returns (uint256 _updateAllowedAt);

    /**
     * Due to the effect on the pool, all agent settings are timelocked.
     * This method executes a setting change after the timelock expires.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     * @param _name setting name, same as for `getAgentSetting`
     */
    function executeAgentSettingUpdate(
        address _agentVault,
        string memory _name
    ) external;

    /**
     * If the current agent's vault collateral token gets deprecated, the agent must switch with this method.
     * NOTE: may only be called by the agent vault owner.
     * NOTE: at the time of switch, the agent must have enough of both collaterals in the vault.
     */
    function switchVaultCollateral(
        address _agentVault,
        IERC20 _token
    ) external;

    /**
     * When current pool collateral token contract (WNat) is replaced by the method setPoolWNatCollateralType,
     * pools don't switch automatically. Instead, the agent must call this method that swaps old WNat tokens for
     * new ones and sets it for use by the pool.
     * NOTE: may only be called by the agent vault owner.
     */
    function upgradeWNatContract(
        address _agentVault
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Collateral withdrawal announcement

    /**
     * The agent is going to withdraw `_valueNATWei` amount of collateral from the agent vault.
     * This has to be announced and the agent must then wait `withdrawalWaitMinSeconds` time.
     * After that time, the agent can call `withdrawCollateral(_vaultCollateralToken, _valueNATWei)`
     * on the agent vault.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     * @param _valueNATWei the amount to be withdrawn
     * @return _withdrawalAllowedAt the timestamp when the withdrawal can be made
     */
    function announceVaultCollateralWithdrawal(
        address _agentVault,
        uint256 _valueNATWei
    ) external
        returns (uint256 _withdrawalAllowedAt);

    /**
     * The agent is going to redeem `_valueWei` collateral pool tokens in the agent vault.
     * This has to be announced and the agent must then wait `withdrawalWaitMinSeconds` time.
     * After that time, the agent can call `redeemCollateralPoolTokens(_valueNATWei)` on the agent vault.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     * @param _valueNATWei the amount to be withdrawn
     * @return _redemptionAllowedAt the timestamp when the redemption can be made
     */
    function announceAgentPoolTokenRedemption(
        address _agentVault,
        uint256 _valueNATWei
    ) external
        returns (uint256 _redemptionAllowedAt);

    ////////////////////////////////////////////////////////////////////////////////////
    // Underlying balance topup

    /**
     * When the agent tops up his underlying address, it has to be confirmed by calling this method,
     * which updates the underlying free balance value.
     * NOTE: may only be called by the agent vault owner.
     * @param _payment proof of the underlying payment; must include payment
     *      reference of the form `0x4642505266410011000...0<agents_vault_address>`
     * @param _agentVault agent vault address
     */
    function confirmTopupPayment(
        IPayment.Proof calldata _payment,
        address _agentVault
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Underlying withdrawal announcements

    /**
     * Announce withdrawal of underlying currency.
     * In the event UnderlyingWithdrawalAnnounced the agent receives payment reference, which must be
     * added to the payment, otherwise it can be challenged as illegal.
     * Until the announced withdrawal is performed and confirmed or canceled, no other withdrawal can be announced.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     */
    function announceUnderlyingWithdrawal(
        address _agentVault
    ) external;

    /**
     * Agent must provide confirmation of performed underlying withdrawal, which updates free balance with used gas
     * and releases announcement so that a new one can be made.
     * If the agent doesn't call this method, anyone can call it after a time (`confirmationByOthersAfterSeconds`).
     * NOTE: may only be called by the owner of the agent vault
     *   except if enough time has passed without confirmation - then it can be called by anybody.
     * @param _payment proof of the underlying payment
     * @param _agentVault agent vault address
     */
    function confirmUnderlyingWithdrawal(
        IPayment.Proof calldata _payment,
        address _agentVault
    ) external;

    /**
     * Cancel ongoing withdrawal of underlying currency.
     * Needed in order to reset announcement timestamp, so that others cannot front-run the agent at
     * `confirmUnderlyingWithdrawal` call. This could happen if withdrawal would be performed more
     * than `confirmationByOthersAfterSeconds` seconds after announcement.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     */
    function cancelUnderlyingWithdrawal(
        address _agentVault
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Agent information

    /**
     * Get (a part of) the list of all agents.
     * The list must be retrieved in parts since retrieving the whole list can consume too much gas for one block.
     * @param _start first index to return from the available agent's list
     * @param _end end index (one above last) to return from the available agent's list
     */
    function getAllAgents(uint256 _start, uint256 _end)
        external view
        returns (address[] memory _agents, uint256 _totalLength);

    /**
     * Return detailed info about an agent, typically needed by a minter.
     * @param _agentVault agent vault address
     * @return structure containing agent's minting fee (BIPS), min collateral ratio (BIPS),
     *      and current free collateral (lots)
     */
    function getAgentInfo(address _agentVault)
        external view
        returns (AgentInfo.Info memory);

    /**
     * Get agent's setting by name.
     * This allows reading individual settings.
     * @param _agentVault agent vault address
     * @param _name setting name, one of: `feeBIPS`, `poolFeeShareBIPS`, `redemptionPoolFeeShareBIPS`,
     *  `mintingVaultCollateralRatioBIPS`, `mintingPoolCollateralRatioBIPS`,`buyFAssetByAgentFactorBIPS`,
     *  `poolExitCollateralRatioBIPS`
     */
    function getAgentSetting(address _agentVault, string memory _name)
        external view
        returns (uint256);

    /**
     * Returns the collateral pool address of the agent identified by `_agentVault`.
     */
    function getCollateralPool(address _agentVault)
        external view
        returns (address);

    /**
     * Return the management address of the owner of the agent identified by `_agentVault`.
     */
    function getAgentVaultOwner(address _agentVault)
        external view
        returns (address _ownerManagementAddress);

    /**
     * Return vault collateral ERC20 token chosen by the agent identified by `_agentVault`.
     */
    function getAgentVaultCollateralToken(address _agentVault)
        external view
        returns (IERC20);

    /**
     * Return full vault collateral (free + locked) deposited in the vault `_agentVault`.
     */
    function getAgentFullVaultCollateral(address _agentVault)
        external view
        returns (uint256);

    /**
     * Return full pool NAT collateral (free + locked) deposited in the vault `_agentVault`.
     */
    function getAgentFullPoolCollateral(address _agentVault)
        external view
        returns (uint256);

    /**
     * Return the current liquidation factors and max liquidation amount of the agent
     * identified by `_agentVault`.
     */
    function getAgentLiquidationFactorsAndMaxAmount(address _agentVault)
        external view
        returns (
            uint256 liquidationPaymentFactorVaultBIPS,
            uint256 liquidationPaymentFactorPoolBIPS,
            uint256 maxLiquidationAmountUBA
        );

    /**
     * Return the minimum collateral ratio of the pool collateral owned by vault `_agentVault`.
     */
    function getAgentMinPoolCollateralRatioBIPS(address _agentVault)
        external view
        returns (uint256);

    /**
     * Return the minimum collateral ratio of the vault collateral owned by vault `_agentVault`.
     */
    function getAgentMinVaultCollateralRatioBIPS(address _agentVault)
        external view
        returns (uint256);

    ////////////////////////////////////////////////////////////////////////////////////
    // List of available agents (i.e. publicly available for minting).

    /**
     * Add the agent to the list of publicly available agents.
     * Other agents can only self-mint.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     */
    function makeAgentAvailable(
        address _agentVault
    ) external;

    /**
     * Announce exit from the publicly available agents list.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     * @return _exitAllowedAt the timestamp when the agent can exit
     */
    function announceExitAvailableAgentList(
        address _agentVault
    ) external
        returns (uint256 _exitAllowedAt);

    /**
     * Exit the publicly available agents list.
     * NOTE: may only be called by the agent vault owner and after announcement.
     * @param _agentVault agent vault address
     */
    function exitAvailableAgentList(
        address _agentVault
    ) external;

    /**
     * Get (a part of) the list of available agents.
     * The list must be retrieved in parts since retrieving the whole list can consume too much gas for one block.
     * @param _start first index to return from the available agent's list
     * @param _end end index (one above last) to return from the available agent's list
     */
    function getAvailableAgentsList(uint256 _start, uint256 _end)
        external view
        returns (address[] memory _agents, uint256 _totalLength);

    /**
     * Get (a part of) the list of available agents with extra information about agents' fee, min collateral ratio
     * and available collateral (in lots).
     * The list must be retrieved in parts since retrieving the whole list can consume too much gas for one block.
     * NOTE: agent's available collateral can change anytime due to price changes, minting, or changes
     * in agent's min collateral ratio, so it is only to be used as an estimate.
     * @param _start first index to return from the available agent's list
     * @param _end end index (one above last) to return from the available agent's list
     */
    function getAvailableAgentsDetailedList(uint256 _start, uint256 _end)
        external view
        returns (AvailableAgentInfo.Data[] memory _agents, uint256 _totalLength);

    ////////////////////////////////////////////////////////////////////////////////////
    // Minting

    /**
     * Before paying underlying assets for minting, minter has to reserve collateral and
     * pay collateral reservation fee. Collateral is reserved at ratio of agent's agentMinCollateralRatio
     * to requested lots NAT market price.
     * The minter receives instructions for underlying payment
     * (value, fee and payment reference) in event CollateralReserved.
     * Then the minter has to pay `value + fee` on the underlying chain.
     * If the minter pays the underlying amount, minter obtains f-assets.
     * The collateral reservation fee is split between the agent and the collateral pool.
     * NOTE: the owner of the agent vault must be in the AgentOwnerRegistry.
     * @param _agentVault agent vault address
     * @param _lots the number of lots for which to reserve collateral
     * @param _maxMintingFeeBIPS maximum minting fee (BIPS) that can be charged by the agent - best is just to
     *      copy current agent's published fee; used to prevent agent from front-running reservation request
     *      and increasing fee (that would mean that the minter would have to pay raised fee or forfeit
     *      collateral reservation fee)
     * @param _executor the account that is allowed to execute minting (besides minter and agent)
     */
    function reserveCollateral(
        address _agentVault,
        uint256 _lots,
        uint256 _maxMintingFeeBIPS,
        address payable _executor
    ) external payable
        returns (uint256 _collateralReservationId);

    /**
     * Return the collateral reservation fee amount that has to be passed to the `reserveCollateral` method.
     * NOTE: the amount paid may be larger than the required amount, but the difference is not returned.
     * It is advised that the minter pays the exact amount, but when the amount is so small that the revert
     * would cost more than the lost difference, the minter may want to send a slightly larger amount to compensate
     * for the possibility of a FTSO price change between obtaining this value and calling `reserveCollateral`.
     * @param _lots the number of lots for which to reserve collateral
     * @return _reservationFeeNATWei the amount of reservation fee in NAT wei
     */
    function collateralReservationFee(uint256 _lots)
        external view
        returns (uint256 _reservationFeeNATWei);

    /**
     * Returns the data about the collateral reservation for an ongoing minting.
     * Note: once the minting is executed or defaulted, the collateral reservation is deleted and this method fails.
     * @param _collateralReservationId the collateral reservation id, as used for executing or defaulting the minting
     */
    function collateralReservationInfo(uint256 _collateralReservationId)
        external view
        returns (CollateralReservationInfo.Data memory);

    /**
     * After obtaining proof of underlying payment, the minter calls this method to finish the minting
     * and collect the minted f-assets.
     * NOTE: may only be called by the minter (= creator of CR, the collateral reservation request),
     *   the executor appointed by the minter, or the agent owner (= owner of the agent vault in CR).
     * @param _payment proof of the underlying payment (must contain exact `value + fee` amount and correct
     *      payment reference)
     * @param _collateralReservationId collateral reservation id
     */
    function executeMinting(
        IPayment.Proof calldata _payment,
        uint256 _collateralReservationId
    ) external;

    /**
     * When the time for the minter to pay the underlying amount is over (i.e. the last underlying block has passed),
     * the agent can declare payment default. Then the agent collects the collateral reservation fee
     * (it goes directly to the vault), and the reserved collateral is unlocked.
     * NOTE: The attestation request must be done with `checkSourceAddresses=false`.
     * NOTE: may only be called by the owner of the agent vault in the collateral reservation request.
     * @param _proof proof that the minter didn't pay with correct payment reference on the underlying chain
     * @param _collateralReservationId id of a collateral reservation created by the minter
     */
    function mintingPaymentDefault(
        IReferencedPaymentNonexistence.Proof calldata _proof,
        uint256 _collateralReservationId
    ) external;

    /**
     * If a collateral reservation request exists for more than 24 hours, payment or non-payment proof are no longer
     * available. In this case the agent can call this method, which burns reserved collateral at market price
     * and releases the remaining collateral (CRF is also burned).
     * NOTE: may only be called by the owner of the agent vault in the collateral reservation request.
     * NOTE: the agent (management address) receives the vault collateral and NAT is burned instead. Therefore
     *      this method is `payable` and the caller must provide enough NAT to cover the received vault collateral
     *      amount multiplied by `vaultCollateralBuyForFlareFactorBIPS`.
     * @param _proof proof that the attestation query window can not not contain
     *      the payment/non-payment proof anymore
     * @param _collateralReservationId collateral reservation id
     */
    function unstickMinting(
        IConfirmedBlockHeightExists.Proof calldata _proof,
        uint256 _collateralReservationId
    ) external payable;

    /**
     * Agent can mint against himself.
     * This is a one-step process, skipping collateral reservation and collateral reservation fee payment.
     * Moreover, the agent doesn't have to be on the publicly available agents list to self-mint.
     * NOTE: may only be called by the agent vault owner.
     * NOTE: the caller must be a whitelisted agent.
     * @param _payment proof of the underlying payment; must contain payment reference of the form
     *      `0x4642505266410012000...0<agent_vault_address>`
     * @param _agentVault agent vault address
     * @param _lots number of lots to mint
     */
    function selfMint(
        IPayment.Proof calldata _payment,
        address _agentVault,
        uint256 _lots
    ) external;

    /**
     * If an agent has enough free underlying, they can mint immediately without any underlying payment.
     * This is a one-step process, skipping collateral reservation and collateral reservation fee payment.
     * Moreover, the agent doesn't have to be on the publicly available agents list to self-mint.
     * NOTE: may only be called by the agent vault owner.
     * NOTE: the caller must be a whitelisted agent.
     * @param _agentVault agent vault address
     * @param _lots number of lots to mint
     */
    function mintFromFreeUnderlying(
        address _agentVault,
        uint64 _lots
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Redemption

    /**
     * Redeem (up to) `_lots` lots of f-assets. The corresponding amount of the f-assets belonging
     * to the redeemer will be burned and the redeemer will get paid by the agent in underlying currency
     * (or, in case of agent's payment default, by agent's collateral with a premium).
     * NOTE: in some cases not all sent f-assets can be redeemed (either there are not enough tickets or
     * more than a fixed limit of tickets should be redeemed). In this case only part of the approved assets
     * are burned and redeemed and the redeemer can execute this method again for the remaining lots.
     * In such a case the `RedemptionRequestIncomplete` event will be emitted, indicating the number
     * of remaining lots.
     * Agent receives redemption request id and instructions for underlying payment in
     * RedemptionRequested event and has to pay `value - fee` and use the provided payment reference.
     * @param _lots number of lots to redeem
     * @param _redeemerUnderlyingAddressString the address to which the agent must transfer underlying amount
     * @param _executor the account that is allowed to execute redemption default (besides redeemer and agent)
     * @return _redeemedAmountUBA the actual redeemed amount; may be less than requested if there are not enough
     *      redemption tickets available or the maximum redemption ticket limit is reached
     */
    function redeem(
        uint256 _lots,
        string memory _redeemerUnderlyingAddressString,
        address payable _executor
    ) external payable
        returns (uint256 _redeemedAmountUBA);

    /**
     * If the redeemer provides invalid address, the agent should provide the proof of address invalidity from the
     * Flare data connector. With this, the agent's obligations are fulfilled and they can keep the underlying.
     * NOTE: may only be called by the owner of the agent vault in the redemption request
     * NOTE: also checks that redeemer's address is normalized, so the redeemer must normalize their address,
     *   otherwise it will be rejected!
     * @param _proof proof that the address is invalid
     * @param _redemptionRequestId id of an existing redemption request
     */
    function rejectInvalidRedemption(
        IAddressValidity.Proof calldata _proof,
        uint256 _redemptionRequestId
    ) external;

    /**
     * After paying to the redeemer, the agent must call this method to unlock the collateral
     * and to make sure that the redeemer cannot demand payment in collateral on timeout.
     * The same method must be called for any payment status (SUCCESS, FAILED, BLOCKED).
     * In case of FAILED, it just releases the agent's underlying funds and the redeemer gets paid in collateral
     * after calling redemptionPaymentDefault.
     * In case of SUCCESS or BLOCKED, remaining underlying funds and collateral are released to the agent.
     * If the agent doesn't confirm payment in enough time (several hours, setting
     * `confirmationByOthersAfterSeconds`), anybody can do it and get rewarded from the agent's vault.
     * NOTE: may only be called by the owner of the agent vault in the redemption request
     *   except if enough time has passed without confirmation - then it can be called by anybody
     * @param _payment proof of the underlying payment (must contain exact `value - fee` amount and correct
     *      payment reference)
     * @param _redemptionRequestId id of an existing redemption request
     */
    function confirmRedemptionPayment(
        IPayment.Proof calldata _payment,
        uint256 _redemptionRequestId
    ) external;

    /**
     * If the agent doesn't transfer the redeemed underlying assets in time (until the last allowed block on
     * the underlying chain), the redeemer calls this method and receives payment in collateral (with some extra).
     * The agent can also call default if the redeemer is unresponsive, to payout the redeemer and free the
     * remaining collateral.
     * NOTE: The attestation request must be done with `checkSourceAddresses=false`.
     * NOTE: may only be called by the redeemer (= creator of the redemption request),
     *   the executor appointed by the redeemer,
     *   or the agent owner (= owner of the agent vault in the redemption request)
     * @param _proof proof that the agent didn't pay with correct payment reference on the underlying chain
     * @param _redemptionRequestId id of an existing redemption request
     */
    function redemptionPaymentDefault(
        IReferencedPaymentNonexistence.Proof calldata _proof,
        uint256 _redemptionRequestId
    ) external;

    /**
     * If the agent hasn't performed the payment, the agent can close the redemption request to free underlying funds.
     * It can be done immediately after the redeemer or agent calls `redemptionPaymentDefault`,
     * or this method can trigger the default payment without proof, but only after enough time has passed so that
     * attestation proof of non-payment is not available any more.
     * NOTE: may only be called by the owner of the agent vault in the redemption request.
     * @param _proof proof that the attestation query window can not not contain
     *      the payment/non-payment proof anymore
     * @param _redemptionRequestId id of an existing, but already defaulted, redemption request
     */
    function finishRedemptionWithoutPayment(
        IConfirmedBlockHeightExists.Proof calldata _proof,
        uint256 _redemptionRequestId
    ) external;

    /**
     * Returns the data about an ongoing redemption request.
     * Note: once the redemptions is confirmed, the request is deleted and this method fails.
     * However, if there is no payment and the redemption defaults, the method works and returns status DEFAULTED.
     * @param _redemptionRequestId the redemption request id, as used for confirming or defaulting the redemption
     */
    function redemptionRequestInfo(uint256 _redemptionRequestId)
        external view
        returns (RedemptionRequestInfo.Data memory);

    /**
     * Agent can "redeem against himself" by calling `selfClose`, which burns agent's own f-assets
     * and unlocks agent's collateral. The underlying funds backing the f-assets are released
     * as agent's free underlying funds and can be later withdrawn after announcement.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     * @param _amountUBA amount of f-assets to self-close
     * @return _closedAmountUBA the actual self-closed amount, may be less than requested if there are not enough
     *      redemption tickets available or the maximum redemption ticket limit is reached
     */
    function selfClose(
        address _agentVault,
        uint256 _amountUBA
    ) external
        returns (uint256 _closedAmountUBA);

    ////////////////////////////////////////////////////////////////////////////////////
    // Redemption queue info

    /**
     * Return (part of) the redemption queue.
     * @param _firstRedemptionTicketId the ticket id to start listing from; if 0, starts from the beginning
     * @param _pageSize the maximum number of redemption tickets to return
     * @return _queue the (part of) the redemption queue; maximum length is _pageSize
     * @return _nextRedemptionTicketId works as a cursor - if the _pageSize is reached and there are more tickets,
     *  it is the first ticket id not returned; if the end is reached, it is 0
     */
    function redemptionQueue(
        uint256 _firstRedemptionTicketId,
        uint256 _pageSize
    ) external view
        returns (RedemptionTicketInfo.Data[] memory _queue, uint256 _nextRedemptionTicketId);

    /**
     * Return (part of) the redemption queue for a specific agent.
     * @param _agentVault the agent vault address of the queried agent
     * @param _firstRedemptionTicketId the ticket id to start listing from; if 0, starts from the beginning
     * @param _pageSize the maximum number of redemption tickets to return
     * @return _queue the (part of) the redemption queue; maximum length is _pageSize
     * @return _nextRedemptionTicketId works as a cursor - if the _pageSize is reached and there are more tickets,
     *  it is the first ticket id not returned; if the end is reached, it is 0
     */
    function agentRedemptionQueue(
        address _agentVault,
        uint256 _firstRedemptionTicketId,
        uint256 _pageSize
    ) external view
        returns (RedemptionTicketInfo.Data[] memory _queue, uint256 _nextRedemptionTicketId);

    ////////////////////////////////////////////////////////////////////////////////////
    // Dust

    /**
     * Due to the minting pool fees or after a lot size change by the governance,
     * it may happen that less than one lot remains on a redemption ticket. This is named "dust" and
     * can be self closed or liquidated, but not redeemed. However, after several additions,
     * the total dust can amount to more than one lot. Using this method, the amount, rounded down
     * to a whole number of lots, can be converted to a new redemption ticket.
     * NOTE: we do NOT check that the caller is the agent vault owner, since we want to
     * allow anyone to convert dust to tickets to increase asset fungibility.
     * NOTE: dust above 1 lot is actually added to ticket at every minting, so this function need
     * only be called when the agent doesn't have any minting.
     * @param _agentVault agent vault address
     */
    function convertDustToTicket(
        address _agentVault
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Liquidation

    /**
     * Checks that the agent's collateral is too low and if true, starts agent's liquidation.
     * If the agent is already in liquidation, returns the timestamp when liquidation started.
     * @param _agentVault agent vault address
     * @return _liquidationStartTs timestamp when liquidation started
     */
    function startLiquidation(
        address _agentVault
    ) external
        returns (uint256 _liquidationStartTs);

    /**
     * Burns up to `_amountUBA` f-assets owned by the caller and pays
     * the caller the corresponding amount of native currency with premium
     * (premium depends on the liquidation state).
     * If the agent isn't in liquidation yet, but satisfies conditions,
     * automatically puts the agent in liquidation status.
     * @param _agentVault agent vault address
     * @param _amountUBA the amount of f-assets to liquidate
     * @return _liquidatedAmountUBA liquidated amount of f-asset
     * @return _amountPaidVault amount paid to liquidator (in agent's vault collateral)
     * @return _amountPaidPool amount paid to liquidator (in NAT from pool)
     */
    function liquidate(
        address _agentVault,
        uint256 _amountUBA
    ) external
        returns (uint256 _liquidatedAmountUBA, uint256 _amountPaidVault, uint256 _amountPaidPool);

    /**
     * When the agent's collateral reaches the safe level during liquidation, the liquidation
     * process can be stopped by calling this method.
     * Full liquidation (i.e. the liquidation triggered by illegal underlying payment)
     * cannot be stopped.
     * NOTE: anybody can call.
     * NOTE: if the method succeeds, the agent's liquidation has ended.
     * @param _agentVault agent vault address
     */
    function endLiquidation(
        address _agentVault
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Challenges

    /**
     * Called with a proof of payment made from the agent's underlying address, for which
     * no valid payment reference exists (valid payment references are from redemption and
     * underlying withdrawal announcement calls).
     * On success, immediately triggers full agent liquidation and rewards the caller.
     * @param _payment proof of a transaction from the agent's underlying address
     * @param _agentVault agent vault address
     */
    function illegalPaymentChallenge(
        IBalanceDecreasingTransaction.Proof calldata _payment,
        address _agentVault
    ) external;

    /**
     * Called with proofs of two payments made from the agent's underlying address
     * with the same payment reference (each payment reference is valid for only one payment).
     * On success, immediately triggers full agent liquidation and rewards the caller.
     * @param _payment1 proof of first payment from the agent's underlying address
     * @param _payment2 proof of second payment from the agent's underlying address
     * @param _agentVault agent vault address
     */
    function doublePaymentChallenge(
        IBalanceDecreasingTransaction.Proof calldata _payment1,
        IBalanceDecreasingTransaction.Proof calldata _payment2,
        address _agentVault
    ) external;

    /**
     * Called with proofs of several (otherwise legal) payments, which together make the agent's
     * underlying free balance negative (i.e. the underlying address balance is less than
     * the total amount of backed f-assets).
     * On success, immediately triggers full agent liquidation and rewards the caller.
     * @param _payments proofs of several distinct payments from the agent's underlying address
     * @param _agentVault agent vault address
     */
    function freeBalanceNegativeChallenge(
        IBalanceDecreasingTransaction.Proof[] calldata _payments,
        address _agentVault
    ) external;
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";


interface IGoverned {

    error OnlyExecutor();
    error OnlyGovernance();
    error TimelockInvalidSelector();
    error TimelockNotAllowedYet();
    error AlreadyInProductionMode();
    error GovernedAlreadyInitialized();
    error GovernedAddressZero();

    /**
     * Governance call was timelocked. It can be executed after `allowedAfterTimestamp` by one of the executors.
     * @param encodedCall ABI encoded call data, to be used in executeGovernanceCall
     * @param encodedCallHash keccak256 hash of the ABI encoded call data
     * @param allowedAfterTimestamp the earliest timestamp when the call can be executed
     */
    event GovernanceCallTimelocked(bytes encodedCall, bytes32 encodedCallHash, uint256 allowedAfterTimestamp);

    /**
     * Previously timelocked governance call was executed.
     * @param encodedCallHash keccak256 hash of the ABI encoded call data
     *      (same as `GovernanceCallTimelocked.encodedCallHash`)
     */
    event TimelockedGovernanceCallExecuted(bytes32 encodedCallHash);

    /**
     * Previously timelocked governance call was canceled.
     * @param encodedCallHash keccak256 hash of the ABI encoded call data
     *      (same as `GovernanceCallTimelocked.encodedCallHash`)
     */
    event TimelockedGovernanceCallCanceled(bytes32 encodedCallHash);

    /**
     * Governed contract was initialised (not yet in production mode).
     * @param initialGovernance the governance address used until switch to production mode
     */
    event GovernanceInitialised(address initialGovernance);

    /**
     * The governed contract has switched to production mode
     * Timelocks are now enabled and the governance address is `governanceSettings.getGovernanceAddress()`.
     * @param governanceSettings the system contract holding governance address, timelock and executors settings
     */
    event GovernedProductionModeEntered(address governanceSettings);

    /**
     * @notice Execute the timelocked governance calls once the timelock period expires.
     * @dev Only executor can call this method.
     * @param _encodedCall ABI encoded call data (signature and parameters).
     *      You should use `encodedCall` parameter from `GovernanceCallTimelocked` event.
     */
    function executeGovernanceCall(bytes calldata _encodedCall) external;

    /**
     * Cancel a timelocked governance call before it has been executed.
     * @dev Only governance can call this method.
     * @param _encodedCall ABI encoded call data (signature and parameters).
     *      You should use `encodedCall` parameter from `GovernanceCallTimelocked` event.
     */
    function cancelGovernanceCall(bytes calldata _encodedCall) external;

    /**
     * Enter the production mode after all the initial governance settings have been set.
     * This enables timelocks and the governance is afterwards obtained by calling
     * `governanceSettings.getGovernanceAddress()`.
     */
    function switchToProductionMode() external;

    /**
     * Returns the governance settings contract address.
     */
    function governanceSettings() external view returns (IGovernanceSettings);

    /**
     * True after switching to production mode (see `switchToProductionMode()`).
     */
    function productionMode() external view returns (bool);

    /**
     * Returns the current effective governance address.
     * Before switching to production, the effective governance is `initialGovernance`,
     * and afterwards it is `governanceSettings.getGovernanceAddress()`.
     */
    function governance() external view returns (address);

    /**
     * Check if an address is one of the executors defined in `governanceSettings`.
     */
    function isExecutor(address _address) external view returns (bool);
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;


interface IAddressUpdatable {

    error AUAddressZero();
    error OnlyAddressUpdater();

    /**
     * Return the address updater managing this contract.
     */
    function getAddressUpdater()
        external view
        returns (address);
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IIAddressUpdatable}
    from "@flarenetwork/flare-periphery-contracts/flare/addressUpdater/interfaces/IIAddressUpdatable.sol";
import {IAddressUpdatable} from "../interfaces/IAddressUpdatable.sol";


abstract contract AddressUpdatable is IAddressUpdatable, IIAddressUpdatable {

    // https://docs.soliditylang.org/en/v0.8.7/contracts.html#constant-and-immutable-state-variables
    // No storage slot is allocated
    bytes32 internal constant ADDRESS_STORAGE_POSITION =
        keccak256("flare.diamond.AddressUpdatable.ADDRESS_STORAGE_POSITION");

    modifier onlyAddressUpdater() {
        require (msg.sender == getAddressUpdater(), OnlyAddressUpdater());
        _;
    }

    constructor(address _addressUpdater) {
        setAddressUpdaterValue(_addressUpdater);
    }

    function getAddressUpdater() public view returns (address _addressUpdater) {
        // Only direct constants are allowed in inline assembly, so we assign it here
        bytes32 position = ADDRESS_STORAGE_POSITION;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _addressUpdater := sload(position)
        }
    }

    /**
     * @notice external method called from AddressUpdater only
     */
    function updateContractAddresses(
        bytes32[] memory _contractNameHashes,
        address[] memory _contractAddresses
    )
        external override
        onlyAddressUpdater
    {
        // update addressUpdater address
        setAddressUpdaterValue(_getContractAddress(_contractNameHashes, _contractAddresses, "AddressUpdater"));
        // update all other addresses
        _updateContractAddresses(_contractNameHashes, _contractAddresses);
    }

    /**
     * @notice virtual method that a contract extending AddressUpdatable must implement
     */
    function _updateContractAddresses(
        bytes32[] memory _contractNameHashes,
        address[] memory _contractAddresses
    ) internal virtual;

    /**
     * @notice helper method to get contract address
     * @dev it reverts if contract name does not exist
     */
    function _getContractAddress(
        bytes32[] memory _nameHashes,
        address[] memory _addresses,
        string memory _nameToFind
    )
        internal pure
        returns(address)
    {
        bytes32 nameHash = keccak256(abi.encode(_nameToFind));
        address a = address(0);
        for (uint256 i = 0; i < _nameHashes.length; i++) {
            if (nameHash == _nameHashes[i]) {
                a = _addresses[i];
                break;
            }
        }
        require(a != address(0), AUAddressZero());
        return a;
    }

    function setAddressUpdaterValue(address _addressUpdater) internal {
        // Only direct constants are allowed in inline assembly, so we assign it here
        bytes32 position = ADDRESS_STORAGE_POSITION;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(position, _addressUpdater)
        }
    }
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


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

/**
 * A special contract that holds Flare governance address.
 * This contract enables updating governance address and timelock only by hard forking the network,
 * meaning only by updating validator code.
 */
interface IGovernanceSettings {
    /**
     * Get the governance account address.
     * The governance address can only be changed by a hardfork.
     */
    function getGovernanceAddress() external view returns (address);

    /**
     * Get the time in seconds that must pass between a governance call and execution.
     * The timelock value can only be changed by a hardfork.
     */
    function getTimelock() external view returns (uint256);

    /**
     * Get the addresses of the accounts that are allowed to execute the timelocked governance calls
     * once the timelock period expires.
     * Executors can be changed without a hardfork, via a normal governance call.
     */
    function getExecutors() external view returns (address[] memory);

    /**
     * Check whether an address is one of the executors.
     */
    function isExecutor(address _address) external view returns (bool);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

interface IIAddressUpdatable {
    /**
     * @notice Updates contract addresses - should be called only from AddressUpdater contract
     * @param _contractNameHashes       list of keccak256(abi.encode(...)) contract names
     * @param _contractAddresses        list of contract addresses corresponding to the contract names
     */
    function updateContractAddresses(
        bytes32[] memory _contractNameHashes,
        address[] memory _contractAddresses
    ) external;
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

interface IIAddressUpdater {
    /**
     * @notice Returns all contract names and corresponding addresses
     */
    function getContractNamesAndAddresses()
        external
        view
        returns (
            string[] memory _contractNames,
            address[] memory _contractAddresses
        );

    /**
     * @notice Returns contract address for the given name - might be address(0)
     * @param _name             name of the contract
     */
    function getContractAddress(
        string calldata _name
    ) external view returns (address);

    /**
     * @notice Returns contract address for the given name hash - might be address(0)
     * @param _nameHash         hash of the contract name (keccak256(abi.encode(name))
     */
    function getContractAddressByHash(
        bytes32 _nameHash
    ) external view returns (address);

    /**
     * @notice Returns contract addresses for the given names - might be address(0)
     * @param _names            names of the contracts
     */
    function getContractAddresses(
        string[] calldata _names
    ) external view returns (address[] memory);

    /**
     * @notice Returns contract addresses for the given name hashes - might be address(0)
     * @param _nameHashes       hashes of the contract names (keccak256(abi.encode(name))
     */
    function getContractAddressesByHash(
        bytes32[] calldata _nameHashes
    ) external view returns (address[] memory);
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

import "../../IRelay.sol";

/**
 * Relay internal interface.
 */
interface IIRelay is IRelay {
    struct SigningPolicy {
        uint24 rewardEpochId; // Reward epoch id.
        uint32 startVotingRoundId; // First voting round id of validity.
        // Usually it is the first voting round of reward epoch rID.
        // It can be later,
        // if the confirmation of the signing policy on Flare blockchain gets delayed.
        uint16 threshold; // Confirmation threshold (absolute value of noramalised weights).
        uint256 seed; // Random seed.
        address[] voters; // The list of eligible voters in the canonical order.
        uint16[] weights; // The corresponding list of normalised signing weights of eligible voters.
        // Normalisation is done by compressing the weights from 32-byte values to 2 bytes,
        // while approximately keeping the weight relations.
    }

    /**
     * Sets the signing policy.
     * @param _signingPolicy Signing policy.
     * @return Returns signing policy hash.
     * @dev This method can only be called by the signing policy setter.
     */
    function setSigningPolicy(
        SigningPolicy memory _signingPolicy
    ) external returns (bytes32);
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

import {IAgentPing} from "../../userInterfaces/IAgentPing.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {Agent} from "../../assetManager/library/data/Agent.sol";


contract AgentPingFacet is AssetManagerBase, IAgentPing {
    /**
     * @inheritdoc IAgentPing
     */
    function agentPing(address _agentVault, uint256 _query) external {
        emit AgentPing(_agentVault, msg.sender, _query);
    }

    /**
     * @inheritdoc IAgentPing
     */
    function agentPingResponse(address _agentVault, uint256 _query, string memory _response)
        external
        onlyAgentVaultOwner(_agentVault)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        emit AgentPingResponse(_agentVault, agent.ownerManagementAddress, _query, _response);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {Globals} from "../library/Globals.sol";
import {SettingsUpdater} from "../library/SettingsUpdater.sol";
import {RedemptionTimeExtension} from "../library/data/RedemptionTimeExtension.sol";
import {LibDiamond} from "../../diamond/library/LibDiamond.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {IRedemptionTimeExtension} from "../../userInterfaces/IRedemptionTimeExtension.sol";


contract RedemptionTimeExtensionFacet is AssetManagerBase, IRedemptionTimeExtension {

    error ValueMustBeNonzero();
    error DecreaseTooBig();
    error IncreaseTooBig();
    error AlreadyInitialized();
    error DiamondNotInitialized();

    constructor() {
        // implementation initialization - to prevent reinitialization
        RedemptionTimeExtension.setRedemptionPaymentExtensionSeconds(1);
    }

    // this method is not accessible through diamond proxy
    // it is only used for initialization when the contract is added after proxy deploy
    function initRedemptionTimeExtensionFacet(uint256 _redemptionPaymentExtensionSeconds)
        external
    {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        require(ds.supportedInterfaces[type(IERC165).interfaceId], DiamondNotInitialized());
        ds.supportedInterfaces[type(IRedemptionTimeExtension).interfaceId] = true;
        require(RedemptionTimeExtension.redemptionPaymentExtensionSeconds() == 0, AlreadyInitialized());
        // init settings
        RedemptionTimeExtension.setRedemptionPaymentExtensionSeconds(_redemptionPaymentExtensionSeconds);
    }

    function setRedemptionPaymentExtensionSeconds(uint256 _value)
        external
        onlyAssetManagerController
    {
        SettingsUpdater.checkEnoughTimeSinceLastUpdate();
        // validate
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        uint256 currentValue = RedemptionTimeExtension.redemptionPaymentExtensionSeconds();
        require(_value <= currentValue * 4 + settings.averageBlockTimeMS / 1000, IncreaseTooBig());
        require(_value >= currentValue / 4, DecreaseTooBig());
        require(_value > 0, ValueMustBeNonzero());
        // update
        RedemptionTimeExtension.setRedemptionPaymentExtensionSeconds(_value);
        emit IAssetManagerEvents.SettingChanged("redemptionPaymentExtensionSeconds", _value);
    }

    function redemptionPaymentExtensionSeconds()
        external view
        returns (uint256)
    {
        return RedemptionTimeExtension.redemptionPaymentExtensionSeconds();
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IPayment} from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {ICoreVaultClient} from "../../userInterfaces/ICoreVaultClient.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {ReentrancyGuard} from "../../openzeppelin/security/ReentrancyGuard.sol";
import {Conversion} from "../library/Conversion.sol";
import {CoreVaultClient} from "../library/CoreVaultClient.sol";
import {Agent} from "../library/data/Agent.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {AssetManagerState} from "../library/data/AssetManagerState.sol";
import {PaymentReference} from "../library/data/PaymentReference.sol";
import {AgentCollateral} from "../library/AgentCollateral.sol";
import {Redemptions} from "../library/Redemptions.sol";
import {RedemptionRequests} from "../library/RedemptionRequests.sol";
import {UnderlyingBalance} from "../library/UnderlyingBalance.sol";
import {Collateral} from "../library/data/Collateral.sol";
import {PaymentConfirmations} from "../library/data/PaymentConfirmations.sol";
import {AgentBacking} from "../library/AgentBacking.sol";
import {SafeMath64} from "../../utils/library/SafeMath64.sol";
import {TransactionAttestation} from "../library/TransactionAttestation.sol";
import {UnderlyingBlockUpdater} from "../library/UnderlyingBlockUpdater.sol";


contract CoreVaultClientFacet is AssetManagerBase, ReentrancyGuard, ICoreVaultClient {
    using SafePct for uint256;
    using SafeCast for uint256;
    using SafeCast for int256;
    using AgentCollateral for Collateral.CombinedData;
    using PaymentConfirmations for PaymentConfirmations.State;

    error CannotReturnZeroLots();
    error InvalidAgentStatus();
    error InvalidPaymentReference();
    error NoActiveReturnRequest();
    error NotEnoughAvailableOnCoreVault();
    error NotEnoughFreeCollateral();
    error NotEnoughUnderlying();
    error NothingMinted();
    error PaymentNotFromCoreVault();
    error PaymentNotToAgentsAddress();
    error RequestedAmountTooSmall();
    error ReturnFromCoreVaultAlreadyRequested();
    error TooLittleMintingLeftAfterTransfer();
    error TransferAlreadyActive();
    error ZeroTransferNotAllowed();

    // core vault may not be enabled on all chains
    modifier onlyEnabled {
        CoreVaultClient.checkEnabled();
        _;
    }

    // prevent initialization of implementation contract
    constructor() {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        state.initialized = true;
    }

    /**
     * Agent can transfer their backing to core vault.
     * They then get a redemption requests which the owner pays just like any other redemption request.
     * After that, the agent's collateral is released.
     * NOTE: only agent vault owner can call
     * @param _agentVault the agent vault address
     * @param _amountUBA the amount to transfer to the core vault
     */
    function transferToCoreVault(
        address _agentVault,
        uint256 _amountUBA
    )
        external
        onlyEnabled
        notEmergencyPaused
        nonReentrant
        onlyAgentVaultOwner(_agentVault)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        // for agent in full liquidation, the system cannot know if there is enough underlying for the transfer
        require(agent.status != Agent.Status.FULL_LIQUIDATION, InvalidAgentStatus());
        // forbid 0 transfer
        require(_amountUBA > 0, ZeroTransferNotAllowed());
        // agent must have enough underlying for the transfer (if the required backing < 100%, they may have less)
        require(_amountUBA.toInt256() <= agent.underlyingBalanceUBA, NotEnoughUnderlying());
        // only one transfer can be active
        require(agent.activeTransferToCoreVault == 0, TransferAlreadyActive());
        // close agent's redemption tickets
        uint64 amountAMG = Conversion.convertUBAToAmg(_amountUBA);
        (uint64 transferredAMG,) = Redemptions.closeTickets(agent, amountAMG, false);
        require(transferredAMG > 0, NothingMinted());
        // check the remaining amount
        (uint256 maximumTransferAMG,) = CoreVaultClient.maximumTransferToCoreVaultAMG(agent);
        require(transferredAMG <= maximumTransferAMG, TooLittleMintingLeftAfterTransfer());
        // create ordinary redemption request to core vault address
        string memory underlyingAddress = state.coreVaultManager.coreVaultAddress();
        // NOTE: there will be no redemption fee, so the agent needs enough free underlying for the
        // underlying transaction fee, otherwise they will go into full liquidation
        uint64 redemptionRequestId = RedemptionRequests.createRedemptionRequest(
            RedemptionRequests.AgentRedemptionData(_agentVault, transferredAMG),
            state.nativeAddress, underlyingAddress, false, payable(address(0)), 0,
            state.transferTimeExtensionSeconds, true);
        // set the active request
        agent.activeTransferToCoreVault = redemptionRequestId;
        // send event
        uint256 transferredUBA = Conversion.convertAmgToUBA(transferredAMG);
        emit TransferToCoreVaultStarted(_agentVault, redemptionRequestId, transferredUBA);
    }

    /**
     * Request that core vault transfers funds to the agent's underlying address,
     * which makes them available for redemptions. This method reserves agent's collateral.
     * This may be sent by an agent when redemptions dominate mintings, so that the agents
     * are empty but want to earn from redemptions.
     * NOTE: only agent vault owner can call
     * NOTE: there can be only one active return request (until it is confirmed or cancelled).
     * @param _agentVault the agent vault address
     * @param _lots number of lots (same lots as for minting and redemptions)
     */
    function requestReturnFromCoreVault(
        address _agentVault,
        uint256 _lots
    )
        external
        onlyEnabled
        notEmergencyPaused
        nonReentrant
        onlyAgentVaultOwner(_agentVault)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        require(agent.activeReturnFromCoreVaultId == 0, ReturnFromCoreVaultAlreadyRequested());
        Collateral.CombinedData memory collateralData = AgentCollateral.combinedData(agent);
        require(_lots > 0, CannotReturnZeroLots());
        require(agent.status == Agent.Status.NORMAL, InvalidAgentStatus());
        require(collateralData.freeCollateralLotsOptionalFee(agent, false) >= _lots, NotEnoughFreeCollateral());
        uint256 availableLots = CoreVaultClient.coreVaultAmountLots();
        require(_lots <= availableLots, NotEnoughAvailableOnCoreVault());
        // create new request id
        state.newTransferFromCoreVaultId += PaymentReference.randomizedIdSkip();
        uint64 requestId = state.newTransferFromCoreVaultId;
        agent.activeReturnFromCoreVaultId = requestId;
        // reserve collateral
        assert(agent.returnFromCoreVaultReservedAMG == 0);
        uint64 amountAMG = Conversion.convertLotsToAMG(_lots);
        agent.returnFromCoreVaultReservedAMG = amountAMG;
        agent.reservedAMG += amountAMG;
        // request
        bytes32 paymentReference = PaymentReference.returnFromCoreVault(requestId);
        uint128 amountUBA = Conversion.convertAmgToUBA(amountAMG).toUint128();
        state.coreVaultManager.requestTransferFromCoreVault(
            agent.underlyingAddressString, paymentReference, amountUBA, true);
        emit ReturnFromCoreVaultRequested(_agentVault, requestId, paymentReference, amountUBA);
    }

    /**
     * Before the return request is processed, it can be cancelled, releasing the agent's reserved collateral.
     * @param _agentVault the agent vault address
     */
    function cancelReturnFromCoreVault(
        address _agentVault
    )
        external
        onlyEnabled
        nonReentrant
        onlyAgentVaultOwner(_agentVault)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        uint256 requestId = agent.activeReturnFromCoreVaultId;
        require(requestId != 0, NoActiveReturnRequest());
        state.coreVaultManager.cancelTransferRequestFromCoreVault(agent.underlyingAddressString);
        CoreVaultClient.deleteReturnFromCoreVaultRequest(agent);
        emit ReturnFromCoreVaultCancelled(_agentVault, requestId);
    }

    /**
     * Confirm the payment from core vault to the agent's underlying address.
     * This adds the reserved funds to the agent's backing.
     * @param _payment FDC payment proof
     * @param _agentVault the agent vault address
     */
    function confirmReturnFromCoreVault(
        IPayment.Proof calldata _payment,
        address _agentVault
    )
        external
        onlyEnabled
        nonReentrant
        onlyAgentVaultOwner(_agentVault)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        TransactionAttestation.verifyPaymentSuccess(_payment);
        uint64 requestId = agent.activeReturnFromCoreVaultId;
        require(requestId != 0, NoActiveReturnRequest());
        require(_payment.data.responseBody.sourceAddressHash == state.coreVaultManager.coreVaultAddressHash(),
            PaymentNotFromCoreVault());
        require(_payment.data.responseBody.receivingAddressHash == agent.underlyingAddressHash,
            PaymentNotToAgentsAddress());
        require(_payment.data.responseBody.standardPaymentReference == PaymentReference.returnFromCoreVault(requestId),
            InvalidPaymentReference());
        // make sure payment isn't used again
        AssetManagerState.get().paymentConfirmations.confirmIncomingPayment(_payment);
        // we account for the option that CV pays more or less than the reserved amount:
        // - if less, only the amount received gets converted to redemption ticket
        // - if more, the extra amount becomes the agent's free underlying
        uint256 receivedAmountUBA = _payment.data.responseBody.receivedAmount.toUint256();
        uint64 receivedAmountAMG = Conversion.convertUBAToAmg(receivedAmountUBA);
        uint64 remintedAMG = SafeMath64.min64(agent.returnFromCoreVaultReservedAMG, receivedAmountAMG);
        // create redemption ticket
        AgentBacking.createNewMinting(agent, remintedAMG);
        // update underlying amount
        UnderlyingBalance.increaseBalance(agent, receivedAmountUBA);
        // update underlying block
        UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(_payment);
        // clear the reservation
        CoreVaultClient.deleteReturnFromCoreVaultRequest(agent);
        // send event
        uint256 remintedUBA = Conversion.convertAmgToUBA(remintedAMG);
        emit ReturnFromCoreVaultConfirmed(_agentVault, requestId, receivedAmountUBA, remintedUBA);
    }

    /**
     * Directly redeem from core vault by a user holding FAssets.
     * This is like ordinary redemption, but the redemption time is much longer (a day or more)
     * and there is no possibility of redemption.
     * @param _lots the number of lots, must be larger than `coreVaultMinimumRedeemLots` setting
     * @param _redeemerUnderlyingAddress the underlying address to which the assets will be redeemed;
     *      must have been added to the `allowedDestinations` list in the core vault manager by
     *      the governance before the redemption request.
     */
    function redeemFromCoreVault(
        uint256 _lots,
        string memory _redeemerUnderlyingAddress
    )
        external
        onlyEnabled
        notEmergencyPaused
        nonReentrant
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        uint256 availableLots = CoreVaultClient.coreVaultAmountLots();
        require(_lots <= availableLots, NotEnoughAvailableOnCoreVault());
        uint256 minimumRedeemLots = Math.min(state.minimumRedeemLots, availableLots);
        require(_lots >= minimumRedeemLots, RequestedAmountTooSmall());
        // burn the senders fassets
        uint256 redeemedUBA = Conversion.convertLotsToUBA(_lots);
        Redemptions.burnFAssets(msg.sender, redeemedUBA);
        // subtract the redemption fee
        uint256 redemptionFeeUBA = redeemedUBA.mulBips(state.redemptionFeeBIPS);
        uint128 paymentUBA = (redeemedUBA - redemptionFeeUBA).toUint128();
        // create new request id
        state.newRedemptionFromCoreVaultId += PaymentReference.randomizedIdSkip();
        bytes32 paymentReference = PaymentReference.redemptionFromCoreVault(state.newRedemptionFromCoreVaultId);
        // transfer from core vault (paymentReference may change when the request is merged)
        paymentReference = state.coreVaultManager.requestTransferFromCoreVault(
            _redeemerUnderlyingAddress, paymentReference, paymentUBA, false);
        emit CoreVaultRedemptionRequested(msg.sender, _redeemerUnderlyingAddress, paymentReference,
            redeemedUBA, redemptionFeeUBA);
    }

    function maximumTransferToCoreVault(
        address _agentVault
    )
        external view
        returns (uint256 _maximumTransferUBA, uint256 _minimumLeftAmountUBA)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        (uint256 _maximumTransferAMG, uint256 _minimumLeftAmountAMG) =
             CoreVaultClient.maximumTransferToCoreVaultAMG(agent);
        _maximumTransferUBA = Conversion.convertAmgToUBA(_maximumTransferAMG.toUint64());
        _minimumLeftAmountUBA = Conversion.convertAmgToUBA(_minimumLeftAmountAMG.toUint64());
    }

    function coreVaultAvailableAmount()
        external view
        returns (uint256 _immediatelyAvailableUBA, uint256 _totalAvailableUBA)
    {
        return CoreVaultClient.coreVaultAvailableAmount();
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {Globals} from "../library/Globals.sol";
import {AssetManagerState} from "../library/data/AssetManagerState.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";


contract EmergencyPauseTransfersFacet is AssetManagerBase, IAssetManagerEvents {
    using SafeCast for uint256;

    error PausedByGovernance();

    function emergencyPauseTransfers(bool _byGovernance, uint256 _duration)
        external
        onlyAssetManagerController
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        bool pausedAtStart = _transfersPaused();
        if (_byGovernance) {
            state.transfersEmergencyPausedUntil = (block.timestamp + _duration).toUint64();
            state.transfersEmergencyPausedByGovernance = true;
        } else {
            if (pausedAtStart && state.transfersEmergencyPausedByGovernance) {
                revert PausedByGovernance();
            }
            AssetManagerSettings.Data storage settings = Globals.getSettings();
            uint256 resetTs = state.transfersEmergencyPausedUntil + settings.emergencyPauseDurationResetAfterSeconds;
            if (resetTs <= block.timestamp) {
                state.transfersEmergencyPausedTotalDuration = 0;
            }
            uint256 currentPauseEndTime = Math.max(state.transfersEmergencyPausedUntil, block.timestamp);
            uint256 projectedStartTime =
                Math.min(currentPauseEndTime - state.transfersEmergencyPausedTotalDuration, block.timestamp);
            uint256 maxEndTime = projectedStartTime + settings.maxEmergencyPauseDurationSeconds;
            uint256 endTime = Math.min(block.timestamp + _duration, maxEndTime);
            state.transfersEmergencyPausedUntil = endTime.toUint64();
            state.transfersEmergencyPausedTotalDuration = (endTime - projectedStartTime).toUint64();
            state.transfersEmergencyPausedByGovernance = false;
        }
        if (_transfersPaused()) {
            emit EmergencyPauseTransfersTriggered(state.transfersEmergencyPausedUntil);
        } else if (pausedAtStart) {
            emit EmergencyPauseTransfersCanceled();
        }
    }

    function resetEmergencyPauseTransfersTotalDuration()
        external
        onlyAssetManagerController
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        state.transfersEmergencyPausedTotalDuration = 0;
    }

    function transfersEmergencyPaused()
        external view
        returns (bool)
    {
        return _transfersPaused();
    }

    function transfersEmergencyPausedUntil()
        external view
        returns (uint256)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return _transfersPaused() ? state.transfersEmergencyPausedUntil : 0;
    }

    function emergencyPauseTransfersDetails()
        external view
        returns (uint256 _pausedUntil, uint256 _totalPauseDuration, bool _pausedByGovernance)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return (state.transfersEmergencyPausedUntil, state.transfersEmergencyPausedTotalDuration,
            state.transfersEmergencyPausedByGovernance);
    }

    function _transfersPaused() private view returns (bool) {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.transfersEmergencyPausedUntil > block.timestamp;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {ICoreVaultManager} from "../../userInterfaces/ICoreVaultManager.sol";

/**
 * Core vault manager internal interface
 */
interface IICoreVaultManager is ICoreVaultManager {

    /**
     * Requests transfer from core vault to destination address.
     * @param _destinationAddress destination address
     * @param _paymentReference payment reference
     * @param _amount amount
     * @param _cancelable cancelable flag (if true, the request can be canceled)
     * @return _actualPaymentReference the actual payment reference that will be used - for non-cancelable requests
     *  it can differ from the requested payment reference, because multiple queued payments to the same address
     *  are merged in which case the reference of the previous payment to the same address will be used
     * NOTE: destination address must be allowed otherwise the request will revert.
     * NOTE: may only be called by the asset manager.
     */
    function requestTransferFromCoreVault(
        string memory _destinationAddress,
        bytes32 _paymentReference,
        uint128 _amount,
        bool _cancelable
    )
        external
        returns (bytes32 _actualPaymentReference);

    /**
     * Cancels transfer request from core vault.
     * @param _destinationAddress destination address
     * NOTE: if the request does not exist (anymore), the call will revert.
     * NOTE: may only be called by the asset manager.
     */
    function cancelTransferRequestFromCoreVault(
        string memory _destinationAddress
    )
        external;
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

import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {Globals} from "../library/Globals.sol";
import {AssetManagerState} from "../library/data/AssetManagerState.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";


contract EmergencyPauseFacet is AssetManagerBase, IAssetManagerEvents {
    using SafeCast for uint256;

    error PausedByGovernance();

    function emergencyPause(bool _byGovernance, uint256 _duration)
        external
        onlyAssetManagerController
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        bool pausedAtStart = _paused();
        if (_byGovernance) {
            state.emergencyPausedUntil = (block.timestamp + _duration).toUint64();
            state.emergencyPausedByGovernance = true;
        } else {
            if (pausedAtStart && state.emergencyPausedByGovernance) {
                revert PausedByGovernance();
            }
            AssetManagerSettings.Data storage settings = Globals.getSettings();
            if (state.emergencyPausedUntil + settings.emergencyPauseDurationResetAfterSeconds <= block.timestamp) {
                state.emergencyPausedTotalDuration = 0;
            }
            uint256 currentPauseEndTime = Math.max(state.emergencyPausedUntil, block.timestamp);
            uint256 projectedStartTime =
                Math.min(currentPauseEndTime - state.emergencyPausedTotalDuration, block.timestamp);
            uint256 maxEndTime = projectedStartTime + settings.maxEmergencyPauseDurationSeconds;
            uint256 endTime = Math.min(block.timestamp + _duration, maxEndTime);
            state.emergencyPausedUntil = endTime.toUint64();
            state.emergencyPausedTotalDuration = (endTime - projectedStartTime).toUint64();
            state.emergencyPausedByGovernance = false;
        }
        if (_paused()) {
            emit EmergencyPauseTriggered(state.emergencyPausedUntil);
        } else if (pausedAtStart) {
            emit EmergencyPauseCanceled();
        }
    }

    function resetEmergencyPauseTotalDuration()
        external
        onlyAssetManagerController
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        state.emergencyPausedTotalDuration = 0;
    }

    function emergencyPaused()
        external view
        returns (bool)
    {
        return _paused();
    }

    function emergencyPausedUntil()
        external view
        returns (uint256)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return _paused() ? state.emergencyPausedUntil : 0;
    }

    function emergencyPauseDetails()
        external view
        returns (uint256 _pausedUntil, uint256 _totalPauseDuration, bool _pausedByGovernance)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return (state.emergencyPausedUntil, state.emergencyPausedTotalDuration, state.emergencyPausedByGovernance);
    }

    function _paused() private view returns (bool) {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.emergencyPausedUntil > block.timestamp;
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
pragma solidity ^0.8.0;
/******************************************************************************\
* Author: Nick Mudge <nick@perfectabstractions.com> (https://twitter.com/mudgen)
* EIP-2535 Diamonds: https://eips.ethereum.org/EIPS/eip-2535
/******************************************************************************/

// The functions in DiamondLoupeFacet MUST be added to a diamond.
// The EIP-2535 Diamond standard requires these functions.

import { IERC165 } from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import { IDiamondLoupe } from "../interfaces/IDiamondLoupe.sol";
import { LibDiamond } from  "../library/LibDiamond.sol";

// solhint-disable no-inline-assembly
contract DiamondLoupeFacet is IDiamondLoupe, IERC165 {
    // Diamond Loupe Functions
    ////////////////////////////////////////////////////////////////////
    /// These functions are expected to be called frequently by tools.
    //
    // struct Facet {
    //     address facetAddress;
    //     bytes4[] functionSelectors;
    // }

    /// @notice Gets all facets and their selectors.
    /// @return facets_ Facet
    function facets()
        external override view
        returns (Facet[] memory facets_)
    {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        uint256 selectorCount = ds.selectors.length;
        // create an array set to the maximum size possible
        facets_ = new Facet[](selectorCount);
        // create an array for counting the number of selectors for each facet
        uint16[] memory numFacetSelectors = new uint16[](selectorCount);
        // total number of facets
        uint256 numFacets;
        // loop through function selectors
        for (uint256 selectorIndex; selectorIndex < selectorCount; selectorIndex++) {
            bytes4 selector = ds.selectors[selectorIndex];
            address facetAddress_ = ds.facetAddressAndSelectorPosition[selector].facetAddress;
            bool continueLoop = false;
            // find the functionSelectors array for selector and add selector to it
            for (uint256 facetIndex; facetIndex < numFacets; facetIndex++) {
                if (facets_[facetIndex].facetAddress == facetAddress_) {
                    facets_[facetIndex].functionSelectors[numFacetSelectors[facetIndex]] = selector;
                    numFacetSelectors[facetIndex]++;
                    continueLoop = true;
                    break;
                }
            }
            // if functionSelectors array exists for selector then continue loop
            if (continueLoop) {
                continueLoop = false;
                continue;
            }
            // create a new functionSelectors array for selector
            facets_[numFacets].facetAddress = facetAddress_;
            facets_[numFacets].functionSelectors = new bytes4[](selectorCount);
            facets_[numFacets].functionSelectors[0] = selector;
            numFacetSelectors[numFacets] = 1;
            numFacets++;
        }
        for (uint256 facetIndex; facetIndex < numFacets; facetIndex++) {
            uint256 numSelectors = numFacetSelectors[facetIndex];
            bytes4[] memory selectors = facets_[facetIndex].functionSelectors;
            // setting the number of selectors
            assembly {
                mstore(selectors, numSelectors)
            }
        }
        // setting the number of facets
        assembly {
            mstore(facets_, numFacets)
        }
    }

    /// @notice Gets all the function selectors supported by a specific facet.
    /// @param _facet The facet address.
    /// @return _facetFunctionSelectors The selectors associated with a facet address.
    function facetFunctionSelectors(address _facet)
        external override view
        returns (bytes4[] memory _facetFunctionSelectors)
    {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        uint256 selectorCount = ds.selectors.length;
        uint256 numSelectors;
        _facetFunctionSelectors = new bytes4[](selectorCount);
        // loop through function selectors
        for (uint256 selectorIndex; selectorIndex < selectorCount; selectorIndex++) {
            bytes4 selector = ds.selectors[selectorIndex];
            address facetAddress_ = ds.facetAddressAndSelectorPosition[selector].facetAddress;
            if (_facet == facetAddress_) {
                _facetFunctionSelectors[numSelectors] = selector;
                numSelectors++;
            }
        }
        // Set the number of selectors in the array
        assembly {
            mstore(_facetFunctionSelectors, numSelectors)
        }
    }

    /// @notice Get all the facet addresses used by a diamond.
    /// @return facetAddresses_
    function facetAddresses()
        external override view
        returns (address[] memory facetAddresses_)
    {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        uint256 selectorCount = ds.selectors.length;
        // create an array set to the maximum size possible
        facetAddresses_ = new address[](selectorCount);
        uint256 numFacets;
        // loop through function selectors
        for (uint256 selectorIndex; selectorIndex < selectorCount; selectorIndex++) {
            bytes4 selector = ds.selectors[selectorIndex];
            address facetAddress_ = ds.facetAddressAndSelectorPosition[selector].facetAddress;
            bool continueLoop = false;
            // see if we have collected the address already and break out of loop if we have
            for (uint256 facetIndex; facetIndex < numFacets; facetIndex++) {
                if (facetAddress_ == facetAddresses_[facetIndex]) {
                    continueLoop = true;
                    break;
                }
            }
            // continue loop if we already have the address
            if (continueLoop) {
                continueLoop = false;
                continue;
            }
            // include address
            facetAddresses_[numFacets] = facetAddress_;
            numFacets++;
        }
        // Set the number of facet addresses in the array
        assembly {
            mstore(facetAddresses_, numFacets)
        }
    }

    /// @notice Gets the facet address that supports the given selector.
    /// @dev If facet is not found return address(0).
    /// @param _functionSelector The function selector.
    /// @return facetAddress_ The facet address.
    function facetAddress(bytes4 _functionSelector)
        external override view
        returns (address facetAddress_)
    {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        facetAddress_ = ds.facetAddressAndSelectorPosition[_functionSelector].facetAddress;
    }

    // This implements ERC-165.
    function supportsInterface(bytes4 _interfaceId)
        external override view
        returns (bool)
    {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        return ds.supportedInterfaces[_interfaceId];
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

import {Diamond} from "../../diamond/implementation/Diamond.sol";
import {LibDiamond} from "../../diamond/library/LibDiamond.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {IDiamondCut} from "../../diamond/interfaces/IDiamondCut.sol";

/**
 * The contract that can mint and burn f-assets while managing collateral and backing funds.
 * There is one instance of AssetManager per f-asset type.
 */
contract AssetManager is Diamond, IAssetManagerEvents {
    // IAssetManagerEvents interface is included so that blockchain explorers will be able
    // to decode events for verified AssetManager instances.
    constructor(IDiamondCut.FacetCut[] memory _diamondCut, address _init, bytes memory _initCalldata) payable {
        LibDiamond.diamondCut(_diamondCut, _init, _initCalldata);
    }
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

