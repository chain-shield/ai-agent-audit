
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
### PlumeStaking Contract
The PlumeStaking contract is a solidity contract acting as the main entry point for Plume Staking Diamond, extending the functionality of SolidStateDiamond. It's designed for initializing and handling plumbing staking operations with added security and flexibility features.

### Storage Variables
1. **PlumeStakingStorage.Layout storage $**: 
   Reference to a layout in PlumeStakingStorage library, storing staking-related parameters (like `minStakeAmount`, `cooldownInterval`, and `initialized`).

### Functions

- **initializePlume Function**
  ```solidity
  function initializePlume(address initialOwner, uint256 minStake, uint256 cooldown) external virtual onlyOwner
  ```
  Initializes staking parameters including the owner, minimum staking amount, and cooldown period. It uses checks for previous initialization and valid stake amount. Ownership can be transferred from the deployer if needed.

- **isInitialized Function**
  ```solidity
  function isInitialized() external view returns (bool)
  ```
  Checks if the Plume-specific initialization has been performed already, returning a boolean status from the storage.


## contracts/plume/src/facets/ValidatorFacet.sol SUMMARY OF MAIN FILES
### File List Overview
The provided document lists various Solidity source files and related contracts, libraries, and interfaces used in a blockchain project. These are organized by function, such as staking or validator management, and are further divided into categories like `facets`, `helpers`, `interfaces`, `lib`, and `proxy`.

### `ValidatorFacet` Contract Summary
This contract is part of a decentralized staking system that manages validators for a blockchain network. It allows for adding, updating, and maintaining validators, handling their commissions, capacity, status, and vote for slashing malicious validators. It leverages several external libraries and interfaces to enhance functionality and ensure security.

#### Functions and Features:
- **`addValidator()`**: Adds a new validator, checking commission rates and administrator assignments to ensure everything is set correctly.
- **`setValidatorCapacity()`**: Sets the maximum staking capacity for a validator, ensuring their activity status is checked.
- **`setValidatorStatus()`**: Modifies validator activity status, requiring specific roles.
- **`setValidatorCommission()`**: Adjusts the validator's commission rate, calculating accrued commissions before updating.
- **`setValidatorAddresses()`**: Allows updates to validator and related addresses while maintaining consistency in assignments.
- **`requestCommissionClaim()`**: Validator admins can request commission payment, triggering a timelock.
- **`finalizeCommissionClaim()`**: Finalizes claims post-timelock, ensuring treasury transfers if conditions meet.
- **`_cleanupExpiredVotes()`**: Internal function handling vote expiration management.
- **`voteToSlashValidator()`**: Active validators can vote to slash a malicious validator, details are stored and votes checked.
- **`slashValidator()`**: An admin-slash function that executes if the vote threshold is met, ensuring no rewards are lost.
- **`forceSettleValidatorCommission()`**: Manual commission settlement for validators.
- **`getValidatorInfo()`** & **`getValidatorStats()`**: Provides information on validators like stake and status.
- **`getUserValidators()`**: Lists validators that a user has staked with.

#### Key Storage Variables:
- **`ValidatorListData`**: Contains validator details, including ID, total staked, and commission.
- **`validatorStorage`**: Manages storage for validator data across the contract, ensuring security.
- **`voteStorage`**: Manages voting details for slash operations including expiration checks.

The contract employs SafeERC20 for secure token transfers and ReentrancyGuardUpgradeable to prevent reentrant calls, safeguarding critical operations. The usage of PlumeStakingStorage ensures a structured and efficient handling of validator data.


## contracts/plume/src/facets/StakingFacet.sol SUMMARY OF MAIN FILES
### Contract Summary: StakingFacet
This is a Solidity contract named `StakingFacet` authored by Eugene Y. Q. Shen and Alp Guneysel. It manages core user actions related to staking, unstaking, and withdrawal functions within a staking mechanism for `PLUME` tokens. It uses several error and event definitions from `PlumeErrors.sol` and `PlumeEvents.sol` libraries and integrates OpenZeppelin's `ReentrancyGuardUpgradeable` for security.

