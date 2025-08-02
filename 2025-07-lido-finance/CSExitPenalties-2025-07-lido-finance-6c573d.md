

### Storage layout (CSExitPenalties) 

```text
_exitPenaltyInfo mapping(bytes32 => ExitPenaltyInfo)

```


#### CSExitPenalties.processExitDelayReport(uint256,bytes,uint256) [EXTERNAL]
```slithir
PARAMETERS_REGISTRY_2(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_9', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_21', 'PARAMETERS_REGISTRY_15', 'PARAMETERS_REGISTRY_17', 'PARAMETERS_REGISTRY_10', 'PARAMETERS_REGISTRY_6', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_7', 'PARAMETERS_REGISTRY_0'])
ACCOUNTING_2(ICSAccounting) := phi(['ACCOUNTING_8', 'ACCOUNTING_6', 'ACCOUNTING_15', 'ACCOUNTING_11', 'ACCOUNTING_1', 'ACCOUNTING_7', 'ACCOUNTING_0', 'ACCOUNTING_12', 'ACCOUNTING_4'])
_exitPenaltyInfo_1(mapping(bytes32 => ExitPenaltyInfo)) := phi(['_exitPenaltyInfo_9', '_exitPenaltyInfo_19', '_exitPenaltyInfo_14', '_exitPenaltyInfo_6', '_exitPenaltyInfo_8', '_exitPenaltyInfo_18', '_exitPenaltyInfo_21', '_exitPenaltyInfo_13', '_exitPenaltyInfo_5', '_exitPenaltyInfo_10', '_exitPenaltyInfo_0'])
 curveId = ACCOUNTING.getBondCurveId(nodeOperatorId)
TMP_1824(uint256) = HIGH_LEVEL_CALL, dest:ACCOUNTING_3(ICSAccounting), function:getBondCurveId, arguments:['nodeOperatorId_1']  
PARAMETERS_REGISTRY_4(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_9', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_21', 'PARAMETERS_REGISTRY_15', 'PARAMETERS_REGISTRY_17', 'PARAMETERS_REGISTRY_10', 'PARAMETERS_REGISTRY_6', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_7', 'PARAMETERS_REGISTRY_3'])
ACCOUNTING_4(ICSAccounting) := phi(['ACCOUNTING_3', 'ACCOUNTING_8', 'ACCOUNTING_6', 'ACCOUNTING_15', 'ACCOUNTING_11', 'ACCOUNTING_1', 'ACCOUNTING_7', 'ACCOUNTING_12', 'ACCOUNTING_4'])
_exitPenaltyInfo_3(mapping(bytes32 => ExitPenaltyInfo)) := phi(['_exitPenaltyInfo_9', '_exitPenaltyInfo_19', '_exitPenaltyInfo_2', '_exitPenaltyInfo_14', '_exitPenaltyInfo_6', '_exitPenaltyInfo_8', '_exitPenaltyInfo_18', '_exitPenaltyInfo_21', '_exitPenaltyInfo_13', '_exitPenaltyInfo_5', '_exitPenaltyInfo_10'])
curveId_1(uint256) := TMP_1824(uint256)
 allowedExitDelay = PARAMETERS_REGISTRY.getAllowedExitDelay(curveId)
TMP_1825(uint256) = HIGH_LEVEL_CALL, dest:PARAMETERS_REGISTRY_4(ICSParametersRegistry), function:getAllowedExitDelay, arguments:['curveId_1']  
PARAMETERS_REGISTRY_5(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_9', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_21', 'PARAMETERS_REGISTRY_15', 'PARAMETERS_REGISTRY_17', 'PARAMETERS_REGISTRY_10', 'PARAMETERS_REGISTRY_6', 'PARAMETERS_REGISTRY_4', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_7'])
_exitPenaltyInfo_4(mapping(bytes32 => ExitPenaltyInfo)) := phi(['_exitPenaltyInfo_9', '_exitPenaltyInfo_19', '_exitPenaltyInfo_3', '_exitPenaltyInfo_14', '_exitPenaltyInfo_6', '_exitPenaltyInfo_8', '_exitPenaltyInfo_18', '_exitPenaltyInfo_21', '_exitPenaltyInfo_13', '_exitPenaltyInfo_5', '_exitPenaltyInfo_10'])
allowedExitDelay_1(uint256) := TMP_1825(uint256)
 eligibleToExitInSec <= allowedExitDelay
TMP_1826(bool) = eligibleToExitInSec_1 <= allowedExitDelay_1
CONDITION TMP_1826
 revert ValidatorExitDelayNotApplicable()()
TMP_1827(None) = SOLIDITY_CALL revert ValidatorExitDelayNotApplicable()()
 keyPointer = _keyPointer(nodeOperatorId,publicKey)
TMP_1828(bytes32) = INTERNAL_CALL, CSExitPenalties._keyPointer(uint256,bytes)(nodeOperatorId_1,publicKey_1)
keyPointer_1(bytes32) := TMP_1828(bytes32)
 exitPenaltyInfo = _exitPenaltyInfo[keyPointer]
REF_662(ExitPenaltyInfo) -> _exitPenaltyInfo_5[keyPointer_1]
exitPenaltyInfo_1 (-> ['_exitPenaltyInfo'])(ExitPenaltyInfo) := REF_662(ExitPenaltyInfo)
 exitPenaltyInfo.delayPenalty.isValue
REF_663(MarkedUint248) -> exitPenaltyInfo_1 (-> ['_exitPenaltyInfo']).delayPenalty
REF_664(bool) -> REF_663.isValue
CONDITION REF_664
 delayPenalty = PARAMETERS_REGISTRY.getExitDelayPenalty(curveId)
TMP_1829(uint256) = HIGH_LEVEL_CALL, dest:PARAMETERS_REGISTRY_6(ICSParametersRegistry), function:getExitDelayPenalty, arguments:['curveId_1']  
PARAMETERS_REGISTRY_7(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_9', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_21', 'PARAMETERS_REGISTRY_15', 'PARAMETERS_REGISTRY_17', 'PARAMETERS_REGISTRY_10', 'PARAMETERS_REGISTRY_6', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_7'])
delayPenalty_1(uint256) := TMP_1829(uint256)
 exitPenaltyInfo.delayPenalty = MarkedUint248(delayPenalty.toUint248(),true)
REF_666(MarkedUint248) -> exitPenaltyInfo_1 (-> ['_exitPenaltyInfo']).delayPenalty
TMP_1830(uint248) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint248(uint256), arguments:['delayPenalty_1'] 
TMP_1831(MarkedUint248) = new MarkedUint248(TMP_1830,True)
exitPenaltyInfo_2 (-> ['_exitPenaltyInfo'])(ExitPenaltyInfo) := phi(["exitPenaltyInfo_1 (-> ['_exitPenaltyInfo'])"])
REF_666(MarkedUint248) (->exitPenaltyInfo_2 (-> ['_exitPenaltyInfo'])) := TMP_1831(MarkedUint248)
_exitPenaltyInfo_6(mapping(bytes32 => ExitPenaltyInfo)) := phi(["exitPenaltyInfo_2 (-> ['_exitPenaltyInfo'])"])
 ValidatorExitDelayProcessed(nodeOperatorId,publicKey,delayPenalty)
Emit ValidatorExitDelayProcessed(nodeOperatorId_1,publicKey_1,delayPenalty_1)
 onlyModule()
MODIFIER_CALL, CSExitPenalties.onlyModule()()
```
#### CSExitPenalties.processTriggeredExit(uint256,bytes,uint256,uint256) [EXTERNAL]
```slithir
VOLUNTARY_EXIT_TYPE_ID_1(uint8) := phi(['VOLUNTARY_EXIT_TYPE_ID_2', 'VOLUNTARY_EXIT_TYPE_ID_0'])
PARAMETERS_REGISTRY_8(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_9', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_21', 'PARAMETERS_REGISTRY_15', 'PARAMETERS_REGISTRY_17', 'PARAMETERS_REGISTRY_10', 'PARAMETERS_REGISTRY_6', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_7', 'PARAMETERS_REGISTRY_0'])
ACCOUNTING_5(ICSAccounting) := phi(['ACCOUNTING_8', 'ACCOUNTING_6', 'ACCOUNTING_15', 'ACCOUNTING_11', 'ACCOUNTING_1', 'ACCOUNTING_7', 'ACCOUNTING_0', 'ACCOUNTING_12', 'ACCOUNTING_4'])
_exitPenaltyInfo_7(mapping(bytes32 => ExitPenaltyInfo)) := phi(['_exitPenaltyInfo_9', '_exitPenaltyInfo_19', '_exitPenaltyInfo_14', '_exitPenaltyInfo_6', '_exitPenaltyInfo_8', '_exitPenaltyInfo_18', '_exitPenaltyInfo_21', '_exitPenaltyInfo_13', '_exitPenaltyInfo_5', '_exitPenaltyInfo_10', '_exitPenaltyInfo_0'])
 exitType == VOLUNTARY_EXIT_TYPE_ID
TMP_1834(bool) = exitType_1 == VOLUNTARY_EXIT_TYPE_ID_2
CONDITION TMP_1834
 keyPointer = _keyPointer(nodeOperatorId,publicKey)
TMP_1835(bytes32) = INTERNAL_CALL, CSExitPenalties._keyPointer(uint256,bytes)(nodeOperatorId_1,publicKey_1)
keyPointer_1(bytes32) := TMP_1835(bytes32)
 exitPenaltyInfo = _exitPenaltyInfo[keyPointer]
REF_668(ExitPenaltyInfo) -> _exitPenaltyInfo_9[keyPointer_1]
exitPenaltyInfo_1 (-> ['_exitPenaltyInfo'])(ExitPenaltyInfo) := REF_668(ExitPenaltyInfo)
 exitPenaltyInfo.withdrawalRequestFee.isValue
REF_669(MarkedUint248) -> exitPenaltyInfo_1 (-> ['_exitPenaltyInfo']).withdrawalRequestFee
REF_670(bool) -> REF_669.isValue
CONDITION REF_670
 curveId = ACCOUNTING.getBondCurveId(nodeOperatorId)
TMP_1836(uint256) = HIGH_LEVEL_CALL, dest:ACCOUNTING_7(ICSAccounting), function:getBondCurveId, arguments:['nodeOperatorId_1']  
PARAMETERS_REGISTRY_11(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_9', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_21', 'PARAMETERS_REGISTRY_15', 'PARAMETERS_REGISTRY_17', 'PARAMETERS_REGISTRY_10', 'PARAMETERS_REGISTRY_6', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_7'])
ACCOUNTING_8(ICSAccounting) := phi(['ACCOUNTING_8', 'ACCOUNTING_6', 'ACCOUNTING_15', 'ACCOUNTING_11', 'ACCOUNTING_1', 'ACCOUNTING_7', 'ACCOUNTING_12', 'ACCOUNTING_4'])
curveId_1(uint256) := TMP_1836(uint256)
 maxFee = PARAMETERS_REGISTRY.getMaxWithdrawalRequestFee(curveId)
TMP_1837(uint256) = HIGH_LEVEL_CALL, dest:PARAMETERS_REGISTRY_11(ICSParametersRegistry), function:getMaxWithdrawalRequestFee, arguments:['curveId_1']  
PARAMETERS_REGISTRY_12(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_9', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_21', 'PARAMETERS_REGISTRY_15', 'PARAMETERS_REGISTRY_17', 'PARAMETERS_REGISTRY_10', 'PARAMETERS_REGISTRY_6', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_7', 'PARAMETERS_REGISTRY_11'])
maxFee_1(uint256) := TMP_1837(uint256)
 fee = Math.min(withdrawalRequestPaidFee,maxFee)
TMP_1838(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['withdrawalRequestPaidFee_1', 'maxFee_1'] 
fee_1(uint256) := TMP_1838(uint256)
 exitPenaltyInfo.withdrawalRequestFee = MarkedUint248(fee.toUint248(),true)
REF_674(MarkedUint248) -> exitPenaltyInfo_1 (-> ['_exitPenaltyInfo']).withdrawalRequestFee
TMP_1839(uint248) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint248(uint256), arguments:['fee_1'] 
TMP_1840(MarkedUint248) = new MarkedUint248(TMP_1839,True)
exitPenaltyInfo_2 (-> ['_exitPenaltyInfo'])(ExitPenaltyInfo) := phi(["exitPenaltyInfo_1 (-> ['_exitPenaltyInfo'])"])
REF_674(MarkedUint248) (->exitPenaltyInfo_2 (-> ['_exitPenaltyInfo'])) := TMP_1840(MarkedUint248)
_exitPenaltyInfo_10(mapping(bytes32 => ExitPenaltyInfo)) := phi(["exitPenaltyInfo_2 (-> ['_exitPenaltyInfo'])"])
 TriggeredExitFeeRecorded({nodeOperatorId:nodeOperatorId,exitType:exitType,pubkey:publicKey,withdrawalRequestPaidFee:withdrawalRequestPaidFee,withdrawalRequestRecordedFee:fee})
Emit TriggeredExitFeeRecorded(nodeOperatorId_1,exitType_1,publicKey_1,withdrawalRequestPaidFee_1,fee_1)
 onlyModule()
MODIFIER_CALL, CSExitPenalties.onlyModule()()
```
#### CSExitPenalties.processStrikesReport(uint256,bytes) [EXTERNAL]
```slithir
PARAMETERS_REGISTRY_13(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_9', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_21', 'PARAMETERS_REGISTRY_15', 'PARAMETERS_REGISTRY_17', 'PARAMETERS_REGISTRY_10', 'PARAMETERS_REGISTRY_6', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_7', 'PARAMETERS_REGISTRY_0'])
ACCOUNTING_9(ICSAccounting) := phi(['ACCOUNTING_8', 'ACCOUNTING_6', 'ACCOUNTING_15', 'ACCOUNTING_11', 'ACCOUNTING_1', 'ACCOUNTING_7', 'ACCOUNTING_0', 'ACCOUNTING_12', 'ACCOUNTING_4'])
_exitPenaltyInfo_11(mapping(bytes32 => ExitPenaltyInfo)) := phi(['_exitPenaltyInfo_9', '_exitPenaltyInfo_19', '_exitPenaltyInfo_14', '_exitPenaltyInfo_6', '_exitPenaltyInfo_8', '_exitPenaltyInfo_18', '_exitPenaltyInfo_21', '_exitPenaltyInfo_13', '_exitPenaltyInfo_5', '_exitPenaltyInfo_10', '_exitPenaltyInfo_0'])
 keyPointer = _keyPointer(nodeOperatorId,publicKey)
TMP_1843(bytes32) = INTERNAL_CALL, CSExitPenalties._keyPointer(uint256,bytes)(nodeOperatorId_1,publicKey_1)
keyPointer_1(bytes32) := TMP_1843(bytes32)
 exitPenaltyInfo = _exitPenaltyInfo[keyPointer]
REF_676(ExitPenaltyInfo) -> _exitPenaltyInfo_13[keyPointer_1]
exitPenaltyInfo_1 (-> ['_exitPenaltyInfo'])(ExitPenaltyInfo) := REF_676(ExitPenaltyInfo)
 exitPenaltyInfo.strikesPenalty.isValue
REF_677(MarkedUint248) -> exitPenaltyInfo_1 (-> ['_exitPenaltyInfo']).strikesPenalty
REF_678(bool) -> REF_677.isValue
CONDITION REF_678
 curveId = ACCOUNTING.getBondCurveId(nodeOperatorId)
TMP_1844(uint256) = HIGH_LEVEL_CALL, dest:ACCOUNTING_11(ICSAccounting), function:getBondCurveId, arguments:['nodeOperatorId_1']  
PARAMETERS_REGISTRY_16(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_9', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_21', 'PARAMETERS_REGISTRY_15', 'PARAMETERS_REGISTRY_17', 'PARAMETERS_REGISTRY_10', 'PARAMETERS_REGISTRY_6', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_7'])
ACCOUNTING_12(ICSAccounting) := phi(['ACCOUNTING_8', 'ACCOUNTING_6', 'ACCOUNTING_15', 'ACCOUNTING_11', 'ACCOUNTING_1', 'ACCOUNTING_7', 'ACCOUNTING_12', 'ACCOUNTING_4'])
curveId_1(uint256) := TMP_1844(uint256)
 penalty = PARAMETERS_REGISTRY.getBadPerformancePenalty(curveId)
TMP_1845(uint256) = HIGH_LEVEL_CALL, dest:PARAMETERS_REGISTRY_16(ICSParametersRegistry), function:getBadPerformancePenalty, arguments:['curveId_1']  
PARAMETERS_REGISTRY_17(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_9', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_21', 'PARAMETERS_REGISTRY_15', 'PARAMETERS_REGISTRY_17', 'PARAMETERS_REGISTRY_10', 'PARAMETERS_REGISTRY_6', 'PARAMETERS_REGISTRY_16', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_7'])
penalty_1(uint256) := TMP_1845(uint256)
 exitPenaltyInfo.strikesPenalty = MarkedUint248(penalty.toUint248(),true)
REF_681(MarkedUint248) -> exitPenaltyInfo_1 (-> ['_exitPenaltyInfo']).strikesPenalty
TMP_1846(uint248) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint248(uint256), arguments:['penalty_1'] 
TMP_1847(MarkedUint248) = new MarkedUint248(TMP_1846,True)
exitPenaltyInfo_2 (-> ['_exitPenaltyInfo'])(ExitPenaltyInfo) := phi(["exitPenaltyInfo_1 (-> ['_exitPenaltyInfo'])"])
REF_681(MarkedUint248) (->exitPenaltyInfo_2 (-> ['_exitPenaltyInfo'])) := TMP_1847(MarkedUint248)
_exitPenaltyInfo_14(mapping(bytes32 => ExitPenaltyInfo)) := phi(["exitPenaltyInfo_2 (-> ['_exitPenaltyInfo'])"])
 StrikesPenaltyProcessed(nodeOperatorId,publicKey,penalty)
Emit StrikesPenaltyProcessed(nodeOperatorId_1,publicKey_1,penalty_1)
 onlyStrikes()
MODIFIER_CALL, CSExitPenalties.onlyStrikes()()
```
#### CSExitPenalties.isValidatorExitDelayPenaltyApplicable(uint256,bytes,uint256) [EXTERNAL]
```slithir
PARAMETERS_REGISTRY_18(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_9', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_21', 'PARAMETERS_REGISTRY_15', 'PARAMETERS_REGISTRY_17', 'PARAMETERS_REGISTRY_10', 'PARAMETERS_REGISTRY_6', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_7', 'PARAMETERS_REGISTRY_0'])
ACCOUNTING_13(ICSAccounting) := phi(['ACCOUNTING_8', 'ACCOUNTING_6', 'ACCOUNTING_15', 'ACCOUNTING_11', 'ACCOUNTING_1', 'ACCOUNTING_7', 'ACCOUNTING_0', 'ACCOUNTING_12', 'ACCOUNTING_4'])
_exitPenaltyInfo_15(mapping(bytes32 => ExitPenaltyInfo)) := phi(['_exitPenaltyInfo_9', '_exitPenaltyInfo_19', '_exitPenaltyInfo_14', '_exitPenaltyInfo_6', '_exitPenaltyInfo_8', '_exitPenaltyInfo_18', '_exitPenaltyInfo_21', '_exitPenaltyInfo_13', '_exitPenaltyInfo_5', '_exitPenaltyInfo_10', '_exitPenaltyInfo_0'])
 curveId = ACCOUNTING.getBondCurveId(nodeOperatorId)
TMP_1850(uint256) = HIGH_LEVEL_CALL, dest:ACCOUNTING_14(ICSAccounting), function:getBondCurveId, arguments:['nodeOperatorId_1']  
PARAMETERS_REGISTRY_20(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_9', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_21', 'PARAMETERS_REGISTRY_15', 'PARAMETERS_REGISTRY_19', 'PARAMETERS_REGISTRY_17', 'PARAMETERS_REGISTRY_10', 'PARAMETERS_REGISTRY_6', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_7'])
ACCOUNTING_15(ICSAccounting) := phi(['ACCOUNTING_8', 'ACCOUNTING_6', 'ACCOUNTING_15', 'ACCOUNTING_11', 'ACCOUNTING_14', 'ACCOUNTING_1', 'ACCOUNTING_7', 'ACCOUNTING_12', 'ACCOUNTING_4'])
_exitPenaltyInfo_17(mapping(bytes32 => ExitPenaltyInfo)) := phi(['_exitPenaltyInfo_9', '_exitPenaltyInfo_19', '_exitPenaltyInfo_14', '_exitPenaltyInfo_6', '_exitPenaltyInfo_8', '_exitPenaltyInfo_18', '_exitPenaltyInfo_21', '_exitPenaltyInfo_16', '_exitPenaltyInfo_13', '_exitPenaltyInfo_5', '_exitPenaltyInfo_10'])
curveId_1(uint256) := TMP_1850(uint256)
 allowedExitDelay = PARAMETERS_REGISTRY.getAllowedExitDelay(curveId)
TMP_1851(uint256) = HIGH_LEVEL_CALL, dest:PARAMETERS_REGISTRY_20(ICSParametersRegistry), function:getAllowedExitDelay, arguments:['curveId_1']  
PARAMETERS_REGISTRY_21(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_9', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_20', 'PARAMETERS_REGISTRY_21', 'PARAMETERS_REGISTRY_15', 'PARAMETERS_REGISTRY_17', 'PARAMETERS_REGISTRY_10', 'PARAMETERS_REGISTRY_6', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_7'])
_exitPenaltyInfo_18(mapping(bytes32 => ExitPenaltyInfo)) := phi(['_exitPenaltyInfo_9', '_exitPenaltyInfo_19', '_exitPenaltyInfo_14', '_exitPenaltyInfo_6', '_exitPenaltyInfo_8', '_exitPenaltyInfo_18', '_exitPenaltyInfo_21', '_exitPenaltyInfo_17', '_exitPenaltyInfo_13', '_exitPenaltyInfo_5', '_exitPenaltyInfo_10'])
allowedExitDelay_1(uint256) := TMP_1851(uint256)
 eligibleToExitInSec <= allowedExitDelay
TMP_1852(bool) = eligibleToExitInSec_1 <= allowedExitDelay_1
CONDITION TMP_1852
 false
RETURN False
 keyPointer = _keyPointer(nodeOperatorId,publicKey)
TMP_1853(bytes32) = INTERNAL_CALL, CSExitPenalties._keyPointer(uint256,bytes)(nodeOperatorId_1,publicKey_1)
keyPointer_1(bytes32) := TMP_1853(bytes32)
 isPenaltySet = _exitPenaltyInfo[keyPointer].delayPenalty.isValue
REF_685(ExitPenaltyInfo) -> _exitPenaltyInfo_19[keyPointer_1]
REF_686(MarkedUint248) -> REF_685.delayPenalty
REF_687(bool) -> REF_686.isValue
isPenaltySet_1(bool) := REF_687(bool)
 ! isPenaltySet
TMP_1854 = UnaryType.BANG isPenaltySet_1 
RETURN TMP_1854
 onlyModule()
MODIFIER_CALL, CSExitPenalties.onlyModule()()
```
#### CSExitPenalties.getExitPenaltyInfo(uint256,bytes) [EXTERNAL]
```slithir
_exitPenaltyInfo_20(mapping(bytes32 => ExitPenaltyInfo)) := phi(['_exitPenaltyInfo_9', '_exitPenaltyInfo_19', '_exitPenaltyInfo_14', '_exitPenaltyInfo_6', '_exitPenaltyInfo_8', '_exitPenaltyInfo_18', '_exitPenaltyInfo_21', '_exitPenaltyInfo_13', '_exitPenaltyInfo_5', '_exitPenaltyInfo_10', '_exitPenaltyInfo_0'])
 keyPointer = _keyPointer(nodeOperatorId,publicKey)
TMP_1856(bytes32) = INTERNAL_CALL, CSExitPenalties._keyPointer(uint256,bytes)(nodeOperatorId_1,publicKey_1)
keyPointer_1(bytes32) := TMP_1856(bytes32)
 _exitPenaltyInfo[keyPointer]
REF_688(ExitPenaltyInfo) -> _exitPenaltyInfo_21[keyPointer_1]
RETURN REF_688
```
#### CSExitPenalties.constructor(address,address,address) [PUBLIC]
```slithir
 module == address(0)
TMP_1812 = CONVERT 0 to address
TMP_1813(bool) = module_1 == TMP_1812
CONDITION TMP_1813
 revert ZeroModuleAddress()()
TMP_1814(None) = SOLIDITY_CALL revert ZeroModuleAddress()()
 parametersRegistry == address(0)
TMP_1815 = CONVERT 0 to address
TMP_1816(bool) = parametersRegistry_1 == TMP_1815
CONDITION TMP_1816
 revert ZeroParametersRegistryAddress()()
TMP_1817(None) = SOLIDITY_CALL revert ZeroParametersRegistryAddress()()
 strikes == address(0)
TMP_1818 = CONVERT 0 to address
TMP_1819(bool) = strikes_1 == TMP_1818
CONDITION TMP_1819
 revert ZeroStrikesAddress()()
TMP_1820(None) = SOLIDITY_CALL revert ZeroStrikesAddress()()
 MODULE = ICSModule(module)
TMP_1821 = CONVERT module_1 to ICSModule
MODULE_1(ICSModule) := TMP_1821(ICSModule)
 PARAMETERS_REGISTRY = ICSParametersRegistry(parametersRegistry)
TMP_1822 = CONVERT parametersRegistry_1 to ICSParametersRegistry
PARAMETERS_REGISTRY_1(ICSParametersRegistry) := TMP_1822(ICSParametersRegistry)
 ACCOUNTING = MODULE.accounting()
TMP_1823(ICSAccounting) = HIGH_LEVEL_CALL, dest:MODULE_1(ICSModule), function:accounting, arguments:[]  
MODULE_2(ICSModule) := phi(['MODULE_1', 'MODULE_2'])
ACCOUNTING_1(ICSAccounting) := TMP_1823(ICSAccounting)
 STRIKES = strikes
STRIKES_1(address) := strikes_1(address)
```
#### CSExitPenalties._keyPointer(uint256,bytes) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1'])
publicKey_1(bytes) := phi(['publicKey_1', 'publicKey_1', 'publicKey_1', 'publicKey_1', 'publicKey_1'])
 keccak256(bytes)(abi.encode(nodeOperatorId,publicKey))
