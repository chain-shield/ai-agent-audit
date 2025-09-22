
NOTE: GovernedBase is NOT showing up in IR because its abstract contract!






#### AssetManagerInit.init(IGovernanceSettings,address,AssetManagerSettings.Data,CollateralType.Data[]) [EXTERNAL]
```slithir
 GovernedBase.initialise(_governanceSettings,_initialGovernance)
INTERNAL_CALL, GovernedBase.initialise(IGovernanceSettings,address)(_governanceSettings_1,_initialGovernance_1)
 ReentrancyGuard.initializeReentrancyGuard()
INTERNAL_CALL, ReentrancyGuard.initializeReentrancyGuard()()
 SettingsInitializer.validateAndSet(_settings)
LIBRARY_CALL, dest:SettingsInitializer, function:SettingsInitializer.validateAndSet(AssetManagerSettings.Data), arguments:['_settings_1'] 
 CollateralTypes.initialize(_initialCollateralTypes)
LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.initialize(CollateralType.Data[]), arguments:['_initialCollateralTypes_1'] 
 _initIERC165()
INTERNAL_CALL, AssetManagerInit._initIERC165()()
```
#### AssetManagerInit.upgradeERC165Identifiers() [EXTERNAL]
```slithir
 ds = LibDiamond.diamondStorage()
TMP_2087(LibDiamond.DiamondStorage) = LIBRARY_CALL, dest:LibDiamond, function:LibDiamond.diamondStorage(), arguments:[] 
ds_1 (-> ['TMP_2087'])(LibDiamond.DiamondStorage) := TMP_2087(LibDiamond.DiamondStorage)
 require(bool,error)(ds.supportedInterfaces[type()(IERC165).interfaceId],revert NotInitialized()())
REF_995(mapping(bytes4 => bool)) -> ds_1 (-> ['TMP_2087']).supportedInterfaces
TMP_2088(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_996(bytes4) (->None) := 33540519(bytes4)
REF_997(bool) -> REF_995[REF_996]
TMP_2089(None) = SOLIDITY_CALL revert NotInitialized()()
TMP_2090(None) = SOLIDITY_CALL require(bool,error)(REF_997,TMP_2089)
 ds.supportedInterfaces[type()(IGoverned).interfaceId] = true
REF_998(mapping(bytes4 => bool)) -> ds_1 (-> ['TMP_2087']).supportedInterfaces
TMP_2091(type(IGoverned)) = SOLIDITY_CALL type()(IGoverned)
REF_999(bytes4) (->None) := 3301115419(bytes4)
REF_1000(bool) -> REF_998[REF_999]
ds_2 (-> ['TMP_2087'])(LibDiamond.DiamondStorage) := phi(["ds_1 (-> ['TMP_2087'])"])
REF_1000(bool) (->ds_2 (-> ['TMP_2087'])) := True(bool)
TMP_2087(LibDiamond.DiamondStorage) := phi(["ds_2 (-> ['TMP_2087'])"])
 ds.supportedInterfaces[type()(IAssetManager).interfaceId] = true
REF_1001(mapping(bytes4 => bool)) -> ds_2 (-> ['TMP_2087']).supportedInterfaces
TMP_2092(type(IAssetManager)) = SOLIDITY_CALL type()(IAssetManager)
REF_1002(bytes4) (->None) := 2989796443(bytes4)
REF_1003(bool) -> REF_1001[REF_1002]
ds_3 (-> ['TMP_2087'])(LibDiamond.DiamondStorage) := phi(["ds_2 (-> ['TMP_2087'])"])
REF_1003(bool) (->ds_3 (-> ['TMP_2087'])) := True(bool)
TMP_2087(LibDiamond.DiamondStorage) := phi(["ds_3 (-> ['TMP_2087'])"])
 ds.supportedInterfaces[type()(IIAssetManager).interfaceId] = true
REF_1004(mapping(bytes4 => bool)) -> ds_3 (-> ['TMP_2087']).supportedInterfaces
TMP_2093(type(IIAssetManager)) = SOLIDITY_CALL type()(IIAssetManager)
REF_1005(bytes4) (->None) := 3793145862(bytes4)
REF_1006(bool) -> REF_1004[REF_1005]
ds_4 (-> ['TMP_2087'])(LibDiamond.DiamondStorage) := phi(["ds_3 (-> ['TMP_2087'])"])
REF_1006(bool) (->ds_4 (-> ['TMP_2087'])) := True(bool)
TMP_2087(LibDiamond.DiamondStorage) := phi(["ds_4 (-> ['TMP_2087'])"])
 ds.supportedInterfaces[type()(IAgentPing).interfaceId] = true
REF_1007(mapping(bytes4 => bool)) -> ds_4 (-> ['TMP_2087']).supportedInterfaces
TMP_2094(type(IAgentPing)) = SOLIDITY_CALL type()(IAgentPing)
REF_1008(bytes4) (->None) := 3063171591(bytes4)
REF_1009(bool) -> REF_1007[REF_1008]
ds_5 (-> ['TMP_2087'])(LibDiamond.DiamondStorage) := phi(["ds_4 (-> ['TMP_2087'])"])
REF_1009(bool) (->ds_5 (-> ['TMP_2087'])) := True(bool)
TMP_2087(LibDiamond.DiamondStorage) := phi(["ds_5 (-> ['TMP_2087'])"])
```
#### LibDiamond.diamondStorage() [INTERNAL]
```slithir
DIAMOND_STORAGE_POSITION_1(bytes32) := phi(['DIAMOND_STORAGE_POSITION_0'])
 position = DIAMOND_STORAGE_POSITION
position_1(bytes32) := DIAMOND_STORAGE_POSITION_1(bytes32)
 ds = position
ds_1 (-> ['position'])(LibDiamond.DiamondStorage) := position_1(bytes32)
 ds
RETURN ds_1 (-> ['position'])
```
#### CollateralTypes.initialize(CollateralType.Data[]) [INTERNAL]
```slithir
 require(bool,error)(_data.length >= 2,revert AtLeastTwoCollateralsRequired()())
REF_3042 -> LENGTH _data_1
TMP_4535(bool) = REF_3042 >= 2
TMP_4536(None) = SOLIDITY_CALL revert AtLeastTwoCollateralsRequired()()
TMP_4537(None) = SOLIDITY_CALL require(bool,error)(TMP_4535,TMP_4536)
 require(bool,error)(_data[0].collateralClass == CollateralType.Class.POOL,revert NotAPoolCollateralAtZero()())
REF_3043(CollateralType.Data) -> _data_1[0]
REF_3044(CollateralType.Class) -> REF_3043.collateralClass
REF_3045(CollateralType.Class) -> Class.POOL
TMP_4538(bool) = REF_3044 == REF_3045
TMP_4539(None) = SOLIDITY_CALL revert NotAPoolCollateralAtZero()()
TMP_4540(None) = SOLIDITY_CALL require(bool,error)(TMP_4538,TMP_4539)
 _add(_data[0])
REF_3046(CollateralType.Data) -> _data_1[0]
TMP_4541(uint256) = INTERNAL_CALL, CollateralTypes._add(CollateralType.Data)(REF_3046)
 _setPoolCollateralTypeIndex(0)
INTERNAL_CALL, CollateralTypes._setPoolCollateralTypeIndex(uint256)(0)
 i = 1
i_1(uint256) := 1(uint256)
 i < _data.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3047 -> LENGTH _data_1
TMP_4543(bool) = i_2 < REF_3047
CONDITION TMP_4543
 require(bool,error)(_data[i].collateralClass == CollateralType.Class.VAULT,revert NotAVaultCollateral()())
REF_3048(CollateralType.Data) -> _data_1[i_2]
REF_3049(CollateralType.Class) -> REF_3048.collateralClass
REF_3050(CollateralType.Class) -> Class.VAULT
TMP_4544(bool) = REF_3049 == REF_3050
TMP_4545(None) = SOLIDITY_CALL revert NotAVaultCollateral()()
TMP_4546(None) = SOLIDITY_CALL require(bool,error)(TMP_4544,TMP_4545)
 _add(_data[i])
REF_3051(CollateralType.Data) -> _data_1[i_2]
TMP_4547(uint256) = INTERNAL_CALL, CollateralTypes._add(CollateralType.Data)(REF_3051)
 i ++
TMP_4548(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### SettingsInitializer.validateAndSet(AssetManagerSettings.Data) [INTERNAL]
```slithir
 _validateSettings(_settings)