### Function Summaries
1. **_checkValidatorSlashedAndRevert(uint16 validatorId):**
   - **Purpose:** Validates if a validator has been slashed, and reverts to throw an error if true.
   - **Interface:** `internal view`

2. **_validateValidatorForStaking(uint16 validatorId):**
   - **Purpose:** Ensures a validator is both existent and active, not slashed, before proceeding with staking.
   - **Interface:** `internal view`

3. **_validateStakeAmount(uint256 amount):**
   - **Purpose:** Checks stake amount against minimum requirements; throws error if below minimum or zero.
   - **Interface:** `internal view`

4. **_validateStaking(uint16 validatorId, uint256 amount):**
   - **Purpose:** Confirms that both validator and stake amount are valid before staking can proceed.
   - **Interface:** `internal view`

5. **_validateValidatorCapacity(uint16 validatorId, uint256 stakeAmount):**
   - **Purpose:** Confirms the proposed staking will not exceed the validator's capacity limits.
   - **Interface:** `internal view`

6. **_validateValidatorPercentage(uint16 validatorId):**
   - **Purpose:** Ensures a validator’s staked percentage is within permissible total network limits.
   - **Interface:** `internal view`

7. **_validateCapacityLimits(uint16 validatorId, uint256 stakeAmount):**
   - **Purpose:** Integrates capacity and percentage checks to validate staking constraints.
   - **Interface:** `internal view`

8. **_validateValidatorForUnstaking(uint16 validatorId):**
   - **Purpose:** Validates that a validator exists and is not slashed in the context of unstaking.
   - **Interface:** `internal view`

9. **_performStakeSetup(address user, uint16 validatorId, uint256 stakeAmount):**
   - **Purpose:** Conducts initial setup and validation for new stakes, initializes rewards and updates stake amounts.
   - **Interface:** `internal returns (bool isNewStake)`

10. **_performRestakeWorkflow(address user, uint16 validatorId, uint256 amount, string memory fromSource):**
   - **Purpose:** Handles restaking from cooled or parked funds with necessary validations and updates.
   - **Interface:** `internal`

11. **stake(uint16 validatorId):**
   - **Purpose:** Lets users stake PLUME using wallet funds to a specific validator.
   - **Interface:** `external payable returns (uint256)`

12. **restake(uint16 validatorId, uint256 amount):**
   - **Purpose:** Allows users to restake cooled or parked PLUME funds to a validator.
   - **Interface:** `external nonReentrant`

13. **unstake(uint16 validatorId):**
   - **Purpose:** Unstakes entire PLUME balance for a specific validator, moving it to the cooling phase.
   - **Interface:** `external returns (uint256)`

14. **unstake(uint16 validatorId, uint256 amount):**
   - **Purpose:** Unstakes a specified amount of PLUME from a validator.
   - **Interface:** `external returns (uint256)`

15. **withdraw():**
   - **Purpose:** Allows users to withdraw all PLUME in the parked balance after clearing cooldowns.
   - **Interface:** `external`

16. **stakeOnBehalf(uint16 validatorId, address staker):**
   - **Purpose:** Allows staking on behalf of another user.
   - **Interface:** `external payable returns (uint256)`

17. **restakeRewards(uint16 validatorId):**
   - **Purpose:** Restakes all pending rewards for a user to a specific validator.
   - **Interface:** `external nonReentrant returns (uint256)`

### Storage Variables
- Uses a PlumeStakingStorage.Layout struct extensively through shorthand `PlumeStakingStorage.layout()` to access staking related state data.

Overall, the contract is designed to manage various staking operations, ensuring validator and stake amount validations, handling restakes from parked and cooled positions, updating necessary state variables, and emitting relevant events for transparency.


## contracts/plume/src/facets/ManagementFacet.sol SUMMARY OF MAIN FILES
### Contract Summary: ManagementFacet
The `ManagementFacet` contract handles administrative tasks within a Plume staking system, mainly focusing on setting parameters and managing contract funds. It extends functionalities like `ReentrancyGuardUpgradeable` and `OwnableInternal` and interacts heavily with a diamond structure of other facets for role-based control.

