
## *MAIN TARGET CONTRACT* TO REVIEW

//SPDX-License-Identifier: UNLICENSED

/*
Copyright (C) 2025 Coordination Inc.
All rights reserved.

This software is proprietary and confidential. Unauthorized copying,
distribution, or use is strictly prohibited and may result in legal action.

For licensing inquiries: legal@coordinationlabs.com
*/

pragma solidity ^0.8.28;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { IEntropyConsumer } from "@pythnetwork/entropy-sdk-solidity/IEntropyConsumer.sol";
import { IEntropyV2 } from "@pythnetwork/entropy-sdk-solidity/IEntropyV2.sol";

import { FisherYatesRejection } from "./lib/FisherYatesWithRejection.sol";
import { IScaledEntropyProvider } from "./interfaces/IScaledEntropyProvider.sol";

/**
 * @title ScaledEntropyProvider
 * @notice Provides scaled random number generation using Pyth Network entropy with callback functionality
 * @dev Integrates with Pyth Network's entropy service to generate cryptographically secure random numbers:
 *      - Handles entropy requests with custom scaling and range parameters
 *      - Supports both sampling with and without replacement using Fisher-Yates algorithm
 *      - Provides callback mechanism for asynchronous random number delivery
 *      - Implements unbiased rejection sampling to prevent modulo bias
 *      - Manages fee payments to entropy providers
 *      - Stores pending requests and validates callback execution
 */
