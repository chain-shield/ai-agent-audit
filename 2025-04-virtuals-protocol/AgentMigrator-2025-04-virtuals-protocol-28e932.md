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



### Storage layout (AgentMigrator) 

```text
_nft AgentNftV2
_tokenSupplyParams bytes
_tokenTaxParams bytes
_tokenAdmin address
_assetToken address
_uniswapRouter address
initialAmount uint256
tokenImplementation address
daoImplementation address
veTokenImplementation address
maturityDuration uint256
migratedAgents mapping(uint256 => bool)
locked bool

```



#### AgentMigrator._createNewAgentToken(string,string) [INTERNAL]
```slithir
name_1(string) := phi(['name_1'])
symbol_1(string) := phi(['symbol_1'])
_tokenSupplyParams_2(bytes) := phi(['_tokenSupplyParams_0', '_tokenSupplyParams_1', '_tokenSupplyParams_3'])
_tokenTaxParams_2(bytes) := phi(['_tokenTaxParams_1', '_tokenTaxParams_0', '_tokenTaxParams_3'])
_tokenAdmin_2(address) := phi(['_tokenAdmin_3', '_tokenAdmin_0', '_tokenAdmin_1'])
_assetToken_9(address) := phi(['_assetToken_0', '_assetToken_1', '_assetToken_10', '_assetToken_8'])
_uniswapRouter_2(address) := phi(['_uniswapRouter_3', '_uniswapRouter_1', '_uniswapRouter_0'])
tokenImplementation_2(address) := phi(['tokenImplementation_0', 'tokenImplementation_1'])
 instance = Clones.clone(tokenImplementation)
TMP_12950(address) = LIBRARY_CALL, dest:Clones, function:Clones.clone(address), arguments:['tokenImplementation_2'] 
instance_1(address) := TMP_12950(address)
 IAgentToken(instance).initialize((_tokenAdmin,_uniswapRouter,_assetToken),abi.encode(name,symbol),_tokenSupplyParams,_tokenTaxParams)
TMP_12951 = CONVERT instance_1 to IAgentToken
TMP_12952(bytes) = SOLIDITY_CALL abi.encode()(name_1,symbol_1)
HIGH_LEVEL_CALL, dest:TMP_12951(IAgentToken), function:initialize, arguments:['[<slither.slithir.variables.state_variable.StateIRVariable object at 0xffff7490cd30>, <slither.slithir.variables.state_variable.StateIRVariable object at 0xffff7490cd90>, <slither.slithir.variables.state_variable.StateIRVariable object at 0xffff7490cd00>]', 'TMP_12952', '_tokenSupplyParams_2', '_tokenTaxParams_2']  
_tokenSupplyParams_3(bytes) := phi(['_tokenSupplyParams_2', '_tokenSupplyParams_1', '_tokenSupplyParams_3'])
_tokenTaxParams_3(bytes) := phi(['_tokenTaxParams_1', '_tokenTaxParams_2', '_tokenTaxParams_3'])
_tokenAdmin_3(address) := phi(['_tokenAdmin_2', '_tokenAdmin_3', '_tokenAdmin_1'])
_assetToken_10(address) := phi(['_assetToken_1', '_assetToken_10', '_assetToken_9', '_assetToken_8'])
_uniswapRouter_3(address) := phi(['_uniswapRouter_1', '_uniswapRouter_3', '_uniswapRouter_2'])
 instance
RETURN instance_1
 instance
```
#### AgentMigrator._createNewAgentVeToken(string,string,address,address,bool) [INTERNAL]
```slithir
name_1(string) := phi(['TMP_12918'])
symbol_1(string) := phi(['TMP_12919'])
stakingAsset_1(address) := phi(['lp_1'])
founder_1(address) := phi(['founder_1'])
canStake_1(bool) := phi(['canStake_1'])
_nft_18(AgentNftV2) := phi(['_nft_17', '_nft_15', '_nft_19', '_nft_1', '_nft_0'])
veTokenImplementation_2(address) := phi(['veTokenImplementation_1', 'veTokenImplementation_0'])
maturityDuration_2(uint256) := phi(['maturityDuration_0', 'maturityDuration_1', 'maturityDuration_3'])
 instance = Clones.clone(veTokenImplementation)
TMP_12945(address) = LIBRARY_CALL, dest:Clones, function:Clones.clone(address), arguments:['veTokenImplementation_2'] 
instance_1(address) := TMP_12945(address)
 IAgentVeToken(instance).initialize(name,symbol,founder,stakingAsset,block.timestamp + maturityDuration,address(_nft),canStake)
TMP_12946 = CONVERT instance_1 to IAgentVeToken
TMP_12947(uint256) = block.timestamp (c)+ maturityDuration_2
TMP_12948 = CONVERT _nft_18 to address
HIGH_LEVEL_CALL, dest:TMP_12946(IAgentVeToken), function:initialize, arguments:['name_1', 'symbol_1', 'founder_1', 'stakingAsset_1', 'TMP_12947', 'TMP_12948', 'canStake_1']  
_nft_19(AgentNftV2) := phi(['_nft_18', '_nft_17', '_nft_15', '_nft_19', '_nft_1'])
maturityDuration_3(uint256) := phi(['maturityDuration_1', 'maturityDuration_3', 'maturityDuration_2'])
 instance
RETURN instance_1
 instance
```
#### AgentMigrator._createNewDAO(string,IVotes,uint32,uint256) [INTERNAL]
```slithir
name_1(string) := phi(['TMP_12922'])
token_1(IVotes) := phi(['TMP_12923'])
daoVotingPeriod_1(uint32) := phi(['TMP_12925'])
daoThreshold_1(uint256) := phi(['TMP_12926'])
_nft_16(AgentNftV2) := phi(['_nft_17', '_nft_15', '_nft_19', '_nft_1', '_nft_0'])
daoImplementation_2(address) := phi(['daoImplementation_0', 'daoImplementation_1'])
 instance = Clones.clone(daoImplementation)
TMP_12941(address) = LIBRARY_CALL, dest:Clones, function:Clones.clone(address), arguments:['daoImplementation_2'] 
instance_1(address) := TMP_12941(address)
 IAgentDAO(instance).initialize(name,token,address(_nft),daoThreshold,daoVotingPeriod)
TMP_12942 = CONVERT instance_1 to IAgentDAO
TMP_12943 = CONVERT _nft_16 to address
HIGH_LEVEL_CALL, dest:TMP_12942(IAgentDAO), function:initialize, arguments:['name_1', 'token_1', 'TMP_12943', 'daoThreshold_1', 'daoVotingPeriod_1']  
_nft_17(AgentNftV2) := phi(['_nft_17', '_nft_15', '_nft_16', '_nft_19', '_nft_1'])
 instance
RETURN instance_1
 instance
```
#### AgentMigrator.constructor(address) [PUBLIC]
```slithir
 _nft = AgentNftV2(agentNft_)
TMP_12893 = CONVERT agentNft__1 to AgentNftV2
_nft_1(AgentNftV2) := TMP_12893(AgentNftV2)
 Ownable(_msgSender())
TMP_12894(address) = INTERNAL_CALL, Context._msgSender()()
INTERNAL_CALL, Ownable.constructor(address)(TMP_12894)
 Ownable(_msgSender())
TMP_12896(address) = INTERNAL_CALL, Context._msgSender()()
INTERNAL_CALL, Ownable.constructor(address)(TMP_12896)
```
#### AgentMigrator.migrateAgent(uint256,string,string,bool) [EXTERNAL]
```slithir
_nft_2(AgentNftV2) := phi(['_nft_17', '_nft_15', '_nft_19', '_nft_1', '_nft_0'])
_assetToken_2(address) := phi(['_assetToken_0', '_assetToken_1', '_assetToken_10', '_assetToken_8'])
initialAmount_2(uint256) := phi(['initialAmount_1', 'initialAmount_8', 'initialAmount_0'])
migratedAgents_1(mapping(uint256 => bool)) := phi(['migratedAgents_4', 'migratedAgents_0', 'migratedAgents_3'])
 require(bool,string)(! migratedAgents[id],Agent already migrated)
REF_5315(bool) -> migratedAgents_2[id_1]
TMP_12904 = UnaryType.BANG REF_5315 
TMP_12905(None) = SOLIDITY_CALL require(bool,string)(TMP_12904,Agent already migrated)
 virtualInfo = _nft.virtualInfo(id)
TMP_12906(IAgentNft.VirtualInfo) = HIGH_LEVEL_CALL, dest:_nft_3(AgentNftV2), function:virtualInfo, arguments:['id_1']  
_nft_4(AgentNftV2) := phi(['_nft_17', '_nft_15', '_nft_3', '_nft_19', '_nft_1'])
_assetToken_4(address) := phi(['_assetToken_1', '_assetToken_3', '_assetToken_10', '_assetToken_8'])
initialAmount_4(uint256) := phi(['initialAmount_3', 'initialAmount_1', 'initialAmount_8'])
virtualInfo_1(IAgentNft.VirtualInfo) := TMP_12906(IAgentNft.VirtualInfo)
 founder = virtualInfo.founder
REF_5317(address) -> virtualInfo_1.founder
founder_1(address) := REF_5317(address)
 require(bool,string)(founder == _msgSender(),Not founder)
TMP_12907(address) = INTERNAL_CALL, Context._msgSender()()
TMP_12908(bool) = founder_1 == TMP_12907
TMP_12909(None) = SOLIDITY_CALL require(bool,string)(TMP_12908,Not founder)
 token = _createNewAgentToken(name,symbol)
TMP_12910(address) = INTERNAL_CALL, AgentMigrator._createNewAgentToken(string,string)(name_1,symbol_1)
_assetToken_6(address) := phi(['_assetToken_10'])
token_1(address) := TMP_12910(address)
 lp = IAgentToken(token).liquidityPools()[0]
TMP_12911 = CONVERT token_1 to IAgentToken
TMP_12912(address[]) = HIGH_LEVEL_CALL, dest:TMP_12911(IAgentToken), function:liquidityPools, arguments:[]  
_nft_7(AgentNftV2) := phi(['_nft_6', '_nft_17', '_nft_15', '_nft_19', '_nft_1'])
_assetToken_7(address) := phi(['_assetToken_1', '_assetToken_6', '_assetToken_10', '_assetToken_8'])
initialAmount_7(uint256) := phi(['initialAmount_1', 'initialAmount_8', 'initialAmount_6'])
REF_5319(address) -> TMP_12912[0]
lp_1(address) := REF_5319(address)
 IERC20(_assetToken).transferFrom(founder,token,initialAmount)
TMP_12913 = CONVERT _assetToken_7 to IERC20
TMP_12914(bool) = HIGH_LEVEL_CALL, dest:TMP_12913(IERC20), function:transferFrom, arguments:['founder_1', 'token_1', 'initialAmount_7']  
_nft_8(AgentNftV2) := phi(['_nft_17', '_nft_7', '_nft_15', '_nft_19', '_nft_1'])
_assetToken_8(address) := phi(['_assetToken_1', '_assetToken_7', '_assetToken_10', '_assetToken_8'])
initialAmount_8(uint256) := phi(['initialAmount_7', 'initialAmount_1', 'initialAmount_8'])
 IAgentToken(token).addInitialLiquidity(address(this))
TMP_12915 = CONVERT token_1 to IAgentToken
TMP_12916 = CONVERT this to address
HIGH_LEVEL_CALL, dest:TMP_12915(IAgentToken), function:addInitialLiquidity, arguments:['TMP_12916']  
_nft_9(AgentNftV2) := phi(['_nft_17', '_nft_15', '_nft_19', '_nft_8', '_nft_1'])
 veToken = _createNewAgentVeToken(string.concat(Staked ,name),string.concat(s,symbol),lp,founder,canStake)
TMP_12918(string) = SOLIDITY_CALL string.concat()(Staked ,name_1)
TMP_12919(string) = SOLIDITY_CALL string.concat()(s,symbol_1)
TMP_12920(address) = INTERNAL_CALL, AgentMigrator._createNewAgentVeToken(string,string,address,address,bool)(TMP_12918,TMP_12919,lp_1,founder_1,canStake_1)
_nft_10(AgentNftV2) := phi(['_nft_19'])
veToken_1(address) := TMP_12920(address)
 oldDAO = IGovernor(virtualInfo.dao)
REF_5324(address) -> virtualInfo_1.dao
TMP_12921 = CONVERT REF_5324 to IGovernor
oldDAO_1(IGovernor) := TMP_12921(IGovernor)
 dao = address(_createNewDAO(oldDAO.name(),IVotes(veToken),uint32(oldDAO.votingPeriod()),oldDAO.proposalThreshold()))
TMP_12922(string) = HIGH_LEVEL_CALL, dest:oldDAO_1(IGovernor), function:name, arguments:[]  
_nft_11(AgentNftV2) := phi(['_nft_17', '_nft_15', '_nft_10', '_nft_19', '_nft_1'])
TMP_12923 = CONVERT veToken_1 to IVotes
TMP_12924(uint256) = HIGH_LEVEL_CALL, dest:oldDAO_1(IGovernor), function:votingPeriod, arguments:[]  
_nft_12(AgentNftV2) := phi(['_nft_11', '_nft_17', '_nft_15', '_nft_19', '_nft_1'])
TMP_12925 = CONVERT TMP_12924 to uint32
TMP_12926(uint256) = HIGH_LEVEL_CALL, dest:oldDAO_1(IGovernor), function:proposalThreshold, arguments:[]  
_nft_13(AgentNftV2) := phi(['_nft_17', '_nft_15', '_nft_12', '_nft_19', '_nft_1'])
TMP_12927(address) = INTERNAL_CALL, AgentMigrator._createNewDAO(string,IVotes,uint32,uint256)(TMP_12922,TMP_12923,TMP_12925,TMP_12926)
_nft_14(AgentNftV2) := phi(['_nft_17'])
TMP_12928 = CONVERT TMP_12927 to address
dao_1(address) := TMP_12928(address)
 _nft.migrateVirtual(id,dao,token,lp,veToken)
HIGH_LEVEL_CALL, dest:_nft_14(AgentNftV2), function:migrateVirtual, arguments:['id_1', 'dao_1', 'token_1', 'lp_1', 'veToken_1']  
_nft_15(AgentNftV2) := phi(['_nft_17', '_nft_15', '_nft_14', '_nft_19', '_nft_1'])
 IERC20(lp).approve(veToken,type()(uint256).max)
TMP_12930 = CONVERT lp_1 to IERC20
TMP_12932(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_12933(bool) = HIGH_LEVEL_CALL, dest:TMP_12930(IERC20), function:approve, arguments:['veToken_1', 'TMP_12932']  
 IAgentVeToken(veToken).stake(IERC20(lp).balanceOf(address(this)),founder,founder)
TMP_12934 = CONVERT veToken_1 to IAgentVeToken
TMP_12935 = CONVERT lp_1 to IERC20
TMP_12936 = CONVERT this to address
TMP_12937(uint256) = HIGH_LEVEL_CALL, dest:TMP_12935(IERC20), function:balanceOf, arguments:['TMP_12936']  
HIGH_LEVEL_CALL, dest:TMP_12934(IAgentVeToken), function:stake, arguments:['TMP_12937', 'founder_1', 'founder_1']  
 migratedAgents[id] = true
REF_5332(bool) -> migratedAgents_2[id_1]
migratedAgents_3(mapping(uint256 => bool)) := phi(['migratedAgents_2'])
REF_5332(bool) (->migratedAgents_3) := True(bool)
 AgentMigrated(id,dao,token,lp,veToken)
Emit AgentMigrated(id_1,dao_1,token_1,lp_1,veToken_1)
 noReentrant()
MODIFIER_CALL, AgentMigrator.noReentrant()()
```
#### AgentMigrator.pause() [EXTERNAL][OWNER]
```slithir
 super._pause()
INTERNAL_CALL, Pausable._pause()()
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```

