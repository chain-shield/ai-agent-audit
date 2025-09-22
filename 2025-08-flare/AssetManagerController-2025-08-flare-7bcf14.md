### Storage layout (AssetManagerController) 

```text
replacedBy address
assetManagerIndex mapping(address => uint256)
assetManagers IIAssetManager[]
emergencyPauseSenders EnumerableSet.AddressSet

```


### Storage layout (GovernanceSettingsMock) 

```text
governanceAddress address
timelock uint64
initialised bool
executors address[]
executorMap mapping(address => bool)

```





### Storage layout (AddressUpdaterMock) 

```text
contractNames string[]
contractAddresses mapping(bytes32 => address)

```

#### AssetManagerController._authorizeUpgrade(address) [INTERNAL]
```slithir
 assert(bool)(false)
TMP_5725(None) = SOLIDITY_CALL assert(bool)(False)
```
#### AssetManagerController._callOnManagers(IIAssetManager[],bytes) [PRIVATE]
```slithir
_assetManagers_1(IIAssetManager[]) := phi(['_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1'])
_calldata_1(bytes) := phi(['TMP_5881', 'TMP_5861', 'TMP_5871', 'TMP_5817', 'TMP_5814', 'TMP_5894', 'TMP_5879', 'TMP_5811', 'TMP_5896', 'TMP_5755', 'TMP_5734', 'TMP_5883', 'TMP_5800', 'TMP_5808', 'TMP_5805', 'TMP_5745', 'TMP_5752'])
assetManagerIndex_9(mapping(address => uint256)) := phi(['assetManagerIndex_5', 'assetManagerIndex_7', 'assetManagerIndex_3', 'assetManagerIndex_8', 'assetManagerIndex_9', 'assetManagerIndex_0', 'assetManagerIndex_2'])
 i = 0
i_1(uint256) := 0(uint256)
 i < _assetManagers.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4122 -> LENGTH _assetManagers_1
TMP_5898(bool) = i_2 < REF_4122
CONDITION TMP_5898
 assetManager = address(_assetManagers[i])
REF_4123(IIAssetManager) -> _assetManagers_1[i_2]
TMP_5899 = CONVERT REF_4123 to address
assetManager_1(address) := TMP_5899(address)
 require(bool,error)(assetManagerIndex[assetManager] != 0,revert AssetManagerNotManaged()())
REF_4124(uint256) -> assetManagerIndex_9[assetManager_1]
TMP_5900(bool) = REF_4124 != 0
TMP_5901(None) = SOLIDITY_CALL revert AssetManagerNotManaged()()
TMP_5902(None) = SOLIDITY_CALL require(bool,error)(TMP_5900,TMP_5901)
 Address.functionCall(assetManager,_calldata)
TMP_5903(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionCall(address,bytes), arguments:['assetManager_1', '_calldata_1'] 
 i ++
TMP_5904(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256) [PRIVATE]
```slithir
_assetManagers_1(IIAssetManager[]) := phi(['_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1', '_assetManagers_1'])
_selector_1(bytes4) := phi(['REF_4050', 'REF_4119', 'REF_4076', 'REF_4042', 'REF_4068', 'REF_4060', 'REF_4052', 'REF_4078', 'REF_4044', 'REF_4070', 'REF_4030', 'REF_4062', 'REF_4054', 'REF_4046', 'REF_4038', 'REF_4072', 'REF_4032', 'REF_4064', 'REF_4056', 'REF_4048', 'REF_4040', 'REF_4117', 'REF_4074', 'REF_4082', 'REF_4066', 'REF_4058'])
_value_1(uint256) := phi(['_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1', '_value_1'])
 _callOnManagers(_assetManagers,abi.encodeWithSelector(_selector,(_value)))
TMP_5896(bytes) = SOLIDITY_CALL abi.encodeWithSelector()(_selector_1,_value_1)
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5896)
```
#### AssetManagerController._updateContractAddresses(bytes32[],address[]) [INTERNAL]
```slithir
_contractNameHashes_1(bytes32[]) := phi(['_contractNameHashes_1'])
_contractAddresses_1(address[]) := phi(['_contractAddresses_1'])
assetManagers_11(IIAssetManager[]) := phi(['assetManagers_2', 'assetManagers_4', 'assetManagers_10', 'assetManagers_6', 'assetManagers_9', 'assetManagers_15', 'assetManagers_0'])
 addressUpdater = _getContractAddress(_contractNameHashes,_contractAddresses,AddressUpdater)
TMP_5853(address) = INTERNAL_CALL, AddressUpdatable._getContractAddress(bytes32[],address[],string)(_contractNameHashes_1,_contractAddresses_1,AddressUpdater)
addressUpdater_1(address) := TMP_5853(address)
 assetManagerController = _getContractAddress(_contractNameHashes,_contractAddresses,AssetManagerController)
TMP_5854(address) = INTERNAL_CALL, AddressUpdatable._getContractAddress(bytes32[],address[],string)(_contractNameHashes_1,_contractAddresses_1,AssetManagerController)
assetManagerController_1(address) := TMP_5854(address)
 wNat = _getContractAddress(_contractNameHashes,_contractAddresses,WNat)
TMP_5855(address) = INTERNAL_CALL, AddressUpdatable._getContractAddress(bytes32[],address[],string)(_contractNameHashes_1,_contractAddresses_1,WNat)
wNat_1(address) := TMP_5855(address)
 _updateContracts(assetManagers,addressUpdater,assetManagerController,wNat)
INTERNAL_CALL, AssetManagerController._updateContracts(IIAssetManager[],address,address,address)(assetManagers_14,addressUpdater_1,assetManagerController_1,wNat_1)
```
#### AssetManagerController._updateContracts(IIAssetManager[],address,address,address) [PRIVATE]
```slithir
_assetManagers_1(IIAssetManager[]) := phi(['assetManagers_14', '_assetManagers_1'])
addressUpdater_1(address) := phi(['newAddressUpdater_1', 'addressUpdater_1'])
assetManagerController_1(address) := phi(['assetManagerController_1', 'assetManagerController_1'])
wNat_1(address) := phi(['wNat_1', 'wNat_1'])
 addressUpdater != getAddressUpdater()
TMP_5857(address) = INTERNAL_CALL, AddressUpdatable.getAddressUpdater()()
TMP_5858(bool) = addressUpdater_1 != TMP_5857
CONDITION TMP_5858
 setAddressUpdaterValue(addressUpdater)
INTERNAL_CALL, AddressUpdatable.setAddressUpdaterValue(address)(addressUpdater_1)
 _callOnManagers(_assetManagers,abi.encodeCall(IISettingsManagement.updateSystemContracts,(assetManagerController,IWNat(wNat))))
REF_4103(updateSystemContracts) -> IISettingsManagement.updateSystemContracts
TMP_5860 = CONVERT wNat_1 to IWNat
TMP_5861(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4103,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83188dc0>, <slither.slithir.variables.temporary_ssa.TemporaryVariableSSA object at 0xffff831897b0>])
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5861)
 assetManagerController != address(this)
TMP_5863 = CONVERT this to address
TMP_5864(bool) = assetManagerController_1 != TMP_5863
CONDITION TMP_5864
 replacedBy = assetManagerController
replacedBy_1(address) := assetManagerController_1(address)
```
#### AssetManagerController.addAssetManager(IIAssetManager) [EXTERNAL]
```slithir
assetManagerIndex_1(mapping(address => uint256)) := phi(['assetManagerIndex_5', 'assetManagerIndex_7', 'assetManagerIndex_3', 'assetManagerIndex_8', 'assetManagerIndex_9', 'assetManagerIndex_0', 'assetManagerIndex_2'])
assetManagers_1(IIAssetManager[]) := phi(['assetManagers_2', 'assetManagers_4', 'assetManagers_10', 'assetManagers_6', 'assetManagers_9', 'assetManagers_15', 'assetManagers_0'])
 assetManagerIndex[address(_assetManager)] != 0
