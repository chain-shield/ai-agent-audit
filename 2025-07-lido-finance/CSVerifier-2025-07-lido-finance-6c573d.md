


#### CSVerifier.pauseFor(uint256) [EXTERNAL]
```slithir
PAUSE_ROLE_1(bytes32) := phi(['PAUSE_ROLE_0', 'PAUSE_ROLE_2'])
 _pauseFor(duration)
INTERNAL_CALL, PausableUntil._pauseFor(uint256)(duration_1)
 onlyRole(PAUSE_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(PAUSE_ROLE_1)
```
#### CSVerifier.resume() [EXTERNAL]
```slithir
RESUME_ROLE_1(bytes32) := phi(['RESUME_ROLE_2', 'RESUME_ROLE_0'])
 _resume()
INTERNAL_CALL, PausableUntil._resume()()
 onlyRole(RESUME_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(RESUME_ROLE_1)
```
#### CSVerifier.processWithdrawalProof(ICSVerifier.ProvableBeaconBlockHeader,ICSVerifier.WithdrawalWitness,uint256,uint256) [EXTERNAL]
```slithir
FIRST_SUPPORTED_SLOT_2(Slot) := phi(['FIRST_SUPPORTED_SLOT_0', 'FIRST_SUPPORTED_SLOT_1', 'FIRST_SUPPORTED_SLOT_4', 'FIRST_SUPPORTED_SLOT_8'])
MODULE_2(ICSModule) := phi(['MODULE_17', 'MODULE_0', 'MODULE_8', 'MODULE_1'])
 lt(beaconBlock.header.slot,FIRST_SUPPORTED_SLOT)
REF_1474(BeaconBlockHeader) -> beaconBlock_1.header
REF_1475(Slot) -> REF_1474.slot
TMP_3520(bool) = INTERNAL_CALL, lt(Slot,Slot)(REF_1475,FIRST_SUPPORTED_SLOT_3)
CONDITION TMP_3520
 revert UnsupportedSlot(Slot)(beaconBlock.header.slot)
REF_1476(BeaconBlockHeader) -> beaconBlock_1.header
REF_1477(Slot) -> REF_1476.slot
TMP_3521(None) = SOLIDITY_CALL revert UnsupportedSlot(Slot)(REF_1477)
 trustedHeaderRoot = _getParentBlockRoot(beaconBlock.rootsTimestamp)
REF_1478(uint64) -> beaconBlock_1.rootsTimestamp
TMP_3522(bytes32) = INTERNAL_CALL, CSVerifier._getParentBlockRoot(uint64)(REF_1478)
trustedHeaderRoot_1(bytes32) := TMP_3522(bytes32)
 trustedHeaderRoot != beaconBlock.header.hashTreeRoot()
REF_1479(BeaconBlockHeader) -> beaconBlock_1.header
TMP_3523(bytes32) = LIBRARY_CALL, dest:SSZ, function:SSZ.hashTreeRoot(BeaconBlockHeader), arguments:['REF_1479'] 
TMP_3524(bool) = trustedHeaderRoot_1 != TMP_3523
CONDITION TMP_3524
 revert InvalidBlockHeader()()
TMP_3525(None) = SOLIDITY_CALL revert InvalidBlockHeader()()
 pubkey = MODULE.getSigningKeys(nodeOperatorId,keyIndex,1)
TMP_3526(bytes) = HIGH_LEVEL_CALL, dest:MODULE_5(ICSModule), function:getSigningKeys, arguments:['nodeOperatorId_1', 'keyIndex_1', '1']  
MODULE_6(ICSModule) := phi(['MODULE_5', 'MODULE_17', 'MODULE_8', 'MODULE_1'])
pubkey_1(bytes) := TMP_3526(bytes)
 withdrawalAmount = _processWithdrawalProof({witness:witness,stateSlot:beaconBlock.header.slot,stateRoot:beaconBlock.header.stateRoot,pubkey:pubkey})
REF_1482(BeaconBlockHeader) -> beaconBlock_1.header
REF_1483(Slot) -> REF_1482.slot
REF_1484(BeaconBlockHeader) -> beaconBlock_1.header
REF_1485(bytes32) -> REF_1484.stateRoot
TMP_3527(uint256) = INTERNAL_CALL, CSVerifier._processWithdrawalProof(ICSVerifier.WithdrawalWitness,Slot,bytes32,bytes)(witness_1,REF_1483,REF_1485,pubkey_1)
withdrawalAmount_1(uint256) := TMP_3527(uint256)
 withdrawalsInfo = new ValidatorWithdrawalInfo[](1)
TMP_3529(ValidatorWithdrawalInfo[])  = new ValidatorWithdrawalInfo[](1)
withdrawalsInfo_1(ValidatorWithdrawalInfo[]) = ['TMP_3529(ValidatorWithdrawalInfo[])']
 withdrawalsInfo[0] = ValidatorWithdrawalInfo(nodeOperatorId,keyIndex,withdrawalAmount)
REF_1486(ValidatorWithdrawalInfo) -> withdrawalsInfo_1[0]
TMP_3530(ValidatorWithdrawalInfo) = new ValidatorWithdrawalInfo(nodeOperatorId_1,keyIndex_1,withdrawalAmount_1)
withdrawalsInfo_2(ValidatorWithdrawalInfo[]) := phi(['withdrawalsInfo_1'])
REF_1486(ValidatorWithdrawalInfo) (->withdrawalsInfo_2) := TMP_3530(ValidatorWithdrawalInfo)
 MODULE.submitWithdrawals(withdrawalsInfo)
HIGH_LEVEL_CALL, dest:MODULE_7(ICSModule), function:submitWithdrawals, arguments:['withdrawalsInfo_2']  
MODULE_8(ICSModule) := phi(['MODULE_7', 'MODULE_17', 'MODULE_8', 'MODULE_1'])
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
```
#### CSVerifier.processHistoricalWithdrawalProof(ICSVerifier.ProvableBeaconBlockHeader,ICSVerifier.HistoricalHeaderWitness,ICSVerifier.WithdrawalWitness,uint256,uint256) [EXTERNAL]
```slithir
FIRST_SUPPORTED_SLOT_5(Slot) := phi(['FIRST_SUPPORTED_SLOT_0', 'FIRST_SUPPORTED_SLOT_1', 'FIRST_SUPPORTED_SLOT_4', 'FIRST_SUPPORTED_SLOT_8'])
MODULE_9(ICSModule) := phi(['MODULE_17', 'MODULE_0', 'MODULE_8', 'MODULE_1'])
 lt(beaconBlock.header.slot,FIRST_SUPPORTED_SLOT)
REF_1488(BeaconBlockHeader) -> beaconBlock_1.header
REF_1489(Slot) -> REF_1488.slot
TMP_3533(bool) = INTERNAL_CALL, lt(Slot,Slot)(REF_1489,FIRST_SUPPORTED_SLOT_6)
CONDITION TMP_3533
 revert UnsupportedSlot(Slot)(beaconBlock.header.slot)
REF_1490(BeaconBlockHeader) -> beaconBlock_1.header
REF_1491(Slot) -> REF_1490.slot
TMP_3534(None) = SOLIDITY_CALL revert UnsupportedSlot(Slot)(REF_1491)
 lt(oldBlock.header.slot,FIRST_SUPPORTED_SLOT)
REF_1492(BeaconBlockHeader) -> oldBlock_1.header
REF_1493(Slot) -> REF_1492.slot
TMP_3535(bool) = INTERNAL_CALL, lt(Slot,Slot)(REF_1493,FIRST_SUPPORTED_SLOT_7)
CONDITION TMP_3535
 revert UnsupportedSlot(Slot)(oldBlock.header.slot)
REF_1494(BeaconBlockHeader) -> oldBlock_1.header
REF_1495(Slot) -> REF_1494.slot
TMP_3536(None) = SOLIDITY_CALL revert UnsupportedSlot(Slot)(REF_1495)
 trustedHeaderRoot = _getParentBlockRoot(beaconBlock.rootsTimestamp)
REF_1496(uint64) -> beaconBlock_1.rootsTimestamp
TMP_3537(bytes32) = INTERNAL_CALL, CSVerifier._getParentBlockRoot(uint64)(REF_1496)
trustedHeaderRoot_1(bytes32) := TMP_3537(bytes32)
 headerRoot = beaconBlock.header.hashTreeRoot()
REF_1497(BeaconBlockHeader) -> beaconBlock_1.header
TMP_3538(bytes32) = LIBRARY_CALL, dest:SSZ, function:SSZ.hashTreeRoot(BeaconBlockHeader), arguments:['REF_1497'] 
headerRoot_1(bytes32) := TMP_3538(bytes32)
 trustedHeaderRoot != headerRoot
TMP_3539(bool) = trustedHeaderRoot_1 != headerRoot_1
CONDITION TMP_3539
 revert InvalidBlockHeader()()
TMP_3540(None) = SOLIDITY_CALL revert InvalidBlockHeader()()
 SSZ.verifyProof({proof:oldBlock.proof,root:beaconBlock.header.stateRoot,leaf:oldBlock.header.hashTreeRoot(),gI:_getHistoricalBlockRootGI(beaconBlock.header.slot,oldBlock.header.slot)})
REF_1500(bytes32[]) -> oldBlock_1.proof
REF_1501(BeaconBlockHeader) -> beaconBlock_1.header
REF_1502(bytes32) -> REF_1501.stateRoot
REF_1503(BeaconBlockHeader) -> oldBlock_1.header
TMP_3541(bytes32) = LIBRARY_CALL, dest:SSZ, function:SSZ.hashTreeRoot(BeaconBlockHeader), arguments:['REF_1503'] 
REF_1505(BeaconBlockHeader) -> beaconBlock_1.header
REF_1506(Slot) -> REF_1505.slot
REF_1507(BeaconBlockHeader) -> oldBlock_1.header
REF_1508(Slot) -> REF_1507.slot
TMP_3542(GIndex) = INTERNAL_CALL, CSVerifier._getHistoricalBlockRootGI(Slot,Slot)(REF_1506,REF_1508)
LIBRARY_CALL, dest:SSZ, function:SSZ.verifyProof(bytes32[],bytes32,bytes32,GIndex), arguments:['REF_1500', 'REF_1502', 'TMP_3541', 'TMP_3542'] 
 pubkey = MODULE.getSigningKeys(nodeOperatorId,keyIndex,1)
TMP_3544(bytes) = HIGH_LEVEL_CALL, dest:MODULE_14(ICSModule), function:getSigningKeys, arguments:['nodeOperatorId_1', 'keyIndex_1', '1']  
MODULE_15(ICSModule) := phi(['MODULE_14', 'MODULE_17', 'MODULE_8', 'MODULE_1'])
pubkey_1(bytes) := TMP_3544(bytes)
 withdrawalAmount = _processWithdrawalProof({witness:witness,stateSlot:oldBlock.header.slot,stateRoot:oldBlock.header.stateRoot,pubkey:pubkey})
REF_1510(BeaconBlockHeader) -> oldBlock_1.header
REF_1511(Slot) -> REF_1510.slot
REF_1512(BeaconBlockHeader) -> oldBlock_1.header
REF_1513(bytes32) -> REF_1512.stateRoot
TMP_3545(uint256) = INTERNAL_CALL, CSVerifier._processWithdrawalProof(ICSVerifier.WithdrawalWitness,Slot,bytes32,bytes)(witness_1,REF_1511,REF_1513,pubkey_1)
withdrawalAmount_1(uint256) := TMP_3545(uint256)
 withdrawalsInfo = new ValidatorWithdrawalInfo[](1)
TMP_3547(ValidatorWithdrawalInfo[])  = new ValidatorWithdrawalInfo[](1)
withdrawalsInfo_1(ValidatorWithdrawalInfo[]) = ['TMP_3547(ValidatorWithdrawalInfo[])']
 withdrawalsInfo[0] = ValidatorWithdrawalInfo(nodeOperatorId,keyIndex,withdrawalAmount)
REF_1514(ValidatorWithdrawalInfo) -> withdrawalsInfo_1[0]
TMP_3548(ValidatorWithdrawalInfo) = new ValidatorWithdrawalInfo(nodeOperatorId_1,keyIndex_1,withdrawalAmount_1)
withdrawalsInfo_2(ValidatorWithdrawalInfo[]) := phi(['withdrawalsInfo_1'])
REF_1514(ValidatorWithdrawalInfo) (->withdrawalsInfo_2) := TMP_3548(ValidatorWithdrawalInfo)
 MODULE.submitWithdrawals(withdrawalsInfo)
HIGH_LEVEL_CALL, dest:MODULE_16(ICSModule), function:submitWithdrawals, arguments:['withdrawalsInfo_2']  
MODULE_17(ICSModule) := phi(['MODULE_17', 'MODULE_16', 'MODULE_8', 'MODULE_1'])
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
```
#### CSVerifier.constructor(address,address,uint64,uint64,ICSVerifier.GIndices,Slot,Slot,Slot,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_0'])
 withdrawalAddress == address(0)
