



### Storage layout (AgentFactoryV3) 

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
_applications mapping(uint256 => AgentFactoryV3.Application)
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





#### AgentFactoryV3._createNewAgentToken(string,string,bytes) [INTERNAL]
```slithir
name_1(string) := phi(['REF_5093'])
symbol_1(string) := phi(['REF_5094'])
tokenSupplyParams__1(bytes) := phi(['tokenSupplyParams__1'])
tokenImplementation_2(address) := phi(['tokenImplementation_3', 'tokenImplementation_0', 'tokenImplementation_1'])
assetToken_13(address) := phi(['assetToken_15', 'assetToken_14', 'assetToken_1', 'assetToken_21', 'assetToken_0', 'assetToken_6', 'assetToken_12', 'assetToken_9'])
allTradingTokens_1(address[]) := phi(['allTradingTokens_0', 'allTradingTokens_4'])
_uniswapRouter_1(address) := phi(['_uniswapRouter_0', '_uniswapRouter_2', '_uniswapRouter_3'])
_tokenAdmin_2(address) := phi(['_tokenAdmin_3', '_tokenAdmin_4', '_tokenAdmin_0'])
_tokenTaxParams_1(bytes) := phi(['_tokenTaxParams_0', '_tokenTaxParams_2', '_tokenTaxParams_3'])
 instance = Clones.clone(tokenImplementation)
TMP_12477(address) = LIBRARY_CALL, dest:Clones, function:Clones.clone(address), arguments:['tokenImplementation_2'] 
instance_1(address) := TMP_12477(address)
 IAgentToken(instance).initialize((_tokenAdmin,_uniswapRouter,assetToken),abi.encode(name,symbol),tokenSupplyParams_,_tokenTaxParams)
TMP_12478 = CONVERT instance_1 to IAgentToken
TMP_12479(bytes) = SOLIDITY_CALL abi.encode()(name_1,symbol_1)
HIGH_LEVEL_CALL, dest:TMP_12478(IAgentToken), function:initialize, arguments:['[<slither.slithir.variables.state_variable.StateIRVariable object at 0xffff74bdd810>, <slither.slithir.variables.state_variable.StateIRVariable object at 0xffff74bdd7b0>, <slither.slithir.variables.state_variable.StateIRVariable object at 0xffff74bdcf40>]', 'TMP_12479', 'tokenSupplyParams__1', '_tokenTaxParams_1']  
assetToken_14(address) := phi(['assetToken_15', 'assetToken_14', 'assetToken_1', 'assetToken_21', 'assetToken_6', 'assetToken_13', 'assetToken_12', 'assetToken_9'])
allTradingTokens_2(address[]) := phi(['allTradingTokens_1', 'allTradingTokens_4'])
_uniswapRouter_2(address) := phi(['_uniswapRouter_2', '_uniswapRouter_1', '_uniswapRouter_3'])
_tokenAdmin_3(address) := phi(['_tokenAdmin_3', '_tokenAdmin_2', '_tokenAdmin_4'])
_tokenTaxParams_2(bytes) := phi(['_tokenTaxParams_1', '_tokenTaxParams_2', '_tokenTaxParams_3'])
 allTradingTokens.push(instance)
REF_5133 -> LENGTH allTradingTokens_2
TMP_12482(uint256) := REF_5133(uint256)
TMP_12483(uint256) = TMP_12482 (c)+ 1
allTradingTokens_3(address[]) := phi(['allTradingTokens_2'])
REF_5133(uint256) (->allTradingTokens_3) := TMP_12483(uint256)
REF_5134(address) -> allTradingTokens_3[TMP_12482]
allTradingTokens_4(address[]) := phi(['allTradingTokens_3'])
REF_5134(address) (->allTradingTokens_4) := instance_1(address)
 instance
RETURN instance_1
 instance
```
#### AgentFactoryV3._createNewAgentVeToken(string,string,address,address,bool) [INTERNAL]
```slithir
name_1(string) := phi(['TMP_12439'])
symbol_1(string) := phi(['TMP_12440'])
stakingAsset_1(address) := phi(['lp_1'])
founder_1(address) := phi(['REF_5103'])
canStake_1(bool) := phi(['canStake_1'])
nft_14(address) := phi(['nft_13', 'nft_0', 'nft_15', 'nft_1', 'nft_20', 'nft_11'])
allTokens_1(address[]) := phi(['allTokens_5', 'allTokens_0', 'allTokens_4'])
maturityDuration_1(uint256) := phi(['maturityDuration_2', 'maturityDuration_0', 'maturityDuration_3'])
veTokenImplementation_2(address) := phi(['veTokenImplementation_3', 'veTokenImplementation_0', 'veTokenImplementation_1'])
 instance = Clones.clone(veTokenImplementation)
TMP_12484(address) = LIBRARY_CALL, dest:Clones, function:Clones.clone(address), arguments:['veTokenImplementation_2'] 
instance_1(address) := TMP_12484(address)
 IAgentVeToken(instance).initialize(name,symbol,founder,stakingAsset,block.timestamp + maturityDuration,address(nft),canStake)
TMP_12485 = CONVERT instance_1 to IAgentVeToken
TMP_12486(uint256) = block.timestamp (c)+ maturityDuration_1
TMP_12487 = CONVERT nft_14 to address
HIGH_LEVEL_CALL, dest:TMP_12485(IAgentVeToken), function:initialize, arguments:['name_1', 'symbol_1', 'founder_1', 'stakingAsset_1', 'TMP_12486', 'TMP_12487', 'canStake_1']  
nft_15(address) := phi(['nft_13', 'nft_15', 'nft_14', 'nft_1', 'nft_20', 'nft_11'])
allTokens_2(address[]) := phi(['allTokens_1', 'allTokens_5', 'allTokens_4'])
maturityDuration_2(uint256) := phi(['maturityDuration_1', 'maturityDuration_2', 'maturityDuration_3'])
 allTokens.push(instance)
REF_5138 -> LENGTH allTokens_2
TMP_12490(uint256) := REF_5138(uint256)
TMP_12491(uint256) = TMP_12490 (c)+ 1
allTokens_3(address[]) := phi(['allTokens_2'])
REF_5138(uint256) (->allTokens_3) := TMP_12491(uint256)
REF_5139(address) -> allTokens_3[TMP_12490]
allTokens_4(address[]) := phi(['allTokens_3'])
REF_5139(address) (->allTokens_4) := instance_1(address)
 instance
RETURN instance_1
 instance
```
#### AgentFactoryV3._createNewDAO(string,IVotes,uint32,uint256) [INTERNAL]
```slithir
name_1(string) := phi(['daoName_1'])
token_1(IVotes) := phi(['TMP_12443'])
daoVotingPeriod_1(uint32) := phi(['REF_5106'])
daoThreshold_1(uint256) := phi(['REF_5107'])
daoImplementation_2(address) := phi(['daoImplementation_3', 'daoImplementation_1', 'daoImplementation_0'])
nft_12(address) := phi(['nft_13', 'nft_0', 'nft_15', 'nft_1', 'nft_20', 'nft_11'])
allDAOs_1(address[]) := phi(['allDAOs_4', 'allDAOs_0'])
 instance = Clones.clone(daoImplementation)
TMP_12471(address) = LIBRARY_CALL, dest:Clones, function:Clones.clone(address), arguments:['daoImplementation_2'] 
instance_1(address) := TMP_12471(address)
 IAgentDAO(instance).initialize(name,token,nft,daoThreshold,daoVotingPeriod)
TMP_12472 = CONVERT instance_1 to IAgentDAO
HIGH_LEVEL_CALL, dest:TMP_12472(IAgentDAO), function:initialize, arguments:['name_1', 'token_1', 'nft_12', 'daoThreshold_1', 'daoVotingPeriod_1']  
nft_13(address) := phi(['nft_13', 'nft_15', 'nft_1', 'nft_20', 'nft_12', 'nft_11'])
allDAOs_2(address[]) := phi(['allDAOs_4', 'allDAOs_1'])
 allDAOs.push(instance)
REF_5127 -> LENGTH allDAOs_2
TMP_12475(uint256) := REF_5127(uint256)
TMP_12476(uint256) = TMP_12475 (c)+ 1
allDAOs_3(address[]) := phi(['allDAOs_2'])
REF_5127(uint256) (->allDAOs_3) := TMP_12476(uint256)
REF_5128(address) -> allDAOs_3[TMP_12475]
allDAOs_4(address[]) := phi(['allDAOs_3'])
REF_5128(address) (->allDAOs_4) := instance_1(address)
 instance
RETURN instance_1
 instance
```
#### AgentFactoryV3._executeApplication(uint256,bool,bytes) [INTERNAL]
```slithir
id_1(uint256) := phi(['id_1', 'id_1'])
canStake_1(bool) := phi(['canStake_1'])
tokenSupplyParams__1(bytes) := phi(['_tokenSupplyParams_3', 'tokenSupplyParams_1'])
nft_2(address) := phi(['nft_13', 'nft_0', 'nft_15', 'nft_1', 'nft_20', 'nft_11'])
tbaRegistry_2(address) := phi(['tbaRegistry_1', 'tbaRegistry_0', 'tbaRegistry_10'])
assetToken_10(address) := phi(['assetToken_15', 'assetToken_14', 'assetToken_1', 'assetToken_21', 'assetToken_0', 'assetToken_6', 'assetToken_12', 'assetToken_9'])
_applications_7(mapping(uint256 => AgentFactoryV3.Application)) := phi(['_applications_10', '_applications_6', '_applications_2', '_applications_1', '_applications_17', '_applications_12', '_applications_13', '_applications_0'])
_vault_2(address) := phi(['_vault_0', '_vault_10', '_vault_1', '_vault_9'])
_tokenAdmin_1(address) := phi(['_tokenAdmin_3', '_tokenAdmin_4', '_tokenAdmin_0'])
defaultDelegatee_1(address) := phi(['defaultDelegatee_13', 'defaultDelegatee_14', 'defaultDelegatee_0'])
 require(bool,string)(_applications[id].status == ApplicationStatus.Active,Application is not active)
REF_5085(AgentFactoryV3.Application) -> _applications_7[id_1]
REF_5086(AgentFactoryV3.ApplicationStatus) -> REF_5085.status
REF_5087(AgentFactoryV3.ApplicationStatus) -> ApplicationStatus.Active
TMP_12426(bool) = REF_5086 == REF_5087
TMP_12427(None) = SOLIDITY_CALL require(bool,string)(TMP_12426,Application is not active)
 require(bool,string)(_tokenAdmin != address(0),Token admin not set)
TMP_12428 = CONVERT 0 to address
TMP_12429(bool) = _tokenAdmin_1 != TMP_12428
TMP_12430(None) = SOLIDITY_CALL require(bool,string)(TMP_12429,Token admin not set)
 application = _applications[id]
REF_5088(AgentFactoryV3.Application) -> _applications_7[id_1]
application_1 (-> ['_applications'])(AgentFactoryV3.Application) := REF_5088(AgentFactoryV3.Application)
 initialAmount = application.withdrawableAmount
REF_5089(uint256) -> application_1 (-> ['_applications']).withdrawableAmount
initialAmount_1(uint256) := REF_5089(uint256)
 application.withdrawableAmount = 0
REF_5090(uint256) -> application_1 (-> ['_applications']).withdrawableAmount
application_2 (-> ['_applications'])(AgentFactoryV3.Application) := phi(["application_1 (-> ['_applications'])"])
REF_5090(uint256) (->application_2 (-> ['_applications'])) := 0(uint256)
_applications_8(mapping(uint256 => AgentFactoryV3.Application)) := phi(["application_2 (-> ['_applications'])"])
 application.status = ApplicationStatus.Executed
REF_5091(AgentFactoryV3.ApplicationStatus) -> application_2 (-> ['_applications']).status
REF_5092(AgentFactoryV3.ApplicationStatus) -> ApplicationStatus.Executed
application_3 (-> ['_applications'])(AgentFactoryV3.Application) := phi(["application_2 (-> ['_applications'])"])
REF_5091(AgentFactoryV3.ApplicationStatus) (->application_3 (-> ['_applications'])) := REF_5092(AgentFactoryV3.ApplicationStatus)
_applications_9(mapping(uint256 => AgentFactoryV3.Application)) := phi(["application_3 (-> ['_applications'])"])
 token = _createNewAgentToken(application.name,application.symbol,tokenSupplyParams_)
REF_5093(string) -> application_3 (-> ['_applications']).name
REF_5094(string) -> application_3 (-> ['_applications']).symbol
TMP_12431(address) = INTERNAL_CALL, AgentFactoryV3._createNewAgentToken(string,string,bytes)(REF_5093,REF_5094,tokenSupplyParams__1)
assetToken_11(address) := phi(['assetToken_14'])
token_1(address) := TMP_12431(address)
 lp = IAgentToken(token).liquidityPools()[0]
TMP_12432 = CONVERT token_1 to IAgentToken
TMP_12433(address[]) = HIGH_LEVEL_CALL, dest:TMP_12432(IAgentToken), function:liquidityPools, arguments:[]  
nft_4(address) := phi(['nft_13', 'nft_15', 'nft_1', 'nft_20', 'nft_3', 'nft_11'])
tbaRegistry_4(address) := phi(['tbaRegistry_1', 'tbaRegistry_3', 'tbaRegistry_10'])
assetToken_12(address) := phi(['assetToken_15', 'assetToken_14', 'assetToken_1', 'assetToken_21', 'assetToken_11', 'assetToken_6', 'assetToken_12', 'assetToken_9'])
_vault_4(address) := phi(['_vault_10', '_vault_1', '_vault_9', '_vault_3'])
defaultDelegatee_3(address) := phi(['defaultDelegatee_13', 'defaultDelegatee_14', 'defaultDelegatee_2'])
REF_5096(address) -> TMP_12433[0]
lp_1(address) := REF_5096(address)
 IERC20(assetToken).safeTransfer(token,initialAmount)
TMP_12434 = CONVERT assetToken_12 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_12434', 'token_1', 'initialAmount_1'] 
 IAgentToken(token).addInitialLiquidity(address(this))
TMP_12436 = CONVERT token_1 to IAgentToken
TMP_12437 = CONVERT this to address
HIGH_LEVEL_CALL, dest:TMP_12436(IAgentToken), function:addInitialLiquidity, arguments:['TMP_12437']  
nft_5(address) := phi(['nft_13', 'nft_15', 'nft_4', 'nft_1', 'nft_20', 'nft_11'])
tbaRegistry_5(address) := phi(['tbaRegistry_1', 'tbaRegistry_10', 'tbaRegistry_4'])
_vault_5(address) := phi(['_vault_4', '_vault_10', '_vault_1', '_vault_9'])
defaultDelegatee_4(address) := phi(['defaultDelegatee_13', 'defaultDelegatee_14', 'defaultDelegatee_3'])
 veToken = _createNewAgentVeToken(string.concat(Staked ,application.name),string.concat(s,application.symbol),lp,application.proposer,canStake)
REF_5100(string) -> application_3 (-> ['_applications']).name
TMP_12439(string) = SOLIDITY_CALL string.concat()(Staked ,REF_5100)
REF_5102(string) -> application_3 (-> ['_applications']).symbol
TMP_12440(string) = SOLIDITY_CALL string.concat()(s,REF_5102)
REF_5103(address) -> application_3 (-> ['_applications']).proposer
TMP_12441(address) = INTERNAL_CALL, AgentFactoryV3._createNewAgentVeToken(string,string,address,address,bool)(TMP_12439,TMP_12440,lp_1,REF_5103,canStake_1)
nft_6(address) := phi(['nft_15'])
veToken_1(address) := TMP_12441(address)
 daoName = string.concat(application.name, DAO)
REF_5105(string) -> application_3 (-> ['_applications']).name
TMP_12442(string) = SOLIDITY_CALL string.concat()(REF_5105, DAO)
daoName_1(string) := TMP_12442(string)
 dao = address(_createNewDAO(daoName,IVotes(veToken),application.daoVotingPeriod,application.daoThreshold))
TMP_12443 = CONVERT veToken_1 to IVotes
REF_5106(uint32) -> application_3 (-> ['_applications']).daoVotingPeriod
REF_5107(uint256) -> application_3 (-> ['_applications']).daoThreshold
TMP_12444(address) = INTERNAL_CALL, AgentFactoryV3._createNewDAO(string,IVotes,uint32,uint256)(daoName_1,TMP_12443,REF_5106,REF_5107)
nft_7(address) := phi(['nft_13'])
TMP_12445 = CONVERT TMP_12444 to address
dao_1(address) := TMP_12445(address)
 virtualId = IAgentNft(nft).nextVirtualId()
TMP_12446 = CONVERT nft_7 to IAgentNft
TMP_12447(uint256) = HIGH_LEVEL_CALL, dest:TMP_12446(IAgentNft), function:nextVirtualId, arguments:[]  
nft_8(address) := phi(['nft_13', 'nft_7', 'nft_15', 'nft_1', 'nft_20', 'nft_11'])
tbaRegistry_8(address) := phi(['tbaRegistry_1', 'tbaRegistry_10', 'tbaRegistry_7'])
_vault_8(address) := phi(['_vault_7', '_vault_10', '_vault_1', '_vault_9'])
defaultDelegatee_7(address) := phi(['defaultDelegatee_13', 'defaultDelegatee_14', 'defaultDelegatee_6'])
virtualId_1(uint256) := TMP_12447(uint256)
 IAgentNft(nft).mint(virtualId,_vault,application.tokenURI,dao,application.proposer,application.cores,lp,token)
TMP_12448 = CONVERT nft_8 to IAgentNft
REF_5110(string) -> application_3 (-> ['_applications']).tokenURI
REF_5111(address) -> application_3 (-> ['_applications']).proposer
REF_5112(uint8[]) -> application_3 (-> ['_applications']).cores
TMP_12449(uint256) = HIGH_LEVEL_CALL, dest:TMP_12448(IAgentNft), function:mint, arguments:['virtualId_1', '_vault_8', 'REF_5110', 'dao_1', 'REF_5111', 'REF_5112', 'lp_1', 'token_1']  
nft_9(address) := phi(['nft_13', 'nft_8', 'nft_15', 'nft_1', 'nft_20', 'nft_11'])
tbaRegistry_9(address) := phi(['tbaRegistry_1', 'tbaRegistry_10', 'tbaRegistry_8'])
_vault_9(address) := phi(['_vault_10', '_vault_1', '_vault_9', '_vault_8'])
defaultDelegatee_8(address) := phi(['defaultDelegatee_13', 'defaultDelegatee_14', 'defaultDelegatee_7'])
 application.virtualId = virtualId
REF_5113(uint256) -> application_3 (-> ['_applications']).virtualId
application_4 (-> ['_applications'])(AgentFactoryV3.Application) := phi(["application_3 (-> ['_applications'])"])
REF_5113(uint256) (->application_4 (-> ['_applications'])) := virtualId_1(uint256)
_applications_10(mapping(uint256 => AgentFactoryV3.Application)) := phi(["application_4 (-> ['_applications'])"])
 chainId = chainid()()
TMP_12450(uint256) = SOLIDITY_CALL chainid()()
chainId_1(uint256) := TMP_12450(uint256)
 tbaAddress = IERC6551Registry(tbaRegistry).createAccount(application.tbaImplementation,application.tbaSalt,chainId,nft,virtualId)
TMP_12451 = CONVERT tbaRegistry_9 to IERC6551Registry
REF_5115(address) -> application_4 (-> ['_applications']).tbaImplementation
REF_5116(bytes32) -> application_4 (-> ['_applications']).tbaSalt
TMP_12452(address) = HIGH_LEVEL_CALL, dest:TMP_12451(IERC6551Registry), function:createAccount, arguments:['REF_5115', 'REF_5116', 'chainId_1', 'nft_9', 'virtualId_1']  
nft_10(address) := phi(['nft_13', 'nft_9', 'nft_15', 'nft_1', 'nft_20', 'nft_11'])
tbaRegistry_10(address) := phi(['tbaRegistry_9', 'tbaRegistry_1', 'tbaRegistry_10'])
defaultDelegatee_9(address) := phi(['defaultDelegatee_13', 'defaultDelegatee_14', 'defaultDelegatee_8'])
tbaAddress_1(address) := TMP_12452(address)
 IAgentNft(nft).setTBA(virtualId,tbaAddress)
TMP_12453 = CONVERT nft_10 to IAgentNft
HIGH_LEVEL_CALL, dest:TMP_12453(IAgentNft), function:setTBA, arguments:['virtualId_1', 'tbaAddress_1']  
nft_11(address) := phi(['nft_13', 'nft_15', 'nft_10', 'nft_1', 'nft_20', 'nft_11'])
defaultDelegatee_10(address) := phi(['defaultDelegatee_13', 'defaultDelegatee_14', 'defaultDelegatee_9'])
 IERC20(lp).approve(veToken,type()(uint256).max)
TMP_12455 = CONVERT lp_1 to IERC20
TMP_12457(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_12458(bool) = HIGH_LEVEL_CALL, dest:TMP_12455(IERC20), function:approve, arguments:['veToken_1', 'TMP_12457']  
defaultDelegatee_11(address) := phi(['defaultDelegatee_13', 'defaultDelegatee_14', 'defaultDelegatee_10'])
 IAgentVeToken(veToken).stake(IERC20(lp).balanceOf(address(this)),application.proposer,defaultDelegatee)
TMP_12459 = CONVERT veToken_1 to IAgentVeToken
TMP_12460 = CONVERT lp_1 to IERC20
TMP_12461 = CONVERT this to address
TMP_12462(uint256) = HIGH_LEVEL_CALL, dest:TMP_12460(IERC20), function:balanceOf, arguments:['TMP_12461']  
defaultDelegatee_12(address) := phi(['defaultDelegatee_13', 'defaultDelegatee_14', 'defaultDelegatee_11'])
REF_5121(address) -> application_4 (-> ['_applications']).proposer
HIGH_LEVEL_CALL, dest:TMP_12459(IAgentVeToken), function:stake, arguments:['TMP_12462', 'REF_5121', 'defaultDelegatee_12']  
defaultDelegatee_13(address) := phi(['defaultDelegatee_13', 'defaultDelegatee_14', 'defaultDelegatee_12'])
 NewPersona(virtualId,token,dao,tbaAddress,veToken,lp)
Emit NewPersona(virtualId_1,token_1,dao_1,tbaAddress_1,veToken_1,lp_1)
```
#### AgentFactoryV3._msgData() [INTERNAL]
```slithir
 ContextUpgradeable._msgData()
TMP_12509(bytes) = INTERNAL_CALL, ContextUpgradeable._msgData()()
RETURN TMP_12509
```
#### AgentFactoryV3._msgSender() [INTERNAL]
```slithir
 sender = ContextUpgradeable._msgSender()
TMP_12508(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
sender_1(address) := TMP_12508(address)
 sender
RETURN sender_1
```
#### AgentFactoryV3.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### AgentFactoryV3.executeApplication(uint256,bool) [PUBLIC]
```slithir
WITHDRAW_ROLE_4(bytes32) := phi(['WITHDRAW_ROLE_6', 'WITHDRAW_ROLE_3', 'WITHDRAW_ROLE_0'])
_applications_11(mapping(uint256 => AgentFactoryV3.Application)) := phi(['_applications_10', '_applications_6', '_applications_2', '_applications_1', '_applications_17', '_applications_12', '_applications_13', '_applications_0'])
_tokenSupplyParams_1(bytes) := phi(['_tokenSupplyParams_0', '_tokenSupplyParams_5', '_tokenSupplyParams_4'])
 application = _applications[id]
REF_5122(AgentFactoryV3.Application) -> _applications_12[id_1]
application_1 (-> ['_applications'])(AgentFactoryV3.Application) := REF_5122(AgentFactoryV3.Application)
 require(bool,string)(msg.sender == application.proposer || hasRole(WITHDRAW_ROLE,msg.sender),Not proposer)
REF_5123(address) -> application_1 (-> ['_applications']).proposer
TMP_12465(bool) = msg.sender == REF_5123
TMP_12466(bool) = INTERNAL_CALL, AccessControl.hasRole(bytes32,address)(WITHDRAW_ROLE_5,msg.sender)
TMP_12467(bool) = TMP_12465 || TMP_12466
TMP_12468(None) = SOLIDITY_CALL require(bool,string)(TMP_12467,Not proposer)
 _executeApplication(id,canStake,_tokenSupplyParams)
INTERNAL_CALL, AgentFactoryV3._executeApplication(uint256,bool,bytes)(id_1,canStake_1,_tokenSupplyParams_3)
 noReentrant()
MODIFIER_CALL, AgentFactoryV3.noReentrant()()
```
#### AgentFactoryV3.executeBondingCurveApplication(uint256,uint256,uint256,address) [PUBLIC]
```slithir
nft_16(address) := phi(['nft_13', 'nft_0', 'nft_15', 'nft_1', 'nft_20', 'nft_11'])
_applications_14(mapping(uint256 => AgentFactoryV3.Application)) := phi(['_applications_10', '_applications_6', '_applications_2', '_applications_1', '_applications_17', '_applications_12', '_applications_13', '_applications_0'])
BONDING_ROLE_4(bytes32) := phi(['BONDING_ROLE_3', 'BONDING_ROLE_0', 'BONDING_ROLE_5'])
 tokenSupplyParams = abi.encode(totalSupply,lpSupply,totalSupply - lpSupply,totalSupply,totalSupply,0,vault)
TMP_12530(uint256) = totalSupply_1 (c)- lpSupply_1
TMP_12531(bytes) = SOLIDITY_CALL abi.encode()(totalSupply_1,lpSupply_1,TMP_12530,totalSupply_1,totalSupply_1,0,vault_1)
tokenSupplyParams_1(bytes) := TMP_12531(bytes)
 _executeApplication(id,true,tokenSupplyParams)
INTERNAL_CALL, AgentFactoryV3._executeApplication(uint256,bool,bytes)(id_1,True,tokenSupplyParams_1)
nft_19(address) := phi(['nft_11'])
_applications_17(mapping(uint256 => AgentFactoryV3.Application)) := phi(['_applications_10'])
 application = _applications[id]
REF_5152(AgentFactoryV3.Application) -> _applications_17[id_1]
application_1(AgentFactoryV3.Application) := REF_5152(AgentFactoryV3.Application)
 IAgentNft(nft).virtualInfo(application.virtualId).token
TMP_12533 = CONVERT nft_19 to IAgentNft
REF_5154(uint256) -> application_1.virtualId
TMP_12534(IAgentNft.VirtualInfo) = HIGH_LEVEL_CALL, dest:TMP_12533(IAgentNft), function:virtualInfo, arguments:['REF_5154']  
nft_20(address) := phi(['nft_13', 'nft_19', 'nft_15', 'nft_1', 'nft_20', 'nft_11'])
REF_5155(address) -> TMP_12534.token
RETURN REF_5155
 onlyRole(BONDING_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(BONDING_ROLE_4)
 noReentrant()
MODIFIER_CALL, AgentFactoryV3.noReentrant()()
```
#### AgentFactoryV3.getApplication(uint256) [PUBLIC]
```slithir
_applications_1(mapping(uint256 => AgentFactoryV3.Application)) := phi(['_applications_10', '_applications_6', '_applications_2', '_applications_1', '_applications_17', '_applications_12', '_applications_13', '_applications_0'])
 _applications[proposalId]
REF_5067(AgentFactoryV3.Application) -> _applications_1[proposalId_1]
RETURN REF_5067
```
#### AgentFactoryV3.initFromBondingCurve(string,string,uint8[],bytes32,address,uint32,uint256,uint256,address) [PUBLIC]
```slithir
_nextId_8(uint256) := phi(['_nextId_0', '_nextId_14', '_nextId_1', '_nextId_7'])
assetToken_16(address) := phi(['assetToken_15', 'assetToken_14', 'assetToken_1', 'assetToken_21', 'assetToken_0', 'assetToken_6', 'assetToken_12', 'assetToken_9'])
BONDING_ROLE_1(bytes32) := phi(['BONDING_ROLE_3', 'BONDING_ROLE_0', 'BONDING_ROLE_5'])
 sender = _msgSender()
TMP_12510(address) = INTERNAL_CALL, AgentFactoryV3._msgSender()()
sender_1(address) := TMP_12510(address)
 require(bool,string)(IERC20(assetToken).balanceOf(sender) >= applicationThreshold_,Insufficient asset token)
TMP_12511 = CONVERT assetToken_19 to IERC20
TMP_12512(uint256) = HIGH_LEVEL_CALL, dest:TMP_12511(IERC20), function:balanceOf, arguments:['sender_1']  
_nextId_12(uint256) := phi(['_nextId_14', '_nextId_1', '_nextId_7', '_nextId_11'])
assetToken_20(address) := phi(['assetToken_15', 'assetToken_14', 'assetToken_1', 'assetToken_21', 'assetToken_6', 'assetToken_19', 'assetToken_12', 'assetToken_9'])
TMP_12513(bool) = TMP_12512 >= applicationThreshold__1
TMP_12514(None) = SOLIDITY_CALL require(bool,string)(TMP_12513,Insufficient asset token)
 require(bool,string)(IERC20(assetToken).allowance(sender,address(this)) >= applicationThreshold_,Insufficient asset token allowance)
TMP_12515 = CONVERT assetToken_20 to IERC20
TMP_12516 = CONVERT this to address
TMP_12517(uint256) = HIGH_LEVEL_CALL, dest:TMP_12515(IERC20), function:allowance, arguments:['sender_1', 'TMP_12516']  
_nextId_13(uint256) := phi(['_nextId_12', '_nextId_14', '_nextId_1', '_nextId_7'])
assetToken_21(address) := phi(['assetToken_15', 'assetToken_14', 'assetToken_1', 'assetToken_21', 'assetToken_6', 'assetToken_20', 'assetToken_12', 'assetToken_9'])
TMP_12518(bool) = TMP_12517 >= applicationThreshold__1
TMP_12519(None) = SOLIDITY_CALL require(bool,string)(TMP_12518,Insufficient asset token allowance)
 require(bool,string)(cores.length > 0,Cores must be provided)
REF_5147 -> LENGTH cores_1
TMP_12520(bool) = REF_5147 > 0
TMP_12521(None) = SOLIDITY_CALL require(bool,string)(TMP_12520,Cores must be provided)
 IERC20(assetToken).safeTransferFrom(sender,address(this),applicationThreshold_)
TMP_12522 = CONVERT assetToken_21 to IERC20
TMP_12523 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_12522', 'sender_1', 'TMP_12523', 'applicationThreshold__1'] 
 id = _nextId ++
TMP_12525(uint256) := _nextId_13(uint256)
_nextId_14(uint256) = _nextId_13 (c)+ 1
id_1(uint256) := TMP_12525(uint256)
 proposalEndBlock = block.number
proposalEndBlock_1(uint256) := block.number(uint256)
 application = Application(name,symbol,,ApplicationStatus.Active,applicationThreshold_,creator,cores,proposalEndBlock,0,tbaSalt,tbaImplementation,daoVotingPeriod,daoThreshold)
REF_5149(AgentFactoryV3.ApplicationStatus) -> ApplicationStatus.Active
TMP_12526(AgentFactoryV3.Application) = new Application(name_1,symbol_1,,REF_5149,applicationThreshold__1,creator_1,cores_1,proposalEndBlock_1,0,tbaSalt_1,tbaImplementation_1,daoVotingPeriod_1,daoThreshold_1)
application_1(AgentFactoryV3.Application) := TMP_12526(AgentFactoryV3.Application)
 _applications[id] = application
REF_5150(AgentFactoryV3.Application) -> _applications_12[id_1]
_applications_13(mapping(uint256 => AgentFactoryV3.Application)) := phi(['_applications_12'])
REF_5150(AgentFactoryV3.Application) (->_applications_13) := application_1(AgentFactoryV3.Application)
 NewApplication(id)
Emit NewApplication(id_1)
 id
RETURN id_1
 whenNotPaused()
MODIFIER_CALL, PausableUpgradeable.whenNotPaused()()
 onlyRole(BONDING_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(BONDING_ROLE_2)
```
#### AgentFactoryV3.initialize(address,address,address,address,address,address,uint256,address,uint256) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12'])
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
 _nextId = nextId_
