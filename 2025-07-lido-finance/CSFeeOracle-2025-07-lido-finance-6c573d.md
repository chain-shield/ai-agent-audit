

### Storage layout (CSFeeOracle) 

```text
_feeDistributor ICSFeeDistributor
_avgPerfLeewayBP uint256

```
#### CSFeeOracle._onlyRecoverer() [INTERNAL]
```slithir
RECOVERER_ROLE_1(bytes32) := phi(['RECOVERER_ROLE_0', 'RECOVERER_ROLE_2'])
 _checkRole(RECOVERER_ROLE)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(RECOVERER_ROLE_1)
```
#### CSFeeOracle.constructor(address,address,uint256,uint256) [INTERNAL]
```slithir
 feeDistributor == address(0)
TMP_2311 = CONVERT 0 to address
TMP_2312(bool) = feeDistributor_1 == TMP_2311
CONDITION TMP_2312
 revert ZeroFeeDistributorAddress()()
TMP_2313(None) = SOLIDITY_CALL revert ZeroFeeDistributorAddress()()
 strikes == address(0)
TMP_2314 = CONVERT 0 to address
TMP_2315(bool) = strikes_1 == TMP_2314
CONDITION TMP_2315
 revert ZeroStrikesAddress()()
TMP_2316(None) = SOLIDITY_CALL revert ZeroStrikesAddress()()
 FEE_DISTRIBUTOR = ICSFeeDistributor(feeDistributor)
TMP_2317 = CONVERT feeDistributor_1 to ICSFeeDistributor
FEE_DISTRIBUTOR_1(ICSFeeDistributor) := TMP_2317(ICSFeeDistributor)
 STRIKES = ICSStrikes(strikes)
TMP_2318 = CONVERT strikes_1 to ICSStrikes
STRIKES_1(ICSStrikes) := TMP_2318(ICSStrikes)
 BaseOracle(secondsPerSlot,genesisTime)
INTERNAL_CALL, BaseOracle.constructor(uint256,uint256)(secondsPerSlot_1,genesisTime_1)
```
#### CSFeeOracle._handleConsensusReport(BaseOracle.ConsensusReport,uint256,uint256) [INTERNAL]
```slithir

```
#### CSFeeOracle.submitReportData(ICSFeeOracle.ReportData,uint256) [EXTERNAL]
```slithir
 _checkMsgSenderIsAllowedToSubmitData()
INTERNAL_CALL, CSFeeOracle._checkMsgSenderIsAllowedToSubmitData()()
 _checkContractVersion(contractVersion)
INTERNAL_CALL, Versioned._checkContractVersion(uint256)(contractVersion_1)
 _checkConsensusData(data.refSlot,data.consensusVersion,keccak256(bytes)(abi.encode(data)))
REF_862(uint256) -> data_1.refSlot
REF_863(uint256) -> data_1.consensusVersion
TMP_2334(bytes) = SOLIDITY_CALL abi.encode()(data_1)
TMP_2335(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_2334)
INTERNAL_CALL, BaseOracle._checkConsensusData(uint256,uint256,bytes32)(REF_862,REF_863,TMP_2335)
 _startProcessing()
TMP_2337(uint256) = INTERNAL_CALL, BaseOracle._startProcessing()()
 _handleConsensusReportData(data)
INTERNAL_CALL, CSFeeOracle._handleConsensusReportData(ICSFeeOracle.ReportData)(data_1)
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
```
#### CSFeeOracle.resume() [EXTERNAL]
```slithir
RESUME_ROLE_1(bytes32) := phi(['RESUME_ROLE_2', 'RESUME_ROLE_0'])
 _resume()
INTERNAL_CALL, PausableUntil._resume()()
 onlyRole(RESUME_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(RESUME_ROLE_1)
```
#### CSFeeOracle.pauseFor(uint256) [EXTERNAL]
```slithir
PAUSE_ROLE_1(bytes32) := phi(['PAUSE_ROLE_0', 'PAUSE_ROLE_2'])
 _pauseFor(duration)
INTERNAL_CALL, PausableUntil._pauseFor(uint256)(duration_1)
 onlyRole(PAUSE_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(PAUSE_ROLE_1)
```
#### CSFeeOracle.initialize(address,address,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_2', 'DEFAULT_ADMIN_ROLE_0'])
 admin == address(0)
TMP_2320 = CONVERT 0 to address
TMP_2321(bool) = admin_1 == TMP_2320
CONDITION TMP_2321
 revert ZeroAdminAddress()()
TMP_2322(None) = SOLIDITY_CALL revert ZeroAdminAddress()()
 _grantRole(DEFAULT_ADMIN_ROLE,admin)
