
## PROTOCOL OVERVIEW:

**Virtuals Protocol** is a modular Solidity framework for launching and operating on-chain "Virtual Agents"—self-contained micro-ecosystems with their own token, DAO, NFT identity and revenue flow.  

1. Native Currency  
   • `Virtual` (ERC20) is the base token. Voting power is locked into non-transferable `veVirtualToken`, which feeds system-wide governance via `VirtualProtocolDAO` and fast-tracked `VirtualGenesisDAO`.  

2. Agent Creation  
   • Anyone stakes Virtual into `AgentFactoryV*` and submits an application.  
   • When a DAO proposal passes, the factory clones templates (`AgentToken`, `AgentDAO`, `AgentVeToken`) and mints an `AgentNftV2` that represents the persona. Optional Token-Bound Accounts (ERC-6551) are created for smart-wallet utility.  

3. Operations inside an Agent  
   • Holders stake the agent’s ERC20 into its veToken to gain voting power.  
   • Builders mint `ContributionNft` proposals; once accepted they mature into `ServiceNft`, updating the agent’s impact score.  
   • Daily income is funneled to `AgentRewardV2/V3`, which splits rewards among protocol, stakers, validators, model & dataset owners.  

4. DeFi & Treasury  
   • Swaps, liquidity and bonding-curve issuance use `FFactory`, `FRouter`, `FPair`, `Bonding` and taxation helpers (`AgentTax`, `BondingTax`, `LPRefund`).  

5. Auxiliary Tools  
   • `Airdrop`, `TokenSaver`, `EloCalculator`, bridging contracts, and upgradeable admin utilities round out the stack.  

Together these pieces let communities spin up fully-governed, revenue-sharing virtual personas with minimal code and maximum composability.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/token/Airdrop.sol
### Contract: Airdrop
The `Airdrop` contract facilitates batch token distribution using the ERC20 standard. It executes token transfers from the caller to multiple recipients, specified in arrays, by leveraging inline assembly for efficiency.

### Function: airdrop
```solidity
function airdrop(
    IERC20 _token,
    address[] calldata _recipients,
    uint256[] calldata _amounts,
    uint256 _total
) external
```
Summary: The `airdrop` function allows an external caller to distribute specified amounts of an ERC20 token to multiple recipients. It performs this by transferring the total required amount from the caller, then iterates through recipients and amounts to execute individual transfers. Inline assembly optimizes these operations by directly calling the `transferFrom` and `transfer` methods of the ERC20 token contract.

Parameters:
- `_token`: The ERC20 token contract address.
- `_recipients`: The list of recipient addresses.
- `_amounts`: The list of amounts corresponding to each recipient.
- `_total`: The total amount to be deducted from the caller's balance.

### Storage Variables
This contract has no storage variables as it operates purely through function parameters.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/token/Minter.sol
```solidity
contract Minter is IMinter, Initializable, OwnableUpgradeable
```
The `Minter` contract is responsible for minting tokens to models and dataset owners. It integrates with proxy, NFT, and ERC20 contracts, and manages impact multipliers and IP shares for token distribution.

### Functions:

#### `initialize`
```solidity
function initialize(
    address serviceAddress,
    address contributionAddress,
    address agentAddress,
    uint256 ipShare_,
    uint256 impactMultiplier_,
    address ipVault_,
    address agentFactory_,
    address initialOwner,
    uint256 maxImpact_
) public initializer
```
Initializes the contract with addresses for service, contribution, agent, IP vault, agent factory, and sets ownership and impact parameters. It uses an initializer pattern for upgradeability.

#### `mint`
```solidity
function mint(uint256 nftId) public noReentrant
```
Mints tokens based on the NFT's impact, splits among model, dataset owners, and an IP vault. It checks valid NFT status and ensures the operation isn't repeated.

### Storage Variables:

- `address public serviceNft;`
  Stores the address of the service NFT.

- `uint256 public ipShare;`
  Holds the IP share percentage for distribution.

- `mapping(uint256 => bool) _mintedNfts;`
  Tracks minted NFTs to prevent re-minting.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/token/Virtual.sol
### Contract: VirtualToken
`VirtualToken` is an ERC20 token contract with a specified cap, leveraging OpenZeppelin's `ERC20Capped` and `Ownable` modules for enhanced functionality. It is initialized with an initial supply and an owner address.

#### Constructor
```solidity
constructor(uint256 _initialSupply, address initialOwner)
```
**Summary**: Initializes the ERC20 token named "Virtual Protocol" with the symbol "VIRTUAL". Sets a cap of 1 billion tokens and assigns the ownership to the specified `initialOwner`. Mints the `initialSupply` to the deployer’s address.

#### Function: mint
```solidity
function mint(address _to, uint256 _amount) external onlyOwner
```
**Summary**: Mints the specified `_amount` of tokens to the address `_to`. Can only be invoked by the owner.

#### Storage: Cap
```solidity
uint256 private _cap;
```
**Summary**: Sets the maximum number of tokens that can ever be created, fixed at 1 billion tokens (`1000000000 * 10 ** 18`).



## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/fun/FRouter.sol
### Contract: FRouter

`FRouter` is an upgradeable smart contract designed for managing token exchanges in a decentralized finance ecosystem. It interacts with `FFactory` to handle liquidity pools and performs token swaps with tax considerations. 

### Functions

#### initialize(address factory_, address assetToken_)
Initializes the contract with required addresses and roles. Ensures non-zero addresses.

#### getAmountsOut(address token, address assetToken_, uint256 amountIn)
Computes the output token amount for a given input amount by accessing pair reserves and performing calculations based on the constant product formula.

#### addInitialLiquidity(address token_, uint256 amountToken_, uint256 amountAsset_)
Adds initial liquidity to a token pair, transferring specified amounts and minting new tokens.

#### sell(uint256 amountIn, address tokenAddress, address to)
Executes a sell operation, determining the output amount, applying tax, and finalizing the swap.

#### buy(uint256 amountIn, address tokenAddress, address to)
Facilitates a token purchase, deducting tax, and swapping tokens to the recipient.

#### graduate(address tokenAddress)
Transfers remaining assets from a pair to the caller following a swap completion.

#### approval(address pair, address asset, address spender, uint256 amount)
Enables spending approvals for specified token pairs and amounts.

#### setTaxManager(address newManager)
Updates the tax manager address, ensuring the caller has the admin role.

### Storage Variables

- **ADMIN_ROLE**: Identifier for admin role, facilitating access control.
- **EXECUTOR_ROLE**: Identifier for the executor role, enabling transaction execution.
- **factory**: Reference to `FFactory`, providing access to liquidity pair information.
- **assetToken**: Address of the asset token, central to the swap operations.
- **taxManager**: Address of the tax manager, handling tax processes.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/fun/FFactory.sol
### FFactory Contract

The `FFactory` contract is an upgradeable smart contract that facilitates the creation and management of trading pairs using the `FPair` contract. It provides role-based access control, allowing designated creators to set up new pairs with associated taxes.

### Interfaces and Functions

- **constructor()** `constructor()`:
  Disables initializers at deployment to prevent misuse.

- **initialize** `initialize(address taxVault_, uint256 buyTax_, uint256 sellTax_) external initializer`:
  Initializes the contract, setting tax vault and tax percentages.

- **_createPair** `_createPair(address tokenA, address tokenB) internal returns (address)`:
  Validates inputs and instantiates a new `FPair`, adding it to the pairs list.

- **createPair** `createPair(address tokenA, address tokenB) external onlyRole(CREATOR_ROLE) nonReentrant returns (address)`:
  Creates a new token pair if caller has `CREATOR_ROLE`.

- **getPair** `getPair(address tokenA, address tokenB) public view returns (address)`:
  Returns the address of the pair for given token addresses.

- **allPairsLength** `allPairsLength() public view returns (uint)`:
  Returns the number of pairs created.

- **setTaxParams** `setTaxParams(address newVault_, uint256 buyTax_, uint256 sellTax_) public onlyRole(ADMIN_ROLE)`:
  Sets tax vault and percentages; requires `ADMIN_ROLE`.

- **setRouter** `setRouter(address router_) public onlyRole(ADMIN_ROLE)`:
  Sets the router address; requires `ADMIN_ROLE`.

### Storage Variables

- **ADMIN_ROLE:** `bytes32 public constant ADMIN_ROLE`: Role constant for admin.
- **CREATOR_ROLE:** `bytes32 public constant CREATOR_ROLE`: Role constant for creator.
- **_pair:** `mapping(address => mapping(address => address)) private _pair`: Maps token pairs to their addresses.
- **pairs:** `address[] public pairs`: Array storing all pair addresses.
- **router:** `address public router`: Address of the associated router.
- **taxVault:** `address public taxVault`: Address for tax collection.
- **buyTax:** `uint256 public buyTax`: Tax percentage applied on buy transactions.
- **sellTax:** `uint256 public sellTax`: Tax percentage applied on sell transactions.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/fun/FPair.sol
### Contract: FPair

The `FPair` contract implements a token pair for a DeFi liquidity pool, involving two ERC20 tokens. It facilitates operations such as minting initial reserves and swapping tokens, managed by a designated router.

### Storage Variables

- `address public router`: Router address that manages the contract. Only this address can invoke specific functions.

- `address public tokenA` & `address public tokenB`: Addresses of the two tokens involved in the pair.

- `struct Pool private _pool`: Holds the reserve amounts and a constant product (k) for the liquidity pool mechanics.

### Function: mint

```solidity
function mint(uint256 reserve0, uint256 reserve1) public onlyRouter returns (bool)
```

Initializes the liquidity pool with the given reserves for tokens. It sets the initial state of the `_pool` only if it hasn't been set previously, emitting a `Mint` event on success.

### Function: swap

```solidity
function swap(uint256 amount0In, uint256 amount0Out, uint256 amount1In, uint256 amount1Out) public onlyRouter returns (bool)
```

Allows the designated router to manage the token swapping within the pool. It updates reserves per swap dynamics and emits a `Swap` event.

### Function: approval

```solidity
function approval(address _user, address _token, uint256 amount) public onlyRouter returns (bool)
```

Enables the router to approve a set amount of a specified token for a user. This reinforces control over token allowances within the contract.

### Function: transferAsset & transferTo

```solidity
function transferAsset(address recipient, uint256 amount) public onlyRouter
```

```solidity
function transferTo(address recipient, uint256 amount) public onlyRouter
```

Facilitates the transfer of respective tokens (A or B) to a recipient, controlled by the router.

### Function: getReserves, kLast, priceALast, priceBLast, balance, assetBalance

Provides read-only access to the pool's current state, including token reserves, the constant product, and balance details for analytics and management purposes.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/fun/Bonding.sol
### Contract: Bonding
**Definition:**

```solidity
contract Bonding is Initializable, ReentrancyGuardUpgradeable, OwnableUpgradeable {
```

The `Bonding` contract allows users to create and trade custom tokens using bonding curves. It handles the initialization of token pairs, trading mechanisms, and integration with a factory to deploy new token contracts.

### Storage Variables

- **address private _feeTo;**
   - Address to receive fees.

- **FFactory public factory;**
   - Factory contract reference.

- **FRouter public router;**
   - Router contract reference.

- **uint256 public initialSupply;**
   - Initial supply of the token.

- **uint256 public fee;**
   - Fee amount.

- **uint256 public constant K = 3_000_000_000_000;**
   - Constant for calculations.

- **uint256 public assetRate;**
   - Rate for assets.

- **uint256 public gradThreshold;**
   - Threshold for graduation in trading.

- **uint256 public maxTx;**
   - Maximum transaction amount.

- **address public agentFactory;**
   - Address of the agent factory.

