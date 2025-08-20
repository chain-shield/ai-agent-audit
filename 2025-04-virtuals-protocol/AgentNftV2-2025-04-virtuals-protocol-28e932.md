### Storage layout (AgentNftV2) 

```text
_nextVirtualId uint256
_stakingTokenToVirtualId mapping(address => uint256)
virtualInfos mapping(uint256 => IAgentNft.VirtualInfo)
_contributionNft address
_serviceNft address
_blacklists mapping(uint256 => bool)
virtualLPs mapping(uint256 => IAgentNft.VirtualLP)
_eloCalculator address

```




#### AgentNftV2._getPastValidatorScore(uint256,address,uint256) [INTERNAL]
```slithir
virtualInfos_11(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(['virtualInfos_0', 'virtualInfos_26', 'virtualInfos_28', 'virtualInfos_22', 'virtualInfos_9', 'virtualInfos_10', 'virtualInfos_15', 'virtualInfos_18', 'virtualInfos_8', 'virtualInfos_12', 'virtualInfos_21', 'virtualInfos_11'])
 info = virtualInfos[virtualId]
REF_5448(IAgentNft.VirtualInfo) -> virtualInfos_11[virtualId_1]
info_1(IAgentNft.VirtualInfo) := REF_5448(IAgentNft.VirtualInfo)
 dao = IAgentDAO(info.dao)
REF_5449(address) -> info_1.dao
TMP_13236 = CONVERT REF_5449 to IAgentDAO
dao_1(IAgentDAO) := TMP_13236(IAgentDAO)
 dao.getPastScore(account,timepoint)
TMP_13237(uint256) = HIGH_LEVEL_CALL, dest:dao_1(IAgentDAO), function:getPastScore, arguments:['account_1', 'timepoint_1']  
RETURN TMP_13237
```
#### AgentNftV2._validatorScoreOf(uint256,address) [INTERNAL]
```slithir
virtualInfos_10(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(['virtualInfos_0', 'virtualInfos_26', 'virtualInfos_28', 'virtualInfos_22', 'virtualInfos_9', 'virtualInfos_10', 'virtualInfos_15', 'virtualInfos_18', 'virtualInfos_8', 'virtualInfos_12', 'virtualInfos_21', 'virtualInfos_11'])
 info = virtualInfos[virtualId]
REF_5445(IAgentNft.VirtualInfo) -> virtualInfos_10[virtualId_1]
info_1(IAgentNft.VirtualInfo) := REF_5445(IAgentNft.VirtualInfo)
 dao = IAgentDAO(info.dao)
REF_5446(address) -> info_1.dao
TMP_13234 = CONVERT REF_5446 to IAgentDAO
dao_1(IAgentDAO) := TMP_13234(IAgentDAO)
 dao.scoreOf(account)
TMP_13235(uint256) = HIGH_LEVEL_CALL, dest:dao_1(IAgentDAO), function:scoreOf, arguments:['account_1']  
RETURN TMP_13235
```
#### AgentNftV2.addCoreType(string) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_11(bytes32) := phi(['DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0'])
 super._addCoreType(label)
INTERNAL_CALL, CoreRegistry._addCoreType(string)(label_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_11)
```
#### AgentNftV2.addValidator(uint256,address) [PUBLIC]
```slithir
 isValidator(virtualId,validator)
TMP_13231(bool) = INTERNAL_CALL, ValidatorRegistry.isValidator(uint256,address)(virtualId_1,validator_1)
CONDITION TMP_13231
 _addValidator(virtualId,validator)
INTERNAL_CALL, ValidatorRegistry._addValidator(uint256,address)(virtualId_1,validator_1)
 _initValidatorScore(virtualId,validator)
