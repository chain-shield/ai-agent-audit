







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




### Storage layout (WNatMock) 

```text
governanceVP IGovernanceVotePower
delegations mapping(address => WNatMock.Delegation[])
delegators mapping(address => EnumerableSet.AddressSet)

```



#### MintingFacet.executeMinting(IPayment.Proof,uint256) [EXTERNAL]
```slithir
 crt = Minting.getCollateralReservation(_crtId,true)
TMP_3072(CollateralReservation.Data) = LIBRARY_CALL, dest:Minting, function:Minting.getCollateralReservation(uint256,bool), arguments:['_crtId_1', 'True'] 
crt_1 (-> ['TMP_3072'])(CollateralReservation.Data) := TMP_3072(CollateralReservation.Data)
 agent = Agent.get(crt.agentVault)
REF_1863(address) -> crt_1 (-> ['TMP_3072']).agentVault
TMP_3073(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_1863'] 
agent_1 (-> ['TMP_3073'])(Agent.State) := TMP_3073(Agent.State)
 TransactionAttestation.verifyPaymentSuccess(_payment)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyPaymentSuccess(IPayment.Proof), arguments:['_payment_1'] 
 require(bool,error)(msg.sender == crt.minter || msg.sender == crt.executor || Agents.isOwner(agent,msg.sender),revert OnlyMinterExecutorOrAgent()())
REF_1865(address) -> crt_1 (-> ['TMP_3072']).minter
TMP_3075(bool) = msg.sender == REF_1865
REF_1866(address) -> crt_1 (-> ['TMP_3072']).executor
TMP_3076(bool) = msg.sender == REF_1866
TMP_3077(bool) = TMP_3075 || TMP_3076
TMP_3078(bool) = LIBRARY_CALL, dest:Agents, function:Agents.isOwner(Agent.State,address), arguments:["agent_1 (-> ['TMP_3073'])", 'msg.sender'] 
TMP_3079(bool) = TMP_3077 || TMP_3078
TMP_3080(None) = SOLIDITY_CALL revert OnlyMinterExecutorOrAgent()()
TMP_3081(None) = SOLIDITY_CALL require(bool,error)(TMP_3079,TMP_3080)
 require(bool,error)(_payment.data.responseBody.standardPaymentReference == PaymentReference.minting(_crtId),revert InvalidMintingReference()())
REF_1868(IPayment.Response) -> _payment_1.data
REF_1869(IPayment.ResponseBody) -> REF_1868.responseBody
REF_1870(bytes32) -> REF_1869.standardPaymentReference
TMP_3082(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.minting(uint256), arguments:['_crtId_1'] 
TMP_3083(bool) = REF_1870 == TMP_3082
TMP_3084(None) = SOLIDITY_CALL revert InvalidMintingReference()()
TMP_3085(None) = SOLIDITY_CALL require(bool,error)(TMP_3083,TMP_3084)
 require(bool,error)(_payment.data.responseBody.receivingAddressHash == agent.underlyingAddressHash,revert NotMintingAgentsAddress()())
REF_1872(IPayment.Response) -> _payment_1.data
REF_1873(IPayment.ResponseBody) -> REF_1872.responseBody
REF_1874(bytes32) -> REF_1873.receivingAddressHash
REF_1875(bytes32) -> agent_1 (-> ['TMP_3073']).underlyingAddressHash
TMP_3086(bool) = REF_1874 == REF_1875
TMP_3087(None) = SOLIDITY_CALL revert NotMintingAgentsAddress()()
TMP_3088(None) = SOLIDITY_CALL require(bool,error)(TMP_3086,TMP_3087)
 mintValueUBA = Conversion.convertAmgToUBA(crt.valueAMG)
REF_1877(uint64) -> crt_1 (-> ['TMP_3072']).valueAMG
TMP_3089(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_1877'] 
mintValueUBA_1(uint256) := TMP_3089(uint256)
 require(bool,error)(_payment.data.responseBody.receivedAmount >= SafeCast.toInt256(mintValueUBA + crt.underlyingFeeUBA),revert MintingPaymentTooSmall()())
REF_1878(IPayment.Response) -> _payment_1.data
REF_1879(IPayment.ResponseBody) -> REF_1878.responseBody
REF_1880(int256) -> REF_1879.receivedAmount
REF_1882(uint128) -> crt_1 (-> ['TMP_3072']).underlyingFeeUBA
TMP_3090(uint256) = mintValueUBA_1 (c)+ REF_1882
TMP_3091(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['TMP_3090'] 
TMP_3092(bool) = REF_1880 >= TMP_3091
TMP_3093(None) = SOLIDITY_CALL revert MintingPaymentTooSmall()()
TMP_3094(None) = SOLIDITY_CALL require(bool,error)(TMP_3092,TMP_3093)
 require(bool,error)(_payment.data.responseBody.blockNumber >= crt.firstUnderlyingBlock,revert MintingPaymentTooOld()())
REF_1883(IPayment.Response) -> _payment_1.data
REF_1884(IPayment.ResponseBody) -> REF_1883.responseBody
REF_1885(uint64) -> REF_1884.blockNumber
REF_1886(uint64) -> crt_1 (-> ['TMP_3072']).firstUnderlyingBlock
TMP_3095(bool) = REF_1885 >= REF_1886
TMP_3096(None) = SOLIDITY_CALL revert MintingPaymentTooOld()()
TMP_3097(None) = SOLIDITY_CALL require(bool,error)(TMP_3095,TMP_3096)
 AssetManagerState.get().paymentConfirmations.confirmIncomingPayment(_payment)
TMP_3098(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
REF_1888(PaymentConfirmations.State) -> TMP_3098.paymentConfirmations
LIBRARY_CALL, dest:PaymentConfirmations, function:PaymentConfirmations.confirmIncomingPayment(PaymentConfirmations.State,IPayment.Proof), arguments:['REF_1888', '_payment_1'] 
 _performMinting(agent,MintingType.PUBLIC,_crtId,crt.minter,crt.valueAMG,uint256(_payment.data.responseBody.receivedAmount),Minting.calculatePoolFeeUBA(agent,crt))
REF_1890(MintingFacet.MintingType) -> MintingType.PUBLIC
REF_1891(address) -> crt_1 (-> ['TMP_3072']).minter
REF_1892(uint64) -> crt_1 (-> ['TMP_3072']).valueAMG
REF_1893(IPayment.Response) -> _payment_1.data
REF_1894(IPayment.ResponseBody) -> REF_1893.responseBody
REF_1895(int256) -> REF_1894.receivedAmount
TMP_3100 = CONVERT REF_1895 to uint256
TMP_3101(uint256) = LIBRARY_CALL, dest:Minting, function:Minting.calculatePoolFeeUBA(Agent.State,CollateralReservation.Data), arguments:["agent_1 (-> ['TMP_3073'])", "crt_1 (-> ['TMP_3072'])"] 
INTERNAL_CALL, MintingFacet._performMinting(Agent.State,MintingFacet.MintingType,uint256,address,uint64,uint256,uint256)(agent_1 (-> ['TMP_3073']),REF_1890,_crtId_1,REF_1891,REF_1892,TMP_3100,TMP_3101)
 UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(_payment)
LIBRARY_CALL, dest:UnderlyingBlockUpdater, function:UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(IPayment.Proof), arguments:['_payment_1'] 
 Minting.releaseCollateralReservation(crt,CollateralReservation.Status.SUCCESSFUL)
REF_1899(CollateralReservation.Status) -> Status.SUCCESSFUL
LIBRARY_CALL, dest:Minting, function:Minting.releaseCollateralReservation(CollateralReservation.Data,CollateralReservation.Status), arguments:["crt_1 (-> ['TMP_3072'])", 'REF_1899'] 
 Minting.payOrBurnExecutorFee(crt)
LIBRARY_CALL, dest:Minting, function:Minting.payOrBurnExecutorFee(CollateralReservation.Data), arguments:["crt_1 (-> ['TMP_3072'])"] 
 Minting.distributeCollateralReservationFee(agent,crt.reservationFeeNatWei)
REF_1902(uint128) -> crt_1 (-> ['TMP_3072']).reservationFeeNatWei
LIBRARY_CALL, dest:Minting, function:Minting.distributeCollateralReservationFee(Agent.State,uint256), arguments:["agent_1 (-> ['TMP_3073'])", 'REF_1902'] 
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### MintingFacet.mintFromFreeUnderlying(address,uint64) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_3154(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_3154'])(AssetManagerState.State) := TMP_3154(AssetManagerState.State)
 agent = Agent.get(_agentVault)
TMP_3155(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_3155'])(Agent.State) := TMP_3155(Agent.State)
 Agents.requireAgentVaultOwner(agent)
LIBRARY_CALL, dest:Agents, function:Agents.requireAgentVaultOwner(Agent.State), arguments:["agent_1 (-> ['TMP_3155'])"] 
 Agents.requireWhitelistedAgentVaultOwner(agent)
LIBRARY_CALL, dest:Agents, function:Agents.requireWhitelistedAgentVaultOwner(Agent.State), arguments:["agent_1 (-> ['TMP_3155'])"] 
 collateralData = AgentCollateral.combinedData(agent)
TMP_3158(Collateral.CombinedData) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.combinedData(Agent.State), arguments:["agent_1 (-> ['TMP_3155'])"] 
collateralData_1(Collateral.CombinedData) := TMP_3158(Collateral.CombinedData)
 require(bool,error)(state.mintingPausedAt == 0,revert MintingPaused()())
REF_1948(uint64) -> state_1 (-> ['TMP_3154']).mintingPausedAt
TMP_3159(bool) = REF_1948 == 0
TMP_3160(None) = SOLIDITY_CALL revert MintingPaused()()
TMP_3161(None) = SOLIDITY_CALL require(bool,error)(TMP_3159,TMP_3160)
 require(bool,error)(_lots > 0,revert CannotMintZeroLots()())
TMP_3162(bool) = _lots_1 > 0
TMP_3163(None) = SOLIDITY_CALL revert CannotMintZeroLots()()
TMP_3164(None) = SOLIDITY_CALL require(bool,error)(TMP_3162,TMP_3163)
 require(bool,error)(agent.status == Agent.Status.NORMAL,revert SelfMintInvalidAgentStatus()())
REF_1949(Agent.Status) -> agent_1 (-> ['TMP_3155']).status
REF_1950(Agent.Status) -> Status.NORMAL
TMP_3165(bool) = REF_1949 == REF_1950
TMP_3166(None) = SOLIDITY_CALL revert SelfMintInvalidAgentStatus()()
TMP_3167(None) = SOLIDITY_CALL require(bool,error)(TMP_3165,TMP_3166)
 require(bool,error)(collateralData.freeCollateralLots(agent) >= _lots,revert NotEnoughFreeCollateral()())
TMP_3168(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.freeCollateralLots(Collateral.CombinedData,Agent.State), arguments:['collateralData_1', "agent_1 (-> ['TMP_3155'])"] 
TMP_3169(bool) = TMP_3168 >= _lots_1
TMP_3170(None) = SOLIDITY_CALL revert NotEnoughFreeCollateral()()
TMP_3171(None) = SOLIDITY_CALL require(bool,error)(TMP_3169,TMP_3170)
 valueAMG = Conversion.convertLotsToAMG(_lots)
TMP_3172(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertLotsToAMG(uint256), arguments:['_lots_1'] 
valueAMG_1(uint64) := TMP_3172(uint64)
 mintValueUBA = Conversion.convertAmgToUBA(valueAMG)
TMP_3173(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['valueAMG_1'] 
mintValueUBA_1(uint256) := TMP_3173(uint256)
 poolFeeUBA = Minting.calculateCurrentPoolFeeUBA(agent,mintValueUBA)
TMP_3174(uint256) = LIBRARY_CALL, dest:Minting, function:Minting.calculateCurrentPoolFeeUBA(Agent.State,uint256), arguments:["agent_1 (-> ['TMP_3155'])", 'mintValueUBA_1'] 
poolFeeUBA_1(uint256) := TMP_3174(uint256)
 Minting.checkMintingCap(valueAMG + Conversion.convertUBAToAmg(poolFeeUBA))
TMP_3175(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['poolFeeUBA_1'] 
TMP_3176(uint64) = valueAMG_1 (c)+ TMP_3175
LIBRARY_CALL, dest:Minting, function:Minting.checkMintingCap(uint64), arguments:['TMP_3176'] 
 requiredUnderlyingAfter = UnderlyingBalance.requiredUnderlyingUBA(agent) + mintValueUBA + poolFeeUBA
TMP_3178(uint256) = LIBRARY_CALL, dest:UnderlyingBalance, function:UnderlyingBalance.requiredUnderlyingUBA(Agent.State), arguments:["agent_1 (-> ['TMP_3155'])"] 
TMP_3179(uint256) = TMP_3178 (c)+ mintValueUBA_1
TMP_3180(uint256) = TMP_3179 (c)+ poolFeeUBA_1
requiredUnderlyingAfter_1(uint256) := TMP_3180(uint256)
 require(bool,error)(requiredUnderlyingAfter.toInt256() <= agent.underlyingBalanceUBA,revert FreeUnderlyingBalanceToSmall()())
TMP_3181(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['requiredUnderlyingAfter_1'] 
REF_1959(int128) -> agent_1 (-> ['TMP_3155']).underlyingBalanceUBA
TMP_3182(bool) = TMP_3181 <= REF_1959
TMP_3183(None) = SOLIDITY_CALL revert FreeUnderlyingBalanceToSmall()()
TMP_3184(None) = SOLIDITY_CALL require(bool,error)(TMP_3182,TMP_3183)
 _performMinting(agent,MintingType.FROM_FREE_UNDERLYING,0,msg.sender,valueAMG,0,poolFeeUBA)
REF_1960(MintingFacet.MintingType) -> MintingType.FROM_FREE_UNDERLYING
INTERNAL_CALL, MintingFacet._performMinting(Agent.State,MintingFacet.MintingType,uint256,address,uint64,uint256,uint256)(agent_1 (-> ['TMP_3155']),REF_1960,0,msg.sender,valueAMG_1,0,poolFeeUBA_1)
 onlyAttached()
MODIFIER_CALL, AssetManagerBase.onlyAttached()()
 notEmergencyPaused()
MODIFIER_CALL, AssetManagerBase.notEmergencyPaused()()
```
#### MintingFacet.selfMint(IPayment.Proof,address,uint256) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_3108(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_3108'])(AssetManagerState.State) := TMP_3108(AssetManagerState.State)
 agent = Agent.get(_agentVault)
