




### Storage layout (CSStrikes) 

```text
ejector ICSEjector
treeRoot bytes32
treeCid string

```
#### CSStrikes.setEjector(address) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_6(bytes32) := phi(['DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_7', 'DEFAULT_ADMIN_ROLE_5'])
 _setEjector(_ejector)
INTERNAL_CALL, CSStrikes._setEjector(address)(_ejector_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_6)
```
#### CSStrikes.processBadPerformanceProof(ICSStrikes.KeyStrikes[],bytes32[],bool[],address) [EXTERNAL]
```slithir
MODULE_3(ICSModule) := phi(['MODULE_4', 'MODULE_0', 'MODULE_2'])
 keyStrikesList.length == 0
REF_1391 -> LENGTH keyStrikesList_1
TMP_3361(bool) = REF_1391 == 0
CONDITION TMP_3361
 revert EmptyKeyStrikesList()()
TMP_3362(None) = SOLIDITY_CALL revert EmptyKeyStrikesList()()
 msg.value == 0
TMP_3363(bool) = msg.value == 0
CONDITION TMP_3363
 revert ZeroMsgValue()()
TMP_3364(None) = SOLIDITY_CALL revert ZeroMsgValue()()
 msg.value % keyStrikesList.length > 0
REF_1392 -> LENGTH keyStrikesList_1
TMP_3365(uint256) = msg.value % REF_1392
TMP_3366(bool) = TMP_3365 > 0
CONDITION TMP_3366
 revert ValueNotEvenlyDivisible()()
TMP_3367(None) = SOLIDITY_CALL revert ValueNotEvenlyDivisible()()
 pubkeys = new bytes[](keyStrikesList.length)
REF_1393 -> LENGTH keyStrikesList_1
TMP_3369(bytes[])  = new bytes[](REF_1393)
pubkeys_1(bytes[]) = ['TMP_3369(bytes[])']
 i < pubkeys.length
pubkeys_2(bytes[]) := phi(['pubkeys_3', 'pubkeys_1'])
i_1(uint256) := phi(['i_0', 'i_2'])
REF_1394 -> LENGTH pubkeys_2
TMP_3370(bool) = i_1 < REF_1394
CONDITION TMP_3370
 pubkeys[i] = MODULE.getSigningKeys(keyStrikesList[i].nodeOperatorId,keyStrikesList[i].keyIndex,1)
REF_1395(bytes) -> pubkeys_2[i_1]
REF_1397(ICSStrikes.KeyStrikes) -> keyStrikesList_1[i_1]
REF_1398(uint256) -> REF_1397.nodeOperatorId
REF_1399(ICSStrikes.KeyStrikes) -> keyStrikesList_1[i_1]
REF_1400(uint256) -> REF_1399.keyIndex
TMP_3371(bytes) = HIGH_LEVEL_CALL, dest:MODULE_3(ICSModule), function:getSigningKeys, arguments:['REF_1398', 'REF_1400', '1']  
MODULE_4(ICSModule) := phi(['MODULE_4', 'MODULE_2', 'MODULE_3'])
pubkeys_3(bytes[]) := phi(['pubkeys_2'])
REF_1395(bytes) (->pubkeys_3) := TMP_3371(bytes)
 ++ i
i_2(uint256) = i_1 (c)+ 1
 ! verifyProof(keyStrikesList,pubkeys,proof,proofFlags)
TMP_3372(bool) = INTERNAL_CALL, CSStrikes.verifyProof(ICSStrikes.KeyStrikes[],bytes[],bytes32[],bool[])(keyStrikesList_1,pubkeys_2,proof_1,proofFlags_1)
TMP_3373 = UnaryType.BANG TMP_3372 
CONDITION TMP_3373
 revert InvalidProof()()
TMP_3374(None) = SOLIDITY_CALL revert InvalidProof()()
 valuePerKey = msg.value / keyStrikesList.length
REF_1401 -> LENGTH keyStrikesList_1
TMP_3375(uint256) = msg.value (c)/ REF_1401
valuePerKey_1(uint256) := TMP_3375(uint256)
 i_scope_0 < keyStrikesList.length
i_scope_0_1(uint256) := phi(['i_scope_0_0', 'i_scope_0_2'])
REF_1402 -> LENGTH keyStrikesList_1
TMP_3376(bool) = i_scope_0_1 < REF_1402
CONDITION TMP_3376
 _ejectByStrikes(keyStrikesList[i_scope_0],pubkeys[i_scope_0],valuePerKey,refundRecipient)
