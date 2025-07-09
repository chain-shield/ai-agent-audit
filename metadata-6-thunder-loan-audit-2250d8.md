
## Slither Contract Summary
--ignore-compile used, if something goes wrong, consider removing the ignore compile flag
INFO:Printers:
+ Contract IERC1822Proxiable
  - From IERC1822Proxiable
    - proxiableUUID() (external)

+ Contract IERC20Errors

+ Contract IERC721Errors (Most derived contract)

+ Contract IERC1155Errors (Most derived contract)

+ Contract ERC1967Utils (Most derived contract)
  - From ERC1967Utils
    - _checkNonPayable() (private)
    - _setAdmin(address) (private)
    - _setBeacon(address) (private)
    - _setImplementation(address) (private)
    - changeAdmin(address) (internal)
    - getAdmin() (internal)
    - getBeacon() (internal)
    - getImplementation() (internal)
    - upgradeBeaconToAndCall(address,bytes) (internal)
    - upgradeToAndCall(address,bytes) (internal)

+ Contract IBeacon (Most derived contract)
  - From IBeacon
    - implementation() (external)

+ Contract ERC20
  - From Context
    - _msgData() (internal)
    - _msgSender() (internal)
  - From ERC20
    - _approve(address,address,uint256) (internal)
    - _approve(address,address,uint256,bool) (internal)
    - _burn(address,uint256) (internal)
    - _mint(address,uint256) (internal)
    - _spendAllowance(address,address,uint256) (internal)
    - _transfer(address,address,uint256) (internal)
    - _update(address,address,uint256) (internal)
    - allowance(address,address) (public)
    - approve(address,uint256) (public)
    - balanceOf(address) (public)
    - constructor(string,string) (internal)
    - decimals() (public)
    - name() (public)
    - symbol() (public)
    - totalSupply() (public)
    - transfer(address,uint256) (public)
    - transferFrom(address,address,uint256) (public)

+ Contract IERC20
  - From IERC20
    - allowance(address,address) (external)
    - approve(address,uint256) (external)
    - balanceOf(address) (external)
    - totalSupply() (external)
    - transfer(address,uint256) (external)
    - transferFrom(address,address,uint256) (external)

+ Contract IERC20Metadata
  - From IERC20
    - allowance(address,address) (external)
    - approve(address,uint256) (external)
    - balanceOf(address) (external)
    - totalSupply() (external)
    - transfer(address,uint256) (external)
    - transferFrom(address,address,uint256) (external)
  - From IERC20Metadata
    - decimals() (external)
    - name() (external)
    - symbol() (external)

+ Contract IERC20Permit (Most derived contract)
  - From IERC20Permit
    - DOMAIN_SEPARATOR() (external)
    - nonces(address) (external)
    - permit(address,address,uint256,uint256,uint8,bytes32,bytes32) (external)

+ Contract SafeERC20 (Most derived contract)
  - From SafeERC20
    - _callOptionalReturn(IERC20,bytes) (private)
    - _callOptionalReturnBool(IERC20,bytes) (private)
    - forceApprove(IERC20,address,uint256) (internal)
    - safeDecreaseAllowance(IERC20,address,uint256) (internal)
    - safeIncreaseAllowance(IERC20,address,uint256) (internal)
    - safeTransfer(IERC20,address,uint256) (internal)
    - safeTransferFrom(IERC20,address,address,uint256) (internal)

+ Contract Address (Most derived contract)
  - From Address
    - _revert(bytes) (private)
    - functionCall(address,bytes) (internal)
    - functionCallWithValue(address,bytes,uint256) (internal)
    - functionDelegateCall(address,bytes) (internal)
    - functionStaticCall(address,bytes) (internal)
    - sendValue(address,uint256) (internal)
    - verifyCallResult(bool,bytes) (internal)
    - verifyCallResultFromTarget(address,bool,bytes) (internal)

+ Contract Context
  - From Context
    - _msgData() (internal)
    - _msgSender() (internal)

