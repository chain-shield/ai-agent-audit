

### Storage layout (VettedGate) 

```text
curveId uint256
treeRoot bytes32
treeCid string
_consumedAddresses mapping(address => bool)
isReferralProgramSeasonActive bool
referralProgramSeasonNumber uint256
referralCurveId uint256
referralsThreshold uint256
_referralCounts mapping(bytes32 => uint256)
_consumedReferrers mapping(bytes32 => bool)

```

#### VettedGate._onlyRecoverer() [INTERNAL]
```slithir
RECOVERER_ROLE_1(bytes32) := phi(['RECOVERER_ROLE_0', 'RECOVERER_ROLE_2'])
 _checkRole(RECOVERER_ROLE)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(RECOVERER_ROLE_1)
```
#### VettedGate.pauseFor(uint256) [EXTERNAL]
```slithir
PAUSE_ROLE_1(bytes32) := phi(['PAUSE_ROLE_2', 'PAUSE_ROLE_0'])
 _pauseFor(duration)
INTERNAL_CALL, PausableUntil._pauseFor(uint256)(duration_1)
 onlyRole(PAUSE_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(PAUSE_ROLE_1)
```
#### VettedGate.resume() [EXTERNAL]
```slithir
RESUME_ROLE_1(bytes32) := phi(['RESUME_ROLE_0', 'RESUME_ROLE_2'])
 _resume()
INTERNAL_CALL, PausableUntil._resume()()
 onlyRole(RESUME_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(RESUME_ROLE_1)
```
#### VettedGate.startNewReferralProgramSeason(uint256,uint256) [EXTERNAL]
```slithir
START_REFERRAL_SEASON_ROLE_1(bytes32) := phi(['START_REFERRAL_SEASON_ROLE_2', 'START_REFERRAL_SEASON_ROLE_0'])
ACCOUNTING_6(ICSAccounting) := phi(['ACCOUNTING_28', 'ACCOUNTING_0', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23'])
isReferralProgramSeasonActive_1(bool) := phi(['isReferralProgramSeasonActive_6', 'isReferralProgramSeasonActive_10', 'isReferralProgramSeasonActive_3', 'isReferralProgramSeasonActive_0'])
referralProgramSeasonNumber_1(uint256) := phi(['referralProgramSeasonNumber_0', 'referralProgramSeasonNumber_10', 'referralProgramSeasonNumber_4', 'referralProgramSeasonNumber_6', 'referralProgramSeasonNumber_13'])
 isReferralProgramSeasonActive
CONDITION isReferralProgramSeasonActive_2
 revert ReferralProgramIsActive()()
TMP_3809(None) = SOLIDITY_CALL revert ReferralProgramIsActive()()
 _referralCurveId == ACCOUNTING.DEFAULT_BOND_CURVE_ID()
TMP_3810(uint256) = HIGH_LEVEL_CALL, dest:ACCOUNTING_7(ICSAccounting), function:DEFAULT_BOND_CURVE_ID, arguments:[]  
ACCOUNTING_8(ICSAccounting) := phi(['ACCOUNTING_28', 'ACCOUNTING_7', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23'])
referralProgramSeasonNumber_3(uint256) := phi(['referralProgramSeasonNumber_10', 'referralProgramSeasonNumber_4', 'referralProgramSeasonNumber_2', 'referralProgramSeasonNumber_6', 'referralProgramSeasonNumber_13'])
TMP_3811(bool) = _referralCurveId_1 == TMP_3810
CONDITION TMP_3811
 revert InvalidCurveId()()
TMP_3812(None) = SOLIDITY_CALL revert InvalidCurveId()()
 _referralsThreshold == 0
TMP_3813(bool) = _referralsThreshold_1 == 0
CONDITION TMP_3813
 revert InvalidReferralsThreshold()()
TMP_3814(None) = SOLIDITY_CALL revert InvalidReferralsThreshold()()
 referralCurveId = _referralCurveId
referralCurveId_1(uint256) := _referralCurveId_1(uint256)
 referralsThreshold = _referralsThreshold
referralsThreshold_1(uint256) := _referralsThreshold_1(uint256)
 isReferralProgramSeasonActive = true
isReferralProgramSeasonActive_3(bool) := True(bool)
 season = referralProgramSeasonNumber + 1
TMP_3815(uint256) = referralProgramSeasonNumber_3 (c)+ 1
season_1(uint256) := TMP_3815(uint256)
 referralProgramSeasonNumber = season
referralProgramSeasonNumber_4(uint256) := season_1(uint256)
 ReferralProgramSeasonStarted(season,_referralCurveId,_referralsThreshold)
Emit ReferralProgramSeasonStarted(season_1,_referralCurveId_1,_referralsThreshold_1)
 onlyRole(START_REFERRAL_SEASON_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(START_REFERRAL_SEASON_ROLE_1)
 season
RETURN season_1
```
#### VettedGate.endCurrentReferralProgramSeason() [EXTERNAL]
```slithir
END_REFERRAL_SEASON_ROLE_1(bytes32) := phi(['END_REFERRAL_SEASON_ROLE_0', 'END_REFERRAL_SEASON_ROLE_2'])
isReferralProgramSeasonActive_4(bool) := phi(['isReferralProgramSeasonActive_6', 'isReferralProgramSeasonActive_10', 'isReferralProgramSeasonActive_3', 'isReferralProgramSeasonActive_0'])
referralProgramSeasonNumber_5(uint256) := phi(['referralProgramSeasonNumber_0', 'referralProgramSeasonNumber_10', 'referralProgramSeasonNumber_4', 'referralProgramSeasonNumber_6', 'referralProgramSeasonNumber_13'])
 ! isReferralProgramSeasonActive || referralProgramSeasonNumber == 0
TMP_3818 = UnaryType.BANG isReferralProgramSeasonActive_5 
TMP_3819(bool) = referralProgramSeasonNumber_6 == 0
TMP_3820(bool) = TMP_3818 || TMP_3819
CONDITION TMP_3820
 revert ReferralProgramIsNotActive()()
TMP_3821(None) = SOLIDITY_CALL revert ReferralProgramIsNotActive()()
 isReferralProgramSeasonActive = false
isReferralProgramSeasonActive_6(bool) := False(bool)
 ReferralProgramSeasonEnded(referralProgramSeasonNumber)
Emit ReferralProgramSeasonEnded(referralProgramSeasonNumber_6)
 onlyRole(END_REFERRAL_SEASON_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(END_REFERRAL_SEASON_ROLE_1)
```
#### VettedGate.addNodeOperatorETH(uint256,bytes,bytes,NodeOperatorManagementProperties,bytes32[],address) [EXTERNAL]
```slithir
MODULE_3(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_14', 'MODULE_0', 'MODULE_20'])
ACCOUNTING_9(ICSAccounting) := phi(['ACCOUNTING_28', 'ACCOUNTING_0', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23'])
curveId_2(uint256) := phi(['curveId_11', 'curveId_16', 'curveId_21', 'curveId_0', 'curveId_1', 'curveId_6'])
 _consume(proof)
INTERNAL_CALL, VettedGate._consume(bytes32[])(proof_1)
 nodeOperatorId = MODULE.createNodeOperator({from:msg.sender,managementProperties:managementProperties,referrer:referrer})
TMP_3825(uint256) = HIGH_LEVEL_CALL, dest:MODULE_5(ICSModule), function:createNodeOperator, arguments:['msg.sender', 'managementProperties_1', 'referrer_1']  
MODULE_6(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_14', 'MODULE_20', 'MODULE_5'])
ACCOUNTING_12(ICSAccounting) := phi(['ACCOUNTING_28', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23', 'ACCOUNTING_11'])
curveId_5(uint256) := phi(['curveId_11', 'curveId_16', 'curveId_4', 'curveId_21', 'curveId_1', 'curveId_6'])
nodeOperatorId_1(uint256) := TMP_3825(uint256)
 ACCOUNTING.setBondCurve(nodeOperatorId,curveId)
HIGH_LEVEL_CALL, dest:ACCOUNTING_12(ICSAccounting), function:setBondCurve, arguments:['nodeOperatorId_1', 'curveId_5']  
MODULE_7(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_14', 'MODULE_20', 'MODULE_6'])
ACCOUNTING_13(ICSAccounting) := phi(['ACCOUNTING_28', 'ACCOUNTING_12', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23'])
curveId_6(uint256) := phi(['curveId_11', 'curveId_16', 'curveId_5', 'curveId_21', 'curveId_1', 'curveId_6'])
 MODULE.addValidatorKeysETH{value: msg.value}({from:msg.sender,nodeOperatorId:nodeOperatorId,keysCount:keysCount,publicKeys:publicKeys,signatures:signatures})
HIGH_LEVEL_CALL, dest:MODULE_7(ICSModule), function:addValidatorKeysETH, arguments:['msg.sender', 'nodeOperatorId_1', 'keysCount_1', 'publicKeys_1', 'signatures_1'] value:msg.value 
MODULE_8(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_14', 'MODULE_20', 'MODULE_7'])
 _bumpReferralCount(referrer,nodeOperatorId)
INTERNAL_CALL, VettedGate._bumpReferralCount(address,uint256)(referrer_1,nodeOperatorId_1)
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
 nodeOperatorId
RETURN nodeOperatorId_1
```
#### VettedGate.addNodeOperatorStETH(uint256,bytes,bytes,NodeOperatorManagementProperties,ICSAccounting.PermitInput,bytes32[],address) [EXTERNAL]
```slithir
MODULE_9(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_14', 'MODULE_0', 'MODULE_20'])
ACCOUNTING_14(ICSAccounting) := phi(['ACCOUNTING_28', 'ACCOUNTING_0', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23'])
curveId_7(uint256) := phi(['curveId_11', 'curveId_16', 'curveId_21', 'curveId_0', 'curveId_1', 'curveId_6'])
 _consume(proof)
INTERNAL_CALL, VettedGate._consume(bytes32[])(proof_1)
 nodeOperatorId = MODULE.createNodeOperator({from:msg.sender,managementProperties:managementProperties,referrer:referrer})
TMP_3831(uint256) = HIGH_LEVEL_CALL, dest:MODULE_11(ICSModule), function:createNodeOperator, arguments:['msg.sender', 'managementProperties_1', 'referrer_1']  
MODULE_12(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_14', 'MODULE_20', 'MODULE_11'])
ACCOUNTING_17(ICSAccounting) := phi(['ACCOUNTING_16', 'ACCOUNTING_28', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23'])
curveId_10(uint256) := phi(['curveId_11', 'curveId_16', 'curveId_9', 'curveId_21', 'curveId_1', 'curveId_6'])
nodeOperatorId_1(uint256) := TMP_3831(uint256)
 ACCOUNTING.setBondCurve(nodeOperatorId,curveId)
HIGH_LEVEL_CALL, dest:ACCOUNTING_17(ICSAccounting), function:setBondCurve, arguments:['nodeOperatorId_1', 'curveId_10']  
MODULE_13(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_14', 'MODULE_20', 'MODULE_12'])
ACCOUNTING_18(ICSAccounting) := phi(['ACCOUNTING_28', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_17', 'ACCOUNTING_23'])
curveId_11(uint256) := phi(['curveId_11', 'curveId_10', 'curveId_16', 'curveId_21', 'curveId_1', 'curveId_6'])
 MODULE.addValidatorKeysStETH({from:msg.sender,nodeOperatorId:nodeOperatorId,keysCount:keysCount,publicKeys:publicKeys,signatures:signatures,permit:permit})
HIGH_LEVEL_CALL, dest:MODULE_13(ICSModule), function:addValidatorKeysStETH, arguments:['msg.sender', 'nodeOperatorId_1', 'keysCount_1', 'publicKeys_1', 'signatures_1', 'permit_1']  
MODULE_14(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_14', 'MODULE_20', 'MODULE_13'])
 _bumpReferralCount(referrer,nodeOperatorId)
INTERNAL_CALL, VettedGate._bumpReferralCount(address,uint256)(referrer_1,nodeOperatorId_1)
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
 nodeOperatorId
RETURN nodeOperatorId_1
```
#### VettedGate.addNodeOperatorWstETH(uint256,bytes,bytes,NodeOperatorManagementProperties,ICSAccounting.PermitInput,bytes32[],address) [EXTERNAL]
```slithir
MODULE_15(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_14', 'MODULE_0', 'MODULE_20'])
ACCOUNTING_19(ICSAccounting) := phi(['ACCOUNTING_28', 'ACCOUNTING_0', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23'])
curveId_12(uint256) := phi(['curveId_11', 'curveId_16', 'curveId_21', 'curveId_0', 'curveId_1', 'curveId_6'])
 _consume(proof)
INTERNAL_CALL, VettedGate._consume(bytes32[])(proof_1)
 nodeOperatorId = MODULE.createNodeOperator({from:msg.sender,managementProperties:managementProperties,referrer:referrer})
TMP_3837(uint256) = HIGH_LEVEL_CALL, dest:MODULE_17(ICSModule), function:createNodeOperator, arguments:['msg.sender', 'managementProperties_1', 'referrer_1']  
MODULE_18(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_14', 'MODULE_20', 'MODULE_17'])
ACCOUNTING_22(ICSAccounting) := phi(['ACCOUNTING_21', 'ACCOUNTING_28', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23'])
curveId_15(uint256) := phi(['curveId_11', 'curveId_16', 'curveId_14', 'curveId_21', 'curveId_1', 'curveId_6'])
nodeOperatorId_1(uint256) := TMP_3837(uint256)
 ACCOUNTING.setBondCurve(nodeOperatorId,curveId)
HIGH_LEVEL_CALL, dest:ACCOUNTING_22(ICSAccounting), function:setBondCurve, arguments:['nodeOperatorId_1', 'curveId_15']  
MODULE_19(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_14', 'MODULE_20', 'MODULE_18'])
ACCOUNTING_23(ICSAccounting) := phi(['ACCOUNTING_28', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23', 'ACCOUNTING_22'])
curveId_16(uint256) := phi(['curveId_11', 'curveId_16', 'curveId_15', 'curveId_21', 'curveId_1', 'curveId_6'])
 MODULE.addValidatorKeysWstETH({from:msg.sender,nodeOperatorId:nodeOperatorId,keysCount:keysCount,publicKeys:publicKeys,signatures:signatures,permit:permit})
HIGH_LEVEL_CALL, dest:MODULE_19(ICSModule), function:addValidatorKeysWstETH, arguments:['msg.sender', 'nodeOperatorId_1', 'keysCount_1', 'publicKeys_1', 'signatures_1', 'permit_1']  
MODULE_20(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_14', 'MODULE_20', 'MODULE_19'])
 _bumpReferralCount(referrer,nodeOperatorId)
INTERNAL_CALL, VettedGate._bumpReferralCount(address,uint256)(referrer_1,nodeOperatorId_1)
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
 nodeOperatorId
RETURN nodeOperatorId_1
```
#### VettedGate.claimBondCurve(uint256,bytes32[]) [EXTERNAL]
```slithir
ACCOUNTING_24(ICSAccounting) := phi(['ACCOUNTING_28', 'ACCOUNTING_0', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23'])
curveId_17(uint256) := phi(['curveId_11', 'curveId_16', 'curveId_21', 'curveId_0', 'curveId_1', 'curveId_6'])
 _onlyNodeOperatorOwner(nodeOperatorId)
INTERNAL_CALL, VettedGate._onlyNodeOperatorOwner(uint256)(nodeOperatorId_1)
 _consume(proof)
INTERNAL_CALL, VettedGate._consume(bytes32[])(proof_1)
 ACCOUNTING.setBondCurve(nodeOperatorId,curveId)
HIGH_LEVEL_CALL, dest:ACCOUNTING_27(ICSAccounting), function:setBondCurve, arguments:['nodeOperatorId_1', 'curveId_20']  
ACCOUNTING_28(ICSAccounting) := phi(['ACCOUNTING_28', 'ACCOUNTING_27', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23'])
curveId_21(uint256) := phi(['curveId_11', 'curveId_16', 'curveId_20', 'curveId_21', 'curveId_1', 'curveId_6'])
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
```
#### VettedGate.claimReferrerBondCurve(uint256,bytes32[]) [EXTERNAL]
```slithir
ACCOUNTING_29(ICSAccounting) := phi(['ACCOUNTING_28', 'ACCOUNTING_0', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23'])
isReferralProgramSeasonActive_7(bool) := phi(['isReferralProgramSeasonActive_6', 'isReferralProgramSeasonActive_10', 'isReferralProgramSeasonActive_3', 'isReferralProgramSeasonActive_0'])
referralProgramSeasonNumber_7(uint256) := phi(['referralProgramSeasonNumber_0', 'referralProgramSeasonNumber_10', 'referralProgramSeasonNumber_4', 'referralProgramSeasonNumber_6', 'referralProgramSeasonNumber_13'])
referralCurveId_2(uint256) := phi(['referralCurveId_1', 'referralCurveId_0', 'referralCurveId_7'])
referralsThreshold_2(uint256) := phi(['referralsThreshold_6', 'referralsThreshold_1', 'referralsThreshold_0'])
_referralCounts_1(mapping(bytes32 => uint256)) := phi(['_referralCounts_12', '_referralCounts_7', '_referralCounts_0', '_referralCounts_9', '_referralCounts_5'])
_consumedReferrers_1(mapping(bytes32 => bool)) := phi(['_consumedReferrers_6', '_consumedReferrers_8', '_consumedReferrers_0'])
 _onlyNodeOperatorOwner(nodeOperatorId)
INTERNAL_CALL, VettedGate._onlyNodeOperatorOwner(uint256)(nodeOperatorId_1)
 ! verifyProof(msg.sender,proof)
TMP_3847(bool) = INTERNAL_CALL, VettedGate.verifyProof(address,bytes32[])(msg.sender,proof_1)
TMP_3848 = UnaryType.BANG TMP_3847 
CONDITION TMP_3848
 revert InvalidProof()()
TMP_3849(None) = SOLIDITY_CALL revert InvalidProof()()
 ! isReferralProgramSeasonActive
TMP_3850 = UnaryType.BANG isReferralProgramSeasonActive_10 
CONDITION TMP_3850
 revert ReferralProgramIsNotActive()()
TMP_3851(None) = SOLIDITY_CALL revert ReferralProgramIsNotActive()()
 season = referralProgramSeasonNumber
season_1(uint256) := referralProgramSeasonNumber_10(uint256)
 referrer = _seasonedAddress(msg.sender,season)
TMP_3852(bytes32) = INTERNAL_CALL, VettedGate._seasonedAddress(address,uint256)(msg.sender,season_1)
referrer_1(bytes32) := TMP_3852(bytes32)
 _referralCounts[referrer] < referralsThreshold
REF_1648(uint256) -> _referralCounts_5[referrer_1]
TMP_3853(bool) = REF_1648 < referralsThreshold_6
CONDITION TMP_3853
 revert NotEnoughReferrals()()
TMP_3854(None) = SOLIDITY_CALL revert NotEnoughReferrals()()
 _consumedReferrers[referrer]
REF_1649(bool) -> _consumedReferrers_5[referrer_1]
CONDITION REF_1649
 revert AlreadyConsumed()()
TMP_3855(None) = SOLIDITY_CALL revert AlreadyConsumed()()
 _consumedReferrers[referrer] = true
REF_1650(bool) -> _consumedReferrers_5[referrer_1]
_consumedReferrers_6(mapping(bytes32 => bool)) := phi(['_consumedReferrers_5'])
REF_1650(bool) (->_consumedReferrers_6) := True(bool)
 ReferrerConsumed(msg.sender,season)
Emit ReferrerConsumed(msg.sender,season_1)
 ACCOUNTING.setBondCurve(nodeOperatorId,referralCurveId)
HIGH_LEVEL_CALL, dest:ACCOUNTING_33(ICSAccounting), function:setBondCurve, arguments:['nodeOperatorId_1', 'referralCurveId_6']  
ACCOUNTING_34(ICSAccounting) := phi(['ACCOUNTING_33', 'ACCOUNTING_28', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23'])
referralCurveId_7(uint256) := phi(['referralCurveId_7', 'referralCurveId_1', 'referralCurveId_6'])
 whenResumed()
MODIFIER_CALL, PausableUntil.whenResumed()()
```
#### VettedGate.verifyProof(address,bytes32[]) [EXTERNAL]
```slithir
member_1(address) := phi(['msg.sender'])
proof_1(bytes32[]) := phi(['proof_1', 'proof_1'])
treeRoot_1(bytes32) := phi(['treeRoot_2', 'treeRoot_0', 'treeRoot_4'])
 MerkleProof.verifyCalldata(proof,treeRoot,hashLeaf(member))
TMP_3865(bytes32) = INTERNAL_CALL, VettedGate.hashLeaf(address)(member_1)
TMP_3866(bool) = LIBRARY_CALL, dest:MerkleProof, function:MerkleProof.verifyCalldata(bytes32[],bytes32,bytes32), arguments:['proof_1', 'treeRoot_2', 'TMP_3865'] 
RETURN TMP_3866
```
#### VettedGate.isConsumed(address) [EXTERNAL]
```slithir
member_1(address) := phi(['msg.sender'])
_consumedAddresses_1(mapping(address => bool)) := phi(['_consumedAddresses_1', '_consumedAddresses_0', '_consumedAddresses_2'])
 _consumedAddresses[member]
REF_1655(bool) -> _consumedAddresses_1[member_1]
RETURN REF_1655
```
#### VettedGate.isReferrerConsumed(address) [EXTERNAL]
```slithir
_consumedReferrers_7(mapping(bytes32 => bool)) := phi(['_consumedReferrers_6', '_consumedReferrers_8', '_consumedReferrers_0'])
 _consumedReferrers[_seasonedAddress(referrer)]
TMP_3864(bytes32) = INTERNAL_CALL, VettedGate._seasonedAddress(address)(referrer_1)
REF_1654(bool) -> _consumedReferrers_8[TMP_3864]
RETURN REF_1654
```
#### VettedGate.hashLeaf(address) [EXTERNAL]
```slithir
member_1(address) := phi(['member_1'])
 keccak256(bytes)(bytes.concat(keccak256(bytes)(abi.encode(member))))
TMP_3867(bytes) = SOLIDITY_CALL abi.encode()(member_1)
TMP_3868(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_3867)
TMP_3869(bytes) = SOLIDITY_CALL bytes.concat()(TMP_3868)
TMP_3870(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_3869)
RETURN TMP_3870
```
#### VettedGate.setTreeParams(bytes32,string) [EXTERNAL]
```slithir
SET_TREE_ROLE_1(bytes32) := phi(['SET_TREE_ROLE_0', 'SET_TREE_ROLE_2'])
 _setTreeParams(_treeRoot,_treeCid)
INTERNAL_CALL, VettedGate._setTreeParams(bytes32,string)(_treeRoot_1,_treeCid_1)
 onlyRole(SET_TREE_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(SET_TREE_ROLE_1)
```
#### VettedGate.getReferralsCount(address,uint256) [EXTERNAL]
```slithir
_referralCounts_8(mapping(bytes32 => uint256)) := phi(['_referralCounts_12', '_referralCounts_7', '_referralCounts_0', '_referralCounts_9', '_referralCounts_5'])
 _referralCounts[_seasonedAddress(referrer,season)]
TMP_3862(bytes32) = INTERNAL_CALL, VettedGate._seasonedAddress(address,uint256)(referrer_1,season_1)
REF_1653(uint256) -> _referralCounts_9[TMP_3862]
RETURN REF_1653
```
#### VettedGate.getInitializedVersion() [EXTERNAL]
```slithir
 _getInitializedVersion()
TMP_3863(uint64) = INTERNAL_CALL, Initializable._getInitializedVersion()()
RETURN TMP_3863
```
#### VettedGate.constructor(address) [PUBLIC]
```slithir
 module == address(0)
TMP_3788 = CONVERT 0 to address
TMP_3789(bool) = module_1 == TMP_3788
CONDITION TMP_3789
 revert ZeroModuleAddress()()
TMP_3790(None) = SOLIDITY_CALL revert ZeroModuleAddress()()
 MODULE = ICSModule(module)
TMP_3791 = CONVERT module_1 to ICSModule
MODULE_1(ICSModule) := TMP_3791(ICSModule)
 ACCOUNTING = ICSAccounting(MODULE.accounting())
TMP_3792(ICSAccounting) = HIGH_LEVEL_CALL, dest:MODULE_1(ICSModule), function:accounting, arguments:[]  
MODULE_2(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_14', 'MODULE_20', 'MODULE_1'])
TMP_3793 = CONVERT TMP_3792 to ICSAccounting
ACCOUNTING_1(ICSAccounting) := TMP_3793(ICSAccounting)
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### VettedGate.initialize(uint256,bytes32,string,address) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_0'])
ACCOUNTING_2(ICSAccounting) := phi(['ACCOUNTING_28', 'ACCOUNTING_0', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23'])
 __AccessControlEnumerable_init()
INTERNAL_CALL, AccessControlEnumerableUpgradeable.__AccessControlEnumerable_init()()
 _curveId == ACCOUNTING.DEFAULT_BOND_CURVE_ID()
TMP_3796(uint256) = HIGH_LEVEL_CALL, dest:ACCOUNTING_4(ICSAccounting), function:DEFAULT_BOND_CURVE_ID, arguments:[]  
DEFAULT_ADMIN_ROLE_4(bytes32) := phi(['DEFAULT_ADMIN_ROLE_3', 'DEFAULT_ADMIN_ROLE_6'])
ACCOUNTING_5(ICSAccounting) := phi(['ACCOUNTING_28', 'ACCOUNTING_5', 'ACCOUNTING_34', 'ACCOUNTING_13', 'ACCOUNTING_1', 'ACCOUNTING_18', 'ACCOUNTING_8', 'ACCOUNTING_23', 'ACCOUNTING_4'])
TMP_3797(bool) = _curveId_1 == TMP_3796
CONDITION TMP_3797
 revert InvalidCurveId()()
TMP_3798(None) = SOLIDITY_CALL revert InvalidCurveId()()
 curveId = _curveId
curveId_1(uint256) := _curveId_1(uint256)
 admin == address(0)
TMP_3799 = CONVERT 0 to address
TMP_3800(bool) = admin_1 == TMP_3799
CONDITION TMP_3800
 revert ZeroAdminAddress()()
TMP_3801(None) = SOLIDITY_CALL revert ZeroAdminAddress()()
 _setTreeParams(_treeRoot,_treeCid)
INTERNAL_CALL, VettedGate._setTreeParams(bytes32,string)(_treeRoot_1,_treeCid_1)
 _grantRole(DEFAULT_ADMIN_ROLE,admin)
TMP_3803(bool) = INTERNAL_CALL, AccessControlEnumerableUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_5,admin_1)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### VettedGate._consume(bytes32[]) [INTERNAL]
```slithir
proof_1(bytes32[]) := phi(['proof_1', 'proof_1', 'proof_1', 'proof_1'])
 isConsumed(msg.sender)
