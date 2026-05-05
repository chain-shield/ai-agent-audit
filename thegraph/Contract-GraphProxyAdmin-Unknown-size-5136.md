
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

/* solhint-disable gas-custom-errors */ // Cannot use custom errors with 0.7.6

import { Governed } from "../governance/Governed.sol";

import { IGraphProxy } from "@graphprotocol/interfaces/contracts/contracts/upgrades/IGraphProxy.sol";
import { GraphUpgradeable } from "./GraphUpgradeable.sol";

/**
 * @title GraphProxyAdmin
 * @author Edge & Node
 * @notice This is the owner of upgradeable proxy contracts.
 * Proxy contracts use a TransparentProxy pattern, any admin related call
 * like upgrading a contract or changing the admin needs to be send through
 * this contract.
 */
contract GraphProxyAdmin is Governed {
    /**
     * @notice Contract constructor.
     */
    constructor() {
        Governed._initialize(msg.sender);
    }

    /**
     * @notice Returns the current implementation of a proxy.
     * @dev This is needed because only the proxy admin can query it.
     * @param _proxy Address of the proxy for which to get the implementation.
     * @return The address of the current implementation of the proxy.
     */
    function getProxyImplementation(IGraphProxy _proxy) external view returns (address) {
        // We need to manually run the static call since the getter cannot be flagged as view
        // bytes4(keccak256("implementation()")) == 0x5c60da1b
        (bool success, bytes memory returndata) = address(_proxy).staticcall(hex"5c60da1b");
        require(success, "Proxy impl call failed");
        return abi.decode(returndata, (address));
    }

    /**
     * @notice Returns the pending implementation of a proxy.
     * @dev This is needed because only the proxy admin can query it.
     * @param _proxy Address of the proxy for which to get the pending implementation.
     * @return The address of the pending implementation of the proxy.
     */
    function getProxyPendingImplementation(IGraphProxy _proxy) external view returns (address) {
        // We need to manually run the static call since the getter cannot be flagged as view
        // bytes4(keccak256("pendingImplementation()")) == 0x396f7b23
        (bool success, bytes memory returndata) = address(_proxy).staticcall(hex"396f7b23");
        require(success, "Proxy pendingImpl call failed");
        return abi.decode(returndata, (address));
    }

    /**
     * @notice Returns the admin of a proxy. Only the admin can query it.
     * @param _proxy Address of the proxy for which to get the admin.
     * @return The address of the current admin of the proxy.
     */
    function getProxyAdmin(IGraphProxy _proxy) external view returns (address) {
        // We need to manually run the static call since the getter cannot be flagged as view
        // bytes4(keccak256("admin()")) == 0xf851a440
        (bool success, bytes memory returndata) = address(_proxy).staticcall(hex"f851a440");
        require(success, "Proxy admin call failed");
        return abi.decode(returndata, (address));
    }

    /**
     * @notice Changes the admin of a proxy.
     * @param _proxy Proxy to change admin.
     * @param _newAdmin Address to transfer proxy administration to.
     */
    function changeProxyAdmin(IGraphProxy _proxy, address _newAdmin) external onlyGovernor {
        _proxy.setAdmin(_newAdmin);
    }

    /**
     * @notice Upgrades a proxy to the newest implementation of a contract.
     * @param _proxy Proxy to be upgraded.
     * @param _implementation the address of the Implementation.
     */
    function upgrade(IGraphProxy _proxy, address _implementation) external onlyGovernor {
        _proxy.upgradeTo(_implementation);
    }

    /**
     * @notice Accepts a proxy.
     * @param _implementation Address of the implementation accepting the proxy.
     * @param _proxy Address of the proxy being accepted.
     */
    function acceptProxy(GraphUpgradeable _implementation, IGraphProxy _proxy) external onlyGovernor {
        _implementation.acceptProxy(_proxy);
    }

    /**
     * @notice Accepts a proxy and call a function on the implementation.
     * @param _implementation Address of the implementation accepting the proxy.
     * @param _proxy Address of the proxy being accepted.
     * @param _data Encoded function to call on the implementation after accepting the proxy.
     */
    function acceptProxyAndCall(
        GraphUpgradeable _implementation,
        IGraphProxy _proxy,
        bytes calldata _data
    ) external onlyGovernor {
        _implementation.acceptProxyAndCall(_proxy, _data);
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

/* solhint-disable gas-custom-errors */ // Cannot use custom errors with 0.7.6

/**
 * @title Graph Proxy Storage
 * @author Edge & Node
 * @notice Contract functions related to getting and setting proxy storage.
 * This contract does not actually define state variables managed by the compiler
 * but uses fixed slot locations.
 */
abstract contract GraphProxyStorage {
    /**
     * @dev Storage slot with the address of the current implementation.
     * This is the keccak-256 hash of "eip1967.proxy.implementation" subtracted by 1, and is
     * validated in the constructor.
     */
    bytes32 internal constant IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

    /**
     * @dev Storage slot with the address of the pending implementation.
     * This is the keccak-256 hash of "eip1967.proxy.pendingImplementation" subtracted by 1, and is
     * validated in the constructor.
     */
    bytes32 internal constant PENDING_IMPLEMENTATION_SLOT =
        0x9e5eddc59e0b171f57125ab86bee043d9128098c3a6b9adb4f2e86333c2f6f8c;

    /**
     * @dev Storage slot with the admin of the contract.
     * This is the keccak-256 hash of "eip1967.proxy.admin" subtracted by 1, and is
     * validated in the constructor.
     */
    bytes32 internal constant ADMIN_SLOT = 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103;

    /**
     * @notice Emitted when pendingImplementation is changed.
     * @param oldPendingImplementation Address of the previous pending implementation
     * @param newPendingImplementation Address of the new pending implementation
     */
    event PendingImplementationUpdated(
        address indexed oldPendingImplementation,
        address indexed newPendingImplementation
    );

    /**
     * @notice Emitted when pendingImplementation is accepted,
     * which means contract implementation is updated.
     * @param oldImplementation Address of the previous implementation
     * @param newImplementation Address of the new implementation
     */
    event ImplementationUpdated(address indexed oldImplementation, address indexed newImplementation);

    /**
     * @notice Emitted when the admin account has changed.
     * @param oldAdmin Address of the previous admin
     * @param newAdmin Address of the new admin
     */
    event AdminUpdated(address indexed oldAdmin, address indexed newAdmin);

    /**
     * @dev Modifier to check whether the `msg.sender` is the admin.
     */
    modifier onlyAdmin() {
        require(msg.sender == _getAdmin(), "Caller must be admin");
        _;
    }

    /**
     * @notice Returns the current admin address
     * @return adm The admin slot.
     */
    function _getAdmin() internal view returns (address adm) {
        bytes32 slot = ADMIN_SLOT;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            adm := sload(slot)
        }
    }

    /**
     * @notice Sets the address of the proxy admin.
     * @param _newAdmin Address of the new proxy admin
     */
    function _setAdmin(address _newAdmin) internal {
        address oldAdmin = _getAdmin();
        bytes32 slot = ADMIN_SLOT;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(slot, _newAdmin)
        }

        emit AdminUpdated(oldAdmin, _newAdmin);
    }

    /**
     * @notice Returns the current implementation.
     * @return impl Address of the current implementation
     */
    function _getImplementation() internal view returns (address impl) {
        bytes32 slot = IMPLEMENTATION_SLOT;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            impl := sload(slot)
        }
    }

    /**
     * @notice Returns the current pending implementation.
     * @return impl Address of the current pending implementation
     */
    function _getPendingImplementation() internal view returns (address impl) {
        bytes32 slot = PENDING_IMPLEMENTATION_SLOT;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            impl := sload(slot)
        }
    }

    /**
     * @notice Sets the implementation address of the proxy.
     * @param _newImplementation Address of the new implementation
     */
    function _setImplementation(address _newImplementation) internal {
        address oldImplementation = _getImplementation();

        bytes32 slot = IMPLEMENTATION_SLOT;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(slot, _newImplementation)
        }

        emit ImplementationUpdated(oldImplementation, _newImplementation);
    }

    /**
     * @notice Sets the pending implementation address of the proxy.
     * @param _newImplementation Address of the new pending implementation
     */
    function _setPendingImplementation(address _newImplementation) internal {
        address oldPendingImplementation = _getPendingImplementation();

        bytes32 slot = PENDING_IMPLEMENTATION_SLOT;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(slot, _newImplementation)
        }

        emit PendingImplementationUpdated(oldPendingImplementation, _newImplementation);
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-small-strings

