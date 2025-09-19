### Storage layout (AgentOwnerRegistry) 

```text
manager address
whitelist mapping(address => bool)
workToMgmtAddress mapping(address => address)
mgmtToWorkAddress mapping(address => address)
agentName mapping(address => string)
agentDescription mapping(address => string)
agentIconUrl mapping(address => string)
agentTouUrl mapping(address => string)

```

#### AgentOwnerRegistry._addAddressToWhitelist(address) [INTERNAL]
```slithir
_address_1(address) := phi(['_managementAddress_1'])
whitelist_2(mapping(address => bool)) := phi(['whitelist_2', 'whitelist_1', 'whitelist_4', 'whitelist_5', 'whitelist_0', 'whitelist_3'])
 require(bool,error)(_address != address(0),revert AddressZero()())
TMP_1121 = CONVERT 0 to address
TMP_1122(bool) = _address_1 != TMP_1121
TMP_1123(None) = SOLIDITY_CALL revert AddressZero()()
TMP_1124(None) = SOLIDITY_CALL require(bool,error)(TMP_1122,TMP_1123)
 whitelist[_address]
REF_314(bool) -> whitelist_2[_address_1]
CONDITION REF_314
 whitelist[_address] = true
REF_315(bool) -> whitelist_2[_address_1]
whitelist_3(mapping(address => bool)) := phi(['whitelist_2'])
REF_315(bool) (->whitelist_3) := True(bool)
 Whitelisted(_address)
Emit Whitelisted(_address_1)
```
#### AgentOwnerRegistry._emitDataChanged(address) [PRIVATE]
```slithir
_managementAddress_1(address) := phi(['_managementAddress_1', '_managementAddress_1', '_managementAddress_1', '_managementAddress_1'])
agentName_4(mapping(address => string)) := phi(['agentName_1', 'agentName_3', 'agentName_4', 'agentName_0', 'agentName_2'])
agentDescription_4(mapping(address => string)) := phi(['agentDescription_0', 'agentDescription_2', 'agentDescription_1', 'agentDescription_3', 'agentDescription_4'])
agentIconUrl_4(mapping(address => string)) := phi(['agentIconUrl_0', 'agentIconUrl_2', 'agentIconUrl_3', 'agentIconUrl_4', 'agentIconUrl_1'])
agentTouUrl_4(mapping(address => string)) := phi(['agentTouUrl_3', 'agentTouUrl_1', 'agentTouUrl_0', 'agentTouUrl_2', 'agentTouUrl_4'])
 AgentDataChanged(_managementAddress,agentName[_managementAddress],agentDescription[_managementAddress],agentIconUrl[_managementAddress],agentTouUrl[_managementAddress])
REF_322(string) -> agentName_4[_managementAddress_1]
REF_323(string) -> agentDescription_4[_managementAddress_1]
REF_324(string) -> agentIconUrl_4[_managementAddress_1]
REF_325(string) -> agentTouUrl_4[_managementAddress_1]
Emit AgentDataChanged(_managementAddress_1,REF_322,REF_323,REF_324,REF_325)
```
#### AgentOwnerRegistry._removeAddressFromWhitelist(address) [INTERNAL]
```slithir
_address_1(address) := phi(['_address_1'])
whitelist_4(mapping(address => bool)) := phi(['whitelist_2', 'whitelist_1', 'whitelist_4', 'whitelist_5', 'whitelist_0', 'whitelist_3'])
 ! whitelist[_address]
REF_316(bool) -> whitelist_4[_address_1]
TMP_1126 = UnaryType.BANG REF_316 
CONDITION TMP_1126
 delete whitelist[_address]
REF_317(bool) -> whitelist_4[_address_1]
whitelist_5 = delete REF_317 
 WhitelistingRevoked(_address)
Emit WhitelistingRevoked(_address_1)
```
#### AgentOwnerRegistry._setAgentData(address,string,string,string,string) [PRIVATE]
```slithir
_managementAddress_1(address) := phi(['_managementAddress_1'])
_name_1(string) := phi(['_name_1'])
_description_1(string) := phi(['_description_1'])
_iconUrl_1(string) := phi(['_iconUrl_1'])
_touUrl_1(string) := phi(['_touUrl_1'])
 agentName[_managementAddress] = _name
REF_318(string) -> agentName_2[_managementAddress_1]
agentName_3(mapping(address => string)) := phi(['agentName_2'])
REF_318(string) (->agentName_3) := _name_1(string)
 agentDescription[_managementAddress] = _description
REF_319(string) -> agentDescription_2[_managementAddress_1]
agentDescription_3(mapping(address => string)) := phi(['agentDescription_2'])
REF_319(string) (->agentDescription_3) := _description_1(string)
 agentIconUrl[_managementAddress] = _iconUrl
REF_320(string) -> agentIconUrl_2[_managementAddress_1]
agentIconUrl_3(mapping(address => string)) := phi(['agentIconUrl_2'])
REF_320(string) (->agentIconUrl_3) := _iconUrl_1(string)
 agentTouUrl[_managementAddress] = _touUrl
REF_321(string) -> agentTouUrl_2[_managementAddress_1]
agentTouUrl_3(mapping(address => string)) := phi(['agentTouUrl_2'])
REF_321(string) (->agentTouUrl_3) := _touUrl_1(string)
 AgentDataChanged(_managementAddress,_name,_description,_iconUrl,_touUrl)
Emit AgentDataChanged(_managementAddress_1,_name_1,_description_1,_iconUrl_1,_touUrl_1)
```
#### AgentOwnerRegistry.getAgentDescription(address) [EXTERNAL]
```slithir
agentDescription_2(mapping(address => string)) := phi(['agentDescription_0', 'agentDescription_2', 'agentDescription_1', 'agentDescription_3', 'agentDescription_4'])
 agentDescription[_managementAddress]
REF_308(string) -> agentDescription_2[_managementAddress_1]
RETURN REF_308
```
#### AgentOwnerRegistry.getAgentIconUrl(address) [EXTERNAL]
```slithir
agentIconUrl_2(mapping(address => string)) := phi(['agentIconUrl_0', 'agentIconUrl_2', 'agentIconUrl_3', 'agentIconUrl_4', 'agentIconUrl_1'])
 agentIconUrl[_managementAddress]
REF_309(string) -> agentIconUrl_2[_managementAddress_1]
RETURN REF_309
```
#### AgentOwnerRegistry.getAgentName(address) [EXTERNAL]
```slithir
agentName_2(mapping(address => string)) := phi(['agentName_1', 'agentName_3', 'agentName_4', 'agentName_0', 'agentName_2'])
 agentName[_managementAddress]
REF_307(string) -> agentName_2[_managementAddress_1]
RETURN REF_307
```
#### AgentOwnerRegistry.getAgentTermsOfUseUrl(address) [EXTERNAL]
```slithir
agentTouUrl_2(mapping(address => string)) := phi(['agentTouUrl_3', 'agentTouUrl_1', 'agentTouUrl_0', 'agentTouUrl_2', 'agentTouUrl_4'])
 agentTouUrl[_managementAddress]
REF_310(string) -> agentTouUrl_2[_managementAddress_1]
RETURN REF_310
```
#### AgentOwnerRegistry.getManagementAddress(address) [EXTERNAL]
```slithir
workToMgmtAddress_5(mapping(address => address)) := phi(['workToMgmtAddress_5', 'workToMgmtAddress_2', 'workToMgmtAddress_3', 'workToMgmtAddress_0', 'workToMgmtAddress_4'])
 workToMgmtAddress[_workAddress]
REF_312(address) -> workToMgmtAddress_5[_workAddress_1]
RETURN REF_312
```
#### AgentOwnerRegistry.getWorkAddress(address) [EXTERNAL]
```slithir
mgmtToWorkAddress_4(mapping(address => address)) := phi(['mgmtToWorkAddress_3', 'mgmtToWorkAddress_0', 'mgmtToWorkAddress_4'])
 mgmtToWorkAddress[_managementAddress]
REF_311(address) -> mgmtToWorkAddress_4[_managementAddress_1]
RETURN REF_311
```
#### AgentOwnerRegistry.initialize(IGovernanceSettings,address) [EXTERNAL]
```slithir
 initialise(_governanceSettings,_initialGovernance)
INTERNAL_CALL, GovernedBase.initialise(IGovernanceSettings,address)(_governanceSettings_1,_initialGovernance_1)
```
#### AgentOwnerRegistry.isWhitelisted(address) [PUBLIC]
```slithir
_address_1(address) := phi(['msg.sender'])
whitelist_1(mapping(address => bool)) := phi(['whitelist_2', 'whitelist_1', 'whitelist_4', 'whitelist_5', 'whitelist_0', 'whitelist_3'])
 whitelist[_address]
REF_313(bool) -> whitelist_1[_address_1]
RETURN REF_313
```
#### AgentOwnerRegistry.revokeAddress(address) [EXTERNAL]
```slithir
 _removeAddressFromWhitelist(_address)
INTERNAL_CALL, AgentOwnerRegistry._removeAddressFromWhitelist(address)(_address_1)
 onlyGovernanceOrManager()
MODIFIER_CALL, AgentOwnerRegistry.onlyGovernanceOrManager()()
```
#### AgentOwnerRegistry.setAgentDescription(address,string) [EXTERNAL]
```slithir
 agentDescription[_managementAddress] = _description
REF_304(string) -> agentDescription_0[_managementAddress_1]
agentDescription_1(mapping(address => string)) := phi(['agentDescription_0'])
REF_304(string) (->agentDescription_1) := _description_1(string)
 _emitDataChanged(_managementAddress)
INTERNAL_CALL, AgentOwnerRegistry._emitDataChanged(address)(_managementAddress_1)
 onlyGovernanceOrManager()
MODIFIER_CALL, AgentOwnerRegistry.onlyGovernanceOrManager()()
```
#### AgentOwnerRegistry.setAgentIconUrl(address,string) [EXTERNAL]
```slithir
 agentIconUrl[_managementAddress] = _iconUrl
REF_305(string) -> agentIconUrl_0[_managementAddress_1]
agentIconUrl_1(mapping(address => string)) := phi(['agentIconUrl_0'])
REF_305(string) (->agentIconUrl_1) := _iconUrl_1(string)
 _emitDataChanged(_managementAddress)
INTERNAL_CALL, AgentOwnerRegistry._emitDataChanged(address)(_managementAddress_1)
 onlyGovernanceOrManager()
MODIFIER_CALL, AgentOwnerRegistry.onlyGovernanceOrManager()()
```
#### AgentOwnerRegistry.setAgentName(address,string) [EXTERNAL]
```slithir
 agentName[_managementAddress] = _name
REF_303(string) -> agentName_0[_managementAddress_1]
agentName_1(mapping(address => string)) := phi(['agentName_0'])
REF_303(string) (->agentName_1) := _name_1(string)
 _emitDataChanged(_managementAddress)
INTERNAL_CALL, AgentOwnerRegistry._emitDataChanged(address)(_managementAddress_1)
 onlyGovernanceOrManager()
MODIFIER_CALL, AgentOwnerRegistry.onlyGovernanceOrManager()()
```
#### AgentOwnerRegistry.setAgentTermsOfUseUrl(address,string) [EXTERNAL]
```slithir
 agentTouUrl[_managementAddress] = _touUrl
REF_306(string) -> agentTouUrl_0[_managementAddress_1]
agentTouUrl_1(mapping(address => string)) := phi(['agentTouUrl_0'])
REF_306(string) (->agentTouUrl_1) := _touUrl_1(string)
 _emitDataChanged(_managementAddress)
INTERNAL_CALL, AgentOwnerRegistry._emitDataChanged(address)(_managementAddress_1)
 onlyGovernanceOrManager()
MODIFIER_CALL, AgentOwnerRegistry.onlyGovernanceOrManager()()
```
#### AgentOwnerRegistry.setManager(address) [EXTERNAL]
```slithir
 manager = _manager
manager_1(address) := _manager_1(address)
 ManagerChanged(_manager)
Emit ManagerChanged(_manager_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### AgentOwnerRegistry.setWorkAddress(address) [EXTERNAL]
```slithir
workToMgmtAddress_1(mapping(address => address)) := phi(['workToMgmtAddress_5', 'workToMgmtAddress_2', 'workToMgmtAddress_3', 'workToMgmtAddress_0', 'workToMgmtAddress_4'])
mgmtToWorkAddress_1(mapping(address => address)) := phi(['mgmtToWorkAddress_3', 'mgmtToWorkAddress_0', 'mgmtToWorkAddress_4'])
 require(bool,error)(isWhitelisted(msg.sender),revert AgentNotWhitelisted()())
