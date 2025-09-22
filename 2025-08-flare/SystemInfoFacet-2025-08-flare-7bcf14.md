









#### SystemInfoFacet._convertCollateralReservationStatus(CollateralReservation.Status) [PRIVATE]
```slithir
_status_1(CollateralReservation.Status) := phi(['REF_2604', 'REF_2621'])
 _status == CollateralReservation.Status.ACTIVE
REF_2642(CollateralReservation.Status) -> Status.ACTIVE
TMP_4112(bool) = _status_1 == REF_2642
CONDITION TMP_4112
 CollateralReservationInfo.Status.ACTIVE
REF_2643(CollateralReservationInfo.Status) -> Status.ACTIVE
RETURN REF_2643
 _status == CollateralReservation.Status.SUCCESSFUL
REF_2644(CollateralReservation.Status) -> Status.SUCCESSFUL
TMP_4113(bool) = _status_1 == REF_2644
CONDITION TMP_4113
 CollateralReservationInfo.Status.SUCCESSFUL
REF_2645(CollateralReservationInfo.Status) -> Status.SUCCESSFUL
RETURN REF_2645
 _status == CollateralReservation.Status.DEFAULTED
REF_2646(CollateralReservation.Status) -> Status.DEFAULTED
TMP_4114(bool) = _status_1 == REF_2646
CONDITION TMP_4114
 CollateralReservationInfo.Status.DEFAULTED
REF_2647(CollateralReservationInfo.Status) -> Status.DEFAULTED
RETURN REF_2647
 assert(bool)(_status == CollateralReservation.Status.EXPIRED)
REF_2648(CollateralReservation.Status) -> Status.EXPIRED
TMP_4115(bool) = _status_1 == REF_2648
TMP_4116(None) = SOLIDITY_CALL assert(bool)(TMP_4115)
 CollateralReservationInfo.Status.EXPIRED
REF_2649(CollateralReservationInfo.Status) -> Status.EXPIRED
RETURN REF_2649
```

