
## SUMMARY OF FILE: 2025-07-lido-finance/src/CSModule.sol
### Contract: `CSModule`
The `CSModule` contract extends several modules like `ICSModule`, `AccessControlEnumerableUpgradeable`, and others. It serves as a comprehensive staking module managing operations for node operators including key management, penalties, module state transitions, and interaction with staking routers and fee distributors.

### Key Storage Variables:
- **Roles**: Various roles like `PAUSE_ROLE`, `RESUME_ROLE`, `STAKING_ROUTER_ROLE` are defined using `keccak256`. These facilitate access control in the contract.
- **Constants**: Constants like `DEPOSIT_SIZE` and `FORCED_TARGET_LIMIT_MODE_ID` are used for defining limits and operational modes.
- **Immutables**: `MODULE_TYPE`, `LIDO_LOCATOR`, `STETH`, `PARAMETERS_REGISTRY` are initialized through the constructor, aiding in module identification and contract interaction.
- **Mappings**: `_queueByPriority`, `_nodeOperators`, `_isValidatorWithdrawn` manage queue priorities, node operator details, and validator withdrawal states.
- **Counters**: `_nonce`, `_totalDepositedValidators`, `_totalExitedValidators` keep track of state changes and validator activity.

### Functions Summary:
1. **Constructor**: Initializes key external dependencies and constants defining core parameters for operation.
2. **initialize**: Sets up access control and initializes the module like pausing the CSM initially.
3. **finalizeUpgradeV2**: Housekeeping operation to nullify deprecated references post-upgrade.
4. **resume & pauseFor**: Manage the active state, transitioning between paused and resumed states.
5. **createNodeOperator**: Establishes a new node operator, configured with management and reward properties.
6. **addValidatorKeysETH/WstETH/StETH**: Upload keys with corresponding stETH or wstETH as collateral.
7. **propose/confirm/modifyNodeOperatorAddress**: Manage node operator address changes for management or rewards.
8. **onRewardsMinted & onNodeOperatorXXXX**: Handle reward distribution and update node operator summary.
9. **obtainDepositData**: Fetch depositable keys and signatures from the queue based on demands.
10. **_enqueueNodeOperatorKeys**: Internally manage the queue of keys for maximum allowable limits and prioritization.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSParametersRegistry.sol
### Contract Summary: CSParametersRegistry
The `CSParametersRegistry` contract is responsible for managing various configuration parameters used in staking operations. It offers flexibility to modify these parameters depending on specific curves while providing a set of default values. Dual Governance is implemented to ensure malicious changes can be checked by stETH holders.

### Function Summaries

- **Constructor (uint256 queueLowestPriority)**
  - **Summary:** Initializes the contract with a specified `queueLowestPriority`, ensuring it's non-zero, and disables initializers for upgradeability.
  - **Interface:** `constructor(uint256 queueLowestPriority)`

- **Initialize (address admin, InitializationData calldata data)**
  - **Summary:** Sets initial configuration parameters based on input data and assigns the admin role, ensuring necessary validations.
  - **Interface:** `function initialize(address admin, InitializationData calldata data) external initializer`

- **Various set and unset functions**
  - **Summary:** Functions like `setDefaultKeyRemovalCharge`, `unsetKeyRemovalCharge`, etc., allow admins to set and unset various configuration parameters specific to curveIds. They are guarded by role-based access.
  - **Interface:** Functions include setting methods like `function setDefaultKeyRemovalCharge(uint256 keyRemovalCharge) external` or unsetting like `function unsetKeyRemovalCharge(uint256 curveId) external`

- **Getters for configuration parameters**
  - **Summary:** Functions such as `getKeyRemovalCharge` provide access to configuration values, defaulting to global defaults if specific values are unset.
  - **Interface:** `function getKeyRemovalCharge(uint256 curveId) external view returns (uint256)`

- **Internal Setters**
  - **Summary:** Internal methods such as `_setDefaultQueueConfig` handle validations and state updates for default parameters.
  - **Interface:** Internal functions are not directly accessible but are invoked within the public functions to perform core logic.

### Storage Variables

- **MAX_BP**
  - **Definition:** An internal constant representing the maximal allowable value for basis points, set to 10,000.
  - **Explanation:** Used to validate configurations ensuring they don't exceed limits.

- **defaultKeyRemovalCharge, defaultElRewardsStealingAdditionalFine, defaultKeysLimit**
  - **Definition:** Public uint256 variables representing default values for key removal, additional EL fines, and key limits.
  - **Explanation:** These defaults apply at the contract level unless overridden for specific curves.

- **Mapping `curveId` Mappings**
  - **Definition:** Mappings such as `_keyRemovalCharges`, `_performanceLeewayData` link curveId to specific parameter settings.
  - **Explanation:** They allow customized configurations per curveId, offering tailored parameter management.

