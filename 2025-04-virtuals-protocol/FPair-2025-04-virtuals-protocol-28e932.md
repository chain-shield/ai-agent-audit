


### Storage layout (FPair) 

```text
router address
tokenA address
tokenB address
_pool FPair.Pool

```
#### FPair.approval(address,address,uint256) [PUBLIC]
```slithir
 require(bool,string)(_user != address(0),Zero addresses are not allowed.)
TMP_8423 = CONVERT 0 to address
TMP_8424(bool) = _user_1 != TMP_8423
TMP_8425(None) = SOLIDITY_CALL require(bool,string)(TMP_8424,Zero addresses are not allowed.)
 require(bool,string)(_token != address(0),Zero addresses are not allowed.)
TMP_8426 = CONVERT 0 to address
TMP_8427(bool) = _token_1 != TMP_8426
TMP_8428(None) = SOLIDITY_CALL require(bool,string)(TMP_8427,Zero addresses are not allowed.)
 token = IERC20(_token)
TMP_8429 = CONVERT _token_1 to IERC20
token_1(IERC20) := TMP_8429(IERC20)
 token.forceApprove(_user,amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.forceApprove(IERC20,address,uint256), arguments:['token_1', '_user_1', 'amount_1'] 
 true
RETURN True
 onlyRouter()
MODIFIER_CALL, FPair.onlyRouter()()
```
#### IFPair.assetBalance() [PUBLIC]
```slithir

```
#### FPair.balance() [PUBLIC]
```slithir
tokenA_4(address) := phi(['tokenA_5', 'tokenA_3', 'tokenA_0', 'tokenA_1'])
 IERC20(tokenA).balanceOf(address(this))
TMP_8446 = CONVERT tokenA_4 to IERC20
TMP_8447 = CONVERT this to address
TMP_8448(uint256) = HIGH_LEVEL_CALL, dest:TMP_8446(IERC20), function:balanceOf, arguments:['TMP_8447']  
tokenA_5(address) := phi(['tokenA_4', 'tokenA_5', 'tokenA_3', 'tokenA_1'])
RETURN TMP_8448
```
#### FPair.constructor(address,address,address) [PUBLIC]
```slithir
 require(bool,string)(router_ != address(0),Zero addresses are not allowed.)
TMP_8401 = CONVERT 0 to address
TMP_8402(bool) = router__1 != TMP_8401
TMP_8403(None) = SOLIDITY_CALL require(bool,string)(TMP_8402,Zero addresses are not allowed.)
 require(bool,string)(token0 != address(0),Zero addresses are not allowed.)
TMP_8404 = CONVERT 0 to address
TMP_8405(bool) = token0_1 != TMP_8404
TMP_8406(None) = SOLIDITY_CALL require(bool,string)(TMP_8405,Zero addresses are not allowed.)
 require(bool,string)(token1 != address(0),Zero addresses are not allowed.)
TMP_8407 = CONVERT 0 to address
TMP_8408(bool) = token1_1 != TMP_8407
TMP_8409(None) = SOLIDITY_CALL require(bool,string)(TMP_8408,Zero addresses are not allowed.)
 router = router_
router_1(address) := router__1(address)
 tokenA = token0
tokenA_1(address) := token0_1(address)
 tokenB = token1
tokenB_1(address) := token1_1(address)
```
#### FPair.getReserves() [PUBLIC]
```slithir
_pool_7(FPair.Pool) := phi(['_pool_3', '_pool_9', '_pool_6', '_pool_0', '_pool_8', '_pool_7', '_pool_10'])
 (_pool.reserve0,_pool.reserve1)
REF_3400(uint256) -> _pool_7.reserve0
REF_3401(uint256) -> _pool_7.reserve1
RETURN REF_3400,REF_3401
```
#### FPair.kLast() [PUBLIC]
```slithir
_pool_8(FPair.Pool) := phi(['_pool_3', '_pool_9', '_pool_6', '_pool_0', '_pool_8', '_pool_7', '_pool_10'])
 _pool.k
REF_3402(uint256) -> _pool_8.k
RETURN REF_3402
```
#### FPair.mint(uint256,uint256) [PUBLIC]
```slithir
_pool_1(FPair.Pool) := phi(['_pool_3', '_pool_9', '_pool_6', '_pool_0', '_pool_8', '_pool_7', '_pool_10'])
 require(bool,string)(_pool.lastUpdated == 0,Already minted)
REF_3393(uint256) -> _pool_2.lastUpdated
TMP_8410(bool) = REF_3393 == 0
TMP_8411(None) = SOLIDITY_CALL require(bool,string)(TMP_8410,Already minted)
 _pool = Pool({reserve0:reserve0,reserve1:reserve1,k:reserve0 * reserve1,lastUpdated:block.timestamp})
TMP_8412(uint256) = reserve0_1 (c)* reserve1_1
TMP_8413(FPair.Pool) = new Pool(reserve0_1,reserve1_1,TMP_8412,block.timestamp)
_pool_3(FPair.Pool) := TMP_8413(FPair.Pool)
 Mint(reserve0,reserve1)
Emit Mint(reserve0_1,reserve1_1)
 true
RETURN True
 onlyRouter()
MODIFIER_CALL, FPair.onlyRouter()()
```
#### FPair.priceALast() [PUBLIC]
```slithir
_pool_9(FPair.Pool) := phi(['_pool_3', '_pool_9', '_pool_6', '_pool_0', '_pool_8', '_pool_7', '_pool_10'])
 _pool.reserve1 / _pool.reserve0
REF_3403(uint256) -> _pool_9.reserve1
REF_3404(uint256) -> _pool_9.reserve0
TMP_8444(uint256) = REF_3403 (c)/ REF_3404
RETURN TMP_8444
```
#### FPair.priceBLast() [PUBLIC]
```slithir
_pool_10(FPair.Pool) := phi(['_pool_3', '_pool_9', '_pool_6', '_pool_0', '_pool_8', '_pool_7', '_pool_10'])
 _pool.reserve0 / _pool.reserve1
REF_3405(uint256) -> _pool_10.reserve0
REF_3406(uint256) -> _pool_10.reserve1
TMP_8445(uint256) = REF_3405 (c)/ REF_3406
RETURN TMP_8445
```
#### FPair.swap(uint256,uint256,uint256,uint256) [PUBLIC]
```slithir
_pool_4(FPair.Pool) := phi(['_pool_3', '_pool_9', '_pool_6', '_pool_0', '_pool_8', '_pool_7', '_pool_10'])
 _reserve0 = (_pool.reserve0 + amount0In) - amount0Out
REF_3394(uint256) -> _pool_5.reserve0
TMP_8416(uint256) = REF_3394 (c)+ amount0In_1
TMP_8417(uint256) = TMP_8416 (c)- amount0Out_1
_reserve0_1(uint256) := TMP_8417(uint256)
 _reserve1 = (_pool.reserve1 + amount1In) - amount1Out
REF_3395(uint256) -> _pool_5.reserve1
TMP_8418(uint256) = REF_3395 (c)+ amount1In_1
TMP_8419(uint256) = TMP_8418 (c)- amount1Out_1
_reserve1_1(uint256) := TMP_8419(uint256)
 _pool = Pool({reserve0:_reserve0,reserve1:_reserve1,k:_pool.k,lastUpdated:block.timestamp})
REF_3396(uint256) -> _pool_5.k
TMP_8420(FPair.Pool) = new Pool(_reserve0_1,_reserve1_1,REF_3396,block.timestamp)
_pool_6(FPair.Pool) := TMP_8420(FPair.Pool)
 Swap(amount0In,amount0Out,amount1In,amount1Out)
Emit Swap(amount0In_1,amount0Out_1,amount1In_1,amount1Out_1)
 true
RETURN True
 onlyRouter()
MODIFIER_CALL, FPair.onlyRouter()()
```
#### FPair.transferAsset(address,uint256) [PUBLIC]
```slithir
tokenB_2(address) := phi(['tokenB_3', 'tokenB_5', 'tokenB_1', 'tokenB_0'])
 require(bool,string)(recipient != address(0),Zero addresses are not allowed.)
TMP_8432 = CONVERT 0 to address
TMP_8433(bool) = recipient_1 != TMP_8432
TMP_8434(None) = SOLIDITY_CALL require(bool,string)(TMP_8433,Zero addresses are not allowed.)
 IERC20(tokenB).safeTransfer(recipient,amount)
TMP_8435 = CONVERT tokenB_3 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_8435', 'recipient_1', 'amount_1'] 
 onlyRouter()
MODIFIER_CALL, FPair.onlyRouter()()
```
#### FPair.transferTo(address,uint256) [PUBLIC]
```slithir
tokenA_2(address) := phi(['tokenA_5', 'tokenA_3', 'tokenA_0', 'tokenA_1'])
 require(bool,string)(recipient != address(0),Zero addresses are not allowed.)
TMP_8438 = CONVERT 0 to address
TMP_8439(bool) = recipient_1 != TMP_8438
TMP_8440(None) = SOLIDITY_CALL require(bool,string)(TMP_8439,Zero addresses are not allowed.)
 IERC20(tokenA).safeTransfer(recipient,amount)
TMP_8441 = CONVERT tokenA_3 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_8441', 'recipient_1', 'amount_1'] 
 onlyRouter()
MODIFIER_CALL, FPair.onlyRouter()()
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
