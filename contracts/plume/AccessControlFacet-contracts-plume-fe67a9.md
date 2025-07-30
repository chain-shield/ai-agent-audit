

#### AccessControlFacet.hasRole(bytes32,address) [EXTERNAL]
```slithir
 _hasRole(role,account)
TMP_12811(bool) = INTERNAL_CALL, AccessControlInternal._hasRole(bytes32,address)(role_1,account_1)
RETURN TMP_12811
```
#### AccessControlFacet.getRoleAdmin(bytes32) [EXTERNAL]
```slithir
 _getRoleAdmin(role)
TMP_12812(bytes32) = INTERNAL_CALL, AccessControlInternal._getRoleAdmin(bytes32)(role_1)
RETURN TMP_12812
```
#### AccessControlFacet.grantRole(bytes32,address) [EXTERNAL]
```slithir
 _grantRole(role,account)
INTERNAL_CALL, AccessControlInternal._grantRole(bytes32,address)(role_1,account_1)
 onlyRole(_getRoleAdmin(role))
TMP_12814(bytes32) = INTERNAL_CALL, AccessControlInternal._getRoleAdmin(bytes32)(role_1)
MODIFIER_CALL, AccessControlInternal.onlyRole(bytes32)(TMP_12814)
 onlyRole(_getRoleAdmin(role))
TMP_12816(bytes32) = INTERNAL_CALL, AccessControlInternal._getRoleAdmin(bytes32)(role_1)
MODIFIER_CALL, AccessControlInternal.onlyRole(bytes32)(TMP_12816)
```
#### AccessControlFacet.revokeRole(bytes32,address) [EXTERNAL]
```slithir
 _revokeRole(role,account)
INTERNAL_CALL, AccessControlInternal._revokeRole(bytes32,address)(role_1,account_1)
 onlyRole(_getRoleAdmin(role))
TMP_12819(bytes32) = INTERNAL_CALL, AccessControlInternal._getRoleAdmin(bytes32)(role_1)
MODIFIER_CALL, AccessControlInternal.onlyRole(bytes32)(TMP_12819)
 onlyRole(_getRoleAdmin(role))
TMP_12821(bytes32) = INTERNAL_CALL, AccessControlInternal._getRoleAdmin(bytes32)(role_1)
MODIFIER_CALL, AccessControlInternal.onlyRole(bytes32)(TMP_12821)
```
#### AccessControlFacet.renounceRole(bytes32,address) [EXTERNAL]
```slithir
 require(bool,string)(account == msg.sender,AccessControl: can only renounce roles for self)
TMP_12823(bool) = account_1 == msg.sender
TMP_12824(None) = SOLIDITY_CALL require(bool,string)(TMP_12823,AccessControl: can only renounce roles for self)
 _renounceRole(role)
INTERNAL_CALL, AccessControlInternal._renounceRole(bytes32)(role_1)
```
#### AccessControlFacet.setRoleAdmin(bytes32,bytes32) [EXTERNAL]
```slithir
ADMIN_ROLE_9(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0'])
 _setRoleAdmin(role,adminRole)
INTERNAL_CALL, AccessControlInternal._setRoleAdmin(bytes32,bytes32)(role_1,adminRole_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlInternal.onlyRole(bytes32)(ADMIN_ROLE_9)
```
#### AccessControlFacet.initializeAccessControl() [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_2'])
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0'])
UPGRADER_ROLE_1(bytes32) := phi(['UPGRADER_ROLE_0', 'UPGRADER_ROLE_9'])
VALIDATOR_ROLE_1(bytes32) := phi(['VALIDATOR_ROLE_0', 'VALIDATOR_ROLE_7'])
REWARD_MANAGER_ROLE_1(bytes32) := phi(['REWARD_MANAGER_ROLE_0', 'REWARD_MANAGER_ROLE_10'])
TIMELOCK_ROLE_1(bytes32) := phi(['TIMELOCK_ROLE_5', 'TIMELOCK_ROLE_0'])
 $ = PlumeStakingStorage.layout()