- **Queue and Strikes Configurations**
  - **Definition:** Public and internal variables managing default queue settings and strike policies for operations.
  - **Explanation:** Provide control over staking queues and performance penalties through adjustable parameters.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSAccounting.sol
### Contract: CSAccounting

The `CSAccounting` contract manages Node Operators' bonds using stETH shares. It handles bond lock periods, supports pausing and resuming operations, and includes penalty and fee charge mechanisms.

#### Functions:

- **constructor:** Configures parameters for Lido locator, module, fee distributor, and bond lock periods. Initializes immutable variables for the MODULE and FEE_DISTRIBUTOR.
- **initialize:** Sets up roles and approvals, configures bond curves, and sets the charge penalty recipient. Requires non-zero admin and charge penalty recipient addresses.
- **finalizeUpgradeV2:** Migrates bond curves to the new format and initializes them.
- **resume:** Allows operations to continue by removing the paused state.
- **pauseFor:** Temporarily halts operations for a specified duration.
- **setChargePenaltyRecipient:** Updates the recipient address for charge penalties.
- **setBondLockPeriod:** Modifies the bond lock period.
- **addBondCurve:** Adds new bond curves and returns their ID.
- **updateBondCurve:** Updates existing bond curve values using a specified ID.
- **setBondCurve:** Assigns a bond curve to a Node Operator and updates its depositable validators count.
- **depositETH:** Handles ETH deposits for Node Operators, either directly or via a module.
- **depositStETH & depositWstETH:** Manages stETH and wstETH deposits, validating ownership through permit if needed.
- **claimRewardsStETH & claimRewardsWstETH:** Facilitates rewards claim, optionally using a proof, and updates available validators.
- **lockBondETH, releaseLockedBondETH, and compensateLockedBondETH:** Manages bond locking and compensation mechanisms.
- **settleLockedBondETH:** Processes locked bond settlement with core burning.
- **penalize:** Reduces Node Operator's bond by burning.
- **chargeFee:** Charges fees from Node Operator’s bond.
- **pullFeeRewards:** Retrieves and accounts for pending rewards, updating the validator count.
- **recoverERC20 and recoverStETHShares:** Implements asset recovery mechanisms ensuring bonds are not affected.
- **renewBurnerAllowance:** Re-approves the burner address for all LIDO tokens.
- **getInitializedVersion, getBondSummary, getUnbondedKeysCount, etc.**: Getter functions provide bond and operator data.

#### Storage Variables:

- **bytes32 public constant PAUSE_ROLE, RESUME_ROLE, etc.:** Define roles for specific contract permissions.
- **ICSModule public immutable MODULE, ICSFeeDistributor public immutable FEE_DISTRIBUTOR:** Store module and fee distributor addresses.
- **address public chargePenaltyRecipient:** Address that receives penalties, configurable by the admin role.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSStrikes.sol
### CSStrikes Contract Summary

The `CSStrikes` contract in Solidity is an implementation of the `ICSStrikes` interface aimed at handling bad performance proofs via Merkle proofs. It incorporates modules like ICSModule, ICSAccounting, ICSExitPenalties, and ICSParametersRegistry. The contract provides methods to set ejectors, process Oracle reports, and manage bad performance proofs.

#### Contract Definition
- **Name**: `CSStrikes`
- **Author**: vgorkavenko
- **Inherits**: `ICSStrikes`, `Initializable`, `AccessControlEnumerableUpgradeable`

#### Storage Variables
- **ORACLE**: (`address`) immutable address of the Oracle responsible for reports.
- **MODULE**: (`ICSModule`) immutable module address used for accounting.
- **ACCOUNTING**: (`ICSAccounting`) immutable associated accounting interface.
- **EXIT_PENALTIES**: (`ICSExitPenalties`) immutable penalties interface for exits.
- **PARAMETERS_REGISTRY**: (`ICSParametersRegistry`) immutable registry for parameter settings.
- **`ejector`**: (`ICSEjector`) responsible for executing the ejection of bad performance.
- **`treeRoot`**: (`bytes32`) latest Merkle Tree root representing valid performance data.
- **`treeCid`**: (`string`) CID of the latest published Merkle Tree for validation purposes.

#### Functions

- **Constructor**: 
  - **Interface**: `constructor(address module, address oracle, address exitPenalties, address parametersRegistry)`
  - **Summary**: Initializes immutables for module interaction, oracle setup, exit penalties, and parameters registry, also disables further initializers.

- **initialize**:
  - **Interface**: `function initialize(address admin, address _ejector) external initializer`
  - **Summary**: Sets up the admin role and assigns an ejector for managing bad performers.

