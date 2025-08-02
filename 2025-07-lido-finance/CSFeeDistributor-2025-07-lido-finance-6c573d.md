
### Storage layout (CSFeeDistributor) 

```text
treeRoot bytes32
treeCid string
logCid string
distributedShares mapping(uint256 => uint256)
totalClaimableShares uint256
_distributionDataHistory mapping(uint256 => ICSFeeDistributor.DistributionData)
distributionDataHistoryCount uint256
rebateRecipient address

```



#### CSFeeDistributor.recoverERC20(address,uint256) [EXTERNAL]
```slithir
STETH_10(IStETH) := phi(['STETH_0', 'STETH_8', 'STETH_1', 'STETH_11', 'STETH_13', 'STETH_9', 'STETH_5', 'STETH_4'])
 _onlyRecoverer()
INTERNAL_CALL, CSFeeDistributor._onlyRecoverer()()
 token == address(STETH)
TMP_2020 = CONVERT STETH_11 to address
TMP_2021(bool) = token_1 == TMP_2020
CONDITION TMP_2021
 revert NotAllowedToRecover()()
TMP_2022(None) = SOLIDITY_CALL revert NotAllowedToRecover()()
 AssetRecovererLib.recoverERC20(token,amount)
LIBRARY_CALL, dest:AssetRecovererLib, function:AssetRecovererLib.recoverERC20(address,uint256), arguments:['token_1', 'amount_1']
```
#### CSFeeDistributor._onlyRecoverer() [INTERNAL]
```slithir
RECOVERER_ROLE_1(bytes32) := phi(['RECOVERER_ROLE_2', 'RECOVERER_ROLE_0'])
 _checkRole(RECOVERER_ROLE)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(RECOVERER_ROLE_1)
```
#### CSFeeDistributor.getInitializedVersion() [EXTERNAL]
```slithir
 _getInitializedVersion()
TMP_2024(uint64) = INTERNAL_CALL, Initializable._getInitializedVersion()()
RETURN TMP_2024
```
#### CSFeeDistributor.setRebateRecipient(address) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_6(bytes32) := phi(['DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_7', 'DEFAULT_ADMIN_ROLE_5'])
 _setRebateRecipient(_rebateRecipient)
