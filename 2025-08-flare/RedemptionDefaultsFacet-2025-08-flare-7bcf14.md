



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






### Storage layout (CollateralPool) 

```text
agentVault address
assetManager IIAssetManager
fAsset IFAsset
token IICollateralPoolToken
wNat IWNat
exitCollateralRatioBIPS uint32
__topupCollateralRatioBIPS uint32
__topupTokenPriceFactorBIPS uint16
internalWithdrawal bool
initialized bool
_fAssetFeeDebtOf mapping(address => int256)
totalFAssetFeeDebt int256
totalFAssetFees uint256
totalCollateral uint256

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
### Storage layout (WNatMock) 

```text
governanceVP IGovernanceVotePower
delegations mapping(address => WNatMock.Delegation[])
delegators mapping(address => EnumerableSet.AddressSet)

```





#### RedemptionDefaultsFacet.finishRedemptionWithoutPayment(IConfirmedBlockHeightExists.Proof,uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3358(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3358'])(AssetManagerSettings.Data) := TMP_3358(AssetManagerSettings.Data)
 request = Redemptions.getRedemptionRequest(_redemptionRequestId,true)
TMP_3359(Redemption.Request) = LIBRARY_CALL, dest:Redemptions, function:Redemptions.getRedemptionRequest(uint256,bool), arguments:['_redemptionRequestId_1', 'True'] 
request_1 (-> ['TMP_3359'])(Redemption.Request) := TMP_3359(Redemption.Request)
 agent = Agent.get(request.agentVault)
REF_2172(address) -> request_1 (-> ['TMP_3359']).agentVault
TMP_3360(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_2172'] 
agent_1 (-> ['TMP_3360'])(Agent.State) := TMP_3360(Agent.State)
 Agents.requireAgentVaultOwner(agent)
LIBRARY_CALL, dest:Agents, function:Agents.requireAgentVaultOwner(Agent.State), arguments:["agent_1 (-> ['TMP_3360'])"] 
 request.status == Redemption.Status.ACTIVE
REF_2174(Redemption.Status) -> request_1 (-> ['TMP_3359']).status
REF_2175(Redemption.Status) -> Status.ACTIVE
TMP_3362(bool) = REF_2174 == REF_2175
CONDITION TMP_3362
 TransactionAttestation.verifyConfirmedBlockHeightExists(_proof)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyConfirmedBlockHeightExists(IConfirmedBlockHeightExists.Proof), arguments:['_proof_1'] 
 require(bool,error)(_proof.data.responseBody.lowestQueryWindowBlockNumber > request.lastUnderlyingBlock && _proof.data.responseBody.lowestQueryWindowBlockTimestamp > request.lastUnderlyingTimestamp && _proof.data.responseBody.lowestQueryWindowBlockTimestamp + settings.attestationWindowSeconds <= _proof.data.responseBody.blockTimestamp,revert ShouldDefaultFirst()())
REF_2177(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_2178(IConfirmedBlockHeightExists.ResponseBody) -> REF_2177.responseBody
REF_2179(uint64) -> REF_2178.lowestQueryWindowBlockNumber
REF_2180(uint64) -> request_1 (-> ['TMP_3359']).lastUnderlyingBlock
TMP_3364(bool) = REF_2179 > REF_2180
REF_2181(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_2182(IConfirmedBlockHeightExists.ResponseBody) -> REF_2181.responseBody
REF_2183(uint64) -> REF_2182.lowestQueryWindowBlockTimestamp
REF_2184(uint64) -> request_1 (-> ['TMP_3359']).lastUnderlyingTimestamp
TMP_3365(bool) = REF_2183 > REF_2184
TMP_3366(bool) = TMP_3364 && TMP_3365
REF_2185(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_2186(IConfirmedBlockHeightExists.ResponseBody) -> REF_2185.responseBody
REF_2187(uint64) -> REF_2186.lowestQueryWindowBlockTimestamp
REF_2188(uint64) -> settings_1 (-> ['TMP_3358']).attestationWindowSeconds
TMP_3367(uint64) = REF_2187 (c)+ REF_2188
REF_2189(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_2190(IConfirmedBlockHeightExists.ResponseBody) -> REF_2189.responseBody
REF_2191(uint64) -> REF_2190.blockTimestamp
TMP_3368(bool) = TMP_3367 <= REF_2191
TMP_3369(bool) = TMP_3366 && TMP_3368
TMP_3370(None) = SOLIDITY_CALL revert ShouldDefaultFirst()()
TMP_3371(None) = SOLIDITY_CALL require(bool,error)(TMP_3369,TMP_3370)
 RedemptionDefaults.executeDefaultOrCancel(agent,request,_redemptionRequestId)
LIBRARY_CALL, dest:RedemptionDefaults, function:RedemptionDefaults.executeDefaultOrCancel(Agent.State,Redemption.Request,uint256), arguments:["agent_1 (-> ['TMP_3360'])", "request_1 (-> ['TMP_3359'])", '_redemptionRequestId_1'] 
 Redemptions.burnExecutorFee(request)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.burnExecutorFee(Redemption.Request), arguments:["request_1 (-> ['TMP_3359'])"] 
 request.status = Redemption.Status.DEFAULTED
REF_2194(Redemption.Status) -> request_1 (-> ['TMP_3359']).status
REF_2195(Redemption.Status) -> Status.DEFAULTED
request_2 (-> ['TMP_3359'])(Redemption.Request) := phi(["request_1 (-> ['TMP_3359'])"])
REF_2194(Redemption.Status) (->request_2 (-> ['TMP_3359'])) := REF_2195(Redemption.Status)
TMP_3359(Redemption.Request) := phi(["request_2 (-> ['TMP_3359'])"])
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### RedemptionDefaultsFacet.redemptionPaymentDefault(IReferencedPaymentNonexistence.Proof,uint256) [EXTERNAL]
```slithir
 require(bool,error)(! _proof.data.requestBody.checkSourceAddresses,revert SourceAddressesNotSupported()())