+ Contract StorageSlot (Most derived contract)
  - From StorageSlot
    - getAddressSlot(bytes32) (internal)
    - getBooleanSlot(bytes32) (internal)
    - getBytes32Slot(bytes32) (internal)
    - getBytesSlot(bytes) (internal)
    - getBytesSlot(bytes32) (internal)
    - getStringSlot(bytes32) (internal)
    - getStringSlot(string) (internal)
    - getUint256Slot(bytes32) (internal)

+ Contract OwnableUpgradeable (Upgradeable)
  - From ContextUpgradeable
    - __Context_init() (internal)
    - __Context_init_unchained() (internal)
    - _msgData() (internal)
    - _msgSender() (internal)
  - From Initializable
    - _checkInitializing() (internal)
    - _disableInitializers() (internal)
    - _getInitializableStorage() (private)
    - _getInitializedVersion() (internal)
    - _isInitializing() (internal)
  - From OwnableUpgradeable
    - __Ownable_init(address) (internal)
    - __Ownable_init_unchained(address) (internal)
    - _checkOwner() (internal)
    - _getOwnableStorage() (private)
    - _transferOwnership(address) (internal)
    - owner() (public)
    - renounceOwnership() (public)
    - transferOwnership(address) (public)

+ Contract Initializable
  - From Initializable
    - _checkInitializing() (internal)
    - _disableInitializers() (internal)
    - _getInitializableStorage() (private)
    - _getInitializedVersion() (internal)
    - _isInitializing() (internal)

+ Contract UUPSUpgradeable (Upgradeable)
  - From Initializable
    - _checkInitializing() (internal)
    - _disableInitializers() (internal)
    - _getInitializableStorage() (private)
    - _getInitializedVersion() (internal)
    - _isInitializing() (internal)
  - From UUPSUpgradeable
    - __UUPSUpgradeable_init() (internal)
    - __UUPSUpgradeable_init_unchained() (internal)
    - _authorizeUpgrade(address) (internal)
    - _checkNotDelegated() (internal)
    - _checkProxy() (internal)
    - _upgradeToAndCallUUPS(address,bytes) (private)
    - proxiableUUID() (external)
    - upgradeToAndCall(address,bytes) (public)

+ Contract ContextUpgradeable (Upgradeable)
  - From Initializable
    - _checkInitializing() (internal)
    - _disableInitializers() (internal)
    - _getInitializableStorage() (private)
    - _getInitializedVersion() (internal)
    - _isInitializing() (internal)
  - From ContextUpgradeable
    - __Context_init() (internal)
    - __Context_init_unchained() (internal)
    - _msgData() (internal)
    - _msgSender() (internal)

+ Contract IFlashLoanReceiver (Most derived contract)
  - From IFlashLoanReceiver
    - executeOperation(address,uint256,uint256,address,bytes) (external)

+ Contract IPoolFactory (Most derived contract)
  - From IPoolFactory
    - getPool(address) (external)

+ Contract ITSwapPool (Most derived contract)
  - From ITSwapPool
    - getPriceOfOnePoolTokenInWeth() (external)

+ Contract IThunderLoan (Most derived contract)
  - From IThunderLoan
    - repay(address,uint256) (external)

+ Contract AssetToken (Most derived contract)
  - From ERC20
    - _approve(address,address,uint256) (internal)
    - _approve(address,address,uint256,bool) (internal)
    - _burn(address,uint256) (internal)
    - _mint(address,uint256) (internal)
    - _spendAllowance(address,address,uint256) (internal)
    - _transfer(address,address,uint256) (internal)
    - _update(address,address,uint256) (internal)
    - allowance(address,address) (public)
    - approve(address,uint256) (public)
    - balanceOf(address) (public)
    - constructor(string,string) (internal)
    - decimals() (public)
    - name() (public)
    - symbol() (public)
    - totalSupply() (public)
    - transfer(address,uint256) (public)
    - transferFrom(address,address,uint256) (public)
  - From Context
    - _msgData() (internal)
    - _msgSender() (internal)
  - From AssetToken
    - burn(address,uint256) (external)
    - constructor(address,IERC20,string,string) (public)
    - getExchangeRate() (external)
    - getUnderlying() (external)
    - mint(address,uint256) (external)
    - transferUnderlyingTo(address,uint256) (external)
    - updateExchangeRate(uint256) (external)

