



#### ManagementFacet.setMinStakeAmount(uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_12863(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12863'])(PlumeStakingStorage.Layout) := TMP_12863(PlumeStakingStorage.Layout)
 oldAmount = $.minStakeAmount
REF_2813(uint256) -> $_1 (-> ['TMP_12863']).minStakeAmount
oldAmount_1(uint256) := REF_2813(uint256)
 _minStakeAmount == 0
TMP_12864(bool) = _minStakeAmount_1 == 0
CONDITION TMP_12864
 revert InvalidAmount(uint256)(_minStakeAmount)
TMP_12865(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(_minStakeAmount_1)
 $.minStakeAmount = _minStakeAmount
REF_2814(uint256) -> $_1 (-> ['TMP_12863']).minStakeAmount
$_2 (-> ['TMP_12863'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12863'])"])
REF_2814(uint256) (->$_2 (-> ['TMP_12863'])) := _minStakeAmount_1(uint256)
TMP_12863(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12863'])"])
 MinStakeAmountSet(_minStakeAmount)
Emit MinStakeAmountSet(_minStakeAmount_1)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2815(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2815)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2816(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2816)
```
#### ManagementFacet.setCooldownInterval(uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_12869(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12869'])(PlumeStakingStorage.Layout) := TMP_12869(PlumeStakingStorage.Layout)
 interval == 0
TMP_12870(bool) = interval_1 == 0
CONDITION TMP_12870
 revert InvalidInterval(uint256)(interval)
TMP_12871(None) = SOLIDITY_CALL revert InvalidInterval(uint256)(interval_1)
 $.maxSlashVoteDurationInSeconds != 0 && interval <= $.maxSlashVoteDurationInSeconds
REF_2818(uint256) -> $_1 (-> ['TMP_12869']).maxSlashVoteDurationInSeconds
TMP_12872(bool) = REF_2818 != 0
REF_2819(uint256) -> $_1 (-> ['TMP_12869']).maxSlashVoteDurationInSeconds
TMP_12873(bool) = interval_1 <= REF_2819
TMP_12874(bool) = TMP_12872 && TMP_12873
CONDITION TMP_12874
 revert CooldownTooShortForSlashVote(uint256,uint256)(interval,$.maxSlashVoteDurationInSeconds)
REF_2820(uint256) -> $_1 (-> ['TMP_12869']).maxSlashVoteDurationInSeconds
TMP_12875(None) = SOLIDITY_CALL revert CooldownTooShortForSlashVote(uint256,uint256)(interval_1,REF_2820)
 $.cooldownInterval = interval
REF_2821(uint256) -> $_1 (-> ['TMP_12869']).cooldownInterval
$_2 (-> ['TMP_12869'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12869'])"])
REF_2821(uint256) (->$_2 (-> ['TMP_12869'])) := interval_1(uint256)
TMP_12869(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12869'])"])
 CooldownIntervalSet(interval)
Emit CooldownIntervalSet(interval_1)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2822(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2822)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2823(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2823)
```
#### ManagementFacet.adminWithdraw(address,uint256,address) [EXTERNAL]
```slithir
 token == address(0)
TMP_12879 = CONVERT 0 to address
TMP_12880(bool) = token_1 == TMP_12879
CONDITION TMP_12880
 revert ZeroAddress(string)(token)
TMP_12881(None) = SOLIDITY_CALL revert ZeroAddress(string)(token)
 recipient == address(0)
TMP_12882 = CONVERT 0 to address
TMP_12883(bool) = recipient_1 == TMP_12882
CONDITION TMP_12883
 revert ZeroAddress(string)(recipient)
TMP_12884(None) = SOLIDITY_CALL revert ZeroAddress(string)(recipient)
 amount == 0
TMP_12885(bool) = amount_1 == 0
CONDITION TMP_12885
 revert InvalidAmount(uint256)(amount)
TMP_12886(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(amount_1)
 token == PlumeStakingStorage.PLUME_NATIVE
REF_2824(address) -> PlumeStakingStorage.PLUME_NATIVE
TMP_12887(bool) = token_1 == REF_2824
CONDITION TMP_12887
 balance = address(this).balance
TMP_12888 = CONVERT this to address
TMP_12889(uint256) = SOLIDITY_CALL balance(address)(TMP_12888)
balance_1(uint256) := TMP_12889(uint256)
 amount > balance
TMP_12890(bool) = amount_1 > balance_1
CONDITION TMP_12890
 revert InsufficientFunds(uint256,uint256)(balance,amount)
TMP_12891(None) = SOLIDITY_CALL revert InsufficientFunds(uint256,uint256)(balance_1,amount_1)
 (success,None) = address(recipient).call{value: amount}()
TMP_12892 = CONVERT recipient_1 to address
TUPLE_86(bool,bytes) = LOW_LEVEL_CALL, dest:TMP_12892, function:call, arguments:[''] value:amount_1 
success_1(bool)= UNPACK TUPLE_86 index: 0 
 ! success
TMP_12893 = UnaryType.BANG success_1 
CONDITION TMP_12893
 revert AdminTransferFailed()()
TMP_12894(None) = SOLIDITY_CALL revert AdminTransferFailed()()
 erc20Token = IERC20(token)
TMP_12895 = CONVERT token_1 to IERC20
erc20Token_1(IERC20) := TMP_12895(IERC20)
 balance_scope_0 = erc20Token.balanceOf(address(this))
TMP_12896 = CONVERT this to address
TMP_12897(uint256) = HIGH_LEVEL_CALL, dest:erc20Token_1(IERC20), function:balanceOf, arguments:['TMP_12896']  
balance_scope_0_1(uint256) := TMP_12897(uint256)
 amount > balance_scope_0
TMP_12898(bool) = amount_1 > balance_scope_0_1
CONDITION TMP_12898
 revert InsufficientFunds(uint256,uint256)(balance_scope_0,amount)
TMP_12899(None) = SOLIDITY_CALL revert InsufficientFunds(uint256,uint256)(balance_scope_0_1,amount_1)
 erc20Token.safeTransfer(recipient,amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['erc20Token_1', 'recipient_1', 'amount_1'] 
 AdminWithdraw(token,amount,recipient)
Emit AdminWithdraw(token_1,amount_1,recipient_1)
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_2828(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2828)
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_2829(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2829)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### ManagementFacet.getMinStakeAmount() [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().minStakeAmount
TMP_12905(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_2831(uint256) -> TMP_12905.minStakeAmount
RETURN REF_2831
```
#### ManagementFacet.getCooldownInterval() [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().cooldownInterval
TMP_12906(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_2833(uint256) -> TMP_12906.cooldownInterval
RETURN REF_2833
```
#### ManagementFacet.setMaxSlashVoteDuration(uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_12907(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12907'])(PlumeStakingStorage.Layout) := TMP_12907(PlumeStakingStorage.Layout)
 duration == 0
TMP_12908(bool) = duration_1 == 0
CONDITION TMP_12908
 revert InvalidInterval(uint256)(duration)
TMP_12909(None) = SOLIDITY_CALL revert InvalidInterval(uint256)(duration_1)
 $.cooldownInterval != 0 && duration >= $.cooldownInterval
REF_2835(uint256) -> $_1 (-> ['TMP_12907']).cooldownInterval
TMP_12910(bool) = REF_2835 != 0
REF_2836(uint256) -> $_1 (-> ['TMP_12907']).cooldownInterval
TMP_12911(bool) = duration_1 >= REF_2836
TMP_12912(bool) = TMP_12910 && TMP_12911
CONDITION TMP_12912
 revert SlashVoteDurationTooLongForCooldown(uint256,uint256)(duration,$.cooldownInterval)
REF_2837(uint256) -> $_1 (-> ['TMP_12907']).cooldownInterval
TMP_12913(None) = SOLIDITY_CALL revert SlashVoteDurationTooLongForCooldown(uint256,uint256)(duration_1,REF_2837)
 duration >= PlumeStakingStorage.COMMISSION_CLAIM_TIMELOCK
REF_2838(uint256) -> PlumeStakingStorage.COMMISSION_CLAIM_TIMELOCK
TMP_12914(bool) = duration_1 >= REF_2838
CONDITION TMP_12914
 revert SlashVoteDurationExceedsCommissionTimelock(uint256,uint256)(duration,PlumeStakingStorage.COMMISSION_CLAIM_TIMELOCK)
REF_2839(uint256) -> PlumeStakingStorage.COMMISSION_CLAIM_TIMELOCK
TMP_12915(None) = SOLIDITY_CALL revert SlashVoteDurationExceedsCommissionTimelock(uint256,uint256)(duration_1,REF_2839)
 $.maxSlashVoteDurationInSeconds = duration
REF_2840(uint256) -> $_1 (-> ['TMP_12907']).maxSlashVoteDurationInSeconds
$_2 (-> ['TMP_12907'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12907'])"])
REF_2840(uint256) (->$_2 (-> ['TMP_12907'])) := duration_1(uint256)
TMP_12907(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12907'])"])
 MaxSlashVoteDurationSet(duration)
Emit MaxSlashVoteDurationSet(duration_1)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2841(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2841)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2842(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2842)
```
#### ManagementFacet.setMaxAllowedValidatorCommission(uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_12919(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12919'])(PlumeStakingStorage.Layout) := TMP_12919(PlumeStakingStorage.Layout)
 newMaxRate > PlumeStakingStorage.REWARD_PRECISION / 2
REF_2844(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_12920(uint256) = REF_2844 (c)/ 2
TMP_12921(bool) = newMaxRate_1 > TMP_12920
CONDITION TMP_12921
 revert InvalidMaxCommissionRate(uint256,uint256)(newMaxRate,PlumeStakingStorage.REWARD_PRECISION / 2)
REF_2845(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_12922(uint256) = REF_2845 (c)/ 2
TMP_12923(None) = SOLIDITY_CALL revert InvalidMaxCommissionRate(uint256,uint256)(newMaxRate_1,TMP_12922)
 oldMaxRate = $.maxAllowedValidatorCommission
REF_2846(uint256) -> $_1 (-> ['TMP_12919']).maxAllowedValidatorCommission
oldMaxRate_1(uint256) := REF_2846(uint256)
 $.maxAllowedValidatorCommission = newMaxRate
REF_2847(uint256) -> $_1 (-> ['TMP_12919']).maxAllowedValidatorCommission
$_2 (-> ['TMP_12919'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12919'])"])
REF_2847(uint256) (->$_2 (-> ['TMP_12919'])) := newMaxRate_1(uint256)
TMP_12919(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12919'])"])
 MaxAllowedValidatorCommissionSet(oldMaxRate,newMaxRate)
Emit MaxAllowedValidatorCommissionSet(oldMaxRate_1,newMaxRate_1)
 validatorIds = $.validatorIds
REF_2848(uint16[]) -> $_2 (-> ['TMP_12919']).validatorIds
validatorIds_1(uint16[]) = ['REF_2848(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_2849 -> LENGTH validatorIds_1
TMP_12925(bool) = i_2 < REF_2849
CONDITION TMP_12925
 validatorId = validatorIds[i]
REF_2850(uint16) -> validatorIds_1[i_2]
validatorId_1(uint16) := REF_2850(uint16)
 validator = $.validators[validatorId]
REF_2851(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_2 (-> ['TMP_12919']).validators
REF_2852(PlumeStakingStorage.ValidatorInfo) -> REF_2851[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_2852(PlumeStakingStorage.ValidatorInfo)
 validator.commission > newMaxRate
REF_2853(uint256) -> validator_1 (-> ['$']).commission
TMP_12926(bool) = REF_2853 > newMaxRate_1
CONDITION TMP_12926
 oldCommission = validator.commission
REF_2854(uint256) -> validator_1 (-> ['$']).commission
oldCommission_1(uint256) := REF_2854(uint256)
 PlumeRewardLogic._settleCommissionForValidatorUpToNow($,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic._settleCommissionForValidatorUpToNow(PlumeStakingStorage.Layout,uint16), arguments:["$_2 (-> ['TMP_12919'])", 'validatorId_1'] 
 validator.commission = newMaxRate
REF_2856(uint256) -> validator_1 (-> ['$']).commission
validator_2 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])"])
REF_2856(uint256) (->validator_2 (-> ['$'])) := newMaxRate_1(uint256)
$_3 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_2 (-> ['$'])"])
 PlumeRewardLogic.createCommissionRateCheckpoint($,validatorId,newMaxRate)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.createCommissionRateCheckpoint(PlumeStakingStorage.Layout,uint16,uint256), arguments:["$_2 (-> ['TMP_12919'])", 'validatorId_1', 'newMaxRate_1'] 
 ValidatorCommissionSet(validatorId,oldCommission,newMaxRate)
Emit ValidatorCommissionSet(validatorId_1,oldCommission_1,newMaxRate_1)
 i ++
TMP_12930(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_2858(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2858)
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_2859(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2859)
```
#### ManagementFacet.setMaxCommissionCheckpoints(uint16) [EXTERNAL]
```slithir
 newLimit < 10
TMP_12933(bool) = newLimit_1 < 10
CONDITION TMP_12933
 revert InvalidAmount(uint256)(newLimit)
TMP_12934(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(newLimit_1)
 PlumeStakingStorage.layout().maxCommissionCheckpoints = newLimit
TMP_12935(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_2861(uint16) -> TMP_12935.maxCommissionCheckpoints
REF_2861(uint16) (->TMP_12935) := newLimit_1(uint16)
 MaxCommissionCheckpointsSet(newLimit)
Emit MaxCommissionCheckpointsSet(newLimit_1)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2862(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2862)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2863(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2863)
```
#### ManagementFacet.setMaxValidatorPercentage(uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_12939(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12939'])(PlumeStakingStorage.Layout) := TMP_12939(PlumeStakingStorage.Layout)
 newPercentage > 10_000
TMP_12940(bool) = newPercentage_1 > 10000
CONDITION TMP_12940
 revert InvalidPercentage(uint256)(newPercentage)
TMP_12941(None) = SOLIDITY_CALL revert InvalidPercentage(uint256)(newPercentage_1)
 oldPercentage = $.maxValidatorPercentage
REF_2865(uint256) -> $_1 (-> ['TMP_12939']).maxValidatorPercentage
oldPercentage_1(uint256) := REF_2865(uint256)
 $.maxValidatorPercentage = newPercentage
REF_2866(uint256) -> $_1 (-> ['TMP_12939']).maxValidatorPercentage
$_2 (-> ['TMP_12939'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12939'])"])
REF_2866(uint256) (->$_2 (-> ['TMP_12939'])) := newPercentage_1(uint256)
TMP_12939(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12939'])"])
 MaxValidatorPercentageUpdated(oldPercentage,newPercentage)
Emit MaxValidatorPercentageUpdated(oldPercentage_1,newPercentage_1)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2867(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2867)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2868(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2868)
```
#### ManagementFacet.pruneCommissionCheckpoints(uint16,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_12945(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12945'])(PlumeStakingStorage.Layout) := TMP_12945(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_2870(mapping(uint16 => bool)) -> $_1 (-> ['TMP_12945']).validatorExists
REF_2871(bool) -> REF_2870[validatorId_1]
TMP_12946 = UnaryType.BANG REF_2871 
CONDITION TMP_12946
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_12947(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 count == 0
TMP_12948(bool) = count_1 == 0
CONDITION TMP_12948
 revert InvalidAmount(uint256)(count)
TMP_12949(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(count_1)
 checkpoints = $.validatorCommissionCheckpoints[validatorId]
REF_2872(mapping(uint16 => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> ['TMP_12945']).validatorCommissionCheckpoints
REF_2873(PlumeStakingStorage.RateCheckpoint[]) -> REF_2872[validatorId_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_2873(PlumeStakingStorage.RateCheckpoint[])']
 len = checkpoints.length
REF_2874 -> LENGTH checkpoints_1 (-> [])
len_1(uint256) := REF_2874(uint256)
 count >= len
TMP_12950(bool) = count_1 >= len_1
CONDITION TMP_12950
 revert CannotPruneAllCheckpoints()()
TMP_12951(None) = SOLIDITY_CALL revert CannotPruneAllCheckpoints()()
 i = 0
i_1(uint256) := 0(uint256)
 i < len - count
checkpoints_2 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_3 (-> [])', 'checkpoints_1 (-> [])'])
i_2(uint256) := phi(['i_3', 'i_1'])
TMP_12952(uint256) = len_1 (c)- count_1
TMP_12953(bool) = i_2 < TMP_12952
CONDITION TMP_12953
 checkpoints[i] = checkpoints[i + count]
REF_2875(PlumeStakingStorage.RateCheckpoint) -> checkpoints_2 (-> [])[i_2]
TMP_12954(uint256) = i_2 (c)+ count_1
REF_2876(PlumeStakingStorage.RateCheckpoint) -> checkpoints_2 (-> [])[TMP_12954]
checkpoints_3 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_2 (-> [])'])
REF_2875(PlumeStakingStorage.RateCheckpoint) (->checkpoints_3 (-> [])) := REF_2876(PlumeStakingStorage.RateCheckpoint)
 i ++
TMP_12955(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < count
checkpoints_4 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_6 (-> [])', 'checkpoints_1 (-> [])'])
i_scope_0_2(uint256) := phi(['i_scope_0_1', 'i_scope_0_3'])
TMP_12956(bool) = i_scope_0_2 < count_1
CONDITION TMP_12956
 checkpoints.pop()
REF_2878 -> LENGTH checkpoints_4 (-> [])
TMP_12958(uint256) = REF_2878 (c)- 1
REF_2879(PlumeStakingStorage.RateCheckpoint) -> checkpoints_4 (-> [])[TMP_12958]
checkpoints_5 (-> []) = delete REF_2879 
REF_2880 -> LENGTH checkpoints_5 (-> [])
checkpoints_6 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_5 (-> [])'])
REF_2880(uint256) (->checkpoints_6 (-> [])) := TMP_12958(uint256)
 i_scope_0 ++
TMP_12959(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 (c)+ 1
 CommissionCheckpointsPruned(validatorId,count)
Emit CommissionCheckpointsPruned(validatorId_1,count_1)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2881(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2881)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2882(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2882)
```
#### ManagementFacet.pruneRewardRateCheckpoints(uint16,address,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_12963(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12963'])(PlumeStakingStorage.Layout) := TMP_12963(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_2884(mapping(uint16 => bool)) -> $_1 (-> ['TMP_12963']).validatorExists
REF_2885(bool) -> REF_2884[validatorId_1]
TMP_12964 = UnaryType.BANG REF_2885 
CONDITION TMP_12964
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_12965(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 ! $.isRewardToken[token] && $.tokenAdditionTimestamps[token] == 0
REF_2886(mapping(address => bool)) -> $_1 (-> ['TMP_12963']).isRewardToken
REF_2887(bool) -> REF_2886[token_1]
TMP_12966 = UnaryType.BANG REF_2887 
REF_2888(mapping(address => uint256)) -> $_1 (-> ['TMP_12963']).tokenAdditionTimestamps
REF_2889(uint256) -> REF_2888[token_1]
TMP_12967(bool) = REF_2889 == 0
TMP_12968(bool) = TMP_12966 && TMP_12967
CONDITION TMP_12968
 revert TokenDoesNotExist(address)(token)
TMP_12969(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 count == 0
TMP_12970(bool) = count_1 == 0
CONDITION TMP_12970
 revert InvalidAmount(uint256)(count)
TMP_12971(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(count_1)
 checkpoints = $.validatorRewardRateCheckpoints[validatorId][token]
REF_2890(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> ['TMP_12963']).validatorRewardRateCheckpoints
REF_2891(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_2890[validatorId_1]
REF_2892(PlumeStakingStorage.RateCheckpoint[]) -> REF_2891[token_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_2892(PlumeStakingStorage.RateCheckpoint[])']
 len = checkpoints.length
REF_2893 -> LENGTH checkpoints_1 (-> [])
len_1(uint256) := REF_2893(uint256)
 count >= len
TMP_12972(bool) = count_1 >= len_1
CONDITION TMP_12972
 revert CannotPruneAllCheckpoints()()
TMP_12973(None) = SOLIDITY_CALL revert CannotPruneAllCheckpoints()()
 i = 0
i_1(uint256) := 0(uint256)
 i < len - count
checkpoints_2 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_3 (-> [])', 'checkpoints_1 (-> [])'])
i_2(uint256) := phi(['i_3', 'i_1'])
TMP_12974(uint256) = len_1 (c)- count_1
TMP_12975(bool) = i_2 < TMP_12974
CONDITION TMP_12975
 checkpoints[i] = checkpoints[i + count]
REF_2894(PlumeStakingStorage.RateCheckpoint) -> checkpoints_2 (-> [])[i_2]
TMP_12976(uint256) = i_2 (c)+ count_1
REF_2895(PlumeStakingStorage.RateCheckpoint) -> checkpoints_2 (-> [])[TMP_12976]
checkpoints_3 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_2 (-> [])'])
REF_2894(PlumeStakingStorage.RateCheckpoint) (->checkpoints_3 (-> [])) := REF_2895(PlumeStakingStorage.RateCheckpoint)
 i ++
TMP_12977(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < count
checkpoints_4 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_6 (-> [])', 'checkpoints_1 (-> [])'])
i_scope_0_2(uint256) := phi(['i_scope_0_1', 'i_scope_0_3'])
TMP_12978(bool) = i_scope_0_2 < count_1
CONDITION TMP_12978
 checkpoints.pop()
REF_2897 -> LENGTH checkpoints_4 (-> [])
TMP_12980(uint256) = REF_2897 (c)- 1
REF_2898(PlumeStakingStorage.RateCheckpoint) -> checkpoints_4 (-> [])[TMP_12980]
checkpoints_5 (-> []) = delete REF_2898 
REF_2899 -> LENGTH checkpoints_5 (-> [])
checkpoints_6 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_5 (-> [])'])
REF_2899(uint256) (->checkpoints_6 (-> [])) := TMP_12980(uint256)
 i_scope_0 ++
TMP_12981(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 (c)+ 1
 RewardRateCheckpointsPruned(validatorId,token,count)
Emit RewardRateCheckpointsPruned(validatorId_1,token_1,count_1)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2900(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2900)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2901(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2901)
```
#### ManagementFacet.adminClearValidatorRecord(address,uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_12985(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12985'])(PlumeStakingStorage.Layout) := TMP_12985(PlumeStakingStorage.Layout)
 user == address(0)
TMP_12986 = CONVERT 0 to address
TMP_12987(bool) = user_1 == TMP_12986
CONDITION TMP_12987
 revert ZeroAddress(string)(user)
TMP_12988(None) = SOLIDITY_CALL revert ZeroAddress(string)(user)
 ! $.validatorExists[slashedValidatorId]
REF_2903(mapping(uint16 => bool)) -> $_1 (-> ['TMP_12985']).validatorExists
REF_2904(bool) -> REF_2903[slashedValidatorId_1]
TMP_12989 = UnaryType.BANG REF_2904 
CONDITION TMP_12989
 revert ValidatorDoesNotExist(uint16)(slashedValidatorId)
TMP_12990(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(slashedValidatorId_1)
 ! $.validators[slashedValidatorId].slashed
REF_2905(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_12985']).validators
REF_2906(PlumeStakingStorage.ValidatorInfo) -> REF_2905[slashedValidatorId_1]
REF_2907(bool) -> REF_2906.slashed
TMP_12991 = UnaryType.BANG REF_2907 
CONDITION TMP_12991
 revert ValidatorNotSlashed(uint16)(slashedValidatorId)
TMP_12992(None) = SOLIDITY_CALL revert ValidatorNotSlashed(uint16)(slashedValidatorId_1)
 userActiveStakeToClear = $.userValidatorStakes[user][slashedValidatorId].staked
REF_2908(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_12985']).userValidatorStakes
REF_2909(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_2908[user_1]
REF_2910(PlumeStakingStorage.UserValidatorStake) -> REF_2909[slashedValidatorId_1]
REF_2911(uint256) -> REF_2910.staked
userActiveStakeToClear_1(uint256) := REF_2911(uint256)
 cooldownEntry = $.userValidatorCooldowns[user][slashedValidatorId]
REF_2912(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_12985']).userValidatorCooldowns
REF_2913(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_2912[user_1]
REF_2914(PlumeStakingStorage.CooldownEntry) -> REF_2913[slashedValidatorId_1]
cooldownEntry_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_2914(PlumeStakingStorage.CooldownEntry)
 userCooledAmountToClear = cooldownEntry.amount
REF_2915(uint256) -> cooldownEntry_1 (-> ['$']).amount
userCooledAmountToClear_1(uint256) := REF_2915(uint256)
 recordChanged = false
recordChanged_1(bool) := False(bool)
 userActiveStakeToClear > 0
TMP_12993(bool) = userActiveStakeToClear_1 > 0
CONDITION TMP_12993
 $.userValidatorStakes[user][slashedValidatorId].staked = 0
REF_2916(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_12985']).userValidatorStakes
REF_2917(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_2916[user_1]
REF_2918(PlumeStakingStorage.UserValidatorStake) -> REF_2917[slashedValidatorId_1]
REF_2919(uint256) -> REF_2918.staked
$_2 (-> ['TMP_12985'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12985'])"])
REF_2919(uint256) (->$_2 (-> ['TMP_12985'])) := 0(uint256)
TMP_12985(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12985'])"])
 $.stakeInfo[user].staked >= userActiveStakeToClear
REF_2920(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_2 (-> ['TMP_12985']).stakeInfo
REF_2921(PlumeStakingStorage.StakeInfo) -> REF_2920[user_1]
REF_2922(uint256) -> REF_2921.staked
TMP_12994(bool) = REF_2922 >= userActiveStakeToClear_1
CONDITION TMP_12994
 $.stakeInfo[user].staked -= userActiveStakeToClear
REF_2923(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_2 (-> ['TMP_12985']).stakeInfo
REF_2924(PlumeStakingStorage.StakeInfo) -> REF_2923[user_1]
REF_2925(uint256) -> REF_2924.staked
$_3 (-> ['TMP_12985'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12985'])"])
REF_2925(-> $_3 (-> ['TMP_12985'])) = REF_2925 (c)- userActiveStakeToClear_1
TMP_12985(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_12985'])"])
 $.stakeInfo[user].staked = 0
REF_2926(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_2 (-> ['TMP_12985']).stakeInfo
REF_2927(PlumeStakingStorage.StakeInfo) -> REF_2926[user_1]
REF_2928(uint256) -> REF_2927.staked
$_4 (-> ['TMP_12985'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12985'])"])
REF_2928(uint256) (->$_4 (-> ['TMP_12985'])) := 0(uint256)
TMP_12985(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_12985'])"])
$_5 (-> ['TMP_12985', 'TMP_12985'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_12985'])", "$_4 (-> ['TMP_12985'])"])
 AdminClearedSlashedStake(user,slashedValidatorId,userActiveStakeToClear)
Emit AdminClearedSlashedStake(user_1,slashedValidatorId_1,userActiveStakeToClear_1)
 recordChanged = true
recordChanged_2(bool) := True(bool)
$_6 (-> ['TMP_12985'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12985'])", "$_2 (-> ['TMP_12985'])"])
recordChanged_3(bool) := phi(['recordChanged_2', 'recordChanged_1'])
 userCooledAmountToClear > 0
TMP_12996(bool) = userCooledAmountToClear_1 > 0
CONDITION TMP_12996
 slashTimestamp = $.validators[slashedValidatorId].slashedAtTimestamp
REF_2929(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_6 (-> ['TMP_12985']).validators
REF_2930(PlumeStakingStorage.ValidatorInfo) -> REF_2929[slashedValidatorId_1]
REF_2931(uint256) -> REF_2930.slashedAtTimestamp
slashTimestamp_1(uint256) := REF_2931(uint256)
 cooldownIsLost = cooldownEntry.cooldownEndTime >= slashTimestamp
REF_2932(uint256) -> cooldownEntry_1 (-> ['$']).cooldownEndTime
TMP_12997(bool) = REF_2932 >= slashTimestamp_1
cooldownIsLost_1(bool) := TMP_12997(bool)
 cooldownIsLost
CONDITION cooldownIsLost_1
 delete $.userValidatorCooldowns[user][slashedValidatorId]
REF_2933(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_6 (-> ['TMP_12985']).userValidatorCooldowns
REF_2934(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_2933[user_1]
REF_2935(PlumeStakingStorage.CooldownEntry) -> REF_2934[slashedValidatorId_1]
REF_2934 = delete REF_2935 
 $.stakeInfo[user].cooled >= userCooledAmountToClear
REF_2936(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_6 (-> ['TMP_12985']).stakeInfo
REF_2937(PlumeStakingStorage.StakeInfo) -> REF_2936[user_1]
REF_2938(uint256) -> REF_2937.cooled
TMP_12998(bool) = REF_2938 >= userCooledAmountToClear_1
CONDITION TMP_12998
 $.stakeInfo[user].cooled -= userCooledAmountToClear
REF_2939(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_6 (-> ['TMP_12985']).stakeInfo
REF_2940(PlumeStakingStorage.StakeInfo) -> REF_2939[user_1]
REF_2941(uint256) -> REF_2940.cooled
$_7 (-> ['TMP_12985'])(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_12985'])"])
REF_2941(-> $_7 (-> ['TMP_12985'])) = REF_2941 (c)- userCooledAmountToClear_1
TMP_12985(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_12985'])"])
 $.stakeInfo[user].cooled = 0
REF_2942(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_6 (-> ['TMP_12985']).stakeInfo
REF_2943(PlumeStakingStorage.StakeInfo) -> REF_2942[user_1]
REF_2944(uint256) -> REF_2943.cooled
$_8 (-> ['TMP_12985'])(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_12985'])"])
REF_2944(uint256) (->$_8 (-> ['TMP_12985'])) := 0(uint256)
TMP_12985(PlumeStakingStorage.Layout) := phi(["$_8 (-> ['TMP_12985'])"])
$_9 (-> ['TMP_12985', 'TMP_12985'])(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_12985'])", "$_8 (-> ['TMP_12985'])"])
 AdminClearedSlashedCooldown(user,slashedValidatorId,userCooledAmountToClear)
Emit AdminClearedSlashedCooldown(user_1,slashedValidatorId_1,userCooledAmountToClear_1)
 recordChanged = true
recordChanged_4(bool) := True(bool)
$_10 (-> ['TMP_12985'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12985'])", "$_6 (-> ['TMP_12985'])"])
recordChanged_5(bool) := phi(['recordChanged_4', 'recordChanged_1'])
 $.userHasStakedWithValidator[user][slashedValidatorId] || recordChanged
REF_2945(mapping(address => mapping(uint16 => bool))) -> $_10 (-> ['TMP_12985']).userHasStakedWithValidator
REF_2946(mapping(uint16 => bool)) -> REF_2945[user_1]
REF_2947(bool) -> REF_2946[slashedValidatorId_1]
TMP_13000(bool) = REF_2947 || recordChanged_5
CONDITION TMP_13000
 PlumeValidatorLogic.removeStakerFromValidator($,user,slashedValidatorId)
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.removeStakerFromValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_10 (-> ['TMP_12985'])", 'user_1', 'slashedValidatorId_1'] 
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2949(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2949)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_2950(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_2950)
```
#### ManagementFacet.adminBatchClearValidatorRecords(address[],uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13004(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13004'])(PlumeStakingStorage.Layout) := TMP_13004(PlumeStakingStorage.Layout)
 ! $.validatorExists[slashedValidatorId]
REF_2952(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13004']).validatorExists
REF_2953(bool) -> REF_2952[slashedValidatorId_1]
TMP_13005 = UnaryType.BANG REF_2953 
CONDITION TMP_13005
 revert ValidatorDoesNotExist(uint16)(slashedValidatorId)
TMP_13006(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(slashedValidatorId_1)
 ! $.validators[slashedValidatorId].slashed
REF_2954(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13004']).validators
REF_2955(PlumeStakingStorage.ValidatorInfo) -> REF_2954[slashedValidatorId_1]
REF_2956(bool) -> REF_2955.slashed
TMP_13007 = UnaryType.BANG REF_2956 
CONDITION TMP_13007
 revert ValidatorNotSlashed(uint16)(slashedValidatorId)
TMP_13008(None) = SOLIDITY_CALL revert ValidatorNotSlashed(uint16)(slashedValidatorId_1)
 users.length == 0
REF_2957 -> LENGTH users_1
TMP_13009(bool) = REF_2957 == 0
CONDITION TMP_13009
 revert EmptyArray()()
TMP_13010(None) = SOLIDITY_CALL revert EmptyArray()()
 slashTimestamp = $.validators[slashedValidatorId].slashedAtTimestamp
REF_2958(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13004']).validators
REF_2959(PlumeStakingStorage.ValidatorInfo) -> REF_2958[slashedValidatorId_1]
REF_2960(uint256) -> REF_2959.slashedAtTimestamp
slashTimestamp_1(uint256) := REF_2960(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < users.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_2961 -> LENGTH users_1
TMP_13011(bool) = i_2 < REF_2961
CONDITION TMP_13011
 user = users[i]
REF_2962(address) -> users_1[i_2]
user_1(address) := REF_2962(address)
 user != address(0)
TMP_13012 = CONVERT 0 to address
TMP_13013(bool) = user_1 != TMP_13012
CONDITION TMP_13013
 userActiveStakeToClear = $.userValidatorStakes[user][slashedValidatorId].staked
REF_2963(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13004']).userValidatorStakes
REF_2964(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_2963[user_1]
REF_2965(PlumeStakingStorage.UserValidatorStake) -> REF_2964[slashedValidatorId_1]
REF_2966(uint256) -> REF_2965.staked
userActiveStakeToClear_1(uint256) := REF_2966(uint256)
 cooldownEntry = $.userValidatorCooldowns[user][slashedValidatorId]
REF_2967(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13004']).userValidatorCooldowns
REF_2968(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_2967[user_1]
REF_2969(PlumeStakingStorage.CooldownEntry) -> REF_2968[slashedValidatorId_1]
cooldownEntry_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_2969(PlumeStakingStorage.CooldownEntry)
 userCooledAmountToClear = cooldownEntry.amount
REF_2970(uint256) -> cooldownEntry_1 (-> ['$']).amount
userCooledAmountToClear_1(uint256) := REF_2970(uint256)
 recordActuallyChangedForThisUser = false
recordActuallyChangedForThisUser_1(bool) := False(bool)
 userActiveStakeToClear > 0
TMP_13014(bool) = userActiveStakeToClear_1 > 0
CONDITION TMP_13014
 $.userValidatorStakes[user][slashedValidatorId].staked = 0
REF_2971(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13004']).userValidatorStakes
REF_2972(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_2971[user_1]
REF_2973(PlumeStakingStorage.UserValidatorStake) -> REF_2972[slashedValidatorId_1]
REF_2974(uint256) -> REF_2973.staked
$_2 (-> ['TMP_13004'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13004'])"])
REF_2974(uint256) (->$_2 (-> ['TMP_13004'])) := 0(uint256)
TMP_13004(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13004'])"])
 $.stakeInfo[user].staked >= userActiveStakeToClear
REF_2975(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_2 (-> ['TMP_13004']).stakeInfo
REF_2976(PlumeStakingStorage.StakeInfo) -> REF_2975[user_1]
REF_2977(uint256) -> REF_2976.staked
TMP_13015(bool) = REF_2977 >= userActiveStakeToClear_1
CONDITION TMP_13015
 $.stakeInfo[user].staked -= userActiveStakeToClear
REF_2978(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_2 (-> ['TMP_13004']).stakeInfo
REF_2979(PlumeStakingStorage.StakeInfo) -> REF_2978[user_1]
REF_2980(uint256) -> REF_2979.staked
$_3 (-> ['TMP_13004'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13004'])"])
REF_2980(-> $_3 (-> ['TMP_13004'])) = REF_2980 (c)- userActiveStakeToClear_1
TMP_13004(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13004'])"])
 $.stakeInfo[user].staked = 0
REF_2981(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_2 (-> ['TMP_13004']).stakeInfo
REF_2982(PlumeStakingStorage.StakeInfo) -> REF_2981[user_1]
REF_2983(uint256) -> REF_2982.staked
$_4 (-> ['TMP_13004'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13004'])"])
REF_2983(uint256) (->$_4 (-> ['TMP_13004'])) := 0(uint256)
TMP_13004(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13004'])"])
$_5 (-> ['TMP_13004'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13004'])", "$_4 (-> ['TMP_13004'])"])
 AdminClearedSlashedStake(user,slashedValidatorId,userActiveStakeToClear)
Emit AdminClearedSlashedStake(user_1,slashedValidatorId_1,userActiveStakeToClear_1)
 recordActuallyChangedForThisUser = true
recordActuallyChangedForThisUser_2(bool) := True(bool)
$_6 (-> ['TMP_13004'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13004'])", "$_2 (-> ['TMP_13004'])"])
recordActuallyChangedForThisUser_3(bool) := phi(['recordActuallyChangedForThisUser_1', 'recordActuallyChangedForThisUser_2'])
 userCooledAmountToClear > 0
TMP_13017(bool) = userCooledAmountToClear_1 > 0
CONDITION TMP_13017
 cooldownIsLost = cooldownEntry.cooldownEndTime >= slashTimestamp
REF_2984(uint256) -> cooldownEntry_1 (-> ['$']).cooldownEndTime
TMP_13018(bool) = REF_2984 >= slashTimestamp_1
cooldownIsLost_1(bool) := TMP_13018(bool)
 cooldownIsLost
CONDITION cooldownIsLost_1
 delete $.userValidatorCooldowns[user][slashedValidatorId]
REF_2985(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_6 (-> ['TMP_13004']).userValidatorCooldowns
REF_2986(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_2985[user_1]
REF_2987(PlumeStakingStorage.CooldownEntry) -> REF_2986[slashedValidatorId_1]
REF_2986 = delete REF_2987 
 $.stakeInfo[user].cooled >= userCooledAmountToClear
REF_2988(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_6 (-> ['TMP_13004']).stakeInfo
REF_2989(PlumeStakingStorage.StakeInfo) -> REF_2988[user_1]
REF_2990(uint256) -> REF_2989.cooled
TMP_13019(bool) = REF_2990 >= userCooledAmountToClear_1
CONDITION TMP_13019
 $.stakeInfo[user].cooled -= userCooledAmountToClear
REF_2991(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_6 (-> ['TMP_13004']).stakeInfo
REF_2992(PlumeStakingStorage.StakeInfo) -> REF_2991[user_1]
REF_2993(uint256) -> REF_2992.cooled
$_7 (-> ['TMP_13004'])(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13004'])"])
REF_2993(-> $_7 (-> ['TMP_13004'])) = REF_2993 (c)- userCooledAmountToClear_1
TMP_13004(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_13004'])"])
 $.stakeInfo[user].cooled = 0
REF_2994(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_6 (-> ['TMP_13004']).stakeInfo
REF_2995(PlumeStakingStorage.StakeInfo) -> REF_2994[user_1]
REF_2996(uint256) -> REF_2995.cooled
$_8 (-> ['TMP_13004'])(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13004'])"])
REF_2996(uint256) (->$_8 (-> ['TMP_13004'])) := 0(uint256)
TMP_13004(PlumeStakingStorage.Layout) := phi(["$_8 (-> ['TMP_13004'])"])
$_9 (-> ['TMP_13004', 'TMP_13004'])(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_13004'])", "$_8 (-> ['TMP_13004'])"])
 AdminClearedSlashedCooldown(user,slashedValidatorId,userCooledAmountToClear)
Emit AdminClearedSlashedCooldown(user_1,slashedValidatorId_1,userCooledAmountToClear_1)
 recordActuallyChangedForThisUser = true
recordActuallyChangedForThisUser_4(bool) := True(bool)
$_10 (-> ['TMP_13004'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13004'])", "$_6 (-> ['TMP_13004'])"])
recordActuallyChangedForThisUser_5(bool) := phi(['recordActuallyChangedForThisUser_1', 'recordActuallyChangedForThisUser_4'])
 $.userHasStakedWithValidator[user][slashedValidatorId] || recordActuallyChangedForThisUser
REF_2997(mapping(address => mapping(uint16 => bool))) -> $_10 (-> ['TMP_13004']).userHasStakedWithValidator
REF_2998(mapping(uint16 => bool)) -> REF_2997[user_1]
REF_2999(bool) -> REF_2998[slashedValidatorId_1]
TMP_13021(bool) = REF_2999 || recordActuallyChangedForThisUser_5
CONDITION TMP_13021
 PlumeValidatorLogic.removeStakerFromValidator($,user,slashedValidatorId)
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.removeStakerFromValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_10 (-> ['TMP_13004'])", 'user_1', 'slashedValidatorId_1'] 
 i ++
TMP_13023(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_3001(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_3001)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_3002(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_3002)
```
#### ManagementFacet.addHistoricalRewardToken(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13026(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13026'])(PlumeStakingStorage.Layout) := TMP_13026(PlumeStakingStorage.Layout)
 token == address(0)
TMP_13027 = CONVERT 0 to address
TMP_13028(bool) = token_1 == TMP_13027
CONDITION TMP_13028
 revert ZeroAddress(string)(token)
TMP_13029(None) = SOLIDITY_CALL revert ZeroAddress(string)(token)
 $.isHistoricalRewardToken[token]
REF_3004(mapping(address => bool)) -> $_1 (-> ['TMP_13026']).isHistoricalRewardToken
REF_3005(bool) -> REF_3004[token_1]
CONDITION REF_3005
 revert TokenAlreadyExists()()
TMP_13030(None) = SOLIDITY_CALL revert TokenAlreadyExists()()
 $.isHistoricalRewardToken[token] = true
REF_3006(mapping(address => bool)) -> $_1 (-> ['TMP_13026']).isHistoricalRewardToken
REF_3007(bool) -> REF_3006[token_1]
$_2 (-> ['TMP_13026'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13026'])"])
REF_3007(bool) (->$_2 (-> ['TMP_13026'])) := True(bool)
TMP_13026(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13026'])"])
 $.historicalRewardTokens.push(token)
REF_3008(address[]) -> $_2 (-> ['TMP_13026']).historicalRewardTokens
REF_3010 -> LENGTH REF_3008
TMP_13032(uint256) := REF_3010(uint256)
TMP_13033(uint256) = TMP_13032 (c)+ 1
$_3 (-> ['TMP_13026'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13026'])"])
REF_3010(uint256) (->$_4 (-> ['TMP_13026'])) := TMP_13033(uint256)
REF_3011(address) -> REF_3008[TMP_13032]
$_4 (-> ['TMP_13026'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13026'])"])
REF_3011(address) (->$_4 (-> ['TMP_13026'])) := token_1(address)
TMP_13026(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13026'])"])
 HistoricalRewardTokenAdded(token)
Emit HistoricalRewardTokenAdded(token_1)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_3012(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_3012)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_3013(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_3013)
```
#### ManagementFacet.removeHistoricalRewardToken(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13037(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13037'])(PlumeStakingStorage.Layout) := TMP_13037(PlumeStakingStorage.Layout)
 token == address(0)
TMP_13038 = CONVERT 0 to address
TMP_13039(bool) = token_1 == TMP_13038
CONDITION TMP_13039
 revert ZeroAddress(string)(token)
TMP_13040(None) = SOLIDITY_CALL revert ZeroAddress(string)(token)
 $.isRewardToken[token]
REF_3015(mapping(address => bool)) -> $_1 (-> ['TMP_13037']).isRewardToken
REF_3016(bool) -> REF_3015[token_1]
CONDITION REF_3016
 revert TokenAlreadyExists()()
TMP_13041(None) = SOLIDITY_CALL revert TokenAlreadyExists()()
 historicalTokens = $.historicalRewardTokens
REF_3017(address[]) -> $_1 (-> ['TMP_13037']).historicalRewardTokens
historicalTokens_1 (-> [])(address[]) = ['REF_3017(address[])']
 tokenIndex = type()(uint256).max
TMP_13043(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
tokenIndex_1(uint256) := TMP_13043(uint256)
tokenIndex_3(uint256) := phi(['tokenIndex_1', 'tokenIndex_2'])
 i = 0
i_1(uint256) := 0(uint256)
 i < historicalTokens.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3018 -> LENGTH historicalTokens_1 (-> [])
TMP_13044(bool) = i_2 < REF_3018
CONDITION TMP_13044
 historicalTokens[i] == token
REF_3019(address) -> historicalTokens_1 (-> [])[i_2]
TMP_13045(bool) = REF_3019 == token_1
CONDITION TMP_13045
 tokenIndex = i
tokenIndex_2(uint256) := i_2(uint256)
 i ++
TMP_13046(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 tokenIndex == type()(uint256).max
TMP_13048(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_13049(bool) = tokenIndex_3 == TMP_13048
CONDITION TMP_13049
 revert TokenDoesNotExist(address)(token)
TMP_13050(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 historicalTokens[tokenIndex] = historicalTokens[historicalTokens.length - 1]
REF_3020(address) -> historicalTokens_1 (-> [])[tokenIndex_3]
REF_3021 -> LENGTH historicalTokens_1 (-> [])
TMP_13051(uint256) = REF_3021 (c)- 1
REF_3022(address) -> historicalTokens_1 (-> [])[TMP_13051]
historicalTokens_2 (-> [])(address[]) := phi(['historicalTokens_1 (-> [])'])
REF_3020(address) (->historicalTokens_2 (-> [])) := REF_3022(address)
 historicalTokens.pop()
REF_3024 -> LENGTH historicalTokens_2 (-> [])
TMP_13053(uint256) = REF_3024 (c)- 1
REF_3025(address) -> historicalTokens_2 (-> [])[TMP_13053]
historicalTokens_3 (-> []) = delete REF_3025 
REF_3026 -> LENGTH historicalTokens_3 (-> [])
historicalTokens_4 (-> [])(address[]) := phi(['historicalTokens_3 (-> [])'])
REF_3026(uint256) (->historicalTokens_4 (-> [])) := TMP_13053(uint256)
 $.isHistoricalRewardToken[token] = false
REF_3027(mapping(address => bool)) -> $_1 (-> ['TMP_13037']).isHistoricalRewardToken
REF_3028(bool) -> REF_3027[token_1]
$_2 (-> ['TMP_13037'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13037'])"])
REF_3028(bool) (->$_2 (-> ['TMP_13037'])) := False(bool)
TMP_13037(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13037'])"])
 HistoricalRewardTokenRemoved(token)
Emit HistoricalRewardTokenRemoved(token_1)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_3029(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_3029)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_3030(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_3030)
```
#### ManagementFacet.isHistoricalRewardToken(address) [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().isHistoricalRewardToken[token]
TMP_13057(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_3032(mapping(address => bool)) -> TMP_13057.isHistoricalRewardToken
REF_3033(bool) -> REF_3032[token_1]
RETURN REF_3033
```
#### ManagementFacet.getHistoricalRewardTokens() [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().historicalRewardTokens
TMP_13058(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_3035(address[]) -> TMP_13058.historicalRewardTokens
RETURN REF_3035
```
#### ManagementFacet.adminCreateHistoricalRewardCheckpoint(uint16,address,uint256,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13059(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13059'])(PlumeStakingStorage.Layout) := TMP_13059(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_3037(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13059']).validatorExists
REF_3038(bool) -> REF_3037[validatorId_1]
TMP_13060 = UnaryType.BANG REF_3038 
CONDITION TMP_13060
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13061(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 ! $.isHistoricalRewardToken[token]
REF_3039(mapping(address => bool)) -> $_1 (-> ['TMP_13059']).isHistoricalRewardToken
REF_3040(bool) -> REF_3039[token_1]
TMP_13062 = UnaryType.BANG REF_3040 
CONDITION TMP_13062
 revert TokenDoesNotExist(address)(token)
TMP_13063(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 checkpoint = PlumeStakingStorage.RateCheckpoint({timestamp:timestamp,rate:rate,cumulativeIndex:0})
TMP_13064(PlumeStakingStorage.RateCheckpoint) = new RateCheckpoint(timestamp_1,rate_1,0)
checkpoint_1(PlumeStakingStorage.RateCheckpoint) := TMP_13064(PlumeStakingStorage.RateCheckpoint)
 $.validatorRewardRateCheckpoints[validatorId][token].push(checkpoint)
REF_3042(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> ['TMP_13059']).validatorRewardRateCheckpoints
REF_3043(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_3042[validatorId_1]
REF_3044(PlumeStakingStorage.RateCheckpoint[]) -> REF_3043[token_1]
REF_3046 -> LENGTH REF_3044
TMP_13066(uint256) := REF_3046(uint256)
TMP_13067(uint256) = TMP_13066 (c)+ 1
$_2 (-> ['TMP_13059'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13059'])"])
REF_3046(uint256) (->$_3 (-> ['TMP_13059'])) := TMP_13067(uint256)
REF_3047(PlumeStakingStorage.RateCheckpoint) -> REF_3044[TMP_13066]
$_3 (-> ['TMP_13059'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13059'])"])
REF_3047(PlumeStakingStorage.RateCheckpoint) (->$_3 (-> ['TMP_13059'])) := checkpoint_1(PlumeStakingStorage.RateCheckpoint)
TMP_13059(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13059'])"])
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_3048(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_3048)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_3049(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ManagementFacet.onlyRole(bytes32)(REF_3049)
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
#### PlumeRewardLogic._settleCommissionForValidatorUpToNow(PlumeStakingStorage.Layout,uint16) [INTERNAL]
```slithir
 rewardTokens = $.rewardTokens
REF_4413(address[]) -> $_1 (-> []).rewardTokens
rewardTokens_1(address[]) = ['REF_4413(address[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < rewardTokens.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4414 -> LENGTH rewardTokens_1
TMP_14215(bool) = i_2 < REF_4414
CONDITION TMP_14215
 token = rewardTokens[i]
REF_4415(address) -> rewardTokens_1[i_2]
token_1(address) := REF_4415(address)
 updateRewardPerTokenForValidator($,token,validatorId)
INTERNAL_CALL, PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16)($_1 (-> []),token_1,validatorId_1)
 i ++
TMP_14217(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### PlumeRewardLogic.createCommissionRateCheckpoint(PlumeStakingStorage.Layout,uint16,uint256) [INTERNAL]
```slithir
 checkpoints = $.validatorCommissionCheckpoints[validatorId]
REF_4400(mapping(uint16 => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> []).validatorCommissionCheckpoints
REF_4401(PlumeStakingStorage.RateCheckpoint[]) -> REF_4400[validatorId_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4401(PlumeStakingStorage.RateCheckpoint[])']
 len = checkpoints.length
REF_4402 -> LENGTH checkpoints_1 (-> [])
len_1(uint256) := REF_4402(uint256)
 checkpoint = PlumeStakingStorage.RateCheckpoint({timestamp:block.timestamp,rate:commissionRate,cumulativeIndex:0})
TMP_14201(PlumeStakingStorage.RateCheckpoint) = new RateCheckpoint(block.timestamp,commissionRate_1,0)
checkpoint_1(PlumeStakingStorage.RateCheckpoint) := TMP_14201(PlumeStakingStorage.RateCheckpoint)
 len > 0 && checkpoints[len - 1].timestamp == block.timestamp
TMP_14202(bool) = len_1 > 0
TMP_14203(uint256) = len_1 (c)- 1
REF_4404(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[TMP_14203]
REF_4405(uint256) -> REF_4404.timestamp
TMP_14204(bool) = REF_4405 == block.timestamp
TMP_14205(bool) = TMP_14202 && TMP_14204
CONDITION TMP_14205
 checkpoints[len - 1] = checkpoint
TMP_14206(uint256) = len_1 (c)- 1
REF_4406(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[TMP_14206]
checkpoints_4 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_1 (-> [])'])
REF_4406(PlumeStakingStorage.RateCheckpoint) (->checkpoints_4 (-> [])) := checkpoint_1(PlumeStakingStorage.RateCheckpoint)
 $.maxCommissionCheckpoints > 0 && len >= $.maxCommissionCheckpoints
REF_4407(uint16) -> $_1 (-> []).maxCommissionCheckpoints
TMP_14207(bool) = REF_4407 > 0
REF_4408(uint16) -> $_1 (-> []).maxCommissionCheckpoints
TMP_14208(bool) = len_1 >= REF_4408
TMP_14209(bool) = TMP_14207 && TMP_14208
CONDITION TMP_14209
 revert MaxCommissionCheckpointsExceeded(uint16,uint256)(validatorId,$.maxCommissionCheckpoints)
REF_4409(uint16) -> $_1 (-> []).maxCommissionCheckpoints
TMP_14210(None) = SOLIDITY_CALL revert MaxCommissionCheckpointsExceeded(uint16,uint256)(validatorId_1,REF_4409)
 checkpoints.push(checkpoint)
REF_4411 -> LENGTH checkpoints_1 (-> [])
TMP_14212(uint256) := REF_4411(uint256)
TMP_14213(uint256) = TMP_14212 (c)+ 1
checkpoints_2 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_1 (-> [])'])
REF_4411(uint256) (->checkpoints_2 (-> [])) := TMP_14213(uint256)
REF_4412(PlumeStakingStorage.RateCheckpoint) -> checkpoints_2 (-> [])[TMP_14212]
checkpoints_3 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_2 (-> [])'])
REF_4412(PlumeStakingStorage.RateCheckpoint) (->checkpoints_3 (-> [])) := checkpoint_1(PlumeStakingStorage.RateCheckpoint)
 ValidatorCommissionCheckpointCreated(validatorId,commissionRate,block.timestamp)
Emit ValidatorCommissionCheckpointCreated(validatorId_1,commissionRate_1,block.timestamp)
```
#### PlumeValidatorLogic.removeStakerFromValidator(PlumeStakingStorage.Layout,address,uint16) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
staker_1(address) := phi(['staker_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
 $.userValidatorStakes[staker][validatorId].staked == 0 && $.isStakerForValidator[validatorId][staker]
REF_4486(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> []).userValidatorStakes
REF_4487(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_4486[staker_1]
REF_4488(PlumeStakingStorage.UserValidatorStake) -> REF_4487[validatorId_1]
REF_4489(uint256) -> REF_4488.staked
TMP_14255(bool) = REF_4489 == 0
REF_4490(mapping(uint16 => mapping(address => bool))) -> $_1 (-> []).isStakerForValidator
REF_4491(mapping(address => bool)) -> REF_4490[validatorId_1]
REF_4492(bool) -> REF_4491[staker_1]
TMP_14256(bool) = TMP_14255 && REF_4492
CONDITION TMP_14256
 stakersList = $.validatorStakers[validatorId]
REF_4493(mapping(uint16 => address[])) -> $_1 (-> []).validatorStakers
REF_4494(address[]) -> REF_4493[validatorId_1]
stakersList_1 (-> [])(address[]) = ['REF_4494(address[])']
 listLength = stakersList.length
REF_4495 -> LENGTH stakersList_1 (-> [])
listLength_1(uint256) := REF_4495(uint256)
 listLength > 0
TMP_14257(bool) = listLength_1 > 0
CONDITION TMP_14257
 indexToRemove = $.userIndexInValidatorStakers[staker][validatorId]
REF_4496(mapping(address => mapping(uint16 => uint256))) -> $_1 (-> []).userIndexInValidatorStakers
REF_4497(mapping(uint16 => uint256)) -> REF_4496[staker_1]
REF_4498(uint256) -> REF_4497[validatorId_1]
indexToRemove_1(uint256) := REF_4498(uint256)
 indexToRemove < listLength && stakersList[indexToRemove] == staker
TMP_14258(bool) = indexToRemove_1 < listLength_1
REF_4499(address) -> stakersList_1 (-> [])[indexToRemove_1]
TMP_14259(bool) = REF_4499 == staker_1
TMP_14260(bool) = TMP_14258 && TMP_14259
CONDITION TMP_14260
 lastStaker = stakersList[listLength - 1]
TMP_14261(uint256) = listLength_1 (c)- 1
REF_4500(address) -> stakersList_1 (-> [])[TMP_14261]
lastStaker_1(address) := REF_4500(address)
 indexToRemove != listLength - 1
TMP_14262(uint256) = listLength_1 (c)- 1
TMP_14263(bool) = indexToRemove_1 != TMP_14262
CONDITION TMP_14263
 stakersList[indexToRemove] = lastStaker
REF_4501(address) -> stakersList_1 (-> [])[indexToRemove_1]
stakersList_2 (-> [])(address[]) := phi(['stakersList_1 (-> [])'])
REF_4501(address) (->stakersList_2 (-> [])) := lastStaker_1(address)
 $.userIndexInValidatorStakers[lastStaker][validatorId] = indexToRemove
REF_4502(mapping(address => mapping(uint16 => uint256))) -> $_1 (-> []).userIndexInValidatorStakers
REF_4503(mapping(uint16 => uint256)) -> REF_4502[lastStaker_1]
REF_4504(uint256) -> REF_4503[validatorId_1]
$_2 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4504(uint256) (->$_2 (-> [])) := indexToRemove_1(uint256)
$_3 (-> [])(PlumeStakingStorage.Layout) := phi(['$_2 (-> [])', '$_1 (-> [])'])
stakersList_3 (-> [])(address[]) := phi(['stakersList_1 (-> [])', 'stakersList_2 (-> [])'])
 stakersList.pop()
REF_4506 -> LENGTH stakersList_3 (-> [])
TMP_14265(uint256) = REF_4506 (c)- 1
REF_4507(address) -> stakersList_3 (-> [])[TMP_14265]
stakersList_4 (-> []) = delete REF_4507 
REF_4508 -> LENGTH stakersList_4 (-> [])
stakersList_5 (-> [])(address[]) := phi(['stakersList_4 (-> [])'])
REF_4508(uint256) (->stakersList_5 (-> [])) := TMP_14265(uint256)
 $.isStakerForValidator[validatorId][staker] = false
REF_4509(mapping(uint16 => mapping(address => bool))) -> $_3 (-> []).isStakerForValidator
REF_4510(mapping(address => bool)) -> REF_4509[validatorId_1]
REF_4511(bool) -> REF_4510[staker_1]
$_4 (-> [])(PlumeStakingStorage.Layout) := phi(['$_3 (-> [])'])
REF_4511(bool) (->$_4 (-> [])) := False(bool)
 delete $.userIndexInValidatorStakers[staker][validatorId]
REF_4512(mapping(address => mapping(uint16 => uint256))) -> $_4 (-> []).userIndexInValidatorStakers
REF_4513(mapping(uint16 => uint256)) -> REF_4512[staker_1]
REF_4514(uint256) -> REF_4513[validatorId_1]
REF_4513 = delete REF_4514 
$_5 (-> [])(PlumeStakingStorage.Layout) := phi(['$_4 (-> [])', '$_1 (-> [])'])
 hasActiveStakeForThisVal = $.userValidatorStakes[staker][validatorId].staked > 0
REF_4515(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_5 (-> []).userValidatorStakes
REF_4516(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_4515[staker_1]
REF_4517(PlumeStakingStorage.UserValidatorStake) -> REF_4516[validatorId_1]
REF_4518(uint256) -> REF_4517.staked
TMP_14266(bool) = REF_4518 > 0
hasActiveStakeForThisVal_1(bool) := TMP_14266(bool)
 hasActiveCooldownForThisVal = $.userValidatorCooldowns[staker][validatorId].amount > 0
REF_4519(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_5 (-> []).userValidatorCooldowns
REF_4520(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_4519[staker_1]
REF_4521(PlumeStakingStorage.CooldownEntry) -> REF_4520[validatorId_1]
REF_4522(uint256) -> REF_4521.amount
TMP_14267(bool) = REF_4522 > 0
hasActiveCooldownForThisVal_1(bool) := TMP_14267(bool)
 hasPendingRewardsForThisVal = $.userHasPendingRewards[staker][validatorId]
REF_4523(mapping(address => mapping(uint16 => bool))) -> $_5 (-> []).userHasPendingRewards
REF_4524(mapping(uint16 => bool)) -> REF_4523[staker_1]
REF_4525(bool) -> REF_4524[validatorId_1]
hasPendingRewardsForThisVal_1(bool) := REF_4525(bool)
 ! hasActiveStakeForThisVal && ! hasActiveCooldownForThisVal && ! hasPendingRewardsForThisVal
TMP_14268 = UnaryType.BANG hasActiveStakeForThisVal_1 
TMP_14269 = UnaryType.BANG hasActiveCooldownForThisVal_1 
TMP_14270(bool) = TMP_14268 && TMP_14269
TMP_14271 = UnaryType.BANG hasPendingRewardsForThisVal_1 
TMP_14272(bool) = TMP_14270 && TMP_14271
CONDITION TMP_14272
 $.userHasStakedWithValidator[staker][validatorId]
REF_4526(mapping(address => mapping(uint16 => bool))) -> $_5 (-> []).userHasStakedWithValidator
REF_4527(mapping(uint16 => bool)) -> REF_4526[staker_1]
REF_4528(bool) -> REF_4527[validatorId_1]
CONDITION REF_4528
 userValidators_ = $.userValidators[staker]
REF_4529(mapping(address => uint16[])) -> $_5 (-> []).userValidators
REF_4530(uint16[]) -> REF_4529[staker_1]
userValidators__1 (-> [])(uint16[]) = ['REF_4530(uint16[])']
 removed = false
removed_1(bool) := False(bool)
removed_3(bool) := phi(['removed_2', 'removed_1'])
 i = 0
i_1(uint256) := 0(uint256)
 i < userValidators_.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4531 -> LENGTH userValidators__1 (-> [])
TMP_14273(bool) = i_2 < REF_4531
CONDITION TMP_14273
 userValidators_[i] == validatorId
REF_4532(uint16) -> userValidators__1 (-> [])[i_2]
TMP_14274(bool) = REF_4532 == validatorId_1
CONDITION TMP_14274
 userValidators_[i] = userValidators_[userValidators_.length - 1]
REF_4533(uint16) -> userValidators__1 (-> [])[i_2]
REF_4534 -> LENGTH userValidators__1 (-> [])
TMP_14275(uint256) = REF_4534 (c)- 1
REF_4535(uint16) -> userValidators__1 (-> [])[TMP_14275]
userValidators__2 (-> [])(uint16[]) := phi(['userValidators__1 (-> [])'])
REF_4533(uint16) (->userValidators__2 (-> [])) := REF_4535(uint16)
 userValidators_.pop()
REF_4537 -> LENGTH userValidators__2 (-> [])
TMP_14277(uint256) = REF_4537 (c)- 1
REF_4538(uint16) -> userValidators__2 (-> [])[TMP_14277]
userValidators__3 (-> []) = delete REF_4538 
REF_4539 -> LENGTH userValidators__3 (-> [])
userValidators__4 (-> [])(uint16[]) := phi(['userValidators__3 (-> [])'])
REF_4539(uint256) (->userValidators__4 (-> [])) := TMP_14277(uint256)
 removed = true
removed_2(bool) := True(bool)
 i ++
TMP_14278(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 removed
CONDITION removed_3
 $.userHasStakedWithValidator[staker][validatorId] = false
REF_4540(mapping(address => mapping(uint16 => bool))) -> $_5 (-> []).userHasStakedWithValidator
REF_4541(mapping(uint16 => bool)) -> REF_4540[staker_1]
REF_4542(bool) -> REF_4541[validatorId_1]
$_6 (-> [])(PlumeStakingStorage.Layout) := phi(['$_5 (-> [])'])
REF_4542(bool) (->$_6 (-> [])) := False(bool)
```
#### PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_1 (-> [])', '$_1 (-> [])', '$_1 (-> [])'])
token_1(address) := phi(['token_1', 'token_1', 'token_1', 'token_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1', 'validatorId_1', 'validatorId_1'])
 validator = $.validators[validatorId]
REF_4252(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> []).validators
REF_4253(PlumeStakingStorage.ValidatorInfo) -> REF_4252[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4253(PlumeStakingStorage.ValidatorInfo)
 validator.slashed
REF_4254(bool) -> validator_1 (-> ['$']).slashed
CONDITION REF_4254
 $.validatorLastUpdateTimes[validatorId][token] = block.timestamp
REF_4255(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4256(mapping(address => uint256)) -> REF_4255[validatorId_1]
REF_4257(uint256) -> REF_4256[token_1]
$_2 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4257(uint256) (->$_2 (-> [])) := block.timestamp(uint256)
 $.validatorTotalStaked[validatorId] > 0
REF_4258(mapping(uint16 => uint256)) -> $_2 (-> []).validatorTotalStaked
REF_4259(uint256) -> REF_4258[validatorId_1]
TMP_14042(bool) = REF_4259 > 0
CONDITION TMP_14042
 revert InternalInconsistency(string)(Slashed validator has non-zero totalStaked)
TMP_14043(None) = SOLIDITY_CALL revert InternalInconsistency(string)(Slashed validator has non-zero totalStaked)
 ! validator.active
REF_4260(bool) -> validator_1 (-> ['$']).active
TMP_14044 = UnaryType.BANG REF_4260 
CONDITION TMP_14044
 $.validatorLastUpdateTimes[validatorId][token] = block.timestamp
REF_4261(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4262(mapping(address => uint256)) -> REF_4261[validatorId_1]
REF_4263(uint256) -> REF_4262[token_1]
$_8 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4263(uint256) (->$_8 (-> [])) := block.timestamp(uint256)
 totalStaked = $.validatorTotalStaked[validatorId]
REF_4264(mapping(uint16 => uint256)) -> $_1 (-> []).validatorTotalStaked
REF_4265(uint256) -> REF_4264[validatorId_1]
totalStaked_1(uint256) := REF_4265(uint256)
 oldLastUpdateTime = $.validatorLastUpdateTimes[validatorId][token]
REF_4266(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4267(mapping(address => uint256)) -> REF_4266[validatorId_1]
REF_4268(uint256) -> REF_4267[token_1]
oldLastUpdateTime_1(uint256) := REF_4268(uint256)
 block.timestamp > oldLastUpdateTime
TMP_14045(bool) = block.timestamp > oldLastUpdateTime_1
CONDITION TMP_14045
 totalStaked > 0
TMP_14046(bool) = totalStaked_1 > 0
CONDITION TMP_14046
 timeDelta = block.timestamp - oldLastUpdateTime
TMP_14047(uint256) = block.timestamp (c)- oldLastUpdateTime_1
timeDelta_1(uint256) := TMP_14047(uint256)
 effectiveRewardRateChk = getEffectiveRewardRateAt($,token,validatorId,block.timestamp)
TMP_14048(PlumeStakingStorage.RateCheckpoint) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256)($_1 (-> []),token_1,validatorId_1,block.timestamp)
effectiveRewardRateChk_1(PlumeStakingStorage.RateCheckpoint) := TMP_14048(PlumeStakingStorage.RateCheckpoint)
 effectiveRewardRate = effectiveRewardRateChk.rate
REF_4269(uint256) -> effectiveRewardRateChk_1.rate
effectiveRewardRate_1(uint256) := REF_4269(uint256)
 effectiveRewardRate > 0
TMP_14049(bool) = effectiveRewardRate_1 > 0
CONDITION TMP_14049
 rewardPerTokenIncrease = timeDelta * effectiveRewardRate
TMP_14050(uint256) = timeDelta_1 (c)* effectiveRewardRate_1
rewardPerTokenIncrease_1(uint256) := TMP_14050(uint256)
 $.validatorRewardPerTokenCumulative[validatorId][token] += rewardPerTokenIncrease
REF_4270(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorRewardPerTokenCumulative
REF_4271(mapping(address => uint256)) -> REF_4270[validatorId_1]
REF_4272(uint256) -> REF_4271[token_1]
$_3 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4272(-> $_3 (-> [])) = REF_4272 (c)+ rewardPerTokenIncrease_1
 commissionRateForSegment = getEffectiveCommissionRateAt($,validatorId,oldLastUpdateTime)
TMP_14051(uint256) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveCommissionRateAt(PlumeStakingStorage.Layout,uint16,uint256)($_3 (-> []),validatorId_1,oldLastUpdateTime_1)
commissionRateForSegment_1(uint256) := TMP_14051(uint256)
 grossRewardForValidatorThisSegment = (totalStaked * rewardPerTokenIncrease) / PlumeStakingStorage.REWARD_PRECISION
TMP_14052(uint256) = totalStaked_1 (c)* rewardPerTokenIncrease_1
REF_4273(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_14053(uint256) = TMP_14052 (c)/ REF_4273
grossRewardForValidatorThisSegment_1(uint256) := TMP_14053(uint256)
 commissionDeltaForValidator = (grossRewardForValidatorThisSegment * commissionRateForSegment) / PlumeStakingStorage.REWARD_PRECISION
TMP_14054(uint256) = grossRewardForValidatorThisSegment_1 (c)* commissionRateForSegment_1
REF_4274(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_14055(uint256) = TMP_14054 (c)/ REF_4274
commissionDeltaForValidator_1(uint256) := TMP_14055(uint256)
 commissionDeltaForValidator > 0
TMP_14056(bool) = commissionDeltaForValidator_1 > 0
CONDITION TMP_14056
 $.validatorAccruedCommission[validatorId][token] += commissionDeltaForValidator
REF_4275(mapping(uint16 => mapping(address => uint256))) -> $_3 (-> []).validatorAccruedCommission
REF_4276(mapping(address => uint256)) -> REF_4275[validatorId_1]
REF_4277(uint256) -> REF_4276[token_1]
$_4 (-> [])(PlumeStakingStorage.Layout) := phi(['$_3 (-> [])'])
REF_4277(-> $_4 (-> [])) = REF_4277 (c)+ commissionDeltaForValidator_1
$_5 (-> [])(PlumeStakingStorage.Layout) := phi(['$_4 (-> [])', '$_3 (-> [])'])
$_6 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_3 (-> [])'])
 $.validatorLastUpdateTimes[validatorId][token] = block.timestamp
REF_4278(mapping(uint16 => mapping(address => uint256))) -> $_6 (-> []).validatorLastUpdateTimes
REF_4279(mapping(address => uint256)) -> REF_4278[validatorId_1]
REF_4280(uint256) -> REF_4279[token_1]
$_7 (-> [])(PlumeStakingStorage.Layout) := phi(['$_6 (-> [])'])
REF_4280(uint256) (->$_7 (-> [])) := block.timestamp(uint256)
```
#### PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_1 (-> [])', '$_1 (-> [])', '$_1 (-> [])'])
token_1(address) := phi(['token_1', 'token_1', 'token_1', 'token_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1', 'validatorId_1', 'validatorId_1'])
timestamp_1(uint256) := phi(['block.timestamp', 'segmentStartTime_1', 'validatorLastUpdateTime_1', 'segmentStartTime_1'])
 checkpoints = $.validatorRewardRateCheckpoints[validatorId][token]
REF_4353(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> []).validatorRewardRateCheckpoints
REF_4354(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4353[validatorId_1]
REF_4355(PlumeStakingStorage.RateCheckpoint[]) -> REF_4354[token_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4355(PlumeStakingStorage.RateCheckpoint[])']
 chkCount = checkpoints.length
REF_4356 -> LENGTH checkpoints_1 (-> [])
chkCount_1(uint256) := REF_4356(uint256)
 chkCount > 0
TMP_14154(bool) = chkCount_1 > 0
CONDITION TMP_14154
 idx = findRewardRateCheckpointIndexAtOrBefore($,validatorId,token,timestamp)
TMP_14155(uint256) = INTERNAL_CALL, PlumeRewardLogic.findRewardRateCheckpointIndexAtOrBefore(PlumeStakingStorage.Layout,uint16,address,uint256)($_1 (-> []),validatorId_1,token_1,timestamp_1)
idx_1(uint256) := TMP_14155(uint256)
 idx < chkCount && checkpoints[idx].timestamp <= timestamp
TMP_14156(bool) = idx_1 < chkCount_1
REF_4357(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
REF_4358(uint256) -> REF_4357.timestamp
TMP_14157(bool) = REF_4358 <= timestamp_1
TMP_14158(bool) = TMP_14156 && TMP_14157
CONDITION TMP_14158
 idx + 1 < chkCount && checkpoints[idx + 1].timestamp <= timestamp
TMP_14159(uint256) = idx_1 (c)+ 1
TMP_14160(bool) = TMP_14159 < chkCount_1
TMP_14161(uint256) = idx_1 (c)+ 1
REF_4359(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[TMP_14161]
REF_4360(uint256) -> REF_4359.timestamp
TMP_14162(bool) = REF_4360 <= timestamp_1
TMP_14163(bool) = TMP_14160 && TMP_14162
CONDITION TMP_14163
 checkpoints[idx]
REF_4361(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
RETURN REF_4361
 effectiveCheckpoint.rate = 0
REF_4362(uint256) -> effectiveCheckpoint_0.rate
effectiveCheckpoint_1(PlumeStakingStorage.RateCheckpoint) := phi(['effectiveCheckpoint_0'])
REF_4362(uint256) (->effectiveCheckpoint_1) := 0(uint256)
 effectiveCheckpoint.timestamp = timestamp
REF_4363(uint256) -> effectiveCheckpoint_1.timestamp
effectiveCheckpoint_2(PlumeStakingStorage.RateCheckpoint) := phi(['effectiveCheckpoint_1'])
REF_4363(uint256) (->effectiveCheckpoint_2) := timestamp_1(uint256)
 effectiveCheckpoint.cumulativeIndex = 0
REF_4364(uint256) -> effectiveCheckpoint_2.cumulativeIndex
effectiveCheckpoint_3(PlumeStakingStorage.RateCheckpoint) := phi(['effectiveCheckpoint_2'])
REF_4364(uint256) (->effectiveCheckpoint_3) := 0(uint256)
 effectiveCheckpoint
RETURN effectiveCheckpoint_3
 effectiveCheckpoint
```
#### PlumeRewardLogic.getEffectiveCommissionRateAt(PlumeStakingStorage.Layout,uint16,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_3 (-> [])'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
timestamp_1(uint256) := phi(['oldLastUpdateTime_1', 'segmentStartTime_1'])
 checkpoints = $.validatorCommissionCheckpoints[validatorId]
REF_4365(mapping(uint16 => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> []).validatorCommissionCheckpoints
REF_4366(PlumeStakingStorage.RateCheckpoint[]) -> REF_4365[validatorId_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4366(PlumeStakingStorage.RateCheckpoint[])']
 chkCount = checkpoints.length
REF_4367 -> LENGTH checkpoints_1 (-> [])
chkCount_1(uint256) := REF_4367(uint256)
 chkCount > 0
TMP_14164(bool) = chkCount_1 > 0
CONDITION TMP_14164
 idx = findCommissionCheckpointIndexAtOrBefore($,validatorId,timestamp)
TMP_14165(uint256) = INTERNAL_CALL, PlumeRewardLogic.findCommissionCheckpointIndexAtOrBefore(PlumeStakingStorage.Layout,uint16,uint256)($_1 (-> []),validatorId_1,timestamp_1)
idx_1(uint256) := TMP_14165(uint256)
 idx < chkCount && checkpoints[idx].timestamp <= timestamp
TMP_14166(bool) = idx_1 < chkCount_1
REF_4368(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
REF_4369(uint256) -> REF_4368.timestamp
TMP_14167(bool) = REF_4369 <= timestamp_1
TMP_14168(bool) = TMP_14166 && TMP_14167
CONDITION TMP_14168
 checkpoints[idx].rate
REF_4370(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
REF_4371(uint256) -> REF_4370.rate
RETURN REF_4371
 fallbackComm = $.validators[validatorId].commission
REF_4372(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> []).validators
REF_4373(PlumeStakingStorage.ValidatorInfo) -> REF_4372[validatorId_1]
REF_4374(uint256) -> REF_4373.commission
fallbackComm_1(uint256) := REF_4374(uint256)
 fallbackComm
RETURN fallbackComm_1
```
