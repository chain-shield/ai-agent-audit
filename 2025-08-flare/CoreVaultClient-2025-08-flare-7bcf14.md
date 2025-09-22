












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
#### CoreVaultClient._minimumRemainingAfterTransferAMG(Agent.State) [PRIVATE]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 cd = AgentCollateral.combinedData(_agent)
TMP_4719(Collateral.CombinedData) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.combinedData(Agent.State), arguments:['_agent_1 (-> [])'] 
cd_1(Collateral.CombinedData) := TMP_4719(Collateral.CombinedData)
 resultWRTVault = _minimumRemainingAfterTransferForCollateralAMG(_agent,cd.agentCollateral)
REF_3216(Collateral.Data) -> cd_1.agentCollateral
TMP_4720(uint256) = INTERNAL_CALL, CoreVaultClient._minimumRemainingAfterTransferForCollateralAMG(Agent.State,Collateral.Data)(_agent_1 (-> []),REF_3216)
resultWRTVault_1(uint256) := TMP_4720(uint256)
 resultWRTPool = _minimumRemainingAfterTransferForCollateralAMG(_agent,cd.poolCollateral)
REF_3217(Collateral.Data) -> cd_1.poolCollateral
TMP_4721(uint256) = INTERNAL_CALL, CoreVaultClient._minimumRemainingAfterTransferForCollateralAMG(Agent.State,Collateral.Data)(_agent_1 (-> []),REF_3217)
resultWRTPool_1(uint256) := TMP_4721(uint256)
 resultWRTAgentPT = _minimumRemainingAfterTransferForCollateralAMG(_agent,cd.agentPoolTokens)
REF_3218(Collateral.Data) -> cd_1.agentPoolTokens
TMP_4722(uint256) = INTERNAL_CALL, CoreVaultClient._minimumRemainingAfterTransferForCollateralAMG(Agent.State,Collateral.Data)(_agent_1 (-> []),REF_3218)
resultWRTAgentPT_1(uint256) := TMP_4722(uint256)
 Math.min(resultWRTVault,Math.min(resultWRTPool,resultWRTAgentPT))
