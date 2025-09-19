
#### FtsoV2PriceStoreProxy.constructor(address,IGovernanceSettings,address,address,uint64,uint8,uint8) [PUBLIC]
```slithir
 ERC1967Proxy(_implementationAddress,abi.encodeCall(FtsoV2PriceStore.initialize,(_governanceSettings,_initialGovernance,_addressUpdater,_firstVotingRoundStartTs,_votingEpochDurationSeconds,_ftsoProtocolId)))
REF_5569(initialize) -> FtsoV2PriceStore.initialize
TMP_8975(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_5569,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff81c4f280>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff81c4fc10>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff81c4fdf0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff81a8c040>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff81a8c070>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff81a8c0d0>])
INTERNAL_CALL, ERC1967Proxy.constructor(address,bytes)(_implementationAddress_1,TMP_8975)
 ERC1967Proxy(_implementationAddress,abi.encodeCall(FtsoV2PriceStore.initialize,(_governanceSettings,_initialGovernance,_addressUpdater,_firstVotingRoundStartTs,_votingEpochDurationSeconds,_ftsoProtocolId)))
REF_5571(initialize) -> FtsoV2PriceStore.initialize
TMP_8977(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_5571,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff81c4f280>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff81c4fc10>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff81c4fdf0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff81a8c040>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff81a8c070>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff81a8c0d0>])
INTERNAL_CALL, ERC1967Proxy.constructor(address,bytes)(_implementationAddress_1,TMP_8977)
```
#### FtsoV2PriceStore.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 _IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
 _ADMIN_SLOT = 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103
 _BEACON_SLOT = 0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50
 ADDRESS_STORAGE_POSITION = keccak256(bytes)(flare.diamond.AddressUpdatable.ADDRESS_STORAGE_POSITION)
 MAX_BIPS = 1e4
 require(bool,error)(msg.sender == getAddressUpdater(),revert OnlyAddressUpdater()())
TMP_8895(address) = INTERNAL_CALL, AddressUpdatable.getAddressUpdater()()
TMP_8896(bool) = msg.sender == TMP_8895
TMP_8897(None) = SOLIDITY_CALL revert OnlyAddressUpdater()()
TMP_8898(None) = SOLIDITY_CALL require(bool,error)(TMP_8896,TMP_8897)
 _timeToExecute()
TMP_8899(bool) = INTERNAL_CALL, GovernedBase._timeToExecute()()
CONDITION TMP_8899
 _beforeExecute()
INTERNAL_CALL, GovernedBase._beforeExecute()()
 _recordTimelockedCall(msg.data,0)
INTERNAL_CALL, GovernedBase._recordTimelockedCall(bytes,uint256)(msg.data,0)
 _timeToExecute()
TMP_8902(bool) = INTERNAL_CALL, GovernedBase._timeToExecute()()
CONDITION TMP_8902
 _beforeExecute()
INTERNAL_CALL, GovernedBase._beforeExecute()()
 _recordTimelockedCall(msg.data,_minimumTimelock)
INTERNAL_CALL, GovernedBase._recordTimelockedCall(bytes,uint256)(msg.data,_minimumTimelock_1)
 _checkOnlyGovernance()
INTERNAL_CALL, GovernedBase._checkOnlyGovernance()()
__self_1(address) := phi(['__self_2', '__self_0'])
 require(bool,string)(address(this) != __self,Function must be called through delegatecall)
TMP_8906 = CONVERT this to address
TMP_8907(bool) = TMP_8906 != __self_1
TMP_8908(None) = SOLIDITY_CALL require(bool,string)(TMP_8907,Function must be called through delegatecall)
 require(bool,string)(_getImplementation() == __self,Function must be called through active proxy)
TMP_8909(address) = INTERNAL_CALL, ERC1967Upgrade._getImplementation()()
TMP_8910(bool) = TMP_8909 == __self_2
TMP_8911(None) = SOLIDITY_CALL require(bool,string)(TMP_8910,Function must be called through active proxy)
__self_3(address) := phi(['__self_2', '__self_0'])
 require(bool,string)(address(this) == __self,UUPSUpgradeable: must not be called through delegatecall)
TMP_8912 = CONVERT this to address
TMP_8913(bool) = TMP_8912 == __self_3
TMP_8914(None) = SOLIDITY_CALL require(bool,string)(TMP_8913,UUPSUpgradeable: must not be called through delegatecall)
```
