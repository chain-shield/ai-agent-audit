
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
#### IVettedGateFactory.create(uint256,bytes32,string,address) [EXTERNAL]
```slithir

```
#### VettedGateFactory.constructor(address) [PUBLIC]
```slithir
 vettedGateImpl == address(0)
TMP_3941 = CONVERT 0 to address
TMP_3942(bool) = vettedGateImpl_1 == TMP_3941
CONDITION TMP_3942
 revert ZeroImplementationAddress()()
TMP_3943(None) = SOLIDITY_CALL revert ZeroImplementationAddress()()
 VETTED_GATE_IMPL = vettedGateImpl
VETTED_GATE_IMPL_1(address) := vettedGateImpl_1(address)
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
