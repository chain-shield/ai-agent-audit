
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
_migrationComplete bool
__gap uint256[50]
nextPrizeId uint256

```
#### Raffle._authorizeUpgrade(address) [INTERNAL]
```slithir
newImplementation_1(address) := phi(['newImplementation_1'])
ADMIN_ROLE_21(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_20', 'ADMIN_ROLE_12', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_6', 'ADMIN_ROLE_16', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10'])
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_21)
```
#### Raffle.initialize(address,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_5', 'DEFAULT_ADMIN_ROLE_0'])
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_20', 'ADMIN_ROLE_12', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_6', 'ADMIN_ROLE_16', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10'])
SUPRA_ROLE_1(bytes32) := phi(['SUPRA_ROLE_0', 'SUPRA_ROLE_9', 'SUPRA_ROLE_7'])
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 __UUPSUpgradeable_init()
INTERNAL_CALL, UUPSUpgradeable.__UUPSUpgradeable_init()()
 spinContract = ISpin(_spinContract)
TMP_23453 = CONVERT _spinContract_1 to ISpin
spinContract_1(ISpin) := TMP_23453(ISpin)
 supraRouter = ISupraRouterContract(_supraRouter)
TMP_23454 = CONVERT _supraRouter_1 to ISupraRouterContract
supraRouter_1(ISupraRouterContract) := TMP_23454(ISupraRouterContract)
 admin = msg.sender
admin_1(address) := msg.sender(address)
 nextPrizeId = 1
nextPrizeId_1(uint256) := 1(uint256)
 _grantRole(DEFAULT_ADMIN_ROLE,msg.sender)
TMP_23455(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_4,msg.sender)
 _grantRole(ADMIN_ROLE,msg.sender)
TMP_23456(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(ADMIN_ROLE_5,msg.sender)
 _grantRole(SUPRA_ROLE,_supraRouter)
TMP_23457(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(SUPRA_ROLE_6,_supraRouter_1)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### Raffle.addPrize(string,string,uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_7(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_20', 'ADMIN_ROLE_12', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_6', 'ADMIN_ROLE_16', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10'])
prizes_1(mapping(uint256 => Raffle.Prize)) := phi(['prizes_23', 'prizes_13', 'prizes_3', 'prizes_35', 'prizes_0', 'prizes_20', 'prizes_26', 'prizes_28', 'prizes_24', 'prizes_9', 'prizes_36', 'prizes_27', 'prizes_32', 'prizes_16'])
prizeIds_1(uint256[]) := phi(['prizeIds_12', 'prizeIds_0', 'prizeIds_4', 'prizeIds_7'])
nextPrizeId_2(uint256) := phi(['nextPrizeId_0', 'nextPrizeId_1', 'nextPrizeId_4'])
 prizeId = nextPrizeId ++
TMP_23459(uint256) := nextPrizeId_3(uint256)
nextPrizeId_4(uint256) = nextPrizeId_3 (c)+ 1
prizeId_1(uint256) := TMP_23459(uint256)
 prizeIds.push(prizeId)
REF_9585 -> LENGTH prizeIds_2
TMP_23461(uint256) := REF_9585(uint256)
TMP_23462(uint256) = TMP_23461 (c)+ 1
prizeIds_3(uint256[]) := phi(['prizeIds_2'])
REF_9585(uint256) (->prizeIds_3) := TMP_23462(uint256)
REF_9586(uint256) -> prizeIds_3[TMP_23461]
prizeIds_4(uint256[]) := phi(['prizeIds_3'])
REF_9586(uint256) (->prizeIds_4) := prizeId_1(uint256)
 require(bool,string)(bytes(prizes[prizeId].name).length == 0,Prize ID already in use)
REF_9587(Raffle.Prize) -> prizes_2[prizeId_1]
REF_9588(string) -> REF_9587.name
TMP_23463 = CONVERT REF_9588 to bytes
REF_9589 -> LENGTH TMP_23463
TMP_23464(bool) = REF_9589 == 0
TMP_23465(None) = SOLIDITY_CALL require(bool,string)(TMP_23464,Prize ID already in use)
 prizes[prizeId] = Prize({name:name,description:description,value:value,endTimestamp:0,isActive:true,winner:address(0),winnerIndex:0,claimed:false})
REF_9590(Raffle.Prize) -> prizes_2[prizeId_1]
TMP_23466 = CONVERT 0 to address
TMP_23467(Raffle.Prize) = new Prize(name_1,description_1,value_1,0,True,TMP_23466,0,False)
prizes_3(mapping(uint256 => Raffle.Prize)) := phi(['prizes_2'])
REF_9590(Raffle.Prize) (->prizes_3) := TMP_23467(Raffle.Prize)
 PrizeAdded(prizeId,name)
Emit PrizeAdded(prizeId_1,name_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_7)
```
#### Raffle.editPrize(uint256,string,string,uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_9(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_20', 'ADMIN_ROLE_12', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_6', 'ADMIN_ROLE_16', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10'])
prizes_4(mapping(uint256 => Raffle.Prize)) := phi(['prizes_23', 'prizes_13', 'prizes_3', 'prizes_35', 'prizes_0', 'prizes_20', 'prizes_26', 'prizes_28', 'prizes_24', 'prizes_9', 'prizes_36', 'prizes_27', 'prizes_32', 'prizes_16'])
 prize = prizes[prizeId]