_nextId_1(uint256) := nextId__1(uint256)
 _grantRole(DEFAULT_ADMIN_ROLE,msg.sender)
TMP_12394(bool) = INTERNAL_CALL, AccessControl._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_3,msg.sender)
 _vault = vault_
_vault_1(address) := vault__1(address)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### AgentFactoryV3.pause() [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_23(bytes32) := phi(['DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12'])
 _pause()
INTERNAL_CALL, PausableUpgradeable._pause()()
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_23)
```
#### AgentFactoryV3.proposeAgent(string,string,string,uint8[],bytes32,address,uint32,uint256) [PUBLIC]
```slithir
_nextId_2(uint256) := phi(['_nextId_0', '_nextId_14', '_nextId_1', '_nextId_7'])
applicationThreshold_2(uint256) := phi(['applicationThreshold_0', 'applicationThreshold_1', 'applicationThreshold_7', 'applicationThreshold_6'])
assetToken_2(address) := phi(['assetToken_15', 'assetToken_14', 'assetToken_1', 'assetToken_21', 'assetToken_0', 'assetToken_6', 'assetToken_12', 'assetToken_9'])
 sender = _msgSender()
TMP_12396(address) = INTERNAL_CALL, AgentFactoryV3._msgSender()()
sender_1(address) := TMP_12396(address)
 require(bool,string)(IERC20(assetToken).balanceOf(sender) >= applicationThreshold,Insufficient asset token)
