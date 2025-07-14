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
__gap uint256[50]

```


#### Spin._authorizeUpgrade(address) [INTERNAL]
```slithir
newImplementation_1(address) := phi(['newImplementation_1'])
ADMIN_ROLE_37(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_37)
```
#### Spin.initialize(address,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_7', 'DEFAULT_ADMIN_ROLE_0'])
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
SUPRA_ROLE_1(bytes32) := phi(['SUPRA_ROLE_11', 'SUPRA_ROLE_9', 'SUPRA_ROLE_0'])
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 __UUPSUpgradeable_init()
INTERNAL_CALL, UUPSUpgradeable.__UUPSUpgradeable_init()()
 __Pausable_init()
INTERNAL_CALL, PausableUpgradeable.__Pausable_init()()
 __ReentrancyGuard_init()
INTERNAL_CALL, ReentrancyGuardUpgradeable.__ReentrancyGuard_init()()
 _grantRole(DEFAULT_ADMIN_ROLE,msg.sender)
TMP_23694(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_6,msg.sender)
 _grantRole(ADMIN_ROLE,msg.sender)
TMP_23695(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(ADMIN_ROLE_7,msg.sender)
 _grantRole(SUPRA_ROLE,supraRouterAddress)
TMP_23696(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(SUPRA_ROLE_8,supraRouterAddress_1)
 supraRouter = ISupraRouterContract(supraRouterAddress)
TMP_23697 = CONVERT supraRouterAddress_1 to ISupraRouterContract
supraRouter_1(ISupraRouterContract) := TMP_23697(ISupraRouterContract)
 dateTime = IDateTime(dateTimeAddress)
TMP_23698 = CONVERT dateTimeAddress_1 to IDateTime
dateTime_1(IDateTime) := TMP_23698(IDateTime)
 admin = msg.sender
admin_1(address) := msg.sender(address)
 enableSpin = false
enableSpin_1(bool) := False(bool)
 jackpotProbabilities = (1,2,3,5,7,10,20)
jackpotProbabilities_1(uint256[7]) = ['1(uint256)', '2(uint256)', '3(uint256)', '5(uint256)', '7(uint256)', '10(uint256)', '20(uint256)']
 jackpotPrizes[0] = 5000
REF_9747(uint256) -> jackpotPrizes_0[0]
jackpotPrizes_1(mapping(uint8 => uint256)) := phi(['jackpotPrizes_0'])
REF_9747(uint256) (->jackpotPrizes_1) := 5000(uint256)
 jackpotPrizes[1] = 5000
REF_9748(uint256) -> jackpotPrizes_1[1]
jackpotPrizes_2(mapping(uint8 => uint256)) := phi(['jackpotPrizes_1'])
REF_9748(uint256) (->jackpotPrizes_2) := 5000(uint256)
 jackpotPrizes[2] = 10_000
REF_9749(uint256) -> jackpotPrizes_2[2]
jackpotPrizes_3(mapping(uint8 => uint256)) := phi(['jackpotPrizes_2'])
REF_9749(uint256) (->jackpotPrizes_3) := 10000(uint256)
 jackpotPrizes[3] = 10_000
REF_9750(uint256) -> jackpotPrizes_3[3]
jackpotPrizes_4(mapping(uint8 => uint256)) := phi(['jackpotPrizes_3'])
REF_9750(uint256) (->jackpotPrizes_4) := 10000(uint256)
 jackpotPrizes[4] = 20_000
REF_9751(uint256) -> jackpotPrizes_4[4]
jackpotPrizes_5(mapping(uint8 => uint256)) := phi(['jackpotPrizes_4'])
REF_9751(uint256) (->jackpotPrizes_5) := 20000(uint256)
 jackpotPrizes[5] = 20_000
REF_9752(uint256) -> jackpotPrizes_5[5]
jackpotPrizes_6(mapping(uint8 => uint256)) := phi(['jackpotPrizes_5'])
REF_9752(uint256) (->jackpotPrizes_6) := 20000(uint256)
 jackpotPrizes[6] = 30_000
REF_9753(uint256) -> jackpotPrizes_6[6]
jackpotPrizes_7(mapping(uint8 => uint256)) := phi(['jackpotPrizes_6'])
REF_9753(uint256) (->jackpotPrizes_7) := 30000(uint256)
 jackpotPrizes[7] = 30_000
REF_9754(uint256) -> jackpotPrizes_7[7]
jackpotPrizes_8(mapping(uint8 => uint256)) := phi(['jackpotPrizes_7'])
REF_9754(uint256) (->jackpotPrizes_8) := 30000(uint256)
 jackpotPrizes[8] = 40_000
REF_9755(uint256) -> jackpotPrizes_8[8]
jackpotPrizes_9(mapping(uint8 => uint256)) := phi(['jackpotPrizes_8'])
REF_9755(uint256) (->jackpotPrizes_9) := 40000(uint256)
 jackpotPrizes[9] = 40_000
REF_9756(uint256) -> jackpotPrizes_9[9]
jackpotPrizes_10(mapping(uint8 => uint256)) := phi(['jackpotPrizes_9'])
REF_9756(uint256) (->jackpotPrizes_10) := 40000(uint256)
 jackpotPrizes[10] = 50_000
REF_9757(uint256) -> jackpotPrizes_10[10]
jackpotPrizes_11(mapping(uint8 => uint256)) := phi(['jackpotPrizes_10'])
REF_9757(uint256) (->jackpotPrizes_11) := 50000(uint256)
 jackpotPrizes[11] = 100_000
REF_9758(uint256) -> jackpotPrizes_11[11]
jackpotPrizes_12(mapping(uint8 => uint256)) := phi(['jackpotPrizes_11'])
REF_9758(uint256) (->jackpotPrizes_12) := 100000(uint256)
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
TMP_23699(Spin.RewardProbabilities) = new RewardProbabilities(200000,600000,900000)
rewardProbabilities_1(Spin.RewardProbabilities) := TMP_23699(Spin.RewardProbabilities)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### Spin.startSpin() [EXTERNAL]
```slithir
admin_2(address) := phi(['admin_0', 'admin_1', 'admin_5'])
supraRouter_2(ISupraRouterContract) := phi(['supraRouter_0', 'supraRouter_5', 'supraRouter_1'])
enableSpin_2(bool) := phi(['enableSpin_5', 'enableSpin_0', 'enableSpin_1', 'enableSpin_4'])
isSpinPending_1(mapping(address => bool)) := phi(['isSpinPending_4', 'isSpinPending_0', 'isSpinPending_5'])
spinPrice_2(uint256) := phi(['spinPrice_6', 'spinPrice_1', 'spinPrice_4', 'spinPrice_0'])
 ! enableSpin