### Function Summaries

- **modifier onlyRole(bytes32 _role)**
  
  Permits only users with a specified role to execute the function, utilizing the `AccessControlFacet`. Fails for unauthorized users.

- **function setMinStakeAmount(uint256 _minStakeAmount) external**

  Updates the minimum staking amount, requiring `ADMIN_ROLE`. Throws error if the new amount is zero. Emits a `MinStakeAmountSet` event.

- **function setCooldownInterval(uint256 interval) external**

  Sets the unstaking cooldown period, mandating `ADMIN_ROLE`. Validates against zero and certain existing parameters. Emits a `CooldownIntervalSet` event.

- **function adminWithdraw(address token, uint256 amount, address recipient) external**

  Allows TIMLOCK_ROLE, non-reentrant withdrawal of specified token or native PLUME tokens. Errors for invalid inputs or insufficient funds.

- **function getMinStakeAmount() external view returns (uint256)**

  Returns the current minimum stake amount.

- **function getCooldownInterval() external view returns (uint256)**

  Retrieves the current cooldown interval.

- **function setMaxSlashVoteDuration(uint256 duration) external**

  Sets maximum duration for slashing votes, needing `ADMIN_ROLE`. Checks compatibility against cooldown interval.

- **function setMaxAllowedValidatorCommission(uint256 newMaxRate) external**

  Updates the system-wide maximum validator commission, requiring `TIMELOCK_ROLE`, with a limit of 50%.

- **function adminClearValidatorRecord(address user, uint16 slashedValidatorId) external**

  Clears stale records for a user's slashed validator, requiring `ADMIN_ROLE`. Ensures consistent state logic.

- **function adminBatchClearValidatorRecords(address[] calldata users, uint16 slashedValidatorId) external**

  Processes stale records cleanup in batch for a slashed validator. Only works for an `ADMIN_ROLE` with validation checks.


## contracts/plume/src/facets/RewardsFacet.sol SUMMARY OF MAIN FILES
### RewardsFacet Contract

**Definition:** `RewardsFacet` handles the management of reward tokens, sets reward rates, calculates rewards, and allows users to claim their earned rewards. It is built upon the `ReentrancyGuardUpgradeable` and `OwnableInternal` to prevent reentry attacks and manage ownership, respectively.

#### Storage Variables

- **BASE:** Constant multiplier set to `1e18` for scaling purposes.
- **MAX_REWARD_RATE:** Upper limit to the reward rate, set to `3171 * 1e9`.
- **TREASURY_STORAGE_POSITION:** A `bytes32` value used to access the treasury address from storage.

#### Key Functions

- **getTreasuryAddress():** *Returns the address of the treasury contract using low-level `assembly` code.*

- **setTreasuryAddress(address _treasury):** *Integrates a new treasury address into storage. Checks for zero address scenarios and emits `TreasurySet` event.*

- **onlyRole(bytes32 _role):** *Ensures that the executing sender has the specified role. Throws an `Unauthorized` error if not.*

- **_earned(address user, address token, uint16 validatorId):** *Calculates the earned reward for a specific user-validator-token combination. Returns reward amount.*

- **_calculateTotalEarned(address user, address token):** *Iterates through validators to sum total earned rewards for a user across validators.*

- **setTreasury(address _treasury):** *External function restricted to ADMIN role, allowing only trusted entities to change the treasury address.*

- **addRewardToken(address token):** *Allows a REWARD_MANAGER_ROLE to add new tokens to the pool of reward tokens.*

- **removeRewardToken(address token):** *Allows removal of a token by a reward manager, effectively halting future claims on it.*

- **setRewardRates(address[] calldata tokens, uint256[] calldata rewardRates_):** *Updates reward rates for specified tokens if they are valid.*

- **setMaxRewardRate(address token, uint256 newMaxRate):** *Permits adjustment of the maximum reward rate for a token, respecting constraints.*

- **claim(address token, uint16 validatorId):** *Enables users to claim rewards for a specific validator.*

- **claim(address token):** *Claims rewards from all active validators for a given token.*

