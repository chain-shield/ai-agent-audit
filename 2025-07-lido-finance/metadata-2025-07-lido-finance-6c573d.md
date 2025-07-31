
## SUMMARY OF FILE: 2025-07-lido-finance/src/CSModule.sol
### CSModule Contract Summary
CSModule is a contract for managing staking operations, utilizing roles for controlled actions like pausing, resuming, and managing node operators. It extends several contracts for access control, pause functionality, and asset recovery.

#### Functions and Interfaces
- **initialize(address admin)**: Initializes the module with a specific function of settung an admin and roles. 
- **finalizeUpgradeV2()**: Prepares state for post-upgrade changes.
- **resume()**: Resumes the module's operations if paused.
- **pauseFor(uint256 duration)**: Pauses the contract for a specified duration.
- **createNodeOperator**: Registers a new node operator, setting management and reward addresses, handling referrer logic. 
- **reportELRewardsStealingPenalty**: Charges penalties for EL rewards stealing, with role-based access.
- **submitWithdrawals**: Processes validators' withdrawals with penalties if applicable.
- **obtainDepositData**: Retrieves deposit data for validators, organizing it by priority.

#### Variables
- **MODULE_TYPE (bytes32)**: Identifies the module.
- **LIDO_LOCATOR (ILidoLocator)**: Provides access to the Lido contract.
- **STETH (IStETH)**: Reference to the stETH contract.
- **PARAMETERS_REGISTRY (ICSParametersRegistry)**: Holds parameter configurations.
- **ACCOUNTING (ICSAccounting)**: Manages validator accounts and fees.
- **_nodeOperators (mapping)**: Stores details of each node operator.
- **_queueByPriority (mapping)**: Organizes validator queues by priority.



## SUMMARY OF FILE: 2025-07-lido-finance/src/CSParametersRegistry.sol
### CSParametersRegistry Contract Summary
The `CSParametersRegistry` smart contract is a comprehensive parameter registry for the crypto staking ecosystem, focusing on Curve Staking Module parameters. It's designed with an emphasis on flexible control of staking parameters by admins.

#### Key Features:
- Utilizes role-based access control to limit who can alter parameters.
- Implements default settings integrated with dynamic curve-based configurations.

#### Key Functions:
- **initialize** - Sets up the admin role and initializes multiple default parameters.
- **Configurations Setters and Getters** - Functions allow admins to set and get configurations related to key removal charges, performance leeway, strikes parameters, etc.
- **Data Management** - Internal functions manage parameter validity, storage, and event emission for state changes.

#### Key Storage Variables:
- `QUEUE_LOWEST_PRIORITY`, `QUEUE_LEGACY_PRIORITY` - Constants defining operational priorities.
- Mappings for parameter storage per curve, e.g., `_keyRemovalCharges`, `_performanceCoefficients`.

The code structure relies on modular parameter setting methods with robust validation to prevent sybil attacks or malicious changes.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSAccounting.sol
### Contract: CSAccounting

**Purpose**: The `CSAccounting` contract manages Node Operators' bonds in the form of stETH shares and is part of the Node Operator management and recovery process within the Lido protocol.

**Inheritance**: This contract inherits from `ICSAccounting`, `CSBondCore`, `CSBondCurve`, `CSBondLock`, `PausableUntil`, `AccessControlEnumerableUpgradeable`, and `AssetRecoverer`.

### Summary of Key Functions and Variables

#### Storage Variables
- **MODULE**: An immutable interface to the Staking Module, used for interaction with node operators.
- **FEE_DISTRIBUTOR**: An immutable interface to the Fee Distributor for handling fee operations.
- **chargePenaltyRecipient**: Address of the recipient for penalty charges, modifiable by admin roles.

#### Functions

1. **Constructor**
   - Initializes the contract with addresses for Lido locator, module, fee distributor, and bond lock periods.
   - Validates that essential addresses are non-zero.

2. **Initialize**
   - Reinitializer for setting up roles, bonding curves, and admin privileges.
   - Sets up Lido allowance for transactions.

3. **finalizeUpgradeV2**
   - Migrates existing bond curves to a new format during upgrades, ensuring consistency in bond-related parameters.

4. **resume**
   - Resumes contract operations that were previously paused.

5. **pauseFor**
   - Pauses the contract for a specified duration if invoked by an authorized role.

6. **setChargePenaltyRecipient**
   - Updates the penalty charge recipient address, restricting changes to admin role holders.

7. **setBondLockPeriod**
   - Sets the duration for which bonds are locked, managed by admin roles.

8. **addBondCurve**
   - Allows for the addition of new bond curves by members with manage privileges.

9. **depositETH**
   - Handles deposits in ETH to node operators' bond accounts, enforcing module-only access.

10. **claimRewardsStETH**
    - Claims rewards in stETH, with proof verification for node operators.

11. **lockBondETH**
    - Locks a specified amount of ETH in bond for node operators, module restricted.

