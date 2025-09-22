





#### RedemptionTimeExtensionFacet.constructor() [PUBLIC]
```slithir
 RedemptionTimeExtension.setRedemptionPaymentExtensionSeconds(1)
LIBRARY_CALL, dest:RedemptionTimeExtension, function:RedemptionTimeExtension.setRedemptionPaymentExtensionSeconds(uint256), arguments:['1']
```
#### RedemptionTimeExtensionFacet.initRedemptionTimeExtensionFacet(uint256) [EXTERNAL]
```slithir
 ds = LibDiamond.diamondStorage()
TMP_3545(LibDiamond.DiamondStorage) = LIBRARY_CALL, dest:LibDiamond, function:LibDiamond.diamondStorage(), arguments:[] 
ds_1 (-> ['TMP_3545'])(LibDiamond.DiamondStorage) := TMP_3545(LibDiamond.DiamondStorage)
 require(bool,error)(ds.supportedInterfaces[type()(IERC165).interfaceId],revert DiamondNotInitialized()())
REF_2349(mapping(bytes4 => bool)) -> ds_1 (-> ['TMP_3545']).supportedInterfaces
TMP_3546(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_2350(bytes4) (->None) := 33540519(bytes4)
REF_2351(bool) -> REF_2349[REF_2350]
TMP_3547(None) = SOLIDITY_CALL revert DiamondNotInitialized()()
TMP_3548(None) = SOLIDITY_CALL require(bool,error)(REF_2351,TMP_3547)
 ds.supportedInterfaces[type()(IRedemptionTimeExtension).interfaceId] = true
REF_2352(mapping(bytes4 => bool)) -> ds_1 (-> ['TMP_3545']).supportedInterfaces
TMP_3549(type(IRedemptionTimeExtension)) = SOLIDITY_CALL type()(IRedemptionTimeExtension)
REF_2353(bytes4) (->None) := 1597866665(bytes4)
REF_2354(bool) -> REF_2352[REF_2353]
ds_2 (-> ['TMP_3545'])(LibDiamond.DiamondStorage) := phi(["ds_1 (-> ['TMP_3545'])"])
REF_2354(bool) (->ds_2 (-> ['TMP_3545'])) := True(bool)
TMP_3545(LibDiamond.DiamondStorage) := phi(["ds_2 (-> ['TMP_3545'])"])
 require(bool,error)(RedemptionTimeExtension.redemptionPaymentExtensionSeconds() == 0,revert AlreadyInitialized()())
TMP_3550(uint256) = LIBRARY_CALL, dest:RedemptionTimeExtension, function:RedemptionTimeExtension.redemptionPaymentExtensionSeconds(), arguments:[] 
TMP_3551(bool) = TMP_3550 == 0
TMP_3552(None) = SOLIDITY_CALL revert AlreadyInitialized()()
TMP_3553(None) = SOLIDITY_CALL require(bool,error)(TMP_3551,TMP_3552)
 RedemptionTimeExtension.setRedemptionPaymentExtensionSeconds(_redemptionPaymentExtensionSeconds)
LIBRARY_CALL, dest:RedemptionTimeExtension, function:RedemptionTimeExtension.setRedemptionPaymentExtensionSeconds(uint256), arguments:['_redemptionPaymentExtensionSeconds_1']
```
#### IRedemptionTimeExtension.redemptionPaymentExtensionSeconds() [EXTERNAL]
```slithir

```
#### RedemptionTimeExtensionFacet.setRedemptionPaymentExtensionSeconds(uint256) [EXTERNAL]
```slithir
 SettingsUpdater.checkEnoughTimeSinceLastUpdate()
LIBRARY_CALL, dest:SettingsUpdater, function:SettingsUpdater.checkEnoughTimeSinceLastUpdate(), arguments:[] 
 settings = Globals.getSettings()
TMP_3556(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3556'])(AssetManagerSettings.Data) := TMP_3556(AssetManagerSettings.Data)
 currentValue = RedemptionTimeExtension.redemptionPaymentExtensionSeconds()
TMP_3557(uint256) = LIBRARY_CALL, dest:RedemptionTimeExtension, function:RedemptionTimeExtension.redemptionPaymentExtensionSeconds(), arguments:[] 
currentValue_1(uint256) := TMP_3557(uint256)
 require(bool,error)(_value <= currentValue * 4 + settings.averageBlockTimeMS / 1000,revert IncreaseTooBig()())
TMP_3558(uint256) = currentValue_1 (c)* 4
REF_2360(uint32) -> settings_1 (-> ['TMP_3556']).averageBlockTimeMS
TMP_3559(uint32) = REF_2360 (c)/ 1000
TMP_3560(uint256) = TMP_3558 (c)+ TMP_3559
TMP_3561(bool) = _value_1 <= TMP_3560
TMP_3562(None) = SOLIDITY_CALL revert IncreaseTooBig()()
TMP_3563(None) = SOLIDITY_CALL require(bool,error)(TMP_3561,TMP_3562)
 require(bool,error)(_value >= currentValue / 4,revert DecreaseTooBig()())
TMP_3564(uint256) = currentValue_1 (c)/ 4
TMP_3565(bool) = _value_1 >= TMP_3564
TMP_3566(None) = SOLIDITY_CALL revert DecreaseTooBig()()
TMP_3567(None) = SOLIDITY_CALL require(bool,error)(TMP_3565,TMP_3566)
 require(bool,error)(_value > 0,revert ValueMustBeNonzero()())
TMP_3568(bool) = _value_1 > 0
TMP_3569(None) = SOLIDITY_CALL revert ValueMustBeNonzero()()
TMP_3570(None) = SOLIDITY_CALL require(bool,error)(TMP_3568,TMP_3569)
 RedemptionTimeExtension.setRedemptionPaymentExtensionSeconds(_value)
LIBRARY_CALL, dest:RedemptionTimeExtension, function:RedemptionTimeExtension.setRedemptionPaymentExtensionSeconds(uint256), arguments:['_value_1'] 
 IAssetManagerEvents.SettingChanged(redemptionPaymentExtensionSeconds,_value)
Emit SettingChanged(redemptionPaymentExtensionSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
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
#### RedemptionTimeExtension.redemptionPaymentExtensionSeconds() [INTERNAL]
```slithir
 state = getState()
TMP_5440(RedemptionTimeExtension.State) = INTERNAL_CALL, RedemptionTimeExtension.getState()()
state_1 (-> ['TMP_5440'])(RedemptionTimeExtension.State) := TMP_5440(RedemptionTimeExtension.State)
 state.redemptionPaymentExtensionSeconds
REF_3860(uint64) -> state_1 (-> ['TMP_5440']).redemptionPaymentExtensionSeconds
RETURN REF_3860
```
#### LibDiamond.diamondStorage() [INTERNAL]
```slithir
DIAMOND_STORAGE_POSITION_1(bytes32) := phi(['DIAMOND_STORAGE_POSITION_0'])
 position = DIAMOND_STORAGE_POSITION
position_1(bytes32) := DIAMOND_STORAGE_POSITION_1(bytes32)
 ds = position
ds_1 (-> ['position'])(LibDiamond.DiamondStorage) := position_1(bytes32)
 ds
RETURN ds_1 (-> ['position'])
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