+ Contract OracleUpgradeable (Upgradeable)
  - From Initializable
    - _checkInitializing() (internal)
    - _disableInitializers() (internal)
    - _getInitializableStorage() (private)
    - _getInitializedVersion() (internal)
    - _isInitializing() (internal)
  - From OracleUpgradeable
    - __Oracle_init(address) (internal)
    - __Oracle_init_unchained(address) (internal)
    - getPoolFactoryAddress() (external)
    - getPrice(address) (external)
    - getPriceInWeth(address) (public)

+ Contract ThunderLoan (Upgradeable) (Most derived contract)
  - From OracleUpgradeable
    - __Oracle_init(address) (internal)
    - __Oracle_init_unchained(address) (internal)
    - getPoolFactoryAddress() (external)
    - getPrice(address) (external)
    - getPriceInWeth(address) (public)
  - From Initializable
    - _checkInitializing() (internal)
    - _disableInitializers() (internal)
    - _getInitializableStorage() (private)
    - _getInitializedVersion() (internal)
    - _isInitializing() (internal)
  - From UUPSUpgradeable
    - __UUPSUpgradeable_init() (internal)
    - __UUPSUpgradeable_init_unchained() (internal)
    - _checkNotDelegated() (internal)
    - _checkProxy() (internal)
    - _upgradeToAndCallUUPS(address,bytes) (private)
    - proxiableUUID() (external)
    - upgradeToAndCall(address,bytes) (public)
  - From OwnableUpgradeable
    - __Ownable_init(address) (internal)
    - __Ownable_init_unchained(address) (internal)
    - _checkOwner() (internal)
    - _getOwnableStorage() (private)
    - _transferOwnership(address) (internal)
    - owner() (public)
    - renounceOwnership() (public)
    - transferOwnership(address) (public)
  - From ContextUpgradeable
    - __Context_init() (internal)
    - __Context_init_unchained() (internal)
    - _msgData() (internal)
    - _msgSender() (internal)
  - From ThunderLoan
    - _authorizeUpgrade(address) (internal)
    - constructor() (public)
    - deposit(IERC20,uint256) (external)
    - flashloan(address,IERC20,uint256,bytes) (external)
    - getAssetFromToken(IERC20) (public)
    - getCalculatedFee(IERC20,uint256) (public)
    - getFee() (external)
    - getFeePrecision() (external)
    - initialize(address) (external)
    - isAllowedToken(IERC20) (public)
    - isCurrentlyFlashLoaning(IERC20) (public)
    - redeem(IERC20,uint256) (external)
    - repay(IERC20,uint256) (public)
    - setAllowedToken(IERC20,bool) (external)
    - updateFlashLoanFee(uint256) (external)

+ Contract ThunderLoanUpgraded (Upgradeable) (Most derived contract)
  - From OracleUpgradeable
    - __Oracle_init(address) (internal)
    - __Oracle_init_unchained(address) (internal)
    - getPoolFactoryAddress() (external)
    - getPrice(address) (external)
    - getPriceInWeth(address) (public)
  - From Initializable
    - _checkInitializing() (internal)
    - _disableInitializers() (internal)
    - _getInitializableStorage() (private)
    - _getInitializedVersion() (internal)
    - _isInitializing() (internal)
  - From UUPSUpgradeable
    - __UUPSUpgradeable_init() (internal)
    - __UUPSUpgradeable_init_unchained() (internal)
    - _checkNotDelegated() (internal)
    - _checkProxy() (internal)
    - _upgradeToAndCallUUPS(address,bytes) (private)
    - proxiableUUID() (external)
    - upgradeToAndCall(address,bytes) (public)
  - From OwnableUpgradeable
    - __Ownable_init(address) (internal)
    - __Ownable_init_unchained(address) (internal)
    - _checkOwner() (internal)
    - _getOwnableStorage() (private)
    - _transferOwnership(address) (internal)
    - owner() (public)
    - renounceOwnership() (public)
    - transferOwnership(address) (public)
  - From ContextUpgradeable
    - __Context_init() (internal)
    - __Context_init_unchained() (internal)
    - _msgData() (internal)
    - _msgSender() (internal)
  - From ThunderLoanUpgraded
    - _authorizeUpgrade(address) (internal)
    - constructor() (public)
    - deposit(IERC20,uint256) (external)
    - flashloan(address,IERC20,uint256,bytes) (external)
    - getAssetFromToken(IERC20) (public)
    - getCalculatedFee(IERC20,uint256) (public)
    - getFee() (external)
    - initialize(address) (external)
    - isAllowedToken(IERC20) (public)
    - isCurrentlyFlashLoaning(IERC20) (public)
    - redeem(IERC20,uint256) (external)
    - repay(IERC20,uint256) (public)
    - setAllowedToken(IERC20,bool) (external)
    - updateFlashLoanFee(uint256) (external)