REF_9591(Raffle.Prize) -> prizes_6[prizeId_1]
prize_1 (-> ['prizes'])(Raffle.Prize) := REF_9591(Raffle.Prize)
 prize.name = name
REF_9592(string) -> prize_1 (-> ['prizes']).name
prize_2 (-> ['prizes'])(Raffle.Prize) := phi(["prize_1 (-> ['prizes'])"])
REF_9592(string) (->prize_2 (-> ['prizes'])) := name_1(string)
prizes_7(mapping(uint256 => Raffle.Prize)) := phi(["prize_2 (-> ['prizes'])"])
 prize.description = description
REF_9593(string) -> prize_2 (-> ['prizes']).description
prize_3 (-> ['prizes'])(Raffle.Prize) := phi(["prize_2 (-> ['prizes'])"])
REF_9593(string) (->prize_3 (-> ['prizes'])) := description_1(string)
prizes_8(mapping(uint256 => Raffle.Prize)) := phi(["prize_3 (-> ['prizes'])"])
 prize.value = value
REF_9594(uint256) -> prize_3 (-> ['prizes']).value
prize_4 (-> ['prizes'])(Raffle.Prize) := phi(["prize_3 (-> ['prizes'])"])
REF_9594(uint256) (->prize_4 (-> ['prizes'])) := value_1(uint256)
prizes_9(mapping(uint256 => Raffle.Prize)) := phi(["prize_4 (-> ['prizes'])"])
 PrizeEdited(prizeId,name,description,value)
Emit PrizeEdited(prizeId_1,name_1,description_1,value_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_9)
 prizeIsActive(prizeId)
