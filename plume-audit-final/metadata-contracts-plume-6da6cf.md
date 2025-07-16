
## SLITHER GENERATED METADATA 


## List of Files in Src Folder
Plume.sol
PlumeStaking.sol
PlumeStakingRewardTreasury.sol
facets/AccessControlFacet.sol
facets/ManagementFacet.sol
facets/RewardsFacet.sol
facets/StakingFacet.sol
facets/ValidatorFacet.sol
helpers/ArbSys.sol
interfaces/IAccessControl.sol
interfaces/IDateTime.sol
interfaces/IDeploy.sol
interfaces/IDeployer.sol
interfaces/IPlumeStaking.sol
interfaces/IPlumeStakingRewardTreasury.sol
interfaces/ISupraRouterContract.sol
lib/PlumeErrors.sol
lib/PlumeEvents.sol
lib/PlumeRewardLogic.sol
lib/PlumeRoles.sol
lib/PlumeStakingStorage.sol
lib/PlumeValidatorLogic.sol
mocks/MockPUSD.sol
proxy/MockPUSDProxy.sol
proxy/PlumeProxy.sol
proxy/PlumeStakingProxy.sol
proxy/PlumeStakingRewardTreasuryProxy.sol
proxy/RaffleProxy.sol
proxy/SPINProxy.sol
spin/DateTime.sol
spin/Raffle.sol
spin/Spin.sol
## contracts/plume/src/PlumeStaking.sol SUMMARY OF MAIN FILES
### **PlumeStaking Contract**

The `PlumeStaking` contract is a part of the Plume Staking Diamond, a Ethereum solidity-based smart contract framework leveraging the SolidStateDiamond proxy pattern.

#### **Function: initializePlume**
```solidity
function initializePlume(address initialOwner, uint256 minStake, uint256 cooldown, uint256 maxSlashVoteDuration, uint256 maxValidatorCommission) external virtual onlyOwner
```
**Description**: This is an initializer function for setting up the Plume staking contract. It sets various staking parameters and allows ownership to be transferred if a different initial owner is provided. The function includes checks for ensuring valid parameters are set, such as ensuring non-zero stake values and proper cooldown duration relative to slash vote duration.

#### **Function: isInitialized**
```solidity
function isInitialized() external view returns (bool)
```
**Description**: Returns whether the Plume staking contract has been initialized, ensuring deployment consistency.

#### **Storage Variables**:
- **PlumeStakingStorage.Layout storage $**: Utilizes storage layout pattern for managing contract data, ensuring structured access and modification.


## contracts/plume/src/facets/ValidatorFacet.sol SUMMARY OF MAIN FILES
# ValidatorFacet Contract

## Overview
The `ValidatorFacet` contract is designed to manage validators in a blockchain network. It includes functionalities for adding, updating, and managing validator status, capacity, commission, and handling slash votes. It integrates multiple imported libraries and contracts for comprehensive access control, security, and reward logic.

## Key Functions

### addValidator
```solidity
function addValidator(uint16 validatorId, uint256 commission, address l2AdminAddress, address l2WithdrawAddress, string calldata l1ValidatorAddress, string calldata l1AccountAddress, address l1AccountEvmAddress, uint256 maxCapacity)
```
Initializes a new validator with specified parameters. Checks that IDs and addresses are unique and that commission does not exceed the allowed maximum.

### setValidatorCapacity
```solidity
function setValidatorCapacity(uint16 validatorId, uint256 maxCapacity)
```
Sets a new maximum staking capacity for a given validator, ensuring the validator is not inactive or slashed.

### setValidatorStatus
```solidity
function setValidatorStatus(uint16 validatorId, bool newActiveStatus)
```
Changes the active status of a validator. It manages commission settlements and updates reward logic checkpoints.

### setValidatorCommission
```solidity
function setValidatorCommission(uint16 validatorId, uint256 newCommission)
```
Allows the validator's admin to update commission rates, with reward logic adjustments and checks for commission limits.

### requestCommissionClaim & finalizeCommissionClaim
```solidity
function requestCommissionClaim(uint16 validatorId, address token)
function finalizeCommissionClaim(uint16 validatorId, address token)
```
These functions manage commission claims for validators, involving a timelock mechanism. Only callable by the validator admin, these functions handle reward distribution and ensure conditions are met before finalizing a claim.

### voteToSlashValidator & slashValidator
```solidity
function voteToSlashValidator(uint16 maliciousValidatorId, uint256 voteExpiration)
function slashValidator(uint16 validatorId)
```
These functions allow the vote-based slashing of malicious validators. They include mechanisms for vote casting and require unanimity for executing slashing.

## Storage Variables

### Validators Storage
```solidity
PlumeStakingStorage.Layout internal layout
```
Holds validator data, including mapping from validator IDs to their information structures, tracking of votes, and the state of validator commissions.