- **struct Profile { address user; address[] tokens; }**
   - Stores user's profile data.

- **struct Token {...}**
   - Detailed token information.

- **struct Data {...}**
   - Token data structure.

- **struct DeployParams {...}**
   - Parameters for deployment.

- **DeployParams private _deployParams;**
   - Stores deploy parameters.

- **mapping(address => Profile) public profile;**
   - Maps user to their profile.

- **address[] public profiles;**
   - List of profiles.

- **mapping(address => Token) public tokenInfo;**
   - Maps token address to token info.

- **address[] public tokenInfos;**
   - List of token information.

### Functions

- **constructor**
   - Disables initializers to prevent misuse.

- **function initialize(...)**
   - Initializes the contract with the given parameters.

- **function _createUserProfile(address _user) internal returns (bool)**
   - Creates a user profile.

- **function _checkIfProfileExists(address _user) internal view returns (bool)**
   - Checks if a user profile exists.

- **function _approval(address _spender, address _token, uint256 amount) internal returns (bool)**
   - Approves token spending for a given amount.

- **function setInitialSupply(uint256 newSupply) public onlyOwner**
   - Sets the initial supply value.

- **function setGradThreshold(uint256 newThreshold) public onlyOwner**
   - Sets the graduation threshold.

- **function setFee(uint256 newFee, address newFeeTo) public onlyOwner**
   - Updates fee and recipient.

- **function setMaxTx(uint256 maxTx_) public onlyOwner**
   - Sets the maximum transaction amount.

- **function setAssetRate(uint256 newRate) public onlyOwner**
   - Updates the asset rate.

- **function setDeployParams(DeployParams memory params) public onlyOwner**
   - Sets deployment parameters.

- **function getUserTokens(address account) public view returns (address[] memory)**
   - Retrieves user token information.

- **function launch(...) public nonReentrant returns (address, address, uint)**
   - Launches a new token.

- **function launchFor(...) public nonReentrant returns (address, address, uint)**
   - Launches a token for a specified creator.

- **function sell(uint256 amountIn, address tokenAddress) public returns (bool)**
   - Allows trading for selling tokens.

- **function buy(uint256 amountIn, address tokenAddress) public payable returns (bool)**
   - Enables token buying.

- **function _openTradingOnUniswap(address tokenAddress) private**
   - Opens trading on Uniswap for a given token.

- **function unwrapToken(address srcTokenAddress, address[] memory accounts) public**
   - Unwraps tokens to convert them back to their original form.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/fun/FERC20.sol
```solidity
contract FERC20 is Context, IERC20, Ownable { ... }
```

**FERC20 Contract**

The `FERC20` contract is a custom ERC20 token implementation with additional features like max transaction limits and owner privileges. It inherits from `Context`, `IERC20`, and `Ownable` and provides standard ERC20 functionalities along with extensions to control token distribution via max transaction limitations.

**Functions**

- `function name() public view returns (string memory)`  
  Returns the name of the token.

- `function symbol() public view returns (string memory)`  
  Returns the symbol of the token.

- `function decimals() public pure returns (uint8)`  
  Returns the decimals used for token balance representation.

- `function totalSupply() public view override returns (uint256)`  
  Returns the total token supply.

- `function balanceOf(address account) public view override returns (uint256)`  
  Returns the token balance of a given address.

- `function transfer(address recipient, uint256 amount) public override returns (bool)`  
  Transfers a specified amount to a recipient if it does not exceed the max transaction limit (unless excluded).

- `function allowance(address owner, address spender) public view override returns (uint256)`  
  Returns the remaining allowed amount that a spender can spend on behalf of the owner.

- `function approve(address spender, uint256 amount) public override returns (bool)`  
  Approves a specified spender to spend a set amount on behalf of the msg.sender.

- `function transferFrom(address sender, address recipient, uint256 amount) public override returns (bool)`  
  Allows a spender to send a specified amount from a sender to a recipient, deducting the amount from the spender's allowance.

- `function updateMaxTx(uint256 _maxTx) public onlyOwner`  
  Updates the maximum transaction percentage allowed for non-excluded addresses.

- `function excludeFromMaxTx(address user) public onlyOwner`  
  Excludes an address from the max transaction limit.

- `function burnFrom(address user, uint256 amount) public onlyOwner`  
  Allows the owner to burn a specified amount from a given address's balance.

**Storage Variables**

- `uint8 private constant _decimals = 18;`
  Defines the number of decimal places for the token.

- `uint256 private _totalSupply;`
  Stores the total supply of tokens.

- `string private _name;`
  Stores the token name.

- `string private _symbol;`
  Stores the token symbol.

- `uint public maxTx;`
  Holds the maximum allowable transaction percentage.

- `uint256 private _maxTxAmount;`
  Holds the calculated max transaction amount.

- `mapping(address => uint256) private _balances;`
  Maps addresses to their respective token balances.

- `mapping(address => mapping(address => uint256)) private _allowances;`
  Maps owner's addresses to spender's addresses to remaining allowance amounts.

- `mapping(address => bool) private isExcludedFromMaxTx;`
  Tracks addresses excluded from max transaction limits.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/tax/LPRefund.sol
## LPRefund Contract Summary

The LPRefund contract is designed to handle token refunds for specific transactions. It utilizes upgradeable access control patterns from OpenZeppelin.

### Contract Definition
```solidity
contract LPRefund is Initializable, AccessControlUpgradeable
```

### Functions

#### `initialize`
Initializes the contract with a default admin and tax token address.
- **Interface**: `function initialize(address defaultAdmin_, address taxToken_) external initializer`
- **Summary**: Sets up roles and assigns the tax token. Only callable once.

#### `withdraw`
Allows an admin to withdraw any ERC20 token from the contract.
- **Interface**: `function withdraw(address token) external onlyRole(ADMIN_ROLE)`
- **Summary**: Transfers all of the given token balance to the caller with admin role.

#### `refund`
Processes batch refunds for multiple transactions.
- **Interface**: `function refund(address recipient, bytes32[] memory txhashes, uint256[] memory amounts) public onlyRole(EXECUTOR_ROLE)`
- **Summary**: Refunds specified amounts to a recipient if the transaction hash is not already refunded.

#### `manualRefund`
Allows an admin to manually approve a refund.
- **Interface**: `function manualRefund(bytes32 txhash, address recipient, uint256 amount) public onlyRole(ADMIN_ROLE)`
- **Summary**: Adds a specified amount to a refund and transfers the tokens.

### Storage Variables

#### `ADMIN_ROLE`
- **Definition**: `bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");`
- **Summary**: Constant for admin role identifier.

#### `EXECUTOR_ROLE`
- **Definition**: `bytes32 public constant EXECUTOR_ROLE = keccak256("EXECUTOR_ROLE");`
- **Summary**: Constant for executor role identifier.

#### `taxToken`
- **Definition**: `address public taxToken;`
- **Summary**: Address of the token used for taxation.

#### `refunds`
- **Definition**: `mapping(bytes32 txhash => uint256 amount) public refunds;`
- **Summary**: Maps transaction hashes to refund amounts.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/tax/AgentTax.sol
### Contract: AgentTax
The `AgentTax` contract manages tax collection and distribution for agents in a blockchain network. It handles roles, establishes swap parameters and thresholds, and facilitates taxes to swap tokens via a router interface.

#### Storage Variables:
- **assetToken** (`address`): The asset token address used for fee swaps.
- **taxToken** (`address`): Token used to collect taxes.
- **router** (`IRouter`): Interface to handle token swaps.
- **treasury** (`address`): Destination address for collected fees.
- **feeRate** (`uint16`): Fee percentage for swaps and distributions.
- **minSwapThreshold** (`uint256`): Minimum amount limit for swapping.
- **maxSwapThreshold** (`uint256`): Maximum amount limit for swapping.
- **agentNft** (`IAgentNft`): Interface to map agent IDs to NFTs.
- **creatorFeeRate** (`uint16`): Fee percentage attributed to the creator.
- **tbaBonus** (`ITBABonus`): Interface for TBA bonus distribution.
- **_agentTba** (`mapping(uint256 => address)`): Caches Agent NFT information to optimize calls.
- **taxHistory** (`mapping(bytes32 => TaxHistory)`): Logs historical tax collection data.
- **agentTaxAmounts** (`mapping(uint256 => TaxAmounts)`): Tracks agent-specific tax collection amounts.

#### Functions:
- **initialize()**: `initializer`
  ```solidity
  function initialize(address defaultAdmin_, address assetToken_, address taxToken_, address router_, address treasury_, uint256 minSwapThreshold_, uint256 maxSwapThreshold_, address nft_)
  ```
  Initializes the contract, setting up initial parameters like tokens, thresholds, and roles. Ensures asset and tax tokens are distinct, and grants roles.

- **updateSwapParams()**: `ADMIN_ROLE`
  ```solidity
  function updateSwapParams(address router_, address assetToken_, uint16 feeRate_, uint16 creatorFeeRate_)
  ```
  Adjusts the swap parameters including the router, tokens, and fee rates. Validates fee rates sum to 10000 (the denominator).

- **handleAgentTaxes()**: `EXECUTOR_ROLE`
  ```solidity
  function handleAgentTaxes(uint256 agentId, bytes32[] memory txhashes, uint256[] memory amounts, uint256 minOutput)
  ```
  Records collected taxes and swaps them via a router. Checks for transaction hash duplication and swaps only above threshold.

- **updateSwapThresholds()**: `ADMIN_ROLE`
  ```solidity
  function updateSwapThresholds(uint256 minSwapThreshold_, uint256 maxSwapThreshold_)
  ```
  Sets new minimum and maximum swap thresholds emitting relevant events.

- **updateTreasury()**: `ADMIN_ROLE`
  ```solidity
  function updateTreasury(address treasury_)
  ```
  Updates the treasury address for future fee distributions.

- **withdraw()**: `ADMIN_ROLE`
  ```solidity
  function withdraw(address token)
  ```
  Allows administrative withdrawal of tokens from the contract balance to the treasury.

- **dcaSell()**: `EXECUTOR_ROLE`
  ```solidity
  function dcaSell(uint256[] memory agentIds, uint256 slippage, uint256 maxOverride)
  ```
  Executes automated selling of collected taxes with allowances for slippage and maximum limits.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/tax/TBABonus.sol
```solidity
contract TBABonus is ITBABonus, Initializable, AccessControlUpgradeable
```

### Summary:
The `TBABonus` contract implements a bonus distribution system for agents. Utilizing ERC20 tokens, it implements role-based access control to update bonus rates and allowances and to execute bonus distribution securely. Key focus areas include allowance management and ensuring bonuses do not exceed set limits.

### Functions:

#### `initialize`
```solidity
function initialize(address defaultAdmin_, address assetToken_) external initializer
```
This function initializes the contract with a specified admin and asset token for the bonuses. It sets the default bonus rate to 3500 and assigns the admin roles appropriately.

#### `updateBonusRate`
```solidity
function updateBonusRate(uint16 bonusRate_) public onlyRole(ADMIN_ROLE)
```
Allows an admin to update the bonus rate, emitting an event to log changes.

#### `setAllowances`
```solidity
function setAllowances(uint256[] memory agentIds, uint256[] memory allowances) public onlyRole(ADMIN_ROLE)
```
Sets allowances for a list of agent IDs, ensuring the new allowance is not less than what's already paid.

#### `distributeBonus`
```solidity
function distributeBonus(uint256 agentId, address recipient, uint256 amount) public
```
Distributes bonus to a specified recipient, ensuring all validations regarding role, balance, and allowance are met.

### Storage Variables:

#### `bonusRate`
```solidity
uint16 public bonusRate;
```
Determines the percentage rate for bonuses, initially set to 3500.