MODIFIER_CALL, Raffle.prizeIsActive(uint256)(prizeId_1)
prizes_6(mapping(uint256 => Raffle.Prize)) := phi(['prizes_36'])
```
#### Raffle.removePrize(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_11(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_20', 'ADMIN_ROLE_12', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_6', 'ADMIN_ROLE_16', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10'])
prizes_10(mapping(uint256 => Raffle.Prize)) := phi(['prizes_23', 'prizes_13', 'prizes_3', 'prizes_35', 'prizes_0', 'prizes_20', 'prizes_26', 'prizes_28', 'prizes_24', 'prizes_9', 'prizes_36', 'prizes_27', 'prizes_32', 'prizes_16'])
prizeIds_5(uint256[]) := phi(['prizeIds_12', 'prizeIds_0', 'prizeIds_4', 'prizeIds_7'])
 prizes[prizeId].isActive = false
REF_9595(Raffle.Prize) -> prizes_12[prizeId_1]
REF_9596(bool) -> REF_9595.isActive
prizes_13(mapping(uint256 => Raffle.Prize)) := phi(['prizes_12'])
REF_9596(bool) (->prizes_13) := False(bool)
 len = prizeIds.length
REF_9597 -> LENGTH prizeIds_7
len_1(uint256) := REF_9597(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < len
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_23473(bool) = i_2 < len_1
CONDITION TMP_23473
 prizeIds[i] == prizeId
REF_9598(uint256) -> prizeIds_7[i_2]
TMP_23474(bool) = REF_9598 == prizeId_1
CONDITION TMP_23474
 prizeIds[i] = prizeIds[len - 1]
REF_9599(uint256) -> prizeIds_7[i_2]
TMP_23475(uint256) = len_1 (c)- 1
REF_9600(uint256) -> prizeIds_7[TMP_23475]
prizeIds_8(uint256[]) := phi(['prizeIds_7'])
REF_9599(uint256) (->prizeIds_8) := REF_9600(uint256)
 prizeIds.pop()
REF_9602 -> LENGTH prizeIds_8
TMP_23477(uint256) = REF_9602 (c)- 1
REF_9603(uint256) -> prizeIds_8[TMP_23477]
prizeIds_9 = delete REF_9603 
REF_9604 -> LENGTH prizeIds_9
prizeIds_10(uint256[]) := phi(['prizeIds_9'])
REF_9604(uint256) (->prizeIds_10) := TMP_23477(uint256)
 i ++
TMP_23478(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 PrizeRemoved(prizeId)
Emit PrizeRemoved(prizeId_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_11)
 prizeIsActive(prizeId)
MODIFIER_CALL, Raffle.prizeIsActive(uint256)(prizeId_1)
prizes_12(mapping(uint256 => Raffle.Prize)) := phi(['prizes_36'])
```
#### Raffle.spendRaffle(uint256,uint256) [EXTERNAL]
```slithir
spinContract_2(ISpin) := phi(['spinContract_0', 'spinContract_1', 'spinContract_5'])
prizeRanges_1(mapping(uint256 => Raffle.Range[])) := phi(['prizeRanges_9', 'prizeRanges_0', 'prizeRanges_11', 'prizeRanges_6'])
totalTickets_1(mapping(uint256 => uint256)) := phi(['totalTickets_7', 'totalTickets_8', 'totalTickets_5', 'totalTickets_0', 'totalTickets_9'])
userHasEnteredPrize_1(mapping(uint256 => mapping(address => bool))) := phi(['userHasEnteredPrize_4', 'userHasEnteredPrize_0'])
totalUniqueUsers_1(mapping(uint256 => uint256)) := phi(['totalUniqueUsers_7', 'totalUniqueUsers_6', 'totalUniqueUsers_0', 'totalUniqueUsers_4'])
 require(bool,string)(ticketAmount > 0,Must spend at least 1 ticket)
TMP_23482(bool) = ticketAmount_1 > 0
TMP_23483(None) = SOLIDITY_CALL require(bool,string)(TMP_23482,Must spend at least 1 ticket)
 (None,None,None,None,userRaffleTickets,None,None) = spinContract.getUserData(msg.sender)
TUPLE_147(uint256,uint256,uint256,uint256,uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:spinContract_3(ISpin), function:getUserData, arguments:['msg.sender']  
spinContract_4(ISpin) := phi(['spinContract_1', 'spinContract_3', 'spinContract_5'])
prizeRanges_3(mapping(uint256 => Raffle.Range[])) := phi(['prizeRanges_2', 'prizeRanges_9', 'prizeRanges_11', 'prizeRanges_6'])
totalTickets_3(mapping(uint256 => uint256)) := phi(['totalTickets_7', 'totalTickets_8', 'totalTickets_5', 'totalTickets_2', 'totalTickets_9'])
userHasEnteredPrize_3(mapping(uint256 => mapping(address => bool))) := phi(['userHasEnteredPrize_4', 'userHasEnteredPrize_2'])
totalUniqueUsers_3(mapping(uint256 => uint256)) := phi(['totalUniqueUsers_7', 'totalUniqueUsers_6', 'totalUniqueUsers_2', 'totalUniqueUsers_4'])
userRaffleTickets_1(uint256)= UNPACK TUPLE_147 index: 4 
 userRaffleTickets < ticketAmount
TMP_23484(bool) = userRaffleTickets_1 < ticketAmount_1
CONDITION TMP_23484
 revert InsufficientTickets()()
TMP_23485(None) = SOLIDITY_CALL revert InsufficientTickets()()
 spinContract.spendRaffleTickets(msg.sender,ticketAmount)
HIGH_LEVEL_CALL, dest:spinContract_4(ISpin), function:spendRaffleTickets, arguments:['msg.sender', 'ticketAmount_1']  
spinContract_5(ISpin) := phi(['spinContract_4', 'spinContract_1', 'spinContract_5'])
prizeRanges_4(mapping(uint256 => Raffle.Range[])) := phi(['prizeRanges_3', 'prizeRanges_9', 'prizeRanges_11', 'prizeRanges_6'])
totalTickets_4(mapping(uint256 => uint256)) := phi(['totalTickets_7', 'totalTickets_8', 'totalTickets_5', 'totalTickets_3', 'totalTickets_9'])
userHasEnteredPrize_4(mapping(uint256 => mapping(address => bool))) := phi(['userHasEnteredPrize_4', 'userHasEnteredPrize_3'])
totalUniqueUsers_4(mapping(uint256 => uint256)) := phi(['totalUniqueUsers_7', 'totalUniqueUsers_6', 'totalUniqueUsers_3', 'totalUniqueUsers_4'])
 newTotal = totalTickets[prizeId] + ticketAmount
REF_9607(uint256) -> totalTickets_4[prizeId_1]
TMP_23487(uint256) = REF_9607 (c)+ ticketAmount_1
newTotal_1(uint256) := TMP_23487(uint256)
 prizeRanges[prizeId].push(Range({user:msg.sender,cumulativeEnd:newTotal}))
REF_9608(Raffle.Range[]) -> prizeRanges_4[prizeId_1]
TMP_23488(Raffle.Range) = new Range(msg.sender,newTotal_1)
REF_9610 -> LENGTH REF_9608
TMP_23490(uint256) := REF_9610(uint256)
TMP_23491(uint256) = TMP_23490 (c)+ 1
prizeRanges_5(mapping(uint256 => Raffle.Range[])) := phi(['prizeRanges_4'])
REF_9610(uint256) (->prizeRanges_6) := TMP_23491(uint256)
REF_9611(Raffle.Range) -> REF_9608[TMP_23490]
prizeRanges_6(mapping(uint256 => Raffle.Range[])) := phi(['prizeRanges_5'])
REF_9611(Raffle.Range) (->prizeRanges_6) := TMP_23488(Raffle.Range)
 totalTickets[prizeId] = newTotal
REF_9612(uint256) -> totalTickets_4[prizeId_1]
totalTickets_5(mapping(uint256 => uint256)) := phi(['totalTickets_4'])
REF_9612(uint256) (->totalTickets_5) := newTotal_1(uint256)
 ! userHasEnteredPrize[prizeId][msg.sender]
REF_9613(mapping(address => bool)) -> userHasEnteredPrize_4[prizeId_1]
REF_9614(bool) -> REF_9613[msg.sender]
TMP_23492 = UnaryType.BANG REF_9614 
CONDITION TMP_23492
 userHasEnteredPrize[prizeId][msg.sender] = true
REF_9615(mapping(address => bool)) -> userHasEnteredPrize_4[prizeId_1]
REF_9616(bool) -> REF_9615[msg.sender]
userHasEnteredPrize_5(mapping(uint256 => mapping(address => bool))) := phi(['userHasEnteredPrize_4'])
REF_9616(bool) (->userHasEnteredPrize_5) := True(bool)
 totalUniqueUsers[prizeId] ++
REF_9617(uint256) -> totalUniqueUsers_4[prizeId_1]
TMP_23493(uint256) := REF_9617(uint256)
totalUniqueUsers_5(mapping(uint256 => uint256)) := phi(['totalUniqueUsers_4'])
REF_9617(-> totalUniqueUsers_5) = REF_9617 (c)+ 1
 TicketSpent(msg.sender,prizeId,ticketAmount)
Emit TicketSpent(msg.sender,prizeId_1,ticketAmount_1)
 prizeIsActive(prizeId)
MODIFIER_CALL, Raffle.prizeIsActive(uint256)(prizeId_1)
```
#### Raffle.requestWinner(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_13(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_20', 'ADMIN_ROLE_12', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_6', 'ADMIN_ROLE_16', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10'])
supraRouter_2(ISupraRouterContract) := phi(['supraRouter_1', 'supraRouter_0', 'supraRouter_5'])
prizes_14(mapping(uint256 => Raffle.Prize)) := phi(['prizes_23', 'prizes_13', 'prizes_3', 'prizes_35', 'prizes_0', 'prizes_20', 'prizes_26', 'prizes_28', 'prizes_24', 'prizes_9', 'prizes_36', 'prizes_27', 'prizes_32', 'prizes_16'])
prizeRanges_7(mapping(uint256 => Raffle.Range[])) := phi(['prizeRanges_9', 'prizeRanges_0', 'prizeRanges_11', 'prizeRanges_6'])
isWinnerRequestPending_1(mapping(uint256 => bool)) := phi(['isWinnerRequestPending_5', 'isWinnerRequestPending_4', 'isWinnerRequestPending_0'])
 prizeRanges[prizeId].length == 0
