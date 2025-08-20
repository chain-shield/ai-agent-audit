

### Storage layout (AgentFactoryV2) 

```text
_nextId uint256
tokenImplementation address
daoImplementation address
nft address
tbaRegistry address
applicationThreshold uint256
allTokens address[]
allDAOs address[]
assetToken address
maturityDuration uint256
_applications mapping(uint256 => AgentFactoryV2.Application)
gov address
_vault address
locked bool
allTradingTokens address[]
_uniswapRouter address
veTokenImplementation address
_minter address
_tokenAdmin address
defaultDelegatee address
_tokenSupplyParams bytes
_tokenTaxParams bytes
_tokenMultiplier uint16

```







#### AgentFactoryV2._createNewAgentToken(string,string) [INTERNAL]
```slithir
name_1(string) := phi(['REF_4981'])
symbol_1(string) := phi(['REF_4982'])
tokenImplementation_2(address) := phi(['tokenImplementation_3', 'tokenImplementation_0', 'tokenImplementation_1'])
assetToken_16(address) := phi(['assetToken_6', 'assetToken_0', 'assetToken_17', 'assetToken_9', 'assetToken_15', 'assetToken_18', 'assetToken_1'])
allTradingTokens_1(address[]) := phi(['allTradingTokens_4', 'allTradingTokens_0'])
_uniswapRouter_1(address) := phi(['_uniswapRouter_0', '_uniswapRouter_3', '_uniswapRouter_2'])
_tokenAdmin_3(address) := phi(['_tokenAdmin_2', '_tokenAdmin_0', '_tokenAdmin_4', '_tokenAdmin_5'])
_tokenSupplyParams_1(bytes) := phi(['_tokenSupplyParams_3', '_tokenSupplyParams_2', '_tokenSupplyParams_0'])
_tokenTaxParams_1(bytes) := phi(['_tokenTaxParams_3', '_tokenTaxParams_0', '_tokenTaxParams_2'])
 instance = Clones.clone(tokenImplementation)
TMP_12262(address) = LIBRARY_CALL, dest:Clones, function:Clones.clone(address), arguments:['tokenImplementation_2'] 
instance_1(address) := TMP_12262(address)
 IAgentToken(instance).initialize((_tokenAdmin,_uniswapRouter,assetToken),abi.encode(name,symbol),_tokenSupplyParams,_tokenTaxParams)
TMP_12263 = CONVERT instance_1 to IAgentToken
TMP_12264(bytes) = SOLIDITY_CALL abi.encode()(name_1,symbol_1)
HIGH_LEVEL_CALL, dest:TMP_12263(IAgentToken), function:initialize, arguments:['[<slither.slithir.variables.state_variable.StateIRVariable object at 0xffff74dffa30>, <slither.slithir.variables.state_variable.StateIRVariable object at 0xffff74dff9d0>, <slither.slithir.variables.state_variable.StateIRVariable object at 0xffff74dfda50>]', 'TMP_12264', '_tokenSupplyParams_1', '_tokenTaxParams_1']  
assetToken_17(address) := phi(['assetToken_16', 'assetToken_6', 'assetToken_17', 'assetToken_9', 'assetToken_15', 'assetToken_18', 'assetToken_1'])
allTradingTokens_2(address[]) := phi(['allTradingTokens_1', 'allTradingTokens_4'])
_uniswapRouter_2(address) := phi(['_uniswapRouter_3', '_uniswapRouter_2', '_uniswapRouter_1'])
_tokenAdmin_4(address) := phi(['_tokenAdmin_2', '_tokenAdmin_3', '_tokenAdmin_4', '_tokenAdmin_5'])
_tokenSupplyParams_2(bytes) := phi(['_tokenSupplyParams_3', '_tokenSupplyParams_2', '_tokenSupplyParams_1'])
_tokenTaxParams_2(bytes) := phi(['_tokenTaxParams_3', '_tokenTaxParams_1', '_tokenTaxParams_2'])
 allTradingTokens.push(instance)
REF_5020 -> LENGTH allTradingTokens_2
TMP_12267(uint256) := REF_5020(uint256)
TMP_12268(uint256) = TMP_12267 (c)+ 1
allTradingTokens_3(address[]) := phi(['allTradingTokens_2'])
REF_5020(uint256) (->allTradingTokens_3) := TMP_12268(uint256)
REF_5021(address) -> allTradingTokens_3[TMP_12267]
allTradingTokens_4(address[]) := phi(['allTradingTokens_3'])
REF_5021(address) (->allTradingTokens_4) := instance_1(address)
 instance
RETURN instance_1
 instance
```
#### AgentFactoryV2._createNewAgentVeToken(string,string,address,address,bool) [INTERNAL]
```slithir
name_1(string) := phi(['TMP_12229'])
symbol_1(string) := phi(['TMP_12230'])
stakingAsset_1(address) := phi(['lp_1'])
founder_1(address) := phi(['REF_4991'])
canStake_1(bool) := phi(['canStake_1'])
nft_17(address) := phi(['nft_0', 'nft_18', 'nft_16', 'nft_14', 'nft_1'])
allTokens_1(address[]) := phi(['allTokens_4', 'allTokens_0', 'allTokens_5'])
maturityDuration_1(uint256) := phi(['maturityDuration_2', 'maturityDuration_0', 'maturityDuration_3'])
veTokenImplementation_2(address) := phi(['veTokenImplementation_0', 'veTokenImplementation_1', 'veTokenImplementation_3'])
 instance = Clones.clone(veTokenImplementation)
TMP_12269(address) = LIBRARY_CALL, dest:Clones, function:Clones.clone(address), arguments:['veTokenImplementation_2'] 
instance_1(address) := TMP_12269(address)
 IAgentVeToken(instance).initialize(name,symbol,founder,stakingAsset,block.timestamp + maturityDuration,address(nft),canStake)
TMP_12270 = CONVERT instance_1 to IAgentVeToken
TMP_12271(uint256) = block.timestamp (c)+ maturityDuration_1
TMP_12272 = CONVERT nft_17 to address
HIGH_LEVEL_CALL, dest:TMP_12270(IAgentVeToken), function:initialize, arguments:['name_1', 'symbol_1', 'founder_1', 'stakingAsset_1', 'TMP_12271', 'TMP_12272', 'canStake_1']  
nft_18(address) := phi(['nft_18', 'nft_17', 'nft_16', 'nft_14', 'nft_1'])
allTokens_2(address[]) := phi(['allTokens_4', 'allTokens_1', 'allTokens_5'])
maturityDuration_2(uint256) := phi(['maturityDuration_2', 'maturityDuration_3', 'maturityDuration_1'])
 allTokens.push(instance)
REF_5025 -> LENGTH allTokens_2
TMP_12275(uint256) := REF_5025(uint256)
TMP_12276(uint256) = TMP_12275 (c)+ 1
allTokens_3(address[]) := phi(['allTokens_2'])
REF_5025(uint256) (->allTokens_3) := TMP_12276(uint256)
REF_5026(address) -> allTokens_3[TMP_12275]
allTokens_4(address[]) := phi(['allTokens_3'])
REF_5026(address) (->allTokens_4) := instance_1(address)
 instance
RETURN instance_1
 instance
```
#### AgentFactoryV2._createNewDAO(string,IVotes,uint32,uint256) [INTERNAL]
```slithir
name_1(string) := phi(['daoName_1'])
token_1(IVotes) := phi(['TMP_12233'])
daoVotingPeriod_1(uint32) := phi(['REF_4994'])
daoThreshold_1(uint256) := phi(['REF_4995'])
daoImplementation_2(address) := phi(['daoImplementation_3', 'daoImplementation_1', 'daoImplementation_0'])
nft_15(address) := phi(['nft_0', 'nft_18', 'nft_16', 'nft_14', 'nft_1'])
allDAOs_1(address[]) := phi(['allDAOs_4', 'allDAOs_0'])
 instance = Clones.clone(daoImplementation)
TMP_12256(address) = LIBRARY_CALL, dest:Clones, function:Clones.clone(address), arguments:['daoImplementation_2'] 
instance_1(address) := TMP_12256(address)
 IAgentDAO(instance).initialize(name,token,nft,daoThreshold,daoVotingPeriod)
TMP_12257 = CONVERT instance_1 to IAgentDAO
HIGH_LEVEL_CALL, dest:TMP_12257(IAgentDAO), function:initialize, arguments:['name_1', 'token_1', 'nft_15', 'daoThreshold_1', 'daoVotingPeriod_1']  
nft_16(address) := phi(['nft_18', 'nft_16', 'nft_14', 'nft_1', 'nft_15'])
allDAOs_2(address[]) := phi(['allDAOs_4', 'allDAOs_1'])
 allDAOs.push(instance)
REF_5014 -> LENGTH allDAOs_2
TMP_12260(uint256) := REF_5014(uint256)
TMP_12261(uint256) = TMP_12260 (c)+ 1
allDAOs_3(address[]) := phi(['allDAOs_2'])
REF_5014(uint256) (->allDAOs_3) := TMP_12261(uint256)
REF_5015(address) -> allDAOs_3[TMP_12260]
allDAOs_4(address[]) := phi(['allDAOs_3'])
REF_5015(address) (->allDAOs_4) := instance_1(address)
 instance
RETURN instance_1
 instance
```
#### AgentFactoryV2._msgData() [INTERNAL]
```slithir
 ContextUpgradeable._msgData()
TMP_12294(bytes) = INTERNAL_CALL, ContextUpgradeable._msgData()()
RETURN TMP_12294
```
#### AgentFactoryV2._msgSender() [INTERNAL]
```slithir
 sender = ContextUpgradeable._msgSender()
TMP_12293(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
sender_1(address) := TMP_12293(address)
 sender
RETURN sender_1
```
#### AgentFactoryV2.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### AgentFactoryV2.executeApplication(uint256,bool) [PUBLIC]
```slithir
nft_2(address) := phi(['nft_0', 'nft_18', 'nft_16', 'nft_14', 'nft_1'])
tbaRegistry_2(address) := phi(['tbaRegistry_1', 'tbaRegistry_0', 'tbaRegistry_13'])
assetToken_10(address) := phi(['assetToken_6', 'assetToken_0', 'assetToken_17', 'assetToken_9', 'assetToken_15', 'assetToken_18', 'assetToken_1'])
WITHDRAW_ROLE_4(bytes32) := phi(['WITHDRAW_ROLE_3', 'WITHDRAW_ROLE_6', 'WITHDRAW_ROLE_0'])
_applications_7(mapping(uint256 => AgentFactoryV2.Application)) := phi(['_applications_11', '_applications_1', '_applications_6', '_applications_0', '_applications_2'])
_vault_2(address) := phi(['_vault_13', '_vault_1', '_vault_12', '_vault_0'])
_tokenAdmin_1(address) := phi(['_tokenAdmin_2', '_tokenAdmin_0', '_tokenAdmin_4', '_tokenAdmin_5'])
 require(bool,string)(_applications[id].status == ApplicationStatus.Active,Application is not active)
REF_4972(AgentFactoryV2.Application) -> _applications_8[id_1]
REF_4973(AgentFactoryV2.ApplicationStatus) -> REF_4972.status
REF_4974(AgentFactoryV2.ApplicationStatus) -> ApplicationStatus.Active
TMP_12212(bool) = REF_4973 == REF_4974
TMP_12213(None) = SOLIDITY_CALL require(bool,string)(TMP_12212,Application is not active)
 require(bool,string)(_tokenAdmin != address(0),Token admin not set)
TMP_12214 = CONVERT 0 to address
TMP_12215(bool) = _tokenAdmin_2 != TMP_12214
TMP_12216(None) = SOLIDITY_CALL require(bool,string)(TMP_12215,Token admin not set)
 application = _applications[id]
REF_4975(AgentFactoryV2.Application) -> _applications_8[id_1]
application_1 (-> ['_applications'])(AgentFactoryV2.Application) := REF_4975(AgentFactoryV2.Application)
 require(bool,string)(msg.sender == application.proposer || hasRole(WITHDRAW_ROLE,msg.sender),Not proposer)
REF_4976(address) -> application_1 (-> ['_applications']).proposer
TMP_12217(bool) = msg.sender == REF_4976
TMP_12218(bool) = INTERNAL_CALL, AccessControl.hasRole(bytes32,address)(WITHDRAW_ROLE_5,msg.sender)
TMP_12219(bool) = TMP_12217 || TMP_12218
TMP_12220(None) = SOLIDITY_CALL require(bool,string)(TMP_12219,Not proposer)
 initialAmount = application.withdrawableAmount
REF_4977(uint256) -> application_1 (-> ['_applications']).withdrawableAmount
initialAmount_1(uint256) := REF_4977(uint256)
 application.withdrawableAmount = 0
REF_4978(uint256) -> application_1 (-> ['_applications']).withdrawableAmount
application_2 (-> ['_applications'])(AgentFactoryV2.Application) := phi(["application_1 (-> ['_applications'])"])
REF_4978(uint256) (->application_2 (-> ['_applications'])) := 0(uint256)
_applications_9(mapping(uint256 => AgentFactoryV2.Application)) := phi(["application_2 (-> ['_applications'])"])
 application.status = ApplicationStatus.Executed
REF_4979(AgentFactoryV2.ApplicationStatus) -> application_2 (-> ['_applications']).status
REF_4980(AgentFactoryV2.ApplicationStatus) -> ApplicationStatus.Executed
application_3 (-> ['_applications'])(AgentFactoryV2.Application) := phi(["application_2 (-> ['_applications'])"])
REF_4979(AgentFactoryV2.ApplicationStatus) (->application_3 (-> ['_applications'])) := REF_4980(AgentFactoryV2.ApplicationStatus)
_applications_10(mapping(uint256 => AgentFactoryV2.Application)) := phi(["application_3 (-> ['_applications'])"])
 token = _createNewAgentToken(application.name,application.symbol)
REF_4981(string) -> application_3 (-> ['_applications']).name
REF_4982(string) -> application_3 (-> ['_applications']).symbol
TMP_12221(address) = INTERNAL_CALL, AgentFactoryV2._createNewAgentToken(string,string)(REF_4981,REF_4982)
assetToken_13(address) := phi(['assetToken_17'])
token_1(address) := TMP_12221(address)
 lp = IAgentToken(token).liquidityPools()[0]
TMP_12222 = CONVERT token_1 to IAgentToken
TMP_12223(address[]) = HIGH_LEVEL_CALL, dest:TMP_12222(IAgentToken), function:liquidityPools, arguments:[]  
nft_6(address) := phi(['nft_18', 'nft_5', 'nft_16', 'nft_14', 'nft_1'])
tbaRegistry_6(address) := phi(['tbaRegistry_1', 'tbaRegistry_5', 'tbaRegistry_13'])
assetToken_14(address) := phi(['assetToken_6', 'assetToken_17', 'assetToken_9', 'assetToken_13', 'assetToken_15', 'assetToken_18', 'assetToken_1'])
_vault_6(address) := phi(['_vault_5', '_vault_13', '_vault_1', '_vault_12'])
REF_4984(address) -> TMP_12223[0]
lp_1(address) := REF_4984(address)
 IERC20(assetToken).transfer(token,initialAmount)
TMP_12224 = CONVERT assetToken_14 to IERC20
TMP_12225(bool) = HIGH_LEVEL_CALL, dest:TMP_12224(IERC20), function:transfer, arguments:['token_1', 'initialAmount_1']  
nft_7(address) := phi(['nft_18', 'nft_16', 'nft_14', 'nft_1', 'nft_6'])
tbaRegistry_7(address) := phi(['tbaRegistry_1', 'tbaRegistry_6', 'tbaRegistry_13'])
assetToken_15(address) := phi(['assetToken_14', 'assetToken_6', 'assetToken_17', 'assetToken_9', 'assetToken_15', 'assetToken_18', 'assetToken_1'])
_vault_7(address) := phi(['_vault_13', '_vault_1', '_vault_6', '_vault_12'])
 IAgentToken(token).addInitialLiquidity(address(this))
TMP_12226 = CONVERT token_1 to IAgentToken
TMP_12227 = CONVERT this to address
HIGH_LEVEL_CALL, dest:TMP_12226(IAgentToken), function:addInitialLiquidity, arguments:['TMP_12227']  
nft_8(address) := phi(['nft_18', 'nft_7', 'nft_16', 'nft_14', 'nft_1'])
tbaRegistry_8(address) := phi(['tbaRegistry_1', 'tbaRegistry_7', 'tbaRegistry_13'])
_vault_8(address) := phi(['_vault_13', '_vault_1', '_vault_12', '_vault_7'])
 veToken = _createNewAgentVeToken(string.concat(Staked ,application.name),string.concat(s,application.symbol),lp,application.proposer,canStake)
REF_4988(string) -> application_3 (-> ['_applications']).name
TMP_12229(string) = SOLIDITY_CALL string.concat()(Staked ,REF_4988)
REF_4990(string) -> application_3 (-> ['_applications']).symbol
TMP_12230(string) = SOLIDITY_CALL string.concat()(s,REF_4990)
REF_4991(address) -> application_3 (-> ['_applications']).proposer
TMP_12231(address) = INTERNAL_CALL, AgentFactoryV2._createNewAgentVeToken(string,string,address,address,bool)(TMP_12229,TMP_12230,lp_1,REF_4991,canStake_1)
nft_9(address) := phi(['nft_18'])
veToken_1(address) := TMP_12231(address)
 daoName = string.concat(application.name, DAO)
REF_4993(string) -> application_3 (-> ['_applications']).name
TMP_12232(string) = SOLIDITY_CALL string.concat()(REF_4993, DAO)
daoName_1(string) := TMP_12232(string)
 dao = address(_createNewDAO(daoName,IVotes(veToken),application.daoVotingPeriod,application.daoThreshold))
TMP_12233 = CONVERT veToken_1 to IVotes
REF_4994(uint32) -> application_3 (-> ['_applications']).daoVotingPeriod
REF_4995(uint256) -> application_3 (-> ['_applications']).daoThreshold
TMP_12234(address) = INTERNAL_CALL, AgentFactoryV2._createNewDAO(string,IVotes,uint32,uint256)(daoName_1,TMP_12233,REF_4994,REF_4995)
nft_10(address) := phi(['nft_16'])
TMP_12235 = CONVERT TMP_12234 to address
dao_1(address) := TMP_12235(address)
 virtualId = IAgentNft(nft).nextVirtualId()
TMP_12236 = CONVERT nft_10 to IAgentNft
TMP_12237(uint256) = HIGH_LEVEL_CALL, dest:TMP_12236(IAgentNft), function:nextVirtualId, arguments:[]  
nft_11(address) := phi(['nft_18', 'nft_16', 'nft_14', 'nft_1', 'nft_10'])
tbaRegistry_11(address) := phi(['tbaRegistry_1', 'tbaRegistry_10', 'tbaRegistry_13'])
_vault_11(address) := phi(['_vault_10', '_vault_13', '_vault_1', '_vault_12'])
virtualId_1(uint256) := TMP_12237(uint256)
 IAgentNft(nft).mint(virtualId,_vault,application.tokenURI,dao,application.proposer,application.cores,lp,token)
TMP_12238 = CONVERT nft_11 to IAgentNft
REF_4998(string) -> application_3 (-> ['_applications']).tokenURI
REF_4999(address) -> application_3 (-> ['_applications']).proposer
REF_5000(uint8[]) -> application_3 (-> ['_applications']).cores
TMP_12239(uint256) = HIGH_LEVEL_CALL, dest:TMP_12238(IAgentNft), function:mint, arguments:['virtualId_1', '_vault_11', 'REF_4998', 'dao_1', 'REF_4999', 'REF_5000', 'lp_1', 'token_1']  
nft_12(address) := phi(['nft_11', 'nft_18', 'nft_16', 'nft_14', 'nft_1'])
tbaRegistry_12(address) := phi(['tbaRegistry_1', 'tbaRegistry_11', 'tbaRegistry_13'])
_vault_12(address) := phi(['_vault_13', '_vault_1', '_vault_11', '_vault_12'])
 application.virtualId = virtualId
REF_5001(uint256) -> application_3 (-> ['_applications']).virtualId
application_4 (-> ['_applications'])(AgentFactoryV2.Application) := phi(["application_3 (-> ['_applications'])"])
REF_5001(uint256) (->application_4 (-> ['_applications'])) := virtualId_1(uint256)
_applications_11(mapping(uint256 => AgentFactoryV2.Application)) := phi(["application_4 (-> ['_applications'])"])
 chainId = chainid()()
TMP_12240(uint256) = SOLIDITY_CALL chainid()()
chainId_1(uint256) := TMP_12240(uint256)
 tbaAddress = IERC6551Registry(tbaRegistry).createAccount(application.tbaImplementation,application.tbaSalt,chainId,nft,virtualId)
TMP_12241 = CONVERT tbaRegistry_12 to IERC6551Registry
REF_5003(address) -> application_4 (-> ['_applications']).tbaImplementation
REF_5004(bytes32) -> application_4 (-> ['_applications']).tbaSalt
TMP_12242(address) = HIGH_LEVEL_CALL, dest:TMP_12241(IERC6551Registry), function:createAccount, arguments:['REF_5003', 'REF_5004', 'chainId_1', 'nft_12', 'virtualId_1']  
nft_13(address) := phi(['nft_18', 'nft_16', 'nft_14', 'nft_1', 'nft_12'])
tbaRegistry_13(address) := phi(['tbaRegistry_1', 'tbaRegistry_12', 'tbaRegistry_13'])
tbaAddress_1(address) := TMP_12242(address)
 IAgentNft(nft).setTBA(virtualId,tbaAddress)
TMP_12243 = CONVERT nft_13 to IAgentNft
HIGH_LEVEL_CALL, dest:TMP_12243(IAgentNft), function:setTBA, arguments:['virtualId_1', 'tbaAddress_1']  
nft_14(address) := phi(['nft_18', 'nft_16', 'nft_14', 'nft_13', 'nft_1'])
 IERC20(lp).approve(veToken,type()(uint256).max)
TMP_12245 = CONVERT lp_1 to IERC20
TMP_12247(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_12248(bool) = HIGH_LEVEL_CALL, dest:TMP_12245(IERC20), function:approve, arguments:['veToken_1', 'TMP_12247']  
 IAgentVeToken(veToken).stake(IERC20(lp).balanceOf(address(this)),application.proposer,application.proposer)
TMP_12249 = CONVERT veToken_1 to IAgentVeToken
TMP_12250 = CONVERT lp_1 to IERC20
TMP_12251 = CONVERT this to address
TMP_12252(uint256) = HIGH_LEVEL_CALL, dest:TMP_12250(IERC20), function:balanceOf, arguments:['TMP_12251']  
REF_5009(address) -> application_4 (-> ['_applications']).proposer
REF_5010(address) -> application_4 (-> ['_applications']).proposer
HIGH_LEVEL_CALL, dest:TMP_12249(IAgentVeToken), function:stake, arguments:['TMP_12252', 'REF_5009', 'REF_5010']  
 NewPersona(virtualId,token,dao,tbaAddress,veToken,lp)
Emit NewPersona(virtualId_1,token_1,dao_1,tbaAddress_1,veToken_1,lp_1)
 noReentrant()
MODIFIER_CALL, AgentFactoryV2.noReentrant()()
```
#### AgentFactoryV2.getApplication(uint256) [PUBLIC]
```slithir
_applications_1(mapping(uint256 => AgentFactoryV2.Application)) := phi(['_applications_11', '_applications_1', '_applications_6', '_applications_0', '_applications_2'])
 _applications[proposalId]
REF_4954(AgentFactoryV2.Application) -> _applications_1[proposalId_1]
RETURN REF_4954
```
#### AgentFactoryV2.initialize(address,address,address,address,address,address,uint256,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_24'])
 __Pausable_init()