- **claimAll():** *Claims all tokens rewards from all validators for the user.*


## contracts/plume/src/facets/AccessControlFacet.sol SUMMARY OF MAIN FILES
### AccessControlFacet Contract
**Purpose**: This contract implements access control functionality using SolidState's library for role management. It's designed to manage specific roles and their privileges within a diamond contract architecture, leveraging PlumeRoles and utilizing storage layouts for shared states.

### Key Storage Variables
- `DEFAULT_ADMIN_ROLE`, `ADMIN_ROLE`, `UPGRADER_ROLE`, `VALIDATOR_ROLE`, `REWARD_MANAGER_ROLE`, `TIMELOCK_ROLE`: **Type**: `bytes32`. These constants are used to define roles and their hierarchy within the contract, structured using PlumeRoles.
  
### Key Functions

- **initializeAccessControl**
  **Purpose**: Initializes the contract's role structure, designating the caller as the admin. This is guarded against re-initialization.
  **Interface**: `function initializeAccessControl() external`
  **Summary**: Sets up all roles with the caller as the primary admin and assigns several roles initially, ensuring that the facet is initialized only once.

- **hasRole**
  **Purpose**: Checks if an account holds a specific role.
  **Interface**: `function hasRole(bytes32 role, address account) external view returns (bool)`

- **getRoleAdmin**
  **Purpose**: Retrieves the admin role for a specified role.
  **Interface**: `function getRoleAdmin(bytes32 role) external view returns (bytes32)`

- **grantRole**
  **Purpose**: Assigns a role to an account, contingent on having the corresponding administrative role.
  **Interface**: `function grantRole(bytes32 role, address account) external`

- **revokeRole**
  **Purpose**: Removes a role from an account, requiring the administrative privilege for the role.
  **Interface**: `function revokeRole(bytes32 role, address account) external`

- **renounceRole**
  **Purpose**: Allows the account to renounce their own role.
  **Interface**: `function renounceRole(bytes32 role, address account) external`
  **Summary**: Enables self-removal from assigned roles.

- **setRoleAdmin**
  **Purpose**: Alters the admin role of a particular role, accessible only by the admin role.
  **Interface**: `function setRoleAdmin(bytes32 role, bytes32 adminRole) external`


## contracts/plume/src/proxy/PlumeStakingRewardTreasuryProxy.sol SUMMARY OF MAIN FILES
The `PlumeStakingRewardTreasuryProxy` is a Solidity contract that acts as a proxy for the `PlumeStakingRewardTreasury`. It extends the `ERC1967Proxy` from the OpenZeppelin library, facilitating upgradeable proxy architectures. 

### Contract Definition:
- **PlumeStakingRewardTreasuryProxy**: This proxy contract offers the ability to safely update the implementation logic while preserving state, a common practice for upgradeable smart contracts.

### Key Functions:
- **constructor**:
  - **Summary**: Initializes the proxy with logic address and optional initialization data.
  - **Interface**: `constructor(address logic, bytes memory data) ERC1967Proxy(logic, data)`

- **receive**:
  - **Summary**: Enables the proxy to receive Ether (`ETH`), making it capable of handling ETH transactions.
  - **Interface**: `receive() external payable`

### Storage Variable:
- **PROXY_NAME**:
  - **Definition**: A constant bytes32 variable that ensures each proxy's bytecode is unique by assigning a specific name.
  - **Explanation**: Helps differentiate between proxies, especially if there are multiple instances in the smart contract ecosystem.


## contracts/plume/src/proxy/SPINProxy.sol SUMMARY OF MAIN FILES
## SpinProxy Contract Summary

The `SpinProxy` contract inherits from OpenZeppelin's `ERC1967Proxy`. It acts as a proxy for an upgradeable smart contract design. It enables the separation of logic and storage, allowing the logic contract to be upgraded without losing state data.

### Functionality
- **Constructor:** The constructor initializes the proxy with a logic contract address and initialization data, calling the parent `ERC1967Proxy` constructor.
  ```solidity
  constructor(address logic, bytes memory data) ERC1967Proxy(logic, data) { }
  ```
  *Summary:* Sets up the proxy with the initial logic contract and any initialization parameters.

