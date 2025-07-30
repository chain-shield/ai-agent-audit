



#### CSEjector._onlyRecoverer() [INTERNAL]
```slithir
RECOVERER_ROLE_1(bytes32) := phi(['RECOVERER_ROLE_2', 'RECOVERER_ROLE_0'])
 _checkRole(RECOVERER_ROLE)
INTERNAL_CALL, AccessControl._checkRole(bytes32)(RECOVERER_ROLE_1)
```
#### CSEjector.pauseFor(uint256) [EXTERNAL]
```slithir
PAUSE_ROLE_1(bytes32) := phi(['PAUSE_ROLE_0', 'PAUSE_ROLE_2'])
 _pauseFor(duration)
INTERNAL_CALL, PausableUntil._pauseFor(uint256)(duration_1)
 onlyRole(PAUSE_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(PAUSE_ROLE_1)
```
#### CSEjector.resume() [EXTERNAL]
```slithir
RESUME_ROLE_1(bytes32) := phi(['RESUME_ROLE_0', 'RESUME_ROLE_2'])
 _resume()
INTERNAL_CALL, PausableUntil._resume()()
 onlyRole(RESUME_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(RESUME_ROLE_1)
```
#### CSEjector.voluntaryEject(uint256,uint256,uint256,address) [EXTERNAL]
```slithir
VOLUNTARY_EXIT_TYPE_ID_1(uint8) := phi(['VOLUNTARY_EXIT_TYPE_ID_20', 'VOLUNTARY_EXIT_TYPE_ID_7', 'VOLUNTARY_EXIT_TYPE_ID_18', 'VOLUNTARY_EXIT_TYPE_ID_0', 'VOLUNTARY_EXIT_TYPE_ID_9'])
STAKING_MODULE_ID_2(uint256) := phi(['STAKING_MODULE_ID_11', 'STAKING_MODULE_ID_0', 'STAKING_MODULE_ID_19', 'STAKING_MODULE_ID_1', 'STAKING_MODULE_ID_6'])
MODULE_2(ICSModule) := phi(['MODULE_22', 'MODULE_1', 'MODULE_11', 'MODULE_0', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
 _onlyNodeOperatorOwner(nodeOperatorId)
INTERNAL_CALL, CSEjector._onlyNodeOperatorOwner(uint256)(nodeOperatorId_1)
MODULE_4(ICSModule) := phi(['MODULE_24'])
 maxKeyIndex = startFrom + keysCount
TMP_1732(uint256) = startFrom_1 (c)+ keysCount_1
maxKeyIndex_1(uint256) := TMP_1732(uint256)
 maxKeyIndex > MODULE.getNodeOperatorTotalDepositedKeys(nodeOperatorId)
TMP_1733(uint256) = HIGH_LEVEL_CALL, dest:MODULE_4(ICSModule), function:getNodeOperatorTotalDepositedKeys, arguments:['nodeOperatorId_1']  
VOLUNTARY_EXIT_TYPE_ID_4(uint8) := phi(['VOLUNTARY_EXIT_TYPE_ID_20', 'VOLUNTARY_EXIT_TYPE_ID_7', 'VOLUNTARY_EXIT_TYPE_ID_3', 'VOLUNTARY_EXIT_TYPE_ID_18', 'VOLUNTARY_EXIT_TYPE_ID_9'])
STAKING_MODULE_ID_5(uint256) := phi(['STAKING_MODULE_ID_11', 'STAKING_MODULE_ID_19', 'STAKING_MODULE_ID_1', 'STAKING_MODULE_ID_4', 'STAKING_MODULE_ID_6'])
MODULE_5(ICSModule) := phi(['MODULE_22', 'MODULE_1', 'MODULE_11', 'MODULE_4', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
TMP_1734(bool) = maxKeyIndex_1 > TMP_1733
CONDITION TMP_1734
 revert SigningKeysInvalidOffset()()
TMP_1735(None) = SOLIDITY_CALL revert SigningKeysInvalidOffset()()
 i = startFrom
i_1(uint256) := startFrom_1(uint256)
 i < maxKeyIndex
i_2(uint256) := phi(['i_3', 'i_1'])
TMP_1736(bool) = i_2 < maxKeyIndex_1
CONDITION TMP_1736
 MODULE.isValidatorWithdrawn(nodeOperatorId,i)
TMP_1737(bool) = HIGH_LEVEL_CALL, dest:MODULE_5(ICSModule), function:isValidatorWithdrawn, arguments:['nodeOperatorId_1', 'i_2']  
VOLUNTARY_EXIT_TYPE_ID_10(uint8) := phi(['VOLUNTARY_EXIT_TYPE_ID_7', 'VOLUNTARY_EXIT_TYPE_ID_20', 'VOLUNTARY_EXIT_TYPE_ID_9', 'VOLUNTARY_EXIT_TYPE_ID_18'])
STAKING_MODULE_ID_7(uint256) := phi(['STAKING_MODULE_ID_6', 'STAKING_MODULE_ID_19', 'STAKING_MODULE_ID_1', 'STAKING_MODULE_ID_11'])
MODULE_7(ICSModule) := phi(['MODULE_22', 'MODULE_1', 'MODULE_11', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
CONDITION TMP_1737
 revert AlreadyWithdrawn()()
TMP_1738(None) = SOLIDITY_CALL revert AlreadyWithdrawn()()
 ++ i
i_3(uint256) = i_2 (c)+ 1
 pubkeys = MODULE.getSigningKeys(nodeOperatorId,startFrom,keysCount)
TMP_1739(bytes) = HIGH_LEVEL_CALL, dest:MODULE_5(ICSModule), function:getSigningKeys, arguments:['nodeOperatorId_1', 'startFrom_1', 'keysCount_1']  
VOLUNTARY_EXIT_TYPE_ID_5(uint8) := phi(['VOLUNTARY_EXIT_TYPE_ID_20', 'VOLUNTARY_EXIT_TYPE_ID_7', 'VOLUNTARY_EXIT_TYPE_ID_18', 'VOLUNTARY_EXIT_TYPE_ID_4', 'VOLUNTARY_EXIT_TYPE_ID_9'])
STAKING_MODULE_ID_6(uint256) := phi(['STAKING_MODULE_ID_11', 'STAKING_MODULE_ID_19', 'STAKING_MODULE_ID_1', 'STAKING_MODULE_ID_6', 'STAKING_MODULE_ID_5'])
MODULE_6(ICSModule) := phi(['MODULE_5', 'MODULE_22', 'MODULE_1', 'MODULE_11', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
pubkeys_1(bytes) := TMP_1739(bytes)
 exitsData = new ValidatorData[](keysCount)
TMP_1741(ValidatorData[])  = new ValidatorData[](keysCount_1)
exitsData_1(ValidatorData[]) = ['TMP_1741(ValidatorData[])']
 i_scope_0 < keysCount
exitsData_2(ValidatorData[]) := phi(['exitsData_1', 'exitsData_3'])
i_scope_0_1(uint256) := phi(['i_scope_0_0', 'i_scope_0_2'])
TMP_1742(bool) = i_scope_0_1 < keysCount_1
CONDITION TMP_1742
 pubkey = new bytes(SigningKeys.PUBKEY_LENGTH)
REF_636(uint64) -> SigningKeys.PUBKEY_LENGTH
TMP_1744 = new bytes(REF_636)
pubkey_1(bytes) := TMP_1744(bytes)
 keyLen_voluntaryEject_asm_0 = mload(uint256)(pubkey)
TMP_1745(uint256) = SOLIDITY_CALL mload(uint256)(pubkey_1)
keyLen_voluntaryEject_asm_0_1(uint256) := TMP_1745(uint256)
 offset_voluntaryEject_asm_0 = keyLen_voluntaryEject_asm_0 * i
TMP_1746(uint256) = keyLen_voluntaryEject_asm_0_1 * i_2
offset_voluntaryEject_asm_0_1(uint256) := TMP_1746(uint256)
 keyPos_voluntaryEject_asm_0 = pubkeys + 0x20 + offset_voluntaryEject_asm_0
TMP_1747(bytes) = pubkeys_1 + 32
TMP_1748(bytes) = TMP_1747 + offset_voluntaryEject_asm_0_1
keyPos_voluntaryEject_asm_0_1(uint256) := TMP_1748(bytes)
 mcopy(uint256,uint256,uint256)(pubkey + 0x20,keyPos_voluntaryEject_asm_0,keyLen_voluntaryEject_asm_0)
TMP_1749(bytes) = pubkey_1 + 32
TMP_1750(None) = SOLIDITY_CALL mcopy(uint256,uint256,uint256)(TMP_1749,keyPos_voluntaryEject_asm_0_1,keyLen_voluntaryEject_asm_0_1)
 exitsData[i_scope_0] = ValidatorData({stakingModuleId:STAKING_MODULE_ID,nodeOperatorId:nodeOperatorId,pubkey:pubkey})
REF_637(ValidatorData) -> exitsData_2[i_scope_0_1]
TMP_1751(ValidatorData) = new ValidatorData(STAKING_MODULE_ID_6,nodeOperatorId_1,pubkey_1)
exitsData_3(ValidatorData[]) := phi(['exitsData_2'])
REF_637(ValidatorData) (->exitsData_3) := TMP_1751(ValidatorData)
 ++ i_scope_0
i_scope_0_2(uint256) = i_scope_0_1 (c)+ 1
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
 refundRecipient == address(0)
TMP_1753 = CONVERT 0 to address
TMP_1754(bool) = refundRecipient_1 == TMP_1753
CONDITION TMP_1754
 triggerableWithdrawalsGateway().triggerFullWithdrawals{value: msg.value}(exitsData,msg.sender,VOLUNTARY_EXIT_TYPE_ID)
TMP_1755(ITriggerableWithdrawalsGateway) = INTERNAL_CALL, CSEjector.triggerableWithdrawalsGateway()()
HIGH_LEVEL_CALL, dest:TMP_1755(ITriggerableWithdrawalsGateway), function:triggerFullWithdrawals, arguments:['exitsData_2', 'msg.sender', 'VOLUNTARY_EXIT_TYPE_ID_6'] value:msg.value 
VOLUNTARY_EXIT_TYPE_ID_7(uint8) := phi(['VOLUNTARY_EXIT_TYPE_ID_20', 'VOLUNTARY_EXIT_TYPE_ID_7', 'VOLUNTARY_EXIT_TYPE_ID_18', 'VOLUNTARY_EXIT_TYPE_ID_9', 'VOLUNTARY_EXIT_TYPE_ID_6'])
 triggerableWithdrawalsGateway().triggerFullWithdrawals{value: msg.value}(exitsData,refundRecipient,VOLUNTARY_EXIT_TYPE_ID)
TMP_1757(ITriggerableWithdrawalsGateway) = INTERNAL_CALL, CSEjector.triggerableWithdrawalsGateway()()
HIGH_LEVEL_CALL, dest:TMP_1757(ITriggerableWithdrawalsGateway), function:triggerFullWithdrawals, arguments:['exitsData_2', 'refundRecipient_1', 'VOLUNTARY_EXIT_TYPE_ID_8'] value:msg.value 
VOLUNTARY_EXIT_TYPE_ID_9(uint8) := phi(['VOLUNTARY_EXIT_TYPE_ID_20', 'VOLUNTARY_EXIT_TYPE_ID_7', 'VOLUNTARY_EXIT_TYPE_ID_18', 'VOLUNTARY_EXIT_TYPE_ID_8', 'VOLUNTARY_EXIT_TYPE_ID_9'])
```
#### CSEjector.voluntaryEjectByArray(uint256,uint256[],address) [EXTERNAL]
```slithir
VOLUNTARY_EXIT_TYPE_ID_11(uint8) := phi(['VOLUNTARY_EXIT_TYPE_ID_20', 'VOLUNTARY_EXIT_TYPE_ID_7', 'VOLUNTARY_EXIT_TYPE_ID_18', 'VOLUNTARY_EXIT_TYPE_ID_0', 'VOLUNTARY_EXIT_TYPE_ID_9'])
STAKING_MODULE_ID_8(uint256) := phi(['STAKING_MODULE_ID_11', 'STAKING_MODULE_ID_0', 'STAKING_MODULE_ID_19', 'STAKING_MODULE_ID_1', 'STAKING_MODULE_ID_6'])
MODULE_8(ICSModule) := phi(['MODULE_22', 'MODULE_1', 'MODULE_11', 'MODULE_0', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
 _onlyNodeOperatorOwner(nodeOperatorId)
INTERNAL_CALL, CSEjector._onlyNodeOperatorOwner(uint256)(nodeOperatorId_1)
MODULE_10(ICSModule) := phi(['MODULE_24'])
 totalDepositedKeys = MODULE.getNodeOperatorTotalDepositedKeys(nodeOperatorId)
TMP_1760(uint256) = HIGH_LEVEL_CALL, dest:MODULE_10(ICSModule), function:getNodeOperatorTotalDepositedKeys, arguments:['nodeOperatorId_1']  
VOLUNTARY_EXIT_TYPE_ID_14(uint8) := phi(['VOLUNTARY_EXIT_TYPE_ID_13', 'VOLUNTARY_EXIT_TYPE_ID_20', 'VOLUNTARY_EXIT_TYPE_ID_7', 'VOLUNTARY_EXIT_TYPE_ID_18', 'VOLUNTARY_EXIT_TYPE_ID_9'])
STAKING_MODULE_ID_11(uint256) := phi(['STAKING_MODULE_ID_11', 'STAKING_MODULE_ID_10', 'STAKING_MODULE_ID_19', 'STAKING_MODULE_ID_1', 'STAKING_MODULE_ID_6'])
MODULE_11(ICSModule) := phi(['MODULE_22', 'MODULE_1', 'MODULE_11', 'MODULE_10', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
totalDepositedKeys_1(uint256) := TMP_1760(uint256)
 exitsData = new ValidatorData[](keyIndices.length)
REF_641 -> LENGTH keyIndices_1
TMP_1762(ValidatorData[])  = new ValidatorData[](REF_641)
exitsData_1(ValidatorData[]) = ['TMP_1762(ValidatorData[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < keyIndices.length
exitsData_2(ValidatorData[]) := phi(['exitsData_3', 'exitsData_1'])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_642 -> LENGTH keyIndices_1
TMP_1763(bool) = i_2 < REF_642
CONDITION TMP_1763
 keyIndices[i] >= totalDepositedKeys
REF_643(uint256) -> keyIndices_1[i_2]
TMP_1764(bool) = REF_643 >= totalDepositedKeys_1
CONDITION TMP_1764
 revert SigningKeysInvalidOffset()()
TMP_1765(None) = SOLIDITY_CALL revert SigningKeysInvalidOffset()()
 MODULE.isValidatorWithdrawn(nodeOperatorId,keyIndices[i])
REF_645(uint256) -> keyIndices_1[i_2]
TMP_1766(bool) = HIGH_LEVEL_CALL, dest:MODULE_11(ICSModule), function:isValidatorWithdrawn, arguments:['nodeOperatorId_1', 'REF_645']  
VOLUNTARY_EXIT_TYPE_ID_15(uint8) := phi(['VOLUNTARY_EXIT_TYPE_ID_20', 'VOLUNTARY_EXIT_TYPE_ID_7', 'VOLUNTARY_EXIT_TYPE_ID_18', 'VOLUNTARY_EXIT_TYPE_ID_9', 'VOLUNTARY_EXIT_TYPE_ID_14'])
STAKING_MODULE_ID_12(uint256) := phi(['STAKING_MODULE_ID_6', 'STAKING_MODULE_ID_19', 'STAKING_MODULE_ID_1', 'STAKING_MODULE_ID_11'])
MODULE_12(ICSModule) := phi(['MODULE_22', 'MODULE_1', 'MODULE_11', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
CONDITION TMP_1766
 revert AlreadyWithdrawn()()
TMP_1767(None) = SOLIDITY_CALL revert AlreadyWithdrawn()()
 pubkey = MODULE.getSigningKeys(nodeOperatorId,keyIndices[i],1)
REF_647(uint256) -> keyIndices_1[i_2]
TMP_1768(bytes) = HIGH_LEVEL_CALL, dest:MODULE_12(ICSModule), function:getSigningKeys, arguments:['nodeOperatorId_1', 'REF_647', '1']  
VOLUNTARY_EXIT_TYPE_ID_16(uint8) := phi(['VOLUNTARY_EXIT_TYPE_ID_15', 'VOLUNTARY_EXIT_TYPE_ID_20', 'VOLUNTARY_EXIT_TYPE_ID_7', 'VOLUNTARY_EXIT_TYPE_ID_18', 'VOLUNTARY_EXIT_TYPE_ID_9'])
STAKING_MODULE_ID_13(uint256) := phi(['STAKING_MODULE_ID_11', 'STAKING_MODULE_ID_12', 'STAKING_MODULE_ID_19', 'STAKING_MODULE_ID_1', 'STAKING_MODULE_ID_6'])
MODULE_13(ICSModule) := phi(['MODULE_22', 'MODULE_1', 'MODULE_11', 'MODULE_12', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
pubkey_1(bytes) := TMP_1768(bytes)
 exitsData[i] = ValidatorData({stakingModuleId:STAKING_MODULE_ID,nodeOperatorId:nodeOperatorId,pubkey:pubkey})
REF_648(ValidatorData) -> exitsData_2[i_2]
TMP_1769(ValidatorData) = new ValidatorData(STAKING_MODULE_ID_13,nodeOperatorId_1,pubkey_1)
exitsData_3(ValidatorData[]) := phi(['exitsData_2'])
REF_648(ValidatorData) (->exitsData_3) := TMP_1769(ValidatorData)
 i ++
TMP_1770(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
 refundRecipient == address(0)
TMP_1772 = CONVERT 0 to address
TMP_1773(bool) = refundRecipient_1 == TMP_1772
CONDITION TMP_1773
 triggerableWithdrawalsGateway().triggerFullWithdrawals{value: msg.value}(exitsData,msg.sender,VOLUNTARY_EXIT_TYPE_ID)
TMP_1774(ITriggerableWithdrawalsGateway) = INTERNAL_CALL, CSEjector.triggerableWithdrawalsGateway()()
HIGH_LEVEL_CALL, dest:TMP_1774(ITriggerableWithdrawalsGateway), function:triggerFullWithdrawals, arguments:['exitsData_2', 'msg.sender', 'VOLUNTARY_EXIT_TYPE_ID_17'] value:msg.value 
VOLUNTARY_EXIT_TYPE_ID_18(uint8) := phi(['VOLUNTARY_EXIT_TYPE_ID_20', 'VOLUNTARY_EXIT_TYPE_ID_17', 'VOLUNTARY_EXIT_TYPE_ID_7', 'VOLUNTARY_EXIT_TYPE_ID_18', 'VOLUNTARY_EXIT_TYPE_ID_9'])
 triggerableWithdrawalsGateway().triggerFullWithdrawals{value: msg.value}(exitsData,refundRecipient,VOLUNTARY_EXIT_TYPE_ID)
TMP_1776(ITriggerableWithdrawalsGateway) = INTERNAL_CALL, CSEjector.triggerableWithdrawalsGateway()()
HIGH_LEVEL_CALL, dest:TMP_1776(ITriggerableWithdrawalsGateway), function:triggerFullWithdrawals, arguments:['exitsData_2', 'refundRecipient_1', 'VOLUNTARY_EXIT_TYPE_ID_19'] value:msg.value 
VOLUNTARY_EXIT_TYPE_ID_20(uint8) := phi(['VOLUNTARY_EXIT_TYPE_ID_20', 'VOLUNTARY_EXIT_TYPE_ID_7', 'VOLUNTARY_EXIT_TYPE_ID_18', 'VOLUNTARY_EXIT_TYPE_ID_19', 'VOLUNTARY_EXIT_TYPE_ID_9'])
```
#### CSEjector.ejectBadPerformer(uint256,uint256,address) [EXTERNAL]
```slithir
STRIKES_EXIT_TYPE_ID_1(uint8) := phi(['STRIKES_EXIT_TYPE_ID_0', 'STRIKES_EXIT_TYPE_ID_8'])
STAKING_MODULE_ID_14(uint256) := phi(['STAKING_MODULE_ID_11', 'STAKING_MODULE_ID_0', 'STAKING_MODULE_ID_19', 'STAKING_MODULE_ID_1', 'STAKING_MODULE_ID_6'])
MODULE_14(ICSModule) := phi(['MODULE_22', 'MODULE_1', 'MODULE_11', 'MODULE_0', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
 keyIndex >= MODULE.getNodeOperatorTotalDepositedKeys(nodeOperatorId)
TMP_1778(uint256) = HIGH_LEVEL_CALL, dest:MODULE_16(ICSModule), function:getNodeOperatorTotalDepositedKeys, arguments:['nodeOperatorId_1']  
STRIKES_EXIT_TYPE_ID_4(uint8) := phi(['STRIKES_EXIT_TYPE_ID_8', 'STRIKES_EXIT_TYPE_ID_3'])
STAKING_MODULE_ID_17(uint256) := phi(['STAKING_MODULE_ID_11', 'STAKING_MODULE_ID_19', 'STAKING_MODULE_ID_1', 'STAKING_MODULE_ID_6', 'STAKING_MODULE_ID_16'])
MODULE_17(ICSModule) := phi(['MODULE_22', 'MODULE_16', 'MODULE_1', 'MODULE_11', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
TMP_1779(bool) = keyIndex_1 >= TMP_1778
CONDITION TMP_1779
 revert SigningKeysInvalidOffset()()
TMP_1780(None) = SOLIDITY_CALL revert SigningKeysInvalidOffset()()
 MODULE.isValidatorWithdrawn(nodeOperatorId,keyIndex)
TMP_1781(bool) = HIGH_LEVEL_CALL, dest:MODULE_17(ICSModule), function:isValidatorWithdrawn, arguments:['nodeOperatorId_1', 'keyIndex_1']  
STRIKES_EXIT_TYPE_ID_5(uint8) := phi(['STRIKES_EXIT_TYPE_ID_8', 'STRIKES_EXIT_TYPE_ID_4'])
STAKING_MODULE_ID_18(uint256) := phi(['STAKING_MODULE_ID_11', 'STAKING_MODULE_ID_19', 'STAKING_MODULE_ID_1', 'STAKING_MODULE_ID_6', 'STAKING_MODULE_ID_17'])
MODULE_18(ICSModule) := phi(['MODULE_17', 'MODULE_22', 'MODULE_1', 'MODULE_11', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
CONDITION TMP_1781
 revert AlreadyWithdrawn()()
TMP_1782(None) = SOLIDITY_CALL revert AlreadyWithdrawn()()
 exitsData = new ValidatorData[](1)
TMP_1784(ValidatorData[])  = new ValidatorData[](1)
exitsData_1(ValidatorData[]) = ['TMP_1784(ValidatorData[])']
 pubkey = MODULE.getSigningKeys(nodeOperatorId,keyIndex,1)
TMP_1785(bytes) = HIGH_LEVEL_CALL, dest:MODULE_18(ICSModule), function:getSigningKeys, arguments:['nodeOperatorId_1', 'keyIndex_1', '1']  
STRIKES_EXIT_TYPE_ID_6(uint8) := phi(['STRIKES_EXIT_TYPE_ID_8', 'STRIKES_EXIT_TYPE_ID_5'])
STAKING_MODULE_ID_19(uint256) := phi(['STAKING_MODULE_ID_11', 'STAKING_MODULE_ID_19', 'STAKING_MODULE_ID_1', 'STAKING_MODULE_ID_18', 'STAKING_MODULE_ID_6'])
MODULE_19(ICSModule) := phi(['MODULE_22', 'MODULE_1', 'MODULE_11', 'MODULE_24', 'MODULE_19', 'MODULE_18', 'MODULE_6'])
pubkey_1(bytes) := TMP_1785(bytes)
 exitsData[0] = ValidatorData({stakingModuleId:STAKING_MODULE_ID,nodeOperatorId:nodeOperatorId,pubkey:pubkey})
REF_654(ValidatorData) -> exitsData_1[0]
TMP_1786(ValidatorData) = new ValidatorData(STAKING_MODULE_ID_19,nodeOperatorId_1,pubkey_1)
exitsData_2(ValidatorData[]) := phi(['exitsData_1'])
REF_654(ValidatorData) (->exitsData_2) := TMP_1786(ValidatorData)
 triggerableWithdrawalsGateway().triggerFullWithdrawals{value: msg.value}(exitsData,refundRecipient,STRIKES_EXIT_TYPE_ID)
TMP_1787(ITriggerableWithdrawalsGateway) = INTERNAL_CALL, CSEjector.triggerableWithdrawalsGateway()()
HIGH_LEVEL_CALL, dest:TMP_1787(ITriggerableWithdrawalsGateway), function:triggerFullWithdrawals, arguments:['exitsData_2', 'refundRecipient_1', 'STRIKES_EXIT_TYPE_ID_7'] value:msg.value 
STRIKES_EXIT_TYPE_ID_8(uint8) := phi(['STRIKES_EXIT_TYPE_ID_7', 'STRIKES_EXIT_TYPE_ID_8'])
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
 onlyStrikes()
MODIFIER_CALL, CSEjector.onlyStrikes()()
```
#### CSEjector.triggerableWithdrawalsGateway() [EXTERNAL]
```slithir
MODULE_20(ICSModule) := phi(['MODULE_22', 'MODULE_1', 'MODULE_11', 'MODULE_0', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
 ITriggerableWithdrawalsGateway(MODULE.LIDO_LOCATOR().triggerableWithdrawalsGateway())
TMP_1791(ILidoLocator) = HIGH_LEVEL_CALL, dest:MODULE_20(ICSModule), function:LIDO_LOCATOR, arguments:[]  
MODULE_21(ICSModule) := phi(['MODULE_22', 'MODULE_1', 'MODULE_11', 'MODULE_24', 'MODULE_19', 'MODULE_20', 'MODULE_6'])
TMP_1792(address) = HIGH_LEVEL_CALL, dest:TMP_1791(ILidoLocator), function:triggerableWithdrawalsGateway, arguments:[]  
MODULE_22(ICSModule) := phi(['MODULE_22', 'MODULE_21', 'MODULE_1', 'MODULE_11', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
TMP_1793 = CONVERT TMP_1792 to ITriggerableWithdrawalsGateway
RETURN TMP_1793
```
#### CSEjector.constructor(address,address,uint256,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_2'])
 module == address(0)