INTERNAL_CALL, PausableUpgradeable.__Pausable_init()()
 tokenImplementation = tokenImplementation_
tokenImplementation_1(address) := tokenImplementation__1(address)
 veTokenImplementation = veTokenImplementation_
veTokenImplementation_1(address) := veTokenImplementation__1(address)
 daoImplementation = daoImplementation_
daoImplementation_1(address) := daoImplementation__1(address)
 assetToken = assetToken_
assetToken_1(address) := assetToken__1(address)
 tbaRegistry = tbaRegistry_
tbaRegistry_1(address) := tbaRegistry__1(address)
 nft = nft_
nft_1(address) := nft__1(address)
 applicationThreshold = applicationThreshold_
applicationThreshold_1(uint256) := applicationThreshold__1(uint256)
 _nextId = 1
_nextId_1(uint256) := 1(uint256)
 _grantRole(DEFAULT_ADMIN_ROLE,msg.sender)
TMP_12180(bool) = INTERNAL_CALL, AccessControl._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_3,msg.sender)
 _vault = vault_
_vault_1(address) := vault__1(address)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### AgentFactoryV2.pause() [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_23(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_24'])
 _pause()
INTERNAL_CALL, PausableUpgradeable._pause()()
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_23)
```
#### AgentFactoryV2.proposeAgent(string,string,string,uint8[],bytes32,address,uint32,uint256) [PUBLIC]
```slithir
_nextId_2(uint256) := phi(['_nextId_0', '_nextId_1', '_nextId_7'])
applicationThreshold_2(uint256) := phi(['applicationThreshold_7', 'applicationThreshold_1', 'applicationThreshold_6', 'applicationThreshold_0'])
assetToken_2(address) := phi(['assetToken_6', 'assetToken_0', 'assetToken_17', 'assetToken_9', 'assetToken_15', 'assetToken_18', 'assetToken_1'])
 sender = _msgSender()
