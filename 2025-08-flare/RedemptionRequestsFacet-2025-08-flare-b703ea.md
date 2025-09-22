





### Storage layout (ERC20) 

```text
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string

```
















#### RedemptionRequestsFacet.convertDustToTicket(address) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3500(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3500'])(AssetManagerSettings.Data) := TMP_3500(AssetManagerSettings.Data)
 agent = Agent.get(_agentVault)
TMP_3501(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_3501'])(Agent.State) := TMP_3501(Agent.State)
 agent.dustAMG >= settings.lotSizeAMG
REF_2301(uint64) -> agent_1 (-> ['TMP_3501']).dustAMG
REF_2302(uint64) -> settings_1 (-> ['TMP_3500']).lotSizeAMG
TMP_3502(bool) = REF_2301 >= REF_2302
CONDITION TMP_3502
 remainingDustAMG = agent.dustAMG % settings.lotSizeAMG
REF_2303(uint64) -> agent_1 (-> ['TMP_3501']).dustAMG
REF_2304(uint64) -> settings_1 (-> ['TMP_3500']).lotSizeAMG
TMP_3503(uint64) = REF_2303 % REF_2304
remainingDustAMG_1(uint64) := TMP_3503(uint64)
 ticketValueAMG = agent.dustAMG - remainingDustAMG
REF_2305(uint64) -> agent_1 (-> ['TMP_3501']).dustAMG
TMP_3504(uint64) = REF_2305 (c)- remainingDustAMG_1
ticketValueAMG_1(uint64) := TMP_3504(uint64)
 AgentBacking.createRedemptionTicket(agent,ticketValueAMG)
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.createRedemptionTicket(Agent.State,uint64), arguments:["agent_1 (-> ['TMP_3501'])", 'ticketValueAMG_1'] 
 AgentBacking.changeDust(agent,remainingDustAMG)
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.changeDust(Agent.State,uint64), arguments:["agent_1 (-> ['TMP_3501'])", 'remainingDustAMG_1'] 
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### RedemptionRequestsFacet.maxRedemptionFromAgent(address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_3454(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_3454'])(Agent.State) := TMP_3454(Agent.State)
 state = AssetManagerState.get()
TMP_3455(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_3455'])(AssetManagerState.State) := TMP_3455(AssetManagerState.State)
 maxRedemptionAMG = agent.dustAMG
REF_2250(uint64) -> agent_1 (-> ['TMP_3454']).dustAMG
maxRedemptionAMG_1(uint64) := REF_2250(uint64)
 maxRedeemedTickets = Globals.getSettings().maxRedeemedTickets
TMP_3456(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_2252(uint16) -> TMP_3456.maxRedeemedTickets
maxRedeemedTickets_1(uint256) := REF_2252(uint16)
 ticketId = state.redemptionQueue.agents[agent.vaultAddress()].firstTicketId
REF_2253(RedemptionQueue.State) -> state_1 (-> ['TMP_3455']).redemptionQueue
REF_2254(mapping(address => RedemptionQueue.AgentQueue)) -> REF_2253.agents
TMP_3457(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:["agent_1 (-> ['TMP_3454'])"] 
REF_2256(RedemptionQueue.AgentQueue) -> REF_2254[TMP_3457]
REF_2257(uint64) -> REF_2256.firstTicketId
ticketId_1(uint64) := REF_2257(uint64)
 i = 0
i_1(uint256) := 0(uint256)
 ticketId != 0 && i < maxRedeemedTickets
maxRedemptionAMG_2(uint64) := phi(['maxRedemptionAMG_3', 'maxRedemptionAMG_1'])
ticketId_2(uint64) := phi(['ticketId_3', 'ticketId_1'])
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_3458(bool) = ticketId_2 != 0
TMP_3459(bool) = i_2 < maxRedeemedTickets_1
TMP_3460(bool) = TMP_3458 && TMP_3459
CONDITION TMP_3460
 ticket = state.redemptionQueue.getTicket(ticketId)
REF_2258(RedemptionQueue.State) -> state_1 (-> ['TMP_3455']).redemptionQueue
TMP_3461(RedemptionQueue.Ticket) = LIBRARY_CALL, dest:RedemptionQueue, function:RedemptionQueue.getTicket(RedemptionQueue.State,uint64), arguments:['REF_2258', 'ticketId_2'] 
ticket_1 (-> ['TMP_3461'])(RedemptionQueue.Ticket) := TMP_3461(RedemptionQueue.Ticket)
 maxRedemptionAMG += ticket.valueAMG
REF_2260(uint64) -> ticket_1 (-> ['TMP_3461']).valueAMG
maxRedemptionAMG_3(uint64) = maxRedemptionAMG_2 (c)+ REF_2260
 ticketId = ticket.nextForAgent
REF_2261(uint64) -> ticket_1 (-> ['TMP_3461']).nextForAgent
ticketId_3(uint64) := REF_2261(uint64)
 i ++
TMP_3462(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 Conversion.convertAmgToUBA(maxRedemptionAMG)
TMP_3463(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['maxRedemptionAMG_2'] 
RETURN TMP_3463
```
#### RedemptionRequestsFacet.redeem(uint256,string,address) [EXTERNAL]
```slithir
 maxRedeemedTickets = Globals.getSettings().maxRedeemedTickets
TMP_3398(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_2212(uint16) -> TMP_3398.maxRedeemedTickets
maxRedeemedTickets_1(uint256) := REF_2212(uint16)
 redemptionList = RedemptionRequests.AgentRedemptionList({length:0,items:new RedemptionRequests.AgentRedemptionData[](maxRedeemedTickets)})
TMP_3400(RedemptionRequests.AgentRedemptionData[])  = new RedemptionRequests.AgentRedemptionData[](maxRedeemedTickets_1)
TMP_3401(RedemptionRequests.AgentRedemptionList) = new AgentRedemptionList(TMP_3400,0)
redemptionList_1(RedemptionRequests.AgentRedemptionList) := TMP_3401(RedemptionRequests.AgentRedemptionList)
 redeemedLots = 0
redeemedLots_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < maxRedeemedTickets && redeemedLots < _lots
redeemedLots_2(uint256) := phi(['redeemedLots_1', 'redeemedLots_3'])
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_3402(bool) = i_2 < maxRedeemedTickets_1
TMP_3403(bool) = redeemedLots_2 < _lots_1
TMP_3404(bool) = TMP_3402 && TMP_3403
CONDITION TMP_3404
 AssetManagerState.get().redemptionQueue.firstTicketId == 0
TMP_3405(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
REF_2215(RedemptionQueue.State) -> TMP_3405.redemptionQueue
REF_2216(uint64) -> REF_2215.firstTicketId
TMP_3406(bool) = REF_2216 == 0
CONDITION TMP_3406
 require(bool,error)(redeemedLots != 0,revert RedeemZeroLots()())
TMP_3407(bool) = redeemedLots_2 != 0
TMP_3408(None) = SOLIDITY_CALL revert RedeemZeroLots()()
TMP_3409(None) = SOLIDITY_CALL require(bool,error)(TMP_3407,TMP_3408)
 redeemedLots += _redeemFirstTicket(_lots - redeemedLots,redemptionList)
TMP_3410(uint256) = _lots_1 (c)- redeemedLots_2
TMP_3411(uint256) = INTERNAL_CALL, RedemptionRequestsFacet._redeemFirstTicket(uint256,RedemptionRequests.AgentRedemptionList)(TMP_3410,redemptionList_1)
redeemedLots_3(uint256) = redeemedLots_2 (c)+ TMP_3411
 i ++
TMP_3412(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 executorFeeNatGWei = msg.value / Conversion.GWEI
REF_2217(uint256) -> Conversion.GWEI
TMP_3413(uint256) = msg.value (c)/ REF_2217
executorFeeNatGWei_1(uint256) := TMP_3413(uint256)
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < redemptionList.length
executorFeeNatGWei_2(uint256) := phi(['executorFeeNatGWei_1', 'executorFeeNatGWei_3'])
i_scope_0_2(uint256) := phi(['i_scope_0_1', 'i_scope_0_3'])
REF_2218(uint256) -> redemptionList_1.length
TMP_3414(bool) = i_scope_0_2 < REF_2218
CONDITION TMP_3414
 currentExecutorFeeNatGWei = executorFeeNatGWei / (redemptionList.length - i_scope_0)
REF_2219(uint256) -> redemptionList_1.length
TMP_3415(uint256) = REF_2219 (c)- i_scope_0_2
TMP_3416(uint256) = executorFeeNatGWei_2 (c)/ TMP_3415
currentExecutorFeeNatGWei_1(uint256) := TMP_3416(uint256)
 executorFeeNatGWei -= currentExecutorFeeNatGWei
executorFeeNatGWei_3(uint256) = executorFeeNatGWei_2 (c)- currentExecutorFeeNatGWei_1
 RedemptionRequests.createRedemptionRequest(redemptionList.items[i_scope_0],msg.sender,_redeemerUnderlyingAddressString,false,_executor,currentExecutorFeeNatGWei.toUint64(),0,false)
REF_2221(RedemptionRequests.AgentRedemptionData[]) -> redemptionList_1.items
REF_2222(RedemptionRequests.AgentRedemptionData) -> REF_2221[i_scope_0_2]
TMP_3417(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['currentExecutorFeeNatGWei_1'] 
TMP_3418(uint64) = LIBRARY_CALL, dest:RedemptionRequests, function:RedemptionRequests.createRedemptionRequest(RedemptionRequests.AgentRedemptionData,address,string,bool,address,uint64,uint64,bool), arguments:['REF_2222', 'msg.sender', '_redeemerUnderlyingAddressString_1', 'False', '_executor_1', 'TMP_3417', '0', 'False'] 
 i_scope_0 ++
TMP_3419(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 (c)+ 1
 redeemedLots < _lots
TMP_3420(bool) = redeemedLots_2 < _lots_1
CONDITION TMP_3420
 IAssetManagerEvents.RedemptionRequestIncomplete(msg.sender,_lots - redeemedLots)
TMP_3421(uint256) = _lots_1 (c)- redeemedLots_2
Emit RedemptionRequestIncomplete(msg.sender,TMP_3421)
 redeemedUBA = Conversion.convertLotsToUBA(redeemedLots)
TMP_3423(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertLotsToUBA(uint256), arguments:['redeemedLots_2'] 
redeemedUBA_1(uint256) := TMP_3423(uint256)
 Redemptions.burnFAssets(msg.sender,redeemedUBA)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.burnFAssets(address,uint256), arguments:['msg.sender', 'redeemedUBA_1'] 
 redeemedUBA
RETURN redeemedUBA_1
 notEmergencyPaused()
MODIFIER_CALL, AssetManagerBase.notEmergencyPaused()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 _redeemedAmountUBA
```
#### RedemptionRequestsFacet.redeemFromAgent(address,address,uint256,string,address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_3427(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_3427'])(Agent.State) := TMP_3427(Agent.State)
 Agents.requireCollateralPool(agent)
LIBRARY_CALL, dest:Agents, function:Agents.requireCollateralPool(Agent.State), arguments:["agent_1 (-> ['TMP_3427'])"] 
 require(bool,error)(_amountUBA != 0,revert RedemptionOfZero()())
TMP_3429(bool) = _amountUBA_1 != 0
TMP_3430(None) = SOLIDITY_CALL revert RedemptionOfZero()()
TMP_3431(None) = SOLIDITY_CALL require(bool,error)(TMP_3429,TMP_3430)
 amountAMG = Conversion.convertUBAToAmg(_amountUBA)
TMP_3432(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['_amountUBA_1'] 
amountAMG_1(uint64) := TMP_3432(uint64)
 (closedAMG,closedUBA) = Redemptions.closeTickets(agent,amountAMG,false)
TUPLE_34(uint64,uint256) = LIBRARY_CALL, dest:Redemptions, function:Redemptions.closeTickets(Agent.State,uint64,bool), arguments:["agent_1 (-> ['TMP_3427'])", 'amountAMG_1', 'False'] 
closedAMG_1(uint64)= UNPACK TUPLE_34 index: 0 
closedUBA_1(uint256)= UNPACK TUPLE_34 index: 1 
 redemption = RedemptionRequests.AgentRedemptionData(_agentVault,closedAMG)
TMP_3433(RedemptionRequests.AgentRedemptionData) = new AgentRedemptionData(_agentVault_1,closedAMG_1)
redemption_1(RedemptionRequests.AgentRedemptionData) := TMP_3433(RedemptionRequests.AgentRedemptionData)
 RedemptionRequests.createRedemptionRequest(redemption,_receiver,_receiverUnderlyingAddress,true,_executor,(msg.value / Conversion.GWEI).toUint64(),0,false)
REF_2233(uint256) -> Conversion.GWEI
TMP_3434(uint256) = msg.value (c)/ REF_2233
TMP_3435(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_3434'] 
TMP_3436(uint64) = LIBRARY_CALL, dest:RedemptionRequests, function:RedemptionRequests.createRedemptionRequest(RedemptionRequests.AgentRedemptionData,address,string,bool,address,uint64,uint64,bool), arguments:['redemption_1', '_receiver_1', '_receiverUnderlyingAddress_1', 'True', '_executor_1', 'TMP_3435', '0', 'False'] 
 Redemptions.burnFAssets(msg.sender,closedUBA)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.burnFAssets(address,uint256), arguments:['msg.sender', 'closedUBA_1'] 
 notEmergencyPaused()
MODIFIER_CALL, AssetManagerBase.notEmergencyPaused()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### RedemptionRequestsFacet.redeemFromAgentInCollateral(address,address,uint256) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_3440(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_3440'])(Agent.State) := TMP_3440(Agent.State)
 Agents.requireCollateralPool(agent)
LIBRARY_CALL, dest:Agents, function:Agents.requireCollateralPool(Agent.State), arguments:["agent_1 (-> ['TMP_3440'])"] 
 require(bool,error)(_amountUBA != 0,revert RedemptionOfZero()())
TMP_3442(bool) = _amountUBA_1 != 0
TMP_3443(None) = SOLIDITY_CALL revert RedemptionOfZero()()
TMP_3444(None) = SOLIDITY_CALL require(bool,error)(TMP_3442,TMP_3443)
 amountAMG = Conversion.convertUBAToAmg(_amountUBA)
TMP_3445(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['_amountUBA_1'] 
amountAMG_1(uint64) := TMP_3445(uint64)
 (closedAMG,closedUBA) = Redemptions.closeTickets(agent,amountAMG,true)
TUPLE_35(uint64,uint256) = LIBRARY_CALL, dest:Redemptions, function:Redemptions.closeTickets(Agent.State,uint64,bool), arguments:["agent_1 (-> ['TMP_3440'])", 'amountAMG_1', 'True'] 
closedAMG_1(uint64)= UNPACK TUPLE_35 index: 0 
closedUBA_1(uint256)= UNPACK TUPLE_35 index: 1 
 priceAmgToWei = Conversion.currentAmgPriceInTokenWei(agent.vaultCollateralIndex)
REF_2241(uint16) -> agent_1 (-> ['TMP_3440']).vaultCollateralIndex
TMP_3446(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.currentAmgPriceInTokenWei(uint256), arguments:['REF_2241'] 
priceAmgToWei_1(uint256) := TMP_3446(uint256)
 paymentWei = Conversion.convertAmgToTokenWei(closedAMG,priceAmgToWei).mulBips(agent.buyFAssetByAgentFactorBIPS)
TMP_3447(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToTokenWei(uint256,uint256), arguments:['closedAMG_1', 'priceAmgToWei_1'] 
REF_2244(uint16) -> agent_1 (-> ['TMP_3440']).buyFAssetByAgentFactorBIPS
TMP_3448(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_3447', 'REF_2244'] 
paymentWei_1(uint256) := TMP_3448(uint256)
 AgentPayout.payoutFromVault(agent,_receiver,paymentWei)
TMP_3449(uint256) = LIBRARY_CALL, dest:AgentPayout, function:AgentPayout.payoutFromVault(Agent.State,address,uint256), arguments:["agent_1 (-> ['TMP_3440'])", '_receiver_1', 'paymentWei_1'] 
 IAssetManagerEvents.RedeemedInCollateral(_agentVault,_receiver,closedUBA,paymentWei)
Emit RedeemedInCollateral(_agentVault_1,_receiver_1,closedUBA_1,paymentWei_1)
 Redemptions.burnFAssets(msg.sender,closedUBA)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.burnFAssets(address,uint256), arguments:['msg.sender', 'closedUBA_1'] 
 notEmergencyPaused()
MODIFIER_CALL, AssetManagerBase.notEmergencyPaused()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### RedemptionRequestsFacet.rejectInvalidRedemption(IAddressValidity.Proof,uint256) [EXTERNAL]
```slithir
 request = Redemptions.getRedemptionRequest(_redemptionRequestId,true)
TMP_3464(Redemption.Request) = LIBRARY_CALL, dest:Redemptions, function:Redemptions.getRedemptionRequest(uint256,bool), arguments:['_redemptionRequestId_1', 'True'] 
request_1 (-> ['TMP_3464'])(Redemption.Request) := TMP_3464(Redemption.Request)
 assert(bool)(! request.transferToCoreVault)
REF_2264(bool) -> request_1 (-> ['TMP_3464']).transferToCoreVault
TMP_3465 = UnaryType.BANG REF_2264 
TMP_3466(None) = SOLIDITY_CALL assert(bool)(TMP_3465)
 agent = Agent.get(request.agentVault)
REF_2266(address) -> request_1 (-> ['TMP_3464']).agentVault
TMP_3467(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_2266'] 
agent_1 (-> ['TMP_3467'])(Agent.State) := TMP_3467(Agent.State)
 require(bool,error)(request.status == Redemption.Status.ACTIVE,revert InvalidRedemptionStatus()())
REF_2267(Redemption.Status) -> request_1 (-> ['TMP_3464']).status
REF_2268(Redemption.Status) -> Status.ACTIVE
TMP_3468(bool) = REF_2267 == REF_2268
TMP_3469(None) = SOLIDITY_CALL revert InvalidRedemptionStatus()()
TMP_3470(None) = SOLIDITY_CALL require(bool,error)(TMP_3468,TMP_3469)
 Agents.requireAgentVaultOwner(agent)
LIBRARY_CALL, dest:Agents, function:Agents.requireAgentVaultOwner(Agent.State), arguments:["agent_1 (-> ['TMP_3467'])"] 
 TransactionAttestation.verifyAddressValidity(_proof)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyAddressValidity(IAddressValidity.Proof), arguments:['_proof_1'] 
 addressHash = keccak256(bytes)(bytes(_proof.data.requestBody.addressStr))
REF_2271(IAddressValidity.Response) -> _proof_1.data
REF_2272(IAddressValidity.RequestBody) -> REF_2271.requestBody
REF_2273(string) -> REF_2272.addressStr
TMP_3473 = CONVERT REF_2273 to bytes
TMP_3474(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_3473)
addressHash_1(bytes32) := TMP_3474(bytes32)
 require(bool,error)(addressHash == request.redeemerUnderlyingAddressHash,revert WrongAddress()())
REF_2274(bytes32) -> request_1 (-> ['TMP_3464']).redeemerUnderlyingAddressHash
TMP_3475(bool) = addressHash_1 == REF_2274
TMP_3476(None) = SOLIDITY_CALL revert WrongAddress()()
TMP_3477(None) = SOLIDITY_CALL require(bool,error)(TMP_3475,TMP_3476)
 valid = _proof.data.responseBody.isValid && _proof.data.responseBody.standardAddressHash == request.redeemerUnderlyingAddressHash
REF_2275(IAddressValidity.Response) -> _proof_1.data
REF_2276(IAddressValidity.ResponseBody) -> REF_2275.responseBody
REF_2277(bool) -> REF_2276.isValid
REF_2278(IAddressValidity.Response) -> _proof_1.data
REF_2279(IAddressValidity.ResponseBody) -> REF_2278.responseBody
REF_2280(bytes32) -> REF_2279.standardAddressHash
REF_2281(bytes32) -> request_1 (-> ['TMP_3464']).redeemerUnderlyingAddressHash
TMP_3478(bool) = REF_2280 == REF_2281
TMP_3479(bool) = REF_2277 && TMP_3478
valid_1(bool) := TMP_3479(bool)
 require(bool,error)(! valid,revert AddressValid()())
TMP_3480 = UnaryType.BANG valid_1 
TMP_3481(None) = SOLIDITY_CALL revert AddressValid()()
TMP_3482(None) = SOLIDITY_CALL require(bool,error)(TMP_3480,TMP_3481)
 AgentBacking.endRedeemingAssets(agent,request.valueAMG,request.poolSelfClose)
REF_2283(uint64) -> request_1 (-> ['TMP_3464']).valueAMG
REF_2284(bool) -> request_1 (-> ['TMP_3464']).poolSelfClose
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.endRedeemingAssets(Agent.State,uint64,bool), arguments:["agent_1 (-> ['TMP_3467'])", 'REF_2283', 'REF_2284'] 
 Redemptions.burnExecutorFee(request)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.burnExecutorFee(Redemption.Request), arguments:["request_1 (-> ['TMP_3464'])"] 
 valueUBA = Conversion.convertAmgToUBA(request.valueAMG)
REF_2287(uint64) -> request_1 (-> ['TMP_3464']).valueAMG
TMP_3485(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_2287'] 
valueUBA_1(uint256) := TMP_3485(uint256)
 IAssetManagerEvents.RedemptionRejected(request.agentVault,request.redeemer,_redemptionRequestId,valueUBA)
REF_2289(address) -> request_1 (-> ['TMP_3464']).agentVault
REF_2290(address) -> request_1 (-> ['TMP_3464']).redeemer
Emit RedemptionRejected(REF_2289,REF_2290,_redemptionRequestId_1,valueUBA_1)
 Redemptions.finishRedemptionRequest(_redemptionRequestId,request,Redemption.Status.REJECTED)
REF_2292(Redemption.Status) -> Status.REJECTED
LIBRARY_CALL, dest:Redemptions, function:Redemptions.finishRedemptionRequest(uint256,Redemption.Request,Redemption.Status), arguments:['_redemptionRequestId_1', "request_1 (-> ['TMP_3464'])", 'REF_2292'] 
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### RedemptionRequestsFacet.selfClose(address,uint256) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_3489(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_3489'])(Agent.State) := TMP_3489(Agent.State)
 require(bool,error)(_amountUBA != 0,revert SelfCloseOfZero()())
TMP_3490(bool) = _amountUBA_1 != 0
TMP_3491(None) = SOLIDITY_CALL revert SelfCloseOfZero()()
TMP_3492(None) = SOLIDITY_CALL require(bool,error)(TMP_3490,TMP_3491)
 amountAMG = Conversion.convertUBAToAmg(_amountUBA)
TMP_3493(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['_amountUBA_1'] 
amountAMG_1(uint64) := TMP_3493(uint64)
 (None,closedUBA) = Redemptions.closeTickets(agent,amountAMG,true)
TUPLE_36(uint64,uint256) = LIBRARY_CALL, dest:Redemptions, function:Redemptions.closeTickets(Agent.State,uint64,bool), arguments:["agent_1 (-> ['TMP_3489'])", 'amountAMG_1', 'True'] 
closedUBA_1(uint256)= UNPACK TUPLE_36 index: 1 
 Redemptions.burnFAssets(msg.sender,closedUBA)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.burnFAssets(address,uint256), arguments:['msg.sender', 'closedUBA_1'] 
 Liquidation.endLiquidationIfHealthy(agent)
LIBRARY_CALL, dest:Liquidation, function:Liquidation.endLiquidationIfHealthy(Agent.State), arguments:["agent_1 (-> ['TMP_3489'])"] 
 IAssetManagerEvents.SelfClose(_agentVault,closedUBA)
Emit SelfClose(_agentVault_1,closedUBA_1)
 closedUBA
RETURN closedUBA_1
 notEmergencyPaused()
MODIFIER_CALL, AssetManagerBase.notEmergencyPaused()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
 _closedAmountUBA
```
#### Conversion.convertLotsToAMG(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4646(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4646'])(AssetManagerSettings.Data) := TMP_4646(AssetManagerSettings.Data)
 SafeCast.toUint64(_lots * settings.lotSizeAMG)
REF_3153(uint64) -> settings_1 (-> ['TMP_4646']).lotSizeAMG
TMP_4647(uint256) = _lots_1 (c)* REF_3153
TMP_4648(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_4647'] 
RETURN TMP_4648
```
#### Globals.getSettings() [INTERNAL]
```slithir
ASSET_MANAGER_SETTINGS_POSITION_1(bytes32) := phi(['ASSET_MANAGER_SETTINGS_POSITION_0'])
 position = ASSET_MANAGER_SETTINGS_POSITION
position_1(bytes32) := ASSET_MANAGER_SETTINGS_POSITION_1(bytes32)
 _settings = position
_settings_1 (-> ['position'])(AssetManagerSettings.Data) := position_1(bytes32)
 _settings
RETURN _settings_1 (-> ['position'])
```
#### Redemptions.removeFromTicket(uint64,uint64) [INTERNAL]
```slithir
_redemptionTicketId_1(uint64) := phi(['ticketId_1'])
_redeemedAMG_1(uint64) := phi(['ticketRedeemAMG_1'])
 redemptionQueue = AssetManagerState.get().redemptionQueue
TMP_5005(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
REF_3540(RedemptionQueue.State) -> TMP_5005.redemptionQueue
redemptionQueue_1 (-> ['TMP_5005'])(RedemptionQueue.State) := REF_3540(RedemptionQueue.State)
 ticket = redemptionQueue.getTicket(_redemptionTicketId)
TMP_5006(RedemptionQueue.Ticket) = LIBRARY_CALL, dest:RedemptionQueue, function:RedemptionQueue.getTicket(RedemptionQueue.State,uint64), arguments:["redemptionQueue_1 (-> ['TMP_5005'])", '_redemptionTicketId_1'] 
ticket_1 (-> ['TMP_5006'])(RedemptionQueue.Ticket) := TMP_5006(RedemptionQueue.Ticket)
 agent = Agent.get(ticket.agentVault)
REF_3543(address) -> ticket_1 (-> ['TMP_5006']).agentVault
TMP_5007(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_3543'] 
agent_1 (-> ['TMP_5007'])(Agent.State) := TMP_5007(Agent.State)
 lotSize = Globals.getSettings().lotSizeAMG
TMP_5008(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_3545(uint64) -> TMP_5008.lotSizeAMG
lotSize_1(uint64) := REF_3545(uint64)
 remainingAMG = ticket.valueAMG + agent.dustAMG - _redeemedAMG
REF_3546(uint64) -> ticket_1 (-> ['TMP_5006']).valueAMG
REF_3547(uint64) -> agent_1 (-> ['TMP_5007']).dustAMG
TMP_5009(uint64) = REF_3546 (c)+ REF_3547
TMP_5010(uint64) = TMP_5009 (c)- _redeemedAMG_1
remainingAMG_1(uint64) := TMP_5010(uint64)
 remainingAMGDust = remainingAMG % lotSize
TMP_5011(uint64) = remainingAMG_1 % lotSize_1
remainingAMGDust_1(uint64) := TMP_5011(uint64)
 remainingAMGLots = remainingAMG - remainingAMGDust
TMP_5012(uint64) = remainingAMG_1 (c)- remainingAMGDust_1
remainingAMGLots_1(uint64) := TMP_5012(uint64)
 remainingAMGLots == 0
TMP_5013(bool) = remainingAMGLots_1 == 0
CONDITION TMP_5013
 redemptionQueue.deleteRedemptionTicket(_redemptionTicketId)
LIBRARY_CALL, dest:RedemptionQueue, function:RedemptionQueue.deleteRedemptionTicket(RedemptionQueue.State,uint64), arguments:["redemptionQueue_1 (-> ['TMP_5005'])", '_redemptionTicketId_1'] 
 IAssetManagerEvents.RedemptionTicketDeleted(agent.vaultAddress(),_redemptionTicketId)
TMP_5015(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:["agent_1 (-> ['TMP_5007'])"] 
Emit RedemptionTicketDeleted(TMP_5015,_redemptionTicketId_1)
 remainingAMGLots != ticket.valueAMG
REF_3551(uint64) -> ticket_1 (-> ['TMP_5006']).valueAMG
TMP_5017(bool) = remainingAMGLots_1 != REF_3551
CONDITION TMP_5017
 ticket.valueAMG = remainingAMGLots
REF_3552(uint64) -> ticket_1 (-> ['TMP_5006']).valueAMG
ticket_2 (-> ['TMP_5006'])(RedemptionQueue.Ticket) := phi(["ticket_1 (-> ['TMP_5006'])"])
REF_3552(uint64) (->ticket_2 (-> ['TMP_5006'])) := remainingAMGLots_1(uint64)
TMP_5006(RedemptionQueue.Ticket) := phi(["ticket_2 (-> ['TMP_5006'])"])
 remainingUBA = Conversion.convertAmgToUBA(remainingAMGLots)
TMP_5018(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['remainingAMGLots_1'] 
remainingUBA_1(uint256) := TMP_5018(uint256)
 IAssetManagerEvents.RedemptionTicketUpdated(agent.vaultAddress(),_redemptionTicketId,remainingUBA)
TMP_5019(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:["agent_1 (-> ['TMP_5007'])"] 
Emit RedemptionTicketUpdated(TMP_5019,_redemptionTicketId_1,remainingUBA_1)
 AgentBacking.changeDust(agent,remainingAMGDust)
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.changeDust(Agent.State,uint64), arguments:["agent_1 (-> ['TMP_5007'])", 'remainingAMGDust_1']
```
#### Agent.get(address) [INTERNAL]
```slithir
 agent = getWithoutCheck(_address)
TMP_5309(Agent.State) = INTERNAL_CALL, Agent.getWithoutCheck(address)(_address_1)
agent_1 (-> ['TMP_5309'])(Agent.State) := TMP_5309(Agent.State)
 status = agent.status
REF_3748(Agent.Status) -> agent_1 (-> ['TMP_5309']).status
status_1(Agent.Status) := REF_3748(Agent.Status)
 require(bool,error)(status != Agent.Status.EMPTY && status != Agent.Status.DESTROYED,revert InvalidAgentVaultAddress()())
REF_3749(Agent.Status) -> Status.EMPTY
TMP_5310(bool) = status_1 != REF_3749
REF_3750(Agent.Status) -> Status.DESTROYED
TMP_5311(bool) = status_1 != REF_3750
TMP_5312(bool) = TMP_5310 && TMP_5311
TMP_5313(None) = SOLIDITY_CALL revert InvalidAgentVaultAddress()()
TMP_5314(None) = SOLIDITY_CALL require(bool,error)(TMP_5312,TMP_5313)
 agent
RETURN agent_1 (-> ['TMP_5309'])
```
#### AssetManagerState.get() [INTERNAL]
```slithir
STATE_POSITION_1(bytes32) := phi(['STATE_POSITION_0'])
 position = STATE_POSITION
position_1(bytes32) := STATE_POSITION_1(bytes32)
 _state = position
_state_1 (-> ['position'])(AssetManagerState.State) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
```

#### Math.min(uint256,uint256) [INTERNAL]
```slithir
a_1(uint256) := phi(['result_8'])
b_1(uint256) := phi(['TMP_554'])
 a < b
TMP_477(bool) = a_1 < b_1
CONDITION TMP_477
 a
RETURN a_1
 b
RETURN b_1
```
#### AgentBacking.changeDust(Agent.State,uint64) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_2 (-> [])'])
_newDustAMG_1(uint64) := phi(['newDustAMG_1', 'newDustAMG_1'])
 _agent.dustAMG == _newDustAMG
REF_2808(uint64) -> _agent_1 (-> []).dustAMG
TMP_4290(bool) = REF_2808 == _newDustAMG_1
CONDITION TMP_4290
 _agent.dustAMG = _newDustAMG
REF_2809(uint64) -> _agent_1 (-> []).dustAMG
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2809(uint64) (->_agent_2 (-> [])) := _newDustAMG_1(uint64)
 dustUBA = Conversion.convertAmgToUBA(_newDustAMG)
TMP_4291(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['_newDustAMG_1'] 
dustUBA_1(uint256) := TMP_4291(uint256)
 IAssetManagerEvents.DustChanged(_agent.vaultAddress(),dustUBA)
TMP_4292(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_2 (-> [])'] 
Emit DustChanged(TMP_4292,dustUBA_1)
```
#### AgentBacking.createRedemptionTicket(Agent.State,uint64) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_2 (-> [])'])
_ticketValueAMG_1(uint64) := phi(['ticketValueAMG_1'])
 state = AssetManagerState.get()
TMP_4280(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4280'])(AssetManagerState.State) := TMP_4280(AssetManagerState.State)
 _ticketValueAMG == 0
TMP_4281(bool) = _ticketValueAMG_1 == 0
CONDITION TMP_4281
 vaultAddress = _agent.vaultAddress()
TMP_4282(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
vaultAddress_1(address) := TMP_4282(address)
 lastTicketId = state.redemptionQueue.lastTicketId
REF_2795(RedemptionQueue.State) -> state_1 (-> ['TMP_4280']).redemptionQueue
REF_2796(uint64) -> REF_2795.lastTicketId
lastTicketId_1(uint64) := REF_2796(uint64)
 lastTicket = state.redemptionQueue.getTicket(lastTicketId)
REF_2797(RedemptionQueue.State) -> state_1 (-> ['TMP_4280']).redemptionQueue
TMP_4283(RedemptionQueue.Ticket) = LIBRARY_CALL, dest:RedemptionQueue, function:RedemptionQueue.getTicket(RedemptionQueue.State,uint64), arguments:['REF_2797', 'lastTicketId_1'] 
lastTicket_1 (-> ['TMP_4283'])(RedemptionQueue.Ticket) := TMP_4283(RedemptionQueue.Ticket)
 lastTicket.agentVault == vaultAddress
REF_2799(address) -> lastTicket_1 (-> ['TMP_4283']).agentVault
TMP_4284(bool) = REF_2799 == vaultAddress_1
CONDITION TMP_4284
 lastTicket.valueAMG += _ticketValueAMG
REF_2800(uint64) -> lastTicket_1 (-> ['TMP_4283']).valueAMG
lastTicket_2 (-> ['TMP_4283'])(RedemptionQueue.Ticket) := phi(["lastTicket_1 (-> ['TMP_4283'])"])
REF_2800(-> lastTicket_2 (-> ['TMP_4283'])) = REF_2800 (c)+ _ticketValueAMG_1
TMP_4283(RedemptionQueue.Ticket) := phi(["lastTicket_2 (-> ['TMP_4283'])"])
 ticketValueUBA = Conversion.convertAmgToUBA(lastTicket.valueAMG)
REF_2802(uint64) -> lastTicket_2 (-> ['TMP_4283']).valueAMG
TMP_4285(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_2802'] 
ticketValueUBA_1(uint256) := TMP_4285(uint256)
 IAssetManagerEvents.RedemptionTicketUpdated(vaultAddress,lastTicketId,ticketValueUBA)
Emit RedemptionTicketUpdated(vaultAddress_1,lastTicketId_1,ticketValueUBA_1)
 ticketId = state.redemptionQueue.createRedemptionTicket(vaultAddress,_ticketValueAMG)
REF_2804(RedemptionQueue.State) -> state_1 (-> ['TMP_4280']).redemptionQueue
TMP_4287(uint64) = LIBRARY_CALL, dest:RedemptionQueue, function:RedemptionQueue.createRedemptionTicket(RedemptionQueue.State,address,uint64), arguments:['REF_2804', 'vaultAddress_1', '_ticketValueAMG_1'] 
ticketId_1(uint64) := TMP_4287(uint64)
 ticketValueUBA_scope_0 = Conversion.convertAmgToUBA(_ticketValueAMG)
TMP_4288(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['_ticketValueAMG_1'] 
ticketValueUBA_scope_0_1(uint256) := TMP_4288(uint256)
 IAssetManagerEvents.RedemptionTicketCreated(vaultAddress,ticketId,ticketValueUBA_scope_0)
Emit RedemptionTicketCreated(vaultAddress_1,ticketId_1,ticketValueUBA_scope_0_1)
```
#### Conversion.convertAmgToUBA(uint64) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4637(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4637'])(AssetManagerSettings.Data) := TMP_4637(AssetManagerSettings.Data)
 uint256(_valueAMG) * settings.assetMintingGranularityUBA
TMP_4638 = CONVERT _valueAMG_1 to uint256
REF_3145(uint64) -> settings_1 (-> ['TMP_4637']).assetMintingGranularityUBA
TMP_4639(uint256) = TMP_4638 (c)* REF_3145
RETURN TMP_4639
```
#### Agent.vaultAddress(Agent.State) [INTERNAL]
```slithir
AGENTS_POSITION_2(bytes32) := phi(['AGENTS_POSITION_0'])
 position = _agent
position_1(bytes32) := _agent_1 (-> [])(Agent.State)
 address(uint160((uint256(position) ^ uint256(AGENTS_POSITION)) >> 64))
TMP_5325 = CONVERT position_1 to uint256
TMP_5326 = CONVERT AGENTS_POSITION_2 to uint256
TMP_5327(uint256) = TMP_5325 ^ TMP_5326
TMP_5328(uint256) = TMP_5327 >> 64
TMP_5329 = CONVERT TMP_5328 to uint160
TMP_5330 = CONVERT TMP_5329 to address
RETURN TMP_5330
```
#### Conversion.convertLotsToUBA(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4649(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4649'])(AssetManagerSettings.Data) := TMP_4649(AssetManagerSettings.Data)
 _lots * settings.lotSizeAMG * settings.assetMintingGranularityUBA
REF_3155(uint64) -> settings_1 (-> ['TMP_4649']).lotSizeAMG
TMP_4650(uint256) = _lots_1 (c)* REF_3155
REF_3156(uint64) -> settings_1 (-> ['TMP_4649']).assetMintingGranularityUBA
TMP_4651(uint256) = TMP_4650 (c)* REF_3156
RETURN TMP_4651
```
#### RedemptionRequests.createRedemptionRequest(RedemptionRequests.AgentRedemptionData,address,string,bool,address,uint64,uint64,bool) [INTERNAL]
```slithir
 require(bool,error)(_executorFeeNatGWei == 0 || _executor != address(0),revert ExecutorFeeWithoutExecutor()())
TMP_4926(bool) = _executorFeeNatGWei_1 == 0
TMP_4927 = CONVERT 0 to address
TMP_4928(bool) = _executor_1 != TMP_4927
TMP_4929(bool) = TMP_4926 || TMP_4928
TMP_4930(None) = SOLIDITY_CALL revert ExecutorFeeWithoutExecutor()()
TMP_4931(None) = SOLIDITY_CALL require(bool,error)(TMP_4929,TMP_4930)
 state = AssetManagerState.get()
TMP_4932(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4932'])(AssetManagerState.State) := TMP_4932(AssetManagerState.State)
 agent = Agent.get(_data.agentVault)
REF_3453(address) -> _data_1.agentVault
TMP_4933(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_3453'] 
agent_1 (-> ['TMP_4933'])(Agent.State) := TMP_4933(Agent.State)
 require(bool,error)(bytes(_redeemerUnderlyingAddressString).length < 128,revert UnderlyingAddressTooLong()())
TMP_4934 = CONVERT _redeemerUnderlyingAddressString_1 to bytes
REF_3454 -> LENGTH TMP_4934
TMP_4935(bool) = REF_3454 < 128
TMP_4936(None) = SOLIDITY_CALL revert UnderlyingAddressTooLong()()
TMP_4937(None) = SOLIDITY_CALL require(bool,error)(TMP_4935,TMP_4936)
 underlyingAddressHash = keccak256(bytes)(bytes(_redeemerUnderlyingAddressString))
TMP_4938 = CONVERT _redeemerUnderlyingAddressString_1 to bytes
TMP_4939(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_4938)
underlyingAddressHash_1(bytes32) := TMP_4939(bytes32)
 require(bool,error)(underlyingAddressHash != agent.underlyingAddressHash,revert CannotRedeemToAgentsAddress()())
REF_3455(bytes32) -> agent_1 (-> ['TMP_4933']).underlyingAddressHash
TMP_4940(bool) = underlyingAddressHash_1 != REF_3455
TMP_4941(None) = SOLIDITY_CALL revert CannotRedeemToAgentsAddress()()
TMP_4942(None) = SOLIDITY_CALL require(bool,error)(TMP_4940,TMP_4941)
 redeemedValueUBA = Conversion.convertAmgToUBA(_data.valueAMG).toUint128()
REF_3457(uint64) -> _data_1.valueAMG
TMP_4943(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_3457'] 
TMP_4944(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['TMP_4943'] 
redeemedValueUBA_1(uint128) := TMP_4944(uint128)
 _requestId = _newRequestId(_poolSelfClose)
TMP_4945(uint64) = INTERNAL_CALL, RedemptionRequests._newRequestId(bool)(_poolSelfClose_1)
_requestId_1(uint64) := TMP_4945(uint64)
 request.redeemerUnderlyingAddressHash = underlyingAddressHash
REF_3459(bytes32) -> request_0.redeemerUnderlyingAddressHash
request_1(Redemption.Request) := phi(['request_0'])
REF_3459(bytes32) (->request_1) := underlyingAddressHash_1(bytes32)
 request.underlyingValueUBA = redeemedValueUBA
REF_3460(uint128) -> request_1.underlyingValueUBA
request_2(Redemption.Request) := phi(['request_1'])
REF_3460(uint128) (->request_2) := redeemedValueUBA_1(uint128)
 request.firstUnderlyingBlock = state.currentUnderlyingBlock
REF_3461(uint64) -> request_2.firstUnderlyingBlock
REF_3462(uint64) -> state_1 (-> ['TMP_4932']).currentUnderlyingBlock
request_3(Redemption.Request) := phi(['request_2'])
REF_3461(uint64) (->request_3) := REF_3462(uint64)
 (request.lastUnderlyingBlock,request.lastUnderlyingTimestamp) = _lastPaymentBlock(_data.agentVault,_additionalPaymentTime)
REF_3463(uint64) -> request_3.lastUnderlyingBlock
REF_3464(uint64) -> request_3.lastUnderlyingTimestamp
REF_3465(address) -> _data_1.agentVault
TUPLE_63(uint64,uint64) = INTERNAL_CALL, RedemptionRequests._lastPaymentBlock(address,uint64)(REF_3465,_additionalPaymentTime_1)
REF_3463(uint64)= UNPACK TUPLE_63 index: 0 
REF_3464(uint64)= UNPACK TUPLE_63 index: 1 
 request.timestamp = block.timestamp.toUint64()
REF_3466(uint64) -> request_3.timestamp
TMP_4946(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.timestamp'] 
request_4(Redemption.Request) := phi(['request_3'])
REF_3466(uint64) (->request_4) := TMP_4946(uint64)
 request.redeemer = _redeemer
REF_3468(address) -> request_7.redeemer
request_8(Redemption.Request) := phi(['request_7'])
REF_3468(address) (->request_8) := _redeemer_1(address)
 request.agentVault = _data.agentVault
REF_3469(address) -> request_8.agentVault
REF_3470(address) -> _data_1.agentVault
request_9(Redemption.Request) := phi(['request_8'])
REF_3469(address) (->request_9) := REF_3470(address)
 request.valueAMG = _data.valueAMG
REF_3471(uint64) -> request_9.valueAMG
REF_3472(uint64) -> _data_1.valueAMG
request_10(Redemption.Request) := phi(['request_9'])
REF_3471(uint64) (->request_10) := REF_3472(uint64)
 request.status = Redemption.Status.ACTIVE
REF_3473(Redemption.Status) -> request_10.status
REF_3474(Redemption.Status) -> Status.ACTIVE
request_11(Redemption.Request) := phi(['request_10'])
REF_3473(Redemption.Status) (->request_11) := REF_3474(Redemption.Status)
 request.poolSelfClose = _poolSelfClose
REF_3475(bool) -> request_11.poolSelfClose
request_12(Redemption.Request) := phi(['request_11'])
REF_3475(bool) (->request_12) := _poolSelfClose_1(bool)
 request.executor = _executor
REF_3476(address) -> request_12.executor
request_13(Redemption.Request) := phi(['request_12'])
REF_3476(address) (->request_13) := _executor_1(address)
 request.executorFeeNatGWei = _executorFeeNatGWei
REF_3477(uint64) -> request_13.executorFeeNatGWei
request_14(Redemption.Request) := phi(['request_13'])
REF_3477(uint64) (->request_14) := _executorFeeNatGWei_1(uint64)
 request.redeemerUnderlyingAddressString = _redeemerUnderlyingAddressString
REF_3478(string) -> request_14.redeemerUnderlyingAddressString
request_15(Redemption.Request) := phi(['request_14'])
REF_3478(string) (->request_15) := _redeemerUnderlyingAddressString_1(string)
 request.transferToCoreVault = _transferToCoreVault
REF_3479(bool) -> request_15.transferToCoreVault
request_16(Redemption.Request) := phi(['request_15'])
REF_3479(bool) (->request_16) := _transferToCoreVault_1(bool)
 request.poolFeeShareBIPS = agent.redemptionPoolFeeShareBIPS
REF_3480(uint16) -> request_16.poolFeeShareBIPS
REF_3481(uint16) -> agent_1 (-> ['TMP_4933']).redemptionPoolFeeShareBIPS
request_17(Redemption.Request) := phi(['request_16'])
REF_3480(uint16) (->request_17) := REF_3481(uint16)
 state.redemptionRequests[_requestId] = request
REF_3482(mapping(uint256 => Redemption.Request)) -> state_1 (-> ['TMP_4932']).redemptionRequests
REF_3483(Redemption.Request) -> REF_3482[_requestId_1]
state_2 (-> ['TMP_4932'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_4932'])"])
REF_3483(Redemption.Request) (->state_2 (-> ['TMP_4932'])) := request_17(Redemption.Request)
TMP_4932(AssetManagerState.State) := phi(["state_2 (-> ['TMP_4932'])"])
 AgentBacking.startRedeemingAssets(agent,_data.valueAMG,_poolSelfClose)
REF_3485(uint64) -> _data_1.valueAMG
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.startRedeemingAssets(Agent.State,uint64,bool), arguments:["agent_1 (-> ['TMP_4933'])", 'REF_3485', '_poolSelfClose_1'] 
 _emitRedemptionRequestedEvent(request,_requestId,_redeemerUnderlyingAddressString)
INTERNAL_CALL, RedemptionRequests._emitRedemptionRequestedEvent(Redemption.Request,uint64,string)(request_17,_requestId_1,_redeemerUnderlyingAddressString_1)
 _transferToCoreVault
CONDITION _transferToCoreVault_1
 request.underlyingFeeUBA = 0
REF_3486(uint128) -> request_4.underlyingFeeUBA
request_6(Redemption.Request) := phi(['request_4'])
REF_3486(uint128) (->request_6) := 0(uint256)
 request.underlyingFeeUBA = uint256(redeemedValueUBA).mulBips(Globals.getSettings().redemptionFeeBIPS).toUint128()
REF_3487(uint128) -> request_4.underlyingFeeUBA
TMP_4949 = CONVERT redeemedValueUBA_1 to uint256
TMP_4950(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_3490(uint16) -> TMP_4950.redemptionFeeBIPS
TMP_4951(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_4949', 'REF_3490'] 
TMP_4952(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['TMP_4951'] 
request_5(Redemption.Request) := phi(['request_4'])
REF_3487(uint128) (->request_5) := TMP_4952(uint128)
request_7(Redemption.Request) := phi(['request_5', 'request_6'])
 _requestId
RETURN _requestId_1
```
#### Redemptions.burnFAssets(address,uint256) [INTERNAL]
```slithir
 Globals.getFAsset().burn(_owner,_amountUBA)
TMP_5022(IIFAsset) = LIBRARY_CALL, dest:Globals, function:Globals.getFAsset(), arguments:[] 
HIGH_LEVEL_CALL, dest:TMP_5022(IIFAsset), function:burn, arguments:['_owner_1', '_amountUBA_1']
```
#### SafeCast.toUint64(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint64).max,SafeCast: value doesn't fit in 64 bits)
TMP_747(uint64) := 18446744073709551615(uint64)
TMP_748(bool) = value_1 <= TMP_747
TMP_749(None) = SOLIDITY_CALL require(bool,string)(TMP_748,SafeCast: value doesn't fit in 64 bits)
 uint64(value)
TMP_750 = CONVERT value_1 to uint64
RETURN TMP_750
```
#### Agents.requireCollateralPool(Agent.State) [INTERNAL]
```slithir
 require(bool,error)(msg.sender == address(_agent.collateralPool),revert OnlyCollateralPool()())
REF_2998(IICollateralPool) -> _agent_1 (-> []).collateralPool
TMP_4500 = CONVERT REF_2998 to address
TMP_4501(bool) = msg.sender == TMP_4500
TMP_4502(None) = SOLIDITY_CALL revert OnlyCollateralPool()()
TMP_4503(None) = SOLIDITY_CALL require(bool,error)(TMP_4501,TMP_4502)
```
#### Conversion.convertUBAToAmg(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4640(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4640'])(AssetManagerSettings.Data) := TMP_4640(AssetManagerSettings.Data)
 SafeCast.toUint64(_valueUBA / settings.assetMintingGranularityUBA)
REF_3148(uint64) -> settings_1 (-> ['TMP_4640']).assetMintingGranularityUBA
TMP_4641(uint256) = _valueUBA_1 (c)/ REF_3148
TMP_4642(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_4641'] 
RETURN TMP_4642
```
#### Redemptions.closeTickets(Agent.State,uint64,bool) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4984(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4984'])(AssetManagerState.State) := TMP_4984(AssetManagerState.State)
 maxRedeemedTickets = Globals.getSettings().maxRedeemedTickets
TMP_4985(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_3521(uint16) -> TMP_4985.maxRedeemedTickets
maxRedeemedTickets_1(uint256) := REF_3521(uint16)
 lotSize = Globals.getSettings().lotSizeAMG
TMP_4986(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_3523(uint64) -> TMP_4986.lotSizeAMG
lotSize_1(uint64) := REF_3523(uint64)
 i = 0
i_1(uint256) := 0(uint256)
 i < maxRedeemedTickets && _closedAMG < _amountAMG
_closedAMG_1(uint64) := phi(['_closedAMG_0', '_closedAMG_2'])
i_2(uint256) := phi(['i_3', 'i_1'])
TMP_4987(bool) = i_2 < maxRedeemedTickets_1
TMP_4988(bool) = _closedAMG_1 < _amountAMG_1
TMP_4989(bool) = TMP_4987 && TMP_4988
CONDITION TMP_4989
 ticketId = state.redemptionQueue.agents[_agent.vaultAddress()].firstTicketId
REF_3524(RedemptionQueue.State) -> state_1 (-> ['TMP_4984']).redemptionQueue
REF_3525(mapping(address => RedemptionQueue.AgentQueue)) -> REF_3524.agents
TMP_4990(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
REF_3527(RedemptionQueue.AgentQueue) -> REF_3525[TMP_4990]
REF_3528(uint64) -> REF_3527.firstTicketId
ticketId_1(uint64) := REF_3528(uint64)
 ticketId == 0
TMP_4991(bool) = ticketId_1 == 0
CONDITION TMP_4991
 ticket = state.redemptionQueue.getTicket(ticketId)
REF_3529(RedemptionQueue.State) -> state_1 (-> ['TMP_4984']).redemptionQueue
TMP_4992(RedemptionQueue.Ticket) = LIBRARY_CALL, dest:RedemptionQueue, function:RedemptionQueue.getTicket(RedemptionQueue.State,uint64), arguments:['REF_3529', 'ticketId_1'] 
ticket_1 (-> ['TMP_4992'])(RedemptionQueue.Ticket) := TMP_4992(RedemptionQueue.Ticket)
 maxTicketRedeemAMG = ticket.valueAMG + _agent.dustAMG
REF_3531(uint64) -> ticket_1 (-> ['TMP_4992']).valueAMG
REF_3532(uint64) -> _agent_1 (-> []).dustAMG
TMP_4993(uint64) = REF_3531 (c)+ REF_3532
maxTicketRedeemAMG_1(uint64) := TMP_4993(uint64)
 maxTicketRedeemAMG -= maxTicketRedeemAMG % lotSize
TMP_4994(uint64) = maxTicketRedeemAMG_1 % lotSize_1
maxTicketRedeemAMG_2(uint64) = maxTicketRedeemAMG_1 (c)- TMP_4994
 ticketRedeemAMG = SafeMath64.min64(_amountAMG - _closedAMG,maxTicketRedeemAMG)
TMP_4995(uint64) = _amountAMG_1 (c)- _closedAMG_1
TMP_4996(uint64) = LIBRARY_CALL, dest:SafeMath64, function:SafeMath64.min64(uint64,uint64), arguments:['TMP_4995', 'maxTicketRedeemAMG_2'] 
ticketRedeemAMG_1(uint64) := TMP_4996(uint64)
 removeFromTicket(ticketId,ticketRedeemAMG)
INTERNAL_CALL, Redemptions.removeFromTicket(uint64,uint64)(ticketId_1,ticketRedeemAMG_1)
 _closedAMG += ticketRedeemAMG
_closedAMG_2(uint64) = _closedAMG_1 (c)+ ticketRedeemAMG_1
 i ++
TMP_4998(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 closeDustAMG = SafeMath64.min64(_amountAMG - _closedAMG,_agent.dustAMG)
TMP_4999(uint64) = _amountAMG_1 (c)- _closedAMG_1
REF_3535(uint64) -> _agent_1 (-> []).dustAMG
TMP_5000(uint64) = LIBRARY_CALL, dest:SafeMath64, function:SafeMath64.min64(uint64,uint64), arguments:['TMP_4999', 'REF_3535'] 
closeDustAMG_1(uint64) := TMP_5000(uint64)
 closeDustAMG > 0
TMP_5001(bool) = closeDustAMG_1 > 0
CONDITION TMP_5001
 _closedAMG += closeDustAMG
_closedAMG_3(uint64) = _closedAMG_1 (c)+ closeDustAMG_1
 AgentBacking.decreaseDust(_agent,closeDustAMG)
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.decreaseDust(Agent.State,uint64), arguments:['_agent_1 (-> [])', 'closeDustAMG_1'] 
_closedAMG_4(uint64) := phi(['_closedAMG_0', '_closedAMG_3'])
 _immediatelyReleaseMinted
CONDITION _immediatelyReleaseMinted_1
 AgentBacking.releaseMintedAssets(_agent,_closedAMG)
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.releaseMintedAssets(Agent.State,uint64), arguments:['_agent_1 (-> [])', '_closedAMG_4'] 
 _closedUBA = Conversion.convertAmgToUBA(_closedAMG)
TMP_5004(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['_closedAMG_4'] 
_closedUBA_1(uint256) := TMP_5004(uint256)
 (_closedAMG,_closedUBA)
RETURN _closedAMG_4,_closedUBA_1
```
#### AgentPayout.payoutFromVault(Agent.State,address,uint256) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_receiver_1(address) := phi(['_receiver_1'])
_amountWei_1(uint256) := phi(['amount_1'])
 collateral = Agents.getVaultCollateral(_agent)
TMP_4380(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4380'])(CollateralTypeInt.Data) := TMP_4380(CollateralTypeInt.Data)
 vault = IIAgentVault(_agent.vaultAddress())
TMP_4381(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4382 = CONVERT TMP_4381 to IIAgentVault
vault_1(IIAgentVault) := TMP_4382(IIAgentVault)
 _amountPaid = Math.min(_amountWei,collateral.token.balanceOf(address(vault)))
REF_2911(IERC20) -> collateral_1 (-> ['TMP_4380']).token
TMP_4383 = CONVERT vault_1 to address
TMP_4384(uint256) = HIGH_LEVEL_CALL, dest:REF_2911(IERC20), function:balanceOf, arguments:['TMP_4383']  
TMP_4385(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_amountWei_1', 'TMP_4384'] 
_amountPaid_1(uint256) := TMP_4385(uint256)
 vault.payout(collateral.token,_receiver,_amountPaid)
REF_2914(IERC20) -> collateral_1 (-> ['TMP_4380']).token
HIGH_LEVEL_CALL, dest:vault_1(IIAgentVault), function:payout, arguments:['REF_2914', '_receiver_1', '_amountPaid_1']  
 _amountPaid
RETURN _amountPaid_1
```
#### Conversion.convertAmgToTokenWei(uint256,uint256) [INTERNAL]
```slithir
AMG_TOKEN_WEI_PRICE_SCALE_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_0'])
 _valueAMG.mulDiv(_amgToTokenWeiPrice,AMG_TOKEN_WEI_PRICE_SCALE)
TMP_4674(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_valueAMG_1', '_amgToTokenWeiPrice_1', 'AMG_TOKEN_WEI_PRICE_SCALE_1'] 
RETURN TMP_4674
```
#### Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data) [INTERNAL]
```slithir
_token_1 (-> [])(CollateralTypeInt.Data) := phi(['_toToken_1 (-> [])', '_fromToken_1 (-> [])'])
 (_price,None,None) = currentAmgPriceInTokenWeiWithTs(_token,false)
TUPLE_44(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool)(_token_1 (-> []),False)
_price_1(uint256)= UNPACK TUPLE_44 index: 0 
 _price
RETURN _price_1
```
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
```
#### AgentBacking.endRedeemingAssets(Agent.State,uint64,bool) [INTERNAL]
```slithir
 _agent.redeemingAMG = _agent.redeemingAMG - _valueAMG
REF_2785(uint64) -> _agent_1 (-> []).redeemingAMG
REF_2786(uint64) -> _agent_1 (-> []).redeemingAMG
TMP_4270(uint64) = REF_2786 (c)- _valueAMG_1
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2785(uint64) (->_agent_2 (-> [])) := TMP_4270(uint64)
 ! _poolSelfCloseRedemption
TMP_4271 = UnaryType.BANG _poolSelfCloseRedemption_1 
CONDITION TMP_4271
 _agent.poolRedeemingAMG = _agent.poolRedeemingAMG - _valueAMG
REF_2787(uint64) -> _agent_2 (-> []).poolRedeemingAMG
REF_2788(uint64) -> _agent_2 (-> []).poolRedeemingAMG
TMP_4272(uint64) = REF_2788 (c)- _valueAMG_1
_agent_3 (-> [])(Agent.State) := phi(['_agent_2 (-> [])'])
REF_2787(uint64) (->_agent_3 (-> [])) := TMP_4272(uint64)
```
#### Agents.requireAgentVaultOwner(Agent.State) [INTERNAL]
```slithir
 require(bool,error)(isOwner(_agent,msg.sender),revert OnlyAgentVaultOwner()())
TMP_4497(bool) = INTERNAL_CALL, Agents.isOwner(Agent.State,address)(_agent_1 (-> []),msg.sender)
TMP_4498(None) = SOLIDITY_CALL revert OnlyAgentVaultOwner()()
TMP_4499(None) = SOLIDITY_CALL require(bool,error)(TMP_4497,TMP_4498)
```
#### Redemptions.burnExecutorFee(Redemption.Request) [INTERNAL]
```slithir
 executorFeeNatWei = _request.executorFeeNatGWei * Conversion.GWEI
REF_3568(uint64) -> _request_1 (-> []).executorFeeNatGWei
REF_3569(uint256) -> Conversion.GWEI
TMP_5031(uint64) = REF_3568 (c)* REF_3569
executorFeeNatWei_1(uint256) := TMP_5031(uint64)
 executorFeeNatWei > 0
TMP_5032(bool) = executorFeeNatWei_1 > 0
CONDITION TMP_5032
 _request.executorFeeNatGWei = 0
REF_3570(uint64) -> _request_1 (-> []).executorFeeNatGWei
_request_2 (-> [])(Redemption.Request) := phi(['_request_1 (-> [])'])
REF_3570(uint64) (->_request_2 (-> [])) := 0(uint256)
 Globals.getBurnAddress().transfer(executorFeeNatWei)
TMP_5033(address) = LIBRARY_CALL, dest:Globals, function:Globals.getBurnAddress(), arguments:[] 
Transfer dest:TMP_5033 value:executorFeeNatWei_1
```
#### Redemptions.finishRedemptionRequest(uint256,Redemption.Request,Redemption.Status) [INTERNAL]
```slithir
 assert(bool)(_status >= Redemption.Status.SUCCESSFUL)
REF_3578(Redemption.Status) -> Status.SUCCESSFUL
TMP_5037(bool) = _status_1 >= REF_3578
TMP_5038(None) = SOLIDITY_CALL assert(bool)(TMP_5037)
 _request.status = _status
REF_3579(Redemption.Status) -> _request_1 (-> []).status
_request_2 (-> [])(Redemption.Request) := phi(['_request_1 (-> [])'])
REF_3579(Redemption.Status) (->_request_2 (-> [])) := _status_1(Redemption.Status)
 releaseTransferToCoreVault(_redemptionRequestId,_request)
INTERNAL_CALL, Redemptions.releaseTransferToCoreVault(uint256,Redemption.Request)(_redemptionRequestId_1,_request_2 (-> []))
```
#### Redemptions.getRedemptionRequest(uint256,bool) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_5042(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_5042'])(AssetManagerState.State) := TMP_5042(AssetManagerState.State)
 require(bool,error)(_redemptionRequestId != 0,revert InvalidRequestId()())
TMP_5043(bool) = _redemptionRequestId_1 != 0
TMP_5044(None) = SOLIDITY_CALL revert InvalidRequestId()()
TMP_5045(None) = SOLIDITY_CALL require(bool,error)(TMP_5043,TMP_5044)
 _request = state.redemptionRequests[_redemptionRequestId]
REF_3586(mapping(uint256 => Redemption.Request)) -> state_1 (-> ['TMP_5042']).redemptionRequests
REF_3587(Redemption.Request) -> REF_3586[_redemptionRequestId_1]
_request_1 (-> ['state'])(Redemption.Request) := REF_3587(Redemption.Request)
 _requireUnconfirmed
CONDITION _requireUnconfirmed_1
 require(bool,error)(isOpen(_request),revert InvalidRequestId()())
TMP_5046(bool) = INTERNAL_CALL, Redemptions.isOpen(Redemption.Request)(_request_1 (-> ['state']))
TMP_5047(None) = SOLIDITY_CALL revert InvalidRequestId()()
TMP_5048(None) = SOLIDITY_CALL require(bool,error)(TMP_5046,TMP_5047)
 require(bool,error)(_request.status != Redemption.Status.EMPTY,revert InvalidRequestId()())
REF_3588(Redemption.Status) -> _request_1 (-> ['state']).status
REF_3589(Redemption.Status) -> Status.EMPTY
TMP_5049(bool) = REF_3588 != REF_3589
TMP_5050(None) = SOLIDITY_CALL revert InvalidRequestId()()
TMP_5051(None) = SOLIDITY_CALL require(bool,error)(TMP_5049,TMP_5050)
 _request
RETURN _request_1 (-> ['state'])
```
#### TransactionAttestation.verifyAddressValidity(IAddressValidity.Proof) [INTERNAL]
```slithir
 _settings = Globals.getSettings()
TMP_5269(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5269'])(AssetManagerSettings.Data) := TMP_5269(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3694(address) -> _settings_1 (-> ['TMP_5269']).fdcVerification
TMP_5270 = CONVERT REF_3694 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5270(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3695(IAddressValidity.Response) -> _proof_1.data
REF_3696(bytes32) -> REF_3695.sourceId
REF_3697(bytes32) -> _settings_1 (-> ['TMP_5269']).chainId
TMP_5271(bool) = REF_3696 == REF_3697
TMP_5272(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5273(None) = SOLIDITY_CALL require(bool,error)(TMP_5271,TMP_5272)
 require(bool,error)(fdcVerification.verifyAddressValidity(_proof),revert AddressValidityNotProven()())
TMP_5274(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyAddressValidity, arguments:['_proof_1']  
TMP_5275(None) = SOLIDITY_CALL revert AddressValidityNotProven()()
TMP_5276(None) = SOLIDITY_CALL require(bool,error)(TMP_5274,TMP_5275)
```
#### Liquidation.endLiquidationIfHealthy(Agent.State) [INTERNAL]
```slithir
 _agent.status != Agent.Status.LIQUIDATION
REF_3256(Agent.Status) -> _agent_1 (-> []).status
REF_3257(Agent.Status) -> Status.LIQUIDATION
TMP_4748(bool) = REF_3256 != REF_3257
CONDITION TMP_4748
 cr = getCollateralRatiosBIPS(_agent)
TMP_4749(Liquidation.CRData) = INTERNAL_CALL, Liquidation.getCollateralRatiosBIPS(Agent.State)(_agent_1 (-> []))
cr_1(Liquidation.CRData) := TMP_4749(Liquidation.CRData)
 targetRatioVaultCollateralBIPS = _targetRatioBIPS(_agent,Collateral.Kind.VAULT)
REF_3258(Collateral.Kind) -> Kind.VAULT
TMP_4750(uint256) = INTERNAL_CALL, Liquidation._targetRatioBIPS(Agent.State,Collateral.Kind)(_agent_1 (-> []),REF_3258)
targetRatioVaultCollateralBIPS_1(uint256) := TMP_4750(uint256)
 targetRatioPoolBIPS = _targetRatioBIPS(_agent,Collateral.Kind.POOL)
REF_3259(Collateral.Kind) -> Kind.POOL
TMP_4751(uint256) = INTERNAL_CALL, Liquidation._targetRatioBIPS(Agent.State,Collateral.Kind)(_agent_1 (-> []),REF_3259)
targetRatioPoolBIPS_1(uint256) := TMP_4751(uint256)
 cr.vaultCR >= targetRatioVaultCollateralBIPS && cr.poolCR >= targetRatioPoolBIPS
REF_3260(uint256) -> cr_1.vaultCR
TMP_4752(bool) = REF_3260 >= targetRatioVaultCollateralBIPS_1
REF_3261(uint256) -> cr_1.poolCR
TMP_4753(bool) = REF_3261 >= targetRatioPoolBIPS_1
TMP_4754(bool) = TMP_4752 && TMP_4753
CONDITION TMP_4754
 _agent.status = Agent.Status.NORMAL
REF_3262(Agent.Status) -> _agent_1 (-> []).status
REF_3263(Agent.Status) -> Status.NORMAL
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_3262(Agent.Status) (->_agent_2 (-> [])) := REF_3263(Agent.Status)
 _agent.liquidationStartedAt = 0
REF_3264(uint64) -> _agent_2 (-> []).liquidationStartedAt
_agent_3 (-> [])(Agent.State) := phi(['_agent_2 (-> [])'])
REF_3264(uint64) (->_agent_3 (-> [])) := 0(uint256)
 _agent.collateralsUnderwater = 0
REF_3265(uint8) -> _agent_3 (-> []).collateralsUnderwater
_agent_4 (-> [])(Agent.State) := phi(['_agent_3 (-> [])'])
REF_3265(uint8) (->_agent_4 (-> [])) := 0(uint256)
 IAssetManagerEvents.LiquidationEnded(_agent.vaultAddress())
TMP_4755(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_4 (-> [])'] 
Emit LiquidationEnded(TMP_4755)
```
#### RedemptionQueue.deleteRedemptionTicket(RedemptionQueue.State,uint64) [INTERNAL]
```slithir
 ticket = _state.tickets[_ticketId]
REF_3800(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_1 (-> []).tickets
REF_3801(RedemptionQueue.Ticket) -> REF_3800[_ticketId_1]
ticket_1 (-> ['_state'])(RedemptionQueue.Ticket) := REF_3801(RedemptionQueue.Ticket)
 assert(bool)(ticket.agentVault != address(0))
REF_3802(address) -> ticket_1 (-> ['_state']).agentVault
TMP_5410 = CONVERT 0 to address
TMP_5411(bool) = REF_3802 != TMP_5410
TMP_5412(None) = SOLIDITY_CALL assert(bool)(TMP_5411)
 agent = _state.agents[ticket.agentVault]
REF_3803(mapping(address => RedemptionQueue.AgentQueue)) -> _state_1 (-> []).agents
REF_3804(address) -> ticket_1 (-> ['_state']).agentVault
REF_3805(RedemptionQueue.AgentQueue) -> REF_3803[REF_3804]
agent_1 (-> ['_state'])(RedemptionQueue.AgentQueue) := REF_3805(RedemptionQueue.AgentQueue)
 ticket.prev == 0
REF_3806(uint64) -> ticket_1 (-> ['_state']).prev
TMP_5413(bool) = REF_3806 == 0
CONDITION TMP_5413
 assert(bool)(_ticketId == _state.firstTicketId)
REF_3807(uint64) -> _state_1 (-> []).firstTicketId
TMP_5414(bool) = _ticketId_1 == REF_3807
TMP_5415(None) = SOLIDITY_CALL assert(bool)(TMP_5414)
 _state.firstTicketId = ticket.next
REF_3808(uint64) -> _state_1 (-> []).firstTicketId
REF_3809(uint64) -> ticket_1 (-> ['_state']).next
_state_3 (-> [])(RedemptionQueue.State) := phi(['_state_1 (-> [])'])
REF_3808(uint64) (->_state_3 (-> [])) := REF_3809(uint64)
 assert(bool)(_ticketId != _state.firstTicketId)
REF_3810(uint64) -> _state_1 (-> []).firstTicketId
TMP_5416(bool) = _ticketId_1 != REF_3810
TMP_5417(None) = SOLIDITY_CALL assert(bool)(TMP_5416)
 _state.tickets[ticket.prev].next = ticket.next
REF_3811(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_1 (-> []).tickets
REF_3812(uint64) -> ticket_1 (-> ['_state']).prev
REF_3813(RedemptionQueue.Ticket) -> REF_3811[REF_3812]
REF_3814(uint64) -> REF_3813.next
REF_3815(uint64) -> ticket_1 (-> ['_state']).next
_state_2 (-> [])(RedemptionQueue.State) := phi(['_state_1 (-> [])'])
REF_3814(uint64) (->_state_2 (-> [])) := REF_3815(uint64)
_state_4 (-> [])(RedemptionQueue.State) := phi(['_state_3 (-> [])', '_state_2 (-> [])'])
 ticket.next == 0
REF_3816(uint64) -> ticket_1 (-> ['_state']).next
TMP_5418(bool) = REF_3816 == 0
CONDITION TMP_5418
 assert(bool)(_ticketId == _state.lastTicketId)
REF_3817(uint64) -> _state_4 (-> []).lastTicketId
TMP_5419(bool) = _ticketId_1 == REF_3817
TMP_5420(None) = SOLIDITY_CALL assert(bool)(TMP_5419)
 _state.lastTicketId = ticket.prev
REF_3818(uint64) -> _state_4 (-> []).lastTicketId
REF_3819(uint64) -> ticket_1 (-> ['_state']).prev
_state_6 (-> [])(RedemptionQueue.State) := phi(['_state_4 (-> [])'])
REF_3818(uint64) (->_state_6 (-> [])) := REF_3819(uint64)
 assert(bool)(_ticketId != _state.lastTicketId)
REF_3820(uint64) -> _state_4 (-> []).lastTicketId
TMP_5421(bool) = _ticketId_1 != REF_3820
TMP_5422(None) = SOLIDITY_CALL assert(bool)(TMP_5421)
 _state.tickets[ticket.next].prev = ticket.prev
REF_3821(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_4 (-> []).tickets
REF_3822(uint64) -> ticket_1 (-> ['_state']).next
REF_3823(RedemptionQueue.Ticket) -> REF_3821[REF_3822]
REF_3824(uint64) -> REF_3823.prev
REF_3825(uint64) -> ticket_1 (-> ['_state']).prev
_state_5 (-> [])(RedemptionQueue.State) := phi(['_state_4 (-> [])'])
REF_3824(uint64) (->_state_5 (-> [])) := REF_3825(uint64)
_state_7 (-> [])(RedemptionQueue.State) := phi(['_state_5 (-> [])', '_state_6 (-> [])'])
 ticket.prevForAgent == 0
REF_3826(uint64) -> ticket_1 (-> ['_state']).prevForAgent
TMP_5423(bool) = REF_3826 == 0
CONDITION TMP_5423
 assert(bool)(_ticketId == agent.firstTicketId)
REF_3827(uint64) -> agent_1 (-> ['_state']).firstTicketId
TMP_5424(bool) = _ticketId_1 == REF_3827
TMP_5425(None) = SOLIDITY_CALL assert(bool)(TMP_5424)
 agent.firstTicketId = ticket.nextForAgent
REF_3828(uint64) -> agent_1 (-> ['_state']).firstTicketId
REF_3829(uint64) -> ticket_1 (-> ['_state']).nextForAgent
agent_2 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_1 (-> ['_state'])"])
REF_3828(uint64) (->agent_2 (-> ['_state'])) := REF_3829(uint64)
_state_12 (-> ['_state'])(RedemptionQueue.State) := phi(["agent_2 (-> ['_state'])"])
 assert(bool)(_ticketId != agent.firstTicketId)
REF_3830(uint64) -> agent_1 (-> ['_state']).firstTicketId
TMP_5426(bool) = _ticketId_1 != REF_3830
TMP_5427(None) = SOLIDITY_CALL assert(bool)(TMP_5426)
 _state.tickets[ticket.prevForAgent].nextForAgent = ticket.nextForAgent
REF_3831(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_7 (-> []).tickets
REF_3832(uint64) -> ticket_1 (-> ['_state']).prevForAgent
REF_3833(RedemptionQueue.Ticket) -> REF_3831[REF_3832]
REF_3834(uint64) -> REF_3833.nextForAgent
REF_3835(uint64) -> ticket_1 (-> ['_state']).nextForAgent
_state_8 (-> [])(RedemptionQueue.State) := phi(['_state_7 (-> [])'])
REF_3834(uint64) (->_state_8 (-> [])) := REF_3835(uint64)
_state_9 (-> [])(RedemptionQueue.State) := phi(['_state_8 (-> [])', '_state_1 (-> [])'])
agent_3 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_1 (-> ['_state'])", "agent_2 (-> ['_state'])"])
 ticket.nextForAgent == 0
REF_3836(uint64) -> ticket_1 (-> ['_state']).nextForAgent
TMP_5428(bool) = REF_3836 == 0
CONDITION TMP_5428
 assert(bool)(_ticketId == agent.lastTicketId)
REF_3837(uint64) -> agent_3 (-> ['_state']).lastTicketId
TMP_5429(bool) = _ticketId_1 == REF_3837
TMP_5430(None) = SOLIDITY_CALL assert(bool)(TMP_5429)
 agent.lastTicketId = ticket.prevForAgent
REF_3838(uint64) -> agent_3 (-> ['_state']).lastTicketId
REF_3839(uint64) -> ticket_1 (-> ['_state']).prevForAgent
agent_4 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_3 (-> ['_state'])"])
REF_3838(uint64) (->agent_4 (-> ['_state'])) := REF_3839(uint64)
_state_13 (-> ['_state'])(RedemptionQueue.State) := phi(["agent_4 (-> ['_state'])"])
 assert(bool)(_ticketId != agent.lastTicketId)
REF_3840(uint64) -> agent_3 (-> ['_state']).lastTicketId
TMP_5431(bool) = _ticketId_1 != REF_3840
TMP_5432(None) = SOLIDITY_CALL assert(bool)(TMP_5431)
 _state.tickets[ticket.nextForAgent].prevForAgent = ticket.prevForAgent
REF_3841(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_9 (-> []).tickets
REF_3842(uint64) -> ticket_1 (-> ['_state']).nextForAgent
REF_3843(RedemptionQueue.Ticket) -> REF_3841[REF_3842]
REF_3844(uint64) -> REF_3843.prevForAgent
REF_3845(uint64) -> ticket_1 (-> ['_state']).prevForAgent
_state_10 (-> [])(RedemptionQueue.State) := phi(['_state_9 (-> [])'])
REF_3844(uint64) (->_state_10 (-> [])) := REF_3845(uint64)
_state_11 (-> [])(RedemptionQueue.State) := phi(['_state_10 (-> [])', '_state_1 (-> [])'])
 delete _state.tickets[_ticketId]
REF_3846(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_11 (-> []).tickets
REF_3847(RedemptionQueue.Ticket) -> REF_3846[_ticketId_1]
REF_3846 = delete REF_3847
```
#### Agent.getWithoutCheck(address) [INTERNAL]
```slithir
_address_1(address) := phi(['_address_1', '_address_1'])
AGENTS_POSITION_1(bytes32) := phi(['AGENTS_POSITION_0'])
 position = bytes32(uint256(AGENTS_POSITION) ^ (uint256(uint160(_address)) << 64))
TMP_5319 = CONVERT AGENTS_POSITION_1 to uint256
TMP_5320 = CONVERT _address_1 to uint160
TMP_5321 = CONVERT TMP_5320 to uint256
TMP_5322(uint256) = TMP_5321 << 64
TMP_5323(uint256) = TMP_5319 ^ TMP_5322
TMP_5324 = CONVERT TMP_5323 to bytes32
position_1(bytes32) := TMP_5324(bytes32)
 _agent = position
_agent_1 (-> ['position'])(Agent.State) := position_1(bytes32)
 _agent
RETURN _agent_1 (-> ['position'])
```
#### RedemptionQueue.createRedemptionTicket(RedemptionQueue.State,address,uint64) [INTERNAL]
```slithir
 agent = _state.agents[_agentVault]
REF_3775(mapping(address => RedemptionQueue.AgentQueue)) -> _state_1 (-> []).agents
REF_3776(RedemptionQueue.AgentQueue) -> REF_3775[_agentVault_1]
agent_1 (-> ['_state'])(RedemptionQueue.AgentQueue) := REF_3776(RedemptionQueue.AgentQueue)
 ticketId = ++ _state.newTicketId
REF_3777(uint64) -> _state_1 (-> []).newTicketId
_state_2 (-> [])(RedemptionQueue.State) := phi(['_state_1 (-> [])'])
REF_3777(-> _state_2 (-> [])) = REF_3777 (c)+ 1
ticketId_1(uint64) := REF_3777(uint64)
 _state.tickets[ticketId] = Ticket({agentVault:_agentVault,valueAMG:_valueAMG,prev:_state.lastTicketId,next:0,prevForAgent:agent.lastTicketId,nextForAgent:0})
REF_3778(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_2 (-> []).tickets
REF_3779(RedemptionQueue.Ticket) -> REF_3778[ticketId_1]
REF_3780(uint64) -> _state_2 (-> []).lastTicketId
REF_3781(uint64) -> agent_1 (-> ['_state']).lastTicketId
TMP_5399(RedemptionQueue.Ticket) = new Ticket(_agentVault_1,_valueAMG_1,REF_3780,0,REF_3781,0)
_state_3 (-> [])(RedemptionQueue.State) := phi(['_state_2 (-> [])'])
REF_3779(RedemptionQueue.Ticket) (->_state_3 (-> [])) := TMP_5399(RedemptionQueue.Ticket)
 _state.firstTicketId == 0
REF_3782(uint64) -> _state_3 (-> []).firstTicketId
TMP_5400(bool) = REF_3782 == 0
CONDITION TMP_5400
 assert(bool)(_state.lastTicketId == 0)
REF_3783(uint64) -> _state_3 (-> []).lastTicketId
TMP_5401(bool) = REF_3783 == 0
TMP_5402(None) = SOLIDITY_CALL assert(bool)(TMP_5401)
 _state.firstTicketId = ticketId
REF_3784(uint64) -> _state_3 (-> []).firstTicketId
_state_4 (-> [])(RedemptionQueue.State) := phi(['_state_3 (-> [])'])
REF_3784(uint64) (->_state_4 (-> [])) := ticketId_1(uint64)
 assert(bool)(_state.lastTicketId != 0)
REF_3785(uint64) -> _state_3 (-> []).lastTicketId
TMP_5403(bool) = REF_3785 != 0
TMP_5404(None) = SOLIDITY_CALL assert(bool)(TMP_5403)
 _state.tickets[_state.lastTicketId].next = ticketId
REF_3786(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_3 (-> []).tickets
REF_3787(uint64) -> _state_3 (-> []).lastTicketId
REF_3788(RedemptionQueue.Ticket) -> REF_3786[REF_3787]
REF_3789(uint64) -> REF_3788.next
_state_5 (-> [])(RedemptionQueue.State) := phi(['_state_3 (-> [])'])
REF_3789(uint64) (->_state_5 (-> [])) := ticketId_1(uint64)
_state_6 (-> [])(RedemptionQueue.State) := phi(['_state_4 (-> [])', '_state_5 (-> [])'])
 _state.lastTicketId = ticketId
REF_3790(uint64) -> _state_6 (-> []).lastTicketId
_state_7 (-> [])(RedemptionQueue.State) := phi(['_state_6 (-> [])'])
REF_3790(uint64) (->_state_7 (-> [])) := ticketId_1(uint64)
 agent.firstTicketId == 0
REF_3791(uint64) -> agent_1 (-> ['_state']).firstTicketId
TMP_5405(bool) = REF_3791 == 0
CONDITION TMP_5405
 assert(bool)(agent.lastTicketId == 0)
REF_3792(uint64) -> agent_1 (-> ['_state']).lastTicketId
TMP_5406(bool) = REF_3792 == 0
TMP_5407(None) = SOLIDITY_CALL assert(bool)(TMP_5406)
 agent.firstTicketId = ticketId
REF_3793(uint64) -> agent_1 (-> ['_state']).firstTicketId
agent_2 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_1 (-> ['_state'])"])
REF_3793(uint64) (->agent_2 (-> ['_state'])) := ticketId_1(uint64)
_state_9 (-> ['_state'])(RedemptionQueue.State) := phi(["agent_2 (-> ['_state'])"])
 assert(bool)(agent.lastTicketId != 0)
REF_3794(uint64) -> agent_1 (-> ['_state']).lastTicketId
TMP_5408(bool) = REF_3794 != 0
TMP_5409(None) = SOLIDITY_CALL assert(bool)(TMP_5408)
 _state.tickets[agent.lastTicketId].nextForAgent = ticketId
REF_3795(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_7 (-> []).tickets
REF_3796(uint64) -> agent_1 (-> ['_state']).lastTicketId
REF_3797(RedemptionQueue.Ticket) -> REF_3795[REF_3796]
REF_3798(uint64) -> REF_3797.nextForAgent
_state_8 (-> [])(RedemptionQueue.State) := phi(['_state_7 (-> [])'])
REF_3798(uint64) (->_state_8 (-> [])) := ticketId_1(uint64)
agent_3 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_2 (-> ['_state'])", "agent_1 (-> ['_state'])"])
 agent.lastTicketId = ticketId
REF_3799(uint64) -> agent_3 (-> ['_state']).lastTicketId
agent_4 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_3 (-> ['_state'])"])
REF_3799(uint64) (->agent_4 (-> ['_state'])) := ticketId_1(uint64)
_state_10 (-> ['_state'])(RedemptionQueue.State) := phi(["agent_4 (-> ['_state'])"])
 ticketId
RETURN ticketId_1
```
#### AgentBacking.startRedeemingAssets(Agent.State,uint64,bool) [INTERNAL]
```slithir
 _agent.redeemingAMG += _valueAMG
REF_2783(uint64) -> _agent_1 (-> []).redeemingAMG
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2783(-> _agent_2 (-> [])) = REF_2783 (c)+ _valueAMG_1
 ! _poolSelfCloseRedemption
TMP_4268 = UnaryType.BANG _poolSelfCloseRedemption_1 
CONDITION TMP_4268
 _agent.poolRedeemingAMG += _valueAMG
REF_2784(uint64) -> _agent_2 (-> []).poolRedeemingAMG
_agent_3 (-> [])(Agent.State) := phi(['_agent_2 (-> [])'])
REF_2784(-> _agent_3 (-> [])) = REF_2784 (c)+ _valueAMG_1
_agent_4 (-> [])(Agent.State) := phi(['_agent_3 (-> [])', '_agent_2 (-> [])'])
 releaseMintedAssets(_agent,_valueAMG)
INTERNAL_CALL, AgentBacking.releaseMintedAssets(Agent.State,uint64)(_agent_4 (-> []),_valueAMG_1)
```
#### RedemptionRequests._emitRedemptionRequestedEvent(Redemption.Request,uint64,string) [PRIVATE]
```slithir
_request_1(Redemption.Request) := phi(['request_17'])
_requestId_1(uint64) := phi(['_requestId_1'])
_redeemerUnderlyingAddressString_1(string) := phi(['_redeemerUnderlyingAddressString_1'])
 IAssetManagerEvents.RedemptionRequested(_request.agentVault,_request.redeemer,_requestId,_redeemerUnderlyingAddressString,_request.underlyingValueUBA,_request.underlyingFeeUBA,_request.firstUnderlyingBlock,_request.lastUnderlyingBlock,_request.lastUnderlyingTimestamp,PaymentReference.redemption(_requestId),_request.executor,_request.executorFeeNatGWei * Conversion.GWEI)
REF_3493(address) -> _request_1.agentVault
REF_3494(address) -> _request_1.redeemer
REF_3495(uint128) -> _request_1.underlyingValueUBA
REF_3496(uint128) -> _request_1.underlyingFeeUBA
REF_3497(uint64) -> _request_1.firstUnderlyingBlock
REF_3498(uint64) -> _request_1.lastUnderlyingBlock
REF_3499(uint64) -> _request_1.lastUnderlyingTimestamp
TMP_4953(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.redemption(uint256), arguments:['_requestId_1'] 
REF_3501(address) -> _request_1.executor
REF_3502(uint64) -> _request_1.executorFeeNatGWei
REF_3503(uint256) -> Conversion.GWEI
TMP_4954(uint64) = REF_3502 (c)* REF_3503
Emit RedemptionRequested(REF_3493,REF_3494,_requestId_1,_redeemerUnderlyingAddressString_1,REF_3495,REF_3496,REF_3497,REF_3498,REF_3499,TMP_4953,REF_3501,TMP_4954)
```

#### RedemptionRequests._newRequestId(bool) [PRIVATE]
```slithir
_poolSelfClose_1(bool) := phi(['_poolSelfClose_1'])
 state = AssetManagerState.get()
TMP_4956(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4956'])(AssetManagerState.State) := TMP_4956(AssetManagerState.State)
 nextRequestId = state.newRedemptionRequestId + PaymentReference.randomizedIdSkip()
REF_3505(uint64) -> state_1 (-> ['TMP_4956']).newRedemptionRequestId
TMP_4957(uint64) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.randomizedIdSkip(), arguments:[] 
TMP_4958(uint64) = REF_3505 (c)+ TMP_4957
nextRequestId_1(uint64) := TMP_4958(uint64)
 state.newRedemptionRequestId = requestId
REF_3507(uint64) -> state_1 (-> ['TMP_4956']).newRedemptionRequestId
state_2 (-> ['TMP_4956'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_4956'])"])
REF_3507(uint64) (->state_2 (-> ['TMP_4956'])) := requestId_3(uint64)
TMP_4956(AssetManagerState.State) := phi(["state_2 (-> ['TMP_4956'])"])
 requestId
RETURN requestId_3
 _poolSelfClose
CONDITION _poolSelfClose_1
 requestId = ((nextRequestId + 1) & ~ uint64(1)) | 1
TMP_4959(uint64) = nextRequestId_1 (c)+ 1
TMP_4960 = CONVERT 1 to uint64
TMP_4961 = UnaryType.TILD TMP_4960 
TMP_4962(uint64) = TMP_4959 & TMP_4961
TMP_4963(uint64) = TMP_4962 | 1
requestId_2(uint64) := TMP_4963(uint64)
 requestId = ((nextRequestId + 1) & ~ uint64(1)) | 0
TMP_4964(uint64) = nextRequestId_1 (c)+ 1
TMP_4965 = CONVERT 1 to uint64
TMP_4966 = UnaryType.TILD TMP_4965 
TMP_4967(uint64) = TMP_4964 & TMP_4966
TMP_4968(uint64) = TMP_4967 | 0
requestId_1(uint64) := TMP_4968(uint64)
requestId_3(uint64) := phi(['requestId_1', 'requestId_2'])
```
#### SafeCast.toUint128(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint128).max,SafeCast: value doesn't fit in 128 bits)
TMP_707(uint128) := 340282366920938463463374607431768211455(uint128)
TMP_708(bool) = value_1 <= TMP_707
TMP_709(None) = SOLIDITY_CALL require(bool,string)(TMP_708,SafeCast: value doesn't fit in 128 bits)
 uint128(value)
TMP_710 = CONVERT value_1 to uint128
RETURN TMP_710
```
#### Globals.getFAsset() [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4735(AssetManagerSettings.Data) = INTERNAL_CALL, Globals.getSettings()()
settings_1 (-> ['TMP_4735'])(AssetManagerSettings.Data) := TMP_4735(AssetManagerSettings.Data)
 IIFAsset(settings.fAsset)
REF_3240(address) -> settings_1 (-> ['TMP_4735']).fAsset
TMP_4736 = CONVERT REF_3240 to IIFAsset
RETURN TMP_4736
```
#### IIFAsset.burn(address,uint256) [EXTERNAL]
```slithir

```

#### AgentBacking.releaseMintedAssets(Agent.State,uint64) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_4 (-> [])'])
_valueAMG_1(uint64) := phi(['_valueAMG_1'])
 _agent.mintedAMG = _agent.mintedAMG - _valueAMG
REF_2781(uint64) -> _agent_1 (-> []).mintedAMG
REF_2782(uint64) -> _agent_1 (-> []).mintedAMG
TMP_4267(uint64) = REF_2782 (c)- _valueAMG_1
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2781(uint64) (->_agent_2 (-> [])) := TMP_4267(uint64)
```
#### SafeMath64.min64(uint64,uint64) [INTERNAL]
```slithir
 a <= b
TMP_10505(bool) = a_1 <= b_1
CONDITION TMP_10505
 a
RETURN a_1
 b
RETURN b_1
```
#### Agents.getVaultCollateral(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 state = AssetManagerState.get()
TMP_4510(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4510'])(AssetManagerState.State) := TMP_4510(AssetManagerState.State)
 state.collateralTokens[_agent.vaultCollateralIndex]
REF_3005(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4510']).collateralTokens
REF_3006(uint16) -> _agent_1 (-> []).vaultCollateralIndex
REF_3007(CollateralTypeInt.Data) -> REF_3005[REF_3006]
RETURN REF_3007
```
#### ERC20.balanceOf(address) [PUBLIC]
```slithir
_balances_1(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_8', '_balances_5', '_balances_0'])
 _balances[account]
REF_73(uint256) -> _balances_1[account_1]
RETURN REF_73
```
#### IIAgentVault.payout(IERC20,address,uint256) [EXTERNAL]
```slithir

```
#### SafePct.mulDiv(uint256,uint256,uint256) [INTERNAL]
```slithir
x_1(uint256) := phi(['x_1', 'x_1'])
y_1(uint256) := phi(['y_1', 'y_1'])
z_1(uint256) := phi(['z_1', 'MAX_BIPS_1'])
 require(bool,error)(z > 0,revert DivisionByZero()())
TMP_10510(bool) = z_1 > 0
TMP_10511(None) = SOLIDITY_CALL revert DivisionByZero()()
TMP_10512(None) = SOLIDITY_CALL require(bool,error)(TMP_10510,TMP_10511)
 x == 0
TMP_10513(bool) = x_1 == 0
CONDITION TMP_10513
 0
RETURN 0
 xy = x * y
TMP_10514(uint256) = x_1 * y_1
xy_1(uint256) := TMP_10514(uint256)
 xy / x == y
TMP_10515(uint256) = xy_1 / x_1
TMP_10516(bool) = TMP_10515 == y_1
CONDITION TMP_10516
 xy / z
TMP_10517(uint256) = xy_1 / z_1
RETURN TMP_10517
 a = x / z
TMP_10518(uint256) = x_1 (c)/ z_1
a_1(uint256) := TMP_10518(uint256)
 b = x % z
TMP_10519(uint256) = x_1 % z_1
b_1(uint256) := TMP_10519(uint256)
 c = y / z
TMP_10520(uint256) = y_1 (c)/ z_1
c_1(uint256) := TMP_10520(uint256)
 d = y % z
TMP_10521(uint256) = y_1 % z_1
d_1(uint256) := TMP_10521(uint256)
 (a * c * z) + (a * d) + (b * c) + (b * d / z)
TMP_10522(uint256) = a_1 (c)* c_1
TMP_10523(uint256) = TMP_10522 (c)* z_1
TMP_10524(uint256) = a_1 (c)* d_1
TMP_10525(uint256) = TMP_10523 (c)+ TMP_10524
TMP_10526(uint256) = b_1 (c)* c_1
TMP_10527(uint256) = TMP_10525 (c)+ TMP_10526
TMP_10528(uint256) = b_1 (c)* d_1
TMP_10529(uint256) = TMP_10528 (c)/ z_1
TMP_10530(uint256) = TMP_10527 (c)+ TMP_10529
RETURN TMP_10530
```
#### Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool) [INTERNAL]
```slithir
_token_1 (-> [])(CollateralTypeInt.Data) := phi(['_token_1 (-> [])', 'REF_3140', '_token_1 (-> [])'])
 (assetPrice,assetTs,assetFtsoDec) = readFtsoPrice(_token.assetFtsoSymbol,_fromTrustedProviders)
REF_3163(string) -> _token_1 (-> []).assetFtsoSymbol
TUPLE_48(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.readFtsoPrice(string,bool)(REF_3163,_fromTrustedProviders_1)
assetPrice_1(uint256)= UNPACK TUPLE_48 index: 0 
assetTs_1(uint256)= UNPACK TUPLE_48 index: 1 
assetFtsoDec_1(uint256)= UNPACK TUPLE_48 index: 2 
 _token.directPricePair
REF_3164(bool) -> _token_1 (-> []).directPricePair
CONDITION REF_3164
 price = calcAmgToTokenWeiPrice(_token.decimals,1,0,assetPrice,assetFtsoDec)
REF_3165(uint8) -> _token_1 (-> []).decimals
TMP_4661(uint256) = INTERNAL_CALL, Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256)(REF_3165,1,0,assetPrice_1,assetFtsoDec_1)
price_1(uint256) := TMP_4661(uint256)
 (price,assetTs,assetTs)
RETURN price_1,assetTs_1,assetTs_1
 (tokenPrice,tokenTs,tokenFtsoDec) = readFtsoPrice(_token.tokenFtsoSymbol,_fromTrustedProviders)
REF_3166(string) -> _token_1 (-> []).tokenFtsoSymbol
TUPLE_49(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.readFtsoPrice(string,bool)(REF_3166,_fromTrustedProviders_1)
tokenPrice_1(uint256)= UNPACK TUPLE_49 index: 0 
tokenTs_1(uint256)= UNPACK TUPLE_49 index: 1 
tokenFtsoDec_1(uint256)= UNPACK TUPLE_49 index: 2 
 price_scope_0 = calcAmgToTokenWeiPrice(_token.decimals,tokenPrice,tokenFtsoDec,assetPrice,assetFtsoDec)
REF_3167(uint8) -> _token_1 (-> []).decimals
TMP_4662(uint256) = INTERNAL_CALL, Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256)(REF_3167,tokenPrice_1,tokenFtsoDec_1,assetPrice_1,assetFtsoDec_1)
price_scope_0_1(uint256) := TMP_4662(uint256)
 (price_scope_0,assetTs,tokenTs)
RETURN price_scope_0_1,assetTs_1,tokenTs_1
```
#### Agents.isOwner(Agent.State,address) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['TMP_4493', '_agent_1 (-> [])'])
_address_1(address) := phi(['msg.sender'])
 _address == _agent.ownerManagementAddress || _address == getWorkAddress(_agent)
REF_2989(address) -> _agent_1 (-> []).ownerManagementAddress
TMP_4477(bool) = _address_1 == REF_2989
TMP_4478(address) = INTERNAL_CALL, Agents.getWorkAddress(Agent.State)(_agent_1 (-> []))
TMP_4479(bool) = _address_1 == TMP_4478
TMP_4480(bool) = TMP_4477 || TMP_4479
RETURN TMP_4480
```
#### Globals.getBurnAddress() [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4739(AssetManagerSettings.Data) = INTERNAL_CALL, Globals.getSettings()()
settings_1 (-> ['TMP_4739'])(AssetManagerSettings.Data) := TMP_4739(AssetManagerSettings.Data)
 settings.burnAddress
REF_3244(address) -> settings_1 (-> ['TMP_4739']).burnAddress
RETURN REF_3244
```
#### Redemptions.releaseTransferToCoreVault(uint256,Redemption.Request) [INTERNAL]
```slithir
_redemptionRequestId_1(uint256) := phi(['_redemptionRequestId_1'])
_request_1 (-> [])(Redemption.Request) := phi(['_request_2 (-> [])'])
 _request.transferToCoreVault
REF_3580(bool) -> _request_1 (-> []).transferToCoreVault
CONDITION REF_3580
 agent = Agent.get(_request.agentVault)
REF_3582(address) -> _request_1 (-> []).agentVault
TMP_5040(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_3582'] 
agent_1 (-> ['TMP_5040'])(Agent.State) := TMP_5040(Agent.State)
 agent.activeTransferToCoreVault == _redemptionRequestId
REF_3583(uint64) -> agent_1 (-> ['TMP_5040']).activeTransferToCoreVault
TMP_5041(bool) = REF_3583 == _redemptionRequestId_1
CONDITION TMP_5041
 agent.activeTransferToCoreVault = 0
REF_3584(uint64) -> agent_1 (-> ['TMP_5040']).activeTransferToCoreVault
agent_2 (-> ['TMP_5040'])(Agent.State) := phi(["agent_1 (-> ['TMP_5040'])"])
REF_3584(uint64) (->agent_2 (-> ['TMP_5040'])) := 0(uint256)
TMP_5040(Agent.State) := phi(["agent_2 (-> ['TMP_5040'])"])
```

#### Liquidation._targetRatioBIPS(Agent.State,Collateral.Kind) [PRIVATE]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
_collateralKind_1(Collateral.Kind) := phi(['REF_3259', 'REF_3258', '_collateralKind_1'])
 collateral = _agent.getCollateral(_collateralKind)
TMP_4770(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getCollateral(Agent.State,Collateral.Kind), arguments:['_agent_1 (-> [])', '_collateralKind_1'] 
collateral_1 (-> ['TMP_4770'])(CollateralTypeInt.Data) := TMP_4770(CollateralTypeInt.Data)
 ! _agent.collateralUnderwater(_collateralKind)
TMP_4771(bool) = LIBRARY_CALL, dest:Agents, function:Agents.collateralUnderwater(Agent.State,Collateral.Kind), arguments:['_agent_1 (-> [])', '_collateralKind_1'] 
TMP_4772 = UnaryType.BANG TMP_4771 
CONDITION TMP_4772
 collateral.minCollateralRatioBIPS
REF_3284(uint32) -> collateral_1 (-> ['TMP_4770']).minCollateralRatioBIPS
RETURN REF_3284
 collateral.safetyMinCollateralRatioBIPS
REF_3285(uint32) -> collateral_1 (-> ['TMP_4770']).safetyMinCollateralRatioBIPS
RETURN REF_3285
```
#### Liquidation.getCollateralRatiosBIPS(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 (vaultCR,amgToC1WeiPrice) = getCollateralRatioBIPS(_agent,Collateral.Kind.VAULT)
REF_3268(Collateral.Kind) -> Kind.VAULT
TUPLE_55(uint256,uint256) = INTERNAL_CALL, Liquidation.getCollateralRatioBIPS(Agent.State,Collateral.Kind)(_agent_1 (-> []),REF_3268)
vaultCR_1(uint256)= UNPACK TUPLE_55 index: 0 
amgToC1WeiPrice_1(uint256)= UNPACK TUPLE_55 index: 1 
 (poolCR,amgToPoolWeiPrice) = getCollateralRatioBIPS(_agent,Collateral.Kind.POOL)
REF_3269(Collateral.Kind) -> Kind.POOL
TUPLE_56(uint256,uint256) = INTERNAL_CALL, Liquidation.getCollateralRatioBIPS(Agent.State,Collateral.Kind)(_agent_1 (-> []),REF_3269)
poolCR_1(uint256)= UNPACK TUPLE_56 index: 0 
amgToPoolWeiPrice_1(uint256)= UNPACK TUPLE_56 index: 1 
 CRData({vaultCR:vaultCR,poolCR:poolCR,amgToC1WeiPrice:amgToC1WeiPrice,amgToPoolWeiPrice:amgToPoolWeiPrice})
TMP_4757(Liquidation.CRData) = new CRData(vaultCR_1,poolCR_1,amgToC1WeiPrice_1,amgToPoolWeiPrice_1)
RETURN TMP_4757
```
#### PaymentReference.redemption(uint256) [INTERNAL]
```slithir
MAX_ID_2(uint256) := phi(['MAX_ID_0'])
REDEMPTION_1(uint256) := phi(['REDEMPTION_0'])
 assert(bool)(_id <= MAX_ID)
TMP_5349(bool) = _id_1 <= MAX_ID_2
TMP_5350(None) = SOLIDITY_CALL assert(bool)(TMP_5349)
 bytes32(_id | REDEMPTION)
TMP_5351(uint256) = _id_1 | REDEMPTION_1
TMP_5352 = CONVERT TMP_5351 to bytes32
RETURN TMP_5352
```
#### RedemptionTimeExtension.extendTimeForRedemption(address) [INTERNAL]
```slithir
 state = getState()
TMP_5433(RedemptionTimeExtension.State) = INTERNAL_CALL, RedemptionTimeExtension.getState()()
state_1 (-> ['TMP_5433'])(RedemptionTimeExtension.State) := TMP_5433(RedemptionTimeExtension.State)
 agentData = state.agents[_agentVault]
REF_3850(mapping(address => RedemptionTimeExtension.AgentTimeExtensionData)) -> state_1 (-> ['TMP_5433']).agents
REF_3851(RedemptionTimeExtension.AgentTimeExtensionData) -> REF_3850[_agentVault_1]
agentData_1 (-> ['state'])(RedemptionTimeExtension.AgentTimeExtensionData) := REF_3851(RedemptionTimeExtension.AgentTimeExtensionData)
 timestamp = block.timestamp.toUint64()
TMP_5434(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.timestamp'] 
timestamp_1(uint64) := TMP_5434(uint64)
 accumulatedTimestamp = agentData.extendedTimestamp + state.redemptionPaymentExtensionSeconds
REF_3853(uint64) -> agentData_1 (-> ['state']).extendedTimestamp
REF_3854(uint64) -> state_1 (-> ['TMP_5433']).redemptionPaymentExtensionSeconds
TMP_5435(uint64) = REF_3853 (c)+ REF_3854
accumulatedTimestamp_1(uint64) := TMP_5435(uint64)
 agentData.extendedTimestamp = SafeMath64.max64(accumulatedTimestamp,timestamp)
REF_3855(uint64) -> agentData_1 (-> ['state']).extendedTimestamp
TMP_5436(uint64) = LIBRARY_CALL, dest:SafeMath64, function:SafeMath64.max64(uint64,uint64), arguments:['accumulatedTimestamp_1', 'timestamp_1'] 
agentData_2 (-> ['state'])(RedemptionTimeExtension.AgentTimeExtensionData) := phi(["agentData_1 (-> ['state'])"])
REF_3855(uint64) (->agentData_2 (-> ['state'])) := TMP_5436(uint64)
state_2 (-> ['state'])(RedemptionTimeExtension.State) := phi(["agentData_2 (-> ['state'])"])
 agentData.extendedTimestamp - timestamp
REF_3857(uint64) -> agentData_2 (-> ['state']).extendedTimestamp
TMP_5437(uint64) = REF_3857 (c)- timestamp_1
RETURN TMP_5437
```
#### PaymentReference.randomizedIdSkip() [INTERNAL]
```slithir
ID_RANDOMIZATION_1(uint256) := phi(['ID_RANDOMIZATION_0'])
 uint64(block.number % ID_RANDOMIZATION + 1)
TMP_5382(uint256) = block.number % ID_RANDOMIZATION_1
TMP_5383(uint256) = TMP_5382 (c)+ 1
TMP_5384 = CONVERT TMP_5383 to uint64
RETURN TMP_5384
```
#### Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256) [INTERNAL]
```slithir
_tokenDecimals_1(uint256) := phi(['REF_3165', 'REF_3167'])
_tokenPrice_1(uint256) := phi(['tokenPrice_1'])
_tokenFtsoDecimals_1(uint256) := phi(['tokenFtsoDec_1'])
_assetPrice_1(uint256) := phi(['assetPrice_1'])
_assetFtsoDecimals_1(uint256) := phi(['assetFtsoDec_1'])
AMG_TOKEN_WEI_PRICE_SCALE_EXP_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_EXP_0'])
 settings = Globals.getSettings()
TMP_4665(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4665'])(AssetManagerSettings.Data) := TMP_4665(AssetManagerSettings.Data)
 expPlus = _tokenDecimals + _tokenFtsoDecimals + AMG_TOKEN_WEI_PRICE_SCALE_EXP
TMP_4666(uint256) = _tokenDecimals_1 (c)+ _tokenFtsoDecimals_1
TMP_4667(uint256) = TMP_4666 (c)+ AMG_TOKEN_WEI_PRICE_SCALE_EXP_1
expPlus_1(uint256) := TMP_4667(uint256)
 expMinus = settings.assetMintingDecimals + _assetFtsoDecimals
REF_3173(uint8) -> settings_1 (-> ['TMP_4665']).assetMintingDecimals
TMP_4668(uint8) = REF_3173 (c)+ _assetFtsoDecimals_1
expMinus_1(uint256) := TMP_4668(uint8)
 assert(bool)(expPlus >= expMinus)
TMP_4669(bool) = expPlus_1 >= expMinus_1
TMP_4670(None) = SOLIDITY_CALL assert(bool)(TMP_4669)
 _assetPrice.mulDiv(10 ** (expPlus - expMinus),_tokenPrice)
TMP_4671(uint256) = expPlus_1 (c)- expMinus_1
TMP_4672(uint256) = 10 (c)** TMP_4671
TMP_4673(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_assetPrice_1', 'TMP_4672', '_tokenPrice_1'] 
RETURN TMP_4673
```
#### Conversion.readFtsoPrice(string,bool) [INTERNAL]
```slithir
_symbol_1(string) := phi(['REF_3163', 'REF_3166', 'REF_3160'])
_fromTrustedProviders_1(bool) := phi(['_fromTrustedProviders_1'])
 settings = Globals.getSettings()
TMP_4663(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4663'])(AssetManagerSettings.Data) := TMP_4663(AssetManagerSettings.Data)
 priceReader = IPriceReader(settings.priceReader)
REF_3169(address) -> settings_1 (-> ['TMP_4663']).priceReader
TMP_4664 = CONVERT REF_3169 to IPriceReader
priceReader_1(IPriceReader) := TMP_4664(IPriceReader)
 _fromTrustedProviders
CONDITION _fromTrustedProviders_1
 priceReader.getPriceFromTrustedProviders(_symbol)
TUPLE_50(uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:priceReader_1(IPriceReader), function:getPriceFromTrustedProviders, arguments:['_symbol_1']  
RETURN TUPLE_50
 priceReader.getPrice(_symbol)
TUPLE_51(uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:priceReader_1(IPriceReader), function:getPrice, arguments:['_symbol_1']  
RETURN TUPLE_51
```
#### Agents.getWorkAddress(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
 Globals.getAgentOwnerRegistry().getWorkAddress(_agent.ownerManagementAddress)
TMP_4481(IAgentOwnerRegistry) = LIBRARY_CALL, dest:Globals, function:Globals.getAgentOwnerRegistry(), arguments:[] 
REF_2992(address) -> _agent_1 (-> []).ownerManagementAddress
TMP_4482(address) = HIGH_LEVEL_CALL, dest:TMP_4481(IAgentOwnerRegistry), function:getWorkAddress, arguments:['REF_2992']  
RETURN TMP_4482
```
#### Agents.collateralUnderwater(Agent.State,Collateral.Kind) [INTERNAL]
```slithir
 _kind == Collateral.Kind.VAULT
REF_3027(Collateral.Kind) -> Kind.VAULT
TMP_4521(bool) = _kind_1 == REF_3027
CONDITION TMP_4521
 (_agent.collateralsUnderwater & Agent.LF_VAULT) != 0
REF_3028(uint8) -> _agent_1 (-> []).collateralsUnderwater
REF_3029(uint8) -> Agent.LF_VAULT
TMP_4522(uint8) = REF_3028 & REF_3029
TMP_4523(bool) = TMP_4522 != 0
RETURN TMP_4523
 assert(bool)(_kind == Collateral.Kind.POOL)
REF_3030(Collateral.Kind) -> Kind.POOL
TMP_4524(bool) = _kind_1 == REF_3030
TMP_4525(None) = SOLIDITY_CALL assert(bool)(TMP_4524)
 (_agent.collateralsUnderwater & Agent.LF_POOL) != 0
REF_3031(uint8) -> _agent_1 (-> []).collateralsUnderwater
REF_3032(uint8) -> Agent.LF_POOL
TMP_4526(uint8) = REF_3031 & REF_3032
TMP_4527(bool) = TMP_4526 != 0
RETURN TMP_4527
```
#### Agents.getCollateral(Agent.State,Collateral.Kind) [INTERNAL]
```slithir
 assert(bool)(_kind != Collateral.Kind.AGENT_POOL)
REF_3018(Collateral.Kind) -> Kind.AGENT_POOL
TMP_4517(bool) = _kind_1 != REF_3018
TMP_4518(None) = SOLIDITY_CALL assert(bool)(TMP_4517)
 state = AssetManagerState.get()
TMP_4519(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4519'])(AssetManagerState.State) := TMP_4519(AssetManagerState.State)
 _kind == Collateral.Kind.VAULT
REF_3020(Collateral.Kind) -> Kind.VAULT
TMP_4520(bool) = _kind_1 == REF_3020
CONDITION TMP_4520
 state.collateralTokens[_agent.vaultCollateralIndex]
REF_3021(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4519']).collateralTokens
REF_3022(uint16) -> _agent_1 (-> []).vaultCollateralIndex
REF_3023(CollateralTypeInt.Data) -> REF_3021[REF_3022]
RETURN REF_3023
 state.collateralTokens[_agent.poolCollateralIndex]
REF_3024(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4519']).collateralTokens
REF_3025(uint16) -> _agent_1 (-> []).poolCollateralIndex
REF_3026(CollateralTypeInt.Data) -> REF_3024[REF_3025]
RETURN REF_3026
```
#### Liquidation.getCollateralRatioBIPS(Agent.State,Collateral.Kind) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_collateralKind_1(Collateral.Kind) := phi(['REF_3268', 'REF_3269'])
 (_data,_trustedData) = _collateralDataWithTrusted(_agent,_collateralKind)
TUPLE_57(Collateral.Data,Collateral.Data) = INTERNAL_CALL, Liquidation._collateralDataWithTrusted(Agent.State,Collateral.Kind)(_agent_1 (-> []),_collateralKind_1)
_data_1(Collateral.Data)= UNPACK TUPLE_57 index: 0 
_trustedData_1(Collateral.Data)= UNPACK TUPLE_57 index: 1 
 ratio = AgentCollateral.collateralRatioBIPS(_data,_agent)
TMP_4758(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.collateralRatioBIPS(Collateral.Data,Agent.State), arguments:['_data_1', '_agent_1 (-> [])'] 
ratio_1(uint256) := TMP_4758(uint256)
 ratioTrusted = AgentCollateral.collateralRatioBIPS(_trustedData,_agent)
TMP_4759(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.collateralRatioBIPS(Collateral.Data,Agent.State), arguments:['_trustedData_1', '_agent_1 (-> [])'] 
ratioTrusted_1(uint256) := TMP_4759(uint256)
 _amgToTokenWeiPrice = _data.amgToTokenWeiPrice
REF_3272(uint256) -> _data_1.amgToTokenWeiPrice
_amgToTokenWeiPrice_1(uint256) := REF_3272(uint256)
 _collateralRatioBIPS = Math.max(ratio,ratioTrusted)
TMP_4760(uint256) = LIBRARY_CALL, dest:Math, function:Math.max(uint256,uint256), arguments:['ratio_1', 'ratioTrusted_1'] 
_collateralRatioBIPS_1(uint256) := TMP_4760(uint256)
 (_collateralRatioBIPS,_amgToTokenWeiPrice)
RETURN _collateralRatioBIPS_1,_amgToTokenWeiPrice_1
```
