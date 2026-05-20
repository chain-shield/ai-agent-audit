
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { InitializeGovernedUpgradeabilityProxy } from "./InitializeGovernedUpgradeabilityProxy.sol";
import { InitializeGovernedUpgradeabilityProxy2 } from "./InitializeGovernedUpgradeabilityProxy2.sol";

/**
 * @notice OUSDProxy delegates calls to an OUSD implementation
 */
contract OUSDProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice WrappedOUSDProxy delegates calls to a WrappedOUSD implementation
 */
contract WrappedOUSDProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice VaultProxy delegates calls to a Vault implementation
 */
contract VaultProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice HarvesterProxy delegates calls to a Harvester implementation
 */
contract HarvesterProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice DripperProxy delegates calls to a Dripper implementation
 */
contract DripperProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice OETHProxy delegates calls to nowhere for now
 */
contract OETHProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice WOETHProxy delegates calls to nowhere for now
 */
contract WOETHProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice OETHVaultProxy delegates calls to a Vault implementation
 */
contract OETHVaultProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice OETHDripperProxy delegates calls to a OETHDripper implementation
 */
contract OETHDripperProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice OETHHarvesterProxy delegates calls to a Harvester implementation
 */
contract OETHHarvesterProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice OETHMorphoAaveStrategyProxy delegates calls to a MorphoAaveStrategy implementation
 */
contract OETHMorphoAaveStrategyProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice OETHBalancerMetaPoolrEthStrategyProxy delegates calls to a BalancerMetaPoolStrategy implementation
 */
contract OETHBalancerMetaPoolrEthStrategyProxy is
    InitializeGovernedUpgradeabilityProxy
{

}

/**
 * @notice OETHBalancerMetaPoolwstEthStrategyProxy delegates calls to a BalancerMetaPoolStrategy implementation
 */
contract OETHBalancerMetaPoolwstEthStrategyProxy is
    InitializeGovernedUpgradeabilityProxy
{

}

/**
 * @notice MakerDsrStrategyProxy delegates calls to a Generalized4626Strategy implementation
 */
contract MakerDsrStrategyProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice BridgedWOETHProxy delegates calls to BridgedWOETH implementation
 */
contract BridgedWOETHProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice NativeStakingSSVStrategyProxy delegates calls to NativeStakingSSVStrategy implementation
 */
contract NativeStakingSSVStrategyProxy is
    InitializeGovernedUpgradeabilityProxy
{

}

/**
 * @notice NativeStakingFeeAccumulatorProxy delegates calls to FeeAccumulator implementation
 */
contract NativeStakingFeeAccumulatorProxy is
    InitializeGovernedUpgradeabilityProxy
{

}

/**
 * @notice NativeStakingSSVStrategy2Proxy delegates calls to NativeStakingSSVStrategy implementation
 */
contract NativeStakingSSVStrategy2Proxy is
    InitializeGovernedUpgradeabilityProxy
{

}

/**
 * @notice NativeStakingFeeAccumulator2Proxy delegates calls to FeeAccumulator implementation
 */
contract NativeStakingFeeAccumulator2Proxy is
    InitializeGovernedUpgradeabilityProxy
{

}

/**
 * @notice NativeStakingSSVStrategy3Proxy delegates calls to NativeStakingSSVStrategy implementation
 */
contract NativeStakingSSVStrategy3Proxy is
    InitializeGovernedUpgradeabilityProxy
{

}

/**
 * @notice NativeStakingFeeAccumulator3Proxy delegates calls to FeeAccumulator implementation
 */
contract NativeStakingFeeAccumulator3Proxy is
    InitializeGovernedUpgradeabilityProxy
{

}

/**
 * @notice MetaMorphoStrategyProxy delegates calls to a Generalized4626Strategy implementation
 */
contract MetaMorphoStrategyProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice MorphoGauntletPrimeUSDCStrategyProxy delegates calls to a Generalized4626Strategy implementation
 */
contract MorphoGauntletPrimeUSDCStrategyProxy is
    InitializeGovernedUpgradeabilityProxy
{

}

