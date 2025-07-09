### Storage layout (OracleUpgradeable) 

```text
s_poolFactory address
s_poolFactory address

```


#### OracleUpgradeable.__Oracle_init(address) [INTERNAL]
```slithir
 __Oracle_init_unchained(poolFactoryAddress)
INTERNAL_CALL, OracleUpgradeable.__Oracle_init_unchained(address)(poolFactoryAddress_1)
 onlyInitializing()
MODIFIER_CALL, Initializable.onlyInitializing()()
```
#### OracleUpgradeable.__Oracle_init_unchained(address) [INTERNAL]
```slithir
poolFactoryAddress_1(address) := phi(['poolFactoryAddress_1'])
 s_poolFactory = poolFactoryAddress
s_poolFactory_1(address) := poolFactoryAddress_1(address)
 onlyInitializing()
MODIFIER_CALL, Initializable.onlyInitializing()()
```
#### OracleUpgradeable.getPriceInWeth(address) [PUBLIC]
```slithir
token_1(address) := phi(['token_1'])
s_poolFactory_2(address) := phi(['s_poolFactory_1', 's_poolFactory_0', 's_poolFactory_3'])
 swapPoolOfToken = IPoolFactory(s_poolFactory).getPool(token)
TMP_449 = CONVERT s_poolFactory_2 to IPoolFactory
TMP_450(address) = HIGH_LEVEL_CALL, dest:TMP_449(IPoolFactory), function:getPool, arguments:['token_1']  
s_poolFactory_3(address) := phi(['s_poolFactory_1', 's_poolFactory_3', 's_poolFactory_2'])
swapPoolOfToken_1(address) := TMP_450(address)
 ITSwapPool(swapPoolOfToken).getPriceOfOnePoolTokenInWeth()
TMP_451 = CONVERT swapPoolOfToken_1 to ITSwapPool
TMP_452(uint256) = HIGH_LEVEL_CALL, dest:TMP_451(ITSwapPool), function:getPriceOfOnePoolTokenInWeth, arguments:[]  
RETURN TMP_452
```
#### OracleUpgradeable.getPrice(address) [EXTERNAL]
```slithir
 getPriceInWeth(token)
TMP_453(uint256) = INTERNAL_CALL, OracleUpgradeable.getPriceInWeth(address)(token_1)
RETURN TMP_453
```