### Reward & Voting Management
- **rewardTokens**: Array of tokens eligible for rewards.
- **slashVoteCounts**: Keeps track of current valid vote counts for slashing.
- **slashingVotes**: Stores expiration timestamps for slash votes per validator.

The contract effectively centralizes validator management, ensuring network integrity through robust governance and reward mechanisms.


## contracts/plume/src/facets/StakingFacet.sol SUMMARY OF MAIN FILES
### Summary of the `StakingFacet` Contract

#### Contract Definition:
`StakingFacet`: The contract handles staking, unstaking, and withdrawal operations for users using the PLUME token on validators. It supports staking setup, validation, and capacity limits using various internal and public functions.

#### Functions:
- **_checkValidatorSlashedAndRevert**: Ensures a validator isn't slashed by checking the storage. Throws `ActionOnSlashedValidatorError` if the validator is slashed.
- **_validateValidatorForStaking**: Checks if a validator exists and is active, otherwise throws relevant errors.
- **_validateStakeAmount**: Verifies if the stake amount meets specific requirements, throwing an error if not.
- **_validateStaking**: Combines validator and stake amount validations into one.
- **_validateValidatorCapacity**: Checks if the validator's capacity isn't exceeded with the current stake amount.
- **_validateValidatorPercentage**: Ensures the stake doesn't surpass the validator's percentage limits related to the total staked amount.
- **_validateCapacityLimits**: Performs both capacity and percentage validation checks for a validator.
- **_validateValidatorForUnstaking**: Validates if a validator can be used for unstaking operations.
- **_performStakeSetup**: Sets up the necessary user, validator, and amount checks before staking.
- **_performRestakeWorkflow**: Handles restaking logic from cooled/parked funds after performing necessary validations.

#### Public Functions:
- **stake**: Stakes PLUME tokens to a specific validator using the caller's wallet funds.
- **restake**: Restakes PLUME tokens under cooldown or parked for a particular validator.
- **unstake**: Unstakes PLUME tokens from a specific validator either as a full or partial unstake, depending on the parameters.
- **withdraw**: Allows users to withdraw matured parked PLUME tokens.
- **stakeOnBehalf**: Enables staking on behalf of another user for a specific validator.
- **restakeRewards**: Restakes a user's pending rewards to a selected validator after verification.
- **View Functions**: Several functions, like `amountStaked`, `amountCooling`, `amountWithdrawable`, etc., are available for querying different staking-related statuses.

#### Storage Variables:
The contract utilizes `PlumeStakingStorage` for storing validators, users, stakes, rewards, cooling periods, and more, ensuring efficient state management and data retrieval across its operations.


## contracts/plume/src/facets/ManagementFacet.sol SUMMARY OF MAIN FILES
### Contract: ManagementFacet
The `ManagementFacet` contract in the code snippet is a component of a modular architecture using Solidity. It handles administrative functions related to parameter settings and contract fund management. This contract interfaces with a staking infrastructure where administrators can manage staking parameters like minimum stake amounts, cooldown intervals, and validator commission rates. It employs modifiers to ensure roles are checked for executing functions, such as using `onlyRole` for access control.

### Function: setMinStakeAmount
#### Interface
```solidity
function setMinStakeAmount(uint256 _minStakeAmount) external onlyRole(PlumeRoles.ADMIN_ROLE)
```
#### Summary
Set the minimum amount of tokens that can be staked by a user. Function requires an ADMIN_ROLE and reverts if `_minStakeAmount` is zero. The function updates the `minStakeAmount` in storage and emits a `MinStakeAmountSet` event on success.

### Function: setCooldownInterval
#### Interface
```solidity
function setCooldownInterval(uint256 interval) external onlyRole(PlumeRoles.ADMIN_ROLE)
```
#### Summary
Adjusts the cooldown interval required before a user can unstake tokens, ensuring it is greater than the slashing vote duration. It updates the `cooldownInterval` in storage and emits `CooldownIntervalSet` upon successful execution.

### Function: adminWithdraw
#### Interface
```solidity
function adminWithdraw(address token, uint256 amount, address recipient) external onlyRole(PlumeRoles.TIMELOCK_ROLE) nonReentrant
```
#### Summary
Allows an admin to withdraw tokens or native PLUME from the contract. Checks address validity and ensures the amount is within balance limits. Supports ERC20 withdrawals using SafeERC20. Emits `AdminWithdraw` after a successful transaction.

### Function: getMinStakeAmount
#### Interface
```solidity
function getMinStakeAmount() external view returns (uint256)
```
#### Summary
Fetches the current minimum stake amount required for users, returning the value stored in `PlumeStakingStorage`.

### Function: getCooldownInterval
#### Interface
```solidity
function getCooldownInterval() external view returns (uint256)
```
#### Summary
Provides the current cooldown interval for unstaking, extracting the value from the storage layout.

### Function: setMaxSlashVoteDuration
#### Interface
```solidity
function setMaxSlashVoteDuration(uint256 duration) external onlyRole(PlumeRoles.ADMIN_ROLE)
```
#### Summary
Sets the maximum duration allowed for slashing votes, ensuring it doesn’t exceed the cooldown interval or commission claim timelock. Emits `MaxSlashVoteDurationSet` after updating storage.