TMP_1857(bytes) = SOLIDITY_CALL abi.encode()(nodeOperatorId_1,publicKey_1)
TMP_1858(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_1857)
RETURN TMP_1858
```
#### CSEjector.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 VOLUNTARY_EXIT_TYPE_ID = 0
 STRIKES_EXIT_TYPE_ID = 1
 DEFAULT_ADMIN_ROLE = 0x00
 RESUME_SINCE_TIMESTAMP_POSITION = keccak256(bytes)(lido.PausableUntil.resumeSinceTimestamp)
 PAUSE_INFINITELY = type()(uint256).max
 PAUSE_ROLE = keccak256(bytes)(PAUSE_ROLE)
 RESUME_ROLE = keccak256(bytes)(RESUME_ROLE)
 RECOVERER_ROLE = keccak256(bytes)(RECOVERER_ROLE)
 _checkPaused()
INTERNAL_CALL, PausableUntil._checkPaused()()
 _checkResumed()
INTERNAL_CALL, PausableUntil._checkResumed()()
role_1(bytes32) := phi(['TMP_1697', 'TMP_1694', 'RESUME_ROLE_1', 'TMP_1699', 'TMP_1692', 'PAUSE_ROLE_1'])
 _checkRole(role)
INTERNAL_CALL, AccessControl._checkRole(bytes32)(role_1)
STRIKES_2(address) := phi(['STRIKES_1', 'STRIKES_0'])
 msg.sender != STRIKES
TMP_1810(bool) = msg.sender != STRIKES_2
CONDITION TMP_1810
 revert SenderIsNotStrikes()()
TMP_1811(None) = SOLIDITY_CALL revert SenderIsNotStrikes()()
```
#### SafeCast.toUint248(uint256) [INTERNAL]
```slithir
 value > type()(uint248).max
TMP_453(uint248) := 452312848583266388373324160190187140051835877600158453279131187530910662655(uint248)
TMP_454(bool) = value_1 > TMP_453
CONDITION TMP_454
 revert SafeCastOverflowedUintDowncast(uint8,uint256)(248,value)
TMP_455(None) = SOLIDITY_CALL revert SafeCastOverflowedUintDowncast(uint8,uint256)(248,value_1)
 uint248(value)
TMP_456 = CONVERT value_1 to uint248
RETURN TMP_456
```
#### ICSParametersRegistry.getAllowedExitDelay(uint256) [EXTERNAL]
```slithir

```
#### ICSParametersRegistry.getExitDelayPenalty(uint256) [EXTERNAL]
```slithir

```
#### Math.min(uint256,uint256) [INTERNAL]
```slithir
a_1(uint256) := phi(['result_8'])
b_1(uint256) := phi(['TMP_372'])
 a < b
TMP_294(bool) = a_1 < b_1
CONDITION TMP_294
 a
RETURN a_1
 b
RETURN b_1
```
#### ICSParametersRegistry.getMaxWithdrawalRequestFee(uint256) [EXTERNAL]
```slithir

```
#### ICSParametersRegistry.getBadPerformancePenalty(uint256) [EXTERNAL]
```slithir

```
#### ICSModule.accounting() [EXTERNAL]
```slithir

```