INTERNAL_CALL, SettingsInitializer._validateSettings(AssetManagerSettings.Data)(_settings_1)
 _setAllSettings(_settings)
INTERNAL_CALL, SettingsInitializer._setAllSettings(AssetManagerSettings.Data)(_settings_1)
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
#### CollateralTypes._setPoolCollateralTypeIndex(uint256) [PRIVATE]
```slithir
_index_1(uint256) := phi(['index_1'])
 state = AssetManagerState.get()
TMP_4618(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4618'])(AssetManagerState.State) := TMP_4618(AssetManagerState.State)
 token = state.collateralTokens[_index]
REF_3122(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4618']).collateralTokens
REF_3123(CollateralTypeInt.Data) -> REF_3122[_index_1]
token_1 (-> ['state'])(CollateralTypeInt.Data) := REF_3123(CollateralTypeInt.Data)
 assert(bool)(token.collateralClass == CollateralType.Class.POOL)
REF_3124(CollateralType.Class) -> token_1 (-> ['state']).collateralClass
REF_3125(CollateralType.Class) -> Class.POOL
TMP_4619(bool) = REF_3124 == REF_3125
TMP_4620(None) = SOLIDITY_CALL assert(bool)(TMP_4619)
 state.poolCollateralIndex = _index.toUint16()
REF_3126(uint16) -> state_1 (-> ['TMP_4618']).poolCollateralIndex
TMP_4621(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_index_1'] 
state_2 (-> ['TMP_4618'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_4618'])"])
REF_3126(uint16) (->state_2 (-> ['TMP_4618'])) := TMP_4621(uint16)
TMP_4618(AssetManagerState.State) := phi(["state_2 (-> ['TMP_4618'])"])
```
#### SettingsInitializer._setAllSettings(AssetManagerSettings.Data) [PRIVATE]
```slithir
_settings_1(AssetManagerSettings.Data) := phi(['_settings_1'])
 position = Globals.ASSET_MANAGER_SETTINGS_POSITION
REF_3593(bytes32) -> Globals.ASSET_MANAGER_SETTINGS_POSITION
position_1(bytes32) := REF_3593(bytes32)
 wrapper = position
wrapper_1 (-> ['position'])(SettingsInitializer.SettingsWrapper) := position_1(bytes32)
 wrapper.settings = _settings
REF_3594(AssetManagerSettings.Data) -> wrapper_1 (-> ['position']).settings
wrapper_2 (-> ['position'])(SettingsInitializer.SettingsWrapper) := phi(["wrapper_1 (-> ['position'])"])
REF_3594(AssetManagerSettings.Data) (->wrapper_2 (-> ['position'])) := _settings_1(AssetManagerSettings.Data)
position_2(bytes32) := phi(["wrapper_2 (-> ['position'])"])
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
#### SafeCast.toUint16(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint16).max,SafeCast: value doesn't fit in 16 bits)
TMP_777(uint16) := 65535(uint16)
TMP_778(bool) = value_1 <= TMP_777
TMP_779(None) = SOLIDITY_CALL require(bool,string)(TMP_778,SafeCast: value doesn't fit in 16 bits)
 uint16(value)
TMP_780 = CONVERT value_1 to uint16
RETURN TMP_780
```
#### SettingsValidators.validateLiquidationFactors(uint256[],uint256[]) [INTERNAL]
```slithir
 require(bool,error)(liquidationFactors.length == vaultCollateralFactors.length,revert LengthsNotEqual()())
REF_3656 -> LENGTH liquidationFactors_1
REF_3657 -> LENGTH vaultCollateralFactors_1
TMP_5213(bool) = REF_3656 == REF_3657
TMP_5214(None) = SOLIDITY_CALL revert LengthsNotEqual()()
TMP_5215(None) = SOLIDITY_CALL require(bool,error)(TMP_5213,TMP_5214)
 require(bool,error)(liquidationFactors.length >= 1,revert AtLeastOneFactorRequired()())
REF_3658 -> LENGTH liquidationFactors_1
TMP_5216(bool) = REF_3658 >= 1
TMP_5217(None) = SOLIDITY_CALL revert AtLeastOneFactorRequired()()
TMP_5218(None) = SOLIDITY_CALL require(bool,error)(TMP_5216,TMP_5217)
 i = 0
i_1(uint256) := 0(uint256)
 i < liquidationFactors.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3659 -> LENGTH liquidationFactors_1
TMP_5219(bool) = i_2 < REF_3659
CONDITION TMP_5219
 require(bool,error)(liquidationFactors[i] > SafePct.MAX_BIPS,revert FactorNotAboveOne()())
REF_3660(uint256) -> liquidationFactors_1[i_2]
REF_3661(uint256) -> SafePct.MAX_BIPS
TMP_5220(bool) = REF_3660 > REF_3661
TMP_5221(None) = SOLIDITY_CALL revert FactorNotAboveOne()()
TMP_5222(None) = SOLIDITY_CALL require(bool,error)(TMP_5220,TMP_5221)
 require(bool,error)(vaultCollateralFactors[i] <= liquidationFactors[i],revert VaultCollateralFactorHigherThanTotal()())
REF_3662(uint256) -> vaultCollateralFactors_1[i_2]
REF_3663(uint256) -> liquidationFactors_1[i_2]
TMP_5223(bool) = REF_3662 <= REF_3663
TMP_5224(None) = SOLIDITY_CALL revert VaultCollateralFactorHigherThanTotal()()
TMP_5225(None) = SOLIDITY_CALL require(bool,error)(TMP_5223,TMP_5224)
 require(bool,error)(i == 0 || liquidationFactors[i] > liquidationFactors[i - 1],revert FactorsNotIncreasing()())
TMP_5226(bool) = i_2 == 0
REF_3664(uint256) -> liquidationFactors_1[i_2]
TMP_5227(uint256) = i_2 (c)- 1
REF_3665(uint256) -> liquidationFactors_1[TMP_5227]
TMP_5228(bool) = REF_3664 > REF_3665
TMP_5229(bool) = TMP_5226 || TMP_5228
TMP_5230(None) = SOLIDITY_CALL revert FactorsNotIncreasing()()
TMP_5231(None) = SOLIDITY_CALL require(bool,error)(TMP_5229,TMP_5230)
 i ++
TMP_5232(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### SettingsValidators.validateTimeForPayment(uint256,uint256,uint256) [INTERNAL]
```slithir
MAXIMUM_PROOF_WINDOW_1(uint256) := phi(['MAXIMUM_PROOF_WINDOW_0'])
 require(bool,error)(_underlyingSeconds <= MAXIMUM_PROOF_WINDOW,revert ValueTooHigh()())
TMP_5205(bool) = _underlyingSeconds_1 <= MAXIMUM_PROOF_WINDOW_1
TMP_5206(None) = SOLIDITY_CALL revert ValueTooHigh()()
TMP_5207(None) = SOLIDITY_CALL require(bool,error)(TMP_5205,TMP_5206)
 require(bool,error)(_underlyingBlocks * _averageBlockTimeMS / 1000 <= MAXIMUM_PROOF_WINDOW,revert ValueTooHigh()())
TMP_5208(uint256) = _underlyingBlocks_1 (c)* _averageBlockTimeMS_1
TMP_5209(uint256) = TMP_5208 (c)/ 1000
TMP_5210(bool) = TMP_5209 <= MAXIMUM_PROOF_WINDOW_1
TMP_5211(None) = SOLIDITY_CALL revert ValueTooHigh()()
TMP_5212(None) = SOLIDITY_CALL require(bool,error)(TMP_5210,TMP_5211)
```
