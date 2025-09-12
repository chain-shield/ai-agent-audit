
#### FAssetProxy.constructor(address,string,string,string,string,uint8) [PUBLIC]
```slithir
 ERC1967Proxy(_implementationAddress,abi.encodeCall(FAsset.initialize,(_name,_symbol,_assetName,_assetSymbol,_decimals)))
REF_5024(initialize) -> FAsset.initialize
TMP_7966(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_5024,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b33f490>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b33f790>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b33f7f0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b33f850>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b33f8b0>])
INTERNAL_CALL, ERC1967Proxy.constructor(address,bytes)(_implementationAddress_1,TMP_7966)
 ERC1967Proxy(_implementationAddress,abi.encodeCall(FAsset.initialize,(_name,_symbol,_assetName,_assetSymbol,_decimals)))
REF_5026(initialize) -> FAsset.initialize
TMP_7968(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_5026,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b33f490>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b33f790>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b33f7f0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b33f850>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b33f8b0>])
INTERNAL_CALL, ERC1967Proxy.constructor(address,bytes)(_implementationAddress_1,TMP_7968)
```
#### FAsset.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 _IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
 _ADMIN_SLOT = 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103
 _BEACON_SLOT = 0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50
__self_1(address) := phi(['__self_0', '__self_2'])
 require(bool,string)(address(this) != __self,Function must be called through delegatecall)
TMP_7888 = CONVERT this to address
TMP_7889(bool) = TMP_7888 != __self_1
TMP_7890(None) = SOLIDITY_CALL require(bool,string)(TMP_7889,Function must be called through delegatecall)
 require(bool,string)(_getImplementation() == __self,Function must be called through active proxy)
TMP_7891(address) = INTERNAL_CALL, ERC1967Upgrade._getImplementation()()
TMP_7892(bool) = TMP_7891 == __self_2
TMP_7893(None) = SOLIDITY_CALL require(bool,string)(TMP_7892,Function must be called through active proxy)
__self_3(address) := phi(['__self_0', '__self_2'])
 require(bool,string)(address(this) == __self,UUPSUpgradeable: must not be called through delegatecall)
TMP_7894 = CONVERT this to address
TMP_7895(bool) = TMP_7894 == __self_3
TMP_7896(None) = SOLIDITY_CALL require(bool,string)(TMP_7895,UUPSUpgradeable: must not be called through delegatecall)
_blockNumber_1(uint256) := phi(['_blockNumber_1', '_blockNumber_1'])
cleanupBlockNumber_15(uint256) := phi(['cleanupBlockNumber_0', 'cleanupBlockNumber_6', 'cleanupBlockNumber_12', 'cleanupBlockNumber_9', 'cleanupBlockNumber_3', 'cleanupBlockNumber_14'])
 require(bool,error)(_blockNumber >= cleanupBlockNumber,revert CheckPointableReadingFromCleanedupBlock()())
TMP_7897(bool) = _blockNumber_1 >= cleanupBlockNumber_15
TMP_7898(None) = SOLIDITY_CALL revert CheckPointableReadingFromCleanedupBlock()()
TMP_7899(None) = SOLIDITY_CALL require(bool,error)(TMP_7897,TMP_7898)
cleanerContract_2(address) := phi(['cleanerContract_0', 'cleanerContract_1'])
 require(bool,error)(msg.sender == cleanerContract,revert OnlyCleanerContract()())
TMP_7900(bool) = msg.sender == cleanerContract_2
TMP_7901(None) = SOLIDITY_CALL revert OnlyCleanerContract()()
TMP_7902(None) = SOLIDITY_CALL require(bool,error)(TMP_7900,TMP_7901)
assetManager_6(address) := phi(['assetManager_0', 'assetManager_2', 'assetManager_5'])
 require(bool,error)(msg.sender == assetManager,revert OnlyAssetManager()())
TMP_7903(bool) = msg.sender == assetManager_6
TMP_7904(None) = SOLIDITY_CALL revert OnlyAssetManager()()
TMP_7905(None) = SOLIDITY_CALL require(bool,error)(TMP_7903,TMP_7904)
```
