





#### StakingFacet._checkValidatorSlashedAndRevert(uint16) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13406(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13406'])(PlumeStakingStorage.Layout) := TMP_13406(PlumeStakingStorage.Layout)
 $.validatorExists[validatorId] && $.validators[validatorId].slashed
REF_3392(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13406']).validatorExists
REF_3393(bool) -> REF_3392[validatorId_1]
REF_3394(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13406']).validators
REF_3395(PlumeStakingStorage.ValidatorInfo) -> REF_3394[validatorId_1]
REF_3396(bool) -> REF_3395.slashed
TMP_13407(bool) = REF_3393 && REF_3396
CONDITION TMP_13407
 revert ActionOnSlashedValidatorError(uint16)(validatorId)
TMP_13408(None) = SOLIDITY_CALL revert ActionOnSlashedValidatorError(uint16)(validatorId_1)
```
#### StakingFacet._validateValidatorForStaking(uint16) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13409(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13409'])(PlumeStakingStorage.Layout) := TMP_13409(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_3398(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13409']).validatorExists
REF_3399(bool) -> REF_3398[validatorId_1]
TMP_13410 = UnaryType.BANG REF_3399 
CONDITION TMP_13410
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13411(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 _checkValidatorSlashedAndRevert(validatorId)
INTERNAL_CALL, StakingFacet._checkValidatorSlashedAndRevert(uint16)(validatorId_1)
 ! $.validators[validatorId].active
REF_3400(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13409']).validators
REF_3401(PlumeStakingStorage.ValidatorInfo) -> REF_3400[validatorId_1]
REF_3402(bool) -> REF_3401.active
TMP_13413 = UnaryType.BANG REF_3402 
CONDITION TMP_13413
 revert ValidatorInactive(uint16)(validatorId)
TMP_13414(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(validatorId_1)
```
#### StakingFacet._validateStakeAmount(uint256) [INTERNAL]
```slithir
amount_1(uint256) := phi(['amount_1'])
 $ = PlumeStakingStorage.layout()
TMP_13415(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13415'])(PlumeStakingStorage.Layout) := TMP_13415(PlumeStakingStorage.Layout)
 amount == 0
TMP_13416(bool) = amount_1 == 0
CONDITION TMP_13416
 revert InvalidAmount(uint256)(0)
TMP_13417(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(0)
 amount < $.minStakeAmount
REF_3404(uint256) -> $_1 (-> ['TMP_13415']).minStakeAmount
TMP_13418(bool) = amount_1 < REF_3404
CONDITION TMP_13418
 revert StakeAmountTooSmall(uint256,uint256)(amount,$.minStakeAmount)
REF_3405(uint256) -> $_1 (-> ['TMP_13415']).minStakeAmount
TMP_13419(None) = SOLIDITY_CALL revert StakeAmountTooSmall(uint256,uint256)(amount_1,REF_3405)
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
TMP_13422(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13422'])(PlumeStakingStorage.Layout) := TMP_13422(PlumeStakingStorage.Layout)
 newDelegatedAmount = $.validators[validatorId].delegatedAmount
REF_3407(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13422']).validators
REF_3408(PlumeStakingStorage.ValidatorInfo) -> REF_3407[validatorId_1]
REF_3409(uint256) -> REF_3408.delegatedAmount
newDelegatedAmount_1(uint256) := REF_3409(uint256)
 maxCapacity = $.validators[validatorId].maxCapacity
REF_3410(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13422']).validators
REF_3411(PlumeStakingStorage.ValidatorInfo) -> REF_3410[validatorId_1]
REF_3412(uint256) -> REF_3411.maxCapacity
maxCapacity_1(uint256) := REF_3412(uint256)
 maxCapacity > 0 && newDelegatedAmount > maxCapacity
TMP_13423(bool) = maxCapacity_1 > 0
TMP_13424(bool) = newDelegatedAmount_1 > maxCapacity_1
TMP_13425(bool) = TMP_13423 && TMP_13424
CONDITION TMP_13425
 revert ExceedsValidatorCapacity(uint16,uint256,uint256,uint256)(validatorId,newDelegatedAmount,maxCapacity,stakeAmount)
TMP_13426(None) = SOLIDITY_CALL revert ExceedsValidatorCapacity(uint16,uint256,uint256,uint256)(validatorId_1,newDelegatedAmount_1,maxCapacity_1,stakeAmount_1)
```
#### StakingFacet._validateValidatorPercentage(uint16,uint256) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1'])
stakeAmount_1(uint256) := phi(['stakeAmount_1'])
 $ = PlumeStakingStorage.layout()
TMP_13427(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13427'])(PlumeStakingStorage.Layout) := TMP_13427(PlumeStakingStorage.Layout)
 previousTotalStaked = $.totalStaked - stakeAmount
REF_3414(uint256) -> $_1 (-> ['TMP_13427']).totalStaked
TMP_13428(uint256) = REF_3414 (c)- stakeAmount_1
previousTotalStaked_1(uint256) := TMP_13428(uint256)
 previousTotalStaked > 0 && $.maxValidatorPercentage > 0
TMP_13429(bool) = previousTotalStaked_1 > 0
REF_3415(uint256) -> $_1 (-> ['TMP_13427']).maxValidatorPercentage
TMP_13430(bool) = REF_3415 > 0
TMP_13431(bool) = TMP_13429 && TMP_13430
CONDITION TMP_13431
 newDelegatedAmount = $.validators[validatorId].delegatedAmount
REF_3416(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13427']).validators
REF_3417(PlumeStakingStorage.ValidatorInfo) -> REF_3416[validatorId_1]
REF_3418(uint256) -> REF_3417.delegatedAmount
newDelegatedAmount_1(uint256) := REF_3418(uint256)
 validatorPercentage = (newDelegatedAmount * 10_000) / $.totalStaked
TMP_13432(uint256) = newDelegatedAmount_1 (c)* 10000
REF_3419(uint256) -> $_1 (-> ['TMP_13427']).totalStaked
TMP_13433(uint256) = TMP_13432 (c)/ REF_3419
validatorPercentage_1(uint256) := TMP_13433(uint256)
 validatorPercentage > $.maxValidatorPercentage
REF_3420(uint256) -> $_1 (-> ['TMP_13427']).maxValidatorPercentage
TMP_13434(bool) = validatorPercentage_1 > REF_3420
CONDITION TMP_13434
 revert ValidatorPercentageExceeded()()
TMP_13435(None) = SOLIDITY_CALL revert ValidatorPercentageExceeded()()
```
#### StakingFacet._validateCapacityLimits(uint16,uint256) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1'])
stakeAmount_1(uint256) := phi(['stakeAmount_1'])
 _validateValidatorCapacity(validatorId,stakeAmount)
INTERNAL_CALL, StakingFacet._validateValidatorCapacity(uint16,uint256)(validatorId_1,stakeAmount_1)
 _validateValidatorPercentage(validatorId,stakeAmount)
INTERNAL_CALL, StakingFacet._validateValidatorPercentage(uint16,uint256)(validatorId_1,stakeAmount_1)
```
#### StakingFacet._validateValidatorForUnstaking(uint16) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13438(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13438'])(PlumeStakingStorage.Layout) := TMP_13438(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_3422(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13438']).validatorExists
REF_3423(bool) -> REF_3422[validatorId_1]
TMP_13439 = UnaryType.BANG REF_3423 
CONDITION TMP_13439
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13440(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 _checkValidatorSlashedAndRevert(validatorId)
INTERNAL_CALL, StakingFacet._checkValidatorSlashedAndRevert(uint16)(validatorId_1)
```
#### StakingFacet._performStakeSetup(address,uint16,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'user_1', 'staker_1', 'msg.sender'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1', 'validatorId_1', 'validatorId_1'])
stakeAmount_1(uint256) := phi(['stakeAmount_1', 'amount_1', 'stakeAmount_1', 'amountRestaked_1'])
 $ = PlumeStakingStorage.layout()
TMP_13442(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13442'])(PlumeStakingStorage.Layout) := TMP_13442(PlumeStakingStorage.Layout)
 _validateStaking(validatorId,stakeAmount)
INTERNAL_CALL, StakingFacet._validateStaking(uint16,uint256)(validatorId_1,stakeAmount_1)
 isNewStake = $.userValidatorStakes[user][validatorId].staked == 0
REF_3425(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13442']).userValidatorStakes
REF_3426(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3425[user_1]
REF_3427(PlumeStakingStorage.UserValidatorStake) -> REF_3426[validatorId_1]
REF_3428(uint256) -> REF_3427.staked
TMP_13444(bool) = REF_3428 == 0
isNewStake_1(bool) := TMP_13444(bool)
 ! isNewStake
TMP_13445 = UnaryType.BANG isNewStake_1 
CONDITION TMP_13445
 PlumeRewardLogic.updateRewardsForValidator($,user,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardsForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13442'])", 'user_1', 'validatorId_1'] 
 _initializeRewardStateForNewStake(user,validatorId)
INTERNAL_CALL, StakingFacet._initializeRewardStateForNewStake(address,uint16)(user_1,validatorId_1)
 _updateStakeAmounts(user,validatorId,stakeAmount)
INTERNAL_CALL, StakingFacet._updateStakeAmounts(address,uint16,uint256)(user_1,validatorId_1,stakeAmount_1)
 _validateCapacityLimits(validatorId,stakeAmount)
INTERNAL_CALL, StakingFacet._validateCapacityLimits(uint16,uint256)(validatorId_1,stakeAmount_1)
 PlumeValidatorLogic.addStakerToValidator($,user,validatorId)
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.addStakerToValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13442'])", 'user_1', 'validatorId_1'] 
 isNewStake
RETURN isNewStake_1
```
#### StakingFacet._performRestakeWorkflow(address,uint16,uint256,string) [INTERNAL]
```slithir
 _validateStaking(validatorId,amount)
INTERNAL_CALL, StakingFacet._validateStaking(uint16,uint256)(validatorId_1,amount_1)
 PlumeRewardLogic.updateRewardsForValidator(PlumeStakingStorage.layout(),user,validatorId)
TMP_13452(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardsForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:['TMP_13452', 'user_1', 'validatorId_1'] 
 _updateStakeAmounts(user,validatorId,amount)
INTERNAL_CALL, StakingFacet._updateStakeAmounts(address,uint16,uint256)(user_1,validatorId_1,amount_1)
 PlumeValidatorLogic.addStakerToValidator(PlumeStakingStorage.layout(),user,validatorId)
TMP_13455(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.addStakerToValidator(PlumeStakingStorage.Layout,address,uint16), arguments:['TMP_13455', 'user_1', 'validatorId_1']
```
#### StakingFacet.stake(uint16) [EXTERNAL]
```slithir
 stakeAmount = msg.value
stakeAmount_1(uint256) := msg.value(uint256)
 isNewStake = _performStakeSetup(msg.sender,validatorId,stakeAmount)
TMP_13457(bool) = INTERNAL_CALL, StakingFacet._performStakeSetup(address,uint16,uint256)(msg.sender,validatorId_1,stakeAmount_1)
isNewStake_1(bool) := TMP_13457(bool)
 Staked(msg.sender,validatorId,stakeAmount,0,0,stakeAmount)
Emit Staked(msg.sender,validatorId_1,stakeAmount_1,0,0,stakeAmount_1)
 stakeAmount
RETURN stakeAmount_1
```
#### StakingFacet.restake(uint16,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13459(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13459'])(PlumeStakingStorage.Layout) := TMP_13459(PlumeStakingStorage.Layout)
 user = msg.sender
user_1(address) := msg.sender(address)
 amount == 0
TMP_13460(bool) = amount_1 == 0
CONDITION TMP_13460
 revert InvalidAmount(uint256)(0)
TMP_13461(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(0)
 _processMaturedCooldowns(user)
TMP_13462(uint256) = INTERNAL_CALL, StakingFacet._processMaturedCooldowns(address)(user_1)
 parkedAmount = $.stakeInfo[user].parked
REF_3436(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13459']).stakeInfo
REF_3437(PlumeStakingStorage.StakeInfo) -> REF_3436[user_1]
REF_3438(uint256) -> REF_3437.parked
parkedAmount_1(uint256) := REF_3438(uint256)
 unmaturedCooldownFromTarget = $.userValidatorCooldowns[user][validatorId].amount
REF_3439(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13459']).userValidatorCooldowns
REF_3440(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3439[user_1]
REF_3441(PlumeStakingStorage.CooldownEntry) -> REF_3440[validatorId_1]
REF_3442(uint256) -> REF_3441.amount
unmaturedCooldownFromTarget_1(uint256) := REF_3442(uint256)
 totalAvailable = parkedAmount + unmaturedCooldownFromTarget
TMP_13463(uint256) = parkedAmount_1 (c)+ unmaturedCooldownFromTarget_1
totalAvailable_1(uint256) := TMP_13463(uint256)
 totalAvailable < amount
TMP_13464(bool) = totalAvailable_1 < amount_1
CONDITION TMP_13464
 revert InsufficientCooledAndParkedBalance(uint256,uint256)(totalAvailable,amount)
TMP_13465(None) = SOLIDITY_CALL revert InsufficientCooledAndParkedBalance(uint256,uint256)(totalAvailable_1,amount_1)
 _performStakeSetup(user,validatorId,amount)
TMP_13466(bool) = INTERNAL_CALL, StakingFacet._performStakeSetup(address,uint16,uint256)(user_1,validatorId_1,amount_1)
 fromCooled = 0
fromCooled_1(uint256) := 0(uint256)
 fromParked = 0
fromParked_1(uint256) := 0(uint256)
 remaining = amount
remaining_1(uint256) := amount_1(uint256)
 remaining > 0 && unmaturedCooldownFromTarget > 0
TMP_13467(bool) = remaining_1 > 0
TMP_13468(bool) = unmaturedCooldownFromTarget_1 > 0
TMP_13469(bool) = TMP_13467 && TMP_13468
CONDITION TMP_13469
 fromCooled = useAmount
fromCooled_2(uint256) := useAmount_3(uint256)
 remaining -= useAmount
remaining_2(uint256) = remaining_1 (c)- useAmount_3
 _removeCoolingAmounts(user,validatorId,useAmount)
INTERNAL_CALL, StakingFacet._removeCoolingAmounts(address,uint16,uint256)(user_1,validatorId_1,useAmount_3)
fromCooled_3(uint256) := phi(['fromCooled_2', 'fromCooled_1'])
remaining_3(uint256) := phi(['remaining_2', 'remaining_1'])
 remaining > 0
TMP_13471(bool) = remaining_3 > 0
CONDITION TMP_13471
 currentParked = $.stakeInfo[user].parked
REF_3443(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13459']).stakeInfo
REF_3444(PlumeStakingStorage.StakeInfo) -> REF_3443[user_1]
REF_3445(uint256) -> REF_3444.parked
currentParked_1(uint256) := REF_3445(uint256)
 remaining > currentParked
TMP_13472(bool) = remaining_3 > currentParked_1
CONDITION TMP_13472
 revert InternalInconsistency(string)(Insufficient parked funds for restake allocation)
TMP_13473(None) = SOLIDITY_CALL revert InternalInconsistency(string)(Insufficient parked funds for restake allocation)
 fromParked = remaining
fromParked_2(uint256) := remaining_3(uint256)
 _removeParkedAmounts(user,fromParked)
INTERNAL_CALL, StakingFacet._removeParkedAmounts(address,uint256)(user_1,fromParked_2)
fromParked_3(uint256) := phi(['fromParked_1', 'fromParked_2'])
 Staked(user,validatorId,amount,fromCooled,fromParked,0)
Emit Staked(user_1,validatorId_1,amount_1,fromCooled_3,fromParked_3,0)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
 remaining > unmaturedCooldownFromTarget
TMP_13477(bool) = remaining_1 > unmaturedCooldownFromTarget_1
CONDITION TMP_13477
 useAmount = unmaturedCooldownFromTarget
useAmount_1(uint256) := unmaturedCooldownFromTarget_1(uint256)
 useAmount = remaining
useAmount_2(uint256) := remaining_1(uint256)
useAmount_3(uint256) := phi(['useAmount_1', 'useAmount_2'])
```
#### StakingFacet.unstake(uint16,uint256) [EXTERNAL]
```slithir
 amount == 0
TMP_13482(bool) = amount_1 == 0
CONDITION TMP_13482
 revert InvalidAmount(uint256)(0)
TMP_13483(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(0)
 _unstake(validatorId,amount)
TMP_13484(uint256) = INTERNAL_CALL, StakingFacet._unstake(uint16,uint256)(validatorId_1,amount_1)
RETURN TMP_13484
 amountUnstaked
```
#### StakingFacet._unstake(uint16,uint256) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
amount_1(uint256) := phi(['REF_3451', 'amount_1'])
 $s = PlumeStakingStorage.layout()
TMP_13485(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$s_1 (-> ['TMP_13485'])(PlumeStakingStorage.Layout) := TMP_13485(PlumeStakingStorage.Layout)
 _validateValidatorForUnstaking(validatorId)
INTERNAL_CALL, StakingFacet._validateValidatorForUnstaking(uint16)(validatorId_1)
 amount == 0
TMP_13487(bool) = amount_1 == 0
CONDITION TMP_13487
 revert InvalidAmount(uint256)(amount)
TMP_13488(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(amount_1)
 $s.userValidatorStakes[msg.sender][validatorId].staked < amount
REF_3453(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $s_1 (-> ['TMP_13485']).userValidatorStakes
REF_3454(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3453[msg.sender]
REF_3455(PlumeStakingStorage.UserValidatorStake) -> REF_3454[validatorId_1]
REF_3456(uint256) -> REF_3455.staked
TMP_13489(bool) = REF_3456 < amount_1
CONDITION TMP_13489
 revert InsufficientFunds(uint256,uint256)($s.userValidatorStakes[msg.sender][validatorId].staked,amount)
REF_3457(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $s_1 (-> ['TMP_13485']).userValidatorStakes
REF_3458(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3457[msg.sender]
REF_3459(PlumeStakingStorage.UserValidatorStake) -> REF_3458[validatorId_1]
REF_3460(uint256) -> REF_3459.staked
TMP_13490(None) = SOLIDITY_CALL revert InsufficientFunds(uint256,uint256)(REF_3460,amount_1)
 PlumeRewardLogic.updateRewardsForValidator($s,msg.sender,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardsForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$s_1 (-> ['TMP_13485'])", 'msg.sender', 'validatorId_1'] 
 _updateUnstakeAmounts(msg.sender,validatorId,amount)
INTERNAL_CALL, StakingFacet._updateUnstakeAmounts(address,uint16,uint256)(msg.sender,validatorId_1,amount_1)
 newCooldownEndTimestamp = _processCooldownLogic(msg.sender,validatorId,amount)
TMP_13493(uint256) = INTERNAL_CALL, StakingFacet._processCooldownLogic(address,uint16,uint256)(msg.sender,validatorId_1,amount_1)
newCooldownEndTimestamp_1(uint256) := TMP_13493(uint256)
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
TMP_13496(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13496'])(PlumeStakingStorage.Layout) := TMP_13496(PlumeStakingStorage.Layout)
 user = msg.sender
user_1(address) := msg.sender(address)
 _processMaturedCooldowns(user)
TMP_13497(uint256) = INTERNAL_CALL, StakingFacet._processMaturedCooldowns(address)(user_1)
 amountToWithdraw = $.stakeInfo[user].parked
REF_3463(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13496']).stakeInfo
REF_3464(PlumeStakingStorage.StakeInfo) -> REF_3463[user_1]
REF_3465(uint256) -> REF_3464.parked
amountToWithdraw_1(uint256) := REF_3465(uint256)
 amountToWithdraw == 0
TMP_13498(bool) = amountToWithdraw_1 == 0
CONDITION TMP_13498
 revert InvalidAmount(uint256)(0)
TMP_13499(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(0)
 _removeParkedAmounts(user,amountToWithdraw)
INTERNAL_CALL, StakingFacet._removeParkedAmounts(address,uint256)(user_1,amountToWithdraw_1)
 _cleanupValidatorRelationships(user)
INTERNAL_CALL, StakingFacet._cleanupValidatorRelationships(address)(user_1)
 Withdrawn(user,amountToWithdraw)
Emit Withdrawn(user_1,amountToWithdraw_1)
 (success,None) = user.call{value: amountToWithdraw}()
TUPLE_91(bool,bytes) = LOW_LEVEL_CALL, dest:user_1, function:call, arguments:[''] value:amountToWithdraw_1 
success_1(bool)= UNPACK TUPLE_91 index: 0 
 ! success
TMP_13503 = UnaryType.BANG success_1 
CONDITION TMP_13503
 revert NativeTransferFailed()()
TMP_13504(None) = SOLIDITY_CALL revert NativeTransferFailed()()
```
#### StakingFacet.stakeOnBehalf(uint16,address) [EXTERNAL]
```slithir
 staker == address(0)
TMP_13505 = CONVERT 0 to address
TMP_13506(bool) = staker_1 == TMP_13505
CONDITION TMP_13506
 revert ZeroRecipientAddress()()
TMP_13507(None) = SOLIDITY_CALL revert ZeroRecipientAddress()()
 stakeAmount = msg.value
stakeAmount_1(uint256) := msg.value(uint256)
 isNewStake = _performStakeSetup(staker,validatorId,stakeAmount)
TMP_13508(bool) = INTERNAL_CALL, StakingFacet._performStakeSetup(address,uint16,uint256)(staker_1,validatorId_1,stakeAmount_1)
isNewStake_1(bool) := TMP_13508(bool)
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
TMP_13511(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13511'])(PlumeStakingStorage.Layout) := TMP_13511(PlumeStakingStorage.Layout)
 user = msg.sender
user_1(address) := msg.sender(address)
 _processMaturedCooldowns(user)
TMP_13512(uint256) = INTERNAL_CALL, StakingFacet._processMaturedCooldowns(address)(user_1)
 _validateValidatorForStaking(validatorId)
INTERNAL_CALL, StakingFacet._validateValidatorForStaking(uint16)(validatorId_1)
 tokenToRestake = PlumeStakingStorage.PLUME_NATIVE
REF_3468(address) -> PlumeStakingStorage.PLUME_NATIVE
tokenToRestake_1(address) := REF_3468(address)
 ! $.isRewardToken[tokenToRestake]
REF_3469(mapping(address => bool)) -> $_1 (-> ['TMP_13511']).isRewardToken
REF_3470(bool) -> REF_3469[tokenToRestake_1]
TMP_13514 = UnaryType.BANG REF_3470 
CONDITION TMP_13514
 revert TokenDoesNotExist(address)(tokenToRestake)
TMP_13515(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(tokenToRestake_1)
 amountRestaked = _calculateAndClaimAllRewardsWithCleanup(user,tokenToRestake)
TMP_13516(uint256) = INTERNAL_CALL, StakingFacet._calculateAndClaimAllRewardsWithCleanup(address,address)(user_1,tokenToRestake_1)
amountRestaked_1(uint256) := TMP_13516(uint256)
 amountRestaked == 0
TMP_13517(bool) = amountRestaked_1 == 0
CONDITION TMP_13517
 revert NoRewardsToRestake()()
TMP_13518(None) = SOLIDITY_CALL revert NoRewardsToRestake()()
 amountRestaked < $.minStakeAmount
REF_3471(uint256) -> $_1 (-> ['TMP_13511']).minStakeAmount
TMP_13519(bool) = amountRestaked_1 < REF_3471
CONDITION TMP_13519
 revert StakeAmountTooSmall(uint256,uint256)(amountRestaked,$.minStakeAmount)
REF_3472(uint256) -> $_1 (-> ['TMP_13511']).minStakeAmount
TMP_13520(None) = SOLIDITY_CALL revert StakeAmountTooSmall(uint256,uint256)(amountRestaked_1,REF_3472)
 _transferRewardFromTreasury(tokenToRestake,amountRestaked,address(this))
TMP_13521 = CONVERT this to address
INTERNAL_CALL, StakingFacet._transferRewardFromTreasury(address,uint256,address)(tokenToRestake_1,amountRestaked_1,TMP_13521)
 isNewStake = _performStakeSetup(user,validatorId,amountRestaked)
TMP_13523(bool) = INTERNAL_CALL, StakingFacet._performStakeSetup(address,uint16,uint256)(user_1,validatorId_1,amountRestaked_1)
isNewStake_1(bool) := TMP_13523(bool)
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
TMP_13527(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_3474(mapping(address => PlumeStakingStorage.StakeInfo)) -> TMP_13527.stakeInfo
REF_3475(PlumeStakingStorage.StakeInfo) -> REF_3474[msg.sender]
REF_3476(uint256) -> REF_3475.staked
RETURN REF_3476
 amount
```
#### StakingFacet.amountCooling() [EXTERNAL]
```slithir
 _calculateActivelyCoolingAmount(msg.sender)
TMP_13528(uint256) = INTERNAL_CALL, StakingFacet._calculateActivelyCoolingAmount(address)(msg.sender)
RETURN TMP_13528
 activelyCoolingAmount
```
#### StakingFacet.amountWithdrawable() [EXTERNAL]
```slithir
 _calculateTotalWithdrawableAmount(msg.sender)
TMP_13529(uint256) = INTERNAL_CALL, StakingFacet._calculateTotalWithdrawableAmount(address)(msg.sender)
RETURN TMP_13529
 totalWithdrawableAmount
```
#### StakingFacet.stakeInfo(address) [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().stakeInfo[user]
TMP_13530(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_3478(mapping(address => PlumeStakingStorage.StakeInfo)) -> TMP_13530.stakeInfo
REF_3479(PlumeStakingStorage.StakeInfo) -> REF_3478[user_1]
RETURN REF_3479
```
#### StakingFacet.totalAmountStaked() [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().totalStaked
TMP_13531(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_3481(uint256) -> TMP_13531.totalStaked
RETURN REF_3481
 amount
```
#### StakingFacet.totalAmountCooling() [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().totalCooling
TMP_13532(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_3483(uint256) -> TMP_13532.totalCooling
RETURN REF_3483
 amount
```
#### StakingFacet.totalAmountWithdrawable() [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().totalWithdrawable
TMP_13533(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_3485(uint256) -> TMP_13533.totalWithdrawable
RETURN REF_3485
 amount
```
#### StakingFacet.totalAmountClaimable(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13534(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13534'])(PlumeStakingStorage.Layout) := TMP_13534(PlumeStakingStorage.Layout)
 require(bool,string)($.isRewardToken[token],Token is not a reward token)
REF_3487(mapping(address => bool)) -> $_1 (-> ['TMP_13534']).isRewardToken
REF_3488(bool) -> REF_3487[token_1]
TMP_13535(None) = SOLIDITY_CALL require(bool,string)(REF_3488,Token is not a reward token)
 $.totalClaimableByToken[token]
REF_3489(mapping(address => uint256)) -> $_1 (-> ['TMP_13534']).totalClaimableByToken
REF_3490(uint256) -> REF_3489[token_1]
RETURN REF_3490
 amount
```
#### StakingFacet.getUserValidatorStake(address,uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13536(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13536'])(PlumeStakingStorage.Layout) := TMP_13536(PlumeStakingStorage.Layout)
 $.userValidatorStakes[user][validatorId].staked
REF_3492(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13536']).userValidatorStakes
REF_3493(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3492[user_1]
REF_3494(PlumeStakingStorage.UserValidatorStake) -> REF_3493[validatorId_1]
REF_3495(uint256) -> REF_3494.staked
RETURN REF_3495
```
#### StakingFacet.getUserCooldowns(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13537(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13537'])(PlumeStakingStorage.Layout) := TMP_13537(PlumeStakingStorage.Layout)
 userAssociatedValidators = $.userValidators[user]
REF_3497(mapping(address => uint16[])) -> $_1 (-> ['TMP_13537']).userValidators
REF_3498(uint16[]) -> REF_3497[user_1]
userAssociatedValidators_1 (-> [])(uint16[]) = ['REF_3498(uint16[])']
 activeCooldownCount = _countActiveCooldowns(user)
TMP_13538(uint256) = INTERNAL_CALL, StakingFacet._countActiveCooldowns(address)(user_1)
activeCooldownCount_1(uint256) := TMP_13538(uint256)
 cooldowns = new StakingFacet.CooldownView[](activeCooldownCount)
TMP_13540(StakingFacet.CooldownView[])  = new StakingFacet.CooldownView[](activeCooldownCount_1)
cooldowns_1(StakingFacet.CooldownView[]) = ['TMP_13540(StakingFacet.CooldownView[])']
 currentIndex = 0
currentIndex_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < userAssociatedValidators.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3499 -> LENGTH userAssociatedValidators_1 (-> [])
TMP_13541(bool) = i_2 < REF_3499
CONDITION TMP_13541
 validatorId_iterator = userAssociatedValidators[i]
REF_3500(uint16) -> userAssociatedValidators_1 (-> [])[i_2]
validatorId_iterator_1(uint16) := REF_3500(uint16)
 $.validatorExists[validatorId_iterator]
REF_3501(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13537']).validatorExists
REF_3502(bool) -> REF_3501[validatorId_iterator_1]
CONDITION REF_3502
 entry = $.userValidatorCooldowns[user][validatorId_iterator]
REF_3503(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13537']).userValidatorCooldowns
REF_3504(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3503[user_1]
REF_3505(PlumeStakingStorage.CooldownEntry) -> REF_3504[validatorId_iterator_1]
entry_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_3505(PlumeStakingStorage.CooldownEntry)
 entry.amount > 0
REF_3506(uint256) -> entry_1 (-> ['$']).amount
TMP_13542(bool) = REF_3506 > 0
CONDITION TMP_13542
 currentIndex < activeCooldownCount
TMP_13543(bool) = currentIndex_1 < activeCooldownCount_1
CONDITION TMP_13543
 cooldowns[currentIndex] = CooldownView({validatorId:validatorId_iterator,amount:entry.amount,cooldownEndTime:entry.cooldownEndTime})
REF_3507(StakingFacet.CooldownView) -> cooldowns_1[currentIndex_1]
REF_3508(uint256) -> entry_1 (-> ['$']).amount
REF_3509(uint256) -> entry_1 (-> ['$']).cooldownEndTime
TMP_13544(StakingFacet.CooldownView) = new CooldownView(validatorId_iterator_1,REF_3508,REF_3509)
cooldowns_2(StakingFacet.CooldownView[]) := phi(['cooldowns_1'])
REF_3507(StakingFacet.CooldownView) (->cooldowns_2) := TMP_13544(StakingFacet.CooldownView)
 currentIndex ++
TMP_13545(uint256) := currentIndex_1(uint256)
currentIndex_2(uint256) = currentIndex_1 (c)+ 1
cooldowns_3(StakingFacet.CooldownView[]) := phi(['cooldowns_2', 'cooldowns_1'])
currentIndex_3(uint256) := phi(['currentIndex_1', 'currentIndex_2'])
 i ++
TMP_13546(uint256) := i_2(uint256)
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
TMP_13547(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13547'])(PlumeStakingStorage.Layout) := TMP_13547(PlumeStakingStorage.Layout)
 $.userValidatorStakes[user][validatorId].staked += amount
REF_3511(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13547']).userValidatorStakes
REF_3512(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3511[user_1]
REF_3513(PlumeStakingStorage.UserValidatorStake) -> REF_3512[validatorId_1]
REF_3514(uint256) -> REF_3513.staked
$_2 (-> ['TMP_13547'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13547'])"])
REF_3514(-> $_2 (-> ['TMP_13547'])) = REF_3514 (c)+ amount_1
TMP_13547(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13547'])"])
 $.stakeInfo[user].staked += amount
REF_3515(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_2 (-> ['TMP_13547']).stakeInfo
REF_3516(PlumeStakingStorage.StakeInfo) -> REF_3515[user_1]
REF_3517(uint256) -> REF_3516.staked
$_3 (-> ['TMP_13547'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13547'])"])
REF_3517(-> $_3 (-> ['TMP_13547'])) = REF_3517 (c)+ amount_1
TMP_13547(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13547'])"])
 $.validators[validatorId].delegatedAmount += amount
REF_3518(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_3 (-> ['TMP_13547']).validators
REF_3519(PlumeStakingStorage.ValidatorInfo) -> REF_3518[validatorId_1]
REF_3520(uint256) -> REF_3519.delegatedAmount
$_4 (-> ['TMP_13547'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13547'])"])
REF_3520(-> $_4 (-> ['TMP_13547'])) = REF_3520 (c)+ amount_1
TMP_13547(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13547'])"])
 $.validatorTotalStaked[validatorId] += amount
REF_3521(mapping(uint16 => uint256)) -> $_4 (-> ['TMP_13547']).validatorTotalStaked
REF_3522(uint256) -> REF_3521[validatorId_1]
$_5 (-> ['TMP_13547'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13547'])"])
REF_3522(-> $_5 (-> ['TMP_13547'])) = REF_3522 (c)+ amount_1
TMP_13547(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13547'])"])
 $.totalStaked += amount
REF_3523(uint256) -> $_5 (-> ['TMP_13547']).totalStaked
$_6 (-> ['TMP_13547'])(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13547'])"])
REF_3523(-> $_6 (-> ['TMP_13547'])) = REF_3523 (c)+ amount_1
TMP_13547(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13547'])"])
```
#### StakingFacet._updateUnstakeAmounts(address,uint16,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender'])
validatorId_1(uint16) := phi(['validatorId_1'])
amount_1(uint256) := phi(['amount_1'])
 $ = PlumeStakingStorage.layout()
TMP_13548(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13548'])(PlumeStakingStorage.Layout) := TMP_13548(PlumeStakingStorage.Layout)
 $.userValidatorStakes[user][validatorId].staked -= amount
REF_3525(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13548']).userValidatorStakes
REF_3526(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3525[user_1]
REF_3527(PlumeStakingStorage.UserValidatorStake) -> REF_3526[validatorId_1]
REF_3528(uint256) -> REF_3527.staked
$_2 (-> ['TMP_13548'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13548'])"])
REF_3528(-> $_2 (-> ['TMP_13548'])) = REF_3528 (c)- amount_1
TMP_13548(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13548'])"])
 $.stakeInfo[user].staked -= amount
REF_3529(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_2 (-> ['TMP_13548']).stakeInfo
REF_3530(PlumeStakingStorage.StakeInfo) -> REF_3529[user_1]
REF_3531(uint256) -> REF_3530.staked
$_3 (-> ['TMP_13548'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13548'])"])
REF_3531(-> $_3 (-> ['TMP_13548'])) = REF_3531 (c)- amount_1
TMP_13548(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13548'])"])
 $.validators[validatorId].delegatedAmount -= amount
REF_3532(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_3 (-> ['TMP_13548']).validators
REF_3533(PlumeStakingStorage.ValidatorInfo) -> REF_3532[validatorId_1]
REF_3534(uint256) -> REF_3533.delegatedAmount
$_4 (-> ['TMP_13548'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13548'])"])
REF_3534(-> $_4 (-> ['TMP_13548'])) = REF_3534 (c)- amount_1
TMP_13548(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13548'])"])
 $.validatorTotalStaked[validatorId] -= amount
REF_3535(mapping(uint16 => uint256)) -> $_4 (-> ['TMP_13548']).validatorTotalStaked
REF_3536(uint256) -> REF_3535[validatorId_1]
$_5 (-> ['TMP_13548'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13548'])"])
REF_3536(-> $_5 (-> ['TMP_13548'])) = REF_3536 (c)- amount_1
TMP_13548(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13548'])"])
 $.totalStaked -= amount
REF_3537(uint256) -> $_5 (-> ['TMP_13548']).totalStaked
$_6 (-> ['TMP_13548'])(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13548'])"])
REF_3537(-> $_6 (-> ['TMP_13548'])) = REF_3537 (c)- amount_1
TMP_13548(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13548'])"])
```
#### StakingFacet._updateCoolingAmounts(address,uint16,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
amount_1(uint256) := phi(['amount_1'])
 $ = PlumeStakingStorage.layout()
TMP_13549(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13549'])(PlumeStakingStorage.Layout) := TMP_13549(PlumeStakingStorage.Layout)
 $.stakeInfo[user].cooled += amount
REF_3539(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13549']).stakeInfo
REF_3540(PlumeStakingStorage.StakeInfo) -> REF_3539[user_1]
REF_3541(uint256) -> REF_3540.cooled
$_2 (-> ['TMP_13549'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13549'])"])
REF_3541(-> $_2 (-> ['TMP_13549'])) = REF_3541 (c)+ amount_1
TMP_13549(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13549'])"])
 $.totalCooling += amount
REF_3542(uint256) -> $_2 (-> ['TMP_13549']).totalCooling
$_3 (-> ['TMP_13549'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13549'])"])
REF_3542(-> $_3 (-> ['TMP_13549'])) = REF_3542 (c)+ amount_1
TMP_13549(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13549'])"])
 $.validatorTotalCooling[validatorId] += amount
REF_3543(mapping(uint16 => uint256)) -> $_3 (-> ['TMP_13549']).validatorTotalCooling
REF_3544(uint256) -> REF_3543[validatorId_1]
$_4 (-> ['TMP_13549'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13549'])"])
REF_3544(-> $_4 (-> ['TMP_13549'])) = REF_3544 (c)+ amount_1
TMP_13549(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13549'])"])
```
#### StakingFacet._removeCoolingAmounts(address,uint16,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'user_1', 'user_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1', 'validatorId_1'])
amount_1(uint256) := phi(['amountInThisCooldown_1', 'useAmount_3', 'currentCooledAmountInSlot_1'])
 $ = PlumeStakingStorage.layout()
TMP_13550(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13550'])(PlumeStakingStorage.Layout) := TMP_13550(PlumeStakingStorage.Layout)
 isSlashed = $.validators[validatorId].slashed
REF_3546(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13550']).validators
REF_3547(PlumeStakingStorage.ValidatorInfo) -> REF_3546[validatorId_1]
REF_3548(bool) -> REF_3547.slashed
isSlashed_1(bool) := REF_3548(bool)
 $.stakeInfo[user].cooled >= amount
REF_3549(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13550']).stakeInfo
REF_3550(PlumeStakingStorage.StakeInfo) -> REF_3549[user_1]
REF_3551(uint256) -> REF_3550.cooled
TMP_13551(bool) = REF_3551 >= amount_1
CONDITION TMP_13551
 $.stakeInfo[user].cooled -= amount
REF_3552(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13550']).stakeInfo
REF_3553(PlumeStakingStorage.StakeInfo) -> REF_3552[user_1]
REF_3554(uint256) -> REF_3553.cooled
$_3 (-> ['TMP_13550'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13550'])"])
REF_3554(-> $_3 (-> ['TMP_13550'])) = REF_3554 (c)- amount_1
TMP_13550(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13550'])"])
 $.stakeInfo[user].cooled = 0
REF_3555(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13550']).stakeInfo
REF_3556(PlumeStakingStorage.StakeInfo) -> REF_3555[user_1]
REF_3557(uint256) -> REF_3556.cooled
$_2 (-> ['TMP_13550'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13550'])"])
REF_3557(uint256) (->$_2 (-> ['TMP_13550'])) := 0(uint256)
TMP_13550(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13550'])"])
$_4 (-> ['TMP_13550'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13550'])", "$_3 (-> ['TMP_13550'])"])
 ! isSlashed
TMP_13552 = UnaryType.BANG isSlashed_1 
CONDITION TMP_13552
 $.totalCooling >= amount
REF_3558(uint256) -> $_4 (-> ['TMP_13550']).totalCooling
TMP_13553(bool) = REF_3558 >= amount_1
CONDITION TMP_13553
 $.totalCooling -= amount
REF_3559(uint256) -> $_4 (-> ['TMP_13550']).totalCooling
$_6 (-> ['TMP_13550'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13550'])"])
REF_3559(-> $_6 (-> ['TMP_13550'])) = REF_3559 (c)- amount_1
TMP_13550(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13550'])"])
 $.totalCooling = 0
REF_3560(uint256) -> $_4 (-> ['TMP_13550']).totalCooling
$_5 (-> ['TMP_13550'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13550'])"])
REF_3560(uint256) (->$_5 (-> ['TMP_13550'])) := 0(uint256)
TMP_13550(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13550'])"])
$_7 (-> ['TMP_13550'])(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13550'])", "$_5 (-> ['TMP_13550'])"])
 $.validatorTotalCooling[validatorId] >= amount
REF_3561(mapping(uint16 => uint256)) -> $_7 (-> ['TMP_13550']).validatorTotalCooling
REF_3562(uint256) -> REF_3561[validatorId_1]
TMP_13554(bool) = REF_3562 >= amount_1
CONDITION TMP_13554
 $.validatorTotalCooling[validatorId] -= amount
REF_3563(mapping(uint16 => uint256)) -> $_7 (-> ['TMP_13550']).validatorTotalCooling
REF_3564(uint256) -> REF_3563[validatorId_1]
$_8 (-> ['TMP_13550'])(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_13550'])"])
REF_3564(-> $_8 (-> ['TMP_13550'])) = REF_3564 (c)- amount_1
TMP_13550(PlumeStakingStorage.Layout) := phi(["$_8 (-> ['TMP_13550'])"])
 $.validatorTotalCooling[validatorId] = 0
REF_3565(mapping(uint16 => uint256)) -> $_7 (-> ['TMP_13550']).validatorTotalCooling
REF_3566(uint256) -> REF_3565[validatorId_1]
$_9 (-> ['TMP_13550'])(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_13550'])"])
REF_3566(uint256) (->$_9 (-> ['TMP_13550'])) := 0(uint256)
TMP_13550(PlumeStakingStorage.Layout) := phi(["$_9 (-> ['TMP_13550'])"])
$_10 (-> ['TMP_13550'])(PlumeStakingStorage.Layout) := phi(["$_8 (-> ['TMP_13550'])", "$_9 (-> ['TMP_13550'])"])
 entry = $.userValidatorCooldowns[user][validatorId]
REF_3567(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_10 (-> ['TMP_13550']).userValidatorCooldowns
REF_3568(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3567[user_1]
REF_3569(PlumeStakingStorage.CooldownEntry) -> REF_3568[validatorId_1]
entry_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_3569(PlumeStakingStorage.CooldownEntry)
 entry.amount >= amount
REF_3570(uint256) -> entry_1 (-> ['$']).amount
TMP_13555(bool) = REF_3570 >= amount_1
CONDITION TMP_13555
 entry.amount -= amount
REF_3571(uint256) -> entry_1 (-> ['$']).amount
entry_2 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := phi(["entry_1 (-> ['$'])"])
REF_3571(-> entry_2 (-> ['$'])) = REF_3571 (c)- amount_1
$_11 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["entry_2 (-> ['$'])"])
 entry.amount == 0
REF_3572(uint256) -> entry_2 (-> ['$']).amount
TMP_13556(bool) = REF_3572 == 0
CONDITION TMP_13556
 entry.cooldownEndTime = 0
REF_3573(uint256) -> entry_2 (-> ['$']).cooldownEndTime
entry_3 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := phi(["entry_2 (-> ['$'])"])
REF_3573(uint256) (->entry_3 (-> ['$'])) := 0(uint256)
$_12 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["entry_3 (-> ['$'])"])
 entry.amount = 0
REF_3574(uint256) -> entry_1 (-> ['$']).amount
entry_4 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := phi(["entry_1 (-> ['$'])"])
REF_3574(uint256) (->entry_4 (-> ['$'])) := 0(uint256)
$_13 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["entry_4 (-> ['$'])"])
 entry.cooldownEndTime = 0
REF_3575(uint256) -> entry_4 (-> ['$']).cooldownEndTime
entry_5 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := phi(["entry_4 (-> ['$'])"])
REF_3575(uint256) (->entry_5 (-> ['$'])) := 0(uint256)
$_14 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["entry_5 (-> ['$'])"])
```
#### StakingFacet._updateParkedAmounts(address,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'user_1'])
amount_1(uint256) := phi(['amountMovedToParked_1', 'currentCooledAmountInSlot_1'])
 $ = PlumeStakingStorage.layout()
TMP_13557(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13557'])(PlumeStakingStorage.Layout) := TMP_13557(PlumeStakingStorage.Layout)
 $.stakeInfo[user].parked += amount
REF_3577(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13557']).stakeInfo
REF_3578(PlumeStakingStorage.StakeInfo) -> REF_3577[user_1]
REF_3579(uint256) -> REF_3578.parked
$_2 (-> ['TMP_13557'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13557'])"])
REF_3579(-> $_2 (-> ['TMP_13557'])) = REF_3579 (c)+ amount_1
TMP_13557(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13557'])"])
 $.totalWithdrawable += amount
REF_3580(uint256) -> $_2 (-> ['TMP_13557']).totalWithdrawable
$_3 (-> ['TMP_13557'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13557'])"])
REF_3580(-> $_3 (-> ['TMP_13557'])) = REF_3580 (c)+ amount_1
TMP_13557(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13557'])"])
```
#### StakingFacet._updateWithdrawalAmounts(address,uint256) [INTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13558(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13558'])(PlumeStakingStorage.Layout) := TMP_13558(PlumeStakingStorage.Layout)
 $.stakeInfo[user].parked = 0
REF_3582(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13558']).stakeInfo
REF_3583(PlumeStakingStorage.StakeInfo) -> REF_3582[user_1]
REF_3584(uint256) -> REF_3583.parked
$_2 (-> ['TMP_13558'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13558'])"])
REF_3584(uint256) (->$_2 (-> ['TMP_13558'])) := 0(uint256)
TMP_13558(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13558'])"])
 $.totalWithdrawable >= amount
REF_3585(uint256) -> $_2 (-> ['TMP_13558']).totalWithdrawable
TMP_13559(bool) = REF_3585 >= amount_1
CONDITION TMP_13559
 $.totalWithdrawable -= amount
REF_3586(uint256) -> $_2 (-> ['TMP_13558']).totalWithdrawable
$_3 (-> ['TMP_13558'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13558'])"])
REF_3586(-> $_3 (-> ['TMP_13558'])) = REF_3586 (c)- amount_1
TMP_13558(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13558'])"])
 $.totalWithdrawable = 0
REF_3587(uint256) -> $_2 (-> ['TMP_13558']).totalWithdrawable
$_4 (-> ['TMP_13558'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13558'])"])
REF_3587(uint256) (->$_4 (-> ['TMP_13558'])) := 0(uint256)
TMP_13558(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13558'])"])
```
#### StakingFacet._updateRewardClaim(address,uint16,address,uint256) [INTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13560(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13560'])(PlumeStakingStorage.Layout) := TMP_13560(PlumeStakingStorage.Layout)
 $.userRewards[user][validatorId][token] = 0
REF_3589(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13560']).userRewards
REF_3590(mapping(uint16 => mapping(address => uint256))) -> REF_3589[user_1]
REF_3591(mapping(address => uint256)) -> REF_3590[validatorId_1]
REF_3592(uint256) -> REF_3591[token_1]
$_2 (-> ['TMP_13560'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13560'])"])
REF_3592(uint256) (->$_2 (-> ['TMP_13560'])) := 0(uint256)
TMP_13560(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13560'])"])
 $.userValidatorRewardPerTokenPaidTimestamp[user][validatorId][token] = block.timestamp
REF_3593(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_2 (-> ['TMP_13560']).userValidatorRewardPerTokenPaidTimestamp
REF_3594(mapping(uint16 => mapping(address => uint256))) -> REF_3593[user_1]
REF_3595(mapping(address => uint256)) -> REF_3594[validatorId_1]
REF_3596(uint256) -> REF_3595[token_1]
$_3 (-> ['TMP_13560'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13560'])"])
REF_3596(uint256) (->$_3 (-> ['TMP_13560'])) := block.timestamp(uint256)
TMP_13560(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13560'])"])
 $.userValidatorRewardPerTokenPaid[user][validatorId][token] = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_3597(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_3 (-> ['TMP_13560']).userValidatorRewardPerTokenPaid
REF_3598(mapping(uint16 => mapping(address => uint256))) -> REF_3597[user_1]
REF_3599(mapping(address => uint256)) -> REF_3598[validatorId_1]
REF_3600(uint256) -> REF_3599[token_1]
REF_3601(mapping(uint16 => mapping(address => uint256))) -> $_3 (-> ['TMP_13560']).validatorRewardPerTokenCumulative
REF_3602(mapping(address => uint256)) -> REF_3601[validatorId_1]
REF_3603(uint256) -> REF_3602[token_1]
$_4 (-> ['TMP_13560'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13560'])"])
REF_3600(uint256) (->$_4 (-> ['TMP_13560'])) := REF_3603(uint256)
TMP_13560(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13560'])"])
 $.totalClaimableByToken[token] >= amount
REF_3604(mapping(address => uint256)) -> $_4 (-> ['TMP_13560']).totalClaimableByToken
REF_3605(uint256) -> REF_3604[token_1]
TMP_13561(bool) = REF_3605 >= amount_1
CONDITION TMP_13561
 $.totalClaimableByToken[token] -= amount
REF_3606(mapping(address => uint256)) -> $_4 (-> ['TMP_13560']).totalClaimableByToken
REF_3607(uint256) -> REF_3606[token_1]
$_5 (-> ['TMP_13560'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13560'])"])
REF_3607(-> $_5 (-> ['TMP_13560'])) = REF_3607 (c)- amount_1
TMP_13560(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13560'])"])
 $.totalClaimableByToken[token] = 0
REF_3608(mapping(address => uint256)) -> $_4 (-> ['TMP_13560']).totalClaimableByToken
REF_3609(uint256) -> REF_3608[token_1]
$_6 (-> ['TMP_13560'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13560'])"])
REF_3609(uint256) (->$_6 (-> ['TMP_13560'])) := 0(uint256)
TMP_13560(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13560'])"])
```
#### StakingFacet._updateCommissionClaim(uint16,address,uint256,address) [INTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13562(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13562'])(PlumeStakingStorage.Layout) := TMP_13562(PlumeStakingStorage.Layout)
 $.pendingCommissionClaims[validatorId][token] = PlumeStakingStorage.PendingCommissionClaim({amount:amount,requestTimestamp:block.timestamp,token:token,recipient:recipient})
REF_3611(mapping(uint16 => mapping(address => PlumeStakingStorage.PendingCommissionClaim))) -> $_1 (-> ['TMP_13562']).pendingCommissionClaims
REF_3612(mapping(address => PlumeStakingStorage.PendingCommissionClaim)) -> REF_3611[validatorId_1]
REF_3613(PlumeStakingStorage.PendingCommissionClaim) -> REF_3612[token_1]
TMP_13563(PlumeStakingStorage.PendingCommissionClaim) = new PendingCommissionClaim(amount_1,block.timestamp,token_1,recipient_1)
$_2 (-> ['TMP_13562'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13562'])"])
REF_3613(PlumeStakingStorage.PendingCommissionClaim) (->$_2 (-> ['TMP_13562'])) := TMP_13563(PlumeStakingStorage.PendingCommissionClaim)
TMP_13562(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13562'])"])
 $.validatorAccruedCommission[validatorId][token] = 0
REF_3615(mapping(uint16 => mapping(address => uint256))) -> $_2 (-> ['TMP_13562']).validatorAccruedCommission
REF_3616(mapping(address => uint256)) -> REF_3615[validatorId_1]
REF_3617(uint256) -> REF_3616[token_1]
$_3 (-> ['TMP_13562'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13562'])"])
REF_3617(uint256) (->$_3 (-> ['TMP_13562'])) := 0(uint256)
TMP_13562(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13562'])"])
```
#### StakingFacet._removeParkedAmounts(address,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'user_1'])
amount_1(uint256) := phi(['fromParked_2', 'amountToWithdraw_1'])
 $ = PlumeStakingStorage.layout()
TMP_13564(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13564'])(PlumeStakingStorage.Layout) := TMP_13564(PlumeStakingStorage.Layout)
 $.stakeInfo[user].parked -= amount
REF_3619(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13564']).stakeInfo
REF_3620(PlumeStakingStorage.StakeInfo) -> REF_3619[user_1]
REF_3621(uint256) -> REF_3620.parked
$_2 (-> ['TMP_13564'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13564'])"])
REF_3621(-> $_2 (-> ['TMP_13564'])) = REF_3621 (c)- amount_1
TMP_13564(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13564'])"])
 $.totalWithdrawable -= amount
REF_3622(uint256) -> $_2 (-> ['TMP_13564']).totalWithdrawable
$_3 (-> ['TMP_13564'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13564'])"])
REF_3622(-> $_3 (-> ['TMP_13564'])) = REF_3622 (c)- amount_1
TMP_13564(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13564'])"])
```
#### StakingFacet._processCooldownLogic(address,uint16,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender'])
validatorId_1(uint16) := phi(['validatorId_1'])
amount_1(uint256) := phi(['amount_1'])
 $ = PlumeStakingStorage.layout()
TMP_13565(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13565'])(PlumeStakingStorage.Layout) := TMP_13565(PlumeStakingStorage.Layout)
 cooldownEntrySlot = $.userValidatorCooldowns[user][validatorId]
REF_3624(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13565']).userValidatorCooldowns
REF_3625(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3624[user_1]
REF_3626(PlumeStakingStorage.CooldownEntry) -> REF_3625[validatorId_1]
cooldownEntrySlot_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_3626(PlumeStakingStorage.CooldownEntry)
 currentCooledAmountInSlot = cooldownEntrySlot.amount
REF_3627(uint256) -> cooldownEntrySlot_1 (-> ['$']).amount
currentCooledAmountInSlot_1(uint256) := REF_3627(uint256)
 currentCooldownEndTimeInSlot = cooldownEntrySlot.cooldownEndTime
REF_3628(uint256) -> cooldownEntrySlot_1 (-> ['$']).cooldownEndTime
currentCooldownEndTimeInSlot_1(uint256) := REF_3628(uint256)
 newCooldownEndTime = block.timestamp + $.cooldownInterval
REF_3629(uint256) -> $_1 (-> ['TMP_13565']).cooldownInterval
TMP_13566(uint256) = block.timestamp (c)+ REF_3629
newCooldownEndTime_1(uint256) := TMP_13566(uint256)
 currentCooledAmountInSlot > 0 && block.timestamp >= currentCooldownEndTimeInSlot
TMP_13567(bool) = currentCooledAmountInSlot_1 > 0
TMP_13568(bool) = block.timestamp >= currentCooldownEndTimeInSlot_1
TMP_13569(bool) = TMP_13567 && TMP_13568
CONDITION TMP_13569
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
TMP_13574(uint256) = currentCooledAmountInSlot_1 (c)+ amount_1
finalNewCooledAmountForSlot_2(uint256) := TMP_13574(uint256)
finalNewCooledAmountForSlot_3(uint256) := phi(['finalNewCooledAmountForSlot_2', 'finalNewCooledAmountForSlot_1'])
 cooldownEntrySlot.amount = finalNewCooledAmountForSlot
REF_3630(uint256) -> cooldownEntrySlot_1 (-> ['$']).amount
cooldownEntrySlot_2 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := phi(["cooldownEntrySlot_1 (-> ['$'])"])
REF_3630(uint256) (->cooldownEntrySlot_2 (-> ['$'])) := finalNewCooledAmountForSlot_3(uint256)
$_2 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["cooldownEntrySlot_2 (-> ['$'])"])
 cooldownEntrySlot.cooldownEndTime = newCooldownEndTime
REF_3631(uint256) -> cooldownEntrySlot_2 (-> ['$']).cooldownEndTime
cooldownEntrySlot_3 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := phi(["cooldownEntrySlot_2 (-> ['$'])"])
REF_3631(uint256) (->cooldownEntrySlot_3 (-> ['$'])) := newCooldownEndTime_1(uint256)
$_3 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["cooldownEntrySlot_3 (-> ['$'])"])
 newCooldownEndTime
RETURN newCooldownEndTime_1
 newCooldownEndTime
```
#### StakingFacet._processMaturedCooldowns(address) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'user_1', 'user_1'])
 $ = PlumeStakingStorage.layout()
TMP_13575(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13575'])(PlumeStakingStorage.Layout) := TMP_13575(PlumeStakingStorage.Layout)
 amountMovedToParked = 0
amountMovedToParked_1(uint256) := 0(uint256)
 userAssociatedValidators = $.userValidators[user]
REF_3633(mapping(address => uint16[])) -> $_1 (-> ['TMP_13575']).userValidators
REF_3634(uint16[]) -> REF_3633[user_1]
userAssociatedValidators_1(uint16[]) = ['REF_3634(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < userAssociatedValidators.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3635 -> LENGTH userAssociatedValidators_1
TMP_13576(bool) = i_2 < REF_3635
CONDITION TMP_13576
 validatorId = userAssociatedValidators[i]
REF_3636(uint16) -> userAssociatedValidators_1[i_2]
validatorId_1(uint16) := REF_3636(uint16)
 cooldownEntry = $.userValidatorCooldowns[user][validatorId]
REF_3637(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13575']).userValidatorCooldowns
REF_3638(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3637[user_1]
REF_3639(PlumeStakingStorage.CooldownEntry) -> REF_3638[validatorId_1]
cooldownEntry_1(PlumeStakingStorage.CooldownEntry) := REF_3639(PlumeStakingStorage.CooldownEntry)
 cooldownEntry.amount == 0
REF_3640(uint256) -> cooldownEntry_1.amount
TMP_13577(bool) = REF_3640 == 0
CONDITION TMP_13577
 canRecoverFromThisCooldown = _canRecoverFromCooldown(user,validatorId,cooldownEntry)
TMP_13578(bool) = INTERNAL_CALL, StakingFacet._canRecoverFromCooldown(address,uint16,PlumeStakingStorage.CooldownEntry)(user_1,validatorId_1,cooldownEntry_1)
canRecoverFromThisCooldown_1(bool) := TMP_13578(bool)
 canRecoverFromThisCooldown
CONDITION canRecoverFromThisCooldown_1
 amountInThisCooldown = cooldownEntry.amount
REF_3641(uint256) -> cooldownEntry_1.amount
amountInThisCooldown_1(uint256) := REF_3641(uint256)
 amountMovedToParked += amountInThisCooldown
amountMovedToParked_2(uint256) = amountMovedToParked_1 (c)+ amountInThisCooldown_1
 _removeCoolingAmounts(user,validatorId,amountInThisCooldown)
INTERNAL_CALL, StakingFacet._removeCoolingAmounts(address,uint16,uint256)(user_1,validatorId_1,amountInThisCooldown_1)
 delete $.userValidatorCooldowns[user][validatorId]
REF_3642(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13575']).userValidatorCooldowns
REF_3643(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3642[user_1]
REF_3644(PlumeStakingStorage.CooldownEntry) -> REF_3643[validatorId_1]
REF_3643 = delete REF_3644 
 $.userValidatorStakes[user][validatorId].staked == 0
REF_3645(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13575']).userValidatorStakes
REF_3646(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3645[user_1]
REF_3647(PlumeStakingStorage.UserValidatorStake) -> REF_3646[validatorId_1]
REF_3648(uint256) -> REF_3647.staked
TMP_13580(bool) = REF_3648 == 0
CONDITION TMP_13580
 PlumeValidatorLogic.removeStakerFromValidator($,user,validatorId)
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.removeStakerFromValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13575'])", 'user_1', 'validatorId_1'] 
amountMovedToParked_3(uint256) := phi(['amountMovedToParked_2', 'amountMovedToParked_1'])
$_2 (-> ['TMP_13575'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13575'])", "$_1 (-> ['TMP_13575'])"])
 i ++
TMP_13582(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 amountMovedToParked > 0
TMP_13583(bool) = amountMovedToParked_1 > 0
CONDITION TMP_13583
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
cooldownEntry_1(PlumeStakingStorage.CooldownEntry) := phi(['cooldownEntry_1'])
 $ = PlumeStakingStorage.layout()
TMP_13585(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13585'])(PlumeStakingStorage.Layout) := TMP_13585(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_3651(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13585']).validatorExists
REF_3652(bool) -> REF_3651[validatorId_1]
TMP_13586 = UnaryType.BANG REF_3652 
CONDITION TMP_13586
 false
RETURN False
 $.validators[validatorId].slashed
REF_3653(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13585']).validators
REF_3654(PlumeStakingStorage.ValidatorInfo) -> REF_3653[validatorId_1]
REF_3655(bool) -> REF_3654.slashed
CONDITION REF_3655
 slashTs = $.validators[validatorId].slashedAtTimestamp
REF_3656(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13585']).validators
REF_3657(PlumeStakingStorage.ValidatorInfo) -> REF_3656[validatorId_1]
REF_3658(uint256) -> REF_3657.slashedAtTimestamp
slashTs_1(uint256) := REF_3658(uint256)
 (cooldownEntry.cooldownEndTime < slashTs && block.timestamp >= cooldownEntry.cooldownEndTime)
REF_3659(uint256) -> cooldownEntry_1.cooldownEndTime
TMP_13587(bool) = REF_3659 < slashTs_1
REF_3660(uint256) -> cooldownEntry_1.cooldownEndTime
TMP_13588(bool) = block.timestamp >= REF_3660
TMP_13589(bool) = TMP_13587 && TMP_13588
RETURN TMP_13589
 (block.timestamp >= cooldownEntry.cooldownEndTime)
REF_3661(uint256) -> cooldownEntry_1.cooldownEndTime
TMP_13590(bool) = block.timestamp >= REF_3661
RETURN TMP_13590
 canRecover
```
#### StakingFacet._initializeRewardStateForNewStake(address,uint16) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13591(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13591'])(PlumeStakingStorage.Layout) := TMP_13591(PlumeStakingStorage.Layout)
 $.userValidatorStakeStartTime[user][validatorId] = block.timestamp
REF_3663(mapping(address => mapping(uint16 => uint256))) -> $_1 (-> ['TMP_13591']).userValidatorStakeStartTime
REF_3664(mapping(uint16 => uint256)) -> REF_3663[user_1]
REF_3665(uint256) -> REF_3664[validatorId_1]
$_2 (-> ['TMP_13591'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13591'])"])
REF_3665(uint256) (->$_2 (-> ['TMP_13591'])) := block.timestamp(uint256)
TMP_13591(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13591'])"])
 rewardTokens = $.rewardTokens
REF_3666(address[]) -> $_2 (-> ['TMP_13591']).rewardTokens
rewardTokens_1(address[]) = ['REF_3666(address[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < rewardTokens.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3667 -> LENGTH rewardTokens_1
TMP_13592(bool) = i_2 < REF_3667
CONDITION TMP_13592
 token = rewardTokens[i]
REF_3668(address) -> rewardTokens_1[i_2]
token_1(address) := REF_3668(address)
 $.isRewardToken[token]
REF_3669(mapping(address => bool)) -> $_2 (-> ['TMP_13591']).isRewardToken
REF_3670(bool) -> REF_3669[token_1]
CONDITION REF_3670
 PlumeRewardLogic.updateRewardPerTokenForValidator($,token,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_2 (-> ['TMP_13591'])", 'token_1', 'validatorId_1'] 
 $.userValidatorRewardPerTokenPaid[user][validatorId][token] = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_3672(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_2 (-> ['TMP_13591']).userValidatorRewardPerTokenPaid
REF_3673(mapping(uint16 => mapping(address => uint256))) -> REF_3672[user_1]
REF_3674(mapping(address => uint256)) -> REF_3673[validatorId_1]
REF_3675(uint256) -> REF_3674[token_1]
REF_3676(mapping(uint16 => mapping(address => uint256))) -> $_2 (-> ['TMP_13591']).validatorRewardPerTokenCumulative
REF_3677(mapping(address => uint256)) -> REF_3676[validatorId_1]
REF_3678(uint256) -> REF_3677[token_1]
$_3 (-> ['TMP_13591'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13591'])"])
REF_3675(uint256) (->$_3 (-> ['TMP_13591'])) := REF_3678(uint256)
TMP_13591(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13591'])"])
 $.userValidatorRewardPerTokenPaidTimestamp[user][validatorId][token] = block.timestamp
REF_3679(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_3 (-> ['TMP_13591']).userValidatorRewardPerTokenPaidTimestamp
REF_3680(mapping(uint16 => mapping(address => uint256))) -> REF_3679[user_1]
REF_3681(mapping(address => uint256)) -> REF_3680[validatorId_1]
REF_3682(uint256) -> REF_3681[token_1]
$_4 (-> ['TMP_13591'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13591'])"])
REF_3682(uint256) (->$_4 (-> ['TMP_13591'])) := block.timestamp(uint256)
TMP_13591(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13591'])"])
$_5 (-> ['TMP_13591'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13591'])", "$_4 (-> ['TMP_13591'])"])
 i ++
TMP_13594(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### StakingFacet._calculateAndClaimAllRewardsWithCleanup(address,address) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
targetToken_1(address) := phi(['tokenToRestake_1'])
 $ = PlumeStakingStorage.layout()
TMP_13595(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13595'])(PlumeStakingStorage.Layout) := TMP_13595(PlumeStakingStorage.Layout)
 totalRewards = 0
totalRewards_1(uint256) := 0(uint256)
 currentUserValidators = $.userValidators[user]
REF_3684(mapping(address => uint16[])) -> $_1 (-> ['TMP_13595']).userValidators
REF_3685(uint16[]) -> REF_3684[user_1]
currentUserValidators_1(uint16[]) = ['REF_3685(uint16[])']
 validatorsToCheck = new uint16[](currentUserValidators.length)
REF_3686 -> LENGTH currentUserValidators_1
TMP_13597(uint16[])  = new uint16[](REF_3686)
validatorsToCheck_1(uint16[]) = ['TMP_13597(uint16[])']
 checkCount = 0
checkCount_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < currentUserValidators.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3687 -> LENGTH currentUserValidators_1
TMP_13598(bool) = i_2 < REF_3687
CONDITION TMP_13598
 userValidatorId = currentUserValidators[i]
REF_3688(uint16) -> currentUserValidators_1[i_2]
userValidatorId_1(uint16) := REF_3688(uint16)
 existingRewards = $.userRewards[user][userValidatorId][targetToken]
REF_3689(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13595']).userRewards
REF_3690(mapping(uint16 => mapping(address => uint256))) -> REF_3689[user_1]
REF_3691(mapping(address => uint256)) -> REF_3690[userValidatorId_1]
REF_3692(uint256) -> REF_3691[targetToken_1]
existingRewards_1(uint256) := REF_3692(uint256)
 rewardDelta = IRewardsGetter(address(this)).getPendingRewardForValidator(user,userValidatorId,targetToken)
TMP_13599 = CONVERT this to address
TMP_13600 = CONVERT TMP_13599 to IRewardsGetter
TMP_13601(uint256) = HIGH_LEVEL_CALL, dest:TMP_13600(IRewardsGetter), function:getPendingRewardForValidator, arguments:['user_1', 'userValidatorId_1', 'targetToken_1']  
rewardDelta_1(uint256) := TMP_13601(uint256)
 totalValidatorReward = existingRewards + rewardDelta
TMP_13602(uint256) = existingRewards_1 (c)+ rewardDelta_1
totalValidatorReward_1(uint256) := TMP_13602(uint256)
 totalValidatorReward > 0
TMP_13603(bool) = totalValidatorReward_1 > 0
CONDITION TMP_13603
 totalRewards += totalValidatorReward
totalRewards_2(uint256) = totalRewards_1 (c)+ totalValidatorReward_1
 PlumeRewardLogic.updateRewardsForValidator($,user,userValidatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardsForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13595'])", 'user_1', 'userValidatorId_1'] 
 $.userRewards[user][userValidatorId][targetToken] = 0
REF_3695(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13595']).userRewards
REF_3696(mapping(uint16 => mapping(address => uint256))) -> REF_3695[user_1]
REF_3697(mapping(address => uint256)) -> REF_3696[userValidatorId_1]
REF_3698(uint256) -> REF_3697[targetToken_1]
$_2 (-> ['TMP_13595'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13595'])"])
REF_3698(uint256) (->$_2 (-> ['TMP_13595'])) := 0(uint256)
TMP_13595(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13595'])"])
 $.totalClaimableByToken[targetToken] >= totalValidatorReward
REF_3699(mapping(address => uint256)) -> $_2 (-> ['TMP_13595']).totalClaimableByToken
REF_3700(uint256) -> REF_3699[targetToken_1]
TMP_13605(bool) = REF_3700 >= totalValidatorReward_1
CONDITION TMP_13605
 $.totalClaimableByToken[targetToken] -= totalValidatorReward
REF_3701(mapping(address => uint256)) -> $_2 (-> ['TMP_13595']).totalClaimableByToken
REF_3702(uint256) -> REF_3701[targetToken_1]
$_3 (-> ['TMP_13595'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13595'])"])
REF_3702(-> $_3 (-> ['TMP_13595'])) = REF_3702 (c)- totalValidatorReward_1
TMP_13595(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13595'])"])
 $.totalClaimableByToken[targetToken] = 0
REF_3703(mapping(address => uint256)) -> $_2 (-> ['TMP_13595']).totalClaimableByToken
REF_3704(uint256) -> REF_3703[targetToken_1]
$_4 (-> ['TMP_13595'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13595'])"])
REF_3704(uint256) (->$_4 (-> ['TMP_13595'])) := 0(uint256)
TMP_13595(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13595'])"])
$_5 (-> ['TMP_13595'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13595'])", "$_4 (-> ['TMP_13595'])"])
 RewardClaimedFromValidator(user,targetToken,userValidatorId,totalValidatorReward)
Emit RewardClaimedFromValidator(user_1,targetToken_1,userValidatorId_1,totalValidatorReward_1)
 PlumeRewardLogic.clearPendingRewardsFlagIfEmpty($,user,userValidatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.clearPendingRewardsFlagIfEmpty(PlumeStakingStorage.Layout,address,uint16), arguments:["$_5 (-> ['TMP_13595'])", 'user_1', 'userValidatorId_1'] 
 validatorsToCheck[checkCount] = userValidatorId
REF_3706(uint16) -> validatorsToCheck_1[checkCount_1]
validatorsToCheck_2(uint16[]) := phi(['validatorsToCheck_1'])
REF_3706(uint16) (->validatorsToCheck_2) := userValidatorId_1(uint16)
 checkCount ++
TMP_13608(uint256) := checkCount_1(uint256)
checkCount_2(uint256) = checkCount_1 (c)+ 1
totalRewards_3(uint256) := phi(['totalRewards_2', 'totalRewards_1'])
$_6 (-> ['TMP_13595'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13595'])", "$_1 (-> ['TMP_13595'])"])
validatorsToCheck_3(uint16[]) := phi(['validatorsToCheck_2', 'validatorsToCheck_1'])
checkCount_3(uint256) := phi(['checkCount_2', 'checkCount_1'])
 i ++
TMP_13609(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < checkCount
i_scope_0_2(uint256) := phi(['i_scope_0_1', 'i_scope_0_3'])
TMP_13610(bool) = i_scope_0_2 < checkCount_1
CONDITION TMP_13610
 PlumeValidatorLogic.removeStakerFromValidator($,user,validatorsToCheck[i_scope_0])
REF_3708(uint16) -> validatorsToCheck_1[i_scope_0_2]
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.removeStakerFromValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13595'])", 'user_1', 'REF_3708'] 
 i_scope_0 ++
TMP_13612(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 (c)+ 1
 totalRewards
RETURN totalRewards_1
 totalRewards
```
#### StakingFacet._handlePostUnstakeCleanup(address,uint16) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender'])
validatorId_1(uint16) := phi(['validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13613(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13613'])(PlumeStakingStorage.Layout) := TMP_13613(PlumeStakingStorage.Layout)
 $.userValidatorStakes[user][validatorId].staked == 0
REF_3710(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13613']).userValidatorStakes
REF_3711(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3710[user_1]
REF_3712(PlumeStakingStorage.UserValidatorStake) -> REF_3711[validatorId_1]
REF_3713(uint256) -> REF_3712.staked
TMP_13614(bool) = REF_3713 == 0
CONDITION TMP_13614
 PlumeValidatorLogic.removeStakerFromValidator($,user,validatorId)
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.removeStakerFromValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13613'])", 'user_1', 'validatorId_1']
```
#### StakingFacet._countActiveCooldowns(address) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
 $ = PlumeStakingStorage.layout()
TMP_13616(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13616'])(PlumeStakingStorage.Layout) := TMP_13616(PlumeStakingStorage.Layout)
 userAssociatedValidators = $.userValidators[user]
REF_3716(mapping(address => uint16[])) -> $_1 (-> ['TMP_13616']).userValidators
REF_3717(uint16[]) -> REF_3716[user_1]
userAssociatedValidators_1 (-> [])(uint16[]) = ['REF_3717(uint16[])']
 count = 0
count_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < userAssociatedValidators.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3718 -> LENGTH userAssociatedValidators_1 (-> [])
TMP_13617(bool) = i_2 < REF_3718
CONDITION TMP_13617
 validatorId = userAssociatedValidators[i]
REF_3719(uint16) -> userAssociatedValidators_1 (-> [])[i_2]
validatorId_1(uint16) := REF_3719(uint16)
 $.validatorExists[validatorId] && $.userValidatorCooldowns[user][validatorId].amount > 0
REF_3720(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13616']).validatorExists
REF_3721(bool) -> REF_3720[validatorId_1]
REF_3722(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13616']).userValidatorCooldowns
REF_3723(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3722[user_1]
REF_3724(PlumeStakingStorage.CooldownEntry) -> REF_3723[validatorId_1]
REF_3725(uint256) -> REF_3724.amount
TMP_13618(bool) = REF_3725 > 0
TMP_13619(bool) = REF_3721 && TMP_13618
CONDITION TMP_13619
 count ++
TMP_13620(uint256) := count_1(uint256)
count_2(uint256) = count_1 (c)+ 1
count_3(uint256) := phi(['count_2', 'count_1'])
 i ++
TMP_13621(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 count
RETURN count_1
 count
```
#### StakingFacet._calculateActivelyCoolingAmount(address) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender'])
 $ = PlumeStakingStorage.layout()
TMP_13622(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13622'])(PlumeStakingStorage.Layout) := TMP_13622(PlumeStakingStorage.Layout)
 userAssociatedValidators = $.userValidators[user]
REF_3727(mapping(address => uint16[])) -> $_1 (-> ['TMP_13622']).userValidators
REF_3728(uint16[]) -> REF_3727[user_1]
userAssociatedValidators_1 (-> [])(uint16[]) = ['REF_3728(uint16[])']
 activelyCoolingAmount = 0
activelyCoolingAmount_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < userAssociatedValidators.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3729 -> LENGTH userAssociatedValidators_1 (-> [])
TMP_13623(bool) = i_2 < REF_3729
CONDITION TMP_13623
 validatorId = userAssociatedValidators[i]
REF_3730(uint16) -> userAssociatedValidators_1 (-> [])[i_2]
validatorId_1(uint16) := REF_3730(uint16)
 $.validatorExists[validatorId]
REF_3731(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13622']).validatorExists
REF_3732(bool) -> REF_3731[validatorId_1]
CONDITION REF_3732
 cooldownEntry = $.userValidatorCooldowns[user][validatorId]
REF_3733(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13622']).userValidatorCooldowns
REF_3734(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3733[user_1]
REF_3735(PlumeStakingStorage.CooldownEntry) -> REF_3734[validatorId_1]
cooldownEntry_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_3735(PlumeStakingStorage.CooldownEntry)
 cooldownEntry.amount > 0 && block.timestamp < cooldownEntry.cooldownEndTime
REF_3736(uint256) -> cooldownEntry_1 (-> ['$']).amount
TMP_13624(bool) = REF_3736 > 0
REF_3737(uint256) -> cooldownEntry_1 (-> ['$']).cooldownEndTime
TMP_13625(bool) = block.timestamp < REF_3737
TMP_13626(bool) = TMP_13624 && TMP_13625
CONDITION TMP_13626
 activelyCoolingAmount += cooldownEntry.amount
REF_3738(uint256) -> cooldownEntry_1 (-> ['$']).amount
activelyCoolingAmount_2(uint256) = activelyCoolingAmount_1 (c)+ REF_3738
activelyCoolingAmount_3(uint256) := phi(['activelyCoolingAmount_1', 'activelyCoolingAmount_2'])
 i ++
TMP_13627(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 activelyCoolingAmount
RETURN activelyCoolingAmount_1
 activelyCoolingAmount
```
#### StakingFacet._calculateTotalWithdrawableAmount(address) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender'])
 $ = PlumeStakingStorage.layout()
TMP_13628(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13628'])(PlumeStakingStorage.Layout) := TMP_13628(PlumeStakingStorage.Layout)
 userAssociatedValidators = $.userValidators[user]
REF_3740(mapping(address => uint16[])) -> $_1 (-> ['TMP_13628']).userValidators
REF_3741(uint16[]) -> REF_3740[user_1]
userAssociatedValidators_1 (-> [])(uint16[]) = ['REF_3741(uint16[])']
 totalWithdrawableAmount = $.stakeInfo[user].parked
REF_3742(mapping(address => PlumeStakingStorage.StakeInfo)) -> $_1 (-> ['TMP_13628']).stakeInfo
REF_3743(PlumeStakingStorage.StakeInfo) -> REF_3742[user_1]
REF_3744(uint256) -> REF_3743.parked
totalWithdrawableAmount_1(uint256) := REF_3744(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < userAssociatedValidators.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3745 -> LENGTH userAssociatedValidators_1 (-> [])
TMP_13629(bool) = i_2 < REF_3745
CONDITION TMP_13629
 validatorId = userAssociatedValidators[i]
REF_3746(uint16) -> userAssociatedValidators_1 (-> [])[i_2]
validatorId_1(uint16) := REF_3746(uint16)
 cooldownEntry = $.userValidatorCooldowns[user][validatorId]
REF_3747(mapping(address => mapping(uint16 => PlumeStakingStorage.CooldownEntry))) -> $_1 (-> ['TMP_13628']).userValidatorCooldowns
REF_3748(mapping(uint16 => PlumeStakingStorage.CooldownEntry)) -> REF_3747[user_1]
REF_3749(PlumeStakingStorage.CooldownEntry) -> REF_3748[validatorId_1]
cooldownEntry_1 (-> ['$'])(PlumeStakingStorage.CooldownEntry) := REF_3749(PlumeStakingStorage.CooldownEntry)
 cooldownEntry.amount > 0 && block.timestamp >= cooldownEntry.cooldownEndTime
REF_3750(uint256) -> cooldownEntry_1 (-> ['$']).amount
TMP_13630(bool) = REF_3750 > 0
REF_3751(uint256) -> cooldownEntry_1 (-> ['$']).cooldownEndTime
TMP_13631(bool) = block.timestamp >= REF_3751
TMP_13632(bool) = TMP_13630 && TMP_13631
CONDITION TMP_13632
 $.validatorExists[validatorId] && $.validators[validatorId].slashed
REF_3752(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13628']).validatorExists
REF_3753(bool) -> REF_3752[validatorId_1]
REF_3754(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13628']).validators
REF_3755(PlumeStakingStorage.ValidatorInfo) -> REF_3754[validatorId_1]
REF_3756(bool) -> REF_3755.slashed
TMP_13633(bool) = REF_3753 && REF_3756
CONDITION TMP_13633
 cooldownEntry.cooldownEndTime < $.validators[validatorId].slashedAtTimestamp
REF_3757(uint256) -> cooldownEntry_1 (-> ['$']).cooldownEndTime
REF_3758(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13628']).validators
REF_3759(PlumeStakingStorage.ValidatorInfo) -> REF_3758[validatorId_1]
REF_3760(uint256) -> REF_3759.slashedAtTimestamp
TMP_13634(bool) = REF_3757 < REF_3760
CONDITION TMP_13634
 totalWithdrawableAmount += cooldownEntry.amount
REF_3761(uint256) -> cooldownEntry_1 (-> ['$']).amount
totalWithdrawableAmount_2(uint256) = totalWithdrawableAmount_1 (c)+ REF_3761
totalWithdrawableAmount_3(uint256) := phi(['totalWithdrawableAmount_1', 'totalWithdrawableAmount_2'])
 $.validatorExists[validatorId] && ! $.validators[validatorId].slashed
REF_3762(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13628']).validatorExists
REF_3763(bool) -> REF_3762[validatorId_1]
REF_3764(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13628']).validators
REF_3765(PlumeStakingStorage.ValidatorInfo) -> REF_3764[validatorId_1]
REF_3766(bool) -> REF_3765.slashed
TMP_13635 = UnaryType.BANG REF_3766 
TMP_13636(bool) = REF_3763 && TMP_13635
CONDITION TMP_13636
 totalWithdrawableAmount += cooldownEntry.amount
REF_3767(uint256) -> cooldownEntry_1 (-> ['$']).amount
totalWithdrawableAmount_4(uint256) = totalWithdrawableAmount_1 (c)+ REF_3767
totalWithdrawableAmount_5(uint256) := phi(['totalWithdrawableAmount_4', 'totalWithdrawableAmount_1'])
 i ++
TMP_13637(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 totalWithdrawableAmount
RETURN totalWithdrawableAmount_1
 totalWithdrawableAmount
```
#### StakingFacet._cleanupValidatorRelationships(address) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
 $ = PlumeStakingStorage.layout()
TMP_13638(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13638'])(PlumeStakingStorage.Layout) := TMP_13638(PlumeStakingStorage.Layout)
 PlumeValidatorLogic.removeStakerFromAllValidators($,user)
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.removeStakerFromAllValidators(PlumeStakingStorage.Layout,address), arguments:["$_1 (-> ['TMP_13638'])", 'user_1']
```
#### StakingFacet._calculateAndClaimAllRewards(address,address) [INTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13640(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13640'])(PlumeStakingStorage.Layout) := TMP_13640(PlumeStakingStorage.Layout)
 totalRewards = 0
totalRewards_1(uint256) := 0(uint256)
 currentUserValidators = $.userValidators[user]
REF_3771(mapping(address => uint16[])) -> $_1 (-> ['TMP_13640']).userValidators
REF_3772(uint16[]) -> REF_3771[user_1]
currentUserValidators_1(uint16[]) = ['REF_3772(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < currentUserValidators.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3773 -> LENGTH currentUserValidators_1
TMP_13641(bool) = i_2 < REF_3773
CONDITION TMP_13641
 userValidatorId = currentUserValidators[i]
REF_3774(uint16) -> currentUserValidators_1[i_2]
userValidatorId_1(uint16) := REF_3774(uint16)
 existingRewards = $.userRewards[user][userValidatorId][targetToken]
REF_3775(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13640']).userRewards
REF_3776(mapping(uint16 => mapping(address => uint256))) -> REF_3775[user_1]
REF_3777(mapping(address => uint256)) -> REF_3776[userValidatorId_1]
REF_3778(uint256) -> REF_3777[targetToken_1]
existingRewards_1(uint256) := REF_3778(uint256)
 rewardDelta = IRewardsGetter(address(this)).getPendingRewardForValidator(user,userValidatorId,targetToken)
TMP_13642 = CONVERT this to address
TMP_13643 = CONVERT TMP_13642 to IRewardsGetter
TMP_13644(uint256) = HIGH_LEVEL_CALL, dest:TMP_13643(IRewardsGetter), function:getPendingRewardForValidator, arguments:['user_1', 'userValidatorId_1', 'targetToken_1']  
rewardDelta_1(uint256) := TMP_13644(uint256)
 totalValidatorReward = existingRewards + rewardDelta
TMP_13645(uint256) = existingRewards_1 (c)+ rewardDelta_1
totalValidatorReward_1(uint256) := TMP_13645(uint256)
 totalValidatorReward > 0
TMP_13646(bool) = totalValidatorReward_1 > 0
CONDITION TMP_13646
 totalRewards += totalValidatorReward
totalRewards_2(uint256) = totalRewards_1 (c)+ totalValidatorReward_1
 PlumeRewardLogic.updateRewardsForValidator($,user,userValidatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardsForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13640'])", 'user_1', 'userValidatorId_1'] 
 $.userRewards[user][userValidatorId][targetToken] = 0
REF_3781(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13640']).userRewards
REF_3782(mapping(uint16 => mapping(address => uint256))) -> REF_3781[user_1]
REF_3783(mapping(address => uint256)) -> REF_3782[userValidatorId_1]
REF_3784(uint256) -> REF_3783[targetToken_1]
$_2 (-> ['TMP_13640'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13640'])"])
REF_3784(uint256) (->$_2 (-> ['TMP_13640'])) := 0(uint256)
TMP_13640(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13640'])"])
 $.totalClaimableByToken[targetToken] >= totalValidatorReward
REF_3785(mapping(address => uint256)) -> $_2 (-> ['TMP_13640']).totalClaimableByToken
REF_3786(uint256) -> REF_3785[targetToken_1]
TMP_13648(bool) = REF_3786 >= totalValidatorReward_1
CONDITION TMP_13648
 $.totalClaimableByToken[targetToken] -= totalValidatorReward
REF_3787(mapping(address => uint256)) -> $_2 (-> ['TMP_13640']).totalClaimableByToken
REF_3788(uint256) -> REF_3787[targetToken_1]
$_3 (-> ['TMP_13640'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13640'])"])
REF_3788(-> $_3 (-> ['TMP_13640'])) = REF_3788 (c)- totalValidatorReward_1
TMP_13640(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13640'])"])
 $.totalClaimableByToken[targetToken] = 0
REF_3789(mapping(address => uint256)) -> $_2 (-> ['TMP_13640']).totalClaimableByToken
REF_3790(uint256) -> REF_3789[targetToken_1]
$_4 (-> ['TMP_13640'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13640'])"])
REF_3790(uint256) (->$_4 (-> ['TMP_13640'])) := 0(uint256)
TMP_13640(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13640'])"])
$_5 (-> ['TMP_13640'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13640'])", "$_4 (-> ['TMP_13640'])"])
 RewardClaimedFromValidator(user,targetToken,userValidatorId,totalValidatorReward)
Emit RewardClaimedFromValidator(user_1,targetToken_1,userValidatorId_1,totalValidatorReward_1)
totalRewards_3(uint256) := phi(['totalRewards_2', 'totalRewards_1'])
$_6 (-> ['TMP_13640'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13640'])", "$_2 (-> ['TMP_13640'])"])
 i ++
TMP_13650(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 totalRewards
RETURN totalRewards_1
 totalRewards
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
 historicalTokens = $.historicalRewardTokens
REF_4205(address[]) -> $_1 (-> []).historicalRewardTokens
historicalTokens_1 (-> [])(address[]) = ['REF_4205(address[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < historicalTokens.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4206 -> LENGTH historicalTokens_1 (-> [])
TMP_14034(bool) = i_2 < REF_4206
CONDITION TMP_14034
 token = historicalTokens[i]
REF_4207(address) -> historicalTokens_1 (-> [])[i_2]
token_1(address) := REF_4207(address)
 updateRewardsForValidatorAndToken($,user,validatorId,token)
INTERNAL_CALL, PlumeRewardLogic.updateRewardsForValidatorAndToken(PlumeStakingStorage.Layout,address,uint16,address)($_1 (-> []),user_1,validatorId_1,token_1)
 i ++
TMP_14036(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### PlumeValidatorLogic.addStakerToValidator(PlumeStakingStorage.Layout,address,uint16) [INTERNAL]
```slithir
 ! $.userHasStakedWithValidator[staker][validatorId]
REF_4458(mapping(address => mapping(uint16 => bool))) -> $_1 (-> []).userHasStakedWithValidator
REF_4459(mapping(uint16 => bool)) -> REF_4458[staker_1]
REF_4460(bool) -> REF_4459[validatorId_1]
TMP_14247 = UnaryType.BANG REF_4460 
CONDITION TMP_14247
 $.userValidators[staker].push(validatorId)
REF_4461(mapping(address => uint16[])) -> $_1 (-> []).userValidators
REF_4462(uint16[]) -> REF_4461[staker_1]
REF_4464 -> LENGTH REF_4462
TMP_14249(uint256) := REF_4464(uint256)
TMP_14250(uint256) = TMP_14249 (c)+ 1
$_2 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4464(uint256) (->$_3 (-> [])) := TMP_14250(uint256)
REF_4465(uint16) -> REF_4462[TMP_14249]
$_3 (-> [])(PlumeStakingStorage.Layout) := phi(['$_2 (-> [])'])
REF_4465(uint16) (->$_3 (-> [])) := validatorId_1(uint16)
 $.userHasStakedWithValidator[staker][validatorId] = true
REF_4466(mapping(address => mapping(uint16 => bool))) -> $_3 (-> []).userHasStakedWithValidator
REF_4467(mapping(uint16 => bool)) -> REF_4466[staker_1]
REF_4468(bool) -> REF_4467[validatorId_1]
$_4 (-> [])(PlumeStakingStorage.Layout) := phi(['$_3 (-> [])'])
REF_4468(bool) (->$_4 (-> [])) := True(bool)
$_5 (-> [])(PlumeStakingStorage.Layout) := phi(['$_4 (-> [])', '$_1 (-> [])'])
 ! $.isStakerForValidator[validatorId][staker]
REF_4469(mapping(uint16 => mapping(address => bool))) -> $_5 (-> []).isStakerForValidator
REF_4470(mapping(address => bool)) -> REF_4469[validatorId_1]
REF_4471(bool) -> REF_4470[staker_1]
TMP_14251 = UnaryType.BANG REF_4471 
CONDITION TMP_14251
 index = $.validatorStakers[validatorId].length
REF_4472(mapping(uint16 => address[])) -> $_5 (-> []).validatorStakers
REF_4473(address[]) -> REF_4472[validatorId_1]
REF_4474 -> LENGTH REF_4473
index_1(uint256) := REF_4474(uint256)
 $.validatorStakers[validatorId].push(staker)
REF_4475(mapping(uint16 => address[])) -> $_5 (-> []).validatorStakers
REF_4476(address[]) -> REF_4475[validatorId_1]
REF_4478 -> LENGTH REF_4476
TMP_14253(uint256) := REF_4478(uint256)
TMP_14254(uint256) = TMP_14253 (c)+ 1
$_6 (-> [])(PlumeStakingStorage.Layout) := phi(['$_5 (-> [])'])
REF_4478(uint256) (->$_7 (-> [])) := TMP_14254(uint256)
REF_4479(address) -> REF_4476[TMP_14253]
$_7 (-> [])(PlumeStakingStorage.Layout) := phi(['$_6 (-> [])'])
REF_4479(address) (->$_7 (-> [])) := staker_1(address)
 $.isStakerForValidator[validatorId][staker] = true
REF_4480(mapping(uint16 => mapping(address => bool))) -> $_7 (-> []).isStakerForValidator
REF_4481(mapping(address => bool)) -> REF_4480[validatorId_1]
REF_4482(bool) -> REF_4481[staker_1]
$_8 (-> [])(PlumeStakingStorage.Layout) := phi(['$_7 (-> [])'])
REF_4482(bool) (->$_8 (-> [])) := True(bool)
 $.userIndexInValidatorStakers[staker][validatorId] = index
REF_4483(mapping(address => mapping(uint16 => uint256))) -> $_8 (-> []).userIndexInValidatorStakers
REF_4484(mapping(uint16 => uint256)) -> REF_4483[staker_1]
REF_4485(uint256) -> REF_4484[validatorId_1]
$_9 (-> [])(PlumeStakingStorage.Layout) := phi(['$_8 (-> [])'])
REF_4485(uint256) (->$_9 (-> [])) := index_1(uint256)
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
#### PlumeRewardLogic.clearPendingRewardsFlagIfEmpty(PlumeStakingStorage.Layout,address,uint16) [INTERNAL]
```slithir
 ! $.userHasPendingRewards[user][validatorId]
REF_4416(mapping(address => mapping(uint16 => bool))) -> $_1 (-> []).userHasPendingRewards
REF_4417(mapping(uint16 => bool)) -> REF_4416[user_1]
REF_4418(bool) -> REF_4417[validatorId_1]
TMP_14218 = UnaryType.BANG REF_4418 
CONDITION TMP_14218
 historicalTokens = $.historicalRewardTokens
REF_4419(address[]) -> $_1 (-> []).historicalRewardTokens
historicalTokens_1 (-> [])(address[]) = ['REF_4419(address[])']
 userStakedAmount = $.userValidatorStakes[user][validatorId].staked
REF_4420(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> []).userValidatorStakes
REF_4421(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_4420[user_1]
REF_4422(PlumeStakingStorage.UserValidatorStake) -> REF_4421[validatorId_1]
REF_4423(uint256) -> REF_4422.staked
userStakedAmount_1(uint256) := REF_4423(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < historicalTokens.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4424 -> LENGTH historicalTokens_1 (-> [])
TMP_14219(bool) = i_2 < REF_4424
CONDITION TMP_14219
 token = historicalTokens[i]
REF_4425(address) -> historicalTokens_1 (-> [])[i_2]
token_1(address) := REF_4425(address)
 $.userRewards[user][validatorId][token] > 0
REF_4426(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> []).userRewards
REF_4427(mapping(uint16 => mapping(address => uint256))) -> REF_4426[user_1]
REF_4428(mapping(address => uint256)) -> REF_4427[validatorId_1]
REF_4429(uint256) -> REF_4428[token_1]
TMP_14220(bool) = REF_4429 > 0
CONDITION TMP_14220
 userStakedAmount > 0
TMP_14221(bool) = userStakedAmount_1 > 0
CONDITION TMP_14221
 (unsettledRewards,None,None) = calculateRewardsWithCheckpointsView($,user,validatorId,token,userStakedAmount)
TUPLE_95(uint256,uint256,uint256) = INTERNAL_CALL, PlumeRewardLogic.calculateRewardsWithCheckpointsView(PlumeStakingStorage.Layout,address,uint16,address,uint256)($_1 (-> []),user_1,validatorId_1,token_1,userStakedAmount_1)
unsettledRewards_1(uint256)= UNPACK TUPLE_95 index: 0 
 unsettledRewards > 0
TMP_14222(bool) = unsettledRewards_1 > 0
CONDITION TMP_14222
 i ++
TMP_14223(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 $.userHasPendingRewards[user][validatorId] = false
REF_4430(mapping(address => mapping(uint16 => bool))) -> $_1 (-> []).userHasPendingRewards
REF_4431(mapping(uint16 => bool)) -> REF_4430[user_1]
REF_4432(bool) -> REF_4431[validatorId_1]
$_2 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4432(bool) (->$_2 (-> [])) := False(bool)
```
#### IRewardsGetter.getPendingRewardForValidator(address,uint16,address) [EXTERNAL]
```slithir

```

#### IPlumeStakingRewardTreasury.distributeReward(address,uint256,address) [EXTERNAL]
```slithir

```

#### PlumeRewardLogic.updateRewardsForValidatorAndToken(PlumeStakingStorage.Layout,address,uint16,address) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
user_1(address) := phi(['user_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
token_1(address) := phi(['token_1'])
 userStakedAmount = $.userValidatorStakes[user][validatorId].staked
REF_4208(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> []).userValidatorStakes
REF_4209(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_4208[user_1]
REF_4210(PlumeStakingStorage.UserValidatorStake) -> REF_4209[validatorId_1]
REF_4211(uint256) -> REF_4210.staked
userStakedAmount_1(uint256) := REF_4211(uint256)
 userStakedAmount == 0
TMP_14037(bool) = userStakedAmount_1 == 0
CONDITION TMP_14037
 ! $.validators[validatorId].slashed
REF_4212(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> []).validators
REF_4213(PlumeStakingStorage.ValidatorInfo) -> REF_4212[validatorId_1]
REF_4214(bool) -> REF_4213.slashed
TMP_14038 = UnaryType.BANG REF_4214 
CONDITION TMP_14038
 updateRewardPerTokenForValidator($,token,validatorId)
INTERNAL_CALL, PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16)($_1 (-> []),token_1,validatorId_1)
 $.userValidatorRewardPerTokenPaid[user][validatorId][token] = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_4215(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> []).userValidatorRewardPerTokenPaid
REF_4216(mapping(uint16 => mapping(address => uint256))) -> REF_4215[user_1]
REF_4217(mapping(address => uint256)) -> REF_4216[validatorId_1]
REF_4218(uint256) -> REF_4217[token_1]
REF_4219(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorRewardPerTokenCumulative
REF_4220(mapping(address => uint256)) -> REF_4219[validatorId_1]
REF_4221(uint256) -> REF_4220[token_1]
$_10 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4218(uint256) (->$_10 (-> [])) := REF_4221(uint256)
 $.userValidatorRewardPerTokenPaidTimestamp[user][validatorId][token] = block.timestamp
REF_4222(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_10 (-> []).userValidatorRewardPerTokenPaidTimestamp
REF_4223(mapping(uint16 => mapping(address => uint256))) -> REF_4222[user_1]
REF_4224(mapping(address => uint256)) -> REF_4223[validatorId_1]
REF_4225(uint256) -> REF_4224[token_1]
$_11 (-> [])(PlumeStakingStorage.Layout) := phi(['$_10 (-> [])'])
REF_4225(uint256) (->$_11 (-> [])) := block.timestamp(uint256)
 $.userValidatorStakeStartTime[user][validatorId] == 0
REF_4226(mapping(address => mapping(uint16 => uint256))) -> $_1 (-> []).userValidatorStakeStartTime
REF_4227(mapping(uint16 => uint256)) -> REF_4226[user_1]
REF_4228(uint256) -> REF_4227[validatorId_1]
TMP_14040(bool) = REF_4228 == 0
CONDITION TMP_14040
 $.userValidatorStakeStartTime[user][validatorId] = block.timestamp
REF_4229(mapping(address => mapping(uint16 => uint256))) -> $_1 (-> []).userValidatorStakeStartTime
REF_4230(mapping(uint16 => uint256)) -> REF_4229[user_1]
REF_4231(uint256) -> REF_4230[validatorId_1]
$_2 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4231(uint256) (->$_2 (-> [])) := block.timestamp(uint256)
$_3 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_2 (-> [])'])
 (userRewardDelta,None,None) = calculateRewardsWithCheckpoints($,user,validatorId,token,userStakedAmount)
TUPLE_92(uint256,uint256,uint256) = INTERNAL_CALL, PlumeRewardLogic.calculateRewardsWithCheckpoints(PlumeStakingStorage.Layout,address,uint16,address,uint256)($_3 (-> []),user_1,validatorId_1,token_1,userStakedAmount_1)
userRewardDelta_1(uint256)= UNPACK TUPLE_92 index: 0 
 userRewardDelta > 0
TMP_14041(bool) = userRewardDelta_1 > 0
CONDITION TMP_14041
 $.userRewards[user][validatorId][token] += userRewardDelta
REF_4232(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_3 (-> []).userRewards
REF_4233(mapping(uint16 => mapping(address => uint256))) -> REF_4232[user_1]
REF_4234(mapping(address => uint256)) -> REF_4233[validatorId_1]
REF_4235(uint256) -> REF_4234[token_1]
$_4 (-> [])(PlumeStakingStorage.Layout) := phi(['$_3 (-> [])'])
REF_4235(-> $_4 (-> [])) = REF_4235 (c)+ userRewardDelta_1
 $.totalClaimableByToken[token] += userRewardDelta
REF_4236(mapping(address => uint256)) -> $_4 (-> []).totalClaimableByToken
REF_4237(uint256) -> REF_4236[token_1]
$_5 (-> [])(PlumeStakingStorage.Layout) := phi(['$_4 (-> [])'])
REF_4237(-> $_5 (-> [])) = REF_4237 (c)+ userRewardDelta_1
 $.userHasPendingRewards[user][validatorId] = true
REF_4238(mapping(address => mapping(uint16 => bool))) -> $_5 (-> []).userHasPendingRewards
REF_4239(mapping(uint16 => bool)) -> REF_4238[user_1]
REF_4240(bool) -> REF_4239[validatorId_1]
$_6 (-> [])(PlumeStakingStorage.Layout) := phi(['$_5 (-> [])'])
REF_4240(bool) (->$_6 (-> [])) := True(bool)
$_7 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_6 (-> [])'])
 $.userValidatorRewardPerTokenPaid[user][validatorId][token] = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_4241(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_7 (-> []).userValidatorRewardPerTokenPaid
REF_4242(mapping(uint16 => mapping(address => uint256))) -> REF_4241[user_1]
REF_4243(mapping(address => uint256)) -> REF_4242[validatorId_1]
REF_4244(uint256) -> REF_4243[token_1]
REF_4245(mapping(uint16 => mapping(address => uint256))) -> $_7 (-> []).validatorRewardPerTokenCumulative
REF_4246(mapping(address => uint256)) -> REF_4245[validatorId_1]
REF_4247(uint256) -> REF_4246[token_1]
$_8 (-> [])(PlumeStakingStorage.Layout) := phi(['$_7 (-> [])'])
REF_4244(uint256) (->$_8 (-> [])) := REF_4247(uint256)
 $.userValidatorRewardPerTokenPaidTimestamp[user][validatorId][token] = block.timestamp
REF_4248(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_8 (-> []).userValidatorRewardPerTokenPaidTimestamp
REF_4249(mapping(uint16 => mapping(address => uint256))) -> REF_4248[user_1]
REF_4250(mapping(address => uint256)) -> REF_4249[validatorId_1]
REF_4251(uint256) -> REF_4250[token_1]
$_9 (-> [])(PlumeStakingStorage.Layout) := phi(['$_8 (-> [])'])
REF_4251(uint256) (->$_9 (-> [])) := block.timestamp(uint256)
```
#### PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_1 (-> [])', '$_1 (-> [])', '$_1 (-> [])'])
token_1(address) := phi(['token_1', 'token_1', 'token_1', 'token_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1', 'validatorId_1', 'validatorId_1'])
timestamp_1(uint256) := phi(['segmentStartTime_1', 'block.timestamp', 'segmentStartTime_1', 'validatorLastUpdateTime_1'])
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
timestamp_1(uint256) := phi(['segmentStartTime_1', 'oldLastUpdateTime_1'])
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

#### PlumeRewardLogic.calculateRewardsWithCheckpoints(PlumeStakingStorage.Layout,address,uint16,address,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_3 (-> [])'])
user_1(address) := phi(['user_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
token_1(address) := phi(['token_1'])
userStakedAmount_1(uint256) := phi(['userStakedAmount_1'])
 validator = $.validators[validatorId]
REF_4318(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> []).validators
REF_4319(PlumeStakingStorage.ValidatorInfo) -> REF_4318[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4319(PlumeStakingStorage.ValidatorInfo)
 ! validator.slashed
REF_4320(bool) -> validator_1 (-> ['$']).slashed
TMP_14098 = UnaryType.BANG REF_4320 
CONDITION TMP_14098
 updateRewardPerTokenForValidator($,token,validatorId)
INTERNAL_CALL, PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16)($_1 (-> []),token_1,validatorId_1)
 finalCumulativeRewardPerToken = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_4321(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorRewardPerTokenCumulative
REF_4322(mapping(address => uint256)) -> REF_4321[validatorId_1]
REF_4323(uint256) -> REF_4322[token_1]
finalCumulativeRewardPerToken_1(uint256) := REF_4323(uint256)
 _calculateRewardsCore($,user,validatorId,token,userStakedAmount,finalCumulativeRewardPerToken)
TUPLE_93(uint256,uint256,uint256) = INTERNAL_CALL, PlumeRewardLogic._calculateRewardsCore(PlumeStakingStorage.Layout,address,uint16,address,uint256,uint256)($_1 (-> []),user_1,validatorId_1,token_1,userStakedAmount_1,finalCumulativeRewardPerToken_1)
RETURN TUPLE_93
 currentCumulativeRewardPerToken = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_4324(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorRewardPerTokenCumulative
REF_4325(mapping(address => uint256)) -> REF_4324[validatorId_1]
REF_4326(uint256) -> REF_4325[token_1]
currentCumulativeRewardPerToken_1(uint256) := REF_4326(uint256)
 effectiveEndTime = validator.slashedAtTimestamp
REF_4327(uint256) -> validator_1 (-> ['$']).slashedAtTimestamp
effectiveEndTime_1(uint256) := REF_4327(uint256)
 tokenRemovalTime = $.tokenRemovalTimestamps[token]
REF_4328(mapping(address => uint256)) -> $_1 (-> []).tokenRemovalTimestamps
REF_4329(uint256) -> REF_4328[token_1]
tokenRemovalTime_1(uint256) := REF_4329(uint256)
 tokenRemovalTime > 0 && tokenRemovalTime < effectiveEndTime
TMP_14100(bool) = tokenRemovalTime_1 > 0
TMP_14101(bool) = tokenRemovalTime_1 < effectiveEndTime_1
TMP_14102(bool) = TMP_14100 && TMP_14101
CONDITION TMP_14102
 effectiveEndTime = tokenRemovalTime
effectiveEndTime_2(uint256) := tokenRemovalTime_1(uint256)
effectiveEndTime_3(uint256) := phi(['effectiveEndTime_2', 'effectiveEndTime_1'])
 validatorLastUpdateTime = $.validatorLastUpdateTimes[validatorId][token]
REF_4330(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4331(mapping(address => uint256)) -> REF_4330[validatorId_1]
REF_4332(uint256) -> REF_4331[token_1]
validatorLastUpdateTime_1(uint256) := REF_4332(uint256)
 effectiveEndTime > validatorLastUpdateTime
TMP_14103(bool) = effectiveEndTime_3 > validatorLastUpdateTime_1
CONDITION TMP_14103
 timeSinceLastUpdate = effectiveEndTime - validatorLastUpdateTime
TMP_14104(uint256) = effectiveEndTime_3 (c)- validatorLastUpdateTime_1
timeSinceLastUpdate_1(uint256) := TMP_14104(uint256)
 userStakedAmount > 0
TMP_14105(bool) = userStakedAmount_1 > 0
CONDITION TMP_14105
 effectiveRewardRateChk = getEffectiveRewardRateAt($,token,validatorId,validatorLastUpdateTime)
TMP_14106(PlumeStakingStorage.RateCheckpoint) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256)($_1 (-> []),token_1,validatorId_1,validatorLastUpdateTime_1)
effectiveRewardRateChk_1(PlumeStakingStorage.RateCheckpoint) := TMP_14106(PlumeStakingStorage.RateCheckpoint)
 effectiveRewardRate = effectiveRewardRateChk.rate
REF_4333(uint256) -> effectiveRewardRateChk_1.rate
effectiveRewardRate_1(uint256) := REF_4333(uint256)
 effectiveRewardRate > 0
TMP_14107(bool) = effectiveRewardRate_1 > 0
CONDITION TMP_14107
 rewardPerTokenIncrease = timeSinceLastUpdate * effectiveRewardRate
TMP_14108(uint256) = timeSinceLastUpdate_1 (c)* effectiveRewardRate_1
rewardPerTokenIncrease_1(uint256) := TMP_14108(uint256)
 currentCumulativeRewardPerToken += rewardPerTokenIncrease
currentCumulativeRewardPerToken_2(uint256) = currentCumulativeRewardPerToken_1 (c)+ rewardPerTokenIncrease_1
currentCumulativeRewardPerToken_3(uint256) := phi(['currentCumulativeRewardPerToken_1', 'currentCumulativeRewardPerToken_2'])
 _calculateRewardsCore($,user,validatorId,token,userStakedAmount,currentCumulativeRewardPerToken)
TUPLE_94(uint256,uint256,uint256) = INTERNAL_CALL, PlumeRewardLogic._calculateRewardsCore(PlumeStakingStorage.Layout,address,uint16,address,uint256,uint256)($_1 (-> []),user_1,validatorId_1,token_1,userStakedAmount_1,currentCumulativeRewardPerToken_3)
RETURN TUPLE_94
 (totalUserRewardDelta,totalCommissionAmountDelta,effectiveTimeDelta)
```
#### PlumeRewardLogic.findRewardRateCheckpointIndexAtOrBefore(PlumeStakingStorage.Layout,uint16,address,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
validatorId_1(uint16) := phi(['validatorId_1'])
token_1(address) := phi(['token_1'])
timestamp_1(uint256) := phi(['timestamp_1'])
 checkpoints = $.validatorRewardRateCheckpoints[validatorId][token]
REF_4375(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> []).validatorRewardRateCheckpoints
REF_4376(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4375[validatorId_1]
REF_4377(PlumeStakingStorage.RateCheckpoint[]) -> REF_4376[token_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4377(PlumeStakingStorage.RateCheckpoint[])']
 len = checkpoints.length
REF_4378 -> LENGTH checkpoints_1 (-> [])
len_1(uint256) := REF_4378(uint256)
 len == 0
TMP_14169(bool) = len_1 == 0
CONDITION TMP_14169
 0
RETURN 0
 low = 0
low_1(uint256) := 0(uint256)
 high = len - 1
TMP_14170(uint256) = len_1 (c)- 1
high_1(uint256) := TMP_14170(uint256)
 ans = 0
ans_1(uint256) := 0(uint256)
 foundSuitable = false
foundSuitable_1(bool) := False(bool)
 low <= high
TMP_14171(bool) = low_1 <= high_1
CONDITION TMP_14171
 mid = low + (high - low) / 2
TMP_14172(uint256) = high_1 (c)- low_1
TMP_14173(uint256) = TMP_14172 (c)/ 2
TMP_14174(uint256) = low_1 (c)+ TMP_14173
mid_1(uint256) := TMP_14174(uint256)
 checkpoints[mid].timestamp <= timestamp
REF_4379(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[mid_1]
REF_4380(uint256) -> REF_4379.timestamp
TMP_14175(bool) = REF_4380 <= timestamp_1
CONDITION TMP_14175
 ans = mid
ans_2(uint256) := mid_1(uint256)
 foundSuitable = true
foundSuitable_2(bool) := True(bool)
 low = mid + 1
TMP_14176(uint256) = mid_1 (c)+ 1
low_2(uint256) := TMP_14176(uint256)
 mid == 0
TMP_14177(bool) = mid_1 == 0
CONDITION TMP_14177
 high = mid - 1
TMP_14178(uint256) = mid_1 (c)- 1
high_2(uint256) := TMP_14178(uint256)
low_3(uint256) := phi(['low_1', 'low_2'])
high_3(uint256) := phi(['high_2', 'high_1'])
ans_3(uint256) := phi(['ans_2', 'ans_1'])
 ans
RETURN ans_1
```
#### PlumeRewardLogic.findCommissionCheckpointIndexAtOrBefore(PlumeStakingStorage.Layout,uint16,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
validatorId_1(uint16) := phi(['validatorId_1'])
timestamp_1(uint256) := phi(['timestamp_1'])
 checkpoints = $.validatorCommissionCheckpoints[validatorId]
REF_4381(mapping(uint16 => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> []).validatorCommissionCheckpoints
REF_4382(PlumeStakingStorage.RateCheckpoint[]) -> REF_4381[validatorId_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4382(PlumeStakingStorage.RateCheckpoint[])']
 len = checkpoints.length
REF_4383 -> LENGTH checkpoints_1 (-> [])
len_1(uint256) := REF_4383(uint256)
 len == 0
TMP_14179(bool) = len_1 == 0
CONDITION TMP_14179
 0
RETURN 0
 low = 0
low_1(uint256) := 0(uint256)
 high = len - 1
TMP_14180(uint256) = len_1 (c)- 1
high_1(uint256) := TMP_14180(uint256)
 ans = 0
ans_1(uint256) := 0(uint256)
 foundSuitable = false
foundSuitable_1(bool) := False(bool)
 low <= high
TMP_14181(bool) = low_1 <= high_1
CONDITION TMP_14181
 mid = low + (high - low) / 2
TMP_14182(uint256) = high_1 (c)- low_1
TMP_14183(uint256) = TMP_14182 (c)/ 2
TMP_14184(uint256) = low_1 (c)+ TMP_14183
mid_1(uint256) := TMP_14184(uint256)
 checkpoints[mid].timestamp <= timestamp
REF_4384(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[mid_1]
REF_4385(uint256) -> REF_4384.timestamp
TMP_14185(bool) = REF_4385 <= timestamp_1
CONDITION TMP_14185
 ans = mid
ans_2(uint256) := mid_1(uint256)
 foundSuitable = true
foundSuitable_2(bool) := True(bool)
 low = mid + 1
TMP_14186(uint256) = mid_1 (c)+ 1
low_2(uint256) := TMP_14186(uint256)
 mid == 0
TMP_14187(bool) = mid_1 == 0
CONDITION TMP_14187
 high = mid - 1
TMP_14188(uint256) = mid_1 (c)- 1
high_2(uint256) := TMP_14188(uint256)
low_3(uint256) := phi(['low_1', 'low_2'])
high_3(uint256) := phi(['high_2', 'high_1'])
ans_3(uint256) := phi(['ans_1', 'ans_2'])
 ans
RETURN ans_1
```
#### PlumeRewardLogic._calculateRewardsCore(PlumeStakingStorage.Layout,address,uint16,address,uint256,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_1 (-> [])'])
user_1(address) := phi(['user_1', 'user_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
token_1(address) := phi(['token_1', 'token_1'])
userStakedAmount_1(uint256) := phi(['userStakedAmount_1', 'userStakedAmount_1'])
currentCumulativeRewardPerToken_1(uint256) := phi(['currentCumulativeRewardPerToken_3', 'finalCumulativeRewardPerToken_1', 'simulatedCumulativeRPT_1'])
 lastUserPaidCumulativeRewardPerToken = $.userValidatorRewardPerTokenPaid[user][validatorId][token]
REF_4281(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> []).userValidatorRewardPerTokenPaid
REF_4282(mapping(uint16 => mapping(address => uint256))) -> REF_4281[user_1]
REF_4283(mapping(address => uint256)) -> REF_4282[validatorId_1]
REF_4284(uint256) -> REF_4283[token_1]
lastUserPaidCumulativeRewardPerToken_1(uint256) := REF_4284(uint256)
 lastUserRewardUpdateTime = $.userValidatorRewardPerTokenPaidTimestamp[user][validatorId][token]
REF_4285(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> []).userValidatorRewardPerTokenPaidTimestamp
REF_4286(mapping(uint16 => mapping(address => uint256))) -> REF_4285[user_1]
REF_4287(mapping(address => uint256)) -> REF_4286[validatorId_1]
REF_4288(uint256) -> REF_4287[token_1]
lastUserRewardUpdateTime_1(uint256) := REF_4288(uint256)
 lastUserRewardUpdateTime == 0
TMP_14057(bool) = lastUserRewardUpdateTime_1 == 0
CONDITION TMP_14057
 lastUserRewardUpdateTime = $.userValidatorStakeStartTime[user][validatorId]
REF_4289(mapping(address => mapping(uint16 => uint256))) -> $_1 (-> []).userValidatorStakeStartTime
REF_4290(mapping(uint16 => uint256)) -> REF_4289[user_1]
REF_4291(uint256) -> REF_4290[validatorId_1]
lastUserRewardUpdateTime_2(uint256) := REF_4291(uint256)
 lastUserRewardUpdateTime == 0 && $.userValidatorStakes[user][validatorId].staked > 0
TMP_14058(bool) = lastUserRewardUpdateTime_2 == 0
REF_4292(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> []).userValidatorStakes
REF_4293(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_4292[user_1]
REF_4294(PlumeStakingStorage.UserValidatorStake) -> REF_4293[validatorId_1]
REF_4295(uint256) -> REF_4294.staked
TMP_14059(bool) = REF_4295 > 0
TMP_14060(bool) = TMP_14058 && TMP_14059
CONDITION TMP_14060
 fallbackTime = block.timestamp
fallbackTime_1(uint256) := block.timestamp(uint256)
 validator = $.validators[validatorId]
REF_4296(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> []).validators
REF_4297(PlumeStakingStorage.ValidatorInfo) -> REF_4296[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4297(PlumeStakingStorage.ValidatorInfo)
 validator.slashedAtTimestamp > 0 && validator.slashedAtTimestamp < fallbackTime
REF_4298(uint256) -> validator_1 (-> ['$']).slashedAtTimestamp
TMP_14061(bool) = REF_4298 > 0
REF_4299(uint256) -> validator_1 (-> ['$']).slashedAtTimestamp
TMP_14062(bool) = REF_4299 < fallbackTime_1
TMP_14063(bool) = TMP_14061 && TMP_14062
CONDITION TMP_14063
 fallbackTime = validator.slashedAtTimestamp
REF_4300(uint256) -> validator_1 (-> ['$']).slashedAtTimestamp
fallbackTime_2(uint256) := REF_4300(uint256)
fallbackTime_3(uint256) := phi(['fallbackTime_2', 'fallbackTime_1'])
 lastUserRewardUpdateTime = fallbackTime
lastUserRewardUpdateTime_3(uint256) := fallbackTime_3(uint256)
lastUserRewardUpdateTime_4(uint256) := phi(['lastUserRewardUpdateTime_2', 'lastUserRewardUpdateTime_3'])
lastUserRewardUpdateTime_5(uint256) := phi(['lastUserRewardUpdateTime_2', 'lastUserRewardUpdateTime_1'])
 validatorLastUpdateTime = $.validatorLastUpdateTimes[validatorId][token]
REF_4301(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4302(mapping(address => uint256)) -> REF_4301[validatorId_1]
REF_4303(uint256) -> REF_4302[token_1]
validatorLastUpdateTime_1(uint256) := REF_4303(uint256)
 validator_scope_0 = $.validators[validatorId]
REF_4304(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> []).validators
REF_4305(PlumeStakingStorage.ValidatorInfo) -> REF_4304[validatorId_1]
validator_scope_0_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4305(PlumeStakingStorage.ValidatorInfo)
 effectiveEndTime = block.timestamp
effectiveEndTime_1(uint256) := block.timestamp(uint256)
 tokenRemovalTime = $.tokenRemovalTimestamps[token]
REF_4306(mapping(address => uint256)) -> $_1 (-> []).tokenRemovalTimestamps
REF_4307(uint256) -> REF_4306[token_1]
tokenRemovalTime_1(uint256) := REF_4307(uint256)
 tokenRemovalTime > 0 && tokenRemovalTime < effectiveEndTime
TMP_14064(bool) = tokenRemovalTime_1 > 0
TMP_14065(bool) = tokenRemovalTime_1 < effectiveEndTime_1
TMP_14066(bool) = TMP_14064 && TMP_14065
CONDITION TMP_14066
 effectiveEndTime = tokenRemovalTime
effectiveEndTime_2(uint256) := tokenRemovalTime_1(uint256)
effectiveEndTime_3(uint256) := phi(['effectiveEndTime_1', 'effectiveEndTime_2'])
 validator_scope_0.slashedAtTimestamp > 0
REF_4308(uint256) -> validator_scope_0_1 (-> ['$']).slashedAtTimestamp
TMP_14067(bool) = REF_4308 > 0
CONDITION TMP_14067
 validator_scope_0.slashedAtTimestamp < effectiveEndTime
REF_4309(uint256) -> validator_scope_0_1 (-> ['$']).slashedAtTimestamp
TMP_14068(bool) = REF_4309 < effectiveEndTime_3
CONDITION TMP_14068
 effectiveEndTime = validator_scope_0.slashedAtTimestamp
REF_4310(uint256) -> validator_scope_0_1 (-> ['$']).slashedAtTimestamp
effectiveEndTime_4(uint256) := REF_4310(uint256)
effectiveEndTime_5(uint256) := phi(['effectiveEndTime_1', 'effectiveEndTime_4'])
 effectiveEndTime <= lastUserRewardUpdateTime || currentCumulativeRewardPerToken <= lastUserPaidCumulativeRewardPerToken
TMP_14069(bool) = effectiveEndTime_5 <= lastUserRewardUpdateTime_5
TMP_14070(bool) = currentCumulativeRewardPerToken_1 <= lastUserPaidCumulativeRewardPerToken_1
TMP_14071(bool) = TMP_14069 || TMP_14070
CONDITION TMP_14071
 (0,0,0)
RETURN 0,0,0
 effectiveTimeDelta = effectiveEndTime - lastUserRewardUpdateTime
TMP_14072(uint256) = effectiveEndTime_5 (c)- lastUserRewardUpdateTime_5
effectiveTimeDelta_1(uint256) := TMP_14072(uint256)
 distinctTimestamps = getDistinctTimestamps($,validatorId,token,lastUserRewardUpdateTime,effectiveEndTime)
TMP_14073(uint256[]) = INTERNAL_CALL, PlumeRewardLogic.getDistinctTimestamps(PlumeStakingStorage.Layout,uint16,address,uint256,uint256)($_1 (-> []),validatorId_1,token_1,lastUserRewardUpdateTime_5,effectiveEndTime_5)
distinctTimestamps_1(uint256[]) = ['TMP_14073(uint256[])']
 distinctTimestamps.length < 2
REF_4311 -> LENGTH distinctTimestamps_1
TMP_14074(bool) = REF_4311 < 2
CONDITION TMP_14074
 (0,0,0)
RETURN 0,0,0
 rptTracker = lastUserPaidCumulativeRewardPerToken
rptTracker_1(uint256) := lastUserPaidCumulativeRewardPerToken_1(uint256)
 k = 0
k_1(uint256) := 0(uint256)
 k < distinctTimestamps.length - 1
k_2(uint256) := phi(['k_1', 'k_3'])
REF_4312 -> LENGTH distinctTimestamps_1
TMP_14075(uint256) = REF_4312 (c)- 1
TMP_14076(bool) = k_2 < TMP_14075
CONDITION TMP_14076
 segmentStartTime = distinctTimestamps[k]
REF_4313(uint256) -> distinctTimestamps_1[k_2]
segmentStartTime_1(uint256) := REF_4313(uint256)
 segmentEndTime = distinctTimestamps[k + 1]
TMP_14077(uint256) = k_2 (c)+ 1
REF_4314(uint256) -> distinctTimestamps_1[TMP_14077]
segmentEndTime_1(uint256) := REF_4314(uint256)
 segmentEndTime <= segmentStartTime
TMP_14078(bool) = segmentEndTime_1 <= segmentStartTime_1
CONDITION TMP_14078
 k == 0
TMP_14079(bool) = k_2 == 0
CONDITION TMP_14079
 rptAtSegmentStart = lastUserPaidCumulativeRewardPerToken
rptAtSegmentStart_2(uint256) := lastUserPaidCumulativeRewardPerToken_1(uint256)
 rptAtSegmentStart = rptTracker
rptAtSegmentStart_1(uint256) := rptTracker_1(uint256)
rptAtSegmentStart_3(uint256) := phi(['rptAtSegmentStart_2', 'rptAtSegmentStart_1'])
 rewardRateInfoForSegment = getEffectiveRewardRateAt($,token,validatorId,segmentStartTime)
TMP_14080(PlumeStakingStorage.RateCheckpoint) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256)($_1 (-> []),token_1,validatorId_1,segmentStartTime_1)
rewardRateInfoForSegment_1(PlumeStakingStorage.RateCheckpoint) := TMP_14080(PlumeStakingStorage.RateCheckpoint)
 effectiveRewardRate = rewardRateInfoForSegment.rate
REF_4315(uint256) -> rewardRateInfoForSegment_1.rate
effectiveRewardRate_1(uint256) := REF_4315(uint256)
 segmentDuration = segmentEndTime - segmentStartTime
TMP_14081(uint256) = segmentEndTime_1 (c)- segmentStartTime_1
segmentDuration_1(uint256) := TMP_14081(uint256)
 rptIncreaseInSegment = 0
rptIncreaseInSegment_1(uint256) := 0(uint256)
 effectiveRewardRate > 0 && segmentDuration > 0
TMP_14082(bool) = effectiveRewardRate_1 > 0
TMP_14083(bool) = segmentDuration_1 > 0
TMP_14084(bool) = TMP_14082 && TMP_14083
CONDITION TMP_14084
 rptIncreaseInSegment = segmentDuration * effectiveRewardRate
TMP_14085(uint256) = segmentDuration_1 (c)* effectiveRewardRate_1
rptIncreaseInSegment_2(uint256) := TMP_14085(uint256)
rptIncreaseInSegment_3(uint256) := phi(['rptIncreaseInSegment_2', 'rptIncreaseInSegment_1'])
 rptAtSegmentEnd = rptAtSegmentStart + rptIncreaseInSegment
TMP_14086(uint256) = rptAtSegmentStart_3 (c)+ rptIncreaseInSegment_3
rptAtSegmentEnd_1(uint256) := TMP_14086(uint256)
 rewardPerTokenDeltaForUserInSegment = rptAtSegmentEnd - rptAtSegmentStart
TMP_14087(uint256) = rptAtSegmentEnd_1 (c)- rptAtSegmentStart_3
rewardPerTokenDeltaForUserInSegment_1(uint256) := TMP_14087(uint256)
 rewardPerTokenDeltaForUserInSegment > 0 && userStakedAmount > 0
TMP_14088(bool) = rewardPerTokenDeltaForUserInSegment_1 > 0
TMP_14089(bool) = userStakedAmount_1 > 0
TMP_14090(bool) = TMP_14088 && TMP_14089
CONDITION TMP_14090
 grossRewardForSegment = (userStakedAmount * rewardPerTokenDeltaForUserInSegment) / PlumeStakingStorage.REWARD_PRECISION
TMP_14091(uint256) = userStakedAmount_1 (c)* rewardPerTokenDeltaForUserInSegment_1
REF_4316(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_14092(uint256) = TMP_14091 (c)/ REF_4316
grossRewardForSegment_1(uint256) := TMP_14092(uint256)
 effectiveCommissionRate = getEffectiveCommissionRateAt($,validatorId,segmentStartTime)
TMP_14093(uint256) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveCommissionRateAt(PlumeStakingStorage.Layout,uint16,uint256)($_1 (-> []),validatorId_1,segmentStartTime_1)
effectiveCommissionRate_1(uint256) := TMP_14093(uint256)
 commissionForThisSegment = _ceilDiv(grossRewardForSegment * effectiveCommissionRate,PlumeStakingStorage.REWARD_PRECISION)
TMP_14094(uint256) = grossRewardForSegment_1 (c)* effectiveCommissionRate_1
REF_4317(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_14095(uint256) = INTERNAL_CALL, PlumeRewardLogic._ceilDiv(uint256,uint256)(TMP_14094,REF_4317)
commissionForThisSegment_1(uint256) := TMP_14095(uint256)
 grossRewardForSegment >= commissionForThisSegment
TMP_14096(bool) = grossRewardForSegment_1 >= commissionForThisSegment_1
CONDITION TMP_14096
 totalUserRewardDelta += (grossRewardForSegment - commissionForThisSegment)
TMP_14097(uint256) = grossRewardForSegment_1 (c)- commissionForThisSegment_1
totalUserRewardDelta_1(uint256) = totalUserRewardDelta_0 (c)+ TMP_14097
totalUserRewardDelta_2(uint256) := phi(['totalUserRewardDelta_1', 'totalUserRewardDelta_0'])
 totalCommissionAmountDelta += commissionForThisSegment
totalCommissionAmountDelta_1(uint256) = totalCommissionAmountDelta_0 (c)+ commissionForThisSegment_1
totalCommissionAmountDelta_2(uint256) := phi(['totalCommissionAmountDelta_1', 'totalCommissionAmountDelta_0'])
 rptTracker = rptAtSegmentEnd
rptTracker_3(uint256) := rptAtSegmentEnd_1(uint256)
 ++ k
rptTracker_2(uint256) := phi(['rptTracker_3', 'rptTracker_1'])
k_3(uint256) = k_2 (c)+ 1
 (totalUserRewardDelta,totalCommissionAmountDelta,effectiveTimeDelta)
RETURN totalUserRewardDelta_0,totalCommissionAmountDelta_0,effectiveTimeDelta_1
 (totalUserRewardDelta,totalCommissionAmountDelta,effectiveTimeDelta)
```
#### PlumeRewardLogic.getDistinctTimestamps(PlumeStakingStorage.Layout,uint16,address,uint256,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_1 (-> [])'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
token_1(address) := phi(['token_1', 'token_1'])
periodStart_1(uint256) := phi(['lastUpdateTime_1', 'lastUserRewardUpdateTime_5'])
periodEnd_1(uint256) := phi(['effectiveEndTime_5', 'effectiveEndTime_5'])
 rewardCheckpoints = $.validatorRewardRateCheckpoints[validatorId][token]
REF_4334(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> []).validatorRewardRateCheckpoints
REF_4335(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4334[validatorId_1]
REF_4336(PlumeStakingStorage.RateCheckpoint[]) -> REF_4335[token_1]
rewardCheckpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4336(PlumeStakingStorage.RateCheckpoint[])']
 commissionCheckpoints = $.validatorCommissionCheckpoints[validatorId]
REF_4337(mapping(uint16 => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> []).validatorCommissionCheckpoints
REF_4338(PlumeStakingStorage.RateCheckpoint[]) -> REF_4337[validatorId_1]
commissionCheckpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4338(PlumeStakingStorage.RateCheckpoint[])']
 len1 = rewardCheckpoints.length
REF_4339 -> LENGTH rewardCheckpoints_1 (-> [])
len1_1(uint256) := REF_4339(uint256)
 len2 = commissionCheckpoints.length
REF_4340 -> LENGTH commissionCheckpoints_1 (-> [])
len2_1(uint256) := REF_4340(uint256)
 periodStart > periodEnd
TMP_14113(bool) = periodStart_1 > periodEnd_1
CONDITION TMP_14113
 new uint256[](0)
TMP_14115(uint256[])  = new uint256[](0)
RETURN TMP_14115
 periodStart == periodEnd
TMP_14116(bool) = periodStart_1 == periodEnd_1
CONDITION TMP_14116
 singlePoint = new uint256[](1)
TMP_14118(uint256[])  = new uint256[](1)
singlePoint_1(uint256[]) = ['TMP_14118(uint256[])']
 singlePoint[0] = periodStart
REF_4341(uint256) -> singlePoint_1[0]
singlePoint_2(uint256[]) := phi(['singlePoint_1'])
REF_4341(uint256) (->singlePoint_2) := periodStart_1(uint256)
 singlePoint
RETURN singlePoint_2
 result = new uint256[](len1 + len2 + 2)
TMP_14120(uint256) = len1_1 (c)+ len2_1
TMP_14121(uint256) = TMP_14120 (c)+ 2
TMP_14122(uint256[])  = new uint256[](TMP_14121)
result_1(uint256[]) = ['TMP_14122(uint256[])']
 i = 0
i_1(uint256) := 0(uint256)
 j = 0
j_1(uint256) := 0(uint256)
 k = 0
k_1(uint256) := 0(uint256)
 result[k ++] = periodStart
TMP_14123(uint256) := k_1(uint256)
k_2(uint256) = k_1 (c)+ 1
REF_4342(uint256) -> result_1[TMP_14123]
result_2(uint256[]) := phi(['result_1'])
REF_4342(uint256) (->result_2) := periodStart_1(uint256)
 lastAddedTimestamp = periodStart
lastAddedTimestamp_1(uint256) := periodStart_1(uint256)
 i < len1 && rewardCheckpoints[i].timestamp < periodStart
i_2(uint256) := phi(['i_3', 'i_1'])
TMP_14124(bool) = i_2 < len1_1
REF_4343(PlumeStakingStorage.RateCheckpoint) -> rewardCheckpoints_1 (-> [])[i_2]
REF_4344(uint256) -> REF_4343.timestamp
TMP_14125(bool) = REF_4344 < periodStart_1
TMP_14126(bool) = TMP_14124 && TMP_14125
CONDITION TMP_14126
 i ++
TMP_14127(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 j < len2 && commissionCheckpoints[j].timestamp < periodStart
j_2(uint256) := phi(['j_1', 'j_5'])
TMP_14128(bool) = j_2 < len2_1
REF_4345(PlumeStakingStorage.RateCheckpoint) -> commissionCheckpoints_1 (-> [])[j_2]
REF_4346(uint256) -> REF_4345.timestamp
TMP_14129(bool) = REF_4346 < periodStart_1
TMP_14130(bool) = TMP_14128 && TMP_14129
CONDITION TMP_14130
 j ++
TMP_14131(uint256) := j_2(uint256)
j_5(uint256) = j_2 (c)+ 1
 i < len1 || j < len2
TMP_14132(bool) = i_2 < len1_1
TMP_14133(bool) = j_2 < len2_1
TMP_14134(bool) = TMP_14132 || TMP_14133
CONDITION TMP_14134
 advanceI = false
advanceI_1(bool) := False(bool)
 advanceJ = false
advanceJ_1(bool) := False(bool)
 t1 < t2
TMP_14135(bool) = t1_3 < t2_3
CONDITION TMP_14135
 currentTimestampToAdd = t1
currentTimestampToAdd_1(uint256) := t1_3(uint256)
 advanceI = true
advanceI_2(bool) := True(bool)
 t2 < t1
TMP_14136(bool) = t2_3 < t1_3
CONDITION TMP_14136
 currentTimestampToAdd = t2
currentTimestampToAdd_2(uint256) := t2_3(uint256)
 advanceJ = true
advanceJ_2(bool) := True(bool)
 t1 != type()(uint256).max
TMP_14138(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_14139(bool) = t1_3 != TMP_14138
CONDITION TMP_14139
 currentTimestampToAdd = t1
currentTimestampToAdd_3(uint256) := t1_3(uint256)
 advanceI = true
advanceI_3(bool) := True(bool)
 advanceJ = true
advanceJ_3(bool) := True(bool)
currentTimestampToAdd_4(uint256) := phi(['currentTimestampToAdd_2', 'currentTimestampToAdd_0', 'currentTimestampToAdd_3'])
advanceI_4(bool) := phi(['advanceI_3', 'advanceI_1'])
advanceJ_4(bool) := phi(['advanceJ_1', 'advanceJ_2', 'advanceJ_3'])
currentTimestampToAdd_5(uint256) := phi(['currentTimestampToAdd_1', 'currentTimestampToAdd_0'])
advanceI_5(bool) := phi(['advanceI_1', 'advanceI_2'])
 currentTimestampToAdd >= periodEnd
TMP_14140(bool) = currentTimestampToAdd_5 >= periodEnd_1
CONDITION TMP_14140
 currentTimestampToAdd > lastAddedTimestamp
TMP_14141(bool) = currentTimestampToAdd_5 > lastAddedTimestamp_1
CONDITION TMP_14141
 result[k ++] = currentTimestampToAdd
TMP_14142(uint256) := k_2(uint256)
k_3(uint256) = k_2 (c)+ 1
REF_4347(uint256) -> result_2[TMP_14142]
result_3(uint256[]) := phi(['result_2'])
REF_4347(uint256) (->result_3) := currentTimestampToAdd_5(uint256)
 lastAddedTimestamp = currentTimestampToAdd
lastAddedTimestamp_2(uint256) := currentTimestampToAdd_5(uint256)
k_4(uint256) := phi(['k_2', 'k_3'])
lastAddedTimestamp_3(uint256) := phi(['lastAddedTimestamp_2', 'lastAddedTimestamp_1'])
 advanceI
CONDITION advanceI_5
 i ++
TMP_14143(uint256) := i_2(uint256)
i_4(uint256) = i_2 (c)+ 1
i_5(uint256) := phi(['i_4', 'i_1'])
 advanceJ
CONDITION advanceJ_4
 j ++
TMP_14144(uint256) := j_2(uint256)
j_3(uint256) = j_2 (c)+ 1
j_4(uint256) := phi(['j_1', 'j_3'])
 lastAddedTimestamp < periodEnd
TMP_14145(bool) = lastAddedTimestamp_1 < periodEnd_1
CONDITION TMP_14145
 result[k ++] = periodEnd
TMP_14146(uint256) := k_2(uint256)
k_5(uint256) = k_2 (c)+ 1
REF_4348(uint256) -> result_2[TMP_14146]
result_4(uint256[]) := phi(['result_2'])
REF_4348(uint256) (->result_4) := periodEnd_1(uint256)
result_5(uint256[]) := phi(['result_2', 'result_4'])
k_6(uint256) := phi(['k_2', 'k_5'])
 mstore(uint256,uint256)(result,k)
TMP_14147(None) = SOLIDITY_CALL mstore(uint256,uint256)(result_5,k_6)
 result
RETURN result_5
 (i < len1)
TMP_14148(bool) = i_2 < len1_1
CONDITION TMP_14148
 t1 = rewardCheckpoints[i].timestamp
REF_4349(PlumeStakingStorage.RateCheckpoint) -> rewardCheckpoints_1 (-> [])[i_2]
REF_4350(uint256) -> REF_4349.timestamp
t1_1(uint256) := REF_4350(uint256)
 t1 = type()(uint256).max
TMP_14150(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
t1_2(uint256) := TMP_14150(uint256)
t1_3(uint256) := phi(['t1_1', 't1_2'])
 (j < len2)
TMP_14151(bool) = j_2 < len2_1
CONDITION TMP_14151
 t2 = commissionCheckpoints[j].timestamp
REF_4351(PlumeStakingStorage.RateCheckpoint) -> commissionCheckpoints_1 (-> [])[j_2]
REF_4352(uint256) -> REF_4351.timestamp
t2_1(uint256) := REF_4352(uint256)
 t2 = type()(uint256).max
TMP_14153(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
t2_2(uint256) := TMP_14153(uint256)
t2_3(uint256) := phi(['t2_1', 't2_2'])
```