TMP_23701 = UnaryType.BANG enableSpin_4 
CONDITION TMP_23701
 revert CampaignNotStarted()()
TMP_23702(None) = SOLIDITY_CALL revert CampaignNotStarted()()
 require(bool,string)(msg.value == spinPrice,Incorrect spin price sent)
TMP_23703(bool) = msg.value == spinPrice_4
TMP_23704(None) = SOLIDITY_CALL require(bool,string)(TMP_23703,Incorrect spin price sent)
 isSpinPending[msg.sender]
REF_9759(bool) -> isSpinPending_3[msg.sender]
CONDITION REF_9759
 revert SpinRequestPending(address)(msg.sender)
TMP_23705(None) = SOLIDITY_CALL revert SpinRequestPending(address)(msg.sender)
 isSpinPending[msg.sender] = true
REF_9760(bool) -> isSpinPending_3[msg.sender]
isSpinPending_4(mapping(address => bool)) := phi(['isSpinPending_3'])
REF_9760(bool) (->isSpinPending_4) := True(bool)
 callbackSignature = handleRandomness(uint256,uint256[])
callbackSignature_1(string) := handleRandomness(uint256,uint256[])(string)
 rngCount = 1
rngCount_1(uint8) := 1(uint256)
 numConfirmations = 1
numConfirmations_1(uint256) := 1(uint256)
 clientSeed = uint256(keccak256(bytes)(abi.encodePacked(admin,block.timestamp)))
TMP_23706(bytes) = SOLIDITY_CALL abi.encodePacked()(admin_4,block.timestamp)
TMP_23707(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23706)
TMP_23708 = CONVERT TMP_23707 to uint256
clientSeed_1(uint256) := TMP_23708(uint256)
 nonce = supraRouter.generateRequest(callbackSignature,rngCount,numConfirmations,clientSeed,admin)
TMP_23709(uint256) = HIGH_LEVEL_CALL, dest:supraRouter_4(ISupraRouterContract), function:generateRequest, arguments:['callbackSignature_1', 'rngCount_1', 'numConfirmations_1', 'clientSeed_1', 'admin_4']  
admin_5(address) := phi(['admin_4', 'admin_1', 'admin_5'])
supraRouter_5(ISupraRouterContract) := phi(['supraRouter_5', 'supraRouter_1', 'supraRouter_4'])
nonce_1(uint256) := TMP_23709(uint256)
 userNonce[nonce] = address(msg.sender)
REF_9763(address) -> userNonce_0[nonce_1]
TMP_23710 = CONVERT msg.sender to address
userNonce_1(mapping(uint256 => address)) := phi(['userNonce_0'])
REF_9763(address) (->userNonce_1) := TMP_23710(address)
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
TMP_23714(uint256) = block.timestamp (c)- campaignStartDate_1
TMP_23715(uint256) = TMP_23714 (c)/ 604800
RETURN TMP_23715
```
#### Spin.handleRandomness(uint256,uint256[]) [EXTERNAL]
```slithir
SUPRA_ROLE_10(bytes32) := phi(['SUPRA_ROLE_11', 'SUPRA_ROLE_9', 'SUPRA_ROLE_0'])
lastJackpotClaimWeek_2(uint256) := phi(['lastJackpotClaimWeek_0', 'lastJackpotClaimWeek_7', 'lastJackpotClaimWeek_6', 'lastJackpotClaimWeek_5', 'lastJackpotClaimWeek_1'])
userData_1(mapping(address => Spin.UserData)) := phi(['userData_0', 'userData_17', 'userData_14', 'userData_16', 'userData_21', 'userData_22', 'userData_20'])
userNonce_2(mapping(uint256 => address)) := phi(['userNonce_5', 'userNonce_1', 'userNonce_0'])
 user = userNonce[nonce]
REF_9764(address) -> userNonce_4[nonce_1]
user_1(address) := REF_9764(address)
 user == address(0)
TMP_23716 = CONVERT 0 to address
TMP_23717(bool) = user_1 == TMP_23716
CONDITION TMP_23717
 revert InvalidNonce()()
TMP_23718(None) = SOLIDITY_CALL revert InvalidNonce()()
 isSpinPending[user] = false
REF_9765(bool) -> isSpinPending_4[user_1]
isSpinPending_5(mapping(address => bool)) := phi(['isSpinPending_4'])
REF_9765(bool) (->isSpinPending_5) := False(bool)
 delete userNonce[nonce]
REF_9766(address) -> userNonce_4[nonce_1]
userNonce_5 = delete REF_9766 
 randomness = rngList[0]
REF_9767(uint256) -> rngList_1[0]
randomness_1(uint256) := REF_9767(uint256)
 (rewardCategory,rewardAmount) = determineReward(randomness,user)
TUPLE_148(string,uint256) = INTERNAL_CALL, Spin.determineReward(uint256,address)(randomness_1,user_1)
userData_4(mapping(address => Spin.UserData)) := phi(['userData_16'])
rewardCategory_1(string)= UNPACK TUPLE_148 index: 0 
rewardAmount_1(uint256)= UNPACK TUPLE_148 index: 1 
 userDataStorage = userData[user]
REF_9768(Spin.UserData) -> userData_4[user_1]
userDataStorage_1 (-> ['userData'])(Spin.UserData) := REF_9768(Spin.UserData)
 keccak256(bytes)(bytes(rewardCategory)) == keccak256(bytes)(Jackpot)
TMP_23719 = CONVERT rewardCategory_1 to bytes
TMP_23720(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23719)
TMP_23721(bytes32) = SOLIDITY_CALL keccak256(bytes)(Jackpot)
TMP_23722(bool) = TMP_23720 == TMP_23721
CONDITION TMP_23722
 currentWeek = getCurrentWeek()
TMP_23723(uint256) = INTERNAL_CALL, Spin.getCurrentWeek()()
currentWeek_1(uint256) := TMP_23723(uint256)
 currentWeek == lastJackpotClaimWeek
TMP_23724(bool) = currentWeek_1 == lastJackpotClaimWeek_6
CONDITION TMP_23724
 userDataStorage.nothingCounts += 1
REF_9769(uint256) -> userDataStorage_1 (-> ['userData']).nothingCounts
userDataStorage_2 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9769(-> userDataStorage_2 (-> ['userData'])) = REF_9769 (c)+ 1
userData_5(mapping(address => Spin.UserData)) := phi(["userDataStorage_2 (-> ['userData'])"])
 rewardCategory = Nothing
