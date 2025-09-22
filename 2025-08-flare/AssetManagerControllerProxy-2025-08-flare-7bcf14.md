
#### AssetManagerControllerProxy.constructor(address,IGovernanceSettings,address,address) [PUBLIC]
```slithir
 ERC1967Proxy(_implementationAddress,abi.encodeCall(AssetManagerController.initialize,(_governanceSettings,_initialGovernance,_addressUpdater)))
REF_4152(initialize) -> AssetManagerController.initialize
TMP_5986(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4152,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff82ff6770>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff82ff7100>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff82ff72e0>])
INTERNAL_CALL, ERC1967Proxy.constructor(address,bytes)(_implementationAddress_1,TMP_5986)
 ERC1967Proxy(_implementationAddress,abi.encodeCall(AssetManagerController.initialize,(_governanceSettings,_initialGovernance,_addressUpdater)))
REF_4154(initialize) -> AssetManagerController.initialize
TMP_5988(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_4154,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff82ff6770>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff82ff7100>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff82ff72e0>])
INTERNAL_CALL, ERC1967Proxy.constructor(address,bytes)(_implementationAddress_1,TMP_5988)
```
#### AssetManagerController.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 _IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
 _ADMIN_SLOT = 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103
 _BEACON_SLOT = 0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50
 ADDRESS_STORAGE_POSITION = keccak256(bytes)(flare.diamond.AddressUpdatable.ADDRESS_STORAGE_POSITION)
 require(bool,error)(msg.sender == getAddressUpdater(),revert OnlyAddressUpdater()())
TMP_5906(address) = INTERNAL_CALL, AddressUpdatable.getAddressUpdater()()
TMP_5907(bool) = msg.sender == TMP_5906
TMP_5908(None) = SOLIDITY_CALL revert OnlyAddressUpdater()()
TMP_5909(None) = SOLIDITY_CALL require(bool,error)(TMP_5907,TMP_5908)
 _timeToExecute()
TMP_5910(bool) = INTERNAL_CALL, GovernedBase._timeToExecute()()
CONDITION TMP_5910
 _beforeExecute()
INTERNAL_CALL, GovernedBase._beforeExecute()()
 _recordTimelockedCall(msg.data,0)
INTERNAL_CALL, GovernedBase._recordTimelockedCall(bytes,uint256)(msg.data,0)
 _timeToExecute()
TMP_5913(bool) = INTERNAL_CALL, GovernedBase._timeToExecute()()
CONDITION TMP_5913
 _beforeExecute()
INTERNAL_CALL, GovernedBase._beforeExecute()()
 _recordTimelockedCall(msg.data,_minimumTimelock)
INTERNAL_CALL, GovernedBase._recordTimelockedCall(bytes,uint256)(msg.data,_minimumTimelock_1)
 _checkOnlyGovernance()
INTERNAL_CALL, GovernedBase._checkOnlyGovernance()()
__self_1(address) := phi(['__self_0', '__self_2'])
 require(bool,string)(address(this) != __self,Function must be called through delegatecall)
TMP_5917 = CONVERT this to address
TMP_5918(bool) = TMP_5917 != __self_1
TMP_5919(None) = SOLIDITY_CALL require(bool,string)(TMP_5918,Function must be called through delegatecall)
 require(bool,string)(_getImplementation() == __self,Function must be called through active proxy)
TMP_5920(address) = INTERNAL_CALL, ERC1967Upgrade._getImplementation()()
TMP_5921(bool) = TMP_5920 == __self_2
TMP_5922(None) = SOLIDITY_CALL require(bool,string)(TMP_5921,Function must be called through active proxy)
__self_3(address) := phi(['__self_0', '__self_2'])
 require(bool,string)(address(this) == __self,UUPSUpgradeable: must not be called through delegatecall)
TMP_5923 = CONVERT this to address
TMP_5924(bool) = TMP_5923 == __self_3
TMP_5925(None) = SOLIDITY_CALL require(bool,string)(TMP_5924,UUPSUpgradeable: must not be called through delegatecall)
```
