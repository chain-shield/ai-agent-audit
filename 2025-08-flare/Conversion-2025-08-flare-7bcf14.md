





#### Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256) [INTERNAL]
```slithir
_tokenDecimals_1(uint256) := phi(['REF_3165', 'REF_3167'])
_tokenPrice_1(uint256) := phi(['tokenPrice_1'])
_tokenFtsoDecimals_1(uint256) := phi(['tokenFtsoDec_1'])
_assetPrice_1(uint256) := phi(['assetPrice_1'])
_assetFtsoDecimals_1(uint256) := phi(['assetFtsoDec_1'])
AMG_TOKEN_WEI_PRICE_SCALE_EXP_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_EXP_0'])
 settings = Globals.getSettings()
TMP_4665(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4665'])(AssetManagerSettings.Data) := TMP_4665(AssetManagerSettings.Data)
 expPlus = _tokenDecimals + _tokenFtsoDecimals + AMG_TOKEN_WEI_PRICE_SCALE_EXP
TMP_4666(uint256) = _tokenDecimals_1 (c)+ _tokenFtsoDecimals_1
TMP_4667(uint256) = TMP_4666 (c)+ AMG_TOKEN_WEI_PRICE_SCALE_EXP_1
expPlus_1(uint256) := TMP_4667(uint256)
 expMinus = settings.assetMintingDecimals + _assetFtsoDecimals
REF_3173(uint8) -> settings_1 (-> ['TMP_4665']).assetMintingDecimals
TMP_4668(uint8) = REF_3173 (c)+ _assetFtsoDecimals_1
expMinus_1(uint256) := TMP_4668(uint8)
 assert(bool)(expPlus >= expMinus)
TMP_4669(bool) = expPlus_1 >= expMinus_1
TMP_4670(None) = SOLIDITY_CALL assert(bool)(TMP_4669)
 _assetPrice.mulDiv(10 ** (expPlus - expMinus),_tokenPrice)
TMP_4671(uint256) = expPlus_1 (c)- expMinus_1
TMP_4672(uint256) = 10 (c)** TMP_4671
TMP_4673(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_assetPrice_1', 'TMP_4672', '_tokenPrice_1'] 
RETURN TMP_4673
```
#### Conversion.convert(uint256,CollateralTypeInt.Data,CollateralTypeInt.Data) [INTERNAL]
```slithir
 priceMul = currentAmgPriceInTokenWei(_toToken)
TMP_4652(uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data)(_toToken_1 (-> []))
priceMul_1(uint256) := TMP_4652(uint256)
 priceDiv = currentAmgPriceInTokenWei(_fromToken)
TMP_4653(uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data)(_fromToken_1 (-> []))
priceDiv_1(uint256) := TMP_4653(uint256)
 _amount.mulDiv(priceMul,priceDiv)
TMP_4654(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_amount_1', 'priceMul_1', 'priceDiv_1'] 
RETURN TMP_4654
```
#### Conversion.convertAmgToTokenWei(uint256,uint256) [INTERNAL]
```slithir
AMG_TOKEN_WEI_PRICE_SCALE_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_0'])
 _valueAMG.mulDiv(_amgToTokenWeiPrice,AMG_TOKEN_WEI_PRICE_SCALE)
TMP_4674(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_valueAMG_1', '_amgToTokenWeiPrice_1', 'AMG_TOKEN_WEI_PRICE_SCALE_1'] 
RETURN TMP_4674
```
#### Conversion.convertAmgToUBA(uint64) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4637(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4637'])(AssetManagerSettings.Data) := TMP_4637(AssetManagerSettings.Data)
 uint256(_valueAMG) * settings.assetMintingGranularityUBA
TMP_4638 = CONVERT _valueAMG_1 to uint256
REF_3145(uint64) -> settings_1 (-> ['TMP_4637']).assetMintingGranularityUBA
TMP_4639(uint256) = TMP_4638 (c)* REF_3145
RETURN TMP_4639
```
#### Conversion.convertFromUSD5(uint256,CollateralTypeInt.Data) [INTERNAL]
```slithir
 bytes(_token.tokenFtsoSymbol).length == 0
REF_3158(string) -> _token_1 (-> []).tokenFtsoSymbol
TMP_4655 = CONVERT REF_3158 to bytes
REF_3159 -> LENGTH TMP_4655
TMP_4656(bool) = REF_3159 == 0
CONDITION TMP_4656
 _amountUSD5
RETURN _amountUSD5_1
 (tokenPrice,None,tokenFtsoDec) = readFtsoPrice(_token.tokenFtsoSymbol,false)