TMP_3497 = CONVERT 0 to address
TMP_3498(bool) = withdrawalAddress_1 == TMP_3497
CONDITION TMP_3498
 revert ZeroWithdrawalAddress()()
TMP_3499(None) = SOLIDITY_CALL revert ZeroWithdrawalAddress()()
 module == address(0)
TMP_3500 = CONVERT 0 to address
TMP_3501(bool) = module_1 == TMP_3500
CONDITION TMP_3501
 revert ZeroModuleAddress()()
TMP_3502(None) = SOLIDITY_CALL revert ZeroModuleAddress()()
 admin == address(0)
TMP_3503 = CONVERT 0 to address
TMP_3504(bool) = admin_1 == TMP_3503
CONDITION TMP_3504
 revert ZeroAdminAddress()()
TMP_3505(None) = SOLIDITY_CALL revert ZeroAdminAddress()()
 slotsPerEpoch == 0
TMP_3506(bool) = slotsPerEpoch_1 == 0
CONDITION TMP_3506
 revert InvalidChainConfig()()
TMP_3507(None) = SOLIDITY_CALL revert InvalidChainConfig()()
 slotsPerHistoricalRoot == 0
TMP_3508(bool) = slotsPerHistoricalRoot_1 == 0
CONDITION TMP_3508
 revert InvalidChainConfig()()
