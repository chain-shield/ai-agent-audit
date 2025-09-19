
### Storage layout (AgentVault) 

```text
assetManager IIAssetManager
initialized bool
__usedTokens IERC20[]
__tokenUseFlags mapping(IERC20 => uint256)
__internalWithdrawal bool
destroyed bool

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











#### ChallengesFacet._validateAgentStatus(Agent.State) [PRIVATE]
```slithir
_agent_1 (-> ['TMP_2266', 'TMP_2246', 'TMP_2219'])(Agent.State) := phi(["agent_1 (-> ['TMP_2266'])", "agent_1 (-> ['TMP_2246'])", "agent_1 (-> ['TMP_2219'])"])
 status = _agent.status
REF_1233(Agent.Status) -> _agent_1 (-> ['TMP_2266', 'TMP_2246', 'TMP_2219']).status
status_1(Agent.Status) := REF_1233(Agent.Status)
 require(bool,error)(status != Agent.Status.FULL_LIQUIDATION,revert ChallengeAlreadyLiquidating()())
REF_1234(Agent.Status) -> Status.FULL_LIQUIDATION
TMP_2294(bool) = status_1 != REF_1234
TMP_2295(None) = SOLIDITY_CALL revert ChallengeAlreadyLiquidating()()
TMP_2296(None) = SOLIDITY_CALL require(bool,error)(TMP_2294,TMP_2295)
 require(bool,error)(status != Agent.Status.DESTROYING,revert ChallengeInvalidAgentStatus()())