INFO:Slither:6-thunder-loan-audit analyzed (26 contracts)

## List of Files in Src Folder
6-thunder-loan-audit/src/upgradedProtocol/ThunderLoanUpgraded.sol
6-thunder-loan-audit/src/protocol/AssetToken.sol
6-thunder-loan-audit/src/protocol/ThunderLoan.sol
6-thunder-loan-audit/src/protocol/OracleUpgradeable.sol
6-thunder-loan-audit/src/interfaces/IFlashLoanReceiver.sol
6-thunder-loan-audit/src/interfaces/IThunderLoan.sol
6-thunder-loan-audit/src/interfaces/ITSwapPool.sol
6-thunder-loan-audit/src/interfaces/IPoolFactory.sol
## 6-thunder-loan-audit/README.md summary
## Thunder Loan Documentation Summary

### About
The ThunderLoan protocol facilitates flash loans, allowing users to access instant loans repaid within the same transaction. Liquidity providers can earn returns by depositing assets into the platform and receive AssetTokens, accruing interest based on loan activity. A fee structure is in place determined by the TSwap on-chain price oracle.

### Getting Started
**Requirements:** Install git and foundry software to clone and manage the repository effectively. 
- **Git:** Verify installation with `git --version`.
- **Foundry:** Verify installation with `forge --version`.

**Quickstart:**
1. Clone the repository.
2. Navigate to the project directory.
3. Run `make` to set up the environment.

### Usage
To conduct tests and view test coverage, use:
- `forge test` for running tests.
- `forge coverage` for coverage data.
- `forge coverage --report debug` for in-depth coverage analysis.

### Audit Scope Details
This section details the contracts in scope for security review, including interfaces and the `ThunderLoan` and `ThunderLoanUpgraded` contracts, with emphasis on the upgrade from `ThunderLoan` to `ThunderLoanUpgraded`. Deployment is intended for Ethereum, supporting ERC20 tokens like USDC, DAI, LINK, and WETH.

### Roles
The protocol identifies three roles:
- **Owner:** Has the authority to upgrade contract implementations.
- **Liquidity Provider:** Deposits assets to gain interest.
- **User:** Engages in flash loans.

### Known Issues
Highlighted issues include zero fees for negligible loans, initial depositor advantages, and incompatibility with certain complex ERC20 tokens. These are acknowledged, with planned mitigation for some.


## 6-thunder-loan-audit/src/upgradedProtocol/ThunderLoanUpgraded.sol summary
The ThunderLoanUpgraded contract, written in Solidity, is an upgradeable smart contract that implements the ThunderLoan decentralized finance protocol features, namely flash loans. It inherits from Initializable, OwnableUpgradeable, UUPSUpgradeable, and OracleUpgradeable, leveraging these for upgradeability, ownership management, and pricing oracle integration.

Key functions include initialize(), which sets up the contract with essential variables and ownership, deposit() for depositing tokens and receiving corresponding asset tokens, redeem() for withdrawing tokens by burning asset tokens, and flashloan() for executing a flash loan operation. The contract also includes auxiliary functions for managing allowed tokens, calculating fees, and checking states of tokens. Events are emitted for deposits, token allowance changes, redemptions, and flash loans. Significant errors are defined to handle invalid states and operations.


## 6-thunder-loan-audit/src/protocol/AssetToken.sol summary
### Contract: AssetToken
The `AssetToken` contract is an ERC20 token that represents an asset tied to an underlying cryptocurrency or token. It's specifically designed to interact with a separate `ThunderLoan` contract. By leveraging SafeERC20 for secure transactions, it focuses on maintaining a dynamic exchange rate that adjusts based on operational fees. This token is immutable, implying certain variables cannot be modified once initialized.

