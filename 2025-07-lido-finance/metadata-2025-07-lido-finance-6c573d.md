
## SUMMARY OF FILE: 2025-07-lido-finance/src/CSModule.sol
### Project File Overview
The project contains a comprehensive set of files, consisting of scripts, source code, libraries, and tests that are organized into various directories:

- **Scripts**: Contain deployment scripts such as `DeployBase.s.sol`, `DeployMainnet.s.sol`, and several others tailored for different environments and functionalities. It also includes helper scripts like `NodeOperators.s.sol` for facilitating various operations.

- **Source (`src`)**: This comprises core contracts for modules such as `CSModule`, `CSVerifier`, `PermissionlessGate`, among others. It also includes abstract contracts like `CSBondCore` and `CSBondLock`, and interfaces providing contract standards.

- **Libraries (`lib`)**: Essential utility libraries like `QueueLib` and `SigningKeys.sol` provide functionality to abstract reusable functionalities across the project.

- **Tests**: Comprehensive set of test contracts across different modules like `AssetRecoverer.t.sol` and specific environments like `integration/Oracle.t.sol` ensure code reliability.

- **Utilities**: Auxiliary scripts including `Json.sol` and `Dummy.sol` provide additional functionalities required across the project. 

This structure ensures coherence and modularity across the project, promoting efficient development and testing cycles.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSParametersRegistry.sol
### CSParametersRegistry Contract

**Definition:**
The `CSParametersRegistry` contract is responsible for managing staking parameters related to staking nodes, including fees, limits, and performance metrics. It allows for flexible configuration per curve and includes fail-safe settings to prevent sybil attacks.

### Contract Summary
This contract is a part of the Lido Finance ecosystem, providing configuration and management of staking parameters for node operators. Utilizing OpenZeppelin's upgradeable access control and initialization mechanisms, it maintains various default and curve-specific settings for staking operations. The contract includes numerous functions to set or unset values per curve ID, ensuring precise control over staking rules.

### Function Summaries

- **initialize**
  ```solidity
  function initialize(address admin, InitializationData calldata data) external initializer
  ```
  Initializes contract settings with administrator rights, default values, and configurations ensuring only an assigned admin can manage these configurations.

- **setDefaultKeyRemovalCharge**
  ```solidity
  function setDefaultKeyRemovalCharge(uint256 keyRemovalCharge) external
  ```
  Sets the fee for key removal, which is a cost associated with removing validator keys, ensuring fair operation.

- **setDefaultElRewardsStealingAdditionalFine**
  ```solidity
  function setDefaultElRewardsStealingAdditionalFine(uint256 fine) external
  ```
  Configures an additional fine for stealing EL rewards, enhancing integrity by penalizing malicious behavior.

- **setDefaultKeysLimit**
  ```solidity
  function setDefaultKeysLimit(uint256 limit) external
  ```
  Sets limits for validator keys, helping to prevent over-provisioning and maintain network security.

- **setDefaultRewardShare**
  ```solidity
  function setDefaultRewardShare(uint256 share) external
  ```
  Defines the default reward share for validators, crucial for incentive alignment; capped at a maximum of 10,000 basis points (BP).

- **setKeyRemovalCharge**
  ```solidity
  function setKeyRemovalCharge(uint256 curveId, uint256 keyRemovalCharge) external
  ```
  Allows setting a specific key removal charge for a given curve, providing flexibility in operational fees.

### Storage Variables

- **MAX_BP**
  ```solidity
  uint256 internal constant MAX_BP = 10000
  ```
  Represents the maximum value for a basis point used in relative fee calculations, equivalent to 100%.

- **defaultKeyRemovalCharge**
  ```solidity
  uint256 public defaultKeyRemovalCharge
  ```
  Stores the default cost for removing keys, used to enforce payment for maintenance actions.

- **QUEUE_LOWEST_PRIORITY**
  ```solidity
  uint256 public immutable QUEUE_LOWEST_PRIORITY
  ```
  Establishes the lowest priority for queue management, facilitating organized transaction processing and execution precedence.

Overall, `CSParametersRegistry` is an essential contract driving flexibility and security in Lido's node operations by managing parameters on an individual and default basis.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSAccounting.sol
### CSAccounting Contract Summary

**Contract Definition**: The `CSAccounting` contract is responsible for managing node operators' bonds in stETH shares and is integrated with Lido's staking module. It handles various bond-related operations within a network, such as deposit, withdraw, and reward claims for node operators.

#### Main Components:
- **Roles**: Utilizes multiple access roles like `PAUSE_ROLE`, `RESUME_ROLE`, etc., for access management.
- **Modules**: It interfaces with `ICSModule` and `ICSFeeDistributor` to handle modules and fee distributions.

### Functions Explained

- **Constructor**: `constructor(address lidoLocator, address module, address _feeDistributor, uint256 minBondLockPeriod, uint256 maxBondLockPeriod)` initializes the contract with relevant module addresses and bond lock periods.
- **initialize**: `initialize(BondCurveIntervalInput[] calldata bondCurve, address admin, uint256 bondLockPeriod, address _chargePenaltyRecipient)` sets up bond curves and admin access post-deployment.
- **finalizeUpgradeV2**: `finalizeUpgradeV2(BondCurveIntervalInput[][] calldata bondCurvesInputs)` is used to migrate existing bond curves to a new format.
- **resume**: `resume()` is used to resume bond functions, only callable by authorized roles.
- **pauseFor**: `pauseFor(uint256 duration)` pauses operations for a specified duration.
- **setChargePenaltyRecipient**: `setChargePenaltyRecipient(address _chargePenaltyRecipient)` defines who receives penalties.
- **depositETH**: `depositETH(address from, uint256 nodeOperatorId)` facilitates ETH deposit processes.
- **depositStETH**: `depositStETH(address from, uint256 nodeOperatorId, uint256 stETHAmount, PermitInput calldata permit)` allows for stETH deposits with permit check.
- **lockBondETH**: `lockBondETH(uint256 nodeOperatorId, uint256 amount)` locks specified bond amount for a node operator.
- **penalize**: `penalize(uint256 nodeOperatorId, uint256 amount)` burns a specified bond amount as a penalty.
- **getBondSummary**: `getBondSummary(uint256 nodeOperatorId)` provides the current and required bond status for a node operator.

### Key Storage Variables

- **MODULE**: Interface reference to the staking module used for node operations.
- **FEE_DISTRIBUTOR**: Interface reference for managing fee distributions to node operators.
- **chargePenaltyRecipient**: Address to receive penalty charges for unauthorized actions.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSStrikes.sol
### CSStrikes Contract

The `CSStrikes` contract is a part of a larger smart contract suite dealing with the management of node operators in a staking ecosystem. This contract extends `ICSStrikes`, `Initializable`, and `AccessControlEnumerableUpgradeable`, focusing on verifying Merkle proofs of bad performance and managing strikes against node operators. It utilizes an oracle to update performance data and applies a system to handle poorly performing operators.

### Storage Variables

- **ORACLE**: An immutable address of the oracle that provides updated Merkle tree roots.
- **MODULE**: Immutable reference to the CS module, used to interact with other components.
- **ACCOUNTING**: Immutable interface to manage financial aspects of operations.
- **EXIT_PENALTIES**: Reference for handling penalty enforcement.
- **PARAMETERS_REGISTRY**: Registry for CS parameters affecting operations.
- **ejector**: Address that handles the ejection of bad performers.
- **treeRoot**: Stores the current Merkle tree root linked to node performance.
- **treeCid**: Contains the CID of the latest published Merkle tree.

### Constructor

```solidity
constructor(address module, address oracle, address exitPenalties, address parametersRegistry)
```
The constructor initializes immutable contract references. It ensures no zero addresses are used.

### Functions

- **initialize(address admin, address _ejector)**:
  ```solidity
  function initialize(address admin, address _ejector) external initializer
  ```
  Sets up the contract's roles and can only be called once, linking an ejector for handling underperforming entities.

- **setEjector(address _ejector)**:
  ```solidity
  function setEjector(address _ejector) external onlyRole(DEFAULT_ADMIN_ROLE)
  ```
  Updates the ejector address to manage underperformers, restricted to administrators.

- **processOracleReport(bytes32 _treeRoot, string calldata _treeCid)**:
  ```solidity
  function processOracleReport(bytes32 _treeRoot, string calldata _treeCid) external onlyOracle
  ```
  Updates or deletes Merkle tree indicators, ensuring data consistency and originated by the oracle.

- **processBadPerformanceProof(KeyStrikes[] calldata keyStrikesList, bytes32[] calldata proof, bool[] calldata proofFlags, address refundRecipient)**:
  ```solidity
  function processBadPerformanceProof(KeyStrikes[] calldata keyStrikesList, bytes32[] calldata proof, bool[] calldata proofFlags, address refundRecipient) external payable
  ```
  Processes proofs of poor performance, verifying claims with a Merkle tree approach, distributing refunds if authenticated.

- **getInitializedVersion()**:
  ```solidity
  function getInitializedVersion() external view returns (uint64)
  ```
  Returns the version number indicating the state of initialization, ensuring all settings are properly configured.

- **verifyProof(KeyStrikes[] calldata keyStrikesList, bytes[] memory pubkeys, bytes32[] calldata proof, bool[] calldata proofFlags)**:
  ```solidity
  function verifyProof(KeyStrikes[] calldata keyStrikesList, bytes[] memory pubkeys, bytes32[] calldata proof, bool[] calldata proofFlags) public view returns (bool)
  ```
  Confirms the validity of a Merkle proof against the stored tree root, pivotal for strike verification.

- **hashLeaf(KeyStrikes calldata keyStrikes, bytes memory pubkey)**:
  ```solidity
  function hashLeaf(KeyStrikes calldata keyStrikes, bytes memory pubkey) public pure returns (bytes32)
  ```
  Hashes relevant data to create tree leaves, essential for proof generation and verification.

- **_setEjector(address _ejector)**:
  ```solidity
  function _setEjector(address _ejector) internal
  ```
  Internal method for assigning a new ejector, pivotal for contract operation and performance management.

- **_ejectByStrikes(KeyStrikes calldata keyStrikes, bytes memory pubkey, uint256 value, address refundRecipient)**:
  ```solidity
  function _ejectByStrikes(KeyStrikes calldata keyStrikes, bytes memory pubkey, uint256 value, address refundRecipient) internal
  ```
  Ejects nodes that meet or exceed a strike threshold, acting based on verified performance data.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSFeeOracle.sol
# CSFeeOracle Contract Summary

The `CSFeeOracle` contract is defined to work within a decentralized oracle network, performing functions related to fee distribution and strike handling. It incorporates base oracle functionalities and pause-resume mechanisms. This contract is primarily meant to interact with other components of a decentralized staking solution, such as the `ICSFeeDistributor` and `ICSStrikes` interfaces.

## Contract: CSFeeOracle

### Contract Definition

This contract inherits from multiple interfaces and abstract contracts:
- `ICSFeeOracle`
- `BaseOracle`
- `PausableUntil`
- `AssetRecoverer`

### Important Variables

- `SUBMIT_DATA_ROLE`: A constant bytes32 representing an ACL role allowing data submission (`"SUBMIT_DATA_ROLE"`).
- `PAUSE_ROLE`: Allows pausing oracle reports (`"PAUSE_ROLE"`).
- `RESUME_ROLE`: Allows resumption of oracle reports (`"RESUME_ROLE"`).
- `RECOVERER_ROLE`: Allows asset recovery (`"RECOVERER_ROLE"`).
- `FEE_DISTRIBUTOR`: An immutable reference to the fee distributor contract.
- `STRIKES`: An immutable reference to the strikes contract.
- `_feeDistributor` and `_avgPerfLeewayBP`: Deprecated internal variables for backward support.

### Function Summaries

#### `constructor(address feeDistributor, address strikes, uint256 secondsPerSlot, uint256 genesisTime)`: 
Initializes the contract with references to fee distributor and strikes, validating non-zero addresses.

#### `initialize(address admin, address consensusContract, uint256 consensusVersion)`: 
Further sets up the contract with admin rights and initializes base oracle functionalities.

#### `finalizeUpgradeV2(uint256 consensusVersion)`: 
Handles updates post-proxy upgrades, ensuring the version consistency and storage slot cleanup.

#### `resume()`: 
Resumes oracle operations. Requires `RESUME_ROLE`.

#### `pauseFor(uint256 duration)`: 
Pauses oracle operations for a specified duration. Requires `PAUSE_ROLE`.

#### `submitReportData(ReportData calldata data, uint256 contractVersion)`: 
Processes report data submission when the system is resumed.

#### `_handleConsensusReport(ConsensusReport memory, uint256, uint256)`: 
(BaseOracle override) No asynchronous processing needed so far.

#### `_handleConsensusReportData(ReportData calldata data)`: 
Internal function to process the consensus report.

