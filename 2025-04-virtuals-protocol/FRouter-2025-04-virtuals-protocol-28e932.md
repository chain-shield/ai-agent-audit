
### Storage layout (FPair) 

```text
router address
tokenA address
tokenB address
_pool FPair.Pool

```



### Storage layout (FRouter) 

```text
factory FFactory
assetToken address
taxManager address

```
### Storage layout (FFactory) 

```text
_pair mapping(address => mapping(address => address))
pairs address[]
router address
taxVault address
buyTax uint256
sellTax uint256

```
#### FRouter.addInitialLiquidity(address,uint256,uint256) [PUBLIC]
```slithir
EXECUTOR_ROLE_1(bytes32) := phi(['EXECUTOR_ROLE_0', 'EXECUTOR_ROLE_2', 'EXECUTOR_ROLE_7', 'EXECUTOR_ROLE_9', 'EXECUTOR_ROLE_11', 'EXECUTOR_ROLE_5'])
factory_4(FFactory) := phi(['factory_6', 'factory_23', 'factory_0', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_6(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_0', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
 require(bool,string)(token_ != address(0),Zero addresses are not allowed.)
TMP_8551 = CONVERT 0 to address
TMP_8552(bool) = token__1 != TMP_8551
TMP_8553(None) = SOLIDITY_CALL require(bool,string)(TMP_8552,Zero addresses are not allowed.)
 pairAddress = factory.getPair(token_,assetToken)
TMP_8554(address) = HIGH_LEVEL_CALL, dest:factory_5(FFactory), function:getPair, arguments:['token__1', 'assetToken_7']  
factory_6(FFactory) := phi(['factory_6', 'factory_23', 'factory_1', 'factory_3', 'factory_5', 'factory_19', 'factory_13'])
assetToken_8(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_1', 'assetToken_7', 'assetToken_12', 'assetToken_5'])
pairAddress_1(address) := TMP_8554(address)
 pair = IFPair(pairAddress)
TMP_8555 = CONVERT pairAddress_1 to IFPair
pair_1(IFPair) := TMP_8555(IFPair)
 token = IERC20(token_)
TMP_8556 = CONVERT token__1 to IERC20
token_1(IERC20) := TMP_8556(IERC20)
 token.safeTransferFrom(msg.sender,pairAddress,amountToken_)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['token_1', 'msg.sender', 'pairAddress_1', 'amountToken__1'] 
 pair.mint(amountToken_,amountAsset_)
TMP_8558(bool) = HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:mint, arguments:['amountToken__1', 'amountAsset__1']  
 (amountToken_,amountAsset_)
RETURN amountToken__1,amountAsset__1
 onlyRole(EXECUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(EXECUTOR_ROLE_1)
```
#### FRouter.approval(address,address,address,uint256) [PUBLIC]
```slithir
EXECUTOR_ROLE_10(bytes32) := phi(['EXECUTOR_ROLE_0', 'EXECUTOR_ROLE_2', 'EXECUTOR_ROLE_7', 'EXECUTOR_ROLE_9', 'EXECUTOR_ROLE_11', 'EXECUTOR_ROLE_5'])
 require(bool,string)(spender != address(0),Zero addresses are not allowed.)
TMP_8621 = CONVERT 0 to address
TMP_8622(bool) = spender_1 != TMP_8621
TMP_8623(None) = SOLIDITY_CALL require(bool,string)(TMP_8622,Zero addresses are not allowed.)
 IFPair(pair).approval(spender,asset,amount)
TMP_8624 = CONVERT pair_1 to IFPair
TMP_8625(bool) = HIGH_LEVEL_CALL, dest:TMP_8624(IFPair), function:approval, arguments:['spender_1', 'asset_1', 'amount_1']  
 onlyRole(EXECUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(EXECUTOR_ROLE_10)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### FRouter.buy(uint256,address,address) [PUBLIC]
```slithir
EXECUTOR_ROLE_6(bytes32) := phi(['EXECUTOR_ROLE_0', 'EXECUTOR_ROLE_2', 'EXECUTOR_ROLE_7', 'EXECUTOR_ROLE_9', 'EXECUTOR_ROLE_11', 'EXECUTOR_ROLE_5'])
factory_14(FFactory) := phi(['factory_6', 'factory_23', 'factory_0', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_13(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_0', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
taxManager_12(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_0', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
 require(bool,string)(tokenAddress != address(0),Zero addresses are not allowed.)
TMP_8584 = CONVERT 0 to address
TMP_8585(bool) = tokenAddress_1 != TMP_8584
TMP_8586(None) = SOLIDITY_CALL require(bool,string)(TMP_8585,Zero addresses are not allowed.)
 require(bool,string)(to != address(0),Zero addresses are not allowed.)
TMP_8587 = CONVERT 0 to address
TMP_8588(bool) = to_1 != TMP_8587
TMP_8589(None) = SOLIDITY_CALL require(bool,string)(TMP_8588,Zero addresses are not allowed.)
 require(bool,string)(amountIn > 0,amountIn must be greater than 0)
TMP_8590(bool) = amountIn_1 > 0
TMP_8591(None) = SOLIDITY_CALL require(bool,string)(TMP_8590,amountIn must be greater than 0)
 pair = factory.getPair(tokenAddress,assetToken)
TMP_8592(address) = HIGH_LEVEL_CALL, dest:factory_16(FFactory), function:getPair, arguments:['tokenAddress_1', 'assetToken_15']  
factory_17(FFactory) := phi(['factory_6', 'factory_23', 'factory_1', 'factory_3', 'factory_16', 'factory_19', 'factory_13'])
assetToken_16(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_1', 'assetToken_15', 'assetToken_12', 'assetToken_5'])
taxManager_15(address) := phi(['taxManager_22', 'taxManager_14', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
pair_1(address) := TMP_8592(address)
 fee = factory.buyTax()
TMP_8593(uint256) = HIGH_LEVEL_CALL, dest:factory_17(FFactory), function:buyTax, arguments:[]  
factory_18(FFactory) := phi(['factory_6', 'factory_17', 'factory_23', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_17(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_16', 'assetToken_23', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
taxManager_16(address) := phi(['taxManager_22', 'taxManager_15', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
fee_1(uint256) := TMP_8593(uint256)
 txFee = (fee * amountIn) / 100
TMP_8594(uint256) = fee_1 (c)* amountIn_1
TMP_8595(uint256) = TMP_8594 (c)/ 100
txFee_1(uint256) := TMP_8595(uint256)
 feeTo = factory.taxVault()
TMP_8596(address) = HIGH_LEVEL_CALL, dest:factory_18(FFactory), function:taxVault, arguments:[]  
factory_19(FFactory) := phi(['factory_6', 'factory_18', 'factory_23', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_18(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_17', 'assetToken_23', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
taxManager_17(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_16', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
feeTo_1(address) := TMP_8596(address)
 amount = amountIn - txFee
TMP_8597(uint256) = amountIn_1 (c)- txFee_1
amount_1(uint256) := TMP_8597(uint256)
 IERC20(assetToken).safeTransferFrom(to,pair,amount)
TMP_8598 = CONVERT assetToken_18 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_8598', 'to_1', 'pair_1', 'amount_1'] 
 IERC20(assetToken).safeTransferFrom(to,feeTo,txFee)
TMP_8600 = CONVERT assetToken_18 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_8600', 'to_1', 'feeTo_1', 'txFee_1'] 
 amountOut = getAmountsOut(tokenAddress,assetToken,amount)
TMP_8602(uint256) = INTERNAL_CALL, FRouter.getAmountsOut(address,address,uint256)(tokenAddress_1,assetToken_18,amount_1)
assetToken_19(address) := phi(['assetToken_5'])
amountOut_1(uint256) := TMP_8602(uint256)
 IFPair(pair).transferTo(to,amountOut)
TMP_8603 = CONVERT pair_1 to IFPair
HIGH_LEVEL_CALL, dest:TMP_8603(IFPair), function:transferTo, arguments:['to_1', 'amountOut_1']  
taxManager_19(address) := phi(['taxManager_22', 'taxManager_18', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
 IFPair(pair).swap(0,amountOut,amount,0)
TMP_8605 = CONVERT pair_1 to IFPair
TMP_8606(bool) = HIGH_LEVEL_CALL, dest:TMP_8605(IFPair), function:swap, arguments:['0', 'amountOut_1', 'amount_1', '0']  
taxManager_20(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21', 'taxManager_19'])
 feeTo == taxManager
TMP_8607(bool) = feeTo_1 == taxManager_20
CONDITION TMP_8607
 IBondingTax(taxManager).swapForAsset()
TMP_8608 = CONVERT taxManager_20 to IBondingTax
TUPLE_93(bool,uint256) = HIGH_LEVEL_CALL, dest:TMP_8608(IBondingTax), function:swapForAsset, arguments:[]  
taxManager_21(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
 (amount,amountOut)
RETURN amount_1,amountOut_1
 onlyRole(EXECUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(EXECUTOR_ROLE_6)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### FRouter.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### FRouter.getAmountsOut(address,address,uint256) [PUBLIC]
```slithir
token_1(address) := phi(['tokenAddress_1', 'tokenAddress_1'])
assetToken__1(address) := phi(['assetToken_18', 'TMP_8569'])
amountIn_1(uint256) := phi(['amountIn_1', 'amount_1'])
factory_2(FFactory) := phi(['factory_6', 'factory_23', 'factory_0', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_2(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_0', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
 require(bool,string)(token != address(0),Zero addresses are not allowed.)
TMP_8538 = CONVERT 0 to address
TMP_8539(bool) = token_1 != TMP_8538
TMP_8540(None) = SOLIDITY_CALL require(bool,string)(TMP_8539,Zero addresses are not allowed.)
 pairAddress = factory.getPair(token,assetToken)
TMP_8541(address) = HIGH_LEVEL_CALL, dest:factory_2(FFactory), function:getPair, arguments:['token_1', 'assetToken_2']  
factory_3(FFactory) := phi(['factory_6', 'factory_23', 'factory_2', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_3(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_2', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
pairAddress_1(address) := TMP_8541(address)
 pair = IFPair(pairAddress)
TMP_8542 = CONVERT pairAddress_1 to IFPair
pair_1(IFPair) := TMP_8542(IFPair)
 (reserveA,reserveB) = pair.getReserves()
TUPLE_91(uint256,uint256) = HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:getReserves, arguments:[]  
assetToken_4(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_1', 'assetToken_3', 'assetToken_12', 'assetToken_5'])
reserveA_1(uint256)= UNPACK TUPLE_91 index: 0 
reserveB_1(uint256)= UNPACK TUPLE_91 index: 1 
 k = pair.kLast()
TMP_8543(uint256) = HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:kLast, arguments:[]  
assetToken_5(address) := phi(['assetToken_19', 'assetToken_4', 'assetToken_8', 'assetToken_23', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
k_1(uint256) := TMP_8543(uint256)
 assetToken_ == assetToken
TMP_8544(bool) = assetToken__1 == assetToken_5
CONDITION TMP_8544
 newReserveB = reserveB + amountIn
TMP_8545(uint256) = reserveB_1 (c)+ amountIn_1
newReserveB_1(uint256) := TMP_8545(uint256)
 newReserveA = k / newReserveB
TMP_8546(uint256) = k_1 (c)/ newReserveB_1
newReserveA_1(uint256) := TMP_8546(uint256)
 amountOut = reserveA - newReserveA
TMP_8547(uint256) = reserveA_1 (c)- newReserveA_1
amountOut_1(uint256) := TMP_8547(uint256)
 newReserveA_scope_0 = reserveA + amountIn
TMP_8548(uint256) = reserveA_1 (c)+ amountIn_1
newReserveA_scope_0_1(uint256) := TMP_8548(uint256)
 newReserveB_scope_1 = k / newReserveA_scope_0
TMP_8549(uint256) = k_1 (c)/ newReserveA_scope_0_1
newReserveB_scope_1_1(uint256) := TMP_8549(uint256)
 amountOut = reserveB - newReserveB_scope_1
TMP_8550(uint256) = reserveB_1 (c)- newReserveB_scope_1_1
amountOut_2(uint256) := TMP_8550(uint256)
amountOut_3(uint256) := phi(['amountOut_1', 'amountOut_2'])
 amountOut
RETURN amountOut_3
 _amountOut
```
#### FRouter.graduate(address) [PUBLIC]
```slithir
EXECUTOR_ROLE_8(bytes32) := phi(['EXECUTOR_ROLE_0', 'EXECUTOR_ROLE_2', 'EXECUTOR_ROLE_7', 'EXECUTOR_ROLE_9', 'EXECUTOR_ROLE_11', 'EXECUTOR_ROLE_5'])
factory_20(FFactory) := phi(['factory_6', 'factory_23', 'factory_0', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_20(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_0', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
 require(bool,string)(tokenAddress != address(0),Zero addresses are not allowed.)
TMP_8611 = CONVERT 0 to address
TMP_8612(bool) = tokenAddress_1 != TMP_8611
TMP_8613(None) = SOLIDITY_CALL require(bool,string)(TMP_8612,Zero addresses are not allowed.)
 pair = factory.getPair(tokenAddress,assetToken)
TMP_8614(address) = HIGH_LEVEL_CALL, dest:factory_22(FFactory), function:getPair, arguments:['tokenAddress_1', 'assetToken_22']  
factory_23(FFactory) := phi(['factory_6', 'factory_23', 'factory_1', 'factory_3', 'factory_22', 'factory_19', 'factory_13'])
assetToken_23(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_1', 'assetToken_22', 'assetToken_12', 'assetToken_5'])
pair_1(address) := TMP_8614(address)
 assetBalance = IFPair(pair).assetBalance()
TMP_8615 = CONVERT pair_1 to IFPair
TMP_8616(uint256) = HIGH_LEVEL_CALL, dest:TMP_8615(IFPair), function:assetBalance, arguments:[]  
assetBalance_1(uint256) := TMP_8616(uint256)
 FPair(pair).transferAsset(msg.sender,assetBalance)
TMP_8617 = CONVERT pair_1 to FPair
HIGH_LEVEL_CALL, dest:TMP_8617(FPair), function:transferAsset, arguments:['msg.sender', 'assetBalance_1']  
 onlyRole(EXECUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(EXECUTOR_ROLE_8)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### FRouter.initialize(address,address) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_5'])
 __ReentrancyGuard_init()
INTERNAL_CALL, ReentrancyGuardUpgradeable.__ReentrancyGuard_init()()
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 _grantRole(DEFAULT_ADMIN_ROLE,msg.sender)
TMP_8529(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_4,msg.sender)
 require(bool,string)(factory_ != address(0),Zero addresses are not allowed.)
TMP_8530 = CONVERT 0 to address
TMP_8531(bool) = factory__1 != TMP_8530
TMP_8532(None) = SOLIDITY_CALL require(bool,string)(TMP_8531,Zero addresses are not allowed.)
 require(bool,string)(assetToken_ != address(0),Zero addresses are not allowed.)
TMP_8533 = CONVERT 0 to address
TMP_8534(bool) = assetToken__1 != TMP_8533
TMP_8535(None) = SOLIDITY_CALL require(bool,string)(TMP_8534,Zero addresses are not allowed.)
 factory = FFactory(factory_)
TMP_8536 = CONVERT factory__1 to FFactory
factory_1(FFactory) := TMP_8536(FFactory)
 assetToken = assetToken_
assetToken_1(address) := assetToken__1(address)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### FRouter.sell(uint256,address,address) [PUBLIC]
```slithir
EXECUTOR_ROLE_3(bytes32) := phi(['EXECUTOR_ROLE_0', 'EXECUTOR_ROLE_2', 'EXECUTOR_ROLE_7', 'EXECUTOR_ROLE_9', 'EXECUTOR_ROLE_11', 'EXECUTOR_ROLE_5'])
factory_7(FFactory) := phi(['factory_6', 'factory_23', 'factory_0', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_9(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_0', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
taxManager_1(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_0', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
 require(bool,string)(tokenAddress != address(0),Zero addresses are not allowed.)
TMP_8560 = CONVERT 0 to address
TMP_8561(bool) = tokenAddress_1 != TMP_8560
TMP_8562(None) = SOLIDITY_CALL require(bool,string)(TMP_8561,Zero addresses are not allowed.)
 require(bool,string)(to != address(0),Zero addresses are not allowed.)
TMP_8563 = CONVERT 0 to address
TMP_8564(bool) = to_1 != TMP_8563
TMP_8565(None) = SOLIDITY_CALL require(bool,string)(TMP_8564,Zero addresses are not allowed.)
 pairAddress = factory.getPair(tokenAddress,assetToken)
TMP_8566(address) = HIGH_LEVEL_CALL, dest:factory_9(FFactory), function:getPair, arguments:['tokenAddress_1', 'assetToken_11']  
factory_10(FFactory) := phi(['factory_6', 'factory_23', 'factory_9', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_12(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_11', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
taxManager_4(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_3', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
pairAddress_1(address) := TMP_8566(address)
 pair = IFPair(pairAddress)
TMP_8567 = CONVERT pairAddress_1 to IFPair
pair_1(IFPair) := TMP_8567(IFPair)
 token = IERC20(tokenAddress)
TMP_8568 = CONVERT tokenAddress_1 to IERC20
token_1(IERC20) := TMP_8568(IERC20)
 amountOut = getAmountsOut(tokenAddress,address(0),amountIn)
TMP_8569 = CONVERT 0 to address
TMP_8570(uint256) = INTERNAL_CALL, FRouter.getAmountsOut(address,address,uint256)(tokenAddress_1,TMP_8569,amountIn_1)
factory_11(FFactory) := phi(['factory_3'])
amountOut_1(uint256) := TMP_8570(uint256)
 token.safeTransferFrom(to,pairAddress,amountIn)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['token_1', 'to_1', 'pairAddress_1', 'amountIn_1'] 
 fee = factory.sellTax()
TMP_8572(uint256) = HIGH_LEVEL_CALL, dest:factory_11(FFactory), function:sellTax, arguments:[]  
factory_12(FFactory) := phi(['factory_6', 'factory_23', 'factory_1', 'factory_3', 'factory_19', 'factory_13', 'factory_11'])
taxManager_6(address) := phi(['taxManager_22', 'taxManager_5', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
fee_1(uint256) := TMP_8572(uint256)
 txFee = (fee * amountOut) / 100
TMP_8573(uint256) = fee_1 (c)* amountOut_1
TMP_8574(uint256) = TMP_8573 (c)/ 100
txFee_1(uint256) := TMP_8574(uint256)
 amount = amountOut - txFee
TMP_8575(uint256) = amountOut_1 (c)- txFee_1
amount_1(uint256) := TMP_8575(uint256)
 feeTo = factory.taxVault()
TMP_8576(address) = HIGH_LEVEL_CALL, dest:factory_12(FFactory), function:taxVault, arguments:[]  
factory_13(FFactory) := phi(['factory_6', 'factory_23', 'factory_1', 'factory_3', 'factory_19', 'factory_13', 'factory_12'])
taxManager_7(address) := phi(['taxManager_22', 'taxManager_6', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
feeTo_1(address) := TMP_8576(address)
 pair.transferAsset(to,amount)
HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:transferAsset, arguments:['to_1', 'amount_1']  
taxManager_8(address) := phi(['taxManager_22', 'taxManager_7', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
 pair.transferAsset(feeTo,txFee)
HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:transferAsset, arguments:['feeTo_1', 'txFee_1']  
taxManager_9(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_20', 'taxManager_8', 'taxManager_11', 'taxManager_21'])
 pair.swap(amountIn,0,0,amountOut)
TMP_8579(bool) = HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:swap, arguments:['amountIn_1', '0', '0', 'amountOut_1']  
taxManager_10(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21', 'taxManager_9'])
 feeTo == taxManager
TMP_8580(bool) = feeTo_1 == taxManager_10
CONDITION TMP_8580
 IBondingTax(taxManager).swapForAsset()
TMP_8581 = CONVERT taxManager_10 to IBondingTax
TUPLE_92(bool,uint256) = HIGH_LEVEL_CALL, dest:TMP_8581(IBondingTax), function:swapForAsset, arguments:[]  
taxManager_11(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
 (amountIn,amountOut)
RETURN amountIn_1,amountOut_1
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
 onlyRole(EXECUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(EXECUTOR_ROLE_4)
```
#### FRouter.setTaxManager(address) [PUBLIC]
```slithir
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_2', 'ADMIN_ROLE_0'])
 taxManager = newManager
taxManager_22(address) := newManager_1(address)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_1)
```

#### SafeERC20.safeTransferFrom(IERC20,address,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transferFrom,(from,to,value)))
REF_2046(transferFrom) -> token_1.transferFrom
TMP_5415(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2046,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000160>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001150>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001c90>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5415)
```
#### FFactory.getPair(address,address) [PUBLIC]
```slithir
_pair_3(mapping(address => mapping(address => address))) := phi(['_pair_0', '_pair_3', '_pair_2'])
 _pair[tokenA][tokenB]
REF_3379(mapping(address => address)) -> _pair_3[tokenA_1]
REF_3380(address) -> REF_3379[tokenB_1]
RETURN REF_3380
```
#### IFPair.mint(uint256,uint256) [EXTERNAL]
```slithir

```

#### IFPair.swap(uint256,uint256,uint256,uint256) [EXTERNAL]
```slithir

```
#### IFPair.transferTo(address,uint256) [EXTERNAL]
```slithir

```

#### IFPair.getReserves() [EXTERNAL]
```slithir

```
#### IFPair.kLast() [EXTERNAL]
```slithir

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
#### IFPair.assetBalance() [EXTERNAL]
```slithir

```
#### IFPair.transferAsset(address,uint256) [EXTERNAL]
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
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_2044(transfer) -> token_1.transfer
TMP_5413(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2044,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000550>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001210>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5413)
```
#### Address.functionCall(address,bytes) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0)
TMP_5457(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256)(target_1,data_1,0)
RETURN TMP_5457
```
