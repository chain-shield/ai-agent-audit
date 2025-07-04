
## List of Files in Src Folder
src/PoolFactory.sol
src/TSwapPool.sol

## README.md summary
## TSwap Overview
TSwap is a decentralized exchange (DEX) functioning as an Automated Market Maker (AMM) for swapping tokens without a traditional order book. It uses "pools" to facilitate these swaps, similar to Uniswap, with WETH as the constant base asset. Each pool is a pair between any ERC20 token and WETH.

## TSwap Pools
The protocol utilizes a `PoolFactory` contract for creating pools that each operate as an exchange between a specific ERC20 token and WETH. Users can swap tokens via two main functions, `swapExactInput` and `swapExactOutput`, allowing swaps according to input or desired output quantities.

## Liquidity Providers
Liquidity Providers (LPs) supply tokens to the pools and receive LP tokens representing their share. They earn a 0.3% fee per swap conducted with their tokens. As an incentive, these fees slightly adjust the pool's ratio.

## Core Invariant
The AMM maintains a balance between the token and WETH using the constant product formula `x * y = k`, where fees adjust the invariant. This ensures stable ratios for swaps.

## Getting Started & Usage
To start with TSwap, install Git and Foundry, clone the repository, and use `forge` for testing and coverage."

## Audit Scope & Roles
The code audit focuses on the primary contracts in the `/src` folder using Solc 0.8.20. Key roles include LPs, who add liquidity and profit from swap fees, and regular users looking to trade tokens. There are no known issues with the code as of this summary.


## src/TSwapPool.sol summary
### Contract Summary

**TSwapPool**: This contract implements a token swap and liquidity pool functionality. It manages liquidity provision and token swaps between WETH and pool tokens. The contract extends the ERC20 standard, allowing it to mint and burn liquidity tokens when liquidity is added or removed.

### Function Summaries

- **Constructor**: Initializes the liquidity pool by specifying `poolToken`, `wethToken`, `liquidityTokenName`, and `liquidityTokenSymbol`. Sets immutable token contract references and initializes the ERC20 liquidity token.
  
- **deposit**: Adds liquidity to the pool. Users provide WETH, and pool tokens are deposited into the contract to maintain the invariant ratio of tokens. Liquidity tokens are minted and the event is emitted.
- **_addLiquidityMintAndTransfer**: Private helper to mint liquidity tokens and transfer WETH and pool tokens to the contract.
- **withdraw**: Burns liquidity tokens to withdraw proportional amounts of WETH and pool tokens, ensuring minimum amounts are met.
- **getOutputAmountBasedOnInput**: Computes the amount of output tokens given an input amount, maintaining x*y=k invariant.
- **getInputAmountBasedOnOutput**: Calculates required input tokens for a desired output token amount.
- **swapExactInput**: Facilitates swapping tokens based on an exact input amount. Verifies minimum output before proceeding.
- **swapExactOutput**: Determines necessary input tokens for an exact output amount, then executes the swap.
- **sellPoolTokens**: Sells pool tokens in exchange for WETH, facilitating user redemption.
- **_swap**: Internal function executing token swaps; rewards users with extra tokens for every 10 swaps.
  
### Variable Summaries

- **i_wethToken**: (IERC20) Immutable reference to the WETH token contract, used for transactions and accounting.
- **i_poolToken**: (IERC20) Immutable reference to the pool token contract, interacts with deposited pool tokens.
- **MINIMUM_WETH_LIQUIDITY**: (uint256) Constant setting the minimum WETH required for any liquidity deposit.
- **swap_count**: (uint256) Tracks swaps performed; resets after reaching SWAP_COUNT_MAX to provide user rewards.
- **SWAP_COUNT_MAX**: (uint256) Constant threshold for swap_count to trigger user reward incentives.


## src/PoolFactory.sol summary
### Contract: PoolFactory
The `PoolFactory` contract is responsible for the creation and management of `TSwapPool` instances. It tracks pools associated with ERC20 tokens and manages their lifecycle events. The contract includes safeguards to prevent the creation of duplicate pools for the same token.

#### Function: constructor
```solidity
constructor(address wethToken)
```
- **Summary**: Initializes the contract with a specified WETH token address, setting it as an immutable variable.

#### Function: createPool
```solidity
function createPool(address tokenAddress) external returns (address)
```
- **Summary**: Creates a new pool for a given token address if it does not already exist, constructs the liquidity token name and symbol, and emits a `PoolCreated` event. Throws an error if the pool already exists.

#### Function: getPool
```solidity
function getPool(address tokenAddress) external view returns (address)
```
- **Summary**: Fetches the pool address associated with a given token address, enabling external visibility of the mapping.

#### Function: getToken
```solidity
function getToken(address pool) external view returns (address)
```
- **Summary**: Retrieves the token address associated with a given pool address, providing a reverse lookup capability.

#### Function: getWethToken
```solidity
function getWethToken() external view returns (address)
```
- **Summary**: Returns the immutable WETH token address defined at contract deployment.

#### Variable: s_pools
```solidity
mapping(address token => address pool) private s_pools;
```
- **Explanation**: A mapping of token addresses to their respective pool addresses, maintaining the linkage between tokens and corresponding pools.

#### Variable: s_tokens
```solidity
mapping(address pool => address token) private s_tokens;
```
- **Explanation**: A mapping of pool addresses to their respective token addresses, allowing reverse look-up capability from pool to token.

#### Variable: i_wethToken
```solidity
address private immutable i_wethToken;
```
- **Explanation**: The address of the immutable WETH token used within the pool factory for operations.