REF_1235(Agent.Status) -> Status.DESTROYING
TMP_2297(bool) = status_1 != REF_1235
TMP_2298(None) = SOLIDITY_CALL revert ChallengeInvalidAgentStatus()()
TMP_2299(None) = SOLIDITY_CALL require(bool,error)(TMP_2297,TMP_2298)
```
#### ChallengesFacet.doublePaymentChallenge(IBalanceDecreasingTransaction.Proof,IBalanceDecreasingTransaction.Proof,address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_2246(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2246'])(Agent.State) := TMP_2246(Agent.State)
 _validateAgentStatus(agent)
INTERNAL_CALL, ChallengesFacet._validateAgentStatus(Agent.State)(agent_1 (-> ['TMP_2246']))
 TransactionAttestation.verifyBalanceDecreasingTransaction(_payment1)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyBalanceDecreasingTransaction(IBalanceDecreasingTransaction.Proof), arguments:['_payment1_1'] 
 TransactionAttestation.verifyBalanceDecreasingTransaction(_payment2)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyBalanceDecreasingTransaction(IBalanceDecreasingTransaction.Proof), arguments:['_payment2_1'] 
 require(bool,error)(_payment1.data.requestBody.transactionId != _payment2.data.requestBody.transactionId,revert ChallengeSameTransactionRepeated()())
REF_1162(IBalanceDecreasingTransaction.Response) -> _payment1_1.data
REF_1163(IBalanceDecreasingTransaction.RequestBody) -> REF_1162.requestBody
REF_1164(bytes32) -> REF_1163.transactionId
REF_1165(IBalanceDecreasingTransaction.Response) -> _payment2_1.data
REF_1166(IBalanceDecreasingTransaction.RequestBody) -> REF_1165.requestBody
REF_1167(bytes32) -> REF_1166.transactionId
TMP_2250(bool) = REF_1164 != REF_1167
TMP_2251(None) = SOLIDITY_CALL revert ChallengeSameTransactionRepeated()()
TMP_2252(None) = SOLIDITY_CALL require(bool,error)(TMP_2250,TMP_2251)
 require(bool,error)(_payment1.data.responseBody.sourceAddressHash == agent.underlyingAddressHash,revert ChallengeNotAgentsAddress()())
REF_1168(IBalanceDecreasingTransaction.Response) -> _payment1_1.data
REF_1169(IBalanceDecreasingTransaction.ResponseBody) -> REF_1168.responseBody
REF_1170(bytes32) -> REF_1169.sourceAddressHash
REF_1171(bytes32) -> agent_1 (-> ['TMP_2246']).underlyingAddressHash
TMP_2253(bool) = REF_1170 == REF_1171
TMP_2254(None) = SOLIDITY_CALL revert ChallengeNotAgentsAddress()()
TMP_2255(None) = SOLIDITY_CALL require(bool,error)(TMP_2253,TMP_2254)
 require(bool,error)(_payment2.data.responseBody.sourceAddressHash == agent.underlyingAddressHash,revert ChallengeNotAgentsAddress()())
REF_1172(IBalanceDecreasingTransaction.Response) -> _payment2_1.data
REF_1173(IBalanceDecreasingTransaction.ResponseBody) -> REF_1172.responseBody
REF_1174(bytes32) -> REF_1173.sourceAddressHash
REF_1175(bytes32) -> agent_1 (-> ['TMP_2246']).underlyingAddressHash
TMP_2256(bool) = REF_1174 == REF_1175
TMP_2257(None) = SOLIDITY_CALL revert ChallengeNotAgentsAddress()()
TMP_2258(None) = SOLIDITY_CALL require(bool,error)(TMP_2256,TMP_2257)
 require(bool,error)(_payment1.data.responseBody.standardPaymentReference == _payment2.data.responseBody.standardPaymentReference,revert ChallengeNotDuplicate()())
REF_1176(IBalanceDecreasingTransaction.Response) -> _payment1_1.data
REF_1177(IBalanceDecreasingTransaction.ResponseBody) -> REF_1176.responseBody
REF_1178(bytes32) -> REF_1177.standardPaymentReference
REF_1179(IBalanceDecreasingTransaction.Response) -> _payment2_1.data
REF_1180(IBalanceDecreasingTransaction.ResponseBody) -> REF_1179.responseBody
REF_1181(bytes32) -> REF_1180.standardPaymentReference
TMP_2259(bool) = REF_1178 == REF_1181
TMP_2260(None) = SOLIDITY_CALL revert ChallengeNotDuplicate()()
TMP_2261(None) = SOLIDITY_CALL require(bool,error)(TMP_2259,TMP_2260)
 _liquidateAndRewardChallenger(agent,msg.sender,agent.mintedAMG)
REF_1182(uint64) -> agent_1 (-> ['TMP_2246']).mintedAMG
INTERNAL_CALL, ChallengesFacet._liquidateAndRewardChallenger(Agent.State,address,uint256)(agent_1 (-> ['TMP_2246']),msg.sender,REF_1182)
 IAssetManagerEvents.DuplicatePaymentConfirmed(_agentVault,_payment1.data.requestBody.transactionId,_payment2.data.requestBody.transactionId)
REF_1184(IBalanceDecreasingTransaction.Response) -> _payment1_1.data
REF_1185(IBalanceDecreasingTransaction.RequestBody) -> REF_1184.requestBody
REF_1186(bytes32) -> REF_1185.transactionId
REF_1187(IBalanceDecreasingTransaction.Response) -> _payment2_1.data
REF_1188(IBalanceDecreasingTransaction.RequestBody) -> REF_1187.requestBody
REF_1189(bytes32) -> REF_1188.transactionId
Emit DuplicatePaymentConfirmed(_agentVault_1,REF_1186,REF_1189)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### ChallengesFacet.freeBalanceNegativeChallenge(IBalanceDecreasingTransaction.Proof[],address) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_2265(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2265'])(AssetManagerState.State) := TMP_2265(AssetManagerState.State)
 agent = Agent.get(_agentVault)
TMP_2266(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2266'])(Agent.State) := TMP_2266(Agent.State)
 _validateAgentStatus(agent)
INTERNAL_CALL, ChallengesFacet._validateAgentStatus(Agent.State)(agent_1 (-> ['TMP_2266']))
 total = 0
total_1(int256) := 0(int256)
 i = 0
i_1(uint256) := 0(uint256)
 i < _payments.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_1192 -> LENGTH _payments_1
TMP_2268(bool) = i_2 < REF_1192
CONDITION TMP_2268
 pmi = _payments[i]
REF_1193(IBalanceDecreasingTransaction.Proof) -> _payments_1[i_2]
pmi_1(IBalanceDecreasingTransaction.Proof) := REF_1193(IBalanceDecreasingTransaction.Proof)
 TransactionAttestation.verifyBalanceDecreasingTransaction(pmi)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyBalanceDecreasingTransaction(IBalanceDecreasingTransaction.Proof), arguments:['pmi_1'] 
 j = 0
j_1(uint256) := 0(uint256)
 j < i
j_2(uint256) := phi(['j_1', 'j_3'])
TMP_2270(bool) = j_2 < i_2
CONDITION TMP_2270
 require(bool,error)(_payments[j].data.requestBody.transactionId != pmi.data.requestBody.transactionId,revert ChallengeSameTransactionRepeated()())
REF_1195(IBalanceDecreasingTransaction.Proof) -> _payments_1[j_2]
REF_1196(IBalanceDecreasingTransaction.Response) -> REF_1195.data
REF_1197(IBalanceDecreasingTransaction.RequestBody) -> REF_1196.requestBody
REF_1198(bytes32) -> REF_1197.transactionId
REF_1199(IBalanceDecreasingTransaction.Response) -> pmi_1.data
REF_1200(IBalanceDecreasingTransaction.RequestBody) -> REF_1199.requestBody
REF_1201(bytes32) -> REF_1200.transactionId
TMP_2271(bool) = REF_1198 != REF_1201
TMP_2272(None) = SOLIDITY_CALL revert ChallengeSameTransactionRepeated()()
TMP_2273(None) = SOLIDITY_CALL require(bool,error)(TMP_2271,TMP_2272)
 j ++
TMP_2274(uint256) := j_2(uint256)
j_3(uint256) = j_2 (c)+ 1
 require(bool,error)(pmi.data.responseBody.sourceAddressHash == agent.underlyingAddressHash,revert ChallengeNotAgentsAddress()())
REF_1202(IBalanceDecreasingTransaction.Response) -> pmi_1.data
REF_1203(IBalanceDecreasingTransaction.ResponseBody) -> REF_1202.responseBody
REF_1204(bytes32) -> REF_1203.sourceAddressHash
REF_1205(bytes32) -> agent_1 (-> ['TMP_2266']).underlyingAddressHash
TMP_2275(bool) = REF_1204 == REF_1205
TMP_2276(None) = SOLIDITY_CALL revert ChallengeNotAgentsAddress()()
TMP_2277(None) = SOLIDITY_CALL require(bool,error)(TMP_2275,TMP_2276)
 state.paymentConfirmations.transactionConfirmed(pmi)
REF_1206(PaymentConfirmations.State) -> state_1 (-> ['TMP_2265']).paymentConfirmations
TMP_2278(bool) = LIBRARY_CALL, dest:PaymentConfirmations, function:PaymentConfirmations.transactionConfirmed(PaymentConfirmations.State,IBalanceDecreasingTransaction.Proof), arguments:['REF_1206', 'pmi_1'] 
CONDITION TMP_2278
 paymentReference = pmi.data.responseBody.standardPaymentReference
REF_1208(IBalanceDecreasingTransaction.Response) -> pmi_1.data
REF_1209(IBalanceDecreasingTransaction.ResponseBody) -> REF_1208.responseBody
REF_1210(bytes32) -> REF_1209.standardPaymentReference
paymentReference_1(bytes32) := REF_1210(bytes32)
 PaymentReference.isValid(paymentReference,PaymentReference.REDEMPTION)
REF_1212(uint256) -> PaymentReference.REDEMPTION
TMP_2279(bool) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.isValid(bytes32,uint256), arguments:['paymentReference_1', 'REF_1212'] 
CONDITION TMP_2279
 redemptionId = PaymentReference.decodeId(pmi.data.responseBody.standardPaymentReference)
REF_1214(IBalanceDecreasingTransaction.Response) -> pmi_1.data
REF_1215(IBalanceDecreasingTransaction.ResponseBody) -> REF_1214.responseBody
REF_1216(bytes32) -> REF_1215.standardPaymentReference
TMP_2280(uint256) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.decodeId(bytes32), arguments:['REF_1216'] 
redemptionId_1(uint256) := TMP_2280(uint256)
 request = state.redemptionRequests[redemptionId]
REF_1217(mapping(uint256 => Redemption.Request)) -> state_1 (-> ['TMP_2265']).redemptionRequests
REF_1218(Redemption.Request) -> REF_1217[redemptionId_1]
request_1 (-> ['state'])(Redemption.Request) := REF_1218(Redemption.Request)
 total += pmi.data.responseBody.spentAmount - SafeCast.toInt256(redemptionValue)
REF_1219(IBalanceDecreasingTransaction.Response) -> pmi_1.data
REF_1220(IBalanceDecreasingTransaction.ResponseBody) -> REF_1219.responseBody
REF_1221(int256) -> REF_1220.spentAmount
TMP_2281(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['redemptionValue_3'] 
TMP_2282(int256) = REF_1221 (c)- TMP_2281
total_3(int256) = total_1 (c)+ TMP_2282
 total += pmi.data.responseBody.spentAmount
REF_1223(IBalanceDecreasingTransaction.Response) -> pmi_1.data
REF_1224(IBalanceDecreasingTransaction.ResponseBody) -> REF_1223.responseBody
REF_1225(int256) -> REF_1224.spentAmount
total_2(int256) = total_1 (c)+ REF_1225
total_4(int256) := phi(['total_1', 'total_3', 'total_2'])
 i ++
TMP_2283(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 balanceAfterPayments = agent.underlyingBalanceUBA - total
REF_1226(int128) -> agent_1 (-> ['TMP_2266']).underlyingBalanceUBA
TMP_2284(int128) = REF_1226 (c)- total_1
balanceAfterPayments_1(int256) := TMP_2284(int128)
 requiredBalance = UnderlyingBalance.requiredUnderlyingUBA(agent)
TMP_2285(uint256) = LIBRARY_CALL, dest:UnderlyingBalance, function:UnderlyingBalance.requiredUnderlyingUBA(Agent.State), arguments:["agent_1 (-> ['TMP_2266'])"] 
requiredBalance_1(uint256) := TMP_2285(uint256)
 require(bool,error)(balanceAfterPayments < requiredBalance.toInt256(),revert MultiplePaymentsChallengeEnoughBalance()())
TMP_2286(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['requiredBalance_1'] 
TMP_2287(bool) = balanceAfterPayments_1 < TMP_2286
TMP_2288(None) = SOLIDITY_CALL revert MultiplePaymentsChallengeEnoughBalance()()
TMP_2289(None) = SOLIDITY_CALL require(bool,error)(TMP_2287,TMP_2288)
 _liquidateAndRewardChallenger(agent,msg.sender,agent.mintedAMG)
REF_1229(uint64) -> agent_1 (-> ['TMP_2266']).mintedAMG
INTERNAL_CALL, ChallengesFacet._liquidateAndRewardChallenger(Agent.State,address,uint256)(agent_1 (-> ['TMP_2266']),msg.sender,REF_1229)
 IAssetManagerEvents.UnderlyingBalanceTooLow(_agentVault,balanceAfterPayments,requiredBalance)
Emit UnderlyingBalanceTooLow(_agentVault_1,balanceAfterPayments_1,requiredBalance_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 Redemptions.isOpen(request)
TMP_2293(bool) = LIBRARY_CALL, dest:Redemptions, function:Redemptions.isOpen(Redemption.Request), arguments:["request_1 (-> ['state'])"] 
CONDITION TMP_2293
 redemptionValue = request.underlyingValueUBA
REF_1232(uint128) -> request_1 (-> ['state']).underlyingValueUBA
redemptionValue_1(uint256) := REF_1232(uint128)
 redemptionValue = 0
redemptionValue_2(uint256) := 0(uint256)
redemptionValue_3(uint256) := phi(['redemptionValue_1', 'redemptionValue_2'])
```
#### ChallengesFacet.illegalPaymentChallenge(IBalanceDecreasingTransaction.Proof,address) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_2218(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2218'])(AssetManagerState.State) := TMP_2218(AssetManagerState.State)
 agent = Agent.get(_agentVault)