contract ScaledEntropyProvider is Ownable, IScaledEntropyProvider, IEntropyConsumer {
    // =============================================================
    //                           STRUCTS
    // =============================================================
    struct PendingRequest {
        address callback;
        bytes4 selector;
        bytes context;
        bytes32 userRandomNumber;
        SetRequest[] setRequests;
    }

    // =============================================================
    //                           EVENTS
    // =============================================================

    event ScaledRandomnessDelivered(uint64 indexed sequence, address indexed callback, uint256 samples);
    event EntropyFulfilled(uint64 indexed sequence, bytes32 randomNumber);

    // =============================================================
    //                           ERRORS
    // =============================================================
    error InvalidCallback();
    error CallbackFailed(bytes4 selector);
    error ZeroAddress();
    error InvalidSelector();
    error InvalidRequests();
    error InvalidRange();
    error InvalidSamples();
    error InsufficientFee();
    error UnknownSequence();

    // =============================================================
    //                       STATE VARIABLES
    // =============================================================

    IEntropyV2 private entropy;
    address private entropyProvider;
    mapping(uint64 => PendingRequest) private pending;

    // =============================================================
    //                         CONSTRUCTOR
    // =============================================================
    
    /**
     * @notice Initializes the ScaledEntropyProvider with Pyth Network entropy configuration
     * @dev Sets up connections to Pyth Network entropy contract and provider.
     *      Both addresses are validated and stored as immutable references.
     * @param _entropy Address of the Pyth Network entropy contract
     * @param _entropyProvider Address of the specific entropy provider to use
     * @custom:requirements
     * - Entropy contract address must not be zero
     * - Entropy provider address must not be zero
     * @custom:effects
     * - Sets immutable entropy contract reference
     * - Configures entropy provider for fee calculations
     * - Sets deployer as contract owner
     * @custom:security
     * - Address validation prevents zero address configuration
     * - Immutable references prevent unauthorized changes
     * - Owner-based access control for administrative functions
     */
    constructor(address _entropy, address _entropyProvider) Ownable(msg.sender) {
        if (_entropy == address(0)) revert ZeroAddress();
        if (_entropyProvider == address(0)) revert ZeroAddress();
        entropy = IEntropyV2(_entropy);
        entropyProvider = _entropyProvider;
    }

    // =============================================================
    //                      EXTERNAL FUNCTIONS
    // =============================================================

    /**
     * @notice Requests scaled random numbers from Pyth Network with callback delivery
     * @dev Submits entropy request to Pyth Network and stores callback details for async delivery.
     *      The callback will receive scaled random numbers according to the specified requests. Developer
     *      needs to ensure that the range is not too large to be able to build an array of the appropriate
     *      size in memory in order to avoid out of gas errors during Fisher-Yates sampling.
     *      IMPORTANT: The callback address is automatically set to msg.sender (the calling contract).
     * @param _gasLimit Gas limit for the entropy callback execution
     * @param _requests Array of SetRequest structs defining random number requirements
     * @param _selector Function selector for the callback method on the calling contract
     * @param _context Additional data to pass to the callback
     * @return sequence Unique identifier for tracking this entropy request
     * @custom:requirements
     * - Calling contract (msg.sender) must implement the callback function
     * - Provided fee (msg.value) must meet minimum requirements
     * - Function selector must not be zero
     * - All set requests must be valid (proper ranges and sample counts)
     * @custom:emits None (events emitted in callback)
     * @custom:effects
     * - Submits entropy request to Pyth Network
     * - Stores pending request details with msg.sender as callback address
     * - Transfers fee to entropy provider
     * @custom:security
     * - Callback address is restricted to msg.sender preventing unauthorized callbacks
     * - Fee validation ensures sufficient payment
     * - Request validation prevents invalid random number generation
     */
    function requestAndCallbackScaledRandomness(
        uint32 _gasLimit,
        SetRequest[] memory _requests,
        bytes4 _selector,
        bytes memory _context
    )
        external
        payable
        returns (uint64 sequence)
    {
        // We assume that the caller has already checked that the fee is sufficient
        if (msg.value < getFee(_gasLimit)) revert InsufficientFee();
        if (_selector == bytes4(0)) revert InvalidSelector();
        _validateRequests(_requests);

        sequence = entropy.requestV2{value: msg.value}(entropyProvider, _gasLimit);
        _storePendingRequest(sequence, _selector, _context, _requests);
    }

    /**
     * @notice Returns the fee required for an entropy request with specified gas limit
     * @dev Queries the Pyth Network entropy contract for current fee requirements.
     *      Fee covers entropy generation and callback execution costs.
     * @param _gasLimit Gas limit for the callback execution
     * @return Fee amount in wei required for the entropy request
     */
    function getFee(uint32 _gasLimit) public view returns (uint256) {
        return entropy.getFeeV2(entropyProvider, _gasLimit);
    }

    /**
     * @notice Returns the address of the Pyth Network entropy contract
     * @dev Provides access to the entropy contract address for integration purposes.
     * @return Address of the entropy contract
     */
    function getEntropyContract() external view returns (address) {
        return address(entropy);
    }

    /**
     * @notice Returns the address of the currently configured entropy provider
     * @dev Shows which entropy provider is being used for fee calculations and requests.
     * @return Address of the entropy provider
     */
    function getEntropyProvider() external view returns (address) {
        return entropyProvider;
    }

    /**
     * @notice Returns the details of a pending entropy request
     * @dev Retrieves stored request information for a specific sequence number.
     *      Useful for debugging and monitoring pending requests.
     * @param sequence Unique identifier of the entropy request
     * @return PendingRequest struct containing callback details and request parameters
     */
    function getPendingRequest(uint64 sequence) external view returns (PendingRequest memory) {
        return pending[sequence];
    }

    // =============================================================
    //                      ADMIN FUNCTIONS
    // =============================================================

    /**
     * @notice Updates the entropy provider address
     * @dev Changes which entropy provider is used for fee calculations and requests.
     *      Only affects future requests, not pending ones.
     * @param _entropyProvider New entropy provider address
     * @custom:requirements
     * - Only owner can call
     * - Provider address must not be zero
     * @custom:emits None
     * @custom:effects
     * - Updates entropy provider for future requests
     * - Changes fee calculations for new requests
     * @custom:security
     * - Owner-only access restriction
     * - Zero address validation
     */
    function setEntropyProvider(address _entropyProvider) external onlyOwner {
        if (_entropyProvider == address(0)) revert ZeroAddress();
        entropyProvider = _entropyProvider;
    }

    // =============================================================
    //                      INTERNAL FUNCTIONS
    // =============================================================

    /**
     * @notice Processes entropy callback from Pyth Network and delivers scaled random numbers
     * @dev Called by Pyth Network when entropy is available. Processes the raw entropy into scaled
     *      random numbers according to stored request parameters and delivers via callback.
     *      This is the core function that bridges Pyth entropy with application-specific randomness.
     * @param sequence Unique identifier for the entropy request
     * @param randomNumber Raw entropy value from Pyth Network (provider parameter ignored)
     * @custom:requirements
     * - Sequence must correspond to a valid pending request
     * - Callback execution must succeed
     * - Only called by Pyth Network entropy contract
     * @custom:emits EntropyFulfilled with sequence and raw random number
     * @custom:emits ScaledRandomnessDelivered with sequence, callback address, and sample count
     * @custom:effects
     * - Retrieves and deletes pending request data
     * - Generates scaled random numbers using Fisher-Yates or replacement sampling
     * - Executes callback with scaled results and original context
     * - Cleans up pending request storage
     * @custom:security
     * - Validates sequence corresponds to pending request
     * - Ensures callback execution succeeds before cleanup
     * - Uses unbiased sampling methods to prevent statistical attacks
     * - Immediate cleanup prevents replay attacks
     */
    function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override {
        PendingRequest memory req = pending[sequence];
        if (req.callback == address(0)) revert UnknownSequence();
        
        delete pending[sequence];

        uint256[][] memory scaledRandomNumbers = _getScaledRandomness(randomNumber, req.setRequests);
        (bool success, ) = req.callback.call(abi.encodeWithSelector(req.selector, sequence, scaledRandomNumbers, req.context));
        if (!success) revert CallbackFailed(req.selector);

        emit EntropyFulfilled(sequence, randomNumber);
        emit ScaledRandomnessDelivered(sequence, req.callback, scaledRandomNumbers.length);
    }

    function _getScaledRandomness(
        bytes32 _randomNumber,
        SetRequest[] memory _setRequests
    )
        internal
        pure
        returns (uint256[][] memory requestsOutputs)
    {
        requestsOutputs = new uint256[][](_setRequests.length);
        
        for (uint256 i = 0; i < _setRequests.length; i++) {
            if (!_setRequests[i].withReplacement) {
                requestsOutputs[i] = FisherYatesRejection.draw(
                    _setRequests[i].minRange,
                    _setRequests[i].maxRange,
                    _setRequests[i].samples,
                    uint256(_randomNumber)
                );
            } else {
                requestsOutputs[i] = _drawWithReplacement(
                    _setRequests[i].minRange,
                    _setRequests[i].maxRange,
                    _setRequests[i].samples,
                    uint256(_randomNumber)
                );
            }
        }
    }

    function getEntropy() internal view override returns (address) {
        return address(entropy);
    }

    function _validateRequests(SetRequest[] memory _requests) internal pure {
        if (_requests.length == 0) revert InvalidRequests();
        for (uint256 i = 0; i < _requests.length; i++) {
            if (_requests[i].minRange > _requests[i].maxRange) revert InvalidRange();
            if (_requests[i].samples == 0) revert InvalidSamples();
        }
    }

    function _storePendingRequest(
        uint64 sequence,
        bytes4 _selector,
        bytes memory _context,
        SetRequest[] memory _setRequests
    ) internal {
        pending[sequence].callback = msg.sender;
        pending[sequence].selector = _selector;
        pending[sequence].context = _context;
        for (uint256 i = 0; i < _setRequests.length; i++) {
            pending[sequence].setRequests.push(_setRequests[i]);
        }
    }

    function _drawWithReplacement(
        uint256 _minRange,
        uint256 _maxRange,
        uint8 _samples,
        uint256 _randomNumber
    ) internal pure returns (uint256[] memory) {
        uint256[] memory result = new uint256[](_samples);
        uint256 range = _maxRange - _minRange + 1;
        uint256 nonce = 0;

        for (uint256 i = 0; i < _samples; i++) {
            uint256 rand;
            while (true) {
                rand = uint256(keccak256(abi.encode(_randomNumber, nonce)));
                uint256 limit = (type(uint256).max / range) * range;

                if (rand < limit) {
                    result[i] = uint256((rand % range) + _minRange); // [1..range]
                    break;
                }
                nonce++;
            }
            nonce++;
        }

        return result;
    }
}
END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