12. **recoverERC20**
    - Enables ERC20 token recovery, excluding Lido tokens, restricted to recoverer roles.

### Overall Summary
The `CSAccounting` contract implements comprehensive functions for managing bonds, distributing fees, and controlling operational states related to node operator management in the Lido ecosystem. It enables role-based access control, supports bond curve modifications, and facilitates staking and fee reward handling through secure allowances and verifications. The contract ensures structured upgrades through initializer functions and maintains recovery and penalty mechanisms to align with Lido's network reliability and security standards.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSStrikes.sol
### Contract: CSStrikes
The `CSStrikes` contract manages key strikes within a defined module, accounting for bad performance of keys validated through a Merkle proof system. It uses multiple interfaces to facilitate these operations.

#### Storage Variables
- **ORACLE** (`address`) - The address of the Oracle used for processing reports.
- **MODULE, ACCOUNTING, EXIT_PENALTIES, PARAMETERS_REGISTRY** (`immutable` instances) - References to module contracts for organizing related operations.
- **ejector** (`ICSEjector`) - Reference to the Ejector contract facilitating the ejection of nodes after verifying strikes.
- **treeRoot** (`bytes32`) - Holds the latest Merkle tree root for strike data validation.
- **treeCid** (`string`) - Stores CID for the last published Merkle tree for tracing data sources.

#### Constructor
```solidity
constructor(
    address module,
    address oracle,
    address exitPenalties,
    address parametersRegistry
)
```
- Initializes the contract with module, oracle, exit penalties, and parameters registry addresses.

#### Functions
- **initialize**
```solidity
function initialize(address admin, address _ejector) external initializer
```
Initializes the contract, setting up roles and the ejector.

- **setEjector**
```solidity
function setEjector(address _ejector) external onlyRole(DEFAULT_ADMIN_ROLE)
```
Allows admin to update the ejector address.

- **processOracleReport**
```solidity
function processOracleReport(bytes32 _treeRoot, string calldata _treeCid) external onlyOracle
```
Handles new data from the Oracle, updating the tree root and CID.

- **processBadPerformanceProof**
```solidity
function processBadPerformanceProof(KeyStrikes[] calldata keyStrikesList, bytes32[] calldata proof, bool[] calldata proofFlags, address refundRecipient) external payable
```
Processes proofs of bad performance, ejecting and refunding accordingly.

- **getInitializedVersion**
```solidity
function getInitializedVersion() external view returns (uint64)
```
Returns the initialized version of the contract.

- **verifyProof**
```solidity
function verifyProof(KeyStrikes[] calldata keyStrikesList, bytes[] memory pubkeys, bytes32[] calldata proof, bool[] calldata proofFlags) public view returns (bool)
```
Verifies the Merkle proof of key strikes.

- **hashLeaf**
```solidity
function hashLeaf(KeyStrikes calldata keyStrikes, bytes memory pubkey) public pure returns (bytes32)
```
Creates a hash from key strikes data using a Merkle proof leaf.

This contract relies on accurate Merkle proofs to maintain integrity and enforce ejections related to performance issues.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSFeeOracle.sol
### CSFeeOracle Contract Summary

The `CSFeeOracle` contract inherits from `ICSFeeOracle`, `BaseOracle`, `PausableUntil`, and `AssetRecoverer`. It provides fee distribution logic and handles consensus report data processing by interacting with `ICSFeeDistributor` and `ICSStrikes` interfaces.

### Function Summaries

- **Constructor**
  - **Definition:** `constructor(address feeDistributor, address strikes, uint256 secondsPerSlot, uint256 genesisTime)`
  - **Summary:** Initializes the `CSFeeOracle` contract with fee distributor and strikes addresses, validating that they are not zero. Inherits initialization from `BaseOracle` using slot timing parameters.

- **initialize**
  - **Definition:** `initialize(address admin, address consensusContract, uint256 consensusVersion) external`
  - **Summary:** Sets up initial roles and contract parameters, specifically granting the admin the default role. It also initializes the base oracle contract with consensus details.

- **finalizeUpgradeV2**
  - **Definition:** `finalizeUpgradeV2(uint256 consensusVersion) external`
  - **Summary:** Upgrades the contract consensus version and clears deprecated storage slots. Updates the version to 2.

- **resume**
  - **Definition:** `function resume() external onlyRole(RESUME_ROLE)`
  - **Summary:** Resumes the oracle operations using the `_resume` method, requiring the `RESUME_ROLE` permission.

- **pauseFor**
  - **Definition:** `function pauseFor(uint256 duration) external onlyRole(PAUSE_ROLE)`
  - **Summary:** Temporarily pauses the oracle using the `_pauseFor` method for a specified duration, needing the `PAUSE_ROLE`.

- **submitReportData**
  - **Definition:** `function submitReportData(ReportData calldata data, uint256 contractVersion) external whenResumed`
  - **Summary:** Submits oracle report data for processing, conducting all necessary checks on roles, consensus data, and contract version.

