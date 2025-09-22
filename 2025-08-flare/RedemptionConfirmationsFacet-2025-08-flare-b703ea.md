





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














### Storage layout (AgentOwnerRegistry) 

```text
manager address
whitelist mapping(address => bool)
workToMgmtAddress mapping(address => address)
mgmtToWorkAddress mapping(address => address)
agentName mapping(address => string)
agentDescription mapping(address => string)
agentIconUrl mapping(address => string)
agentTouUrl mapping(address => string)

```





#### RedemptionConfirmationsFacet._mintPoolFee(Agent.State,Redemption.Request,uint256) [PRIVATE]
```slithir
_agent_1 (-> ['TMP_3227'])(Agent.State) := phi(["agent_1 (-> ['TMP_3227'])"])
_request_1 (-> ['TMP_3226'])(Redemption.Request) := phi(["request_1 (-> ['TMP_3226'])"])
_redemptionRequestId_1(uint256) := phi(['_redemptionRequestId_1'])
 poolFeeUBA = uint256(_request.underlyingFeeUBA).mulBips(_request.poolFeeShareBIPS)
REF_2072(uint128) -> _request_1 (-> ['TMP_3226']).underlyingFeeUBA
TMP_3271 = CONVERT REF_2072 to uint256
REF_2074(uint16) -> _request_1 (-> ['TMP_3226']).poolFeeShareBIPS
TMP_3272(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_3271', 'REF_2074'] 
poolFeeUBA_1(uint256) := TMP_3272(uint256)
 poolFeeUBA = Conversion.roundUBAToAmg(poolFeeUBA)
TMP_3273(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.roundUBAToAmg(uint256), arguments:['poolFeeUBA_1'] 
poolFeeUBA_2(uint256) := TMP_3273(uint256)
 poolFeeUBA > 0
TMP_3274(bool) = poolFeeUBA_2 > 0
CONDITION TMP_3274
 AgentBacking.createNewMinting(_agent,Conversion.convertUBAToAmg(poolFeeUBA))
TMP_3275(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['poolFeeUBA_2'] 
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.createNewMinting(Agent.State,uint64), arguments:["_agent_1 (-> ['TMP_3227'])", 'TMP_3275'] 
 Globals.getFAsset().mint(address(_agent.collateralPool),poolFeeUBA)
TMP_3277(IIFAsset) = LIBRARY_CALL, dest:Globals, function:Globals.getFAsset(), arguments:[] 
REF_2080(IICollateralPool) -> _agent_1 (-> ['TMP_3227']).collateralPool
TMP_3278 = CONVERT REF_2080 to address
HIGH_LEVEL_CALL, dest:TMP_3277(IIFAsset), function:mint, arguments:['TMP_3278', 'poolFeeUBA_2']  
 _agent.collateralPool.fAssetFeeDeposited(poolFeeUBA)
REF_2081(IICollateralPool) -> _agent_1 (-> ['TMP_3227']).collateralPool
HIGH_LEVEL_CALL, dest:REF_2081(IICollateralPool), function:fAssetFeeDeposited, arguments:['poolFeeUBA_2']  
 IAssetManagerEvents.RedemptionPoolFeeMinted(_agent.vaultAddress(),_redemptionRequestId,poolFeeUBA)
TMP_3281(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:["_agent_1 (-> ['TMP_3227'])"] 
Emit RedemptionPoolFeeMinted(TMP_3281,_redemptionRequestId_1,poolFeeUBA_2)
```
#### RedemptionConfirmationsFacet._othersCanConfirmPayment(Redemption.Request) [PRIVATE]
```slithir
_request_1 (-> ['TMP_3226'])(Redemption.Request) := phi(["request_1 (-> ['TMP_3226'])"])
 settings = Globals.getSettings()
TMP_3283(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3283'])(AssetManagerSettings.Data) := TMP_3283(AssetManagerSettings.Data)
 block.timestamp > _request.timestamp + settings.confirmationByOthersAfterSeconds
REF_2086(uint64) -> _request_1 (-> ['TMP_3226']).timestamp
REF_2087(uint64) -> settings_1 (-> ['TMP_3283']).confirmationByOthersAfterSeconds
TMP_3284(uint64) = REF_2086 (c)+ REF_2087
TMP_3285(bool) = block.timestamp > TMP_3284
RETURN TMP_3285
```