- **setEjector**:
  - **Interface**: `function setEjector(address _ejector) external onlyRole(DEFAULT_ADMIN_ROLE)`
  - **Summary**: Updates the ejector address responsible for managing poorly performing nodes.

- **processOracleReport**: 
  - **Interface**: `function processOracleReport(bytes32 _treeRoot, string calldata _treeCid) external onlyOracle`
  - **Summary**: Processes a new oracle report to update the Merkle Tree root and CID, ensuring consistency.

- **processBadPerformanceProof**: 
  - **Interface**: `function processBadPerformanceProof(KeyStrikes[] calldata keyStrikesList, bytes32[] calldata proof, bool[] calldata proofFlags, address refundRecipient) external payable`
  - **Summary**: Validates performance data using Merkle proof, handling any penalties and refunds accordingly.

- **getInitializedVersion**: 
  - **Interface**: `function getInitializedVersion() external view returns (uint64)`
  - **Summary**: Retrieves the initialized version of the contract.

- **verifyProof**:
  - **Interface**: `function verifyProof(KeyStrikes[] calldata keyStrikesList, bytes[] memory pubkeys, bytes32[] calldata proof, bool[] calldata proofFlags) public view returns (bool)`
  - **Summary**: Verifies given performance data against stored Merkle Tree proof.

- **hashLeaf**:
  - **Interface**: `function hashLeaf(KeyStrikes calldata keyStrikes, bytes memory pubkey) public pure returns (bytes32)`
  - **Summary**: Generates a hash for Merkle Tree verification from the given leaf data.

- **_setEjector**:
  - **Interface**: `function _setEjector(address _ejector) internal`
  - **Summary**: Internal method to safely update the ejector address.

- **_ejectByStrikes**:
  - **Interface**: `function _ejectByStrikes(KeyStrikes calldata keyStrikes, bytes memory pubkey, uint256 value, address refundRecipient) internal`
  - **Summary**: Internal logic to manage node ejection based on performance threshold.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSFeeOracle.sol
### Contract: CSFeeOracle
The `CSFeeOracle` contract, inheriting from multiple libraries and interfaces, manages reporting related to fees and strikes on a blockchain system. It defines several ACL roles such as `SUBMIT_DATA_ROLE`, `PAUSE_ROLE`, `RESUME_ROLE`, and `RECOVERER_ROLE`. These roles manage permissions for data submission, pausing and resuming operations, and recovering assets respectively. The contract references fee distribution through `ICSFeeDistributor` and handles strike data using `ICSStrikes`.

### Storage Variables
- **SUBMIT_DATA_ROLE** `bytes32`: A constant defining permissions for data submission related to committee reports.
- **PAUSE_ROLE** `bytes32`: A constant defining permissions to pause oracle report acceptance.
- **RESUME_ROLE** `bytes32`: A constant that allows resuming paused oracle operations.
- **RECOVERER_ROLE** `bytes32`: A constant that grants permission to recover assets.
- **FEE_DISTRIBUTOR** `ICSFeeDistributor`: Immutable reference to the fee distributor.
- **STRIKES** `ICSStrikes`: Immutable reference to the strikes management.
- **_feeDistributor** `ICSFeeDistributor`: Deprecated internal storage for fee distributor reference.
- **_avgPerfLeewayBP** `uint256`: Deprecated internal storage likely for performance leeway.

### Functions
- **constructor**
```solidity
constructor(address feeDistributor, address strikes, uint256 secondsPerSlot, uint256 genesisTime)
```
This initializes the contract with addresses for the fee distributor and strikes, and includes time parameters for the `BaseOracle`.

- **initialize**
```solidity
function initialize(address admin, address consensusContract, uint256 consensusVersion) external
```
Initializes the contract from scratch, requisite roles are assigned, and consensus parameters are set.

- **finalizeUpgradeV2**
```solidity
function finalizeUpgradeV2(uint256 consensusVersion) external
```
Finalizes upgrades by setting the consensus version and clearing deprecated storage.

- **resume**
```solidity
function resume() external
```
Resumes contract operations post-pause, needs `RESUME_ROLE` permission.

- **pauseFor**
```solidity
function pauseFor(uint256 duration) external
```
Pauses contract operations for a given duration, accessible under `PAUSE_ROLE`.

- **submitReportData**
```solidity
function submitReportData(ReportData calldata data, uint256 contractVersion) external
```
Allows submission of report data ensuring the sender is authorized, and processes consensus-aligned data.

- **_handleConsensusReport**
```solidity
function _handleConsensusReport(ConsensusReport memory, uint256, uint256) internal
```
Handles consensus report but remains unimplemented as no async processing is required.

- **_handleConsensusReportData**
```solidity
function _handleConsensusReportData(ReportData calldata data) internal
```
Processes oracle report data utilizing `FEE_DISTRIBUTOR` and `STRIKES` for information related to fees and strikes.