TMP_12397 = CONVERT assetToken_4 to IERC20
TMP_12398(uint256) = HIGH_LEVEL_CALL, dest:TMP_12397(IERC20), function:balanceOf, arguments:['sender_1']  
_nextId_5(uint256) := phi(['_nextId_4', '_nextId_14', '_nextId_1', '_nextId_7'])
applicationThreshold_5(uint256) := phi(['applicationThreshold_7', 'applicationThreshold_1', 'applicationThreshold_6', 'applicationThreshold_4'])
assetToken_5(address) := phi(['assetToken_15', 'assetToken_14', 'assetToken_1', 'assetToken_21', 'assetToken_6', 'assetToken_4', 'assetToken_12', 'assetToken_9'])
TMP_12399(bool) = TMP_12398 >= applicationThreshold_5
TMP_12400(None) = SOLIDITY_CALL require(bool,string)(TMP_12399,Insufficient asset token)
 require(bool,string)(IERC20(assetToken).allowance(sender,address(this)) >= applicationThreshold,Insufficient asset token allowance)
TMP_12401 = CONVERT assetToken_5 to IERC20
TMP_12402 = CONVERT this to address
TMP_12403(uint256) = HIGH_LEVEL_CALL, dest:TMP_12401(IERC20), function:allowance, arguments:['sender_1', 'TMP_12402']  
_nextId_6(uint256) := phi(['_nextId_14', '_nextId_1', '_nextId_7', '_nextId_5'])
applicationThreshold_6(uint256) := phi(['applicationThreshold_7', 'applicationThreshold_1', 'applicationThreshold_5', 'applicationThreshold_6'])
assetToken_6(address) := phi(['assetToken_15', 'assetToken_14', 'assetToken_1', 'assetToken_21', 'assetToken_6', 'assetToken_5', 'assetToken_12', 'assetToken_9'])
TMP_12404(bool) = TMP_12403 >= applicationThreshold_6
TMP_12405(None) = SOLIDITY_CALL require(bool,string)(TMP_12404,Insufficient asset token allowance)
 require(bool,string)(cores.length > 0,Cores must be provided)
