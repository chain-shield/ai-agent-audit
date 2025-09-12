


#### SystemStateManagementFacet.attachController(bool) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4139(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4139'])(AssetManagerState.State) := TMP_4139(AssetManagerState.State)
 state.attached = attached
REF_2670(bool) -> state_1 (-> ['TMP_4139']).attached
state_2 (-> ['TMP_4139'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_4139'])"])
REF_2670(bool) (->state_2 (-> ['TMP_4139'])) := attached_1(bool)
TMP_4139(AssetManagerState.State) := phi(["state_2 (-> ['TMP_4139'])"])
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
```
#### SystemStateManagementFacet.pauseMinting() [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4141(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4141'])(AssetManagerState.State) := TMP_4141(AssetManagerState.State)
 state.mintingPausedAt == 0
REF_2672(uint64) -> state_1 (-> ['TMP_4141']).mintingPausedAt
TMP_4142(bool) = REF_2672 == 0
CONDITION TMP_4142
 state.mintingPausedAt = block.timestamp.toUint64()
REF_2673(uint64) -> state_1 (-> ['TMP_4141']).mintingPausedAt
TMP_4143(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.timestamp'] 
state_2 (-> ['TMP_4141'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_4141'])"])
REF_2673(uint64) (->state_2 (-> ['TMP_4141'])) := TMP_4143(uint64)
TMP_4141(AssetManagerState.State) := phi(["state_2 (-> ['TMP_4141'])"])
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
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