TMP_2219(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2219'])(Agent.State) := TMP_2219(Agent.State)
 _validateAgentStatus(agent)
INTERNAL_CALL, ChallengesFacet._validateAgentStatus(Agent.State)(agent_1 (-> ['TMP_2219']))
 TransactionAttestation.verifyBalanceDecreasingTransaction(_payment)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyBalanceDecreasingTransaction(IBalanceDecreasingTransaction.Proof), arguments:['_payment_1'] 
 require(bool,error)(_payment.data.responseBody.sourceAddressHash == agent.underlyingAddressHash,revert ChallengeNotAgentsAddress()())
REF_1134(IBalanceDecreasingTransaction.Response) -> _payment_1.data
REF_1135(IBalanceDecreasingTransaction.ResponseBody) -> REF_1134.responseBody
REF_1136(bytes32) -> REF_1135.sourceAddressHash
REF_1137(bytes32) -> agent_1 (-> ['TMP_2219']).underlyingAddressHash
TMP_2222(bool) = REF_1136 == REF_1137
TMP_2223(None) = SOLIDITY_CALL revert ChallengeNotAgentsAddress()()
TMP_2224(None) = SOLIDITY_CALL require(bool,error)(TMP_2222,TMP_2223)
 require(bool,error)(! state.paymentConfirmations.transactionConfirmed(_payment),revert ChallengeTransactionAlreadyConfirmed()())
REF_1138(PaymentConfirmations.State) -> state_1 (-> ['TMP_2218']).paymentConfirmations
TMP_2225(bool) = LIBRARY_CALL, dest:PaymentConfirmations, function:PaymentConfirmations.transactionConfirmed(PaymentConfirmations.State,IBalanceDecreasingTransaction.Proof), arguments:['REF_1138', '_payment_1'] 
TMP_2226 = UnaryType.BANG TMP_2225 
TMP_2227(None) = SOLIDITY_CALL revert ChallengeTransactionAlreadyConfirmed()()
TMP_2228(None) = SOLIDITY_CALL require(bool,error)(TMP_2226,TMP_2227)
 paymentReference = _payment.data.responseBody.standardPaymentReference
REF_1140(IBalanceDecreasingTransaction.Response) -> _payment_1.data
REF_1141(IBalanceDecreasingTransaction.ResponseBody) -> REF_1140.responseBody
REF_1142(bytes32) -> REF_1141.standardPaymentReference
paymentReference_1(bytes32) := REF_1142(bytes32)
 paymentReference != 0
TMP_2229(bool) = paymentReference_1 != 0
CONDITION TMP_2229
 PaymentReference.isValid(paymentReference,PaymentReference.REDEMPTION)
REF_1144(uint256) -> PaymentReference.REDEMPTION
TMP_2230(bool) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.isValid(bytes32,uint256), arguments:['paymentReference_1', 'REF_1144'] 
CONDITION TMP_2230
 redemptionId = PaymentReference.decodeId(paymentReference)
TMP_2231(uint256) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.decodeId(bytes32), arguments:['paymentReference_1'] 
redemptionId_1(uint256) := TMP_2231(uint256)
 redemption = state.redemptionRequests[redemptionId]
REF_1146(mapping(uint256 => Redemption.Request)) -> state_1 (-> ['TMP_2218']).redemptionRequests
REF_1147(Redemption.Request) -> REF_1146[redemptionId_1]
redemption_1 (-> ['state'])(Redemption.Request) := REF_1147(Redemption.Request)
 redemptionActive = redemption.agentVault == _agentVault && Redemptions.isOpen(redemption)
REF_1148(address) -> redemption_1 (-> ['state']).agentVault
TMP_2232(bool) = REF_1148 == _agentVault_1
TMP_2233(bool) = LIBRARY_CALL, dest:Redemptions, function:Redemptions.isOpen(Redemption.Request), arguments:["redemption_1 (-> ['state'])"] 
TMP_2234(bool) = TMP_2232 && TMP_2233
redemptionActive_1(bool) := TMP_2234(bool)
 require(bool,error)(! redemptionActive,revert MatchingRedemptionActive()())
TMP_2235 = UnaryType.BANG redemptionActive_1 
TMP_2236(None) = SOLIDITY_CALL revert MatchingRedemptionActive()()
TMP_2237(None) = SOLIDITY_CALL require(bool,error)(TMP_2235,TMP_2236)
 PaymentReference.isValid(paymentReference,PaymentReference.ANNOUNCED_WITHDRAWAL)
REF_1151(uint256) -> PaymentReference.ANNOUNCED_WITHDRAWAL
TMP_2238(bool) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.isValid(bytes32,uint256), arguments:['paymentReference_1', 'REF_1151'] 
CONDITION TMP_2238
 announcementId = PaymentReference.decodeId(paymentReference)
TMP_2239(uint256) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.decodeId(bytes32), arguments:['paymentReference_1'] 
announcementId_1(uint256) := TMP_2239(uint256)
 require(bool,error)(announcementId != agent.announcedUnderlyingWithdrawalId,revert MatchingAnnouncedPaymentActive()())
REF_1153(uint64) -> agent_1 (-> ['TMP_2219']).announcedUnderlyingWithdrawalId
TMP_2240(bool) = announcementId_1 != REF_1153
TMP_2241(None) = SOLIDITY_CALL revert MatchingAnnouncedPaymentActive()()
TMP_2242(None) = SOLIDITY_CALL require(bool,error)(TMP_2240,TMP_2241)
 _liquidateAndRewardChallenger(agent,msg.sender,agent.mintedAMG)
REF_1154(uint64) -> agent_1 (-> ['TMP_2219']).mintedAMG
INTERNAL_CALL, ChallengesFacet._liquidateAndRewardChallenger(Agent.State,address,uint256)(agent_1 (-> ['TMP_2219']),msg.sender,REF_1154)
 IAssetManagerEvents.IllegalPaymentConfirmed(_agentVault,_payment.data.requestBody.transactionId)
REF_1156(IBalanceDecreasingTransaction.Response) -> _payment_1.data
REF_1157(IBalanceDecreasingTransaction.RequestBody) -> REF_1156.requestBody
REF_1158(bytes32) -> REF_1157.transactionId
Emit IllegalPaymentConfirmed(_agentVault_1,REF_1158)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
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
#### Conversion.convertAmgToTokenWei(uint256,uint256) [INTERNAL]
```slithir
AMG_TOKEN_WEI_PRICE_SCALE_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_0'])
 _valueAMG.mulDiv(_amgToTokenWeiPrice,AMG_TOKEN_WEI_PRICE_SCALE)
TMP_4674(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_valueAMG_1', '_amgToTokenWeiPrice_1', 'AMG_TOKEN_WEI_PRICE_SCALE_1'] 
RETURN TMP_4674
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
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
```
#### TransactionAttestation.verifyBalanceDecreasingTransaction(IBalanceDecreasingTransaction.Proof) [INTERNAL]
```slithir
 _settings = Globals.getSettings()
TMP_5245(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5245'])(AssetManagerSettings.Data) := TMP_5245(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3676(address) -> _settings_1 (-> ['TMP_5245']).fdcVerification
TMP_5246 = CONVERT REF_3676 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5246(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3677(IBalanceDecreasingTransaction.Response) -> _proof_1.data
REF_3678(bytes32) -> REF_3677.sourceId
REF_3679(bytes32) -> _settings_1 (-> ['TMP_5245']).chainId
TMP_5247(bool) = REF_3678 == REF_3679
TMP_5248(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5249(None) = SOLIDITY_CALL require(bool,error)(TMP_5247,TMP_5248)
 require(bool,error)(fdcVerification.verifyBalanceDecreasingTransaction(_proof),revert TransactionNotProven()())
TMP_5250(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyBalanceDecreasingTransaction, arguments:['_proof_1']  
TMP_5251(None) = SOLIDITY_CALL revert TransactionNotProven()()
TMP_5252(None) = SOLIDITY_CALL require(bool,error)(TMP_5250,TMP_5251)
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
#### PaymentConfirmations.transactionConfirmed(PaymentConfirmations.State,IBalanceDecreasingTransaction.Proof) [INTERNAL]
```slithir
 txKey = transactionKey(_transaction.data.responseBody.sourceAddressHash,_transaction.data.requestBody.transactionId)
REF_3762(IBalanceDecreasingTransaction.Response) -> _transaction_1.data
REF_3763(IBalanceDecreasingTransaction.ResponseBody) -> REF_3762.responseBody
REF_3764(bytes32) -> REF_3763.sourceAddressHash
REF_3765(IBalanceDecreasingTransaction.Response) -> _transaction_1.data
REF_3766(IBalanceDecreasingTransaction.RequestBody) -> REF_3765.requestBody
REF_3767(bytes32) -> REF_3766.transactionId
TMP_5338(bytes32) = INTERNAL_CALL, PaymentConfirmations.transactionKey(bytes32,bytes32)(REF_3764,REF_3767)
txKey_1(bytes32) := TMP_5338(bytes32)
 _state.verifiedPayments[txKey] != 0
REF_3768(mapping(bytes32 => bytes32)) -> _state_1 (-> []).verifiedPayments
REF_3769(bytes32) -> REF_3768[txKey_1]
TMP_5339(bool) = REF_3769 != 0
RETURN TMP_5339
```
#### PaymentReference.decodeId(bytes32) [INTERNAL]
```slithir
LOW_BITS_MASK_2(uint256) := phi(['LOW_BITS_MASK_0'])
 uint256(_reference) & LOW_BITS_MASK
TMP_5380 = CONVERT _reference_1 to uint256
TMP_5381(uint256) = TMP_5380 & LOW_BITS_MASK_2
RETURN TMP_5381
```
#### PaymentReference.isValid(bytes32,uint256) [INTERNAL]
```slithir
TYPE_MASK_1(uint256) := phi(['TYPE_MASK_0'])
LOW_BITS_MASK_1(uint256) := phi(['LOW_BITS_MASK_0'])
 refType = uint256(_reference) & TYPE_MASK
TMP_5373 = CONVERT _reference_1 to uint256
TMP_5374(uint256) = TMP_5373 & TYPE_MASK_1
refType_1(uint256) := TMP_5374(uint256)
 refLowBits = uint256(_reference) & LOW_BITS_MASK
TMP_5375 = CONVERT _reference_1 to uint256
TMP_5376(uint256) = TMP_5375 & LOW_BITS_MASK_1
refLowBits_1(uint256) := TMP_5376(uint256)
 refType == _type && refLowBits != 0
TMP_5377(bool) = refType_1 == _type_1
TMP_5378(bool) = refLowBits_1 != 0
TMP_5379(bool) = TMP_5377 && TMP_5378
RETURN TMP_5379
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
#### Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data) [INTERNAL]
```slithir
_token_1 (-> [])(CollateralTypeInt.Data) := phi(['_toToken_1 (-> [])', '_fromToken_1 (-> [])'])
 (_price,None,None) = currentAmgPriceInTokenWeiWithTs(_token,false)
TUPLE_44(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool)(_token_1 (-> []),False)
_price_1(uint256)= UNPACK TUPLE_44 index: 0 
 _price
RETURN _price_1
```
#### ERC20.balanceOf(address) [PUBLIC]
```slithir
_balances_1(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
 _balances[account]
REF_73(uint256) -> _balances_1[account_1]
RETURN REF_73
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
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeWithSelector(token.transfer.selector,to,value))
REF_86(bytes4) (->None) := 2835717307(bytes4)
TMP_243(bytes) = SOLIDITY_CALL abi.encodeWithSelector()(REF_86,to_1,value_1)
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_243)
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