- **_checkMsgSenderIsAllowedToSubmitData**
```solidity
function _checkMsgSenderIsAllowedToSubmitData() internal view
```
Verifies if the sender is permitted to submit data based on consensus or role assignment.

- **_onlyRecoverer**
```solidity
function _onlyRecoverer() internal view
```
Asserts role presence necessary for asset recovery.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSExitPenalties.sol
### CSExitPenalties Contract
The `CSExitPenalties` contract manages penalties and fees related to node operators' exit operations within a curved staking setup. It inherits the `ExitTypes` functionality and implements the `ICSExitPenalties` interface.

#### Constructor
- **Interface**: `constructor(address module, address parametersRegistry, address strikes)`
- **Summary**: Initializes the contract with module, parameters registry, and strikes addresses, each of which must be non-zero.

#### processExitDelayReport
- **Interface**: `function processExitDelayReport(uint256 nodeOperatorId, bytes calldata publicKey, uint256 eligibleToExitInSec) external onlyModule`
- **Summary**: Processes the exit delay report for a node operator. Issues a delay penalty if applicable, and emits `ValidatorExitDelayProcessed`.

#### processTriggeredExit
- **Interface**: `function processTriggeredExit(uint256 nodeOperatorId, bytes calldata publicKey, uint256 withdrawalRequestPaidFee, uint256 exitType) external onlyModule`
- **Summary**: Processes an exit triggered by a node operator. Sets a withdrawal request fee if not already set and emits `TriggeredExitFeeRecorded`.

#### processStrikesReport
- **Interface**: `function processStrikesReport(uint256 nodeOperatorId, bytes calldata publicKey) external onlyStrikes`
- **Summary**: Handles strikes reports for a node operator. Issues a strikes penalty and emits `StrikesPenaltyProcessed`.

#### isValidatorExitDelayPenaltyApplicable
- **Interface**: `function isValidatorExitDelayPenaltyApplicable(uint256 nodeOperatorId, bytes calldata publicKey, uint256 eligibleToExitInSec) external view onlyModule returns (bool)`
- **Summary**: Checks if a delay penalty is applicable for a validator exit based on allowed delay. Returns true if applicable and not previously set.

#### getExitPenaltyInfo
- **Interface**: `function getExitPenaltyInfo(uint256 nodeOperatorId, bytes calldata publicKey) external view returns (ExitPenaltyInfo memory)`
- **Summary**: Retrieves penalty information for a given node operator and public key.

#### _keyPointer
- **Interface**: `function _keyPointer(uint256 nodeOperatorId, bytes calldata publicKey) internal pure returns (bytes32)`
- **Summary**: Generates a unique key for storing penalty information based on node operator ID and public key.

### Storage Variables
- **MODULE (ICSModule)**: References the module interface, set during initialization, immutable.
- **PARAMETERS_REGISTRY (ICSParametersRegistry)**: Points to the parameters registry interface, immutable.
- **ACCOUNTING (ICSAccounting)**: Holds the accounting interface reference, derived from the module.
- **STRIKES (address)**: Address of the strikes, immutable.
- **_exitPenaltyInfo (mapping)**: Stores exit penalty info, using a node operator's ID and public key hash as a key.


## SUMMARY OF FILE: 2025-07-lido-finance/src/VettedGateFactory.sol
The `VettedGateFactory` contract is designed to create instances of `VettedGate` using an ossifiable proxy pattern. This allows a vetted gate implementation to be deployed and managed with an ability to upgrade if needed, while deploying new instances specific to a certain curve and tree. 

### Contract: VettedGateFactory
This contract implements `IVettedGateFactory` interface, providing the functionality to create new `VettedGate` instances.

#### Variable: 
- `VETTED_GATE_IMPL`: An address that points to the implementation of the `VettedGate`. It is immutable and set at the time of the contract's construction.

#### Constructor:
```solidity
constructor(address vettedGateImpl)
```
- **Summary**: Sets the vetted gate implementation address. Ensures the address is not zero to avoid deployment with an invalid implementation.

#### Function: 
- `create`: 
```solidity
function create(uint256 curveId, bytes32 treeRoot, string calldata treeCid, address admin) external returns (address instance)
```
- **Summary**: This function creates a new instance of a `VettedGate` by using an `OssifiableProxy` pointing to the `VETTED_GATE_IMPL`. It initializes the instance and emits a `VettedGateCreated` event to log the address of the newly created vetted gate.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSVerifier.sol
The project documentation provides a comprehensive list of files included in the project, categorized by their location and purpose. The folders primarily consist of script and source (src) files, with additional test files to ensure functionality. These files include scripts for deploying various components in different environments (e.g., Mainnet, DevNet), constants, fork helpers, utilities, and core source files for different modules like CSVerifier, CSExitPenalties, and more. The abstracts and interfaces provide necessary structure and communication standards across various parts of the project. Numerous libraries assist in implementing tasks related to asset management, queue handling, and consensus. Additionally, test files exist to validate code functionality, with mocks provided for easier testing of components.


