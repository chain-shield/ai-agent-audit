






#### CollateralTypesFacet.addCollateralType(CollateralType.Data) [EXTERNAL]
```slithir
 CollateralTypes.add(_data)
LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.add(CollateralType.Data), arguments:['_data_1'] 
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
```
#### CollateralTypesFacet.deprecateCollateralType(CollateralType.Class,IERC20,uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_2446(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_2446'])(AssetManagerSettings.Data) := TMP_2446(AssetManagerSettings.Data)
 token = CollateralTypes.get(_collateralClass,_token)
TMP_2447(CollateralTypeInt.Data) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.get(CollateralType.Class,IERC20), arguments:['_collateralClass_1', '_token_1'] 
token_1 (-> ['TMP_2447'])(CollateralTypeInt.Data) := TMP_2447(CollateralTypeInt.Data)
 require(bool,error)(token.validUntil == 0 || token.validUntil > block.timestamp,revert TokenNotValid()())
REF_1362(uint64) -> token_1 (-> ['TMP_2447']).validUntil
TMP_2448(bool) = REF_1362 == 0
REF_1363(uint64) -> token_1 (-> ['TMP_2447']).validUntil
TMP_2449(bool) = REF_1363 > block.timestamp
TMP_2450(bool) = TMP_2448 || TMP_2449
TMP_2451(None) = SOLIDITY_CALL revert TokenNotValid()()
TMP_2452(None) = SOLIDITY_CALL require(bool,error)(TMP_2450,TMP_2451)
 require(bool,error)(_invalidationTimeSec >= settings.tokenInvalidationTimeMinSeconds,revert DeprecationTimeToShort()())
REF_1364(uint64) -> settings_1 (-> ['TMP_2446']).tokenInvalidationTimeMinSeconds
TMP_2453(bool) = _invalidationTimeSec_1 >= REF_1364
TMP_2454(None) = SOLIDITY_CALL revert DeprecationTimeToShort()()
TMP_2455(None) = SOLIDITY_CALL require(bool,error)(TMP_2453,TMP_2454)
 validUntil = block.timestamp + _invalidationTimeSec
TMP_2456(uint256) = block.timestamp (c)+ _invalidationTimeSec_1
validUntil_1(uint256) := TMP_2456(uint256)
 token.validUntil = validUntil.toUint64()
