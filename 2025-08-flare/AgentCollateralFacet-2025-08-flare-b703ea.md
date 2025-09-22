







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








#### AgentCollateralFacet.announceAgentPoolTokenRedemption(address,uint256) [EXTERNAL]
```slithir
 _announceWithdrawal(Collateral.Kind.AGENT_POOL,_agentVault,_valueNATWei)
REF_435(Collateral.Kind) -> Kind.AGENT_POOL
TMP_1412(uint256) = INTERNAL_CALL, AgentCollateralFacet._announceWithdrawal(Collateral.Kind,address,uint256)(REF_435,_agentVault_1,_valueNATWei_1)
RETURN TMP_1412
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
 _redemptionAllowedAt
```
#### AgentCollateralFacet.announceVaultCollateralWithdrawal(address,uint256) [EXTERNAL]
```slithir
 _announceWithdrawal(Collateral.Kind.VAULT,_agentVault,_valueNATWei)
REF_434(Collateral.Kind) -> Kind.VAULT
TMP_1410(uint256) = INTERNAL_CALL, AgentCollateralFacet._announceWithdrawal(Collateral.Kind,address,uint256)(REF_434,_agentVault_1,_valueNATWei_1)
RETURN TMP_1410
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
 _withdrawalAllowedAt
```
#### AgentCollateralFacet.beforeCollateralWithdrawal(IERC20,uint256) [EXTERNAL]
```slithir
 agent = Agent.get(msg.sender)
TMP_1414(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['msg.sender'] 
agent_1 (-> ['TMP_1414'])(Agent.State) := TMP_1414(Agent.State)
 _token == agent.getVaultCollateralToken()
TMP_1415(IERC20) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateralToken(Agent.State), arguments:["agent_1 (-> ['TMP_1414'])"] 
TMP_1416(bool) = _token_1 == TMP_1415
CONDITION TMP_1416
 kind = Collateral.Kind.VAULT
REF_438(Collateral.Kind) -> Kind.VAULT
kind_2(Collateral.Kind) := REF_438(Collateral.Kind)
 _token == agent.collateralPool.poolToken()
REF_439(IICollateralPool) -> agent_1 (-> ['TMP_1414']).collateralPool
TMP_1417(ICollateralPoolToken) = HIGH_LEVEL_CALL, dest:REF_439(IICollateralPool), function:poolToken, arguments:[]  
TMP_1418(bool) = _token_1 == TMP_1417
CONDITION TMP_1418
 kind = Collateral.Kind.AGENT_POOL
REF_441(Collateral.Kind) -> Kind.AGENT_POOL
kind_1(Collateral.Kind) := REF_441(Collateral.Kind)
kind_3(Collateral.Kind) := phi(['kind_1', 'kind_2', 'kind_0'])
 withdrawal = agent.withdrawalAnnouncement(kind)
TMP_1419(Agent.WithdrawalAnnouncement) = LIBRARY_CALL, dest:Agents, function:Agents.withdrawalAnnouncement(Agent.State,Collateral.Kind), arguments:["agent_1 (-> ['TMP_1414'])", 'kind_3'] 
withdrawal_1 (-> ['TMP_1419'])(Agent.WithdrawalAnnouncement) := TMP_1419(Agent.WithdrawalAnnouncement)
 collateralData = AgentCollateral.singleCollateralData(agent,kind)
TMP_1420(Collateral.Data) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.singleCollateralData(Agent.State,Collateral.Kind), arguments:["agent_1 (-> ['TMP_1414'])", 'kind_3'] 
collateralData_1(Collateral.Data) := TMP_1420(Collateral.Data)
 require(bool,error)(agent.status == Agent.Status.NORMAL || agent.totalBackedAMG() == 0,revert WithdrawalInvalidAgentStatus()())
REF_444(Agent.Status) -> agent_1 (-> ['TMP_1414']).status
REF_445(Agent.Status) -> Status.NORMAL
TMP_1421(bool) = REF_444 == REF_445
TMP_1422(uint64) = LIBRARY_CALL, dest:Agents, function:Agents.totalBackedAMG(Agent.State), arguments:["agent_1 (-> ['TMP_1414'])"] 
TMP_1423(bool) = TMP_1422 == 0
TMP_1424(bool) = TMP_1421 || TMP_1423
TMP_1425(None) = SOLIDITY_CALL revert WithdrawalInvalidAgentStatus()()
TMP_1426(None) = SOLIDITY_CALL require(bool,error)(TMP_1424,TMP_1425)
 require(bool,error)(withdrawal.allowedAt != 0,revert WithdrawalNotAnnounced()())
REF_447(uint64) -> withdrawal_1 (-> ['TMP_1419']).allowedAt
TMP_1427(bool) = REF_447 != 0
TMP_1428(None) = SOLIDITY_CALL revert WithdrawalNotAnnounced()()
TMP_1429(None) = SOLIDITY_CALL require(bool,error)(TMP_1427,TMP_1428)
 require(bool,error)(_amountWei <= withdrawal.amountWei,revert WithdrawalMoreThanAnnounced()())
REF_448(uint128) -> withdrawal_1 (-> ['TMP_1419']).amountWei
TMP_1430(bool) = _amountWei_1 <= REF_448
TMP_1431(None) = SOLIDITY_CALL revert WithdrawalMoreThanAnnounced()()
TMP_1432(None) = SOLIDITY_CALL require(bool,error)(TMP_1430,TMP_1431)
 require(bool,error)(block.timestamp >= withdrawal.allowedAt,revert WithdrawalNotAllowedYet()())
REF_449(uint64) -> withdrawal_1 (-> ['TMP_1419']).allowedAt
TMP_1433(bool) = block.timestamp >= REF_449
TMP_1434(None) = SOLIDITY_CALL revert WithdrawalNotAllowedYet()()
TMP_1435(None) = SOLIDITY_CALL require(bool,error)(TMP_1433,TMP_1434)
 settings = Globals.getSettings()
TMP_1436(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_1436'])(AssetManagerSettings.Data) := TMP_1436(AssetManagerSettings.Data)
 require(bool,error)(block.timestamp <= withdrawal.allowedAt + settings.agentTimelockedOperationWindowSeconds,revert WithdrawalTooLate()())
REF_451(uint64) -> withdrawal_1 (-> ['TMP_1419']).allowedAt
REF_452(uint64) -> settings_1 (-> ['TMP_1436']).agentTimelockedOperationWindowSeconds
TMP_1437(uint64) = REF_451 (c)+ REF_452
TMP_1438(bool) = block.timestamp <= TMP_1437
TMP_1439(None) = SOLIDITY_CALL revert WithdrawalTooLate()()
TMP_1440(None) = SOLIDITY_CALL require(bool,error)(TMP_1438,TMP_1439)
 require(bool,error)(collateralData.lockedCollateralWei(agent) <= collateralData.fullCollateral,revert WithdrawalCRTooLow()())
TMP_1441(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.lockedCollateralWei(Collateral.Data,Agent.State), arguments:['collateralData_1', "agent_1 (-> ['TMP_1414'])"] 
REF_454(uint256) -> collateralData_1.fullCollateral
TMP_1442(bool) = TMP_1441 <= REF_454
TMP_1443(None) = SOLIDITY_CALL revert WithdrawalCRTooLow()()
TMP_1444(None) = SOLIDITY_CALL require(bool,error)(TMP_1442,TMP_1443)
 remaining = withdrawal.amountWei - _amountWei
REF_455(uint128) -> withdrawal_1 (-> ['TMP_1419']).amountWei
TMP_1445(uint128) = REF_455 (c)- _amountWei_1
remaining_1(uint256) := TMP_1445(uint128)
 withdrawal.amountWei = uint128(remaining)
REF_456(uint128) -> withdrawal_1 (-> ['TMP_1419']).amountWei
TMP_1446 = CONVERT remaining_1 to uint128
withdrawal_2 (-> ['TMP_1419'])(Agent.WithdrawalAnnouncement) := phi(["withdrawal_1 (-> ['TMP_1419'])"])
REF_456(uint128) (->withdrawal_2 (-> ['TMP_1419'])) := TMP_1446(uint128)
TMP_1419(Agent.WithdrawalAnnouncement) := phi(["withdrawal_2 (-> ['TMP_1419'])"])
 remaining == 0
TMP_1447(bool) = remaining_1 == 0
CONDITION TMP_1447
 withdrawal.allowedAt = 0
REF_457(uint64) -> withdrawal_2 (-> ['TMP_1419']).allowedAt
withdrawal_3 (-> ['TMP_1419'])(Agent.WithdrawalAnnouncement) := phi(["withdrawal_2 (-> ['TMP_1419'])"])
REF_457(uint64) (->withdrawal_3 (-> ['TMP_1419'])) := 0(uint256)
TMP_1419(Agent.WithdrawalAnnouncement) := phi(["withdrawal_3 (-> ['TMP_1419'])"])
```
#### AgentCollateralFacet.switchVaultCollateral(address,IERC20) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_1457(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1457'])(Agent.State) := TMP_1457(Agent.State)
 currentCollateral = agent.getVaultCollateral()