REF_9618(Raffle.Range[]) -> prizeRanges_9[prizeId_1]
REF_9619 -> LENGTH REF_9618
TMP_23496(bool) = REF_9619 == 0
CONDITION TMP_23496
 revert EmptyTicketPool()()
TMP_23497(None) = SOLIDITY_CALL revert EmptyTicketPool()()
 prizes[prizeId].winner != address(0)
REF_9620(Raffle.Prize) -> prizes_16[prizeId_1]
REF_9621(address) -> REF_9620.winner
TMP_23498 = CONVERT 0 to address
TMP_23499(bool) = REF_9621 != TMP_23498
CONDITION TMP_23499
 revert WinnerDrawn(address)(prizes[prizeId].winner)
REF_9622(Raffle.Prize) -> prizes_16[prizeId_1]
REF_9623(address) -> REF_9622.winner
TMP_23500(None) = SOLIDITY_CALL revert WinnerDrawn(address)(REF_9623)
 isWinnerRequestPending[prizeId]
REF_9624(bool) -> isWinnerRequestPending_3[prizeId_1]
CONDITION REF_9624
 revert WinnerRequestPending(uint256)(prizeId)
TMP_23501(None) = SOLIDITY_CALL revert WinnerRequestPending(uint256)(prizeId_1)
 isWinnerRequestPending[prizeId] = true
REF_9625(bool) -> isWinnerRequestPending_3[prizeId_1]
isWinnerRequestPending_4(mapping(uint256 => bool)) := phi(['isWinnerRequestPending_3'])
REF_9625(bool) (->isWinnerRequestPending_4) := True(bool)
 callbackSig = handleWinnerSelection(uint256,uint256[])
callbackSig_1(string) := handleWinnerSelection(uint256,uint256[])(string)
 requestId = supraRouter.generateRequest(callbackSig,1,1,uint256(keccak256(bytes)(abi.encodePacked(prizeId,block.timestamp))),msg.sender)
TMP_23502(bytes) = SOLIDITY_CALL abi.encodePacked()(prizeId_1,block.timestamp)
TMP_23503(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_23502)
TMP_23504 = CONVERT TMP_23503 to uint256
TMP_23505(uint256) = HIGH_LEVEL_CALL, dest:supraRouter_4(ISupraRouterContract), function:generateRequest, arguments:['callbackSig_1', '1', '1', 'TMP_23504', 'msg.sender']  
supraRouter_5(ISupraRouterContract) := phi(['supraRouter_1', 'supraRouter_5', 'supraRouter_4'])
requestId_1(uint256) := TMP_23505(uint256)
 pendingVRFRequests[requestId] = prizeId