TMP_1716 = CONVERT 0 to address
TMP_1717(bool) = module_1 == TMP_1716
CONDITION TMP_1717
 revert ZeroModuleAddress()()
TMP_1718(None) = SOLIDITY_CALL revert ZeroModuleAddress()()
 strikes == address(0)
TMP_1719 = CONVERT 0 to address
TMP_1720(bool) = strikes_1 == TMP_1719
CONDITION TMP_1720
 revert ZeroStrikesAddress()()
TMP_1721(None) = SOLIDITY_CALL revert ZeroStrikesAddress()()
 admin == address(0)
TMP_1722 = CONVERT 0 to address
TMP_1723(bool) = admin_1 == TMP_1722
CONDITION TMP_1723
 revert ZeroAdminAddress()()
TMP_1724(None) = SOLIDITY_CALL revert ZeroAdminAddress()()
 STRIKES = strikes
STRIKES_1(address) := strikes_1(address)
 MODULE = ICSModule(module)
TMP_1725 = CONVERT module_1 to ICSModule
MODULE_1(ICSModule) := TMP_1725(ICSModule)
 STAKING_MODULE_ID = stakingModuleId
STAKING_MODULE_ID_1(uint256) := stakingModuleId_1(uint256)
 _grantRole(DEFAULT_ADMIN_ROLE,admin)
