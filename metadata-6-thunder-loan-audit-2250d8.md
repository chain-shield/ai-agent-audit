
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
# Thunder Loan
Thunder Loan is a flash loan protocol similar to Aave and Compound, designed to facilitate flash loans and provide liquidity providers with a way to earn interest on their assets. Liquidity providers deposit assets into the ThunderLoan contract and receive AssetTokens, which accrue interest through the usage of flash loans, where users borrow and repay funds within a single transaction, paying a small fee calculated via the TSwap price oracle.

## Getting Started
Requires Git and Foundry. Installation and setup involve cloning the repo and using `make` to prepare the environment. Testing utilities are supported via Forge, e.g., `forge test` for tests and `forge coverage` for coverage reports.

## Audit Scope Details
The audit includes various interfaces and protocol files like AssetToken, OracleUpgradeable, ThunderLoan, and ThunderLoanUpgraded. The upcoming upgrade to ThunderLoanUpgraded should be considered in the security review. The code targets the Ethereum blockchain with Solidity 0.8.20, and handles ERC20s like USDC, DAI, LINK, and WETH.

## Roles
Three main roles are defined: Owner, who can upgrade the system; Liquidity Providers, who earn interest by depositing assets; and Users, who take out flash loans.

## Known Issues
Known issues include zero fees for small loans due to rounding, the first depositor's advantage in token distribution, and incompatibility with certain "weird" ERC20 tokens. These issues have been acknowledged and mitigation strategies, like initial deposits and token vetting, are in place.


## 6-thunder-loan-audit/src/upgradedProtocol/ThunderLoanUpgraded.sol summary
**ThunderLoanUpgraded Contract** is an upgradable smart contract that extends the functionality of thunder loans with flash loans using the UUPS upgradeability pattern. It allows for depositing, redeeming, and executing secured flash loans with verification mechanisms to ensure loan repaid along with accurately calculated fees.

- **Storage Variables:**
  - `s_tokenToAssetToken`: A mapping associating ERC20 tokens to their corresponding AssetToken contracts, serving as wrappers for liquidity management.
  - `s_flashLoanFee`: A uint variable determining fee percentage for flash loans. Default is 0.3% ETH.
  - `s_currentlyFlashLoaning`: A mapping keeping track of tokens currently being flash loaned.

**Functions**

- `initialize(address tswapAddress)`: This function initializes the contract, setting the owner, upgrade, and Oracle functionalities.

- `deposit`: Allows depositing of ERC20 tokens, verifying non-zero amount and allowance.

- `redeem`: Withdraws tokens by redeeming the equivalent amount in AssetTokens.

- `flashloan`: Initiates a flash loan, ensuring repayability and fee deductions.

- `repay`: Used to repay a flash loan, checking current flash loan status.

- `setAllowedToken`: This function sets or removes a token's allowed status for thunder loan services.

- `getCalculatedFee`: Calculates the loan fee based on token value and loan fee percentage.

- `updateFlashLoanFee`: Updates the flash loan fee percentage, ensuring it does not exceed 100%.

- `isAllowedToken`: Checks if a token is allowed for servicing by the contract.

- `getAssetFromToken`: Fetches the AssetToken associated with a given ERC20 token.

- `isCurrentlyFlashLoaning`: Verifies if a specific token is currently involved in a flash loan.

- `getFee`: Retrieves the current flash loan fee.

- `_authorizeUpgrade`: Ensures that the contract upgrade authorization is restricted to the owner.


## 6-thunder-loan-audit/src/protocol/AssetToken.sol summary
### Contract: AssetToken

The `AssetToken` contract extends the `ERC20` token to represent a customized asset token within a decentralized finance application. This contract is designed for integration with a larger system, specifically interfacing with a component called `ThunderLoan`. It includes key features like minting, burning, and an adjustable exchange rate tied to the underlying asset value relative to the tokens.

### Storage Variables:

- **i_underlying: IERC20**
  The ERC20 token that represents the underlying asset backing the asset token.
  
- **i_thunderLoan: address**
  The address of the `ThunderLoan` contract. Only this address can perform certain operations on the asset token.

- **s_exchangeRate: uint256**
  Controls the value exchange rate between asset token and its underlying asset, initialized to `1e18`.

### Functions:

- **constructor(address thunderLoan, IERC20 underlying, string memory assetName, string memory assetSymbol)**
  Initializes the contract, setting up the ThunderLoan integration, underlying asset, and token metadata.

- **mint(address to, uint256 amount)**
  Mints a specific amount of new asset tokens to a given address, restricted to ThunderLoan.

- **burn(address account, uint256 amount)**
  Destroys a given amount of asset tokens from a specified address, also restricted to ThunderLoan.

- **transferUnderlyingTo(address to, uint256 amount)**
  Transfers a specified amount of the underlying asset to a certain address, exclusively callable by ThunderLoan.

- **updateExchangeRate(uint256 fee)**
  Adjusts the token's exchange rate with its underlying asset, ensuring it can only increase. The exchange rate is affected by the fee and total supply.

- **getExchangeRate()**
  Returns the current exchange rate between the asset token and its underlying asset.

- **getUnderlying()**
  Provides access to the underlying asset token instance.


