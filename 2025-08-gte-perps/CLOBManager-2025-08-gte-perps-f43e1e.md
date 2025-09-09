






#### CLOBManager._assertValidSettings(SettingsParams,uint256) [INTERNAL]
```slithir
settings_1(SettingsParams) := phi(['settings_1'])
baseSize_1(uint256) := phi(['REF_606'])
 settings.maxLimitsPerTx == 0
REF_639(uint8) -> settings_1.maxLimitsPerTx
TMP_1345(bool) = REF_639 == 0
CONDITION TMP_1345
 revert InvalidSettings()()
TMP_1346(None) = SOLIDITY_CALL revert InvalidSettings()()
 settings.minLimitOrderAmountInBase < MIN_MIN_LIMIT_ORDER_AMOUNT_BASE
REF_640(uint256) -> settings_1.minLimitOrderAmountInBase
TMP_1347(bool) = REF_640 < MIN_MIN_LIMIT_ORDER_AMOUNT_BASE
CONDITION TMP_1347
 revert InvalidSettings()()
TMP_1348(None) = SOLIDITY_CALL revert InvalidSettings()()
 settings.minLimitOrderAmountInBase < settings.lotSizeInBase
REF_641(uint256) -> settings_1.minLimitOrderAmountInBase
REF_642(uint256) -> settings_1.lotSizeInBase
TMP_1349(bool) = REF_641 < REF_642
CONDITION TMP_1349
 revert InvalidSettings()()
TMP_1350(None) = SOLIDITY_CALL revert InvalidSettings()()
 settings.tickSize.fullMulDiv(settings.lotSizeInBase,baseSize) == 0
REF_643(uint256) -> settings_1.tickSize
REF_645(uint256) -> settings_1.lotSizeInBase
TMP_1351(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_643', 'REF_645', 'baseSize_1'] 
TMP_1352(bool) = TMP_1351 == 0
CONDITION TMP_1352
 revert InvalidSettings()()
TMP_1353(None) = SOLIDITY_CALL revert InvalidSettings()()
```
#### CLOBManager._assertValidTokenPair(address,address) [INTERNAL]
```slithir
quoteToken_1(address) := phi(['quoteToken_1'])
baseToken_1(address) := phi(['baseToken_1'])
 quoteToken == baseToken
TMP_1354(bool) = quoteToken_1 == baseToken_1
CONDITION TMP_1354
 revert InvalidPair()()
TMP_1355(None) = SOLIDITY_CALL revert InvalidPair()()
 quoteToken == address(0)
TMP_1356 = CONVERT 0 to address
TMP_1357(bool) = quoteToken_1 == TMP_1356
CONDITION TMP_1357
 revert InvalidTokenAddress()()
TMP_1358(None) = SOLIDITY_CALL revert InvalidTokenAddress()()
 baseToken == address(0)
TMP_1359 = CONVERT 0 to address
TMP_1360(bool) = baseToken_1 == TMP_1359
CONDITION TMP_1360
 revert InvalidTokenAddress()()
TMP_1361(None) = SOLIDITY_CALL revert InvalidTokenAddress()()
```
#### CLOBManager._emitMarketCreated(address,address,uint8,uint8,ConfigParams,SettingsParams) [INTERNAL]
```slithir
creator_1(address) := phi(['msg.sender'])
marketAddress_1(address) := phi(['marketAddress_1'])
quoteDecimals_1(uint8) := phi(['quoteDecimals_1'])
baseDecimals_1(uint8) := phi(['baseDecimals_1'])
config_1(ConfigParams) := phi(['config_4'])
settings_1(SettingsParams) := phi(['settings_1'])
 MarketCreated(EventNonceLib.inc(),creator,config.baseToken,config.quoteToken,marketAddress,quoteDecimals,baseDecimals,config,settings)
TMP_1362(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
REF_647(address) -> config_1.baseToken
REF_648(address) -> config_1.quoteToken
Emit MarketCreated(TMP_1362,creator_1,REF_647,REF_648,marketAddress_1,quoteDecimals_1,baseDecimals_1,config_1,settings_1)
```
#### CLOBManager._getStorage() [INTERNAL]
```slithir
 CLOBManagerStorageLib.getCLOBManagerStorage()
TMP_1367(CLOBManagerStorage) = LIBRARY_CALL, dest:CLOBManagerStorageLib, function:CLOBManagerStorageLib.getCLOBManagerStorage(), arguments:[] 
RETURN TMP_1367
 ds
```
#### CLOBManager._getTokenHash(address,address) [INTERNAL]
```slithir
tokenA_1(address) := phi(['tokenA_1', 'quoteToken_1'])
tokenB_1(address) := phi(['baseToken_1', 'tokenB_1'])
 keccak256(bytes)(abi.encodePacked(tokenA,tokenB))
TMP_1364(bytes) = SOLIDITY_CALL abi.encodePacked()(tokenA_4,tokenB_4)
TMP_1365(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_1364)
RETURN TMP_1365
 tokenA < tokenB
TMP_1366(bool) = tokenA_1 < tokenB_1
CONDITION TMP_1366
 (tokenA,tokenB) = (tokenA,tokenB)
tokenA_2(address) := tokenA_1(address)
tokenB_2(address) := tokenB_1(address)
 (tokenA,tokenB) = (tokenB,tokenA)
tokenA_3(address) := tokenB_1(address)
tokenB_3(address) := tokenA_3(address)
tokenA_4(address) := phi(['tokenA_2', 'tokenA_3'])
tokenB_4(address) := phi(['tokenB_2', 'tokenB_3'])
```
#### CLOBManager.adminCancelExpiredOrders(ICLOB,OrderId[],Side) [EXTERNAL]
```slithir
EXPIRED_ORDER_CLEARER_1(uint256) := phi(['EXPIRED_ORDER_CLEARER_0', 'EXPIRED_ORDER_CLEARER_2'])
 market.adminCancelExpiredOrders(ids,side)
TMP_1333(bool[]) = HIGH_LEVEL_CALL, dest:market_1(ICLOB), function:adminCancelExpiredOrders, arguments:['ids_1', 'side_1']  
 onlyOwnerOrRoles(EXPIRED_ORDER_CLEARER)
MODIFIER_CALL, OwnableRoles.onlyOwnerOrRoles(uint256)(EXPIRED_ORDER_CLEARER_1)
```
#### CLOBManager.constructor(address,address) [PUBLIC]
```slithir
 _beacon == address(0)
TMP_1293 = CONVERT 0 to address
TMP_1294(bool) = _beacon_1 == TMP_1293
CONDITION TMP_1294
 revert InvalidBeaconAddress()()
TMP_1295(None) = SOLIDITY_CALL revert InvalidBeaconAddress()()
 beacon = _beacon
beacon_1(address) := _beacon_1(address)
 accountManager = IAccountManager(_accountManager)
TMP_1296 = CONVERT _accountManager_1 to IAccountManager
accountManager_1(IAccountManager) := TMP_1296(IAccountManager)
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### CLOBManager.createMarket(address,address,SettingsParams) [EXTERNAL]
```slithir
MARKET_MANAGER_1(uint256) := phi(['MARKET_MANAGER_10', 'MARKET_MANAGER_2', 'MARKET_MANAGER_6', 'MARKET_MANAGER_8', 'MARKET_MANAGER_0', 'MARKET_MANAGER_4'])
beacon_2(address) := phi(['beacon_9', 'beacon_0', 'beacon_1'])
accountManager_2(IAccountManager) := phi(['accountManager_0', 'accountManager_13', 'accountManager_1', 'accountManager_10'])
 _assertValidTokenPair(quoteToken,baseToken)
