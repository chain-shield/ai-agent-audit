
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
#### Agent.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 LF_VAULT = 1 << 0
 LF_POOL = 1 << 1
 AGENTS_POSITION = keccak256(bytes)(fasset.AssetManager.Agent)
```