TMP_12799(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12799'])(PlumeStakingStorage.Layout) := TMP_12799(PlumeStakingStorage.Layout)
 require(bool,string)(! $.accessControlFacetInitialized,ACF: init)
REF_2788(bool) -> $_1 (-> ['TMP_12799']).accessControlFacetInitialized
TMP_12800 = UnaryType.BANG REF_2788 
TMP_12801(None) = SOLIDITY_CALL require(bool,string)(TMP_12800,ACF: init)
 _grantRole(DEFAULT_ADMIN_ROLE,msg.sender)
INTERNAL_CALL, AccessControlInternal._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_1,msg.sender)
 _grantRole(ADMIN_ROLE,msg.sender)
INTERNAL_CALL, AccessControlInternal._grantRole(bytes32,address)(ADMIN_ROLE_2,msg.sender)
 _setRoleAdmin(ADMIN_ROLE,ADMIN_ROLE)
INTERNAL_CALL, AccessControlInternal._setRoleAdmin(bytes32,bytes32)(ADMIN_ROLE_3,ADMIN_ROLE_3)
 _setRoleAdmin(TIMELOCK_ROLE,ADMIN_ROLE)
INTERNAL_CALL, AccessControlInternal._setRoleAdmin(bytes32,bytes32)(TIMELOCK_ROLE_4,ADMIN_ROLE_4)
 _setRoleAdmin(UPGRADER_ROLE,ADMIN_ROLE)
INTERNAL_CALL, AccessControlInternal._setRoleAdmin(bytes32,bytes32)(UPGRADER_ROLE_5,ADMIN_ROLE_5)
 _setRoleAdmin(VALIDATOR_ROLE,ADMIN_ROLE)
INTERNAL_CALL, AccessControlInternal._setRoleAdmin(bytes32,bytes32)(VALIDATOR_ROLE_6,ADMIN_ROLE_6)
 _setRoleAdmin(REWARD_MANAGER_ROLE,ADMIN_ROLE)
INTERNAL_CALL, AccessControlInternal._setRoleAdmin(bytes32,bytes32)(REWARD_MANAGER_ROLE_7,ADMIN_ROLE_7)
 _grantRole(UPGRADER_ROLE,msg.sender)
INTERNAL_CALL, AccessControlInternal._grantRole(bytes32,address)(UPGRADER_ROLE_8,msg.sender)
 _grantRole(REWARD_MANAGER_ROLE,msg.sender)
INTERNAL_CALL, AccessControlInternal._grantRole(bytes32,address)(REWARD_MANAGER_ROLE_9,msg.sender)
 $.accessControlFacetInitialized = true