REF_1365(uint64) -> token_1 (-> ['TMP_2447']).validUntil
TMP_2457(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['validUntil_1'] 
token_2 (-> ['TMP_2447'])(CollateralTypeInt.Data) := phi(["token_1 (-> ['TMP_2447'])"])
REF_1365(uint64) (->token_2 (-> ['TMP_2447'])) := TMP_2457(uint64)
TMP_2447(CollateralTypeInt.Data) := phi(["token_2 (-> ['TMP_2447'])"])
 IAssetManagerEvents.CollateralTypeDeprecated(uint8(_collateralClass),address(_token),validUntil)
TMP_2458 = CONVERT _collateralClass_1 to uint8
TMP_2459 = CONVERT _token_1 to address
Emit CollateralTypeDeprecated(TMP_2458,TMP_2459,validUntil_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
```
#### CollateralTypesFacet.getCollateralType(CollateralType.Class,IERC20) [EXTERNAL]
```slithir
 CollateralTypes.getInfo(_collateralClass,_token)
TMP_2462(CollateralType.Data) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.getInfo(CollateralType.Class,IERC20), arguments:['_collateralClass_1', '_token_1'] 
RETURN TMP_2462
```

#### CollateralTypesFacet.setCollateralRatiosForToken(CollateralType.Class,IERC20,uint256,uint256) [EXTERNAL]
```slithir
 actionKey = keccak256(bytes)(abi.encode(msg.sig,_collateralClass,_token))
TMP_2431(bytes) = SOLIDITY_CALL abi.encode()(msg.sig,_collateralClass_1,_token_1)
TMP_2432(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_2431)
actionKey_1(bytes32) := TMP_2432(bytes32)
 SettingsUpdater.checkEnoughTimeSinceLastUpdate(actionKey)
LIBRARY_CALL, dest:SettingsUpdater, function:SettingsUpdater.checkEnoughTimeSinceLastUpdate(bytes32), arguments:['actionKey_1'] 
 ratiosValid = SafePct.MAX_BIPS < _minCollateralRatioBIPS && _minCollateralRatioBIPS <= _safetyMinCollateralRatioBIPS
REF_1353(uint256) -> SafePct.MAX_BIPS
TMP_2434(bool) = REF_1353 < _minCollateralRatioBIPS_1
TMP_2435(bool) = _minCollateralRatioBIPS_1 <= _safetyMinCollateralRatioBIPS_1
TMP_2436(bool) = TMP_2434 && TMP_2435
ratiosValid_1(bool) := TMP_2436(bool)
 require(bool,error)(ratiosValid,CollateralTypes.InvalidCollateralRatios())
TMP_2437(None) = SOLIDITY_CALL revert InvalidCollateralRatios()()
TMP_2438(None) = SOLIDITY_CALL require(bool,error)(ratiosValid_1,TMP_2437)
 token = CollateralTypes.get(_collateralClass,_token)
TMP_2439(CollateralTypeInt.Data) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.get(CollateralType.Class,IERC20), arguments:['_collateralClass_1', '_token_1'] 
token_1 (-> ['TMP_2439'])(CollateralTypeInt.Data) := TMP_2439(CollateralTypeInt.Data)
 token.minCollateralRatioBIPS = _minCollateralRatioBIPS.toUint32()
REF_1355(uint32) -> token_1 (-> ['TMP_2439']).minCollateralRatioBIPS
TMP_2440(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_minCollateralRatioBIPS_1'] 
token_2 (-> ['TMP_2439'])(CollateralTypeInt.Data) := phi(["token_1 (-> ['TMP_2439'])"])
REF_1355(uint32) (->token_2 (-> ['TMP_2439'])) := TMP_2440(uint32)
TMP_2439(CollateralTypeInt.Data) := phi(["token_2 (-> ['TMP_2439'])"])
 token.safetyMinCollateralRatioBIPS = _safetyMinCollateralRatioBIPS.toUint32()
REF_1357(uint32) -> token_2 (-> ['TMP_2439']).safetyMinCollateralRatioBIPS
TMP_2441(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_safetyMinCollateralRatioBIPS_1'] 
token_3 (-> ['TMP_2439'])(CollateralTypeInt.Data) := phi(["token_2 (-> ['TMP_2439'])"])
REF_1357(uint32) (->token_3 (-> ['TMP_2439'])) := TMP_2441(uint32)
TMP_2439(CollateralTypeInt.Data) := phi(["token_3 (-> ['TMP_2439'])"])
 IAssetManagerEvents.CollateralRatiosChanged(uint8(_collateralClass),address(_token),_minCollateralRatioBIPS,_safetyMinCollateralRatioBIPS)
TMP_2442 = CONVERT _collateralClass_1 to uint8
TMP_2443 = CONVERT _token_1 to address
Emit CollateralRatiosChanged(TMP_2442,TMP_2443,_minCollateralRatioBIPS_1,_safetyMinCollateralRatioBIPS_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
```
#### CollateralTypes.add(CollateralType.Data) [INTERNAL]
```slithir
 require(bool,error)(_data.collateralClass == CollateralType.Class.VAULT,revert NotAVaultCollateral()())
REF_3052(CollateralType.Class) -> _data_1.collateralClass
REF_3053(CollateralType.Class) -> Class.VAULT
TMP_4549(bool) = REF_3052 == REF_3053
TMP_4550(None) = SOLIDITY_CALL revert NotAVaultCollateral()()
TMP_4551(None) = SOLIDITY_CALL require(bool,error)(TMP_4549,TMP_4550)
 _add(_data)
TMP_4552(uint256) = INTERNAL_CALL, CollateralTypes._add(CollateralType.Data)(_data_1)
```
#### CollateralTypes.get(CollateralType.Class,IERC20) [INTERNAL]
```slithir
_collateralClass_1(CollateralType.Class) := phi(['_collateralClass_1'])
_token_1(IERC20) := phi(['_token_1'])
 state = AssetManagerState.get()
TMP_4563(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4563'])(AssetManagerState.State) := TMP_4563(AssetManagerState.State)
 index = state.collateralTokenIndex[_tokenKey(_collateralClass,_token)]
REF_3062(mapping(bytes32 => uint256)) -> state_1 (-> ['TMP_4563']).collateralTokenIndex
TMP_4564(bytes32) = INTERNAL_CALL, CollateralTypes._tokenKey(CollateralType.Class,IERC20)(_collateralClass_1,_token_1)
REF_3063(uint256) -> REF_3062[TMP_4564]
index_1(uint256) := REF_3063(uint256)
 require(bool,error)(index > 0,revert UnknownToken()())
TMP_4565(bool) = index_1 > 0
TMP_4566(None) = SOLIDITY_CALL revert UnknownToken()()
TMP_4567(None) = SOLIDITY_CALL require(bool,error)(TMP_4565,TMP_4566)
 state.collateralTokens[index - 1]
REF_3064(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4563']).collateralTokens
TMP_4568(uint256) = index_1 (c)- 1
REF_3065(CollateralTypeInt.Data) -> REF_3064[TMP_4568]
RETURN REF_3065
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
#### CollateralTypes.getInfo(CollateralType.Class,IERC20) [INTERNAL]
```slithir
 token = CollateralTypes.get(_collateralClass,_token)
TMP_4555(CollateralTypeInt.Data) = INTERNAL_CALL, CollateralTypes.get(CollateralType.Class,IERC20)(_collateralClass_1,_token_1)
token_1 (-> ['TMP_4555'])(CollateralTypeInt.Data) := TMP_4555(CollateralTypeInt.Data)
 _getInfo(token)
TMP_4556(CollateralType.Data) = INTERNAL_CALL, CollateralTypes._getInfo(CollateralTypeInt.Data)(token_1 (-> ['TMP_4555']))
RETURN TMP_4556
```
#### CollateralTypes.getAllInfos() [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4557(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4557'])(AssetManagerState.State) := TMP_4557(AssetManagerState.State)
 length = state.collateralTokens.length
REF_3056(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4557']).collateralTokens
REF_3057 -> LENGTH REF_3056
length_1(uint256) := REF_3057(uint256)
 _result = new CollateralType.Data[](length)
TMP_4559(CollateralType.Data[])  = new CollateralType.Data[](length_1)
_result_1(CollateralType.Data[]) = ['TMP_4559(CollateralType.Data[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < length
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_4560(bool) = i_2 < length_1
CONDITION TMP_4560
 _result[i] = _getInfo(state.collateralTokens[i])
REF_3058(CollateralType.Data) -> _result_1[i_2]
REF_3059(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4557']).collateralTokens
REF_3060(CollateralTypeInt.Data) -> REF_3059[i_2]
TMP_4561(CollateralType.Data) = INTERNAL_CALL, CollateralTypes._getInfo(CollateralTypeInt.Data)(REF_3060)
_result_2(CollateralType.Data[]) := phi(['_result_1'])
REF_3058(CollateralType.Data) (->_result_2) := TMP_4561(CollateralType.Data)
 i ++
TMP_4562(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 _result
RETURN _result_1
```
#### SettingsUpdater.checkEnoughTimeSinceLastUpdate(bytes32) [INTERNAL]
```slithir
_action_1(bytes32) := phi(['msg.sig'])
 _state = _getUpdaterState()
TMP_5196(SettingsUpdater.UpdaterState) = INTERNAL_CALL, SettingsUpdater._getUpdaterState()()
_state_1 (-> ['TMP_5196'])(SettingsUpdater.UpdaterState) := TMP_5196(SettingsUpdater.UpdaterState)
 settings = Globals.getSettings()
TMP_5197(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_5197'])(AssetManagerSettings.Data) := TMP_5197(AssetManagerSettings.Data)
 lastUpdate = _state.lastUpdate[_action]
REF_3651(mapping(bytes32 => uint256)) -> _state_1 (-> ['TMP_5196']).lastUpdate
REF_3652(uint256) -> REF_3651[_action_1]
lastUpdate_1(uint256) := REF_3652(uint256)
 require(bool,error)(lastUpdate == 0 || block.timestamp >= lastUpdate + settings.minUpdateRepeatTimeSeconds,revert TooCloseToPreviousUpdate()())
TMP_5198(bool) = lastUpdate_1 == 0
REF_3653(uint64) -> settings_1 (-> ['TMP_5197']).minUpdateRepeatTimeSeconds
TMP_5199(uint256) = lastUpdate_1 (c)+ REF_3653
TMP_5200(bool) = block.timestamp >= TMP_5199
TMP_5201(bool) = TMP_5198 || TMP_5200
TMP_5202(None) = SOLIDITY_CALL revert TooCloseToPreviousUpdate()()
TMP_5203(None) = SOLIDITY_CALL require(bool,error)(TMP_5201,TMP_5202)
 _state.lastUpdate[_action] = block.timestamp
REF_3654(mapping(bytes32 => uint256)) -> _state_1 (-> ['TMP_5196']).lastUpdate
REF_3655(uint256) -> REF_3654[_action_1]
_state_2 (-> ['TMP_5196'])(SettingsUpdater.UpdaterState) := phi(["_state_1 (-> ['TMP_5196'])"])
REF_3655(uint256) (->_state_2 (-> ['TMP_5196'])) := block.timestamp(uint256)
TMP_5196(SettingsUpdater.UpdaterState) := phi(["_state_2 (-> ['TMP_5196'])"])
```
#### SafeCast.toUint32(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint32).max,SafeCast: value doesn't fit in 32 bits)
TMP_767(uint32) := 4294967295(uint32)
TMP_768(bool) = value_1 <= TMP_767
TMP_769(None) = SOLIDITY_CALL require(bool,string)(TMP_768,SafeCast: value doesn't fit in 32 bits)
 uint32(value)
TMP_770 = CONVERT value_1 to uint32
RETURN TMP_770
```
#### CollateralTypes._add(CollateralType.Data) [PRIVATE]
```slithir
_data_1(CollateralType.Data) := phi(['_data_1', 'REF_3051', '_data_1', 'REF_3046'])
 state = AssetManagerState.get()
TMP_4581(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4581'])(AssetManagerState.State) := TMP_4581(AssetManagerState.State)
 require(bool,error)(address(_data.token) != address(0),revert TokenZero()())
REF_3075(IERC20) -> _data_1.token
TMP_4582 = CONVERT REF_3075 to address
TMP_4583 = CONVERT 0 to address
TMP_4584(bool) = TMP_4582 != TMP_4583
TMP_4585(None) = SOLIDITY_CALL revert TokenZero()()
TMP_4586(None) = SOLIDITY_CALL require(bool,error)(TMP_4584,TMP_4585)
 tokenKey = _tokenKey(_data.collateralClass,_data.token)
REF_3076(CollateralType.Class) -> _data_1.collateralClass
REF_3077(IERC20) -> _data_1.token
TMP_4587(bytes32) = INTERNAL_CALL, CollateralTypes._tokenKey(CollateralType.Class,IERC20)(REF_3076,REF_3077)
tokenKey_1(bytes32) := TMP_4587(bytes32)
 require(bool,error)(state.collateralTokenIndex[tokenKey] == 0,revert TokenAlreadyExists()())
REF_3078(mapping(bytes32 => uint256)) -> state_1 (-> ['TMP_4581']).collateralTokenIndex
REF_3079(uint256) -> REF_3078[tokenKey_1]
TMP_4588(bool) = REF_3079 == 0
TMP_4589(None) = SOLIDITY_CALL revert TokenAlreadyExists()()
TMP_4590(None) = SOLIDITY_CALL require(bool,error)(TMP_4588,TMP_4589)
 require(bool,error)(_data.validUntil == 0,revert CannotAddDeprecatedToken()())
REF_3080(uint256) -> _data_1.validUntil
TMP_4591(bool) = REF_3080 == 0
TMP_4592(None) = SOLIDITY_CALL revert CannotAddDeprecatedToken()()
TMP_4593(None) = SOLIDITY_CALL require(bool,error)(TMP_4591,TMP_4592)
 ratiosValid = SafePct.MAX_BIPS < _data.minCollateralRatioBIPS && _data.minCollateralRatioBIPS <= _data.safetyMinCollateralRatioBIPS
REF_3081(uint256) -> SafePct.MAX_BIPS
REF_3082(uint256) -> _data_1.minCollateralRatioBIPS
TMP_4594(bool) = REF_3081 < REF_3082
REF_3083(uint256) -> _data_1.minCollateralRatioBIPS
REF_3084(uint256) -> _data_1.safetyMinCollateralRatioBIPS
TMP_4595(bool) = REF_3083 <= REF_3084
TMP_4596(bool) = TMP_4594 && TMP_4595
ratiosValid_1(bool) := TMP_4596(bool)
 require(bool,error)(ratiosValid,revert InvalidCollateralRatios()())
TMP_4597(None) = SOLIDITY_CALL revert InvalidCollateralRatios()()
TMP_4598(None) = SOLIDITY_CALL require(bool,error)(ratiosValid_1,TMP_4597)
 (assetPrice,None,None) = Conversion.readFtsoPrice(_data.assetFtsoSymbol,false)
REF_3086(string) -> _data_1.assetFtsoSymbol
TUPLE_41(uint256,uint256,uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.readFtsoPrice(string,bool), arguments:['REF_3086', 'False'] 
assetPrice_1(uint256)= UNPACK TUPLE_41 index: 0 
 require(bool,error)(assetPrice != 0,revert PriceNotInitialized()())
TMP_4599(bool) = assetPrice_1 != 0
TMP_4600(None) = SOLIDITY_CALL revert PriceNotInitialized()()
TMP_4601(None) = SOLIDITY_CALL require(bool,error)(TMP_4599,TMP_4600)
 ! _data.directPricePair
REF_3087(bool) -> _data_1.directPricePair
TMP_4602 = UnaryType.BANG REF_3087 
CONDITION TMP_4602
 (tokenPrice,None,None) = Conversion.readFtsoPrice(_data.tokenFtsoSymbol,false)
REF_3089(string) -> _data_1.tokenFtsoSymbol
TUPLE_42(uint256,uint256,uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.readFtsoPrice(string,bool), arguments:['REF_3089', 'False'] 
tokenPrice_1(uint256)= UNPACK TUPLE_42 index: 0 
 require(bool,error)(tokenPrice != 0,revert PriceNotInitialized()())
TMP_4603(bool) = tokenPrice_1 != 0
TMP_4604(None) = SOLIDITY_CALL revert PriceNotInitialized()()
TMP_4605(None) = SOLIDITY_CALL require(bool,error)(TMP_4603,TMP_4604)
 newTokenIndex = state.collateralTokens.length
REF_3090(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4581']).collateralTokens
REF_3091 -> LENGTH REF_3090
newTokenIndex_1(uint256) := REF_3091(uint256)
 state.collateralTokens.push(CollateralTypeInt.Data({token:_data.token,collateralClass:_data.collateralClass,decimals:_data.decimals.toUint8(),validUntil:_data.validUntil.toUint64(),directPricePair:_data.directPricePair,assetFtsoSymbol:_data.assetFtsoSymbol,tokenFtsoSymbol:_data.tokenFtsoSymbol,minCollateralRatioBIPS:_data.minCollateralRatioBIPS.toUint32(),__ccbMinCollateralRatioBIPS:0,safetyMinCollateralRatioBIPS:_data.safetyMinCollateralRatioBIPS.toUint32()}))
REF_3092(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4581']).collateralTokens
REF_3095(IERC20) -> _data_1.token
REF_3096(CollateralType.Class) -> _data_1.collateralClass
REF_3097(uint256) -> _data_1.decimals
TMP_4606(uint8) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint8(uint256), arguments:['REF_3097'] 
REF_3099(uint256) -> _data_1.validUntil
TMP_4607(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['REF_3099'] 
REF_3101(bool) -> _data_1.directPricePair
REF_3102(string) -> _data_1.assetFtsoSymbol
REF_3103(string) -> _data_1.tokenFtsoSymbol
REF_3104(uint256) -> _data_1.minCollateralRatioBIPS
TMP_4608(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['REF_3104'] 
REF_3106(uint256) -> _data_1.safetyMinCollateralRatioBIPS
TMP_4609(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['REF_3106'] 
TMP_4610(CollateralTypeInt.Data) = new Data(REF_3095,REF_3096,TMP_4606,TMP_4607,REF_3101,REF_3102,REF_3103,TMP_4608,0,TMP_4609)
REF_3108 -> LENGTH REF_3092
TMP_4612(uint256) := REF_3108(uint256)
TMP_4613(uint256) = TMP_4612 (c)+ 1
state_2 (-> ['TMP_4581'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_4581'])"])
REF_3108(uint256) (->state_3 (-> ['TMP_4581'])) := TMP_4613(uint256)
REF_3109(CollateralTypeInt.Data) -> REF_3092[TMP_4612]
state_3 (-> ['TMP_4581'])(AssetManagerState.State) := phi(["state_2 (-> ['TMP_4581'])"])
REF_3109(CollateralTypeInt.Data) (->state_3 (-> ['TMP_4581'])) := TMP_4610(CollateralTypeInt.Data)
TMP_4581(AssetManagerState.State) := phi(["state_3 (-> ['TMP_4581'])"])
 state.collateralTokenIndex[tokenKey] = newTokenIndex + 1
REF_3110(mapping(bytes32 => uint256)) -> state_3 (-> ['TMP_4581']).collateralTokenIndex
REF_3111(uint256) -> REF_3110[tokenKey_1]
TMP_4614(uint256) = newTokenIndex_1 (c)+ 1
state_4 (-> ['TMP_4581'])(AssetManagerState.State) := phi(["state_3 (-> ['TMP_4581'])"])
REF_3111(uint256) (->state_4 (-> ['TMP_4581'])) := TMP_4614(uint256)
TMP_4581(AssetManagerState.State) := phi(["state_4 (-> ['TMP_4581'])"])
 IAssetManagerEvents.CollateralTypeAdded(uint8(_data.collateralClass),address(_data.token),_data.decimals,_data.directPricePair,_data.assetFtsoSymbol,_data.tokenFtsoSymbol,_data.minCollateralRatioBIPS,_data.safetyMinCollateralRatioBIPS)
REF_3113(CollateralType.Class) -> _data_1.collateralClass
TMP_4615 = CONVERT REF_3113 to uint8
REF_3114(IERC20) -> _data_1.token
TMP_4616 = CONVERT REF_3114 to address
REF_3115(uint256) -> _data_1.decimals
REF_3116(bool) -> _data_1.directPricePair
REF_3117(string) -> _data_1.assetFtsoSymbol
REF_3118(string) -> _data_1.tokenFtsoSymbol
REF_3119(uint256) -> _data_1.minCollateralRatioBIPS
REF_3120(uint256) -> _data_1.safetyMinCollateralRatioBIPS
Emit CollateralTypeAdded(TMP_4615,TMP_4616,REF_3115,REF_3116,REF_3117,REF_3118,REF_3119,REF_3120)
 newTokenIndex
RETURN newTokenIndex_1
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
#### CollateralTypes._getInfo(CollateralTypeInt.Data) [PRIVATE]
```slithir
token_1 (-> ['TMP_4555'])(CollateralTypeInt.Data) := phi(["token_1 (-> ['TMP_4555'])", 'REF_3060'])
 CollateralType.Data({token:token.token,collateralClass:token.collateralClass,decimals:token.decimals,validUntil:token.validUntil,directPricePair:token.directPricePair,assetFtsoSymbol:token.assetFtsoSymbol,tokenFtsoSymbol:token.tokenFtsoSymbol,minCollateralRatioBIPS:token.minCollateralRatioBIPS,safetyMinCollateralRatioBIPS:token.safetyMinCollateralRatioBIPS})
REF_3129(IERC20) -> token_1 (-> ['TMP_4555']).token
REF_3130(CollateralType.Class) -> token_1 (-> ['TMP_4555']).collateralClass
REF_3131(uint8) -> token_1 (-> ['TMP_4555']).decimals
REF_3132(uint64) -> token_1 (-> ['TMP_4555']).validUntil
REF_3133(bool) -> token_1 (-> ['TMP_4555']).directPricePair
REF_3134(string) -> token_1 (-> ['TMP_4555']).assetFtsoSymbol
REF_3135(string) -> token_1 (-> ['TMP_4555']).tokenFtsoSymbol
REF_3136(uint32) -> token_1 (-> ['TMP_4555']).minCollateralRatioBIPS
REF_3137(uint32) -> token_1 (-> ['TMP_4555']).safetyMinCollateralRatioBIPS
TMP_4622(CollateralType.Data) = new Data(REF_3130,REF_3129,REF_3131,REF_3132,REF_3133,REF_3134,REF_3135,REF_3136,REF_3137)
RETURN TMP_4622
```
#### SettingsUpdater._getUpdaterState() [PRIVATE]
```slithir
UPDATES_STATE_POSITION_1(bytes32) := phi(['UPDATES_STATE_POSITION_0'])
 position = UPDATES_STATE_POSITION
position_1(bytes32) := UPDATES_STATE_POSITION_1(bytes32)
 _state = position
_state_1 (-> ['position'])(SettingsUpdater.UpdaterState) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
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
#### SafeCast.toUint8(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint8).max,SafeCast: value doesn't fit in 8 bits)
TMP_782(uint8) := 255(uint8)
TMP_783(bool) = value_1 <= TMP_782
TMP_784(None) = SOLIDITY_CALL require(bool,string)(TMP_783,SafeCast: value doesn't fit in 8 bits)
 uint8(value)
TMP_785 = CONVERT value_1 to uint8
RETURN TMP_785
```