## SUMMARY OF FILE: 2025-07-lido-finance/src/lib/proxy/OssifiableProxy.sol
### OssifiableProxy Contract:

The `OssifiableProxy` contract extends the `ERC1967Proxy` to include additional administrative features for managing proxy upgrades and ossification. An ossified proxy cannot be upgraded as it locks the admin rights permanently, effectively freezing the current implementation.

#### Contract Definition
- **OssifiableProxy**: Enhances an upgradeable proxy with functionalities to make the proxy immutable, or ossified, which disallows future changes after administration is relinquished.

### Function Summaries:

**onlyAdmin Modifier**
```solidity
modifier onlyAdmin()
```
- Ensures that only the current admin can execute the function, and the proxy is not ossified (admin rights given to the zero address).

**Constructor**
```solidity
constructor(address implementation_, address admin_, bytes memory data_)
```
- Initializes the proxy with the given implementation and admin, and optionally executes a call with setup data.

**receive() Function**
```solidity
receive() external payable
```
- Ensures fallback mechanism for receiving ether, primarily included for compatibility with legacy warning suppression.

**proxy__ossify Function:**
```solidity
function proxy__ossify() external onlyAdmin
```
- Transfers admin rights to the zero address, thereby ossifying the proxy and preventing further upgrades.

**proxy__changeAdmin Function**
```solidity
function proxy__changeAdmin(address newAdmin_) external onlyAdmin
```
- Allows changing the admin within the proxy.

**proxy__upgradeTo Function:**
```solidity
function proxy__upgradeTo(address newImplementation_) external onlyAdmin
```
- Upgrades to a new implementation without additional setup calls.

**proxy__upgradeToAndCall Function:**
```solidity
function proxy__upgradeToAndCall(address newImplementation_, bytes calldata setupCalldata_) external onlyAdmin
```
- Upgrades the proxy while allowing immediate setup calls with provided calldata.

**proxy__getAdmin Function:**
```solidity
function proxy__getAdmin() external view returns (address)
```
- Returns the current admin address of the proxy.

**proxy__getImplementation Function:**
```solidity
function proxy__getImplementation() external view returns (address)
```
- Retrieves the current implementation address.

**proxy__getIsOssified Function:**
```solidity
function proxy__getIsOssified() external view returns (bool)
```
- Checks if the proxy is ossified by verifying if the admin address has been set to zero.

### Storage Variables

**Storage of the contract relies on inherited state management from `ERC1967Proxy` and uses utility functions within `ERC1967Utils` to manage proxy state such as admin and implementation addresses.**


## SUMMARY OF FILE: 2025-07-lido-finance/src/lib/base-oracle/HashConsensus.sol
# HashConsensus Contract Summary

## Contract Overview
HashConsensus is a smart contract that manages an oracle members committee allowing members to achieve consensus on hashed data reports over specified time frames. These time frames are defined by epochs and slots, aligning with Ethereum's consensus layer. This contract ensures that relevant on-chain data is sampled and processed efficiently within these frames, supporting functionalities such as quorum setting, member management, and reporting.

## Functions

### getChainConfig
```solidity
function getChainConfig() external view returns (uint256 slotsPerEpoch, uint256 secondsPerSlot, uint256 genesisTime);
```
Provides the chain parameters that relate epoch and slot to timestamp, crucial for determining timeframes and deadlines within the contract.

### getFrameConfig
```solidity
function getFrameConfig() external view returns (uint256 initialEpoch, uint256 epochsPerFrame, uint256 fastLaneLengthSlots);
```
Returns the current frame configuration including the initial frame epoch, the length of frames in epochs, and the fast lane reporting slots.

### getCurrentFrame
```solidity
function getCurrentFrame() external view returns (uint256 refSlot, uint256 reportProcessingDeadlineSlot);
```
Fetches details about the current reporting frame, providing the reference slot for consensus and the deadline for report processing.

### getIsMember
```solidity
function getIsMember(address addr) external view returns (bool);
```
Checks if the provided address is part of the current oracle committee.

### getMembers
```solidity
function getMembers() external view returns (address[] memory addresses, uint256[] memory lastReportedRefSlots);
```
Retrieves a list of all current members along with the reference slots of their latest reports.

### submitReport
```solidity
function submitReport(uint256 slot, bytes32 report, uint256 consensusVersion) external;
```
Allows oracle members to submit a hash for the specific reference slot, facilitating the consensus process.

## Main Storage Variables

