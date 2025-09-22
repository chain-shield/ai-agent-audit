



#### CoreVaultClientSettingsFacet.constructor() [PUBLIC]
```slithir
 CoreVaultClient.getState().initialized = true
TMP_2713(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
REF_1529(bool) -> TMP_2713.initialized
REF_1529(bool) (->TMP_2713) := True(bool)
```
#### CoreVaultClientSettingsFacet.getCoreVaultManager() [EXTERNAL]
```slithir
 state = CoreVaultClient.getState()
TMP_2774(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2774'])(CoreVaultClient.State) := TMP_2774(CoreVaultClient.State)
 address(state.coreVaultManager)
REF_1584(IICoreVaultManager) -> state_1 (-> ['TMP_2774']).coreVaultManager
TMP_2775 = CONVERT REF_1584 to address
RETURN TMP_2775
```
#### CoreVaultClientSettingsFacet.getCoreVaultMinimumAmountLeftBIPS() [EXTERNAL]
```slithir
 state = CoreVaultClient.getState()
TMP_2779(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2779'])(CoreVaultClient.State) := TMP_2779(CoreVaultClient.State)
 state.minimumAmountLeftBIPS
REF_1592(uint16) -> state_1 (-> ['TMP_2779']).minimumAmountLeftBIPS
RETURN REF_1592
```
#### ICoreVaultClientSettings.getCoreVaultMinimumRedeemLots() [EXTERNAL]
```slithir

```
#### CoreVaultClientSettingsFacet.getCoreVaultNativeAddress() [EXTERNAL]
```slithir
 state = CoreVaultClient.getState()
TMP_2776(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2776'])(CoreVaultClient.State) := TMP_2776(CoreVaultClient.State)
 state.nativeAddress
REF_1586(address) -> state_1 (-> ['TMP_2776']).nativeAddress
RETURN REF_1586
```
#### CoreVaultClientSettingsFacet.getCoreVaultRedemptionFeeBIPS() [EXTERNAL]
```slithir
 state = CoreVaultClient.getState()
TMP_2778(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2778'])(CoreVaultClient.State) := TMP_2778(CoreVaultClient.State)
 state.redemptionFeeBIPS
REF_1590(uint16) -> state_1 (-> ['TMP_2778']).redemptionFeeBIPS
RETURN REF_1590
```
#### CoreVaultClientSettingsFacet.getCoreVaultTransferTimeExtensionSeconds() [EXTERNAL]
```slithir
 state = CoreVaultClient.getState()
TMP_2777(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2777'])(CoreVaultClient.State) := TMP_2777(CoreVaultClient.State)
 state.transferTimeExtensionSeconds
REF_1588(uint64) -> state_1 (-> ['TMP_2777']).transferTimeExtensionSeconds
RETURN REF_1588
```
#### CoreVaultClientSettingsFacet.initCoreVaultFacet(IICoreVaultManager,address,uint256,uint256,uint256,uint256) [EXTERNAL]
```slithir
 updateInterfacesAtCoreVaultDeploy()
INTERNAL_CALL, CoreVaultClientSettingsFacet.updateInterfacesAtCoreVaultDeploy()()
 require(bool,error)(_redemptionFeeBIPS <= SafePct.MAX_BIPS,revert BipsValueTooHigh()())
REF_1530(uint256) -> SafePct.MAX_BIPS
TMP_2715(bool) = _redemptionFeeBIPS_1 <= REF_1530
TMP_2716(None) = SOLIDITY_CALL revert BipsValueTooHigh()()
TMP_2717(None) = SOLIDITY_CALL require(bool,error)(TMP_2715,TMP_2716)
 require(bool,error)(_minimumAmountLeftBIPS <= SafePct.MAX_BIPS,revert BipsValueTooHigh()())
REF_1531(uint256) -> SafePct.MAX_BIPS
TMP_2718(bool) = _minimumAmountLeftBIPS_1 <= REF_1531
TMP_2719(None) = SOLIDITY_CALL revert BipsValueTooHigh()()
TMP_2720(None) = SOLIDITY_CALL require(bool,error)(TMP_2718,TMP_2719)
 state = CoreVaultClient.getState()
TMP_2721(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2721'])(CoreVaultClient.State) := TMP_2721(CoreVaultClient.State)
 require(bool,error)(! state.initialized,revert AlreadyInitialized()())
REF_1533(bool) -> state_1 (-> ['TMP_2721']).initialized
TMP_2722 = UnaryType.BANG REF_1533 
TMP_2723(None) = SOLIDITY_CALL revert AlreadyInitialized()()
TMP_2724(None) = SOLIDITY_CALL require(bool,error)(TMP_2722,TMP_2723)
 state.initialized = true
REF_1534(bool) -> state_1 (-> ['TMP_2721']).initialized
state_2 (-> ['TMP_2721'])(CoreVaultClient.State) := phi(["state_1 (-> ['TMP_2721'])"])
REF_1534(bool) (->state_2 (-> ['TMP_2721'])) := True(bool)
TMP_2721(CoreVaultClient.State) := phi(["state_2 (-> ['TMP_2721'])"])
 state.coreVaultManager = _coreVaultManager
REF_1535(IICoreVaultManager) -> state_2 (-> ['TMP_2721']).coreVaultManager
state_3 (-> ['TMP_2721'])(CoreVaultClient.State) := phi(["state_2 (-> ['TMP_2721'])"])
REF_1535(IICoreVaultManager) (->state_3 (-> ['TMP_2721'])) := _coreVaultManager_1(IICoreVaultManager)
TMP_2721(CoreVaultClient.State) := phi(["state_3 (-> ['TMP_2721'])"])
 state.nativeAddress = _nativeAddress
REF_1536(address) -> state_3 (-> ['TMP_2721']).nativeAddress
state_4 (-> ['TMP_2721'])(CoreVaultClient.State) := phi(["state_3 (-> ['TMP_2721'])"])
REF_1536(address) (->state_4 (-> ['TMP_2721'])) := _nativeAddress_1(address)
TMP_2721(CoreVaultClient.State) := phi(["state_4 (-> ['TMP_2721'])"])
 state.transferTimeExtensionSeconds = _transferTimeExtensionSeconds.toUint64()
REF_1537(uint64) -> state_4 (-> ['TMP_2721']).transferTimeExtensionSeconds
TMP_2725(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_transferTimeExtensionSeconds_1'] 
state_5 (-> ['TMP_2721'])(CoreVaultClient.State) := phi(["state_4 (-> ['TMP_2721'])"])
REF_1537(uint64) (->state_5 (-> ['TMP_2721'])) := TMP_2725(uint64)
TMP_2721(CoreVaultClient.State) := phi(["state_5 (-> ['TMP_2721'])"])
 state.redemptionFeeBIPS = _redemptionFeeBIPS.toUint16()
REF_1539(uint16) -> state_5 (-> ['TMP_2721']).redemptionFeeBIPS
TMP_2726(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_redemptionFeeBIPS_1'] 
state_6 (-> ['TMP_2721'])(CoreVaultClient.State) := phi(["state_5 (-> ['TMP_2721'])"])
REF_1539(uint16) (->state_6 (-> ['TMP_2721'])) := TMP_2726(uint16)
TMP_2721(CoreVaultClient.State) := phi(["state_6 (-> ['TMP_2721'])"])
 state.minimumAmountLeftBIPS = _minimumAmountLeftBIPS.toUint16()
REF_1541(uint16) -> state_6 (-> ['TMP_2721']).minimumAmountLeftBIPS
TMP_2727(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_minimumAmountLeftBIPS_1'] 
state_7 (-> ['TMP_2721'])(CoreVaultClient.State) := phi(["state_6 (-> ['TMP_2721'])"])
REF_1541(uint16) (->state_7 (-> ['TMP_2721'])) := TMP_2727(uint16)
TMP_2721(CoreVaultClient.State) := phi(["state_7 (-> ['TMP_2721'])"])
 state.minimumRedeemLots = _minimumRedeemLots.toUint64()
REF_1543(uint64) -> state_7 (-> ['TMP_2721']).minimumRedeemLots
TMP_2728(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_minimumRedeemLots_1'] 
state_8 (-> ['TMP_2721'])(CoreVaultClient.State) := phi(["state_7 (-> ['TMP_2721'])"])
REF_1543(uint64) (->state_8 (-> ['TMP_2721'])) := TMP_2728(uint64)
TMP_2721(CoreVaultClient.State) := phi(["state_8 (-> ['TMP_2721'])"])
```
#### CoreVaultClientSettingsFacet.setCoreVaultManager(address) [EXTERNAL]
```slithir
 require(bool,error)(_coreVaultManager != address(0),revert CannotDisable()())
TMP_2736 = CONVERT 0 to address
TMP_2737(bool) = _coreVaultManager_1 != TMP_2736
TMP_2738(None) = SOLIDITY_CALL revert CannotDisable()()
TMP_2739(None) = SOLIDITY_CALL require(bool,error)(TMP_2737,TMP_2738)
 coreVaultManager = IICoreVaultManager(_coreVaultManager)
TMP_2740 = CONVERT _coreVaultManager_1 to IICoreVaultManager
coreVaultManager_1(IICoreVaultManager) := TMP_2740(IICoreVaultManager)
 require(bool,error)(coreVaultManager.assetManager() == address(this),revert WrongAssetManager()())
TMP_2741(address) = HIGH_LEVEL_CALL, dest:coreVaultManager_1(IICoreVaultManager), function:assetManager, arguments:[]  
TMP_2742 = CONVERT this to address
TMP_2743(bool) = TMP_2741 == TMP_2742
TMP_2744(None) = SOLIDITY_CALL revert WrongAssetManager()()
TMP_2745(None) = SOLIDITY_CALL require(bool,error)(TMP_2743,TMP_2744)
 state = CoreVaultClient.getState()
TMP_2746(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2746'])(CoreVaultClient.State) := TMP_2746(CoreVaultClient.State)
 state.coreVaultManager = coreVaultManager
REF_1560(IICoreVaultManager) -> state_1 (-> ['TMP_2746']).coreVaultManager
state_2 (-> ['TMP_2746'])(CoreVaultClient.State) := phi(["state_1 (-> ['TMP_2746'])"])
REF_1560(IICoreVaultManager) (->state_2 (-> ['TMP_2746'])) := coreVaultManager_1(IICoreVaultManager)
TMP_2746(CoreVaultClient.State) := phi(["state_2 (-> ['TMP_2746'])"])
 IAssetManagerEvents.ContractChanged(coreVaultManager,_coreVaultManager)
Emit ContractChanged(coreVaultManager,_coreVaultManager_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### CoreVaultClientSettingsFacet.setCoreVaultMinimumAmountLeftBIPS(uint256) [EXTERNAL]
```slithir
 require(bool,error)(_minimumAmountLeftBIPS <= SafePct.MAX_BIPS,revert BipsValueTooHigh()())