REF_5070 -> LENGTH cores_1
TMP_12406(bool) = REF_5070 > 0
TMP_12407(None) = SOLIDITY_CALL require(bool,string)(TMP_12406,Cores must be provided)
 IERC20(assetToken).safeTransferFrom(sender,address(this),applicationThreshold)
TMP_12408 = CONVERT assetToken_6 to IERC20
TMP_12409 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_12408', 'sender_1', 'TMP_12409', 'applicationThreshold_6'] 
 id = _nextId ++
TMP_12411(uint256) := _nextId_6(uint256)
_nextId_7(uint256) = _nextId_6 (c)+ 1
id_1(uint256) := TMP_12411(uint256)
 proposalEndBlock = block.number
proposalEndBlock_1(uint256) := block.number(uint256)
 application = Application(name,symbol,tokenURI,ApplicationStatus.Active,applicationThreshold,sender,cores,proposalEndBlock,0,tbaSalt,tbaImplementation,daoVotingPeriod,daoThreshold)
REF_5072(AgentFactoryV3.ApplicationStatus) -> ApplicationStatus.Active
TMP_12412(AgentFactoryV3.Application) = new Application(name_1,symbol_1,tokenURI_1,REF_5072,applicationThreshold_6,sender_1,cores_1,proposalEndBlock_1,0,tbaSalt_1,tbaImplementation_1,daoVotingPeriod_1,daoThreshold_1)
application_1(AgentFactoryV3.Application) := TMP_12412(AgentFactoryV3.Application)
 _applications[id] = application
