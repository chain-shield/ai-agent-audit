
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Permit.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {ITrailsIntentEntrypoint} from "./interfaces/ITrailsIntentEntrypoint.sol";

/// @title TrailsIntentEntrypoint
/// @author Miguel Mota
/// @notice A contract to facilitate deposits to intent addresses with off-chain signed intents.
contract TrailsIntentEntrypoint is ReentrancyGuard, ITrailsIntentEntrypoint {
    // -------------------------------------------------------------------------
    // Libraries
    // -------------------------------------------------------------------------
    using ECDSA for bytes32;
    using SafeERC20 for IERC20;

    // -------------------------------------------------------------------------
    // Constants
    // -------------------------------------------------------------------------

    bytes32 public constant TRAILS_INTENT_TYPEHASH = keccak256(
        "TrailsIntent(address user,address token,uint256 amount,address intentAddress,uint256 deadline,uint256 chainId,uint256 nonce,uint256 feeAmount,address feeCollector)"
    );
    string public constant VERSION = "1";

    // -------------------------------------------------------------------------
    // Errors
    // -------------------------------------------------------------------------

    error InvalidAmount();
    error InvalidToken();
    error InvalidIntentAddress();
    error IntentExpired();
    error InvalidIntentSignature();
    error IntentAlreadyUsed();
    error InvalidChainId();
    error InvalidNonce();
    error PermitAmountMismatch();

    // -------------------------------------------------------------------------
    // Immutable Variables
    // -------------------------------------------------------------------------

    /// @notice EIP-712 domain separator used for intent signatures.
    bytes32 public immutable DOMAIN_SEPARATOR;

    // -------------------------------------------------------------------------
    // State Variables
    // -------------------------------------------------------------------------

    /// @notice Tracks whether an intent digest has been consumed to prevent replays.
    mapping(bytes32 => bool) public usedIntents;

    /// @notice Tracks nonce for each user to prevent replay attacks.
    mapping(address => uint256) public nonces;

    // -------------------------------------------------------------------------
    // Constructor
    // -------------------------------------------------------------------------

    constructor() {
        DOMAIN_SEPARATOR = keccak256(
            abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256(bytes("TrailsIntentEntrypoint")),
                keccak256(bytes(VERSION)),
                block.chainid,
                address(this)
            )
        );
    }

    // -------------------------------------------------------------------------
    // Functions
    // -------------------------------------------------------------------------

    /// @inheritdoc ITrailsIntentEntrypoint
    function depositToIntentWithPermit(
        address user,
        address token,
        uint256 amount,
        uint256 permitAmount,
        address intentAddress,
        uint256 deadline,
        uint256 nonce,
        uint256 feeAmount,
        address feeCollector,
        uint8 permitV,
        bytes32 permitR,
        bytes32 permitS,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS
    ) external nonReentrant {
        _verifyAndMarkIntent(
            user, token, amount, intentAddress, deadline, nonce, feeAmount, feeCollector, sigV, sigR, sigS
        );

        // Validate permitAmount exactly matches the total required amount (deposit + fee)
        // This prevents permit/approval mismatches that could cause DoS or unexpected behavior
        unchecked {
            if (permitAmount != amount + feeAmount) revert PermitAmountMismatch();
        }

        IERC20Permit(token).permit(user, address(this), permitAmount, deadline, permitV, permitR, permitS);
        IERC20(token).safeTransferFrom(user, intentAddress, amount);

        // Pay fee if specified (fee token is same as deposit token)
        if (feeAmount > 0 && feeCollector != address(0)) {
            IERC20(token).safeTransferFrom(user, feeCollector, feeAmount);
            emit FeePaid(user, token, feeAmount, feeCollector);
        }

        emit IntentDeposit(user, intentAddress, amount);
    }

    /// @inheritdoc ITrailsIntentEntrypoint
    function depositToIntent(
        address user,
        address token,
        uint256 amount,
        address intentAddress,
        uint256 deadline,
        uint256 nonce,
        uint256 feeAmount,
        address feeCollector,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS
    ) external nonReentrant {
        _verifyAndMarkIntent(
            user, token, amount, intentAddress, deadline, nonce, feeAmount, feeCollector, sigV, sigR, sigS
        );

        IERC20(token).safeTransferFrom(user, intentAddress, amount);

        // Pay fee if specified (fee token is same as deposit token)
        if (feeAmount > 0 && feeCollector != address(0)) {
            IERC20(token).safeTransferFrom(user, feeCollector, feeAmount);
            emit FeePaid(user, token, feeAmount, feeCollector);
        }

        emit IntentDeposit(user, intentAddress, amount);
    }

    // -------------------------------------------------------------------------
    // Internal Functions
    // -------------------------------------------------------------------------

    /// forge-lint: disable-next-line(mixed-case-function)
    function _verifyAndMarkIntent(
        address user,
        address token,
        uint256 amount,
        address intentAddress,
        uint256 deadline,
        uint256 nonce,
        uint256 feeAmount,
        address feeCollector,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS
    ) internal {
        if (amount == 0) revert InvalidAmount();
        if (token == address(0)) revert InvalidToken();
        if (intentAddress == address(0)) revert InvalidIntentAddress();
        if (block.timestamp > deadline) revert IntentExpired();
        // Chain ID is already included in the signature, so we don't need to check it here
        // The signature verification will fail if the chain ID doesn't match
        if (nonce != nonces[user]) revert InvalidNonce();

        bytes32 _typehash = TRAILS_INTENT_TYPEHASH;
        bytes32 intentHash;
        // keccak256(abi.encode(TRAILS_INTENT_TYPEHASH, user, token, amount, intentAddress, deadline, chainId, nonce, feeAmount, feeCollector));
        assembly {
            let ptr := mload(0x40)
            mstore(ptr, _typehash)
            mstore(add(ptr, 0x20), user)
            mstore(add(ptr, 0x40), token)
            mstore(add(ptr, 0x60), amount)
            mstore(add(ptr, 0x80), intentAddress)
            mstore(add(ptr, 0xa0), deadline)
            mstore(add(ptr, 0xc0), chainid())
            mstore(add(ptr, 0xe0), nonce)
            mstore(add(ptr, 0x100), feeAmount)
            mstore(add(ptr, 0x120), feeCollector)
            intentHash := keccak256(ptr, 0x140)
        }

        bytes32 _domainSeparator = DOMAIN_SEPARATOR;
        bytes32 digest;
        // keccak256(abi.encodePacked("\x19\x01", DOMAIN_SEPARATOR, intentHash));
        assembly {
            let ptr := mload(0x40)
            mstore(ptr, 0x1901)
            mstore(add(ptr, 0x20), _domainSeparator)
            mstore(add(ptr, 0x40), intentHash)
            digest := keccak256(add(ptr, 0x1e), 0x42)
        }
        address recovered = ECDSA.recover(digest, sigV, sigR, sigS);
        if (recovered != user) revert InvalidIntentSignature();

        if (usedIntents[digest]) revert IntentAlreadyUsed();
        usedIntents[digest] = true;

        // Increment nonce for the user
        nonces[user]++;
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title ITrailsIntentEntrypoint
/// @notice Interface for the TrailsIntentEntrypoint contract
interface ITrailsIntentEntrypoint {
    // -------------------------------------------------------------------------
    // Events
    // -------------------------------------------------------------------------

    /// @notice Emitted when a user deposits tokens to an intent address
    /// @param user The user making the deposit
    /// @param intentAddress The intent address receiving the deposit
    /// @param amount The amount of tokens deposited
    event IntentDeposit(address indexed user, address indexed intentAddress, uint256 amount);

    /// @notice Emitted when a fee is paid.
    /// @param user The account from which the fee was taken.
    /// @param feeToken The ERC-20 token used to pay the fee.
    /// @param feeAmount The amount of the fee paid.
    /// @param feeCollector The address receiving the fee.
    event FeePaid(address indexed user, address indexed feeToken, uint256 feeAmount, address indexed feeCollector);

    // -------------------------------------------------------------------------
    // Views
    // -------------------------------------------------------------------------

    /// @notice Returns the EIP-712 domain separator used for intent signatures.
    /// forge-lint: disable-next-line(mixed-case-function)
    function DOMAIN_SEPARATOR() external view returns (bytes32);

    /// @notice Returns the trails intent typehash constant used in EIP-712 signatures.
    /// forge-lint: disable-next-line(mixed-case-function)
    function TRAILS_INTENT_TYPEHASH() external view returns (bytes32);

    /// @notice Returns the version string of the contract.
    /// forge-lint: disable-next-line(mixed-case-function)
    function VERSION() external view returns (string memory);

    /// @notice Returns whether an intent digest has already been consumed.
    /// @param digest The EIP-712 digest of the intent message.
    function usedIntents(bytes32 digest) external view returns (bool);

    /// @notice Returns the current nonce for a given user.
    /// @param user The user address to query.
    function nonces(address user) external view returns (uint256);

    // -------------------------------------------------------------------------
    // Functions
    // -------------------------------------------------------------------------

    /// @notice Deposit tokens to an intent address using ERC20 permit
    /// @param user The user making the deposit
    /// @param token The token to deposit (also used for fee payment)
    /// @param amount The amount to deposit
    /// @param permitAmount The amount to permit for spending (amount + feeAmount if paying fee)
    /// @param intentAddress The intent address to deposit to
    /// @param deadline The permit deadline
    /// @param nonce The nonce for this user
    /// @param feeAmount The amount of fee to pay (0 for no fee, paid in same token)
    /// @param feeCollector The address to receive the fee (address(0) for no fee)
    /// @param permitV The permit signature v component
    /// @param permitR The permit signature r component
    /// @param permitS The permit signature s component
    /// @param sigV The intent signature v component
    /// @param sigR The intent signature r component
    /// @param sigS The intent signature s component
    function depositToIntentWithPermit(
        address user,
        address token,
        uint256 amount,
        uint256 permitAmount,
        address intentAddress,
        uint256 deadline,
        uint256 nonce,
        uint256 feeAmount,
        address feeCollector,
        uint8 permitV,
        bytes32 permitR,
        bytes32 permitS,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS
    ) external;

    /// @notice Deposit tokens to an intent address (requires prior approval)
    /// @param user The user making the deposit
    /// @param token The token to deposit (also used for fee payment)
    /// @param amount The amount to deposit
    /// @param intentAddress The intent address to deposit to
    /// @param deadline The intent deadline
    /// @param nonce The nonce for this user
    /// @param feeAmount The amount of fee to pay (0 for no fee, paid in same token)
    /// @param feeCollector The address to receive the fee (address(0) for no fee)
    /// @param sigV The intent signature v component
    /// @param sigR The intent signature r component
    /// @param sigS The intent signature s component
    function depositToIntent(
        address user,
        address token,
        uint256 amount,
        address intentAddress,
        uint256 deadline,
        uint256 nonce,
        uint256 feeAmount,
        address feeCollector,
        uint8 sigV,
        bytes32 sigR,
        bytes32 sigS
    ) external;
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.1.0) (utils/ReentrancyGuard.sol)

pragma solidity ^0.8.20;

import {StorageSlot} from "./StorageSlot.sol";

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
 * TIP: If EIP-1153 (transient storage) is available on the chain you're deploying at,
 * consider using {ReentrancyGuardTransient} instead.
 *
 * TIP: If you would like to learn more about reentrancy and alternative ways
 * to protect against it, check out our blog post
 * https://blog.openzeppelin.com/reentrancy-after-istanbul/[Reentrancy After Istanbul].
 *
 * IMPORTANT: Deprecated. This storage-based reentrancy guard will be removed and replaced
 * by the {ReentrancyGuardTransient} variant in v6.0.
 *
 * @custom:stateless
 */
abstract contract ReentrancyGuard {
    using StorageSlot for bytes32;

    // keccak256(abi.encode(uint256(keccak256("openzeppelin.storage.ReentrancyGuard")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant REENTRANCY_GUARD_STORAGE =
        0x9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f00;

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
    uint256 private constant NOT_ENTERED = 1;
    uint256 private constant ENTERED = 2;

    /**
     * @dev Unauthorized reentrant call.
     */
    error ReentrancyGuardReentrantCall();

    constructor() {
        _reentrancyGuardStorageSlot().getUint256Slot().value = NOT_ENTERED;
    }

    /**
     * @dev Prevents a contract from calling itself, directly or indirectly.
     * Calling a `nonReentrant` function from another `nonReentrant`
     * function is not supported. It is possible to prevent this from happening
     * by making the `nonReentrant` function external, and making it call a
     * `private` function that does the actual work.
     */
    modifier nonReentrant() {
        _nonReentrantBefore();
        _;
        _nonReentrantAfter();
    }

    /**
     * @dev A `view` only version of {nonReentrant}. Use to block view functions
     * from being called, preventing reading from inconsistent contract state.
     *
     * CAUTION: This is a "view" modifier and does not change the reentrancy
     * status. Use it only on view functions. For payable or non-payable functions,
     * use the standard {nonReentrant} modifier instead.
     */
    modifier nonReentrantView() {
        _nonReentrantBeforeView();
        _;
    }

    function _nonReentrantBeforeView() private view {
        if (_reentrancyGuardEntered()) {
            revert ReentrancyGuardReentrantCall();
        }
    }

    function _nonReentrantBefore() private {
        // On the first call to nonReentrant, _status will be NOT_ENTERED
        _nonReentrantBeforeView();

        // Any calls to nonReentrant after this point will fail
        _reentrancyGuardStorageSlot().getUint256Slot().value = ENTERED;
    }

    function _nonReentrantAfter() private {
        // By storing the original value once again, a refund is triggered (see
        // https://eips.ethereum.org/EIPS/eip-2200)
        _reentrancyGuardStorageSlot().getUint256Slot().value = NOT_ENTERED;
    }

    /**
     * @dev Returns true if the reentrancy guard is currently set to "entered", which indicates there is a
     * `nonReentrant` function in the call stack.
     */
    function _reentrancyGuardEntered() internal view returns (bool) {
        return _reentrancyGuardStorageSlot().getUint256Slot().value == ENTERED;
    }

    function _reentrancyGuardStorageSlot() internal pure virtual returns (bytes32) {
        return REENTRANCY_GUARD_STORAGE;
    }
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

import {SingletonDeployer, console} from "erc2470-libs/script/SingletonDeployer.s.sol";
import {TrailsIntentEntrypoint} from "../src/TrailsIntentEntrypoint.sol";

contract Deploy is SingletonDeployer {
    // -------------------------------------------------------------------------
    // Run
    // -------------------------------------------------------------------------

    function run() external {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");
        address deployerAddress = vm.addr(privateKey);
        console.log("Deployer Address:", deployerAddress);

        bytes32 salt = bytes32(0);

        // Deploy TrailsIntentEntrypoint deterministically via ERC-2470 SingletonDeployer
        bytes memory initCode = type(TrailsIntentEntrypoint).creationCode;
        address sweeper = _deployIfNotAlready("TrailsIntentEntrypoint", initCode, salt, privateKey);

        console.log("TrailsIntentEntrypoint deployed at:", sweeper);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";
import {Deploy as TrailsIntentEntrypointDeploy} from "script/TrailsIntentEntrypoint.s.sol";
import {TrailsIntentEntrypoint} from "src/TrailsIntentEntrypoint.sol";
import {Create2Utils} from "../utils/Create2Utils.sol";

// -----------------------------------------------------------------------------
// Test Contract
// -----------------------------------------------------------------------------

contract TrailsIntentEntrypointDeploymentTest is Test {
    // -------------------------------------------------------------------------
    // Test State Variables
    // -------------------------------------------------------------------------

    TrailsIntentEntrypointDeploy internal _deployScript;
    address internal _deployer;
    uint256 internal _deployerPk;
    string internal _deployerPkStr;

    // -------------------------------------------------------------------------
    // Pure Functions
    // -------------------------------------------------------------------------

    // Expected predetermined address (calculated using CREATE2)
    function expectedIntentEntrypointAddress() internal pure returns (address payable) {
        return
            Create2Utils.calculateCreate2Address(type(TrailsIntentEntrypoint).creationCode, Create2Utils.standardSalt());
    }

    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------

    function setUp() public {
        _deployScript = new TrailsIntentEntrypointDeploy();
        _deployerPk = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80; // anvil default key
        _deployerPkStr = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        _deployer = vm.addr(_deployerPk);
        vm.deal(_deployer, 100 ether);
    }

    // -------------------------------------------------------------------------
    // Tests
    // -------------------------------------------------------------------------

    function test_DeployIntentEntrypoint_Success() public {
        vm.setEnv("PRIVATE_KEY", _deployerPkStr);

        vm.recordLogs();
        _deployScript.run();

        // Get the expected address
        address payable expectedAddress = expectedIntentEntrypointAddress();

        // Verify the deployed contract is functional
        TrailsIntentEntrypoint entrypoint = TrailsIntentEntrypoint(expectedAddress);
        assertEq(address(entrypoint).code.length > 0, true, "Entrypoint should have code");

        // Verify domain separator is set (basic functionality test)
        bytes32 domainSeparator = entrypoint.DOMAIN_SEPARATOR();
        assertTrue(domainSeparator != bytes32(0), "Domain separator should be set");
    }

    function test_DeployIntentEntrypoint_SameAddress() public {
        vm.setEnv("PRIVATE_KEY", _deployerPkStr);

        // Get the expected address
        address payable expectedAddress = expectedIntentEntrypointAddress();

        // First deployment
        vm.recordLogs();
        _deployScript.run();

        // Verify first deployment address
        assertEq(expectedAddress.code.length > 0, true, "First deployment: TrailsIntentEntrypoint deployed");

        // Re-set the PRIVATE_KEY for second deployment
        vm.setEnv("PRIVATE_KEY", _deployerPkStr);

        // Second deployment should result in the same address (deterministic)
        vm.recordLogs();
        _deployScript.run();

        // Verify second deployment still has contract at same address
        assertEq(expectedAddress.code.length > 0, true, "Second deployment: TrailsIntentEntrypoint still deployed");
    }

    function test_DeployedIntentEntrypoint_HasCorrectConfiguration() public {
        vm.setEnv("PRIVATE_KEY", _deployerPkStr);

        // Deploy the script
        _deployScript.run();

        // Get reference to deployed contract
        address payable expectedAddress = expectedIntentEntrypointAddress();
        TrailsIntentEntrypoint entrypoint = TrailsIntentEntrypoint(expectedAddress);

        // Verify contract is deployed and functional
        assertEq(address(entrypoint).code.length > 0, true, "Entrypoint should have code");

        // Verify EIP-712 domain separator is properly constructed
        bytes32 domainSeparator = entrypoint.DOMAIN_SEPARATOR();
        assertTrue(domainSeparator != bytes32(0), "Domain separator should be initialized");

        // Verify constants are set correctly
        assertEq(entrypoint.VERSION(), "1", "Version should be 1");
        assertTrue(entrypoint.TRAILS_INTENT_TYPEHASH() != bytes32(0), "Intent typehash should be set");

        // Verify contract has expected storage layout by checking usedIntents mapping
        // This is a smoke test that the contract is properly initialized
        bytes32 testIntentHash = keccak256("test");
        assertEq(entrypoint.usedIntents(testIntentHash), false, "usedIntents should be false for unused intent");
    }
}

