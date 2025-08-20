

### Storage layout (TBABonus) 

```text
bonusRate uint16
assetToken IERC20
_agentAllowances mapping(uint256 => uint256)
_agentPaidAmounts mapping(uint256 => uint256)

```

#### TBABonus.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### TBABonus.distributeBonus(uint256,address,uint256) [PUBLIC]
```slithir
EXECUTOR_ROLE_1(bytes32) := phi(['EXECUTOR_ROLE_0', 'EXECUTOR_ROLE_2'])
DENOM_1(uint256) := phi(['DENOM_0', 'DENOM_2'])
bonusRate_5(uint16) := phi(['bonusRate_6', 'bonusRate_1', 'bonusRate_4', 'bonusRate_0'])
assetToken_2(IERC20) := phi(['assetToken_3', 'assetToken_0', 'assetToken_4', 'assetToken_1'])
_agentAllowances_2(mapping(uint256 => uint256)) := phi(['_agentAllowances_0', '_agentAllowances_3', '_agentAllowances_1'])
_agentPaidAmounts_3(mapping(uint256 => uint256)) := phi(['_agentPaidAmounts_6', '_agentPaidAmounts_0', '_agentPaidAmounts_5', '_agentPaidAmounts_2', '_agentPaidAmounts_4'])
 require(bool,string)(agentId > 0,Invalid agent ID)
TMP_11270(bool) = agentId_1 > 0
TMP_11271(None) = SOLIDITY_CALL require(bool,string)(TMP_11270,Invalid agent ID)
 require(bool,string)(recipient != address(0),Invalid recipient)
TMP_11272 = CONVERT 0 to address
TMP_11273(bool) = recipient_1 != TMP_11272
TMP_11274(None) = SOLIDITY_CALL require(bool,string)(TMP_11273,Invalid recipient)
 amount == 0 || ! hasRole(EXECUTOR_ROLE,msg.sender)
TMP_11275(bool) = amount_1 == 0
TMP_11276(bool) = INTERNAL_CALL, AccessControlUpgradeable.hasRole(bytes32,address)(EXECUTOR_ROLE_1,msg.sender)
TMP_11277 = UnaryType.BANG TMP_11276 
TMP_11278(bool) = TMP_11275 || TMP_11277
CONDITION TMP_11278
 allowance = _agentAllowances[agentId] - _agentPaidAmounts[agentId]
REF_4609(uint256) -> _agentAllowances_3[agentId_1]
REF_4610(uint256) -> _agentPaidAmounts_4[agentId_1]
TMP_11279(uint256) = REF_4609 (c)- REF_4610
allowance_1(uint256) := TMP_11279(uint256)
 bonus = (amount * bonusRate) / DENOM
TMP_11280(uint256) = amount_1 (c)* bonusRate_6
TMP_11281(uint256) = TMP_11280 (c)/ DENOM_2
bonus_1(uint256) := TMP_11281(uint256)
 bonus > allowance
TMP_11282(bool) = bonus_1 > allowance_1
CONDITION TMP_11282
 bonus = allowance
bonus_2(uint256) := allowance_1(uint256)
bonus_3(uint256) := phi(['bonus_1', 'bonus_2'])
 bonus > 0 && assetToken.balanceOf(address(this)) >= bonus
TMP_11283(bool) = bonus_3 > 0
TMP_11284 = CONVERT this to address
TMP_11285(uint256) = HIGH_LEVEL_CALL, dest:assetToken_3(IERC20), function:balanceOf, arguments:['TMP_11284']  
assetToken_4(IERC20) := phi(['assetToken_3', 'assetToken_1', 'assetToken_4'])
_agentPaidAmounts_5(mapping(uint256 => uint256)) := phi(['_agentPaidAmounts_5', '_agentPaidAmounts_2', '_agentPaidAmounts_6', '_agentPaidAmounts_4'])
TMP_11286(bool) = TMP_11285 >= bonus_3
TMP_11287(bool) = TMP_11283 && TMP_11286
CONDITION TMP_11287
 _agentPaidAmounts[agentId] += bonus
REF_4612(uint256) -> _agentPaidAmounts_5[agentId_1]
_agentPaidAmounts_6(mapping(uint256 => uint256)) := phi(['_agentPaidAmounts_5'])
REF_4612(-> _agentPaidAmounts_6) = REF_4612 (c)+ bonus_3
 assetToken.safeTransfer(recipient,bonus)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['assetToken_4', 'recipient_1', 'bonus_3'] 
 PaidAgent(agentId,bonus)
Emit PaidAgent(agentId_1,bonus_3)
```
#### TBABonus.initialize(address,address) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_5'])
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_4', 'ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_6'])
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 _grantRole(ADMIN_ROLE,defaultAdmin_)
TMP_11256(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(ADMIN_ROLE_3,defaultAdmin__1)
 _grantRole(DEFAULT_ADMIN_ROLE,defaultAdmin_)
TMP_11257(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_4,defaultAdmin__1)
 assetToken = IERC20(assetToken_)
TMP_11258 = CONVERT assetToken__1 to IERC20
assetToken_1(IERC20) := TMP_11258(IERC20)
 bonusRate = 3500
bonusRate_1(uint16) := 3500(uint256)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### TBABonus.setAllowances(uint256[],uint256[]) [PUBLIC]
```slithir
ADMIN_ROLE_7(bytes32) := phi(['ADMIN_ROLE_4', 'ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_6'])
_agentPaidAmounts_1(mapping(uint256 => uint256)) := phi(['_agentPaidAmounts_6', '_agentPaidAmounts_0', '_agentPaidAmounts_5', '_agentPaidAmounts_2', '_agentPaidAmounts_4'])
 require(bool,string)(agentIds.length == allowances.length,Invalid input)
REF_4602 -> LENGTH agentIds_1
REF_4603 -> LENGTH allowances_1
TMP_11262(bool) = REF_4602 == REF_4603
TMP_11263(None) = SOLIDITY_CALL require(bool,string)(TMP_11262,Invalid input)
 i = 0
i_1(uint256) := 0(uint256)
 i < agentIds.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4604 -> LENGTH agentIds_1
TMP_11264(bool) = i_2 < REF_4604
CONDITION TMP_11264
 agentId = agentIds[i]
REF_4605(uint256) -> agentIds_1[i_2]
agentId_1(uint256) := REF_4605(uint256)
 allowance = allowances[i]
REF_4606(uint256) -> allowances_1[i_2]
allowance_1(uint256) := REF_4606(uint256)
 require(bool,string)(allowance >= _agentPaidAmounts[agentId],Allowance cannot be less than paid amount)
REF_4607(uint256) -> _agentPaidAmounts_2[agentId_1]
TMP_11265(bool) = allowance_1 >= REF_4607
TMP_11266(None) = SOLIDITY_CALL require(bool,string)(TMP_11265,Allowance cannot be less than paid amount)
 _agentAllowances[agentId] = allowance
REF_4608(uint256) -> _agentAllowances_0[agentId_1]
_agentAllowances_1(mapping(uint256 => uint256)) := phi(['_agentAllowances_0'])
REF_4608(uint256) (->_agentAllowances_1) := allowance_1(uint256)
 AllowanceUpdated(agentId,allowance)
Emit AllowanceUpdated(agentId_1,allowance_1)
 i ++
TMP_11268(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_7)
```
#### LPRefund.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
 ADMIN_ROLE = keccak256(bytes)(ADMIN_ROLE)
 EXECUTOR_ROLE = keccak256(bytes)(EXECUTOR_ROLE)
role_1(bytes32) := phi(['ADMIN_ROLE_6', 'TMP_11100', 'EXECUTOR_ROLE_1', 'TMP_11105', 'TMP_11102', 'ADMIN_ROLE_8', 'TMP_11107'])
 _checkRole(role)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(role_1)
 $ = _getInitializableStorage()
TMP_11174(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_11174'])(Initializable.InitializableStorage) := TMP_11174(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_4566(bool) -> $_1 (-> ['TMP_11174'])._initializing
TMP_11175 = UnaryType.BANG REF_4566 
isTopLevelCall_1(bool) := TMP_11175(bool)
 initialized = $._initialized
REF_4567(uint64) -> $_1 (-> ['TMP_11174'])._initialized
initialized_1(uint64) := REF_4567(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_11176(bool) = initialized_1 == 0
TMP_11177(bool) = TMP_11176 && isTopLevelCall_1
initialSetup_1(bool) := TMP_11177(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_11178(bool) = initialized_1 == 1
TMP_11179 = CONVERT this to address
TMP_11180(bytes) = SOLIDITY_CALL code(address)(TMP_11179)
REF_4568 -> LENGTH TMP_11180
TMP_11181(bool) = REF_4568 == 0
TMP_11182(bool) = TMP_11178 && TMP_11181
construction_1(bool) := TMP_11182(bool)
 ! initialSetup && ! construction
TMP_11183 = UnaryType.BANG initialSetup_1 
TMP_11184 = UnaryType.BANG construction_1 
TMP_11185(bool) = TMP_11183 && TMP_11184
CONDITION TMP_11185
 revert InvalidInitialization()()
TMP_11186(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_4569(uint64) -> $_1 (-> ['TMP_11174'])._initialized
$_2 (-> ['TMP_11174'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_11174'])"])
REF_4569(uint64) (->$_2 (-> ['TMP_11174'])) := 1(uint256)
TMP_11174(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_11174'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_4570(bool) -> $_2 (-> ['TMP_11174'])._initializing
$_3 (-> ['TMP_11174'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_11174'])"])
REF_4570(bool) (->$_3 (-> ['TMP_11174'])) := True(bool)
TMP_11174(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_11174'])"])
$_4 (-> ['TMP_11174'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_11174'])", "$_2 (-> ['TMP_11174'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_4571(bool) -> $_4 (-> ['TMP_11174'])._initializing
$_5 (-> ['TMP_11174'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_11174'])"])
REF_4571(bool) (->$_5 (-> ['TMP_11174'])) := False(bool)
TMP_11174(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_11174'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_11188(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_11188'])(Initializable.InitializableStorage) := TMP_11188(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_4572(bool) -> $_1 (-> ['TMP_11188'])._initializing
REF_4573(uint64) -> $_1 (-> ['TMP_11188'])._initialized
TMP_11189(bool) = REF_4573 >= version_1
TMP_11190(bool) = REF_4572 || TMP_11189
CONDITION TMP_11190
 revert InvalidInitialization()()
TMP_11191(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_4574(uint64) -> $_1 (-> ['TMP_11188'])._initialized
$_2 (-> ['TMP_11188'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_11188'])"])
REF_4574(uint64) (->$_2 (-> ['TMP_11188'])) := version_1(uint64)
TMP_11188(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_11188'])"])
 $._initializing = true
REF_4575(bool) -> $_2 (-> ['TMP_11188'])._initializing
$_3 (-> ['TMP_11188'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_11188'])"])
REF_4575(bool) (->$_3 (-> ['TMP_11188'])) := True(bool)
TMP_11188(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_11188'])"])
 $._initializing = false
REF_4576(bool) -> $_3 (-> ['TMP_11188'])._initializing
$_4 (-> ['TMP_11188'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_11188'])"])
REF_4576(bool) (->$_4 (-> ['TMP_11188'])) := False(bool)
TMP_11188(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_11188'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
```
#### TBABonus.updateBonusRate(uint16) [PUBLIC]
```slithir
ADMIN_ROLE_5(bytes32) := phi(['ADMIN_ROLE_4', 'ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_6'])
bonusRate_2(uint16) := phi(['bonusRate_6', 'bonusRate_1', 'bonusRate_4', 'bonusRate_0'])
 oldBonusRate = bonusRate
oldBonusRate_1(uint16) := bonusRate_3(uint16)
 bonusRate = bonusRate_
bonusRate_4(uint16) := bonusRate__1(uint16)
 BonusRateUpdated(bonusRate_,oldBonusRate)
Emit BonusRateUpdated(bonusRate__1,oldBonusRate_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_5)
```
#### IERC20.balanceOf(address) [EXTERNAL]
```slithir

```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_2044(transfer) -> token_1.transfer
TMP_5413(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2044,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000550>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001210>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5413)
```
#### SafeERC20._callOptionalReturn(IERC20,bytes) [PRIVATE]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1', 'token_1'])
data_1(bytes) := phi(['approvalCall_1', 'TMP_5413', 'TMP_5415', 'TMP_5430'])
 returndata = address(token).functionCall(data)
TMP_5433 = CONVERT token_1 to address
TMP_5434(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionCall(address,bytes), arguments:['TMP_5433', 'data_1'] 
returndata_1(bytes) := TMP_5434(bytes)
 returndata.length != 0 && ! abi.decode(returndata,(bool))
REF_2054 -> LENGTH returndata_1
TMP_5435(bool) = REF_2054 != 0
TMP_5436(bool) = SOLIDITY_CALL abi.decode()(returndata_1,bool)
TMP_5437 = UnaryType.BANG TMP_5436 
TMP_5438(bool) = TMP_5435 && TMP_5437
CONDITION TMP_5438
 revert SafeERC20FailedOperation(address)(address(token))
TMP_5439 = CONVERT token_1 to address
TMP_5440(None) = SOLIDITY_CALL revert SafeERC20FailedOperation(address)(TMP_5439)
```
#### Address.functionCall(address,bytes) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0)
TMP_5457(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256)(target_1,data_1,0)
RETURN TMP_5457
```