/**
 * @title FisherYatesRejection
 * @notice Library implementing Fisher-Yates shuffle with rejection sampling for unbiased random selection
 * @dev Provides cryptographically secure random number selection without modulo bias:
 *      - Uses Fisher-Yates shuffle algorithm for uniform distribution
 *      - Implements rejection sampling to eliminate modulo bias
 *      - Supports configurable range and sample count
 *      - Ensures each selected number has equal probability
 *      - Optimized for jackpot drawing and other applications requiring provable fairness
 *      - Returns selections without replacement (no duplicates)
 */
library FisherYatesRejection {
    uint256 constant MAX_UINT = type(uint256).max;

    /**
     * @notice Generates random numbers using Fisher-Yates shuffle with rejection sampling
     * @dev Implements unbiased random selection by:
     *      1. Building a pool of all numbers in the specified range
     *      2. Using Fisher-Yates shuffle with rejection sampling to avoid modulo bias
     *      3. Selecting the first 'count' numbers from the shuffled pool
     *      The rejection sampling ensures uniform distribution by rejecting random values
     *      that would create bias when reduced to the required range. Developer needs to ensure
     *      that the range is not too large to be able to build an array of the appropriate size
     *      in memory.
     * @param minRange Minimum value in the selection range (inclusive)
     * @param maxRange Maximum value in the selection range (inclusive)
     * @param count Number of unique values to select
     * @param seed Cryptographic seed for random number generation
     * @return result Array of selected numbers in the order they were shuffled
     * @custom:requirements
     * - count must be <= (maxRange - minRange + 1) to ensure sufficient pool size
     * - minRange must be <= maxRange for valid range
     * - seed should be cryptographically secure for unbiased results
     * @custom:effects
     * - Returns 'count' unique numbers from the specified range
     * - Each number in range has equal probability of selection
     * - No duplicates in the result array
     * @custom:security
     * - Rejection sampling eliminates modulo bias
     * - Fisher-Yates algorithm ensures uniform distribution
     * - Deterministic output for given seed enables verification
     * - Gas usage scales with rejection rate (worst case for biased ranges)
     */
    function draw(
        uint256 minRange,
        uint256 maxRange,
        uint256 count,
        uint256 seed
    ) external pure returns (uint256[] memory result) {
        require(count <= maxRange - minRange + 1, "Too many draws");

        // Build pool [1, 2, ..., range]
        uint256 rangeSize = maxRange - minRange + 1;
        uint256[] memory pool = new uint256[](rangeSize);
        for (uint256 i = 0; i < rangeSize; i++) {
            pool[i] = i + minRange;
        }

        uint256 nonce = 0;

        // Fisher-Yates shuffle with rejection sampling
        for (uint256 i = rangeSize - 1; i > 0; i--) {
            uint256 rand;
            while (true) {
                rand = uint256(keccak256(abi.encode(seed, nonce)));
                uint256 limit = (MAX_UINT / (i + 1)) * (i + 1);

                if (rand < limit) {
                    rand = rand % (i + 1);
                    break;
                }
                nonce++;
            }

            // Swap pool[i] and pool[rand]
            (pool[i], pool[rand]) = (pool[rand], pool[i]);
            nonce++;
        }

        // Take first `count` numbers
        result = new uint256[](count);
        for (uint256 j = 0; j < count; j++) {
            result[j] = pool[j];
        }
    }
}