rewardCategory_2(string) := Nothing(string)
 rewardAmount = 0
rewardAmount_2(uint256) := 0(uint256)
 JackpotAlreadyClaimed(Jackpot already claimed this week)
Emit JackpotAlreadyClaimed(Jackpot already claimed this week)
 userDataStorage.streakCount < (currentWeek + 2)
REF_9770(uint256) -> userDataStorage_1 (-> ['userData']).streakCount
TMP_23726(uint256) = currentWeek_1 (c)+ 2
TMP_23727(bool) = REF_9770 < TMP_23726
CONDITION TMP_23727
 userDataStorage.nothingCounts += 1
REF_9771(uint256) -> userDataStorage_1 (-> ['userData']).nothingCounts
userDataStorage_3 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9771(-> userDataStorage_3 (-> ['userData'])) = REF_9771 (c)+ 1
userData_6(mapping(address => Spin.UserData)) := phi(["userDataStorage_3 (-> ['userData'])"])
 rewardCategory = Nothing
rewardCategory_3(string) := Nothing(string)
 rewardAmount = 0
rewardAmount_3(uint256) := 0(uint256)
 NotEnoughStreak(Not enough streak count to claim Jackpot)
Emit NotEnoughStreak(Not enough streak count to claim Jackpot)
 userDataStorage.jackpotWins ++
REF_9772(uint256) -> userDataStorage_1 (-> ['userData']).jackpotWins
TMP_23729(uint256) := REF_9772(uint256)
userDataStorage_4 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9772(-> userDataStorage_4 (-> ['userData'])) = REF_9772 (c)+ 1
userData_7(mapping(address => Spin.UserData)) := phi(["userDataStorage_4 (-> ['userData'])"])
 lastJackpotClaimWeek = currentWeek
lastJackpotClaimWeek_7(uint256) := currentWeek_1(uint256)
rewardCategory_4(string) := phi(['rewardCategory_3', 'rewardCategory_1'])
rewardAmount_4(uint256) := phi(['rewardAmount_3', 'rewardAmount_1'])
userDataStorage_5 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_3 (-> ['userData'])", "userDataStorage_4 (-> ['userData'])"])
rewardCategory_5(string) := phi(['rewardCategory_1', 'rewardCategory_2'])
rewardAmount_5(uint256) := phi(['rewardAmount_2', 'rewardAmount_1'])
userDataStorage_6 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_2 (-> ['userData'])", "userDataStorage_1 (-> ['userData'])"])
 keccak256(bytes)(bytes(rewardCategory)) == keccak256(bytes)(Raffle Ticket)
TMP_23730 = CONVERT rewardCategory_1 to bytes
TMP_23731(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23730)
TMP_23732(bytes32) = SOLIDITY_CALL keccak256(bytes)(Raffle Ticket)
TMP_23733(bool) = TMP_23731 == TMP_23732
CONDITION TMP_23733
 userDataStorage.raffleTicketsGained += rewardAmount
REF_9773(uint256) -> userDataStorage_1 (-> ['userData']).raffleTicketsGained
userDataStorage_12 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9773(-> userDataStorage_12 (-> ['userData'])) = REF_9773 (c)+ rewardAmount_1
userData_11(mapping(address => Spin.UserData)) := phi(["userDataStorage_12 (-> ['userData'])"])
 userDataStorage.raffleTicketsBalance += rewardAmount
REF_9774(uint256) -> userDataStorage_12 (-> ['userData']).raffleTicketsBalance
userDataStorage_13 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_12 (-> ['userData'])"])
REF_9774(-> userDataStorage_13 (-> ['userData'])) = REF_9774 (c)+ rewardAmount_1
userData_12(mapping(address => Spin.UserData)) := phi(["userDataStorage_13 (-> ['userData'])"])
 keccak256(bytes)(bytes(rewardCategory)) == keccak256(bytes)(PP)
TMP_23734 = CONVERT rewardCategory_1 to bytes
TMP_23735(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23734)
TMP_23736(bytes32) = SOLIDITY_CALL keccak256(bytes)(PP)
TMP_23737(bool) = TMP_23735 == TMP_23736
CONDITION TMP_23737
 userDataStorage.PPGained += rewardAmount
REF_9775(uint256) -> userDataStorage_1 (-> ['userData']).PPGained
userDataStorage_10 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9775(-> userDataStorage_10 (-> ['userData'])) = REF_9775 (c)+ rewardAmount_1
userData_10(mapping(address => Spin.UserData)) := phi(["userDataStorage_10 (-> ['userData'])"])
 keccak256(bytes)(bytes(rewardCategory)) == keccak256(bytes)(Plume Token)
TMP_23738 = CONVERT rewardCategory_1 to bytes
TMP_23739(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23738)
TMP_23740(bytes32) = SOLIDITY_CALL keccak256(bytes)(Plume Token)
TMP_23741(bool) = TMP_23739 == TMP_23740
CONDITION TMP_23741
 userDataStorage.plumeTokens += rewardAmount
REF_9776(uint256) -> userDataStorage_1 (-> ['userData']).plumeTokens
userDataStorage_8 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9776(-> userDataStorage_8 (-> ['userData'])) = REF_9776 (c)+ rewardAmount_1
userData_9(mapping(address => Spin.UserData)) := phi(["userDataStorage_8 (-> ['userData'])"])
 userDataStorage.nothingCounts += 1
REF_9777(uint256) -> userDataStorage_1 (-> ['userData']).nothingCounts
userDataStorage_7 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9777(-> userDataStorage_7 (-> ['userData'])) = REF_9777 (c)+ 1
userData_8(mapping(address => Spin.UserData)) := phi(["userDataStorage_7 (-> ['userData'])"])
userDataStorage_9 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_8 (-> ['userData'])", "userDataStorage_7 (-> ['userData'])"])
userDataStorage_11 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])", "userDataStorage_10 (-> ['userData'])"])
userDataStorage_14 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_13 (-> ['userData'])", "userDataStorage_1 (-> ['userData'])"])
 userDataStorage.streakCount = _computeStreak(user,block.timestamp,true)
REF_9778(uint256) -> userDataStorage_14 (-> ['userData']).streakCount
TMP_23742(uint256) = INTERNAL_CALL, Spin._computeStreak(address,uint256,bool)(user_1,block.timestamp,True)
userDataStorage_15 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_14 (-> ['userData'])"])
REF_9778(uint256) (->userDataStorage_15 (-> ['userData'])) := TMP_23742(uint256)
userData_13(mapping(address => Spin.UserData)) := phi(["userDataStorage_15 (-> ['userData'])"])
 userDataStorage.lastSpinTimestamp = block.timestamp