- **Receive:** The contract includes a payable `receive` function.
  ```solidity
  receive() external payable { }
  ```
  *Summary:* Allows the contract to accept Ether transfers.

### Storage Variables
- **PROXY_NAME:** A constant bytes32 storing the unique proxy name.
  ```solidity
  bytes32 public constant PROXY_NAME = keccak256("SpinProxy");
  ```
  *Summary:* Ensures each named proxy has unique bytecode by storing a unique identifier as a hash.


## contracts/plume/src/proxy/PlumeStakingProxy.sol SUMMARY OF MAIN FILES
### Contract: PlumeStakingProxy
The `PlumeStakingProxy` contract is a smart contract proxy for the PlumeStaking contract. Developed by Eugene Y. Q. Shen and Alp Guneysel, it extends OpenZeppelin's ERC1967Proxy for upgradeability, enabling logic contract substitution while persisting storage.

#### Constructor
- **Interface**: `constructor(address logic, bytes memory data) ERC1967Proxy(logic, data)`
- **Summary**: Initializes the proxy with a given logic address for the PlumeStaking logic contract and optional initialization data. Inherits constructor functionality from ERC1967Proxy to set up the proxy's storage slots.

#### Storage Variables
- **PROXY_NAME**: `bytes32 public constant PROXY_NAME = keccak256("PlumeStakingProxy");`
  - **Summary**: Stores the hashed bytecode unique identifier for this proxy, ensuring uniqueness among different proxies.

#### Functions
- **receive()**
  - **Interface**: `receive() external payable`
  - **Summary**: A fallback function to allow the proxy to receive and handle ETH transfers directly.


## contracts/plume/src/proxy/RaffleProxy.sol SUMMARY OF MAIN FILES
### RaffleProxy Contract Summary
The `RaffleProxy` is a smart contract functioning as a proxy that employs the ERC1967 proxy pattern provided by OpenZeppelin. It serves to delegate calls to a logic contract and ensures that its proxy bytecode is unique by attaching a specific proxy name. This simplistic storage layout inherently lacks storage variables and operations, focusing solely on forwarding operations.

#### Contract Definition
**Contract Name:** RaffleProxy
**Inheritance:** ERC1967Proxy

#### Constructor: `constructor(address logic, bytes memory data)`
- **Summary:** Initializes the `RaffleProxy` with a given logic contract and data.
- **Interface:**
  ```solidity
  constructor(address logic, bytes memory data)
  ```
- **Details:**
  - Calls the ERC1967Proxy constructor with parameters for proxy initialization.

#### Fallback Function: `receive()`
- **Summary:** Overrides the `receive` function to prevent ether from being transferred to the contract.
- **Interface:**
  ```solidity
  receive() external payable
  ```
- **Details:**
  - The function will always revert with the `ETHTransferUnsupported` error if called.

#### Storage Variable: `PROXY_NAME`
- **Definition:** `bytes32 public constant PROXY_NAME = keccak256("RaffleProxy");`
- **Summary:** Uniquely identifies the proxy with the hash of the "RaffleProxy" string. Ensures different proxies have distinct bytecode.


## contracts/plume/src/proxy/PlumeProxy.sol SUMMARY OF MAIN FILES
### PlumeProxy Contract
**PlumeProxy** is a proxy contract that extends OpenZeppelin's `ERC1967Proxy`, providing upgradeability features via the ERC-1967 standard. It's designed for the Plume project to allow upgrades without changing the proxy interface.

#### Constructor
```solidity
constructor(address logic, bytes memory data)
```
- Initializes with the logic contract address and optional initialization data.
- Ensures standard proxy operations using `ERC1967Proxy`'s initialization.

#### receive
```solidity
receive() external payable
```
- Intended to receive ETH, but usage reverts with `ETHTransferUnsupported` error.
- Enforces that direct ETH transfers to the proxy are not supported.

#### Storage Variables
- `error ETHTransferUnsupported`: Custom error for unsupported ETH transfer attempts.
- `bytes32 public constant PROXY_NAME`: Immutable storage variable with identifier for the proxy, ensuring unique bytecode.