### Function: setMaxAllowedValidatorCommission
#### Interface
```solidity
function setMaxAllowedValidatorCommission(uint256 newMaxRate) external onlyRole(PlumeRoles.TIMELOCK_ROLE)
```
#### Summary
Updates system-wide max commission rate for validators, enforcing the rate on existing validators and emitting `MaxAllowedValidatorCommissionSet`.

### Function: setMaxCommissionCheckpoints
#### Interface
```solidity
function setMaxCommissionCheckpoints(uint16 newLimit) external onlyRole(PlumeRoles.ADMIN_ROLE)
```
#### Summary
Limits the number of commission checkpoints to prevent gas exhaustion, requiring a minimum of 10. Triggers the `MaxCommissionCheckpointsSet` event.

### Function: setMaxValidatorPercentage
#### Interface
```solidity
function setMaxValidatorPercentage(uint256 newPercentage) external onlyRole(PlumeRoles.ADMIN_ROLE)
```
#### Summary
Controls the percentage of total stakes a validator can hold; checks that it doesn’t exceed 100%. Updates storage and emits `MaxValidatorPercentageUpdated`.

### Function: pruneCommissionCheckpoints
#### Interface
```solidity
function pruneCommissionCheckpoints(uint16 validatorId, uint256 count) external onlyRole(PlumeRoles.ADMIN_ROLE)
```
#### Summary
Allows manual removal of old commission checkpoints for validators but ensures at least one checkpoint remains for current rate definition. Emits `CommissionCheckpointsPruned`.

### Function: pruneRewardRateCheckpoints
#### Interface
```solidity
function pruneRewardRateCheckpoints(uint16 validatorId, address token, uint256 count) external onlyRole(PlumeRoles.ADMIN_ROLE)
```
#### Summary
Enables removal of out-of-date reward rate checkpoints for specified validators and tokens, with similar constraints as commission pruning.

### Function: adminClearValidatorRecord
#### Interface
```solidity
function adminClearValidatorRecord(address user, uint16 slashedValidatorId) external onlyRole(PlumeRoles.ADMIN_ROLE)
```
#### Summary
Admin clearance of user stakes associated with a slashed validator. This handles cleaning up internal records post-slash, ensuring administrative consistency.

### Function: adminBatchClearValidatorRecords
#### Interface
```solidity
function adminBatchClearValidatorRecords(address[] calldata users, uint16 slashedValidatorId) external onlyRole(PlumeRoles.ADMIN_ROLE)
```
#### Summary
Similar to `adminClearValidatorRecord`, but processes multiple users. Careful with gas usage and ensures every user’s stake records with slashed validators are cleared.

### Variables
- **PlumeStakingStorage.Layout storage $:** Primary storage layout for staking-related data: manages global & user-specific states for staking, cooldown, rewards.
- **PlumeRoles.ADMIN_ROLE & TIMELOCK_ROLE:** Represents roles necessary for executing admin-level functions.


## contracts/plume/src/facets/RewardsFacet.sol SUMMARY OF MAIN FILES
## RewardsFacet Contract
**Definition**: `RewardsFacet` is a Solidity smart contract that manages reward tokens associated with staking, including adding/removing tokens, setting reward rates, and handling the claiming process for these rewards. It inherits from `ReentrancyGuardUpgradeable` and `OwnableInternal`.

### Storage Variables:
- **BASE**: `uint256 internal constant BASE = 1e18;` - A constant used as a base for calculations.
- **MAX_REWARD_RATE**: `uint256 internal constant MAX_REWARD_RATE = 3171 * 1e9;` - Maximum allowable reward rate.
- **TREASURY_STORAGE_POSITION**: `bytes32 internal constant TREASURY_STORAGE_POSITION = keccak256("plume.storage.RewardTreasury");` - Position of the treasury in storage.

### Functions:

#### Internal Functions:
- **getTreasuryAddress**: Gets the treasury address from storage.
  ```solidity
  function getTreasuryAddress() internal view returns (address)
  ```
- **setTreasuryAddress**: Sets the treasury address.
  ```solidity
  function setTreasuryAddress(address _treasury) internal
  ```
- **_earned**: Calculates earned rewards for a user per token and validator.
  ```solidity
  function _earned(address user, address token, uint16 validatorId) internal returns (uint256 rewards)
  ```
- **_calculateTotalEarned**: Calculates the total rewards earned by a user across all validators.
  ```solidity
  function _calculateTotalEarned(address user, address token) internal returns (uint256 totalEarned)
  ```

#### External Functions:
- **setTreasury**: Set the treasury address, `ADMIN` role only.
  ```solidity
  function setTreasury(address _treasury) external onlyRole(PlumeRoles.TIMELOCK_ROLE)
  ```
