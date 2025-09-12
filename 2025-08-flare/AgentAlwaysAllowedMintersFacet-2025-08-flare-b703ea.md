


#### AgentAlwaysAllowedMintersFacet.addAlwaysAllowedMinterForAgent(address,address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_1385(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1385'])(Agent.State) := TMP_1385(Agent.State)
 agent.alwaysAllowedMinters.add(_minter)
REF_417(EnumerableSet.AddressSet) -> agent_1 (-> ['TMP_1385']).alwaysAllowedMinters
TMP_1386(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.add(EnumerableSet.AddressSet,address), arguments:['REF_417', '_minter_1'] 
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
```

#### AgentAlwaysAllowedMintersFacet.removeAlwaysAllowedMinterForAgent(address,address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_1388(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1388'])(Agent.State) := TMP_1388(Agent.State)
 agent.alwaysAllowedMinters.remove(_minter)
REF_420(EnumerableSet.AddressSet) -> agent_1 (-> ['TMP_1388']).alwaysAllowedMinters
TMP_1389(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.remove(EnumerableSet.AddressSet,address), arguments:['REF_420', '_minter_1'] 
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
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
#### EnumerableSet.add(EnumerableSet.UintSet,uint256) [INTERNAL]
```slithir
 _add(set._inner,bytes32(value))
REF_232(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_940 = CONVERT value_1 to bytes32
TMP_941(bool) = INTERNAL_CALL, EnumerableSet._add(EnumerableSet.Set,bytes32)(REF_232,TMP_940)
RETURN TMP_941
```
#### EnumerableSet.values(EnumerableSet.AddressSet) [INTERNAL]
```slithir
 store = _values(set._inner)
REF_231(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_939(bytes32[]) = INTERNAL_CALL, EnumerableSet._values(EnumerableSet.Set)(REF_231)
store_1(bytes32[]) = ['TMP_939(bytes32[])']
 result = store
result_1(address[]) := store_1(bytes32[])
 result
RETURN result_1
```
#### EnumerableSet.remove(EnumerableSet.UintSet,uint256) [INTERNAL]
```slithir
 _remove(set._inner,bytes32(value))
REF_233(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_942 = CONVERT value_1 to bytes32
TMP_943(bool) = INTERNAL_CALL, EnumerableSet._remove(EnumerableSet.Set,bytes32)(REF_233,TMP_942)
RETURN TMP_943
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
#### EnumerableSet._add(EnumerableSet.Set,bytes32) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_226', 'REF_220', 'REF_232'])
value_1(bytes32) := phi(['TMP_940', 'value_1', 'TMP_924'])
 ! _contains(set,value)
TMP_904(bool) = INTERNAL_CALL, EnumerableSet._contains(EnumerableSet.Set,bytes32)(set_1 (-> []),value_1)
TMP_905 = UnaryType.BANG TMP_904 
CONDITION TMP_905
 set._values.push(value)
REF_188(bytes32[]) -> set_1 (-> [])._values
REF_190 -> LENGTH REF_188
TMP_907(uint256) := REF_190(uint256)
TMP_908(uint256) = TMP_907 (c)+ 1
set_2 (-> [])(EnumerableSet.Set) := phi(['set_1 (-> [])'])
REF_190(uint256) (->set_3 (-> [])) := TMP_908(uint256)
REF_191(bytes32) -> REF_188[TMP_907]
set_3 (-> [])(EnumerableSet.Set) := phi(['set_2 (-> [])'])
REF_191(bytes32) (->set_3 (-> [])) := value_1(bytes32)
 set._indexes[value] = set._values.length
REF_192(mapping(bytes32 => uint256)) -> set_3 (-> [])._indexes
REF_193(uint256) -> REF_192[value_1]
REF_194(bytes32[]) -> set_3 (-> [])._values
REF_195 -> LENGTH REF_194
set_4 (-> [])(EnumerableSet.Set) := phi(['set_3 (-> [])'])
REF_193(uint256) (->set_4 (-> [])) := REF_195(uint256)
 true
RETURN True
 false
RETURN False
```
#### EnumerableSet._values(EnumerableSet.Set) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_237', 'REF_225', 'REF_231'])
 set._values
REF_219(bytes32[]) -> set_1 (-> [])._values
RETURN REF_219
```
#### EnumerableSet._remove(EnumerableSet.Set,bytes32) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_221', 'REF_233', 'REF_227'])
value_1(bytes32) := phi(['TMP_942', 'value_1', 'TMP_928'])
 valueIndex = set._indexes[value]
REF_196(mapping(bytes32 => uint256)) -> set_1 (-> [])._indexes
REF_197(uint256) -> REF_196[value_1]
valueIndex_1(uint256) := REF_197(uint256)
 valueIndex != 0
TMP_909(bool) = valueIndex_1 != 0
CONDITION TMP_909
 toDeleteIndex = valueIndex - 1
TMP_910(uint256) = valueIndex_1 (c)- 1
toDeleteIndex_1(uint256) := TMP_910(uint256)
 lastIndex = set._values.length - 1
REF_198(bytes32[]) -> set_1 (-> [])._values
REF_199 -> LENGTH REF_198
TMP_911(uint256) = REF_199 (c)- 1
lastIndex_1(uint256) := TMP_911(uint256)
 lastIndex != toDeleteIndex
TMP_912(bool) = lastIndex_1 != toDeleteIndex_1
CONDITION TMP_912
 lastValue = set._values[lastIndex]
REF_200(bytes32[]) -> set_1 (-> [])._values
REF_201(bytes32) -> REF_200[lastIndex_1]
lastValue_1(bytes32) := REF_201(bytes32)
 set._values[toDeleteIndex] = lastValue
REF_202(bytes32[]) -> set_1 (-> [])._values
REF_203(bytes32) -> REF_202[toDeleteIndex_1]
set_2 (-> [])(EnumerableSet.Set) := phi(['set_1 (-> [])'])
REF_203(bytes32) (->set_2 (-> [])) := lastValue_1(bytes32)
 set._indexes[lastValue] = valueIndex
REF_204(mapping(bytes32 => uint256)) -> set_2 (-> [])._indexes
REF_205(uint256) -> REF_204[lastValue_1]
set_3 (-> [])(EnumerableSet.Set) := phi(['set_2 (-> [])'])
REF_205(uint256) (->set_3 (-> [])) := valueIndex_1(uint256)
set_4 (-> [])(EnumerableSet.Set) := phi(['set_1 (-> [])', 'set_3 (-> [])'])
 set._values.pop()
REF_206(bytes32[]) -> set_4 (-> [])._values
REF_208 -> LENGTH REF_206
TMP_914(uint256) = REF_208 (c)- 1
REF_209(bytes32) -> REF_206[TMP_914]
REF_206 = delete REF_209 
REF_210 -> LENGTH REF_206
set_5 (-> [])(EnumerableSet.Set) := phi(['set_4 (-> [])'])
REF_210(uint256) (->set_5 (-> [])) := TMP_914(uint256)
 delete set._indexes[value]
REF_211(mapping(bytes32 => uint256)) -> set_5 (-> [])._indexes
REF_212(uint256) -> REF_211[value_1]
REF_211 = delete REF_212 
 true
RETURN True
 false
RETURN False
```
#### EnumerableSet._contains(EnumerableSet.Set,bytes32) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['set_1 (-> [])', 'REF_234', 'REF_228', 'REF_222'])
value_1(bytes32) := phi(['value_1', 'value_1', 'TMP_944', 'TMP_932'])
 set._indexes[value] != 0
REF_213(mapping(bytes32 => uint256)) -> set_1 (-> [])._indexes
REF_214(uint256) -> REF_213[value_1]
TMP_915(bool) = REF_214 != 0
RETURN TMP_915
```
