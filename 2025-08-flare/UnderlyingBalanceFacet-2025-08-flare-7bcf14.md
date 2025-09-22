





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


### Storage layout (AgentVault) 

```text
assetManager IIAssetManager
initialized bool
__usedTokens IERC20[]
__tokenUseFlags mapping(IERC20 => uint256)
__internalWithdrawal bool
destroyed bool

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



#### UnderlyingBalanceFacet.announceUnderlyingWithdrawal(address) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4184(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4184'])(AssetManagerState.State) := TMP_4184(AssetManagerState.State)
 agent = Agent.get(_agentVault)
TMP_4185(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_4185'])(Agent.State) := TMP_4185(Agent.State)
 require(bool,error)(agent.announcedUnderlyingWithdrawalId == 0,revert AnnouncedUnderlyingWithdrawalActive()())
REF_2716(uint64) -> agent_1 (-> ['TMP_4185']).announcedUnderlyingWithdrawalId
TMP_4186(bool) = REF_2716 == 0
TMP_4187(None) = SOLIDITY_CALL revert AnnouncedUnderlyingWithdrawalActive()()
TMP_4188(None) = SOLIDITY_CALL require(bool,error)(TMP_4186,TMP_4187)
 state.newPaymentAnnouncementId += PaymentReference.randomizedIdSkip()
REF_2717(uint64) -> state_1 (-> ['TMP_4184']).newPaymentAnnouncementId
TMP_4189(uint64) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.randomizedIdSkip(), arguments:[] 
state_2 (-> ['TMP_4184'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_4184'])"])
REF_2717(-> state_2 (-> ['TMP_4184'])) = REF_2717 (c)+ TMP_4189
TMP_4184(AssetManagerState.State) := phi(["state_2 (-> ['TMP_4184'])"])
 announcementId = state.newPaymentAnnouncementId
REF_2719(uint64) -> state_2 (-> ['TMP_4184']).newPaymentAnnouncementId
announcementId_1(uint64) := REF_2719(uint64)
 agent.announcedUnderlyingWithdrawalId = announcementId
REF_2720(uint64) -> agent_1 (-> ['TMP_4185']).announcedUnderlyingWithdrawalId
agent_2 (-> ['TMP_4185'])(Agent.State) := phi(["agent_1 (-> ['TMP_4185'])"])
REF_2720(uint64) (->agent_2 (-> ['TMP_4185'])) := announcementId_1(uint64)
TMP_4185(Agent.State) := phi(["agent_2 (-> ['TMP_4185'])"])
 agent.underlyingWithdrawalAnnouncedAt = block.timestamp.toUint64()
REF_2721(uint64) -> agent_2 (-> ['TMP_4185']).underlyingWithdrawalAnnouncedAt
TMP_4190(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.timestamp'] 
agent_3 (-> ['TMP_4185'])(Agent.State) := phi(["agent_2 (-> ['TMP_4185'])"])
REF_2721(uint64) (->agent_3 (-> ['TMP_4185'])) := TMP_4190(uint64)
TMP_4185(Agent.State) := phi(["agent_3 (-> ['TMP_4185'])"])
 paymentReference = PaymentReference.announcedWithdrawal(announcementId)
TMP_4191(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.announcedWithdrawal(uint256), arguments:['announcementId_1'] 
paymentReference_1(bytes32) := TMP_4191(bytes32)
 IAssetManagerEvents.UnderlyingWithdrawalAnnounced(_agentVault,announcementId,paymentReference)
Emit UnderlyingWithdrawalAnnounced(_agentVault_1,announcementId_1,paymentReference_1)
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
```

#### UnderlyingBalanceFacet.confirmTopupPayment(IPayment.Proof,address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_4164(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_4164'])(Agent.State) := TMP_4164(Agent.State)
 state = AssetManagerState.get()