TMP_12182(address) = INTERNAL_CALL, AgentFactoryV2._msgSender()()
sender_1(address) := TMP_12182(address)
 require(bool,string)(IERC20(assetToken).balanceOf(sender) >= applicationThreshold,Insufficient asset token)
TMP_12183 = CONVERT assetToken_4 to IERC20
TMP_12184(uint256) = HIGH_LEVEL_CALL, dest:TMP_12183(IERC20), function:balanceOf, arguments:['sender_1']  
_nextId_5(uint256) := phi(['_nextId_1', '_nextId_7', '_nextId_4'])
applicationThreshold_5(uint256) := phi(['applicationThreshold_7', 'applicationThreshold_1', 'applicationThreshold_6', 'applicationThreshold_4'])
assetToken_5(address) := phi(['assetToken_6', 'assetToken_4', 'assetToken_17', 'assetToken_9', 'assetToken_15', 'assetToken_18', 'assetToken_1'])
TMP_12185(bool) = TMP_12184 >= applicationThreshold_5
TMP_12186(None) = SOLIDITY_CALL require(bool,string)(TMP_12185,Insufficient asset token)
 require(bool,string)(IERC20(assetToken).allowance(sender,address(this)) >= applicationThreshold,Insufficient asset token allowance)
TMP_12187 = CONVERT assetToken_5 to IERC20
TMP_12188 = CONVERT this to address
TMP_12189(uint256) = HIGH_LEVEL_CALL, dest:TMP_12187(IERC20), function:allowance, arguments:['sender_1', 'TMP_12188']  
_nextId_6(uint256) := phi(['_nextId_1', '_nextId_7', '_nextId_5'])
applicationThreshold_6(uint256) := phi(['applicationThreshold_7', 'applicationThreshold_1', 'applicationThreshold_5', 'applicationThreshold_6'])
assetToken_6(address) := phi(['assetToken_6', 'assetToken_17', 'assetToken_9', 'assetToken_5', 'assetToken_15', 'assetToken_18', 'assetToken_1'])
TMP_12190(bool) = TMP_12189 >= applicationThreshold_6
TMP_12191(None) = SOLIDITY_CALL require(bool,string)(TMP_12190,Insufficient asset token allowance)
 require(bool,string)(cores.length > 0,Cores must be provided)
