

### Storage layout (BondingTax) 

```text
assetToken address
taxToken address
router IRouter
bondingRouter address
treasury address
minSwapThreshold uint256
maxSwapThreshold uint256
_slippage uint16

```


#### BondingTax.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### BondingTax.initialize(address,address,address,address,address,address,uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_5'])
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_12', 'ADMIN_ROLE_8', 'ADMIN_ROLE_6', 'ADMIN_ROLE_4', 'ADMIN_ROLE_0', 'ADMIN_ROLE_10'])
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 _grantRole(ADMIN_ROLE,defaultAdmin_)
TMP_11012(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(ADMIN_ROLE_3,defaultAdmin__1)
 _grantRole(DEFAULT_ADMIN_ROLE,defaultAdmin_)
TMP_11013(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_4,defaultAdmin__1)
 assetToken = assetToken_
assetToken_1(address) := assetToken__1(address)
 taxToken = taxToken_
taxToken_1(address) := taxToken__1(address)
 router = IRouter(router_)
TMP_11014 = CONVERT router__1 to IRouter
router_1(IRouter) := TMP_11014(IRouter)
 bondingRouter = bondingRouter_
bondingRouter_1(address) := bondingRouter__1(address)
 treasury = treasury_
treasury_1(address) := treasury__1(address)
 minSwapThreshold = minSwapThreshold_
minSwapThreshold_1(uint256) := minSwapThreshold__1(uint256)
 maxSwapThreshold = maxSwapThreshold_
maxSwapThreshold_1(uint256) := maxSwapThreshold__1(uint256)
 IERC20(taxToken).forceApprove(router_,type()(uint256).max)
TMP_11015 = CONVERT taxToken_1 to IERC20
TMP_11017(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.forceApprove(IERC20,address,uint256), arguments:['TMP_11015', 'router__1', 'TMP_11017'] 
 _slippage = 100
_slippage_1(uint16) := 100(uint256)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### AgentTax.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
 ADMIN_ROLE = keccak256(bytes)(ADMIN_ROLE)
 EXECUTOR_ROLE = keccak256(bytes)(EXECUTOR_ROLE)
 DENOM = 10000
role_1(bytes32) := phi(['TMP_10780', 'EXECUTOR_ROLE_3', 'ADMIN_ROLE_17', 'TMP_10773', 'ADMIN_ROLE_11', 'ADMIN_ROLE_9', 'ADMIN_ROLE_7', 'EXECUTOR_ROLE_1', 'ADMIN_ROLE_5', 'TMP_10778', 'TMP_10775'])
 _checkRole(role)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(role_1)
 $ = _getInitializableStorage()
TMP_10930(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_10930'])(Initializable.InitializableStorage) := TMP_10930(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_4468(bool) -> $_1 (-> ['TMP_10930'])._initializing
TMP_10931 = UnaryType.BANG REF_4468 
isTopLevelCall_1(bool) := TMP_10931(bool)
 initialized = $._initialized
REF_4469(uint64) -> $_1 (-> ['TMP_10930'])._initialized
initialized_1(uint64) := REF_4469(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_10932(bool) = initialized_1 == 0
TMP_10933(bool) = TMP_10932 && isTopLevelCall_1
initialSetup_1(bool) := TMP_10933(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_10934(bool) = initialized_1 == 1
TMP_10935 = CONVERT this to address
TMP_10936(bytes) = SOLIDITY_CALL code(address)(TMP_10935)
REF_4470 -> LENGTH TMP_10936
TMP_10937(bool) = REF_4470 == 0
TMP_10938(bool) = TMP_10934 && TMP_10937
construction_1(bool) := TMP_10938(bool)
 ! initialSetup && ! construction
TMP_10939 = UnaryType.BANG initialSetup_1 
TMP_10940 = UnaryType.BANG construction_1 
TMP_10941(bool) = TMP_10939 && TMP_10940
CONDITION TMP_10941
 revert InvalidInitialization()()
TMP_10942(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_4471(uint64) -> $_1 (-> ['TMP_10930'])._initialized
$_2 (-> ['TMP_10930'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_10930'])"])
REF_4471(uint64) (->$_2 (-> ['TMP_10930'])) := 1(uint256)
TMP_10930(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_10930'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_4472(bool) -> $_2 (-> ['TMP_10930'])._initializing
$_3 (-> ['TMP_10930'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_10930'])"])
REF_4472(bool) (->$_3 (-> ['TMP_10930'])) := True(bool)
TMP_10930(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_10930'])"])
$_4 (-> ['TMP_10930'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_10930'])", "$_2 (-> ['TMP_10930'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_4473(bool) -> $_4 (-> ['TMP_10930'])._initializing
$_5 (-> ['TMP_10930'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_10930'])"])
REF_4473(bool) (->$_5 (-> ['TMP_10930'])) := False(bool)
TMP_10930(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_10930'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_10944(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_10944'])(Initializable.InitializableStorage) := TMP_10944(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_4474(bool) -> $_1 (-> ['TMP_10944'])._initializing
REF_4475(uint64) -> $_1 (-> ['TMP_10944'])._initialized
TMP_10945(bool) = REF_4475 >= version_1
TMP_10946(bool) = REF_4474 || TMP_10945
CONDITION TMP_10946
 revert InvalidInitialization()()
TMP_10947(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_4476(uint64) -> $_1 (-> ['TMP_10944'])._initialized
$_2 (-> ['TMP_10944'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_10944'])"])
REF_4476(uint64) (->$_2 (-> ['TMP_10944'])) := version_1(uint64)
TMP_10944(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_10944'])"])
 $._initializing = true
REF_4477(bool) -> $_2 (-> ['TMP_10944'])._initializing
$_3 (-> ['TMP_10944'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_10944'])"])
REF_4477(bool) (->$_3 (-> ['TMP_10944'])) := True(bool)
TMP_10944(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_10944'])"])
 $._initializing = false
REF_4478(bool) -> $_3 (-> ['TMP_10944'])._initializing
$_4 (-> ['TMP_10944'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_10944'])"])
REF_4478(bool) (->$_4 (-> ['TMP_10944'])) := False(bool)
TMP_10944(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_10944'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
```
#### BondingTax.swapForAsset() [PUBLIC]
```slithir
assetToken_5(address) := phi(['assetToken_0', 'assetToken_1', 'assetToken_4', 'assetToken_7'])
taxToken_4(address) := phi(['taxToken_6', 'taxToken_3', 'taxToken_0', 'taxToken_1'])
router_5(IRouter) := phi(['router_7', 'router_1', 'router_4', 'router_0', 'router_9'])
treasury_8(address) := phi(['treasury_12', 'treasury_10', 'treasury_7', 'treasury_4', 'treasury_0', 'treasury_1'])
minSwapThreshold_5(uint256) := phi(['minSwapThreshold_0', 'minSwapThreshold_7', 'minSwapThreshold_4', 'minSwapThreshold_1'])
maxSwapThreshold_5(uint256) := phi(['maxSwapThreshold_1', 'maxSwapThreshold_4', 'maxSwapThreshold_7', 'maxSwapThreshold_0'])
_slippage_3(uint16) := phi(['_slippage_1', '_slippage_6', '_slippage_2', '_slippage_5', '_slippage_0'])
 amount = IERC20(taxToken).balanceOf(address(this))
TMP_11040 = CONVERT taxToken_5 to IERC20
TMP_11041 = CONVERT this to address
TMP_11042(uint256) = HIGH_LEVEL_CALL, dest:TMP_11040(IERC20), function:balanceOf, arguments:['TMP_11041']  
assetToken_7(address) := phi(['assetToken_1', 'assetToken_4', 'assetToken_6', 'assetToken_7'])
taxToken_6(address) := phi(['taxToken_5', 'taxToken_6', 'taxToken_3', 'taxToken_1'])
router_7(IRouter) := phi(['router_7', 'router_6', 'router_1', 'router_4', 'router_9'])
treasury_10(address) := phi(['treasury_12', 'treasury_10', 'treasury_9', 'treasury_7', 'treasury_4', 'treasury_1'])
minSwapThreshold_7(uint256) := phi(['minSwapThreshold_6', 'minSwapThreshold_7', 'minSwapThreshold_4', 'minSwapThreshold_1'])
maxSwapThreshold_7(uint256) := phi(['maxSwapThreshold_1', 'maxSwapThreshold_7', 'maxSwapThreshold_6', 'maxSwapThreshold_4'])
_slippage_5(uint16) := phi(['_slippage_1', '_slippage_6', '_slippage_2', '_slippage_5', '_slippage_4'])
amount_1(uint256) := TMP_11042(uint256)
 require(bool,string)(amount > 0,Nothing to be swapped)
TMP_11043(bool) = amount_1 > 0
TMP_11044(None) = SOLIDITY_CALL require(bool,string)(TMP_11043,Nothing to be swapped)
 amount < minSwapThreshold
TMP_11045(bool) = amount_1 < minSwapThreshold_7
CONDITION TMP_11045
 (false,0)
RETURN False,0
 amount > maxSwapThreshold
TMP_11046(bool) = amount_1 > maxSwapThreshold_7
CONDITION TMP_11046
 amount = maxSwapThreshold
amount_2(uint256) := maxSwapThreshold_7(uint256)
amount_3(uint256) := phi(['amount_1', 'amount_2'])
 path = new address[](2)
TMP_11048(address[])  = new address[](2)
path_1(address[]) = ['TMP_11048(address[])']
 path[0] = taxToken
REF_4510(address) -> path_1[0]
path_2(address[]) := phi(['path_1'])
REF_4510(address) (->path_2) := taxToken_6(address)
 path[1] = assetToken
REF_4511(address) -> path_2[1]
path_3(address[]) := phi(['path_2'])
REF_4511(address) (->path_3) := assetToken_7(address)
 amountsOut = router.getAmountsOut(amount,path)
TMP_11049(uint256[]) = HIGH_LEVEL_CALL, dest:router_7(IRouter), function:getAmountsOut, arguments:['amount_3', 'path_3']  
router_8(IRouter) := phi(['router_7', 'router_1', 'router_9', 'router_4'])
treasury_11(address) := phi(['treasury_12', 'treasury_10', 'treasury_7', 'treasury_4', 'treasury_1'])
_slippage_6(uint16) := phi(['_slippage_6', '_slippage_5', '_slippage_1', '_slippage_2'])
amountsOut_1(uint256[]) = ['TMP_11049(uint256[])']
 require(bool,string)(amountsOut.length > 1,Failed to fetch token price)
REF_4513 -> LENGTH amountsOut_1
TMP_11050(bool) = REF_4513 > 1
TMP_11051(None) = SOLIDITY_CALL require(bool,string)(TMP_11050,Failed to fetch token price)
 expectedOutput = amountsOut[1]
REF_4514(uint256) -> amountsOut_1[1]
expectedOutput_1(uint256) := REF_4514(uint256)
 minOutput = (expectedOutput * (10000 - _slippage)) / 10000
TMP_11052(uint256) = 10000 (c)- _slippage_6
TMP_11053(uint256) = expectedOutput_1 (c)* TMP_11052
TMP_11054(uint256) = TMP_11053 (c)/ 10000
minOutput_1(uint256) := TMP_11054(uint256)
 amounts = router.swapExactTokensForTokens(amount,minOutput,path,treasury,block.timestamp + 300)
TMP_11055(uint256) = block.timestamp (c)+ 300
TMP_11056(uint256[]) = HIGH_LEVEL_CALL, dest:router_8(IRouter), function:swapExactTokensForTokens, arguments:['amount_3', 'minOutput_1', 'path_3', 'treasury_11', 'TMP_11055']  
router_9(IRouter) := phi(['router_7', 'router_1', 'router_4', 'router_8', 'router_9'])
treasury_12(address) := phi(['treasury_11', 'treasury_12', 'treasury_10', 'treasury_7', 'treasury_4', 'treasury_1'])
amounts_1(uint256[]) = ['TMP_11056(uint256[])']
 SwapExecuted(amount,amounts[1])
REF_4516(uint256) -> amounts_1[1]
Emit SwapExecuted(amount_3,REF_4516)
 (true,amounts[1])
REF_4517(uint256) -> amounts_1[1]
RETURN True,REF_4517
 SwapFailed(amount)
Emit SwapFailed(amount_3)
 (false,0)
RETURN False,0
 onlyBondingRouter()
MODIFIER_CALL, BondingTax.onlyBondingRouter()()
```
#### BondingTax.updateSwapParams(address,address,address,uint16) [PUBLIC]
```slithir
ADMIN_ROLE_5(bytes32) := phi(['ADMIN_ROLE_12', 'ADMIN_ROLE_8', 'ADMIN_ROLE_6', 'ADMIN_ROLE_4', 'ADMIN_ROLE_0', 'ADMIN_ROLE_10'])
assetToken_2(address) := phi(['assetToken_0', 'assetToken_1', 'assetToken_4', 'assetToken_7'])
taxToken_2(address) := phi(['taxToken_6', 'taxToken_3', 'taxToken_0', 'taxToken_1'])
router_2(IRouter) := phi(['router_7', 'router_1', 'router_4', 'router_0', 'router_9'])
bondingRouter_2(address) := phi(['bondingRouter_0', 'bondingRouter_4', 'bondingRouter_1', 'bondingRouter_6'])
 oldRouter = address(router)
TMP_11020 = CONVERT router_3 to address
oldRouter_1(address) := TMP_11020(address)
 oldBondingRouter = bondingRouter
oldBondingRouter_1(address) := bondingRouter_3(address)
 oldAsset = assetToken
oldAsset_1(address) := assetToken_3(address)
 assetToken = assetToken_
assetToken_4(address) := assetToken__1(address)
 router = IRouter(router_)
TMP_11021 = CONVERT router__1 to IRouter
router_4(IRouter) := TMP_11021(IRouter)
 bondingRouter = bondingRouter_
bondingRouter_4(address) := bondingRouter__1(address)
 _slippage = slippage_
_slippage_2(uint16) := slippage__1(uint16)
 IERC20(taxToken).forceApprove(router_,type()(uint256).max)
TMP_11022 = CONVERT taxToken_3 to IERC20
TMP_11024(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.forceApprove(IERC20,address,uint256), arguments:['TMP_11022', 'router__1', 'TMP_11024'] 
 IERC20(taxToken).forceApprove(oldRouter,0)
TMP_11026 = CONVERT taxToken_3 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.forceApprove(IERC20,address,uint256), arguments:['TMP_11026', 'oldRouter_1', '0'] 
 SwapParamsUpdated(oldRouter,router_,oldBondingRouter,bondingRouter_,oldAsset,assetToken_)
Emit SwapParamsUpdated(oldRouter_1,router__1,oldBondingRouter_1,bondingRouter__1,oldAsset_1,assetToken__1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_5)
```
#### BondingTax.updateSwapThresholds(uint256,uint256) [PUBLIC]
```slithir
ADMIN_ROLE_7(bytes32) := phi(['ADMIN_ROLE_12', 'ADMIN_ROLE_8', 'ADMIN_ROLE_6', 'ADMIN_ROLE_4', 'ADMIN_ROLE_0', 'ADMIN_ROLE_10'])
minSwapThreshold_2(uint256) := phi(['minSwapThreshold_0', 'minSwapThreshold_7', 'minSwapThreshold_4', 'minSwapThreshold_1'])
maxSwapThreshold_2(uint256) := phi(['maxSwapThreshold_1', 'maxSwapThreshold_4', 'maxSwapThreshold_7', 'maxSwapThreshold_0'])
 oldMin = minSwapThreshold
oldMin_1(uint256) := minSwapThreshold_3(uint256)
 oldMax = maxSwapThreshold
oldMax_1(uint256) := maxSwapThreshold_3(uint256)
 minSwapThreshold = minSwapThreshold_
minSwapThreshold_4(uint256) := minSwapThreshold__1(uint256)
 maxSwapThreshold = maxSwapThreshold_
maxSwapThreshold_4(uint256) := maxSwapThreshold__1(uint256)
 SwapThresholdUpdated(oldMin,minSwapThreshold_,oldMax,maxSwapThreshold_)
Emit SwapThresholdUpdated(oldMin_1,minSwapThreshold__1,oldMax_1,maxSwapThreshold__1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_7)
```
#### BondingTax.updateTreasury(address) [PUBLIC]
```slithir
ADMIN_ROLE_9(bytes32) := phi(['ADMIN_ROLE_12', 'ADMIN_ROLE_8', 'ADMIN_ROLE_6', 'ADMIN_ROLE_4', 'ADMIN_ROLE_0', 'ADMIN_ROLE_10'])
treasury_2(address) := phi(['treasury_12', 'treasury_10', 'treasury_7', 'treasury_4', 'treasury_0', 'treasury_1'])
 oldTreasury = treasury
oldTreasury_1(address) := treasury_3(address)
 treasury = treasury_
treasury_4(address) := treasury__1(address)
 TreasuryUpdated(oldTreasury,treasury_)
Emit TreasuryUpdated(oldTreasury_1,treasury__1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_9)
```
#### BondingTax.withdraw(address) [EXTERNAL]
```slithir
ADMIN_ROLE_11(bytes32) := phi(['ADMIN_ROLE_12', 'ADMIN_ROLE_8', 'ADMIN_ROLE_6', 'ADMIN_ROLE_4', 'ADMIN_ROLE_0', 'ADMIN_ROLE_10'])
treasury_5(address) := phi(['treasury_12', 'treasury_10', 'treasury_7', 'treasury_4', 'treasury_0', 'treasury_1'])
 IERC20(token).safeTransfer(treasury,IERC20(token).balanceOf(address(this)))
TMP_11034 = CONVERT token_1 to IERC20
TMP_11035 = CONVERT token_1 to IERC20
TMP_11036 = CONVERT this to address
TMP_11037(uint256) = HIGH_LEVEL_CALL, dest:TMP_11035(IERC20), function:balanceOf, arguments:['TMP_11036']  
treasury_7(address) := phi(['treasury_12', 'treasury_10', 'treasury_6', 'treasury_7', 'treasury_4', 'treasury_1'])
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_11034', 'treasury_7', 'TMP_11037'] 
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_11)
```
#### SafeERC20.forceApprove(IERC20,address,uint256) [INTERNAL]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1'])
spender_1(address) := phi(['spender_1', 'spender_1'])
value_1(uint256) := phi(['TMP_5419', 'TMP_5425'])
 approvalCall = abi.encodeCall(token.approve,(spender,value))
REF_2050(approve) -> token_1.approve
TMP_5427(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2050,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78002500>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff780023e0>])
approvalCall_1(bytes) := TMP_5427(bytes)
 ! _callOptionalReturnBool(token,approvalCall)
TMP_5428(bool) = INTERNAL_CALL, SafeERC20._callOptionalReturnBool(IERC20,bytes)(token_1,approvalCall_1)
TMP_5429 = UnaryType.BANG TMP_5428 
CONDITION TMP_5429
 _callOptionalReturn(token,abi.encodeCall(token.approve,(spender,0)))
REF_2052(approve) -> token_1.approve
TMP_5430(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2052,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78002500>, 0])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5430)
 _callOptionalReturn(token,approvalCall)
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,approvalCall_1)
```
#### IERC20.balanceOf(address) [EXTERNAL]
```slithir

```
#### AeroAdaptor.getAmountsOut(uint256,address[]) [EXTERNAL]
```slithir
router_4(address) := phi(['router_1', 'router_3', 'router_5', 'router_0'])
tokenIn_3(address) := phi(['tokenIn_1', 'tokenIn_0'])
tokenOut_3(address) := phi(['tokenOut_1', 'tokenOut_0'])
factory_3(address) := phi(['factory_1', 'factory_0'])
 routes = new IAeroRouter.Route[](1)
TMP_10755(IAeroRouter.Route[])  = new IAeroRouter.Route[](1)
routes_1(IAeroRouter.Route[]) = ['TMP_10755(IAeroRouter.Route[])']
 routes[0] = IAeroRouter.Route(tokenIn,tokenOut,false,factory)
REF_4383(IAeroRouter.Route) -> routes_1[0]
TMP_10756(IAeroRouter.Route) = new Route(tokenIn_3,tokenOut_3,False,factory_3)
routes_2(IAeroRouter.Route[]) := phi(['routes_1'])
REF_4383(IAeroRouter.Route) (->routes_2) := TMP_10756(IAeroRouter.Route)
 IAeroRouter(router).getAmountsOut(amountIn,routes)
TMP_10757 = CONVERT router_4 to IAeroRouter
TMP_10758(uint256[]) = HIGH_LEVEL_CALL, dest:TMP_10757(IAeroRouter), function:getAmountsOut, arguments:['amountIn_1', 'routes_2']  
router_5(address) := phi(['router_1', 'router_3', 'router_5', 'router_4'])
RETURN TMP_10758
 amounts
```
#### IRouter.swapExactTokensForTokens(uint256,uint256,address[],address,uint256) [EXTERNAL]
```slithir

```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_2044(transfer) -> token_1.transfer
TMP_5413(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2044,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000550>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001210>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5413)
```
#### SafeERC20._callOptionalReturn(IERC20,bytes) [PRIVATE]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1', 'token_1'])
data_1(bytes) := phi(['approvalCall_1', 'TMP_5413', 'TMP_5415', 'TMP_5430'])
 returndata = address(token).functionCall(data)
TMP_5433 = CONVERT token_1 to address
TMP_5434(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionCall(address,bytes), arguments:['TMP_5433', 'data_1'] 
returndata_1(bytes) := TMP_5434(bytes)
 returndata.length != 0 && ! abi.decode(returndata,(bool))
REF_2054 -> LENGTH returndata_1
TMP_5435(bool) = REF_2054 != 0
TMP_5436(bool) = SOLIDITY_CALL abi.decode()(returndata_1,bool)
TMP_5437 = UnaryType.BANG TMP_5436 
TMP_5438(bool) = TMP_5435 && TMP_5437
CONDITION TMP_5438
 revert SafeERC20FailedOperation(address)(address(token))
TMP_5439 = CONVERT token_1 to address
TMP_5440(None) = SOLIDITY_CALL revert SafeERC20FailedOperation(address)(TMP_5439)
```

#### Address.functionCall(address,bytes) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0)
TMP_5457(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256)(target_1,data_1,0)
RETURN TMP_5457
```
