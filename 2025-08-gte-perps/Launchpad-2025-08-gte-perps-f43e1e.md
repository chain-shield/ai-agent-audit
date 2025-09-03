


### Storage layout (Launchpad) 

```text
launchpadLPVault LaunchpadLPVault
currentQuoteAsset LaunchToken
currentBondingCurve IBondingCurveMinimal
_launches mapping(address => ILaunchpad.LaunchData)
launchFee uint256
uniV2InitCodeHash bytes

```





### Storage layout (LaunchToken) 

```text
_name string
_symbol string
_mediaURI string
unlocked bool
eventNonce uint256
totalFeeShare uint256
bondingShare mapping(address => uint256)

```
#### Launchpad._assertValidRecipient(address,address) [INTERNAL]
```slithir
recipient_1(address) := phi(['REF_1104'])
baseToken_1(address) := phi(['REF_1105'])
uniV2Factory_7(IUniswapV2FactoryMinimal) := phi(['uniV2Factory_1', 'uniV2Factory_4', 'uniV2Factory_6', 'uniV2Factory_8', 'uniV2Factory_0'])
_launches_27(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_0', '_launches_1', '_launches_9', '_launches_15', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
 pair = pairFor(address(uniV2Factory),baseToken,_launches[baseToken].quote)
TMP_3319 = CONVERT uniV2Factory_7 to address
REF_1206(ILaunchpad.LaunchData) -> _launches_27[baseToken_1]
REF_1207(address) -> REF_1206.quote
TMP_3320(IUniswapV2Pair) = INTERNAL_CALL, Launchpad.pairFor(address,address,address)(TMP_3319,baseToken_1,REF_1207)
pair_1(IUniswapV2Pair) := TMP_3320(IUniswapV2Pair)
 address(pair) == recipient
TMP_3321 = CONVERT pair_1 to address
TMP_3322(bool) = TMP_3321 == recipient_1
CONDITION TMP_3322
 revert InvalidRecipient()()
TMP_3323(None) = SOLIDITY_CALL revert InvalidRecipient()()
 pair
RETURN pair_1
```
#### Launchpad._checkGraduation(address,ILaunchpad.LaunchData,uint256) [INTERNAL]
```slithir
token_1(address) := phi(['REF_1109'])
data_1(ILaunchpad.LaunchData) := phi(['data_1'])
amountOutBase_1(uint256) := phi(['REF_1110'])
 maxBaseForSale = data.curve.bondingSupply(token)
REF_1165(IBondingCurveMinimal) -> data_1.curve
TMP_3283(uint256) = HIGH_LEVEL_CALL, dest:REF_1165(IBondingCurveMinimal), function:bondingSupply, arguments:['token_1']  
maxBaseForSale_1(uint256) := TMP_3283(uint256)
 baseSold = data.curve.baseSoldFromCurve(token)
REF_1167(IBondingCurveMinimal) -> data_1.curve
TMP_3284(uint256) = HIGH_LEVEL_CALL, dest:REF_1167(IBondingCurveMinimal), function:baseSoldFromCurve, arguments:['token_1']  
baseSold_1(uint256) := TMP_3284(uint256)
 nextAmountSold = baseSold + amountOutBase
TMP_3285(uint256) = baseSold_1 (c)+ amountOutBase_1
nextAmountSold_1(uint256) := TMP_3285(uint256)
 nextAmountSold < maxBaseForSale
TMP_3286(bool) = nextAmountSold_1 < maxBaseForSale_1
CONDITION TMP_3286
 (amountOutBase,true)
RETURN amountOutBase_1,True
 amountOutBaseActual = maxBaseForSale - baseSold
TMP_3287(uint256) = maxBaseForSale_1 (c)- baseSold_1
amountOutBaseActual_1(uint256) := TMP_3287(uint256)
 (amountOutBaseActual,false)
RETURN amountOutBaseActual_1,False
 (amountOutBaseActual,stillActive)
```
#### Launchpad._createPairAndSwapRemaining(address,IUniswapV2Pair,ILaunchpad.LaunchData,uint256,uint256,address) [INTERNAL]
```slithir
token_1(address) := phi(['REF_1136'])
pair_1(IUniswapV2Pair) := phi(['pair_1'])
data_1(ILaunchpad.LaunchData) := phi(['data_1'])
remainingBase_1(uint256) := phi(['TMP_3220'])
remainingQuote_1(uint256) := phi(['TMP_3221'])
recipient_1(address) := phi(['REF_1139'])
uniV2Router_8(IUniswapV2RouterMinimal) := phi(['uniV2Router_15', 'uniV2Router_16', 'uniV2Router_2', 'uniV2Router_18', 'uniV2Router_7', 'uniV2Router_0'])
uniV2Factory_5(IUniswapV2FactoryMinimal) := phi(['uniV2Factory_1', 'uniV2Factory_4', 'uniV2Factory_6', 'uniV2Factory_8', 'uniV2Factory_0'])
launchpadLPVault_3(LaunchpadLPVault) := phi(['launchpadLPVault_2', 'launchpadLPVault_10', 'launchpadLPVault_1', 'launchpadLPVault_0'])
 p = uniV2Factory.createPair(token,data.quote)
REF_1170(address) -> data_1.quote
TMP_3288(address) = HIGH_LEVEL_CALL, dest:uniV2Factory_5(IUniswapV2FactoryMinimal), function:createPair, arguments:['token_1', 'REF_1170']  
uniV2Router_9(IUniswapV2RouterMinimal) := phi(['uniV2Router_8', 'uniV2Router_15', 'uniV2Router_16', 'uniV2Router_18', 'uniV2Router_2', 'uniV2Router_7'])
uniV2Factory_6(IUniswapV2FactoryMinimal) := phi(['uniV2Factory_1', 'uniV2Factory_4', 'uniV2Factory_5', 'uniV2Factory_6', 'uniV2Factory_8'])
launchpadLPVault_4(LaunchpadLPVault) := phi(['launchpadLPVault_3', 'launchpadLPVault_2', 'launchpadLPVault_10', 'launchpadLPVault_1'])
p_1(address) := TMP_3288(address)
 pair = IUniswapV2Pair(p)
TMP_3289 = CONVERT p_1 to IUniswapV2Pair
pair_3(IUniswapV2Pair) := TMP_3289(IUniswapV2Pair)
 pair.skim(owner())
pair_2(IUniswapV2Pair) := phi(['pair_1', 'pair_3'])
TMP_3290(address) = INTERNAL_CALL, Ownable.owner()()
HIGH_LEVEL_CALL, dest:pair_2(IUniswapV2Pair), function:skim, arguments:['TMP_3290']  
uniV2Router_11(IUniswapV2RouterMinimal) := phi(['uniV2Router_15', 'uniV2Router_10', 'uniV2Router_16', 'uniV2Router_18', 'uniV2Router_2', 'uniV2Router_7'])
launchpadLPVault_6(LaunchpadLPVault) := phi(['launchpadLPVault_2', 'launchpadLPVault_5', 'launchpadLPVault_10', 'launchpadLPVault_1'])
 tokensToLock = data.curve.totalSupply(token) - data.curve.bondingSupply(token)
REF_1172(IBondingCurveMinimal) -> data_1.curve
TMP_3292(uint256) = HIGH_LEVEL_CALL, dest:REF_1172(IBondingCurveMinimal), function:totalSupply, arguments:['token_1']  
uniV2Router_12(IUniswapV2RouterMinimal) := phi(['uniV2Router_11', 'uniV2Router_15', 'uniV2Router_16', 'uniV2Router_18', 'uniV2Router_2', 'uniV2Router_7'])
launchpadLPVault_7(LaunchpadLPVault) := phi(['launchpadLPVault_2', 'launchpadLPVault_6', 'launchpadLPVault_10', 'launchpadLPVault_1'])
REF_1174(IBondingCurveMinimal) -> data_1.curve
TMP_3293(uint256) = HIGH_LEVEL_CALL, dest:REF_1174(IBondingCurveMinimal), function:bondingSupply, arguments:['token_1']  
uniV2Router_13(IUniswapV2RouterMinimal) := phi(['uniV2Router_12', 'uniV2Router_15', 'uniV2Router_16', 'uniV2Router_18', 'uniV2Router_2', 'uniV2Router_7'])
launchpadLPVault_8(LaunchpadLPVault) := phi(['launchpadLPVault_2', 'launchpadLPVault_10', 'launchpadLPVault_1', 'launchpadLPVault_7'])
TMP_3294(uint256) = TMP_3292 (c)- TMP_3293
tokensToLock_1(uint256) := TMP_3294(uint256)
 quoteToLock = data.curve.quoteBoughtByCurve(token)
REF_1176(IBondingCurveMinimal) -> data_1.curve
TMP_3295(uint256) = HIGH_LEVEL_CALL, dest:REF_1176(IBondingCurveMinimal), function:quoteBoughtByCurve, arguments:['token_1']  
uniV2Router_14(IUniswapV2RouterMinimal) := phi(['uniV2Router_15', 'uniV2Router_16', 'uniV2Router_18', 'uniV2Router_2', 'uniV2Router_7', 'uniV2Router_13'])
launchpadLPVault_9(LaunchpadLPVault) := phi(['launchpadLPVault_2', 'launchpadLPVault_10', 'launchpadLPVault_1', 'launchpadLPVault_8'])
quoteToLock_1(uint256) := TMP_3295(uint256)
 token.safeApprove(address(uniV2Router),tokensToLock)
TMP_3296 = CONVERT uniV2Router_14 to address
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeApprove(address,address,uint256), arguments:['token_1', 'TMP_3296', 'tokensToLock_1'] 
 data.quote.safeApprove(address(uniV2Router),quoteToLock)
REF_1179(address) -> data_1.quote
TMP_3298 = CONVERT uniV2Router_14 to address
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeApprove(address,address,uint256), arguments:['REF_1179', 'TMP_3298', 'quoteToLock_1'] 
 uniV2Router.addLiquidity({tokenA:token,tokenB:address(data.quote),amountADesired:tokensToLock,amountBDesired:quoteToLock,amountAMin:0,amountBMin:0,to:address(launchpadLPVault),deadline:block.timestamp})
REF_1182(address) -> data_1.quote
TMP_3300 = CONVERT REF_1182 to address
TMP_3301 = CONVERT launchpadLPVault_9 to address
TUPLE_25(uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:uniV2Router_14(IUniswapV2RouterMinimal), function:addLiquidity, arguments:['token_1', 'TMP_3300', 'tokensToLock_1', 'quoteToLock_1', '0', '0', 'TMP_3301', 'block.timestamp']  
uniV2Router_15(IUniswapV2RouterMinimal) := phi(['uniV2Router_14', 'uniV2Router_15', 'uniV2Router_16', 'uniV2Router_18', 'uniV2Router_2', 'uniV2Router_7'])
launchpadLPVault_10(LaunchpadLPVault) := phi(['launchpadLPVault_2', 'launchpadLPVault_9', 'launchpadLPVault_10', 'launchpadLPVault_1'])
 remainingBase > 0 && remainingQuote > 0
TMP_3302(bool) = remainingBase_1 > 0
TMP_3303(bool) = remainingQuote_1 > 0
TMP_3304(bool) = TMP_3302 && TMP_3303
CONDITION TMP_3304
 quoteNeeded = uniV2Router.getAmountIn({amountOut:remainingBase,reserveIn:quoteToLock,reserveOut:tokensToLock})
TMP_3305(uint256) = HIGH_LEVEL_CALL, dest:uniV2Router_15(IUniswapV2RouterMinimal), function:getAmountIn, arguments:['remainingBase_1', 'quoteToLock_1', 'tokensToLock_1']  
uniV2Router_16(IUniswapV2RouterMinimal) := phi(['uniV2Router_15', 'uniV2Router_16', 'uniV2Router_18', 'uniV2Router_2', 'uniV2Router_7'])
quoteNeeded_1(uint256) := TMP_3305(uint256)
 remainingQuote >= quoteNeeded
TMP_3306(bool) = remainingQuote_1 >= quoteNeeded_1
CONDITION TMP_3306
 d = SwapRemainingData({token:token,quote:data.quote,recipient:recipient,baseAmount:remainingBase,quoteAmount:quoteNeeded})
REF_1184(address) -> data_1.quote
TMP_3307(Launchpad.SwapRemainingData) = new SwapRemainingData(token_1,REF_1184,recipient_1,remainingBase_1,quoteNeeded_1)
d_1(Launchpad.SwapRemainingData) := TMP_3307(Launchpad.SwapRemainingData)
 (None,quoteUsed) = _swapRemaining(d)
TUPLE_26(uint256,uint256) = INTERNAL_CALL, Launchpad._swapRemaining(Launchpad.SwapRemainingData)(d_1)
quoteUsed_1(uint256)= UNPACK TUPLE_26 index: 1 
 quoteUsed
RETURN quoteUsed_1
 0
RETURN 0
 additionalQuoteUsed
```
#### Launchpad._emitSwapEvent(address,address,uint256,uint256,bool,IBondingCurveMinimal) [INTERNAL]
```slithir
account_1(address) := phi(['REF_1122', 'account_1'])
token_1(address) := phi(['token_1', 'REF_1123'])
baseAmount_1(uint256) := phi(['amountOutBaseActual_1', 'amountInBase_1'])
quoteAmount_1(uint256) := phi(['amountInQuote_1', 'amountOutQuote_1'])
curve_1(IBondingCurveMinimal) := phi(['REF_1124', 'REF_1146'])
 Swap({buyer:account,token:token,baseDelta:baseDelta,quoteDelta:quoteDelta,nextAmountSold:curve.baseSoldFromCurve(token),newPrice:curve.quoteBoughtByCurve(token),eventNonce:EventNonceLib.inc()})
TMP_3332(uint256) = HIGH_LEVEL_CALL, dest:curve_1(IBondingCurveMinimal), function:baseSoldFromCurve, arguments:['token_1']  
TMP_3333(uint256) = HIGH_LEVEL_CALL, dest:curve_1(IBondingCurveMinimal), function:quoteBoughtByCurve, arguments:['token_1']  
TMP_3334(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit Swap(account_1,token_1,baseDelta_3,quoteDelta_3,TMP_3332,TMP_3333,TMP_3334)
 isBuy
CONDITION isBuy_1
 baseDelta = int256(baseAmount)
TMP_3336 = CONVERT baseAmount_1 to int256
baseDelta_1(int256) := TMP_3336(int256)
 baseDelta = - int256(baseAmount)
TMP_3337 = CONVERT baseAmount_1 to int256
TMP_3338(int256) = 0 (c)- TMP_3337
baseDelta_2(int256) := TMP_3338(int256)
baseDelta_3(int256) := phi(['baseDelta_1', 'baseDelta_2'])
 isBuy
CONDITION isBuy_1
 quoteDelta = - int256(quoteAmount)
TMP_3339 = CONVERT quoteAmount_1 to int256
TMP_3340(int256) = 0 (c)- TMP_3339
quoteDelta_1(int256) := TMP_3340(int256)
 quoteDelta = int256(quoteAmount)
TMP_3341 = CONVERT quoteAmount_1 to int256
quoteDelta_2(int256) := TMP_3341(int256)
quoteDelta_3(int256) := phi(['quoteDelta_1', 'quoteDelta_2'])
```
#### Launchpad._graduate(ILaunchpad.BuyData,IUniswapV2Pair,ILaunchpad.LaunchData,uint256,uint256) [INTERNAL]
```slithir
buyData_1(ILaunchpad.BuyData) := phi(['buyData_1'])
pair_1(IUniswapV2Pair) := phi(['pair_1'])
data_1(ILaunchpad.LaunchData) := phi(['data_1'])
amountOutBaseActual_1(uint256) := phi(['amountOutBaseActual_1'])
amountInQuote_1(uint256) := phi(['amountInQuote_1'])
_launches_16(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_0', '_launches_1', '_launches_9', '_launches_15', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
 LaunchToken(buyData.token).unlock()
REF_1129(address) -> buyData_1.token
TMP_3216 = CONVERT REF_1129 to LaunchToken
HIGH_LEVEL_CALL, dest:TMP_3216(LaunchToken), function:unlock, arguments:[]  
_launches_17(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_1', '_launches_15', '_launches_9', '_launches_24', '_launches_3', '_launches_26', '_launches_16', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
 _launches[buyData.token].active = false
REF_1131(address) -> buyData_1.token
REF_1132(ILaunchpad.LaunchData) -> _launches_17[REF_1131]
REF_1133(bool) -> REF_1132.active
_launches_18(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_17'])
REF_1133(bool) (->_launches_18) := False(bool)
 BondingLocked(buyData.token,pair,EventNonceLib.inc())
REF_1134(address) -> buyData_1.token
TMP_3218(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit BondingLocked(REF_1134,pair_1,TMP_3218)
 additionalQuote = _createPairAndSwapRemaining({token:buyData.token,pair:pair,data:data,remainingBase:buyData.amountOutBase - amountOutBaseActual,remainingQuote:buyData.maxAmountInQuote - amountInQuote,recipient:buyData.recipient})
REF_1136(address) -> buyData_1.token
REF_1137(uint256) -> buyData_1.amountOutBase
TMP_3220(uint256) = REF_1137 (c)- amountOutBaseActual_1
REF_1138(uint256) -> buyData_1.maxAmountInQuote
TMP_3221(uint256) = REF_1138 (c)- amountInQuote_1
REF_1139(address) -> buyData_1.recipient
TMP_3222(uint256) = INTERNAL_CALL, Launchpad._createPairAndSwapRemaining(address,IUniswapV2Pair,ILaunchpad.LaunchData,uint256,uint256,address)(REF_1136,pair_1,data_1,TMP_3220,TMP_3221,REF_1139)
additionalQuote_1(uint256) := TMP_3222(uint256)
 finalAmountInQuote = amountInQuote + additionalQuote
TMP_3223(uint256) = amountInQuote_1 (c)+ additionalQuote_1
finalAmountInQuote_1(uint256) := TMP_3223(uint256)
 additionalQuote > 0
TMP_3224(bool) = additionalQuote_1 > 0
CONDITION TMP_3224
 finalAmountOutBaseActual = buyData.amountOutBase
REF_1140(uint256) -> buyData_1.amountOutBase
finalAmountOutBaseActual_1(uint256) := REF_1140(uint256)
 finalAmountOutBaseActual = amountOutBaseActual
finalAmountOutBaseActual_2(uint256) := amountOutBaseActual_1(uint256)
finalAmountOutBaseActual_3(uint256) := phi(['finalAmountOutBaseActual_1', 'finalAmountOutBaseActual_2'])
 (finalAmountOutBaseActual,finalAmountInQuote)
RETURN finalAmountOutBaseActual_3,finalAmountInQuote_1
```
#### Launchpad._swapRemaining(Launchpad.SwapRemainingData) [INTERNAL]
```slithir
data_1(Launchpad.SwapRemainingData) := phi(['d_1'])
uniV2Router_17(IUniswapV2RouterMinimal) := phi(['uniV2Router_15', 'uniV2Router_16', 'uniV2Router_2', 'uniV2Router_18', 'uniV2Router_7', 'uniV2Router_0'])
 data.quote.safeTransferFrom(msg.sender,address(this),data.quoteAmount)
REF_1185(address) -> data_1.quote
TMP_3308 = CONVERT this to address
REF_1187(uint256) -> data_1.quoteAmount
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransferFrom(address,address,address,uint256), arguments:['REF_1185', 'msg.sender', 'TMP_3308', 'REF_1187'] 
 path = new address[](2)
TMP_3311(address[])  = new address[](2)
path_1(address[]) = ['TMP_3311(address[])']
 path[0] = data.quote
REF_1188(address) -> path_1[0]
REF_1189(address) -> data_1.quote
path_2(address[]) := phi(['path_1'])
REF_1188(address) (->path_2) := REF_1189(address)
 path[1] = data.token
REF_1190(address) -> path_2[1]
REF_1191(address) -> data_1.token
path_3(address[]) := phi(['path_2'])
REF_1190(address) (->path_3) := REF_1191(address)
 data.quote.safeApprove(address(uniV2Router),data.quoteAmount)
REF_1192(address) -> data_1.quote
TMP_3312 = CONVERT uniV2Router_17 to address
REF_1194(uint256) -> data_1.quoteAmount
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeApprove(address,address,uint256), arguments:['REF_1192', 'TMP_3312', 'REF_1194'] 
 uniV2Router.swapTokensForExactTokens(data.baseAmount,data.quoteAmount,path,data.recipient,block.timestamp + 1)
REF_1196(uint256) -> data_1.baseAmount
REF_1197(uint256) -> data_1.quoteAmount
REF_1198(address) -> data_1.recipient
TMP_3314(uint256) = block.timestamp (c)+ 1
TMP_3315(uint256[]) = HIGH_LEVEL_CALL, dest:uniV2Router_17(IUniswapV2RouterMinimal), function:swapTokensForExactTokens, arguments:['REF_1196', 'REF_1197', 'path_3', 'REF_1198', 'TMP_3314']  
uniV2Router_18(IUniswapV2RouterMinimal) := phi(['uniV2Router_15', 'uniV2Router_16', 'uniV2Router_18', 'uniV2Router_2', 'uniV2Router_7', 'uniV2Router_17'])
 (data.baseAmount,data.quoteAmount)
REF_1199(uint256) -> data_1.baseAmount
REF_1200(uint256) -> data_1.quoteAmount
RETURN REF_1199,REF_1200
 data.quote.safeApprove(address(uniV2Router),0)
REF_1201(address) -> data_1.quote
TMP_3316 = CONVERT uniV2Router_18 to address
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeApprove(address,address,uint256), arguments:['REF_1201', 'TMP_3316', '0'] 
 data.quote.safeTransfer(msg.sender,data.quoteAmount)
REF_1203(address) -> data_1.quote
REF_1205(uint256) -> data_1.quoteAmount
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransfer(address,address,uint256), arguments:['REF_1203', 'msg.sender', 'REF_1205'] 
 (0,0)
RETURN 0,0
```
#### Launchpad.baseSoldFromCurve(address) [PUBLIC]
```slithir
_launches_2(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_0', '_launches_1', '_launches_9', '_launches_15', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
 _launches[token].curve.baseSoldFromCurve(token)
REF_1086(ILaunchpad.LaunchData) -> _launches_2[token_1]
REF_1087(IBondingCurveMinimal) -> REF_1086.curve
TMP_3174(uint256) = HIGH_LEVEL_CALL, dest:REF_1087(IBondingCurveMinimal), function:baseSoldFromCurve, arguments:['token_1']  
_launches_3(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_1', '_launches_15', '_launches_9', '_launches_24', '_launches_3', '_launches_2', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
RETURN TMP_3174
```
#### Launchpad.buy(ILaunchpad.BuyData) [EXTERNAL]
```slithir
_launches_11(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_0', '_launches_1', '_launches_9', '_launches_15', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
 pair = _assertValidRecipient(buyData.recipient,buyData.token)
REF_1104(address) -> buyData_1.recipient
REF_1105(address) -> buyData_1.token
TMP_3200(IUniswapV2Pair) = INTERNAL_CALL, Launchpad._assertValidRecipient(address,address)(REF_1104,REF_1105)
_launches_15(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_28'])
pair_1(IUniswapV2Pair) := TMP_3200(IUniswapV2Pair)
 data = _launches[buyData.token]
REF_1106(address) -> buyData_1.token
REF_1107(ILaunchpad.LaunchData) -> _launches_15[REF_1106]
data_1(ILaunchpad.LaunchData) := REF_1107(ILaunchpad.LaunchData)
 (amountOutBaseActual,data.active) = _checkGraduation(buyData.token,data,buyData.amountOutBase)
REF_1108(bool) -> data_1.active
REF_1109(address) -> buyData_1.token
REF_1110(uint256) -> buyData_1.amountOutBase
TUPLE_20(uint256,bool) = INTERNAL_CALL, Launchpad._checkGraduation(address,ILaunchpad.LaunchData,uint256)(REF_1109,data_1,REF_1110)
amountOutBaseActual_1(uint256)= UNPACK TUPLE_20 index: 0 
REF_1108(bool)= UNPACK TUPLE_20 index: 1 
 amountInQuote = data.curve.buy(buyData.token,amountOutBaseActual)
REF_1111(IBondingCurveMinimal) -> data_1.curve
REF_1113(address) -> buyData_1.token
TMP_3201(uint256) = HIGH_LEVEL_CALL, dest:REF_1111(IBondingCurveMinimal), function:buy, arguments:['REF_1113', 'amountOutBaseActual_1']  
amountInQuote_1(uint256) := TMP_3201(uint256)
 data.active && amountInQuote == 0
REF_1114(bool) -> data_1.active
TMP_3202(bool) = amountInQuote_1 == 0
TMP_3203(bool) = REF_1114 && TMP_3202
CONDITION TMP_3203
 revert DustAttackInvalid()()
TMP_3204(None) = SOLIDITY_CALL revert DustAttackInvalid()()
 amountInQuote > buyData.maxAmountInQuote
REF_1115(uint256) -> buyData_1.maxAmountInQuote
TMP_3205(bool) = amountInQuote_1 > REF_1115
CONDITION TMP_3205
 revert SlippageToleranceExceeded()()
TMP_3206(None) = SOLIDITY_CALL revert SlippageToleranceExceeded()()
 buyData.token.safeTransfer(buyData.recipient,amountOutBaseActual)
REF_1116(address) -> buyData_1.token
REF_1118(address) -> buyData_1.recipient
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransfer(address,address,uint256), arguments:['REF_1116', 'REF_1118', 'amountOutBaseActual_1'] 
 address(data.quote).safeTransferFrom(buyData.account,address(this),amountInQuote)
REF_1119(address) -> data_1.quote
TMP_3208 = CONVERT REF_1119 to address
REF_1121(address) -> buyData_1.account
TMP_3209 = CONVERT this to address
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransferFrom(address,address,address,uint256), arguments:['TMP_3208', 'REF_1121', 'TMP_3209', 'amountInQuote_1'] 
 _emitSwapEvent({account:buyData.account,token:buyData.token,baseAmount:amountOutBaseActual,quoteAmount:amountInQuote,isBuy:true,curve:data.curve})
REF_1122(address) -> buyData_1.account
REF_1123(address) -> buyData_1.token
REF_1124(IBondingCurveMinimal) -> data_1.curve
INTERNAL_CALL, Launchpad._emitSwapEvent(address,address,uint256,uint256,bool,IBondingCurveMinimal)(REF_1122,REF_1123,amountOutBaseActual_1,amountInQuote_1,True,REF_1124)
 ! data.active
REF_1125(bool) -> data_1.active
TMP_3212 = UnaryType.BANG REF_1125 
CONDITION TMP_3212
 (amountOutBaseActual,amountInQuote) = _graduate(buyData,pair,data,amountOutBaseActual,amountInQuote)
TUPLE_21(uint256,uint256) = INTERNAL_CALL, Launchpad._graduate(ILaunchpad.BuyData,IUniswapV2Pair,ILaunchpad.LaunchData,uint256,uint256)(buyData_1,pair_1,data_1,amountOutBaseActual_1,amountInQuote_1)
amountOutBaseActual_2(uint256)= UNPACK TUPLE_21 index: 0 
amountInQuote_2(uint256)= UNPACK TUPLE_21 index: 1 
amountOutBaseActual_3(uint256) := phi(['amountOutBaseActual_2', 'amountOutBaseActual_1'])
amountInQuote_3(uint256) := phi(['amountInQuote_2', 'amountInQuote_1'])
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 onlyBondingActive(buyData.token)
REF_1126(address) -> buyData_1.token
MODIFIER_CALL, Launchpad.onlyBondingActive(address)(REF_1126)
_launches_13(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_30'])
 onlySenderOrOperator(buyData.account,SpotOperatorRoles.LAUNCHPAD_FILL)
REF_1127(address) -> buyData_1.account
REF_1128(SpotOperatorRoles) -> SpotOperatorRoles.LAUNCHPAD_FILL
MODIFIER_CALL, Launchpad.onlySenderOrOperator(address,SpotOperatorRoles)(REF_1127,REF_1128)
 (amountOutBaseActual,amountInQuote)
RETURN amountOutBaseActual_3,amountInQuote_3
```
#### Launchpad.constructor(address,address,address,address,address) [PUBLIC]
```slithir
 uniV2Router = IUniswapV2RouterMinimal(uniV2Router_)
TMP_3149 = CONVERT uniV2Router__1 to IUniswapV2RouterMinimal
uniV2Router_1(IUniswapV2RouterMinimal) := TMP_3149(IUniswapV2RouterMinimal)
 gteRouter = gteRouter_
gteRouter_1(address) := gteRouter__1(address)
 operator = IOperatorPanel(operator_)
TMP_3150 = CONVERT operator__1 to IOperatorPanel
operator_1(IOperatorPanel) := TMP_3150(IOperatorPanel)
 uniV2Factory = IUniswapV2FactoryMinimal(uniV2Router.factory())
TMP_3151(address) = HIGH_LEVEL_CALL, dest:uniV2Router_1(IUniswapV2RouterMinimal), function:factory, arguments:[]  
uniV2Router_2(IUniswapV2RouterMinimal) := phi(['uniV2Router_15', 'uniV2Router_16', 'uniV2Router_1', 'uniV2Router_18', 'uniV2Router_2', 'uniV2Router_7'])
TMP_3152 = CONVERT TMP_3151 to IUniswapV2FactoryMinimal
uniV2Factory_1(IUniswapV2FactoryMinimal) := TMP_3152(IUniswapV2FactoryMinimal)
 distributor = IDistributor(distributor_)
TMP_3153 = CONVERT distributor__1 to IDistributor
distributor_1(IDistributor) := TMP_3153(IDistributor)
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### Launchpad.decreaseStake(address,uint96) [EXTERNAL]
```slithir
distributor_9(IDistributor) := phi(['distributor_15', 'distributor_5', 'distributor_11', 'distributor_1', 'distributor_0', 'distributor_8'])
 distributor.decreaseStake(msg.sender,account,shares)
