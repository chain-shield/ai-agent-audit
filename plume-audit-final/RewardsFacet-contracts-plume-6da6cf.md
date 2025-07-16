




#### RewardsFacet.getTreasuryAddress() [INTERNAL]
```slithir
TREASURY_STORAGE_POSITION_1(bytes32) := phi(['TREASURY_STORAGE_POSITION_0'])
 position = TREASURY_STORAGE_POSITION
position_1(bytes32) := TREASURY_STORAGE_POSITION_1(bytes32)
 treasuryAddress = sload(uint256)(position)
TMP_13147(uint256) = SOLIDITY_CALL sload(uint256)(position_1)
treasuryAddress_1(address) := TMP_13147(uint256)
 treasuryAddress
RETURN treasuryAddress_1
```
#### RewardsFacet.setTreasuryAddress(address) [INTERNAL]
```slithir
_treasury_1(address) := phi(['_treasury_1'])
TREASURY_STORAGE_POSITION_2(bytes32) := phi(['TREASURY_STORAGE_POSITION_0'])
 position = TREASURY_STORAGE_POSITION
position_1(bytes32) := TREASURY_STORAGE_POSITION_2(bytes32)
 sstore(uint256,uint256)(position,_treasury)
TMP_13148(None) = SOLIDITY_CALL sstore(uint256,uint256)(position_1,_treasury_1)
```
#### RewardsFacet._earned(address,address,uint16) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
token_1(address) := phi(['token_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13149(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13149'])(PlumeStakingStorage.Layout) := TMP_13149(PlumeStakingStorage.Layout)
 userStakedAmount = $.userValidatorStakes[user][validatorId].staked
