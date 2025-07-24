
### Storage layout (Spin) 

```text
admin address
lastJackpotClaimWeek uint256
userData mapping(address => Spin.UserData)
jackpotProbabilities uint256[7]
baseRaffleMultiplier uint256
PP_PerSpin uint256
plumeAmounts uint256[3]
userNonce mapping(uint256 => address)
supraRouter ISupraRouterContract
dateTime IDateTime
raffleContract address
campaignStartDate uint256
jackpotPrizes mapping(uint8 => uint256)
whitelists mapping(address => bool)
enableSpin bool
rewardProbabilities Spin.RewardProbabilities
isSpinPending mapping(address => bool)
spinPrice uint256
pendingNonce mapping(address => uint256)
__gap uint256[49]

```
#### Spin._authorizeUpgrade(address) [INTERNAL]
```slithir
newImplementation_1(address) := phi(['newImplementation_1'])
ADMIN_ROLE_41(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_41)
```
#### Spin.initialize(address,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_7', 'DEFAULT_ADMIN_ROLE_0'])
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
SUPRA_ROLE_1(bytes32) := phi(['SUPRA_ROLE_9', 'SUPRA_ROLE_11', 'SUPRA_ROLE_0'])
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 __UUPSUpgradeable_init()
INTERNAL_CALL, UUPSUpgradeable.__UUPSUpgradeable_init()()
 __Pausable_init()
INTERNAL_CALL, PausableUpgradeable.__Pausable_init()()
 __ReentrancyGuard_init()
INTERNAL_CALL, ReentrancyGuardUpgradeable.__ReentrancyGuard_init()()
 _grantRole(DEFAULT_ADMIN_ROLE,msg.sender)
TMP_23920(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_6,msg.sender)
 _grantRole(ADMIN_ROLE,msg.sender)
