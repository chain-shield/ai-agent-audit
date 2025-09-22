







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
#### AgentUpdates.setBuyFAssetByAgentFactorBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 require(bool,error)(_buyFAssetByAgentFactorBIPS <= SafePct.MAX_BIPS,revert ValueTooHigh()())
REF_2961(uint256) -> SafePct.MAX_BIPS
TMP_4436(bool) = _buyFAssetByAgentFactorBIPS_1 <= REF_2961
TMP_4437(None) = SOLIDITY_CALL revert ValueTooHigh()()
TMP_4438(None) = SOLIDITY_CALL require(bool,error)(TMP_4436,TMP_4437)
 require(bool,error)(_buyFAssetByAgentFactorBIPS >= 9000,revert ValueTooLow()())
TMP_4439(bool) = _buyFAssetByAgentFactorBIPS_1 >= 9000
TMP_4440(None) = SOLIDITY_CALL revert ValueTooLow()()
TMP_4441(None) = SOLIDITY_CALL require(bool,error)(TMP_4439,TMP_4440)
 _agent.buyFAssetByAgentFactorBIPS = _buyFAssetByAgentFactorBIPS.toUint16()
REF_2962(uint16) -> _agent_1 (-> []).buyFAssetByAgentFactorBIPS
TMP_4442(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_buyFAssetByAgentFactorBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2962(uint16) (->_agent_2 (-> [])) := TMP_4442(uint16)
```
#### AgentUpdates.setFeeBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 require(bool,error)(_feeBIPS <= SafePct.MAX_BIPS,revert FeeTooHigh()())
REF_2952(uint256) -> SafePct.MAX_BIPS
TMP_4424(bool) = _feeBIPS_1 <= REF_2952
TMP_4425(None) = SOLIDITY_CALL revert FeeTooHigh()()
TMP_4426(None) = SOLIDITY_CALL require(bool,error)(TMP_4424,TMP_4425)
 _agent.feeBIPS = _feeBIPS.toUint16()
REF_2953(uint16) -> _agent_1 (-> []).feeBIPS
TMP_4427(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_feeBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2953(uint16) (->_agent_2 (-> [])) := TMP_4427(uint16)
```
#### AgentUpdates.setMintingPoolCollateralRatioBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 collateral = Agents.getPoolCollateral(_agent)
TMP_4419(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getPoolCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4419'])(CollateralTypeInt.Data) := TMP_4419(CollateralTypeInt.Data)
 require(bool,error)(_mintingPoolCollateralRatioBIPS >= collateral.minCollateralRatioBIPS,revert CollateralRatioTooSmall()())
REF_2949(uint32) -> collateral_1 (-> ['TMP_4419']).minCollateralRatioBIPS
TMP_4420(bool) = _mintingPoolCollateralRatioBIPS_1 >= REF_2949
TMP_4421(None) = SOLIDITY_CALL revert CollateralRatioTooSmall()()
TMP_4422(None) = SOLIDITY_CALL require(bool,error)(TMP_4420,TMP_4421)
 _agent.mintingPoolCollateralRatioBIPS = _mintingPoolCollateralRatioBIPS.toUint32()
REF_2950(uint32) -> _agent_1 (-> []).mintingPoolCollateralRatioBIPS
TMP_4423(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_mintingPoolCollateralRatioBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2950(uint32) (->_agent_2 (-> [])) := TMP_4423(uint32)
```
#### AgentUpdates.setMintingVaultCollateralRatioBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 collateral = Agents.getVaultCollateral(_agent)
TMP_4414(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4414'])(CollateralTypeInt.Data) := TMP_4414(CollateralTypeInt.Data)
 require(bool,error)(_mintingVaultCollateralRatioBIPS >= collateral.minCollateralRatioBIPS,revert CollateralRatioTooSmall()())
REF_2945(uint32) -> collateral_1 (-> ['TMP_4414']).minCollateralRatioBIPS
TMP_4415(bool) = _mintingVaultCollateralRatioBIPS_1 >= REF_2945
TMP_4416(None) = SOLIDITY_CALL revert CollateralRatioTooSmall()()
TMP_4417(None) = SOLIDITY_CALL require(bool,error)(TMP_4415,TMP_4416)
 _agent.mintingVaultCollateralRatioBIPS = _mintingVaultCollateralRatioBIPS.toUint32()
REF_2946(uint32) -> _agent_1 (-> []).mintingVaultCollateralRatioBIPS
TMP_4418(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_mintingVaultCollateralRatioBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2946(uint32) (->_agent_2 (-> [])) := TMP_4418(uint32)
```

#### AgentUpdates.setPoolFeeShareBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 require(bool,error)(_poolFeeShareBIPS <= SafePct.MAX_BIPS,revert ValueTooHigh()())
REF_2955(uint256) -> SafePct.MAX_BIPS
TMP_4428(bool) = _poolFeeShareBIPS_1 <= REF_2955
TMP_4429(None) = SOLIDITY_CALL revert ValueTooHigh()()
TMP_4430(None) = SOLIDITY_CALL require(bool,error)(TMP_4428,TMP_4429)
 _agent.poolFeeShareBIPS = _poolFeeShareBIPS.toUint16()
REF_2956(uint16) -> _agent_1 (-> []).poolFeeShareBIPS
TMP_4431(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_poolFeeShareBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2956(uint16) (->_agent_2 (-> [])) := TMP_4431(uint16)
```
#### AgentUpdates.setRedemptionPoolFeeShareBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 require(bool,error)(_redemptionPoolFeeShareBIPS <= SafePct.MAX_BIPS,revert ValueTooHigh()())
REF_2958(uint256) -> SafePct.MAX_BIPS
TMP_4432(bool) = _redemptionPoolFeeShareBIPS_1 <= REF_2958
TMP_4433(None) = SOLIDITY_CALL revert ValueTooHigh()()
TMP_4434(None) = SOLIDITY_CALL require(bool,error)(TMP_4432,TMP_4433)
 _agent.redemptionPoolFeeShareBIPS = _redemptionPoolFeeShareBIPS.toUint16()
REF_2959(uint16) -> _agent_1 (-> []).redemptionPoolFeeShareBIPS
TMP_4435(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_redemptionPoolFeeShareBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2959(uint16) (->_agent_2 (-> [])) := TMP_4435(uint16)
```
#### AgentUpdates.setVaultCollateral(Agent.State,IERC20) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4401(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4401'])(AssetManagerState.State) := TMP_4401(AssetManagerState.State)
 tokenIndex = CollateralTypes.getIndex(CollateralType.Class.VAULT,_token)
REF_2933(CollateralType.Class) -> Class.VAULT
TMP_4402(uint256) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.getIndex(CollateralType.Class,IERC20), arguments:['REF_2933', '_token_1'] 
tokenIndex_1(uint256) := TMP_4402(uint256)
 collateral = state.collateralTokens[tokenIndex]
REF_2934(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4401']).collateralTokens
REF_2935(CollateralTypeInt.Data) -> REF_2934[tokenIndex_1]
collateral_1 (-> ['state'])(CollateralTypeInt.Data) := REF_2935(CollateralTypeInt.Data)
 assert(bool)(collateral.collateralClass == CollateralType.Class.VAULT)
REF_2936(CollateralType.Class) -> collateral_1 (-> ['state']).collateralClass
REF_2937(CollateralType.Class) -> Class.VAULT
TMP_4403(bool) = REF_2936 == REF_2937
TMP_4404(None) = SOLIDITY_CALL assert(bool)(TMP_4403)
 require(bool,error)(collateral.validUntil == 0,revert CollateralDeprecated()())
REF_2938(uint64) -> collateral_1 (-> ['state']).validUntil
TMP_4405(bool) = REF_2938 == 0
TMP_4406(None) = SOLIDITY_CALL revert CollateralDeprecated()()
TMP_4407(None) = SOLIDITY_CALL require(bool,error)(TMP_4405,TMP_4406)
 _agent.vaultCollateralIndex = tokenIndex.toUint16()
REF_2939(uint16) -> _agent_1 (-> []).vaultCollateralIndex
TMP_4408(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['tokenIndex_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2939(uint16) (->_agent_2 (-> [])) := TMP_4408(uint16)
 switchCollateralData = AgentCollateral.agentVaultCollateralData(_agent)
TMP_4409(Collateral.Data) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.agentVaultCollateralData(Agent.State), arguments:['_agent_2 (-> [])'] 
switchCollateralData_1(Collateral.Data) := TMP_4409(Collateral.Data)
 crBIPS = AgentCollateral.collateralRatioBIPS(switchCollateralData,_agent)
TMP_4410(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.collateralRatioBIPS(Collateral.Data,Agent.State), arguments:['switchCollateralData_1', '_agent_2 (-> [])'] 
crBIPS_1(uint256) := TMP_4410(uint256)
 require(bool,error)(crBIPS >= collateral.minCollateralRatioBIPS,revert NotEnoughCollateral()())
REF_2943(uint32) -> collateral_1 (-> ['state']).minCollateralRatioBIPS
TMP_4411(bool) = crBIPS_1 >= REF_2943
TMP_4412(None) = SOLIDITY_CALL revert NotEnoughCollateral()()
TMP_4413(None) = SOLIDITY_CALL require(bool,error)(TMP_4411,TMP_4412)
```
#### SafeCast.toUint16(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint16).max,SafeCast: value doesn't fit in 16 bits)
TMP_777(uint16) := 65535(uint16)
TMP_778(bool) = value_1 <= TMP_777
TMP_779(None) = SOLIDITY_CALL require(bool,string)(TMP_778,SafeCast: value doesn't fit in 16 bits)
 uint16(value)
TMP_780 = CONVERT value_1 to uint16
RETURN TMP_780
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
#### SafeCast.toUint32(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint32).max,SafeCast: value doesn't fit in 32 bits)
TMP_767(uint32) := 4294967295(uint32)
TMP_768(bool) = value_1 <= TMP_767
TMP_769(None) = SOLIDITY_CALL require(bool,string)(TMP_768,SafeCast: value doesn't fit in 32 bits)
 uint32(value)
TMP_770 = CONVERT value_1 to uint32
RETURN TMP_770
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
#### CollateralPool.setExitCollateralRatioBIPS(uint256) [EXTERNAL]
```slithir
 exitCollateralRatioBIPS = _exitCollateralRatioBIPS.toUint32()
TMP_6064(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_exitCollateralRatioBIPS_1'] 
exitCollateralRatioBIPS_2(uint32) := TMP_6064(uint32)
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
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
#### AgentCollateral.collateralRatioBIPS(Collateral.Data,Agent.State) [INTERNAL]
```slithir
 totalAMG = totalBackedAMG(_agent,_data.kind)
REF_2897(Collateral.Kind) -> _data_1.kind
TMP_4370(uint256) = INTERNAL_CALL, AgentCollateral.totalBackedAMG(Agent.State,Collateral.Kind)(_agent_1 (-> []),REF_2897)
totalAMG_1(uint256) := TMP_4370(uint256)
 backingTokenWei = Conversion.convertAmgToTokenWei(totalAMG,_data.amgToTokenWeiPrice)
REF_2899(uint256) -> _data_1.amgToTokenWeiPrice
TMP_4371(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToTokenWei(uint256,uint256), arguments:['totalAMG_1', 'REF_2899'] 
backingTokenWei_1(uint256) := TMP_4371(uint256)
 backingTokenWei == 0
TMP_4372(bool) = backingTokenWei_1 == 0
CONDITION TMP_4372
 1e10
RETURN 10000000000
 _data.fullCollateral.mulDiv(SafePct.MAX_BIPS,backingTokenWei)
REF_2900(uint256) -> _data_1.fullCollateral
REF_2902(uint256) -> SafePct.MAX_BIPS
TMP_4373(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['REF_2900', 'REF_2902', 'backingTokenWei_1'] 
RETURN TMP_4373
```
#### CollateralTypes.getIndex(CollateralType.Class,IERC20) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4569(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4569'])(AssetManagerState.State) := TMP_4569(AssetManagerState.State)
 index = state.collateralTokenIndex[_tokenKey(_collateralClass,_token)]
REF_3067(mapping(bytes32 => uint256)) -> state_1 (-> ['TMP_4569']).collateralTokenIndex
TMP_4570(bytes32) = INTERNAL_CALL, CollateralTypes._tokenKey(CollateralType.Class,IERC20)(_collateralClass_1,_token_1)
REF_3068(uint256) -> REF_3067[TMP_4570]
index_1(uint256) := REF_3068(uint256)
 require(bool,error)(index > 0,revert UnknownToken()())
TMP_4571(bool) = index_1 > 0
TMP_4572(None) = SOLIDITY_CALL revert UnknownToken()()
TMP_4573(None) = SOLIDITY_CALL require(bool,error)(TMP_4571,TMP_4572)
 index - 1
TMP_4574(uint256) = index_1 (c)- 1
RETURN TMP_4574
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

#### Conversion.convertAmgToTokenWei(uint256,uint256) [INTERNAL]
```slithir
AMG_TOKEN_WEI_PRICE_SCALE_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_0'])
 _valueAMG.mulDiv(_amgToTokenWeiPrice,AMG_TOKEN_WEI_PRICE_SCALE)
TMP_4674(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_valueAMG_1', '_amgToTokenWeiPrice_1', 'AMG_TOKEN_WEI_PRICE_SCALE_1'] 
RETURN TMP_4674
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