REF_4957 -> LENGTH cores_1
TMP_12192(bool) = REF_4957 > 0
TMP_12193(None) = SOLIDITY_CALL require(bool,string)(TMP_12192,Cores must be provided)
 IERC20(assetToken).safeTransferFrom(sender,address(this),applicationThreshold)
TMP_12194 = CONVERT assetToken_6 to IERC20
TMP_12195 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_12194', 'sender_1', 'TMP_12195', 'applicationThreshold_6'] 
 id = _nextId ++
TMP_12197(uint256) := _nextId_6(uint256)
_nextId_7(uint256) = _nextId_6 (c)+ 1
id_1(uint256) := TMP_12197(uint256)
 proposalEndBlock = block.number
proposalEndBlock_1(uint256) := block.number(uint256)
 application = Application(name,symbol,tokenURI,ApplicationStatus.Active,applicationThreshold,sender,cores,proposalEndBlock,0,tbaSalt,tbaImplementation,daoVotingPeriod,daoThreshold)
REF_4959(AgentFactoryV2.ApplicationStatus) -> ApplicationStatus.Active
TMP_12198(AgentFactoryV2.Application) = new Application(name_1,symbol_1,tokenURI_1,REF_4959,applicationThreshold_6,sender_1,cores_1,proposalEndBlock_1,0,tbaSalt_1,tbaImplementation_1,daoVotingPeriod_1,daoThreshold_1)
application_1(AgentFactoryV2.Application) := TMP_12198(AgentFactoryV2.Application)
 _applications[id] = application