- **_handleConsensusReport**
  - **Definition:** `function _handleConsensusReport(ConsensusReport memory, uint256, uint256) internal override`
  - **Summary:** Placeholder for handling report consensus once reached; currently no action taken.

- **_handleConsensusReportData**
  - **Definition:** `function _handleConsensusReportData(ReportData calldata data) internal`
  - **Summary:** Processes oracle report data via external contracts `FEE_DISTRIBUTOR` and `STRIKES`.

- **_checkMsgSenderIsAllowedToSubmitData**
  - **Definition:** `function _checkMsgSenderIsAllowedToSubmitData() internal view`
  - **Summary:** Ensures the message sender has permission to submit data by checking roles.

- **_onlyRecoverer**
  - **Definition:** `function _onlyRecoverer() internal view override`
  - **Summary:** Validates if the caller has the recovery role.

### Storage Variables

- **SUBMIT_DATA_ROLE**
  - **Definition:** `bytes32 public constant SUBMIT_DATA_ROLE = keccak256("SUBMIT_DATA_ROLE");`
  - **Explanation:** Unique identifier for role allowing report data submission.

- **PAUSE_ROLE**
  - **Definition:** `bytes32 public constant PAUSE_ROLE = keccak256("PAUSE_ROLE");`
  - **Explanation:** Identifier for the role authorized to pause oracle reports.

- **RESUME_ROLE**
  - **Definition:** `bytes32 public constant RESUME_ROLE = keccak256("RESUME_ROLE");`
  - **Explanation:** Role that allows resumption of paused reports.

- **RECOVERER_ROLE**
  - **Definition:** `bytes32 public constant RECOVERER_ROLE = keccak256("RECOVERER_ROLE");`
  - **Explanation:** Grants permission for asset recovery operations.

- **FEE_DISTRIBUTOR**
  - **Definition:** `ICSFeeDistributor public immutable FEE_DISTRIBUTOR;`
  - **Explanation:** Immutable reference to the fee distributor contract interface.

- **STRIKES**
  - **Definition:** `ICSStrikes public immutable STRIKES;`
  - **Explanation:** Immutable reference to the strikes processing interface.

- **_feeDistributor**
  - **Definition:** `ICSFeeDistributor internal _feeDistributor;`
  - **Explanation:** Deprecated storage for fee distributor, clearing in upgrade to V2.

- **_avgPerfLeewayBP**
  - **Definition:** `uint256 internal _avgPerfLeewayBP;`
  - **Explanation:** Previously used for performance leeway, deprecated and cleared in upgrade.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSExitPenalties.sol
### CSExitPenalties Contract Summary
The `CSExitPenalties` contract is part of a smart contract system designed to manage penalties for node operators who do not comply with exit protocols. This contract addresses penalties related to delayed exits, triggered exits, and strikes (performance penalties). It uses interfaces to interact with modules for accounting and parameter management.

#### Storage Variables
- **MODULE**: Holds the ICSModule instance, ensuring methods are called by authorized modules.
- **PARAMETERS_REGISTRY**: Stores ICSParametersRegistry for configuring penalty parameters.
- **ACCOUNTING**: Represents ICSAccounting module for accounting-related features.
- **STRIKES**: Holds the address for STRIKES, ensuring only authorized contracts can report performance issues.
- **_exitPenaltyInfo**: Mapping storing exit penalty information with a unique key comprising the node operator's ID and public key.

#### Functions
- **constructor**: Initializes the contract with module addresses, ensuring they are not zero addresses.

- **processExitDelayReport**: Allows the module to report delayed exits, calculating penalties if applicable, based on predefined parameters.

- **processTriggeredExit**: Handles penalties for non-voluntary exits. Updates penalty records and ensures fees are within limits.

- **processStrikesReport**: Allows STRIKES to report performance issues, calculating penalties as needed.

- **isValidatorExitDelayPenaltyApplicable**: Checks if a delayed exit penalty is applicable, only accessible to the module.

- **getExitPenaltyInfo**: Provides penalty information for a node operator's validator, accessible to external queries.

- **_keyPointer**: Internal function generating a unique key for storage mapping using node operator ID and public key.


## SUMMARY OF FILE: 2025-07-lido-finance/src/VettedGateFactory.sol
### `VettedGateFactory` Contract
The `VettedGateFactory` contract is a factory designed to create instances of the `VettedGate` contract. It uses the `OssifiableProxy` to delegate functionality to an implementation address, ensuring a flexible and upgradable architecture.

### `VETTED_GATE_IMPL` Storage Variable
- **Definition**: `address public immutable VETTED_GATE_IMPL;`
- **Explanation**: Stores the address of the `VettedGate` implementation, which is immutable and set at deployment. This ensures that all created proxies point to the same implementation, maintaining consistency across all `VettedGate` instances.

