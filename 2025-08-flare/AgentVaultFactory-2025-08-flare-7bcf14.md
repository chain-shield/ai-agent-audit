### Storage layout (AgentVaultFactory) 

```text
implementation address

```


### Storage layout (AgentVault) 

```text
assetManager IIAssetManager
initialized bool
__usedTokens IERC20[]
__tokenUseFlags mapping(IERC20 => uint256)
__internalWithdrawal bool
destroyed bool

```
#### AgentVaultFactory.constructor(address) [PUBLIC]
```slithir
 implementation = _implementation
implementation_1(address) := _implementation_1(address)
```
#### AgentVaultFactory.create(IIAssetManager) [EXTERNAL]
```slithir
implementation_2(address) := phi(['implementation_0', 'implementation_1'])
 proxy = new ERC1967Proxy(implementation,new bytes(0))
TMP_1359 = new bytes(0)
TMP_1360(ERC1967Proxy) = new ERC1967Proxy(implementation_2,TMP_1359) 
proxy_1(ERC1967Proxy) := TMP_1360(ERC1967Proxy)
 agentVault = AgentVault(address(address(proxy)))
TMP_1361 = CONVERT proxy_1 to address
TMP_1362 = CONVERT TMP_1361 to address
TMP_1363 = CONVERT TMP_1362 to AgentVault
agentVault_1(AgentVault) := TMP_1363(AgentVault)
 agentVault.initialize(_assetManager)
HIGH_LEVEL_CALL, dest:agentVault_1(AgentVault), function:initialize, arguments:['_assetManager_1']  
 agentVault
RETURN agentVault_1
```
#### IERC165.supportsInterface(bytes4) [EXTERNAL]
```slithir

```
#### AgentVaultFactory.upgradeInitCall(address) [EXTERNAL]
```slithir
 new bytes(0)
TMP_1366 = new bytes(0)
RETURN TMP_1366
```
#### AgentVault.initialize(IIAssetManager) [PUBLIC]
```slithir
_assetManager_1(IIAssetManager) := phi(['_assetManager_1'])
initialized_1(bool) := phi(['initialized_2', 'initialized_0'])
 require(bool,error)(! initialized,revert AlreadyInitialized()())
TMP_1274 = UnaryType.BANG initialized_1 
TMP_1275(None) = SOLIDITY_CALL revert AlreadyInitialized()()
TMP_1276(None) = SOLIDITY_CALL require(bool,error)(TMP_1274,TMP_1275)
 initialized = true
initialized_2(bool) := True(bool)
 assetManager = _assetManager
assetManager_1(IIAssetManager) := _assetManager_1(IIAssetManager)
 initializeReentrancyGuard()
INTERNAL_CALL, ReentrancyGuard.initializeReentrancyGuard()()
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