REF_4960(AgentFactoryV2.Application) -> _applications_1[id_1]
_applications_2(mapping(uint256 => AgentFactoryV2.Application)) := phi(['_applications_1'])
REF_4960(AgentFactoryV2.Application) (->_applications_2) := application_1(AgentFactoryV2.Application)
 NewApplication(id)
Emit NewApplication(id_1)
 id
RETURN id_1
 whenNotPaused()
MODIFIER_CALL, PausableUpgradeable.whenNotPaused()()
```
#### AgentFactoryV2.setApplicationThreshold(uint256) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_5(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_24'])
 applicationThreshold = newThreshold
applicationThreshold_7(uint256) := newThreshold_1(uint256)
 ApplicationThresholdUpdated(newThreshold)
Emit ApplicationThresholdUpdated(newThreshold_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_5)
```
#### AgentFactoryV2.setAssetToken(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_21(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_24'])
 assetToken = newToken
assetToken_18(address) := newToken_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_21)
```
#### AgentFactoryV2.setImplementations(address,address,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_9(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_24'])
 tokenImplementation = token
tokenImplementation_3(address) := token_1(address)
 daoImplementation = dao
daoImplementation_3(address) := dao_1(address)
 veTokenImplementation = veToken
veTokenImplementation_3(address) := veToken_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_9)
```
#### AgentFactoryV2.setMaturityDuration(uint256) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_11(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_24'])
 maturityDuration = newDuration
