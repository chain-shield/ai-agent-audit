


#### ManagementFacet.setMinStakeAmount(uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_12853(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12853'])(PlumeStakingStorage.Layout) := TMP_12853(PlumeStakingStorage.Layout)
 oldAmount = $.minStakeAmount
REF_2808(uint256) -> $_1 (-> ['TMP_12853']).minStakeAmount
oldAmount_1(uint256) := REF_2808(uint256)
 _minStakeAmount == 0
TMP_12854(bool) = _minStakeAmount_1 == 0
CONDITION TMP_12854
 revert InvalidAmount(uint256)(_minStakeAmount)
TMP_12855(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(_minStakeAmount_1)
 $.minStakeAmount = _minStakeAmount
REF_2809(uint256) -> $_1 (-> ['TMP_12853']).minStakeAmount
$_2 (-> ['TMP_12853'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12853'])"])
REF_2809(uint256) (->$_2 (-> ['TMP_12853'])) := _minStakeAmount_1(uint256)
TMP_12853(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12853'])"])
 MinStakeAmountSet(_minStakeAmount)
Emit MinStakeAmountSet(_minStakeAmount_1)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2810(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2810)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2811(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2811)
```
#### ManagementFacet.setCooldownInterval(uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_12859(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12859'])(PlumeStakingStorage.Layout) := TMP_12859(PlumeStakingStorage.Layout)
 interval == 0
TMP_12860(bool) = interval_1 == 0
CONDITION TMP_12860
 revert InvalidInterval(uint256)(interval)
TMP_12861(None) = SOLIDITY_CALL revert InvalidInterval(uint256)(interval_1)
 $.maxSlashVoteDurationInSeconds != 0 && interval <= $.maxSlashVoteDurationInSeconds
REF_2813(uint256) -> $_1 (-> ['TMP_12859']).maxSlashVoteDurationInSeconds
TMP_12862(bool) = REF_2813 != 0
REF_2814(uint256) -> $_1 (-> ['TMP_12859']).maxSlashVoteDurationInSeconds
TMP_12863(bool) = interval_1 <= REF_2814
TMP_12864(bool) = TMP_12862 && TMP_12863
CONDITION TMP_12864
 revert CooldownTooShortForSlashVote(uint256,uint256)(interval,$.maxSlashVoteDurationInSeconds)
REF_2815(uint256) -> $_1 (-> ['TMP_12859']).maxSlashVoteDurationInSeconds
TMP_12865(None) = SOLIDITY_CALL revert CooldownTooShortForSlashVote(uint256,uint256)(interval_1,REF_2815)
 $.cooldownInterval = interval
REF_2816(uint256) -> $_1 (-> ['TMP_12859']).cooldownInterval
$_2 (-> ['TMP_12859'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12859'])"])
REF_2816(uint256) (->$_2 (-> ['TMP_12859'])) := interval_1(uint256)
TMP_12859(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12859'])"])
 CooldownIntervalSet(interval)
Emit CooldownIntervalSet(interval_1)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2817(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2817)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2818(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2818)
```
#### ManagementFacet.adminWithdraw(address,uint256,address) [EXTERNAL]
```slithir
 token == address(0)
TMP_12869 = CONVERT 0 to address
TMP_12870(bool) = token_1 == TMP_12869
CONDITION TMP_12870
 revert ZeroAddress(string)(token)
TMP_12871(None) = SOLIDITY_CALL revert ZeroAddress(string)(token)
 recipient == address(0)
TMP_12872 = CONVERT 0 to address
TMP_12873(bool) = recipient_1 == TMP_12872
CONDITION TMP_12873
 revert ZeroAddress(string)(recipient)
TMP_12874(None) = SOLIDITY_CALL revert ZeroAddress(string)(recipient)
 amount == 0
TMP_12875(bool) = amount_1 == 0
CONDITION TMP_12875
 revert InvalidAmount(uint256)(amount)
TMP_12876(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(amount_1)
 token == PlumeStakingStorage.PLUME_NATIVE
REF_2819(address) -> PlumeStakingStorage.PLUME_NATIVE
TMP_12877(bool) = token_1 == REF_2819
CONDITION TMP_12877
 balance = address(this).balance
TMP_12878 = CONVERT this to address
TMP_12879(uint256) = SOLIDITY_CALL balance(address)(TMP_12878)
balance_1(uint256) := TMP_12879(uint256)
 amount > balance
TMP_12880(bool) = amount_1 > balance_1
CONDITION TMP_12880
 revert InsufficientFunds(uint256,uint256)(balance,amount)
TMP_12881(None) = SOLIDITY_CALL revert InsufficientFunds(uint256,uint256)(balance_1,amount_1)
 (success,None) = address(recipient).call{value: amount}()
TMP_12882 = CONVERT recipient_1 to address
TUPLE_86(bool,bytes) = LOW_LEVEL_CALL, dest:TMP_12882, function:call, arguments:[''] value:amount_1 
success_1(bool)= UNPACK TUPLE_86 index: 0 
 ! success
TMP_12883 = UnaryType.BANG success_1 
CONDITION TMP_12883
 revert AdminTransferFailed()()
TMP_12884(None) = SOLIDITY_CALL revert AdminTransferFailed()()
 erc20Token = IERC20(token)
TMP_12885 = CONVERT token_1 to IERC20
erc20Token_1(IERC20) := TMP_12885(IERC20)
 balance_scope_0 = erc20Token.balanceOf(address(this))
TMP_12886 = CONVERT this to address
TMP_12887(uint256) = HIGH_LEVEL_CALL, dest:erc20Token_1(IERC20), function:balanceOf, arguments:['TMP_12886']  
balance_scope_0_1(uint256) := TMP_12887(uint256)
 amount > balance_scope_0
TMP_12888(bool) = amount_1 > balance_scope_0_1
CONDITION TMP_12888
 revert InsufficientFunds(uint256,uint256)(balance_scope_0,amount)
TMP_12889(None) = SOLIDITY_CALL revert InsufficientFunds(uint256,uint256)(balance_scope_0_1,amount_1)
 erc20Token.safeTransfer(recipient,amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['erc20Token_1', 'recipient_1', 'amount_1'] 
 AdminWithdraw(token,amount,recipient)
Emit AdminWithdraw(token_1,amount_1,recipient_1)
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_2823(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2823)
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_2824(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2824)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### ManagementFacet.getMinStakeAmount() [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().minStakeAmount
TMP_12895(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_2826(uint256) -> TMP_12895.minStakeAmount
RETURN REF_2826
```
#### ManagementFacet.getCooldownInterval() [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().cooldownInterval
TMP_12896(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_2828(uint256) -> TMP_12896.cooldownInterval
RETURN REF_2828
```
#### ManagementFacet.setMaxSlashVoteDuration(uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_12897(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12897'])(PlumeStakingStorage.Layout) := TMP_12897(PlumeStakingStorage.Layout)
 duration == 0
TMP_12898(bool) = duration_1 == 0
CONDITION TMP_12898
 revert InvalidInterval(uint256)(duration)
TMP_12899(None) = SOLIDITY_CALL revert InvalidInterval(uint256)(duration_1)
 $.cooldownInterval != 0 && duration >= $.cooldownInterval
REF_2830(uint256) -> $_1 (-> ['TMP_12897']).cooldownInterval
TMP_12900(bool) = REF_2830 != 0
REF_2831(uint256) -> $_1 (-> ['TMP_12897']).cooldownInterval
TMP_12901(bool) = duration_1 >= REF_2831
TMP_12902(bool) = TMP_12900 && TMP_12901
CONDITION TMP_12902
 revert SlashVoteDurationTooLongForCooldown(uint256,uint256)(duration,$.cooldownInterval)
REF_2832(uint256) -> $_1 (-> ['TMP_12897']).cooldownInterval
TMP_12903(None) = SOLIDITY_CALL revert SlashVoteDurationTooLongForCooldown(uint256,uint256)(duration_1,REF_2832)
 $.maxSlashVoteDurationInSeconds = duration
REF_2833(uint256) -> $_1 (-> ['TMP_12897']).maxSlashVoteDurationInSeconds
$_2 (-> ['TMP_12897'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12897'])"])
REF_2833(uint256) (->$_2 (-> ['TMP_12897'])) := duration_1(uint256)
TMP_12897(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12897'])"])
 MaxSlashVoteDurationSet(duration)
Emit MaxSlashVoteDurationSet(duration_1)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2834(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2834)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2835(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2835)
```
#### ManagementFacet.setMaxAllowedValidatorCommission(uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_12907(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12907'])(PlumeStakingStorage.Layout) := TMP_12907(PlumeStakingStorage.Layout)
 newMaxRate > PlumeStakingStorage.REWARD_PRECISION / 2
REF_2837(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_12908(uint256) = REF_2837 (c)/ 2
TMP_12909(bool) = newMaxRate_1 > TMP_12908
CONDITION TMP_12909
 revert InvalidMaxCommissionRate(uint256,uint256)(newMaxRate,PlumeStakingStorage.REWARD_PRECISION / 2)
REF_2838(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_12910(uint256) = REF_2838 (c)/ 2
TMP_12911(None) = SOLIDITY_CALL revert InvalidMaxCommissionRate(uint256,uint256)(newMaxRate_1,TMP_12910)
 oldMaxRate = $.maxAllowedValidatorCommission
REF_2839(uint256) -> $_1 (-> ['TMP_12907']).maxAllowedValidatorCommission
oldMaxRate_1(uint256) := REF_2839(uint256)
 $.maxAllowedValidatorCommission = newMaxRate
REF_2840(uint256) -> $_1 (-> ['TMP_12907']).maxAllowedValidatorCommission
$_2 (-> ['TMP_12907'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12907'])"])
REF_2840(uint256) (->$_2 (-> ['TMP_12907'])) := newMaxRate_1(uint256)
TMP_12907(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12907'])"])
 MaxAllowedValidatorCommissionSet(oldMaxRate,newMaxRate)
Emit MaxAllowedValidatorCommissionSet(oldMaxRate_1,newMaxRate_1)
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_2841(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2841)
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_2842(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2842)
```
#### ManagementFacet.adminClearValidatorRecord(address,uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_12915(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12915'])(PlumeStakingStorage.Layout) := TMP_12915(PlumeStakingStorage.Layout)
 user == address(0)
TMP_12916 = CONVERT 0 to address
TMP_12917(bool) = user_1 == TMP_12916
CONDITION TMP_12917
 revert ZeroAddress(string)(user)
TMP_12918(None) = SOLIDITY_CALL revert ZeroAddress(string)(user)
 ! $.validatorExists[slashedValidatorId]
REF_2844(mapping(uint16 => bool)) -> $_1 (-> ['TMP_12915']).validatorExists
REF_2845(bool) -> REF_2844[slashedValidatorId_1]
TMP_12919 = UnaryType.BANG REF_2845 
CONDITION TMP_12919
 revert ValidatorDoesNotExist(uint16)(slashedValidatorId)
TMP_12920(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(slashedValidatorId_1)
 ! $.validators[slashedValidatorId].slashed
REF_2846(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_12915']).validators
REF_2847(PlumeStakingStorage.ValidatorInfo) -> REF_2846[slashedValidatorId_1]
REF_2848(bool) -> REF_2847.slashed
TMP_12921 = UnaryType.BANG REF_2848 
CONDITION TMP_12921
 revert ValidatorNotSlashed(uint16)(slashedValidatorId)
TMP_12922(None) = SOLIDITY_CALL revert ValidatorNotSlashed(uint16)(slashedValidatorId_1)
 userActiveStakeToClear = $.userValidatorStakes[user][slashedValidatorId].staked
REF_2849(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_12915']).userValidatorStakes
REF_2850(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_2849[user_1]
REF_2851(PlumeStakingStorage.UserValidatorStake) -> REF_2850[slashedValidatorId_1]
REF_2852(uint256) -> REF_2851.staked
userActiveStakeToClear_1(uint256) := REF_2852(uint256)
 userCooledAmountToClear = $.userValidatorCooldowns[user][slashedValidatorId].amount
REF_2853(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_12915']).userValidatorCooldowns
REF_2854(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_2853[user_1]
REF_2855(PlumeStakingStorage.CooldownEntry) -> REF_2854[slashedValidatorId_1]
REF_2856(uint256) -> REF_2855.amount
userCooledAmountToClear_1(uint256) := REF_2856(uint256)
 recordChanged = false
recordChanged_1(bool) := False(bool)
 userActiveStakeToClear > 0
TMP_12923(bool) = userActiveStakeToClear_1 > 0
CONDITION TMP_12923
 $.userValidatorStakes[user][slashedValidatorId].staked = 0
REF_2857(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_12915']).userValidatorStakes
REF_2858(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_2857[user_1]
REF_2859(PlumeStakingStorage.UserValidatorStake) -> REF_2858[slashedValidatorId_1]
REF_2860(uint256) -> REF_2859.staked
$_2 (-> ['TMP_12915'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12915'])"])
REF_2860(uint256) (->$_2 (-> ['TMP_12915'])) := 0(uint256)
TMP_12915(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12915'])"])
 $.stakeInfo[user].staked >= userActiveStakeToClear
REF_2861(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_2 (-> ['TMP_12915']).stakeInfo
REF_2862(PlumeStakingStorage.StakeInfo) -> REF_2861[user_1]
REF_2863(uint256) -> REF_2862.staked
TMP_12924(bool) = REF_2863 >= userActiveStakeToClear_1
CONDITION TMP_12924
 $.stakeInfo[user].staked -= userActiveStakeToClear
REF_2864(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_2 (-> ['TMP_12915']).stakeInfo
REF_2865(PlumeStakingStorage.StakeInfo) -> REF_2864[user_1]
REF_2866(uint256) -> REF_2865.staked
$_3 (-> ['TMP_12915'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12915'])"])
REF_2866(-> $_3 (-> ['TMP_12915'])) = REF_2866 (c)- userActiveStakeToClear_1
TMP_12915(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_12915'])"])
 $.stakeInfo[user].staked = 0
REF_2867(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_2 (-> ['TMP_12915']).stakeInfo
REF_2868(PlumeStakingStorage.StakeInfo) -> REF_2867[user_1]
REF_2869(uint256) -> REF_2868.staked
$_4 (-> ['TMP_12915'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12915'])"])
REF_2869(uint256) (->$_4 (-> ['TMP_12915'])) := 0(uint256)
TMP_12915(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_12915'])"])
$_5 (-> ['TMP_12915'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_12915'])", "$_4 (-> ['TMP_12915'])"])
 AdminClearedSlashedStake(user,slashedValidatorId,userActiveStakeToClear)
Emit AdminClearedSlashedStake(user_1,slashedValidatorId_1,userActiveStakeToClear_1)
 recordChanged = true
recordChanged_2(bool) := True(bool)
$_6 (-> ['TMP_12915'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12915'])", "$_1 (-> ['TMP_12915'])"])
recordChanged_3(bool) := phi(['recordChanged_1', 'recordChanged_2'])
 userCooledAmountToClear > 0
TMP_12926(bool) = userCooledAmountToClear_1 > 0
CONDITION TMP_12926
 delete $.userValidatorCooldowns[user][slashedValidatorId]
REF_2870(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_6 (-> ['TMP_12915']).userValidatorCooldowns
REF_2871(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_2870[user_1]
REF_2872(PlumeStakingStorage.CooldownEntry) -> REF_2871[slashedValidatorId_1]
REF_2871 = delete REF_2872 
 $.stakeInfo[user].cooled >= userCooledAmountToClear
REF_2873(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_6 (-> ['TMP_12915']).stakeInfo
REF_2874(PlumeStakingStorage.StakeInfo) -> REF_2873[user_1]
REF_2875(uint256) -> REF_2874.cooled
TMP_12927(bool) = REF_2875 >= userCooledAmountToClear_1
CONDITION TMP_12927
 $.stakeInfo[user].cooled -= userCooledAmountToClear
REF_2876(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_6 (-> ['TMP_12915']).stakeInfo
REF_2877(PlumeStakingStorage.StakeInfo) -> REF_2876[user_1]
REF_2878(uint256) -> REF_2877.cooled
$_7 (-> ['TMP_12915'])(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_12915'])"])
REF_2878(-> $_7 (-> ['TMP_12915'])) = REF_2878 (c)- userCooledAmountToClear_1
TMP_12915(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_12915'])"])
 $.stakeInfo[user].cooled = 0
REF_2879(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_6 (-> ['TMP_12915']).stakeInfo
REF_2880(PlumeStakingStorage.StakeInfo) -> REF_2879[user_1]
REF_2881(uint256) -> REF_2880.cooled
$_8 (-> ['TMP_12915'])(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_12915'])"])
REF_2881(uint256) (->$_8 (-> ['TMP_12915'])) := 0(uint256)
TMP_12915(PlumeStakingStorage.Layout) := phi(["$_8 (-> ['TMP_12915'])"])
$_9 (-> ['TMP_12915', 'TMP_12915'])(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_12915'])", "$_8 (-> ['TMP_12915'])"])
 AdminClearedSlashedCooldown(user,slashedValidatorId,userCooledAmountToClear)
Emit AdminClearedSlashedCooldown(user_1,slashedValidatorId_1,userCooledAmountToClear_1)
 recordChanged = true
recordChanged_4(bool) := True(bool)
$_10 (-> ['TMP_12915'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12915'])", "$_6 (-> ['TMP_12915'])"])
recordChanged_5(bool) := phi(['recordChanged_1', 'recordChanged_4'])
 $.userHasStakedWithValidator[user][slashedValidatorId] || recordChanged
REF_2882(mapping(address => mapping(uint16 => bool))) -> $_10 (-> ['TMP_12915']).userHasStakedWithValidator
REF_2883(mapping(uint16 => bool)) -> REF_2882[user_1]
REF_2884(bool) -> REF_2883[slashedValidatorId_1]
TMP_12929(bool) = REF_2884 || recordChanged_5
CONDITION TMP_12929
 PlumeValidatorLogic.removeStakerFromValidator($,user,slashedValidatorId)
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.removeStakerFromValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_10 (-> ['TMP_12915'])", 'user_1', 'slashedValidatorId_1'] 
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2886(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2886)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2887(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2887)
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
#### PlumeValidatorLogic.removeStakerFromValidator(PlumeStakingStorage.Layout,address,uint16) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
staker_1(address) := phi(['staker_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
 $.userValidatorStakes[staker][validatorId].staked == 0 && $.isStakerForValidator[validatorId][staker]
REF_4341(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> []).userValidatorStakes
REF_4342(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_4341[staker_1]
REF_4343(PlumeStakingStorage.UserValidatorStake) -> REF_4342[validatorId_1]
REF_4344(uint256) -> REF_4343.staked
TMP_14033(bool) = REF_4344 == 0
REF_4345(mapping(uint16 => mapping(address => bool))) -> $_1 (-> []).isStakerForValidator
REF_4346(mapping(address => bool)) -> REF_4345[validatorId_1]
REF_4347(bool) -> REF_4346[staker_1]
TMP_14034(bool) = TMP_14033 && REF_4347
CONDITION TMP_14034
 stakersList = $.validatorStakers[validatorId]
REF_4348(mapping(uint16 => address[])) -> $_1 (-> []).validatorStakers
REF_4349(address[]) -> REF_4348[validatorId_1]
stakersList_1 (-> [])(address[]) = ['REF_4349(address[])']
 listLength = stakersList.length
REF_4350 -> LENGTH stakersList_1 (-> [])
listLength_1(uint256) := REF_4350(uint256)
 listLength > 0
TMP_14035(bool) = listLength_1 > 0
CONDITION TMP_14035
 indexToRemove = $.userIndexInValidatorStakers[staker][validatorId]
REF_4351(mapping(address => mapping(uint16 => uint256))) -> $_1 (-> []).userIndexInValidatorStakers
REF_4352(mapping(uint16 => uint256)) -> REF_4351[staker_1]
REF_4353(uint256) -> REF_4352[validatorId_1]
indexToRemove_1(uint256) := REF_4353(uint256)
 indexToRemove < listLength && stakersList[indexToRemove] == staker
TMP_14036(bool) = indexToRemove_1 < listLength_1
REF_4354(address) -> stakersList_1 (-> [])[indexToRemove_1]
TMP_14037(bool) = REF_4354 == staker_1
TMP_14038(bool) = TMP_14036 && TMP_14037
CONDITION TMP_14038
 lastStaker = stakersList[listLength - 1]
TMP_14039(uint256) = listLength_1 (c)- 1
REF_4355(address) -> stakersList_1 (-> [])[TMP_14039]
lastStaker_1(address) := REF_4355(address)
 indexToRemove != listLength - 1
TMP_14040(uint256) = listLength_1 (c)- 1
TMP_14041(bool) = indexToRemove_1 != TMP_14040
CONDITION TMP_14041
 stakersList[indexToRemove] = lastStaker
REF_4356(address) -> stakersList_1 (-> [])[indexToRemove_1]
stakersList_2 (-> [])(address[]) := phi(['stakersList_1 (-> [])'])
REF_4356(address) (->stakersList_2 (-> [])) := lastStaker_1(address)
 $.userIndexInValidatorStakers[lastStaker][validatorId] = indexToRemove
REF_4357(mapping(address => mapping(uint16 => uint256))) -> $_1 (-> []).userIndexInValidatorStakers
REF_4358(mapping(uint16 => uint256)) -> REF_4357[lastStaker_1]
REF_4359(uint256) -> REF_4358[validatorId_1]
$_2 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4359(uint256) (->$_2 (-> [])) := indexToRemove_1(uint256)
$_3 (-> [])(PlumeStakingStorage.Layout) := phi(['$_2 (-> [])', '$_1 (-> [])'])
stakersList_3 (-> [])(address[]) := phi(['stakersList_1 (-> [])', 'stakersList_2 (-> [])'])
 stakersList.pop()
REF_4361 -> LENGTH stakersList_3 (-> [])
TMP_14043(uint256) = REF_4361 (c)- 1
REF_4362(address) -> stakersList_3 (-> [])[TMP_14043]
stakersList_4 (-> []) = delete REF_4362 
REF_4363 -> LENGTH stakersList_4 (-> [])
stakersList_5 (-> [])(address[]) := phi(['stakersList_4 (-> [])'])
REF_4363(uint256) (->stakersList_5 (-> [])) := TMP_14043(uint256)
 $.isStakerForValidator[validatorId][staker] = false
REF_4364(mapping(uint16 => mapping(address => bool))) -> $_3 (-> []).isStakerForValidator
REF_4365(mapping(address => bool)) -> REF_4364[validatorId_1]
REF_4366(bool) -> REF_4365[staker_1]
$_4 (-> [])(PlumeStakingStorage.Layout) := phi(['$_3 (-> [])'])
REF_4366(bool) (->$_4 (-> [])) := False(bool)
 delete $.userIndexInValidatorStakers[staker][validatorId]
REF_4367(mapping(address => mapping(uint16 => uint256))) -> $_4 (-> []).userIndexInValidatorStakers
REF_4368(mapping(uint16 => uint256)) -> REF_4367[staker_1]
REF_4369(uint256) -> REF_4368[validatorId_1]
REF_4368 = delete REF_4369 
$_5 (-> [])(PlumeStakingStorage.Layout) := phi(['$_4 (-> [])', '$_1 (-> [])'])
 hasActiveStakeForThisVal = $.userValidatorStakes[staker][validatorId].staked > 0
REF_4370(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_5 (-> []).userValidatorStakes
REF_4371(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_4370[staker_1]
REF_4372(PlumeStakingStorage.UserValidatorStake) -> REF_4371[validatorId_1]
REF_4373(uint256) -> REF_4372.staked
TMP_14044(bool) = REF_4373 > 0
hasActiveStakeForThisVal_1(bool) := TMP_14044(bool)
 hasActiveCooldownForThisVal = $.userValidatorCooldowns[staker][validatorId].amount > 0
REF_4374(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_5 (-> []).userValidatorCooldowns
REF_4375(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_4374[staker_1]
REF_4376(PlumeStakingStorage.CooldownEntry) -> REF_4375[validatorId_1]
REF_4377(uint256) -> REF_4376.amount
TMP_14045(bool) = REF_4377 > 0
hasActiveCooldownForThisVal_1(bool) := TMP_14045(bool)
 hasPendingRewardsForThisVal = $.userHasPendingRewards[staker][validatorId]
REF_4378(mapping(address => mapping(uint16 => bool))) -> $_5 (-> []).userHasPendingRewards
REF_4379(mapping(uint16 => bool)) -> REF_4378[staker_1]
REF_4380(bool) -> REF_4379[validatorId_1]
hasPendingRewardsForThisVal_1(bool) := REF_4380(bool)
 ! hasActiveStakeForThisVal && ! hasActiveCooldownForThisVal && ! hasPendingRewardsForThisVal
TMP_14046 = UnaryType.BANG hasActiveStakeForThisVal_1 
TMP_14047 = UnaryType.BANG hasActiveCooldownForThisVal_1 
TMP_14048(bool) = TMP_14046 && TMP_14047
TMP_14049 = UnaryType.BANG hasPendingRewardsForThisVal_1 
TMP_14050(bool) = TMP_14048 && TMP_14049
CONDITION TMP_14050
 $.userHasStakedWithValidator[staker][validatorId]
REF_4381(mapping(address => mapping(uint16 => bool))) -> $_5 (-> []).userHasStakedWithValidator
REF_4382(mapping(uint16 => bool)) -> REF_4381[staker_1]
REF_4383(bool) -> REF_4382[validatorId_1]
CONDITION REF_4383
 userValidators_ = $.userValidators[staker]
REF_4384(mapping(address => uint16[])) -> $_5 (-> []).userValidators
REF_4385(uint16[]) -> REF_4384[staker_1]
userValidators__1 (-> [])(uint16[]) = ['REF_4385(uint16[])']
 removed = false
removed_1(bool) := False(bool)
removed_3(bool) := phi(['removed_1', 'removed_2'])
 i = 0
i_1(uint256) := 0(uint256)
 i < userValidators_.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4386 -> LENGTH userValidators__1 (-> [])
TMP_14051(bool) = i_2 < REF_4386
CONDITION TMP_14051
 userValidators_[i] == validatorId
REF_4387(uint16) -> userValidators__1 (-> [])[i_2]
TMP_14052(bool) = REF_4387 == validatorId_1
CONDITION TMP_14052
 userValidators_[i] = userValidators_[userValidators_.length - 1]
REF_4388(uint16) -> userValidators__1 (-> [])[i_2]
REF_4389 -> LENGTH userValidators__1 (-> [])
TMP_14053(uint256) = REF_4389 (c)- 1
REF_4390(uint16) -> userValidators__1 (-> [])[TMP_14053]
userValidators__2 (-> [])(uint16[]) := phi(['userValidators__1 (-> [])'])
REF_4388(uint16) (->userValidators__2 (-> [])) := REF_4390(uint16)
 userValidators_.pop()
REF_4392 -> LENGTH userValidators__2 (-> [])
TMP_14055(uint256) = REF_4392 (c)- 1
REF_4393(uint16) -> userValidators__2 (-> [])[TMP_14055]
userValidators__3 (-> []) = delete REF_4393 
REF_4394 -> LENGTH userValidators__3 (-> [])
userValidators__4 (-> [])(uint16[]) := phi(['userValidators__3 (-> [])'])
REF_4394(uint256) (->userValidators__4 (-> [])) := TMP_14055(uint256)
 removed = true
removed_2(bool) := True(bool)
 i ++
TMP_14056(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 removed
CONDITION removed_3
 $.userHasStakedWithValidator[staker][validatorId] = false
REF_4395(mapping(address => mapping(uint16 => bool))) -> $_5 (-> []).userHasStakedWithValidator
REF_4396(mapping(uint16 => bool)) -> REF_4395[staker_1]
REF_4397(bool) -> REF_4396[validatorId_1]
$_6 (-> [])(PlumeStakingStorage.Layout) := phi(['$_5 (-> [])'])
REF_4397(bool) (->$_6 (-> [])) := False(bool)
```