### Constructor
- **Summary**: `constructor(address vettedGateImpl)`
- **Interface**: Sets the `VETTED_GATE_IMPL` address during deployment. Throws an error if the provided address is zero.

### `create` Function
- **Summary**: `function create(uint256 curveId, bytes32 treeRoot, string calldata treeCid, address admin) external returns (address instance)`
- **Interface**: Creates a new `OssifiableProxy` pointing to `VETTED_GATE_IMPL`, initializes it with given parameters, and returns its address. Emits a `VettedGateCreated` event upon successful creation.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSVerifier.sol
### CSVerifier Contract Summary
The `CSVerifier` contract is part of a smart contract ecosystem for Ethereum 2.0 staking and validator management by Lido. This contract implements the `ICSVerifier` interface and extends functionality from `AccessControlEnumerable` and `PausableUntil`. It performs tasks related to verifying Ethereum 2.0 validator withdrawals and integrates with a staking module to submit withdrawals.

### Function Summaries

#### amountWei
```solidity
function amountWei(Withdrawal memory withdrawal) pure returns (uint256)
```
Converts a withdrawal amount represented in gwei to wei.

#### gweiToWei
```solidity
function gweiToWei(uint64 amount) pure returns (uint256)
```
Helper function to convert a given amount in gwei to wei units by multiplying the gwei value by a factor of 1 gwei.

#### constructor
```solidity
constructor(...)
```
Initializes the `CSVerifier` contract with specified parameters for withdrawal addresses, module addresses, epochs, historical roots, and GIndices. It checks for valid configurations and sets default roles for admin.

#### resume
```solidity
function resume() external onlyRole(RESUME_ROLE)
```
Allows resuming the verifier's operations if the caller has the `RESUME_ROLE`.

#### pauseFor
```solidity
function pauseFor(uint256 duration) external onlyRole(PAUSE_ROLE)
```
Pauses the verifier's operations for a specified duration if the caller has the `PAUSE_ROLE`.

#### processWithdrawalProof
```solidity
function processWithdrawalProof(...)
```
Processes a withdrawal proof for a given validator using provided beacon block header and witness information when the verifier is not paused.

#### processHistoricalWithdrawalProof
```solidity
function processHistoricalWithdrawalProof(...)
```
Handles proof processing for historical withdrawals with detailed validations and state checks, using supplied proof data.

### Storage Variables

- `PAUSE_ROLE` & `RESUME_ROLE`: Constants for access control roles.
- `BEACON_ROOTS`: Address constant used in the EIP-4788 context.
- `SLOTS_PER_EPOCH` & `SLOTS_PER_HISTORICAL_ROOT`: Constants defining Ethereum blockchain time parameters.
- `GI_*`: Various `GIndex` instances holding constants related to GIndexing withdrawable states and historical summaries.
- `FIRST_SUPPORTED_SLOT`, `PIVOT_SLOT`, `CAPELLA_SLOT`: Slots related to Ethereum chain forks and supported operations.
- `WITHDRAWAL_ADDRESS`: Immutable address where validator withdrawals are directed.
- `MODULE`: Immutable instance of the staking module contract.


## SUMMARY OF FILE: 2025-07-lido-finance/src/lib/proxy/OssifiableProxy.sol
The `OssifiableProxy` contract is an advanced proxy implementation based on the ERC1967 standard, featuring additional admin functionalities. The contract allows for the ossification of the proxy, a state where it cannot undergo further upgrades by removing admin privileges.

### Functions:

**`modifier onlyAdmin()`**
- **Interface:** `modifier onlyAdmin()`
- **Summary:** Validates that the method caller is the admin and that the proxy is not ossified. It throws a `NotAdmin` error if the caller is not the admin and a `ProxyIsOssified` error if the admin address is `0`.

**`constructor (address implementation_, address admin_, bytes memory data_)`**
- **Interface:** `constructor(address implementation_, address admin_, bytes memory data_)`
- **Summary:** Initializes the proxy with a specified implementation and admin and calls the implementation's initialization logic if `data_` is not empty.

**`receive()`**
- **Interface:** `receive() external payable`
- **Summary:** A fallback function that delegates calls to the address returned by `_implementation()`. It handles empty call data by suppressing Solidity warnings regarding payable fallback function with no receive function.

**`proxy__ossify()`**
- **Interface:** `function proxy__ossify() external onlyAdmin`
- **Summary:** Transfers admin rights to the zero address, effectively ossifying the proxy to block any future upgrades. Emits `ProxyOssified` and `AdminChanged` events.

**`proxy__changeAdmin(address newAdmin_)`**
- **Interface:** `function proxy__changeAdmin(address newAdmin_) external onlyAdmin`
- **Summary:** Updates the proxy's admin to a new address, ensuring future administrative actions can only be executed by the new admin address.

**`proxy__upgradeTo(address newImplementation_)`**
- **Interface:** `function proxy__upgradeTo(address newImplementation_) external onlyAdmin`
- **Summary:** Upgrades the proxy to a new implementation using the `ERC1967Utils` utilities without any additional setup call.

