






### Storage layout (AgentVault) 

```text
assetManager IIAssetManager
initialized bool
__usedTokens IERC20[]
__tokenUseFlags mapping(IERC20 => uint256)
__internalWithdrawal bool
destroyed bool

```
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

### Storage layout (CollateralPoolToken) 

```text
collateralPool address
tokenName string
tokenSymbol string
timelocksByAccount mapping(address => CollateralPoolToken.TimelockQueue)
ignoreTimelocked bool
initialized bool

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



#### LiquidationFacet._isCollateralUnderwater(uint256,uint256) [PRIVATE]
```slithir
_collateralRatioBIPS_1(uint256) := phi(['REF_1705', 'REF_1701'])
_collateralIndex_1(uint256) := phi(['REF_1706', 'REF_1702'])
 state = AssetManagerState.get()
TMP_2940(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2940'])(AssetManagerState.State) := TMP_2940(AssetManagerState.State)
 collateral = state.collateralTokens[_collateralIndex]
REF_1716(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_2940']).collateralTokens
REF_1717(CollateralTypeInt.Data) -> REF_1716[_collateralIndex_1]
collateral_1 (-> ['state'])(CollateralTypeInt.Data) := REF_1717(CollateralTypeInt.Data)
 _collateralRatioBIPS < collateral.minCollateralRatioBIPS
REF_1718(uint32) -> collateral_1 (-> ['state']).minCollateralRatioBIPS
TMP_2941(bool) = _collateralRatioBIPS_1 < REF_1718
RETURN TMP_2941
```
#### LiquidationFacet._performLiquidation(Agent.State,Liquidation.CRData,uint64) [PRIVATE]
```slithir
_agent_1 (-> ['TMP_2902'])(Agent.State) := phi(["agent_1 (-> ['TMP_2902'])"])
_cr_1(Liquidation.CRData) := phi(['cr_1'])
_amountAMG_1(uint64) := phi(['TMP_2907'])
 (vaultFactor,poolFactor) = LiquidationPaymentStrategy.currentLiquidationFactorBIPS(_agent,_cr.vaultCR,_cr.poolCR)
REF_1720(uint256) -> _cr_1.vaultCR
REF_1721(uint256) -> _cr_1.poolCR
TUPLE_31(uint256,uint256) = LIBRARY_CALL, dest:LiquidationPaymentStrategy, function:LiquidationPaymentStrategy.currentLiquidationFactorBIPS(Agent.State,uint256,uint256), arguments:["_agent_1 (-> ['TMP_2902'])", 'REF_1720', 'REF_1721'] 
vaultFactor_1(uint256)= UNPACK TUPLE_31 index: 0 
poolFactor_1(uint256)= UNPACK TUPLE_31 index: 1 
 maxLiquidatedAMG = Math.max(Liquidation.maxLiquidationAmountAMG(_agent,_cr.vaultCR,vaultFactor,Collateral.Kind.VAULT),Liquidation.maxLiquidationAmountAMG(_agent,_cr.poolCR,poolFactor,Collateral.Kind.POOL))
REF_1724(uint256) -> _cr_1.vaultCR
REF_1725(Collateral.Kind) -> Kind.VAULT
TMP_2942(uint256) = LIBRARY_CALL, dest:Liquidation, function:Liquidation.maxLiquidationAmountAMG(Agent.State,uint256,uint256,Collateral.Kind), arguments:["_agent_1 (-> ['TMP_2902'])", 'REF_1724', 'vaultFactor_1', 'REF_1725'] 
REF_1727(uint256) -> _cr_1.poolCR
REF_1728(Collateral.Kind) -> Kind.POOL
TMP_2943(uint256) = LIBRARY_CALL, dest:Liquidation, function:Liquidation.maxLiquidationAmountAMG(Agent.State,uint256,uint256,Collateral.Kind), arguments:["_agent_1 (-> ['TMP_2902'])", 'REF_1727', 'poolFactor_1', 'REF_1728'] 
TMP_2944(uint256) = LIBRARY_CALL, dest:Math, function:Math.max(uint256,uint256), arguments:['TMP_2942', 'TMP_2943'] 
maxLiquidatedAMG_1(uint256) := TMP_2944(uint256)
 amountToLiquidateAMG = Math.min(maxLiquidatedAMG,_amountAMG).toUint64()
TMP_2945(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['maxLiquidatedAMG_1', '_amountAMG_1'] 
TMP_2946(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_2945'] 
amountToLiquidateAMG_1(uint64) := TMP_2946(uint64)
 (_liquidatedAMG,None) = Redemptions.closeTickets(_agent,amountToLiquidateAMG,true)
TUPLE_32(uint64,uint256) = LIBRARY_CALL, dest:Redemptions, function:Redemptions.closeTickets(Agent.State,uint64,bool), arguments:["_agent_1 (-> ['TMP_2902'])", 'amountToLiquidateAMG_1', 'True'] 
_liquidatedAMG_1(uint64)= UNPACK TUPLE_32 index: 0 
 _payoutC1Wei = Conversion.convertAmgToTokenWei(uint256(_liquidatedAMG).mulBips(vaultFactor),_cr.amgToC1WeiPrice)
TMP_2947 = CONVERT _liquidatedAMG_1 to uint256
TMP_2948(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_2947', 'vaultFactor_1'] 
REF_1734(uint256) -> _cr_1.amgToC1WeiPrice
TMP_2949(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToTokenWei(uint256,uint256), arguments:['TMP_2948', 'REF_1734'] 
_payoutC1Wei_1(uint256) := TMP_2949(uint256)
 _payoutPoolWei = Conversion.convertAmgToTokenWei(uint256(_liquidatedAMG).mulBips(poolFactor),_cr.amgToPoolWeiPrice)
TMP_2950 = CONVERT _liquidatedAMG_1 to uint256
TMP_2951(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_2950', 'poolFactor_1'] 
REF_1737(uint256) -> _cr_1.amgToPoolWeiPrice
TMP_2952(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToTokenWei(uint256,uint256), arguments:['TMP_2951', 'REF_1737'] 
_payoutPoolWei_1(uint256) := TMP_2952(uint256)
 (_liquidatedAMG,_payoutC1Wei,_payoutPoolWei)
RETURN _liquidatedAMG_1,_payoutC1Wei_1,_payoutPoolWei_1
```
#### LiquidationFacet._startLiquidation(Agent.State,Liquidation.CRData) [PRIVATE]
```slithir
_agent_1 (-> ['TMP_2895', 'TMP_2902'])(Agent.State) := phi(["agent_1 (-> ['TMP_2895'])", "agent_1 (-> ['TMP_2902'])"])
_cr_1(Liquidation.CRData) := phi(['cr_1', 'cr_1'])
 status = _agent.status
REF_1696(Agent.Status) -> _agent_1 (-> ['TMP_2895', 'TMP_2902']).status
status_1(Agent.Status) := REF_1696(Agent.Status)
 status == Agent.Status.LIQUIDATION || status == Agent.Status.FULL_LIQUIDATION
REF_1697(Agent.Status) -> Status.LIQUIDATION
TMP_2926(bool) = status_1 == REF_1697
REF_1698(Agent.Status) -> Status.FULL_LIQUIDATION
TMP_2927(bool) = status_1 == REF_1698
TMP_2928(bool) = TMP_2926 || TMP_2927
CONDITION TMP_2928
 _inLiquidation = true
_inLiquidation_1(bool) := True(bool)
 status != Agent.Status.NORMAL
REF_1699(Agent.Status) -> Status.NORMAL
TMP_2929(bool) = status_1 != REF_1699
CONDITION TMP_2929
 revert LiquidationNotPossible(AgentInfo.Status)(Agents.getAgentStatus(_agent))
TMP_2930(AgentInfo.Status) = LIBRARY_CALL, dest:Agents, function:Agents.getAgentStatus(Agent.State), arguments:["_agent_1 (-> ['TMP_2895', 'TMP_2902'])"] 
TMP_2931(None) = SOLIDITY_CALL revert LiquidationNotPossible(AgentInfo.Status)(TMP_2930)
_inLiquidation_2(bool) := phi(['_inLiquidation_1', '_inLiquidation_0'])
 vaultUnderwater = _isCollateralUnderwater(_cr.vaultCR,_agent.vaultCollateralIndex)
REF_1701(uint256) -> _cr_1.vaultCR
REF_1702(uint16) -> _agent_1 (-> ['TMP_2895', 'TMP_2902']).vaultCollateralIndex
TMP_2932(bool) = INTERNAL_CALL, LiquidationFacet._isCollateralUnderwater(uint256,uint256)(REF_1701,REF_1702)
vaultUnderwater_1(bool) := TMP_2932(bool)
 vaultUnderwater
CONDITION vaultUnderwater_1
 _agent.collateralsUnderwater |= Agent.LF_VAULT
REF_1703(uint8) -> _agent_1 (-> ['TMP_2895', 'TMP_2902']).collateralsUnderwater
REF_1704(uint8) -> Agent.LF_VAULT
_agent_2 (-> ['TMP_2895', 'TMP_2902'])(Agent.State) := phi(["_agent_1 (-> ['TMP_2895', 'TMP_2902'])"])
REF_1703(-> _agent_2 (-> ['TMP_2895', 'TMP_2902'])) = REF_1703 | REF_1704
_agent_3 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902'])(Agent.State) := phi(["_agent_2 (-> ['TMP_2895', 'TMP_2902'])", "_agent_1 (-> ['TMP_2895', 'TMP_2902'])"])
 poolUnderwater = _isCollateralUnderwater(_cr.poolCR,_agent.poolCollateralIndex)
REF_1705(uint256) -> _cr_1.poolCR
REF_1706(uint16) -> _agent_3 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902']).poolCollateralIndex
TMP_2933(bool) = INTERNAL_CALL, LiquidationFacet._isCollateralUnderwater(uint256,uint256)(REF_1705,REF_1706)
poolUnderwater_1(bool) := TMP_2933(bool)
 poolUnderwater
CONDITION poolUnderwater_1
 _agent.collateralsUnderwater |= Agent.LF_POOL
REF_1707(uint8) -> _agent_3 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902']).collateralsUnderwater
REF_1708(uint8) -> Agent.LF_POOL
_agent_4 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902'])(Agent.State) := phi(["_agent_3 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902'])"])
REF_1707(-> _agent_4 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902'])) = REF_1707 | REF_1708
_agent_5 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902'])(Agent.State) := phi(["_agent_4 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902'])", "_agent_1 (-> ['TMP_2895', 'TMP_2902'])"])
 ! _inLiquidation && (vaultUnderwater || poolUnderwater)
TMP_2934 = UnaryType.BANG _inLiquidation_2 
TMP_2935(bool) = vaultUnderwater_1 || poolUnderwater_1
TMP_2936(bool) = TMP_2934 && TMP_2935
CONDITION TMP_2936
 _inLiquidation = true
_inLiquidation_3(bool) := True(bool)
 _agent.status = Agent.Status.LIQUIDATION
REF_1709(Agent.Status) -> _agent_5 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902']).status
REF_1710(Agent.Status) -> Status.LIQUIDATION
_agent_6 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902'])(Agent.State) := phi(["_agent_5 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902'])"])
REF_1709(Agent.Status) (->_agent_6 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902'])) := REF_1710(Agent.Status)
 _agent.liquidationStartedAt = block.timestamp.toUint64()
REF_1711(uint64) -> _agent_6 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902']).liquidationStartedAt
TMP_2937(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.timestamp'] 
_agent_7 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902'])(Agent.State) := phi(["_agent_6 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902'])"])
REF_1711(uint64) (->_agent_7 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902'])) := TMP_2937(uint64)
 IAssetManagerEvents.LiquidationStarted(_agent.vaultAddress(),block.timestamp)
TMP_2938(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:["_agent_7 (-> ['TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902', 'TMP_2895', 'TMP_2902'])"] 
Emit LiquidationStarted(TMP_2938,block.timestamp)
_inLiquidation_4(bool) := phi(['_inLiquidation_3', '_inLiquidation_0'])
 _inLiquidation
RETURN _inLiquidation_4
```
#### LiquidationFacet.endLiquidation(address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_2920(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2920'])(Agent.State) := TMP_2920(Agent.State)
 Liquidation.endLiquidationIfHealthy(agent)
LIBRARY_CALL, dest:Liquidation, function:Liquidation.endLiquidationIfHealthy(Agent.State), arguments:["agent_1 (-> ['TMP_2920'])"] 
 require(bool,error)(agent.status == Agent.Status.NORMAL,revert CannotStopLiquidation()())
REF_1694(Agent.Status) -> agent_1 (-> ['TMP_2920']).status
REF_1695(Agent.Status) -> Status.NORMAL
TMP_2922(bool) = REF_1694 == REF_1695
TMP_2923(None) = SOLIDITY_CALL revert CannotStopLiquidation()()
TMP_2924(None) = SOLIDITY_CALL require(bool,error)(TMP_2922,TMP_2923)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### LiquidationFacet.liquidate(address,uint256) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_2902(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2902'])(Agent.State) := TMP_2902(Agent.State)
 cr = Liquidation.getCollateralRatiosBIPS(agent)
TMP_2903(Liquidation.CRData) = LIBRARY_CALL, dest:Liquidation, function:Liquidation.getCollateralRatiosBIPS(Agent.State), arguments:["agent_1 (-> ['TMP_2902'])"] 
cr_1(Liquidation.CRData) := TMP_2903(Liquidation.CRData)
 inLiquidation = _startLiquidation(agent,cr)
TMP_2904(bool) = INTERNAL_CALL, LiquidationFacet._startLiquidation(Agent.State,Liquidation.CRData)(agent_1 (-> ['TMP_2902']),cr_1)
inLiquidation_1(bool) := TMP_2904(bool)
 require(bool,error)(inLiquidation,revert NotInLiquidation()())
TMP_2905(None) = SOLIDITY_CALL revert NotInLiquidation()()
TMP_2906(None) = SOLIDITY_CALL require(bool,error)(inLiquidation_1,TMP_2905)
 (liquidatedAmountAMG,payoutC1Wei,payoutPoolWei) = _performLiquidation(agent,cr,Conversion.convertUBAToAmg(_amountUBA))
TMP_2907(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['_amountUBA_1'] 
TUPLE_30(uint64,uint256,uint256) = INTERNAL_CALL, LiquidationFacet._performLiquidation(Agent.State,Liquidation.CRData,uint64)(agent_1 (-> ['TMP_2902']),cr_1,TMP_2907)
liquidatedAmountAMG_1(uint64)= UNPACK TUPLE_30 index: 0 
payoutC1Wei_1(uint256)= UNPACK TUPLE_30 index: 1 
payoutPoolWei_1(uint256)= UNPACK TUPLE_30 index: 2 
 _liquidatedAmountUBA = Conversion.convertAmgToUBA(liquidatedAmountAMG)
TMP_2908(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['liquidatedAmountAMG_1'] 
_liquidatedAmountUBA_1(uint256) := TMP_2908(uint256)
 payoutC1Wei > 0
TMP_2909(bool) = payoutC1Wei_1 > 0
CONDITION TMP_2909
 _amountPaidVault = AgentPayout.payoutFromVault(agent,msg.sender,payoutC1Wei)
TMP_2910(uint256) = LIBRARY_CALL, dest:AgentPayout, function:AgentPayout.payoutFromVault(Agent.State,address,uint256), arguments:["agent_1 (-> ['TMP_2902'])", 'msg.sender', 'payoutC1Wei_1'] 
_amountPaidVault_1(uint256) := TMP_2910(uint256)
_amountPaidVault_2(uint256) := phi(['_amountPaidVault_0', '_amountPaidVault_1'])
 payoutPoolWei > 0
TMP_2911(bool) = payoutPoolWei_1 > 0
CONDITION TMP_2911
 agentResponsibilityWei = _agentResponsibilityWei(agent,payoutPoolWei)
TMP_2912(uint256) = INTERNAL_CALL, LiquidationFacet._agentResponsibilityWei(Agent.State,uint256)(agent_1 (-> ['TMP_2902']),payoutPoolWei_1)
agentResponsibilityWei_1(uint256) := TMP_2912(uint256)
 _amountPaidPool = AgentPayout.payoutFromPool(agent,msg.sender,payoutPoolWei,agentResponsibilityWei)
TMP_2913(uint256) = LIBRARY_CALL, dest:AgentPayout, function:AgentPayout.payoutFromPool(Agent.State,address,uint256,uint256), arguments:["agent_1 (-> ['TMP_2902'])", 'msg.sender', 'payoutPoolWei_1', 'agentResponsibilityWei_1'] 
_amountPaidPool_1(uint256) := TMP_2913(uint256)
_amountPaidPool_2(uint256) := phi(['_amountPaidPool_0', '_amountPaidPool_1'])
 _liquidatedAmountUBA > 0
TMP_2914(bool) = _liquidatedAmountUBA_1 > 0
CONDITION TMP_2914
 Redemptions.burnFAssets(msg.sender,_liquidatedAmountUBA)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.burnFAssets(address,uint256), arguments:['msg.sender', '_liquidatedAmountUBA_1'] 
 IAssetManagerEvents.LiquidationPerformed(_agentVault,msg.sender,_liquidatedAmountUBA,_amountPaidVault,_amountPaidPool)
Emit LiquidationPerformed(_agentVault_1,msg.sender,_liquidatedAmountUBA_1,_amountPaidVault_2,_amountPaidPool_2)
 Liquidation.endLiquidationIfHealthy(agent)
LIBRARY_CALL, dest:Liquidation, function:Liquidation.endLiquidationIfHealthy(Agent.State), arguments:["agent_1 (-> ['TMP_2902'])"] 
 notEmergencyPaused()
MODIFIER_CALL, AssetManagerBase.notEmergencyPaused()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 (_liquidatedAmountUBA,_amountPaidVault,_amountPaidPool)
RETURN _liquidatedAmountUBA_1,_amountPaidVault_2,_amountPaidPool_2
```
#### LiquidationFacet.startLiquidation(address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_2895(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2895'])(Agent.State) := TMP_2895(Agent.State)
 cr = Liquidation.getCollateralRatiosBIPS(agent)
TMP_2896(Liquidation.CRData) = LIBRARY_CALL, dest:Liquidation, function:Liquidation.getCollateralRatiosBIPS(Agent.State), arguments:["agent_1 (-> ['TMP_2895'])"] 
cr_1(Liquidation.CRData) := TMP_2896(Liquidation.CRData)
 inLiquidation = _startLiquidation(agent,cr)
TMP_2897(bool) = INTERNAL_CALL, LiquidationFacet._startLiquidation(Agent.State,Liquidation.CRData)(agent_1 (-> ['TMP_2895']),cr_1)
inLiquidation_1(bool) := TMP_2897(bool)
 require(bool,error)(inLiquidation,revert LiquidationNotStarted()())
TMP_2898(None) = SOLIDITY_CALL revert LiquidationNotStarted()()
TMP_2899(None) = SOLIDITY_CALL require(bool,error)(inLiquidation_1,TMP_2898)
 _liquidationStartTs = agent.liquidationStartedAt
REF_1682(uint64) -> agent_1 (-> ['TMP_2895']).liquidationStartedAt
_liquidationStartTs_1(uint256) := REF_1682(uint64)
 notEmergencyPaused()
MODIFIER_CALL, AssetManagerBase.notEmergencyPaused()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 _liquidationStartTs
RETURN _liquidationStartTs_1
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
#### Conversion.convertAmgToTokenWei(uint256,uint256) [INTERNAL]
```slithir
AMG_TOKEN_WEI_PRICE_SCALE_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_0'])
 _valueAMG.mulDiv(_amgToTokenWeiPrice,AMG_TOKEN_WEI_PRICE_SCALE)
TMP_4674(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_valueAMG_1', '_amgToTokenWeiPrice_1', 'AMG_TOKEN_WEI_PRICE_SCALE_1'] 
RETURN TMP_4674
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
#### LiquidationPaymentStrategy.currentLiquidationFactorBIPS(Agent.State,uint256,uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4783(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4783'])(AssetManagerSettings.Data) := TMP_4783(AssetManagerSettings.Data)
 step = _currentLiquidationStep(_agent)
TMP_4784(uint256) = INTERNAL_CALL, LiquidationPaymentStrategy._currentLiquidationStep(Agent.State)(_agent_1 (-> []))
step_1(uint256) := TMP_4784(uint256)
 factorBIPS = settings.liquidationCollateralFactorBIPS[step]
REF_3298(uint256[]) -> settings_1 (-> ['TMP_4783']).liquidationCollateralFactorBIPS
REF_3299(uint256) -> REF_3298[step_1]
factorBIPS_1(uint256) := REF_3299(uint256)
 _c1FactorBIPS = Math.min(settings.liquidationFactorVaultCollateralBIPS[step],factorBIPS)
REF_3301(uint256[]) -> settings_1 (-> ['TMP_4783']).liquidationFactorVaultCollateralBIPS
REF_3302(uint256) -> REF_3301[step_1]
TMP_4785(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['REF_3302', 'factorBIPS_1'] 
_c1FactorBIPS_1(uint256) := TMP_4785(uint256)
 vaultCollateral = _agent.getVaultCollateral()
TMP_4786(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
vaultCollateral_1 (-> ['TMP_4786'])(CollateralTypeInt.Data) := TMP_4786(CollateralTypeInt.Data)
 poolCollateral = _agent.getPoolCollateral()
TMP_4787(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getPoolCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
poolCollateral_1 (-> ['TMP_4787'])(CollateralTypeInt.Data) := TMP_4787(CollateralTypeInt.Data)
 ! vaultCollateral.isValid() && poolCollateral.isValid()
TMP_4788(bool) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.isValid(CollateralTypeInt.Data), arguments:["vaultCollateral_1 (-> ['TMP_4786'])"] 
TMP_4789 = UnaryType.BANG TMP_4788 
TMP_4790(bool) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.isValid(CollateralTypeInt.Data), arguments:["poolCollateral_1 (-> ['TMP_4787'])"] 
TMP_4791(bool) = TMP_4789 && TMP_4790
CONDITION TMP_4791
 _c1FactorBIPS = 0
_c1FactorBIPS_2(uint256) := 0(uint256)
 vaultCollateral.isValid() && ! poolCollateral.isValid()
TMP_4792(bool) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.isValid(CollateralTypeInt.Data), arguments:["vaultCollateral_1 (-> ['TMP_4786'])"] 
TMP_4793(bool) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.isValid(CollateralTypeInt.Data), arguments:["poolCollateral_1 (-> ['TMP_4787'])"] 
TMP_4794 = UnaryType.BANG TMP_4793 
TMP_4795(bool) = TMP_4792 && TMP_4794
CONDITION TMP_4795
 _c1FactorBIPS = factorBIPS
_c1FactorBIPS_3(uint256) := factorBIPS_1(uint256)
_c1FactorBIPS_4(uint256) := phi(['_c1FactorBIPS_3', '_c1FactorBIPS_1'])
_c1FactorBIPS_5(uint256) := phi(['_c1FactorBIPS_2', '_c1FactorBIPS_1'])
 _c1FactorBIPS > _vaultCR
TMP_4796(bool) = _c1FactorBIPS_5 > _vaultCR_1
CONDITION TMP_4796
 _c1FactorBIPS = _vaultCR
_c1FactorBIPS_6(uint256) := _vaultCR_1(uint256)
_c1FactorBIPS_7(uint256) := phi(['_c1FactorBIPS_6', '_c1FactorBIPS_1'])
 _poolFactorBIPS = factorBIPS - _c1FactorBIPS
TMP_4797(uint256) = factorBIPS_1 (c)- _c1FactorBIPS_7
_poolFactorBIPS_1(uint256) := TMP_4797(uint256)
 _poolFactorBIPS > _poolCR
TMP_4798(bool) = _poolFactorBIPS_1 > _poolCR_1
CONDITION TMP_4798
 _poolFactorBIPS = _poolCR
_poolFactorBIPS_2(uint256) := _poolCR_1(uint256)
 _c1FactorBIPS = Math.min(factorBIPS - _poolFactorBIPS,_vaultCR)
TMP_4799(uint256) = factorBIPS_1 (c)- _poolFactorBIPS_2
TMP_4800(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['TMP_4799', '_vaultCR_1'] 
_c1FactorBIPS_8(uint256) := TMP_4800(uint256)
_c1FactorBIPS_9(uint256) := phi(['_c1FactorBIPS_8', '_c1FactorBIPS_1'])
_poolFactorBIPS_3(uint256) := phi(['_poolFactorBIPS_2', '_poolFactorBIPS_1'])
 (_c1FactorBIPS,_poolFactorBIPS)
RETURN _c1FactorBIPS_9,_poolFactorBIPS_3
```
#### Redemptions.closeTickets(Agent.State,uint64,bool) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4984(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4984'])(AssetManagerState.State) := TMP_4984(AssetManagerState.State)
 maxRedeemedTickets = Globals.getSettings().maxRedeemedTickets
TMP_4985(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_3521(uint16) -> TMP_4985.maxRedeemedTickets
maxRedeemedTickets_1(uint256) := REF_3521(uint16)
 lotSize = Globals.getSettings().lotSizeAMG
TMP_4986(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_3523(uint64) -> TMP_4986.lotSizeAMG
lotSize_1(uint64) := REF_3523(uint64)
 i = 0
i_1(uint256) := 0(uint256)
 i < maxRedeemedTickets && _closedAMG < _amountAMG
_closedAMG_1(uint64) := phi(['_closedAMG_0', '_closedAMG_2'])
i_2(uint256) := phi(['i_3', 'i_1'])
TMP_4987(bool) = i_2 < maxRedeemedTickets_1
TMP_4988(bool) = _closedAMG_1 < _amountAMG_1
TMP_4989(bool) = TMP_4987 && TMP_4988
CONDITION TMP_4989
 ticketId = state.redemptionQueue.agents[_agent.vaultAddress()].firstTicketId
REF_3524(RedemptionQueue.State) -> state_1 (-> ['TMP_4984']).redemptionQueue
REF_3525(mapping(address => RedemptionQueue.AgentQueue)) -> REF_3524.agents
TMP_4990(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
REF_3527(RedemptionQueue.AgentQueue) -> REF_3525[TMP_4990]
REF_3528(uint64) -> REF_3527.firstTicketId
ticketId_1(uint64) := REF_3528(uint64)
 ticketId == 0
TMP_4991(bool) = ticketId_1 == 0
CONDITION TMP_4991
 ticket = state.redemptionQueue.getTicket(ticketId)
REF_3529(RedemptionQueue.State) -> state_1 (-> ['TMP_4984']).redemptionQueue
TMP_4992(RedemptionQueue.Ticket) = LIBRARY_CALL, dest:RedemptionQueue, function:RedemptionQueue.getTicket(RedemptionQueue.State,uint64), arguments:['REF_3529', 'ticketId_1'] 
ticket_1 (-> ['TMP_4992'])(RedemptionQueue.Ticket) := TMP_4992(RedemptionQueue.Ticket)
 maxTicketRedeemAMG = ticket.valueAMG + _agent.dustAMG
REF_3531(uint64) -> ticket_1 (-> ['TMP_4992']).valueAMG
REF_3532(uint64) -> _agent_1 (-> []).dustAMG
TMP_4993(uint64) = REF_3531 (c)+ REF_3532
maxTicketRedeemAMG_1(uint64) := TMP_4993(uint64)
 maxTicketRedeemAMG -= maxTicketRedeemAMG % lotSize
TMP_4994(uint64) = maxTicketRedeemAMG_1 % lotSize_1
maxTicketRedeemAMG_2(uint64) = maxTicketRedeemAMG_1 (c)- TMP_4994
 ticketRedeemAMG = SafeMath64.min64(_amountAMG - _closedAMG,maxTicketRedeemAMG)
TMP_4995(uint64) = _amountAMG_1 (c)- _closedAMG_1
TMP_4996(uint64) = LIBRARY_CALL, dest:SafeMath64, function:SafeMath64.min64(uint64,uint64), arguments:['TMP_4995', 'maxTicketRedeemAMG_2'] 
ticketRedeemAMG_1(uint64) := TMP_4996(uint64)
 removeFromTicket(ticketId,ticketRedeemAMG)
INTERNAL_CALL, Redemptions.removeFromTicket(uint64,uint64)(ticketId_1,ticketRedeemAMG_1)
 _closedAMG += ticketRedeemAMG
_closedAMG_2(uint64) = _closedAMG_1 (c)+ ticketRedeemAMG_1
 i ++
TMP_4998(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 closeDustAMG = SafeMath64.min64(_amountAMG - _closedAMG,_agent.dustAMG)
TMP_4999(uint64) = _amountAMG_1 (c)- _closedAMG_1
REF_3535(uint64) -> _agent_1 (-> []).dustAMG
TMP_5000(uint64) = LIBRARY_CALL, dest:SafeMath64, function:SafeMath64.min64(uint64,uint64), arguments:['TMP_4999', 'REF_3535'] 
closeDustAMG_1(uint64) := TMP_5000(uint64)
 closeDustAMG > 0
TMP_5001(bool) = closeDustAMG_1 > 0
CONDITION TMP_5001
 _closedAMG += closeDustAMG
_closedAMG_3(uint64) = _closedAMG_1 (c)+ closeDustAMG_1
 AgentBacking.decreaseDust(_agent,closeDustAMG)
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.decreaseDust(Agent.State,uint64), arguments:['_agent_1 (-> [])', 'closeDustAMG_1'] 
_closedAMG_4(uint64) := phi(['_closedAMG_0', '_closedAMG_3'])
 _immediatelyReleaseMinted
CONDITION _immediatelyReleaseMinted_1
 AgentBacking.releaseMintedAssets(_agent,_closedAMG)
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.releaseMintedAssets(Agent.State,uint64), arguments:['_agent_1 (-> [])', '_closedAMG_4'] 
 _closedUBA = Conversion.convertAmgToUBA(_closedAMG)
TMP_5004(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['_closedAMG_4'] 
_closedUBA_1(uint256) := TMP_5004(uint256)
 (_closedAMG,_closedUBA)
RETURN _closedAMG_4,_closedUBA_1
```
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
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
#### Agents.getAgentStatus(Agent.State) [INTERNAL]
```slithir
 status = _agent.status
REF_2978(Agent.Status) -> _agent_1 (-> []).status
status_1(Agent.Status) := REF_2978(Agent.Status)
 status == Agent.Status.NORMAL
REF_2979(Agent.Status) -> Status.NORMAL
TMP_4471(bool) = status_1 == REF_2979
CONDITION TMP_4471
 AgentInfo.Status.NORMAL
REF_2980(AgentInfo.Status) -> Status.NORMAL
RETURN REF_2980
 status == Agent.Status.LIQUIDATION
REF_2981(Agent.Status) -> Status.LIQUIDATION
TMP_4472(bool) = status_1 == REF_2981
CONDITION TMP_4472
 AgentInfo.Status.LIQUIDATION
REF_2982(AgentInfo.Status) -> Status.LIQUIDATION
RETURN REF_2982
 status == Agent.Status.FULL_LIQUIDATION
REF_2983(Agent.Status) -> Status.FULL_LIQUIDATION
TMP_4473(bool) = status_1 == REF_2983
CONDITION TMP_4473
 AgentInfo.Status.FULL_LIQUIDATION
REF_2984(AgentInfo.Status) -> Status.FULL_LIQUIDATION
RETURN REF_2984
 status == Agent.Status.DESTROYING
REF_2985(Agent.Status) -> Status.DESTROYING
TMP_4474(bool) = status_1 == REF_2985
CONDITION TMP_4474
 AgentInfo.Status.DESTROYING
REF_2986(AgentInfo.Status) -> Status.DESTROYING
RETURN REF_2986
 assert(bool)(status == Agent.Status.DESTROYED)
REF_2987(Agent.Status) -> Status.DESTROYED
TMP_4475(bool) = status_1 == REF_2987
TMP_4476(None) = SOLIDITY_CALL assert(bool)(TMP_4475)
 AgentInfo.Status.DESTROYED
REF_2988(AgentInfo.Status) -> Status.DESTROYED
RETURN REF_2988
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
#### Redemptions.burnFAssets(address,uint256) [INTERNAL]
```slithir
 Globals.getFAsset().burn(_owner,_amountUBA)
TMP_5022(IIFAsset) = LIBRARY_CALL, dest:Globals, function:Globals.getFAsset(), arguments:[] 
HIGH_LEVEL_CALL, dest:TMP_5022(IIFAsset), function:burn, arguments:['_owner_1', '_amountUBA_1']
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


#### AgentBacking.releaseMintedAssets(Agent.State,uint64) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_4 (-> [])'])
_valueAMG_1(uint64) := phi(['_valueAMG_1'])
 _agent.mintedAMG = _agent.mintedAMG - _valueAMG
REF_2781(uint64) -> _agent_1 (-> []).mintedAMG
REF_2782(uint64) -> _agent_1 (-> []).mintedAMG
TMP_4267(uint64) = REF_2782 (c)- _valueAMG_1
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2781(uint64) (->_agent_2 (-> [])) := TMP_4267(uint64)
```
#### Redemptions.removeFromTicket(uint64,uint64) [INTERNAL]
```slithir
_redemptionTicketId_1(uint64) := phi(['ticketId_1'])
_redeemedAMG_1(uint64) := phi(['ticketRedeemAMG_1'])
 redemptionQueue = AssetManagerState.get().redemptionQueue
TMP_5005(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
REF_3540(RedemptionQueue.State) -> TMP_5005.redemptionQueue
redemptionQueue_1 (-> ['TMP_5005'])(RedemptionQueue.State) := REF_3540(RedemptionQueue.State)
 ticket = redemptionQueue.getTicket(_redemptionTicketId)
TMP_5006(RedemptionQueue.Ticket) = LIBRARY_CALL, dest:RedemptionQueue, function:RedemptionQueue.getTicket(RedemptionQueue.State,uint64), arguments:["redemptionQueue_1 (-> ['TMP_5005'])", '_redemptionTicketId_1'] 
ticket_1 (-> ['TMP_5006'])(RedemptionQueue.Ticket) := TMP_5006(RedemptionQueue.Ticket)
 agent = Agent.get(ticket.agentVault)
REF_3543(address) -> ticket_1 (-> ['TMP_5006']).agentVault
TMP_5007(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_3543'] 
agent_1 (-> ['TMP_5007'])(Agent.State) := TMP_5007(Agent.State)
 lotSize = Globals.getSettings().lotSizeAMG
TMP_5008(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_3545(uint64) -> TMP_5008.lotSizeAMG
lotSize_1(uint64) := REF_3545(uint64)
 remainingAMG = ticket.valueAMG + agent.dustAMG - _redeemedAMG
REF_3546(uint64) -> ticket_1 (-> ['TMP_5006']).valueAMG
REF_3547(uint64) -> agent_1 (-> ['TMP_5007']).dustAMG
TMP_5009(uint64) = REF_3546 (c)+ REF_3547
TMP_5010(uint64) = TMP_5009 (c)- _redeemedAMG_1
remainingAMG_1(uint64) := TMP_5010(uint64)
 remainingAMGDust = remainingAMG % lotSize
TMP_5011(uint64) = remainingAMG_1 % lotSize_1
remainingAMGDust_1(uint64) := TMP_5011(uint64)
 remainingAMGLots = remainingAMG - remainingAMGDust
TMP_5012(uint64) = remainingAMG_1 (c)- remainingAMGDust_1
remainingAMGLots_1(uint64) := TMP_5012(uint64)
 remainingAMGLots == 0
TMP_5013(bool) = remainingAMGLots_1 == 0
CONDITION TMP_5013
 redemptionQueue.deleteRedemptionTicket(_redemptionTicketId)
LIBRARY_CALL, dest:RedemptionQueue, function:RedemptionQueue.deleteRedemptionTicket(RedemptionQueue.State,uint64), arguments:["redemptionQueue_1 (-> ['TMP_5005'])", '_redemptionTicketId_1'] 
 IAssetManagerEvents.RedemptionTicketDeleted(agent.vaultAddress(),_redemptionTicketId)
TMP_5015(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:["agent_1 (-> ['TMP_5007'])"] 
Emit RedemptionTicketDeleted(TMP_5015,_redemptionTicketId_1)
 remainingAMGLots != ticket.valueAMG
REF_3551(uint64) -> ticket_1 (-> ['TMP_5006']).valueAMG
TMP_5017(bool) = remainingAMGLots_1 != REF_3551
CONDITION TMP_5017
 ticket.valueAMG = remainingAMGLots
REF_3552(uint64) -> ticket_1 (-> ['TMP_5006']).valueAMG
ticket_2 (-> ['TMP_5006'])(RedemptionQueue.Ticket) := phi(["ticket_1 (-> ['TMP_5006'])"])
REF_3552(uint64) (->ticket_2 (-> ['TMP_5006'])) := remainingAMGLots_1(uint64)
TMP_5006(RedemptionQueue.Ticket) := phi(["ticket_2 (-> ['TMP_5006'])"])
 remainingUBA = Conversion.convertAmgToUBA(remainingAMGLots)
TMP_5018(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['remainingAMGLots_1'] 
remainingUBA_1(uint256) := TMP_5018(uint256)
 IAssetManagerEvents.RedemptionTicketUpdated(agent.vaultAddress(),_redemptionTicketId,remainingUBA)
TMP_5019(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:["agent_1 (-> ['TMP_5007'])"] 
Emit RedemptionTicketUpdated(TMP_5019,_redemptionTicketId_1,remainingUBA_1)
 AgentBacking.changeDust(agent,remainingAMGDust)
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.changeDust(Agent.State,uint64), arguments:["agent_1 (-> ['TMP_5007'])", 'remainingAMGDust_1']
```

#### SafeMath64.min64(uint64,uint64) [INTERNAL]
```slithir
 a <= b
TMP_10505(bool) = a_1 <= b_1
CONDITION TMP_10505
 a
RETURN a_1
 b
RETURN b_1
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
#### FAsset.burn(address,uint256) [EXTERNAL]
```slithir
 _burn(_owner,_amount)
INTERNAL_CALL, ERC20._burn(address,uint256)(_owner_1,_amount_1)
 onlyAssetManager()
MODIFIER_CALL, FAsset.onlyAssetManager()()
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
#### RedemptionQueue.deleteRedemptionTicket(RedemptionQueue.State,uint64) [INTERNAL]
```slithir
 ticket = _state.tickets[_ticketId]
REF_3800(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_1 (-> []).tickets
REF_3801(RedemptionQueue.Ticket) -> REF_3800[_ticketId_1]
ticket_1 (-> ['_state'])(RedemptionQueue.Ticket) := REF_3801(RedemptionQueue.Ticket)
 assert(bool)(ticket.agentVault != address(0))
REF_3802(address) -> ticket_1 (-> ['_state']).agentVault
TMP_5410 = CONVERT 0 to address
TMP_5411(bool) = REF_3802 != TMP_5410
TMP_5412(None) = SOLIDITY_CALL assert(bool)(TMP_5411)
 agent = _state.agents[ticket.agentVault]
REF_3803(mapping(address => RedemptionQueue.AgentQueue)) -> _state_1 (-> []).agents
REF_3804(address) -> ticket_1 (-> ['_state']).agentVault
REF_3805(RedemptionQueue.AgentQueue) -> REF_3803[REF_3804]
agent_1 (-> ['_state'])(RedemptionQueue.AgentQueue) := REF_3805(RedemptionQueue.AgentQueue)
 ticket.prev == 0
REF_3806(uint64) -> ticket_1 (-> ['_state']).prev
TMP_5413(bool) = REF_3806 == 0
CONDITION TMP_5413
 assert(bool)(_ticketId == _state.firstTicketId)
REF_3807(uint64) -> _state_1 (-> []).firstTicketId
TMP_5414(bool) = _ticketId_1 == REF_3807
TMP_5415(None) = SOLIDITY_CALL assert(bool)(TMP_5414)
 _state.firstTicketId = ticket.next
REF_3808(uint64) -> _state_1 (-> []).firstTicketId
REF_3809(uint64) -> ticket_1 (-> ['_state']).next
_state_3 (-> [])(RedemptionQueue.State) := phi(['_state_1 (-> [])'])
REF_3808(uint64) (->_state_3 (-> [])) := REF_3809(uint64)
 assert(bool)(_ticketId != _state.firstTicketId)
REF_3810(uint64) -> _state_1 (-> []).firstTicketId
TMP_5416(bool) = _ticketId_1 != REF_3810
TMP_5417(None) = SOLIDITY_CALL assert(bool)(TMP_5416)
 _state.tickets[ticket.prev].next = ticket.next
REF_3811(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_1 (-> []).tickets
REF_3812(uint64) -> ticket_1 (-> ['_state']).prev
REF_3813(RedemptionQueue.Ticket) -> REF_3811[REF_3812]
REF_3814(uint64) -> REF_3813.next
REF_3815(uint64) -> ticket_1 (-> ['_state']).next
_state_2 (-> [])(RedemptionQueue.State) := phi(['_state_1 (-> [])'])
REF_3814(uint64) (->_state_2 (-> [])) := REF_3815(uint64)
_state_4 (-> [])(RedemptionQueue.State) := phi(['_state_3 (-> [])', '_state_2 (-> [])'])
 ticket.next == 0
REF_3816(uint64) -> ticket_1 (-> ['_state']).next
TMP_5418(bool) = REF_3816 == 0
CONDITION TMP_5418
 assert(bool)(_ticketId == _state.lastTicketId)
REF_3817(uint64) -> _state_4 (-> []).lastTicketId
TMP_5419(bool) = _ticketId_1 == REF_3817
TMP_5420(None) = SOLIDITY_CALL assert(bool)(TMP_5419)
 _state.lastTicketId = ticket.prev
REF_3818(uint64) -> _state_4 (-> []).lastTicketId
REF_3819(uint64) -> ticket_1 (-> ['_state']).prev
_state_6 (-> [])(RedemptionQueue.State) := phi(['_state_4 (-> [])'])
REF_3818(uint64) (->_state_6 (-> [])) := REF_3819(uint64)
 assert(bool)(_ticketId != _state.lastTicketId)
REF_3820(uint64) -> _state_4 (-> []).lastTicketId
TMP_5421(bool) = _ticketId_1 != REF_3820
TMP_5422(None) = SOLIDITY_CALL assert(bool)(TMP_5421)
 _state.tickets[ticket.next].prev = ticket.prev
REF_3821(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_4 (-> []).tickets
REF_3822(uint64) -> ticket_1 (-> ['_state']).next
REF_3823(RedemptionQueue.Ticket) -> REF_3821[REF_3822]
REF_3824(uint64) -> REF_3823.prev
REF_3825(uint64) -> ticket_1 (-> ['_state']).prev
_state_5 (-> [])(RedemptionQueue.State) := phi(['_state_4 (-> [])'])
REF_3824(uint64) (->_state_5 (-> [])) := REF_3825(uint64)
_state_7 (-> [])(RedemptionQueue.State) := phi(['_state_5 (-> [])', '_state_6 (-> [])'])
 ticket.prevForAgent == 0
REF_3826(uint64) -> ticket_1 (-> ['_state']).prevForAgent
TMP_5423(bool) = REF_3826 == 0
CONDITION TMP_5423
 assert(bool)(_ticketId == agent.firstTicketId)
REF_3827(uint64) -> agent_1 (-> ['_state']).firstTicketId
TMP_5424(bool) = _ticketId_1 == REF_3827
TMP_5425(None) = SOLIDITY_CALL assert(bool)(TMP_5424)
 agent.firstTicketId = ticket.nextForAgent
REF_3828(uint64) -> agent_1 (-> ['_state']).firstTicketId
REF_3829(uint64) -> ticket_1 (-> ['_state']).nextForAgent
agent_2 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_1 (-> ['_state'])"])
REF_3828(uint64) (->agent_2 (-> ['_state'])) := REF_3829(uint64)
_state_12 (-> ['_state'])(RedemptionQueue.State) := phi(["agent_2 (-> ['_state'])"])
 assert(bool)(_ticketId != agent.firstTicketId)
REF_3830(uint64) -> agent_1 (-> ['_state']).firstTicketId
TMP_5426(bool) = _ticketId_1 != REF_3830
TMP_5427(None) = SOLIDITY_CALL assert(bool)(TMP_5426)
 _state.tickets[ticket.prevForAgent].nextForAgent = ticket.nextForAgent
REF_3831(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_7 (-> []).tickets
REF_3832(uint64) -> ticket_1 (-> ['_state']).prevForAgent
REF_3833(RedemptionQueue.Ticket) -> REF_3831[REF_3832]
REF_3834(uint64) -> REF_3833.nextForAgent
REF_3835(uint64) -> ticket_1 (-> ['_state']).nextForAgent
_state_8 (-> [])(RedemptionQueue.State) := phi(['_state_7 (-> [])'])
REF_3834(uint64) (->_state_8 (-> [])) := REF_3835(uint64)
_state_9 (-> [])(RedemptionQueue.State) := phi(['_state_8 (-> [])', '_state_1 (-> [])'])
agent_3 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_1 (-> ['_state'])", "agent_2 (-> ['_state'])"])
 ticket.nextForAgent == 0
REF_3836(uint64) -> ticket_1 (-> ['_state']).nextForAgent
TMP_5428(bool) = REF_3836 == 0
CONDITION TMP_5428
 assert(bool)(_ticketId == agent.lastTicketId)
REF_3837(uint64) -> agent_3 (-> ['_state']).lastTicketId
TMP_5429(bool) = _ticketId_1 == REF_3837
TMP_5430(None) = SOLIDITY_CALL assert(bool)(TMP_5429)
 agent.lastTicketId = ticket.prevForAgent
REF_3838(uint64) -> agent_3 (-> ['_state']).lastTicketId
REF_3839(uint64) -> ticket_1 (-> ['_state']).prevForAgent
agent_4 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_3 (-> ['_state'])"])
REF_3838(uint64) (->agent_4 (-> ['_state'])) := REF_3839(uint64)
_state_13 (-> ['_state'])(RedemptionQueue.State) := phi(["agent_4 (-> ['_state'])"])
 assert(bool)(_ticketId != agent.lastTicketId)
REF_3840(uint64) -> agent_3 (-> ['_state']).lastTicketId
TMP_5431(bool) = _ticketId_1 != REF_3840
TMP_5432(None) = SOLIDITY_CALL assert(bool)(TMP_5431)
 _state.tickets[ticket.nextForAgent].prevForAgent = ticket.prevForAgent
REF_3841(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_9 (-> []).tickets
REF_3842(uint64) -> ticket_1 (-> ['_state']).nextForAgent
REF_3843(RedemptionQueue.Ticket) -> REF_3841[REF_3842]
REF_3844(uint64) -> REF_3843.prevForAgent
REF_3845(uint64) -> ticket_1 (-> ['_state']).prevForAgent
_state_10 (-> [])(RedemptionQueue.State) := phi(['_state_9 (-> [])'])
REF_3844(uint64) (->_state_10 (-> [])) := REF_3845(uint64)
_state_11 (-> [])(RedemptionQueue.State) := phi(['_state_10 (-> [])', '_state_1 (-> [])'])
 delete _state.tickets[_ticketId]
REF_3846(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_11 (-> []).tickets
REF_3847(RedemptionQueue.Ticket) -> REF_3846[_ticketId_1]
REF_3846 = delete REF_3847
```
#### CollateralPool._deleteFAssetFeeDebt(address,uint256) [INTERNAL]
```slithir
_account_1(address) := phi(['agentVault_22', 'msg.sender'])
_fAssets_1(uint256) := phi(['debtFAssetFeeShare_1', '_fAssets_1', 'debtFAssetFeeShare_1', 'debtFAssetFeeShare_1'])
_fAssetFeeDebtOf_8(mapping(address => int256)) := phi(['_fAssetFeeDebtOf_5', '_fAssetFeeDebtOf_0', '_fAssetFeeDebtOf_2', '_fAssetFeeDebtOf_7', '_fAssetFeeDebtOf_4', '_fAssetFeeDebtOf_9', '_fAssetFeeDebtOf_10'])
totalFAssetFeeDebt_4(int256) := phi(['totalFAssetFeeDebt_0', 'totalFAssetFeeDebt_5', 'totalFAssetFeeDebt_3'])
 _fAssets == 0
TMP_6299(bool) = _fAssets_1 == 0
CONDITION TMP_6299
 fAssets = _fAssets.toInt256()
TMP_6300(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['_fAssets_1'] 
fAssets_1(int256) := TMP_6300(int256)
 _fAssetFeeDebtOf[_account] -= fAssets
REF_4251(int256) -> _fAssetFeeDebtOf_8[_account_1]
_fAssetFeeDebtOf_9(mapping(address => int256)) := phi(['_fAssetFeeDebtOf_8'])
REF_4251(-> _fAssetFeeDebtOf_9) = REF_4251 (c)- fAssets_1
 totalFAssetFeeDebt -= fAssets
totalFAssetFeeDebt_5(int256) = totalFAssetFeeDebt_4 (c)- fAssets_1
 CPFeeDebtChanged(_account,_fAssetFeeDebtOf[_account])
REF_4252(int256) -> _fAssetFeeDebtOf_9[_account_1]
Emit CPFeeDebtChanged(_account_1,REF_4252)
```
#### CollateralPool._tokensToVirtualFeeShare(uint256) [INTERNAL]
```slithir
_tokens_1(uint256) := phi(['slashedTokens_1', '_tokenShare_1', 'tokens_1', '_tokenShare_1'])
token_50(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 _tokens == 0
TMP_6224(bool) = _tokens_1 == 0
CONDITION TMP_6224
 0
RETURN 0
 totalPoolTokens = token.totalSupply()
TMP_6225(uint256) = HIGH_LEVEL_CALL, dest:token_50(IICollateralPoolToken), function:totalSupply, arguments:[]  
token_51(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_50', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalPoolTokens_1(uint256) := TMP_6225(uint256)
 assert(bool)(_tokens <= totalPoolTokens)
TMP_6226(bool) = _tokens_1 <= totalPoolTokens_1
TMP_6227(None) = SOLIDITY_CALL assert(bool)(TMP_6226)
 _totalVirtualFees().mulDiv(_tokens,totalPoolTokens)
TMP_6228(uint256) = INTERNAL_CALL, CollateralPool._totalVirtualFees()()
TMP_6229(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['TMP_6228', '_tokens_1', 'totalPoolTokens_1'] 
RETURN TMP_6229
```
#### CollateralPool._transferWNatTo(address,uint256) [INTERNAL]
```slithir
_to_1(address) := phi(['_recipient_1'])
_amount_1(uint256) := phi(['_amount_1'])
wNat_4(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_32(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 _amount > 0
TMP_6307(bool) = _amount_1 > 0
CONDITION TMP_6307
 totalCollateral -= _amount
totalCollateral_33(uint256) = totalCollateral_32 (c)- _amount_1
 wNat.safeTransfer(_to,_amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['wNat_4', '_to_1', '_amount_1']
```
#### CollateralPoolToken.burn(address,uint256,bool) [EXTERNAL]
```slithir
 _ignoreTimelocked
CONDITION _ignoreTimelocked_1
 ignoreTimelocked = true
ignoreTimelocked_1(bool) := True(bool)
 _burn(_account,_amount)
INTERNAL_CALL, ERC20._burn(address,uint256)(_account_1,_amount_1)
 _ignoreTimelocked
CONDITION _ignoreTimelocked_1
 ignoreTimelocked = false
ignoreTimelocked_2(bool) := False(bool)
 onlyCollateralPool()
MODIFIER_CALL, CollateralPoolToken.onlyCollateralPool()()
```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeWithSelector(token.transfer.selector,to,value))
REF_86(bytes4) (->None) := 2835717307(bytes4)
TMP_243(bytes) = SOLIDITY_CALL abi.encodeWithSelector()(REF_86,to_1,value_1)
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_243)
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
#### ERC20._burn(address,uint256) [INTERNAL]
```slithir
_balances_9(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
_totalSupply_5(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 require(bool,string)(account != address(0),ERC20: burn from the zero address)
TMP_216 = CONVERT 0 to address
TMP_217(bool) = account_1 != TMP_216
TMP_218(None) = SOLIDITY_CALL require(bool,string)(TMP_217,ERC20: burn from the zero address)
 _beforeTokenTransfer(account,address(0),amount)
TMP_219 = CONVERT 0 to address
INTERNAL_CALL, ERC20._beforeTokenTransfer(address,address,uint256)(account_1,TMP_219,amount_1)
 accountBalance = _balances[account]
REF_80(uint256) -> _balances_10[account_1]
accountBalance_1(uint256) := REF_80(uint256)
 require(bool,string)(accountBalance >= amount,ERC20: burn amount exceeds balance)
TMP_221(bool) = accountBalance_1 >= amount_1
TMP_222(None) = SOLIDITY_CALL require(bool,string)(TMP_221,ERC20: burn amount exceeds balance)
 _balances[account] = accountBalance - amount
REF_81(uint256) -> _balances_10[account_1]
TMP_223(uint256) = accountBalance_1 - amount_1
_balances_11(mapping(address => uint256)) := phi(['_balances_10'])
REF_81(uint256) (->_balances_11) := TMP_223(uint256)
 _totalSupply -= amount
_totalSupply_7(uint256) = _totalSupply_6 - amount_1
 Transfer(account,address(0),amount)
TMP_224 = CONVERT 0 to address
Emit Transfer(account_1,TMP_224,amount_1)
 _afterTokenTransfer(account,address(0),amount)
TMP_226 = CONVERT 0 to address
INTERNAL_CALL, ERC20._afterTokenTransfer(address,address,uint256)(account_1,TMP_226,amount_1)
```
