### Storage layout (FFactory) 

```text
_pair mapping(address => mapping(address => address))
pairs address[]
router address
taxVault address
buyTax uint256
sellTax uint256

```
#### FFactory._createPair(address,address) [INTERNAL]
```slithir
tokenA_1(address) := phi(['tokenA_1'])
tokenB_1(address) := phi(['tokenB_1'])
pairs_1(address[]) := phi(['pairs_3', 'pairs_0', 'pairs_4'])
router_1(address) := phi(['router_2', 'router_0'])
 require(bool,string)(tokenA != address(0),Zero addresses are not allowed.)
TMP_8345 = CONVERT 0 to address
TMP_8346(bool) = tokenA_1 != TMP_8345
TMP_8347(None) = SOLIDITY_CALL require(bool,string)(TMP_8346,Zero addresses are not allowed.)
 require(bool,string)(tokenB != address(0),Zero addresses are not allowed.)
TMP_8348 = CONVERT 0 to address
TMP_8349(bool) = tokenB_1 != TMP_8348
TMP_8350(None) = SOLIDITY_CALL require(bool,string)(TMP_8349,Zero addresses are not allowed.)
 require(bool,string)(router != address(0),No router)
TMP_8351 = CONVERT 0 to address
TMP_8352(bool) = router_1 != TMP_8351
TMP_8353(None) = SOLIDITY_CALL require(bool,string)(TMP_8352,No router)
 pair_ = new FPair(router,tokenA,tokenB)
TMP_8355(FPair) = new FPair(router_1,tokenA_1,tokenB_1) 
pair__1(FPair) := TMP_8355(FPair)
 _pair[tokenA][tokenB] = address(pair_)
REF_3371(mapping(address => address)) -> _pair_0[tokenA_1]
REF_3372(address) -> REF_3371[tokenB_1]
TMP_8356 = CONVERT pair__1 to address
_pair_1(mapping(address => mapping(address => address))) := phi(['_pair_0'])
REF_3372(address) (->_pair_1) := TMP_8356(address)
 _pair[tokenB][tokenA] = address(pair_)
REF_3373(mapping(address => address)) -> _pair_1[tokenB_1]
REF_3374(address) -> REF_3373[tokenA_1]
TMP_8357 = CONVERT pair__1 to address
_pair_2(mapping(address => mapping(address => address))) := phi(['_pair_1'])
REF_3374(address) (->_pair_2) := TMP_8357(address)
 pairs.push(address(pair_))
TMP_8358 = CONVERT pair__1 to address
REF_3376 -> LENGTH pairs_1
TMP_8360(uint256) := REF_3376(uint256)
TMP_8361(uint256) = TMP_8360 (c)+ 1
pairs_2(address[]) := phi(['pairs_1'])
REF_3376(uint256) (->pairs_2) := TMP_8361(uint256)
REF_3377(address) -> pairs_2[TMP_8360]
pairs_3(address[]) := phi(['pairs_2'])
REF_3377(address) (->pairs_3) := TMP_8358(address)
 n = pairs.length
REF_3378 -> LENGTH pairs_3
n_1(uint256) := REF_3378(uint256)
 PairCreated(tokenA,tokenB,address(pair_),n)
TMP_8362 = CONVERT pair__1 to address
Emit PairCreated(tokenA_1,tokenB_1,TMP_8362,n_1)
 address(pair_)
TMP_8364 = CONVERT pair__1 to address
RETURN TMP_8364
```
#### FFactory.allPairsLength() [PUBLIC]
```slithir
pairs_4(address[]) := phi(['pairs_3', 'pairs_0', 'pairs_4'])
 pairs.length
REF_3381 -> LENGTH pairs_4
RETURN REF_3381
```
#### FFactory.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### FFactory.createPair(address,address) [EXTERNAL]
```slithir
CREATOR_ROLE_1(bytes32) := phi(['CREATOR_ROLE_2', 'CREATOR_ROLE_0'])
 pair = _createPair(tokenA,tokenB)
TMP_8365(address) = INTERNAL_CALL, FFactory._createPair(address,address)(tokenA_1,tokenB_1)
pair_1(address) := TMP_8365(address)
 pair
RETURN pair_1
 onlyRole(CREATOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(CREATOR_ROLE_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### FFactory.getPair(address,address) [PUBLIC]
```slithir
_pair_3(mapping(address => mapping(address => address))) := phi(['_pair_0', '_pair_3', '_pair_2'])
 _pair[tokenA][tokenB]
REF_3379(mapping(address => address)) -> _pair_3[tokenA_1]
REF_3380(address) -> REF_3379[tokenB_1]
RETURN REF_3380
```
#### FFactory.initialize(address,uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_5', 'DEFAULT_ADMIN_ROLE_0'])
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 __ReentrancyGuard_init()
INTERNAL_CALL, ReentrancyGuardUpgradeable.__ReentrancyGuard_init()()
 _grantRole(DEFAULT_ADMIN_ROLE,msg.sender)
TMP_8343(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_4,msg.sender)
 taxVault = taxVault_
taxVault_1(address) := taxVault__1(address)
 buyTax = buyTax_
buyTax_1(uint256) := buyTax__1(uint256)
 sellTax = sellTax_
sellTax_1(uint256) := sellTax__1(uint256)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### FFactory.setRouter(address) [PUBLIC]
```slithir
ADMIN_ROLE_3(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_4', 'ADMIN_ROLE_2'])
 router = router_
router_2(address) := router__1(address)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_3)
```
#### FFactory.setTaxParams(address,uint256,uint256) [PUBLIC]
```slithir
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_4', 'ADMIN_ROLE_2'])
 require(bool,string)(newVault_ != address(0),Zero addresses are not allowed.)
TMP_8368 = CONVERT 0 to address
TMP_8369(bool) = newVault__1 != TMP_8368
TMP_8370(None) = SOLIDITY_CALL require(bool,string)(TMP_8369,Zero addresses are not allowed.)
 taxVault = newVault_
taxVault_2(address) := newVault__1(address)
 buyTax = buyTax_
buyTax_2(uint256) := buyTax__1(uint256)
 sellTax = sellTax_
sellTax_2(uint256) := sellTax__1(uint256)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_1)
```
#### FERC20.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 _decimals = 18
 _checkOwner()
INTERNAL_CALL, Ownable._checkOwner()()
```