**`proxy__upgradeToAndCall(address newImplementation_, bytes calldata setupCalldata_)`**
- **Interface:** `function proxy__upgradeToAndCall(address newImplementation_, bytes calldata setupCalldata_) external onlyAdmin`
- **Summary:** Upgrades the proxy to a new implementation and executes a setup call if `setupCalldata_` is provided.

**`proxy__getAdmin()`**
- **Interface:** `function proxy__getAdmin() external view returns (address)`
- **Summary:** Returns the current admin address, allowing external contracts or interfaces to verify the proxy's admin.

**`proxy__getImplementation()`**
- **Interface:** `function proxy__getImplementation() external view returns (address)`
- **Summary:** Returns the address of the current implementation, providing transparency about whom the proxy is delegating calls to.

**`proxy__getIsOssified()`**
- **Interface:** `function proxy__getIsOssified() external view returns (bool)`
- **Summary:** Verifies if the proxy has been ossified by checking if the admin address is `0`.

### Storage Variables:
- **N/A (No specific storage variables defined as all variables rely on inherited utility library functions and parent classes)**


## SUMMARY OF FILE: 2025-07-lido-finance/src/lib/base-oracle/HashConsensus.sol
HashConsensus is a smart contract for managing an oracle committee, which enables members to reach consensus on hash reports. It divides time into frames, each with a reference slot and processing deadline, ensuring all state changes a report could entail are observed before processing the next frame's report. The contract incorporates roles for managing members, reporting intervals, and changing the report processor.

### Contract Definition
```solidity
contract HashConsensus is IConsensusContract, AccessControlEnumerableUpgradeable
```

### Key Functions

**Constructor**
Sets immutable parameters for the Ethereum chain and initializes consensus frame configuration.
```solidity
constructor(
    uint256 slotsPerEpoch,
    uint256 secondsPerSlot,
    uint256 genesisTime,
    uint256 epochsPerFrame,
    uint256 fastLaneLengthSlots,
    address admin,
    address reportProcessor
)
```

**getChainConfig**
Returns immutable chain parameters, such as slots per epoch and genesis time.
```solidity
function getChainConfig() external view returns (
    uint256 slotsPerEpoch,
    uint256 secondsPerSlot,
    uint256 genesisTime
)
```

**getFrameConfig**
Provides current configuration of time-related settings, like initial epoch and frame length.
```solidity
function getFrameConfig() external view returns (
    uint256 initialEpoch,
    uint256 epochsPerFrame,
    uint256 fastLaneLengthSlots
)
```

**getCurrentFrame**
Delivers the current frame details, including its reference and processing deadline slots.
```solidity
function getCurrentFrame() external view returns (
    uint256 refSlot,
    uint256 reportProcessingDeadlineSlot
)
```

**getIsMember**
Checks membership status in the oracle by address.
```solidity
function getIsMember(address addr) external view returns (bool)
```

**getIsFastLaneMember**
Determines if an address is a fast lane member for the reporting frame.
```solidity
function getIsFastLaneMember(address addr) external view returns (bool)
```

**setFrameConfig**
Updates configuration for epochs per frame and fast lane slots, controlling the frame operations.
```solidity
function setFrameConfig(
    uint256 epochsPerFrame,
    uint256 fastLaneLengthSlots
) external
```

**submitReport**
Enables oracle members to submit their computed hash reports for a specified reference slot.
```solidity
function submitReport(
    uint256 slot,
    bytes32 report,
    uint256 consensusVersion
) external
```

### Key Variables

**SLOTS_PER_EPOCH**
Immutable storage tracking the number of slots per epoch in Ethereum, setting time calculation parameters.
```solidity
uint64 internal immutable SLOTS_PER_EPOCH;
```

**SECONDS_PER_SLOT**
Immutable storage indicating seconds per slot, essential for synchronizing with Ethereum timeframes.
```solidity
uint64 internal immutable SECONDS_PER_SLOT;
```

**GENESIS_TIME**
Immutable genesis timestamp, fundamental for mapping Ethereum slots and epochs to real-world time.
```solidity
uint64 internal immutable GENESIS_TIME;
```

**_reportVariants**
Maps report variant index to its structure, key for managing multiple data report states.
```solidity
mapping(uint256 => ReportVariant) internal _reportVariants;
```

**_frameConfig**
Structure tracking reporting frame settings, affecting how data is divided and analyzed across time.
```solidity
FrameConfig internal _frameConfig;
```

**_reportProcessor**
Stores the contract address responsible for processing and validating consensus reports.
```solidity
address internal _reportProcessor;
```

### Events