// SPDX-License-Identifier: Apache 2
pragma solidity ^0.8.0;

import "./EntropyEvents.sol";
import "./EntropyEventsV2.sol";
import "./EntropyStructsV2.sol";

interface IEntropyV2 is EntropyEventsV2 {
    /// @notice Request a random number using the default provider with default gas limit
    /// @return assignedSequenceNumber A unique identifier for this request
    /// @dev The address calling this function should be a contract that inherits from the IEntropyConsumer interface.
    /// The `entropyCallback` method on that interface will receive a callback with the returned sequence number and
    /// the generated random number.
    ///
    /// `entropyCallback` will be run with the `gasLimit` provided to this function.
    /// The `gasLimit` will be rounded up to a multiple of 10k (e.g., 19000 -> 20000), and furthermore is lower bounded
    /// by the provider's configured default limit.
    ///
    /// This method will revert unless the caller provides a sufficient fee (at least `getFeeV2()`) as msg.value.
    /// Note that the fee can change over time. Callers of this method should explicitly compute `getFeeV2()`
    /// prior to each invocation (as opposed to hardcoding a value). Further note that excess value is *not* refunded to the caller.
    ///
    /// Note that this method uses an in-contract PRNG to generate the user's contribution to the random number.
    /// This approach modifies the security guarantees such that a dishonest validator and provider can
    /// collude to manipulate the result (as opposed to a malicious user and provider). That is, the user
    /// now trusts the validator honestly draw a random number. If you wish to avoid this trust assumption,
    /// call a variant of `requestV2` that accepts a `userRandomNumber` parameter.
    function requestV2()
        external
        payable
        returns (uint64 assignedSequenceNumber);

    /// @notice Request a random number using the default provider with specified gas limit
    /// @param gasLimit The gas limit for the callback function.
    /// @return assignedSequenceNumber A unique identifier for this request
    /// @dev The address calling this function should be a contract that inherits from the IEntropyConsumer interface.
    /// The `entropyCallback` method on that interface will receive a callback with the returned sequence number and
    /// the generated random number.
    ///
    /// `entropyCallback` will be run with the `gasLimit` provided to this function.
    /// The `gasLimit` will be rounded up to a multiple of 10k (e.g., 19000 -> 20000), and furthermore is lower bounded
    /// by the provider's configured default limit.
    ///
    /// This method will revert unless the caller provides a sufficient fee (at least `getFeeV2(gasLimit)`) as msg.value.
    /// Note that the fee can change over time. Callers of this method should explicitly compute `getFeeV2(gasLimit)`
    /// prior to each invocation (as opposed to hardcoding a value). Further note that excess value is *not* refunded to the caller.
    ///
    /// Note that this method uses an in-contract PRNG to generate the user's contribution to the random number.
    /// This approach modifies the security guarantees such that a dishonest validator and provider can
    /// collude to manipulate the result (as opposed to a malicious user and provider). That is, the user
    /// now trusts the validator honestly draw a random number. If you wish to avoid this trust assumption,
    /// call a variant of `requestV2` that accepts a `userRandomNumber` parameter.
    function requestV2(
        uint32 gasLimit
    ) external payable returns (uint64 assignedSequenceNumber);

    /// @notice Request a random number from a specific provider with specified gas limit
    /// @param provider The address of the provider to request from
    /// @param gasLimit The gas limit for the callback function
    /// @return assignedSequenceNumber A unique identifier for this request
    /// @dev The address calling this function should be a contract that inherits from the IEntropyConsumer interface.
    /// The `entropyCallback` method on that interface will receive a callback with the returned sequence number and
    /// the generated random number.
    ///
    /// `entropyCallback` will be run with the `gasLimit` provided to this function.
    /// The `gasLimit` will be rounded up to a multiple of 10k (e.g., 19000 -> 20000), and furthermore is lower bounded
    /// by the provider's configured default limit.
    ///
    /// This method will revert unless the caller provides a sufficient fee (at least `getFeeV2(provider, gasLimit)`) as msg.value.
    /// Note that provider fees can change over time. Callers of this method should explicitly compute `getFeeV2(provider, gasLimit)`
    /// prior to each invocation (as opposed to hardcoding a value). Further note that excess value is *not* refunded to the caller.
    ///
    /// Note that this method uses an in-contract PRNG to generate the user's contribution to the random number.
    /// This approach modifies the security guarantees such that a dishonest validator and provider can
    /// collude to manipulate the result (as opposed to a malicious user and provider). That is, the user
    /// now trusts the validator honestly draw a random number. If you wish to avoid this trust assumption,
    /// call a variant of `requestV2` that accepts a `userRandomNumber` parameter.
    function requestV2(
        address provider,
        uint32 gasLimit
    ) external payable returns (uint64 assignedSequenceNumber);