REF_9628(uint256) -> pendingVRFRequests_0[requestId_1]
pendingVRFRequests_1(mapping(uint256 => uint256)) := phi(['pendingVRFRequests_0'])
REF_9628(uint256) (->pendingVRFRequests_1) := prizeId_1(uint256)
 WinnerRequested(prizeId,requestId)
Emit WinnerRequested(prizeId_1,requestId_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_13)
 prizeIsActive(prizeId)
MODIFIER_CALL, Raffle.prizeIsActive(uint256)(prizeId_1)
prizes_16(mapping(uint256 => Raffle.Prize)) := phi(['prizes_36'])
```
#### Raffle.handleWinnerSelection(uint256,uint256[]) [EXTERNAL]
```slithir
SUPRA_ROLE_8(bytes32) := phi(['SUPRA_ROLE_0', 'SUPRA_ROLE_9', 'SUPRA_ROLE_7'])
prizes_17(mapping(uint256 => Raffle.Prize)) := phi(['prizes_23', 'prizes_13', 'prizes_3', 'prizes_35', 'prizes_0', 'prizes_20', 'prizes_26', 'prizes_28', 'prizes_24', 'prizes_9', 'prizes_36', 'prizes_27', 'prizes_32', 'prizes_16'])
totalTickets_6(mapping(uint256 => uint256)) := phi(['totalTickets_7', 'totalTickets_8', 'totalTickets_5', 'totalTickets_0', 'totalTickets_9'])
pendingVRFRequests_2(mapping(uint256 => uint256)) := phi(['pendingVRFRequests_1', 'pendingVRFRequests_4', 'pendingVRFRequests_0'])
 prizeId = pendingVRFRequests[requestId]
REF_9629(uint256) -> pendingVRFRequests_3[requestId_1]
prizeId_1(uint256) := REF_9629(uint256)
 isWinnerRequestPending[prizeId] = false
REF_9630(bool) -> isWinnerRequestPending_4[prizeId_1]
isWinnerRequestPending_5(mapping(uint256 => bool)) := phi(['isWinnerRequestPending_4'])
REF_9630(bool) (->isWinnerRequestPending_5) := False(bool)
 delete pendingVRFRequests[requestId]
REF_9631(uint256) -> pendingVRFRequests_3[requestId_1]
pendingVRFRequests_4 = delete REF_9631 
 ! prizes[prizeId].isActive
REF_9632(Raffle.Prize) -> prizes_18[prizeId_1]
REF_9633(bool) -> REF_9632.isActive
TMP_23509 = UnaryType.BANG REF_9633 
CONDITION TMP_23509
 revert PrizeInactive()()
TMP_23510(None) = SOLIDITY_CALL revert PrizeInactive()()
 idx = (rng[0] % totalTickets[prizeId]) + 1
REF_9634(uint256) -> rng_1[0]
REF_9635(uint256) -> totalTickets_7[prizeId_1]
TMP_23511(uint256) = REF_9634 % REF_9635
TMP_23512(uint256) = TMP_23511 (c)+ 1
idx_1(uint256) := TMP_23512(uint256)
 prizes[prizeId].winnerIndex = idx
REF_9636(Raffle.Prize) -> prizes_18[prizeId_1]
REF_9637(uint256) -> REF_9636.winnerIndex
prizes_19(mapping(uint256 => Raffle.Prize)) := phi(['prizes_18'])
REF_9637(uint256) (->prizes_19) := idx_1(uint256)
 prizes[prizeId].isActive = false
REF_9638(Raffle.Prize) -> prizes_19[prizeId_1]
REF_9639(bool) -> REF_9638.isActive
prizes_20(mapping(uint256 => Raffle.Prize)) := phi(['prizes_19'])
REF_9639(bool) (->prizes_20) := False(bool)
 WinnerSelected(prizeId,idx)
