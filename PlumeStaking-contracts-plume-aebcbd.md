

#### PlumeStaking.initializePlume(address,uint256,uint256) [EXTERNAL][OWNER]
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
 initialOwner != address(0) && initialOwner != owner()
TMP_12566 = CONVERT 0 to address
TMP_12567(bool) = initialOwner_1 != TMP_12566
TMP_12568(address) = INTERNAL_CALL, Ownable.owner()()
TMP_12569(bool) = initialOwner_1 != TMP_12568
TMP_12570(bool) = TMP_12567 && TMP_12569
CONDITION TMP_12570
 _transferOwnership(initialOwner)
INTERNAL_CALL, SolidStateDiamond._transferOwnership(address)(initialOwner_1)
 $.minStakeAmount = minStake
REF_2683(uint256) -> $_1 (-> ['TMP_12561']).minStakeAmount
$_2 (-> ['TMP_12561'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_12561'])"])
REF_2683(uint256) (->$_2 (-> ['TMP_12561'])) := minStake_1(uint256)
TMP_12561(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12561'])"])
 $.cooldownInterval = cooldown
REF_2684(uint256) -> $_2 (-> ['TMP_12561']).cooldownInterval
$_3 (-> ['TMP_12561'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_12561'])"])
REF_2684(uint256) (->$_3 (-> ['TMP_12561'])) := cooldown_1(uint256)
TMP_12561(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_12561'])"])
 $.initialized = true
REF_2685(bool) -> $_3 (-> ['TMP_12561']).initialized
$_4 (-> ['TMP_12561'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_12561'])"])
REF_2685(bool) (->$_4 (-> ['TMP_12561'])) := True(bool)
TMP_12561(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_12561'])"])
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