INTERNAL_CALL, CLOBManager._assertValidTokenPair(address,address)(quoteToken_1,baseToken_1)
 quoteDecimals = IERC20Metadata(quoteToken).decimals()
TMP_1306 = CONVERT quoteToken_1 to IERC20Metadata
TMP_1307(uint8) = HIGH_LEVEL_CALL, dest:TMP_1306(IERC20Metadata), function:decimals, arguments:[]  
beacon_5(address) := phi(['beacon_9', 'beacon_4', 'beacon_1'])
accountManager_5(IAccountManager) := phi(['accountManager_13', 'accountManager_4', 'accountManager_1', 'accountManager_10'])
quoteDecimals_1(uint8) := TMP_1307(uint8)
 baseDecimals = IERC20Metadata(baseToken).decimals()
TMP_1308 = CONVERT baseToken_1 to IERC20Metadata
TMP_1309(uint8) = HIGH_LEVEL_CALL, dest:TMP_1308(IERC20Metadata), function:decimals, arguments:[]  
beacon_6(address) := phi(['beacon_9', 'beacon_5', 'beacon_1'])
accountManager_6(IAccountManager) := phi(['accountManager_13', 'accountManager_5', 'accountManager_1', 'accountManager_10'])
baseDecimals_1(uint8) := TMP_1309(uint8)
 config.quoteToken = quoteToken
