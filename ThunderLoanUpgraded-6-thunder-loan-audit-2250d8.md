
### Storage layout (AssetToken) 

```text
s_exchangeRate uint256

```

### Storage layout (ThunderLoanUpgraded) 

```text
s_tokenToAssetToken mapping(IERC20 => AssetToken)
s_flashLoanFee uint256
s_currentlyFlashLoaning mapping(IERC20 => bool)

```


#### ThunderLoanUpgraded._authorizeUpgrade(address) [INTERNAL]
```slithir
newImplementation_1(address) := phi(['newImplementation_1'])
 onlyOwner()
MODIFIER_CALL, OwnableUpgradeable.onlyOwner()()
```
#### ThunderLoanUpgraded.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### ThunderLoanUpgraded.initialize(address) [EXTERNAL]
```slithir
 __Ownable_init(msg.sender)
INTERNAL_CALL, OwnableUpgradeable.__Ownable_init(address)(msg.sender)
 __UUPSUpgradeable_init()
INTERNAL_CALL, UUPSUpgradeable.__UUPSUpgradeable_init()()
 __Oracle_init(tswapAddress)
INTERNAL_CALL, OracleUpgradeable.__Oracle_init(address)(tswapAddress_1)
 s_flashLoanFee = 3e15
s_flashLoanFee_1(uint256) := 3000000000000000(uint256)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### ThunderLoanUpgraded.deposit(IERC20,uint256) [EXTERNAL]
```slithir
s_tokenToAssetToken_1(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_9', 's_tokenToAssetToken_0', 's_tokenToAssetToken_15', 's_tokenToAssetToken_10', 's_tokenToAssetToken_13', 's_tokenToAssetToken_14', 's_tokenToAssetToken_3', 's_tokenToAssetToken_6', 's_tokenToAssetToken_16'])
 assetToken = s_tokenToAssetToken[token]
REF_230(AssetToken) -> s_tokenToAssetToken_3[token_1]
assetToken_1(AssetToken) := REF_230(AssetToken)
 exchangeRate = assetToken.getExchangeRate()
TMP_737(uint256) = HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:getExchangeRate, arguments:[]  
exchangeRate_1(uint256) := TMP_737(uint256)
 mintAmount = (amount * assetToken.EXCHANGE_RATE_PRECISION()) / exchangeRate
TMP_738(uint256) = HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:EXCHANGE_RATE_PRECISION, arguments:[]  
TMP_739(uint256) = amount_1 (c)* TMP_738
TMP_740(uint256) = TMP_739 (c)/ exchangeRate_1
mintAmount_1(uint256) := TMP_740(uint256)
 Deposit(msg.sender,token,amount)
Emit Deposit(msg.sender,token_1,amount_1)
 assetToken.mint(msg.sender,mintAmount)
HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:mint, arguments:['msg.sender', 'mintAmount_1']  
 calculatedFee = getCalculatedFee(token,amount)
TMP_743(uint256) = INTERNAL_CALL, ThunderLoanUpgraded.getCalculatedFee(IERC20,uint256)(token_1,amount_1)
calculatedFee_1(uint256) := TMP_743(uint256)
 assetToken.updateExchangeRate(calculatedFee)
HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:updateExchangeRate, arguments:['calculatedFee_1']  
 token.safeTransferFrom(msg.sender,address(assetToken),amount)
TMP_745 = CONVERT assetToken_1 to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['token_1', 'msg.sender', 'TMP_745', 'amount_1'] 
 revertIfZero(amount)
MODIFIER_CALL, ThunderLoanUpgraded.revertIfZero(uint256)(amount_1)
 revertIfNotAllowedToken(token)
MODIFIER_CALL, ThunderLoanUpgraded.revertIfNotAllowedToken(IERC20)(token_1)
```
#### ThunderLoanUpgraded.redeem(IERC20,uint256) [EXTERNAL]
```slithir
s_tokenToAssetToken_4(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_9', 's_tokenToAssetToken_0', 's_tokenToAssetToken_15', 's_tokenToAssetToken_10', 's_tokenToAssetToken_13', 's_tokenToAssetToken_14', 's_tokenToAssetToken_3', 's_tokenToAssetToken_6', 's_tokenToAssetToken_16'])
 assetToken = s_tokenToAssetToken[token]
REF_236(AssetToken) -> s_tokenToAssetToken_6[token_1]
assetToken_1(AssetToken) := REF_236(AssetToken)
 exchangeRate = assetToken.getExchangeRate()
TMP_749(uint256) = HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:getExchangeRate, arguments:[]  
exchangeRate_1(uint256) := TMP_749(uint256)
 amountOfAssetToken == type()(uint256).max
TMP_751(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_752(bool) = amountOfAssetToken_1 == TMP_751
CONDITION TMP_752
 amountOfAssetToken = assetToken.balanceOf(msg.sender)
TMP_753(uint256) = HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:balanceOf, arguments:['msg.sender']  
amountOfAssetToken_2(uint256) := TMP_753(uint256)
amountOfAssetToken_3(uint256) := phi(['amountOfAssetToken_1', 'amountOfAssetToken_2'])
 amountUnderlying = (amountOfAssetToken * exchangeRate) / assetToken.EXCHANGE_RATE_PRECISION()
TMP_754(uint256) = amountOfAssetToken_3 (c)* exchangeRate_1
TMP_755(uint256) = HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:EXCHANGE_RATE_PRECISION, arguments:[]  
TMP_756(uint256) = TMP_754 (c)/ TMP_755
amountUnderlying_1(uint256) := TMP_756(uint256)
 Redeemed(msg.sender,token,amountOfAssetToken,amountUnderlying)
Emit Redeemed(msg.sender,token_1,amountOfAssetToken_3,amountUnderlying_1)
 assetToken.burn(msg.sender,amountOfAssetToken)
HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:burn, arguments:['msg.sender', 'amountOfAssetToken_3']  
 assetToken.transferUnderlyingTo(msg.sender,amountUnderlying)
HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:transferUnderlyingTo, arguments:['msg.sender', 'amountUnderlying_1']  
 revertIfZero(amountOfAssetToken)
MODIFIER_CALL, ThunderLoanUpgraded.revertIfZero(uint256)(amountOfAssetToken_1)
 revertIfNotAllowedToken(token)
MODIFIER_CALL, ThunderLoanUpgraded.revertIfNotAllowedToken(IERC20)(token_1)
```
#### ThunderLoanUpgraded.flashloan(address,IERC20,uint256,bytes) [EXTERNAL]
```slithir
s_tokenToAssetToken_7(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_9', 's_tokenToAssetToken_0', 's_tokenToAssetToken_15', 's_tokenToAssetToken_10', 's_tokenToAssetToken_13', 's_tokenToAssetToken_14', 's_tokenToAssetToken_3', 's_tokenToAssetToken_6', 's_tokenToAssetToken_16'])
 assetToken = s_tokenToAssetToken[token]
REF_242(AssetToken) -> s_tokenToAssetToken_9[token_1]
assetToken_1(AssetToken) := REF_242(AssetToken)
 startingBalance = IERC20(token).balanceOf(address(assetToken))
TMP_762 = CONVERT token_1 to IERC20
TMP_763 = CONVERT assetToken_1 to address
TMP_764(uint256) = HIGH_LEVEL_CALL, dest:TMP_762(IERC20), function:balanceOf, arguments:['TMP_763']  
startingBalance_1(uint256) := TMP_764(uint256)
 amount > startingBalance
TMP_765(bool) = amount_1 > startingBalance_1
CONDITION TMP_765
 revert ThunderLoan__NotEnoughTokenBalance(uint256,uint256)(startingBalance,amount)
TMP_766(None) = SOLIDITY_CALL revert ThunderLoan__NotEnoughTokenBalance(uint256,uint256)(startingBalance_1,amount_1)
 receiverAddress.code.length == 0
TMP_767(bytes) = SOLIDITY_CALL code(address)(receiverAddress_1)
REF_244 -> LENGTH TMP_767
TMP_768(bool) = REF_244 == 0
CONDITION TMP_768
 revert ThunderLoan__CallerIsNotContract()()
TMP_769(None) = SOLIDITY_CALL revert ThunderLoan__CallerIsNotContract()()
 fee = getCalculatedFee(token,amount)
TMP_770(uint256) = INTERNAL_CALL, ThunderLoanUpgraded.getCalculatedFee(IERC20,uint256)(token_1,amount_1)
fee_1(uint256) := TMP_770(uint256)
 assetToken.updateExchangeRate(fee)
HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:updateExchangeRate, arguments:['fee_1']  
 FlashLoan(receiverAddress,token,amount,fee,params)
Emit FlashLoan(receiverAddress_1,token_1,amount_1,fee_1,params_1)
 s_currentlyFlashLoaning[token] = true
REF_246(bool) -> s_currentlyFlashLoaning_0[token_1]
s_currentlyFlashLoaning_1(mapping(IERC20 => bool)) := phi(['s_currentlyFlashLoaning_0'])
REF_246(bool) (->s_currentlyFlashLoaning_1) := True(bool)
 assetToken.transferUnderlyingTo(receiverAddress,amount)
HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:transferUnderlyingTo, arguments:['receiverAddress_1', 'amount_1']  
 receiverAddress.functionCall(abi.encodeCall(IFlashLoanReceiver.executeOperation,(address(token),amount,fee,msg.sender,params)))
REF_250(executeOperation) -> IFlashLoanReceiver.executeOperation
TMP_774 = CONVERT token_1 to address
TMP_775(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_250,[<slither.slithir.variables.temporary_ssa.TemporaryVariableSSA object at 0xffffb1a506d0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffb1a16e60>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffb1a503a0>, <slither.core.declarations.solidity_variables.SolidityVariableComposed object at 0xffffb1ff4280>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffb1a16fe0>])
TMP_776(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionCall(address,bytes), arguments:['receiverAddress_1', 'TMP_775'] 
 endingBalance = token.balanceOf(address(assetToken))
TMP_777 = CONVERT assetToken_1 to address
TMP_778(uint256) = HIGH_LEVEL_CALL, dest:token_1(IERC20), function:balanceOf, arguments:['TMP_777']  
endingBalance_1(uint256) := TMP_778(uint256)
 endingBalance < startingBalance + fee
TMP_779(uint256) = startingBalance_1 (c)+ fee_1
TMP_780(bool) = endingBalance_1 < TMP_779
CONDITION TMP_780
 revert ThunderLoan__NotPaidBack(uint256,uint256)(startingBalance + fee,endingBalance)
TMP_781(uint256) = startingBalance_1 (c)+ fee_1
TMP_782(None) = SOLIDITY_CALL revert ThunderLoan__NotPaidBack(uint256,uint256)(TMP_781,endingBalance_1)
 s_currentlyFlashLoaning[token] = false
REF_252(bool) -> s_currentlyFlashLoaning_1[token_1]
s_currentlyFlashLoaning_2(mapping(IERC20 => bool)) := phi(['s_currentlyFlashLoaning_1'])
REF_252(bool) (->s_currentlyFlashLoaning_2) := False(bool)
 revertIfZero(amount)
MODIFIER_CALL, ThunderLoanUpgraded.revertIfZero(uint256)(amount_1)
 revertIfNotAllowedToken(token)
MODIFIER_CALL, ThunderLoanUpgraded.revertIfNotAllowedToken(IERC20)(token_1)
```
#### ThunderLoanUpgraded.repay(IERC20,uint256) [PUBLIC]
```slithir
s_tokenToAssetToken_10(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_9', 's_tokenToAssetToken_0', 's_tokenToAssetToken_15', 's_tokenToAssetToken_10', 's_tokenToAssetToken_13', 's_tokenToAssetToken_14', 's_tokenToAssetToken_3', 's_tokenToAssetToken_6', 's_tokenToAssetToken_16'])
s_currentlyFlashLoaning_3(mapping(IERC20 => bool)) := phi(['s_currentlyFlashLoaning_2', 's_currentlyFlashLoaning_4', 's_currentlyFlashLoaning_3', 's_currentlyFlashLoaning_0'])
 ! s_currentlyFlashLoaning[token]
REF_253(bool) -> s_currentlyFlashLoaning_3[token_1]
TMP_785 = UnaryType.BANG REF_253 
CONDITION TMP_785
 revert ThunderLoan__NotCurrentlyFlashLoaning()()
TMP_786(None) = SOLIDITY_CALL revert ThunderLoan__NotCurrentlyFlashLoaning()()
 assetToken = s_tokenToAssetToken[token]
REF_254(AssetToken) -> s_tokenToAssetToken_10[token_1]
assetToken_1(AssetToken) := REF_254(AssetToken)
 token.safeTransferFrom(msg.sender,address(assetToken),amount)
TMP_787 = CONVERT assetToken_1 to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['token_1', 'msg.sender', 'TMP_787', 'amount_1']
```
#### ThunderLoanUpgraded.setAllowedToken(IERC20,bool) [EXTERNAL][OWNER]
```slithir
s_tokenToAssetToken_11(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_9', 's_tokenToAssetToken_0', 's_tokenToAssetToken_15', 's_tokenToAssetToken_10', 's_tokenToAssetToken_13', 's_tokenToAssetToken_14', 's_tokenToAssetToken_3', 's_tokenToAssetToken_6', 's_tokenToAssetToken_16'])
 allowed
CONDITION allowed_1
 address(s_tokenToAssetToken[token]) != address(0)
REF_256(AssetToken) -> s_tokenToAssetToken_12[token_1]
TMP_789 = CONVERT REF_256 to address
TMP_790 = CONVERT 0 to address
TMP_791(bool) = TMP_789 != TMP_790
CONDITION TMP_791
 revert ThunderLoan__AlreadyAllowed()()
TMP_792(None) = SOLIDITY_CALL revert ThunderLoan__AlreadyAllowed()()
 name = string.concat(ThunderLoan ,IERC20Metadata(address(token)).name())
TMP_793 = CONVERT token_1 to address
TMP_794 = CONVERT TMP_793 to IERC20Metadata
TMP_795(string) = HIGH_LEVEL_CALL, dest:TMP_794(IERC20Metadata), function:name, arguments:[]  
TMP_796(string) = SOLIDITY_CALL string.concat()(ThunderLoan ,TMP_795)
name_1(string) := TMP_796(string)
 symbol = string.concat(tl,IERC20Metadata(address(token)).symbol())
TMP_797 = CONVERT token_1 to address
TMP_798 = CONVERT TMP_797 to IERC20Metadata
TMP_799(string) = HIGH_LEVEL_CALL, dest:TMP_798(IERC20Metadata), function:symbol, arguments:[]  
TMP_800(string) = SOLIDITY_CALL string.concat()(tl,TMP_799)
symbol_1(string) := TMP_800(string)
 assetToken = new AssetToken(address(this),token,name,symbol)
TMP_802 = CONVERT this to address
TMP_803(AssetToken) = new AssetToken(TMP_802,token_1,name_1,symbol_1) 
assetToken_1(AssetToken) := TMP_803(AssetToken)
 s_tokenToAssetToken[token] = assetToken
REF_261(AssetToken) -> s_tokenToAssetToken_12[token_1]
s_tokenToAssetToken_14(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_12'])
REF_261(AssetToken) (->s_tokenToAssetToken_14) := assetToken_1(AssetToken)
 AllowedTokenSet(token,assetToken,allowed)
Emit AllowedTokenSet(token_1,assetToken_1,allowed_1)
 assetToken
RETURN assetToken_1
 assetToken_scope_0 = s_tokenToAssetToken[token]
REF_262(AssetToken) -> s_tokenToAssetToken_12[token_1]
assetToken_scope_0_1(AssetToken) := REF_262(AssetToken)
 delete s_tokenToAssetToken[token]
REF_263(AssetToken) -> s_tokenToAssetToken_12[token_1]
s_tokenToAssetToken_13 = delete REF_263 
 AllowedTokenSet(token,assetToken_scope_0,allowed)
Emit AllowedTokenSet(token_1,assetToken_scope_0_1,allowed_1)
 assetToken_scope_0
RETURN assetToken_scope_0_1
 onlyOwner()
MODIFIER_CALL, OwnableUpgradeable.onlyOwner()()
```
#### ThunderLoanUpgraded.getCalculatedFee(IERC20,uint256) [PUBLIC]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1'])
amount_1(uint256) := phi(['amount_1', 'amount_1'])
s_flashLoanFee_2(uint256) := phi(['s_flashLoanFee_4', 's_flashLoanFee_0', 's_flashLoanFee_3', 's_flashLoanFee_1'])
FEE_PRECISION_1(uint256) := phi(['FEE_PRECISION_4', 'FEE_PRECISION_0', 'FEE_PRECISION_2'])
 valueOfBorrowedToken = (amount * getPriceInWeth(address(token))) / FEE_PRECISION
TMP_807 = CONVERT token_1 to address
TMP_808(uint256) = INTERNAL_CALL, OracleUpgradeable.getPriceInWeth(address)(TMP_807)
TMP_809(uint256) = amount_1 (c)* TMP_808
TMP_810(uint256) = TMP_809 (c)/ FEE_PRECISION_2
valueOfBorrowedToken_1(uint256) := TMP_810(uint256)
 fee = (valueOfBorrowedToken * s_flashLoanFee) / FEE_PRECISION
TMP_811(uint256) = valueOfBorrowedToken_1 (c)* s_flashLoanFee_3
TMP_812(uint256) = TMP_811 (c)/ FEE_PRECISION_2
fee_1(uint256) := TMP_812(uint256)
 fee
RETURN fee_1
```
#### ThunderLoanUpgraded.updateFlashLoanFee(uint256) [EXTERNAL][OWNER]
```slithir
FEE_PRECISION_3(uint256) := phi(['FEE_PRECISION_4', 'FEE_PRECISION_0', 'FEE_PRECISION_2'])
 newFee > FEE_PRECISION
TMP_813(bool) = newFee_1 > FEE_PRECISION_4
CONDITION TMP_813
 revert ThunderLoan__BadNewFee()()
TMP_814(None) = SOLIDITY_CALL revert ThunderLoan__BadNewFee()()
 s_flashLoanFee = newFee
s_flashLoanFee_4(uint256) := newFee_1(uint256)
 onlyOwner()
MODIFIER_CALL, OwnableUpgradeable.onlyOwner()()
```
#### ThunderLoanUpgraded.isAllowedToken(IERC20) [PUBLIC]
```slithir
token_1(IERC20) := phi(['token_1'])
s_tokenToAssetToken_15(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_9', 's_tokenToAssetToken_0', 's_tokenToAssetToken_15', 's_tokenToAssetToken_10', 's_tokenToAssetToken_13', 's_tokenToAssetToken_14', 's_tokenToAssetToken_3', 's_tokenToAssetToken_6', 's_tokenToAssetToken_16'])
 address(s_tokenToAssetToken[token]) != address(0)
REF_264(AssetToken) -> s_tokenToAssetToken_15[token_1]
TMP_816 = CONVERT REF_264 to address
TMP_817 = CONVERT 0 to address
TMP_818(bool) = TMP_816 != TMP_817
RETURN TMP_818
```
#### ThunderLoanUpgraded.getAssetFromToken(IERC20) [PUBLIC]
```slithir
s_tokenToAssetToken_16(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_9', 's_tokenToAssetToken_0', 's_tokenToAssetToken_15', 's_tokenToAssetToken_10', 's_tokenToAssetToken_13', 's_tokenToAssetToken_14', 's_tokenToAssetToken_3', 's_tokenToAssetToken_6', 's_tokenToAssetToken_16'])
 s_tokenToAssetToken[token]
REF_265(AssetToken) -> s_tokenToAssetToken_16[token_1]
RETURN REF_265
```
#### ThunderLoanUpgraded.isCurrentlyFlashLoaning(IERC20) [PUBLIC]
```slithir
s_currentlyFlashLoaning_4(mapping(IERC20 => bool)) := phi(['s_currentlyFlashLoaning_2', 's_currentlyFlashLoaning_4', 's_currentlyFlashLoaning_3', 's_currentlyFlashLoaning_0'])
 s_currentlyFlashLoaning[token]
REF_266(bool) -> s_currentlyFlashLoaning_4[token_1]
RETURN REF_266
```
#### ThunderLoanUpgraded.getFee() [EXTERNAL]
```slithir
s_flashLoanFee_5(uint256) := phi(['s_flashLoanFee_4', 's_flashLoanFee_0', 's_flashLoanFee_3', 's_flashLoanFee_1'])
 s_flashLoanFee
RETURN s_flashLoanFee_5
```
#### ThunderLoanUpgraded.slitherConstructorConstantVariables() [INTERNAL]
```slithir
Expression: UPGRADE_INTERFACE_VERSION = 5.0.0
Expression: FEE_PRECISION = 1e18
Expression: $ = _getInitializableStorage()
IRs:
TMP_820(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_820'])(Initializable.InitializableStorage) := TMP_820(Initializable.InitializableStorage)
Expression: isTopLevelCall = ! $._initializing
IRs:
REF_267(bool) -> $_1 (-> ['TMP_820'])._initializing
TMP_821 = UnaryType.BANG REF_267 
isTopLevelCall_1(bool) := TMP_821(bool)
Expression: initialized = $._initialized
IRs:
REF_268(uint64) -> $_1 (-> ['TMP_820'])._initialized
initialized_1(uint64) := REF_268(uint64)
Expression: initialSetup = initialized == 0 && isTopLevelCall
IRs:
TMP_822(bool) = initialized_1 == 0
TMP_823(bool) = TMP_822 && isTopLevelCall_1
initialSetup_1(bool) := TMP_823(bool)
Expression: construction = initialized == 1 && address(this).code.length == 0
IRs:
TMP_824(bool) = initialized_1 == 1
TMP_825 = CONVERT this to address
TMP_826(bytes) = SOLIDITY_CALL code(address)(TMP_825)
REF_269 -> LENGTH TMP_826
TMP_827(bool) = REF_269 == 0
TMP_828(bool) = TMP_824 && TMP_827
construction_1(bool) := TMP_828(bool)
Expression: ! initialSetup && ! construction
IRs:
TMP_829 = UnaryType.BANG initialSetup_1 
TMP_830 = UnaryType.BANG construction_1 
TMP_831(bool) = TMP_829 && TMP_830
CONDITION TMP_831
Expression: revert InvalidInitialization()()
IRs:
TMP_832(None) = SOLIDITY_CALL revert InvalidInitialization()()
Expression: $._initialized = 1
IRs:
REF_270(uint64) -> $_1 (-> ['TMP_820'])._initialized
$_2 (-> ['TMP_820'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_820'])"])
REF_270(uint64) (->$_2 (-> ['TMP_820'])) := 1(uint256)
TMP_820(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_820'])"])
Expression: isTopLevelCall
IRs:
CONDITION isTopLevelCall_1
Expression: $._initializing = true
IRs:
REF_271(bool) -> $_2 (-> ['TMP_820'])._initializing
$_3 (-> ['TMP_820'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_820'])"])
REF_271(bool) (->$_3 (-> ['TMP_820'])) := True(bool)
TMP_820(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_820'])"])
IRs:
$_4 (-> ['TMP_820'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_820'])", "$_2 (-> ['TMP_820'])"])
Expression: isTopLevelCall
IRs:
CONDITION isTopLevelCall_1
Expression: $._initializing = false
IRs:
REF_272(bool) -> $_4 (-> ['TMP_820'])._initializing
$_5 (-> ['TMP_820'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_820'])"])
REF_272(bool) (->$_5 (-> ['TMP_820'])) := False(bool)
TMP_820(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_820'])"])
Expression: Initialized(1)
IRs:
Emit Initialized(1)
Expression: $ = _getInitializableStorage()
IRs:
TMP_834(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_834'])(Initializable.InitializableStorage) := TMP_834(Initializable.InitializableStorage)
Expression: $._initializing || $._initialized >= version
IRs:
REF_273(bool) -> $_1 (-> ['TMP_834'])._initializing
REF_274(uint64) -> $_1 (-> ['TMP_834'])._initialized
TMP_835(bool) = REF_274 >= version_1
TMP_836(bool) = REF_273 || TMP_835
CONDITION TMP_836
Expression: revert InvalidInitialization()()
IRs:
TMP_837(None) = SOLIDITY_CALL revert InvalidInitialization()()
Expression: $._initialized = version
IRs:
REF_275(uint64) -> $_1 (-> ['TMP_834'])._initialized
$_2 (-> ['TMP_834'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_834'])"])
REF_275(uint64) (->$_2 (-> ['TMP_834'])) := version_1(uint64)
TMP_834(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_834'])"])
Expression: $._initializing = true
IRs:
REF_276(bool) -> $_2 (-> ['TMP_834'])._initializing
$_3 (-> ['TMP_834'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_834'])"])
REF_276(bool) (->$_3 (-> ['TMP_834'])) := True(bool)
TMP_834(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_834'])"])
Expression: $._initializing = false
IRs:
REF_277(bool) -> $_3 (-> ['TMP_834'])._initializing
$_4 (-> ['TMP_834'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_834'])"])
REF_277(bool) (->$_4 (-> ['TMP_834'])) := False(bool)
TMP_834(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_834'])"])
Expression: Initialized(version)
IRs:
Emit Initialized(version_1)
Expression: _checkInitializing()
IRs:
INTERNAL_CALL, Initializable._checkInitializing()()
Expression: _checkProxy()
IRs:
INTERNAL_CALL, UUPSUpgradeable._checkProxy()()
Expression: _checkNotDelegated()
IRs:
INTERNAL_CALL, UUPSUpgradeable._checkNotDelegated()()
Expression: _checkOwner()
IRs:
INTERNAL_CALL, OwnableUpgradeable._checkOwner()()
IRs:
amount_1(uint256) := phi(['amount_1', 'amount_1', 'amountOfAssetToken_1'])
Expression: amount == 0
IRs:
TMP_843(bool) = amount_1 == 0
CONDITION TMP_843
Expression: revert ThunderLoan__CantBeZero()()
IRs:
TMP_844(None) = SOLIDITY_CALL revert ThunderLoan__CantBeZero()()
IRs:
token_1(IERC20) := phi(['token_1', 'token_1', 'token_1'])
Expression: ! isAllowedToken(token)
IRs:
TMP_845(bool) = INTERNAL_CALL, ThunderLoanUpgraded.isAllowedToken(IERC20)(token_1)
TMP_846 = UnaryType.BANG TMP_845 
CONDITION TMP_846
Expression: revert ThunderLoan__NotAllowedToken(address)(token)
IRs:
TMP_847(None) = SOLIDITY_CALL revert ThunderLoan__NotAllowedToken(address)(token_1)
```
#### SafeERC20.safeTransferFrom(IERC20,address,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transferFrom,(from,to,value)))
REF_32(transferFrom) -> token_1.transferFrom
TMP_94(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_32,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffb1ddd420>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffb1dde950>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffb1ddf490>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_94)
```
#### AssetToken.mint(address,uint256) [EXTERNAL]
```slithir
 _mint(to,amount)
INTERNAL_CALL, ERC20._mint(address,uint256)(to_1,amount_1)
 onlyThunderLoan()
MODIFIER_CALL, AssetToken.onlyThunderLoan()()
```
#### AssetToken.getExchangeRate() [EXTERNAL]
```slithir
s_exchangeRate_7(uint256) := phi(['s_exchangeRate_6', 's_exchangeRate_1', 's_exchangeRate_0'])
 s_exchangeRate
RETURN s_exchangeRate_7
```
#### AssetToken.updateExchangeRate(uint256) [EXTERNAL]
```slithir
s_exchangeRate_2(uint256) := phi(['s_exchangeRate_6', 's_exchangeRate_1', 's_exchangeRate_0'])
 newExchangeRate = s_exchangeRate * (totalSupply() + fee) / totalSupply()
TMP_417(uint256) = INTERNAL_CALL, ERC20.totalSupply()()
TMP_418(uint256) = TMP_417 (c)+ fee_1
TMP_419(uint256) = s_exchangeRate_4 (c)* TMP_418
TMP_420(uint256) = INTERNAL_CALL, ERC20.totalSupply()()
TMP_421(uint256) = TMP_419 (c)/ TMP_420
newExchangeRate_1(uint256) := TMP_421(uint256)
 newExchangeRate <= s_exchangeRate
TMP_422(bool) = newExchangeRate_1 <= s_exchangeRate_5
CONDITION TMP_422
 revert AssetToken__ExhangeRateCanOnlyIncrease(uint256,uint256)(s_exchangeRate,newExchangeRate)
TMP_423(None) = SOLIDITY_CALL revert AssetToken__ExhangeRateCanOnlyIncrease(uint256,uint256)(s_exchangeRate_5,newExchangeRate_1)
 s_exchangeRate = newExchangeRate
s_exchangeRate_6(uint256) := newExchangeRate_1(uint256)
 ExchangeRateUpdated(s_exchangeRate)
Emit ExchangeRateUpdated(s_exchangeRate_6)
 onlyThunderLoan()
MODIFIER_CALL, AssetToken.onlyThunderLoan()()
```
#### AssetToken.burn(address,uint256) [EXTERNAL]
```slithir
 _burn(account,amount)
INTERNAL_CALL, ERC20._burn(address,uint256)(account_1,amount_1)
 onlyThunderLoan()
MODIFIER_CALL, AssetToken.onlyThunderLoan()()
```
#### AssetToken.transferUnderlyingTo(address,uint256) [EXTERNAL]
```slithir
i_underlying_2(IERC20) := phi(['i_underlying_3', 'i_underlying_1', 'i_underlying_0'])
 i_underlying.safeTransfer(to,amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['i_underlying_3', 'to_1', 'amount_1'] 
 onlyThunderLoan()
MODIFIER_CALL, AssetToken.onlyThunderLoan()()
```
#### Address.functionCall(address,bytes) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0)
TMP_136(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256)(target_1,data_1,0)
RETURN TMP_136
```
#### IERC20.balanceOf(address) [EXTERNAL]
```slithir

```
#### IERC20Metadata.name() [EXTERNAL]
```slithir

```
#### IERC20Metadata.symbol() [EXTERNAL]
```slithir

```
#### SafeERC20._callOptionalReturn(IERC20,bytes) [PRIVATE]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1', 'token_1'])
data_1(bytes) := phi(['TMP_94', 'TMP_92', 'approvalCall_1', 'TMP_109'])
 returndata = address(token).functionCall(data)
TMP_112 = CONVERT token_1 to address
TMP_113(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionCall(address,bytes), arguments:['TMP_112', 'data_1'] 
returndata_1(bytes) := TMP_113(bytes)
 returndata.length != 0 && ! abi.decode(returndata,(bool))
REF_40 -> LENGTH returndata_1
TMP_114(bool) = REF_40 != 0
TMP_115(bool) = SOLIDITY_CALL abi.decode()(returndata_1,bool)
TMP_116 = UnaryType.BANG TMP_115 
TMP_117(bool) = TMP_114 && TMP_116
CONDITION TMP_117
 revert SafeERC20FailedOperation(address)(address(token))
TMP_118 = CONVERT token_1 to address
TMP_119(None) = SOLIDITY_CALL revert SafeERC20FailedOperation(address)(TMP_118)
```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_30(transfer) -> token_1.transfer
TMP_92(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_30,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffb1dddd50>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffb1ddea10>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_92)
```
#### Address.functionCallWithValue(address,bytes,uint256) [INTERNAL]
```slithir
target_1(address) := phi(['target_1'])
data_1(bytes) := phi(['data_1'])
 address(this).balance < value
TMP_137 = CONVERT this to address
TMP_138(uint256) = SOLIDITY_CALL balance(address)(TMP_137)
TMP_139(bool) = TMP_138 < value_1
CONDITION TMP_139
 revert AddressInsufficientBalance(address)(address(this))
TMP_140 = CONVERT this to address
TMP_141(None) = SOLIDITY_CALL revert AddressInsufficientBalance(address)(TMP_140)
 (success,returndata) = target.call{value: value}(data)
TUPLE_2(bool,bytes) = LOW_LEVEL_CALL, dest:target_1, function:call, arguments:['data_1'] value:value_1 
success_1(bool)= UNPACK TUPLE_2 index: 0 
returndata_1(bytes)= UNPACK TUPLE_2 index: 1 
 verifyCallResultFromTarget(target,success,returndata)
TMP_142(bytes) = INTERNAL_CALL, Address.verifyCallResultFromTarget(address,bool,bytes)(target_1,success_1,returndata_1)
RETURN TMP_142
```
#### Address.verifyCallResultFromTarget(address,bool,bytes) [INTERNAL]
```slithir
target_1(address) := phi(['target_1', 'target_1', 'target_1'])
success_1(bool) := phi(['success_1', 'success_1', 'success_1'])
returndata_1(bytes) := phi(['returndata_1', 'returndata_1', 'returndata_1'])
 ! success
TMP_145 = UnaryType.BANG success_1 
CONDITION TMP_145
 _revert(returndata)
INTERNAL_CALL, Address._revert(bytes)(returndata_1)
 returndata.length == 0 && target.code.length == 0
REF_50 -> LENGTH returndata_1
TMP_147(bool) = REF_50 == 0
TMP_148(bytes) = SOLIDITY_CALL code(address)(target_1)
REF_51 -> LENGTH TMP_148
TMP_149(bool) = REF_51 == 0
TMP_150(bool) = TMP_147 && TMP_149
CONDITION TMP_150
 revert AddressEmptyCode(address)(target)
TMP_151(None) = SOLIDITY_CALL revert AddressEmptyCode(address)(target_1)
 returndata
RETURN returndata_1
```
