
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
#### AgentCollateral.collateralRequiredToMintAmount(Collateral.Data,Agent.State,uint256,bool) [INTERNAL]
```slithir
_data_1(Collateral.Data) := phi(['_data_1'])
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_amountAMG_1(uint256) := phi(['REF_2868'])
_chargePoolFee_1(bool) := phi(['_chargePoolFee_1'])
 totalMintAmountAMG = _amountAMG + amountPoolFeeAMG
TMP_4348(uint256) = _amountAMG_1 (c)+ amountPoolFeeAMG_3
totalMintAmountAMG_1(uint256) := TMP_4348(uint256)
 totalMintAmountWei = Conversion.convertAmgToTokenWei(totalMintAmountAMG,_data.amgToTokenWeiPrice)
REF_2870(uint256) -> _data_1.amgToTokenWeiPrice
TMP_4349(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToTokenWei(uint256,uint256), arguments:['totalMintAmountAMG_1', 'REF_2870'] 
totalMintAmountWei_1(uint256) := TMP_4349(uint256)
 (mintingCollateralRatio,None) = mintingMinCollateralRatio(_agent,_data.kind)
REF_2871(Collateral.Kind) -> _data_1.kind
TUPLE_40(uint256,uint256) = INTERNAL_CALL, AgentCollateral.mintingMinCollateralRatio(Agent.State,Collateral.Kind)(_agent_1 (-> []),REF_2871)
mintingCollateralRatio_1(uint256)= UNPACK TUPLE_40 index: 0 
 totalMintAmountWei.mulBips(mintingCollateralRatio)
TMP_4350(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['totalMintAmountWei_1', 'mintingCollateralRatio_1'] 
RETURN TMP_4350
 _chargePoolFee
CONDITION _chargePoolFee_1
 amountPoolFeeAMG = _amountAMG.mulBips(_agent.feeBIPS).mulBips(_agent.poolFeeShareBIPS)
REF_2874(uint16) -> _agent_1 (-> []).feeBIPS
TMP_4351(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['_amountAMG_1', 'REF_2874'] 
REF_2876(uint16) -> _agent_1 (-> []).poolFeeShareBIPS
TMP_4352(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_4351', 'REF_2876'] 
amountPoolFeeAMG_1(uint256) := TMP_4352(uint256)
 amountPoolFeeAMG = 0
amountPoolFeeAMG_2(uint256) := 0(uint256)
amountPoolFeeAMG_3(uint256) := phi(['amountPoolFeeAMG_1', 'amountPoolFeeAMG_2'])
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
#### AgentCollateral.freeCollateralWei(Collateral.Data,Agent.State) [INTERNAL]
```slithir
_data_1(Collateral.Data) := phi(['_data_1'])
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 lockedCollateral = lockedCollateralWei(_data,_agent)
TMP_4332(uint256) = INTERNAL_CALL, AgentCollateral.lockedCollateralWei(Collateral.Data,Agent.State)(_data_1,_agent_1 (-> []))
lockedCollateral_1(uint256) := TMP_4332(uint256)
 MathUtils.subOrZero(_data.fullCollateral,lockedCollateral)
REF_2848(uint256) -> _data_1.fullCollateral
TMP_4333(uint256) = LIBRARY_CALL, dest:MathUtils, function:MathUtils.subOrZero(uint256,uint256), arguments:['REF_2848', 'lockedCollateral_1'] 
RETURN TMP_4333
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
#### AgentCollateral.lockedCollateralWei(Collateral.Data,Agent.State) [INTERNAL]
```slithir
_data_1(Collateral.Data) := phi(['_data_1'])
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 (mintingMinCollateralRatioBIPS,systemMinCollateralRatioBIPS) = mintingMinCollateralRatio(_agent,_data.kind)
REF_2849(Collateral.Kind) -> _data_1.kind
TUPLE_39(uint256,uint256) = INTERNAL_CALL, AgentCollateral.mintingMinCollateralRatio(Agent.State,Collateral.Kind)(_agent_1 (-> []),REF_2849)
mintingMinCollateralRatioBIPS_1(uint256)= UNPACK TUPLE_39 index: 0 
systemMinCollateralRatioBIPS_1(uint256)= UNPACK TUPLE_39 index: 1 
 backedAMG = uint256(_agent.reservedAMG) + uint256(_agent.mintedAMG)
REF_2850(uint64) -> _agent_1 (-> []).reservedAMG
TMP_4334 = CONVERT REF_2850 to uint256
REF_2851(uint64) -> _agent_1 (-> []).mintedAMG
TMP_4335 = CONVERT REF_2851 to uint256
TMP_4336(uint256) = TMP_4334 (c)+ TMP_4335
backedAMG_1(uint256) := TMP_4336(uint256)
 mintingCollateral = Conversion.convertAmgToTokenWei(backedAMG,_data.amgToTokenWeiPrice).mulBips(mintingMinCollateralRatioBIPS)
REF_2853(uint256) -> _data_1.amgToTokenWeiPrice
TMP_4337(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToTokenWei(uint256,uint256), arguments:['backedAMG_1', 'REF_2853'] 
TMP_4338(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_4337', 'mintingMinCollateralRatioBIPS_1'] 
mintingCollateral_1(uint256) := TMP_4338(uint256)
 redeemingCollateral = Conversion.convertAmgToTokenWei(redeemingAMG,_data.amgToTokenWeiPrice).mulBips(systemMinCollateralRatioBIPS)
REF_2856(uint256) -> _data_1.amgToTokenWeiPrice
TMP_4339(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToTokenWei(uint256,uint256), arguments:['redeemingAMG_3', 'REF_2856'] 
TMP_4340(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_4339', 'systemMinCollateralRatioBIPS_1'] 
redeemingCollateral_1(uint256) := TMP_4340(uint256)
 mintingCollateral + redeemingCollateral + announcedWithdrawal
TMP_4341(uint256) = mintingCollateral_1 (c)+ redeemingCollateral_1
TMP_4342(uint256) = TMP_4341 (c)+ announcedWithdrawal_3
RETURN TMP_4342
 _data.kind == Collateral.Kind.POOL
REF_2858(Collateral.Kind) -> _data_1.kind
REF_2859(Collateral.Kind) -> Kind.POOL
TMP_4343(bool) = REF_2858 == REF_2859
CONDITION TMP_4343
 redeemingAMG = _agent.poolRedeemingAMG
REF_2860(uint64) -> _agent_1 (-> []).poolRedeemingAMG
redeemingAMG_2(uint64) := REF_2860(uint64)
 redeemingAMG = _agent.redeemingAMG
REF_2861(uint64) -> _agent_1 (-> []).redeemingAMG
redeemingAMG_1(uint64) := REF_2861(uint64)
redeemingAMG_3(uint64) := phi(['redeemingAMG_1', 'redeemingAMG_2'])
 _data.kind != Collateral.Kind.POOL
REF_2862(Collateral.Kind) -> _data_1.kind
REF_2863(Collateral.Kind) -> Kind.POOL
TMP_4344(bool) = REF_2862 != REF_2863
CONDITION TMP_4344
 announcedWithdrawal = _agent.withdrawalAnnouncement(_data.kind).amountWei
REF_2865(Collateral.Kind) -> _data_1.kind
TMP_4345(Agent.WithdrawalAnnouncement) = LIBRARY_CALL, dest:Agents, function:Agents.withdrawalAnnouncement(Agent.State,Collateral.Kind), arguments:['_agent_1 (-> [])', 'REF_2865'] 
REF_2866(uint128) -> TMP_4345.amountWei
announcedWithdrawal_1(uint256) := REF_2866(uint128)
 announcedWithdrawal = 0
announcedWithdrawal_2(uint256) := 0(uint256)
announcedWithdrawal_3(uint256) := phi(['announcedWithdrawal_1', 'announcedWithdrawal_2'])
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
#### AgentCollateral.mintingLotCollateralWei(Collateral.Data,Agent.State,bool) [INTERNAL]
```slithir
_data_1(Collateral.Data) := phi(['_data_1'])
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_chargePoolFee_1(bool) := phi(['_chargePoolFee_1'])
 settings = Globals.getSettings()