TMP_23921(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(ADMIN_ROLE_7,msg.sender)
 _grantRole(SUPRA_ROLE,supraRouterAddress)
TMP_23922(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(SUPRA_ROLE_8,supraRouterAddress_1)
 supraRouter = ISupraRouterContract(supraRouterAddress)
TMP_23923 = CONVERT supraRouterAddress_1 to ISupraRouterContract
supraRouter_1(ISupraRouterContract) := TMP_23923(ISupraRouterContract)
 dateTime = IDateTime(dateTimeAddress)
TMP_23924 = CONVERT dateTimeAddress_1 to IDateTime
dateTime_1(IDateTime) := TMP_23924(IDateTime)
 admin = msg.sender
admin_1(address) := msg.sender(address)
 enableSpin = false
enableSpin_1(bool) := False(bool)
 jackpotProbabilities = (1,2,3,5,7,10,20)
jackpotProbabilities_1(uint256[7]) = ['1(uint256)', '2(uint256)', '3(uint256)', '5(uint256)', '7(uint256)', '10(uint256)', '20(uint256)']
 jackpotPrizes[0] = 5000
REF_9910(uint256) -> jackpotPrizes_0[0]
jackpotPrizes_1(mapping(uint8 => uint256)) := phi(['jackpotPrizes_0'])
REF_9910(uint256) (->jackpotPrizes_1) := 5000(uint256)
 jackpotPrizes[1] = 5000
REF_9911(uint256) -> jackpotPrizes_1[1]
jackpotPrizes_2(mapping(uint8 => uint256)) := phi(['jackpotPrizes_1'])
REF_9911(uint256) (->jackpotPrizes_2) := 5000(uint256)
 jackpotPrizes[2] = 10_000
REF_9912(uint256) -> jackpotPrizes_2[2]
jackpotPrizes_3(mapping(uint8 => uint256)) := phi(['jackpotPrizes_2'])
REF_9912(uint256) (->jackpotPrizes_3) := 10000(uint256)
 jackpotPrizes[3] = 10_000
REF_9913(uint256) -> jackpotPrizes_3[3]
jackpotPrizes_4(mapping(uint8 => uint256)) := phi(['jackpotPrizes_3'])
REF_9913(uint256) (->jackpotPrizes_4) := 10000(uint256)
 jackpotPrizes[4] = 20_000
REF_9914(uint256) -> jackpotPrizes_4[4]
jackpotPrizes_5(mapping(uint8 => uint256)) := phi(['jackpotPrizes_4'])
REF_9914(uint256) (->jackpotPrizes_5) := 20000(uint256)
 jackpotPrizes[5] = 20_000
REF_9915(uint256) -> jackpotPrizes_5[5]
jackpotPrizes_6(mapping(uint8 => uint256)) := phi(['jackpotPrizes_5'])
REF_9915(uint256) (->jackpotPrizes_6) := 20000(uint256)
 jackpotPrizes[6] = 30_000
REF_9916(uint256) -> jackpotPrizes_6[6]
jackpotPrizes_7(mapping(uint8 => uint256)) := phi(['jackpotPrizes_6'])
REF_9916(uint256) (->jackpotPrizes_7) := 30000(uint256)
 jackpotPrizes[7] = 30_000
REF_9917(uint256) -> jackpotPrizes_7[7]
jackpotPrizes_8(mapping(uint8 => uint256)) := phi(['jackpotPrizes_7'])
REF_9917(uint256) (->jackpotPrizes_8) := 30000(uint256)
 jackpotPrizes[8] = 40_000
REF_9918(uint256) -> jackpotPrizes_8[8]
jackpotPrizes_9(mapping(uint8 => uint256)) := phi(['jackpotPrizes_8'])
REF_9918(uint256) (->jackpotPrizes_9) := 40000(uint256)
 jackpotPrizes[9] = 40_000
REF_9919(uint256) -> jackpotPrizes_9[9]
jackpotPrizes_10(mapping(uint8 => uint256)) := phi(['jackpotPrizes_9'])
REF_9919(uint256) (->jackpotPrizes_10) := 40000(uint256)
 jackpotPrizes[10] = 50_000
REF_9920(uint256) -> jackpotPrizes_10[10]
jackpotPrizes_11(mapping(uint8 => uint256)) := phi(['jackpotPrizes_10'])
REF_9920(uint256) (->jackpotPrizes_11) := 50000(uint256)
 jackpotPrizes[11] = 100_000
REF_9921(uint256) -> jackpotPrizes_11[11]
jackpotPrizes_12(mapping(uint8 => uint256)) := phi(['jackpotPrizes_11'])
REF_9921(uint256) (->jackpotPrizes_12) := 100000(uint256)
 baseRaffleMultiplier = 8
baseRaffleMultiplier_1(uint256) := 8(uint256)
 PP_PerSpin = 100
PP_PerSpin_1(uint256) := 100(uint256)
 plumeAmounts = (1,1,1)
plumeAmounts_1(uint256[3]) = ['1(uint256)', '1(uint256)', '1(uint256)']
 lastJackpotClaimWeek = 999
lastJackpotClaimWeek_1(uint256) := 999(uint256)
 spinPrice = 2000000000000000000
spinPrice_1(uint256) := 2000000000000000000(uint256)
 rewardProbabilities = RewardProbabilities({plumeTokenThreshold:200_000,raffleTicketThreshold:600_000,ppThreshold:900_000})
TMP_23925(Spin.RewardProbabilities) = new RewardProbabilities(200000,600000,900000)
rewardProbabilities_1(Spin.RewardProbabilities) := TMP_23925(Spin.RewardProbabilities)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### Spin.startSpin() [EXTERNAL]
```slithir
admin_2(address) := phi(['admin_1', 'admin_5', 'admin_0'])
supraRouter_2(ISupraRouterContract) := phi(['supraRouter_5', 'supraRouter_1', 'supraRouter_0'])
enableSpin_2(bool) := phi(['enableSpin_1', 'enableSpin_4', 'enableSpin_5', 'enableSpin_0'])
isSpinPending_1(mapping(address => bool)) := phi(['isSpinPending_5', 'isSpinPending_0', 'isSpinPending_4', 'isSpinPending_8'])
spinPrice_2(uint256) := phi(['spinPrice_4', 'spinPrice_1', 'spinPrice_6', 'spinPrice_0'])
 ! enableSpin
TMP_23927 = UnaryType.BANG enableSpin_4 
CONDITION TMP_23927
 revert CampaignNotStarted()()
TMP_23928(None) = SOLIDITY_CALL revert CampaignNotStarted()()
 require(bool,string)(msg.value == spinPrice,Incorrect spin price sent)
TMP_23929(bool) = msg.value == spinPrice_4
TMP_23930(None) = SOLIDITY_CALL require(bool,string)(TMP_23929,Incorrect spin price sent)
 isSpinPending[msg.sender]
REF_9922(bool) -> isSpinPending_3[msg.sender]
CONDITION REF_9922
 revert SpinRequestPending(address)(msg.sender)
TMP_23931(None) = SOLIDITY_CALL revert SpinRequestPending(address)(msg.sender)
 isSpinPending[msg.sender] = true
REF_9923(bool) -> isSpinPending_3[msg.sender]
isSpinPending_4(mapping(address => bool)) := phi(['isSpinPending_3'])
REF_9923(bool) (->isSpinPending_4) := True(bool)
 callbackSignature = handleRandomness(uint256,uint256[])
callbackSignature_1(string) := handleRandomness(uint256,uint256[])(string)
 rngCount = 1
rngCount_1(uint8) := 1(uint256)
 numConfirmations = 1
numConfirmations_1(uint256) := 1(uint256)
 clientSeed = uint256(keccak256(bytes)(abi.encodePacked(admin,block.timestamp)))
TMP_23932(bytes) = SOLIDITY_CALL abi.encodePacked()(admin_4,block.timestamp)
TMP_23933(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23932)
TMP_23934 = CONVERT TMP_23933 to uint256
clientSeed_1(uint256) := TMP_23934(uint256)
 nonce = supraRouter.generateRequest(callbackSignature,rngCount,numConfirmations,clientSeed,admin)
TMP_23935(uint256) = HIGH_LEVEL_CALL, dest:supraRouter_4(ISupraRouterContract), function:generateRequest, arguments:['callbackSignature_1', 'rngCount_1', 'numConfirmations_1', 'clientSeed_1', 'admin_4']  
admin_5(address) := phi(['admin_4', 'admin_1', 'admin_5'])
supraRouter_5(ISupraRouterContract) := phi(['supraRouter_5', 'supraRouter_1', 'supraRouter_4'])
nonce_1(uint256) := TMP_23935(uint256)
 userNonce[nonce] = address(msg.sender)
REF_9926(address) -> userNonce_0[nonce_1]
TMP_23936 = CONVERT msg.sender to address
userNonce_1(mapping(uint256 => address)) := phi(['userNonce_0'])
REF_9926(address) (->userNonce_1) := TMP_23936(address)
 pendingNonce[msg.sender] = nonce
REF_9927(uint256) -> pendingNonce_0[msg.sender]
pendingNonce_1(mapping(address => uint256)) := phi(['pendingNonce_0'])
REF_9927(uint256) (->pendingNonce_1) := nonce_1(uint256)
 SpinRequested(nonce,msg.sender)
Emit SpinRequested(nonce_1,msg.sender)
 whenNotPaused()
MODIFIER_CALL, PausableUpgradeable.whenNotPaused()()
 canSpin()
MODIFIER_CALL, Spin.canSpin()()
```
#### Spin.getCurrentWeek() [PUBLIC]
```slithir
campaignStartDate_1(uint256) := phi(['campaignStartDate_7', 'campaignStartDate_0'])
 (block.timestamp - campaignStartDate) / 604800
TMP_23940(uint256) = block.timestamp (c)- campaignStartDate_1
TMP_23941(uint256) = TMP_23940 (c)/ 604800
RETURN TMP_23941
```
#### Spin.handleRandomness(uint256,uint256[]) [EXTERNAL]
```slithir
SUPRA_ROLE_10(bytes32) := phi(['SUPRA_ROLE_9', 'SUPRA_ROLE_11', 'SUPRA_ROLE_0'])
lastJackpotClaimWeek_2(uint256) := phi(['lastJackpotClaimWeek_8', 'lastJackpotClaimWeek_7', 'lastJackpotClaimWeek_6', 'lastJackpotClaimWeek_1', 'lastJackpotClaimWeek_0'])
userData_1(mapping(address => Spin.UserData)) := phi(['userData_15', 'userData_0', 'userData_16', 'userData_19', 'userData_21', 'userData_20'])
userNonce_2(mapping(uint256 => address)) := phi(['userNonce_7', 'userNonce_8', 'userNonce_0', 'userNonce_1', 'userNonce_5'])
pendingNonce_2(mapping(address => uint256)) := phi(['pendingNonce_0', 'pendingNonce_5', 'pendingNonce_8', 'pendingNonce_1'])
 user = userNonce[nonce]
REF_9928(address) -> userNonce_4[nonce_1]
user_1(address) := REF_9928(address)
 user == address(0)
TMP_23942 = CONVERT 0 to address
TMP_23943(bool) = user_1 == TMP_23942
CONDITION TMP_23943
 revert InvalidNonce()()
TMP_23944(None) = SOLIDITY_CALL revert InvalidNonce()()
 isSpinPending[user] = false
REF_9929(bool) -> isSpinPending_4[user_1]
isSpinPending_5(mapping(address => bool)) := phi(['isSpinPending_4'])
REF_9929(bool) (->isSpinPending_5) := False(bool)
 delete userNonce[nonce]
REF_9930(address) -> userNonce_4[nonce_1]
userNonce_5 = delete REF_9930 
 delete pendingNonce[user]
REF_9931(uint256) -> pendingNonce_4[user_1]
pendingNonce_5 = delete REF_9931 
 currentSpinStreak = _computeStreak(user,block.timestamp,true)
TMP_23945(uint256) = INTERNAL_CALL, Spin._computeStreak(address,uint256,bool)(user_1,block.timestamp,True)
userData_4(mapping(address => Spin.UserData)) := phi(['userData_16'])
currentSpinStreak_1(uint256) := TMP_23945(uint256)
 randomness = rngList[0]
REF_9932(uint256) -> rngList_1[0]
randomness_1(uint256) := REF_9932(uint256)
 (rewardCategory,rewardAmount) = determineReward(randomness,currentSpinStreak)
TUPLE_152(string,uint256) = INTERNAL_CALL, Spin.determineReward(uint256,uint256)(randomness_1,currentSpinStreak_1)
rewardCategory_1(string)= UNPACK TUPLE_152 index: 0 
rewardAmount_1(uint256)= UNPACK TUPLE_152 index: 1 
 userDataStorage = userData[user]
REF_9933(Spin.UserData) -> userData_5[user_1]
userDataStorage_1 (-> ['userData'])(Spin.UserData) := REF_9933(Spin.UserData)
 keccak256(bytes)(bytes(rewardCategory)) == keccak256(bytes)(Jackpot)
TMP_23946 = CONVERT rewardCategory_1 to bytes
TMP_23947(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23946)
TMP_23948(bytes32) = SOLIDITY_CALL keccak256(bytes)(Jackpot)
TMP_23949(bool) = TMP_23947 == TMP_23948
CONDITION TMP_23949
 currentWeek = getCurrentWeek()
TMP_23950(uint256) = INTERNAL_CALL, Spin.getCurrentWeek()()
currentWeek_1(uint256) := TMP_23950(uint256)
 currentWeek == lastJackpotClaimWeek
TMP_23951(bool) = currentWeek_1 == lastJackpotClaimWeek_7
CONDITION TMP_23951
 userDataStorage.nothingCounts += 1
REF_9934(uint256) -> userDataStorage_1 (-> ['userData']).nothingCounts
userDataStorage_2 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9934(-> userDataStorage_2 (-> ['userData'])) = REF_9934 (c)+ 1
userData_6(mapping(address => Spin.UserData)) := phi(["userDataStorage_2 (-> ['userData'])"])
 rewardCategory = Nothing
rewardCategory_2(string) := Nothing(string)
 rewardAmount = 0
rewardAmount_2(uint256) := 0(uint256)
 JackpotAlreadyClaimed(Jackpot already claimed this week)
Emit JackpotAlreadyClaimed(Jackpot already claimed this week)
 userDataStorage.streakCount < (currentWeek + 2)
REF_9935(uint256) -> userDataStorage_1 (-> ['userData']).streakCount
TMP_23953(uint256) = currentWeek_1 (c)+ 2
TMP_23954(bool) = REF_9935 < TMP_23953
CONDITION TMP_23954
 userDataStorage.nothingCounts += 1
REF_9936(uint256) -> userDataStorage_1 (-> ['userData']).nothingCounts
userDataStorage_3 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9936(-> userDataStorage_3 (-> ['userData'])) = REF_9936 (c)+ 1
userData_7(mapping(address => Spin.UserData)) := phi(["userDataStorage_3 (-> ['userData'])"])
 rewardCategory = Nothing
rewardCategory_3(string) := Nothing(string)
 rewardAmount = 0
rewardAmount_3(uint256) := 0(uint256)
 NotEnoughStreak(Not enough streak count to claim Jackpot)
Emit NotEnoughStreak(Not enough streak count to claim Jackpot)
 userDataStorage.jackpotWins ++
REF_9937(uint256) -> userDataStorage_1 (-> ['userData']).jackpotWins
TMP_23956(uint256) := REF_9937(uint256)
userDataStorage_4 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9937(-> userDataStorage_4 (-> ['userData'])) = REF_9937 (c)+ 1
userData_8(mapping(address => Spin.UserData)) := phi(["userDataStorage_4 (-> ['userData'])"])
 lastJackpotClaimWeek = currentWeek
lastJackpotClaimWeek_8(uint256) := currentWeek_1(uint256)
rewardCategory_4(string) := phi(['rewardCategory_3', 'rewardCategory_1'])
rewardAmount_4(uint256) := phi(['rewardAmount_3', 'rewardAmount_1'])
userDataStorage_5 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_3 (-> ['userData'])", "userDataStorage_4 (-> ['userData'])"])
rewardCategory_5(string) := phi(['rewardCategory_1', 'rewardCategory_2'])
rewardAmount_5(uint256) := phi(['rewardAmount_2', 'rewardAmount_1'])
userDataStorage_6 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_2 (-> ['userData'])", "userDataStorage_1 (-> ['userData'])"])
 keccak256(bytes)(bytes(rewardCategory)) == keccak256(bytes)(Raffle Ticket)
TMP_23957 = CONVERT rewardCategory_1 to bytes
TMP_23958(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23957)
TMP_23959(bytes32) = SOLIDITY_CALL keccak256(bytes)(Raffle Ticket)
TMP_23960(bool) = TMP_23958 == TMP_23959
CONDITION TMP_23960
 userDataStorage.raffleTicketsGained += rewardAmount
REF_9938(uint256) -> userDataStorage_1 (-> ['userData']).raffleTicketsGained
userDataStorage_12 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9938(-> userDataStorage_12 (-> ['userData'])) = REF_9938 (c)+ rewardAmount_1
userData_12(mapping(address => Spin.UserData)) := phi(["userDataStorage_12 (-> ['userData'])"])
 userDataStorage.raffleTicketsBalance += rewardAmount
REF_9939(uint256) -> userDataStorage_12 (-> ['userData']).raffleTicketsBalance
userDataStorage_13 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_12 (-> ['userData'])"])
REF_9939(-> userDataStorage_13 (-> ['userData'])) = REF_9939 (c)+ rewardAmount_1
userData_13(mapping(address => Spin.UserData)) := phi(["userDataStorage_13 (-> ['userData'])"])
 keccak256(bytes)(bytes(rewardCategory)) == keccak256(bytes)(PP)
TMP_23961 = CONVERT rewardCategory_1 to bytes
TMP_23962(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23961)
TMP_23963(bytes32) = SOLIDITY_CALL keccak256(bytes)(PP)
TMP_23964(bool) = TMP_23962 == TMP_23963
CONDITION TMP_23964
 userDataStorage.PPGained += rewardAmount
REF_9940(uint256) -> userDataStorage_1 (-> ['userData']).PPGained
userDataStorage_10 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9940(-> userDataStorage_10 (-> ['userData'])) = REF_9940 (c)+ rewardAmount_1
userData_11(mapping(address => Spin.UserData)) := phi(["userDataStorage_10 (-> ['userData'])"])
 keccak256(bytes)(bytes(rewardCategory)) == keccak256(bytes)(Plume Token)
TMP_23965 = CONVERT rewardCategory_1 to bytes
TMP_23966(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23965)
TMP_23967(bytes32) = SOLIDITY_CALL keccak256(bytes)(Plume Token)
TMP_23968(bool) = TMP_23966 == TMP_23967
CONDITION TMP_23968
 userDataStorage.plumeTokens += rewardAmount
REF_9941(uint256) -> userDataStorage_1 (-> ['userData']).plumeTokens
userDataStorage_8 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9941(-> userDataStorage_8 (-> ['userData'])) = REF_9941 (c)+ rewardAmount_1
userData_10(mapping(address => Spin.UserData)) := phi(["userDataStorage_8 (-> ['userData'])"])
 userDataStorage.nothingCounts += 1
REF_9942(uint256) -> userDataStorage_1 (-> ['userData']).nothingCounts
userDataStorage_7 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9942(-> userDataStorage_7 (-> ['userData'])) = REF_9942 (c)+ 1
userData_9(mapping(address => Spin.UserData)) := phi(["userDataStorage_7 (-> ['userData'])"])
userDataStorage_9 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_8 (-> ['userData'])", "userDataStorage_7 (-> ['userData'])"])
userDataStorage_11 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])", "userDataStorage_10 (-> ['userData'])"])
userDataStorage_14 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])", "userDataStorage_13 (-> ['userData'])"])
 userDataStorage.streakCount = currentSpinStreak
REF_9943(uint256) -> userDataStorage_14 (-> ['userData']).streakCount
userDataStorage_15 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_14 (-> ['userData'])"])
REF_9943(uint256) (->userDataStorage_15 (-> ['userData'])) := currentSpinStreak_1(uint256)
userData_14(mapping(address => Spin.UserData)) := phi(["userDataStorage_15 (-> ['userData'])"])
 userDataStorage.lastSpinTimestamp = block.timestamp
REF_9944(uint256) -> userDataStorage_15 (-> ['userData']).lastSpinTimestamp
userDataStorage_16 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_15 (-> ['userData'])"])
REF_9944(uint256) (->userDataStorage_16 (-> ['userData'])) := block.timestamp(uint256)
userData_15(mapping(address => Spin.UserData)) := phi(["userDataStorage_16 (-> ['userData'])"])
 keccak256(bytes)(bytes(rewardCategory)) == keccak256(bytes)(Jackpot) || keccak256(bytes)(bytes(rewardCategory)) == keccak256(bytes)(Plume Token)
TMP_23969 = CONVERT rewardCategory_1 to bytes
TMP_23970(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23969)
TMP_23971(bytes32) = SOLIDITY_CALL keccak256(bytes)(Jackpot)
TMP_23972(bool) = TMP_23970 == TMP_23971
TMP_23973 = CONVERT rewardCategory_1 to bytes
TMP_23974(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23973)
TMP_23975(bytes32) = SOLIDITY_CALL keccak256(bytes)(Plume Token)
TMP_23976(bool) = TMP_23974 == TMP_23975
TMP_23977(bool) = TMP_23972 || TMP_23976
CONDITION TMP_23977
 _safeTransferPlume(user,rewardAmount * 1000000000000000000)
TMP_23978(uint256) = rewardAmount_1 (c)* 1000000000000000000
INTERNAL_CALL, Spin._safeTransferPlume(address,uint256)(user_1,TMP_23978)
 SpinCompleted(user,rewardCategory,rewardAmount)
Emit SpinCompleted(user_1,rewardCategory_1,rewardAmount_1)
 onlyRole(SUPRA_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(SUPRA_ROLE_10)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### Spin.determineReward(uint256,uint256) [INTERNAL]
```slithir
randomness_1(uint256) := phi(['randomness_1'])
streakForReward_1(uint256) := phi(['currentSpinStreak_1'])
jackpotProbabilities_2(uint256[7]) := phi(['jackpotProbabilities_4', 'jackpotProbabilities_3', 'jackpotProbabilities_1', 'jackpotProbabilities_0'])
baseRaffleMultiplier_2(uint256) := phi(['baseRaffleMultiplier_0', 'baseRaffleMultiplier_4', 'baseRaffleMultiplier_1', 'baseRaffleMultiplier_3'])
PP_PerSpin_2(uint256) := phi(['PP_PerSpin_0', 'PP_PerSpin_3', 'PP_PerSpin_1', 'PP_PerSpin_4'])
plumeAmounts_2(uint256[3]) := phi(['plumeAmounts_1', 'plumeAmounts_3', 'plumeAmounts_0', 'plumeAmounts_4'])
campaignStartDate_2(uint256) := phi(['campaignStartDate_7', 'campaignStartDate_0'])
jackpotPrizes_13(mapping(uint8 => uint256)) := phi(['jackpotPrizes_0', 'jackpotPrizes_14', 'jackpotPrizes_12', 'jackpotPrizes_16', 'jackpotPrizes_15'])
rewardProbabilities_2(Spin.RewardProbabilities) := phi(['rewardProbabilities_0', 'rewardProbabilities_8', 'rewardProbabilities_3', 'rewardProbabilities_1'])
 probability = randomness % 1_000_000
TMP_23983(uint256) = randomness_1 % 1000000
probability_1(uint256) := TMP_23983(uint256)
 daysSinceStart = (block.timestamp - campaignStartDate) / 86400
TMP_23984(uint256) = block.timestamp (c)- campaignStartDate_2
TMP_23985(uint256) = TMP_23984 (c)/ 86400
daysSinceStart_1(uint256) := TMP_23985(uint256)
 weekNumber = uint8(getCurrentWeek())
TMP_23986(uint256) = INTERNAL_CALL, Spin.getCurrentWeek()()
TMP_23987 = CONVERT TMP_23986 to uint8
weekNumber_1(uint8) := TMP_23987(uint8)
 dayOfWeek = uint8(daysSinceStart % 7)
TMP_23988(uint256) = daysSinceStart_1 % 7
TMP_23989 = CONVERT TMP_23988 to uint8
dayOfWeek_1(uint8) := TMP_23989(uint8)
 jackpotThreshold = jackpotProbabilities[dayOfWeek]
REF_9945(uint256) -> jackpotProbabilities_3[dayOfWeek_1]
jackpotThreshold_1(uint256) := REF_9945(uint256)
 probability < jackpotThreshold
TMP_23990(bool) = probability_1 < jackpotThreshold_1
CONDITION TMP_23990
 (Jackpot,jackpotPrizes[weekNumber])
REF_9946(uint256) -> jackpotPrizes_14[weekNumber_1]
RETURN Jackpot,REF_9946
 probability <= rewardProbabilities.plumeTokenThreshold
REF_9947(uint256) -> rewardProbabilities_3.plumeTokenThreshold
TMP_23991(bool) = probability_1 <= REF_9947
CONDITION TMP_23991
 plumeAmount = plumeAmounts[probability % 3]
TMP_23992(uint256) = probability_1 % 3
REF_9948(uint256) -> plumeAmounts_3[TMP_23992]
plumeAmount_1(uint256) := REF_9948(uint256)
 (Plume Token,plumeAmount)
RETURN Plume Token,plumeAmount_1
 probability <= rewardProbabilities.raffleTicketThreshold
REF_9949(uint256) -> rewardProbabilities_3.raffleTicketThreshold
TMP_23993(bool) = probability_1 <= REF_9949
CONDITION TMP_23993
 (Raffle Ticket,baseRaffleMultiplier * streakForReward)
TMP_23994(uint256) = baseRaffleMultiplier_3 (c)* streakForReward_1
RETURN Raffle Ticket,TMP_23994
 probability <= rewardProbabilities.ppThreshold
REF_9950(uint256) -> rewardProbabilities_3.ppThreshold
TMP_23995(bool) = probability_1 <= REF_9950
CONDITION TMP_23995
 (PP,PP_PerSpin)
RETURN PP,PP_PerSpin_3
 (Nothing,0)
RETURN Nothing,0
```
#### Spin._computeStreak(address,uint256,bool) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'user_1'])
nowTs_1(uint256) := phi(['block.timestamp'])
SECONDS_PER_DAY_1(uint256) := phi(['SECONDS_PER_DAY_0'])
userData_16(mapping(address => Spin.UserData)) := phi(['userData_15', 'userData_0', 'userData_16', 'userData_19', 'userData_21', 'userData_20'])
 lastSpinTs = userData[user].lastSpinTimestamp
REF_9951(Spin.UserData) -> userData_16[user_1]
REF_9952(uint256) -> REF_9951.lastSpinTimestamp
lastSpinTs_1(uint256) := REF_9952(uint256)
 lastSpinTs == 0
TMP_23996(bool) = lastSpinTs_1 == 0
CONDITION TMP_23996
 0 + streakAdjustment
TMP_23997(uint256) = 0 (c)+ streakAdjustment_3
RETURN TMP_23997
 lastDaySpun = lastSpinTs / SECONDS_PER_DAY
TMP_23998(uint256) = lastSpinTs_1 (c)/ SECONDS_PER_DAY_1
lastDaySpun_1(uint256) := TMP_23998(uint256)
 today = nowTs / SECONDS_PER_DAY
TMP_23999(uint256) = nowTs_1 (c)/ SECONDS_PER_DAY_1
today_1(uint256) := TMP_23999(uint256)
 today == lastDaySpun
TMP_24000(bool) = today_1 == lastDaySpun_1
CONDITION TMP_24000
 userData[user].streakCount
REF_9953(Spin.UserData) -> userData_16[user_1]
REF_9954(uint256) -> REF_9953.streakCount
RETURN REF_9954
 today == lastDaySpun + 1
TMP_24001(uint256) = lastDaySpun_1 (c)+ 1
TMP_24002(bool) = today_1 == TMP_24001
CONDITION TMP_24002
 userData[user].streakCount + streakAdjustment
REF_9955(Spin.UserData) -> userData_16[user_1]
REF_9956(uint256) -> REF_9955.streakCount
TMP_24003(uint256) = REF_9956 (c)+ streakAdjustment_3
RETURN TMP_24003
 0 + streakAdjustment
TMP_24004(uint256) = 0 (c)+ streakAdjustment_3
RETURN TMP_24004
 justSpun
CONDITION justSpun_1
 streakAdjustment = 1
streakAdjustment_2(uint256) := 1(uint256)
 streakAdjustment = 0
streakAdjustment_1(uint256) := 0(uint256)
streakAdjustment_3(uint256) := phi(['streakAdjustment_1', 'streakAdjustment_2'])
```
#### Spin.currentStreak(address) [PUBLIC]
```slithir
user_1(address) := phi(['user_1'])
 _computeStreak(user,block.timestamp,false)
TMP_24005(uint256) = INTERNAL_CALL, Spin._computeStreak(address,uint256,bool)(user_1,block.timestamp,False)
RETURN TMP_24005
```
#### Spin.spendRaffleTickets(address,uint256) [EXTERNAL]
```slithir
userData_17(mapping(address => Spin.UserData)) := phi(['userData_15', 'userData_0', 'userData_16', 'userData_19', 'userData_21', 'userData_20'])
 userDataStorage = userData[user]
REF_9957(Spin.UserData) -> userData_18[user_1]
userDataStorage_1 (-> ['userData'])(Spin.UserData) := REF_9957(Spin.UserData)
 require(bool,string)(userDataStorage.raffleTicketsBalance >= amount,Insufficient raffle tickets)
REF_9958(uint256) -> userDataStorage_1 (-> ['userData']).raffleTicketsBalance
TMP_24006(bool) = REF_9958 >= amount_1
TMP_24007(None) = SOLIDITY_CALL require(bool,string)(TMP_24006,Insufficient raffle tickets)
 userDataStorage.raffleTicketsBalance -= amount
REF_9959(uint256) -> userDataStorage_1 (-> ['userData']).raffleTicketsBalance
userDataStorage_2 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9959(-> userDataStorage_2 (-> ['userData'])) = REF_9959 (c)- amount_1
userData_19(mapping(address => Spin.UserData)) := phi(["userDataStorage_2 (-> ['userData'])"])
 RaffleTicketsSpent(user,amount,userDataStorage.raffleTicketsBalance)
REF_9960(uint256) -> userDataStorage_2 (-> ['userData']).raffleTicketsBalance
Emit RaffleTicketsSpent(user_1,amount_1,REF_9960)
 onlyRaffleContract()
MODIFIER_CALL, Spin.onlyRaffleContract()()
```
#### Spin.adminWithdraw(address,uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_9(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 require(bool,string)(recipient != address(0),Invalid recipient address)
TMP_24010 = CONVERT 0 to address
TMP_24011(bool) = recipient_1 != TMP_24010
TMP_24012(None) = SOLIDITY_CALL require(bool,string)(TMP_24011,Invalid recipient address)
 _safeTransferPlume(recipient,amount)
INTERNAL_CALL, Spin._safeTransferPlume(address,uint256)(recipient_1,amount_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_9)
```
#### Spin.pause() [EXTERNAL]
```slithir
ADMIN_ROLE_11(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 _pause()
INTERNAL_CALL, PausableUpgradeable._pause()()
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_11)
```
#### Spin.unpause() [EXTERNAL]
```slithir
ADMIN_ROLE_13(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 _unpause()
INTERNAL_CALL, PausableUpgradeable._unpause()()
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_13)
```
#### Spin.isSameDay(uint16,uint8,uint8,uint16,uint8,uint8) [INTERNAL]
```slithir
year1_1(uint16) := phi(['lastSpinYear_1'])
month1_1(uint8) := phi(['lastSpinMonth_1'])
day1_1(uint8) := phi(['lastSpinDay_1'])
year2_1(uint16) := phi(['currentYear_1'])
month2_1(uint8) := phi(['currentMonth_1'])
day2_1(uint8) := phi(['currentDay_1'])
 (year1 == year2 && month1 == month2 && day1 == day2)
TMP_24019(bool) = year1_1 == year2_1
TMP_24020(bool) = month1_1 == month2_1
TMP_24021(bool) = TMP_24019 && TMP_24020
TMP_24022(bool) = day1_1 == day2_1
TMP_24023(bool) = TMP_24021 && TMP_24022
RETURN TMP_24023
```
#### Spin.getUserData(address) [EXTERNAL]
```slithir
userData_20(mapping(address => Spin.UserData)) := phi(['userData_15', 'userData_0', 'userData_16', 'userData_19', 'userData_21', 'userData_20'])
 userDataStorage = userData[user]
REF_9961(Spin.UserData) -> userData_20[user_1]
userDataStorage_1 (-> ['userData'])(Spin.UserData) := REF_9961(Spin.UserData)
 (currentStreak(user),userDataStorage.lastSpinTimestamp,userDataStorage.jackpotWins,userDataStorage.raffleTicketsGained,userDataStorage.raffleTicketsBalance,userDataStorage.PPGained,userDataStorage.plumeTokens)
TMP_24024(uint256) = INTERNAL_CALL, Spin.currentStreak(address)(user_1)
REF_9962(uint256) -> userDataStorage_1 (-> ['userData']).lastSpinTimestamp
REF_9963(uint256) -> userDataStorage_1 (-> ['userData']).jackpotWins
REF_9964(uint256) -> userDataStorage_1 (-> ['userData']).raffleTicketsGained
REF_9965(uint256) -> userDataStorage_1 (-> ['userData']).raffleTicketsBalance
REF_9966(uint256) -> userDataStorage_1 (-> ['userData']).PPGained
REF_9967(uint256) -> userDataStorage_1 (-> ['userData']).plumeTokens
RETURN TMP_24024,REF_9962,REF_9963,REF_9964,REF_9965,REF_9966,REF_9967
 (dailyStreak,lastSpinTimestamp,jackpotWins,raffleTicketsGained,raffleTicketsBalance,ppGained,smallPlumeTokens)
```
#### Spin.getWeeklyJackpot() [EXTERNAL]
```slithir
campaignStartDate_3(uint256) := phi(['campaignStartDate_7', 'campaignStartDate_0'])
jackpotPrizes_15(mapping(uint8 => uint256)) := phi(['jackpotPrizes_0', 'jackpotPrizes_14', 'jackpotPrizes_12', 'jackpotPrizes_16', 'jackpotPrizes_15'])
 require(bool,string)(campaignStartDate > 0,Campaign not started)
TMP_24025(bool) = campaignStartDate_3 > 0
TMP_24026(None) = SOLIDITY_CALL require(bool,string)(TMP_24025,Campaign not started)
 daysSinceStart = (block.timestamp - campaignStartDate) / 86400
TMP_24027(uint256) = block.timestamp (c)- campaignStartDate_3
TMP_24028(uint256) = TMP_24027 (c)/ 86400
daysSinceStart_1(uint256) := TMP_24028(uint256)
 weekNumber = daysSinceStart / 7
TMP_24029(uint256) = daysSinceStart_1 (c)/ 7
weekNumber_1(uint256) := TMP_24029(uint256)
 weekNumber > 11
TMP_24030(bool) = weekNumber_1 > 11
CONDITION TMP_24030
 (weekNumber,0,0)
RETURN weekNumber_1,0,0
 jackpotPrize = jackpotPrizes[uint8(weekNumber)]
TMP_24031 = CONVERT weekNumber_1 to uint8
REF_9968(uint256) -> jackpotPrizes_15[TMP_24031]
jackpotPrize_1(uint256) := REF_9968(uint256)
 requiredStreak = weekNumber + 2
TMP_24032(uint256) = weekNumber_1 (c)+ 2
requiredStreak_1(uint256) := TMP_24032(uint256)
 (weekNumber,jackpotPrize,requiredStreak)
RETURN weekNumber_1,jackpotPrize_1,requiredStreak_1
```
#### Spin.getCampaignStartDate() [EXTERNAL]
```slithir
campaignStartDate_4(uint256) := phi(['campaignStartDate_7', 'campaignStartDate_0'])
 campaignStartDate
RETURN campaignStartDate_4
```
#### Spin.getContractBalance() [EXTERNAL]
```slithir
 address(this).balance
TMP_24033 = CONVERT this to address
TMP_24034(uint256) = SOLIDITY_CALL balance(address)(TMP_24033)
RETURN TMP_24034
```
#### Spin.setJackpotProbabilities(uint8[7]) [EXTERNAL]
```slithir
ADMIN_ROLE_15(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 jackpotProbabilities = _jackpotProbabilities
jackpotProbabilities_4(uint256[7]) := _jackpotProbabilities_1(uint8[7])
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_15)
```
#### Spin.setJackpotPrizes(uint8,uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_17(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 jackpotPrizes[week] = prize
REF_9969(uint256) -> jackpotPrizes_15[week_1]
jackpotPrizes_16(mapping(uint8 => uint256)) := phi(['jackpotPrizes_15'])
REF_9969(uint256) (->jackpotPrizes_16) := prize_1(uint256)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_17)
```
#### Spin.setCampaignStartDate(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_19(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_19)
 start == 0
TMP_24038(bool) = start_1 == 0
CONDITION TMP_24038
 campaignStartDate = block.timestamp
campaignStartDate_5(uint256) := block.timestamp(uint256)
 campaignStartDate = start
campaignStartDate_6(uint256) := start_1(uint256)
campaignStartDate_7(uint256) := phi(['campaignStartDate_5', 'campaignStartDate_6'])
```
#### Spin.setBaseRaffleMultiplier(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_21(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 baseRaffleMultiplier = _baseRaffleMultiplier
baseRaffleMultiplier_4(uint256) := _baseRaffleMultiplier_1(uint256)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_21)
```
#### Spin.setPP_PerSpin(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_23(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 PP_PerSpin = _PP_PerSpin
PP_PerSpin_4(uint256) := _PP_PerSpin_1(uint256)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_23)
```
#### Spin.setPlumeAmounts(uint256[3]) [EXTERNAL]
```slithir
ADMIN_ROLE_25(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 plumeAmounts = _plumeAmounts
plumeAmounts_4(uint256[3]) := _plumeAmounts_1(uint256[3])
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_25)
```
#### Spin.setRaffleContract(address) [EXTERNAL]
```slithir
ADMIN_ROLE_27(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 raffleContract = _raffleContract
raffleContract_1(address) := _raffleContract_1(address)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_27)
```
#### Spin.whitelist(address) [EXTERNAL]
```slithir
ADMIN_ROLE_29(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 whitelists[user] = true
REF_9970(bool) -> whitelists_0[user_1]
whitelists_1(mapping(address => bool)) := phi(['whitelists_0'])
REF_9970(bool) (->whitelists_1) := True(bool)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_29)
```
#### Spin.removeWhitelist(address) [EXTERNAL]
```slithir
ADMIN_ROLE_31(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 whitelists[user] = false
REF_9971(bool) -> whitelists_1[user_1]
whitelists_2(mapping(address => bool)) := phi(['whitelists_1'])
REF_9971(bool) (->whitelists_2) := False(bool)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_31)
```
#### Spin.setEnableSpin(bool) [EXTERNAL]
```slithir
ADMIN_ROLE_33(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 enableSpin = _enableSpin
enableSpin_5(bool) := _enableSpin_1(bool)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_33)
```
#### Spin.setRewardProbabilities(uint256,uint256,uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_35(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
rewardProbabilities_4(Spin.RewardProbabilities) := phi(['rewardProbabilities_0', 'rewardProbabilities_8', 'rewardProbabilities_3', 'rewardProbabilities_1'])
 require(bool,string)(_plumeTokenThreshold < _raffleTicketThreshold,Invalid thresholds order)
TMP_24046(bool) = _plumeTokenThreshold_1 < _raffleTicketThreshold_1
TMP_24047(None) = SOLIDITY_CALL require(bool,string)(TMP_24046,Invalid thresholds order)
 require(bool,string)(_raffleTicketThreshold < _ppThreshold,Invalid thresholds order)
TMP_24048(bool) = _raffleTicketThreshold_1 < _ppThreshold_1
TMP_24049(None) = SOLIDITY_CALL require(bool,string)(TMP_24048,Invalid thresholds order)
 require(bool,string)(_ppThreshold <= 1_000_000,Threshold exceeds maximum)
TMP_24050(bool) = _ppThreshold_1 <= 1000000
TMP_24051(None) = SOLIDITY_CALL require(bool,string)(TMP_24050,Threshold exceeds maximum)
 rewardProbabilities.plumeTokenThreshold = _plumeTokenThreshold
REF_9972(uint256) -> rewardProbabilities_5.plumeTokenThreshold
rewardProbabilities_6(Spin.RewardProbabilities) := phi(['rewardProbabilities_5'])
REF_9972(uint256) (->rewardProbabilities_6) := _plumeTokenThreshold_1(uint256)
 rewardProbabilities.raffleTicketThreshold = _raffleTicketThreshold
REF_9973(uint256) -> rewardProbabilities_6.raffleTicketThreshold
rewardProbabilities_7(Spin.RewardProbabilities) := phi(['rewardProbabilities_6'])
REF_9973(uint256) (->rewardProbabilities_7) := _raffleTicketThreshold_1(uint256)
 rewardProbabilities.ppThreshold = _ppThreshold
REF_9974(uint256) -> rewardProbabilities_7.ppThreshold
rewardProbabilities_8(Spin.RewardProbabilities) := phi(['rewardProbabilities_7'])
REF_9974(uint256) (->rewardProbabilities_8) := _ppThreshold_1(uint256)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_35)
```
#### Spin.getSpinPrice() [EXTERNAL]
```slithir
spinPrice_5(uint256) := phi(['spinPrice_4', 'spinPrice_1', 'spinPrice_6', 'spinPrice_0'])
 spinPrice
RETURN spinPrice_5
```
#### Spin.setSpinPrice(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_37(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
 spinPrice = _newPrice
spinPrice_6(uint256) := _newPrice_1(uint256)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_37)
```
#### Spin.cancelPendingSpin(address) [EXTERNAL]
```slithir
ADMIN_ROLE_39(bytes32) := phi(['ADMIN_ROLE_16', 'ADMIN_ROLE_30', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_36', 'ADMIN_ROLE_38', 'ADMIN_ROLE_10', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_42', 'ADMIN_ROLE_24', 'ADMIN_ROLE_32', 'ADMIN_ROLE_26', 'ADMIN_ROLE_12', 'ADMIN_ROLE_40', 'ADMIN_ROLE_20', 'ADMIN_ROLE_14', 'ADMIN_ROLE_34', 'ADMIN_ROLE_28'])
userNonce_6(mapping(uint256 => address)) := phi(['userNonce_7', 'userNonce_8', 'userNonce_0', 'userNonce_1', 'userNonce_5'])
isSpinPending_6(mapping(address => bool)) := phi(['isSpinPending_5', 'isSpinPending_0', 'isSpinPending_4', 'isSpinPending_8'])
pendingNonce_6(mapping(address => uint256)) := phi(['pendingNonce_0', 'pendingNonce_5', 'pendingNonce_8', 'pendingNonce_1'])
 require(bool,string)(isSpinPending[user],No spin pending for this user)
