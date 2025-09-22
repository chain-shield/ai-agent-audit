

#### SettingsUpdater._getUpdaterState() [PRIVATE]
```slithir
UPDATES_STATE_POSITION_1(bytes32) := phi(['UPDATES_STATE_POSITION_0'])
 position = UPDATES_STATE_POSITION
position_1(bytes32) := UPDATES_STATE_POSITION_1(bytes32)
 _state = position
_state_1 (-> ['position'])(SettingsUpdater.UpdaterState) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
```
#### SettingsUpdater.checkEnoughTimeSinceLastUpdate(bytes32) [INTERNAL]
```slithir
_action_1(bytes32) := phi(['msg.sig'])
 _state = _getUpdaterState()
TMP_5196(SettingsUpdater.UpdaterState) = INTERNAL_CALL, SettingsUpdater._getUpdaterState()()
_state_1 (-> ['TMP_5196'])(SettingsUpdater.UpdaterState) := TMP_5196(SettingsUpdater.UpdaterState)
 settings = Globals.getSettings()
TMP_5197(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_5197'])(AssetManagerSettings.Data) := TMP_5197(AssetManagerSettings.Data)
 lastUpdate = _state.lastUpdate[_action]
REF_3651(mapping(bytes32 => uint256)) -> _state_1 (-> ['TMP_5196']).lastUpdate
REF_3652(uint256) -> REF_3651[_action_1]
lastUpdate_1(uint256) := REF_3652(uint256)
 require(bool,error)(lastUpdate == 0 || block.timestamp >= lastUpdate + settings.minUpdateRepeatTimeSeconds,revert TooCloseToPreviousUpdate()())
TMP_5198(bool) = lastUpdate_1 == 0
REF_3653(uint64) -> settings_1 (-> ['TMP_5197']).minUpdateRepeatTimeSeconds
TMP_5199(uint256) = lastUpdate_1 (c)+ REF_3653
TMP_5200(bool) = block.timestamp >= TMP_5199
TMP_5201(bool) = TMP_5198 || TMP_5200
TMP_5202(None) = SOLIDITY_CALL revert TooCloseToPreviousUpdate()()
TMP_5203(None) = SOLIDITY_CALL require(bool,error)(TMP_5201,TMP_5202)
 _state.lastUpdate[_action] = block.timestamp
REF_3654(mapping(bytes32 => uint256)) -> _state_1 (-> ['TMP_5196']).lastUpdate
REF_3655(uint256) -> REF_3654[_action_1]
_state_2 (-> ['TMP_5196'])(SettingsUpdater.UpdaterState) := phi(["_state_1 (-> ['TMP_5196'])"])
REF_3655(uint256) (->_state_2 (-> ['TMP_5196'])) := block.timestamp(uint256)
TMP_5196(SettingsUpdater.UpdaterState) := phi(["_state_2 (-> ['TMP_5196'])"])
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