TMP_5686 = CONVERT _assetManager_1 to address
REF_3984(uint256) -> assetManagerIndex_2[TMP_5686]
TMP_5687(bool) = REF_3984 != 0
CONDITION TMP_5687
 assetManagers.push(_assetManager)
REF_3986 -> LENGTH assetManagers_2
TMP_5689(uint256) := REF_3986(uint256)
TMP_5690(uint256) = TMP_5689 (c)+ 1
assetManagers_3(IIAssetManager[]) := phi(['assetManagers_2'])
REF_3986(uint256) (->assetManagers_3) := TMP_5690(uint256)
REF_3987(IIAssetManager) -> assetManagers_3[TMP_5689]
assetManagers_4(IIAssetManager[]) := phi(['assetManagers_3'])
REF_3987(IIAssetManager) (->assetManagers_4) := _assetManager_1(IIAssetManager)
 assetManagerIndex[address(_assetManager)] = assetManagers.length
TMP_5691 = CONVERT _assetManager_1 to address
REF_3988(uint256) -> assetManagerIndex_2[TMP_5691]
REF_3989 -> LENGTH assetManagers_4
assetManagerIndex_3(mapping(address => uint256)) := phi(['assetManagerIndex_2'])
REF_3988(uint256) (->assetManagerIndex_3) := REF_3989(uint256)
 _assetManager.assetManagerController() == address(this)
TMP_5692(address) = HIGH_LEVEL_CALL, dest:_assetManager_1(IIAssetManager), function:assetManagerController, arguments:[]  
TMP_5693 = CONVERT this to address
TMP_5694(bool) = TMP_5692 == TMP_5693
CONDITION TMP_5694
 _assetManager.attachController(true)
HIGH_LEVEL_CALL, dest:_assetManager_1(IIAssetManager), function:attachController, arguments:['True']  
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.addCollateralType(IIAssetManager[],CollateralType.Data) [EXTERNAL]
```slithir
 _callOnManagers(_assetManagers,abi.encodeCall(IIAssetManager.addCollateralType,(_data)))
REF_4084(addCollateralType) -> IIAssetManager.addCollateralType
TMP_5805(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4084,_data_1)
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5805)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.addEmergencyPauseSender(address) [EXTERNAL]
```slithir
emergencyPauseSenders_5(EnumerableSet.AddressSet) := phi(['emergencyPauseSenders_2', 'emergencyPauseSenders_0', 'emergencyPauseSenders_8', 'emergencyPauseSenders_4', 'emergencyPauseSenders_6'])
 emergencyPauseSenders.add(_address)
TMP_5886(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.add(EnumerableSet.AddressSet,address), arguments:['emergencyPauseSenders_6', '_address_1'] 
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.assetManagerExists(address) [EXTERNAL]
```slithir
assetManagerIndex_8(mapping(address => uint256)) := phi(['assetManagerIndex_5', 'assetManagerIndex_7', 'assetManagerIndex_3', 'assetManagerIndex_8', 'assetManagerIndex_9', 'assetManagerIndex_0', 'assetManagerIndex_2'])
 assetManagerIndex[_assetManager] != 0
REF_4008(uint256) -> assetManagerIndex_8[_assetManager_1]
TMP_5716(bool) = REF_4008 != 0
RETURN TMP_5716
```
#### AssetManagerController.constructor() [PUBLIC]
```slithir
 GovernedProxyImplementation()
INTERNAL_CALL, GovernedProxyImplementation.constructor()()
 AddressUpdatable(address(0))
TMP_5682 = CONVERT 0 to address
INTERNAL_CALL, AddressUpdatable.constructor(address)(TMP_5682)
```
#### AssetManagerController.deprecateCollateralType(IIAssetManager[],CollateralType.Class,IERC20,uint256) [EXTERNAL]
```slithir
 _callOnManagers(_assetManagers,abi.encodeCall(IIAssetManager.deprecateCollateralType,(_class,_token,_invalidationTimeSec)))
REF_4088(deprecateCollateralType) -> IIAssetManager.deprecateCollateralType
TMP_5811(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4088,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83152830>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83152b00>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83152dd0>])
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5811)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.emergencyPause(IIAssetManager[],uint256) [EXTERNAL]
```slithir
emergencyPauseSenders_1(EnumerableSet.AddressSet) := phi(['emergencyPauseSenders_2', 'emergencyPauseSenders_0', 'emergencyPauseSenders_8', 'emergencyPauseSenders_4', 'emergencyPauseSenders_6'])
 byGovernance = msg.sender == governance()
TMP_5865(address) = INTERNAL_CALL, GovernedBase.governance()()
TMP_5866(bool) = msg.sender == TMP_5865
byGovernance_1(bool) := TMP_5866(bool)
 require(bool,error)(byGovernance || emergencyPauseSenders.contains(msg.sender),revert OnlyGovernanceOrEmergencyPauseSenders()())
TMP_5867(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.contains(EnumerableSet.AddressSet,address), arguments:['emergencyPauseSenders_2', 'msg.sender'] 
TMP_5868(bool) = byGovernance_1 || TMP_5867
TMP_5869(None) = SOLIDITY_CALL revert OnlyGovernanceOrEmergencyPauseSenders()()
TMP_5870(None) = SOLIDITY_CALL require(bool,error)(TMP_5868,TMP_5869)
 _callOnManagers(_assetManagers,abi.encodeCall(IIAssetManager.emergencyPause,(byGovernance,_duration)))
REF_4106(emergencyPause) -> IIAssetManager.emergencyPause
TMP_5871(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4106,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83189d50>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff831894b0>])
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5871)
```
#### AssetManagerController.emergencyPauseTransfers(IIAssetManager[],uint256) [EXTERNAL]
```slithir
emergencyPauseSenders_3(EnumerableSet.AddressSet) := phi(['emergencyPauseSenders_2', 'emergencyPauseSenders_0', 'emergencyPauseSenders_8', 'emergencyPauseSenders_4', 'emergencyPauseSenders_6'])
 byGovernance = msg.sender == governance()
TMP_5873(address) = INTERNAL_CALL, GovernedBase.governance()()
TMP_5874(bool) = msg.sender == TMP_5873
byGovernance_1(bool) := TMP_5874(bool)
 require(bool,error)(byGovernance || emergencyPauseSenders.contains(msg.sender),revert OnlyGovernanceOrEmergencyPauseSenders()())
TMP_5875(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.contains(EnumerableSet.AddressSet,address), arguments:['emergencyPauseSenders_4', 'msg.sender'] 
TMP_5876(bool) = byGovernance_1 || TMP_5875
TMP_5877(None) = SOLIDITY_CALL revert OnlyGovernanceOrEmergencyPauseSenders()()
TMP_5878(None) = SOLIDITY_CALL require(bool,error)(TMP_5876,TMP_5877)
 _callOnManagers(_assetManagers,abi.encodeCall(IIAssetManager.emergencyPauseTransfers,(byGovernance,_duration)))
REF_4109(emergencyPauseTransfers) -> IIAssetManager.emergencyPauseTransfers
TMP_5879(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4109,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff8318a3b0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83189a50>])
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5879)
```
#### AssetManagerController.getAssetManagers() [EXTERNAL]
```slithir
assetManagers_10(IIAssetManager[]) := phi(['assetManagers_2', 'assetManagers_4', 'assetManagers_10', 'assetManagers_6', 'assetManagers_9', 'assetManagers_15', 'assetManagers_0'])
 length = assetManagers.length