#### AgentMigrator.setImplementations(address,address,address) [EXTERNAL][OWNER]
```slithir
 tokenImplementation = token
tokenImplementation_1(address) := token_1(address)
 daoImplementation = dao
daoImplementation_1(address) := dao_1(address)
 veTokenImplementation = veToken
veTokenImplementation_1(address) := veToken_1(address)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### AgentMigrator.setInitParams(address,address,address,uint256,uint256) [EXTERNAL][OWNER]
```slithir
 _tokenAdmin = tokenAdmin_
_tokenAdmin_1(address) := tokenAdmin__1(address)
 _assetToken = assetToken_
_assetToken_1(address) := assetToken__1(address)
 _uniswapRouter = uniswapRouter_
_uniswapRouter_1(address) := uniswapRouter__1(address)
 initialAmount = initialAmount_
initialAmount_1(uint256) := initialAmount__1(uint256)
 maturityDuration = maturityDuration_
maturityDuration_1(uint256) := maturityDuration__1(uint256)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### AgentMigrator.setTokenSupplyParams(uint256,uint256,uint256,uint256,uint256,uint256,address) [PUBLIC][OWNER]
```slithir
 _tokenSupplyParams = abi.encode(maxSupply,lpSupply,vaultSupply,maxTokensPerWallet,maxTokensPerTxn,botProtectionDurationInSeconds,vault)
TMP_12899(bytes) = SOLIDITY_CALL abi.encode()(maxSupply_1,lpSupply_1,vaultSupply_1,maxTokensPerWallet_1,maxTokensPerTxn_1,botProtectionDurationInSeconds_1,vault_1)
_tokenSupplyParams_1(bytes) := TMP_12899(bytes)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### AgentMigrator.setTokenTaxParams(uint256,uint256,uint256,address) [PUBLIC][OWNER]
```slithir
 _tokenTaxParams = abi.encode(projectBuyTaxBasisPoints,projectSellTaxBasisPoints,taxSwapThresholdBasisPoints,projectTaxRecipient)
