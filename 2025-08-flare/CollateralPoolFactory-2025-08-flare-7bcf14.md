### Storage layout (AssetManagerMock) 

```text
wNat IWNat
fasset IIFAsset
commonOwner address
checkForValidAgentVaultAddress bool
collateralPool address
maxRedemption uint256
fassetsBackedByPool uint256
timelockDuration uint256
assetPriceMul uint256
assetPriceDiv uint256
lotSize uint256
minPoolCollateralRatioBIPS uint256
assetMintingGranularityUBA uint256

```
### Storage layout (CollateralPool) 

```text
agentVault address
assetManager IIAssetManager
fAsset IFAsset
token IICollateralPoolToken
wNat IWNat
exitCollateralRatioBIPS uint32
__topupCollateralRatioBIPS uint32
__topupTokenPriceFactorBIPS uint16
internalWithdrawal bool
initialized bool
_fAssetFeeDebtOf mapping(address => int256)
totalFAssetFeeDebt int256
totalFAssetFees uint256
totalCollateral uint256

```



### Storage layout (CollateralPoolFactory) 

```text
implementation address

```
#### CollateralPoolFactory.constructor(address) [PUBLIC]
```slithir
 implementation = _implementation
implementation_1(address) := _implementation_1(address)
```
#### CollateralPoolFactory.create(IIAssetManager,address,AgentSettings.Data) [EXTERNAL]
```slithir
implementation_2(address) := phi(['implementation_1', 'implementation_0', 'implementation_3'])
 fAsset = address(_assetManager.fAsset())
TMP_6414(IERC20) = HIGH_LEVEL_CALL, dest:_assetManager_1(IIAssetManager), function:fAsset, arguments:[]  
implementation_3(address) := phi(['implementation_2', 'implementation_1', 'implementation_3'])
TMP_6415 = CONVERT TMP_6414 to address
fAsset_1(address) := TMP_6415(address)
 proxy = new ERC1967Proxy(implementation,new bytes(0))
TMP_6418 = new bytes(0)
TMP_6419(ERC1967Proxy) = new ERC1967Proxy(implementation_3,TMP_6418) 
proxy_1(ERC1967Proxy) := TMP_6419(ERC1967Proxy)
 pool = CollateralPool(address(address(proxy)))
TMP_6420 = CONVERT proxy_1 to address
TMP_6421 = CONVERT TMP_6420 to address
TMP_6422 = CONVERT TMP_6421 to CollateralPool
pool_1(CollateralPool) := TMP_6422(CollateralPool)
 pool.initialize(_agentVault,address(_assetManager),fAsset,_settings.poolExitCollateralRatioBIPS.toUint32())
TMP_6423 = CONVERT _assetManager_1 to address
REF_4296(uint256) -> _settings_1.poolExitCollateralRatioBIPS
TMP_6424(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['REF_4296'] 
HIGH_LEVEL_CALL, dest:pool_1(CollateralPool), function:initialize, arguments:['_agentVault_1', 'TMP_6423', 'fAsset_1', 'TMP_6424']  
 pool
RETURN pool_1
```
#### IERC165.supportsInterface(bytes4) [EXTERNAL]
```slithir

```
#### CollateralPoolFactory.upgradeInitCall(address) [EXTERNAL]
```slithir
 new bytes(0)
TMP_6427 = new bytes(0)
RETURN TMP_6427
```
#### CollateralPool.initialize(address,address,address,uint32) [PUBLIC]
```slithir
_agentVault_1(address) := phi(['_agentVault_1'])
_assetManager_1(address) := phi(['_assetManager_1'])
_fAsset_1(address) := phi(['_fAsset_1'])
_exitCollateralRatioBIPS_1(uint32) := phi(['_exitCollateralRatioBIPS_1'])
initialized_1(bool) := phi(['initialized_2', 'initialized_0'])
 require(bool,error)(! initialized,revert AlreadyInitialized()())
TMP_6043 = UnaryType.BANG initialized_1 
TMP_6044(None) = SOLIDITY_CALL revert AlreadyInitialized()()
TMP_6045(None) = SOLIDITY_CALL require(bool,error)(TMP_6043,TMP_6044)
 initialized = true
initialized_2(bool) := True(bool)
 agentVault = _agentVault
agentVault_1(address) := _agentVault_1(address)
 assetManager = IIAssetManager(_assetManager)
TMP_6046 = CONVERT _assetManager_1 to IIAssetManager
assetManager_1(IIAssetManager) := TMP_6046(IIAssetManager)
 fAsset = IFAsset(_fAsset)
TMP_6047 = CONVERT _fAsset_1 to IFAsset
fAsset_1(IFAsset) := TMP_6047(IFAsset)
 exitCollateralRatioBIPS = _exitCollateralRatioBIPS
exitCollateralRatioBIPS_1(uint32) := _exitCollateralRatioBIPS_1(uint32)
 initializeReentrancyGuard()
INTERNAL_CALL, ReentrancyGuard.initializeReentrancyGuard()()
 address(assetManager) != address(0)
TMP_6049 = CONVERT assetManager_1 to address
TMP_6050 = CONVERT 0 to address
TMP_6051(bool) = TMP_6049 != TMP_6050
CONDITION TMP_6051
 wNat = assetManager.getWNat()
TMP_6052(IWNat) = HIGH_LEVEL_CALL, dest:assetManager_1(IIAssetManager), function:getWNat, arguments:[]  
assetManager_2(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_2(IWNat) := TMP_6052(IWNat)
 wNat = IWNat(address(0))
TMP_6053 = CONVERT 0 to address
TMP_6054 = CONVERT TMP_6053 to IWNat
wNat_1(IWNat) := TMP_6054(IWNat)
wNat_3(IWNat) := phi(['wNat_1', 'wNat_2'])
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
#### AssetManagerMock.getWNat() [EXTERNAL]
```slithir
wNat_2(IWNat) := phi(['wNat_1', 'wNat_0'])
 wNat
RETURN wNat_2
```
#### ReentrancyGuard.initializeReentrancyGuard() [INTERNAL]
```slithir
 Reentrancy.initializeReentrancyGuard()
LIBRARY_CALL, dest:Reentrancy, function:Reentrancy.initializeReentrancyGuard(), arguments:[]
```
#### Reentrancy.initializeReentrancyGuard() [INTERNAL]
```slithir
_NOT_ENTERED_1(uint256) := phi(['_NOT_ENTERED_2', '_NOT_ENTERED_0', '_NOT_ENTERED_4'])
 state = _reentrancyGuardState()
TMP_9987(Reentrancy.ReentrancyGuardState) = INTERNAL_CALL, Reentrancy._reentrancyGuardState()()
state_1 (-> ['TMP_9987'])(Reentrancy.ReentrancyGuardState) := TMP_9987(Reentrancy.ReentrancyGuardState)
 state.status = _NOT_ENTERED
REF_6037(uint256) -> state_1 (-> ['TMP_9987']).status
state_2 (-> ['TMP_9987'])(Reentrancy.ReentrancyGuardState) := phi(["state_1 (-> ['TMP_9987'])"])
REF_6037(uint256) (->state_2 (-> ['TMP_9987'])) := _NOT_ENTERED_2(uint256)
TMP_9987(Reentrancy.ReentrancyGuardState) := phi(["state_2 (-> ['TMP_9987'])"])
```