REF_5073(AgentFactoryV3.Application) -> _applications_1[id_1]
_applications_2(mapping(uint256 => AgentFactoryV3.Application)) := phi(['_applications_1'])
REF_5073(AgentFactoryV3.Application) (->_applications_2) := application_1(AgentFactoryV3.Application)
 NewApplication(id)
Emit NewApplication(id_1)
 id
RETURN id_1
 whenNotPaused()
MODIFIER_CALL, PausableUpgradeable.whenNotPaused()()
```
#### AgentFactoryV3.setApplicationThreshold(uint256) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_5(bytes32) := phi(['DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12'])
 applicationThreshold = newThreshold
applicationThreshold_7(uint256) := newThreshold_1(uint256)
 ApplicationThresholdUpdated(newThreshold)
Emit ApplicationThresholdUpdated(newThreshold_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_5)
```
#### AgentFactoryV3.setAssetToken(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_21(bytes32) := phi(['DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12'])
 assetToken = newToken
assetToken_15(address) := newToken_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_21)
```
#### AgentFactoryV3.setDefaultDelegatee(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_27(bytes32) := phi(['DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12'])
 defaultDelegatee = newDelegatee
defaultDelegatee_14(address) := newDelegatee_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_27)
```
#### AgentFactoryV3.setImplementations(address,address,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_9(bytes32) := phi(['DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12'])
 tokenImplementation = token