REF_9779(uint256) -> userDataStorage_15 (-> ['userData']).lastSpinTimestamp
userDataStorage_16 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_15 (-> ['userData'])"])
REF_9779(uint256) (->userDataStorage_16 (-> ['userData'])) := block.timestamp(uint256)
userData_14(mapping(address => Spin.UserData)) := phi(["userDataStorage_16 (-> ['userData'])"])
 keccak256(bytes)(bytes(rewardCategory)) == keccak256(bytes)(Jackpot) || keccak256(bytes)(bytes(rewardCategory)) == keccak256(bytes)(Plume Token)
TMP_23743 = CONVERT rewardCategory_1 to bytes
TMP_23744(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23743)
TMP_23745(bytes32) = SOLIDITY_CALL keccak256(bytes)(Jackpot)
TMP_23746(bool) = TMP_23744 == TMP_23745
TMP_23747 = CONVERT rewardCategory_1 to bytes
TMP_23748(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23747)
TMP_23749(bytes32) = SOLIDITY_CALL keccak256(bytes)(Plume Token)
TMP_23750(bool) = TMP_23748 == TMP_23749
TMP_23751(bool) = TMP_23746 || TMP_23750
CONDITION TMP_23751
 _safeTransferPlume(user,rewardAmount * 1000000000000000000)
TMP_23752(uint256) = rewardAmount_1 (c)* 1000000000000000000
INTERNAL_CALL, Spin._safeTransferPlume(address,uint256)(user_1,TMP_23752)
 SpinCompleted(user,rewardCategory,rewardAmount)