TMP_1726(bool) = INTERNAL_CALL, AccessControlEnumerable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_1,admin_1)
```
#### CSEjector._onlyNodeOperatorOwner(uint256) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1'])
MODULE_23(ICSModule) := phi(['MODULE_22', 'MODULE_1', 'MODULE_11', 'MODULE_0', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
 owner = MODULE.getNodeOperatorOwner(nodeOperatorId)
TMP_1794(address) = HIGH_LEVEL_CALL, dest:MODULE_23(ICSModule), function:getNodeOperatorOwner, arguments:['nodeOperatorId_1']  
MODULE_24(ICSModule) := phi(['MODULE_22', 'MODULE_23', 'MODULE_1', 'MODULE_11', 'MODULE_24', 'MODULE_19', 'MODULE_6'])
owner_1(address) := TMP_1794(address)
 owner == address(0)
TMP_1795 = CONVERT 0 to address
TMP_1796(bool) = owner_1 == TMP_1795
CONDITION TMP_1796
 revert NodeOperatorDoesNotExist()()
TMP_1797(None) = SOLIDITY_CALL revert NodeOperatorDoesNotExist()()
 owner != msg.sender
TMP_1798(bool) = owner_1 != msg.sender
CONDITION TMP_1798
 revert SenderIsNotEligible()()
TMP_1799(None) = SOLIDITY_CALL revert SenderIsNotEligible()()
```
#### CSAccounting.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 MIN_CURVE_LENGTH = 1
 DEFAULT_BOND_CURVE_ID = 0
 MAX_CURVE_LENGTH = 100
 RESUME_SINCE_TIMESTAMP_POSITION = keccak256(bytes)(lido.PausableUntil.resumeSinceTimestamp)
 PAUSE_INFINITELY = type()(uint256).max
 DEFAULT_ADMIN_ROLE = 0x00
 PAUSE_ROLE = keccak256(bytes)(PAUSE_ROLE)
 RESUME_ROLE = keccak256(bytes)(RESUME_ROLE)
 MANAGE_BOND_CURVES_ROLE = keccak256(bytes)(MANAGE_BOND_CURVES_ROLE)
 SET_BOND_CURVE_ROLE = keccak256(bytes)(SET_BOND_CURVE_ROLE)
 RECOVERER_ROLE = keccak256(bytes)(RECOVERER_ROLE)
