
#### CoreVaultManagerProxy.constructor(address,IGovernanceSettings,address,address,address,bytes32,string,string,uint256) [PUBLIC]
```slithir
 ERC1967Proxy(_implementationAddress,abi.encodeCall(CoreVaultManager.initialize,(_governanceSettings,_initialGovernance,_addressUpdater,_assetManager,_chainId,_custodianAddress,_coreVaultAddress,_nextSequenceNumber)))
REF_4690(initialize) -> CoreVaultManager.initialize
TMP_7238(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4690,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86e8f0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86f280>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86f460>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86f640>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86f6a0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86f700>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86f760>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86f7c0>])
INTERNAL_CALL, ERC1967Proxy.constructor(address,bytes)(_implementationAddress_1,TMP_7238)
 ERC1967Proxy(_implementationAddress,abi.encodeCall(CoreVaultManager.initialize,(_governanceSettings,_initialGovernance,_addressUpdater,_assetManager,_chainId,_custodianAddress,_coreVaultAddress,_nextSequenceNumber)))
REF_4692(initialize) -> CoreVaultManager.initialize
TMP_7240(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4692,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86e8f0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86f280>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86f460>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86f640>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86f6a0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86f700>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86f760>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9b86f7c0>])
INTERNAL_CALL, ERC1967Proxy.constructor(address,bytes)(_implementationAddress_1,TMP_7240)
```
#### CoreVaultManager.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 _IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
 _ADMIN_SLOT = 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103
 _BEACON_SLOT = 0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50
 ADDRESS_STORAGE_POSITION = keccak256(bytes)(flare.diamond.AddressUpdatable.ADDRESS_STORAGE_POSITION)
 require(bool,error)(msg.sender == getAddressUpdater(),revert OnlyAddressUpdater()())
TMP_7156(address) = INTERNAL_CALL, AddressUpdatable.getAddressUpdater()()
TMP_7157(bool) = msg.sender == TMP_7156
TMP_7158(None) = SOLIDITY_CALL revert OnlyAddressUpdater()()
TMP_7159(None) = SOLIDITY_CALL require(bool,error)(TMP_7157,TMP_7158)
 _timeToExecute()
TMP_7160(bool) = INTERNAL_CALL, GovernedBase._timeToExecute()()
CONDITION TMP_7160
 _beforeExecute()
INTERNAL_CALL, GovernedBase._beforeExecute()()
 _recordTimelockedCall(msg.data,0)
INTERNAL_CALL, GovernedBase._recordTimelockedCall(bytes,uint256)(msg.data,0)
 _timeToExecute()
TMP_7163(bool) = INTERNAL_CALL, GovernedBase._timeToExecute()()
CONDITION TMP_7163
 _beforeExecute()
INTERNAL_CALL, GovernedBase._beforeExecute()()
 _recordTimelockedCall(msg.data,_minimumTimelock)
INTERNAL_CALL, GovernedBase._recordTimelockedCall(bytes,uint256)(msg.data,_minimumTimelock_1)
 _checkOnlyGovernance()
INTERNAL_CALL, GovernedBase._checkOnlyGovernance()()
__self_1(address) := phi(['__self_0', '__self_2'])
 require(bool,string)(address(this) != __self,Function must be called through delegatecall)
TMP_7167 = CONVERT this to address
TMP_7168(bool) = TMP_7167 != __self_1
TMP_7169(None) = SOLIDITY_CALL require(bool,string)(TMP_7168,Function must be called through delegatecall)
 require(bool,string)(_getImplementation() == __self,Function must be called through active proxy)
TMP_7170(address) = INTERNAL_CALL, ERC1967Upgrade._getImplementation()()
TMP_7171(bool) = TMP_7170 == __self_2
TMP_7172(None) = SOLIDITY_CALL require(bool,string)(TMP_7171,Function must be called through active proxy)
__self_3(address) := phi(['__self_0', '__self_2'])
 require(bool,string)(address(this) == __self,UUPSUpgradeable: must not be called through delegatecall)
TMP_7173 = CONVERT this to address
TMP_7174(bool) = TMP_7173 == __self_3
TMP_7175(None) = SOLIDITY_CALL require(bool,string)(TMP_7174,UUPSUpgradeable: must not be called through delegatecall)
 _checkOnlyAssetManager()
INTERNAL_CALL, CoreVaultManager._checkOnlyAssetManager()()
 _checkNotPaused()
INTERNAL_CALL, CoreVaultManager._checkNotPaused()()
```
