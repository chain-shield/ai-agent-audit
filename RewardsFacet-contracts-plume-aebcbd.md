




#### RewardsFacet.getTreasuryAddress() [INTERNAL]
```slithir
TREASURY_STORAGE_POSITION_1(bytes32) := phi(['TREASURY_STORAGE_POSITION_0'])
 position = TREASURY_STORAGE_POSITION
position_1(bytes32) := TREASURY_STORAGE_POSITION_1(bytes32)
 treasuryAddress = sload(uint256)(position)
TMP_13021(uint256) = SOLIDITY_CALL sload(uint256)(position_1)
treasuryAddress_1(address) := TMP_13021(uint256)
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
TMP_13022(None) = SOLIDITY_CALL sstore(uint256,uint256)(position_1,_treasury_1)
```
#### RewardsFacet._earned(address,address,uint16) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
token_1(address) := phi(['token_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13023(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13023'])(PlumeStakingStorage.Layout) := TMP_13023(PlumeStakingStorage.Layout)
 userStakedAmount = $.userValidatorStakes[user][validatorId].staked
REF_2966(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13023']).userValidatorStakes
REF_2967(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_2966[user_1]
REF_2968(PlumeStakingStorage.UserValidatorStake) -> REF_2967[validatorId_1]
REF_2969(uint256) -> REF_2968.staked
userStakedAmount_1(uint256) := REF_2969(uint256)
 userStakedAmount == 0
TMP_13024(bool) = userStakedAmount_1 == 0
CONDITION TMP_13024
 $.userRewards[user][validatorId][token]
REF_2970(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13023']).userRewards
REF_2971(mapping(uint16 => mapping(address => uint256))) -> REF_2970[user_1]
REF_2972(mapping(address => uint256)) -> REF_2971[validatorId_1]
REF_2973(uint256) -> REF_2972[token_1]
RETURN REF_2973
 (userRewardDelta,None,None) = PlumeRewardLogic.calculateRewardsWithCheckpoints($,user,validatorId,token,userStakedAmount)
TUPLE_87(uint256,uint256,uint256) = LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.calculateRewardsWithCheckpoints(PlumeStakingStorage.Layout,address,uint16,address,uint256), arguments:["$_1 (-> ['TMP_13023'])", 'user_1', 'validatorId_1', 'token_1', 'userStakedAmount_1'] 
userRewardDelta_1(uint256)= UNPACK TUPLE_87 index: 0 
 rewards = $.userRewards[user][validatorId][token] + userRewardDelta
REF_2975(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13023']).userRewards
REF_2976(mapping(uint16 => mapping(address => uint256))) -> REF_2975[user_1]
REF_2977(mapping(address => uint256)) -> REF_2976[validatorId_1]
REF_2978(uint256) -> REF_2977[token_1]
TMP_13025(uint256) = REF_2978 (c)+ userRewardDelta_1
rewards_1(uint256) := TMP_13025(uint256)
 rewards
RETURN rewards_1
 rewards
```
#### RewardsFacet._calculateTotalEarned(address,address) [INTERNAL]
```slithir
user_1(address) := phi(['user_1'])
token_1(address) := phi(['token_1'])
 $ = PlumeStakingStorage.layout()
TMP_13026(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13026'])(PlumeStakingStorage.Layout) := TMP_13026(PlumeStakingStorage.Layout)
 validatorIds = $.userValidators[user]
REF_2980(mapping(address => uint16[])) -> $_1 (-> ['TMP_13026']).userValidators
REF_2981(uint16[]) -> REF_2980[user_1]
validatorIds_1(uint16[]) = ['REF_2981(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_2982 -> LENGTH validatorIds_1
TMP_13027(bool) = i_2 < REF_2982
CONDITION TMP_13027
 validatorId = validatorIds[i]
REF_2983(uint16) -> validatorIds_1[i_2]
validatorId_1(uint16) := REF_2983(uint16)
 ! $.validators[validatorId].active
REF_2984(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13026']).validators
REF_2985(PlumeStakingStorage.ValidatorInfo) -> REF_2984[validatorId_1]
REF_2986(bool) -> REF_2985.active
TMP_13028 = UnaryType.BANG REF_2986 
CONDITION TMP_13028
 totalEarned += $.userRewards[user][validatorId][token]
REF_2987(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13026']).userRewards
REF_2988(mapping(uint16 => mapping(address => uint256))) -> REF_2987[user_1]
REF_2989(mapping(address => uint256)) -> REF_2988[validatorId_1]
REF_2990(uint256) -> REF_2989[token_1]
totalEarned_1(uint256) = totalEarned_0 (c)+ REF_2990
 totalEarned += _earned(user,token,validatorId)
TMP_13029(uint256) = INTERNAL_CALL, RewardsFacet._earned(address,address,uint16)(user_1,token_1,validatorId_1)
totalEarned_3(uint256) = totalEarned_0 (c)+ TMP_13029
 i ++
totalEarned_2(uint256) := phi(['totalEarned_1', 'totalEarned_3'])
TMP_13030(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 totalEarned
RETURN totalEarned_0
 totalEarned
```
#### RewardsFacet.setTreasury(address) [EXTERNAL]
```slithir
 _treasury == address(0)
TMP_13031 = CONVERT 0 to address
TMP_13032(bool) = _treasury_1 == TMP_13031
CONDITION TMP_13032
 revert ZeroAddress(string)(treasury)
TMP_13033(None) = SOLIDITY_CALL revert ZeroAddress(string)(treasury)
 setTreasuryAddress(_treasury)
INTERNAL_CALL, RewardsFacet.setTreasuryAddress(address)(_treasury_1)
 TreasurySet(_treasury)
Emit TreasurySet(_treasury_1)
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_2991(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_2991)
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_2992(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_2992)
```
#### RewardsFacet.addRewardToken(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13038(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13038'])(PlumeStakingStorage.Layout) := TMP_13038(PlumeStakingStorage.Layout)
 token == address(0)
TMP_13039 = CONVERT 0 to address
TMP_13040(bool) = token_1 == TMP_13039
CONDITION TMP_13040
 revert ZeroAddress(string)(token)
TMP_13041(None) = SOLIDITY_CALL revert ZeroAddress(string)(token)
 $.isRewardToken[token]
REF_2994(mapping(address => bool)) -> $_1 (-> ['TMP_13038']).isRewardToken
REF_2995(bool) -> REF_2994[token_1]
CONDITION REF_2995
 revert TokenAlreadyExists()()
TMP_13042(None) = SOLIDITY_CALL revert TokenAlreadyExists()()
 $.rewardTokens.push(token)
REF_2996(address[]) -> $_1 (-> ['TMP_13038']).rewardTokens
REF_2998 -> LENGTH REF_2996
TMP_13044(uint256) := REF_2998(uint256)
TMP_13045(uint256) = TMP_13044 (c)+ 1
$_2 (-> ['TMP_13038'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13038'])"])
REF_2998(uint256) (->$_3 (-> ['TMP_13038'])) := TMP_13045(uint256)
REF_2999(address) -> REF_2996[TMP_13044]
$_3 (-> ['TMP_13038'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13038'])"])
REF_2999(address) (->$_3 (-> ['TMP_13038'])) := token_1(address)
TMP_13038(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13038'])"])
 $.isRewardToken[token] = true
REF_3000(mapping(address => bool)) -> $_3 (-> ['TMP_13038']).isRewardToken
REF_3001(bool) -> REF_3000[token_1]
$_4 (-> ['TMP_13038'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13038'])"])
REF_3001(bool) (->$_4 (-> ['TMP_13038'])) := True(bool)
TMP_13038(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13038'])"])
 i = 0
i_1(uint256) := 0(uint256)
 i < $.validatorIds.length
$_5 (-> ['TMP_13038'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13038'])", "$_6 (-> ['TMP_13038'])"])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3002(uint16[]) -> $_5 (-> ['TMP_13038']).validatorIds
REF_3003 -> LENGTH REF_3002
TMP_13046(bool) = i_2 < REF_3003
CONDITION TMP_13046
 $.validatorLastUpdateTimes[$.validatorIds[i]][token] = block.timestamp
REF_3004(mapping(uint16 => mapping(address => uint256))) -> $_5 (-> ['TMP_13038']).validatorLastUpdateTimes
REF_3005(uint16[]) -> $_5 (-> ['TMP_13038']).validatorIds
REF_3006(uint16) -> REF_3005[i_2]
REF_3007(mapping(address => uint256)) -> REF_3004[REF_3006]
REF_3008(uint256) -> REF_3007[token_1]
$_6 (-> ['TMP_13038'])(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13038'])"])
REF_3008(uint256) (->$_6 (-> ['TMP_13038'])) := block.timestamp(uint256)
TMP_13038(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13038'])"])
 i ++
TMP_13047(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 RewardTokenAdded(token)
Emit RewardTokenAdded(token_1)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3009(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3009)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3010(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3010)
```
#### RewardsFacet.removeRewardToken(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13051(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13051'])(PlumeStakingStorage.Layout) := TMP_13051(PlumeStakingStorage.Layout)
 ! $.isRewardToken[token]
REF_3012(mapping(address => bool)) -> $_1 (-> ['TMP_13051']).isRewardToken
REF_3013(bool) -> REF_3012[token_1]
TMP_13052 = UnaryType.BANG REF_3013 
CONDITION TMP_13052
 revert TokenDoesNotExist(address)(token)
TMP_13053(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 tokenIndex = _getTokenIndex(token)
TMP_13054(uint256) = INTERNAL_CALL, RewardsFacet._getTokenIndex(address)(token_1)
tokenIndex_1(uint256) := TMP_13054(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < $.validatorIds.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3014(uint16[]) -> $_1 (-> ['TMP_13051']).validatorIds
REF_3015 -> LENGTH REF_3014
TMP_13055(bool) = i_2 < REF_3015
CONDITION TMP_13055
 PlumeRewardLogic.updateRewardPerTokenForValidator($,token,$.validatorIds[i])
REF_3017(uint16[]) -> $_1 (-> ['TMP_13051']).validatorIds
REF_3018(uint16) -> REF_3017[i_2]
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13051'])", 'token_1', 'REF_3018'] 
 i ++
TMP_13057(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 $.rewardRates[token] = 0
REF_3019(mapping(address => uint256)) -> $_1 (-> ['TMP_13051']).rewardRates
REF_3020(uint256) -> REF_3019[token_1]
$_2 (-> ['TMP_13051'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13051'])"])
REF_3020(uint256) (->$_2 (-> ['TMP_13051'])) := 0(uint256)
TMP_13051(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13051'])"])
 $.rewardTokens[tokenIndex] = $.rewardTokens[$.rewardTokens.length - 1]
REF_3021(address[]) -> $_2 (-> ['TMP_13051']).rewardTokens
REF_3022(address) -> REF_3021[tokenIndex_1]
REF_3023(address[]) -> $_2 (-> ['TMP_13051']).rewardTokens
REF_3024(address[]) -> $_2 (-> ['TMP_13051']).rewardTokens
REF_3025 -> LENGTH REF_3024
TMP_13058(uint256) = REF_3025 (c)- 1
REF_3026(address) -> REF_3023[TMP_13058]
$_3 (-> ['TMP_13051'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13051'])"])
REF_3022(address) (->$_3 (-> ['TMP_13051'])) := REF_3026(address)
TMP_13051(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13051'])"])
 $.rewardTokens.pop()
REF_3027(address[]) -> $_3 (-> ['TMP_13051']).rewardTokens
REF_3029 -> LENGTH REF_3027
TMP_13060(uint256) = REF_3029 (c)- 1
REF_3030(address) -> REF_3027[TMP_13060]
REF_3027 = delete REF_3030 
REF_3031 -> LENGTH REF_3027
$_4 (-> ['TMP_13051'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13051'])"])
REF_3031(uint256) (->$_4 (-> ['TMP_13051'])) := TMP_13060(uint256)
TMP_13051(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13051'])"])
 $.isRewardToken[token] = false
REF_3032(mapping(address => bool)) -> $_4 (-> ['TMP_13051']).isRewardToken
REF_3033(bool) -> REF_3032[token_1]
$_5 (-> ['TMP_13051'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13051'])"])
REF_3033(bool) (->$_5 (-> ['TMP_13051'])) := False(bool)
TMP_13051(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13051'])"])
 delete $.maxRewardRates[token]
REF_3034(mapping(address => uint256)) -> $_5 (-> ['TMP_13051']).maxRewardRates
REF_3035(uint256) -> REF_3034[token_1]
REF_3034 = delete REF_3035 
 RewardTokenRemoved(token)
Emit RewardTokenRemoved(token_1)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3036(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3036)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3037(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3037)
```
#### RewardsFacet.setRewardRates(address[],uint256[]) [EXTERNAL]
```slithir
MAX_REWARD_RATE_1(uint256) := phi(['MAX_REWARD_RATE_0', 'MAX_REWARD_RATE_3'])
 $ = PlumeStakingStorage.layout()
TMP_13064(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13064'])(PlumeStakingStorage.Layout) := TMP_13064(PlumeStakingStorage.Layout)
 tokens.length == 0
REF_3039 -> LENGTH tokens_1
TMP_13065(bool) = REF_3039 == 0
CONDITION TMP_13065
 revert EmptyArray()()
TMP_13066(None) = SOLIDITY_CALL revert EmptyArray()()
 tokens.length != rewardRates_.length
REF_3040 -> LENGTH tokens_1
REF_3041 -> LENGTH rewardRates__1
TMP_13067(bool) = REF_3040 != REF_3041
CONDITION TMP_13067
 revert ArrayLengthMismatch()()
TMP_13068(None) = SOLIDITY_CALL revert ArrayLengthMismatch()()
 validatorIds = $.validatorIds
REF_3042(uint16[]) -> $_1 (-> ['TMP_13064']).validatorIds
validatorIds_1(uint16[]) = ['REF_3042(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < tokens.length
$_2 (-> ['TMP_13064'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13064'])", "$_3 (-> ['TMP_13064'])"])
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3043 -> LENGTH tokens_1
TMP_13069(bool) = i_2 < REF_3043
CONDITION TMP_13069
 token_loop = tokens[i]
REF_3044(address) -> tokens_1[i_2]
token_loop_1(address) := REF_3044(address)
 rate_loop = rewardRates_[i]
REF_3045(uint256) -> rewardRates__1[i_2]
rate_loop_1(uint256) := REF_3045(uint256)
 ! $.isRewardToken[token_loop]
REF_3046(mapping(address => bool)) -> $_2 (-> ['TMP_13064']).isRewardToken
REF_3047(bool) -> REF_3046[token_loop_1]
TMP_13070 = UnaryType.BANG REF_3047 
CONDITION TMP_13070
 revert TokenDoesNotExist(address)(token_loop)
TMP_13071(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_loop_1)
 rate_loop > maxRate
TMP_13072(bool) = rate_loop_1 > maxRate_3
CONDITION TMP_13072
 revert RewardRateExceedsMax()()
TMP_13073(None) = SOLIDITY_CALL revert RewardRateExceedsMax()()
 j = 0
j_1(uint256) := 0(uint256)
 j < validatorIds.length
j_2(uint256) := phi(['j_3', 'j_1'])
REF_3048 -> LENGTH validatorIds_1
TMP_13074(bool) = j_2 < REF_3048
CONDITION TMP_13074
 validatorId_for_crrc = validatorIds[j]
REF_3049(uint16) -> validatorIds_1[j_2]
validatorId_for_crrc_1(uint16) := REF_3049(uint16)
 PlumeRewardLogic.createRewardRateCheckpoint($,token_loop,validatorId_for_crrc,rate_loop)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.createRewardRateCheckpoint(PlumeStakingStorage.Layout,address,uint16,uint256), arguments:["$_2 (-> ['TMP_13064'])", 'token_loop_1', 'validatorId_for_crrc_1', 'rate_loop_1'] 
 j ++
TMP_13076(uint256) := j_2(uint256)
j_3(uint256) = j_2 (c)+ 1
 $.rewardRates[token_loop] = rate_loop
REF_3051(mapping(address => uint256)) -> $_2 (-> ['TMP_13064']).rewardRates
REF_3052(uint256) -> REF_3051[token_loop_1]
$_3 (-> ['TMP_13064'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13064'])"])
REF_3052(uint256) (->$_3 (-> ['TMP_13064'])) := rate_loop_1(uint256)
TMP_13064(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13064'])"])
 i ++
TMP_13077(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 RewardRatesSet(tokens,rewardRates_)
Emit RewardRatesSet(tokens_1,rewardRates__1)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3053(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3053)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3054(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3054)
 $.maxRewardRates[token_loop] > 0
REF_3055(mapping(address => uint256)) -> $_2 (-> ['TMP_13064']).maxRewardRates
REF_3056(uint256) -> REF_3055[token_loop_1]
TMP_13081(bool) = REF_3056 > 0
CONDITION TMP_13081
 maxRate = $.maxRewardRates[token_loop]
REF_3057(mapping(address => uint256)) -> $_2 (-> ['TMP_13064']).maxRewardRates
REF_3058(uint256) -> REF_3057[token_loop_1]
maxRate_1(uint256) := REF_3058(uint256)
 maxRate = MAX_REWARD_RATE
maxRate_2(uint256) := MAX_REWARD_RATE_3(uint256)
maxRate_3(uint256) := phi(['maxRate_1', 'maxRate_2'])
```
#### RewardsFacet.setMaxRewardRate(address,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13082(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13082'])(PlumeStakingStorage.Layout) := TMP_13082(PlumeStakingStorage.Layout)
 ! $.isRewardToken[token]
REF_3060(mapping(address => bool)) -> $_1 (-> ['TMP_13082']).isRewardToken
REF_3061(bool) -> REF_3060[token_1]
TMP_13083 = UnaryType.BANG REF_3061 
CONDITION TMP_13083
 revert TokenDoesNotExist(address)(token)
TMP_13084(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 $.rewardRates[token] > newMaxRate
REF_3062(mapping(address => uint256)) -> $_1 (-> ['TMP_13082']).rewardRates
REF_3063(uint256) -> REF_3062[token_1]
TMP_13085(bool) = REF_3063 > newMaxRate_1
CONDITION TMP_13085
 revert RewardRateExceedsMax()()
TMP_13086(None) = SOLIDITY_CALL revert RewardRateExceedsMax()()
 $.maxRewardRates[token] = newMaxRate
REF_3064(mapping(address => uint256)) -> $_1 (-> ['TMP_13082']).maxRewardRates
REF_3065(uint256) -> REF_3064[token_1]
$_2 (-> ['TMP_13082'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13082'])"])
REF_3065(uint256) (->$_2 (-> ['TMP_13082'])) := newMaxRate_1(uint256)
TMP_13082(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13082'])"])
 MaxRewardRateUpdated(token,newMaxRate)
Emit MaxRewardRateUpdated(token_1,newMaxRate_1)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3066(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3066)
 onlyRole(PlumeRoles.REWARD_MANAGER_ROLE)
REF_3067(bytes32) -> PlumeRoles.REWARD_MANAGER_ROLE
MODIFIER_CALL, RewardsFacet.onlyRole(bytes32)(REF_3067)
```
#### RewardsFacet.claim(address) [EXTERNAL]
```slithir
 _validateTokenForClaim(token,msg.sender)
TMP_13098(bool) = INTERNAL_CALL, RewardsFacet._validateTokenForClaim(address,address)(token_1,msg.sender)
 totalReward = _processAllValidatorRewards(msg.sender,token)
TMP_13099(uint256) = INTERNAL_CALL, RewardsFacet._processAllValidatorRewards(address,address)(msg.sender,token_1)
totalReward_1(uint256) := TMP_13099(uint256)
 totalReward > 0
TMP_13100(bool) = totalReward_1 > 0
CONDITION TMP_13100
 _finalizeRewardClaim(token,totalReward,msg.sender)
INTERNAL_CALL, RewardsFacet._finalizeRewardClaim(address,uint256,address)(token_1,totalReward_1,msg.sender)
 RewardClaimed(msg.sender,token,totalReward)
Emit RewardClaimed(msg.sender,token_1,totalReward_1)
 $ = PlumeStakingStorage.layout()
TMP_13103(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13103'])(PlumeStakingStorage.Layout) := TMP_13103(PlumeStakingStorage.Layout)
 validatorIds = $.userValidators[msg.sender]
REF_3071(mapping(address => uint16[])) -> $_1 (-> ['TMP_13103']).userValidators
REF_3072(uint16[]) -> REF_3071[msg.sender]
validatorIds_1(uint16[]) = ['REF_3072(uint16[])']
 _clearPendingRewardFlags(msg.sender,validatorIds)
INTERNAL_CALL, RewardsFacet._clearPendingRewardFlags(address,uint16[])(msg.sender,validatorIds_1)
 PlumeValidatorLogic.removeStakerFromAllValidators($,msg.sender)
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.removeStakerFromAllValidators(PlumeStakingStorage.Layout,address), arguments:["$_1 (-> ['TMP_13103'])", 'msg.sender'] 
 totalReward
RETURN totalReward_1
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### RewardsFacet.claimAll() [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13107(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13107'])(PlumeStakingStorage.Layout) := TMP_13107(PlumeStakingStorage.Layout)
 tokens = $.rewardTokens
REF_3075(address[]) -> $_1 (-> ['TMP_13107']).rewardTokens
tokens_1(address[]) = ['REF_3075(address[])']
 claims = new uint256[](tokens.length)
REF_3076 -> LENGTH tokens_1
TMP_13109(uint256[])  = new uint256[](REF_3076)
claims_1(uint256[]) = ['TMP_13109(uint256[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < tokens.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3077 -> LENGTH tokens_1
TMP_13110(bool) = i_2 < REF_3077
CONDITION TMP_13110
 token = tokens[i]
REF_3078(address) -> tokens_1[i_2]
token_1(address) := REF_3078(address)
 totalReward = _processAllValidatorRewards(msg.sender,token)
TMP_13111(uint256) = INTERNAL_CALL, RewardsFacet._processAllValidatorRewards(address,address)(msg.sender,token_1)
totalReward_1(uint256) := TMP_13111(uint256)
 totalReward > 0
TMP_13112(bool) = totalReward_1 > 0
CONDITION TMP_13112
 _finalizeRewardClaim(token,totalReward,msg.sender)
INTERNAL_CALL, RewardsFacet._finalizeRewardClaim(address,uint256,address)(token_1,totalReward_1,msg.sender)
 claims[i] = totalReward
REF_3079(uint256) -> claims_1[i_2]
claims_2(uint256[]) := phi(['claims_1'])
REF_3079(uint256) (->claims_2) := totalReward_1(uint256)
 RewardClaimed(msg.sender,token,totalReward)
Emit RewardClaimed(msg.sender,token_1,totalReward_1)
claims_3(uint256[]) := phi(['claims_1', 'claims_2'])
 i ++
TMP_13115(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 validatorIds = $.userValidators[msg.sender]
REF_3080(mapping(address => uint16[])) -> $_1 (-> ['TMP_13107']).userValidators
REF_3081(uint16[]) -> REF_3080[msg.sender]
validatorIds_1(uint16[]) = ['REF_3081(uint16[])']
 _clearPendingRewardFlags(msg.sender,validatorIds)
INTERNAL_CALL, RewardsFacet._clearPendingRewardFlags(address,uint16[])(msg.sender,validatorIds_1)
 PlumeValidatorLogic.removeStakerFromAllValidators($,msg.sender)
LIBRARY_CALL, dest:PlumeValidatorLogic, function:PlumeValidatorLogic.removeStakerFromAllValidators(PlumeStakingStorage.Layout,address), arguments:["$_1 (-> ['TMP_13107'])", 'msg.sender'] 
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
TMP_13119(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13119'])(PlumeStakingStorage.Layout) := TMP_13119(PlumeStakingStorage.Layout)
 isActive = $.isRewardToken[token]
REF_3084(mapping(address => bool)) -> $_1 (-> ['TMP_13119']).isRewardToken
REF_3085(bool) -> REF_3084[token_1]
isActive_1(bool) := REF_3085(bool)
 ! isActive
TMP_13120 = UnaryType.BANG isActive_1 
CONDITION TMP_13120
 validatorIds = $.userValidators[user]
REF_3086(mapping(address => uint16[])) -> $_1 (-> ['TMP_13119']).userValidators
REF_3087(uint16[]) -> REF_3086[user_1]
validatorIds_1(uint16[]) = ['REF_3087(uint16[])']
 hasExistingRewards = false
hasExistingRewards_1(bool) := False(bool)
hasExistingRewards_3(bool) := phi(['hasExistingRewards_2', 'hasExistingRewards_1'])
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3088 -> LENGTH validatorIds_1
TMP_13121(bool) = i_2 < REF_3088
CONDITION TMP_13121
 $.userRewards[user][validatorIds[i]][token] > 0
REF_3089(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13119']).userRewards
REF_3090(mapping(uint16 => mapping(address => uint256))) -> REF_3089[user_1]
REF_3091(uint16) -> validatorIds_1[i_2]
REF_3092(mapping(address => uint256)) -> REF_3090[REF_3091]
REF_3093(uint256) -> REF_3092[token_1]
TMP_13122(bool) = REF_3093 > 0
CONDITION TMP_13122
 hasExistingRewards = true
hasExistingRewards_2(bool) := True(bool)
 i ++
TMP_13123(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 ! hasExistingRewards
TMP_13124 = UnaryType.BANG hasExistingRewards_3 
CONDITION TMP_13124
 revert TokenDoesNotExist(address)(token)
TMP_13125(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 isActive
RETURN isActive_1
```
#### RewardsFacet._validateValidatorForClaim(uint16) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13126(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13126'])(PlumeStakingStorage.Layout) := TMP_13126(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_3095(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13126']).validatorExists
REF_3096(bool) -> REF_3095[validatorId_1]
TMP_13127 = UnaryType.BANG REF_3096 
CONDITION TMP_13127
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13128(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 ! $.validators[validatorId].active
REF_3097(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13126']).validators
REF_3098(PlumeStakingStorage.ValidatorInfo) -> REF_3097[validatorId_1]
REF_3099(bool) -> REF_3098.active
TMP_13129 = UnaryType.BANG REF_3099 
CONDITION TMP_13129
 revert ValidatorInactive(uint16)(validatorId)
TMP_13130(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(validatorId_1)
```
#### RewardsFacet._processValidatorRewards(address,uint16,address) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender', 'user_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
token_1(address) := phi(['token_1', 'token_1'])
 $ = PlumeStakingStorage.layout()
TMP_13131(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13131'])(PlumeStakingStorage.Layout) := TMP_13131(PlumeStakingStorage.Layout)
 userStakedAmount = $.userValidatorStakes[user][validatorId].staked
REF_3101(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13131']).userValidatorStakes
REF_3102(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3101[user_1]
REF_3103(PlumeStakingStorage.UserValidatorStake) -> REF_3102[validatorId_1]
REF_3104(uint256) -> REF_3103.staked
userStakedAmount_1(uint256) := REF_3104(uint256)
 PlumeRewardLogic.updateRewardPerTokenForValidator($,token,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13131'])", 'token_1', 'validatorId_1'] 
 (userRewardDelta,None,None) = PlumeRewardLogic.calculateRewardsWithCheckpoints($,user,validatorId,token,userStakedAmount)
TUPLE_88(uint256,uint256,uint256) = LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.calculateRewardsWithCheckpoints(PlumeStakingStorage.Layout,address,uint16,address,uint256), arguments:["$_1 (-> ['TMP_13131'])", 'user_1', 'validatorId_1', 'token_1', 'userStakedAmount_1'] 
userRewardDelta_1(uint256)= UNPACK TUPLE_88 index: 0 
 reward = $.userRewards[user][validatorId][token] + userRewardDelta
REF_3107(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13131']).userRewards
REF_3108(mapping(uint16 => mapping(address => uint256))) -> REF_3107[user_1]
REF_3109(mapping(address => uint256)) -> REF_3108[validatorId_1]
REF_3110(uint256) -> REF_3109[token_1]
TMP_13133(uint256) = REF_3110 (c)+ userRewardDelta_1
reward_1(uint256) := TMP_13133(uint256)
 reward > 0
TMP_13134(bool) = reward_1 > 0
CONDITION TMP_13134
 _updateUserRewardState(user,validatorId,token,reward)
INTERNAL_CALL, RewardsFacet._updateUserRewardState(address,uint16,address,uint256)(user_1,validatorId_1,token_1,reward_1)
 RewardClaimedFromValidator(user,token,validatorId,reward)
Emit RewardClaimedFromValidator(user_1,token_1,validatorId_1,reward_1)
 reward
RETURN reward_1
 reward
```
#### RewardsFacet._updateUserRewardState(address,uint16,address,uint256) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'user_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
token_1(address) := phi(['token_1', 'token_1'])
rewardAmount_1(uint256) := phi(['storedReward_1', 'reward_1'])
 $ = PlumeStakingStorage.layout()
TMP_13137(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13137'])(PlumeStakingStorage.Layout) := TMP_13137(PlumeStakingStorage.Layout)
 $.userRewards[user][validatorId][token] = 0
REF_3112(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13137']).userRewards
REF_3113(mapping(uint16 => mapping(address => uint256))) -> REF_3112[user_1]
REF_3114(mapping(address => uint256)) -> REF_3113[validatorId_1]
REF_3115(uint256) -> REF_3114[token_1]
$_2 (-> ['TMP_13137'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13137'])"])
REF_3115(uint256) (->$_2 (-> ['TMP_13137'])) := 0(uint256)
TMP_13137(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13137'])"])
 $.userValidatorRewardPerTokenPaidTimestamp[user][validatorId][token] = block.timestamp
REF_3116(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_2 (-> ['TMP_13137']).userValidatorRewardPerTokenPaidTimestamp
REF_3117(mapping(uint16 => mapping(address => uint256))) -> REF_3116[user_1]
REF_3118(mapping(address => uint256)) -> REF_3117[validatorId_1]
REF_3119(uint256) -> REF_3118[token_1]
$_3 (-> ['TMP_13137'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13137'])"])
REF_3119(uint256) (->$_3 (-> ['TMP_13137'])) := block.timestamp(uint256)
TMP_13137(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13137'])"])
 $.userValidatorRewardPerTokenPaid[user][validatorId][token] = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_3120(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_3 (-> ['TMP_13137']).userValidatorRewardPerTokenPaid
REF_3121(mapping(uint16 => mapping(address => uint256))) -> REF_3120[user_1]
REF_3122(mapping(address => uint256)) -> REF_3121[validatorId_1]
REF_3123(uint256) -> REF_3122[token_1]
REF_3124(mapping(uint16 => mapping(address => uint256))) -> $_3 (-> ['TMP_13137']).validatorRewardPerTokenCumulative
REF_3125(mapping(address => uint256)) -> REF_3124[validatorId_1]
REF_3126(uint256) -> REF_3125[token_1]
$_4 (-> ['TMP_13137'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13137'])"])
REF_3123(uint256) (->$_4 (-> ['TMP_13137'])) := REF_3126(uint256)
TMP_13137(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13137'])"])
```
#### RewardsFacet._finalizeRewardClaim(address,uint256,address) [INTERNAL]
```slithir
token_1(address) := phi(['token_1', 'token_1', 'token_1'])
totalAmount_1(uint256) := phi(['reward_1', 'totalReward_1', 'totalReward_1'])
recipient_1(address) := phi(['msg.sender'])
 totalAmount == 0
TMP_13138(bool) = totalAmount_1 == 0
CONDITION TMP_13138
 $ = PlumeStakingStorage.layout()
TMP_13139(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13139'])(PlumeStakingStorage.Layout) := TMP_13139(PlumeStakingStorage.Layout)
 $.totalClaimableByToken[token] >= totalAmount
REF_3128(mapping(address => uint256)) -> $_1 (-> ['TMP_13139']).totalClaimableByToken
REF_3129(uint256) -> REF_3128[token_1]
TMP_13140(bool) = REF_3129 >= totalAmount_1
CONDITION TMP_13140
 $.totalClaimableByToken[token] -= totalAmount
REF_3130(mapping(address => uint256)) -> $_1 (-> ['TMP_13139']).totalClaimableByToken
REF_3131(uint256) -> REF_3130[token_1]
$_3 (-> ['TMP_13139'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13139'])"])
REF_3131(-> $_3 (-> ['TMP_13139'])) = REF_3131 (c)- totalAmount_1
TMP_13139(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13139'])"])
 $.totalClaimableByToken[token] = 0
REF_3132(mapping(address => uint256)) -> $_1 (-> ['TMP_13139']).totalClaimableByToken
REF_3133(uint256) -> REF_3132[token_1]
$_2 (-> ['TMP_13139'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13139'])"])
REF_3133(uint256) (->$_2 (-> ['TMP_13139'])) := 0(uint256)
TMP_13139(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13139'])"])
 _transferRewardFromTreasury(token,totalAmount,recipient)
INTERNAL_CALL, RewardsFacet._transferRewardFromTreasury(address,uint256,address)(token_1,totalAmount_1,recipient_1)
```
#### RewardsFacet._clearPendingRewardFlags(address,uint16[]) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender'])
validatorIds_1(uint16[]) := phi(['validatorIds_1', 'validatorIds_1'])
 $ = PlumeStakingStorage.layout()
TMP_13142(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13142'])(PlumeStakingStorage.Layout) := TMP_13142(PlumeStakingStorage.Layout)
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3135 -> LENGTH validatorIds_1
TMP_13143(bool) = i_2 < REF_3135
CONDITION TMP_13143
 PlumeRewardLogic.clearPendingRewardsFlagIfEmpty($,user,validatorIds[i])
REF_3137(uint16) -> validatorIds_1[i_2]
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.clearPendingRewardsFlagIfEmpty(PlumeStakingStorage.Layout,address,uint16), arguments:["$_1 (-> ['TMP_13142'])", 'user_1', 'REF_3137'] 
 i ++
TMP_13145(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### RewardsFacet._processAllValidatorRewards(address,address) [INTERNAL]
```slithir
user_1(address) := phi(['msg.sender'])
token_1(address) := phi(['token_1', 'token_1'])
 $ = PlumeStakingStorage.layout()
TMP_13146(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13146'])(PlumeStakingStorage.Layout) := TMP_13146(PlumeStakingStorage.Layout)
 validatorIds = $.userValidators[user]
REF_3139(mapping(address => uint16[])) -> $_1 (-> ['TMP_13146']).userValidators
REF_3140(uint16[]) -> REF_3139[user_1]
validatorIds_1(uint16[]) = ['REF_3140(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3141 -> LENGTH validatorIds_1
TMP_13147(bool) = i_2 < REF_3141
CONDITION TMP_13147
 validatorId = validatorIds[i]
REF_3142(uint16) -> validatorIds_1[i_2]
validatorId_1(uint16) := REF_3142(uint16)
 $.validators[validatorId].slashed
REF_3143(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13146']).validators
REF_3144(PlumeStakingStorage.ValidatorInfo) -> REF_3143[validatorId_1]
REF_3145(bool) -> REF_3144.slashed
CONDITION REF_3145
 storedReward = $.userRewards[user][validatorId][token]
REF_3146(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13146']).userRewards
REF_3147(mapping(uint16 => mapping(address => uint256))) -> REF_3146[user_1]
REF_3148(mapping(address => uint256)) -> REF_3147[validatorId_1]
REF_3149(uint256) -> REF_3148[token_1]
storedReward_1(uint256) := REF_3149(uint256)
 storedReward > 0
TMP_13148(bool) = storedReward_1 > 0
CONDITION TMP_13148
 _updateUserRewardState(user,validatorId,token,storedReward)
INTERNAL_CALL, RewardsFacet._updateUserRewardState(address,uint16,address,uint256)(user_1,validatorId_1,token_1,storedReward_1)
 totalReward += storedReward
totalReward_3(uint256) = totalReward_0 (c)+ storedReward_1
 RewardClaimedFromValidator(user,token,validatorId,storedReward)
Emit RewardClaimedFromValidator(user_1,token_1,validatorId_1,storedReward_1)
totalReward_4(uint256) := phi(['totalReward_3', 'totalReward_0'])
 ! $.validators[validatorId].active
REF_3150(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13146']).validators
REF_3151(PlumeStakingStorage.ValidatorInfo) -> REF_3150[validatorId_1]
REF_3152(bool) -> REF_3151.active
TMP_13151 = UnaryType.BANG REF_3152 
CONDITION TMP_13151
 rewardFromValidator = _processValidatorRewards(user,validatorId,token)
TMP_13152(uint256) = INTERNAL_CALL, RewardsFacet._processValidatorRewards(address,uint16,address)(user_1,validatorId_1,token_1)
rewardFromValidator_1(uint256) := TMP_13152(uint256)
 totalReward += rewardFromValidator
totalReward_1(uint256) = totalReward_0 (c)+ rewardFromValidator_1
 i ++
totalReward_2(uint256) := phi(['totalReward_0', 'totalReward_1'])
TMP_13153(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 totalReward
RETURN totalReward_0
 totalReward
```
#### RewardsFacet._transferRewardFromTreasury(address,uint256,address) [INTERNAL]
```slithir
token_1(address) := phi(['token_1'])
amount_1(uint256) := phi(['totalAmount_1'])
recipient_1(address) := phi(['recipient_1'])
 treasury = getTreasuryAddress()
TMP_13154(address) = INTERNAL_CALL, RewardsFacet.getTreasuryAddress()()
treasury_1(address) := TMP_13154(address)
 treasury == address(0)
TMP_13155 = CONVERT 0 to address
TMP_13156(bool) = treasury_1 == TMP_13155
CONDITION TMP_13156
 revert TreasuryNotSet()()
TMP_13157(None) = SOLIDITY_CALL revert TreasuryNotSet()()
 IPlumeStakingRewardTreasury(treasury).distributeReward(token,amount,recipient)
TMP_13158 = CONVERT treasury_1 to IPlumeStakingRewardTreasury
HIGH_LEVEL_CALL, dest:TMP_13158(IPlumeStakingRewardTreasury), function:distributeReward, arguments:['token_1', 'amount_1', 'recipient_1']
```
#### RewardsFacet._isRewardToken(address) [INTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13160(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13160'])(PlumeStakingStorage.Layout) := TMP_13160(PlumeStakingStorage.Layout)
 $.isRewardToken[token]
REF_3155(mapping(address => bool)) -> $_1 (-> ['TMP_13160']).isRewardToken
REF_3156(bool) -> REF_3155[token_1]
RETURN REF_3156
```
#### RewardsFacet._getTokenIndex(address) [INTERNAL]
```slithir
token_1(address) := phi(['token_1'])
 $ = PlumeStakingStorage.layout()
TMP_13161(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13161'])(PlumeStakingStorage.Layout) := TMP_13161(PlumeStakingStorage.Layout)
 ! $.isRewardToken[token]
REF_3158(mapping(address => bool)) -> $_1 (-> ['TMP_13161']).isRewardToken
REF_3159(bool) -> REF_3158[token_1]
TMP_13162 = UnaryType.BANG REF_3159 
CONDITION TMP_13162
 revert TokenDoesNotExist(address)(token)
TMP_13163(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 rewardTokens = $.rewardTokens
REF_3160(address[]) -> $_1 (-> ['TMP_13161']).rewardTokens
rewardTokens_1(address[]) = ['REF_3160(address[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < rewardTokens.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3161 -> LENGTH rewardTokens_1
TMP_13164(bool) = i_2 < REF_3161
CONDITION TMP_13164
 rewardTokens[i] == token
REF_3162(address) -> rewardTokens_1[i_2]
TMP_13165(bool) = REF_3162 == token_1
CONDITION TMP_13165
 i
RETURN i_2
 i ++
TMP_13166(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 revert InternalInconsistency(string)(Reward token map/array mismatch)
TMP_13167(None) = SOLIDITY_CALL revert InternalInconsistency(string)(Reward token map/array mismatch)
```
#### RewardsFacet.earned(address,address) [EXTERNAL]
```slithir
 _calculateTotalEarned(user,token)
TMP_13168(uint256) = INTERNAL_CALL, RewardsFacet._calculateTotalEarned(address,address)(user_1,token_1)
RETURN TMP_13168
```
#### RewardsFacet.getClaimableReward(address,address) [EXTERNAL]
```slithir
 this.earned(user,token)
TMP_13169(uint256) = HIGH_LEVEL_CALL, dest:this(address), function:earned, arguments:['user_1', 'token_1']  
RETURN TMP_13169
```
#### RewardsFacet.getRewardTokens() [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13170(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13170'])(PlumeStakingStorage.Layout) := TMP_13170(PlumeStakingStorage.Layout)
 $.rewardTokens
REF_3165(address[]) -> $_1 (-> ['TMP_13170']).rewardTokens
RETURN REF_3165
```
#### RewardsFacet.getMaxRewardRate(address) [EXTERNAL]
```slithir
MAX_REWARD_RATE_4(uint256) := phi(['MAX_REWARD_RATE_0', 'MAX_REWARD_RATE_3'])
 $ = PlumeStakingStorage.layout()
TMP_13171(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13171'])(PlumeStakingStorage.Layout) := TMP_13171(PlumeStakingStorage.Layout)
 ! $.isRewardToken[token]
REF_3167(mapping(address => bool)) -> $_1 (-> ['TMP_13171']).isRewardToken
REF_3168(bool) -> REF_3167[token_1]
TMP_13172 = UnaryType.BANG REF_3168 
CONDITION TMP_13172
 revert TokenDoesNotExist(address)(token)
TMP_13173(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 $.maxRewardRates[token] > 0
REF_3169(mapping(address => uint256)) -> $_1 (-> ['TMP_13171']).maxRewardRates
REF_3170(uint256) -> REF_3169[token_1]
TMP_13174(bool) = REF_3170 > 0
CONDITION TMP_13174
 $.maxRewardRates[token]
REF_3171(mapping(address => uint256)) -> $_1 (-> ['TMP_13171']).maxRewardRates
REF_3172(uint256) -> REF_3171[token_1]
RETURN REF_3172
 MAX_REWARD_RATE
RETURN MAX_REWARD_RATE_4
```
#### RewardsFacet.getRewardRate(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13175(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13175'])(PlumeStakingStorage.Layout) := TMP_13175(PlumeStakingStorage.Layout)
 $.rewardRates[token]
REF_3174(mapping(address => uint256)) -> $_1 (-> ['TMP_13175']).rewardRates
REF_3175(uint256) -> REF_3174[token_1]
RETURN REF_3175
```
#### RewardsFacet.tokenRewardInfo(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13176(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13176'])(PlumeStakingStorage.Layout) := TMP_13176(PlumeStakingStorage.Layout)
 rewardRate = $.rewardRates[token]
REF_3177(mapping(address => uint256)) -> $_1 (-> ['TMP_13176']).rewardRates
REF_3178(uint256) -> REF_3177[token_1]
rewardRate_1(uint256) := REF_3178(uint256)
 validatorIds = $.validatorIds
REF_3179(uint16[]) -> $_1 (-> ['TMP_13176']).validatorIds
validatorIds_1(uint16[]) = ['REF_3179(uint16[])']
 lastUpdateTime = 0
lastUpdateTime_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3180 -> LENGTH validatorIds_1
TMP_13177(bool) = i_2 < REF_3180
CONDITION TMP_13177
 validatorUpdateTime = $.validatorLastUpdateTimes[validatorIds[i]][token]
REF_3181(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> ['TMP_13176']).validatorLastUpdateTimes
REF_3182(uint16) -> validatorIds_1[i_2]
REF_3183(mapping(address => uint256)) -> REF_3181[REF_3182]
REF_3184(uint256) -> REF_3183[token_1]
validatorUpdateTime_1(uint256) := REF_3184(uint256)
 validatorUpdateTime > lastUpdateTime
TMP_13178(bool) = validatorUpdateTime_1 > lastUpdateTime_1
CONDITION TMP_13178
 lastUpdateTime = validatorUpdateTime
lastUpdateTime_2(uint256) := validatorUpdateTime_1(uint256)
lastUpdateTime_3(uint256) := phi(['lastUpdateTime_2', 'lastUpdateTime_1'])
 i ++
TMP_13179(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 (rewardRate,lastUpdateTime)
RETURN rewardRate_1,lastUpdateTime_1
```
#### RewardsFacet.getRewardRateCheckpointCount(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13180(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13180'])(PlumeStakingStorage.Layout) := TMP_13180(PlumeStakingStorage.Layout)
 $.rewardRateCheckpoints[token].length
REF_3186(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> ['TMP_13180']).rewardRateCheckpoints
REF_3187(PlumeStakingStorage.RateCheckpoint[]) -> REF_3186[token_1]
REF_3188 -> LENGTH REF_3187
RETURN REF_3188
```
#### RewardsFacet.getValidatorRewardRateCheckpointCount(uint16,address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13181(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13181'])(PlumeStakingStorage.Layout) := TMP_13181(PlumeStakingStorage.Layout)
 $.validatorRewardRateCheckpoints[validatorId][token].length
REF_3190(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> ['TMP_13181']).validatorRewardRateCheckpoints
REF_3191(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_3190[validatorId_1]
REF_3192(PlumeStakingStorage.RateCheckpoint[]) -> REF_3191[token_1]
REF_3193 -> LENGTH REF_3192
RETURN REF_3193
```
#### RewardsFacet.getUserLastCheckpointIndex(address,uint16,address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13182(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13182'])(PlumeStakingStorage.Layout) := TMP_13182(PlumeStakingStorage.Layout)
 $.userLastCheckpointIndex[user][validatorId][token]
REF_3195(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> ['TMP_13182']).userLastCheckpointIndex
REF_3196(mapping(uint16 => mapping(address => uint256))) -> REF_3195[user_1]
REF_3197(mapping(address => uint256)) -> REF_3196[validatorId_1]
REF_3198(uint256) -> REF_3197[token_1]
RETURN REF_3198
```
#### RewardsFacet.getRewardRateCheckpoint(address,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13183(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13183'])(PlumeStakingStorage.Layout) := TMP_13183(PlumeStakingStorage.Layout)
 index >= $.rewardRateCheckpoints[token].length
REF_3200(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> ['TMP_13183']).rewardRateCheckpoints
REF_3201(PlumeStakingStorage.RateCheckpoint[]) -> REF_3200[token_1]
REF_3202 -> LENGTH REF_3201
TMP_13184(bool) = index_1 >= REF_3202
CONDITION TMP_13184
 revert InvalidRewardRateCheckpoint(address,uint256)(token,index)
TMP_13185(None) = SOLIDITY_CALL revert InvalidRewardRateCheckpoint(address,uint256)(token_1,index_1)
 checkpoint = $.rewardRateCheckpoints[token][index]
REF_3203(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> ['TMP_13183']).rewardRateCheckpoints
REF_3204(PlumeStakingStorage.RateCheckpoint[]) -> REF_3203[token_1]
REF_3205(PlumeStakingStorage.RateCheckpoint) -> REF_3204[index_1]
checkpoint_1(PlumeStakingStorage.RateCheckpoint) := REF_3205(PlumeStakingStorage.RateCheckpoint)
 (checkpoint.timestamp,checkpoint.rate,checkpoint.cumulativeIndex)
REF_3206(uint256) -> checkpoint_1.timestamp
REF_3207(uint256) -> checkpoint_1.rate
REF_3208(uint256) -> checkpoint_1.cumulativeIndex
RETURN REF_3206,REF_3207,REF_3208
 (timestamp,rate,cumulativeIndex)
```
#### RewardsFacet.getValidatorRewardRateCheckpoint(uint16,address,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13186(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13186'])(PlumeStakingStorage.Layout) := TMP_13186(PlumeStakingStorage.Layout)
 index >= $.validatorRewardRateCheckpoints[validatorId][token].length
REF_3210(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> ['TMP_13186']).validatorRewardRateCheckpoints
REF_3211(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_3210[validatorId_1]
REF_3212(PlumeStakingStorage.RateCheckpoint[]) -> REF_3211[token_1]
REF_3213 -> LENGTH REF_3212
TMP_13187(bool) = index_1 >= REF_3213
CONDITION TMP_13187
 revert InvalidRewardRateCheckpoint(address,uint256)(token,index)
TMP_13188(None) = SOLIDITY_CALL revert InvalidRewardRateCheckpoint(address,uint256)(token_1,index_1)
 checkpoint = $.validatorRewardRateCheckpoints[validatorId][token][index]
REF_3214(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> ['TMP_13186']).validatorRewardRateCheckpoints
REF_3215(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_3214[validatorId_1]
REF_3216(PlumeStakingStorage.RateCheckpoint[]) -> REF_3215[token_1]
REF_3217(PlumeStakingStorage.RateCheckpoint) -> REF_3216[index_1]
checkpoint_1(PlumeStakingStorage.RateCheckpoint) := REF_3217(PlumeStakingStorage.RateCheckpoint)
 (checkpoint.timestamp,checkpoint.rate,checkpoint.cumulativeIndex)
REF_3218(uint256) -> checkpoint_1.timestamp
REF_3219(uint256) -> checkpoint_1.rate
REF_3220(uint256) -> checkpoint_1.cumulativeIndex
RETURN REF_3218,REF_3219,REF_3220
 (timestamp,rate,cumulativeIndex)
```
#### RewardsFacet.getTreasury() [EXTERNAL]
```slithir
 getTreasuryAddress()
TMP_13189(address) = INTERNAL_CALL, RewardsFacet.getTreasuryAddress()()
RETURN TMP_13189
```
#### RewardsFacet.getPendingRewardForValidator(address,uint16,address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13190(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13190'])(PlumeStakingStorage.Layout) := TMP_13190(PlumeStakingStorage.Layout)
 userStakedAmount = $.userValidatorStakes[user][validatorId].staked
REF_3222(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> ['TMP_13190']).userValidatorStakes
REF_3223(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3222[user_1]
REF_3224(PlumeStakingStorage.UserValidatorStake) -> REF_3223[validatorId_1]
REF_3225(uint256) -> REF_3224.staked
userStakedAmount_1(uint256) := REF_3225(uint256)
 (userRewardDelta,None,None) = PlumeRewardLogic.calculateRewardsWithCheckpoints($,user,validatorId,token,userStakedAmount)
TUPLE_89(uint256,uint256,uint256) = LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.calculateRewardsWithCheckpoints(PlumeStakingStorage.Layout,address,uint16,address,uint256), arguments:["$_1 (-> ['TMP_13190'])", 'user_1', 'validatorId_1', 'token_1', 'userStakedAmount_1'] 
userRewardDelta_1(uint256)= UNPACK TUPLE_89 index: 0 
 userRewardDelta
RETURN userRewardDelta_1
 pendingReward
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
#### PlumeRewardLogic.createRewardRateCheckpoint(PlumeStakingStorage.Layout,address,uint16,uint256) [INTERNAL]
```slithir
 len_before = 0
len_before_1(uint256) := 0(uint256)
 len_before = $.validatorRewardRateCheckpoints[validatorId][token].length
REF_4266(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> []).validatorRewardRateCheckpoints
REF_4267(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4266[validatorId_1]
REF_4268(PlumeStakingStorage.RateCheckpoint[]) -> REF_4267[token_1]
REF_4269 -> LENGTH REF_4268
len_before_2(uint256) := REF_4269(uint256)
 updateRewardPerTokenForValidator($,token,validatorId)
INTERNAL_CALL, PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16)($_1 (-> []),token_1,validatorId_1)
 currentCumulativeIndex = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_4270(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorRewardPerTokenCumulative
REF_4271(mapping(address => uint256)) -> REF_4270[validatorId_1]
REF_4272(uint256) -> REF_4271[token_1]
currentCumulativeIndex_1(uint256) := REF_4272(uint256)
 checkpoint = PlumeStakingStorage.RateCheckpoint({timestamp:block.timestamp,rate:rate,cumulativeIndex:currentCumulativeIndex})
TMP_14001(PlumeStakingStorage.RateCheckpoint) = new RateCheckpoint(block.timestamp,rate_1,currentCumulativeIndex_1)
checkpoint_1(PlumeStakingStorage.RateCheckpoint) := TMP_14001(PlumeStakingStorage.RateCheckpoint)
 $.validatorRewardRateCheckpoints[validatorId][token].push(checkpoint)
REF_4274(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> []).validatorRewardRateCheckpoints
REF_4275(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4274[validatorId_1]
REF_4276(PlumeStakingStorage.RateCheckpoint[]) -> REF_4275[token_1]
REF_4278 -> LENGTH REF_4276
TMP_14003(uint256) := REF_4278(uint256)
TMP_14004(uint256) = TMP_14003 (c)+ 1
$_2 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4278(uint256) (->$_3 (-> [])) := TMP_14004(uint256)
REF_4279(PlumeStakingStorage.RateCheckpoint) -> REF_4276[TMP_14003]
$_3 (-> [])(PlumeStakingStorage.Layout) := phi(['$_2 (-> [])'])
REF_4279(PlumeStakingStorage.RateCheckpoint) (->$_3 (-> [])) := checkpoint_1(PlumeStakingStorage.RateCheckpoint)
 len_after = $.validatorRewardRateCheckpoints[validatorId][token].length
REF_4280(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_3 (-> []).validatorRewardRateCheckpoints
REF_4281(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4280[validatorId_1]
REF_4282(PlumeStakingStorage.RateCheckpoint[]) -> REF_4281[token_1]
REF_4283 -> LENGTH REF_4282
len_after_1(uint256) := REF_4283(uint256)
 checkpointIndex = len_after - 1
TMP_14005(uint256) = len_after_1 (c)- 1
checkpointIndex_1(uint256) := TMP_14005(uint256)
 RewardRateCheckpointCreated(token,validatorId,rate,block.timestamp,checkpointIndex,currentCumulativeIndex)
Emit RewardRateCheckpointCreated(token_1,validatorId_1,rate_1,block.timestamp,checkpointIndex_1,currentCumulativeIndex_1)
```


#### IPlumeStakingRewardTreasury.distributeReward(address,uint256,address) [EXTERNAL]
```slithir

```
#### PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_1 (-> [])'])
token_1(address) := phi(['token_1', 'token_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
timestamp_1(uint256) := phi(['block.timestamp', 'segmentStartTime_1', 'effectiveTimestampForUpdate_3'])
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