- **FrameConfigSet(uint256 newInitialEpoch, uint256 newEpochsPerFrame);**
- **FastLaneConfigSet(uint256 fastLaneLengthSlots);**
- **MemberAdded(address indexed addr, uint256 newTotalMembers, uint256 newQuorum);**
- **MemberRemoved(address indexed addr, uint256 newTotalMembers, uint256 newQuorum);**
- **QuorumSet(uint256 newQuorum, uint256 totalMembers, uint256 prevQuorum);**
- **ReportReceived(uint256 indexed refSlot, address indexed member, bytes32 report);**
- **ConsensusReached(uint256 indexed refSlot, bytes32 report, uint256 support);**
- **ConsensusLost(uint256 indexed refSlot);**
- **ReportProcessorSet(address indexed processor, address indexed prevProcessor);**

### Errors

- **InvalidChainConfig** – Thrown if chain configuration is invalid.
- **NumericOverflow** – Thrown when a value exceeds allowable numeric limits.
- **AdminCannotBeZero** – Thrown if admin role address is zero.
- **ReportProcessorCannotBeZero** – Thrown if report processor address is zero.
- **DuplicateMember** – Thrown when attempting to add a member that already exists.
- **AddressCannotBeZero** – Thrown if a zero address is used where it's not allowed.

### Explanation

HashConsensus organizes oracle-driven hash consensus in time frames, respecting the Ethereum epoch durations. It defines roles and reports processing workflow, with mechanisms to handle member management, quorum setting, and fast-lane features for prioritizing report submissions. This ensures robust and timely report visibility and facilitates decision-making in dynamic environments.


## SUMMARY OF FILE: 2025-07-lido-finance/src/lib/utils/PausableUntil.sol
### PausableUntil Contract
The `PausableUntil` contract provides functionality to pause and resume operations for a configurable duration. It utilizes unstructured storage to maintain the pause state, ensuring flexibility and security.

#### Key Variables:
- **RESUME_SINCE_TIMESTAMP_POSITION**: Used to store the timestamp when the contract resumes post-pause.
- **PAUSE_INFINITELY**: A special uint256 constant marking an indefinite pause.

#### Events:
- **Paused(uint256 duration)**: Triggered when the contract is paused, indicating the duration.
- **Resumed()**: Triggered when the contract resumes.

#### Functions:
- **getResumeSinceTimestamp()**: Returns when the contract will resume if paused, an infinite pause marker, or a past timestamp.
- **isPaused()**: Indicates whether the contract is currently paused.
- **_resume()**: Internal function to resume the contract, marking the operation and emitting `Resumed`.
- **_pauseFor(uint256 duration)**: Pauses the contract for a specified duration; throws if duration is zero.
- **_pauseUntil(uint256 pauseUntilInclusive)**: Pauses until a specified future timestamp; throws if in the past.
- **_setPausedState(uint256 resumeSince)**: Sets the pause state using the `resumeSince` value and emits `Paused`.
- **_checkPaused()/_checkResumed()**: Internal checks ensuring the correct paused/resumed state.


## SUMMARY OF FILE: 2025-07-lido-finance/src/lib/utils/Versioned.sol
### Contract: `Versioned`
The `Versioned` contract ensures correct contract version management. It uses `UnstructuredStorage` to store version information, supporting initialization and upgrade processes by managing the version state safely. It prevents reinitialization, with version control marked by `CONTRACT_VERSION_POSITION` and locked by `PETRIFIED_VERSION_MARK`.

#### Storage Variables:
- **`CONTRACT_VERSION_POSITION`**: `bytes32` constant storing the position of the contract version in storage, initialized to a specific keccak256 hash key.
- **`PETRIFIED_VERSION_MARK`**: `uint256` constant set to the max uint256 value, used to lock the version in the contract's storage, preventing reinitialization.

#### Functions:
- **`constructor()`**: Initializes the contract's version to `PETRIFIED_VERSION_MARK` to prevent initialization. 
  ```solidity
  constructor()
  ```
- **`getContractVersion()`**: Returns the current contract version.
  ```solidity
  function getContractVersion() public view returns (uint256)
  ```
- **`_initializeContractVersionTo(uint256 version)`**: Internal function to set the initial contract version. Checks are done to ensure the version is non-zero and uninitialized.
  ```solidity
  function _initializeContractVersionTo(uint256 version) internal
  ```
- **`_updateContractVersion(uint256 newVersion)`**: Updates to a new version, ensuring correct increment from the current version.
  ```solidity
  function _updateContractVersion(uint256 newVersion) internal
  ```
- **`_checkContractVersion(uint256 version)`**: Validates expected version matches the current version.
  ```solidity
  function _checkContractVersion(uint256 version) internal view
  ```
- **`_setContractVersion(uint256 version)`**: Private setter function that updates the version and emits a version set event.
  ```solidity
  function _setContractVersion(uint256 version) private
  ```


## SUMMARY OF FILE: 2025-07-lido-finance/src/VettedGate.sol
# VettedGate Contract Summary
### Contract Definition
The `VettedGate` contract integrates several functionalities including access control, staking management, and an optional referral program using merkle trees. It enables control over node operators within a smart contract ecosystem, particularly for staking applications.

