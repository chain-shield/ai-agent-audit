
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "../interfaces/ISwapper.sol";

/**
 * @title HybrSwapper
 * @notice Default implementation of ISwapper for swapping tokens to HYBR
 * @dev Can be replaced with other implementations for different swap strategies
 */
contract HybrSwapper is ISwapper, Ownable, ReentrancyGuard {
    using SafeERC20 for IERC20;

    // Core addresses
    address public immutable override HYBR;

    // Aggregator whitelist
    mapping(address => bool) private whitelistedAggregators;

    // Errors
    error AggregatorNotWhitelisted(address aggregator);
    error AggregatorReverted(bytes returnData);
    error AmountOutTooLow(uint256 actual, uint256 minimum);
    error ForbiddenToken(address token);

    /**
     * @notice Constructor
     * @param _hybr HYBR token address
     */
    constructor(address _hybr) {
        require(_hybr != address(0), "Invalid HYBR");

        HYBR = _hybr;
    }

 

    /**
     * @notice Swap tokens to HYBR via aggregator with slippage protection
     * @param params Swap parameters including aggregator and calldata
     * @return hybrReceived Amount of HYBR received
     */
    function swapToHYBR(SwapParams calldata params)
        external
        override
        nonReentrant
        returns (uint256 hybrReceived)
    {
        // Validate aggregator is whitelisted
        if (!whitelistedAggregators[params.aggregator]) {
            revert AggregatorNotWhitelisted(params.aggregator);
        }

        // Prevent swapping HYBR itself
        if (params.tokenIn == HYBR) {
            revert ForbiddenToken(HYBR);
        }

        // Record HYBR balance before swap
        uint256 hybrBalanceBefore = IERC20(HYBR).balanceOf(address(this));

        // Transfer tokens from caller to this contract
        IERC20(params.tokenIn).safeTransferFrom(
            msg.sender,
            address(this),
            params.amountIn
        );

        // Approve aggregator to spend input token
        IERC20(params.tokenIn).safeApprove(params.aggregator, params.amountIn);

        // Execute swap via aggregator
        (bool success, bytes memory returnData) = params.aggregator.call(params.callData);
        if (!success) {
            revert AggregatorReverted(returnData);
        }

        // Reset approval for safety
        IERC20(params.tokenIn).safeApprove(params.aggregator, 0);

        // Calculate HYBR received
        uint256 hybrBalanceAfter = IERC20(HYBR).balanceOf(address(this));
        hybrReceived = hybrBalanceAfter - hybrBalanceBefore;

        // Check slippage protection
        if (hybrReceived < params.minAmountOut) {
            revert AmountOutTooLow(hybrReceived, params.minAmountOut);
        }

        // Transfer HYBR back to caller (GovernanceHYBR)
        IERC20(HYBR).safeTransfer(msg.sender, hybrReceived);

        emit SwappedToHYBR(msg.sender, params.tokenIn, params.amountIn, hybrReceived);

        return hybrReceived;
    }

    /**
     * @notice Set aggregator whitelist status
     * @param aggregator Aggregator contract address
     * @param whitelisted Whether to whitelist or not
     */
    function setAggregatorWhitelist(address aggregator, bool whitelisted)
        external
        override
        onlyOwner
    {
        whitelistedAggregators[aggregator] = whitelisted;
        emit AggregatorWhitelisted(aggregator, whitelisted);
    }

    /**
     * @notice Check if an aggregator is whitelisted
     * @param aggregator Aggregator address to check
     * @return Whether the aggregator is whitelisted
     */
    function isWhitelistedAggregator(address aggregator)
        external
        view
        override
        returns (bool)
    {
        return whitelistedAggregators[aggregator];
    }



 

    /**
     * @notice Emergency withdraw stuck tokens
     * @param token Token address to withdraw
     * @param to Recipient address
     * @param amount Amount to withdraw
     */
    function emergencyWithdraw(
        address token,
        address to,
        uint256 amount
    ) external override onlyOwner {
        require(to != address(0), "Invalid recipient");

        if (token == address(0)) {
            // Withdraw native token
            (bool success, ) = to.call{value: amount}("");
            require(success, "Transfer failed");
        } else {
            // Withdraw ERC20 token
            IERC20(token).safeTransfer(to, amount);
        }
    }

    /**
     * @notice Receive native token
     */
    receive() external payable {}
}
END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

/**
 * @title ISwapper
 * @notice Interface for modular swap functionality
 * @dev Allows GovernanceHYBR to use different swap implementations
 */
interface ISwapper {
    /**
     * @notice Swap parameters for aggregator calls
     */
    struct SwapParams {
        address aggregator;     // Aggregator contract address
        address tokenIn;        // Input token address
        uint256 amountIn;       // Input token amount
        uint256 minAmountOut;   // Minimum HYBR expected
        bytes callData;         // Aggregator call data
    }

    /**
     * @notice Event emitted when aggregator whitelist is updated
     */
    event AggregatorWhitelisted(address indexed aggregator, bool whitelisted);

    /**
     * @notice Event emitted when a swap is executed
     */
    event SwappedToHYBR(
        address indexed executor,
        address indexed tokenIn,
        uint256 amountIn,
        uint256 hybrOut
    );

    /**
     * @notice Event emitted when authorized caller is updated
     */
    event AuthorizedCallerUpdated(address indexed oldCaller, address indexed newCaller);

    /**
     * @notice Swap tokens to HYBR via aggregator with slippage protection
     * @param params Swap parameters including aggregator and calldata
     * @return hybrReceived Amount of HYBR received
     */
    function swapToHYBR(SwapParams calldata params) external returns (uint256 hybrReceived);

    /**
     * @notice Set aggregator whitelist status
     * @param aggregator Aggregator contract address
     * @param whitelisted Whether to whitelist or not
     */
    function setAggregatorWhitelist(address aggregator, bool whitelisted) external;

    /**
     * @notice Check if an aggregator is whitelisted
     * @param aggregator Aggregator address to check
     * @return whitelisted Whether the aggregator is whitelisted
     */
    function isWhitelistedAggregator(address aggregator) external view returns (bool whitelisted);

    /**
     * @notice Get the HYBR token address
     * @return hybr The HYBR token address
     */
    function HYBR() external view returns (address hybr);




    /**
     * @notice Emergency withdraw stuck tokens
     * @param token Token address to withdraw
     * @param to Recipient address
     * @param amount Amount to withdraw
     */
    function emergencyWithdraw(address token, address to, uint256 amount) external;
}

END OF SUPPORTING CONTRACTS AND INTERFACES