#### SystemInfoFacet.agentRedemptionQueue(address,uint256,uint256) [EXTERNAL]
```slithir
 RedemptionQueueInfo.agentRedemptionQueue(_agentVault,_firstRedemptionTicketId,_pageSize)
TUPLE_38(RedemptionTicketInfo.Data[],uint256) = LIBRARY_CALL, dest:RedemptionQueueInfo, function:RedemptionQueueInfo.agentRedemptionQueue(address,uint256,uint256), arguments:['_agentVault_1', '_firstRedemptionTicketId_1', '_pageSize_1'] 
RETURN TUPLE_38
 (_queue,_nextRedemptionTicketId)
```
#### SystemInfoFacet.collateralReservationInfo(uint256) [EXTERNAL]
```slithir
 crtId = SafeCast.toUint64(_collateralReservationId)
TMP_4091(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_collateralReservationId_1'] 
crtId_1(uint64) := TMP_4091(uint64)
 crt = Minting.getCollateralReservation(crtId,false)
TMP_4092(CollateralReservation.Data) = LIBRARY_CALL, dest:Minting, function:Minting.getCollateralReservation(uint256,bool), arguments:['crtId_1', 'False'] 
crt_1 (-> ['TMP_4092'])(CollateralReservation.Data) := TMP_4092(CollateralReservation.Data)
 agent = Agent.get(crt.agentVault)
REF_2586(address) -> crt_1 (-> ['TMP_4092']).agentVault
TMP_4093(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_2586'] 
agent_1 (-> ['TMP_4093'])(Agent.State) := TMP_4093(Agent.State)
 crt.poolFeeShareBIPS > 0
REF_2587(uint16) -> crt_1 (-> ['TMP_4092']).poolFeeShareBIPS
TMP_4094(bool) = REF_2587 > 0
CONDITION TMP_4094
 CollateralReservationInfo.Data({collateralReservationId:crtId,agentVault:crt.agentVault,minter:crt.minter,paymentAddress:agent.underlyingAddressString,paymentReference:PaymentReference.minting(crtId),valueUBA:Conversion.convertAmgToUBA(crt.valueAMG),mintingFeeUBA:crt.underlyingFeeUBA,reservationFeeNatWei:crt.reservationFeeNatWei,poolFeeShareBIPS:crt.poolFeeShareBIPS - 1,firstUnderlyingBlock:crt.firstUnderlyingBlock,lastUnderlyingBlock:crt.lastUnderlyingBlock,lastUnderlyingTimestamp:crt.lastUnderlyingTimestamp,executor:crt.executor,executorFeeNatWei:crt.executorFeeNatGWei * Conversion.GWEI,status:_convertCollateralReservationStatus(crt.status)})
REF_2589(address) -> crt_1 (-> ['TMP_4092']).agentVault
REF_2590(address) -> crt_1 (-> ['TMP_4092']).minter
REF_2591(string) -> agent_1 (-> ['TMP_4093']).underlyingAddressString
TMP_4095(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.minting(uint256), arguments:['crtId_1'] 
REF_2594(uint64) -> crt_1 (-> ['TMP_4092']).valueAMG
TMP_4096(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_2594'] 
REF_2595(uint128) -> crt_1 (-> ['TMP_4092']).underlyingFeeUBA
REF_2596(uint128) -> crt_1 (-> ['TMP_4092']).reservationFeeNatWei
REF_2597(uint16) -> crt_1 (-> ['TMP_4092']).poolFeeShareBIPS
TMP_4097(uint16) = REF_2597 (c)- 1
REF_2598(uint64) -> crt_1 (-> ['TMP_4092']).firstUnderlyingBlock
REF_2599(uint64) -> crt_1 (-> ['TMP_4092']).lastUnderlyingBlock
REF_2600(uint64) -> crt_1 (-> ['TMP_4092']).lastUnderlyingTimestamp
REF_2601(address) -> crt_1 (-> ['TMP_4092']).executor
REF_2602(uint64) -> crt_1 (-> ['TMP_4092']).executorFeeNatGWei
REF_2603(uint256) -> Conversion.GWEI
TMP_4098(uint64) = REF_2602 (c)* REF_2603
REF_2604(CollateralReservation.Status) -> crt_1 (-> ['TMP_4092']).status
TMP_4099(CollateralReservationInfo.Status) = INTERNAL_CALL, SystemInfoFacet._convertCollateralReservationStatus(CollateralReservation.Status)(REF_2604)
TMP_4100(CollateralReservationInfo.Data) = new Data(crtId_1,REF_2589,REF_2590,REF_2591,TMP_4095,TMP_4096,REF_2595,REF_2596,TMP_4097,REF_2598,REF_2599,REF_2600,REF_2601,TMP_4098,TMP_4099)
RETURN TMP_4100
 CollateralReservationInfo.Data({collateralReservationId:crtId,agentVault:crt.agentVault,minter:crt.minter,paymentAddress:agent.underlyingAddressString,paymentReference:PaymentReference.minting(crtId),valueUBA:Conversion.convertAmgToUBA(crt.valueAMG),mintingFeeUBA:crt.underlyingFeeUBA,reservationFeeNatWei:crt.reservationFeeNatWei,poolFeeShareBIPS:agent.poolFeeShareBIPS,firstUnderlyingBlock:crt.firstUnderlyingBlock,lastUnderlyingBlock:crt.lastUnderlyingBlock,lastUnderlyingTimestamp:crt.lastUnderlyingTimestamp,executor:crt.executor,executorFeeNatWei:crt.executorFeeNatGWei * Conversion.GWEI,status:_convertCollateralReservationStatus(crt.status)})
REF_2606(address) -> crt_1 (-> ['TMP_4092']).agentVault
REF_2607(address) -> crt_1 (-> ['TMP_4092']).minter
REF_2608(string) -> agent_1 (-> ['TMP_4093']).underlyingAddressString
TMP_4101(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.minting(uint256), arguments:['crtId_1'] 
REF_2611(uint64) -> crt_1 (-> ['TMP_4092']).valueAMG
TMP_4102(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_2611'] 
REF_2612(uint128) -> crt_1 (-> ['TMP_4092']).underlyingFeeUBA
REF_2613(uint128) -> crt_1 (-> ['TMP_4092']).reservationFeeNatWei
REF_2614(uint16) -> agent_1 (-> ['TMP_4093']).poolFeeShareBIPS
REF_2615(uint64) -> crt_1 (-> ['TMP_4092']).firstUnderlyingBlock
REF_2616(uint64) -> crt_1 (-> ['TMP_4092']).lastUnderlyingBlock
REF_2617(uint64) -> crt_1 (-> ['TMP_4092']).lastUnderlyingTimestamp
REF_2618(address) -> crt_1 (-> ['TMP_4092']).executor
REF_2619(uint64) -> crt_1 (-> ['TMP_4092']).executorFeeNatGWei
REF_2620(uint256) -> Conversion.GWEI
TMP_4103(uint64) = REF_2619 (c)* REF_2620
REF_2621(CollateralReservation.Status) -> crt_1 (-> ['TMP_4092']).status
TMP_4104(CollateralReservationInfo.Status) = INTERNAL_CALL, SystemInfoFacet._convertCollateralReservationStatus(CollateralReservation.Status)(REF_2621)
TMP_4105(CollateralReservationInfo.Data) = new Data(crtId_1,REF_2606,REF_2607,REF_2608,TMP_4101,TMP_4102,REF_2612,REF_2613,REF_2614,REF_2615,REF_2616,REF_2617,REF_2618,TMP_4103,TMP_4104)
RETURN TMP_4105
```
#### SystemInfoFacet.controllerAttached() [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4088(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4088'])(AssetManagerState.State) := TMP_4088(AssetManagerState.State)
 state.attached