TMP_3871(bool) = INTERNAL_CALL, VettedGate.isConsumed(address)(msg.sender)
CONDITION TMP_3871
 revert AlreadyConsumed()()
TMP_3872(None) = SOLIDITY_CALL revert AlreadyConsumed()()
 ! verifyProof(msg.sender,proof)
TMP_3873(bool) = INTERNAL_CALL, VettedGate.verifyProof(address,bytes32[])(msg.sender,proof_1)
TMP_3874 = UnaryType.BANG TMP_3873 
CONDITION TMP_3874
 revert InvalidProof()()
TMP_3875(None) = SOLIDITY_CALL revert InvalidProof()()
 _consumedAddresses[msg.sender] = true
REF_1659(bool) -> _consumedAddresses_1[msg.sender]
_consumedAddresses_2(mapping(address => bool)) := phi(['_consumedAddresses_1'])
REF_1659(bool) (->_consumedAddresses_2) := True(bool)
 Consumed(msg.sender)
Emit Consumed(msg.sender)
```
#### VettedGate._setTreeParams(bytes32,string) [INTERNAL]
```slithir
_treeRoot_1(bytes32) := phi(['_treeRoot_1', '_treeRoot_1'])
_treeCid_1(string) := phi(['_treeCid_1', '_treeCid_1'])
treeRoot_3(bytes32) := phi(['treeRoot_2', 'treeRoot_0', 'treeRoot_4'])
treeCid_1(string) := phi(['treeCid_0', 'treeCid_2'])
 _treeRoot == bytes32(0)