TMP_1097(bool) = INTERNAL_CALL, AgentOwnerRegistry.isWhitelisted(address)(msg.sender)
TMP_1098(None) = SOLIDITY_CALL revert AgentNotWhitelisted()()
TMP_1099(None) = SOLIDITY_CALL require(bool,error)(TMP_1097,TMP_1098)
 require(bool,error)(_ownerWorkAddress == address(0) || workToMgmtAddress[_ownerWorkAddress] == address(0),revert WorkAddressInUse()())
TMP_1100 = CONVERT 0 to address
TMP_1101(bool) = _ownerWorkAddress_1 == TMP_1100
REF_298(address) -> workToMgmtAddress_2[_ownerWorkAddress_1]
TMP_1102 = CONVERT 0 to address
TMP_1103(bool) = REF_298 == TMP_1102
TMP_1104(bool) = TMP_1101 || TMP_1103
TMP_1105(None) = SOLIDITY_CALL revert WorkAddressInUse()()
TMP_1106(None) = SOLIDITY_CALL require(bool,error)(TMP_1104,TMP_1105)
 oldWorkAddress = mgmtToWorkAddress[msg.sender]
REF_299(address) -> mgmtToWorkAddress_2[msg.sender]
oldWorkAddress_1(address) := REF_299(address)
 oldWorkAddress != address(0)
