
#### AgentOwnerRegistryProxy.constructor(address,IGovernanceSettings,address) [PUBLIC]
```slithir
 ERC1967Proxy(_implementationAddress,abi.encodeCall(AgentOwnerRegistry.initialize,(_governanceSettings,_initialGovernance)))
REF_354(initialize) -> AgentOwnerRegistry.initialize
TMP_1217(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_354,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9e1379a0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9e168400>])
INTERNAL_CALL, ERC1967Proxy.constructor(address,bytes)(_implementationAddress_1,TMP_1217)
 ERC1967Proxy(_implementationAddress,abi.encodeCall(AgentOwnerRegistry.initialize,(_governanceSettings,_initialGovernance)))
REF_356(initialize) -> AgentOwnerRegistry.initialize
TMP_1219(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_356,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9e1379a0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff9e168400>])
INTERNAL_CALL, ERC1967Proxy.constructor(address,bytes)(_implementationAddress_1,TMP_1219)
```
#### AgentOwnerRegistry.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 _IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
 _ADMIN_SLOT = 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103
 _BEACON_SLOT = 0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50
 _timeToExecute()
TMP_1135(bool) = INTERNAL_CALL, GovernedBase._timeToExecute()()
CONDITION TMP_1135
 _beforeExecute()
INTERNAL_CALL, GovernedBase._beforeExecute()()
 _recordTimelockedCall(msg.data,0)
INTERNAL_CALL, GovernedBase._recordTimelockedCall(bytes,uint256)(msg.data,0)
 _timeToExecute()
TMP_1138(bool) = INTERNAL_CALL, GovernedBase._timeToExecute()()
CONDITION TMP_1138
 _beforeExecute()
INTERNAL_CALL, GovernedBase._beforeExecute()()
 _recordTimelockedCall(msg.data,_minimumTimelock)
INTERNAL_CALL, GovernedBase._recordTimelockedCall(bytes,uint256)(msg.data,_minimumTimelock_1)
 _checkOnlyGovernance()
INTERNAL_CALL, GovernedBase._checkOnlyGovernance()()
__self_1(address) := phi(['__self_2', '__self_0'])
 require(bool,string)(address(this) != __self,Function must be called through delegatecall)
TMP_1142 = CONVERT this to address
TMP_1143(bool) = TMP_1142 != __self_1
TMP_1144(None) = SOLIDITY_CALL require(bool,string)(TMP_1143,Function must be called through delegatecall)
 require(bool,string)(_getImplementation() == __self,Function must be called through active proxy)
TMP_1145(address) = INTERNAL_CALL, ERC1967Upgrade._getImplementation()()
TMP_1146(bool) = TMP_1145 == __self_2
TMP_1147(None) = SOLIDITY_CALL require(bool,string)(TMP_1146,Function must be called through active proxy)
__self_3(address) := phi(['__self_2', '__self_0'])
 require(bool,string)(address(this) == __self,UUPSUpgradeable: must not be called through delegatecall)
TMP_1148 = CONVERT this to address
TMP_1149(bool) = TMP_1148 == __self_3
TMP_1150(None) = SOLIDITY_CALL require(bool,string)(TMP_1149,UUPSUpgradeable: must not be called through delegatecall)
manager_2(address) := phi(['manager_0', 'manager_3', 'manager_1'])
 require(bool,error)(msg.sender == manager || msg.sender == governance(),revert OnlyGovernanceOrManager()())
TMP_1151(bool) = msg.sender == manager_2
TMP_1152(address) = INTERNAL_CALL, GovernedBase.governance()()
TMP_1153(bool) = msg.sender == TMP_1152
TMP_1154(bool) = TMP_1151 || TMP_1153
TMP_1155(None) = SOLIDITY_CALL revert OnlyGovernanceOrManager()()
TMP_1156(None) = SOLIDITY_CALL require(bool,error)(TMP_1154,TMP_1155)
```