maturityDuration_3(uint256) := newDuration_1(uint256)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_11)
```
#### AgentFactoryV2.setTokenAdmin(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_15(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_24'])
 _tokenAdmin = newTokenAdmin
_tokenAdmin_5(address) := newTokenAdmin_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_15)
```
#### AgentFactoryV2.setTokenSupplyParams(uint256,uint256,uint256,uint256,uint256,uint256,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_17(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_24'])
 _tokenSupplyParams = abi.encode(maxSupply,lpSupply,vaultSupply,maxTokensPerWallet,maxTokensPerTxn,botProtectionDurationInSeconds,vault)
TMP_12284(bytes) = SOLIDITY_CALL abi.encode()(maxSupply_1,lpSupply_1,vaultSupply_1,maxTokensPerWallet_1,maxTokensPerTxn_1,botProtectionDurationInSeconds_1,vault_1)
_tokenSupplyParams_3(bytes) := TMP_12284(bytes)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_17)
```
#### AgentFactoryV2.setTokenTaxParams(uint256,uint256,uint256,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_19(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_24'])
 _tokenTaxParams = abi.encode(projectBuyTaxBasisPoints,projectSellTaxBasisPoints,taxSwapThresholdBasisPoints,projectTaxRecipient)
TMP_12286(bytes) = SOLIDITY_CALL abi.encode()(projectBuyTaxBasisPoints_1,projectSellTaxBasisPoints_1,taxSwapThresholdBasisPoints_1,projectTaxRecipient_1)
_tokenTaxParams_3(bytes) := TMP_12286(bytes)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_19)
```
#### AgentFactoryV2.setUniswapRouter(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_13(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_24'])
 _uniswapRouter = router