Emit SpinCompleted(user_1,rewardCategory_1,rewardAmount_1)
 onlyRole(SUPRA_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(SUPRA_ROLE_10)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### Spin.determineReward(uint256,address) [INTERNAL]
```slithir
randomness_1(uint256) := phi(['randomness_1'])
user_1(address) := phi(['user_1'])
userData_15(mapping(address => Spin.UserData)) := phi(['userData_0', 'userData_17', 'userData_14', 'userData_16', 'userData_21', 'userData_22', 'userData_20'])
jackpotProbabilities_2(uint256[7]) := phi(['jackpotProbabilities_4', 'jackpotProbabilities_0', 'jackpotProbabilities_3', 'jackpotProbabilities_1'])
baseRaffleMultiplier_2(uint256) := phi(['baseRaffleMultiplier_3', 'baseRaffleMultiplier_1', 'baseRaffleMultiplier_0', 'baseRaffleMultiplier_4'])
PP_PerSpin_2(uint256) := phi(['PP_PerSpin_3', 'PP_PerSpin_0', 'PP_PerSpin_1', 'PP_PerSpin_4'])
plumeAmounts_2(uint256[3]) := phi(['plumeAmounts_4', 'plumeAmounts_0', 'plumeAmounts_1', 'plumeAmounts_3'])
campaignStartDate_2(uint256) := phi(['campaignStartDate_7', 'campaignStartDate_0'])
jackpotPrizes_13(mapping(uint8 => uint256)) := phi(['jackpotPrizes_17', 'jackpotPrizes_0', 'jackpotPrizes_14', 'jackpotPrizes_12', 'jackpotPrizes_16', 'jackpotPrizes_15'])
rewardProbabilities_2(Spin.RewardProbabilities) := phi(['rewardProbabilities_1', 'rewardProbabilities_3', 'rewardProbabilities_8', 'rewardProbabilities_0'])
 probability = randomness % 1_000_000
TMP_23757(uint256) = randomness_1 % 1000000
probability_1(uint256) := TMP_23757(uint256)
 daysSinceStart = (block.timestamp - campaignStartDate) / 86400
TMP_23758(uint256) = block.timestamp (c)- campaignStartDate_2
TMP_23759(uint256) = TMP_23758 (c)/ 86400
daysSinceStart_1(uint256) := TMP_23759(uint256)
 weekNumber = uint8(getCurrentWeek())
TMP_23760(uint256) = INTERNAL_CALL, Spin.getCurrentWeek()()
TMP_23761 = CONVERT TMP_23760 to uint8
weekNumber_1(uint8) := TMP_23761(uint8)
 dayOfWeek = uint8(daysSinceStart % 7)
TMP_23762(uint256) = daysSinceStart_1 % 7
TMP_23763 = CONVERT TMP_23762 to uint8
dayOfWeek_1(uint8) := TMP_23763(uint8)
 jackpotThreshold = jackpotProbabilities[dayOfWeek]
REF_9780(uint256) -> jackpotProbabilities_3[dayOfWeek_1]
jackpotThreshold_1(uint256) := REF_9780(uint256)
 probability < jackpotThreshold
TMP_23764(bool) = probability_1 < jackpotThreshold_1
CONDITION TMP_23764
 (Jackpot,jackpotPrizes[weekNumber])
REF_9781(uint256) -> jackpotPrizes_14[weekNumber_1]
RETURN Jackpot,REF_9781
 probability <= rewardProbabilities.plumeTokenThreshold
REF_9782(uint256) -> rewardProbabilities_3.plumeTokenThreshold
TMP_23765(bool) = probability_1 <= REF_9782
CONDITION TMP_23765
 plumeAmount = plumeAmounts[probability % 3]
TMP_23766(uint256) = probability_1 % 3
REF_9783(uint256) -> plumeAmounts_3[TMP_23766]
plumeAmount_1(uint256) := REF_9783(uint256)
 (Plume Token,plumeAmount)
RETURN Plume Token,plumeAmount_1
 probability <= rewardProbabilities.raffleTicketThreshold
REF_9784(uint256) -> rewardProbabilities_3.raffleTicketThreshold
TMP_23767(bool) = probability_1 <= REF_9784
CONDITION TMP_23767
 (Raffle Ticket,baseRaffleMultiplier * (userData[user].streakCount + 1))
REF_9785(Spin.UserData) -> userData_16[user_1]
REF_9786(uint256) -> REF_9785.streakCount
TMP_23768(uint256) = REF_9786 (c)+ 1
TMP_23769(uint256) = baseRaffleMultiplier_3 (c)* TMP_23768
RETURN Raffle Ticket,TMP_23769
 probability <= rewardProbabilities.ppThreshold
REF_9787(uint256) -> rewardProbabilities_3.ppThreshold
TMP_23770(bool) = probability_1 <= REF_9787
CONDITION TMP_23770
 (PP,PP_PerSpin)
RETURN PP,PP_PerSpin_3
 (Nothing,0)
RETURN Nothing,0
```
#### Spin._computeStreak(address,uint256,bool) [INTERNAL]
```slithir
user_1(address) := phi(['user_1', 'user_1'])
nowTs_1(uint256) := phi(['block.timestamp'])
SECONDS_PER_DAY_1(uint256) := phi(['SECONDS_PER_DAY_3', 'SECONDS_PER_DAY_0'])
userData_17(mapping(address => Spin.UserData)) := phi(['userData_0', 'userData_17', 'userData_14', 'userData_16', 'userData_21', 'userData_22', 'userData_20'])
 lastSpinTs = userData[user].lastSpinTimestamp
REF_9788(Spin.UserData) -> userData_17[user_1]
REF_9789(uint256) -> REF_9788.lastSpinTimestamp
lastSpinTs_1(uint256) := REF_9789(uint256)
 lastSpinTs == 0
TMP_23771(bool) = lastSpinTs_1 == 0
CONDITION TMP_23771
 0 + streakAdjustment
TMP_23772(uint256) = 0 (c)+ streakAdjustment_3
RETURN TMP_23772
 lastDaySpun = lastSpinTs / SECONDS_PER_DAY
TMP_23773(uint256) = lastSpinTs_1 (c)/ SECONDS_PER_DAY_1
lastDaySpun_1(uint256) := TMP_23773(uint256)
 today = nowTs / SECONDS_PER_DAY
TMP_23774(uint256) = nowTs_1 (c)/ SECONDS_PER_DAY_1
today_1(uint256) := TMP_23774(uint256)
 today == lastDaySpun
TMP_23775(bool) = today_1 == lastDaySpun_1
CONDITION TMP_23775
 userData[user].streakCount
REF_9790(Spin.UserData) -> userData_17[user_1]
REF_9791(uint256) -> REF_9790.streakCount
RETURN REF_9791
 today == lastDaySpun + 1
TMP_23776(uint256) = lastDaySpun_1 (c)+ 1
TMP_23777(bool) = today_1 == TMP_23776
CONDITION TMP_23777
 userData[user].streakCount + streakAdjustment
REF_9792(Spin.UserData) -> userData_17[user_1]
REF_9793(uint256) -> REF_9792.streakCount
TMP_23778(uint256) = REF_9793 (c)+ streakAdjustment_3
RETURN TMP_23778
 0 + streakAdjustment
TMP_23779(uint256) = 0 (c)+ streakAdjustment_3
RETURN TMP_23779
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
TMP_23780(uint256) = INTERNAL_CALL, Spin._computeStreak(address,uint256,bool)(user_1,block.timestamp,False)
RETURN TMP_23780
```
#### Spin.spendRaffleTickets(address,uint256) [EXTERNAL]
```slithir
userData_18(mapping(address => Spin.UserData)) := phi(['userData_0', 'userData_17', 'userData_14', 'userData_16', 'userData_21', 'userData_22', 'userData_20'])
 userDataStorage = userData[user]
REF_9794(Spin.UserData) -> userData_19[user_1]
userDataStorage_1 (-> ['userData'])(Spin.UserData) := REF_9794(Spin.UserData)
 require(bool,string)(userDataStorage.raffleTicketsBalance >= amount,Insufficient raffle tickets)
REF_9795(uint256) -> userDataStorage_1 (-> ['userData']).raffleTicketsBalance
TMP_23781(bool) = REF_9795 >= amount_1
TMP_23782(None) = SOLIDITY_CALL require(bool,string)(TMP_23781,Insufficient raffle tickets)
 userDataStorage.raffleTicketsBalance -= amount
REF_9796(uint256) -> userDataStorage_1 (-> ['userData']).raffleTicketsBalance
userDataStorage_2 (-> ['userData'])(Spin.UserData) := phi(["userDataStorage_1 (-> ['userData'])"])
REF_9796(-> userDataStorage_2 (-> ['userData'])) = REF_9796 (c)- amount_1
userData_20(mapping(address => Spin.UserData)) := phi(["userDataStorage_2 (-> ['userData'])"])
 RaffleTicketsSpent(user,amount,userDataStorage.raffleTicketsBalance)
REF_9797(uint256) -> userDataStorage_2 (-> ['userData']).raffleTicketsBalance
Emit RaffleTicketsSpent(user_1,amount_1,REF_9797)
 onlyRaffleContract()
MODIFIER_CALL, Spin.onlyRaffleContract()()
```
#### Spin.adminWithdraw(address,uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_9(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
 require(bool,string)(recipient != address(0),Invalid recipient address)
TMP_23785 = CONVERT 0 to address
TMP_23786(bool) = recipient_1 != TMP_23785
TMP_23787(None) = SOLIDITY_CALL require(bool,string)(TMP_23786,Invalid recipient address)
 _safeTransferPlume(recipient,amount)
INTERNAL_CALL, Spin._safeTransferPlume(address,uint256)(recipient_1,amount_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_9)
```
#### Spin.pause() [EXTERNAL]
```slithir
ADMIN_ROLE_11(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
 _pause()
INTERNAL_CALL, PausableUpgradeable._pause()()
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_11)
```
#### Spin.unpause() [EXTERNAL]
```slithir
ADMIN_ROLE_13(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
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
TMP_23794(bool) = year1_1 == year2_1
TMP_23795(bool) = month1_1 == month2_1
TMP_23796(bool) = TMP_23794 && TMP_23795
TMP_23797(bool) = day1_1 == day2_1
TMP_23798(bool) = TMP_23796 && TMP_23797
RETURN TMP_23798
```
#### Spin.isNextDay(uint16,uint8,uint8,uint16,uint8,uint8) [INTERNAL]
```slithir
SECONDS_PER_DAY_2(uint256) := phi(['SECONDS_PER_DAY_3', 'SECONDS_PER_DAY_0'])
dateTime_2(IDateTime) := phi(['dateTime_6', 'dateTime_13', 'dateTime_1', 'dateTime_0'])
 lastDateTimestamp = dateTime.toTimestamp(lastYear,lastMonth,lastDay)
TMP_23799(uint256) = HIGH_LEVEL_CALL, dest:dateTime_2(IDateTime), function:toTimestamp, arguments:['lastYear_1', 'lastMonth_1', 'lastDay_1']  
SECONDS_PER_DAY_3(uint256) := phi(['SECONDS_PER_DAY_2', 'SECONDS_PER_DAY_3'])
dateTime_3(IDateTime) := phi(['dateTime_6', 'dateTime_13', 'dateTime_1', 'dateTime_2'])
lastDateTimestamp_1(uint256) := TMP_23799(uint256)
 nextDayTimestamp = lastDateTimestamp + SECONDS_PER_DAY
TMP_23800(uint256) = lastDateTimestamp_1 (c)+ SECONDS_PER_DAY_3
nextDayTimestamp_1(uint256) := TMP_23800(uint256)
 nextDayYear = dateTime.getYear(nextDayTimestamp)
TMP_23801(uint16) = HIGH_LEVEL_CALL, dest:dateTime_3(IDateTime), function:getYear, arguments:['nextDayTimestamp_1']  
dateTime_4(IDateTime) := phi(['dateTime_6', 'dateTime_13', 'dateTime_3', 'dateTime_1'])
nextDayYear_1(uint16) := TMP_23801(uint16)
 nextDayMonth = dateTime.getMonth(nextDayTimestamp)
TMP_23802(uint8) = HIGH_LEVEL_CALL, dest:dateTime_4(IDateTime), function:getMonth, arguments:['nextDayTimestamp_1']  
dateTime_5(IDateTime) := phi(['dateTime_6', 'dateTime_13', 'dateTime_1', 'dateTime_4'])
nextDayMonth_1(uint8) := TMP_23802(uint8)
 nextDayDay = dateTime.getDay(nextDayTimestamp)
TMP_23803(uint8) = HIGH_LEVEL_CALL, dest:dateTime_5(IDateTime), function:getDay, arguments:['nextDayTimestamp_1']  
dateTime_6(IDateTime) := phi(['dateTime_6', 'dateTime_13', 'dateTime_1', 'dateTime_5'])
nextDayDay_1(uint8) := TMP_23803(uint8)
 (nextDayYear == currentYear) && (nextDayMonth == currentMonth) && (nextDayDay == currentDay)
TMP_23804(bool) = nextDayYear_1 == currentYear_1
TMP_23805(bool) = nextDayMonth_1 == currentMonth_1
TMP_23806(bool) = TMP_23804 && TMP_23805
TMP_23807(bool) = nextDayDay_1 == currentDay_1
TMP_23808(bool) = TMP_23806 && TMP_23807
RETURN TMP_23808
```
#### Spin.getUserData(address) [EXTERNAL]
```slithir
userData_21(mapping(address => Spin.UserData)) := phi(['userData_0', 'userData_17', 'userData_14', 'userData_16', 'userData_21', 'userData_22', 'userData_20'])
 userDataStorage = userData[user]
REF_9802(Spin.UserData) -> userData_21[user_1]
userDataStorage_1 (-> ['userData'])(Spin.UserData) := REF_9802(Spin.UserData)
 (currentStreak(user),userDataStorage.lastSpinTimestamp,userDataStorage.jackpotWins,userDataStorage.raffleTicketsGained,userDataStorage.raffleTicketsBalance,userDataStorage.PPGained,userDataStorage.plumeTokens)
TMP_23809(uint256) = INTERNAL_CALL, Spin.currentStreak(address)(user_1)
REF_9803(uint256) -> userDataStorage_1 (-> ['userData']).lastSpinTimestamp
REF_9804(uint256) -> userDataStorage_1 (-> ['userData']).jackpotWins
REF_9805(uint256) -> userDataStorage_1 (-> ['userData']).raffleTicketsGained
REF_9806(uint256) -> userDataStorage_1 (-> ['userData']).raffleTicketsBalance
REF_9807(uint256) -> userDataStorage_1 (-> ['userData']).PPGained
REF_9808(uint256) -> userDataStorage_1 (-> ['userData']).plumeTokens
RETURN TMP_23809,REF_9803,REF_9804,REF_9805,REF_9806,REF_9807,REF_9808
 (dailyStreak,lastSpinTimestamp,jackpotWins,raffleTicketsGained,raffleTicketsBalance,ppGained,smallPlumeTokens)
```
#### Spin.getWeeklyJackpot() [EXTERNAL]
```slithir
campaignStartDate_3(uint256) := phi(['campaignStartDate_7', 'campaignStartDate_0'])
jackpotPrizes_15(mapping(uint8 => uint256)) := phi(['jackpotPrizes_17', 'jackpotPrizes_0', 'jackpotPrizes_14', 'jackpotPrizes_12', 'jackpotPrizes_16', 'jackpotPrizes_15'])
 require(bool,string)(campaignStartDate > 0,Campaign not started)
TMP_23810(bool) = campaignStartDate_3 > 0
TMP_23811(None) = SOLIDITY_CALL require(bool,string)(TMP_23810,Campaign not started)
 daysSinceStart = (block.timestamp - campaignStartDate) / 86400
TMP_23812(uint256) = block.timestamp (c)- campaignStartDate_3
TMP_23813(uint256) = TMP_23812 (c)/ 86400
daysSinceStart_1(uint256) := TMP_23813(uint256)
 weekNumber = daysSinceStart / 7
TMP_23814(uint256) = daysSinceStart_1 (c)/ 7
weekNumber_1(uint256) := TMP_23814(uint256)
 weekNumber > 11
TMP_23815(bool) = weekNumber_1 > 11
CONDITION TMP_23815
 (weekNumber,0,0)
RETURN weekNumber_1,0,0
 jackpotPrize = jackpotPrizes[uint8(weekNumber)]
TMP_23816 = CONVERT weekNumber_1 to uint8
REF_9809(uint256) -> jackpotPrizes_15[TMP_23816]
jackpotPrize_1(uint256) := REF_9809(uint256)
 requiredStreak = weekNumber + 2
TMP_23817(uint256) = weekNumber_1 (c)+ 2
requiredStreak_1(uint256) := TMP_23817(uint256)
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
TMP_23818 = CONVERT this to address
TMP_23819(uint256) = SOLIDITY_CALL balance(address)(TMP_23818)
RETURN TMP_23819
```
#### Spin.setJackpotProbabilities(uint8[7]) [EXTERNAL]
```slithir
ADMIN_ROLE_15(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
 jackpotProbabilities = _jackpotProbabilities
jackpotProbabilities_4(uint256[7]) := _jackpotProbabilities_1(uint8[7])
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_15)
```
#### Spin.setJackpotPrizes(uint8,uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_17(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
 jackpotPrizes[week] = prize
REF_9810(uint256) -> jackpotPrizes_15[week_1]
jackpotPrizes_16(mapping(uint8 => uint256)) := phi(['jackpotPrizes_15'])
REF_9810(uint256) (->jackpotPrizes_16) := prize_1(uint256)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_17)
```
#### Spin.setCampaignStartDate(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_19(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_19)
 start == 0
TMP_23823(bool) = start_1 == 0
CONDITION TMP_23823
 campaignStartDate = block.timestamp
campaignStartDate_5(uint256) := block.timestamp(uint256)
 campaignStartDate = start
campaignStartDate_6(uint256) := start_1(uint256)
campaignStartDate_7(uint256) := phi(['campaignStartDate_5', 'campaignStartDate_6'])
```
#### Spin.setBaseRaffleMultiplier(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_21(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
 baseRaffleMultiplier = _baseRaffleMultiplier
baseRaffleMultiplier_4(uint256) := _baseRaffleMultiplier_1(uint256)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_21)
```
#### Spin.setPP_PerSpin(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_23(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
 PP_PerSpin = _PP_PerSpin
PP_PerSpin_4(uint256) := _PP_PerSpin_1(uint256)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_23)
```
#### Spin.setPlumeAmounts(uint256[3]) [EXTERNAL]
```slithir
ADMIN_ROLE_25(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
 plumeAmounts = _plumeAmounts
plumeAmounts_4(uint256[3]) := _plumeAmounts_1(uint256[3])
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_25)
```
#### Spin.setRaffleContract(address) [EXTERNAL]
```slithir
ADMIN_ROLE_27(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
 raffleContract = _raffleContract
raffleContract_1(address) := _raffleContract_1(address)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_27)
```
#### Spin.whitelist(address) [EXTERNAL]
```slithir
ADMIN_ROLE_29(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
 whitelists[user] = true
REF_9811(bool) -> whitelists_0[user_1]
whitelists_1(mapping(address => bool)) := phi(['whitelists_0'])
REF_9811(bool) (->whitelists_1) := True(bool)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_29)
```
#### Spin.setEnableSpin(bool) [EXTERNAL]
```slithir
ADMIN_ROLE_31(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
 enableSpin = _enableSpin
enableSpin_5(bool) := _enableSpin_1(bool)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_31)
```
#### Spin.setRewardProbabilities(uint256,uint256,uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_33(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
rewardProbabilities_4(Spin.RewardProbabilities) := phi(['rewardProbabilities_1', 'rewardProbabilities_3', 'rewardProbabilities_8', 'rewardProbabilities_0'])
 require(bool,string)(_plumeTokenThreshold < _raffleTicketThreshold,Invalid thresholds order)
TMP_23830(bool) = _plumeTokenThreshold_1 < _raffleTicketThreshold_1
TMP_23831(None) = SOLIDITY_CALL require(bool,string)(TMP_23830,Invalid thresholds order)
 require(bool,string)(_raffleTicketThreshold < _ppThreshold,Invalid thresholds order)
TMP_23832(bool) = _raffleTicketThreshold_1 < _ppThreshold_1
TMP_23833(None) = SOLIDITY_CALL require(bool,string)(TMP_23832,Invalid thresholds order)
 require(bool,string)(_ppThreshold <= 1_000_000,Threshold exceeds maximum)
TMP_23834(bool) = _ppThreshold_1 <= 1000000
TMP_23835(None) = SOLIDITY_CALL require(bool,string)(TMP_23834,Threshold exceeds maximum)
 rewardProbabilities.plumeTokenThreshold = _plumeTokenThreshold
REF_9812(uint256) -> rewardProbabilities_5.plumeTokenThreshold
rewardProbabilities_6(Spin.RewardProbabilities) := phi(['rewardProbabilities_5'])
REF_9812(uint256) (->rewardProbabilities_6) := _plumeTokenThreshold_1(uint256)
 rewardProbabilities.raffleTicketThreshold = _raffleTicketThreshold
REF_9813(uint256) -> rewardProbabilities_6.raffleTicketThreshold
rewardProbabilities_7(Spin.RewardProbabilities) := phi(['rewardProbabilities_6'])
REF_9813(uint256) (->rewardProbabilities_7) := _raffleTicketThreshold_1(uint256)
 rewardProbabilities.ppThreshold = _ppThreshold
REF_9814(uint256) -> rewardProbabilities_7.ppThreshold
rewardProbabilities_8(Spin.RewardProbabilities) := phi(['rewardProbabilities_7'])
REF_9814(uint256) (->rewardProbabilities_8) := _ppThreshold_1(uint256)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_33)
```
#### Spin.getSpinPrice() [EXTERNAL]
```slithir
spinPrice_5(uint256) := phi(['spinPrice_6', 'spinPrice_1', 'spinPrice_4', 'spinPrice_0'])
 spinPrice
RETURN spinPrice_5
```
#### Spin.setSpinPrice(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_35(bytes32) := phi(['ADMIN_ROLE_24', 'ADMIN_ROLE_22', 'ADMIN_ROLE_38', 'ADMIN_ROLE_14', 'ADMIN_ROLE_10', 'ADMIN_ROLE_32', 'ADMIN_ROLE_36', 'ADMIN_ROLE_20', 'ADMIN_ROLE_28', 'ADMIN_ROLE_12', 'ADMIN_ROLE_16', 'ADMIN_ROLE_0', 'ADMIN_ROLE_30', 'ADMIN_ROLE_34', 'ADMIN_ROLE_18', 'ADMIN_ROLE_8', 'ADMIN_ROLE_26'])
 spinPrice = _newPrice
spinPrice_6(uint256) := _newPrice_1(uint256)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_35)
```
#### Spin._getWeeklyJackpotDetails(uint256) [INTERNAL]
```slithir
jackpotPrizes_17(mapping(uint8 => uint256)) := phi(['jackpotPrizes_17', 'jackpotPrizes_0', 'jackpotPrizes_14', 'jackpotPrizes_12', 'jackpotPrizes_16', 'jackpotPrizes_15'])
 _week >= 12