## 6-thunder-loan-audit/src/protocol/ThunderLoan.sol summary
The Slither contract summary outlines the analysis of several Ethereum contracts related to the ThunderLoan system. The report summarizes the structure of multiple contracts, including ERC20 interfaces, upgradeable and non-upgradeable contracts, and various extension utilities derived from OpenZeppelin libraries. Notably, contracts like `ThunderLoan`, `ThunderLoanUpgraded`, `AssetToken`, and `OracleUpgradeable` provide functionalities associated with lending protocols, asset management, and token exchanges.

The document highlights key contracts like `ThunderLoan`, which facilitates flash loans and integrates with other components such as `AssetToken` for handling tokenized assets and `OracleUpgradeable` for price data. Several interfaces defined provide a structure to these interactions, ensuring modularity and interface compliance, critical in audited systems. The summary succinctly describes key methods, modifiers, and event emissions within these contracts, indicating a robust framework designed to support safe and efficient operations in decentralized finance (DeFi) systems.


## 6-thunder-loan-audit/src/protocol/OracleUpgradeable.sol summary
### OracleUpgradeable Contract Summary
The `OracleUpgradeable` is a smart contract that provides functionality to get the price of a token in Ether (WETH). It uses interfaces `ITSwapPool` and `IPoolFactory` to interact with different pools and obtain token pricing. This contract is upgradeable via OpenZeppelin's `Initializable`.

#### Contract Definition
```solidity
contract OracleUpgradeable is Initializable
```

#### Storage Variables
- `address private s_poolFactory` :
  A private storage variable for the address of the pool factory. This variable holds the address of the factory responsible for creating token swap pools.

#### Function: __Oracle_init
```solidity
function __Oracle_init(address poolFactoryAddress) internal onlyInitializing
```
- Initializes the Oracle contract with a specific pool factory address. Utilizes OpenZeppelin's `onlyInitializing` modifier to ensure initialization can only occur once.

#### Function: __Oracle_init_unchained
```solidity
function __Oracle_init_unchained(address poolFactoryAddress) internal onlyInitializing
```
- A continuation of `__Oracle_init`, responsible for setting the pool factory address.

#### Function: getPriceInWeth
```solidity
function getPriceInWeth(address token) public view returns (uint256)
```
- Retrieves the price of one pool token in WETH for a given token by interacting with the swap pool.

#### Function: getPrice
```solidity
function getPrice(address token) external view returns (uint256)
```
- An external interface to get token prices in WETH by calling the `getPriceInWeth` function.

#### Function: getPoolFactoryAddress
```solidity
function getPoolFactoryAddress() external view returns (address)
```
- Returns the address of the pool factory, providing external visibility to this private variable.


## 6-thunder-loan-audit/src/interfaces/IFlashLoanReceiver.sol summary
The `IFlashLoanReceiver` interface defines a single function for implementing flash loans, inspired by Aave's protocol. It works in conjunction with the `IThunderLoan` interface, requiring a function called `executeOperation`. The interface reflects Aave's flash loan strategy, allowing a contract to receive, use, and repay a loan within a single transaction.


## 6-thunder-loan-audit/src/interfaces/IThunderLoan.sol summary
The provided code defines an interface `IThunderLoan` for a smart contract in Solidity. This interface contains a single function:

### Interface: IThunderLoan

- **Function:** `repay`
  - **Interface:** `function repay(address token, uint256 amount) external;`
  - **Summary:** This function allows external calls to repay a specified `amount` of a particular `token`. Implementations of this interface must define the logic for handling the repayment process, likely involving transferring tokens from the caller to a designated address within the smart contract.

The `pragma solidity 0.8.20;` line specifies that the contract is intended to be compiled with version 0.8.20 of Solidity, ensuring compatibility with its language features and fixes.


## 6-thunder-loan-audit/src/interfaces/ITSwapPool.sol summary
## Contract: ITSwapPool
The `ITSwapPool` is an interface that outlines a single function used in the context of a token pool pricing mechanism within the Ethereum blockchain environment.

### Function Summary
- **getPriceOfOnePoolTokenInWeth()**
  - **Interface**: `function getPriceOfOnePoolTokenInWeth() external view returns (uint256);`
  - **Summary**: This function retrieves the price of one pool token in terms of WETH. It is an external view function, meaning it can be called from outside the contract and doesn't modify the state.

This interface serves as a contract blueprint to ensure any implementing contracts have this specific pricing function, crucial for applications involving token exchange rates in decentralized finance (DeFi) platforms, where price calculations in terms of WETH (Wrapped Ether) are common.


## 6-thunder-loan-audit/src/interfaces/IPoolFactory.sol summary
### Interface: IPoolFactory
The `IPoolFactory` is a straightforward interface contract written in Solidity for version 0.8.20. 

#### Function: getPool

- **Definition:** `function getPool(address tokenAddress) external view returns (address);`
- **Summary:** This interface declares a single function, `getPool`, which is designed to retrieve the address of a pool associated with a given token address. The function takes one parameter: the `tokenAddress`, which is the address of the token linked to the pool, and it returns the pool's address. The function is marked as `external` and `view`, indicating it is accessible outside the contract and does not modify the blockchain state.

