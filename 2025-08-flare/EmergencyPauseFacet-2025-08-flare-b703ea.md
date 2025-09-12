





#### EmergencyPauseFacet.emergencyPause(bool,uint256) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_2803(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2803'])(AssetManagerState.State) := TMP_2803(AssetManagerState.State)
 pausedAtStart = _paused()
TMP_2804(bool) = INTERNAL_CALL, EmergencyPauseFacet._paused()()
pausedAtStart_1(bool) := TMP_2804(bool)
 _byGovernance
CONDITION _byGovernance_1
 state.emergencyPausedUntil = (block.timestamp + _duration).toUint64()
REF_1603(uint64) -> state_1 (-> ['TMP_2803']).emergencyPausedUntil
TMP_2805(uint256) = block.timestamp (c)+ _duration_1
TMP_2806(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_2805'] 
state_2 (-> ['TMP_2803'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_2803'])"])
REF_1603(uint64) (->state_2 (-> ['TMP_2803'])) := TMP_2806(uint64)
TMP_2803(AssetManagerState.State) := phi(["state_2 (-> ['TMP_2803'])"])
 state.emergencyPausedByGovernance = true
REF_1605(bool) -> state_2 (-> ['TMP_2803']).emergencyPausedByGovernance
state_3 (-> ['TMP_2803'])(AssetManagerState.State) := phi(["state_2 (-> ['TMP_2803'])"])
REF_1605(bool) (->state_3 (-> ['TMP_2803'])) := True(bool)
TMP_2803(AssetManagerState.State) := phi(["state_3 (-> ['TMP_2803'])"])
 pausedAtStart && state.emergencyPausedByGovernance
REF_1606(bool) -> state_1 (-> ['TMP_2803']).emergencyPausedByGovernance
TMP_2807(bool) = pausedAtStart_1 && REF_1606
CONDITION TMP_2807
 revert PausedByGovernance()()
TMP_2808(None) = SOLIDITY_CALL revert PausedByGovernance()()
 settings = Globals.getSettings()
TMP_2809(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_2809'])(AssetManagerSettings.Data) := TMP_2809(AssetManagerSettings.Data)
 state.emergencyPausedUntil + settings.emergencyPauseDurationResetAfterSeconds <= block.timestamp
REF_1608(uint64) -> state_1 (-> ['TMP_2803']).emergencyPausedUntil
REF_1609(uint64) -> settings_1 (-> ['TMP_2809']).emergencyPauseDurationResetAfterSeconds
TMP_2810(uint64) = REF_1608 (c)+ REF_1609
TMP_2811(bool) = TMP_2810 <= block.timestamp
CONDITION TMP_2811
 state.emergencyPausedTotalDuration = 0
REF_1610(uint64) -> state_1 (-> ['TMP_2803']).emergencyPausedTotalDuration
state_4 (-> ['TMP_2803'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_2803'])"])
REF_1610(uint64) (->state_4 (-> ['TMP_2803'])) := 0(uint256)
TMP_2803(AssetManagerState.State) := phi(["state_4 (-> ['TMP_2803'])"])
state_5 (-> ['TMP_2803'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_2803'])", "state_4 (-> ['TMP_2803'])"])
 currentPauseEndTime = Math.max(state.emergencyPausedUntil,block.timestamp)
REF_1612(uint64) -> state_5 (-> ['TMP_2803']).emergencyPausedUntil
TMP_2812(uint256) = LIBRARY_CALL, dest:Math, function:Math.max(uint256,uint256), arguments:['REF_1612', 'block.timestamp'] 
currentPauseEndTime_1(uint256) := TMP_2812(uint256)
 projectedStartTime = Math.min(currentPauseEndTime - state.emergencyPausedTotalDuration,block.timestamp)
REF_1614(uint64) -> state_5 (-> ['TMP_2803']).emergencyPausedTotalDuration
TMP_2813(uint256) = currentPauseEndTime_1 (c)- REF_1614
TMP_2814(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['TMP_2813', 'block.timestamp'] 
projectedStartTime_1(uint256) := TMP_2814(uint256)
 maxEndTime = projectedStartTime + settings.maxEmergencyPauseDurationSeconds
REF_1615(uint64) -> settings_1 (-> ['TMP_2809']).maxEmergencyPauseDurationSeconds
TMP_2815(uint256) = projectedStartTime_1 (c)+ REF_1615
maxEndTime_1(uint256) := TMP_2815(uint256)
 endTime = Math.min(block.timestamp + _duration,maxEndTime)
TMP_2816(uint256) = block.timestamp (c)+ _duration_1
TMP_2817(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['TMP_2816', 'maxEndTime_1'] 
endTime_1(uint256) := TMP_2817(uint256)
 state.emergencyPausedUntil = endTime.toUint64()
REF_1617(uint64) -> state_5 (-> ['TMP_2803']).emergencyPausedUntil
TMP_2818(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['endTime_1'] 
state_6 (-> ['TMP_2803'])(AssetManagerState.State) := phi(["state_5 (-> ['TMP_2803'])"])
REF_1617(uint64) (->state_6 (-> ['TMP_2803'])) := TMP_2818(uint64)
TMP_2803(AssetManagerState.State) := phi(["state_6 (-> ['TMP_2803'])"])
 state.emergencyPausedTotalDuration = (endTime - projectedStartTime).toUint64()
REF_1619(uint64) -> state_6 (-> ['TMP_2803']).emergencyPausedTotalDuration
TMP_2819(uint256) = endTime_1 (c)- projectedStartTime_1
TMP_2820(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_2819'] 
state_7 (-> ['TMP_2803'])(AssetManagerState.State) := phi(["state_6 (-> ['TMP_2803'])"])
REF_1619(uint64) (->state_7 (-> ['TMP_2803'])) := TMP_2820(uint64)
TMP_2803(AssetManagerState.State) := phi(["state_7 (-> ['TMP_2803'])"])
 state.emergencyPausedByGovernance = false
REF_1621(bool) -> state_7 (-> ['TMP_2803']).emergencyPausedByGovernance
state_8 (-> ['TMP_2803'])(AssetManagerState.State) := phi(["state_7 (-> ['TMP_2803'])"])
REF_1621(bool) (->state_8 (-> ['TMP_2803'])) := False(bool)
TMP_2803(AssetManagerState.State) := phi(["state_8 (-> ['TMP_2803'])"])
state_9 (-> ['TMP_2803'])(AssetManagerState.State) := phi(["state_3 (-> ['TMP_2803'])", "state_8 (-> ['TMP_2803'])", "state_1 (-> ['TMP_2803'])"])
 _paused()
TMP_2821(bool) = INTERNAL_CALL, EmergencyPauseFacet._paused()()
CONDITION TMP_2821
 EmergencyPauseTriggered(state.emergencyPausedUntil)
REF_1622(uint64) -> state_9 (-> ['TMP_2803']).emergencyPausedUntil
Emit EmergencyPauseTriggered(REF_1622)
 pausedAtStart
CONDITION pausedAtStart_1
 EmergencyPauseCanceled()
Emit EmergencyPauseCanceled()
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
```
#### EmergencyPauseFacet.emergencyPauseDetails() [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_2830(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2830'])(AssetManagerState.State) := TMP_2830(AssetManagerState.State)
 (state.emergencyPausedUntil,state.emergencyPausedTotalDuration,state.emergencyPausedByGovernance)
REF_1628(uint64) -> state_1 (-> ['TMP_2830']).emergencyPausedUntil
REF_1629(uint64) -> state_1 (-> ['TMP_2830']).emergencyPausedTotalDuration
REF_1630(bool) -> state_1 (-> ['TMP_2830']).emergencyPausedByGovernance
RETURN REF_1628,REF_1629,REF_1630
 (_pausedUntil,_totalPauseDuration,_pausedByGovernance)
```
#### EmergencyPauseFacet.emergencyPaused() [EXTERNAL]
```slithir
 _paused()
TMP_2827(bool) = INTERNAL_CALL, EmergencyPauseFacet._paused()()
RETURN TMP_2827
```
#### EmergencyPauseFacet.emergencyPausedUntil() [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_2828(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2828'])(AssetManagerState.State) := TMP_2828(AssetManagerState.State)
 _paused()
TMP_2829(bool) = INTERNAL_CALL, EmergencyPauseFacet._paused()()
CONDITION TMP_2829
 state.emergencyPausedUntil
REF_1626(uint64) -> state_1 (-> ['TMP_2828']).emergencyPausedUntil
RETURN REF_1626
 0
RETURN 0
```
#### EmergencyPauseFacet.resetEmergencyPauseTotalDuration() [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_2825(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2825'])(AssetManagerState.State) := TMP_2825(AssetManagerState.State)
 state.emergencyPausedTotalDuration = 0
REF_1624(uint64) -> state_1 (-> ['TMP_2825']).emergencyPausedTotalDuration
state_2 (-> ['TMP_2825'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_2825'])"])
REF_1624(uint64) (->state_2 (-> ['TMP_2825'])) := 0(uint256)
TMP_2825(AssetManagerState.State) := phi(["state_2 (-> ['TMP_2825'])"])
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