INTERNAL_CALL, CSFeeDistributor._setRebateRecipient(address)(_rebateRecipient_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_6)
```
#### CSFeeDistributor.getFeesToDistribute(uint256,uint256,bytes32[]) [EXTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1'])
cumulativeFeeShares_1(uint256) := phi(['cumulativeFeeShares_1'])
proof_1(bytes32[]) := phi(['proof_1'])
treeRoot_6(bytes32) := phi(['treeRoot_5', 'treeRoot_7', 'treeRoot_3', 'treeRoot_0'])
distributedShares_5(mapping(uint256 => uint256)) := phi(['distributedShares_0', 'distributedShares_3', 'distributedShares_4', 'distributedShares_6'])
 proof.length == 0
REF_742 -> LENGTH proof_1
TMP_2028(bool) = REF_742 == 0
CONDITION TMP_2028
 revert InvalidProof()()
TMP_2029(None) = SOLIDITY_CALL revert InvalidProof()()
 isValid = MerkleProof.verifyCalldata(proof,treeRoot,hashLeaf(nodeOperatorId,cumulativeFeeShares))
TMP_2030(bytes32) = INTERNAL_CALL, CSFeeDistributor.hashLeaf(uint256,uint256)(nodeOperatorId_1,cumulativeFeeShares_1)
TMP_2031(bool) = LIBRARY_CALL, dest:MerkleProof, function:MerkleProof.verifyCalldata(bytes32[],bytes32,bytes32), arguments:['proof_1', 'treeRoot_7', 'TMP_2030'] 
isValid_1(bool) := TMP_2031(bool)
 ! isValid
TMP_2032 = UnaryType.BANG isValid_1 
CONDITION TMP_2032
 revert InvalidProof()()
TMP_2033(None) = SOLIDITY_CALL revert InvalidProof()()
 _distributedShares = distributedShares[nodeOperatorId]
REF_744(uint256) -> distributedShares_6[nodeOperatorId_1]
_distributedShares_1(uint256) := REF_744(uint256)
 _distributedShares > cumulativeFeeShares
TMP_2034(bool) = _distributedShares_1 > cumulativeFeeShares_1
CONDITION TMP_2034
 revert FeeSharesDecrease()()
TMP_2035(None) = SOLIDITY_CALL revert FeeSharesDecrease()()
 sharesToDistribute = cumulativeFeeShares - _distributedShares
TMP_2036(uint256) = cumulativeFeeShares_1 - _distributedShares_1
sharesToDistribute_1(uint256) := TMP_2036(uint256)
 sharesToDistribute
RETURN sharesToDistribute_1
```
#### CSFeeDistributor.distributeFees(uint256,uint256,bytes32[]) [EXTERNAL]
```slithir
STETH_2(IStETH) := phi(['STETH_0', 'STETH_8', 'STETH_1', 'STETH_11', 'STETH_13', 'STETH_9', 'STETH_5', 'STETH_4'])
ACCOUNTING_2(address) := phi(['ACCOUNTING_5', 'ACCOUNTING_4', 'ACCOUNTING_0', 'ACCOUNTING_1'])
distributedShares_1(mapping(uint256 => uint256)) := phi(['distributedShares_0', 'distributedShares_3', 'distributedShares_4', 'distributedShares_6'])
totalClaimableShares_1(uint256) := phi(['totalClaimableShares_3', 'totalClaimableShares_0', 'totalClaimableShares_4', 'totalClaimableShares_7', 'totalClaimableShares_10'])
 sharesToDistribute = getFeesToDistribute(nodeOperatorId,cumulativeFeeShares,proof)
TMP_1970(uint256) = INTERNAL_CALL, CSFeeDistributor.getFeesToDistribute(uint256,uint256,bytes32[])(nodeOperatorId_1,cumulativeFeeShares_1,proof_1)
distributedShares_3(mapping(uint256 => uint256)) := phi(['distributedShares_6'])
sharesToDistribute_1(uint256) := TMP_1970(uint256)
 sharesToDistribute == 0
TMP_1971(bool) = sharesToDistribute_1 == 0
CONDITION TMP_1971
 0
RETURN 0
 totalClaimableShares < sharesToDistribute
TMP_1972(bool) = totalClaimableShares_3 < sharesToDistribute_1
CONDITION TMP_1972
 revert NotEnoughShares()()
TMP_1973(None) = SOLIDITY_CALL revert NotEnoughShares()()
 totalClaimableShares -= sharesToDistribute
totalClaimableShares_4(uint256) = totalClaimableShares_3 - sharesToDistribute_1
 distributedShares[nodeOperatorId] += sharesToDistribute
REF_732(uint256) -> distributedShares_3[nodeOperatorId_1]
distributedShares_4(mapping(uint256 => uint256)) := phi(['distributedShares_3'])
REF_732(-> distributedShares_4) = REF_732 + sharesToDistribute_1
 STETH.transferShares(ACCOUNTING,sharesToDistribute)
TMP_1974(uint256) = HIGH_LEVEL_CALL, dest:STETH_4(IStETH), function:transferShares, arguments:['ACCOUNTING_4', 'sharesToDistribute_1']  
STETH_5(IStETH) := phi(['STETH_8', 'STETH_1', 'STETH_11', 'STETH_13', 'STETH_9', 'STETH_5', 'STETH_4'])
ACCOUNTING_5(address) := phi(['ACCOUNTING_5', 'ACCOUNTING_1', 'ACCOUNTING_4'])
 OperatorFeeDistributed(nodeOperatorId,sharesToDistribute)
Emit OperatorFeeDistributed(nodeOperatorId_1,sharesToDistribute_1)
 onlyAccounting()
MODIFIER_CALL, CSFeeDistributor.onlyAccounting()()
 sharesToDistribute
RETURN sharesToDistribute_1
```
#### CSFeeDistributor.processOracleReport(bytes32,string,string,uint256,uint256,uint256) [EXTERNAL]
```slithir
STETH_6(IStETH) := phi(['STETH_0', 'STETH_8', 'STETH_1', 'STETH_11', 'STETH_13', 'STETH_9', 'STETH_5', 'STETH_4'])
treeRoot_1(bytes32) := phi(['treeRoot_5', 'treeRoot_7', 'treeRoot_3', 'treeRoot_0'])
treeCid_1(string) := phi(['treeCid_3', 'treeCid_0', 'treeCid_5'])
logCid_1(string) := phi(['logCid_5', 'logCid_0'])
totalClaimableShares_5(uint256) := phi(['totalClaimableShares_3', 'totalClaimableShares_0', 'totalClaimableShares_4', 'totalClaimableShares_7', 'totalClaimableShares_10'])
distributionDataHistoryCount_1(uint256) := phi(['distributionDataHistoryCount_0', 'distributionDataHistoryCount_5'])
rebateRecipient_1(address) := phi(['rebateRecipient_4', 'rebateRecipient_3', 'rebateRecipient_0', 'rebateRecipient_5'])
 totalClaimableShares + distributed + rebate > STETH.sharesOf(address(this))
TMP_1977(uint256) = totalClaimableShares_6 (c)+ distributed_1
TMP_1978(uint256) = TMP_1977 (c)+ rebate_1
TMP_1979 = CONVERT this to address
TMP_1980(uint256) = HIGH_LEVEL_CALL, dest:STETH_7(IStETH), function:sharesOf, arguments:['TMP_1979']  
STETH_8(IStETH) := phi(['STETH_7', 'STETH_8', 'STETH_1', 'STETH_11', 'STETH_13', 'STETH_9', 'STETH_5', 'STETH_4'])
treeRoot_3(bytes32) := phi(['treeRoot_5', 'treeRoot_7', 'treeRoot_3', 'treeRoot_2'])
treeCid_3(string) := phi(['treeCid_3', 'treeCid_2', 'treeCid_5'])
logCid_3(string) := phi(['logCid_2', 'logCid_5'])
totalClaimableShares_7(uint256) := phi(['totalClaimableShares_3', 'totalClaimableShares_6', 'totalClaimableShares_4', 'totalClaimableShares_7', 'totalClaimableShares_10'])
distributionDataHistoryCount_3(uint256) := phi(['distributionDataHistoryCount_5', 'distributionDataHistoryCount_2'])
rebateRecipient_3(address) := phi(['rebateRecipient_4', 'rebateRecipient_3', 'rebateRecipient_2', 'rebateRecipient_5'])
TMP_1981(bool) = TMP_1978 > TMP_1980
CONDITION TMP_1981
 revert InvalidShares()()
TMP_1982(None) = SOLIDITY_CALL revert InvalidShares()()
 distributed == 0 && rebate > 0
TMP_1983(bool) = distributed_1 == 0
TMP_1984(bool) = rebate_1 > 0
TMP_1985(bool) = TMP_1983 && TMP_1984
CONDITION TMP_1985
 revert InvalidReportData()()
TMP_1986(None) = SOLIDITY_CALL revert InvalidReportData()()
 distributed > 0
TMP_1987(bool) = distributed_1 > 0
CONDITION TMP_1987
 bytes(_treeCid).length == 0
TMP_1988 = CONVERT _treeCid_1 to bytes
REF_735 -> LENGTH TMP_1988
TMP_1989(bool) = REF_735 == 0
CONDITION TMP_1989
 revert InvalidTreeCid()()
TMP_1990(None) = SOLIDITY_CALL revert InvalidTreeCid()()
 keccak256(bytes)(bytes(_treeCid)) == keccak256(bytes)(bytes(treeCid))
TMP_1991 = CONVERT _treeCid_1 to bytes
TMP_1992(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_1991)
TMP_1993 = CONVERT treeCid_3 to bytes
TMP_1994(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_1993)
TMP_1995(bool) = TMP_1992 == TMP_1994
CONDITION TMP_1995
 revert InvalidTreeCid()()
TMP_1996(None) = SOLIDITY_CALL revert InvalidTreeCid()()
 _treeRoot == bytes32(0)
TMP_1997 = CONVERT 0 to bytes32
TMP_1998(bool) = _treeRoot_1 == TMP_1997
CONDITION TMP_1998
 revert InvalidTreeRoot()()
TMP_1999(None) = SOLIDITY_CALL revert InvalidTreeRoot()()
 _treeRoot == treeRoot
TMP_2000(bool) = _treeRoot_1 == treeRoot_3
CONDITION TMP_2000
 revert InvalidTreeRoot()()
TMP_2001(None) = SOLIDITY_CALL revert InvalidTreeRoot()()
 totalClaimableShares += distributed
totalClaimableShares_8(uint256) = totalClaimableShares_7 + distributed_1
 treeRoot = _treeRoot
treeRoot_4(bytes32) := _treeRoot_1(bytes32)
 treeCid = _treeCid
treeCid_4(string) := _treeCid_1(string)
 DistributionDataUpdated(totalClaimableShares,_treeRoot,_treeCid)
Emit DistributionDataUpdated(totalClaimableShares_8,_treeRoot_1,_treeCid_1)
 ModuleFeeDistributed(distributed)
Emit ModuleFeeDistributed(distributed_1)
 rebate > 0
TMP_2004(bool) = rebate_1 > 0
CONDITION TMP_2004
 STETH.transferShares(rebateRecipient,rebate)
TMP_2005(uint256) = HIGH_LEVEL_CALL, dest:STETH_8(IStETH), function:transferShares, arguments:['rebateRecipient_3', 'rebate_1']  
STETH_9(IStETH) := phi(['STETH_8', 'STETH_1', 'STETH_11', 'STETH_13', 'STETH_9', 'STETH_5', 'STETH_4'])
treeRoot_5(bytes32) := phi(['treeRoot_5', 'treeRoot_4', 'treeRoot_7', 'treeRoot_3'])
treeCid_5(string) := phi(['treeCid_3', 'treeCid_4', 'treeCid_5'])
logCid_4(string) := phi(['logCid_5', 'logCid_3'])
distributionDataHistoryCount_4(uint256) := phi(['distributionDataHistoryCount_5', 'distributionDataHistoryCount_3'])
rebateRecipient_4(address) := phi(['rebateRecipient_4', 'rebateRecipient_3', 'rebateRecipient_5'])
 RebateTransferred(rebate)
Emit RebateTransferred(rebate_1)
 bytes(_logCid).length == 0
TMP_2007 = CONVERT _logCid_1 to bytes
REF_737 -> LENGTH TMP_2007
TMP_2008(bool) = REF_737 == 0
CONDITION TMP_2008
 revert InvalidLogCID()()
TMP_2009(None) = SOLIDITY_CALL revert InvalidLogCID()()
 keccak256(bytes)(bytes(_logCid)) == keccak256(bytes)(bytes(logCid))
TMP_2010 = CONVERT _logCid_1 to bytes
TMP_2011(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_2010)
TMP_2012 = CONVERT logCid_4 to bytes
TMP_2013(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_2012)
TMP_2014(bool) = TMP_2011 == TMP_2013
CONDITION TMP_2014
 revert InvalidLogCID()()
TMP_2015(None) = SOLIDITY_CALL revert InvalidLogCID()()
 logCid = _logCid
logCid_5(string) := _logCid_1(string)
 DistributionLogUpdated(_logCid)
Emit DistributionLogUpdated(_logCid_1)
 _distributionDataHistory[distributionDataHistoryCount] = DistributionData({refSlot:refSlot,treeRoot:treeRoot,treeCid:treeCid,logCid:_logCid,distributed:distributed,rebate:rebate})
REF_738(ICSFeeDistributor.DistributionData) -> _distributionDataHistory_0[distributionDataHistoryCount_4]
TMP_2017(ICSFeeDistributor.DistributionData) = new DistributionData(refSlot_1,treeRoot_5,treeCid_5,_logCid_1,distributed_1,rebate_1)
_distributionDataHistory_1(mapping(uint256 => ICSFeeDistributor.DistributionData)) := phi(['_distributionDataHistory_0'])
REF_738(ICSFeeDistributor.DistributionData) (->_distributionDataHistory_1) := TMP_2017(ICSFeeDistributor.DistributionData)
 ++ distributionDataHistoryCount
distributionDataHistoryCount_5(uint256) = distributionDataHistoryCount_4 + 1
 onlyOracle()
MODIFIER_CALL, CSFeeDistributor.onlyOracle()()
```
#### CSFeeDistributor.pendingSharesToDistribute() [EXTERNAL]
```slithir
STETH_12(IStETH) := phi(['STETH_0', 'STETH_8', 'STETH_1', 'STETH_11', 'STETH_13', 'STETH_9', 'STETH_5', 'STETH_4'])
totalClaimableShares_9(uint256) := phi(['totalClaimableShares_3', 'totalClaimableShares_0', 'totalClaimableShares_4', 'totalClaimableShares_7', 'totalClaimableShares_10'])
 STETH.sharesOf(address(this)) - totalClaimableShares
TMP_2025 = CONVERT this to address
TMP_2026(uint256) = HIGH_LEVEL_CALL, dest:STETH_12(IStETH), function:sharesOf, arguments:['TMP_2025']  
STETH_13(IStETH) := phi(['STETH_8', 'STETH_12', 'STETH_1', 'STETH_11', 'STETH_13', 'STETH_9', 'STETH_5', 'STETH_4'])
totalClaimableShares_10(uint256) := phi(['totalClaimableShares_3', 'totalClaimableShares_9', 'totalClaimableShares_4', 'totalClaimableShares_7', 'totalClaimableShares_10'])
TMP_2027(uint256) = TMP_2026 (c)- totalClaimableShares_10
RETURN TMP_2027
```
#### CSFeeDistributor.getHistoricalDistributionData(uint256) [EXTERNAL]
```slithir
_distributionDataHistory_2(mapping(uint256 => ICSFeeDistributor.DistributionData)) := phi(['_distributionDataHistory_1', '_distributionDataHistory_2', '_distributionDataHistory_0'])
 _distributionDataHistory[index]
REF_741(ICSFeeDistributor.DistributionData) -> _distributionDataHistory_2[index_1]
RETURN REF_741
```
#### CSFeeDistributor.hashLeaf(uint256,uint256) [EXTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1'])
shares_1(uint256) := phi(['cumulativeFeeShares_1'])
 keccak256(bytes)(bytes.concat(keccak256(bytes)(abi.encode(nodeOperatorId,shares))))