_uniswapRouter_3(address) := router_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_13)
```
#### AgentFactoryV2.setVault(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_7(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_24'])
 _vault = newVault
_vault_13(address) := newVault_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_7)
```
#### AgentDAO.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 BALLOT_TYPEHASH = keccak256(bytes)(Ballot(uint256 proposalId,uint8 support,address voter,uint256 nonce))
 EXTENDED_BALLOT_TYPEHASH = keccak256(bytes)(ExtendedBallot(uint256 proposalId,uint8 support,address voter,uint256 nonce,string reason,bytes params))
 _checkGovernance()
INTERNAL_CALL, GovernorUpgradeable._checkGovernance()()
 $ = _getInitializableStorage()
TMP_12089(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_12089'])(Initializable.InitializableStorage) := TMP_12089(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_4919(bool) -> $_1 (-> ['TMP_12089'])._initializing
TMP_12090 = UnaryType.BANG REF_4919 
isTopLevelCall_1(bool) := TMP_12090(bool)
 initialized = $._initialized
REF_4920(uint64) -> $_1 (-> ['TMP_12089'])._initialized
initialized_1(uint64) := REF_4920(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_12091(bool) = initialized_1 == 0
TMP_12092(bool) = TMP_12091 && isTopLevelCall_1
initialSetup_1(bool) := TMP_12092(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_12093(bool) = initialized_1 == 1
TMP_12094 = CONVERT this to address
TMP_12095(bytes) = SOLIDITY_CALL code(address)(TMP_12094)
REF_4921 -> LENGTH TMP_12095
TMP_12096(bool) = REF_4921 == 0
TMP_12097(bool) = TMP_12093 && TMP_12096
construction_1(bool) := TMP_12097(bool)
 ! initialSetup && ! construction
TMP_12098 = UnaryType.BANG initialSetup_1 
TMP_12099 = UnaryType.BANG construction_1 
TMP_12100(bool) = TMP_12098 && TMP_12099
CONDITION TMP_12100
 revert InvalidInitialization()()
TMP_12101(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_4922(uint64) -> $_1 (-> ['TMP_12089'])._initialized
$_2 (-> ['TMP_12089'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_12089'])"])
REF_4922(uint64) (->$_2 (-> ['TMP_12089'])) := 1(uint256)
TMP_12089(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12089'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_4923(bool) -> $_2 (-> ['TMP_12089'])._initializing
$_3 (-> ['TMP_12089'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12089'])"])
REF_4923(bool) (->$_3 (-> ['TMP_12089'])) := True(bool)
TMP_12089(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12089'])"])
$_4 (-> ['TMP_12089'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12089'])", "$_2 (-> ['TMP_12089'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_4924(bool) -> $_4 (-> ['TMP_12089'])._initializing
$_5 (-> ['TMP_12089'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_12089'])"])
REF_4924(bool) (->$_5 (-> ['TMP_12089'])) := False(bool)
TMP_12089(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_12089'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_12103(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_12103'])(Initializable.InitializableStorage) := TMP_12103(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_4925(bool) -> $_1 (-> ['TMP_12103'])._initializing
REF_4926(uint64) -> $_1 (-> ['TMP_12103'])._initialized
TMP_12104(bool) = REF_4926 >= version_1
TMP_12105(bool) = REF_4925 || TMP_12104
CONDITION TMP_12105
 revert InvalidInitialization()()
TMP_12106(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_4927(uint64) -> $_1 (-> ['TMP_12103'])._initialized
$_2 (-> ['TMP_12103'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_12103'])"])
REF_4927(uint64) (->$_2 (-> ['TMP_12103'])) := version_1(uint64)
TMP_12103(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12103'])"])
 $._initializing = true
REF_4928(bool) -> $_2 (-> ['TMP_12103'])._initializing
$_3 (-> ['TMP_12103'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12103'])"])
REF_4928(bool) (->$_3 (-> ['TMP_12103'])) := True(bool)
TMP_12103(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12103'])"])
 $._initializing = false
REF_4929(bool) -> $_3 (-> ['TMP_12103'])._initializing
$_4 (-> ['TMP_12103'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12103'])"])
REF_4929(bool) (->$_4 (-> ['TMP_12103'])) := False(bool)
TMP_12103(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_12103'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
```
#### AgentFactoryV2.totalAgents() [PUBLIC]
```slithir
allTokens_5(address[]) := phi(['allTokens_4', 'allTokens_0', 'allTokens_5'])
 allTokens.length
REF_5027 -> LENGTH allTokens_5
RETURN REF_5027
```
#### AgentFactoryV2.unpause() [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_25(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_24'])
 _unpause()
INTERNAL_CALL, PausableUpgradeable._unpause()()
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_25)
```
#### AgentFactoryV2.withdraw(uint256) [PUBLIC]
```slithir
assetToken_7(address) := phi(['assetToken_6', 'assetToken_0', 'assetToken_17', 'assetToken_9', 'assetToken_15', 'assetToken_18', 'assetToken_1'])
WITHDRAW_ROLE_1(bytes32) := phi(['WITHDRAW_ROLE_3', 'WITHDRAW_ROLE_6', 'WITHDRAW_ROLE_0'])
_applications_3(mapping(uint256 => AgentFactoryV2.Application)) := phi(['_applications_11', '_applications_1', '_applications_6', '_applications_0', '_applications_2'])
 application = _applications[id]
REF_4961(AgentFactoryV2.Application) -> _applications_4[id_1]
application_1 (-> ['_applications'])(AgentFactoryV2.Application) := REF_4961(AgentFactoryV2.Application)
 require(bool,string)(msg.sender == application.proposer || hasRole(WITHDRAW_ROLE,msg.sender),Not proposer)
REF_4962(address) -> application_1 (-> ['_applications']).proposer
TMP_12201(bool) = msg.sender == REF_4962
TMP_12202(bool) = INTERNAL_CALL, AccessControl.hasRole(bytes32,address)(WITHDRAW_ROLE_2,msg.sender)
TMP_12203(bool) = TMP_12201 || TMP_12202
TMP_12204(None) = SOLIDITY_CALL require(bool,string)(TMP_12203,Not proposer)
 require(bool,string)(application.status == ApplicationStatus.Active,Application is not active)
REF_4963(AgentFactoryV2.ApplicationStatus) -> application_1 (-> ['_applications']).status
REF_4964(AgentFactoryV2.ApplicationStatus) -> ApplicationStatus.Active
TMP_12205(bool) = REF_4963 == REF_4964
TMP_12206(None) = SOLIDITY_CALL require(bool,string)(TMP_12205,Application is not active)
 require(bool,string)(block.number > application.proposalEndBlock,Application is not matured yet)
REF_4965(uint256) -> application_1 (-> ['_applications']).proposalEndBlock
TMP_12207(bool) = block.number > REF_4965
TMP_12208(None) = SOLIDITY_CALL require(bool,string)(TMP_12207,Application is not matured yet)
 withdrawableAmount = application.withdrawableAmount
REF_4966(uint256) -> application_1 (-> ['_applications']).withdrawableAmount
withdrawableAmount_1(uint256) := REF_4966(uint256)
 application.withdrawableAmount = 0
REF_4967(uint256) -> application_1 (-> ['_applications']).withdrawableAmount
application_2 (-> ['_applications'])(AgentFactoryV2.Application) := phi(["application_1 (-> ['_applications'])"])
REF_4967(uint256) (->application_2 (-> ['_applications'])) := 0(uint256)
_applications_5(mapping(uint256 => AgentFactoryV2.Application)) := phi(["application_2 (-> ['_applications'])"])
 application.status = ApplicationStatus.Withdrawn
REF_4968(AgentFactoryV2.ApplicationStatus) -> application_2 (-> ['_applications']).status
REF_4969(AgentFactoryV2.ApplicationStatus) -> ApplicationStatus.Withdrawn
application_3 (-> ['_applications'])(AgentFactoryV2.Application) := phi(["application_2 (-> ['_applications'])"])
REF_4968(AgentFactoryV2.ApplicationStatus) (->application_3 (-> ['_applications'])) := REF_4969(AgentFactoryV2.ApplicationStatus)
_applications_6(mapping(uint256 => AgentFactoryV2.Application)) := phi(["application_3 (-> ['_applications'])"])
 IERC20(assetToken).safeTransfer(application.proposer,withdrawableAmount)
TMP_12209 = CONVERT assetToken_9 to IERC20
REF_4971(address) -> application_3 (-> ['_applications']).proposer
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_12209', 'REF_4971', 'withdrawableAmount_1'] 
 noReentrant()
MODIFIER_CALL, AgentFactoryV2.noReentrant()()
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
#### IERC20.approve(address,uint256) [EXTERNAL]
```slithir

```
#### IERC20.balanceOf(address) [EXTERNAL]
```slithir

```
#### IERC20.transfer(address,uint256) [EXTERNAL]
```slithir

```
#### IERC6551Registry.createAccount(address,bytes32,uint256,address,uint256) [EXTERNAL]
```slithir

```
#### IAgentNft.mint(uint256,address,string,address,address,uint8[],address,address) [EXTERNAL]
```slithir

```
#### IAgentNft.nextVirtualId() [EXTERNAL]
```slithir

```
#### IAgentNft.setTBA(uint256,address) [EXTERNAL]
```slithir

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
#### IERC20.allowance(address,address) [EXTERNAL]
```slithir

```
#### SafeERC20.safeTransferFrom(IERC20,address,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transferFrom,(from,to,value)))
REF_2046(transferFrom) -> token_1.transferFrom
TMP_5415(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2046,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000160>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001150>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001c90>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5415)
```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_2044(transfer) -> token_1.transfer
TMP_5413(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2044,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000550>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001210>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5413)
```
#### SafeERC20._callOptionalReturn(IERC20,bytes) [PRIVATE]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1', 'token_1'])
data_1(bytes) := phi(['approvalCall_1', 'TMP_5413', 'TMP_5415', 'TMP_5430'])
 returndata = address(token).functionCall(data)
TMP_5433 = CONVERT token_1 to address
TMP_5434(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionCall(address,bytes), arguments:['TMP_5433', 'data_1'] 
returndata_1(bytes) := TMP_5434(bytes)
 returndata.length != 0 && ! abi.decode(returndata,(bool))
REF_2054 -> LENGTH returndata_1
TMP_5435(bool) = REF_2054 != 0
TMP_5436(bool) = SOLIDITY_CALL abi.decode()(returndata_1,bool)
TMP_5437 = UnaryType.BANG TMP_5436 
TMP_5438(bool) = TMP_5435 && TMP_5437
CONDITION TMP_5438
 revert SafeERC20FailedOperation(address)(address(token))
TMP_5439 = CONVERT token_1 to address
TMP_5440(None) = SOLIDITY_CALL revert SafeERC20FailedOperation(address)(TMP_5439)
```
#### Address.functionCall(address,bytes) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0)
TMP_5457(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256)(target_1,data_1,0)
RETURN TMP_5457
```