TMP_2323(bool) = INTERNAL_CALL, AccessControlEnumerableUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_1,admin_1)
 BaseOracle._initialize(consensusContract,consensusVersion,0)
INTERNAL_CALL, BaseOracle._initialize(address,uint256,uint256)(consensusContract_1,consensusVersion_1,0)
 _updateContractVersion(2)
INTERNAL_CALL, Versioned._updateContractVersion(uint256)(2)
```
#### CSFeeOracle.finalizeUpgradeV2(uint256) [EXTERNAL]
```slithir
 _setConsensusVersion(consensusVersion)
INTERNAL_CALL, BaseOracle._setConsensusVersion(uint256)(consensusVersion_1)
 sstore(uint256,uint256)(_feeDistributor,0x00)
_feeDistributor_1(ICSFeeDistributor) := 0(uint256)
 sstore(uint256,uint256)(_avgPerfLeewayBP,0x00)
_avgPerfLeewayBP_1(uint256) := 0(uint256)
 _updateContractVersion(2)
INTERNAL_CALL, Versioned._updateContractVersion(uint256)(2)
```
#### CSFeeOracle._handleConsensusReportData(ICSFeeOracle.ReportData) [INTERNAL]
```slithir
data_1(ICSFeeOracle.ReportData) := phi(['data_1'])
FEE_DISTRIBUTOR_2(ICSFeeDistributor) := phi(['FEE_DISTRIBUTOR_1', 'FEE_DISTRIBUTOR_0', 'FEE_DISTRIBUTOR_3'])
STRIKES_2(ICSStrikes) := phi(['STRIKES_1', 'STRIKES_4', 'STRIKES_0'])
 FEE_DISTRIBUTOR.processOracleReport({_treeRoot:data.treeRoot,_treeCid:data.treeCid,_logCid:data.logCid,distributed:data.distributed,rebate:data.rebate,refSlot:data.refSlot})
REF_866(bytes32) -> data_1.treeRoot
REF_867(string) -> data_1.treeCid
REF_868(string) -> data_1.logCid
REF_869(uint256) -> data_1.distributed
REF_870(uint256) -> data_1.rebate
REF_871(uint256) -> data_1.refSlot
HIGH_LEVEL_CALL, dest:FEE_DISTRIBUTOR_2(ICSFeeDistributor), function:processOracleReport, arguments:['REF_866', 'REF_867', 'REF_868', 'REF_869', 'REF_870', 'REF_871']  
FEE_DISTRIBUTOR_3(ICSFeeDistributor) := phi(['FEE_DISTRIBUTOR_2', 'FEE_DISTRIBUTOR_1', 'FEE_DISTRIBUTOR_3'])
STRIKES_3(ICSStrikes) := phi(['STRIKES_2', 'STRIKES_1', 'STRIKES_4'])
 STRIKES.processOracleReport(data.strikesTreeRoot,data.strikesTreeCid)
REF_873(bytes32) -> data_1.strikesTreeRoot
REF_874(string) -> data_1.strikesTreeCid
HIGH_LEVEL_CALL, dest:STRIKES_3(ICSStrikes), function:processOracleReport, arguments:['REF_873', 'REF_874']  
STRIKES_4(ICSStrikes) := phi(['STRIKES_1', 'STRIKES_4', 'STRIKES_3'])
```
#### CSFeeOracle._checkMsgSenderIsAllowedToSubmitData() [INTERNAL]
```slithir
SUBMIT_DATA_ROLE_1(bytes32) := phi(['SUBMIT_DATA_ROLE_0', 'SUBMIT_DATA_ROLE_3'])
 _isConsensusMember(msg.sender) || hasRole(SUBMIT_DATA_ROLE,msg.sender)
TMP_2342(bool) = INTERNAL_CALL, BaseOracle._isConsensusMember(address)(msg.sender)
TMP_2343(bool) = INTERNAL_CALL, AccessControlUpgradeable.hasRole(bytes32,address)(SUBMIT_DATA_ROLE_2,msg.sender)
TMP_2344(bool) = TMP_2342 || TMP_2343
CONDITION TMP_2344
 revert SenderNotAllowed()()
