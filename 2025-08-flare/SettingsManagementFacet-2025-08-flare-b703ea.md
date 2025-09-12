









#### SettingsManagementFacet.setAgentExitAvailableTimelockSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3922(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3922'])(AssetManagerSettings.Data) := TMP_3922(AssetManagerSettings.Data)
 require(bool,error)(_value <= settings.agentExitAvailableTimelockSeconds * 4 + 604800,revert ValueTooBig()())
REF_2494(uint64) -> settings_1 (-> ['TMP_3922']).agentExitAvailableTimelockSeconds
TMP_3923(uint64) = REF_2494 (c)* 4
TMP_3924(uint64) = TMP_3923 (c)+ 604800
TMP_3925(bool) = _value_1 <= TMP_3924
TMP_3926(None) = SOLIDITY_CALL revert ValueTooBig()()
TMP_3927(None) = SOLIDITY_CALL require(bool,error)(TMP_3925,TMP_3926)
 settings.agentExitAvailableTimelockSeconds = _value.toUint64()
REF_2495(uint64) -> settings_1 (-> ['TMP_3922']).agentExitAvailableTimelockSeconds
TMP_3928(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3922'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3922'])"])
REF_2495(uint64) (->settings_2 (-> ['TMP_3922'])) := TMP_3928(uint64)
TMP_3922(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3922'])"])
 SettingChanged(agentExitAvailableTimelockSeconds,_value)
Emit SettingChanged(agentExitAvailableTimelockSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setAgentFeeChangeTimelockSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3932(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3932'])(AssetManagerSettings.Data) := TMP_3932(AssetManagerSettings.Data)
 require(bool,error)(_value <= settings.agentFeeChangeTimelockSeconds * 4 + 86400,revert ValueTooBig()())
REF_2498(uint64) -> settings_1 (-> ['TMP_3932']).agentFeeChangeTimelockSeconds
TMP_3933(uint64) = REF_2498 (c)* 4
TMP_3934(uint64) = TMP_3933 (c)+ 86400
TMP_3935(bool) = _value_1 <= TMP_3934
TMP_3936(None) = SOLIDITY_CALL revert ValueTooBig()()
TMP_3937(None) = SOLIDITY_CALL require(bool,error)(TMP_3935,TMP_3936)
 settings.agentFeeChangeTimelockSeconds = _value.toUint64()
REF_2499(uint64) -> settings_1 (-> ['TMP_3932']).agentFeeChangeTimelockSeconds
TMP_3938(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3932'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3932'])"])
REF_2499(uint64) (->settings_2 (-> ['TMP_3932'])) := TMP_3938(uint64)
TMP_3932(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3932'])"])
 SettingChanged(agentFeeChangeTimelockSeconds,_value)
Emit SettingChanged(agentFeeChangeTimelockSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setAgentMintingCRChangeTimelockSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3942(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3942'])(AssetManagerSettings.Data) := TMP_3942(AssetManagerSettings.Data)
 require(bool,error)(_value <= settings.agentMintingCRChangeTimelockSeconds * 4 + 86400,revert ValueTooBig()())
REF_2502(uint64) -> settings_1 (-> ['TMP_3942']).agentMintingCRChangeTimelockSeconds
TMP_3943(uint64) = REF_2502 (c)* 4
TMP_3944(uint64) = TMP_3943 (c)+ 86400
TMP_3945(bool) = _value_1 <= TMP_3944
TMP_3946(None) = SOLIDITY_CALL revert ValueTooBig()()
TMP_3947(None) = SOLIDITY_CALL require(bool,error)(TMP_3945,TMP_3946)
 settings.agentMintingCRChangeTimelockSeconds = _value.toUint64()
REF_2503(uint64) -> settings_1 (-> ['TMP_3942']).agentMintingCRChangeTimelockSeconds
TMP_3948(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3942'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3942'])"])
REF_2503(uint64) (->settings_2 (-> ['TMP_3942'])) := TMP_3948(uint64)
TMP_3942(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3942'])"])
 SettingChanged(agentMintingCRChangeTimelockSeconds,_value)
Emit SettingChanged(agentMintingCRChangeTimelockSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setAgentOwnerRegistry(address) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3601(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3601'])(AssetManagerSettings.Data) := TMP_3601(AssetManagerSettings.Data)
 require(bool,error)(_value != address(0),revert InvalidAddress()())
TMP_3602 = CONVERT 0 to address
TMP_3603(bool) = _value_1 != TMP_3602
TMP_3604(None) = SOLIDITY_CALL revert InvalidAddress()()
TMP_3605(None) = SOLIDITY_CALL require(bool,error)(TMP_3603,TMP_3604)
 settings.agentOwnerRegistry = _value
REF_2381(address) -> settings_1 (-> ['TMP_3601']).agentOwnerRegistry
settings_2 (-> ['TMP_3601'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3601'])"])
REF_2381(address) (->settings_2 (-> ['TMP_3601'])) := _value_1(address)
TMP_3601(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3601'])"])
 ContractChanged(agentOwnerRegistry,_value)
Emit ContractChanged(agentOwnerRegistry,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setAgentTimelockedOperationWindowSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3962(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3962'])(AssetManagerSettings.Data) := TMP_3962(AssetManagerSettings.Data)
 require(bool,error)(_value >= 3600,revert ValueTooSmall()())
TMP_3963(bool) = _value_1 >= 3600
TMP_3964(None) = SOLIDITY_CALL revert ValueTooSmall()()
TMP_3965(None) = SOLIDITY_CALL require(bool,error)(TMP_3963,TMP_3964)
 settings.agentTimelockedOperationWindowSeconds = _value.toUint64()
REF_2510(uint64) -> settings_1 (-> ['TMP_3962']).agentTimelockedOperationWindowSeconds
TMP_3966(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3962'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3962'])"])
REF_2510(uint64) (->settings_2 (-> ['TMP_3962'])) := TMP_3966(uint64)
TMP_3962(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3962'])"])
 SettingChanged(agentTimelockedOperationWindowSeconds,_value)
Emit SettingChanged(agentTimelockedOperationWindowSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setAgentVaultFactory(address) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3609(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3609'])(AssetManagerSettings.Data) := TMP_3609(AssetManagerSettings.Data)
 require(bool,error)(_value != address(0),revert InvalidAddress()())
TMP_3610 = CONVERT 0 to address
TMP_3611(bool) = _value_1 != TMP_3610
TMP_3612(None) = SOLIDITY_CALL revert InvalidAddress()()
TMP_3613(None) = SOLIDITY_CALL require(bool,error)(TMP_3611,TMP_3612)
 settings.agentVaultFactory = _value
REF_2383(address) -> settings_1 (-> ['TMP_3609']).agentVaultFactory
settings_2 (-> ['TMP_3609'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3609'])"])
REF_2383(address) (->settings_2 (-> ['TMP_3609'])) := _value_1(address)
TMP_3609(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3609'])"])
 ContractChanged(agentVaultFactory,_value)
Emit ContractChanged(agentVaultFactory,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setAttestationWindowSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3865(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3865'])(AssetManagerSettings.Data) := TMP_3865(AssetManagerSettings.Data)
 require(bool,error)(_value >= 86400,revert WindowTooSmall()())
TMP_3866(bool) = _value_1 >= 86400
TMP_3867(None) = SOLIDITY_CALL revert WindowTooSmall()()
TMP_3868(None) = SOLIDITY_CALL require(bool,error)(TMP_3866,TMP_3867)
 settings.attestationWindowSeconds = _value.toUint64()
REF_2470(uint64) -> settings_1 (-> ['TMP_3865']).attestationWindowSeconds
TMP_3869(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3865'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3865'])"])
REF_2470(uint64) (->settings_2 (-> ['TMP_3865'])) := TMP_3869(uint64)
TMP_3865(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3865'])"])
 SettingChanged(attestationWindowSeconds,_value)
Emit SettingChanged(attestationWindowSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setAverageBlockTimeMS(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3873(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3873'])(AssetManagerSettings.Data) := TMP_3873(AssetManagerSettings.Data)
 require(bool,error)(_value > 0,revert CannotBeZero()())
TMP_3874(bool) = _value_1 > 0
TMP_3875(None) = SOLIDITY_CALL revert CannotBeZero()()
TMP_3876(None) = SOLIDITY_CALL require(bool,error)(TMP_3874,TMP_3875)
 require(bool,error)(_value <= settings.averageBlockTimeMS * 2,revert IncreaseTooBig()())
REF_2473(uint32) -> settings_1 (-> ['TMP_3873']).averageBlockTimeMS
TMP_3877(uint32) = REF_2473 (c)* 2
TMP_3878(bool) = _value_1 <= TMP_3877
TMP_3879(None) = SOLIDITY_CALL revert IncreaseTooBig()()
TMP_3880(None) = SOLIDITY_CALL require(bool,error)(TMP_3878,TMP_3879)
 require(bool,error)(_value >= settings.averageBlockTimeMS / 2,revert DecreaseTooBig()())
REF_2474(uint32) -> settings_1 (-> ['TMP_3873']).averageBlockTimeMS
TMP_3881(uint32) = REF_2474 (c)/ 2
TMP_3882(bool) = _value_1 >= TMP_3881
TMP_3883(None) = SOLIDITY_CALL revert DecreaseTooBig()()
TMP_3884(None) = SOLIDITY_CALL require(bool,error)(TMP_3882,TMP_3883)
 settings.averageBlockTimeMS = _value.toUint32()
REF_2475(uint32) -> settings_1 (-> ['TMP_3873']).averageBlockTimeMS
TMP_3885(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3873'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3873'])"])
REF_2475(uint32) (->settings_2 (-> ['TMP_3873'])) := TMP_3885(uint32)
TMP_3873(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3873'])"])
 SettingChanged(averageBlockTimeMS,_value)
Emit SettingChanged(averageBlockTimeMS,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setCleanerContract(address) [EXTERNAL]
```slithir
 fAsset = Globals.getFAsset()
TMP_3649(IIFAsset) = LIBRARY_CALL, dest:Globals, function:Globals.getFAsset(), arguments:[] 
fAsset_1(IIFAsset) := TMP_3649(IIFAsset)
 fAsset.setCleanerContract(_value)
HIGH_LEVEL_CALL, dest:fAsset_1(IIFAsset), function:setCleanerContract, arguments:['_value_1']  
 ContractChanged(cleanerContract,_value)
Emit ContractChanged(cleanerContract,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setCleanupBlockNumberManager(address) [EXTERNAL]
```slithir
 fAsset = Globals.getFAsset()
TMP_3654(IIFAsset) = LIBRARY_CALL, dest:Globals, function:Globals.getFAsset(), arguments:[] 
fAsset_1(IIFAsset) := TMP_3654(IIFAsset)
 fAsset.setCleanupBlockNumberManager(_value)
HIGH_LEVEL_CALL, dest:fAsset_1(IIFAsset), function:setCleanupBlockNumberManager, arguments:['_value_1']  
 ContractChanged(cleanupBlockNumberManager,_value)
Emit ContractChanged(cleanupBlockNumberManager,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setCollateralPoolFactory(address) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3617(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3617'])(AssetManagerSettings.Data) := TMP_3617(AssetManagerSettings.Data)
 require(bool,error)(_value != address(0),revert InvalidAddress()())
TMP_3618 = CONVERT 0 to address
TMP_3619(bool) = _value_1 != TMP_3618
TMP_3620(None) = SOLIDITY_CALL revert InvalidAddress()()
TMP_3621(None) = SOLIDITY_CALL require(bool,error)(TMP_3619,TMP_3620)
 settings.collateralPoolFactory = _value
REF_2385(address) -> settings_1 (-> ['TMP_3617']).collateralPoolFactory
settings_2 (-> ['TMP_3617'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3617'])"])
REF_2385(address) (->settings_2 (-> ['TMP_3617'])) := _value_1(address)
TMP_3617(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3617'])"])
 ContractChanged(collateralPoolFactory,_value)
Emit ContractChanged(collateralPoolFactory,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setCollateralPoolTokenFactory(address) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3625(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3625'])(AssetManagerSettings.Data) := TMP_3625(AssetManagerSettings.Data)
 require(bool,error)(_value != address(0),revert InvalidAddress()())
TMP_3626 = CONVERT 0 to address
TMP_3627(bool) = _value_1 != TMP_3626
TMP_3628(None) = SOLIDITY_CALL revert InvalidAddress()()
TMP_3629(None) = SOLIDITY_CALL require(bool,error)(TMP_3627,TMP_3628)
 settings.collateralPoolTokenFactory = _value
REF_2387(address) -> settings_1 (-> ['TMP_3625']).collateralPoolTokenFactory
settings_2 (-> ['TMP_3625'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3625'])"])
REF_2387(address) (->settings_2 (-> ['TMP_3625'])) := _value_1(address)
TMP_3625(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3625'])"])
 ContractChanged(collateralPoolTokenFactory,_value)
Emit ContractChanged(collateralPoolTokenFactory,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setCollateralPoolTokenTimelockSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3970(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3970'])(AssetManagerSettings.Data) := TMP_3970(AssetManagerSettings.Data)
 require(bool,error)(_value >= 60,revert ValueTooSmall()())
TMP_3971(bool) = _value_1 >= 60
TMP_3972(None) = SOLIDITY_CALL revert ValueTooSmall()()
TMP_3973(None) = SOLIDITY_CALL require(bool,error)(TMP_3971,TMP_3972)
 settings.collateralPoolTokenTimelockSeconds = _value.toUint32()
REF_2513(uint32) -> settings_1 (-> ['TMP_3970']).collateralPoolTokenTimelockSeconds
TMP_3974(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3970'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3970'])"])
REF_2513(uint32) (->settings_2 (-> ['TMP_3970'])) := TMP_3974(uint32)
TMP_3970(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3970'])"])
 SettingChanged(collateralPoolTokenTimelockSeconds,_value)
Emit SettingChanged(collateralPoolTokenTimelockSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setCollateralReservationFeeBips(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3756(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3756'])(AssetManagerSettings.Data) := TMP_3756(AssetManagerSettings.Data)
 require(bool,error)(_value > 0,revert CannotBeZero()())
TMP_3757(bool) = _value_1 > 0
TMP_3758(None) = SOLIDITY_CALL revert CannotBeZero()()
TMP_3759(None) = SOLIDITY_CALL require(bool,error)(TMP_3757,TMP_3758)
 require(bool,error)(_value <= SafePct.MAX_BIPS,revert BipsValueTooHigh()())
REF_2433(uint256) -> SafePct.MAX_BIPS
TMP_3760(bool) = _value_1 <= REF_2433
TMP_3761(None) = SOLIDITY_CALL revert BipsValueTooHigh()()
TMP_3762(None) = SOLIDITY_CALL require(bool,error)(TMP_3760,TMP_3761)
 require(bool,error)(_value <= settings.collateralReservationFeeBIPS * 4,revert FeeIncreaseTooBig()())
REF_2434(uint16) -> settings_1 (-> ['TMP_3756']).collateralReservationFeeBIPS
TMP_3763(uint16) = REF_2434 (c)* 4
TMP_3764(bool) = _value_1 <= TMP_3763
TMP_3765(None) = SOLIDITY_CALL revert FeeIncreaseTooBig()()
TMP_3766(None) = SOLIDITY_CALL require(bool,error)(TMP_3764,TMP_3765)
 require(bool,error)(_value >= settings.collateralReservationFeeBIPS / 4,revert FeeDecreaseTooBig()())
REF_2435(uint16) -> settings_1 (-> ['TMP_3756']).collateralReservationFeeBIPS
TMP_3767(uint16) = REF_2435 (c)/ 4
TMP_3768(bool) = _value_1 >= TMP_3767
TMP_3769(None) = SOLIDITY_CALL revert FeeDecreaseTooBig()()
TMP_3770(None) = SOLIDITY_CALL require(bool,error)(TMP_3768,TMP_3769)
 settings.collateralReservationFeeBIPS = _value.toUint16()
REF_2436(uint16) -> settings_1 (-> ['TMP_3756']).collateralReservationFeeBIPS
TMP_3771(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3756'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3756'])"])
REF_2436(uint16) (->settings_2 (-> ['TMP_3756'])) := TMP_3771(uint16)
TMP_3756(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3756'])"])
 SettingChanged(collateralReservationFeeBIPS,_value)
Emit SettingChanged(collateralReservationFeeBIPS,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setConfirmationByOthersAfterSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3813(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3813'])(AssetManagerSettings.Data) := TMP_3813(AssetManagerSettings.Data)
 require(bool,error)(_value >= 7200,revert MustBeAtLeastTwoHours()())
TMP_3814(bool) = _value_1 >= 7200
TMP_3815(None) = SOLIDITY_CALL revert MustBeAtLeastTwoHours()()
TMP_3816(None) = SOLIDITY_CALL require(bool,error)(TMP_3814,TMP_3815)
 settings.confirmationByOthersAfterSeconds = _value.toUint64()
REF_2453(uint64) -> settings_1 (-> ['TMP_3813']).confirmationByOthersAfterSeconds
TMP_3817(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3813'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3813'])"])
REF_2453(uint64) (->settings_2 (-> ['TMP_3813'])) := TMP_3817(uint64)
TMP_3813(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3813'])"])
 SettingChanged(confirmationByOthersAfterSeconds,_value)
Emit SettingChanged(confirmationByOthersAfterSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setConfirmationByOthersRewardUSD5(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3821(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3821'])(AssetManagerSettings.Data) := TMP_3821(AssetManagerSettings.Data)
 require(bool,error)(_value > 0,revert CannotBeZero()())
TMP_3822(bool) = _value_1 > 0
TMP_3823(None) = SOLIDITY_CALL revert CannotBeZero()()
TMP_3824(None) = SOLIDITY_CALL require(bool,error)(TMP_3822,TMP_3823)
 require(bool,error)(_value <= settings.confirmationByOthersRewardUSD5 * 4,revert FeeIncreaseTooBig()())
REF_2456(uint128) -> settings_1 (-> ['TMP_3821']).confirmationByOthersRewardUSD5
TMP_3825(uint128) = REF_2456 (c)* 4
TMP_3826(bool) = _value_1 <= TMP_3825
TMP_3827(None) = SOLIDITY_CALL revert FeeIncreaseTooBig()()
TMP_3828(None) = SOLIDITY_CALL require(bool,error)(TMP_3826,TMP_3827)
 require(bool,error)(_value >= settings.confirmationByOthersRewardUSD5 / 4,revert FeeDecreaseTooBig()())
REF_2457(uint128) -> settings_1 (-> ['TMP_3821']).confirmationByOthersRewardUSD5
TMP_3829(uint128) = REF_2457 (c)/ 4
TMP_3830(bool) = _value_1 >= TMP_3829
TMP_3831(None) = SOLIDITY_CALL revert FeeDecreaseTooBig()()
TMP_3832(None) = SOLIDITY_CALL require(bool,error)(TMP_3830,TMP_3831)
 settings.confirmationByOthersRewardUSD5 = _value.toUint128()
REF_2458(uint128) -> settings_1 (-> ['TMP_3821']).confirmationByOthersRewardUSD5
TMP_3833(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3821'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3821'])"])
REF_2458(uint128) (->settings_2 (-> ['TMP_3821'])) := TMP_3833(uint128)
TMP_3821(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3821'])"])
 SettingChanged(confirmationByOthersRewardUSD5,_value)
Emit SettingChanged(confirmationByOthersRewardUSD5,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setEmergencyPauseDurationResetAfterSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4027(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4027'])(AssetManagerSettings.Data) := TMP_4027(AssetManagerSettings.Data)
 require(bool,error)(_value > 0,revert CannotBeZero()())
TMP_4028(bool) = _value_1 > 0
TMP_4029(None) = SOLIDITY_CALL revert CannotBeZero()()
TMP_4030(None) = SOLIDITY_CALL require(bool,error)(TMP_4028,TMP_4029)
 require(bool,error)(_value <= settings.emergencyPauseDurationResetAfterSeconds * 4 + 3600,revert IncreaseTooBig()())
REF_2543(uint64) -> settings_1 (-> ['TMP_4027']).emergencyPauseDurationResetAfterSeconds
TMP_4031(uint64) = REF_2543 (c)* 4
TMP_4032(uint64) = TMP_4031 (c)+ 3600
TMP_4033(bool) = _value_1 <= TMP_4032
TMP_4034(None) = SOLIDITY_CALL revert IncreaseTooBig()()
TMP_4035(None) = SOLIDITY_CALL require(bool,error)(TMP_4033,TMP_4034)
 require(bool,error)(_value >= settings.emergencyPauseDurationResetAfterSeconds / 4,revert DecreaseTooBig()())
REF_2544(uint64) -> settings_1 (-> ['TMP_4027']).emergencyPauseDurationResetAfterSeconds
TMP_4036(uint64) = REF_2544 (c)/ 4
TMP_4037(bool) = _value_1 >= TMP_4036
TMP_4038(None) = SOLIDITY_CALL revert DecreaseTooBig()()
TMP_4039(None) = SOLIDITY_CALL require(bool,error)(TMP_4037,TMP_4038)
 settings.emergencyPauseDurationResetAfterSeconds = _value.toUint64()
REF_2545(uint64) -> settings_1 (-> ['TMP_4027']).emergencyPauseDurationResetAfterSeconds
TMP_4040(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_4027'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_4027'])"])
REF_2545(uint64) (->settings_2 (-> ['TMP_4027'])) := TMP_4040(uint64)
TMP_4027(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_4027'])"])
 SettingChanged(emergencyPauseDurationResetAfterSeconds,_value)
Emit SettingChanged(emergencyPauseDurationResetAfterSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setFdcVerification(address) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3641(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3641'])(AssetManagerSettings.Data) := TMP_3641(AssetManagerSettings.Data)
 require(bool,error)(_value != address(0),revert InvalidAddress()())
TMP_3642 = CONVERT 0 to address
TMP_3643(bool) = _value_1 != TMP_3642
TMP_3644(None) = SOLIDITY_CALL revert InvalidAddress()()
TMP_3645(None) = SOLIDITY_CALL require(bool,error)(TMP_3643,TMP_3644)
 settings.fdcVerification = _value
REF_2391(address) -> settings_1 (-> ['TMP_3641']).fdcVerification
settings_2 (-> ['TMP_3641'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3641'])"])
REF_2391(address) (->settings_2 (-> ['TMP_3641'])) := _value_1(address)
TMP_3641(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3641'])"])
 IAssetManagerEvents.ContractChanged(fdcVerification,_value)
Emit ContractChanged(fdcVerification,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setLiquidationPaymentFactors(uint256[],uint256[]) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3994(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3994'])(AssetManagerSettings.Data) := TMP_3994(AssetManagerSettings.Data)
 SettingsValidators.validateLiquidationFactors(_liquidationFactors,_vaultCollateralFactors)
LIBRARY_CALL, dest:SettingsValidators, function:SettingsValidators.validateLiquidationFactors(uint256[],uint256[]), arguments:['_liquidationFactors_1', '_vaultCollateralFactors_1'] 
 delete settings.liquidationCollateralFactorBIPS
REF_2522(uint256[]) -> settings_1 (-> ['TMP_3994']).liquidationCollateralFactorBIPS
settings_2 (-> []) = delete REF_2522 
 delete settings.liquidationFactorVaultCollateralBIPS
REF_2523(uint256[]) -> settings_2 (-> []).liquidationFactorVaultCollateralBIPS
settings_3 (-> []) = delete REF_2523 
 i = 0
i_1(uint256) := 0(uint256)
 i < _liquidationFactors.length
settings_4 (-> [])(AssetManagerSettings.Data) := phi(['settings_8 (-> [])', 'settings_3 (-> [])'])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_2524 -> LENGTH _liquidationFactors_1
TMP_3996(bool) = i_2 < REF_2524
CONDITION TMP_3996
 settings.liquidationCollateralFactorBIPS.push(_liquidationFactors[i].toUint32())
REF_2525(uint256[]) -> settings_4 (-> []).liquidationCollateralFactorBIPS
REF_2527(uint256) -> _liquidationFactors_1[i_2]
TMP_3997(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['REF_2527'] 
REF_2529 -> LENGTH REF_2525
TMP_3999(uint256) := REF_2529(uint256)
TMP_4000(uint256) = TMP_3999 (c)+ 1
settings_5 (-> [])(AssetManagerSettings.Data) := phi(['settings_4 (-> [])'])
REF_2529(uint256) (->settings_6 (-> [])) := TMP_4000(uint256)
REF_2530(uint256) -> REF_2525[TMP_3999]
settings_6 (-> [])(AssetManagerSettings.Data) := phi(['settings_5 (-> [])'])
REF_2530(uint256) (->settings_6 (-> [])) := TMP_3997(uint32)
 settings.liquidationFactorVaultCollateralBIPS.push(_vaultCollateralFactors[i].toUint32())
REF_2531(uint256[]) -> settings_6 (-> []).liquidationFactorVaultCollateralBIPS
REF_2533(uint256) -> _vaultCollateralFactors_1[i_2]
TMP_4001(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['REF_2533'] 
REF_2535 -> LENGTH REF_2531
TMP_4003(uint256) := REF_2535(uint256)
TMP_4004(uint256) = TMP_4003 (c)+ 1
settings_7 (-> [])(AssetManagerSettings.Data) := phi(['settings_6 (-> [])'])
REF_2535(uint256) (->settings_8 (-> [])) := TMP_4004(uint256)
REF_2536(uint256) -> REF_2531[TMP_4003]
settings_8 (-> [])(AssetManagerSettings.Data) := phi(['settings_7 (-> [])'])
REF_2536(uint256) (->settings_8 (-> [])) := TMP_4001(uint32)
 i ++
TMP_4005(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 SettingArrayChanged(liquidationCollateralFactorBIPS,_liquidationFactors)
Emit SettingArrayChanged(liquidationCollateralFactorBIPS,_liquidationFactors_1)
 SettingArrayChanged(liquidationFactorVaultCollateralBIPS,_vaultCollateralFactors)
Emit SettingArrayChanged(liquidationFactorVaultCollateralBIPS,_vaultCollateralFactors_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setLiquidationStepSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3978(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3978'])(AssetManagerSettings.Data) := TMP_3978(AssetManagerSettings.Data)
 require(bool,error)(_stepSeconds > 0,revert CannotBeZero()())
TMP_3979(bool) = _stepSeconds_1 > 0
TMP_3980(None) = SOLIDITY_CALL revert CannotBeZero()()
TMP_3981(None) = SOLIDITY_CALL require(bool,error)(TMP_3979,TMP_3980)
 require(bool,error)(_stepSeconds <= settings.liquidationStepSeconds * 2,revert IncreaseTooBig()())
REF_2516(uint64) -> settings_1 (-> ['TMP_3978']).liquidationStepSeconds
TMP_3982(uint64) = REF_2516 (c)* 2
TMP_3983(bool) = _stepSeconds_1 <= TMP_3982
TMP_3984(None) = SOLIDITY_CALL revert IncreaseTooBig()()
TMP_3985(None) = SOLIDITY_CALL require(bool,error)(TMP_3983,TMP_3984)
 require(bool,error)(_stepSeconds >= settings.liquidationStepSeconds / 2,revert DecreaseTooBig()())
REF_2517(uint64) -> settings_1 (-> ['TMP_3978']).liquidationStepSeconds
TMP_3986(uint64) = REF_2517 (c)/ 2
TMP_3987(bool) = _stepSeconds_1 >= TMP_3986
TMP_3988(None) = SOLIDITY_CALL revert DecreaseTooBig()()
TMP_3989(None) = SOLIDITY_CALL require(bool,error)(TMP_3987,TMP_3988)
 settings.liquidationStepSeconds = _stepSeconds.toUint64()
REF_2518(uint64) -> settings_1 (-> ['TMP_3978']).liquidationStepSeconds
TMP_3990(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_stepSeconds_1'] 
settings_2 (-> ['TMP_3978'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3978'])"])
REF_2518(uint64) (->settings_2 (-> ['TMP_3978'])) := TMP_3990(uint64)
TMP_3978(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3978'])"])
 SettingChanged(liquidationStepSeconds,_stepSeconds)
Emit SettingChanged(liquidationStepSeconds,_stepSeconds_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setLotSizeAmg(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3719(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3719'])(AssetManagerSettings.Data) := TMP_3719(AssetManagerSettings.Data)
 require(bool,error)(_value > 0,revert CannotBeZero()())
TMP_3720(bool) = _value_1 > 0
TMP_3721(None) = SOLIDITY_CALL revert CannotBeZero()()
TMP_3722(None) = SOLIDITY_CALL require(bool,error)(TMP_3720,TMP_3721)
 require(bool,error)(_value <= settings.lotSizeAMG * 10,revert LotSizeIncreaseTooBig()())
REF_2421(uint64) -> settings_1 (-> ['TMP_3719']).lotSizeAMG
TMP_3723(uint64) = REF_2421 (c)* 10
TMP_3724(bool) = _value_1 <= TMP_3723
TMP_3725(None) = SOLIDITY_CALL revert LotSizeIncreaseTooBig()()
TMP_3726(None) = SOLIDITY_CALL require(bool,error)(TMP_3724,TMP_3725)
 require(bool,error)(_value >= settings.lotSizeAMG / 10,revert LotSizeDecreaseTooBig()())
REF_2422(uint64) -> settings_1 (-> ['TMP_3719']).lotSizeAMG
TMP_3727(uint64) = REF_2422 (c)/ 10
TMP_3728(bool) = _value_1 >= TMP_3727
TMP_3729(None) = SOLIDITY_CALL revert LotSizeDecreaseTooBig()()
TMP_3730(None) = SOLIDITY_CALL require(bool,error)(TMP_3728,TMP_3729)
 require(bool,error)(settings.mintingCapAMG == 0 || settings.mintingCapAMG >= _value,revert LotSizeBiggerThanMintingCap()())
REF_2423(uint64) -> settings_1 (-> ['TMP_3719']).mintingCapAMG
TMP_3731(bool) = REF_2423 == 0
REF_2424(uint64) -> settings_1 (-> ['TMP_3719']).mintingCapAMG
TMP_3732(bool) = REF_2424 >= _value_1
TMP_3733(bool) = TMP_3731 || TMP_3732
TMP_3734(None) = SOLIDITY_CALL revert LotSizeBiggerThanMintingCap()()
TMP_3735(None) = SOLIDITY_CALL require(bool,error)(TMP_3733,TMP_3734)
 settings.lotSizeAMG = _value.toUint64()
REF_2425(uint64) -> settings_1 (-> ['TMP_3719']).lotSizeAMG
TMP_3736(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3719'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3719'])"])
REF_2425(uint64) (->settings_2 (-> ['TMP_3719'])) := TMP_3736(uint64)
TMP_3719(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3719'])"])
 SettingChanged(lotSizeAMG,_value)
Emit SettingChanged(lotSizeAMG,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setMaxEmergencyPauseDurationSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4010(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4010'])(AssetManagerSettings.Data) := TMP_4010(AssetManagerSettings.Data)
 require(bool,error)(_value > 0,revert CannotBeZero()())
TMP_4011(bool) = _value_1 > 0
TMP_4012(None) = SOLIDITY_CALL revert CannotBeZero()()
TMP_4013(None) = SOLIDITY_CALL require(bool,error)(TMP_4011,TMP_4012)
 require(bool,error)(_value <= settings.maxEmergencyPauseDurationSeconds * 4 + 60,revert IncreaseTooBig()())
REF_2538(uint64) -> settings_1 (-> ['TMP_4010']).maxEmergencyPauseDurationSeconds
TMP_4014(uint64) = REF_2538 (c)* 4
TMP_4015(uint64) = TMP_4014 (c)+ 60
TMP_4016(bool) = _value_1 <= TMP_4015
TMP_4017(None) = SOLIDITY_CALL revert IncreaseTooBig()()
TMP_4018(None) = SOLIDITY_CALL require(bool,error)(TMP_4016,TMP_4017)
 require(bool,error)(_value >= settings.maxEmergencyPauseDurationSeconds / 4,revert DecreaseTooBig()())
REF_2539(uint64) -> settings_1 (-> ['TMP_4010']).maxEmergencyPauseDurationSeconds
TMP_4019(uint64) = REF_2539 (c)/ 4
TMP_4020(bool) = _value_1 >= TMP_4019
TMP_4021(None) = SOLIDITY_CALL revert DecreaseTooBig()()
TMP_4022(None) = SOLIDITY_CALL require(bool,error)(TMP_4020,TMP_4021)
 settings.maxEmergencyPauseDurationSeconds = _value.toUint64()
REF_2540(uint64) -> settings_1 (-> ['TMP_4010']).maxEmergencyPauseDurationSeconds
TMP_4023(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_4010'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_4010'])"])
REF_2540(uint64) (->settings_2 (-> ['TMP_4010'])) := TMP_4023(uint64)
TMP_4010(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_4010'])"])
 SettingChanged(maxEmergencyPauseDurationSeconds,_value)
Emit SettingChanged(maxEmergencyPauseDurationSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setMaxRedeemedTickets(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3837(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3837'])(AssetManagerSettings.Data) := TMP_3837(AssetManagerSettings.Data)
 require(bool,error)(_value > 0,revert CannotBeZero()())
TMP_3838(bool) = _value_1 > 0
TMP_3839(None) = SOLIDITY_CALL revert CannotBeZero()()
TMP_3840(None) = SOLIDITY_CALL require(bool,error)(TMP_3838,TMP_3839)
 require(bool,error)(_value <= settings.maxRedeemedTickets * 2,revert IncreaseTooBig()())
REF_2461(uint16) -> settings_1 (-> ['TMP_3837']).maxRedeemedTickets
TMP_3841(uint16) = REF_2461 (c)* 2
TMP_3842(bool) = _value_1 <= TMP_3841
TMP_3843(None) = SOLIDITY_CALL revert IncreaseTooBig()()
TMP_3844(None) = SOLIDITY_CALL require(bool,error)(TMP_3842,TMP_3843)
 require(bool,error)(_value >= settings.maxRedeemedTickets / 4,revert DecreaseTooBig()())
REF_2462(uint16) -> settings_1 (-> ['TMP_3837']).maxRedeemedTickets
TMP_3845(uint16) = REF_2462 (c)/ 4
TMP_3846(bool) = _value_1 >= TMP_3845
TMP_3847(None) = SOLIDITY_CALL revert DecreaseTooBig()()
TMP_3848(None) = SOLIDITY_CALL require(bool,error)(TMP_3846,TMP_3847)
 settings.maxRedeemedTickets = _value.toUint16()
REF_2463(uint16) -> settings_1 (-> ['TMP_3837']).maxRedeemedTickets
TMP_3849(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3837'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3837'])"])
REF_2463(uint16) (->settings_2 (-> ['TMP_3837'])) := TMP_3849(uint16)
TMP_3837(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3837'])"])
 SettingChanged(maxRedeemedTickets,_value)
Emit SettingChanged(maxRedeemedTickets,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setMaxTrustedPriceAgeSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3740(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3740'])(AssetManagerSettings.Data) := TMP_3740(AssetManagerSettings.Data)
 require(bool,error)(_value > 0,revert CannotBeZero()())
TMP_3741(bool) = _value_1 > 0
TMP_3742(None) = SOLIDITY_CALL revert CannotBeZero()()
TMP_3743(None) = SOLIDITY_CALL require(bool,error)(TMP_3741,TMP_3742)
 require(bool,error)(_value <= settings.maxTrustedPriceAgeSeconds * 2,revert FeeIncreaseTooBig()())
REF_2428(uint64) -> settings_1 (-> ['TMP_3740']).maxTrustedPriceAgeSeconds
TMP_3744(uint64) = REF_2428 (c)* 2
TMP_3745(bool) = _value_1 <= TMP_3744
TMP_3746(None) = SOLIDITY_CALL revert FeeIncreaseTooBig()()
TMP_3747(None) = SOLIDITY_CALL require(bool,error)(TMP_3745,TMP_3746)
 require(bool,error)(_value >= settings.maxTrustedPriceAgeSeconds / 2,revert FeeDecreaseTooBig()())
REF_2429(uint64) -> settings_1 (-> ['TMP_3740']).maxTrustedPriceAgeSeconds
TMP_3748(uint64) = REF_2429 (c)/ 2
TMP_3749(bool) = _value_1 >= TMP_3748
TMP_3750(None) = SOLIDITY_CALL revert FeeDecreaseTooBig()()
TMP_3751(None) = SOLIDITY_CALL require(bool,error)(TMP_3749,TMP_3750)
 settings.maxTrustedPriceAgeSeconds = _value.toUint64()
REF_2430(uint64) -> settings_1 (-> ['TMP_3740']).maxTrustedPriceAgeSeconds
TMP_3752(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3740'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3740'])"])
REF_2430(uint64) (->settings_2 (-> ['TMP_3740'])) := TMP_3752(uint64)
TMP_3740(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3740'])"])
 SettingChanged(maxTrustedPriceAgeSeconds,_value)
Emit SettingChanged(maxTrustedPriceAgeSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setMinUpdateRepeatTimeSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3711(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3711'])(AssetManagerSettings.Data) := TMP_3711(AssetManagerSettings.Data)
 require(bool,error)(_value > 0,revert CannotBeZero()())
TMP_3712(bool) = _value_1 > 0
TMP_3713(None) = SOLIDITY_CALL revert CannotBeZero()()
TMP_3714(None) = SOLIDITY_CALL require(bool,error)(TMP_3712,TMP_3713)
 settings.minUpdateRepeatTimeSeconds = _value.toUint64()
REF_2418(uint64) -> settings_1 (-> ['TMP_3711']).minUpdateRepeatTimeSeconds
TMP_3715(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3711'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3711'])"])
REF_2418(uint64) (->settings_2 (-> ['TMP_3711'])) := TMP_3715(uint64)
TMP_3711(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3711'])"])
 SettingChanged(minUpdateRepeatTimeSeconds,_value)
Emit SettingChanged(minUpdateRepeatTimeSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setMintingCapAmg(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3899(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3899'])(AssetManagerSettings.Data) := TMP_3899(AssetManagerSettings.Data)
 require(bool,error)(_value == 0 || _value >= settings.lotSizeAMG,revert ValueTooSmall()())
TMP_3900(bool) = _value_1 == 0
REF_2483(uint64) -> settings_1 (-> ['TMP_3899']).lotSizeAMG
TMP_3901(bool) = _value_1 >= REF_2483
TMP_3902(bool) = TMP_3900 || TMP_3901
TMP_3903(None) = SOLIDITY_CALL revert ValueTooSmall()()
TMP_3904(None) = SOLIDITY_CALL require(bool,error)(TMP_3902,TMP_3903)
 settings.mintingCapAMG = _value.toUint64()
REF_2484(uint64) -> settings_1 (-> ['TMP_3899']).mintingCapAMG
TMP_3905(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3899'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3899'])"])
REF_2484(uint64) (->settings_2 (-> ['TMP_3899'])) := TMP_3905(uint64)
TMP_3899(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3899'])"])
 SettingChanged(mintingCapAMG,_value)
Emit SettingChanged(mintingCapAMG,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setMintingPoolHoldingsRequiredBIPS(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3889(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3889'])(AssetManagerSettings.Data) := TMP_3889(AssetManagerSettings.Data)
 require(bool,error)(_value <= settings.mintingPoolHoldingsRequiredBIPS * 4 + SafePct.MAX_BIPS,revert ValueTooBig()())
REF_2478(uint32) -> settings_1 (-> ['TMP_3889']).mintingPoolHoldingsRequiredBIPS
TMP_3890(uint32) = REF_2478 (c)* 4
REF_2479(uint256) -> SafePct.MAX_BIPS
TMP_3891(uint32) = TMP_3890 (c)+ REF_2479
TMP_3892(bool) = _value_1 <= TMP_3891
TMP_3893(None) = SOLIDITY_CALL revert ValueTooBig()()
TMP_3894(None) = SOLIDITY_CALL require(bool,error)(TMP_3892,TMP_3893)
 settings.mintingPoolHoldingsRequiredBIPS = _value.toUint32()
REF_2480(uint32) -> settings_1 (-> ['TMP_3889']).mintingPoolHoldingsRequiredBIPS
TMP_3895(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3889'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3889'])"])
REF_2480(uint32) (->settings_2 (-> ['TMP_3889'])) := TMP_3895(uint32)
TMP_3889(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3889'])"])
 SettingChanged(mintingPoolHoldingsRequiredBIPS,_value)
Emit SettingChanged(mintingPoolHoldingsRequiredBIPS,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setPaymentChallengeReward(uint256,uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3686(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3686'])(AssetManagerSettings.Data) := TMP_3686(AssetManagerSettings.Data)
 require(bool,error)(_rewardNATWei <= (settings.paymentChallengeRewardUSD5 * 4) + 100000000000000000000,revert IncreaseTooBig()())
REF_2409(uint128) -> settings_1 (-> ['TMP_3686']).paymentChallengeRewardUSD5
TMP_3687(uint128) = REF_2409 (c)* 4
TMP_3688(uint128) = TMP_3687 (c)+ 100000000000000000000
TMP_3689(bool) = _rewardNATWei_1 <= TMP_3688
TMP_3690(None) = SOLIDITY_CALL revert IncreaseTooBig()()
TMP_3691(None) = SOLIDITY_CALL require(bool,error)(TMP_3689,TMP_3690)
 require(bool,error)(_rewardNATWei >= (settings.paymentChallengeRewardUSD5) / 4,revert DecreaseTooBig()())
REF_2410(uint128) -> settings_1 (-> ['TMP_3686']).paymentChallengeRewardUSD5
TMP_3692(uint128) = REF_2410 (c)/ 4
TMP_3693(bool) = _rewardNATWei_1 >= TMP_3692
TMP_3694(None) = SOLIDITY_CALL revert DecreaseTooBig()()
TMP_3695(None) = SOLIDITY_CALL require(bool,error)(TMP_3693,TMP_3694)
 require(bool,error)(_rewardBIPS <= (settings.paymentChallengeRewardBIPS * 4) + 100,revert IncreaseTooBig()())
REF_2411(uint16) -> settings_1 (-> ['TMP_3686']).paymentChallengeRewardBIPS
TMP_3696(uint16) = REF_2411 (c)* 4
TMP_3697(uint16) = TMP_3696 (c)+ 100
TMP_3698(bool) = _rewardBIPS_1 <= TMP_3697
TMP_3699(None) = SOLIDITY_CALL revert IncreaseTooBig()()
TMP_3700(None) = SOLIDITY_CALL require(bool,error)(TMP_3698,TMP_3699)
 require(bool,error)(_rewardBIPS >= (settings.paymentChallengeRewardBIPS) / 4,revert DecreaseTooBig()())
REF_2412(uint16) -> settings_1 (-> ['TMP_3686']).paymentChallengeRewardBIPS
TMP_3701(uint16) = REF_2412 (c)/ 4
TMP_3702(bool) = _rewardBIPS_1 >= TMP_3701
TMP_3703(None) = SOLIDITY_CALL revert DecreaseTooBig()()
TMP_3704(None) = SOLIDITY_CALL require(bool,error)(TMP_3702,TMP_3703)
 settings.paymentChallengeRewardUSD5 = _rewardNATWei.toUint128()
REF_2413(uint128) -> settings_1 (-> ['TMP_3686']).paymentChallengeRewardUSD5
TMP_3705(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['_rewardNATWei_1'] 
settings_2 (-> ['TMP_3686'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3686'])"])
REF_2413(uint128) (->settings_2 (-> ['TMP_3686'])) := TMP_3705(uint128)
TMP_3686(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3686'])"])
 settings.paymentChallengeRewardBIPS = _rewardBIPS.toUint16()
REF_2415(uint16) -> settings_2 (-> ['TMP_3686']).paymentChallengeRewardBIPS
TMP_3706(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_rewardBIPS_1'] 
settings_3 (-> ['TMP_3686'])(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3686'])"])
REF_2415(uint16) (->settings_3 (-> ['TMP_3686'])) := TMP_3706(uint16)
TMP_3686(AssetManagerSettings.Data) := phi(["settings_3 (-> ['TMP_3686'])"])
 SettingChanged(paymentChallengeRewardUSD5,_rewardNATWei)
Emit SettingChanged(paymentChallengeRewardUSD5,_rewardNATWei_1)
 SettingChanged(paymentChallengeRewardBIPS,_rewardBIPS)
Emit SettingChanged(paymentChallengeRewardBIPS,_rewardBIPS_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setPoolExitCRChangeTimelockSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3952(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3952'])(AssetManagerSettings.Data) := TMP_3952(AssetManagerSettings.Data)
 require(bool,error)(_value <= settings.poolExitCRChangeTimelockSeconds * 4 + 86400,revert ValueTooBig()())
REF_2506(uint64) -> settings_1 (-> ['TMP_3952']).poolExitCRChangeTimelockSeconds
TMP_3953(uint64) = REF_2506 (c)* 4
TMP_3954(uint64) = TMP_3953 (c)+ 86400
TMP_3955(bool) = _value_1 <= TMP_3954
TMP_3956(None) = SOLIDITY_CALL revert ValueTooBig()()
TMP_3957(None) = SOLIDITY_CALL require(bool,error)(TMP_3955,TMP_3956)
 settings.poolExitCRChangeTimelockSeconds = _value.toUint64()
REF_2507(uint64) -> settings_1 (-> ['TMP_3952']).poolExitCRChangeTimelockSeconds
TMP_3958(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3952'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3952'])"])
REF_2507(uint64) (->settings_2 (-> ['TMP_3952'])) := TMP_3958(uint64)
TMP_3952(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3952'])"])
 SettingChanged(poolExitCRChangeTimelockSeconds,_value)
Emit SettingChanged(poolExitCRChangeTimelockSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setPriceReader(address) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3633(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3633'])(AssetManagerSettings.Data) := TMP_3633(AssetManagerSettings.Data)
 require(bool,error)(_value != address(0),revert InvalidAddress()())
TMP_3634 = CONVERT 0 to address
TMP_3635(bool) = _value_1 != TMP_3634
TMP_3636(None) = SOLIDITY_CALL revert InvalidAddress()()
TMP_3637(None) = SOLIDITY_CALL require(bool,error)(TMP_3635,TMP_3636)
 settings.priceReader = _value
REF_2389(address) -> settings_1 (-> ['TMP_3633']).priceReader
settings_2 (-> ['TMP_3633'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3633'])"])
REF_2389(address) (->settings_2 (-> ['TMP_3633'])) := _value_1(address)
TMP_3633(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3633'])"])
 ContractChanged(priceReader,_value)
Emit ContractChanged(priceReader,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setRedemptionDefaultFactorVaultCollateralBIPS(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3794(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3794'])(AssetManagerSettings.Data) := TMP_3794(AssetManagerSettings.Data)
 require(bool,error)(_value > SafePct.MAX_BIPS,revert BipsValueTooLow()())
REF_2445(uint256) -> SafePct.MAX_BIPS
TMP_3795(bool) = _value_1 > REF_2445
TMP_3796(None) = SOLIDITY_CALL revert BipsValueTooLow()()
TMP_3797(None) = SOLIDITY_CALL require(bool,error)(TMP_3795,TMP_3796)
 require(bool,error)(_value <= uint256(settings.redemptionDefaultFactorVaultCollateralBIPS).mulBips(12000) + 1000,revert FeeIncreaseTooBig()())
REF_2446(uint32) -> settings_1 (-> ['TMP_3794']).redemptionDefaultFactorVaultCollateralBIPS
TMP_3798 = CONVERT REF_2446 to uint256
TMP_3799(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_3798', '12000'] 
TMP_3800(uint256) = TMP_3799 (c)+ 1000
TMP_3801(bool) = _value_1 <= TMP_3800
TMP_3802(None) = SOLIDITY_CALL revert FeeIncreaseTooBig()()
TMP_3803(None) = SOLIDITY_CALL require(bool,error)(TMP_3801,TMP_3802)
 require(bool,error)(_value >= uint256(settings.redemptionDefaultFactorVaultCollateralBIPS).mulBips(8333),revert FeeDecreaseTooBig()())
REF_2448(uint32) -> settings_1 (-> ['TMP_3794']).redemptionDefaultFactorVaultCollateralBIPS
TMP_3804 = CONVERT REF_2448 to uint256
TMP_3805(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_3804', '8333'] 
TMP_3806(bool) = _value_1 >= TMP_3805
TMP_3807(None) = SOLIDITY_CALL revert FeeDecreaseTooBig()()
TMP_3808(None) = SOLIDITY_CALL require(bool,error)(TMP_3806,TMP_3807)
 settings.redemptionDefaultFactorVaultCollateralBIPS = _value.toUint32()
REF_2450(uint32) -> settings_1 (-> ['TMP_3794']).redemptionDefaultFactorVaultCollateralBIPS
TMP_3809(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3794'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3794'])"])
REF_2450(uint32) (->settings_2 (-> ['TMP_3794'])) := TMP_3809(uint32)
TMP_3794(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3794'])"])
 SettingChanged(redemptionDefaultFactorVaultCollateralBIPS,_value)
Emit SettingChanged(redemptionDefaultFactorVaultCollateralBIPS,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setRedemptionFeeBips(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3775(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3775'])(AssetManagerSettings.Data) := TMP_3775(AssetManagerSettings.Data)
 require(bool,error)(_value > 0,revert CannotBeZero()())
TMP_3776(bool) = _value_1 > 0
TMP_3777(None) = SOLIDITY_CALL revert CannotBeZero()()
TMP_3778(None) = SOLIDITY_CALL require(bool,error)(TMP_3776,TMP_3777)
 require(bool,error)(_value <= SafePct.MAX_BIPS,revert BipsValueTooHigh()())
REF_2439(uint256) -> SafePct.MAX_BIPS
TMP_3779(bool) = _value_1 <= REF_2439
TMP_3780(None) = SOLIDITY_CALL revert BipsValueTooHigh()()
TMP_3781(None) = SOLIDITY_CALL require(bool,error)(TMP_3779,TMP_3780)
 require(bool,error)(_value <= settings.redemptionFeeBIPS * 4,revert FeeIncreaseTooBig()())
REF_2440(uint16) -> settings_1 (-> ['TMP_3775']).redemptionFeeBIPS
TMP_3782(uint16) = REF_2440 (c)* 4
TMP_3783(bool) = _value_1 <= TMP_3782
TMP_3784(None) = SOLIDITY_CALL revert FeeIncreaseTooBig()()
TMP_3785(None) = SOLIDITY_CALL require(bool,error)(TMP_3783,TMP_3784)
 require(bool,error)(_value >= settings.redemptionFeeBIPS / 4,revert FeeDecreaseTooBig()())
REF_2441(uint16) -> settings_1 (-> ['TMP_3775']).redemptionFeeBIPS
TMP_3786(uint16) = REF_2441 (c)/ 4
TMP_3787(bool) = _value_1 >= TMP_3786
TMP_3788(None) = SOLIDITY_CALL revert FeeDecreaseTooBig()()
TMP_3789(None) = SOLIDITY_CALL require(bool,error)(TMP_3787,TMP_3788)
 settings.redemptionFeeBIPS = _value.toUint16()
REF_2442(uint16) -> settings_1 (-> ['TMP_3775']).redemptionFeeBIPS
TMP_3790(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3775'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3775'])"])
REF_2442(uint16) (->settings_2 (-> ['TMP_3775'])) := TMP_3790(uint16)
TMP_3775(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3775'])"])
 SettingChanged(redemptionFeeBIPS,_value)
Emit SettingChanged(redemptionFeeBIPS,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setTimeForPayment(uint256,uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3672(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3672'])(AssetManagerSettings.Data) := TMP_3672(AssetManagerSettings.Data)
 require(bool,error)(_underlyingSeconds > 0,revert CannotBeZero()())
TMP_3673(bool) = _underlyingSeconds_1 > 0
TMP_3674(None) = SOLIDITY_CALL revert CannotBeZero()()
TMP_3675(None) = SOLIDITY_CALL require(bool,error)(TMP_3673,TMP_3674)
 require(bool,error)(_underlyingBlocks > 0,revert CannotBeZero()())
TMP_3676(bool) = _underlyingBlocks_1 > 0
TMP_3677(None) = SOLIDITY_CALL revert CannotBeZero()()
TMP_3678(None) = SOLIDITY_CALL require(bool,error)(TMP_3676,TMP_3677)
 SettingsValidators.validateTimeForPayment(_underlyingBlocks,_underlyingSeconds,settings.averageBlockTimeMS)
REF_2403(uint32) -> settings_1 (-> ['TMP_3672']).averageBlockTimeMS
LIBRARY_CALL, dest:SettingsValidators, function:SettingsValidators.validateTimeForPayment(uint256,uint256,uint256), arguments:['_underlyingBlocks_1', '_underlyingSeconds_1', 'REF_2403'] 
 settings.underlyingBlocksForPayment = _underlyingBlocks.toUint64()
REF_2404(uint64) -> settings_1 (-> ['TMP_3672']).underlyingBlocksForPayment
TMP_3680(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_underlyingBlocks_1'] 
settings_2 (-> ['TMP_3672'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3672'])"])
REF_2404(uint64) (->settings_2 (-> ['TMP_3672'])) := TMP_3680(uint64)
TMP_3672(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3672'])"])
 settings.underlyingSecondsForPayment = _underlyingSeconds.toUint64()
REF_2406(uint64) -> settings_2 (-> ['TMP_3672']).underlyingSecondsForPayment
TMP_3681(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_underlyingSeconds_1'] 
settings_3 (-> ['TMP_3672'])(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3672'])"])
REF_2406(uint64) (->settings_3 (-> ['TMP_3672'])) := TMP_3681(uint64)
TMP_3672(AssetManagerSettings.Data) := phi(["settings_3 (-> ['TMP_3672'])"])
 SettingChanged(underlyingBlocksForPayment,_underlyingBlocks)
Emit SettingChanged(underlyingBlocksForPayment,_underlyingBlocks_1)
 SettingChanged(underlyingSecondsForPayment,_underlyingSeconds)
Emit SettingChanged(underlyingSecondsForPayment,_underlyingSeconds_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setTokenInvalidationTimeMinSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3909(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3909'])(AssetManagerSettings.Data) := TMP_3909(AssetManagerSettings.Data)
 settings.tokenInvalidationTimeMinSeconds = _value.toUint64()
REF_2487(uint64) -> settings_1 (-> ['TMP_3909']).tokenInvalidationTimeMinSeconds
TMP_3910(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3909'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3909'])"])
REF_2487(uint64) (->settings_2 (-> ['TMP_3909'])) := TMP_3910(uint64)
TMP_3909(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3909'])"])
 SettingChanged(tokenInvalidationTimeMinSeconds,_value)
Emit SettingChanged(tokenInvalidationTimeMinSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setVaultCollateralBuyForFlareFactorBIPS(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3914(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3914'])(AssetManagerSettings.Data) := TMP_3914(AssetManagerSettings.Data)
 require(bool,error)(_value >= SafePct.MAX_BIPS,revert ValueTooSmall()())
REF_2490(uint256) -> SafePct.MAX_BIPS
TMP_3915(bool) = _value_1 >= REF_2490
TMP_3916(None) = SOLIDITY_CALL revert ValueTooSmall()()
TMP_3917(None) = SOLIDITY_CALL require(bool,error)(TMP_3915,TMP_3916)
 settings.vaultCollateralBuyForFlareFactorBIPS = _value.toUint32()
REF_2491(uint32) -> settings_1 (-> ['TMP_3914']).vaultCollateralBuyForFlareFactorBIPS
TMP_3918(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3914'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3914'])"])
REF_2491(uint32) (->settings_2 (-> ['TMP_3914'])) := TMP_3918(uint32)
TMP_3914(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3914'])"])
 SettingChanged(vaultCollateralBuyForFlareFactorBIPS,_value)
Emit SettingChanged(vaultCollateralBuyForFlareFactorBIPS,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```
#### SettingsManagementFacet.setWithdrawalOrDestroyWaitMinSeconds(uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3853(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3853'])(AssetManagerSettings.Data) := TMP_3853(AssetManagerSettings.Data)
 require(bool,error)(_value > 0,revert CannotBeZero()())
TMP_3854(bool) = _value_1 > 0
TMP_3855(None) = SOLIDITY_CALL revert CannotBeZero()()
TMP_3856(None) = SOLIDITY_CALL require(bool,error)(TMP_3854,TMP_3855)
 require(bool,error)(_value <= settings.withdrawalWaitMinSeconds + 600,revert IncreaseTooBig()())
REF_2466(uint64) -> settings_1 (-> ['TMP_3853']).withdrawalWaitMinSeconds
TMP_3857(uint64) = REF_2466 (c)+ 600
TMP_3858(bool) = _value_1 <= TMP_3857
TMP_3859(None) = SOLIDITY_CALL revert IncreaseTooBig()()
TMP_3860(None) = SOLIDITY_CALL require(bool,error)(TMP_3858,TMP_3859)
 settings.withdrawalWaitMinSeconds = _value.toUint64()
REF_2467(uint64) -> settings_1 (-> ['TMP_3853']).withdrawalWaitMinSeconds
TMP_3861(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_value_1'] 
settings_2 (-> ['TMP_3853'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3853'])"])
REF_2467(uint64) (->settings_2 (-> ['TMP_3853'])) := TMP_3861(uint64)
TMP_3853(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3853'])"])
 SettingChanged(withdrawalWaitMinSeconds,_value)
Emit SettingChanged(withdrawalWaitMinSeconds,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
```

#### SettingsManagementFacet.updateSystemContracts(address,IWNat) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3590(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3590'])(AssetManagerSettings.Data) := TMP_3590(AssetManagerSettings.Data)
 settings.assetManagerController != _controller
REF_2372(address) -> settings_1 (-> ['TMP_3590']).assetManagerController
TMP_3591(bool) = REF_2372 != _controller_1
CONDITION TMP_3591
 settings.assetManagerController = _controller
REF_2373(address) -> settings_1 (-> ['TMP_3590']).assetManagerController
settings_2 (-> ['TMP_3590'])(AssetManagerSettings.Data) := phi(["settings_1 (-> ['TMP_3590'])"])
REF_2373(address) (->settings_2 (-> ['TMP_3590'])) := _controller_1(address)
TMP_3590(AssetManagerSettings.Data) := phi(["settings_2 (-> ['TMP_3590'])"])
 ContractChanged(assetManagerController,address(_controller))
TMP_3592 = CONVERT _controller_1 to address
Emit ContractChanged(assetManagerController,TMP_3592)
 oldWNat = Globals.getWNat()
TMP_3594(IWNat) = LIBRARY_CALL, dest:Globals, function:Globals.getWNat(), arguments:[] 
oldWNat_1(IWNat) := TMP_3594(IWNat)
 oldWNat != _wNat
TMP_3595(bool) = oldWNat_1 != _wNat_1
CONDITION TMP_3595
 data = CollateralTypes.getInfo(CollateralType.Class.POOL,oldWNat)
REF_2376(CollateralType.Class) -> Class.POOL
TMP_3596(CollateralType.Data) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.getInfo(CollateralType.Class,IERC20), arguments:['REF_2376', 'oldWNat_1'] 
data_1(CollateralType.Data) := TMP_3596(CollateralType.Data)
 data.validUntil = 0
REF_2377(uint256) -> data_1.validUntil
data_2(CollateralType.Data) := phi(['data_1'])
REF_2377(uint256) (->data_2) := 0(uint256)
 data.token = _wNat
REF_2378(IERC20) -> data_2.token
data_3(CollateralType.Data) := phi(['data_2'])
REF_2378(IERC20) (->data_3) := _wNat_1(IWNat)
 CollateralTypes.setPoolWNatCollateralType(data)
LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.setPoolWNatCollateralType(CollateralType.Data), arguments:['data_3'] 
 ContractChanged(wNat,address(_wNat))
TMP_3598 = CONVERT _wNat_1 to address
Emit ContractChanged(wNat,TMP_3598)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
```
#### SettingsManagementFacet.upgradeFAssetImplementation(address,bytes) [EXTERNAL]
```slithir
 fAssetProxy = IUpgradableProxy(address(Globals.getFAsset()))
TMP_3659(IIFAsset) = LIBRARY_CALL, dest:Globals, function:Globals.getFAsset(), arguments:[] 
TMP_3660 = CONVERT TMP_3659 to address
TMP_3661 = CONVERT TMP_3660 to IUpgradableProxy
fAssetProxy_1(IUpgradableProxy) := TMP_3661(IUpgradableProxy)
 require(bool,error)(_value != address(0),revert InvalidAddress()())
TMP_3662 = CONVERT 0 to address
TMP_3663(bool) = _value_1 != TMP_3662
TMP_3664(None) = SOLIDITY_CALL revert InvalidAddress()()
TMP_3665(None) = SOLIDITY_CALL require(bool,error)(TMP_3663,TMP_3664)
 callData.length > 0
REF_2398 -> LENGTH callData_1
TMP_3666(bool) = REF_2398 > 0
CONDITION TMP_3666
 fAssetProxy.upgradeToAndCall(_value,callData)
HIGH_LEVEL_CALL, dest:fAssetProxy_1(IUpgradableProxy), function:upgradeToAndCall, arguments:['_value_1', 'callData_1']  
 fAssetProxy.upgradeTo(_value)
HIGH_LEVEL_CALL, dest:fAssetProxy_1(IUpgradableProxy), function:upgradeTo, arguments:['_value_1']  
 ContractChanged(fAsset,_value)
Emit ContractChanged(fAsset,_value_1)
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
 rateLimited()
MODIFIER_CALL, SettingsManagementFacet.rateLimited()()
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
#### Globals.getFAsset() [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4735(AssetManagerSettings.Data) = INTERNAL_CALL, Globals.getSettings()()
settings_1 (-> ['TMP_4735'])(AssetManagerSettings.Data) := TMP_4735(AssetManagerSettings.Data)
 IIFAsset(settings.fAsset)
REF_3240(address) -> settings_1 (-> ['TMP_4735']).fAsset
TMP_4736 = CONVERT REF_3240 to IIFAsset
RETURN TMP_4736
```
#### IIFAsset.setCleanupBlockNumberManager(address) [EXTERNAL]
```slithir

```
#### SafeCast.toUint16(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint16).max,SafeCast: value doesn't fit in 16 bits)
TMP_777(uint16) := 65535(uint16)
TMP_778(bool) = value_1 <= TMP_777
TMP_779(None) = SOLIDITY_CALL require(bool,string)(TMP_778,SafeCast: value doesn't fit in 16 bits)
 uint16(value)
TMP_780 = CONVERT value_1 to uint16
RETURN TMP_780
```
#### SafeCast.toUint128(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint128).max,SafeCast: value doesn't fit in 128 bits)
TMP_707(uint128) := 340282366920938463463374607431768211455(uint128)
TMP_708(bool) = value_1 <= TMP_707
TMP_709(None) = SOLIDITY_CALL require(bool,string)(TMP_708,SafeCast: value doesn't fit in 128 bits)
 uint128(value)
TMP_710 = CONVERT value_1 to uint128
RETURN TMP_710
```
#### SettingsValidators.validateLiquidationFactors(uint256[],uint256[]) [INTERNAL]
```slithir
 require(bool,error)(liquidationFactors.length == vaultCollateralFactors.length,revert LengthsNotEqual()())
REF_3656 -> LENGTH liquidationFactors_1
REF_3657 -> LENGTH vaultCollateralFactors_1
TMP_5213(bool) = REF_3656 == REF_3657
TMP_5214(None) = SOLIDITY_CALL revert LengthsNotEqual()()
TMP_5215(None) = SOLIDITY_CALL require(bool,error)(TMP_5213,TMP_5214)
 require(bool,error)(liquidationFactors.length >= 1,revert AtLeastOneFactorRequired()())
REF_3658 -> LENGTH liquidationFactors_1
TMP_5216(bool) = REF_3658 >= 1
TMP_5217(None) = SOLIDITY_CALL revert AtLeastOneFactorRequired()()
TMP_5218(None) = SOLIDITY_CALL require(bool,error)(TMP_5216,TMP_5217)
 i = 0
i_1(uint256) := 0(uint256)
 i < liquidationFactors.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3659 -> LENGTH liquidationFactors_1
TMP_5219(bool) = i_2 < REF_3659
CONDITION TMP_5219
 require(bool,error)(liquidationFactors[i] > SafePct.MAX_BIPS,revert FactorNotAboveOne()())
REF_3660(uint256) -> liquidationFactors_1[i_2]
REF_3661(uint256) -> SafePct.MAX_BIPS
TMP_5220(bool) = REF_3660 > REF_3661
TMP_5221(None) = SOLIDITY_CALL revert FactorNotAboveOne()()
TMP_5222(None) = SOLIDITY_CALL require(bool,error)(TMP_5220,TMP_5221)
 require(bool,error)(vaultCollateralFactors[i] <= liquidationFactors[i],revert VaultCollateralFactorHigherThanTotal()())
REF_3662(uint256) -> vaultCollateralFactors_1[i_2]
REF_3663(uint256) -> liquidationFactors_1[i_2]
TMP_5223(bool) = REF_3662 <= REF_3663
TMP_5224(None) = SOLIDITY_CALL revert VaultCollateralFactorHigherThanTotal()()
TMP_5225(None) = SOLIDITY_CALL require(bool,error)(TMP_5223,TMP_5224)
 require(bool,error)(i == 0 || liquidationFactors[i] > liquidationFactors[i - 1],revert FactorsNotIncreasing()())
TMP_5226(bool) = i_2 == 0
REF_3664(uint256) -> liquidationFactors_1[i_2]
TMP_5227(uint256) = i_2 (c)- 1
REF_3665(uint256) -> liquidationFactors_1[TMP_5227]
TMP_5228(bool) = REF_3664 > REF_3665
TMP_5229(bool) = TMP_5226 || TMP_5228
TMP_5230(None) = SOLIDITY_CALL revert FactorsNotIncreasing()()
TMP_5231(None) = SOLIDITY_CALL require(bool,error)(TMP_5229,TMP_5230)
 i ++
TMP_5232(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
```
#### SettingsValidators.validateTimeForPayment(uint256,uint256,uint256) [INTERNAL]
```slithir
MAXIMUM_PROOF_WINDOW_1(uint256) := phi(['MAXIMUM_PROOF_WINDOW_0'])
 require(bool,error)(_underlyingSeconds <= MAXIMUM_PROOF_WINDOW,revert ValueTooHigh()())
TMP_5205(bool) = _underlyingSeconds_1 <= MAXIMUM_PROOF_WINDOW_1
TMP_5206(None) = SOLIDITY_CALL revert ValueTooHigh()()
TMP_5207(None) = SOLIDITY_CALL require(bool,error)(TMP_5205,TMP_5206)
 require(bool,error)(_underlyingBlocks * _averageBlockTimeMS / 1000 <= MAXIMUM_PROOF_WINDOW,revert ValueTooHigh()())
TMP_5208(uint256) = _underlyingBlocks_1 (c)* _averageBlockTimeMS_1
TMP_5209(uint256) = TMP_5208 (c)/ 1000
TMP_5210(bool) = TMP_5209 <= MAXIMUM_PROOF_WINDOW_1
TMP_5211(None) = SOLIDITY_CALL revert ValueTooHigh()()
TMP_5212(None) = SOLIDITY_CALL require(bool,error)(TMP_5210,TMP_5211)
```
#### CollateralTypes.getInfo(CollateralType.Class,IERC20) [INTERNAL]
```slithir
 token = CollateralTypes.get(_collateralClass,_token)
TMP_4555(CollateralTypeInt.Data) = INTERNAL_CALL, CollateralTypes.get(CollateralType.Class,IERC20)(_collateralClass_1,_token_1)
token_1 (-> ['TMP_4555'])(CollateralTypeInt.Data) := TMP_4555(CollateralTypeInt.Data)
 _getInfo(token)
TMP_4556(CollateralType.Data) = INTERNAL_CALL, CollateralTypes._getInfo(CollateralTypeInt.Data)(token_1 (-> ['TMP_4555']))
RETURN TMP_4556
```
#### CollateralTypes.setPoolWNatCollateralType(CollateralType.Data) [INTERNAL]
```slithir
 index = _add(_data)
TMP_4553(uint256) = INTERNAL_CALL, CollateralTypes._add(CollateralType.Data)(_data_1)
index_1(uint256) := TMP_4553(uint256)
 _setPoolCollateralTypeIndex(index)
INTERNAL_CALL, CollateralTypes._setPoolCollateralTypeIndex(uint256)(index_1)
```
#### Globals.getWNat() [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4731(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4731'])(AssetManagerState.State) := TMP_4731(AssetManagerState.State)
 IWNat(address(state.collateralTokens[state.poolCollateralIndex].token))
REF_3231(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4731']).collateralTokens
REF_3232(uint16) -> state_1 (-> ['TMP_4731']).poolCollateralIndex
REF_3233(CollateralTypeInt.Data) -> REF_3231[REF_3232]
REF_3234(IERC20) -> REF_3233.token
TMP_4732 = CONVERT REF_3234 to address
TMP_4733 = CONVERT TMP_4732 to IWNat
RETURN TMP_4733
```
#### IUpgradableProxy.upgradeTo(address) [EXTERNAL]
```slithir

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
#### CollateralTypes._getInfo(CollateralTypeInt.Data) [PRIVATE]
```slithir
token_1 (-> ['TMP_4555'])(CollateralTypeInt.Data) := phi(["token_1 (-> ['TMP_4555'])", 'REF_3060'])
 CollateralType.Data({token:token.token,collateralClass:token.collateralClass,decimals:token.decimals,validUntil:token.validUntil,directPricePair:token.directPricePair,assetFtsoSymbol:token.assetFtsoSymbol,tokenFtsoSymbol:token.tokenFtsoSymbol,minCollateralRatioBIPS:token.minCollateralRatioBIPS,safetyMinCollateralRatioBIPS:token.safetyMinCollateralRatioBIPS})
REF_3129(IERC20) -> token_1 (-> ['TMP_4555']).token
REF_3130(CollateralType.Class) -> token_1 (-> ['TMP_4555']).collateralClass
REF_3131(uint8) -> token_1 (-> ['TMP_4555']).decimals
REF_3132(uint64) -> token_1 (-> ['TMP_4555']).validUntil
REF_3133(bool) -> token_1 (-> ['TMP_4555']).directPricePair
REF_3134(string) -> token_1 (-> ['TMP_4555']).assetFtsoSymbol
REF_3135(string) -> token_1 (-> ['TMP_4555']).tokenFtsoSymbol
REF_3136(uint32) -> token_1 (-> ['TMP_4555']).minCollateralRatioBIPS
REF_3137(uint32) -> token_1 (-> ['TMP_4555']).safetyMinCollateralRatioBIPS
TMP_4622(CollateralType.Data) = new Data(REF_3130,REF_3129,REF_3131,REF_3132,REF_3133,REF_3134,REF_3135,REF_3136,REF_3137)
RETURN TMP_4622
```
#### CollateralTypes.get(CollateralType.Class,IERC20) [INTERNAL]
```slithir
_collateralClass_1(CollateralType.Class) := phi(['_collateralClass_1'])
_token_1(IERC20) := phi(['_token_1'])
 state = AssetManagerState.get()
TMP_4563(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4563'])(AssetManagerState.State) := TMP_4563(AssetManagerState.State)
 index = state.collateralTokenIndex[_tokenKey(_collateralClass,_token)]
REF_3062(mapping(bytes32 => uint256)) -> state_1 (-> ['TMP_4563']).collateralTokenIndex
TMP_4564(bytes32) = INTERNAL_CALL, CollateralTypes._tokenKey(CollateralType.Class,IERC20)(_collateralClass_1,_token_1)
REF_3063(uint256) -> REF_3062[TMP_4564]
index_1(uint256) := REF_3063(uint256)
 require(bool,error)(index > 0,revert UnknownToken()())
TMP_4565(bool) = index_1 > 0
TMP_4566(None) = SOLIDITY_CALL revert UnknownToken()()
TMP_4567(None) = SOLIDITY_CALL require(bool,error)(TMP_4565,TMP_4566)
 state.collateralTokens[index - 1]
REF_3064(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4563']).collateralTokens
TMP_4568(uint256) = index_1 (c)- 1
REF_3065(CollateralTypeInt.Data) -> REF_3064[TMP_4568]
RETURN REF_3065
```
#### CollateralTypes._add(CollateralType.Data) [PRIVATE]
```slithir
_data_1(CollateralType.Data) := phi(['_data_1', 'REF_3051', '_data_1', 'REF_3046'])
 state = AssetManagerState.get()
TMP_4581(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4581'])(AssetManagerState.State) := TMP_4581(AssetManagerState.State)
 require(bool,error)(address(_data.token) != address(0),revert TokenZero()())
REF_3075(IERC20) -> _data_1.token
TMP_4582 = CONVERT REF_3075 to address
TMP_4583 = CONVERT 0 to address
TMP_4584(bool) = TMP_4582 != TMP_4583
TMP_4585(None) = SOLIDITY_CALL revert TokenZero()()
TMP_4586(None) = SOLIDITY_CALL require(bool,error)(TMP_4584,TMP_4585)
 tokenKey = _tokenKey(_data.collateralClass,_data.token)
REF_3076(CollateralType.Class) -> _data_1.collateralClass
REF_3077(IERC20) -> _data_1.token
TMP_4587(bytes32) = INTERNAL_CALL, CollateralTypes._tokenKey(CollateralType.Class,IERC20)(REF_3076,REF_3077)
tokenKey_1(bytes32) := TMP_4587(bytes32)
 require(bool,error)(state.collateralTokenIndex[tokenKey] == 0,revert TokenAlreadyExists()())
REF_3078(mapping(bytes32 => uint256)) -> state_1 (-> ['TMP_4581']).collateralTokenIndex
REF_3079(uint256) -> REF_3078[tokenKey_1]
TMP_4588(bool) = REF_3079 == 0
TMP_4589(None) = SOLIDITY_CALL revert TokenAlreadyExists()()
TMP_4590(None) = SOLIDITY_CALL require(bool,error)(TMP_4588,TMP_4589)
 require(bool,error)(_data.validUntil == 0,revert CannotAddDeprecatedToken()())
REF_3080(uint256) -> _data_1.validUntil
TMP_4591(bool) = REF_3080 == 0
TMP_4592(None) = SOLIDITY_CALL revert CannotAddDeprecatedToken()()
TMP_4593(None) = SOLIDITY_CALL require(bool,error)(TMP_4591,TMP_4592)
 ratiosValid = SafePct.MAX_BIPS < _data.minCollateralRatioBIPS && _data.minCollateralRatioBIPS <= _data.safetyMinCollateralRatioBIPS
REF_3081(uint256) -> SafePct.MAX_BIPS
REF_3082(uint256) -> _data_1.minCollateralRatioBIPS
TMP_4594(bool) = REF_3081 < REF_3082
REF_3083(uint256) -> _data_1.minCollateralRatioBIPS
REF_3084(uint256) -> _data_1.safetyMinCollateralRatioBIPS
TMP_4595(bool) = REF_3083 <= REF_3084
TMP_4596(bool) = TMP_4594 && TMP_4595
ratiosValid_1(bool) := TMP_4596(bool)
 require(bool,error)(ratiosValid,revert InvalidCollateralRatios()())
TMP_4597(None) = SOLIDITY_CALL revert InvalidCollateralRatios()()
TMP_4598(None) = SOLIDITY_CALL require(bool,error)(ratiosValid_1,TMP_4597)
 (assetPrice,None,None) = Conversion.readFtsoPrice(_data.assetFtsoSymbol,false)
REF_3086(string) -> _data_1.assetFtsoSymbol
TUPLE_41(uint256,uint256,uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.readFtsoPrice(string,bool), arguments:['REF_3086', 'False'] 
assetPrice_1(uint256)= UNPACK TUPLE_41 index: 0 
 require(bool,error)(assetPrice != 0,revert PriceNotInitialized()())
TMP_4599(bool) = assetPrice_1 != 0
TMP_4600(None) = SOLIDITY_CALL revert PriceNotInitialized()()
TMP_4601(None) = SOLIDITY_CALL require(bool,error)(TMP_4599,TMP_4600)
 ! _data.directPricePair
REF_3087(bool) -> _data_1.directPricePair
TMP_4602 = UnaryType.BANG REF_3087 
CONDITION TMP_4602
 (tokenPrice,None,None) = Conversion.readFtsoPrice(_data.tokenFtsoSymbol,false)
REF_3089(string) -> _data_1.tokenFtsoSymbol
TUPLE_42(uint256,uint256,uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.readFtsoPrice(string,bool), arguments:['REF_3089', 'False'] 
tokenPrice_1(uint256)= UNPACK TUPLE_42 index: 0 
 require(bool,error)(tokenPrice != 0,revert PriceNotInitialized()())
TMP_4603(bool) = tokenPrice_1 != 0
TMP_4604(None) = SOLIDITY_CALL revert PriceNotInitialized()()
TMP_4605(None) = SOLIDITY_CALL require(bool,error)(TMP_4603,TMP_4604)
 newTokenIndex = state.collateralTokens.length
REF_3090(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4581']).collateralTokens
REF_3091 -> LENGTH REF_3090
newTokenIndex_1(uint256) := REF_3091(uint256)
 state.collateralTokens.push(CollateralTypeInt.Data({token:_data.token,collateralClass:_data.collateralClass,decimals:_data.decimals.toUint8(),validUntil:_data.validUntil.toUint64(),directPricePair:_data.directPricePair,assetFtsoSymbol:_data.assetFtsoSymbol,tokenFtsoSymbol:_data.tokenFtsoSymbol,minCollateralRatioBIPS:_data.minCollateralRatioBIPS.toUint32(),__ccbMinCollateralRatioBIPS:0,safetyMinCollateralRatioBIPS:_data.safetyMinCollateralRatioBIPS.toUint32()}))
REF_3092(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4581']).collateralTokens
REF_3095(IERC20) -> _data_1.token
REF_3096(CollateralType.Class) -> _data_1.collateralClass
REF_3097(uint256) -> _data_1.decimals
TMP_4606(uint8) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint8(uint256), arguments:['REF_3097'] 
REF_3099(uint256) -> _data_1.validUntil
TMP_4607(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['REF_3099'] 
REF_3101(bool) -> _data_1.directPricePair
REF_3102(string) -> _data_1.assetFtsoSymbol
REF_3103(string) -> _data_1.tokenFtsoSymbol
REF_3104(uint256) -> _data_1.minCollateralRatioBIPS
TMP_4608(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['REF_3104'] 
REF_3106(uint256) -> _data_1.safetyMinCollateralRatioBIPS
TMP_4609(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['REF_3106'] 
TMP_4610(CollateralTypeInt.Data) = new Data(REF_3095,REF_3096,TMP_4606,TMP_4607,REF_3101,REF_3102,REF_3103,TMP_4608,0,TMP_4609)
REF_3108 -> LENGTH REF_3092
TMP_4612(uint256) := REF_3108(uint256)
TMP_4613(uint256) = TMP_4612 (c)+ 1
state_2 (-> ['TMP_4581'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_4581'])"])
REF_3108(uint256) (->state_3 (-> ['TMP_4581'])) := TMP_4613(uint256)
REF_3109(CollateralTypeInt.Data) -> REF_3092[TMP_4612]
state_3 (-> ['TMP_4581'])(AssetManagerState.State) := phi(["state_2 (-> ['TMP_4581'])"])
REF_3109(CollateralTypeInt.Data) (->state_3 (-> ['TMP_4581'])) := TMP_4610(CollateralTypeInt.Data)
TMP_4581(AssetManagerState.State) := phi(["state_3 (-> ['TMP_4581'])"])
 state.collateralTokenIndex[tokenKey] = newTokenIndex + 1
REF_3110(mapping(bytes32 => uint256)) -> state_3 (-> ['TMP_4581']).collateralTokenIndex
REF_3111(uint256) -> REF_3110[tokenKey_1]
TMP_4614(uint256) = newTokenIndex_1 (c)+ 1
state_4 (-> ['TMP_4581'])(AssetManagerState.State) := phi(["state_3 (-> ['TMP_4581'])"])
REF_3111(uint256) (->state_4 (-> ['TMP_4581'])) := TMP_4614(uint256)
TMP_4581(AssetManagerState.State) := phi(["state_4 (-> ['TMP_4581'])"])
 IAssetManagerEvents.CollateralTypeAdded(uint8(_data.collateralClass),address(_data.token),_data.decimals,_data.directPricePair,_data.assetFtsoSymbol,_data.tokenFtsoSymbol,_data.minCollateralRatioBIPS,_data.safetyMinCollateralRatioBIPS)
REF_3113(CollateralType.Class) -> _data_1.collateralClass
TMP_4615 = CONVERT REF_3113 to uint8
REF_3114(IERC20) -> _data_1.token
TMP_4616 = CONVERT REF_3114 to address
REF_3115(uint256) -> _data_1.decimals
REF_3116(bool) -> _data_1.directPricePair
REF_3117(string) -> _data_1.assetFtsoSymbol
REF_3118(string) -> _data_1.tokenFtsoSymbol
REF_3119(uint256) -> _data_1.minCollateralRatioBIPS
REF_3120(uint256) -> _data_1.safetyMinCollateralRatioBIPS
Emit CollateralTypeAdded(TMP_4615,TMP_4616,REF_3115,REF_3116,REF_3117,REF_3118,REF_3119,REF_3120)
 newTokenIndex
RETURN newTokenIndex_1
```
#### CollateralTypes._setPoolCollateralTypeIndex(uint256) [PRIVATE]
```slithir
_index_1(uint256) := phi(['index_1'])
 state = AssetManagerState.get()
TMP_4618(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4618'])(AssetManagerState.State) := TMP_4618(AssetManagerState.State)
 token = state.collateralTokens[_index]
REF_3122(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4618']).collateralTokens
REF_3123(CollateralTypeInt.Data) -> REF_3122[_index_1]
token_1 (-> ['state'])(CollateralTypeInt.Data) := REF_3123(CollateralTypeInt.Data)
 assert(bool)(token.collateralClass == CollateralType.Class.POOL)
REF_3124(CollateralType.Class) -> token_1 (-> ['state']).collateralClass
REF_3125(CollateralType.Class) -> Class.POOL
TMP_4619(bool) = REF_3124 == REF_3125
TMP_4620(None) = SOLIDITY_CALL assert(bool)(TMP_4619)
 state.poolCollateralIndex = _index.toUint16()
REF_3126(uint16) -> state_1 (-> ['TMP_4618']).poolCollateralIndex
TMP_4621(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_index_1'] 
state_2 (-> ['TMP_4618'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_4618'])"])
REF_3126(uint16) (->state_2 (-> ['TMP_4618'])) := TMP_4621(uint16)
TMP_4618(AssetManagerState.State) := phi(["state_2 (-> ['TMP_4618'])"])
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
#### SafeCast.toUint8(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint8).max,SafeCast: value doesn't fit in 8 bits)
TMP_782(uint8) := 255(uint8)
TMP_783(bool) = value_1 <= TMP_782
TMP_784(None) = SOLIDITY_CALL require(bool,string)(TMP_783,SafeCast: value doesn't fit in 8 bits)
 uint8(value)
TMP_785 = CONVERT value_1 to uint8
RETURN TMP_785
```