### Summary
The PlumeProxy contract is a straightforward ERC1967-based proxy designed for upgradeable deployments. Direct ETH transfers are disabled by reverting such transactions, aligning with the logic of a proxy that manages other functionality within the underlying Plume logic contracts.


## contracts/plume/src/PlumeStakingRewardTreasury.sol SUMMARY OF MAIN FILES
### PlumeStakingRewardTreasury Contract Summary
The `PlumeStakingRewardTreasury` contract is responsible for managing and distributing reward tokens within the PlumeStaking system. It is built using the UUPS upgradeable pattern, ensuring it can be updated as needed. Access control is implemented via OpenZeppelin's `AccessControlUpgradeable`, providing roles for admins, distributors, and upgraders.

### Functions
- **constructor()**: Disables initializers to prevent the contract from being initialized multiple times.
  ```solidity
  constructor()
  ```
- **initialize(admin, distributor)**: Initializes the contract with specified admin and distributor roles.
  ```solidity
  function initialize(address admin, address distributor)
  ```
- **_authorizeUpgrade(newImplementation)**: Ensures only authorized users can upgrade the contract.
  ```solidity
  function _authorizeUpgrade(address newImplementation)
  ```
- **addRewardToken(token)**: Adds a new token to the reward list if not already added.
  ```solidity
  function addRewardToken(address token)
  ```
- **distributeReward(token, amount, recipient)**: Distributes specified reward token to a recipient.
  ```solidity
  function distributeReward(address token, uint256 amount, address recipient)
  ```
- **getRewardTokens()**: Returns all managed reward tokens.
  ```solidity
  function getRewardTokens()
  ```
- **getBalance(token)**: Retrieves the balance of a specific token in the treasury.
  ```solidity
  function getBalance(address token)
  ```
- **isRewardToken(token)**: Checks if a token is registered as a reward token.
  ```solidity
  function isRewardToken(address token)
  ```
- **receive()**: Allows the contract to receive native PLUME tokens.
  ```solidity
  receive()
  ```

### Storage Variables
- **PLUME_NATIVE**: Stores the native PLUME token address.
  ```solidity
  address public constant PLUME_NATIVE
  ```
- **DISTRIBUTOR_ROLE, ADMIN_ROLE, UPGRADER_ROLE**: Role identifiers for access control.
  ```solidity
  bytes32 public constant [ROLE]
  ```
- **_rewardTokens**: Array storing listed reward tokens.
  ```solidity
  address[] private _rewardTokens
  ```
- **_isRewardToken**: Mapping tracking registered reward tokens.
  ```solidity
  mapping(address => bool) private _isRewardToken
  ```


## contracts/plume/src/Plume.sol SUMMARY OF MAIN FILES
### Plume Contract
Plume is an ERC20 token serving as a governance token for the Plume Network. It utilizes various OpenZeppelin libraries for upgradeability, access control, and additional ERC20 functionalities such as burning, pausing, and permits. The contract is protected by roles for actions such as minting, burning, pausing, and upgrades.

#### Storage Variables
- **UPGRADER_ROLE**: `bytes32` defining the role for contract upgrades.
- **MINTER_ROLE**: `bytes32` defining the role for minting tokens.
- **BURNER_ROLE**: `bytes32` defining the role for burning tokens.
- **PAUSER_ROLE**: `bytes32` defining the role for pausing/unpausing the contract.

#### Functions
- **`constructor()`**: Disables initializers to prevent misuse.
- **`initialize(address owner)`**: Initializes the contract, granting all roles to the specified owner address, and prepares the token functionalities.
- **`reinitialize()`**: Allows for reinitialization of the token's symbol, accessible only by the upgrader.
- **`_authorizeUpgrade(address newImplementation)`**: Ensures only authorized roles can upgrade the contract.
- **`_update(address from, address to, uint256 value)`**: Overrides token transfer mechanics to comply with ERC20Pausable.
- **`mint(address to, uint256 amount)`**: Mints new tokens to a specified address, restricted to minters.
- **`burn(address from, uint256 amount)`**: Burns tokens from a specified address, restricted to burners.
- **`pause()`**: Pauses the contract, accessible only by pausers.
- **`unpause()`**: Unpauses the contract, accessible only by pausers.