### Storage Variables
- `i_underlying (IERC20)`: The underlying token that the `AssetToken` represents, immutable and established upon construction.
- `i_thunderLoan (address)`: Address of the `ThunderLoan` contract, used for security checks, and is immutable.
- `s_exchangeRate (uint256)`: Dynamic storage of the current exchange rate between the asset token and its underlying token, initially set to `1e18` for precision.
- `EXCHANGE_RATE_PRECISION (uint256)`: Constant precision factor `1e18` ensures consistent calculations.
- `STARTING_EXCHANGE_RATE (uint256)`: Constant (`1e18`) that initializes the starting exchange rate.

### Functions
#### constructor
```
constructor(address thunderLoan, IERC20 underlying, string memory assetName, string memory assetSymbol)
```

Initializes the `AssetToken` with references to the `ThunderLoan` and underlying token. It also sets the token's name and symbol, ensuring neither address is zero.

#### mint
```
function mint(address to, uint256 amount) external onlyThunderLoan
```
Mints the specified number of tokens to a given address, callable only by the `ThunderLoan` contract.

#### burn
```
function burn(address account, uint256 amount) external onlyThunderLoan
```
Burns a number of tokens from an account, reducing the total supply, and is restricted to `ThunderLoan` calls.

#### transferUnderlyingTo
```
function transferUnderlyingTo(address to, uint256 amount) external onlyThunderLoan
```
Transfers the underlying tokens to a specified address safely, enforced by the `ThunderLoan` contract.

#### updateExchangeRate
```
function updateExchangeRate(uint256 fee) external onlyThunderLoan
```
Calculates a new exchange rate factoring in the fee over total supply, ensuring that the rate only increases, maintaining economic stability.

#### getExchangeRate
```
function getExchangeRate() external view returns (uint256)
```
Returns the current exchange rate for the token.

#### getUnderlying
```
function getUnderlying() external view returns (IERC20)
```
Returns the IERC20 interface of the underlying token represented by the `AssetToken`. 


## 6-thunder-loan-audit/src/protocol/ThunderLoan.sol summary
The `ThunderLoan` contract is an upgradeable smart contract facilitating flash loans on the Ethereum network. It integrates multiple OpenZeppelin contracts and uses a modular architecture to manage assets and handle flash loan operations, including fee management and allowance of ERC20 tokens.

**ThunderLoan Contract**:
This contract extends several upgradable contracts and requires initialization through `initialize()`. It manages tokens and facilitates flash loans with specific fee structures.

**State Variables**:
- `s_tokenToAssetToken`: A mapping linking `IERC20` tokens to their corresponding `AssetToken`.
- `s_feePrecision`: Precision for calculating fees, set to 18 decimals for WEI.
- `s_flashLoanFee`: The fee rate for flash loans, defaulting to 0.3%.
- `s_currentlyFlashLoaning`: A mapping tracking whether a token is currently involved in a flash loan.

**Functions**:
- `initialize(address)`: Initializes the contract with owner and flash loan parameters.
- `deposit(IERC20, uint256)`: Deposits tokens, mints asset tokens, and updates the exchange rate.
- `redeem(IERC20, uint256)`: Redeems the underlying token by burning asset tokens.
- `flashloan(address, IERC20, uint256, bytes)`: Facilitates a flash loan, encompasses fee calculation and checks for repayment.
- `repay(IERC20, uint256)`: Handles repayment of flash loans.
- `setAllowedToken(IERC20, bool)`: Allows or disallows a token for flash loan operations.
- `getCalculatedFee(IERC20, uint256)`: Computes the fee for a flash loan based on token amount.
- `updateFlashLoanFee(uint256)`: Updates the flash loan fee.
- `isAllowedToken(IERC20)`, `getAssetFromToken(IERC20)`, `isCurrentlyFlashLoaning(IERC20)`, `getFee()`, `getFeePrecision()`: Getter functions for various states.
- `_authorizeUpgrade(address)`: Ensures only the owner can upgrade the contract.