TMP_4165(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4165'])(AssetManagerState.State) := TMP_4165(AssetManagerState.State)
 TransactionAttestation.verifyPaymentSuccess(_payment)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyPaymentSuccess(IPayment.Proof), arguments:['_payment_1'] 
 require(bool,error)(_payment.data.responseBody.receivingAddressHash == agent.underlyingAddressHash,revert NotUnderlyingAddress()())
REF_2689(IPayment.Response) -> _payment_1.data
REF_2690(IPayment.ResponseBody) -> REF_2689.responseBody
REF_2691(bytes32) -> REF_2690.receivingAddressHash
REF_2692(bytes32) -> agent_1 (-> ['TMP_4164']).underlyingAddressHash
TMP_4167(bool) = REF_2691 == REF_2692
TMP_4168(None) = SOLIDITY_CALL revert NotUnderlyingAddress()()
TMP_4169(None) = SOLIDITY_CALL require(bool,error)(TMP_4167,TMP_4168)
 require(bool,error)(_payment.data.responseBody.standardPaymentReference == PaymentReference.topup(_agentVault),revert NotATopupPayment()())
REF_2693(IPayment.Response) -> _payment_1.data
REF_2694(IPayment.ResponseBody) -> REF_2693.responseBody
REF_2695(bytes32) -> REF_2694.standardPaymentReference
TMP_4170(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.topup(address), arguments:['_agentVault_1'] 
TMP_4171(bool) = REF_2695 == TMP_4170
TMP_4172(None) = SOLIDITY_CALL revert NotATopupPayment()()
TMP_4173(None) = SOLIDITY_CALL require(bool,error)(TMP_4171,TMP_4172)
 require(bool,error)(_payment.data.responseBody.blockNumber > agent.underlyingBlockAtCreation,revert TopupBeforeAgentCreated()())
REF_2697(IPayment.Response) -> _payment_1.data
REF_2698(IPayment.ResponseBody) -> REF_2697.responseBody
REF_2699(uint64) -> REF_2698.blockNumber
REF_2700(uint64) -> agent_1 (-> ['TMP_4164']).underlyingBlockAtCreation
TMP_4174(bool) = REF_2699 > REF_2700
TMP_4175(None) = SOLIDITY_CALL revert TopupBeforeAgentCreated()()
TMP_4176(None) = SOLIDITY_CALL require(bool,error)(TMP_4174,TMP_4175)
 state.paymentConfirmations.confirmIncomingPayment(_payment)
REF_2701(PaymentConfirmations.State) -> state_1 (-> ['TMP_4165']).paymentConfirmations
LIBRARY_CALL, dest:PaymentConfirmations, function:PaymentConfirmations.confirmIncomingPayment(PaymentConfirmations.State,IPayment.Proof), arguments:['REF_2701', '_payment_1'] 
 amountUBA = SafeCast.toUint256(_payment.data.responseBody.receivedAmount)
REF_2704(IPayment.Response) -> _payment_1.data
REF_2705(IPayment.ResponseBody) -> REF_2704.responseBody
REF_2706(int256) -> REF_2705.receivedAmount
TMP_4178(uint256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint256(int256), arguments:['REF_2706'] 
amountUBA_1(uint256) := TMP_4178(uint256)
 UnderlyingBalance.increaseBalance(agent,amountUBA.toUint128())
TMP_4179(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['amountUBA_1'] 
LIBRARY_CALL, dest:UnderlyingBalance, function:UnderlyingBalance.increaseBalance(Agent.State,uint256), arguments:["agent_1 (-> ['TMP_4164'])", 'TMP_4179'] 
 UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(_payment)
LIBRARY_CALL, dest:UnderlyingBlockUpdater, function:UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(IPayment.Proof), arguments:['_payment_1'] 
 IAssetManagerEvents.UnderlyingBalanceToppedUp(_agentVault,_payment.data.requestBody.transactionId,amountUBA)
REF_2711(IPayment.Response) -> _payment_1.data
REF_2712(IPayment.RequestBody) -> REF_2711.requestBody
REF_2713(bytes32) -> REF_2712.transactionId
Emit UnderlyingBalanceToppedUp(_agentVault_1,REF_2713,amountUBA_1)
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
```
#### UnderlyingBalanceFacet.confirmUnderlyingWithdrawal(IPayment.Proof,address) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4194(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4194'])(AssetManagerState.State) := TMP_4194(AssetManagerState.State)
 settings = Globals.getSettings()
TMP_4195(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4195'])(AssetManagerSettings.Data) := TMP_4195(AssetManagerSettings.Data)
 TransactionAttestation.verifyPayment(_payment)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyPayment(IPayment.Proof), arguments:['_payment_1'] 
 agent = Agent.get(_agentVault)
TMP_4197(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_4197'])(Agent.State) := TMP_4197(Agent.State)
 isAgent = Agents.isOwner(agent,msg.sender)
TMP_4198(bool) = LIBRARY_CALL, dest:Agents, function:Agents.isOwner(Agent.State,address), arguments:["agent_1 (-> ['TMP_4197'])", 'msg.sender'] 
isAgent_1(bool) := TMP_4198(bool)
 announcementId = agent.announcedUnderlyingWithdrawalId
REF_2730(uint64) -> agent_1 (-> ['TMP_4197']).announcedUnderlyingWithdrawalId
announcementId_1(uint64) := REF_2730(uint64)
 require(bool,error)(announcementId != 0,revert NoActiveAnnouncement()())
TMP_4199(bool) = announcementId_1 != 0
TMP_4200(None) = SOLIDITY_CALL revert NoActiveAnnouncement()()
TMP_4201(None) = SOLIDITY_CALL require(bool,error)(TMP_4199,TMP_4200)
 paymentReference = PaymentReference.announcedWithdrawal(announcementId)
TMP_4202(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.announcedWithdrawal(uint256), arguments:['announcementId_1'] 
paymentReference_1(bytes32) := TMP_4202(bytes32)
 require(bool,error)(_payment.data.responseBody.standardPaymentReference == paymentReference,revert WrongAnnouncedPaymentReference()())
REF_2732(IPayment.Response) -> _payment_1.data
REF_2733(IPayment.ResponseBody) -> REF_2732.responseBody
REF_2734(bytes32) -> REF_2733.standardPaymentReference
TMP_4203(bool) = REF_2734 == paymentReference_1
TMP_4204(None) = SOLIDITY_CALL revert WrongAnnouncedPaymentReference()()
TMP_4205(None) = SOLIDITY_CALL require(bool,error)(TMP_4203,TMP_4204)
 require(bool,error)(_payment.data.responseBody.sourceAddressHash == agent.underlyingAddressHash,revert WrongAnnouncedPaymentSource()())
REF_2735(IPayment.Response) -> _payment_1.data
REF_2736(IPayment.ResponseBody) -> REF_2735.responseBody
REF_2737(bytes32) -> REF_2736.sourceAddressHash
REF_2738(bytes32) -> agent_1 (-> ['TMP_4197']).underlyingAddressHash
TMP_4206(bool) = REF_2737 == REF_2738
TMP_4207(None) = SOLIDITY_CALL revert WrongAnnouncedPaymentSource()()
TMP_4208(None) = SOLIDITY_CALL require(bool,error)(TMP_4206,TMP_4207)
 require(bool,error)(isAgent || block.timestamp > agent.underlyingWithdrawalAnnouncedAt + settings.confirmationByOthersAfterSeconds,Agents.OnlyAgentVaultOwner())
REF_2739(uint64) -> agent_1 (-> ['TMP_4197']).underlyingWithdrawalAnnouncedAt
REF_2740(uint64) -> settings_1 (-> ['TMP_4195']).confirmationByOthersAfterSeconds
TMP_4209(uint64) = REF_2739 (c)+ REF_2740
TMP_4210(bool) = block.timestamp > TMP_4209
TMP_4211(bool) = isAgent_1 || TMP_4210
TMP_4212(None) = SOLIDITY_CALL revert OnlyAgentVaultOwner()()
TMP_4213(None) = SOLIDITY_CALL require(bool,error)(TMP_4211,TMP_4212)
 state.paymentConfirmations.confirmSourceDecreasingTransaction(_payment)
REF_2741(PaymentConfirmations.State) -> state_1 (-> ['TMP_4194']).paymentConfirmations
LIBRARY_CALL, dest:PaymentConfirmations, function:PaymentConfirmations.confirmSourceDecreasingTransaction(PaymentConfirmations.State,IPayment.Proof), arguments:['REF_2741', '_payment_1'] 
 agent.announcedUnderlyingWithdrawalId = 0
REF_2743(uint64) -> agent_1 (-> ['TMP_4197']).announcedUnderlyingWithdrawalId
agent_2 (-> ['TMP_4197'])(Agent.State) := phi(["agent_1 (-> ['TMP_4197'])"])
REF_2743(uint64) (->agent_2 (-> ['TMP_4197'])) := 0(uint256)
TMP_4197(Agent.State) := phi(["agent_2 (-> ['TMP_4197'])"])
 UnderlyingBalance.updateBalance(agent,- _payment.data.responseBody.spentAmount)
REF_2745(IPayment.Response) -> _payment_1.data
REF_2746(IPayment.ResponseBody) -> REF_2745.responseBody
REF_2747(int256) -> REF_2746.spentAmount
TMP_4215(int256) = 0 (c)- REF_2747
LIBRARY_CALL, dest:UnderlyingBalance, function:UnderlyingBalance.updateBalance(Agent.State,int256), arguments:["agent_2 (-> ['TMP_4197'])", 'TMP_4215'] 
 ! isAgent
TMP_4217 = UnaryType.BANG isAgent_1 
CONDITION TMP_4217
 AgentPayout.payForConfirmationByOthers(agent,msg.sender)
LIBRARY_CALL, dest:AgentPayout, function:AgentPayout.payForConfirmationByOthers(Agent.State,address), arguments:["agent_2 (-> ['TMP_4197'])", 'msg.sender'] 
 UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(_payment)
LIBRARY_CALL, dest:UnderlyingBlockUpdater, function:UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(IPayment.Proof), arguments:['_payment_1'] 
 IAssetManagerEvents.UnderlyingWithdrawalConfirmed(_agentVault,announcementId,_payment.data.responseBody.spentAmount,_payment.data.requestBody.transactionId)
REF_2751(IPayment.Response) -> _payment_1.data
REF_2752(IPayment.ResponseBody) -> REF_2751.responseBody
REF_2753(int256) -> REF_2752.spentAmount
REF_2754(IPayment.Response) -> _payment_1.data
REF_2755(IPayment.RequestBody) -> REF_2754.requestBody
REF_2756(bytes32) -> REF_2755.transactionId
Emit UnderlyingWithdrawalConfirmed(_agentVault_1,announcementId_1,REF_2753,REF_2756)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
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
#### PaymentReference.announcedWithdrawal(uint256) [INTERNAL]
```slithir
MAX_ID_3(uint256) := phi(['MAX_ID_0'])
ANNOUNCED_WITHDRAWAL_1(uint256) := phi(['ANNOUNCED_WITHDRAWAL_0'])
 assert(bool)(_id <= MAX_ID)
TMP_5353(bool) = _id_1 <= MAX_ID_3
TMP_5354(None) = SOLIDITY_CALL assert(bool)(TMP_5353)
 bytes32(_id | ANNOUNCED_WITHDRAWAL)
TMP_5355(uint256) = _id_1 | ANNOUNCED_WITHDRAWAL_1
TMP_5356 = CONVERT TMP_5355 to bytes32
RETURN TMP_5356
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
#### TransactionAttestation.verifyPaymentSuccess(IPayment.Proof) [INTERNAL]
```slithir
PAYMENT_SUCCESS_1(uint8) := phi(['PAYMENT_SUCCESS_0'])
 require(bool,error)(_proof.data.responseBody.status == PAYMENT_SUCCESS,revert PaymentFailed()())
REF_3666(IPayment.Response) -> _proof_1.data
REF_3667(IPayment.ResponseBody) -> REF_3666.responseBody
REF_3668(uint8) -> REF_3667.status
TMP_5233(bool) = REF_3668 == PAYMENT_SUCCESS_1
TMP_5234(None) = SOLIDITY_CALL revert PaymentFailed()()
TMP_5235(None) = SOLIDITY_CALL require(bool,error)(TMP_5233,TMP_5234)
 verifyPayment(_proof)
INTERNAL_CALL, TransactionAttestation.verifyPayment(IPayment.Proof)(_proof_1)
```
#### UnderlyingBalance.increaseBalance(Agent.State,uint256) [INTERNAL]
```slithir
 _agent.underlyingBalanceUBA += _balanceIncrease.toInt256().toInt128()
REF_3709(int128) -> _agent_1 (-> []).underlyingBalanceUBA
TMP_5287(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['_balanceIncrease_1'] 
TMP_5288(int128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt128(int256), arguments:['TMP_5287'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_3709(-> _agent_2 (-> [])) = REF_3709 (c)+ TMP_5288
 IAssetManagerEvents.UnderlyingBalanceChanged(_agent.vaultAddress(),_agent.underlyingBalanceUBA)
TMP_5289(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_2 (-> [])'] 
REF_3714(int128) -> _agent_2 (-> []).underlyingBalanceUBA
Emit UnderlyingBalanceChanged(TMP_5289,REF_3714)
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
#### PaymentConfirmations.confirmIncomingPayment(PaymentConfirmations.State,IPayment.Proof) [INTERNAL]
```slithir
 _recordPaymentVerification(_state,_payment.data.requestBody.transactionId)
REF_3753(IPayment.Response) -> _payment_1.data
REF_3754(IPayment.RequestBody) -> REF_3753.requestBody
REF_3755(bytes32) -> REF_3754.transactionId
INTERNAL_CALL, PaymentConfirmations._recordPaymentVerification(PaymentConfirmations.State,bytes32)(_state_1 (-> []),REF_3755)
```
#### PaymentReference.topup(address) [INTERNAL]
```slithir
TOPUP_1(uint256) := phi(['TOPUP_0'])
 bytes32(uint256(uint160(_agentVault)) | TOPUP)
TMP_5365 = CONVERT _agentVault_1 to uint160
TMP_5366 = CONVERT TMP_5365 to uint256
TMP_5367(uint256) = TMP_5366 | TOPUP_1
TMP_5368 = CONVERT TMP_5367 to bytes32
RETURN TMP_5368
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
#### SafeCast.toUint256(int256) [INTERNAL]
```slithir
 require(bool,string)(value >= 0,SafeCast: value must be positive)
TMP_786(bool) = value_1 >= 0
TMP_787(None) = SOLIDITY_CALL require(bool,string)(TMP_786,SafeCast: value must be positive)
 uint256(value)
TMP_788 = CONVERT value_1 to uint256
RETURN TMP_788
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

#### PaymentConfirmations.transactionKey(bytes32,bytes32) [INTERNAL]
```slithir
_underlyingSourceAddressHash_1(bytes32) := phi(['REF_3758', 'REF_3764'])
_transactionHash_1(bytes32) := phi(['REF_3761', 'REF_3767'])
 keccak256(bytes)(abi.encode(_underlyingSourceAddressHash,_transactionHash))
TMP_5340(bytes) = SOLIDITY_CALL abi.encode()(_underlyingSourceAddressHash_1,_transactionHash_1)
TMP_5341(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_5340)
RETURN TMP_5341
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
_balances_1(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
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
#### AgentVault.payout(IERC20,address,uint256) [EXTERNAL]
```slithir
 _token.safeTransfer(_recipient,_amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['_token_1', '_recipient_1', '_amount_1'] 
 onlyAssetManager()
MODIFIER_CALL, AgentVault.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
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