INTERNAL_CALL, ValidatorRegistry._initValidatorScore(uint256,address)(virtualId_1,validator_1)
```
#### AgentNftV2.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### AgentNftV2.getAllServices(uint256) [PUBLIC]
```slithir
virtualInfos_22(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(['virtualInfos_0', 'virtualInfos_26', 'virtualInfos_28', 'virtualInfos_22', 'virtualInfos_9', 'virtualInfos_10', 'virtualInfos_15', 'virtualInfos_18', 'virtualInfos_8', 'virtualInfos_12', 'virtualInfos_21', 'virtualInfos_11'])
_serviceNft_3(address) := phi(['_serviceNft_0', '_serviceNft_1', '_serviceNft_5'])
 info = virtualInfos[virtualId]
REF_5469(IAgentNft.VirtualInfo) -> virtualInfos_22[virtualId_1]
info_1(IAgentNft.VirtualInfo) := REF_5469(IAgentNft.VirtualInfo)
 serviceNft = IERC721Enumerable(_serviceNft)
TMP_13255 = CONVERT _serviceNft_3 to IERC721Enumerable
serviceNft_1(IERC721Enumerable) := TMP_13255(IERC721Enumerable)
 total = serviceNft.balanceOf(info.tba)
REF_5471(address) -> info_1.tba
TMP_13256(uint256) = HIGH_LEVEL_CALL, dest:serviceNft_1(IERC721Enumerable), function:balanceOf, arguments:['REF_5471']  
total_1(uint256) := TMP_13256(uint256)
 services = new uint256[](total)
TMP_13258(uint256[])  = new uint256[](total_1)
services_1(uint256[]) = ['TMP_13258(uint256[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < total
services_2(uint256[]) := phi(['services_3', 'services_1'])
i_2(uint256) := phi(['i_3', 'i_1'])
TMP_13259(bool) = i_2 < total_1
CONDITION TMP_13259
 services[i] = serviceNft.tokenOfOwnerByIndex(info.tba,i)
REF_5472(uint256) -> services_2[i_2]
REF_5474(address) -> info_1.tba
TMP_13260(uint256) = HIGH_LEVEL_CALL, dest:serviceNft_1(IERC721Enumerable), function:tokenOfOwnerByIndex, arguments:['REF_5474', 'i_2']  
services_3(uint256[]) := phi(['services_2'])
REF_5472(uint256) (->services_3) := TMP_13260(uint256)
 i ++
TMP_13261(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 services
RETURN services_2
```
#### AgentNftV2.getContributionNft() [PUBLIC]
```slithir
_contributionNft_2(address) := phi(['_contributionNft_0', '_contributionNft_1'])
 _contributionNft
RETURN _contributionNft_2
```
#### AgentNftV2.getEloCalculator() [PUBLIC]
```slithir
_eloCalculator_2(address) := phi(['_eloCalculator_1', '_eloCalculator_0'])
 _eloCalculator
RETURN _eloCalculator_2
```
#### AgentNftV2.getServiceNft() [PUBLIC]
```slithir
_serviceNft_2(address) := phi(['_serviceNft_0', '_serviceNft_1', '_serviceNft_5'])
 _serviceNft
RETURN _serviceNft_2
```
#### AgentNftV2.getVotes(uint256,address) [PUBLIC]
```slithir
virtualLPs_11(mapping(uint256 => IAgentNft.VirtualLP)) := phi(['virtualLPs_16', 'virtualLPs_8', 'virtualLPs_0', 'virtualLPs_7', 'virtualLPs_10', 'virtualLPs_12'])
 IERC5805(virtualLPs[virtualId].veToken).getVotes(validator)
REF_5466(IAgentNft.VirtualLP) -> virtualLPs_11[virtualId_1]
REF_5467(address) -> REF_5466.veToken
TMP_13253 = CONVERT REF_5467 to IERC5805
TMP_13254(uint256) = HIGH_LEVEL_CALL, dest:TMP_13253(IERC5805), function:getVotes, arguments:['validator_1']  
virtualLPs_12(mapping(uint256 => IAgentNft.VirtualLP)) := phi(['virtualLPs_16', 'virtualLPs_8', 'virtualLPs_11', 'virtualLPs_7', 'virtualLPs_10', 'virtualLPs_12'])
RETURN TMP_13254
```
#### AgentNftV2.initialize(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0'])
VALIDATOR_ADMIN_ROLE_1(bytes32) := phi(['VALIDATOR_ADMIN_ROLE_9', 'VALIDATOR_ADMIN_ROLE_0'])
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_18', 'ADMIN_ROLE_14', 'ADMIN_ROLE_0', 'ADMIN_ROLE_10', 'ADMIN_ROLE_16', 'ADMIN_ROLE_12'])
 __ERC721_init(Agent,AGENT)
INTERNAL_CALL, ERC721Upgradeable.__ERC721_init(string,string)(Agent,AGENT)
 __ERC721URIStorage_init()
INTERNAL_CALL, ERC721URIStorageUpgradeable.__ERC721URIStorage_init()()
 __CoreRegistry_init()
INTERNAL_CALL, CoreRegistry.__CoreRegistry_init()()
 __ValidatorRegistry_init(_validatorScoreOf,totalProposals,_getPastValidatorScore)
INTERNAL_CALL, ValidatorRegistry.__ValidatorRegistry_init(function(uint256,address) returns(uint256),function(uint256) returns(uint256),function(uint256,address,uint256) returns(uint256))(_validatorScoreOf,totalProposals,_getPastValidatorScore)
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 _grantRole(DEFAULT_ADMIN_ROLE,defaultAdmin)
TMP_13212(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_7,defaultAdmin_1)
 _grantRole(VALIDATOR_ADMIN_ROLE,defaultAdmin)
TMP_13213(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(VALIDATOR_ADMIN_ROLE_8,defaultAdmin_1)
 _grantRole(ADMIN_ROLE,defaultAdmin)
TMP_13214(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(ADMIN_ROLE_9,defaultAdmin_1)
 _nextVirtualId = 1
_nextVirtualId_1(uint256) := 1(uint256)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### AgentNftV2.isBlacklisted(uint256) [PUBLIC]
```slithir
_blacklists_1(mapping(uint256 => bool)) := phi(['_blacklists_2', '_blacklists_1', '_blacklists_0'])
 _blacklists[virtualId]
REF_5475(bool) -> _blacklists_1[virtualId_1]
RETURN REF_5475
```
#### AgentNftV2.migrateScoreFunctions() [PUBLIC]
```slithir
ADMIN_ROLE_13(bytes32) := phi(['ADMIN_ROLE_18', 'ADMIN_ROLE_14', 'ADMIN_ROLE_0', 'ADMIN_ROLE_10', 'ADMIN_ROLE_16', 'ADMIN_ROLE_12'])
 _migrateScoreFunctions(_validatorScoreOf,totalProposals,_getPastValidatorScore)
INTERNAL_CALL, ValidatorRegistry._migrateScoreFunctions(function(uint256,address) returns(uint256),function(uint256) returns(uint256),function(uint256,address,uint256) returns(uint256))(_validatorScoreOf,totalProposals,_getPastValidatorScore)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_13)
```
#### AgentNftV2.migrateVirtual(uint256,address,address,address,address) [PUBLIC]
```slithir
virtualInfos_23(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(['virtualInfos_0', 'virtualInfos_26', 'virtualInfos_28', 'virtualInfos_22', 'virtualInfos_9', 'virtualInfos_10', 'virtualInfos_15', 'virtualInfos_18', 'virtualInfos_8', 'virtualInfos_12', 'virtualInfos_21', 'virtualInfos_11'])
ADMIN_ROLE_17(bytes32) := phi(['ADMIN_ROLE_18', 'ADMIN_ROLE_14', 'ADMIN_ROLE_0', 'ADMIN_ROLE_10', 'ADMIN_ROLE_16', 'ADMIN_ROLE_12'])
virtualLPs_13(mapping(uint256 => IAgentNft.VirtualLP)) := phi(['virtualLPs_16', 'virtualLPs_8', 'virtualLPs_0', 'virtualLPs_7', 'virtualLPs_10', 'virtualLPs_12'])
 info = virtualInfos[virtualId]
REF_5477(IAgentNft.VirtualInfo) -> virtualInfos_24[virtualId_1]
info_1 (-> ['virtualInfos'])(IAgentNft.VirtualInfo) := REF_5477(IAgentNft.VirtualInfo)
 info.dao = dao
REF_5478(address) -> info_1 (-> ['virtualInfos']).dao
info_2 (-> ['virtualInfos'])(IAgentNft.VirtualInfo) := phi(["info_1 (-> ['virtualInfos'])"])
REF_5478(address) (->info_2 (-> ['virtualInfos'])) := dao_1(address)
virtualInfos_25(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(["info_2 (-> ['virtualInfos'])"])
 info.token = token
REF_5479(address) -> info_2 (-> ['virtualInfos']).token
info_3 (-> ['virtualInfos'])(IAgentNft.VirtualInfo) := phi(["info_2 (-> ['virtualInfos'])"])
REF_5479(address) (->info_3 (-> ['virtualInfos'])) := token_1(address)
virtualInfos_26(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(["info_3 (-> ['virtualInfos'])"])
 lp = virtualLPs[virtualId]
REF_5480(IAgentNft.VirtualLP) -> virtualLPs_14[virtualId_1]
lp_1 (-> ['virtualLPs'])(IAgentNft.VirtualLP) := REF_5480(IAgentNft.VirtualLP)
 lp.pool = pool
REF_5481(address) -> lp_1 (-> ['virtualLPs']).pool
lp_2 (-> ['virtualLPs'])(IAgentNft.VirtualLP) := phi(["lp_1 (-> ['virtualLPs'])"])
REF_5481(address) (->lp_2 (-> ['virtualLPs'])) := pool_1(address)
virtualLPs_15(mapping(uint256 => IAgentNft.VirtualLP)) := phi(["lp_2 (-> ['virtualLPs'])"])
 lp.veToken = veToken
REF_5482(address) -> lp_2 (-> ['virtualLPs']).veToken
lp_3 (-> ['virtualLPs'])(IAgentNft.VirtualLP) := phi(["lp_2 (-> ['virtualLPs'])"])
REF_5482(address) (->lp_3 (-> ['virtualLPs'])) := veToken_1(address)
virtualLPs_16(mapping(uint256 => IAgentNft.VirtualLP)) := phi(["lp_3 (-> ['virtualLPs'])"])
 _stakingTokenToVirtualId[address(veToken)] = virtualId
TMP_13270 = CONVERT veToken_1 to address
REF_5483(uint256) -> _stakingTokenToVirtualId_2[TMP_13270]
_stakingTokenToVirtualId_3(mapping(address => uint256)) := phi(['_stakingTokenToVirtualId_2'])
REF_5483(uint256) (->_stakingTokenToVirtualId_3) := virtualId_1(uint256)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_17)
```
#### AgentNftV2.mint(uint256,address,string,address,address,uint8[],address,address) [EXTERNAL]
```slithir
_nextVirtualId_3(uint256) := phi(['_nextVirtualId_5', '_nextVirtualId_0', '_nextVirtualId_1'])
MINTER_ROLE_1(bytes32) := phi(['MINTER_ROLE_0', 'MINTER_ROLE_4', 'MINTER_ROLE_2'])
virtualInfos_1(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(['virtualInfos_0', 'virtualInfos_26', 'virtualInfos_28', 'virtualInfos_22', 'virtualInfos_9', 'virtualInfos_10', 'virtualInfos_15', 'virtualInfos_18', 'virtualInfos_8', 'virtualInfos_12', 'virtualInfos_21', 'virtualInfos_11'])
virtualLPs_1(mapping(uint256 => IAgentNft.VirtualLP)) := phi(['virtualLPs_16', 'virtualLPs_8', 'virtualLPs_0', 'virtualLPs_7', 'virtualLPs_10', 'virtualLPs_12'])
 require(bool,string)(virtualId == _nextVirtualId,Invalid virtualId)
TMP_13217(bool) = virtualId_1 == _nextVirtualId_4
TMP_13218(None) = SOLIDITY_CALL require(bool,string)(TMP_13217,Invalid virtualId)
 _nextVirtualId ++
TMP_13219(uint256) := _nextVirtualId_4(uint256)
_nextVirtualId_5(uint256) = _nextVirtualId_4 (c)+ 1
 _mint(to,virtualId)
INTERNAL_CALL, ERC721Upgradeable._mint(address,uint256)(to_1,virtualId_1)
 _setTokenURI(virtualId,newTokenURI)
INTERNAL_CALL, ERC721URIStorageUpgradeable._setTokenURI(uint256,string)(virtualId_1,newTokenURI_1)
 info = virtualInfos[virtualId]
REF_5432(IAgentNft.VirtualInfo) -> virtualInfos_4[virtualId_1]
info_1 (-> ['virtualInfos'])(IAgentNft.VirtualInfo) := REF_5432(IAgentNft.VirtualInfo)
 info.dao = theDAO
REF_5433(address) -> info_1 (-> ['virtualInfos']).dao
info_2 (-> ['virtualInfos'])(IAgentNft.VirtualInfo) := phi(["info_1 (-> ['virtualInfos'])"])
REF_5433(address) (->info_2 (-> ['virtualInfos'])) := theDAO_1(address)
virtualInfos_5(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(["info_2 (-> ['virtualInfos'])"])
 info.coreTypes = coreTypes
REF_5434(uint8[]) -> info_2 (-> ['virtualInfos']).coreTypes
info_3 (-> ['virtualInfos'])(IAgentNft.VirtualInfo) := phi(["info_2 (-> ['virtualInfos'])"])
REF_5434(uint8[]) (->info_3 (-> ['virtualInfos'])) := coreTypes_1(uint8[])
virtualInfos_6(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(["info_3 (-> ['virtualInfos'])"])
 info.founder = founder
REF_5435(address) -> info_3 (-> ['virtualInfos']).founder
info_4 (-> ['virtualInfos'])(IAgentNft.VirtualInfo) := phi(["info_3 (-> ['virtualInfos'])"])
REF_5435(address) (->info_4 (-> ['virtualInfos'])) := founder_1(address)
virtualInfos_7(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(["info_4 (-> ['virtualInfos'])"])
 daoToken = GovernorVotes(theDAO).token()
TMP_13222 = CONVERT theDAO_1 to GovernorVotes
TMP_13223(IERC5805) = HIGH_LEVEL_CALL, dest:TMP_13222(GovernorVotes), function:token, arguments:[]  
virtualLPs_5(mapping(uint256 => IAgentNft.VirtualLP)) := phi(['virtualLPs_16', 'virtualLPs_8', 'virtualLPs_4', 'virtualLPs_7', 'virtualLPs_10', 'virtualLPs_12'])
daoToken_1(IERC5805) := TMP_13223(IERC5805)
 info.token = token
REF_5437(address) -> info_4 (-> ['virtualInfos']).token
info_5 (-> ['virtualInfos'])(IAgentNft.VirtualInfo) := phi(["info_4 (-> ['virtualInfos'])"])
REF_5437(address) (->info_5 (-> ['virtualInfos'])) := token_1(address)
virtualInfos_8(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(["info_5 (-> ['virtualInfos'])"])
 lp = virtualLPs[virtualId]
REF_5438(IAgentNft.VirtualLP) -> virtualLPs_5[virtualId_1]
lp_1 (-> ['virtualLPs'])(IAgentNft.VirtualLP) := REF_5438(IAgentNft.VirtualLP)
 lp.pool = pool
REF_5439(address) -> lp_1 (-> ['virtualLPs']).pool
lp_2 (-> ['virtualLPs'])(IAgentNft.VirtualLP) := phi(["lp_1 (-> ['virtualLPs'])"])
REF_5439(address) (->lp_2 (-> ['virtualLPs'])) := pool_1(address)
virtualLPs_6(mapping(uint256 => IAgentNft.VirtualLP)) := phi(["lp_2 (-> ['virtualLPs'])"])
 lp.veToken = address(daoToken)
REF_5440(address) -> lp_2 (-> ['virtualLPs']).veToken
TMP_13224 = CONVERT daoToken_1 to address
lp_3 (-> ['virtualLPs'])(IAgentNft.VirtualLP) := phi(["lp_2 (-> ['virtualLPs'])"])
REF_5440(address) (->lp_3 (-> ['virtualLPs'])) := TMP_13224(address)
virtualLPs_7(mapping(uint256 => IAgentNft.VirtualLP)) := phi(["lp_3 (-> ['virtualLPs'])"])
 _stakingTokenToVirtualId[address(daoToken)] = virtualId
TMP_13225 = CONVERT daoToken_1 to address
REF_5441(uint256) -> _stakingTokenToVirtualId_0[TMP_13225]
_stakingTokenToVirtualId_1(mapping(address => uint256)) := phi(['_stakingTokenToVirtualId_0'])
REF_5441(uint256) (->_stakingTokenToVirtualId_1) := virtualId_1(uint256)
 _addValidator(virtualId,founder)
INTERNAL_CALL, ValidatorRegistry._addValidator(uint256,address)(virtualId_1,founder_1)
 _initValidatorScore(virtualId,founder)
INTERNAL_CALL, ValidatorRegistry._initValidatorScore(uint256,address)(virtualId_1,founder_1)
 virtualId
RETURN virtualId_1
 onlyRole(MINTER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(MINTER_ROLE_1)
```
#### AgentNftV2.nextVirtualId() [PUBLIC]
```slithir
_nextVirtualId_2(uint256) := phi(['_nextVirtualId_5', '_nextVirtualId_0', '_nextVirtualId_1'])
 _nextVirtualId
RETURN _nextVirtualId_2
```
#### AgentNftV2.setBlacklist(uint256,bool) [PUBLIC]
```slithir
ADMIN_ROLE_11(bytes32) := phi(['ADMIN_ROLE_18', 'ADMIN_ROLE_14', 'ADMIN_ROLE_0', 'ADMIN_ROLE_10', 'ADMIN_ROLE_16', 'ADMIN_ROLE_12'])
 _blacklists[virtualId] = value
REF_5476(bool) -> _blacklists_1[virtualId_1]
_blacklists_2(mapping(uint256 => bool)) := phi(['_blacklists_1'])
REF_5476(bool) (->_blacklists_2) := value_1(bool)
 AgentBlacklisted(virtualId,value)
Emit AgentBlacklisted(virtualId_1,value_1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_11)
```
#### AgentNftV2.setContributionService(address,address) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_9(bytes32) := phi(['DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0'])
 _contributionNft = contributionNft_
_contributionNft_1(address) := contributionNft__1(address)
 _serviceNft = serviceNft_
_serviceNft_1(address) := serviceNft__1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_9)
```
#### AgentNftV2.setCoreTypes(uint256,uint8[]) [EXTERNAL]
```slithir
virtualInfos_13(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(['virtualInfos_0', 'virtualInfos_26', 'virtualInfos_28', 'virtualInfos_22', 'virtualInfos_9', 'virtualInfos_10', 'virtualInfos_15', 'virtualInfos_18', 'virtualInfos_8', 'virtualInfos_12', 'virtualInfos_21', 'virtualInfos_11'])
 info = virtualInfos[virtualId]
REF_5454(IAgentNft.VirtualInfo) -> virtualInfos_14[virtualId_1]
info_1 (-> ['virtualInfos'])(IAgentNft.VirtualInfo) := REF_5454(IAgentNft.VirtualInfo)
 info.coreTypes = coreTypes
REF_5455(uint8[]) -> info_1 (-> ['virtualInfos']).coreTypes
info_2 (-> ['virtualInfos'])(IAgentNft.VirtualInfo) := phi(["info_1 (-> ['virtualInfos'])"])
REF_5455(uint8[]) (->info_2 (-> ['virtualInfos'])) := coreTypes_1(uint8[])
virtualInfos_15(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(["info_2 (-> ['virtualInfos'])"])
 CoresUpdated(virtualId,coreTypes)
Emit CoresUpdated(virtualId_1,coreTypes_1)
 onlyVirtualDAO(virtualId)
MODIFIER_CALL, AgentNftV2.onlyVirtualDAO(uint256)(virtualId_1)
virtualInfos_14(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(['virtualInfos_28'])
```
#### AgentNftV2.setDAO(uint256,address) [PUBLIC]
```slithir
virtualInfos_19(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(['virtualInfos_0', 'virtualInfos_26', 'virtualInfos_28', 'virtualInfos_22', 'virtualInfos_9', 'virtualInfos_10', 'virtualInfos_15', 'virtualInfos_18', 'virtualInfos_8', 'virtualInfos_12', 'virtualInfos_21', 'virtualInfos_11'])
 require(bool,string)(_msgSender() == virtualInfos[virtualId].dao,Caller is not VIRTUAL DAO)
TMP_13248(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
REF_5459(IAgentNft.VirtualInfo) -> virtualInfos_20[virtualId_1]
REF_5460(address) -> REF_5459.dao
TMP_13249(bool) = TMP_13248 == REF_5460
TMP_13250(None) = SOLIDITY_CALL require(bool,string)(TMP_13249,Caller is not VIRTUAL DAO)
 info = virtualInfos[virtualId]
REF_5461(IAgentNft.VirtualInfo) -> virtualInfos_20[virtualId_1]
info_1 (-> ['virtualInfos'])(IAgentNft.VirtualInfo) := REF_5461(IAgentNft.VirtualInfo)
 info.dao = newDAO
REF_5462(address) -> info_1 (-> ['virtualInfos']).dao
info_2 (-> ['virtualInfos'])(IAgentNft.VirtualInfo) := phi(["info_1 (-> ['virtualInfos'])"])
REF_5462(address) (->info_2 (-> ['virtualInfos'])) := newDAO_1(address)
virtualInfos_21(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(["info_2 (-> ['virtualInfos'])"])
```
#### AgentNftV2.setEloCalculator(address) [PUBLIC]
```slithir
ADMIN_ROLE_15(bytes32) := phi(['ADMIN_ROLE_18', 'ADMIN_ROLE_14', 'ADMIN_ROLE_0', 'ADMIN_ROLE_10', 'ADMIN_ROLE_16', 'ADMIN_ROLE_12'])
 _eloCalculator = eloCalculator
_eloCalculator_1(address) := eloCalculator_1(address)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_15)
```
#### AgentNftV2.setTBA(uint256,address) [EXTERNAL]
```slithir
MINTER_ROLE_3(bytes32) := phi(['MINTER_ROLE_0', 'MINTER_ROLE_4', 'MINTER_ROLE_2'])
virtualInfos_16(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(['virtualInfos_0', 'virtualInfos_26', 'virtualInfos_28', 'virtualInfos_22', 'virtualInfos_9', 'virtualInfos_10', 'virtualInfos_15', 'virtualInfos_18', 'virtualInfos_8', 'virtualInfos_12', 'virtualInfos_21', 'virtualInfos_11'])
 info = virtualInfos[virtualId]
REF_5456(IAgentNft.VirtualInfo) -> virtualInfos_17[virtualId_1]
info_1 (-> ['virtualInfos'])(IAgentNft.VirtualInfo) := REF_5456(IAgentNft.VirtualInfo)
 require(bool,string)(info.tba == address(0),TBA already set)
REF_5457(address) -> info_1 (-> ['virtualInfos']).tba
TMP_13244 = CONVERT 0 to address
TMP_13245(bool) = REF_5457 == TMP_13244
TMP_13246(None) = SOLIDITY_CALL require(bool,string)(TMP_13245,TBA already set)
 info.tba = tba
REF_5458(address) -> info_1 (-> ['virtualInfos']).tba
info_2 (-> ['virtualInfos'])(IAgentNft.VirtualInfo) := phi(["info_1 (-> ['virtualInfos'])"])
REF_5458(address) (->info_2 (-> ['virtualInfos'])) := tba_1(address)
virtualInfos_18(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(["info_2 (-> ['virtualInfos'])"])
 onlyRole(MINTER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(MINTER_ROLE_3)
```
#### AgentNftV2.setTokenURI(uint256,string) [PUBLIC]
```slithir
 _setTokenURI(virtualId,newTokenURI)
INTERNAL_CALL, ERC721URIStorageUpgradeable._setTokenURI(uint256,string)(virtualId_1,newTokenURI_1)
RETURN TMP_13242
 onlyVirtualDAO(virtualId)
MODIFIER_CALL, AgentNftV2.onlyVirtualDAO(uint256)(virtualId_1)
```

#### AgentNftV2.stakingTokenToVirtualId(address) [EXTERNAL]
```slithir
_stakingTokenToVirtualId_2(mapping(address => uint256)) := phi(['_stakingTokenToVirtualId_3', '_stakingTokenToVirtualId_0', '_stakingTokenToVirtualId_2', '_stakingTokenToVirtualId_1'])
 _stakingTokenToVirtualId[stakingToken]
REF_5444(uint256) -> _stakingTokenToVirtualId_2[stakingToken_1]
RETURN REF_5444
```
#### AgentNftV2.supportsInterface(bytes4) [PUBLIC]
```slithir
 super.supportsInterface(interfaceId)
TMP_13263(bool) = INTERNAL_CALL, AccessControlUpgradeable.supportsInterface(bytes4)(interfaceId_1)
RETURN TMP_13263
```
#### AgentNftV2.tokenURI(uint256) [PUBLIC]
```slithir
 super.tokenURI(tokenId)
TMP_13262(string) = INTERNAL_CALL, ERC721URIStorageUpgradeable.tokenURI(uint256)(tokenId_1)
RETURN TMP_13262
```
#### AgentNftV2.totalProposals(uint256) [PUBLIC]
```slithir
virtualInfos_12(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(['virtualInfos_0', 'virtualInfos_26', 'virtualInfos_28', 'virtualInfos_22', 'virtualInfos_9', 'virtualInfos_10', 'virtualInfos_15', 'virtualInfos_18', 'virtualInfos_8', 'virtualInfos_12', 'virtualInfos_21', 'virtualInfos_11'])
 info = virtualInfos[virtualId]
REF_5451(IAgentNft.VirtualInfo) -> virtualInfos_12[virtualId_1]
info_1(IAgentNft.VirtualInfo) := REF_5451(IAgentNft.VirtualInfo)
 dao = IAgentDAO(info.dao)
REF_5452(address) -> info_1.dao
TMP_13238 = CONVERT REF_5452 to IAgentDAO
dao_1(IAgentDAO) := TMP_13238(IAgentDAO)
 dao.proposalCount()
TMP_13239(uint256) = HIGH_LEVEL_CALL, dest:dao_1(IAgentDAO), function:proposalCount, arguments:[]  
RETURN TMP_13239
```
#### AgentNftV2.totalStaked(uint256) [PUBLIC]
```slithir
virtualLPs_9(mapping(uint256 => IAgentNft.VirtualLP)) := phi(['virtualLPs_16', 'virtualLPs_8', 'virtualLPs_0', 'virtualLPs_7', 'virtualLPs_10', 'virtualLPs_12'])
 IERC20(virtualLPs[virtualId].veToken).totalSupply()
REF_5463(IAgentNft.VirtualLP) -> virtualLPs_9[virtualId_1]
REF_5464(address) -> REF_5463.veToken
TMP_13251 = CONVERT REF_5464 to IERC20
TMP_13252(uint256) = HIGH_LEVEL_CALL, dest:TMP_13251(IERC20), function:totalSupply, arguments:[]  
virtualLPs_10(mapping(uint256 => IAgentNft.VirtualLP)) := phi(['virtualLPs_16', 'virtualLPs_9', 'virtualLPs_8', 'virtualLPs_7', 'virtualLPs_10', 'virtualLPs_12'])
RETURN TMP_13252
```
#### AgentNftV2.totalSupply() [PUBLIC]
```slithir
_nextVirtualId_6(uint256) := phi(['_nextVirtualId_5', '_nextVirtualId_0', '_nextVirtualId_1'])
 _nextVirtualId - 1
TMP_13264(uint256) = _nextVirtualId_6 (c)- 1
RETURN TMP_13264
```
#### AgentNftV2.virtualInfo(uint256) [PUBLIC]
```slithir
virtualInfos_9(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(['virtualInfos_0', 'virtualInfos_26', 'virtualInfos_28', 'virtualInfos_22', 'virtualInfos_9', 'virtualInfos_10', 'virtualInfos_15', 'virtualInfos_18', 'virtualInfos_8', 'virtualInfos_12', 'virtualInfos_21', 'virtualInfos_11'])
 virtualInfos[virtualId]
REF_5442(IAgentNft.VirtualInfo) -> virtualInfos_9[virtualId_1]
RETURN REF_5442
```
#### AgentNftV2.virtualLP(uint256) [PUBLIC]
```slithir
virtualLPs_8(mapping(uint256 => IAgentNft.VirtualLP)) := phi(['virtualLPs_16', 'virtualLPs_8', 'virtualLPs_0', 'virtualLPs_7', 'virtualLPs_10', 'virtualLPs_12'])
 virtualLPs[virtualId]
REF_5443(IAgentNft.VirtualLP) -> virtualLPs_8[virtualId_1]
RETURN REF_5443
```
#### IAgentDAO.getPastScore(address,uint256) [EXTERNAL]
```slithir

```
#### IAgentDAO.scoreOf(address) [EXTERNAL]
```slithir

```
#### IERC721Enumerable.tokenOfOwnerByIndex(address,uint256) [EXTERNAL]
```slithir

```
#### GovernorVotes.token() [PUBLIC]
```slithir
_token_2(IERC5805) := phi(['_token_1', '_token_0'])
 _token
RETURN _token_2
```
#### IAgentDAO.proposalCount() [EXTERNAL]
```slithir

```
#### IERC20.totalSupply() [EXTERNAL]
```slithir

```
