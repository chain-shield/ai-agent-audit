




#### StakingFacet._checkValidatorSlashedAndRevert(uint16) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13252(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13252'])(PlumeStakingStorage.Layout) := TMP_13252(PlumeStakingStorage.Layout)
 $.validatorExists[validatorId] && $.validators[validatorId].slashed
REF_3250(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13252']).validatorExists
REF_3251(bool) -> REF_3250[validatorId_1]
REF_3252(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13252']).validators
REF_3253(PlumeStakingStorage.ValidatorInfo) -> REF_3252[validatorId_1]
REF_3254(bool) -> REF_3253.slashed
TMP_13253(bool) = REF_3251 && REF_3254
CONDITION TMP_13253
 revert ActionOnSlashedValidatorError(uint16)(validatorId)
TMP_13254(None) = SOLIDITY_CALL revert ActionOnSlashedValidatorError(uint16)(validatorId_1)
```
#### StakingFacet._validateValidatorForStaking(uint16) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1', 'validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13255(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13255'])(PlumeStakingStorage.Layout) := TMP_13255(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_3256(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13255']).validatorExists
REF_3257(bool) -> REF_3256[validatorId_1]
TMP_13256 = UnaryType.BANG REF_3257 
CONDITION TMP_13256
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13257(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 _checkValidatorSlashedAndRevert(validatorId)
INTERNAL_CALL, StakingFacet._checkValidatorSlashedAndRevert(uint16)(validatorId_1)
 ! $.validators[validatorId].active
REF_3258(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13255']).validators
REF_3259(PlumeStakingStorage.ValidatorInfo) -> REF_3258[validatorId_1]
REF_3260(bool) -> REF_3259.active
TMP_13259 = UnaryType.BANG REF_3260 
CONDITION TMP_13259
 revert ValidatorInactive(uint16)(validatorId)
TMP_13260(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(validatorId_1)
```
#### StakingFacet._validateStakeAmount(uint256) [INTERNAL]
```slithir
amount_1(uint256) := phi(['amount_1'])
 $ = PlumeStakingStorage.layout()
TMP_13261(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13261'])(PlumeStakingStorage.Layout) := TMP_13261(PlumeStakingStorage.Layout)
 amount == 0
TMP_13262(bool) = amount_1 == 0
CONDITION TMP_13262
 revert InvalidAmount(uint256)(0)
TMP_13263(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(0)
 amount < $.minStakeAmount
REF_3262(uint256) -> $_1 (-> ['TMP_13261']).minStakeAmount
TMP_13264(bool) = amount_1 < REF_3262
CONDITION TMP_13264
 revert StakeAmountTooSmall(uint256,uint256)(amount,$.minStakeAmount)
REF_3263(uint256) -> $_1 (-> ['TMP_13261']).minStakeAmount
TMP_13265(None) = SOLIDITY_CALL revert StakeAmountTooSmall(uint256,uint256)(amount_1,REF_3263)
```
#### StakingFacet._validateStaking(uint16,uint256) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
amount_1(uint256) := phi(['amount_1', 'stakeAmount_1'])
 _validateValidatorForStaking(validatorId)
INTERNAL_CALL, StakingFacet._validateValidatorForStaking(uint16)(validatorId_1)
 _validateStakeAmount(amount)
INTERNAL_CALL, StakingFacet._validateStakeAmount(uint256)(amount_1)
```
#### StakingFacet._validateValidatorCapacity(uint16,uint256) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1'])
stakeAmount_1(uint256) := phi(['stakeAmount_1'])
 $ = PlumeStakingStorage.layout()
TMP_13268(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13268'])(PlumeStakingStorage.Layout) := TMP_13268(PlumeStakingStorage.Layout)
 newDelegatedAmount = $.validators[validatorId].delegatedAmount
REF_3265(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13268']).validators
REF_3266(PlumeStakingStorage.ValidatorInfo) -> REF_3265[validatorId_1]
REF_3267(uint256) -> REF_3266.delegatedAmount
newDelegatedAmount_1(uint256) := REF_3267(uint256)
 maxCapacity = $.validators[validatorId].maxCapacity
REF_3268(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13268']).validators
REF_3269(PlumeStakingStorage.ValidatorInfo) -> REF_3268[validatorId_1]
REF_3270(uint256) -> REF_3269.maxCapacity
maxCapacity_1(uint256) := REF_3270(uint256)
 maxCapacity > 0 && newDelegatedAmount > maxCapacity
TMP_13269(bool) = maxCapacity_1 > 0
TMP_13270(bool) = newDelegatedAmount_1 > maxCapacity_1
TMP_13271(bool) = TMP_13269 && TMP_13270
CONDITION TMP_13271
 revert ExceedsValidatorCapacity(uint16,uint256,uint256,uint256)(validatorId,newDelegatedAmount,maxCapacity,stakeAmount)
TMP_13272(None) = SOLIDITY_CALL revert ExceedsValidatorCapacity(uint16,uint256,uint256,uint256)(validatorId_1,newDelegatedAmount_1,maxCapacity_1,stakeAmount_1)
```
#### StakingFacet._validateValidatorPercentage(uint16) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13273(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13273'])(PlumeStakingStorage.Layout) := TMP_13273(PlumeStakingStorage.Layout)
 $.totalStaked > 0 && $.maxValidatorPercentage > 0
REF_3272(uint256) -> $_1 (-> ['TMP_13273']).totalStaked
TMP_13274(bool) = REF_3272 > 0
REF_3273(uint256) -> $_1 (-> ['TMP_13273']).maxValidatorPercentage
TMP_13275(bool) = REF_3273 > 0
TMP_13276(bool) = TMP_13274 && TMP_13275
CONDITION TMP_13276
 newDelegatedAmount = $.validators[validatorId].delegatedAmount
REF_3274(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13273']).validators
REF_3275(PlumeStakingStorage.ValidatorInfo) -> REF_3274[validatorId_1]
REF_3276(uint256) -> REF_3275.delegatedAmount
newDelegatedAmount_1(uint256) := REF_3276(uint256)
 validatorPercentage = (newDelegatedAmount * 10_000) / $.totalStaked
TMP_13277(uint256) = newDelegatedAmount_1 (c)* 10000
REF_3277(uint256) -> $_1 (-> ['TMP_13273']).totalStaked
TMP_13278(uint256) = TMP_13277 (c)/ REF_3277
validatorPercentage_1(uint256) := TMP_13278(uint256)
 validatorPercentage > $.maxValidatorPercentage
REF_3278(uint256) -> $_1 (-> ['TMP_13273']).maxValidatorPercentage
TMP_13279(bool) = validatorPercentage_1 > REF_3278
CONDITION TMP_13279
 revert ValidatorPercentageExceeded()()
TMP_13280(None) = SOLIDITY_CALL revert ValidatorPercentageExceeded()()
```
#### StakingFacet._validateCapacityLimits(uint16,uint256) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
stakeAmount_1(uint256) := phi(['stakeAmount_1', 'amountRestaked_1'])
 _validateValidatorCapacity(validatorId,stakeAmount)
INTERNAL_CALL, StakingFacet._validateValidatorCapacity(uint16,uint256)(validatorId_1,stakeAmount_1)
 _validateValidatorPercentage(validatorId)
INTERNAL_CALL, StakingFacet._validateValidatorPercentage(uint16)(validatorId_1)
```
#### StakingFacet._validateValidatorForUnstaking(uint16) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13283(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13283'])(PlumeStakingStorage.Layout) := TMP_13283(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_3280(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13283']).validatorExists
REF_3281(bool) -> REF_3280[validatorId_1]
TMP_13284 = UnaryType.BANG REF_3281 
CONDITION TMP_13284
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13285(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 _checkValidatorSlashedAndRevert(validatorId)
INTERNAL_CALL, StakingFacet._checkValidatorSlashedAndRevert(uint16)(validatorId_1)
```
#### StakingFacet._performStakeSetup(address,uint16,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender', 'staker_1', 'user_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1', 'validatorId_1'])
stakeAmount_1(uint256) := phi(['amount_1', 'stakeAmount_1', 'stakeAmount_1'])
 $ = PlumeStakingStorage.layout()
TMP_13287(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13287'])(PlumeStakingStorage.Layout) := TMP_13287(PlumeStakingStorage.Layout)
 _validateStaking(validatorId,stakeAmount)
INTERNAL_CALL, StakingFacet._validateStaking(uint16,uint256)(validatorId_1,stakeAmount_1)
 isNewStake = $.userValidatorStakes[user][validatorId].staked == 0
REF_3283(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13287']).userValidatorStakes
REF_3284(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3283[user_1]
REF_3285(PlumeStakingStorage.UserValidatorStake) -> REF_3284[validatorId_1]
REF_3286(uint256) -> REF_3285.staked
TMP_13289(bool) = REF_3286 == 0
isNewStake_1(bool) := TMP_13289(bool)
 ! isNewStake
TMP_13290 = UnaryType.BANG isNewStake_1 
CONDITION TMP_13290
 PlumeRewardLogic.updateRewardsForValidator($,user,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardsForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13287'])", 'user_1', 'validatorId_1'] 
 _initializeRewardStateForNewStake(user,validatorId)
INTERNAL_CALL, StakingFacet._initializeRewardStateForNewStake(address,uint16)(user_1,validatorId_1)
 _updateStakeAmounts(user,validatorId,stakeAmount)
INTERNAL_CALL, StakingFacet._updateStakeAmounts(address,uint16,uint256)(user_1,validatorId_1,stakeAmount_1)
 _validateCapacityLimits(validatorId,stakeAmount)
INTERNAL_CALL, StakingFacet._validateCapacityLimits(uint16,uint256)(validatorId_1,stakeAmount_1)
 PlumeValidatorLogic.addStakerToValidator($,user,validatorId)
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.addStakerToValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13287'])", 'user_1', 'validatorId_1'] 
 isNewStake
RETURN isNewStake_1
```
#### StakingFacet._performRestakeWorkflow(address,uint16,uint256,string) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
amount_1(uint256) := phi(['amountRestaked_1'])
 _validateStaking(validatorId,amount)
INTERNAL_CALL, StakingFacet._validateStaking(uint16,uint256)(validatorId_1,amount_1)
 PlumeRewardLogic.updateRewardsForValidator(PlumeStakingStorage.layout(),user,validatorId)
TMP_13297(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardsForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:['TMP_13297', 'user_1', 'validatorId_1'] 
 _updateStakeAmounts(user,validatorId,amount)
INTERNAL_CALL, StakingFacet._updateStakeAmounts(address,uint16,uint256)(user_1,validatorId_1,amount_1)
 PlumeValidatorLogic.addStakerToValidator(PlumeStakingStorage.layout(),user,validatorId)
TMP_13300(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.addStakerToValidator(PlumeStakingStorage.Layout,address,uint16), arguments:['TMP_13300', 'user_1', 'validatorId_1']
```
#### StakingFacet.stake(uint16) [EXTERNAL]
```slithir
 stakeAmount = msg.value
stakeAmount_1(uint256) := msg.value(uint256)
 isNewStake = _performStakeSetup(msg.sender,validatorId,stakeAmount)
TMP_13302(bool) = INTERNAL_CALL, StakingFacet._performStakeSetup(address,uint16,uint256)(msg.sender,validatorId_1,stakeAmount_1)
isNewStake_1(bool) := TMP_13302(bool)
 Staked(msg.sender,validatorId,stakeAmount,0,0,stakeAmount)
Emit Staked(msg.sender,validatorId_1,stakeAmount_1,0,0,stakeAmount_1)
 stakeAmount
RETURN stakeAmount_1
```
#### StakingFacet.restake(uint16,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13304(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13304'])(PlumeStakingStorage.Layout) := TMP_13304(PlumeStakingStorage.Layout)
 user = msg.sender
user_1(address) := msg.sender(address)
 amount == 0
TMP_13305(bool) = amount_1 == 0
CONDITION TMP_13305
 revert InvalidAmount(uint256)(0)
TMP_13306(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(0)
 coolingAmount = $.stakeInfo[user].cooled
REF_3294(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13304']).stakeInfo
REF_3295(PlumeStakingStorage.StakeInfo) -> REF_3294[user_1]
REF_3296(uint256) -> REF_3295.cooled
coolingAmount_1(uint256) := REF_3296(uint256)
 parkedAmount = $.stakeInfo[user].parked
REF_3297(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13304']).stakeInfo
REF_3298(PlumeStakingStorage.StakeInfo) -> REF_3297[user_1]
REF_3299(uint256) -> REF_3298.parked
parkedAmount_1(uint256) := REF_3299(uint256)
 totalAvailable = coolingAmount + parkedAmount
TMP_13307(uint256) = coolingAmount_1 (c)+ parkedAmount_1
totalAvailable_1(uint256) := TMP_13307(uint256)
 totalAvailable < amount
TMP_13308(bool) = totalAvailable_1 < amount_1
CONDITION TMP_13308
 revert InsufficientCooledAndParkedBalance(uint256,uint256)(totalAvailable,amount)
TMP_13309(None) = SOLIDITY_CALL revert InsufficientCooledAndParkedBalance(uint256,uint256)(totalAvailable_1,amount_1)
 _processMaturedCooldowns(user)
TMP_13310(uint256) = INTERNAL_CALL, StakingFacet._processMaturedCooldowns(address)(user_1)
 isNewStake = $.userValidatorStakes[user][validatorId].staked == 0
REF_3300(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13304']).userValidatorStakes
REF_3301(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3300[user_1]
REF_3302(PlumeStakingStorage.UserValidatorStake) -> REF_3301[validatorId_1]
REF_3303(uint256) -> REF_3302.staked
TMP_13311(bool) = REF_3303 == 0
isNewStake_1(bool) := TMP_13311(bool)
 _validateValidatorForStaking(validatorId)
INTERNAL_CALL, StakingFacet._validateValidatorForStaking(uint16)(validatorId_1)
 _performStakeSetup(user,validatorId,amount)
TMP_13313(bool) = INTERNAL_CALL, StakingFacet._performStakeSetup(address,uint16,uint256)(user_1,validatorId_1,amount_1)
 validatorsToCheck = new uint16[]($.userValidators[user].length)
REF_3304(mapping(address => uint16[])) -> $_1 (-> ['TMP_13304']).userValidators
REF_3305(uint16[]) -> REF_3304[user_1]
REF_3306 -> LENGTH REF_3305
TMP_13315(uint16[])  = new uint16[](REF_3306)
validatorsToCheck_1(uint16[]) = ['TMP_13315(uint16[])']
 checkCount = 0
checkCount_1(uint256) := 0(uint256)
 fromCooled = 0
fromCooled_1(uint256) := 0(uint256)
 fromParked = 0
fromParked_1(uint256) := 0(uint256)
 remaining = amount
remaining_1(uint256) := amount_1(uint256)
 userAssociatedValidators = $.userValidators[user]
REF_3307(mapping(address => uint16[])) -> $_1 (-> ['TMP_13304']).userValidators
REF_3308(uint16[]) -> REF_3307[user_1]
userAssociatedValidators_1(uint16[]) = ['REF_3308(uint16[])']
 remaining > 0
TMP_13316(bool) = remaining_1 > 0
CONDITION TMP_13316
 targetEntry = $.userValidatorCooldowns[user][validatorId]
REF_3309(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13304']).userValidatorCooldowns
REF_3310(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3309[user_1]
REF_3311(PlumeStakingStorage.CooldownEntry) -> REF_3310[validatorId_1]
targetEntry_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_3311(PlumeStakingStorage.CooldownEntry)
 targetEntry.amount > 0
REF_3312(uint256) -> targetEntry_1 (-> ['$']).amount
TMP_13317(bool) = REF_3312 > 0
CONDITION TMP_13317
 fromCooled += fromTarget
fromCooled_2(uint256) = fromCooled_1 (c)+ fromTarget_3
 remaining -= fromTarget
remaining_2(uint256) = remaining_1 (c)- fromTarget_3
 _removeCoolingAmounts(user,validatorId,fromTarget)
INTERNAL_CALL, StakingFacet._removeCoolingAmounts(address,uint16,uint256)(user_1,validatorId_1,fromTarget_3)
fromCooled_3(uint256) := phi(['fromCooled_2', 'fromCooled_1'])
remaining_3(uint256) := phi(['remaining_2', 'remaining_1'])
 remaining > 0
TMP_13319(bool) = remaining_3 > 0
CONDITION TMP_13319
 i = 0
i_1(uint256) := 0(uint256)
 i < userAssociatedValidators.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3313 -> LENGTH userAssociatedValidators_1
TMP_13320(bool) = i_2 < REF_3313
CONDITION TMP_13320
 remaining == 0
TMP_13321(bool) = remaining_3 == 0
CONDITION TMP_13321
 otherValidatorId = userAssociatedValidators[i]
REF_3314(uint16) -> userAssociatedValidators_1[i_2]
otherValidatorId_1(uint16) := REF_3314(uint16)
 otherValidatorId == validatorId
TMP_13322(bool) = otherValidatorId_1 == validatorId_1
CONDITION TMP_13322
 otherEntry = $.userValidatorCooldowns[user][otherValidatorId]
REF_3315(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13304']).userValidatorCooldowns
REF_3316(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3315[user_1]
REF_3317(PlumeStakingStorage.CooldownEntry) -> REF_3316[otherValidatorId_1]
otherEntry_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_3317(PlumeStakingStorage.CooldownEntry)
 otherEntry.amount > 0
REF_3318(uint256) -> otherEntry_1 (-> ['$']).amount
TMP_13323(bool) = REF_3318 > 0
CONDITION TMP_13323
 fromCooled += fromOther
fromCooled_4(uint256) = fromCooled_3 (c)+ fromOther_3
 remaining -= fromOther
remaining_6(uint256) = remaining_3 (c)- fromOther_3
 _removeCoolingAmounts(user,otherValidatorId,fromOther)
INTERNAL_CALL, StakingFacet._removeCoolingAmounts(address,uint16,uint256)(user_1,otherValidatorId_1,fromOther_3)
 $.userValidatorCooldowns[user][otherValidatorId].amount == 0
REF_3319(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13304']).userValidatorCooldowns
REF_3320(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3319[user_1]
REF_3321(PlumeStakingStorage.CooldownEntry) -> REF_3320[otherValidatorId_1]
REF_3322(uint256) -> REF_3321.amount
TMP_13325(bool) = REF_3322 == 0
CONDITION TMP_13325
 validatorsToCheck[checkCount] = otherValidatorId
REF_3323(uint16) -> validatorsToCheck_1[checkCount_1]
validatorsToCheck_2(uint16[]) := phi(['validatorsToCheck_1'])
REF_3323(uint16) (->validatorsToCheck_2) := otherValidatorId_1(uint16)
 checkCount ++
TMP_13326(uint256) := checkCount_1(uint256)
checkCount_2(uint256) = checkCount_1 (c)+ 1
validatorsToCheck_3(uint16[]) := phi(['validatorsToCheck_1', 'validatorsToCheck_2'])
checkCount_3(uint256) := phi(['checkCount_1', 'checkCount_2'])
fromCooled_5(uint256) := phi(['fromCooled_1', 'fromCooled_4'])
remaining_7(uint256) := phi(['remaining_6', 'remaining_1'])
 i ++
TMP_13327(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 remaining > 0
TMP_13328(bool) = remaining_3 > 0
CONDITION TMP_13328
 fromParked = remaining
fromParked_2(uint256) := remaining_3(uint256)
 remaining = 0
remaining_4(uint256) := 0(uint256)
 _removeParkedAmounts(user,fromParked)
INTERNAL_CALL, StakingFacet._removeParkedAmounts(address,uint256)(user_1,fromParked_2)
fromParked_3(uint256) := phi(['fromParked_1', 'fromParked_2'])
remaining_5(uint256) := phi(['remaining_4', 'remaining_1'])
 remaining != 0
TMP_13330(bool) = remaining_5 != 0
CONDITION TMP_13330
 revert InternalInconsistency(string)(Restake fund allocation failed - remaining amount not zero)
TMP_13331(None) = SOLIDITY_CALL revert InternalInconsistency(string)(Restake fund allocation failed - remaining amount not zero)
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < checkCount
i_scope_0_2(uint256) := phi(['i_scope_0_1', 'i_scope_0_3'])
TMP_13332(bool) = i_scope_0_2 < checkCount_1
CONDITION TMP_13332
 PlumeValidatorLogic.removeStakerFromValidator($,user,validatorsToCheck[i_scope_0])
REF_3325(uint16) -> validatorsToCheck_1[i_scope_0_2]
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.removeStakerFromValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13304'])", 'user_1', 'REF_3325'] 
 i_scope_0 ++
TMP_13334(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 (c)+ 1
 Staked(user,validatorId,amount,fromCooled,fromParked,0)
Emit Staked(user_1,validatorId_1,amount_1,fromCooled_3,fromParked_3,0)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
 remaining > targetEntry.amount
REF_3326(uint256) -> targetEntry_1 (-> ['$']).amount
TMP_13337(bool) = remaining_1 > REF_3326
CONDITION TMP_13337
 fromTarget = targetEntry.amount
REF_3327(uint256) -> targetEntry_1 (-> ['$']).amount
fromTarget_1(uint256) := REF_3327(uint256)
 fromTarget = remaining
fromTarget_2(uint256) := remaining_1(uint256)
fromTarget_3(uint256) := phi(['fromTarget_1', 'fromTarget_2'])
 remaining > otherEntry.amount
REF_3328(uint256) -> otherEntry_1 (-> ['$']).amount
TMP_13338(bool) = remaining_3 > REF_3328
CONDITION TMP_13338
 fromOther = otherEntry.amount
REF_3329(uint256) -> otherEntry_1 (-> ['$']).amount
fromOther_1(uint256) := REF_3329(uint256)
 fromOther = remaining
fromOther_2(uint256) := remaining_3(uint256)
fromOther_3(uint256) := phi(['fromOther_1', 'fromOther_2'])
```
#### StakingFacet.unstake(uint16,uint256) [EXTERNAL]
```slithir
 amount == 0
TMP_13343(bool) = amount_1 == 0
CONDITION TMP_13343
 revert InvalidAmount(uint256)(0)
TMP_13344(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(0)
 _unstake(validatorId,amount)
TMP_13345(uint256) = INTERNAL_CALL, StakingFacet._unstake(uint16,uint256)(validatorId_1,amount_1)
RETURN TMP_13345
 amountUnstaked
```
#### StakingFacet._unstake(uint16,uint256) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
amount_1(uint256) := phi(['amount_1', 'REF_3335'])
 $s = PlumeStakingStorage.layout()
TMP_13346(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$s_1 (-> ['TMP_13346'])(PlumeStakingStorage.Layout) := TMP_13346(PlumeStakingStorage.Layout)
 _validateValidatorForUnstaking(validatorId)
INTERNAL_CALL, StakingFacet._validateValidatorForUnstaking(uint16)(validatorId_1)
 amount == 0
TMP_13348(bool) = amount_1 == 0
CONDITION TMP_13348
 revert InvalidAmount(uint256)(amount)
TMP_13349(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(amount_1)
 $s.userValidatorStakes[msg.sender][validatorId].staked < amount
REF_3337(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $s_1 (-> ['TMP_13346']).userValidatorStakes
REF_3338(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3337[msg.sender]
REF_3339(PlumeStakingStorage.UserValidatorStake) -> REF_3338[validatorId_1]
REF_3340(uint256) -> REF_3339.staked
TMP_13350(bool) = REF_3340 < amount_1
CONDITION TMP_13350
 revert InsufficientFunds(uint256,uint256)($s.userValidatorStakes[msg.sender][validatorId].staked,amount)
REF_3341(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $s_1 (-> ['TMP_13346']).userValidatorStakes
REF_3342(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3341[msg.sender]
REF_3343(PlumeStakingStorage.UserValidatorStake) -> REF_3342[validatorId_1]
REF_3344(uint256) -> REF_3343.staked
TMP_13351(None) = SOLIDITY_CALL revert InsufficientFunds(uint256,uint256)(REF_3344,amount_1)
 PlumeRewardLogic.updateRewardsForValidator($s,msg.sender,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardsForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$s_1 (-> ['TMP_13346'])", 'msg.sender', 'validatorId_1'] 
 _updateUnstakeAmounts(msg.sender,validatorId,amount)
INTERNAL_CALL, StakingFacet._updateUnstakeAmounts(address,uint16,uint256)(msg.sender,validatorId_1,amount_1)
 newCooldownEndTimestamp = _processCooldownLogic(msg.sender,validatorId,amount)
TMP_13354(uint256) = INTERNAL_CALL, StakingFacet._processCooldownLogic(address,uint16,uint256)(msg.sender,validatorId_1,amount_1)
newCooldownEndTimestamp_1(uint256) := TMP_13354(uint256)
 _handlePostUnstakeCleanup(msg.sender,validatorId)
INTERNAL_CALL, StakingFacet._handlePostUnstakeCleanup(address,uint16)(msg.sender,validatorId_1)
 CooldownStarted(msg.sender,validatorId,amount,newCooldownEndTimestamp)
Emit CooldownStarted(msg.sender,validatorId_1,amount_1,newCooldownEndTimestamp_1)
 amount
RETURN amount_1
 amountToUnstake
```
#### StakingFacet.withdraw() [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13357(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13357'])(PlumeStakingStorage.Layout) := TMP_13357(PlumeStakingStorage.Layout)
 user = msg.sender
user_1(address) := msg.sender(address)
 _processMaturedCooldowns(user)
TMP_13358(uint256) = INTERNAL_CALL, StakingFacet._processMaturedCooldowns(address)(user_1)
 amountToWithdraw = $.stakeInfo[user].parked
REF_3347(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13357']).stakeInfo
REF_3348(PlumeStakingStorage.StakeInfo) -> REF_3347[user_1]
REF_3349(uint256) -> REF_3348.parked
amountToWithdraw_1(uint256) := REF_3349(uint256)
 amountToWithdraw == 0
TMP_13359(bool) = amountToWithdraw_1 == 0
CONDITION TMP_13359
 revert InvalidAmount(uint256)(0)
TMP_13360(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(0)
 _removeParkedAmounts(user,amountToWithdraw)
INTERNAL_CALL, StakingFacet._removeParkedAmounts(address,uint256)(user_1,amountToWithdraw_1)
 _cleanupValidatorRelationships(user)
INTERNAL_CALL, StakingFacet._cleanupValidatorRelationships(address)(user_1)
 Withdrawn(user,amountToWithdraw)
Emit Withdrawn(user_1,amountToWithdraw_1)
 (success,None) = user.call{value: amountToWithdraw}()
TUPLE_90(bool,bytes) = LOW_LEVEL_CALL, dest:user_1, function:call, arguments:[''] value:amountToWithdraw_1 
success_1(bool)= UNPACK TUPLE_90 index: 0 
 ! success
TMP_13364 = UnaryType.BANG success_1 
CONDITION TMP_13364
 revert NativeTransferFailed()()
TMP_13365(None) = SOLIDITY_CALL revert NativeTransferFailed()()
```
#### StakingFacet.stakeOnBehalf(uint16,address) [EXTERNAL]
```slithir
 staker == address(0)
TMP_13366 = CONVERT 0 to address
TMP_13367(bool) = staker_1 == TMP_13366
CONDITION TMP_13367
 revert ZeroRecipientAddress()()
TMP_13368(None) = SOLIDITY_CALL revert ZeroRecipientAddress()()
 stakeAmount = msg.value
stakeAmount_1(uint256) := msg.value(uint256)
 isNewStake = _performStakeSetup(staker,validatorId,stakeAmount)
TMP_13369(bool) = INTERNAL_CALL, StakingFacet._performStakeSetup(address,uint16,uint256)(staker_1,validatorId_1,stakeAmount_1)
isNewStake_1(bool) := TMP_13369(bool)
 Staked(staker,validatorId,stakeAmount,0,0,stakeAmount)
Emit Staked(staker_1,validatorId_1,stakeAmount_1,0,0,stakeAmount_1)
 StakedOnBehalf(msg.sender,staker,validatorId,stakeAmount)
Emit StakedOnBehalf(msg.sender,staker_1,validatorId_1,stakeAmount_1)
 stakeAmount
RETURN stakeAmount_1
```
#### StakingFacet.restakeRewards(uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13372(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13372'])(PlumeStakingStorage.Layout) := TMP_13372(PlumeStakingStorage.Layout)
 user = msg.sender
user_1(address) := msg.sender(address)
 _processMaturedCooldowns(user)
TMP_13373(uint256) = INTERNAL_CALL, StakingFacet._processMaturedCooldowns(address)(user_1)
 _validateValidatorForStaking(validatorId)
INTERNAL_CALL, StakingFacet._validateValidatorForStaking(uint16)(validatorId_1)
 tokenToRestake = PlumeStakingStorage.PLUME_NATIVE
REF_3352(address) -> PlumeStakingStorage.PLUME_NATIVE
tokenToRestake_1(address) := REF_3352(address)
 ! $.isRewardToken[tokenToRestake]
REF_3353(mapping(address => bool)) -> $_1 (-> ['TMP_13372']).isRewardToken
REF_3354(bool) -> REF_3353[tokenToRestake_1]
TMP_13375 = UnaryType.BANG REF_3354 
CONDITION TMP_13375
 revert TokenDoesNotExist(address)(tokenToRestake)
TMP_13376(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(tokenToRestake_1)
 amountRestaked = _calculateAndClaimAllRewards(user,tokenToRestake)
TMP_13377(uint256) = INTERNAL_CALL, StakingFacet._calculateAndClaimAllRewards(address,address)(user_1,tokenToRestake_1)
amountRestaked_1(uint256) := TMP_13377(uint256)
 amountRestaked == 0
TMP_13378(bool) = amountRestaked_1 == 0
CONDITION TMP_13378
 revert NoRewardsToRestake()()
TMP_13379(None) = SOLIDITY_CALL revert NoRewardsToRestake()()
 amountRestaked < $.minStakeAmount
REF_3355(uint256) -> $_1 (-> ['TMP_13372']).minStakeAmount
TMP_13380(bool) = amountRestaked_1 < REF_3355
CONDITION TMP_13380
 revert StakeAmountTooSmall(uint256,uint256)(amountRestaked,$.minStakeAmount)
REF_3356(uint256) -> $_1 (-> ['TMP_13372']).minStakeAmount
TMP_13381(None) = SOLIDITY_CALL revert StakeAmountTooSmall(uint256,uint256)(amountRestaked_1,REF_3356)
 _performRestakeWorkflow(user,validatorId,amountRestaked,rewards)
INTERNAL_CALL, StakingFacet._performRestakeWorkflow(address,uint16,uint256,string)(user_1,validatorId_1,amountRestaked_1,rewards)
 _validateCapacityLimits(validatorId,amountRestaked)
INTERNAL_CALL, StakingFacet._validateCapacityLimits(uint16,uint256)(validatorId_1,amountRestaked_1)
 Staked(user,validatorId,amountRestaked,0,0,amountRestaked)
Emit Staked(user_1,validatorId_1,amountRestaked_1,0,0,amountRestaked_1)
 RewardsRestaked(user,validatorId,amountRestaked)
Emit RewardsRestaked(user_1,validatorId_1,amountRestaked_1)
 amountRestaked
RETURN amountRestaked_1
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
 amountRestaked
```
#### StakingFacet.amountStaked() [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().stakeInfo[msg.sender].staked
TMP_13387(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_3358(mapping(address => PlumeStakingStorage.StakeInfo)) -> TMP_13387.stakeInfo
REF_3359(PlumeStakingStorage.StakeInfo) -> REF_3358[msg.sender]
REF_3360(uint256) -> REF_3359.staked
RETURN REF_3360
 amount
```
#### StakingFacet.amountCooling() [EXTERNAL]
```slithir
 _calculateActivelyCoolingAmount(msg.sender)
TMP_13388(uint256) = INTERNAL_CALL, StakingFacet._calculateActivelyCoolingAmount(address)(msg.sender)
RETURN TMP_13388
 activelyCoolingAmount
```
#### StakingFacet.amountWithdrawable() [EXTERNAL]
```slithir
 _calculateTotalWithdrawableAmount(msg.sender)
TMP_13389(uint256) = INTERNAL_CALL, StakingFacet._calculateTotalWithdrawableAmount(address)(msg.sender)
RETURN TMP_13389
 totalWithdrawableAmount
```
#### StakingFacet.stakeInfo(address) [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().stakeInfo[user]
TMP_13390(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_3362(mapping(address => PlumeStakingStorage.StakeInfo)) -> TMP_13390.stakeInfo
REF_3363(PlumeStakingStorage.StakeInfo) -> REF_3362[user_1]
RETURN REF_3363
```
#### StakingFacet.totalAmountStaked() [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().totalStaked
TMP_13391(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_3365(uint256) -> TMP_13391.totalStaked
RETURN REF_3365
 amount
```
#### StakingFacet.totalAmountCooling() [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().totalCooling
TMP_13392(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_3367(uint256) -> TMP_13392.totalCooling
RETURN REF_3367
 amount
```
#### StakingFacet.totalAmountWithdrawable() [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().totalWithdrawable
TMP_13393(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_3369(uint256) -> TMP_13393.totalWithdrawable
RETURN REF_3369
 amount
```
#### StakingFacet.totalAmountClaimable(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13394(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13394'])(PlumeStakingStorage.Layout) := TMP_13394(PlumeStakingStorage.Layout)
 require(bool,string)($.isRewardToken[token],Token is not a reward token)
REF_3371(mapping(address => bool)) -> $_1 (-> ['TMP_13394']).isRewardToken
REF_3372(bool) -> REF_3371[token_1]
TMP_13395(None) = SOLIDITY_CALL require(bool,string)(REF_3372,Token is not a reward token)
 $.totalClaimableByToken[token]
REF_3373(mapping(address => uint256)) -> $_1 (-> ['TMP_13394']).totalClaimableByToken
REF_3374(uint256) -> REF_3373[token_1]
RETURN REF_3374
 amount
```
#### StakingFacet.getUserValidatorStake(address,uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13396(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13396'])(PlumeStakingStorage.Layout) := TMP_13396(PlumeStakingStorage.Layout)
 $.userValidatorStakes[user][validatorId].staked
REF_3376(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13396']).userValidatorStakes
REF_3377(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3376[user_1]
REF_3378(PlumeStakingStorage.UserValidatorStake) -> REF_3377[validatorId_1]
REF_3379(uint256) -> REF_3378.staked
RETURN REF_3379
```
#### StakingFacet.getUserCooldowns(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13397(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13397'])(PlumeStakingStorage.Layout) := TMP_13397(PlumeStakingStorage.Layout)
 userAssociatedValidators = $.userValidators[user]
REF_3381(mapping(address => uint16[])) -> $_1 (-> ['TMP_13397']).userValidators
REF_3382(uint16[]) -> REF_3381[user_1]
userAssociatedValidators_1 (-> [])(uint16[]) = ['REF_3382(uint16[])']
 activeCooldownCount = _countActiveCooldowns(user)
TMP_13398(uint256) = INTERNAL_CALL, StakingFacet._countActiveCooldowns(address)(user_1)
activeCooldownCount_1(uint256) := TMP_13398(uint256)
 cooldowns = new StakingFacet.CooldownView[](activeCooldownCount)
TMP_13400(StakingFacet.CooldownView[])  = new StakingFacet.CooldownView[](activeCooldownCount_1)
cooldowns_1(StakingFacet.CooldownView[]) = ['TMP_13400(StakingFacet.CooldownView[])']
 currentIndex = 0
currentIndex_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < userAssociatedValidators.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3383 -> LENGTH userAssociatedValidators_1 (-> [])
TMP_13401(bool) = i_2 < REF_3383
CONDITION TMP_13401
 validatorId_iterator = userAssociatedValidators[i]
REF_3384(uint16) -> userAssociatedValidators_1 (-> [])[i_2]
validatorId_iterator_1(uint16) := REF_3384(uint16)
 $.validatorExists[validatorId_iterator] && ! $.validators[validatorId_iterator].slashed
REF_3385(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13397']).validatorExists
REF_3386(bool) -> REF_3385[validatorId_iterator_1]
REF_3387(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13397']).validators
REF_3388(PlumeStakingStorage.ValidatorInfo) -> REF_3387[validatorId_iterator_1]
REF_3389(bool) -> REF_3388.slashed
TMP_13402 = UnaryType.BANG REF_3389 
TMP_13403(bool) = REF_3386 && TMP_13402
CONDITION TMP_13403
 entry = $.userValidatorCooldowns[user][validatorId_iterator]
REF_3390(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13397']).userValidatorCooldowns
REF_3391(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3390[user_1]
REF_3392(PlumeStakingStorage.CooldownEntry) -> REF_3391[validatorId_iterator_1]
entry_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_3392(PlumeStakingStorage.CooldownEntry)
 entry.amount > 0
REF_3393(uint256) -> entry_1 (-> ['$']).amount
TMP_13404(bool) = REF_3393 > 0
CONDITION TMP_13404
 cooldowns[currentIndex] = CooldownView({validatorId:validatorId_iterator,amount:entry.amount,cooldownEndTime:entry.cooldownEndTime})
REF_3394(StakingFacet.CooldownView) -> cooldowns_1[currentIndex_1]
REF_3395(uint256) -> entry_1 (-> ['$']).amount
REF_3396(uint256) -> entry_1 (-> ['$']).cooldownEndTime
TMP_13405(StakingFacet.CooldownView) = new CooldownView(validatorId_iterator_1,REF_3395,REF_3396)
cooldowns_2(StakingFacet.CooldownView[]) := phi(['cooldowns_1'])
REF_3394(StakingFacet.CooldownView) (->cooldowns_2) := TMP_13405(StakingFacet.CooldownView)
 currentIndex ++
TMP_13406(uint256) := currentIndex_1(uint256)
currentIndex_2(uint256) = currentIndex_1 (c)+ 1
cooldowns_3(StakingFacet.CooldownView[]) := phi(['cooldowns_1', 'cooldowns_2'])
currentIndex_3(uint256) := phi(['currentIndex_2', 'currentIndex_1'])
 i ++
TMP_13407(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 cooldowns
RETURN cooldowns_1
```
#### StakingFacet._updateStakeAmounts(address,uint16,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'user_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
amount_1(uint256) := phi(['amount_1', 'stakeAmount_1'])
 $ = PlumeStakingStorage.layout()
TMP_13408(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13408'])(PlumeStakingStorage.Layout) := TMP_13408(PlumeStakingStorage.Layout)
 $.userValidatorStakes[user][validatorId].staked += amount
REF_3398(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13408']).userValidatorStakes
REF_3399(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3398[user_1]
REF_3400(PlumeStakingStorage.UserValidatorStake) -> REF_3399[validatorId_1]
REF_3401(uint256) -> REF_3400.staked
$_2 (-> ['TMP_13408'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13408'])"])
REF_3401(-> $_2 (-> ['TMP_13408'])) = REF_3401 (c)+ amount_1
TMP_13408(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13408'])"])
 $.stakeInfo[user].staked += amount
REF_3402(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_2 (-> ['TMP_13408']).stakeInfo
REF_3403(PlumeStakingStorage.StakeInfo) -> REF_3402[user_1]
REF_3404(uint256) -> REF_3403.staked
$_3 (-> ['TMP_13408'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13408'])"])
REF_3404(-> $_3 (-> ['TMP_13408'])) = REF_3404 (c)+ amount_1
TMP_13408(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13408'])"])
 $.validators[validatorId].delegatedAmount += amount
REF_3405(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_3 (-> ['TMP_13408']).validators
REF_3406(PlumeStakingStorage.ValidatorInfo) -> REF_3405[validatorId_1]
REF_3407(uint256) -> REF_3406.delegatedAmount
$_4 (-> ['TMP_13408'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13408'])"])
REF_3407(-> $_4 (-> ['TMP_13408'])) = REF_3407 (c)+ amount_1
TMP_13408(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13408'])"])
 $.validatorTotalStaked[validatorId] += amount
REF_3408(mapping(uint16 => uint256)) -> $_4 (-> ['TMP_13408']).validatorTotalStaked
REF_3409(uint256) -> REF_3408[validatorId_1]
$_5 (-> ['TMP_13408'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13408'])"])
REF_3409(-> $_5 (-> ['TMP_13408'])) = REF_3409 (c)+ amount_1
TMP_13408(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13408'])"])
 $.totalStaked += amount
REF_3410(uint256) -> $_5 (-> ['TMP_13408']).totalStaked
$_6 (-> ['TMP_13408'])(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13408'])"])
REF_3410(-> $_6 (-> ['TMP_13408'])) = REF_3410 (c)+ amount_1
TMP_13408(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13408'])"])
```
#### StakingFacet._updateUnstakeAmounts(address,uint16,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender'])
validatorId_1(uint16) := phi(['validatorId_1'])
amount_1(uint256) := phi(['amount_1'])
 $ = PlumeStakingStorage.layout()
TMP_13409(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13409'])(PlumeStakingStorage.Layout) := TMP_13409(PlumeStakingStorage.Layout)
 $.userValidatorStakes[user][validatorId].staked -= amount
REF_3412(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13409']).userValidatorStakes
REF_3413(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3412[user_1]
REF_3414(PlumeStakingStorage.UserValidatorStake) -> REF_3413[validatorId_1]
REF_3415(uint256) -> REF_3414.staked
$_2 (-> ['TMP_13409'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13409'])"])
REF_3415(-> $_2 (-> ['TMP_13409'])) = REF_3415 (c)- amount_1
TMP_13409(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13409'])"])
 $.stakeInfo[user].staked -= amount
REF_3416(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_2 (-> ['TMP_13409']).stakeInfo
REF_3417(PlumeStakingStorage.StakeInfo) -> REF_3416[user_1]
REF_3418(uint256) -> REF_3417.staked
$_3 (-> ['TMP_13409'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13409'])"])
REF_3418(-> $_3 (-> ['TMP_13409'])) = REF_3418 (c)- amount_1
TMP_13409(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13409'])"])
 $.validators[validatorId].delegatedAmount -= amount
REF_3419(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_3 (-> ['TMP_13409']).validators
REF_3420(PlumeStakingStorage.ValidatorInfo) -> REF_3419[validatorId_1]
REF_3421(uint256) -> REF_3420.delegatedAmount
$_4 (-> ['TMP_13409'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13409'])"])
REF_3421(-> $_4 (-> ['TMP_13409'])) = REF_3421 (c)- amount_1
TMP_13409(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13409'])"])
 $.validatorTotalStaked[validatorId] -= amount
REF_3422(mapping(uint16 => uint256)) -> $_4 (-> ['TMP_13409']).validatorTotalStaked
REF_3423(uint256) -> REF_3422[validatorId_1]
$_5 (-> ['TMP_13409'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13409'])"])
REF_3423(-> $_5 (-> ['TMP_13409'])) = REF_3423 (c)- amount_1
TMP_13409(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13409'])"])
 $.totalStaked -= amount
REF_3424(uint256) -> $_5 (-> ['TMP_13409']).totalStaked
$_6 (-> ['TMP_13409'])(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13409'])"])
REF_3424(-> $_6 (-> ['TMP_13409'])) = REF_3424 (c)- amount_1
TMP_13409(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13409'])"])
```
#### StakingFacet._updateCoolingAmounts(address,uint16,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
amount_1(uint256) := phi(['amount_1'])
 $ = PlumeStakingStorage.layout()
TMP_13410(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13410'])(PlumeStakingStorage.Layout) := TMP_13410(PlumeStakingStorage.Layout)
 $.stakeInfo[user].cooled += amount
REF_3426(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13410']).stakeInfo
REF_3427(PlumeStakingStorage.StakeInfo) -> REF_3426[user_1]
REF_3428(uint256) -> REF_3427.cooled
$_2 (-> ['TMP_13410'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13410'])"])
REF_3428(-> $_2 (-> ['TMP_13410'])) = REF_3428 (c)+ amount_1
TMP_13410(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13410'])"])
 $.totalCooling += amount
REF_3429(uint256) -> $_2 (-> ['TMP_13410']).totalCooling
$_3 (-> ['TMP_13410'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13410'])"])
REF_3429(-> $_3 (-> ['TMP_13410'])) = REF_3429 (c)+ amount_1
TMP_13410(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13410'])"])
 $.validatorTotalCooling[validatorId] += amount
REF_3430(mapping(uint16 => uint256)) -> $_3 (-> ['TMP_13410']).validatorTotalCooling
REF_3431(uint256) -> REF_3430[validatorId_1]
$_4 (-> ['TMP_13410'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13410'])"])
REF_3431(-> $_4 (-> ['TMP_13410'])) = REF_3431 (c)+ amount_1
TMP_13410(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13410'])"])
```
#### StakingFacet._removeCoolingAmounts(address,uint16,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'user_1', 'user_1'])
validatorId_1(uint16) := phi(['otherValidatorId_1', 'validatorId_1', 'validatorId_1', 'validatorId_1'])
amount_1(uint256) := phi(['amountInThisCooldown_1', 'fromTarget_3', 'currentCooledAmountInSlot_1', 'fromOther_3'])
 $ = PlumeStakingStorage.layout()
TMP_13411(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13411'])(PlumeStakingStorage.Layout) := TMP_13411(PlumeStakingStorage.Layout)
 $.stakeInfo[user].cooled >= amount
REF_3433(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13411']).stakeInfo
REF_3434(PlumeStakingStorage.StakeInfo) -> REF_3433[user_1]
REF_3435(uint256) -> REF_3434.cooled
TMP_13412(bool) = REF_3435 >= amount_1
CONDITION TMP_13412
 $.stakeInfo[user].cooled -= amount
REF_3436(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13411']).stakeInfo
REF_3437(PlumeStakingStorage.StakeInfo) -> REF_3436[user_1]
REF_3438(uint256) -> REF_3437.cooled
$_2 (-> ['TMP_13411'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13411'])"])
REF_3438(-> $_2 (-> ['TMP_13411'])) = REF_3438 (c)- amount_1
TMP_13411(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13411'])"])
 $.stakeInfo[user].cooled = 0
REF_3439(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13411']).stakeInfo
REF_3440(PlumeStakingStorage.StakeInfo) -> REF_3439[user_1]
REF_3441(uint256) -> REF_3440.cooled
$_3 (-> ['TMP_13411'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13411'])"])
REF_3441(uint256) (->$_3 (-> ['TMP_13411'])) := 0(uint256)
TMP_13411(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13411'])"])
$_4 (-> ['TMP_13411'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13411'])", "$_3 (-> ['TMP_13411'])"])
 $.totalCooling >= amount
REF_3442(uint256) -> $_4 (-> ['TMP_13411']).totalCooling
TMP_13413(bool) = REF_3442 >= amount_1
CONDITION TMP_13413
 $.totalCooling -= amount
REF_3443(uint256) -> $_4 (-> ['TMP_13411']).totalCooling
$_6 (-> ['TMP_13411'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13411'])"])
REF_3443(-> $_6 (-> ['TMP_13411'])) = REF_3443 (c)- amount_1
TMP_13411(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13411'])"])
 $.totalCooling = 0
REF_3444(uint256) -> $_4 (-> ['TMP_13411']).totalCooling
$_5 (-> ['TMP_13411'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13411'])"])
REF_3444(uint256) (->$_5 (-> ['TMP_13411'])) := 0(uint256)
TMP_13411(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13411'])"])
$_7 (-> ['TMP_13411'])(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13411'])", "$_6 (-> ['TMP_13411'])"])
 entry = $.userValidatorCooldowns[user][validatorId]
REF_3445(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_7 (-> ['TMP_13411']).userValidatorCooldowns
REF_3446(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3445[user_1]
REF_3447(PlumeStakingStorage.CooldownEntry) -> REF_3446[validatorId_1]
entry_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_3447(PlumeStakingStorage.CooldownEntry)
 entry.amount >= amount
REF_3448(uint256) -> entry_1 (-> ['$']).amount
TMP_13414(bool) = REF_3448 >= amount_1
CONDITION TMP_13414
 entry.amount -= amount
REF_3449(uint256) -> entry_1 (-> ['$']).amount
entry_2 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := phi(["entry_1 (-> ['$'])"])
REF_3449(-> entry_2 (-> ['$'])) = REF_3449 (c)- amount_1
$_10 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["entry_2 (-> ['$'])"])
 entry.amount == 0
REF_3450(uint256) -> entry_2 (-> ['$']).amount
TMP_13415(bool) = REF_3450 == 0
CONDITION TMP_13415
 entry.cooldownEndTime = 0
REF_3451(uint256) -> entry_2 (-> ['$']).cooldownEndTime
entry_3 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := phi(["entry_2 (-> ['$'])"])
REF_3451(uint256) (->entry_3 (-> ['$'])) := 0(uint256)
$_11 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["entry_3 (-> ['$'])"])
 entry.amount = 0
REF_3452(uint256) -> entry_1 (-> ['$']).amount
entry_4 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := phi(["entry_1 (-> ['$'])"])
REF_3452(uint256) (->entry_4 (-> ['$'])) := 0(uint256)
$_12 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["entry_4 (-> ['$'])"])
 entry.cooldownEndTime = 0
REF_3453(uint256) -> entry_4 (-> ['$']).cooldownEndTime
entry_5 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := phi(["entry_4 (-> ['$'])"])
REF_3453(uint256) (->entry_5 (-> ['$'])) := 0(uint256)
$_13 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["entry_5 (-> ['$'])"])
 $.validatorTotalCooling[validatorId] >= amount
REF_3454(mapping(uint16 => uint256)) -> $_7 (-> ['TMP_13411']).validatorTotalCooling
REF_3455(uint256) -> REF_3454[validatorId_1]
TMP_13416(bool) = REF_3455 >= amount_1
CONDITION TMP_13416
 $.validatorTotalCooling[validatorId] -= amount
REF_3456(mapping(uint16 => uint256)) -> $_7 (-> ['TMP_13411']).validatorTotalCooling
REF_3457(uint256) -> REF_3456[validatorId_1]
$_9 (-> ['TMP_13411'])(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_13411'])"])
REF_3457(-> $_9 (-> ['TMP_13411'])) = REF_3457 (c)- amount_1
TMP_13411(PlumeStakingStorage.Layout) := phi(["$_9 (-> ['TMP_13411'])"])
 $.validatorTotalCooling[validatorId] = 0
REF_3458(mapping(uint16 => uint256)) -> $_7 (-> ['TMP_13411']).validatorTotalCooling
REF_3459(uint256) -> REF_3458[validatorId_1]
$_8 (-> ['TMP_13411'])(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_13411'])"])
REF_3459(uint256) (->$_8 (-> ['TMP_13411'])) := 0(uint256)
TMP_13411(PlumeStakingStorage.Layout) := phi(["$_8 (-> ['TMP_13411'])"])
```
#### StakingFacet._updateParkedAmounts(address,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'user_1'])
amount_1(uint256) := phi(['currentCooledAmountInSlot_1', 'amountMovedToParked_1'])
 $ = PlumeStakingStorage.layout()
TMP_13417(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13417'])(PlumeStakingStorage.Layout) := TMP_13417(PlumeStakingStorage.Layout)
 $.stakeInfo[user].parked += amount
REF_3461(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13417']).stakeInfo
REF_3462(PlumeStakingStorage.StakeInfo) -> REF_3461[user_1]
REF_3463(uint256) -> REF_3462.parked
$_2 (-> ['TMP_13417'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13417'])"])
REF_3463(-> $_2 (-> ['TMP_13417'])) = REF_3463 (c)+ amount_1
TMP_13417(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13417'])"])
 $.totalWithdrawable += amount
REF_3464(uint256) -> $_2 (-> ['TMP_13417']).totalWithdrawable
$_3 (-> ['TMP_13417'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13417'])"])
REF_3464(-> $_3 (-> ['TMP_13417'])) = REF_3464 (c)+ amount_1
TMP_13417(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13417'])"])
```
#### StakingFacet._updateWithdrawalAmounts(address,uint256) [INTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13418(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13418'])(PlumeStakingStorage.Layout) := TMP_13418(PlumeStakingStorage.Layout)
 $.stakeInfo[user].parked = 0
REF_3466(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13418']).stakeInfo
REF_3467(PlumeStakingStorage.StakeInfo) -> REF_3466[user_1]
REF_3468(uint256) -> REF_3467.parked
$_2 (-> ['TMP_13418'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13418'])"])
REF_3468(uint256) (->$_2 (-> ['TMP_13418'])) := 0(uint256)
TMP_13418(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13418'])"])
 $.totalWithdrawable >= amount
REF_3469(uint256) -> $_2 (-> ['TMP_13418']).totalWithdrawable
TMP_13419(bool) = REF_3469 >= amount_1
CONDITION TMP_13419
 $.totalWithdrawable -= amount
REF_3470(uint256) -> $_2 (-> ['TMP_13418']).totalWithdrawable
$_3 (-> ['TMP_13418'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13418'])"])
REF_3470(-> $_3 (-> ['TMP_13418'])) = REF_3470 (c)- amount_1
TMP_13418(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13418'])"])
 $.totalWithdrawable = 0
REF_3471(uint256) -> $_2 (-> ['TMP_13418']).totalWithdrawable
$_4 (-> ['TMP_13418'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13418'])"])
REF_3471(uint256) (->$_4 (-> ['TMP_13418'])) := 0(uint256)
TMP_13418(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13418'])"])
```
#### StakingFacet._updateRewardClaim(address,uint16,address,uint256) [INTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13420(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13420'])(PlumeStakingStorage.Layout) := TMP_13420(PlumeStakingStorage.Layout)
 $.userRewards[user][validatorId][token] = 0
REF_3473(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13420']).userRewards
REF_3474(mapping(uint16 => mapping(address => uint256))) -> REF_3473[user_1]
REF_3475(mapping(address => uint256)) -> REF_3474[validatorId_1]
REF_3476(uint256) -> REF_3475[token_1]
$_2 (-> ['TMP_13420'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13420'])"])
REF_3476(uint256) (->$_2 (-> ['TMP_13420'])) := 0(uint256)
TMP_13420(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13420'])"])
 $.userValidatorRewardPerTokenPaidTimestamp[user][validatorId][token] = block.timestamp
REF_3477(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_2 (-> ['TMP_13420']).userValidatorRewardPerTokenPaidTimestamp
REF_3478(mapping(uint16 => mapping(address => uint256))) -> REF_3477[user_1]
REF_3479(mapping(address => uint256)) -> REF_3478[validatorId_1]
REF_3480(uint256) -> REF_3479[token_1]
$_3 (-> ['TMP_13420'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13420'])"])
REF_3480(uint256) (->$_3 (-> ['TMP_13420'])) := block.timestamp(uint256)
TMP_13420(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13420'])"])
 $.userValidatorRewardPerTokenPaid[user][validatorId][token] = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_3481(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_3 (-> ['TMP_13420']).userValidatorRewardPerTokenPaid
REF_3482(mapping(uint16 => mapping(address => uint256))) -> REF_3481[user_1]
REF_3483(mapping(address => uint256)) -> REF_3482[validatorId_1]
REF_3484(uint256) -> REF_3483[token_1]
REF_3485(mapping(uint16 => mapping(address => uint256))) -> $_3 (-> ['TMP_13420']).validatorRewardPerTokenCumulative
REF_3486(mapping(address => uint256)) -> REF_3485[validatorId_1]
REF_3487(uint256) -> REF_3486[token_1]
$_4 (-> ['TMP_13420'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13420'])"])
REF_3484(uint256) (->$_4 (-> ['TMP_13420'])) := REF_3487(uint256)
TMP_13420(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13420'])"])
 $.totalClaimableByToken[token] >= amount
REF_3488(mapping(address => uint256)) -> $_4 (-> ['TMP_13420']).totalClaimableByToken
REF_3489(uint256) -> REF_3488[token_1]
TMP_13421(bool) = REF_3489 >= amount_1
CONDITION TMP_13421
 $.totalClaimableByToken[token] -= amount
REF_3490(mapping(address => uint256)) -> $_4 (-> ['TMP_13420']).totalClaimableByToken
REF_3491(uint256) -> REF_3490[token_1]
$_5 (-> ['TMP_13420'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13420'])"])
REF_3491(-> $_5 (-> ['TMP_13420'])) = REF_3491 (c)- amount_1
TMP_13420(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13420'])"])
 $.totalClaimableByToken[token] = 0
REF_3492(mapping(address => uint256)) -> $_4 (-> ['TMP_13420']).totalClaimableByToken
REF_3493(uint256) -> REF_3492[token_1]
$_6 (-> ['TMP_13420'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13420'])"])
REF_3493(uint256) (->$_6 (-> ['TMP_13420'])) := 0(uint256)
TMP_13420(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13420'])"])
```
#### StakingFacet._updateCommissionClaim(uint16,address,uint256,address) [INTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13422(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13422'])(PlumeStakingStorage.Layout) := TMP_13422(PlumeStakingStorage.Layout)
 $.pendingCommissionClaims[validatorId][token] = PlumeStakingStorage.PendingCommissionClaim({amount:amount,requestTimestamp:block.timestamp,token:token,recipient:recipient})
REF_3495(mapping(uint16 => mapping(address => PlumeStakingStorage.PendingCommissionClaim))) -> $_1 (-> ['TMP_13422']).pendingCommissionClaims
REF_3496(mapping(address => PlumeStakingStorage.PendingCommissionClaim)) -> REF_3495[validatorId_1]
REF_3497(PlumeStakingStorage.PendingCommissionClaim) -> REF_3496[token_1]
TMP_13423(PlumeStakingStorage.PendingCommissionClaim) = new PendingCommissionClaim(amount_1,block.timestamp,token_1,recipient_1)
$_2 (-> ['TMP_13422'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13422'])"])
REF_3497(PlumeStakingStorage.PendingCommissionClaim) (->$_2 (-> ['TMP_13422'])) := TMP_13423(PlumeStakingStorage.PendingCommissionClaim)
TMP_13422(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13422'])"])
 $.validatorAccruedCommission[validatorId][token] = 0
REF_3499(mapping(uint16 => mapping(address => uint256))) -> $_2 (-> ['TMP_13422']).validatorAccruedCommission
REF_3500(mapping(address => uint256)) -> REF_3499[validatorId_1]
REF_3501(uint256) -> REF_3500[token_1]
$_3 (-> ['TMP_13422'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13422'])"])
REF_3501(uint256) (->$_3 (-> ['TMP_13422'])) := 0(uint256)
TMP_13422(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13422'])"])
```
#### StakingFacet._removeParkedAmounts(address,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'user_1'])
amount_1(uint256) := phi(['amountToWithdraw_1', 'fromParked_2'])
 $ = PlumeStakingStorage.layout()
TMP_13424(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13424'])(PlumeStakingStorage.Layout) := TMP_13424(PlumeStakingStorage.Layout)
 $.stakeInfo[user].parked -= amount
REF_3503(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13424']).stakeInfo
REF_3504(PlumeStakingStorage.StakeInfo) -> REF_3503[user_1]
REF_3505(uint256) -> REF_3504.parked
$_2 (-> ['TMP_13424'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13424'])"])
REF_3505(-> $_2 (-> ['TMP_13424'])) = REF_3505 (c)- amount_1
TMP_13424(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13424'])"])
 $.totalWithdrawable -= amount
REF_3506(uint256) -> $_2 (-> ['TMP_13424']).totalWithdrawable
$_3 (-> ['TMP_13424'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13424'])"])
REF_3506(-> $_3 (-> ['TMP_13424'])) = REF_3506 (c)- amount_1
TMP_13424(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13424'])"])
```
#### StakingFacet._processCooldownLogic(address,uint16,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender'])
validatorId_1(uint16) := phi(['validatorId_1'])
amount_1(uint256) := phi(['amount_1'])
 $ = PlumeStakingStorage.layout()
TMP_13425(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13425'])(PlumeStakingStorage.Layout) := TMP_13425(PlumeStakingStorage.Layout)
 cooldownEntrySlot = $.userValidatorCooldowns[user][validatorId]
REF_3508(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13425']).userValidatorCooldowns
REF_3509(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3508[user_1]
REF_3510(PlumeStakingStorage.CooldownEntry) -> REF_3509[validatorId_1]
cooldownEntrySlot_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_3510(PlumeStakingStorage.CooldownEntry)
 currentCooledAmountInSlot = cooldownEntrySlot.amount
REF_3511(uint256) -> cooldownEntrySlot_1 (-> ['$']).amount
currentCooledAmountInSlot_1(uint256) := REF_3511(uint256)
 currentCooldownEndTimeInSlot = cooldownEntrySlot.cooldownEndTime
REF_3512(uint256) -> cooldownEntrySlot_1 (-> ['$']).cooldownEndTime
currentCooldownEndTimeInSlot_1(uint256) := REF_3512(uint256)
 newCooldownEndTime = block.timestamp + $.cooldownInterval
REF_3513(uint256) -> $_1 (-> ['TMP_13425']).cooldownInterval
TMP_13426(uint256) = block.timestamp (c)+ REF_3513
newCooldownEndTime_1(uint256) := TMP_13426(uint256)
 currentCooledAmountInSlot > 0 && block.timestamp >= currentCooldownEndTimeInSlot
TMP_13427(bool) = currentCooledAmountInSlot_1 > 0
TMP_13428(bool) = block.timestamp >= currentCooldownEndTimeInSlot_1
TMP_13429(bool) = TMP_13427 && TMP_13428
CONDITION TMP_13429
 _updateParkedAmounts(user,currentCooledAmountInSlot)
INTERNAL_CALL, StakingFacet._updateParkedAmounts(address,uint256)(user_1,currentCooledAmountInSlot_1)
 _removeCoolingAmounts(user,validatorId,currentCooledAmountInSlot)
INTERNAL_CALL, StakingFacet._removeCoolingAmounts(address,uint16,uint256)(user_1,validatorId_1,currentCooledAmountInSlot_1)
 _updateCoolingAmounts(user,validatorId,amount)
INTERNAL_CALL, StakingFacet._updateCoolingAmounts(address,uint16,uint256)(user_1,validatorId_1,amount_1)
 finalNewCooledAmountForSlot = amount
finalNewCooledAmountForSlot_1(uint256) := amount_1(uint256)
 _updateCoolingAmounts(user,validatorId,amount)
INTERNAL_CALL, StakingFacet._updateCoolingAmounts(address,uint16,uint256)(user_1,validatorId_1,amount_1)
 finalNewCooledAmountForSlot = currentCooledAmountInSlot + amount
TMP_13434(uint256) = currentCooledAmountInSlot_1 (c)+ amount_1
finalNewCooledAmountForSlot_2(uint256) := TMP_13434(uint256)
finalNewCooledAmountForSlot_3(uint256) := phi(['finalNewCooledAmountForSlot_2', 'finalNewCooledAmountForSlot_1'])
 cooldownEntrySlot.amount = finalNewCooledAmountForSlot
REF_3514(uint256) -> cooldownEntrySlot_1 (-> ['$']).amount
cooldownEntrySlot_2 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := phi(["cooldownEntrySlot_1 (-> ['$'])"])
REF_3514(uint256) (->cooldownEntrySlot_2 (-> ['$'])) := finalNewCooledAmountForSlot_3(uint256)
$_2 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["cooldownEntrySlot_2 (-> ['$'])"])
 cooldownEntrySlot.cooldownEndTime = newCooldownEndTime
REF_3515(uint256) -> cooldownEntrySlot_2 (-> ['$']).cooldownEndTime
cooldownEntrySlot_3 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := phi(["cooldownEntrySlot_2 (-> ['$'])"])
REF_3515(uint256) (->cooldownEntrySlot_3 (-> ['$'])) := newCooldownEndTime_1(uint256)
$_3 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["cooldownEntrySlot_3 (-> ['$'])"])
 newCooldownEndTime
RETURN newCooldownEndTime_1
 newCooldownEndTime
```
#### StakingFacet._processMaturedCooldowns(address) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'user_1', 'user_1'])
 $ = PlumeStakingStorage.layout()
TMP_13435(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13435'])(PlumeStakingStorage.Layout) := TMP_13435(PlumeStakingStorage.Layout)
 amountMovedToParked = 0
amountMovedToParked_1(uint256) := 0(uint256)
 userAssociatedValidators = $.userValidators[user]
REF_3517(mapping(address => uint16[])) -> $_1 (-> ['TMP_13435']).userValidators
REF_3518(uint16[]) -> REF_3517[user_1]
userAssociatedValidators_1(uint16[]) = ['REF_3518(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < userAssociatedValidators.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3519 -> LENGTH userAssociatedValidators_1
TMP_13436(bool) = i_2 < REF_3519
CONDITION TMP_13436
 validatorId = userAssociatedValidators[i]
REF_3520(uint16) -> userAssociatedValidators_1[i_2]
validatorId_1(uint16) := REF_3520(uint16)
 cooldownEntry = $.userValidatorCooldowns[user][validatorId]
REF_3521(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13435']).userValidatorCooldowns
REF_3522(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3521[user_1]
REF_3523(PlumeStakingStorage.CooldownEntry) -> REF_3522[validatorId_1]
cooldownEntry_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_3523(PlumeStakingStorage.CooldownEntry)
 cooldownEntry.amount == 0
REF_3524(uint256) -> cooldownEntry_1 (-> ['$']).amount
TMP_13437(bool) = REF_3524 == 0
CONDITION TMP_13437
 canRecoverFromThisCooldown = _canRecoverFromCooldown(user,validatorId,cooldownEntry)
TMP_13438(bool) = INTERNAL_CALL, StakingFacet._canRecoverFromCooldown(address,uint16,PlumeStakingStorage.CooldownEntry)(user_1,validatorId_1,cooldownEntry_1 (-> ['$']))
canRecoverFromThisCooldown_1(bool) := TMP_13438(bool)
 canRecoverFromThisCooldown
CONDITION canRecoverFromThisCooldown_1
 amountInThisCooldown = cooldownEntry.amount
REF_3525(uint256) -> cooldownEntry_1 (-> ['$']).amount
amountInThisCooldown_1(uint256) := REF_3525(uint256)
 amountMovedToParked += amountInThisCooldown
amountMovedToParked_2(uint256) = amountMovedToParked_1 (c)+ amountInThisCooldown_1
 _removeCoolingAmounts(user,validatorId,amountInThisCooldown)
INTERNAL_CALL, StakingFacet._removeCoolingAmounts(address,uint16,uint256)(user_1,validatorId_1,amountInThisCooldown_1)
 delete $.userValidatorCooldowns[user][validatorId]
REF_3526(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13435']).userValidatorCooldowns
REF_3527(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3526[user_1]
REF_3528(PlumeStakingStorage.CooldownEntry) -> REF_3527[validatorId_1]
REF_3527 = delete REF_3528 
 $.userValidatorStakes[user][validatorId].staked == 0
REF_3529(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13435']).userValidatorStakes
REF_3530(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3529[user_1]
REF_3531(PlumeStakingStorage.UserValidatorStake) -> REF_3530[validatorId_1]
REF_3532(uint256) -> REF_3531.staked
TMP_13440(bool) = REF_3532 == 0
CONDITION TMP_13440
 PlumeValidatorLogic.removeStakerFromValidator($,user,validatorId)
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.removeStakerFromValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13435'])", 'user_1', 'validatorId_1'] 
amountMovedToParked_3(uint256) := phi(['amountMovedToParked_1', 'amountMovedToParked_2'])
$_2 (-> ['TMP_13435'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13435'])", "$_1 (-> ['TMP_13435'])"])
 i ++
TMP_13442(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 amountMovedToParked > 0
TMP_13443(bool) = amountMovedToParked_1 > 0
CONDITION TMP_13443
 _updateParkedAmounts(user,amountMovedToParked)
INTERNAL_CALL, StakingFacet._updateParkedAmounts(address,uint256)(user_1,amountMovedToParked_1)
 amountMovedToParked
RETURN amountMovedToParked_1
 amountMovedToParked
```
#### StakingFacet._canRecoverFromCooldown(address,uint16,PlumeStakingStorage.CooldownEntry) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
cooldownEntry_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := phi(["cooldownEntry_1 (-> ['$'])"])
 $ = PlumeStakingStorage.layout()
TMP_13445(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13445'])(PlumeStakingStorage.Layout) := TMP_13445(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_3535(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13445']).validatorExists
REF_3536(bool) -> REF_3535[validatorId_1]
TMP_13446 = UnaryType.BANG REF_3536 
CONDITION TMP_13446
 false
RETURN False
 $.validators[validatorId].slashed
REF_3537(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13445']).validators
REF_3538(PlumeStakingStorage.ValidatorInfo) -> REF_3537[validatorId_1]
REF_3539(bool) -> REF_3538.slashed
CONDITION REF_3539
 slashTs = $.validators[validatorId].slashedAtTimestamp
REF_3540(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13445']).validators
REF_3541(PlumeStakingStorage.ValidatorInfo) -> REF_3540[validatorId_1]
REF_3542(uint256) -> REF_3541.slashedAtTimestamp
slashTs_1(uint256) := REF_3542(uint256)
 (cooldownEntry.cooldownEndTime < slashTs && block.timestamp >= cooldownEntry.cooldownEndTime)
REF_3543(uint256) -> cooldownEntry_1 (-> ['$']).cooldownEndTime
TMP_13447(bool) = REF_3543 < slashTs_1
REF_3544(uint256) -> cooldownEntry_1 (-> ['$']).cooldownEndTime
TMP_13448(bool) = block.timestamp >= REF_3544
TMP_13449(bool) = TMP_13447 && TMP_13448
RETURN TMP_13449
 (block.timestamp >= cooldownEntry.cooldownEndTime)
REF_3545(uint256) -> cooldownEntry_1 (-> ['$']).cooldownEndTime
TMP_13450(bool) = block.timestamp >= REF_3545
RETURN TMP_13450
 canRecover
```
#### StakingFacet._initializeRewardStateForNewStake(address,uint16) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13451(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13451'])(PlumeStakingStorage.Layout) := TMP_13451(PlumeStakingStorage.Layout)
 $.userValidatorStakeStartTime[user][validatorId] = block.timestamp
REF_3547(mapping(address => mapping(uint16 => uint256))) -> $_1 (-> ['TMP_13451']).userValidatorStakeStartTime
REF_3548(mapping(uint16 => uint256)) -> REF_3547[user_1]
REF_3549(uint256) -> REF_3548[validatorId_1]
$_2 (-> ['TMP_13451'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13451'])"])
REF_3549(uint256) (->$_2 (-> ['TMP_13451'])) := block.timestamp(uint256)
TMP_13451(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13451'])"])
 rewardTokens = $.rewardTokens
REF_3550(address[]) -> $_2 (-> ['TMP_13451']).rewardTokens
rewardTokens_1(address[]) = ['REF_3550(address[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < rewardTokens.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3551 -> LENGTH rewardTokens_1
TMP_13452(bool) = i_2 < REF_3551
CONDITION TMP_13452
 token = rewardTokens[i]
REF_3552(address) -> rewardTokens_1[i_2]
token_1(address) := REF_3552(address)
 $.isRewardToken[token]
REF_3553(mapping(address => bool)) -> $_2 (-> ['TMP_13451']).isRewardToken
REF_3554(bool) -> REF_3553[token_1]
CONDITION REF_3554
 PlumeRewardLogic.updateRewardPerTokenForValidator($,token,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_2 (-> ['TMP_13451'])", 'token_1', 'validatorId_1'] 
 $.userValidatorRewardPerTokenPaid[user][validatorId][token] = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_3556(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_2 (-> ['TMP_13451']).userValidatorRewardPerTokenPaid
REF_3557(mapping(uint16 => mapping(address => uint256))) -> REF_3556[user_1]
REF_3558(mapping(address => uint256)) -> REF_3557[validatorId_1]
REF_3559(uint256) -> REF_3558[token_1]
REF_3560(mapping(uint16 => mapping(address => uint256))) -> $_2 (-> ['TMP_13451']).validatorRewardPerTokenCumulative
REF_3561(mapping(address => uint256)) -> REF_3560[validatorId_1]
REF_3562(uint256) -> REF_3561[token_1]
$_3 (-> ['TMP_13451'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13451'])"])
REF_3559(uint256) (->$_3 (-> ['TMP_13451'])) := REF_3562(uint256)
TMP_13451(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13451'])"])
 $.userValidatorRewardPerTokenPaidTimestamp[user][validatorId][token] = block.timestamp
REF_3563(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_3 (-> ['TMP_13451']).userValidatorRewardPerTokenPaidTimestamp
REF_3564(mapping(uint16 => mapping(address => uint256))) -> REF_3563[user_1]
REF_3565(mapping(address => uint256)) -> REF_3564[validatorId_1]
REF_3566(uint256) -> REF_3565[token_1]
$_4 (-> ['TMP_13451'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13451'])"])
REF_3566(uint256) (->$_4 (-> ['TMP_13451'])) := block.timestamp(uint256)
TMP_13451(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13451'])"])
 $.validatorRewardRateCheckpoints[validatorId][token].length > 0
REF_3567(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_4 (-> ['TMP_13451']).validatorRewardRateCheckpoints
REF_3568(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_3567[validatorId_1]
REF_3569(PlumeStakingStorage.RateCheckpoint[]) -> REF_3568[token_1]
REF_3570 -> LENGTH REF_3569
TMP_13454(bool) = REF_3570 > 0
CONDITION TMP_13454
 $.userLastCheckpointIndex[user][validatorId][token] = $.validatorRewardRateCheckpoints[validatorId][token].length - 1
REF_3571(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_4 (-> ['TMP_13451']).userLastCheckpointIndex
REF_3572(mapping(uint16 => mapping(address => uint256))) -> REF_3571[user_1]
REF_3573(mapping(address => uint256)) -> REF_3572[validatorId_1]
REF_3574(uint256) -> REF_3573[token_1]
REF_3575(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_4 (-> ['TMP_13451']).validatorRewardRateCheckpoints
REF_3576(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_3575[validatorId_1]
REF_3577(PlumeStakingStorage.RateCheckpoint[]) -> REF_3576[token_1]
REF_3578 -> LENGTH REF_3577
TMP_13455(uint256) = REF_3578 (c)- 1
$_5 (-> ['TMP_13451'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13451'])"])
REF_3574(uint256) (->$_5 (-> ['TMP_13451'])) := TMP_13455(uint256)
TMP_13451(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13451'])"])
 $.userLastCheckpointIndex[user][validatorId][token] = 0
REF_3579(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_4 (-> ['TMP_13451']).userLastCheckpointIndex
REF_3580(mapping(uint16 => mapping(address => uint256))) -> REF_3579[user_1]
REF_3581(mapping(address => uint256)) -> REF_3580[validatorId_1]
REF_3582(uint256) -> REF_3581[token_1]
$_6 (-> ['TMP_13451'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13451'])"])
REF_3582(uint256) (->$_6 (-> ['TMP_13451'])) := 0(uint256)
TMP_13451(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13451'])"])
$_7 (-> ['TMP_13451', 'TMP_13451'])(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13451'])", "$_6 (-> ['TMP_13451'])"])
$_8 (-> ['TMP_13451'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13451'])", "$_2 (-> ['TMP_13451'])"])
 i ++
TMP_13456(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### StakingFacet._calculateAndClaimAllRewards(address,address) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
targetToken_1(address) := phi(['tokenToRestake_1'])
 $ = PlumeStakingStorage.layout()
TMP_13457(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13457'])(PlumeStakingStorage.Layout) := TMP_13457(PlumeStakingStorage.Layout)
 totalRewards = 0
totalRewards_1(uint256) := 0(uint256)
 currentUserValidators = $.userValidators[user]
REF_3584(mapping(address => uint16[])) -> $_1 (-> ['TMP_13457']).userValidators
REF_3585(uint16[]) -> REF_3584[user_1]
currentUserValidators_1(uint16[]) = ['REF_3585(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < currentUserValidators.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3586 -> LENGTH currentUserValidators_1
TMP_13458(bool) = i_2 < REF_3586
CONDITION TMP_13458
 userValidatorId = currentUserValidators[i]
REF_3587(uint16) -> currentUserValidators_1[i_2]
userValidatorId_1(uint16) := REF_3587(uint16)
 existingRewards = $.userRewards[user][userValidatorId][targetToken]
REF_3588(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13457']).userRewards
REF_3589(mapping(uint16 => mapping(address => uint256))) -> REF_3588[user_1]
REF_3590(mapping(address => uint256)) -> REF_3589[userValidatorId_1]
REF_3591(uint256) -> REF_3590[targetToken_1]
existingRewards_1(uint256) := REF_3591(uint256)
 rewardDelta = IRewardsGetter(address(this)).getPendingRewardForValidator(user,userValidatorId,targetToken)
TMP_13459 = CONVERT this to address
TMP_13460 = CONVERT TMP_13459 to IRewardsGetter
TMP_13461(uint256) = HIGH_LEVEL_CALL, dest:TMP_13460(IRewardsGetter), function:getPendingRewardForValidator, arguments:['user_1', 'userValidatorId_1', 'targetToken_1']  
rewardDelta_1(uint256) := TMP_13461(uint256)
 totalValidatorReward = existingRewards + rewardDelta
TMP_13462(uint256) = existingRewards_1 (c)+ rewardDelta_1
totalValidatorReward_1(uint256) := TMP_13462(uint256)
 totalValidatorReward > 0
TMP_13463(bool) = totalValidatorReward_1 > 0
CONDITION TMP_13463
 totalRewards += totalValidatorReward
totalRewards_2(uint256) = totalRewards_1 (c)+ totalValidatorReward_1
 PlumeRewardLogic.updateRewardsForValidator($,user,userValidatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardsForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13457'])", 'user_1', 'userValidatorId_1'] 
 $.userRewards[user][userValidatorId][targetToken] = 0
REF_3594(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13457']).userRewards
REF_3595(mapping(uint16 => mapping(address => uint256))) -> REF_3594[user_1]
REF_3596(mapping(address => uint256)) -> REF_3595[userValidatorId_1]
REF_3597(uint256) -> REF_3596[targetToken_1]
$_2 (-> ['TMP_13457'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13457'])"])
REF_3597(uint256) (->$_2 (-> ['TMP_13457'])) := 0(uint256)
TMP_13457(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13457'])"])
 $.totalClaimableByToken[targetToken] >= totalValidatorReward
REF_3598(mapping(address => uint256)) -> $_2 (-> ['TMP_13457']).totalClaimableByToken
REF_3599(uint256) -> REF_3598[targetToken_1]
TMP_13465(bool) = REF_3599 >= totalValidatorReward_1
CONDITION TMP_13465
 $.totalClaimableByToken[targetToken] -= totalValidatorReward
REF_3600(mapping(address => uint256)) -> $_2 (-> ['TMP_13457']).totalClaimableByToken
REF_3601(uint256) -> REF_3600[targetToken_1]
$_3 (-> ['TMP_13457'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13457'])"])
REF_3601(-> $_3 (-> ['TMP_13457'])) = REF_3601 (c)- totalValidatorReward_1
TMP_13457(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13457'])"])
 $.totalClaimableByToken[targetToken] = 0
REF_3602(mapping(address => uint256)) -> $_2 (-> ['TMP_13457']).totalClaimableByToken
REF_3603(uint256) -> REF_3602[targetToken_1]
$_4 (-> ['TMP_13457'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13457'])"])
REF_3603(uint256) (->$_4 (-> ['TMP_13457'])) := 0(uint256)
TMP_13457(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13457'])"])
$_5 (-> ['TMP_13457'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13457'])", "$_4 (-> ['TMP_13457'])"])
 RewardClaimedFromValidator(user,targetToken,userValidatorId,totalValidatorReward)
Emit RewardClaimedFromValidator(user_1,targetToken_1,userValidatorId_1,totalValidatorReward_1)
totalRewards_3(uint256) := phi(['totalRewards_2', 'totalRewards_1'])
$_6 (-> ['TMP_13457'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13457'])", "$_2 (-> ['TMP_13457'])"])
 i ++
TMP_13467(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 totalRewards
RETURN totalRewards_1
 totalRewards
```
#### StakingFacet._handlePostUnstakeCleanup(address,uint16) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender'])
validatorId_1(uint16) := phi(['validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13468(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13468'])(PlumeStakingStorage.Layout) := TMP_13468(PlumeStakingStorage.Layout)
 $.userValidatorStakes[user][validatorId].staked == 0
REF_3605(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13468']).userValidatorStakes
REF_3606(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3605[user_1]
REF_3607(PlumeStakingStorage.UserValidatorStake) -> REF_3606[validatorId_1]
REF_3608(uint256) -> REF_3607.staked
TMP_13469(bool) = REF_3608 == 0
CONDITION TMP_13469
 PlumeValidatorLogic.removeStakerFromValidator($,user,validatorId)
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.removeStakerFromValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13468'])", 'user_1', 'validatorId_1']
```
#### StakingFacet._countActiveCooldowns(address) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
 $ = PlumeStakingStorage.layout()
TMP_13471(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13471'])(PlumeStakingStorage.Layout) := TMP_13471(PlumeStakingStorage.Layout)
 userAssociatedValidators = $.userValidators[user]
REF_3611(mapping(address => uint16[])) -> $_1 (-> ['TMP_13471']).userValidators
REF_3612(uint16[]) -> REF_3611[user_1]
userAssociatedValidators_1 (-> [])(uint16[]) = ['REF_3612(uint16[])']
 count = 0
count_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < userAssociatedValidators.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3613 -> LENGTH userAssociatedValidators_1 (-> [])
TMP_13472(bool) = i_2 < REF_3613
CONDITION TMP_13472
 validatorId = userAssociatedValidators[i]
REF_3614(uint16) -> userAssociatedValidators_1 (-> [])[i_2]
validatorId_1(uint16) := REF_3614(uint16)
 $.validatorExists[validatorId] && ! $.validators[validatorId].slashed && $.userValidatorCooldowns[user][validatorId].amount > 0
REF_3615(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13471']).validatorExists
REF_3616(bool) -> REF_3615[validatorId_1]
REF_3617(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13471']).validators
REF_3618(PlumeStakingStorage.ValidatorInfo) -> REF_3617[validatorId_1]
REF_3619(bool) -> REF_3618.slashed
TMP_13473 = UnaryType.BANG REF_3619 
TMP_13474(bool) = REF_3616 && TMP_13473
REF_3620(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13471']).userValidatorCooldowns
REF_3621(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3620[user_1]
REF_3622(PlumeStakingStorage.CooldownEntry) -> REF_3621[validatorId_1]
REF_3623(uint256) -> REF_3622.amount
TMP_13475(bool) = REF_3623 > 0
TMP_13476(bool) = TMP_13474 && TMP_13475
CONDITION TMP_13476
 count ++
TMP_13477(uint256) := count_1(uint256)
count_2(uint256) = count_1 (c)+ 1
count_3(uint256) := phi(['count_1', 'count_2'])
 i ++
TMP_13478(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 count
RETURN count_1
 count
```
#### StakingFacet._calculateActivelyCoolingAmount(address) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender'])
 $ = PlumeStakingStorage.layout()
TMP_13479(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13479'])(PlumeStakingStorage.Layout) := TMP_13479(PlumeStakingStorage.Layout)
 userAssociatedValidators = $.userValidators[user]
REF_3625(mapping(address => uint16[])) -> $_1 (-> ['TMP_13479']).userValidators
REF_3626(uint16[]) -> REF_3625[user_1]
userAssociatedValidators_1 (-> [])(uint16[]) = ['REF_3626(uint16[])']
 activelyCoolingAmount = 0
activelyCoolingAmount_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < userAssociatedValidators.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3627 -> LENGTH userAssociatedValidators_1 (-> [])
TMP_13480(bool) = i_2 < REF_3627
CONDITION TMP_13480
 validatorId = userAssociatedValidators[i]
REF_3628(uint16) -> userAssociatedValidators_1 (-> [])[i_2]
validatorId_1(uint16) := REF_3628(uint16)
 $.validatorExists[validatorId] && ! $.validators[validatorId].slashed
REF_3629(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13479']).validatorExists
REF_3630(bool) -> REF_3629[validatorId_1]
REF_3631(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13479']).validators
REF_3632(PlumeStakingStorage.ValidatorInfo) -> REF_3631[validatorId_1]
REF_3633(bool) -> REF_3632.slashed
TMP_13481 = UnaryType.BANG REF_3633 
TMP_13482(bool) = REF_3630 && TMP_13481
CONDITION TMP_13482
 cooldownEntry = $.userValidatorCooldowns[user][validatorId]
REF_3634(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13479']).userValidatorCooldowns
REF_3635(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3634[user_1]
REF_3636(PlumeStakingStorage.CooldownEntry) -> REF_3635[validatorId_1]
cooldownEntry_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_3636(PlumeStakingStorage.CooldownEntry)
 cooldownEntry.amount > 0 && block.timestamp < cooldownEntry.cooldownEndTime
REF_3637(uint256) -> cooldownEntry_1 (-> ['$']).amount
TMP_13483(bool) = REF_3637 > 0
REF_3638(uint256) -> cooldownEntry_1 (-> ['$']).cooldownEndTime
TMP_13484(bool) = block.timestamp < REF_3638
TMP_13485(bool) = TMP_13483 && TMP_13484
CONDITION TMP_13485
 activelyCoolingAmount += cooldownEntry.amount
REF_3639(uint256) -> cooldownEntry_1 (-> ['$']).amount
activelyCoolingAmount_2(uint256) = activelyCoolingAmount_1 (c)+ REF_3639
activelyCoolingAmount_3(uint256) := phi(['activelyCoolingAmount_1', 'activelyCoolingAmount_2'])
 i ++
TMP_13486(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 activelyCoolingAmount
RETURN activelyCoolingAmount_1
 activelyCoolingAmount
```
#### StakingFacet._calculateTotalWithdrawableAmount(address) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender'])
 $ = PlumeStakingStorage.layout()
TMP_13487(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13487'])(PlumeStakingStorage.Layout) := TMP_13487(PlumeStakingStorage.Layout)
 userAssociatedValidators = $.userValidators[user]
REF_3641(mapping(address => uint16[])) -> $_1 (-> ['TMP_13487']).userValidators
REF_3642(uint16[]) -> REF_3641[user_1]
userAssociatedValidators_1 (-> [])(uint16[]) = ['REF_3642(uint16[])']
 totalWithdrawableAmount = $.stakeInfo[user].parked
REF_3643(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13487']).stakeInfo
REF_3644(PlumeStakingStorage.StakeInfo) -> REF_3643[user_1]
REF_3645(uint256) -> REF_3644.parked
totalWithdrawableAmount_1(uint256) := REF_3645(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < userAssociatedValidators.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3646 -> LENGTH userAssociatedValidators_1 (-> [])
TMP_13488(bool) = i_2 < REF_3646
CONDITION TMP_13488
 validatorId = userAssociatedValidators[i]
REF_3647(uint16) -> userAssociatedValidators_1 (-> [])[i_2]
validatorId_1(uint16) := REF_3647(uint16)
 cooldownEntry = $.userValidatorCooldowns[user][validatorId]
REF_3648(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13487']).userValidatorCooldowns
REF_3649(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3648[user_1]
REF_3650(PlumeStakingStorage.CooldownEntry) -> REF_3649[validatorId_1]
cooldownEntry_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_3650(PlumeStakingStorage.CooldownEntry)
 cooldownEntry.amount > 0 && block.timestamp >= cooldownEntry.cooldownEndTime
REF_3651(uint256) -> cooldownEntry_1 (-> ['$']).amount
TMP_13489(bool) = REF_3651 > 0
REF_3652(uint256) -> cooldownEntry_1 (-> ['$']).cooldownEndTime
TMP_13490(bool) = block.timestamp >= REF_3652
TMP_13491(bool) = TMP_13489 && TMP_13490
CONDITION TMP_13491
 $.validatorExists[validatorId] && $.validators[validatorId].slashed
REF_3653(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13487']).validatorExists
REF_3654(bool) -> REF_3653[validatorId_1]
REF_3655(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13487']).validators
REF_3656(PlumeStakingStorage.ValidatorInfo) -> REF_3655[validatorId_1]
REF_3657(bool) -> REF_3656.slashed
TMP_13492(bool) = REF_3654 && REF_3657
CONDITION TMP_13492
 cooldownEntry.cooldownEndTime < $.validators[validatorId].slashedAtTimestamp
REF_3658(uint256) -> cooldownEntry_1 (-> ['$']).cooldownEndTime
REF_3659(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13487']).validators
REF_3660(PlumeStakingStorage.ValidatorInfo) -> REF_3659[validatorId_1]
REF_3661(uint256) -> REF_3660.slashedAtTimestamp
TMP_13493(bool) = REF_3658 < REF_3661
CONDITION TMP_13493
 totalWithdrawableAmount += cooldownEntry.amount
REF_3662(uint256) -> cooldownEntry_1 (-> ['$']).amount
totalWithdrawableAmount_4(uint256) = totalWithdrawableAmount_1 (c)+ REF_3662
totalWithdrawableAmount_5(uint256) := phi(['totalWithdrawableAmount_4', 'totalWithdrawableAmount_1'])
 $.validatorExists[validatorId] && ! $.validators[validatorId].slashed
REF_3663(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13487']).validatorExists
REF_3664(bool) -> REF_3663[validatorId_1]
REF_3665(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13487']).validators
REF_3666(PlumeStakingStorage.ValidatorInfo) -> REF_3665[validatorId_1]
REF_3667(bool) -> REF_3666.slashed
TMP_13494 = UnaryType.BANG REF_3667 
TMP_13495(bool) = REF_3664 && TMP_13494
CONDITION TMP_13495
 totalWithdrawableAmount += cooldownEntry.amount
REF_3668(uint256) -> cooldownEntry_1 (-> ['$']).amount
totalWithdrawableAmount_2(uint256) = totalWithdrawableAmount_1 (c)+ REF_3668
totalWithdrawableAmount_3(uint256) := phi(['totalWithdrawableAmount_1', 'totalWithdrawableAmount_2'])
 i ++
TMP_13496(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 totalWithdrawableAmount
RETURN totalWithdrawableAmount_1
 totalWithdrawableAmount
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
#### PlumeRewardLogic.updateRewardsForValidator(PlumeStakingStorage.Layout,address,uint16) [INTERNAL]
```slithir
 rewardTokens = $.rewardTokens
REF_4092(address[]) -> $_1 (-> []).rewardTokens
rewardTokens_1(address[]) = ['REF_4092(address[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < rewardTokens.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4093 -> LENGTH rewardTokens_1
TMP_13850(bool) = i_2 < REF_4093
CONDITION TMP_13850
 token = rewardTokens[i]
REF_4094(address) -> rewardTokens_1[i_2]
token_1(address) := REF_4094(address)
 validatorLastGlobalUpdateTimestampAtLoopStart = $.validatorLastUpdateTimes[validatorId][token]
REF_4095(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4096(mapping(address => uint256)) -> REF_4095[validatorId_1]
REF_4097(uint256) -> REF_4096[token_1]
validatorLastGlobalUpdateTimestampAtLoopStart_1(uint256) := REF_4097(uint256)
 updateRewardPerTokenForValidator($,token,validatorId)
INTERNAL_CALL, PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16)($_1 (-> []),token_1,validatorId_1)
 validator = $.validators[validatorId]
REF_4098(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> []).validators
REF_4099(PlumeStakingStorage.ValidatorInfo) -> REF_4098[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4099(PlumeStakingStorage.ValidatorInfo)
 validatorCommission = validator.commission
REF_4100(uint256) -> validator_1 (-> ['$']).commission
validatorCommission_1(uint256) := REF_4100(uint256)
 userStakedAmount = $.userValidatorStakes[user][validatorId].staked
REF_4101(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> []).userValidatorStakes
REF_4102(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_4101[user_1]
REF_4103(PlumeStakingStorage.UserValidatorStake) -> REF_4102[validatorId_1]
REF_4104(uint256) -> REF_4103.staked
userStakedAmount_1(uint256) := REF_4104(uint256)
 userStakedAmount == 0
TMP_13852(bool) = userStakedAmount_1 == 0
CONDITION TMP_13852
 $.userValidatorStakeStartTime[user][validatorId] == 0
REF_4105(mapping(address => mapping(uint16 => uint256))) -> $_1 (-> []).userValidatorStakeStartTime
REF_4106(mapping(uint16 => uint256)) -> REF_4105[user_1]
REF_4107(uint256) -> REF_4106[validatorId_1]
TMP_13853(bool) = REF_4107 == 0
CONDITION TMP_13853
 $.userValidatorStakeStartTime[user][validatorId] = block.timestamp
REF_4108(mapping(address => mapping(uint16 => uint256))) -> $_1 (-> []).userValidatorStakeStartTime
REF_4109(mapping(uint16 => uint256)) -> REF_4108[user_1]
REF_4110(uint256) -> REF_4109[validatorId_1]
$_3 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4110(uint256) (->$_3 (-> [])) := block.timestamp(uint256)
$_4 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_3 (-> [])'])
 (userRewardDelta,commissionAmountDelta,effectiveTimeDelta) = calculateRewardsWithCheckpoints($,user,validatorId,token,userStakedAmount)
TUPLE_92(uint256,uint256,uint256) = INTERNAL_CALL, PlumeRewardLogic.calculateRewardsWithCheckpoints(PlumeStakingStorage.Layout,address,uint16,address,uint256)($_4 (-> []),user_1,validatorId_1,token_1,userStakedAmount_1)
userRewardDelta_1(uint256)= UNPACK TUPLE_92 index: 0 
commissionAmountDelta_1(uint256)= UNPACK TUPLE_92 index: 1 
effectiveTimeDelta_1(uint256)= UNPACK TUPLE_92 index: 2 
 userRewardDelta > 0
TMP_13854(bool) = userRewardDelta_1 > 0
CONDITION TMP_13854
 $.userRewards[user][validatorId][token] += userRewardDelta
REF_4111(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_4 (-> []).userRewards
REF_4112(mapping(uint16 => mapping(address => uint256))) -> REF_4111[user_1]
REF_4113(mapping(address => uint256)) -> REF_4112[validatorId_1]
REF_4114(uint256) -> REF_4113[token_1]
$_5 (-> [])(PlumeStakingStorage.Layout) := phi(['$_4 (-> [])'])
REF_4114(-> $_5 (-> [])) = REF_4114 (c)+ userRewardDelta_1
 $.totalClaimableByToken[token] += userRewardDelta
REF_4115(mapping(address => uint256)) -> $_5 (-> []).totalClaimableByToken
REF_4116(uint256) -> REF_4115[token_1]
$_6 (-> [])(PlumeStakingStorage.Layout) := phi(['$_5 (-> [])'])
REF_4116(-> $_6 (-> [])) = REF_4116 (c)+ userRewardDelta_1
 $.userHasPendingRewards[user][validatorId] = true
REF_4117(mapping(address => mapping(uint16 => bool))) -> $_6 (-> []).userHasPendingRewards
REF_4118(mapping(uint16 => bool)) -> REF_4117[user_1]
REF_4119(bool) -> REF_4118[validatorId_1]
$_7 (-> [])(PlumeStakingStorage.Layout) := phi(['$_6 (-> [])'])
REF_4119(bool) (->$_7 (-> [])) := True(bool)
$_8 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_7 (-> [])'])
 $.userValidatorRewardPerTokenPaid[user][validatorId][token] = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_4120(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_8 (-> []).userValidatorRewardPerTokenPaid
REF_4121(mapping(uint16 => mapping(address => uint256))) -> REF_4120[user_1]
REF_4122(mapping(address => uint256)) -> REF_4121[validatorId_1]
REF_4123(uint256) -> REF_4122[token_1]
REF_4124(mapping(uint16 => mapping(address => uint256))) -> $_8 (-> []).validatorRewardPerTokenCumulative
REF_4125(mapping(address => uint256)) -> REF_4124[validatorId_1]
REF_4126(uint256) -> REF_4125[token_1]
$_9 (-> [])(PlumeStakingStorage.Layout) := phi(['$_8 (-> [])'])
REF_4123(uint256) (->$_9 (-> [])) := REF_4126(uint256)
 $.userValidatorRewardPerTokenPaidTimestamp[user][validatorId][token] = block.timestamp
REF_4127(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_9 (-> []).userValidatorRewardPerTokenPaidTimestamp
REF_4128(mapping(uint16 => mapping(address => uint256))) -> REF_4127[user_1]
REF_4129(mapping(address => uint256)) -> REF_4128[validatorId_1]
REF_4130(uint256) -> REF_4129[token_1]
$_10 (-> [])(PlumeStakingStorage.Layout) := phi(['$_9 (-> [])'])
REF_4130(uint256) (->$_10 (-> [])) := block.timestamp(uint256)
 $.validatorRewardRateCheckpoints[validatorId][token].length > 0
REF_4131(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_10 (-> []).validatorRewardRateCheckpoints
REF_4132(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4131[validatorId_1]
REF_4133(PlumeStakingStorage.RateCheckpoint[]) -> REF_4132[token_1]
REF_4134 -> LENGTH REF_4133
TMP_13855(bool) = REF_4134 > 0
CONDITION TMP_13855
 $.userLastCheckpointIndex[user][validatorId][token] = $.validatorRewardRateCheckpoints[validatorId][token].length - 1
REF_4135(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_10 (-> []).userLastCheckpointIndex
REF_4136(mapping(uint16 => mapping(address => uint256))) -> REF_4135[user_1]
REF_4137(mapping(address => uint256)) -> REF_4136[validatorId_1]
REF_4138(uint256) -> REF_4137[token_1]
REF_4139(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_10 (-> []).validatorRewardRateCheckpoints
REF_4140(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4139[validatorId_1]
REF_4141(PlumeStakingStorage.RateCheckpoint[]) -> REF_4140[token_1]
REF_4142 -> LENGTH REF_4141
TMP_13856(uint256) = REF_4142 (c)- 1
$_11 (-> [])(PlumeStakingStorage.Layout) := phi(['$_10 (-> [])'])
REF_4138(uint256) (->$_11 (-> [])) := TMP_13856(uint256)
$_12 (-> [])(PlumeStakingStorage.Layout) := phi(['$_11 (-> [])', '$_10 (-> [])'])
 i ++
$_2 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_10 (-> [])'])
TMP_13857(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### PlumeValidatorLogic.addStakerToValidator(PlumeStakingStorage.Layout,address,uint16) [INTERNAL]
```slithir
 ! $.userHasStakedWithValidator[staker][validatorId]
REF_4313(mapping(address => mapping(uint16 => bool))) -> $_1 (-> []).userHasStakedWithValidator
REF_4314(mapping(uint16 => bool)) -> REF_4313[staker_1]
REF_4315(bool) -> REF_4314[validatorId_1]
TMP_14025 = UnaryType.BANG REF_4315 
CONDITION TMP_14025
 $.userValidators[staker].push(validatorId)
REF_4316(mapping(address => uint16[])) -> $_1 (-> []).userValidators
REF_4317(uint16[]) -> REF_4316[staker_1]
REF_4319 -> LENGTH REF_4317
TMP_14027(uint256) := REF_4319(uint256)
TMP_14028(uint256) = TMP_14027 (c)+ 1
$_2 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4319(uint256) (->$_3 (-> [])) := TMP_14028(uint256)
REF_4320(uint16) -> REF_4317[TMP_14027]
$_3 (-> [])(PlumeStakingStorage.Layout) := phi(['$_2 (-> [])'])
REF_4320(uint16) (->$_3 (-> [])) := validatorId_1(uint16)
 $.userHasStakedWithValidator[staker][validatorId] = true
REF_4321(mapping(address => mapping(uint16 => bool))) -> $_3 (-> []).userHasStakedWithValidator
REF_4322(mapping(uint16 => bool)) -> REF_4321[staker_1]
REF_4323(bool) -> REF_4322[validatorId_1]
$_4 (-> [])(PlumeStakingStorage.Layout) := phi(['$_3 (-> [])'])
REF_4323(bool) (->$_4 (-> [])) := True(bool)
$_5 (-> [])(PlumeStakingStorage.Layout) := phi(['$_4 (-> [])', '$_1 (-> [])'])
 ! $.isStakerForValidator[validatorId][staker]
REF_4324(mapping(uint16 => mapping(address => bool))) -> $_5 (-> []).isStakerForValidator
REF_4325(mapping(address => bool)) -> REF_4324[validatorId_1]
REF_4326(bool) -> REF_4325[staker_1]
TMP_14029 = UnaryType.BANG REF_4326 
CONDITION TMP_14029
 index = $.validatorStakers[validatorId].length
REF_4327(mapping(uint16 => address[])) -> $_5 (-> []).validatorStakers
REF_4328(address[]) -> REF_4327[validatorId_1]
REF_4329 -> LENGTH REF_4328
index_1(uint256) := REF_4329(uint256)
 $.validatorStakers[validatorId].push(staker)
REF_4330(mapping(uint16 => address[])) -> $_5 (-> []).validatorStakers
REF_4331(address[]) -> REF_4330[validatorId_1]
REF_4333 -> LENGTH REF_4331
TMP_14031(uint256) := REF_4333(uint256)
TMP_14032(uint256) = TMP_14031 (c)+ 1
$_6 (-> [])(PlumeStakingStorage.Layout) := phi(['$_5 (-> [])'])
REF_4333(uint256) (->$_7 (-> [])) := TMP_14032(uint256)
REF_4334(address) -> REF_4331[TMP_14031]
$_7 (-> [])(PlumeStakingStorage.Layout) := phi(['$_6 (-> [])'])
REF_4334(address) (->$_7 (-> [])) := staker_1(address)
 $.isStakerForValidator[validatorId][staker] = true
REF_4335(mapping(uint16 => mapping(address => bool))) -> $_7 (-> []).isStakerForValidator
REF_4336(mapping(address => bool)) -> REF_4335[validatorId_1]
REF_4337(bool) -> REF_4336[staker_1]
$_8 (-> [])(PlumeStakingStorage.Layout) := phi(['$_7 (-> [])'])
REF_4337(bool) (->$_8 (-> [])) := True(bool)
 $.userIndexInValidatorStakers[staker][validatorId] = index
REF_4338(mapping(address => mapping(uint16 => uint256))) -> $_8 (-> []).userIndexInValidatorStakers
REF_4339(mapping(uint16 => uint256)) -> REF_4338[staker_1]
REF_4340(uint256) -> REF_4339[validatorId_1]
$_9 (-> [])(PlumeStakingStorage.Layout) := phi(['$_8 (-> [])'])
REF_4340(uint256) (->$_9 (-> [])) := index_1(uint256)
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
#### PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_1 (-> [])', '$_1 (-> [])', '$_1 (-> [])'])
token_1(address) := phi(['token_1', 'token_1', 'token_1', 'token_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1', 'validatorId_1', 'validatorId_1'])
 validator = $.validators[validatorId]
REF_4143(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> []).validators
REF_4144(PlumeStakingStorage.ValidatorInfo) -> REF_4143[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4144(PlumeStakingStorage.ValidatorInfo)
 validator.slashed
REF_4145(bool) -> validator_1 (-> ['$']).slashed
CONDITION REF_4145
 slashTs = validator.slashedAtTimestamp
REF_4146(uint256) -> validator_1 (-> ['$']).slashedAtTimestamp
slashTs_1(uint256) := REF_4146(uint256)
 currentLastUpdateTime = $.validatorLastUpdateTimes[validatorId][token]
REF_4147(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4148(mapping(address => uint256)) -> REF_4147[validatorId_1]
REF_4149(uint256) -> REF_4148[token_1]
currentLastUpdateTime_1(uint256) := REF_4149(uint256)
 currentLastUpdateTime < effectiveTimestampForUpdate
TMP_13858(bool) = currentLastUpdateTime_1 < effectiveTimestampForUpdate_3
CONDITION TMP_13858
 totalStakedForCalc = $.validatorTotalStaked[validatorId]
REF_4150(mapping(uint16 => uint256)) -> $_1 (-> []).validatorTotalStaked
REF_4151(uint256) -> REF_4150[validatorId_1]
totalStakedForCalc_1(uint256) := REF_4151(uint256)
 totalStakedForCalc > 0
TMP_13859(bool) = totalStakedForCalc_1 > 0
CONDITION TMP_13859
 timeDelta = effectiveTimestampForUpdate - currentLastUpdateTime
TMP_13860(uint256) = effectiveTimestampForUpdate_3 (c)- currentLastUpdateTime_1
timeDelta_1(uint256) := TMP_13860(uint256)
 effectiveRewardRateChk = getEffectiveRewardRateAt($,token,validatorId,effectiveTimestampForUpdate)
TMP_13861(PlumeStakingStorage.RateCheckpoint) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256)($_1 (-> []),token_1,validatorId_1,effectiveTimestampForUpdate_3)
effectiveRewardRateChk_1(PlumeStakingStorage.RateCheckpoint) := TMP_13861(PlumeStakingStorage.RateCheckpoint)
 effectiveRewardRate = effectiveRewardRateChk.rate
REF_4152(uint256) -> effectiveRewardRateChk_1.rate
effectiveRewardRate_1(uint256) := REF_4152(uint256)
 effectiveRewardRate > 0 && timeDelta > 0
TMP_13862(bool) = effectiveRewardRate_1 > 0
TMP_13863(bool) = timeDelta_1 > 0
TMP_13864(bool) = TMP_13862 && TMP_13863
CONDITION TMP_13864
 rewardPerTokenIncrease = timeDelta * effectiveRewardRate
TMP_13865(uint256) = timeDelta_1 (c)* effectiveRewardRate_1
rewardPerTokenIncrease_1(uint256) := TMP_13865(uint256)
 $.validatorRewardPerTokenCumulative[validatorId][token] += rewardPerTokenIncrease
REF_4153(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorRewardPerTokenCumulative
REF_4154(mapping(address => uint256)) -> REF_4153[validatorId_1]
REF_4155(uint256) -> REF_4154[token_1]
$_3 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4155(-> $_3 (-> [])) = REF_4155 (c)+ rewardPerTokenIncrease_1
 commissionRateForSegment = getEffectiveCommissionRateAt($,validatorId,currentLastUpdateTime)
TMP_13866(uint256) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveCommissionRateAt(PlumeStakingStorage.Layout,uint16,uint256)($_3 (-> []),validatorId_1,currentLastUpdateTime_1)
commissionRateForSegment_1(uint256) := TMP_13866(uint256)
 grossRewardForValidatorThisSegment = (totalStakedForCalc * rewardPerTokenIncrease) / PlumeStakingStorage.REWARD_PRECISION
TMP_13867(uint256) = totalStakedForCalc_1 (c)* rewardPerTokenIncrease_1
REF_4156(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_13868(uint256) = TMP_13867 (c)/ REF_4156
grossRewardForValidatorThisSegment_1(uint256) := TMP_13868(uint256)
 commissionDeltaForValidator = (grossRewardForValidatorThisSegment * commissionRateForSegment) / PlumeStakingStorage.REWARD_PRECISION
TMP_13869(uint256) = grossRewardForValidatorThisSegment_1 (c)* commissionRateForSegment_1
REF_4157(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_13870(uint256) = TMP_13869 (c)/ REF_4157
commissionDeltaForValidator_1(uint256) := TMP_13870(uint256)
 commissionDeltaForValidator > 0
TMP_13871(bool) = commissionDeltaForValidator_1 > 0
CONDITION TMP_13871
 $.validatorAccruedCommission[validatorId][token] += commissionDeltaForValidator
REF_4158(mapping(uint16 => mapping(address => uint256))) -> $_3 (-> []).validatorAccruedCommission
REF_4159(mapping(address => uint256)) -> REF_4158[validatorId_1]
REF_4160(uint256) -> REF_4159[token_1]
$_4 (-> [])(PlumeStakingStorage.Layout) := phi(['$_3 (-> [])'])
REF_4160(-> $_4 (-> [])) = REF_4160 (c)+ commissionDeltaForValidator_1
$_5 (-> [])(PlumeStakingStorage.Layout) := phi(['$_4 (-> [])', '$_3 (-> [])'])
$_6 (-> [])(PlumeStakingStorage.Layout) := phi(['$_3 (-> [])', '$_1 (-> [])'])
 $.validatorLastUpdateTimes[validatorId][token] = effectiveTimestampForUpdate
REF_4161(mapping(uint16 => mapping(address => uint256))) -> $_6 (-> []).validatorLastUpdateTimes
REF_4162(mapping(address => uint256)) -> REF_4161[validatorId_1]
REF_4163(uint256) -> REF_4162[token_1]
$_7 (-> [])(PlumeStakingStorage.Layout) := phi(['$_6 (-> [])'])
REF_4163(uint256) (->$_7 (-> [])) := effectiveTimestampForUpdate_3(uint256)
 block.timestamp > $.validatorLastUpdateTimes[validatorId][token]
REF_4164(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4165(mapping(address => uint256)) -> REF_4164[validatorId_1]
REF_4166(uint256) -> REF_4165[token_1]
TMP_13872(bool) = block.timestamp > REF_4166
CONDITION TMP_13872
 $.validatorLastUpdateTimes[validatorId][token] = block.timestamp
REF_4167(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4168(mapping(address => uint256)) -> REF_4167[validatorId_1]
REF_4169(uint256) -> REF_4168[token_1]
$_2 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4169(uint256) (->$_2 (-> [])) := block.timestamp(uint256)
 totalStaked = $.validatorTotalStaked[validatorId]
REF_4170(mapping(uint16 => uint256)) -> $_1 (-> []).validatorTotalStaked
REF_4171(uint256) -> REF_4170[validatorId_1]
totalStaked_1(uint256) := REF_4171(uint256)
 oldLastUpdateTime = $.validatorLastUpdateTimes[validatorId][token]
REF_4172(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4173(mapping(address => uint256)) -> REF_4172[validatorId_1]
REF_4174(uint256) -> REF_4173[token_1]
oldLastUpdateTime_1(uint256) := REF_4174(uint256)
 block.timestamp > oldLastUpdateTime
TMP_13873(bool) = block.timestamp > oldLastUpdateTime_1
CONDITION TMP_13873
 totalStaked > 0
TMP_13874(bool) = totalStaked_1 > 0
CONDITION TMP_13874
 timeDelta_scope_0 = block.timestamp - oldLastUpdateTime
TMP_13875(uint256) = block.timestamp (c)- oldLastUpdateTime_1
timeDelta_scope_0_1(uint256) := TMP_13875(uint256)
 effectiveRewardRateChk_scope_1 = getEffectiveRewardRateAt($,token,validatorId,block.timestamp)
TMP_13876(PlumeStakingStorage.RateCheckpoint) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256)($_1 (-> []),token_1,validatorId_1,block.timestamp)
effectiveRewardRateChk_scope_1_1(PlumeStakingStorage.RateCheckpoint) := TMP_13876(PlumeStakingStorage.RateCheckpoint)
 effectiveRewardRate_scope_2 = effectiveRewardRateChk_scope_1.rate
REF_4175(uint256) -> effectiveRewardRateChk_scope_1_1.rate
effectiveRewardRate_scope_2_1(uint256) := REF_4175(uint256)
 effectiveRewardRate_scope_2 > 0
TMP_13877(bool) = effectiveRewardRate_scope_2_1 > 0
CONDITION TMP_13877
 rewardPerTokenIncrease_scope_3 = timeDelta_scope_0 * effectiveRewardRate_scope_2
TMP_13878(uint256) = timeDelta_scope_0_1 (c)* effectiveRewardRate_scope_2_1
rewardPerTokenIncrease_scope_3_1(uint256) := TMP_13878(uint256)
 $.validatorRewardPerTokenCumulative[validatorId][token] += rewardPerTokenIncrease_scope_3
REF_4176(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorRewardPerTokenCumulative
REF_4177(mapping(address => uint256)) -> REF_4176[validatorId_1]
REF_4178(uint256) -> REF_4177[token_1]
$_8 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4178(-> $_8 (-> [])) = REF_4178 (c)+ rewardPerTokenIncrease_scope_3_1
 commissionRateForSegment_scope_4 = getEffectiveCommissionRateAt($,validatorId,oldLastUpdateTime)
TMP_13879(uint256) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveCommissionRateAt(PlumeStakingStorage.Layout,uint16,uint256)($_8 (-> []),validatorId_1,oldLastUpdateTime_1)
commissionRateForSegment_scope_4_1(uint256) := TMP_13879(uint256)
 grossRewardForValidatorThisSegment_scope_5 = (totalStaked * rewardPerTokenIncrease_scope_3) / PlumeStakingStorage.REWARD_PRECISION
TMP_13880(uint256) = totalStaked_1 (c)* rewardPerTokenIncrease_scope_3_1
REF_4179(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_13881(uint256) = TMP_13880 (c)/ REF_4179
grossRewardForValidatorThisSegment_scope_5_1(uint256) := TMP_13881(uint256)
 commissionDeltaForValidator_scope_6 = (grossRewardForValidatorThisSegment_scope_5 * commissionRateForSegment_scope_4) / PlumeStakingStorage.REWARD_PRECISION
TMP_13882(uint256) = grossRewardForValidatorThisSegment_scope_5_1 (c)* commissionRateForSegment_scope_4_1
REF_4180(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_13883(uint256) = TMP_13882 (c)/ REF_4180
commissionDeltaForValidator_scope_6_1(uint256) := TMP_13883(uint256)
 commissionDeltaForValidator_scope_6 > 0
TMP_13884(bool) = commissionDeltaForValidator_scope_6_1 > 0
CONDITION TMP_13884
 $.validatorAccruedCommission[validatorId][token] += commissionDeltaForValidator_scope_6
REF_4181(mapping(uint16 => mapping(address => uint256))) -> $_8 (-> []).validatorAccruedCommission
REF_4182(mapping(address => uint256)) -> REF_4181[validatorId_1]
REF_4183(uint256) -> REF_4182[token_1]
$_9 (-> [])(PlumeStakingStorage.Layout) := phi(['$_8 (-> [])'])
REF_4183(-> $_9 (-> [])) = REF_4183 (c)+ commissionDeltaForValidator_scope_6_1
$_10 (-> [])(PlumeStakingStorage.Layout) := phi(['$_8 (-> [])', '$_9 (-> [])'])
$_11 (-> [])(PlumeStakingStorage.Layout) := phi(['$_8 (-> [])', '$_1 (-> [])'])
 $.validatorLastUpdateTimes[validatorId][token] = block.timestamp
REF_4184(mapping(uint16 => mapping(address => uint256))) -> $_11 (-> []).validatorLastUpdateTimes
REF_4185(mapping(address => uint256)) -> REF_4184[validatorId_1]
REF_4186(uint256) -> REF_4185[token_1]
$_12 (-> [])(PlumeStakingStorage.Layout) := phi(['$_11 (-> [])'])
REF_4186(uint256) (->$_12 (-> [])) := block.timestamp(uint256)
 block.timestamp < slashTs
TMP_13885(bool) = block.timestamp < slashTs_1
CONDITION TMP_13885
 effectiveTimestampForUpdate = block.timestamp
effectiveTimestampForUpdate_1(uint256) := block.timestamp(uint256)
 effectiveTimestampForUpdate = slashTs
effectiveTimestampForUpdate_2(uint256) := slashTs_1(uint256)
effectiveTimestampForUpdate_3(uint256) := phi(['effectiveTimestampForUpdate_1', 'effectiveTimestampForUpdate_2'])
```


#### PlumeRewardLogic.calculateRewardsWithCheckpoints(PlumeStakingStorage.Layout,address,uint16,address,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_4 (-> [])'])
user_1(address) := phi(['user_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
token_1(address) := phi(['token_1'])
userStakedAmount_1(uint256) := phi(['userStakedAmount_1'])
 updateRewardPerTokenForValidator($,token,validatorId)
INTERNAL_CALL, PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16)($_1 (-> []),token_1,validatorId_1)
 lastUserPaidCumulativeRewardPerToken = $.userValidatorRewardPerTokenPaid[user][validatorId][token]
REF_4187(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> []).userValidatorRewardPerTokenPaid
REF_4188(mapping(uint16 => mapping(address => uint256))) -> REF_4187[user_1]
REF_4189(mapping(address => uint256)) -> REF_4188[validatorId_1]
REF_4190(uint256) -> REF_4189[token_1]
lastUserPaidCumulativeRewardPerToken_1(uint256) := REF_4190(uint256)
 finalCumulativeRewardPerToken = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_4191(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorRewardPerTokenCumulative
REF_4192(mapping(address => uint256)) -> REF_4191[validatorId_1]
REF_4193(uint256) -> REF_4192[token_1]
finalCumulativeRewardPerToken_1(uint256) := REF_4193(uint256)
 lastUserRewardUpdateTime = $.userValidatorRewardPerTokenPaidTimestamp[user][validatorId][token]
REF_4194(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> []).userValidatorRewardPerTokenPaidTimestamp
REF_4195(mapping(uint16 => mapping(address => uint256))) -> REF_4194[user_1]
REF_4196(mapping(address => uint256)) -> REF_4195[validatorId_1]
REF_4197(uint256) -> REF_4196[token_1]
lastUserRewardUpdateTime_1(uint256) := REF_4197(uint256)
 lastUserRewardUpdateTime == 0
TMP_13887(bool) = lastUserRewardUpdateTime_1 == 0
CONDITION TMP_13887
 lastUserRewardUpdateTime = $.userValidatorStakeStartTime[user][validatorId]
REF_4198(mapping(address => mapping(uint16 => uint256))) -> $_1 (-> []).userValidatorStakeStartTime
REF_4199(mapping(uint16 => uint256)) -> REF_4198[user_1]
REF_4200(uint256) -> REF_4199[validatorId_1]
lastUserRewardUpdateTime_2(uint256) := REF_4200(uint256)
 lastUserRewardUpdateTime == 0 && $.userValidatorStakes[user][validatorId].staked > 0
TMP_13888(bool) = lastUserRewardUpdateTime_2 == 0
REF_4201(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> []).userValidatorStakes
REF_4202(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_4201[user_1]
REF_4203(PlumeStakingStorage.UserValidatorStake) -> REF_4202[validatorId_1]
REF_4204(uint256) -> REF_4203.staked
TMP_13889(bool) = REF_4204 > 0
TMP_13890(bool) = TMP_13888 && TMP_13889
CONDITION TMP_13890
 lastUserRewardUpdateTime = block.timestamp
lastUserRewardUpdateTime_3(uint256) := block.timestamp(uint256)
lastUserRewardUpdateTime_4(uint256) := phi(['lastUserRewardUpdateTime_2', 'lastUserRewardUpdateTime_3'])
lastUserRewardUpdateTime_5(uint256) := phi(['lastUserRewardUpdateTime_2', 'lastUserRewardUpdateTime_1'])
 block.timestamp <= lastUserRewardUpdateTime || finalCumulativeRewardPerToken <= lastUserPaidCumulativeRewardPerToken
TMP_13891(bool) = block.timestamp <= lastUserRewardUpdateTime_5
TMP_13892(bool) = finalCumulativeRewardPerToken_1 <= lastUserPaidCumulativeRewardPerToken_1
TMP_13893(bool) = TMP_13891 || TMP_13892
CONDITION TMP_13893
 (0,0,0)
RETURN 0,0,0
 effectiveTimeDelta = block.timestamp - lastUserRewardUpdateTime
TMP_13894(uint256) = block.timestamp (c)- lastUserRewardUpdateTime_5
effectiveTimeDelta_1(uint256) := TMP_13894(uint256)
 distinctTimestamps = getDistinctTimestamps($,validatorId,token,lastUserRewardUpdateTime,block.timestamp)
TMP_13895(uint256[]) = INTERNAL_CALL, PlumeRewardLogic.getDistinctTimestamps(PlumeStakingStorage.Layout,uint16,address,uint256,uint256)($_1 (-> []),validatorId_1,token_1,lastUserRewardUpdateTime_5,block.timestamp)
distinctTimestamps_1(uint256[]) = ['TMP_13895(uint256[])']
 distinctTimestamps.length < 2
REF_4205 -> LENGTH distinctTimestamps_1
TMP_13896(bool) = REF_4205 < 2
CONDITION TMP_13896
 (0,0,0)
RETURN 0,0,0
 rptTracker = lastUserPaidCumulativeRewardPerToken
rptTracker_1(uint256) := lastUserPaidCumulativeRewardPerToken_1(uint256)
 k = 0
k_1(uint256) := 0(uint256)
 k < distinctTimestamps.length - 1
k_2(uint256) := phi(['k_1', 'k_3'])
REF_4206 -> LENGTH distinctTimestamps_1
TMP_13897(uint256) = REF_4206 (c)- 1
TMP_13898(bool) = k_2 < TMP_13897
CONDITION TMP_13898
 segmentStartTime = distinctTimestamps[k]
REF_4207(uint256) -> distinctTimestamps_1[k_2]
segmentStartTime_1(uint256) := REF_4207(uint256)
 segmentEndTime = distinctTimestamps[k + 1]
TMP_13899(uint256) = k_2 (c)+ 1
REF_4208(uint256) -> distinctTimestamps_1[TMP_13899]
segmentEndTime_1(uint256) := REF_4208(uint256)
 segmentEndTime <= segmentStartTime
TMP_13900(bool) = segmentEndTime_1 <= segmentStartTime_1
CONDITION TMP_13900
 k == 0
TMP_13901(bool) = k_2 == 0
CONDITION TMP_13901
 rptAtSegmentStart = lastUserPaidCumulativeRewardPerToken
rptAtSegmentStart_1(uint256) := lastUserPaidCumulativeRewardPerToken_1(uint256)
 rptAtSegmentStart = rptTracker
rptAtSegmentStart_2(uint256) := rptTracker_1(uint256)
rptAtSegmentStart_3(uint256) := phi(['rptAtSegmentStart_1', 'rptAtSegmentStart_2'])
 rewardRateInfoForSegment = getEffectiveRewardRateAt($,token,validatorId,segmentStartTime)
TMP_13902(PlumeStakingStorage.RateCheckpoint) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256)($_1 (-> []),token_1,validatorId_1,segmentStartTime_1)
rewardRateInfoForSegment_1(PlumeStakingStorage.RateCheckpoint) := TMP_13902(PlumeStakingStorage.RateCheckpoint)
 effectiveRewardRate = rewardRateInfoForSegment.rate
REF_4209(uint256) -> rewardRateInfoForSegment_1.rate
effectiveRewardRate_1(uint256) := REF_4209(uint256)
 segmentDuration = segmentEndTime - segmentStartTime
TMP_13903(uint256) = segmentEndTime_1 (c)- segmentStartTime_1
segmentDuration_1(uint256) := TMP_13903(uint256)
 rptIncreaseInSegment = 0
rptIncreaseInSegment_1(uint256) := 0(uint256)
 effectiveRewardRate > 0 && segmentDuration > 0
TMP_13904(bool) = effectiveRewardRate_1 > 0
TMP_13905(bool) = segmentDuration_1 > 0
TMP_13906(bool) = TMP_13904 && TMP_13905
CONDITION TMP_13906
 rptIncreaseInSegment = segmentDuration * effectiveRewardRate
TMP_13907(uint256) = segmentDuration_1 (c)* effectiveRewardRate_1
rptIncreaseInSegment_2(uint256) := TMP_13907(uint256)
rptIncreaseInSegment_3(uint256) := phi(['rptIncreaseInSegment_2', 'rptIncreaseInSegment_1'])
 rptAtSegmentEnd = rptAtSegmentStart + rptIncreaseInSegment
TMP_13908(uint256) = rptAtSegmentStart_3 (c)+ rptIncreaseInSegment_3
rptAtSegmentEnd_1(uint256) := TMP_13908(uint256)
 rewardPerTokenDeltaForUserInSegment = rptAtSegmentEnd - rptAtSegmentStart
TMP_13909(uint256) = rptAtSegmentEnd_1 (c)- rptAtSegmentStart_3
rewardPerTokenDeltaForUserInSegment_1(uint256) := TMP_13909(uint256)
 rewardPerTokenDeltaForUserInSegment > 0 && userStakedAmount > 0
TMP_13910(bool) = rewardPerTokenDeltaForUserInSegment_1 > 0
TMP_13911(bool) = userStakedAmount_1 > 0
TMP_13912(bool) = TMP_13910 && TMP_13911
CONDITION TMP_13912
 grossRewardForSegment = (userStakedAmount * rewardPerTokenDeltaForUserInSegment) / PlumeStakingStorage.REWARD_PRECISION
TMP_13913(uint256) = userStakedAmount_1 (c)* rewardPerTokenDeltaForUserInSegment_1
REF_4210(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_13914(uint256) = TMP_13913 (c)/ REF_4210
grossRewardForSegment_1(uint256) := TMP_13914(uint256)
 effectiveCommissionRate = getEffectiveCommissionRateAt($,validatorId,segmentStartTime)
TMP_13915(uint256) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveCommissionRateAt(PlumeStakingStorage.Layout,uint16,uint256)($_1 (-> []),validatorId_1,segmentStartTime_1)
effectiveCommissionRate_1(uint256) := TMP_13915(uint256)
 commissionForThisSegment = _ceilDiv(grossRewardForSegment * effectiveCommissionRate,PlumeStakingStorage.REWARD_PRECISION)
TMP_13916(uint256) = grossRewardForSegment_1 (c)* effectiveCommissionRate_1
REF_4211(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_13917(uint256) = INTERNAL_CALL, PlumeRewardLogic._ceilDiv(uint256,uint256)(TMP_13916,REF_4211)
commissionForThisSegment_1(uint256) := TMP_13917(uint256)
 grossRewardForSegment >= commissionForThisSegment
TMP_13918(bool) = grossRewardForSegment_1 >= commissionForThisSegment_1
CONDITION TMP_13918
 totalUserRewardDelta += (grossRewardForSegment - commissionForThisSegment)
TMP_13919(uint256) = grossRewardForSegment_1 (c)- commissionForThisSegment_1
totalUserRewardDelta_1(uint256) = totalUserRewardDelta_0 (c)+ TMP_13919
totalUserRewardDelta_2(uint256) := phi(['totalUserRewardDelta_1', 'totalUserRewardDelta_0'])
 totalCommissionAmountDelta += commissionForThisSegment
totalCommissionAmountDelta_1(uint256) = totalCommissionAmountDelta_0 (c)+ commissionForThisSegment_1
totalCommissionAmountDelta_2(uint256) := phi(['totalCommissionAmountDelta_1', 'totalCommissionAmountDelta_0'])
 rptTracker = rptAtSegmentEnd
rptTracker_3(uint256) := rptAtSegmentEnd_1(uint256)
 ++ k
rptTracker_2(uint256) := phi(['rptTracker_1', 'rptTracker_3'])
k_3(uint256) = k_2 (c)+ 1
 (totalUserRewardDelta,totalCommissionAmountDelta,effectiveTimeDelta)
RETURN totalUserRewardDelta_0,totalCommissionAmountDelta_0,effectiveTimeDelta_1
 (totalUserRewardDelta,totalCommissionAmountDelta,effectiveTimeDelta)
```
#### PlumeRewardLogic.getEffectiveCommissionRateAt(PlumeStakingStorage.Layout,uint16,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_8 (-> [])', '$_3 (-> [])', '$_1 (-> [])'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
timestamp_1(uint256) := phi(['oldLastUpdateTime_1', 'segmentStartTime_1', 'currentLastUpdateTime_1'])
 checkpoints = $.validatorCommissionCheckpoints[validatorId]
REF_4245(mapping(uint16 => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> []).validatorCommissionCheckpoints
REF_4246(PlumeStakingStorage.RateCheckpoint[]) -> REF_4245[validatorId_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4246(PlumeStakingStorage.RateCheckpoint[])']
 chkCount = checkpoints.length
REF_4247 -> LENGTH checkpoints_1 (-> [])
chkCount_1(uint256) := REF_4247(uint256)
 chkCount > 0
TMP_13975(bool) = chkCount_1 > 0
CONDITION TMP_13975
 idx = findCommissionCheckpointIndexAtOrBefore($,validatorId,timestamp)
TMP_13976(uint256) = INTERNAL_CALL, PlumeRewardLogic.findCommissionCheckpointIndexAtOrBefore(PlumeStakingStorage.Layout,uint16,uint256)($_1 (-> []),validatorId_1,timestamp_1)
idx_1(uint256) := TMP_13976(uint256)
 idx < chkCount && checkpoints[idx].timestamp <= timestamp
TMP_13977(bool) = idx_1 < chkCount_1
REF_4248(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
REF_4249(uint256) -> REF_4248.timestamp
TMP_13978(bool) = REF_4249 <= timestamp_1
TMP_13979(bool) = TMP_13977 && TMP_13978
CONDITION TMP_13979
 checkpoints[idx].rate
REF_4250(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
REF_4251(uint256) -> REF_4250.rate
RETURN REF_4251
 fallbackComm = $.validators[validatorId].commission
REF_4252(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> []).validators
REF_4253(PlumeStakingStorage.ValidatorInfo) -> REF_4252[validatorId_1]
REF_4254(uint256) -> REF_4253.commission
fallbackComm_1(uint256) := REF_4254(uint256)
 fallbackComm
RETURN fallbackComm_1
```
#### PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_1 (-> [])'])
token_1(address) := phi(['token_1', 'token_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
timestamp_1(uint256) := phi(['segmentStartTime_1', 'effectiveTimestampForUpdate_3', 'block.timestamp'])
 checkpoints = $.validatorRewardRateCheckpoints[validatorId][token]
REF_4231(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> []).validatorRewardRateCheckpoints
REF_4232(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4231[validatorId_1]
REF_4233(PlumeStakingStorage.RateCheckpoint[]) -> REF_4232[token_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4233(PlumeStakingStorage.RateCheckpoint[])']
 chkCount = checkpoints.length
REF_4234 -> LENGTH checkpoints_1 (-> [])
chkCount_1(uint256) := REF_4234(uint256)
 chkCount > 0
TMP_13965(bool) = chkCount_1 > 0
CONDITION TMP_13965
 idx = findRewardRateCheckpointIndexAtOrBefore($,validatorId,token,timestamp)
TMP_13966(uint256) = INTERNAL_CALL, PlumeRewardLogic.findRewardRateCheckpointIndexAtOrBefore(PlumeStakingStorage.Layout,uint16,address,uint256)($_1 (-> []),validatorId_1,token_1,timestamp_1)
idx_1(uint256) := TMP_13966(uint256)
 idx < chkCount && checkpoints[idx].timestamp <= timestamp
TMP_13967(bool) = idx_1 < chkCount_1
REF_4235(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
REF_4236(uint256) -> REF_4235.timestamp
TMP_13968(bool) = REF_4236 <= timestamp_1
TMP_13969(bool) = TMP_13967 && TMP_13968
CONDITION TMP_13969
 idx + 1 < chkCount && checkpoints[idx + 1].timestamp <= timestamp
TMP_13970(uint256) = idx_1 (c)+ 1
TMP_13971(bool) = TMP_13970 < chkCount_1
TMP_13972(uint256) = idx_1 (c)+ 1
REF_4237(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[TMP_13972]
REF_4238(uint256) -> REF_4237.timestamp
TMP_13973(bool) = REF_4238 <= timestamp_1
TMP_13974(bool) = TMP_13971 && TMP_13973
CONDITION TMP_13974
 checkpoints[idx]
REF_4239(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
RETURN REF_4239
 effectiveCheckpoint.rate = $.rewardRates[token]
REF_4240(uint256) -> effectiveCheckpoint_0.rate
REF_4241(mapping(address => uint256)) -> $_1 (-> []).rewardRates
REF_4242(uint256) -> REF_4241[token_1]
effectiveCheckpoint_1(PlumeStakingStorage.RateCheckpoint) := phi(['effectiveCheckpoint_0'])
REF_4240(uint256) (->effectiveCheckpoint_1) := REF_4242(uint256)
 effectiveCheckpoint.timestamp = timestamp
REF_4243(uint256) -> effectiveCheckpoint_1.timestamp
effectiveCheckpoint_2(PlumeStakingStorage.RateCheckpoint) := phi(['effectiveCheckpoint_1'])
REF_4243(uint256) (->effectiveCheckpoint_2) := timestamp_1(uint256)
 effectiveCheckpoint.cumulativeIndex = 0
REF_4244(uint256) -> effectiveCheckpoint_2.cumulativeIndex
effectiveCheckpoint_3(PlumeStakingStorage.RateCheckpoint) := phi(['effectiveCheckpoint_2'])
REF_4244(uint256) (->effectiveCheckpoint_3) := 0(uint256)
 effectiveCheckpoint
RETURN effectiveCheckpoint_3
 effectiveCheckpoint
```
#### PlumeRewardLogic.getDistinctTimestamps(PlumeStakingStorage.Layout,uint16,address,uint256,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
validatorId_1(uint16) := phi(['validatorId_1'])
token_1(address) := phi(['token_1'])
periodStart_1(uint256) := phi(['lastUserRewardUpdateTime_5'])
periodEnd_1(uint256) := phi(['block.timestamp'])
 rewardCheckpoints = $.validatorRewardRateCheckpoints[validatorId][token]
REF_4212(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> []).validatorRewardRateCheckpoints
REF_4213(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4212[validatorId_1]
REF_4214(PlumeStakingStorage.RateCheckpoint[]) -> REF_4213[token_1]
rewardCheckpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4214(PlumeStakingStorage.RateCheckpoint[])']
 commissionCheckpoints = $.validatorCommissionCheckpoints[validatorId]
REF_4215(mapping(uint16 => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> []).validatorCommissionCheckpoints
REF_4216(PlumeStakingStorage.RateCheckpoint[]) -> REF_4215[validatorId_1]
commissionCheckpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4216(PlumeStakingStorage.RateCheckpoint[])']
 len1 = rewardCheckpoints.length
REF_4217 -> LENGTH rewardCheckpoints_1 (-> [])
len1_1(uint256) := REF_4217(uint256)
 len2 = commissionCheckpoints.length
REF_4218 -> LENGTH commissionCheckpoints_1 (-> [])
len2_1(uint256) := REF_4218(uint256)
 periodStart > periodEnd
TMP_13924(bool) = periodStart_1 > periodEnd_1
CONDITION TMP_13924
 new uint256[](0)
TMP_13926(uint256[])  = new uint256[](0)
RETURN TMP_13926
 periodStart == periodEnd
TMP_13927(bool) = periodStart_1 == periodEnd_1
CONDITION TMP_13927
 singlePoint = new uint256[](1)
TMP_13929(uint256[])  = new uint256[](1)
singlePoint_1(uint256[]) = ['TMP_13929(uint256[])']
 singlePoint[0] = periodStart
REF_4219(uint256) -> singlePoint_1[0]
singlePoint_2(uint256[]) := phi(['singlePoint_1'])
REF_4219(uint256) (->singlePoint_2) := periodStart_1(uint256)
 singlePoint
RETURN singlePoint_2
 result = new uint256[](len1 + len2 + 2)
TMP_13931(uint256) = len1_1 (c)+ len2_1
TMP_13932(uint256) = TMP_13931 (c)+ 2
TMP_13933(uint256[])  = new uint256[](TMP_13932)
result_1(uint256[]) = ['TMP_13933(uint256[])']
 i = 0
i_1(uint256) := 0(uint256)
 j = 0
j_1(uint256) := 0(uint256)
 k = 0
k_1(uint256) := 0(uint256)
 result[k ++] = periodStart
TMP_13934(uint256) := k_1(uint256)
k_2(uint256) = k_1 (c)+ 1
REF_4220(uint256) -> result_1[TMP_13934]
result_2(uint256[]) := phi(['result_1'])
REF_4220(uint256) (->result_2) := periodStart_1(uint256)
 lastAddedTimestamp = periodStart
lastAddedTimestamp_1(uint256) := periodStart_1(uint256)
 i < len1 && rewardCheckpoints[i].timestamp <= periodStart
i_2(uint256) := phi(['i_5', 'i_1'])
TMP_13935(bool) = i_2 < len1_1
REF_4221(PlumeStakingStorage.RateCheckpoint) -> rewardCheckpoints_1 (-> [])[i_2]
REF_4222(uint256) -> REF_4221.timestamp
TMP_13936(bool) = REF_4222 <= periodStart_1
TMP_13937(bool) = TMP_13935 && TMP_13936
CONDITION TMP_13937
 i ++
TMP_13938(uint256) := i_2(uint256)
i_5(uint256) = i_2 (c)+ 1
 j < len2 && commissionCheckpoints[j].timestamp <= periodStart
j_2(uint256) := phi(['j_3', 'j_1'])
TMP_13939(bool) = j_2 < len2_1
REF_4223(PlumeStakingStorage.RateCheckpoint) -> commissionCheckpoints_1 (-> [])[j_2]
REF_4224(uint256) -> REF_4223.timestamp
TMP_13940(bool) = REF_4224 <= periodStart_1
TMP_13941(bool) = TMP_13939 && TMP_13940
CONDITION TMP_13941
 j ++
TMP_13942(uint256) := j_2(uint256)
j_3(uint256) = j_2 (c)+ 1
 i < len1 || j < len2
TMP_13943(bool) = i_2 < len1_1
TMP_13944(bool) = j_2 < len2_1
TMP_13945(bool) = TMP_13943 || TMP_13944
CONDITION TMP_13945
 advanceI = false
advanceI_1(bool) := False(bool)
 advanceJ = false
advanceJ_1(bool) := False(bool)
 t1 < t2
TMP_13946(bool) = t1_3 < t2_3
CONDITION TMP_13946
 currentTimestampToAdd = t1
currentTimestampToAdd_1(uint256) := t1_3(uint256)
 advanceI = true
advanceI_2(bool) := True(bool)
 t2 < t1
TMP_13947(bool) = t2_3 < t1_3
CONDITION TMP_13947
 currentTimestampToAdd = t2
currentTimestampToAdd_3(uint256) := t2_3(uint256)
 advanceJ = true
advanceJ_3(bool) := True(bool)
 t1 != type()(uint256).max
TMP_13949(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_13950(bool) = t1_3 != TMP_13949
CONDITION TMP_13950
 currentTimestampToAdd = t1
currentTimestampToAdd_2(uint256) := t1_3(uint256)
 advanceI = true
advanceI_3(bool) := True(bool)
 advanceJ = true
advanceJ_2(bool) := True(bool)
currentTimestampToAdd_4(uint256) := phi(['currentTimestampToAdd_2', 'currentTimestampToAdd_3', 'currentTimestampToAdd_0'])
advanceI_4(bool) := phi(['advanceI_3', 'advanceI_1'])
advanceJ_4(bool) := phi(['advanceJ_1', 'advanceJ_3', 'advanceJ_2'])
currentTimestampToAdd_5(uint256) := phi(['currentTimestampToAdd_1', 'currentTimestampToAdd_0'])
advanceI_5(bool) := phi(['advanceI_1', 'advanceI_2'])
 currentTimestampToAdd >= periodEnd
TMP_13951(bool) = currentTimestampToAdd_5 >= periodEnd_1
CONDITION TMP_13951
 currentTimestampToAdd > lastAddedTimestamp
TMP_13952(bool) = currentTimestampToAdd_5 > lastAddedTimestamp_1
CONDITION TMP_13952
 result[k ++] = currentTimestampToAdd
TMP_13953(uint256) := k_2(uint256)
k_3(uint256) = k_2 (c)+ 1
REF_4225(uint256) -> result_2[TMP_13953]
result_3(uint256[]) := phi(['result_2'])
REF_4225(uint256) (->result_3) := currentTimestampToAdd_5(uint256)
 lastAddedTimestamp = currentTimestampToAdd
lastAddedTimestamp_2(uint256) := currentTimestampToAdd_5(uint256)
k_4(uint256) := phi(['k_2', 'k_3'])
lastAddedTimestamp_3(uint256) := phi(['lastAddedTimestamp_2', 'lastAddedTimestamp_1'])
 advanceI
CONDITION advanceI_5
 i ++
TMP_13954(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
i_4(uint256) := phi(['i_3', 'i_1'])
 advanceJ
CONDITION advanceJ_4
 j ++
TMP_13955(uint256) := j_2(uint256)
j_4(uint256) = j_2 (c)+ 1
j_5(uint256) := phi(['j_4', 'j_1'])
 lastAddedTimestamp < periodEnd
TMP_13956(bool) = lastAddedTimestamp_1 < periodEnd_1
CONDITION TMP_13956
 result[k ++] = periodEnd
TMP_13957(uint256) := k_2(uint256)
k_5(uint256) = k_2 (c)+ 1
REF_4226(uint256) -> result_2[TMP_13957]
result_4(uint256[]) := phi(['result_2'])
REF_4226(uint256) (->result_4) := periodEnd_1(uint256)
result_5(uint256[]) := phi(['result_4', 'result_2'])
k_6(uint256) := phi(['k_5', 'k_2'])
 mstore(uint256,uint256)(result,k)
TMP_13958(None) = SOLIDITY_CALL mstore(uint256,uint256)(result_5,k_6)
 result
RETURN result_5
 (i < len1)
TMP_13959(bool) = i_2 < len1_1
CONDITION TMP_13959
 t1 = rewardCheckpoints[i].timestamp
REF_4227(PlumeStakingStorage.RateCheckpoint) -> rewardCheckpoints_1 (-> [])[i_2]
REF_4228(uint256) -> REF_4227.timestamp
t1_1(uint256) := REF_4228(uint256)
 t1 = type()(uint256).max
TMP_13961(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
t1_2(uint256) := TMP_13961(uint256)
t1_3(uint256) := phi(['t1_1', 't1_2'])
 (j < len2)
TMP_13962(bool) = j_2 < len2_1
CONDITION TMP_13962
 t2 = commissionCheckpoints[j].timestamp
REF_4229(PlumeStakingStorage.RateCheckpoint) -> commissionCheckpoints_1 (-> [])[j_2]
REF_4230(uint256) -> REF_4229.timestamp
t2_1(uint256) := REF_4230(uint256)
 t2 = type()(uint256).max
TMP_13964(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
t2_2(uint256) := TMP_13964(uint256)
t2_3(uint256) := phi(['t2_1', 't2_2'])
```
#### PlumeRewardLogic._ceilDiv(uint256,uint256) [INTERNAL]
```slithir
a_1(uint256) := phi(['TMP_13916'])
b_1(uint256) := phi(['REF_4211'])
 b == 0
TMP_13920(bool) = b_1 == 0
CONDITION TMP_13920
 0
RETURN 0
 (a + b - 1) / b
TMP_13921(uint256) = a_1 (c)+ b_1
TMP_13922(uint256) = TMP_13921 (c)- 1
TMP_13923(uint256) = TMP_13922 (c)/ b_1
RETURN TMP_13923
 result
```
#### PlumeRewardLogic.findCommissionCheckpointIndexAtOrBefore(PlumeStakingStorage.Layout,uint16,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
validatorId_1(uint16) := phi(['validatorId_1'])
timestamp_1(uint256) := phi(['timestamp_1'])
 checkpoints = $.validatorCommissionCheckpoints[validatorId]
REF_4261(mapping(uint16 => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> []).validatorCommissionCheckpoints
REF_4262(PlumeStakingStorage.RateCheckpoint[]) -> REF_4261[validatorId_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4262(PlumeStakingStorage.RateCheckpoint[])']
 len = checkpoints.length
REF_4263 -> LENGTH checkpoints_1 (-> [])
len_1(uint256) := REF_4263(uint256)
 len == 0
TMP_13990(bool) = len_1 == 0
CONDITION TMP_13990
 0
RETURN 0
 low = 0
low_1(uint256) := 0(uint256)
 high = len - 1
TMP_13991(uint256) = len_1 (c)- 1
high_1(uint256) := TMP_13991(uint256)
 ans = 0
ans_1(uint256) := 0(uint256)
 foundSuitable = false
foundSuitable_1(bool) := False(bool)
 low <= high
TMP_13992(bool) = low_1 <= high_1
CONDITION TMP_13992
 mid = low + (high - low) / 2
TMP_13993(uint256) = high_1 (c)- low_1
TMP_13994(uint256) = TMP_13993 (c)/ 2
TMP_13995(uint256) = low_1 (c)+ TMP_13994
mid_1(uint256) := TMP_13995(uint256)
 checkpoints[mid].timestamp <= timestamp
REF_4264(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[mid_1]
REF_4265(uint256) -> REF_4264.timestamp
TMP_13996(bool) = REF_4265 <= timestamp_1
CONDITION TMP_13996
 ans = mid
ans_2(uint256) := mid_1(uint256)
 foundSuitable = true
foundSuitable_2(bool) := True(bool)
 low = mid + 1
TMP_13997(uint256) = mid_1 (c)+ 1
low_2(uint256) := TMP_13997(uint256)
 mid == 0
TMP_13998(bool) = mid_1 == 0
CONDITION TMP_13998
 high = mid - 1
TMP_13999(uint256) = mid_1 (c)- 1
high_2(uint256) := TMP_13999(uint256)
low_3(uint256) := phi(['low_1', 'low_2'])
high_3(uint256) := phi(['high_2', 'high_1'])
ans_3(uint256) := phi(['ans_1', 'ans_2'])
 ans
RETURN ans_1
```
#### PlumeRewardLogic.findRewardRateCheckpointIndexAtOrBefore(PlumeStakingStorage.Layout,uint16,address,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
validatorId_1(uint16) := phi(['validatorId_1'])
token_1(address) := phi(['token_1'])
timestamp_1(uint256) := phi(['timestamp_1'])
 checkpoints = $.validatorRewardRateCheckpoints[validatorId][token]
REF_4255(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> []).validatorRewardRateCheckpoints
REF_4256(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4255[validatorId_1]
REF_4257(PlumeStakingStorage.RateCheckpoint[]) -> REF_4256[token_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4257(PlumeStakingStorage.RateCheckpoint[])']
 len = checkpoints.length
REF_4258 -> LENGTH checkpoints_1 (-> [])
len_1(uint256) := REF_4258(uint256)
 len == 0
TMP_13980(bool) = len_1 == 0
CONDITION TMP_13980
 0
RETURN 0
 low = 0
low_1(uint256) := 0(uint256)
 high = len - 1
TMP_13981(uint256) = len_1 (c)- 1
high_1(uint256) := TMP_13981(uint256)
 ans = 0
ans_1(uint256) := 0(uint256)
 foundSuitable = false
foundSuitable_1(bool) := False(bool)
 low <= high
TMP_13982(bool) = low_1 <= high_1
CONDITION TMP_13982
 mid = low + (high - low) / 2
TMP_13983(uint256) = high_1 (c)- low_1
TMP_13984(uint256) = TMP_13983 (c)/ 2
TMP_13985(uint256) = low_1 (c)+ TMP_13984
mid_1(uint256) := TMP_13985(uint256)
 checkpoints[mid].timestamp <= timestamp
REF_4259(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[mid_1]
REF_4260(uint256) -> REF_4259.timestamp
TMP_13986(bool) = REF_4260 <= timestamp_1
CONDITION TMP_13986
 ans = mid
ans_2(uint256) := mid_1(uint256)
 foundSuitable = true
foundSuitable_2(bool) := True(bool)
 low = mid + 1
TMP_13987(uint256) = mid_1 (c)+ 1
low_2(uint256) := TMP_13987(uint256)
 mid == 0
TMP_13988(bool) = mid_1 == 0
CONDITION TMP_13988
 high = mid - 1
TMP_13989(uint256) = mid_1 (c)- 1
high_2(uint256) := TMP_13989(uint256)
low_3(uint256) := phi(['low_1', 'low_2'])
high_3(uint256) := phi(['high_1', 'high_2'])
ans_3(uint256) := phi(['ans_2', 'ans_1'])
 ans
RETURN ans_1
```