/**
 * @notice CurvePoolBoosterProxy delegates calls to a CurvePoolBooster implementation
 */
contract CurvePoolBoosterProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice OETHFixedRateDripperProxy delegates calls to a OETHFixedRateDripper implementation
 */
contract OETHFixedRateDripperProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice OETHSimpleHarvesterProxy delegates calls to a OETHSimpleHarvester implementation
 */
contract OETHSimpleHarvesterProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice PoolBoostCentralRegistryProxy delegates calls to the PoolBoostCentralRegistry implementation
 */
contract PoolBoostCentralRegistryProxy is
    InitializeGovernedUpgradeabilityProxy
{

}

/**
 * @notice OUSDCurveAMOProxy delegates calls to a CurveAMOStrategy implementation
 */
contract OUSDCurveAMOProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice OETHCurveAMOProxy delegates calls to a CurveAMOStrategy implementation
 */
contract OETHCurveAMOProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice CompoundingStakingSSVStrategyProxy delegates calls to a CompoundingStakingSSVStrategy implementation
 */
contract CompoundingStakingSSVStrategyProxy is
    InitializeGovernedUpgradeabilityProxy
{

}

/**
 * @notice OUSDMorphoV2StrategyProxy delegates calls to a Generalized4626Strategy implementation
 */
contract OUSDMorphoV2StrategyProxy is InitializeGovernedUpgradeabilityProxy {

}

/**
 * @notice Legacy Supernova AMO proxy
 */
contract OETHSupernovaAMOProxy is InitializeGovernedUpgradeabilityProxy {

}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { Address } from "@openzeppelin/contracts/utils/Address.sol";

import { Governable } from "../governance/Governable.sol";

/**
 * @title BaseGovernedUpgradeabilityProxy
 * @dev This contract combines an upgradeability proxy with our governor system.
 * It is based on an older version of OpenZeppelins BaseUpgradeabilityProxy
 * with Solidity ^0.8.0.
 * @author Origin Protocol Inc
 */