REF_2127(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_2128(IReferencedPaymentNonexistence.RequestBody) -> REF_2127.requestBody
REF_2129(bool) -> REF_2128.checkSourceAddresses
TMP_3318 = UnaryType.BANG REF_2129 
TMP_3319(None) = SOLIDITY_CALL revert SourceAddressesNotSupported()()
TMP_3320(None) = SOLIDITY_CALL require(bool,error)(TMP_3318,TMP_3319)
 request = Redemptions.getRedemptionRequest(_redemptionRequestId,true)
TMP_3321(Redemption.Request) = LIBRARY_CALL, dest:Redemptions, function:Redemptions.getRedemptionRequest(uint256,bool), arguments:['_redemptionRequestId_1', 'True'] 
request_1 (-> ['TMP_3321'])(Redemption.Request) := TMP_3321(Redemption.Request)
 agent = Agent.get(request.agentVault)
REF_2132(address) -> request_1 (-> ['TMP_3321']).agentVault
TMP_3322(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_2132'] 
agent_1 (-> ['TMP_3322'])(Agent.State) := TMP_3322(Agent.State)
 require(bool,error)(request.status == Redemption.Status.ACTIVE,revert InvalidRedemptionStatus()())
REF_2133(Redemption.Status) -> request_1 (-> ['TMP_3321']).status
REF_2134(Redemption.Status) -> Status.ACTIVE
TMP_3323(bool) = REF_2133 == REF_2134
TMP_3324(None) = SOLIDITY_CALL revert InvalidRedemptionStatus()()
TMP_3325(None) = SOLIDITY_CALL require(bool,error)(TMP_3323,TMP_3324)
 TransactionAttestation.verifyReferencedPaymentNonexistence(_proof)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyReferencedPaymentNonexistence(IReferencedPaymentNonexistence.Proof), arguments:['_proof_1'] 
 require(bool,error)(_proof.data.requestBody.standardPaymentReference == PaymentReference.redemption(_redemptionRequestId) && _proof.data.requestBody.destinationAddressHash == request.redeemerUnderlyingAddressHash && _proof.data.requestBody.amount == request.underlyingValueUBA - request.underlyingFeeUBA,revert RedemptionNonPaymentMismatch()())
REF_2136(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_2137(IReferencedPaymentNonexistence.RequestBody) -> REF_2136.requestBody
REF_2138(bytes32) -> REF_2137.standardPaymentReference
TMP_3327(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.redemption(uint256), arguments:['_redemptionRequestId_1'] 
TMP_3328(bool) = REF_2138 == TMP_3327
REF_2140(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_2141(IReferencedPaymentNonexistence.RequestBody) -> REF_2140.requestBody
REF_2142(bytes32) -> REF_2141.destinationAddressHash
REF_2143(bytes32) -> request_1 (-> ['TMP_3321']).redeemerUnderlyingAddressHash
TMP_3329(bool) = REF_2142 == REF_2143
TMP_3330(bool) = TMP_3328 && TMP_3329
REF_2144(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_2145(IReferencedPaymentNonexistence.RequestBody) -> REF_2144.requestBody
REF_2146(uint256) -> REF_2145.amount
REF_2147(uint128) -> request_1 (-> ['TMP_3321']).underlyingValueUBA
REF_2148(uint128) -> request_1 (-> ['TMP_3321']).underlyingFeeUBA
TMP_3331(uint128) = REF_2147 (c)- REF_2148
TMP_3332(bool) = REF_2146 == TMP_3331
TMP_3333(bool) = TMP_3330 && TMP_3332
TMP_3334(None) = SOLIDITY_CALL revert RedemptionNonPaymentMismatch()()
TMP_3335(None) = SOLIDITY_CALL require(bool,error)(TMP_3333,TMP_3334)
 require(bool,error)(_proof.data.responseBody.firstOverflowBlockNumber > request.lastUnderlyingBlock && _proof.data.responseBody.firstOverflowBlockTimestamp > request.lastUnderlyingTimestamp,revert RedemptionDefaultTooEarly()())
REF_2149(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_2150(IReferencedPaymentNonexistence.ResponseBody) -> REF_2149.responseBody
REF_2151(uint64) -> REF_2150.firstOverflowBlockNumber
REF_2152(uint64) -> request_1 (-> ['TMP_3321']).lastUnderlyingBlock
TMP_3336(bool) = REF_2151 > REF_2152
REF_2153(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_2154(IReferencedPaymentNonexistence.ResponseBody) -> REF_2153.responseBody
REF_2155(uint64) -> REF_2154.firstOverflowBlockTimestamp
REF_2156(uint64) -> request_1 (-> ['TMP_3321']).lastUnderlyingTimestamp
TMP_3337(bool) = REF_2155 > REF_2156
TMP_3338(bool) = TMP_3336 && TMP_3337
TMP_3339(None) = SOLIDITY_CALL revert RedemptionDefaultTooEarly()()
TMP_3340(None) = SOLIDITY_CALL require(bool,error)(TMP_3338,TMP_3339)
 require(bool,error)(_proof.data.requestBody.minimalBlockNumber <= request.firstUnderlyingBlock,revert RedemptionNonPaymentProofWindowTooShort()())
REF_2157(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_2158(IReferencedPaymentNonexistence.RequestBody) -> REF_2157.requestBody
REF_2159(uint64) -> REF_2158.minimalBlockNumber
REF_2160(uint64) -> request_1 (-> ['TMP_3321']).firstUnderlyingBlock
TMP_3341(bool) = REF_2159 <= REF_2160
TMP_3342(None) = SOLIDITY_CALL revert RedemptionNonPaymentProofWindowTooShort()()
TMP_3343(None) = SOLIDITY_CALL require(bool,error)(TMP_3341,TMP_3342)
 expectedSender = msg.sender == request.redeemer || msg.sender == request.executor || Agents.isOwner(agent,msg.sender)
REF_2161(address) -> request_1 (-> ['TMP_3321']).redeemer
TMP_3344(bool) = msg.sender == REF_2161
REF_2162(address) -> request_1 (-> ['TMP_3321']).executor
TMP_3345(bool) = msg.sender == REF_2162
TMP_3346(bool) = TMP_3344 || TMP_3345
TMP_3347(bool) = LIBRARY_CALL, dest:Agents, function:Agents.isOwner(Agent.State,address), arguments:["agent_1 (-> ['TMP_3322'])", 'msg.sender'] 
TMP_3348(bool) = TMP_3346 || TMP_3347
expectedSender_1(bool) := TMP_3348(bool)
 require(bool,error)(expectedSender || _othersCanConfirmDefault(request),revert OnlyRedeemerExecutorOrAgent()())
TMP_3349(bool) = INTERNAL_CALL, RedemptionDefaultsFacet._othersCanConfirmDefault(Redemption.Request)(request_1 (-> ['TMP_3321']))
TMP_3350(bool) = expectedSender_1 || TMP_3349
TMP_3351(None) = SOLIDITY_CALL revert OnlyRedeemerExecutorOrAgent()()
TMP_3352(None) = SOLIDITY_CALL require(bool,error)(TMP_3350,TMP_3351)
 RedemptionDefaults.executeDefaultOrCancel(agent,request,_redemptionRequestId)
LIBRARY_CALL, dest:RedemptionDefaults, function:RedemptionDefaults.executeDefaultOrCancel(Agent.State,Redemption.Request,uint256), arguments:["agent_1 (-> ['TMP_3322'])", "request_1 (-> ['TMP_3321'])", '_redemptionRequestId_1'] 
 ! expectedSender
TMP_3354 = UnaryType.BANG expectedSender_1 
CONDITION TMP_3354
 AgentPayout.payForConfirmationByOthers(agent,msg.sender)
LIBRARY_CALL, dest:AgentPayout, function:AgentPayout.payForConfirmationByOthers(Agent.State,address), arguments:["agent_1 (-> ['TMP_3322'])", 'msg.sender'] 
 Redemptions.payOrBurnExecutorFee(request)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.payOrBurnExecutorFee(Redemption.Request), arguments:["request_1 (-> ['TMP_3321'])"] 
 request.status = Redemption.Status.DEFAULTED
REF_2167(Redemption.Status) -> request_1 (-> ['TMP_3321']).status
REF_2168(Redemption.Status) -> Status.DEFAULTED
request_2 (-> ['TMP_3321'])(Redemption.Request) := phi(["request_1 (-> ['TMP_3321'])"])
REF_2167(Redemption.Status) (->request_2 (-> ['TMP_3321'])) := REF_2168(Redemption.Status)
TMP_3321(Redemption.Request) := phi(["request_2 (-> ['TMP_3321'])"])
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
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
#### Agents.requireAgentVaultOwner(Agent.State) [INTERNAL]
```slithir
 require(bool,error)(isOwner(_agent,msg.sender),revert OnlyAgentVaultOwner()())
TMP_4497(bool) = INTERNAL_CALL, Agents.isOwner(Agent.State,address)(_agent_1 (-> []),msg.sender)
TMP_4498(None) = SOLIDITY_CALL revert OnlyAgentVaultOwner()()
TMP_4499(None) = SOLIDITY_CALL require(bool,error)(TMP_4497,TMP_4498)
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
#### TransactionAttestation.verifyReferencedPaymentNonexistence(IReferencedPaymentNonexistence.Proof) [INTERNAL]
```slithir
 _settings = Globals.getSettings()
TMP_5261(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5261'])(AssetManagerSettings.Data) := TMP_5261(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3688(address) -> _settings_1 (-> ['TMP_5261']).fdcVerification
TMP_5262 = CONVERT REF_3688 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5262(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3689(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_3690(bytes32) -> REF_3689.sourceId
REF_3691(bytes32) -> _settings_1 (-> ['TMP_5261']).chainId
TMP_5263(bool) = REF_3690 == REF_3691
TMP_5264(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5265(None) = SOLIDITY_CALL require(bool,error)(TMP_5263,TMP_5264)
 require(bool,error)(fdcVerification.verifyReferencedPaymentNonexistence(_proof),revert NonPaymentNotProven()())
TMP_5266(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyReferencedPaymentNonexistence, arguments:['_proof_1']  
TMP_5267(None) = SOLIDITY_CALL revert NonPaymentNotProven()()
TMP_5268(None) = SOLIDITY_CALL require(bool,error)(TMP_5266,TMP_5267)
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
#### Globals.getBurnAddress() [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4739(AssetManagerSettings.Data) = INTERNAL_CALL, Globals.getSettings()()
settings_1 (-> ['TMP_4739'])(AssetManagerSettings.Data) := TMP_4739(AssetManagerSettings.Data)
 settings.burnAddress
REF_3244(address) -> settings_1 (-> ['TMP_4739']).burnAddress
RETURN REF_3244
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
#### CollateralPool.payout(address,uint256,uint256) [EXTERNAL]
```slithir
agentVault_17(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
token_40(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_14(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 agentTokenBalance = token.balanceOf(agentVault)
TMP_6206(uint256) = HIGH_LEVEL_CALL, dest:token_42(IICollateralPoolToken), function:balanceOf, arguments:['agentVault_19']  
agentVault_20(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_19', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
token_43(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_42', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_17(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_16', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
agentTokenBalance_1(uint256) := TMP_6206(uint256)
 slashedTokens = Math.min(maxSlashedTokens,agentTokenBalance)
TMP_6207(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['maxSlashedTokens_3', 'agentTokenBalance_1'] 
slashedTokens_1(uint256) := TMP_6207(uint256)
 slashedTokens > 0
TMP_6208(bool) = slashedTokens_1 > 0
CONDITION TMP_6208
 debtFAssetFeeShare = _tokensToVirtualFeeShare(slashedTokens)
TMP_6209(uint256) = INTERNAL_CALL, CollateralPool._tokensToVirtualFeeShare(uint256)(slashedTokens_1)
token_45(IICollateralPoolToken) := phi(['token_51'])
debtFAssetFeeShare_1(uint256) := TMP_6209(uint256)
 _deleteFAssetFeeDebt(agentVault,debtFAssetFeeShare)
INTERNAL_CALL, CollateralPool._deleteFAssetFeeDebt(address,uint256)(agentVault_22,debtFAssetFeeShare_1)
 token.burn(agentVault,slashedTokens,true)
HIGH_LEVEL_CALL, dest:token_46(IICollateralPoolToken), function:burn, arguments:['agentVault_23', 'slashedTokens_1', 'True']  
agentVault_24(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_23', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
token_47(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_46', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 _transferWNatTo(_recipient,_amount)
INTERNAL_CALL, CollateralPool._transferWNatTo(address,uint256)(_recipient_1,_amount_1)
 CPPaidOut(_recipient,_amount,slashedTokens)
Emit CPPaidOut(_recipient_1,_amount_1,slashedTokens_1)
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 totalCollateral > 0
TMP_6216(bool) = totalCollateral_17 > 0
CONDITION TMP_6216
 maxSlashedTokens = token.totalSupply().mulDivRoundUp(_agentResponsibilityWei,totalCollateral)
TMP_6217(uint256) = HIGH_LEVEL_CALL, dest:token_43(IICollateralPoolToken), function:totalSupply, arguments:[]  
agentVault_21(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
token_44(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_18(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
TMP_6218(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDivRoundUp(uint256,uint256,uint256), arguments:['TMP_6217', '_agentResponsibilityWei_1', 'totalCollateral_18'] 
maxSlashedTokens_1(uint256) := TMP_6218(uint256)
 maxSlashedTokens = agentTokenBalance
maxSlashedTokens_2(uint256) := agentTokenBalance_1(uint256)
maxSlashedTokens_3(uint256) := phi(['maxSlashedTokens_1', 'maxSlashedTokens_2'])
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
#### AgentVault.payout(IERC20,address,uint256) [EXTERNAL]
```slithir
 _token.safeTransfer(_recipient,_amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['_token_1', '_recipient_1', '_amount_1'] 
 onlyAssetManager()
MODIFIER_CALL, AgentVault.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
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
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
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
#### WNatMock.depositTo(address) [PUBLIC]
```slithir
 _mint(_recipient,msg.value)
INTERNAL_CALL, ERC20._mint(address,uint256)(_recipient_1,msg.value)
```