REF_1403(ICSStrikes.KeyStrikes) -> keyStrikesList_1[i_scope_0_1]
REF_1404(bytes) -> pubkeys_2[i_scope_0_1]
INTERNAL_CALL, CSStrikes._ejectByStrikes(ICSStrikes.KeyStrikes,bytes,uint256,address)(REF_1403,REF_1404,valuePerKey_1,refundRecipient_4)
 ++ i_scope_0
i_scope_0_2(uint256) = i_scope_0_1 (c)+ 1
 refundRecipient == address(0)
TMP_3378 = CONVERT 0 to address
TMP_3379(bool) = refundRecipient_1 == TMP_3378
CONDITION TMP_3379
 refundRecipient = msg.sender
refundRecipient_2(address) := msg.sender(address)
 refundRecipient = refundRecipient
refundRecipient_3(address) := refundRecipient_1(address)
refundRecipient_4(address) := phi(['refundRecipient_2', 'refundRecipient_3'])
```
#### CSStrikes.processOracleReport(bytes32,string) [EXTERNAL]
```slithir
treeRoot_1(bytes32) := phi(['treeRoot_6', 'treeRoot_3', 'treeRoot_2', 'treeRoot_0', 'treeRoot_4'])
treeCid_1(string) := phi(['treeCid_3', 'treeCid_0', 'treeCid_2', 'treeCid_4'])
 isNewRootEmpty = _treeRoot == bytes32(0)
TMP_3341 = CONVERT 0 to bytes32
TMP_3342(bool) = _treeRoot_1 == TMP_3341
isNewRootEmpty_1(bool) := TMP_3342(bool)
 isNewCidEmpty = bytes(_treeCid).length == 0
TMP_3343 = CONVERT _treeCid_1 to bytes
REF_1390 -> LENGTH TMP_3343
TMP_3344(bool) = REF_1390 == 0
isNewCidEmpty_1(bool) := TMP_3344(bool)
 isNewRootEmpty != isNewCidEmpty
TMP_3345(bool) = isNewRootEmpty_1 != isNewCidEmpty_1
CONDITION TMP_3345
 revert InvalidReportData()()
TMP_3346(None) = SOLIDITY_CALL revert InvalidReportData()()
 isNewRootEmpty
CONDITION isNewRootEmpty_1
 treeRoot != bytes32(0)
TMP_3347 = CONVERT 0 to bytes32
TMP_3348(bool) = treeRoot_2 != TMP_3347
CONDITION TMP_3348
 delete treeRoot
treeRoot_3 = delete treeRoot_2 
 delete treeCid
treeCid_3 = delete treeCid_2 
 StrikesDataWiped()
Emit StrikesDataWiped()
 isSameRoot = _treeRoot == treeRoot
TMP_3350(bool) = _treeRoot_1 == treeRoot_2
isSameRoot_1(bool) := TMP_3350(bool)
 isSameCid = keccak256(bytes)(bytes(_treeCid)) == keccak256(bytes)(bytes(treeCid))
TMP_3351 = CONVERT _treeCid_1 to bytes
TMP_3352(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_3351)
TMP_3353 = CONVERT treeCid_2 to bytes
TMP_3354(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_3353)
TMP_3355(bool) = TMP_3352 == TMP_3354
isSameCid_1(bool) := TMP_3355(bool)
 isSameRoot != isSameCid
TMP_3356(bool) = isSameRoot_1 != isSameCid_1
CONDITION TMP_3356
 revert InvalidReportData()()
TMP_3357(None) = SOLIDITY_CALL revert InvalidReportData()()
 ! isSameRoot
TMP_3358 = UnaryType.BANG isSameRoot_1 
CONDITION TMP_3358
 treeRoot = _treeRoot
treeRoot_4(bytes32) := _treeRoot_1(bytes32)
 treeCid = _treeCid
treeCid_4(string) := _treeCid_1(string)
 StrikesDataUpdated(_treeRoot,_treeCid)
Emit StrikesDataUpdated(_treeRoot_1,_treeCid_1)
 onlyOracle()
