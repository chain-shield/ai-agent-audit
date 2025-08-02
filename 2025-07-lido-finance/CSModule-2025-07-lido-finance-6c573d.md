







### Storage layout (CSModule) 

```text
_queueByPriority mapping(uint256 => QueueLib.Queue)
_legacyQueue QueueLib.Queue
_accountingOld ICSAccounting
_earlyAdoption address
_publicRelease bool
_nonce uint256
_nodeOperators mapping(uint256 => NodeOperator)
_isValidatorWithdrawn mapping(uint256 => bool)
_isValidatorSlashed mapping(uint256 => bool)
_totalDepositedValidators uint64
_totalExitedValidators uint64
_depositableValidatorsCount uint64
_nodeOperatorsCount uint64

```



#### CSModule._onlyRecoverer() [INTERNAL]
```slithir
RECOVERER_ROLE_1(bytes32) := phi(['RECOVERER_ROLE_2', 'RECOVERER_ROLE_0'])
 _checkRole(RECOVERER_ROLE)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(RECOVERER_ROLE_1)
```
#### CSModule.accounting() [EXTERNAL]
```slithir
ACCOUNTING_3(ICSAccounting) := phi(['ACCOUNTING_2', 'ACCOUNTING_0'])
 ACCOUNTING
RETURN ACCOUNTING_3
```
#### CSModule.pauseFor(uint256) [EXTERNAL]
```slithir
PAUSE_ROLE_1(bytes32) := phi(['PAUSE_ROLE_2', 'PAUSE_ROLE_0'])
 _pauseFor(duration)
INTERNAL_CALL, PausableUntil._pauseFor(uint256)(duration_1)
 onlyRole(PAUSE_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(PAUSE_ROLE_1)
```
#### CSModule.resume() [EXTERNAL]
```slithir
RESUME_ROLE_1(bytes32) := phi(['RESUME_ROLE_2', 'RESUME_ROLE_0'])
 _resume()
INTERNAL_CALL, PausableUntil._resume()()
 onlyRole(RESUME_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(RESUME_ROLE_1)
```
#### CSModule.getInitializedVersion() [EXTERNAL]
```slithir
 _getInitializedVersion()
TMP_2754(uint64) = INTERNAL_CALL, Initializable._getInitializedVersion()()
RETURN TMP_2754
```
#### CSModule.createNodeOperator(address,NodeOperatorManagementProperties,address) [EXTERNAL]
```slithir
CREATE_NODE_OPERATOR_ROLE_1(bytes32) := phi(['CREATE_NODE_OPERATOR_ROLE_2', 'CREATE_NODE_OPERATOR_ROLE_0', 'CREATE_NODE_OPERATOR_ROLE_4'])
_nodeOperators_1(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
_nodeOperatorsCount_1(uint64) := phi(['_nodeOperatorsCount_0', '_nodeOperatorsCount_5'])
 from == address(0)
TMP_2536 = CONVERT 0 to address
TMP_2537(bool) = from_1 == TMP_2536
CONDITION TMP_2537
 revert ZeroSenderAddress()()
TMP_2538(None) = SOLIDITY_CALL revert ZeroSenderAddress()()
 nodeOperatorId = _nodeOperatorsCount
nodeOperatorId_1(uint256) := _nodeOperatorsCount_3(uint64)
 _recordOperatorCreator(nodeOperatorId)
INTERNAL_CALL, CSModule._recordOperatorCreator(uint256)(nodeOperatorId_1)
 no = _nodeOperators[nodeOperatorId]
REF_938(NodeOperator) -> _nodeOperators_4[nodeOperatorId_1]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_938(NodeOperator)
 no.managerAddress = managerAddress
REF_939(address) -> no_1 (-> ['_nodeOperators']).managerAddress
no_2 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['_nodeOperators'])"])
REF_939(address) (->no_2 (-> ['_nodeOperators'])) := managerAddress_3(address)
_nodeOperators_5(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['_nodeOperators'])"])
 no.rewardAddress = rewardAddress
REF_940(address) -> no_2 (-> ['_nodeOperators']).rewardAddress
no_3 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_2 (-> ['_nodeOperators'])"])
REF_940(address) (->no_3 (-> ['_nodeOperators'])) := rewardAddress_3(address)
_nodeOperators_6(mapping(uint256 => NodeOperator)) := phi(["no_3 (-> ['_nodeOperators'])"])
 managementProperties.extendedManagerPermissions
REF_941(bool) -> managementProperties_1.extendedManagerPermissions
CONDITION REF_941
 no.extendedManagerPermissions = true
REF_942(bool) -> no_3 (-> ['_nodeOperators']).extendedManagerPermissions
no_4 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_3 (-> ['_nodeOperators'])"])
REF_942(bool) (->no_4 (-> ['_nodeOperators'])) := True(bool)
_nodeOperators_7(mapping(uint256 => NodeOperator)) := phi(["no_4 (-> ['_nodeOperators'])"])
 ++ _nodeOperatorsCount
_nodeOperatorsCount_5(uint64) = _nodeOperatorsCount_4 + 1
 NodeOperatorAdded(nodeOperatorId,managerAddress,rewardAddress,managementProperties.extendedManagerPermissions)
REF_943(bool) -> managementProperties_1.extendedManagerPermissions
Emit NodeOperatorAdded(nodeOperatorId_1,managerAddress_3,rewardAddress_3,REF_943)
 referrer != address(0)
TMP_2541 = CONVERT 0 to address
TMP_2542(bool) = referrer_1 != TMP_2541
CONDITION TMP_2542
 ReferrerSet(nodeOperatorId,referrer)
Emit ReferrerSet(nodeOperatorId_1,referrer_1)
 _incrementModuleNonce()
INTERNAL_CALL, CSModule._incrementModuleNonce()()
 onlyRole(CREATE_NODE_OPERATOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(CREATE_NODE_OPERATOR_ROLE_1)
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
 managementProperties.managerAddress == address(0)
REF_944(address) -> managementProperties_1.managerAddress
TMP_2547 = CONVERT 0 to address
TMP_2548(bool) = REF_944 == TMP_2547
CONDITION TMP_2548
 managerAddress = from
managerAddress_1(address) := from_1(address)
 managerAddress = managementProperties.managerAddress
REF_945(address) -> managementProperties_1.managerAddress
managerAddress_2(address) := REF_945(address)
managerAddress_3(address) := phi(['managerAddress_1', 'managerAddress_2'])
 managementProperties.rewardAddress == address(0)
REF_946(address) -> managementProperties_1.rewardAddress
TMP_2549 = CONVERT 0 to address
TMP_2550(bool) = REF_946 == TMP_2549
CONDITION TMP_2550
 rewardAddress = from
rewardAddress_1(address) := from_1(address)
 rewardAddress = managementProperties.rewardAddress
REF_947(address) -> managementProperties_1.rewardAddress
rewardAddress_2(address) := REF_947(address)
rewardAddress_3(address) := phi(['rewardAddress_1', 'rewardAddress_2'])
 nodeOperatorId
RETURN nodeOperatorId_1
```
#### CSModule.addValidatorKeysETH(address,uint256,uint256,bytes,bytes) [EXTERNAL]
```slithir
 _checkCanAddKeys(nodeOperatorId,from)
INTERNAL_CALL, CSModule._checkCanAddKeys(uint256,address)(nodeOperatorId_1,from_1)
 msg.value < accounting().getRequiredBondForNextKeys(nodeOperatorId,keysCount)
TMP_2552(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
TMP_2553(uint256) = HIGH_LEVEL_CALL, dest:TMP_2552(ICSAccounting), function:getRequiredBondForNextKeys, arguments:['nodeOperatorId_1', 'keysCount_1']  
TMP_2554(bool) = msg.value < TMP_2553
CONDITION TMP_2554
 revert InvalidAmount()()
TMP_2555(None) = SOLIDITY_CALL revert InvalidAmount()()
 msg.value != 0
TMP_2556(bool) = msg.value != 0
CONDITION TMP_2556
 accounting().depositETH{value: msg.value}(from,nodeOperatorId)
TMP_2557(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
HIGH_LEVEL_CALL, dest:TMP_2557(ICSAccounting), function:depositETH, arguments:['from_1', 'nodeOperatorId_1'] value:msg.value 
 _addKeysAndUpdateDepositableValidatorsCount(nodeOperatorId,keysCount,publicKeys,signatures)
INTERNAL_CALL, CSModule._addKeysAndUpdateDepositableValidatorsCount(uint256,uint256,bytes,bytes)(nodeOperatorId_1,keysCount_1,publicKeys_1,signatures_1)
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
```
#### CSModule.addValidatorKeysStETH(address,uint256,uint256,bytes,bytes,ICSAccounting.PermitInput) [EXTERNAL]
```slithir
 _checkCanAddKeys(nodeOperatorId,from)
INTERNAL_CALL, CSModule._checkCanAddKeys(uint256,address)(nodeOperatorId_1,from_1)
 amount = accounting().getRequiredBondForNextKeys(nodeOperatorId,keysCount)
TMP_2562(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
TMP_2563(uint256) = HIGH_LEVEL_CALL, dest:TMP_2562(ICSAccounting), function:getRequiredBondForNextKeys, arguments:['nodeOperatorId_1', 'keysCount_1']  
amount_1(uint256) := TMP_2563(uint256)
 amount != 0
TMP_2564(bool) = amount_1 != 0
CONDITION TMP_2564
 accounting().depositStETH(from,nodeOperatorId,amount,permit)
TMP_2565(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
HIGH_LEVEL_CALL, dest:TMP_2565(ICSAccounting), function:depositStETH, arguments:['from_1', 'nodeOperatorId_1', 'amount_1', 'permit_1']  
 _addKeysAndUpdateDepositableValidatorsCount(nodeOperatorId,keysCount,publicKeys,signatures)
INTERNAL_CALL, CSModule._addKeysAndUpdateDepositableValidatorsCount(uint256,uint256,bytes,bytes)(nodeOperatorId_1,keysCount_1,publicKeys_1,signatures_1)
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
```
#### CSModule.addValidatorKeysWstETH(address,uint256,uint256,bytes,bytes,ICSAccounting.PermitInput) [EXTERNAL]
```slithir
 _checkCanAddKeys(nodeOperatorId,from)
INTERNAL_CALL, CSModule._checkCanAddKeys(uint256,address)(nodeOperatorId_1,from_1)
 amount = accounting().getRequiredBondForNextKeysWstETH(nodeOperatorId,keysCount)
TMP_2570(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
TMP_2571(uint256) = HIGH_LEVEL_CALL, dest:TMP_2570(ICSAccounting), function:getRequiredBondForNextKeysWstETH, arguments:['nodeOperatorId_1', 'keysCount_1']  
amount_1(uint256) := TMP_2571(uint256)
 amount != 0
TMP_2572(bool) = amount_1 != 0
CONDITION TMP_2572
 accounting().depositWstETH(from,nodeOperatorId,amount,permit)
TMP_2573(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
HIGH_LEVEL_CALL, dest:TMP_2573(ICSAccounting), function:depositWstETH, arguments:['from_1', 'nodeOperatorId_1', 'amount_1', 'permit_1']  
 _addKeysAndUpdateDepositableValidatorsCount(nodeOperatorId,keysCount,publicKeys,signatures)
INTERNAL_CALL, CSModule._addKeysAndUpdateDepositableValidatorsCount(uint256,uint256,bytes,bytes)(nodeOperatorId_1,keysCount_1,publicKeys_1,signatures_1)
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
```
#### CSModule.reportELRewardsStealingPenalty(uint256,bytes32,uint256) [EXTERNAL]
```slithir
REPORT_EL_REWARDS_STEALING_PENALTY_ROLE_1(bytes32) := phi(['REPORT_EL_REWARDS_STEALING_PENALTY_ROLE_4', 'REPORT_EL_REWARDS_STEALING_PENALTY_ROLE_2', 'REPORT_EL_REWARDS_STEALING_PENALTY_ROLE_0'])
PARAMETERS_REGISTRY_13(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_0', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSModule._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
 amount == 0
TMP_2654(bool) = amount_1 == 0
CONDITION TMP_2654
 revert InvalidAmount()()
TMP_2655(None) = SOLIDITY_CALL revert InvalidAmount()()
 curveId = accounting().getBondCurveId(nodeOperatorId)
TMP_2656(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
TMP_2657(uint256) = HIGH_LEVEL_CALL, dest:TMP_2656(ICSAccounting), function:getBondCurveId, arguments:['nodeOperatorId_1']  
PARAMETERS_REGISTRY_17(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23', 'PARAMETERS_REGISTRY_16'])
curveId_1(uint256) := TMP_2657(uint256)
 additionalFine = PARAMETERS_REGISTRY.getElRewardsStealingAdditionalFine(curveId)
TMP_2658(uint256) = HIGH_LEVEL_CALL, dest:PARAMETERS_REGISTRY_17(ICSParametersRegistry), function:getElRewardsStealingAdditionalFine, arguments:['curveId_1']  
PARAMETERS_REGISTRY_18(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_17', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
additionalFine_1(uint256) := TMP_2658(uint256)
 accounting().lockBondETH(nodeOperatorId,amount + additionalFine)
TMP_2659(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
TMP_2660(uint256) = amount_1 (c)+ additionalFine_1
HIGH_LEVEL_CALL, dest:TMP_2659(ICSAccounting), function:lockBondETH, arguments:['nodeOperatorId_1', 'TMP_2660']  
 ELRewardsStealingPenaltyReported(nodeOperatorId,blockHash,amount)
Emit ELRewardsStealingPenaltyReported(nodeOperatorId_1,blockHash_1,amount_1)
 _updateDepositableValidatorsCount({nodeOperatorId:nodeOperatorId,incrementNonceIfUpdated:true})
INTERNAL_CALL, CSModule._updateDepositableValidatorsCount(uint256,bool)(nodeOperatorId_1,True)
 onlyRole(REPORT_EL_REWARDS_STEALING_PENALTY_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(REPORT_EL_REWARDS_STEALING_PENALTY_ROLE_1)
```
#### CSModule.compensateELRewardsStealingPenalty(uint256) [EXTERNAL]
```slithir
 _onlyNodeOperatorManager(nodeOperatorId,msg.sender)
INTERNAL_CALL, CSModule._onlyNodeOperatorManager(uint256,address)(nodeOperatorId_1,msg.sender)
 accounting().compensateLockedBondETH{value: msg.value}(nodeOperatorId)
TMP_2679(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
HIGH_LEVEL_CALL, dest:TMP_2679(ICSAccounting), function:compensateLockedBondETH, arguments:['nodeOperatorId_1'] value:msg.value 
 ELRewardsStealingPenaltyCompensated(nodeOperatorId,msg.value)
Emit ELRewardsStealingPenaltyCompensated(nodeOperatorId_1,msg.value)
 _updateDepositableValidatorsCount({nodeOperatorId:nodeOperatorId,incrementNonceIfUpdated:true})
INTERNAL_CALL, CSModule._updateDepositableValidatorsCount(uint256,bool)(nodeOperatorId_1,True)
```
#### CSModule.cancelELRewardsStealingPenalty(uint256,uint256) [EXTERNAL]
```slithir
REPORT_EL_REWARDS_STEALING_PENALTY_ROLE_3(bytes32) := phi(['REPORT_EL_REWARDS_STEALING_PENALTY_ROLE_4', 'REPORT_EL_REWARDS_STEALING_PENALTY_ROLE_2', 'REPORT_EL_REWARDS_STEALING_PENALTY_ROLE_0'])
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSModule._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
 accounting().releaseLockedBondETH(nodeOperatorId,amount)
TMP_2666(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
HIGH_LEVEL_CALL, dest:TMP_2666(ICSAccounting), function:releaseLockedBondETH, arguments:['nodeOperatorId_1', 'amount_1']  
 ELRewardsStealingPenaltyCancelled(nodeOperatorId,amount)
Emit ELRewardsStealingPenaltyCancelled(nodeOperatorId_1,amount_1)
 _updateDepositableValidatorsCount({nodeOperatorId:nodeOperatorId,incrementNonceIfUpdated:true})
INTERNAL_CALL, CSModule._updateDepositableValidatorsCount(uint256,bool)(nodeOperatorId_1,True)
 onlyRole(REPORT_EL_REWARDS_STEALING_PENALTY_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(REPORT_EL_REWARDS_STEALING_PENALTY_ROLE_3)
```
#### CSModule.settleELRewardsStealingPenalty(uint256[]) [EXTERNAL]
```slithir
SETTLE_EL_REWARDS_STEALING_PENALTY_ROLE_1(bytes32) := phi(['SETTLE_EL_REWARDS_STEALING_PENALTY_ROLE_0', 'SETTLE_EL_REWARDS_STEALING_PENALTY_ROLE_2'])
 i < nodeOperatorIds.length
i_1(uint256) := phi(['i_0', 'i_2'])
REF_993 -> LENGTH nodeOperatorIds_1
TMP_2671(bool) = i_1 < REF_993
CONDITION TMP_2671
 nodeOperatorId = nodeOperatorIds[i]
REF_994(uint256) -> nodeOperatorIds_1[i_1]
nodeOperatorId_1(uint256) := REF_994(uint256)
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSModule._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
 applied = accounting().settleLockedBondETH(nodeOperatorId)
TMP_2673(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
TMP_2674(bool) = HIGH_LEVEL_CALL, dest:TMP_2673(ICSAccounting), function:settleLockedBondETH, arguments:['nodeOperatorId_1']  
applied_1(bool) := TMP_2674(bool)
 applied
CONDITION applied_1
 ELRewardsStealingPenaltySettled(nodeOperatorId)
Emit ELRewardsStealingPenaltySettled(nodeOperatorId_1)
 _updateDepositableValidatorsCount({nodeOperatorId:nodeOperatorId,incrementNonceIfUpdated:true})
INTERNAL_CALL, CSModule._updateDepositableValidatorsCount(uint256,bool)(nodeOperatorId_1,True)
 ++ i
i_2(uint256) = i_1 (c)+ 1
 onlyRole(SETTLE_EL_REWARDS_STEALING_PENALTY_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(SETTLE_EL_REWARDS_STEALING_PENALTY_ROLE_1)
```
#### CSModule.proposeNodeOperatorManagerAddressChange(uint256,address) [EXTERNAL]
```slithir
_nodeOperators_8(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 NOAddresses.proposeNodeOperatorManagerAddressChange(_nodeOperators,nodeOperatorId,proposedAddress)
LIBRARY_CALL, dest:NOAddresses, function:NOAddresses.proposeNodeOperatorManagerAddressChange(mapping(uint256 => NodeOperator),uint256,address), arguments:['_nodeOperators_8', 'nodeOperatorId_1', 'proposedAddress_1']
```
#### CSModule.confirmNodeOperatorManagerAddressChange(uint256) [EXTERNAL]
```slithir
_nodeOperators_9(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 NOAddresses.confirmNodeOperatorManagerAddressChange(_nodeOperators,nodeOperatorId)
LIBRARY_CALL, dest:NOAddresses, function:NOAddresses.confirmNodeOperatorManagerAddressChange(mapping(uint256 => NodeOperator),uint256), arguments:['_nodeOperators_9', 'nodeOperatorId_1']
```
#### CSModule.resetNodeOperatorManagerAddress(uint256) [EXTERNAL]
```slithir
_nodeOperators_12(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 NOAddresses.resetNodeOperatorManagerAddress(_nodeOperators,nodeOperatorId)
LIBRARY_CALL, dest:NOAddresses, function:NOAddresses.resetNodeOperatorManagerAddress(mapping(uint256 => NodeOperator),uint256), arguments:['_nodeOperators_12', 'nodeOperatorId_1']
```
#### CSModule.proposeNodeOperatorRewardAddressChange(uint256,address) [EXTERNAL]
```slithir
_nodeOperators_10(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 NOAddresses.proposeNodeOperatorRewardAddressChange(_nodeOperators,nodeOperatorId,proposedAddress)
LIBRARY_CALL, dest:NOAddresses, function:NOAddresses.proposeNodeOperatorRewardAddressChange(mapping(uint256 => NodeOperator),uint256,address), arguments:['_nodeOperators_10', 'nodeOperatorId_1', 'proposedAddress_1']
```
#### CSModule.confirmNodeOperatorRewardAddressChange(uint256) [EXTERNAL]
```slithir
_nodeOperators_11(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 NOAddresses.confirmNodeOperatorRewardAddressChange(_nodeOperators,nodeOperatorId)
LIBRARY_CALL, dest:NOAddresses, function:NOAddresses.confirmNodeOperatorRewardAddressChange(mapping(uint256 => NodeOperator),uint256), arguments:['_nodeOperators_11', 'nodeOperatorId_1']
```
#### CSModule.changeNodeOperatorRewardAddress(uint256,address) [EXTERNAL]
```slithir
_nodeOperators_13(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 NOAddresses.changeNodeOperatorRewardAddress(_nodeOperators,nodeOperatorId,newAddress)
LIBRARY_CALL, dest:NOAddresses, function:NOAddresses.changeNodeOperatorRewardAddress(mapping(uint256 => NodeOperator),uint256,address), arguments:['_nodeOperators_13', 'nodeOperatorId_1', 'newAddress_1']
```
#### CSModule.depositQueuePointers(uint256) [EXTERNAL]
```slithir
 q = _getQueue(queuePriority)
TMP_2755(QueueLib.Queue) = INTERNAL_CALL, CSModule._getQueue(uint256)(queuePriority_1)
q_1 (-> ['TMP_2755'])(QueueLib.Queue) := TMP_2755(QueueLib.Queue)
 (q.head,q.tail)
REF_1063(uint128) -> q_1 (-> ['TMP_2755']).head
REF_1064(uint128) -> q_1 (-> ['TMP_2755']).tail
RETURN REF_1063,REF_1064
 (head,tail)
```
#### CSModule.depositQueueItem(uint256,uint128) [EXTERNAL]
```slithir
 _getQueue(queuePriority).at(index)
TMP_2756(QueueLib.Queue) = INTERNAL_CALL, CSModule._getQueue(uint256)(queuePriority_1)
TMP_2757(Batch) = LIBRARY_CALL, dest:QueueLib, function:QueueLib.at(QueueLib.Queue,uint128), arguments:['TMP_2756', 'index_1'] 
RETURN TMP_2757
```
#### CSModule.cleanDepositQueue(uint256) [EXTERNAL]
```slithir
QUEUE_LOWEST_PRIORITY_13(uint256) := phi(['QUEUE_LOWEST_PRIORITY_5', 'QUEUE_LOWEST_PRIORITY_14', 'QUEUE_LOWEST_PRIORITY_20', 'QUEUE_LOWEST_PRIORITY_7', 'QUEUE_LOWEST_PRIORITY_1', 'QUEUE_LOWEST_PRIORITY_0', 'QUEUE_LOWEST_PRIORITY_18'])
_nodeOperators_52(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 removed = 0
removed_1(uint256) := 0(uint256)
 lastRemovedAtDepth = 0
lastRemovedAtDepth_1(uint256) := 0(uint256)
 maxItems == 0
TMP_2747(bool) = maxItems_1 == 0
CONDITION TMP_2747
 (0,0)
RETURN 0,0
 queueLookup = TransientUintUintMapLib.create()
TMP_2748(TransientUintUintMap) = LIBRARY_CALL, dest:TransientUintUintMapLib, function:TransientUintUintMapLib.create(), arguments:[] 
queueLookup_1(TransientUintUintMap) := TMP_2748(TransientUintUintMap)
 totalVisited = 0
totalVisited_1(uint256) := 0(uint256)
 priority = 0
priority_1(uint256) := 0(uint256)
 true
maxItems_2(uint256) := phi(['maxItems_3', 'maxItems_1'])
totalVisited_2(uint256) := phi(['totalVisited_1', 'totalVisited_3'])
priority_2(uint256) := phi(['priority_3', 'priority_1'])
CONDITION True
 priority > QUEUE_LOWEST_PRIORITY
TMP_2749(bool) = priority_2 > QUEUE_LOWEST_PRIORITY_13
CONDITION TMP_2749
 queue = _getQueue(priority)
TMP_2750(QueueLib.Queue) = INTERNAL_CALL, CSModule._getQueue(uint256)(priority_2)
queue_1 (-> ['TMP_2750'])(QueueLib.Queue) := TMP_2750(QueueLib.Queue)
 ++ priority
priority_3(uint256) = priority_2 + 1
 (removedPerQueue,lastRemovedAtDepthPerQueue,visitedPerQueue,reachedOutOfQueue) = queue.clean(_nodeOperators,maxItems,queueLookup)
TUPLE_13(uint256,uint256,uint256,bool) = LIBRARY_CALL, dest:QueueLib, function:QueueLib.clean(QueueLib.Queue,mapping(uint256 => NodeOperator),uint256,TransientUintUintMap), arguments:["queue_1 (-> ['TMP_2750'])", '_nodeOperators_53', 'maxItems_2', 'queueLookup_1'] 
removedPerQueue_1(uint256)= UNPACK TUPLE_13 index: 0 
lastRemovedAtDepthPerQueue_1(uint256)= UNPACK TUPLE_13 index: 1 
visitedPerQueue_1(uint256)= UNPACK TUPLE_13 index: 2 
reachedOutOfQueue_1(bool)= UNPACK TUPLE_13 index: 3 
 removedPerQueue > 0
TMP_2751(bool) = removedPerQueue_1 > 0
CONDITION TMP_2751
 lastRemovedAtDepth = totalVisited + lastRemovedAtDepthPerQueue
TMP_2752(uint256) = totalVisited_2 + lastRemovedAtDepthPerQueue_1
lastRemovedAtDepth_2(uint256) := TMP_2752(uint256)
 removed += removedPerQueue
removed_2(uint256) = removed_1 + removedPerQueue_1
removed_3(uint256) := phi(['removed_2', 'removed_1'])
lastRemovedAtDepth_3(uint256) := phi(['lastRemovedAtDepth_1', 'lastRemovedAtDepth_2'])
 ! reachedOutOfQueue
TMP_2753 = UnaryType.BANG reachedOutOfQueue_1 
CONDITION TMP_2753
 totalVisited += visitedPerQueue
totalVisited_3(uint256) = totalVisited_2 + visitedPerQueue_1
 maxItems -= visitedPerQueue
maxItems_3(uint256) = maxItems_2 - visitedPerQueue_1
 (removed,lastRemovedAtDepth)
RETURN removed_1,lastRemovedAtDepth_1
```
#### CSModule.updateDepositableValidatorsCount(uint256) [EXTERNAL]
```slithir
 _updateDepositableValidatorsCount({nodeOperatorId:nodeOperatorId,incrementNonceIfUpdated:true})
INTERNAL_CALL, CSModule._updateDepositableValidatorsCount(uint256,bool)(nodeOperatorId_1,True)
```
#### CSModule.migrateToPriorityQueue(uint256) [EXTERNAL]
```slithir
PARAMETERS_REGISTRY_9(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_0', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
QUEUE_LOWEST_PRIORITY_2(uint256) := phi(['QUEUE_LOWEST_PRIORITY_5', 'QUEUE_LOWEST_PRIORITY_14', 'QUEUE_LOWEST_PRIORITY_20', 'QUEUE_LOWEST_PRIORITY_7', 'QUEUE_LOWEST_PRIORITY_1', 'QUEUE_LOWEST_PRIORITY_0', 'QUEUE_LOWEST_PRIORITY_18'])
_nodeOperators_28(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 no = _nodeOperators[nodeOperatorId]
REF_981(NodeOperator) -> _nodeOperators_28[nodeOperatorId_1]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_981(NodeOperator)
 no.usedPriorityQueue
REF_982(bool) -> no_1 (-> ['_nodeOperators']).usedPriorityQueue
CONDITION REF_982
 revert PriorityQueueAlreadyUsed()()
TMP_2639(None) = SOLIDITY_CALL revert PriorityQueueAlreadyUsed()()
 curveId = accounting().getBondCurveId(nodeOperatorId)
TMP_2640(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
TMP_2641(uint256) = HIGH_LEVEL_CALL, dest:TMP_2640(ICSAccounting), function:getBondCurveId, arguments:['nodeOperatorId_1']  
PARAMETERS_REGISTRY_11(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_10', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
QUEUE_LOWEST_PRIORITY_4(uint256) := phi(['QUEUE_LOWEST_PRIORITY_5', 'QUEUE_LOWEST_PRIORITY_14', 'QUEUE_LOWEST_PRIORITY_3', 'QUEUE_LOWEST_PRIORITY_20', 'QUEUE_LOWEST_PRIORITY_7', 'QUEUE_LOWEST_PRIORITY_1', 'QUEUE_LOWEST_PRIORITY_18'])
curveId_1(uint256) := TMP_2641(uint256)
 (priority,maxDeposits) = PARAMETERS_REGISTRY.getQueueConfig(curveId)
TUPLE_11(uint32,uint32) = HIGH_LEVEL_CALL, dest:PARAMETERS_REGISTRY_11(ICSParametersRegistry), function:getQueueConfig, arguments:['curveId_1']  
PARAMETERS_REGISTRY_12(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_11', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
QUEUE_LOWEST_PRIORITY_5(uint256) := phi(['QUEUE_LOWEST_PRIORITY_4', 'QUEUE_LOWEST_PRIORITY_5', 'QUEUE_LOWEST_PRIORITY_14', 'QUEUE_LOWEST_PRIORITY_20', 'QUEUE_LOWEST_PRIORITY_7', 'QUEUE_LOWEST_PRIORITY_1', 'QUEUE_LOWEST_PRIORITY_18'])
priority_1(uint32)= UNPACK TUPLE_11 index: 0 
maxDeposits_1(uint32)= UNPACK TUPLE_11 index: 1 
 priority == QUEUE_LOWEST_PRIORITY
TMP_2642(bool) = priority_1 == QUEUE_LOWEST_PRIORITY_5
CONDITION TMP_2642
 revert NotEligibleForPriorityQueue()()
TMP_2643(None) = SOLIDITY_CALL revert NotEligibleForPriorityQueue()()
 enqueued = no.enqueuedCount
REF_985(uint32) -> no_1 (-> ['_nodeOperators']).enqueuedCount
enqueued_1(uint32) := REF_985(uint32)
 enqueued == 0
TMP_2644(bool) = enqueued_1 == 0
CONDITION TMP_2644
 revert NoQueuedKeysToMigrate()()
TMP_2645(None) = SOLIDITY_CALL revert NoQueuedKeysToMigrate()()
 deposited = no.totalDepositedKeys
REF_986(uint32) -> no_1 (-> ['_nodeOperators']).totalDepositedKeys
deposited_1(uint32) := REF_986(uint32)
 maxDeposits <= deposited
TMP_2646(bool) = maxDeposits_1 <= deposited_1
CONDITION TMP_2646
 revert PriorityQueueMaxDepositsUsed()()
TMP_2647(None) = SOLIDITY_CALL revert PriorityQueueMaxDepositsUsed()()
 toMigrate = uint32(Math.min(enqueued,maxDeposits - deposited))
TMP_2648(uint32) = maxDeposits_1 (c)- deposited_1
TMP_2649(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['enqueued_1', 'TMP_2648'] 
TMP_2650 = CONVERT TMP_2649 to uint32
toMigrate_1(uint32) := TMP_2650(uint32)
 _enqueueNodeOperatorKeys(nodeOperatorId,priority,toMigrate)
INTERNAL_CALL, CSModule._enqueueNodeOperatorKeys(uint256,uint256,uint32)(nodeOperatorId_1,priority_1,toMigrate_1)
 no.usedPriorityQueue = true
REF_988(bool) -> no_1 (-> ['_nodeOperators']).usedPriorityQueue
no_2 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['_nodeOperators'])"])
REF_988(bool) (->no_2 (-> ['_nodeOperators'])) := True(bool)
_nodeOperators_29(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['_nodeOperators'])"])
 _incrementModuleNonce()
INTERNAL_CALL, CSModule._incrementModuleNonce()()
```
#### CSModule.getNodeOperator(uint256) [EXTERNAL]
```slithir
_nodeOperators_54(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 _nodeOperators[nodeOperatorId]
REF_1067(NodeOperator) -> _nodeOperators_54[nodeOperatorId_1]
RETURN REF_1067
```
#### CSModule.getNodeOperatorManagementProperties(uint256) [EXTERNAL]
```slithir
_nodeOperators_55(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 no = _nodeOperators[nodeOperatorId]
REF_1068(NodeOperator) -> _nodeOperators_55[nodeOperatorId_1]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_1068(NodeOperator)
 (NodeOperatorManagementProperties(no.managerAddress,no.rewardAddress,no.extendedManagerPermissions))
REF_1069(address) -> no_1 (-> ['_nodeOperators']).managerAddress
REF_1070(address) -> no_1 (-> ['_nodeOperators']).rewardAddress
REF_1071(bool) -> no_1 (-> ['_nodeOperators']).extendedManagerPermissions
TMP_2759(NodeOperatorManagementProperties) = new NodeOperatorManagementProperties(REF_1069,REF_1070,REF_1071)
RETURN TMP_2759
```
#### CSModule.getNodeOperatorOwner(uint256) [EXTERNAL]
```slithir
_nodeOperators_56(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 no = _nodeOperators[nodeOperatorId]
REF_1072(NodeOperator) -> _nodeOperators_56[nodeOperatorId_1]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_1072(NodeOperator)
 no.extendedManagerPermissions
REF_1073(bool) -> no_1 (-> ['_nodeOperators']).extendedManagerPermissions
CONDITION REF_1073
 no.managerAddress
REF_1074(address) -> no_1 (-> ['_nodeOperators']).managerAddress
RETURN REF_1074
 no.rewardAddress
REF_1075(address) -> no_1 (-> ['_nodeOperators']).rewardAddress
RETURN REF_1075
```
#### CSModule.getNodeOperatorNonWithdrawnKeys(uint256) [EXTERNAL]
```slithir
_nodeOperators_57(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 no = _nodeOperators[nodeOperatorId]
REF_1076(NodeOperator) -> _nodeOperators_57[nodeOperatorId_1]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_1076(NodeOperator)
 no.totalAddedKeys - no.totalWithdrawnKeys
REF_1077(uint32) -> no_1 (-> ['_nodeOperators']).totalAddedKeys
REF_1078(uint32) -> no_1 (-> ['_nodeOperators']).totalWithdrawnKeys
TMP_2760(uint32) = REF_1077 - REF_1078
RETURN TMP_2760
```
#### CSModule.getNodeOperatorTotalDepositedKeys(uint256) [EXTERNAL]
```slithir
_nodeOperators_60(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 totalDepositedKeys = _nodeOperators[nodeOperatorId].totalDepositedKeys
REF_1095(NodeOperator) -> _nodeOperators_60[nodeOperatorId_1]
REF_1096(uint32) -> REF_1095.totalDepositedKeys
totalDepositedKeys_1(uint256) := REF_1096(uint32)
 totalDepositedKeys
RETURN totalDepositedKeys_1
```
#### CSModule.getSigningKeys(uint256,uint256,uint256) [EXTERNAL]
```slithir
 _onlyValidIndexRange(nodeOperatorId,startIndex,keysCount)
INTERNAL_CALL, CSModule._onlyValidIndexRange(uint256,uint256,uint256)(nodeOperatorId_1,startIndex_1,keysCount_1)
 SigningKeys.loadKeys(nodeOperatorId,startIndex,keysCount)
TMP_2775(bytes) = LIBRARY_CALL, dest:SigningKeys, function:SigningKeys.loadKeys(uint256,uint256,uint256), arguments:['nodeOperatorId_1', 'startIndex_1', 'keysCount_1'] 
RETURN TMP_2775
```
#### CSModule.getSigningKeysWithSignatures(uint256,uint256,uint256) [EXTERNAL]
```slithir
 _onlyValidIndexRange(nodeOperatorId,startIndex,keysCount)
INTERNAL_CALL, CSModule._onlyValidIndexRange(uint256,uint256,uint256)(nodeOperatorId_1,startIndex_1,keysCount_1)
 (keys,signatures) = SigningKeys.initKeysSigsBuf(keysCount)
TUPLE_14(bytes,bytes) = LIBRARY_CALL, dest:SigningKeys, function:SigningKeys.initKeysSigsBuf(uint256), arguments:['keysCount_1'] 
keys_1(bytes)= UNPACK TUPLE_14 index: 0 
signatures_1(bytes)= UNPACK TUPLE_14 index: 1 
 SigningKeys.loadKeysSigs(nodeOperatorId,startIndex,keysCount,keys,signatures,0)
LIBRARY_CALL, dest:SigningKeys, function:SigningKeys.loadKeysSigs(uint256,uint256,uint256,bytes,bytes,uint256), arguments:['nodeOperatorId_1', 'startIndex_1', 'keysCount_1', 'keys_1', 'signatures_1', '0'] 
 (keys,signatures)
RETURN keys_1,signatures_1
```
#### CSModule.submitWithdrawals(ValidatorWithdrawalInfo[]) [EXTERNAL]
```slithir
VERIFIER_ROLE_1(bytes32) := phi(['VERIFIER_ROLE_2', 'VERIFIER_ROLE_0'])
DEPOSIT_SIZE_1(uint256) := phi(['DEPOSIT_SIZE_0', 'DEPOSIT_SIZE_2'])
EXIT_PENALTIES_2(ICSExitPenalties) := phi(['EXIT_PENALTIES_1', 'EXIT_PENALTIES_19', 'EXIT_PENALTIES_0', 'EXIT_PENALTIES_22', 'EXIT_PENALTIES_3', 'EXIT_PENALTIES_15'])
_nodeOperators_30(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
_isValidatorWithdrawn_1(mapping(uint256 => bool)) := phi(['_isValidatorWithdrawn_13', '_isValidatorWithdrawn_2', '_isValidatorWithdrawn_0'])
 anySubmission = false
anySubmission_1(bool) := False(bool)
 i < withdrawalsInfo.length
i_1(uint256) := phi(['i_2', 'i_0'])
REF_997 -> LENGTH withdrawalsInfo_1
TMP_2683(bool) = i_1 < REF_997
CONDITION TMP_2683
 withdrawalInfo = withdrawalsInfo[i]
REF_998(ValidatorWithdrawalInfo) -> withdrawalsInfo_1[i_1]
withdrawalInfo_1(ValidatorWithdrawalInfo) := REF_998(ValidatorWithdrawalInfo)
 _onlyExistingNodeOperator(withdrawalInfo.nodeOperatorId)
REF_999(uint256) -> withdrawalInfo_1.nodeOperatorId
INTERNAL_CALL, CSModule._onlyExistingNodeOperator(uint256)(REF_999)
 no = _nodeOperators[withdrawalInfo.nodeOperatorId]
REF_1000(uint256) -> withdrawalInfo_1.nodeOperatorId
REF_1001(NodeOperator) -> _nodeOperators_32[REF_1000]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_1001(NodeOperator)
 withdrawalInfo.keyIndex >= no.totalDepositedKeys
REF_1002(uint256) -> withdrawalInfo_1.keyIndex
REF_1003(uint32) -> no_1 (-> ['_nodeOperators']).totalDepositedKeys
TMP_2685(bool) = REF_1002 >= REF_1003
CONDITION TMP_2685
 revert SigningKeysInvalidOffset()()
TMP_2686(None) = SOLIDITY_CALL revert SigningKeysInvalidOffset()()
 pointer = _keyPointer(withdrawalInfo.nodeOperatorId,withdrawalInfo.keyIndex)
REF_1004(uint256) -> withdrawalInfo_1.nodeOperatorId
REF_1005(uint256) -> withdrawalInfo_1.keyIndex
TMP_2687(uint256) = INTERNAL_CALL, CSModule._keyPointer(uint256,uint256)(REF_1004,REF_1005)
pointer_1(uint256) := TMP_2687(uint256)
 _isValidatorWithdrawn[pointer]
REF_1006(bool) -> _isValidatorWithdrawn_4[pointer_1]
CONDITION REF_1006
 _isValidatorWithdrawn[pointer] = true
REF_1007(bool) -> _isValidatorWithdrawn_4[pointer_1]
_isValidatorWithdrawn_5(mapping(uint256 => bool)) := phi(['_isValidatorWithdrawn_4'])
REF_1007(bool) (->_isValidatorWithdrawn_5) := True(bool)
 ++ no.totalWithdrawnKeys
REF_1008(uint32) -> no_1 (-> ['_nodeOperators']).totalWithdrawnKeys
no_2 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['_nodeOperators'])"])
REF_1008(-> no_2 (-> ['_nodeOperators'])) = REF_1008 + 1
_nodeOperators_40(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['_nodeOperators'])"])
 pubkey = SigningKeys.loadKeys(withdrawalInfo.nodeOperatorId,withdrawalInfo.keyIndex,1)
REF_1010(uint256) -> withdrawalInfo_1.nodeOperatorId
REF_1011(uint256) -> withdrawalInfo_1.keyIndex
TMP_2688(bytes) = LIBRARY_CALL, dest:SigningKeys, function:SigningKeys.loadKeys(uint256,uint256,uint256), arguments:['REF_1010', 'REF_1011', '1'] 
pubkey_1(bytes) := TMP_2688(bytes)
 WithdrawalSubmitted(withdrawalInfo.nodeOperatorId,withdrawalInfo.keyIndex,withdrawalInfo.amount,pubkey)
REF_1012(uint256) -> withdrawalInfo_1.nodeOperatorId
REF_1013(uint256) -> withdrawalInfo_1.keyIndex
REF_1014(uint256) -> withdrawalInfo_1.amount
Emit WithdrawalSubmitted(REF_1012,REF_1013,REF_1014,pubkey_1)
 anySubmission = true
anySubmission_3(bool) := True(bool)
 exitPenaltyInfo = EXIT_PENALTIES.getExitPenaltyInfo(withdrawalInfo.nodeOperatorId,pubkey)
REF_1016(uint256) -> withdrawalInfo_1.nodeOperatorId
TMP_2690(ExitPenaltyInfo) = HIGH_LEVEL_CALL, dest:EXIT_PENALTIES_5(ICSExitPenalties), function:getExitPenaltyInfo, arguments:['REF_1016', 'pubkey_1']  
DEPOSIT_SIZE_5(uint256) := phi(['DEPOSIT_SIZE_4', 'DEPOSIT_SIZE_2'])
EXIT_PENALTIES_6(ICSExitPenalties) := phi(['EXIT_PENALTIES_5', 'EXIT_PENALTIES_1', 'EXIT_PENALTIES_19', 'EXIT_PENALTIES_22', 'EXIT_PENALTIES_3', 'EXIT_PENALTIES_15'])
_nodeOperators_34(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_33', '_nodeOperators_18', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
_isValidatorWithdrawn_6(mapping(uint256 => bool)) := phi(['_isValidatorWithdrawn_13', '_isValidatorWithdrawn_2', '_isValidatorWithdrawn_5'])
exitPenaltyInfo_1(ExitPenaltyInfo) := TMP_2690(ExitPenaltyInfo)
 exitPenaltyInfo.delayPenalty.isValue
REF_1017(MarkedUint248) -> exitPenaltyInfo_1.delayPenalty
REF_1018(bool) -> REF_1017.isValue
CONDITION REF_1018
 penaltySum += exitPenaltyInfo.delayPenalty.value
REF_1019(MarkedUint248) -> exitPenaltyInfo_1.delayPenalty
REF_1020(uint248) -> REF_1019.value
penaltySum_1(uint256) = penaltySum_0 + REF_1020
 chargeWithdrawalRequestFee = true
chargeWithdrawalRequestFee_1(bool) := True(bool)
penaltySum_2(uint256) := phi(['penaltySum_0', 'penaltySum_1'])
 exitPenaltyInfo.strikesPenalty.isValue
REF_1021(MarkedUint248) -> exitPenaltyInfo_1.strikesPenalty
REF_1022(bool) -> REF_1021.isValue
CONDITION REF_1022
 penaltySum += exitPenaltyInfo.strikesPenalty.value
REF_1023(MarkedUint248) -> exitPenaltyInfo_1.strikesPenalty
REF_1024(uint248) -> REF_1023.value
penaltySum_3(uint256) = penaltySum_2 + REF_1024
 chargeWithdrawalRequestFee = true
chargeWithdrawalRequestFee_2(bool) := True(bool)
penaltySum_4(uint256) := phi(['penaltySum_3', 'penaltySum_0'])
chargeWithdrawalRequestFee_3(bool) := phi(['chargeWithdrawalRequestFee_2', 'chargeWithdrawalRequestFee_0'])
 chargeWithdrawalRequestFee && exitPenaltyInfo.withdrawalRequestFee.value != 0
REF_1025(MarkedUint248) -> exitPenaltyInfo_1.withdrawalRequestFee
REF_1026(uint248) -> REF_1025.value
TMP_2691(bool) = REF_1026 != 0
TMP_2692(bool) = chargeWithdrawalRequestFee_3 && TMP_2691
CONDITION TMP_2692
 accounting().chargeFee(withdrawalInfo.nodeOperatorId,exitPenaltyInfo.withdrawalRequestFee.value)
TMP_2693(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
REF_1028(uint256) -> withdrawalInfo_1.nodeOperatorId
REF_1029(MarkedUint248) -> exitPenaltyInfo_1.withdrawalRequestFee
REF_1030(uint248) -> REF_1029.value
HIGH_LEVEL_CALL, dest:TMP_2693(ICSAccounting), function:chargeFee, arguments:['REF_1028', 'REF_1030']  
DEPOSIT_SIZE_7(uint256) := phi(['DEPOSIT_SIZE_6', 'DEPOSIT_SIZE_2'])
EXIT_PENALTIES_8(ICSExitPenalties) := phi(['EXIT_PENALTIES_1', 'EXIT_PENALTIES_19', 'EXIT_PENALTIES_7', 'EXIT_PENALTIES_22', 'EXIT_PENALTIES_3', 'EXIT_PENALTIES_15'])
_nodeOperators_36(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_35', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
_isValidatorWithdrawn_8(mapping(uint256 => bool)) := phi(['_isValidatorWithdrawn_13', '_isValidatorWithdrawn_2', '_isValidatorWithdrawn_7'])
 DEPOSIT_SIZE > withdrawalInfo.amount
REF_1031(uint256) -> withdrawalInfo_1.amount
TMP_2695(bool) = DEPOSIT_SIZE_7 > REF_1031
CONDITION TMP_2695
 penaltySum += DEPOSIT_SIZE - withdrawalInfo.amount
REF_1032(uint256) -> withdrawalInfo_1.amount
TMP_2696(uint256) = DEPOSIT_SIZE_7 - REF_1032
penaltySum_5(uint256) = penaltySum_4 + TMP_2696
penaltySum_6(uint256) := phi(['penaltySum_5', 'penaltySum_0'])
 penaltySum > 0
TMP_2697(bool) = penaltySum_6 > 0
CONDITION TMP_2697
 accounting().penalize(withdrawalInfo.nodeOperatorId,penaltySum)
TMP_2698(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
REF_1034(uint256) -> withdrawalInfo_1.nodeOperatorId
HIGH_LEVEL_CALL, dest:TMP_2698(ICSAccounting), function:penalize, arguments:['REF_1034', 'penaltySum_6']  
DEPOSIT_SIZE_9(uint256) := phi(['DEPOSIT_SIZE_8', 'DEPOSIT_SIZE_2'])
EXIT_PENALTIES_10(ICSExitPenalties) := phi(['EXIT_PENALTIES_1', 'EXIT_PENALTIES_19', 'EXIT_PENALTIES_9', 'EXIT_PENALTIES_22', 'EXIT_PENALTIES_3', 'EXIT_PENALTIES_15'])
_nodeOperators_38(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_6', '_nodeOperators_37', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
_isValidatorWithdrawn_10(mapping(uint256 => bool)) := phi(['_isValidatorWithdrawn_13', '_isValidatorWithdrawn_2', '_isValidatorWithdrawn_9'])
 _updateDepositableValidatorsCount({nodeOperatorId:withdrawalInfo.nodeOperatorId,incrementNonceIfUpdated:false})
REF_1035(uint256) -> withdrawalInfo_1.nodeOperatorId
INTERNAL_CALL, CSModule._updateDepositableValidatorsCount(uint256,bool)(REF_1035,False)
_nodeOperators_39(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_69', '_nodeOperators_68'])
 ++ i
anySubmission_2(bool) := phi(['anySubmission_1', 'anySubmission_3'])
i_2(uint256) = i_1 (c)+ 1
 anySubmission
CONDITION anySubmission_1
 _incrementModuleNonce()
INTERNAL_CALL, CSModule._incrementModuleNonce()()
 onlyRole(VERIFIER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(VERIFIER_ROLE_1)
```
#### CSModule.isValidatorWithdrawn(uint256,uint256) [EXTERNAL]
```slithir
_isValidatorWithdrawn_12(mapping(uint256 => bool)) := phi(['_isValidatorWithdrawn_13', '_isValidatorWithdrawn_2', '_isValidatorWithdrawn_0'])
 _isValidatorWithdrawn[_keyPointer(nodeOperatorId,keyIndex)]
TMP_2758(uint256) = INTERNAL_CALL, CSModule._keyPointer(uint256,uint256)(nodeOperatorId_1,keyIndex_1)
REF_1066(bool) -> _isValidatorWithdrawn_13[TMP_2758]
RETURN REF_1066
```
#### CSModule.removeKeys(uint256,uint256,uint256) [EXTERNAL]
```slithir
PARAMETERS_REGISTRY_4(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_0', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
_nodeOperators_24(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 _onlyNodeOperatorManager(nodeOperatorId,msg.sender)
INTERNAL_CALL, CSModule._onlyNodeOperatorManager(uint256,address)(nodeOperatorId_1,msg.sender)
_nodeOperators_25(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_77'])
 no = _nodeOperators[nodeOperatorId]
REF_972(NodeOperator) -> _nodeOperators_25[nodeOperatorId_1]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_972(NodeOperator)
 startIndex < no.totalDepositedKeys
REF_973(uint32) -> no_1 (-> ['_nodeOperators']).totalDepositedKeys
TMP_2621(bool) = startIndex_1 < REF_973
CONDITION TMP_2621
 revert SigningKeysInvalidOffset()()
TMP_2622(None) = SOLIDITY_CALL revert SigningKeysInvalidOffset()()
 newTotalSigningKeys = SigningKeys.removeKeysSigs(nodeOperatorId,startIndex,keysCount,no.totalAddedKeys)
REF_975(uint32) -> no_1 (-> ['_nodeOperators']).totalAddedKeys
TMP_2623(uint256) = LIBRARY_CALL, dest:SigningKeys, function:SigningKeys.removeKeysSigs(uint256,uint256,uint256,uint256), arguments:['nodeOperatorId_1', 'startIndex_1', 'keysCount_1', 'REF_975'] 
newTotalSigningKeys_1(uint256) := TMP_2623(uint256)
 curveId = accounting().getBondCurveId(nodeOperatorId)
TMP_2624(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
TMP_2625(uint256) = HIGH_LEVEL_CALL, dest:TMP_2624(ICSAccounting), function:getBondCurveId, arguments:['nodeOperatorId_1']  
PARAMETERS_REGISTRY_7(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_6', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
curveId_1(uint256) := TMP_2625(uint256)
 amountToCharge = PARAMETERS_REGISTRY.getKeyRemovalCharge(curveId) * keysCount
TMP_2626(uint256) = HIGH_LEVEL_CALL, dest:PARAMETERS_REGISTRY_7(ICSParametersRegistry), function:getKeyRemovalCharge, arguments:['curveId_1']  
PARAMETERS_REGISTRY_8(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_7', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
TMP_2627(uint256) = TMP_2626 (c)* keysCount_1
amountToCharge_1(uint256) := TMP_2627(uint256)
 amountToCharge != 0
TMP_2628(bool) = amountToCharge_1 != 0
CONDITION TMP_2628
 accounting().chargeFee(nodeOperatorId,amountToCharge)
TMP_2629(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
HIGH_LEVEL_CALL, dest:TMP_2629(ICSAccounting), function:chargeFee, arguments:['nodeOperatorId_1', 'amountToCharge_1']  
 KeyRemovalChargeApplied(nodeOperatorId)
Emit KeyRemovalChargeApplied(nodeOperatorId_1)
 no.totalAddedKeys = uint32(newTotalSigningKeys)
REF_979(uint32) -> no_1 (-> ['_nodeOperators']).totalAddedKeys
TMP_2632 = CONVERT newTotalSigningKeys_1 to uint32
no_2 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['_nodeOperators'])"])
REF_979(uint32) (->no_2 (-> ['_nodeOperators'])) := TMP_2632(uint32)
_nodeOperators_26(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['_nodeOperators'])"])
 TotalSigningKeysCountChanged(nodeOperatorId,newTotalSigningKeys)
Emit TotalSigningKeysCountChanged(nodeOperatorId_1,newTotalSigningKeys_1)
 no.totalVettedKeys = uint32(newTotalSigningKeys)
REF_980(uint32) -> no_2 (-> ['_nodeOperators']).totalVettedKeys
TMP_2634 = CONVERT newTotalSigningKeys_1 to uint32
no_3 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_2 (-> ['_nodeOperators'])"])
REF_980(uint32) (->no_3 (-> ['_nodeOperators'])) := TMP_2634(uint32)
_nodeOperators_27(mapping(uint256 => NodeOperator)) := phi(["no_3 (-> ['_nodeOperators'])"])
 VettedSigningKeysCountChanged(nodeOperatorId,newTotalSigningKeys)
Emit VettedSigningKeysCountChanged(nodeOperatorId_1,newTotalSigningKeys_1)
 _updateDepositableValidatorsCount({nodeOperatorId:nodeOperatorId,incrementNonceIfUpdated:false})
INTERNAL_CALL, CSModule._updateDepositableValidatorsCount(uint256,bool)(nodeOperatorId_1,False)
 _incrementModuleNonce()
INTERNAL_CALL, CSModule._incrementModuleNonce()()
```
#### CSModule.reportValidatorExitDelay(uint256,uint256,bytes,uint256) [EXTERNAL]
```slithir
STAKING_ROUTER_ROLE_19(bytes32) := phi(['STAKING_ROUTER_ROLE_8', 'STAKING_ROUTER_ROLE_14', 'STAKING_ROUTER_ROLE_0', 'STAKING_ROUTER_ROLE_16', 'STAKING_ROUTER_ROLE_20', 'STAKING_ROUTER_ROLE_22', 'STAKING_ROUTER_ROLE_24', 'STAKING_ROUTER_ROLE_12', 'STAKING_ROUTER_ROLE_10', 'STAKING_ROUTER_ROLE_18', 'STAKING_ROUTER_ROLE_6'])
EXIT_PENALTIES_12(ICSExitPenalties) := phi(['EXIT_PENALTIES_1', 'EXIT_PENALTIES_19', 'EXIT_PENALTIES_0', 'EXIT_PENALTIES_22', 'EXIT_PENALTIES_3', 'EXIT_PENALTIES_15'])
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSModule._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
 EXIT_PENALTIES.processExitDelayReport(nodeOperatorId,publicKey,eligibleToExitInSec)
HIGH_LEVEL_CALL, dest:EXIT_PENALTIES_14(ICSExitPenalties), function:processExitDelayReport, arguments:['nodeOperatorId_1', 'publicKey_1', 'eligibleToExitInSec_1']  
EXIT_PENALTIES_15(ICSExitPenalties) := phi(['EXIT_PENALTIES_14', 'EXIT_PENALTIES_1', 'EXIT_PENALTIES_19', 'EXIT_PENALTIES_22', 'EXIT_PENALTIES_3', 'EXIT_PENALTIES_15'])
 onlyRole(STAKING_ROUTER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(STAKING_ROUTER_ROLE_19)
```
#### CSModule.onValidatorExitTriggered(uint256,bytes,uint256,uint256) [EXTERNAL]
```slithir
STAKING_ROUTER_ROLE_21(bytes32) := phi(['STAKING_ROUTER_ROLE_8', 'STAKING_ROUTER_ROLE_14', 'STAKING_ROUTER_ROLE_0', 'STAKING_ROUTER_ROLE_16', 'STAKING_ROUTER_ROLE_20', 'STAKING_ROUTER_ROLE_22', 'STAKING_ROUTER_ROLE_24', 'STAKING_ROUTER_ROLE_12', 'STAKING_ROUTER_ROLE_10', 'STAKING_ROUTER_ROLE_18', 'STAKING_ROUTER_ROLE_6'])
EXIT_PENALTIES_16(ICSExitPenalties) := phi(['EXIT_PENALTIES_1', 'EXIT_PENALTIES_19', 'EXIT_PENALTIES_0', 'EXIT_PENALTIES_22', 'EXIT_PENALTIES_3', 'EXIT_PENALTIES_15'])
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSModule._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
 EXIT_PENALTIES.processTriggeredExit(nodeOperatorId,publicKey,withdrawalRequestPaidFee,exitType)
HIGH_LEVEL_CALL, dest:EXIT_PENALTIES_18(ICSExitPenalties), function:processTriggeredExit, arguments:['nodeOperatorId_1', 'publicKey_1', 'withdrawalRequestPaidFee_1', 'exitType_1']  
EXIT_PENALTIES_19(ICSExitPenalties) := phi(['EXIT_PENALTIES_18', 'EXIT_PENALTIES_1', 'EXIT_PENALTIES_19', 'EXIT_PENALTIES_22', 'EXIT_PENALTIES_3', 'EXIT_PENALTIES_15'])
 onlyRole(STAKING_ROUTER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(STAKING_ROUTER_ROLE_21)
```
#### CSModule.isValidatorExitDelayPenaltyApplicable(uint256,uint256,bytes,uint256) [EXTERNAL]
```slithir
EXIT_PENALTIES_20(ICSExitPenalties) := phi(['EXIT_PENALTIES_1', 'EXIT_PENALTIES_19', 'EXIT_PENALTIES_0', 'EXIT_PENALTIES_22', 'EXIT_PENALTIES_3', 'EXIT_PENALTIES_15'])
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSModule._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
 EXIT_PENALTIES.isValidatorExitDelayPenaltyApplicable(nodeOperatorId,publicKey,eligibleToExitInSec)
TMP_2792(bool) = HIGH_LEVEL_CALL, dest:EXIT_PENALTIES_21(ICSExitPenalties), function:isValidatorExitDelayPenaltyApplicable, arguments:['nodeOperatorId_1', 'publicKey_1', 'eligibleToExitInSec_1']  
EXIT_PENALTIES_22(ICSExitPenalties) := phi(['EXIT_PENALTIES_21', 'EXIT_PENALTIES_1', 'EXIT_PENALTIES_19', 'EXIT_PENALTIES_22', 'EXIT_PENALTIES_3', 'EXIT_PENALTIES_15'])
RETURN TMP_2792
```
#### CSModule.exitDeadlineThreshold(uint256) [EXTERNAL]
```slithir
PARAMETERS_REGISTRY_19(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_0', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSModule._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
 PARAMETERS_REGISTRY.getAllowedExitDelay(accounting().getBondCurveId(nodeOperatorId))
TMP_2794(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
TMP_2795(uint256) = HIGH_LEVEL_CALL, dest:TMP_2794(ICSAccounting), function:getBondCurveId, arguments:['nodeOperatorId_1']  
PARAMETERS_REGISTRY_22(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_21', 'PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
TMP_2796(uint256) = HIGH_LEVEL_CALL, dest:PARAMETERS_REGISTRY_22(ICSParametersRegistry), function:getAllowedExitDelay, arguments:['TMP_2795']  
PARAMETERS_REGISTRY_23(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_22', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
RETURN TMP_2796
```
#### CSModule.getType() [EXTERNAL]
```slithir
MODULE_TYPE_2(bytes32) := phi(['MODULE_TYPE_1', 'MODULE_TYPE_0'])
 MODULE_TYPE
RETURN MODULE_TYPE_2
```
#### CSModule.getStakingModuleSummary() [EXTERNAL]
```slithir
_totalDepositedValidators_9(uint64) := phi(['_totalDepositedValidators_0', '_totalDepositedValidators_2', '_totalDepositedValidators_8'])
_totalExitedValidators_1(uint64) := phi(['_totalExitedValidators_3', '_totalExitedValidators_4', '_totalExitedValidators_0'])
_depositableValidatorsCount_9(uint64) := phi(['_depositableValidatorsCount_0', '_depositableValidatorsCount_2', '_depositableValidatorsCount_8', '_depositableValidatorsCount_13', '_depositableValidatorsCount_12'])
 totalExitedValidators = _totalExitedValidators
totalExitedValidators_1(uint256) := _totalExitedValidators_1(uint64)
 totalDepositedValidators = _totalDepositedValidators
totalDepositedValidators_1(uint256) := _totalDepositedValidators_9(uint64)
 depositableValidatorsCount = _depositableValidatorsCount
depositableValidatorsCount_1(uint256) := _depositableValidatorsCount_9(uint64)
 (totalExitedValidators,totalDepositedValidators,depositableValidatorsCount)
RETURN totalExitedValidators_1,totalDepositedValidators_1,depositableValidatorsCount_1
```
#### CSModule.getNodeOperatorSummary(uint256) [EXTERNAL]
```slithir
FORCED_TARGET_LIMIT_MODE_ID_3(uint8) := phi(['FORCED_TARGET_LIMIT_MODE_ID_2', 'FORCED_TARGET_LIMIT_MODE_ID_6', 'FORCED_TARGET_LIMIT_MODE_ID_0'])
_nodeOperators_58(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSModule._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
 no = _nodeOperators[nodeOperatorId]
REF_1079(NodeOperator) -> _nodeOperators_59[nodeOperatorId_1]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_1079(NodeOperator)
 totalUnbondedKeys = accounting().getUnbondedKeysCountToEject(nodeOperatorId)
TMP_2762(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
TMP_2763(uint256) = HIGH_LEVEL_CALL, dest:TMP_2762(ICSAccounting), function:getUnbondedKeysCountToEject, arguments:['nodeOperatorId_1']  
FORCED_TARGET_LIMIT_MODE_ID_6(uint8) := phi(['FORCED_TARGET_LIMIT_MODE_ID_2', 'FORCED_TARGET_LIMIT_MODE_ID_5', 'FORCED_TARGET_LIMIT_MODE_ID_6'])
totalUnbondedKeys_1(uint256) := TMP_2763(uint256)
 totalNonDepositedKeys = no.totalAddedKeys - no.totalDepositedKeys
REF_1081(uint32) -> no_1 (-> ['_nodeOperators']).totalAddedKeys
REF_1082(uint32) -> no_1 (-> ['_nodeOperators']).totalDepositedKeys
TMP_2764(uint32) = REF_1081 (c)- REF_1082
totalNonDepositedKeys_1(uint256) := TMP_2764(uint32)
 totalUnbondedKeys > totalNonDepositedKeys && no.targetLimitMode == FORCED_TARGET_LIMIT_MODE_ID
TMP_2765(bool) = totalUnbondedKeys_1 > totalNonDepositedKeys_1
REF_1083(uint8) -> no_1 (-> ['_nodeOperators']).targetLimitMode
TMP_2766(bool) = REF_1083 == FORCED_TARGET_LIMIT_MODE_ID_6
TMP_2767(bool) = TMP_2765 && TMP_2766
CONDITION TMP_2767
 targetLimitMode = FORCED_TARGET_LIMIT_MODE_ID
targetLimitMode_1(uint256) := FORCED_TARGET_LIMIT_MODE_ID_6(uint8)
 targetValidatorsCount = Math.min(no.targetLimit,no.totalAddedKeys - no.totalWithdrawnKeys - totalUnbondedKeys)
REF_1085(uint32) -> no_1 (-> ['_nodeOperators']).targetLimit
REF_1086(uint32) -> no_1 (-> ['_nodeOperators']).totalAddedKeys
REF_1087(uint32) -> no_1 (-> ['_nodeOperators']).totalWithdrawnKeys
TMP_2768(uint32) = REF_1086 - REF_1087
TMP_2769(uint32) = TMP_2768 - totalUnbondedKeys_1
TMP_2770(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['REF_1085', 'TMP_2769'] 
targetValidatorsCount_1(uint256) := TMP_2770(uint256)
 totalUnbondedKeys > totalNonDepositedKeys
TMP_2771(bool) = totalUnbondedKeys_1 > totalNonDepositedKeys_1
CONDITION TMP_2771
 targetLimitMode = FORCED_TARGET_LIMIT_MODE_ID
targetLimitMode_2(uint256) := FORCED_TARGET_LIMIT_MODE_ID_6(uint8)
 targetValidatorsCount = no.totalAddedKeys - no.totalWithdrawnKeys - totalUnbondedKeys
REF_1088(uint32) -> no_1 (-> ['_nodeOperators']).totalAddedKeys
REF_1089(uint32) -> no_1 (-> ['_nodeOperators']).totalWithdrawnKeys
TMP_2772(uint32) = REF_1088 - REF_1089
TMP_2773(uint32) = TMP_2772 - totalUnbondedKeys_1
targetValidatorsCount_2(uint256) := TMP_2773(uint32)
 targetLimitMode = no.targetLimitMode
REF_1090(uint8) -> no_1 (-> ['_nodeOperators']).targetLimitMode
targetLimitMode_3(uint256) := REF_1090(uint8)
 targetValidatorsCount = no.targetLimit
REF_1091(uint32) -> no_1 (-> ['_nodeOperators']).targetLimit
targetValidatorsCount_3(uint256) := REF_1091(uint32)
targetLimitMode_4(uint256) := phi(['targetLimitMode_2', 'targetLimitMode_3'])
targetValidatorsCount_4(uint256) := phi(['targetValidatorsCount_2', 'targetValidatorsCount_3'])
targetLimitMode_5(uint256) := phi(['targetLimitMode_1', 'targetLimitMode_0'])
targetValidatorsCount_5(uint256) := phi(['targetValidatorsCount_0', 'targetValidatorsCount_1'])
 stuckValidatorsCount = 0
stuckValidatorsCount_1(uint256) := 0(uint256)
 refundedValidatorsCount = 0
refundedValidatorsCount_1(uint256) := 0(uint256)
 stuckPenaltyEndTimestamp = 0
stuckPenaltyEndTimestamp_1(uint256) := 0(uint256)
 totalExitedValidators = no.totalExitedKeys
REF_1092(uint32) -> no_1 (-> ['_nodeOperators']).totalExitedKeys
totalExitedValidators_1(uint256) := REF_1092(uint32)
 totalDepositedValidators = no.totalDepositedKeys
REF_1093(uint32) -> no_1 (-> ['_nodeOperators']).totalDepositedKeys
totalDepositedValidators_1(uint256) := REF_1093(uint32)
 depositableValidatorsCount = no.depositableValidatorsCount
REF_1094(uint32) -> no_1 (-> ['_nodeOperators']).depositableValidatorsCount
depositableValidatorsCount_1(uint256) := REF_1094(uint32)
 (targetLimitMode,targetValidatorsCount,stuckValidatorsCount,refundedValidatorsCount,stuckPenaltyEndTimestamp,totalExitedValidators,totalDepositedValidators,depositableValidatorsCount)
RETURN targetLimitMode_5,targetValidatorsCount_5,stuckValidatorsCount_1,refundedValidatorsCount_1,stuckPenaltyEndTimestamp_1,totalExitedValidators_1,totalDepositedValidators_1,depositableValidatorsCount_1
```
#### CSModule.getNonce() [EXTERNAL]
```slithir
_nonce_1(uint256) := phi(['_nonce_3', '_nonce_0'])
 _nonce
RETURN _nonce_1
```
#### CSModule.getNodeOperatorsCount() [EXTERNAL]
```slithir
_nodeOperatorsCount_6(uint64) := phi(['_nodeOperatorsCount_0', '_nodeOperatorsCount_5'])
 _nodeOperatorsCount
RETURN _nodeOperatorsCount_6
```
#### CSModule.getActiveNodeOperatorsCount() [EXTERNAL]
```slithir
_nodeOperatorsCount_7(uint64) := phi(['_nodeOperatorsCount_0', '_nodeOperatorsCount_5'])
 _nodeOperatorsCount
RETURN _nodeOperatorsCount_7
```
#### CSModule.getNodeOperatorIsActive(uint256) [EXTERNAL]
```slithir
_nodeOperatorsCount_8(uint64) := phi(['_nodeOperatorsCount_0', '_nodeOperatorsCount_5'])
 nodeOperatorId < _nodeOperatorsCount
TMP_2778(bool) = nodeOperatorId_1 < _nodeOperatorsCount_8
RETURN TMP_2778
```
#### CSModule.getNodeOperatorIds(uint256,uint256) [EXTERNAL]
```slithir
_nodeOperatorsCount_9(uint64) := phi(['_nodeOperatorsCount_0', '_nodeOperatorsCount_5'])
 nodeOperatorsCount = _nodeOperatorsCount
nodeOperatorsCount_1(uint256) := _nodeOperatorsCount_9(uint64)
 offset >= nodeOperatorsCount || limit == 0
TMP_2779(bool) = offset_1 >= nodeOperatorsCount_1
TMP_2780(bool) = limit_1 == 0
TMP_2781(bool) = TMP_2779 || TMP_2780
CONDITION TMP_2781
 new uint256[](0)
TMP_2783(uint256[])  = new uint256[](0)
RETURN TMP_2783
 nodeOperatorIds = new uint256[](idsCount)
TMP_2785(uint256[])  = new uint256[](idsCount_3)
nodeOperatorIds_1(uint256[]) = ['TMP_2785(uint256[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < nodeOperatorIds.length
nodeOperatorIds_2(uint256[]) := phi(['nodeOperatorIds_1', 'nodeOperatorIds_3'])
i_2(uint256) := phi(['i_3', 'i_1'])
REF_1100 -> LENGTH nodeOperatorIds_2
TMP_2786(bool) = i_2 < REF_1100
CONDITION TMP_2786
 nodeOperatorIds[i] = offset + i
REF_1101(uint256) -> nodeOperatorIds_2[i_2]
TMP_2787(uint256) = offset_1 (c)+ i_2
nodeOperatorIds_3(uint256[]) := phi(['nodeOperatorIds_2'])
REF_1101(uint256) (->nodeOperatorIds_3) := TMP_2787(uint256)
 ++ i
i_3(uint256) = i_2 (c)+ 1
 limit < nodeOperatorsCount - offset
TMP_2788(uint256) = nodeOperatorsCount_1 (c)- offset_1
TMP_2789(bool) = limit_1 < TMP_2788
CONDITION TMP_2789
 idsCount = limit
idsCount_1(uint256) := limit_1(uint256)
 idsCount = nodeOperatorsCount - offset
TMP_2790(uint256) = nodeOperatorsCount_1 (c)- offset_1
idsCount_2(uint256) := TMP_2790(uint256)
idsCount_3(uint256) := phi(['idsCount_1', 'idsCount_2'])
 nodeOperatorIds
RETURN nodeOperatorIds_2
```
#### CSModule.onRewardsMinted(uint256) [EXTERNAL]
```slithir
STAKING_ROUTER_ROLE_7(bytes32) := phi(['STAKING_ROUTER_ROLE_8', 'STAKING_ROUTER_ROLE_14', 'STAKING_ROUTER_ROLE_0', 'STAKING_ROUTER_ROLE_16', 'STAKING_ROUTER_ROLE_20', 'STAKING_ROUTER_ROLE_22', 'STAKING_ROUTER_ROLE_24', 'STAKING_ROUTER_ROLE_12', 'STAKING_ROUTER_ROLE_10', 'STAKING_ROUTER_ROLE_18', 'STAKING_ROUTER_ROLE_6'])
STETH_2(IStETH) := phi(['STETH_4', 'STETH_0', 'STETH_1'])
FEE_DISTRIBUTOR_2(address) := phi(['FEE_DISTRIBUTOR_4', 'FEE_DISTRIBUTOR_1', 'FEE_DISTRIBUTOR_0'])
 STETH.transferShares(FEE_DISTRIBUTOR,totalShares)
TMP_2583(uint256) = HIGH_LEVEL_CALL, dest:STETH_3(IStETH), function:transferShares, arguments:['FEE_DISTRIBUTOR_3', 'totalShares_1']  
STETH_4(IStETH) := phi(['STETH_4', 'STETH_3', 'STETH_1'])
FEE_DISTRIBUTOR_4(address) := phi(['FEE_DISTRIBUTOR_4', 'FEE_DISTRIBUTOR_3', 'FEE_DISTRIBUTOR_1'])
 onlyRole(STAKING_ROUTER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(STAKING_ROUTER_ROLE_7)
```
#### CSModule.decreaseVettedSigningKeysCount(bytes,bytes) [EXTERNAL]
```slithir
STAKING_ROUTER_ROLE_15(bytes32) := phi(['STAKING_ROUTER_ROLE_8', 'STAKING_ROUTER_ROLE_14', 'STAKING_ROUTER_ROLE_0', 'STAKING_ROUTER_ROLE_16', 'STAKING_ROUTER_ROLE_20', 'STAKING_ROUTER_ROLE_22', 'STAKING_ROUTER_ROLE_24', 'STAKING_ROUTER_ROLE_12', 'STAKING_ROUTER_ROLE_10', 'STAKING_ROUTER_ROLE_18', 'STAKING_ROUTER_ROLE_6'])
_nodeOperators_19(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 operatorsInReport = ValidatorCountsReport.safeCountOperators(nodeOperatorIds,vettedSigningKeysCounts)
TMP_2607(uint256) = LIBRARY_CALL, dest:ValidatorCountsReport, function:ValidatorCountsReport.safeCountOperators(bytes,bytes), arguments:['nodeOperatorIds_1', 'vettedSigningKeysCounts_1'] 
operatorsInReport_1(uint256) := TMP_2607(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < operatorsInReport
i_2(uint256) := phi(['i_3', 'i_1'])
TMP_2608(bool) = i_2 < operatorsInReport_1
CONDITION TMP_2608
 (nodeOperatorId,vettedSigningKeysCount) = ValidatorCountsReport.next(nodeOperatorIds,vettedSigningKeysCounts,i)
TUPLE_10(uint256,uint256) = LIBRARY_CALL, dest:ValidatorCountsReport, function:ValidatorCountsReport.next(bytes,bytes,uint256), arguments:['nodeOperatorIds_1', 'vettedSigningKeysCounts_1', 'i_2'] 
nodeOperatorId_1(uint256)= UNPACK TUPLE_10 index: 0 
vettedSigningKeysCount_1(uint256)= UNPACK TUPLE_10 index: 1 
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSModule._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
 no = _nodeOperators[nodeOperatorId]
REF_968(NodeOperator) -> _nodeOperators_21[nodeOperatorId_1]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_968(NodeOperator)
 vettedSigningKeysCount >= no.totalVettedKeys
REF_969(uint32) -> no_1 (-> ['_nodeOperators']).totalVettedKeys
TMP_2610(bool) = vettedSigningKeysCount_1 >= REF_969
CONDITION TMP_2610
 revert InvalidVetKeysPointer()()
TMP_2611(None) = SOLIDITY_CALL revert InvalidVetKeysPointer()()
 vettedSigningKeysCount < no.totalDepositedKeys
REF_970(uint32) -> no_1 (-> ['_nodeOperators']).totalDepositedKeys
TMP_2612(bool) = vettedSigningKeysCount_1 < REF_970
CONDITION TMP_2612
 revert InvalidVetKeysPointer()()
TMP_2613(None) = SOLIDITY_CALL revert InvalidVetKeysPointer()()
 no.totalVettedKeys = uint32(vettedSigningKeysCount)
REF_971(uint32) -> no_1 (-> ['_nodeOperators']).totalVettedKeys
TMP_2614 = CONVERT vettedSigningKeysCount_1 to uint32
no_2 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['_nodeOperators'])"])
REF_971(uint32) (->no_2 (-> ['_nodeOperators'])) := TMP_2614(uint32)
_nodeOperators_23(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['_nodeOperators'])"])
 VettedSigningKeysCountChanged(nodeOperatorId,vettedSigningKeysCount)
Emit VettedSigningKeysCountChanged(nodeOperatorId_1,vettedSigningKeysCount_1)
 VettedSigningKeysCountDecreased(nodeOperatorId)
Emit VettedSigningKeysCountDecreased(nodeOperatorId_1)
 _updateDepositableValidatorsCount({nodeOperatorId:nodeOperatorId,incrementNonceIfUpdated:false})
INTERNAL_CALL, CSModule._updateDepositableValidatorsCount(uint256,bool)(nodeOperatorId_1,False)
_nodeOperators_22(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_69', '_nodeOperators_68'])
 ++ i
i_3(uint256) = i_2 (c)+ 1
 _incrementModuleNonce()
INTERNAL_CALL, CSModule._incrementModuleNonce()()
 onlyRole(STAKING_ROUTER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(STAKING_ROUTER_ROLE_15)
```
#### CSModule.updateExitedValidatorsCount(bytes,bytes) [EXTERNAL]
```slithir
STAKING_ROUTER_ROLE_9(bytes32) := phi(['STAKING_ROUTER_ROLE_8', 'STAKING_ROUTER_ROLE_14', 'STAKING_ROUTER_ROLE_0', 'STAKING_ROUTER_ROLE_16', 'STAKING_ROUTER_ROLE_20', 'STAKING_ROUTER_ROLE_22', 'STAKING_ROUTER_ROLE_24', 'STAKING_ROUTER_ROLE_12', 'STAKING_ROUTER_ROLE_10', 'STAKING_ROUTER_ROLE_18', 'STAKING_ROUTER_ROLE_6'])
 operatorsInReport = ValidatorCountsReport.safeCountOperators(nodeOperatorIds,exitedValidatorsCounts)
TMP_2585(uint256) = LIBRARY_CALL, dest:ValidatorCountsReport, function:ValidatorCountsReport.safeCountOperators(bytes,bytes), arguments:['nodeOperatorIds_1', 'exitedValidatorsCounts_1'] 
operatorsInReport_1(uint256) := TMP_2585(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < operatorsInReport
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_2586(bool) = i_2 < operatorsInReport_1
CONDITION TMP_2586
 (nodeOperatorId,exitedValidatorsCount) = ValidatorCountsReport.next(nodeOperatorIds,exitedValidatorsCounts,i)
TUPLE_9(uint256,uint256) = LIBRARY_CALL, dest:ValidatorCountsReport, function:ValidatorCountsReport.next(bytes,bytes,uint256), arguments:['nodeOperatorIds_1', 'exitedValidatorsCounts_1', 'i_2'] 
nodeOperatorId_1(uint256)= UNPACK TUPLE_9 index: 0 
exitedValidatorsCount_1(uint256)= UNPACK TUPLE_9 index: 1 
 _updateExitedValidatorsCount({nodeOperatorId:nodeOperatorId,exitedValidatorsCount:exitedValidatorsCount,allowDecrease:false})
INTERNAL_CALL, CSModule._updateExitedValidatorsCount(uint256,uint256,bool)(nodeOperatorId_1,exitedValidatorsCount_1,False)
 ++ i
i_3(uint256) = i_2 (c)+ 1
 _incrementModuleNonce()
INTERNAL_CALL, CSModule._incrementModuleNonce()()
 onlyRole(STAKING_ROUTER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(STAKING_ROUTER_ROLE_9)
```
#### CSModule.updateTargetValidatorsLimits(uint256,uint256,uint256) [EXTERNAL]
```slithir
STAKING_ROUTER_ROLE_11(bytes32) := phi(['STAKING_ROUTER_ROLE_8', 'STAKING_ROUTER_ROLE_14', 'STAKING_ROUTER_ROLE_0', 'STAKING_ROUTER_ROLE_16', 'STAKING_ROUTER_ROLE_20', 'STAKING_ROUTER_ROLE_22', 'STAKING_ROUTER_ROLE_24', 'STAKING_ROUTER_ROLE_12', 'STAKING_ROUTER_ROLE_10', 'STAKING_ROUTER_ROLE_18', 'STAKING_ROUTER_ROLE_6'])
FORCED_TARGET_LIMIT_MODE_ID_1(uint8) := phi(['FORCED_TARGET_LIMIT_MODE_ID_2', 'FORCED_TARGET_LIMIT_MODE_ID_6', 'FORCED_TARGET_LIMIT_MODE_ID_0'])
_nodeOperators_14(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 targetLimitMode > FORCED_TARGET_LIMIT_MODE_ID
TMP_2590(bool) = targetLimitMode_1 > FORCED_TARGET_LIMIT_MODE_ID_2
CONDITION TMP_2590
 revert InvalidInput()()
TMP_2591(None) = SOLIDITY_CALL revert InvalidInput()()
 targetLimit > type()(uint32).max
TMP_2593(uint32) := 4294967295(uint32)
TMP_2594(bool) = targetLimit_1 > TMP_2593
CONDITION TMP_2594
 revert InvalidInput()()
TMP_2595(None) = SOLIDITY_CALL revert InvalidInput()()
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSModule._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
 no = _nodeOperators[nodeOperatorId]
REF_963(NodeOperator) -> _nodeOperators_16[nodeOperatorId_1]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_963(NodeOperator)
 targetLimitMode == 0
TMP_2597(bool) = targetLimitMode_1 == 0
CONDITION TMP_2597
 targetLimit = 0
targetLimit_2(uint256) := 0(uint256)
targetLimit_3(uint256) := phi(['targetLimit_2', 'targetLimit_1'])
 no.targetLimitMode = uint8(targetLimitMode)
REF_964(uint8) -> no_1 (-> ['_nodeOperators']).targetLimitMode
TMP_2598 = CONVERT targetLimitMode_1 to uint8
no_2 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['_nodeOperators'])"])
REF_964(uint8) (->no_2 (-> ['_nodeOperators'])) := TMP_2598(uint8)
_nodeOperators_17(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['_nodeOperators'])"])
 no.targetLimit = uint32(targetLimit)
REF_965(uint32) -> no_2 (-> ['_nodeOperators']).targetLimit
TMP_2599 = CONVERT targetLimit_3 to uint32
no_3 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_2 (-> ['_nodeOperators'])"])
REF_965(uint32) (->no_3 (-> ['_nodeOperators'])) := TMP_2599(uint32)
_nodeOperators_18(mapping(uint256 => NodeOperator)) := phi(["no_3 (-> ['_nodeOperators'])"])
 TargetValidatorsCountChanged(nodeOperatorId,targetLimitMode,targetLimit)
Emit TargetValidatorsCountChanged(nodeOperatorId_1,targetLimitMode_1,targetLimit_3)
 _updateDepositableValidatorsCount({nodeOperatorId:nodeOperatorId,incrementNonceIfUpdated:false})
INTERNAL_CALL, CSModule._updateDepositableValidatorsCount(uint256,bool)(nodeOperatorId_1,False)
 _incrementModuleNonce()
INTERNAL_CALL, CSModule._incrementModuleNonce()()
 onlyRole(STAKING_ROUTER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(STAKING_ROUTER_ROLE_11)
```
#### CSModule.unsafeUpdateValidatorsCount(uint256,uint256) [EXTERNAL]
```slithir
STAKING_ROUTER_ROLE_13(bytes32) := phi(['STAKING_ROUTER_ROLE_8', 'STAKING_ROUTER_ROLE_14', 'STAKING_ROUTER_ROLE_0', 'STAKING_ROUTER_ROLE_16', 'STAKING_ROUTER_ROLE_20', 'STAKING_ROUTER_ROLE_22', 'STAKING_ROUTER_ROLE_24', 'STAKING_ROUTER_ROLE_12', 'STAKING_ROUTER_ROLE_10', 'STAKING_ROUTER_ROLE_18', 'STAKING_ROUTER_ROLE_6'])
 _updateExitedValidatorsCount({nodeOperatorId:nodeOperatorId,exitedValidatorsCount:exitedValidatorsKeysCount,allowDecrease:true})
INTERNAL_CALL, CSModule._updateExitedValidatorsCount(uint256,uint256,bool)(nodeOperatorId_1,exitedValidatorsKeysCount_1,True)
 _incrementModuleNonce()
INTERNAL_CALL, CSModule._incrementModuleNonce()()
 onlyRole(STAKING_ROUTER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(STAKING_ROUTER_ROLE_13)
```
#### CSModule.obtainDepositData(uint256,bytes) [EXTERNAL]
```slithir
STAKING_ROUTER_ROLE_23(bytes32) := phi(['STAKING_ROUTER_ROLE_8', 'STAKING_ROUTER_ROLE_14', 'STAKING_ROUTER_ROLE_0', 'STAKING_ROUTER_ROLE_16', 'STAKING_ROUTER_ROLE_20', 'STAKING_ROUTER_ROLE_22', 'STAKING_ROUTER_ROLE_24', 'STAKING_ROUTER_ROLE_12', 'STAKING_ROUTER_ROLE_10', 'STAKING_ROUTER_ROLE_18', 'STAKING_ROUTER_ROLE_6'])
QUEUE_LOWEST_PRIORITY_6(uint256) := phi(['QUEUE_LOWEST_PRIORITY_5', 'QUEUE_LOWEST_PRIORITY_14', 'QUEUE_LOWEST_PRIORITY_20', 'QUEUE_LOWEST_PRIORITY_7', 'QUEUE_LOWEST_PRIORITY_1', 'QUEUE_LOWEST_PRIORITY_0', 'QUEUE_LOWEST_PRIORITY_18'])
_nodeOperators_41(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
_totalDepositedValidators_1(uint64) := phi(['_totalDepositedValidators_0', '_totalDepositedValidators_2', '_totalDepositedValidators_8'])
_depositableValidatorsCount_1(uint64) := phi(['_depositableValidatorsCount_0', '_depositableValidatorsCount_2', '_depositableValidatorsCount_8', '_depositableValidatorsCount_13', '_depositableValidatorsCount_12'])
 (publicKeys,signatures) = SigningKeys.initKeysSigsBuf(depositsCount)
TUPLE_12(bytes,bytes) = LIBRARY_CALL, dest:SigningKeys, function:SigningKeys.initKeysSigsBuf(uint256), arguments:['depositsCount_1'] 
publicKeys_1(bytes)= UNPACK TUPLE_12 index: 0 
signatures_1(bytes)= UNPACK TUPLE_12 index: 1 
 depositsCount == 0
TMP_2711(bool) = depositsCount_1 == 0
CONDITION TMP_2711
 (publicKeys,signatures)
RETURN publicKeys_1,signatures_1
 depositsLeft = depositsCount
depositsLeft_1(uint256) := depositsCount_1(uint256)
 loadedKeysCount = 0
loadedKeysCount_1(uint256) := 0(uint256)
 priority = 0
priority_1(uint256) := 0(uint256)
 true
priority_2(uint256) := phi(['priority_1', 'priority_3'])
CONDITION True
 priority > QUEUE_LOWEST_PRIORITY || depositsLeft == 0
TMP_2712(bool) = priority_2 > QUEUE_LOWEST_PRIORITY_7
TMP_2713(bool) = depositsLeft_1 == 0
TMP_2714(bool) = TMP_2712 || TMP_2713
CONDITION TMP_2714
 queue = _getQueue(priority)
TMP_2715(QueueLib.Queue) = INTERNAL_CALL, CSModule._getQueue(uint256)(priority_2)
queue_1 (-> ['TMP_2715'])(QueueLib.Queue) := TMP_2715(QueueLib.Queue)
 ++ priority
priority_3(uint256) = priority_2 + 1
depositsLeft_3(uint256) := phi(['depositsLeft_2', 'depositsLeft_1'])
loadedKeysCount_3(uint256) := phi(['loadedKeysCount_1', 'loadedKeysCount_2'])
 item = queue.peek()
TMP_2716(Batch) = LIBRARY_CALL, dest:QueueLib, function:QueueLib.peek(QueueLib.Queue), arguments:["queue_1 (-> ['TMP_2715'])"] 
item_1(Batch) := TMP_2716(Batch)
 ! item.isNil()
item_2(Batch) := phi(['item_1', 'item_4'])
TMP_2717(bool) = INTERNAL_CALL, isNil(Batch)(item_2)
TMP_2718 = UnaryType.BANG TMP_2717 
CONDITION TMP_2718
 noId = item.noId()
TMP_2719(uint64) = INTERNAL_CALL, noId(Batch)(item_2)
noId_1(uint256) := TMP_2719(uint64)
 keysInBatch = item.keys()
TMP_2720(uint64) = INTERNAL_CALL, keys(Batch)(item_2)
keysInBatch_1(uint256) := TMP_2720(uint64)
 no = _nodeOperators[noId]
REF_1043(NodeOperator) -> _nodeOperators_46[noId_1]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_1043(NodeOperator)
 keysCount = Math.min(Math.min(no.depositableValidatorsCount,keysInBatch),depositsLeft)
REF_1046(uint32) -> no_1 (-> ['_nodeOperators']).depositableValidatorsCount
TMP_2721(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['REF_1046', 'keysInBatch_1'] 
TMP_2722(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['TMP_2721', 'depositsLeft_1'] 
keysCount_1(uint256) := TMP_2722(uint256)
 depositsLeft > keysCount || keysCount == keysInBatch
TMP_2723(bool) = depositsLeft_1 > keysCount_1
TMP_2724(bool) = keysCount_1 == keysInBatch_1
TMP_2725(bool) = TMP_2723 || TMP_2724
CONDITION TMP_2725
 no.enqueuedCount -= uint32(keysInBatch)
REF_1047(uint32) -> no_1 (-> ['_nodeOperators']).enqueuedCount
TMP_2726 = CONVERT keysInBatch_1 to uint32
no_2 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['_nodeOperators'])"])
REF_1047(-> no_2 (-> ['_nodeOperators'])) = REF_1047 - TMP_2726
_nodeOperators_48(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['_nodeOperators'])"])
 queue.dequeue()
TMP_2727(Batch) = LIBRARY_CALL, dest:QueueLib, function:QueueLib.dequeue(QueueLib.Queue), arguments:["queue_1 (-> ['TMP_2715'])"] 
 no.enqueuedCount -= uint32(keysCount)
REF_1049(uint32) -> no_1 (-> ['_nodeOperators']).enqueuedCount
TMP_2728 = CONVERT keysCount_1 to uint32
no_3 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['_nodeOperators'])"])
REF_1049(-> no_3 (-> ['_nodeOperators'])) = REF_1049 - TMP_2728
_nodeOperators_51(mapping(uint256 => NodeOperator)) := phi(["no_3 (-> ['_nodeOperators'])"])
 item = item.setKeys(keysInBatch - keysCount)
TMP_2729(uint256) = keysInBatch_1 - keysCount_1
TMP_2730(Batch) = INTERNAL_CALL, setKeys(Batch,uint256)(item_2,TMP_2729)
item_3(Batch) := TMP_2730(Batch)
 queue.queue[queue.head] = item
REF_1051(mapping(uint128 => Batch)) -> queue_1 (-> ['TMP_2715']).queue
REF_1052(uint128) -> queue_1 (-> ['TMP_2715']).head
REF_1053(Batch) -> REF_1051[REF_1052]
queue_2 (-> ['TMP_2715'])(QueueLib.Queue) := phi(["queue_1 (-> ['TMP_2715'])"])
REF_1053(Batch) (->queue_2 (-> ['TMP_2715'])) := item_3(Batch)
TMP_2715(QueueLib.Queue) := phi(["queue_2 (-> ['TMP_2715'])"])
queue_3 (-> ['TMP_2715'])(QueueLib.Queue) := phi(["queue_1 (-> ['TMP_2715'])", "queue_2 (-> ['TMP_2715'])"])
no_4 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_2 (-> ['_nodeOperators'])", "no_3 (-> ['_nodeOperators'])"])
 keysCount == 0
TMP_2731(bool) = keysCount_1 == 0
CONDITION TMP_2731
 SigningKeys.loadKeysSigs(noId,no.totalDepositedKeys,keysCount,publicKeys,signatures,loadedKeysCount)
REF_1055(uint32) -> no_4 (-> ['_nodeOperators']).totalDepositedKeys
LIBRARY_CALL, dest:SigningKeys, function:SigningKeys.loadKeysSigs(uint256,uint256,uint256,bytes,bytes,uint256), arguments:['noId_1', 'REF_1055', 'keysCount_1', 'publicKeys_1', 'signatures_1', 'loadedKeysCount_1'] 
 loadedKeysCount += keysCount
loadedKeysCount_2(uint256) = loadedKeysCount_1 + keysCount_1
 totalDepositedKeys = no.totalDepositedKeys + uint32(keysCount)
REF_1056(uint32) -> no_4 (-> ['_nodeOperators']).totalDepositedKeys
TMP_2733 = CONVERT keysCount_1 to uint32
TMP_2734(uint32) = REF_1056 + TMP_2733
totalDepositedKeys_1(uint32) := TMP_2734(uint32)
 no.totalDepositedKeys = totalDepositedKeys
REF_1057(uint32) -> no_4 (-> ['_nodeOperators']).totalDepositedKeys
no_5 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_4 (-> ['_nodeOperators'])"])
REF_1057(uint32) (->no_5 (-> ['_nodeOperators'])) := totalDepositedKeys_1(uint32)
_nodeOperators_49(mapping(uint256 => NodeOperator)) := phi(["no_5 (-> ['_nodeOperators'])"])
 DepositedSigningKeysCountChanged(noId,totalDepositedKeys)
Emit DepositedSigningKeysCountChanged(noId_1,totalDepositedKeys_1)
 newCount = no.depositableValidatorsCount - uint32(keysCount)
REF_1058(uint32) -> no_5 (-> ['_nodeOperators']).depositableValidatorsCount
TMP_2736 = CONVERT keysCount_1 to uint32
TMP_2737(uint32) = REF_1058 - TMP_2736
newCount_1(uint32) := TMP_2737(uint32)
 no.depositableValidatorsCount = newCount
REF_1059(uint32) -> no_5 (-> ['_nodeOperators']).depositableValidatorsCount
no_6 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_5 (-> ['_nodeOperators'])"])
REF_1059(uint32) (->no_6 (-> ['_nodeOperators'])) := newCount_1(uint32)
_nodeOperators_50(mapping(uint256 => NodeOperator)) := phi(["no_6 (-> ['_nodeOperators'])"])
 DepositableSigningKeysCountChanged(noId,newCount)
Emit DepositableSigningKeysCountChanged(noId_1,newCount_1)
 depositsLeft -= keysCount
depositsLeft_2(uint256) = depositsLeft_1 - keysCount_1
 depositsLeft == 0
TMP_2739(bool) = depositsLeft_2 == 0
CONDITION TMP_2739
 item = queue.peek()
depositsLeft_4(uint256) := phi(['depositsLeft_2', 'depositsLeft_1'])
loadedKeysCount_4(uint256) := phi(['loadedKeysCount_1', 'loadedKeysCount_2'])
TMP_2740(Batch) = LIBRARY_CALL, dest:QueueLib, function:QueueLib.peek(QueueLib.Queue), arguments:["queue_3 (-> ['TMP_2715'])"] 
item_4(Batch) := TMP_2740(Batch)
 loadedKeysCount != depositsCount
TMP_2741(bool) = loadedKeysCount_1 != depositsCount_1
CONDITION TMP_2741
 revert NotEnoughKeys()()
TMP_2742(None) = SOLIDITY_CALL revert NotEnoughKeys()()
 _depositableValidatorsCount -= uint64(depositsCount)
TMP_2743 = CONVERT depositsCount_1 to uint64
_depositableValidatorsCount_8(uint64) = _depositableValidatorsCount_2 - TMP_2743
 _totalDepositedValidators += uint64(depositsCount)
TMP_2744 = CONVERT depositsCount_1 to uint64
_totalDepositedValidators_8(uint64) = _totalDepositedValidators_2 + TMP_2744
 _incrementModuleNonce()
INTERNAL_CALL, CSModule._incrementModuleNonce()()
 onlyRole(STAKING_ROUTER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(STAKING_ROUTER_ROLE_23)
 (publicKeys,signatures)
RETURN publicKeys_1,signatures_1
```
#### CSModule.onExitedAndStuckValidatorsCountsUpdated() [EXTERNAL]
```slithir

```
#### CSModule.onWithdrawalCredentialsChanged() [EXTERNAL]
```slithir
STAKING_ROUTER_ROLE_17(bytes32) := phi(['STAKING_ROUTER_ROLE_8', 'STAKING_ROUTER_ROLE_14', 'STAKING_ROUTER_ROLE_0', 'STAKING_ROUTER_ROLE_16', 'STAKING_ROUTER_ROLE_20', 'STAKING_ROUTER_ROLE_22', 'STAKING_ROUTER_ROLE_24', 'STAKING_ROUTER_ROLE_12', 'STAKING_ROUTER_ROLE_10', 'STAKING_ROUTER_ROLE_18', 'STAKING_ROUTER_ROLE_6'])
 _incrementModuleNonce()
INTERNAL_CALL, CSModule._incrementModuleNonce()()
 onlyRole(STAKING_ROUTER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(STAKING_ROUTER_ROLE_17)
```
#### CSModule.constructor(bytes32,address,address,address,address) [PUBLIC]
```slithir
 lidoLocator == address(0)
TMP_2498 = CONVERT 0 to address
TMP_2499(bool) = lidoLocator_1 == TMP_2498
CONDITION TMP_2499
 revert ZeroLocatorAddress()()
TMP_2500(None) = SOLIDITY_CALL revert ZeroLocatorAddress()()
 parametersRegistry == address(0)
TMP_2501 = CONVERT 0 to address
TMP_2502(bool) = parametersRegistry_1 == TMP_2501
CONDITION TMP_2502
 revert ZeroParametersRegistryAddress()()
TMP_2503(None) = SOLIDITY_CALL revert ZeroParametersRegistryAddress()()
 _accounting == address(0)
TMP_2504 = CONVERT 0 to address
TMP_2505(bool) = _accounting_1 == TMP_2504
CONDITION TMP_2505
 revert ZeroAccountingAddress()()
TMP_2506(None) = SOLIDITY_CALL revert ZeroAccountingAddress()()
 exitPenalties == address(0)
TMP_2507 = CONVERT 0 to address
TMP_2508(bool) = exitPenalties_1 == TMP_2507
CONDITION TMP_2508
 revert ZeroExitPenaltiesAddress()()
TMP_2509(None) = SOLIDITY_CALL revert ZeroExitPenaltiesAddress()()
 MODULE_TYPE = moduleType
MODULE_TYPE_1(bytes32) := moduleType_1(bytes32)
 LIDO_LOCATOR = ILidoLocator(lidoLocator)
TMP_2510 = CONVERT lidoLocator_1 to ILidoLocator
LIDO_LOCATOR_1(ILidoLocator) := TMP_2510(ILidoLocator)
 STETH = IStETH(LIDO_LOCATOR.lido())
TMP_2511(address) = HIGH_LEVEL_CALL, dest:LIDO_LOCATOR_1(ILidoLocator), function:lido, arguments:[]  
LIDO_LOCATOR_2(ILidoLocator) := phi(['LIDO_LOCATOR_1', 'LIDO_LOCATOR_8', 'LIDO_LOCATOR_2'])
TMP_2512 = CONVERT TMP_2511 to IStETH
STETH_1(IStETH) := TMP_2512(IStETH)
 PARAMETERS_REGISTRY = ICSParametersRegistry(parametersRegistry)
TMP_2513 = CONVERT parametersRegistry_1 to ICSParametersRegistry
PARAMETERS_REGISTRY_1(ICSParametersRegistry) := TMP_2513(ICSParametersRegistry)
 QUEUE_LOWEST_PRIORITY = PARAMETERS_REGISTRY.QUEUE_LOWEST_PRIORITY()
TMP_2514(uint256) = HIGH_LEVEL_CALL, dest:PARAMETERS_REGISTRY_1(ICSParametersRegistry), function:QUEUE_LOWEST_PRIORITY, arguments:[]  
PARAMETERS_REGISTRY_2(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_1', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
QUEUE_LOWEST_PRIORITY_1(uint256) := TMP_2514(uint256)
 QUEUE_LEGACY_PRIORITY = PARAMETERS_REGISTRY.QUEUE_LEGACY_PRIORITY()
TMP_2515(uint256) = HIGH_LEVEL_CALL, dest:PARAMETERS_REGISTRY_2(ICSParametersRegistry), function:QUEUE_LEGACY_PRIORITY, arguments:[]  
PARAMETERS_REGISTRY_3(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_2', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
QUEUE_LEGACY_PRIORITY_1(uint256) := TMP_2515(uint256)
 ACCOUNTING = ICSAccounting(_accounting)
TMP_2516 = CONVERT _accounting_1 to ICSAccounting
ACCOUNTING_1(ICSAccounting) := TMP_2516(ICSAccounting)
 EXIT_PENALTIES = ICSExitPenalties(exitPenalties)
TMP_2517 = CONVERT exitPenalties_1 to ICSExitPenalties
EXIT_PENALTIES_1(ICSExitPenalties) := TMP_2517(ICSExitPenalties)
 FEE_DISTRIBUTOR = address(ACCOUNTING.feeDistributor())
TMP_2518(ICSFeeDistributor) = HIGH_LEVEL_CALL, dest:ACCOUNTING_1(ICSAccounting), function:feeDistributor, arguments:[]  
ACCOUNTING_2(ICSAccounting) := phi(['ACCOUNTING_2', 'ACCOUNTING_1'])
TMP_2519 = CONVERT TMP_2518 to address
FEE_DISTRIBUTOR_1(address) := TMP_2519(address)
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### CSModule.initialize(address) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_4'])
STAKING_ROUTER_ROLE_1(bytes32) := phi(['STAKING_ROUTER_ROLE_8', 'STAKING_ROUTER_ROLE_14', 'STAKING_ROUTER_ROLE_0', 'STAKING_ROUTER_ROLE_16', 'STAKING_ROUTER_ROLE_20', 'STAKING_ROUTER_ROLE_22', 'STAKING_ROUTER_ROLE_24', 'STAKING_ROUTER_ROLE_12', 'STAKING_ROUTER_ROLE_10', 'STAKING_ROUTER_ROLE_18', 'STAKING_ROUTER_ROLE_6'])
LIDO_LOCATOR_3(ILidoLocator) := phi(['LIDO_LOCATOR_0', 'LIDO_LOCATOR_8', 'LIDO_LOCATOR_2'])
 admin == address(0)
TMP_2521 = CONVERT 0 to address
TMP_2522(bool) = admin_1 == TMP_2521
CONDITION TMP_2522
 revert ZeroAdminAddress()()
TMP_2523(None) = SOLIDITY_CALL revert ZeroAdminAddress()()
 __AccessControlEnumerable_init()
INTERNAL_CALL, AccessControlEnumerableUpgradeable.__AccessControlEnumerable_init()()
 _grantRole(DEFAULT_ADMIN_ROLE,admin)
TMP_2525(bool) = INTERNAL_CALL, AccessControlEnumerableUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_3,admin_1)
 _grantRole(STAKING_ROUTER_ROLE,address(LIDO_LOCATOR.stakingRouter()))
TMP_2526(address) = HIGH_LEVEL_CALL, dest:LIDO_LOCATOR_6(ILidoLocator), function:stakingRouter, arguments:[]  
STAKING_ROUTER_ROLE_5(bytes32) := phi(['STAKING_ROUTER_ROLE_8', 'STAKING_ROUTER_ROLE_14', 'STAKING_ROUTER_ROLE_16', 'STAKING_ROUTER_ROLE_20', 'STAKING_ROUTER_ROLE_22', 'STAKING_ROUTER_ROLE_4', 'STAKING_ROUTER_ROLE_24', 'STAKING_ROUTER_ROLE_12', 'STAKING_ROUTER_ROLE_10', 'STAKING_ROUTER_ROLE_18', 'STAKING_ROUTER_ROLE_6'])
LIDO_LOCATOR_7(ILidoLocator) := phi(['LIDO_LOCATOR_8', 'LIDO_LOCATOR_6', 'LIDO_LOCATOR_2'])
TMP_2527 = CONVERT TMP_2526 to address
TMP_2528(bool) = INTERNAL_CALL, AccessControlEnumerableUpgradeable._grantRole(bytes32,address)(STAKING_ROUTER_ROLE_5,TMP_2527)
 _pauseFor(PausableUntil.PAUSE_INFINITELY)
REF_937(uint256) -> PausableUntil.PAUSE_INFINITELY
INTERNAL_CALL, PausableUntil._pauseFor(uint256)(REF_937)
 reinitializer(2)
MODIFIER_CALL, Initializable.reinitializer(uint64)(2)
```
#### CSModule.finalizeUpgradeV2() [EXTERNAL]
```slithir
 sstore(uint256,uint256)(_queueByPriority,0x00)
_queueByPriority_1(mapping(uint256 => QueueLib.Queue)) := 0(uint256)
 sstore(uint256,uint256)(_earlyAdoption,0x00)
_earlyAdoption_1(address) := 0(uint256)
 sstore(uint256,uint256)(_accountingOld,0x00)
_accountingOld_1(ICSAccounting) := 0(uint256)
 reinitializer(2)
MODIFIER_CALL, Initializable.reinitializer(uint64)(2)
```
#### CSModule._incrementModuleNonce() [INTERNAL]
```slithir
_nonce_2(uint256) := phi(['_nonce_3', '_nonce_0'])
 NonceChanged(++ _nonce)
_nonce_3(uint256) = _nonce_2 + 1
Emit NonceChanged(_nonce_3)
```
#### CSModule._addKeysAndUpdateDepositableValidatorsCount(uint256,uint256,bytes,bytes) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1'])
keysCount_1(uint256) := phi(['keysCount_1', 'keysCount_1', 'keysCount_1'])
publicKeys_1(bytes) := phi(['publicKeys_1', 'publicKeys_1', 'publicKeys_1'])
signatures_1(bytes) := phi(['signatures_1', 'signatures_1', 'signatures_1'])
PARAMETERS_REGISTRY_24(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_0', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
_nodeOperators_61(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 _forgetOperatorCreator(nodeOperatorId)
INTERNAL_CALL, CSModule._forgetOperatorCreator(uint256)(nodeOperatorId_1)
 no = _nodeOperators[nodeOperatorId]
REF_1105(NodeOperator) -> _nodeOperators_62[nodeOperatorId_1]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_1105(NodeOperator)
 totalAddedKeys = no.totalAddedKeys
REF_1106(uint32) -> no_1 (-> ['_nodeOperators']).totalAddedKeys
totalAddedKeys_1(uint256) := REF_1106(uint32)
 curveId = accounting().getBondCurveId(nodeOperatorId)
TMP_2799(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
TMP_2800(uint256) = HIGH_LEVEL_CALL, dest:TMP_2799(ICSAccounting), function:getBondCurveId, arguments:['nodeOperatorId_1']  
PARAMETERS_REGISTRY_27(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_26', 'PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
curveId_1(uint256) := TMP_2800(uint256)
 keysLimit = PARAMETERS_REGISTRY.getKeysLimit(curveId)
TMP_2801(uint256) = HIGH_LEVEL_CALL, dest:PARAMETERS_REGISTRY_27(ICSParametersRegistry), function:getKeysLimit, arguments:['curveId_1']  
PARAMETERS_REGISTRY_28(ICSParametersRegistry) := phi(['PARAMETERS_REGISTRY_28', 'PARAMETERS_REGISTRY_3', 'PARAMETERS_REGISTRY_12', 'PARAMETERS_REGISTRY_8', 'PARAMETERS_REGISTRY_27', 'PARAMETERS_REGISTRY_18', 'PARAMETERS_REGISTRY_32', 'PARAMETERS_REGISTRY_23'])
keysLimit_1(uint256) := TMP_2801(uint256)
 totalAddedKeys + keysCount - no.totalWithdrawnKeys > keysLimit
TMP_2802(uint256) = totalAddedKeys_1 + keysCount_1
REF_1109(uint32) -> no_1 (-> ['_nodeOperators']).totalWithdrawnKeys
TMP_2803(uint256) = TMP_2802 - REF_1109
TMP_2804(bool) = TMP_2803 > keysLimit_1
CONDITION TMP_2804
 revert KeysLimitExceeded()()
TMP_2805(None) = SOLIDITY_CALL revert KeysLimitExceeded()()
 newTotalAddedKeys = SigningKeys.saveKeysSigs(nodeOperatorId,totalAddedKeys,keysCount,publicKeys,signatures)
TMP_2806(uint256) = LIBRARY_CALL, dest:SigningKeys, function:SigningKeys.saveKeysSigs(uint256,uint256,uint256,bytes,bytes), arguments:['nodeOperatorId_1', 'totalAddedKeys_1', 'keysCount_1', 'publicKeys_1', 'signatures_1'] 
newTotalAddedKeys_1(uint256) := TMP_2806(uint256)
 totalAddedKeys == no.totalVettedKeys
REF_1111(uint32) -> no_1 (-> ['_nodeOperators']).totalVettedKeys
TMP_2807(bool) = totalAddedKeys_1 == REF_1111
CONDITION TMP_2807
 totalVettedKeys = no.totalVettedKeys + uint32(keysCount)
REF_1112(uint32) -> no_1 (-> ['_nodeOperators']).totalVettedKeys
TMP_2808 = CONVERT keysCount_1 to uint32
TMP_2809(uint32) = REF_1112 + TMP_2808
totalVettedKeys_1(uint32) := TMP_2809(uint32)
 no.totalVettedKeys = totalVettedKeys
REF_1113(uint32) -> no_1 (-> ['_nodeOperators']).totalVettedKeys
no_2 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['_nodeOperators'])"])
REF_1113(uint32) (->no_2 (-> ['_nodeOperators'])) := totalVettedKeys_1(uint32)
_nodeOperators_64(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['_nodeOperators'])"])
 VettedSigningKeysCountChanged(nodeOperatorId,totalVettedKeys)
Emit VettedSigningKeysCountChanged(nodeOperatorId_1,totalVettedKeys_1)
no_3 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['_nodeOperators'])", "no_2 (-> ['_nodeOperators'])"])
 no.totalAddedKeys = uint32(newTotalAddedKeys)
REF_1114(uint32) -> no_3 (-> ['_nodeOperators']).totalAddedKeys
TMP_2811 = CONVERT newTotalAddedKeys_1 to uint32
no_4 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_3 (-> ['_nodeOperators'])"])
REF_1114(uint32) (->no_4 (-> ['_nodeOperators'])) := TMP_2811(uint32)
_nodeOperators_63(mapping(uint256 => NodeOperator)) := phi(["no_4 (-> ['_nodeOperators'])"])
 TotalSigningKeysCountChanged(nodeOperatorId,newTotalAddedKeys)
Emit TotalSigningKeysCountChanged(nodeOperatorId_1,newTotalAddedKeys_1)
 _updateDepositableValidatorsCount({nodeOperatorId:nodeOperatorId,incrementNonceIfUpdated:false})
INTERNAL_CALL, CSModule._updateDepositableValidatorsCount(uint256,bool)(nodeOperatorId_1,False)
 _incrementModuleNonce()
INTERNAL_CALL, CSModule._incrementModuleNonce()()
```
#### CSModule._updateExitedValidatorsCount(uint256,uint256,bool) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1'])
exitedValidatorsCount_1(uint256) := phi(['exitedValidatorsCount_1', 'exitedValidatorsKeysCount_1'])
_nodeOperators_65(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
_totalExitedValidators_2(uint64) := phi(['_totalExitedValidators_3', '_totalExitedValidators_4', '_totalExitedValidators_0'])
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSModule._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
 no = _nodeOperators[nodeOperatorId]
REF_1115(NodeOperator) -> _nodeOperators_66[nodeOperatorId_1]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_1115(NodeOperator)
 totalExitedKeys = no.totalExitedKeys
REF_1116(uint32) -> no_1 (-> ['_nodeOperators']).totalExitedKeys
totalExitedKeys_1(uint32) := REF_1116(uint32)
 exitedValidatorsCount == totalExitedKeys
TMP_2816(bool) = exitedValidatorsCount_1 == totalExitedKeys_1
CONDITION TMP_2816
 exitedValidatorsCount > no.totalDepositedKeys
REF_1117(uint32) -> no_1 (-> ['_nodeOperators']).totalDepositedKeys
TMP_2817(bool) = exitedValidatorsCount_1 > REF_1117
CONDITION TMP_2817
 revert ExitedKeysHigherThanTotalDeposited()()
TMP_2818(None) = SOLIDITY_CALL revert ExitedKeysHigherThanTotalDeposited()()
 ! allowDecrease && exitedValidatorsCount < totalExitedKeys
TMP_2819 = UnaryType.BANG allowDecrease_1 
TMP_2820(bool) = exitedValidatorsCount_1 < totalExitedKeys_1
TMP_2821(bool) = TMP_2819 && TMP_2820
CONDITION TMP_2821
 revert ExitedKeysDecrease()()
TMP_2822(None) = SOLIDITY_CALL revert ExitedKeysDecrease()()
 _totalExitedValidators = (_totalExitedValidators - totalExitedKeys) + uint64(exitedValidatorsCount)
TMP_2823(uint64) = _totalExitedValidators_3 - totalExitedKeys_1
TMP_2824 = CONVERT exitedValidatorsCount_1 to uint64
TMP_2825(uint64) = TMP_2823 + TMP_2824
_totalExitedValidators_4(uint64) := TMP_2825(uint64)
 no.totalExitedKeys = uint32(exitedValidatorsCount)
REF_1118(uint32) -> no_1 (-> ['_nodeOperators']).totalExitedKeys
TMP_2826 = CONVERT exitedValidatorsCount_1 to uint32
no_2 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['_nodeOperators'])"])
REF_1118(uint32) (->no_2 (-> ['_nodeOperators'])) := TMP_2826(uint32)
_nodeOperators_67(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['_nodeOperators'])"])
 ExitedSigningKeysCountChanged(nodeOperatorId,exitedValidatorsCount)
Emit ExitedSigningKeysCountChanged(nodeOperatorId_1,exitedValidatorsCount_1)
```
#### CSModule._updateDepositableValidatorsCount(uint256,bool) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1', 'REF_1035', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1'])
_nodeOperators_68(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
_depositableValidatorsCount_10(uint64) := phi(['_depositableValidatorsCount_0', '_depositableValidatorsCount_2', '_depositableValidatorsCount_8', '_depositableValidatorsCount_13', '_depositableValidatorsCount_12'])
 no = _nodeOperators[nodeOperatorId]
REF_1119(NodeOperator) -> _nodeOperators_68[nodeOperatorId_1]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_1119(NodeOperator)
 totalDepositedKeys = no.totalDepositedKeys
REF_1120(uint32) -> no_1 (-> ['_nodeOperators']).totalDepositedKeys
totalDepositedKeys_1(uint32) := REF_1120(uint32)
 newCount = no.totalVettedKeys - totalDepositedKeys
REF_1121(uint32) -> no_1 (-> ['_nodeOperators']).totalVettedKeys
TMP_2828(uint32) = REF_1121 (c)- totalDepositedKeys_1
newCount_1(uint256) := TMP_2828(uint32)
 unbondedKeys = accounting().getUnbondedKeysCount(nodeOperatorId)
TMP_2829(ICSAccounting) = INTERNAL_CALL, CSModule.accounting()()
TMP_2830(uint256) = HIGH_LEVEL_CALL, dest:TMP_2829(ICSAccounting), function:getUnbondedKeysCount, arguments:['nodeOperatorId_1']  
_depositableValidatorsCount_12(uint64) := phi(['_depositableValidatorsCount_2', '_depositableValidatorsCount_11', '_depositableValidatorsCount_8', '_depositableValidatorsCount_13', '_depositableValidatorsCount_12'])
unbondedKeys_1(uint256) := TMP_2830(uint256)
 nonDeposited = no.totalAddedKeys - totalDepositedKeys
REF_1123(uint32) -> no_1 (-> ['_nodeOperators']).totalAddedKeys
TMP_2831(uint32) = REF_1123 (c)- totalDepositedKeys_1
nonDeposited_1(uint256) := TMP_2831(uint32)
 unbondedKeys >= nonDeposited
TMP_2832(bool) = unbondedKeys_1 >= nonDeposited_1
CONDITION TMP_2832
 newCount = 0
newCount_4(uint256) := 0(uint256)
 unbondedKeys > no.totalAddedKeys - no.totalVettedKeys
REF_1124(uint32) -> no_1 (-> ['_nodeOperators']).totalAddedKeys
REF_1125(uint32) -> no_1 (-> ['_nodeOperators']).totalVettedKeys
TMP_2833(uint32) = REF_1124 (c)- REF_1125
TMP_2834(bool) = unbondedKeys_1 > TMP_2833
CONDITION TMP_2834
 newCount = nonDeposited - unbondedKeys
TMP_2835(uint256) = nonDeposited_1 (c)- unbondedKeys_1
newCount_2(uint256) := TMP_2835(uint256)
newCount_3(uint256) := phi(['newCount_2', 'newCount_1'])
newCount_5(uint256) := phi(['newCount_4', 'newCount_1'])
 no.targetLimitMode > 0 && newCount > 0
REF_1126(uint8) -> no_1 (-> ['_nodeOperators']).targetLimitMode
TMP_2836(bool) = REF_1126 > 0
TMP_2837(bool) = newCount_5 > 0
TMP_2838(bool) = TMP_2836 && TMP_2837
CONDITION TMP_2838
 nonWithdrawnValidators = totalDepositedKeys - no.totalWithdrawnKeys
REF_1127(uint32) -> no_1 (-> ['_nodeOperators']).totalWithdrawnKeys
TMP_2839(uint32) = totalDepositedKeys_1 - REF_1127
nonWithdrawnValidators_1(uint256) := TMP_2839(uint32)
 no.depositableValidatorsCount != newCount
REF_1128(uint32) -> no_1 (-> ['_nodeOperators']).depositableValidatorsCount
TMP_2840(bool) = REF_1128 != newCount_8
CONDITION TMP_2840
 _depositableValidatorsCount = _depositableValidatorsCount - no.depositableValidatorsCount + uint64(newCount)
REF_1129(uint32) -> no_1 (-> ['_nodeOperators']).depositableValidatorsCount
TMP_2841(uint64) = _depositableValidatorsCount_12 - REF_1129
TMP_2842 = CONVERT newCount_8 to uint64
TMP_2843(uint64) = TMP_2841 + TMP_2842
_depositableValidatorsCount_13(uint64) := TMP_2843(uint64)
 no.depositableValidatorsCount = uint32(newCount)
REF_1130(uint32) -> no_1 (-> ['_nodeOperators']).depositableValidatorsCount
TMP_2844 = CONVERT newCount_8 to uint32
no_2 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['_nodeOperators'])"])
REF_1130(uint32) (->no_2 (-> ['_nodeOperators'])) := TMP_2844(uint32)
_nodeOperators_69(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['_nodeOperators'])"])
 DepositableSigningKeysCountChanged(nodeOperatorId,newCount)
Emit DepositableSigningKeysCountChanged(nodeOperatorId_1,newCount_8)
 incrementNonceIfUpdated
CONDITION incrementNonceIfUpdated_1
 _incrementModuleNonce()
INTERNAL_CALL, CSModule._incrementModuleNonce()()
 _enqueueNodeOperatorKeys(nodeOperatorId)
INTERNAL_CALL, CSModule._enqueueNodeOperatorKeys(uint256)(nodeOperatorId_1)
 no.targetLimit > nonWithdrawnValidators
REF_1131(uint32) -> no_1 (-> ['_nodeOperators']).targetLimit
TMP_2848(bool) = REF_1131 > nonWithdrawnValidators_1
CONDITION TMP_2848
 newCount = Math.min(no.targetLimit - nonWithdrawnValidators,newCount)
REF_1133(uint32) -> no_1 (-> ['_nodeOperators']).targetLimit
TMP_2849(uint32) = REF_1133 - nonWithdrawnValidators_1
TMP_2850(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['TMP_2849', 'newCount_5'] 
newCount_6(uint256) := TMP_2850(uint256)
 newCount = Math.min(0,newCount)
TMP_2851(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['0', 'newCount_5'] 
newCount_7(uint256) := TMP_2851(uint256)
newCount_8(uint256) := phi(['newCount_6', 'newCount_7'])
```
#### CSModule._enqueueNodeOperatorKeys(uint256,uint256,uint32) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1'])
queuePriority_1(uint256) := phi(['priority_1', 'QUEUE_LOWEST_PRIORITY_19', 'priority_1'])
count_1(uint32) := phi(['toEnqueue_3', 'count_1', 'toMigrate_1'])
_nodeOperators_75(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 no = _nodeOperators[nodeOperatorId]
REF_1144(NodeOperator) -> _nodeOperators_75[nodeOperatorId_1]
no_1 (-> ['_nodeOperators'])(NodeOperator) := REF_1144(NodeOperator)
 no.enqueuedCount += count
REF_1145(uint32) -> no_1 (-> ['_nodeOperators']).enqueuedCount
no_2 (-> ['_nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['_nodeOperators'])"])
REF_1145(-> no_2 (-> ['_nodeOperators'])) = REF_1145 (c)+ count_1
_nodeOperators_76(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['_nodeOperators'])"])
 q = _getQueue(queuePriority)
TMP_2866(QueueLib.Queue) = INTERNAL_CALL, CSModule._getQueue(uint256)(queuePriority_1)
q_1 (-> ['TMP_2866'])(QueueLib.Queue) := TMP_2866(QueueLib.Queue)
 q.enqueue(nodeOperatorId,count)
TMP_2867(Batch) = LIBRARY_CALL, dest:QueueLib, function:QueueLib.enqueue(QueueLib.Queue,uint256,uint256), arguments:["q_1 (-> ['TMP_2866'])", 'nodeOperatorId_1', 'count_1'] 
 BatchEnqueued(queuePriority,nodeOperatorId,count)
Emit BatchEnqueued(queuePriority_1,nodeOperatorId_1,count_1)
```
#### CSModule._recordOperatorCreator(uint256) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1'])
OPERATORS_CREATED_IN_TX_MAP_TSLOT_1(bytes32) := phi(['OPERATORS_CREATED_IN_TX_MAP_TSLOT_0'])
 map = TransientUintUintMapLib.load(OPERATORS_CREATED_IN_TX_MAP_TSLOT)
TMP_2869(TransientUintUintMap) = LIBRARY_CALL, dest:TransientUintUintMapLib, function:TransientUintUintMapLib.load(bytes32), arguments:['OPERATORS_CREATED_IN_TX_MAP_TSLOT_1'] 
map_1(TransientUintUintMap) := TMP_2869(TransientUintUintMap)
 map.set(nodeOperatorId,uint256(uint160(msg.sender)))
TMP_2870 = CONVERT msg.sender to uint160
TMP_2871 = CONVERT TMP_2870 to uint256
LIBRARY_CALL, dest:TransientUintUintMapLib, function:TransientUintUintMapLib.set(TransientUintUintMap,uint256,uint256), arguments:['map_1', 'nodeOperatorId_1', 'TMP_2871']
```
#### CSModule._forgetOperatorCreator(uint256) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1'])
OPERATORS_CREATED_IN_TX_MAP_TSLOT_2(bytes32) := phi(['OPERATORS_CREATED_IN_TX_MAP_TSLOT_0'])
 map = TransientUintUintMapLib.load(OPERATORS_CREATED_IN_TX_MAP_TSLOT)
TMP_2873(TransientUintUintMap) = LIBRARY_CALL, dest:TransientUintUintMapLib, function:TransientUintUintMapLib.load(bytes32), arguments:['OPERATORS_CREATED_IN_TX_MAP_TSLOT_2'] 
map_1(TransientUintUintMap) := TMP_2873(TransientUintUintMap)
 map.set(nodeOperatorId,0)
LIBRARY_CALL, dest:TransientUintUintMapLib, function:TransientUintUintMapLib.set(TransientUintUintMap,uint256,uint256), arguments:['map_1', 'nodeOperatorId_1', '0']
```
#### CSModule._getOperatorCreator(uint256) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1'])
OPERATORS_CREATED_IN_TX_MAP_TSLOT_3(bytes32) := phi(['OPERATORS_CREATED_IN_TX_MAP_TSLOT_0'])
 map = TransientUintUintMapLib.load(OPERATORS_CREATED_IN_TX_MAP_TSLOT)
TMP_2875(TransientUintUintMap) = LIBRARY_CALL, dest:TransientUintUintMapLib, function:TransientUintUintMapLib.load(bytes32), arguments:['OPERATORS_CREATED_IN_TX_MAP_TSLOT_3'] 
map_1(TransientUintUintMap) := TMP_2875(TransientUintUintMap)
 address(uint160(map.get(nodeOperatorId)))
TMP_2876(uint256) = LIBRARY_CALL, dest:TransientUintUintMapLib, function:TransientUintUintMapLib.get(TransientUintUintMap,uint256), arguments:['map_1', 'nodeOperatorId_1'] 
TMP_2877 = CONVERT TMP_2876 to uint160
TMP_2878 = CONVERT TMP_2877 to address
RETURN TMP_2878
```
#### CSModule._getQueue(uint256) [INTERNAL]
```slithir
priority_1(uint256) := phi(['priority_2', 'queuePriority_1', 'queuePriority_1', 'priority_2', 'queuePriority_1'])
QUEUE_LEGACY_PRIORITY_2(uint256) := phi(['QUEUE_LEGACY_PRIORITY_0', 'QUEUE_LEGACY_PRIORITY_1'])
_queueByPriority_2(mapping(uint256 => QueueLib.Queue)) := phi(['_queueByPriority_0', '_queueByPriority_2', '_queueByPriority_1'])
_legacyQueue_1(QueueLib.Queue) := phi(['_legacyQueue_0'])
 priority == QUEUE_LEGACY_PRIORITY
TMP_2879(bool) = priority_1 == QUEUE_LEGACY_PRIORITY_2
CONDITION TMP_2879
 q = _legacyQueue
q_1 (-> ['_legacyQueue'])(QueueLib.Queue) := _legacyQueue_1(QueueLib.Queue)
 q = _queueByPriority[priority]
REF_1153(QueueLib.Queue) -> _queueByPriority_2[priority_1]
q_2 (-> ['_queueByPriority'])(QueueLib.Queue) := REF_1153(QueueLib.Queue)
q_3 (-> ['_queueByPriority', '_legacyQueue'])(QueueLib.Queue) := phi(["q_1 (-> ['_legacyQueue'])", "q_2 (-> ['_queueByPriority'])"])
 q
RETURN q_3 (-> ['_queueByPriority', '_legacyQueue'])
```
#### CSModule._checkCanAddKeys(uint256,address) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1'])
who_1(address) := phi(['from_1', 'from_1', 'from_1'])
CREATE_NODE_OPERATOR_ROLE_3(bytes32) := phi(['CREATE_NODE_OPERATOR_ROLE_2', 'CREATE_NODE_OPERATOR_ROLE_0', 'CREATE_NODE_OPERATOR_ROLE_4'])
 who == msg.sender
TMP_2880(bool) = who_1 == msg.sender
CONDITION TMP_2880
 _onlyNodeOperatorManager(nodeOperatorId,msg.sender)
INTERNAL_CALL, CSModule._onlyNodeOperatorManager(uint256,address)(nodeOperatorId_1,msg.sender)
 _checkRole(CREATE_NODE_OPERATOR_ROLE)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(CREATE_NODE_OPERATOR_ROLE_3)
 _getOperatorCreator(nodeOperatorId) != msg.sender
TMP_2883(address) = INTERNAL_CALL, CSModule._getOperatorCreator(uint256)(nodeOperatorId_1)
TMP_2884(bool) = TMP_2883 != msg.sender
CONDITION TMP_2884
 revert CannotAddKeys()()
TMP_2885(None) = SOLIDITY_CALL revert CannotAddKeys()()
```
#### CSModule._onlyNodeOperatorManager(uint256,address) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1'])
from_1(address) := phi(['msg.sender'])
_nodeOperators_77(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 managerAddress = _nodeOperators[nodeOperatorId].managerAddress
REF_1154(NodeOperator) -> _nodeOperators_77[nodeOperatorId_1]
REF_1155(address) -> REF_1154.managerAddress
managerAddress_1(address) := REF_1155(address)
 managerAddress == address(0)
TMP_2886 = CONVERT 0 to address
TMP_2887(bool) = managerAddress_1 == TMP_2886
CONDITION TMP_2887
 revert NodeOperatorDoesNotExist()()
TMP_2888(None) = SOLIDITY_CALL revert NodeOperatorDoesNotExist()()
 managerAddress != from
TMP_2889(bool) = managerAddress_1 != from_1
CONDITION TMP_2889
 revert SenderIsNotEligible()()
TMP_2890(None) = SOLIDITY_CALL revert SenderIsNotEligible()()
```
#### CSModule._onlyExistingNodeOperator(uint256) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['REF_999', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1'])
_nodeOperatorsCount_10(uint64) := phi(['_nodeOperatorsCount_0', '_nodeOperatorsCount_5'])
 nodeOperatorId < _nodeOperatorsCount
TMP_2891(bool) = nodeOperatorId_1 < _nodeOperatorsCount_10
CONDITION TMP_2891
 revert NodeOperatorDoesNotExist()()
TMP_2892(None) = SOLIDITY_CALL revert NodeOperatorDoesNotExist()()
```
#### CSModule._onlyValidIndexRange(uint256,uint256,uint256) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1'])
startIndex_1(uint256) := phi(['startIndex_1', 'startIndex_1'])
keysCount_1(uint256) := phi(['keysCount_1', 'keysCount_1'])
_nodeOperators_78(mapping(uint256 => NodeOperator)) := phi(['_nodeOperators_66', '_nodeOperators_27', '_nodeOperators_67', '_nodeOperators_77', '_nodeOperators_54', '_nodeOperators_63', '_nodeOperators_73', '_nodeOperators_57', '_nodeOperators_60', '_nodeOperators_53', '_nodeOperators_78', '_nodeOperators_55', '_nodeOperators_18', '_nodeOperators_0', '_nodeOperators_6', '_nodeOperators_69', '_nodeOperators_31', '_nodeOperators_29', '_nodeOperators_7', '_nodeOperators_20', '_nodeOperators_56', '_nodeOperators_59', '_nodeOperators_76', '_nodeOperators_68', '_nodeOperators_42'])
 startIndex + keysCount > _nodeOperators[nodeOperatorId].totalAddedKeys
TMP_2893(uint256) = startIndex_1 (c)+ keysCount_1
REF_1156(NodeOperator) -> _nodeOperators_78[nodeOperatorId_1]
REF_1157(uint32) -> REF_1156.totalAddedKeys
TMP_2894(bool) = TMP_2893 > REF_1157
CONDITION TMP_2894
 revert SigningKeysInvalidOffset()()
TMP_2895(None) = SOLIDITY_CALL revert SigningKeysInvalidOffset()()
```
#### CSModule._keyPointer(uint256,uint256) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['REF_1004', 'nodeOperatorId_1'])
keyIndex_1(uint256) := phi(['keyIndex_1', 'REF_1005'])
 (nodeOperatorId << 128) | keyIndex
TMP_2897(uint256) = nodeOperatorId_1 << 128
TMP_2898(uint256) = TMP_2897 | keyIndex_1
RETURN TMP_2898
```
#### CSFeeOracle.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
 CONTRACT_VERSION_POSITION = keccak256(bytes)(lido.Versioned.contractVersion)
 PETRIFIED_VERSION_MARK = type()(uint256).max
 MANAGE_CONSENSUS_CONTRACT_ROLE = keccak256(bytes)(MANAGE_CONSENSUS_CONTRACT_ROLE)
 MANAGE_CONSENSUS_VERSION_ROLE = keccak256(bytes)(MANAGE_CONSENSUS_VERSION_ROLE)
 CONSENSUS_CONTRACT_POSITION = keccak256(bytes)(lido.BaseOracle.consensusContract)
 CONSENSUS_VERSION_POSITION = keccak256(bytes)(lido.BaseOracle.consensusVersion)
 LAST_PROCESSING_REF_SLOT_POSITION = keccak256(bytes)(lido.BaseOracle.lastProcessingRefSlot)
 CONSENSUS_REPORT_POSITION = keccak256(bytes)(lido.BaseOracle.consensusReport)
 RESUME_SINCE_TIMESTAMP_POSITION = keccak256(bytes)(lido.PausableUntil.resumeSinceTimestamp)
 PAUSE_INFINITELY = type()(uint256).max
 SUBMIT_DATA_ROLE = keccak256(bytes)(SUBMIT_DATA_ROLE)
 PAUSE_ROLE = keccak256(bytes)(PAUSE_ROLE)
 RESUME_ROLE = keccak256(bytes)(RESUME_ROLE)
 RECOVERER_ROLE = keccak256(bytes)(RECOVERER_ROLE)
 _checkPaused()
INTERNAL_CALL, PausableUntil._checkPaused()()
 _checkResumed()
INTERNAL_CALL, PausableUntil._checkResumed()()
role_1(bytes32) := phi(['RESUME_ROLE_1', 'MANAGE_CONSENSUS_CONTRACT_ROLE_1', 'TMP_2270', 'TMP_2267', 'MANAGE_CONSENSUS_VERSION_ROLE_1', 'PAUSE_ROLE_1', 'TMP_2272', 'TMP_2265'])
 _checkRole(role)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(role_1)
 $ = _getInitializableStorage()
TMP_2366(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_2366'])(Initializable.InitializableStorage) := TMP_2366(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_875(bool) -> $_1 (-> ['TMP_2366'])._initializing
TMP_2367 = UnaryType.BANG REF_875 
isTopLevelCall_1(bool) := TMP_2367(bool)
 initialized = $._initialized
REF_876(uint64) -> $_1 (-> ['TMP_2366'])._initialized
initialized_1(uint64) := REF_876(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_2368(bool) = initialized_1 == 0
TMP_2369(bool) = TMP_2368 && isTopLevelCall_1
initialSetup_1(bool) := TMP_2369(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_2370(bool) = initialized_1 == 1
TMP_2371 = CONVERT this to address
TMP_2372(bytes) = SOLIDITY_CALL code(address)(TMP_2371)
REF_877 -> LENGTH TMP_2372
TMP_2373(bool) = REF_877 == 0
TMP_2374(bool) = TMP_2370 && TMP_2373
construction_1(bool) := TMP_2374(bool)
 ! initialSetup && ! construction
TMP_2375 = UnaryType.BANG initialSetup_1 
TMP_2376 = UnaryType.BANG construction_1 
TMP_2377(bool) = TMP_2375 && TMP_2376
CONDITION TMP_2377
 revert InvalidInitialization()()
TMP_2378(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_878(uint64) -> $_1 (-> ['TMP_2366'])._initialized
$_2 (-> ['TMP_2366'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_2366'])"])
REF_878(uint64) (->$_2 (-> ['TMP_2366'])) := 1(uint256)
TMP_2366(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_2366'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_879(bool) -> $_2 (-> ['TMP_2366'])._initializing
$_3 (-> ['TMP_2366'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_2366'])"])
REF_879(bool) (->$_3 (-> ['TMP_2366'])) := True(bool)
TMP_2366(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_2366'])"])
$_4 (-> ['TMP_2366'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_2366'])", "$_2 (-> ['TMP_2366'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_880(bool) -> $_4 (-> ['TMP_2366'])._initializing
$_5 (-> ['TMP_2366'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_2366'])"])
REF_880(bool) (->$_5 (-> ['TMP_2366'])) := False(bool)
TMP_2366(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_2366'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_2380(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_2380'])(Initializable.InitializableStorage) := TMP_2380(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_881(bool) -> $_1 (-> ['TMP_2380'])._initializing
REF_882(uint64) -> $_1 (-> ['TMP_2380'])._initialized
TMP_2381(bool) = REF_882 >= version_1
TMP_2382(bool) = REF_881 || TMP_2381
CONDITION TMP_2382
 revert InvalidInitialization()()
TMP_2383(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_883(uint64) -> $_1 (-> ['TMP_2380'])._initialized
$_2 (-> ['TMP_2380'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_2380'])"])
REF_883(uint64) (->$_2 (-> ['TMP_2380'])) := version_1(uint64)
TMP_2380(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_2380'])"])
 $._initializing = true
REF_884(bool) -> $_2 (-> ['TMP_2380'])._initializing
$_3 (-> ['TMP_2380'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_2380'])"])
REF_884(bool) (->$_3 (-> ['TMP_2380'])) := True(bool)
TMP_2380(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_2380'])"])
 $._initializing = false
REF_885(bool) -> $_3 (-> ['TMP_2380'])._initializing
$_4 (-> ['TMP_2380'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_2380'])"])
REF_885(bool) (->$_4 (-> ['TMP_2380'])) := False(bool)
TMP_2380(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_2380'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
```
#### ICSAccounting.getRequiredBondForNextKeys(uint256,uint256) [EXTERNAL]
```slithir

```
#### ICSAccounting.depositETH(uint256) [EXTERNAL]
```slithir

```
#### ICSAccounting.depositStETH(uint256,uint256,ICSAccounting.PermitInput) [EXTERNAL]
```slithir

```
#### ICSAccounting.getRequiredBondForNextKeysWstETH(uint256,uint256) [EXTERNAL]
```slithir

```
#### ICSAccounting.depositWstETH(uint256,uint256,ICSAccounting.PermitInput) [EXTERNAL]
```slithir

```
#### ICSParametersRegistry.getElRewardsStealingAdditionalFine(uint256) [EXTERNAL]
```slithir

```
#### ICSAccounting.lockBondETH(uint256,uint256) [EXTERNAL]
```slithir

```
#### ICSAccounting.compensateLockedBondETH(uint256) [EXTERNAL]
```slithir

```
#### ICSAccounting.releaseLockedBondETH(uint256,uint256) [EXTERNAL]
```slithir

```
#### ICSAccounting.settleLockedBondETH(uint256) [EXTERNAL]
```slithir

```
#### NOAddresses.proposeNodeOperatorManagerAddressChange(mapping(uint256 => NodeOperator),uint256,address) [EXTERNAL]
```slithir
 no = nodeOperators[nodeOperatorId]
REF_1875(NodeOperator) -> nodeOperators_1 (-> [])[nodeOperatorId_1]
no_1 (-> ['nodeOperators'])(NodeOperator) := REF_1875(NodeOperator)
 no.managerAddress == address(0)
REF_1876(address) -> no_1 (-> ['nodeOperators']).managerAddress
TMP_4287 = CONVERT 0 to address
TMP_4288(bool) = REF_1876 == TMP_4287
CONDITION TMP_4288
 ICSModule.NodeOperatorDoesNotExist()
TMP_4289(None) = SOLIDITY_CALL revert NodeOperatorDoesNotExist()()
 no.managerAddress != msg.sender
REF_1877(address) -> no_1 (-> ['nodeOperators']).managerAddress
TMP_4290(bool) = REF_1877 != msg.sender
CONDITION TMP_4290
 INOAddresses.SenderIsNotManagerAddress()
TMP_4291(None) = SOLIDITY_CALL revert SenderIsNotManagerAddress()()
 no.managerAddress == proposedAddress
REF_1878(address) -> no_1 (-> ['nodeOperators']).managerAddress
TMP_4292(bool) = REF_1878 == proposedAddress_1
CONDITION TMP_4292
 INOAddresses.SameAddress()
TMP_4293(None) = SOLIDITY_CALL revert SameAddress()()
 no.proposedManagerAddress == proposedAddress
REF_1879(address) -> no_1 (-> ['nodeOperators']).proposedManagerAddress
TMP_4294(bool) = REF_1879 == proposedAddress_1
CONDITION TMP_4294
 INOAddresses.AlreadyProposed()
TMP_4295(None) = SOLIDITY_CALL revert AlreadyProposed()()
 oldProposedAddress = no.proposedManagerAddress
REF_1880(address) -> no_1 (-> ['nodeOperators']).proposedManagerAddress
oldProposedAddress_1(address) := REF_1880(address)
 no.proposedManagerAddress = proposedAddress
REF_1881(address) -> no_1 (-> ['nodeOperators']).proposedManagerAddress
no_2 (-> ['nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['nodeOperators'])"])
REF_1881(address) (->no_2 (-> ['nodeOperators'])) := proposedAddress_1(address)
nodeOperators_2 (-> ['nodeOperators'])(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['nodeOperators'])"])
 INOAddresses.NodeOperatorManagerAddressChangeProposed(nodeOperatorId,oldProposedAddress,proposedAddress)
Emit NodeOperatorManagerAddressChangeProposed(nodeOperatorId_1,oldProposedAddress_1,proposedAddress_1)
```
#### NOAddresses.confirmNodeOperatorManagerAddressChange(mapping(uint256 => NodeOperator),uint256) [EXTERNAL]
```slithir
 no = nodeOperators[nodeOperatorId]
REF_1883(NodeOperator) -> nodeOperators_1 (-> [])[nodeOperatorId_1]
no_1 (-> ['nodeOperators'])(NodeOperator) := REF_1883(NodeOperator)
 no.managerAddress == address(0)
REF_1884(address) -> no_1 (-> ['nodeOperators']).managerAddress
TMP_4297 = CONVERT 0 to address
TMP_4298(bool) = REF_1884 == TMP_4297
CONDITION TMP_4298
 ICSModule.NodeOperatorDoesNotExist()
TMP_4299(None) = SOLIDITY_CALL revert NodeOperatorDoesNotExist()()
 no.proposedManagerAddress != msg.sender
REF_1885(address) -> no_1 (-> ['nodeOperators']).proposedManagerAddress
TMP_4300(bool) = REF_1885 != msg.sender
CONDITION TMP_4300
 INOAddresses.SenderIsNotProposedAddress()
TMP_4301(None) = SOLIDITY_CALL revert SenderIsNotProposedAddress()()
 oldAddress = no.managerAddress
REF_1886(address) -> no_1 (-> ['nodeOperators']).managerAddress
oldAddress_1(address) := REF_1886(address)
 no.managerAddress = msg.sender
REF_1887(address) -> no_1 (-> ['nodeOperators']).managerAddress
no_2 (-> ['nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['nodeOperators'])"])
REF_1887(address) (->no_2 (-> ['nodeOperators'])) := msg.sender(address)
nodeOperators_2 (-> ['nodeOperators'])(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['nodeOperators'])"])
 delete no.proposedManagerAddress
REF_1888(address) -> no_2 (-> ['nodeOperators']).proposedManagerAddress
no_3 (-> []) = delete REF_1888 
 INOAddresses.NodeOperatorManagerAddressChanged(nodeOperatorId,oldAddress,msg.sender)
Emit NodeOperatorManagerAddressChanged(nodeOperatorId_1,oldAddress_1,msg.sender)
```
#### NOAddresses.resetNodeOperatorManagerAddress(mapping(uint256 => NodeOperator),uint256) [EXTERNAL]
```slithir
 no = nodeOperators[nodeOperatorId]
REF_1905(NodeOperator) -> nodeOperators_1 (-> [])[nodeOperatorId_1]
no_1 (-> ['nodeOperators'])(NodeOperator) := REF_1905(NodeOperator)
 no.rewardAddress == address(0)
REF_1906(address) -> no_1 (-> ['nodeOperators']).rewardAddress
TMP_4319 = CONVERT 0 to address
TMP_4320(bool) = REF_1906 == TMP_4319
CONDITION TMP_4320
 ICSModule.NodeOperatorDoesNotExist()
TMP_4321(None) = SOLIDITY_CALL revert NodeOperatorDoesNotExist()()
 no.extendedManagerPermissions
REF_1907(bool) -> no_1 (-> ['nodeOperators']).extendedManagerPermissions
CONDITION REF_1907
 INOAddresses.MethodCallIsNotAllowed()
TMP_4322(None) = SOLIDITY_CALL revert MethodCallIsNotAllowed()()
 no.rewardAddress != msg.sender
REF_1908(address) -> no_1 (-> ['nodeOperators']).rewardAddress
TMP_4323(bool) = REF_1908 != msg.sender
CONDITION TMP_4323
 INOAddresses.SenderIsNotRewardAddress()
TMP_4324(None) = SOLIDITY_CALL revert SenderIsNotRewardAddress()()
 no.managerAddress == no.rewardAddress
REF_1909(address) -> no_1 (-> ['nodeOperators']).managerAddress
REF_1910(address) -> no_1 (-> ['nodeOperators']).rewardAddress
TMP_4325(bool) = REF_1909 == REF_1910
CONDITION TMP_4325
 INOAddresses.SameAddress()
TMP_4326(None) = SOLIDITY_CALL revert SameAddress()()
 previousManagerAddress = no.managerAddress
REF_1911(address) -> no_1 (-> ['nodeOperators']).managerAddress
previousManagerAddress_1(address) := REF_1911(address)
 no.managerAddress = no.rewardAddress
REF_1912(address) -> no_1 (-> ['nodeOperators']).managerAddress
REF_1913(address) -> no_1 (-> ['nodeOperators']).rewardAddress
no_2 (-> ['nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['nodeOperators'])"])
REF_1912(address) (->no_2 (-> ['nodeOperators'])) := REF_1913(address)
nodeOperators_2 (-> ['nodeOperators'])(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['nodeOperators'])"])
 no.proposedManagerAddress != address(0)
REF_1914(address) -> no_2 (-> ['nodeOperators']).proposedManagerAddress
TMP_4327 = CONVERT 0 to address
TMP_4328(bool) = REF_1914 != TMP_4327
CONDITION TMP_4328
 delete no.proposedManagerAddress
REF_1915(address) -> no_2 (-> ['nodeOperators']).proposedManagerAddress
no_3 (-> []) = delete REF_1915 
no_4 (-> ['nodeOperators'])(NodeOperator) := phi(['no_3 (-> [])', "no_2 (-> ['nodeOperators'])"])
 INOAddresses.NodeOperatorManagerAddressChanged(nodeOperatorId,previousManagerAddress,no.rewardAddress)
REF_1917(address) -> no_4 (-> ['nodeOperators']).rewardAddress
Emit NodeOperatorManagerAddressChanged(nodeOperatorId_1,previousManagerAddress_1,REF_1917)
```
#### NOAddresses.proposeNodeOperatorRewardAddressChange(mapping(uint256 => NodeOperator),uint256,address) [EXTERNAL]
```slithir
 no = nodeOperators[nodeOperatorId]
REF_1890(NodeOperator) -> nodeOperators_1 (-> [])[nodeOperatorId_1]
no_1 (-> ['nodeOperators'])(NodeOperator) := REF_1890(NodeOperator)
 no.rewardAddress == address(0)
REF_1891(address) -> no_1 (-> ['nodeOperators']).rewardAddress
TMP_4303 = CONVERT 0 to address
TMP_4304(bool) = REF_1891 == TMP_4303
CONDITION TMP_4304
 ICSModule.NodeOperatorDoesNotExist()
TMP_4305(None) = SOLIDITY_CALL revert NodeOperatorDoesNotExist()()
 no.rewardAddress != msg.sender
REF_1892(address) -> no_1 (-> ['nodeOperators']).rewardAddress
TMP_4306(bool) = REF_1892 != msg.sender
CONDITION TMP_4306
 INOAddresses.SenderIsNotRewardAddress()
TMP_4307(None) = SOLIDITY_CALL revert SenderIsNotRewardAddress()()
 no.rewardAddress == proposedAddress
REF_1893(address) -> no_1 (-> ['nodeOperators']).rewardAddress
TMP_4308(bool) = REF_1893 == proposedAddress_1
CONDITION TMP_4308
 INOAddresses.SameAddress()
TMP_4309(None) = SOLIDITY_CALL revert SameAddress()()
 no.proposedRewardAddress == proposedAddress
REF_1894(address) -> no_1 (-> ['nodeOperators']).proposedRewardAddress
TMP_4310(bool) = REF_1894 == proposedAddress_1
CONDITION TMP_4310
 INOAddresses.AlreadyProposed()
TMP_4311(None) = SOLIDITY_CALL revert AlreadyProposed()()
 oldProposedAddress = no.proposedRewardAddress
REF_1895(address) -> no_1 (-> ['nodeOperators']).proposedRewardAddress
oldProposedAddress_1(address) := REF_1895(address)
 no.proposedRewardAddress = proposedAddress
REF_1896(address) -> no_1 (-> ['nodeOperators']).proposedRewardAddress
no_2 (-> ['nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['nodeOperators'])"])
REF_1896(address) (->no_2 (-> ['nodeOperators'])) := proposedAddress_1(address)
nodeOperators_2 (-> ['nodeOperators'])(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['nodeOperators'])"])
 INOAddresses.NodeOperatorRewardAddressChangeProposed(nodeOperatorId,oldProposedAddress,proposedAddress)
Emit NodeOperatorRewardAddressChangeProposed(nodeOperatorId_1,oldProposedAddress_1,proposedAddress_1)
```
#### NOAddresses.confirmNodeOperatorRewardAddressChange(mapping(uint256 => NodeOperator),uint256) [EXTERNAL]
```slithir
 no = nodeOperators[nodeOperatorId]
REF_1898(NodeOperator) -> nodeOperators_1 (-> [])[nodeOperatorId_1]
no_1 (-> ['nodeOperators'])(NodeOperator) := REF_1898(NodeOperator)
 no.rewardAddress == address(0)
REF_1899(address) -> no_1 (-> ['nodeOperators']).rewardAddress
TMP_4313 = CONVERT 0 to address
TMP_4314(bool) = REF_1899 == TMP_4313
CONDITION TMP_4314
 ICSModule.NodeOperatorDoesNotExist()
TMP_4315(None) = SOLIDITY_CALL revert NodeOperatorDoesNotExist()()
 no.proposedRewardAddress != msg.sender
REF_1900(address) -> no_1 (-> ['nodeOperators']).proposedRewardAddress
TMP_4316(bool) = REF_1900 != msg.sender
CONDITION TMP_4316
 INOAddresses.SenderIsNotProposedAddress()
TMP_4317(None) = SOLIDITY_CALL revert SenderIsNotProposedAddress()()
 oldAddress = no.rewardAddress
REF_1901(address) -> no_1 (-> ['nodeOperators']).rewardAddress
oldAddress_1(address) := REF_1901(address)
 no.rewardAddress = msg.sender
REF_1902(address) -> no_1 (-> ['nodeOperators']).rewardAddress
no_2 (-> ['nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['nodeOperators'])"])
REF_1902(address) (->no_2 (-> ['nodeOperators'])) := msg.sender(address)
nodeOperators_2 (-> ['nodeOperators'])(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['nodeOperators'])"])
 delete no.proposedRewardAddress
REF_1903(address) -> no_2 (-> ['nodeOperators']).proposedRewardAddress
no_3 (-> []) = delete REF_1903 
 INOAddresses.NodeOperatorRewardAddressChanged(nodeOperatorId,oldAddress,msg.sender)
Emit NodeOperatorRewardAddressChanged(nodeOperatorId_1,oldAddress_1,msg.sender)
```


#### TransientUintUintMapLib.create() [INTERNAL]
```slithir
 anchor = 0x6e38e7eaa4307e6ee6c66720337876ca65012869fbef035f57219354c1728400
anchor_1(uint256) := 49854957409111686035671519084563680553609307894666343862696953572565795898368(uint256)
 prev_create_asm_0 = tload(uint256)(anchor)
TMP_4644(uint256) = SOLIDITY_CALL tload(uint256)(anchor_1)
prev_create_asm_0_1(uint256) := TMP_4644(uint256)
 mstore(uint256,uint256)(0x00,anchor)
TMP_4645(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,anchor_1)
 mstore(uint256,uint256)(0x20,prev_create_asm_0)
TMP_4646(None) = SOLIDITY_CALL mstore(uint256,uint256)(32,prev_create_asm_0_1)
 self = keccak256(uint256,uint256)(0x00,0x40)
TMP_4647(uint256) = SOLIDITY_CALL keccak256(uint256,uint256)(0,64)
self_1(TransientUintUintMap) := TMP_4647(uint256)
 tstore(uint256,uint256)(anchor,self)
TMP_4648(None) = SOLIDITY_CALL tstore(uint256,uint256)(anchor_1,self_1)
 self
RETURN self_1
```
#### QueueLib.clean(QueueLib.Queue,mapping(uint256 => NodeOperator),uint256,TransientUintUintMap) [EXTERNAL]
```slithir
 removed = 0
removed_1(uint256) := 0(uint256)
 lastRemovedAtDepth = 0
lastRemovedAtDepth_1(uint256) := 0(uint256)
 visited = 0
visited_1(uint256) := 0(uint256)
 reachedOutOfQueue = false
reachedOutOfQueue_1(bool) := False(bool)
 maxItems == 0
TMP_4343(bool) = maxItems_1 == 0
CONDITION TMP_4343
 IQueueLib.QueueLookupNoLimit()
TMP_4344(None) = SOLIDITY_CALL revert QueueLookupNoLimit()()
 head = self.head
REF_1927(uint128) -> self_1 (-> []).head
head_1(uint128) := REF_1927(uint128)
 curr = head
curr_1(uint128) := head_1(uint128)
 visited < maxItems
visited_2(uint256) := phi(['visited_1', 'visited_3'])
curr_2(uint128) := phi(['curr_1', 'curr_3'])
TMP_4345(bool) = visited_2 < maxItems_1
CONDITION TMP_4345
 item = self.queue[curr]
REF_1928(mapping(uint128 => Batch)) -> self_1 (-> []).queue
REF_1929(Batch) -> REF_1928[curr_2]
item_1(Batch) := REF_1929(Batch)
 item.isNil()
TMP_4346(bool) = INTERNAL_CALL, isNil(Batch)(item_1)
CONDITION TMP_4346
 reachedOutOfQueue = true
reachedOutOfQueue_2(bool) := True(bool)
 visited ++
TMP_4347(uint256) := visited_2(uint256)
visited_3(uint256) = visited_2 (c)+ 1
 no = nodeOperators[item.noId()]
TMP_4348(uint64) = INTERNAL_CALL, noId(Batch)(item_1)
REF_1932(NodeOperator) -> nodeOperators_1 (-> [])[TMP_4348]
no_1 (-> ['nodeOperators'])(NodeOperator) := REF_1932(NodeOperator)
 queueLookup.get(item.noId()) >= no.depositableValidatorsCount
TMP_4349(uint64) = INTERNAL_CALL, noId(Batch)(item_1)
TMP_4350(uint256) = LIBRARY_CALL, dest:TransientUintUintMapLib, function:TransientUintUintMapLib.get(TransientUintUintMap,uint256), arguments:['queueLookup_1', 'TMP_4349'] 
REF_1935(uint32) -> no_1 (-> ['nodeOperators']).depositableValidatorsCount
TMP_4351(bool) = TMP_4350 >= REF_1935
CONDITION TMP_4351
 curr == head
TMP_4352(bool) = curr_2 == head_1
CONDITION TMP_4352
 self.dequeue()
TMP_4353(Batch) = LIBRARY_CALL, dest:QueueLib, function:QueueLib.dequeue(QueueLib.Queue), arguments:['self_1 (-> [])'] 
 head = self.head
REF_1937(uint128) -> self_1 (-> []).head
head_2(uint128) := REF_1937(uint128)
 prevItem = prevItem.setNext(item.next())
TMP_4354(uint128) = INTERNAL_CALL, next(Batch)(item_1)
TMP_4355(Batch) = INTERNAL_CALL, setNext(Batch,uint128)(prevItem_0,TMP_4354)
prevItem_2(Batch) := TMP_4355(Batch)
 self.queue[indexOfPrev] = prevItem
REF_1940(mapping(uint128 => Batch)) -> self_1 (-> []).queue
REF_1941(Batch) -> REF_1940[indexOfPrev_0]
self_2 (-> [])(QueueLib.Queue) := phi(['self_1 (-> [])'])
REF_1941(Batch) (->self_2 (-> [])) := prevItem_2(Batch)
self_3 (-> [])(QueueLib.Queue) := phi(['self_2 (-> [])', 'self_1 (-> [])'])
head_3(uint128) := phi(['head_2', 'head_1'])
 no.enqueuedCount -= uint32(item.keys())
REF_1942(uint32) -> no_1 (-> ['nodeOperators']).enqueuedCount
TMP_4356(uint64) = INTERNAL_CALL, keys(Batch)(item_1)
TMP_4357 = CONVERT TMP_4356 to uint32
no_2 (-> ['nodeOperators'])(NodeOperator) := phi(["no_1 (-> ['nodeOperators'])"])
REF_1942(-> no_2 (-> ['nodeOperators'])) = REF_1942 (c)- TMP_4357
nodeOperators_2 (-> ['nodeOperators'])(mapping(uint256 => NodeOperator)) := phi(["no_2 (-> ['nodeOperators'])"])
 lastRemovedAtDepth = visited
lastRemovedAtDepth_2(uint256) := visited_3(uint256)
 ++ removed
removed_2(uint256) = removed_1 + 1
 queueLookup.add(item.noId(),item.keys())
TMP_4358(uint64) = INTERNAL_CALL, noId(Batch)(item_1)
TMP_4359(uint64) = INTERNAL_CALL, keys(Batch)(item_1)
LIBRARY_CALL, dest:TransientUintUintMapLib, function:TransientUintUintMapLib.add(TransientUintUintMap,uint256,uint256), arguments:['queueLookup_1', 'TMP_4358', 'TMP_4359'] 
 indexOfPrev = curr
indexOfPrev_1(uint128) := curr_2(uint128)
 prevItem = item
prevItem_1(Batch) := item_1(Batch)
removed_3(uint256) := phi(['removed_1', 'removed_2'])
lastRemovedAtDepth_3(uint256) := phi(['lastRemovedAtDepth_2', 'lastRemovedAtDepth_1'])
 curr = item.next()
TMP_4361(uint128) = INTERNAL_CALL, next(Batch)(item_1)
curr_3(uint128) := TMP_4361(uint128)
reachedOutOfQueue_3(bool) := phi(['reachedOutOfQueue_2', 'reachedOutOfQueue_1'])
 (removed,lastRemovedAtDepth,visited,reachedOutOfQueue)
RETURN removed_1,lastRemovedAtDepth_1,visited_2,reachedOutOfQueue_3
```
#### ICSParametersRegistry.getQueueConfig(uint256) [EXTERNAL]
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
#### SigningKeys.loadKeys(uint256,uint256,uint256) [INTERNAL]
```slithir
SIGNING_KEYS_POSITION_4(bytes32) := phi(['SIGNING_KEYS_POSITION_0'])
PUBKEY_LENGTH_2(uint64) := phi(['PUBKEY_LENGTH_0'])
 pubkeys = new bytes(keysCount * PUBKEY_LENGTH)
TMP_4618(uint256) = keysCount_1 (c)* PUBKEY_LENGTH_2
TMP_4619 = new bytes(TMP_4618)
pubkeys_1(bytes) := TMP_4619(bytes)
 i < keysCount
i_1(uint256) := phi(['i_0', 'i_2'])
TMP_4620(bool) = i_1 < keysCount_1
CONDITION TMP_4620
 curOffset = SIGNING_KEYS_POSITION.getKeyOffset(nodeOperatorId,startIndex + i)
TMP_4621(uint256) = startIndex_1 (c)+ i_1
TMP_4622(uint256) = LIBRARY_CALL, dest:SigningKeys, function:SigningKeys.getKeyOffset(bytes32,uint256,uint256), arguments:['SIGNING_KEYS_POSITION_4', 'nodeOperatorId_1', 'TMP_4621'] 
curOffset_1(uint256) := TMP_4622(uint256)
 offset_loadKeys_asm_0 = pubkeys + 0x20 + i * 48
TMP_4623(bytes) = pubkeys_1 + 32
TMP_4624(uint256) = i_1 * 48
TMP_4625(bytes) = TMP_4623 + TMP_4624
offset_loadKeys_asm_0_1(uint256) := TMP_4625(bytes)
 mstore(uint256,uint256)(offset_loadKeys_asm_0 + 0x10,sload(uint256)(curOffset + 1) >> 128)
TMP_4626(uint256) = offset_loadKeys_asm_0_1 + 16
TMP_4627(uint256) = curOffset_1 + 1
TMP_4628(uint256) = SOLIDITY_CALL sload(uint256)(TMP_4627)
TMP_4629(uint256) = TMP_4628 >> 128
TMP_4630(None) = SOLIDITY_CALL mstore(uint256,uint256)(TMP_4626,TMP_4629)
 mstore(uint256,uint256)(offset_loadKeys_asm_0,sload(uint256)(curOffset))
TMP_4631(uint256) = SOLIDITY_CALL sload(uint256)(curOffset_1)
TMP_4632(None) = SOLIDITY_CALL mstore(uint256,uint256)(offset_loadKeys_asm_0_1,TMP_4631)
 i = i + 1
TMP_4633(uint256) = i_1 + 1
i_2(uint256) := TMP_4633(uint256)
 pubkeys
RETURN pubkeys_1
```
#### SigningKeys.loadKeysSigs(uint256,uint256,uint256,bytes,bytes,uint256) [INTERNAL]
```slithir
SIGNING_KEYS_POSITION_3(bytes32) := phi(['SIGNING_KEYS_POSITION_0'])
 i < keysCount
i_1(uint256) := phi(['i_2', 'i_0'])
TMP_4587(bool) = i_1 < keysCount_1
CONDITION TMP_4587
 curOffset = SIGNING_KEYS_POSITION.getKeyOffset(nodeOperatorId,startIndex + i)
TMP_4588(uint256) = startIndex_1 (c)+ i_1
TMP_4589(uint256) = LIBRARY_CALL, dest:SigningKeys, function:SigningKeys.getKeyOffset(bytes32,uint256,uint256), arguments:['SIGNING_KEYS_POSITION_3', 'nodeOperatorId_1', 'TMP_4588'] 
curOffset_1(uint256) := TMP_4589(uint256)
 _ofs_loadKeysSigs_asm_0 = pubkeys + 0x20 + bufOffset + i * 48
TMP_4590(bytes) = pubkeys_1 + 32
TMP_4591(uint256) = bufOffset_1 + i_1
TMP_4592(uint256) = TMP_4591 * 48
TMP_4593(bytes) = TMP_4590 + TMP_4592
_ofs_loadKeysSigs_asm_0_1(uint256) := TMP_4593(bytes)
 mstore(uint256,uint256)(_ofs_loadKeysSigs_asm_0 + 0x10,sload(uint256)(curOffset + 1) >> 128)
TMP_4594(uint256) = _ofs_loadKeysSigs_asm_0_1 + 16
TMP_4595(uint256) = curOffset_1 + 1
TMP_4596(uint256) = SOLIDITY_CALL sload(uint256)(TMP_4595)
TMP_4597(uint256) = TMP_4596 >> 128
TMP_4598(None) = SOLIDITY_CALL mstore(uint256,uint256)(TMP_4594,TMP_4597)
 mstore(uint256,uint256)(_ofs_loadKeysSigs_asm_0,sload(uint256)(curOffset))
TMP_4599(uint256) = SOLIDITY_CALL sload(uint256)(curOffset_1)
TMP_4600(None) = SOLIDITY_CALL mstore(uint256,uint256)(_ofs_loadKeysSigs_asm_0_1,TMP_4599)
 _ofs_loadKeysSigs_asm_0 = signatures + 0x20 + bufOffset + i * 96
TMP_4601(bytes) = signatures_1 + 32
TMP_4602(uint256) = bufOffset_1 + i_1
TMP_4603(uint256) = TMP_4602 * 96
TMP_4604(bytes) = TMP_4601 + TMP_4603
_ofs_loadKeysSigs_asm_0_2(uint256) := TMP_4604(bytes)
 mstore(uint256,uint256)(_ofs_loadKeysSigs_asm_0,sload(uint256)(curOffset + 2))
TMP_4605(uint256) = curOffset_1 + 2
TMP_4606(uint256) = SOLIDITY_CALL sload(uint256)(TMP_4605)
TMP_4607(None) = SOLIDITY_CALL mstore(uint256,uint256)(_ofs_loadKeysSigs_asm_0_2,TMP_4606)
 mstore(uint256,uint256)(_ofs_loadKeysSigs_asm_0 + 0x20,sload(uint256)(curOffset + 3))
TMP_4608(uint256) = _ofs_loadKeysSigs_asm_0_2 + 32
TMP_4609(uint256) = curOffset_1 + 3
TMP_4610(uint256) = SOLIDITY_CALL sload(uint256)(TMP_4609)
TMP_4611(None) = SOLIDITY_CALL mstore(uint256,uint256)(TMP_4608,TMP_4610)
 mstore(uint256,uint256)(_ofs_loadKeysSigs_asm_0 + 0x40,sload(uint256)(curOffset + 4))
TMP_4612(uint256) = _ofs_loadKeysSigs_asm_0_2 + 64
TMP_4613(uint256) = curOffset_1 + 4
TMP_4614(uint256) = SOLIDITY_CALL sload(uint256)(TMP_4613)
TMP_4615(None) = SOLIDITY_CALL mstore(uint256,uint256)(TMP_4612,TMP_4614)
 i = i + 1
TMP_4616(uint256) = i_1 + 1
i_2(uint256) := TMP_4616(uint256)
```
#### SigningKeys.initKeysSigsBuf(uint256) [INTERNAL]
```slithir
PUBKEY_LENGTH_3(uint64) := phi(['PUBKEY_LENGTH_0'])
SIGNATURE_LENGTH_2(uint64) := phi(['SIGNATURE_LENGTH_0'])
 (new bytes(count * PUBKEY_LENGTH),new bytes(count * SIGNATURE_LENGTH))
TMP_4635(uint256) = count_1 (c)* PUBKEY_LENGTH_3
TMP_4636 = new bytes(TMP_4635)
TMP_4638(uint256) = count_1 (c)* SIGNATURE_LENGTH_2
TMP_4639 = new bytes(TMP_4638)
RETURN TMP_4636,TMP_4639
```
#### ICSAccounting.penalize(uint256,uint256) [EXTERNAL]
```slithir

```

#### ICSAccounting.chargeFee(uint256,uint256) [EXTERNAL]
```slithir

```
#### SigningKeys.removeKeysSigs(uint256,uint256,uint256,uint256) [INTERNAL]
```slithir
SIGNING_KEYS_POSITION_2(bytes32) := phi(['SIGNING_KEYS_POSITION_0'])
 keysCount == 0 || startIndex + keysCount > totalKeysCount || totalKeysCount > type()(uint32).max
TMP_4548(bool) = keysCount_1 == 0
TMP_4549(uint256) = startIndex_1 (c)+ keysCount_1
TMP_4550(bool) = TMP_4549 > totalKeysCount_1
TMP_4551(bool) = TMP_4548 || TMP_4550
TMP_4553(uint32) := 4294967295(uint32)
TMP_4554(bool) = totalKeysCount_1 > TMP_4553
TMP_4555(bool) = TMP_4551 || TMP_4554
CONDITION TMP_4555
 revert InvalidKeysCount()()
TMP_4556(None) = SOLIDITY_CALL revert InvalidKeysCount()()
 tmpKey = new bytes(48)
TMP_4558 = new bytes(48)
tmpKey_1(bytes) := TMP_4558(bytes)
 i = startIndex + keysCount
TMP_4559(uint256) = startIndex_1 + keysCount_1
i_1(uint256) := TMP_4559(uint256)
 i > startIndex
totalKeysCount_2(uint256) := phi(['totalKeysCount_3', 'totalKeysCount_1'])
i_2(uint256) := phi(['i_3', 'i_1'])
TMP_4560(bool) = i_2 > startIndex_1
CONDITION TMP_4560
 curOffset = SIGNING_KEYS_POSITION.getKeyOffset(nodeOperatorId,i - 1)
TMP_4561(uint256) = i_2 - 1
TMP_4562(uint256) = LIBRARY_CALL, dest:SigningKeys, function:SigningKeys.getKeyOffset(bytes32,uint256,uint256), arguments:['SIGNING_KEYS_POSITION_2', 'nodeOperatorId_1', 'TMP_4561'] 
curOffset_1(uint256) := TMP_4562(uint256)
 mstore(uint256,uint256)(tmpKey + 0x30,sload(uint256)(curOffset + 1) >> 128)
TMP_4563(bytes) = tmpKey_1 + 48
TMP_4564(uint256) = curOffset_1 + 1
TMP_4565(uint256) = SOLIDITY_CALL sload(uint256)(TMP_4564)
TMP_4566(uint256) = TMP_4565 >> 128
TMP_4567(None) = SOLIDITY_CALL mstore(uint256,uint256)(TMP_4563,TMP_4566)
 mstore(uint256,uint256)(tmpKey + 0x20,sload(uint256)(curOffset))
TMP_4568(bytes) = tmpKey_1 + 32
TMP_4569(uint256) = SOLIDITY_CALL sload(uint256)(curOffset_1)
TMP_4570(None) = SOLIDITY_CALL mstore(uint256,uint256)(TMP_4568,TMP_4569)
 i < totalKeysCount
TMP_4571(bool) = i_2 < totalKeysCount_2
CONDITION TMP_4571
 lastOffset = SIGNING_KEYS_POSITION.getKeyOffset(nodeOperatorId,totalKeysCount - 1)
TMP_4572(uint256) = totalKeysCount_2 - 1
TMP_4573(uint256) = LIBRARY_CALL, dest:SigningKeys, function:SigningKeys.getKeyOffset(bytes32,uint256,uint256), arguments:['SIGNING_KEYS_POSITION_2', 'nodeOperatorId_1', 'TMP_4572'] 
lastOffset_1(uint256) := TMP_4573(uint256)
 j = 0
j_1(uint256) := 0(uint256)
 j < 5
j_2(uint256) := phi(['j_6', 'j_1'])
TMP_4574(bool) = j_2 < 5
CONDITION TMP_4574
 sstore(uint256,uint256)(curOffset + j,sload(uint256)(lastOffset + j))
TMP_4575(uint256) = curOffset_1 + j_2
TMP_4576(uint256) = lastOffset_1 + j_2
TMP_4577(uint256) = SOLIDITY_CALL sload(uint256)(TMP_4576)
TMP_4578(None) = SOLIDITY_CALL sstore(uint256,uint256)(TMP_4575,TMP_4577)
 j = j + 1
TMP_4579(uint256) = j_2 + 1
j_6(uint256) := TMP_4579(uint256)
 curOffset = lastOffset
curOffset_2(uint256) := lastOffset_1(uint256)
curOffset_3(uint256) := phi(['curOffset_1', 'curOffset_2'])
 j = 0
j_3(uint256) := 0(uint256)
 j < 5
j_4(uint256) := phi(['j_5', 'j_3'])
TMP_4580(bool) = j_4 < 5
CONDITION TMP_4580
 sstore(uint256,uint256)(curOffset + j,0)
TMP_4581(uint256) = curOffset_3 + j_4
TMP_4582(None) = SOLIDITY_CALL sstore(uint256,uint256)(TMP_4581,0)
 j = j + 1
TMP_4583(uint256) = j_4 + 1
j_5(uint256) := TMP_4583(uint256)
 totalKeysCount = totalKeysCount - 1
TMP_4584(uint256) = totalKeysCount_2 - 1
totalKeysCount_3(uint256) := TMP_4584(uint256)
 i = i - 1
TMP_4585(uint256) = i_2 - 1
i_3(uint256) := TMP_4585(uint256)
 IStakingModule.SigningKeyRemoved(nodeOperatorId,tmpKey)
Emit SigningKeyRemoved(nodeOperatorId_1,tmpKey_1)
 totalKeysCount
RETURN totalKeysCount_2
```
#### ICSParametersRegistry.getKeyRemovalCharge(uint256) [EXTERNAL]
```slithir

```
#### ICSExitPenalties.processExitDelayReport(uint256,bytes,uint256) [EXTERNAL]
```slithir

```
#### ICSExitPenalties.processTriggeredExit(uint256,bytes,uint256,uint256) [EXTERNAL]
```slithir

```
#### ICSExitPenalties.isValidatorExitDelayPenaltyApplicable(uint256,bytes,uint256) [EXTERNAL]
```slithir

```
#### ICSParametersRegistry.getAllowedExitDelay(uint256) [EXTERNAL]
```slithir

```
#### ICSAccounting.getUnbondedKeysCountToEject(uint256) [EXTERNAL]
```slithir

```
#### IStETH.transferShares(address,uint256) [EXTERNAL]
```slithir

```

#### ValidatorCountsReport.safeCountOperators(bytes,bytes) [INTERNAL]
```slithir
 counts.length / 16 != ids.length / 8 || ids.length % 8 != 0 || counts.length % 16 != 0
REF_1991 -> LENGTH counts_1
TMP_4664(uint256) = REF_1991 (c)/ 16
REF_1992 -> LENGTH ids_1
TMP_4665(uint256) = REF_1992 (c)/ 8
TMP_4666(bool) = TMP_4664 != TMP_4665
REF_1993 -> LENGTH ids_1
TMP_4667(uint256) = REF_1993 % 8
TMP_4668(bool) = TMP_4667 != 0
TMP_4669(bool) = TMP_4666 || TMP_4668
REF_1994 -> LENGTH counts_1
TMP_4670(uint256) = REF_1994 % 16
TMP_4671(bool) = TMP_4670 != 0
TMP_4672(bool) = TMP_4669 || TMP_4671
CONDITION TMP_4672
 revert InvalidReportData()()
TMP_4673(None) = SOLIDITY_CALL revert InvalidReportData()()
 ids.length / 8
REF_1995 -> LENGTH ids_1
TMP_4674(uint256) = REF_1995 (c)/ 8
RETURN TMP_4674
```
#### QueueLib.dequeue(QueueLib.Queue) [INTERNAL]
```slithir
 item = peek(self)
TMP_4366(Batch) = INTERNAL_CALL, QueueLib.peek(QueueLib.Queue)(self_1 (-> []))
item_1(Batch) := TMP_4366(Batch)
 item.isNil()
TMP_4367(bool) = INTERNAL_CALL, isNil(Batch)(item_1)
CONDITION TMP_4367
 IQueueLib.QueueIsEmpty()
TMP_4368(None) = SOLIDITY_CALL revert QueueIsEmpty()()
 self.head = item.next()
REF_1953(uint128) -> self_1 (-> []).head
TMP_4369(uint128) = INTERNAL_CALL, next(Batch)(item_1)
self_2 (-> [])(QueueLib.Queue) := phi(['self_1 (-> [])'])
REF_1953(uint128) (->self_2 (-> [])) := TMP_4369(uint128)
 item
RETURN item_1
```
#### QueueLib.peek(QueueLib.Queue) [INTERNAL]
```slithir
self_1 (-> [])(QueueLib.Queue) := phi(['self_1 (-> [])'])
 self.queue[self.head]
REF_1955(mapping(uint128 => Batch)) -> self_1 (-> []).queue
REF_1956(uint128) -> self_1 (-> []).head
REF_1957(Batch) -> REF_1955[REF_1956]
RETURN REF_1957
```
#### ILidoLocator.lido() [EXTERNAL]
```slithir

```
#### ICSParametersRegistry.QUEUE_LOWEST_PRIORITY() [EXTERNAL]
```slithir

```
#### ICSParametersRegistry.QUEUE_LEGACY_PRIORITY() [EXTERNAL]
```slithir

```
#### ICSAccounting.feeDistributor() [EXTERNAL]
```slithir

```
#### ILidoLocator.stakingRouter() [EXTERNAL]
```slithir

```
#### SigningKeys.saveKeysSigs(uint256,uint256,uint256,bytes,bytes) [INTERNAL]
```slithir
SIGNING_KEYS_POSITION_1(bytes32) := phi(['SIGNING_KEYS_POSITION_0'])
PUBKEY_LENGTH_1(uint64) := phi(['PUBKEY_LENGTH_0'])
SIGNATURE_LENGTH_1(uint64) := phi(['SIGNATURE_LENGTH_0'])
 keysCount == 0 || startIndex + keysCount > type()(uint32).max
TMP_4495(bool) = keysCount_1 == 0
TMP_4496(uint256) = startIndex_1 (c)+ keysCount_1
TMP_4498(uint32) := 4294967295(uint32)
TMP_4499(bool) = TMP_4496 > TMP_4498
TMP_4500(bool) = TMP_4495 || TMP_4499
CONDITION TMP_4500
 revert InvalidKeysCount()()
TMP_4501(None) = SOLIDITY_CALL revert InvalidKeysCount()()
 pubkeys.length != keysCount * PUBKEY_LENGTH || signatures.length != keysCount * SIGNATURE_LENGTH
REF_1981 -> LENGTH pubkeys_1
TMP_4502(uint256) = keysCount_1 * PUBKEY_LENGTH_1
TMP_4503(bool) = REF_1981 != TMP_4502
REF_1982 -> LENGTH signatures_1
TMP_4504(uint256) = keysCount_1 * SIGNATURE_LENGTH_1
TMP_4505(bool) = REF_1982 != TMP_4504
TMP_4506(bool) = TMP_4503 || TMP_4505
CONDITION TMP_4506
 revert InvalidLength()()
TMP_4507(None) = SOLIDITY_CALL revert InvalidLength()()
 tmpKey = new bytes(48)
TMP_4509 = new bytes(48)
tmpKey_1(bytes) := TMP_4509(bytes)
 i < keysCount
startIndex_2(uint256) := phi(['startIndex_1', 'startIndex_3'])
i_1(uint256) := phi(['i_2', 'i_0'])
TMP_4510(bool) = i_1 < keysCount_1
CONDITION TMP_4510
 curOffset = SIGNING_KEYS_POSITION.getKeyOffset(nodeOperatorId,startIndex)
TMP_4511(uint256) = LIBRARY_CALL, dest:SigningKeys, function:SigningKeys.getKeyOffset(bytes32,uint256,uint256), arguments:['SIGNING_KEYS_POSITION_1', 'nodeOperatorId_1', 'startIndex_2'] 
curOffset_1(uint256) := TMP_4511(uint256)
 _ofs_saveKeysSigs_asm_0 = pubkeys + i * 48
TMP_4512(uint256) = i_1 * 48
TMP_4513(bytes) = pubkeys_1 + TMP_4512
_ofs_saveKeysSigs_asm_0_1(uint256) := TMP_4513(bytes)
 _part1_saveKeysSigs_asm_0 = calldataload(uint256)(_ofs_saveKeysSigs_asm_0)
TMP_4514(uint256) = SOLIDITY_CALL calldataload(uint256)(_ofs_saveKeysSigs_asm_0_1)
_part1_saveKeysSigs_asm_0_1(uint256) := TMP_4514(uint256)
 _part2_saveKeysSigs_asm_0 = calldataload(uint256)(_ofs_saveKeysSigs_asm_0 + 0x10)
TMP_4515(uint256) = _ofs_saveKeysSigs_asm_0_1 + 16
TMP_4516(uint256) = SOLIDITY_CALL calldataload(uint256)(TMP_4515)
_part2_saveKeysSigs_asm_0_1(uint256) := TMP_4516(uint256)
 isEmpty = ! _part1_saveKeysSigs_asm_0 | _part2_saveKeysSigs_asm_0
TMP_4517(uint256) = _part1_saveKeysSigs_asm_0_1 | _part2_saveKeysSigs_asm_0_1
TMP_4518 = UnaryType.BANG TMP_4517 
isEmpty_1(bool) := TMP_4518(uint256)
 mstore(uint256,uint256)(tmpKey + 0x30,_part2_saveKeysSigs_asm_0)
TMP_4519(bytes) = tmpKey_1 + 48
TMP_4520(None) = SOLIDITY_CALL mstore(uint256,uint256)(TMP_4519,_part2_saveKeysSigs_asm_0_1)
 mstore(uint256,uint256)(tmpKey + 0x20,_part1_saveKeysSigs_asm_0)
TMP_4521(bytes) = tmpKey_1 + 32
TMP_4522(None) = SOLIDITY_CALL mstore(uint256,uint256)(TMP_4521,_part1_saveKeysSigs_asm_0_1)
 isEmpty
CONDITION isEmpty_1
 revert EmptyKey()()
TMP_4523(None) = SOLIDITY_CALL revert EmptyKey()()
 sstore(uint256,uint256)(curOffset,mload(uint256)(tmpKey + 0x20))
TMP_4524(bytes) = tmpKey_1 + 32
TMP_4525(uint256) = SOLIDITY_CALL mload(uint256)(TMP_4524)
TMP_4526(None) = SOLIDITY_CALL sstore(uint256,uint256)(curOffset_1,TMP_4525)
 sstore(uint256,uint256)(curOffset + 1,mload(uint256)(tmpKey + 0x30) << 128)
TMP_4527(uint256) = curOffset_1 + 1
TMP_4528(bytes) = tmpKey_1 + 48
TMP_4529(uint256) = SOLIDITY_CALL mload(uint256)(TMP_4528)
TMP_4530(uint256) = TMP_4529 << 128
TMP_4531(None) = SOLIDITY_CALL sstore(uint256,uint256)(TMP_4527,TMP_4530)
 _ofs_saveKeysSigs_asm_1 = signatures + i * 96
TMP_4532(uint256) = i_1 * 96
TMP_4533(bytes) = signatures_1 + TMP_4532
_ofs_saveKeysSigs_asm_1_1(uint256) := TMP_4533(bytes)
 sstore(uint256,uint256)(curOffset + 2,calldataload(uint256)(_ofs_saveKeysSigs_asm_1))
TMP_4534(uint256) = curOffset_1 + 2
TMP_4535(uint256) = SOLIDITY_CALL calldataload(uint256)(_ofs_saveKeysSigs_asm_1_1)
TMP_4536(None) = SOLIDITY_CALL sstore(uint256,uint256)(TMP_4534,TMP_4535)
 sstore(uint256,uint256)(curOffset + 3,calldataload(uint256)(_ofs_saveKeysSigs_asm_1 + 0x20))
TMP_4537(uint256) = curOffset_1 + 3
TMP_4538(uint256) = _ofs_saveKeysSigs_asm_1_1 + 32
TMP_4539(uint256) = SOLIDITY_CALL calldataload(uint256)(TMP_4538)
TMP_4540(None) = SOLIDITY_CALL sstore(uint256,uint256)(TMP_4537,TMP_4539)
 sstore(uint256,uint256)(curOffset + 4,calldataload(uint256)(_ofs_saveKeysSigs_asm_1 + 0x40))
TMP_4541(uint256) = curOffset_1 + 4
TMP_4542(uint256) = _ofs_saveKeysSigs_asm_1_1 + 64
TMP_4543(uint256) = SOLIDITY_CALL calldataload(uint256)(TMP_4542)
TMP_4544(None) = SOLIDITY_CALL sstore(uint256,uint256)(TMP_4541,TMP_4543)
 i = i + 1
TMP_4545(uint256) = i_1 + 1
i_2(uint256) := TMP_4545(uint256)
 startIndex = startIndex + 1
TMP_4546(uint256) = startIndex_2 + 1
startIndex_3(uint256) := TMP_4546(uint256)
 IStakingModule.SigningKeyAdded(nodeOperatorId,tmpKey)
Emit SigningKeyAdded(nodeOperatorId_1,tmpKey_1)
 startIndex
RETURN startIndex_2
```
#### ICSParametersRegistry.getKeysLimit(uint256) [EXTERNAL]
```slithir

```
#### ICSAccounting.getUnbondedKeysCount(uint256) [EXTERNAL]
```slithir

```
#### QueueLib.enqueue(QueueLib.Queue,uint256,uint256) [INTERNAL]
```slithir
 tail = self.tail
REF_1948(uint128) -> self_1 (-> []).tail
tail_1(uint128) := REF_1948(uint128)
 item = createBatch(nodeOperatorId,keysCount)
TMP_4362(Batch) = INTERNAL_CALL, createBatch(uint256,uint256)(nodeOperatorId_1,keysCount_1)
item_1(Batch) := TMP_4362(Batch)
 item = item & 0xffffffffffffffffffffffffffffffff00000000000000000000000000000000 | tail + 1
TMP_4363(Batch) = item_1 & 115792089237316195423570985008687907852929702298719625575994209400481361428480
TMP_4364(uint128) = tail_1 + 1
TMP_4365(Batch) = TMP_4363 | TMP_4364
item_2(Batch) := TMP_4365(Batch)
 self.queue[tail] = item
REF_1949(mapping(uint128 => Batch)) -> self_1 (-> []).queue
REF_1950(Batch) -> REF_1949[tail_1]
self_2 (-> [])(QueueLib.Queue) := phi(['self_1 (-> [])'])
REF_1950(Batch) (->self_2 (-> [])) := item_2(Batch)
 ++ self.tail
REF_1951(uint128) -> self_2 (-> []).tail
self_3 (-> [])(QueueLib.Queue) := phi(['self_2 (-> [])'])
REF_1951(-> self_3 (-> [])) = REF_1951 + 1
 item
RETURN item_2
```
#### TransientUintUintMapLib.load(bytes32) [INTERNAL]
```slithir
 self = tslot
self_1(TransientUintUintMap) := tslot_1(bytes32)
 self
RETURN self_1
```
#### TransientUintUintMapLib.set(TransientUintUintMap,uint256,uint256) [INTERNAL]
```slithir
 slot = _slot(self,key)
TMP_4653(uint256) = INTERNAL_CALL, TransientUintUintMapLib._slot(TransientUintUintMap,uint256)(self_1,key_1)
slot_1(uint256) := TMP_4653(uint256)
 tstore(uint256,uint256)(slot,value)
TMP_4654(None) = SOLIDITY_CALL tstore(uint256,uint256)(slot_1,value_1)
```
#### TransientUintUintMapLib.get(TransientUintUintMap,uint256) [INTERNAL]
```slithir
 slot = _slot(self,key)
TMP_4655(uint256) = INTERNAL_CALL, TransientUintUintMapLib._slot(TransientUintUintMap,uint256)(self_1,key_1)
slot_1(uint256) := TMP_4655(uint256)
 v = tload(uint256)(slot)
TMP_4656(uint256) = SOLIDITY_CALL tload(uint256)(slot_1)
v_1(uint256) := TMP_4656(uint256)
 v
RETURN v_1
```
#### TransientUintUintMapLib.add(TransientUintUintMap,uint256,uint256) [INTERNAL]
```slithir
 slot = _slot(self,key)
TMP_4649(uint256) = INTERNAL_CALL, TransientUintUintMapLib._slot(TransientUintUintMap,uint256)(self_1,key_1)
slot_1(uint256) := TMP_4649(uint256)
 v_add_asm_0 = tload(uint256)(slot)
TMP_4650(uint256) = SOLIDITY_CALL tload(uint256)(slot_1)
v_add_asm_0_1(uint256) := TMP_4650(uint256)
 v_add_asm_0 = v_add_asm_0 + value
TMP_4651(uint256) = v_add_asm_0_1 + value_1
v_add_asm_0_2(uint256) := TMP_4651(uint256)
 tstore(uint256,uint256)(slot,v_add_asm_0)
TMP_4652(None) = SOLIDITY_CALL tstore(uint256,uint256)(slot_1,v_add_asm_0_2)
```
#### SigningKeys.getKeyOffset(bytes32,uint256,uint256) [INTERNAL]
```slithir
 uint256(keccak256(bytes)(abi.encodePacked(position,nodeOperatorId,keyIndex)))
TMP_4640(bytes) = SOLIDITY_CALL abi.encodePacked()(position_1,nodeOperatorId_1,keyIndex_1)
TMP_4641(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_4640)
TMP_4642 = CONVERT TMP_4641 to uint256
RETURN TMP_4642
```