REF_2789(bool) -> $_1 (-> ['TMP_12799']).accessControlFacetInitialized
$_2 (-> ['TMP_12799'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12799'])"])
REF_2789(bool) (->$_2 (-> ['TMP_12799'])) := True(bool)
TMP_12799(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12799'])"])
```
#### PlumeStakingRewardTreasury.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
 UPGRADE_INTERFACE_VERSION = 5.0.0
 PLUME_NATIVE = 0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE
 DISTRIBUTOR_ROLE = keccak256(bytes)(DISTRIBUTOR_ROLE)
 ADMIN_ROLE = keccak256(bytes)(ADMIN_ROLE)
 UPGRADER_ROLE = keccak256(bytes)(UPGRADER_ROLE)
 _checkProxy()
INTERNAL_CALL, UUPSUpgradeable._checkProxy()()
 _checkNotDelegated()
INTERNAL_CALL, UUPSUpgradeable._checkNotDelegated()()
 $ = _getInitializableStorage()
TMP_12750(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_12750'])(Initializable.InitializableStorage) := TMP_12750(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_2740(bool) -> $_1 (-> ['TMP_12750'])._initializing
TMP_12751 = UnaryType.BANG REF_2740 
isTopLevelCall_1(bool) := TMP_12751(bool)
 initialized = $._initialized
REF_2741(uint64) -> $_1 (-> ['TMP_12750'])._initialized
initialized_1(uint64) := REF_2741(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_12752(bool) = initialized_1 == 0
TMP_12753(bool) = TMP_12752 && isTopLevelCall_1
initialSetup_1(bool) := TMP_12753(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_12754(bool) = initialized_1 == 1
TMP_12755 = CONVERT this to address
TMP_12756(bytes) = SOLIDITY_CALL code(address)(TMP_12755)
REF_2742 -> LENGTH TMP_12756
TMP_12757(bool) = REF_2742 == 0
TMP_12758(bool) = TMP_12754 && TMP_12757
construction_1(bool) := TMP_12758(bool)
 ! initialSetup && ! construction
TMP_12759 = UnaryType.BANG initialSetup_1 
TMP_12760 = UnaryType.BANG construction_1 
TMP_12761(bool) = TMP_12759 && TMP_12760
CONDITION TMP_12761
 revert InvalidInitialization()()
TMP_12762(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_2743(uint64) -> $_1 (-> ['TMP_12750'])._initialized
$_2 (-> ['TMP_12750'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_12750'])"])
REF_2743(uint64) (->$_2 (-> ['TMP_12750'])) := 1(uint256)
TMP_12750(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12750'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_2744(bool) -> $_2 (-> ['TMP_12750'])._initializing
$_3 (-> ['TMP_12750'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12750'])"])
REF_2744(bool) (->$_3 (-> ['TMP_12750'])) := True(bool)
TMP_12750(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12750'])"])
$_4 (-> ['TMP_12750'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12750'])", "$_2 (-> ['TMP_12750'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_2745(bool) -> $_4 (-> ['TMP_12750'])._initializing
$_5 (-> ['TMP_12750'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_12750'])"])
REF_2745(bool) (->$_5 (-> ['TMP_12750'])) := False(bool)
TMP_12750(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_12750'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_12764(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_12764'])(Initializable.InitializableStorage) := TMP_12764(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_2746(bool) -> $_1 (-> ['TMP_12764'])._initializing
REF_2747(uint64) -> $_1 (-> ['TMP_12764'])._initialized
TMP_12765(bool) = REF_2747 >= version_1
TMP_12766(bool) = REF_2746 || TMP_12765
CONDITION TMP_12766
 revert InvalidInitialization()()
TMP_12767(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_2748(uint64) -> $_1 (-> ['TMP_12764'])._initialized
$_2 (-> ['TMP_12764'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_12764'])"])
REF_2748(uint64) (->$_2 (-> ['TMP_12764'])) := version_1(uint64)
TMP_12764(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12764'])"])
 $._initializing = true
REF_2749(bool) -> $_2 (-> ['TMP_12764'])._initializing
$_3 (-> ['TMP_12764'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12764'])"])
REF_2749(bool) (->$_3 (-> ['TMP_12764'])) := True(bool)
TMP_12764(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12764'])"])
 $._initializing = false
REF_2750(bool) -> $_3 (-> ['TMP_12764'])._initializing
$_4 (-> ['TMP_12764'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12764'])"])
REF_2750(bool) (->$_4 (-> ['TMP_12764'])) := False(bool)
TMP_12764(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_12764'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
 _nonReentrantBefore()
INTERNAL_CALL, ReentrancyGuardUpgradeable._nonReentrantBefore()()
 _nonReentrantAfter()
INTERNAL_CALL, ReentrancyGuardUpgradeable._nonReentrantAfter()()
role_1(bytes32) := phi(['TMP_12656', 'UPGRADER_ROLE_12', 'TMP_12661', 'DISTRIBUTOR_ROLE_12', 'ADMIN_ROLE_12', 'TMP_12654', 'TMP_12659'])
 _checkRole(role)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(role_1)
```
#### PlumeStakingStorage.layout() [INTERNAL]
```slithir
DIAMOND_STORAGE_POSITION_1(bytes32) := phi(['DIAMOND_STORAGE_POSITION_0'])
 position = DIAMOND_STORAGE_POSITION
position_1(bytes32) := DIAMOND_STORAGE_POSITION_1(bytes32)
 l = position
l_1 (-> ['position'])(PlumeStakingStorage.Layout) := position_1(bytes32)
 l
RETURN l_1 (-> ['position'])
```