contract InitializeGovernedUpgradeabilityProxy is Governable {
    /**
     * @dev Emitted when the implementation is upgraded.
     * @param implementation Address of the new implementation.
     */
    event Upgraded(address indexed implementation);

    constructor() {
        _setGovernor(msg.sender);
    }

    /**
     * @dev Contract initializer with Governor enforcement
     * @param _logic Address of the initial implementation.
     * @param _initGovernor Address of the initial Governor.
     * @param _data Data to send as msg.data to the implementation to initialize
     * the proxied contract.
     * It should include the signature and the parameters of the function to be
     * called, as described in
     * https://solidity.readthedocs.io/en/v0.4.24/abi-spec.html#function-selector-and-argument-encoding.
     * This parameter is optional, if no data is given the initialization call
     * to proxied contract will be skipped.
     */
    function initialize(
        address _logic,
        address _initGovernor,
        bytes calldata _data
    ) public payable onlyGovernor {
        require(_implementation() == address(0));
        require(_logic != address(0), "Implementation not set");
        assert(
            IMPLEMENTATION_SLOT ==
                bytes32(uint256(keccak256("eip1967.proxy.implementation")) - 1)
        );
        _setImplementation(_logic);
        if (_data.length > 0) {
            (bool success, ) = _logic.delegatecall(_data);
            require(success);
        }
        _changeGovernor(_initGovernor);
    }

    /**
     * @return The address of the proxy admin/it's also the governor.
     */
    function admin() external view returns (address) {
        return _governor();
    }

    /**
     * @return The address of the implementation.
     */
    function implementation() external view returns (address) {
        return _implementation();
    }

    /**
     * @dev Upgrade the backing implementation of the proxy.
     * Only the admin can call this function.
     * @param _newImplementation Address of the new implementation.
     */
    function upgradeTo(address _newImplementation) external onlyGovernor {
        _upgradeTo(_newImplementation);
    }

    /**
     * @dev Upgrade the backing implementation of the proxy and call a function
     * on the new implementation.
     * This is useful to initialize the proxied contract.
     * @param newImplementation Address of the new implementation.
     * @param data Data to send as msg.data in the low level call.
     * It should include the signature and the parameters of the function to be called, as described in
     * https://solidity.readthedocs.io/en/v0.4.24/abi-spec.html#function-selector-and-argument-encoding.
     */
    function upgradeToAndCall(address newImplementation, bytes calldata data)
        external
        payable
        onlyGovernor
    {
        _upgradeTo(newImplementation);
        (bool success, ) = newImplementation.delegatecall(data);
        require(success);
    }

    /**
     * @dev Fallback function.
     * Implemented entirely in `_fallback`.
     */
    fallback() external payable {
        _fallback();
    }

    /**
     * @dev Delegates execution to an implementation contract.
     * This is a low level function that doesn't return to its internal call site.
     * It will return to the external caller whatever the implementation returns.
     * @param _impl Address to delegate.
     */
    function _delegate(address _impl) internal {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            // Copy msg.data. We take full control of memory in this inline assembly
            // block because it will not return to Solidity code. We overwrite the
            // Solidity scratch pad at memory position 0.
            calldatacopy(0, 0, calldatasize())

            // Call the implementation.
            // out and outsize are 0 because we don't know the size yet.
            let result := delegatecall(gas(), _impl, 0, calldatasize(), 0, 0)

            // Copy the returned data.
            returndatacopy(0, 0, returndatasize())

            switch result
            // delegatecall returns 0 on error.
            case 0 {
                revert(0, returndatasize())
            }
            default {
                return(0, returndatasize())
            }
        }
    }

    /**
     * @dev Function that is run as the first thing in the fallback function.
     * Can be redefined in derived contracts to add functionality.
     * Redefinitions must call super._willFallback().
     */
    function _willFallback() internal {}

    /**
     * @dev fallback implementation.
     * Extracted to enable manual triggering.
     */
    function _fallback() internal {
        _willFallback();
        _delegate(_implementation());
    }

    /**
     * @dev Storage slot with the address of the current implementation.
     * This is the keccak-256 hash of "eip1967.proxy.implementation" subtracted by 1, and is
     * validated in the constructor.
     */
    bytes32 internal constant IMPLEMENTATION_SLOT =
        0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

    /**
     * @dev Returns the current implementation.
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
     * @dev Upgrades the proxy to a new implementation.
     * @param newImplementation Address of the new implementation.
     */
    function _upgradeTo(address newImplementation) internal {
        _setImplementation(newImplementation);
        emit Upgraded(newImplementation);
    }

    /**
     * @dev Sets the implementation address of the proxy.
     * @param newImplementation Address of the new implementation.
     */
    function _setImplementation(address newImplementation) internal {
        require(
            Address.isContract(newImplementation),
            "Cannot set a proxy implementation to a non-contract address"
        );

        bytes32 slot = IMPLEMENTATION_SLOT;

        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(slot, newImplementation)
        }
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import { InitializeGovernedUpgradeabilityProxy } from "./InitializeGovernedUpgradeabilityProxy.sol";

/**
 * @title BaseGovernedUpgradeabilityProxy2
 * @dev This is the same as InitializeGovernedUpgradeabilityProxy except that the
 *      governor is defined in the constructor.
 * @author Origin Protocol Inc
 */