### MANAGE_MEMBERS_AND_QUORUM_ROLE
```solidity
bytes32 public constant MANAGE_MEMBERS_AND_QUORUM_ROLE = keccak256("MANAGE_MEMBERS_AND_QUORUM_ROLE");
```
Access Control List (ACL) role enabling modification of members and quorum.

### DISABLE_CONSENSUS_ROLE
```solidity
bytes32 public constant DISABLE_CONSENSUS_ROLE = keccak256("DISABLE_CONSENSUS_ROLE");
```
ACL role permitting the disabling of consensus mechanisms.

### SLOTS_PER_EPOCH
```solidity
uint64 internal immutable SLOTS_PER_EPOCH;
```
Defines the number of slots per epoch as per chain configuration, pivotal for frame calculations.

### _quorum
```solidity
uint256 internal _quorum;
```
Describes the number of members required to reach consensus, ensuring the oracle's validity and operation.


## SUMMARY OF FILE: 2025-07-lido-finance/src/lib/utils/PausableUntil.sol
### Contract: PausableUntil
The `PausableUntil` contract provides functionality to pause and resume contract operations based on time settings. It allows an indefinite pause or a specific time-limited pause, storing the information in a designated storage slot.

### Storage Variables:
- `RESUME_SINCE_TIMESTAMP_POSITION`: (bytes32) The storage position for storing the resumption timestamp, calculated using a hash.
- `PAUSE_INFINITELY`: (uint256) A constant value used to denote indefinite pause using the maximum possible uint256 value.

### Functions:
- **getResumeSinceTimestamp()**: Returns the timestamp for when the contract will resume, or a special value if paused indefinitely.
  ```solidity
  function getResumeSinceTimestamp() external view returns (uint256);
  ```
- **isPaused()**: Checks if the contract is currently in a paused state.
  ```solidity
  function isPaused() public view returns (bool);
  ```
- **_resume()**: Ends the pause period prematurely and resumes contract operations, if currently paused.
  ```solidity
  function _resume() internal;
  ```
- **_pauseFor(uint256 duration)**: Pause the contract for a specific duration or indefinitely.
  ```solidity
  function _pauseFor(uint256 duration) internal;
  ```
- **_pauseUntil(uint256 pauseUntilInclusive)**: Pauses the contract until a specific future timestamp.
  ```solidity
  function _pauseUntil(uint256 pauseUntilInclusive) internal;
  ```
- **_setPausedState(uint256 resumeSince)**: Helper function to update the paused state with a new resume timestamp.
  ```solidity
  function _setPausedState(uint256 resumeSince) internal;
  ```
- **_checkPaused()**: Validates if the contract is paused, reverts if not.
  ```solidity
  function _checkPaused() internal view;
  ```
- **_checkResumed()**: Validates if the contract is not paused, reverts if it is.
  ```solidity
  function _checkResumed() internal view;
  ```


## SUMMARY OF FILE: 2025-07-lido-finance/src/lib/utils/Versioned.sol
### Versioned Contract Summary

The `Versioned` contract is part of the Lido project, focusing on managing the versioning of contracts. It utilizes the `UnstructuredStorage` library for handling storage slots and manages versions of a contract through events and revert errors.

#### Constructor
- **Purpose**: Initializes the contract by setting the version storage to petrified, preventing any initialization until explicitly set up.
- **Definition**: `constructor()`.

#### Functions

- **getContractVersion**
  - **Interface**: `function getContractVersion() public view returns (uint256)`
  - **Summary**: Retrieves the current contract version stored in the blockchain storage.

- **_initializeContractVersionTo**
  - **Interface**: `function _initializeContractVersionTo(uint256 version) internal`
  - **Summary**: Sets the initial contract version. Ensures the version is non-zero and the slot is not already initialized.

- **_updateContractVersion**
  - **Interface**: `function _updateContractVersion(uint256 newVersion) internal`
  - **Summary**: Validates that the new version is exactly one increment above the current version before updating.

- **_checkContractVersion**
  - **Interface**: `function _checkContractVersion(uint256 version) internal view`
  - **Summary**: Confirms the passed version matches the current contract version.

- **_setContractVersion**
  - **Interface**: `function _setContractVersion(uint256 version) private`
  - **Summary**: Sets the contract version in storage and emits a `ContractVersionSet` event.

#### Storage Variables

- **CONTRACT_VERSION_POSITION**
  - Represents the storage slot key for contract version. Ensures unique storage without conflict.

- **PETRIFIED_VERSION_MARK**
  - Marks the version as petrified, with the highest `uint256` to indicate an uninitialized state.

### Special Events and Errors

- **ContractVersionSet**: Emits when the contract version is set.
- **NonZeroContractVersionOnInit**: Thrown if initialization is attempted on non-zero version.
- **InvalidContractVersion & InvalidContractVersionIncrement**: Errors for invalid version settings.
- **UnexpectedContractVersion**: Error for version mismatch.