REF_1574(uint256) -> SafePct.MAX_BIPS
TMP_2763(bool) = _minimumAmountLeftBIPS_1 <= REF_1574
TMP_2764(None) = SOLIDITY_CALL revert BipsValueTooHigh()()
TMP_2765(None) = SOLIDITY_CALL require(bool,error)(TMP_2763,TMP_2764)
 state = CoreVaultClient.getState()
TMP_2766(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2766'])(CoreVaultClient.State) := TMP_2766(CoreVaultClient.State)
 state.minimumAmountLeftBIPS = _minimumAmountLeftBIPS.toUint16()
REF_1576(uint16) -> state_1 (-> ['TMP_2766']).minimumAmountLeftBIPS
TMP_2767(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_minimumAmountLeftBIPS_1'] 
state_2 (-> ['TMP_2766'])(CoreVaultClient.State) := phi(["state_1 (-> ['TMP_2766'])"])
REF_1576(uint16) (->state_2 (-> ['TMP_2766'])) := TMP_2767(uint16)
TMP_2766(CoreVaultClient.State) := phi(["state_2 (-> ['TMP_2766'])"])
 IAssetManagerEvents.SettingChanged(coreVaultMinimumAmountLeftBIPS,_minimumAmountLeftBIPS)
Emit SettingChanged(coreVaultMinimumAmountLeftBIPS,_minimumAmountLeftBIPS_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### CoreVaultClientSettingsFacet.setCoreVaultMinimumRedeemLots(uint256) [EXTERNAL]
```slithir
 state = CoreVaultClient.getState()
TMP_2770(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2770'])(CoreVaultClient.State) := TMP_2770(CoreVaultClient.State)
 state.minimumRedeemLots = _minimumRedeemLots.toUint64()
REF_1580(uint64) -> state_1 (-> ['TMP_2770']).minimumRedeemLots
TMP_2771(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_minimumRedeemLots_1'] 
state_2 (-> ['TMP_2770'])(CoreVaultClient.State) := phi(["state_1 (-> ['TMP_2770'])"])
REF_1580(uint64) (->state_2 (-> ['TMP_2770'])) := TMP_2771(uint64)
TMP_2770(CoreVaultClient.State) := phi(["state_2 (-> ['TMP_2770'])"])
 IAssetManagerEvents.SettingChanged(coreVaultMinimumRedeemLots,_minimumRedeemLots)
Emit SettingChanged(coreVaultMinimumRedeemLots,_minimumRedeemLots_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### CoreVaultClientSettingsFacet.setCoreVaultNativeAddress(address) [EXTERNAL]
```slithir
 state = CoreVaultClient.getState()
TMP_2749(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2749'])(CoreVaultClient.State) := TMP_2749(CoreVaultClient.State)
 state.nativeAddress = _nativeAddress
REF_1563(address) -> state_1 (-> ['TMP_2749']).nativeAddress
state_2 (-> ['TMP_2749'])(CoreVaultClient.State) := phi(["state_1 (-> ['TMP_2749'])"])
REF_1563(address) (->state_2 (-> ['TMP_2749'])) := _nativeAddress_1(address)
TMP_2749(CoreVaultClient.State) := phi(["state_2 (-> ['TMP_2749'])"])
 IAssetManagerEvents.ContractChanged(coreVaultNativeAddress,_nativeAddress)
Emit ContractChanged(coreVaultNativeAddress,_nativeAddress_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### CoreVaultClientSettingsFacet.setCoreVaultRedemptionFeeBIPS(uint256) [EXTERNAL]
```slithir
 require(bool,error)(_redemptionFeeBIPS <= SafePct.MAX_BIPS,revert BipsValueTooHigh()())
REF_1569(uint256) -> SafePct.MAX_BIPS
TMP_2756(bool) = _redemptionFeeBIPS_1 <= REF_1569
TMP_2757(None) = SOLIDITY_CALL revert BipsValueTooHigh()()
TMP_2758(None) = SOLIDITY_CALL require(bool,error)(TMP_2756,TMP_2757)
 state = CoreVaultClient.getState()
TMP_2759(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2759'])(CoreVaultClient.State) := TMP_2759(CoreVaultClient.State)
 state.redemptionFeeBIPS = _redemptionFeeBIPS.toUint16()
REF_1571(uint16) -> state_1 (-> ['TMP_2759']).redemptionFeeBIPS
TMP_2760(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_redemptionFeeBIPS_1'] 
state_2 (-> ['TMP_2759'])(CoreVaultClient.State) := phi(["state_1 (-> ['TMP_2759'])"])
REF_1571(uint16) (->state_2 (-> ['TMP_2759'])) := TMP_2760(uint16)
TMP_2759(CoreVaultClient.State) := phi(["state_2 (-> ['TMP_2759'])"])
 IAssetManagerEvents.SettingChanged(coreVaultRedemptionFeeBIPS,_redemptionFeeBIPS)
Emit SettingChanged(coreVaultRedemptionFeeBIPS,_redemptionFeeBIPS_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### CoreVaultClientSettingsFacet.setCoreVaultTransferTimeExtensionSeconds(uint256) [EXTERNAL]
```slithir
 state = CoreVaultClient.getState()
TMP_2752(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2752'])(CoreVaultClient.State) := TMP_2752(CoreVaultClient.State)
 state.transferTimeExtensionSeconds = _transferTimeExtensionSeconds.toUint64()
REF_1566(uint64) -> state_1 (-> ['TMP_2752']).transferTimeExtensionSeconds
TMP_2753(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_transferTimeExtensionSeconds_1'] 
state_2 (-> ['TMP_2752'])(CoreVaultClient.State) := phi(["state_1 (-> ['TMP_2752'])"])
REF_1566(uint64) (->state_2 (-> ['TMP_2752'])) := TMP_2753(uint64)
TMP_2752(CoreVaultClient.State) := phi(["state_2 (-> ['TMP_2752'])"])
 IAssetManagerEvents.SettingChanged(coreVaultTransferTimeExtensionSeconds,_transferTimeExtensionSeconds)
Emit SettingChanged(coreVaultTransferTimeExtensionSeconds,_transferTimeExtensionSeconds_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### CoreVaultClientSettingsFacet.updateInterfacesAtCoreVaultDeploy() [PUBLIC]
```slithir
 ds = LibDiamond.diamondStorage()
TMP_2729(LibDiamond.DiamondStorage) = LIBRARY_CALL, dest:LibDiamond, function:LibDiamond.diamondStorage(), arguments:[] 
ds_1 (-> ['TMP_2729'])(LibDiamond.DiamondStorage) := TMP_2729(LibDiamond.DiamondStorage)
 require(bool,error)(ds.supportedInterfaces[type()(IERC165).interfaceId],revert DiamondNotInitialized()())
REF_1546(mapping(bytes4 => bool)) -> ds_1 (-> ['TMP_2729']).supportedInterfaces
TMP_2730(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_1547(bytes4) (->None) := 33540519(bytes4)
REF_1548(bool) -> REF_1546[REF_1547]
TMP_2731(None) = SOLIDITY_CALL revert DiamondNotInitialized()()
TMP_2732(None) = SOLIDITY_CALL require(bool,error)(REF_1548,TMP_2731)
 ds.supportedInterfaces[type()(IAssetManager).interfaceId] = true
REF_1549(mapping(bytes4 => bool)) -> ds_1 (-> ['TMP_2729']).supportedInterfaces
TMP_2733(type(IAssetManager)) = SOLIDITY_CALL type()(IAssetManager)
REF_1550(bytes4) (->None) := 2989796443(bytes4)
REF_1551(bool) -> REF_1549[REF_1550]
ds_2 (-> ['TMP_2729'])(LibDiamond.DiamondStorage) := phi(["ds_1 (-> ['TMP_2729'])"])
REF_1551(bool) (->ds_2 (-> ['TMP_2729'])) := True(bool)
TMP_2729(LibDiamond.DiamondStorage) := phi(["ds_2 (-> ['TMP_2729'])"])
 ds.supportedInterfaces[type()(ICoreVaultClient).interfaceId] = true
REF_1552(mapping(bytes4 => bool)) -> ds_2 (-> ['TMP_2729']).supportedInterfaces
TMP_2734(type(ICoreVaultClient)) = SOLIDITY_CALL type()(ICoreVaultClient)
REF_1553(bytes4) (->None) := 3596308755(bytes4)
REF_1554(bool) -> REF_1552[REF_1553]
ds_3 (-> ['TMP_2729'])(LibDiamond.DiamondStorage) := phi(["ds_2 (-> ['TMP_2729'])"])
REF_1554(bool) (->ds_3 (-> ['TMP_2729'])) := True(bool)
TMP_2729(LibDiamond.DiamondStorage) := phi(["ds_3 (-> ['TMP_2729'])"])
 ds.supportedInterfaces[type()(ICoreVaultClientSettings).interfaceId] = true
REF_1555(mapping(bytes4 => bool)) -> ds_3 (-> ['TMP_2729']).supportedInterfaces
TMP_2735(type(ICoreVaultClientSettings)) = SOLIDITY_CALL type()(ICoreVaultClientSettings)
REF_1556(bytes4) (->None) := 1704453253(bytes4)
REF_1557(bool) -> REF_1555[REF_1556]
ds_4 (-> ['TMP_2729'])(LibDiamond.DiamondStorage) := phi(["ds_3 (-> ['TMP_2729'])"])
REF_1557(bool) (->ds_4 (-> ['TMP_2729'])) := True(bool)
TMP_2729(LibDiamond.DiamondStorage) := phi(["ds_4 (-> ['TMP_2729'])"])
```
#### CoreVaultClient.getState() [INTERNAL]
```slithir
STATE_POSITION_1(bytes32) := phi(['STATE_POSITION_0'])
 position = STATE_POSITION
position_1(bytes32) := STATE_POSITION_1(bytes32)
 _state = position
_state_1 (-> ['position'])(CoreVaultClient.State) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
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
