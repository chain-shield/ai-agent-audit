
### Storage layout (FAsset) 

```text
assetName string
assetSymbol string
cleanupBlockNumberManager address
assetManager address
__terminatedAt uint64
_name string
_symbol string
_decimals uint8
_deployer address
_initialized bool
_version uint16

```







### Storage layout (WNatMock) 

```text
governanceVP IGovernanceVotePower
delegations mapping(address => WNatMock.Delegation[])
delegators mapping(address => EnumerableSet.AddressSet)

```

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
#### Redemptions.burnFAssets(address,uint256) [INTERNAL]
```slithir
 Globals.getFAsset().burn(_owner,_amountUBA)
TMP_5022(IIFAsset) = LIBRARY_CALL, dest:Globals, function:Globals.getFAsset(), arguments:[] 
HIGH_LEVEL_CALL, dest:TMP_5022(IIFAsset), function:burn, arguments:['_owner_1', '_amountUBA_1']
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

#### Redemptions.payOrBurnExecutorFee(Redemption.Request) [INTERNAL]
```slithir
 executorFeeNatWei = _request.executorFeeNatGWei * Conversion.GWEI
REF_3559(uint64) -> _request_1 (-> []).executorFeeNatGWei
REF_3560(uint256) -> Conversion.GWEI
TMP_5024(uint64) = REF_3559 (c)* REF_3560
executorFeeNatWei_1(uint256) := TMP_5024(uint64)
 executorFeeNatWei > 0
TMP_5025(bool) = executorFeeNatWei_1 > 0
CONDITION TMP_5025
 _request.executorFeeNatGWei = 0
REF_3561(uint64) -> _request_1 (-> []).executorFeeNatGWei
_request_2 (-> [])(Redemption.Request) := phi(['_request_1 (-> [])'])
REF_3561(uint64) (->_request_2 (-> [])) := 0(uint256)
 msg.sender == _request.executor
REF_3562(address) -> _request_2 (-> []).executor
TMP_5026(bool) = msg.sender == REF_3562
CONDITION TMP_5026
 Transfers.depositWNat(Globals.getWNat(),_request.executor,executorFeeNatWei)
TMP_5027(IWNat) = LIBRARY_CALL, dest:Globals, function:Globals.getWNat(), arguments:[] 
REF_3565(address) -> _request_2 (-> []).executor
LIBRARY_CALL, dest:Transfers, function:Transfers.depositWNat(IWNat,address,uint256), arguments:['TMP_5027', 'REF_3565', 'executorFeeNatWei_1'] 
 Globals.getBurnAddress().transfer(executorFeeNatWei)
TMP_5029(address) = LIBRARY_CALL, dest:Globals, function:Globals.getBurnAddress(), arguments:[] 
Transfer dest:TMP_5029 value:executorFeeNatWei_1
```
#### Redemptions.reCreateRedemptionTicket(Agent.State,Redemption.Request) [INTERNAL]
```slithir
 AgentBacking.endRedeemingAssets(_agent,_request.valueAMG,_request.poolSelfClose)
REF_3574(uint64) -> _request_1 (-> []).valueAMG
REF_3575(bool) -> _request_1 (-> []).poolSelfClose
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.endRedeemingAssets(Agent.State,uint64,bool), arguments:['_agent_1 (-> [])', 'REF_3574', 'REF_3575'] 
 AgentBacking.createNewMinting(_agent,_request.valueAMG)
REF_3577(uint64) -> _request_1 (-> []).valueAMG
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.createNewMinting(Agent.State,uint64), arguments:['_agent_1 (-> [])', 'REF_3577']
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
#### Globals.getBurnAddress() [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4739(AssetManagerSettings.Data) = INTERNAL_CALL, Globals.getSettings()()
settings_1 (-> ['TMP_4739'])(AssetManagerSettings.Data) := TMP_4739(AssetManagerSettings.Data)
 settings.burnAddress
