### Storage layout (PlumeStakingRewardTreasury) 

```text
_rewardTokens address[]
_isRewardToken mapping(address => bool)

```
#### PlumeStakingRewardTreasury._authorizeUpgrade(address) [INTERNAL]
```slithir
newImplementation_1(address) := phi(['newImplementation_1'])
UPGRADER_ROLE_12(bytes32) := phi(['UPGRADER_ROLE_11', 'UPGRADER_ROLE_0', 'UPGRADER_ROLE_13'])
 onlyRole(UPGRADER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(UPGRADER_ROLE_12)
```
#### PlumeStakingRewardTreasury.distributeReward(address,uint256,address) [EXTERNAL]
```slithir
PLUME_NATIVE_1(address) := phi(['PLUME_NATIVE_0', 'PLUME_NATIVE_3'])
DISTRIBUTOR_ROLE_11(bytes32) := phi(['DISTRIBUTOR_ROLE_10', 'DISTRIBUTOR_ROLE_0', 'DISTRIBUTOR_ROLE_13'])
_isRewardToken_4(mapping(address => bool)) := phi(['_isRewardToken_0', '_isRewardToken_3', '_isRewardToken_6', '_isRewardToken_8', '_isRewardToken_7'])
 recipient == address(0)
TMP_12712 = CONVERT 0 to address
TMP_12713(bool) = recipient_1 == TMP_12712
CONDITION TMP_12713
 revert ZeroRecipientAddress()()
TMP_12714(None) = SOLIDITY_CALL revert ZeroRecipientAddress()()
 amount == 0
TMP_12715(bool) = amount_1 == 0
CONDITION TMP_12715
 revert ZeroAmount()()
TMP_12716(None) = SOLIDITY_CALL revert ZeroAmount()()
 token == PLUME_NATIVE
TMP_12717(bool) = token_1 == PLUME_NATIVE_3
CONDITION TMP_12717
 balance = address(this).balance
TMP_12718 = CONVERT this to address
TMP_12719(uint256) = SOLIDITY_CALL balance(address)(TMP_12718)
balance_1(uint256) := TMP_12719(uint256)
 balance < amount
TMP_12720(bool) = balance_1 < amount_1
CONDITION TMP_12720
 revert InsufficientBalance(address,uint256,uint256)(token,balance,amount)
TMP_12721(None) = SOLIDITY_CALL revert InsufficientBalance(address,uint256,uint256)(token_1,balance_1,amount_1)
 (success,None) = recipient.call{value: amount}()
TUPLE_85(bool,bytes) = LOW_LEVEL_CALL, dest:recipient_1, function:call, arguments:[''] value:amount_1 
success_1(bool)= UNPACK TUPLE_85 index: 0 
 ! success
TMP_12722 = UnaryType.BANG success_1 
CONDITION TMP_12722
 revert PlumeTransferFailed(address,uint256)(recipient,amount)
TMP_12723(None) = SOLIDITY_CALL revert PlumeTransferFailed(address,uint256)(recipient_1,amount_1)
 ! _isRewardToken[token]
REF_2734(bool) -> _isRewardToken_6[token_1]
TMP_12724 = UnaryType.BANG REF_2734 
CONDITION TMP_12724
 revert TokenNotRegistered(address)(token)
TMP_12725(None) = SOLIDITY_CALL revert TokenNotRegistered(address)(token_1)
 balance_scope_0 = IERC20(token).balanceOf(address(this))
TMP_12726 = CONVERT token_1 to IERC20
TMP_12727 = CONVERT this to address
TMP_12728(uint256) = HIGH_LEVEL_CALL, dest:TMP_12726(IERC20), function:balanceOf, arguments:['TMP_12727']  
balance_scope_0_1(uint256) := TMP_12728(uint256)
 balance_scope_0 < amount
TMP_12729(bool) = balance_scope_0_1 < amount_1
CONDITION TMP_12729
 revert InsufficientBalance(address,uint256,uint256)(token,balance_scope_0,amount)
TMP_12730(None) = SOLIDITY_CALL revert InsufficientBalance(address,uint256,uint256)(token_1,balance_scope_0_1,amount_1)
 SafeERC20.safeTransfer(IERC20(token),recipient,amount)
TMP_12731 = CONVERT token_1 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_12731', 'recipient_1', 'amount_1'] 
 RewardDistributed(token,amount,recipient)
Emit RewardDistributed(token_1,amount_1,recipient_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
 onlyRole(DISTRIBUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DISTRIBUTOR_ROLE_12)
```
#### PlumeStakingRewardTreasury.getRewardTokens() [EXTERNAL]
```slithir
_rewardTokens_5(address[]) := phi(['_rewardTokens_4', '_rewardTokens_0'])
 _rewardTokens
RETURN _rewardTokens_5
```
#### PlumeStakingRewardTreasury.getBalance(address) [EXTERNAL]
```slithir
PLUME_NATIVE_4(address) := phi(['PLUME_NATIVE_0', 'PLUME_NATIVE_3'])
_isRewardToken_7(mapping(address => bool)) := phi(['_isRewardToken_0', '_isRewardToken_3', '_isRewardToken_6', '_isRewardToken_8', '_isRewardToken_7'])
 token == PLUME_NATIVE
TMP_12736(bool) = token_1 == PLUME_NATIVE_4
CONDITION TMP_12736
 address(this).balance
TMP_12737 = CONVERT this to address
TMP_12738(uint256) = SOLIDITY_CALL balance(address)(TMP_12737)
RETURN TMP_12738
 ! _isRewardToken[token]
REF_2737(bool) -> _isRewardToken_7[token_1]
TMP_12739 = UnaryType.BANG REF_2737 
CONDITION TMP_12739
 revert TokenNotRegistered(address)(token)
TMP_12740(None) = SOLIDITY_CALL revert TokenNotRegistered(address)(token_1)
 IERC20(token).balanceOf(address(this))
TMP_12741 = CONVERT token_1 to IERC20
TMP_12742 = CONVERT this to address
TMP_12743(uint256) = HIGH_LEVEL_CALL, dest:TMP_12741(IERC20), function:balanceOf, arguments:['TMP_12742']  
RETURN TMP_12743
```
#### PlumeStakingRewardTreasury.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### PlumeStakingRewardTreasury.initialize(address,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6'])
DISTRIBUTOR_ROLE_1(bytes32) := phi(['DISTRIBUTOR_ROLE_10', 'DISTRIBUTOR_ROLE_0', 'DISTRIBUTOR_ROLE_13'])
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_13', 'ADMIN_ROLE_0', 'ADMIN_ROLE_11'])
UPGRADER_ROLE_1(bytes32) := phi(['UPGRADER_ROLE_11', 'UPGRADER_ROLE_0', 'UPGRADER_ROLE_13'])
 admin == address(0)
