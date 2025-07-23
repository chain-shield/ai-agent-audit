

#### PlumeStaking.initializePlume(address,uint256,uint256,uint256,uint256) [EXTERNAL][OWNER]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_12561(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_12561'])(PlumeStakingStorage.Layout) := TMP_12561(PlumeStakingStorage.Layout)
 require(bool,string)(! $.initialized,PlumeStaking: Already initialized)
REF_2682(bool) -> $_1 (-> ['TMP_12561']).initialized
TMP_12562 = UnaryType.BANG REF_2682 
TMP_12563(None) = SOLIDITY_CALL require(bool,string)(TMP_12562,PlumeStaking: Already initialized)
 minStake == 0
TMP_12564(bool) = minStake_1 == 0
CONDITION TMP_12564
 revert InvalidAmount(uint256)(minStake)
TMP_12565(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(minStake_1)
 cooldown == 0
TMP_12566(bool) = cooldown_1 == 0
CONDITION TMP_12566
 revert InvalidAmount(uint256)(cooldown)
TMP_12567(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(cooldown_1)
 maxSlashVoteDuration == 0
TMP_12568(bool) = maxSlashVoteDuration_1 == 0
CONDITION TMP_12568
 revert InvalidAmount(uint256)(maxSlashVoteDuration)
TMP_12569(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(maxSlashVoteDuration_1)
 cooldown <= maxSlashVoteDuration
TMP_12570(bool) = cooldown_1 <= maxSlashVoteDuration_1
CONDITION TMP_12570
 revert CooldownTooShortForSlashVote(uint256,uint256)(cooldown,maxSlashVoteDuration)
TMP_12571(None) = SOLIDITY_CALL revert CooldownTooShortForSlashVote(uint256,uint256)(cooldown_1,maxSlashVoteDuration_1)
 maxValidatorCommission > PlumeStakingStorage.REWARD_PRECISION / 2
REF_2683(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_12572(uint256) = REF_2683 (c)/ 2
TMP_12573(bool) = maxValidatorCommission_1 > TMP_12572
CONDITION TMP_12573
 revert InvalidMaxCommissionRate(uint256,uint256)(maxValidatorCommission,PlumeStakingStorage.REWARD_PRECISION / 2)
REF_2684(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_12574(uint256) = REF_2684 (c)/ 2
TMP_12575(None) = SOLIDITY_CALL revert InvalidMaxCommissionRate(uint256,uint256)(maxValidatorCommission_1,TMP_12574)
 initialOwner != address(0) && initialOwner != owner()
TMP_12576 = CONVERT 0 to address
TMP_12577(bool) = initialOwner_1 != TMP_12576
TMP_12578(address) = INTERNAL_CALL, Ownable.owner()()
TMP_12579(bool) = initialOwner_1 != TMP_12578
TMP_12580(bool) = TMP_12577 && TMP_12579
CONDITION TMP_12580
 _transferOwnership(initialOwner)
INTERNAL_CALL, SolidStateDiamond._transferOwnership(address)(initialOwner_1)
 $.minStakeAmount = minStake
REF_2685(uint256) -> $_1 (-> ['TMP_12561']).minStakeAmount
$_2 (-> ['TMP_12561'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12561'])"])
REF_2685(uint256) (->$_2 (-> ['TMP_12561'])) := minStake_1(uint256)
TMP_12561(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12561'])"])
 $.cooldownInterval = cooldown
REF_2686(uint256) -> $_2 (-> ['TMP_12561']).cooldownInterval
$_3 (-> ['TMP_12561'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12561'])"])
REF_2686(uint256) (->$_3 (-> ['TMP_12561'])) := cooldown_1(uint256)
TMP_12561(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_12561'])"])
 $.maxSlashVoteDurationInSeconds = maxSlashVoteDuration
REF_2687(uint256) -> $_3 (-> ['TMP_12561']).maxSlashVoteDurationInSeconds
$_4 (-> ['TMP_12561'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_12561'])"])
REF_2687(uint256) (->$_4 (-> ['TMP_12561'])) := maxSlashVoteDuration_1(uint256)
TMP_12561(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_12561'])"])
 $.maxAllowedValidatorCommission = maxValidatorCommission
REF_2688(uint256) -> $_4 (-> ['TMP_12561']).maxAllowedValidatorCommission
$_5 (-> ['TMP_12561'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_12561'])"])
REF_2688(uint256) (->$_5 (-> ['TMP_12561'])) := maxValidatorCommission_1(uint256)
TMP_12561(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_12561'])"])
 $.maxCommissionCheckpoints = 500
REF_2689(uint16) -> $_5 (-> ['TMP_12561']).maxCommissionCheckpoints
$_6 (-> ['TMP_12561'])(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_12561'])"])
REF_2689(uint16) (->$_6 (-> ['TMP_12561'])) := 500(uint256)
TMP_12561(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_12561'])"])
 $.initialized = true
REF_2690(bool) -> $_6 (-> ['TMP_12561']).initialized
$_7 (-> ['TMP_12561'])(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_12561'])"])
REF_2690(bool) (->$_7 (-> ['TMP_12561'])) := True(bool)
TMP_12561(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_12561'])"])
 onlyOwner()
MODIFIER_CALL, OwnableInternal.onlyOwner()()
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