TMP_12901(bytes) = SOLIDITY_CALL abi.encode()(projectBuyTaxBasisPoints_1,projectSellTaxBasisPoints_1,taxSwapThresholdBasisPoints_1,projectTaxRecipient_1)
_tokenTaxParams_1(bytes) := TMP_12901(bytes)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### AgentMigrator.unpause() [EXTERNAL][OWNER]
```slithir
 super._unpause()
INTERNAL_CALL, Pausable._unpause()()
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### Clones.clone(address) [INTERNAL]
```slithir
 mstore(uint256,uint256)(0x00,implementation << 0x60 >> 0xe8 | 0x3d602d80600a3d3981f3363d3d373d3d3d363d73000000)
TMP_4878(address) = implementation_1 << 96
TMP_4879(address) = TMP_4878 >> 232
TMP_4880(address) = TMP_4879 | 5878623614256507594275124645717695763639139257454428160
TMP_4881(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,TMP_4880)
 mstore(uint256,uint256)(0x20,implementation << 0x78 | 0x5af43d82803e903d91602b57fd5bf3)
TMP_4882(address) = implementation_1 << 120
TMP_4883(address) = TMP_4882 | 472260498517428093450829853099908083
TMP_4884(None) = SOLIDITY_CALL mstore(uint256,uint256)(32,TMP_4883)
 instance = create(uint256,uint256,uint256)(0,0x09,0x37)