TMP_3109(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_3109'])(Agent.State) := TMP_3109(Agent.State)
 Agents.requireAgentVaultOwner(agent)
LIBRARY_CALL, dest:Agents, function:Agents.requireAgentVaultOwner(Agent.State), arguments:["agent_1 (-> ['TMP_3109'])"] 
 Agents.requireWhitelistedAgentVaultOwner(agent)
LIBRARY_CALL, dest:Agents, function:Agents.requireWhitelistedAgentVaultOwner(Agent.State), arguments:["agent_1 (-> ['TMP_3109'])"] 
 collateralData = AgentCollateral.combinedData(agent)
TMP_3112(Collateral.CombinedData) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.combinedData(Agent.State), arguments:["agent_1 (-> ['TMP_3109'])"] 
collateralData_1(Collateral.CombinedData) := TMP_3112(Collateral.CombinedData)
 TransactionAttestation.verifyPaymentSuccess(_payment)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyPaymentSuccess(IPayment.Proof), arguments:['_payment_1'] 
 require(bool,error)(state.mintingPausedAt == 0,revert MintingPaused()())
REF_1909(uint64) -> state_1 (-> ['TMP_3108']).mintingPausedAt
TMP_3114(bool) = REF_1909 == 0
TMP_3115(None) = SOLIDITY_CALL revert MintingPaused()()
TMP_3116(None) = SOLIDITY_CALL require(bool,error)(TMP_3114,TMP_3115)
 require(bool,error)(agent.status == Agent.Status.NORMAL,revert SelfMintInvalidAgentStatus()())
REF_1910(Agent.Status) -> agent_1 (-> ['TMP_3109']).status
REF_1911(Agent.Status) -> Status.NORMAL
TMP_3117(bool) = REF_1910 == REF_1911
TMP_3118(None) = SOLIDITY_CALL revert SelfMintInvalidAgentStatus()()
TMP_3119(None) = SOLIDITY_CALL require(bool,error)(TMP_3117,TMP_3118)
 require(bool,error)(collateralData.freeCollateralLots(agent) >= _lots,revert NotEnoughFreeCollateral()())
TMP_3120(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.freeCollateralLots(Collateral.CombinedData,Agent.State), arguments:['collateralData_1', "agent_1 (-> ['TMP_3109'])"] 
TMP_3121(bool) = TMP_3120 >= _lots_1
TMP_3122(None) = SOLIDITY_CALL revert NotEnoughFreeCollateral()()
TMP_3123(None) = SOLIDITY_CALL require(bool,error)(TMP_3121,TMP_3122)
 valueAMG = Conversion.convertLotsToAMG(_lots)
TMP_3124(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertLotsToAMG(uint256), arguments:['_lots_1'] 
valueAMG_1(uint64) := TMP_3124(uint64)
 mintValueUBA = Conversion.convertAmgToUBA(valueAMG)
TMP_3125(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['valueAMG_1'] 
mintValueUBA_1(uint256) := TMP_3125(uint256)
 poolFeeUBA = Minting.calculateCurrentPoolFeeUBA(agent,mintValueUBA)
TMP_3126(uint256) = LIBRARY_CALL, dest:Minting, function:Minting.calculateCurrentPoolFeeUBA(Agent.State,uint256), arguments:["agent_1 (-> ['TMP_3109'])", 'mintValueUBA_1'] 
poolFeeUBA_1(uint256) := TMP_3126(uint256)
 Minting.checkMintingCap(valueAMG + Conversion.convertUBAToAmg(poolFeeUBA))
TMP_3127(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['poolFeeUBA_1'] 
TMP_3128(uint64) = valueAMG_1 (c)+ TMP_3127
LIBRARY_CALL, dest:Minting, function:Minting.checkMintingCap(uint64), arguments:['TMP_3128'] 
 require(bool,error)(_payment.data.responseBody.standardPaymentReference == PaymentReference.selfMint(_agentVault),revert InvalidSelfMintReference()())
REF_1918(IPayment.Response) -> _payment_1.data
REF_1919(IPayment.ResponseBody) -> REF_1918.responseBody
REF_1920(bytes32) -> REF_1919.standardPaymentReference
TMP_3130(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.selfMint(address), arguments:['_agentVault_1'] 
TMP_3131(bool) = REF_1920 == TMP_3130
TMP_3132(None) = SOLIDITY_CALL revert InvalidSelfMintReference()()
TMP_3133(None) = SOLIDITY_CALL require(bool,error)(TMP_3131,TMP_3132)
 require(bool,error)(_payment.data.responseBody.receivingAddressHash == agent.underlyingAddressHash,revert SelfMintNotAgentsAddress()())
REF_1922(IPayment.Response) -> _payment_1.data
REF_1923(IPayment.ResponseBody) -> REF_1922.responseBody
REF_1924(bytes32) -> REF_1923.receivingAddressHash
REF_1925(bytes32) -> agent_1 (-> ['TMP_3109']).underlyingAddressHash
TMP_3134(bool) = REF_1924 == REF_1925
TMP_3135(None) = SOLIDITY_CALL revert SelfMintNotAgentsAddress()()
TMP_3136(None) = SOLIDITY_CALL require(bool,error)(TMP_3134,TMP_3135)
 require(bool,error)(_payment.data.responseBody.receivedAmount >= SafeCast.toInt256(mintValueUBA + poolFeeUBA),revert SelfMintPaymentTooSmall()())
REF_1926(IPayment.Response) -> _payment_1.data
REF_1927(IPayment.ResponseBody) -> REF_1926.responseBody
REF_1928(int256) -> REF_1927.receivedAmount
TMP_3137(uint256) = mintValueUBA_1 (c)+ poolFeeUBA_1
TMP_3138(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['TMP_3137'] 
TMP_3139(bool) = REF_1928 >= TMP_3138
TMP_3140(None) = SOLIDITY_CALL revert SelfMintPaymentTooSmall()()
TMP_3141(None) = SOLIDITY_CALL require(bool,error)(TMP_3139,TMP_3140)
 require(bool,error)(_payment.data.responseBody.blockNumber > agent.underlyingBlockAtCreation,revert SelfMintPaymentTooOld()())
REF_1930(IPayment.Response) -> _payment_1.data
REF_1931(IPayment.ResponseBody) -> REF_1930.responseBody
REF_1932(uint64) -> REF_1931.blockNumber
REF_1933(uint64) -> agent_1 (-> ['TMP_3109']).underlyingBlockAtCreation
TMP_3142(bool) = REF_1932 > REF_1933
TMP_3143(None) = SOLIDITY_CALL revert SelfMintPaymentTooOld()()
TMP_3144(None) = SOLIDITY_CALL require(bool,error)(TMP_3142,TMP_3143)
 state.paymentConfirmations.confirmIncomingPayment(_payment)
REF_1934(PaymentConfirmations.State) -> state_1 (-> ['TMP_3108']).paymentConfirmations
LIBRARY_CALL, dest:PaymentConfirmations, function:PaymentConfirmations.confirmIncomingPayment(PaymentConfirmations.State,IPayment.Proof), arguments:['REF_1934', '_payment_1'] 
 UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(_payment)
LIBRARY_CALL, dest:UnderlyingBlockUpdater, function:UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(IPayment.Proof), arguments:['_payment_1'] 
 receivedAmount = uint256(_payment.data.responseBody.receivedAmount)
REF_1937(IPayment.Response) -> _payment_1.data
REF_1938(IPayment.ResponseBody) -> REF_1937.responseBody
REF_1939(int256) -> REF_1938.receivedAmount
TMP_3147 = CONVERT REF_1939 to uint256
receivedAmount_1(uint256) := TMP_3147(uint256)
 _lots > 0
TMP_3148(bool) = _lots_1 > 0
CONDITION TMP_3148
 _performMinting(agent,MintingType.SELF_MINT,0,msg.sender,valueAMG,receivedAmount,poolFeeUBA)
REF_1940(MintingFacet.MintingType) -> MintingType.SELF_MINT
INTERNAL_CALL, MintingFacet._performMinting(Agent.State,MintingFacet.MintingType,uint256,address,uint64,uint256,uint256)(agent_1 (-> ['TMP_3109']),REF_1940,0,msg.sender,valueAMG_1,receivedAmount_1,poolFeeUBA_1)
 UnderlyingBalance.increaseBalance(agent,receivedAmount)
LIBRARY_CALL, dest:UnderlyingBalance, function:UnderlyingBalance.increaseBalance(Agent.State,uint256), arguments:["agent_1 (-> ['TMP_3109'])", 'receivedAmount_1'] 
 IAssetManagerEvents.SelfMint(_agentVault,false,0,receivedAmount,0)
Emit SelfMint(_agentVault_1,False,0,receivedAmount_1,0)
 onlyAttached()
MODIFIER_CALL, AssetManagerBase.onlyAttached()()
 notEmergencyPaused()
MODIFIER_CALL, AssetManagerBase.notEmergencyPaused()()
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
#### CollateralPool.fAssetFeeDeposited(uint256) [EXTERNAL]
```slithir
totalFAssetFees_8(uint256) := phi(['totalFAssetFees_3', 'totalFAssetFees_0', 'totalFAssetFees_14', 'totalFAssetFees_12', 'totalFAssetFees_6', 'totalFAssetFees_4', 'totalFAssetFees_10'])
 totalFAssetFees += _amount
totalFAssetFees_10(uint256) = totalFAssetFees_9 (c)+ _amount_1
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
```
#### FAsset.mint(address,uint256) [EXTERNAL]
```slithir
 _mint(_owner,_amount)
INTERNAL_CALL, ERC20._mint(address,uint256)(_owner_1,_amount_1)
 onlyAssetManager()
MODIFIER_CALL, FAsset.onlyAssetManager()()
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
#### Minting.calculatePoolFeeUBA(Agent.State,CollateralReservation.Data) [INTERNAL]
```slithir
_agent_1 (-> ['TMP_4821'])(Agent.State) := phi(["agent_1 (-> ['TMP_4821'])"])
_crt_1 (-> [])(CollateralReservation.Data) := phi(['_crt_1 (-> [])'])
 storedPoolFeeShareBIPS = _crt.poolFeeShareBIPS
REF_3355(uint16) -> _crt_1 (-> []).poolFeeShareBIPS
storedPoolFeeShareBIPS_1(uint16) := REF_3355(uint16)
 _calculatePoolFeeUBA(_crt.underlyingFeeUBA,poolFeeShareBIPS)
REF_3356(uint128) -> _crt_1 (-> []).underlyingFeeUBA
TMP_4849(uint256) = INTERNAL_CALL, Minting._calculatePoolFeeUBA(uint256,uint16)(REF_3356,poolFeeShareBIPS_3)
RETURN TMP_4849
 storedPoolFeeShareBIPS > 0
TMP_4850(bool) = storedPoolFeeShareBIPS_1 > 0
CONDITION TMP_4850
 poolFeeShareBIPS = storedPoolFeeShareBIPS - 1
TMP_4851(uint16) = storedPoolFeeShareBIPS_1 (c)- 1
poolFeeShareBIPS_1(uint16) := TMP_4851(uint16)
 poolFeeShareBIPS = _agent.poolFeeShareBIPS
REF_3357(uint16) -> _agent_1 (-> ['TMP_4821']).poolFeeShareBIPS
poolFeeShareBIPS_2(uint16) := REF_3357(uint16)
poolFeeShareBIPS_3(uint16) := phi(['poolFeeShareBIPS_1', 'poolFeeShareBIPS_2'])
```
#### Minting.distributeCollateralReservationFee(Agent.State,uint256) [INTERNAL]
```slithir
 _fee == 0
TMP_4806(bool) = _fee_1 == 0
CONDITION TMP_4806
 poolFeeShare = _fee.mulBips(_agent.poolFeeShareBIPS)
REF_3317(uint16) -> _agent_1 (-> []).poolFeeShareBIPS
TMP_4807(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['_fee_1', 'REF_3317'] 
poolFeeShare_1(uint256) := TMP_4807(uint256)
 _agent.collateralPool.depositNat{value: poolFeeShare}()
REF_3318(IICollateralPool) -> _agent_1 (-> []).collateralPool
HIGH_LEVEL_CALL, dest:REF_3318(IICollateralPool), function:depositNat, arguments:[] value:poolFeeShare_1 
 Transfers.depositWNat(Globals.getWNat(),Agents.getOwnerPayAddress(_agent),_fee - poolFeeShare)
TMP_4809(IWNat) = LIBRARY_CALL, dest:Globals, function:Globals.getWNat(), arguments:[] 
TMP_4810(address) = LIBRARY_CALL, dest:Agents, function:Agents.getOwnerPayAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4811(uint256) = _fee_1 (c)- poolFeeShare_1
LIBRARY_CALL, dest:Transfers, function:Transfers.depositWNat(IWNat,address,uint256), arguments:['TMP_4809', 'TMP_4810', 'TMP_4811']
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
#### Minting.payOrBurnExecutorFee(CollateralReservation.Data) [INTERNAL]
```slithir
 executorFeeNatWei = _crt.executorFeeNatGWei * Conversion.GWEI
REF_3323(uint64) -> _crt_1 (-> []).executorFeeNatGWei
REF_3324(uint256) -> Conversion.GWEI
TMP_4813(uint64) = REF_3323 (c)* REF_3324
executorFeeNatWei_1(uint256) := TMP_4813(uint64)
 executorFeeNatWei > 0
TMP_4814(bool) = executorFeeNatWei_1 > 0
CONDITION TMP_4814
 _crt.executorFeeNatGWei = 0
REF_3325(uint64) -> _crt_1 (-> []).executorFeeNatGWei
_crt_2 (-> [])(CollateralReservation.Data) := phi(['_crt_1 (-> [])'])
REF_3325(uint64) (->_crt_2 (-> [])) := 0(uint256)
 msg.sender == _crt.executor
REF_3326(address) -> _crt_2 (-> []).executor
TMP_4815(bool) = msg.sender == REF_3326
CONDITION TMP_4815
 Transfers.depositWNat(Globals.getWNat(),_crt.executor,executorFeeNatWei)
TMP_4816(IWNat) = LIBRARY_CALL, dest:Globals, function:Globals.getWNat(), arguments:[] 
REF_3329(address) -> _crt_2 (-> []).executor
LIBRARY_CALL, dest:Transfers, function:Transfers.depositWNat(IWNat,address,uint256), arguments:['TMP_4816', 'REF_3329', 'executorFeeNatWei_1'] 
 Globals.getBurnAddress().transfer(executorFeeNatWei)
TMP_4818(address) = LIBRARY_CALL, dest:Globals, function:Globals.getBurnAddress(), arguments:[] 
Transfer dest:TMP_4818 value:executorFeeNatWei_1
```
#### Minting.releaseCollateralReservation(CollateralReservation.Data,CollateralReservation.Status) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4820(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4820'])(AssetManagerState.State) := TMP_4820(AssetManagerState.State)
 agent = Agent.get(_crt.agentVault)
REF_3334(address) -> _crt_1 (-> []).agentVault
TMP_4821(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_3334'] 
agent_1 (-> ['TMP_4821'])(Agent.State) := TMP_4821(Agent.State)
 reservationAMG = _crt.valueAMG + Conversion.convertUBAToAmg(calculatePoolFeeUBA(agent,_crt))
REF_3335(uint64) -> _crt_1 (-> []).valueAMG
TMP_4822(uint256) = INTERNAL_CALL, Minting.calculatePoolFeeUBA(Agent.State,CollateralReservation.Data)(agent_1 (-> ['TMP_4821']),_crt_1 (-> []))
TMP_4823(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['TMP_4822'] 
TMP_4824(uint64) = REF_3335 (c)+ TMP_4823
reservationAMG_1(uint64) := TMP_4824(uint64)
 agent.reservedAMG = agent.reservedAMG - reservationAMG
REF_3337(uint64) -> agent_1 (-> ['TMP_4821']).reservedAMG
REF_3338(uint64) -> agent_1 (-> ['TMP_4821']).reservedAMG
TMP_4825(uint64) = REF_3338 (c)- reservationAMG_1
agent_2 (-> ['TMP_4821'])(Agent.State) := phi(["agent_1 (-> ['TMP_4821'])"])
REF_3337(uint64) (->agent_2 (-> ['TMP_4821'])) := TMP_4825(uint64)
TMP_4821(Agent.State) := phi(["agent_2 (-> ['TMP_4821'])"])
 state.totalReservedCollateralAMG -= reservationAMG
REF_3339(uint64) -> state_1 (-> ['TMP_4820']).totalReservedCollateralAMG
state_2 (-> ['TMP_4820'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_4820'])"])
REF_3339(-> state_2 (-> ['TMP_4820'])) = REF_3339 (c)- reservationAMG_1
TMP_4820(AssetManagerState.State) := phi(["state_2 (-> ['TMP_4820'])"])
 assert(bool)(_status != CollateralReservation.Status.ACTIVE)
REF_3340(CollateralReservation.Status) -> Status.ACTIVE
TMP_4826(bool) = _status_1 != REF_3340
TMP_4827(None) = SOLIDITY_CALL assert(bool)(TMP_4826)
 _crt.status = _status
REF_3341(CollateralReservation.Status) -> _crt_1 (-> []).status
_crt_2 (-> [])(CollateralReservation.Data) := phi(['_crt_1 (-> [])'])
REF_3341(CollateralReservation.Status) (->_crt_2 (-> [])) := _status_1(CollateralReservation.Status)
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
#### PaymentConfirmations.confirmIncomingPayment(PaymentConfirmations.State,IPayment.Proof) [INTERNAL]
```slithir
 _recordPaymentVerification(_state,_payment.data.requestBody.transactionId)
REF_3753(IPayment.Response) -> _payment_1.data
REF_3754(IPayment.RequestBody) -> REF_3753.requestBody
REF_3755(bytes32) -> REF_3754.transactionId
INTERNAL_CALL, PaymentConfirmations._recordPaymentVerification(PaymentConfirmations.State,bytes32)(_state_1 (-> []),REF_3755)
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
#### AgentCollateral.freeCollateralLots(Collateral.CombinedData,Agent.State) [INTERNAL]
```slithir
 freeCollateralLotsOptionalFee(_data,_agent,true)
TMP_4322(uint256) = INTERNAL_CALL, AgentCollateral.freeCollateralLotsOptionalFee(Collateral.CombinedData,Agent.State,bool)(_data_1,_agent_1 (-> []),True)
RETURN TMP_4322
 _lots
```
#### Agents.requireAgentVaultOwner(Agent.State) [INTERNAL]
```slithir
 require(bool,error)(isOwner(_agent,msg.sender),revert OnlyAgentVaultOwner()())
TMP_4497(bool) = INTERNAL_CALL, Agents.isOwner(Agent.State,address)(_agent_1 (-> []),msg.sender)
TMP_4498(None) = SOLIDITY_CALL revert OnlyAgentVaultOwner()()
TMP_4499(None) = SOLIDITY_CALL require(bool,error)(TMP_4497,TMP_4498)
```
#### Agents.requireWhitelistedAgentVaultOwner(Agent.State) [INTERNAL]
```slithir
 requireWhitelisted(_agent.ownerManagementAddress)
REF_2996(address) -> _agent_1 (-> []).ownerManagementAddress
INTERNAL_CALL, Agents.requireWhitelisted(address)(REF_2996)
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
#### Minting.calculateCurrentPoolFeeUBA(Agent.State,uint256) [INTERNAL]
```slithir
 mintingFeeUBA = _mintingValueUBA.mulBips(_agent.feeBIPS)
REF_3359(uint16) -> _agent_1 (-> []).feeBIPS
TMP_4852(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['_mintingValueUBA_1', 'REF_3359'] 
mintingFeeUBA_1(uint256) := TMP_4852(uint256)
 _calculatePoolFeeUBA(mintingFeeUBA,_agent.poolFeeShareBIPS)
REF_3360(uint16) -> _agent_1 (-> []).poolFeeShareBIPS
TMP_4853(uint256) = INTERNAL_CALL, Minting._calculatePoolFeeUBA(uint256,uint16)(mintingFeeUBA_1,REF_3360)
RETURN TMP_4853
```
#### Minting.checkMintingCap(uint64) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4838(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4838'])(AssetManagerState.State) := TMP_4838(AssetManagerState.State)
 settings = Globals.getSettings()
TMP_4839(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4839'])(AssetManagerSettings.Data) := TMP_4839(AssetManagerSettings.Data)
 mintingCapAMG = settings.mintingCapAMG
REF_3350(uint64) -> settings_1 (-> ['TMP_4839']).mintingCapAMG
mintingCapAMG_1(uint256) := REF_3350(uint64)
 mintingCapAMG == 0
TMP_4840(bool) = mintingCapAMG_1 == 0
CONDITION TMP_4840
 totalMintedUBA = IERC20(settings.fAsset).totalSupply()
REF_3351(address) -> settings_1 (-> ['TMP_4839']).fAsset
TMP_4841 = CONVERT REF_3351 to IERC20
TMP_4842(uint256) = HIGH_LEVEL_CALL, dest:TMP_4841(IERC20), function:totalSupply, arguments:[]  
totalMintedUBA_1(uint256) := TMP_4842(uint256)
 totalAMG = state.totalReservedCollateralAMG + Conversion.convertUBAToAmg(totalMintedUBA)
REF_3353(uint64) -> state_1 (-> ['TMP_4838']).totalReservedCollateralAMG
TMP_4843(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['totalMintedUBA_1'] 
TMP_4844(uint64) = REF_3353 (c)+ TMP_4843
totalAMG_1(uint256) := TMP_4844(uint64)
 require(bool,error)(totalAMG + _increaseAMG <= mintingCapAMG,revert MintingCapExceeded()())
TMP_4845(uint256) = totalAMG_1 (c)+ _increaseAMG_1
TMP_4846(bool) = TMP_4845 <= mintingCapAMG_1
TMP_4847(None) = SOLIDITY_CALL revert MintingCapExceeded()()
TMP_4848(None) = SOLIDITY_CALL require(bool,error)(TMP_4846,TMP_4847)
```

#### PaymentReference.selfMint(address) [INTERNAL]
```slithir
SELF_MINT_1(uint256) := phi(['SELF_MINT_0'])
 bytes32(uint256(uint160(_agentVault)) | SELF_MINT)
TMP_5369 = CONVERT _agentVault_1 to uint160
TMP_5370 = CONVERT TMP_5369 to uint256
TMP_5371(uint256) = TMP_5370 | SELF_MINT_1
TMP_5372 = CONVERT TMP_5371 to bytes32
RETURN TMP_5372
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
#### Agents.getWorkAddress(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
 Globals.getAgentOwnerRegistry().getWorkAddress(_agent.ownerManagementAddress)
TMP_4481(IAgentOwnerRegistry) = LIBRARY_CALL, dest:Globals, function:Globals.getAgentOwnerRegistry(), arguments:[] 
REF_2992(address) -> _agent_1 (-> []).ownerManagementAddress
TMP_4482(address) = HIGH_LEVEL_CALL, dest:TMP_4481(IAgentOwnerRegistry), function:getWorkAddress, arguments:['REF_2992']  
RETURN TMP_4482
```

#### Agents.getOwnerPayAddress(Agent.State) [INTERNAL]
```slithir
 workAddress = getWorkAddress(_agent)
TMP_4483(address) = INTERNAL_CALL, Agents.getWorkAddress(Agent.State)(_agent_1 (-> []))
workAddress_1(address) := TMP_4483(address)
 workAddress != address(0)
TMP_4484 = CONVERT 0 to address
TMP_4485(bool) = workAddress_1 != TMP_4484
CONDITION TMP_4485
 address(workAddress)
TMP_4486 = CONVERT workAddress_1 to address
RETURN TMP_4486
 address(_agent.ownerManagementAddress)
REF_2993(address) -> _agent_1 (-> []).ownerManagementAddress
TMP_4487 = CONVERT REF_2993 to address
RETURN TMP_4487
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
#### CollateralPool.depositNat() [EXTERNAL]
```slithir
 _depositWNat()
INTERNAL_CALL, CollateralPool._depositWNat()()
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
```
#### Transfers.depositWNat(IWNat,address,uint256) [INTERNAL]
```slithir
 _amount > 0
TMP_10540(bool) = _amount_1 > 0
CONDITION TMP_10540
 _wNat.depositTo{value: _amount}(_recipient)
HIGH_LEVEL_CALL, dest:_wNat_1(IWNat), function:depositTo, arguments:['_recipient_1'] value:_amount_1
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
#### AgentCollateral.agentsPoolTokensCollateralData(Agent.State,Collateral.Data) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
_poolCollateral_1(Collateral.Data) := phi(['poolCollateral_1', 'TMP_4304'])
 poolToken = _agent.collateralPool.poolToken()
REF_2830(IICollateralPool) -> _agent_1 (-> []).collateralPool
TMP_4315(ICollateralPoolToken) = HIGH_LEVEL_CALL, dest:REF_2830(IICollateralPool), function:poolToken, arguments:[]  
poolToken_1(IERC20) := TMP_4315(ICollateralPoolToken)
 agentPoolTokens = poolToken.balanceOf(_agent.vaultAddress())
TMP_4316(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4317(uint256) = HIGH_LEVEL_CALL, dest:poolToken_1(IERC20), function:balanceOf, arguments:['TMP_4316']  
agentPoolTokens_1(uint256) := TMP_4317(uint256)
 totalPoolTokens = poolToken.totalSupply()
TMP_4318(uint256) = HIGH_LEVEL_CALL, dest:poolToken_1(IERC20), function:totalSupply, arguments:[]  
totalPoolTokens_1(uint256) := TMP_4318(uint256)
 Collateral.Data({kind:Collateral.Kind.AGENT_POOL,fullCollateral:agentPoolTokens,amgToTokenWeiPrice:amgToPoolTokenWeiPrice})
REF_2836(Collateral.Kind) -> Kind.AGENT_POOL
TMP_4319(Collateral.Data) = new Data(REF_2836,agentPoolTokens_1,amgToPoolTokenWeiPrice_3)
RETURN TMP_4319
 _poolCollateral.fullCollateral != 0
REF_2837(uint256) -> _poolCollateral_1.fullCollateral
TMP_4320(bool) = REF_2837 != 0
CONDITION TMP_4320
 amgToPoolTokenWeiPrice = _poolCollateral.amgToTokenWeiPrice.mulDiv(totalPoolTokens,_poolCollateral.fullCollateral)
REF_2838(uint256) -> _poolCollateral_1.amgToTokenWeiPrice
REF_2840(uint256) -> _poolCollateral_1.fullCollateral
TMP_4321(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['REF_2838', 'totalPoolTokens_1', 'REF_2840'] 
amgToPoolTokenWeiPrice_1(uint256) := TMP_4321(uint256)
 amgToPoolTokenWeiPrice = _poolCollateral.amgToTokenWeiPrice
REF_2841(uint256) -> _poolCollateral_1.amgToTokenWeiPrice
amgToPoolTokenWeiPrice_2(uint256) := REF_2841(uint256)
amgToPoolTokenWeiPrice_3(uint256) := phi(['amgToPoolTokenWeiPrice_1', 'amgToPoolTokenWeiPrice_2'])
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
#### AgentCollateral.freeCollateralLotsOptionalFee(Collateral.CombinedData,Agent.State,bool) [INTERNAL]
```slithir
_data_1(Collateral.CombinedData) := phi(['_data_1'])
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 agentLots = freeSingleCollateralLots(_data.agentCollateral,_agent,_chargePoolFee)
REF_2842(Collateral.Data) -> _data_1.agentCollateral
TMP_4323(uint256) = INTERNAL_CALL, AgentCollateral.freeSingleCollateralLots(Collateral.Data,Agent.State,bool)(REF_2842,_agent_1 (-> []),_chargePoolFee_1)
agentLots_1(uint256) := TMP_4323(uint256)
 poolLots = freeSingleCollateralLots(_data.poolCollateral,_agent,_chargePoolFee)
REF_2843(Collateral.Data) -> _data_1.poolCollateral
TMP_4324(uint256) = INTERNAL_CALL, AgentCollateral.freeSingleCollateralLots(Collateral.Data,Agent.State,bool)(REF_2843,_agent_1 (-> []),_chargePoolFee_1)
poolLots_1(uint256) := TMP_4324(uint256)
 agentPoolTokenLots = freeSingleCollateralLots(_data.agentPoolTokens,_agent,_chargePoolFee)
REF_2844(Collateral.Data) -> _data_1.agentPoolTokens
TMP_4325(uint256) = INTERNAL_CALL, AgentCollateral.freeSingleCollateralLots(Collateral.Data,Agent.State,bool)(REF_2844,_agent_1 (-> []),_chargePoolFee_1)
agentPoolTokenLots_1(uint256) := TMP_4325(uint256)
 Math.min(agentLots,Math.min(poolLots,agentPoolTokenLots))
TMP_4326(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['poolLots_1', 'agentPoolTokenLots_1'] 
TMP_4327(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['agentLots_1', 'TMP_4326'] 
RETURN TMP_4327
 _lots
```
#### Agents.requireWhitelisted(address) [INTERNAL]
```slithir
_ownerManagementAddress_1(address) := phi(['REF_2996'])
 require(bool,error)(Globals.getAgentOwnerRegistry().isWhitelisted(_ownerManagementAddress),revert AgentNotWhitelisted()())
TMP_4488(IAgentOwnerRegistry) = LIBRARY_CALL, dest:Globals, function:Globals.getAgentOwnerRegistry(), arguments:[] 
TMP_4489(bool) = HIGH_LEVEL_CALL, dest:TMP_4488(IAgentOwnerRegistry), function:isWhitelisted, arguments:['_ownerManagementAddress_1']  
TMP_4490(None) = SOLIDITY_CALL revert AgentNotWhitelisted()()
TMP_4491(None) = SOLIDITY_CALL require(bool,error)(TMP_4489,TMP_4490)
```
#### ERC20.totalSupply() [PUBLIC]
```slithir
_totalSupply_1(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 _totalSupply
RETURN _totalSupply_1
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
#### CollateralPool._depositWNat() [INTERNAL]
```slithir
agentVault_29(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_28(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_14', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_0', 'assetManager_30'])
wNat_7(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_36(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 msg.value > 0
TMP_6312(bool) = msg.value > 0
CONDITION TMP_6312
 totalCollateral += msg.value
totalCollateral_37(uint256) = totalCollateral_36 (c)+ msg.value
 wNat.deposit{value: msg.value}()
HIGH_LEVEL_CALL, dest:wNat_7(IWNat), function:deposit, arguments:[] value:msg.value 
agentVault_30(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_29', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_29(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_28', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_8(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_7', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
 assetManager.updateCollateral(agentVault,wNat)
HIGH_LEVEL_CALL, dest:assetManager_29(IIAssetManager), function:updateCollateral, arguments:['agentVault_30', 'wNat_8']  
agentVault_31(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_30', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_30(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_29', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_9(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_8', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
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
#### WNatMock.depositTo(address) [PUBLIC]
```slithir
 _mint(_recipient,msg.value)
INTERNAL_CALL, ERC20._mint(address,uint256)(_recipient_1,msg.value)
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
#### Agents.getPoolCollateral(Agent.State) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4516(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4516'])(AssetManagerState.State) := TMP_4516(AssetManagerState.State)
 state.collateralTokens[_agent.poolCollateralIndex]
REF_3015(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4516']).collateralTokens
REF_3016(uint16) -> _agent_1 (-> []).poolCollateralIndex
REF_3017(CollateralTypeInt.Data) -> REF_3015[REF_3016]
RETURN REF_3017
```
#### AgentCollateral.freeSingleCollateralLots(Collateral.Data,Agent.State,bool) [INTERNAL]
```slithir
_data_1(Collateral.Data) := phi(['REF_2842', 'REF_2844', 'REF_2843'])
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_chargePoolFee_1(bool) := phi(['_chargePoolFee_1'])
 collateralWei = freeCollateralWei(_data,_agent)
TMP_4328(uint256) = INTERNAL_CALL, AgentCollateral.freeCollateralWei(Collateral.Data,Agent.State)(_data_1,_agent_1 (-> []))
collateralWei_1(uint256) := TMP_4328(uint256)
 lotWei = mintingLotCollateralWei(_data,_agent,_chargePoolFee)
TMP_4329(uint256) = INTERNAL_CALL, AgentCollateral.mintingLotCollateralWei(Collateral.Data,Agent.State,bool)(_data_1,_agent_1 (-> []),_chargePoolFee_1)
lotWei_1(uint256) := TMP_4329(uint256)
 lotWei != 0
TMP_4330(bool) = lotWei_1 != 0
CONDITION TMP_4330
 collateralWei / lotWei
TMP_4331(uint256) = collateralWei_1 (c)/ lotWei_1
RETURN TMP_4331
 0
RETURN 0
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
#### AgentOwnerRegistry.isWhitelisted(address) [PUBLIC]
```slithir
_address_1(address) := phi(['msg.sender'])
whitelist_1(mapping(address => bool)) := phi(['whitelist_2', 'whitelist_1', 'whitelist_4', 'whitelist_5', 'whitelist_0', 'whitelist_3'])
 whitelist[_address]
REF_313(bool) -> whitelist_1[_address_1]
RETURN REF_313
```