    /// @notice Request a random number from a specific provider with a user-provided random number and gas limit
    /// @param provider The address of the provider to request from
    /// @param userRandomNumber A random number provided by the user for additional entropy
    /// @param gasLimit The gas limit for the callback function. Pass 0 to get a sane default value -- see note below.
    /// @return assignedSequenceNumber A unique identifier for this request
    /// @dev The address calling this function should be a contract that inherits from the IEntropyConsumer interface.
    /// The `entropyCallback` method on that interface will receive a callback with the returned sequence number and
    /// the generated random number.
    ///
    /// `entropyCallback` will be run with the `gasLimit` provided to this function.
    /// The `gasLimit` will be rounded up to a multiple of 10k (e.g., 19000 -> 20000), and furthermore is lower bounded
    /// by the provider's configured default limit.
    ///
    /// This method will revert unless the caller provides a sufficient fee (at least `getFeeV2(provider, gasLimit)`) as msg.value.
    /// Note that provider fees can change over time. Callers of this method should explicitly compute `getFeeV2(provider, gasLimit)`
    /// prior to each invocation (as opposed to hardcoding a value). Further note that excess value is *not* refunded to the caller.
    function requestV2(
        address provider,
        bytes32 userRandomNumber,
        uint32 gasLimit
    ) external payable returns (uint64 assignedSequenceNumber);

    /// @notice Get information about a specific entropy provider
    /// @param provider The address of the provider to query
    /// @return info The provider information including configuration, fees, and operational status
    /// @dev This method returns detailed information about a provider's configuration and capabilities.
    /// The returned ProviderInfo struct contains information such as the provider's fee structure and gas limits.
    function getProviderInfoV2(
        address provider
    ) external view returns (EntropyStructsV2.ProviderInfo memory info);

    /// @notice Get the address of the default entropy provider
    /// @return provider The address of the default provider
    /// @dev This method returns the address of the provider that will be used when no specific provider is specified
    /// in the requestV2 calls. The default provider can be used to get the base fee and gas limit information.
    function getDefaultProvider() external view returns (address provider);

    /// @notice Get information about a specific request
    /// @param provider The address of the provider that handled the request
    /// @param sequenceNumber The unique identifier of the request
    /// @return req The request information including status, random number, and other metadata
    /// @dev This method allows querying the state of a previously made request. The returned Request struct
    /// contains information about whether the request was fulfilled, the generated random number (if available),
    /// and other metadata about the request.
    function getRequestV2(
        address provider,
        uint64 sequenceNumber
    ) external view returns (EntropyStructsV2.Request memory req);

    /// @notice Get the fee charged by the default provider for the default gas limit
    /// @return feeAmount The fee amount in wei
    /// @dev This method returns the base fee required to make a request using the default provider with
    /// the default gas limit. This fee should be passed as msg.value when calling requestV2().
    /// The fee can change over time, so this method should be called before each request.
    function getFeeV2() external view returns (uint128 feeAmount);

    /// @notice Get the fee charged by the default provider for a specific gas limit
    /// @param gasLimit The gas limit for the callback function
    /// @return feeAmount The fee amount in wei
    /// @dev This method returns the fee required to make a request using the default provider with
    /// the specified gas limit. This fee should be passed as msg.value when calling requestV2(gasLimit).
    /// The fee can change over time, so this method should be called before each request.
    function getFeeV2(
        uint32 gasLimit
    ) external view returns (uint128 feeAmount);

    /// @notice Get the fee charged by a specific provider for a request with a given gas limit
    /// @param provider The address of the provider to query
    /// @param gasLimit The gas limit for the callback function
    /// @return feeAmount The fee amount in wei
    /// @dev This method returns the fee required to make a request using the specified provider with
    /// the given gas limit. This fee should be passed as msg.value when calling requestV2(provider, gasLimit)
    /// or requestV2(provider, userRandomNumber, gasLimit). The fee can change over time, so this method
    /// should be called before each request.
    function getFeeV2(
        address provider,
        uint32 gasLimit
    ) external view returns (uint128 feeAmount);
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.0) (access/Ownable.sol)

pragma solidity ^0.8.20;

import {Context} from "../utils/Context.sol";

/**
 * @dev Contract module which provides a basic access control mechanism, where
 * there is an account (an owner) that can be granted exclusive access to
 * specific functions.
 *
 * The initial owner is set to the address provided by the deployer. This can
 * later be changed with {transferOwnership}.
 *
 * This module is used through inheritance. It will make available the modifier
 * `onlyOwner`, which can be applied to your functions to restrict their use to
 * the owner.
 */
