
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec, gas-indexed-events, gas-strict-inequalities, gas-increment-by-one
// solhint-disable named-parameters-mapping

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";
import { Address } from "@openzeppelin/contracts/utils/Address.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/EnumerableSet.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

import { MinimalProxyFactory } from "./MinimalProxyFactory.sol";
import { IGraphTokenLockManager } from "./IGraphTokenLockManager.sol";
import { IGraphTokenLock } from "./IGraphTokenLock.sol";
import { GraphTokenLockWallet } from "./GraphTokenLockWallet.sol";

/**
 * @title GraphTokenLockManager
 * @notice This contract manages a list of authorized function calls and targets that can be called
 * by any TokenLockWallet contract and it is a factory of TokenLockWallet contracts.
 *
 * This contract receives funds to make the process of creating TokenLockWallet contracts
 * easier by distributing them the initial tokens to be managed.
 *
 * The owner can setup a list of token destinations that will be used by TokenLock contracts to
 * approve the pulling of funds, this way in can be guaranteed that only protocol contracts
 * will manipulate users funds.
 */
contract GraphTokenLockManager is Ownable, MinimalProxyFactory, IGraphTokenLockManager {
    using SafeERC20 for IERC20;
    using EnumerableSet for EnumerableSet.AddressSet;

    // -- State --

    mapping(bytes4 => address) public authFnCalls;
    EnumerableSet.AddressSet private _tokenDestinations;

    address public masterCopy;
    IERC20 internal _token;

    // -- Events --

    event MasterCopyUpdated(address indexed masterCopy);
    event TokenLockCreated(
        address indexed contractAddress,
        bytes32 indexed initHash,
        address indexed beneficiary,
        address token,
        uint256 managedAmount,
        uint256 startTime,
        uint256 endTime,
        uint256 periods,
        uint256 releaseStartTime,
        uint256 vestingCliffTime,
        IGraphTokenLock.Revocability revocable
    );

    event TokensDeposited(address indexed sender, uint256 amount);
    event TokensWithdrawn(address indexed sender, uint256 amount);

    event FunctionCallAuth(address indexed caller, bytes4 indexed sigHash, address indexed target, string signature);
    event TokenDestinationAllowed(address indexed dst, bool allowed);

    /**
     * Constructor.
     * @param _graphToken Token to use for deposits and withdrawals
     * @param _masterCopy Address of the master copy to use to clone proxies
     */
    constructor(IERC20 _graphToken, address _masterCopy) {
        require(address(_graphToken) != address(0), "Token cannot be zero");
        _token = _graphToken;
        setMasterCopy(_masterCopy);
    }

    // -- Factory --

    /**
     * @notice Sets the masterCopy bytecode to use to create clones of TokenLock contracts
     * @param _masterCopy Address of contract bytecode to factory clone
     */
    function setMasterCopy(address _masterCopy) public override onlyOwner {
        require(_masterCopy != address(0), "MasterCopy cannot be zero");
        masterCopy = _masterCopy;
        emit MasterCopyUpdated(_masterCopy);
    }

    /**
     * @notice Creates and fund a new token lock wallet using a minimum proxy
     * @param _owner Address of the contract owner
     * @param _beneficiary Address of the beneficiary of locked tokens
     * @param _managedAmount Amount of tokens to be managed by the lock contract
     * @param _startTime Start time of the release schedule
     * @param _endTime End time of the release schedule
     * @param _periods Number of periods between start time and end time
     * @param _releaseStartTime Override time for when the releases start
     * @param _revocable Whether the contract is revocable
     */
    function createTokenLockWallet(
        address _owner,
        address _beneficiary,
        uint256 _managedAmount,
        uint256 _startTime,
        uint256 _endTime,
        uint256 _periods,
        uint256 _releaseStartTime,
        uint256 _vestingCliffTime,
        IGraphTokenLock.Revocability _revocable
    ) external override onlyOwner {
        require(_token.balanceOf(address(this)) >= _managedAmount, "Not enough tokens to create lock");

        // Create contract using a minimal proxy and call initializer
        bytes memory initializer = abi.encodeWithSelector(
            GraphTokenLockWallet.initialize.selector,
            address(this),
            _owner,
            _beneficiary,
            address(_token),
            _managedAmount,
            _startTime,
            _endTime,
            _periods,
            _releaseStartTime,
            _vestingCliffTime,
            _revocable
        );
        address contractAddress = _deployProxy2(keccak256(initializer), masterCopy, initializer);

        // Send managed amount to the created contract
        _token.safeTransfer(contractAddress, _managedAmount);

        emit TokenLockCreated(
            contractAddress,
            keccak256(initializer),
            _beneficiary,
            address(_token),
            _managedAmount,
            _startTime,
            _endTime,
            _periods,
            _releaseStartTime,
            _vestingCliffTime,
            _revocable
        );
    }

    // -- Funds Management --

    /**
     * @notice Gets the GRT token address
     * @return Token used for transfers and approvals
     */
    function token() external view override returns (IERC20) {
        return _token;
    }

    /**
     * @notice Deposits tokens into the contract
     * @dev Even if the ERC20 token can be transferred directly to the contract
     * this function provide a safe interface to do the transfer and avoid mistakes
     * @param _amount Amount to deposit
     */
    function deposit(uint256 _amount) external override {
        require(_amount > 0, "Amount cannot be zero");
        _token.safeTransferFrom(msg.sender, address(this), _amount);
        emit TokensDeposited(msg.sender, _amount);
    }

    /**
     * @notice Withdraws tokens from the contract
     * @dev Escape hatch in case of mistakes or to recover remaining funds
     * @param _amount Amount of tokens to withdraw
     */
    function withdraw(uint256 _amount) external override onlyOwner {
        require(_amount > 0, "Amount cannot be zero");
        _token.safeTransfer(msg.sender, _amount);
        emit TokensWithdrawn(msg.sender, _amount);
    }

    // -- Token Destinations --

    /**
     * @notice Adds an address that can be allowed by a token lock to pull funds
     * @param _dst Destination address
     */
    function addTokenDestination(address _dst) external override onlyOwner {
        require(_dst != address(0), "Destination cannot be zero");
        require(_tokenDestinations.add(_dst), "Destination already added");
        emit TokenDestinationAllowed(_dst, true);
    }

    /**
     * @notice Removes an address that can be allowed by a token lock to pull funds
     * @param _dst Destination address
     */
    function removeTokenDestination(address _dst) external override onlyOwner {
        require(_tokenDestinations.remove(_dst), "Destination already removed");
        emit TokenDestinationAllowed(_dst, false);
    }

    /**
     * @notice Returns True if the address is authorized to be a destination of tokens
     * @param _dst Destination address
     * @return True if authorized
     */
    function isTokenDestination(address _dst) external view override returns (bool) {
        return _tokenDestinations.contains(_dst);
    }

    /**
     * @notice Returns an array of authorized destination addresses
     * @return Array of addresses authorized to pull funds from a token lock
     */
    function getTokenDestinations() external view override returns (address[] memory) {
        address[] memory dstList = new address[](_tokenDestinations.length());
        for (uint256 i = 0; i < _tokenDestinations.length(); i++) {
            dstList[i] = _tokenDestinations.at(i);
        }
        return dstList;
    }

    // -- Function Call Authorization --

    /**
     * @notice Sets an authorized function call to target
     * @dev Input expected is the function signature as 'transfer(address,uint256)'
     * @param _signature Function signature
     * @param _target Address of the destination contract to call
     */
    function setAuthFunctionCall(string calldata _signature, address _target) external override onlyOwner {
        _setAuthFunctionCall(_signature, _target);
    }

    /**
     * @notice Unsets an authorized function call to target
     * @dev Input expected is the function signature as 'transfer(address,uint256)'
     * @param _signature Function signature
     */
    function unsetAuthFunctionCall(string calldata _signature) external override onlyOwner {
        bytes4 sigHash = _toFunctionSigHash(_signature);
        authFnCalls[sigHash] = address(0);

        emit FunctionCallAuth(msg.sender, sigHash, address(0), _signature);
    }

    /**
     * @notice Sets an authorized function call to target in bulk
     * @dev Input expected is the function signature as 'transfer(address,uint256)'
     * @param _signatures Function signatures
     * @param _targets Address of the destination contract to call
     */
    function setAuthFunctionCallMany(
        string[] calldata _signatures,
        address[] calldata _targets
    ) external override onlyOwner {
        require(_signatures.length == _targets.length, "Array length mismatch");

        for (uint256 i = 0; i < _signatures.length; i++) {
            _setAuthFunctionCall(_signatures[i], _targets[i]);
        }
    }

    /**
     * @notice Sets an authorized function call to target
     * @dev Input expected is the function signature as 'transfer(address,uint256)'
     * @dev Function signatures of Graph Protocol contracts to be used are known ahead of time
     * @param _signature Function signature
     * @param _target Address of the destination contract to call
     */
    function _setAuthFunctionCall(string calldata _signature, address _target) internal {
        require(_target != address(this), "Target must be other contract");
        require(Address.isContract(_target), "Target must be a contract");

        bytes4 sigHash = _toFunctionSigHash(_signature);
        authFnCalls[sigHash] = _target;

        emit FunctionCallAuth(msg.sender, sigHash, _target, _signature);
    }

    /**
     * @notice Gets the target contract to call for a particular function signature
     * @param _sigHash Function signature hash
     * @return Address of the target contract where to send the call
     */
    function getAuthFunctionCallTarget(bytes4 _sigHash) public view override returns (address) {
        return authFnCalls[_sigHash];
    }

    /**
     * @notice Returns true if the function call is authorized
     * @param _sigHash Function signature hash
     * @return True if authorized
     */
    function isAuthFunctionCall(bytes4 _sigHash) external view override returns (bool) {
        return getAuthFunctionCallTarget(_sigHash) != address(0);
    }

    /**
     * @dev Converts a function signature string to 4-bytes hash
     * @param _signature Function signature string
     * @return Function signature hash
     */
    function _toFunctionSigHash(string calldata _signature) internal pure returns (bytes4) {
        return _convertToBytes4(abi.encodeWithSignature(_signature));
    }

    /**
     * @dev Converts function signature bytes to function signature hash (bytes4)
     * @param _signature Function signature
     * @return Function signature in bytes4
     */
    function _convertToBytes4(bytes memory _signature) internal pure returns (bytes4) {
        require(_signature.length == 4, "Invalid method signature");
        bytes4 sigHash;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sigHash := mload(add(_signature, 32))
        }
        return sigHash;
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec

import { Address } from "@openzeppelin/contracts/utils/Address.sol";
import { Create2 } from "@openzeppelin/contracts/utils/Create2.sol";

/**
 * @title MinimalProxyFactory: a factory contract for creating minimal proxies
 * @notice Adapted from https://github.com/OpenZeppelin/openzeppelin-sdk/blob/v2.5.0/packages/lib/contracts/upgradeability/ProxyFactory.sol
 * Based on https://eips.ethereum.org/EIPS/eip-1167
 */
contract MinimalProxyFactory {
    /// @dev Emitted when a new proxy is created
    event ProxyCreated(address indexed proxy);

    /**
     * @notice Gets the deterministic CREATE2 address for MinimalProxy with a particular implementation
     * @dev Uses address(this) as deployer to compute the address. Only for backwards compatibility.
     * @param _salt Bytes32 salt to use for CREATE2
     * @param _implementation Address of the proxy target implementation
     * @return Address of the counterfactual MinimalProxy
     */
    function getDeploymentAddress(bytes32 _salt, address _implementation) public view returns (address) {
        return getDeploymentAddress(_salt, _implementation, address(this));
    }

    /**
     * @notice Gets the deterministic CREATE2 address for MinimalProxy with a particular implementation
     * @param _salt Bytes32 salt to use for CREATE2
     * @param _implementation Address of the proxy target implementation
     * @param _deployer Address of the deployer that creates the contract
     * @return Address of the counterfactual MinimalProxy
     */
    function getDeploymentAddress(
        bytes32 _salt,
        address _implementation,
        address _deployer
    ) public pure returns (address) {
        return Create2.computeAddress(_salt, keccak256(_getContractCreationCode(_implementation)), _deployer);
    }

    /**
     * @dev Deploys a MinimalProxy with CREATE2
     * @param _salt Bytes32 salt to use for CREATE2
     * @param _implementation Address of the proxy target implementation
     * @param _data Bytes with the initializer call
     * @return Address of the deployed MinimalProxy
     */
    function _deployProxy2(bytes32 _salt, address _implementation, bytes memory _data) internal returns (address) {
        address proxyAddress = Create2.deploy(0, _salt, _getContractCreationCode(_implementation));

        emit ProxyCreated(proxyAddress);

        // Call function with data
        if (_data.length > 0) {
            Address.functionCall(proxyAddress, _data);
        }

        return proxyAddress;
    }

    /**
     * @dev Gets the MinimalProxy bytecode
     * @param _implementation Address of the proxy target implementation
     * @return MinimalProxy bytecode
     */
    function _getContractCreationCode(address _implementation) internal pure returns (bytes memory) {
        bytes10 creation = 0x3d602d80600a3d3981f3;
        bytes10 prefix = 0x363d3d373d3d3d363d73;
        bytes20 targetBytes = bytes20(_implementation);
        bytes15 suffix = 0x5af43d82803e903d91602b57fd5bf3;
        return abi.encodePacked(creation, prefix, targetBytes, suffix);
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { IGraphTokenLock } from "./IGraphTokenLock.sol";

interface IGraphTokenLockManager {
    // -- Factory --

    function setMasterCopy(address _masterCopy) external;

    function createTokenLockWallet(
        address _owner,
        address _beneficiary,
        uint256 _managedAmount,
        uint256 _startTime,
        uint256 _endTime,
        uint256 _periods,
        uint256 _releaseStartTime,
        uint256 _vestingCliffTime,
        IGraphTokenLock.Revocability _revocable
    ) external;

    // -- Funds Management --

    function token() external returns (IERC20);

    function deposit(uint256 _amount) external;

    function withdraw(uint256 _amount) external;

    // -- Allowed Funds Destinations --

    function addTokenDestination(address _dst) external;

    function removeTokenDestination(address _dst) external;

    function isTokenDestination(address _dst) external view returns (bool);

    function getTokenDestinations() external view returns (address[] memory);

    // -- Function Call Authorization --

    function setAuthFunctionCall(string calldata _signature, address _target) external;

    function unsetAuthFunctionCall(string calldata _signature) external;

    function setAuthFunctionCallMany(string[] calldata _signatures, address[] calldata _targets) external;

    function getAuthFunctionCallTarget(bytes4 _sigHash) external view returns (address);

    function isAuthFunctionCall(bytes4 _sigHash) external view returns (bool);
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec

interface IGraphTokenLock {
    enum Revocability {
        NotSet,
        Enabled,
        Disabled
    }

    // -- Balances --

    function currentBalance() external view returns (uint256);

    // -- Time & Periods --

    function currentTime() external view returns (uint256);

    function duration() external view returns (uint256);

    function sinceStartTime() external view returns (uint256);

    function amountPerPeriod() external view returns (uint256);

    function periodDuration() external view returns (uint256);

    function currentPeriod() external view returns (uint256);

    function passedPeriods() external view returns (uint256);

    // -- Locking & Release Schedule --

    function availableAmount() external view returns (uint256);

    function vestedAmount() external view returns (uint256);

    function releasableAmount() external view returns (uint256);

    function totalOutstandingAmount() external view returns (uint256);

    function surplusAmount() external view returns (uint256);

    // -- Value Transfer --

    function release() external;

    function withdrawSurplus(uint256 _amount) external;

    function revoke() external;
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec, gas-increment-by-one, gas-strict-inequalities, gas-small-strings

import { Address } from "@openzeppelin/contracts/utils/Address.sol";
import { SafeMath } from "@openzeppelin/contracts/math/SafeMath.sol";

import { GraphTokenLock } from "./GraphTokenLock.sol";
import { IGraphTokenLock } from "./IGraphTokenLock.sol";
import { IGraphTokenLockManager } from "./IGraphTokenLockManager.sol";

/**
 * @title GraphTokenLockWallet
 * @notice This contract is built on top of the base GraphTokenLock functionality.
 * It allows wallet beneficiaries to use the deposited funds to perform specific function calls
 * on specific contracts.
 *
 * The idea is that supporters with locked tokens can participate in the protocol
 * but disallow any release before the vesting/lock schedule.
 * The beneficiary can issue authorized function calls to this contract that will
 * get forwarded to a target contract. A target contract is any of our protocol contracts.
 * The function calls allowed are queried to the GraphTokenLockManager, this way
 * the same configuration can be shared for all the created lock wallet contracts.
 *
 * NOTE: Contracts used as target must have its function signatures checked to avoid collisions
 * with any of this contract functions.
 * Beneficiaries need to approve the use of the tokens to the protocol contracts. For convenience
 * the maximum amount of tokens is authorized.
 * Function calls do not forward ETH value so DO NOT SEND ETH TO THIS CONTRACT.
 */
contract GraphTokenLockWallet is GraphTokenLock {
    using SafeMath for uint256;

    // -- State --

    IGraphTokenLockManager public manager;

    // -- Events --

    event ManagerUpdated(address indexed _oldManager, address indexed _newManager);
    event TokenDestinationsApproved();
    event TokenDestinationsRevoked();

    // Initializer
    function initialize(
        address _manager,
        address _owner,
        address _beneficiary,
        address _token,
        uint256 _managedAmount,
        uint256 _startTime,
        uint256 _endTime,
        uint256 _periods,
        uint256 _releaseStartTime,
        uint256 _vestingCliffTime,
        IGraphTokenLock.Revocability _revocable
    ) external {
        _initialize(
            _owner,
            _beneficiary,
            _token,
            _managedAmount,
            _startTime,
            _endTime,
            _periods,
            _releaseStartTime,
            _vestingCliffTime,
            _revocable
        );
        _setManager(_manager);
    }

    // -- Admin --

    /**
     * @notice Sets a new manager for this contract
     * @param _newManager Address of the new manager
     */
    function setManager(address _newManager) external onlyOwner {
        _setManager(_newManager);
    }

    /**
     * @dev Sets a new manager for this contract
     * @param _newManager Address of the new manager
     */
    function _setManager(address _newManager) internal {
        require(_newManager != address(0), "Manager cannot be empty");
        require(Address.isContract(_newManager), "Manager must be a contract");

        address oldManager = address(manager);
        manager = IGraphTokenLockManager(_newManager);

        emit ManagerUpdated(oldManager, _newManager);
    }

    // -- Beneficiary --

    /**
     * @notice Approves protocol access of the tokens managed by this contract
     * @dev Approves all token destinations registered in the manager to pull tokens
     */
    function approveProtocol() external onlyBeneficiary {
        address[] memory dstList = manager.getTokenDestinations();
        for (uint256 i = 0; i < dstList.length; i++) {
            // Note this is only safe because we are using the max uint256 value
            token.approve(dstList[i], type(uint256).max);
        }
        emit TokenDestinationsApproved();
    }

    /**
     * @notice Revokes protocol access of the tokens managed by this contract
     * @dev Revokes approval to all token destinations in the manager to pull tokens
     */
    function revokeProtocol() external onlyBeneficiary {
        address[] memory dstList = manager.getTokenDestinations();
        for (uint256 i = 0; i < dstList.length; i++) {
            // Note this is only safe cause we're using 0 as the amount
            token.approve(dstList[i], 0);
        }
        emit TokenDestinationsRevoked();
    }

    /**
     * @notice Forward authorized contract calls to protocol contracts
     * @dev Fallback function can be called by the beneficiary only if function call is allowed
     */
    // solhint-disable-next-line no-complex-fallback
    fallback() external {
        // Only beneficiary can forward calls
        require(msg.sender == beneficiary, "Unauthorized caller");

        // Only non-revocable contracts can forward calls
        require(revocable == Revocability.Disabled, "Revocable contracts cannot forward calls");

        // Function call validation
        address _target = manager.getAuthFunctionCallTarget(msg.sig);
        require(_target != address(0), "Unauthorized function");

        // Call function with data
        Address.functionCall(_target, msg.data);
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec, gas-indexed-events, gas-strict-inequalities, gas-small-strings

import { SafeMath } from "@openzeppelin/contracts/math/SafeMath.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";

import { Ownable as OwnableInitializable } from "./Ownable.sol";
import { MathUtils } from "./MathUtils.sol";
import { IGraphTokenLock } from "./IGraphTokenLock.sol";

/**
 * @title GraphTokenLock
 * @notice Contract that manages an unlocking schedule of tokens.
 * @dev The contract lock manage a number of tokens deposited into the contract to ensure that
 * they can only be released under certain time conditions.
 *
 * This contract implements a release scheduled based on periods and tokens are released in steps
 * after each period ends. It can be configured with one period in which case it is like a plain TimeLock.
 * It also supports revocation to be used for vesting schedules.
 *
 * The contract supports receiving extra funds than the managed tokens ones that can be
 * withdrawn by the beneficiary at any time.
 *
 * A releaseStartTime parameter is included to override the default release schedule and
 * perform the first release on the configured time. After that it will continue with the
 * default schedule.
 */
abstract contract GraphTokenLock is OwnableInitializable, IGraphTokenLock {
    using SafeMath for uint256;
    using SafeERC20 for IERC20;

    uint256 private constant MIN_PERIOD = 1;

    // -- State --

    IERC20 public token;
    address public beneficiary;

    // Configuration

    // Amount of tokens managed by the contract schedule
    uint256 public managedAmount;

    uint256 public startTime; // Start datetime (in unixtimestamp)
    uint256 public endTime; // Datetime after all funds are fully vested/unlocked (in unixtimestamp)
    uint256 public periods; // Number of vesting/release periods

    // First release date for tokens (in unixtimestamp)
    // If set, no tokens will be released before releaseStartTime ignoring
    // the amount to release each period
    uint256 public releaseStartTime;
    // A cliff set a date to which a beneficiary needs to get to vest
    // all preceding periods
    uint256 public vestingCliffTime;
    IGraphTokenLock.Revocability public revocable; // Whether to use vesting for locked funds

    // State

    bool public isRevoked;
    bool public isInitialized;
    bool public isAccepted;
    uint256 public releasedAmount;
    uint256 public revokedAmount;

    // -- Events --

    event TokensReleased(address indexed beneficiary, uint256 amount);
    event TokensWithdrawn(address indexed beneficiary, uint256 amount);
    event TokensRevoked(address indexed beneficiary, uint256 amount);
    event BeneficiaryChanged(address newBeneficiary);
    event LockAccepted();
    event LockCanceled();

    /**
     * @dev Only allow calls from the beneficiary of the contract
     */
    modifier onlyBeneficiary() {
        require(msg.sender == beneficiary, "!auth");
        _;
    }

    /**
     * @notice Initializes the contract
     * @param _owner Address of the contract owner
     * @param _beneficiary Address of the beneficiary of locked tokens
     * @param _managedAmount Amount of tokens to be managed by the lock contract
     * @param _startTime Start time of the release schedule
     * @param _endTime End time of the release schedule
     * @param _periods Number of periods between start time and end time
     * @param _releaseStartTime Override time for when the releases start
     * @param _vestingCliffTime Override time for when the vesting start
     * @param _revocable Whether the contract is revocable
     */
    function _initialize(
        address _owner,
        address _beneficiary,
        address _token,
        uint256 _managedAmount,
        uint256 _startTime,
        uint256 _endTime,
        uint256 _periods,
        uint256 _releaseStartTime,
        uint256 _vestingCliffTime,
        IGraphTokenLock.Revocability _revocable
    ) internal {
        require(!isInitialized, "Already initialized");
        require(_owner != address(0), "Owner cannot be zero");
        require(_beneficiary != address(0), "Beneficiary cannot be zero");
        require(_token != address(0), "Token cannot be zero");
        require(_managedAmount > 0, "Managed tokens cannot be zero");
        require(_startTime != 0, "Start time must be set");
        require(_startTime < _endTime, "Start time > end time");
        require(_periods >= MIN_PERIOD, "Periods cannot be below minimum");
        require(_revocable != IGraphTokenLock.Revocability.NotSet, "Must set a revocability option");
        require(_releaseStartTime < _endTime, "Release start time must be before end time");
        require(_vestingCliffTime < _endTime, "Cliff time must be before end time");

        isInitialized = true;

        OwnableInitializable._initialize(_owner);
        beneficiary = _beneficiary;
        token = IERC20(_token);

        managedAmount = _managedAmount;

        startTime = _startTime;
        endTime = _endTime;
        periods = _periods;

        // Optionals
        releaseStartTime = _releaseStartTime;
        vestingCliffTime = _vestingCliffTime;
        revocable = _revocable;
    }

    /**
     * @notice Change the beneficiary of funds managed by the contract
     * @dev Can only be called by the beneficiary
     * @param _newBeneficiary Address of the new beneficiary address
     */
    function changeBeneficiary(address _newBeneficiary) external onlyBeneficiary {
        require(_newBeneficiary != address(0), "Empty beneficiary");
        beneficiary = _newBeneficiary;
        emit BeneficiaryChanged(_newBeneficiary);
    }

    /**
     * @notice Beneficiary accepts the lock, the owner cannot retrieve back the tokens
     * @dev Can only be called by the beneficiary
     */
    function acceptLock() external onlyBeneficiary {
        isAccepted = true;
        emit LockAccepted();
    }

    /**
     * @notice Owner cancel the lock and return the balance in the contract
     * @dev Can only be called by the owner
     */
    function cancelLock() external onlyOwner {
        require(isAccepted == false, "Cannot cancel accepted contract");

        token.safeTransfer(owner(), currentBalance());

        emit LockCanceled();
    }

    // -- Balances --

    /**
     * @notice Returns the amount of tokens currently held by the contract
     * @return Tokens held in the contract
     */
    function currentBalance() public view override returns (uint256) {
        return token.balanceOf(address(this));
    }

    // -- Time & Periods --

    /**
     * @notice Returns the current block timestamp
     * @return Current block timestamp
     */
    function currentTime() public view override returns (uint256) {
        return block.timestamp;
    }

    /**
     * @notice Gets duration of contract from start to end in seconds
     * @return Amount of seconds from contract startTime to endTime
     */
    function duration() public view override returns (uint256) {
        return endTime.sub(startTime);
    }

    /**
     * @notice Gets time elapsed since the start of the contract
     * @dev Returns zero if called before conctract starTime
     * @return Seconds elapsed from contract startTime
     */
    function sinceStartTime() public view override returns (uint256) {
        uint256 current = currentTime();
        if (current <= startTime) {
            return 0;
        }
        return current.sub(startTime);
    }

    /**
     * @notice Returns amount available to be released after each period according to schedule
     * @return Amount of tokens available after each period
     */
    function amountPerPeriod() public view override returns (uint256) {
        return managedAmount.div(periods);
    }

    /**
     * @notice Returns the duration of each period in seconds
     * @return Duration of each period in seconds
     */
    function periodDuration() public view override returns (uint256) {
        return duration().div(periods);
    }

    /**
     * @notice Gets the current period based on the schedule
     * @return A number that represents the current period
     */
    function currentPeriod() public view override returns (uint256) {
        return sinceStartTime().div(periodDuration()).add(MIN_PERIOD);
    }

    /**
     * @notice Gets the number of periods that passed since the first period
     * @return A number of periods that passed since the schedule started
     */
    function passedPeriods() public view override returns (uint256) {
        return currentPeriod().sub(MIN_PERIOD);
    }

    // -- Locking & Release Schedule --

    /**
     * @notice Gets the currently available token according to the schedule
     * @dev Implements the step-by-step schedule based on periods for available tokens
     * @return Amount of tokens available according to the schedule
     */
    function availableAmount() public view override returns (uint256) {
        uint256 current = currentTime();

        // Before contract start no funds are available
        if (current < startTime) {
            return 0;
        }

        // After contract ended all funds are available
        if (current > endTime) {
            return managedAmount;
        }

        // Get available amount based on period
        return passedPeriods().mul(amountPerPeriod());
    }

    /**
     * @notice Gets the amount of currently vested tokens
     * @dev Similar to available amount, but is fully vested when contract is non-revocable
     * @return Amount of tokens already vested
     */
    function vestedAmount() public view override returns (uint256) {
        // If non-revocable it is fully vested
        if (revocable == IGraphTokenLock.Revocability.Disabled) {
            return managedAmount;
        }

        // Vesting cliff is activated and it has not passed means nothing is vested yet
        if (vestingCliffTime > 0 && currentTime() < vestingCliffTime) {
            return 0;
        }

        return availableAmount();
    }

    /**
     * @notice Gets tokens currently available for release
     * @dev Considers the schedule and takes into account already released tokens
     * @return Amount of tokens ready to be released
     */
    function releasableAmount() public view virtual override returns (uint256) {
        // If a release start time is set no tokens are available for release before this date
        // If not set it follows the default schedule and tokens are available on
        // the first period passed
        if (releaseStartTime > 0 && currentTime() < releaseStartTime) {
            return 0;
        }

        // Vesting cliff is activated and it has not passed means nothing is vested yet
        // so funds cannot be released
        if (
            revocable == IGraphTokenLock.Revocability.Enabled &&
            vestingCliffTime > 0 &&
            currentTime() < vestingCliffTime
        ) {
            return 0;
        }

        // A beneficiary can never have more releasable tokens than the contract balance
        uint256 releasable = availableAmount().sub(releasedAmount);
        return MathUtils.min(currentBalance(), releasable);
    }

    /**
     * @notice Gets the outstanding amount yet to be released based on the whole contract lifetime
     * @dev Does not consider schedule but just global amounts tracked
     * @return Amount of outstanding tokens for the lifetime of the contract
     */
    function totalOutstandingAmount() public view override returns (uint256) {
        return managedAmount.sub(releasedAmount).sub(revokedAmount);
    }

    /**
     * @notice Gets surplus amount in the contract based on outstanding amount to release
     * @dev All funds over outstanding amount is considered surplus that can be withdrawn by beneficiary.
     * Note this might not be the correct value for wallets transferred to L2 (i.e. an L2GraphTokenLockWallet), as the released amount will be
     * skewed, so the beneficiary might have to bridge back to L1 to release the surplus.
     * @return Amount of tokens considered as surplus
     */
    function surplusAmount() public view override returns (uint256) {
        uint256 balance = currentBalance();
        uint256 outstandingAmount = totalOutstandingAmount();
        if (balance > outstandingAmount) {
            return balance.sub(outstandingAmount);
        }
        return 0;
    }

    // -- Value Transfer --

    /**
     * @notice Releases tokens based on the configured schedule
     * @dev All available releasable tokens are transferred to beneficiary
     */
    function release() external override onlyBeneficiary {
        uint256 amountToRelease = releasableAmount();
        require(amountToRelease > 0, "No available releasable amount");

        releasedAmount = releasedAmount.add(amountToRelease);

        token.safeTransfer(beneficiary, amountToRelease);

        emit TokensReleased(beneficiary, amountToRelease);
    }

    /**
     * @notice Withdraws surplus, unmanaged tokens from the contract
     * @dev Tokens in the contract over outstanding amount are considered as surplus
     * @param _amount Amount of tokens to withdraw
     */
    function withdrawSurplus(uint256 _amount) external override onlyBeneficiary {
        require(_amount > 0, "Amount cannot be zero");
        require(surplusAmount() >= _amount, "Amount requested > surplus available");

        token.safeTransfer(beneficiary, _amount);

        emit TokensWithdrawn(beneficiary, _amount);
    }

    /**
     * @notice Revokes a vesting schedule and return the unvested tokens to the owner
     * @dev Vesting schedule is always calculated based on managed tokens
     */
    function revoke() external override onlyOwner {
        require(revocable == IGraphTokenLock.Revocability.Enabled, "Contract is non-revocable");
        require(isRevoked == false, "Already revoked");

        uint256 unvestedAmount = managedAmount.sub(vestedAmount());
        require(unvestedAmount > 0, "No available unvested amount");

        revokedAmount = unvestedAmount;
        isRevoked = true;

        token.safeTransfer(owner(), unvestedAmount);

        emit TokensRevoked(beneficiary, unvestedAmount);
    }
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 
// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-indexed-events, use-natspec
// solhint-disable named-parameters-mapping

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";

import { ICallhookReceiver } from "@graphprotocol/interfaces/contracts/contracts/gateway/ICallhookReceiver.sol";
import { GraphTokenLockManager } from "./GraphTokenLockManager.sol";
import { L2GraphTokenLockWallet } from "./L2GraphTokenLockWallet.sol";

/**
 * @title L2GraphTokenLockManager
 * @notice This contract manages a list of authorized function calls and targets that can be called
 * by any TokenLockWallet contract and it is a factory of TokenLockWallet contracts.
 *
 * This contract receives funds to make the process of creating TokenLockWallet contracts
 * easier by distributing them the initial tokens to be managed.
 *
 * In particular, this L2 variant is designed to receive token lock wallets from L1,
 * through the GRT bridge. These transferred wallets will not allow releasing funds in L2 until
 * the end of the vesting timeline, but they can allow withdrawing funds back to L1 using
 * the L2GraphTokenLockTransferTool contract.
 *
 * The owner can setup a list of token destinations that will be used by TokenLock contracts to
 * approve the pulling of funds, this way in can be guaranteed that only protocol contracts
 * will manipulate users funds.
 */
contract L2GraphTokenLockManager is GraphTokenLockManager, ICallhookReceiver {
    using SafeERC20 for IERC20;

    /// @dev Struct to hold the data of a transferred wallet; this is
    /// the data that must be encoded in L1 to send a wallet to L2.
    struct TransferredWalletData {
        address l1Address;
        address owner;
        address beneficiary;
        uint256 managedAmount;
        uint256 startTime;
        uint256 endTime;
    }

    /// Address of the L2GraphTokenGateway
    // solhint-disable-next-line immutable-vars-naming
    address public immutable l2Gateway;
    /// Address of the L1 transfer tool contract (in L1, no aliasing)
    // solhint-disable-next-line immutable-vars-naming
    address public immutable l1TransferTool;
    /// Mapping of each L1 wallet to its L2 wallet counterpart (populated when each wallet is received)
    /// L1 address => L2 address
    mapping(address => address) public l1WalletToL2Wallet;
    /// Mapping of each L2 wallet to its L1 wallet counterpart (populated when each wallet is received)
    /// L2 address => L1 address
    mapping(address => address) public l2WalletToL1Wallet;

    /// @dev Event emitted when a wallet is received and created from L1
    event TokenLockCreatedFromL1(
        address indexed contractAddress,
        bytes32 initHash,
        address indexed beneficiary,
        uint256 managedAmount,
        uint256 startTime,
        uint256 endTime,
        address indexed l1Address
    );

    /// @dev Emitted when locked tokens are received from L1 (whether the wallet
    /// had already been received or not)
    event LockedTokensReceivedFromL1(address indexed l1Address, address indexed l2Address, uint256 amount);

    /**
     * @dev Checks that the sender is the L2GraphTokenGateway.
     */
    modifier onlyL2Gateway() {
        require(msg.sender == l2Gateway, "ONLY_GATEWAY");
        _;
    }

    /**
     * @notice Constructor for the L2GraphTokenLockManager contract.
     * @param _graphToken Address of the L2 GRT token contract
     * @param _masterCopy Address of the master copy of the L2GraphTokenLockWallet implementation
     * @param _l2Gateway Address of the L2GraphTokenGateway contract
     * @param _l1TransferTool Address of the L1 transfer tool contract (in L1, without aliasing)
     */
    constructor(
        IERC20 _graphToken,
        address _masterCopy,
        address _l2Gateway,
        address _l1TransferTool
    ) GraphTokenLockManager(_graphToken, _masterCopy) {
        l2Gateway = _l2Gateway;
        l1TransferTool = _l1TransferTool;
    }

    /**
     * @notice This function is called by the L2GraphTokenGateway when tokens are sent from L1.
     * @dev This function will create a new wallet if it doesn't exist yet, or send the tokens to
     * the existing wallet if it does.
     * @param _from Address of the sender in L1, which must be the L1GraphTokenLockTransferTool
     * @param _amount Amount of tokens received
     * @param _data Encoded data of the transferred wallet, which must be an ABI-encoded TransferredWalletData struct
     */
    function onTokenTransfer(address _from, uint256 _amount, bytes calldata _data) external override onlyL2Gateway {
        require(_from == l1TransferTool, "ONLY_TRANSFER_TOOL");
        TransferredWalletData memory walletData = abi.decode(_data, (TransferredWalletData));

        if (l1WalletToL2Wallet[walletData.l1Address] != address(0)) {
            // If the wallet was already received, just send the tokens to the L2 address
            _token.safeTransfer(l1WalletToL2Wallet[walletData.l1Address], _amount);
        } else {
            // Create contract using a minimal proxy and call initializer
            (bytes32 initHash, address contractAddress) = _deployFromL1(keccak256(_data), walletData);
            l1WalletToL2Wallet[walletData.l1Address] = contractAddress;
            l2WalletToL1Wallet[contractAddress] = walletData.l1Address;

            // Send managed amount to the created contract
            _token.safeTransfer(contractAddress, _amount);

            emit TokenLockCreatedFromL1(
                contractAddress,
                initHash,
                walletData.beneficiary,
                walletData.managedAmount,
                walletData.startTime,
                walletData.endTime,
                walletData.l1Address
            );
        }
        emit LockedTokensReceivedFromL1(walletData.l1Address, l1WalletToL2Wallet[walletData.l1Address], _amount);
    }

    /**
     * @dev Deploy a token lock wallet with data received from L1
     * @param _salt Salt for the CREATE2 call, which must be the hash of the wallet data
     * @param _walletData Data of the wallet to be created
     * @return Hash of the initialization calldata
     * @return Address of the created contract
     */
    function _deployFromL1(
        bytes32 _salt,
        TransferredWalletData memory _walletData
    ) internal returns (bytes32, address) {
        bytes memory initializer = _encodeInitializer(_walletData);
        address contractAddress = _deployProxy2(_salt, masterCopy, initializer);
        return (keccak256(initializer), contractAddress);
    }

    /**
     * @dev Encode the initializer for the token lock wallet received from L1
     * @param _walletData Data of the wallet to be created
     * @return Encoded initializer calldata, including the function signature
     */
    function _encodeInitializer(TransferredWalletData memory _walletData) internal view returns (bytes memory) {
        return
            abi.encodeWithSelector(
                L2GraphTokenLockWallet.initializeFromL1.selector,
                address(this),
                address(_token),
                _walletData
            );
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { GraphTokenLockWallet } from "./GraphTokenLockWallet.sol";
import { Ownable as OwnableInitializable } from "./Ownable.sol";
import { L2GraphTokenLockManager } from "./L2GraphTokenLockManager.sol";

/**
 * @title L2GraphTokenLockWallet
 * @notice This contract is built on top of the base GraphTokenLock functionality.
 * It allows wallet beneficiaries to use the deposited funds to perform specific function calls
 * on specific contracts.
 *
 * The idea is that supporters with locked tokens can participate in the protocol
 * but disallow any release before the vesting/lock schedule.
 * The beneficiary can issue authorized function calls to this contract that will
 * get forwarded to a target contract. A target contract is any of our protocol contracts.
 * The function calls allowed are queried to the GraphTokenLockManager, this way
 * the same configuration can be shared for all the created lock wallet contracts.
 *
 * This L2 variant includes a special initializer so that it can be created from
 * a wallet's data received from L1. These transferred wallets will not allow releasing
 * funds in L2 until the end of the vesting timeline, but they can allow withdrawing
 * funds back to L1 using the L2GraphTokenLockTransferTool contract.
 *
 * Note that surplusAmount and releasedAmount in L2 will be skewed for wallets received from L1,
 * so releasing surplus tokens might also only be possible by bridging tokens back to L1.
 *
 * NOTE: Contracts used as target must have its function signatures checked to avoid collisions
 * with any of this contract functions.
 * Beneficiaries need to approve the use of the tokens to the protocol contracts. For convenience
 * the maximum amount of tokens is authorized.
 * Function calls do not forward ETH value so DO NOT SEND ETH TO THIS CONTRACT.
 */
contract L2GraphTokenLockWallet is GraphTokenLockWallet {
    // Initializer when created from a message from L1
    function initializeFromL1(
        address _manager,
        address _token,
        L2GraphTokenLockManager.TransferredWalletData calldata _walletData
    ) external {
        require(!isInitialized, "Already initialized");
        isInitialized = true;

        OwnableInitializable._initialize(_walletData.owner);
        beneficiary = _walletData.beneficiary;
        token = IERC20(_token);

        managedAmount = _walletData.managedAmount;

        startTime = _walletData.startTime;
        endTime = _walletData.endTime;
        periods = 1;
        isAccepted = true;

        // Optionals
        releaseStartTime = _walletData.endTime;
        revocable = Revocability.Disabled;

        _setManager(_manager);
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec

import { GraphTokenLock } from "./GraphTokenLock.sol";
import { IGraphTokenLock } from "./IGraphTokenLock.sol";
import { Ownable as OwnableInitializable } from "./Ownable.sol";

/**
 * @title GraphTokenLockSimple
 * @notice This contract is the concrete simple implementation built on top of the base
 * GraphTokenLock functionality for use when we only need the token lock schedule
 * features but no interaction with the network.
 *
 * This contract is designed to be deployed without the use of a TokenManager.
 */
contract GraphTokenLockSimple is GraphTokenLock {
    // Constructor
    constructor() {
        OwnableInitializable._initialize(msg.sender);
    }

    // Initializer
    function initialize(
        address _owner,
        address _beneficiary,
        address _token,
        uint256 _managedAmount,
        uint256 _startTime,
        uint256 _endTime,
        uint256 _periods,
        uint256 _releaseStartTime,
        uint256 _vestingCliffTime,
        IGraphTokenLock.Revocability _revocable
    ) external onlyOwner {
        _initialize(
            _owner,
            _beneficiary,
            _token,
            _managedAmount,
            _startTime,
            _endTime,
            _periods,
            _releaseStartTime,
            _vestingCliffTime,
            _revocable
        );
    }
}


## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