- **addRewardToken**: Adds a token as a reward token, checks conditions, emits `RewardTokenAdded`.
  ```solidity
  function addRewardToken(address token, uint256 initialRate, uint256 maxRate) external onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
  ```
- **removeRewardToken**: Removes a reward token, prevents further claims, emits `RewardTokenRemoved`.
  ```solidity
  function removeRewardToken(address token) external onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
  ```
- **setRewardRates**: Sets reward rates for an array of tokens, maintains checkpoint updates.
  ```solidity
  function setRewardRates(address[] calldata tokens, uint256[] calldata rewardRates_) external onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
  ```
- **setMaxRewardRate**: Sets the maximum reward rate for a given token.
  ```solidity
  function setMaxRewardRate(address token, uint256 newMaxRate) external onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
  ```
- **claim**: Overloaded version to claim rewards for a specific validator or for all tokens.
  ```solidity
  function claim(address token, uint16 validatorId) external nonReentrant returns (uint256)
  function claim(address token) external nonReentrant returns (uint256)
  ```
- **claimAll**: Claims all rewards for all tokens.
  ```solidity
  function claimAll() external nonReentrant returns (uint256[] memory)
  ```

### View Functions:
- **earned**: View-only function that returns total earned rewards for a user and token.
  ```solidity
  function earned(address user, address token) external view returns (uint256)
  ```
- **isRewardToken**: Checks if a token is an active reward token.
  ```solidity
  function isRewardToken(address token) external view returns (bool)
  ```
- **getMaxRewardRate**: Returns the maximum reward rate for a token.
  ```solidity
  function getMaxRewardRate(address token) external view returns (uint256)
  ```
- **getRewardRate**: Retrieves the current reward rate for a token.
  ```solidity
  function getRewardRate(address token) external view returns (uint256)
  ```


## contracts/plume/src/facets/AccessControlFacet.sol SUMMARY OF MAIN FILES
### AccessControlFacet Contract
The `AccessControlFacet` contract manages roles using SolidState's AccessControl framework. It optimizes role management and uses local definitions for roles to facilitate clarity. 

#### Contract Definition:
- **AccessControlFacet**: Integrates with SolidState's core to manage roles via a diamond architecture for enhancing reusability.

### Key Functions
- **initializeAccessControl()**: Initializes roles and sets up role hierarchy, can be called only once typically by the diamond owner.
  - `function initializeAccessControl() external` 
- **hasRole(bytes32 role, address account)**: Checks if an account has a particular role.
  - `function hasRole(bytes32 role, address account) external view override returns (bool)`
- **getRoleAdmin(bytes32 role)**: Retrieves the admin role for a specified role.
  - `function getRoleAdmin(bytes32 role) external view override returns (bytes32)`
- **grantRole(bytes32 role, address account)**: Grants a role to an account, requires caller to possess admin privileges for said role.
  - `function grantRole(bytes32 role, address account) external override`
- **revokeRole(bytes32 role, address account)**: Removes a role from an account, enforcing admin verification.
  - `function revokeRole(bytes32 role, address account) external override`
- **renounceRole(bytes32 role, address account)**: Allows individuals to renounce roles, applicable only to self.
  - `function renounceRole(bytes32 role, address account) external override`
- **setRoleAdmin(bytes32 role, bytes32 adminRole)**: Alters admin role settings for a specified role; needs ADMIN_ROLE authority.
  - `function setRoleAdmin(bytes32 role, bytes32 adminRole) external override`

### Storage Variables
- **DEFAULT_ADMIN_ROLE, ADMIN_ROLE, UPGRADER_ROLE, VALIDATOR_ROLE, REWARD_MANAGER_ROLE, TIMELOCK_ROLE**: Various role definitions modeled after PlumeRoles. 
- **accessControlFacetInitialized**: A boolean indicator from `PlumeStakingStorage` used to track if the initialization has been executed.


## contracts/plume/src/proxy/PlumeStakingRewardTreasuryProxy.sol SUMMARY OF MAIN FILES
### PlumeStakingRewardTreasuryProxy Contract Summary

**Contract Definition:**
PlumeStakingRewardTreasuryProxy is a proxy contract extending from OpenZeppelin's ERC1967Proxy. It's designed as a proxy specifically for PlumeStakingRewardTreasury.

**Constructor**: `constructor(address logic, bytes memory data) ERC1967Proxy(logic, data)`
- **Summary**: Initializes the proxy with the logic contract address and any initialization data required. It inherits the constructor from OpenZeppelin’s ERC1967Proxy, which handles delegating calls and upgrades.

**Functions and Variables:**

1. **PROXY_NAME**
   - **Definition**: `bytes32 public constant PROXY_NAME = keccak256("PlumeStakingRewardTreasuryProxy");`
   - **Summary**: A constant variable holding a unique name hash for the proxy. Used to ensure that the bytecode for each named proxy remains unique.

2. **receive() External Function**
   - **Definition**: `receive() external payable { }`
   - **Summary**: A fallback function that allows the contract to accept ETH. This feature ensures the proxy can handle Ether transactions efficiently.