#### `_checkMsgSenderIsAllowedToSubmitData()`: 
Verifies if the message sender has the permission to submit data.

#### `_onlyRecoverer()`: 
Ensures that only users with the `RECOVERER_ROLE` can perform recoverer actions.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSExitPenalties.sol
### CSExitPenalties Contract
The `CSExitPenalties` contract manages penalties associated with validator exits in a staking module. The main purpose is to process and record penalties when validators exit a staking pool under specific conditions.

#### Contract Definition:
- **Contract Name:** CSExitPenalties
- **Inherits:** ICSExitPenalties, ExitTypes
- **Libraries Used:** SafeCast
- **Modifiers:** onlyModule, onlyStrikes

### Functions Summary:

1. **Constructor**: 
   - **Interface**: `constructor(address module, address parametersRegistry, address strikes)`
   - **Summary**: Initializes the contract, setting module, parameters registry, and strikes addresses. Validates non-zero addresses.

2. **processExitDelayReport**:
   - **Interface**: `function processExitDelayReport(uint256 nodeOperatorId, bytes calldata publicKey, uint256 eligibleToExitInSec)`
   - **Summary**: Records an exit delay penalty if applicable. Checks if the exit delay exceeds allowance.

3. **processTriggeredExit**:
   - **Interface**: `function processTriggeredExit(uint256 nodeOperatorId, bytes calldata publicKey, uint256 withdrawalRequestPaidFee, uint256 exitType)`
   - **Summary**: Records a fee for triggered exits, ensuring no manipulation occurs for previously set fees.

4. **processStrikesReport**:
   - **Interface**: `function processStrikesReport(uint256 nodeOperatorId, bytes calldata publicKey)`
   - **Summary**: Applies a strikes penalty to the exit penalty info if not already set.

5. **isValidatorExitDelayPenaltyApplicable**:
   - **Interface**: `function isValidatorExitDelayPenaltyApplicable(uint256 nodeOperatorId, bytes calldata publicKey, uint256 eligibleToExitInSec)`
   - **Summary**: Determines if an exit delay penalty is applicable for a validator. Accessible only by module.

6. **getExitPenaltyInfo**:
   - **Interface**: `function getExitPenaltyInfo(uint256 nodeOperatorId, bytes calldata publicKey)`
   - **Summary**: Retrieves the penalty information for a specific validator.

7. **_keyPointer**:
   - **Interface**: `function _keyPointer(uint256 nodeOperatorId, bytes calldata publicKey) internal pure returns (bytes32)`
   - **Summary**: Generates a key pointer for a validator using node operator ID and public key.

### Storage Variables:

1. **MODULE**
   - **Definition**: `ICSModule public immutable MODULE`
   - **Explanation**: An immutable reference to the ICSModule contract.

2. **PARAMETERS_REGISTRY**
   - **Definition**: `ICSParametersRegistry public immutable PARAMETERS_REGISTRY`
   - **Explanation**: Holds reference to the parameters registry used to fetch parameters such as exit delay and penalties.

3. **ACCOUNTING**
   - **Definition**: `ICSAccounting public immutable ACCOUNTING`
   - **Explanation**: Immutable reference for accounting, connected to MODULE contracts.

4. **STRIKES**
   - **Definition**: `address public immutable STRIKES`
   - **Explanation**: Address of the STRIKES contract, used for penalty strike processing.

5. **_exitPenaltyInfo**
   - **Definition**: `mapping(bytes32 keyPointer => ExitPenaltyInfo) private _exitPenaltyInfo`
   - **Explanation**: Maps validator identifiers to their exit penalty information.


## SUMMARY OF FILE: 2025-07-lido-finance/src/VettedGateFactory.sol
### VettedGateFactory Contract Summary

The VettedGateFactory contract is designed to deploy new instances of the VettedGate contract using the OssifiableProxy pattern. It serves as a factory for creating vetted gates with specific configurations.

### Constructor

```solidity
constructor(address vettedGateImpl)
```

**Summary:**
Initializes the contract with a specified implementation address for the VettedGate. It checks if the provided address is non-zero. If a zero address is passed, it reverts with an error.

### Function: create

```solidity
function create(uint256 curveId, bytes32 treeRoot, string calldata treeCid, address admin) external returns (address instance)
```

**Summary:**
Creates a new VettedGate instance. This function uses the OssifiableProxy to create a proxy instance pointing to VETTED_GATE_IMPL. It initializes the new VettedGate with parameters such as `curveId`, `treeRoot`, `treeCid`, and `admin`. The function emits a `VettedGateCreated` event with the instance address.

### Storage Variable: VETTED_GATE_IMPL

```solidity
address public immutable VETTED_GATE_IMPL;
```

**Summary:**
Stores the address of the VettedGate implementation used by the factory to create new instances. Its value is immutable and set during contract deployment.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSVerifier.sol
# Project File List and CSVerifier Contract Documentation

## Main List of Files in Project

The project contains a structured set of scripts, source code, library, and testing files, organized as follows:

### Scripts
- Deployment scripts for various networks and constants.
- Fork helpers including node operators and vote simulation utilities.
- Common, Dummy, and JSON utilities.

### Source Code
- Core smart contracts for verification, accounting, ejection, penalties, fee distribution, modules, strikes, gates, and permissions.
- Abstract contracts for asset recovery and bond-related functionalities.
- Interface contracts that define required functionalities for each module.

### Libraries
- Libraries for handling specific functionalities such as indexing, signing keys, queue management, unstructured storage, and the base oracle.

### Testing
- Comprehensive test cases for smart contracts and libraries, alongside integration tests focusing on deployment, staking, and security features.


## CSVerifier Contract

The `CSVerifier` contract is designed for managing and verifying withdrawal proofs on the Ethereum beacon chain. It relies on OpenZeppelin's `AccessControlEnumerable` for role-based access management and the `PausableUntil` utility for contract pausing.

### Key Functions Overview:

- **amountWei: `function amountWei(Withdrawal memory withdrawal) pure returns (uint256)`**
  Converts withdrawal amounts from gwei to wei allowing precise ether calculations.

- **gweiToWei: `function gweiToWei(uint64 amount) pure returns (uint256)`**
  Utility to precisely convert gwei to wei, fundamental for Ethereum currency operations.

- **resume: `function resume() external onlyRole(RESUME_ROLE)`** 
  Enables resumption of operations and mainly used to recover contract functionality after it has been paused.

- **pauseFor: `function pauseFor(uint256 duration) external onlyRole(PAUSE_ROLE)`**
  Invokes a temporary pause, accepting a duration to manage unexpected contract behavior securely.

- **processWithdrawalProof: `function processWithdrawalProof(ProvableBeaconBlockHeader calldata beaconBlock, WithdrawalWitness calldata witness, uint256 nodeOperatorId, uint256 keyIndex) external whenResumed`**
  A central function to process withdrawal proofs, accommodate withdrawals, and manage node operator funds.

- **processHistoricalWithdrawalProof: `function processHistoricalWithdrawalProof(ProvableBeaconBlockHeader calldata beaconBlock, HistoricalHeaderWitness calldata oldBlock, WithdrawalWitness calldata witness, uint256 nodeOperatorId, uint256 keyIndex) external whenResumed`**
  Ensures historical withdrawal proofs are processed, assisting in comprehensive recovery across multiple chain states.

### Storage Variables Summary:

- **PAUSE_ROLE, RESUME_ROLE**: Role identifiers controlling the ability to pause or resume operations.

- **BEACON_ROOTS**: A constant address aligned with EIP-4788 for block roots.

- **SLOTS_PER_EPOCH, SLOTS_PER_HISTORICAL_ROOT**: Chain configuration constants defining epoch and historical rooting timing.

- **GI_FIRST_WITHDRAWAL_PREV, GI_FOIRST_WITHDRAWAL_CURR**: Indices related to withdrawal states for different slots.

- **WITHDRAWAL_ADDRESS**: Ethereum address where withdrawals are directed.


## SUMMARY OF FILE: 2025-07-lido-finance/src/lib/proxy/OssifiableProxy.sol
### OssifiableProxy
OssifiableProxy is an Ethereum smart contract that extends the ERC1967Proxy, adding enhanced admin functionalities. It incorporates mechanisms to ossify (lock) the proxy, preventing future updates. The contract primarily manages the admin roles and supports upgrades through a set of defined rules.