### Functions
- `constructor(address module)`: Initializes with a module address and checks for zero address, setting up the MODULE and ACCOUNTING interfaces.

- `initialize(uint256 _curveId, bytes32 _treeRoot, string calldata _treeCid, address admin) external`: Sets initial configurations such as curve ID, merkle tree parameters, and assigns admin role with access control.

- `resume() external`: Allows contract functionality to resume by the authorized user, granting normal operations to continue.

- `pauseFor(uint256 duration) external`: Pauses operations temporarily, controlled by users with PAUSE_ROLE.

- `startNewReferralProgramSeason(uint256 _referralCurveId, uint256 _referralsThreshold) external`: Initiates a new referral program season, setting required parameters and emitting an event to log the start of the new season.

- `endCurrentReferralProgramSeason() external`: Ends the current referral program season, ensuring no duplicate end actions by auth checks.

- `addNodeOperatorETH(...) external payable`: Adds a new node operator using ETH, verifying merkle proof and updating referrals, if applicable.

- `addNodeOperatorStETH(...) external`: Similar to ETH version, this adds node operators using StETH.

- `addNodeOperatorWstETH(...) external`: Allows addition using wrapped StETH, sharing functionality with StETH operators.

- `claimBondCurve(uint256 nodeOperatorId, bytes32[] calldata proof) external`: Claims a bond curve by verifying proof and ensuring unique claims.

- `claimReferrerBondCurve(...) external`: Allows referrers who meet specific requirements to claim a referral bond curve.

- `setTreeParams(bytes32 _treeRoot, string calldata _treeCid) external`: Updates the Merkle tree's root and CID, ensuring integrity and no unauthorized changes.

- `getReferralsCount(address referrer) external view`: Retrieves a referrer’s number of referrals.

- `getInitializedVersion() external view`: Fetches the initialized version of the contract for compatibility checks.

- `isReferrerConsumed(address referrer) external view`: Checks if a referrer's claim actions have been fulfilled.

- `isConsumed(address member) external view`: Determines if a member's account has been consumed within the proof tree.

- `verifyProof(address member, bytes32[] calldata proof) external view`: Validates a Merkle proof for membership verification.

### Storage Variables
- `PAUSE_ROLE`, `RESUME_ROLE`, `RECOVERER_ROLE`, et al.: Bytes32 constants defining roles for access control, allowing specific operations by authorized users.

- `MODULE`: Immutable address of the ICSModule, handling members' staking module operations.

- `ACCOUNTING`: Immutable address for CS accounting operations, ensuring financial integrity within the module.

- `curveId`, `treeRoot`, `treeCid`: Used to define and verify eligible members using Merkle tree properties, and to assign bond curves.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSFeeDistributor.sol
### `CSFeeDistributor` Contract
The `CSFeeDistributor` is a contract designed to manage the distribution of fees in the form of stETH shares to node operators, with facilities for upgrading, recovering assets, and integrating with ACL roles.

#### Key Contract Interfaces:
- **constructor:** Initializes the contract with key addresses and sets them as immutable.
- **initialize:** Sets up the admin and rebate recipient roles for the initial upgrade.
- **finalizeUpgradeV2:** Allows setting a rebate recipient during contract upgrade.
- **setRebateRecipient:** Restricted to Admin, sets the recipient of rebate amounts.
- **distributeFees:** Allocates fees based on node operators' ID, utilizing Merkle proof validation to ensure legitimacy.
- **processOracleReport:** Handles reporting from the oracle, updating shares and distributing rebates.
- **recoverERC20:** Enables ERC20 recovery, with constraints on certain tokens.
- **getInitializedVersion:** Returns the initialized version of the contract.
- **pendingSharesToDistribute:** Calculates shares pending for distribution.
- **getHistoricalDistributionData:** Retrieves historical distribution data.
- **getFeesToDistribute:** Calculates distributable shares for a provided Merkle proof.
- **hashLeaf:** Hashes the input node operator ID and shares for Merkle proof.

#### Important Storage Variables:
- **RECOVERER_ROLE**: Role identifier for authorization to perform recovery operations.
- **STETH, ACCOUNTING, ORACLE**: Immutable addresses for stETH handling, accounting operations, and Oracle data.
- **treeRoot, treeCid, logCid:** Hold the Merkle Tree root and related metadata.
- **distributedShares, totalClaimableShares:** Track shares distributed to nodes and total shares available for claiming.
- **_distributionDataHistory:** Stores historical distribution data entries.
- **distributionDataHistoryCount:** Counter for the number of distribution history records.
- **rebateRecipient:** Address designated to receive rebate shares.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSEjector.sol
### CSEjector Contract Summary

The `CSEjector` contract orchestrates validators' ejection processes, crucial for managing staking modules, validators, and handling role-based control via OpenZeppelin's AccessControl.