The proxy allows for the separation of upgradeable logic from data, facilitating contract upgrades.


## contracts/plume/src/proxy/SPINProxy.sol SUMMARY OF MAIN FILES
### Contract: SpinProxy\nSpinProxy is a proxy contract for the "Faucet" system. It inherits from \`ERC1967Proxy\`, which is a part of OpenZeppelin's contracts library for upgradable proxies. This contract is authored by Eugene Y. Q. Shen and Alp Guneysel.\n\n#### Functions\n- **Constructor: `constructor(address logic, bytes memory data)`**\n  Initializes a new instance of the `ERC1967Proxy` with the address of the logic contract and any initialization data. It sets up the proxy with the implementation logic provided.\n  \n  **Interface:**\n  ```solidity\n  constructor(address logic, bytes memory data)\n  ```\n\n- **Receive Function: `receive()`**\n  A special Solidity function to enable receiving Ether. It does not have an implementation; its presence alone allows the contract to accept Ether transfers.\n  \n  **Interface:**\n  ```solidity\n  receive() external payable\n  ```\n\n#### Storage Variables\n- **PROXY_NAME: `bytes32 public constant PROXY_NAME`**\n  A constant variable that holds the unique name of the proxy, ensuring unique bytecode by using the hash of the string "SpinProxy". This helps in identifying the proxy distinctly.\n  \n  **Definition:**\n  ```solidity\n  bytes32 public constant PROXY_NAME = keccak256("SpinProxy");\n  ```


## contracts/plume/src/proxy/PlumeStakingProxy.sol SUMMARY OF MAIN FILES
## File Summary: PlumeStakingProxy.sol

### Contract Definition
**Contract Name:** PlumeStakingProxy
**Author:** Eugene Y. Q. Shen, Alp Guneysel
**Inherits:** ERC1967Proxy from OpenZeppelin
**Purpose:** This contract serves as a proxy for the PlumeStaking smart contract, using the ERC1967 proxy standard.

### Storage Variables
- **PROXY_NAME**: `bytes32 public constant PROXY_NAME = keccak256("PlumeStakingProxy")`
  - Explanation: A unique bytecode identifier for the PlumeStakingProxy to ensure each named proxy in the ecosystem is distinct.

### Constructor
- **Constructor(address logic, bytes memory data)**
```solidity
constructor(address logic, bytes memory data) ERC1967Proxy(logic, data) { }
```
  - Explanation: Initializes the proxy with the given logic contract address and initialization bytecode.

### Function Definitions
- **receive() external payable**
```solidity
receive() external payable { }
```
  - Explanation: This function enables the contract to accept Ether directly, without calling any specific function. It adheres to the Solidity `receive()` function standard for payable fallback execution when Ether is sent.


## contracts/plume/src/proxy/RaffleProxy.sol SUMMARY OF MAIN FILES
## RaffleProxy Contract Summary

### Contract Definition
- **RaffleProxy**: This contract extends OpenZeppelin’s ERC1967Proxy, which is a transparent proxy compatible with EIP-1967. It serves as a proxy for a Raffle smart contract, managing vital aspects of the proxy pattern.

### Function and Interface Summaries
- **Constructor**: `constructor(address logic, bytes memory data)`
  - Deploys a new RaffleProxy contract with a given logic contract address and initialization data. The superclass constructor is called with these parameters, ensuring the proxy interacts with the correct underlying logic.

- **receive**: `receive() external payable`
  - Handles ETH transactions sent to the proxy. In this implementation, it reverts any ETH transfers with an error `ETHTransferUnsupported`, indicating that transferring ETH to this proxy is not permitted.

### Storage Variables
- **PROXY_NAME**: `bytes32 public constant PROXY_NAME`
  - Stores the unique identifier for this proxy, ensuring it has distinct bytecode. It's defined as a constant with the hash value of "RaffleProxy" to avoid clashes with other proxies.


## contracts/plume/src/proxy/PlumeProxy.sol SUMMARY OF MAIN FILES
### Contract: PlumeProxy
PlumeProxy is a smart contract that acts as a proxy for the Plume system. It extends the functionality of OpenZeppelin's `ERC1967Proxy` to facilitate upgradeability. The contract implements specific error handling for unsupported ETH transfers and defines a unique identifier for the proxy.

### Summary of Functions:

#### Constructor
- **Function:** `constructor(address logic, bytes memory data)`
- **Summary:** The constructor initializes the PlumeProxy by invoking the `ERC1967Proxy` constructor with the specified `logic` address and initialization `data`. It sets up the proxy with the initial implementation logic.
- **Interface:**
  ```solidity
  constructor(address logic, bytes memory data)
  ```

#### receive Function
- **Function:** `receive() external payable`
- **Summary:** This is a fallback function that triggers if ether is sent to the contract. It immediately reverts by throwing an `ETHTransferUnsupported` error, thereby blocking any ETH transfer to the proxy contract.
- **Interface:**
  ```solidity
  receive() external payable
  ```