## contracts/plume/src/spin/DateTime.sol SUMMARY OF MAIN FILES
**`DateTime` Contract Summary**

The `DateTime` contract provides various utilities for handling date and time calculations in Ethereum smart contracts. It follows the UNIX timestamp standard and includes functions for determining leap years, converting timestamps to date components, and computing time intervals.

**Storage Variables:**
- `_DateTime`: Struct that holds date and time components like year, month, day, hour, minute, second, and weekday.
- Constants for time calculations: `DAY_IN_SECONDS`, `YEAR_IN_SECONDS`, `LEAP_YEAR_IN_SECONDS`, `HOUR_IN_SECONDS`, `MINUTE_IN_SECONDS`, `ORIGIN_YEAR`. These constants are used to convert dates to seconds and vice versa.

**Function Summaries:**

- `function isLeapYear(uint16 year) public pure returns (bool)`:
  - Calculates whether a given year is a leap year.
  - Uses standard leap year calculation rules to return `true` or `false`.

- `function leapYearsBefore(uint256 year) public pure returns (uint256)`:
  - Computes the number of leap years before a specified year.
  - Iteratively subtracts 1 from the year and calculates the division results to determine the number of leap years.

- `function getDaysInMonth(uint8 month, uint16 year) public pure returns (uint8)`:
  - Determines the number of days in a specified month and considers leap years for February.

- `function parseTimestamp(uint256 timestamp) internal pure returns (_DateTime memory dt)`:
  - Parses a given timestamp into the _DateTime struct, filling in all date components.
  - Uses iterative calculation to account for leap years and accumulate time components.

- `function getYear(uint256 timestamp) public pure returns (uint16)`:
  - Extracts the year from a provided timestamp.
  - Adjusts for leap years and calculates based on the origin year and accumulated seconds.

- `function getMonth(uint256 timestamp) public pure returns (uint8)`:
  - Returns the month component of the given timestamp.
  - Relies on the internal parseTimestamp method.

- `function getDay(uint256 timestamp) public pure returns (uint8)`:
  - Returns the day component from a timestamp using the internal parse system.

- `function getHour(uint256 timestamp) public pure returns (uint8)`:
  - Extracts the hour from a given timestamp.
  - Utilizes modulus operation to provide the hour in a 24-hour format.

- `function getMinute(uint256 timestamp) public pure returns (uint8)`:
  - Returns the minute component of a timestamp. Uses modulus division by 60.

- `function getSecond(uint256 timestamp) public pure returns (uint8)`:
  - Retrieves the second part from the timestamp using modulus to split time into second-based increments.

- `function getWeekday(uint256 timestamp) public pure returns (uint8)`:
  - Uses simple modulus arithmetic to determine the weekday for a given timestamp, indexed from Monday.

- `function toTimestamp(uint16 year, uint8 month, uint8 day) public pure returns (uint256 timestamp)`:
  - Several overloads convert complete date-time information into a single timestamp.
  - Accumulates time components from the earliest defined year.

- `function getWeekNumber(uint256 timestamp) public pure returns (uint8)`:
  - Determines the week number of the year for a particular timestamp.
  - Computes based on days since the start of the year and adjusts for weekdays.

- `function getDaysSinceYearStart(uint16 year, uint8 month, uint8 day) internal pure returns (uint256)`:
  - Calculates the total day count since the year's start, excluding the current day.


## contracts/plume/src/spin/Spin.sol SUMMARY OF MAIN FILES
The `Spin` contract is a gamified contract function that allows users to spin and potentially earn rewards such as jackpot, plume tokens, raffle tickets, or PP (Bonus Points). It incorporates security and upgrade modules from OpenZeppelin and interfaces for date-time and randomness through the `SupraRouterContract` and `DateTime` modules.