REF_2578(bool) -> state_1 (-> ['TMP_4088']).attached
RETURN REF_2578
```
#### SystemInfoFacet.mintingPaused() [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4089(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4089'])(AssetManagerState.State) := TMP_4089(AssetManagerState.State)
 state.mintingPausedAt != 0
REF_2580(uint64) -> state_1 (-> ['TMP_4089']).mintingPausedAt
TMP_4090(bool) = REF_2580 != 0
RETURN TMP_4090
```
#### SystemInfoFacet.redemptionQueue(uint256,uint256) [EXTERNAL]
```slithir
 RedemptionQueueInfo.redemptionQueue(_firstRedemptionTicketId,_pageSize)
TUPLE_37(RedemptionTicketInfo.Data[],uint256) = LIBRARY_CALL, dest:RedemptionQueueInfo, function:RedemptionQueueInfo.redemptionQueue(uint256,uint256), arguments:['_firstRedemptionTicketId_1', '_pageSize_1'] 
RETURN TUPLE_37
 (_queue,_nextRedemptionTicketId)
```
#### SystemInfoFacet.redemptionRequestInfo(uint256) [EXTERNAL]
```slithir
 requestId = SafeCast.toUint64(_redemptionRequestId)
TMP_4106(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_redemptionRequestId_1'] 
requestId_1(uint64) := TMP_4106(uint64)
 request = Redemptions.getRedemptionRequest(requestId,false)
TMP_4107(Redemption.Request) = LIBRARY_CALL, dest:Redemptions, function:Redemptions.getRedemptionRequest(uint256,bool), arguments:['requestId_1', 'False'] 
request_1 (-> ['TMP_4107'])(Redemption.Request) := TMP_4107(Redemption.Request)
 RedemptionRequestInfo.Data({redemptionRequestId:requestId,status:_convertRedemptionStatus(request.status),agentVault:request.agentVault,redeemer:request.redeemer,paymentAddress:request.redeemerUnderlyingAddressString,paymentReference:PaymentReference.redemption(requestId),valueUBA:request.underlyingValueUBA,feeUBA:request.underlyingFeeUBA,poolFeeShareBIPS:request.poolFeeShareBIPS,firstUnderlyingBlock:request.firstUnderlyingBlock,lastUnderlyingBlock:request.lastUnderlyingBlock,lastUnderlyingTimestamp:request.lastUnderlyingTimestamp,timestamp:request.timestamp,poolSelfClose:request.poolSelfClose,transferToCoreVault:request.transferToCoreVault,executor:request.executor,executorFeeNatWei:request.executorFeeNatGWei * Conversion.GWEI})
REF_2625(Redemption.Status) -> request_1 (-> ['TMP_4107']).status
TMP_4108(RedemptionRequestInfo.Status) = INTERNAL_CALL, SystemInfoFacet._convertRedemptionStatus(Redemption.Status)(REF_2625)
REF_2626(address) -> request_1 (-> ['TMP_4107']).agentVault
REF_2627(address) -> request_1 (-> ['TMP_4107']).redeemer
REF_2628(string) -> request_1 (-> ['TMP_4107']).redeemerUnderlyingAddressString
TMP_4109(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.redemption(uint256), arguments:['requestId_1'] 
REF_2630(uint128) -> request_1 (-> ['TMP_4107']).underlyingValueUBA
REF_2631(uint128) -> request_1 (-> ['TMP_4107']).underlyingFeeUBA
REF_2632(uint16) -> request_1 (-> ['TMP_4107']).poolFeeShareBIPS
REF_2633(uint64) -> request_1 (-> ['TMP_4107']).firstUnderlyingBlock
REF_2634(uint64) -> request_1 (-> ['TMP_4107']).lastUnderlyingBlock
REF_2635(uint64) -> request_1 (-> ['TMP_4107']).lastUnderlyingTimestamp
REF_2636(uint64) -> request_1 (-> ['TMP_4107']).timestamp
REF_2637(bool) -> request_1 (-> ['TMP_4107']).poolSelfClose
REF_2638(bool) -> request_1 (-> ['TMP_4107']).transferToCoreVault
REF_2639(address) -> request_1 (-> ['TMP_4107']).executor
REF_2640(uint64) -> request_1 (-> ['TMP_4107']).executorFeeNatGWei
REF_2641(uint256) -> Conversion.GWEI
TMP_4110(uint64) = REF_2640 (c)* REF_2641
TMP_4111(RedemptionRequestInfo.Data) = new Data(requestId_1,TMP_4108,REF_2626,REF_2627,REF_2628,TMP_4109,REF_2630,REF_2631,REF_2632,REF_2633,REF_2634,REF_2635,REF_2636,REF_2637,REF_2638,REF_2639,TMP_4110)
RETURN TMP_4111
```
#### RedemptionQueueInfo.agentRedemptionQueue(address,uint256,uint256) [INTERNAL]
```slithir
 Agent.get(_agentVault)
TMP_4904(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
 _getRedemptionQueue(_agentVault,_firstRedemptionTicketId,_pageSize)
TUPLE_62(RedemptionTicketInfo.Data[],uint256) = INTERNAL_CALL, RedemptionQueueInfo._getRedemptionQueue(address,uint256,uint256)(_agentVault_1,_firstRedemptionTicketId_1,_pageSize_1)
RETURN TUPLE_62
 (_queue,_nextRedemptionTicketId)
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
#### Minting.getCollateralReservation(uint256,bool) [INTERNAL]
```slithir
 require(bool,error)(_crtId > 0,revert InvalidCrtId()())