/* solhint-disable gas-custom-errors */ // Cannot use custom errors with 0.7.6

import { GraphProxyStorage } from "./GraphProxyStorage.sol";

import { IGraphProxy } from "@graphprotocol/interfaces/contracts/contracts/upgrades/IGraphProxy.sol";

/**
 * @title Graph Proxy
 * @author Edge & Node
 * @notice Graph Proxy contract used to delegate call implementation contracts and support upgrades.
 * This contract should NOT define storage as it is managed by GraphProxyStorage.
 * This contract implements a proxy that is upgradeable by an admin.
 * https://docs.openzeppelin.com/upgrades-plugins/1.x/proxies#transparent-proxies-and-function-clashes
 */
contract GraphProxy is GraphProxyStorage, IGraphProxy {
    /**
     * @dev Modifier used internally that will delegate the call to the implementation unless
     * the sender is the admin.
     */
    modifier ifAdmin() {
        if (msg.sender == _getAdmin()) {
            _;
        } else {
            _fallback();
        }
    }

    /**
     * @dev Modifier used internally that will delegate the call to the implementation unless
     * the sender is the admin or pending implementation.
     */
    modifier ifAdminOrPendingImpl() {
        if (msg.sender == _getAdmin() || msg.sender == _getPendingImplementation()) {
            _;
        } else {
            _fallback();
        }
    }

    /**
     * @notice GraphProxy contract constructor.
     * @param _impl Address of the initial implementation
     * @param _admin Address of the proxy admin
     */
    constructor(address _impl, address _admin) {
        assert(ADMIN_SLOT == bytes32(uint256(keccak256("eip1967.proxy.admin")) - 1));
        assert(IMPLEMENTATION_SLOT == bytes32(uint256(keccak256("eip1967.proxy.implementation")) - 1));
        assert(PENDING_IMPLEMENTATION_SLOT == bytes32(uint256(keccak256("eip1967.proxy.pendingImplementation")) - 1));

        _setAdmin(_admin);
        _setPendingImplementation(_impl);
    }

    /**
     * @notice Fallback function that delegates calls to implementation. Will run if call data
     * is empty.
     */
    receive() external payable {
        _fallback();
    }

    /**
     * @notice Fallback function that delegates calls to implementation. Will run if no other
     * function in the contract matches the call data.
     */
    fallback() external payable {
        _fallback();
    }

    /**
     * @inheritdoc IGraphProxy
     * @dev NOTE: Only the admin and implementation can call this function.
     *
     * TIP: To get this value clients can read directly from the storage slot shown below (specified by EIP1967) using the
     * https://eth.wiki/json-rpc/API#eth_getstorageat[`eth_getStorageAt`] RPC call.
     * `0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103`
     */
    function admin() external override ifAdminOrPendingImpl returns (address adminAddress) {
        return _getAdmin();
    }

    /**
     * @inheritdoc IGraphProxy
     * @dev NOTE: Only the admin can call this function.
     *
     * TIP: To get this value clients can read directly from the storage slot shown below (specified by EIP1967) using the
     * https://eth.wiki/json-rpc/API#eth_getstorageat[`eth_getStorageAt`] RPC call.
     * `0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc`
     */
    function implementation() external override ifAdminOrPendingImpl returns (address implementationAddress) {
        return _getImplementation();
    }

    /**
     * @inheritdoc IGraphProxy
     * @dev NOTE: Only the admin can call this function.
     *
     * TIP: To get this value clients can read directly from the storage slot shown below (specified by EIP1967) using the
     * https://eth.wiki/json-rpc/API#eth_getstorageat[`eth_getStorageAt`] RPC call.
     * `0x9e5eddc59e0b171f57125ab86bee043d9128098c3a6b9adb4f2e86333c2f6f8c`
     */
    function pendingImplementation()
        external
        override
        ifAdminOrPendingImpl
        returns (address pendingImplementationAddress)
    {
        return _getPendingImplementation();
    }

    /**
     * @inheritdoc IGraphProxy
     * @dev NOTE: Only the admin can call this function.
     */
    function setAdmin(address _newAdmin) external override ifAdmin {
        require(_newAdmin != address(0), "Admin cant be the zero address");
        _setAdmin(_newAdmin);
    }

    /**
     * @inheritdoc IGraphProxy
     * @dev NOTE: Only the admin can call this function.
     */
    function upgradeTo(address _newImplementation) external override ifAdmin {
        _setPendingImplementation(_newImplementation);
    }

    /**
     * @inheritdoc IGraphProxy
     */
    function acceptUpgrade() external override ifAdminOrPendingImpl {
        _acceptUpgrade();
    }

    /**
     * @inheritdoc IGraphProxy
     */
    function acceptUpgradeAndCall(bytes calldata data) external override ifAdminOrPendingImpl {
        _acceptUpgrade();
        // solhint-disable-next-line avoid-low-level-calls
        (bool success, ) = _getImplementation().delegatecall(data);
        require(success, "Impl call failed");
    }

    /**
     * @notice Admin function for new implementation to accept its role as implementation.
     */
    function _acceptUpgrade() internal {
        address _pendingImplementation = _getPendingImplementation();
        require(_pendingImplementation != address(0), "Impl cannot be zero address");
        require(msg.sender == _pendingImplementation, "Only pending implementation");

        _setImplementation(_pendingImplementation);
        _setPendingImplementation(address(0));
    }

    /**
     * @notice Delegates the current call to implementation.
     * This function does not return to its internal call site, it will return directly to the
     * external caller.
     */
    function _fallback() internal {
        require(msg.sender != _getAdmin(), "Cannot fallback to proxy target");

        // solhint-disable-next-line no-inline-assembly
        assembly {
            // (a) get free memory pointer
            let ptr := mload(0x40)

            // (b) get address of the implementation
            let impl := and(sload(IMPLEMENTATION_SLOT), 0xffffffffffffffffffffffffffffffffffffffff)

            // (1) copy incoming call data
            calldatacopy(ptr, 0, calldatasize())

            // (2) forward call to logic contract
            let result := delegatecall(gas(), impl, ptr, calldatasize(), 0, 0)
            let size := returndatasize()

            // (3) retrieve return data
            returndatacopy(ptr, 0, size)

            // (4) forward return data back to caller
            switch result
            case 0 {
                revert(ptr, size)
            }
            default {
                return(ptr, size)
            }
        }
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


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