TMP_3509(None) = SOLIDITY_CALL revert InvalidChainConfig()()
 gt(firstSupportedSlot,pivotSlot)
TMP_3510(bool) = INTERNAL_CALL, gt(Slot,Slot)(firstSupportedSlot_1,pivotSlot_1)
CONDITION TMP_3510
 revert InvalidPivotSlot()()
TMP_3511(None) = SOLIDITY_CALL revert InvalidPivotSlot()()
 gt(capellaSlot,firstSupportedSlot)
TMP_3512(bool) = INTERNAL_CALL, gt(Slot,Slot)(capellaSlot_1,firstSupportedSlot_1)
CONDITION TMP_3512
 revert InvalidCapellaSlot()()
TMP_3513(None) = SOLIDITY_CALL revert InvalidCapellaSlot()()
 WITHDRAWAL_ADDRESS = withdrawalAddress
WITHDRAWAL_ADDRESS_1(address) := withdrawalAddress_1(address)
 MODULE = ICSModule(module)
TMP_3514 = CONVERT module_1 to ICSModule
MODULE_1(ICSModule) := TMP_3514(ICSModule)
 SLOTS_PER_EPOCH = slotsPerEpoch
SLOTS_PER_EPOCH_1(uint64) := slotsPerEpoch_1(uint64)
 SLOTS_PER_HISTORICAL_ROOT = slotsPerHistoricalRoot
SLOTS_PER_HISTORICAL_ROOT_1(uint64) := slotsPerHistoricalRoot_1(uint64)
 GI_FIRST_WITHDRAWAL_PREV = gindices.gIFirstWithdrawalPrev
REF_1466(GIndex) -> gindices_1.gIFirstWithdrawalPrev
GI_FIRST_WITHDRAWAL_PREV_1(GIndex) := REF_1466(GIndex)
 GI_FIRST_WITHDRAWAL_CURR = gindices.gIFirstWithdrawalCurr
REF_1467(GIndex) -> gindices_1.gIFirstWithdrawalCurr
GI_FIRST_WITHDRAWAL_CURR_1(GIndex) := REF_1467(GIndex)
 GI_FIRST_VALIDATOR_PREV = gindices.gIFirstValidatorPrev
REF_1468(GIndex) -> gindices_1.gIFirstValidatorPrev
GI_FIRST_VALIDATOR_PREV_1(GIndex) := REF_1468(GIndex)
 GI_FIRST_VALIDATOR_CURR = gindices.gIFirstValidatorCurr
REF_1469(GIndex) -> gindices_1.gIFirstValidatorCurr
GI_FIRST_VALIDATOR_CURR_1(GIndex) := REF_1469(GIndex)
 GI_FIRST_HISTORICAL_SUMMARY_PREV = gindices.gIFirstHistoricalSummaryPrev
REF_1470(GIndex) -> gindices_1.gIFirstHistoricalSummaryPrev
GI_FIRST_HISTORICAL_SUMMARY_PREV_1(GIndex) := REF_1470(GIndex)
 GI_FIRST_HISTORICAL_SUMMARY_CURR = gindices.gIFirstHistoricalSummaryCurr
REF_1471(GIndex) -> gindices_1.gIFirstHistoricalSummaryCurr
GI_FIRST_HISTORICAL_SUMMARY_CURR_1(GIndex) := REF_1471(GIndex)
 GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV = gindices.gIFirstBlockRootInSummaryPrev
REF_1472(GIndex) -> gindices_1.gIFirstBlockRootInSummaryPrev
GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV_1(GIndex) := REF_1472(GIndex)
 GI_FIRST_BLOCK_ROOT_IN_SUMMARY_CURR = gindices.gIFirstBlockRootInSummaryCurr
REF_1473(GIndex) -> gindices_1.gIFirstBlockRootInSummaryCurr
GI_FIRST_BLOCK_ROOT_IN_SUMMARY_CURR_1(GIndex) := REF_1473(GIndex)
 FIRST_SUPPORTED_SLOT = firstSupportedSlot
FIRST_SUPPORTED_SLOT_1(Slot) := firstSupportedSlot_1(Slot)
 PIVOT_SLOT = pivotSlot
PIVOT_SLOT_1(Slot) := pivotSlot_1(Slot)
 CAPELLA_SLOT = capellaSlot
CAPELLA_SLOT_1(Slot) := capellaSlot_1(Slot)
 _grantRole(DEFAULT_ADMIN_ROLE,admin)