TMP_4828(bool) = _crtId_1 > 0
TMP_4829(None) = SOLIDITY_CALL revert InvalidCrtId()()
TMP_4830(None) = SOLIDITY_CALL require(bool,error)(TMP_4828,TMP_4829)
 state = AssetManagerState.get()
TMP_4831(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4831'])(AssetManagerState.State) := TMP_4831(AssetManagerState.State)
 _crt = state.crts[_crtId]
REF_3343(mapping(uint256 => CollateralReservation.Data)) -> state_1 (-> ['TMP_4831']).crts
REF_3344(CollateralReservation.Data) -> REF_3343[_crtId_1]
_crt_1 (-> ['state'])(CollateralReservation.Data) := REF_3344(CollateralReservation.Data)
 require(bool,error)(_crt.valueAMG != 0,revert InvalidCrtId()())
REF_3345(uint64) -> _crt_1 (-> ['state']).valueAMG
TMP_4832(bool) = REF_3345 != 0
TMP_4833(None) = SOLIDITY_CALL revert InvalidCrtId()()
TMP_4834(None) = SOLIDITY_CALL require(bool,error)(TMP_4832,TMP_4833)
 _requireActive
CONDITION _requireActive_1
 require(bool,error)(_crt.status == CollateralReservation.Status.ACTIVE,revert InvalidCrtId()())
REF_3346(CollateralReservation.Status) -> _crt_1 (-> ['state']).status
REF_3347(CollateralReservation.Status) -> Status.ACTIVE
TMP_4835(bool) = REF_3346 == REF_3347
TMP_4836(None) = SOLIDITY_CALL revert InvalidCrtId()()
TMP_4837(None) = SOLIDITY_CALL require(bool,error)(TMP_4835,TMP_4836)
 _crt
RETURN _crt_1 (-> ['state'])
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
#### PaymentReference.minting(uint256) [INTERNAL]
```slithir
MAX_ID_1(uint256) := phi(['MAX_ID_0'])
MINTING_1(uint256) := phi(['MINTING_0'])
 assert(bool)(_id <= MAX_ID)
TMP_5345(bool) = _id_1 <= MAX_ID_1
TMP_5346(None) = SOLIDITY_CALL assert(bool)(TMP_5345)
 bytes32(_id | MINTING)
TMP_5347(uint256) = _id_1 | MINTING_1
TMP_5348 = CONVERT TMP_5347 to bytes32
RETURN TMP_5348
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
#### RedemptionQueueInfo.redemptionQueue(uint256,uint256) [INTERNAL]
```slithir
 _getRedemptionQueue(address(0),_firstRedemptionTicketId,_pageSize)
TMP_4903 = CONVERT 0 to address
TUPLE_61(RedemptionTicketInfo.Data[],uint256) = INTERNAL_CALL, RedemptionQueueInfo._getRedemptionQueue(address,uint256,uint256)(TMP_4903,_firstRedemptionTicketId_1,_pageSize_1)
RETURN TUPLE_61
 (_queue,_nextRedemptionTicketId)
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