### Storage Variables:

#### PROXY_NAME
- **Definition:** `bytes32 public constant PROXY_NAME = keccak256("PlumeProxy")`
- **Summary:** `PROXY_NAME` is a constant storage variable that stores a unique identifier for the PlumeProxy. It ensures that every named proxy has unique bytecode by using a hash of the proxy's name.


## contracts/plume/src/PlumeStakingRewardTreasury.sol SUMMARY OF MAIN FILES
### PlumeStakingRewardTreasury Contract Summary

This contract, **PlumeStakingRewardTreasury**, is responsible for managing reward tokens within the PlumeStaking system. It's designed to be upgradeable using the UUPS pattern and employs AccessControl for role-based permissions. The contract holds reward tokens and allows for their secure distribution.

#### Contract Definition
The contract inherits from multiple modules for functionality including initialization, access control, reentrancy protection, and upgradeability:
- `Initializable`
- `AccessControlUpgradeable`
- `ReentrancyGuardUpgradeable`
- `UUPSUpgradeable`

#### Constants and Storage Variables
- `PLUME_NATIVE`: An address constant representing the native PLUME token.
- `DISTRIBUTOR_ROLE`, `ADMIN_ROLE`, `UPGRADER_ROLE`: Defined roles for access control.
- `_rewardTokens`: An array storing addresses of reward tokens.
- `_isRewardToken`: A mapping to check if a token is registered.

#### Functions
- `initialize(address admin, address distributor)`: Sets roles for admin and distributor, ensuring only these roles can manage and distribute rewards.
- `_authorizeUpgrade(address newImplementation)`: Used for authorizing contract upgrades, restricted by the `UPGRADER_ROLE`.
- `addRewardToken(address token)`: Allows adding a new token to the reward list by an admin.
- `distributeReward(address token, uint256 amount, address recipient)`: Distributes specified token amounts to addresses, ensuring safe transfers.
- `getRewardTokens()`: Returns a list of all reward tokens managed by the treasury.
- `getBalance(address token)`: Returns the contract's balance for a specific token.
- `isRewardToken(address token)`: Checks if a given token is registered as a reward token.

### Usage
1. **Roles Management:** Admin can set roles and manage the distribution of the rewards.
2. **Token Management:** Admins can register reward tokens and ensure they are safe to use.
3. **Reward Distribution:** Distributors can allocate and send rewards to designated recipients, facilitated securely by the contract's functions.


## contracts/plume/src/Plume.sol SUMMARY OF MAIN FILES
### Contract: Plume
The `Plume` contract is an upgradeable ERC20 token contract that serves as the governance token for the Plume Network. It includes features for minting, burning, and pausing of tokens. It utilizes OpenZeppelin's upgradeable libraries, allowing for UUPS upgradeability, access control, and token functionalities.

### Functions and Their Brief Descriptions:

**initialize(address owner)**
- **Summary:** Initializes the contract, sets up initial roles for the admin, and prepares the token for use.
- **Interface:** `function initialize(address owner) public initializer`
- **Details:** Calls initializer methods for the ERC20 utility functions and grants various roles (admin, minter, burner, pauser, upgrader) to the owner.

**reinitialize()**
- **Summary:** Reinitializes the contract with the symbol "$PLUME".
- **Interface:** `function reinitialize() public reinitializer(1) onlyRole(UPGRADER_ROLE)`
- **Details:** Sets token symbol to "PLUME" and is restricted to those with the UPGRADER role.

**_authorizeUpgrade(address newImplementation)**
- **Summary:** Ensures that only authorized roles can upgrade the contract.
- **Interface:** `function _authorizeUpgrade(address newImplementation) internal override onlyRole(UPGRADER_ROLE)`
- **Details:** This function is called when the contract is upgraded, restricting access to the UPGRADER role.

**_update(address from, address to, uint256 value)**
- **Summary:** Internal function that updates balances post token transfer.
- **Interface:** `function _update(address from, address to, uint256 value) internal override(ERC20Upgradeable, ERC20PausableUpgradeable)`
- **Details:** Overrides internal update functionality from the ERC20 and ERC20Pausable base implementations.

**mint(address to, uint256 amount)**
- **Summary:** Mints new Plume tokens to a specified address, restricted by the MINTER role.
- **Interface:** `function mint(address to, uint256 amount) external onlyRole(MINTER_ROLE)`
- **Details:** Directly calls `_mint` from the ERC20 interface, ensuring role-based access.

**burn(address from, uint256 amount)**
- **Summary:** Allows those with BURNER role to burn tokens from a specified address.
- **Interface:** `function burn(address from, uint256 amount) external onlyRole(BURNER_ROLE)`
- **Details:** Facilitates burning of tokens and decreases total supply as per role permissions.

