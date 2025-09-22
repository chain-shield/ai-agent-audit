










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
#### PaymentReference.randomizedIdSkip() [INTERNAL]
```slithir
ID_RANDOMIZATION_1(uint256) := phi(['ID_RANDOMIZATION_0'])
 uint64(block.number % ID_RANDOMIZATION + 1)
TMP_5382(uint256) = block.number % ID_RANDOMIZATION_1
TMP_5383(uint256) = TMP_5382 (c)+ 1
TMP_5384 = CONVERT TMP_5383 to uint64
RETURN TMP_5384
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
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
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
#### RedemptionTimeExtension.getState() [INTERNAL]
```slithir
STATE_POSITION_1(bytes32) := phi(['STATE_POSITION_0'])
 position = STATE_POSITION
position_1(bytes32) := STATE_POSITION_1(bytes32)
 _state = position
_state_1 (-> ['position'])(RedemptionTimeExtension.State) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
```
#### SafeMath64.max64(uint64,uint64) [INTERNAL]
```slithir
 a >= b
TMP_10504(bool) = a_1 >= b_1
CONDITION TMP_10504
 a
RETURN a_1
 b
RETURN b_1
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
