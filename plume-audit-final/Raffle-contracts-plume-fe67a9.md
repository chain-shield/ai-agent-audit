
### Storage layout (Raffle) 

```text
admin address
spinContract ISpin
supraRouter ISupraRouterContract
prizes mapping(uint256 => Raffle.Prize)
prizeIds uint256[]
prizeRanges mapping(uint256 => Raffle.Range[])
totalTickets mapping(uint256 => uint256)
userHasEnteredPrize mapping(uint256 => mapping(address => bool))
totalUniqueUsers mapping(uint256 => uint256)
winnings mapping(address => uint256[])
pendingVRFRequests mapping(uint256 => uint256)
isWinnerRequestPending mapping(uint256 => bool)
prizeWinners mapping(uint256 => Raffle.Winner[])
winnersDrawn mapping(uint256 => uint256)
userWinCount mapping(uint256 => mapping(address => uint256))
_migrationComplete bool
__gap uint256[50]
nextPrizeId uint256

```
#### Raffle._authorizeUpgrade(address) [INTERNAL]
```slithir
newImplementation_1(address) := phi(['newImplementation_1'])
ADMIN_ROLE_23(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_12', 'ADMIN_ROLE_10', 'ADMIN_ROLE_22', 'ADMIN_ROLE_18', 'ADMIN_ROLE_24', 'ADMIN_ROLE_20', 'ADMIN_ROLE_6'])
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_23)
```
#### Raffle.initialize(address,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_5', 'DEFAULT_ADMIN_ROLE_0'])
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_12', 'ADMIN_ROLE_10', 'ADMIN_ROLE_22', 'ADMIN_ROLE_18', 'ADMIN_ROLE_24', 'ADMIN_ROLE_20', 'ADMIN_ROLE_6'])
SUPRA_ROLE_1(bytes32) := phi(['SUPRA_ROLE_9', 'SUPRA_ROLE_0', 'SUPRA_ROLE_7'])
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 __UUPSUpgradeable_init()
INTERNAL_CALL, UUPSUpgradeable.__UUPSUpgradeable_init()()
 spinContract = ISpin(_spinContract)
TMP_23675 = CONVERT _spinContract_1 to ISpin
spinContract_1(ISpin) := TMP_23675(ISpin)
 supraRouter = ISupraRouterContract(_supraRouter)
TMP_23676 = CONVERT _supraRouter_1 to ISupraRouterContract
supraRouter_1(ISupraRouterContract) := TMP_23676(ISupraRouterContract)
 admin = msg.sender
admin_1(address) := msg.sender(address)
 nextPrizeId = 1
nextPrizeId_1(uint256) := 1(uint256)
 _grantRole(DEFAULT_ADMIN_ROLE,msg.sender)
TMP_23677(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_4,msg.sender)
 _grantRole(ADMIN_ROLE,msg.sender)