**pause()**
- **Summary:** Pauses all token transfers, callable only by those with the PAUSER role.
- **Interface:** `function pause() external onlyRole(PAUSER_ROLE)`
- **Details:** Utilizes `_pause` from ERC20Pausable, stopping all token transactions temporarily.

**unpause()**
- **Summary:** Reenables token transfers, callable only by those with the PAUSER role.
- **Interface:** `function unpause() external onlyRole(PAUSER_ROLE)`
- **Details:** Calls `_unpause` from ERC20Pausable to resume normal operations.

### Storage Variables:

**UPGRADER_ROLE**
- **Definition:** `bytes32 public constant UPGRADER_ROLE = keccak256("UPGRADER_ROLE");`
- **Explanation:** Specifies the role for users authorized to upgrade the contract.

**MINTER_ROLE**
- **Definition:** `bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");`
- **Explanation:** Designates the role for minting new Plume tokens.

**BURNER_ROLE**
- **Definition:** `bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");`
- **Explanation:** Designates the role for burning Plume tokens.

**PAUSER_ROLE**
- **Definition:** `bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");`
- **Explanation:** Designates the role for pausing and unpausing the contract.


## contracts/plume/src/spin/DateTime.sol SUMMARY OF MAIN FILES
### DateTime Contract Summary

The `DateTime` contract provides date and time utilities for Ethereum smart contracts. It includes functions to handle common date-related operations, such as checking leap years, calculating days in a month, and parsing timestamps into more human-readable date and time components. This contract is based on a utility library available on GitHub.

#### Contract: DateTime
The `DateTime` contract facilitates the manipulation and extraction of date and time information from timestamps in the Ethereum blockchain.

#### Function: isLeapYear
```solidity
function isLeapYear(uint16 year) public pure returns (bool)
```
This function checks whether a given year is a leap year.

#### Function: leapYearsBefore
```solidity
function leapYearsBefore(uint256 year) public pure returns (uint256)
```
Calculates the number of leap years before a given year.

#### Function: getDaysInMonth
```solidity
function getDaysInMonth(uint8 month, uint16 year) public pure returns (uint8)
```
Returns the number of days in a specified month.

#### Function: parseTimestamp
```solidity
function parseTimestamp(uint256 timestamp) internal pure returns (_DateTime memory dt)
```
Parses a timestamp into its components: year, month, day, hour, minute, second, and weekday.

#### Function: getYear
```solidity
function getYear(uint256 timestamp) public pure returns (uint16)
```
Extracts the year from a timestamp.

#### Function: getMonth
```solidity
function getMonth(uint256 timestamp) public pure returns (uint8)
```
Gets the month from a timestamp.

#### Function: getDay
```solidity
function getDay(uint256 timestamp) public pure returns (uint8)
```
Retrieves the day from a timestamp.

#### Function: getHour
```solidity
function getHour(uint256 timestamp) public pure returns (uint8)
```
Gets the hour from a timestamp.

#### Function: getMinute
```solidity
function getMinute(uint256 timestamp) public pure returns (uint8)
```
Returns the minute from a timestamp.

#### Function: getSecond
```solidity
function getSecond(uint256 timestamp) public pure returns (uint8)
```
Retrieves the second from a timestamp.

#### Function: getWeekday
```solidity
function getWeekday(uint256 timestamp) public pure returns (uint8)
```
Calculates the weekday of a given timestamp.

#### Function: toTimestamp
```solidity
function toTimestamp(uint16 year, uint8 month, uint8 day, uint8 hour, uint8 minute, uint8 second) public pure returns (uint256 timestamp)
```
Converts date and time components into a Unix timestamp.

#### Function: getWeekNumber
```solidity
function getWeekNumber(uint256 timestamp) public pure returns (uint8)
```
Determines the week number of a given timestamp.

#### Function: getDaysSinceYearStart
```solidity
function getDaysSinceYearStart(uint16 year, uint8 month, uint8 day) internal pure returns (uint256)
```
Calculates the number of days since the start of the year.


## contracts/plume/src/spin/Spin.sol SUMMARY OF MAIN FILES
### Contract Summary: `Spin`

The `Spin` contract is a multi-faceted, upgradable reward mechanism that engages users through daily spins to potentially win a variety of rewards. It relies on a daily streak system and random number generation via the Supra Router Contract. The spin activity is tied to a campaign that offers weekly jackpot prizes which can be controlled via admin roles.

### Key Functions:

- **`initialize`**: 
  `function initialize(address supraRouterAddress, address dateTimeAddress) public initializer;`
  Initializes the contract, sets up AccessControl, relevant contract roles, jackpot probabilities, default settings, and associates with the Supra Router and DateTime contracts.

- **`startSpin`**: 
  `function startSpin() external payable whenNotPaused canSpin;`
  Allows users to start the spin contingent on payment and non-pending spin status, generating a randomness request through the Supra Router.

- **`handleRandomness`**: 
  `function handleRandomness(uint256 nonce, uint256[] memory rngList) external onlyRole(SUPRA_ROLE) nonReentrant;`
  Handles the randomness callback, determining and distributing rewards, and updating user data.