TMP_23838(bool) = _week_1 >= 12
CONDITION TMP_23838
 (0,0)
RETURN 0,0
 prize = jackpotPrizes[uint8(_week)]
TMP_23839 = CONVERT _week_1 to uint8
REF_9815(uint256) -> jackpotPrizes_17[TMP_23839]
prize_1(uint256) := REF_9815(uint256)
 requiredStreak = _week + 2
TMP_23840(uint256) = _week_1 (c)+ 2
requiredStreak_1(uint256) := TMP_23840(uint256)
 (prize,requiredStreak)
RETURN prize_1,requiredStreak_1
 (prize,requiredStreak)
```
#### Spin._safeTransferPlume(address,uint256) [INTERNAL]
```slithir
_to_1(address) := phi(['recipient_1', 'user_1'])
_amount_1(uint256) := phi(['amount_1', 'TMP_23752'])
 require(bool,string)(address(this).balance >= _amount,insufficient Plume in the Spin contract)
TMP_23841 = CONVERT this to address
TMP_23842(uint256) = SOLIDITY_CALL balance(address)(TMP_23841)
TMP_23843(bool) = TMP_23842 >= _amount_1
TMP_23844(None) = SOLIDITY_CALL require(bool,string)(TMP_23843,insufficient Plume in the Spin contract)
 (success,None) = _to.call{value: _amount}()