tokenImplementation_3(address) := token_1(address)
 daoImplementation = dao
daoImplementation_3(address) := dao_1(address)
 veTokenImplementation = veToken
veTokenImplementation_3(address) := veToken_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_9)
```
#### AgentFactoryV3.setMaturityDuration(uint256) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_11(bytes32) := phi(['DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12'])
 maturityDuration = newDuration
maturityDuration_3(uint256) := newDuration_1(uint256)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_11)
```
#### AgentFactoryV3.setTokenAdmin(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_15(bytes32) := phi(['DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12'])
 _tokenAdmin = newTokenAdmin
_tokenAdmin_4(address) := newTokenAdmin_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_15)
```
#### AgentFactoryV3.setTokenSupplyParams(uint256,uint256,uint256,uint256,uint256,uint256,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_17(bytes32) := phi(['DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12'])
 _tokenSupplyParams = abi.encode(maxSupply,lpSupply,vaultSupply,maxTokensPerWallet,maxTokensPerTxn,botProtectionDurationInSeconds,vault)
TMP_12499(bytes) = SOLIDITY_CALL abi.encode()(maxSupply_1,lpSupply_1,vaultSupply_1,maxTokensPerWallet_1,maxTokensPerTxn_1,botProtectionDurationInSeconds_1,vault_1)
_tokenSupplyParams_5(bytes) := TMP_12499(bytes)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_17)
```
#### AgentFactoryV3.setTokenTaxParams(uint256,uint256,uint256,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_19(bytes32) := phi(['DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12'])
 _tokenTaxParams = abi.encode(projectBuyTaxBasisPoints,projectSellTaxBasisPoints,taxSwapThresholdBasisPoints,projectTaxRecipient)