TMP_3877 = CONVERT 0 to bytes32
TMP_3878(bool) = _treeRoot_1 == TMP_3877
CONDITION TMP_3878
 revert InvalidTreeRoot()()
TMP_3879(None) = SOLIDITY_CALL revert InvalidTreeRoot()()
 _treeRoot == treeRoot
TMP_3880(bool) = _treeRoot_1 == treeRoot_3
CONDITION TMP_3880
 revert InvalidTreeRoot()()
TMP_3881(None) = SOLIDITY_CALL revert InvalidTreeRoot()()
 bytes(_treeCid).length == 0
TMP_3882 = CONVERT _treeCid_1 to bytes
REF_1660 -> LENGTH TMP_3882
TMP_3883(bool) = REF_1660 == 0
CONDITION TMP_3883
 revert InvalidTreeCid()()
TMP_3884(None) = SOLIDITY_CALL revert InvalidTreeCid()()
 keccak256(bytes)(bytes(_treeCid)) == keccak256(bytes)(bytes(treeCid))
TMP_3885 = CONVERT _treeCid_1 to bytes
TMP_3886(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_3885)
TMP_3887 = CONVERT treeCid_1 to bytes
TMP_3888(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_3887)
TMP_3889(bool) = TMP_3886 == TMP_3888
CONDITION TMP_3889
 revert InvalidTreeCid()()