TMP_2037(bytes) = SOLIDITY_CALL abi.encode()(nodeOperatorId_1,shares_1)
TMP_2038(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_2037)
TMP_2039(bytes) = SOLIDITY_CALL bytes.concat()(TMP_2038)
TMP_2040(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_2039)
RETURN TMP_2040
```
#### CSFeeDistributor.constructor(address,address,address) [PUBLIC]
```slithir
 accounting == address(0)
TMP_1948 = CONVERT 0 to address
TMP_1949(bool) = accounting_1 == TMP_1948
CONDITION TMP_1949
 revert ZeroAccountingAddress()()
TMP_1950(None) = SOLIDITY_CALL revert ZeroAccountingAddress()()
 oracle == address(0)
TMP_1951 = CONVERT 0 to address
TMP_1952(bool) = oracle_1 == TMP_1951
CONDITION TMP_1952
 revert ZeroOracleAddress()()
TMP_1953(None) = SOLIDITY_CALL revert ZeroOracleAddress()()
 stETH == address(0)
TMP_1954 = CONVERT 0 to address
TMP_1955(bool) = stETH_1 == TMP_1954
CONDITION TMP_1955
 revert ZeroStEthAddress()()
TMP_1956(None) = SOLIDITY_CALL revert ZeroStEthAddress()()
 ACCOUNTING = accounting
