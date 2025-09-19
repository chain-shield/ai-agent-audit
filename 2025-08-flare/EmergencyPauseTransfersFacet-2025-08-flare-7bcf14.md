





#### EmergencyPauseTransfersFacet.emergencyPauseTransfers(bool,uint256) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_2848(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2848'])(AssetManagerState.State) := TMP_2848(AssetManagerState.State)
 pausedAtStart = _transfersPaused()
TMP_2849(bool) = INTERNAL_CALL, EmergencyPauseTransfersFacet._transfersPaused()()
pausedAtStart_1(bool) := TMP_2849(bool)
 _byGovernance
CONDITION _byGovernance_1
 state.transfersEmergencyPausedUntil = (block.timestamp + _duration).toUint64()
REF_1641(uint64) -> state_1 (-> ['TMP_2848']).transfersEmergencyPausedUntil
TMP_2850(uint256) = block.timestamp (c)+ _duration_1
TMP_2851(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_2850'] 
state_2 (-> ['TMP_2848'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_2848'])"])
REF_1641(uint64) (->state_2 (-> ['TMP_2848'])) := TMP_2851(uint64)
TMP_2848(AssetManagerState.State) := phi(["state_2 (-> ['TMP_2848'])"])
 state.transfersEmergencyPausedByGovernance = true
REF_1643(bool) -> state_2 (-> ['TMP_2848']).transfersEmergencyPausedByGovernance
state_3 (-> ['TMP_2848'])(AssetManagerState.State) := phi(["state_2 (-> ['TMP_2848'])"])
REF_1643(bool) (->state_3 (-> ['TMP_2848'])) := True(bool)
TMP_2848(AssetManagerState.State) := phi(["state_3 (-> ['TMP_2848'])"])
 pausedAtStart && state.transfersEmergencyPausedByGovernance
REF_1644(bool) -> state_1 (-> ['TMP_2848']).transfersEmergencyPausedByGovernance
TMP_2852(bool) = pausedAtStart_1 && REF_1644
CONDITION TMP_2852
 revert PausedByGovernance()()
TMP_2853(None) = SOLIDITY_CALL revert PausedByGovernance()()
 settings = Globals.getSettings()
TMP_2854(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_2854'])(AssetManagerSettings.Data) := TMP_2854(AssetManagerSettings.Data)
 resetTs = state.transfersEmergencyPausedUntil + settings.emergencyPauseDurationResetAfterSeconds
REF_1646(uint64) -> state_1 (-> ['TMP_2848']).transfersEmergencyPausedUntil
REF_1647(uint64) -> settings_1 (-> ['TMP_2854']).emergencyPauseDurationResetAfterSeconds
TMP_2855(uint64) = REF_1646 (c)+ REF_1647
resetTs_1(uint256) := TMP_2855(uint64)
 resetTs <= block.timestamp
TMP_2856(bool) = resetTs_1 <= block.timestamp
CONDITION TMP_2856
 state.transfersEmergencyPausedTotalDuration = 0
REF_1648(uint64) -> state_1 (-> ['TMP_2848']).transfersEmergencyPausedTotalDuration
state_4 (-> ['TMP_2848'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_2848'])"])
REF_1648(uint64) (->state_4 (-> ['TMP_2848'])) := 0(uint256)
TMP_2848(AssetManagerState.State) := phi(["state_4 (-> ['TMP_2848'])"])
state_5 (-> ['TMP_2848'])(AssetManagerState.State) := phi(["state_4 (-> ['TMP_2848'])", "state_1 (-> ['TMP_2848'])"])
 currentPauseEndTime = Math.max(state.transfersEmergencyPausedUntil,block.timestamp)
REF_1650(uint64) -> state_5 (-> ['TMP_2848']).transfersEmergencyPausedUntil
TMP_2857(uint256) = LIBRARY_CALL, dest:Math, function:Math.max(uint256,uint256), arguments:['REF_1650', 'block.timestamp'] 
currentPauseEndTime_1(uint256) := TMP_2857(uint256)
 projectedStartTime = Math.min(currentPauseEndTime - state.transfersEmergencyPausedTotalDuration,block.timestamp)
REF_1652(uint64) -> state_5 (-> ['TMP_2848']).transfersEmergencyPausedTotalDuration
TMP_2858(uint256) = currentPauseEndTime_1 (c)- REF_1652
TMP_2859(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['TMP_2858', 'block.timestamp'] 
projectedStartTime_1(uint256) := TMP_2859(uint256)
 maxEndTime = projectedStartTime + settings.maxEmergencyPauseDurationSeconds
REF_1653(uint64) -> settings_1 (-> ['TMP_2854']).maxEmergencyPauseDurationSeconds
TMP_2860(uint256) = projectedStartTime_1 (c)+ REF_1653
maxEndTime_1(uint256) := TMP_2860(uint256)
 endTime = Math.min(block.timestamp + _duration,maxEndTime)
TMP_2861(uint256) = block.timestamp (c)+ _duration_1
TMP_2862(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['TMP_2861', 'maxEndTime_1'] 
endTime_1(uint256) := TMP_2862(uint256)
 state.transfersEmergencyPausedUntil = endTime.toUint64()
REF_1655(uint64) -> state_5 (-> ['TMP_2848']).transfersEmergencyPausedUntil
TMP_2863(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['endTime_1'] 
state_6 (-> ['TMP_2848'])(AssetManagerState.State) := phi(["state_5 (-> ['TMP_2848'])"])
REF_1655(uint64) (->state_6 (-> ['TMP_2848'])) := TMP_2863(uint64)
TMP_2848(AssetManagerState.State) := phi(["state_6 (-> ['TMP_2848'])"])
 state.transfersEmergencyPausedTotalDuration = (endTime - projectedStartTime).toUint64()
REF_1657(uint64) -> state_6 (-> ['TMP_2848']).transfersEmergencyPausedTotalDuration
TMP_2864(uint256) = endTime_1 (c)- projectedStartTime_1
TMP_2865(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_2864'] 
state_7 (-> ['TMP_2848'])(AssetManagerState.State) := phi(["state_6 (-> ['TMP_2848'])"])
REF_1657(uint64) (->state_7 (-> ['TMP_2848'])) := TMP_2865(uint64)
TMP_2848(AssetManagerState.State) := phi(["state_7 (-> ['TMP_2848'])"])
 state.transfersEmergencyPausedByGovernance = false
REF_1659(bool) -> state_7 (-> ['TMP_2848']).transfersEmergencyPausedByGovernance
state_8 (-> ['TMP_2848'])(AssetManagerState.State) := phi(["state_7 (-> ['TMP_2848'])"])
REF_1659(bool) (->state_8 (-> ['TMP_2848'])) := False(bool)
TMP_2848(AssetManagerState.State) := phi(["state_8 (-> ['TMP_2848'])"])
state_9 (-> ['TMP_2848'])(AssetManagerState.State) := phi(["state_8 (-> ['TMP_2848'])", "state_1 (-> ['TMP_2848'])", "state_3 (-> ['TMP_2848'])"])
 _transfersPaused()
TMP_2866(bool) = INTERNAL_CALL, EmergencyPauseTransfersFacet._transfersPaused()()
CONDITION TMP_2866
 EmergencyPauseTransfersTriggered(state.transfersEmergencyPausedUntil)
REF_1660(uint64) -> state_9 (-> ['TMP_2848']).transfersEmergencyPausedUntil
Emit EmergencyPauseTransfersTriggered(REF_1660)
 pausedAtStart
CONDITION pausedAtStart_1
 EmergencyPauseTransfersCanceled()
Emit EmergencyPauseTransfersCanceled()
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
```
#### EmergencyPauseTransfersFacet.emergencyPauseTransfersDetails() [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_2875(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2875'])(AssetManagerState.State) := TMP_2875(AssetManagerState.State)
 (state.transfersEmergencyPausedUntil,state.transfersEmergencyPausedTotalDuration,state.transfersEmergencyPausedByGovernance)
REF_1666(uint64) -> state_1 (-> ['TMP_2875']).transfersEmergencyPausedUntil
REF_1667(uint64) -> state_1 (-> ['TMP_2875']).transfersEmergencyPausedTotalDuration
REF_1668(bool) -> state_1 (-> ['TMP_2875']).transfersEmergencyPausedByGovernance
RETURN REF_1666,REF_1667,REF_1668
 (_pausedUntil,_totalPauseDuration,_pausedByGovernance)
```
#### EmergencyPauseTransfersFacet.resetEmergencyPauseTransfersTotalDuration() [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_2870(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2870'])(AssetManagerState.State) := TMP_2870(AssetManagerState.State)
 state.transfersEmergencyPausedTotalDuration = 0
REF_1662(uint64) -> state_1 (-> ['TMP_2870']).transfersEmergencyPausedTotalDuration
state_2 (-> ['TMP_2870'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_2870'])"])
REF_1662(uint64) (->state_2 (-> ['TMP_2870'])) := 0(uint256)
TMP_2870(AssetManagerState.State) := phi(["state_2 (-> ['TMP_2870'])"])
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
```
#### EmergencyPauseTransfersFacet.transfersEmergencyPaused() [EXTERNAL]
```slithir
 _transfersPaused()
TMP_2872(bool) = INTERNAL_CALL, EmergencyPauseTransfersFacet._transfersPaused()()
RETURN TMP_2872
```
#### EmergencyPauseTransfersFacet.transfersEmergencyPausedUntil() [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_2873(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2873'])(AssetManagerState.State) := TMP_2873(AssetManagerState.State)
 _transfersPaused()
TMP_2874(bool) = INTERNAL_CALL, EmergencyPauseTransfersFacet._transfersPaused()()
CONDITION TMP_2874
 state.transfersEmergencyPausedUntil
REF_1664(uint64) -> state_1 (-> ['TMP_2873']).transfersEmergencyPausedUntil
RETURN REF_1664
 0
RETURN 0
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