TMP_1107 = CONVERT 0 to address
TMP_1108(bool) = oldWorkAddress_1 != TMP_1107
CONDITION TMP_1108
 workToMgmtAddress[oldWorkAddress] = address(0)
REF_300(address) -> workToMgmtAddress_2[oldWorkAddress_1]
TMP_1109 = CONVERT 0 to address
workToMgmtAddress_3(mapping(address => address)) := phi(['workToMgmtAddress_2'])
REF_300(address) (->workToMgmtAddress_3) := TMP_1109(address)
 mgmtToWorkAddress[msg.sender] = _ownerWorkAddress
REF_301(address) -> mgmtToWorkAddress_2[msg.sender]
mgmtToWorkAddress_3(mapping(address => address)) := phi(['mgmtToWorkAddress_2'])
REF_301(address) (->mgmtToWorkAddress_3) := _ownerWorkAddress_1(address)
 _ownerWorkAddress != address(0)
TMP_1110 = CONVERT 0 to address
TMP_1111(bool) = _ownerWorkAddress_1 != TMP_1110
CONDITION TMP_1111
 workToMgmtAddress[_ownerWorkAddress] = msg.sender
REF_302(address) -> workToMgmtAddress_3[_ownerWorkAddress_1]
workToMgmtAddress_4(mapping(address => address)) := phi(['workToMgmtAddress_3'])
REF_302(address) (->workToMgmtAddress_4) := msg.sender(address)
 WorkAddressChanged(msg.sender,oldWorkAddress,_ownerWorkAddress)