### Contract Summary:
- **Initialize**: Sets initial roles, links contracts, and defines default values.
- **canSpin**: Modifier to ensure the user hasn't spun today.
- **startSpin**: Initiates a spin, processing payment and requesting randomness.
- **handleRandomness**: Processes the random number and calculates rewards.
- **determineReward**: Determines categories and amounts of rewards.
- **_computeStreak**: Calculates and updates user spin streaks.
- **Spending, admin control, and settings**: Functions to manage spins, raffle tickets, and configurations such as probabilities, streak requirements, and spin prices.

### Functions:
- `initialize`: Initializes contract and default values with addresses for supraRouter and dateTime.
- `startSpin`: Initiates a user's spin with necessary checks and payment.
- `handleRandomness`: Callback handling to give user a reward based on randomness received.
- `determineReward`: Internal logic to assign rewards considering randomness and user state.

### Storage:
- `userData`: Tracks individual user spin and reward data.
- `jackpotProbabilities`: Daily jackpot probabilities.
- `jackpotPrizes`: Weekly jackpots for a 12-week campaign.
- **etc**.


## contracts/plume/src/spin/Raffle.sol SUMMARY OF MAIN FILES
### Raffle Contract Overview

The `Raffle` contract is an upgradeable Solidity smart contract that facilitates a raffle system, integrating with spinning and VRF services to manage prize distribution and raffle ticket usage.

### Contract Definition
The `Raffle` contract implements `Initializable`, `AccessControlUpgradeable`, and `UUPSUpgradeable`, utilizing OpenZeppelin libraries for proxy and access control mechanics.

### Key Functions
- **initialize**: Sets up the contract, initializing roles and connecting to external contracts. 
  ```solidity
  function initialize(address _spinContract, address _supraRouter) public initializer
  ```
- **addPrize**: Adds a new prize to the raffle by the admin.
  ```solidity
  function addPrize(string calldata name, ... ) external onlyRole(ADMIN_ROLE)
  ```
- **editPrize**: Edits an existing prize’s details.
  ```solidity
  function editPrize(uint256 prizeId, ... ) external onlyRole(ADMIN_ROLE)
  ```
- **removePrize**: Marks a prize as inactive.
  ```solidity
  function removePrize(uint256 prizeId) external onlyRole(ADMIN_ROLE)
  ```
- **spendRaffle**: Allows users to spend tickets on a prize.
  ```solidity
  function spendRaffle(uint256 prizeId, uint256 ticketAmount) external
  ```
- **requestWinner**: Admin initiates a VA-randoming winner selection.
  ```solidity
  function requestWinner(uint256 prizeId) external onlyRole(ADMIN_ROLE)
  ```
- **handleWinnerSelection**: Sets the winner using a VRF callback.
  ```solidity
  function handleWinnerSelection(uint256 requestId, ... ) external onlyRole(SUPRA_ROLE)
  ```
- **setWinner**: Finalizes the selection of a winner.
  ```solidity
  function setWinner(uint256 prizeId) external onlyRole(ADMIN_ROLE)
  ```
- **claimPrize**: Allows winners to claim their prize.
  ```solidity
  function claimPrize(uint256 prizeId) external
  ```

### Storage Variables
- **admin**: Stores admin address for management rights.
  ```solidity
  address public admin;
  ```
- **spinContract**: Stores reference to the spinning contract interface.
  ```solidity
  ISpin public spinContract;
  ```
- **supraRouter**: Stores reference to the VRF service contract.
  ```solidity
  ISupraRouterContract public supraRouter;
  ```
- **prizes, prizeIds**: Handles storage for prize details and their IDs.
  ```solidity
  mapping(uint256 => Prize) public prizes;
  uint256[] public prizeIds;
  ```
- **totalTickets, prizeRanges**: Tracks the total number of tickets and user ticket ranges per prize.
  ```solidity
  mapping(uint256 => uint256) public totalTickets;
  mapping(uint256 => Range[]) public prizeRanges;
  ```

This setup allows for a decentralized and autonomous operation of the raffle system, utilizing randomness and ticketing entries to designate winners efficiently.


 ## DOCUMENTATION: 

