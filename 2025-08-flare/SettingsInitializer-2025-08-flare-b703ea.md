

#### SettingsInitializer._setAllSettings(AssetManagerSettings.Data) [PRIVATE]
```slithir
_settings_1(AssetManagerSettings.Data) := phi(['_settings_1'])
 position = Globals.ASSET_MANAGER_SETTINGS_POSITION
REF_3593(bytes32) -> Globals.ASSET_MANAGER_SETTINGS_POSITION
position_1(bytes32) := REF_3593(bytes32)
 wrapper = position
wrapper_1 (-> ['position'])(SettingsInitializer.SettingsWrapper) := position_1(bytes32)
 wrapper.settings = _settings
REF_3594(AssetManagerSettings.Data) -> wrapper_1 (-> ['position']).settings
wrapper_2 (-> ['position'])(SettingsInitializer.SettingsWrapper) := phi(["wrapper_1 (-> ['position'])"])
REF_3594(AssetManagerSettings.Data) (->wrapper_2 (-> ['position'])) := _settings_1(AssetManagerSettings.Data)
position_2(bytes32) := phi(["wrapper_2 (-> ['position'])"])
```

#### SettingsInitializer.validateAndSet(AssetManagerSettings.Data) [INTERNAL]
```slithir
 _validateSettings(_settings)
INTERNAL_CALL, SettingsInitializer._validateSettings(AssetManagerSettings.Data)(_settings_1)
 _setAllSettings(_settings)
INTERNAL_CALL, SettingsInitializer._setAllSettings(AssetManagerSettings.Data)(_settings_1)
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