TMP_4346(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4346'])(AssetManagerSettings.Data) := TMP_4346(AssetManagerSettings.Data)
 collateralRequiredToMintAmount(_data,_agent,settings.lotSizeAMG,_chargePoolFee)
REF_2868(uint64) -> settings_1 (-> ['TMP_4346']).lotSizeAMG
TMP_4347(uint256) = INTERNAL_CALL, AgentCollateral.collateralRequiredToMintAmount(Collateral.Data,Agent.State,uint256,bool)(_data_1,_agent_1 (-> []),REF_2868,_chargePoolFee_1)
RETURN TMP_4347
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
#### AgentCollateral.singleCollateralData(Agent.State,Collateral.Kind) [INTERNAL]
```slithir
 _kind == Collateral.Kind.VAULT
REF_2815(Collateral.Kind) -> Kind.VAULT
TMP_4300(bool) = _kind_1 == REF_2815
CONDITION TMP_4300
 agentVaultCollateralData(_agent)
TMP_4301(Collateral.Data) = INTERNAL_CALL, AgentCollateral.agentVaultCollateralData(Agent.State)(_agent_1 (-> []))
RETURN TMP_4301
 _kind == Collateral.Kind.POOL
REF_2816(Collateral.Kind) -> Kind.POOL
TMP_4302(bool) = _kind_1 == REF_2816
CONDITION TMP_4302
 poolCollateralData(_agent)
TMP_4303(Collateral.Data) = INTERNAL_CALL, AgentCollateral.poolCollateralData(Agent.State)(_agent_1 (-> []))
RETURN TMP_4303
 agentsPoolTokensCollateralData(_agent,poolCollateralData(_agent))
TMP_4304(Collateral.Data) = INTERNAL_CALL, AgentCollateral.poolCollateralData(Agent.State)(_agent_1 (-> []))
TMP_4305(Collateral.Data) = INTERNAL_CALL, AgentCollateral.agentsPoolTokensCollateralData(Agent.State,Collateral.Data)(_agent_1 (-> []),TMP_4304)
RETURN TMP_4305
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
#### ERC20.totalSupply() [PUBLIC]
```slithir
_totalSupply_1(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 _totalSupply
RETURN _totalSupply_1
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
#### Agents.withdrawalAnnouncement(Agent.State,Collateral.Kind) [INTERNAL]
```slithir
 assert(bool)(_kind != Collateral.Kind.POOL)
REF_3033(Collateral.Kind) -> Kind.POOL
TMP_4528(bool) = _kind_1 != REF_3033
TMP_4529(None) = SOLIDITY_CALL assert(bool)(TMP_4528)
 _kind == Collateral.Kind.VAULT
REF_3034(Collateral.Kind) -> Kind.VAULT
TMP_4530(bool) = _kind_1 == REF_3034
CONDITION TMP_4530
 _agent.vaultCollateralWithdrawalAnnouncement
REF_3035(Agent.WithdrawalAnnouncement) -> _agent_1 (-> []).vaultCollateralWithdrawalAnnouncement
RETURN REF_3035
 _agent.poolTokenWithdrawalAnnouncement
REF_3036(Agent.WithdrawalAnnouncement) -> _agent_1 (-> []).poolTokenWithdrawalAnnouncement
RETURN REF_3036
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