abstract contract Ownable is Context {
    address private _owner;

    /**
     * @dev The caller account is not authorized to perform an operation.
     */
    error OwnableUnauthorizedAccount(address account);

    /**
     * @dev The owner is not a valid owner account. (eg. `address(0)`)
     */
    error OwnableInvalidOwner(address owner);

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    /**
     * @dev Initializes the contract setting the address provided by the deployer as the initial owner.
     */
    constructor(address initialOwner) {
        if (initialOwner == address(0)) {
            revert OwnableInvalidOwner(address(0));
        }
        _transferOwnership(initialOwner);
    }

    /**
     * @dev Throws if called by any account other than the owner.
     */
    modifier onlyOwner() {
        _checkOwner();
        _;
    }

    /**
     * @dev Returns the address of the current owner.
     */
    function owner() public view virtual returns (address) {
        return _owner;
    }

    /**
     * @dev Throws if the sender is not the owner.
     */
    function _checkOwner() internal view virtual {
        if (owner() != _msgSender()) {
            revert OwnableUnauthorizedAccount(_msgSender());
        }
    }

    /**
     * @dev Leaves the contract without owner. It will not be possible to call
     * `onlyOwner` functions. Can only be called by the current owner.
     *
     * NOTE: Renouncing ownership will leave the contract without an owner,
     * thereby disabling any functionality that is only available to the owner.
     */
    function renounceOwnership() public virtual onlyOwner {
        _transferOwnership(address(0));
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    function transferOwnership(address newOwner) public virtual onlyOwner {
        if (newOwner == address(0)) {
            revert OwnableInvalidOwner(address(0));
        }
        _transferOwnership(newOwner);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Internal function without access restriction.
     */
    function _transferOwnership(address newOwner) internal virtual {
        address oldOwner = _owner;
        _owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
    }
}

//SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.28;

interface IScaledEntropyProvider {
    struct SetRequest {
        uint8 samples;
        uint256 minRange;
        uint256 maxRange;
        bool withReplacement;
    }
    function requestAndCallbackScaledRandomness(
        uint32 _gasLimit,
        SetRequest[] memory _requests,
        bytes4 _selector,
        bytes memory _context
    )
        external
        payable
        returns (uint64 requestId);
    function getFee(uint32 _gasLimit) external view returns (uint256);
}
// SPDX-License-Identifier: Apache 2
pragma solidity ^0.8.0;

abstract contract IEntropyConsumer {
    // This method is called by Entropy to provide the random number to the consumer.
    // It asserts that the msg.sender is the Entropy contract. It is not meant to be
    // override by the consumer.
    function _entropyCallback(
        uint64 sequence,
        address provider,
        bytes32 randomNumber
    ) external {
        address entropy = getEntropy();
        require(entropy != address(0), "Entropy address not set");
        require(msg.sender == entropy, "Only Entropy can call this function");

        entropyCallback(sequence, provider, randomNumber);
    }

    // getEntropy returns Entropy contract address. The method is being used to check that the
    // callback is indeed from Entropy contract. The consumer is expected to implement this method.
    // Entropy address can be found here - https://docs.pyth.network/entropy/contract-addresses
    function getEntropy() internal view virtual returns (address);

    // This method is expected to be implemented by the consumer to handle the random number.
    // It will be called by _entropyCallback after _entropyCallback ensures that the call is
    // indeed from Entropy contract.
    function entropyCallback(
        uint64 sequence,
        address provider,
        bytes32 randomNumber
    ) internal virtual;
}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "./EntropyStructs.sol";

/**
 * @title EntropyEventsV2
 * @notice Interface defining events for the Entropy V2 system, which handles random number generation
 * and provider management on Ethereum.
 * @dev This interface is used to emit events that track the lifecycle of random number requests,
 * provider registrations, and system configurations.
 */
interface EntropyEventsV2 {
    /**
     * @notice Emitted when a new provider registers with the Entropy system
     * @param provider The address of the registered provider
     * @param extraArgs A field for extra data for forward compatibility.
     */
    event Registered(address indexed provider, bytes extraArgs);

    /**
     * @notice Emitted when a user requests a random number from a provider
     * @param provider The address of the provider handling the request
     * @param caller The address of the user requesting the random number
     * @param sequenceNumber A unique identifier for this request
     * @param userContribution The user's contribution to the random number
     * @param gasLimit The gas limit for the callback.
     * @param extraArgs A field for extra data for forward compatibility.
     */
    event Requested(
        address indexed provider,
        address indexed caller,
        uint64 indexed sequenceNumber,
        bytes32 userContribution,
        uint32 gasLimit,
        bytes extraArgs
    );

    /**
     * @notice Emitted when a provider reveals the generated random number
     * @param provider The address of the provider that generated the random number
     * @param caller The address of the user who requested the random number (and who receives a callback)
     * @param sequenceNumber The unique identifier of the request
     * @param randomNumber The generated random number
     * @param userContribution The user's contribution to the random number
     * @param providerContribution The provider's contribution to the random number
     * @param callbackFailed Whether the callback to the caller failed
     * @param callbackReturnValue Return value from the callback. If the callback failed, this field contains
     * the error code and any additional returned data. Note that "" often indicates an out-of-gas error.
     * If the callback returns more than 256 bytes, only the first 256 bytes of the callback return value are included.
     * @param callbackGasUsed How much gas the callback used.
     * @param extraArgs A field for extra data for forward compatibility.
     */
    event Revealed(
        address indexed provider,
        address indexed caller,
        uint64 indexed sequenceNumber,
        bytes32 randomNumber,
        bytes32 userContribution,
        bytes32 providerContribution,
        bool callbackFailed,
        bytes callbackReturnValue,
        uint32 callbackGasUsed,
        bytes extraArgs
    );

    /**
     * @notice Emitted when a provider updates their fee
     * @param provider The address of the provider updating their fee
     * @param oldFee The previous fee amount
     * @param newFee The new fee amount
     * @param extraArgs A field for extra data for forward compatibility.
     */
    event ProviderFeeUpdated(
        address indexed provider,
        uint128 oldFee,
        uint128 newFee,
        bytes extraArgs
    );

    /**
     * @notice Emitted when a provider updates their default gas limit
     * @param provider The address of the provider updating their gas limit
     * @param oldDefaultGasLimit The previous default gas limit
     * @param newDefaultGasLimit The new default gas limit
     * @param extraArgs A field for extra data for forward compatibility.
     */
    event ProviderDefaultGasLimitUpdated(
        address indexed provider,
        uint32 oldDefaultGasLimit,
        uint32 newDefaultGasLimit,
        bytes extraArgs
    );

    /**
     * @notice Emitted when a provider updates their URI
     * @param provider The address of the provider updating their URI
     * @param oldUri The previous URI
     * @param newUri The new URI
     * @param extraArgs A field for extra data for forward compatibility.
     */
    event ProviderUriUpdated(
        address indexed provider,
        bytes oldUri,
        bytes newUri,
        bytes extraArgs
    );

    /**
     * @notice Emitted when a provider updates their fee manager address
     * @param provider The address of the provider updating their fee manager
     * @param oldFeeManager The previous fee manager address
     * @param newFeeManager The new fee manager address
     * @param extraArgs A field for extra data for forward compatibility.
     */
    event ProviderFeeManagerUpdated(
        address indexed provider,
        address oldFeeManager,
        address newFeeManager,
        bytes extraArgs
    );

    /**
     * @notice Emitted when a provider updates their maximum number of hashes that can be advanced
     * @param provider The address of the provider updating their max hashes
     * @param oldMaxNumHashes The previous maximum number of hashes
     * @param newMaxNumHashes The new maximum number of hashes
     * @param extraArgs A field for extra data for forward compatibility.
     */
    event ProviderMaxNumHashesAdvanced(
        address indexed provider,
        uint32 oldMaxNumHashes,
        uint32 newMaxNumHashes,
        bytes extraArgs
    );

    /**
     * @notice Emitted when a provider withdraws their accumulated fees
     * @param provider The address of the provider withdrawing fees
     * @param recipient The address receiving the withdrawn fees
     * @param withdrawnAmount The amount of fees withdrawn
     * @param extraArgs A field for extra data for forward compatibility.
     */
    event Withdrawal(
        address indexed provider,
        address indexed recipient,
        uint128 withdrawnAmount,
        bytes extraArgs
    );
}

// SPDX-License-Identifier: Apache 2

pragma solidity ^0.8.0;

contract EntropyStructsV2 {
    struct ProviderInfo {
        uint128 feeInWei;
        uint128 accruedFeesInWei;
        // The commitment that the provider posted to the blockchain, and the sequence number
        // where they committed to this. This value is not advanced after the provider commits,
        // and instead is stored to help providers track where they are in the hash chain.
        bytes32 originalCommitment;
        uint64 originalCommitmentSequenceNumber;
        // Metadata for the current commitment. Providers may optionally use this field to help
        // manage rotations (i.e., to pick the sequence number from the correct hash chain).
        bytes commitmentMetadata;
        // Optional URI where clients can retrieve revelations for the provider.
        // Client SDKs can use this field to automatically determine how to retrieve random values for each provider.
        // TODO: specify the API that must be implemented at this URI
        bytes uri;
        // The first sequence number that is *not* included in the current commitment (i.e., an exclusive end index).
        // The contract maintains the invariant that sequenceNumber <= endSequenceNumber.
        // If sequenceNumber == endSequenceNumber, the provider must rotate their commitment to add additional random values.
        uint64 endSequenceNumber;
        // The sequence number that will be assigned to the next inbound user request.
        uint64 sequenceNumber;
        // The current commitment represents an index/value in the provider's hash chain.
        // These values are used to verify requests for future sequence numbers. Note that
        // currentCommitmentSequenceNumber < sequenceNumber.
        //
        // The currentCommitment advances forward through the provider's hash chain as values
        // are revealed on-chain.
        bytes32 currentCommitment;
        uint64 currentCommitmentSequenceNumber;
        // An address that is authorized to set / withdraw fees on behalf of this provider.
        address feeManager;
        // Maximum number of hashes to record in a request. This should be set according to the maximum gas limit
        // the provider supports for callbacks.
        uint32 maxNumHashes;
        // Default gas limit to use for callbacks.
        uint32 defaultGasLimit;
    }

    struct Request {
        // Storage slot 1 //
        address provider;
        uint64 sequenceNumber;
        // The number of hashes required to verify the provider revelation.
        uint32 numHashes;
        // Storage slot 2 //
        // The commitment is keccak256(userCommitment, providerCommitment). Storing the hash instead of both saves 20k gas by
        // eliminating 1 store.
        bytes32 commitment;
        // Storage slot 3 //
        // The number of the block where this request was created.
        // Note that we're using a uint64 such that we have an additional space for an address and other fields in
        // this storage slot. Although block.number returns a uint256, 64 bits should be plenty to index all of the
        // blocks ever generated.
        uint64 blockNumber;
        // The address that requested this random number.
        address requester;
        // If true, incorporate the blockhash of blockNumber into the generated random value.
        bool useBlockhash;
        // Status flag for requests with callbacks. See EntropyConstants for the possible values of this flag.
        uint8 callbackStatus;
        // The gasLimit in units of 10k gas. (i.e., 2 = 20k gas). We're using units of 10k in order to fit this
        // field into the remaining 2 bytes of this storage slot. The dynamic range here is 10k - 655M, which should
        // cover all real-world use cases.
        uint16 gasLimit10k;
    }
}

// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "./EntropyStructs.sol";

// Deprecated -- these events are still emitted, but the lack of indexing
// makes them hard to use.
interface EntropyEvents {
    event Registered(EntropyStructs.ProviderInfo provider);

    event Requested(EntropyStructs.Request request);
    event RequestedWithCallback(
        address indexed provider,
        address indexed requestor,
        uint64 indexed sequenceNumber,
        bytes32 userRandomNumber,
        EntropyStructs.Request request
    );

    event Revealed(
        EntropyStructs.Request request,
        bytes32 userRevelation,
        bytes32 providerRevelation,
        bytes32 blockHash,
        bytes32 randomNumber
    );
    event RevealedWithCallback(
        EntropyStructs.Request request,
        bytes32 userRandomNumber,
        bytes32 providerRevelation,
        bytes32 randomNumber
    );

    event CallbackFailed(
        address indexed provider,
        address indexed requestor,
        uint64 indexed sequenceNumber,
        bytes32 userRandomNumber,
        bytes32 providerRevelation,
        bytes32 randomNumber,
        bytes errorCode
    );

    event ProviderFeeUpdated(address provider, uint128 oldFee, uint128 newFee);

    event ProviderDefaultGasLimitUpdated(
        address indexed provider,
        uint32 oldDefaultGasLimit,
        uint32 newDefaultGasLimit
    );

    event ProviderUriUpdated(address provider, bytes oldUri, bytes newUri);

    event ProviderFeeManagerUpdated(
        address provider,
        address oldFeeManager,
        address newFeeManager
    );
    event ProviderMaxNumHashesAdvanced(
        address provider,
        uint32 oldMaxNumHashes,
        uint32 newMaxNumHashes
    );

    event Withdrawal(
        address provider,
        address recipient,
        uint128 withdrawnAmount
    );
}

// SPDX-License-Identifier: Apache 2

pragma solidity ^0.8.0;

// This contract holds old versions of the Entropy structs that are no longer used for contract storage.
// However, they are still used in EntropyEvents to maintain the public interface of prior versions of
// the Entropy contract.
//
// See EntropyStructsV2 for the struct definitions currently in use.
contract EntropyStructs {
    struct ProviderInfo {
        uint128 feeInWei;
        uint128 accruedFeesInWei;
        // The commitment that the provider posted to the blockchain, and the sequence number
        // where they committed to this. This value is not advanced after the provider commits,
        // and instead is stored to help providers track where they are in the hash chain.
        bytes32 originalCommitment;
        uint64 originalCommitmentSequenceNumber;
        // Metadata for the current commitment. Providers may optionally use this field to help
        // manage rotations (i.e., to pick the sequence number from the correct hash chain).
        bytes commitmentMetadata;
        // Optional URI where clients can retrieve revelations for the provider.
        // Client SDKs can use this field to automatically determine how to retrieve random values for each provider.
        // TODO: specify the API that must be implemented at this URI
        bytes uri;
        // The first sequence number that is *not* included in the current commitment (i.e., an exclusive end index).
        // The contract maintains the invariant that sequenceNumber <= endSequenceNumber.
        // If sequenceNumber == endSequenceNumber, the provider must rotate their commitment to add additional random values.
        uint64 endSequenceNumber;
        // The sequence number that will be assigned to the next inbound user request.
        uint64 sequenceNumber;
        // The current commitment represents an index/value in the provider's hash chain.
        // These values are used to verify requests for future sequence numbers. Note that
        // currentCommitmentSequenceNumber < sequenceNumber.
        //
        // The currentCommitment advances forward through the provider's hash chain as values
        // are revealed on-chain.
        bytes32 currentCommitment;
        uint64 currentCommitmentSequenceNumber;
        // An address that is authorized to set / withdraw fees on behalf of this provider.
        address feeManager;
        // Maximum number of hashes to record in a request. This should be set according to the maximum gas limit
        // the provider supports for callbacks.
        uint32 maxNumHashes;
    }

    struct Request {
        // Storage slot 1 //
        address provider;
        uint64 sequenceNumber;
        // The number of hashes required to verify the provider revelation.
        uint32 numHashes;
        // Storage slot 2 //
        // The commitment is keccak256(userCommitment, providerCommitment). Storing the hash instead of both saves 20k gas by
        // eliminating 1 store.
        bytes32 commitment;
        // Storage slot 3 //
        // The number of the block where this request was created.
        // Note that we're using a uint64 such that we have an additional space for an address and other fields in
        // this storage slot. Although block.number returns a uint256, 64 bits should be plenty to index all of the
        // blocks ever generated.
        uint64 blockNumber;
        // The address that requested this random number.
        address requester;
        // If true, incorporate the blockhash of blockNumber into the generated random value.
        bool useBlockhash;
        // True if this is a request that expects a callback.
        bool isRequestWithCallback;
    }
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