TMP_3515(bool) = INTERNAL_CALL, AccessControlEnumerable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_3,admin_1)
```
#### CSVerifier._getParentBlockRoot(uint64) [INTERNAL]
```slithir
blockTimestamp_1(uint64) := phi(['REF_1478', 'REF_1496'])
BEACON_ROOTS_1(address) := phi(['BEACON_ROOTS_2', 'BEACON_ROOTS_0'])
 (success,data) = BEACON_ROOTS.staticcall(abi.encode(blockTimestamp))
TMP_3551(bytes) = SOLIDITY_CALL abi.encode()(blockTimestamp_1)
TUPLE_17(bool,bytes) = LOW_LEVEL_CALL, dest:BEACON_ROOTS_1, function:staticcall, arguments:['TMP_3551']  
BEACON_ROOTS_2(address) := phi(['BEACON_ROOTS_1', 'BEACON_ROOTS_2'])
success_1(bool)= UNPACK TUPLE_17 index: 0 
data_1(bytes)= UNPACK TUPLE_17 index: 1 
 ! success || data.length == 0
TMP_3552 = UnaryType.BANG success_1 
REF_1518 -> LENGTH data_1
TMP_3553(bool) = REF_1518 == 0
TMP_3554(bool) = TMP_3552 || TMP_3553
CONDITION TMP_3554
 revert RootNotFound()()
TMP_3555(None) = SOLIDITY_CALL revert RootNotFound()()
 abi.decode(data,(bytes32))
TMP_3556(bytes32) = SOLIDITY_CALL abi.decode()(data_1,bytes32)
RETURN TMP_3556
```
#### CSVerifier._processWithdrawalProof(ICSVerifier.WithdrawalWitness,Slot,bytes32,bytes) [INTERNAL]
```slithir
witness_1(ICSVerifier.WithdrawalWitness) := phi(['witness_1', 'witness_1'])
stateSlot_1(Slot) := phi(['REF_1511', 'REF_1483'])
stateRoot_1(bytes32) := phi(['REF_1513', 'REF_1485'])
pubkey_1(bytes) := phi(['pubkey_1', 'pubkey_1'])
WITHDRAWAL_ADDRESS_2(address) := phi(['WITHDRAWAL_ADDRESS_1', 'WITHDRAWAL_ADDRESS_0'])
 withdrawalAddress = address(uint160(uint256(witness.withdrawalCredentials)))
REF_1520(bytes32) -> witness_1.withdrawalCredentials
TMP_3557 = CONVERT REF_1520 to uint256
TMP_3558 = CONVERT TMP_3557 to uint160
TMP_3559 = CONVERT TMP_3558 to address
withdrawalAddress_1(address) := TMP_3559(address)
 withdrawalAddress != WITHDRAWAL_ADDRESS
TMP_3560(bool) = withdrawalAddress_1 != WITHDRAWAL_ADDRESS_2
CONDITION TMP_3560
 revert InvalidWithdrawalAddress()()
TMP_3561(None) = SOLIDITY_CALL revert InvalidWithdrawalAddress()()
 _computeEpochAtSlot(stateSlot) < witness.withdrawableEpoch
TMP_3562(uint256) = INTERNAL_CALL, CSVerifier._computeEpochAtSlot(Slot)(stateSlot_1)
REF_1521(uint64) -> witness_1.withdrawableEpoch
TMP_3563(bool) = TMP_3562 < REF_1521
CONDITION TMP_3563
 revert ValidatorNotWithdrawn()()
TMP_3564(None) = SOLIDITY_CALL revert ValidatorNotWithdrawn()()
 ! witness.slashed && gweiToWei(witness.amount) < 8000000000000000000
REF_1522(bool) -> witness_1.slashed
TMP_3565 = UnaryType.BANG REF_1522 
REF_1523(uint64) -> witness_1.amount
TMP_3566(uint256) = INTERNAL_CALL, gweiToWei(uint64)(REF_1523)
TMP_3567(bool) = TMP_3566 < 8000000000000000000
TMP_3568(bool) = TMP_3565 && TMP_3567
CONDITION TMP_3568
 revert PartialWithdrawal()()
TMP_3569(None) = SOLIDITY_CALL revert PartialWithdrawal()()
 validator = Validator({pubkey:pubkey,withdrawalCredentials:witness.withdrawalCredentials,effectiveBalance:witness.effectiveBalance,slashed:witness.slashed,activationEligibilityEpoch:witness.activationEligibilityEpoch,activationEpoch:witness.activationEpoch,exitEpoch:witness.exitEpoch,withdrawableEpoch:witness.withdrawableEpoch})
REF_1524(bytes32) -> witness_1.withdrawalCredentials
REF_1525(uint64) -> witness_1.effectiveBalance
REF_1526(bool) -> witness_1.slashed
REF_1527(uint64) -> witness_1.activationEligibilityEpoch
REF_1528(uint64) -> witness_1.activationEpoch
REF_1529(uint64) -> witness_1.exitEpoch
REF_1530(uint64) -> witness_1.withdrawableEpoch
TMP_3570(Validator) = new Validator(pubkey_1,REF_1524,REF_1525,REF_1526,REF_1527,REF_1528,REF_1529,REF_1530)
validator_1(Validator) := TMP_3570(Validator)
 SSZ.verifyProof({proof:witness.validatorProof,root:stateRoot,leaf:validator.hashTreeRoot(),gI:_getValidatorGI(witness.validatorIndex,stateSlot)})
REF_1532(bytes32[]) -> witness_1.validatorProof
TMP_3571(bytes32) = LIBRARY_CALL, dest:SSZ, function:SSZ.hashTreeRoot(Validator), arguments:['validator_1'] 
REF_1534(uint64) -> witness_1.validatorIndex
TMP_3572(GIndex) = INTERNAL_CALL, CSVerifier._getValidatorGI(uint256,Slot)(REF_1534,stateSlot_1)
LIBRARY_CALL, dest:SSZ, function:SSZ.verifyProof(bytes32[],bytes32,bytes32,GIndex), arguments:['REF_1532', 'stateRoot_1', 'TMP_3571', 'TMP_3572'] 
 withdrawal = Withdrawal({index:witness.withdrawalIndex,validatorIndex:witness.validatorIndex,withdrawalAddress:withdrawalAddress,amount:witness.amount})