MODIFIER_CALL, CSStrikes.onlyOracle()()
```
#### CSStrikes.verifyProof(ICSStrikes.KeyStrikes[],bytes[],bytes32[],bool[]) [EXTERNAL]
```slithir
keyStrikesList_1(ICSStrikes.KeyStrikes[]) := phi(['keyStrikesList_1'])
pubkeys_1(bytes[]) := phi(['pubkeys_2'])
proof_1(bytes32[]) := phi(['proof_1'])
proofFlags_1(bool[]) := phi(['proofFlags_1'])
treeRoot_5(bytes32) := phi(['treeRoot_6', 'treeRoot_3', 'treeRoot_2', 'treeRoot_0', 'treeRoot_4'])
 leaves = new bytes32[](keyStrikesList.length)
REF_1405 -> LENGTH keyStrikesList_1
TMP_3382(bytes32[])  = new bytes32[](REF_1405)
leaves_1(bytes32[]) = ['TMP_3382(bytes32[])']
 i < leaves.length
leaves_2(bytes32[]) := phi(['leaves_3', 'leaves_1'])
i_1(uint256) := phi(['i_0', 'i_2'])
REF_1406 -> LENGTH leaves_2
TMP_3383(bool) = i_1 < REF_1406
CONDITION TMP_3383
 leaves[i] = hashLeaf(keyStrikesList[i],pubkeys[i])
REF_1407(bytes32) -> leaves_2[i_1]
REF_1408(ICSStrikes.KeyStrikes) -> keyStrikesList_1[i_1]
REF_1409(bytes) -> pubkeys_1[i_1]
TMP_3384(bytes32) = INTERNAL_CALL, CSStrikes.hashLeaf(ICSStrikes.KeyStrikes,bytes)(REF_1408,REF_1409)
leaves_3(bytes32[]) := phi(['leaves_2'])
REF_1407(bytes32) (->leaves_3) := TMP_3384(bytes32)
 i ++
TMP_3385(uint256) := i_1(uint256)
i_2(uint256) = i_1 (c)+ 1
 MerkleProof.multiProofVerifyCalldata(proof,proofFlags,treeRoot,leaves)
TMP_3386(bool) = LIBRARY_CALL, dest:MerkleProof, function:MerkleProof.multiProofVerifyCalldata(bytes32[],bool[],bytes32,bytes32[]), arguments:['proof_1', 'proofFlags_1', 'treeRoot_5', 'leaves_2'] 
RETURN TMP_3386
```
#### CSStrikes.hashLeaf(ICSStrikes.KeyStrikes,bytes) [EXTERNAL]
```slithir
keyStrikes_1(ICSStrikes.KeyStrikes) := phi(['REF_1408'])
pubkey_1(bytes) := phi(['REF_1409'])
 keccak256(bytes)(bytes.concat(keccak256(bytes)(abi.encode(keyStrikes.nodeOperatorId,pubkey,keyStrikes.data))))
REF_1413(uint256) -> keyStrikes_1.nodeOperatorId
REF_1414(uint256[]) -> keyStrikes_1.data
TMP_3387(bytes) = SOLIDITY_CALL abi.encode()(REF_1413,pubkey_1,REF_1414)
TMP_3388(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_3387)
TMP_3389(bytes) = SOLIDITY_CALL bytes.concat()(TMP_3388)
TMP_3390(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_3389)
RETURN TMP_3390
```
#### CSStrikes.getInitializedVersion() [EXTERNAL]
```slithir
 _getInitializedVersion()
TMP_3380(uint64) = INTERNAL_CALL, Initializable._getInitializedVersion()()
RETURN TMP_3380
```
#### CSStrikes.constructor(address,address,address,address) [PUBLIC]
```slithir
 module == address(0)
TMP_3315 = CONVERT 0 to address
TMP_3316(bool) = module_1 == TMP_3315
CONDITION TMP_3316
 revert ZeroModuleAddress()()
TMP_3317(None) = SOLIDITY_CALL revert ZeroModuleAddress()()
 oracle == address(0)
TMP_3318 = CONVERT 0 to address
TMP_3319(bool) = oracle_1 == TMP_3318
CONDITION TMP_3319
 revert ZeroOracleAddress()()
TMP_3320(None) = SOLIDITY_CALL revert ZeroOracleAddress()()
 exitPenalties == address(0)