REF_602(address) -> config_0.quoteToken
config_1(ConfigParams) := phi(['config_0'])
REF_602(address) (->config_1) := quoteToken_1(address)
 config.baseToken = baseToken
REF_603(address) -> config_1.baseToken
config_2(ConfigParams) := phi(['config_1'])
REF_603(address) (->config_2) := baseToken_1(address)
 config.quoteSize = 10 ** quoteDecimals
REF_604(uint256) -> config_2.quoteSize
TMP_1310(uint256) = 10 (c)** quoteDecimals_1
config_3(ConfigParams) := phi(['config_2'])
REF_604(uint256) (->config_3) := TMP_1310(uint256)
 config.baseSize = 10 ** baseDecimals
REF_605(uint256) -> config_3.baseSize
TMP_1311(uint256) = 10 (c)** baseDecimals_1
config_4(ConfigParams) := phi(['config_3'])
REF_605(uint256) (->config_4) := TMP_1311(uint256)
 _assertValidSettings(settings,config.baseSize)
REF_606(uint256) -> config_4.baseSize
INTERNAL_CALL, CLOBManager._assertValidSettings(SettingsParams,uint256)(settings_1,REF_606)
 self = _getStorage()
TMP_1313(CLOBManagerStorage) = INTERNAL_CALL, CLOBManager._getStorage()()
self_1 (-> ['TMP_1313'])(CLOBManagerStorage) := TMP_1313(CLOBManagerStorage)
 tokenPairHash = _getTokenHash(quoteToken,baseToken)
TMP_1314(bytes32) = INTERNAL_CALL, CLOBManager._getTokenHash(address,address)(quoteToken_1,baseToken_1)
tokenPairHash_1(bytes32) := TMP_1314(bytes32)
 self.clob[tokenPairHash] > address(0)
REF_607(mapping(bytes32 => address)) -> self_1 (-> ['TMP_1313']).clob
REF_608(address) -> REF_607[tokenPairHash_1]
TMP_1315 = CONVERT 0 to address
TMP_1316(bool) = REF_608 > TMP_1315
CONDITION TMP_1316
 revert MarketExists()()
TMP_1317(None) = SOLIDITY_CALL revert MarketExists()()
 initData = abi.encodeWithSelector(CLOB.initialize.selector,MarketConfig({quoteToken:config.quoteToken,baseToken:config.baseToken,quoteSize:config.quoteSize,baseSize:config.baseSize}),MarketSettings({status:true,maxLimitsPerTx:settings.maxLimitsPerTx,minLimitOrderAmountInBase:settings.minLimitOrderAmountInBase,tickSize:settings.tickSize,lotSizeInBase:settings.lotSizeInBase}),settings.owner)