REF_4005 -> LENGTH assetManagers_10
length_1(uint256) := REF_4005(uint256)
 _assetManagers = new IAssetManager[](length)
TMP_5713(IAssetManager[])  = new IAssetManager[](length_1)
_assetManagers_1(IAssetManager[]) = ['TMP_5713(IAssetManager[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < length
i_2(uint256) := phi(['i_3', 'i_1'])
TMP_5714(bool) = i_2 < length_1
CONDITION TMP_5714
 _assetManagers[i] = assetManagers[i]
REF_4006(IAssetManager) -> _assetManagers_1[i_2]
REF_4007(IIAssetManager) -> assetManagers_10[i_2]
_assetManagers_2(IAssetManager[]) := phi(['_assetManagers_1'])
REF_4006(IAssetManager) (->_assetManagers_2) := REF_4007(IIAssetManager)
 i ++
TMP_5715(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 _assetManagers
RETURN _assetManagers_1
```
#### AssetManagerController.initialize(IGovernanceSettings,address,address) [EXTERNAL]
```slithir
 GovernedBase.initialise(_governanceSettings,_initialGovernance)
INTERNAL_CALL, GovernedBase.initialise(IGovernanceSettings,address)(_governanceSettings_1,_initialGovernance_1)
 AddressUpdatable.setAddressUpdaterValue(_addressUpdater)
INTERNAL_CALL, AddressUpdatable.setAddressUpdaterValue(address)(_addressUpdater_1)
```
#### AssetManagerController.pauseMinting(IIAssetManager[]) [EXTERNAL]
```slithir
 _callOnManagers(_assetManagers,abi.encodeCall(IIAssetManager.pauseMinting,()))
REF_4090(pauseMinting) -> IIAssetManager.pauseMinting
TMP_5814(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4090,[])
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5814)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.removeAssetManager(IIAssetManager) [EXTERNAL]
```slithir
assetManagerIndex_4(mapping(address => uint256)) := phi(['assetManagerIndex_5', 'assetManagerIndex_7', 'assetManagerIndex_3', 'assetManagerIndex_8', 'assetManagerIndex_9', 'assetManagerIndex_0', 'assetManagerIndex_2'])
assetManagers_5(IIAssetManager[]) := phi(['assetManagers_2', 'assetManagers_4', 'assetManagers_10', 'assetManagers_6', 'assetManagers_9', 'assetManagers_15', 'assetManagers_0'])
 position = assetManagerIndex[address(_assetManager)]
TMP_5697 = CONVERT _assetManager_1 to address
REF_3992(uint256) -> assetManagerIndex_5[TMP_5697]
position_1(uint256) := REF_3992(uint256)
 position == 0
TMP_5698(bool) = position_1 == 0
CONDITION TMP_5698
 index = position - 1
TMP_5699(uint256) = position_1 (c)- 1
index_1(uint256) := TMP_5699(uint256)
 lastIndex = assetManagers.length - 1
REF_3993 -> LENGTH assetManagers_6
TMP_5700(uint256) = REF_3993 (c)- 1
lastIndex_1(uint256) := TMP_5700(uint256)
 index < lastIndex
TMP_5701(bool) = index_1 < lastIndex_1
CONDITION TMP_5701
 assetManagers[index] = assetManagers[lastIndex]
REF_3994(IIAssetManager) -> assetManagers_6[index_1]
REF_3995(IIAssetManager) -> assetManagers_6[lastIndex_1]
assetManagers_7(IIAssetManager[]) := phi(['assetManagers_6'])
REF_3994(IIAssetManager) (->assetManagers_7) := REF_3995(IIAssetManager)
 assetManagerIndex[address(assetManagers[index])] = index + 1
REF_3996(IIAssetManager) -> assetManagers_7[index_1]
TMP_5702 = CONVERT REF_3996 to address
REF_3997(uint256) -> assetManagerIndex_5[TMP_5702]
TMP_5703(uint256) = index_1 (c)+ 1
assetManagerIndex_6(mapping(address => uint256)) := phi(['assetManagerIndex_5'])
REF_3997(uint256) (->assetManagerIndex_6) := TMP_5703(uint256)
 assetManagers.pop()
REF_3999 -> LENGTH assetManagers_7
TMP_5705(uint256) = REF_3999 (c)- 1
REF_4000(IIAssetManager) -> assetManagers_7[TMP_5705]
assetManagers_8 = delete REF_4000 
REF_4001 -> LENGTH assetManagers_8
assetManagers_9(IIAssetManager[]) := phi(['assetManagers_8'])
REF_4001(uint256) (->assetManagers_9) := TMP_5705(uint256)
 assetManagerIndex[address(_assetManager)] = 0
TMP_5706 = CONVERT _assetManager_1 to address
REF_4002(uint256) -> assetManagerIndex_6[TMP_5706]
assetManagerIndex_7(mapping(address => uint256)) := phi(['assetManagerIndex_6'])
REF_4002(uint256) (->assetManagerIndex_7) := 0(uint256)
 _assetManager.assetManagerController() == address(this)
TMP_5707(address) = HIGH_LEVEL_CALL, dest:_assetManager_1(IIAssetManager), function:assetManagerController, arguments:[]  
TMP_5708 = CONVERT this to address
TMP_5709(bool) = TMP_5707 == TMP_5708
CONDITION TMP_5709
 _assetManager.attachController(false)
HIGH_LEVEL_CALL, dest:_assetManager_1(IIAssetManager), function:attachController, arguments:['False']  
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.removeEmergencyPauseSender(address) [EXTERNAL]
```slithir
emergencyPauseSenders_7(EnumerableSet.AddressSet) := phi(['emergencyPauseSenders_2', 'emergencyPauseSenders_0', 'emergencyPauseSenders_8', 'emergencyPauseSenders_4', 'emergencyPauseSenders_6'])
 emergencyPauseSenders.remove(_address)
TMP_5888(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.remove(EnumerableSet.AddressSet,address), arguments:['emergencyPauseSenders_8', '_address_1'] 
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.resetEmergencyPauseTotalDuration(IIAssetManager[]) [EXTERNAL]
```slithir
 _callOnManagers(_assetManagers,abi.encodeCall(IIAssetManager.resetEmergencyPauseTotalDuration,()))
REF_4111(resetEmergencyPauseTotalDuration) -> IIAssetManager.resetEmergencyPauseTotalDuration
TMP_5881(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4111,[])
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5881)
 _callOnManagers(_assetManagers,abi.encodeCall(IIAssetManager.resetEmergencyPauseTransfersTotalDuration,()))
REF_4113(resetEmergencyPauseTransfersTotalDuration) -> IIAssetManager.resetEmergencyPauseTransfersTotalDuration
TMP_5883(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4113,[])
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5883)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setAgentExitAvailableTimelockSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setAgentExitAvailableTimelockSeconds.selector,_value)
REF_4066(bytes4) (->None) := 2070914914(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4066,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setAgentFeeChangeTimelockSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setAgentFeeChangeTimelockSeconds.selector,_value)
REF_4068(bytes4) (->None) := 2910111533(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4068,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setAgentMintingCRChangeTimelockSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setAgentMintingCRChangeTimelockSeconds.selector,_value)
REF_4070(bytes4) (->None) := 738634736(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4070,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setAgentOwnerRegistry(IIAssetManager[],address) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setAgentOwnerRegistry.selector,_value)
REF_4010(bytes4) (->None) := 3450127681(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,address)(_assetManagers_1,REF_4010,_value_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setAgentTimelockedOperationWindowSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setAgentTimelockedOperationWindowSeconds.selector,_value)
REF_4074(bytes4) (->None) := 2167307566(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4074,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setAgentVaultFactory(IIAssetManager[],address) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setAgentVaultFactory.selector,_value)
REF_4012(bytes4) (->None) := 332834274(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,address)(_assetManagers_1,REF_4012,_value_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setAttestationWindowSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setAttestationWindowSeconds.selector,_value)
REF_4054(bytes4) (->None) := 662284970(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4054,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setAverageBlockTimeMS(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setAverageBlockTimeMS.selector,_value)
REF_4056(bytes4) (->None) := 3486226717(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4056,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setCleanerContract(IIAssetManager[],address) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setCleanerContract.selector,_value)
REF_4024(bytes4) (->None) := 4137981103(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,address)(_assetManagers_1,REF_4024,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setCleanupBlockNumberManager(IIAssetManager[],address) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setCleanupBlockNumberManager.selector,_value)
REF_4026(bytes4) (->None) := 2135935657(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,address)(_assetManagers_1,REF_4026,_value_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setCollateralPoolFactory(IIAssetManager[],address) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setCollateralPoolFactory.selector,_value)
REF_4014(bytes4) (->None) := 1842599305(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,address)(_assetManagers_1,REF_4014,_value_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setCollateralPoolTokenFactory(IIAssetManager[],address) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setCollateralPoolTokenFactory.selector,_value)
REF_4016(bytes4) (->None) := 3741772438(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,address)(_assetManagers_1,REF_4016,_value_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setCollateralPoolTokenTimelockSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setCollateralPoolTokenTimelockSeconds.selector,_value)
REF_4076(bytes4) (->None) := 1792473935(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4076,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setCollateralRatiosForToken(IIAssetManager[],CollateralType.Class,IERC20,uint256,uint256) [EXTERNAL]
```slithir
 _callOnManagers(_assetManagers,abi.encodeCall(IIAssetManager.setCollateralRatiosForToken,(_class,_token,_minCollateralRatioBIPS,_safetyMinCollateralRatioBIPS)))
REF_4086(setCollateralRatiosForToken) -> IIAssetManager.setCollateralRatiosForToken
TMP_5808(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4086,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff831528c0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83152a10>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83152a40>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83152aa0>])
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5808)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setCollateralReservationFeeBips(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setCollateralReservationFeeBips.selector,_value)
REF_4040(bytes4) (->None) := 1522909470(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4040,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setConfirmationByOthersAfterSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setConfirmationByOthersAfterSeconds.selector,_value)
REF_4046(bytes4) (->None) := 3754267671(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4046,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setConfirmationByOthersRewardUSD5(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setConfirmationByOthersRewardUSD5.selector,_value)
REF_4048(bytes4) (->None) := 3497276309(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4048,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setEmergencyPauseDurationResetAfterSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setEmergencyPauseDurationResetAfterSeconds.selector,_value)
REF_4119(bytes4) (->None) := 2013307167(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4119,_value_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setFdcVerification(IIAssetManager[],address) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setFdcVerification.selector,_value)
REF_4022(bytes4) (->None) := 3829601407(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,address)(_assetManagers_1,REF_4022,_value_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setLiquidationPaymentFactors(IIAssetManager[],uint256[],uint256[]) [EXTERNAL]
```slithir
 _callOnManagers(_assetManagers,abi.encodeCall(IISettingsManagement.setLiquidationPaymentFactors,(_paymentFactors,_vaultCollateralFactors)))
REF_4080(setLiquidationPaymentFactors) -> IISettingsManagement.setLiquidationPaymentFactors
TMP_5800(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4080,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff831521a0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83150070>])
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5800)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setLiquidationStepSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setLiquidationStepSeconds.selector,_value)
REF_4078(bytes4) (->None) := 2375307530(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4078,_value_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setLotSizeAmg(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setLotSizeAmg.selector,_value)
REF_4032(bytes4) (->None) := 2942249880(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4032,_value_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setMaxEmergencyPauseDurationSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setMaxEmergencyPauseDurationSeconds.selector,_value)
REF_4117(bytes4) (->None) := 1844646102(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4117,_value_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setMaxRedeemedTickets(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setMaxRedeemedTickets.selector,_value)
REF_4050(bytes4) (->None) := 1199655942(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4050,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setMaxTrustedPriceAgeSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setMaxTrustedPriceAgeSeconds.selector,_value)
REF_4038(bytes4) (->None) := 1451943310(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4038,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setMinUpdateRepeatTimeSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setMinUpdateRepeatTimeSeconds.selector,_value)
REF_4030(bytes4) (->None) := 3581748677(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4030,_value_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setMintingCapAmg(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setMintingCapAmg.selector,_value)
REF_4060(bytes4) (->None) := 3001459381(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4060,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setMintingPoolHoldingsRequiredBIPS(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setMintingPoolHoldingsRequiredBIPS.selector,_value)
REF_4058(bytes4) (->None) := 3868147647(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4058,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setPaymentChallengeReward(IIAssetManager[],uint256,uint256) [EXTERNAL]
```slithir
 _callOnManagers(_assetManagers,abi.encodeCall(IISettingsManagement.setPaymentChallengeReward,(_rewardVaultCollateralWei,_rewardBIPS)))
REF_4036(setPaymentChallengeReward) -> IISettingsManagement.setPaymentChallengeReward
TMP_5755(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4036,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff831136d0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff831135b0>])
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5755)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setPoolExitCRChangeTimelockSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setPoolExitCRChangeTimelockSeconds.selector,_value)
REF_4072(bytes4) (->None) := 3967037604(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4072,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setPriceReader(IIAssetManager[],address) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setPriceReader.selector,_value)
REF_4020(bytes4) (->None) := 3188102937(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,address)(_assetManagers_1,REF_4020,_value_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setRedemptionDefaultFactorVaultCollateralBIPS(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setRedemptionDefaultFactorVaultCollateralBIPS.selector,_value)
REF_4044(bytes4) (->None) := 2652451394(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4044,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setRedemptionFeeBips(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setRedemptionFeeBips.selector,_value)
REF_4042(bytes4) (->None) := 2645720917(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4042,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setRedemptionPaymentExtensionSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IRedemptionTimeExtension.setRedemptionPaymentExtensionSeconds.selector,_value)
REF_4082(bytes4) (->None) := 714237073(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4082,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.setTimeForPayment(IIAssetManager[],uint256,uint256) [EXTERNAL]
```slithir
 _callOnManagers(_assetManagers,abi.encodeCall(IISettingsManagement.setTimeForPayment,(_underlyingBlocks,_underlyingSeconds)))
REF_4034(setTimeForPayment) -> IISettingsManagement.setTimeForPayment
TMP_5752(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4034,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83113460>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83112f20>])
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5752)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setTokenInvalidationTimeMinSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setTokenInvalidationTimeMinSeconds.selector,_value)
REF_4062(bytes4) (->None) := 84886585(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4062,_value_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setVaultCollateralBuyForFlareFactorBIPS(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setVaultCollateralBuyForFlareFactorBIPS.selector,_value)
REF_4064(bytes4) (->None) := 1798882883(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4064,_value_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.setWithdrawalOrDestroyWaitMinSeconds(IIAssetManager[],uint256) [EXTERNAL]
```slithir
 _setValueOnManagers(_assetManagers,IISettingsManagement.setWithdrawalOrDestroyWaitMinSeconds.selector,_value)
REF_4052(bytes4) (->None) := 3714366724(bytes4)
INTERNAL_CALL, AssetManagerController._setValueOnManagers(IIAssetManager[],bytes4,uint256)(_assetManagers_1,REF_4052,_value_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```

#### AssetManagerController.supportsInterface(bytes4) [EXTERNAL]
```slithir
 _interfaceId == type()(IERC165).interfaceId || _interfaceId == type()(IAddressUpdatable).interfaceId || _interfaceId == type()(IIAddressUpdatable).interfaceId || _interfaceId == type()(IAssetManagerController).interfaceId || _interfaceId == type()(IIAssetManagerController).interfaceId || _interfaceId == type()(IGoverned).interfaceId
TMP_5820(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_4093(bytes4) (->None) := 33540519(bytes4)
TMP_5821(bool) = _interfaceId_1 == REF_4093
TMP_5822(type(IAddressUpdatable)) = SOLIDITY_CALL type()(IAddressUpdatable)
REF_4094(bytes4) (->None) := 1382523229(bytes4)
TMP_5823(bool) = _interfaceId_1 == REF_4094
TMP_5824(bool) = TMP_5821 || TMP_5823
TMP_5825(type(IIAddressUpdatable)) = SOLIDITY_CALL type()(IIAddressUpdatable)
REF_4095(bytes4) (->None) := 2953579382(bytes4)
TMP_5826(bool) = _interfaceId_1 == REF_4095
TMP_5827(bool) = TMP_5824 || TMP_5826
TMP_5828(type(IAssetManagerController)) = SOLIDITY_CALL type()(IAssetManagerController)
REF_4096(bytes4) (->None) := 3266162081(bytes4)
TMP_5829(bool) = _interfaceId_1 == REF_4096
TMP_5830(bool) = TMP_5827 || TMP_5829
TMP_5831(type(IIAssetManagerController)) = SOLIDITY_CALL type()(IIAssetManagerController)
REF_4097(bytes4) (->None) := 2551734324(bytes4)
TMP_5832(bool) = _interfaceId_1 == REF_4097
TMP_5833(bool) = TMP_5830 || TMP_5832
TMP_5834(type(IGoverned)) = SOLIDITY_CALL type()(IGoverned)
REF_4098(bytes4) (->None) := 3301115419(bytes4)
TMP_5835(bool) = _interfaceId_1 == REF_4098
TMP_5836(bool) = TMP_5833 || TMP_5835
RETURN TMP_5836
```
#### AssetManagerController.unpauseMinting(IIAssetManager[]) [EXTERNAL]
```slithir
 _callOnManagers(_assetManagers,abi.encodeCall(IIAssetManager.unpauseMinting,()))
REF_4092(unpauseMinting) -> IIAssetManager.unpauseMinting
TMP_5817(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4092,[])
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5817)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.updateContracts(IIAssetManager[]) [EXTERNAL]
```slithir
 addressUpdater = IIAddressUpdater(getAddressUpdater())
TMP_5837(address) = INTERNAL_CALL, AddressUpdatable.getAddressUpdater()()
TMP_5838 = CONVERT TMP_5837 to IIAddressUpdater
addressUpdater_1(IIAddressUpdater) := TMP_5838(IIAddressUpdater)
 newAddressUpdater = addressUpdater.getContractAddress(AddressUpdater)
TMP_5839(address) = HIGH_LEVEL_CALL, dest:addressUpdater_1(IIAddressUpdater), function:getContractAddress, arguments:['AddressUpdater']  
newAddressUpdater_1(address) := TMP_5839(address)
 assetManagerController = addressUpdater.getContractAddress(AssetManagerController)
TMP_5840(address) = HIGH_LEVEL_CALL, dest:addressUpdater_1(IIAddressUpdater), function:getContractAddress, arguments:['AssetManagerController']  
assetManagerController_1(address) := TMP_5840(address)
 wNat = addressUpdater.getContractAddress(WNat)
TMP_5841(address) = HIGH_LEVEL_CALL, dest:addressUpdater_1(IIAddressUpdater), function:getContractAddress, arguments:['WNat']  
wNat_1(address) := TMP_5841(address)
 require(bool,error)(newAddressUpdater != address(0) && assetManagerController != address(0) && wNat != address(0),revert AddressZero()())
TMP_5842 = CONVERT 0 to address
TMP_5843(bool) = newAddressUpdater_1 != TMP_5842
TMP_5844 = CONVERT 0 to address
TMP_5845(bool) = assetManagerController_1 != TMP_5844
TMP_5846(bool) = TMP_5843 && TMP_5845
TMP_5847 = CONVERT 0 to address
TMP_5848(bool) = wNat_1 != TMP_5847
TMP_5849(bool) = TMP_5846 && TMP_5848
TMP_5850(None) = SOLIDITY_CALL revert AddressZero()()
TMP_5851(None) = SOLIDITY_CALL require(bool,error)(TMP_5849,TMP_5850)
 _updateContracts(_assetManagers,newAddressUpdater,assetManagerController,wNat)
INTERNAL_CALL, AssetManagerController._updateContracts(IIAssetManager[],address,address,address)(_assetManagers_1,newAddressUpdater_1,assetManagerController_1,wNat_1)
```
#### AssetManagerController.upgradeAgentVaultsAndPools(IIAssetManager[],uint256,uint256) [EXTERNAL]
```slithir
 _callOnManagers(_assetManagers,abi.encodeCall(IIAssetManager.upgradeAgentVaultsAndPools,(_start,_end)))
REF_4018(upgradeAgentVaultsAndPools) -> IIAssetManager.upgradeAgentVaultsAndPools
TMP_5734(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4018,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83112380>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83111c60>])
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5734)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### AssetManagerController.upgradeFAssetImplementation(IIAssetManager[],address,bytes) [EXTERNAL]
```slithir
 _callOnManagers(_assetManagers,abi.encodeCall(IISettingsManagement.upgradeFAssetImplementation,(_implementation,_callData)))
REF_4028(upgradeFAssetImplementation) -> IISettingsManagement.upgradeFAssetImplementation
TMP_5745(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4028,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff83112dd0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff831124d0>])
INTERNAL_CALL, AssetManagerController._callOnManagers(IIAssetManager[],bytes)(_assetManagers_1,TMP_5745)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AssetManagerController.upgradeTo(address) [PUBLIC]
```slithir
 _upgradeToAndCallUUPS(newImplementation,new bytes(0),false)
TMP_5718 = new bytes(0)
INTERNAL_CALL, ERC1967Upgrade._upgradeToAndCallUUPS(address,bytes,bool)(newImplementation_1,TMP_5718,False)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
 onlyProxy()
MODIFIER_CALL, UUPSUpgradeable.onlyProxy()()
```
#### AssetManagerController.upgradeToAndCall(address,bytes) [PUBLIC]
```slithir
 _upgradeToAndCallUUPS(newImplementation,data,true)
INTERNAL_CALL, ERC1967Upgrade._upgradeToAndCallUUPS(address,bytes,bool)(newImplementation_1,data_1,True)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
 onlyProxy()
MODIFIER_CALL, UUPSUpgradeable.onlyProxy()()
```
#### Address.functionCall(address,bytes,string) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0,errorMessage)
TMP_301(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256,string)(target_1,data_1,0,errorMessage_1)
RETURN TMP_301
```
#### AddressUpdatable._getContractAddress(bytes32[],address[],string) [INTERNAL]
```slithir
_nameHashes_1(bytes32[]) := phi(['_contractNameHashes_1'])
_addresses_1(address[]) := phi(['_contractAddresses_1'])
 nameHash = keccak256(bytes)(abi.encode(_nameToFind))
TMP_8165(bytes) = SOLIDITY_CALL abi.encode()(_nameToFind_1)
TMP_8166(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_8165)
nameHash_1(bytes32) := TMP_8166(bytes32)
 a = address(0)
TMP_8167 = CONVERT 0 to address
a_1(address) := TMP_8167(address)
a_3(address) := phi(['a_1', 'a_2'])
 i = 0
i_1(uint256) := 0(uint256)
 i < _nameHashes.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_5176 -> LENGTH _nameHashes_1
TMP_8168(bool) = i_2 < REF_5176
CONDITION TMP_8168
 nameHash == _nameHashes[i]
REF_5177(bytes32) -> _nameHashes_1[i_2]
TMP_8169(bool) = nameHash_1 == REF_5177
CONDITION TMP_8169
 a = _addresses[i]
REF_5178(address) -> _addresses_1[i_2]
a_2(address) := REF_5178(address)
 i ++
TMP_8170(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 require(bool,error)(a != address(0),revert AUAddressZero()())
TMP_8171 = CONVERT 0 to address
TMP_8172(bool) = a_3 != TMP_8171
TMP_8173(None) = SOLIDITY_CALL revert AUAddressZero()()
TMP_8174(None) = SOLIDITY_CALL require(bool,error)(TMP_8172,TMP_8173)
 a
RETURN a_3
```
#### AddressUpdatable.getAddressUpdater() [PUBLIC]
```slithir
ADDRESS_STORAGE_POSITION_1(bytes32) := phi(['ADDRESS_STORAGE_POSITION_0'])
 position = ADDRESS_STORAGE_POSITION
position_1(bytes32) := ADDRESS_STORAGE_POSITION_1(bytes32)
 _addressUpdater = sload(uint256)(position)
TMP_8160(uint256) = SOLIDITY_CALL sload(uint256)(position_1)
_addressUpdater_1(address) := TMP_8160(uint256)
 _addressUpdater
RETURN _addressUpdater_1
```
#### AddressUpdatable.setAddressUpdaterValue(address) [INTERNAL]
```slithir
_addressUpdater_1(address) := phi(['TMP_8161', '_addressUpdater_1'])
ADDRESS_STORAGE_POSITION_2(bytes32) := phi(['ADDRESS_STORAGE_POSITION_0'])
 position = ADDRESS_STORAGE_POSITION
position_1(bytes32) := ADDRESS_STORAGE_POSITION_2(bytes32)
 sstore(uint256,uint256)(position,_addressUpdater)
TMP_8175(None) = SOLIDITY_CALL sstore(uint256,uint256)(position_1,_addressUpdater_1)
```
#### IIAssetManager.attachController(bool) [EXTERNAL]
```slithir

```
#### EnumerableSet.add(EnumerableSet.UintSet,uint256) [INTERNAL]
```slithir
 _add(set._inner,bytes32(value))
REF_232(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_940 = CONVERT value_1 to bytes32
TMP_941(bool) = INTERNAL_CALL, EnumerableSet._add(EnumerableSet.Set,bytes32)(REF_232,TMP_940)
RETURN TMP_941
```
#### GovernedBase.governance() [PUBLIC]
```slithir
 state = _governedState()
TMP_9529(GovernedBase.GovernedState) = INTERNAL_CALL, GovernedBase._governedState()()
state_1 (-> ['TMP_9529'])(GovernedBase.GovernedState) := TMP_9529(GovernedBase.GovernedState)
 state.productionMode
REF_5853(bool) -> state_1 (-> ['TMP_9529']).productionMode
CONDITION REF_5853
 state.governanceSettings.getGovernanceAddress()
REF_5854(IGovernanceSettings) -> state_1 (-> ['TMP_9529']).governanceSettings
TMP_9530(address) = HIGH_LEVEL_CALL, dest:REF_5854(IGovernanceSettings), function:getGovernanceAddress, arguments:[]  
RETURN TMP_9530
 state.initialGovernance
REF_5856(address) -> state_1 (-> ['TMP_9529']).initialGovernance
RETURN REF_5856
```
#### EnumerableSet.contains(EnumerableSet.UintSet,uint256) [INTERNAL]
```slithir
 _contains(set._inner,bytes32(value))
REF_234(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_944 = CONVERT value_1 to bytes32
TMP_945(bool) = INTERNAL_CALL, EnumerableSet._contains(EnumerableSet.Set,bytes32)(REF_234,TMP_944)
RETURN TMP_945
```
#### GovernedBase.initialise(IGovernanceSettings,address) [INTERNAL]
```slithir
 state = _governedState()
TMP_9513(GovernedBase.GovernedState) = INTERNAL_CALL, GovernedBase._governedState()()
state_1 (-> ['TMP_9513'])(GovernedBase.GovernedState) := TMP_9513(GovernedBase.GovernedState)
 require(bool,error)(state.initialised == false,revert GovernedAlreadyInitialized()())
REF_5847(bool) -> state_1 (-> ['TMP_9513']).initialised
TMP_9514(bool) = REF_5847 == False
TMP_9515(None) = SOLIDITY_CALL revert GovernedAlreadyInitialized()()
TMP_9516(None) = SOLIDITY_CALL require(bool,error)(TMP_9514,TMP_9515)
 require(bool,error)(address(_governanceSettings) != address(0),revert GovernedAddressZero()())
TMP_9517 = CONVERT _governanceSettings_1 to address
TMP_9518 = CONVERT 0 to address
TMP_9519(bool) = TMP_9517 != TMP_9518
TMP_9520(None) = SOLIDITY_CALL revert GovernedAddressZero()()
TMP_9521(None) = SOLIDITY_CALL require(bool,error)(TMP_9519,TMP_9520)
 require(bool,error)(_initialGovernance != address(0),revert GovernedAddressZero()())
TMP_9522 = CONVERT 0 to address
TMP_9523(bool) = _initialGovernance_1 != TMP_9522
TMP_9524(None) = SOLIDITY_CALL revert GovernedAddressZero()()
TMP_9525(None) = SOLIDITY_CALL require(bool,error)(TMP_9523,TMP_9524)
 state.initialised = true
REF_5848(bool) -> state_1 (-> ['TMP_9513']).initialised
state_2 (-> ['TMP_9513'])(GovernedBase.GovernedState) := phi(["state_1 (-> ['TMP_9513'])"])
REF_5848(bool) (->state_2 (-> ['TMP_9513'])) := True(bool)
TMP_9513(GovernedBase.GovernedState) := phi(["state_2 (-> ['TMP_9513'])"])
 state.governanceSettings = _governanceSettings
REF_5849(IGovernanceSettings) -> state_2 (-> ['TMP_9513']).governanceSettings
state_3 (-> ['TMP_9513'])(GovernedBase.GovernedState) := phi(["state_2 (-> ['TMP_9513'])"])
REF_5849(IGovernanceSettings) (->state_3 (-> ['TMP_9513'])) := _governanceSettings_1(IGovernanceSettings)
TMP_9513(GovernedBase.GovernedState) := phi(["state_3 (-> ['TMP_9513'])"])
 state.initialGovernance = _initialGovernance
REF_5850(address) -> state_3 (-> ['TMP_9513']).initialGovernance
state_4 (-> ['TMP_9513'])(GovernedBase.GovernedState) := phi(["state_3 (-> ['TMP_9513'])"])
REF_5850(address) (->state_4 (-> ['TMP_9513'])) := _initialGovernance_1(address)
TMP_9513(GovernedBase.GovernedState) := phi(["state_4 (-> ['TMP_9513'])"])
 GovernanceInitialised(_initialGovernance)
Emit GovernanceInitialised(_initialGovernance_1)
```
#### EnumerableSet.remove(EnumerableSet.UintSet,uint256) [INTERNAL]
```slithir
 _remove(set._inner,bytes32(value))
REF_233(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_942 = CONVERT value_1 to bytes32
TMP_943(bool) = INTERNAL_CALL, EnumerableSet._remove(EnumerableSet.Set,bytes32)(REF_233,TMP_942)
RETURN TMP_943
```
#### AddressUpdaterMock.getContractAddress(string) [EXTERNAL]
```slithir
contractAddresses_8(mapping(bytes32 => address)) := phi(['contractAddresses_12', 'contractAddresses_2', 'contractAddresses_10', 'contractAddresses_9', 'contractAddresses_0', 'contractAddresses_7', 'contractAddresses_13', 'contractAddresses_16', 'contractAddresses_18'])
 contractAddresses[_keccak256AbiEncode(_name)]
TMP_8309(bytes32) = INTERNAL_CALL, AddressUpdaterMock._keccak256AbiEncode(string)(_name_1)
REF_5236(address) -> contractAddresses_9[TMP_8309]
RETURN REF_5236
```
#### ERC1967Upgrade._upgradeToAndCallUUPS(address,bytes,bool) [INTERNAL]
```slithir
_ROLLBACK_SLOT_1(bytes32) := phi(['_ROLLBACK_SLOT_0'])
_IMPLEMENTATION_SLOT_3(bytes32) := phi(['_IMPLEMENTATION_SLOT_4', '_IMPLEMENTATION_SLOT_0'])
 StorageSlot.getBooleanSlot(_ROLLBACK_SLOT).value
TMP_70(StorageSlot.BooleanSlot) = LIBRARY_CALL, dest:StorageSlot, function:StorageSlot.getBooleanSlot(bytes32), arguments:['_ROLLBACK_SLOT_1'] 
REF_33(bool) -> TMP_70.value
CONDITION REF_33
 _setImplementation(newImplementation)
INTERNAL_CALL, ERC1967Upgrade._setImplementation(address)(newImplementation_1)
 slot = IERC1822Proxiable(newImplementation).proxiableUUID()
TMP_72 = CONVERT newImplementation_1 to IERC1822Proxiable
TMP_73(bytes32) = HIGH_LEVEL_CALL, dest:TMP_72(IERC1822Proxiable), function:proxiableUUID, arguments:[]  
_IMPLEMENTATION_SLOT_4(bytes32) := phi(['_IMPLEMENTATION_SLOT_4', '_IMPLEMENTATION_SLOT_3'])
slot_1(bytes32) := TMP_73(bytes32)
 require(bool,string)(slot == _IMPLEMENTATION_SLOT,ERC1967Upgrade: unsupported proxiableUUID)
TMP_74(bool) = slot_1 == _IMPLEMENTATION_SLOT_4
TMP_75(None) = SOLIDITY_CALL require(bool,string)(TMP_74,ERC1967Upgrade: unsupported proxiableUUID)
 revert(string)(ERC1967Upgrade: new implementation is not UUPS)
TMP_76(None) = SOLIDITY_CALL revert(string)(ERC1967Upgrade: new implementation is not UUPS)
 _upgradeToAndCall(newImplementation,data,forceCall)
INTERNAL_CALL, ERC1967Upgrade._upgradeToAndCall(address,bytes,bool)(newImplementation_1,data_1,forceCall_1)
```
#### Address.functionCallWithValue(address,bytes,uint256,string) [INTERNAL]
```slithir
target_1(address) := phi(['target_1', 'target_1', 'target_1'])
data_1(bytes) := phi(['data_1', 'data_1', 'data_1'])
value_1(uint256) := phi(['value_1'])
errorMessage_1(string) := phi(['errorMessage_1'])
 require(bool,string)(address(this).balance >= value,Address: insufficient balance for call)
TMP_303 = CONVERT this to address
TMP_304(uint256) = SOLIDITY_CALL balance(address)(TMP_303)
TMP_305(bool) = TMP_304 >= value_1
TMP_306(None) = SOLIDITY_CALL require(bool,string)(TMP_305,Address: insufficient balance for call)
 (success,returndata) = target.call{value: value}(data)
TUPLE_2(bool,bytes) = LOW_LEVEL_CALL, dest:target_1, function:call, arguments:['data_1'] value:value_1 
success_1(bool)= UNPACK TUPLE_2 index: 0 
returndata_1(bytes)= UNPACK TUPLE_2 index: 1 
 verifyCallResultFromTarget(target,success,returndata,errorMessage)
TMP_307(bytes) = INTERNAL_CALL, Address.verifyCallResultFromTarget(address,bool,bytes,string)(target_1,success_1,returndata_1,errorMessage_1)
RETURN TMP_307
```
#### EnumerableSet._add(EnumerableSet.Set,bytes32) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_226', 'REF_220', 'REF_232'])
value_1(bytes32) := phi(['TMP_940', 'value_1', 'TMP_924'])
 ! _contains(set,value)
TMP_904(bool) = INTERNAL_CALL, EnumerableSet._contains(EnumerableSet.Set,bytes32)(set_1 (-> []),value_1)
TMP_905 = UnaryType.BANG TMP_904 
CONDITION TMP_905
 set._values.push(value)
REF_188(bytes32[]) -> set_1 (-> [])._values
REF_190 -> LENGTH REF_188
TMP_907(uint256) := REF_190(uint256)
TMP_908(uint256) = TMP_907 (c)+ 1
set_2 (-> [])(EnumerableSet.Set) := phi(['set_1 (-> [])'])
REF_190(uint256) (->set_3 (-> [])) := TMP_908(uint256)
REF_191(bytes32) -> REF_188[TMP_907]
set_3 (-> [])(EnumerableSet.Set) := phi(['set_2 (-> [])'])
REF_191(bytes32) (->set_3 (-> [])) := value_1(bytes32)
 set._indexes[value] = set._values.length
REF_192(mapping(bytes32 => uint256)) -> set_3 (-> [])._indexes
REF_193(uint256) -> REF_192[value_1]
REF_194(bytes32[]) -> set_3 (-> [])._values
REF_195 -> LENGTH REF_194
set_4 (-> [])(EnumerableSet.Set) := phi(['set_3 (-> [])'])
REF_193(uint256) (->set_4 (-> [])) := REF_195(uint256)
 true
RETURN True
 false
RETURN False
```
#### GovernedBase._governedState() [PRIVATE]
```slithir
 position = keccak256(bytes)(fasset.GovernedBase.GovernedState)
TMP_9553(bytes32) = SOLIDITY_CALL keccak256(bytes)(fasset.GovernedBase.GovernedState)
position_1(bytes32) := TMP_9553(bytes32)
 _state = position
_state_1 (-> ['position'])(GovernedBase.GovernedState) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
```
#### GovernanceSettingsMock.getGovernanceAddress() [EXTERNAL]
```slithir
governanceAddress_5(address) := phi(['governanceAddress_1', 'governanceAddress_0', 'governanceAddress_3'])
 governanceAddress
RETURN governanceAddress_5
```
#### EnumerableSet._contains(EnumerableSet.Set,bytes32) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_234', 'set_1 (-> [])', 'REF_228', 'REF_222'])
value_1(bytes32) := phi(['value_1', 'value_1', 'TMP_944', 'TMP_932'])
 set._indexes[value] != 0
REF_213(mapping(bytes32 => uint256)) -> set_1 (-> [])._indexes
REF_214(uint256) -> REF_213[value_1]
TMP_915(bool) = REF_214 != 0
RETURN TMP_915
```
#### EnumerableSet._remove(EnumerableSet.Set,bytes32) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_221', 'REF_233', 'REF_227'])
value_1(bytes32) := phi(['TMP_942', 'value_1', 'TMP_928'])
 valueIndex = set._indexes[value]
REF_196(mapping(bytes32 => uint256)) -> set_1 (-> [])._indexes
REF_197(uint256) -> REF_196[value_1]
valueIndex_1(uint256) := REF_197(uint256)
 valueIndex != 0
TMP_909(bool) = valueIndex_1 != 0
CONDITION TMP_909
 toDeleteIndex = valueIndex - 1
TMP_910(uint256) = valueIndex_1 (c)- 1
toDeleteIndex_1(uint256) := TMP_910(uint256)
 lastIndex = set._values.length - 1
REF_198(bytes32[]) -> set_1 (-> [])._values
REF_199 -> LENGTH REF_198
TMP_911(uint256) = REF_199 (c)- 1
lastIndex_1(uint256) := TMP_911(uint256)
 lastIndex != toDeleteIndex
TMP_912(bool) = lastIndex_1 != toDeleteIndex_1
CONDITION TMP_912
 lastValue = set._values[lastIndex]
REF_200(bytes32[]) -> set_1 (-> [])._values
REF_201(bytes32) -> REF_200[lastIndex_1]
lastValue_1(bytes32) := REF_201(bytes32)
 set._values[toDeleteIndex] = lastValue
REF_202(bytes32[]) -> set_1 (-> [])._values
REF_203(bytes32) -> REF_202[toDeleteIndex_1]
set_2 (-> [])(EnumerableSet.Set) := phi(['set_1 (-> [])'])
REF_203(bytes32) (->set_2 (-> [])) := lastValue_1(bytes32)
 set._indexes[lastValue] = valueIndex
REF_204(mapping(bytes32 => uint256)) -> set_2 (-> [])._indexes
REF_205(uint256) -> REF_204[lastValue_1]
set_3 (-> [])(EnumerableSet.Set) := phi(['set_2 (-> [])'])
REF_205(uint256) (->set_3 (-> [])) := valueIndex_1(uint256)
set_4 (-> [])(EnumerableSet.Set) := phi(['set_1 (-> [])', 'set_3 (-> [])'])
 set._values.pop()
REF_206(bytes32[]) -> set_4 (-> [])._values
REF_208 -> LENGTH REF_206
TMP_914(uint256) = REF_208 (c)- 1
REF_209(bytes32) -> REF_206[TMP_914]
REF_206 = delete REF_209 
REF_210 -> LENGTH REF_206
set_5 (-> [])(EnumerableSet.Set) := phi(['set_4 (-> [])'])
REF_210(uint256) (->set_5 (-> [])) := TMP_914(uint256)
 delete set._indexes[value]
REF_211(mapping(bytes32 => uint256)) -> set_5 (-> [])._indexes
REF_212(uint256) -> REF_211[value_1]
REF_211 = delete REF_212 
 true
RETURN True
 false
RETURN False
```
#### AddressUpdaterMock._keccak256AbiEncode(string) [INTERNAL]
```slithir
_value_1(string) := phi(['_name_1', 'REF_5224', 'contractName_1', 'REF_5251', 'REF_5241', 'REF_5234', 'REF_5261'])
 keccak256(bytes)(abi.encode(_value))
TMP_8342(bytes) = SOLIDITY_CALL abi.encode()(_value_1)
TMP_8343(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_8342)
RETURN TMP_8343
```

#### ERC1967Upgrade._setImplementation(address) [PRIVATE]
```slithir
newImplementation_1(address) := phi(['newImplementation_1', 'newImplementation_1'])
_IMPLEMENTATION_SLOT_2(bytes32) := phi(['_IMPLEMENTATION_SLOT_4', '_IMPLEMENTATION_SLOT_0'])
 require(bool,string)(Address.isContract(newImplementation),ERC1967: new implementation is not a contract)
TMP_61(bool) = LIBRARY_CALL, dest:Address, function:Address.isContract(address), arguments:['newImplementation_1'] 
TMP_62(None) = SOLIDITY_CALL require(bool,string)(TMP_61,ERC1967: new implementation is not a contract)
 StorageSlot.getAddressSlot(_IMPLEMENTATION_SLOT).value = newImplementation
TMP_63(StorageSlot.AddressSlot) = LIBRARY_CALL, dest:StorageSlot, function:StorageSlot.getAddressSlot(bytes32), arguments:['_IMPLEMENTATION_SLOT_2'] 
REF_29(address) -> TMP_63.value
REF_29(address) (->TMP_63) := newImplementation_1(address)
```
#### ERC1967Upgrade._upgradeToAndCall(address,bytes,bool) [INTERNAL]
```slithir
newImplementation_1(address) := phi(['newImplementation_1'])
data_1(bytes) := phi(['data_1'])
forceCall_1(bool) := phi(['forceCall_1'])
 _upgradeTo(newImplementation)
INTERNAL_CALL, ERC1967Upgrade._upgradeTo(address)(newImplementation_1)
 data.length > 0 || forceCall
REF_30 -> LENGTH data_1
TMP_67(bool) = REF_30 > 0
TMP_68(bool) = TMP_67 || forceCall_1
CONDITION TMP_68
 Address.functionDelegateCall(newImplementation,data)
TMP_69(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionDelegateCall(address,bytes), arguments:['newImplementation_1', 'data_1']
```
#### StorageSlot.getBooleanSlot(bytes32) [INTERNAL]
```slithir
 r = slot
r_1 (-> ['slot'])(StorageSlot.BooleanSlot) := slot_1(bytes32)
 r
RETURN r_1 (-> ['slot'])
```
#### Address.verifyCallResultFromTarget(address,bool,bytes,string) [INTERNAL]
```slithir
target_1(address) := phi(['target_1', 'target_1', 'target_1'])
success_1(bool) := phi(['success_1', 'success_1', 'success_1'])
returndata_1(bytes) := phi(['returndata_1', 'returndata_1', 'returndata_1'])
errorMessage_1(string) := phi(['errorMessage_1', 'errorMessage_1', 'errorMessage_1'])
 success
CONDITION success_1
 returndata.length == 0
REF_123 -> LENGTH returndata_1
TMP_312(bool) = REF_123 == 0
CONDITION TMP_312
 require(bool,string)(isContract(target),Address: call to non-contract)
TMP_313(bool) = INTERNAL_CALL, Address.isContract(address)(target_1)
TMP_314(None) = SOLIDITY_CALL require(bool,string)(TMP_313,Address: call to non-contract)
 returndata
RETURN returndata_1
 _revert(returndata,errorMessage)
INTERNAL_CALL, Address._revert(bytes,string)(returndata_1,errorMessage_1)
```
#### Address.isContract(address) [INTERNAL]
```slithir
account_1(address) := phi(['target_1'])
 account.code.length > 0
TMP_293(bytes) = SOLIDITY_CALL code(address)(account_1)
REF_118 -> LENGTH TMP_293
TMP_294(bool) = REF_118 > 0
RETURN TMP_294
```
#### StorageSlot.getAddressSlot(bytes32) [INTERNAL]
```slithir
 r = slot
r_1 (-> ['slot'])(StorageSlot.AddressSlot) := slot_1(bytes32)
 r
RETURN r_1 (-> ['slot'])
```
#### ERC1967Upgrade._upgradeTo(address) [INTERNAL]
```slithir
newImplementation_1(address) := phi(['newImplementation_1'])
 _setImplementation(newImplementation)
INTERNAL_CALL, ERC1967Upgrade._setImplementation(address)(newImplementation_1)
 Upgraded(newImplementation)
Emit Upgraded(newImplementation_1)
```
#### Address.functionDelegateCall(address,bytes,string) [INTERNAL]
```slithir
target_1(address) := phi(['target_1'])
data_1(bytes) := phi(['data_1'])
 (success,returndata) = target.delegatecall(data)
TUPLE_4(bool,bytes) = LOW_LEVEL_CALL, dest:target_1, function:delegatecall, arguments:['data_1']  
success_1(bool)= UNPACK TUPLE_4 index: 0 
returndata_1(bytes)= UNPACK TUPLE_4 index: 1 
 verifyCallResultFromTarget(target,success,returndata,errorMessage)
TMP_311(bytes) = INTERNAL_CALL, Address.verifyCallResultFromTarget(address,bool,bytes,string)(target_1,success_1,returndata_1,errorMessage_1)
RETURN TMP_311
```
