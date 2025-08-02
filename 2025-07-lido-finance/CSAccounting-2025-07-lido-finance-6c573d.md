




### Storage layout (CSAccounting) 

```text
_feeDistributorOld ICSFeeDistributor
chargePenaltyRecipient address

```



#### CSAccounting.recoverERC20(address,uint256) [EXTERNAL]
```slithir
LIDO_51(ILido) := phi(['LIDO_62', 'LIDO_56', 'LIDO_0', 'LIDO_31', 'LIDO_34', 'LIDO_37', 'LIDO_59', 'LIDO_52', 'LIDO_61', 'LIDO_3', 'LIDO_25', 'LIDO_39', 'LIDO_50', 'LIDO_35', 'LIDO_19', 'LIDO_1', 'LIDO_6', 'LIDO_11'])
 _onlyRecoverer()
INTERNAL_CALL, CSAccounting._onlyRecoverer()()
 token == address(LIDO)
TMP_1515 = CONVERT LIDO_52 to address
TMP_1516(bool) = token_1 == TMP_1515
CONDITION TMP_1516
 revert NotAllowedToRecover()()
TMP_1517(None) = SOLIDITY_CALL revert NotAllowedToRecover()()
 AssetRecovererLib.recoverERC20(token,amount)
LIBRARY_CALL, dest:AssetRecovererLib, function:AssetRecovererLib.recoverERC20(address,uint256), arguments:['token_1', 'amount_1']
```
#### CSAccounting._onlyRecoverer() [INTERNAL]
```slithir
RECOVERER_ROLE_1(bytes32) := phi(['RECOVERER_ROLE_2', 'RECOVERER_ROLE_0'])
 _checkRole(RECOVERER_ROLE)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(RECOVERER_ROLE_1)
```
#### CSAccounting.constructor(address,address,address,uint256,uint256) [INTERNAL]
```slithir
 module == address(0)
TMP_1391 = CONVERT 0 to address
TMP_1392(bool) = module_1 == TMP_1391
CONDITION TMP_1392
 revert ZeroModuleAddress()()
TMP_1393(None) = SOLIDITY_CALL revert ZeroModuleAddress()()
 _feeDistributor == address(0)
TMP_1394 = CONVERT 0 to address
TMP_1395(bool) = _feeDistributor_1 == TMP_1394
CONDITION TMP_1395
 revert ZeroFeeDistributorAddress()()
TMP_1396(None) = SOLIDITY_CALL revert ZeroFeeDistributorAddress()()
 MODULE = ICSModule(module)
TMP_1397 = CONVERT module_1 to ICSModule
MODULE_1(ICSModule) := TMP_1397(ICSModule)
 FEE_DISTRIBUTOR = ICSFeeDistributor(_feeDistributor)
TMP_1398 = CONVERT _feeDistributor_1 to ICSFeeDistributor
FEE_DISTRIBUTOR_1(ICSFeeDistributor) := TMP_1398(ICSFeeDistributor)
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
 CSBondCore(lidoLocator)
INTERNAL_CALL, CSBondCore.constructor(address)(lidoLocator_1)
 CSBondLock(minBondLockPeriod,maxBondLockPeriod)
INTERNAL_CALL, CSBondLock.constructor(uint256,uint256)(minBondLockPeriod_1,maxBondLockPeriod_1)
```
#### CSAccounting._getClaimableBondShares(uint256) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1'])
 (currentShares,requiredShares) = getBondSummaryShares(nodeOperatorId)
TUPLE_7(uint256,uint256) = INTERNAL_CALL, CSAccounting.getBondSummaryShares(uint256)(nodeOperatorId_1)
currentShares_1(uint256)= UNPACK TUPLE_7 index: 0 
requiredShares_1(uint256)= UNPACK TUPLE_7 index: 1 
 currentShares > requiredShares
TMP_1566(bool) = currentShares_1 > requiredShares_1
CONDITION TMP_1566
 currentShares - requiredShares
TMP_1567(uint256) = currentShares_1 - requiredShares_1
RETURN TMP_1567
 0
RETURN 0
```
#### CSAccounting.feeDistributor() [EXTERNAL]
```slithir
FEE_DISTRIBUTOR_4(ICSFeeDistributor) := phi(['FEE_DISTRIBUTOR_3', 'FEE_DISTRIBUTOR_1', 'FEE_DISTRIBUTOR_0', 'FEE_DISTRIBUTOR_6'])
 FEE_DISTRIBUTOR
RETURN FEE_DISTRIBUTOR_4
```
#### CSAccounting.getInitializedVersion() [EXTERNAL]
```slithir
 _getInitializedVersion()
TMP_1530(uint64) = INTERNAL_CALL, Initializable._getInitializedVersion()()
RETURN TMP_1530
```
#### CSAccounting.resume() [EXTERNAL]
```slithir
RESUME_ROLE_1(bytes32) := phi(['RESUME_ROLE_2', 'RESUME_ROLE_0'])
 _resume()