#### Contract Definition 
- **Contract Name**: `CSEjector`
- **Inherits**: `ICSEjector`, `ExitTypes`, `AccessControlEnumerable`, `PausableUntil`, `AssetRecoverer`

### Variables
- **PAUSE_ROLE (bytes32)**: Defined for role-based access to pause functionality.
- **RESUME_ROLE (bytes32)**: For resuming control via access.
- **RECOVERER_ROLE (bytes32)**: Grants role-based access for asset recovery.
- **STAKING_MODULE_ID (uint256)**: Unique identifier for staking module group.
- **MODULE (ICSModule)**: Immutable ICSModule interface reference aiding in validators operations.
- **STRIKES (address)**: Immutable address accessing the STRIKES module.

### Functions

#### Constructor
- **Interface**:  
  ```solidity
  constructor(address module, address strikes, uint256 stakingModuleId, address admin)
  ```
- **Summary**: Initializes critical contract variables by confirming valid non-zero addresses for module, strikes, and admin, and assigns the admin role.

#### resume
- **Interface**:  
  ```solidity
  function resume() external onlyRole(RESUME_ROLE) succinctly
  ```
- **Summary**: Enables resuming of paused operations, executing under the `RESUME_ROLE`.

#### pauseFor
- **Interface**:  
  ```solidity
  function pauseFor(uint256 duration) external onlyRole(PAUSE_ROLE)
  ```
- **Summary**: Pauses contract activity for a specified duration, accessible with the `PAUSE_ROLE`.

#### voluntaryEject
- **Interface**:  
  ```solidity
  function voluntaryEject(uint256 nodeOperatorId, uint256 startFrom, uint256 keysCount, address refundRecipient) external payable whenResumed
  ```
- **Summary**: Allows node operators with the `nodeOperatorId` to voluntarily eject validators, operating only when the contract is active, ensuring non-withdrawn state of validators.

#### voluntaryEjectByArray
- **Interface**:  
  ```solidity
  function voluntaryEjectByArray(uint256 nodeOperatorId, uint256[] calldata keyIndices, address refundRecipient) external payable whenResumed
  ```
- **Summary**: Facilitates non-sequential ejection of validators indexed by `keyIndices`, heightened by consolidated gas efficiency.

#### ejectBadPerformer
- **Interface**:  
  ```solidity
  function ejectBadPerformer(uint256 nodeOperatorId, uint256 keyIndex, address refundRecipient) external payable whenResumed onlyStrikes
  ```
- **Summary**: Enables strikes-triggered ejection operations for underperforming nodes or unchecked validators, verified against staking modules.

#### triggerableWithdrawalsGateway
- **Interface**:  
  ```solidity
  function triggerableWithdrawalsGateway() public view returns (ITriggerableWithdrawalsGateway)
  ```
- **Summary**: Facilitates access to a `triggerableWithdrawalsGateway`, critical for orchestrating validator exit operations.

### Internal Functions
#### _onlyNodeOperatorOwner
- **Interface**:
  ```solidity
  function _onlyNodeOperatorOwner(uint256 nodeOperatorId) internal view
  ```
- **Summary**: Confirms the sender's ownership over a `nodeOperatorId`, essential for eligibility verification.

#### _onlyRecoverer
- **Interface**:
  ```solidity
  function _onlyRecoverer() internal view override
  ```
- **Summary**: Confirms role adherence for asset recovery operations, guarded by the `RECOVERER_ROLE`.


## SUMMARY OF FILE: 2025-07-lido-finance/src/PermissionlessGate.sol
### Contract: PermissionlessGate
This Solidity contract, `PermissionlessGate`, enables the permissionless addition of Node Operators. It extends `AccessControlEnumerable` for role management and `AssetRecoverer` for asset recovery. The module has a fixed `CURVE_ID` from its associated staking module.

### Storage Variables:
- **RECOVERER_ROLE** (`bytes32`): A constant role identifier for managing asset recovery. 
- **CURVE_ID** (`uint256`): Immutable ID associated with the default bond curve from the accounting contract.
- **MODULE** (`ICSModule`): The staking module's address, immutable once set. Ensures interactions with staking infrastructure.

### Constructor:
- **PermissionlessGate(address module, address admin)**: Sets up the module and admin, assigning necessary roles. 
  - Validates addresses, setting `MODULE` and `CURVE_ID`. Grants `DEFAULT_ADMIN_ROLE` to admin.

### Functions:
- **addNodeOperatorETH**: Adds node operators funded by ETH.
  - Uses `MODULE` to create a node operator and add validator keys.
- **addNodeOperatorStETH**: Adds node operators funded by staked ETH.
  - Similar to ETH function, but uses `stETH` permits.
- **addNodeOperatorWstETH**: Adds node operators funded by wrapped staked ETH.
  - Similar to `stETH` function, but uses `wstETH` permits.
- **_onlyRecoverer**: Checks if the caller has `RECOVERER_ROLE` before asset recovery.


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