TMP_12501(bytes) = SOLIDITY_CALL abi.encode()(projectBuyTaxBasisPoints_1,projectSellTaxBasisPoints_1,taxSwapThresholdBasisPoints_1,projectTaxRecipient_1)
_tokenTaxParams_3(bytes) := TMP_12501(bytes)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_19)
```
#### AgentFactoryV3.setUniswapRouter(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_13(bytes32) := phi(['DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12'])
 _uniswapRouter = router
_uniswapRouter_3(address) := router_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_13)
```
#### AgentFactoryV3.setVault(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_7(bytes32) := phi(['DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12'])
 _vault = newVault
_vault_10(address) := newVault_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_7)
```
#### AgentFactoryV2.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
 WITHDRAW_ROLE = keccak256(bytes)(WITHDRAW_ROLE)
 _requireNotPaused()
INTERNAL_CALL, PausableUpgradeable._requireNotPaused()()
 _requirePaused()
INTERNAL_CALL, PausableUpgradeable._requirePaused()()
 $ = _getInitializableStorage()
TMP_12298(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_12298'])(Initializable.InitializableStorage) := TMP_12298(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_5032(bool) -> $_1 (-> ['TMP_12298'])._initializing
TMP_12299 = UnaryType.BANG REF_5032 
isTopLevelCall_1(bool) := TMP_12299(bool)
 initialized = $._initialized
REF_5033(uint64) -> $_1 (-> ['TMP_12298'])._initialized
initialized_1(uint64) := REF_5033(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_12300(bool) = initialized_1 == 0
TMP_12301(bool) = TMP_12300 && isTopLevelCall_1
initialSetup_1(bool) := TMP_12301(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_12302(bool) = initialized_1 == 1
TMP_12303 = CONVERT this to address
TMP_12304(bytes) = SOLIDITY_CALL code(address)(TMP_12303)
REF_5034 -> LENGTH TMP_12304
TMP_12305(bool) = REF_5034 == 0
TMP_12306(bool) = TMP_12302 && TMP_12305
construction_1(bool) := TMP_12306(bool)
 ! initialSetup && ! construction
TMP_12307 = UnaryType.BANG initialSetup_1 
TMP_12308 = UnaryType.BANG construction_1 
TMP_12309(bool) = TMP_12307 && TMP_12308
CONDITION TMP_12309
 revert InvalidInitialization()()
TMP_12310(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_5035(uint64) -> $_1 (-> ['TMP_12298'])._initialized
$_2 (-> ['TMP_12298'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_12298'])"])
REF_5035(uint64) (->$_2 (-> ['TMP_12298'])) := 1(uint256)
TMP_12298(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12298'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_5036(bool) -> $_2 (-> ['TMP_12298'])._initializing
$_3 (-> ['TMP_12298'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12298'])"])
REF_5036(bool) (->$_3 (-> ['TMP_12298'])) := True(bool)
TMP_12298(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12298'])"])
$_4 (-> ['TMP_12298'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12298'])", "$_2 (-> ['TMP_12298'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_5037(bool) -> $_4 (-> ['TMP_12298'])._initializing
$_5 (-> ['TMP_12298'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_12298'])"])
REF_5037(bool) (->$_5 (-> ['TMP_12298'])) := False(bool)
TMP_12298(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_12298'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_12312(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_12312'])(Initializable.InitializableStorage) := TMP_12312(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_5038(bool) -> $_1 (-> ['TMP_12312'])._initializing
REF_5039(uint64) -> $_1 (-> ['TMP_12312'])._initialized
TMP_12313(bool) = REF_5039 >= version_1
TMP_12314(bool) = REF_5038 || TMP_12313
CONDITION TMP_12314
 revert InvalidInitialization()()
TMP_12315(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_5040(uint64) -> $_1 (-> ['TMP_12312'])._initialized
$_2 (-> ['TMP_12312'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_12312'])"])
REF_5040(uint64) (->$_2 (-> ['TMP_12312'])) := version_1(uint64)
TMP_12312(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12312'])"])
 $._initializing = true
REF_5041(bool) -> $_2 (-> ['TMP_12312'])._initializing
$_3 (-> ['TMP_12312'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12312'])"])
REF_5041(bool) (->$_3 (-> ['TMP_12312'])) := True(bool)
TMP_12312(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12312'])"])
 $._initializing = false
REF_5042(bool) -> $_3 (-> ['TMP_12312'])._initializing
$_4 (-> ['TMP_12312'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12312'])"])
REF_5042(bool) (->$_4 (-> ['TMP_12312'])) := False(bool)
TMP_12312(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_12312'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
role_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_19', 'DEFAULT_ADMIN_ROLE_13', 'TMP_12156', 'TMP_12159', 'DEFAULT_ADMIN_ROLE_15', 'DEFAULT_ADMIN_ROLE_5', 'DEFAULT_ADMIN_ROLE_9', 'DEFAULT_ADMIN_ROLE_7', 'DEFAULT_ADMIN_ROLE_21', 'DEFAULT_ADMIN_ROLE_25', 'TMP_12161', 'TMP_12154', 'DEFAULT_ADMIN_ROLE_11', 'DEFAULT_ADMIN_ROLE_23', 'DEFAULT_ADMIN_ROLE_17'])
 _checkRole(role)
INTERNAL_CALL, AccessControl._checkRole(bytes32)(role_1)
gov_1(address) := phi(['gov_0'])
 require(bool,string)(msg.sender == gov,Only DAO can execute proposal)
TMP_12319(bool) = msg.sender == gov_1
TMP_12320(None) = SOLIDITY_CALL require(bool,string)(TMP_12319,Only DAO can execute proposal)
locked_1(bool) := phi(['locked_0', 'locked_3'])
 require(bool,string)(! locked,cannot reenter)
TMP_12321 = UnaryType.BANG locked_1 
TMP_12322(None) = SOLIDITY_CALL require(bool,string)(TMP_12321,cannot reenter)
 locked = true
locked_2(bool) := True(bool)
 locked = false
locked_3(bool) := False(bool)
```
#### AgentFactoryV3.totalAgents() [PUBLIC]
```slithir
allTokens_5(address[]) := phi(['allTokens_5', 'allTokens_0', 'allTokens_4'])
 allTokens.length
REF_5140 -> LENGTH allTokens_5
RETURN REF_5140
```
#### AgentFactoryV3.unpause() [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_25(bytes32) := phi(['DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_12'])
 _unpause()
