### Storage layout (PlumeStakingRewardTreasury) 

```text
_rewardTokens address[]
_isRewardToken mapping(address => bool)

```
#### PlumeStakingRewardTreasury._authorizeUpgrade(address) [INTERNAL]
```slithir
newImplementation_1(address) := phi(['newImplementation_1'])
UPGRADER_ROLE_12(bytes32) := phi(['UPGRADER_ROLE_11', 'UPGRADER_ROLE_13', 'UPGRADER_ROLE_0'])
 onlyRole(UPGRADER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(UPGRADER_ROLE_12)
```
#### PlumeStakingRewardTreasury.distributeReward(address,uint256,address) [EXTERNAL]
```slithir
PLUME_NATIVE_1(address) := phi(['PLUME_NATIVE_0', 'PLUME_NATIVE_3'])
DISTRIBUTOR_ROLE_11(bytes32) := phi(['DISTRIBUTOR_ROLE_0', 'DISTRIBUTOR_ROLE_10', 'DISTRIBUTOR_ROLE_13'])
_isRewardToken_4(mapping(address => bool)) := phi(['_isRewardToken_3', '_isRewardToken_6', '_isRewardToken_8', '_isRewardToken_7', '_isRewardToken_0'])
 recipient == address(0)
TMP_12702 = CONVERT 0 to address
TMP_12703(bool) = recipient_1 == TMP_12702
CONDITION TMP_12703
 revert ZeroRecipientAddress()()
TMP_12704(None) = SOLIDITY_CALL revert ZeroRecipientAddress()()
 amount == 0
TMP_12705(bool) = amount_1 == 0
CONDITION TMP_12705
 revert ZeroAmount()()
TMP_12706(None) = SOLIDITY_CALL revert ZeroAmount()()
 token == PLUME_NATIVE
TMP_12707(bool) = token_1 == PLUME_NATIVE_3
CONDITION TMP_12707
 balance = address(this).balance
TMP_12708 = CONVERT this to address
TMP_12709(uint256) = SOLIDITY_CALL balance(address)(TMP_12708)
balance_1(uint256) := TMP_12709(uint256)
 balance < amount
TMP_12710(bool) = balance_1 < amount_1
CONDITION TMP_12710
 revert InsufficientBalance(address,uint256,uint256)(token,balance,amount)
TMP_12711(None) = SOLIDITY_CALL revert InsufficientBalance(address,uint256,uint256)(token_1,balance_1,amount_1)
 (success,None) = recipient.call{value: amount}()
TUPLE_85(bool,bytes) = LOW_LEVEL_CALL, dest:recipient_1, function:call, arguments:[''] value:amount_1 
success_1(bool)= UNPACK TUPLE_85 index: 0 
 ! success
TMP_12712 = UnaryType.BANG success_1 
CONDITION TMP_12712
 revert PlumeTransferFailed(address,uint256)(recipient,amount)
TMP_12713(None) = SOLIDITY_CALL revert PlumeTransferFailed(address,uint256)(recipient_1,amount_1)
 ! _isRewardToken[token]
REF_2729(bool) -> _isRewardToken_6[token_1]
TMP_12714 = UnaryType.BANG REF_2729 
CONDITION TMP_12714
 revert TokenNotRegistered(address)(token)
TMP_12715(None) = SOLIDITY_CALL revert TokenNotRegistered(address)(token_1)
 balance_scope_0 = IERC20(token).balanceOf(address(this))
TMP_12716 = CONVERT token_1 to IERC20
TMP_12717 = CONVERT this to address
TMP_12718(uint256) = HIGH_LEVEL_CALL, dest:TMP_12716(IERC20), function:balanceOf, arguments:['TMP_12717']  
balance_scope_0_1(uint256) := TMP_12718(uint256)
 balance_scope_0 < amount
TMP_12719(bool) = balance_scope_0_1 < amount_1
CONDITION TMP_12719
 revert InsufficientBalance(address,uint256,uint256)(token,balance_scope_0,amount)
TMP_12720(None) = SOLIDITY_CALL revert InsufficientBalance(address,uint256,uint256)(token_1,balance_scope_0_1,amount_1)
 SafeERC20.safeTransfer(IERC20(token),recipient,amount)
TMP_12721 = CONVERT token_1 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_12721', 'recipient_1', 'amount_1'] 
 RewardDistributed(token,amount,recipient)
Emit RewardDistributed(token_1,amount_1,recipient_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
 onlyRole(DISTRIBUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DISTRIBUTOR_ROLE_12)
```
#### PlumeStakingRewardTreasury.getRewardTokens() [EXTERNAL]
```slithir
_rewardTokens_5(address[]) := phi(['_rewardTokens_0', '_rewardTokens_4'])
 _rewardTokens
RETURN _rewardTokens_5
```
#### PlumeStakingRewardTreasury.getBalance(address) [EXTERNAL]
```slithir
PLUME_NATIVE_4(address) := phi(['PLUME_NATIVE_0', 'PLUME_NATIVE_3'])
_isRewardToken_7(mapping(address => bool)) := phi(['_isRewardToken_3', '_isRewardToken_6', '_isRewardToken_8', '_isRewardToken_7', '_isRewardToken_0'])
 token == PLUME_NATIVE
TMP_12726(bool) = token_1 == PLUME_NATIVE_4
CONDITION TMP_12726
 address(this).balance
TMP_12727 = CONVERT this to address
TMP_12728(uint256) = SOLIDITY_CALL balance(address)(TMP_12727)
RETURN TMP_12728
 ! _isRewardToken[token]
REF_2732(bool) -> _isRewardToken_7[token_1]
TMP_12729 = UnaryType.BANG REF_2732 
CONDITION TMP_12729
 revert TokenNotRegistered(address)(token)
TMP_12730(None) = SOLIDITY_CALL revert TokenNotRegistered(address)(token_1)
 IERC20(token).balanceOf(address(this))
TMP_12731 = CONVERT token_1 to IERC20
TMP_12732 = CONVERT this to address
TMP_12733(uint256) = HIGH_LEVEL_CALL, dest:TMP_12731(IERC20), function:balanceOf, arguments:['TMP_12732']  
RETURN TMP_12733
```
#### PlumeStakingRewardTreasury.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### PlumeStakingRewardTreasury.initialize(address,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_0'])
DISTRIBUTOR_ROLE_1(bytes32) := phi(['DISTRIBUTOR_ROLE_0', 'DISTRIBUTOR_ROLE_10', 'DISTRIBUTOR_ROLE_13'])
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_11', 'ADMIN_ROLE_13', 'ADMIN_ROLE_0'])
UPGRADER_ROLE_1(bytes32) := phi(['UPGRADER_ROLE_11', 'UPGRADER_ROLE_13', 'UPGRADER_ROLE_0'])
 admin == address(0)