#### RedemptionConfirmationsFacet.confirmRedemptionPayment(IPayment.Proof,uint256) [EXTERNAL]
```slithir
 request = Redemptions.getRedemptionRequest(_redemptionRequestId,true)
TMP_3226(Redemption.Request) = LIBRARY_CALL, dest:Redemptions, function:Redemptions.getRedemptionRequest(uint256,bool), arguments:['_redemptionRequestId_1', 'True'] 
request_1 (-> ['TMP_3226'])(Redemption.Request) := TMP_3226(Redemption.Request)
 agent = Agent.get(request.agentVault)
REF_1991(address) -> request_1 (-> ['TMP_3226']).agentVault
TMP_3227(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_1991'] 
agent_1 (-> ['TMP_3227'])(Agent.State) := TMP_3227(Agent.State)
 isAgent = Agents.isOwner(agent,msg.sender)
TMP_3228(bool) = LIBRARY_CALL, dest:Agents, function:Agents.isOwner(Agent.State,address), arguments:["agent_1 (-> ['TMP_3227'])", 'msg.sender'] 
isAgent_1(bool) := TMP_3228(bool)
 require(bool,error)(isAgent || _othersCanConfirmPayment(request),Agents.OnlyAgentVaultOwner())
TMP_3229(bool) = INTERNAL_CALL, RedemptionConfirmationsFacet._othersCanConfirmPayment(Redemption.Request)(request_1 (-> ['TMP_3226']))
TMP_3230(bool) = isAgent_1 || TMP_3229
TMP_3231(None) = SOLIDITY_CALL revert OnlyAgentVaultOwner()()
TMP_3232(None) = SOLIDITY_CALL require(bool,error)(TMP_3230,TMP_3231)
 TransactionAttestation.verifyPayment(_payment)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyPayment(IPayment.Proof), arguments:['_payment_1'] 
 require(bool,error)(_payment.data.responseBody.standardPaymentReference == PaymentReference.redemption(_redemptionRequestId),revert InvalidRedemptionReference()())
REF_1994(IPayment.Response) -> _payment_1.data
REF_1995(IPayment.ResponseBody) -> REF_1994.responseBody
REF_1996(bytes32) -> REF_1995.standardPaymentReference
TMP_3234(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.redemption(uint256), arguments:['_redemptionRequestId_1'] 
TMP_3235(bool) = REF_1996 == TMP_3234
TMP_3236(None) = SOLIDITY_CALL revert InvalidRedemptionReference()()
TMP_3237(None) = SOLIDITY_CALL require(bool,error)(TMP_3235,TMP_3236)
 require(bool,error)(_payment.data.responseBody.blockNumber >= request.firstUnderlyingBlock,revert RedemptionPaymentTooOld()())
REF_1998(IPayment.Response) -> _payment_1.data
REF_1999(IPayment.ResponseBody) -> REF_1998.responseBody
REF_2000(uint64) -> REF_1999.blockNumber
REF_2001(uint64) -> request_1 (-> ['TMP_3226']).firstUnderlyingBlock
TMP_3238(bool) = REF_2000 >= REF_2001
TMP_3239(None) = SOLIDITY_CALL revert RedemptionPaymentTooOld()()
TMP_3240(None) = SOLIDITY_CALL require(bool,error)(TMP_3238,TMP_3239)
 require(bool,error)(_payment.data.responseBody.sourceAddressHash == agent.underlyingAddressHash,revert SourceNotAgentsUnderlyingAddress()())
REF_2002(IPayment.Response) -> _payment_1.data
REF_2003(IPayment.ResponseBody) -> REF_2002.responseBody
REF_2004(bytes32) -> REF_2003.sourceAddressHash
REF_2005(bytes32) -> agent_1 (-> ['TMP_3227']).underlyingAddressHash
TMP_3241(bool) = REF_2004 == REF_2005
TMP_3242(None) = SOLIDITY_CALL revert SourceNotAgentsUnderlyingAddress()()
TMP_3243(None) = SOLIDITY_CALL require(bool,error)(TMP_3241,TMP_3242)
 require(bool,error)(_payment.data.responseBody.intendedReceivingAddressHash != agent.underlyingAddressHash,revert InvalidReceivingAddressSelected()())
REF_2006(IPayment.Response) -> _payment_1.data
REF_2007(IPayment.ResponseBody) -> REF_2006.responseBody
REF_2008(bytes32) -> REF_2007.intendedReceivingAddressHash
REF_2009(bytes32) -> agent_1 (-> ['TMP_3227']).underlyingAddressHash
TMP_3244(bool) = REF_2008 != REF_2009
TMP_3245(None) = SOLIDITY_CALL revert InvalidReceivingAddressSelected()()
TMP_3246(None) = SOLIDITY_CALL require(bool,error)(TMP_3244,TMP_3245)
 (paymentValid,failureReason) = _validatePayment(request,_payment)
TUPLE_33(bool,string) = INTERNAL_CALL, RedemptionConfirmationsFacet._validatePayment(Redemption.Request,IPayment.Proof)(request_1 (-> ['TMP_3226']),_payment_1)
paymentValid_1(bool)= UNPACK TUPLE_33 index: 0 
failureReason_1(string)= UNPACK TUPLE_33 index: 1 
 paymentValid
CONDITION paymentValid_1
 assert(bool)(request.status == Redemption.Status.ACTIVE)
REF_2010(Redemption.Status) -> request_1 (-> ['TMP_3226']).status
REF_2011(Redemption.Status) -> Status.ACTIVE
TMP_3247(bool) = REF_2010 == REF_2011
TMP_3248(None) = SOLIDITY_CALL assert(bool)(TMP_3247)
 AgentBacking.endRedeemingAssets(agent,request.valueAMG,request.poolSelfClose)
REF_2013(uint64) -> request_1 (-> ['TMP_3226']).valueAMG
REF_2014(bool) -> request_1 (-> ['TMP_3226']).poolSelfClose
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.endRedeemingAssets(Agent.State,uint64,bool), arguments:["agent_1 (-> ['TMP_3227'])", 'REF_2013', 'REF_2014'] 
 _payment.data.responseBody.status == TransactionAttestation.PAYMENT_SUCCESS
REF_2015(IPayment.Response) -> _payment_1.data
REF_2016(IPayment.ResponseBody) -> REF_2015.responseBody
REF_2017(uint8) -> REF_2016.status
REF_2018(uint8) -> TransactionAttestation.PAYMENT_SUCCESS
TMP_3250(bool) = REF_2017 == REF_2018
CONDITION TMP_3250
 finalStatus = Redemption.Status.SUCCESSFUL
REF_2019(Redemption.Status) -> Status.SUCCESSFUL
finalStatus_3(Redemption.Status) := REF_2019(Redemption.Status)
 IAssetManagerEvents.RedemptionPerformed(request.agentVault,request.redeemer,_redemptionRequestId,_payment.data.requestBody.transactionId,request.underlyingValueUBA,_payment.data.responseBody.spentAmount)
REF_2021(address) -> request_1 (-> ['TMP_3226']).agentVault
REF_2022(address) -> request_1 (-> ['TMP_3226']).redeemer
REF_2023(IPayment.Response) -> _payment_1.data
REF_2024(IPayment.RequestBody) -> REF_2023.requestBody
REF_2025(bytes32) -> REF_2024.transactionId
REF_2026(uint128) -> request_1 (-> ['TMP_3226']).underlyingValueUBA
REF_2027(IPayment.Response) -> _payment_1.data
REF_2028(IPayment.ResponseBody) -> REF_2027.responseBody
REF_2029(int256) -> REF_2028.spentAmount
Emit RedemptionPerformed(REF_2021,REF_2022,_redemptionRequestId_1,REF_2025,REF_2026,REF_2029)
 request.transferToCoreVault
REF_2030(bool) -> request_1 (-> ['TMP_3226']).transferToCoreVault
CONDITION REF_2030
 CoreVaultClient.confirmTransferToCoreVault(_payment,agent,_redemptionRequestId)
LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.confirmTransferToCoreVault(IPayment.Proof,Agent.State,uint256), arguments:['_payment_1', "agent_1 (-> ['TMP_3227'])", '_redemptionRequestId_1'] 
 assert(bool)(_payment.data.responseBody.status == TransactionAttestation.PAYMENT_BLOCKED)
REF_2032(IPayment.Response) -> _payment_1.data
REF_2033(IPayment.ResponseBody) -> REF_2032.responseBody
REF_2034(uint8) -> REF_2033.status
REF_2035(uint8) -> TransactionAttestation.PAYMENT_BLOCKED
TMP_3253(bool) = REF_2034 == REF_2035
TMP_3254(None) = SOLIDITY_CALL assert(bool)(TMP_3253)
 finalStatus = Redemption.Status.BLOCKED
REF_2036(Redemption.Status) -> Status.BLOCKED
finalStatus_2(Redemption.Status) := REF_2036(Redemption.Status)
 IAssetManagerEvents.RedemptionPaymentBlocked(request.agentVault,request.redeemer,_redemptionRequestId,_payment.data.requestBody.transactionId,request.underlyingValueUBA,_payment.data.responseBody.spentAmount)
REF_2038(address) -> request_1 (-> ['TMP_3226']).agentVault
REF_2039(address) -> request_1 (-> ['TMP_3226']).redeemer
REF_2040(IPayment.Response) -> _payment_1.data
REF_2041(IPayment.RequestBody) -> REF_2040.requestBody
REF_2042(bytes32) -> REF_2041.transactionId
REF_2043(uint128) -> request_1 (-> ['TMP_3226']).underlyingValueUBA
REF_2044(IPayment.Response) -> _payment_1.data
REF_2045(IPayment.ResponseBody) -> REF_2044.responseBody
REF_2046(int256) -> REF_2045.spentAmount
Emit RedemptionPaymentBlocked(REF_2038,REF_2039,_redemptionRequestId_1,REF_2042,REF_2043,REF_2046)
finalStatus_4(Redemption.Status) := phi(['finalStatus_3', 'finalStatus_2'])
 _mintPoolFee(agent,request,_redemptionRequestId)
INTERNAL_CALL, RedemptionConfirmationsFacet._mintPoolFee(Agent.State,Redemption.Request,uint256)(agent_1 (-> ['TMP_3227']),request_1 (-> ['TMP_3226']),_redemptionRequestId_1)
 finalStatus = Redemption.Status.FAILED
REF_2047(Redemption.Status) -> Status.FAILED
finalStatus_1(Redemption.Status) := REF_2047(Redemption.Status)
 request.status == Redemption.Status.ACTIVE
REF_2048(Redemption.Status) -> request_1 (-> ['TMP_3226']).status
REF_2049(Redemption.Status) -> Status.ACTIVE
TMP_3257(bool) = REF_2048 == REF_2049
CONDITION TMP_3257
 RedemptionDefaults.executeDefaultOrCancel(agent,request,_redemptionRequestId)
LIBRARY_CALL, dest:RedemptionDefaults, function:RedemptionDefaults.executeDefaultOrCancel(Agent.State,Redemption.Request,uint256), arguments:["agent_1 (-> ['TMP_3227'])", "request_1 (-> ['TMP_3226'])", '_redemptionRequestId_1'] 
 IAssetManagerEvents.RedemptionPaymentFailed(request.agentVault,request.redeemer,_redemptionRequestId,_payment.data.requestBody.transactionId,_payment.data.responseBody.spentAmount,failureReason)
REF_2052(address) -> request_1 (-> ['TMP_3226']).agentVault
REF_2053(address) -> request_1 (-> ['TMP_3226']).redeemer
REF_2054(IPayment.Response) -> _payment_1.data
REF_2055(IPayment.RequestBody) -> REF_2054.requestBody
REF_2056(bytes32) -> REF_2055.transactionId
REF_2057(IPayment.Response) -> _payment_1.data
REF_2058(IPayment.ResponseBody) -> REF_2057.responseBody
REF_2059(int256) -> REF_2058.spentAmount
Emit RedemptionPaymentFailed(REF_2052,REF_2053,_redemptionRequestId_1,REF_2056,REF_2059,failureReason_1)
finalStatus_5(Redemption.Status) := phi(['finalStatus_1', 'finalStatus_0'])
 UnderlyingBalance.updateBalance(agent,- _payment.data.responseBody.spentAmount)
REF_2061(IPayment.Response) -> _payment_1.data
REF_2062(IPayment.ResponseBody) -> REF_2061.responseBody
REF_2063(int256) -> REF_2062.spentAmount
TMP_3260(int256) = 0 (c)- REF_2063
LIBRARY_CALL, dest:UnderlyingBalance, function:UnderlyingBalance.updateBalance(Agent.State,int256), arguments:["agent_1 (-> ['TMP_3227'])", 'TMP_3260'] 
 state = AssetManagerState.get()
TMP_3262(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_3262'])(AssetManagerState.State) := TMP_3262(AssetManagerState.State)
 state.paymentConfirmations.confirmSourceDecreasingTransaction(_payment)
REF_2065(PaymentConfirmations.State) -> state_1 (-> ['TMP_3262']).paymentConfirmations
LIBRARY_CALL, dest:PaymentConfirmations, function:PaymentConfirmations.confirmSourceDecreasingTransaction(PaymentConfirmations.State,IPayment.Proof), arguments:['REF_2065', '_payment_1'] 
 ! isAgent
TMP_3264 = UnaryType.BANG isAgent_1 
CONDITION TMP_3264
 AgentPayout.payForConfirmationByOthers(agent,msg.sender)
LIBRARY_CALL, dest:AgentPayout, function:AgentPayout.payForConfirmationByOthers(Agent.State,address), arguments:["agent_1 (-> ['TMP_3227'])", 'msg.sender'] 
 Redemptions.burnExecutorFee(request)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.burnExecutorFee(Redemption.Request), arguments:["request_1 (-> ['TMP_3226'])"] 
 Liquidation.endLiquidationIfHealthy(agent)
LIBRARY_CALL, dest:Liquidation, function:Liquidation.endLiquidationIfHealthy(Agent.State), arguments:["agent_1 (-> ['TMP_3227'])"] 
 UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(_payment)
LIBRARY_CALL, dest:UnderlyingBlockUpdater, function:UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(IPayment.Proof), arguments:['_payment_1'] 
 Redemptions.finishRedemptionRequest(_redemptionRequestId,request,finalStatus)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.finishRedemptionRequest(uint256,Redemption.Request,Redemption.Status), arguments:['_redemptionRequestId_1', "request_1 (-> ['TMP_3226'])", 'finalStatus_5'] 
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
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
#### Conversion.roundUBAToAmg(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4643(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4643'])(AssetManagerSettings.Data) := TMP_4643(AssetManagerSettings.Data)
 _valueUBA - (_valueUBA % settings.assetMintingGranularityUBA)
REF_3150(uint64) -> settings_1 (-> ['TMP_4643']).assetMintingGranularityUBA
TMP_4644(uint256) = _valueUBA_1 % REF_3150
TMP_4645(uint256) = _valueUBA_1 (c)- TMP_4644
RETURN TMP_4645
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
#### IICollateralPool.fAssetFeeDeposited(uint256) [EXTERNAL]
```slithir