REF_3160(string) -> _token_1 (-> []).tokenFtsoSymbol
TUPLE_47(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.readFtsoPrice(string,bool)(REF_3160,False)
tokenPrice_1(uint256)= UNPACK TUPLE_47 index: 0 
tokenFtsoDec_1(uint256)= UNPACK TUPLE_47 index: 2 
 expPlus = _token.decimals + tokenFtsoDec - 5
REF_3161(uint8) -> _token_1 (-> []).decimals
TMP_4657(uint8) = REF_3161 (c)+ tokenFtsoDec_1
TMP_4658(uint8) = TMP_4657 (c)- 5
expPlus_1(uint256) := TMP_4658(uint8)
 _amountUSD5.mulDiv(10 ** expPlus,tokenPrice)
TMP_4659(uint256) = 10 (c)** expPlus_1
TMP_4660(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_amountUSD5_1', 'TMP_4659', 'tokenPrice_1'] 
RETURN TMP_4660
```
#### Conversion.convertLotsToAMG(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4646(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4646'])(AssetManagerSettings.Data) := TMP_4646(AssetManagerSettings.Data)
 SafeCast.toUint64(_lots * settings.lotSizeAMG)
REF_3153(uint64) -> settings_1 (-> ['TMP_4646']).lotSizeAMG
TMP_4647(uint256) = _lots_1 (c)* REF_3153
TMP_4648(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_4647'] 
RETURN TMP_4648
```
#### Conversion.convertLotsToUBA(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4649(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4649'])(AssetManagerSettings.Data) := TMP_4649(AssetManagerSettings.Data)
 _lots * settings.lotSizeAMG * settings.assetMintingGranularityUBA
REF_3155(uint64) -> settings_1 (-> ['TMP_4649']).lotSizeAMG
TMP_4650(uint256) = _lots_1 (c)* REF_3155
REF_3156(uint64) -> settings_1 (-> ['TMP_4649']).assetMintingGranularityUBA
TMP_4651(uint256) = TMP_4650 (c)* REF_3156
RETURN TMP_4651
```
#### Conversion.convertTokenWeiToAMG(uint256,uint256) [INTERNAL]
```slithir
AMG_TOKEN_WEI_PRICE_SCALE_2(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_0'])
 _valueNATWei.mulDiv(AMG_TOKEN_WEI_PRICE_SCALE,_amgToTokenWeiPrice)
TMP_4675(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_valueNATWei_1', 'AMG_TOKEN_WEI_PRICE_SCALE_2', '_amgToTokenWeiPrice_1'] 
RETURN TMP_4675
```
#### Conversion.convertUBAToAmg(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4640(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4640'])(AssetManagerSettings.Data) := TMP_4640(AssetManagerSettings.Data)
 SafeCast.toUint64(_valueUBA / settings.assetMintingGranularityUBA)
REF_3148(uint64) -> settings_1 (-> ['TMP_4640']).assetMintingGranularityUBA
TMP_4641(uint256) = _valueUBA_1 (c)/ REF_3148
TMP_4642(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_4641'] 
RETURN TMP_4642
```
#### Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data) [INTERNAL]
```slithir
_token_1 (-> [])(CollateralTypeInt.Data) := phi(['_toToken_1 (-> [])', '_fromToken_1 (-> [])'])
 (_price,None,None) = currentAmgPriceInTokenWeiWithTs(_token,false)
TUPLE_44(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool)(_token_1 (-> []),False)
_price_1(uint256)= UNPACK TUPLE_44 index: 0 
 _price
RETURN _price_1
```
#### Conversion.currentAmgPriceInTokenWeiWithTrusted(CollateralTypeInt.Data) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4631(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4631'])(AssetManagerSettings.Data) := TMP_4631(AssetManagerSettings.Data)
 (ftsoPrice,assetTimestamp,tokenTimestamp) = currentAmgPriceInTokenWeiWithTs(_token,false)
TUPLE_45(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool)(_token_1 (-> []),False)
ftsoPrice_1(uint256)= UNPACK TUPLE_45 index: 0 
assetTimestamp_1(uint256)= UNPACK TUPLE_45 index: 1 
tokenTimestamp_1(uint256)= UNPACK TUPLE_45 index: 2 
 (trustedPrice,assetTimestampTrusted,tokenTimestampTrusted) = currentAmgPriceInTokenWeiWithTs(_token,true)
TUPLE_46(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool)(_token_1 (-> []),True)
trustedPrice_1(uint256)= UNPACK TUPLE_46 index: 0 
assetTimestampTrusted_1(uint256)= UNPACK TUPLE_46 index: 1 
tokenTimestampTrusted_1(uint256)= UNPACK TUPLE_46 index: 2 
 trustedPriceFresh = tokenTimestampTrusted + settings.maxTrustedPriceAgeSeconds >= tokenTimestamp && assetTimestampTrusted + settings.maxTrustedPriceAgeSeconds >= assetTimestamp
REF_3142(uint64) -> settings_1 (-> ['TMP_4631']).maxTrustedPriceAgeSeconds
TMP_4632(uint256) = tokenTimestampTrusted_1 (c)+ REF_3142
TMP_4633(bool) = TMP_4632 >= tokenTimestamp_1
REF_3143(uint64) -> settings_1 (-> ['TMP_4631']).maxTrustedPriceAgeSeconds
TMP_4634(uint256) = assetTimestampTrusted_1 (c)+ REF_3143
TMP_4635(bool) = TMP_4634 >= assetTimestamp_1
TMP_4636(bool) = TMP_4633 && TMP_4635
trustedPriceFresh_1(bool) := TMP_4636(bool)
 _ftsoPrice = ftsoPrice
_ftsoPrice_1(uint256) := ftsoPrice_1(uint256)
 trustedPriceFresh
CONDITION trustedPriceFresh_1
 _trustedPrice = trustedPrice
_trustedPrice_1(uint256) := trustedPrice_1(uint256)
 _trustedPrice = ftsoPrice
_trustedPrice_2(uint256) := ftsoPrice_1(uint256)
_trustedPrice_3(uint256) := phi(['_trustedPrice_1', '_trustedPrice_2'])
 (_ftsoPrice,_trustedPrice)
RETURN _ftsoPrice_1,_trustedPrice_3
```
#### Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool) [INTERNAL]
```slithir
_token_1 (-> [])(CollateralTypeInt.Data) := phi(['_token_1 (-> [])', 'REF_3140', '_token_1 (-> [])'])
 (assetPrice,assetTs,assetFtsoDec) = readFtsoPrice(_token.assetFtsoSymbol,_fromTrustedProviders)
REF_3163(string) -> _token_1 (-> []).assetFtsoSymbol
TUPLE_48(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.readFtsoPrice(string,bool)(REF_3163,_fromTrustedProviders_1)
assetPrice_1(uint256)= UNPACK TUPLE_48 index: 0 
assetTs_1(uint256)= UNPACK TUPLE_48 index: 1 
assetFtsoDec_1(uint256)= UNPACK TUPLE_48 index: 2 
 _token.directPricePair
REF_3164(bool) -> _token_1 (-> []).directPricePair
CONDITION REF_3164
 price = calcAmgToTokenWeiPrice(_token.decimals,1,0,assetPrice,assetFtsoDec)
REF_3165(uint8) -> _token_1 (-> []).decimals
TMP_4661(uint256) = INTERNAL_CALL, Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256)(REF_3165,1,0,assetPrice_1,assetFtsoDec_1)
price_1(uint256) := TMP_4661(uint256)
 (price,assetTs,assetTs)
RETURN price_1,assetTs_1,assetTs_1
 (tokenPrice,tokenTs,tokenFtsoDec) = readFtsoPrice(_token.tokenFtsoSymbol,_fromTrustedProviders)
REF_3166(string) -> _token_1 (-> []).tokenFtsoSymbol
TUPLE_49(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.readFtsoPrice(string,bool)(REF_3166,_fromTrustedProviders_1)
tokenPrice_1(uint256)= UNPACK TUPLE_49 index: 0 
tokenTs_1(uint256)= UNPACK TUPLE_49 index: 1 
tokenFtsoDec_1(uint256)= UNPACK TUPLE_49 index: 2 
 price_scope_0 = calcAmgToTokenWeiPrice(_token.decimals,tokenPrice,tokenFtsoDec,assetPrice,assetFtsoDec)
REF_3167(uint8) -> _token_1 (-> []).decimals
TMP_4662(uint256) = INTERNAL_CALL, Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256)(REF_3167,tokenPrice_1,tokenFtsoDec_1,assetPrice_1,assetFtsoDec_1)
price_scope_0_1(uint256) := TMP_4662(uint256)
 (price_scope_0,assetTs,tokenTs)
RETURN price_scope_0_1,assetTs_1,tokenTs_1
```
#### Conversion.readFtsoPrice(string,bool) [INTERNAL]
```slithir
_symbol_1(string) := phi(['REF_3163', 'REF_3166', 'REF_3160'])
_fromTrustedProviders_1(bool) := phi(['_fromTrustedProviders_1'])
 settings = Globals.getSettings()
TMP_4663(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4663'])(AssetManagerSettings.Data) := TMP_4663(AssetManagerSettings.Data)
 priceReader = IPriceReader(settings.priceReader)
REF_3169(address) -> settings_1 (-> ['TMP_4663']).priceReader
TMP_4664 = CONVERT REF_3169 to IPriceReader
priceReader_1(IPriceReader) := TMP_4664(IPriceReader)
 _fromTrustedProviders
CONDITION _fromTrustedProviders_1
 priceReader.getPriceFromTrustedProviders(_symbol)
TUPLE_50(uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:priceReader_1(IPriceReader), function:getPriceFromTrustedProviders, arguments:['_symbol_1']  
RETURN TUPLE_50
 priceReader.getPrice(_symbol)
TUPLE_51(uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:priceReader_1(IPriceReader), function:getPrice, arguments:['_symbol_1']  
RETURN TUPLE_51
```
#### Conversion.roundUBAToAmg(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4643(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4643'])(AssetManagerSettings.Data) := TMP_4643(AssetManagerSettings.Data)
 _valueUBA - (_valueUBA % settings.assetMintingGranularityUBA)
REF_3150(uint64) -> settings_1 (-> ['TMP_4643']).assetMintingGranularityUBA
TMP_4644(uint256) = _valueUBA_1 % REF_3150
TMP_4645(uint256) = _valueUBA_1 (c)- TMP_4644
RETURN TMP_4645
```

#### Globals.getSettings() [INTERNAL]
```slithir
ASSET_MANAGER_SETTINGS_POSITION_1(bytes32) := phi(['ASSET_MANAGER_SETTINGS_POSITION_0'])
 position = ASSET_MANAGER_SETTINGS_POSITION
position_1(bytes32) := ASSET_MANAGER_SETTINGS_POSITION_1(bytes32)
 _settings = position
_settings_1 (-> ['position'])(AssetManagerSettings.Data) := position_1(bytes32)
 _settings
RETURN _settings_1 (-> ['position'])
```
#### SafePct.mulDiv(uint256,uint256,uint256) [INTERNAL]
```slithir
x_1(uint256) := phi(['x_1', 'x_1'])
y_1(uint256) := phi(['y_1', 'y_1'])
z_1(uint256) := phi(['z_1', 'MAX_BIPS_1'])
 require(bool,error)(z > 0,revert DivisionByZero()())
TMP_10510(bool) = z_1 > 0
TMP_10511(None) = SOLIDITY_CALL revert DivisionByZero()()
TMP_10512(None) = SOLIDITY_CALL require(bool,error)(TMP_10510,TMP_10511)
 x == 0
TMP_10513(bool) = x_1 == 0
CONDITION TMP_10513
 0
RETURN 0
 xy = x * y
TMP_10514(uint256) = x_1 * y_1
xy_1(uint256) := TMP_10514(uint256)
 xy / x == y
TMP_10515(uint256) = xy_1 / x_1
TMP_10516(bool) = TMP_10515 == y_1
CONDITION TMP_10516
 xy / z
TMP_10517(uint256) = xy_1 / z_1
RETURN TMP_10517
 a = x / z
TMP_10518(uint256) = x_1 (c)/ z_1
a_1(uint256) := TMP_10518(uint256)
 b = x % z
TMP_10519(uint256) = x_1 % z_1
b_1(uint256) := TMP_10519(uint256)
 c = y / z
TMP_10520(uint256) = y_1 (c)/ z_1
c_1(uint256) := TMP_10520(uint256)
 d = y % z
TMP_10521(uint256) = y_1 % z_1
d_1(uint256) := TMP_10521(uint256)
 (a * c * z) + (a * d) + (b * c) + (b * d / z)
TMP_10522(uint256) = a_1 (c)* c_1
TMP_10523(uint256) = TMP_10522 (c)* z_1
TMP_10524(uint256) = a_1 (c)* d_1
TMP_10525(uint256) = TMP_10523 (c)+ TMP_10524
TMP_10526(uint256) = b_1 (c)* c_1
TMP_10527(uint256) = TMP_10525 (c)+ TMP_10526
TMP_10528(uint256) = b_1 (c)* d_1
TMP_10529(uint256) = TMP_10528 (c)/ z_1
TMP_10530(uint256) = TMP_10527 (c)+ TMP_10529
RETURN TMP_10530
```
#### SafeCast.toUint64(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint64).max,SafeCast: value doesn't fit in 64 bits)
TMP_747(uint64) := 18446744073709551615(uint64)
TMP_748(bool) = value_1 <= TMP_747
TMP_749(None) = SOLIDITY_CALL require(bool,string)(TMP_748,SafeCast: value doesn't fit in 64 bits)
 uint64(value)
TMP_750 = CONVERT value_1 to uint64
RETURN TMP_750
```
#### AssetManagerState.get() [INTERNAL]
```slithir
STATE_POSITION_1(bytes32) := phi(['STATE_POSITION_0'])
 position = STATE_POSITION
position_1(bytes32) := STATE_POSITION_1(bytes32)
 _state = position
_state_1 (-> ['position'])(AssetManagerState.State) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
```
#### IPriceReader.getPrice(string) [EXTERNAL]
```slithir

```
#### IPriceReader.getPriceFromTrustedProviders(string) [EXTERNAL]
```slithir

```
