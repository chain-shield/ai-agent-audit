
### Storage layout (AssetToken) 

```text
s_exchangeRate uint256

```

#### AssetToken.constructor(address,IERC20,string,string) [INTERNAL]
```slithir
STARTING_EXCHANGE_RATE_1(uint256) := phi(['STARTING_EXCHANGE_RATE_0', 'STARTING_EXCHANGE_RATE_4'])
 i_thunderLoan = thunderLoan
i_thunderLoan_1(address) := thunderLoan_1(address)
 i_underlying = underlying
i_underlying_1(IERC20) := underlying_1(IERC20)
 s_exchangeRate = STARTING_EXCHANGE_RATE
s_exchangeRate_1(uint256) := STARTING_EXCHANGE_RATE_4(uint256)
 ERC20(assetName,assetSymbol)
INTERNAL_CALL, ERC20.constructor(string,string)(assetName_1,assetSymbol_1)
 revertIfZeroAddress(thunderLoan)
MODIFIER_CALL, AssetToken.revertIfZeroAddress(address)(thunderLoan_1)
 revertIfZeroAddress(address(underlying))
TMP_409 = CONVERT underlying_1 to address
MODIFIER_CALL, AssetToken.revertIfZeroAddress(address)(TMP_409)
```
#### AssetToken.mint(address,uint256) [EXTERNAL]
```slithir
 _mint(to,amount)
INTERNAL_CALL, ERC20._mint(address,uint256)(to_1,amount_1)
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
#### AssetToken.getExchangeRate() [EXTERNAL]
```slithir
s_exchangeRate_7(uint256) := phi(['s_exchangeRate_6', 's_exchangeRate_1', 's_exchangeRate_0'])
 s_exchangeRate
RETURN s_exchangeRate_7
```
#### AssetToken.getUnderlying() [EXTERNAL]
```slithir
i_underlying_4(IERC20) := phi(['i_underlying_3', 'i_underlying_1', 'i_underlying_0'])
 i_underlying
RETURN i_underlying_4
```

#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_30(transfer) -> token_1.transfer
TMP_92(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_30,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffa4a3dd50>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffffa4a3ea10>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_92)
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
#### Address.functionCall(address,bytes) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0)
TMP_136(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256)(target_1,data_1,0)
RETURN TMP_136
```