Emit WorkAddressChanged(msg.sender,oldWorkAddress_1,_ownerWorkAddress_1)
```

#### AgentOwnerRegistry.supportsInterface(bytes4) [PUBLIC]
```slithir
 _interfaceId == type()(IERC165).interfaceId || _interfaceId == type()(IAgentOwnerRegistry).interfaceId
TMP_1130(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_326(bytes4) (->None) := 33540519(bytes4)
TMP_1131(bool) = _interfaceId_1 == REF_326
TMP_1132(type(IAgentOwnerRegistry)) = SOLIDITY_CALL type()(IAgentOwnerRegistry)
REF_327(bytes4) (->None) := 2807267766(bytes4)
TMP_1133(bool) = _interfaceId_1 == REF_327
TMP_1134(bool) = TMP_1131 || TMP_1133
RETURN TMP_1134
```
#### AgentOwnerRegistry.whitelistAndDescribeAgent(address,string,string,string,string) [EXTERNAL]
```slithir
 _addAddressToWhitelist(_managementAddress)
INTERNAL_CALL, AgentOwnerRegistry._addAddressToWhitelist(address)(_managementAddress_1)
 _setAgentData(_managementAddress,_name,_description,_iconUrl,_touUrl)
INTERNAL_CALL, AgentOwnerRegistry._setAgentData(address,string,string,string,string)(_managementAddress_1,_name_1,_description_1,_iconUrl_1,_touUrl_1)
 onlyGovernanceOrManager()
MODIFIER_CALL, AgentOwnerRegistry.onlyGovernanceOrManager()()
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