REF_1535(uint64) -> witness_1.withdrawalIndex
REF_1536(uint64) -> witness_1.validatorIndex
REF_1537(uint64) -> witness_1.amount
TMP_3574(Withdrawal) = new Withdrawal(REF_1535,REF_1536,withdrawalAddress_1,REF_1537)
withdrawal_1(Withdrawal) := TMP_3574(Withdrawal)
 SSZ.verifyProof({proof:witness.withdrawalProof,root:stateRoot,leaf:withdrawal.hashTreeRoot(),gI:_getWithdrawalGI(witness.withdrawalOffset,stateSlot)})
REF_1539(bytes32[]) -> witness_1.withdrawalProof
TMP_3575(bytes32) = LIBRARY_CALL, dest:SSZ, function:SSZ.hashTreeRoot(Withdrawal), arguments:['withdrawal_1'] 
REF_1541(uint8) -> witness_1.withdrawalOffset
TMP_3576(GIndex) = INTERNAL_CALL, CSVerifier._getWithdrawalGI(uint256,Slot)(REF_1541,stateSlot_1)
LIBRARY_CALL, dest:SSZ, function:SSZ.verifyProof(bytes32[],bytes32,bytes32,GIndex), arguments:['REF_1539', 'stateRoot_1', 'TMP_3575', 'TMP_3576'] 
 withdrawal.amountWei()
TMP_3578(uint256) = INTERNAL_CALL, amountWei(Withdrawal)(withdrawal_1)
RETURN TMP_3578
 withdrawalAmount
```
#### CSVerifier._getValidatorGI(uint256,Slot) [INTERNAL]
```slithir
offset_1(uint256) := phi(['REF_1534'])
stateSlot_1(Slot) := phi(['stateSlot_1'])
GI_FIRST_VALIDATOR_PREV_2(GIndex) := phi(['GI_FIRST_VALIDATOR_PREV_1', 'GI_FIRST_VALIDATOR_PREV_3', 'GI_FIRST_VALIDATOR_PREV_0'])
GI_FIRST_VALIDATOR_CURR_2(GIndex) := phi(['GI_FIRST_VALIDATOR_CURR_0', 'GI_FIRST_VALIDATOR_CURR_3', 'GI_FIRST_VALIDATOR_CURR_1'])
PIVOT_SLOT_2(Slot) := phi(['PIVOT_SLOT_1', 'PIVOT_SLOT_0', 'PIVOT_SLOT_3', 'PIVOT_SLOT_5', 'PIVOT_SLOT_12'])
 gI.shr(offset)
TMP_3579(GIndex) = INTERNAL_CALL, shr(GIndex,uint256)(gI_3,offset_1)
RETURN TMP_3579
 lt(stateSlot,PIVOT_SLOT)
TMP_3580(bool) = INTERNAL_CALL, lt(Slot,Slot)(stateSlot_1,PIVOT_SLOT_2)
CONDITION TMP_3580
 gI = GI_FIRST_VALIDATOR_PREV
gI_1(GIndex) := GI_FIRST_VALIDATOR_PREV_3(GIndex)
 gI = GI_FIRST_VALIDATOR_CURR
gI_2(GIndex) := GI_FIRST_VALIDATOR_CURR_3(GIndex)
gI_3(GIndex) := phi(['gI_1', 'gI_2'])
```
#### CSVerifier._getWithdrawalGI(uint256,Slot) [INTERNAL]
```slithir
offset_1(uint256) := phi(['REF_1541'])
stateSlot_1(Slot) := phi(['stateSlot_1'])
GI_FIRST_WITHDRAWAL_PREV_2(GIndex) := phi(['GI_FIRST_WITHDRAWAL_PREV_0', 'GI_FIRST_WITHDRAWAL_PREV_1', 'GI_FIRST_WITHDRAWAL_PREV_3'])
GI_FIRST_WITHDRAWAL_CURR_2(GIndex) := phi(['GI_FIRST_WITHDRAWAL_CURR_0', 'GI_FIRST_WITHDRAWAL_CURR_3', 'GI_FIRST_WITHDRAWAL_CURR_1'])
PIVOT_SLOT_4(Slot) := phi(['PIVOT_SLOT_1', 'PIVOT_SLOT_0', 'PIVOT_SLOT_3', 'PIVOT_SLOT_5', 'PIVOT_SLOT_12'])
 gI.shr(offset)
TMP_3581(GIndex) = INTERNAL_CALL, shr(GIndex,uint256)(gI_3,offset_1)
RETURN TMP_3581
 lt(stateSlot,PIVOT_SLOT)
TMP_3582(bool) = INTERNAL_CALL, lt(Slot,Slot)(stateSlot_1,PIVOT_SLOT_4)
CONDITION TMP_3582
 gI = GI_FIRST_WITHDRAWAL_PREV
gI_2(GIndex) := GI_FIRST_WITHDRAWAL_PREV_3(GIndex)
 gI = GI_FIRST_WITHDRAWAL_CURR
gI_1(GIndex) := GI_FIRST_WITHDRAWAL_CURR_3(GIndex)
gI_3(GIndex) := phi(['gI_1', 'gI_2'])
```
#### CSVerifier._getHistoricalBlockRootGI(Slot,Slot) [INTERNAL]
```slithir
recentSlot_1(Slot) := phi(['REF_1506'])
targetSlot_1(Slot) := phi(['REF_1508'])
SLOTS_PER_HISTORICAL_ROOT_2(uint64) := phi(['SLOTS_PER_HISTORICAL_ROOT_1', 'SLOTS_PER_HISTORICAL_ROOT_5', 'SLOTS_PER_HISTORICAL_ROOT_0'])
GI_FIRST_HISTORICAL_SUMMARY_PREV_2(GIndex) := phi(['GI_FIRST_HISTORICAL_SUMMARY_PREV_1', 'GI_FIRST_HISTORICAL_SUMMARY_PREV_0', 'GI_FIRST_HISTORICAL_SUMMARY_PREV_6'])
GI_FIRST_HISTORICAL_SUMMARY_CURR_2(GIndex) := phi(['GI_FIRST_HISTORICAL_SUMMARY_CURR_6', 'GI_FIRST_HISTORICAL_SUMMARY_CURR_1', 'GI_FIRST_HISTORICAL_SUMMARY_CURR_0'])
GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV_2(GIndex) := phi(['GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV_1', 'GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV_0', 'GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV_8', 'GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV_9'])
GI_FIRST_BLOCK_ROOT_IN_SUMMARY_CURR_2(GIndex) := phi(['GI_FIRST_BLOCK_ROOT_IN_SUMMARY_CURR_1', 'GI_FIRST_BLOCK_ROOT_IN_SUMMARY_CURR_9', 'GI_FIRST_BLOCK_ROOT_IN_SUMMARY_CURR_8', 'GI_FIRST_BLOCK_ROOT_IN_SUMMARY_CURR_0'])
PIVOT_SLOT_6(Slot) := phi(['PIVOT_SLOT_1', 'PIVOT_SLOT_0', 'PIVOT_SLOT_3', 'PIVOT_SLOT_5', 'PIVOT_SLOT_12'])
CAPELLA_SLOT_2(Slot) := phi(['CAPELLA_SLOT_4', 'CAPELLA_SLOT_1', 'CAPELLA_SLOT_0'])
 targetSlotShifted = targetSlot.unwrap() - CAPELLA_SLOT.unwrap()