TUPLE_149(bool,bytes) = LOW_LEVEL_CALL, dest:_to_1, function:call, arguments:[''] value:_amount_1 
success_1(bool)= UNPACK TUPLE_149 index: 0 
 require(bool,string)(success,Plume transfer failed)
TMP_23845(None) = SOLIDITY_CALL require(bool,string)(success_1,Plume transfer failed)
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
TMP_23560(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_23560'])(Initializable.InitializableStorage) := TMP_23560(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_9696(bool) -> $_1 (-> ['TMP_23560'])._initializing
TMP_23561 = UnaryType.BANG REF_9696 
isTopLevelCall_1(bool) := TMP_23561(bool)
 initialized = $._initialized
REF_9697(uint64) -> $_1 (-> ['TMP_23560'])._initialized
initialized_1(uint64) := REF_9697(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_23562(bool) = initialized_1 == 0
TMP_23563(bool) = TMP_23562 && isTopLevelCall_1
initialSetup_1(bool) := TMP_23563(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_23564(bool) = initialized_1 == 1
TMP_23565 = CONVERT this to address
TMP_23566(bytes) = SOLIDITY_CALL code(address)(TMP_23565)
REF_9698 -> LENGTH TMP_23566
TMP_23567(bool) = REF_9698 == 0
TMP_23568(bool) = TMP_23564 && TMP_23567
construction_1(bool) := TMP_23568(bool)
 ! initialSetup && ! construction
TMP_23569 = UnaryType.BANG initialSetup_1 
TMP_23570 = UnaryType.BANG construction_1 
TMP_23571(bool) = TMP_23569 && TMP_23570
CONDITION TMP_23571
 revert InvalidInitialization()()
TMP_23572(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_9699(uint64) -> $_1 (-> ['TMP_23560'])._initialized
$_2 (-> ['TMP_23560'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_23560'])"])
REF_9699(uint64) (->$_2 (-> ['TMP_23560'])) := 1(uint256)
TMP_23560(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_23560'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_9700(bool) -> $_2 (-> ['TMP_23560'])._initializing
$_3 (-> ['TMP_23560'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_23560'])"])
REF_9700(bool) (->$_3 (-> ['TMP_23560'])) := True(bool)
TMP_23560(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_23560'])"])
$_4 (-> ['TMP_23560'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_23560'])", "$_2 (-> ['TMP_23560'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_9701(bool) -> $_4 (-> ['TMP_23560'])._initializing
$_5 (-> ['TMP_23560'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_23560'])"])
REF_9701(bool) (->$_5 (-> ['TMP_23560'])) := False(bool)
TMP_23560(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_23560'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_23574(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_23574'])(Initializable.InitializableStorage) := TMP_23574(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_9702(bool) -> $_1 (-> ['TMP_23574'])._initializing
REF_9703(uint64) -> $_1 (-> ['TMP_23574'])._initialized
TMP_23575(bool) = REF_9703 >= version_1
TMP_23576(bool) = REF_9702 || TMP_23575
CONDITION TMP_23576
 revert InvalidInitialization()()
TMP_23577(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_9704(uint64) -> $_1 (-> ['TMP_23574'])._initialized
$_2 (-> ['TMP_23574'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_23574'])"])
REF_9704(uint64) (->$_2 (-> ['TMP_23574'])) := version_1(uint64)
TMP_23574(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_23574'])"])
 $._initializing = true
REF_9705(bool) -> $_2 (-> ['TMP_23574'])._initializing
$_3 (-> ['TMP_23574'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_23574'])"])
REF_9705(bool) (->$_3 (-> ['TMP_23574'])) := True(bool)
TMP_23574(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_23574'])"])
 $._initializing = false
REF_9706(bool) -> $_3 (-> ['TMP_23574'])._initializing
$_4 (-> ['TMP_23574'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_23574'])"])
REF_9706(bool) (->$_4 (-> ['TMP_23574'])) := False(bool)
TMP_23574(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_23574'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
role_1(bytes32) := phi(['ADMIN_ROLE_11', 'TMP_23420', 'ADMIN_ROLE_17', 'ADMIN_ROLE_13', 'ADMIN_ROLE_7', 'ADMIN_ROLE_15', 'ADMIN_ROLE_9', 'TMP_23425', 'TMP_23422', 'ADMIN_ROLE_21', 'ADMIN_ROLE_19', 'SUPRA_ROLE_8', 'TMP_23427'])
 _checkRole(role)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(role_1)
prizeId_1(uint256) := phi(['prizeId_1', 'prizeId_1', 'prizeId_1', 'prizeId_1', 'prizeId_1'])
prizes_36(mapping(uint256 => Raffle.Prize)) := phi(['prizes_23', 'prizes_13', 'prizes_3', 'prizes_35', 'prizes_0', 'prizes_20', 'prizes_26', 'prizes_28', 'prizes_24', 'prizes_9', 'prizes_36', 'prizes_27', 'prizes_32', 'prizes_16'])
 require(bool,string)(prizes[prizeId].isActive,Prize not available)
REF_9707(Raffle.Prize) -> prizes_36[prizeId_1]
REF_9708(bool) -> REF_9707.isActive
TMP_23581(None) = SOLIDITY_CALL require(bool,string)(REF_9708,Prize not available)
```
#### ISupraRouterContract.generateRequest(string,uint8,uint256,address) [EXTERNAL]
```slithir

```
#### IDateTime.toTimestamp(uint16,uint8,uint8,uint8,uint8,uint8) [EXTERNAL]
```slithir

```
#### IDateTime.getMonth(uint256) [EXTERNAL]
```slithir

```
#### IDateTime.getYear(uint256) [EXTERNAL]
```slithir

```
#### IDateTime.getDay(uint256) [EXTERNAL]
```slithir

```