REF_3244(address) -> settings_1 (-> ['TMP_4739']).burnAddress
RETURN REF_3244
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
#### FAsset.burn(address,uint256) [EXTERNAL]
```slithir
 _burn(_owner,_amount)
INTERNAL_CALL, ERC20._burn(address,uint256)(_owner_1,_amount_1)
 onlyAssetManager()
MODIFIER_CALL, FAsset.onlyAssetManager()()
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
#### Globals.getWNat() [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4731(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4731'])(AssetManagerState.State) := TMP_4731(AssetManagerState.State)
 IWNat(address(state.collateralTokens[state.poolCollateralIndex].token))
REF_3231(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4731']).collateralTokens
REF_3232(uint16) -> state_1 (-> ['TMP_4731']).poolCollateralIndex
REF_3233(CollateralTypeInt.Data) -> REF_3231[REF_3232]
REF_3234(IERC20) -> REF_3233.token
TMP_4732 = CONVERT REF_3234 to address
TMP_4733 = CONVERT TMP_4732 to IWNat
RETURN TMP_4733
```
#### Transfers.depositWNat(IWNat,address,uint256) [INTERNAL]
```slithir
 _amount > 0
TMP_10540(bool) = _amount_1 > 0
CONDITION TMP_10540
 _wNat.depositTo{value: _amount}(_recipient)
HIGH_LEVEL_CALL, dest:_wNat_1(IWNat), function:depositTo, arguments:['_recipient_1'] value:_amount_1
```
#### AgentBacking.createNewMinting(Agent.State,uint64) [INTERNAL]
```slithir
 _agent.mintedAMG += _valueAMG
REF_2789(uint64) -> _agent_1 (-> []).mintedAMG
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2789(-> _agent_2 (-> [])) = REF_2789 (c)+ _valueAMG_1
 settings = Globals.getSettings()
TMP_4273(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4273'])(AssetManagerSettings.Data) := TMP_4273(AssetManagerSettings.Data)
 valueWithDustAMG = _agent.dustAMG + _valueAMG
REF_2791(uint64) -> _agent_2 (-> []).dustAMG
TMP_4274(uint64) = REF_2791 (c)+ _valueAMG_1
valueWithDustAMG_1(uint64) := TMP_4274(uint64)
 newDustAMG = valueWithDustAMG % settings.lotSizeAMG
REF_2792(uint64) -> settings_1 (-> ['TMP_4273']).lotSizeAMG
TMP_4275(uint64) = valueWithDustAMG_1 % REF_2792
newDustAMG_1(uint64) := TMP_4275(uint64)
 ticketValueAMG = valueWithDustAMG - newDustAMG
TMP_4276(uint64) = valueWithDustAMG_1 (c)- newDustAMG_1
ticketValueAMG_1(uint64) := TMP_4276(uint64)
 ticketValueAMG > 0
TMP_4277(bool) = ticketValueAMG_1 > 0
CONDITION TMP_4277
 createRedemptionTicket(_agent,ticketValueAMG)
INTERNAL_CALL, AgentBacking.createRedemptionTicket(Agent.State,uint64)(_agent_2 (-> []),ticketValueAMG_1)
 changeDust(_agent,newDustAMG)
INTERNAL_CALL, AgentBacking.changeDust(Agent.State,uint64)(_agent_2 (-> []),newDustAMG_1)
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
#### ERC20._burn(address,uint256) [INTERNAL]
```slithir
_balances_9(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
_totalSupply_5(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 require(bool,string)(account != address(0),ERC20: burn from the zero address)
TMP_216 = CONVERT 0 to address
TMP_217(bool) = account_1 != TMP_216
TMP_218(None) = SOLIDITY_CALL require(bool,string)(TMP_217,ERC20: burn from the zero address)
 _beforeTokenTransfer(account,address(0),amount)
TMP_219 = CONVERT 0 to address
INTERNAL_CALL, ERC20._beforeTokenTransfer(address,address,uint256)(account_1,TMP_219,amount_1)
 accountBalance = _balances[account]
REF_80(uint256) -> _balances_10[account_1]
accountBalance_1(uint256) := REF_80(uint256)
 require(bool,string)(accountBalance >= amount,ERC20: burn amount exceeds balance)
TMP_221(bool) = accountBalance_1 >= amount_1
TMP_222(None) = SOLIDITY_CALL require(bool,string)(TMP_221,ERC20: burn amount exceeds balance)
 _balances[account] = accountBalance - amount
REF_81(uint256) -> _balances_10[account_1]
TMP_223(uint256) = accountBalance_1 - amount_1
_balances_11(mapping(address => uint256)) := phi(['_balances_10'])
REF_81(uint256) (->_balances_11) := TMP_223(uint256)
 _totalSupply -= amount
_totalSupply_7(uint256) = _totalSupply_6 - amount_1
 Transfer(account,address(0),amount)
TMP_224 = CONVERT 0 to address
Emit Transfer(account_1,TMP_224,amount_1)
 _afterTokenTransfer(account,address(0),amount)
TMP_226 = CONVERT 0 to address
INTERNAL_CALL, ERC20._afterTokenTransfer(address,address,uint256)(account_1,TMP_226,amount_1)
```
#### WNatMock.depositTo(address) [PUBLIC]
```slithir
 _mint(_recipient,msg.value)
INTERNAL_CALL, ERC20._mint(address,uint256)(_recipient_1,msg.value)
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
#### ERC20._mint(address,uint256) [INTERNAL]
```slithir
_balances_6(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
_totalSupply_2(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 require(bool,string)(account != address(0),ERC20: mint to the zero address)
TMP_207 = CONVERT 0 to address
TMP_208(bool) = account_1 != TMP_207
TMP_209(None) = SOLIDITY_CALL require(bool,string)(TMP_208,ERC20: mint to the zero address)
 _beforeTokenTransfer(address(0),account,amount)
TMP_210 = CONVERT 0 to address
INTERNAL_CALL, ERC20._beforeTokenTransfer(address,address,uint256)(TMP_210,account_1,amount_1)
 _totalSupply += amount
_totalSupply_4(uint256) = _totalSupply_3 (c)+ amount_1
 _balances[account] += amount
REF_79(uint256) -> _balances_7[account_1]
_balances_8(mapping(address => uint256)) := phi(['_balances_7'])
REF_79(-> _balances_8) = REF_79 + amount_1
 Transfer(address(0),account,amount)
TMP_212 = CONVERT 0 to address
Emit Transfer(TMP_212,account_1,amount_1)
 _afterTokenTransfer(address(0),account,amount)
TMP_214 = CONVERT 0 to address
INTERNAL_CALL, ERC20._afterTokenTransfer(address,address,uint256)(TMP_214,account_1,amount_1)
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