TMP_3890(None) = SOLIDITY_CALL revert InvalidTreeCid()()
 treeRoot = _treeRoot
treeRoot_4(bytes32) := _treeRoot_1(bytes32)
 treeCid = _treeCid
treeCid_2(string) := _treeCid_1(string)
 TreeSet(_treeRoot,_treeCid)
Emit TreeSet(_treeRoot_1,_treeCid_1)
```
#### VettedGate._bumpReferralCount(address,uint256) [INTERNAL]
```slithir
referrer_1(address) := phi(['referrer_1', 'referrer_1', 'referrer_1'])
referralNodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1', 'nodeOperatorId_1'])
isReferralProgramSeasonActive_11(bool) := phi(['isReferralProgramSeasonActive_6', 'isReferralProgramSeasonActive_10', 'isReferralProgramSeasonActive_3', 'isReferralProgramSeasonActive_0'])
referralProgramSeasonNumber_11(uint256) := phi(['referralProgramSeasonNumber_0', 'referralProgramSeasonNumber_10', 'referralProgramSeasonNumber_4', 'referralProgramSeasonNumber_6', 'referralProgramSeasonNumber_13'])
_referralCounts_10(mapping(bytes32 => uint256)) := phi(['_referralCounts_12', '_referralCounts_7', '_referralCounts_0', '_referralCounts_9', '_referralCounts_5'])
 season = referralProgramSeasonNumber
