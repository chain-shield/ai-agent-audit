


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








#### Liquidation._collateralDataWithTrusted(Agent.State,Collateral.Kind) [PRIVATE]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_kind_1(Collateral.Kind) := phi(['_collateralKind_1'])
 collateral = _agent.getCollateral(_kind)
TMP_4773(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getCollateral(Agent.State,Collateral.Kind), arguments:['_agent_1 (-> [])', '_kind_1'] 
collateral_1 (-> ['TMP_4773'])(CollateralTypeInt.Data) := TMP_4773(CollateralTypeInt.Data)
 fullCollateral = _getCollateralAmount(_agent,_kind,collateral)
TMP_4774(uint256) = INTERNAL_CALL, Liquidation._getCollateralAmount(Agent.State,Collateral.Kind,CollateralTypeInt.Data)(_agent_1 (-> []),_kind_1,collateral_1 (-> ['TMP_4773']))
fullCollateral_1(uint256) := TMP_4774(uint256)
 (price,trusted) = Conversion.currentAmgPriceInTokenWeiWithTrusted(collateral)
TUPLE_58(uint256,uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.currentAmgPriceInTokenWeiWithTrusted(CollateralTypeInt.Data), arguments:["collateral_1 (-> ['TMP_4773'])"] 
price_1(uint256)= UNPACK TUPLE_58 index: 0 
trusted_1(uint256)= UNPACK TUPLE_58 index: 1 
 _data = Collateral.Data({kind:_kind,fullCollateral:fullCollateral,amgToTokenWeiPrice:price})
TMP_4775(Collateral.Data) = new Data(_kind_1,fullCollateral_1,price_1)
_data_1(Collateral.Data) := TMP_4775(Collateral.Data)
 _trustedData = Collateral.Data({kind:_kind,fullCollateral:fullCollateral,amgToTokenWeiPrice:trusted})
TMP_4776(Collateral.Data) = new Data(_kind_1,fullCollateral_1,trusted_1)
_trustedData_1(Collateral.Data) := TMP_4776(Collateral.Data)
 (_data,_trustedData)
RETURN _data_1,_trustedData_1
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
#### Liquidation.maxLiquidationAmountAMG(Agent.State,uint256,uint256,Collateral.Kind) [INTERNAL]
```slithir
 _agent.status == Agent.Status.FULL_LIQUIDATION
REF_3274(Agent.Status) -> _agent_1 (-> []).status
REF_3275(Agent.Status) -> Status.FULL_LIQUIDATION
TMP_4761(bool) = REF_3274 == REF_3275
CONDITION TMP_4761
 _agent.mintedAMG
REF_3276(uint64) -> _agent_1 (-> []).mintedAMG
RETURN REF_3276
 targetRatioBIPS = _targetRatioBIPS(_agent,_collateralKind)
TMP_4762(uint256) = INTERNAL_CALL, Liquidation._targetRatioBIPS(Agent.State,Collateral.Kind)(_agent_1 (-> []),_collateralKind_1)
targetRatioBIPS_1(uint256) := TMP_4762(uint256)
 targetRatioBIPS <= _collateralRatioBIPS
TMP_4763(bool) = targetRatioBIPS_1 <= _collateralRatioBIPS_1
CONDITION TMP_4763
 0
RETURN 0
 _collateralRatioBIPS <= _factorBIPS
TMP_4764(bool) = _collateralRatioBIPS_1 <= _factorBIPS_1
CONDITION TMP_4764
 _agent.mintedAMG
REF_3277(uint64) -> _agent_1 (-> []).mintedAMG
RETURN REF_3277
 maxLiquidatedAMG = AgentCollateral.totalBackedAMG(_agent,_collateralKind).mulDivRoundUp(targetRatioBIPS - _collateralRatioBIPS,targetRatioBIPS - _factorBIPS)
TMP_4765(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.totalBackedAMG(Agent.State,Collateral.Kind), arguments:['_agent_1 (-> [])', '_collateralKind_1'] 
TMP_4766(uint256) = targetRatioBIPS_1 (c)- _collateralRatioBIPS_1
TMP_4767(uint256) = targetRatioBIPS_1 (c)- _factorBIPS_1
TMP_4768(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDivRoundUp(uint256,uint256,uint256), arguments:['TMP_4765', 'TMP_4766', 'TMP_4767'] 
maxLiquidatedAMG_1(uint256) := TMP_4768(uint256)
 Math.min(maxLiquidatedAMG,_agent.mintedAMG)
REF_3281(uint64) -> _agent_1 (-> []).mintedAMG
TMP_4769(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['maxLiquidatedAMG_1', 'REF_3281'] 
RETURN TMP_4769
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
#### Conversion.currentAmgPriceInTokenWeiWithTrusted(CollateralTypeInt.Data) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4631(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4631'])(AssetManagerSettings.Data) := TMP_4631(AssetManagerSettings.Data)
 (ftsoPrice,assetTimestamp,tokenTimestamp) = currentAmgPriceInTokenWeiWithTs(_token,false)
TUPLE_45(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool)(_token_1 (-> []),False)
ftsoPrice_1(uint256)= UNPACK TUPLE_45 index: 0 
assetTimestamp_1(uint256)= UNPACK TUPLE_45 index: 1 
tokenTimestamp_1(uint256)= UNPACK TUPLE_45 index: 2 
 (trustedPrice,assetTimestampTrusted,tokenTimestampTrusted) = currentAmgPriceInTokenWeiWithTs(_token,true)
TUPLE_46(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool)(_token_1 (-> []),True)
trustedPrice_1(uint256)= UNPACK TUPLE_46 index: 0 
assetTimestampTrusted_1(uint256)= UNPACK TUPLE_46 index: 1 
tokenTimestampTrusted_1(uint256)= UNPACK TUPLE_46 index: 2 
 trustedPriceFresh = tokenTimestampTrusted + settings.maxTrustedPriceAgeSeconds >= tokenTimestamp && assetTimestampTrusted + settings.maxTrustedPriceAgeSeconds >= assetTimestamp
REF_3142(uint64) -> settings_1 (-> ['TMP_4631']).maxTrustedPriceAgeSeconds
TMP_4632(uint256) = tokenTimestampTrusted_1 (c)+ REF_3142
TMP_4633(bool) = TMP_4632 >= tokenTimestamp_1
REF_3143(uint64) -> settings_1 (-> ['TMP_4631']).maxTrustedPriceAgeSeconds
TMP_4634(uint256) = assetTimestampTrusted_1 (c)+ REF_3143
TMP_4635(bool) = TMP_4634 >= assetTimestamp_1
TMP_4636(bool) = TMP_4633 && TMP_4635
trustedPriceFresh_1(bool) := TMP_4636(bool)
 _ftsoPrice = ftsoPrice
_ftsoPrice_1(uint256) := ftsoPrice_1(uint256)
 trustedPriceFresh
CONDITION trustedPriceFresh_1
 _trustedPrice = trustedPrice
_trustedPrice_1(uint256) := trustedPrice_1(uint256)
 _trustedPrice = ftsoPrice
_trustedPrice_2(uint256) := ftsoPrice_1(uint256)
_trustedPrice_3(uint256) := phi(['_trustedPrice_1', '_trustedPrice_2'])
 (_ftsoPrice,_trustedPrice)
RETURN _ftsoPrice_1,_trustedPrice_3
```
#### CollateralTypes.isValid(CollateralTypeInt.Data) [INTERNAL]
```slithir
 _token.validUntil == 0 || _token.validUntil > block.timestamp
REF_3072(uint64) -> _token_1 (-> []).validUntil
TMP_4578(bool) = REF_3072 == 0
REF_3073(uint64) -> _token_1 (-> []).validUntil
TMP_4579(bool) = REF_3073 > block.timestamp
TMP_4580(bool) = TMP_4578 || TMP_4579
RETURN TMP_4580
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
#### Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256) [INTERNAL]
```slithir
_tokenDecimals_1(uint256) := phi(['REF_3165', 'REF_3167'])
_tokenPrice_1(uint256) := phi(['tokenPrice_1'])
_tokenFtsoDecimals_1(uint256) := phi(['tokenFtsoDec_1'])
_assetPrice_1(uint256) := phi(['assetPrice_1'])
_assetFtsoDecimals_1(uint256) := phi(['assetFtsoDec_1'])
AMG_TOKEN_WEI_PRICE_SCALE_EXP_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_EXP_0'])
 settings = Globals.getSettings()
TMP_4665(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4665'])(AssetManagerSettings.Data) := TMP_4665(AssetManagerSettings.Data)
 expPlus = _tokenDecimals + _tokenFtsoDecimals + AMG_TOKEN_WEI_PRICE_SCALE_EXP
TMP_4666(uint256) = _tokenDecimals_1 (c)+ _tokenFtsoDecimals_1
TMP_4667(uint256) = TMP_4666 (c)+ AMG_TOKEN_WEI_PRICE_SCALE_EXP_1
expPlus_1(uint256) := TMP_4667(uint256)
 expMinus = settings.assetMintingDecimals + _assetFtsoDecimals
REF_3173(uint8) -> settings_1 (-> ['TMP_4665']).assetMintingDecimals
TMP_4668(uint8) = REF_3173 (c)+ _assetFtsoDecimals_1
expMinus_1(uint256) := TMP_4668(uint8)
 assert(bool)(expPlus >= expMinus)
TMP_4669(bool) = expPlus_1 >= expMinus_1
TMP_4670(None) = SOLIDITY_CALL assert(bool)(TMP_4669)
 _assetPrice.mulDiv(10 ** (expPlus - expMinus),_tokenPrice)
TMP_4671(uint256) = expPlus_1 (c)- expMinus_1
TMP_4672(uint256) = 10 (c)** TMP_4671
TMP_4673(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_assetPrice_1', 'TMP_4672', '_tokenPrice_1'] 
RETURN TMP_4673
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