REF_3087(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13149']).userValidatorStakes
REF_3088(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3087[user_1]
REF_3089(PlumeStakingStorage.UserValidatorStake) -> REF_3088[validatorId_1]
REF_3090(uint256) -> REF_3089.staked
userStakedAmount_1(uint256) := REF_3090(uint256)
 userStakedAmount == 0
TMP_13150(bool) = userStakedAmount_1 == 0
CONDITION TMP_13150
 $.userRewards[user][validatorId][token]
REF_3091(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13149']).userRewards
REF_3092(mapping(uint16 => mapping(address => uint256))) -> REF_3091[user_1]
REF_3093(mapping(address => uint256)) -> REF_3092[validatorId_1]
REF_3094(uint256) -> REF_3093[token_1]
RETURN REF_3094
 (userRewardDelta,None,None) = PlumeRewardLogic.calculateRewardsWithCheckpoints($,user,validatorId,token,userStakedAmount)
TUPLE_87(uint256,uint256,uint256) = LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.calculateRewardsWithCheckpoints(PlumeStakingStorage.Layout,address,uint16,address,uint256), arguments:["$_1 (-> ['TMP_13149'])", 'user_1', 'validatorId_1', 'token_1', 'userStakedAmount_1'] 
userRewardDelta_1(uint256)= UNPACK TUPLE_87 index: 0 
 rewards = $.userRewards[user][validatorId][token] + userRewardDelta
REF_3096(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13149']).userRewards
REF_3097(mapping(uint16 => mapping(address => uint256))) -> REF_3096[user_1]
REF_3098(mapping(address => uint256)) -> REF_3097[validatorId_1]
REF_3099(uint256) -> REF_3098[token_1]
TMP_13151(uint256) = REF_3099 (c)+ userRewardDelta_1
rewards_1(uint256) := TMP_13151(uint256)
 rewards
RETURN rewards_1
 rewards
```
#### RewardsFacet._calculateTotalEarned(address,address) [INTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13152(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13152'])(PlumeStakingStorage.Layout) := TMP_13152(PlumeStakingStorage.Layout)
 validatorIds = $.userValidators[user]
REF_3101(mapping(address => uint16[])) -> $_1 (-> ['TMP_13152']).userValidators
REF_3102(uint16[]) -> REF_3101[user_1]
validatorIds_1(uint16[]) = ['REF_3102(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
totalEarned_1(uint256) := phi(['totalEarned_2', 'totalEarned_0'])
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3103 -> LENGTH validatorIds_1
TMP_13153(bool) = i_2 < REF_3103
CONDITION TMP_13153
 validatorId = validatorIds[i]
REF_3104(uint16) -> validatorIds_1[i_2]
validatorId_1(uint16) := REF_3104(uint16)
 totalEarned += _earned(user,token,validatorId)
TMP_13154(uint256) = INTERNAL_CALL, RewardsFacet._earned(address,address,uint16)(user_1,token_1,validatorId_1)
totalEarned_2(uint256) = totalEarned_1 (c)+ TMP_13154
 i ++
TMP_13155(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 totalEarned
RETURN totalEarned_1
 totalEarned
```
#### RewardsFacet.setTreasury(address) [EXTERNAL]
```slithir
 _treasury == address(0)
TMP_13156 = CONVERT 0 to address
TMP_13157(bool) = _treasury_1 == TMP_13156
CONDITION TMP_13157
 revert ZeroAddress(string)(treasury)
TMP_13158(None) = SOLIDITY_CALL revert ZeroAddress(string)(treasury)
 setTreasuryAddress(_treasury)
INTERNAL_CALL, RewardsFacet.setTreasuryAddress(address)(_treasury_1)
 TreasurySet(_treasury)
Emit TreasurySet(_treasury_1)
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_3105(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3105)
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_3106(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3106)
```
#### RewardsFacet.addRewardToken(address,uint256,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13163(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13163'])(PlumeStakingStorage.Layout) := TMP_13163(PlumeStakingStorage.Layout)
 token == address(0)
TMP_13164 = CONVERT 0 to address
TMP_13165(bool) = token_1 == TMP_13164
CONDITION TMP_13165
 revert ZeroAddress(string)(token)
TMP_13166(None) = SOLIDITY_CALL revert ZeroAddress(string)(token)
 $.isRewardToken[token]
REF_3108(mapping(address => bool)) -> $_1 (-> ['TMP_13163']).isRewardToken
REF_3109(bool) -> REF_3108[token_1]
CONDITION REF_3109
 revert TokenAlreadyExists()()
TMP_13167(None) = SOLIDITY_CALL revert TokenAlreadyExists()()
 initialRate > maxRate
TMP_13168(bool) = initialRate_1 > maxRate_1
CONDITION TMP_13168
 revert RewardRateExceedsMax()()
TMP_13169(None) = SOLIDITY_CALL revert RewardRateExceedsMax()()
 $.tokenRemovalTimestamps[token] == block.timestamp
REF_3110(mapping(address => uint256)) -> $_1 (-> ['TMP_13163']).tokenRemovalTimestamps
REF_3111(uint256) -> REF_3110[token_1]
TMP_13170(bool) = REF_3111 == block.timestamp
CONDITION TMP_13170
 revert CannotReAddTokenInSameBlock(address)(token)
TMP_13171(None) = SOLIDITY_CALL revert CannotReAddTokenInSameBlock(address)(token_1)
 ! $.isHistoricalRewardToken[token]
REF_3112(mapping(address => bool)) -> $_1 (-> ['TMP_13163']).isHistoricalRewardToken
REF_3113(bool) -> REF_3112[token_1]
TMP_13172 = UnaryType.BANG REF_3113 
CONDITION TMP_13172
 $.isHistoricalRewardToken[token] = true
REF_3114(mapping(address => bool)) -> $_1 (-> ['TMP_13163']).isHistoricalRewardToken
REF_3115(bool) -> REF_3114[token_1]
$_2 (-> ['TMP_13163'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13163'])"])
REF_3115(bool) (->$_2 (-> ['TMP_13163'])) := True(bool)
TMP_13163(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13163'])"])
 $.historicalRewardTokens.push(token)
REF_3116(address[]) -> $_2 (-> ['TMP_13163']).historicalRewardTokens
REF_3118 -> LENGTH REF_3116
TMP_13174(uint256) := REF_3118(uint256)
TMP_13175(uint256) = TMP_13174 (c)+ 1
$_3 (-> ['TMP_13163'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13163'])"])
REF_3118(uint256) (->$_4 (-> ['TMP_13163'])) := TMP_13175(uint256)
REF_3119(address) -> REF_3116[TMP_13174]
$_4 (-> ['TMP_13163'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13163'])"])
REF_3119(address) (->$_4 (-> ['TMP_13163'])) := token_1(address)
TMP_13163(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13163'])"])
$_5 (-> ['TMP_13163'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13163'])", "$_1 (-> ['TMP_13163'])"])
 additionTimestamp = block.timestamp
additionTimestamp_1(uint256) := block.timestamp(uint256)
 $.tokenRemovalTimestamps[token] = 0
REF_3120(mapping(address => uint256)) -> $_5 (-> ['TMP_13163']).tokenRemovalTimestamps
REF_3121(uint256) -> REF_3120[token_1]
$_6 (-> ['TMP_13163'])(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13163'])"])
REF_3121(uint256) (->$_6 (-> ['TMP_13163'])) := 0(uint256)
TMP_13163(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13163'])"])
 $.rewardTokens.push(token)
REF_3122(address[]) -> $_6 (-> ['TMP_13163']).rewardTokens
REF_3124 -> LENGTH REF_3122
TMP_13177(uint256) := REF_3124(uint256)
TMP_13178(uint256) = TMP_13177 (c)+ 1
$_7 (-> ['TMP_13163'])(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13163'])"])
REF_3124(uint256) (->$_8 (-> ['TMP_13163'])) := TMP_13178(uint256)
REF_3125(address) -> REF_3122[TMP_13177]
$_8 (-> ['TMP_13163'])(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_13163'])"])
REF_3125(address) (->$_8 (-> ['TMP_13163'])) := token_1(address)
TMP_13163(PlumeStakingStorage.Layout) := phi(["$_8 (-> ['TMP_13163'])"])
 $.isRewardToken[token] = true
REF_3126(mapping(address => bool)) -> $_8 (-> ['TMP_13163']).isRewardToken
REF_3127(bool) -> REF_3126[token_1]
$_9 (-> ['TMP_13163'])(PlumeStakingStorage.Layout) := phi(["$_8 (-> ['TMP_13163'])"])
REF_3127(bool) (->$_9 (-> ['TMP_13163'])) := True(bool)
TMP_13163(PlumeStakingStorage.Layout) := phi(["$_9 (-> ['TMP_13163'])"])
 $.maxRewardRates[token] = maxRate
REF_3128(mapping(address => uint256)) -> $_9 (-> ['TMP_13163']).maxRewardRates
REF_3129(uint256) -> REF_3128[token_1]
$_10 (-> ['TMP_13163'])(PlumeStakingStorage.Layout) := phi(["$_9 (-> ['TMP_13163'])"])
REF_3129(uint256) (->$_10 (-> ['TMP_13163'])) := maxRate_1(uint256)
TMP_13163(PlumeStakingStorage.Layout) := phi(["$_10 (-> ['TMP_13163'])"])
 $.rewardRates[token] = initialRate
REF_3130(mapping(address => uint256)) -> $_10 (-> ['TMP_13163']).rewardRates
REF_3131(uint256) -> REF_3130[token_1]
$_11 (-> ['TMP_13163'])(PlumeStakingStorage.Layout) := phi(["$_10 (-> ['TMP_13163'])"])
REF_3131(uint256) (->$_11 (-> ['TMP_13163'])) := initialRate_1(uint256)
TMP_13163(PlumeStakingStorage.Layout) := phi(["$_11 (-> ['TMP_13163'])"])
 $.tokenAdditionTimestamps[token] = additionTimestamp
REF_3132(mapping(address => uint256)) -> $_11 (-> ['TMP_13163']).tokenAdditionTimestamps
REF_3133(uint256) -> REF_3132[token_1]
$_12 (-> ['TMP_13163'])(PlumeStakingStorage.Layout) := phi(["$_11 (-> ['TMP_13163'])"])
REF_3133(uint256) (->$_12 (-> ['TMP_13163'])) := additionTimestamp_1(uint256)
TMP_13163(PlumeStakingStorage.Layout) := phi(["$_12 (-> ['TMP_13163'])"])
 validatorIds = $.validatorIds
REF_3134(uint16[]) -> $_12 (-> ['TMP_13163']).validatorIds
validatorIds_1(uint16[]) = ['REF_3134(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3135 -> LENGTH validatorIds_1
TMP_13179(bool) = i_2 < REF_3135
CONDITION TMP_13179
 validatorId = validatorIds[i]
REF_3136(uint16) -> validatorIds_1[i_2]
validatorId_1(uint16) := REF_3136(uint16)
 PlumeRewardLogic.createRewardRateCheckpoint($,token,validatorId,initialRate)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.createRewardRateCheckpoint(PlumeStakingStorage.Layout,address,uint16,uint256), arguments:["$_12 (-> ['TMP_13163'])", 'token_1', 'validatorId_1', 'initialRate_1'] 
 i ++
TMP_13181(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 RewardTokenAdded(token)
Emit RewardTokenAdded(token_1)
 maxRate > 0
TMP_13183(bool) = maxRate_1 > 0
CONDITION TMP_13183
 MaxRewardRateUpdated(token,maxRate)
Emit MaxRewardRateUpdated(token_1,maxRate_1)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3138(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3138)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3139(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3139)
```
#### RewardsFacet.removeRewardToken(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13187(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13187'])(PlumeStakingStorage.Layout) := TMP_13187(PlumeStakingStorage.Layout)
 ! $.isRewardToken[token]
REF_3141(mapping(address => bool)) -> $_1 (-> ['TMP_13187']).isRewardToken
REF_3142(bool) -> REF_3141[token_1]
TMP_13188 = UnaryType.BANG REF_3142 
CONDITION TMP_13188
 revert TokenDoesNotExist(address)(token)
TMP_13189(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 tokenIndex = _getTokenIndex(token)
TMP_13190(uint256) = INTERNAL_CALL, RewardsFacet._getTokenIndex(address)(token_1)
tokenIndex_1(uint256) := TMP_13190(uint256)
 removalTimestamp = block.timestamp
removalTimestamp_1(uint256) := block.timestamp(uint256)
 $.tokenRemovalTimestamps[token] = removalTimestamp
REF_3143(mapping(address => uint256)) -> $_1 (-> ['TMP_13187']).tokenRemovalTimestamps
REF_3144(uint256) -> REF_3143[token_1]
$_2 (-> ['TMP_13187'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13187'])"])
REF_3144(uint256) (->$_2 (-> ['TMP_13187'])) := removalTimestamp_1(uint256)
TMP_13187(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13187'])"])
 i = 0
i_1(uint256) := 0(uint256)
 i < $.validatorIds.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3145(uint16[]) -> $_2 (-> ['TMP_13187']).validatorIds
REF_3146 -> LENGTH REF_3145
TMP_13191(bool) = i_2 < REF_3146
CONDITION TMP_13191
 validatorId = $.validatorIds[i]
REF_3147(uint16[]) -> $_2 (-> ['TMP_13187']).validatorIds
REF_3148(uint16) -> REF_3147[i_2]
validatorId_1(uint16) := REF_3148(uint16)
 PlumeRewardLogic.updateRewardPerTokenForValidator($,token,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_2 (-> ['TMP_13187'])", 'token_1', 'validatorId_1'] 
 PlumeRewardLogic.createRewardRateCheckpoint($,token,validatorId,0)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.createRewardRateCheckpoint(PlumeStakingStorage.Layout,address,uint16,uint256), arguments:["$_2 (-> ['TMP_13187'])", 'token_1', 'validatorId_1', '0'] 
 i ++
TMP_13194(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 $.rewardRates[token] = 0
REF_3151(mapping(address => uint256)) -> $_2 (-> ['TMP_13187']).rewardRates
REF_3152(uint256) -> REF_3151[token_1]
$_3 (-> ['TMP_13187'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13187'])"])
REF_3152(uint256) (->$_3 (-> ['TMP_13187'])) := 0(uint256)
TMP_13187(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13187'])"])
 $.rewardTokens[tokenIndex] = $.rewardTokens[$.rewardTokens.length - 1]
REF_3153(address[]) -> $_3 (-> ['TMP_13187']).rewardTokens
REF_3154(address) -> REF_3153[tokenIndex_1]
REF_3155(address[]) -> $_3 (-> ['TMP_13187']).rewardTokens
REF_3156(address[]) -> $_3 (-> ['TMP_13187']).rewardTokens
REF_3157 -> LENGTH REF_3156
TMP_13195(uint256) = REF_3157 (c)- 1
REF_3158(address) -> REF_3155[TMP_13195]
$_4 (-> ['TMP_13187'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13187'])"])
REF_3154(address) (->$_4 (-> ['TMP_13187'])) := REF_3158(address)
TMP_13187(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13187'])"])
 $.rewardTokens.pop()
REF_3159(address[]) -> $_4 (-> ['TMP_13187']).rewardTokens
REF_3161 -> LENGTH REF_3159
TMP_13197(uint256) = REF_3161 (c)- 1
REF_3162(address) -> REF_3159[TMP_13197]
REF_3159 = delete REF_3162 
REF_3163 -> LENGTH REF_3159
$_5 (-> ['TMP_13187'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13187'])"])
REF_3163(uint256) (->$_5 (-> ['TMP_13187'])) := TMP_13197(uint256)
TMP_13187(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13187'])"])
 $.isRewardToken[token] = false
REF_3164(mapping(address => bool)) -> $_5 (-> ['TMP_13187']).isRewardToken
REF_3165(bool) -> REF_3164[token_1]
$_6 (-> ['TMP_13187'])(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13187'])"])
REF_3165(bool) (->$_6 (-> ['TMP_13187'])) := False(bool)
TMP_13187(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13187'])"])
 delete $.maxRewardRates[token]
REF_3166(mapping(address => uint256)) -> $_6 (-> ['TMP_13187']).maxRewardRates
REF_3167(uint256) -> REF_3166[token_1]
REF_3166 = delete REF_3167 
 RewardTokenRemoved(token)
Emit RewardTokenRemoved(token_1)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3168(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3168)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3169(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3169)
```
#### RewardsFacet.setRewardRates(address[],uint256[]) [EXTERNAL]
```slithir
MAX_REWARD_RATE_1(uint256) := phi(['MAX_REWARD_RATE_0', 'MAX_REWARD_RATE_3'])
 $ = PlumeStakingStorage.layout()
TMP_13201(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13201'])(PlumeStakingStorage.Layout) := TMP_13201(PlumeStakingStorage.Layout)
 tokens.length == 0
REF_3171 -> LENGTH tokens_1
TMP_13202(bool) = REF_3171 == 0
CONDITION TMP_13202
 revert EmptyArray()()
TMP_13203(None) = SOLIDITY_CALL revert EmptyArray()()
 tokens.length != rewardRates_.length
REF_3172 -> LENGTH tokens_1
REF_3173 -> LENGTH rewardRates__1
TMP_13204(bool) = REF_3172 != REF_3173
CONDITION TMP_13204
 revert ArrayLengthMismatch()()
TMP_13205(None) = SOLIDITY_CALL revert ArrayLengthMismatch()()
 validatorIds = $.validatorIds
REF_3174(uint16[]) -> $_1 (-> ['TMP_13201']).validatorIds
validatorIds_1(uint16[]) = ['REF_3174(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < tokens.length
$_2 (-> ['TMP_13201'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13201'])", "$_1 (-> ['TMP_13201'])"])
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3175 -> LENGTH tokens_1
TMP_13206(bool) = i_2 < REF_3175
CONDITION TMP_13206
 token_loop = tokens[i]
REF_3176(address) -> tokens_1[i_2]
token_loop_1(address) := REF_3176(address)
 rate_loop = rewardRates_[i]
REF_3177(uint256) -> rewardRates__1[i_2]
rate_loop_1(uint256) := REF_3177(uint256)
 ! $.isRewardToken[token_loop]
REF_3178(mapping(address => bool)) -> $_2 (-> ['TMP_13201']).isRewardToken
REF_3179(bool) -> REF_3178[token_loop_1]
TMP_13207 = UnaryType.BANG REF_3179 
CONDITION TMP_13207
 revert TokenDoesNotExist(address)(token_loop)
TMP_13208(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_loop_1)
 rate_loop > maxRate
TMP_13209(bool) = rate_loop_1 > maxRate_3
CONDITION TMP_13209
 revert RewardRateExceedsMax()()
TMP_13210(None) = SOLIDITY_CALL revert RewardRateExceedsMax()()
 j = 0
j_1(uint256) := 0(uint256)
 j < validatorIds.length
j_2(uint256) := phi(['j_1', 'j_3'])
REF_3180 -> LENGTH validatorIds_1
TMP_13211(bool) = j_2 < REF_3180
CONDITION TMP_13211
 validatorId_for_crrc = validatorIds[j]
REF_3181(uint16) -> validatorIds_1[j_2]
validatorId_for_crrc_1(uint16) := REF_3181(uint16)
 PlumeRewardLogic.createRewardRateCheckpoint($,token_loop,validatorId_for_crrc,rate_loop)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.createRewardRateCheckpoint(PlumeStakingStorage.Layout,address,uint16,uint256), arguments:["$_2 (-> ['TMP_13201'])", 'token_loop_1', 'validatorId_for_crrc_1', 'rate_loop_1'] 
 j ++
TMP_13213(uint256) := j_2(uint256)
j_3(uint256) = j_2 (c)+ 1
 $.rewardRates[token_loop] = rate_loop
REF_3183(mapping(address => uint256)) -> $_2 (-> ['TMP_13201']).rewardRates
REF_3184(uint256) -> REF_3183[token_loop_1]
$_3 (-> ['TMP_13201'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13201'])"])
REF_3184(uint256) (->$_3 (-> ['TMP_13201'])) := rate_loop_1(uint256)
TMP_13201(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13201'])"])
 i ++
TMP_13214(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 RewardRatesSet(tokens,rewardRates_)
Emit RewardRatesSet(tokens_1,rewardRates__1)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3185(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3185)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3186(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3186)
 $.maxRewardRates[token_loop] > 0
REF_3187(mapping(address => uint256)) -> $_2 (-> ['TMP_13201']).maxRewardRates
REF_3188(uint256) -> REF_3187[token_loop_1]
TMP_13218(bool) = REF_3188 > 0
CONDITION TMP_13218
 maxRate = $.maxRewardRates[token_loop]
REF_3189(mapping(address => uint256)) -> $_2 (-> ['TMP_13201']).maxRewardRates
REF_3190(uint256) -> REF_3189[token_loop_1]
maxRate_1(uint256) := REF_3190(uint256)
 maxRate = MAX_REWARD_RATE
maxRate_2(uint256) := MAX_REWARD_RATE_3(uint256)
maxRate_3(uint256) := phi(['maxRate_1', 'maxRate_2'])
```
#### RewardsFacet.setMaxRewardRate(address,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13219(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13219'])(PlumeStakingStorage.Layout) := TMP_13219(PlumeStakingStorage.Layout)
 ! $.isRewardToken[token]
REF_3192(mapping(address => bool)) -> $_1 (-> ['TMP_13219']).isRewardToken
REF_3193(bool) -> REF_3192[token_1]
TMP_13220 = UnaryType.BANG REF_3193 
CONDITION TMP_13220
 revert TokenDoesNotExist(address)(token)
TMP_13221(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 $.rewardRates[token] > newMaxRate
REF_3194(mapping(address => uint256)) -> $_1 (-> ['TMP_13219']).rewardRates
REF_3195(uint256) -> REF_3194[token_1]
TMP_13222(bool) = REF_3195 > newMaxRate_1
CONDITION TMP_13222
 $.rewardRates[token] = newMaxRate
REF_3196(mapping(address => uint256)) -> $_1 (-> ['TMP_13219']).rewardRates
REF_3197(uint256) -> REF_3196[token_1]
$_2 (-> ['TMP_13219'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13219'])"])
REF_3197(uint256) (->$_2 (-> ['TMP_13219'])) := newMaxRate_1(uint256)
TMP_13219(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13219'])"])
 validatorIds = $.validatorIds
REF_3198(uint16[]) -> $_2 (-> ['TMP_13219']).validatorIds
validatorIds_1(uint16[]) = ['REF_3198(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3199 -> LENGTH validatorIds_1
TMP_13223(bool) = i_2 < REF_3199
CONDITION TMP_13223
 PlumeRewardLogic.createRewardRateCheckpoint($,token,validatorIds[i],newMaxRate)
REF_3201(uint16) -> validatorIds_1[i_2]
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.createRewardRateCheckpoint(PlumeStakingStorage.Layout,address,uint16,uint256), arguments:["$_2 (-> ['TMP_13219'])", 'token_1', 'REF_3201', 'newMaxRate_1'] 
 i ++
TMP_13225(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
$_3 (-> ['TMP_13219'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13219'])", "$_1 (-> ['TMP_13219'])"])
 $.maxRewardRates[token] = newMaxRate
REF_3202(mapping(address => uint256)) -> $_3 (-> ['TMP_13219']).maxRewardRates
REF_3203(uint256) -> REF_3202[token_1]
$_4 (-> ['TMP_13219'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13219'])"])
REF_3203(uint256) (->$_4 (-> ['TMP_13219'])) := newMaxRate_1(uint256)
TMP_13219(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13219'])"])
 MaxRewardRateUpdated(token,newMaxRate)
Emit MaxRewardRateUpdated(token_1,newMaxRate_1)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3204(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3204)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3205(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3205)
```
#### RewardsFacet.claim(address) [EXTERNAL]
```slithir
 _validateTokenForClaim(token,msg.sender)
TMP_13238(bool) = INTERNAL_CALL, RewardsFacet._validateTokenForClaim(address,address)(token_1,msg.sender)
 totalReward = _processAllValidatorRewards(msg.sender,token)
TMP_13239(uint256) = INTERNAL_CALL, RewardsFacet._processAllValidatorRewards(address,address)(msg.sender,token_1)
totalReward_1(uint256) := TMP_13239(uint256)
 totalReward > 0
TMP_13240(bool) = totalReward_1 > 0
CONDITION TMP_13240
 _finalizeRewardClaim(token,totalReward,msg.sender)
INTERNAL_CALL, RewardsFacet._finalizeRewardClaim(address,uint256,address)(token_1,totalReward_1,msg.sender)
 RewardClaimed(msg.sender,token,totalReward)
Emit RewardClaimed(msg.sender,token_1,totalReward_1)
 $ = PlumeStakingStorage.layout()
TMP_13243(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13243'])(PlumeStakingStorage.Layout) := TMP_13243(PlumeStakingStorage.Layout)
 validatorIds = $.userValidators[msg.sender]
REF_3210(mapping(address => uint16[])) -> $_1 (-> ['TMP_13243']).userValidators
REF_3211(uint16[]) -> REF_3210[msg.sender]
validatorIds_1(uint16[]) = ['REF_3211(uint16[])']
 _clearPendingRewardFlags(msg.sender,validatorIds)
INTERNAL_CALL, RewardsFacet._clearPendingRewardFlags(address,uint16[])(msg.sender,validatorIds_1)
 PlumeValidatorLogic.removeStakerFromAllValidators($,msg.sender)
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.removeStakerFromAllValidators(PlumeStakingStorage.Layout,address), arguments:["$_1 (-> ['TMP_13243'])", 'msg.sender'] 
 totalReward
RETURN totalReward_1
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### RewardsFacet.claimAll() [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13247(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13247'])(PlumeStakingStorage.Layout) := TMP_13247(PlumeStakingStorage.Layout)
 tokens = $.rewardTokens
REF_3214(address[]) -> $_1 (-> ['TMP_13247']).rewardTokens
tokens_1(address[]) = ['REF_3214(address[])']
 claims = new uint256[](tokens.length)
REF_3215 -> LENGTH tokens_1
TMP_13249(uint256[])  = new uint256[](REF_3215)
claims_1(uint256[]) = ['TMP_13249(uint256[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < tokens.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3216 -> LENGTH tokens_1
TMP_13250(bool) = i_2 < REF_3216
CONDITION TMP_13250
 token = tokens[i]
REF_3217(address) -> tokens_1[i_2]
token_1(address) := REF_3217(address)
 totalReward = _processAllValidatorRewards(msg.sender,token)
TMP_13251(uint256) = INTERNAL_CALL, RewardsFacet._processAllValidatorRewards(address,address)(msg.sender,token_1)
totalReward_1(uint256) := TMP_13251(uint256)
 totalReward > 0
TMP_13252(bool) = totalReward_1 > 0
CONDITION TMP_13252
 _finalizeRewardClaim(token,totalReward,msg.sender)
INTERNAL_CALL, RewardsFacet._finalizeRewardClaim(address,uint256,address)(token_1,totalReward_1,msg.sender)
 claims[i] = totalReward
REF_3218(uint256) -> claims_1[i_2]
claims_2(uint256[]) := phi(['claims_1'])
REF_3218(uint256) (->claims_2) := totalReward_1(uint256)
 RewardClaimed(msg.sender,token,totalReward)
Emit RewardClaimed(msg.sender,token_1,totalReward_1)
 i ++
TMP_13255(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 validatorIds = $.userValidators[msg.sender]
REF_3219(mapping(address => uint16[])) -> $_1 (-> ['TMP_13247']).userValidators
REF_3220(uint16[]) -> REF_3219[msg.sender]
validatorIds_1(uint16[]) = ['REF_3220(uint16[])']
 _clearPendingRewardFlags(msg.sender,validatorIds)
INTERNAL_CALL, RewardsFacet._clearPendingRewardFlags(address,uint16[])(msg.sender,validatorIds_1)
 PlumeValidatorLogic.removeStakerFromAllValidators($,msg.sender)
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.removeStakerFromAllValidators(PlumeStakingStorage.Layout,address), arguments:["$_1 (-> ['TMP_13247'])", 'msg.sender'] 
 claims
RETURN claims_1
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### RewardsFacet._validateTokenForClaim(address,address) [INTERNAL]
```slithir
token_1(address) := phi(['token_1', 'token_1'])
user_1(address) := phi(['msg.sender'])
 $ = PlumeStakingStorage.layout()
TMP_13259(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13259'])(PlumeStakingStorage.Layout) := TMP_13259(PlumeStakingStorage.Layout)
 isActive = $.isRewardToken[token]
REF_3223(mapping(address => bool)) -> $_1 (-> ['TMP_13259']).isRewardToken
REF_3224(bool) -> REF_3223[token_1]
isActive_1(bool) := REF_3224(bool)
 ! isActive
TMP_13260 = UnaryType.BANG isActive_1 
CONDITION TMP_13260
 validatorIds = $.userValidators[user]
REF_3225(mapping(address => uint16[])) -> $_1 (-> ['TMP_13259']).userValidators
REF_3226(uint16[]) -> REF_3225[user_1]
validatorIds_1(uint16[]) = ['REF_3226(uint16[])']
 hasRewards = false
hasRewards_1(bool) := False(bool)
hasRewards_4(bool) := phi(['hasRewards_1', 'hasRewards_3', 'hasRewards_2'])
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3227 -> LENGTH validatorIds_1
TMP_13261(bool) = i_2 < REF_3227
CONDITION TMP_13261
 validatorId = validatorIds[i]
REF_3228(uint16) -> validatorIds_1[i_2]
validatorId_1(uint16) := REF_3228(uint16)
 $.userRewards[user][validatorId][token] > 0
REF_3229(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13259']).userRewards
REF_3230(mapping(uint16 => mapping(address => uint256))) -> REF_3229[user_1]
REF_3231(mapping(address => uint256)) -> REF_3230[validatorId_1]
REF_3232(uint256) -> REF_3231[token_1]
TMP_13262(bool) = REF_3232 > 0
CONDITION TMP_13262
 hasRewards = true
hasRewards_2(bool) := True(bool)
 userStakedAmount = $.userValidatorStakes[user][validatorId].staked
REF_3233(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13259']).userValidatorStakes
REF_3234(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3233[user_1]
REF_3235(PlumeStakingStorage.UserValidatorStake) -> REF_3234[validatorId_1]
REF_3236(uint256) -> REF_3235.staked
userStakedAmount_1(uint256) := REF_3236(uint256)
 userStakedAmount > 0
TMP_13263(bool) = userStakedAmount_1 > 0
CONDITION TMP_13263
 (userRewardDelta,None,None) = PlumeRewardLogic.calculateRewardsWithCheckpointsView($,user,validatorId,token,userStakedAmount)
TUPLE_88(uint256,uint256,uint256) = LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.calculateRewardsWithCheckpointsView(PlumeStakingStorage.Layout,address,uint16,address,uint256), arguments:["$_1 (-> ['TMP_13259'])", 'user_1', 'validatorId_1', 'token_1', 'userStakedAmount_1'] 
userRewardDelta_1(uint256)= UNPACK TUPLE_88 index: 0 
 userRewardDelta > 0
TMP_13264(bool) = userRewardDelta_1 > 0
CONDITION TMP_13264
 hasRewards = true
hasRewards_3(bool) := True(bool)
 i ++
TMP_13265(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 ! hasRewards
TMP_13266 = UnaryType.BANG hasRewards_4 
CONDITION TMP_13266
 revert TokenDoesNotExist(address)(token)
TMP_13267(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 isActive
RETURN isActive_1
```
#### RewardsFacet._validateValidatorForClaim(uint16) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13268(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13268'])(PlumeStakingStorage.Layout) := TMP_13268(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_3239(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13268']).validatorExists
REF_3240(bool) -> REF_3239[validatorId_1]
TMP_13269 = UnaryType.BANG REF_3240 
CONDITION TMP_13269
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13270(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
```
#### RewardsFacet._processValidatorRewards(address,uint16,address) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'msg.sender'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
token_1(address) := phi(['token_1', 'token_1'])
 $ = PlumeStakingStorage.layout()
TMP_13271(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13271'])(PlumeStakingStorage.Layout) := TMP_13271(PlumeStakingStorage.Layout)
 PlumeRewardLogic.updateRewardsForValidatorAndToken($,user,validatorId,token)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardsForValidatorAndToken(PlumeStakingStorage.Layout,address,uint16,address), arguments:["$_1 (-> ['TMP_13271'])", 'user_1', 'validatorId_1', 'token_1'] 
 reward = $.userRewards[user][validatorId][token]
REF_3243(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13271']).userRewards
REF_3244(mapping(uint16 => mapping(address => uint256))) -> REF_3243[user_1]
REF_3245(mapping(address => uint256)) -> REF_3244[validatorId_1]
REF_3246(uint256) -> REF_3245[token_1]
reward_1(uint256) := REF_3246(uint256)
 reward > 0
TMP_13273(bool) = reward_1 > 0
CONDITION TMP_13273
 _updateUserRewardState(user,validatorId,token)
INTERNAL_CALL, RewardsFacet._updateUserRewardState(address,uint16,address)(user_1,validatorId_1,token_1)
 RewardClaimedFromValidator(user,token,validatorId,reward)
Emit RewardClaimedFromValidator(user_1,token_1,validatorId_1,reward_1)
 reward
RETURN reward_1
```
#### RewardsFacet._updateUserRewardState(address,uint16,address) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
token_1(address) := phi(['token_1'])
 $ = PlumeStakingStorage.layout()
TMP_13276(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13276'])(PlumeStakingStorage.Layout) := TMP_13276(PlumeStakingStorage.Layout)
 $.userRewards[user][validatorId][token] = 0
REF_3248(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13276']).userRewards
REF_3249(mapping(uint16 => mapping(address => uint256))) -> REF_3248[user_1]
REF_3250(mapping(address => uint256)) -> REF_3249[validatorId_1]
REF_3251(uint256) -> REF_3250[token_1]
$_2 (-> ['TMP_13276'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13276'])"])
REF_3251(uint256) (->$_2 (-> ['TMP_13276'])) := 0(uint256)
TMP_13276(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13276'])"])
```
#### RewardsFacet._finalizeRewardClaim(address,uint256,address) [INTERNAL]
```slithir
token_1(address) := phi(['token_1', 'token_1', 'token_1'])
totalAmount_1(uint256) := phi(['totalReward_1', 'reward_1', 'totalReward_1'])
recipient_1(address) := phi(['msg.sender'])
 totalAmount == 0
TMP_13277(bool) = totalAmount_1 == 0
CONDITION TMP_13277
 $ = PlumeStakingStorage.layout()
TMP_13278(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13278'])(PlumeStakingStorage.Layout) := TMP_13278(PlumeStakingStorage.Layout)
 $.totalClaimableByToken[token] >= totalAmount
REF_3253(mapping(address => uint256)) -> $_1 (-> ['TMP_13278']).totalClaimableByToken
REF_3254(uint256) -> REF_3253[token_1]
TMP_13279(bool) = REF_3254 >= totalAmount_1
CONDITION TMP_13279
 $.totalClaimableByToken[token] -= totalAmount
REF_3255(mapping(address => uint256)) -> $_1 (-> ['TMP_13278']).totalClaimableByToken
REF_3256(uint256) -> REF_3255[token_1]
$_2 (-> ['TMP_13278'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13278'])"])
REF_3256(-> $_2 (-> ['TMP_13278'])) = REF_3256 (c)- totalAmount_1
TMP_13278(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13278'])"])
 $.totalClaimableByToken[token] = 0
REF_3257(mapping(address => uint256)) -> $_1 (-> ['TMP_13278']).totalClaimableByToken
REF_3258(uint256) -> REF_3257[token_1]
$_3 (-> ['TMP_13278'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13278'])"])
REF_3258(uint256) (->$_3 (-> ['TMP_13278'])) := 0(uint256)
TMP_13278(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13278'])"])
 _transferRewardFromTreasury(token,totalAmount,recipient)
INTERNAL_CALL, RewardsFacet._transferRewardFromTreasury(address,uint256,address)(token_1,totalAmount_1,recipient_1)
```
#### RewardsFacet._clearPendingRewardFlags(address,uint16[]) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender'])
validatorIds_1(uint16[]) := phi(['validatorIds_1', 'validatorIds_1'])
 $ = PlumeStakingStorage.layout()
TMP_13281(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13281'])(PlumeStakingStorage.Layout) := TMP_13281(PlumeStakingStorage.Layout)
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3260 -> LENGTH validatorIds_1
TMP_13282(bool) = i_2 < REF_3260
CONDITION TMP_13282
 PlumeRewardLogic.clearPendingRewardsFlagIfEmpty($,user,validatorIds[i])
REF_3262(uint16) -> validatorIds_1[i_2]
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.clearPendingRewardsFlagIfEmpty(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13281'])", 'user_1', 'REF_3262'] 
 i ++
TMP_13284(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### RewardsFacet._processAllValidatorRewards(address,address) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender'])
token_1(address) := phi(['token_1', 'token_1'])
 $ = PlumeStakingStorage.layout()
TMP_13285(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13285'])(PlumeStakingStorage.Layout) := TMP_13285(PlumeStakingStorage.Layout)
 validatorIds = $.userValidators[user]
REF_3264(mapping(address => uint16[])) -> $_1 (-> ['TMP_13285']).userValidators
REF_3265(uint16[]) -> REF_3264[user_1]
validatorIds_1(uint16[]) = ['REF_3265(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
totalReward_1(uint256) := phi(['totalReward_2', 'totalReward_0'])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3266 -> LENGTH validatorIds_1
TMP_13286(bool) = i_2 < REF_3266
CONDITION TMP_13286
 validatorId = validatorIds[i]
REF_3267(uint16) -> validatorIds_1[i_2]
validatorId_1(uint16) := REF_3267(uint16)
 rewardFromValidator = _processValidatorRewards(user,validatorId,token)
TMP_13287(uint256) = INTERNAL_CALL, RewardsFacet._processValidatorRewards(address,uint16,address)(user_1,validatorId_1,token_1)
rewardFromValidator_1(uint256) := TMP_13287(uint256)
 totalReward += rewardFromValidator
totalReward_2(uint256) = totalReward_1 (c)+ rewardFromValidator_1
 i ++
TMP_13288(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 totalReward
RETURN totalReward_1
 totalReward
```
#### RewardsFacet._transferRewardFromTreasury(address,uint256,address) [INTERNAL]
```slithir
token_1(address) := phi(['token_1'])
amount_1(uint256) := phi(['totalAmount_1'])
recipient_1(address) := phi(['recipient_1'])
 treasury = getTreasuryAddress()
TMP_13289(address) = INTERNAL_CALL, RewardsFacet.getTreasuryAddress()()
treasury_1(address) := TMP_13289(address)
 treasury == address(0)
TMP_13290 = CONVERT 0 to address
TMP_13291(bool) = treasury_1 == TMP_13290
CONDITION TMP_13291
 revert TreasuryNotSet()()
TMP_13292(None) = SOLIDITY_CALL revert TreasuryNotSet()()
 IPlumeStakingRewardTreasury(treasury).distributeReward(token,amount,recipient)
TMP_13293 = CONVERT treasury_1 to IPlumeStakingRewardTreasury
HIGH_LEVEL_CALL, dest:TMP_13293(IPlumeStakingRewardTreasury), function:distributeReward, arguments:['token_1', 'amount_1', 'recipient_1']
```
#### RewardsFacet._isRewardToken(address) [INTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13295(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13295'])(PlumeStakingStorage.Layout) := TMP_13295(PlumeStakingStorage.Layout)
 $.isRewardToken[token]
REF_3270(mapping(address => bool)) -> $_1 (-> ['TMP_13295']).isRewardToken
REF_3271(bool) -> REF_3270[token_1]
RETURN REF_3271
```
#### RewardsFacet._getTokenIndex(address) [INTERNAL]
```slithir
token_1(address) := phi(['token_1'])
 $ = PlumeStakingStorage.layout()
TMP_13296(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13296'])(PlumeStakingStorage.Layout) := TMP_13296(PlumeStakingStorage.Layout)
 ! $.isRewardToken[token]
REF_3273(mapping(address => bool)) -> $_1 (-> ['TMP_13296']).isRewardToken
REF_3274(bool) -> REF_3273[token_1]
TMP_13297 = UnaryType.BANG REF_3274 
CONDITION TMP_13297
 revert TokenDoesNotExist(address)(token)
TMP_13298(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 rewardTokens = $.rewardTokens
REF_3275(address[]) -> $_1 (-> ['TMP_13296']).rewardTokens
rewardTokens_1(address[]) = ['REF_3275(address[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < rewardTokens.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3276 -> LENGTH rewardTokens_1
TMP_13299(bool) = i_2 < REF_3276
CONDITION TMP_13299
 rewardTokens[i] == token
REF_3277(address) -> rewardTokens_1[i_2]
TMP_13300(bool) = REF_3277 == token_1
CONDITION TMP_13300
 i
RETURN i_2
 i ++
TMP_13301(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 revert InternalInconsistency(string)(Reward token map/array mismatch)
TMP_13302(None) = SOLIDITY_CALL revert InternalInconsistency(string)(Reward token map/array mismatch)
```
#### RewardsFacet._earnedView(address,address,uint16) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
token_1(address) := phi(['token_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13303(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13303'])(PlumeStakingStorage.Layout) := TMP_13303(PlumeStakingStorage.Layout)
 userStakedAmount = $.userValidatorStakes[user][validatorId].staked
REF_3279(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13303']).userValidatorStakes
REF_3280(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3279[user_1]
REF_3281(PlumeStakingStorage.UserValidatorStake) -> REF_3280[validatorId_1]
REF_3282(uint256) -> REF_3281.staked
userStakedAmount_1(uint256) := REF_3282(uint256)
 userStakedAmount == 0
TMP_13304(bool) = userStakedAmount_1 == 0
CONDITION TMP_13304
 $.userRewards[user][validatorId][token]
REF_3283(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13303']).userRewards
REF_3284(mapping(uint16 => mapping(address => uint256))) -> REF_3283[user_1]
REF_3285(mapping(address => uint256)) -> REF_3284[validatorId_1]
REF_3286(uint256) -> REF_3285[token_1]
RETURN REF_3286
 (userRewardDelta,None,None) = PlumeRewardLogic.calculateRewardsWithCheckpointsView($,user,validatorId,token,userStakedAmount)
TUPLE_89(uint256,uint256,uint256) = LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.calculateRewardsWithCheckpointsView(PlumeStakingStorage.Layout,address,uint16,address,uint256), arguments:["$_1 (-> ['TMP_13303'])", 'user_1', 'validatorId_1', 'token_1', 'userStakedAmount_1'] 
userRewardDelta_1(uint256)= UNPACK TUPLE_89 index: 0 
 rewards = $.userRewards[user][validatorId][token] + userRewardDelta
REF_3288(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13303']).userRewards
REF_3289(mapping(uint16 => mapping(address => uint256))) -> REF_3288[user_1]
REF_3290(mapping(address => uint256)) -> REF_3289[validatorId_1]
REF_3291(uint256) -> REF_3290[token_1]
TMP_13305(uint256) = REF_3291 (c)+ userRewardDelta_1
rewards_1(uint256) := TMP_13305(uint256)
 rewards
RETURN rewards_1
 rewards
```
#### RewardsFacet._calculateTotalEarnedView(address,address) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'user_1'])
token_1(address) := phi(['token_1', 'token_1'])
 $ = PlumeStakingStorage.layout()
TMP_13306(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13306'])(PlumeStakingStorage.Layout) := TMP_13306(PlumeStakingStorage.Layout)
 validatorIds = $.userValidators[user]
REF_3293(mapping(address => uint16[])) -> $_1 (-> ['TMP_13306']).userValidators
REF_3294(uint16[]) -> REF_3293[user_1]
validatorIds_1(uint16[]) = ['REF_3294(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
totalEarned_1(uint256) := phi(['totalEarned_0', 'totalEarned_2'])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3295 -> LENGTH validatorIds_1
TMP_13307(bool) = i_2 < REF_3295
CONDITION TMP_13307
 validatorId = validatorIds[i]
REF_3296(uint16) -> validatorIds_1[i_2]
validatorId_1(uint16) := REF_3296(uint16)
 totalEarned += _earnedView(user,token,validatorId)
TMP_13308(uint256) = INTERNAL_CALL, RewardsFacet._earnedView(address,address,uint16)(user_1,token_1,validatorId_1)
totalEarned_2(uint256) = totalEarned_1 (c)+ TMP_13308
 i ++
TMP_13309(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 totalEarned
RETURN totalEarned_1
 totalEarned
```
#### RewardsFacet.earned(address,address) [EXTERNAL]
```slithir
 _calculateTotalEarnedView(user,token)
TMP_13310(uint256) = INTERNAL_CALL, RewardsFacet._calculateTotalEarnedView(address,address)(user_1,token_1)
RETURN TMP_13310
```
#### RewardsFacet.getClaimableReward(address,address) [EXTERNAL]
```slithir
 _calculateTotalEarnedView(user,token)
TMP_13311(uint256) = INTERNAL_CALL, RewardsFacet._calculateTotalEarnedView(address,address)(user_1,token_1)
RETURN TMP_13311
```
#### RewardsFacet.getRewardTokens() [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13312(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13312'])(PlumeStakingStorage.Layout) := TMP_13312(PlumeStakingStorage.Layout)
 $.rewardTokens
REF_3298(address[]) -> $_1 (-> ['TMP_13312']).rewardTokens
RETURN REF_3298
```
#### RewardsFacet.isRewardToken(address) [EXTERNAL]
```slithir
 PlumeStakingStorage.layout().isRewardToken[token]
TMP_13313(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
REF_3300(mapping(address => bool)) -> TMP_13313.isRewardToken
REF_3301(bool) -> REF_3300[token_1]
RETURN REF_3301
```
#### RewardsFacet.getMaxRewardRate(address) [EXTERNAL]
```slithir
MAX_REWARD_RATE_4(uint256) := phi(['MAX_REWARD_RATE_0', 'MAX_REWARD_RATE_3'])
 $ = PlumeStakingStorage.layout()
TMP_13314(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13314'])(PlumeStakingStorage.Layout) := TMP_13314(PlumeStakingStorage.Layout)
 ! $.isRewardToken[token]
REF_3303(mapping(address => bool)) -> $_1 (-> ['TMP_13314']).isRewardToken
REF_3304(bool) -> REF_3303[token_1]
TMP_13315 = UnaryType.BANG REF_3304 
CONDITION TMP_13315
 revert TokenDoesNotExist(address)(token)
TMP_13316(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 $.maxRewardRates[token] > 0
REF_3305(mapping(address => uint256)) -> $_1 (-> ['TMP_13314']).maxRewardRates
REF_3306(uint256) -> REF_3305[token_1]
TMP_13317(bool) = REF_3306 > 0
CONDITION TMP_13317
 $.maxRewardRates[token]
REF_3307(mapping(address => uint256)) -> $_1 (-> ['TMP_13314']).maxRewardRates
REF_3308(uint256) -> REF_3307[token_1]
RETURN REF_3308
 MAX_REWARD_RATE
RETURN MAX_REWARD_RATE_4
```
#### RewardsFacet.getRewardRate(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13318(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13318'])(PlumeStakingStorage.Layout) := TMP_13318(PlumeStakingStorage.Layout)
 $.rewardRates[token]
REF_3310(mapping(address => uint256)) -> $_1 (-> ['TMP_13318']).rewardRates
REF_3311(uint256) -> REF_3310[token_1]
RETURN REF_3311
```
#### RewardsFacet.tokenRewardInfo(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13319(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13319'])(PlumeStakingStorage.Layout) := TMP_13319(PlumeStakingStorage.Layout)
 rewardRate = $.rewardRates[token]
REF_3313(mapping(address => uint256)) -> $_1 (-> ['TMP_13319']).rewardRates
REF_3314(uint256) -> REF_3313[token_1]
rewardRate_1(uint256) := REF_3314(uint256)
 validatorIds = $.validatorIds
REF_3315(uint16[]) -> $_1 (-> ['TMP_13319']).validatorIds
validatorIds_1(uint16[]) = ['REF_3315(uint16[])']
 lastUpdateTime = 0
lastUpdateTime_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3316 -> LENGTH validatorIds_1
TMP_13320(bool) = i_2 < REF_3316
CONDITION TMP_13320
 validatorUpdateTime = $.validatorLastUpdateTimes[validatorIds[i]][token]
REF_3317(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> ['TMP_13319']).validatorLastUpdateTimes
REF_3318(uint16) -> validatorIds_1[i_2]
REF_3319(mapping(address => uint256)) -> REF_3317[REF_3318]
REF_3320(uint256) -> REF_3319[token_1]
validatorUpdateTime_1(uint256) := REF_3320(uint256)
 validatorUpdateTime > lastUpdateTime
TMP_13321(bool) = validatorUpdateTime_1 > lastUpdateTime_1
CONDITION TMP_13321
 lastUpdateTime = validatorUpdateTime
lastUpdateTime_2(uint256) := validatorUpdateTime_1(uint256)
lastUpdateTime_3(uint256) := phi(['lastUpdateTime_2', 'lastUpdateTime_1'])
 i ++
TMP_13322(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 (rewardRate,lastUpdateTime)
RETURN rewardRate_1,lastUpdateTime_1
```
#### RewardsFacet.getRewardRateCheckpointCount(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13323(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13323'])(PlumeStakingStorage.Layout) := TMP_13323(PlumeStakingStorage.Layout)
 $.rewardRateCheckpoints[token].length
REF_3322(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> ['TMP_13323']).rewardRateCheckpoints
REF_3323(PlumeStakingStorage.RateCheckpoint[]) -> REF_3322[token_1]
REF_3324 -> LENGTH REF_3323
RETURN REF_3324
```
#### RewardsFacet.getValidatorRewardRateCheckpointCount(uint16,address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13324(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13324'])(PlumeStakingStorage.Layout) := TMP_13324(PlumeStakingStorage.Layout)
 $.validatorRewardRateCheckpoints[validatorId][token].length
REF_3326(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> ['TMP_13324']).validatorRewardRateCheckpoints
REF_3327(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_3326[validatorId_1]
REF_3328(PlumeStakingStorage.RateCheckpoint[]) -> REF_3327[token_1]
REF_3329 -> LENGTH REF_3328
RETURN REF_3329
```
#### RewardsFacet.getUserLastCheckpointIndex(address,uint16,address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13325(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13325'])(PlumeStakingStorage.Layout) := TMP_13325(PlumeStakingStorage.Layout)
 lastUpdateTimestamp = $.userValidatorRewardPerTokenPaidTimestamp[user][validatorId][token]
REF_3331(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13325']).userValidatorRewardPerTokenPaidTimestamp
REF_3332(mapping(uint16 => mapping(address => uint256))) -> REF_3331[user_1]
REF_3333(mapping(address => uint256)) -> REF_3332[validatorId_1]
REF_3334(uint256) -> REF_3333[token_1]
lastUpdateTimestamp_1(uint256) := REF_3334(uint256)
 lastUpdateTimestamp == 0
TMP_13326(bool) = lastUpdateTimestamp_1 == 0
CONDITION TMP_13326
 0
RETURN 0
 checkpoints = $.validatorRewardRateCheckpoints[validatorId][token]
REF_3335(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> ['TMP_13325']).validatorRewardRateCheckpoints
REF_3336(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_3335[validatorId_1]
REF_3337(PlumeStakingStorage.RateCheckpoint[]) -> REF_3336[token_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_3337(PlumeStakingStorage.RateCheckpoint[])']
 len = checkpoints.length
REF_3338 -> LENGTH checkpoints_1 (-> [])
len_1(uint256) := REF_3338(uint256)
 len == 0
TMP_13327(bool) = len_1 == 0
CONDITION TMP_13327
 0
RETURN 0
 low = 0
low_1(uint256) := 0(uint256)
 high = len - 1
TMP_13328(uint256) = len_1 (c)- 1
high_1(uint256) := TMP_13328(uint256)
 resultIndex = 0
resultIndex_1(uint256) := 0(uint256)
 found = false
found_1(bool) := False(bool)
 low <= high
TMP_13329(bool) = low_1 <= high_1
CONDITION TMP_13329
 mid = low + (high - low) / 2
TMP_13330(uint256) = high_1 (c)- low_1
TMP_13331(uint256) = TMP_13330 (c)/ 2
TMP_13332(uint256) = low_1 (c)+ TMP_13331
mid_1(uint256) := TMP_13332(uint256)
 checkpoints[mid].timestamp <= lastUpdateTimestamp
REF_3339(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[mid_1]
REF_3340(uint256) -> REF_3339.timestamp
TMP_13333(bool) = REF_3340 <= lastUpdateTimestamp_1
CONDITION TMP_13333
 resultIndex = mid
resultIndex_2(uint256) := mid_1(uint256)
 found = true
found_2(bool) := True(bool)
 low = mid + 1
TMP_13334(uint256) = mid_1 (c)+ 1
low_2(uint256) := TMP_13334(uint256)
 mid == 0
TMP_13335(bool) = mid_1 == 0
CONDITION TMP_13335
 high = mid - 1
TMP_13336(uint256) = mid_1 (c)- 1
high_2(uint256) := TMP_13336(uint256)
low_3(uint256) := phi(['low_1', 'low_2'])
high_3(uint256) := phi(['high_2', 'high_1'])
resultIndex_3(uint256) := phi(['resultIndex_1', 'resultIndex_2'])
found_3(bool) := phi(['found_2', 'found_1'])
 found
CONDITION found_1
 resultIndex
RETURN resultIndex_1
 0
RETURN 0
```
#### RewardsFacet.getRewardRateCheckpoint(address,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13337(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13337'])(PlumeStakingStorage.Layout) := TMP_13337(PlumeStakingStorage.Layout)
 index >= $.rewardRateCheckpoints[token].length
REF_3342(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> ['TMP_13337']).rewardRateCheckpoints
REF_3343(PlumeStakingStorage.RateCheckpoint[]) -> REF_3342[token_1]
REF_3344 -> LENGTH REF_3343
TMP_13338(bool) = index_1 >= REF_3344
CONDITION TMP_13338
 revert InvalidRewardRateCheckpoint(address,uint256)(token,index)
TMP_13339(None) = SOLIDITY_CALL revert InvalidRewardRateCheckpoint(address,uint256)(token_1,index_1)
 checkpoint = $.rewardRateCheckpoints[token][index]
REF_3345(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> ['TMP_13337']).rewardRateCheckpoints
REF_3346(PlumeStakingStorage.RateCheckpoint[]) -> REF_3345[token_1]
REF_3347(PlumeStakingStorage.RateCheckpoint) -> REF_3346[index_1]
checkpoint_1(PlumeStakingStorage.RateCheckpoint) := REF_3347(PlumeStakingStorage.RateCheckpoint)
 (checkpoint.timestamp,checkpoint.rate,checkpoint.cumulativeIndex)
REF_3348(uint256) -> checkpoint_1.timestamp
REF_3349(uint256) -> checkpoint_1.rate
REF_3350(uint256) -> checkpoint_1.cumulativeIndex
RETURN REF_3348,REF_3349,REF_3350
 (timestamp,rate,cumulativeIndex)
```
#### RewardsFacet.getValidatorRewardRateCheckpoint(uint16,address,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13340(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13340'])(PlumeStakingStorage.Layout) := TMP_13340(PlumeStakingStorage.Layout)
 index >= $.validatorRewardRateCheckpoints[validatorId][token].length
REF_3352(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> ['TMP_13340']).validatorRewardRateCheckpoints
REF_3353(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_3352[validatorId_1]
REF_3354(PlumeStakingStorage.RateCheckpoint[]) -> REF_3353[token_1]
REF_3355 -> LENGTH REF_3354
TMP_13341(bool) = index_1 >= REF_3355
CONDITION TMP_13341
 revert InvalidRewardRateCheckpoint(address,uint256)(token,index)
TMP_13342(None) = SOLIDITY_CALL revert InvalidRewardRateCheckpoint(address,uint256)(token_1,index_1)
 checkpoint = $.validatorRewardRateCheckpoints[validatorId][token][index]
REF_3356(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> ['TMP_13340']).validatorRewardRateCheckpoints
REF_3357(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_3356[validatorId_1]
REF_3358(PlumeStakingStorage.RateCheckpoint[]) -> REF_3357[token_1]
REF_3359(PlumeStakingStorage.RateCheckpoint) -> REF_3358[index_1]
checkpoint_1(PlumeStakingStorage.RateCheckpoint) := REF_3359(PlumeStakingStorage.RateCheckpoint)
 (checkpoint.timestamp,checkpoint.rate,checkpoint.cumulativeIndex)
REF_3360(uint256) -> checkpoint_1.timestamp
REF_3361(uint256) -> checkpoint_1.rate
REF_3362(uint256) -> checkpoint_1.cumulativeIndex
RETURN REF_3360,REF_3361,REF_3362
 (timestamp,rate,cumulativeIndex)
```
#### RewardsFacet.getTreasury() [EXTERNAL]
```slithir
 getTreasuryAddress()
TMP_13343(address) = INTERNAL_CALL, RewardsFacet.getTreasuryAddress()()
RETURN TMP_13343
```
#### RewardsFacet.getPendingRewardForValidator(address,uint16,address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13344(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13344'])(PlumeStakingStorage.Layout) := TMP_13344(PlumeStakingStorage.Layout)
 userStakedAmount = $.userValidatorStakes[user][validatorId].staked
REF_3364(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13344']).userValidatorStakes
REF_3365(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3364[user_1]
REF_3366(PlumeStakingStorage.UserValidatorStake) -> REF_3365[validatorId_1]
REF_3367(uint256) -> REF_3366.staked
userStakedAmount_1(uint256) := REF_3367(uint256)
 (userRewardDelta,None,None) = PlumeRewardLogic.calculateRewardsWithCheckpoints($,user,validatorId,token,userStakedAmount)
TUPLE_90(uint256,uint256,uint256) = LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.calculateRewardsWithCheckpoints(PlumeStakingStorage.Layout,address,uint16,address,uint256), arguments:["$_1 (-> ['TMP_13344'])", 'user_1', 'validatorId_1', 'token_1', 'userStakedAmount_1'] 
userRewardDelta_1(uint256)= UNPACK TUPLE_90 index: 0 
 userRewardDelta
RETURN userRewardDelta_1
 pendingReward
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
#### PlumeRewardLogic.createRewardRateCheckpoint(PlumeStakingStorage.Layout,address,uint16,uint256) [INTERNAL]
```slithir
 updateRewardPerTokenForValidator($,token,validatorId)
INTERNAL_CALL, PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16)($_1 (-> []),token_1,validatorId_1)
 currentCumulativeIndex = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_4386(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorRewardPerTokenCumulative
REF_4387(mapping(address => uint256)) -> REF_4386[validatorId_1]
REF_4388(uint256) -> REF_4387[token_1]
currentCumulativeIndex_1(uint256) := REF_4388(uint256)
 checkpoints = $.validatorRewardRateCheckpoints[validatorId][token]
REF_4389(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> []).validatorRewardRateCheckpoints
REF_4390(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4389[validatorId_1]
REF_4391(PlumeStakingStorage.RateCheckpoint[]) -> REF_4390[token_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4391(PlumeStakingStorage.RateCheckpoint[])']
 len = checkpoints.length
REF_4392 -> LENGTH checkpoints_1 (-> [])
len_1(uint256) := REF_4392(uint256)
 checkpoint = PlumeStakingStorage.RateCheckpoint({timestamp:block.timestamp,rate:rate,cumulativeIndex:currentCumulativeIndex})
TMP_14190(PlumeStakingStorage.RateCheckpoint) = new RateCheckpoint(block.timestamp,rate_1,currentCumulativeIndex_1)
checkpoint_1(PlumeStakingStorage.RateCheckpoint) := TMP_14190(PlumeStakingStorage.RateCheckpoint)
 len > 0 && checkpoints[len - 1].timestamp == block.timestamp
TMP_14191(bool) = len_1 > 0
TMP_14192(uint256) = len_1 (c)- 1
REF_4394(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[TMP_14192]
REF_4395(uint256) -> REF_4394.timestamp
TMP_14193(bool) = REF_4395 == block.timestamp
TMP_14194(bool) = TMP_14191 && TMP_14193
CONDITION TMP_14194
 checkpoints[len - 1] = checkpoint
TMP_14195(uint256) = len_1 (c)- 1
REF_4396(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[TMP_14195]
checkpoints_2 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_1 (-> [])'])
REF_4396(PlumeStakingStorage.RateCheckpoint) (->checkpoints_2 (-> [])) := checkpoint_1(PlumeStakingStorage.RateCheckpoint)
 checkpointIndex = len - 1
TMP_14196(uint256) = len_1 (c)- 1
checkpointIndex_1(uint256) := TMP_14196(uint256)
 checkpoints.push(checkpoint)
REF_4398 -> LENGTH checkpoints_1 (-> [])
TMP_14198(uint256) := REF_4398(uint256)
TMP_14199(uint256) = TMP_14198 (c)+ 1
checkpoints_3 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_1 (-> [])'])
REF_4398(uint256) (->checkpoints_3 (-> [])) := TMP_14199(uint256)
REF_4399(PlumeStakingStorage.RateCheckpoint) -> checkpoints_3 (-> [])[TMP_14198]
checkpoints_4 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_3 (-> [])'])
REF_4399(PlumeStakingStorage.RateCheckpoint) (->checkpoints_4 (-> [])) := checkpoint_1(PlumeStakingStorage.RateCheckpoint)
 checkpointIndex = len
checkpointIndex_2(uint256) := len_1(uint256)
checkpointIndex_3(uint256) := phi(['checkpointIndex_2', 'checkpointIndex_1'])
 RewardRateCheckpointCreated(token,validatorId,rate,block.timestamp,checkpointIndex,currentCumulativeIndex)
Emit RewardRateCheckpointCreated(token_1,validatorId_1,rate_1,block.timestamp,checkpointIndex_3,currentCumulativeIndex_1)
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
#### IPlumeStakingRewardTreasury.distributeReward(address,uint256,address) [EXTERNAL]
```slithir

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
#### PlumeRewardLogic._ceilDiv(uint256,uint256) [INTERNAL]
```slithir
a_1(uint256) := phi(['TMP_14094'])
b_1(uint256) := phi(['REF_4317'])
 b == 0
TMP_14109(bool) = b_1 == 0
CONDITION TMP_14109
 0
RETURN 0
 (a + b - 1) / b
TMP_14110(uint256) = a_1 (c)+ b_1
TMP_14111(uint256) = TMP_14110 (c)- 1
TMP_14112(uint256) = TMP_14111 (c)/ b_1
RETURN TMP_14112
 result
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
