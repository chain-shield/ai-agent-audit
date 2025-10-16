
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "./IMarket.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

/**
 * @title MarketContract
 * @notice Upgradeable version of MarketContract - manages market metadata and configuration with restricted write access
 * @dev Only MarketController can modify market data, implements UUPS upgradeability
 */
contract MarketContract is IMarket, Initializable, UUPSUpgradeable, OwnableUpgradeable {
    /// @notice Maps question ID to number of possible outcomes
    mapping(bytes32 => uint256) private outcomeCount;

    /// @notice Maps question ID to current epoch number (only used for manual epoch markets)
    mapping(bytes32 => uint256) private currentEpoch;

    /// @notice Maps question ID to market creation status
    mapping(bytes32 => bool) private marketExists;

    /// @notice Maps question ID to resolution timestamp
    mapping(bytes32 => uint256) private resolutionTime;

    /// @notice Maps question ID to market creation timestamp
    mapping(bytes32 => uint256) private creationTime;

    /// @notice Address of authorized MarketController contract
    address public marketController;

    /// @notice Maps question ID to epoch duration in seconds (0 = manual epochs)
    mapping(bytes32 => uint256) private epochDuration;

    /// @notice Maps question ID to the timestamp when epoch 1 started
    mapping(bytes32 => uint256) private epochStartTime;

    /// @dev Gap for future storage variables
    uint256[42] private __gap;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the upgradeable contract
     * @param _initialOwner Initial owner of the contract
     */
    function initialize(address _initialOwner) public initializer {
        __Ownable_init(_initialOwner);
        __UUPSUpgradeable_init();
    }

    /// @dev Restricts function access only to authorized MarketController
    modifier onlyMarketController() {
        require(msg.sender == marketController, "Only MarketController can call this function");
        _;
    }

    /**
     * @notice Authorizes contract upgrades
     * @param newImplementation Address of the new implementation
     * @dev Only callable by contract owner
     */
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    /**
     * @notice Sets the authorized MarketController address
     * @param _marketController Address of MarketController contract
     * @dev Only callable by contract owner, establishes access control
     */
    function setMarketController(address _marketController) external onlyOwner {
        require(_marketController != address(0), "Invalid MarketController address");
        marketController = _marketController;
    }

    /**
     * @notice Creates new market with specified outcomes and resolution time
     * @param questionId Unique market question identifier
     * @param _outcomeCount Number of possible outcomes for this market
     * @param _resolutionTime Timestamp when market should resolve (0 for manual resolution)
     * @param _epochDuration Duration of each epoch in seconds (0 for manual epoch advancement)
     * @dev Only MarketController can create markets
     * @dev If epochDuration > 0, market uses time-based automatic epoch rolling
     * @dev If epochDuration = 0, market uses manual epoch advancement (legacy behavior)
     */
    function createMarket(
        bytes32 questionId,
        uint256 _outcomeCount,
        uint256 _resolutionTime,
        uint256 _epochDuration
    ) external onlyMarketController {
        require(questionId != bytes32(0), "Invalid question ID");
        require(_outcomeCount > 1, "Must have at least 2 outcomes");
        require(_outcomeCount <= 256, "Maximum 256 outcomes supported");
        require(!marketExists[questionId], "Market already exists");

        // If resolution time is set, it must be in the future
        if (_resolutionTime > 0) {
            require(_resolutionTime > block.timestamp, "Resolution time must be in the future");
        }

        outcomeCount[questionId] = _outcomeCount;
        currentEpoch[questionId] = 1; // Initial epoch for manual mode
        marketExists[questionId] = true;
        resolutionTime[questionId] = _resolutionTime;
        creationTime[questionId] = block.timestamp;
        epochDuration[questionId] = _epochDuration;
        
        if (_epochDuration > 0) {
            epochStartTime[questionId] = block.timestamp;
        }

        emit MarketCreated(questionId, _outcomeCount, 1, _resolutionTime);
    }

    /**
     * @notice Updates resolution time for existing market
     * @param questionId Market question identifier
     * @param _resolutionTime New resolution timestamp (0 for manual resolution)
     * @dev Only MarketController can update resolution time
     */
    function updateResolutionTime(bytes32 questionId, uint256 _resolutionTime) external onlyMarketController {
        require(marketExists[questionId], "Market does not exist");
        // If setting a new resolution time, it must be in the future
        if (_resolutionTime > 0) {
            require(_resolutionTime > block.timestamp, "Resolution time must be in the future");
        }

        uint256 oldResolutionTime = resolutionTime[questionId];
        resolutionTime[questionId] = _resolutionTime;

        emit ResolutionTimeUpdated(questionId, oldResolutionTime, _resolutionTime);
    }

    /**
     * @notice Advances market to next epoch (manual mode only)
     * @param questionId Market question identifier
     * @dev Only MarketController can advance epochs
     * @dev For time-based markets (epochDuration > 0), this function does nothing
     */
    function advanceEpoch(bytes32 questionId) external onlyMarketController {
        require(marketExists[questionId], "Market does not exist");
        require(epochDuration[questionId] == 0, "Cannot manually advance time-based epochs");

        uint256 previousEpoch = currentEpoch[questionId];
        currentEpoch[questionId] = previousEpoch + 1;

        emit EpochAdvanced(questionId, previousEpoch, currentEpoch[questionId]);
    }

    /**
     * @notice Checks if market is currently open for betting
     * @param questionId Market question identifier
     * @return True if market is open for betting
     * @dev Market is open if it exists and resolution time hasn't passed (or is 0 for manual)
     */
    function isMarketOpen(bytes32 questionId) external view returns (bool) {
        if (!marketExists[questionId]) {
            return false;
        }

        uint256 resolveTime = resolutionTime[questionId];
        // If resolution time is 0, market is manually resolved (always open until manually closed)
        // If resolution time is set, market is open until that time
        return resolveTime == 0 || block.timestamp < resolveTime;
    }

    /**
     * @notice Checks if market is ready for resolution
     * @param questionId Market question identifier
     * @return True if market can be resolved
     * @dev Market is ready for resolution if resolution time has passed (or is manual)
     */
    function isMarketReadyForResolution(bytes32 questionId) external view returns (bool) {
        if (!marketExists[questionId]) {
            return false;
        }

        uint256 resolveTime = resolutionTime[questionId];
        // If resolution time is 0, market can be resolved manually anytime
        // If resolution time is set, market can only be resolved after that time
        return resolveTime == 0 || block.timestamp >= resolveTime;
    }

    /**
     * @notice Gets resolution timestamp for market
     * @param questionId Market question identifier
     * @return Resolution timestamp (0 if manual resolution)
     */
    function getResolutionTime(bytes32 questionId) external view returns (uint256) {
        return resolutionTime[questionId];
    }

    /**
     * @notice Gets creation timestamp for market
     * @param questionId Market question identifier
     * @return Creation timestamp
     */
    function getCreationTime(bytes32 questionId) external view returns (uint256) {
        return creationTime[questionId];
    }

    /**
     * @notice Gets number of possible outcomes for market
     * @param questionId Market question identifier
     * @return Number of outcomes
     */
    function getOutcomeCount(bytes32 questionId) external view returns (uint256) {
        return outcomeCount[questionId];
    }

    /**
     * @notice Gets current epoch for market (time-based or manual)
     * @param questionId Market question identifier
     * @return Current epoch number
     * @dev For time-based markets (epochDuration > 0), calculates current epoch from time elapsed
     * @dev For manual markets (epochDuration = 0), returns stored epoch value
     */
    function getCurrentEpoch(bytes32 questionId) public view returns (uint256) {
        if (!marketExists[questionId]) {
            return 0;
        }

        uint256 duration = epochDuration[questionId];
        
        // Manual epoch mode (legacy behavior)
        if (duration == 0) {
            return currentEpoch[questionId];
        }

        // Time-based epoch mode (automatic rolling)
        uint256 elapsed = block.timestamp - epochStartTime[questionId];
        return 1 + (elapsed / duration);
    }

    /**
     * @notice Gets epoch duration for market
     * @param questionId Market question identifier
     * @return Epoch duration in seconds (0 if manual epochs)
     */
    function getEpochDuration(bytes32 questionId) external view returns (uint256) {
        return epochDuration[questionId];
    }

    /**
     * @notice Gets the timestamp when a specific epoch starts
     * @param questionId Market question identifier
     * @param epoch The epoch number to query
     * @return Timestamp when the epoch starts (0 if manual epochs)
     */
    function getEpochStartTime(bytes32 questionId, uint256 epoch) external view returns (uint256) {
        uint256 duration = epochDuration[questionId];
        if (duration == 0 || epoch == 0) {
            return 0;
        }
        return epochStartTime[questionId] + (duration * (epoch - 1));
    }

    /**
     * @notice Gets the timestamp when a specific epoch ends
     * @param questionId Market question identifier
     * @param epoch The epoch number to query
     * @return Timestamp when the epoch ends (0 if manual epochs)
     */
    function getEpochEndTime(bytes32 questionId, uint256 epoch) external view returns (uint256) {
        uint256 duration = epochDuration[questionId];
        if (duration == 0 || epoch == 0) {
            return 0;
        }
        return epochStartTime[questionId] + (duration * epoch);
    }

    /**
     * @notice Generates condition ID for market and epoch
     * @param oracle Oracle address
     * @param questionId Market question identifier
     * @param numberOfOutcomes Number of outcomes
     * @param epoch Specific epoch (0 for current)
     * @return Condition identifier
     * @dev Centralizes condition ID generation logic in market metadata layer
     */
    function getConditionId(address oracle, bytes32 questionId, uint256 numberOfOutcomes, uint256 epoch)
        external
        view
        returns (bytes32)
    {
        require(numberOfOutcomes <= 256, "Maximum 256 outcomes supported");
        uint256 targetEpoch = epoch == 0 ? getCurrentEpoch(questionId) : epoch;
        return keccak256(abi.encodePacked(oracle, questionId, numberOfOutcomes, targetEpoch));
    }

    /**
     * @notice Checks if market exists
     * @param questionId Market question identifier
     * @return True if market exists
     */
    function getMarketExists(bytes32 questionId) external view returns (bool) {
        return marketExists[questionId];
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

/**
 * @title IMarketContract
 * @notice Interface for market metadata and configuration management
 */
interface IMarket {
    /// @notice Emitted when new market is created
    event MarketCreated(bytes32 indexed questionId, uint256 outcomeCount, uint256 initialEpoch, uint256 resolutionTime);

    /// @notice Emitted when market epoch is advanced
    event EpochAdvanced(bytes32 indexed questionId, uint256 previousEpoch, uint256 newEpoch);

    /// @notice Emitted when market resolution time is updated
    event ResolutionTimeUpdated(bytes32 indexed questionId, uint256 oldResolutionTime, uint256 newResolutionTime);

    /**
     * @notice Creates new market with specified outcomes and resolution time
     * @param questionId Unique market question identifier
     * @param outcomeCount Number of possible outcomes for this market
     * @param resolutionTime Timestamp when market should resolve (0 for manual resolution)
     * @param epochDuration Duration of each epoch in seconds (0 for manual epoch advancement)
     */
    function createMarket(bytes32 questionId, uint256 outcomeCount, uint256 resolutionTime, uint256 epochDuration)
        external;

    /**
     * @notice Updates resolution time for existing market
     * @param questionId Market question identifier
     * @param resolutionTime New resolution timestamp (0 for manual resolution)
     */
    function updateResolutionTime(bytes32 questionId, uint256 resolutionTime) external;

    /**
     * @notice Advances market to next epoch (manual mode only)
     * @param questionId Market question identifier
     */
    function advanceEpoch(bytes32 questionId) external;

    /**
     * @notice Checks if market is currently open for betting
     * @param questionId Market question identifier
     * @return True if market is open for betting
     */
    function isMarketOpen(bytes32 questionId) external view returns (bool);

    /**
     * @notice Checks if market is ready for resolution
     * @param questionId Market question identifier
     * @return True if market can be resolved
     */
    function isMarketReadyForResolution(bytes32 questionId) external view returns (bool);

    /**
     * @notice Gets resolution timestamp for market
     * @param questionId Market question identifier
     * @return Resolution timestamp (0 if manual resolution)
     */
    function getResolutionTime(bytes32 questionId) external view returns (uint256);

    /**
     * @notice Gets creation timestamp for market
     * @param questionId Market question identifier
     * @return Creation timestamp
     */
    function getCreationTime(bytes32 questionId) external view returns (uint256);

    /**
     * @notice Generates condition ID for market and epoch
     * @param oracle Oracle address
     * @param questionId Market question identifier
     * @param numberOfOutcomes Number of outcomes
     * @param epoch Specific epoch (0 for current)
     * @return Condition identifier
     */
    function getConditionId(address oracle, bytes32 questionId, uint256 numberOfOutcomes, uint256 epoch)
        external
        view
        returns (bytes32);

    /**
     * @notice Gets number of possible outcomes for market
     * @param questionId Market question identifier
     * @return Number of outcomes
     */
    function getOutcomeCount(bytes32 questionId) external view returns (uint256);

    /**
     * @notice Gets current epoch for market
     * @param questionId Market question identifier
     * @return Current epoch number
     */
    function getCurrentEpoch(bytes32 questionId) external view returns (uint256);

    /**
     * @notice Gets epoch duration for market
     * @param questionId Market question identifier
     * @return Epoch duration in seconds (0 if manual epochs)
     */
    function getEpochDuration(bytes32 questionId) external view returns (uint256);

    /**
     * @notice Gets the timestamp when a specific epoch starts
     * @param questionId Market question identifier
     * @param epoch The epoch number to query
     * @return Timestamp when the epoch starts (0 if manual epochs)
     */
    function getEpochStartTime(bytes32 questionId, uint256 epoch) external view returns (uint256);

    /**
     * @notice Gets the timestamp when a specific epoch ends
     * @param questionId Market question identifier
     * @param epoch The epoch number to query
     * @return Timestamp when the epoch ends (0 if manual epochs)
     */
    function getEpochEndTime(bytes32 questionId, uint256 epoch) external view returns (uint256);

    /**
     * @notice Checks if market exists
     * @param questionId Market question identifier
     * @return True if market exists
     */
    function getMarketExists(bytes32 questionId) external view returns (bool);

    /**
     * @notice Sets the authorized MarketController address
     * @param marketController Address of MarketController contract
     */
    function setMarketController(address marketController) external;
}


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import {Script, console} from "forge-std/Script.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";

import "../src/Market/Market.sol";
import "../src/Market/MarketController.sol";
import "../src/Market/MarketResolver.sol";
import "../src/Token/PositionTokens.sol";
import "../src/Vault/Vault.sol";

/**
 * @title Deploy
 * @notice Deterministic deployment script using Solidity's built-in Create2
 * @dev Deploys all contracts with deterministic salts for consistent addresses across chains
 */
contract Deploy is Script {
    // Deployment configuration
    struct DeployConfig {
        address owner;
        address oracle;
        address collateralToken;
        bool deployMockToken;
        uint256 mockTokenSupply;
        bytes32 salt; // Global salt for deterministic deployment
    }

    // Deployed contract addresses
    struct DeployedContracts {
        address collateralToken;
        address marketImpl;
        address marketResolverImpl;
        address positionTokensImpl;
        address vaultImpl;
        address marketControllerImpl;
        address market;
        address marketResolver;
        address positionTokens;
        address vault;
        address marketController;
    }

    // Custom salts for each contract type
    bytes32 constant MARKET_IMPL_SALT = keccak256("PredictionMarket.MarketImpl.v1");
    bytes32 constant MARKET_RESOLVER_IMPL_SALT = keccak256("PredictionMarket.MarketResolverImpl.v1");
    bytes32 constant POSITION_TOKENS_IMPL_SALT = keccak256("PredictionMarket.PositionTokensImpl.v1");
    bytes32 constant VAULT_IMPL_SALT = keccak256("PredictionMarket.VaultImpl.v1");
    bytes32 constant MARKET_CONTROLLER_IMPL_SALT = keccak256("PredictionMarket.MarketControllerImpl.v1");
    bytes32 constant MARKET_PROXY_SALT = keccak256("PredictionMarket.Market.v1");
    bytes32 constant MARKET_RESOLVER_PROXY_SALT = keccak256("PredictionMarket.MarketResolver.v1");
    bytes32 constant POSITION_TOKENS_PROXY_SALT = keccak256("PredictionMarket.PositionTokens.v1");
    bytes32 constant VAULT_PROXY_SALT = keccak256("PredictionMarket.Vault.v1");
    bytes32 constant MARKET_CONTROLLER_PROXY_SALT = keccak256("PredictionMarket.MarketController.v1");

    function run() external {
        // Load configuration
        DeployConfig memory config = getDeployConfig();

        console.log("=== Deterministic Prediction Market Deployment (Create2) ===");
        console.log("Deployer:", msg.sender);
        console.log("Owner:", config.owner);
        console.log("Oracle:", config.oracle);
        console.log("Deploy Mock Token:", config.deployMockToken);
        console.log("Global Salt:", vm.toString(config.salt));

        vm.startBroadcast();

        // Deploy contracts using Create2
        DeployedContracts memory contracts = deployContractsCreate2(config);

        // Link contracts
        linkContracts(contracts);

        vm.stopBroadcast();

        // Save deployment addresses
        saveDeploymentAddresses(contracts, config);

        // Verify deployment
        verifyDeployment(contracts, config);

        console.log("=== Deployment Complete ===");
        logFinalAddresses(contracts);
    }

    function deployContractsCreate2(DeployConfig memory config)
        internal
        returns (DeployedContracts memory contracts)
    {
        console.log("\n--- Deploying Contracts with Create2 ---");

        // 1. Deploy or use existing collateral token
        if (config.deployMockToken) {
            console.log("Deploying mock ERC20 token with Create2...");
            
            address expectedToken = Create2.computeAddress(
                config.salt,
                keccak256(type(ERC20Mock).creationCode),
                msg.sender
            );
            
            if (expectedToken.code.length == 0) {
                ERC20Mock token = new ERC20Mock{salt: config.salt}();
                contracts.collateralToken = address(token);
                
                // Mint initial supply to deployer for testing
                token.mint(msg.sender, config.mockTokenSupply);
                console.log("Mock token deployed:", contracts.collateralToken);
            } else {
                contracts.collateralToken = expectedToken;
                console.log("Mock token already exists:", contracts.collateralToken);
            }
        } else {
            contracts.collateralToken = config.collateralToken;
            console.log("Using existing collateral token:", contracts.collateralToken);
        }

        // 2. Deploy implementation contracts with Create2
        console.log("\nDeploying implementation contracts...");

        // MarketContract implementation
        address expectedMarketImpl = Create2.computeAddress(
            MARKET_IMPL_SALT,
            keccak256(type(MarketContract).creationCode),
            msg.sender
        );
        
        if (expectedMarketImpl.code.length == 0) {
            MarketContract marketImpl = new MarketContract{salt: MARKET_IMPL_SALT}();
            contracts.marketImpl = address(marketImpl);
            console.log("MarketContract implementation deployed:", contracts.marketImpl);
        } else {
            contracts.marketImpl = expectedMarketImpl;
            console.log("MarketContract implementation already exists:", contracts.marketImpl);
        }

        // MarketResolver implementation
        address expectedMarketResolverImpl = Create2.computeAddress(
            MARKET_RESOLVER_IMPL_SALT,
            keccak256(type(MarketResolver).creationCode),
            msg.sender
        );
        
        if (expectedMarketResolverImpl.code.length == 0) {
            MarketResolver marketResolverImpl = new MarketResolver{salt: MARKET_RESOLVER_IMPL_SALT}();
            contracts.marketResolverImpl = address(marketResolverImpl);
            console.log("MarketResolver implementation deployed:", contracts.marketResolverImpl);
        } else {
            contracts.marketResolverImpl = expectedMarketResolverImpl;
            console.log("MarketResolver implementation already exists:", contracts.marketResolverImpl);
        }

        // PositionTokens implementation
        address expectedPositionTokensImpl = Create2.computeAddress(
            POSITION_TOKENS_IMPL_SALT,
            keccak256(type(PositionTokens).creationCode),
            msg.sender
        );
        
        if (expectedPositionTokensImpl.code.length == 0) {
            PositionTokens positionTokensImpl = new PositionTokens{salt: POSITION_TOKENS_IMPL_SALT}();
            contracts.positionTokensImpl = address(positionTokensImpl);
            console.log("PositionTokens implementation deployed:", contracts.positionTokensImpl);
        } else {
            contracts.positionTokensImpl = expectedPositionTokensImpl;
            console.log("PositionTokens implementation already exists:", contracts.positionTokensImpl);
        }

        // Vault implementation
        address expectedVaultImpl = Create2.computeAddress(
            VAULT_IMPL_SALT,
            keccak256(type(Vault).creationCode),
            msg.sender
        );
        
        if (expectedVaultImpl.code.length == 0) {
            Vault vaultImpl = new Vault{salt: VAULT_IMPL_SALT}();
            contracts.vaultImpl = address(vaultImpl);
            console.log("Vault implementation deployed:", contracts.vaultImpl);
        } else {
            contracts.vaultImpl = expectedVaultImpl;
            console.log("Vault implementation already exists:", contracts.vaultImpl);
        }

        // MarketController implementation
        address expectedMarketControllerImpl = Create2.computeAddress(
            MARKET_CONTROLLER_IMPL_SALT,
            keccak256(type(MarketController).creationCode),
            msg.sender
        );
        
        if (expectedMarketControllerImpl.code.length == 0) {
            MarketController marketControllerImpl = new MarketController{salt: MARKET_CONTROLLER_IMPL_SALT}();
            contracts.marketControllerImpl = address(marketControllerImpl);
            console.log("MarketController implementation deployed:", contracts.marketControllerImpl);
        } else {
            contracts.marketControllerImpl = expectedMarketControllerImpl;
            console.log("MarketController implementation already exists:", contracts.marketControllerImpl);
        }

        // 3. Deploy proxies with Create2
        console.log("\nDeploying proxies...");

        // Market proxy
        bytes memory marketInitData = abi.encodeWithSelector(MarketContract.initialize.selector, config.owner);
        bytes memory marketProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(contracts.marketImpl, marketInitData)
        );
        
        address expectedMarket = Create2.computeAddress(
            MARKET_PROXY_SALT,
            keccak256(marketProxyBytecode),
            msg.sender
        );
        
        if (expectedMarket.code.length == 0) {
            ERC1967Proxy marketProxy = new ERC1967Proxy{salt: MARKET_PROXY_SALT}(
                contracts.marketImpl,
                marketInitData
            );
            contracts.market = address(marketProxy);
            console.log("Market proxy deployed:", contracts.market);
        } else {
            contracts.market = expectedMarket;
            console.log("Market proxy already exists:", contracts.market);
        }

        // MarketResolver proxy
        bytes memory marketResolverInitData = abi.encodeWithSelector(
            MarketResolver.initialize.selector,
            config.owner,
            config.oracle
        );
        bytes memory marketResolverProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(contracts.marketResolverImpl, marketResolverInitData)
        );
        
        address expectedMarketResolver = Create2.computeAddress(
            MARKET_RESOLVER_PROXY_SALT,
            keccak256(marketResolverProxyBytecode),
            msg.sender
        );
        
        if (expectedMarketResolver.code.length == 0) {
            ERC1967Proxy marketResolverProxy = new ERC1967Proxy{salt: MARKET_RESOLVER_PROXY_SALT}(
                contracts.marketResolverImpl,
                marketResolverInitData
            );
            contracts.marketResolver = address(marketResolverProxy);
            console.log("MarketResolver proxy deployed:", contracts.marketResolver);
        } else {
            contracts.marketResolver = expectedMarketResolver;
            console.log("MarketResolver proxy already exists:", contracts.marketResolver);
        }

        // PositionTokens proxy
        bytes memory positionTokensInitData = abi.encodeWithSelector(PositionTokens.initialize.selector, config.owner);
        bytes memory positionTokensProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(contracts.positionTokensImpl, positionTokensInitData)
        );
        
        address expectedPositionTokens = Create2.computeAddress(
            POSITION_TOKENS_PROXY_SALT,
            keccak256(positionTokensProxyBytecode),
            msg.sender
        );
        
        if (expectedPositionTokens.code.length == 0) {
            ERC1967Proxy positionTokensProxy = new ERC1967Proxy{salt: POSITION_TOKENS_PROXY_SALT}(
                contracts.positionTokensImpl,
                positionTokensInitData
            );
            contracts.positionTokens = address(positionTokensProxy);
            console.log("PositionTokens proxy deployed:", contracts.positionTokens);
        } else {
            contracts.positionTokens = expectedPositionTokens;
            console.log("PositionTokens proxy already exists:", contracts.positionTokens);
        }

        // Vault proxy
        bytes memory vaultInitData = abi.encodeWithSelector(
            Vault.initialize.selector,
            config.owner,
            contracts.collateralToken,
            msg.sender // Temporary, will be updated to MarketController
        );
        bytes memory vaultProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(contracts.vaultImpl, vaultInitData)
        );
        
        address expectedVault = Create2.computeAddress(
            VAULT_PROXY_SALT,
            keccak256(vaultProxyBytecode),
            msg.sender
        );
        
        if (expectedVault.code.length == 0) {
            ERC1967Proxy vaultProxy = new ERC1967Proxy{salt: VAULT_PROXY_SALT}(
                contracts.vaultImpl,
                vaultInitData
            );
            contracts.vault = address(vaultProxy);
            console.log("Vault proxy deployed:", contracts.vault);
        } else {
            contracts.vault = expectedVault;
            console.log("Vault proxy already exists:", contracts.vault);
        }

        // MarketController proxy (deployed last as it needs other contract addresses)
        bytes memory marketControllerInitData = abi.encodeWithSelector(
            MarketController.initialize.selector,
            config.owner,
            contracts.positionTokens,
            contracts.marketResolver,
            contracts.vault,
            contracts.market,
            config.oracle
        );
        bytes memory marketControllerProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(contracts.marketControllerImpl, marketControllerInitData)
        );
        
        address expectedMarketController = Create2.computeAddress(
            MARKET_CONTROLLER_PROXY_SALT,
            keccak256(marketControllerProxyBytecode),
            msg.sender
        );
        
        if (expectedMarketController.code.length == 0) {
            ERC1967Proxy marketControllerProxy = new ERC1967Proxy{salt: MARKET_CONTROLLER_PROXY_SALT}(
                contracts.marketControllerImpl,
                marketControllerInitData
            );
            contracts.marketController = address(marketControllerProxy);
            console.log("MarketController proxy deployed:", contracts.marketController);
        } else {
            contracts.marketController = expectedMarketController;
            console.log("MarketController proxy already exists:", contracts.marketController);
        }
    }

    function linkContracts(DeployedContracts memory contracts) internal {
        console.log("\n--- Linking Contracts ---");

        // Set MarketController in all contracts
        MarketContract(contracts.market).setMarketController(contracts.marketController);
        console.log("Set MarketController in Market");

        PositionTokens(contracts.positionTokens).setMarketController(contracts.marketController);
        console.log("Set MarketController in PositionTokens");

        Vault(contracts.vault).setMarketController(contracts.marketController);
        console.log("Set MarketController in Vault");

        // Set EmergencyResolver in MarketResolver
        MarketResolver(contracts.marketResolver).setEmergencyResolver(contracts.marketController);
        console.log("Set EmergencyResolver in MarketResolver");

        console.log("Contract linking complete!");
    }

    function getDeployConfig() internal view returns (DeployConfig memory config) {
        // Try to read from environment variables, otherwise use defaults
        try vm.envAddress("OWNER_ADDRESS") returns (address ownerAddr) {
            config.owner = ownerAddr;
        } catch {
            config.owner = msg.sender;
        }
        
        try vm.envAddress("ORACLE_ADDRESS") returns (address oracleAddr) {
            config.oracle = oracleAddr;
        } catch {
            config.oracle = msg.sender;
        }
        
        try vm.envAddress("COLLATERAL_TOKEN_ADDRESS") returns (address tokenAddr) {
            config.collateralToken = tokenAddr;
        } catch {
            config.collateralToken = address(0);
        }
        
        config.deployMockToken = (config.collateralToken == address(0));
        
        try vm.envUint("MOCK_TOKEN_SUPPLY") returns (uint256 supply) {
            config.mockTokenSupply = supply;
        } catch {
            config.mockTokenSupply = 1_000_000e18;
        }
        
        // Generate deterministic salt from deployer address and protocol name
        // This ensures same addresses across chains when using same deployer
        string memory saltString;
        try vm.envString("DEPLOYMENT_SALT") returns (string memory envSalt) {
            saltString = envSalt;
        } catch {
            saltString = "PredictionMarket.v1.0";
        }
        config.salt = keccak256(abi.encodePacked(saltString, msg.sender));

        // Validate configuration
        require(config.owner != address(0), "Owner address cannot be zero");
        require(config.oracle != address(0), "Oracle address cannot be zero");
        if (!config.deployMockToken) {
            require(config.collateralToken != address(0), "Collateral token address cannot be zero");
        }
    }

    function logFinalAddresses(DeployedContracts memory contracts) internal pure {
        console.log("\n--- Final Deployed Addresses ---");
        console.log("Main Entry Point:");
        console.log("  MarketController:", contracts.marketController);
        console.log("Supporting Contracts:");
        console.log("  Market:", contracts.market);
        console.log("  MarketResolver:", contracts.marketResolver);
        console.log("  PositionTokens:", contracts.positionTokens);
        console.log("  Vault:", contracts.vault);
        console.log("  CollateralToken:", contracts.collateralToken);
        console.log("\nThese addresses will be identical on all chains when using same deployer!");
    }

    function saveDeploymentAddresses(DeployedContracts memory contracts, DeployConfig memory config) internal {
        console.log("\n--- Saving Deployment Addresses ---");

        string memory json = "deployment";

        // Network info
        vm.serializeString(json, "network", getNetworkName());
        vm.serializeUint(json, "chainId", block.chainid);
        vm.serializeUint(json, "blockNumber", block.number);
        vm.serializeUint(json, "timestamp", block.timestamp);
        vm.serializeBytes32(json, "deploymentSalt", config.salt);
        vm.serializeBool(json, "deterministicDeployment", true);

        // Contract addresses
        vm.serializeAddress(json, "collateralToken", contracts.collateralToken);
        vm.serializeAddress(json, "marketImpl", contracts.marketImpl);
        vm.serializeAddress(json, "marketResolverImpl", contracts.marketResolverImpl);
        vm.serializeAddress(json, "positionTokensImpl", contracts.positionTokensImpl);
        vm.serializeAddress(json, "vaultImpl", contracts.vaultImpl);
        vm.serializeAddress(json, "marketControllerImpl", contracts.marketControllerImpl);
        vm.serializeAddress(json, "market", contracts.market);
        vm.serializeAddress(json, "marketResolver", contracts.marketResolver);
        vm.serializeAddress(json, "positionTokens", contracts.positionTokens);
        vm.serializeAddress(json, "vault", contracts.vault);
        string memory finalJson = vm.serializeAddress(json, "marketController", contracts.marketController);

        string memory fileName = string.concat("deployments/", getNetworkName(), ".json");
        vm.writeJson(finalJson, fileName);
        console.log("Deployment addresses saved to:", fileName);
    }

    function verifyDeployment(DeployedContracts memory contracts, DeployConfig memory config) internal view {
        console.log("\n--- Verifying Deployment ---");

        // Verify proxy ownership
        require(MarketContract(contracts.market).owner() == config.owner, "Market owner mismatch");
        require(MarketResolver(contracts.marketResolver).owner() == config.owner, "MarketResolver owner mismatch");
        require(PositionTokens(contracts.positionTokens).owner() == config.owner, "PositionTokens owner mismatch");
        require(Vault(contracts.vault).owner() == config.owner, "Vault owner mismatch");
        require(MarketController(contracts.marketController).owner() == config.owner, "MarketController owner mismatch");
        console.log("Owner verification passed");

        // Verify contract linking
        require(
            MarketContract(contracts.market).marketController() == contracts.marketController,
            "Market controller link failed"
        );
        require(
            PositionTokens(contracts.positionTokens).marketController() == contracts.marketController,
            "PositionTokens controller link failed"
        );
        require(Vault(contracts.vault).marketController() == contracts.marketController, "Vault controller link failed");
        require(
            MarketResolver(contracts.marketResolver).emergencyResolver() == contracts.marketController,
            "Emergency resolver link failed"
        );
        console.log("Contract linking verification passed");

        // Verify oracle setup
        require(MarketResolver(contracts.marketResolver).oracle() == config.oracle, "Oracle setup failed");
        console.log("Oracle verification passed");

        // Verify collateral token
        require(
            address(Vault(contracts.vault).collateralToken()) == contracts.collateralToken,
            "Collateral token setup failed"
        );
        console.log("Collateral token verification passed");

        console.log("All verifications passed!");
    }

    function getNetworkName() internal view returns (string memory) {
        uint256 chainId = block.chainid;

        if (chainId == 1) return "mainnet";
        if (chainId == 11155111) return "sepolia";
        if (chainId == 17000) return "holesky";
        if (chainId == 137) return "polygon";
        if (chainId == 80001) return "mumbai";
        if (chainId == 42161) return "arbitrum";
        if (chainId == 421614) return "arbitrum-sepolia";
        if (chainId == 10) return "optimism";
        if (chainId == 11155420) return "optimism-sepolia";
        if (chainId == 8453) return "base";
        if (chainId == 84532) return "base-sepolia";
        if (chainId == 56) return "bsc";
        if (chainId == 97) return "bsc-testnet";
        if (chainId == 146) return "sonic";
        if (chainId == 57054) return "sonic-testnet";
        if (chainId == 31337) return "anvil";

        return string.concat("chain-", vm.toString(chainId));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import {Script, console} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import "../src/Market/Market.sol";
import "../src/Market/MarketController.sol";
import "../src/Market/MarketResolver.sol";
import "../src/Token/PositionTokens.sol";
import "../src/Vault/Vault.sol";

/**
 * @title Upgrade
 * @notice UUPS upgrade script for prediction market contracts
 * @dev Upgrades implementation contracts while preserving proxy addresses and state
 */
contract Upgrade is Script {
    using stdJson for string;

    struct CurrentDeployment {
        address collateralToken;
        address marketImpl;
        address marketResolverImpl;
        address positionTokensImpl;
        address vaultImpl;
        address marketControllerImpl;
        address market;           // proxy
        address marketResolver;   // proxy
        address positionTokens;   // proxy
        address vault;           // proxy
        address marketController; // proxy
        bytes32 deploymentSalt;
        bool deterministicDeployment;
    }

    struct NewImplementations {
        address marketImpl;
        address marketResolverImpl;
        address positionTokensImpl;
        address vaultImpl;
        address marketControllerImpl;
    }

    struct UpgradeConfig {
        bool upgradeMarket;
        bool upgradeMarketResolver;
        bool upgradePositionTokens;
        bool upgradeVault;
        bool upgradeMarketController;
        bool deployDeterministic;  // Whether to use Create2 for new implementations
        string upgradeReason;      // Optional reason for upgrade
    }

    // New salts for upgraded implementations (increment version)
    bytes32 constant MARKET_IMPL_SALT_V6 = keccak256("PredictionMarket.MarketImpl.v6");
    bytes32 constant MARKET_RESOLVER_IMPL_SALT_V6 = keccak256("PredictionMarket.MarketResolverImpl.v6");
    bytes32 constant POSITION_TOKENS_IMPL_SALT_V6 = keccak256("PredictionMarket.PositionTokensImpl.v6");
    bytes32 constant VAULT_IMPL_SALT_V6 = keccak256("PredictionMarket.VaultImpl.v6");
    bytes32 constant MARKET_CONTROLLER_IMPL_SALT_V6 = keccak256("PredictionMarket.MarketControllerImpl.v6");

    function run() external {
        console.log("=== UUPS Contract Upgrade ===");

        // Load current deployment
        CurrentDeployment memory current = loadCurrentDeployment();

        // Get upgrade configuration
        UpgradeConfig memory config = getUpgradeConfig();

        console.log("Upgrade Configuration:");
        console.log("  Network:", getNetworkName());
        console.log("  Upgrade Market:", config.upgradeMarket);
        console.log("  Upgrade MarketResolver:", config.upgradeMarketResolver);
        console.log("  Upgrade PositionTokens:", config.upgradePositionTokens);
        console.log("  Upgrade Vault:", config.upgradeVault);
        console.log("  Upgrade MarketController:", config.upgradeMarketController);
        console.log("  Deploy Deterministic:", config.deployDeterministic);
        if (bytes(config.upgradeReason).length > 0) {
            console.log("  Reason:", config.upgradeReason);
        }

        vm.startBroadcast();

        // Deploy new implementations
        NewImplementations memory newImpls = deployNewImplementations(current, config);

        // Perform upgrades
        performUpgrades(current, newImpls, config);

        vm.stopBroadcast();

        // Verify upgrades
        verifyUpgrades(current, newImpls, config);

        // Save upgrade information
        saveUpgradeInfo(current, newImpls, config);

        console.log("=== Upgrade Complete ===");
        logUpgradeSummary(current, newImpls, config);
    }

    function loadCurrentDeployment() internal view returns (CurrentDeployment memory deployment) {
        string memory networkName = getNetworkName();
        string memory fileName = string.concat("deployments/", networkName, ".json");

        console.log("Loading current deployment from:", fileName);

        string memory json = vm.readFile(fileName);

        deployment.collateralToken = json.readAddress(".collateralToken");
        deployment.marketImpl = json.readAddress(".marketImpl");
        deployment.marketResolverImpl = json.readAddress(".marketResolverImpl");
        deployment.positionTokensImpl = json.readAddress(".positionTokensImpl");
        deployment.vaultImpl = json.readAddress(".vaultImpl");
        deployment.marketControllerImpl = json.readAddress(".marketControllerImpl");
        deployment.market = json.readAddress(".market");
        deployment.marketResolver = json.readAddress(".marketResolver");
        deployment.positionTokens = json.readAddress(".positionTokens");
        deployment.vault = json.readAddress(".vault");
        deployment.marketController = json.readAddress(".marketController");
        deployment.deploymentSalt = json.readBytes32(".deploymentSalt");
        deployment.deterministicDeployment = json.readBool(".deterministicDeployment");

        console.log("Current deployment loaded:");
        console.log("  MarketController proxy:", deployment.marketController);
        console.log("  Current implementation:", deployment.marketControllerImpl);
    }

    function deployNewImplementations(CurrentDeployment memory current, UpgradeConfig memory config)
        internal
        returns (NewImplementations memory newImpls)
    {
        console.log("\n--- Deploying New Implementations ---");

        if (config.upgradeMarket) {
            if (config.deployDeterministic) {
                console.log("Deploying MarketContract implementation (Create2)...");
                MarketContract marketImpl = new MarketContract{salt: MARKET_IMPL_SALT_V6}();
                newImpls.marketImpl = address(marketImpl);
            } else {
                console.log("Deploying MarketContract implementation...");
                MarketContract marketImpl = new MarketContract();
                newImpls.marketImpl = address(marketImpl);
            }
            console.log("  New MarketContract implementation:", newImpls.marketImpl);
        } else {
            newImpls.marketImpl = current.marketImpl;
        }

        if (config.upgradeMarketResolver) {
            if (config.deployDeterministic) {
                console.log("Deploying MarketResolver implementation (Create2)...");
                MarketResolver marketResolverImpl = new MarketResolver{salt: MARKET_RESOLVER_IMPL_SALT_V6}();
                newImpls.marketResolverImpl = address(marketResolverImpl);
            } else {
                console.log("Deploying MarketResolver implementation...");
                MarketResolver marketResolverImpl = new MarketResolver();
                newImpls.marketResolverImpl = address(marketResolverImpl);
            }
            console.log("  New MarketResolver implementation:", newImpls.marketResolverImpl);
        } else {
            newImpls.marketResolverImpl = current.marketResolverImpl;
        }

        if (config.upgradePositionTokens) {
            if (config.deployDeterministic) {
                console.log("Deploying PositionTokens implementation (Create2)...");
                PositionTokens positionTokensImpl = new PositionTokens{salt: POSITION_TOKENS_IMPL_SALT_V6}();
                newImpls.positionTokensImpl = address(positionTokensImpl);
            } else {
                console.log("Deploying PositionTokens implementation...");
                PositionTokens positionTokensImpl = new PositionTokens();
                newImpls.positionTokensImpl = address(positionTokensImpl);
            }
            console.log("  New PositionTokens implementation:", newImpls.positionTokensImpl);
        } else {
            newImpls.positionTokensImpl = current.positionTokensImpl;
        }

        if (config.upgradeVault) {
            if (config.deployDeterministic) {
                console.log("Deploying Vault implementation (Create2)...");
                Vault vaultImpl = new Vault{salt: VAULT_IMPL_SALT_V6}();
                newImpls.vaultImpl = address(vaultImpl);
            } else {
                console.log("Deploying Vault implementation...");
                Vault vaultImpl = new Vault();
                newImpls.vaultImpl = address(vaultImpl);
            }
            console.log("  New Vault implementation:", newImpls.vaultImpl);
        } else {
            newImpls.vaultImpl = current.vaultImpl;
        }

        if (config.upgradeMarketController) {
            if (config.deployDeterministic) {
                console.log("Deploying MarketController implementation (Create2)...");
                MarketController marketControllerImpl = new MarketController{salt: MARKET_CONTROLLER_IMPL_SALT_V6}();
                newImpls.marketControllerImpl = address(marketControllerImpl);
            } else {
                console.log("Deploying MarketController implementation...");
                MarketController marketControllerImpl = new MarketController();
                newImpls.marketControllerImpl = address(marketControllerImpl);
            }
            console.log("  New MarketController implementation:", newImpls.marketControllerImpl);
        } else {
            newImpls.marketControllerImpl = current.marketControllerImpl;
        }
    }

    function performUpgrades(
        CurrentDeployment memory current,
        NewImplementations memory newImpls,
        UpgradeConfig memory config
    ) internal {
        console.log("\n--- Performing UUPS Upgrades ---");

        if (config.upgradeMarket) {
            console.log("Upgrading Market proxy...");
            UUPSUpgradeable(current.market).upgradeToAndCall(
                newImpls.marketImpl,
                ""  // No initialization data needed
            );
            console.log("  Market upgraded successfully");
        }

        if (config.upgradeMarketResolver) {
            console.log("Upgrading MarketResolver proxy...");
            UUPSUpgradeable(current.marketResolver).upgradeToAndCall(
                newImpls.marketResolverImpl,
                ""
            );
            console.log("  MarketResolver upgraded successfully");
        }

        if (config.upgradePositionTokens) {
            console.log("Upgrading PositionTokens proxy...");
            UUPSUpgradeable(current.positionTokens).upgradeToAndCall(
                newImpls.positionTokensImpl,
                ""
            );
            console.log("  PositionTokens upgraded successfully");
        }

        if (config.upgradeVault) {
            console.log("Upgrading Vault proxy...");
            UUPSUpgradeable(current.vault).upgradeToAndCall(
                newImpls.vaultImpl,
                ""
            );
            console.log("  Vault upgraded successfully");
        }

        if (config.upgradeMarketController) {
            console.log("Upgrading MarketController proxy...");
            UUPSUpgradeable(current.marketController).upgradeToAndCall(
                newImpls.marketControllerImpl,
                ""
            );
            console.log("  MarketController upgraded successfully");
        }
    }

    function verifyUpgrades(
        CurrentDeployment memory current,
        NewImplementations memory newImpls,
        UpgradeConfig memory config
    ) internal view {
        console.log("\n--- Verifying Upgrades ---");

        if (config.upgradeMarket) {
            // Verify the proxy is still owned by the correct owner and functioning
            address owner = MarketContract(current.market).owner();
            console.log("Market proxy owner verified:", owner);

            // Verify proxy points to new implementation
            // Note: This verification is simplified - in practice you'd check implementation address
            require(owner != address(0), "Market upgrade verification failed");
        }

        if (config.upgradeMarketController) {
            address owner = MarketController(current.marketController).owner();
            console.log("MarketController proxy owner verified:", owner);
            require(owner != address(0), "MarketController upgrade verification failed");
        }

        if (config.upgradeVault) {
            address owner = Vault(current.vault).owner();
            console.log("Vault proxy owner verified:", owner);
            require(owner != address(0), "Vault upgrade verification failed");
        }

        console.log("All upgrade verifications passed!");
    }

    function saveUpgradeInfo(
        CurrentDeployment memory current,
        NewImplementations memory newImpls,
        UpgradeConfig memory config
    ) internal {
        console.log("\n--- Saving Upgrade Information ---");

        string memory networkName = getNetworkName();

        // Update the main deployment file with new implementation addresses
        string memory json = "upgrade";

        // Network info
        vm.serializeString(json, "network", networkName);
        vm.serializeUint(json, "chainId", block.chainid);
        vm.serializeUint(json, "upgradeBlockNumber", block.number);
        vm.serializeUint(json, "upgradeTimestamp", block.timestamp);
        vm.serializeBytes32(json, "deploymentSalt", current.deploymentSalt);
        vm.serializeBool(json, "deterministicDeployment", current.deterministicDeployment);

        // Contract addresses (proxies remain the same)
        vm.serializeAddress(json, "collateralToken", current.collateralToken);
        vm.serializeAddress(json, "marketImpl", newImpls.marketImpl);
        vm.serializeAddress(json, "marketResolverImpl", newImpls.marketResolverImpl);
        vm.serializeAddress(json, "positionTokensImpl", newImpls.positionTokensImpl);
        vm.serializeAddress(json, "vaultImpl", newImpls.vaultImpl);
        vm.serializeAddress(json, "marketControllerImpl", newImpls.marketControllerImpl);
        vm.serializeAddress(json, "market", current.market);
        vm.serializeAddress(json, "marketResolver", current.marketResolver);
        vm.serializeAddress(json, "positionTokens", current.positionTokens);
        vm.serializeAddress(json, "vault", current.vault);
        string memory finalJson = vm.serializeAddress(json, "marketController", current.marketController);

        // Save updated deployment
        string memory fileName = string.concat("deployments/", networkName, ".json");
        vm.writeJson(finalJson, fileName);
        console.log("Updated deployment saved to:", fileName);

        // Also save upgrade history
        string memory upgradeJson = "upgradeHistory";
        vm.serializeUint(upgradeJson, "blockNumber", block.number);
        vm.serializeUint(upgradeJson, "timestamp", block.timestamp);
        vm.serializeString(upgradeJson, "reason", config.upgradeReason);

        // Previous implementations
        vm.serializeAddress(upgradeJson, "previous_marketImpl", current.marketImpl);
        vm.serializeAddress(upgradeJson, "previous_marketResolverImpl", current.marketResolverImpl);
        vm.serializeAddress(upgradeJson, "previous_positionTokensImpl", current.positionTokensImpl);
        vm.serializeAddress(upgradeJson, "previous_vaultImpl", current.vaultImpl);
        vm.serializeAddress(upgradeJson, "previous_marketControllerImpl", current.marketControllerImpl);

        // New implementations
        vm.serializeAddress(upgradeJson, "new_marketImpl", newImpls.marketImpl);
        vm.serializeAddress(upgradeJson, "new_marketResolverImpl", newImpls.marketResolverImpl);
        vm.serializeAddress(upgradeJson, "new_positionTokensImpl", newImpls.positionTokensImpl);
        vm.serializeAddress(upgradeJson, "new_vaultImpl", newImpls.vaultImpl);
        string memory finalUpgradeJson = vm.serializeAddress(upgradeJson, "new_marketControllerImpl", newImpls.marketControllerImpl);

        string memory upgradeHistoryFile = string.concat("deployments/", networkName, "-upgrade-", vm.toString(block.timestamp), ".json");
        vm.writeJson(finalUpgradeJson, upgradeHistoryFile);
        console.log("Upgrade history saved to:", upgradeHistoryFile);
    }

    function getUpgradeConfig() internal view returns (UpgradeConfig memory config) {
        // Read configuration from environment variables with defaults
        try vm.envBool("UPGRADE_MARKET") returns (bool upgrade) {
            config.upgradeMarket = upgrade;
        } catch {
            config.upgradeMarket = true;  // Default: upgrade all
        }

        try vm.envBool("UPGRADE_MARKET_RESOLVER") returns (bool upgrade) {
            config.upgradeMarketResolver = upgrade;
        } catch {
            config.upgradeMarketResolver = true;
        }

        try vm.envBool("UPGRADE_POSITION_TOKENS") returns (bool upgrade) {
            config.upgradePositionTokens = upgrade;
        } catch {
            config.upgradePositionTokens = true;
        }

        try vm.envBool("UPGRADE_VAULT") returns (bool upgrade) {
            config.upgradeVault = upgrade;
        } catch {
            config.upgradeVault = true;
        }

        try vm.envBool("UPGRADE_MARKET_CONTROLLER") returns (bool upgrade) {
            config.upgradeMarketController = upgrade;
        } catch {
            config.upgradeMarketController = true;
        }

        try vm.envBool("UPGRADE_DETERMINISTIC") returns (bool deterministic) {
            config.deployDeterministic = deterministic;
        } catch {
            config.deployDeterministic = true;  // Default to deterministic for consistency
        }

        try vm.envString("UPGRADE_REASON") returns (string memory reason) {
            config.upgradeReason = reason;
        } catch {
            config.upgradeReason = "Contract upgrade";
        }
    }

    function logUpgradeSummary(
        CurrentDeployment memory current,
        NewImplementations memory newImpls,
        UpgradeConfig memory config
    ) internal pure {
        console.log("\n--- Upgrade Summary ---");
        console.log("Proxy addresses (unchanged):");
        console.log("  MarketController:", current.marketController);
        console.log("  Market:", current.market);
        console.log("  MarketResolver:", current.marketResolver);
        console.log("  PositionTokens:", current.positionTokens);
        console.log("  Vault:", current.vault);
        console.log("");
        console.log("Implementation changes:");

        if (config.upgradeMarketController) {
            console.log("  MarketController:", current.marketControllerImpl, "->", newImpls.marketControllerImpl);
        }
        if (config.upgradeMarket) {
            console.log("  Market:", current.marketImpl, "->", newImpls.marketImpl);
        }
        if (config.upgradeMarketResolver) {
            console.log("  MarketResolver:", current.marketResolverImpl, "->", newImpls.marketResolverImpl);
        }
        if (config.upgradePositionTokens) {
            console.log("  PositionTokens:", current.positionTokensImpl, "->", newImpls.positionTokensImpl);
        }
        if (config.upgradeVault) {
            console.log("  Vault:", current.vaultImpl, "->", newImpls.vaultImpl);
        }
        console.log("");
        console.log("Users can continue using the same proxy addresses!");
        console.log("All state and balances are preserved.");
    }

    function getNetworkName() internal view returns (string memory) {
        uint256 chainId = block.chainid;

        if (chainId == 1) return "mainnet";
        if (chainId == 11155111) return "sepolia";
        if (chainId == 17000) return "holesky";
        if (chainId == 137) return "polygon";
        if (chainId == 80001) return "mumbai";
        if (chainId == 42161) return "arbitrum";
        if (chainId == 421614) return "arbitrum-sepolia";
        if (chainId == 10) return "optimism";
        if (chainId == 11155420) return "optimism-sepolia";
        if (chainId == 8453) return "base";
        if (chainId == 84532) return "base-sepolia";
        if (chainId == 56) return "bsc";
        if (chainId == 97) return "bsc-testnet";
        if (chainId == 146) return "sonic";
        if (chainId == 57054) return "sonic-testnet";
        if (chainId == 31337) return "anvil";

        return string.concat("chain-", vm.toString(chainId));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import {Script, console} from "forge-std/Script.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

import "../src/Market/Market.sol";
import "../src/Market/MarketController.sol";
import "../src/Market/MarketResolver.sol";
import "../src/Token/PositionTokens.sol";
import "../src/Vault/Vault.sol";

/**
 * @title Setup
 * @notice Post-deployment setup script for prediction market contracts
 * @dev Handles initial configuration, test markets, and user setup
 */
contract Setup is Script {
    using stdJson for string;

    struct DeployedContracts {
        address collateralToken;
        address market;
        address marketResolver;
        address positionTokens;
        address vault;
        address marketController;
    }

    struct SetupConfig {
        bool createTestMarkets;
        bool setupTestUsers;
        bool setAuthorizedMatchers;
        address[] testUsers;
        address[] authorizedMatchers;
        uint256 testTokenAmount;
    }

    function run() external {
        console.log("=== Post-Deployment Setup ===");

        // Load deployed contract addresses
        DeployedContracts memory contracts = loadDeployedContracts();

        // Load setup configuration
        SetupConfig memory config = getSetupConfig();

        vm.startBroadcast();

        // Execute setup steps
        if (config.createTestMarkets) {
            createTestMarkets(contracts);
        }

        if (config.setupTestUsers) {
            setupTestUsers(contracts, config);
        }

        if (config.setAuthorizedMatchers) {
            setAuthorizedMatchers(contracts, config);
        }

        vm.stopBroadcast();

        // Verify setup
        verifySetup(contracts, config);

        console.log("=== Setup Complete ===");
    }

    function loadDeployedContracts() internal view returns (DeployedContracts memory contracts) {
        string memory networkName = getNetworkName();
        string memory fileName = string.concat("deployments/", networkName, ".json");

        console.log("Loading deployment from:", fileName);

        string memory json = vm.readFile(fileName);

        contracts.collateralToken = json.readAddress(".collateralToken");
        contracts.market = json.readAddress(".market");
        contracts.marketResolver = json.readAddress(".marketResolver");
        contracts.positionTokens = json.readAddress(".positionTokens");
        contracts.vault = json.readAddress(".vault");
        contracts.marketController = json.readAddress(".marketController");

        console.log("Loaded deployment addresses:");
        console.log("  Market:", contracts.market);
        console.log("  MarketController:", contracts.marketController);
        console.log("  Vault:", contracts.vault);
    }

    function createTestMarkets(DeployedContracts memory contracts) internal {
        console.log("\n--- Creating Test Markets ---");

        MarketController controller = MarketController(contracts.marketController);

        // Create various test markets
        bytes32[] memory questionIds = new bytes32[](5);
        string[] memory descriptions = new string[](5);
        uint256[] memory outcomeCounts = new uint256[](5);
        uint256[] memory resolutionTimes = new uint256[](5);

        // Market 1: Bitcoin price binary (manual resolution)
        questionIds[0] = keccak256("BTC_USD_50000");
        descriptions[0] = "Will Bitcoin price exceed $50,000 by end of month?";
        outcomeCounts[0] = 2;
        resolutionTimes[0] = 0; // Manual resolution

        // Market 2: Ethereum price binary (time-based resolution)
        questionIds[1] = keccak256("ETH_USD_3000");
        descriptions[1] = "Will Ethereum price exceed $3,000 by end of week?";
        outcomeCounts[1] = 2;
        resolutionTimes[1] = block.timestamp + 7 days;

        // Market 3: Stock market direction (multi-outcome)
        questionIds[2] = keccak256("SPY_DIRECTION");
        descriptions[2] = "S&P 500 direction next week: Up, Down, Sideways, Volatile";
        outcomeCounts[2] = 4;
        resolutionTimes[2] = block.timestamp + 7 days;

        // Market 4: Weather prediction
        questionIds[3] = keccak256("WEATHER_NYC");
        descriptions[3] = "NYC weather tomorrow: Sunny, Cloudy, Rainy";
        outcomeCounts[3] = 3;
        resolutionTimes[3] = block.timestamp + 1 days;

        // Market 5: Sports outcome
        questionIds[4] = keccak256("SPORTS_MATCH");
        descriptions[4] = "Next major game outcome: Team A, Team B";
        outcomeCounts[4] = 2;
        resolutionTimes[4] = block.timestamp + 3 days;

        for (uint256 i = 0; i < questionIds.length; i++) {
            try controller.createMarket(questionIds[i], outcomeCounts[i], resolutionTimes[i], 0) {
                console.log("Created market:", descriptions[i]);
                console.log("  Question ID:", vm.toString(questionIds[i]));
                console.log("  Outcomes:", outcomeCounts[i]);
                if (resolutionTimes[i] > 0) {
                    console.log("  Resolution time:", resolutionTimes[i]);
                } else {
                    console.log("  Manual resolution");
                }
            } catch {
                console.log("Failed to create market:", descriptions[i]);
            }
        }

        // Save test market info
        saveTestMarketInfo(questionIds, descriptions, outcomeCounts, resolutionTimes);
    }

    function setupTestUsers(DeployedContracts memory contracts, SetupConfig memory config) internal {
        console.log("\n--- Setting Up Test Users ---");

        // Only setup test users if we have a mock token (development environment)
        if (config.testUsers.length == 0) {
            console.log("No test users specified, skipping...");
            return;
        }

        ERC20Mock collateralToken = ERC20Mock(contracts.collateralToken);
        // Vault vault = Vault(contracts.vault);

        // Check if this is a mock token by trying to mint (will revert if not mock)
        try collateralToken.mint(address(this), 1) {
            // This is a mock token, we can mint for test users
            for (uint256 i = 0; i < config.testUsers.length; i++) {
                address user = config.testUsers[i];

                // Mint test tokens
                collateralToken.mint(user, config.testTokenAmount);
                console.log("Minted", config.testTokenAmount, "tokens for user:", user);

                // Note: Users will need to approve and deposit manually or via frontend
            }
        } catch {
            console.log("Not a mock token, skipping test user setup");
        }
    }

    function setAuthorizedMatchers(DeployedContracts memory contracts, SetupConfig memory config) internal {
        console.log("\n--- Setting Authorized Matchers ---");

        if (config.authorizedMatchers.length == 0) {
            console.log("No authorized matchers specified, skipping...");
            return;
        }

        MarketController controller = MarketController(contracts.marketController);

        for (uint256 i = 0; i < config.authorizedMatchers.length; i++) {
            address matcher = config.authorizedMatchers[i];
            controller.setAuthorizedMatcher(matcher, true);
            console.log("Authorized matcher:", matcher);
        }
    }

    function getSetupConfig() internal view returns (SetupConfig memory config) {
        // Read configuration from environment variables with try/catch
        try vm.envBool("CREATE_TEST_MARKETS") returns (bool createMarkets) {
            config.createTestMarkets = createMarkets;
        } catch {
            config.createTestMarkets = true;
        }
        
        try vm.envBool("SETUP_TEST_USERS") returns (bool setupUsers) {
            config.setupTestUsers = setupUsers;
        } catch {
            config.setupTestUsers = false;
        }
        
        try vm.envBool("SET_AUTHORIZED_MATCHERS") returns (bool setMatchers) {
            config.setAuthorizedMatchers = setMatchers;
        } catch {
            config.setAuthorizedMatchers = false;
        }
        
        try vm.envUint("TEST_TOKEN_AMOUNT") returns (uint256 tokenAmount) {
            config.testTokenAmount = tokenAmount;
        } catch {
            config.testTokenAmount = 10000e18;
        }

        // Parse test user addresses (comma-separated)
        try vm.envString("TEST_USER_ADDRESSES") returns (string memory userAddresses) {
            if (bytes(userAddresses).length > 0) {
                config.testUsers = parseAddresses(userAddresses);
            }
        } catch {
            // No test users specified
        }

        // Parse authorized matcher addresses (comma-separated)
        try vm.envString("AUTHORIZED_MATCHER_ADDRESSES") returns (string memory matcherAddresses) {
            if (bytes(matcherAddresses).length > 0) {
                config.authorizedMatchers = parseAddresses(matcherAddresses);
            }
        } catch {
            // No matchers specified
        }

        console.log("Setup configuration:");
        console.log("  Create test markets:", config.createTestMarkets);
        console.log("  Setup test users:", config.setupTestUsers);
        console.log("  Set authorized matchers:", config.setAuthorizedMatchers);
        console.log("  Test users count:", config.testUsers.length);
        console.log("  Authorized matchers count:", config.authorizedMatchers.length);
    }

    function parseAddresses(string memory addressString) internal pure returns (address[] memory addresses) {
        // Simple comma-separated address parser
        // In practice, you might want a more robust parser
        bytes memory data = bytes(addressString);
        uint256 count = 1;

        // Count commas to determine array size
        for (uint256 i = 0; i < data.length; i++) {
            if (data[i] == bytes1(",")) {
                count++;
            }
        }

        addresses = new address[](count);
        // Note: This is a simplified implementation
        // For production, use a proper CSV parser or JSON format
    }

    function saveTestMarketInfo(
        bytes32[] memory questionIds,
        string[] memory descriptions,
        uint256[] memory outcomeCounts,
        uint256[] memory resolutionTimes
    ) internal {
        console.log("\n--- Saving Test Market Info ---");

        string memory json = "testMarkets";

        for (uint256 i = 0; i < questionIds.length; i++) {
            string memory marketKey = string.concat("market", vm.toString(i));

            vm.serializeBytes32(json, string.concat(marketKey, ".questionId"), questionIds[i]);
            vm.serializeString(json, string.concat(marketKey, ".description"), descriptions[i]);
            vm.serializeUint(json, string.concat(marketKey, ".outcomeCount"), outcomeCounts[i]);
            vm.serializeUint(json, string.concat(marketKey, ".resolutionTime"), resolutionTimes[i]);
        }

        string memory finalJson = vm.serializeUint(json, "count", questionIds.length);

        string memory fileName = string.concat("deployments/", getNetworkName(), "-test-markets.json");
        vm.writeJson(finalJson, fileName);
        console.log("Test market info saved to:", fileName);
    }

    function verifySetup(DeployedContracts memory contracts, SetupConfig memory config) internal view {
        console.log("\n--- Verifying Setup ---");

        MarketContract market = MarketContract(contracts.market);

        if (config.createTestMarkets) {
            // Verify at least one test market was created
            bytes32 testQuestionId = keccak256("BTC_USD_50000");
            require(market.getMarketExists(testQuestionId), "Test market creation failed");
            console.log("Test markets verified");
        }

        console.log("Setup verification complete!");
    }

    function getNetworkName() internal view returns (string memory) {
        uint256 chainId = block.chainid;

        if (chainId == 1) return "mainnet";
        if (chainId == 11155111) return "sepolia";
        if (chainId == 17000) return "holesky";
        if (chainId == 137) return "polygon";
        if (chainId == 80001) return "mumbai";
        if (chainId == 42161) return "arbitrum";
        if (chainId == 421614) return "arbitrum-sepolia";
        if (chainId == 10) return "optimism";
        if (chainId == 11155420) return "optimism-sepolia";
        if (chainId == 8453) return "base";
        if (chainId == 84532) return "base-sepolia";
        if (chainId == 56) return "bsc";
        if (chainId == 97) return "bsc-testnet";
        if (chainId == 146) return "sonic";
        if (chainId == 57054) return "sonic-testnet";
        if (chainId == 31337) return "anvil";

        return string.concat("chain-", vm.toString(chainId));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import {Script, console} from "forge-std/Script.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {Create2} from "@openzeppelin/contracts/utils/Create2.sol";

import "../src/Market/Market.sol";
import "../src/Market/MarketController.sol";
import "../src/Market/MarketResolver.sol";
import "../src/Token/PositionTokens.sol";
import "../src/Vault/Vault.sol";

/**
 * @title PredictAddresses
 * @notice Predict contract addresses that will be deployed via built-in Create2
 * @dev Run this before deployment to verify addresses will be consistent across chains
 */
contract PredictAddresses is Script {
    
    // Custom salts for each contract type (must match Deploy.s.sol)
    bytes32 constant MARKET_IMPL_SALT = keccak256("PredictionMarket.MarketImpl.v1");
    bytes32 constant MARKET_RESOLVER_IMPL_SALT = keccak256("PredictionMarket.MarketResolverImpl.v1");
    bytes32 constant POSITION_TOKENS_IMPL_SALT = keccak256("PredictionMarket.PositionTokensImpl.v1");
    bytes32 constant VAULT_IMPL_SALT = keccak256("PredictionMarket.VaultImpl.v1");
    bytes32 constant MARKET_CONTROLLER_IMPL_SALT = keccak256("PredictionMarket.MarketControllerImpl.v1");
    bytes32 constant MARKET_PROXY_SALT = keccak256("PredictionMarket.Market.v1");
    bytes32 constant MARKET_RESOLVER_PROXY_SALT = keccak256("PredictionMarket.MarketResolver.v1");
    bytes32 constant POSITION_TOKENS_PROXY_SALT = keccak256("PredictionMarket.PositionTokens.v1");
    bytes32 constant VAULT_PROXY_SALT = keccak256("PredictionMarket.Vault.v1");
    bytes32 constant MARKET_CONTROLLER_PROXY_SALT = keccak256("PredictionMarket.MarketController.v1");

    struct PredictionConfig {
        address deployer;
        address owner;
        address oracle;
        address collateralToken;
        bool deployMockToken;
        bytes32 salt;
    }

    struct PredictedAddresses {
        address collateralToken;
        address marketImpl;
        address marketResolverImpl;
        address positionTokensImpl;
        address vaultImpl;
        address marketControllerImpl;
        address market;
        address marketResolver;
        address positionTokens;
        address vault;
        address marketController;
    }

    function run() external view {
        console.log("=== Address Prediction for Create2 Deployment ===");
        
        PredictionConfig memory config = getPredictionConfig();
        
        console.log("Deployer:", config.deployer);
        console.log("Owner:", config.owner);
        console.log("Oracle:", config.oracle);
        console.log("Deploy Mock Token:", config.deployMockToken);
        console.log("Global Salt:", vm.toString(config.salt));
        console.log("Current Chain ID:", block.chainid);
        
        PredictedAddresses memory addresses = predictAllAddresses(config);
        
        logPredictedAddresses(addresses);
        generateDeploymentSummary(addresses, config);
        logNetworkSpecificInfo();
    }

    function predictAllAddresses(PredictionConfig memory config) internal pure returns (PredictedAddresses memory addresses) {
        // Predict collateral token
        if (config.deployMockToken) {
            addresses.collateralToken = Create2.computeAddress(
                config.salt,
                keccak256(type(ERC20Mock).creationCode),
                config.deployer
            );
        } else {
            addresses.collateralToken = config.collateralToken;
        }

        // Predict implementation addresses using Create2
        addresses.marketImpl = Create2.computeAddress(
            MARKET_IMPL_SALT,
            keccak256(type(MarketContract).creationCode),
            config.deployer
        );

        addresses.marketResolverImpl = Create2.computeAddress(
            MARKET_RESOLVER_IMPL_SALT,
            keccak256(type(MarketResolver).creationCode),
            config.deployer
        );

        addresses.positionTokensImpl = Create2.computeAddress(
            POSITION_TOKENS_IMPL_SALT,
            keccak256(type(PositionTokens).creationCode),
            config.deployer
        );

        addresses.vaultImpl = Create2.computeAddress(
            VAULT_IMPL_SALT,
            keccak256(type(Vault).creationCode),
            config.deployer
        );

        addresses.marketControllerImpl = Create2.computeAddress(
            MARKET_CONTROLLER_IMPL_SALT,
            keccak256(type(MarketController).creationCode),
            config.deployer
        );

        // Predict proxy addresses - need to include constructor parameters
        // Market proxy
        bytes memory marketInitData = abi.encodeWithSelector(MarketContract.initialize.selector, config.owner);
        bytes memory marketProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(addresses.marketImpl, marketInitData)
        );
        addresses.market = Create2.computeAddress(MARKET_PROXY_SALT, keccak256(marketProxyBytecode), config.deployer);

        // MarketResolver proxy
        bytes memory marketResolverInitData = abi.encodeWithSelector(
            MarketResolver.initialize.selector,
            config.owner,
            config.oracle
        );
        bytes memory marketResolverProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(addresses.marketResolverImpl, marketResolverInitData)
        );
        addresses.marketResolver = Create2.computeAddress(
            MARKET_RESOLVER_PROXY_SALT,
            keccak256(marketResolverProxyBytecode),
            config.deployer
        );

        // PositionTokens proxy
        bytes memory positionTokensInitData = abi.encodeWithSelector(PositionTokens.initialize.selector, config.owner);
        bytes memory positionTokensProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(addresses.positionTokensImpl, positionTokensInitData)
        );
        addresses.positionTokens = Create2.computeAddress(
            POSITION_TOKENS_PROXY_SALT,
            keccak256(positionTokensProxyBytecode),
            config.deployer
        );

        // Vault proxy
        bytes memory vaultInitData = abi.encodeWithSelector(
            Vault.initialize.selector,
            config.owner,
            addresses.collateralToken,
            config.deployer // Temporary, will be updated to MarketController
        );
        bytes memory vaultProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(addresses.vaultImpl, vaultInitData)
        );
        addresses.vault = Create2.computeAddress(VAULT_PROXY_SALT, keccak256(vaultProxyBytecode), config.deployer);

        // MarketController proxy
        bytes memory marketControllerInitData = abi.encodeWithSelector(
            MarketController.initialize.selector,
            config.owner,
            addresses.positionTokens,
            addresses.marketResolver,
            addresses.vault,
            addresses.market,
            config.oracle
        );
        bytes memory marketControllerProxyBytecode = abi.encodePacked(
            type(ERC1967Proxy).creationCode,
            abi.encode(addresses.marketControllerImpl, marketControllerInitData)
        );
        addresses.marketController = Create2.computeAddress(
            MARKET_CONTROLLER_PROXY_SALT,
            keccak256(marketControllerProxyBytecode),
            config.deployer
        );
    }

    function getPredictionConfig() internal view returns (PredictionConfig memory config) {
        // Use msg.sender as the deployer (this matches the Deploy script)
        config.deployer = msg.sender;
        
        try vm.envAddress("OWNER_ADDRESS") returns (address ownerAddr) {
            config.owner = ownerAddr;
        } catch {
            config.owner = msg.sender;
        }
        
        try vm.envAddress("ORACLE_ADDRESS") returns (address oracleAddr) {
            config.oracle = oracleAddr;
        } catch {
            config.oracle = msg.sender;
        }
        
        try vm.envAddress("COLLATERAL_TOKEN_ADDRESS") returns (address tokenAddr) {
            config.collateralToken = tokenAddr;
        } catch {
            config.collateralToken = address(0);
        }
        
        config.deployMockToken = (config.collateralToken == address(0));
        
        // Generate deterministic salt from deployer address and protocol name
        string memory saltString;
        try vm.envString("DEPLOYMENT_SALT") returns (string memory envSalt) {
            saltString = envSalt;
        } catch {
            saltString = "PredictionMarket.v1.0";
        }
        config.salt = keccak256(abi.encodePacked(saltString, msg.sender));
    }

    function logPredictedAddresses(PredictedAddresses memory addresses) internal pure {
        console.log("\n=== PREDICTED ADDRESSES ===");
        console.log("(These will be IDENTICAL on all supported chains)");
        
        console.log("\nIMPLEMENTATION CONTRACTS:");
        console.log("MarketContract:      ", addresses.marketImpl);
        console.log("MarketResolver:      ", addresses.marketResolverImpl);
        console.log("PositionTokens:      ", addresses.positionTokensImpl);
        console.log("Vault:               ", addresses.vaultImpl);
        console.log("MarketController:    ", addresses.marketControllerImpl);
        
        console.log("\nPROXY CONTRACTS (Main Interfaces):");
        console.log("Market:              ", addresses.market);
        console.log("MarketResolver:      ", addresses.marketResolver);
        console.log("PositionTokens:      ", addresses.positionTokens);
        console.log("Vault:               ", addresses.vault);
        console.log("MarketController:    ", addresses.marketController);
        
        console.log("\nCOLLATERAL TOKEN:");
        console.log("CollateralToken:     ", addresses.collateralToken);
        
        console.log("\nMAIN ENTRY POINT:");
        console.log("MarketController:    ", addresses.marketController);
        console.log("(This is the contract users will interact with)");
    }

    function generateDeploymentSummary(PredictedAddresses memory addresses, PredictionConfig memory config) internal pure {
        console.log("\n=== DEPLOYMENT SUMMARY ===");
        
        console.log("Deployment Method: Solidity Create2 (Deterministic)");
        console.log("Deployer Address:  ", config.deployer);
        console.log("Same addresses on: ALL supported chains");
        console.log("Owner Will Be:     ", config.owner);
        console.log("Oracle Will Be:    ", config.oracle);
        console.log("");
        
        console.log("PRIMARY CONTRACT FOR USERS:");
        console.log("MarketController: ", addresses.marketController);
        console.log("");
        
        console.log("COPY-PASTE READY ADDRESSES:");
        console.log("MARKET_CONTROLLER_ADDRESS=", addresses.marketController);
        console.log("MARKET_ADDRESS=", addresses.market);
        console.log("VAULT_ADDRESS=", addresses.vault);
        console.log("POSITION_TOKENS_ADDRESS=", addresses.positionTokens);
        console.log("MARKET_RESOLVER_ADDRESS=", addresses.marketResolver);
        console.log("COLLATERAL_TOKEN_ADDRESS=", addresses.collateralToken);
    }

    function logNetworkSpecificInfo() internal view {
        console.log("\n=== NETWORK DEPLOYMENT STATUS ===");
        
        uint256 chainId = block.chainid;
        string memory networkName = getNetworkName();
        
        console.log("Current Network:", networkName);
        console.log("Chain ID:", chainId);
        
        // Check if we have deployed contracts at predicted addresses
        string memory deploymentFile = string.concat("deployments/", networkName, ".json");
        
        try vm.readFile(deploymentFile) returns (string memory) {
            console.log("Deployment exists for this network");
            console.log("File:", deploymentFile);
        } catch {
            console.log("No deployment found for this network");
            console.log("Run deployment with: make deploy-", networkName);
        }
        
        console.log("\nSUPPORTED NETWORKS FOR IDENTICAL ADDRESSES:");
        console.log("- Ethereum Mainnet (chainId: 1)");
        console.log("- Ethereum Sepolia (chainId: 11155111)");
        console.log("- Arbitrum One (chainId: 42161)");
        console.log("- Arbitrum Sepolia (chainId: 421614)");
        console.log("- BSC Mainnet (chainId: 56)");
        console.log("- BSC Testnet (chainId: 97)");
        console.log("- Sonic Mainnet (chainId: 146)");
        console.log("- Sonic Testnet (chainId: 57054)");
        console.log("- Polygon (chainId: 137)");
        console.log("- Base (chainId: 8453)");
        console.log("- Optimism (chainId: 10)");
        console.log("");
        console.log("Addresses will be identical when using the same deployer account.");
    }

    function getNetworkName() internal view returns (string memory) {
        uint256 chainId = block.chainid;

        if (chainId == 1) return "mainnet";
        if (chainId == 11155111) return "sepolia";
        if (chainId == 17000) return "holesky";
        if (chainId == 137) return "polygon";
        if (chainId == 80001) return "mumbai";
        if (chainId == 42161) return "arbitrum";
        if (chainId == 421614) return "arbitrum-sepolia";
        if (chainId == 10) return "optimism";
        if (chainId == 11155420) return "optimism-sepolia";
        if (chainId == 8453) return "base";
        if (chainId == 84532) return "base-sepolia";
        if (chainId == 56) return "bsc";
        if (chainId == 97) return "bsc-testnet";
        if (chainId == 146) return "sonic";
        if (chainId == 57054) return "sonic-testnet";
        if (chainId == 31337) return "anvil";

        return string.concat("chain-", vm.toString(chainId));
    }
}