TMP_12686 = CONVERT 0 to address
TMP_12687(bool) = admin_1 == TMP_12686
CONDITION TMP_12687
 revert ZeroAddressToken()()
TMP_12688(None) = SOLIDITY_CALL revert ZeroAddressToken()()
 distributor == address(0)
TMP_12689 = CONVERT 0 to address
TMP_12690(bool) = distributor_1 == TMP_12689
CONDITION TMP_12690
 revert ZeroAddressToken()()
TMP_12691(None) = SOLIDITY_CALL revert ZeroAddressToken()()
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 __ReentrancyGuard_init()
INTERNAL_CALL, ReentrancyGuardUpgradeable.__ReentrancyGuard_init()()
 __UUPSUpgradeable_init()
INTERNAL_CALL, UUPSUpgradeable.__UUPSUpgradeable_init()()
 _grantRole(DEFAULT_ADMIN_ROLE,admin)
TMP_12695(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_5,admin_1)
 _grantRole(ADMIN_ROLE,admin)
TMP_12696(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(ADMIN_ROLE_6,admin_1)
 _grantRole(UPGRADER_ROLE,admin)
TMP_12697(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(UPGRADER_ROLE_7,admin_1)
 _grantRole(DISTRIBUTOR_ROLE,distributor)
TMP_12698(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DISTRIBUTOR_ROLE_8,distributor_1)
 _setRoleAdmin(DISTRIBUTOR_ROLE,ADMIN_ROLE)
INTERNAL_CALL, AccessControlUpgradeable._setRoleAdmin(bytes32,bytes32)(DISTRIBUTOR_ROLE_9,ADMIN_ROLE_9)
 _setRoleAdmin(UPGRADER_ROLE,ADMIN_ROLE)
INTERNAL_CALL, AccessControlUpgradeable._setRoleAdmin(bytes32,bytes32)(UPGRADER_ROLE_10,ADMIN_ROLE_10)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### PlumeStakingRewardTreasury.addRewardToken(address) [EXTERNAL]
```slithir
ADMIN_ROLE_12(bytes32) := phi(['ADMIN_ROLE_13', 'ADMIN_ROLE_0', 'ADMIN_ROLE_11'])
_rewardTokens_1(address[]) := phi(['_rewardTokens_4', '_rewardTokens_0'])
_isRewardToken_1(mapping(address => bool)) := phi(['_isRewardToken_0', '_isRewardToken_3', '_isRewardToken_6', '_isRewardToken_8', '_isRewardToken_7'])
 token == address(0)
TMP_12703 = CONVERT 0 to address
TMP_12704(bool) = token_1 == TMP_12703
CONDITION TMP_12704
 revert ZeroAddressToken()()
TMP_12705(None) = SOLIDITY_CALL revert ZeroAddressToken()()
 _isRewardToken[token]
REF_2728(bool) -> _isRewardToken_2[token_1]
CONDITION REF_2728
 revert TokenAlreadyAdded(address)(token)
TMP_12706(None) = SOLIDITY_CALL revert TokenAlreadyAdded(address)(token_1)
 _rewardTokens.push(token)
REF_2730 -> LENGTH _rewardTokens_2
TMP_12708(uint256) := REF_2730(uint256)
TMP_12709(uint256) = TMP_12708 (c)+ 1
_rewardTokens_3(address[]) := phi(['_rewardTokens_2'])
REF_2730(uint256) (->_rewardTokens_3) := TMP_12709(uint256)
REF_2731(address) -> _rewardTokens_3[TMP_12708]
_rewardTokens_4(address[]) := phi(['_rewardTokens_3'])
REF_2731(address) (->_rewardTokens_4) := token_1(address)
 _isRewardToken[token] = true
REF_2732(bool) -> _isRewardToken_2[token_1]
_isRewardToken_3(mapping(address => bool)) := phi(['_isRewardToken_2'])
REF_2732(bool) (->_isRewardToken_3) := True(bool)
 RewardTokenAdded(token)
Emit RewardTokenAdded(token_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_12)
```
#### PlumeStakingRewardTreasury.isRewardToken(address) [EXTERNAL]
```slithir
_isRewardToken_8(mapping(address => bool)) := phi(['_isRewardToken_0', '_isRewardToken_3', '_isRewardToken_6', '_isRewardToken_8', '_isRewardToken_7'])
 _isRewardToken[token]
REF_2739(bool) -> _isRewardToken_8[token_1]
RETURN REF_2739
```
#### PlumeStakingRewardTreasury.receive() [EXTERNAL]
```slithir
 PlumeReceived(msg.sender,msg.value)
Emit PlumeReceived(msg.sender,msg.value)
```