INTERNAL_CALL, PausableUpgradeable._unpause()()
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_25)
```
#### AgentFactoryV3.withdraw(uint256) [PUBLIC]
```slithir
assetToken_7(address) := phi(['assetToken_15', 'assetToken_14', 'assetToken_1', 'assetToken_21', 'assetToken_0', 'assetToken_6', 'assetToken_12', 'assetToken_9'])
WITHDRAW_ROLE_1(bytes32) := phi(['WITHDRAW_ROLE_6', 'WITHDRAW_ROLE_3', 'WITHDRAW_ROLE_0'])
_applications_3(mapping(uint256 => AgentFactoryV3.Application)) := phi(['_applications_10', '_applications_6', '_applications_2', '_applications_1', '_applications_17', '_applications_12', '_applications_13', '_applications_0'])
 application = _applications[id]
REF_5074(AgentFactoryV3.Application) -> _applications_4[id_1]
application_1 (-> ['_applications'])(AgentFactoryV3.Application) := REF_5074(AgentFactoryV3.Application)
 require(bool,string)(msg.sender == application.proposer || hasRole(WITHDRAW_ROLE,msg.sender),Not proposer)
REF_5075(address) -> application_1 (-> ['_applications']).proposer
TMP_12415(bool) = msg.sender == REF_5075
TMP_12416(bool) = INTERNAL_CALL, AccessControl.hasRole(bytes32,address)(WITHDRAW_ROLE_2,msg.sender)
TMP_12417(bool) = TMP_12415 || TMP_12416
TMP_12418(None) = SOLIDITY_CALL require(bool,string)(TMP_12417,Not proposer)
 require(bool,string)(application.status == ApplicationStatus.Active,Application is not active)
REF_5076(AgentFactoryV3.ApplicationStatus) -> application_1 (-> ['_applications']).status
REF_5077(AgentFactoryV3.ApplicationStatus) -> ApplicationStatus.Active
TMP_12419(bool) = REF_5076 == REF_5077
TMP_12420(None) = SOLIDITY_CALL require(bool,string)(TMP_12419,Application is not active)
 require(bool,string)(block.number > application.proposalEndBlock,Application is not matured yet)
REF_5078(uint256) -> application_1 (-> ['_applications']).proposalEndBlock
TMP_12421(bool) = block.number > REF_5078
TMP_12422(None) = SOLIDITY_CALL require(bool,string)(TMP_12421,Application is not matured yet)
 withdrawableAmount = application.withdrawableAmount
REF_5079(uint256) -> application_1 (-> ['_applications']).withdrawableAmount
withdrawableAmount_1(uint256) := REF_5079(uint256)
 application.withdrawableAmount = 0
REF_5080(uint256) -> application_1 (-> ['_applications']).withdrawableAmount
application_2 (-> ['_applications'])(AgentFactoryV3.Application) := phi(["application_1 (-> ['_applications'])"])
REF_5080(uint256) (->application_2 (-> ['_applications'])) := 0(uint256)
_applications_5(mapping(uint256 => AgentFactoryV3.Application)) := phi(["application_2 (-> ['_applications'])"])
 application.status = ApplicationStatus.Withdrawn
REF_5081(AgentFactoryV3.ApplicationStatus) -> application_2 (-> ['_applications']).status
REF_5082(AgentFactoryV3.ApplicationStatus) -> ApplicationStatus.Withdrawn
application_3 (-> ['_applications'])(AgentFactoryV3.Application) := phi(["application_2 (-> ['_applications'])"])
REF_5081(AgentFactoryV3.ApplicationStatus) (->application_3 (-> ['_applications'])) := REF_5082(AgentFactoryV3.ApplicationStatus)
_applications_6(mapping(uint256 => AgentFactoryV3.Application)) := phi(["application_3 (-> ['_applications'])"])
 IERC20(assetToken).safeTransfer(application.proposer,withdrawableAmount)
TMP_12423 = CONVERT assetToken_9 to IERC20
REF_5084(address) -> application_3 (-> ['_applications']).proposer
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_12423', 'REF_5084', 'withdrawableAmount_1'] 
 noReentrant()
MODIFIER_CALL, AgentFactoryV3.noReentrant()()
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
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_2044(transfer) -> token_1.transfer
TMP_5413(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2044,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000550>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001210>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5413)
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
#### IAgentNft.virtualInfo(uint256) [EXTERNAL]
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