TUPLE_24(uint256,uint256) = HIGH_LEVEL_CALL, dest:distributor_10(IDistributor), function:decreaseStake, arguments:['msg.sender', 'account_1', 'shares_1']  
distributor_11(IDistributor) := phi(['distributor_15', 'distributor_5', 'distributor_11', 'distributor_1', 'distributor_10', 'distributor_8'])
 onlyLaunchAsset()
MODIFIER_CALL, Launchpad.onlyLaunchAsset()()
```
#### Launchpad.endRewards() [EXTERNAL]
```slithir
distributor_12(IDistributor) := phi(['distributor_15', 'distributor_5', 'distributor_11', 'distributor_1', 'distributor_0', 'distributor_8'])
uniV2Factory_2(IUniswapV2FactoryMinimal) := phi(['uniV2Factory_1', 'uniV2Factory_4', 'uniV2Factory_6', 'uniV2Factory_8', 'uniV2Factory_0'])
_launches_25(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_0', '_launches_1', '_launches_9', '_launches_15', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
 quote = _launches[msg.sender].quote
REF_1162(ILaunchpad.LaunchData) -> _launches_26[msg.sender]
REF_1163(address) -> REF_1162.quote
quote_1(address) := REF_1163(address)
 pair = IGTELaunchpadV2Pair(address(pairFor(address(uniV2Factory),msg.sender,quote)))
TMP_3277 = CONVERT uniV2Factory_3 to address
TMP_3278(IUniswapV2Pair) = INTERNAL_CALL, Launchpad.pairFor(address,address,address)(TMP_3277,msg.sender,quote_1)
TMP_3279 = CONVERT TMP_3278 to address
TMP_3280 = CONVERT TMP_3279 to IGTELaunchpadV2Pair
pair_1(IGTELaunchpadV2Pair) := TMP_3280(IGTELaunchpadV2Pair)
 distributor.endRewards(pair)
HIGH_LEVEL_CALL, dest:distributor_14(IDistributor), function:endRewards, arguments:['pair_1']  
distributor_15(IDistributor) := phi(['distributor_15', 'distributor_5', 'distributor_11', 'distributor_1', 'distributor_8', 'distributor_14'])
 onlyLaunchAsset()
MODIFIER_CALL, Launchpad.onlyLaunchAsset()()
_launches_26(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_29'])
```
#### Launchpad.eventNonce() [EXTERNAL]
```slithir
 EventNonceLib.getCurrentNonce()
TMP_3178(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.getCurrentNonce(), arguments:[] 
RETURN TMP_3178
```
#### Launchpad.increaseStake(address,uint96) [EXTERNAL]
```slithir
distributor_6(IDistributor) := phi(['distributor_15', 'distributor_5', 'distributor_11', 'distributor_1', 'distributor_0', 'distributor_8'])
 distributor.increaseStake(msg.sender,account,shares)
TUPLE_23(uint256,uint256) = HIGH_LEVEL_CALL, dest:distributor_7(IDistributor), function:increaseStake, arguments:['msg.sender', 'account_1', 'shares_1']  
distributor_8(IDistributor) := phi(['distributor_15', 'distributor_5', 'distributor_11', 'distributor_7', 'distributor_1', 'distributor_8'])
 onlyLaunchAsset()
MODIFIER_CALL, Launchpad.onlyLaunchAsset()()
```
#### Launchpad.initialize(address,address,address,address,bytes) [EXTERNAL]
```slithir
uniV2Router_3(IUniswapV2RouterMinimal) := phi(['uniV2Router_15', 'uniV2Router_16', 'uniV2Router_2', 'uniV2Router_18', 'uniV2Router_7', 'uniV2Router_0'])
 _initializeOwner(owner_)
INTERNAL_CALL, Ownable._initializeOwner(address)(owner__1)
 quoteAsset_ == address(0)
TMP_3156 = CONVERT 0 to address
TMP_3157(bool) = quoteAsset__1 == TMP_3156
CONDITION TMP_3157
 revert InvalidQuoteAsset()()
TMP_3158(None) = SOLIDITY_CALL revert InvalidQuoteAsset()()
 ! ERC165Checker.supportsInterface(bondingCurve_,type()(IBondingCurveMinimal).interfaceId)
TMP_3159(type(IBondingCurveMinimal)) = SOLIDITY_CALL type()(IBondingCurveMinimal)
REF_1081(bytes4) (->None) := 1992782646(bytes4)
TMP_3160(bool) = LIBRARY_CALL, dest:ERC165Checker, function:ERC165Checker.supportsInterface(address,bytes4), arguments:['bondingCurve__1', 'REF_1081'] 
TMP_3161 = UnaryType.BANG TMP_3160 
CONDITION TMP_3161
 revert InvalidCurve()()
TMP_3162(None) = SOLIDITY_CALL revert InvalidCurve()()
 LaunchToken(quoteAsset_).approve(address(this),0)
TMP_3163 = CONVERT quoteAsset__1 to LaunchToken
TMP_3164 = CONVERT this to address
TMP_3165(bool) = HIGH_LEVEL_CALL, dest:TMP_3163(LaunchToken), function:approve, arguments:['TMP_3164', '0']  
uniV2Router_6(IUniswapV2RouterMinimal) := phi(['uniV2Router_15', 'uniV2Router_5', 'uniV2Router_16', 'uniV2Router_18', 'uniV2Router_2', 'uniV2Router_7'])
 currentBondingCurve = IBondingCurveMinimal(bondingCurve_)
TMP_3166 = CONVERT bondingCurve__1 to IBondingCurveMinimal
currentBondingCurve_1(IBondingCurveMinimal) := TMP_3166(IBondingCurveMinimal)
 currentQuoteAsset = LaunchToken(quoteAsset_)
TMP_3167 = CONVERT quoteAsset__1 to LaunchToken
currentQuoteAsset_1(LaunchToken) := TMP_3167(LaunchToken)
 launchpadLPVault = LaunchpadLPVault(launchpadLPVault_)
TMP_3168 = CONVERT launchpadLPVault__1 to LaunchpadLPVault
launchpadLPVault_1(LaunchpadLPVault) := TMP_3168(LaunchpadLPVault)
 currentBondingCurve.init(bondingCurveInitData)
HIGH_LEVEL_CALL, dest:currentBondingCurve_1(IBondingCurveMinimal), function:init, arguments:['bondingCurveInitData_1']  
uniV2Router_7(IUniswapV2RouterMinimal) := phi(['uniV2Router_15', 'uniV2Router_16', 'uniV2Router_18', 'uniV2Router_2', 'uniV2Router_7', 'uniV2Router_6'])
currentBondingCurve_2(IBondingCurveMinimal) := phi(['currentBondingCurve_1', 'currentBondingCurve_4', 'currentBondingCurve_7', 'currentBondingCurve_2'])
 LaunchpadDeployed(quoteAsset_,bondingCurve_,address(uniV2Router),EventNonceLib.inc())
TMP_3170 = CONVERT uniV2Router_7 to address
TMP_3171(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit LaunchpadDeployed(quoteAsset__1,bondingCurve__1,TMP_3170,TMP_3171)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### Launchpad.launch(string,string,string) [EXTERNAL]
```slithir
TOTAL_SUPPLY_1(uint256) := phi(['TOTAL_SUPPLY_5', 'TOTAL_SUPPLY_0'])
BONDING_SUPPLY_1(uint256) := phi(['BONDING_SUPPLY_3', 'BONDING_SUPPLY_0'])
gteRouter_2(address) := phi(['gteRouter_0', 'gteRouter_1', 'gteRouter_3'])
distributor_2(IDistributor) := phi(['distributor_15', 'distributor_5', 'distributor_11', 'distributor_1', 'distributor_0', 'distributor_8'])
currentQuoteAsset_2(LaunchToken) := phi(['currentQuoteAsset_8', 'currentQuoteAsset_0', 'currentQuoteAsset_3', 'currentQuoteAsset_1'])
currentBondingCurve_3(IBondingCurveMinimal) := phi(['currentBondingCurve_4', 'currentBondingCurve_2', 'currentBondingCurve_7', 'currentBondingCurve_0'])
launchFee_1(uint256) := phi(['launchFee_0', 'launchFee_3', 'launchFee_2'])
 msg.value != launchFee
TMP_3179(bool) = msg.value != launchFee_2
CONDITION TMP_3179
 revert BadLaunchFee()()
TMP_3180(None) = SOLIDITY_CALL revert BadLaunchFee()()
 quote = address(currentQuoteAsset)
TMP_3181 = CONVERT currentQuoteAsset_3 to address
quote_1(address) := TMP_3181(address)
 curve = currentBondingCurve
curve_1(IBondingCurveMinimal) := currentBondingCurve_4(IBondingCurveMinimal)
 quote == address(0)
TMP_3182 = CONVERT 0 to address
TMP_3183(bool) = quote_1 == TMP_3182
CONDITION TMP_3183
 revert UninitializedQuote()()
TMP_3184(None) = SOLIDITY_CALL revert UninitializedQuote()()
 address(curve) == address(0)
TMP_3185 = CONVERT curve_1 to address
TMP_3186 = CONVERT 0 to address
TMP_3187(bool) = TMP_3185 == TMP_3186
CONDITION TMP_3187
 revert UninitializedCurve()()
TMP_3188(None) = SOLIDITY_CALL revert UninitializedCurve()()
 token = address(new LaunchToken(name,symbol,mediaURI,gteRouter))
TMP_3190(LaunchToken) = new LaunchToken(name_1,symbol_1,mediaURI_1,gteRouter_3) 
TMP_3191 = CONVERT TMP_3190 to address
token_1(address) := TMP_3191(address)
 curve.initializeCurve(token,TOTAL_SUPPLY,BONDING_SUPPLY)
HIGH_LEVEL_CALL, dest:curve_1(IBondingCurveMinimal), function:initializeCurve, arguments:['token_1', 'TOTAL_SUPPLY_2', 'BONDING_SUPPLY_2']  
TOTAL_SUPPLY_3(uint256) := phi(['TOTAL_SUPPLY_5', 'TOTAL_SUPPLY_2'])
BONDING_SUPPLY_3(uint256) := phi(['BONDING_SUPPLY_2', 'BONDING_SUPPLY_3'])
distributor_4(IDistributor) := phi(['distributor_15', 'distributor_3', 'distributor_5', 'distributor_11', 'distributor_1', 'distributor_8'])
 distributor.createRewardsPair(token,quote)
HIGH_LEVEL_CALL, dest:distributor_4(IDistributor), function:createRewardsPair, arguments:['token_1', 'quote_1']  
TOTAL_SUPPLY_4(uint256) := phi(['TOTAL_SUPPLY_5', 'TOTAL_SUPPLY_3'])
distributor_5(IDistributor) := phi(['distributor_15', 'distributor_5', 'distributor_11', 'distributor_1', 'distributor_4', 'distributor_8'])
 _launches[token] = LaunchData({active:true,curve:curve,quote:quote})
REF_1101(ILaunchpad.LaunchData) -> _launches_9[token_1]
TMP_3194(ILaunchpad.LaunchData) = new LaunchData(True,quote_1,curve_1)
_launches_10(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_9'])
REF_1101(ILaunchpad.LaunchData) (->_launches_10) := TMP_3194(ILaunchpad.LaunchData)
 TokenLaunched({dev:msg.sender,token:token,quoteAsset:quote,bondingCurve:curve,timestamp:block.timestamp,eventNonce:EventNonceLib.inc()})
TMP_3195(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit TokenLaunched(msg.sender,token_1,quote_1,curve_1,block.timestamp,TMP_3195)
 LaunchToken(token).mint(TOTAL_SUPPLY)
TMP_3197 = CONVERT token_1 to LaunchToken
HIGH_LEVEL_CALL, dest:TMP_3197(LaunchToken), function:mint, arguments:['TOTAL_SUPPLY_4']  
TOTAL_SUPPLY_5(uint256) := phi(['TOTAL_SUPPLY_5', 'TOTAL_SUPPLY_4'])
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 token
RETURN token_1
```
#### Launchpad.launches(address) [PUBLIC]
```slithir
_launches_1(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_0', '_launches_1', '_launches_9', '_launches_15', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
 _launches[launchToken]
REF_1085(ILaunchpad.LaunchData) -> _launches_1[launchToken_1]
RETURN REF_1085
```
#### Launchpad.pairFor(address,address,address) [INTERNAL]
```slithir
factory_1(address) := phi(['TMP_3277', 'TMP_3319'])
tokenA_1(address) := phi(['msg.sender', 'baseToken_1'])
tokenB_1(address) := phi(['REF_1207', 'quote_1'])
uniV2InitCodeHash_2(bytes) := phi(['uniV2InitCodeHash_1', 'uniV2InitCodeHash_0', 'uniV2InitCodeHash_3'])
 (token0,token1) = sortTokens(tokenA,tokenB)
TUPLE_27(address,address) = INTERNAL_CALL, Launchpad.sortTokens(address,address)(tokenA_1,tokenB_1)
token0_1(address)= UNPACK TUPLE_27 index: 0 
token1_1(address)= UNPACK TUPLE_27 index: 1 
 pair = IUniswapV2Pair(address(uint160(uint256(keccak256(bytes)(abi.encodePacked(0xff,factory,keccak256(bytes)(abi.encodePacked(token0,token1)),uniV2InitCodeHash))))))
TMP_3324(bytes) = SOLIDITY_CALL abi.encodePacked()(token0_1,token1_1)
TMP_3325(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_3324)
TMP_3326(bytes) = SOLIDITY_CALL abi.encodePacked()(0xff,factory_1,TMP_3325,uniV2InitCodeHash_3)
TMP_3327(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_3326)
TMP_3328 = CONVERT TMP_3327 to uint256
TMP_3329 = CONVERT TMP_3328 to uint160
TMP_3330 = CONVERT TMP_3329 to address
TMP_3331 = CONVERT TMP_3330 to IUniswapV2Pair
pair_1(IUniswapV2Pair) := TMP_3331(IUniswapV2Pair)
 pair
RETURN pair_1
```
#### Launchpad.pullFees() [EXTERNAL][OWNER]
```slithir
 (success,None) = address(msg.sender).call{value: address(this).balance}()
TMP_3266 = CONVERT msg.sender to address
TMP_3267 = CONVERT this to address
TMP_3268(uint256) = SOLIDITY_CALL balance(address)(TMP_3267)
TUPLE_22(bool,bytes) = LOW_LEVEL_CALL, dest:TMP_3266, function:call, arguments:[''] value:TMP_3268 
success_1(bool)= UNPACK TUPLE_22 index: 0 
 ! success
TMP_3269 = UnaryType.BANG success_1 
CONDITION TMP_3269
 revert ETHTransferFailed()()
TMP_3270(None) = SOLIDITY_CALL revert ETHTransferFailed()()
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### Launchpad.quoteBaseForQuote(address,uint256,bool) [PUBLIC]
```slithir
_launches_6(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_0', '_launches_1', '_launches_9', '_launches_15', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
 _launches[token].curve.quoteBaseForQuote(token,quoteAmount,isBuy)
REF_1092(ILaunchpad.LaunchData) -> _launches_6[token_1]
REF_1093(IBondingCurveMinimal) -> REF_1092.curve
TMP_3176(uint256) = HIGH_LEVEL_CALL, dest:REF_1093(IBondingCurveMinimal), function:quoteBaseForQuote, arguments:['token_1', 'quoteAmount_1', 'isBuy_1']  
_launches_7(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_1', '_launches_15', '_launches_9', '_launches_24', '_launches_3', '_launches_26', '_launches_6', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
RETURN TMP_3176
 baseAmount
```
#### Launchpad.quoteBoughtByCurve(address) [PUBLIC]
```slithir
_launches_4(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_0', '_launches_1', '_launches_9', '_launches_15', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
 _launches[token].curve.quoteBoughtByCurve(token)
REF_1089(ILaunchpad.LaunchData) -> _launches_4[token_1]
REF_1090(IBondingCurveMinimal) -> REF_1089.curve
TMP_3175(uint256) = HIGH_LEVEL_CALL, dest:REF_1090(IBondingCurveMinimal), function:quoteBoughtByCurve, arguments:['token_1']  
_launches_5(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_4', '_launches_7', '_launches_1', '_launches_15', '_launches_9', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
RETURN TMP_3175
```
#### Launchpad.quoteQuoteForBase(address,uint256,bool) [PUBLIC]
```slithir
_launches_8(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_0', '_launches_1', '_launches_9', '_launches_15', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
 _launches[token].curve.quoteQuoteForBase(token,baseAmount,isBuy)
REF_1095(ILaunchpad.LaunchData) -> _launches_8[token_1]
REF_1096(IBondingCurveMinimal) -> REF_1095.curve
TMP_3177(uint256) = HIGH_LEVEL_CALL, dest:REF_1096(IBondingCurveMinimal), function:quoteQuoteForBase, arguments:['token_1', 'baseAmount_1', 'isBuy_1']  
_launches_9(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_1', '_launches_15', '_launches_9', '_launches_8', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
RETURN TMP_3177
 quoteAmount
```
#### Launchpad.sell(address,address,address,uint256,uint256) [EXTERNAL]
```slithir
_launches_19(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_0', '_launches_1', '_launches_9', '_launches_15', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
 data = _launches[token]
REF_1141(ILaunchpad.LaunchData) -> _launches_22[token_1]
data_1(ILaunchpad.LaunchData) := REF_1141(ILaunchpad.LaunchData)
 currentBaseSold = data.curve.baseSoldFromCurve(token)
REF_1142(IBondingCurveMinimal) -> data_1.curve
TMP_3225(uint256) = HIGH_LEVEL_CALL, dest:REF_1142(IBondingCurveMinimal), function:baseSoldFromCurve, arguments:['token_1']  
currentBaseSold_1(uint256) := TMP_3225(uint256)
 currentBaseSold < amountInBase
TMP_3226(bool) = currentBaseSold_1 < amountInBase_1
CONDITION TMP_3226
 revert InsufficientBaseSold()()
TMP_3227(None) = SOLIDITY_CALL revert InsufficientBaseSold()()
 amountOutQuote = data.curve.sell(token,amountInBase)
REF_1144(IBondingCurveMinimal) -> data_1.curve
TMP_3228(uint256) = HIGH_LEVEL_CALL, dest:REF_1144(IBondingCurveMinimal), function:sell, arguments:['token_1', 'amountInBase_1']  
amountOutQuote_1(uint256) := TMP_3228(uint256)
 amountOutQuote == 0
TMP_3229(bool) = amountOutQuote_1 == 0
CONDITION TMP_3229
 revert DustAttackInvalid()()
TMP_3230(None) = SOLIDITY_CALL revert DustAttackInvalid()()
 amountOutQuote < minAmountOutQuote
TMP_3231(bool) = amountOutQuote_1 < minAmountOutQuote_1
CONDITION TMP_3231
 revert SlippageToleranceExceeded()()
TMP_3232(None) = SOLIDITY_CALL revert SlippageToleranceExceeded()()
 _emitSwapEvent({account:account,token:token,baseAmount:amountInBase,quoteAmount:amountOutQuote,isBuy:false,curve:data.curve})
REF_1146(IBondingCurveMinimal) -> data_1.curve
INTERNAL_CALL, Launchpad._emitSwapEvent(address,address,uint256,uint256,bool,IBondingCurveMinimal)(account_1,token_1,amountInBase_1,amountOutQuote_1,False,REF_1146)
 token.safeTransferFrom(account,address(this),amountInBase)
TMP_3234 = CONVERT this to address
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransferFrom(address,address,address,uint256), arguments:['token_1', 'account_1', 'TMP_3234', 'amountInBase_1'] 
 data.quote.safeTransfer(recipient,amountOutQuote)
REF_1148(address) -> data_1.quote
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransfer(address,address,uint256), arguments:['REF_1148', 'recipient_1', 'amountOutQuote_1'] 
 (amountInBase,amountOutQuote)
RETURN amountInBase_1,amountOutQuote_1
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 onlyBondingActive(token)
MODIFIER_CALL, Launchpad.onlyBondingActive(address)(token_1)
_launches_21(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_30'])
 onlySenderOrOperator(account,SpotOperatorRoles.LAUNCHPAD_FILL)
REF_1150(SpotOperatorRoles) -> SpotOperatorRoles.LAUNCHPAD_FILL
MODIFIER_CALL, Launchpad.onlySenderOrOperator(address,SpotOperatorRoles)(account_1,REF_1150)
 (amountInBaseActual,amountOutQuoteActual)
```
#### LaunchToken.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 _PERMIT2 = 0x000000000022D473030F116dDEE9F6B43aC78BA3
 ABI_VERSION = 1
launchpad_16(address) := phi(['launchpad_6', 'launchpad_0', 'launchpad_13', 'launchpad_1', 'launchpad_15', 'launchpad_4', 'launchpad_9'])
 msg.sender != launchpad
TMP_3046(bool) = msg.sender != launchpad_16
CONDITION TMP_3046
 revert BadAuth()()
TMP_3047(None) = SOLIDITY_CALL revert BadAuth()()
```
#### Launchpad.sortTokens(address,address) [INTERNAL]
```slithir
tokenA_1(address) := phi(['tokenA_1'])
tokenB_1(address) := phi(['tokenB_1'])
 tokenA == tokenB
TMP_3342(bool) = tokenA_1 == tokenB_1
CONDITION TMP_3342
 revert(string)(UniswapV2Library: IDENTICAL_ADDRESSES)
TMP_3343(None) = SOLIDITY_CALL revert(string)(UniswapV2Library: IDENTICAL_ADDRESSES)
 token0 == address(0)
TMP_3344 = CONVERT 0 to address
TMP_3345(bool) = token0_3 == TMP_3344
CONDITION TMP_3345
 revert(string)(UniswapV2Library: ZERO_ADDRESS)
TMP_3346(None) = SOLIDITY_CALL revert(string)(UniswapV2Library: ZERO_ADDRESS)
 tokenA < tokenB
TMP_3347(bool) = tokenA_1 < tokenB_1
CONDITION TMP_3347
 (token0,token1) = (tokenA,tokenB)
token0_1(address) := tokenA_1(address)
token1_1(address) := tokenB_1(address)
 (token0,token1) = (tokenB,tokenA)
token0_2(address) := tokenB_1(address)
token1_2(address) := tokenA_1(address)
token0_3(address) := phi(['token0_1', 'token0_2'])
token1_3(address) := phi(['token1_1', 'token1_2'])
 (token0,token1)
RETURN token0_3,token1_3
```
#### Launchpad.updateBondingCurve(address) [EXTERNAL][OWNER]
```slithir
currentBondingCurve_5(IBondingCurveMinimal) := phi(['currentBondingCurve_4', 'currentBondingCurve_2', 'currentBondingCurve_7', 'currentBondingCurve_0'])
 ! ERC165Checker.supportsInterface(newBondingCurve,type()(IBondingCurveMinimal).interfaceId)
TMP_3240(type(IBondingCurveMinimal)) = SOLIDITY_CALL type()(IBondingCurveMinimal)
REF_1152(bytes4) (->None) := 1992782646(bytes4)
TMP_3241(bool) = LIBRARY_CALL, dest:ERC165Checker, function:ERC165Checker.supportsInterface(address,bytes4), arguments:['newBondingCurve_1', 'REF_1152'] 
TMP_3242 = UnaryType.BANG TMP_3241 
CONDITION TMP_3242
 revert InvalidCurve()()
TMP_3243(None) = SOLIDITY_CALL revert InvalidCurve()()
 BondingCurveUpdated(address(currentBondingCurve),newBondingCurve,EventNonceLib.inc())
TMP_3244 = CONVERT currentBondingCurve_6 to address
TMP_3245(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit BondingCurveUpdated(TMP_3244,newBondingCurve_1,TMP_3245)
 currentBondingCurve = IBondingCurveMinimal(newBondingCurve)
TMP_3247 = CONVERT newBondingCurve_1 to IBondingCurveMinimal
currentBondingCurve_7(IBondingCurveMinimal) := TMP_3247(IBondingCurveMinimal)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### Launchpad.updateInitCodeHash(bytes) [EXTERNAL][OWNER]
```slithir
 uniV2InitCodeHash = newHash
uniV2InitCodeHash_1(bytes) := newHash_1(bytes)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### Launchpad.updateLaunchFee(uint256) [EXTERNAL][OWNER]
```slithir
 launchFee = newLaunchFee
launchFee_3(uint256) := newLaunchFee_1(uint256)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### Launchpad.updateLaunchpadLPVault(address) [EXTERNAL][OWNER]
```slithir
 launchpadLPVault = LaunchpadLPVault(newLaunchpadLPVault)
TMP_3273 = CONVERT newLaunchpadLPVault_1 to LaunchpadLPVault
launchpadLPVault_2(LaunchpadLPVault) := TMP_3273(LaunchpadLPVault)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### Launchpad.updateQuoteAsset(address) [EXTERNAL][OWNER]
```slithir
currentQuoteAsset_4(LaunchToken) := phi(['currentQuoteAsset_8', 'currentQuoteAsset_0', 'currentQuoteAsset_3', 'currentQuoteAsset_1'])
_launches_23(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_0', '_launches_1', '_launches_9', '_launches_15', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
 newQuoteAsset == address(0) || _launches[newQuoteAsset].quote != address(0)
TMP_3249 = CONVERT 0 to address
TMP_3250(bool) = newQuoteAsset_1 == TMP_3249
REF_1154(ILaunchpad.LaunchData) -> _launches_24[newQuoteAsset_1]
REF_1155(address) -> REF_1154.quote
TMP_3251 = CONVERT 0 to address
TMP_3252(bool) = REF_1155 != TMP_3251
TMP_3253(bool) = TMP_3250 || TMP_3252
CONDITION TMP_3253
 revert InvalidQuoteAsset()()
TMP_3254(None) = SOLIDITY_CALL revert InvalidQuoteAsset()()
 LaunchToken(newQuoteAsset).approve(address(this),0)
TMP_3255 = CONVERT newQuoteAsset_1 to LaunchToken
TMP_3256 = CONVERT this to address
TMP_3257(bool) = HIGH_LEVEL_CALL, dest:TMP_3255(LaunchToken), function:approve, arguments:['TMP_3256', '0']  
currentQuoteAsset_6(LaunchToken) := phi(['currentQuoteAsset_8', 'currentQuoteAsset_5', 'currentQuoteAsset_3', 'currentQuoteAsset_1'])
 QuoteAssetUpdated(address(currentQuoteAsset),newQuoteAsset,LaunchToken(newQuoteAsset).decimals(),EventNonceLib.inc())
TMP_3258 = CONVERT currentQuoteAsset_6 to address
TMP_3259 = CONVERT newQuoteAsset_1 to LaunchToken
TMP_3260(uint8) = HIGH_LEVEL_CALL, dest:TMP_3259(LaunchToken), function:decimals, arguments:[]  
currentQuoteAsset_7(LaunchToken) := phi(['currentQuoteAsset_8', 'currentQuoteAsset_3', 'currentQuoteAsset_6', 'currentQuoteAsset_1'])
TMP_3261(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit QuoteAssetUpdated(TMP_3258,newQuoteAsset_1,TMP_3260,TMP_3261)
 currentQuoteAsset = LaunchToken(newQuoteAsset)
TMP_3263 = CONVERT newQuoteAsset_1 to LaunchToken
currentQuoteAsset_8(LaunchToken) := TMP_3263(LaunchToken)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### IBondingCurveMinimal.baseSoldFromCurve(address) [EXTERNAL]
```slithir

```
#### IBondingCurveMinimal.bondingSupply(address) [EXTERNAL]
```slithir

```
#### IUniswapV2FactoryMinimal.createPair(address,address) [EXTERNAL]
```slithir

```
#### IUniswapV2Pair.skim(address) [EXTERNAL]
```slithir

```
#### IUniswapV2RouterMinimal.addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256) [EXTERNAL]
```slithir

```

#### SafeTransferLib.safeApprove(address,address,uint256) [INTERNAL]
```slithir
 mstore(uint256,uint256)(0x14,to)
TMP_15369(None) = SOLIDITY_CALL mstore(uint256,uint256)(20,to_1)
 mstore(uint256,uint256)(0x34,amount)
TMP_15370(None) = SOLIDITY_CALL mstore(uint256,uint256)(52,amount_1)
 mstore(uint256,uint256)(0x00,0x095ea7b3000000000000000000000000)
TMP_15371(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,12454529211011416535493358632801665024)
 success_safeApprove_asm_0 = call(uint256,uint256,uint256,uint256,uint256,uint256,uint256)(gas()(),token,0,0x10,0x44,0x00,0x20)
TMP_15372(uint256) = SOLIDITY_CALL gas()()
TMP_15373(uint256) = SOLIDITY_CALL call(uint256,uint256,uint256,uint256,uint256,uint256,uint256)(TMP_15372,token_1,0,16,68,0,32)
success_safeApprove_asm_0_1(uint256) := TMP_15373(uint256)
 ! mload(uint256)(0x00) == 1 & success_safeApprove_asm_0
TMP_15374(uint256) = SOLIDITY_CALL mload(uint256)(0)
TMP_15375(bool) = TMP_15374 == 1
TMP_15376(bool) = TMP_15375 & success_safeApprove_asm_0_1
TMP_15377 = UnaryType.BANG TMP_15376 
CONDITION TMP_15377
 ! ! extcodesize(uint256)(token) | returndatasize()() < success_safeApprove_asm_0
REF_5788 -> CODESIZE token_1
TMP_15378 = UnaryType.BANG REF_5788 
TMP_15379(uint256) = SOLIDITY_CALL returndatasize()(token_1)
TMP_15380(uint256) = TMP_15378 | TMP_15379
TMP_15381(bool) = TMP_15380 < success_safeApprove_asm_0_1
TMP_15382 = UnaryType.BANG TMP_15381 
CONDITION TMP_15382
 mstore(uint256,uint256)(0x00,0x3e3f8f73)
TMP_15383(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,1044352883)
 revert(uint256,uint256)(0x1c,0x04)
TMP_15384(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 mstore(uint256,uint256)(0x34,0)
TMP_15385(None) = SOLIDITY_CALL mstore(uint256,uint256)(52,0)
```
#### IBondingCurveMinimal.quoteBoughtByCurve(address) [EXTERNAL]
```slithir

```
#### IBondingCurveMinimal.totalSupply(address) [EXTERNAL]
```slithir

```
#### EventNonceLib.inc() [INTERNAL]
```slithir
 ds = getEventNonceStorage()
TMP_9860(EventNonceStorage) = INTERNAL_CALL, EventNonceLib.getEventNonceStorage()()
ds_1 (-> ['TMP_9860'])(EventNonceStorage) := TMP_9860(EventNonceStorage)
 ++ ds.eventNonce
REF_5582(uint256) -> ds_1 (-> ['TMP_9860']).eventNonce
ds_2 (-> ['TMP_9860'])(EventNonceStorage) := phi(["ds_1 (-> ['TMP_9860'])"])
REF_5582(-> ds_2 (-> ['TMP_9860'])) = REF_5582 (c)+ 1
RETURN REF_5582
TMP_9860(EventNonceStorage) := phi(["ds_2 (-> ['TMP_9860'])"])
```
#### LaunchToken.unlock() [EXTERNAL]
```slithir
 unlocked = true
unlocked_1(bool) := True(bool)
 TransfersUnlocked(block.timestamp,_incEventNonce())
TMP_2983(uint256) = INTERNAL_CALL, LaunchToken._incEventNonce()()
Emit TransfersUnlocked(block.timestamp,TMP_2983)
 onlyLaunchpad()
MODIFIER_CALL, LaunchToken.onlyLaunchpad()()
```
#### IUniswapV2RouterMinimal.swapTokensForExactTokens(uint256,uint256,address[],address,uint256) [EXTERNAL]
```slithir

```
#### SafeTransferLib.safeTransfer(address,address,uint256) [INTERNAL]
```slithir
 mstore(uint256,uint256)(0x14,to)
TMP_15324(None) = SOLIDITY_CALL mstore(uint256,uint256)(20,to_1)
 mstore(uint256,uint256)(0x34,amount)
TMP_15325(None) = SOLIDITY_CALL mstore(uint256,uint256)(52,amount_1)
 mstore(uint256,uint256)(0x00,0xa9059cbb000000000000000000000000)
TMP_15326(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,224668671643508016486903311432943665152)
 success_safeTransfer_asm_0 = call(uint256,uint256,uint256,uint256,uint256,uint256,uint256)(gas()(),token,0,0x10,0x44,0x00,0x20)
TMP_15327(uint256) = SOLIDITY_CALL gas()()
TMP_15328(uint256) = SOLIDITY_CALL call(uint256,uint256,uint256,uint256,uint256,uint256,uint256)(TMP_15327,token_1,0,16,68,0,32)
success_safeTransfer_asm_0_1(uint256) := TMP_15328(uint256)
 ! mload(uint256)(0x00) == 1 & success_safeTransfer_asm_0
TMP_15329(uint256) = SOLIDITY_CALL mload(uint256)(0)
TMP_15330(bool) = TMP_15329 == 1
TMP_15331(bool) = TMP_15330 & success_safeTransfer_asm_0_1
TMP_15332 = UnaryType.BANG TMP_15331 
CONDITION TMP_15332
 ! ! extcodesize(uint256)(token) | returndatasize()() < success_safeTransfer_asm_0
REF_5786 -> CODESIZE token_1
TMP_15333 = UnaryType.BANG REF_5786 
TMP_15334(uint256) = SOLIDITY_CALL returndatasize()(token_1)
TMP_15335(uint256) = TMP_15333 | TMP_15334
TMP_15336(bool) = TMP_15335 < success_safeTransfer_asm_0_1
TMP_15337 = UnaryType.BANG TMP_15336 
CONDITION TMP_15337
 mstore(uint256,uint256)(0x00,0x90b8ec18)
TMP_15338(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,2428038168)
 revert(uint256,uint256)(0x1c,0x04)
TMP_15339(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 mstore(uint256,uint256)(0x34,0)
TMP_15340(None) = SOLIDITY_CALL mstore(uint256,uint256)(52,0)
```
#### SafeTransferLib.safeTransferFrom(address,address,address,uint256) [INTERNAL]
```slithir
 m_safeTransferFrom_asm_0 = mload(uint256)(0x40)
TMP_15255(uint256) = SOLIDITY_CALL mload(uint256)(64)
m_safeTransferFrom_asm_0_1(uint256) := TMP_15255(uint256)
 mstore(uint256,uint256)(0x60,amount)
TMP_15256(None) = SOLIDITY_CALL mstore(uint256,uint256)(96,amount_1)
 mstore(uint256,uint256)(0x40,to)
TMP_15257(None) = SOLIDITY_CALL mstore(uint256,uint256)(64,to_1)
 mstore(uint256,uint256)(0x2c,from << 96)
TMP_15258(address) = from_1 << 96
TMP_15259(None) = SOLIDITY_CALL mstore(uint256,uint256)(44,TMP_15258)
 mstore(uint256,uint256)(0x0c,0x23b872dd000000000000000000000000)
TMP_15260(None) = SOLIDITY_CALL mstore(uint256,uint256)(12,47480692178561195778129796594248187904)
 success_safeTransferFrom_asm_0 = call(uint256,uint256,uint256,uint256,uint256,uint256,uint256)(gas()(),token,0,0x1c,0x64,0x00,0x20)
TMP_15261(uint256) = SOLIDITY_CALL gas()()
TMP_15262(uint256) = SOLIDITY_CALL call(uint256,uint256,uint256,uint256,uint256,uint256,uint256)(TMP_15261,token_1,0,28,100,0,32)
success_safeTransferFrom_asm_0_1(uint256) := TMP_15262(uint256)
 ! mload(uint256)(0x00) == 1 & success_safeTransferFrom_asm_0
TMP_15263(uint256) = SOLIDITY_CALL mload(uint256)(0)
TMP_15264(bool) = TMP_15263 == 1
TMP_15265(bool) = TMP_15264 & success_safeTransferFrom_asm_0_1
TMP_15266 = UnaryType.BANG TMP_15265 
CONDITION TMP_15266
 ! ! extcodesize(uint256)(token) | returndatasize()() < success_safeTransferFrom_asm_0
REF_5783 -> CODESIZE token_1
TMP_15267 = UnaryType.BANG REF_5783 
TMP_15268(uint256) = SOLIDITY_CALL returndatasize()(token_1)
TMP_15269(uint256) = TMP_15267 | TMP_15268
TMP_15270(bool) = TMP_15269 < success_safeTransferFrom_asm_0_1
TMP_15271 = UnaryType.BANG TMP_15270 
CONDITION TMP_15271
 mstore(uint256,uint256)(0x00,0x7939f424)
TMP_15272(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,2033841188)
 revert(uint256,uint256)(0x1c,0x04)
TMP_15273(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 mstore(uint256,uint256)(0x60,0)
TMP_15274(None) = SOLIDITY_CALL mstore(uint256,uint256)(96,0)
 mstore(uint256,uint256)(0x40,m_safeTransferFrom_asm_0)
TMP_15275(None) = SOLIDITY_CALL mstore(uint256,uint256)(64,m_safeTransferFrom_asm_0_1)
```
#### IBondingCurveMinimal.buy(address,uint256) [EXTERNAL]
```slithir

```
#### IUniswapV2RouterMinimal.factory() [EXTERNAL]
```slithir

```
#### IDistributor.decreaseStake(address,address,uint96) [EXTERNAL]
```slithir

```

#### EventNonceLib.getCurrentNonce() [INTERNAL]
```slithir
 ds = getEventNonceStorage()
TMP_9861(EventNonceStorage) = INTERNAL_CALL, EventNonceLib.getEventNonceStorage()()
ds_1 (-> ['TMP_9861'])(EventNonceStorage) := TMP_9861(EventNonceStorage)
 ds.eventNonce
REF_5583(uint256) -> ds_1 (-> ['TMP_9861']).eventNonce
RETURN REF_5583
```
#### IDistributor.increaseStake(address,address,uint96) [EXTERNAL]
```slithir

```
#### ERC165Checker.supportsInterface(address,bytes4) [INTERNAL]
```slithir
 supportsERC165(account) && supportsERC165InterfaceUnchecked(account,interfaceId)
TMP_10064(bool) = INTERNAL_CALL, ERC165Checker.supportsERC165(address)(account_1)
TMP_10065(bool) = INTERNAL_CALL, ERC165Checker.supportsERC165InterfaceUnchecked(address,bytes4)(account_1,interfaceId_1)
TMP_10066(bool) = TMP_10064 && TMP_10065
RETURN TMP_10066
```
#### IBondingCurveMinimal.init(bytes) [EXTERNAL]
```slithir

```
#### LaunchToken.mint(uint256) [EXTERNAL]
```slithir
launchpad_2(address) := phi(['launchpad_6', 'launchpad_0', 'launchpad_13', 'launchpad_1', 'launchpad_15', 'launchpad_4', 'launchpad_9'])
 _mint(launchpad,amount)
INTERNAL_CALL, ERC20._mint(address,uint256)(launchpad_3,amount_1)
 totalSupply() > type()(uint96).max
TMP_2987(uint256) = INTERNAL_CALL, ERC20.totalSupply()()
TMP_2989(uint96) := 79228162514264337593543950335(uint96)
TMP_2990(bool) = TMP_2987 > TMP_2989
CONDITION TMP_2990
 revert TotalSupplyExceedsMaxShares()()
TMP_2991(None) = SOLIDITY_CALL revert TotalSupplyExceedsMaxShares()()
 onlyLaunchpad()
MODIFIER_CALL, LaunchToken.onlyLaunchpad()()
```
#### IDistributor.createRewardsPair(address,address) [EXTERNAL]
```slithir

```
#### IBondingCurveMinimal.initializeCurve(address,uint256,uint256) [EXTERNAL]
```slithir

```
#### IBondingCurveMinimal.quoteBaseForQuote(address,uint256,bool) [EXTERNAL]
```slithir

```
#### IBondingCurveMinimal.quoteQuoteForBase(address,uint256,bool) [EXTERNAL]
```slithir

```
#### IBondingCurveMinimal.sell(address,uint256) [EXTERNAL]
```slithir

```
#### EventNonceLib.getEventNonceStorage() [INTERNAL]
```slithir
EVENT_NONCE_STORAGE_POSITION_1(bytes32) := phi(['EVENT_NONCE_STORAGE_POSITION_0'])
 position = EVENT_NONCE_STORAGE_POSITION
position_1(bytes32) := EVENT_NONCE_STORAGE_POSITION_1(bytes32)
 ds = position
ds_1 (-> ['position'])(EventNonceStorage) := position_1(bytes32)
 ds
RETURN ds_1 (-> ['position'])
```
#### LaunchToken._incEventNonce() [INTERNAL]
```slithir
eventNonce_1(uint256) := phi(['eventNonce_2', 'eventNonce_0'])
 nonce = eventNonce
nonce_1(uint256) := eventNonce_1(uint256)
 eventNonce ++
TMP_3045(uint256) := eventNonce_1(uint256)
eventNonce_2(uint256) = eventNonce_1 (c)+ 1
 nonce
RETURN nonce_1
```
#### ERC165Checker.supportsERC165(address) [INTERNAL]
```slithir
account_1(address) := phi(['account_1', 'account_1', 'account_1'])
INTERFACE_ID_INVALID_1(bytes4) := phi(['INTERFACE_ID_INVALID_0', 'INTERFACE_ID_INVALID_3'])
 supportsERC165InterfaceUnchecked(account,type()(IERC165).interfaceId) && ! supportsERC165InterfaceUnchecked(account,INTERFACE_ID_INVALID)
TMP_10059(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_5653(bytes4) (->None) := 33540519(bytes4)
TMP_10060(bool) = INTERNAL_CALL, ERC165Checker.supportsERC165InterfaceUnchecked(address,bytes4)(account_1,REF_5653)
TMP_10061(bool) = INTERNAL_CALL, ERC165Checker.supportsERC165InterfaceUnchecked(address,bytes4)(account_1,INTERFACE_ID_INVALID_2)
TMP_10062 = UnaryType.BANG TMP_10061 
TMP_10063(bool) = TMP_10060 && TMP_10062
RETURN TMP_10063
```
#### ERC165Checker.supportsERC165InterfaceUnchecked(address,bytes4) [INTERNAL]
```slithir
account_1(address) := phi(['account_1', 'account_1', 'account_1', 'account_1'])
interfaceId_1(bytes4) := phi(['REF_5653', 'INTERFACE_ID_INVALID_2', 'interfaceId_1', 'REF_5659', 'REF_5657'])
 encodedParams = abi.encodeCall(IERC165.supportsInterface,(interfaceId))
REF_5661(supportsInterface) -> IERC165.supportsInterface
TMP_10079(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_5661,interfaceId_1)
encodedParams_1(bytes) := TMP_10079(bytes)
 success = staticcall(uint256,uint256,uint256,uint256,uint256,uint256)(30000,account,encodedParams + 0x20,mload(uint256)(encodedParams),0x00,0x20)
TMP_10080(bytes) = encodedParams_1 + 32
TMP_10081(uint256) = SOLIDITY_CALL mload(uint256)(encodedParams_1)
TMP_10082(uint256) = SOLIDITY_CALL staticcall(uint256,uint256,uint256,uint256,uint256,uint256)(30000,account_1,TMP_10080,TMP_10081,0,32)
success_1(bool) := TMP_10082(uint256)
 returnSize = returndatasize()()
TMP_10083(uint256) = SOLIDITY_CALL returndatasize()()
returnSize_1(uint256) := TMP_10083(uint256)
 returnValue = mload(uint256)(0x00)
TMP_10084(uint256) = SOLIDITY_CALL mload(uint256)(0)
returnValue_1(uint256) := TMP_10084(uint256)
 success && returnSize >= 0x20 && returnValue > 0
TMP_10085(bool) = returnSize_1 >= 32
TMP_10086(bool) = success_1 && TMP_10085
TMP_10087(bool) = returnValue_1 > 0
TMP_10088(bool) = TMP_10086 && TMP_10087
RETURN TMP_10088
```