ACCOUNTING_1(address) := accounting_1(address)
 STETH = IStETH(stETH)
TMP_1957 = CONVERT stETH_1 to IStETH
STETH_1(IStETH) := TMP_1957(IStETH)
 ORACLE = oracle
ORACLE_1(address) := oracle_1(address)
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### CSFeeDistributor.initialize(address,address) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_7', 'DEFAULT_ADMIN_ROLE_5'])
 admin == address(0)
TMP_1959 = CONVERT 0 to address
TMP_1960(bool) = admin_1 == TMP_1959
CONDITION TMP_1960
 revert ZeroAdminAddress()()
TMP_1961(None) = SOLIDITY_CALL revert ZeroAdminAddress()()
 _setRebateRecipient(_rebateRecipient)
INTERNAL_CALL, CSFeeDistributor._setRebateRecipient(address)(_rebateRecipient_1)
 __AccessControlEnumerable_init()
INTERNAL_CALL, AccessControlEnumerableUpgradeable.__AccessControlEnumerable_init()()
 _grantRole(DEFAULT_ADMIN_ROLE,admin)
TMP_1964(bool) = INTERNAL_CALL, AccessControlEnumerableUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_4,admin_1)
 reinitializer(2)
MODIFIER_CALL, Initializable.reinitializer(uint64)(2)
```
#### CSFeeDistributor.finalizeUpgradeV2(address) [EXTERNAL]
```slithir
 _setRebateRecipient(_rebateRecipient)
