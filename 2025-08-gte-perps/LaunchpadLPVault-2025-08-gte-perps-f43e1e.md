### Storage layout (LaunchpadLPVault) 

```text
launchpad address

```
#### LaunchpadLPVault.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### LaunchpadLPVault.fallback() [EXTERNAL]
```slithir
 revert FallbackRevert()()
TMP_3461(None) = SOLIDITY_CALL revert FallbackRevert()()
```
#### LaunchpadLPVault.initialize(address,address) [EXTERNAL]
```slithir
 launchpad = launchpad_
launchpad_1(address) := launchpad__1(address)
 __Ownable_init(initialOwner)
INTERNAL_CALL, OwnableUpgradeable.__Ownable_init(address)(initialOwner_1)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### Launchpad.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 _OWNER_SLOT = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927
 ABI_VERSION = 1
 TOTAL_SUPPLY = 1000000000000000000 * 1e9
 BONDING_SUPPLY = 800000000000000000000000000
_REENTRANCY_GUARD_SLOT_1(uint256) := phi(['_REENTRANCY_GUARD_SLOT_3', '_REENTRANCY_GUARD_SLOT_0'])
 sload(uint256)(_REENTRANCY_GUARD_SLOT) == address()()
TMP_3349(uint256) := _REENTRANCY_GUARD_SLOT_1(uint256)
TMP_3350 = CONVERT this to address
TMP_3351(bool) = TMP_3349 == TMP_3350
CONDITION TMP_3351
 mstore(uint256,uint256)(0x00,0xab143c06)
TMP_3352(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,2870230022)
 revert(uint256,uint256)(0x1c,0x04)
TMP_3353(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 sstore(uint256,uint256)(_REENTRANCY_GUARD_SLOT,address()())
TMP_3354 = CONVERT this to address
_REENTRANCY_GUARD_SLOT_2(uint256) := TMP_3354(address)
 sstore(uint256,uint256)(_REENTRANCY_GUARD_SLOT,codesize()())
TMP_3355(uint256) = SOLIDITY_CALL codesize()()
_REENTRANCY_GUARD_SLOT_3(uint256) := TMP_3355(uint256)
_REENTRANCY_GUARD_SLOT_4(uint256) := phi(['_REENTRANCY_GUARD_SLOT_3', '_REENTRANCY_GUARD_SLOT_0'])
 sload(uint256)(_REENTRANCY_GUARD_SLOT) == address()()
TMP_3356(uint256) := _REENTRANCY_GUARD_SLOT_4(uint256)
TMP_3357 = CONVERT this to address
TMP_3358(bool) = TMP_3356 == TMP_3357
CONDITION TMP_3358
 mstore(uint256,uint256)(0x00,0xab143c06)
TMP_3359(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,2870230022)
 revert(uint256,uint256)(0x1c,0x04)
TMP_3360(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 _checkOwner()
INTERNAL_CALL, Ownable._checkOwner()()
_INTIALIZED_EVENT_SIGNATURE_3(bytes32) := phi(['_INTIALIZED_EVENT_SIGNATURE_0', '_INTIALIZED_EVENT_SIGNATURE_4', '_INTIALIZED_EVENT_SIGNATURE_2', '_INTIALIZED_EVENT_SIGNATURE_6'])
 s = _initializableSlot()
TMP_3362(bytes32) = INTERNAL_CALL, Initializable._initializableSlot()()
s_1(bytes32) := TMP_3362(bytes32)
 i_initializer_asm_0 = sload(uint256)(s)
TMP_3363(uint256) = SOLIDITY_CALL sload(uint256)(s_1)
i_initializer_asm_0_1(uint256) := TMP_3363(uint256)
 sstore(uint256,uint256)(s,3)
TMP_3364(None) = SOLIDITY_CALL sstore(uint256,uint256)(s_1,3)
 i_initializer_asm_0
CONDITION i_initializer_asm_0_1
s_3(bytes32) := phi(['s_2', 's_1'])
 ! extcodesize(uint256)(address()()) < i_initializer_asm_0 >> 1 == 1
TMP_3365 = CONVERT this to address
REF_1213 -> CODESIZE TMP_3365
TMP_3366(uint256) = i_initializer_asm_0_1 >> 1
TMP_3367(bool) = TMP_3366 == 1
TMP_3368(bool) = REF_1213 < TMP_3367
TMP_3369 = UnaryType.BANG TMP_3368 
CONDITION TMP_3369
 mstore(uint256,uint256)(0x00,0xf92ee8a9)
TMP_3370(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,4180601001)
 revert(uint256,uint256)(0x1c,0x04)
TMP_3371(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 s = s << i_initializer_asm_0 << 255
TMP_3372(uint256) = i_initializer_asm_0_1 << 255
TMP_3373(bytes32) = s_1 << TMP_3372
s_2(bytes32) := TMP_3373(bytes32)
 s
CONDITION s_3
 sstore(uint256,uint256)(s,2)
TMP_3374(None) = SOLIDITY_CALL sstore(uint256,uint256)(s_3,2)
 mstore(uint256,uint256)(0x20,1)
TMP_3375(None) = SOLIDITY_CALL mstore(uint256,uint256)(32,1)
 log1(uint256,uint256,uint256)(0x20,0x20,_INTIALIZED_EVENT_SIGNATURE)
TMP_3376(None) = SOLIDITY_CALL log1(uint256,uint256,uint256)(32,32,_INTIALIZED_EVENT_SIGNATURE_4)
_INTIALIZED_EVENT_SIGNATURE_5(bytes32) := phi(['_INTIALIZED_EVENT_SIGNATURE_0', '_INTIALIZED_EVENT_SIGNATURE_4', '_INTIALIZED_EVENT_SIGNATURE_2', '_INTIALIZED_EVENT_SIGNATURE_6'])
 s = _initializableSlot()
TMP_3377(bytes32) = INTERNAL_CALL, Initializable._initializableSlot()()
s_1(bytes32) := TMP_3377(bytes32)
 version = version & 0xffffffffffffffff
TMP_3378(uint64) = version_1 & 18446744073709551615
version_2(uint64) := TMP_3378(uint64)
 i_reinitializer_asm_0 = sload(uint256)(s)
TMP_3379(uint256) = SOLIDITY_CALL sload(uint256)(s_1)
i_reinitializer_asm_0_1(uint256) := TMP_3379(uint256)
 ! i_reinitializer_asm_0 & 1 < i_reinitializer_asm_0 >> 1 < version
TMP_3380(uint256) = i_reinitializer_asm_0_1 & 1
TMP_3381(uint256) = i_reinitializer_asm_0_1 >> 1
TMP_3382(bool) = TMP_3381 < version_2
TMP_3383(bool) = TMP_3380 < TMP_3382
TMP_3384 = UnaryType.BANG TMP_3383 
CONDITION TMP_3384
 mstore(uint256,uint256)(0x00,0xf92ee8a9)
TMP_3385(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,4180601001)
 revert(uint256,uint256)(0x1c,0x04)
TMP_3386(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 sstore(uint256,uint256)(s,1 | version << 1)
TMP_3387(uint64) = version_2 << 1
TMP_3388(uint256) = 1 | TMP_3387
TMP_3389(None) = SOLIDITY_CALL sstore(uint256,uint256)(s_1,TMP_3388)
 sstore(uint256,uint256)(s,version << 1)
TMP_3390(uint64) = version_2 << 1
TMP_3391(None) = SOLIDITY_CALL sstore(uint256,uint256)(s_1,TMP_3390)
 mstore(uint256,uint256)(0x20,version)
TMP_3392(None) = SOLIDITY_CALL mstore(uint256,uint256)(32,version_2)
 log1(uint256,uint256,uint256)(0x20,0x20,_INTIALIZED_EVENT_SIGNATURE)
TMP_3393(None) = SOLIDITY_CALL log1(uint256,uint256,uint256)(32,32,_INTIALIZED_EVENT_SIGNATURE_6)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
_launches_29(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_0', '_launches_1', '_launches_9', '_launches_15', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
 _launches[msg.sender].quote == address(0)
REF_1214(ILaunchpad.LaunchData) -> _launches_29[msg.sender]
REF_1215(address) -> REF_1214.quote
TMP_3395 = CONVERT 0 to address
TMP_3396(bool) = REF_1215 == TMP_3395
CONDITION TMP_3396
 revert OnlyLaunchAsset()()
TMP_3397(None) = SOLIDITY_CALL revert OnlyLaunchAsset()()
token_1(address) := phi(['token_1', 'REF_1126'])
_launches_30(mapping(address => ILaunchpad.LaunchData)) := phi(['_launches_5', '_launches_28', '_launches_10', '_launches_7', '_launches_0', '_launches_1', '_launches_9', '_launches_15', '_launches_24', '_launches_3', '_launches_26', '_launches_29', '_launches_22', '_launches_30', '_launches_18'])
 ! _launches[token].active
REF_1216(ILaunchpad.LaunchData) -> _launches_30[token_1]
REF_1217(bool) -> REF_1216.active
TMP_3398 = UnaryType.BANG REF_1217 
CONDITION TMP_3398
 revert BondingInactive()()
TMP_3399(None) = SOLIDITY_CALL revert BondingInactive()()
account_1(address) := phi(['REF_1127', 'account_1'])
requiredRole_1(SpotOperatorRoles) := phi(['REF_1150', 'REF_1128'])
gteRouter_4(address) := phi(['gteRouter_0', 'gteRouter_1', 'gteRouter_3'])
operator_2(IOperatorPanel) := phi(['operator_1', 'operator_0'])
 msg.sender != gteRouter
TMP_3400(bool) = msg.sender != gteRouter_4
CONDITION TMP_3400
 OperatorHelperLib.onlySenderOrOperator(operator,account,requiredRole)
LIBRARY_CALL, dest:OperatorHelperLib, function:OperatorHelperLib.onlySenderOrOperator(IOperatorPanel,address,SpotOperatorRoles), arguments:['operator_2', 'account_1', 'requiredRole_1']
```