Emit WinnerSelected(prizeId_1,idx_1)
 onlyRole(SUPRA_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(SUPRA_ROLE_8)
```
#### Raffle.setWinner(uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_15(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_20', 'ADMIN_ROLE_12', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_6', 'ADMIN_ROLE_16', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10'])
prizes_21(mapping(uint256 => Raffle.Prize)) := phi(['prizes_23', 'prizes_13', 'prizes_3', 'prizes_35', 'prizes_0', 'prizes_20', 'prizes_26', 'prizes_28', 'prizes_24', 'prizes_9', 'prizes_36', 'prizes_27', 'prizes_32', 'prizes_16'])
prizeRanges_10(mapping(uint256 => Raffle.Range[])) := phi(['prizeRanges_9', 'prizeRanges_0', 'prizeRanges_11', 'prizeRanges_6'])
 prize = prizes[prizeId]
REF_9640(Raffle.Prize) -> prizes_22[prizeId_1]
prize_1 (-> ['prizes'])(Raffle.Prize) := REF_9640(Raffle.Prize)
 require(bool,string)(prize.winnerIndex > 0,Winner index not set)
REF_9641(uint256) -> prize_1 (-> ['prizes']).winnerIndex
TMP_23515(bool) = REF_9641 > 0
TMP_23516(None) = SOLIDITY_CALL require(bool,string)(TMP_23515,Winner index not set)
 require(bool,string)(prize.winner == address(0),Winner already set)
REF_9642(address) -> prize_1 (-> ['prizes']).winner
TMP_23517 = CONVERT 0 to address
TMP_23518(bool) = REF_9642 == TMP_23517
TMP_23519(None) = SOLIDITY_CALL require(bool,string)(TMP_23518,Winner already set)
 ranges = prizeRanges[prizeId]
REF_9643(Raffle.Range[]) -> prizeRanges_11[prizeId_1]
ranges_1 (-> [])(Raffle.Range[]) = ['REF_9643(Raffle.Range[])']
 target = prize.winnerIndex
REF_9644(uint256) -> prize_1 (-> ['prizes']).winnerIndex
target_1(uint256) := REF_9644(uint256)
 ranges.length == 0 || target == 0
REF_9645 -> LENGTH ranges_1 (-> [])
TMP_23520(bool) = REF_9645 == 0
TMP_23521(bool) = target_1 == 0
TMP_23522(bool) = TMP_23520 || TMP_23521
CONDITION TMP_23522
 revert(string)(Invalid winner index)
TMP_23523(None) = SOLIDITY_CALL revert(string)(Invalid winner index)
 lo = 0
lo_1(uint256) := 0(uint256)
 hi = ranges.length - 1
REF_9646 -> LENGTH ranges_1 (-> [])
TMP_23524(uint256) = REF_9646 (c)- 1
hi_1(uint256) := TMP_23524(uint256)
 lo < hi
TMP_23525(bool) = lo_1 < hi_1
CONDITION TMP_23525
 mid = (lo + hi) >> 1
TMP_23526(uint256) = lo_1 (c)+ hi_1
TMP_23527(uint256) = TMP_23526 >> 1
mid_1(uint256) := TMP_23527(uint256)
 target <= ranges[mid].cumulativeEnd
REF_9647(Raffle.Range) -> ranges_1 (-> [])[mid_1]
REF_9648(uint256) -> REF_9647.cumulativeEnd
TMP_23528(bool) = target_1 <= REF_9648
CONDITION TMP_23528
 hi = mid
hi_2(uint256) := mid_1(uint256)
 lo = mid + 1
TMP_23529(uint256) = mid_1 (c)+ 1
lo_2(uint256) := TMP_23529(uint256)
lo_3(uint256) := phi(['lo_1', 'lo_2'])
hi_3(uint256) := phi(['hi_1', 'hi_2'])
 prize.winner = ranges[lo].user
REF_9649(address) -> prize_1 (-> ['prizes']).winner
REF_9650(Raffle.Range) -> ranges_1 (-> [])[lo_1]
REF_9651(address) -> REF_9650.user
prize_2 (-> ['prizes'])(Raffle.Prize) := phi(["prize_1 (-> ['prizes'])"])
REF_9649(address) (->prize_2 (-> ['prizes'])) := REF_9651(address)
prizes_23(mapping(uint256 => Raffle.Prize)) := phi(["prize_2 (-> ['prizes'])"])
 WinnerSet(prizeId,prize.winner)
REF_9652(address) -> prize_2 (-> ['prizes']).winner
Emit WinnerSet(prizeId_1,REF_9652)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_15)
```
#### Raffle.getWinner(uint256) [PUBLIC]
```slithir
prizes_24(mapping(uint256 => Raffle.Prize)) := phi(['prizes_23', 'prizes_13', 'prizes_3', 'prizes_35', 'prizes_0', 'prizes_20', 'prizes_26', 'prizes_28', 'prizes_24', 'prizes_9', 'prizes_36', 'prizes_27', 'prizes_32', 'prizes_16'])
 prizes[prizeId].winner
REF_9653(Raffle.Prize) -> prizes_24[prizeId_1]
REF_9654(address) -> REF_9653.winner
RETURN REF_9654
```
#### Raffle.claimPrize(uint256) [EXTERNAL]
```slithir
prizes_25(mapping(uint256 => Raffle.Prize)) := phi(['prizes_23', 'prizes_13', 'prizes_3', 'prizes_35', 'prizes_0', 'prizes_20', 'prizes_26', 'prizes_28', 'prizes_24', 'prizes_9', 'prizes_36', 'prizes_27', 'prizes_32', 'prizes_16'])
winnings_1(mapping(address => uint256[])) := phi(['winnings_0', 'winnings_3'])
 prize = prizes[prizeId]
REF_9655(Raffle.Prize) -> prizes_25[prizeId_1]
prize_1 (-> ['prizes'])(Raffle.Prize) := REF_9655(Raffle.Prize)
 prize.isActive
REF_9656(bool) -> prize_1 (-> ['prizes']).isActive
CONDITION REF_9656
 revert()()
TMP_23532(None) = SOLIDITY_CALL revert()()
 prize.winnerIndex == 0
REF_9657(uint256) -> prize_1 (-> ['prizes']).winnerIndex
TMP_23533(bool) = REF_9657 == 0
CONDITION TMP_23533
 revert WinnerNotDrawn()()
TMP_23534(None) = SOLIDITY_CALL revert WinnerNotDrawn()()
 prize.claimed
REF_9658(bool) -> prize_1 (-> ['prizes']).claimed
CONDITION REF_9658
 revert WinnerClaimed()()
TMP_23535(None) = SOLIDITY_CALL revert WinnerClaimed()()
 msg.sender != prize.winner
REF_9659(address) -> prize_1 (-> ['prizes']).winner
TMP_23536(bool) = msg.sender != REF_9659
CONDITION TMP_23536
 revert NotAWinner()()
TMP_23537(None) = SOLIDITY_CALL revert NotAWinner()()
 prize.claimed = true
REF_9660(bool) -> prize_1 (-> ['prizes']).claimed
prize_2 (-> ['prizes'])(Raffle.Prize) := phi(["prize_1 (-> ['prizes'])"])
REF_9660(bool) (->prize_2 (-> ['prizes'])) := True(bool)
prizes_26(mapping(uint256 => Raffle.Prize)) := phi(["prize_2 (-> ['prizes'])"])
 winnings[msg.sender].push(prizeId)
REF_9661(uint256[]) -> winnings_1[msg.sender]
REF_9663 -> LENGTH REF_9661
TMP_23539(uint256) := REF_9663(uint256)
TMP_23540(uint256) = TMP_23539 (c)+ 1
winnings_2(mapping(address => uint256[])) := phi(['winnings_1'])
REF_9663(uint256) (->winnings_3) := TMP_23540(uint256)
REF_9664(uint256) -> REF_9661[TMP_23539]
winnings_3(mapping(address => uint256[])) := phi(['winnings_2'])
REF_9664(uint256) (->winnings_3) := prizeId_1(uint256)
 PrizeClaimed(msg.sender,prizeId)
Emit PrizeClaimed(msg.sender,prizeId_1)
```
#### Raffle.getPrizeIds() [EXTERNAL]
```slithir
prizeIds_11(uint256[]) := phi(['prizeIds_12', 'prizeIds_0', 'prizeIds_4', 'prizeIds_7'])
 prizeIds
RETURN prizeIds_11
```
#### Raffle.getPrizeDetails() [EXTERNAL]
```slithir
prizes_28(mapping(uint256 => Raffle.Prize)) := phi(['prizes_23', 'prizes_13', 'prizes_3', 'prizes_35', 'prizes_0', 'prizes_20', 'prizes_26', 'prizes_28', 'prizes_24', 'prizes_9', 'prizes_36', 'prizes_27', 'prizes_32', 'prizes_16'])
prizeIds_12(uint256[]) := phi(['prizeIds_12', 'prizeIds_0', 'prizeIds_4', 'prizeIds_7'])
totalTickets_9(mapping(uint256 => uint256)) := phi(['totalTickets_7', 'totalTickets_8', 'totalTickets_5', 'totalTickets_0', 'totalTickets_9'])
totalUniqueUsers_7(mapping(uint256 => uint256)) := phi(['totalUniqueUsers_7', 'totalUniqueUsers_6', 'totalUniqueUsers_0', 'totalUniqueUsers_4'])
 prizeCount = prizeIds.length
REF_9674 -> LENGTH prizeIds_12
prizeCount_1(uint256) := REF_9674(uint256)
 prizeArray = new Raffle.PrizeWithTickets[](prizeCount)
TMP_23543(Raffle.PrizeWithTickets[])  = new Raffle.PrizeWithTickets[](prizeCount_1)
prizeArray_1(Raffle.PrizeWithTickets[]) = ['TMP_23543(Raffle.PrizeWithTickets[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < prizeCount
prizeArray_2(Raffle.PrizeWithTickets[]) := phi(['prizeArray_3', 'prizeArray_1'])
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_23544(bool) = i_2 < prizeCount_1
CONDITION TMP_23544
 currentPrizeId = prizeIds[i]
REF_9675(uint256) -> prizeIds_12[i_2]
currentPrizeId_1(uint256) := REF_9675(uint256)
 currentPrize = prizes[currentPrizeId]
REF_9676(Raffle.Prize) -> prizes_28[currentPrizeId_1]
currentPrize_1 (-> ['prizes'])(Raffle.Prize) := REF_9676(Raffle.Prize)
 prizeArray[i] = PrizeWithTickets({name:currentPrize.name,description:currentPrize.description,value:currentPrize.value,endTimestamp:currentPrize.endTimestamp,isActive:currentPrize.isActive,winner:currentPrize.winner,winnerIndex:currentPrize.winnerIndex,claimed:currentPrize.claimed,totalTickets:totalTickets[currentPrizeId],totalUsers:totalUniqueUsers[currentPrizeId]})
REF_9677(Raffle.PrizeWithTickets) -> prizeArray_2[i_2]
REF_9678(string) -> currentPrize_1 (-> ['prizes']).name
REF_9679(string) -> currentPrize_1 (-> ['prizes']).description
REF_9680(uint256) -> currentPrize_1 (-> ['prizes']).value
REF_9681(uint256) -> currentPrize_1 (-> ['prizes']).endTimestamp
REF_9682(bool) -> currentPrize_1 (-> ['prizes']).isActive
REF_9683(address) -> currentPrize_1 (-> ['prizes']).winner
REF_9684(uint256) -> currentPrize_1 (-> ['prizes']).winnerIndex
REF_9685(bool) -> currentPrize_1 (-> ['prizes']).claimed
REF_9686(uint256) -> totalTickets_9[currentPrizeId_1]
REF_9687(uint256) -> totalUniqueUsers_7[currentPrizeId_1]
TMP_23545(Raffle.PrizeWithTickets) = new PrizeWithTickets(REF_9678,REF_9679,REF_9680,REF_9681,REF_9682,REF_9683,REF_9684,REF_9685,REF_9686,REF_9687)
prizeArray_3(Raffle.PrizeWithTickets[]) := phi(['prizeArray_2'])
REF_9677(Raffle.PrizeWithTickets) (->prizeArray_3) := TMP_23545(Raffle.PrizeWithTickets)
 i ++
TMP_23546(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 prizeArray
RETURN prizeArray_2
```
#### Raffle.updatePrizeEndTimestamp(uint256,uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_17(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_20', 'ADMIN_ROLE_12', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_6', 'ADMIN_ROLE_16', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10'])
prizes_29(mapping(uint256 => Raffle.Prize)) := phi(['prizes_23', 'prizes_13', 'prizes_3', 'prizes_35', 'prizes_0', 'prizes_20', 'prizes_26', 'prizes_28', 'prizes_24', 'prizes_9', 'prizes_36', 'prizes_27', 'prizes_32', 'prizes_16'])
 prizes[prizeId].endTimestamp = endTimestamp
REF_9688(Raffle.Prize) -> prizes_31[prizeId_1]
REF_9689(uint256) -> REF_9688.endTimestamp
prizes_32(mapping(uint256 => Raffle.Prize)) := phi(['prizes_31'])
REF_9689(uint256) (->prizes_32) := endTimestamp_1(uint256)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_17)
 prizeIsActive(prizeId)