#### `assetToken`
```solidity
IERC20 public assetToken;
```
Represents the ERC20 token used for bonuses.

#### `_agentAllowances`
```solidity
mapping(uint256 agentId => uint256 allowance) private _agentAllowances;
```
Tracks the allowances for each agent.

#### `_agentPaidAmounts`
```solidity
mapping(uint256 agentId => uint256 paidAmount) private _agentPaidAmounts;
```
Records the amounts already paid out to agents.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/tax/BondingTax.sol
### Contract: BondingTax
The `BondingTax` contract manages tax operations within a bonding system. It is an upgradeable contract utilizing OpenZeppelin standards and interfaces with an external `IRouter`. The contract handles swapping tax tokens to asset tokens and adjusts various parameters and thresholds for these operations.

### Storage Variables
- **assetToken** (`address`): The asset token address.
- **taxToken** (`address`): The tax token address.
- **router** (`IRouter`): Router interface for swaps.
- **bondingRouter** (`address`): Address for bonding operations.
- **treasury** (`address`): Treasury address for funds.
- **minSwapThreshold** (`uint256`): Minimum swap threshold.
- **maxSwapThreshold** (`uint256`): Maximum swap threshold.
- **_slippage** (`uint16`): Slippage percentage for swaps.

### Functions
#### `initialize`
Initializes the contract with default admin roles and settings for tokens, router, and treasury.
- **Interface**: `function initialize(address defaultAdmin_, address assetToken_, address taxToken_, address router_, address bondingRouter_, address treasury_, uint256 minSwapThreshold_, uint256 maxSwapThreshold_) external initializer`
#### `updateSwapParams`
Updates the swap parameters such as router, bonding router, asset token, and slippage value.
- **Interface**: `function updateSwapParams(address router_, address bondingRouter_, address assetToken_, uint16 slippage_) public onlyRole(ADMIN_ROLE)`
#### `updateSwapThresholds`
Updates the minimum and maximum swap thresholds.
- **Interface**: `function updateSwapThresholds(uint256 minSwapThreshold_, uint256 maxSwapThreshold_) public onlyRole(ADMIN_ROLE)`
#### `updateTreasury`
Updates the treasury address for fund storage.
- **Interface**: `function updateTreasury(address treasury_) public onlyRole(ADMIN_ROLE)`
#### `withdraw`
Allows the admin to withdraw specified tokens to the treasury.
- **Interface**: `function withdraw(address token) external onlyRole(ADMIN_ROLE)`
#### `swapForAsset`
Performs token swaps from taxToken to assetToken if conditions are met.
- **Interface**: `function swapForAsset() public onlyBondingRouter returns (bool, uint256)`




## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/libs/TokenSaver.sol
### TokenSaver Contract

The `TokenSaver` contract is designed to securely transfer tokens using access control mechanisms. It utilizes OpenZeppelin's AccessControl to manage roles and permissions for authorized token transfers.

### Contract Definition
```solidity
tokenSaver.sol
```

### Storage Variables
- `TOKEN_SAVER_ROLE`: A hash representing the role identifier for users permitted to transfer tokens.

  ```solidity
  bytes32 public constant TOKEN_SAVER_ROLE = keccak256("TOKEN_SAVER_ROLE");
  ```

### Functions

#### `saveToken`
Transfers a specified amount of a given token to a receiver, ensuring only authorized users can execute it. It employs SafeERC20's safeTransfer method to ensure secure token transactions.

**Interface:**
```solidity
function saveToken(address _token, address _receiver, uint256 _amount) external onlyTokenSaver
```

**Summary:**
The `saveToken` function allows authorized users (those with the `TOKEN_SAVER_ROLE`) to transfer tokens securely to a specified receiver address. The function emits a `TokenSaved` event after a successful transfer for logging purposes.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/AgentRewardV2.sol
**Contract: AgentRewardV2**

The `AgentRewardV2` contract handles the distribution and claiming of agent, staker, and validator rewards using checkpoints for reward settings management. It is upgradeable and incorporates access control and safe token handling.

**Contract Variables**:
- `uint48 private _nextRewardId;`: Tracks the next reward ID to be issued.
- `uint256 public constant DENOMINATOR = 10000;`: Divider for percentage calculations.
- `bytes32 public constant GOV_ROLE = keccak256("GOV_ROLE");`: Role identifier for governance.
- Addresses for various referenced tokens (reward, agent, contribution, and service NFTs).
- Reward management structures like `_mainRewards`, `_rewards`, `_rewardSettings`, etc., manage the state of rewards distribution.

**Functions**:
1. **initialize: (address, address, address, address, RewardSettingsCheckpoints.RewardSettings)**
   Initializes the contract with specified token addresses and reward settings.

2. **getRewardSettings: () returns (RewardSettingsCheckpoints.RewardSettings)**
   Retrieves current reward settings.

3. **getPastRewardSettings: (uint32) returns (RewardSettingsCheckpoints.RewardSettings)**
   Gets reward settings at a specific block.

4. **getMainReward: (uint32) returns (MainReward)**
   Fetches main reward data by index.

5. **getReward: (uint256, uint32) returns (Reward)**
   Fetches virtual ID reward data by position.

6. **rewardCount: (uint256) returns (uint256)**
   Returns total rewards for a virtual ID.

7. **distributeRewards: (uint256) returns (uint32)**
   Distributes rewards to protocol and agents.

8. **distributeRewardsForAgents: (uint32, uint256[])**
   Allocates rewards to specified agents.

9. **_distributeProtocolRewards: (uint256) private returns (uint256)**
   Distributes protocol share of rewards.

