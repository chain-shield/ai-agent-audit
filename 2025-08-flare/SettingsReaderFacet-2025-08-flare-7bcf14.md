

#### SettingsReaderFacet.assetManagerController() [EXTERNAL]
```slithir
 Globals.getSettings().assetManagerController
TMP_4071(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_2567(address) -> TMP_4071.assetManagerController
RETURN REF_2567
```
#### SettingsReaderFacet.assetMintingDecimals() [EXTERNAL]
```slithir
 Globals.getSettings().assetMintingDecimals
TMP_4070(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_2565(uint8) -> TMP_4070.assetMintingDecimals
RETURN REF_2565
```
#### SettingsReaderFacet.assetMintingGranularityUBA() [EXTERNAL]
```slithir
 Globals.getSettings().assetMintingGranularityUBA
TMP_4069(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_2563(uint64) -> TMP_4069.assetMintingGranularityUBA
RETURN REF_2563
```
#### SettingsReaderFacet.fAsset() [EXTERNAL]
```slithir
 IERC20(address(Globals.getFAsset()))
TMP_4062(IIFAsset) = LIBRARY_CALL, dest:Globals, function:Globals.getFAsset(), arguments:[] 
TMP_4063 = CONVERT TMP_4062 to address
TMP_4064 = CONVERT TMP_4063 to IERC20
RETURN TMP_4064
```

#### SettingsReaderFacet.getSettings() [EXTERNAL]
```slithir
 Globals.getSettings()
TMP_4061(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
RETURN TMP_4061
```
#### SettingsReaderFacet.lotSize() [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4066(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4066'])(AssetManagerSettings.Data) := TMP_4066(AssetManagerSettings.Data)
 uint256(settings.lotSizeAMG) * settings.assetMintingGranularityUBA
REF_2560(uint64) -> settings_1 (-> ['TMP_4066']).lotSizeAMG
TMP_4067 = CONVERT REF_2560 to uint256
REF_2561(uint64) -> settings_1 (-> ['TMP_4066']).assetMintingGranularityUBA
TMP_4068(uint256) = TMP_4067 (c)* REF_2561
RETURN TMP_4068
 _lotSizeUBA
```
#### SettingsReaderFacet.priceReader() [EXTERNAL]
```slithir
 Globals.getSettings().priceReader
TMP_4065(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_2558(address) -> TMP_4065.priceReader
RETURN REF_2558
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
