### Storage layout (CollateralPoolTokenFactory) 

```text
implementation address

```
### Storage layout (CollateralPoolToken) 

```text
collateralPool address
tokenName string
tokenSymbol string
timelocksByAccount mapping(address => CollateralPoolToken.TimelockQueue)
ignoreTimelocked bool
initialized bool

```
#### CollateralPoolTokenFactory.constructor(address) [PUBLIC]
```slithir
 implementation = _implementation
implementation_1(address) := _implementation_1(address)
```
#### CollateralPoolTokenFactory.create(IICollateralPool,string,string) [EXTERNAL]
```slithir
TOKEN_NAME_PREFIX_1(string) := phi(['TOKEN_NAME_PREFIX_0'])
TOKEN_SYMBOL_PREFIX_1(string) := phi(['TOKEN_SYMBOL_PREFIX_0'])
implementation_2(address) := phi(['implementation_1', 'implementation_0'])
 tokenName = string.concat(TOKEN_NAME_PREFIX,_systemSuffix,-,_agentSuffix)
TMP_6638(string) = SOLIDITY_CALL string.concat()(TOKEN_NAME_PREFIX_1,_systemSuffix_1,-,_agentSuffix_1)
tokenName_1(string) := TMP_6638(string)
 tokenSymbol = string.concat(TOKEN_SYMBOL_PREFIX,_systemSuffix,-,_agentSuffix)
TMP_6639(string) = SOLIDITY_CALL string.concat()(TOKEN_SYMBOL_PREFIX_1,_systemSuffix_1,-,_agentSuffix_1)
tokenSymbol_1(string) := TMP_6639(string)
 proxy = new ERC1967Proxy(implementation,new bytes(0))
TMP_6642 = new bytes(0)
TMP_6643(ERC1967Proxy) = new ERC1967Proxy(implementation_2,TMP_6642) 
proxy_1(ERC1967Proxy) := TMP_6643(ERC1967Proxy)
 poolToken = CollateralPoolToken(address(proxy))
TMP_6644 = CONVERT proxy_1 to address
TMP_6645 = CONVERT TMP_6644 to CollateralPoolToken
poolToken_1(CollateralPoolToken) := TMP_6645(CollateralPoolToken)
 poolToken.initialize(address(_pool),tokenName,tokenSymbol)
TMP_6646 = CONVERT _pool_1 to address
HIGH_LEVEL_CALL, dest:poolToken_1(CollateralPoolToken), function:initialize, arguments:['TMP_6646', 'tokenName_1', 'tokenSymbol_1']  
 address(poolToken)
TMP_6648 = CONVERT poolToken_1 to address
RETURN TMP_6648
```
#### CollateralPoolToken.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 _IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
 _ADMIN_SLOT = 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103
 _BEACON_SLOT = 0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50
__self_1(address) := phi(['__self_0', '__self_2'])
 require(bool,string)(address(this) != __self,Function must be called through delegatecall)
TMP_6626 = CONVERT this to address
TMP_6627(bool) = TMP_6626 != __self_1
TMP_6628(None) = SOLIDITY_CALL require(bool,string)(TMP_6627,Function must be called through delegatecall)
 require(bool,string)(_getImplementation() == __self,Function must be called through active proxy)
TMP_6629(address) = INTERNAL_CALL, ERC1967Upgrade._getImplementation()()
TMP_6630(bool) = TMP_6629 == __self_2
TMP_6631(None) = SOLIDITY_CALL require(bool,string)(TMP_6630,Function must be called through active proxy)
__self_3(address) := phi(['__self_0', '__self_2'])
 require(bool,string)(address(this) == __self,UUPSUpgradeable: must not be called through delegatecall)
TMP_6632 = CONVERT this to address
TMP_6633(bool) = TMP_6632 == __self_3
TMP_6634(None) = SOLIDITY_CALL require(bool,string)(TMP_6633,UUPSUpgradeable: must not be called through delegatecall)
collateralPool_11(address) := phi(['collateralPool_8', 'collateralPool_1', 'collateralPool_3', 'collateralPool_10', 'collateralPool_0', 'collateralPool_5'])
 require(bool,error)(msg.sender == collateralPool,revert OnlyCollateralPool()())
TMP_6635(bool) = msg.sender == collateralPool_11
TMP_6636(None) = SOLIDITY_CALL revert OnlyCollateralPool()()
TMP_6637(None) = SOLIDITY_CALL require(bool,error)(TMP_6635,TMP_6636)
```
#### CollateralPoolTokenFactory.supportsInterface(bytes4) [EXTERNAL]
```slithir
 _interfaceId == type()(IERC165).interfaceId || _interfaceId == type()(IICollateralPoolTokenFactory).interfaceId
TMP_6651(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_4373(bytes4) (->None) := 33540519(bytes4)
TMP_6652(bool) = _interfaceId_1 == REF_4373
TMP_6653(type(IICollateralPoolTokenFactory)) = SOLIDITY_CALL type()(IICollateralPoolTokenFactory)
REF_4374(bytes4) (->None) := 3980672658(bytes4)
TMP_6654(bool) = _interfaceId_1 == REF_4374
TMP_6655(bool) = TMP_6652 || TMP_6654
RETURN TMP_6655
```
#### CollateralPoolTokenFactory.upgradeInitCall(address) [EXTERNAL]
```slithir
 new bytes(0)
TMP_6650 = new bytes(0)
RETURN TMP_6650
```
#### CollateralPoolToken.initialize(address,string,string) [PUBLIC]
```slithir
_collateralPool_1(address) := phi(['_collateralPool_1'])
_tokenName_1(string) := phi(['_tokenName_1'])
_tokenSymbol_1(string) := phi(['_tokenSymbol_1'])
initialized_1(bool) := phi(['initialized_2', 'initialized_0'])
 require(bool,error)(! initialized,revert AlreadyInitialized()())
TMP_6550 = UnaryType.BANG initialized_1 
TMP_6551(None) = SOLIDITY_CALL revert AlreadyInitialized()()
TMP_6552(None) = SOLIDITY_CALL require(bool,error)(TMP_6550,TMP_6551)
 initialized = true
initialized_2(bool) := True(bool)
 collateralPool = _collateralPool
collateralPool_1(address) := _collateralPool_1(address)
 tokenName = _tokenName
tokenName_1(string) := _tokenName_1(string)
 tokenSymbol = _tokenSymbol
tokenSymbol_1(string) := _tokenSymbol_1(string)
```