10. **_prepareAgentsRewards: (uint256, RewardSettingsCheckpoints.RewardSettings private returns (uint256)**
    Prepares reward placeholders by calculating total staked tokens for eligible agents.

11. **setRewardSettings: (uint16, uint16, uint16, uint16, uint256) public**
    Modifies reward settings.

The contract emphasizes modular reward allocation and secure operations through safe token methods and reentrancy protection.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/AgentRewardV3.sol
```solidity
contract AgentRewardV3 is IAgentRewardV3, Initializable, AccessControl, TokenSaver {
```
### Summary
AgentRewardV3 is a smart contract implementing ERC-based staking rewards for virtual assets. It manages reward distribution for stakers and validators, supports governance control, and integrates with various virtual assets and DAOs.

### Functions
- **initialize**: Sets up the reward token, NFT agent, and reward settings. Grants admin role to the caller.
  ```solidity
  function initialize(address rewardToken_, address agentNft_, RewardSettingsCheckpointsV2.RewardSettings memory settings_)
  ```

- **getRewardSettings**: Returns current reward settings.
  ```solidity
  function getRewardSettings() public view returns (RewardSettingsCheckpointsV2.RewardSettings memory)
  ```

- **distributeRewards**: Distributes rewards to stakers and validators based on LP values and reward settings.
  ```solidity
  function distributeRewards(uint256 amount, uint256[] memory virtualIds, bool shouldShareWithProtocol) public onlyGov
  ```

- **claimStakerRewards**: Claims staker rewards for the caller.
  ```solidity
  function claimStakerRewards(uint256 virtualId) public noReentrant
  ```

- **setRewardSettings**: Allows the governance role to update reward settings.
  ```solidity
  function setRewardSettings(uint16 protocolShares_, uint16 stakerShares_) public onlyGov
  ```

### Storage Variables
- **_nextAgentRewardId**: Tracks the next reward ID for agents.
  ```solidity
  uint256 private _nextAgentRewardId;
  ```

- **rewardToken**: Address of the token used for rewards.
  ```solidity
  address public rewardToken;
  ```

- **agentNft**: Address of the AgentNFT contract.
  ```solidity
  address public agentNft;
  ```


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/AgentInference.sol
### AgentInference Contract
The `AgentInference` contract manages inference actions for agents, using ERC20 tokens as payment. It requires both administrative setup and user interactions, allowing tracked activities with mappings for inference counts.

#### Contract Definition
```solidity
contract AgentInference is Initializable, AccessControlUpgradeable, ReentrancyGuardUpgradeable {
```

### Storage Variables
- **`ADMIN_ROLE`** (`bytes32`): Identifier for admin role; manages administrative permissions.
- **`inferenceCount`** (`mapping(uint256 => uint256)`): Tracks the number of inferences per agentId.
- **`token`** (`IERC20`): ERC20 token used for transactions.
- **`agentNft`** (`IAgentNft`): Interface for agent NFTs.

### Functions
#### initialize
```solidity
function initialize(address defaultAdmin_, address token_, address agentNft_) external initializer
```
Initializes the contract, setting up roles and contract references. Admin role is granted to the given address, and references to token and agent NFT are established.

#### prompt
```solidity
function prompt(bytes32 promptHash, uint256[] memory agentIds, uint256[] memory amounts, uint8[][] memory coreIds) public nonReentrant
```
Handles inference by transferring specified amounts of tokens from the sender to each agent, increments inference count and emits an event.

#### promptMulti
```solidity
function promptMulti(bytes32[] memory promptHashes, uint256[] memory agentIds, uint256[] memory amounts, uint8[][] memory coreIds) public nonReentrant
```
Similar to `prompt`, but supports multiple inferences in a single transaction. Ensures efficient balance checks and handles multiple agents per call.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/governance/veVirtualToken.sol
### Contract: veVirtualToken

The `veVirtualToken` contract is an extension of the ERC20 token, providing governance voting capabilities via ERC20Votes and ERC20Permit. It is specifically designed to disable manual transfers and approvals, ensuring that token distribution only occurs through an oracle. The contract is also owned by an initial owner, adding an access control layer.

### Constructor: veVirtualToken

```solidity
constructor(address initialOwner)
```
Initializes the ERC20 token with a name "Virtual Protocol Voting" and symbol "veVIRTUAL". It also sets up the owner for the contract, who has exclusive rights to execute certain functions.

### Function: oracleTransfer

```solidity
function oracleTransfer(address[] memory froms, address[] memory tos, uint256[] memory values) external onlyOwner returns (bool)
```
This function allows the owner to update token balances across multiple addresses simultaneously. It acts as a synchronized mechanism to reflect token transfers made by protocol oracles, which is vital for multi-chain compatibility.

### Storage Variables

N/A

### Comments

This contract ensures governance safety and compatibility with multi-chain operations by disabling average user token transfers and relying on a designated oracle.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/governance/VirtualProtocolDAO.sol
### Contract: `VirtualProtocolDAO`

The `VirtualProtocolDAO` contract is a governance contract that extends OpenZeppelin's governance framework. It leverages multiple extensions for handling voting, proposal settings, and quorum. This contract is specifically set for a virtual protocol, inheriting functionalities from classes such as `Governor`, `GovernorSettings`, and others.

### Constructor

#### Definition
```solidity
constructor(
    IVotes token,
    uint48 initialVotingDelay,
    uint32 initialVotingPeriod,
    uint256 initialProposalThreshold,
    uint256 initialQuorumNumerator
)```

#### Summary
Initializes the `VirtualProtocolDAO` contract with parameters such as token, voting delay, voting period, proposal threshold, and quorum numerator. Sets up the governance name as "VirtualProtocol".

### Functions

#### `votingDelay`
```solidity
function votingDelay() public view override returns (uint256)
```
Returns the delay before voting starts, overridden from `Governor` and `GovernorSettings`.

#### `votingPeriod`
```solidity
function votingPeriod() public view override returns (uint256)
```
Returns the duration of the voting period, overridden to retrieve from parent settings.

#### `proposalThreshold`
```solidity
function proposalThreshold() public view override returns (uint256)
```
Retrieves the minimum proposal threshold, ensuring compatibility with inherited settings.

#### `propose`
```solidity
function propose(
    address[] memory targets,
    uint256[] memory values,
    bytes[] memory calldatas,
    string memory description
) public override returns (uint256)
```
Allows users to propose governance actions. Overridden to integrate target, value, calldata and description inputs.

#### `_propose`
```solidity
function _propose(
    address[] memory targets,
    uint256[] memory values,
    bytes[] memory calldatas,
    string memory description,
    address proposer
) internal override returns (uint256)
```
Internal function that handles the core functionality of proposing, ensuring compatibility with `GovernorStorage`.

#### `quorum`
```solidity
function quorum(uint256 blockNumber) public view override returns (uint256)
```
Calculates the quorum required for a specific block, using inherited quorums.

#### `quorumDenominator`
```solidity
function quorumDenominator() public pure override returns (uint256)
```
Returns the fixed denominator of the quorum, hard-coded to 10,000 to simplify quorum fraction calculations.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/governance/VirtualGenesisDAO.sol
## Contract: VirtualGenesisDAO

VirtualGenesisDAO is a governance smart contract that extends OpenZeppelin's Governor contracts with additional features for early proposal execution. It allows proposals to be executed as soon as the quorum is reached, given that certain criteria are fulfilled. It integrates various extensions like GovernorSettings, GovernorStorage, GovernorVotes, and others, alongside custom checkpoint mechanisms for quorum management.

### Constructor
```solidity
constructor(
    IVotes token,
    uint48 initialVotingDelay,
    uint32 initialVotingPeriod,
    uint256 initialProposalThreshold
) Governor("VirtualGenesis") GovernorSettings(initialVotingDelay, initialVotingPeriod, initialProposalThreshold) GovernorVotes(token) {}
```
**Summary**: Initializes the DAO with voting settings and grants admin roles. Utilizes checkpoints to set an initial quorum level.

### Functions
- **votingDelay**
  ```solidity
  function votingDelay() public view returns (uint256)
  ```
  **Summary**: Overrides the voting delay settings from inherited contracts.

- **votingPeriod**
  ```solidity
  function votingPeriod() public view returns (uint256)
  ```
  **Summary**: Overrides the voting period settings from inherited contracts.

- **proposalThreshold**
  ```solidity
  function proposalThreshold() public view returns (uint256)
  ```
  **Summary**: Overrides the proposal threshold settings from inherited contracts.

- **propose**
  ```solidity
  function propose(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, string memory description) public returns (uint256)
  ```
  **Summary**: Proposes a new governance action, returning a unique proposal identifier.

- **_propose**
  ```solidity
  function _propose(address[] memory targets, uint256[] memory values, bytes[] memory calldatas, string memory description, address proposer) internal returns (uint256)
  ```
  **Summary**: Internal function implementing proposal logic.

- **quorum**
  ```solidity
  function quorum(uint256 blockNumber) public view returns (uint256)
  ```
  **Summary**: Determines the minimum number of votes needed to pass proposals.

- **earlyExecute**
  ```solidity
  function earlyExecute(uint256 proposalId) public payable onlyRole(EXECUTOR_ROLE) returns (uint256)
  ```
  **Summary**: Executes a proposal early if voting is complete and quorum is met.

- **state**
  ```solidity
  function state(uint256 proposalId) public view returns (ProposalState)
  ```
  **Summary**: Checks and returns the current state of a proposal.

- **updateQuorum**
  ```solidity
  function updateQuorum(uint224 newQuorum) public onlyGovernance
  ```
  **Summary**: Updates the quorum to a new value, maintaining governance integrity.

- **supportsInterface**
  ```solidity
  function supportsInterface(bytes4 interfaceId) public view returns (bool)
  ```
  **Summary**: Checks interface support for the contract.

### Storage
- **_earlyExecutions**
  ```solidity
  mapping(uint256 => bool) _earlyExecutions
  ```
  **Summary**: Tracks proposals that have been executed early.

- **_quorumCheckpoints**
  ```solidity
  Checkpoints.Trace224 private _quorumCheckpoints
  ```
  **Summary**: Manages historical quorum values using checkpoints.

- **_quorum**
  ```solidity
  uint256 private _quorum
  ```
  **Summary**: Stores the current quorum level.

- **EXECUTOR_ROLE**
  ```solidity
  bytes32 public constant EXECUTOR_ROLE
  ```
  **Summary**: Defines a role for executing proposals early.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/dev/BMWTokenChild.sol
### Contract: BMWTokenChild
BMWTokenChild is a simple ERC20 token contract with the name 'BeemerToken' and symbol 'BMW'. The contract introduces a mechanism to manage token minting and burning by a specified manager.

### Function: constructor
`constructor(address fxManager) ERC20("BeemerToken", "BMW")`
- **Purpose**: Initializes the token with a given manager.
- **Parameters**: `fxManager` - address that will manage minting and burning.
- **Summary**: Sets the initial manager address and establishes the token name and symbol.

### Function: setFxManager
`function setFxManager(address fxManager) public`
- **Purpose**: Updates the manager who can mint or burn tokens.
- **Parameters**: `fxManager` - new manager's address.
- **Summary**: Changes the `_fxManager` to allow a different entity to control minting and burning.

### Function: mint
`function mint(address user, uint256 amount) public`
- **Purpose**: Mints tokens to a specified user.
- **Parameters**: `user` - recipient address, `amount` - number of tokens to mint.
- **Summary**: Only callable by `_fxManager`. Mints the specified amount to the user.

### Function: burn
`function burn(address user, uint256 amount) public`
- **Purpose**: Burns a specified amount of tokens from a user.
- **Parameters**: `user` - address to burn tokens from, `amount` - number of tokens to burn.
- **Summary**: Only callable by `_fxManager`. Burns the specified amount from the user.

### Storage Variable: _fxManager
`address internal _fxManager`
- **Purpose**: Stores the address authorized to mint and burn tokens.
- **Summary**: Controls token issuance and destruction rights, can be updated via `setFxManager`.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/dev/ProxyAdmin.sol
### MyProxyAdmin Contract Definition:

```solidity
contract MyProxyAdmin is ProxyAdmin {
    constructor(address initialOwner) ProxyAdmin(initialOwner) {}
}
```

#### Summary
The `MyProxyAdmin` contract is a simple extension of OpenZeppelin's `ProxyAdmin` contract. This contract is used to manage a proxy's admin address, utilizing the inherited functionality from `ProxyAdmin`. It includes a constructor that requires an `address` parameter to set the initial owner of the proxy admin.

#### Constructor
```solidity
constructor(address initialOwner)
```
- **Summary**: Initializes the `MyProxyAdmin` contract with the given `initialOwner` address, passing this to the `ProxyAdmin` constructor to establish ownership.
- **Parameter**:
    - `initialOwner` (`address`): The address designated as the initial owner of the admin proxy.

The contract does not introduce additional storage variables beyond those inherited from `ProxyAdmin`.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/dev/BMWToken.sol
### Contract: BMWToken

This contract defines a simple ERC20 token named "BeemerToken" with the symbol "BMW." It incorporates the `Ownable` and `ERC20Permit` functionalities from OpenZeppelin.

#### Constructor
The constructor initializes the ERC20 token with the specified `name` and `symbol` and sets the initial owner of the contract.

```solidity
constructor(address initialOwner) ERC20("BeemerToken", "BMW") Ownable(initialOwner) ERC20Permit("BeemerToken") {}
```

- **Parameters:**
  - `initialOwner`: The address that will possess the ownership rights of this token contract immediately upon deployment.

### Function: mint

#### Definition
The `mint` function allows the creation of new tokens and credits them to the specified address.

```solidity
function mint(address to, uint256 amount) public {
    _mint(to, amount);
}
```

- **Parameters:**
  - `to`: The address that will receive the minted tokens.
  - `amount`: The number of tokens to be minted.

### Storage Variables
There are no explicit storage variables declared in this contract, relying primarily on inherited structures for state management.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/dev/FxERC20ChildTunnel.sol
# FxERC20ChildTunnel Contract

The `FxERC20ChildTunnel` contract facilitates bridging of tokens between Layer 1 (L1) and Layer 2 (L2) in a blockchain environment. It interacts specifically with the `BMWTokenChild` contract.

## Functions

### syncDeposit

```solidity
tfunction syncDeposit(address childToken, uint256 amount) public
```
The `syncDeposit` function is used to transfer tokens from L1 to L2. It takes the child token's address and the amount to be deposited. It calls the `mint` function on the `BMWTokenChild` contract to create the specified amount of tokens in the user's address on L2.

### withdraw

```solidity
function withdraw(address childToken, uint256 amount) public
```
The `withdraw` function is used to bridge tokens back from L2 to L1. It accepts the address of the child token and the amount to withdraw. It invokes the `burn` function on the `BMWTokenChild` contract, which destroys the tokens from the user's address on L2.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/dev/FxERC20RootTunnel.sol
### Contract: FxERC20RootTunnel

The `FxERC20RootTunnel` contract facilitates token transactions in a secure manner using the OpenZeppelin `SafeERC20` library. It enables the deposit and withdrawal of tokens on the Ethereum blockchain.

### Function: deposit

```solidity
deposit(address rootToken, uint256 amount)
```

**Summary**: Allows a user to deposit a specified amount of an ERC20 token into the contract. The function uses OpenZeppelin's `SafeERC20` library to securely transfer the tokens from the user's address to the contract.

**Interface**:
- `rootToken`: The ERC20 token's contract address.
- `amount`: The amount of tokens to deposit.

### Function: syncWithdraw

```solidity
syncWithdraw(address rootToken, uint256 amount)
```

**Summary**: Enables users to withdraw a specified amount of an ERC20 token from the contract back to their address. The transfer is made secure by the `SafeERC20` library.

**Interface**:
- `rootToken`: The ERC20 token's contract address.
- `amount`: The amount of tokens to withdraw.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/dev/ERC6551Registry.sol
```solidity
contract ERC6551Registry is IERC6551Registry
```

### Summary
The `ERC6551Registry` contract is a fake implementation of an ERC6551 Registry. This smart contract inherits from `IERC6551Registry` and uses OpenZeppelin's `Create2` utility. It includes error handling for initialization failures.

### Functions

#### createAccount
```solidity
function createAccount(address implementation, bytes32 salt, uint256 chainId, address tokenContract, uint256 tokenId) external returns (address)
```

This function is intended to create a new account based on a `Create2` deterministic deployment. However, in this fake implementation, it simply returns a hardcoded address. Parameters include the address of the implementation, a salt, chain ID, and token details.

#### account
```solidity
function account(address implementation, uint256 chainId, address tokenContract, uint256 tokenId, uint256 salt) external view returns (address)
```

Returns the address of an account based on given parameters, but similarly to `createAccount`, it returns a hardcoded address. This function can be used to view the deterministic address associated with a combination of parameters.

### Variables
There are no storage variables defined in this contract.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/contribution/ServiceNft.sol
### ServiceNft Contract

The `ServiceNft` contract is an implementation of a standard NFT focused on virtual services. It utilizes the OpenZeppelin library for NFT functionality and supports governance contracts. Key functionalities include minting new service NFTs tied to virtual IDs, and managing impacts and maturity of these NFTs.

#### Contract Definition
```solidity
contract ServiceNft is IServiceNft, Initializable, ERC721Upgradeable, ERC721EnumerableUpgradeable, ERC721URIStorageUpgradeable, OwnableUpgradeable
```

### Functions

#### `initialize`
```solidity
function initialize(address initialAgentNft, address initialContributionNft, uint16 initialDatasetImpactWeight) public initializer
```
Initializes the contract with agent NFT, contribution NFT addresses, and an impact weight. Sets tokens symbol and name and calls parent initializations.

#### `mint`
```solidity
function mint(uint256 virtualId, bytes32 descHash) public returns (uint256)
```
Mints a service NFT associated with a given virtual ID, utilizing governance proposals for authorization. Calculates maturity and updates records based on proposal impact.

#### `updateImpact`
```solidity
function updateImpact(uint256 virtualId, uint256 proposalId) public
```
Updates the impact of a given service NFT proposal based on dataset and maturity, modifying internal records accordingly.

### Storage Variables

#### `_nextTokenId`
```solidity
uint256 private _nextTokenId;
```
Stores the next available token ID to be minted, ensuring unique identifiers.

#### `personaNft`
```solidity
address public personaNft;
```
Holds the address of the associated persona NFT contract, linking services to personas.

#### `contributionNft`
```solidity
address public contributionNft;
```
Holds the address of the contribution NFT contract, used for accessing contribution data related to services.

#### `datasetImpactWeight`
```solidity
uint16 public datasetImpactWeight;
```
Defines the weight of dataset impact in service evaluation, affecting impacts calculations.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/contribution/ContributionNft.sol
# ContributionNft Contract

## Contract Definition
The `ContributionNft` contract is an ERC721 token with additional functionality for tracking contributions in a virtual persona ecosystem. It is upgradeable and supports metadata URIs, enumeration, and storage.

## Storage Variables
- **personaNft (address public):** Holds the address for the connected persona NFT contract.
- **_contributionVirtualId (mapping):** Maps token IDs to virtual IDs, linking them to specific persona NFTs.
- **_parents (mapping):** Maps token IDs to their parent contribution IDs.
- **_children (mapping):** Maps parent IDs to their child contribution arrays.
- **_cores (mapping):** Maps token IDs to a core ID.
- **modelContributions (mapping):** Flags token IDs indicating if they're model contributions.
- **modelDatasets (mapping):** Associates token IDs with dataset IDs if they're models.
- **_admin (address private):** Addresses with privilege to create contribution proposals without votes.
- **_eloCalculator (address private):** Holds the address for the Elo calculator.

## Functions
### initialize function
Initializes the contract with the persona address, setting up ERC721 configurations.

### tokenVirtualId function
```solidity
function tokenVirtualId(uint256 tokenId) public view returns (uint256)
```
Retrieves the virtual ID linked to a given token ID.

### getAgentDAO function
```solidity
function getAgentDAO(uint256 virtualId) public view returns (IGovernor)
```
Obtains the DAO governor contract associated with a virtual ID.

### isAccepted function
```solidity
function isAccepted(uint256 tokenId) public view returns (bool)
```
Checks if a tokenId is considered 'accepted' by evaluating its proposal state.

### mint function
```solidity
function mint(address to, uint256 virtualId, uint8 coreId, string memory newTokenURI,
  uint256 proposalId, uint256 parentId, bool isModel_, uint256 datasetId) external returns (uint256)
```
Mints a new contribution NFT, setting up necessary relationships and emitting an event.

### getAdmin function
```solidity
function getAdmin() public view override returns (address)
```
Returns the current admin address.

### setAdmin function
```solidity
function setAdmin(address newAdmin) public
```
Allows the current admin to set a new admin address.

### tokenURI function
Overrides the method to return the token URI using different interfaces.

### getChildren function
```solidity
function getChildren(uint256 tokenId) public view returns (uint256[] memory)
```
Returns the array of children IDs for a given token ID.

### getParentId function
```solidity
function getParentId(uint256 tokenId) public view returns (uint256)
```
Fetches the parent ID for a specific token ID.

### getCore function
```solidity
function getCore(uint256 tokenId) public view returns (uint8)
```
Gets the core ID associated with a token ID.

### supportsInterface function
Checks supported interfaces across inherited contracts.

### _increaseBalance function
Handles internal logic to manage balance increases.

### _update function
Performs internal updates on token ownership and authorization.

### isModel function
```solidity
function isModel(uint256 tokenId) public view returns (bool)
```
Determines if a token ID represents a model.

### ownerOf function
Returns the owner of a particular token ID.

### getDatasetId function
```solidity
function getDatasetId(uint256 tokenId) external view returns (uint256)
```
Gets the dataset ID linked to a model token ID.

### getEloCalculator function
```solidity
function getEloCalculator() external view returns (address)
```
Returns the current Elo calculator address.

### setEloCalculator function
```solidity
function setEloCalculator(address eloCalculator_) public
```
Allows admin to set a new Elo calculator address.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/virtualPersona/EloCalculator.sol
### Contract: EloCalculator

EloCalculator is a contract designed to compute Elo ratings based on game outcomes. It leverages OpenZeppelin's upgradeable utilities and implements the IEloCalculator interface. This contract allows the owner to set Elo calculation parameters and process battle results to update ratings. The primary focus is on accurate and adjustable Elo calculation with upgradeable support.

### Function: initialize
```solidity
function initialize(address initialOwner) public initializer
```
Initializes the contract, setting the contract owner and a default K-factor of 30 for Elo calculations. Utilizes OpenZeppelin's upgradeable initializer.

### Function: mapBattleResultToGameResult
```solidity
function mapBattleResultToGameResult(uint8 result) internal pure returns (uint256)
```
Maps battle outcomes to a game result value. It returns 100 for a win, 50 for a draw, and 0 for a loss.

### Function: _roundUp
```solidity
function _roundUp(uint256 numerator, uint256 denominator) internal pure returns (uint256)
```
Rounds up the division of two numbers, ensuring proper calculation of fractional Elo changes.

### Function: battleElo
```solidity
function battleElo(uint256 currentRating, uint8[] memory battles) public view returns (uint256)
```
Calculates the Elo rating after a series of battles. Updates ratings by applying changes through the Elo library and returns the new rating.

### Function: setK
```solidity
function setK(uint256 k_) public onlyOwner
```
Allows the owner to set the K-factor, providing flexibility in adjusting Elo rating sensitivity.

### Variable: k
```solidity
uint256 public k;
```
Stores the K-factor used in Elo calculations, determining the volatility of rating changes, initially set to 30.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/virtualPersona/AgentNftV2.sol
# AgentNftV2 Contract

## Contract Definition
The `AgentNftV2` is an upgradeable ERC721 contract designed to manage virtual personas (Agents) through a decentralized autonomous organization (DAO) structure. It supports validation, staking, and contribution services with extensive roles management.

## Storage Variables
- `uint256 private _nextVirtualId`: Tracks the next available virtual ID.
- `mapping(address => uint256) private _stakingTokenToVirtualId`: Maps staking tokens to virtual IDs for identification.
- `bytes32 public constant MINTER_ROLE`: Role for minting new tokens.
- `bytes32 public constant VALIDATOR_ADMIN_ROLE`: Role for administering validators across all personas.
- `mapping(uint256 => VirtualInfo) public virtualInfos`: Stores information about each virtual entity.
- `address private _contributionNft`: Address of the contribution NFT.
- `address private _serviceNft`: Address of the service NFT.
- `bytes32 public constant ADMIN_ROLE`: Administrator role for various permissions.
- `mapping(uint256 => bool) private _blacklists`: Manages blacklisting of virtual IDs.
- `mapping(uint256 => VirtualLP) public virtualLPs`: Details of LP tokens for each virtual.
- `address private _eloCalculator`: Address of the Elo calculator.

## Functions

### Function: initialize
```solidity
function initialize(address defaultAdmin) public initializer
```
Initializes the contract with default roles and setups.

### Function: setContributionService
```solidity
function setContributionService(address contributionNft_, address serviceNft_) external onlyRole(DEFAULT_ADMIN_ROLE)
```
Configures contribution and service NFTs addresses.

### Function: nextVirtualId
```solidity
function nextVirtualId() public view returns (uint256)
```
Returns the next available virtual ID.

### Function: mint
```solidity
function mint(uint256 virtualId, address to, string memory newTokenURI, address payable theDAO, address founder, uint8[] memory coreTypes, address pool, address token) external onlyRole(MINTER_ROLE) returns (uint256)
```
Mints a new virtual agent NFT with associated details.

### Function: addCoreType
```solidity
function addCoreType(string memory label) public onlyRole(DEFAULT_ADMIN_ROLE)
```
Adds a core type for the platform.

### Function: virtualInfo
```solidity
function virtualInfo(uint256 virtualId) public view returns (VirtualInfo memory)
```
Retrieves information about a specific virtual ID.

### Function: virtualLP
```solidity
function virtualLP(uint256 virtualId) public view returns (VirtualLP memory)
```
Returns LP details of a specific virtual ID.

### Function: stakingTokenToVirtualId
```solidity
function stakingTokenToVirtualId(address stakingToken) external view returns (uint256)
```
Finds the virtual ID associated with a staking token.

### Function: addValidator
```solidity
function addValidator(uint256 virtualId, address validator) public
```
Adds a validator to a virtual persona.

### Function: _validatorScoreOf
```solidity
function _validatorScoreOf(uint256 virtualId, address account) internal view returns (uint256)
``` 
Internally computes the score of a validator.

### Function: totalProposals
```solidity
function totalProposals(uint256 virtualId) public view returns (uint256)
```
Returns the total number of proposals for a virtual ID.

### Function: setCoreTypes
```solidity
function setCoreTypes(uint256 virtualId, uint8[] memory coreTypes) external onlyVirtualDAO(virtualId)
```
Sets core types for a virtual persona.

### Function: setDAO
```solidity
function setDAO(uint256 virtualId, address newDAO) public
```
Changes the DAO of a virtual persona.

### Function: totalStaked
```solidity
function totalStaked(uint256 virtualId) public view returns (uint256)
```
Returns total staked tokens for the persona's veToken.

### Function: getVotes
```solidity
function getVotes(uint256 virtualId, address validator) public view returns (uint256)
```
Gets the number of votes a validator has for a persona.

### Function: totalSupply
```solidity
function totalSupply() public view returns (uint256)
```
Returns the total number of minted virtual IDs.

### Function: isBlacklisted
```solidity
function isBlacklisted(uint256 virtualId) public view returns (bool)
```
Checks if a virtual ID is blacklisted.

### Function: setBlacklist
```solidity
function setBlacklist(uint256 virtualId, bool value) public onlyRole(ADMIN_ROLE)
```
Sets the blacklist status for a virtual ID.

### Function: setEloCalculator
```solidity
function setEloCalculator(address eloCalculator) public onlyRole(ADMIN_ROLE)
```
Configures the Elo calculator address.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/virtualPersona/AgentMigrator.sol
### Contract: AgentMigrator
AgentMigrator is designed for migrating agents in a blockchain system. It allows the creation and management of NFT-based agents with token and DAO governance capabilities. The contract integrates with OpenZeppelin libraries and implements functionality for migration control, token creation, and policy setting.

#### Contract Variables:
- **_nft**: `AgentNftV2 private _nft;`
  - Links to the AgentNftV2 contract handling the agents.
- **_tokenSupplyParams, _tokenTaxParams**: `bytes private _tokenSupplyParams; bytes private _tokenTaxParams;`
  - Stores encoded parameters for tokens.
- **_tokenAdmin, _assetToken, _uniswapRouter**: `address private _tokenAdmin; address private _assetToken; address private _uniswapRouter;`
  - Addresses for token admin, asset token, and Uniswap router respectively.
- **initialAmount, maturityDuration**: `uint256 public initialAmount; uint256 public maturityDuration;`
  - Initial token amount for liquidity and maturity duration for staking.
- **tokenImplementation, daoImplementation, veTokenImplementation**: `address public tokenImplementation; address public daoImplementation; address public veTokenImplementation;`
  - Addresses for cloning implementations.
- **migratedAgents**: `mapping(uint256 => bool) public migratedAgents;`
  - Tracks migrated agent status.
- **locked**: `bool internal locked;`
  - A reentrancy guard.

### Modifiers:
- **noReentrant**: Prevents reentrant calls by locking the function execution.
  ```solidity
  modifier noReentrant() {...}
  ```

### Functions:
- **constructor**: Initializes the contract with the NFT address.
  ```solidity
  constructor(address agentNft_) Ownable(_msgSender()) {...}
  ```

- **setInitParams**: Sets initial parameters for token and maturity settings.
  ```solidity
  function setInitParams(...)
  ```

- **setTokenSupplyParams**: Configures supply-related token parameters.
  ```solidity
  function setTokenSupplyParams(...)
  ```

- **setTokenTaxParams**: Configures tax-related token parameters.
  ```solidity
  function setTokenTaxParams(...)
  ```

- **setImplementations**: Sets contract addresses for cloning.
  ```solidity
  function setImplementations(...)
  ```

- **migrateAgent**: Manages the agent migration process, including DAO and token setup.
  ```solidity
  function migrateAgent(...)
  ```

- **pause & unpause**: Activates or deactivates the contract's paused state.
  ```solidity
  function pause() & unpause() {...}
  ```

- **reset**: Resets migration status for specified agents.
  ```solidity
  function reset(uint256 id) {...}
  ```


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/virtualPersona/AgentFactory.sol
### Contract: AgentFactoryV2

`AgentFactoryV2` is an upgradeable smart contract designed to manage the creation and execution of "Agent" applications, which involve ERC20 tokens, DAOs, NFTs, and other components. The contract inherits from OpenZeppelin’s `Initializable`, `AccessControl`, and `PausableUpgradeable` to offer role-based access, pausing capabilities, and initialization features.

#### Storage Variables

- **_nextId:** `uint256` - Keeps track of the next application ID.
- **tokenImplementation:** `address` - Address of the token implementation contract.
- **daoImplementation:** `address` - Address of the DAO implementation contract.
- **nft:** `address` - Address of the NFT contract.
- **tbaRegistry:** `address` - Token Bound Account registry.
- **applicationThreshold:** `uint256` - Minimum asset tokens needed to submit an application.
- **allTokens:** `address[]` - Stores created token addresses.
- **allDAOs:** `address[]` - Stores created DAO addresses.
- **assetToken:** `address` - Base currency address.
- **maturityDuration:** `uint256` - Duration for staking, in seconds.
- **WITHDRAW_ROLE:** `bytes32` - Role for withdrawing and executing applications.
- **gov:** `address` - Deprecated; required in previous versions.
- **_vault:** `address` - Vault to hold all Virtual NFTs.

#### Functions

- **initialize:** Initializes core contract parameters. 
  ```solidity
  function initialize(
      address tokenImplementation_,
      address veTokenImplementation_,
      address daoImplementation_,
      address tbaRegistry_,
      address assetToken_,
      address nft_,
      uint256 applicationThreshold_,
      address vault_
  ) public initializer
  ```

- **getApplication:** Returns application details for a given ID.
  ```solidity
  function getApplication(uint256 proposalId) public view returns (Application memory)
  ```

- **proposeAgent:** Allows users to propose a new agent.
  ```solidity
  function proposeAgent(
      string memory name,
      string memory symbol,
      string memory tokenURI,
      uint8[] memory cores,
      bytes32 tbaSalt,
      address tbaImplementation,
      uint32 daoVotingPeriod,
      uint256 daoThreshold
  ) public whenNotPaused returns (uint256)
  ```

- **withdraw:** Withdraws funds from an active application.
  ```solidity
  function withdraw(uint256 id) public noReentrant
  ```

- **executeApplication:** Executes a given application, creating various agent components.
  ```solidity
  function executeApplication(uint256 id, bool canStake) public noReentrant
  ```

- **_createNewDAO:** Internally creates a new DAO using a clone.
  ```solidity
  function _createNewDAO(
      string memory name,
      IVotes token,
      uint32 daoVotingPeriod,
      uint256 daoThreshold
  ) internal returns (address instance)
  ```

- **_createNewAgentToken:** Internally creates a new agent token using a clone.
  ```solidity
  function _createNewAgentToken(string memory name, string memory symbol) internal returns (address instance)
  ```

- **_createNewAgentVeToken:** Internally creates a new veToken.
  ```solidity
  function _createNewAgentVeToken(
      string memory name,
      string memory symbol,
      address stakingAsset,
      address founder,
      bool canStake
  ) internal returns (address instance)
  ```


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/virtualPersona/AgentVeToken.sol
# Contract: `AgentVeToken`

## Summary
The `AgentVeToken` contract is an upgradeable, non-transferable staking token that implements voting and checkpoint mechanisms. It extends `ERC20Upgradeable` and `ERC20Votes`. This contract allows users to stake tokens, delegate voting power, and manages validator delegation. It employs safety checks and prevents reentrancy attacks.

### Storage Variables
- **`founder`** (address): The creator address with control privileges.
- **`assetToken`** (address): The address of the token designated for staking.
- **`agentNft`** (address): The associated NFT address used for additional validation.
- **`matureAt`** (uint256): The timestamp after which the founder can withdraw locked tokens.
- **`canStake`** (bool): Boolean flag to allow/disallow token staking.
- **`initialLock`** (uint256): Amount locked initially, vital for withdrawal restriction.
- **`locked`** (bool): Internal lock to prevent reentrancy.

### Functions

#### `initialize`
```solidity
function initialize(string memory _name, string memory _symbol, address _founder, address _assetToken, uint256 _matureAt, address _agentNft, bool _canStake) external initializer
```
Initializes the contract with token details and administrative settings, setting parameters like founder and indicator for staking permission.

#### `stake`
```solidity
function stake(uint256 amount, address receiver, address delegatee) public
```
Allows token staking by transferring given amount from sender to contract and minting equivalent tokens to the receiver. Enforces multiple access checks and requires valid delegation parameters.

#### `setCanStake`
```solidity
function setCanStake(bool _canStake) public
```
Sets the staking permission flag, restricted to founder's access.

#### `setMatureAt`
```solidity
function setMatureAt(uint256 _matureAt) public
```
Allows updating of maturity timestamp, requiring admin role verification.

#### `withdraw`
```solidity
function withdraw(uint256 amount) public noReentrant
```
Permits withdrawal of staked tokens, maintaining an initial lock condition for the founder and ensuring timestamp maturity.

#### `getPastBalanceOf`
```solidity
function getPastBalanceOf(address account, uint256 timepoint) public view returns (uint256)
```
Returns the token balance of an address at a specific historical time, utilizing checkpoint functionality.

#### Non-Transferable Token Functions
- **`transfer`**, **`transferFrom`**, **`approve`**: Override default ERC20 functions to revert and signify non-transferability.

### Utility Overridden Function
- **`_update`**: Updates balance effectively across ERC20 and voting modules.

This contract ensures security, aligns staking logic with administrative control, and employs OpenZeppelin libraries for extended functionalities.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/virtualPersona/AgentToken.sol
### Contract: AgentToken

`AgentToken` is an upgradable contract implementing a custom ERC20 token with Uniswap integration and tax management features. It extends `ContextUpgradeable` and `Ownable2StepUpgradeable` to provide operational context and ownership control.

### Functions

- **initialize**(addresses, params): Initializes the token with integration setups and supply/tax parameters. Sets up initial balances and creates Uniswap pair.

- **_decodeBaseParams**(owner, params): Internal function to decode and set base parameters like name and symbol.

- **_processSupplyParams**(params): Validates and sets up supply parameters ensuring totals match expected values.

- **_processTaxParams**(params): Configures tax parameters and checks if the token has active taxes.

- **_mintBalances**(lpMint, vaultMint): Mints initial token balances for liquidity and vault.

- **_createPair**(): Creates a Uniswap pair and stores the address in a set.

- **addInitialLiquidity**(lpOwner): Adds initial liquidity to the Uniswap pair and emits an event.

- **isLiquidityPool**(address): Checks if a given address is a registered liquidity pool.

### Storage Variables

- **uniswapV2Pair**: Holds the address of the Uniswap V2 pair.
- **botProtectionDurationInSeconds**: Duration of trading bot protection in seconds.
- **projectBuyTaxBasisPoints**: Basis points representing buy tax rate.
- **projectSellTaxBasisPoints**: Basis points for sell tax rate.
- **projectTaxPendingSwap**: Tracks tax accrued but not yet swapped.

The contract performs tax management and integrates with Uniswap for liquidity provision. It ensures efficient gas usage through structural and logical optimizations, aiming to offer features like automatic tax swapping and validation of liquidity pools effectively. This design supports extensibility and upgradability in complex DeFi ecosystems.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/virtualPersona/AgentFactoryV3.sol
### Contract: AgentFactoryV3  
This contract is designed to facilitate the creation and management of virtual agents and their associated components such as Tokens, DAOs, and NFTs. It includes functionalities for proposing new agents, executing applications, and managing configurations. It utilizes concepts like token binding and roles to manage access and actions within the platform.

### Storage Variables:
- **_nextId**: `uint256 private _nextId;` - Tracks the unique identifier for each new application created.
- **tokenImplementation**: `address public tokenImplementation;` - Holds the address of the token contract template used to create new agent tokens.
- **daoImplementation**: `address public daoImplementation;` - Stores the address of the DAO contract template.
- **nft**: `address public nft;` - The address of the NFT contract associated with the agent.
- **tbaRegistry**: `address public tbaRegistry;` - The registry address for token bound accounts.
- **applicationThreshold**: `uint256 public applicationThreshold;` - Minimum token balance required to propose an agent.
- **allTokens** & **allDAOs**: `address[] public allTokens, allDAOs;` - Arrays storing addresses of all created tokens and DAOs.
- **assetToken**: `address public assetToken;` - The base currency token address.

### Functions:

#### `initialize(address, address, address, address, address, address, uint256, address, uint256)`
Initializes the contract with the necessary implementations, tokens, and threshold settings.

#### `getApplication(uint256)`
Returns the application details for a given proposal ID.

#### `proposeAgent(string, string, string, uint8[], bytes32, address, uint32, uint256)`
Allows a user to propose a new agent if they meet the application threshold.

#### `withdraw(uint256)`
Enables withdrawal of funds from an application if certain conditions are met.

#### `executeApplication(uint256, bool)`
Executes an application, deploying and initializing all associated components for the virtual agent.

### Events:
- **NewPersona**: Emitted when a new agent is created, containing details of the creation.
- **NewApplication**: Emitted when a new application is proposed.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/virtualPersona/AgentDAO.sol
### Contract: `AgentDAO`
The `AgentDAO` contract extends several OpenZeppelin upgradeable governance modules and implements a decentralized autonomous organization (DAO) with voting and proposal functionalities.

### Storage Variables
- `mapping(address => Checkpoints.Trace208) private _scores;`
  - Stores voters' scores with checkpoints for historical lookups.
- `mapping(uint256 => uint256) private _proposalMaturities;`
  - Keeps track of the maturities of proposals.
- `uint256 private _totalScore;`
  - Maintains the total score of all voters.
- `address private _agentNft;`
  - Stores the address of the associated Agent NFT contract.

### Functions
- `constructor()`
  - Disables initializers to prevent unauthorized contract upgrades.

- `initialize(...)`
  - Initializes the contract with DAO settings, including name, token, and voting parameters.

- `votingDelay()`, `votingPeriod()`, `proposalThreshold()`
  - Override OpenZeppelin governance settings to configure DAO-specific parameters.

- `propose(...)`
  - Allows an account to propose a governance action if they meet certain thresholds or are an admin.

- `proposalCount()`
  - Returns the number of proposals created.

- `scoreOf(...)`
  - Returns the latest score of the specified account.

- `getPastScore(...)`
  - Gets a historical score for an account at a specific time.

- `_castVote(...)`
  - Handles voting logic, including score updates and maturity calculation.

- `_tryAutoExecute(...)`
  - Executes proposals automatically when conditions are met.

- `_updateMaturity(...)`
  - Updates the maturity of a proposal based on voting results and parameters.

- `getMaturity(...)`
  - Calculates and returns the maturity of a given proposal.

- `quorum(...)`
  - Returns the quorum required based on a block number.

- `quorumDenominator()`
  - Provides the denominator for quorum calculations.

- `state(...)`
  - Overrides the proposal state to allow early execution of successful proposals.

- `totalScore()`
  - Provides the total accumulated score across all voters.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/virtualPersona/AgentFactoryV4.sol
## Contract: AgentFactoryV4

The `AgentFactoryV4` contract allows the creation and management of decentralized autonomous organizations (DAOs), agent tokens, virtual personas, and related liquidity pools and staking mechanisms. It supports the creation of these entities using either custom ERC20 tokens or by generating new agent tokens.

### Functions

- **initialize**: Initializes the contract with specified implementations, assets, IDs, and thresholds. Ensures the contract is correctly set up before use.
  ```solidity
  function initialize(address tokenImplementation_, address veTokenImplementation_, address daoImplementation_, address tbaRegistry_, address assetToken_, address nft_, uint256 applicationThreshold_, address vault_, uint256 nextId_)
  ```

- **getApplication**: Returns an application struct for a given proposal ID.
  ```solidity
  function getApplication(uint256 proposalId) public view returns (Application memory)
  ```

- **proposeAgent**: Proposes a new agent (DAO, token) by submitting necessary details and transferring the application threshold.
  ```solidity
  function proposeAgent(string memory name, string memory symbol, string memory tokenURI, uint8[] memory cores, bytes32 tbaSalt, address tbaImplementation, uint32 daoVotingPeriod, uint256 daoThreshold) public whenNotPaused returns (uint256)
  ```

- **withdraw**: Allows the proposer to withdraw their application after the application maturing period has ended.
  ```solidity
  function withdraw(uint256 id) public noReentrant
  ```

- **executeApplication**: Executes an active application, creating the requested agent components like token, DAO, veToken, etc.
  ```solidity
  function executeApplication(uint256 id, bool canStake) public noReentrant
  ```

### Storage Variables

- **_nextId**: Tracks the next available application ID.
  ```solidity
  uint256 private _nextId
  ```

- **tokenImplementation**: Address of the token implementation.
  ```solidity
  address public tokenImplementation
  ```

- **applicationThreshold**: Minimum asset tokens required to propose a new agent.
  ```solidity
  uint256 public applicationThreshold
  ```

- **assetToken**: Address of the base currency used in interactions.
  ```solidity
  address public assetToken
  ```

- **_applications**: Mapping of application IDs to Application structs.
  ```solidity
  mapping(uint256 => Application) private _applications
  ```


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/pool/AeroAdaptor.sol
### Contract: AeroAdaptor

The **AeroAdaptor** contract acts as an intermediary between an aerodrome pool and the UniswapRouter v2, facilitating token swaps. It uses the OpenZeppelin library for secure ERC20 token operations, ensuring robust and safe transfers between addresses.

#### Constructor
```solidity
constructor(address router_, address tokenIn_, address tokenOut_, address factory_)
```
Initializes the contract with specified router, input/output tokens, and factory. Approves the router to allow unlimited token transfers.

#### Function: swapExactTokensForTokens
```solidity
function swapExactTokensForTokens(uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline) external returns (uint[] memory amounts)
```
Conducts token swaps by transferring `amountIn` from the caller to the contract and proceeding through the aerodrome router, ensuring output meets or exceeds `amountOutMin`.

#### Function: getAmountsOut
```solidity
function getAmountsOut(uint amountIn, address[] calldata path) external view returns (uint[] memory amounts)
```
Calculates the potential output amounts for a given input without executing a swap.

#### Variables:
- **router**: `address public router`
  - Address of the aerodrome router.
- **tokenIn**: `address public tokenIn`
  - Address of the input token contract.
- **tokenOut**: `address public tokenOut`
  - Address of the output token contract.
- **factory**: `address public factory`
  - Address of the factory managing the swap routes.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/genesis/Genesis.sol
### Genesis Contract Overview

The `Genesis` contract manages a genesis event, allowing participants to contribute virtual tokens for potential rewards if the event succeeds. It utilizes features from the OpenZeppelin library for security and access control.

### Functions

- **initialize(GenesisInitParams calldata params)**: Initializes the genesis event with parameters like ID, times, and addresses. Uses `AccessControlUpgradeable` for permissions.
- **participate(uint256 pointAmt, uint256 virtualsAmt)**: Allows users to participate by contributing virtual tokens, updating mappings and triggering emissions on participation.
- **onGenesisSuccess(...)**: Handles successful genesis outcome, distributing agent tokens to users and processing refunds as needed.
- **claimAgentToken(address userAddress)**: Allows users to claim their agent tokens if they're stored as claimable.
- **onGenesisFailed(uint256[] calldata participantIndexes)**: Processes a failed genesis event, refunding participants accordingly.
- **isEnded() returns (bool)**: Checks if the genesis event has ended.
- **isStarted() returns (bool)**: Checks if the event has started.
- **getParticipantCount() returns (uint256)**: Obtains the total participant count.
- **getParticipantsPaginated(uint256 startIndex, uint256 pageSize) returns (address[])**: Retrieves participants' addresses in pages.
- **getParticipantsInfo(uint256[] calldata participantIndexes) returns (ParticipantInfo[])**: Obtains detailed info for specific participants.
- **getGenesisInfo() returns (GenesisInfo)**: Provides comprehensive information about the genesis event.
- **withdrawLeftAssetsAfterFinalized(...)**: Allows admin to withdraw leftover assets post-finalization.
- **resetTime(uint256 newStartTime, uint256 newEndTime)**: Reschedules the genesis timing.
- **cancelGenesis()**: Cancels the genesis event.

### Storage Variables

- **FACTORY_ROLE**: `bytes32` constant defining the factory role.
- **mapAddrToVirtuals**: `mapping(address => uint256)` tracking virtual contributions by addresses.
- **claimableAgentTokens**: `mapping(address => uint256)` for users' claimable agent tokens.
- **participants**: `address[]` holding the event participants.
- **refundUserCountForFailed**: `uint256` tracking failed genesis refunds.
- **genesisId**: `uint256` unique identifier for the genesis event.
- **factory**: `FGenesis` reference to genesis factory.
- **startTime, endTime**: `uint256` values for event timing.
- **genesisName, genesisTicker**: `string` identifiers for the event.
- **genesisCores**: `uint8[]` for event core specifications.
- **TBA and DAO variables**: Include addresses, voting periods, and thresholds.
- **agentToken and virtualToken related**: Include addresses, total supplies, and states like failed/cancelled status.


## SUMMARY OF FILE: 2025-04-virtuals-protocol/contracts/genesis/FGenesis.sol
### FGenesis Contract

The `FGenesis` contract, written in Solidity, is an upgradeable contract for managing the creation and operation of Genesis contracts. It relies on external libraries and contracts like `GenesisLib`, `AgentFactoryV3`, and others.

#### Contract Definition
```solidity
contract FGenesis is Initializable, AccessControlUpgradeable {...}
```

- **ADMIN_ROLE**: `bytes32 public constant` - Role identifier for administrative access.
- **OPERATION_ROLE**: `bytes32 public constant` – Role identifier for operational access.

#### Storage Variables
- **params**: `Params public` - Stores the parameters for the Genesis contracts, including token addresses and thresholds.
- **genesisContracts**: `mapping(uint256 => address) public` - Maps ID to Genesis contract addresses.
- **genesisID**: `uint256 public` - Stores the current Genesis ID.

#### Functions

- **initialize**: Sets up initial roles and parameters.
```solidity
function initialize(Params memory p) external initializer {...}
```
- **setParams**: Updates Genesis parameters.
```solidity
function setParams(Params calldata p) external onlyRole(ADMIN_ROLE) {...}
```
- **createGenesis**: Deploys a new Genesis contract.
```solidity
function createGenesis(GenesisCreationParams memory gParams) external returns (address) {...}
```
- **_getGenesis**: Internal function to get a Genesis contract by ID.
```solidity
function _getGenesis(uint256 id) internal view returns (Genesis) {...}
```
- **onGenesisSuccess**: Handles operations on successful Genesis completion.
```solidity
function onGenesisSuccess(uint256 id, SuccessParams calldata p) external onlyRole(OPERATION_ROLE) returns (address) {...}
```
- **onGenesisFailed**: Handles failed Genesis operations.
```solidity
function onGenesisFailed(uint256 id, uint256[] calldata participantIndexes) external onlyRole(OPERATION_ROLE) {...}
```
- **withdrawLeftAssetsAfterFinalized**: Withdraws remaining assets after Genesis finalization.
```solidity
function withdrawLeftAssetsAfterFinalized(uint256 id, address to, address token, uint256 amount) external onlyRole(ADMIN_ROLE) {...}
```
- **resetTime**: Resets the start and end time for a Genesis contract.
```solidity
function resetTime(uint256 id, uint256 newStartTime, uint256 newEndTime) external onlyRole(OPERATION_ROLE) {...}
```
- **cancelGenesis**: Cancels an active Genesis contract.
```solidity
function cancelGenesis(uint256 id) external onlyRole(OPERATION_ROLE) {...}
```


## Main List of Files in Project

contracts/AgentInference.sol
contracts/AgentRewardV2.sol
contracts/AgentRewardV3.sol
contracts/IAgentReward.sol
contracts/IAgentRewardV3.sol
contracts/contribution/ContributionNft.sol
contracts/contribution/IContributionNft.sol
contracts/contribution/IServiceNft.sol
contracts/contribution/ServiceNft.sol
contracts/dev/BMWToken.sol
contracts/dev/BMWTokenChild.sol
contracts/dev/ERC6551BytecodeLib.sol
contracts/dev/ERC6551Registry.sol
contracts/dev/FxERC20ChildTunnel.sol
contracts/dev/FxERC20RootTunnel.sol
contracts/dev/ProxyAdmin.sol
contracts/dev/tba/lib/ERC6551AccountLib.sol
contracts/dev/tba/lib/ERC6551BytecodeLib.sol
contracts/fun/Bonding.sol
contracts/fun/FERC20.sol
contracts/fun/FFactory.sol
contracts/fun/FPair.sol
contracts/fun/FRouter.sol
contracts/fun/IFPair.sol
contracts/genesis/FGenesis.sol
contracts/genesis/Genesis.sol
contracts/genesis/GenesisLib.sol
contracts/genesis/GenesisTypes.sol
contracts/genesis/MockAgentFactoryV3.sol
contracts/genesis/MockERC20.sol
contracts/governance/GovernorCountingSimple.sol
contracts/governance/VirtualGenesisDAO.sol
contracts/governance/VirtualProtocolDAO.sol
contracts/governance/veVirtualToken.sol
contracts/libs/AddressCheckpoints.sol
contracts/libs/Elo.sol
contracts/libs/FixedPointMathLib.sol
contracts/libs/IERC6551Registry.sol
contracts/libs/RewardSettingsCheckpoints.sol
contracts/libs/RewardSettingsCheckpointsV2.sol
contracts/libs/TokenSaver.sol
contracts/pool/AeroAdaptor.sol
contracts/pool/IRouter.sol
contracts/pool/IUniswapV2Factory.sol
contracts/pool/IUniswapV2Pair.sol
contracts/pool/IUniswapV2Router01.sol
contracts/pool/IUniswapV2Router02.sol
contracts/tax/AgentTax.sol
contracts/tax/BondingTax.sol
contracts/tax/IBondingTax.sol
contracts/tax/ITBABonus.sol
contracts/tax/LPRefund.sol
contracts/tax/TBABonus.sol
contracts/token/Airdrop.sol
contracts/token/IMinter.sol
contracts/token/Minter.sol
contracts/token/Virtual.sol
contracts/virtualPersona/AgentDAO.sol
contracts/virtualPersona/AgentFactory.sol
contracts/virtualPersona/AgentFactoryV3.sol
contracts/virtualPersona/AgentFactoryV4.sol
contracts/virtualPersona/AgentMigrator.sol
contracts/virtualPersona/AgentNftV2.sol
contracts/virtualPersona/AgentToken.sol
contracts/virtualPersona/AgentVeToken.sol
contracts/virtualPersona/CoreRegistry.sol
contracts/virtualPersona/ERC20Votes.sol
contracts/virtualPersona/EloCalculator.sol
contracts/virtualPersona/GovernorCountingSimpleUpgradeable.sol
contracts/virtualPersona/IAgentDAO.sol
contracts/virtualPersona/IAgentFactory.sol
contracts/virtualPersona/IAgentFactoryV3.sol
contracts/virtualPersona/IAgentFactoryV4.sol
contracts/virtualPersona/IAgentNft.sol
contracts/virtualPersona/IAgentToken.sol
contracts/virtualPersona/IAgentVeToken.sol
contracts/virtualPersona/IERC20Config.sol
contracts/virtualPersona/IEloCalculator.sol
contracts/virtualPersona/IErrors.sol
contracts/virtualPersona/IExecutionInterface.sol
contracts/virtualPersona/IValidatorRegistry.sol
contracts/virtualPersona/ValidatorRegistry.sol


 ## DOCUMENTATION: 

 ### virtuals-docs.md


# Overview

| Contract | Purpose | Access Control | Upgradable |
| ------ | ------ | ------ | ------ |
| veVirtualToken | This is a non-transferrable voting token to be used to vote on Virtual Protocol DAO and Virtual Genesis DAO  | Ownable | N |
| VirtualProtocolDAO | Regular DAO to maintain the VIRTUAL ecosystem | - | N |
| VirtualGenesisDAO | Used to vote for instantiation of a VIRTUAL. This DAO allows early execution of proposal as soon as quorum (10k votes) is reached. | - | N |
| AgentFactory | Handles the application & instantiation of a new VIRTUAL. References to TBA registry, VIRTUAL DAO/Token implementation and Persona NFT vault contracts are stored here. | Roles : DEFAULT_ADMIN_ROLE, WITHDRAW_ROLE | Y |
| AgentNft | This is the main registry for Persona, Core and Validator. Used to generate ICV wallet address.  | Roles: DEFAULT_ADMIN_ROLE, VALIDATOR_ADMIN_ROLE, MINTER_ROLE | Y |
| ContributionNft | Each contribution will mint a new ContributionNft. Anyone can propose a new contribution at the VIRTUAL DAO and mint token using the proposal Id.  | - | Y |
| ServiceNft | Accepted contribution will mint a ServiceNft, restricted to only VIRTUAL DAO can mint a ServiceNft. User can query the latest service NFT for a VIRTUAL CORE. | - | Y |
| AgentToken | This is implementation contract for VIRTUAL staking. AgentFactory will clone this during VIRTUAL instantiation. Staked token is non-transferable. | - | N |
| AgentDAO | This is implementation contract for VIRTUAL specific DAO. AgentFactory will clone this during VIRTUAL instantiation. It holds the maturity score for each core service. | - | N |
| AgentReward | This is reward distribution center. | Roles: GOV_ROLE, TOKEN_SAVER_ROLE | Y |
| TimeLockStaking | Allows user to stake their VIRTUAL in exchange for sVIRTUAL | Roles: GOV_ROLE, TOKEN_SAVER_ROLE | N |
| Virtual | VIRTUAL token | Ownable | N |
| Airdrop | Airdrop token to holders | - | N |

## Main Activities

### VIRTUAL Genesis

1. Submit a new application at **AgentFactory**
 a. It will transfer VIRTUAL to AgentFactory
2. Propose at **VirtualGenesisDAO** (action = ```VirtualFactory.executeApplication``` )
3. Start voting at **VirtualGenesisDAO**
4. Execute proposal at  **VirtualGenesisDAO**  , it will do following:
 a. Clone **AgentToken**
 b. Clone **AgentDAO**
 c. Mint **AgentNft**
 d. Stake VIRTUAL -> &#36;`PERSONA (depending on the symbol sent to application)
 e. Create **TBA** with **AgentNft**

### Submit Contribution

1. Create proposal at **AgentDAO** (action = ServiceNft.mint)
2. Mint **ContributionNft** , it will authenticate by checking whether sender is the proposal's proposer.

### Upgrading Core

1. Validator vote for contribution proposal at **AgentDAO**
2. Execute proposal at **AgentDAO**, it will mint a **ServiceNft**, and trigger following actions:
 a. Update maturity score
 b. Update VIRTUAL core service id.

### Distribute Reward

1. On daily basis, protocol backend will conclude daily profits into a single amount.
2. Protocol backend calls **AgentReward**.distributeRewards , triggering following:
 a. Transfer VIRTUAL into **AgentReward**
 b. Account & update claimable amounts for: Protocol, Stakers, Validators, Dataset Contributors, Model Contributors

### Claim Reward

1. Protocol calls **AgentReward**.withdrawProtocolRewards
2. Stakers, Validators, Dataset Contributors, Model Contributors calls **AgentReward**.claimAllRewards

### Staking VIRTUAL

1. Call **AgentToken**.stake , pass in the validator that you would like to delegate your voting power to. It will take in sVIRTUAL and mint &#36;`_PERSONA_ to you.
2. Call **AgentToken**.withdraw to withdraw , will burn your &#36;`_PERSONA_ and return sVIRTUAL to you.



 ## CONFIG FILES: 

 ### hardhat.config.js

/** @type import('hardhat/config').HardhatUserConfig */
require("dotenv").config();
require("@nomicfoundation/hardhat-toolbox");
require("hardhat-deploy");
require("@openzeppelin/hardhat-upgrades");
require("@fireblocks/hardhat-fireblocks");

const { ApiBaseUrl } = require("@fireblocks/fireblocks-web3-provider");

module.exports = {
  solidity: {
    version: "0.8.26",
    settings: {
      viaIR: true,
      optimizer: {
        enabled: true,
        runs: 200,
      },
    },
  },
  overrides: {
    "contracts/genesis/FGenesis.sol": {
      version: "0.8.26",
      settings: {
        optimizer: {
          enabled: true,
          runs: 200,
        },
        viaIR: false,
      },
    },
    "contracts/genesis/Genesis.sol": {
      version: "0.8.26",
      settings: {
        optimizer: {
          enabled: true,
          runs: 200,
        },
        viaIR: false,
      },
    },
  },
  namedAccounts: {
    deployer: `privatekey://${process.env.PRIVATE_KEY}`,
  },
  etherscan: {
    apiKey: process.env.ETHERSCAN_API_KEY,
    customChains: [
      {
        network: "base_sepolia",
        chainId: 84532,
        urls: {
          apiURL: "https://api-sepolia.basescan.org/api",
          browserURL: "https://sepolia.basescan.org/",
        },
      },
    ],
  },
  networks: {
    base: {
      url: "https://mainnet.base.org",
      accounts: [process.env.PRIVATE_KEY],
    },
    base_fire: {
      url: "https://mainnet.base.org",
      accounts: [process.env.PRIVATE_KEY],
      fireblocks: {
        privateKey: process.env.FIREBLOCKS_API_PRIVATE_KEY_PATH,
        apiKey: process.env.FIREBLOCKS_API_KEY,
        vaultAccountIds: process.env.FIREBLOCKS_VAULT_ACCOUNT_IDS,
      },
    },
    base_sepolia: {
      url: "https://sepolia.base.org",
      accounts: [process.env.PRIVATE_KEY],
      verify: {
        etherscan: {
          apiUrl: "https://api-sepolia.basescan.org",
          apiKey: process.env.ETHERSCAN_API_KEY,
        },
      },
    },
    base_sepolia_fire: {
      url: "https://sepolia.base.org",
      accounts: [process.env.PRIVATE_KEY],
      verify: {
        etherscan: {
          apiUrl: "https://api-sepolia.basescan.org",
          apiKey: process.env.ETHERSCAN_API_KEY,
        },
      },
      fireblocks: {
        apiBaseUrl: ApiBaseUrl.Sandbox,
        privateKey: process.env.FIREBLOCKS_API_PRIVATE_KEY_PATH,
        apiKey: process.env.FIREBLOCKS_API_KEY,
        vaultAccountIds: process.env.FIREBLOCKS_VAULT_ACCOUNT_IDS,
      },
    },
    local: {
      url: "http://127.0.0.1:8545",
    },
    polygon: {
      url: "https://rpc-mainnet.maticvigil.com/",
      accounts: [process.env.PRIVATE_KEY],
    },
    mumbai: {
      url: "https://rpc.ankr.com/polygon_mumbai",
      accounts: [process.env.PRIVATE_KEY],
    },
    goerli: {
      url: "https://rpc.ankr.com/eth_goerli",
      accounts: [process.env.PRIVATE_KEY],
    },
  },
};


### package.json

{
  "name": "virtual-contracts",
  "version": "1.0.0",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "repository": {
    "type": "git",
    "url": "git+https://github.com/PathDAO/virtual-contracts.git"
  },
  "keywords": [],
  "author": "",
  "license": "ISC",
  "bugs": {
    "url": "https://github.com/PathDAO/virtual-contracts/issues"
  },
  "homepage": "https://github.com/PathDAO/virtual-contracts#readme",
  "description": "",
  "devDependencies": {
    "@nomicfoundation/hardhat-chai-matchers": "^2.0.0",
    "@nomicfoundation/hardhat-ethers": "^3.0.0",
    "@nomicfoundation/hardhat-network-helpers": "^1.0.0",
    "@nomicfoundation/hardhat-toolbox": "^3.0.0",
    "@nomiclabs/hardhat-ethers": "npm:hardhat-deploy-ethers",
    "@nomiclabs/hardhat-etherscan": "^3.1.8",
    "@types/chai": "^4.2.0",
    "@types/mocha": ">=9.1.0",
    "ethers": "^6.9.2",
    "hardhat": "2.19.4",
    "hardhat-deploy": "^0.11.45",
    "hardhat-gas-reporter": "^1.0.8",
    "solidity-coverage": "^0.8.1",
    "ts-node": "^10.9.2",
    "typechain": "^8.2.0",
    "typescript": "^5.3.3"
  },
  "dependencies": {
    "@account-abstraction/contracts": "^0.6.0",
    "@fireblocks/hardhat-fireblocks": "^1.3.5",
    "@nomicfoundation/hardhat-verify": "^2.0.3",
    "@openzeppelin/contracts": "5.0.0",
    "@openzeppelin/contracts-upgradeable": "^5.0.1",
    "@openzeppelin/hardhat-upgrades": "^3.0.2",
    "@typechain/ethers-v6": "^0.5.1",
    "@typechain/hardhat": "^9.1.0",
    "@uniswap/v2-core": "^1.0.1",
    "chai": "^4.2.0",
    "dotenv": "^16.3.1",
    "erc6551": "^0.3.1"
  }
}


