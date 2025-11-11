
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Tstorish} from "tstorish/Tstorish.sol";
import {TrailsSentinelLib} from "./libraries/TrailsSentinelLib.sol";
import {ITrailsRouterShim} from "./interfaces/ITrailsRouterShim.sol";
import {DelegatecallGuard} from "./guards/DelegatecallGuard.sol";

/// @title TrailsRouterShim
/// @author Shun Kakinoki
/// @notice Sequence delegate-call extension that forwards Trails router calls and records success sentinels.
contract TrailsRouterShim is ITrailsRouterShim, DelegatecallGuard, Tstorish {
    // -------------------------------------------------------------------------
    // Immutable variables
    // -------------------------------------------------------------------------

    /// @notice Address of the deployed TrailsMulticall3Router to forward calls to
    address public immutable ROUTER;
    // SELF provided by DelegatecallGuard

    // -------------------------------------------------------------------------
    // Errors
    // -------------------------------------------------------------------------

    error RouterCallFailed(bytes data);
    error ZeroRouterAddress();

    // -------------------------------------------------------------------------
    // Constructor
    // -------------------------------------------------------------------------

    /// @param router_ The address of the router to forward calls to
    constructor(address router_) {
        if (router_ == address(0)) revert ZeroRouterAddress();
        ROUTER = router_;
    }

    // -------------------------------------------------------------------------
    // Sequence delegated entry point
    // -------------------------------------------------------------------------

    /// @inheritdoc ITrailsRouterShim
    function handleSequenceDelegateCall(
        bytes32 opHash,
        uint256, // startingGas (unused)
        uint256, // index (unused)
        uint256, // numCalls (unused)
        uint256, // space (unused)
        bytes calldata data
    )
        external
        onlyDelegatecall
    {
        // Decode the inner call data and call value forwarded to the router
        (bytes memory inner, uint256 callValue) = abi.decode(data, (bytes, uint256));

        // Forward the call to the router
        bytes memory routerReturn = _forwardToRouter(inner, callValue);

        // Set the success sentinel storage slot for the opHash
        uint256 slot = TrailsSentinelLib.successSlot(opHash);
        _setTstorish(slot, TrailsSentinelLib.SUCCESS_VALUE);

        assembly {
            return(add(routerReturn, 32), mload(routerReturn))
        }
    }

    // -------------------------------------------------------------------------
    // Internal Helpers
    // -------------------------------------------------------------------------

    /// forge-lint: disable-next-line(mixed-case-function)
    function _forwardToRouter(bytes memory forwardData, uint256 callValue) internal returns (bytes memory) {
        (bool success, bytes memory ret) = ROUTER.call{value: callValue}(forwardData);
        if (!success) {
            revert RouterCallFailed(ret);
        }
        return ret;
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

import {IDelegatedExtension} from "wallet-contracts-v3/modules/interfaces/IDelegatedExtension.sol";

/// @title ITrailsRouterShim
/// @notice Interface for the router shim that bridges Sequence wallets to the Trails router.
interface ITrailsRouterShim is IDelegatedExtension {
    // -------------------------------------------------------------------------
    // Functions
    // -------------------------------------------------------------------------

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


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

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

import {SingletonDeployer, console} from "erc2470-libs/script/SingletonDeployer.s.sol";
import {TrailsRouterShim} from "../src/TrailsRouterShim.sol";
import {Deploy as TrailsRouterDeploy} from "./TrailsRouter.s.sol";

contract Deploy is SingletonDeployer {
    // -------------------------------------------------------------------------
    // State Variables
    // -------------------------------------------------------------------------

    address public routerAddress;

    // -------------------------------------------------------------------------
    // Run
    // -------------------------------------------------------------------------

    function run() external {
        uint256 pk = vm.envUint("PRIVATE_KEY");
        address deployerAddress = vm.addr(pk);
        console.log("Deployer Address:", deployerAddress);

        bytes32 salt = bytes32(0);

        // Deploy TrailsRouter using the TrailsRouter deployment script
        TrailsRouterDeploy routerDeploy = new TrailsRouterDeploy();
        routerDeploy.run();

        // Get the deployed router address from the deployment script
        routerAddress = routerDeploy.deployRouter(pk);
        console.log("TrailsRouter deployed at:", routerAddress);

        // Deploy TrailsRouterShim with the router address
        bytes memory initCode = abi.encodePacked(type(TrailsRouterShim).creationCode, abi.encode(routerAddress));
        address wrapper = _deployIfNotAlready("TrailsRouterShim", initCode, salt, pk);

        console.log("TrailsRouterShim deployed at:", wrapper);
    }
}