season_1(uint256) := referralProgramSeasonNumber_11(uint256)
 isReferralProgramSeasonActive && referrer != address(0) && referrer != msg.sender
TMP_3892 = CONVERT 0 to address
TMP_3893(bool) = referrer_1 != TMP_3892
TMP_3894(bool) = isReferralProgramSeasonActive_11 && TMP_3893
TMP_3895(bool) = referrer_1 != msg.sender
TMP_3896(bool) = TMP_3894 && TMP_3895
CONDITION TMP_3896
 _referralCounts[_seasonedAddress(referrer,season)] += 1
TMP_3897(bytes32) = INTERNAL_CALL, VettedGate._seasonedAddress(address,uint256)(referrer_1,season_1)
REF_1661(uint256) -> _referralCounts_11[TMP_3897]
_referralCounts_12(mapping(bytes32 => uint256)) := phi(['_referralCounts_11'])
REF_1661(-> _referralCounts_12) = REF_1661 (c)+ 1
 ReferralRecorded(referrer,season,referralNodeOperatorId)
Emit ReferralRecorded(referrer_1,season_1,referralNodeOperatorId_1)
```
#### VettedGate._seasonedAddress(address,uint256) [INTERNAL]
```slithir
referrer_1(address) := phi(['referrer_1', 'msg.sender', 'referrer_1', 'referrer_1'])
season_1(uint256) := phi(['season_1', 'season_1', 'referralProgramSeasonNumber_12', 'season_1'])
 keccak256(bytes)(abi.encode(referrer,season))