```
#### IIFAsset.mint(address,uint256) [EXTERNAL]
```slithir

```
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
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
#### CoreVaultClient.confirmTransferToCoreVault(IPayment.Proof,Agent.State,uint256) [INTERNAL]
```slithir
 state = getState()
TMP_4677(CoreVaultClient.State) = INTERNAL_CALL, CoreVaultClient.getState()()
state_1 (-> ['TMP_4677'])(CoreVaultClient.State) := TMP_4677(CoreVaultClient.State)
 state.coreVaultManager.confirmPayment(_payment)
REF_3177(IICoreVaultManager) -> state_1 (-> ['TMP_4677']).coreVaultManager
HIGH_LEVEL_CALL, dest:REF_3177(IICoreVaultManager), function:confirmPayment, arguments:['_payment_1']  
 receivedAmount = _payment.data.responseBody.receivedAmount.toUint256()
REF_3179(IPayment.Response) -> _payment_1.data
REF_3180(IPayment.ResponseBody) -> REF_3179.responseBody
REF_3181(int256) -> REF_3180.receivedAmount
TMP_4679(uint256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint256(int256), arguments:['REF_3181'] 
receivedAmount_1(uint256) := TMP_4679(uint256)
 ICoreVaultClient.TransferToCoreVaultSuccessful(_agent.vaultAddress(),_redemptionRequestId,receivedAmount)
TMP_4680(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
Emit TransferToCoreVaultSuccessful(TMP_4680,_redemptionRequestId_1,receivedAmount_1)
 onlyEnabled()
MODIFIER_CALL, CoreVaultClient.onlyEnabled()()
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
#### RedemptionDefaults.executeDefaultOrCancel(Agent.State,Redemption.Request,uint256) [INTERNAL]
```slithir
 assert(bool)(_request.status == Redemption.Status.ACTIVE)
REF_3363(Redemption.Status) -> _request_1 (-> []).status
REF_3364(Redemption.Status) -> Status.ACTIVE
TMP_4856(bool) = REF_3363 == REF_3364
TMP_4857(None) = SOLIDITY_CALL assert(bool)(TMP_4856)
 ! _request.transferToCoreVault
REF_3365(bool) -> _request_1 (-> []).transferToCoreVault
TMP_4858 = UnaryType.BANG REF_3365 
CONDITION TMP_4858
 (paidC1Wei,paidPoolWei) = _collateralAmountForRedemption(_agent,_request)
TUPLE_59(uint256,uint256) = INTERNAL_CALL, RedemptionDefaults._collateralAmountForRedemption(Agent.State,Redemption.Request)(_agent_1 (-> []),_request_1 (-> []))
paidC1Wei_1(uint256)= UNPACK TUPLE_59 index: 0 
paidPoolWei_1(uint256)= UNPACK TUPLE_59 index: 1 
 (successVault,None) = AgentPayout.tryPayoutFromVault(_agent,_request.redeemer,paidC1Wei)
REF_3367(address) -> _request_1 (-> []).redeemer
TUPLE_60(bool,uint256) = LIBRARY_CALL, dest:AgentPayout, function:AgentPayout.tryPayoutFromVault(Agent.State,address,uint256), arguments:['_agent_1 (-> [])', 'REF_3367', 'paidC1Wei_1'] 
successVault_1(bool)= UNPACK TUPLE_60 index: 0 
 ! successVault
TMP_4859 = UnaryType.BANG successVault_1 
CONDITION TMP_4859
 paidPoolWei = _replaceFailedVaultPaymentWithPool(_agent,_request,paidC1Wei,paidPoolWei)
TMP_4860(uint256) = INTERNAL_CALL, RedemptionDefaults._replaceFailedVaultPaymentWithPool(Agent.State,Redemption.Request,uint256,uint256)(_agent_1 (-> []),_request_1 (-> []),paidC1Wei_1,paidPoolWei_1)
paidPoolWei_2(uint256) := TMP_4860(uint256)
 paidC1Wei = 0
paidC1Wei_2(uint256) := 0(uint256)
paidC1Wei_3(uint256) := phi(['paidC1Wei_1', 'paidC1Wei_2'])
paidPoolWei_3(uint256) := phi(['paidPoolWei_1', 'paidPoolWei_2'])
 paidPoolWei > 0
TMP_4861(bool) = paidPoolWei_3 > 0
CONDITION TMP_4861
 AgentPayout.payoutFromPool(_agent,_request.redeemer,paidPoolWei,paidPoolWei)
REF_3369(address) -> _request_1 (-> []).redeemer
TMP_4862(uint256) = LIBRARY_CALL, dest:AgentPayout, function:AgentPayout.payoutFromPool(Agent.State,address,uint256,uint256), arguments:['_agent_1 (-> [])', 'REF_3369', 'paidPoolWei_3', 'paidPoolWei_3'] 
 AgentBacking.endRedeemingAssets(_agent,_request.valueAMG,_request.poolSelfClose)
REF_3371(uint64) -> _request_1 (-> []).valueAMG
REF_3372(bool) -> _request_1 (-> []).poolSelfClose
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.endRedeemingAssets(Agent.State,uint64,bool), arguments:['_agent_1 (-> [])', 'REF_3371', 'REF_3372'] 
 IAssetManagerEvents.RedemptionDefault(_agent.vaultAddress(),_request.redeemer,_redemptionRequestId,_request.underlyingValueUBA,paidC1Wei,paidPoolWei)
TMP_4864(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
REF_3375(address) -> _request_1 (-> []).redeemer
REF_3376(uint128) -> _request_1 (-> []).underlyingValueUBA
Emit RedemptionDefault(TMP_4864,REF_3375,_redemptionRequestId_1,REF_3376,paidC1Wei_3,paidPoolWei_3)
 IAssetManagerEvents.RedemptionDefault(_agent.vaultAddress(),_request.redeemer,_redemptionRequestId,_request.underlyingValueUBA,0,0)
TMP_4866(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
REF_3379(address) -> _request_1 (-> []).redeemer
REF_3380(uint128) -> _request_1 (-> []).underlyingValueUBA
Emit RedemptionDefault(TMP_4866,REF_3379,_redemptionRequestId_1,REF_3380,0,0)
 CoreVaultClient.cancelTransferToCoreVault(_agent,_request,_redemptionRequestId)
LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.cancelTransferToCoreVault(Agent.State,Redemption.Request,uint256), arguments:['_agent_1 (-> [])', '_request_1 (-> [])', '_redemptionRequestId_1']
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
#### TransactionAttestation.verifyPayment(IPayment.Proof) [INTERNAL]
```slithir
_proof_1(IPayment.Proof) := phi(['_proof_1'])
 _settings = Globals.getSettings()
TMP_5237(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5237'])(AssetManagerSettings.Data) := TMP_5237(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3670(address) -> _settings_1 (-> ['TMP_5237']).fdcVerification
TMP_5238 = CONVERT REF_3670 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5238(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3671(IPayment.Response) -> _proof_1.data
REF_3672(bytes32) -> REF_3671.sourceId
REF_3673(bytes32) -> _settings_1 (-> ['TMP_5237']).chainId
TMP_5239(bool) = REF_3672 == REF_3673
TMP_5240(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5241(None) = SOLIDITY_CALL require(bool,error)(TMP_5239,TMP_5240)
 require(bool,error)(fdcVerification.verifyPayment(_proof),revert LegalPaymentNotProven()())
TMP_5242(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyPayment, arguments:['_proof_1']  
TMP_5243(None) = SOLIDITY_CALL revert LegalPaymentNotProven()()
TMP_5244(None) = SOLIDITY_CALL require(bool,error)(TMP_5242,TMP_5243)
```
#### UnderlyingBalance.updateBalance(Agent.State,int256) [INTERNAL]
```slithir
 newBalance = _agent.underlyingBalanceUBA + _balanceChange
REF_3699(int128) -> _agent_1 (-> []).underlyingBalanceUBA
TMP_5277(int128) = REF_3699 (c)+ _balanceChange_1
newBalance_1(int256) := TMP_5277(int128)
 requiredBalance = requiredUnderlyingUBA(_agent)
TMP_5278(uint256) = INTERNAL_CALL, UnderlyingBalance.requiredUnderlyingUBA(Agent.State)(_agent_1 (-> []))
requiredBalance_1(uint256) := TMP_5278(uint256)
 newBalance < requiredBalance.toInt256()
TMP_5279(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['requiredBalance_1'] 
TMP_5280(bool) = newBalance_1 < TMP_5279
CONDITION TMP_5280
 IAssetManagerEvents.UnderlyingBalanceTooLow(_agent.vaultAddress(),newBalance,requiredBalance)
TMP_5281(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
Emit UnderlyingBalanceTooLow(TMP_5281,newBalance_1,requiredBalance_1)
 Liquidation.startFullLiquidation(_agent)
LIBRARY_CALL, dest:Liquidation, function:Liquidation.startFullLiquidation(Agent.State), arguments:['_agent_1 (-> [])'] 
 _agent.underlyingBalanceUBA = newBalance.toInt128()
REF_3704(int128) -> _agent_1 (-> []).underlyingBalanceUBA
TMP_5284(int128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt128(int256), arguments:['newBalance_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_3704(int128) (->_agent_2 (-> [])) := TMP_5284(int128)
 IAssetManagerEvents.UnderlyingBalanceChanged(_agent.vaultAddress(),_agent.underlyingBalanceUBA)
TMP_5285(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_2 (-> [])'] 
REF_3708(int128) -> _agent_2 (-> []).underlyingBalanceUBA
Emit UnderlyingBalanceChanged(TMP_5285,REF_3708)
```
#### UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(IPayment.Proof) [INTERNAL]
```slithir
 updateCurrentBlock(_proof.data.responseBody.blockNumber,_proof.data.responseBody.blockTimestamp,1)
REF_3729(IPayment.Response) -> _proof_1.data
REF_3730(IPayment.ResponseBody) -> REF_3729.responseBody
REF_3731(uint64) -> REF_3730.blockNumber
REF_3732(IPayment.Response) -> _proof_1.data
REF_3733(IPayment.ResponseBody) -> REF_3732.responseBody
REF_3734(uint64) -> REF_3733.blockTimestamp
INTERNAL_CALL, UnderlyingBlockUpdater.updateCurrentBlock(uint64,uint64,uint64)(REF_3731,REF_3734,1)
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
#### PaymentConfirmations.confirmSourceDecreasingTransaction(PaymentConfirmations.State,IPayment.Proof) [INTERNAL]
```slithir
 txKey = transactionKey(_payment.data.responseBody.sourceAddressHash,_payment.data.requestBody.transactionId)
REF_3756(IPayment.Response) -> _payment_1.data
REF_3757(IPayment.ResponseBody) -> REF_3756.responseBody
REF_3758(bytes32) -> REF_3757.sourceAddressHash
REF_3759(IPayment.Response) -> _payment_1.data
REF_3760(IPayment.RequestBody) -> REF_3759.requestBody
REF_3761(bytes32) -> REF_3760.transactionId
TMP_5336(bytes32) = INTERNAL_CALL, PaymentConfirmations.transactionKey(bytes32,bytes32)(REF_3758,REF_3761)
txKey_1(bytes32) := TMP_5336(bytes32)
 _recordPaymentVerification(_state,txKey)
INTERNAL_CALL, PaymentConfirmations._recordPaymentVerification(PaymentConfirmations.State,bytes32)(_state_1 (-> []),txKey_1)
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
#### Agents.convertUSD5ToVaultCollateralWei(Agent.State,uint256) [INTERNAL]
```slithir
 Conversion.convertFromUSD5(_amountUSD5,getVaultCollateral(_agent))
TMP_4511(CollateralTypeInt.Data) = INTERNAL_CALL, Agents.getVaultCollateral(Agent.State)(_agent_1 (-> []))
TMP_4512(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertFromUSD5(uint256,CollateralTypeInt.Data), arguments:['_amountUSD5_1', 'TMP_4511'] 
RETURN TMP_4512
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
#### CoreVaultClient.getState() [INTERNAL]
```slithir
STATE_POSITION_1(bytes32) := phi(['STATE_POSITION_0'])
 position = STATE_POSITION
position_1(bytes32) := STATE_POSITION_1(bytes32)
 _state = position
_state_1 (-> ['position'])(CoreVaultClient.State) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
```
#### SafeCast.toUint256(int256) [INTERNAL]
```slithir
 require(bool,string)(value >= 0,SafeCast: value must be positive)
TMP_786(bool) = value_1 >= 0
TMP_787(None) = SOLIDITY_CALL require(bool,string)(TMP_786,SafeCast: value must be positive)
 uint256(value)
TMP_788 = CONVERT value_1 to uint256
RETURN TMP_788
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
#### AgentPayout.payoutFromPool(Agent.State,address,uint256,uint256) [INTERNAL]
```slithir
 poolBalance = _agent.collateralPool.totalCollateral()
REF_2922(IICollateralPool) -> _agent_1 (-> []).collateralPool
TMP_4394(uint256) = HIGH_LEVEL_CALL, dest:REF_2922(IICollateralPool), function:totalCollateral, arguments:[]  
poolBalance_1(uint256) := TMP_4394(uint256)
 _amountPaid = Math.min(_amountWei,poolBalance)
TMP_4395(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_amountWei_1', 'poolBalance_1'] 
_amountPaid_1(uint256) := TMP_4395(uint256)
 _agentResponsibilityWei = Math.min(_agentResponsibilityWei,_amountPaid)
TMP_4396(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_agentResponsibilityWei_1', '_amountPaid_1'] 
_agentResponsibilityWei_2(uint256) := TMP_4396(uint256)
 _agent.collateralPool.payout(_receiver,_amountPaid,_agentResponsibilityWei)
REF_2926(IICollateralPool) -> _agent_1 (-> []).collateralPool
HIGH_LEVEL_CALL, dest:REF_2926(IICollateralPool), function:payout, arguments:['_receiver_1', '_amountPaid_1', '_agentResponsibilityWei_2']  
 _amountPaid
RETURN _amountPaid_1
```
#### AgentPayout.tryPayoutFromVault(Agent.State,address,uint256) [INTERNAL]
```slithir
 collateral = Agents.getVaultCollateral(_agent)
TMP_4387(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4387'])(CollateralTypeInt.Data) := TMP_4387(CollateralTypeInt.Data)
 vault = IIAgentVault(_agent.vaultAddress())
TMP_4388(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4389 = CONVERT TMP_4388 to IIAgentVault
vault_1(IIAgentVault) := TMP_4389(IIAgentVault)
 _amountPaid = Math.min(_amountWei,collateral.token.balanceOf(address(vault)))
REF_2918(IERC20) -> collateral_1 (-> ['TMP_4387']).token
TMP_4390 = CONVERT vault_1 to address
TMP_4391(uint256) = HIGH_LEVEL_CALL, dest:REF_2918(IERC20), function:balanceOf, arguments:['TMP_4390']  
TMP_4392(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_amountWei_1', 'TMP_4391'] 
_amountPaid_1(uint256) := TMP_4392(uint256)
 vault.payout(collateral.token,_receiver,_amountPaid)
REF_2921(IERC20) -> collateral_1 (-> ['TMP_4387']).token
HIGH_LEVEL_CALL, dest:vault_1(IIAgentVault), function:payout, arguments:['REF_2921', '_receiver_1', '_amountPaid_1']  
 _success = true
_success_1(bool) := True(bool)
 _success = false
_success_3(bool) := False(bool)
 _amountPaid = 0
_amountPaid_3(uint256) := 0(uint256)
 (_success,_amountPaid)
_success_2(bool) := phi(['_success_1', '_success_3'])
_amountPaid_2(uint256) := phi(['_amountPaid_3', '_amountPaid_1'])
RETURN _success_2,_amountPaid_2
```
#### CoreVaultClient.cancelTransferToCoreVault(Agent.State,Redemption.Request,uint256) [INTERNAL]
```slithir
 Redemptions.releaseTransferToCoreVault(_redemptionRequestId,_request)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.releaseTransferToCoreVault(uint256,Redemption.Request), arguments:['_redemptionRequestId_1', '_request_1 (-> [])'] 
 Redemptions.reCreateRedemptionTicket(_agent,_request)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.reCreateRedemptionTicket(Agent.State,Redemption.Request), arguments:['_agent_1 (-> [])', '_request_1 (-> [])'] 
 ICoreVaultClient.TransferToCoreVaultDefaulted(_agent.vaultAddress(),_redemptionRequestId,_request.underlyingValueUBA)
TMP_4685(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
REF_3189(uint128) -> _request_1 (-> []).underlyingValueUBA
Emit TransferToCoreVaultDefaulted(TMP_4685,_redemptionRequestId_1,REF_3189)
 onlyEnabled()
MODIFIER_CALL, CoreVaultClient.onlyEnabled()()
```

#### RedemptionDefaults._replaceFailedVaultPaymentWithPool(Agent.State,Redemption.Request,uint256,uint256) [PRIVATE]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_request_1 (-> [])(Redemption.Request) := phi(['_request_1 (-> [])'])
_paidC1Wei_1(uint256) := phi(['paidC1Wei_1'])
_paidPoolWei_1(uint256) := phi(['paidPoolWei_1'])
 cd = AgentCollateral.combinedData(_agent)
TMP_4869(Collateral.CombinedData) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.combinedData(Agent.State), arguments:['_agent_1 (-> [])'] 
cd_1(Collateral.CombinedData) := TMP_4869(Collateral.CombinedData)
 poolTokenEquiv = _paidC1Wei.mulDiv(cd.agentPoolTokens.amgToTokenWeiPrice,cd.agentCollateral.amgToTokenWeiPrice)
REF_3384(Collateral.Data) -> cd_1.agentPoolTokens
REF_3385(uint256) -> REF_3384.amgToTokenWeiPrice
REF_3386(Collateral.Data) -> cd_1.agentCollateral
REF_3387(uint256) -> REF_3386.amgToTokenWeiPrice
TMP_4870(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_paidC1Wei_1', 'REF_3385', 'REF_3387'] 
poolTokenEquiv_1(uint256) := TMP_4870(uint256)
 requiredPoolTokensForRemainder = uint256(_agent.reservedAMG + _agent.mintedAMG + _agent.redeemingAMG - _request.valueAMG).mulDiv(cd.agentPoolTokens.amgToTokenWeiPrice,Conversion.AMG_TOKEN_WEI_PRICE_SCALE).mulBips(Globals.getSettings().mintingPoolHoldingsRequiredBIPS)
REF_3388(uint64) -> _agent_1 (-> []).reservedAMG
REF_3389(uint64) -> _agent_1 (-> []).mintedAMG
TMP_4871(uint64) = REF_3388 (c)+ REF_3389
REF_3390(uint64) -> _agent_1 (-> []).redeemingAMG
TMP_4872(uint64) = TMP_4871 (c)+ REF_3390
REF_3391(uint64) -> _request_1 (-> []).valueAMG
TMP_4873(uint64) = TMP_4872 (c)- REF_3391
TMP_4874 = CONVERT TMP_4873 to uint256
REF_3393(Collateral.Data) -> cd_1.agentPoolTokens
REF_3394(uint256) -> REF_3393.amgToTokenWeiPrice
REF_3395(uint256) -> Conversion.AMG_TOKEN_WEI_PRICE_SCALE
TMP_4875(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['TMP_4874', 'REF_3394', 'REF_3395'] 
TMP_4876(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_3398(uint32) -> TMP_4876.mintingPoolHoldingsRequiredBIPS
TMP_4877(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_4875', 'REF_3398'] 
requiredPoolTokensForRemainder_1(uint256) := TMP_4877(uint256)
 require(bool,error)(requiredPoolTokensForRemainder + poolTokenEquiv <= cd.agentPoolTokens.fullCollateral,revert NotEnoughAgentPoolTokensToCoverFailedVaultPayment()())
TMP_4878(uint256) = requiredPoolTokensForRemainder_1 (c)+ poolTokenEquiv_1
REF_3399(Collateral.Data) -> cd_1.agentPoolTokens
REF_3400(uint256) -> REF_3399.fullCollateral
TMP_4879(bool) = TMP_4878 <= REF_3400
TMP_4880(None) = SOLIDITY_CALL revert NotEnoughAgentPoolTokensToCoverFailedVaultPayment()()
TMP_4881(None) = SOLIDITY_CALL require(bool,error)(TMP_4879,TMP_4880)
 poolWeiEquiv = _paidC1Wei.mulDiv(cd.poolCollateral.amgToTokenWeiPrice,cd.agentCollateral.amgToTokenWeiPrice)
REF_3402(Collateral.Data) -> cd_1.poolCollateral
REF_3403(uint256) -> REF_3402.amgToTokenWeiPrice
REF_3404(Collateral.Data) -> cd_1.agentCollateral
REF_3405(uint256) -> REF_3404.amgToTokenWeiPrice
TMP_4882(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_paidC1Wei_1', 'REF_3403', 'REF_3405'] 
poolWeiEquiv_1(uint256) := TMP_4882(uint256)
 combinedPaidPoolWei = _paidPoolWei + poolWeiEquiv
TMP_4883(uint256) = _paidPoolWei_1 (c)+ poolWeiEquiv_1
combinedPaidPoolWei_1(uint256) := TMP_4883(uint256)
 require(bool,error)(combinedPaidPoolWei <= cd.poolCollateral.maxRedemptionCollateral(_agent,_request.valueAMG),revert NotEnoughPoolCollateralToCoverFailedVaultPayment()())
REF_3406(Collateral.Data) -> cd_1.poolCollateral
REF_3408(uint64) -> _request_1 (-> []).valueAMG
TMP_4884(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.maxRedemptionCollateral(Collateral.Data,Agent.State,uint256), arguments:['REF_3406', '_agent_1 (-> [])', 'REF_3408'] 
TMP_4885(bool) = combinedPaidPoolWei_1 <= TMP_4884
TMP_4886(None) = SOLIDITY_CALL revert NotEnoughPoolCollateralToCoverFailedVaultPayment()()
TMP_4887(None) = SOLIDITY_CALL require(bool,error)(TMP_4885,TMP_4886)
 combinedPaidPoolWei
RETURN combinedPaidPoolWei_1
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

#### Liquidation.startFullLiquidation(Agent.State) [INTERNAL]
```slithir
 _agent.status == Agent.Status.FULL_LIQUIDATION || _agent.status == Agent.Status.DESTROYING
REF_3245(Agent.Status) -> _agent_1 (-> []).status
REF_3246(Agent.Status) -> Status.FULL_LIQUIDATION
TMP_4741(bool) = REF_3245 == REF_3246
REF_3247(Agent.Status) -> _agent_1 (-> []).status
REF_3248(Agent.Status) -> Status.DESTROYING
TMP_4742(bool) = REF_3247 == REF_3248
TMP_4743(bool) = TMP_4741 || TMP_4742
CONDITION TMP_4743
 _agent.liquidationStartedAt == 0
REF_3249(uint64) -> _agent_1 (-> []).liquidationStartedAt
TMP_4744(bool) = REF_3249 == 0
CONDITION TMP_4744
 _agent.liquidationStartedAt = block.timestamp.toUint64()
REF_3250(uint64) -> _agent_1 (-> []).liquidationStartedAt
TMP_4745(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.timestamp'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_3250(uint64) (->_agent_2 (-> [])) := TMP_4745(uint64)
_agent_3 (-> [])(Agent.State) := phi(['_agent_2 (-> [])', '_agent_1 (-> [])'])
 _agent.status = Agent.Status.FULL_LIQUIDATION
REF_3252(Agent.Status) -> _agent_3 (-> []).status
REF_3253(Agent.Status) -> Status.FULL_LIQUIDATION
_agent_4 (-> [])(Agent.State) := phi(['_agent_3 (-> [])'])
REF_3252(Agent.Status) (->_agent_4 (-> [])) := REF_3253(Agent.Status)
 IAssetManagerEvents.FullLiquidationStarted(_agent.vaultAddress(),block.timestamp)
TMP_4746(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_4 (-> [])'] 
Emit FullLiquidationStarted(TMP_4746,block.timestamp)
```

#### SafeCast.toInt128(int256) [INTERNAL]
```slithir
 downcasted = int128(value)
TMP_834 = CONVERT value_1 to int128
downcasted_1(int128) := TMP_834(int128)
 require(bool,string)(downcasted == value,SafeCast: value doesn't fit in 128 bits)
TMP_835(bool) = downcasted_1 == value_1
TMP_836(None) = SOLIDITY_CALL require(bool,string)(TMP_835,SafeCast: value doesn't fit in 128 bits)
 downcasted
RETURN downcasted_1
```

#### UnderlyingBlockUpdater.updateCurrentBlock(IConfirmedBlockHeightExists.Proof) [INTERNAL]
```slithir
 TransactionAttestation.verifyConfirmedBlockHeightExists(_proof)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyConfirmedBlockHeightExists(IConfirmedBlockHeightExists.Proof), arguments:['_proof_1'] 
 updateCurrentBlock(_proof.data.requestBody.blockNumber,_proof.data.responseBody.blockTimestamp,_proof.data.responseBody.numberOfConfirmations)
REF_3720(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_3721(IConfirmedBlockHeightExists.RequestBody) -> REF_3720.requestBody
REF_3722(uint64) -> REF_3721.blockNumber
REF_3723(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_3724(IConfirmedBlockHeightExists.ResponseBody) -> REF_3723.responseBody
REF_3725(uint64) -> REF_3724.blockTimestamp
REF_3726(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_3727(IConfirmedBlockHeightExists.ResponseBody) -> REF_3726.responseBody
REF_3728(uint64) -> REF_3727.numberOfConfirmations
INTERNAL_CALL, UnderlyingBlockUpdater.updateCurrentBlock(uint64,uint64,uint64)(REF_3722,REF_3725,REF_3728)
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

#### PaymentConfirmations.transactionKey(bytes32,bytes32) [INTERNAL]
```slithir
_underlyingSourceAddressHash_1(bytes32) := phi(['REF_3758', 'REF_3764'])
_transactionHash_1(bytes32) := phi(['REF_3761', 'REF_3767'])
 keccak256(bytes)(abi.encode(_underlyingSourceAddressHash,_transactionHash))
TMP_5340(bytes) = SOLIDITY_CALL abi.encode()(_underlyingSourceAddressHash_1,_transactionHash_1)
TMP_5341(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_5340)
RETURN TMP_5341
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
#### IIAgentVault.payout(IERC20,address,uint256) [EXTERNAL]
```slithir

```
#### Conversion.convertFromUSD5(uint256,CollateralTypeInt.Data) [INTERNAL]
```slithir
 bytes(_token.tokenFtsoSymbol).length == 0
REF_3158(string) -> _token_1 (-> []).tokenFtsoSymbol
TMP_4655 = CONVERT REF_3158 to bytes
REF_3159 -> LENGTH TMP_4655
TMP_4656(bool) = REF_3159 == 0
CONDITION TMP_4656
 _amountUSD5
RETURN _amountUSD5_1
 (tokenPrice,None,tokenFtsoDec) = readFtsoPrice(_token.tokenFtsoSymbol,false)
REF_3160(string) -> _token_1 (-> []).tokenFtsoSymbol
TUPLE_47(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.readFtsoPrice(string,bool)(REF_3160,False)
tokenPrice_1(uint256)= UNPACK TUPLE_47 index: 0 
tokenFtsoDec_1(uint256)= UNPACK TUPLE_47 index: 2 
 expPlus = _token.decimals + tokenFtsoDec - 5
REF_3161(uint8) -> _token_1 (-> []).decimals
TMP_4657(uint8) = REF_3161 (c)+ tokenFtsoDec_1
TMP_4658(uint8) = TMP_4657 (c)- 5
expPlus_1(uint256) := TMP_4658(uint8)
 _amountUSD5.mulDiv(10 ** expPlus,tokenPrice)
TMP_4659(uint256) = 10 (c)** expPlus_1
TMP_4660(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_amountUSD5_1', 'TMP_4659', 'tokenPrice_1'] 
RETURN TMP_4660
```
#### Globals.getAgentOwnerRegistry() [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4737(AssetManagerSettings.Data) = INTERNAL_CALL, Globals.getSettings()()
settings_1 (-> ['TMP_4737'])(AssetManagerSettings.Data) := TMP_4737(AssetManagerSettings.Data)
 IAgentOwnerRegistry(settings.agentOwnerRegistry)
REF_3242(address) -> settings_1 (-> ['TMP_4737']).agentOwnerRegistry
TMP_4738 = CONVERT REF_3242 to IAgentOwnerRegistry
RETURN TMP_4738
```
#### AgentOwnerRegistry.getWorkAddress(address) [EXTERNAL]
```slithir
mgmtToWorkAddress_4(mapping(address => address)) := phi(['mgmtToWorkAddress_3', 'mgmtToWorkAddress_0', 'mgmtToWorkAddress_4'])
 mgmtToWorkAddress[_managementAddress]
REF_311(address) -> mgmtToWorkAddress_4[_managementAddress_1]
RETURN REF_311
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
#### IICollateralPool.payout(address,uint256,uint256) [EXTERNAL]
```slithir

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
#### AgentCollateral.agentVaultCollateralData(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
 collateral = _agent.getVaultCollateral()
TMP_4306(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4306'])(CollateralTypeInt.Data) := TMP_4306(CollateralTypeInt.Data)
 Collateral.Data({kind:Collateral.Kind.VAULT,fullCollateral:collateral.token.balanceOf(_agent.vaultAddress()),amgToTokenWeiPrice:Conversion.currentAmgPriceInTokenWei(collateral)})
REF_2819(Collateral.Kind) -> Kind.VAULT
REF_2820(IERC20) -> collateral_1 (-> ['TMP_4306']).token
TMP_4307(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4308(uint256) = HIGH_LEVEL_CALL, dest:REF_2820(IERC20), function:balanceOf, arguments:['TMP_4307']  
TMP_4309(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data), arguments:["collateral_1 (-> ['TMP_4306'])"] 
TMP_4310(Collateral.Data) = new Data(REF_2819,TMP_4308,TMP_4309)
RETURN TMP_4310
```
#### AgentCollateral.maxRedemptionCollateral(Collateral.Data,Agent.State,uint256) [INTERNAL]
```slithir
 _valueAMG == 0
TMP_4360(bool) = _valueAMG_1 == 0
CONDITION TMP_4360
 0
RETURN 0
 assert(bool)(_valueAMG <= redeemingAMG)
TMP_4361(bool) = _valueAMG_1 <= redeemingAMG_3
TMP_4362(None) = SOLIDITY_CALL assert(bool)(TMP_4361)
 totalAMG = uint256(_agent.mintedAMG) + uint256(_agent.reservedAMG) + uint256(redeemingAMG)
REF_2889(uint64) -> _agent_1 (-> []).mintedAMG
TMP_4363 = CONVERT REF_2889 to uint256
REF_2890(uint64) -> _agent_1 (-> []).reservedAMG
TMP_4364 = CONVERT REF_2890 to uint256
TMP_4365(uint256) = TMP_4363 (c)+ TMP_4364
TMP_4366 = CONVERT redeemingAMG_3 to uint256
TMP_4367(uint256) = TMP_4365 (c)+ TMP_4366
totalAMG_1(uint256) := TMP_4367(uint256)
 _data.fullCollateral.mulDiv(_valueAMG,totalAMG)
REF_2891(uint256) -> _data_1.fullCollateral
TMP_4368(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['REF_2891', '_valueAMG_1', 'totalAMG_1'] 
RETURN TMP_4368
 _data.kind == Collateral.Kind.POOL
REF_2893(Collateral.Kind) -> _data_1.kind
REF_2894(Collateral.Kind) -> Kind.POOL
TMP_4369(bool) = REF_2893 == REF_2894
CONDITION TMP_4369
 redeemingAMG = _agent.poolRedeemingAMG
REF_2895(uint64) -> _agent_1 (-> []).poolRedeemingAMG
redeemingAMG_1(uint256) := REF_2895(uint64)
 redeemingAMG = _agent.redeemingAMG
REF_2896(uint64) -> _agent_1 (-> []).redeemingAMG
redeemingAMG_2(uint256) := REF_2896(uint64)
redeemingAMG_3(uint256) := phi(['redeemingAMG_1', 'redeemingAMG_2'])
```
#### AgentCollateral.poolCollateralData(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
 collateral = _agent.getPoolCollateral()
TMP_4311(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getPoolCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4311'])(CollateralTypeInt.Data) := TMP_4311(CollateralTypeInt.Data)
 Collateral.Data({kind:Collateral.Kind.POOL,fullCollateral:_agent.collateralPool.totalCollateral(),amgToTokenWeiPrice:Conversion.currentAmgPriceInTokenWei(collateral)})
REF_2826(Collateral.Kind) -> Kind.POOL
REF_2827(IICollateralPool) -> _agent_1 (-> []).collateralPool
TMP_4312(uint256) = HIGH_LEVEL_CALL, dest:REF_2827(IICollateralPool), function:totalCollateral, arguments:[]  
TMP_4313(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data), arguments:["collateral_1 (-> ['TMP_4311'])"] 
TMP_4314(Collateral.Data) = new Data(REF_2826,TMP_4312,TMP_4313)
RETURN TMP_4314
```
#### Conversion.convertAmgToTokenWei(uint256,uint256) [INTERNAL]
```slithir
AMG_TOKEN_WEI_PRICE_SCALE_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_0'])
 _valueAMG.mulDiv(_amgToTokenWeiPrice,AMG_TOKEN_WEI_PRICE_SCALE)
TMP_4674(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_valueAMG_1', '_amgToTokenWeiPrice_1', 'AMG_TOKEN_WEI_PRICE_SCALE_1'] 
RETURN TMP_4674
```
#### SafePct.mulDivRoundUp(uint256,uint256,uint256) [INTERNAL]
```slithir
 resultRoundDown = mulDiv(x,y,z)
TMP_10531(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,z_1)
resultRoundDown_1(uint256) := TMP_10531(uint256)
 remainder = mulmod(uint256,uint256,uint256)(x,y,z)
TMP_10532(uint256) = SOLIDITY_CALL mulmod(uint256,uint256,uint256)(x_1,y_1,z_1)
remainder_1(uint256) := TMP_10532(uint256)
 remainder == 0
TMP_10533(bool) = remainder_1 == 0
CONDITION TMP_10533
 resultRoundDown
RETURN resultRoundDown_1
 resultRoundDown + 1
TMP_10534(uint256) = resultRoundDown_1 + 1
RETURN TMP_10534
```
#### AgentCollateral.combinedData(Agent.State) [INTERNAL]
```slithir
 poolCollateral = poolCollateralData(_agent)
TMP_4296(Collateral.Data) = INTERNAL_CALL, AgentCollateral.poolCollateralData(Agent.State)(_agent_1 (-> []))
poolCollateral_1(Collateral.Data) := TMP_4296(Collateral.Data)
 Collateral.CombinedData({agentCollateral:agentVaultCollateralData(_agent),poolCollateral:poolCollateral,agentPoolTokens:agentsPoolTokensCollateralData(_agent,poolCollateral)})
TMP_4297(Collateral.Data) = INTERNAL_CALL, AgentCollateral.agentVaultCollateralData(Agent.State)(_agent_1 (-> []))
TMP_4298(Collateral.Data) = INTERNAL_CALL, AgentCollateral.agentsPoolTokensCollateralData(Agent.State,Collateral.Data)(_agent_1 (-> []),poolCollateral_1)
TMP_4299(Collateral.CombinedData) = new CombinedData(TMP_4297,poolCollateral_1,TMP_4298)
RETURN TMP_4299
```
#### TransactionAttestation.verifyConfirmedBlockHeightExists(IConfirmedBlockHeightExists.Proof) [INTERNAL]
```slithir
 _settings = Globals.getSettings()
TMP_5253(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5253'])(AssetManagerSettings.Data) := TMP_5253(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3682(address) -> _settings_1 (-> ['TMP_5253']).fdcVerification
TMP_5254 = CONVERT REF_3682 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5254(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3683(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_3684(bytes32) -> REF_3683.sourceId
REF_3685(bytes32) -> _settings_1 (-> ['TMP_5253']).chainId
TMP_5255(bool) = REF_3684 == REF_3685
TMP_5256(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5257(None) = SOLIDITY_CALL require(bool,error)(TMP_5255,TMP_5256)
 require(bool,error)(fdcVerification.verifyConfirmedBlockHeightExists(_proof),revert BlockHeightNotProven()())
TMP_5258(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyConfirmedBlockHeightExists, arguments:['_proof_1']  
TMP_5259(None) = SOLIDITY_CALL revert BlockHeightNotProven()()
TMP_5260(None) = SOLIDITY_CALL require(bool,error)(TMP_5258,TMP_5259)
```