TMP_4723(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['resultWRTPool_1', 'resultWRTAgentPT_1'] 
TMP_4724(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['resultWRTVault_1', 'TMP_4723'] 
RETURN TMP_4724
```
#### CoreVaultClient._minimumRemainingAfterTransferForCollateralAMG(Agent.State,Collateral.Data) [PRIVATE]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_data_1(Collateral.Data) := phi(['REF_3216', 'REF_3218', 'REF_3217'])
 state = getState()
TMP_4725(CoreVaultClient.State) = INTERNAL_CALL, CoreVaultClient.getState()()
state_1 (-> ['TMP_4725'])(CoreVaultClient.State) := TMP_4725(CoreVaultClient.State)
 (None,systemMinCrBIPS) = AgentCollateral.mintingMinCollateralRatio(_agent,_data.kind)
REF_3222(Collateral.Kind) -> _data_1.kind
TUPLE_54(uint256,uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.mintingMinCollateralRatio(Agent.State,Collateral.Kind), arguments:['_agent_1 (-> [])', 'REF_3222'] 
systemMinCrBIPS_1(uint256)= UNPACK TUPLE_54 index: 1 
 collateralEquivAMG = Conversion.convertTokenWeiToAMG(_data.fullCollateral,_data.amgToTokenWeiPrice)
REF_3224(uint256) -> _data_1.fullCollateral
REF_3225(uint256) -> _data_1.amgToTokenWeiPrice
TMP_4726(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertTokenWeiToAMG(uint256,uint256), arguments:['REF_3224', 'REF_3225'] 
collateralEquivAMG_1(uint256) := TMP_4726(uint256)
 maxSupportedAMG = collateralEquivAMG.mulDiv(SafePct.MAX_BIPS,systemMinCrBIPS)
REF_3227(uint256) -> SafePct.MAX_BIPS
TMP_4727(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['collateralEquivAMG_1', 'REF_3227', 'systemMinCrBIPS_1'] 
maxSupportedAMG_1(uint256) := TMP_4727(uint256)
 maxSupportedAMG.mulBips(state.minimumAmountLeftBIPS)
REF_3229(uint16) -> state_1 (-> ['TMP_4725']).minimumAmountLeftBIPS
TMP_4728(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['maxSupportedAMG_1', 'REF_3229'] 
RETURN TMP_4728
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
#### CoreVaultClient.checkEnabled() [INTERNAL]
```slithir
 state = getState()
TMP_4707(CoreVaultClient.State) = INTERNAL_CALL, CoreVaultClient.getState()()
state_1 (-> ['TMP_4707'])(CoreVaultClient.State) := TMP_4707(CoreVaultClient.State)
 require(bool,error)(address(state.coreVaultManager) != address(0),revert CoreVaultNotEnabled()())
REF_3211(IICoreVaultManager) -> state_1 (-> ['TMP_4707']).coreVaultManager
TMP_4708 = CONVERT REF_3211 to address
TMP_4709 = CONVERT 0 to address
TMP_4710(bool) = TMP_4708 != TMP_4709
TMP_4711(None) = SOLIDITY_CALL revert CoreVaultNotEnabled()()
TMP_4712(None) = SOLIDITY_CALL require(bool,error)(TMP_4710,TMP_4711)
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
#### CoreVaultClient.coreVaultAmountLots() [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4703(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4703'])(AssetManagerSettings.Data) := TMP_4703(AssetManagerSettings.Data)
 (None,totalAmountUBA) = coreVaultAvailableAmount()
TUPLE_52(uint256,uint256) = INTERNAL_CALL, CoreVaultClient.coreVaultAvailableAmount()()
totalAmountUBA_1(uint256)= UNPACK TUPLE_52 index: 1 
 Conversion.convertUBAToAmg(totalAmountUBA) / settings.lotSizeAMG
TMP_4704(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['totalAmountUBA_1'] 
REF_3208(uint64) -> settings_1 (-> ['TMP_4703']).lotSizeAMG
TMP_4705(uint64) = TMP_4704 (c)/ REF_3208
RETURN TMP_4705
```
#### CoreVaultClient.coreVaultAvailableAmount() [INTERNAL]
```slithir
 state = getState()
TMP_4694(CoreVaultClient.State) = INTERNAL_CALL, CoreVaultClient.getState()()
state_1 (-> ['TMP_4694'])(CoreVaultClient.State) := TMP_4694(CoreVaultClient.State)
 availableFunds = state.coreVaultManager.availableFunds()
REF_3198(IICoreVaultManager) -> state_1 (-> ['TMP_4694']).coreVaultManager
TMP_4695(uint128) = HIGH_LEVEL_CALL, dest:REF_3198(IICoreVaultManager), function:availableFunds, arguments:[]  
availableFunds_1(uint256) := TMP_4695(uint128)
 escrowedFunds = state.coreVaultManager.escrowedFunds()
REF_3200(IICoreVaultManager) -> state_1 (-> ['TMP_4694']).coreVaultManager
TMP_4696(uint128) = HIGH_LEVEL_CALL, dest:REF_3200(IICoreVaultManager), function:escrowedFunds, arguments:[]  
escrowedFunds_1(uint256) := TMP_4696(uint128)
 requestedAmountWithFee = state.coreVaultManager.totalRequestAmountWithFee() + coreVaultUnderlyingPaymentFee()
REF_3202(IICoreVaultManager) -> state_1 (-> ['TMP_4694']).coreVaultManager
TMP_4697(uint256) = HIGH_LEVEL_CALL, dest:REF_3202(IICoreVaultManager), function:totalRequestAmountWithFee, arguments:[]  
TMP_4698(uint256) = INTERNAL_CALL, CoreVaultClient.coreVaultUnderlyingPaymentFee()()
TMP_4699(uint256) = TMP_4697 (c)+ TMP_4698
requestedAmountWithFee_1(uint256) := TMP_4699(uint256)
 _immediatelyAvailableUBA = MathUtils.subOrZero(availableFunds,requestedAmountWithFee)
TMP_4700(uint256) = LIBRARY_CALL, dest:MathUtils, function:MathUtils.subOrZero(uint256,uint256), arguments:['availableFunds_1', 'requestedAmountWithFee_1'] 
_immediatelyAvailableUBA_1(uint256) := TMP_4700(uint256)
 _totalAvailableUBA = MathUtils.subOrZero(availableFunds + escrowedFunds,requestedAmountWithFee)
TMP_4701(uint256) = availableFunds_1 (c)+ escrowedFunds_1
TMP_4702(uint256) = LIBRARY_CALL, dest:MathUtils, function:MathUtils.subOrZero(uint256,uint256), arguments:['TMP_4701', 'requestedAmountWithFee_1'] 
_totalAvailableUBA_1(uint256) := TMP_4702(uint256)
 (_immediatelyAvailableUBA,_totalAvailableUBA)
RETURN _immediatelyAvailableUBA_1,_totalAvailableUBA_1
```
#### CoreVaultClient.coreVaultUnderlyingAddressHash() [INTERNAL]
```slithir
 state = getState()
TMP_4713(CoreVaultClient.State) = INTERNAL_CALL, CoreVaultClient.getState()()
state_1 (-> ['TMP_4713'])(CoreVaultClient.State) := TMP_4713(CoreVaultClient.State)
 address(state.coreVaultManager) == address(0)
REF_3212(IICoreVaultManager) -> state_1 (-> ['TMP_4713']).coreVaultManager
TMP_4714 = CONVERT REF_3212 to address
TMP_4715 = CONVERT 0 to address
TMP_4716(bool) = TMP_4714 == TMP_4715
CONDITION TMP_4716
 bytes32(0)
TMP_4717 = CONVERT 0 to bytes32
RETURN TMP_4717
 state.coreVaultManager.coreVaultAddressHash()
REF_3213(IICoreVaultManager) -> state_1 (-> ['TMP_4713']).coreVaultManager
TMP_4718(bytes32) = HIGH_LEVEL_CALL, dest:REF_3213(IICoreVaultManager), function:coreVaultAddressHash, arguments:[]  
RETURN TMP_4718
```
#### CoreVaultClient.coreVaultUnderlyingPaymentFee() [INTERNAL]
```slithir
 state = getState()
TMP_4706(CoreVaultClient.State) = INTERNAL_CALL, CoreVaultClient.getState()()
state_1 (-> ['TMP_4706'])(CoreVaultClient.State) := TMP_4706(CoreVaultClient.State)
 (None,None,None,fee) = state.coreVaultManager.getSettings()
REF_3209(IICoreVaultManager) -> state_1 (-> ['TMP_4706']).coreVaultManager
TUPLE_53(uint128,uint128,uint128,uint128) = HIGH_LEVEL_CALL, dest:REF_3209(IICoreVaultManager), function:getSettings, arguments:[]  
fee_1(uint256)= UNPACK TUPLE_53 index: 3 
 fee
RETURN fee_1
```
#### CoreVaultClient.deleteReturnFromCoreVaultRequest(Agent.State) [INTERNAL]
```slithir
 assert(bool)(_agent.activeReturnFromCoreVaultId != 0 && _agent.returnFromCoreVaultReservedAMG != 0)
REF_3190(uint64) -> _agent_1 (-> []).activeReturnFromCoreVaultId
TMP_4688(bool) = REF_3190 != 0
REF_3191(uint64) -> _agent_1 (-> []).returnFromCoreVaultReservedAMG
TMP_4689(bool) = REF_3191 != 0
TMP_4690(bool) = TMP_4688 && TMP_4689
TMP_4691(None) = SOLIDITY_CALL assert(bool)(TMP_4690)
 _agent.reservedAMG -= _agent.returnFromCoreVaultReservedAMG
REF_3192(uint64) -> _agent_1 (-> []).reservedAMG
REF_3193(uint64) -> _agent_1 (-> []).returnFromCoreVaultReservedAMG
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_3192(-> _agent_2 (-> [])) = REF_3192 (c)- REF_3193
 _agent.activeReturnFromCoreVaultId = 0
REF_3194(uint64) -> _agent_2 (-> []).activeReturnFromCoreVaultId
_agent_3 (-> [])(Agent.State) := phi(['_agent_2 (-> [])'])
REF_3194(uint64) (->_agent_3 (-> [])) := 0(uint256)
 _agent.returnFromCoreVaultReservedAMG = 0
REF_3195(uint64) -> _agent_3 (-> []).returnFromCoreVaultReservedAMG
_agent_4 (-> [])(Agent.State) := phi(['_agent_3 (-> [])'])
REF_3195(uint64) (->_agent_4 (-> [])) := 0(uint256)
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
#### CoreVaultClient.maximumTransferToCoreVaultAMG(Agent.State) [INTERNAL]
```slithir
 _minimumLeftAmountAMG = _minimumRemainingAfterTransferAMG(_agent)
TMP_4692(uint256) = INTERNAL_CALL, CoreVaultClient._minimumRemainingAfterTransferAMG(Agent.State)(_agent_1 (-> []))
_minimumLeftAmountAMG_1(uint256) := TMP_4692(uint256)
 _maximumTransferAMG = MathUtils.subOrZero(_agent.mintedAMG,_minimumLeftAmountAMG)
REF_3197(uint64) -> _agent_1 (-> []).mintedAMG
TMP_4693(uint256) = LIBRARY_CALL, dest:MathUtils, function:MathUtils.subOrZero(uint256,uint256), arguments:['REF_3197', '_minimumLeftAmountAMG_1'] 
_maximumTransferAMG_1(uint256) := TMP_4693(uint256)
 (_maximumTransferAMG,_minimumLeftAmountAMG)
RETURN _maximumTransferAMG_1,_minimumLeftAmountAMG_1
```
#### Conversion.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 AMG_TOKEN_WEI_PRICE_SCALE_EXP = 9
 AMG_TOKEN_WEI_PRICE_SCALE = 10 ** AMG_TOKEN_WEI_PRICE_SCALE_EXP
 NAT_WEI = 1e18
 GWEI = 1e9
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
#### AgentCollateral.mintingMinCollateralRatio(Agent.State,Collateral.Kind) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
_kind_1(Collateral.Kind) := phi(['REF_2871', 'REF_2849'])
 _kind == Collateral.Kind.AGENT_POOL
REF_2877(Collateral.Kind) -> Kind.AGENT_POOL
TMP_4353(bool) = _kind_1 == REF_2877
CONDITION TMP_4353
 mintingPoolHoldingsRequiredBIPS = Globals.getSettings().mintingPoolHoldingsRequiredBIPS
TMP_4354(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_2879(uint32) -> TMP_4354.mintingPoolHoldingsRequiredBIPS
mintingPoolHoldingsRequiredBIPS_1(uint256) := REF_2879(uint32)
 _systemMinCollateralRatioBIPS = mintingPoolHoldingsRequiredBIPS
_systemMinCollateralRatioBIPS_4(uint256) := mintingPoolHoldingsRequiredBIPS_1(uint256)
 _mintingMinCollateralRatioBIPS = mintingPoolHoldingsRequiredBIPS
_mintingMinCollateralRatioBIPS_4(uint256) := mintingPoolHoldingsRequiredBIPS_1(uint256)
 _kind == Collateral.Kind.POOL
REF_2880(Collateral.Kind) -> Kind.POOL
TMP_4355(bool) = _kind_1 == REF_2880
CONDITION TMP_4355
 _systemMinCollateralRatioBIPS = _agent.getPoolCollateral().minCollateralRatioBIPS
TMP_4356(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getPoolCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
REF_2882(uint32) -> TMP_4356.minCollateralRatioBIPS
_systemMinCollateralRatioBIPS_2(uint256) := REF_2882(uint32)
 _mintingMinCollateralRatioBIPS = Math.max(_agent.mintingPoolCollateralRatioBIPS,_systemMinCollateralRatioBIPS)
REF_2884(uint32) -> _agent_1 (-> []).mintingPoolCollateralRatioBIPS
TMP_4357(uint256) = LIBRARY_CALL, dest:Math, function:Math.max(uint256,uint256), arguments:['REF_2884', '_systemMinCollateralRatioBIPS_2'] 
_mintingMinCollateralRatioBIPS_2(uint256) := TMP_4357(uint256)
 _systemMinCollateralRatioBIPS = _agent.getVaultCollateral().minCollateralRatioBIPS
TMP_4358(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
REF_2886(uint32) -> TMP_4358.minCollateralRatioBIPS
_systemMinCollateralRatioBIPS_1(uint256) := REF_2886(uint32)
 _mintingMinCollateralRatioBIPS = Math.max(_agent.mintingVaultCollateralRatioBIPS,_systemMinCollateralRatioBIPS)
REF_2888(uint32) -> _agent_1 (-> []).mintingVaultCollateralRatioBIPS
TMP_4359(uint256) = LIBRARY_CALL, dest:Math, function:Math.max(uint256,uint256), arguments:['REF_2888', '_systemMinCollateralRatioBIPS_1'] 
_mintingMinCollateralRatioBIPS_1(uint256) := TMP_4359(uint256)
_mintingMinCollateralRatioBIPS_3(uint256) := phi(['_mintingMinCollateralRatioBIPS_2', '_mintingMinCollateralRatioBIPS_1'])
_systemMinCollateralRatioBIPS_3(uint256) := phi(['_systemMinCollateralRatioBIPS_1', '_systemMinCollateralRatioBIPS_2'])
_mintingMinCollateralRatioBIPS_5(uint256) := phi(['_mintingMinCollateralRatioBIPS_0', '_mintingMinCollateralRatioBIPS_4'])
_systemMinCollateralRatioBIPS_5(uint256) := phi(['_systemMinCollateralRatioBIPS_0', '_systemMinCollateralRatioBIPS_4'])
 (_mintingMinCollateralRatioBIPS,_systemMinCollateralRatioBIPS)
RETURN _mintingMinCollateralRatioBIPS_5,_systemMinCollateralRatioBIPS_5
```
#### Conversion.convertTokenWeiToAMG(uint256,uint256) [INTERNAL]
```slithir
AMG_TOKEN_WEI_PRICE_SCALE_2(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_0'])
 _valueNATWei.mulDiv(AMG_TOKEN_WEI_PRICE_SCALE,_amgToTokenWeiPrice)
TMP_4675(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_valueNATWei_1', 'AMG_TOKEN_WEI_PRICE_SCALE_2', '_amgToTokenWeiPrice_1'] 
RETURN TMP_4675
```
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
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
#### SafeCast.toUint256(int256) [INTERNAL]
```slithir
 require(bool,string)(value >= 0,SafeCast: value must be positive)
TMP_786(bool) = value_1 >= 0
TMP_787(None) = SOLIDITY_CALL require(bool,string)(TMP_786,SafeCast: value must be positive)
 uint256(value)
TMP_788 = CONVERT value_1 to uint256
RETURN TMP_788
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
#### MathUtils.subOrZero(uint256,uint256) [INTERNAL]
```slithir
 _a > _b
TMP_10425(bool) = _a_1 > _b_1
CONDITION TMP_10425
 _a - _b
TMP_10426(uint256) = _a_1 (c)- _b_1
RETURN TMP_10426
 0
RETURN 0
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
#### Math.max(uint256,uint256) [INTERNAL]
```slithir
 a > b
TMP_476(bool) = a_1 > b_1
CONDITION TMP_476
 a
RETURN a_1
 b
RETURN b_1
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
#### ERC20.totalSupply() [PUBLIC]
```slithir
_totalSupply_1(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 _totalSupply
RETURN _totalSupply_1
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
