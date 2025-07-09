

### Storage layout (ThunderLoan) 

```text
s_tokenToAssetToken mapping(IERC20 => AssetToken)
s_feePrecision uint256
s_flashLoanFee uint256
s_currentlyFlashLoaning mapping(IERC20 => bool)

```


### Storage layout (AssetToken) 

```text
s_exchangeRate uint256

```
#### ThunderLoan._authorizeUpgrade(address) [INTERNAL]
```slithir
newImplementation_1(address) := phi(['newImplementation_1'])
 onlyOwner()
MODIFIER_CALL, OwnableUpgradeable.onlyOwner()()
```
#### ThunderLoan.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### ThunderLoan.initialize(address) [EXTERNAL]
```slithir
 __Ownable_init(msg.sender)
INTERNAL_CALL, OwnableUpgradeable.__Ownable_init(address)(msg.sender)
 __UUPSUpgradeable_init()
INTERNAL_CALL, UUPSUpgradeable.__UUPSUpgradeable_init()()
 __Oracle_init(tswapAddress)
INTERNAL_CALL, OracleUpgradeable.__Oracle_init(address)(tswapAddress_1)
 s_feePrecision = 1e18
s_feePrecision_1(uint256) := 1000000000000000000(uint256)
 s_flashLoanFee = 3e15
s_flashLoanFee_1(uint256) := 3000000000000000(uint256)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### ThunderLoan.deposit(IERC20,uint256) [EXTERNAL]
```slithir
s_tokenToAssetToken_1(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_3', 's_tokenToAssetToken_6', 's_tokenToAssetToken_0', 's_tokenToAssetToken_16', 's_tokenToAssetToken_9', 's_tokenToAssetToken_13', 's_tokenToAssetToken_15', 's_tokenToAssetToken_10', 's_tokenToAssetToken_14'])
 assetToken = s_tokenToAssetToken[token]
REF_167(AssetToken) -> s_tokenToAssetToken_3[token_1]
assetToken_1(AssetToken) := REF_167(AssetToken)
 exchangeRate = assetToken.getExchangeRate()
TMP_550(uint256) = HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:getExchangeRate, arguments:[]  
exchangeRate_1(uint256) := TMP_550(uint256)
 mintAmount = (amount * assetToken.EXCHANGE_RATE_PRECISION()) / exchangeRate
TMP_551(uint256) = HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:EXCHANGE_RATE_PRECISION, arguments:[]  
TMP_552(uint256) = amount_1 (c)* TMP_551
TMP_553(uint256) = TMP_552 (c)/ exchangeRate_1
mintAmount_1(uint256) := TMP_553(uint256)
 Deposit(msg.sender,token,amount)
Emit Deposit(msg.sender,token_1,amount_1)
 assetToken.mint(msg.sender,mintAmount)
HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:mint, arguments:['msg.sender', 'mintAmount_1']  
 calculatedFee = getCalculatedFee(token,amount)
TMP_556(uint256) = INTERNAL_CALL, ThunderLoan.getCalculatedFee(IERC20,uint256)(token_1,amount_1)
calculatedFee_1(uint256) := TMP_556(uint256)
 assetToken.updateExchangeRate(calculatedFee)
HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:updateExchangeRate, arguments:['calculatedFee_1']  
 token.safeTransferFrom(msg.sender,address(assetToken),amount)
TMP_558 = CONVERT assetToken_1 to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['token_1', 'msg.sender', 'TMP_558', 'amount_1'] 
 revertIfZero(amount)
MODIFIER_CALL, ThunderLoan.revertIfZero(uint256)(amount_1)
 revertIfNotAllowedToken(token)
MODIFIER_CALL, ThunderLoan.revertIfNotAllowedToken(IERC20)(token_1)
```
#### ThunderLoan.redeem(IERC20,uint256) [EXTERNAL]
```slithir
s_tokenToAssetToken_4(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_3', 's_tokenToAssetToken_6', 's_tokenToAssetToken_0', 's_tokenToAssetToken_16', 's_tokenToAssetToken_9', 's_tokenToAssetToken_13', 's_tokenToAssetToken_15', 's_tokenToAssetToken_10', 's_tokenToAssetToken_14'])
 assetToken = s_tokenToAssetToken[token]
REF_173(AssetToken) -> s_tokenToAssetToken_6[token_1]
assetToken_1(AssetToken) := REF_173(AssetToken)
 exchangeRate = assetToken.getExchangeRate()
TMP_562(uint256) = HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:getExchangeRate, arguments:[]  
exchangeRate_1(uint256) := TMP_562(uint256)
 amountOfAssetToken == type()(uint256).max
TMP_564(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_565(bool) = amountOfAssetToken_1 == TMP_564
CONDITION TMP_565
 amountOfAssetToken = assetToken.balanceOf(msg.sender)
TMP_566(uint256) = HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:balanceOf, arguments:['msg.sender']  
amountOfAssetToken_2(uint256) := TMP_566(uint256)
amountOfAssetToken_3(uint256) := phi(['amountOfAssetToken_2', 'amountOfAssetToken_1'])
 amountUnderlying = (amountOfAssetToken * exchangeRate) / assetToken.EXCHANGE_RATE_PRECISION()
TMP_567(uint256) = amountOfAssetToken_3 (c)* exchangeRate_1
TMP_568(uint256) = HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:EXCHANGE_RATE_PRECISION, arguments:[]  
TMP_569(uint256) = TMP_567 (c)/ TMP_568
amountUnderlying_1(uint256) := TMP_569(uint256)
 Redeemed(msg.sender,token,amountOfAssetToken,amountUnderlying)
Emit Redeemed(msg.sender,token_1,amountOfAssetToken_3,amountUnderlying_1)
 assetToken.burn(msg.sender,amountOfAssetToken)
HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:burn, arguments:['msg.sender', 'amountOfAssetToken_3']  
 assetToken.transferUnderlyingTo(msg.sender,amountUnderlying)
HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:transferUnderlyingTo, arguments:['msg.sender', 'amountUnderlying_1']  
 revertIfZero(amountOfAssetToken)
MODIFIER_CALL, ThunderLoan.revertIfZero(uint256)(amountOfAssetToken_1)
 revertIfNotAllowedToken(token)
MODIFIER_CALL, ThunderLoan.revertIfNotAllowedToken(IERC20)(token_1)
```
#### ThunderLoan.flashloan(address,IERC20,uint256,bytes) [EXTERNAL]
```slithir
s_tokenToAssetToken_7(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_3', 's_tokenToAssetToken_6', 's_tokenToAssetToken_0', 's_tokenToAssetToken_16', 's_tokenToAssetToken_9', 's_tokenToAssetToken_13', 's_tokenToAssetToken_15', 's_tokenToAssetToken_10', 's_tokenToAssetToken_14'])
 assetToken = s_tokenToAssetToken[token]
REF_179(AssetToken) -> s_tokenToAssetToken_9[token_1]
assetToken_1(AssetToken) := REF_179(AssetToken)
 startingBalance = IERC20(token).balanceOf(address(assetToken))
TMP_575 = CONVERT token_1 to IERC20
TMP_576 = CONVERT assetToken_1 to address
TMP_577(uint256) = HIGH_LEVEL_CALL, dest:TMP_575(IERC20), function:balanceOf, arguments:['TMP_576']  
startingBalance_1(uint256) := TMP_577(uint256)
 amount > startingBalance
TMP_578(bool) = amount_1 > startingBalance_1
CONDITION TMP_578
 revert ThunderLoan__NotEnoughTokenBalance(uint256,uint256)(startingBalance,amount)
TMP_579(None) = SOLIDITY_CALL revert ThunderLoan__NotEnoughTokenBalance(uint256,uint256)(startingBalance_1,amount_1)
 receiverAddress.code.length == 0
TMP_580(bytes) = SOLIDITY_CALL code(address)(receiverAddress_1)
REF_181 -> LENGTH TMP_580
TMP_581(bool) = REF_181 == 0
CONDITION TMP_581
 revert ThunderLoan__CallerIsNotContract()()
TMP_582(None) = SOLIDITY_CALL revert ThunderLoan__CallerIsNotContract()()
 fee = getCalculatedFee(token,amount)
TMP_583(uint256) = INTERNAL_CALL, ThunderLoan.getCalculatedFee(IERC20,uint256)(token_1,amount_1)
fee_1(uint256) := TMP_583(uint256)
 assetToken.updateExchangeRate(fee)
HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:updateExchangeRate, arguments:['fee_1']  
 FlashLoan(receiverAddress,token,amount,fee,params)
Emit FlashLoan(receiverAddress_1,token_1,amount_1,fee_1,params_1)
 s_currentlyFlashLoaning[token] = true
REF_183(bool) -> s_currentlyFlashLoaning_0[token_1]
s_currentlyFlashLoaning_1(mapping(IERC20 => bool)) := phi(['s_currentlyFlashLoaning_0'])
REF_183(bool) (->s_currentlyFlashLoaning_1) := True(bool)
 assetToken.transferUnderlyingTo(receiverAddress,amount)
HIGH_LEVEL_CALL, dest:assetToken_1(AssetToken), function:transferUnderlyingTo, arguments:['receiverAddress_1', 'amount_1']  
 receiverAddress.functionCall(abi.encodeCall(IFlashLoanReceiver.executeOperation,(address(token),amount,fee,msg.sender,params)))
REF_187(executeOperation) -> IFlashLoanReceiver.executeOperation
TMP_587 = CONVERT token_1 to address
TMP_588(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_187,[<slither.slithir.variables.temporary_ssa.TemporaryVariableSSA object at 0xffffa4777be0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffa47763b0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffa47778b0>, <slither.core.declarations.solidity_variables.SolidityVariableComposed object at 0xffffa4e428c0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffa4776530>])
TMP_589(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionCall(address,bytes), arguments:['receiverAddress_1', 'TMP_588'] 
 endingBalance = token.balanceOf(address(assetToken))
TMP_590 = CONVERT assetToken_1 to address
TMP_591(uint256) = HIGH_LEVEL_CALL, dest:token_1(IERC20), function:balanceOf, arguments:['TMP_590']  
endingBalance_1(uint256) := TMP_591(uint256)
 endingBalance < startingBalance + fee
TMP_592(uint256) = startingBalance_1 (c)+ fee_1
TMP_593(bool) = endingBalance_1 < TMP_592
CONDITION TMP_593
 revert ThunderLoan__NotPaidBack(uint256,uint256)(startingBalance + fee,endingBalance)
TMP_594(uint256) = startingBalance_1 (c)+ fee_1
TMP_595(None) = SOLIDITY_CALL revert ThunderLoan__NotPaidBack(uint256,uint256)(TMP_594,endingBalance_1)
 s_currentlyFlashLoaning[token] = false
REF_189(bool) -> s_currentlyFlashLoaning_1[token_1]
s_currentlyFlashLoaning_2(mapping(IERC20 => bool)) := phi(['s_currentlyFlashLoaning_1'])
REF_189(bool) (->s_currentlyFlashLoaning_2) := False(bool)
 revertIfZero(amount)
MODIFIER_CALL, ThunderLoan.revertIfZero(uint256)(amount_1)
 revertIfNotAllowedToken(token)
MODIFIER_CALL, ThunderLoan.revertIfNotAllowedToken(IERC20)(token_1)
```
#### ThunderLoan.repay(IERC20,uint256) [PUBLIC]
```slithir
s_tokenToAssetToken_10(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_3', 's_tokenToAssetToken_6', 's_tokenToAssetToken_0', 's_tokenToAssetToken_16', 's_tokenToAssetToken_9', 's_tokenToAssetToken_13', 's_tokenToAssetToken_15', 's_tokenToAssetToken_10', 's_tokenToAssetToken_14'])
s_currentlyFlashLoaning_3(mapping(IERC20 => bool)) := phi(['s_currentlyFlashLoaning_4', 's_currentlyFlashLoaning_2', 's_currentlyFlashLoaning_3', 's_currentlyFlashLoaning_0'])
 ! s_currentlyFlashLoaning[token]
REF_190(bool) -> s_currentlyFlashLoaning_3[token_1]
TMP_598 = UnaryType.BANG REF_190 
CONDITION TMP_598
 revert ThunderLoan__NotCurrentlyFlashLoaning()()
TMP_599(None) = SOLIDITY_CALL revert ThunderLoan__NotCurrentlyFlashLoaning()()
 assetToken = s_tokenToAssetToken[token]
REF_191(AssetToken) -> s_tokenToAssetToken_10[token_1]
assetToken_1(AssetToken) := REF_191(AssetToken)
 token.safeTransferFrom(msg.sender,address(assetToken),amount)
TMP_600 = CONVERT assetToken_1 to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['token_1', 'msg.sender', 'TMP_600', 'amount_1']
```
#### ThunderLoan.setAllowedToken(IERC20,bool) [EXTERNAL][OWNER]
```slithir
s_tokenToAssetToken_11(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_3', 's_tokenToAssetToken_6', 's_tokenToAssetToken_0', 's_tokenToAssetToken_16', 's_tokenToAssetToken_9', 's_tokenToAssetToken_13', 's_tokenToAssetToken_15', 's_tokenToAssetToken_10', 's_tokenToAssetToken_14'])
 allowed
CONDITION allowed_1
 address(s_tokenToAssetToken[token]) != address(0)
REF_193(AssetToken) -> s_tokenToAssetToken_12[token_1]
TMP_602 = CONVERT REF_193 to address
TMP_603 = CONVERT 0 to address
TMP_604(bool) = TMP_602 != TMP_603
CONDITION TMP_604
 revert ThunderLoan__AlreadyAllowed()()
TMP_605(None) = SOLIDITY_CALL revert ThunderLoan__AlreadyAllowed()()
 name = string.concat(ThunderLoan ,IERC20Metadata(address(token)).name())
TMP_606 = CONVERT token_1 to address
TMP_607 = CONVERT TMP_606 to IERC20Metadata
TMP_608(string) = HIGH_LEVEL_CALL, dest:TMP_607(IERC20Metadata), function:name, arguments:[]  
TMP_609(string) = SOLIDITY_CALL string.concat()(ThunderLoan ,TMP_608)
name_1(string) := TMP_609(string)
 symbol = string.concat(tl,IERC20Metadata(address(token)).symbol())
TMP_610 = CONVERT token_1 to address
TMP_611 = CONVERT TMP_610 to IERC20Metadata
TMP_612(string) = HIGH_LEVEL_CALL, dest:TMP_611(IERC20Metadata), function:symbol, arguments:[]  
TMP_613(string) = SOLIDITY_CALL string.concat()(tl,TMP_612)
symbol_1(string) := TMP_613(string)
 assetToken = new AssetToken(address(this),token,name,symbol)
TMP_615 = CONVERT this to address
TMP_616(AssetToken) = new AssetToken(TMP_615,token_1,name_1,symbol_1) 
assetToken_1(AssetToken) := TMP_616(AssetToken)
 s_tokenToAssetToken[token] = assetToken
REF_198(AssetToken) -> s_tokenToAssetToken_12[token_1]
s_tokenToAssetToken_14(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_12'])
REF_198(AssetToken) (->s_tokenToAssetToken_14) := assetToken_1(AssetToken)
 AllowedTokenSet(token,assetToken,allowed)
Emit AllowedTokenSet(token_1,assetToken_1,allowed_1)
 assetToken
RETURN assetToken_1
 assetToken_scope_0 = s_tokenToAssetToken[token]
REF_199(AssetToken) -> s_tokenToAssetToken_12[token_1]
assetToken_scope_0_1(AssetToken) := REF_199(AssetToken)
 delete s_tokenToAssetToken[token]
REF_200(AssetToken) -> s_tokenToAssetToken_12[token_1]
s_tokenToAssetToken_13 = delete REF_200 
 AllowedTokenSet(token,assetToken_scope_0,allowed)
Emit AllowedTokenSet(token_1,assetToken_scope_0_1,allowed_1)
 assetToken_scope_0
RETURN assetToken_scope_0_1
 onlyOwner()
MODIFIER_CALL, OwnableUpgradeable.onlyOwner()()
```
#### ThunderLoan.getCalculatedFee(IERC20,uint256) [PUBLIC]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1'])
amount_1(uint256) := phi(['amount_1', 'amount_1'])
s_feePrecision_2(uint256) := phi(['s_feePrecision_3', 's_feePrecision_0', 's_feePrecision_1', 's_feePrecision_5'])
s_flashLoanFee_2(uint256) := phi(['s_flashLoanFee_3', 's_flashLoanFee_0', 's_flashLoanFee_4', 's_flashLoanFee_1'])
 valueOfBorrowedToken = (amount * getPriceInWeth(address(token))) / s_feePrecision
TMP_620 = CONVERT token_1 to address
TMP_621(uint256) = INTERNAL_CALL, OracleUpgradeable.getPriceInWeth(address)(TMP_620)
TMP_622(uint256) = amount_1 (c)* TMP_621
TMP_623(uint256) = TMP_622 (c)/ s_feePrecision_3
valueOfBorrowedToken_1(uint256) := TMP_623(uint256)
 fee = (valueOfBorrowedToken * s_flashLoanFee) / s_feePrecision
TMP_624(uint256) = valueOfBorrowedToken_1 (c)* s_flashLoanFee_3
TMP_625(uint256) = TMP_624 (c)/ s_feePrecision_3
fee_1(uint256) := TMP_625(uint256)
 fee
RETURN fee_1
```
#### ThunderLoan.updateFlashLoanFee(uint256) [EXTERNAL][OWNER]
```slithir
s_feePrecision_4(uint256) := phi(['s_feePrecision_3', 's_feePrecision_0', 's_feePrecision_1', 's_feePrecision_5'])
 newFee > s_feePrecision
TMP_626(bool) = newFee_1 > s_feePrecision_5
CONDITION TMP_626
 revert ThunderLoan__BadNewFee()()
TMP_627(None) = SOLIDITY_CALL revert ThunderLoan__BadNewFee()()
 s_flashLoanFee = newFee
s_flashLoanFee_4(uint256) := newFee_1(uint256)
 onlyOwner()
MODIFIER_CALL, OwnableUpgradeable.onlyOwner()()
```
#### ThunderLoan.isAllowedToken(IERC20) [PUBLIC]
```slithir
token_1(IERC20) := phi(['token_1'])
s_tokenToAssetToken_15(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_3', 's_tokenToAssetToken_6', 's_tokenToAssetToken_0', 's_tokenToAssetToken_16', 's_tokenToAssetToken_9', 's_tokenToAssetToken_13', 's_tokenToAssetToken_15', 's_tokenToAssetToken_10', 's_tokenToAssetToken_14'])
 address(s_tokenToAssetToken[token]) != address(0)
REF_201(AssetToken) -> s_tokenToAssetToken_15[token_1]
TMP_629 = CONVERT REF_201 to address
TMP_630 = CONVERT 0 to address
TMP_631(bool) = TMP_629 != TMP_630
RETURN TMP_631
```
#### ThunderLoan.getAssetFromToken(IERC20) [PUBLIC]
```slithir
s_tokenToAssetToken_16(mapping(IERC20 => AssetToken)) := phi(['s_tokenToAssetToken_3', 's_tokenToAssetToken_6', 's_tokenToAssetToken_0', 's_tokenToAssetToken_16', 's_tokenToAssetToken_9', 's_tokenToAssetToken_13', 's_tokenToAssetToken_15', 's_tokenToAssetToken_10', 's_tokenToAssetToken_14'])
 s_tokenToAssetToken[token]
REF_202(AssetToken) -> s_tokenToAssetToken_16[token_1]
RETURN REF_202
```
#### ThunderLoan.isCurrentlyFlashLoaning(IERC20) [PUBLIC]
```slithir
s_currentlyFlashLoaning_4(mapping(IERC20 => bool)) := phi(['s_currentlyFlashLoaning_4', 's_currentlyFlashLoaning_2', 's_currentlyFlashLoaning_3', 's_currentlyFlashLoaning_0'])
 s_currentlyFlashLoaning[token]
REF_203(bool) -> s_currentlyFlashLoaning_4[token_1]
RETURN REF_203
```
#### ThunderLoan.getFee() [EXTERNAL]
```slithir
s_flashLoanFee_5(uint256) := phi(['s_flashLoanFee_3', 's_flashLoanFee_0', 's_flashLoanFee_4', 's_flashLoanFee_1'])
 s_flashLoanFee
RETURN s_flashLoanFee_5
```
#### ThunderLoan.getFeePrecision() [EXTERNAL]
```slithir
s_feePrecision_6(uint256) := phi(['s_feePrecision_3', 's_feePrecision_0', 's_feePrecision_1', 's_feePrecision_5'])
 s_feePrecision
RETURN s_feePrecision_6
```

#### SafeERC20.safeTransferFrom(IERC20,address,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transferFrom,(from,to,value)))
REF_32(transferFrom) -> token_1.transferFrom
TMP_94(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_32,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffa4a3d420>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffa4a3e950>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffa4a3f490>])
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
#### AssetToken.transferUnderlyingTo(address,uint256) [EXTERNAL]
```slithir
i_underlying_2(IERC20) := phi(['i_underlying_3', 'i_underlying_1', 'i_underlying_0'])
 i_underlying.safeTransfer(to,amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['i_underlying_3', 'to_1', 'amount_1'] 
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
#### IERC20.balanceOf(address) [EXTERNAL]
```slithir

```
#### Address.functionCall(address,bytes) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0)
TMP_136(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256)(target_1,data_1,0)
RETURN TMP_136
```
#### IERC20Metadata.symbol() [EXTERNAL]
```slithir

```
#### IERC20Metadata.name() [EXTERNAL]
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
TMP_92(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_30,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffa4a3dd50>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffa4a3ea10>])
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
