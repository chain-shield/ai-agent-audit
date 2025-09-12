


#### RedemptionTimeExtension.extendTimeForRedemption(address) [INTERNAL]
```slithir
 state = getState()
TMP_5433(RedemptionTimeExtension.State) = INTERNAL_CALL, RedemptionTimeExtension.getState()()
state_1 (-> ['TMP_5433'])(RedemptionTimeExtension.State) := TMP_5433(RedemptionTimeExtension.State)
 agentData = state.agents[_agentVault]
REF_3850(mapping(address => RedemptionTimeExtension.AgentTimeExtensionData)) -> state_1 (-> ['TMP_5433']).agents
REF_3851(RedemptionTimeExtension.AgentTimeExtensionData) -> REF_3850[_agentVault_1]
agentData_1 (-> ['state'])(RedemptionTimeExtension.AgentTimeExtensionData) := REF_3851(RedemptionTimeExtension.AgentTimeExtensionData)
 timestamp = block.timestamp.toUint64()
TMP_5434(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.timestamp'] 
timestamp_1(uint64) := TMP_5434(uint64)
 accumulatedTimestamp = agentData.extendedTimestamp + state.redemptionPaymentExtensionSeconds
REF_3853(uint64) -> agentData_1 (-> ['state']).extendedTimestamp
REF_3854(uint64) -> state_1 (-> ['TMP_5433']).redemptionPaymentExtensionSeconds
TMP_5435(uint64) = REF_3853 (c)+ REF_3854
accumulatedTimestamp_1(uint64) := TMP_5435(uint64)
 agentData.extendedTimestamp = SafeMath64.max64(accumulatedTimestamp,timestamp)
REF_3855(uint64) -> agentData_1 (-> ['state']).extendedTimestamp
TMP_5436(uint64) = LIBRARY_CALL, dest:SafeMath64, function:SafeMath64.max64(uint64,uint64), arguments:['accumulatedTimestamp_1', 'timestamp_1'] 
agentData_2 (-> ['state'])(RedemptionTimeExtension.AgentTimeExtensionData) := phi(["agentData_1 (-> ['state'])"])
REF_3855(uint64) (->agentData_2 (-> ['state'])) := TMP_5436(uint64)
state_2 (-> ['state'])(RedemptionTimeExtension.State) := phi(["agentData_2 (-> ['state'])"])
 agentData.extendedTimestamp - timestamp
REF_3857(uint64) -> agentData_2 (-> ['state']).extendedTimestamp
TMP_5437(uint64) = REF_3857 (c)- timestamp_1
RETURN TMP_5437
```
#### RedemptionTimeExtension.getState() [INTERNAL]
```slithir
STATE_POSITION_1(bytes32) := phi(['STATE_POSITION_0'])
 position = STATE_POSITION
position_1(bytes32) := STATE_POSITION_1(bytes32)
 _state = position
_state_1 (-> ['position'])(RedemptionTimeExtension.State) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
```
#### RedemptionTimeExtension.redemptionPaymentExtensionSeconds() [INTERNAL]
```slithir
 state = getState()
TMP_5440(RedemptionTimeExtension.State) = INTERNAL_CALL, RedemptionTimeExtension.getState()()
state_1 (-> ['TMP_5440'])(RedemptionTimeExtension.State) := TMP_5440(RedemptionTimeExtension.State)
 state.redemptionPaymentExtensionSeconds
REF_3860(uint64) -> state_1 (-> ['TMP_5440']).redemptionPaymentExtensionSeconds
RETURN REF_3860
```
#### RedemptionTimeExtension.setRedemptionPaymentExtensionSeconds(uint256) [INTERNAL]
```slithir
 state = getState()
TMP_5438(RedemptionTimeExtension.State) = INTERNAL_CALL, RedemptionTimeExtension.getState()()
state_1 (-> ['TMP_5438'])(RedemptionTimeExtension.State) := TMP_5438(RedemptionTimeExtension.State)
 state.redemptionPaymentExtensionSeconds = _value.toUint64()
REF_3858(uint64) -> state_1 (-> ['TMP_5438']).redemptionPaymentExtensionSeconds
TMP_5439(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
state_2 (-> ['TMP_5438'])(RedemptionTimeExtension.State) := phi(["state_1 (-> ['TMP_5438'])"])
REF_3858(uint64) (->state_2 (-> ['TMP_5438'])) := TMP_5439(uint64)
TMP_5438(RedemptionTimeExtension.State) := phi(["state_2 (-> ['TMP_5438'])"])
```

#### SafeMath64.max64(uint64,uint64) [INTERNAL]
```slithir
 a >= b
TMP_10504(bool) = a_1 >= b_1
CONDITION TMP_10504
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