TMP_3583(uint64) = INTERNAL_CALL, unwrap(Slot)(targetSlot_1)
TMP_3584(uint64) = INTERNAL_CALL, unwrap(Slot)(CAPELLA_SLOT_3)
TMP_3585(uint64) = TMP_3583 (c)- TMP_3584
targetSlotShifted_1(uint256) := TMP_3585(uint64)
 summaryIndex = targetSlotShifted / SLOTS_PER_HISTORICAL_ROOT
TMP_3586(uint256) = targetSlotShifted_1 (c)/ SLOTS_PER_HISTORICAL_ROOT_4
summaryIndex_1(uint256) := TMP_3586(uint256)
 rootIndex = targetSlot.unwrap() % SLOTS_PER_HISTORICAL_ROOT
TMP_3587(uint64) = INTERNAL_CALL, unwrap(Slot)(targetSlot_1)
TMP_3588(uint64) = TMP_3587 % SLOTS_PER_HISTORICAL_ROOT_5
rootIndex_1(uint256) := TMP_3588(uint64)
 gI = gI.shr(summaryIndex)
TMP_3589(GIndex) = INTERNAL_CALL, shr(GIndex,uint256)(gI_3,summaryIndex_1)
gI_4(GIndex) := TMP_3589(GIndex)
 gI = gI.shr(rootIndex)
TMP_3590(GIndex) = INTERNAL_CALL, shr(GIndex,uint256)(gI_7,rootIndex_1)
gI_8(GIndex) := TMP_3590(GIndex)
 lt(recentSlot,PIVOT_SLOT)
TMP_3591(bool) = INTERNAL_CALL, lt(Slot,Slot)(recentSlot_1,PIVOT_SLOT_9)
CONDITION TMP_3591
 gI = GI_FIRST_HISTORICAL_SUMMARY_PREV
gI_1(GIndex) := GI_FIRST_HISTORICAL_SUMMARY_PREV_6(GIndex)
 gI = GI_FIRST_HISTORICAL_SUMMARY_CURR
gI_2(GIndex) := GI_FIRST_HISTORICAL_SUMMARY_CURR_6(GIndex)
gI_3(GIndex) := phi(['gI_1', 'gI_2'])
 lt(targetSlot,PIVOT_SLOT)
TMP_3592(bool) = INTERNAL_CALL, lt(Slot,Slot)(targetSlot_1,PIVOT_SLOT_11)
CONDITION TMP_3592
 gI = gI.concat(GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV)
TMP_3593(GIndex) = INTERNAL_CALL, concat(GIndex,GIndex)(gI_4,GI_FIRST_BLOCK_ROOT_IN_SUMMARY_PREV_8)
gI_6(GIndex) := TMP_3593(GIndex)
 gI = gI.concat(GI_FIRST_BLOCK_ROOT_IN_SUMMARY_CURR)
TMP_3594(GIndex) = INTERNAL_CALL, concat(GIndex,GIndex)(gI_4,GI_FIRST_BLOCK_ROOT_IN_SUMMARY_CURR_8)
gI_5(GIndex) := TMP_3594(GIndex)
gI_7(GIndex) := phi(['gI_5', 'gI_6'])
 gI
RETURN gI_8
```
#### CSVerifier._computeEpochAtSlot(Slot) [INTERNAL]
```slithir
slot_1(Slot) := phi(['stateSlot_1'])
SLOTS_PER_EPOCH_2(uint64) := phi(['SLOTS_PER_EPOCH_1', 'SLOTS_PER_EPOCH_0', 'SLOTS_PER_EPOCH_3'])
 slot.unwrap() / SLOTS_PER_EPOCH
