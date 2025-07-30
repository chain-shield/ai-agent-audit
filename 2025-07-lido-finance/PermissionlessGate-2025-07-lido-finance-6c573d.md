

#### PermissionlessGate._onlyRecoverer() [INTERNAL]
```slithir
RECOVERER_ROLE_1(bytes32) := phi(['RECOVERER_ROLE_2', 'RECOVERER_ROLE_0'])
 _checkRole(RECOVERER_ROLE)
INTERNAL_CALL, AccessControl._checkRole(bytes32)(RECOVERER_ROLE_1)
```
#### PermissionlessGate.addNodeOperatorETH(uint256,bytes,bytes,NodeOperatorManagementProperties,address) [EXTERNAL]
```slithir
MODULE_4(ICSModule) := phi(['MODULE_12', 'MODULE_9', 'MODULE_6', 'MODULE_0', 'MODULE_3'])
 nodeOperatorId = MODULE.createNodeOperator({from:msg.sender,managementProperties:managementProperties,referrer:referrer})
TMP_3667(uint256) = HIGH_LEVEL_CALL, dest:MODULE_4(ICSModule), function:createNodeOperator, arguments:['msg.sender', 'managementProperties_1', 'referrer_1']  
MODULE_5(ICSModule) := phi(['MODULE_12', 'MODULE_9', 'MODULE_6', 'MODULE_4', 'MODULE_3'])
nodeOperatorId_1(uint256) := TMP_3667(uint256)
 MODULE.addValidatorKeysETH{value: msg.value}({from:msg.sender,nodeOperatorId:nodeOperatorId,keysCount:keysCount,publicKeys:publicKeys,signatures:signatures})
HIGH_LEVEL_CALL, dest:MODULE_5(ICSModule), function:addValidatorKeysETH, arguments:['msg.sender', 'nodeOperatorId_1', 'keysCount_1', 'publicKeys_1', 'signatures_1'] value:msg.value 
MODULE_6(ICSModule) := phi(['MODULE_12', 'MODULE_9', 'MODULE_6', 'MODULE_5', 'MODULE_3'])
 nodeOperatorId
RETURN nodeOperatorId_1
```
#### PermissionlessGate.addNodeOperatorStETH(uint256,bytes,bytes,NodeOperatorManagementProperties,ICSAccounting.PermitInput,address) [EXTERNAL]
```slithir
MODULE_7(ICSModule) := phi(['MODULE_12', 'MODULE_9', 'MODULE_6', 'MODULE_0', 'MODULE_3'])
 nodeOperatorId = MODULE.createNodeOperator({from:msg.sender,managementProperties:managementProperties,referrer:referrer})
TMP_3669(uint256) = HIGH_LEVEL_CALL, dest:MODULE_7(ICSModule), function:createNodeOperator, arguments:['msg.sender', 'managementProperties_1', 'referrer_1']  
MODULE_8(ICSModule) := phi(['MODULE_12', 'MODULE_9', 'MODULE_6', 'MODULE_7', 'MODULE_3'])
nodeOperatorId_1(uint256) := TMP_3669(uint256)
 MODULE.addValidatorKeysStETH({from:msg.sender,nodeOperatorId:nodeOperatorId,keysCount:keysCount,publicKeys:publicKeys,signatures:signatures,permit:permit})
HIGH_LEVEL_CALL, dest:MODULE_8(ICSModule), function:addValidatorKeysStETH, arguments:['msg.sender', 'nodeOperatorId_1', 'keysCount_1', 'publicKeys_1', 'signatures_1', 'permit_1']  
MODULE_9(ICSModule) := phi(['MODULE_12', 'MODULE_9', 'MODULE_6', 'MODULE_8', 'MODULE_3'])
 nodeOperatorId
RETURN nodeOperatorId_1
```
#### PermissionlessGate.addNodeOperatorWstETH(uint256,bytes,bytes,NodeOperatorManagementProperties,ICSAccounting.PermitInput,address) [EXTERNAL]
```slithir
MODULE_10(ICSModule) := phi(['MODULE_12', 'MODULE_9', 'MODULE_6', 'MODULE_0', 'MODULE_3'])
 nodeOperatorId = MODULE.createNodeOperator({from:msg.sender,managementProperties:managementProperties,referrer:referrer})
TMP_3671(uint256) = HIGH_LEVEL_CALL, dest:MODULE_10(ICSModule), function:createNodeOperator, arguments:['msg.sender', 'managementProperties_1', 'referrer_1']  
MODULE_11(ICSModule) := phi(['MODULE_12', 'MODULE_9', 'MODULE_6', 'MODULE_10', 'MODULE_3'])
nodeOperatorId_1(uint256) := TMP_3671(uint256)
 MODULE.addValidatorKeysWstETH({from:msg.sender,nodeOperatorId:nodeOperatorId,keysCount:keysCount,publicKeys:publicKeys,signatures:signatures,permit:permit})
HIGH_LEVEL_CALL, dest:MODULE_11(ICSModule), function:addValidatorKeysWstETH, arguments:['msg.sender', 'nodeOperatorId_1', 'keysCount_1', 'publicKeys_1', 'signatures_1', 'permit_1']  
MODULE_12(ICSModule) := phi(['MODULE_12', 'MODULE_9', 'MODULE_6', 'MODULE_11', 'MODULE_3'])
 nodeOperatorId
RETURN nodeOperatorId_1
```
#### PermissionlessGate.constructor(address,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_0'])
 module == address(0)