INTERNAL_CALL, CSFeeDistributor._setRebateRecipient(address)(_rebateRecipient_1)
 reinitializer(2)
MODIFIER_CALL, Initializable.reinitializer(uint64)(2)
```
#### CSFeeDistributor._setRebateRecipient(address) [INTERNAL]
```slithir
_rebateRecipient_1(address) := phi(['_rebateRecipient_1', '_rebateRecipient_1', '_rebateRecipient_1'])
 _rebateRecipient == address(0)
TMP_2041 = CONVERT 0 to address
TMP_2042(bool) = _rebateRecipient_1 == TMP_2041
CONDITION TMP_2042
 revert ZeroRebateRecipientAddress()()
TMP_2043(None) = SOLIDITY_CALL revert ZeroRebateRecipientAddress()()
 rebateRecipient = _rebateRecipient
rebateRecipient_5(address) := _rebateRecipient_1(address)
 RebateRecipientSet(_rebateRecipient)
Emit RebateRecipientSet(_rebateRecipient_1)
```
#### CSExitPenalties.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 VOLUNTARY_EXIT_TYPE_ID = 0
 STRIKES_EXIT_TYPE_ID = 1
MODULE_3(ICSModule) := phi(['MODULE_0', 'MODULE_2'])
 msg.sender != address(MODULE)
TMP_1859 = CONVERT MODULE_3 to address
TMP_1860(bool) = msg.sender != TMP_1859
CONDITION TMP_1860
 revert SenderIsNotModule()()
TMP_1861(None) = SOLIDITY_CALL revert SenderIsNotModule()()
STRIKES_2(address) := phi(['STRIKES_1', 'STRIKES_0'])
 msg.sender != STRIKES
TMP_1862(bool) = msg.sender != STRIKES_2
CONDITION TMP_1862
 revert SenderIsNotStrikes()()
TMP_1863(None) = SOLIDITY_CALL revert SenderIsNotStrikes()()
```
#### AssetRecovererLib.recoverERC20(address,uint256) [EXTERNAL]
```slithir
 IERC20(token).safeTransfer(msg.sender,amount)
TMP_4270 = CONVERT token_1 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_4270', 'msg.sender', 'amount_1'] 
 IAssetRecovererLib.ERC20Recovered(token,msg.sender,amount)
Emit ERC20Recovered(token_1,msg.sender,amount_1)
```
#### MerkleProof.verifyCalldata(bytes32[],bytes32,bytes32) [INTERNAL]
```slithir
 processProofCalldata(proof,leaf) == root
TMP_219(bytes32) = INTERNAL_CALL, MerkleProof.processProofCalldata(bytes32[],bytes32)(proof_1,leaf_1)
TMP_220(bool) = TMP_219 == root_1
RETURN TMP_220
```
#### IStETH.transferShares(address,uint256) [EXTERNAL]
```slithir

```
#### IStETH.sharesOf(address) [EXTERNAL]
```slithir

```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_63(transfer) -> token_1.transfer
TMP_150(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_63,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff980e4a00>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff980e56c0>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_150)
```
#### MerkleProof.processProofCalldata(bytes32[],bytes32) [INTERNAL]
```slithir
proof_1(bytes32[]) := phi(['proof_1'])
leaf_1(bytes32) := phi(['leaf_1'])
 computedHash = leaf
computedHash_1(bytes32) := leaf_1(bytes32)
 i = 0
i_1(uint256) := 0(uint256)
 i < proof.length
computedHash_2(bytes32) := phi(['computedHash_3', 'computedHash_1'])
i_2(uint256) := phi(['i_3', 'i_1'])
REF_88 -> LENGTH proof_1
TMP_224(bool) = i_2 < REF_88
CONDITION TMP_224
 computedHash = _hashPair(computedHash,proof[i])
REF_89(bytes32) -> proof_1[i_2]
TMP_225(bytes32) = INTERNAL_CALL, MerkleProof._hashPair(bytes32,bytes32)(computedHash_2,REF_89)
computedHash_3(bytes32) := TMP_225(bytes32)
 i ++
TMP_226(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 computedHash
RETURN computedHash_2
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