MODIFIER_CALL, Raffle.prizeIsActive(uint256)(prizeId_1)
prizes_31(mapping(uint256 => Raffle.Prize)) := phi(['prizes_36'])
```
#### Raffle.setPrizeActive(uint256,bool) [EXTERNAL]
```slithir
ADMIN_ROLE_19(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_14', 'ADMIN_ROLE_20', 'ADMIN_ROLE_12', 'ADMIN_ROLE_8', 'ADMIN_ROLE_22', 'ADMIN_ROLE_6', 'ADMIN_ROLE_16', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10'])
prizes_33(mapping(uint256 => Raffle.Prize)) := phi(['prizes_23', 'prizes_13', 'prizes_3', 'prizes_35', 'prizes_0', 'prizes_20', 'prizes_26', 'prizes_28', 'prizes_24', 'prizes_9', 'prizes_36', 'prizes_27', 'prizes_32', 'prizes_16'])
 prize = prizes[prizeId]
REF_9690(Raffle.Prize) -> prizes_34[prizeId_1]
prize_1 (-> ['prizes'])(Raffle.Prize) := REF_9690(Raffle.Prize)
 require(bool,string)(bytes(prize.name).length != 0,Prize does not exist)
REF_9691(string) -> prize_1 (-> ['prizes']).name
TMP_23549 = CONVERT REF_9691 to bytes
REF_9692 -> LENGTH TMP_23549
TMP_23550(bool) = REF_9692 != 0
TMP_23551(None) = SOLIDITY_CALL require(bool,string)(TMP_23550,Prize does not exist)
 require(bool,string)(prize.winnerIndex == 0,Winner already selected)
REF_9693(uint256) -> prize_1 (-> ['prizes']).winnerIndex
TMP_23552(bool) = REF_9693 == 0
TMP_23553(None) = SOLIDITY_CALL require(bool,string)(TMP_23552,Winner already selected)
 prizes[prizeId].isActive = active
REF_9694(Raffle.Prize) -> prizes_34[prizeId_1]
REF_9695(bool) -> REF_9694.isActive
prizes_35(mapping(uint256 => Raffle.Prize)) := phi(['prizes_34'])
REF_9695(bool) (->prizes_35) := active_1(bool)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_19)
```
#### Raffle.receive() [EXTERNAL]
```slithir

```

#### ISupraRouterContract.generateRequest(string,uint8,uint256,address) [EXTERNAL]
```slithir

```