## 6-thunder-loan-audit/src/protocol/OracleUpgradeable.sol summary
### OracleUpgradeable Contract Summary

**Contract Overview**

`OracleUpgradeable` is a contract that helps retrieve pricing information of tokens in terms of WETH. It makes use of a `poolFactory` which is set during initialization and used to locate relevant swap pools for tokens.

**Functions**

- `__Oracle_init(address poolFactoryAddress) internal onlyInitializing`
  - Initializes the Oracle with a provided pool factory address. Calls another function for unchained initialization.

- `__Oracle_init_unchained(address poolFactoryAddress) internal onlyInitializing`
  - Directly sets the pool factory address used by the Oracle. Similar to a constructor for upgradeable contracts.

- `getPriceInWeth(address token) public view returns (uint256)`
  - Retrieves the price of a token in WETH by interacting with the appropriate swap pool. Utilizes the IPoolFactory to locate the swap pool and then calls the pool to get the price.

- `getPrice(address token) external view returns (uint256)`
  - A wrapper around `getPriceInWeth` that serves as an external interface.

- `getPoolFactoryAddress() external view returns (address)`
  - Returns the stored pool factory address.

**Storage Variables**

- `address private s_poolFactory`
  - Holds the address of the pool factory, a central point for locating swap pools for tokens.


## 6-thunder-loan-audit/src/interfaces/IFlashLoanReceiver.sol summary
### IFlashLoanReceiver Interface

This Solidity code defines an interface for the `IFlashLoanReceiver`, inspired by the Aave protocol, specifically designed for flash loan operations. The interface includes the function:

- **`executeOperation(address token, uint256 amount, uint256 fee, address initiator, bytes calldata params) external returns (bool);`**: This function is intended to be called after a flash loan is received. It takes parameters such as the address of the token borrowed, the amount borrowed, the fee, the initiator of the loan, and any additional parameters needed for the transaction. The function returns a boolean indicating the success of the operation. This design allows the implementation contract to define what actions should be taken with the borrowed funds, such as arbitrage or refinancing, as long as the borrowed amount plus fees can be repaid to the lending protocol by the end of the transaction block. The interface is part of a project dealing with flash loans, likely requiring contracts that use it to interact with flash loan protocols.


## 6-thunder-loan-audit/src/interfaces/IThunderLoan.sol summary
The code above defines an interface called `IThunderLoan`, which is a part of a smart contract in Solidity. This interface contains a single function `repay`, intended to be called externally. It takes two parameters: `token` of type `address`, representing the token address to repay, and `amount` of type `uint256`, signifying the amount to be repaid. Since it's an interface, the actual function implementation is not provided in this contract. This interface can be implemented by any contract engaging in operations requiring a repay functionality, particularly for loans or similar financial products in a blockchain context.


## 6-thunder-loan-audit/src/interfaces/ITSwapPool.sol summary
The ITSwapPool interface provides a single function: `getPriceOfOnePoolTokenInWeth`. This function allows contracts to retrieve the price of a single pool token measured in Wrapped Ether (WETH). The function is marked as external, indicating it can be called from outside the contract and is a view function, meaning it does not modify the state of the blockchain.

### getPriceOfOnePoolTokenInWeth
```solidity
function getPriceOfOnePoolTokenInWeth() external view returns (uint256);
```
- **Summary**: Returns the price of one pool token in WETH.
- **Visibility**: external
- **Modifiers**: view
- **Returns**: A `uint256` representing the price of one pool token in terms of WETH.


## 6-thunder-loan-audit/src/interfaces/IPoolFactory.sol summary
### IPoolFactory Interface Contract

#### Overview:
The `IPoolFactory` is a simple interface in Solidity, specifying a single function designed for retrieving the pool address associated with a given token address.

#### Functions:
- **getPool**
  - **Interface**: `function getPool(address tokenAddress) external view returns (address);`
  - **Summary**: This function takes a token address as an input and returns the corresponding pool address. It is an externally accessible view function which ensures that the state is not modified when called, and it provides essential functionality for considering token pools within the system.

This interface is a crucial part of a decentralized finance or similar token management system, facilitating interactions with pool contracts by enabling retrieval of pool addresses based on token addresses.

