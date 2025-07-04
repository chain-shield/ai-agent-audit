### Storage layout (PoolFactory) 

```text
s_pools mapping(address => address)
s_tokens mapping(address => address)

```
#### PoolFactory.constructor(address) [PUBLIC]
```slithir
 i_wethToken = wethToken
```
#### PoolFactory.createPool(address) [EXTERNAL]
```slithir
 s_pools[tokenAddress] != address(0)
 revert PoolFactory__PoolAlreadyExists(address)(tokenAddress)
 liquidityTokenName = string.concat(T-Swap ,IERC20(tokenAddress).name())
 liquidityTokenSymbol = string.concat(ts,IERC20(tokenAddress).name())
 tPool = new TSwapPool(tokenAddress,i_wethToken,liquidityTokenName,liquidityTokenSymbol)
 s_pools[tokenAddress] = address(tPool)
 s_tokens[address(tPool)] = tokenAddress
 PoolCreated(tokenAddress,address(tPool))
 address(tPool)
```
#### PoolFactory.getPool(address) [EXTERNAL]
```slithir
 s_pools[tokenAddress]
```
#### PoolFactory.getToken(address) [EXTERNAL]
```slithir
 s_tokens[pool]
```