TMP_23678(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(ADMIN_ROLE_5,msg.sender)
 _grantRole(SUPRA_ROLE,_supraRouter)
TMP_23679(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(SUPRA_ROLE_6,_supraRouter_1)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### Raffle.addPrize(string,string,uint256,uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_7(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_12', 'ADMIN_ROLE_10', 'ADMIN_ROLE_22', 'ADMIN_ROLE_18', 'ADMIN_ROLE_24', 'ADMIN_ROLE_20', 'ADMIN_ROLE_6'])
prizes_1(mapping(uint256 => Raffle.Prize)) := phi(['prizes_26', 'prizes_14', 'prizes_16', 'prizes_19', 'prizes_10', 'prizes_0', 'prizes_29', 'prizes_20', 'prizes_30', 'prizes_3', 'prizes_18', 'prizes_21', 'prizes_22'])
prizeIds_1(uint256[]) := phi(['prizeIds_7', 'prizeIds_4', 'prizeIds_0', 'prizeIds_12'])
nextPrizeId_2(uint256) := phi(['nextPrizeId_1', 'nextPrizeId_0', 'nextPrizeId_4'])
 prizeId = nextPrizeId ++
TMP_23681(uint256) := nextPrizeId_3(uint256)
nextPrizeId_4(uint256) = nextPrizeId_3 (c)+ 1
prizeId_1(uint256) := TMP_23681(uint256)
 prizeIds.push(prizeId)
REF_9730 -> LENGTH prizeIds_2
TMP_23683(uint256) := REF_9730(uint256)
TMP_23684(uint256) = TMP_23683 (c)+ 1
prizeIds_3(uint256[]) := phi(['prizeIds_2'])
REF_9730(uint256) (->prizeIds_3) := TMP_23684(uint256)
REF_9731(uint256) -> prizeIds_3[TMP_23683]
prizeIds_4(uint256[]) := phi(['prizeIds_3'])
REF_9731(uint256) (->prizeIds_4) := prizeId_1(uint256)
 require(bool,string)(bytes(prizes[prizeId].name).length == 0,Prize ID already in use)
REF_9732(Raffle.Prize) -> prizes_2[prizeId_1]
REF_9733(string) -> REF_9732.name
TMP_23685 = CONVERT REF_9733 to bytes
REF_9734 -> LENGTH TMP_23685
TMP_23686(bool) = REF_9734 == 0
TMP_23687(None) = SOLIDITY_CALL require(bool,string)(TMP_23686,Prize ID already in use)
 require(bool,string)(quantity > 0,Quantity must be greater than 0)
TMP_23688(bool) = quantity_1 > 0
TMP_23689(None) = SOLIDITY_CALL require(bool,string)(TMP_23688,Quantity must be greater than 0)
 prizes[prizeId] = Prize({name:name,description:description,value:value,endTimestamp:0,isActive:true,winner:address(0),winnerIndex:0,claimed:false,quantity:quantity})
REF_9735(Raffle.Prize) -> prizes_2[prizeId_1]
TMP_23690 = CONVERT 0 to address
TMP_23691(Raffle.Prize) = new Prize(name_1,description_1,value_1,0,True,TMP_23690,0,False,quantity_1)
prizes_3(mapping(uint256 => Raffle.Prize)) := phi(['prizes_2'])
REF_9735(Raffle.Prize) (->prizes_3) := TMP_23691(Raffle.Prize)
 PrizeAdded(prizeId,name)
Emit PrizeAdded(prizeId_1,name_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_7)
```
#### Raffle.editPrize(uint256,string,string,uint256,uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_9(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_12', 'ADMIN_ROLE_10', 'ADMIN_ROLE_22', 'ADMIN_ROLE_18', 'ADMIN_ROLE_24', 'ADMIN_ROLE_20', 'ADMIN_ROLE_6'])
prizes_4(mapping(uint256 => Raffle.Prize)) := phi(['prizes_26', 'prizes_14', 'prizes_16', 'prizes_19', 'prizes_10', 'prizes_0', 'prizes_29', 'prizes_20', 'prizes_30', 'prizes_3', 'prizes_18', 'prizes_21', 'prizes_22'])
 prize = prizes[prizeId]
REF_9736(Raffle.Prize) -> prizes_6[prizeId_1]
prize_1 (-> ['prizes'])(Raffle.Prize) := REF_9736(Raffle.Prize)
 prize.name = name
REF_9737(string) -> prize_1 (-> ['prizes']).name
prize_2 (-> ['prizes'])(Raffle.Prize) := phi(["prize_1 (-> ['prizes'])"])
REF_9737(string) (->prize_2 (-> ['prizes'])) := name_1(string)
prizes_7(mapping(uint256 => Raffle.Prize)) := phi(["prize_2 (-> ['prizes'])"])
 prize.description = description
REF_9738(string) -> prize_2 (-> ['prizes']).description
prize_3 (-> ['prizes'])(Raffle.Prize) := phi(["prize_2 (-> ['prizes'])"])
REF_9738(string) (->prize_3 (-> ['prizes'])) := description_1(string)
prizes_8(mapping(uint256 => Raffle.Prize)) := phi(["prize_3 (-> ['prizes'])"])
 prize.value = value
REF_9739(uint256) -> prize_3 (-> ['prizes']).value
prize_4 (-> ['prizes'])(Raffle.Prize) := phi(["prize_3 (-> ['prizes'])"])
REF_9739(uint256) (->prize_4 (-> ['prizes'])) := value_1(uint256)
prizes_9(mapping(uint256 => Raffle.Prize)) := phi(["prize_4 (-> ['prizes'])"])
 prize.quantity = quantity
REF_9740(uint256) -> prize_4 (-> ['prizes']).quantity
prize_5 (-> ['prizes'])(Raffle.Prize) := phi(["prize_4 (-> ['prizes'])"])
REF_9740(uint256) (->prize_5 (-> ['prizes'])) := quantity_1(uint256)
prizes_10(mapping(uint256 => Raffle.Prize)) := phi(["prize_5 (-> ['prizes'])"])
 PrizeEdited(prizeId,name,description,value,quantity)
Emit PrizeEdited(prizeId_1,name_1,description_1,value_1,quantity_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_9)
 prizeIsActive(prizeId)
MODIFIER_CALL, Raffle.prizeIsActive(uint256)(prizeId_1)
prizes_6(mapping(uint256 => Raffle.Prize)) := phi(['prizes_30'])
```
#### Raffle.removePrize(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_11(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_12', 'ADMIN_ROLE_10', 'ADMIN_ROLE_22', 'ADMIN_ROLE_18', 'ADMIN_ROLE_24', 'ADMIN_ROLE_20', 'ADMIN_ROLE_6'])
prizes_11(mapping(uint256 => Raffle.Prize)) := phi(['prizes_26', 'prizes_14', 'prizes_16', 'prizes_19', 'prizes_10', 'prizes_0', 'prizes_29', 'prizes_20', 'prizes_30', 'prizes_3', 'prizes_18', 'prizes_21', 'prizes_22'])
prizeIds_5(uint256[]) := phi(['prizeIds_7', 'prizeIds_4', 'prizeIds_0', 'prizeIds_12'])
 prizes[prizeId].isActive = false
REF_9741(Raffle.Prize) -> prizes_13[prizeId_1]
REF_9742(bool) -> REF_9741.isActive
prizes_14(mapping(uint256 => Raffle.Prize)) := phi(['prizes_13'])
REF_9742(bool) (->prizes_14) := False(bool)
 len = prizeIds.length
REF_9743 -> LENGTH prizeIds_7
len_1(uint256) := REF_9743(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < len
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_23697(bool) = i_2 < len_1
CONDITION TMP_23697
 prizeIds[i] == prizeId
REF_9744(uint256) -> prizeIds_7[i_2]
TMP_23698(bool) = REF_9744 == prizeId_1
CONDITION TMP_23698
 prizeIds[i] = prizeIds[len - 1]
REF_9745(uint256) -> prizeIds_7[i_2]
TMP_23699(uint256) = len_1 (c)- 1
REF_9746(uint256) -> prizeIds_7[TMP_23699]
prizeIds_8(uint256[]) := phi(['prizeIds_7'])
REF_9745(uint256) (->prizeIds_8) := REF_9746(uint256)
 prizeIds.pop()
REF_9748 -> LENGTH prizeIds_8
TMP_23701(uint256) = REF_9748 (c)- 1
REF_9749(uint256) -> prizeIds_8[TMP_23701]
prizeIds_9 = delete REF_9749 
REF_9750 -> LENGTH prizeIds_9
prizeIds_10(uint256[]) := phi(['prizeIds_9'])
REF_9750(uint256) (->prizeIds_10) := TMP_23701(uint256)
 i ++
TMP_23702(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 PrizeRemoved(prizeId)
Emit PrizeRemoved(prizeId_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_11)
 prizeIsActive(prizeId)
MODIFIER_CALL, Raffle.prizeIsActive(uint256)(prizeId_1)
prizes_13(mapping(uint256 => Raffle.Prize)) := phi(['prizes_30'])
```
#### Raffle.spendRaffle(uint256,uint256) [EXTERNAL]
```slithir
spinContract_2(ISpin) := phi(['spinContract_0', 'spinContract_5', 'spinContract_1'])
prizeRanges_1(mapping(uint256 => Raffle.Range[])) := phi(['prizeRanges_6', 'prizeRanges_10', 'prizeRanges_0', 'prizeRanges_8'])
totalTickets_1(mapping(uint256 => uint256)) := phi(['totalTickets_7', 'totalTickets_5', 'totalTickets_0', 'totalTickets_9', 'totalTickets_8'])
userHasEnteredPrize_1(mapping(uint256 => mapping(address => bool))) := phi(['userHasEnteredPrize_0', 'userHasEnteredPrize_4'])
totalUniqueUsers_1(mapping(uint256 => uint256)) := phi(['totalUniqueUsers_6', 'totalUniqueUsers_7', 'totalUniqueUsers_4', 'totalUniqueUsers_0'])
 require(bool,string)(ticketAmount > 0,Must spend at least 1 ticket)
TMP_23706(bool) = ticketAmount_1 > 0
TMP_23707(None) = SOLIDITY_CALL require(bool,string)(TMP_23706,Must spend at least 1 ticket)
 (None,None,None,None,userRaffleTickets,None,None) = spinContract.getUserData(msg.sender)
TUPLE_151(uint256,uint256,uint256,uint256,uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:spinContract_3(ISpin), function:getUserData, arguments:['msg.sender']  
spinContract_4(ISpin) := phi(['spinContract_5', 'spinContract_3', 'spinContract_1'])
prizeRanges_3(mapping(uint256 => Raffle.Range[])) := phi(['prizeRanges_2', 'prizeRanges_8', 'prizeRanges_10', 'prizeRanges_6'])
totalTickets_3(mapping(uint256 => uint256)) := phi(['totalTickets_7', 'totalTickets_2', 'totalTickets_5', 'totalTickets_9', 'totalTickets_8'])
userHasEnteredPrize_3(mapping(uint256 => mapping(address => bool))) := phi(['userHasEnteredPrize_2', 'userHasEnteredPrize_4'])
totalUniqueUsers_3(mapping(uint256 => uint256)) := phi(['totalUniqueUsers_7', 'totalUniqueUsers_2', 'totalUniqueUsers_4', 'totalUniqueUsers_6'])
userRaffleTickets_1(uint256)= UNPACK TUPLE_151 index: 4 
 userRaffleTickets < ticketAmount
TMP_23708(bool) = userRaffleTickets_1 < ticketAmount_1
CONDITION TMP_23708
 revert InsufficientTickets()()
TMP_23709(None) = SOLIDITY_CALL revert InsufficientTickets()()
 spinContract.spendRaffleTickets(msg.sender,ticketAmount)
HIGH_LEVEL_CALL, dest:spinContract_4(ISpin), function:spendRaffleTickets, arguments:['msg.sender', 'ticketAmount_1']  
spinContract_5(ISpin) := phi(['spinContract_4', 'spinContract_5', 'spinContract_1'])
prizeRanges_4(mapping(uint256 => Raffle.Range[])) := phi(['prizeRanges_3', 'prizeRanges_8', 'prizeRanges_10', 'prizeRanges_6'])
totalTickets_4(mapping(uint256 => uint256)) := phi(['totalTickets_7', 'totalTickets_3', 'totalTickets_5', 'totalTickets_9', 'totalTickets_8'])
userHasEnteredPrize_4(mapping(uint256 => mapping(address => bool))) := phi(['userHasEnteredPrize_3', 'userHasEnteredPrize_4'])
totalUniqueUsers_4(mapping(uint256 => uint256)) := phi(['totalUniqueUsers_7', 'totalUniqueUsers_3', 'totalUniqueUsers_4', 'totalUniqueUsers_6'])
 newTotal = totalTickets[prizeId] + ticketAmount
REF_9753(uint256) -> totalTickets_4[prizeId_1]
TMP_23711(uint256) = REF_9753 (c)+ ticketAmount_1
newTotal_1(uint256) := TMP_23711(uint256)
 prizeRanges[prizeId].push(Range({user:msg.sender,cumulativeEnd:newTotal}))
REF_9754(Raffle.Range[]) -> prizeRanges_4[prizeId_1]
TMP_23712(Raffle.Range) = new Range(msg.sender,newTotal_1)
REF_9756 -> LENGTH REF_9754
TMP_23714(uint256) := REF_9756(uint256)
TMP_23715(uint256) = TMP_23714 (c)+ 1
prizeRanges_5(mapping(uint256 => Raffle.Range[])) := phi(['prizeRanges_4'])
REF_9756(uint256) (->prizeRanges_6) := TMP_23715(uint256)
REF_9757(Raffle.Range) -> REF_9754[TMP_23714]
prizeRanges_6(mapping(uint256 => Raffle.Range[])) := phi(['prizeRanges_5'])
REF_9757(Raffle.Range) (->prizeRanges_6) := TMP_23712(Raffle.Range)
 totalTickets[prizeId] = newTotal
REF_9758(uint256) -> totalTickets_4[prizeId_1]
totalTickets_5(mapping(uint256 => uint256)) := phi(['totalTickets_4'])
REF_9758(uint256) (->totalTickets_5) := newTotal_1(uint256)
 ! userHasEnteredPrize[prizeId][msg.sender]
REF_9759(mapping(address => bool)) -> userHasEnteredPrize_4[prizeId_1]
REF_9760(bool) -> REF_9759[msg.sender]
TMP_23716 = UnaryType.BANG REF_9760 
CONDITION TMP_23716
 userHasEnteredPrize[prizeId][msg.sender] = true
REF_9761(mapping(address => bool)) -> userHasEnteredPrize_4[prizeId_1]
REF_9762(bool) -> REF_9761[msg.sender]
userHasEnteredPrize_5(mapping(uint256 => mapping(address => bool))) := phi(['userHasEnteredPrize_4'])
REF_9762(bool) (->userHasEnteredPrize_5) := True(bool)
 totalUniqueUsers[prizeId] ++
REF_9763(uint256) -> totalUniqueUsers_4[prizeId_1]
TMP_23717(uint256) := REF_9763(uint256)
totalUniqueUsers_5(mapping(uint256 => uint256)) := phi(['totalUniqueUsers_4'])
REF_9763(-> totalUniqueUsers_5) = REF_9763 (c)+ 1
 TicketSpent(msg.sender,prizeId,ticketAmount)
Emit TicketSpent(msg.sender,prizeId_1,ticketAmount_1)
 prizeIsActive(prizeId)
MODIFIER_CALL, Raffle.prizeIsActive(uint256)(prizeId_1)
```
#### Raffle.requestWinner(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_13(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_12', 'ADMIN_ROLE_10', 'ADMIN_ROLE_22', 'ADMIN_ROLE_18', 'ADMIN_ROLE_24', 'ADMIN_ROLE_20', 'ADMIN_ROLE_6'])
supraRouter_2(ISupraRouterContract) := phi(['supraRouter_4', 'supraRouter_0', 'supraRouter_1'])
prizes_15(mapping(uint256 => Raffle.Prize)) := phi(['prizes_26', 'prizes_14', 'prizes_16', 'prizes_19', 'prizes_10', 'prizes_0', 'prizes_29', 'prizes_20', 'prizes_30', 'prizes_3', 'prizes_18', 'prizes_21', 'prizes_22'])
prizeRanges_7(mapping(uint256 => Raffle.Range[])) := phi(['prizeRanges_6', 'prizeRanges_10', 'prizeRanges_0', 'prizeRanges_8'])
isWinnerRequestPending_1(mapping(uint256 => bool)) := phi(['isWinnerRequestPending_3', 'isWinnerRequestPending_4', 'isWinnerRequestPending_7', 'isWinnerRequestPending_0'])
winnersDrawn_1(mapping(uint256 => uint256)) := phi(['winnersDrawn_10', 'winnersDrawn_5', 'winnersDrawn_6', 'winnersDrawn_0', 'winnersDrawn_8', 'winnersDrawn_7', 'winnersDrawn_2'])
 winnersDrawn[prizeId] >= prizes[prizeId].quantity
REF_9764(uint256) -> winnersDrawn_2[prizeId_1]
REF_9765(Raffle.Prize) -> prizes_16[prizeId_1]
REF_9766(uint256) -> REF_9765.quantity
TMP_23720(bool) = REF_9764 >= REF_9766
CONDITION TMP_23720
 revert AllWinnersDrawn()()
TMP_23721(None) = SOLIDITY_CALL revert AllWinnersDrawn()()
 prizeRanges[prizeId].length == 0
REF_9767(Raffle.Range[]) -> prizeRanges_8[prizeId_1]
REF_9768 -> LENGTH REF_9767
TMP_23722(bool) = REF_9768 == 0
CONDITION TMP_23722
 revert EmptyTicketPool()()
TMP_23723(None) = SOLIDITY_CALL revert EmptyTicketPool()()
 require(bool,string)(prizes[prizeId].isActive,Prize not available)
REF_9769(Raffle.Prize) -> prizes_16[prizeId_1]
REF_9770(bool) -> REF_9769.isActive
TMP_23724(None) = SOLIDITY_CALL require(bool,string)(REF_9770,Prize not available)
 isWinnerRequestPending[prizeId]
REF_9771(bool) -> isWinnerRequestPending_2[prizeId_1]
CONDITION REF_9771
 revert WinnerRequestPending(uint256)(prizeId)
TMP_23725(None) = SOLIDITY_CALL revert WinnerRequestPending(uint256)(prizeId_1)
 isWinnerRequestPending[prizeId] = true
REF_9772(bool) -> isWinnerRequestPending_2[prizeId_1]
isWinnerRequestPending_3(mapping(uint256 => bool)) := phi(['isWinnerRequestPending_2'])
REF_9772(bool) (->isWinnerRequestPending_3) := True(bool)
 callbackSig = handleWinnerSelection(uint256,uint256[])
callbackSig_1(string) := handleWinnerSelection(uint256,uint256[])(string)
 requestId = supraRouter.generateRequest(callbackSig,1,1,uint256(keccak256(bytes)(abi.encodePacked(prizeId,block.timestamp))),msg.sender)
TMP_23726(bytes) = SOLIDITY_CALL abi.encodePacked()(prizeId_1,block.timestamp)
TMP_23727(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23726)
TMP_23728 = CONVERT TMP_23727 to uint256
TMP_23729(uint256) = HIGH_LEVEL_CALL, dest:supraRouter_3(ISupraRouterContract), function:generateRequest, arguments:['callbackSig_1', '1', '1', 'TMP_23728', 'msg.sender']  
supraRouter_4(ISupraRouterContract) := phi(['supraRouter_3', 'supraRouter_4', 'supraRouter_1'])
requestId_1(uint256) := TMP_23729(uint256)
 pendingVRFRequests[requestId] = prizeId
REF_9775(uint256) -> pendingVRFRequests_0[requestId_1]
pendingVRFRequests_1(mapping(uint256 => uint256)) := phi(['pendingVRFRequests_0'])
REF_9775(uint256) (->pendingVRFRequests_1) := prizeId_1(uint256)
 WinnerRequested(prizeId,requestId)
Emit WinnerRequested(prizeId_1,requestId_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_13)
```
#### Raffle.handleWinnerSelection(uint256,uint256[]) [EXTERNAL]
```slithir
SUPRA_ROLE_8(bytes32) := phi(['SUPRA_ROLE_9', 'SUPRA_ROLE_0', 'SUPRA_ROLE_7'])
prizes_17(mapping(uint256 => Raffle.Prize)) := phi(['prizes_26', 'prizes_14', 'prizes_16', 'prizes_19', 'prizes_10', 'prizes_0', 'prizes_29', 'prizes_20', 'prizes_30', 'prizes_3', 'prizes_18', 'prizes_21', 'prizes_22'])
prizeRanges_9(mapping(uint256 => Raffle.Range[])) := phi(['prizeRanges_6', 'prizeRanges_10', 'prizeRanges_0', 'prizeRanges_8'])
totalTickets_6(mapping(uint256 => uint256)) := phi(['totalTickets_7', 'totalTickets_5', 'totalTickets_0', 'totalTickets_9', 'totalTickets_8'])
pendingVRFRequests_2(mapping(uint256 => uint256)) := phi(['pendingVRFRequests_4', 'pendingVRFRequests_0', 'pendingVRFRequests_1'])
prizeWinners_1(mapping(uint256 => Raffle.Winner[])) := phi(['prizeWinners_4', 'prizeWinners_7', 'prizeWinners_0', 'prizeWinners_5', 'prizeWinners_8'])
winnersDrawn_3(mapping(uint256 => uint256)) := phi(['winnersDrawn_10', 'winnersDrawn_5', 'winnersDrawn_6', 'winnersDrawn_0', 'winnersDrawn_8', 'winnersDrawn_7', 'winnersDrawn_2'])
userWinCount_1(mapping(uint256 => mapping(address => uint256))) := phi(['userWinCount_0', 'userWinCount_3'])
 prizeId = pendingVRFRequests[requestId]
REF_9776(uint256) -> pendingVRFRequests_3[requestId_1]
prizeId_1(uint256) := REF_9776(uint256)
 isWinnerRequestPending[prizeId] = false
REF_9777(bool) -> isWinnerRequestPending_3[prizeId_1]
isWinnerRequestPending_4(mapping(uint256 => bool)) := phi(['isWinnerRequestPending_3'])
REF_9777(bool) (->isWinnerRequestPending_4) := False(bool)
 delete pendingVRFRequests[requestId]
REF_9778(uint256) -> pendingVRFRequests_3[requestId_1]
pendingVRFRequests_4 = delete REF_9778 
 ! prizes[prizeId].isActive
REF_9779(Raffle.Prize) -> prizes_18[prizeId_1]
REF_9780(bool) -> REF_9779.isActive
TMP_23732 = UnaryType.BANG REF_9780 
CONDITION TMP_23732
 revert PrizeInactive()()
TMP_23733(None) = SOLIDITY_CALL revert PrizeInactive()()
 winnersDrawn[prizeId] >= prizes[prizeId].quantity
REF_9781(uint256) -> winnersDrawn_4[prizeId_1]
REF_9782(Raffle.Prize) -> prizes_18[prizeId_1]
REF_9783(uint256) -> REF_9782.quantity
TMP_23734(bool) = REF_9781 >= REF_9783
CONDITION TMP_23734
 revert NoMoreWinners()()
TMP_23735(None) = SOLIDITY_CALL revert NoMoreWinners()()
 winningTicketIndex = (rng[0] % totalTickets[prizeId]) + 1
REF_9784(uint256) -> rng_1[0]
REF_9785(uint256) -> totalTickets_7[prizeId_1]
TMP_23736(uint256) = REF_9784 % REF_9785
TMP_23737(uint256) = TMP_23736 (c)+ 1
winningTicketIndex_1(uint256) := TMP_23737(uint256)
 ranges = prizeRanges[prizeId]
REF_9786(Raffle.Range[]) -> prizeRanges_10[prizeId_1]
ranges_1 (-> [])(Raffle.Range[]) = ['REF_9786(Raffle.Range[])']
 ranges.length > 0
REF_9787 -> LENGTH ranges_1 (-> [])
TMP_23738(bool) = REF_9787 > 0
CONDITION TMP_23738
 lo = 0
lo_1(uint256) := 0(uint256)
 hi = ranges.length - 1
REF_9788 -> LENGTH ranges_1 (-> [])
TMP_23739(uint256) = REF_9788 (c)- 1
hi_1(uint256) := TMP_23739(uint256)
 lo < hi
TMP_23740(bool) = lo_1 < hi_1
CONDITION TMP_23740
 mid = (lo + hi) >> 1
TMP_23741(uint256) = lo_1 (c)+ hi_1
TMP_23742(uint256) = TMP_23741 >> 1
mid_1(uint256) := TMP_23742(uint256)
 winningTicketIndex <= ranges[mid].cumulativeEnd
REF_9789(Raffle.Range) -> ranges_1 (-> [])[mid_1]
REF_9790(uint256) -> REF_9789.cumulativeEnd
TMP_23743(bool) = winningTicketIndex_1 <= REF_9790
CONDITION TMP_23743
 hi = mid
hi_2(uint256) := mid_1(uint256)
 lo = mid + 1
TMP_23744(uint256) = mid_1 (c)+ 1
lo_2(uint256) := TMP_23744(uint256)
lo_3(uint256) := phi(['lo_1', 'lo_2'])
hi_3(uint256) := phi(['hi_2', 'hi_1'])
 winnerAddress = ranges[lo].user
REF_9791(Raffle.Range) -> ranges_1 (-> [])[lo_1]
REF_9792(address) -> REF_9791.user
winnerAddress_1(address) := REF_9792(address)
winnerAddress_2(address) := phi(['winnerAddress_0', 'winnerAddress_1'])
 prizeWinners[prizeId].push(Winner({winnerAddress:winnerAddress,winningTicketIndex:winningTicketIndex,drawnAt:block.timestamp,claimed:false}))
REF_9793(Raffle.Winner[]) -> prizeWinners_2[prizeId_1]
TMP_23745(Raffle.Winner) = new Winner(winnerAddress_2,winningTicketIndex_1,block.timestamp,False)
REF_9795 -> LENGTH REF_9793
TMP_23747(uint256) := REF_9795(uint256)
TMP_23748(uint256) = TMP_23747 (c)+ 1
prizeWinners_3(mapping(uint256 => Raffle.Winner[])) := phi(['prizeWinners_2'])
REF_9795(uint256) (->prizeWinners_4) := TMP_23748(uint256)
REF_9796(Raffle.Winner) -> REF_9793[TMP_23747]
prizeWinners_4(mapping(uint256 => Raffle.Winner[])) := phi(['prizeWinners_3'])
REF_9796(Raffle.Winner) (->prizeWinners_4) := TMP_23745(Raffle.Winner)
 winnersDrawn[prizeId] ++
REF_9797(uint256) -> winnersDrawn_4[prizeId_1]
TMP_23749(uint256) := REF_9797(uint256)
winnersDrawn_5(mapping(uint256 => uint256)) := phi(['winnersDrawn_4'])
REF_9797(-> winnersDrawn_5) = REF_9797 (c)+ 1
 userWinCount[prizeId][winnerAddress] ++
REF_9798(mapping(address => uint256)) -> userWinCount_2[prizeId_1]
REF_9799(uint256) -> REF_9798[winnerAddress_2]
TMP_23750(uint256) := REF_9799(uint256)
userWinCount_3(mapping(uint256 => mapping(address => uint256))) := phi(['userWinCount_2'])
REF_9799(-> userWinCount_3) = REF_9799 (c)+ 1
 winnersDrawn[prizeId] == prizes[prizeId].quantity
REF_9800(uint256) -> winnersDrawn_5[prizeId_1]
REF_9801(Raffle.Prize) -> prizes_18[prizeId_1]
REF_9802(uint256) -> REF_9801.quantity
TMP_23751(bool) = REF_9800 == REF_9802
CONDITION TMP_23751
 prizes[prizeId].isActive = false
REF_9803(Raffle.Prize) -> prizes_18[prizeId_1]
REF_9804(bool) -> REF_9803.isActive
prizes_19(mapping(uint256 => Raffle.Prize)) := phi(['prizes_18'])
REF_9804(bool) (->prizes_19) := False(bool)
 WinnerSelected(prizeId,winnerAddress,winningTicketIndex)
Emit WinnerSelected(prizeId_1,winnerAddress_2,winningTicketIndex_1)
 onlyRole(SUPRA_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(SUPRA_ROLE_8)
```
#### Raffle.setWinner(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_15(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_12', 'ADMIN_ROLE_10', 'ADMIN_ROLE_22', 'ADMIN_ROLE_18', 'ADMIN_ROLE_24', 'ADMIN_ROLE_20', 'ADMIN_ROLE_6'])
 revert(string)(setWinner is deprecated, winner is set in handleWinnerSelection)
TMP_23754(None) = SOLIDITY_CALL revert(string)(setWinner is deprecated, winner is set in handleWinnerSelection)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_15)
```
#### Raffle.getWinner(uint256,uint256) [PUBLIC]
```slithir
prizeWinners_5(mapping(uint256 => Raffle.Winner[])) := phi(['prizeWinners_4', 'prizeWinners_7', 'prizeWinners_0', 'prizeWinners_5', 'prizeWinners_8'])
 prizeWinners[prizeId][index].winnerAddress
REF_9805(Raffle.Winner[]) -> prizeWinners_5[prizeId_1]
REF_9806(Raffle.Winner) -> REF_9805[index_1]
REF_9807(address) -> REF_9806.winnerAddress
RETURN REF_9807
```
#### Raffle.claimPrize(uint256,uint256) [EXTERNAL]
```slithir
prizes_20(mapping(uint256 => Raffle.Prize)) := phi(['prizes_26', 'prizes_14', 'prizes_16', 'prizes_19', 'prizes_10', 'prizes_0', 'prizes_29', 'prizes_20', 'prizes_30', 'prizes_3', 'prizes_18', 'prizes_21', 'prizes_22'])
winnings_1(mapping(address => uint256[])) := phi(['winnings_0', 'winnings_3', 'winnings_4'])
prizeWinners_6(mapping(uint256 => Raffle.Winner[])) := phi(['prizeWinners_4', 'prizeWinners_7', 'prizeWinners_0', 'prizeWinners_5', 'prizeWinners_8'])
winnersDrawn_6(mapping(uint256 => uint256)) := phi(['winnersDrawn_10', 'winnersDrawn_5', 'winnersDrawn_6', 'winnersDrawn_0', 'winnersDrawn_8', 'winnersDrawn_7', 'winnersDrawn_2'])
 prizes[prizeId].isActive && winnersDrawn[prizeId] < prizes[prizeId].quantity
REF_9808(Raffle.Prize) -> prizes_20[prizeId_1]
REF_9809(bool) -> REF_9808.isActive
REF_9810(uint256) -> winnersDrawn_6[prizeId_1]
REF_9811(Raffle.Prize) -> prizes_20[prizeId_1]
REF_9812(uint256) -> REF_9811.quantity
TMP_23756(bool) = REF_9810 < REF_9812
TMP_23757(bool) = REF_9809 && TMP_23756
CONDITION TMP_23757
 revert WinnerNotDrawn()()
TMP_23758(None) = SOLIDITY_CALL revert WinnerNotDrawn()()
 individualWin = prizeWinners[prizeId][winnerIndex]
REF_9813(Raffle.Winner[]) -> prizeWinners_6[prizeId_1]
REF_9814(Raffle.Winner) -> REF_9813[winnerIndex_1]
individualWin_1 (-> ['prizeWinners'])(Raffle.Winner) := REF_9814(Raffle.Winner)
 individualWin.claimed
REF_9815(bool) -> individualWin_1 (-> ['prizeWinners']).claimed
CONDITION REF_9815
 revert WinnerClaimed()()
TMP_23759(None) = SOLIDITY_CALL revert WinnerClaimed()()
 msg.sender != individualWin.winnerAddress
REF_9816(address) -> individualWin_1 (-> ['prizeWinners']).winnerAddress
TMP_23760(bool) = msg.sender != REF_9816
CONDITION TMP_23760
 revert NotAWinner()()
TMP_23761(None) = SOLIDITY_CALL revert NotAWinner()()
 individualWin.claimed = true
REF_9817(bool) -> individualWin_1 (-> ['prizeWinners']).claimed
individualWin_2 (-> ['prizeWinners'])(Raffle.Winner) := phi(["individualWin_1 (-> ['prizeWinners'])"])
REF_9817(bool) (->individualWin_2 (-> ['prizeWinners'])) := True(bool)
prizeWinners_7(mapping(uint256 => Raffle.Winner[])) := phi(["individualWin_2 (-> ['prizeWinners'])"])
 winnings[msg.sender].push(prizeId)
REF_9818(uint256[]) -> winnings_1[msg.sender]
REF_9820 -> LENGTH REF_9818
TMP_23763(uint256) := REF_9820(uint256)
TMP_23764(uint256) = TMP_23763 (c)+ 1
winnings_2(mapping(address => uint256[])) := phi(['winnings_1'])
REF_9820(uint256) (->winnings_3) := TMP_23764(uint256)
REF_9821(uint256) -> REF_9818[TMP_23763]
winnings_3(mapping(address => uint256[])) := phi(['winnings_2'])
REF_9821(uint256) (->winnings_3) := prizeId_1(uint256)
 PrizeClaimed(msg.sender,prizeId,winnerIndex)
Emit PrizeClaimed(msg.sender,prizeId_1,winnerIndex_1)
```
#### Raffle.cancelWinnerRequest(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_17(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_12', 'ADMIN_ROLE_10', 'ADMIN_ROLE_22', 'ADMIN_ROLE_18', 'ADMIN_ROLE_24', 'ADMIN_ROLE_20', 'ADMIN_ROLE_6'])
isWinnerRequestPending_5(mapping(uint256 => bool)) := phi(['isWinnerRequestPending_3', 'isWinnerRequestPending_4', 'isWinnerRequestPending_7', 'isWinnerRequestPending_0'])
 require(bool,string)(isWinnerRequestPending[prizeId],No request pending for this prize)
REF_9822(bool) -> isWinnerRequestPending_6[prizeId_1]
TMP_23766(None) = SOLIDITY_CALL require(bool,string)(REF_9822,No request pending for this prize)
 isWinnerRequestPending[prizeId] = false
REF_9823(bool) -> isWinnerRequestPending_6[prizeId_1]
isWinnerRequestPending_7(mapping(uint256 => bool)) := phi(['isWinnerRequestPending_6'])
REF_9823(bool) (->isWinnerRequestPending_7) := False(bool)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_17)
```
#### Raffle.getPrizeIds() [EXTERNAL]
```slithir
prizeIds_11(uint256[]) := phi(['prizeIds_7', 'prizeIds_4', 'prizeIds_0', 'prizeIds_12'])
 prizeIds
RETURN prizeIds_11
```
#### Raffle.getPrizeDetails() [EXTERNAL]
```slithir
prizes_22(mapping(uint256 => Raffle.Prize)) := phi(['prizes_26', 'prizes_14', 'prizes_16', 'prizes_19', 'prizes_10', 'prizes_0', 'prizes_29', 'prizes_20', 'prizes_30', 'prizes_3', 'prizes_18', 'prizes_21', 'prizes_22'])
prizeIds_12(uint256[]) := phi(['prizeIds_7', 'prizeIds_4', 'prizeIds_0', 'prizeIds_12'])
totalTickets_9(mapping(uint256 => uint256)) := phi(['totalTickets_7', 'totalTickets_5', 'totalTickets_0', 'totalTickets_9', 'totalTickets_8'])
totalUniqueUsers_7(mapping(uint256 => uint256)) := phi(['totalUniqueUsers_6', 'totalUniqueUsers_7', 'totalUniqueUsers_4', 'totalUniqueUsers_0'])
winnersDrawn_8(mapping(uint256 => uint256)) := phi(['winnersDrawn_10', 'winnersDrawn_5', 'winnersDrawn_6', 'winnersDrawn_0', 'winnersDrawn_8', 'winnersDrawn_7', 'winnersDrawn_2'])
 prizeCount = prizeIds.length
REF_9835 -> LENGTH prizeIds_12
prizeCount_1(uint256) := REF_9835(uint256)
 prizeArray = new Raffle.PrizeWithTickets[](prizeCount)
TMP_23769(Raffle.PrizeWithTickets[])  = new Raffle.PrizeWithTickets[](prizeCount_1)
prizeArray_1(Raffle.PrizeWithTickets[]) = ['TMP_23769(Raffle.PrizeWithTickets[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < prizeCount
prizeArray_2(Raffle.PrizeWithTickets[]) := phi(['prizeArray_1', 'prizeArray_3'])
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_23770(bool) = i_2 < prizeCount_1
CONDITION TMP_23770
 currentPrizeId = prizeIds[i]
REF_9836(uint256) -> prizeIds_12[i_2]
currentPrizeId_1(uint256) := REF_9836(uint256)
 currentPrize = prizes[currentPrizeId]
REF_9837(Raffle.Prize) -> prizes_22[currentPrizeId_1]
currentPrize_1 (-> ['prizes'])(Raffle.Prize) := REF_9837(Raffle.Prize)
 prizeArray[i] = PrizeWithTickets({name:currentPrize.name,description:currentPrize.description,value:currentPrize.value,endTimestamp:currentPrize.endTimestamp,isActive:currentPrize.isActive,quantity:currentPrize.quantity,winnersDrawn:winnersDrawn[currentPrizeId],totalTickets:totalTickets[currentPrizeId],totalUsers:totalUniqueUsers[currentPrizeId]})
REF_9838(Raffle.PrizeWithTickets) -> prizeArray_2[i_2]
REF_9839(string) -> currentPrize_1 (-> ['prizes']).name
REF_9840(string) -> currentPrize_1 (-> ['prizes']).description
REF_9841(uint256) -> currentPrize_1 (-> ['prizes']).value
REF_9842(uint256) -> currentPrize_1 (-> ['prizes']).endTimestamp
REF_9843(bool) -> currentPrize_1 (-> ['prizes']).isActive
REF_9844(uint256) -> currentPrize_1 (-> ['prizes']).quantity
REF_9845(uint256) -> winnersDrawn_8[currentPrizeId_1]
REF_9846(uint256) -> totalTickets_9[currentPrizeId_1]
REF_9847(uint256) -> totalUniqueUsers_7[currentPrizeId_1]
TMP_23771(Raffle.PrizeWithTickets) = new PrizeWithTickets(REF_9839,REF_9840,REF_9841,REF_9842,REF_9843,REF_9844,REF_9845,REF_9846,REF_9847)
prizeArray_3(Raffle.PrizeWithTickets[]) := phi(['prizeArray_2'])
REF_9838(Raffle.PrizeWithTickets) (->prizeArray_3) := TMP_23771(Raffle.PrizeWithTickets)
 i ++
TMP_23772(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 prizeArray
RETURN prizeArray_2
```
#### Raffle.getPrizeWinners(uint256) [EXTERNAL]
```slithir
prizeWinners_8(mapping(uint256 => Raffle.Winner[])) := phi(['prizeWinners_4', 'prizeWinners_7', 'prizeWinners_0', 'prizeWinners_5', 'prizeWinners_8'])
 prizeWinners[prizeId]
REF_9848(Raffle.Winner[]) -> prizeWinners_8[prizeId_1]
RETURN REF_9848
```
#### Raffle.getUserWinnings(address) [EXTERNAL]
```slithir
winnings_4(mapping(address => uint256[])) := phi(['winnings_0', 'winnings_3', 'winnings_4'])
 winnings[user]
REF_9849(uint256[]) -> winnings_4[user_1]
RETURN REF_9849
```
#### Raffle.updatePrizeEndTimestamp(uint256,uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_19(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_12', 'ADMIN_ROLE_10', 'ADMIN_ROLE_22', 'ADMIN_ROLE_18', 'ADMIN_ROLE_24', 'ADMIN_ROLE_20', 'ADMIN_ROLE_6'])
prizes_23(mapping(uint256 => Raffle.Prize)) := phi(['prizes_26', 'prizes_14', 'prizes_16', 'prizes_19', 'prizes_10', 'prizes_0', 'prizes_29', 'prizes_20', 'prizes_30', 'prizes_3', 'prizes_18', 'prizes_21', 'prizes_22'])
 prizes[prizeId].endTimestamp = endTimestamp
REF_9850(Raffle.Prize) -> prizes_25[prizeId_1]
REF_9851(uint256) -> REF_9850.endTimestamp
prizes_26(mapping(uint256 => Raffle.Prize)) := phi(['prizes_25'])
REF_9851(uint256) (->prizes_26) := endTimestamp_1(uint256)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_19)
 prizeIsActive(prizeId)
MODIFIER_CALL, Raffle.prizeIsActive(uint256)(prizeId_1)
prizes_25(mapping(uint256 => Raffle.Prize)) := phi(['prizes_30'])
```
#### Raffle.setPrizeActive(uint256,bool) [EXTERNAL]
```slithir
ADMIN_ROLE_21(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_12', 'ADMIN_ROLE_10', 'ADMIN_ROLE_22', 'ADMIN_ROLE_18', 'ADMIN_ROLE_24', 'ADMIN_ROLE_20', 'ADMIN_ROLE_6'])
prizes_27(mapping(uint256 => Raffle.Prize)) := phi(['prizes_26', 'prizes_14', 'prizes_16', 'prizes_19', 'prizes_10', 'prizes_0', 'prizes_29', 'prizes_20', 'prizes_30', 'prizes_3', 'prizes_18', 'prizes_21', 'prizes_22'])
winnersDrawn_9(mapping(uint256 => uint256)) := phi(['winnersDrawn_10', 'winnersDrawn_5', 'winnersDrawn_6', 'winnersDrawn_0', 'winnersDrawn_8', 'winnersDrawn_7', 'winnersDrawn_2'])
 prize = prizes[prizeId]
REF_9852(Raffle.Prize) -> prizes_28[prizeId_1]
prize_1 (-> ['prizes'])(Raffle.Prize) := REF_9852(Raffle.Prize)
 require(bool,string)(bytes(prize.name).length != 0,Prize does not exist)
REF_9853(string) -> prize_1 (-> ['prizes']).name
TMP_23775 = CONVERT REF_9853 to bytes
REF_9854 -> LENGTH TMP_23775
TMP_23776(bool) = REF_9854 != 0
TMP_23777(None) = SOLIDITY_CALL require(bool,string)(TMP_23776,Prize does not exist)
 active
CONDITION active_1
 require(bool,string)(winnersDrawn[prizeId] < prize.quantity,All winners already selected)
REF_9855(uint256) -> winnersDrawn_10[prizeId_1]
REF_9856(uint256) -> prize_1 (-> ['prizes']).quantity
TMP_23778(bool) = REF_9855 < REF_9856
TMP_23779(None) = SOLIDITY_CALL require(bool,string)(TMP_23778,All winners already selected)
 prizes[prizeId].isActive = active
REF_9857(Raffle.Prize) -> prizes_28[prizeId_1]
REF_9858(bool) -> REF_9857.isActive
prizes_29(mapping(uint256 => Raffle.Prize)) := phi(['prizes_28'])
REF_9858(bool) (->prizes_29) := active_1(bool)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_21)
```
#### Raffle.receive() [EXTERNAL]
```slithir

```

#### ISupraRouterContract.generateRequest(string,uint8,uint256,address) [EXTERNAL]
```slithir

```