role_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_7', 'TMP_1089', 'MANAGE_BOND_CURVES_ROLE_1', 'SET_BOND_CURVE_ROLE_1', 'TMP_1082', 'DEFAULT_ADMIN_ROLE_9', 'PAUSE_ROLE_1', 'TMP_1087', 'TMP_1084', 'MANAGE_BOND_CURVES_ROLE_3', 'RESUME_ROLE_1'])
 _checkRole(role)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(role_1)
 $ = _getInitializableStorage()
TMP_1611(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_1611'])(Initializable.InitializableStorage) := TMP_1611(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_590(bool) -> $_1 (-> ['TMP_1611'])._initializing
TMP_1612 = UnaryType.BANG REF_590 
isTopLevelCall_1(bool) := TMP_1612(bool)
 initialized = $._initialized
REF_591(uint64) -> $_1 (-> ['TMP_1611'])._initialized
initialized_1(uint64) := REF_591(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_1613(bool) = initialized_1 == 0
TMP_1614(bool) = TMP_1613 && isTopLevelCall_1
initialSetup_1(bool) := TMP_1614(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_1615(bool) = initialized_1 == 1
TMP_1616 = CONVERT this to address
TMP_1617(bytes) = SOLIDITY_CALL code(address)(TMP_1616)
REF_592 -> LENGTH TMP_1617
TMP_1618(bool) = REF_592 == 0
TMP_1619(bool) = TMP_1615 && TMP_1618
construction_1(bool) := TMP_1619(bool)
 ! initialSetup && ! construction
TMP_1620 = UnaryType.BANG initialSetup_1 
TMP_1621 = UnaryType.BANG construction_1 
TMP_1622(bool) = TMP_1620 && TMP_1621
CONDITION TMP_1622
 revert InvalidInitialization()()
TMP_1623(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_593(uint64) -> $_1 (-> ['TMP_1611'])._initialized
$_2 (-> ['TMP_1611'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_1611'])"])
REF_593(uint64) (->$_2 (-> ['TMP_1611'])) := 1(uint256)
TMP_1611(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_1611'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_594(bool) -> $_2 (-> ['TMP_1611'])._initializing
$_3 (-> ['TMP_1611'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_1611'])"])
REF_594(bool) (->$_3 (-> ['TMP_1611'])) := True(bool)
TMP_1611(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_1611'])"])
$_4 (-> ['TMP_1611'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_1611'])", "$_2 (-> ['TMP_1611'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_595(bool) -> $_4 (-> ['TMP_1611'])._initializing
$_5 (-> ['TMP_1611'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_1611'])"])
REF_595(bool) (->$_5 (-> ['TMP_1611'])) := False(bool)
TMP_1611(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_1611'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_1625(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_1625'])(Initializable.InitializableStorage) := TMP_1625(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_596(bool) -> $_1 (-> ['TMP_1625'])._initializing
REF_597(uint64) -> $_1 (-> ['TMP_1625'])._initialized
TMP_1626(bool) = REF_597 >= version_1
TMP_1627(bool) = REF_596 || TMP_1626
CONDITION TMP_1627
 revert InvalidInitialization()()
TMP_1628(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_598(uint64) -> $_1 (-> ['TMP_1625'])._initialized
$_2 (-> ['TMP_1625'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_1625'])"])
REF_598(uint64) (->$_2 (-> ['TMP_1625'])) := version_1(uint64)
TMP_1625(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_1625'])"])
 $._initializing = true
REF_599(bool) -> $_2 (-> ['TMP_1625'])._initializing
$_3 (-> ['TMP_1625'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_1625'])"])
REF_599(bool) (->$_3 (-> ['TMP_1625'])) := True(bool)
TMP_1625(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_1625'])"])
 $._initializing = false
REF_600(bool) -> $_3 (-> ['TMP_1625'])._initializing
$_4 (-> ['TMP_1625'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_1625'])"])
REF_600(bool) (->$_4 (-> ['TMP_1625'])) := False(bool)
TMP_1625(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_1625'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
 _checkPaused()
INTERNAL_CALL, PausableUntil._checkPaused()()
 _checkResumed()
INTERNAL_CALL, PausableUntil._checkResumed()()
MODULE_56(ICSModule) := phi(['MODULE_0', 'MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 msg.sender != address(MODULE)
TMP_1633 = CONVERT MODULE_56 to address
TMP_1634(bool) = msg.sender != TMP_1633
CONDITION TMP_1634
 revert SenderIsNotModule()()
TMP_1635(None) = SOLIDITY_CALL revert SenderIsNotModule()()
```
#### ICSModule.isValidatorWithdrawn(uint256,uint256) [EXTERNAL]
```slithir

```
#### ICSModule.getNodeOperatorTotalDepositedKeys(uint256) [EXTERNAL]
```slithir

```
#### ICSModule.getSigningKeys(uint256,uint256,uint256) [EXTERNAL]
```slithir

```

#### ICSModule.LIDO_LOCATOR() [EXTERNAL]
```slithir

```

#### ICSModule.getNodeOperatorOwner(uint256) [EXTERNAL]
```slithir

```