TMP_3907(bytes) = SOLIDITY_CALL abi.encode()(referrer_1,season_1)
TMP_3908(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_3907)
RETURN TMP_3908
```
#### VettedGate._onlyNodeOperatorOwner(uint256) [INTERNAL]
```slithir
nodeOperatorId_1(uint256) := phi(['nodeOperatorId_1', 'nodeOperatorId_1'])
MODULE_21(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_14', 'MODULE_0', 'MODULE_20'])
 owner = MODULE.getNodeOperatorOwner(nodeOperatorId)
TMP_3900(address) = HIGH_LEVEL_CALL, dest:MODULE_21(ICSModule), function:getNodeOperatorOwner, arguments:['nodeOperatorId_1']  
MODULE_22(ICSModule) := phi(['MODULE_22', 'MODULE_8', 'MODULE_2', 'MODULE_21', 'MODULE_14', 'MODULE_20'])
owner_1(address) := TMP_3900(address)
 owner == address(0)
TMP_3901 = CONVERT 0 to address
TMP_3902(bool) = owner_1 == TMP_3901
CONDITION TMP_3902
 revert NodeOperatorDoesNotExist()()
TMP_3903(None) = SOLIDITY_CALL revert NodeOperatorDoesNotExist()()
 owner != msg.sender
TMP_3904(bool) = owner_1 != msg.sender
CONDITION TMP_3904
 revert NotAllowedToClaim()()
TMP_3905(None) = SOLIDITY_CALL revert NotAllowedToClaim()()
```
#### PermissionlessGate.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
 RECOVERER_ROLE = keccak256(bytes)(RECOVERER_ROLE)
role_1(bytes32) := phi(['TMP_3640', 'TMP_3633', 'TMP_3638', 'TMP_3635'])
 _checkRole(role)
INTERNAL_CALL, AccessControl._checkRole(bytes32)(role_1)
```
#### ICSModule.addValidatorKeysETH(address,uint256,uint256,bytes,bytes) [EXTERNAL]
```slithir

```
#### ICSModule.createNodeOperator(address,NodeOperatorManagementProperties,address) [EXTERNAL]
```slithir

```
#### ICSAccounting.setBondCurve(uint256,uint256) [EXTERNAL]
```slithir

```
#### ICSModule.addValidatorKeysStETH(address,uint256,uint256,bytes,bytes,ICSAccounting.PermitInput) [EXTERNAL]
```slithir

```
#### ICSModule.addValidatorKeysWstETH(address,uint256,uint256,bytes,bytes,ICSAccounting.PermitInput) [EXTERNAL]
```slithir

```
#### MerkleProof.verifyCalldata(bytes32[],bytes32,bytes32) [INTERNAL]
```slithir
 processProofCalldata(proof,leaf) == root
TMP_219(bytes32) = INTERNAL_CALL, MerkleProof.processProofCalldata(bytes32[],bytes32)(proof_1,leaf_1)
TMP_220(bool) = TMP_219 == root_1
RETURN TMP_220
```
#### ICSModule.accounting() [EXTERNAL]
```slithir

```
#### ICSModule.getNodeOperatorOwner(uint256) [EXTERNAL]
```slithir

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