contract InitializeGovernedUpgradeabilityProxy2 is
    InitializeGovernedUpgradeabilityProxy
{
    /**
     * This is used when the msg.sender can not be the governor. E.g. when the proxy is
     * deployed via CreateX
     */
    constructor(address governor) InitializeGovernedUpgradeabilityProxy() {
        _setGovernor(governor);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

/**
 * @title Base for contracts that are managed by the Origin Protocol's Governor.
 * @dev Copy of the openzeppelin Ownable.sol contract with nomenclature change
 *      from owner to governor and renounce methods removed. Does not use
 *      Context.sol like Ownable.sol does for simplification.
 * @author Origin Protocol Inc
 */
abstract contract Governable {
    // Storage position of the owner and pendingOwner of the contract
    // keccak256("OUSD.governor");
    bytes32 private constant governorPosition =
        0x7bea13895fa79d2831e0a9e28edede30099005a50d652d8957cf8a607ee6ca4a;

    // keccak256("OUSD.pending.governor");
    bytes32 private constant pendingGovernorPosition =
        0x44c4d30b2eaad5130ad70c3ba6972730566f3e6359ab83e800d905c61b1c51db;

    // keccak256("OUSD.reentry.status");
    bytes32 private constant reentryStatusPosition =
        0x53bf423e48ed90e97d02ab0ebab13b2a235a6bfbe9c321847d5c175333ac4535;

    // See OpenZeppelin ReentrancyGuard implementation
    uint256 constant _NOT_ENTERED = 1;
    uint256 constant _ENTERED = 2;

    event PendingGovernorshipTransfer(
        address indexed previousGovernor,
        address indexed newGovernor
    );

    event GovernorshipTransferred(
        address indexed previousGovernor,
        address indexed newGovernor
    );

    /**
     * @notice Returns the address of the current Governor.
     */
    function governor() public view returns (address) {
        return _governor();
    }

    /**
     * @dev Returns the address of the current Governor.
     */
    function _governor() internal view returns (address governorOut) {
        bytes32 position = governorPosition;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            governorOut := sload(position)
        }
    }

    /**
     * @dev Returns the address of the pending Governor.
     */
    function _pendingGovernor()
        internal
        view
        returns (address pendingGovernor)
    {
        bytes32 position = pendingGovernorPosition;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            pendingGovernor := sload(position)
        }
    }

    /**
     * @dev Throws if called by any account other than the Governor.
     */
    modifier onlyGovernor() {
        require(isGovernor(), "Caller is not the Governor");
        _;
    }

    /**
     * @notice Returns true if the caller is the current Governor.
     */
    function isGovernor() public view returns (bool) {
        return msg.sender == _governor();
    }

    function _setGovernor(address newGovernor) internal {
        emit GovernorshipTransferred(_governor(), newGovernor);

        bytes32 position = governorPosition;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(position, newGovernor)
        }
    }

    /**
     * @dev Prevents a contract from calling itself, directly or indirectly.
     * Calling a `nonReentrant` function from another `nonReentrant`
     * function is not supported. It is possible to prevent this from happening
     * by making the `nonReentrant` function external, and make it call a
     * `private` function that does the actual work.
     */
    modifier nonReentrant() {
        bytes32 position = reentryStatusPosition;
        uint256 _reentry_status;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _reentry_status := sload(position)
        }

        // On the first call to nonReentrant, _notEntered will be true
        require(_reentry_status != _ENTERED, "Reentrant call");

        // Any calls to nonReentrant after this point will fail
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(position, _ENTERED)
        }

        _;

        // By storing the original value once again, a refund is triggered (see
        // https://eips.ethereum.org/EIPS/eip-2200)
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(position, _NOT_ENTERED)
        }
    }

    function _setPendingGovernor(address newGovernor) internal {
        bytes32 position = pendingGovernorPosition;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(position, newGovernor)
        }
    }

    /**
     * @notice Transfers Governance of the contract to a new account (`newGovernor`).
     * Can only be called by the current Governor. Must be claimed for this to complete
     * @param _newGovernor Address of the new Governor
     */
    function transferGovernance(address _newGovernor) external onlyGovernor {
        _setPendingGovernor(_newGovernor);
        emit PendingGovernorshipTransfer(_governor(), _newGovernor);
    }

    /**
     * @notice Claim Governance of the contract to a new account (`newGovernor`).
     * Can only be called by the new Governor.
     */
    function claimGovernance() external {
        require(
            msg.sender == _pendingGovernor(),
            "Only the pending Governor can complete the claim"
        );
        _changeGovernor(msg.sender);
    }

    /**
     * @dev Change Governance of the contract to a new account (`newGovernor`).
     * @param _newGovernor Address of the new Governor
     */
    function _changeGovernor(address _newGovernor) internal {
        require(_newGovernor != address(0), "New Governor is address(0)");
        _setGovernor(_newGovernor);
    }
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