REF_9975(bool) -> isSpinPending_7[user_1]
TMP_24054(None) = SOLIDITY_CALL require(bool,string)(REF_9975,No spin pending for this user)
 nonce = pendingNonce[user]
REF_9976(uint256) -> pendingNonce_7[user_1]
nonce_1(uint256) := REF_9976(uint256)
 nonce != 0
TMP_24055(bool) = nonce_1 != 0
CONDITION TMP_24055
 delete userNonce[nonce]
REF_9977(address) -> userNonce_7[nonce_1]
userNonce_8 = delete REF_9977 
 delete pendingNonce[user]
REF_9978(uint256) -> pendingNonce_7[user_1]
pendingNonce_8 = delete REF_9978 
 isSpinPending[user] = false
REF_9979(bool) -> isSpinPending_7[user_1]
isSpinPending_8(mapping(address => bool)) := phi(['isSpinPending_7'])
REF_9979(bool) (->isSpinPending_8) := False(bool)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_39)
```
#### Spin._safeTransferPlume(address,uint256) [INTERNAL]
```slithir
_to_1(address) := phi(['recipient_1', 'user_1'])
_amount_1(uint256) := phi(['TMP_23978', 'amount_1'])
 require(bool,string)(address(this).balance >= _amount,insufficient Plume in the Spin contract)