TMP_3321 = CONVERT 0 to address
TMP_3322(bool) = exitPenalties_1 == TMP_3321
CONDITION TMP_3322
 revert ZeroExitPenaltiesAddress()()
TMP_3323(None) = SOLIDITY_CALL revert ZeroExitPenaltiesAddress()()
 parametersRegistry == address(0)
TMP_3324 = CONVERT 0 to address
TMP_3325(bool) = parametersRegistry_1 == TMP_3324
CONDITION TMP_3325
 revert ZeroParametersRegistryAddress()()
TMP_3326(None) = SOLIDITY_CALL revert ZeroParametersRegistryAddress()()
 MODULE = ICSModule(module)
TMP_3327 = CONVERT module_1 to ICSModule
MODULE_1(ICSModule) := TMP_3327(ICSModule)
 ACCOUNTING = MODULE.accounting()
TMP_3328(ICSAccounting) = HIGH_LEVEL_CALL, dest:MODULE_1(ICSModule), function:accounting, arguments:[]  
MODULE_2(ICSModule) := phi(['MODULE_1', 'MODULE_4', 'MODULE_2'])
ACCOUNTING_1(ICSAccounting) := TMP_3328(ICSAccounting)
 EXIT_PENALTIES = ICSExitPenalties(exitPenalties)
TMP_3329 = CONVERT exitPenalties_1 to ICSExitPenalties
EXIT_PENALTIES_1(ICSExitPenalties) := TMP_3329(ICSExitPenalties)
 ORACLE = oracle
ORACLE_1(address) := oracle_1(address)
 PARAMETERS_REGISTRY = ICSParametersRegistry(parametersRegistry)
TMP_3330 = CONVERT parametersRegistry_1 to ICSParametersRegistry
PARAMETERS_REGISTRY_1(ICSParametersRegistry) := TMP_3330(ICSParametersRegistry)
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### CSStrikes.initialize(address,address) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_7', 'DEFAULT_ADMIN_ROLE_5'])
 admin == address(0)
TMP_3332 = CONVERT 0 to address
TMP_3333(bool) = admin_1 == TMP_3332
CONDITION TMP_3333
 revert ZeroAdminAddress()()
TMP_3334(None) = SOLIDITY_CALL revert ZeroAdminAddress()()
 _setEjector(_ejector)
INTERNAL_CALL, CSStrikes._setEjector(address)(_ejector_1)
 __AccessControlEnumerable_init()
INTERNAL_CALL, AccessControlEnumerableUpgradeable.__AccessControlEnumerable_init()()
 _grantRole(DEFAULT_ADMIN_ROLE,admin)
