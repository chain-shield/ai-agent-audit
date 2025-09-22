









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
#### AvailableAgentsFacet.announceExitAvailableAgentList(address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_2142(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2142'])(Agent.State) := TMP_2142(Agent.State)
 require(bool,error)(agent.availableAgentsPos != 0,revert AgentNotAvailable()())
REF_1060(uint32) -> agent_1 (-> ['TMP_2142']).availableAgentsPos
TMP_2143(bool) = REF_1060 != 0
TMP_2144(None) = SOLIDITY_CALL revert AgentNotAvailable()()
TMP_2145(None) = SOLIDITY_CALL require(bool,error)(TMP_2143,TMP_2144)
 settings = Globals.getSettings()
TMP_2146(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_2146'])(AssetManagerSettings.Data) := TMP_2146(AssetManagerSettings.Data)
 _exitAllowedAt = block.timestamp + settings.agentExitAvailableTimelockSeconds
REF_1062(uint64) -> settings_1 (-> ['TMP_2146']).agentExitAvailableTimelockSeconds
TMP_2147(uint256) = block.timestamp (c)+ REF_1062
_exitAllowedAt_1(uint256) := TMP_2147(uint256)
 agent.exitAvailableAfterTs = _exitAllowedAt.toUint64()
REF_1063(uint64) -> agent_1 (-> ['TMP_2142']).exitAvailableAfterTs
TMP_2148(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_exitAllowedAt_1'] 
agent_2 (-> ['TMP_2142'])(Agent.State) := phi(["agent_1 (-> ['TMP_2142'])"])
REF_1063(uint64) (->agent_2 (-> ['TMP_2142'])) := TMP_2148(uint64)
TMP_2142(Agent.State) := phi(["agent_2 (-> ['TMP_2142'])"])
 IAssetManagerEvents.AvailableAgentExitAnnounced(_agentVault,_exitAllowedAt)
Emit AvailableAgentExitAnnounced(_agentVault_1,_exitAllowedAt_1)
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
 _exitAllowedAt
RETURN _exitAllowedAt_1
```
#### AvailableAgentsFacet.exitAvailableAgentList(address) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_2151(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2151'])(AssetManagerState.State) := TMP_2151(AssetManagerState.State)
 settings = Globals.getSettings()
TMP_2152(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_2152'])(AssetManagerSettings.Data) := TMP_2152(AssetManagerSettings.Data)
 agent = Agent.get(_agentVault)
TMP_2153(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2153'])(Agent.State) := TMP_2153(Agent.State)
 require(bool,error)(agent.availableAgentsPos != 0,revert AgentNotAvailable()())
REF_1069(uint32) -> agent_1 (-> ['TMP_2153']).availableAgentsPos
TMP_2154(bool) = REF_1069 != 0
TMP_2155(None) = SOLIDITY_CALL revert AgentNotAvailable()()
TMP_2156(None) = SOLIDITY_CALL require(bool,error)(TMP_2154,TMP_2155)
 require(bool,error)(agent.exitAvailableAfterTs != 0,revert ExitNotAnnounced()())
REF_1070(uint64) -> agent_1 (-> ['TMP_2153']).exitAvailableAfterTs
TMP_2157(bool) = REF_1070 != 0
TMP_2158(None) = SOLIDITY_CALL revert ExitNotAnnounced()()
TMP_2159(None) = SOLIDITY_CALL require(bool,error)(TMP_2157,TMP_2158)
 require(bool,error)(block.timestamp >= agent.exitAvailableAfterTs,revert ExitTooSoon()())
REF_1071(uint64) -> agent_1 (-> ['TMP_2153']).exitAvailableAfterTs
TMP_2160(bool) = block.timestamp >= REF_1071
TMP_2161(None) = SOLIDITY_CALL revert ExitTooSoon()()
TMP_2162(None) = SOLIDITY_CALL require(bool,error)(TMP_2160,TMP_2161)
 require(bool,error)(block.timestamp <= agent.exitAvailableAfterTs + settings.agentTimelockedOperationWindowSeconds,revert ExitTooLate()())
REF_1072(uint64) -> agent_1 (-> ['TMP_2153']).exitAvailableAfterTs
REF_1073(uint64) -> settings_1 (-> ['TMP_2152']).agentTimelockedOperationWindowSeconds
TMP_2163(uint64) = REF_1072 (c)+ REF_1073
TMP_2164(bool) = block.timestamp <= TMP_2163
TMP_2165(None) = SOLIDITY_CALL revert ExitTooLate()()
TMP_2166(None) = SOLIDITY_CALL require(bool,error)(TMP_2164,TMP_2165)
 ind = agent.availableAgentsPos - 1
REF_1074(uint32) -> agent_1 (-> ['TMP_2153']).availableAgentsPos
TMP_2167(uint32) = REF_1074 (c)- 1
ind_1(uint256) := TMP_2167(uint32)
 ind + 1 < state.availableAgents.length
TMP_2168(uint256) = ind_1 (c)+ 1
REF_1075(address[]) -> state_1 (-> ['TMP_2151']).availableAgents
REF_1076 -> LENGTH REF_1075
TMP_2169(bool) = TMP_2168 < REF_1076
CONDITION TMP_2169
 state.availableAgents[ind] = state.availableAgents[state.availableAgents.length - 1]
REF_1077(address[]) -> state_1 (-> ['TMP_2151']).availableAgents
REF_1078(address) -> REF_1077[ind_1]
REF_1079(address[]) -> state_1 (-> ['TMP_2151']).availableAgents
REF_1080(address[]) -> state_1 (-> ['TMP_2151']).availableAgents
REF_1081 -> LENGTH REF_1080
TMP_2170(uint256) = REF_1081 (c)- 1
REF_1082(address) -> REF_1079[TMP_2170]
state_2 (-> ['TMP_2151'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_2151'])"])
REF_1078(address) (->state_2 (-> ['TMP_2151'])) := REF_1082(address)
TMP_2151(AssetManagerState.State) := phi(["state_2 (-> ['TMP_2151'])"])
 movedAgent = Agent.get(state.availableAgents[ind])
REF_1084(address[]) -> state_2 (-> ['TMP_2151']).availableAgents
REF_1085(address) -> REF_1084[ind_1]
TMP_2171(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_1085'] 
movedAgent_1 (-> ['TMP_2171'])(Agent.State) := TMP_2171(Agent.State)
 movedAgent.availableAgentsPos = uint32(ind + 1)
REF_1086(uint32) -> movedAgent_1 (-> ['TMP_2171']).availableAgentsPos
TMP_2172(uint256) = ind_1 (c)+ 1
TMP_2173 = CONVERT TMP_2172 to uint32
movedAgent_2 (-> ['TMP_2171'])(Agent.State) := phi(["movedAgent_1 (-> ['TMP_2171'])"])
REF_1086(uint32) (->movedAgent_2 (-> ['TMP_2171'])) := TMP_2173(uint32)
TMP_2171(Agent.State) := phi(["movedAgent_2 (-> ['TMP_2171'])"])
state_3 (-> ['TMP_2151'])(AssetManagerState.State) := phi(["state_2 (-> ['TMP_2151'])", "state_1 (-> ['TMP_2151'])"])
 agent.availableAgentsPos = 0
REF_1087(uint32) -> agent_1 (-> ['TMP_2153']).availableAgentsPos
agent_2 (-> ['TMP_2153'])(Agent.State) := phi(["agent_1 (-> ['TMP_2153'])"])
REF_1087(uint32) (->agent_2 (-> ['TMP_2153'])) := 0(uint256)
TMP_2153(Agent.State) := phi(["agent_2 (-> ['TMP_2153'])"])
 state.availableAgents.pop()
REF_1088(address[]) -> state_3 (-> ['TMP_2151']).availableAgents
REF_1090 -> LENGTH REF_1088
TMP_2175(uint256) = REF_1090 (c)- 1
REF_1091(address) -> REF_1088[TMP_2175]
REF_1088 = delete REF_1091 
REF_1092 -> LENGTH REF_1088
state_4 (-> ['TMP_2151'])(AssetManagerState.State) := phi(["state_3 (-> ['TMP_2151'])"])
REF_1092(uint256) (->state_4 (-> ['TMP_2151'])) := TMP_2175(uint256)
TMP_2151(AssetManagerState.State) := phi(["state_4 (-> ['TMP_2151'])"])
 agent.exitAvailableAfterTs = 0
REF_1093(uint64) -> agent_2 (-> ['TMP_2153']).exitAvailableAfterTs
agent_3 (-> ['TMP_2153'])(Agent.State) := phi(["agent_2 (-> ['TMP_2153'])"])
REF_1093(uint64) (->agent_3 (-> ['TMP_2153'])) := 0(uint256)
TMP_2153(Agent.State) := phi(["agent_3 (-> ['TMP_2153'])"])
 IAssetManagerEvents.AvailableAgentExited(_agentVault)
Emit AvailableAgentExited(_agentVault_1)
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
```

#### AvailableAgentsFacet.getAvailableAgentsList(uint256,uint256) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_2178(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2178'])(AssetManagerState.State) := TMP_2178(AssetManagerState.State)
 _totalLength = state.availableAgents.length
REF_1096(address[]) -> state_1 (-> ['TMP_2178']).availableAgents
REF_1097 -> LENGTH REF_1096
_totalLength_1(uint256) := REF_1097(uint256)
 _end = Math.min(_end,_totalLength)
TMP_2179(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_end_1', '_totalLength_1'] 
_end_2(uint256) := TMP_2179(uint256)
 _start = Math.min(_start,_end)
TMP_2180(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_start_1', '_end_2'] 
_start_2(uint256) := TMP_2180(uint256)
 _agents = new address[](_end - _start)
TMP_2182(uint256) = _end_2 (c)- _start_2
TMP_2183(address[])  = new address[](TMP_2182)
_agents_1(address[]) = ['TMP_2183(address[])']
 i = _start
i_1(uint256) := _start_2(uint256)
 i < _end
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_2184(bool) = i_2 < _end_2
CONDITION TMP_2184
 _agents[i - _start] = state.availableAgents[i]
TMP_2185(uint256) = i_2 (c)- _start_2
REF_1100(address) -> _agents_1[TMP_2185]
REF_1101(address[]) -> state_1 (-> ['TMP_2178']).availableAgents
REF_1102(address) -> REF_1101[i_2]
_agents_2(address[]) := phi(['_agents_1'])
REF_1100(address) (->_agents_2) := REF_1102(address)
 i ++
TMP_2186(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 (_agents,_totalLength)
RETURN _agents_1,_totalLength_1
```
#### AvailableAgentsFacet.makeAgentAvailable(address) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_2123(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2123'])(AssetManagerState.State) := TMP_2123(AssetManagerState.State)
 agent = Agent.get(_agentVault)
TMP_2124(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2124'])(Agent.State) := TMP_2124(Agent.State)
 require(bool,error)(agent.status == Agent.Status.NORMAL,revert InvalidAgentStatus()())
REF_1042(Agent.Status) -> agent_1 (-> ['TMP_2124']).status
REF_1043(Agent.Status) -> Status.NORMAL
TMP_2125(bool) = REF_1042 == REF_1043
TMP_2126(None) = SOLIDITY_CALL revert InvalidAgentStatus()()
TMP_2127(None) = SOLIDITY_CALL require(bool,error)(TMP_2125,TMP_2126)
 require(bool,error)(agent.availableAgentsPos == 0,revert AgentAlreadyAvailable()())
REF_1044(uint32) -> agent_1 (-> ['TMP_2124']).availableAgentsPos
TMP_2128(bool) = REF_1044 == 0
TMP_2129(None) = SOLIDITY_CALL revert AgentAlreadyAvailable()()
TMP_2130(None) = SOLIDITY_CALL require(bool,error)(TMP_2128,TMP_2129)
 collateralData = AgentCollateral.combinedData(agent)
TMP_2131(Collateral.CombinedData) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.combinedData(Agent.State), arguments:["agent_1 (-> ['TMP_2124'])"] 
collateralData_1(Collateral.CombinedData) := TMP_2131(Collateral.CombinedData)
 freeCollateralLots = collateralData.freeCollateralLots(agent)
TMP_2132(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.freeCollateralLots(Collateral.CombinedData,Agent.State), arguments:['collateralData_1', "agent_1 (-> ['TMP_2124'])"] 
freeCollateralLots_1(uint256) := TMP_2132(uint256)
 require(bool,error)(freeCollateralLots >= 1,revert NotEnoughFreeCollateral()())
TMP_2133(bool) = freeCollateralLots_1 >= 1
TMP_2134(None) = SOLIDITY_CALL revert NotEnoughFreeCollateral()()
TMP_2135(None) = SOLIDITY_CALL require(bool,error)(TMP_2133,TMP_2134)
 state.availableAgents.push(_agentVault)
REF_1047(address[]) -> state_1 (-> ['TMP_2123']).availableAgents
REF_1049 -> LENGTH REF_1047
TMP_2137(uint256) := REF_1049(uint256)
TMP_2138(uint256) = TMP_2137 (c)+ 1
state_2 (-> ['TMP_2123'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_2123'])"])
REF_1049(uint256) (->state_3 (-> ['TMP_2123'])) := TMP_2138(uint256)
REF_1050(address) -> REF_1047[TMP_2137]
state_3 (-> ['TMP_2123'])(AssetManagerState.State) := phi(["state_2 (-> ['TMP_2123'])"])
REF_1050(address) (->state_3 (-> ['TMP_2123'])) := _agentVault_1(address)
TMP_2123(AssetManagerState.State) := phi(["state_3 (-> ['TMP_2123'])"])
 agent.availableAgentsPos = state.availableAgents.length.toUint32()
REF_1051(uint32) -> agent_1 (-> ['TMP_2124']).availableAgentsPos
REF_1052(address[]) -> state_3 (-> ['TMP_2123']).availableAgents
REF_1053 -> LENGTH REF_1052
TMP_2139(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['REF_1053'] 
agent_2 (-> ['TMP_2124'])(Agent.State) := phi(["agent_1 (-> ['TMP_2124'])"])
REF_1051(uint32) (->agent_2 (-> ['TMP_2124'])) := TMP_2139(uint32)
TMP_2124(Agent.State) := phi(["agent_2 (-> ['TMP_2124'])"])
 IAssetManagerEvents.AgentAvailable(_agentVault,agent.feeBIPS,agent.mintingVaultCollateralRatioBIPS,agent.mintingPoolCollateralRatioBIPS,freeCollateralLots)
REF_1056(uint16) -> agent_2 (-> ['TMP_2124']).feeBIPS
REF_1057(uint32) -> agent_2 (-> ['TMP_2124']).mintingVaultCollateralRatioBIPS
REF_1058(uint32) -> agent_2 (-> ['TMP_2124']).mintingPoolCollateralRatioBIPS
Emit AgentAvailable(_agentVault_1,REF_1056,REF_1057,REF_1058,freeCollateralLots_1)
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
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