TMP_1458(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:["agent_1 (-> ['TMP_1457'])"] 
currentCollateral_1 (-> ['TMP_1458'])(CollateralTypeInt.Data) := TMP_1458(CollateralTypeInt.Data)
 require(bool,error)(currentCollateral.validUntil != 0,revert CollateralNotDeprecated()())
REF_464(uint64) -> currentCollateral_1 (-> ['TMP_1458']).validUntil
TMP_1459(bool) = REF_464 != 0
TMP_1460(None) = SOLIDITY_CALL revert CollateralNotDeprecated()()
TMP_1461(None) = SOLIDITY_CALL require(bool,error)(TMP_1459,TMP_1460)
 withdrawal = agent.withdrawalAnnouncement(Collateral.Kind.VAULT)
REF_466(Collateral.Kind) -> Kind.VAULT
TMP_1462(Agent.WithdrawalAnnouncement) = LIBRARY_CALL, dest:Agents, function:Agents.withdrawalAnnouncement(Agent.State,Collateral.Kind), arguments:["agent_1 (-> ['TMP_1457'])", 'REF_466'] 
withdrawal_1 (-> ['TMP_1462'])(Agent.WithdrawalAnnouncement) := TMP_1462(Agent.WithdrawalAnnouncement)
 require(bool,error)(withdrawal.allowedAt == 0,revert CollateralWithdrawalAnnounced()())
REF_467(uint64) -> withdrawal_1 (-> ['TMP_1462']).allowedAt
TMP_1463(bool) = REF_467 == 0
TMP_1464(None) = SOLIDITY_CALL revert CollateralWithdrawalAnnounced()()
TMP_1465(None) = SOLIDITY_CALL require(bool,error)(TMP_1463,TMP_1464)
 agent.setVaultCollateral(_token)
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setVaultCollateral(Agent.State,IERC20), arguments:["agent_1 (-> ['TMP_1457'])", '_token_1'] 
 IAssetManagerEvents.AgentCollateralTypeChanged(_agentVault,uint8(CollateralType.Class.VAULT),address(_token))
REF_470(CollateralType.Class) -> Class.VAULT
TMP_1467 = CONVERT REF_470 to uint8
TMP_1468 = CONVERT _token_1 to address
Emit AgentCollateralTypeChanged(_agentVault_1,TMP_1467,TMP_1468)
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
```
#### AgentCollateralFacet.updateCollateral(address,IERC20) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_1448(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1448'])(Agent.State) := TMP_1448(Agent.State)
 require(bool,error)(msg.sender == _agentVault || msg.sender == address(agent.collateralPool),revert OnlyAgentVaultOrPool()())
TMP_1449(bool) = msg.sender == _agentVault_1
REF_459(IICollateralPool) -> agent_1 (-> ['TMP_1448']).collateralPool
TMP_1450 = CONVERT REF_459 to address
TMP_1451(bool) = msg.sender == TMP_1450
TMP_1452(bool) = TMP_1449 || TMP_1451
TMP_1453(None) = SOLIDITY_CALL revert OnlyAgentVaultOrPool()()
TMP_1454(None) = SOLIDITY_CALL require(bool,error)(TMP_1452,TMP_1453)
 agent.isCollateralToken(_token)
TMP_1455(bool) = LIBRARY_CALL, dest:Agents, function:Agents.isCollateralToken(Agent.State,IERC20), arguments:["agent_1 (-> ['TMP_1448'])", '_token_1'] 
CONDITION TMP_1455
 Liquidation.endLiquidationIfHealthy(agent)
LIBRARY_CALL, dest:Liquidation, function:Liquidation.endLiquidationIfHealthy(Agent.State), arguments:["agent_1 (-> ['TMP_1448'])"]
```
#### AgentCollateralFacet.upgradeWNatContract(address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_1471(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1471'])(Agent.State) := TMP_1471(Agent.State)
 state = AssetManagerState.get()
TMP_1472(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_1472'])(AssetManagerState.State) := TMP_1472(AssetManagerState.State)
 wNat = IWNat(address(state.collateralTokens[state.poolCollateralIndex].token))
REF_473(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_1472']).collateralTokens
REF_474(uint16) -> state_1 (-> ['TMP_1472']).poolCollateralIndex
REF_475(CollateralTypeInt.Data) -> REF_473[REF_474]
REF_476(IERC20) -> REF_475.token
TMP_1473 = CONVERT REF_476 to address
TMP_1474 = CONVERT TMP_1473 to IWNat
wNat_1(IWNat) := TMP_1474(IWNat)
 agent.poolCollateralIndex != state.poolCollateralIndex
REF_477(uint16) -> agent_1 (-> ['TMP_1471']).poolCollateralIndex
REF_478(uint16) -> state_1 (-> ['TMP_1472']).poolCollateralIndex
TMP_1475(bool) = REF_477 != REF_478
CONDITION TMP_1475
 agent.poolCollateralIndex = state.poolCollateralIndex
REF_479(uint16) -> agent_1 (-> ['TMP_1471']).poolCollateralIndex
REF_480(uint16) -> state_1 (-> ['TMP_1472']).poolCollateralIndex
agent_2 (-> ['TMP_1471'])(Agent.State) := phi(["agent_1 (-> ['TMP_1471'])"])
REF_479(uint16) (->agent_2 (-> ['TMP_1471'])) := REF_480(uint16)
TMP_1471(Agent.State) := phi(["agent_2 (-> ['TMP_1471'])"])
 agent.collateralPool.upgradeWNatContract(wNat)
REF_481(IICollateralPool) -> agent_2 (-> ['TMP_1471']).collateralPool
HIGH_LEVEL_CALL, dest:REF_481(IICollateralPool), function:upgradeWNatContract, arguments:['wNat_1']  
 IAssetManagerEvents.AgentCollateralTypeChanged(_agentVault,uint8(CollateralType.Class.POOL),address(wNat))
REF_484(CollateralType.Class) -> Class.POOL
TMP_1477 = CONVERT REF_484 to uint8
TMP_1478 = CONVERT wNat_1 to address
Emit AgentCollateralTypeChanged(_agentVault_1,TMP_1477,TMP_1478)
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
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
#### Agents.getVaultCollateralToken(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 state = AssetManagerState.get()
TMP_4509(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4509'])(AssetManagerState.State) := TMP_4509(AssetManagerState.State)
 state.collateralTokens[_agent.vaultCollateralIndex].token
REF_3000(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4509']).collateralTokens
REF_3001(uint16) -> _agent_1 (-> []).vaultCollateralIndex
REF_3002(CollateralTypeInt.Data) -> REF_3000[REF_3001]
REF_3003(IERC20) -> REF_3002.token
RETURN REF_3003
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
#### Agents.isCollateralToken(Agent.State,IERC20) [INTERNAL]
```slithir
 _token == getPoolWNat(_agent) || _token == getVaultCollateralToken(_agent)
TMP_4504(IWNat) = INTERNAL_CALL, Agents.getPoolWNat(Agent.State)(_agent_1 (-> []))
TMP_4505(bool) = _token_1 == TMP_4504
TMP_4506(IERC20) = INTERNAL_CALL, Agents.getVaultCollateralToken(Agent.State)(_agent_1 (-> []))
TMP_4507(bool) = _token_1 == TMP_4506
TMP_4508(bool) = TMP_4505 || TMP_4507
RETURN TMP_4508
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
#### IICollateralPool.upgradeWNatContract(IWNat) [EXTERNAL]
```slithir

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
#### Agents.getPoolWNat(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 state = AssetManagerState.get()
TMP_4513(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4513'])(AssetManagerState.State) := TMP_4513(AssetManagerState.State)
 IWNat(address(state.collateralTokens[_agent.poolCollateralIndex].token))
REF_3010(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4513']).collateralTokens
REF_3011(uint16) -> _agent_1 (-> []).poolCollateralIndex
REF_3012(CollateralTypeInt.Data) -> REF_3010[REF_3011]
REF_3013(IERC20) -> REF_3012.token
TMP_4514 = CONVERT REF_3013 to address
TMP_4515 = CONVERT TMP_4514 to IWNat
RETURN TMP_4515
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
_balances_1(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_8', '_balances_5', '_balances_0'])
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