## SUMMARY OF FILE: 2025-07-lido-finance/src/VettedGate.sol
### VettedGate Contract

The `VettedGate` contract manages access and control for a staking system with additional functionalities such as referral programs, pausing, and setting Merkle Trees for eligible nodes. It inherits from `IVettedGate`, `AccessControlEnumerableUpgradeable`, `PausableUntil`, and `AssetRecoverer`. 

### Functions

- **initialize**: Initializes contract settings including a curve ID, tree root, and the admin role.  
- **resume**: Resumes contract operations if paused, accessible via the `RESUME_ROLE`.  
- **pauseFor**: Pauses operations for a set duration, protected by `PAUSE_ROLE`.  
- **startNewReferralProgramSeason**: Starts a referral program season with a curve ID and threshold, controlled by `START_REFERRAL_SEASON_ROLE`.  
- **endCurrentReferralProgramSeason**: Ends the active referral program season, requiring `END_REFERRAL_SEASON_ROLE`.  
- **addNodeOperatorETH/StETH/WstETH**: Adds a node operator with specified parameters, using different assets (ETH, StETH, WstETH respectively); includes referral counting.  
- **claimBondCurve/claimReferrerBondCurve**: Claims a bond curve for node operators or referrers, requiring proof via Merkle verification.  
- **setTreeParams**: Sets the Merkle tree root and CID, needs `SET_TREE_ROLE`.  
- **getReferralsCount**: Retrieves the referral count for an address, optionally in a specific season.  
- **getInitializedVersion**: Returns the initialized version number.  
- **isReferrerConsumed/isConsumed**: Checks if a referrer or address has been used.  
- **verifyProof/hashLeaf**: Verifies a Merkle proof or hashes a member address for Merkle checking.

### Storage Variables

- **MODULE**: Immutable reference to `ICSModule` address, representing the staking module.  
- **ACCOUNTING**: Immutable reference to `ICSAccounting`, linked to the `MODULE`.  
- **curveId & referralCurveId**: Store bond curve IDs for different contexts (default and referrals).  
- **treeRoot & treeCid**: Manage the Merkle Tree root and its corresponding CID for validation.  
- **referralsThreshold**: Sets minimum referrals for bond curve eligibility.  
- **isReferralProgramSeasonActive**: Boolean managing referral season status.  
- **referralProgramSeasonNumber**: Tracks current referral program season number.  
- **_consumedAddresses/_referralCounts/_consumedReferrers**: Mapping addresses and referrers for usage tracking and referral counts.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSFeeDistributor.sol
### CSFeeDistributor Contract Summary

#### Contract Definition
`CSFeeDistributor` is a contract designed to manage the distribution of fees in stETH to Node Operators (NOs) and rebate recipients. It integrates with various roles, supports rebates, and maintains a record of distribution history using Merkle proofs for validation.

#### Storage Variables
- **RECOVERER_ROLE**: A constant defining the role necessary to recover assets, initialized to the keccak256 hash of "RECOVERER_ROLE".
- **STETH**: An immutable reference to the stETH contract to handle shares.
- **ACCOUNTING**: The address of the accounting module, immutable for security.
- **ORACLE**: The oracle's address responsible for providing data.
- **treeRoot**: Holds the current Merkle Tree root for fee distribution.
- **treeCid & logCid**: Strings that store IPFS CIDs for distribution logs and trees.
- **distributedShares**: Maps Node Operator IDs to distributed stETH shares, allowing tracking.
- **totalClaimableShares**: Total stETH shares available for NO claims.
- **_distributionDataHistory & distributionDataHistoryCount**: Keeps historical distribution data and a count of these entries.
- **rebateRecipient**: Address set for receiving rebates.

#### Functions
- **constructor**: Sets the stETH, accounting, and oracle addresses. Disables initializers after construction.
- **initialize**: Initializes contract with admin and rebate recipient addresses.
- **finalizeUpgradeV2**: Finishes a contract upgrade by setting the rebate recipient.
- **setRebateRecipient**: Updates the rebate recipient address and emits an event.
- **distributeFees**: Validates proofs and distributes fees to node operators based on cumulative fees and shares.
- **processOracleReport**: Updates state variables based on the oracle's report, including rebate processing.
- **recoverERC20**: Allows authorized recovery of ERC20 tokens, excluding stETH.
- **getInitializedVersion**: Provides the version of initialization.
- **pendingSharesToDistribute**: Returns pending shares for distribution.
- **getHistoricalDistributionData**: Retrieves distribution history data by index.
- **getFeesToDistribute**: Calculates and returns the stETH shares available for distribution.
- **hashLeaf**: Computes the hash for Merkle tree leaves.
- **_setRebateRecipient**: Internal function for setting the rebate recipient.