TMP_4885(uint256) = SOLIDITY_CALL create(uint256,uint256,uint256)(0,9,55)
instance_1(address) := TMP_4885(uint256)
 instance == address(0)
TMP_4886 = CONVERT 0 to address
TMP_4887(bool) = instance_1 == TMP_4886
CONDITION TMP_4887
 revert ERC1167FailedCreateClone()()
TMP_4888(None) = SOLIDITY_CALL revert ERC1167FailedCreateClone()()
 instance
RETURN instance_1
```

#### IAgentVeToken.initialize(string,string,address,address,uint256,address,bool) [EXTERNAL]
```slithir

```
#### IAgentDAO.initialize(string,IVotes,address,uint256,uint32) [EXTERNAL]
```slithir

```
#### IGovernor.name() [EXTERNAL]
```slithir

```
#### IGovernor.proposalThreshold() [EXTERNAL]
```slithir

```
#### IGovernor.votingPeriod() [EXTERNAL]
```slithir

```
#### IERC20.approve(address,uint256) [EXTERNAL]
```slithir

```
#### IERC20.balanceOf(address) [EXTERNAL]
```slithir

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
#### AgentNftV2.virtualInfo(uint256) [PUBLIC]
```slithir
virtualInfos_9(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(['virtualInfos_0', 'virtualInfos_26', 'virtualInfos_28', 'virtualInfos_22', 'virtualInfos_9', 'virtualInfos_10', 'virtualInfos_15', 'virtualInfos_18', 'virtualInfos_8', 'virtualInfos_12', 'virtualInfos_21', 'virtualInfos_11'])
 virtualInfos[virtualId]
REF_5442(IAgentNft.VirtualInfo) -> virtualInfos_9[virtualId_1]
RETURN REF_5442
```
#### IAgentToken.addInitialLiquidity(address) [EXTERNAL]
```slithir

```
#### IAgentToken.liquidityPools() [EXTERNAL]
```slithir

```
#### IAgentVeToken.stake(uint256,address,address) [EXTERNAL]
```slithir

```
