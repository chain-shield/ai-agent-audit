
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "./IMarketResolver.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {ReentrancyGuardUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";
import {MerkleProof} from "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";

/**
 * @title MarketResolver
 * @notice Upgradeable version handling market resolution and merkle proof verification for prediction markets
 * @dev Manages resolution data and validates outcome proofs independently of token mechanics with UUPS upgradeability
 */
contract MarketResolver is
    Initializable,
    UUPSUpgradeable,
    OwnableUpgradeable,
    ReentrancyGuardUpgradeable,
    IMarketResolver
{
    /// @notice Maps condition ID to merkle root for outcome verification
    mapping(bytes32 => bytes32) public resolutionMerkleRoots;

    /// @notice Tracks which conditions have been resolved to prevent re-resolution
    mapping(bytes32 => bool) public isResolved;

    /// @notice Address authorized to resolve markets
    address public oracle;

    /// @notice Address authorized for emergency resolution
    address public emergencyResolver;

    /// @dev Gap for future storage variables
    uint256[46] private __gap;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the upgradeable contract
     * @param _initialOwner Initial owner of the contract
     * @param _oracle Initial oracle address
     */
    function initialize(address _initialOwner, address _oracle) public initializer {
        __Ownable_init(_initialOwner);
        __UUPSUpgradeable_init();
        __ReentrancyGuard_init();

        oracle = _oracle;
    }

    /// @dev Restricts function access to contract owner (oracle)
    modifier onlyOracle() {
        require(msg.sender == oracle, "Caller is not the oracle");
        _;
    }

    /// @dev Restricts emergency resolution to authorized emergency resolver
    modifier onlyEmergencyResolver() {
        require(msg.sender == emergencyResolver, "Caller is not emergency resolver");
        _;
    }

    /// @dev Allows both oracle and emergency resolver to resolve markets
    modifier onlyAuthorizedResolver() {
        require(msg.sender == oracle || msg.sender == emergencyResolver, "Not authorized to resolve");
        _;
    }

    /**
     * @notice Authorizes contract upgrades
     * @param newImplementation Address of the new implementation
     * @dev Only callable by contract owner
     */
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    // ============ Contract Management Functions ============

    /**
     * @notice Updates the oracle address
     * @param _oracle New oracle address
     * @dev Only callable by contract owner, for emergency situations
     */
    function updateOracle(address _oracle) external onlyOwner {
        require(_oracle != address(0), "Invalid oracle address");
        oracle = _oracle;
    }

    /**
     * @notice Updates the emergency resolver address
     * @param _emergencyResolver New emergency resolver address
     * @dev Only callable by contract owner, for emergency situations
     */
    function updateEmergencyResolver(address _emergencyResolver) external onlyOwner {
        require(_emergencyResolver != address(0), "Invalid emergency resolver address");
        emergencyResolver = _emergencyResolver;
    }

    /**
     * @notice Sets the authorized oracle address
     * @param _oracle Address of oracle for market resolution
     * @dev Only callable by contract owner
     */
    function setOracle(address _oracle) external onlyOwner {
        require(_oracle != address(0), "Invalid oracle address");
        oracle = _oracle;
    }

    /**
     * @notice Sets the authorized emergency resolver address
     * @param _emergencyResolver Address of emergency resolver (usually MarketController)
     * @dev Only callable by contract owner
     */
    function setEmergencyResolver(address _emergencyResolver) external onlyOwner {
        require(_emergencyResolver != address(0), "Invalid emergency resolver address");
        emergencyResolver = _emergencyResolver;
    }

    /**
     * @notice Resolves specific market epoch by setting merkle root
     * @param questionId Market question identifier
     * @param epoch Specific epoch to resolve
     * @param numberOfOutcomes Number of possible outcomes for validation
     * @param merkleRoot Root hash of merkle tree containing valid outcomes
     * @dev Oracle can resolve any epoch, enables flexible resolution timing
     */
    function resolveMarketEpoch(bytes32 questionId, uint256 epoch, uint256 numberOfOutcomes, bytes32 merkleRoot)
        public
        onlyAuthorizedResolver
        nonReentrant
    {
        _resolveMarketEpochInternal(questionId, epoch, numberOfOutcomes, merkleRoot);
    }

    /**
     * @notice Verifies if outcome won
     * @param conditionId Condition identifier
     * @param selectedOutcome Outcome being verified (1 or 2 for binary)
     * @param merkleProof Proof data for verification
     * @return True if outcome won
     */
    function verifyProof(bytes32 conditionId, uint256 selectedOutcome, bytes32[] calldata merkleProof)
        external
        view
        returns (bool)
    {
        bytes32 merkleRoot = resolutionMerkleRoots[conditionId];
        require(merkleRoot != bytes32(0), "Condition not resolved");

        bytes32 leaf = keccak256(abi.encodePacked(selectedOutcome));
        return MerkleProof.verify(merkleProof, merkleRoot, leaf);
    }

    /**
     * @notice Gets merkle root for resolved condition
     * @param conditionId Condition identifier
     * @return Merkle root hash, zero if not resolved
     */
    function getResolutionRoot(bytes32 conditionId) external view returns (bytes32) {
        return resolutionMerkleRoots[conditionId];
    }

    /**
     * @notice Checks if condition has been resolved
     * @param conditionId Condition identifier
     * @return True if condition is resolved
     */
    function getResolutionStatus(bytes32 conditionId) external view returns (bool) {
        return isResolved[conditionId];
    }

    /**
     * @notice Generates condition ID for market resolution
     * @param oracleAddr Oracle address resolving the condition
     * @param questionId Market question identifier
     * @param numberOfOutcomes Number of possible outcomes
     * @param epoch Specific epoch for the condition
     * @return Unique condition identifier
     * @dev Matches condition ID generation used in other contracts
     */
    function getConditionId(address oracleAddr, bytes32 questionId, uint256 numberOfOutcomes, uint256 epoch)
        public
        pure
        returns (bytes32)
    {
        return keccak256(abi.encodePacked(oracleAddr, questionId, numberOfOutcomes, epoch));
    }

    /**
     * @notice Batch resolves multiple market epochs for gas efficiency
     * @param questionIds Array of market question identifiers
     * @param epochs Array of epochs to resolve
     * @param numberOfOutcomes Array of outcome counts
     * @param merkleRoots Array of merkle roots
     * @dev Arrays must have equal length, enables efficient bulk resolution
     */
    function batchResolveMarkets(
        bytes32[] calldata questionIds,
        uint256[] calldata epochs,
        uint256[] calldata numberOfOutcomes,
        bytes32[] calldata merkleRoots
    ) external onlyAuthorizedResolver {
        require(
            questionIds.length == epochs.length && epochs.length == numberOfOutcomes.length
                && numberOfOutcomes.length == merkleRoots.length,
            "Array length mismatch"
        );

        for (uint256 i = 0; i < questionIds.length; i++) {
            _resolveMarketEpochInternal(questionIds[i], epochs[i], numberOfOutcomes[i], merkleRoots[i]);
        }
    }

    /**
     * @notice Internal function for resolving market epoch without reentrancy guard
     * @param questionId Market question identifier
     * @param epoch Specific epoch to resolve
     * @param numberOfOutcomes Number of possible outcomes for validation
     * @param merkleRoot Root hash of merkle tree containing valid outcomes
     */
    function _resolveMarketEpochInternal(
        bytes32 questionId,
        uint256 epoch,
        uint256 numberOfOutcomes,
        bytes32 merkleRoot
    ) internal {
        require(epoch > 0, "Invalid epoch");
        require(numberOfOutcomes > 0, "Invalid outcome count");
        require(merkleRoot != bytes32(0), "Invalid merkle root");

        bytes32 conditionId = getConditionId(oracle, questionId, numberOfOutcomes, epoch);

        require(!isResolved[conditionId], "Already resolved");

        resolutionMerkleRoots[conditionId] = merkleRoot;
        isResolved[conditionId] = true;

        emit ConditionResolved(conditionId, questionId, epoch, merkleRoot);
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

/**
 * @title IMarketResolver
 * @notice Interface for market resolution and merkle proof verification
 */
interface IMarketResolver {
    /// @notice Emitted when market condition is resolved with merkle root
    event ConditionResolved(
        bytes32 indexed conditionId,
        bytes32 indexed questionId,
        uint256 indexed epoch,
        bytes32 merkleRoot
    );

    /**
     * @notice Resolves specific market epoch by setting merkle root
     * @param questionId Market question identifier
     * @param epoch Specific epoch to resolve
     * @param numberOfOutcomes Number of possible outcomes for validation
     * @param merkleRoot Root hash of merkle tree containing valid outcomes
     */
    function resolveMarketEpoch(bytes32 questionId, uint256 epoch, uint256 numberOfOutcomes, bytes32 merkleRoot)
        external;

    /**
     * @notice Verifies if outcome won
     * @param conditionId Condition identifier
     * @param selectedOutcome Outcome being verified (1 or 2 for binary)
     * @param merkleProof Proof data for verification
     * @return True if outcome won
     */
    function verifyProof(bytes32 conditionId, uint256 selectedOutcome, bytes32[] calldata merkleProof)
        external
        view
        returns (bool);

    /**
     * @notice Gets merkle root for resolved condition
     * @param conditionId Condition identifier
     * @return Merkle root hash, zero if not resolved
     */
    function getResolutionRoot(bytes32 conditionId) external view returns (bytes32);

    /**
     * @notice Checks if condition has been resolved
     * @param conditionId Condition identifier
     * @return True if condition is resolved
     */
    function getResolutionStatus(bytes32 conditionId) external view returns (bool);

    /**
     * @notice Batch resolves multiple market epochs for gas efficiency
     * @param questionIds Array of market question identifiers
     * @param epochs Array of epochs to resolve
     * @param numberOfOutcomes Array of outcome counts
     * @param merkleRoots Array of merkle roots
     */
    function batchResolveMarkets(
        bytes32[] calldata questionIds,
        uint256[] calldata epochs,
        uint256[] calldata numberOfOutcomes,
        bytes32[] calldata merkleRoots
    ) external;

    /**
     * @notice Sets the authorized oracle address
     * @param oracle Address of oracle for market resolution
     */
    function setOracle(address oracle) external;

    /**
     * @notice Sets the authorized emergency resolver address
     * @param emergencyResolver Address of emergency resolver (usually MarketController)
     */
    function setEmergencyResolver(address emergencyResolver) external;

    /**
     * @notice Returns the current oracle address
     * @return Address of the oracle
     */
    function oracle() external view returns (address);

    /**
     * @notice Returns the current emergency resolver address
     * @return Address of the emergency resolver
     */
    function emergencyResolver() external view returns (address);
}


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

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