TMP_3337(bool) = INTERNAL_CALL, AccessControlEnumerableUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_4,admin_1)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### CSStrikes._setEjector(address) [INTERNAL]
```slithir
_ejector_1(address) := phi(['_ejector_1', '_ejector_1'])
 _ejector == address(0)
TMP_3391 = CONVERT 0 to address
TMP_3392(bool) = _ejector_1 == TMP_3391
CONDITION TMP_3392
 revert ZeroEjectorAddress()()
TMP_3393(None) = SOLIDITY_CALL revert ZeroEjectorAddress()()
 ejector = ICSEjector(_ejector)
TMP_3394 = CONVERT _ejector_1 to ICSEjector
ejector_1(ICSEjector) := TMP_3394(ICSEjector)
 EjectorSet(_ejector)
Emit EjectorSet(_ejector_1)
```
#### CSStrikes._ejectByStrikes(ICSStrikes.KeyStrikes,bytes,uint256,address) [INTERNAL]
```slithir
keyStrikes_1(ICSStrikes.KeyStrikes) := phi(['REF_1403'])
pubkey_1(bytes) := phi(['REF_1404'])
value_1(uint256) := phi(['valuePerKey_1'])
refundRecipient_1(address) := phi(['refundRecipient_4'])
ACCOUNTING_2(ICSAccounting) := phi(['ACCOUNTING_0', 'ACCOUNTING_3', 'ACCOUNTING_1'])
EXIT_PENALTIES_2(ICSExitPenalties) := phi(['EXIT_PENALTIES_1', 'EXIT_PENALTIES_6', 'EXIT_PENALTIES_0'])
PARAMETERS_REGISTRY_2(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_4', 'PARAMETERS_REGISTRY_0', 'PARAMETERS_REGISTRY_1'])
ejector_2(ICSEjector) := phi(['ejector_1', 'ejector_5', 'ejector_0'])
 strikes = 0
strikes_1(uint256) := 0(uint256)
 i < keyStrikes.data.length
strikes_2(uint256) := phi(['strikes_1', 'strikes_3'])
i_1(uint256) := phi(['i_0', 'i_2'])
REF_1415(uint256[]) -> keyStrikes_1.data
REF_1416 -> LENGTH REF_1415
TMP_3396(bool) = i_1 < REF_1416
CONDITION TMP_3396
 strikes += keyStrikes.data[i]
REF_1417(uint256[]) -> keyStrikes_1.data
REF_1418(uint256) -> REF_1417[i_1]
strikes_3(uint256) = strikes_2 (c)+ REF_1418
 ++ i
i_2(uint256) = i_1 (c)+ 1
 curveId = ACCOUNTING.getBondCurveId(keyStrikes.nodeOperatorId)
REF_1420(uint256) -> keyStrikes_1.nodeOperatorId
TMP_3397(uint256) = HIGH_LEVEL_CALL, dest:ACCOUNTING_2(ICSAccounting), function:getBondCurveId, arguments:['REF_1420']  
ACCOUNTING_3(ICSAccounting) := phi(['ACCOUNTING_3', 'ACCOUNTING_2', 'ACCOUNTING_1'])
EXIT_PENALTIES_3(ICSExitPenalties) := phi(['EXIT_PENALTIES_1', 'EXIT_PENALTIES_6', 'EXIT_PENALTIES_2'])
PARAMETERS_REGISTRY_3(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_4', 'PARAMETERS_REGISTRY_2', 'PARAMETERS_REGISTRY_1'])
ejector_3(ICSEjector) := phi(['ejector_1', 'ejector_2', 'ejector_5'])
curveId_1(uint256) := TMP_3397(uint256)
 (None,threshold) = PARAMETERS_REGISTRY.getStrikesParams(curveId)
TUPLE_16(uint256,uint256) = HIGH_LEVEL_CALL, dest:PARAMETERS_REGISTRY_3(ICSParametersRegistry), function:getStrikesParams, arguments:['curveId_1']  
EXIT_PENALTIES_4(ICSExitPenalties) := phi(['EXIT_PENALTIES_1', 'EXIT_PENALTIES_6', 'EXIT_PENALTIES_3'])
PARAMETERS_REGISTRY_4(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_4', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_3'])
ejector_4(ICSEjector) := phi(['ejector_1', 'ejector_3', 'ejector_5'])
threshold_1(uint256)= UNPACK TUPLE_16 index: 1 
 strikes < threshold
TMP_3398(bool) = strikes_2 < threshold_1
CONDITION TMP_3398
 revert NotEnoughStrikesToEject()()
TMP_3399(None) = SOLIDITY_CALL revert NotEnoughStrikesToEject()()
 ejector.ejectBadPerformer{value: value}(keyStrikes.nodeOperatorId,keyStrikes.keyIndex,refundRecipient)
REF_1423(uint256) -> keyStrikes_1.nodeOperatorId
REF_1424(uint256) -> keyStrikes_1.keyIndex
HIGH_LEVEL_CALL, dest:ejector_4(ICSEjector), function:ejectBadPerformer, arguments:['REF_1423', 'REF_1424', 'refundRecipient_1'] value:value_1 
EXIT_PENALTIES_5(ICSExitPenalties) := phi(['EXIT_PENALTIES_4', 'EXIT_PENALTIES_1', 'EXIT_PENALTIES_6'])
ejector_5(ICSEjector) := phi(['ejector_1', 'ejector_5', 'ejector_4'])
 EXIT_PENALTIES.processStrikesReport(keyStrikes.nodeOperatorId,pubkey)
REF_1426(uint256) -> keyStrikes_1.nodeOperatorId
HIGH_LEVEL_CALL, dest:EXIT_PENALTIES_5(ICSExitPenalties), function:processStrikesReport, arguments:['REF_1426', 'pubkey_1']  
EXIT_PENALTIES_6(ICSExitPenalties) := phi(['EXIT_PENALTIES_5', 'EXIT_PENALTIES_1', 'EXIT_PENALTIES_6'])
```
#### CSParametersRegistry.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
 MAX_BP = 10000