## SUMMARY OF FILE: 2025-07-lido-finance/src/CSEjector.sol
# CSEjector Contract

The `CSEjector` contract in Solidity is part of a system that handles the management and ejection of validator keys in a staking environment. This contract relies on roles for access control and includes functionalities around pausing operations, recovering assets, and managing exits for validators.

## Contract Overview
### contract CSEjector
Inherits from multiple contracts such as `ICSEjector`, `ExitTypes`, `AccessControlEnumerable`, `PausableUntil`, and `AssetRecoverer`. It centralizes the ejection process for validators through both voluntary or automated mechanisms, leveraging strict access control.

## Important Functions

### function resume()
```solidity
function resume() external onlyRole(RESUME_ROLE)
```
Resumes operations after a pause, requiring the caller to have the `RESUME_ROLE`.

### function pauseFor()
```solidity
function pauseFor(uint256 duration) external onlyRole(PAUSE_ROLE)
```
Pauses operations for a specified duration, requiring the `PAUSE_ROLE`.

### function voluntaryEject()
```solidity
function voluntaryEject(uint256 nodeOperatorId, uint256 startFrom, uint256 keysCount, address refundRecipient) external payable whenResumed
```
Ejects a specified number of validator keys starting from a given index for a node operator, ensuring all keys are non-withdrawn and deposited, potentially refunding the caller.

### function voluntaryEjectByArray()
```solidity
function voluntaryEjectByArray(uint256 nodeOperatorId, uint256[] calldata keyIndices, address refundRecipient) external payable whenResumed
```
Allows ejection of non-sequential validator keys by their indices, ensuring they are deposited and non-withdrawn before proceeding.

### function ejectBadPerformer()
```solidity
function ejectBadPerformer(uint256 nodeOperatorId, uint256 keyIndex, address refundRecipient) external payable whenResumed onlyStrikes
```
Ejects a specific validator key if it is deemed a 'bad performer', restricted to `STRIKES`.

### function triggerableWithdrawalsGateway()
```solidity
function triggerableWithdrawalsGateway() public view returns (ITriggerableWithdrawalsGateway)
```
Retrieves the triggerable withdrawals gateway address from the module's lido locator, enabling withdrawals.

## Storage Variables

### PAUSE_ROLE
```solidity
bytes32 public constant PAUSE_ROLE = keccak256("PAUSE_ROLE")
```
Role identifier for pausing operations.

### RESUME_ROLE
```solidity
bytes32 public constant RESUME_ROLE = keccak256("RESUME_ROLE")
```
Role identifier for resuming operations.

### RECOVERER_ROLE
```solidity
bytes32 public constant RECOVERER_ROLE = keccak256("RECOVERER_ROLE")
```
Role identifier for asset recovery operations.

### STAKING_MODULE_ID
```solidity
uint256 public immutable STAKING_MODULE_ID
```
The identifier for the staking module being used in the contract operations.

### MODULE
```solidity
ICSModule public immutable MODULE
```
Represents the associated module through which various functionalities like retrieving node operator owner's information are accessed.

### STRIKES
```solidity
address public immutable STRIKES
```
Address designated for receiving strike actions in validator management.


## SUMMARY OF FILE: 2025-07-lido-finance/src/PermissionlessGate.sol
### PermissionlessGate Contract
The `PermissionlessGate` contract, which extends `AccessControlEnumerable` and `AssetRecoverer`, implements the `IPermissionlessGate` interface for adding new Node Operators without restrictions in a decentralized staking module setup.

#### Key Storage Variables
- **`bytes32 public constant RECOVERER_ROLE`**: A hashed constant for the recoverer role, providing access control for asset recovery functionalities.
- **`uint256 public immutable CURVE_ID`**: Stores the default bond curve ID from the accounting contract, ensuring consistency across gates.
- **`ICSModule public immutable MODULE`**: Holds a reference to the Staking Module, represented by an ICSModule interface.

#### Key Functions
- **`constructor`**: Initializes the contract with a module and admin address, setting the MODULE and CURVE_ID variables, and granting the default administrator role.
- **`function addNodeOperatorETH`**: Adds a new Node Operator using ETH for validator keys, managing properties, and registering through the MODULE, and returns the operator ID.
- **`function addNodeOperatorStETH`**: Similar to `addNodeOperatorETH`, but uses staked ETH (StETH) instead, requiring a permit for the transaction.
- **`function addNodeOperatorWstETH`**: Works like `addNodeOperatorStETH`, employing wrapped staked ETH (WstETH) for key addition, also permitting transaction details.
- **`function _onlyRecoverer`**: Internal function enforcing recoverer role checks.


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