TMP_24057 = CONVERT this to address
TMP_24058(uint256) = SOLIDITY_CALL balance(address)(TMP_24057)
TMP_24059(bool) = TMP_24058 >= _amount_1
TMP_24060(None) = SOLIDITY_CALL require(bool,string)(TMP_24059,insufficient Plume in the Spin contract)
 (success,None) = _to.call{value: _amount}()
TUPLE_153(bool,bytes) = LOW_LEVEL_CALL, dest:_to_1, function:call, arguments:[''] value:_amount_1 
success_1(bool)= UNPACK TUPLE_153 index: 0 
 require(bool,string)(success,Plume transfer failed)
TMP_24061(None) = SOLIDITY_CALL require(bool,string)(success_1,Plume transfer failed)
```
#### Spin.receive() [EXTERNAL]
```slithir

```
#### Raffle.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
 UPGRADE_INTERFACE_VERSION = 5.0.0
 ADMIN_ROLE = keccak256(bytes)(ADMIN_ROLE)
 SUPRA_ROLE = keccak256(bytes)(SUPRA_ROLE)
 _checkProxy()
INTERNAL_CALL, UUPSUpgradeable._checkProxy()()
 _checkNotDelegated()
INTERNAL_CALL, UUPSUpgradeable._checkNotDelegated()()
 $ = _getInitializableStorage()
TMP_23786(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_23786'])(Initializable.InitializableStorage) := TMP_23786(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_9859(bool) -> $_1 (-> ['TMP_23786'])._initializing
TMP_23787 = UnaryType.BANG REF_9859 
isTopLevelCall_1(bool) := TMP_23787(bool)
 initialized = $._initialized
REF_9860(uint64) -> $_1 (-> ['TMP_23786'])._initialized
initialized_1(uint64) := REF_9860(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_23788(bool) = initialized_1 == 0
TMP_23789(bool) = TMP_23788 && isTopLevelCall_1
initialSetup_1(bool) := TMP_23789(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_23790(bool) = initialized_1 == 1
TMP_23791 = CONVERT this to address
TMP_23792(bytes) = SOLIDITY_CALL code(address)(TMP_23791)
REF_9861 -> LENGTH TMP_23792
TMP_23793(bool) = REF_9861 == 0
TMP_23794(bool) = TMP_23790 && TMP_23793
construction_1(bool) := TMP_23794(bool)
 ! initialSetup && ! construction
TMP_23795 = UnaryType.BANG initialSetup_1 
TMP_23796 = UnaryType.BANG construction_1 
TMP_23797(bool) = TMP_23795 && TMP_23796
CONDITION TMP_23797
 revert InvalidInitialization()()
TMP_23798(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_9862(uint64) -> $_1 (-> ['TMP_23786'])._initialized
$_2 (-> ['TMP_23786'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_23786'])"])
REF_9862(uint64) (->$_2 (-> ['TMP_23786'])) := 1(uint256)
TMP_23786(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_23786'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_9863(bool) -> $_2 (-> ['TMP_23786'])._initializing
$_3 (-> ['TMP_23786'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_23786'])"])
REF_9863(bool) (->$_3 (-> ['TMP_23786'])) := True(bool)
TMP_23786(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_23786'])"])
$_4 (-> ['TMP_23786'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_23786'])", "$_2 (-> ['TMP_23786'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_9864(bool) -> $_4 (-> ['TMP_23786'])._initializing
$_5 (-> ['TMP_23786'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_23786'])"])
REF_9864(bool) (->$_5 (-> ['TMP_23786'])) := False(bool)
TMP_23786(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_23786'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_23800(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_23800'])(Initializable.InitializableStorage) := TMP_23800(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_9865(bool) -> $_1 (-> ['TMP_23800'])._initializing
REF_9866(uint64) -> $_1 (-> ['TMP_23800'])._initialized
TMP_23801(bool) = REF_9866 >= version_1
TMP_23802(bool) = REF_9865 || TMP_23801
CONDITION TMP_23802
 revert InvalidInitialization()()
TMP_23803(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_9867(uint64) -> $_1 (-> ['TMP_23800'])._initialized
$_2 (-> ['TMP_23800'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_23800'])"])
REF_9867(uint64) (->$_2 (-> ['TMP_23800'])) := version_1(uint64)
TMP_23800(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_23800'])"])
 $._initializing = true
REF_9868(bool) -> $_2 (-> ['TMP_23800'])._initializing
$_3 (-> ['TMP_23800'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_23800'])"])
REF_9868(bool) (->$_3 (-> ['TMP_23800'])) := True(bool)
TMP_23800(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_23800'])"])
 $._initializing = false
REF_9869(bool) -> $_3 (-> ['TMP_23800'])._initializing
$_4 (-> ['TMP_23800'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_23800'])"])
REF_9869(bool) (->$_4 (-> ['TMP_23800'])) := False(bool)
TMP_23800(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_23800'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
role_1(bytes32) := phi(['SUPRA_ROLE_8', 'ADMIN_ROLE_21', 'ADMIN_ROLE_9', 'ADMIN_ROLE_15', 'ADMIN_ROLE_19', 'TMP_23649', 'ADMIN_ROLE_11', 'ADMIN_ROLE_13', 'TMP_23642', 'ADMIN_ROLE_17', 'TMP_23644', 'ADMIN_ROLE_7', 'TMP_23647', 'ADMIN_ROLE_23'])
 _checkRole(role)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(role_1)
prizeId_1(uint256) := phi(['prizeId_1', 'prizeId_1', 'prizeId_1', 'prizeId_1'])
prizes_30(mapping(uint256 => Raffle.Prize)) := phi(['prizes_26', 'prizes_14', 'prizes_16', 'prizes_19', 'prizes_10', 'prizes_0', 'prizes_29', 'prizes_20', 'prizes_30', 'prizes_3', 'prizes_18', 'prizes_21', 'prizes_22'])
 require(bool,string)(prizes[prizeId].isActive,Prize not available)
REF_9870(Raffle.Prize) -> prizes_30[prizeId_1]
REF_9871(bool) -> REF_9870.isActive
TMP_23807(None) = SOLIDITY_CALL require(bool,string)(REF_9871,Prize not available)
```
#### ISupraRouterContract.generateRequest(string,uint8,uint256,address) [EXTERNAL]
```slithir

```