REF_611(bytes4) (->None) := 1307704337(bytes4)
REF_612(address) -> config_4.quoteToken
REF_613(address) -> config_4.baseToken
REF_614(uint256) -> config_4.quoteSize
REF_615(uint256) -> config_4.baseSize
TMP_1318(MarketConfig) = new MarketConfig(REF_612,REF_613,REF_614,REF_615)
REF_616(uint8) -> settings_1.maxLimitsPerTx
REF_617(uint256) -> settings_1.minLimitOrderAmountInBase
REF_618(uint256) -> settings_1.tickSize
REF_619(uint256) -> settings_1.lotSizeInBase
TMP_1319(MarketSettings) = new MarketSettings(True,REF_616,REF_617,REF_618,REF_619)
REF_620(address) -> settings_1.owner
TMP_1320(bytes) = SOLIDITY_CALL abi.encodeWithSelector()(REF_611,TMP_1318,TMP_1319,REF_620)
initData_1(bytes) := TMP_1320(bytes)
 marketAddress = address(new BeaconProxy(beacon,initData))
TMP_1322(BeaconProxy) = new BeaconProxy(beacon_9,initData_1) 
TMP_1323 = CONVERT TMP_1322 to address
marketAddress_1(address) := TMP_1323(address)
 self.isCLOB[marketAddress] = true
REF_621(mapping(address => bool)) -> self_1 (-> ['TMP_1313']).isCLOB
REF_622(bool) -> REF_621[marketAddress_1]
self_2 (-> ['TMP_1313'])(CLOBManagerStorage) := phi(["self_1 (-> ['TMP_1313'])"])
REF_622(bool) (->self_2 (-> ['TMP_1313'])) := True(bool)
TMP_1313(CLOBManagerStorage) := phi(["self_2 (-> ['TMP_1313'])"])
 self.clob[tokenPairHash] = marketAddress
REF_623(mapping(bytes32 => address)) -> self_2 (-> ['TMP_1313']).clob
REF_624(address) -> REF_623[tokenPairHash_1]
self_3 (-> ['TMP_1313'])(CLOBManagerStorage) := phi(["self_2 (-> ['TMP_1313'])"])
REF_624(address) (->self_3 (-> ['TMP_1313'])) := marketAddress_1(address)
TMP_1313(CLOBManagerStorage) := phi(["self_3 (-> ['TMP_1313'])"])
 accountManager.registerMarket(marketAddress)
HIGH_LEVEL_CALL, dest:accountManager_9(IAccountManager), function:registerMarket, arguments:['marketAddress_1']  
accountManager_10(IAccountManager) := phi(['accountManager_13', 'accountManager_9', 'accountManager_1', 'accountManager_10'])
 _emitMarketCreated(msg.sender,marketAddress,quoteDecimals,baseDecimals,config,settings)
INTERNAL_CALL, CLOBManager._emitMarketCreated(address,address,uint8,uint8,ConfigParams,SettingsParams)(msg.sender,marketAddress_1,quoteDecimals_1,baseDecimals_1,config_4,settings_1)
 onlyOwnerOrRoles(MARKET_MANAGER)
MODIFIER_CALL, OwnableRoles.onlyOwnerOrRoles(uint256)(MARKET_MANAGER_1)
 marketAddress
RETURN marketAddress_1
```
#### CLOBManager.getEventNonce() [EXTERNAL]
```slithir
 EventNonceLib.getCurrentNonce()
