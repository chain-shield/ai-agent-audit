
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