TMP_3657 = CONVERT 0 to address
TMP_3658(bool) = module_1 == TMP_3657
CONDITION TMP_3658
 revert ZeroModuleAddress()()
TMP_3659(None) = SOLIDITY_CALL revert ZeroModuleAddress()()
 admin == address(0)
TMP_3660 = CONVERT 0 to address
TMP_3661(bool) = admin_1 == TMP_3660
CONDITION TMP_3661
 revert ZeroAdminAddress()()
TMP_3662(None) = SOLIDITY_CALL revert ZeroAdminAddress()()
 MODULE = ICSModule(module)
TMP_3663 = CONVERT module_1 to ICSModule
MODULE_1(ICSModule) := TMP_3663(ICSModule)
 CURVE_ID = MODULE.accounting().DEFAULT_BOND_CURVE_ID()
TMP_3664(ICSAccounting) = HIGH_LEVEL_CALL, dest:MODULE_1(ICSModule), function:accounting, arguments:[]  
DEFAULT_ADMIN_ROLE_2(bytes32) := phi(['DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_1'])
MODULE_2(ICSModule) := phi(['MODULE_12', 'MODULE_9', 'MODULE_6', 'MODULE_1', 'MODULE_3'])
TMP_3665(uint256) = HIGH_LEVEL_CALL, dest:TMP_3664(ICSAccounting), function:DEFAULT_BOND_CURVE_ID, arguments:[]  
DEFAULT_ADMIN_ROLE_3(bytes32) := phi(['DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_2'])
MODULE_3(ICSModule) := phi(['MODULE_2', 'MODULE_12', 'MODULE_9', 'MODULE_6', 'MODULE_3'])
CURVE_ID_1(uint256) := TMP_3665(uint256)
 _grantRole(DEFAULT_ADMIN_ROLE,admin)
TMP_3666(bool) = INTERNAL_CALL, AccessControlEnumerable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_3,admin_1)
```
#### CSVerifier.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
 RESUME_SINCE_TIMESTAMP_POSITION = keccak256(bytes)(lido.PausableUntil.resumeSinceTimestamp)
 PAUSE_INFINITELY = type()(uint256).max
 PAUSE_ROLE = keccak256(bytes)(PAUSE_ROLE)
 RESUME_ROLE = keccak256(bytes)(RESUME_ROLE)
 BEACON_ROOTS = 0x000F3df6D732807Ef1319fB7B8bB8522d0Beac02
 _checkPaused()
INTERNAL_CALL, PausableUntil._checkPaused()()
 _checkResumed()
INTERNAL_CALL, PausableUntil._checkResumed()()
role_1(bytes32) := phi(['TMP_3480', 'RESUME_ROLE_1', 'TMP_3473', 'TMP_3478', 'TMP_3475', 'PAUSE_ROLE_1'])
 _checkRole(role)
INTERNAL_CALL, AccessControl._checkRole(bytes32)(role_1)
```
#### ICSModule.addValidatorKeysETH(address,uint256,uint256,bytes,bytes) [EXTERNAL]
```slithir

```
#### ICSModule.createNodeOperator(address,NodeOperatorManagementProperties,address) [EXTERNAL]
```slithir

```
#### ICSModule.addValidatorKeysStETH(address,uint256,uint256,bytes,bytes,ICSAccounting.PermitInput) [EXTERNAL]
```slithir

```
#### ICSModule.addValidatorKeysWstETH(address,uint256,uint256,bytes,bytes,ICSAccounting.PermitInput) [EXTERNAL]
```slithir

```
#### ICSModule.accounting() [EXTERNAL]
```slithir

```