INTERNAL_CALL, PausableUntil._resume()()
 onlyRole(RESUME_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(RESUME_ROLE_1)
```
#### CSAccounting.pauseFor(uint256) [EXTERNAL]
```slithir
PAUSE_ROLE_1(bytes32) := phi(['PAUSE_ROLE_0', 'PAUSE_ROLE_2'])
 _pauseFor(duration)
INTERNAL_CALL, PausableUntil._pauseFor(uint256)(duration_1)
 onlyRole(PAUSE_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(PAUSE_ROLE_1)
```
#### CSAccounting.setChargePenaltyRecipient(address) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_7(bytes32) := phi(['DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_10'])
 _setChargePenaltyRecipient(_chargePenaltyRecipient)
INTERNAL_CALL, CSAccounting._setChargePenaltyRecipient(address)(_chargePenaltyRecipient_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_7)
```
#### CSAccounting.setBondLockPeriod(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_9(bytes32) := phi(['DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_10'])
 CSBondLock._setBondLockPeriod(period)
INTERNAL_CALL, CSBondLock._setBondLockPeriod(uint256)(period_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_9)
```
#### CSAccounting.addBondCurve(ICSBondCurve.BondCurveIntervalInput[]) [EXTERNAL]
```slithir
MANAGE_BOND_CURVES_ROLE_1(bytes32) := phi(['MANAGE_BOND_CURVES_ROLE_0', 'MANAGE_BOND_CURVES_ROLE_4', 'MANAGE_BOND_CURVES_ROLE_2'])
 id = CSBondCurve._addBondCurve(bondCurve)
TMP_1438(uint256) = INTERNAL_CALL, CSBondCurve._addBondCurve(ICSBondCurve.BondCurveIntervalInput[])(bondCurve_1)
id_1(uint256) := TMP_1438(uint256)
 onlyRole(MANAGE_BOND_CURVES_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(MANAGE_BOND_CURVES_ROLE_1)
 id
RETURN id_1
```
#### CSAccounting.updateBondCurve(uint256,ICSBondCurve.BondCurveIntervalInput[]) [EXTERNAL]
```slithir
MANAGE_BOND_CURVES_ROLE_3(bytes32) := phi(['MANAGE_BOND_CURVES_ROLE_0', 'MANAGE_BOND_CURVES_ROLE_4', 'MANAGE_BOND_CURVES_ROLE_2'])
 CSBondCurve._updateBondCurve(curveId,bondCurve)
INTERNAL_CALL, CSBondCurve._updateBondCurve(uint256,ICSBondCurve.BondCurveIntervalInput[])(curveId_1,bondCurve_1)
 onlyRole(MANAGE_BOND_CURVES_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(MANAGE_BOND_CURVES_ROLE_3)
```
#### CSAccounting.getRequiredBondForNextKeys(uint256,uint256) [EXTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1'])
additionalKeys_1(uint256) := phi(['additionalKeys_1'])
 current = CSBondCore.getBond(nodeOperatorId)
TMP_1546(uint256) = INTERNAL_CALL, CSBondCore.getBond(uint256)(nodeOperatorId_1)
current_1(uint256) := TMP_1546(uint256)
 totalRequired = _getRequiredBond(nodeOperatorId,additionalKeys)
TMP_1547(uint256) = INTERNAL_CALL, CSAccounting._getRequiredBond(uint256,uint256)(nodeOperatorId_1,additionalKeys_1)
totalRequired_1(uint256) := TMP_1547(uint256)
 totalRequired > current
TMP_1548(bool) = totalRequired_1 > current_1
CONDITION TMP_1548
 totalRequired - current
TMP_1549(uint256) = totalRequired_1 - current_1
RETURN TMP_1549
 0
RETURN 0
```
#### CSAccounting.getBondAmountByKeysCountWstETH(uint256,uint256) [EXTERNAL]
```slithir
 _sharesByEth(CSBondCurve.getBondAmountByKeysCount(keysCount,curveId))
TMP_1535(uint256) = INTERNAL_CALL, CSBondCurve.getBondAmountByKeysCount(uint256,uint256)(keysCount_1,curveId_1)
TMP_1536(uint256) = INTERNAL_CALL, CSBondCore._sharesByEth(uint256)(TMP_1535)
RETURN TMP_1536
```
#### CSAccounting.getRequiredBondForNextKeysWstETH(uint256,uint256) [EXTERNAL]
```slithir
 _sharesByEth(getRequiredBondForNextKeys(nodeOperatorId,additionalKeys))
TMP_1537(uint256) = INTERNAL_CALL, CSAccounting.getRequiredBondForNextKeys(uint256,uint256)(nodeOperatorId_1,additionalKeys_1)
TMP_1538(uint256) = INTERNAL_CALL, CSBondCore._sharesByEth(uint256)(TMP_1537)
RETURN TMP_1538
```
#### CSAccounting.getUnbondedKeysCount(uint256) [EXTERNAL]
```slithir
 _getUnbondedKeysCount({nodeOperatorId:nodeOperatorId,includeLockedBond:true})
TMP_1533(uint256) = INTERNAL_CALL, CSAccounting._getUnbondedKeysCount(uint256,bool)(nodeOperatorId_1,True)
RETURN TMP_1533
```
#### CSAccounting.getUnbondedKeysCountToEject(uint256) [EXTERNAL]
```slithir
 _getUnbondedKeysCount({nodeOperatorId:nodeOperatorId,includeLockedBond:false})
TMP_1534(uint256) = INTERNAL_CALL, CSAccounting._getUnbondedKeysCount(uint256,bool)(nodeOperatorId_1,False)
RETURN TMP_1534
```
#### CSAccounting.getBondSummary(uint256) [EXTERNAL]
```slithir
 current = CSBondCore.getBond(nodeOperatorId)
TMP_1531(uint256) = INTERNAL_CALL, CSBondCore.getBond(uint256)(nodeOperatorId_1)
current_1(uint256) := TMP_1531(uint256)
 required = _getRequiredBond(nodeOperatorId,0)
TMP_1532(uint256) = INTERNAL_CALL, CSAccounting._getRequiredBond(uint256,uint256)(nodeOperatorId_1,0)
required_1(uint256) := TMP_1532(uint256)
 (current,required)
RETURN current_1,required_1
```
#### CSAccounting.getBondSummaryShares(uint256) [EXTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1'])
 current = CSBondCore.getBondShares(nodeOperatorId)
TMP_1544(uint256) = INTERNAL_CALL, CSBondCore.getBondShares(uint256)(nodeOperatorId_1)
current_1(uint256) := TMP_1544(uint256)
 required = _getRequiredBondShares(nodeOperatorId,0)
TMP_1545(uint256) = INTERNAL_CALL, CSAccounting._getRequiredBondShares(uint256,uint256)(nodeOperatorId_1,0)
required_1(uint256) := TMP_1545(uint256)
 (current,required)
RETURN current_1,required_1
```
#### CSAccounting.getClaimableBondShares(uint256) [EXTERNAL]
```slithir
 _getClaimableBondShares(nodeOperatorId)
TMP_1539(uint256) = INTERNAL_CALL, CSAccounting._getClaimableBondShares(uint256)(nodeOperatorId_1)
RETURN TMP_1539
```
#### CSAccounting.getClaimableRewardsAndBondShares(uint256,uint256,bytes32[]) [EXTERNAL]
```slithir
FEE_DISTRIBUTOR_2(ICSFeeDistributor) := phi(['FEE_DISTRIBUTOR_3', 'FEE_DISTRIBUTOR_1', 'FEE_DISTRIBUTOR_0', 'FEE_DISTRIBUTOR_6'])
 feesToDistribute = FEE_DISTRIBUTOR.getFeesToDistribute(nodeOperatorId,cumulativeFeeShares,rewardsProof)
TMP_1540(uint256) = HIGH_LEVEL_CALL, dest:FEE_DISTRIBUTOR_2(ICSFeeDistributor), function:getFeesToDistribute, arguments:['nodeOperatorId_1', 'cumulativeFeeShares_1', 'rewardsProof_1']  
FEE_DISTRIBUTOR_3(ICSFeeDistributor) := phi(['FEE_DISTRIBUTOR_2', 'FEE_DISTRIBUTOR_1', 'FEE_DISTRIBUTOR_3', 'FEE_DISTRIBUTOR_6'])
feesToDistribute_1(uint256) := TMP_1540(uint256)
 (current,required) = getBondSummaryShares(nodeOperatorId)
TUPLE_6(uint256,uint256) = INTERNAL_CALL, CSAccounting.getBondSummaryShares(uint256)(nodeOperatorId_1)
current_1(uint256)= UNPACK TUPLE_6 index: 0 
required_1(uint256)= UNPACK TUPLE_6 index: 1 
 current = current + feesToDistribute
TMP_1541(uint256) = current_1 (c)+ feesToDistribute_1
current_2(uint256) := TMP_1541(uint256)
 current > required
TMP_1542(bool) = current_2 > required_1
CONDITION TMP_1542
 current - required
TMP_1543(uint256) = current_2 (c)- required_1
RETURN TMP_1543
 0
RETURN 0
 claimableShares
```
#### CSAccounting.depositWstETH(uint256,uint256,ICSAccounting.PermitInput) [EXTERNAL]
```slithir
MODULE_18(ICSModule) := phi(['MODULE_0', 'MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSAccounting._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
MODULE_20(ICSModule) := phi(['MODULE_55'])
 _unwrapWstETHPermitIfRequired(msg.sender,permit)
INTERNAL_CALL, CSAccounting._unwrapWstETHPermitIfRequired(address,ICSAccounting.PermitInput)(msg.sender,permit_1)
 CSBondCore._depositWstETH(msg.sender,nodeOperatorId,wstETHAmount)
INTERNAL_CALL, CSBondCore._depositWstETH(address,uint256,uint256)(msg.sender,nodeOperatorId_1,wstETHAmount_1)
 MODULE.updateDepositableValidatorsCount(nodeOperatorId)
HIGH_LEVEL_CALL, dest:MODULE_22(ICSModule), function:updateDepositableValidatorsCount, arguments:['nodeOperatorId_1']  
MODULE_23(ICSModule) := phi(['MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_22', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
```
#### CSAccounting.depositStETH(uint256,uint256,ICSAccounting.PermitInput) [EXTERNAL]
```slithir
MODULE_12(ICSModule) := phi(['MODULE_0', 'MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSAccounting._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
MODULE_14(ICSModule) := phi(['MODULE_55'])
 _unwrapStETHPermitIfRequired(msg.sender,permit)
INTERNAL_CALL, CSAccounting._unwrapStETHPermitIfRequired(address,ICSAccounting.PermitInput)(msg.sender,permit_1)
 CSBondCore._depositStETH(msg.sender,nodeOperatorId,stETHAmount)
INTERNAL_CALL, CSBondCore._depositStETH(address,uint256,uint256)(msg.sender,nodeOperatorId_1,stETHAmount_1)
 MODULE.updateDepositableValidatorsCount(nodeOperatorId)
HIGH_LEVEL_CALL, dest:MODULE_16(ICSModule), function:updateDepositableValidatorsCount, arguments:['nodeOperatorId_1']  
MODULE_17(ICSModule) := phi(['MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_16', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
```
#### CSAccounting.depositETH(uint256) [EXTERNAL]
```slithir
MODULE_7(ICSModule) := phi(['MODULE_0', 'MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSAccounting._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
MODULE_9(ICSModule) := phi(['MODULE_55'])
 CSBondCore._depositETH(msg.sender,nodeOperatorId)
INTERNAL_CALL, CSBondCore._depositETH(address,uint256)(msg.sender,nodeOperatorId_1)
 MODULE.updateDepositableValidatorsCount(nodeOperatorId)
HIGH_LEVEL_CALL, dest:MODULE_10(ICSModule), function:updateDepositableValidatorsCount, arguments:['nodeOperatorId_1']  
MODULE_11(ICSModule) := phi(['MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_10', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
```
#### CSAccounting.claimRewardsStETH(uint256,uint256,uint256,bytes32[]) [EXTERNAL]
```slithir
MODULE_24(ICSModule) := phi(['MODULE_0', 'MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 no = MODULE.getNodeOperatorManagementProperties(nodeOperatorId)
TMP_1471(NodeOperatorManagementProperties) = HIGH_LEVEL_CALL, dest:MODULE_25(ICSModule), function:getNodeOperatorManagementProperties, arguments:['nodeOperatorId_1']  
MODULE_26(ICSModule) := phi(['MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_25', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
no_1(NodeOperatorManagementProperties) := TMP_1471(NodeOperatorManagementProperties)
 _onlyNodeOperatorManagerOrRewardAddresses(no)
INTERNAL_CALL, CSAccounting._onlyNodeOperatorManagerOrRewardAddresses(NodeOperatorManagementProperties)(no_1)
 rewardsProof.length != 0
REF_522 -> LENGTH rewardsProof_1
TMP_1473(bool) = REF_522 != 0
CONDITION TMP_1473
 _pullFeeRewards(nodeOperatorId,cumulativeFeeShares,rewardsProof)
INTERNAL_CALL, CSAccounting._pullFeeRewards(uint256,uint256,bytes32[])(nodeOperatorId_1,cumulativeFeeShares_1,rewardsProof_1)
 claimedShares = CSBondCore._claimStETH(nodeOperatorId,stETHAmount,no.rewardAddress)
REF_524(address) -> no_1.rewardAddress
TMP_1475(uint256) = INTERNAL_CALL, CSBondCore._claimStETH(uint256,uint256,address)(nodeOperatorId_1,stETHAmount_1,REF_524)
claimedShares_1(uint256) := TMP_1475(uint256)
 MODULE.updateDepositableValidatorsCount(nodeOperatorId)
HIGH_LEVEL_CALL, dest:MODULE_29(ICSModule), function:updateDepositableValidatorsCount, arguments:['nodeOperatorId_1']  
MODULE_30(ICSModule) := phi(['MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_29', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
 claimedShares
RETURN claimedShares_1
```
#### CSAccounting.claimRewardsWstETH(uint256,uint256,uint256,bytes32[]) [EXTERNAL]
```slithir
MODULE_31(ICSModule) := phi(['MODULE_0', 'MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 no = MODULE.getNodeOperatorManagementProperties(nodeOperatorId)
TMP_1478(NodeOperatorManagementProperties) = HIGH_LEVEL_CALL, dest:MODULE_32(ICSModule), function:getNodeOperatorManagementProperties, arguments:['nodeOperatorId_1']  
MODULE_33(ICSModule) := phi(['MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_32', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
no_1(NodeOperatorManagementProperties) := TMP_1478(NodeOperatorManagementProperties)
 _onlyNodeOperatorManagerOrRewardAddresses(no)
INTERNAL_CALL, CSAccounting._onlyNodeOperatorManagerOrRewardAddresses(NodeOperatorManagementProperties)(no_1)
 rewardsProof.length != 0
REF_527 -> LENGTH rewardsProof_1
TMP_1480(bool) = REF_527 != 0
CONDITION TMP_1480
 _pullFeeRewards(nodeOperatorId,cumulativeFeeShares,rewardsProof)
INTERNAL_CALL, CSAccounting._pullFeeRewards(uint256,uint256,bytes32[])(nodeOperatorId_1,cumulativeFeeShares_1,rewardsProof_1)
 claimedWstETH = CSBondCore._claimWstETH(nodeOperatorId,wstETHAmount,no.rewardAddress)
REF_529(address) -> no_1.rewardAddress
TMP_1482(uint256) = INTERNAL_CALL, CSBondCore._claimWstETH(uint256,uint256,address)(nodeOperatorId_1,wstETHAmount_1,REF_529)
claimedWstETH_1(uint256) := TMP_1482(uint256)
 MODULE.updateDepositableValidatorsCount(nodeOperatorId)
HIGH_LEVEL_CALL, dest:MODULE_36(ICSModule), function:updateDepositableValidatorsCount, arguments:['nodeOperatorId_1']  
MODULE_37(ICSModule) := phi(['MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_36', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
 claimedWstETH
RETURN claimedWstETH_1
```
#### CSAccounting.claimRewardsUnstETH(uint256,uint256,uint256,bytes32[]) [EXTERNAL]
```slithir
MODULE_38(ICSModule) := phi(['MODULE_0', 'MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 no = MODULE.getNodeOperatorManagementProperties(nodeOperatorId)
TMP_1485(NodeOperatorManagementProperties) = HIGH_LEVEL_CALL, dest:MODULE_39(ICSModule), function:getNodeOperatorManagementProperties, arguments:['nodeOperatorId_1']  
MODULE_40(ICSModule) := phi(['MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_39', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
no_1(NodeOperatorManagementProperties) := TMP_1485(NodeOperatorManagementProperties)
 _onlyNodeOperatorManagerOrRewardAddresses(no)
INTERNAL_CALL, CSAccounting._onlyNodeOperatorManagerOrRewardAddresses(NodeOperatorManagementProperties)(no_1)
 rewardsProof.length != 0
REF_532 -> LENGTH rewardsProof_1
TMP_1487(bool) = REF_532 != 0
CONDITION TMP_1487
 _pullFeeRewards(nodeOperatorId,cumulativeFeeShares,rewardsProof)
INTERNAL_CALL, CSAccounting._pullFeeRewards(uint256,uint256,bytes32[])(nodeOperatorId_1,cumulativeFeeShares_1,rewardsProof_1)
 requestId = CSBondCore._claimUnstETH(nodeOperatorId,stETHAmount,no.rewardAddress)
REF_534(address) -> no_1.rewardAddress
TMP_1489(uint256) = INTERNAL_CALL, CSBondCore._claimUnstETH(uint256,uint256,address)(nodeOperatorId_1,stETHAmount_1,REF_534)
requestId_1(uint256) := TMP_1489(uint256)
 MODULE.updateDepositableValidatorsCount(nodeOperatorId)
HIGH_LEVEL_CALL, dest:MODULE_43(ICSModule), function:updateDepositableValidatorsCount, arguments:['nodeOperatorId_1']  
MODULE_44(ICSModule) := phi(['MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_43', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
 requestId
RETURN requestId_1
```
#### CSAccounting.lockBondETH(uint256,uint256) [EXTERNAL]
```slithir
 CSBondLock._lock(nodeOperatorId,amount)
INTERNAL_CALL, CSBondLock._lock(uint256,uint256)(nodeOperatorId_1,amount_1)
 onlyModule()
MODIFIER_CALL, CSAccounting.onlyModule()()
```
#### CSAccounting.releaseLockedBondETH(uint256,uint256) [EXTERNAL]
```slithir
 CSBondLock._reduceAmount(nodeOperatorId,amount)
INTERNAL_CALL, CSBondLock._reduceAmount(uint256,uint256)(nodeOperatorId_1,amount_1)
 onlyModule()
MODIFIER_CALL, CSAccounting.onlyModule()()
```
#### CSAccounting.settleLockedBondETH(uint256) [EXTERNAL]
```slithir
 applied = false
applied_1(bool) := False(bool)
 lockedAmount = CSBondLock.getActualLockedBond(nodeOperatorId)
TMP_1502(uint256) = INTERNAL_CALL, CSBondLock.getActualLockedBond(uint256)(nodeOperatorId_1)
lockedAmount_1(uint256) := TMP_1502(uint256)
 lockedAmount > 0
TMP_1503(bool) = lockedAmount_1 > 0
CONDITION TMP_1503
 CSBondCore._burn(nodeOperatorId,lockedAmount)
INTERNAL_CALL, CSBondCore._burn(uint256,uint256)(nodeOperatorId_1,lockedAmount_1)
 CSBondLock._remove(nodeOperatorId)
INTERNAL_CALL, CSBondLock._remove(uint256)(nodeOperatorId_1)
 applied = true
applied_2(bool) := True(bool)
applied_3(bool) := phi(['applied_2', 'applied_1'])
 onlyModule()
MODIFIER_CALL, CSAccounting.onlyModule()()
 applied
RETURN applied_3
```
#### CSAccounting.compensateLockedBondETH(uint256) [EXTERNAL]
```slithir
LIDO_LOCATOR_20(ILidoLocator) := phi(['LIDO_LOCATOR_26', 'LIDO_LOCATOR_3', 'LIDO_LOCATOR_0', 'LIDO_LOCATOR_23', 'LIDO_LOCATOR_19', 'LIDO_LOCATOR_8', 'LIDO_LOCATOR_6'])
 (success,None) = LIDO_LOCATOR.elRewardsVault().call{value: msg.value}()
TMP_1496(address) = HIGH_LEVEL_CALL, dest:LIDO_LOCATOR_21(ILidoLocator), function:elRewardsVault, arguments:[]  
LIDO_LOCATOR_22(ILidoLocator) := phi(['LIDO_LOCATOR_26', 'LIDO_LOCATOR_3', 'LIDO_LOCATOR_23', 'LIDO_LOCATOR_19', 'LIDO_LOCATOR_21', 'LIDO_LOCATOR_8', 'LIDO_LOCATOR_6'])
TUPLE_5(bool,bytes) = LOW_LEVEL_CALL, dest:TMP_1496, function:call, arguments:[''] value:msg.value 
LIDO_LOCATOR_23(ILidoLocator) := phi(['LIDO_LOCATOR_22', 'LIDO_LOCATOR_26', 'LIDO_LOCATOR_3', 'LIDO_LOCATOR_23', 'LIDO_LOCATOR_19', 'LIDO_LOCATOR_8', 'LIDO_LOCATOR_6'])
success_1(bool)= UNPACK TUPLE_5 index: 0 
 ! success
TMP_1497 = UnaryType.BANG success_1 
CONDITION TMP_1497
 revert ElRewardsVaultReceiveFailed()()
TMP_1498(None) = SOLIDITY_CALL revert ElRewardsVaultReceiveFailed()()
 CSBondLock._reduceAmount(nodeOperatorId,msg.value)
INTERNAL_CALL, CSBondLock._reduceAmount(uint256,uint256)(nodeOperatorId_1,msg.value)
 BondLockCompensated(nodeOperatorId,msg.value)
Emit BondLockCompensated(nodeOperatorId_1,msg.value)
 onlyModule()
MODIFIER_CALL, CSAccounting.onlyModule()()
```
#### CSAccounting.setBondCurve(uint256,uint256) [EXTERNAL]
```slithir
SET_BOND_CURVE_ROLE_1(bytes32) := phi(['SET_BOND_CURVE_ROLE_2', 'SET_BOND_CURVE_ROLE_0'])
MODULE_2(ICSModule) := phi(['MODULE_0', 'MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSAccounting._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
MODULE_4(ICSModule) := phi(['MODULE_55'])
 CSBondCurve._setBondCurve(nodeOperatorId,curveId)
INTERNAL_CALL, CSBondCurve._setBondCurve(uint256,uint256)(nodeOperatorId_1,curveId_1)
 MODULE.updateDepositableValidatorsCount(nodeOperatorId)
HIGH_LEVEL_CALL, dest:MODULE_5(ICSModule), function:updateDepositableValidatorsCount, arguments:['nodeOperatorId_1']  
MODULE_6(ICSModule) := phi(['MODULE_5', 'MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 onlyRole(SET_BOND_CURVE_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(SET_BOND_CURVE_ROLE_1)
```
#### CSAccounting.penalize(uint256,uint256) [EXTERNAL]
```slithir
 CSBondCore._burn(nodeOperatorId,amount)
INTERNAL_CALL, CSBondCore._burn(uint256,uint256)(nodeOperatorId_1,amount_1)
 onlyModule()
MODIFIER_CALL, CSAccounting.onlyModule()()
```
#### CSAccounting.chargeFee(uint256,uint256) [EXTERNAL]
```slithir
chargePenaltyRecipient_1(address) := phi(['chargePenaltyRecipient_4', 'chargePenaltyRecipient_0', 'chargePenaltyRecipient_3'])
 CSBondCore._charge(nodeOperatorId,amount,chargePenaltyRecipient)
INTERNAL_CALL, CSBondCore._charge(uint256,uint256,address)(nodeOperatorId_1,amount_1,chargePenaltyRecipient_2)
 onlyModule()
MODIFIER_CALL, CSAccounting.onlyModule()()
```
#### CSAccounting.pullFeeRewards(uint256,uint256,bytes32[]) [EXTERNAL]
```slithir
MODULE_45(ICSModule) := phi(['MODULE_0', 'MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 _onlyExistingNodeOperator(nodeOperatorId)
INTERNAL_CALL, CSAccounting._onlyExistingNodeOperator(uint256)(nodeOperatorId_1)
MODULE_46(ICSModule) := phi(['MODULE_55'])
 _pullFeeRewards(nodeOperatorId,cumulativeFeeShares,rewardsProof)
INTERNAL_CALL, CSAccounting._pullFeeRewards(uint256,uint256,bytes32[])(nodeOperatorId_1,cumulativeFeeShares_1,rewardsProof_1)
 MODULE.updateDepositableValidatorsCount(nodeOperatorId)
HIGH_LEVEL_CALL, dest:MODULE_47(ICSModule), function:updateDepositableValidatorsCount, arguments:['nodeOperatorId_1']  
MODULE_48(ICSModule) := phi(['MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_47', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
```
#### CSAccounting.renewBurnerAllowance() [EXTERNAL]
```slithir
LIDO_LOCATOR_24(ILidoLocator) := phi(['LIDO_LOCATOR_26', 'LIDO_LOCATOR_3', 'LIDO_LOCATOR_0', 'LIDO_LOCATOR_23', 'LIDO_LOCATOR_19', 'LIDO_LOCATOR_8', 'LIDO_LOCATOR_6'])
LIDO_57(ILido) := phi(['LIDO_62', 'LIDO_56', 'LIDO_0', 'LIDO_31', 'LIDO_34', 'LIDO_37', 'LIDO_59', 'LIDO_52', 'LIDO_61', 'LIDO_3', 'LIDO_25', 'LIDO_39', 'LIDO_50', 'LIDO_35', 'LIDO_19', 'LIDO_1', 'LIDO_6', 'LIDO_11'])
 LIDO.approve(LIDO_LOCATOR.burner(),type()(uint256).max)
TMP_1526(address) = HIGH_LEVEL_CALL, dest:LIDO_LOCATOR_24(ILidoLocator), function:burner, arguments:[]  
LIDO_LOCATOR_25(ILidoLocator) := phi(['LIDO_LOCATOR_26', 'LIDO_LOCATOR_3', 'LIDO_LOCATOR_24', 'LIDO_LOCATOR_23', 'LIDO_LOCATOR_19', 'LIDO_LOCATOR_8', 'LIDO_LOCATOR_6'])
LIDO_58(ILido) := phi(['LIDO_62', 'LIDO_56', 'LIDO_31', 'LIDO_34', 'LIDO_59', 'LIDO_37', 'LIDO_61', 'LIDO_52', 'LIDO_3', 'LIDO_25', 'LIDO_39', 'LIDO_50', 'LIDO_35', 'LIDO_19', 'LIDO_1', 'LIDO_6', 'LIDO_11', 'LIDO_57'])
TMP_1528(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_1529(bool) = HIGH_LEVEL_CALL, dest:LIDO_58(ILido), function:approve, arguments:['TMP_1526', 'TMP_1528']  
LIDO_LOCATOR_26(ILidoLocator) := phi(['LIDO_LOCATOR_26', 'LIDO_LOCATOR_3', 'LIDO_LOCATOR_23', 'LIDO_LOCATOR_25', 'LIDO_LOCATOR_19', 'LIDO_LOCATOR_8', 'LIDO_LOCATOR_6'])
LIDO_59(ILido) := phi(['LIDO_62', 'LIDO_56', 'LIDO_31', 'LIDO_34', 'LIDO_59', 'LIDO_37', 'LIDO_61', 'LIDO_52', 'LIDO_3', 'LIDO_58', 'LIDO_25', 'LIDO_39', 'LIDO_50', 'LIDO_35', 'LIDO_19', 'LIDO_1', 'LIDO_6', 'LIDO_11'])
```
#### CSAccounting.initialize(ICSBondCurve.BondCurveIntervalInput[],address,uint256,address) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_10'])
LIDO_LOCATOR_9(ILidoLocator) := phi(['LIDO_LOCATOR_26', 'LIDO_LOCATOR_3', 'LIDO_LOCATOR_0', 'LIDO_LOCATOR_23', 'LIDO_LOCATOR_19', 'LIDO_LOCATOR_8', 'LIDO_LOCATOR_6'])
LIDO_40(ILido) := phi(['LIDO_62', 'LIDO_56', 'LIDO_0', 'LIDO_31', 'LIDO_34', 'LIDO_37', 'LIDO_59', 'LIDO_52', 'LIDO_61', 'LIDO_3', 'LIDO_25', 'LIDO_39', 'LIDO_50', 'LIDO_35', 'LIDO_19', 'LIDO_1', 'LIDO_6', 'LIDO_11'])
WITHDRAWAL_QUEUE_10(IWithdrawalQueue) := phi(['WITHDRAWAL_QUEUE_18', 'WITHDRAWAL_QUEUE_2', 'WITHDRAWAL_QUEUE_0', 'WITHDRAWAL_QUEUE_9'])
WSTETH_14(IWstETH) := phi(['WSTETH_21', 'WSTETH_5', 'WSTETH_24', 'WSTETH_0', 'WSTETH_1', 'WSTETH_13', 'WSTETH_23'])
 __AccessControlEnumerable_init()
INTERNAL_CALL, AccessControlEnumerableUpgradeable.__AccessControlEnumerable_init()()
 __CSBondCurve_init(bondCurve)
INTERNAL_CALL, CSBondCurve.__CSBondCurve_init(ICSBondCurve.BondCurveIntervalInput[])(bondCurve_1)
 __CSBondLock_init(bondLockPeriod)
INTERNAL_CALL, CSBondLock.__CSBondLock_init(uint256)(bondLockPeriod_1)
 admin == address(0)
TMP_1405 = CONVERT 0 to address
TMP_1406(bool) = admin_1 == TMP_1405
CONDITION TMP_1406
 revert ZeroAdminAddress()()
TMP_1407(None) = SOLIDITY_CALL revert ZeroAdminAddress()()
 _grantRole(DEFAULT_ADMIN_ROLE,admin)
TMP_1408(bool) = INTERNAL_CALL, AccessControlEnumerableUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_5,admin_1)
 _setChargePenaltyRecipient(_chargePenaltyRecipient)
INTERNAL_CALL, CSAccounting._setChargePenaltyRecipient(address)(_chargePenaltyRecipient_1)
 LIDO.approve(address(WSTETH),type()(uint256).max)
TMP_1410 = CONVERT WSTETH_20 to address
TMP_1412(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_1413(bool) = HIGH_LEVEL_CALL, dest:LIDO_46(ILido), function:approve, arguments:['TMP_1410', 'TMP_1412']  
LIDO_LOCATOR_16(ILidoLocator) := phi(['LIDO_LOCATOR_15', 'LIDO_LOCATOR_26', 'LIDO_LOCATOR_3', 'LIDO_LOCATOR_23', 'LIDO_LOCATOR_19', 'LIDO_LOCATOR_8', 'LIDO_LOCATOR_6'])
LIDO_47(ILido) := phi(['LIDO_62', 'LIDO_56', 'LIDO_46', 'LIDO_31', 'LIDO_34', 'LIDO_59', 'LIDO_37', 'LIDO_61', 'LIDO_52', 'LIDO_3', 'LIDO_25', 'LIDO_39', 'LIDO_50', 'LIDO_35', 'LIDO_19', 'LIDO_1', 'LIDO_6', 'LIDO_11'])
WITHDRAWAL_QUEUE_17(IWithdrawalQueue) := phi(['WITHDRAWAL_QUEUE_18', 'WITHDRAWAL_QUEUE_2', 'WITHDRAWAL_QUEUE_16', 'WITHDRAWAL_QUEUE_9'])
WSTETH_21(IWstETH) := phi(['WSTETH_21', 'WSTETH_5', 'WSTETH_24', 'WSTETH_1', 'WSTETH_13', 'WSTETH_20', 'WSTETH_23'])
 LIDO.approve(address(WITHDRAWAL_QUEUE),type()(uint256).max)
TMP_1414 = CONVERT WITHDRAWAL_QUEUE_17 to address
TMP_1416(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_1417(bool) = HIGH_LEVEL_CALL, dest:LIDO_47(ILido), function:approve, arguments:['TMP_1414', 'TMP_1416']  
LIDO_LOCATOR_17(ILidoLocator) := phi(['LIDO_LOCATOR_26', 'LIDO_LOCATOR_3', 'LIDO_LOCATOR_23', 'LIDO_LOCATOR_19', 'LIDO_LOCATOR_16', 'LIDO_LOCATOR_8', 'LIDO_LOCATOR_6'])
LIDO_48(ILido) := phi(['LIDO_62', 'LIDO_56', 'LIDO_31', 'LIDO_34', 'LIDO_59', 'LIDO_37', 'LIDO_61', 'LIDO_52', 'LIDO_3', 'LIDO_25', 'LIDO_39', 'LIDO_50', 'LIDO_47', 'LIDO_35', 'LIDO_19', 'LIDO_1', 'LIDO_6', 'LIDO_11'])
WITHDRAWAL_QUEUE_18(IWithdrawalQueue) := phi(['WITHDRAWAL_QUEUE_18', 'WITHDRAWAL_QUEUE_2', 'WITHDRAWAL_QUEUE_17', 'WITHDRAWAL_QUEUE_9'])
 LIDO.approve(LIDO_LOCATOR.burner(),type()(uint256).max)
TMP_1418(address) = HIGH_LEVEL_CALL, dest:LIDO_LOCATOR_17(ILidoLocator), function:burner, arguments:[]  
LIDO_LOCATOR_18(ILidoLocator) := phi(['LIDO_LOCATOR_26', 'LIDO_LOCATOR_3', 'LIDO_LOCATOR_23', 'LIDO_LOCATOR_19', 'LIDO_LOCATOR_17', 'LIDO_LOCATOR_8', 'LIDO_LOCATOR_6'])
LIDO_49(ILido) := phi(['LIDO_62', 'LIDO_48', 'LIDO_56', 'LIDO_31', 'LIDO_34', 'LIDO_59', 'LIDO_37', 'LIDO_61', 'LIDO_52', 'LIDO_3', 'LIDO_25', 'LIDO_39', 'LIDO_50', 'LIDO_35', 'LIDO_19', 'LIDO_1', 'LIDO_6', 'LIDO_11'])
TMP_1420(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_1421(bool) = HIGH_LEVEL_CALL, dest:LIDO_49(ILido), function:approve, arguments:['TMP_1418', 'TMP_1420']  
LIDO_LOCATOR_19(ILidoLocator) := phi(['LIDO_LOCATOR_26', 'LIDO_LOCATOR_3', 'LIDO_LOCATOR_23', 'LIDO_LOCATOR_19', 'LIDO_LOCATOR_18', 'LIDO_LOCATOR_8', 'LIDO_LOCATOR_6'])
LIDO_50(ILido) := phi(['LIDO_62', 'LIDO_56', 'LIDO_31', 'LIDO_34', 'LIDO_59', 'LIDO_37', 'LIDO_61', 'LIDO_52', 'LIDO_49', 'LIDO_3', 'LIDO_25', 'LIDO_39', 'LIDO_50', 'LIDO_35', 'LIDO_19', 'LIDO_1', 'LIDO_6', 'LIDO_11'])
 reinitializer(2)
MODIFIER_CALL, Initializable.reinitializer(uint64)(2)
```
#### CSAccounting.finalizeUpgradeV2(ICSBondCurve.BondCurveIntervalInput[][]) [EXTERNAL]
```slithir
 sstore(uint256,uint256)(_feeDistributorOld,0x00)
_feeDistributorOld_1(ICSFeeDistributor) := 0(uint256)
 bondCurvesInputs.length != _getLegacyBondCurvesLength()
REF_503 -> LENGTH bondCurvesInputs_1
TMP_1423(uint256) = INTERNAL_CALL, CSBondCurve._getLegacyBondCurvesLength()()
TMP_1424(bool) = REF_503 != TMP_1423
CONDITION TMP_1424
 revert InvalidBondCurvesLength()()
TMP_1425(None) = SOLIDITY_CALL revert InvalidBondCurvesLength()()
 __CSBondCurve_init(bondCurvesInputs[0])
REF_504(ICSBondCurve.BondCurveIntervalInput[]) -> bondCurvesInputs_1[0]
INTERNAL_CALL, CSBondCurve.__CSBondCurve_init(ICSBondCurve.BondCurveIntervalInput[])(REF_504)
 i = 1
i_1(uint256) := 1(uint256)
 i < bondCurvesInputs.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_505 -> LENGTH bondCurvesInputs_1
TMP_1427(bool) = i_2 < REF_505
CONDITION TMP_1427
 _addBondCurve(bondCurvesInputs[i])
REF_506(ICSBondCurve.BondCurveIntervalInput[]) -> bondCurvesInputs_1[i_2]
TMP_1428(uint256) = INTERNAL_CALL, CSBondCurve._addBondCurve(ICSBondCurve.BondCurveIntervalInput[])(REF_506)
 ++ i
i_3(uint256) = i_2 (c)+ 1
 reinitializer(2)
MODIFIER_CALL, Initializable.reinitializer(uint64)(2)
```
#### CSAccounting.recoverStETHShares() [EXTERNAL]
```slithir
LIDO_53(ILido) := phi(['LIDO_62', 'LIDO_56', 'LIDO_0', 'LIDO_31', 'LIDO_34', 'LIDO_37', 'LIDO_59', 'LIDO_52', 'LIDO_61', 'LIDO_3', 'LIDO_25', 'LIDO_39', 'LIDO_50', 'LIDO_35', 'LIDO_19', 'LIDO_1', 'LIDO_6', 'LIDO_11'])
 _onlyRecoverer()
INTERNAL_CALL, CSAccounting._onlyRecoverer()()
 shares = LIDO.sharesOf(address(this)) - totalBondShares()
TMP_1520 = CONVERT this to address
TMP_1521(uint256) = HIGH_LEVEL_CALL, dest:LIDO_54(ILido), function:sharesOf, arguments:['TMP_1520']  
LIDO_55(ILido) := phi(['LIDO_62', 'LIDO_54', 'LIDO_56', 'LIDO_31', 'LIDO_34', 'LIDO_59', 'LIDO_37', 'LIDO_61', 'LIDO_52', 'LIDO_3', 'LIDO_25', 'LIDO_39', 'LIDO_50', 'LIDO_35', 'LIDO_19', 'LIDO_1', 'LIDO_6', 'LIDO_11'])
TMP_1522(uint256) = INTERNAL_CALL, CSBondCore.totalBondShares()()
TMP_1523(uint256) = TMP_1521 (c)- TMP_1522
shares_1(uint256) := TMP_1523(uint256)
 AssetRecovererLib.recoverStETHShares(address(LIDO),shares)
TMP_1524 = CONVERT LIDO_56 to address
LIBRARY_CALL, dest:AssetRecovererLib, function:AssetRecovererLib.recoverStETHShares(address,uint256), arguments:['TMP_1524', 'shares_1']
```
#### CSAccounting._pullFeeRewards(uint256,uint256,bytes32[]) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1'])
cumulativeFeeShares_1(uint256) := phi(['cumulativeFeeShares_1', 'cumulativeFeeShares_1', 'cumulativeFeeShares_1', 'cumulativeFeeShares_1'])
rewardsProof_1(bytes32[]) := phi(['rewardsProof_1', 'rewardsProof_1', 'rewardsProof_1', 'rewardsProof_1'])
FEE_DISTRIBUTOR_5(ICSFeeDistributor) := phi(['FEE_DISTRIBUTOR_3', 'FEE_DISTRIBUTOR_1', 'FEE_DISTRIBUTOR_0', 'FEE_DISTRIBUTOR_6'])
 distributed = FEE_DISTRIBUTOR.distributeFees(nodeOperatorId,cumulativeFeeShares,rewardsProof)
TMP_1550(uint256) = HIGH_LEVEL_CALL, dest:FEE_DISTRIBUTOR_5(ICSFeeDistributor), function:distributeFees, arguments:['nodeOperatorId_1', 'cumulativeFeeShares_1', 'rewardsProof_1']  
FEE_DISTRIBUTOR_6(ICSFeeDistributor) := phi(['FEE_DISTRIBUTOR_5', 'FEE_DISTRIBUTOR_1', 'FEE_DISTRIBUTOR_3', 'FEE_DISTRIBUTOR_6'])
distributed_1(uint256) := TMP_1550(uint256)
 CSBondCore._increaseBond(nodeOperatorId,distributed)
INTERNAL_CALL, CSBondCore._increaseBond(uint256,uint256)(nodeOperatorId_1,distributed_1)
```
#### CSAccounting._unwrapStETHPermitIfRequired(address,ICSAccounting.PermitInput) [INTERNAL]
```slithir
from_1(address) := phi(['msg.sender', 'from_1'])
permit_1(ICSAccounting.PermitInput) := phi(['permit_1', 'permit_1'])
LIDO_60(ILido) := phi(['LIDO_62', 'LIDO_56', 'LIDO_0', 'LIDO_31', 'LIDO_34', 'LIDO_37', 'LIDO_59', 'LIDO_52', 'LIDO_61', 'LIDO_3', 'LIDO_25', 'LIDO_39', 'LIDO_50', 'LIDO_35', 'LIDO_19', 'LIDO_1', 'LIDO_6', 'LIDO_11'])
 permit.value > 0 && LIDO.allowance(from,address(this)) < permit.value
REF_559(uint256) -> permit_1.value
TMP_1552(bool) = REF_559 > 0
TMP_1553 = CONVERT this to address
TMP_1554(uint256) = HIGH_LEVEL_CALL, dest:LIDO_60(ILido), function:allowance, arguments:['from_1', 'TMP_1553']  
LIDO_61(ILido) := phi(['LIDO_62', 'LIDO_56', 'LIDO_31', 'LIDO_34', 'LIDO_59', 'LIDO_37', 'LIDO_61', 'LIDO_52', 'LIDO_3', 'LIDO_60', 'LIDO_25', 'LIDO_39', 'LIDO_50', 'LIDO_35', 'LIDO_19', 'LIDO_1', 'LIDO_6', 'LIDO_11'])
REF_561(uint256) -> permit_1.value
TMP_1555(bool) = TMP_1554 < REF_561
TMP_1556(bool) = TMP_1552 && TMP_1555
CONDITION TMP_1556
 LIDO.permit({owner:from,spender:address(this),value:permit.value,deadline:permit.deadline,v:permit.v,r:permit.r,s:permit.s})
TMP_1557 = CONVERT this to address
REF_563(uint256) -> permit_1.value
REF_564(uint256) -> permit_1.deadline
REF_565(uint8) -> permit_1.v
REF_566(bytes32) -> permit_1.r
REF_567(bytes32) -> permit_1.s
HIGH_LEVEL_CALL, dest:LIDO_61(ILido), function:permit, arguments:['from_1', 'TMP_1557', 'REF_563', 'REF_564', 'REF_565', 'REF_566', 'REF_567']  
LIDO_62(ILido) := phi(['LIDO_62', 'LIDO_56', 'LIDO_31', 'LIDO_34', 'LIDO_59', 'LIDO_37', 'LIDO_61', 'LIDO_52', 'LIDO_3', 'LIDO_25', 'LIDO_39', 'LIDO_50', 'LIDO_35', 'LIDO_19', 'LIDO_1', 'LIDO_6', 'LIDO_11'])
```
#### CSAccounting._unwrapWstETHPermitIfRequired(address,ICSAccounting.PermitInput) [INTERNAL]
```slithir
from_1(address) := phi(['msg.sender', 'from_1'])
permit_1(ICSAccounting.PermitInput) := phi(['permit_1', 'permit_1'])
WSTETH_22(IWstETH) := phi(['WSTETH_21', 'WSTETH_5', 'WSTETH_24', 'WSTETH_0', 'WSTETH_1', 'WSTETH_13', 'WSTETH_23'])
 permit.value > 0 && WSTETH.allowance(from,address(this)) < permit.value
REF_568(uint256) -> permit_1.value
TMP_1559(bool) = REF_568 > 0
TMP_1560 = CONVERT this to address
TMP_1561(uint256) = HIGH_LEVEL_CALL, dest:WSTETH_22(IWstETH), function:allowance, arguments:['from_1', 'TMP_1560']  
WSTETH_23(IWstETH) := phi(['WSTETH_21', 'WSTETH_5', 'WSTETH_22', 'WSTETH_24', 'WSTETH_1', 'WSTETH_13', 'WSTETH_23'])
REF_570(uint256) -> permit_1.value
TMP_1562(bool) = TMP_1561 < REF_570
TMP_1563(bool) = TMP_1559 && TMP_1562
CONDITION TMP_1563
 WSTETH.permit({owner:from,spender:address(this),value:permit.value,deadline:permit.deadline,v:permit.v,r:permit.r,s:permit.s})
TMP_1564 = CONVERT this to address
REF_572(uint256) -> permit_1.value
REF_573(uint256) -> permit_1.deadline
REF_574(uint8) -> permit_1.v
REF_575(bytes32) -> permit_1.r
REF_576(bytes32) -> permit_1.s
HIGH_LEVEL_CALL, dest:WSTETH_23(IWstETH), function:permit, arguments:['from_1', 'TMP_1564', 'REF_572', 'REF_573', 'REF_574', 'REF_575', 'REF_576']  
WSTETH_24(IWstETH) := phi(['WSTETH_21', 'WSTETH_5', 'WSTETH_24', 'WSTETH_1', 'WSTETH_13', 'WSTETH_23'])
```
#### CSAccounting._getRequiredBond(uint256,uint256) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1'])
additionalKeys_1(uint256) := phi(['additionalKeys_1', 'additionalKeys_1'])
MODULE_49(ICSModule) := phi(['MODULE_0', 'MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 curveId = CSBondCurve.getBondCurveId(nodeOperatorId)
TMP_1568(uint256) = INTERNAL_CALL, CSBondCurve.getBondCurveId(uint256)(nodeOperatorId_1)
curveId_1(uint256) := TMP_1568(uint256)
 nonWithdrawnKeys = MODULE.getNodeOperatorNonWithdrawnKeys(nodeOperatorId)
TMP_1569(uint256) = HIGH_LEVEL_CALL, dest:MODULE_50(ICSModule), function:getNodeOperatorNonWithdrawnKeys, arguments:['nodeOperatorId_1']  
MODULE_51(ICSModule) := phi(['MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_50', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
nonWithdrawnKeys_1(uint256) := TMP_1569(uint256)
 requiredBondForKeys = CSBondCurve.getBondAmountByKeysCount(nonWithdrawnKeys + additionalKeys,curveId)
TMP_1570(uint256) = nonWithdrawnKeys_1 (c)+ additionalKeys_1
TMP_1571(uint256) = INTERNAL_CALL, CSBondCurve.getBondAmountByKeysCount(uint256,uint256)(TMP_1570,curveId_1)
requiredBondForKeys_1(uint256) := TMP_1571(uint256)
 actualLockedBond = CSBondLock.getActualLockedBond(nodeOperatorId)
TMP_1572(uint256) = INTERNAL_CALL, CSBondLock.getActualLockedBond(uint256)(nodeOperatorId_1)
actualLockedBond_1(uint256) := TMP_1572(uint256)
 requiredBondForKeys + actualLockedBond
TMP_1573(uint256) = requiredBondForKeys_1 (c)+ actualLockedBond_1
RETURN TMP_1573
```
#### CSAccounting._getRequiredBondShares(uint256,uint256) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1'])
 _sharesByEth(_getRequiredBond(nodeOperatorId,additionalKeys))
TMP_1574(uint256) = INTERNAL_CALL, CSAccounting._getRequiredBond(uint256,uint256)(nodeOperatorId_1,additionalKeys_1)
TMP_1575(uint256) = INTERNAL_CALL, CSBondCore._sharesByEth(uint256)(TMP_1574)
RETURN TMP_1575
```
#### CSAccounting._getUnbondedKeysCount(uint256,bool) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1'])
MODULE_52(ICSModule) := phi(['MODULE_0', 'MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 nonWithdrawnKeys = MODULE.getNodeOperatorNonWithdrawnKeys(nodeOperatorId)
TMP_1576(uint256) = HIGH_LEVEL_CALL, dest:MODULE_52(ICSModule), function:getNodeOperatorNonWithdrawnKeys, arguments:['nodeOperatorId_1']  
MODULE_53(ICSModule) := phi(['MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_52', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
nonWithdrawnKeys_1(uint256) := TMP_1576(uint256)
 currentBond = CSBondCore.getBond(nodeOperatorId)
TMP_1577(uint256) = INTERNAL_CALL, CSBondCore.getBond(uint256)(nodeOperatorId_1)
currentBond_1(uint256) := TMP_1577(uint256)
 includeLockedBond
CONDITION includeLockedBond_1
 lockedBond = CSBondLock.getActualLockedBond(nodeOperatorId)
TMP_1578(uint256) = INTERNAL_CALL, CSBondLock.getActualLockedBond(uint256)(nodeOperatorId_1)
lockedBond_1(uint256) := TMP_1578(uint256)
 lockedBond > currentBond
TMP_1579(bool) = lockedBond_1 > currentBond_1
CONDITION TMP_1579
 nonWithdrawnKeys
RETURN nonWithdrawnKeys_1
 currentBond -= lockedBond
currentBond_2(uint256) = currentBond_1 - lockedBond_1
currentBond_3(uint256) := phi(['currentBond_1', 'currentBond_2'])
 bondedKeys = CSBondCurve.getKeysCountByBondAmount(currentBond + 10,CSBondCurve.getBondCurveId(nodeOperatorId))
TMP_1580(uint256) = currentBond_3 + 10
TMP_1581(uint256) = INTERNAL_CALL, CSBondCurve.getBondCurveId(uint256)(nodeOperatorId_1)
TMP_1582(uint256) = INTERNAL_CALL, CSBondCurve.getKeysCountByBondAmount(uint256,uint256)(TMP_1580,TMP_1581)
bondedKeys_1(uint256) := TMP_1582(uint256)
 nonWithdrawnKeys > bondedKeys
TMP_1583(bool) = nonWithdrawnKeys_1 > bondedKeys_1
CONDITION TMP_1583
 nonWithdrawnKeys - bondedKeys
TMP_1584(uint256) = nonWithdrawnKeys_1 - bondedKeys_1
RETURN TMP_1584
 0
RETURN 0
```
#### CSAccounting._onlyExistingNodeOperator(uint256) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1'])
MODULE_54(ICSModule) := phi(['MODULE_0', 'MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
 nodeOperatorId < IStakingModule(address(MODULE)).getNodeOperatorsCount()
TMP_1586 = CONVERT MODULE_54 to address
TMP_1587 = CONVERT TMP_1586 to IStakingModule
TMP_1588(uint256) = HIGH_LEVEL_CALL, dest:TMP_1587(IStakingModule), function:getNodeOperatorsCount, arguments:[]  
MODULE_55(ICSModule) := phi(['MODULE_54', 'MODULE_1', 'MODULE_30', 'MODULE_55', 'MODULE_37', 'MODULE_6', 'MODULE_51', 'MODULE_44', 'MODULE_48', 'MODULE_53', 'MODULE_11', 'MODULE_23', 'MODULE_17'])
TMP_1589(bool) = nodeOperatorId_1 < TMP_1588
CONDITION TMP_1589
 revert NodeOperatorDoesNotExist()()
TMP_1590(None) = SOLIDITY_CALL revert NodeOperatorDoesNotExist()()
```
#### CSAccounting._onlyNodeOperatorManagerOrRewardAddresses(NodeOperatorManagementProperties) [INTERNAL]
```slithir
no_1(NodeOperatorManagementProperties) := phi(['no_1', 'no_1', 'no_1'])
 no.managerAddress == address(0)
REF_587(address) -> no_1.managerAddress
TMP_1591 = CONVERT 0 to address
TMP_1592(bool) = REF_587 == TMP_1591
CONDITION TMP_1592
 revert NodeOperatorDoesNotExist()()
TMP_1593(None) = SOLIDITY_CALL revert NodeOperatorDoesNotExist()()
 no.managerAddress == msg.sender || no.rewardAddress == msg.sender
REF_588(address) -> no_1.managerAddress
TMP_1594(bool) = REF_588 == msg.sender
REF_589(address) -> no_1.rewardAddress
TMP_1595(bool) = REF_589 == msg.sender
TMP_1596(bool) = TMP_1594 || TMP_1595
CONDITION TMP_1596
 revert SenderIsNotEligible()()
TMP_1597(None) = SOLIDITY_CALL revert SenderIsNotEligible()()
```
#### CSAccounting._setChargePenaltyRecipient(address) [PRIVATE]
```slithir
_chargePenaltyRecipient_1(address) := phi(['_chargePenaltyRecipient_1', '_chargePenaltyRecipient_1'])
 _chargePenaltyRecipient == address(0)
TMP_1598 = CONVERT 0 to address
TMP_1599(bool) = _chargePenaltyRecipient_1 == TMP_1598
CONDITION TMP_1599
 revert ZeroChargePenaltyRecipientAddress()()
TMP_1600(None) = SOLIDITY_CALL revert ZeroChargePenaltyRecipientAddress()()
 chargePenaltyRecipient = _chargePenaltyRecipient
chargePenaltyRecipient_4(address) := _chargePenaltyRecipient_1(address)
 ChargePenaltyRecipientSet(_chargePenaltyRecipient)
Emit ChargePenaltyRecipientSet(_chargePenaltyRecipient_1)
```

#### AssetRecovererLib.recoverERC20(address,uint256) [EXTERNAL]
```slithir
 IERC20(token).safeTransfer(msg.sender,amount)
TMP_4270 = CONVERT token_1 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_4270', 'msg.sender', 'amount_1'] 
 IAssetRecovererLib.ERC20Recovered(token,msg.sender,amount)
Emit ERC20Recovered(token_1,msg.sender,amount_1)
```
#### ICSFeeDistributor.getFeesToDistribute(uint256,uint256,bytes32[]) [EXTERNAL]
```slithir

```
#### ICSModule.updateDepositableValidatorsCount(uint256) [EXTERNAL]
```slithir

```
#### ICSModule.getNodeOperatorManagementProperties(uint256) [EXTERNAL]
```slithir

```
#### ILidoLocator.elRewardsVault() [EXTERNAL]
```slithir

```
#### ILidoLocator.burner() [EXTERNAL]
```slithir

```
#### AssetRecovererLib.recoverStETHShares(address,uint256) [EXTERNAL]
```slithir
 ILido(lido).transferShares(msg.sender,shares)
TMP_4273 = CONVERT lido_1 to ILido
TMP_4274(uint256) = HIGH_LEVEL_CALL, dest:TMP_4273(ILido), function:transferShares, arguments:['msg.sender', 'shares_1']  
 IAssetRecovererLib.StETHSharesRecovered(msg.sender,shares)
Emit StETHSharesRecovered(msg.sender,shares_1)
```
#### IStETH.sharesOf(address) [EXTERNAL]
```slithir

```
#### ICSFeeDistributor.distributeFees(uint256,uint256,bytes32[]) [EXTERNAL]
```slithir

```
#### IWstETH.permit(address,address,uint256,uint256,uint8,bytes32,bytes32) [EXTERNAL]
```slithir

```

#### ICSModule.getNodeOperatorNonWithdrawnKeys(uint256) [EXTERNAL]
```slithir

```
#### IStakingModule.getNodeOperatorsCount() [EXTERNAL]
```slithir

```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_63(transfer) -> token_1.transfer
TMP_150(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_63,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff980e4a00>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff980e56c0>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_150)
```
#### SafeERC20._callOptionalReturn(IERC20,bytes) [PRIVATE]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1', 'token_1'])
data_1(bytes) := phi(['TMP_167', 'TMP_152', 'approvalCall_1', 'TMP_150'])
 returndata = address(token).functionCall(data)
TMP_170 = CONVERT token_1 to address
TMP_171(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionCall(address,bytes), arguments:['TMP_170', 'data_1'] 
returndata_1(bytes) := TMP_171(bytes)
 returndata.length != 0 && ! abi.decode(returndata,(bool))
REF_73 -> LENGTH returndata_1
TMP_172(bool) = REF_73 != 0
TMP_173(bool) = SOLIDITY_CALL abi.decode()(returndata_1,bool)
TMP_174 = UnaryType.BANG TMP_173 
TMP_175(bool) = TMP_172 && TMP_174
CONDITION TMP_175
 revert SafeERC20FailedOperation(address)(address(token))
TMP_176 = CONVERT token_1 to address
TMP_177(None) = SOLIDITY_CALL revert SafeERC20FailedOperation(address)(TMP_176)
```
