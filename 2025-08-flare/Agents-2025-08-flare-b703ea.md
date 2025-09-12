


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
#### Agents.convertUSD5ToVaultCollateralWei(Agent.State,uint256) [INTERNAL]
```slithir
 Conversion.convertFromUSD5(_amountUSD5,getVaultCollateral(_agent))
TMP_4511(CollateralTypeInt.Data) = INTERNAL_CALL, Agents.getVaultCollateral(Agent.State)(_agent_1 (-> []))
TMP_4512(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertFromUSD5(uint256,CollateralTypeInt.Data), arguments:['_amountUSD5_1', 'TMP_4511'] 
RETURN TMP_4512
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
#### Agents.getAllAgents(uint256,uint256) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4462(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4462'])(AssetManagerState.State) := TMP_4462(AssetManagerState.State)
 _totalLength = state.allAgents.length
REF_2971(address[]) -> state_1 (-> ['TMP_4462']).allAgents
REF_2972 -> LENGTH REF_2971
_totalLength_1(uint256) := REF_2972(uint256)
 _end = Math.min(_end,_totalLength)
TMP_4463(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_end_1', '_totalLength_1'] 
_end_2(uint256) := TMP_4463(uint256)
 _start = Math.min(_start,_end)
TMP_4464(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_start_1', '_end_2'] 
_start_2(uint256) := TMP_4464(uint256)
 _agents = new address[](_end - _start)
TMP_4466(uint256) = _end_2 (c)- _start_2
TMP_4467(address[])  = new address[](TMP_4466)
_agents_1(address[]) = ['TMP_4467(address[])']
 i = _start
i_1(uint256) := _start_2(uint256)
 i < _end
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_4468(bool) = i_2 < _end_2
CONDITION TMP_4468
 _agents[i - _start] = state.allAgents[i]
TMP_4469(uint256) = i_2 (c)- _start_2
REF_2975(address) -> _agents_1[TMP_4469]
REF_2976(address[]) -> state_1 (-> ['TMP_4462']).allAgents
REF_2977(address) -> REF_2976[i_2]
_agents_2(address[]) := phi(['_agents_1'])
REF_2975(address) (->_agents_2) := REF_2977(address)
 i ++
TMP_4470(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 (_agents,_totalLength)
RETURN _agents_1,_totalLength_1
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
#### Agents.getWorkAddress(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
 Globals.getAgentOwnerRegistry().getWorkAddress(_agent.ownerManagementAddress)
TMP_4481(IAgentOwnerRegistry) = LIBRARY_CALL, dest:Globals, function:Globals.getAgentOwnerRegistry(), arguments:[] 
REF_2992(address) -> _agent_1 (-> []).ownerManagementAddress
TMP_4482(address) = HIGH_LEVEL_CALL, dest:TMP_4481(IAgentOwnerRegistry), function:getWorkAddress, arguments:['REF_2992']  
RETURN TMP_4482
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
#### Agents.requireAgentVaultOwner(Agent.State) [INTERNAL]
```slithir
 require(bool,error)(isOwner(_agent,msg.sender),revert OnlyAgentVaultOwner()())
TMP_4497(bool) = INTERNAL_CALL, Agents.isOwner(Agent.State,address)(_agent_1 (-> []),msg.sender)
TMP_4498(None) = SOLIDITY_CALL revert OnlyAgentVaultOwner()()
TMP_4499(None) = SOLIDITY_CALL require(bool,error)(TMP_4497,TMP_4498)
```
#### Agents.requireCollateralPool(Agent.State) [INTERNAL]
```slithir
 require(bool,error)(msg.sender == address(_agent.collateralPool),revert OnlyCollateralPool()())
REF_2998(IICollateralPool) -> _agent_1 (-> []).collateralPool
TMP_4500 = CONVERT REF_2998 to address
TMP_4501(bool) = msg.sender == TMP_4500
TMP_4502(None) = SOLIDITY_CALL revert OnlyCollateralPool()()
TMP_4503(None) = SOLIDITY_CALL require(bool,error)(TMP_4501,TMP_4502)
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
#### Agents.requireWhitelistedAgentVaultOwner(Agent.State) [INTERNAL]
```slithir
 requireWhitelisted(_agent.ownerManagementAddress)
REF_2996(address) -> _agent_1 (-> []).ownerManagementAddress
INTERNAL_CALL, Agents.requireWhitelisted(address)(REF_2996)
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
#### AgentOwnerRegistry.isWhitelisted(address) [PUBLIC]
```slithir
_address_1(address) := phi(['msg.sender'])
whitelist_1(mapping(address => bool)) := phi(['whitelist_2', 'whitelist_1', 'whitelist_4', 'whitelist_5', 'whitelist_0', 'whitelist_3'])
 whitelist[_address]
REF_313(bool) -> whitelist_1[_address_1]
RETURN REF_313
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
#### IPriceReader.getPrice(string) [EXTERNAL]
```slithir

```
#### IPriceReader.getPriceFromTrustedProviders(string) [EXTERNAL]
```slithir

```
