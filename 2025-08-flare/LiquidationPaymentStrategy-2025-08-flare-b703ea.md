






#### LiquidationPaymentStrategy.currentLiquidationFactorBIPS(Agent.State,uint256,uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4783(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4783'])(AssetManagerSettings.Data) := TMP_4783(AssetManagerSettings.Data)
 step = _currentLiquidationStep(_agent)
TMP_4784(uint256) = INTERNAL_CALL, LiquidationPaymentStrategy._currentLiquidationStep(Agent.State)(_agent_1 (-> []))
step_1(uint256) := TMP_4784(uint256)
 factorBIPS = settings.liquidationCollateralFactorBIPS[step]
REF_3298(uint256[]) -> settings_1 (-> ['TMP_4783']).liquidationCollateralFactorBIPS
REF_3299(uint256) -> REF_3298[step_1]
factorBIPS_1(uint256) := REF_3299(uint256)
 _c1FactorBIPS = Math.min(settings.liquidationFactorVaultCollateralBIPS[step],factorBIPS)
REF_3301(uint256[]) -> settings_1 (-> ['TMP_4783']).liquidationFactorVaultCollateralBIPS
REF_3302(uint256) -> REF_3301[step_1]
TMP_4785(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['REF_3302', 'factorBIPS_1'] 
_c1FactorBIPS_1(uint256) := TMP_4785(uint256)
 vaultCollateral = _agent.getVaultCollateral()
TMP_4786(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
vaultCollateral_1 (-> ['TMP_4786'])(CollateralTypeInt.Data) := TMP_4786(CollateralTypeInt.Data)
 poolCollateral = _agent.getPoolCollateral()
TMP_4787(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getPoolCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
poolCollateral_1 (-> ['TMP_4787'])(CollateralTypeInt.Data) := TMP_4787(CollateralTypeInt.Data)
 ! vaultCollateral.isValid() && poolCollateral.isValid()
TMP_4788(bool) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.isValid(CollateralTypeInt.Data), arguments:["vaultCollateral_1 (-> ['TMP_4786'])"] 
TMP_4789 = UnaryType.BANG TMP_4788 
TMP_4790(bool) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.isValid(CollateralTypeInt.Data), arguments:["poolCollateral_1 (-> ['TMP_4787'])"] 
TMP_4791(bool) = TMP_4789 && TMP_4790
CONDITION TMP_4791
 _c1FactorBIPS = 0
_c1FactorBIPS_2(uint256) := 0(uint256)
 vaultCollateral.isValid() && ! poolCollateral.isValid()
TMP_4792(bool) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.isValid(CollateralTypeInt.Data), arguments:["vaultCollateral_1 (-> ['TMP_4786'])"] 
TMP_4793(bool) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.isValid(CollateralTypeInt.Data), arguments:["poolCollateral_1 (-> ['TMP_4787'])"] 
TMP_4794 = UnaryType.BANG TMP_4793 
TMP_4795(bool) = TMP_4792 && TMP_4794
CONDITION TMP_4795
 _c1FactorBIPS = factorBIPS
_c1FactorBIPS_3(uint256) := factorBIPS_1(uint256)
_c1FactorBIPS_4(uint256) := phi(['_c1FactorBIPS_3', '_c1FactorBIPS_1'])
_c1FactorBIPS_5(uint256) := phi(['_c1FactorBIPS_2', '_c1FactorBIPS_1'])
 _c1FactorBIPS > _vaultCR
TMP_4796(bool) = _c1FactorBIPS_5 > _vaultCR_1
CONDITION TMP_4796
 _c1FactorBIPS = _vaultCR
_c1FactorBIPS_6(uint256) := _vaultCR_1(uint256)
_c1FactorBIPS_7(uint256) := phi(['_c1FactorBIPS_6', '_c1FactorBIPS_1'])
 _poolFactorBIPS = factorBIPS - _c1FactorBIPS
TMP_4797(uint256) = factorBIPS_1 (c)- _c1FactorBIPS_7
_poolFactorBIPS_1(uint256) := TMP_4797(uint256)
 _poolFactorBIPS > _poolCR
TMP_4798(bool) = _poolFactorBIPS_1 > _poolCR_1
CONDITION TMP_4798
 _poolFactorBIPS = _poolCR
_poolFactorBIPS_2(uint256) := _poolCR_1(uint256)
 _c1FactorBIPS = Math.min(factorBIPS - _poolFactorBIPS,_vaultCR)
TMP_4799(uint256) = factorBIPS_1 (c)- _poolFactorBIPS_2
TMP_4800(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['TMP_4799', '_vaultCR_1'] 
_c1FactorBIPS_8(uint256) := TMP_4800(uint256)
_c1FactorBIPS_9(uint256) := phi(['_c1FactorBIPS_8', '_c1FactorBIPS_1'])
_poolFactorBIPS_3(uint256) := phi(['_poolFactorBIPS_2', '_poolFactorBIPS_1'])
 (_c1FactorBIPS,_poolFactorBIPS)
RETURN _c1FactorBIPS_9,_poolFactorBIPS_3
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
#### Agents.getPoolCollateral(Agent.State) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4516(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4516'])(AssetManagerState.State) := TMP_4516(AssetManagerState.State)
 state.collateralTokens[_agent.poolCollateralIndex]
REF_3015(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4516']).collateralTokens
REF_3016(uint16) -> _agent_1 (-> []).poolCollateralIndex
REF_3017(CollateralTypeInt.Data) -> REF_3015[REF_3016]
RETURN REF_3017
```
#### Agents.getVaultCollateral(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 state = AssetManagerState.get()
TMP_4510(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4510'])(AssetManagerState.State) := TMP_4510(AssetManagerState.State)
 state.collateralTokens[_agent.vaultCollateralIndex]
REF_3005(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4510']).collateralTokens
REF_3006(uint16) -> _agent_1 (-> []).vaultCollateralIndex
REF_3007(CollateralTypeInt.Data) -> REF_3005[REF_3006]
RETURN REF_3007
```
#### CollateralTypes.isValid(CollateralTypeInt.Data) [INTERNAL]
```slithir
 _token.validUntil == 0 || _token.validUntil > block.timestamp
REF_3072(uint64) -> _token_1 (-> []).validUntil
TMP_4578(bool) = REF_3072 == 0
REF_3073(uint64) -> _token_1 (-> []).validUntil
TMP_4579(bool) = REF_3073 > block.timestamp
TMP_4580(bool) = TMP_4578 || TMP_4579
RETURN TMP_4580
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