TMP_1304(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.getCurrentNonce(), arguments:[] 
RETURN TMP_1304
```
#### CLOBManager.getMarketAddress(address,address) [EXTERNAL]
```slithir
 _getStorage().clob[_getTokenHash(tokenA,tokenB)]
TMP_1300(CLOBManagerStorage) = INTERNAL_CALL, CLOBManager._getStorage()()
REF_593(mapping(bytes32 => address)) -> TMP_1300.clob
TMP_1301(bytes32) = INTERNAL_CALL, CLOBManager._getTokenHash(address,address)(tokenA_1,tokenB_1)
REF_594(address) -> REF_593[TMP_1301]
RETURN REF_594
 marketAddress
```
#### CLOBManager.getMaxLimitExempt(address) [EXTERNAL]
```slithir
 _getStorage().maxLimitWhitelist[account]
TMP_1303(CLOBManagerStorage) = INTERNAL_CALL, CLOBManager._getStorage()()
REF_597(mapping(address => bool)) -> TMP_1303.maxLimitWhitelist
REF_598(bool) -> REF_597[account_1]
RETURN REF_598
```
#### CLOBManager.initialize(address) [EXTERNAL]
```slithir
 _initializeOwner(_owner)
INTERNAL_CALL, Ownable._initializeOwner(address)(_owner_1)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### CLOBManager.isMarket(address) [EXTERNAL]
```slithir
 _getStorage().isCLOB[market]
TMP_1302(CLOBManagerStorage) = INTERNAL_CALL, CLOBManager._getStorage()()
REF_595(mapping(address => bool)) -> TMP_1302.isCLOB
REF_596(bool) -> REF_595[market_1]
RETURN REF_596
```
#### CLOBManager.setAccountFeeTiers(address[],FeeTiers[]) [EXTERNAL]
```slithir
FEE_TIER_SETTER_1(uint256) := phi(['FEE_TIER_SETTER_0', 'FEE_TIER_SETTER_2'])
accountManager_11(IAccountManager) := phi(['accountManager_0', 'accountManager_13', 'accountManager_1', 'accountManager_10'])
 accountManager.setSpotAccountFeeTiers(accounts,feeTiers)
HIGH_LEVEL_CALL, dest:accountManager_12(IAccountManager), function:setSpotAccountFeeTiers, arguments:['accounts_1', 'feeTiers_1']  
accountManager_13(IAccountManager) := phi(['accountManager_13', 'accountManager_12', 'accountManager_1', 'accountManager_10'])
 onlyOwnerOrRoles(FEE_TIER_SETTER)
MODIFIER_CALL, OwnableRoles.onlyOwnerOrRoles(uint256)(FEE_TIER_SETTER_1)
```
#### CLOBManager.setLotSizeInBase(ICLOB,uint256) [EXTERNAL]
```slithir
MARKET_MANAGER_5(uint256) := phi(['MARKET_MANAGER_10', 'MARKET_MANAGER_2', 'MARKET_MANAGER_6', 'MARKET_MANAGER_8', 'MARKET_MANAGER_0', 'MARKET_MANAGER_4'])
 market.setLotSizeInBase(newLotSize)
HIGH_LEVEL_CALL, dest:market_1(ICLOB), function:setLotSizeInBase, arguments:['newLotSize_1']  
 onlyOwnerOrRoles(MARKET_MANAGER)
MODIFIER_CALL, OwnableRoles.onlyOwnerOrRoles(uint256)(MARKET_MANAGER_5)
```
#### CLOBManager.setMaxLimitsExempt(address[],bool[]) [EXTERNAL]
```slithir
MAX_LIMIT_WHITELISTER_1(uint256) := phi(['MAX_LIMIT_WHITELISTER_2', 'MAX_LIMIT_WHITELISTER_0'])
 accounts.length != toggles.length
REF_631 -> LENGTH accounts_1
REF_632 -> LENGTH toggles_1
TMP_1337(bool) = REF_631 != REF_632
CONDITION TMP_1337
 revert AdminPanelArrayLengthsInvalid()()
TMP_1338(None) = SOLIDITY_CALL revert AdminPanelArrayLengthsInvalid()()
 self = _getStorage()
TMP_1339(CLOBManagerStorage) = INTERNAL_CALL, CLOBManager._getStorage()()
self_1 (-> ['TMP_1339'])(CLOBManagerStorage) := TMP_1339(CLOBManagerStorage)
 i = 0
i_1(uint256) := 0(uint256)
 i < accounts.length
self_2 (-> ['TMP_1339'])(CLOBManagerStorage) := phi(["self_1 (-> ['TMP_1339'])", "self_3 (-> ['TMP_1339'])"])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_633 -> LENGTH accounts_1
TMP_1340(bool) = i_2 < REF_633
CONDITION TMP_1340
 self.maxLimitWhitelist[accounts[i]] = toggles[i]
REF_634(mapping(address => bool)) -> self_2 (-> ['TMP_1339']).maxLimitWhitelist
REF_635(address) -> accounts_1[i_2]
REF_636(bool) -> REF_634[REF_635]
REF_637(bool) -> toggles_1[i_2]
self_3 (-> ['TMP_1339'])(CLOBManagerStorage) := phi(["self_2 (-> ['TMP_1339'])"])
REF_636(bool) (->self_3 (-> ['TMP_1339'])) := REF_637(bool)
TMP_1339(CLOBManagerStorage) := phi(["self_3 (-> ['TMP_1339'])"])
 i ++
TMP_1341(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 onlyOwnerOrRoles(MAX_LIMIT_WHITELISTER)
MODIFIER_CALL, OwnableRoles.onlyOwnerOrRoles(uint256)(MAX_LIMIT_WHITELISTER_1)
```
#### CLOBManager.setMaxLimitsPerTx(ICLOB,uint8) [EXTERNAL]
```slithir
MARKET_MANAGER_9(uint256) := phi(['MARKET_MANAGER_10', 'MARKET_MANAGER_2', 'MARKET_MANAGER_6', 'MARKET_MANAGER_8', 'MARKET_MANAGER_0', 'MARKET_MANAGER_4'])
 market.setMaxLimitsPerTx(newMaxLimits)
HIGH_LEVEL_CALL, dest:market_1(ICLOB), function:setMaxLimitsPerTx, arguments:['newMaxLimits_1']  
 onlyOwnerOrRoles(MARKET_MANAGER)
MODIFIER_CALL, OwnableRoles.onlyOwnerOrRoles(uint256)(MARKET_MANAGER_9)
```
#### CLOBManager.setMinLimitOrderAmountInBase(ICLOB,uint256) [EXTERNAL]
```slithir
MARKET_MANAGER_7(uint256) := phi(['MARKET_MANAGER_10', 'MARKET_MANAGER_2', 'MARKET_MANAGER_6', 'MARKET_MANAGER_8', 'MARKET_MANAGER_0', 'MARKET_MANAGER_4'])
 market.setMinLimitOrderAmountInBase(newMinLimitOrderAmountInBase)
HIGH_LEVEL_CALL, dest:market_1(ICLOB), function:setMinLimitOrderAmountInBase, arguments:['newMinLimitOrderAmountInBase_1']  
 onlyOwnerOrRoles(MARKET_MANAGER)
MODIFIER_CALL, OwnableRoles.onlyOwnerOrRoles(uint256)(MARKET_MANAGER_7)
```
#### CLOBManager.setTickSize(ICLOB,uint256) [EXTERNAL]
```slithir
MARKET_MANAGER_3(uint256) := phi(['MARKET_MANAGER_10', 'MARKET_MANAGER_2', 'MARKET_MANAGER_6', 'MARKET_MANAGER_8', 'MARKET_MANAGER_0', 'MARKET_MANAGER_4'])
 market.setTickSize(newTickSize)
HIGH_LEVEL_CALL, dest:market_1(ICLOB), function:setTickSize, arguments:['newTickSize_1']  
 onlyOwnerOrRoles(MARKET_MANAGER)
MODIFIER_CALL, OwnableRoles.onlyOwnerOrRoles(uint256)(MARKET_MANAGER_3)
```
#### CLOBManagerStorageLib.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 CLOB_MANAGER_STORAGE_POSITION = keccak256(bytes)(abi.encode(uint256(keccak256(bytes)(CLOBManagerStorage)) - 1)) & ~ bytes32(uint256(0xff))
```
#### FixedPointMathLib.fullMulDiv(uint256,uint256,uint256) [INTERNAL]
```slithir
x_1(uint256) := phi(['x_1', 'TMP_13961', 'TMP_13966', 'TMP_13980', 'TMP_13990'])
y_1(uint256) := phi(['TMP_13982', 'TMP_13992', 'y_1', 'TMP_13967', 'TMP_13962'])
d_1(uint256) := phi(['d_1', 'TMP_13984', 'TMP_13994', 'TMP_13968', 'TMP_13963'])
 z = x * y
TMP_13384(uint256) = x_1 * y_1
z_1(uint256) := TMP_13384(uint256)
 1
z_2(uint256) := phi(['z_5', 'z_1'])
CONDITION 1
 ! ! x | z / x == y * d
TMP_13385 = UnaryType.BANG x_1 
TMP_13386(uint256) = z_2 / x_1
TMP_13387(bool) = TMP_13386 == y_1
TMP_13388(uint256) = TMP_13385 | TMP_13387
TMP_13389(uint256) = TMP_13388 * d_1
TMP_13390 = UnaryType.BANG TMP_13389 
CONDITION TMP_13390
d_3(uint256) := phi(['d_2', 'd_1'])
z_4(uint256) := phi(['z_3', 'z_1'])
 mm_fullMulDiv_asm_0 = mulmod(uint256,uint256,uint256)(x,y,~ 0)
TMP_13391 = UnaryType.TILD 0 
TMP_13392(uint256) = SOLIDITY_CALL mulmod(uint256,uint256,uint256)(x_1,y_1,TMP_13391)
mm_fullMulDiv_asm_0_1(uint256) := TMP_13392(uint256)
 p1_fullMulDiv_asm_0 = mm_fullMulDiv_asm_0 - z + mm_fullMulDiv_asm_0 < z
TMP_13393(bool) = mm_fullMulDiv_asm_0_1 < z_2
TMP_13394(uint256) = z_2 + TMP_13393
TMP_13395(uint256) = mm_fullMulDiv_asm_0_1 - TMP_13394
p1_fullMulDiv_asm_0_1(uint256) := TMP_13395(uint256)
 r_fullMulDiv_asm_0 = mulmod(uint256,uint256,uint256)(x,y,d)
TMP_13396(uint256) = SOLIDITY_CALL mulmod(uint256,uint256,uint256)(x_1,y_1,d_1)
r_fullMulDiv_asm_0_1(uint256) := TMP_13396(uint256)
 t_fullMulDiv_asm_0 = d & 0 - d
TMP_13397(uint256) = 0 - d_1
TMP_13398(uint256) = d_1 & TMP_13397
t_fullMulDiv_asm_0_1(uint256) := TMP_13398(uint256)
 ! d > p1_fullMulDiv_asm_0
TMP_13399(bool) = d_1 > p1_fullMulDiv_asm_0_1
TMP_13400 = UnaryType.BANG TMP_13399 
CONDITION TMP_13400
 mstore(uint256,uint256)(0x00,0xae47f702)
TMP_13401(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,2923951874)
 revert(uint256,uint256)(0x1c,0x04)
TMP_13402(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 d = d / t_fullMulDiv_asm_0
TMP_13403(uint256) = d_1 / t_fullMulDiv_asm_0_1
d_2(uint256) := TMP_13403(uint256)
 inv_fullMulDiv_asm_0 = 2 ^ 3 * d
TMP_13404(uint256) = 3 * d_2
TMP_13405(uint256) = 2 ^ TMP_13404
inv_fullMulDiv_asm_0_1(uint256) := TMP_13405(uint256)
 inv_fullMulDiv_asm_0 = inv_fullMulDiv_asm_0 * 2 - d * inv_fullMulDiv_asm_0
TMP_13406(uint256) = d_2 * inv_fullMulDiv_asm_0_1
TMP_13407(uint256) = 2 - TMP_13406
TMP_13408(uint256) = inv_fullMulDiv_asm_0_1 * TMP_13407
inv_fullMulDiv_asm_0_2(uint256) := TMP_13408(uint256)
 inv_fullMulDiv_asm_0 = inv_fullMulDiv_asm_0 * 2 - d * inv_fullMulDiv_asm_0
TMP_13409(uint256) = d_2 * inv_fullMulDiv_asm_0_2
TMP_13410(uint256) = 2 - TMP_13409
TMP_13411(uint256) = inv_fullMulDiv_asm_0_2 * TMP_13410
inv_fullMulDiv_asm_0_3(uint256) := TMP_13411(uint256)
 inv_fullMulDiv_asm_0 = inv_fullMulDiv_asm_0 * 2 - d * inv_fullMulDiv_asm_0
TMP_13412(uint256) = d_2 * inv_fullMulDiv_asm_0_3
TMP_13413(uint256) = 2 - TMP_13412
TMP_13414(uint256) = inv_fullMulDiv_asm_0_3 * TMP_13413
inv_fullMulDiv_asm_0_4(uint256) := TMP_13414(uint256)
 inv_fullMulDiv_asm_0 = inv_fullMulDiv_asm_0 * 2 - d * inv_fullMulDiv_asm_0
TMP_13415(uint256) = d_2 * inv_fullMulDiv_asm_0_4
TMP_13416(uint256) = 2 - TMP_13415
TMP_13417(uint256) = inv_fullMulDiv_asm_0_4 * TMP_13416
inv_fullMulDiv_asm_0_5(uint256) := TMP_13417(uint256)
 inv_fullMulDiv_asm_0 = inv_fullMulDiv_asm_0 * 2 - d * inv_fullMulDiv_asm_0
TMP_13418(uint256) = d_2 * inv_fullMulDiv_asm_0_5
TMP_13419(uint256) = 2 - TMP_13418
TMP_13420(uint256) = inv_fullMulDiv_asm_0_5 * TMP_13419
inv_fullMulDiv_asm_0_6(uint256) := TMP_13420(uint256)
 z = p1_fullMulDiv_asm_0 - r_fullMulDiv_asm_0 > z * 0 - t_fullMulDiv_asm_0 / t_fullMulDiv_asm_0 + 1 | z - r_fullMulDiv_asm_0 / t_fullMulDiv_asm_0 * 2 - d * inv_fullMulDiv_asm_0 * inv_fullMulDiv_asm_0
TMP_13421(bool) = r_fullMulDiv_asm_0_1 > z_2
TMP_13422(uint256) = p1_fullMulDiv_asm_0_1 - TMP_13421
TMP_13423(uint256) = 0 - t_fullMulDiv_asm_0_1
TMP_13424(uint256) = TMP_13423 / t_fullMulDiv_asm_0_1
TMP_13425(uint256) = TMP_13424 + 1
TMP_13426(uint256) = TMP_13422 * TMP_13425
TMP_13427(uint256) = z_2 - r_fullMulDiv_asm_0_1
TMP_13428(uint256) = TMP_13427 / t_fullMulDiv_asm_0_1
TMP_13429(uint256) = TMP_13426 | TMP_13428
TMP_13430(uint256) = d_2 * inv_fullMulDiv_asm_0_6
TMP_13431(uint256) = 2 - TMP_13430
TMP_13432(uint256) = TMP_13431 * inv_fullMulDiv_asm_0_6
TMP_13433(uint256) = TMP_13429 * TMP_13432
z_3(uint256) := TMP_13433(uint256)
 z = z / d
TMP_13434(uint256) = z_4 / d_3
z_5(uint256) := TMP_13434(uint256)
 z
RETURN z_2
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
#### CLOBManagerStorageLib.getCLOBManagerStorage() [INTERNAL]
```slithir
CLOB_MANAGER_STORAGE_POSITION_1(bytes32) := phi(['CLOB_MANAGER_STORAGE_POSITION_0'])
 position = CLOB_MANAGER_STORAGE_POSITION
position_1(bytes32) := CLOB_MANAGER_STORAGE_POSITION_1(bytes32)
 self = position
self_1 (-> ['position'])(CLOBManagerStorage) := position_1(bytes32)
 self
RETURN self_1 (-> ['position'])
```

#### IAccountManager.registerMarket(address) [EXTERNAL]
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
#### IAccountManager.setSpotAccountFeeTiers(address[],FeeTiers[]) [EXTERNAL]
```slithir

```
#### ICLOB.setLotSizeInBase(uint256) [EXTERNAL]
```slithir

```
#### ICLOB.setMaxLimitsPerTx(uint8) [EXTERNAL]
```slithir

```
#### ICLOB.setMinLimitOrderAmountInBase(uint256) [EXTERNAL]
```slithir

```
#### ICLOB.setTickSize(uint256) [EXTERNAL]
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