### Function: onlyAdmin modifier
```solidity
modifier onlyAdmin()
```
Ensures that certain functions are only executable by the designated admin. It checks if the admin is not the zero address (meaning the proxy isn't ossified) and if the sender of the function call is indeed the admin.

### Function: constructor
```solidity
constructor(address implementation_, address admin_, bytes memory data_)
```
Initializes the proxy with a specified implementation and admin, optionally executing a delegate call using the provided data. Sets the initial state and admin for the proxy.

### Function: receive()
```solidity
receive() external payable virtual
```
Default function to handle incoming ether, calling the fallback function. Used to suppress compiler warnings related to ether handling.

### Function: proxy__ossify
```solidity
function proxy__ossify() external onlyAdmin
```
Allows the proxy admin to ossify the proxy, revoking admin rights and rendering the proxy immutable against future upgrades.

### Function: proxy__changeAdmin
```solidity
function proxy__changeAdmin(address newAdmin_)
```
Transfers admin rights to a specified new address. Only executable by the current admin.

### Function: proxy__upgradeTo
```solidity
function proxy__upgradeTo(address newImplementation_)
```
Enables the proxy to upgrade its implementation. This is controlled by the current admin.

### Function: proxy__upgradeToAndCall
```solidity
function proxy__upgradeToAndCall(address newImplementation_, bytes calldata setupCalldata_)
```
Performs an upgrade to the proxy's implementation and optionally executes a setup call, contingent on provided data.

### Function: proxy__getAdmin
```solidity
function proxy__getAdmin()
```
Returns the current admin address, facilitating checks on who can perform admin-level tasks.

### Function: proxy__getImplementation
```solidity
function proxy__getImplementation()
```
Provides the current implementation address being pointed to by the proxy.

### Function: proxy__getIsOssified
```solidity
function proxy__getIsOssified()
```
Checks and returns whether the proxy has been ossified, with no active admin to allow future upgrades.

### Storage Variables
- **ADMIN_SLOT**
  - Defined in `ERC1967Utils` and manages the storage slot for admin data. Crucial for handling the admin address safely within the proxy contract.


## SUMMARY OF FILE: 2025-07-lido-finance/src/lib/base-oracle/HashConsensus.sol
**File Overview**
The provided text is a list of files in a project, including Solidity scripts for deploying smart contracts, modules for various functionalities, interfaces for different contract interactions, library files, and a wide range of test files for different modules. These files suggest a comprehensive project setup aimed at managing and testing a full-fledged Ethereum-based application.

**Contract Overview: HashConsensus.sol**
The `HashConsensus` contract manages an oracle members' committee, allowing members to reach consensus on a hash for each reporting frame. The contract uses time frames to organize reporting, with each frame having a reference slot and a processing deadline. Reporting is secured through roles and configurations to ensure proper governance.

**Contract Definition and Functionality**
The `HashConsensus` contract
  - Manages frame-based time organization for oracle reports.
  - Handles committee member roles and quorum for consensus.
  - Provides mechanisms for reporting, consensus achievement, and report processing.

**Function Definitions**
- **Constructor:**
  - `_constructor(...)` configures chain specifics, administrator settings, and frame configurations to ensure functional setup.

- **`getChainConfig`:**
  - Returns immutable network parameters for epoch and slot calculations.

- **`getFrameConfig`:**
  - Provides frame-related configuration details including epochs per frame and fast lane slots.

- **`getCurrentFrame`:**
  - Returns details of the current frame including reference slot and deadline.

- **`updateInitialEpoch`:**
  - Updates initial epoch to reflect new frame initiation standards.

- **`setFrameConfig`:**
  - Reconfigures reporting frames with new epoch and fast lane settings.

- **`getIsMember`:**
  - Checks and returns member status within the consensus framework.

- **`getMembers`:**
  - Fetches active committee members and their last reported slots.

- **`addMember`:**
  - Adds a new member to reporting committee with quorum updates.

- **`removeMember`:**
  - Removes a member and adjusts quorum accordingly.

- **`getQuorum`:**
  - Returns current quorum number required for consensus.

- **`setQuorum`:**
  - Allows quorum adjustments to amend consensus hurdles.

- **`disableConsensus`:**
  - Disables oracle by setting quorum beyond achievable limits.

- **`getReportProcessor`:**
  - Fetches the address of the report processor engaged with consensus operations.

- **`submitReport`:**
  - Oracle members use this to submit data reports for consensus hash calculations.

**Storage Variables**
- **`FrameConfig`:** Configures epochs per frame; critical for reporting organization.
- **`ConsensusFrame`:** Tracks reporting slots and processing deadlines within frames.
- **`ReportingState`:** Monitors recent report consensus activities.
- **`MemberState`:** Records individual member's reporting activities.
- **`ReportVariant`:** Stores variation in report data and their support.


## SUMMARY OF FILE: 2025-07-lido-finance/src/lib/utils/PausableUntil.sol
### PausableUntil Contract
The `PausableUntil` contract allows controlling the pause and resume state of a contract, using Solidity 0.8.24. The contract implements utility functions for pausing the contract until a specified time or indefinitely and resuming it.

### Functions
- **getResumeSinceTimestamp(): uint256**
  - Returns the resume timestamp.
  - Interface: `function getResumeSinceTimestamp() external view returns (uint256)`
  - Determines if paused infinitely, for a specific duration, or not paused.

- **isPaused(): bool**
  - Checks if the contract is paused.
  - Interface: `function isPaused() public view returns (bool)`
  - Compares current time with resume timestamp.
  
- **_resume()**
  - Resumes contract operation.
  - Internal function.
  - Checks if paused and updates resume timestamp to now.

- **_pauseFor(uint256 duration)**
  - Pauses contract for a specific duration.
  - Internal function.
  - Validates non-zero duration and sets resume timestamp.

- **_pauseUntil(uint256 pauseUntilInclusive)**
  - Pauses contract until a specific timestamp.
  - Internal function.
  - Ensures future timestamp and updates pause state.

- **_setPausedState(uint256 resumeSince)**
  - Sets paused state with resume timestamp.
  - Emits `Paused` event.

- **_checkPaused()**
  - Throws if not paused.

- **_checkResumed()**
  - Throws if paused.

### Storage Variables
- **RESUME_SINCE_TIMESTAMP_POSITION**: Stores the timestamp for resuming, positions via `keccak256`.
- **PAUSE_INFINITELY**: Special constant for indefinite pause, holds max uint256 value.


## SUMMARY OF FILE: 2025-07-lido-finance/src/lib/utils/Versioned.sol
## Versioned Contract Summary
The `Versioned` contract ensures controlled versioning of smart contracts. It leverages the `UnstructuredStorage` library to manage contract storage versioning. The contract tracks its version using a fixed storage slot and provides mechanisms to initialize and upgrade the version safely.

### Contract Definition
`contract Versioned` is a contract utilizing unstructured storage for managing contract versions and controlling version upgrades.

### Storage Variables
- **`bytes32 internal constant CONTRACT_VERSION_POSITION`**: Holds the storage position for the contract version with a specific hash to prevent over-initialization.
- **`uint256 internal constant PETRIFIED_VERSION_MARK`**: Represents a locked version state at maximum uint256 value to signal a non-initialized state.

### Functions and Interfaces
- **`constructor`**: Protects the implementation storage by locking its version with `_CONTRACT_VERSION_POSITION.setStorageUint256(PETRIFIED_VERSION_MARK);`.

- **`getContractVersion() public view returns (uint256)`**: Returns the current contract version by fetching it from the storage slot.

- **`_initializeContractVersionTo(uint256 version) internal`**: Called during initialization to set the contract version. Ensures no previous version is set.

- **`_updateContractVersion(uint256 newVersion) internal`**: Updates contract to new version ensuring an increment by one to prevent incorrect upgrades.

- **`_checkContractVersion(uint256 version) internal view`**: Confirms current version matches expected value to avoid unexpected behavior during operations.

- **`_setContractVersion(uint256 version) private`**: Privately updates the version storage and emits a `ContractVersionSet` event.


## SUMMARY OF FILE: 2025-07-lido-finance/src/VettedGate.sol
### Contract: `VettedGate`
The `VettedGate` contract is designed to manage eligible members through a Merkle Tree, node operator management, and an optional referral program. It is derived from multiple base contracts including access control, pausable functionalities, and asset recovery. The contract includes several roles, such as pause, resume, recoverer, set tree, and referral season management roles.

#### Storage Variables
- **MODULE**: (`ICSModule`) - Immutable address of the Staking Module.
- **ACCOUNTING**: (`ICSAccounting`) - Immutable address of the CS Accounting.
- **curveId**: (`uint256`) - Id for the bond curve applicable to eligible members.
- **treeRoot**: (`bytes32`) - Root hash of the eligible members Merkle Tree.
- **treeCid**: (`string`) - CID for the Merkle Tree.
- **isReferralProgramSeasonActive**: (`bool`) - Indicator for active referral season.
- **referralProgramSeasonNumber**: (`uint256`) - Counter for referral program seasons.
- **referralCurveId**: (`uint256`) - Bond curve ID for referral program.
- **referralsThreshold**: (`uint256`) - Referrals needed to claim bond curve.
- **_consumedAddresses**: (`mapping`) - Internal track of consumed addresses in program.

#### Functions
- **constructor(address module)** - Initializes the contract by setting the staking module and accounting module addresses.
- **initialize(uint256 _curveId, bytes32 _treeRoot, string calldata _treeCid, address admin)** - Configures essential parameters like curve ID, tree root, tree CID, and admin access for the contract.
- **resume()** - Allows operation resume.
- **pauseFor(uint256 duration)** - Pauses operations for the specified duration.
- **startNewReferralProgramSeason(uint256 _referralCurveId, uint256 _referralsThreshold)** - Initiates a new referral season with specified parameters.
- **endCurrentReferralProgramSeason()** - Ends the active referral season.
- **addNodeOperatorETH(...)** - Adds a node operator utilizing ETH.
- **addNodeOperatorStETH(...)** - Adds a node operator utilizing stETH.
- **addNodeOperatorWstETH(...)** - Adds node operator using wrapped stETH.
- **claimBondCurve(uint256 nodeOperatorId, bytes32[] calldata proof)** - Claims bond curve for a node operator.
- **claimReferrerBondCurve(uint256 nodeOperatorId, bytes32[] calldata proof)** - Claims referral bond curve given conditions are met.
- **setTreeParams(bytes32 _treeRoot, string calldata _treeCid)** - Updates the tree root and CID for the referral mechanism.
- **getReferralsCount(address referrer)** - Fetches the count of referrals for a referrer.
- **getReferralsCount(address referrer, uint256 season)** - Fetches the count of referrals for a referrer for a specific season.
- **getInitializedVersion()** - Fetches initialized version.
- **isReferrerConsumed(address referrer)** - Checks if the referrer is consumed.
- **isConsumed(address member)** - Checks if a member has consumed their eligibility.
- **verifyProof(address member, bytes32[] calldata proof)** - Verifies the Merkle Proof for a member.
- **hashLeaf(address member)** - Hashes a member's address as a Merkle Tree leaf.
- **_consume(bytes32[] calldata proof)** - Internally consumes a member's eligibility, ensuring it has not been consumed before.
- **_setTreeParams(bytes32 _treeRoot, string calldata _treeCid)** - Internal setting of tree parameters ensuring uniqueness and validity.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSFeeDistributor.sol
### CSFeeDistributor Contract

This Solidity contract, `CSFeeDistributor`, handles the distribution of fees in the form of stETH shares. It inherits from `ICSFeeDistributor`, `AccessControlEnumerableUpgradeable`, and `AssetRecoverer`. The contract uses a Merkle tree mechanism to verify distributions and ensure that shares are correctly allocated. Key storage variables include immutable addresses for `STETH`, `ACCOUNTING`, `ORACLE`, as well as various mappings and counters for distributed shares and historical data.

**constructor(address stETH, address accounting, address oracle)**
Initializes the contract with the addresses of `stETH`, `ACCOUNTING`, and `ORACLE`. These addresses cannot be zero and are stored as immutable variables. Initializers are disabled after setting these values.

**initialize(address admin, address _rebateRecipient)**
Sets the rebate recipient and grants the admin role. It initializes the access control mechanism and is callable only once as it's defined as a reinitializer.

**finalizeUpgradeV2(address _rebateRecipient)**
Sets a new rebate recipient during a contract upgrade.

**setRebateRecipient(address _rebateRecipient)**
Allows the admin to update the rebate recipient address.

**distributeFees(uint256 nodeOperatorId, uint256 cumulativeFeeShares, bytes32[] calldata proof)**
Distributes fees to node operators after verifying a Merkle proof, ensuring that claimable shares are available.

**processOracleReport(bytes32 _treeRoot, string calldata _treeCid, string calldata _logCid, uint256 distributed, uint256 rebate, uint256 refSlot)**
Processes a report from the oracle, updating the Merkle tree root, data history, and transferring shares as required.

**recoverERC20(address token, uint256 amount)**
Implements the ERC20 recovery mechanism for authorized recoverers, barring the recovery of stETH.

**getInitializedVersion()**
Returns the version of the initialization process.

**pendingSharesToDistribute()**
Calculates and returns the pending stETH shares that are distributable.

**getHistoricalDistributionData(uint256 index)**
Provides access to historical distribution data at a specified index.

**getFeesToDistribute(uint256 nodeOperatorId, uint256 cumulativeFeeShares, bytes32[] calldata proof)**
Calculates shares to be distributed after verifying the Merkle proof.

**hashLeaf(uint256 nodeOperatorId, uint256 shares)**
Hashes and returns the Merkle leaf node using the operator ID and their corresponding shares.

*Storage Variables and Their Descriptions:*
- `RECOVERER_ROLE`: A byte value representing the specific role for recoverers; immutable.
- `STETH`: The stETH token contract address; immutable.
- `ACCOUNTING`: The accounting contract address; immutable.
- `ORACLE`: The oracle contract address; immutable.
- `treeRoot`: Stores the latest Merkle tree root.
- `treeCid`, `logCid`: CIDs for the published Merkle tree and the log file for the last report, respectively.
- `distributedShares`: Mapping of nodeOperatorId to distributed stETH shares.
- `totalClaimableShares`: Total shares available for distribution.
- `_distributionDataHistory`: Historical distribution data storage.
- `distributionDataHistoryCount`: Counter for distribution data records.
- `rebateRecipient`: Address to receive the rebate.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSEjector.sol
### CSEjector Contract Summary
The `CSEjector` contract is a component of a larger Ethereum-based staking system, designed to handle the ejection of validators under certain conditions. It extends `ICSEjector`, `ExitTypes`, `AccessControlEnumerable`, `PausableUntil`, and `AssetRecoverer`, integrating roles for pausing/resuming actions and recovering assets.

### Storage Variables
- `bytes32 public constant PAUSE_ROLE;` - Role for pausing functions.
- `bytes32 public constant RESUME_ROLE;` - Role for resuming functions.
- `bytes32 public constant RECOVERER_ROLE;` - Role for recovering assets.
- `uint256 public immutable STAKING_MODULE_ID;` - Identifier for the staking module used.
- `ICSModule public immutable MODULE;` - Reference to the staking module implementation.
- `address public immutable STRIKES;` - Address associated with striking validators.

### Constructor
Instantiates the contract, setting module, strikes address, and staking module ID. Also assigns default admin role.

```solidity
constructor(address module, address strikes, uint256 stakingModuleId, address admin)
```

### Function Summaries
- **`resume()`**
Allows resuming functions if called by an address with `RESUME_ROLE`. 

```solidity
function resume() external onlyRole(RESUME_ROLE)
```


- **`pauseFor(uint256 duration)`**
Pauses contract functionalities for a specified duration, requires `PAUSE_ROLE` permissions.

```solidity
function pauseFor(uint256 duration) external onlyRole(PAUSE_ROLE)
```


- **`voluntaryEject(uint256 nodeOperatorId, uint256 startFrom, uint256 keysCount, address refundRecipient)`**
Facilitates the voluntary ejection of validator keys operated by the node operator, verifying ownership and key status.

```solidity
function voluntaryEject(uint256 nodeOperatorId, uint256 startFrom, uint256 keysCount, address refundRecipient) external payable
```


- **`voluntaryEjectByArray(uint256 nodeOperatorId, uint256[] calldata keyIndices, address refundRecipient)`**
Provides validator ejection for non-sequential keys, executing withdrawals via `triggerableWithdrawalsGateway`.

```solidity
function voluntaryEjectByArray(uint256 nodeOperatorId, uint256[] calldata keyIndices, address refundRecipient) external payable
```


- **`ejectBadPerformer(uint256 nodeOperatorId, uint256 keyIndex, address refundRecipient)`**
Allows the ejection of underperforming validators by the `STRIKES` role, ensuring keys meet criteria before withdrawal.

```solidity
function ejectBadPerformer(uint256 nodeOperatorId, uint256 keyIndex, address refundRecipient) external payable
```


- **`triggerableWithdrawalsGateway()`**
Fetches the current `ITriggerableWithdrawalsGateway` contract from the `MODULE` for handling validator exits.

```solidity
function triggerableWithdrawalsGateway() public view returns (ITriggerableWithdrawalsGateway)
```


- **`_onlyNodeOperatorOwner(uint256 nodeOperatorId)`**
Internal function ensuring the caller is the node operator's owner.

```solidity
function _onlyNodeOperatorOwner(uint256 nodeOperatorId) internal view
```


- **`_onlyRecoverer()`**
Ensures the caller holds the `RECOVERER_ROLE`, overriding a parent class function.

```solidity
function _onlyRecoverer() internal view override
```


## SUMMARY OF FILE: 2025-07-lido-finance/src/PermissionlessGate.sol
### `PermissionlessGate`
The `PermissionlessGate` contract is designed to allow unrestricted addition of new Node Operators (NOs). It inherits from `AccessControlEnumerable` for role-based access control, `AssetRecoverer` for asset recovery capabilities, and implements `IPermissionlessGate`. The core intent is to facilitate node operator creation without limitations.

### Storage Variables
- **`RECOVERER_ROLE`**: `bytes32 public constant` for the recoverer role, enabling permissions tied to asset recovery operations.
- **`CURVE_ID`**: `uint256 public immutable`, storing the default bond curve ID retrieved from the accounting module.
- **`MODULE`**: `ICSModule public immutable`, denotes the address of the staking module involved with node operations.

### Constructor
- **`constructor(address module, address admin)`**: Initializes the contract, storing references to the module and admin. Checks for zero addresses and assigns roles.

### Functions
- **`addNodeOperatorETH`**
  ```solidity
  function addNodeOperatorETH(uint256 keysCount, bytes calldata publicKeys, bytes calldata signatures, NodeOperatorManagementProperties calldata managementProperties, address referrer) external payable returns (uint256 nodeOperatorId)
  ```
  Adds a node operator using ETH. Performs node operator creation, followed by key addition.
  
- **`addNodeOperatorStETH`**
  ```solidity
  function addNodeOperatorStETH(uint256 keysCount, bytes calldata publicKeys, bytes calldata signatures, NodeOperatorManagementProperties calldata managementProperties, ICSAccounting.PermitInput calldata permit, address referrer) external returns (uint256 nodeOperatorId)
  ```
  Similar to `addNodeOperatorETH` but uses stETH for the process.
  
- **`addNodeOperatorWstETH`**
  ```solidity
  function addNodeOperatorWstETH(uint256 keysCount, bytes calldata publicKeys, bytes calldata signatures, NodeOperatorManagementProperties calldata managementProperties, ICSAccounting.PermitInput calldata permit, address referrer) external returns (uint256 nodeOperatorId)
  ```
  Operates like the above functions but involves the use of wstETH.

- **`_onlyRecoverer`**
  ```solidity
  function _onlyRecoverer() internal view override
  ```
  Ensures only an account granted the `RECOVERER_ROLE` can access the asset recovery functionality.


## Main List of Files in Project

script/DeployBase.s.sol
script/DeployCSVerifierElectra.s.sol
script/DeployHolesky.s.sol
script/DeployHoodi.s.sol
script/DeployImplementationsBase.s.sol
script/DeployImplementationsHolesky.s.sol
script/DeployImplementationsHoodi.s.sol
script/DeployImplementationsMainnet.s.sol
script/DeployLocalDevNet.s.sol
script/DeployMainnet.s.sol
script/constants/GIndices.sol
script/fork-helpers/Common.sol
script/fork-helpers/NodeOperators.s.sol
script/fork-helpers/PauseResume.s.sol
script/fork-helpers/SimulateVote.s.sol
script/utils/Common.sol
script/utils/Dummy.sol
script/utils/Json.sol
src/CSAccounting.sol
src/CSEjector.sol
src/CSExitPenalties.sol
src/CSFeeDistributor.sol
src/CSFeeOracle.sol
src/CSModule.sol
src/CSParametersRegistry.sol
src/CSStrikes.sol
src/CSVerifier.sol
src/PermissionlessGate.sol
src/VettedGate.sol
src/VettedGateFactory.sol
src/abstract/AssetRecoverer.sol
src/abstract/CSBondCore.sol
src/abstract/CSBondCurve.sol
src/abstract/CSBondLock.sol
src/abstract/ExitTypes.sol
src/interfaces/IACL.sol
src/interfaces/IBurner.sol
src/interfaces/ICSAccounting.sol
src/interfaces/ICSBondCore.sol
src/interfaces/ICSBondCurve.sol
src/interfaces/ICSBondLock.sol
src/interfaces/ICSEjector.sol
src/interfaces/ICSExitPenalties.sol
src/interfaces/ICSFeeDistributor.sol
src/interfaces/ICSFeeOracle.sol
src/interfaces/ICSModule.sol
src/interfaces/ICSParametersRegistry.sol
src/interfaces/ICSStrikes.sol
src/interfaces/ICSVerifier.sol
src/interfaces/IExitTypes.sol
src/interfaces/IGateSeal.sol
src/interfaces/IGateSealFactory.sol
src/interfaces/IKernel.sol
src/interfaces/ILido.sol
src/interfaces/ILidoLocator.sol
src/interfaces/IPermissionlessGate.sol
src/interfaces/IStETH.sol
src/interfaces/IStakingModule.sol
src/interfaces/IStakingRouter.sol
src/interfaces/ITriggerableWithdrawalsGateway.sol
src/interfaces/IVEBO.sol
src/interfaces/IVettedGate.sol
src/interfaces/IVettedGateFactory.sol
src/interfaces/IWithdrawalQueue.sol
src/interfaces/IWithdrawalVault.sol
src/interfaces/IWstETH.sol
src/lib/AssetRecovererLib.sol
src/lib/GIndex.sol
src/lib/NOAddresses.sol
src/lib/QueueLib.sol
src/lib/SSZ.sol
src/lib/SigningKeys.sol
src/lib/TransientUintUintMapLib.sol
src/lib/Types.sol
src/lib/UnstructuredStorage.sol
src/lib/ValidatorCountsReport.sol
src/lib/base-oracle/BaseOracle.sol
src/lib/base-oracle/HashConsensus.sol
src/lib/base-oracle/interfaces/IConsensusContract.sol
src/lib/base-oracle/interfaces/IReportAsyncProcessor.sol
src/lib/proxy/OssifiableProxy.sol
src/lib/utils/PausableUntil.sol
src/lib/utils/Versioned.sol
test/AssetRecoverer.t.sol
test/BaseOracle.t.sol
test/CSAccounting.t.sol
test/CSBondCore.t.sol
test/CSBondCurve.t.sol
test/CSBondLock.t.sol
test/CSEjector.t.sol
test/CSExitPenalties.t.sol
test/CSFeeDistributor.t.sol
test/CSFeeOracle.t.sol
test/CSModule.t.sol
test/CSParametersRegistry.t.sol
test/CSStrikes.t.sol
test/CSVerifier.t.sol
test/CSVerifierHistorical.t.sol
test/CSVerifierHistoricalCrossForks.t.sol
test/GIndex.t.sol
test/HashConsensus.t.sol
test/OssifiableProxy.t.sol
test/PausableUntil.t.sol
test/PermissionlessGate.t.sol
test/PoC.t.sol
test/QueueLib.t.sol
test/SSZ.t.sol
test/SigningKeys.t.sol
test/TransientUintUintMapLib.t.sol
test/UnstructuredStorage.t.sol
test/ValidatorCountsReport.t.sol
test/Versioned.t.sol
test/VettedGate.t.sol
test/VettedGateFactory.t.sol
test/fork/deployment/PostDeployment.t.sol
test/fork/integration/ClaimInTokens.t.sol
test/fork/integration/CreateAndDeposit.sol
test/fork/integration/Ejection.t.sol
test/fork/integration/GateSeal.t.sol
test/fork/integration/Misc.t.sol
test/fork/integration/NoManagement.t.sol
test/fork/integration/Oracle.t.sol
test/fork/integration/Penalty.t.sol
test/fork/integration/RecoverTokens.t.sol
test/fork/integration/StakingRouter.t.sol
test/fork/integration/misc/Invariants.t.sol
test/fork/integration/misc/ProxyUpgrades.sol
test/fork/vote-upgrade/V2Upgrade.sol
test/helpers/ERCTestable.sol
test/helpers/Fixtures.sol
test/helpers/InvariantAsserts.sol
test/helpers/MerkleTree.sol
test/helpers/MerkleTree.t.sol
test/helpers/Permit.sol
test/helpers/Utilities.sol
test/helpers/mocks/BurnerMock.sol
test/helpers/mocks/CSAccountingMock.sol
test/helpers/mocks/CSMMock.sol
test/helpers/mocks/CSParametersRegistryMock.sol
test/helpers/mocks/CSStrikesMock.sol
test/helpers/mocks/ConsensusContractMock.sol
test/helpers/mocks/DistributorMock.sol
test/helpers/mocks/EjectorMock.sol
test/helpers/mocks/ExitPenaltiesMock.sol
test/helpers/mocks/LidoLocatorMock.sol
test/helpers/mocks/LidoMock.sol
test/helpers/mocks/ReportProcessorMock.sol
test/helpers/mocks/StETHMock.sol
test/helpers/mocks/Stub.sol
test/helpers/mocks/TWGMock.sol
test/helpers/mocks/WithdrawalQueueMock.sol
test/helpers/mocks/WstETHMock.sol

### lido-docs.md

Title: Intro | Lido Docs

URL Source: https://docs.lido.fi/staking-modules/csm/intro/

Markdown Content:
tip

If you're looking for a practical guide to run CSM on your setup, please follow the CSM guide [here](https://docs.lido.fi/run-on-lido/csm/).

info

Terms "validator", "key", "validator key", and "deposit data" have the same meaning within the document.

∑ TL;DR[​](https://docs.lido.fi/staking-modules/csm/intro/#-tldr "Direct link to ∑ TL;DR")
------------------------------------------------------------------------------------------

Community Staking Module (CSM) is a permissionless staking module aimed at attracting community stakers to participate in Lido on Ethereum protocol as Node Operators. The only requirement to join CSM as a Node Operator is to be able to run validators (according to the Lido on Ethereum policies) and supply a [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond). The stake is allocated to the validator keys in the order in which the keys are provided, given the keys are valid. The [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond) is not directly associated with the actual validator's stake but instead treated as a security collateral. The [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond) is a characteristic of a Node Operator; hence, it is collateral for all Node Operator's validators. This allows for the [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond) reduction. The more validators the Node Operator has, the less the [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond) for one validator. Node Operators get their rewards from the [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond) rebase and from the Node Operator's portion of the staking rewards. Node Operator's portion of the staking rewards is socialized (averaged) if the validators perform above the threshold. Accumulated CL penalties resulting in a balance reduction below the deposit balance and stolen EL rewards are confiscated from the Node Operator's [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond). Node Operators should perform validator exits upon protocol request or can exit voluntarily.

📓 Glossary[​](https://docs.lido.fi/staking-modules/csm/intro/#-glossary "Direct link to 📓 Glossary")
------------------------------------------------------------------------------------------------------

*   The[**staking router**](https://docs.lido.fi/contracts/staking-router)(SR) is a smart contract within the Lido on Ethereum protocol that facilitates stake allocation and rewards distribution across different modules;
*   A**staking module**(SM) is a smart contract or a set of smart contracts connected to the staking router, which:
    *   maintains the underlying operator and validator sets,
    *   is responsible for on/off-boarding operators,
    *   maintains validator deposits, withdrawals, and exits,
    *   maintains fee structure and distribution for the module and participants, etc,
    *   conforms to the IStakingModule interface;

*   **[Bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond)** - a security collateral that Node Operators must submit before uploading validator keys into CSM. This collateral covers possible losses caused by inappropriate actions on the Node Operator's side. Once the validator exits from the Beacon chain and all losses that occurred are covered, the collateral can be claimed or reused to upload new validator keys.
*   The **Lido DAO** is a Decentralized Autonomous Organization that decides on the critical parameters of controlled liquid staking protocols through the voting power of governance token (LDO).
*   A**Node Operator**(NO)is a person or entity that runs validators;
*   [`Lido`](https://docs.lido.fi/contracts/lido) is a core contract of the Lido on Ethereum protocol that stores the protocol state, accepts user submissions, and includes the stETH token;
*   **stETH**is an ERC-20 token minted by[`Lido`](https://etherscan.io/address/0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84) smart contract and representing a share of the [`totalPooledEther`](https://docs.lido.fi/contracts/lido#rebase);
*   **Deposit data**refers to a structure consisting of the validator’s public key and deposit signature submitted to`DepositContract`. This term can also be referred to as `keys` in the text. Validator private keys are created, stored, and managed by Node Operators exclusively;
*   `DepositContract` is the official Ethereum deposit contract for validator deposits;
*   `DepositSecurityModule` or [**DSM**](https://docs.lido.fi/guides/deposit-security-manual) is a set of smart contract and off-chain parts mitigating the [deposit front-run vulnerability](https://docs.lido.fi/guides/deposit-security-manual#the-vulnerability);
*   A validator is considered to be[**“unbonded”**](https://docs.lido.fi/staking-modules/csm/join-csm#unbonded-validators)when the current Node Operator [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond) is not sufficient to cover this validator;
*   A validator is considered to be ["**stuck**"](https://docs.lido.fi/contracts/staking-router#exited-and-stuck-validators) if it has not been exited timely following an exit signal from the protocol;
*   The **Curated module** is the first Lido staking module previously referred to as [Node Operators Registry](https://docs.lido.fi/contracts/node-operators-registry);
*   **Easy Track** is a suite of smart contracts and an alternative veto-based voting model that streamlines routine DAO operations;
*   [**Accounting Oracle**](https://docs.lido.fi/contracts/accounting-oracle) is a contract which collects information submitted by the off-chain oracles about state of the Lido-participating validators and their balances, the amount of funds accumulated on the protocol vaults (i.e., withdrawal and execution layer rewards vaults), the number of exited and stuck validators, the number of withdrawal requests the protocol can process and distributes node-operator rewards and performs `stETH` token rebase;
*   [**VEBO**](https://docs.lido.fi/contracts/validators-exit-bus-oracle) or Validators Exit Bus Oracle is a contract that implements an on-chain "source of truth" message bus between the protocol's off-chain oracle and off-chain observers, with the main goal of delivering validator exit requests to the Lido-participating Node Operators.

🌎 General info[​](https://docs.lido.fi/staking-modules/csm/intro/#-general-info "Direct link to 🌎 General info")
------------------------------------------------------------------------------------------------------------------

CSM is a staking module offering permissionless entry with a [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond). This module aims to become a clear pathway for independent [community stakers](https://research.lido.fi/t/lido-on-ethereum-community-validation-manifesto/3331#lido-on-ethereum-community-validation-manifesto-1) (solo stakers or home stakers) to enter the Lido on Ethereum protocol (LoE) node operator set. The [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond) requirement is an essential security and alignment tool that makes permissionless entry possible without compromising the security or reliability of the underlying staking protocol (LoE).

🤓 Module specifics[​](https://docs.lido.fi/staking-modules/csm/intro/#-module-specifics "Direct link to 🤓 Module specifics")
------------------------------------------------------------------------------------------------------------------------------

All staking modules should conform to the same [IStakingModule](https://github.com/lidofinance/core/blob/aada42242e893ea2726e629c135cd375d30575fc/contracts/0.8.9/interfaces/IStakingModule.sol) interface. That inevitably results in modules having a lot of common or similar components and logic. CSM is no exception here. For example, key storage components are based on the existing [Curated module](https://docs.lido.fi/contracts/node-operators-registry). However, several aspects are different and worth a separate mention.

### Exited and Withdrawn[​](https://docs.lido.fi/staking-modules/csm/intro/#exited-and-withdrawn "Direct link to Exited and Withdrawn")

The [Curated module](https://docs.lido.fi/contracts/node-operators-registry) uses the "exited" statuses of the validator (both [Slashed and Exited](https://notes.ethereum.org/7CFxjwMgQSWOHIxLgJP2Bw#44-Step-4-Slashed-and-Exited) and [Unslashed and Exited](https://notes.ethereum.org/7CFxjwMgQSWOHIxLgJP2Bw#45-Step-5-Unslashed-and-Exited)) as the last meaningful status in accounting since, after this status, the validator is no longer responsible for any duties on the Beacon chain (except for the rare cases of the delayed sync committee participation). CSM, in turn, needs to know about each validator's exact withdrawal balance to decide on [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond) penalization. Hence, the module uses the "exited" counter reported by the accounting oracle only to return a correct number of "active" keys to the staking router and implements permissionless reporting methods to report the validator's withdrawal balance once the validator is [withdrawn](https://consensys.io/shanghai-capella-upgrade#:~:text=Finally%2C%20the%20withdrawable%20validator%20is%20subject%20to%20the%20same%2C%20automated%20%E2%80%9Csweep%E2%80%9D%20that%20processes%20partial%20withdrawals%2C%20and%20its%20balance%20is%20withdrawn).

### Stake distribution queue[​](https://docs.lido.fi/staking-modules/csm/intro/#stake-distribution-queue "Direct link to Stake distribution queue")

A Node Operator must supply a [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond) to upload a new validator key to CSM. It is reasonable to allocate a stake in an order similar to the [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond) submission order. For this purpose, a FIFO (first in, first out) [stake allocation queue](https://docs.lido.fi/staking-modules/csm/join-csm#stake-allocation-queue) is utilized. Once the Staking Router requests keys to make a deposit, the next `X` keys from the queue are returned, preserving the [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond) submit order.

### Alternative measures for "stuck" keys[​](https://docs.lido.fi/staking-modules/csm/intro/#alternative-measures-for-stuck-keys "Direct link to Alternative measures for \"stuck\" keys")

The presence of "stuck" keys for the Node Operator indicates a failure of the Node Operator to conform to the [Validator Exits SNOP](https://docs.lido.fi/guides/node-operators/general-overview#validator-exits-protocol-penalties-and-recovering). In this case, each module defines and is responsible for the application of remediative measures for the relevant protocol violation. CSM uses measures that are different from those of the Curated Module and the Simple DVT Module. The measures are described in the corresponding [section](https://docs.lido.fi/staking-modules/csm/validator-exits#protocol-initiated-exits).

info

Note: CSM does not apply any measures to "Delayed" validators mentioned in the [Lido exit policy](https://docs.lido.fi/guides/node-operators/general-overview#validator-exits-protocol-penalties-and-recovering).

### Node Operator structure[​](https://docs.lido.fi/staking-modules/csm/intro/#node-operator-structure "Direct link to Node Operator structure")

The Node Operator data structure in CSM is similar to that of the [Curated module](https://docs.lido.fi/contracts/node-operators-registry), with several minor differences:

*   The `name` property is omitted as redundant for the permissionless module;
*   The `rewardAddress` is used as a recipient of rewards and excess [bond](https://docs.lido.fi/staking-modules/csm/join-csm#bond) claims;
*   A new property, `managerAddress`, is introduced. The Node Operator should perform method calls from this address;
*   A new property, `totalWithdrawnKeys`, is introduced to count the total count of the withdrawn keys per Node Operator;
*   A new property, `depositableValidatorsCount`, is introduced to count the current deposit data eligible for deposits;
*   A new property, `enqueuedCount`, is introduced to keep track of the depositable keys that are in the queue. Also useful to determine depositable keys that are not in the queue at the moment;


## Additional Context

### Areas of Concern --> **WHERE TO FOCUS FOR BUGS**
- Bond accounting consistency
- No possibility to steal bond funds
- No protocol griefing

### Main Invariants
- Node Operators **cannot claim more rewards** than distributed in the Performance Oracle report.
- Node Operators **cannot delete deposited keys**.
- **Add Keys** operation ensures that after the keys addition, all of the Node Operator's keys are covered with the bond.



Title: 📝 Community Staking Module v2. Spec - HackMD

URL Source: https://hackmd.io/@lido/csm-v2-spec

Markdown Content:
![Image 1: csmv2_spec](https://hackmd.io/_uploads/rypGIgm6Jx.png)

Community Staking Module (CSM) v2 is an evolutionary step in the CSM development. This version aims to make CSM even more robust, flexible, and competitive. The main features of CSM v2 are described in a [separate document](https://hackmd.io/@lido/csm-v2-tech) that will be referred to later in the text. This document describes the overall CSM v2 architecture and changes made compared to the existing [CSM v1](https://github.com/lidofinance/lido-improvement-proposals/blob/develop/LIPS/lip-26.md).

> Terms validator, key, validator key, and deposit data meanings are the same within the document

*   [Project repo](https://github.com/lidofinance/community-staking-module)
*   Written in [Solidity 0.8.24](https://github.com/ethereum/solidity/tree/v0.8.24)
*   Developed in [Foundry](https://github.com/foundry-rs/foundry)

[](https://hackmd.io/@lido/csm-v2-spec#General-Architecture "General-Architecture")General Architecture
-------------------------------------------------------------------------------------------------------

![Image 2: image](https://hackmd.io/_uploads/B1p1nvvklx.png)

The scheme above depicts CSM's smart contracts architecture and changes made in CSM v2.

### [](https://hackmd.io/@lido/csm-v2-spec#Contracts "Contracts")Contracts

#### [](https://hackmd.io/@lido/csm-v2-spec#CSModulesol "CSModulesol")`CSModule.sol`

_CSM on the scheme_

Changed in v2

`CSModule.sol` is a core module contract conforming to the `IStakingModule` interface. It stores information about Node Operators and deposit data (DD). This contract is responsible for all interactions with the `StakingRouter`, namely, the DD queue management and some of the Node Operator's parameters. Node Operators manage their validator keys and other parameters they can modify through this contract.

**Changes in v2:**

*   Node Operator creation methods were replaced with a single permissioned method. Node Operators creation is now possible only through [Entry Gates or Extensions](https://hackmd.io/@lido/csm-v2-tech#Entry-Gates-and-Extensions10) contracts attached to `CSModule.sol` via `CREATE_NODE_OPERATOR_ROLE`;
*   Rewards claims and bond top-ups are moved to `CSAccounting.sol`;
*   The slashing reporting method is removed;
*   Node-Operator-type-related parameters moved to `CSParametersRegistry.sol`;
*   DD queue mechanism was reworked to allow for multiple [priority queues](https://hackmd.io/@lido/csm-v2-tech#Priority-Queues);
*   Public release mechanism was deprecated. Permissioned CSM is now possible with the use of the Vetted Gates without Permissionless Gate while setting a key limit for the corresponding Node Operator type;
*   Reset bond curve removed for cases of slashing and settled EL stealing penalty due to the introduction of the Node Operator types associated with the bond curve;

#### [](https://hackmd.io/@lido/csm-v2-spec#CSAccountingsol "CSAccountingsol")`CSAccounting.sol`

_Accounting in the scheme_

Changed in v2

`CSAccounting.sol` is a supplementary contract responsible for the management of bond, rewards, and penalties. It stores bond tokens as `stETH` shares, provides information about the bond required, and provides interfaces for the penalties. Node Operators claim rewards and top-up bonds using this contract.

**Changes in v2:**

*   User-facing methods for reward claims and bond top-ups are moved to `CSAccounting.sol` from `CSModule.sol`;
*   Public methods to get claimable bond amounts and rewards are added;

#### [](https://hackmd.io/@lido/csm-v2-spec#CSVerifiersol "CSVerifiersol")`CSVerifier.sol`

_Verifier on the scheme_

Changed in v2

`CSVerifier.sol` is a utility contract responsible for validating the CL data proofs using EIP-4788. It accepts proof of the validator withdrawals and reports these facts to the `CSModule.sol` if the proof is valid.

**Changes in v2:**

*   The slashing reporting method is removed;
*   Pause methods added;

#### [](https://hackmd.io/@lido/csm-v2-spec#CSEarlyAdoptionsol "CSEarlyAdoptionsol")`CSEarlyAdoption.sol`

_EarlyAdoption on the scheme_

Removed in v2

A contract is **removed** in CSM v2 and replaced with the instance of the `VettedGate.sol`.

#### [](https://hackmd.io/@lido/csm-v2-spec#CSFeeDistributorsol "CSFeeDistributorsol")`CSFeeDistributor.sol`

_FeeDistributor on the scheme_

Changed in v2

`CSFeeDistributor.sol` is a supplementary contract that stores non-claimed and non-distributed Node Operator rewards on its balance. This contract stores the latest root of a rewards distribution Merkle tree. It accepts calls from `CSAccounting.sol` with reward claim requests and stores data about already claimed rewards by the Node Operator. It receives non-distributed rewards from the `CSModule.sol` each time the `StakingRouter` mints the new portion of the module's rewards. This contract transfers excess rewards allocated by `StakingRouter` due to variable Node Operator reward share back to Lido treasury.

**Changes in v2:**

*   Added storage of the distribution history;
*   Added support of the variable Node Operator reward share and rebate transfer to Lido treasury;

#### [](https://hackmd.io/@lido/csm-v2-spec#CSFeeOraclesol "CSFeeOraclesol")`CSFeeOracle.sol`

_FeeOracle on the scheme_

Changed in v2

`CSFeeOracle.sol` is a utility contract responsible for the execution of the CSM Oracle report once the consensus is reached in the `HashConsensus.sol` contract, namely, transforming non-distributed rewards to non-claimed rewards stored on the `CSFeeDistributor.sol` and reporting the latest root of rewards distribution Merkle tree to the `CSFeeDistributor.sol`. Alongside rewards distribution, a contract manages strikes data delivery to the `CSStrikes.sol`. A contract is Inherited from the [`BaseOracle.sol`](https://github.com/lidofinance/core/blob/master/contracts/0.8.9/oracle/BaseOracle.sol) from Lido on Ethereum (LoE) core.

**Changes in v2:**

*   Added strikes reporting support;

#### [](https://hackmd.io/@lido/csm-v2-spec#HashConsensussol "HashConsensussol")`HashConsensus.sol`

_HashConsensus on the scheme_

`HashConsensus.sol` is a utility contract responsible for reaching a consensus between CSM Oracle members. Uses the standard code of the [`HashConsensus`](https://github.com/lidofinance/core/blob/master/contracts/0.8.9/oracle/HashConsensus.sol) contract from Lido on Ethereum (LoE) core.

#### [](https://hackmd.io/@lido/csm-v2-spec#CSParametersRegistrysol "CSParametersRegistrysol")`CSParametersRegistry.sol`

_ParametersRegistry on the scheme_

New in v2

`CSParametersRegistry.sol` is a utility contract that stores Node-Operator-type-related parameters fetched by the other smart contracts related to CSM. A contract requires a mandatory default value for all parameters to ensure consistency. The custom value is returned if it is set for a particular parameter. Otherwise, the default value is returned.

#### [](https://hackmd.io/@lido/csm-v2-spec#CSStrikessol "CSStrikessol")`CSStrikes.sol`

_StrikesRegistry on the scheme_

New in v2

`CSStrikes.sol` is a utility contract that stores information about strikes assigned to the CSM validators by CSM Performance Oracle. It has a permissionless method to prove that a particular validator should be ejected because the number of strikes is above the threshold for this validator. It calls `CSEjector.sol` to perform a strikes threshold check and eject the validator.

#### [](https://hackmd.io/@lido/csm-v2-spec#PermissionlessGatesol "PermissionlessGatesol")`PermissionlessGate.sol`

_PermissionlessGate on the scheme_

New in v2

`PermissionlessGate.sol` is a supplementary contract that enables permissionless Node Operator creation in `CSModule.sol`, serving as an entry point.

#### [](https://hackmd.io/@lido/csm-v2-spec#VettedGatesol "VettedGatesol")`VettedGate.sol`

_VettedGates on the scheme_

New in v2

`VettedGate.sol` is a supplementary contract that enables Node Operator creation for the vetted addresses, which serves as an entry point to `CSModule.sol`. Alongside Node Operator creation, a contract can assign a custom Node Operator type (bondCurveId) in `CSAccounting.sol`. Deployed using `VettedGateFactory.sol` to allow the addition of the new instances later without additional code security audits. The list of the vetted participants is upgradable for each instance of the `VettedGate.sol` individually.

#### [](https://hackmd.io/@lido/csm-v2-spec#CSEjectorsol "CSEjectorsol")`CSEjector.sol`

_Ejector on the scheme_

New in v2

`CSEjector.sol` is a supplementary contract responsible for interactions with EIP-7002-powered Lido Withdrawal credentials via `VEB`. Node Operators can voluntarily eject their validators. `CSStrikes.sol` uses `CSEjector.sol` to trigger exits for validators that have surpassed the strike threshold.

#### [](https://hackmd.io/@lido/csm-v2-spec#CSExitPenaltiessol "CSExitPenaltiessol")`CSExitPenalties.sol`

_ExitPenalties on the scheme_

New in v2

`CSExitPenalties.sol` is a supplementary contract responsible for processing and storing information about exit-related penalties, namely:

*   Delayed exit penalty;
*   Bad performance ejection penalty;
*   TE fee paid in case of a forced and involuntary exit.

#### [](https://hackmd.io/@lido/csm-v2-spec#EasyTrack "EasyTrack")`EasyTrack`

`EasyTrack` is a utility contract responsible for applying the reported EL stealing penalties. A part of the common [`EasyTrack`](https://github.com/lidofinance/easy-track) setup within Lido on Ethereum (LoE).

#### [](https://hackmd.io/@lido/csm-v2-spec#GateSeal "GateSeal")`GateSeal`

Changed in v2

`GateSeal` is a utility contract responsible for the one-time pause of the `CSModule.sol`, `CSAccounting.sol`, `CSFeeOracle.sol`, `VettedGate.sol`, `CSEjector.sol`, and `CSVerifier.sol` contracts to prevent possible module exploitation through zero-day vulnerabilities. Uses the [standard code](https://github.com/lidofinance/gate-seals) of the `GateSeal` contract from Lido on Ethereum (LoE).

The list of sealable contracts:

*   `CSModule.sol`
*   `CSAccounting.sol`
*   `CSFeeOracle.sol`
*   `CSVerifier.sol` (new)
*   `CSVettedGate.sol` (new)
*   `CSEjector.sol` (new)

**Changes in v2:**

*   ;

### [](https://hackmd.io/@lido/csm-v2-spec#Off-chain-tools "Off-chain-tools")Off-chain tools

#### [](https://hackmd.io/@lido/csm-v2-spec#CSM-Bot "CSM-Bot")`CSM Bot`

Changed in v2

`CSM Bot` is a daemon application responsible for monitoring and reporting the withdrawal events associated with the CSM validators. Also responsible for validator ejection invocation due to strikes.

**Changes in v2:**

*   Slashing reporting removed;
*   Validator ejection invocation added;

#### [](https://hackmd.io/@lido/csm-v2-spec#EL-stealing-detector "EL-stealing-detector")`EL stealing detector`

`EL stealing detector` is a daemon application or EOA or Committee Multisig responsible for detecting and reporting the EL stealing facts by the CSM validators. Assigned to CSM Committee Multisig in the existing version of CSM and assumed to be re-assigned to the automated bot at the acceptable level of MEV monitoring software maturity to avoid false-positive activations.

#### [](https://hackmd.io/@lido/csm-v2-spec#CSM-Oracle "CSM-Oracle")`CSM Oracle`

Changed in v2

`CSM Oracle` (also known as CSM Performance Oracle) is a module in the common Lido on Ethereum (LoE) Oracle set. It is operated by the existing Oracles set alongside [Accounting Oracle](https://docs.lido.fi/contracts/accounting-oracle) and [Validator Exit Bus Oracle](https://docs.lido.fi/contracts/validators-exit-bus-oracle). It is responsible for calculating the CSM Node Operators' reward distribution and strike assignment based on their performance on the CL.

**Changes in v2:**

*   Added strikes calculation;
*   Performance calculation algorithm now accounts for block proposals and sync committee participation;
*   Added support of the variable Node Operator reward share, and performance threshold;
*   Added support for the configurable performance coefficients (Attestation, proposals, and sync committee eff);

[](https://hackmd.io/@lido/csm-v2-spec#Main-flows "Main-flows")Main flows
-------------------------------------------------------------------------

### [](https://hackmd.io/@lido/csm-v2-spec#Create-Node-Operator "Create-Node-Operator")Create Node Operator

Changed in v2

*   Node Operator creation is now done via Gates;

![Image 3: image](https://hackmd.io/_uploads/HJ8Agb9qkl.png)

![Image 4: image](https://hackmd.io/_uploads/rJkgZ-q5ke.png)

Node Operator creation uses either `PermisssionlessGate.sol` or `VettedGate.sol` or future [Entry Gates or Extensions](https://hackmd.io/@lido/csm-v2-tech#Gates-and-Extensions10) contracts attached to `CSModule.sol` via `CREATE_NODE_OPERATOR_ROLE`. Entry Gates (Extensions) should ensure that at least one deposit data and the corresponding bond amount are required to create a Node Operator to avoid flooding the module with empty Node Operators. Before Node Operator creation, an amount of bond needed should be fetched from the `CSAccounting.sol`. Depending on the selected token, this amount should be:

*   attached as a payment to the transaction (ETH);
*   approved to be transferred by `CSAccounting.sol` (stETH, wstETH);
*   included in permit data approving transfers by `CSAccounting.sol` (stETH, wstETH);

### [](https://hackmd.io/@lido/csm-v2-spec#Upload-deposit-data "Upload-deposit-data")Upload deposit data

![Image 5: image](https://hackmd.io/_uploads/SkeDR15cJg.png)

Node Operators can upload deposit data after creation. Before uploading, the required bond amount should be fetched from `CSAccounting.sol`, and corresponding approvals, permits, or direct attachments as a payment should be performed like the Node Operator creation described above.

### [](https://hackmd.io/@lido/csm-v2-spec#Delete-deposit-data "Delete-deposit-data")Delete deposit data

![Image 6: image](https://hackmd.io/_uploads/Byfu0yc9kl.png)

If deposit data has not been deposited yet, the Node Operator can request its deletion from `CSModule.sol`. `CSModule.sol` validates that deposit data has not yet been deposited. If deletion is possible, `CSAccounting.sol` confiscates the `keyRemovalCharge` from the Node Operator's bond.

### [](https://hackmd.io/@lido/csm-v2-spec#Top-up-bond-without-deposit-data-upload "Top-up-bond-without-deposit-data-upload")Top-up bond without deposit data upload

Changed in v2

*   Top-up bond is now done via `CSAccounting.sol`;

![Image 7: image](https://hackmd.io/_uploads/B1G5AJ5cke.png)

CSM Node Operators can top-up bond balance at any time to have an excess bond in advance or compensate for the penalties. Top-up is done via `CSAccounting.sol`. Once funds are transferred to `CSAccounting.sol`, `CSModule.sol` is informed about the bond amount change and corresponding changes in the depositable keys are performed regarding the Node Operator to account for the change in the bond balance.

### [](https://hackmd.io/@lido/csm-v2-spec#Stake-allocation "Stake-allocation")Stake allocation

Changed in v2

*   Priority queues added;

CSM utilizes the FIFO queue to determine the next portion of the validator keys to be deposited. Changes to the deposit queue in CSM v2 are described in the [features doc](https://hackmd.io/@lido/csm-v2-tech#Priority-Queues).

#### [](https://hackmd.io/@lido/csm-v2-spec#Basic-flow "Basic-flow")Basic flow

![Image 8: image](https://hackmd.io/_uploads/r1xAiRkq91e.png)

Once uploaded, deposit data is placed in the queue with respect to the Priority queue parameters for the given Node Operator. To allocate stake to the CSM Node Operators, `StakingRouter` calls the `obtainDepositData(depositsCount)` method to get the next `depositsCount` depositable keys from the keys queue.

#### [](https://hackmd.io/@lido/csm-v2-spec#Invalid-keys "Invalid-keys")Invalid keys

![Image 9: image](https://hackmd.io/_uploads/BJ02Ay9c1x.png)

Due to the [optimistic vetting approach](https://hackmd.io/gGRgZ0yeTnm-9SSFuHrXwg#Deposit-data-validation-and-invalidation-aka-vetting-and-unvetting), invalid keys might be present in the queue. [DSM](https://docs.lido.fi/contracts/deposit-security-module) is responsible for detecting and reporting invalid keys through `StakingRouter`. If invalid keys are detected, a call to `decreaseOperatorVettedKeys` is expected from `StakingRouter` to `CSModule.sol`.

### [](https://hackmd.io/@lido/csm-v2-spec#Rewards-distribution "Rewards-distribution")Rewards distribution

Changed in v2

*   Variable fee and rebate to treasury added;

![Image 10: image](https://hackmd.io/_uploads/S12CCJc5Je.png)

`StakingRouter` mint rewards for CSM Node Operators on each report of the [`AccountingOracle`](https://docs.lido.fi/contracts/accounting-oracle). `CSModule.sol` transfers minted rewards to the `CSFeeDistributor.sol`. Once the report slot is reached for the following CSM Oracle report, the rewards distribution tree is [calculated](https://hackmd.io/@lido/csm-v2-tech#Updated-CSM-Performance-Oracle-metric) by each Oracle member. After reaching the quorum, a new Merkle tree root is submitted to the `CSFeeDistributor.sol`, the corresponding portion of the rewards is transferred from the non-distributed to the non-claimed state, and excess rewards transferred by `StakingRouter` due to variable Node Operator reward share are returned to Lido treasury.

### [](https://hackmd.io/@lido/csm-v2-spec#Rewards-claim "Rewards-claim")Rewards claim

Changed in v2

*   Rewards claim is now done via `CSAccounting.sol`;

![Image 11: image](https://hackmd.io/_uploads/SJfg1lc9ke.png)

Total rewards for the CSM Node Operators are comprised of [bond rewards and staking fees](https://docs.lido.fi/staking-modules/csm/rewards). To claim the total rewards, the Node Operator needs to bring proof of the latest `cumulativeFeeShares` in the rewards tree. With that proof `CSAccounting.sol` pulls the Node Operator's portion of the staking fees from the `CSFeeDistributor.sol` and combines it with the Node Operator's bond. After that, all bond funds exceeding the bond required for the currently active keys are available for claim.

Node Operator can transfer staking rewards to the bond without transferring it to the reward address by passing `0` as the amount requested for the claim.

If there are no new rewards to pull from the `CSFeeDistributor.sol` Node Operator can still claim excess bond using the same flow.

### [](https://hackmd.io/@lido/csm-v2-spec#EL-stealing-penalty "EL-stealing-penalty")EL stealing penalty

Changed in v2

*   Additional fine is now configurable for the Node Operator type;
*   Reset bond curve removed due to the introduction of the Node Operator types associated with the bond curve;

![Image 12: image](https://hackmd.io/_uploads/r1y9ZZaT1g.png)

If the Node Operator commits EL rewards stealing (or violates the [Lido on Ethereum Block Proposer Rewards Policy](https://snapshot.box/#/s:lido-snapshot.eth/proposal/0x7ac2431dc0eddcad4a02ba220a19f451ab6b064a0eaef961ed386dc573722a7f)), this fact and the stolen amount are reported to the `CSModule.sol` by the EL stealing detector actor. The corresponding amount of the bond funds (stolen amount + fixed fee) is locked by the `CSAccounting.sol`. Node Operator can compensate for the stolen funds and fixed fee voluntarily. If the Node Operator does not compensate for the stolen funds, `EasyTrack` is started to confirm the penalty application. Once enacted, a penalty is applied (locked funds are burned).

### [](https://hackmd.io/@lido/csm-v2-spec#Validator-ejection-due-to-strikes "Validator-ejection-due-to-strikes")Validator ejection due to strikes

New in v2

![Image 13: image](https://hackmd.io/_uploads/SJmZ6wPJel.png)

If the validator has reached the strikes threshold (`actual strikes >= threshold`) `CSM Bot` will initiate validator ejection using a permissionless method. `CSStrikes.sol` validates the proof and makes a call to `CSEjector.sol` if the number of strikes >= threshold. `CSEjector.sol` notify `VEBO` about the required validator ejection. Corresponding penalties are recorded in `CSExitPenalties.sol`.

### [](https://hackmd.io/@lido/csm-v2-spec#Voluntary-validator-ejection "Voluntary-validator-ejection")Voluntary validator ejection

New in v2

![Image 14: image](https://hackmd.io/_uploads/rJASrho6ke.png)

If Node Operators want to use EIP-7002 to exit their validators, they can do so via a dedicated method in the `CSEjector.sol` contract. In this case, `CSEjector.sol` will notify `VEBO` about the required validator ejection.

### [](https://hackmd.io/@lido/csm-v2-spec#Withdrawal-reporting "Withdrawal-reporting")Withdrawal reporting

Changed in v2

*   Stuck penalty and TE fee are applied upon validator withdrawal if reported before;
*   Reset bond curve removed due to the introduction of the Node Operator types associated with the bond curve;

![Image 15: image](https://hackmd.io/_uploads/B1HApwvJxl.png)

Once the CSM validator is withdrawn, the CSM Bot will report it using a permissionless method. The report is submitted to the `CSVerifier.sol` to validate proof against beaconBlockRoot. The report is bypassed to the `CSModule.sol` if the proof is valid. `CSModule.sol` marks the validator as withdrawn and requests bond penalization for the Node Operator by `CSAccounting.sol` if the withdrawal balance is lower than 32 ETH.

If the validator is reported as stuck, the recorded stuck penalty is applied, and the recorded TE fee is confiscated. If the validator was not reported as stuck but the TE fee is recorded, the TE fee is ignored.

TE fee confiscation limit is introduced to protect Node Operators from excessive bond confiscation due to theoretically unlimited TE fees.

### [](https://hackmd.io/@lido/csm-v2-spec#Stuck-validators-ejection-penalty "Stuck-validators-ejection-penalty")Stuck validators ejection penalty

New in v2

![Image 16: image](https://hackmd.io/_uploads/HkKOawvkxx.png)

![Image 17: image](https://hackmd.io/_uploads/HkRFpvvylx.png)

With its updated functionality, `VEBO` can now trigger exits for the validators requested for exit in the `VEBO` report. However, the time when requested validators can be ejected is not limited. Hence, `CSModule.sol` should be notified by `StakingRouter` about the validator exits and the time between the request and ejection. If the time exceeds the threshold, the Node Operator should be penalized for not exiting their validators in time. If Triggerable Exit (TE) was used for the validator, depending on the exit type and if the validator was delayed to exit, the TE fee should be confiscated from the Node Operator's bond. Both stuck penalty and TE fee are recorded in `CSExitPenalties.sol` and applied upon validator withdrawal described above.

The validator is considered "stuck" if the proof is delivered stating that it was not exited for more than `allowedExitDelay` seconds since the moment it was requested/available for exit. `allowedExitDelay` is a parameter that can be set per-Node-Operator-type.

### [](https://hackmd.io/@lido/csm-v2-spec#Referral-program "Referral-program")Referral program

New in v2

![Image 18: image](https://hackmd.io/_uploads/BywNmcD1lx.png)

The referral program consists of seasons. At the start of each season, a Node Operator type that can be obtained as a reward and a referrals threshold are set at `VettedGate.sol` instance. These parameters can not be changed within a season. Points for inviting referrals are counted and valid only within a season. The beneficial Node Operator type can be claimed only while the season lasts. When a new season starts, all previously collected referral points are dropped.

Invite means that upon NO creation, the referral specifies the referrer's address in the transaction. This information is recorded on-chain. Node Operators with recorded invites get access to the benefits described.

Both the referral and the referrer should pass the identification process and get an ICS pass.

Referrer can not be specified for existing Node Operators, only for new ones who are eligible for the ICS Node Operator type at the moment of creation.

[](https://hackmd.io/@lido/csm-v2-spec#Contracts-specifications "Contracts-specifications")Contracts specifications
-------------------------------------------------------------------------------------------------------------------

### [](https://hackmd.io/@lido/csm-v2-spec#CSModulesol39 "CSModulesol39")[`CSModule.sol`](https://github.com/lidofinance/community-staking-module/blob/develop/docs/src/src/CSModule.sol/contract.CSModule.md)

### [](https://hackmd.io/@lido/csm-v2-spec#CSAccountingsol40 "CSAccountingsol40")[`CSAccounting.sol`](https://github.com/lidofinance/community-staking-module/blob/develop/docs/src/src/CSAccounting.sol/contract.CSAccounting.md)

### [](https://hackmd.io/@lido/csm-v2-spec#CSVerifiersol41 "CSVerifiersol41")[`CSVerifier.sol`](https://github.com/lidofinance/community-staking-module/blob/develop/docs/src/src/CSVerifier.sol/contract.CSVerifier.md)

### [](https://hackmd.io/@lido/csm-v2-spec#CSFeeDistributorsol42 "CSFeeDistributorsol42")[`CSFeeDistributor.sol`](https://github.com/lidofinance/community-staking-module/blob/develop/docs/src/src/CSFeeDistributor.sol/contract.CSFeeDistributor.md)

### [](https://hackmd.io/@lido/csm-v2-spec#CSFeeOraclesol43 "CSFeeOraclesol43")[`CSFeeOracle.sol`](https://github.com/lidofinance/community-staking-module/blob/develop/docs/src/src/CSFeeOracle.sol/contract.CSFeeOracle.md)

### [](https://hackmd.io/@lido/csm-v2-spec#HashConsensussol44 "HashConsensussol44")[`HashConsensus.sol`](https://github.com/lidofinance/community-staking-module/blob/develop/docs/src/src/lib/base-oracle/HashConsensus.sol/contract.HashConsensus.md)

### [](https://hackmd.io/@lido/csm-v2-spec#CSParametersRegistrysol45 "CSParametersRegistrysol45")[`CSParametersRegistry.sol`](https://github.com/lidofinance/community-staking-module/blob/develop/docs/src/src/CSParametersRegistry.sol/contract.CSParametersRegistry.md)

### [](https://hackmd.io/@lido/csm-v2-spec#CSStrikessol46 "CSStrikessol46")[`CSStrikes.sol`](https://github.com/lidofinance/community-staking-module/blob/develop/docs/src/src/CSStrikes.sol/contract.CSStrikes.md)

### [](https://hackmd.io/@lido/csm-v2-spec#PermissionlessGatesol47 "PermissionlessGatesol47")[`PermissionlessGate.sol`](https://github.com/lidofinance/community-staking-module/blob/develop/docs/src/src/PermissionlessGate.sol/contract.PermissionlessGate.md)

### [](https://hackmd.io/@lido/csm-v2-spec#VettedGatesol48 "VettedGatesol48")[`VettedGate.sol`](https://github.com/lidofinance/community-staking-module/blob/develop/docs/src/src/VettedGate.sol/contract.VettedGate.md)

### [](https://hackmd.io/@lido/csm-v2-spec#VettedGateFactorysol "VettedGateFactorysol")[`VettedGateFactory.sol`](https://github.com/lidofinance/community-staking-module/blob/develop/docs/src/src/VettedGateFactory.sol/contract.VettedGateFactory.md)

### [](https://hackmd.io/@lido/csm-v2-spec#CSEjectorsol50 "CSEjectorsol50")[`CSEjector.sol`](https://github.com/lidofinance/community-staking-module/blob/develop/docs/src/src/CSEjector.sol/contract.CSEjector.md)

### [](https://hackmd.io/@lido/csm-v2-spec#CSExitPenaltiessol51 "CSExitPenaltiessol51")[`CSExitPenalties.sol`](https://github.com/lidofinance/community-staking-module/blob/develop/docs/src/src/CSExitPenalties.sol/contract.CSExitPenalties.md)

[](https://hackmd.io/@lido/csm-v2-spec#Administrative-actions "Administrative-actions")Administrative actions
-------------------------------------------------------------------------------------------------------------

Community Staking Module contracts support a set of administrative actions, including:

*   Changing the configuration options.
*   Upgrading the system's code.

Each action can only be performed by a designated admin (`DEFAULT_ADMIN_ROLE`) or other role members. Only members of `DEFAULT_ADMIN_ROLE` can manage role members for the roles in CSM contracts.

[](https://hackmd.io/@lido/csm-v2-spec#Roles-to-actors-mapping "Roles-to-actors-mapping")Roles to actors mapping
----------------------------------------------------------------------------------------------------------------

### [](https://hackmd.io/@lido/csm-v2-spec#CSModulesol54 "CSModulesol54")`CSModule.sol`

| Role | Assignee |
| --- | --- |
| `DEFAULT_ADMIN_ROLE` | Aragon Agent |
| `PAUSE_ROLE` | Gate Seal contract |
| `RESUME_ROLE` | Not assigned by default |
| `STAKING_ROUTER_ROLE` | `StakingRouter` contract |
| `MODULE_MANAGER_ROLE` | Removed in v2 and replaced with `DEFAULT_ADMIN_ROLE` |
| `REPORT_EL_REWARDS_STEALING_PENALTY_ROLE` | CSM Committee Multisig or Bot EOA |
| `SETTLE_EL_REWARDS_STEALING_PENALTY_ROLE` | Dedicated EasyTrack |
| `VERIFIER_ROLE` | `CSVerifier.sol` |
| `RECOVERER_ROLE` | Not assigned by default |
| `CREATE_NODE_OPERATOR_ROLE` | `PermissionlessGate.sol` and `VettedGate.sol` |

### [](https://hackmd.io/@lido/csm-v2-spec#CSAccountingsol55 "CSAccountingsol55")`CSAccounting.sol`

| Role | Assignee |
| --- | --- |
| `DEFAULT_ADMIN_ROLE` | Aragon Agent |
| `PAUSE_ROLE` | Gate Seal contract |
| `RESUME_ROLE` | Not assigned by default |
| `ACCOUNTING_MANAGER_ROLE` | Removed in v2 and replaced with `DEFAULT_ADMIN_ROLE` |
| `MANAGE_BOND_CURVES_ROLE` | Not assigned by default |
| `SET_BOND_CURVE_ROLE` | CSM Committee Multisig and `VettedGate.sol` |
| `RESET_BOND_CURVE_ROLE` | Removed in v2 |
| `RECOVERER_ROLE` | Not assigned by default |

### [](https://hackmd.io/@lido/csm-v2-spec#CSFeeDistributorsol56 "CSFeeDistributorsol56")`CSFeeDistributor.sol`

| Role | Assignee |
| --- | --- |
| `DEFAULT_ADMIN_ROLE` | Aragon Agent |
| `RECOVERER_ROLE` | Not Assigned |

### [](https://hackmd.io/@lido/csm-v2-spec#CSFeeOraclesol57 "CSFeeOraclesol57")`CSFeeOracle.sol`

| Role | Assignee |
| --- | --- |
| `DEFAULT_ADMIN_ROLE` | Aragon Agent |
| `CONTRACT_MANAGER_ROLE` | Removed in v2 and replaced with `DEFAULT_ADMIN_ROLE` |
| `SUBMIT_DATA_ROLE` | Not assigned by default |
| `PAUSE_ROLE` | GateSeal contract |
| `RESUME_ROLE` | Not assigned by default |
| `RECOVERER_ROLE` | Not assigned by default |
| `MANAGE_CONSENSUS_CONTRACT_ROLE` | Not assigned by default |
| `MANAGE_CONSENSUS_VERSION_ROLE` | Not assigned by default |

### [](https://hackmd.io/@lido/csm-v2-spec#HashConsensussol58 "HashConsensussol58")`HashConsensus.sol`

| Role | Assignee |
| --- | --- |
| `DEFAULT_ADMIN_ROLE` | Aragon Agent |
| `MANAGE_MEMBERS_AND_QUORUM_ROLE` | Aragon Agent |
| `DISABLE_CONSENSUS_ROLE` | Not assigned by default |
| `MANAGE_FRAME_CONFIG_ROLE` | Not assigned by default |
| `MANAGE_FAST_LANE_CONFIG_ROLE` | Not assigned by default |
| `MANAGE_REPORT_PROCESSOR_ROLE` | Not assigned by default |

### [](https://hackmd.io/@lido/csm-v2-spec#CSVerifiersol59 "CSVerifiersol59")`CSVerifier.sol`

| Role | Assignee |
| --- | --- |
| `DEFAULT_ADMIN_ROLE` | Aragon Agent |
| `PAUSE_ROLE` | GateSeal contract |
| `RESUME_ROLE` | Not assigned by default |

### [](https://hackmd.io/@lido/csm-v2-spec#CSParametersRegistrysol60 "CSParametersRegistrysol60")`CSParametersRegistry.sol`

| Role | Assignee |
| --- | --- |
| `DEFAULT_ADMIN_ROLE` | Aragon Agent |

### [](https://hackmd.io/@lido/csm-v2-spec#VettedGatesol61 "VettedGatesol61")`VettedGate.sol`

| Role | Assignee |
| --- | --- |
| `DEFAULT_ADMIN_ROLE` | Aragon Agent |
| `PAUSE_ROLE` | GateSeal contract |
| `RESUME_ROLE` | Not assigned by default |
| `SET_TREE_ROLE` | Dedicated EasyTrack |
| `START_REFERRAL_SEASON_ROLE` | Aragon Agent |
| `END_REFERRAL_SEASON_ROLE` | CSM Committee Multisig or Gate Manager |
| `RECOVERER_ROLE` | Not assigned by default |

### [](https://hackmd.io/@lido/csm-v2-spec#CSEjectorsol62 "CSEjectorsol62")`CSEjector.sol`

| Role | Assignee |
| --- | --- |
| `DEFAULT_ADMIN_ROLE` | Aragon Agent |
| `PAUSE_ROLE` | GateSeal contract |
| `RESUME_ROLE` | Not assigned by default |
| `RECOVERER_ROLE` | Not assigned by default |

### [](https://hackmd.io/@lido/csm-v2-spec#CSStrikessol63 "CSStrikessol63")`CSStrikes.sol`

| Role | Assignee |
| --- | --- |
| `DEFAULT_ADMIN_ROLE` | Aragon Agent |

### [](https://hackmd.io/@lido/csm-v2-spec#CSExitPenaltiessol64 "CSExitPenaltiessol64")`CSExitPenalties.sol`

This contract does not have roles.

### [](https://hackmd.io/@lido/csm-v2-spec#PermissionlessGatesol65 "PermissionlessGatesol65")`PermissionlessGate.sol`

| Role | Assignee |
| --- | --- |
| `DEFAULT_ADMIN_ROLE` | Aragon Agent |
| `RECOVERER_ROLE` | Not assigned by default |

[](https://hackmd.io/@lido/csm-v2-spec#Upgradability "Upgradability")Upgradability
----------------------------------------------------------------------------------

`CSModule.sol`, `CSAccounting.sol`, `CSFeeOracle.sol`, `CSFeeDistributor.sol`, `CSParametersRegistry.sol`, `CSStrikes.sol`, `CSExitPenalties.sol`, and `VettedGate.sol` are upgradable using [OssifiableProxy](https://github.com/lidofinance/community-staking-module/blob/main/src/lib/proxy/OssifiableProxy.sol) contracts.

`CSVerifier.sol`, `HashConsensus.sol`, `CSEjector.sol`, and `PermissionlessGate.sol` are not upgradable and should be redeployed if needed.

## Invariants

See below code for full list of invariants

```
```
// SPDX-FileCopyrightText: 2025 Lido <info@lido.fi>
// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import { IStETH } from "../../src/interfaces/IStETH.sol";
import { IBurner } from "../../src/interfaces/IBurner.sol";
import { CSFeeDistributor } from "../../src/CSFeeDistributor.sol";
import { CSModule } from "../../src/CSModule.sol";
import { NodeOperator } from "../../src/interfaces/ICSModule.sol";
import { Batch } from "../../src/lib/QueueLib.sol";
import { CSAccounting } from "../../src/CSAccounting.sol";
import { CSStrikes } from "../../src/CSStrikes.sol";
import { console } from "forge-std/console.sol";
import { CSFeeOracle } from "../../src/CSFeeOracle.sol";

contract InvariantAsserts is Test {
    bool internal _skipped;

    function skipInvariants() public returns (bool skip) {
        if (_skipped) {
            return true;
        }
        string memory profile = vm.envOr("FOUNDRY_PROFILE", string(""));
        bool isCIProfile = keccak256(abi.encodePacked(profile)) ==
            keccak256(abi.encodePacked("ci"));
        bool forkIsActive;
        try vm.activeFork() returns (uint256) {
            forkIsActive = true;
        } catch {}
        skip = !isCIProfile && forkIsActive;
        if (skip) {
            console.log(
                "WARN: Skipping invariants. It only runs with FOUNDRY_PROFILE=ci and active fork"
            );
            _skipped = true;
        }
    }

    function assertCSMKeys(CSModule csm) public {
        if (skipInvariants()) {
            return;
        }
        uint256 noCount = csm.getNodeOperatorsCount();
        NodeOperator memory no;

        uint256 totalDepositedValidators;
        uint256 totalExitedValidators;
        uint256 totalDepositableValidators;

        for (uint256 noId = 0; noId < noCount; noId++) {
            no = csm.getNodeOperator(noId);

            assertGe(
                no.totalAddedKeys,
                no.totalDepositedKeys,
                "assert added >= deposited"
            );
            assertGe(
                no.totalDepositedKeys,
                no.totalWithdrawnKeys,
                "assert deposited >= withdrawn"
            );
            assertGe(
                no.totalVettedKeys,
                no.totalDepositedKeys,
                "assert vetted >= deposited"
            );

            assertGe(
                no.totalDepositedKeys - no.totalExitedKeys,
                no.stuckValidatorsCount,
                "assert deposited - exited >= stuck"
            );

            assertGe(
                no.totalAddedKeys,
                no.depositableValidatorsCount + no.totalWithdrawnKeys,
                "assert added >= depositable + withdrawn"
            );
            assertGe(
                no.totalAddedKeys - no.totalDepositedKeys,
                no.depositableValidatorsCount,
                "assert added - deposited >= depositable"
            );

            assertNotEq(
                no.proposedManagerAddress,
                no.managerAddress,
                "assert proposed != manager"
            );
            assertNotEq(
                no.proposedRewardAddress,
                no.rewardAddress,
                "assert proposed != reward"
            );
            assertNotEq(no.managerAddress, address(0), "assert manager != 0");
            assertNotEq(no.rewardAddress, address(0), "assert reward != 0");

            totalExitedValidators += no.totalExitedKeys;
            totalDepositedValidators += no.totalDepositedKeys;
            totalDepositableValidators += no.depositableValidatorsCount;
        }

        (
            uint256 _totalExitedValidators,
            uint256 _totalDepositedValidators,
            uint256 _depositableValidatorsCount
        ) = csm.getStakingModuleSummary();
        assertEq(
            totalExitedValidators,
            _totalExitedValidators,
            "assert total exited"
        );
        assertEq(
            totalDepositedValidators,
            _totalDepositedValidators,
            "assert total deposited"
        );
        assertEq(
            totalDepositableValidators,
            _depositableValidatorsCount,
            "assert depositable"
        );
    }

    mapping(uint256 => uint256) batchKeys;

    function assertCSMEnqueuedCount(CSModule csm) public {
        if (skipInvariants()) {
            return;
        }
        uint256 noCount = csm.getNodeOperatorsCount();
        NodeOperator memory no;

        for (uint256 p = 0; p <= csm.QUEUE_LOWEST_PRIORITY(); ++p) {
            (uint128 head, uint128 tail) = csm.depositQueuePointers(p);

            for (uint128 i = head; i < tail; ) {
                Batch item = csm.depositQueueItem(p, i);
                batchKeys[item.noId()] += item.keys();
                i = item.next();
            }
        }

        for (uint256 noId = 0; noId < noCount; noId++) {
            no = csm.getNodeOperator(noId);
            assertEq(
                no.enqueuedCount,
                batchKeys[noId],
                "assert enqueued == batch keys"
            );
            assertGe(
                no.enqueuedCount,
                no.depositableValidatorsCount,
                "assert enqueued >= depositable"
            );
        }
    }

    function assertCSMUnusedStorageSlots(CSModule csm) public {
        if (skipInvariants()) {
            return;
        }
        bytes32 value;
        // _accountingOld
        value = vm.load(address(csm), bytes32(uint256(2)));
        assertEq(value, bytes32(0), "assert _accountingOld is empty");

        // _earlyAdoption
        value = vm.load(address(csm), bytes32(uint256(3)));
        assertEq(value, bytes32(0), "assert _earlyAdoption is empty");
    }

    function assertAccountingTotalBondShares(
        uint256 nodeOperatorsCount,
        IStETH steth,
        CSAccounting accounting
    ) public {
        if (skipInvariants()) {
            return;
        }
        uint256 totalNodeOperatorsShares;

        for (uint256 noId = 0; noId < nodeOperatorsCount; noId++) {
            totalNodeOperatorsShares += accounting.getBondShares(noId);
        }
        assertEq(
            totalNodeOperatorsShares,
            accounting.totalBondShares(),
            "total shares mismatch"
        );
        assertGe(
            steth.sharesOf(address(accounting)),
            accounting.totalBondShares(),
            "assert balance >= total shares"
        );
    }

    function assertAccountingBurnerApproval(
        IStETH steth,
        address accounting,
        address burner
    ) public {
        if (skipInvariants()) {
            return;
        }
        assertGe(
            steth.allowance(accounting, burner),
            type(uint128).max,
            "assert allowance"
        );
    }

    function assertAccountingUnusedStorageSlots(
        CSAccounting accounting
    ) public {
        if (skipInvariants()) {
            return;
        }
        // _feeDistributorOld
        bytes32 value = vm.load(address(accounting), bytes32(uint256(0)));
        assertEq(value, bytes32(0), "assert _feeDistributorOld is empty");
    }

    function assertFeeDistributorClaimableShares(
        IStETH lido,
        CSFeeDistributor feeDistributor
    ) public {
        if (skipInvariants()) {
            return;
        }
        assertGe(
            lido.sharesOf(address(feeDistributor)),
            feeDistributor.totalClaimableShares(),
            "assert balance >= claimable"
        );
    }

    function assertFeeDistributorTree(CSFeeDistributor feeDistributor) public {
        if (skipInvariants()) {
            return;
        }
        if (feeDistributor.treeRoot() == bytes32(0)) {
            assertEq(
                feeDistributor.treeCid(),
                "",
                "tree doesn't exist, but has CID"
            );
        } else {
            assertNotEq(
                feeDistributor.treeCid(),
                "",
                "tree exists, but has no CID"
            );
        }
    }

    function assertFeeOracleUnusedStorageSlots(CSFeeOracle feeOracle) public {
        if (skipInvariants()) {
            return;
        }
        bytes32 value;
        // _feeDistributor
        value = vm.load(address(feeOracle), bytes32(uint256(0)));
        assertEq(value, bytes32(0), "assert _feeDistributor is empty");

        // _avgPerfLeewayBP
        value = vm.load(address(feeOracle), bytes32(uint256(1)));
        assertEq(value, bytes32(0), "assert _avgPerfLeewayBP is empty");
    }

    function assertStrikesTree(CSStrikes strikes) public {
        if (skipInvariants()) {
            return;
        }
        if (strikes.treeRoot() == bytes32(0)) {
            assertEq(strikes.treeCid(), "", "tree doesn't exist, but has CID");
        } else {
            assertNotEq(strikes.treeCid(), "", "tree exists, but has no CID");
        }
    }
}
```
```