- **`determineReward`**: 
  `function determineReward(uint256 randomness, uint256 streakForReward) internal view returns (string memory, uint256);`
  Determines the reward category and amount based on randomness and user's streak.

- **`getCurrentWeek`**: 
  `function getCurrentWeek() public view returns (uint256);`
  Calculates the current week within the promotional campaign based on the start date.

- **`setJackpotProbabilities`**, **`setJackpotPrizes`**, **`setCampaignStartDate`**, etc.: 
  Functions allowing admins to configure various aspects of the spin and reward system, including the spin price, jackpot probabilities, and more.

### Storage Variables:

- **`RewardProbabilities` struct**: 
  Defines ranges for plumeToken, raffleTicket, and PP thresholds to classify rewards.

- **`userData` (mapping)**: 
  Maps each user's address to their `UserData` struct, which contains their spin history and rewards balance.

- **`jackpotProbabilities` (uint256[7])**: 
  Probabilities for jackpots on each day of the week.

- **`jackpotPrizes` (mapping)**: 
  Maps weeks to their respective jackpot prize amounts.

- **`supraRouter` (ISupraRouterContract)**: 
  Reference to the Supra Router contract for generating randomness.

This comprehensive reward-based system allows modifications without interrupting user activities via its careful use of role-based permissions and upgradability through UUPS proxy patterns.


## contracts/plume/src/spin/Raffle.sol SUMMARY OF MAIN FILES
### Raffle Contract
The `Raffle` contract in Solidity is designed to manage raffle events where users can participate using tickets. It implements upgradeable proxies through the `UUPSUpgradeable` mechanism and is based on OpenZeppelin's `AccessControl` for role management. Interfaces for interaction include `ISpin` and `ISupraRouterContract`.

#### Contract Definition
The `Raffle` contract facilitates raffle prize management, ticket spending, and winner selection using VRFs (Verifiable Random Functions).

#### Functions
- **initialize**: Sets up the contract, assigning initial roles and interface references:
  ```solidity
  function initialize(address _spinContract, address _supraRouter) public initializer;
  ```
- **addPrize**: Admin-only function to add new raffle prizes:
  ```solidity
  function addPrize(string calldata name, string calldata description, uint256 value, uint256 quantity) external onlyRole(ADMIN_ROLE);
  ```
- **editPrize**: Edits existing prize details:
  ```solidity
  function editPrize(uint256 prizeId, string calldata name, string calldata description, uint256 value, uint256 quantity) external onlyRole(ADMIN_ROLE) prizeIsActive(prizeId);
  ```
- **removePrize**: Deactivates a raffle prize:
  ```solidity
  function removePrize(uint256 prizeId) external onlyRole(ADMIN_ROLE) prizeIsActive(prizeId);
  ```
- **spendRaffle**: Allows users to spend tickets to enter a raffle for a prize:
  ```solidity
  function spendRaffle(uint256 prizeId, uint256 ticketAmount) external prizeIsActive(prizeId);
  ```
- **requestWinner**: Admin-only function to request a VRF-selected winner:
  ```solidity
  function requestWinner(uint256 prizeId) external onlyRole(ADMIN_ROLE);
  ```
- **handleWinnerSelection**: Callback function handling post-VRF processing:
  ```solidity
  function handleWinnerSelection(uint256 requestId, uint256[] memory rng) external onlyRole(SUPRA_ROLE);
  ```
- **cancelWinnerRequest**: Cancels pending VRF requests to escape failures:
  ```solidity
  function cancelWinnerRequest(uint256 prizeId) external onlyRole(ADMIN_ROLE);
  ```
- **claimPrize**: Facilitates users claiming their awarded prizes:
  ```solidity
  function claimPrize(uint256 prizeId, uint256 winnerIndex) external;
  ```
- **getPrizeDetails**: Displays prize attributes and details:
  ```solidity
  function getPrizeDetails(uint256 prizeId) external view returns (PrizeWithTickets);
  ```

#### Storage Variables
- **ADMIN_ROLE/SUPRA_ROLE**: Defines role-based access for admin and VRF interaction:
  ```solidity
  bytes32 public constant ADMIN_ROLE;
  bytes32 public constant SUPRA_ROLE;
  ```
- **spinContract/supraRouter**: Addresses of external contracts (ISpin and ISupraRouterContract):
  ```solidity
  ISpin public spinContract;
  ISupraRouterContract public supraRouter;
  ```
- **prizes/prizeIds**: Track prize information and identifiers:
  ```solidity
  mapping(uint256 => Prize) public prizes;
  uint256[] public prizeIds;
  ```
- **user tracking**: Manages user entries and winnings data:
  ```solidity
  mapping(uint256 => mapping(address => bool)) public userHasEnteredPrize;
  mapping(uint256 => uint256) public totalUniqueUsers;
  ```


 ## DOCUMENTATION: 

