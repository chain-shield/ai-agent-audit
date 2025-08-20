### Storage layout (AeroAdaptor) 

```text
router address
tokenIn address
tokenOut address
factory address

```



#### AeroAdaptor.constructor(address,address,address,address) [PUBLIC]
```slithir
 router = router_
router_1(address) := router__1(address)
 tokenIn = tokenIn_
tokenIn_1(address) := tokenIn__1(address)
 tokenOut = tokenOut_
tokenOut_1(address) := tokenOut__1(address)
 factory = factory_
factory_1(address) := factory__1(address)
 IERC20(tokenIn).forceApprove(router_,type()(uint256).max)
TMP_10742 = CONVERT tokenIn_1 to IERC20
TMP_10744(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.forceApprove(IERC20,address,uint256), arguments:['TMP_10742', 'router__1', 'TMP_10744']
```
#### IRouter.getAmountsOut(uint256,address[]) [EXTERNAL]
```slithir

```
#### AeroAdaptor.swapExactTokensForTokens(uint256,uint256,address[],address,uint256) [EXTERNAL]
```slithir
router_2(address) := phi(['router_1', 'router_3', 'router_5', 'router_0'])
tokenIn_2(address) := phi(['tokenIn_1', 'tokenIn_0'])
tokenOut_2(address) := phi(['tokenOut_1', 'tokenOut_0'])
factory_2(address) := phi(['factory_1', 'factory_0'])
 routes = new IAeroRouter.Route[](1)
TMP_10747(IAeroRouter.Route[])  = new IAeroRouter.Route[](1)
routes_1(IAeroRouter.Route[]) = ['TMP_10747(IAeroRouter.Route[])']
 routes[0] = IAeroRouter.Route(tokenIn,tokenOut,false,factory)
REF_4379(IAeroRouter.Route) -> routes_1[0]
TMP_10748(IAeroRouter.Route) = new Route(tokenIn_2,tokenOut_2,False,factory_2)
routes_2(IAeroRouter.Route[]) := phi(['routes_1'])
REF_4379(IAeroRouter.Route) (->routes_2) := TMP_10748(IAeroRouter.Route)
 IERC20(tokenIn).safeTransferFrom(msg.sender,address(this),amountIn)
TMP_10749 = CONVERT tokenIn_2 to IERC20
TMP_10750 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_10749', 'msg.sender', 'TMP_10750', 'amountIn_1'] 
 amounts = IAeroRouter(router).swapExactTokensForTokens(amountIn,amountOutMin,routes,to,deadline)
TMP_10752 = CONVERT router_2 to IAeroRouter
TMP_10753(uint256[]) = HIGH_LEVEL_CALL, dest:TMP_10752(IAeroRouter), function:swapExactTokensForTokens, arguments:['amountIn_1', 'amountOutMin_1', 'routes_2', 'to_1', 'deadline_1']  
router_3(address) := phi(['router_1', 'router_3', 'router_5', 'router_2'])
amounts_1(uint256[]) = ['TMP_10753(uint256[])']
 amounts
RETURN amounts_1
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

#### SafeERC20.safeTransferFrom(IERC20,address,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transferFrom,(from,to,value)))
REF_2046(transferFrom) -> token_1.transferFrom
TMP_5415(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2046,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000160>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001150>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001c90>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5415)
```
#### IAeroRouter.swapExactTokensForTokens(uint256,uint256,IAeroRouter.Route[],address,uint256) [EXTERNAL]
```slithir

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