TMP_12676 = CONVERT 0 to address
TMP_12677(bool) = admin_1 == TMP_12676
CONDITION TMP_12677
 revert ZeroAddressToken()()
TMP_12678(None) = SOLIDITY_CALL revert ZeroAddressToken()()
 distributor == address(0)
TMP_12679 = CONVERT 0 to address
TMP_12680(bool) = distributor_1 == TMP_12679
CONDITION TMP_12680
 revert ZeroAddressToken()()
TMP_12681(None) = SOLIDITY_CALL revert ZeroAddressToken()()
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 __ReentrancyGuard_init()
INTERNAL_CALL, ReentrancyGuardUpgradeable.__ReentrancyGuard_init()()
 __UUPSUpgradeable_init()
INTERNAL_CALL, UUPSUpgradeable.__UUPSUpgradeable_init()()
 _grantRole(DEFAULT_ADMIN_ROLE,admin)
TMP_12685(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_5,admin_1)
 _grantRole(ADMIN_ROLE,admin)
TMP_12686(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(ADMIN_ROLE_6,admin_1)
 _grantRole(UPGRADER_ROLE,admin)
TMP_12687(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(UPGRADER_ROLE_7,admin_1)
 _grantRole(DISTRIBUTOR_ROLE,distributor)
TMP_12688(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DISTRIBUTOR_ROLE_8,distributor_1)
 _setRoleAdmin(DISTRIBUTOR_ROLE,ADMIN_ROLE)
INTERNAL_CALL, AccessControlUpgradeable._setRoleAdmin(bytes32,bytes32)(DISTRIBUTOR_ROLE_9,ADMIN_ROLE_9)
 _setRoleAdmin(UPGRADER_ROLE,ADMIN_ROLE)
INTERNAL_CALL, AccessControlUpgradeable._setRoleAdmin(bytes32,bytes32)(UPGRADER_ROLE_10,ADMIN_ROLE_10)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### PlumeStakingRewardTreasury.addRewardToken(address) [EXTERNAL]
```slithir
ADMIN_ROLE_12(bytes32) := phi(['ADMIN_ROLE_11', 'ADMIN_ROLE_13', 'ADMIN_ROLE_0'])
_rewardTokens_1(address[]) := phi(['_rewardTokens_0', '_rewardTokens_4'])
_isRewardToken_1(mapping(address => bool)) := phi(['_isRewardToken_3', '_isRewardToken_6', '_isRewardToken_8', '_isRewardToken_7', '_isRewardToken_0'])
 token == address(0)
TMP_12693 = CONVERT 0 to address
TMP_12694(bool) = token_1 == TMP_12693
CONDITION TMP_12694
 revert ZeroAddressToken()()
TMP_12695(None) = SOLIDITY_CALL revert ZeroAddressToken()()
 _isRewardToken[token]
REF_2723(bool) -> _isRewardToken_2[token_1]
CONDITION REF_2723
 revert TokenAlreadyAdded(address)(token)
TMP_12696(None) = SOLIDITY_CALL revert TokenAlreadyAdded(address)(token_1)
 _rewardTokens.push(token)
REF_2725 -> LENGTH _rewardTokens_2
TMP_12698(uint256) := REF_2725(uint256)
TMP_12699(uint256) = TMP_12698 (c)+ 1
_rewardTokens_3(address[]) := phi(['_rewardTokens_2'])
REF_2725(uint256) (->_rewardTokens_3) := TMP_12699(uint256)
REF_2726(address) -> _rewardTokens_3[TMP_12698]
_rewardTokens_4(address[]) := phi(['_rewardTokens_3'])
REF_2726(address) (->_rewardTokens_4) := token_1(address)
 _isRewardToken[token] = true
REF_2727(bool) -> _isRewardToken_2[token_1]
_isRewardToken_3(mapping(address => bool)) := phi(['_isRewardToken_2'])
REF_2727(bool) (->_isRewardToken_3) := True(bool)
 RewardTokenAdded(token)
Emit RewardTokenAdded(token_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_12)
```
#### PlumeStakingRewardTreasury.isRewardToken(address) [EXTERNAL]
```slithir
_isRewardToken_8(mapping(address => bool)) := phi(['_isRewardToken_3', '_isRewardToken_6', '_isRewardToken_8', '_isRewardToken_7', '_isRewardToken_0'])
 _isRewardToken[token]
REF_2734(bool) -> _isRewardToken_8[token_1]
RETURN REF_2734
```
#### PlumeStakingRewardTreasury.receive() [EXTERNAL]
```slithir
 PlumeReceived(msg.sender,msg.value)
Emit PlumeReceived(msg.sender,msg.value)
```