role_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_65', 'DEFAULT_ADMIN_ROLE_37', 'DEFAULT_ADMIN_ROLE_19', 'DEFAULT_ADMIN_ROLE_53', 'TMP_2968', 'DEFAULT_ADMIN_ROLE_49', 'DEFAULT_ADMIN_ROLE_85', 'DEFAULT_ADMIN_ROLE_81', 'DEFAULT_ADMIN_ROLE_45', 'DEFAULT_ADMIN_ROLE_59', 'TMP_2970', 'DEFAULT_ADMIN_ROLE_23', 'DEFAULT_ADMIN_ROLE_39', 'DEFAULT_ADMIN_ROLE_77', 'DEFAULT_ADMIN_ROLE_61', 'DEFAULT_ADMIN_ROLE_31', 'DEFAULT_ADMIN_ROLE_69', 'DEFAULT_ADMIN_ROLE_63', 'DEFAULT_ADMIN_ROLE_25', 'DEFAULT_ADMIN_ROLE_41', 'DEFAULT_ADMIN_ROLE_67', 'DEFAULT_ADMIN_ROLE_33', 'DEFAULT_ADMIN_ROLE_21', 'DEFAULT_ADMIN_ROLE_71', 'DEFAULT_ADMIN_ROLE_17', 'DEFAULT_ADMIN_ROLE_57', 'DEFAULT_ADMIN_ROLE_73', 'DEFAULT_ADMIN_ROLE_51', 'DEFAULT_ADMIN_ROLE_87', 'DEFAULT_ADMIN_ROLE_47', 'DEFAULT_ADMIN_ROLE_83', 'DEFAULT_ADMIN_ROLE_43', 'DEFAULT_ADMIN_ROLE_27', 'DEFAULT_ADMIN_ROLE_75', 'DEFAULT_ADMIN_ROLE_35', 'DEFAULT_ADMIN_ROLE_79', 'TMP_2963', 'DEFAULT_ADMIN_ROLE_55', 'TMP_2965', 'DEFAULT_ADMIN_ROLE_29'])
 _checkRole(role)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(role_1)
 $ = _getInitializableStorage()
