
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


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "forge-std/Script.sol";
import "./BaseDeployScript.sol";
import "../contracts/GovernanceHYBR.sol";
import "../contracts/swapper/HybrSwapper.sol";
import "../contracts/interfaces/ISwapper.sol";

/**
 * @title DeploySwapper
 * @notice Deployment script for the modular swapper architecture
 * @dev Demonstrates how to deploy and configure the swapper plugin system
 */
contract DeploySwapper is BaseDeployScript {
    // Contracts to deploy
    HybrSwapper public hybrSwapper;

    // Existing contracts (to be loaded from previous deployments)
    GrowthHYBR public growthHYBR;
    address public hybr;

    // Known aggregators to whitelist
    address constant PARASWAP = 0xDEF171Fe48CF0115B1d80b88dc8eAB59176FEe57;
    address constant ONE_INCH = 0x1111111254EEB25477B68fb85Ed929f73A960582;
    address constant OKX = 0x0000000000000000000000000000000000000000; // Replace with actual address

    function run() external {
        uint256 deployerKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.rememberKey(deployerKey);

        // Load existing contracts
        _loadExistingContracts();

        console.log("=== Deploying Modular Swapper Architecture ===");
        console.log("Deployer:", deployer);
        console.log("HYBR:", hybr);
        console.log("GrowthHYBR:", address(growthHYBR));
        console.log("");

        vm.startBroadcast(deployer);

        // Step 1: Deploy HybrSwapper
        _deploySwapper();

        // Step 2: Configure swapper with whitelisted aggregators
        _configureSwapper();

        // Step 3: Connect swapper to GrowthHYBR
        _connectSwapper();

        // Step 4: Verify configuration
        _verifyConfiguration();

        vm.stopBroadcast();

        // Save deployment addresses
        _saveDeployment();

        console.log("");
        console.log("=== Deployment Complete ===");
        console.log("HybrSwapper:", address(hybrSwapper));
    }

    function _loadExistingContracts() internal {
        // Load HYBR token address
        string memory tokenPath = getInputPath("Deploy2_TokenSystem");
        string memory tokenJson = vm.readFile(tokenPath);
        hybr = abi.decode(vm.parseJson(tokenJson, ".HYBR"), (address));

            // Load GrowthHYBR address
        string memory governancePath = getInputPath("Deploy4_GrowthHYBR");
        string memory governanceJson = vm.readFile(governancePath);
        address growthHYBRAddress = abi.decode(vm.parseJson(governanceJson, ".GrowthHYBR"), (address));
        growthHYBR = GrowthHYBR(growthHYBRAddress);
    }

    function _deploySwapper() internal {
        console.log("=== Step 1: Deploying HybrSwapper ===");

        // Deploy with growthHYBR as the authorized caller
        hybrSwapper = new HybrSwapper(
            hybr
        );

        console.log("HybrSwapper deployed:", address(hybrSwapper));
        console.log("Authorized caller:", address(growthHYBR));
    }

    function _configureSwapper() internal {
        console.log("");
        console.log("=== Step 2: Configuring Swapper ===");

        // Whitelist aggregators
        console.log("Whitelisting Paraswap...");
        hybrSwapper.setAggregatorWhitelist(PARASWAP, true);

        console.log("Whitelisting 1inch...");
        hybrSwapper.setAggregatorWhitelist(ONE_INCH, true);

        if (OKX != address(0)) {
            console.log("Whitelisting OKX...");
            hybrSwapper.setAggregatorWhitelist(OKX, true);
        }

        console.log("Aggregators whitelisted");
    }

    function _connectSwapper() internal {
        console.log("");
        console.log("=== Step 3: Connecting Swapper to GrowthHYBR ===");

        // Set swapper in growthHYBR
        growthHYBR.setSwapper(address(hybrSwapper));

        console.log("Swapper connected to GrowthHYBR");
    }

    function _verifyConfiguration() internal view {
        console.log("");
        console.log("=== Step 4: Verifying Configuration ===");

        // Verify swapper is set
        require(address(growthHYBR.swapper()) == address(hybrSwapper), "Swapper not set correctly");
        console.log("Swapper correctly set in growthHYBR");

      

        // Verify aggregators are whitelisted
        require(hybrSwapper.isWhitelistedAggregator(PARASWAP), "Paraswap not whitelisted");
        console.log("Paraswap whitelisted");

        require(hybrSwapper.isWhitelistedAggregator(ONE_INCH), "1inch not whitelisted");
        console.log("1inch whitelisted");

        console.log("");
        console.log("All verifications passed!");
    }

    function _saveDeployment() internal {
        string memory json = "";
        json = vm.serializeAddress("deploy", "HybrSwapper", address(hybrSwapper));
        json = vm.serializeAddress("deploy", "HYBR", hybr);
        json = vm.serializeAddress("deploy", "GrowthHYBR", address(growthHYBR));
        json = vm.serializeAddress("deploy", "Paraswap", PARASWAP);
        json = vm.serializeAddress("deploy", "OneInch", ONE_INCH);

        string memory finalJson = vm.serializeString("deploy", "timestamp", vm.toString(block.timestamp));

        // Write to output file using getOutputPath from BaseDeployScript
        string memory outputPath = getOutputPath("Deploy_Swapper");
        vm.writeFile(outputPath, finalJson);

        console.log("Deployment saved to:", outputPath);
    }
}

/**
 * @title UpgradeSwapper
 * @notice Script to upgrade or replace the swapper module
 * @dev Shows how to swap out one swapper implementation for another
 */
contract UpgradeSwapper is BaseDeployScript {
    GrowthHYBR public growthHYBR;
    HybrSwapper public newSwapper;
    address public hybr;

    function run() external {
        uint256 deployerKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.rememberKey(deployerKey);

        // Load existing contracts
        string memory governancePath = getInputPath("Deploy4_growthHYBR");
        string memory governanceJson = vm.readFile(governancePath);
        address growthHYBRAddress = abi.decode(vm.parseJson(governanceJson, ".GrowthHYBR"), (address));
        growthHYBR = GrowthHYBR(growthHYBRAddress);

        string memory tokenPath = getInputPath("Deploy2_TokenSystem");
        string memory tokenJson = vm.readFile(tokenPath);
        hybr = abi.decode(vm.parseJson(tokenJson, ".HYBR"), (address));

        console.log("=== Upgrading Swapper Module ===");
        console.log("Current swapper:", address(growthHYBR.swapper()));

        vm.startBroadcast(deployer);

        // Deploy new swapper with updated logic
        newSwapper = new HybrSwapper(hybr);
        console.log("New swapper deployed:", address(newSwapper));

        // Configure new swapper (copy whitelist from old or set new)
        // This example sets new whitelist
        newSwapper.setAggregatorWhitelist(0xDEF171Fe48CF0115B1d80b88dc8eAB59176FEe57, true); // Paraswap
        newSwapper.setAggregatorWhitelist(0x1111111254EEB25477B68fb85Ed929f73A960582, true); // 1inch

        // Update growthHYBR to use new swapper
        growthHYBR.setSwapper(address(newSwapper));

        vm.stopBroadcast();

        console.log("Swapper upgrade complete!");
        console.log("New swapper:", address(growthHYBR.swapper()));
    }
}