TMP_2345(None) = SOLIDITY_CALL revert SenderNotAllowed()()
```
#### CSFeeDistributor.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
 RECOVERER_ROLE = keccak256(bytes)(RECOVERER_ROLE)
role_1(bytes32) := phi(['TMP_1902', 'TMP_1907', 'TMP_1904', 'TMP_1909', 'DEFAULT_ADMIN_ROLE_6'])
 _checkRole(role)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(role_1)
 $ = _getInitializableStorage()
TMP_2048(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_2048'])(Initializable.InitializableStorage) := TMP_2048(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_747(bool) -> $_1 (-> ['TMP_2048'])._initializing
TMP_2049 = UnaryType.BANG REF_747 
isTopLevelCall_1(bool) := TMP_2049(bool)
 initialized = $._initialized
REF_748(uint64) -> $_1 (-> ['TMP_2048'])._initialized
initialized_1(uint64) := REF_748(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_2050(bool) = initialized_1 == 0
TMP_2051(bool) = TMP_2050 && isTopLevelCall_1
initialSetup_1(bool) := TMP_2051(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_2052(bool) = initialized_1 == 1
TMP_2053 = CONVERT this to address
TMP_2054(bytes) = SOLIDITY_CALL code(address)(TMP_2053)
REF_749 -> LENGTH TMP_2054
TMP_2055(bool) = REF_749 == 0
TMP_2056(bool) = TMP_2052 && TMP_2055
construction_1(bool) := TMP_2056(bool)
 ! initialSetup && ! construction
TMP_2057 = UnaryType.BANG initialSetup_1 
TMP_2058 = UnaryType.BANG construction_1 
TMP_2059(bool) = TMP_2057 && TMP_2058
CONDITION TMP_2059
 revert InvalidInitialization()()
TMP_2060(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_750(uint64) -> $_1 (-> ['TMP_2048'])._initialized
$_2 (-> ['TMP_2048'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_2048'])"])
REF_750(uint64) (->$_2 (-> ['TMP_2048'])) := 1(uint256)
TMP_2048(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_2048'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_751(bool) -> $_2 (-> ['TMP_2048'])._initializing
$_3 (-> ['TMP_2048'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_2048'])"])
REF_751(bool) (->$_3 (-> ['TMP_2048'])) := True(bool)
TMP_2048(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_2048'])"])
$_4 (-> ['TMP_2048'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_2048'])", "$_2 (-> ['TMP_2048'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_752(bool) -> $_4 (-> ['TMP_2048'])._initializing
$_5 (-> ['TMP_2048'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_2048'])"])
REF_752(bool) (->$_5 (-> ['TMP_2048'])) := False(bool)
TMP_2048(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_2048'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_2062(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_2062'])(Initializable.InitializableStorage) := TMP_2062(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_753(bool) -> $_1 (-> ['TMP_2062'])._initializing
REF_754(uint64) -> $_1 (-> ['TMP_2062'])._initialized
TMP_2063(bool) = REF_754 >= version_1
TMP_2064(bool) = REF_753 || TMP_2063
CONDITION TMP_2064
 revert InvalidInitialization()()
TMP_2065(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_755(uint64) -> $_1 (-> ['TMP_2062'])._initialized
$_2 (-> ['TMP_2062'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_2062'])"])
REF_755(uint64) (->$_2 (-> ['TMP_2062'])) := version_1(uint64)
TMP_2062(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_2062'])"])
 $._initializing = true
REF_756(bool) -> $_2 (-> ['TMP_2062'])._initializing
$_3 (-> ['TMP_2062'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_2062'])"])
REF_756(bool) (->$_3 (-> ['TMP_2062'])) := True(bool)
TMP_2062(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_2062'])"])
 $._initializing = false
REF_757(bool) -> $_3 (-> ['TMP_2062'])._initializing
$_4 (-> ['TMP_2062'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_2062'])"])
REF_757(bool) (->$_4 (-> ['TMP_2062'])) := False(bool)
TMP_2062(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_2062'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
ACCOUNTING_6(address) := phi(['ACCOUNTING_4', 'ACCOUNTING_5', 'ACCOUNTING_0', 'ACCOUNTING_1'])
 msg.sender != ACCOUNTING
TMP_2068(bool) = msg.sender != ACCOUNTING_6
CONDITION TMP_2068
 revert SenderIsNotAccounting()()
TMP_2069(None) = SOLIDITY_CALL revert SenderIsNotAccounting()()
ORACLE_2(address) := phi(['ORACLE_0', 'ORACLE_1'])
 msg.sender != ORACLE
TMP_2070(bool) = msg.sender != ORACLE_2
CONDITION TMP_2070
 revert SenderIsNotOracle()()
TMP_2071(None) = SOLIDITY_CALL revert SenderIsNotOracle()()
```
#### ICSStrikes.processOracleReport(bytes32,string) [EXTERNAL]
```slithir

```
#### ICSFeeDistributor.processOracleReport(bytes32,string,string,uint256,uint256,uint256) [EXTERNAL]
```slithir

```