TMP_3219(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_3219'])(Initializable.InitializableStorage) := TMP_3219(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_1340(bool) -> $_1 (-> ['TMP_3219'])._initializing
TMP_3220 = UnaryType.BANG REF_1340 
isTopLevelCall_1(bool) := TMP_3220(bool)
 initialized = $._initialized
REF_1341(uint64) -> $_1 (-> ['TMP_3219'])._initialized
initialized_1(uint64) := REF_1341(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_3221(bool) = initialized_1 == 0
TMP_3222(bool) = TMP_3221 && isTopLevelCall_1
initialSetup_1(bool) := TMP_3222(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_3223(bool) = initialized_1 == 1
TMP_3224 = CONVERT this to address
TMP_3225(bytes) = SOLIDITY_CALL code(address)(TMP_3224)
REF_1342 -> LENGTH TMP_3225
TMP_3226(bool) = REF_1342 == 0
TMP_3227(bool) = TMP_3223 && TMP_3226
construction_1(bool) := TMP_3227(bool)
 ! initialSetup && ! construction
TMP_3228 = UnaryType.BANG initialSetup_1 
TMP_3229 = UnaryType.BANG construction_1 
TMP_3230(bool) = TMP_3228 && TMP_3229
CONDITION TMP_3230
 revert InvalidInitialization()()
TMP_3231(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_1343(uint64) -> $_1 (-> ['TMP_3219'])._initialized
$_2 (-> ['TMP_3219'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_3219'])"])
REF_1343(uint64) (->$_2 (-> ['TMP_3219'])) := 1(uint256)
TMP_3219(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_3219'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_1344(bool) -> $_2 (-> ['TMP_3219'])._initializing
$_3 (-> ['TMP_3219'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_3219'])"])
REF_1344(bool) (->$_3 (-> ['TMP_3219'])) := True(bool)
TMP_3219(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_3219'])"])
$_4 (-> ['TMP_3219'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_3219'])", "$_2 (-> ['TMP_3219'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_1345(bool) -> $_4 (-> ['TMP_3219'])._initializing
$_5 (-> ['TMP_3219'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_3219'])"])
REF_1345(bool) (->$_5 (-> ['TMP_3219'])) := False(bool)
TMP_3219(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_3219'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_3233(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_3233'])(Initializable.InitializableStorage) := TMP_3233(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_1346(bool) -> $_1 (-> ['TMP_3233'])._initializing
REF_1347(uint64) -> $_1 (-> ['TMP_3233'])._initialized
TMP_3234(bool) = REF_1347 >= version_1
TMP_3235(bool) = REF_1346 || TMP_3234
CONDITION TMP_3235
 revert InvalidInitialization()()
TMP_3236(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_1348(uint64) -> $_1 (-> ['TMP_3233'])._initialized
$_2 (-> ['TMP_3233'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_3233'])"])
REF_1348(uint64) (->$_2 (-> ['TMP_3233'])) := version_1(uint64)
TMP_3233(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_3233'])"])
 $._initializing = true
REF_1349(bool) -> $_2 (-> ['TMP_3233'])._initializing
$_3 (-> ['TMP_3233'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_3233'])"])
REF_1349(bool) (->$_3 (-> ['TMP_3233'])) := True(bool)
TMP_3233(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_3233'])"])
 $._initializing = false
REF_1350(bool) -> $_3 (-> ['TMP_3233'])._initializing
$_4 (-> ['TMP_3233'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_3233'])"])
REF_1350(bool) (->$_4 (-> ['TMP_3233'])) := False(bool)
TMP_3233(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_3233'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
```
#### ICSModule.getSigningKeys(uint256,uint256,uint256) [EXTERNAL]
```slithir

```
#### MerkleProof.multiProofVerifyCalldata(bytes32[],bool[],bytes32,bytes32[]) [INTERNAL]
```slithir
 processMultiProofCalldata(proof,proofFlags,leaves) == root
TMP_229(bytes32) = INTERNAL_CALL, MerkleProof.processMultiProofCalldata(bytes32[],bool[],bytes32[])(proof_1,proofFlags_1,leaves_1)
TMP_230(bool) = TMP_229 == root_1
RETURN TMP_230
```
#### ICSModule.accounting() [EXTERNAL]
```slithir

```
#### ICSEjector.ejectBadPerformer(uint256,uint256,address) [EXTERNAL]
```slithir

```
#### ICSExitPenalties.processStrikesReport(uint256,bytes) [EXTERNAL]
```slithir

```
#### ICSParametersRegistry.getStrikesParams(uint256) [EXTERNAL]
```slithir

```
#### MerkleProof.processMultiProofCalldata(bytes32[],bool[],bytes32[]) [INTERNAL]
```slithir
proof_1(bytes32[]) := phi(['proof_1'])
proofFlags_1(bool[]) := phi(['proofFlags_1'])
leaves_1(bytes32[]) := phi(['leaves_1'])
 leavesLen = leaves.length
REF_103 -> LENGTH leaves_1
leavesLen_1(uint256) := REF_103(uint256)
 proofLen = proof.length
REF_104 -> LENGTH proof_1
proofLen_1(uint256) := REF_104(uint256)
 totalHashes = proofFlags.length
REF_105 -> LENGTH proofFlags_1
totalHashes_1(uint256) := REF_105(uint256)
 leavesLen + proofLen != totalHashes + 1
TMP_252(uint256) = leavesLen_1 (c)+ proofLen_1
TMP_253(uint256) = totalHashes_1 (c)+ 1
TMP_254(bool) = TMP_252 != TMP_253
CONDITION TMP_254
 revert MerkleProofInvalidMultiproof()()
TMP_255(None) = SOLIDITY_CALL revert MerkleProofInvalidMultiproof()()
 hashes = new bytes32[](totalHashes)
TMP_257(bytes32[])  = new bytes32[](totalHashes_1)
hashes_1(bytes32[]) = ['TMP_257(bytes32[])']
 leafPos = 0
leafPos_1(uint256) := 0(uint256)
 hashPos = 0
hashPos_1(uint256) := 0(uint256)
 proofPos = 0
proofPos_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < totalHashes
hashes_2(bytes32[]) := phi(['hashes_1', 'hashes_3'])
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_258(bool) = i_2 < totalHashes_1
CONDITION TMP_258
 hashes[i] = _hashPair(a,b)
REF_106(bytes32) -> hashes_2[i_2]
TMP_259(bytes32) = INTERNAL_CALL, MerkleProof._hashPair(bytes32,bytes32)(a_3,b_5)
hashes_3(bytes32[]) := phi(['hashes_2'])
REF_106(bytes32) (->hashes_3) := TMP_259(bytes32)
 i ++
TMP_260(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 totalHashes > 0
TMP_261(bool) = totalHashes_1 > 0
CONDITION TMP_261
 proofPos != proofLen
TMP_262(bool) = proofPos_1 != proofLen_1
CONDITION TMP_262
 revert MerkleProofInvalidMultiproof()()
TMP_263(None) = SOLIDITY_CALL revert MerkleProofInvalidMultiproof()()
 hashes[totalHashes - 1]
TMP_264(uint256) = totalHashes_1 - 1
REF_107(bytes32) -> hashes_2[TMP_264]
RETURN REF_107
 leavesLen > 0
TMP_265(bool) = leavesLen_1 > 0
CONDITION TMP_265
 leaves[0]
REF_108(bytes32) -> leaves_1[0]
RETURN REF_108
 proof[0]
REF_109(bytes32) -> proof_1[0]
RETURN REF_109
 leafPos < leavesLen
TMP_266(bool) = leafPos_1 < leavesLen_1
CONDITION TMP_266
 a = leaves[leafPos ++]
TMP_267(uint256) := leafPos_1(uint256)
leafPos_2(uint256) = leafPos_1 (c)+ 1
REF_110(bytes32) -> leaves_1[TMP_267]
a_1(bytes32) := REF_110(bytes32)
 a = hashes[hashPos ++]
TMP_268(uint256) := hashPos_1(uint256)
hashPos_2(uint256) = hashPos_1 (c)+ 1
REF_111(bytes32) -> hashes_2[TMP_268]
a_2(bytes32) := REF_111(bytes32)
leafPos_3(uint256) := phi(['leafPos_2', 'leafPos_1'])
hashPos_3(uint256) := phi(['hashPos_1', 'hashPos_2'])
a_3(bytes32) := phi(['a_1', 'a_2'])
 proofFlags[i]
REF_112(bool) -> proofFlags_1[i_2]
CONDITION REF_112
 b = proof[proofPos ++]
TMP_269(uint256) := proofPos_1(uint256)
proofPos_2(uint256) = proofPos_1 (c)+ 1
REF_113(bytes32) -> proof_1[TMP_269]
b_4(bytes32) := REF_113(bytes32)
proofPos_3(uint256) := phi(['proofPos_1', 'proofPos_2'])
b_5(bytes32) := phi(['b_0', 'b_4'])
 leafPos < leavesLen
TMP_270(bool) = leafPos_3 < leavesLen_1
CONDITION TMP_270
 b = leaves[leafPos ++]
TMP_271(uint256) := leafPos_3(uint256)
leafPos_4(uint256) = leafPos_3 (c)+ 1
REF_114(bytes32) -> leaves_1[TMP_271]
b_1(bytes32) := REF_114(bytes32)
 b = hashes[hashPos ++]
TMP_272(uint256) := hashPos_3(uint256)
hashPos_4(uint256) = hashPos_3 (c)+ 1
REF_115(bytes32) -> hashes_2[TMP_272]
b_2(bytes32) := REF_115(bytes32)
leafPos_5(uint256) := phi(['leafPos_4', 'leafPos_1'])
hashPos_5(uint256) := phi(['hashPos_4', 'hashPos_1'])
b_3(bytes32) := phi(['b_1', 'b_2'])
 merkleRoot
```
#### MerkleProof._hashPair(bytes32,bytes32) [PRIVATE]
```slithir
a_1(bytes32) := phi(['computedHash_2', 'a_3', 'a_3', 'computedHash_2'])
b_1(bytes32) := phi(['REF_89', 'b_5', 'REF_87', 'b_5'])
 a < b
TMP_273(bool) = a_1 < b_1
CONDITION TMP_273
 _efficientHash(a,b)
TMP_274(bytes32) = INTERNAL_CALL, MerkleProof._efficientHash(bytes32,bytes32)(a_1,b_1)
RETURN TMP_274
 _efficientHash(b,a)
TMP_275(bytes32) = INTERNAL_CALL, MerkleProof._efficientHash(bytes32,bytes32)(b_1,a_1)
RETURN TMP_275
```