TMP_3595(uint64) = INTERNAL_CALL, unwrap(Slot)(slot_1)
TMP_3596(uint64) = TMP_3595 (c)/ SLOTS_PER_EPOCH_3
RETURN TMP_3596
```
#### CSStrikes.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
role_1(bytes32) := phi(['TMP_3269', 'TMP_3274', 'TMP_3271', 'DEFAULT_ADMIN_ROLE_6', 'TMP_3276'])
 _checkRole(role)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(role_1)
 $ = _getInitializableStorage()
TMP_3403(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_3403'])(Initializable.InitializableStorage) := TMP_3403(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_1427(bool) -> $_1 (-> ['TMP_3403'])._initializing
TMP_3404 = UnaryType.BANG REF_1427 
isTopLevelCall_1(bool) := TMP_3404(bool)
 initialized = $._initialized
REF_1428(uint64) -> $_1 (-> ['TMP_3403'])._initialized
initialized_1(uint64) := REF_1428(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_3405(bool) = initialized_1 == 0
TMP_3406(bool) = TMP_3405 && isTopLevelCall_1
initialSetup_1(bool) := TMP_3406(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_3407(bool) = initialized_1 == 1
TMP_3408 = CONVERT this to address
TMP_3409(bytes) = SOLIDITY_CALL code(address)(TMP_3408)
REF_1429 -> LENGTH TMP_3409
TMP_3410(bool) = REF_1429 == 0
TMP_3411(bool) = TMP_3407 && TMP_3410
construction_1(bool) := TMP_3411(bool)
 ! initialSetup && ! construction
TMP_3412 = UnaryType.BANG initialSetup_1 
TMP_3413 = UnaryType.BANG construction_1 
TMP_3414(bool) = TMP_3412 && TMP_3413
CONDITION TMP_3414
 revert InvalidInitialization()()
TMP_3415(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_1430(uint64) -> $_1 (-> ['TMP_3403'])._initialized
$_2 (-> ['TMP_3403'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_3403'])"])
REF_1430(uint64) (->$_2 (-> ['TMP_3403'])) := 1(uint256)
TMP_3403(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_3403'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_1431(bool) -> $_2 (-> ['TMP_3403'])._initializing
$_3 (-> ['TMP_3403'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_3403'])"])
REF_1431(bool) (->$_3 (-> ['TMP_3403'])) := True(bool)
TMP_3403(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_3403'])"])
$_4 (-> ['TMP_3403'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_3403'])", "$_2 (-> ['TMP_3403'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_1432(bool) -> $_4 (-> ['TMP_3403'])._initializing
$_5 (-> ['TMP_3403'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_3403'])"])
REF_1432(bool) (->$_5 (-> ['TMP_3403'])) := False(bool)
TMP_3403(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_3403'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_3417(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_3417'])(Initializable.InitializableStorage) := TMP_3417(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_1433(bool) -> $_1 (-> ['TMP_3417'])._initializing
REF_1434(uint64) -> $_1 (-> ['TMP_3417'])._initialized
TMP_3418(bool) = REF_1434 >= version_1
TMP_3419(bool) = REF_1433 || TMP_3418
CONDITION TMP_3419
 revert InvalidInitialization()()
TMP_3420(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_1435(uint64) -> $_1 (-> ['TMP_3417'])._initialized
$_2 (-> ['TMP_3417'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_3417'])"])
REF_1435(uint64) (->$_2 (-> ['TMP_3417'])) := version_1(uint64)
TMP_3417(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_3417'])"])
 $._initializing = true
REF_1436(bool) -> $_2 (-> ['TMP_3417'])._initializing
$_3 (-> ['TMP_3417'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_3417'])"])
REF_1436(bool) (->$_3 (-> ['TMP_3417'])) := True(bool)
TMP_3417(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_3417'])"])
 $._initializing = false
REF_1437(bool) -> $_3 (-> ['TMP_3417'])._initializing
$_4 (-> ['TMP_3417'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_3417'])"])
REF_1437(bool) (->$_4 (-> ['TMP_3417'])) := False(bool)
TMP_3417(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_3417'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
ORACLE_2(address) := phi(['ORACLE_0', 'ORACLE_1'])
 msg.sender != ORACLE
TMP_3423(bool) = msg.sender != ORACLE_2
CONDITION TMP_3423
 revert SenderIsNotOracle()()
TMP_3424(None) = SOLIDITY_CALL revert SenderIsNotOracle()()
```
#### SSZ.hashTreeRoot(Withdrawal) [INTERNAL]
```slithir
 sha256(bytes)(bytes.concat(sha256(bytes)(bytes.concat(toLittleEndian(withdrawal.index),toLittleEndian(withdrawal.validatorIndex))),sha256(bytes)(bytes.concat(bytes20(withdrawal.withdrawalAddress),bytes12(0),toLittleEndian(withdrawal.amount)))))
REF_1976(uint64) -> withdrawal_1.index
TMP_4457(bytes32) = INTERNAL_CALL, SSZ.toLittleEndian(uint256)(REF_1976)
REF_1977(uint64) -> withdrawal_1.validatorIndex
TMP_4458(bytes32) = INTERNAL_CALL, SSZ.toLittleEndian(uint256)(REF_1977)
TMP_4459(bytes) = SOLIDITY_CALL bytes.concat()(TMP_4457,TMP_4458)
TMP_4460(bytes32) = SOLIDITY_CALL sha256(bytes)(TMP_4459)
REF_1979(address) -> withdrawal_1.withdrawalAddress
TMP_4461 = CONVERT REF_1979 to bytes20
TMP_4462 = CONVERT 0 to bytes12
REF_1980(uint64) -> withdrawal_1.amount
TMP_4463(bytes32) = INTERNAL_CALL, SSZ.toLittleEndian(uint256)(REF_1980)
TMP_4464(bytes) = SOLIDITY_CALL bytes.concat()(TMP_4461,TMP_4462,TMP_4463)
TMP_4465(bytes32) = SOLIDITY_CALL sha256(bytes)(TMP_4464)
TMP_4466(bytes) = SOLIDITY_CALL bytes.concat()(TMP_4460,TMP_4465)
TMP_4467(bytes32) = SOLIDITY_CALL sha256(bytes)(TMP_4466)
RETURN TMP_4467
```
#### ICSModule.submitWithdrawals(ValidatorWithdrawalInfo[]) [EXTERNAL]
```slithir

```
#### ICSModule.getSigningKeys(uint256,uint256,uint256) [EXTERNAL]
```slithir

```
#### SSZ.verifyProof(bytes32[],bytes32,bytes32,GIndex) [INTERNAL]
```slithir
 index = gI.index()
TMP_4425(uint256) = INTERNAL_CALL, index(GIndex)(gI_1)
index_1(uint256) := TMP_4425(uint256)
 ! proof
TMP_4426 = UnaryType.BANG proof_1 
CONDITION TMP_4426
 mstore(uint256,uint256)(0x00,0x09bde339)
TMP_4427(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,163439417)
 revert(uint256,uint256)(0x1c,0x04)
TMP_4428(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 end_verifyProof_asm_0 = proof + proof << 5
TMP_4429(bytes32[]) = proof_1 << 5
TMP_4430(bytes32[]) = proof_1 + TMP_4429
end_verifyProof_asm_0_1(uint256) := TMP_4430(bytes32[])
 offset_verifyProof_asm_0 = proof
offset_verifyProof_asm_0_1(uint256) := proof_1(bytes32[])
 1
leaf_2(bytes32) := phi(['leaf_1', 'leaf_3'])
index_2(uint256) := phi(['index_1', 'index_3'])
offset_verifyProof_asm_0_2(uint256) := phi(['offset_verifyProof_asm_0_1', 'offset_verifyProof_asm_0_3'])
CONDITION 1
 scratch_verifyProof_asm_0 = index & 1 << 5
TMP_4431(uint256) = index_2 & 1
TMP_4432(uint256) = TMP_4431 << 5
scratch_verifyProof_asm_0_1(uint256) := TMP_4432(uint256)
 index = index >> 1
TMP_4433(uint256) = index_2 >> 1
index_3(uint256) := TMP_4433(uint256)
 ! index
TMP_4434 = UnaryType.BANG index_3 
CONDITION TMP_4434
 mstore(uint256,uint256)(0x00,0x5849603f)
TMP_4435(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,1481203775)
 revert(uint256,uint256)(0x1c,0x04)
TMP_4436(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 mstore(uint256,uint256)(scratch_verifyProof_asm_0,leaf)
TMP_4437(None) = SOLIDITY_CALL mstore(uint256,uint256)(scratch_verifyProof_asm_0_1,leaf_2)
 mstore(uint256,uint256)(scratch_verifyProof_asm_0 ^ 0x20,calldataload(uint256)(offset_verifyProof_asm_0))
TMP_4438(uint256) = scratch_verifyProof_asm_0_1 ^ 32
TMP_4439(uint256) = SOLIDITY_CALL calldataload(uint256)(offset_verifyProof_asm_0_2)
TMP_4440(None) = SOLIDITY_CALL mstore(uint256,uint256)(TMP_4438,TMP_4439)
 result_verifyProof_asm_0 = staticcall(uint256,uint256,uint256,uint256,uint256,uint256)(gas()(),0x02,0x00,0x40,0x00,0x20)
TMP_4441(uint256) = SOLIDITY_CALL gas()()
TMP_4442(uint256) = SOLIDITY_CALL staticcall(uint256,uint256,uint256,uint256,uint256,uint256)(TMP_4441,2,0,64,0,32)
result_verifyProof_asm_0_1(uint256) := TMP_4442(uint256)
 ! result_verifyProof_asm_0
TMP_4443 = UnaryType.BANG result_verifyProof_asm_0_1 
CONDITION TMP_4443
 revert(uint256,uint256)(0,0)
TMP_4444(None) = SOLIDITY_CALL revert(uint256,uint256)(0,0)
 leaf = mload(uint256)(0x00)
TMP_4445(uint256) = SOLIDITY_CALL mload(uint256)(0)
leaf_3(bytes32) := TMP_4445(uint256)
 offset_verifyProof_asm_0 = offset_verifyProof_asm_0 + 0x20
TMP_4446(uint256) = offset_verifyProof_asm_0_2 + 32
offset_verifyProof_asm_0_3(uint256) := TMP_4446(uint256)
 ! offset_verifyProof_asm_0 < end_verifyProof_asm_0
TMP_4447(bool) = offset_verifyProof_asm_0_3 < end_verifyProof_asm_0_1
TMP_4448 = UnaryType.BANG TMP_4447 
CONDITION TMP_4448
 ! index == 1
TMP_4449(bool) = index_2 == 1
TMP_4450 = UnaryType.BANG TMP_4449 
CONDITION TMP_4450
 mstore(uint256,uint256)(0x00,0x1b6661c3)
TMP_4451(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,459694531)
 revert(uint256,uint256)(0x1c,0x04)
TMP_4452(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 ! leaf == root
TMP_4453(bool) = leaf_2 == root_1
TMP_4454 = UnaryType.BANG TMP_4453 
CONDITION TMP_4454
 mstore(uint256,uint256)(0x00,0x09bde339)
TMP_4455(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,163439417)
 revert(uint256,uint256)(0x1c,0x04)
TMP_4456(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
```
#### SSZ.toLittleEndian(uint256) [INTERNAL]
```slithir
v_1(uint256) := phi(['TMP_4371', 'REF_1971', 'REF_1969', 'REF_1976', 'REF_1980', 'REF_1962', 'REF_1972', 'REF_1967', 'REF_1970', 'REF_1977'])
 v = ((v & 0xFF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00) >> 8) | ((v & 0x00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF) << 8)
TMP_4468(uint256) = v_1 & 115341536360906404779899502576747487978354537254490211650198994186870666100480
TMP_4469(uint256) = TMP_4468 >> 8
TMP_4470(uint256) = v_1 & 450552876409790643671482431940419874915447411150352389258589821042463539455
TMP_4471(uint256) = TMP_4470 << 8
TMP_4472(uint256) = TMP_4469 | TMP_4471
v_2(uint256) := TMP_4472(uint256)
 v = ((v & 0xFFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000) >> 16) | ((v & 0x0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF) << 16)
TMP_4473(uint256) = v_2 & 115790322417210952336529717160220497262186272106556906860092653394915770695680
TMP_4474(uint256) = TMP_4473 >> 16
TMP_4475(uint256) = v_2 & 1766820105243087041267848467410591083712559083657179364930612997358944255
TMP_4476(uint256) = TMP_4475 << 16
TMP_4477(uint256) = TMP_4474 | TMP_4476
v_3(uint256) := TMP_4477(uint256)
 v = ((v & 0xFFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000) >> 32) | ((v & 0x00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF) << 32)
TMP_4478(uint256) = v_3 & 115792089210356248762697446947946071893095522863849111501270640965525260206080
TMP_4479(uint256) = TMP_4478 >> 32
TMP_4480(uint256) = v_3 & 26959946660873538060741835960174461801791452538186943042387869433855
TMP_4481(uint256) = TMP_4480 << 32
TMP_4482(uint256) = TMP_4479 | TMP_4481
v_4(uint256) := TMP_4482(uint256)
 v = ((v & 0xFFFFFFFFFFFFFFFF0000000000000000FFFFFFFFFFFFFFFF0000000000000000) >> 64) | ((v & 0x0000000000000000FFFFFFFFFFFFFFFF0000000000000000FFFFFFFFFFFFFFFF) << 64)
TMP_4483(uint256) = v_4 & 115792089237316195417293883273301227089774477609353836086800156426807153786880
TMP_4484(uint256) = TMP_4483 >> 64
TMP_4485(uint256) = v_4 & 6277101735386680763495507056286727952657427581105975853055
TMP_4486(uint256) = TMP_4485 << 64
TMP_4487(uint256) = TMP_4484 | TMP_4486
v_5(uint256) := TMP_4487(uint256)
 v = (v >> 128) | (v << 128)
TMP_4488(uint256) = v_5 >> 128
TMP_4489(uint256) = v_5 << 128
TMP_4490(uint256) = TMP_4488 | TMP_4489
v_6(uint256) := TMP_4490(uint256)
 bytes32(v)
TMP_4491 = CONVERT v_6 to bytes32
RETURN TMP_4491
```
