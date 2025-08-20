




### Storage layout (AgentFactoryV4) 

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
_applications mapping(uint256 => AgentFactoryV4.Application)
_vault address
locked bool
allTradingTokens address[]
_uniswapRouter address
veTokenImplementation address
_tokenAdmin address
defaultDelegatee address
_tokenSupplyParams bytes
_tokenTaxParams bytes
_tokenApplication mapping(address => uint256)
_applicationToken mapping(uint256 => address)

```





#### AgentFactoryV4._createNewAgentToken(string,string,bytes) [INTERNAL]
```slithir
name_1(string) := phi(['REF_5224'])
symbol_1(string) := phi(['REF_5225'])
tokenSupplyParams__1(bytes) := phi(['tokenSupplyParams__1'])
tokenImplementation_2(address) := phi(['tokenImplementation_3', 'tokenImplementation_1', 'tokenImplementation_0'])
assetToken_16(address) := phi(['assetToken_24', 'assetToken_0', 'assetToken_1', 'assetToken_9', 'assetToken_15', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12'])
allTradingTokens_1(address[]) := phi(['allTradingTokens_4', 'allTradingTokens_0'])
_uniswapRouter_5(address) := phi(['_uniswapRouter_0', '_uniswapRouter_9', '_uniswapRouter_7', '_uniswapRouter_4', '_uniswapRouter_6'])
_tokenAdmin_2(address) := phi(['_tokenAdmin_4', '_tokenAdmin_3', '_tokenAdmin_0'])
_tokenTaxParams_1(bytes) := phi(['_tokenTaxParams_0', '_tokenTaxParams_2', '_tokenTaxParams_3'])
 instance = Clones.clone(tokenImplementation)
TMP_12744(address) = LIBRARY_CALL, dest:Clones, function:Clones.clone(address), arguments:['tokenImplementation_2'] 
instance_1(address) := TMP_12744(address)
 IAgentToken(instance).initialize((_tokenAdmin,_uniswapRouter,assetToken),abi.encode(name,symbol),tokenSupplyParams_,_tokenTaxParams)
TMP_12745 = CONVERT instance_1 to IAgentToken
TMP_12746(bytes) = SOLIDITY_CALL abi.encode()(name_1,symbol_1)
HIGH_LEVEL_CALL, dest:TMP_12745(IAgentToken), function:initialize, arguments:['[<slither.slithir.variables.state_variable.StateIRVariable object at 0xffff749f5cc0>, <slither.slithir.variables.state_variable.StateIRVariable object at 0xffff749f5c60>, <slither.slithir.variables.state_variable.StateIRVariable object at 0xffff749f53f0>]', 'TMP_12746', 'tokenSupplyParams__1', '_tokenTaxParams_1']  
assetToken_17(address) := phi(['assetToken_24', 'assetToken_1', 'assetToken_15', 'assetToken_9', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12', 'assetToken_16'])
allTradingTokens_2(address[]) := phi(['allTradingTokens_4', 'allTradingTokens_1'])
_uniswapRouter_6(address) := phi(['_uniswapRouter_5', '_uniswapRouter_9', '_uniswapRouter_7', '_uniswapRouter_4', '_uniswapRouter_6'])
_tokenAdmin_3(address) := phi(['_tokenAdmin_4', '_tokenAdmin_3', '_tokenAdmin_2'])
_tokenTaxParams_2(bytes) := phi(['_tokenTaxParams_2', '_tokenTaxParams_3', '_tokenTaxParams_1'])
 allTradingTokens.push(instance)
REF_5268 -> LENGTH allTradingTokens_2
TMP_12749(uint256) := REF_5268(uint256)
TMP_12750(uint256) = TMP_12749 (c)+ 1
allTradingTokens_3(address[]) := phi(['allTradingTokens_2'])
REF_5268(uint256) (->allTradingTokens_3) := TMP_12750(uint256)
REF_5269(address) -> allTradingTokens_3[TMP_12749]
allTradingTokens_4(address[]) := phi(['allTradingTokens_3'])
REF_5269(address) (->allTradingTokens_4) := instance_1(address)
 instance
RETURN instance_1
 instance
```
#### AgentFactoryV4._createNewAgentVeToken(string,string,address,address,bool) [INTERNAL]
```slithir
name_1(string) := phi(['TMP_12706'])
symbol_1(string) := phi(['TMP_12707'])
stakingAsset_1(address) := phi(['lp_4'])
founder_1(address) := phi(['REF_5238'])
canStake_1(bool) := phi(['canStake_1'])
nft_17(address) := phi(['nft_16', 'nft_0', 'nft_14', 'nft_18', 'nft_1'])
allTokens_1(address[]) := phi(['allTokens_5', 'allTokens_0', 'allTokens_4'])
maturityDuration_1(uint256) := phi(['maturityDuration_3', 'maturityDuration_2', 'maturityDuration_0'])
veTokenImplementation_2(address) := phi(['veTokenImplementation_3', 'veTokenImplementation_1', 'veTokenImplementation_0'])
 instance = Clones.clone(veTokenImplementation)
TMP_12751(address) = LIBRARY_CALL, dest:Clones, function:Clones.clone(address), arguments:['veTokenImplementation_2'] 
instance_1(address) := TMP_12751(address)
 IAgentVeToken(instance).initialize(name,symbol,founder,stakingAsset,block.timestamp + maturityDuration,address(nft),canStake)
TMP_12752 = CONVERT instance_1 to IAgentVeToken
TMP_12753(uint256) = block.timestamp (c)+ maturityDuration_1
TMP_12754 = CONVERT nft_17 to address
HIGH_LEVEL_CALL, dest:TMP_12752(IAgentVeToken), function:initialize, arguments:['name_1', 'symbol_1', 'founder_1', 'stakingAsset_1', 'TMP_12753', 'TMP_12754', 'canStake_1']  
nft_18(address) := phi(['nft_16', 'nft_14', 'nft_18', 'nft_17', 'nft_1'])
allTokens_2(address[]) := phi(['allTokens_1', 'allTokens_5', 'allTokens_4'])
maturityDuration_2(uint256) := phi(['maturityDuration_3', 'maturityDuration_1', 'maturityDuration_2'])
 allTokens.push(instance)
REF_5273 -> LENGTH allTokens_2
TMP_12757(uint256) := REF_5273(uint256)
TMP_12758(uint256) = TMP_12757 (c)+ 1
allTokens_3(address[]) := phi(['allTokens_2'])
REF_5273(uint256) (->allTokens_3) := TMP_12758(uint256)
REF_5274(address) -> allTokens_3[TMP_12757]
allTokens_4(address[]) := phi(['allTokens_3'])
REF_5274(address) (->allTokens_4) := instance_1(address)
 instance
RETURN instance_1
 instance
```
#### AgentFactoryV4._createNewDAO(string,IVotes,uint32,uint256) [INTERNAL]
```slithir
name_1(string) := phi(['daoName_1'])
token_1(IVotes) := phi(['TMP_12710'])
daoVotingPeriod_1(uint32) := phi(['REF_5241'])
daoThreshold_1(uint256) := phi(['REF_5242'])
daoImplementation_2(address) := phi(['daoImplementation_3', 'daoImplementation_0', 'daoImplementation_1'])
nft_15(address) := phi(['nft_16', 'nft_0', 'nft_14', 'nft_18', 'nft_1'])
allDAOs_1(address[]) := phi(['allDAOs_0', 'allDAOs_4'])
 instance = Clones.clone(daoImplementation)
TMP_12738(address) = LIBRARY_CALL, dest:Clones, function:Clones.clone(address), arguments:['daoImplementation_2'] 
instance_1(address) := TMP_12738(address)
 IAgentDAO(instance).initialize(name,token,nft,daoThreshold,daoVotingPeriod)
TMP_12739 = CONVERT instance_1 to IAgentDAO
HIGH_LEVEL_CALL, dest:TMP_12739(IAgentDAO), function:initialize, arguments:['name_1', 'token_1', 'nft_15', 'daoThreshold_1', 'daoVotingPeriod_1']  
nft_16(address) := phi(['nft_16', 'nft_15', 'nft_14', 'nft_18', 'nft_1'])
allDAOs_2(address[]) := phi(['allDAOs_1', 'allDAOs_4'])
 allDAOs.push(instance)
REF_5262 -> LENGTH allDAOs_2
TMP_12742(uint256) := REF_5262(uint256)
TMP_12743(uint256) = TMP_12742 (c)+ 1
allDAOs_3(address[]) := phi(['allDAOs_2'])
REF_5262(uint256) (->allDAOs_3) := TMP_12743(uint256)
REF_5263(address) -> allDAOs_3[TMP_12742]
allDAOs_4(address[]) := phi(['allDAOs_3'])
REF_5263(address) (->allDAOs_4) := instance_1(address)
 instance
RETURN instance_1
 instance
```
#### AgentFactoryV4._createPair(address) [INTERNAL]
```slithir
tokenAddr_1(address) := phi(['token_1'])
assetToken_25(address) := phi(['assetToken_24', 'assetToken_0', 'assetToken_1', 'assetToken_9', 'assetToken_15', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12'])
_uniswapRouter_8(address) := phi(['_uniswapRouter_0', '_uniswapRouter_9', '_uniswapRouter_7', '_uniswapRouter_4', '_uniswapRouter_6'])
 factory = IUniswapV2Factory(IUniswapV2Router02(_uniswapRouter).factory())
TMP_12828 = CONVERT _uniswapRouter_8 to IUniswapV2Router02
TMP_12829(address) = HIGH_LEVEL_CALL, dest:TMP_12828(IUniswapV2Router02), function:factory, arguments:[]  
assetToken_26(address) := phi(['assetToken_25', 'assetToken_24', 'assetToken_1', 'assetToken_15', 'assetToken_9', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12'])
_uniswapRouter_9(address) := phi(['_uniswapRouter_8', '_uniswapRouter_9', '_uniswapRouter_7', '_uniswapRouter_4', '_uniswapRouter_6'])
TMP_12830 = CONVERT TMP_12829 to IUniswapV2Factory
factory_1(IUniswapV2Factory) := TMP_12830(IUniswapV2Factory)
 require(bool,string)(factory.getPair(tokenAddr,assetToken) == address(0),pool already exists)
TMP_12831(address) = HIGH_LEVEL_CALL, dest:factory_1(IUniswapV2Factory), function:getPair, arguments:['tokenAddr_1', 'assetToken_26']  
assetToken_27(address) := phi(['assetToken_24', 'assetToken_1', 'assetToken_26', 'assetToken_15', 'assetToken_9', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12'])
TMP_12832 = CONVERT 0 to address
TMP_12833(bool) = TMP_12831 == TMP_12832
TMP_12834(None) = SOLIDITY_CALL require(bool,string)(TMP_12833,pool already exists)
 uniswapV2Pair_ = factory.createPair(tokenAddr,assetToken)
TMP_12835(address) = HIGH_LEVEL_CALL, dest:factory_1(IUniswapV2Factory), function:createPair, arguments:['tokenAddr_1', 'assetToken_27']  
assetToken_28(address) := phi(['assetToken_24', 'assetToken_1', 'assetToken_15', 'assetToken_9', 'assetToken_17', 'assetToken_18', 'assetToken_27', 'assetToken_28', 'assetToken_6', 'assetToken_12'])
uniswapV2Pair__1(address) := TMP_12835(address)
 (uniswapV2Pair_)
RETURN uniswapV2Pair__1
 uniswapV2Pair_
```
#### AgentFactoryV4._executeApplication(uint256,bool,bytes) [INTERNAL]
```slithir
id_1(uint256) := phi(['id_1', 'id_1'])
canStake_1(bool) := phi(['canStake_1', 'canStake_1'])
tokenSupplyParams__1(bytes) := phi(['_tokenSupplyParams_3', '_tokenSupplyParams_8'])
nft_2(address) := phi(['nft_16', 'nft_0', 'nft_14', 'nft_18', 'nft_1'])
tbaRegistry_2(address) := phi(['tbaRegistry_0', 'tbaRegistry_13', 'tbaRegistry_1'])
assetToken_10(address) := phi(['assetToken_24', 'assetToken_0', 'assetToken_1', 'assetToken_9', 'assetToken_15', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12'])
_applications_7(mapping(uint256 => AgentFactoryV4.Application)) := phi(['_applications_13', '_applications_0', '_applications_2', '_applications_10', '_applications_6', '_applications_1', '_applications_15', '_applications_12'])
_vault_2(address) := phi(['_vault_0', '_vault_1', '_vault_12', '_vault_13'])
_uniswapRouter_1(address) := phi(['_uniswapRouter_0', '_uniswapRouter_9', '_uniswapRouter_7', '_uniswapRouter_4', '_uniswapRouter_6'])
_tokenAdmin_1(address) := phi(['_tokenAdmin_4', '_tokenAdmin_3', '_tokenAdmin_0'])
defaultDelegatee_1(address) := phi(['defaultDelegatee_0', 'defaultDelegatee_17', 'defaultDelegatee_16'])
_applicationToken_5(mapping(uint256 => address)) := phi(['_applicationToken_0', '_applicationToken_9', '_applicationToken_6', '_applicationToken_4', '_applicationToken_5', '_applicationToken_3'])
 require(bool,string)(_applications[id].status == ApplicationStatus.Active,Application is not active)
REF_5215(AgentFactoryV4.Application) -> _applications_7[id_1]
REF_5216(AgentFactoryV4.ApplicationStatus) -> REF_5215.status
REF_5217(AgentFactoryV4.ApplicationStatus) -> ApplicationStatus.Active
TMP_12678(bool) = REF_5216 == REF_5217
TMP_12679(None) = SOLIDITY_CALL require(bool,string)(TMP_12678,Application is not active)
 require(bool,string)(_tokenAdmin != address(0),Token admin not set)
TMP_12680 = CONVERT 0 to address
TMP_12681(bool) = _tokenAdmin_1 != TMP_12680
TMP_12682(None) = SOLIDITY_CALL require(bool,string)(TMP_12681,Token admin not set)
 application = _applications[id]
REF_5218(AgentFactoryV4.Application) -> _applications_7[id_1]
application_1 (-> ['_applications'])(AgentFactoryV4.Application) := REF_5218(AgentFactoryV4.Application)
 initialAmount = application.withdrawableAmount
REF_5219(uint256) -> application_1 (-> ['_applications']).withdrawableAmount
initialAmount_1(uint256) := REF_5219(uint256)
 application.withdrawableAmount = 0
REF_5220(uint256) -> application_1 (-> ['_applications']).withdrawableAmount
application_2 (-> ['_applications'])(AgentFactoryV4.Application) := phi(["application_1 (-> ['_applications'])"])
REF_5220(uint256) (->application_2 (-> ['_applications'])) := 0(uint256)
_applications_8(mapping(uint256 => AgentFactoryV4.Application)) := phi(["application_2 (-> ['_applications'])"])
 application.status = ApplicationStatus.Executed
REF_5221(AgentFactoryV4.ApplicationStatus) -> application_2 (-> ['_applications']).status
REF_5222(AgentFactoryV4.ApplicationStatus) -> ApplicationStatus.Executed
application_3 (-> ['_applications'])(AgentFactoryV4.Application) := phi(["application_2 (-> ['_applications'])"])
REF_5221(AgentFactoryV4.ApplicationStatus) (->application_3 (-> ['_applications'])) := REF_5222(AgentFactoryV4.ApplicationStatus)
_applications_9(mapping(uint256 => AgentFactoryV4.Application)) := phi(["application_3 (-> ['_applications'])"])
 token = _applicationToken[id]
REF_5223(address) -> _applicationToken_5[id_1]
token_1(address) := REF_5223(address)
 lp = address(0)
TMP_12683 = CONVERT 0 to address
lp_1(address) := TMP_12683(address)
 token == address(0)
TMP_12684 = CONVERT 0 to address
TMP_12685(bool) = token_1 == TMP_12684
CONDITION TMP_12685
 token = _createNewAgentToken(application.name,application.symbol,tokenSupplyParams_)
REF_5224(string) -> application_3 (-> ['_applications']).name
REF_5225(string) -> application_3 (-> ['_applications']).symbol
TMP_12686(address) = INTERNAL_CALL, AgentFactoryV4._createNewAgentToken(string,string,bytes)(REF_5224,REF_5225,tokenSupplyParams__1)
assetToken_11(address) := phi(['assetToken_17'])
token_2(address) := TMP_12686(address)
 lp = IAgentToken(token).liquidityPools()[0]
TMP_12687 = CONVERT token_2 to IAgentToken
TMP_12688(address[]) = HIGH_LEVEL_CALL, dest:TMP_12687(IAgentToken), function:liquidityPools, arguments:[]  
nft_4(address) := phi(['nft_16', 'nft_14', 'nft_18', 'nft_3', 'nft_1'])
tbaRegistry_4(address) := phi(['tbaRegistry_13', 'tbaRegistry_3', 'tbaRegistry_1'])
assetToken_12(address) := phi(['assetToken_24', 'assetToken_11', 'assetToken_1', 'assetToken_15', 'assetToken_9', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12'])
_vault_4(address) := phi(['_vault_1', '_vault_3', '_vault_12', '_vault_13'])
defaultDelegatee_3(address) := phi(['defaultDelegatee_17', 'defaultDelegatee_2', 'defaultDelegatee_16'])
REF_5227(address) -> TMP_12688[0]
lp_2(address) := REF_5227(address)
 IERC20(assetToken).safeTransfer(token,initialAmount)
TMP_12689 = CONVERT assetToken_12 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_12689', 'token_2', 'initialAmount_1'] 
 IAgentToken(token).addInitialLiquidity(address(this))
TMP_12691 = CONVERT token_2 to IAgentToken
TMP_12692 = CONVERT this to address
HIGH_LEVEL_CALL, dest:TMP_12691(IAgentToken), function:addInitialLiquidity, arguments:['TMP_12692']  
nft_5(address) := phi(['nft_16', 'nft_4', 'nft_14', 'nft_18', 'nft_1'])
tbaRegistry_5(address) := phi(['tbaRegistry_13', 'tbaRegistry_4', 'tbaRegistry_1'])
_vault_5(address) := phi(['_vault_1', '_vault_4', '_vault_12', '_vault_13'])
defaultDelegatee_4(address) := phi(['defaultDelegatee_3', 'defaultDelegatee_17', 'defaultDelegatee_16'])
 lp = _createPair(token)
TMP_12694(address) = INTERNAL_CALL, AgentFactoryV4._createPair(address)(token_1)
assetToken_13(address) := phi(['assetToken_28'])
_uniswapRouter_2(address) := phi(['_uniswapRouter_9'])
lp_3(address) := TMP_12694(address)
 IERC20(token).forceApprove(_uniswapRouter,type()(uint256).max)
TMP_12695 = CONVERT token_1 to IERC20
TMP_12697(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.forceApprove(IERC20,address,uint256), arguments:['TMP_12695', '_uniswapRouter_2', 'TMP_12697'] 
 IERC20(assetToken).forceApprove(_uniswapRouter,initialAmount)
TMP_12699 = CONVERT assetToken_13 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.forceApprove(IERC20,address,uint256), arguments:['TMP_12699', '_uniswapRouter_2', 'initialAmount_1'] 
 IUniswapV2Router02(_uniswapRouter).addLiquidity(token,assetToken,IERC20(token).balanceOf(address(this)),initialAmount,0,0,address(this),block.timestamp)
TMP_12701 = CONVERT _uniswapRouter_2 to IUniswapV2Router02
TMP_12702 = CONVERT token_1 to IERC20
TMP_12703 = CONVERT this to address
TMP_12704(uint256) = HIGH_LEVEL_CALL, dest:TMP_12702(IERC20), function:balanceOf, arguments:['TMP_12703']  
nft_7(address) := phi(['nft_6', 'nft_16', 'nft_14', 'nft_18', 'nft_1'])
tbaRegistry_7(address) := phi(['tbaRegistry_13', 'tbaRegistry_6', 'tbaRegistry_1'])
assetToken_14(address) := phi(['assetToken_24', 'assetToken_1', 'assetToken_15', 'assetToken_9', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12', 'assetToken_13'])
_vault_7(address) := phi(['_vault_1', '_vault_6', '_vault_12', '_vault_13'])
_uniswapRouter_3(address) := phi(['_uniswapRouter_2', '_uniswapRouter_9', '_uniswapRouter_7', '_uniswapRouter_4', '_uniswapRouter_6'])
defaultDelegatee_6(address) := phi(['defaultDelegatee_17', 'defaultDelegatee_5', 'defaultDelegatee_16'])
TMP_12705 = CONVERT this to address
TUPLE_125(uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:TMP_12701(IUniswapV2Router02), function:addLiquidity, arguments:['token_1', 'assetToken_14', 'TMP_12704', 'initialAmount_1', '0', '0', 'TMP_12705', 'block.timestamp']  
nft_8(address) := phi(['nft_16', 'nft_7', 'nft_14', 'nft_18', 'nft_1'])
tbaRegistry_8(address) := phi(['tbaRegistry_13', 'tbaRegistry_7', 'tbaRegistry_1'])
assetToken_15(address) := phi(['assetToken_24', 'assetToken_1', 'assetToken_15', 'assetToken_9', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12', 'assetToken_14'])
_vault_8(address) := phi(['_vault_7', '_vault_1', '_vault_12', '_vault_13'])
_uniswapRouter_4(address) := phi(['_uniswapRouter_3', '_uniswapRouter_9', '_uniswapRouter_7', '_uniswapRouter_4', '_uniswapRouter_6'])
defaultDelegatee_7(address) := phi(['defaultDelegatee_6', 'defaultDelegatee_17', 'defaultDelegatee_16'])
token_3(address) := phi(['token_2', 'token_1'])
lp_4(address) := phi(['lp_2', 'lp_3'])
 veToken = _createNewAgentVeToken(string.concat(Staked ,application.name),string.concat(s,application.symbol),lp,application.proposer,canStake)
REF_5235(string) -> application_3 (-> ['_applications']).name
TMP_12706(string) = SOLIDITY_CALL string.concat()(Staked ,REF_5235)
REF_5237(string) -> application_3 (-> ['_applications']).symbol
TMP_12707(string) = SOLIDITY_CALL string.concat()(s,REF_5237)
REF_5238(address) -> application_3 (-> ['_applications']).proposer
TMP_12708(address) = INTERNAL_CALL, AgentFactoryV4._createNewAgentVeToken(string,string,address,address,bool)(TMP_12706,TMP_12707,lp_4,REF_5238,canStake_1)
nft_9(address) := phi(['nft_18'])
veToken_1(address) := TMP_12708(address)
 daoName = string.concat(application.name, DAO)
REF_5240(string) -> application_3 (-> ['_applications']).name
TMP_12709(string) = SOLIDITY_CALL string.concat()(REF_5240, DAO)
daoName_1(string) := TMP_12709(string)
 dao = address(_createNewDAO(daoName,IVotes(veToken),application.daoVotingPeriod,application.daoThreshold))
TMP_12710 = CONVERT veToken_1 to IVotes
REF_5241(uint32) -> application_3 (-> ['_applications']).daoVotingPeriod
REF_5242(uint256) -> application_3 (-> ['_applications']).daoThreshold
TMP_12711(address) = INTERNAL_CALL, AgentFactoryV4._createNewDAO(string,IVotes,uint32,uint256)(daoName_1,TMP_12710,REF_5241,REF_5242)
nft_10(address) := phi(['nft_16'])
TMP_12712 = CONVERT TMP_12711 to address
dao_1(address) := TMP_12712(address)
 virtualId = IAgentNft(nft).nextVirtualId()
TMP_12713 = CONVERT nft_10 to IAgentNft
TMP_12714(uint256) = HIGH_LEVEL_CALL, dest:TMP_12713(IAgentNft), function:nextVirtualId, arguments:[]  
nft_11(address) := phi(['nft_1', 'nft_16', 'nft_14', 'nft_18', 'nft_10'])
tbaRegistry_11(address) := phi(['tbaRegistry_13', 'tbaRegistry_10', 'tbaRegistry_1'])
_vault_11(address) := phi(['_vault_1', '_vault_10', '_vault_12', '_vault_13'])
defaultDelegatee_10(address) := phi(['defaultDelegatee_9', 'defaultDelegatee_17', 'defaultDelegatee_16'])
virtualId_1(uint256) := TMP_12714(uint256)
 IAgentNft(nft).mint(virtualId,_vault,application.tokenURI,dao,application.proposer,application.cores,lp,token)
TMP_12715 = CONVERT nft_11 to IAgentNft
REF_5245(string) -> application_3 (-> ['_applications']).tokenURI
REF_5246(address) -> application_3 (-> ['_applications']).proposer
REF_5247(uint8[]) -> application_3 (-> ['_applications']).cores
TMP_12716(uint256) = HIGH_LEVEL_CALL, dest:TMP_12715(IAgentNft), function:mint, arguments:['virtualId_1', '_vault_11', 'REF_5245', 'dao_1', 'REF_5246', 'REF_5247', 'lp_4', 'token_3']  
nft_12(address) := phi(['nft_16', 'nft_11', 'nft_14', 'nft_18', 'nft_1'])
tbaRegistry_12(address) := phi(['tbaRegistry_13', 'tbaRegistry_11', 'tbaRegistry_1'])
_vault_12(address) := phi(['_vault_1', '_vault_12', '_vault_11', '_vault_13'])
defaultDelegatee_11(address) := phi(['defaultDelegatee_17', 'defaultDelegatee_10', 'defaultDelegatee_16'])
 application.virtualId = virtualId
REF_5248(uint256) -> application_3 (-> ['_applications']).virtualId
application_4 (-> ['_applications'])(AgentFactoryV4.Application) := phi(["application_3 (-> ['_applications'])"])
REF_5248(uint256) (->application_4 (-> ['_applications'])) := virtualId_1(uint256)
_applications_10(mapping(uint256 => AgentFactoryV4.Application)) := phi(["application_4 (-> ['_applications'])"])
 chainId = chainid()()
TMP_12717(uint256) = SOLIDITY_CALL chainid()()
chainId_1(uint256) := TMP_12717(uint256)
 tbaAddress = IERC6551Registry(tbaRegistry).createAccount(application.tbaImplementation,application.tbaSalt,chainId,nft,virtualId)
TMP_12718 = CONVERT tbaRegistry_12 to IERC6551Registry
REF_5250(address) -> application_4 (-> ['_applications']).tbaImplementation
REF_5251(bytes32) -> application_4 (-> ['_applications']).tbaSalt
TMP_12719(address) = HIGH_LEVEL_CALL, dest:TMP_12718(IERC6551Registry), function:createAccount, arguments:['REF_5250', 'REF_5251', 'chainId_1', 'nft_12', 'virtualId_1']  
nft_13(address) := phi(['nft_16', 'nft_12', 'nft_14', 'nft_18', 'nft_1'])
tbaRegistry_13(address) := phi(['tbaRegistry_13', 'tbaRegistry_1', 'tbaRegistry_12'])
defaultDelegatee_12(address) := phi(['defaultDelegatee_17', 'defaultDelegatee_11', 'defaultDelegatee_16'])
tbaAddress_1(address) := TMP_12719(address)
 IAgentNft(nft).setTBA(virtualId,tbaAddress)
TMP_12720 = CONVERT nft_13 to IAgentNft
HIGH_LEVEL_CALL, dest:TMP_12720(IAgentNft), function:setTBA, arguments:['virtualId_1', 'tbaAddress_1']  
nft_14(address) := phi(['nft_16', 'nft_13', 'nft_14', 'nft_18', 'nft_1'])
defaultDelegatee_13(address) := phi(['defaultDelegatee_17', 'defaultDelegatee_12', 'defaultDelegatee_16'])
 IERC20(lp).approve(veToken,type()(uint256).max)
TMP_12722 = CONVERT lp_4 to IERC20
TMP_12724(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_12725(bool) = HIGH_LEVEL_CALL, dest:TMP_12722(IERC20), function:approve, arguments:['veToken_1', 'TMP_12724']  
defaultDelegatee_14(address) := phi(['defaultDelegatee_17', 'defaultDelegatee_13', 'defaultDelegatee_16'])
 IAgentVeToken(veToken).stake(IERC20(lp).balanceOf(address(this)),application.proposer,defaultDelegatee)
TMP_12726 = CONVERT veToken_1 to IAgentVeToken
TMP_12727 = CONVERT lp_4 to IERC20
TMP_12728 = CONVERT this to address
TMP_12729(uint256) = HIGH_LEVEL_CALL, dest:TMP_12727(IERC20), function:balanceOf, arguments:['TMP_12728']  
defaultDelegatee_15(address) := phi(['defaultDelegatee_14', 'defaultDelegatee_17', 'defaultDelegatee_16'])
REF_5256(address) -> application_4 (-> ['_applications']).proposer
HIGH_LEVEL_CALL, dest:TMP_12726(IAgentVeToken), function:stake, arguments:['TMP_12729', 'REF_5256', 'defaultDelegatee_15']  
defaultDelegatee_16(address) := phi(['defaultDelegatee_15', 'defaultDelegatee_17', 'defaultDelegatee_16'])
 NewPersona(virtualId,token,dao,tbaAddress,veToken,lp)
Emit NewPersona(virtualId_1,token_3,dao_1,tbaAddress_1,veToken_1,lp_4)
```
#### AgentFactoryV4._msgData() [INTERNAL]
```slithir
 ContextUpgradeable._msgData()
TMP_12776(bytes) = INTERNAL_CALL, ContextUpgradeable._msgData()()
RETURN TMP_12776
```
#### AgentFactoryV4._msgSender() [INTERNAL]
```slithir
 sender = ContextUpgradeable._msgSender()
TMP_12775(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
sender_1(address) := TMP_12775(address)
 sender
RETURN sender_1
```
#### AgentFactoryV4.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### AgentFactoryV4.executeApplication(uint256,bool) [PUBLIC]
```slithir
WITHDRAW_ROLE_4(bytes32) := phi(['WITHDRAW_ROLE_6', 'WITHDRAW_ROLE_0', 'WITHDRAW_ROLE_3', 'WITHDRAW_ROLE_9'])
_applications_11(mapping(uint256 => AgentFactoryV4.Application)) := phi(['_applications_13', '_applications_0', '_applications_2', '_applications_10', '_applications_6', '_applications_1', '_applications_15', '_applications_12'])
_tokenSupplyParams_1(bytes) := phi(['_tokenSupplyParams_9', '_tokenSupplyParams_4', '_tokenSupplyParams_5', '_tokenSupplyParams_0'])
 application = _applications[id]
REF_5257(AgentFactoryV4.Application) -> _applications_12[id_1]
application_1 (-> ['_applications'])(AgentFactoryV4.Application) := REF_5257(AgentFactoryV4.Application)
 require(bool,string)(msg.sender == application.proposer || hasRole(WITHDRAW_ROLE,msg.sender),Not proposer)
REF_5258(address) -> application_1 (-> ['_applications']).proposer
TMP_12732(bool) = msg.sender == REF_5258
TMP_12733(bool) = INTERNAL_CALL, AccessControl.hasRole(bytes32,address)(WITHDRAW_ROLE_5,msg.sender)
TMP_12734(bool) = TMP_12732 || TMP_12733
TMP_12735(None) = SOLIDITY_CALL require(bool,string)(TMP_12734,Not proposer)
 _executeApplication(id,canStake,_tokenSupplyParams)
INTERNAL_CALL, AgentFactoryV4._executeApplication(uint256,bool,bytes)(id_1,canStake_1,_tokenSupplyParams_3)
 noReentrant()
MODIFIER_CALL, AgentFactoryV4.noReentrant()()
```
#### AgentFactoryV4.executeTokenApplication(uint256,bool) [PUBLIC]
```slithir
WITHDRAW_ROLE_7(bytes32) := phi(['WITHDRAW_ROLE_6', 'WITHDRAW_ROLE_0', 'WITHDRAW_ROLE_3', 'WITHDRAW_ROLE_9'])
_applications_14(mapping(uint256 => AgentFactoryV4.Application)) := phi(['_applications_13', '_applications_0', '_applications_2', '_applications_10', '_applications_6', '_applications_1', '_applications_15', '_applications_12'])
_tokenSupplyParams_6(bytes) := phi(['_tokenSupplyParams_9', '_tokenSupplyParams_4', '_tokenSupplyParams_5', '_tokenSupplyParams_0'])
_applicationToken_7(mapping(uint256 => address)) := phi(['_applicationToken_0', '_applicationToken_9', '_applicationToken_6', '_applicationToken_4', '_applicationToken_5', '_applicationToken_3'])
 application = _applications[id]
REF_5292(AgentFactoryV4.Application) -> _applications_15[id_1]
application_1 (-> ['_applications'])(AgentFactoryV4.Application) := REF_5292(AgentFactoryV4.Application)
 require(bool,string)(msg.sender == application.proposer || hasRole(WITHDRAW_ROLE,msg.sender),Not proposer)
REF_5293(address) -> application_1 (-> ['_applications']).proposer
TMP_12810(bool) = msg.sender == REF_5293
TMP_12811(bool) = INTERNAL_CALL, AccessControl.hasRole(bytes32,address)(WITHDRAW_ROLE_8,msg.sender)
TMP_12812(bool) = TMP_12810 || TMP_12811
TMP_12813(None) = SOLIDITY_CALL require(bool,string)(TMP_12812,Not proposer)
 require(bool,string)(_applicationToken[id] != address(0),Not custom token application)
REF_5294(address) -> _applicationToken_9[id_1]
TMP_12814 = CONVERT 0 to address
TMP_12815(bool) = REF_5294 != TMP_12814
TMP_12816(None) = SOLIDITY_CALL require(bool,string)(TMP_12815,Not custom token application)
 _executeApplication(id,canStake,_tokenSupplyParams)
INTERNAL_CALL, AgentFactoryV4._executeApplication(uint256,bool,bytes)(id_1,canStake_1,_tokenSupplyParams_8)
 noReentrant()
MODIFIER_CALL, AgentFactoryV4.noReentrant()()
```
#### AgentFactoryV4.getApplication(uint256) [PUBLIC]
```slithir
_applications_1(mapping(uint256 => AgentFactoryV4.Application)) := phi(['_applications_13', '_applications_0', '_applications_2', '_applications_10', '_applications_6', '_applications_1', '_applications_15', '_applications_12'])
 _applications[proposalId]
REF_5191(AgentFactoryV4.Application) -> _applications_1[proposalId_1]
RETURN REF_5191
```
#### AgentFactoryV4.initFromToken(address,uint8[],bytes32,address,uint32,uint256,uint256) [PUBLIC]
```slithir
_nextId_8(uint256) := phi(['_nextId_0', '_nextId_1', '_nextId_7', '_nextId_14'])
applicationThreshold_8(uint256) := phi(['applicationThreshold_1', 'applicationThreshold_6', 'applicationThreshold_0', 'applicationThreshold_15', 'applicationThreshold_7'])
assetToken_19(address) := phi(['assetToken_24', 'assetToken_0', 'assetToken_1', 'assetToken_9', 'assetToken_15', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12'])
_tokenApplication_2(mapping(address => uint256)) := phi(['_tokenApplication_5', '_tokenApplication_0', '_tokenApplication_1'])
 sender = _msgSender()
TMP_12778(address) = INTERNAL_CALL, AgentFactoryV4._msgSender()()
sender_1(address) := TMP_12778(address)
 require(bool,string)(_tokenApplication[tokenAddr] == 0,Token already exists)
REF_5280(uint256) -> _tokenApplication_4[tokenAddr_1]
TMP_12779(bool) = REF_5280 == 0
TMP_12780(None) = SOLIDITY_CALL require(bool,string)(TMP_12779,Token already exists)
 require(bool,string)(isCompatibleToken(tokenAddr),Unsupported token)
TMP_12781(bool) = INTERNAL_CALL, AgentFactoryV4.isCompatibleToken(address)(tokenAddr_1)
TMP_12782(None) = SOLIDITY_CALL require(bool,string)(TMP_12781,Unsupported token)
 require(bool,string)(IERC20(assetToken).balanceOf(sender) >= applicationThreshold,Insufficient asset token)
TMP_12783 = CONVERT assetToken_22 to IERC20
TMP_12784(uint256) = HIGH_LEVEL_CALL, dest:TMP_12783(IERC20), function:balanceOf, arguments:['sender_1']  
_nextId_12(uint256) := phi(['_nextId_1', '_nextId_7', '_nextId_11', '_nextId_14'])
applicationThreshold_12(uint256) := phi(['applicationThreshold_11', 'applicationThreshold_1', 'applicationThreshold_6', 'applicationThreshold_15', 'applicationThreshold_7'])
assetToken_23(address) := phi(['assetToken_24', 'assetToken_22', 'assetToken_1', 'assetToken_15', 'assetToken_9', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12'])
TMP_12785(bool) = TMP_12784 >= applicationThreshold_12
TMP_12786(None) = SOLIDITY_CALL require(bool,string)(TMP_12785,Insufficient asset token)
 require(bool,string)(IERC20(assetToken).allowance(sender,address(this)) >= applicationThreshold,Insufficient asset token allowance)
TMP_12787 = CONVERT assetToken_23 to IERC20
TMP_12788 = CONVERT this to address
TMP_12789(uint256) = HIGH_LEVEL_CALL, dest:TMP_12787(IERC20), function:allowance, arguments:['sender_1', 'TMP_12788']  
_nextId_13(uint256) := phi(['_nextId_1', '_nextId_12', '_nextId_7', '_nextId_14'])
applicationThreshold_13(uint256) := phi(['applicationThreshold_1', 'applicationThreshold_12', 'applicationThreshold_6', 'applicationThreshold_15', 'applicationThreshold_7'])
assetToken_24(address) := phi(['assetToken_24', 'assetToken_1', 'assetToken_15', 'assetToken_9', 'assetToken_17', 'assetToken_18', 'assetToken_23', 'assetToken_28', 'assetToken_6', 'assetToken_12'])
TMP_12790(bool) = TMP_12789 >= applicationThreshold_13
TMP_12791(None) = SOLIDITY_CALL require(bool,string)(TMP_12790,Insufficient asset token allowance)
 require(bool,string)(cores.length > 0,Cores must be provided)
REF_5283 -> LENGTH cores_1
TMP_12792(bool) = REF_5283 > 0
TMP_12793(None) = SOLIDITY_CALL require(bool,string)(TMP_12792,Cores must be provided)
 require(bool,string)(initialLP > 0,InitialLP must be greater than 0)
TMP_12794(bool) = initialLP_1 > 0
TMP_12795(None) = SOLIDITY_CALL require(bool,string)(TMP_12794,InitialLP must be greater than 0)
 IERC20(tokenAddr).safeTransferFrom(sender,address(this),initialLP)
TMP_12796 = CONVERT tokenAddr_1 to IERC20
TMP_12797 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_12796', 'sender_1', 'TMP_12797', 'initialLP_1'] 
 IERC20(assetToken).safeTransferFrom(sender,address(this),applicationThreshold)
TMP_12799 = CONVERT assetToken_24 to IERC20
TMP_12800 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_12799', 'sender_1', 'TMP_12800', 'applicationThreshold_13'] 
 id = _nextId ++
TMP_12802(uint256) := _nextId_13(uint256)
_nextId_14(uint256) = _nextId_13 (c)+ 1
id_1(uint256) := TMP_12802(uint256)
 _tokenApplication[tokenAddr] = id
REF_5286(uint256) -> _tokenApplication_4[tokenAddr_1]
_tokenApplication_5(mapping(address => uint256)) := phi(['_tokenApplication_4'])
REF_5286(uint256) (->_tokenApplication_5) := id_1(uint256)
 _applicationToken[id] = tokenAddr
REF_5287(address) -> _applicationToken_5[id_1]
_applicationToken_6(mapping(uint256 => address)) := phi(['_applicationToken_5'])
REF_5287(address) (->_applicationToken_6) := tokenAddr_1(address)
 application = Application(IAgentToken(tokenAddr).name(),IAgentToken(tokenAddr).symbol(),,ApplicationStatus.Active,applicationThreshold,sender,cores,block.number,0,tbaSalt,tbaImplementation,daoVotingPeriod,daoThreshold)
TMP_12803 = CONVERT tokenAddr_1 to IAgentToken
TMP_12804(string) = HIGH_LEVEL_CALL, dest:TMP_12803(IAgentToken), function:name, arguments:[]  
applicationThreshold_14(uint256) := phi(['applicationThreshold_13', 'applicationThreshold_1', 'applicationThreshold_6', 'applicationThreshold_15', 'applicationThreshold_7'])
TMP_12805 = CONVERT tokenAddr_1 to IAgentToken
TMP_12806(string) = HIGH_LEVEL_CALL, dest:TMP_12805(IAgentToken), function:symbol, arguments:[]  
applicationThreshold_15(uint256) := phi(['applicationThreshold_1', 'applicationThreshold_14', 'applicationThreshold_6', 'applicationThreshold_15', 'applicationThreshold_7'])
REF_5290(AgentFactoryV4.ApplicationStatus) -> ApplicationStatus.Active
TMP_12807(AgentFactoryV4.Application) = new Application(TMP_12804,TMP_12806,,REF_5290,applicationThreshold_15,sender_1,cores_1,block.number,0,tbaSalt_1,tbaImplementation_1,daoVotingPeriod_1,daoThreshold_1)
application_1(AgentFactoryV4.Application) := TMP_12807(AgentFactoryV4.Application)
 _applications[id] = application
REF_5291(AgentFactoryV4.Application) -> _applications_12[id_1]
_applications_13(mapping(uint256 => AgentFactoryV4.Application)) := phi(['_applications_12'])
REF_5291(AgentFactoryV4.Application) (->_applications_13) := application_1(AgentFactoryV4.Application)
 NewApplication(id)
Emit NewApplication(id_1)
 id
RETURN id_1
 whenNotPaused()
MODIFIER_CALL, PausableUpgradeable.whenNotPaused()()
```
#### AgentFactoryV4.initialize(address,address,address,address,address,address,uint256,address,uint256) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_4'])
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
TMP_12638(bool) = INTERNAL_CALL, AccessControl._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_3,msg.sender)
 _vault = vault_
_vault_1(address) := vault__1(address)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### AgentFactoryV4.isCompatibleToken(address) [PUBLIC]
```slithir
tokenAddr_1(address) := phi(['tokenAddr_1'])
 IAgentToken(tokenAddr).name()
TMP_12819 = CONVERT tokenAddr_1 to IAgentToken
TMP_12820(string) = HIGH_LEVEL_CALL, dest:TMP_12819(IAgentToken), function:name, arguments:[]  
 IAgentToken(tokenAddr).symbol()
TMP_12821 = CONVERT tokenAddr_1 to IAgentToken
TMP_12822(string) = HIGH_LEVEL_CALL, dest:TMP_12821(IAgentToken), function:symbol, arguments:[]  
 IAgentToken(tokenAddr).totalSupply()
TMP_12823 = CONVERT tokenAddr_1 to IAgentToken
TMP_12824(uint256) = HIGH_LEVEL_CALL, dest:TMP_12823(IAgentToken), function:totalSupply, arguments:[]  
 IAgentToken(tokenAddr).balanceOf(address(this))
TMP_12825 = CONVERT tokenAddr_1 to IAgentToken
TMP_12826 = CONVERT this to address
TMP_12827(uint256) = HIGH_LEVEL_CALL, dest:TMP_12825(IAgentToken), function:balanceOf, arguments:['TMP_12826']  
 true
RETURN True
 false
RETURN False
 false
RETURN False
 false
RETURN False
 false
RETURN False
```
#### AgentFactoryV4.pause() [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_23(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_4'])
 _pause()
INTERNAL_CALL, PausableUpgradeable._pause()()
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_23)
```
#### AgentFactoryV4.proposeAgent(string,string,string,uint8[],bytes32,address,uint32,uint256) [PUBLIC]
```slithir
_nextId_2(uint256) := phi(['_nextId_0', '_nextId_1', '_nextId_7', '_nextId_14'])
applicationThreshold_2(uint256) := phi(['applicationThreshold_1', 'applicationThreshold_6', 'applicationThreshold_0', 'applicationThreshold_15', 'applicationThreshold_7'])
assetToken_2(address) := phi(['assetToken_24', 'assetToken_0', 'assetToken_1', 'assetToken_9', 'assetToken_15', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12'])
 sender = _msgSender()
TMP_12640(address) = INTERNAL_CALL, AgentFactoryV4._msgSender()()
sender_1(address) := TMP_12640(address)
 require(bool,string)(IERC20(assetToken).balanceOf(sender) >= applicationThreshold,Insufficient asset token)
TMP_12641 = CONVERT assetToken_4 to IERC20
TMP_12642(uint256) = HIGH_LEVEL_CALL, dest:TMP_12641(IERC20), function:balanceOf, arguments:['sender_1']  
_nextId_5(uint256) := phi(['_nextId_1', '_nextId_7', '_nextId_4', '_nextId_14'])
applicationThreshold_5(uint256) := phi(['applicationThreshold_1', 'applicationThreshold_6', 'applicationThreshold_4', 'applicationThreshold_15', 'applicationThreshold_7'])
assetToken_5(address) := phi(['assetToken_24', 'assetToken_1', 'assetToken_4', 'assetToken_15', 'assetToken_9', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12'])
TMP_12643(bool) = TMP_12642 >= applicationThreshold_5
TMP_12644(None) = SOLIDITY_CALL require(bool,string)(TMP_12643,Insufficient asset token)
 require(bool,string)(IERC20(assetToken).allowance(sender,address(this)) >= applicationThreshold,Insufficient asset token allowance)
TMP_12645 = CONVERT assetToken_5 to IERC20
TMP_12646 = CONVERT this to address
TMP_12647(uint256) = HIGH_LEVEL_CALL, dest:TMP_12645(IERC20), function:allowance, arguments:['sender_1', 'TMP_12646']  
_nextId_6(uint256) := phi(['_nextId_1', '_nextId_7', '_nextId_5', '_nextId_14'])
applicationThreshold_6(uint256) := phi(['applicationThreshold_1', 'applicationThreshold_6', 'applicationThreshold_5', 'applicationThreshold_15', 'applicationThreshold_7'])
assetToken_6(address) := phi(['assetToken_5', 'assetToken_24', 'assetToken_1', 'assetToken_15', 'assetToken_9', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12'])
TMP_12648(bool) = TMP_12647 >= applicationThreshold_6
TMP_12649(None) = SOLIDITY_CALL require(bool,string)(TMP_12648,Insufficient asset token allowance)
 require(bool,string)(cores.length > 0,Cores must be provided)
REF_5194 -> LENGTH cores_1
TMP_12650(bool) = REF_5194 > 0
TMP_12651(None) = SOLIDITY_CALL require(bool,string)(TMP_12650,Cores must be provided)
 IERC20(assetToken).safeTransferFrom(sender,address(this),applicationThreshold)
TMP_12652 = CONVERT assetToken_6 to IERC20
TMP_12653 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_12652', 'sender_1', 'TMP_12653', 'applicationThreshold_6'] 
 id = _nextId ++
TMP_12655(uint256) := _nextId_6(uint256)
_nextId_7(uint256) = _nextId_6 (c)+ 1
id_1(uint256) := TMP_12655(uint256)
 proposalEndBlock = block.number
proposalEndBlock_1(uint256) := block.number(uint256)
 application = Application(name,symbol,tokenURI,ApplicationStatus.Active,applicationThreshold,sender,cores,proposalEndBlock,0,tbaSalt,tbaImplementation,daoVotingPeriod,daoThreshold)
REF_5196(AgentFactoryV4.ApplicationStatus) -> ApplicationStatus.Active
TMP_12656(AgentFactoryV4.Application) = new Application(name_1,symbol_1,tokenURI_1,REF_5196,applicationThreshold_6,sender_1,cores_1,proposalEndBlock_1,0,tbaSalt_1,tbaImplementation_1,daoVotingPeriod_1,daoThreshold_1)
application_1(AgentFactoryV4.Application) := TMP_12656(AgentFactoryV4.Application)
 _applications[id] = application
REF_5197(AgentFactoryV4.Application) -> _applications_1[id_1]
_applications_2(mapping(uint256 => AgentFactoryV4.Application)) := phi(['_applications_1'])
REF_5197(AgentFactoryV4.Application) (->_applications_2) := application_1(AgentFactoryV4.Application)
 NewApplication(id)
Emit NewApplication(id_1)
 id
RETURN id_1
 whenNotPaused()
MODIFIER_CALL, PausableUpgradeable.whenNotPaused()()
```
#### AgentFactoryV4.setApplicationThreshold(uint256) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_5(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_4'])
 applicationThreshold = newThreshold
applicationThreshold_7(uint256) := newThreshold_1(uint256)
 ApplicationThresholdUpdated(newThreshold)
Emit ApplicationThresholdUpdated(newThreshold_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_5)
```
#### AgentFactoryV4.setAssetToken(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_21(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_4'])
 assetToken = newToken
assetToken_18(address) := newToken_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_21)
```
#### AgentFactoryV4.setDefaultDelegatee(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_27(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_4'])
 defaultDelegatee = newDelegatee
defaultDelegatee_17(address) := newDelegatee_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_27)
```
#### AgentFactoryV4.setImplementations(address,address,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_9(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_4'])
 tokenImplementation = token
tokenImplementation_3(address) := token_1(address)
 daoImplementation = dao
daoImplementation_3(address) := dao_1(address)
 veTokenImplementation = veToken
veTokenImplementation_3(address) := veToken_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_9)
```
#### AgentFactoryV4.setMaturityDuration(uint256) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_11(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_4'])
 maturityDuration = newDuration
maturityDuration_3(uint256) := newDuration_1(uint256)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_11)
```
#### AgentFactoryV4.setTokenAdmin(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_15(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_4'])
 _tokenAdmin = newTokenAdmin
_tokenAdmin_4(address) := newTokenAdmin_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_15)
```
#### AgentFactoryV4.setTokenSupplyParams(uint256,uint256,uint256,uint256,uint256,uint256,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_17(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_4'])
 _tokenSupplyParams = abi.encode(maxSupply,lpSupply,vaultSupply,maxTokensPerWallet,maxTokensPerTxn,botProtectionDurationInSeconds,vault)
TMP_12766(bytes) = SOLIDITY_CALL abi.encode()(maxSupply_1,lpSupply_1,vaultSupply_1,maxTokensPerWallet_1,maxTokensPerTxn_1,botProtectionDurationInSeconds_1,vault_1)
_tokenSupplyParams_5(bytes) := TMP_12766(bytes)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_17)
```
#### AgentFactoryV4.setTokenTaxParams(uint256,uint256,uint256,address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_19(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_4'])
 _tokenTaxParams = abi.encode(projectBuyTaxBasisPoints,projectSellTaxBasisPoints,taxSwapThresholdBasisPoints,projectTaxRecipient)
TMP_12768(bytes) = SOLIDITY_CALL abi.encode()(projectBuyTaxBasisPoints_1,projectSellTaxBasisPoints_1,taxSwapThresholdBasisPoints_1,projectTaxRecipient_1)
_tokenTaxParams_3(bytes) := TMP_12768(bytes)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_19)
```
#### AgentFactoryV4.setUniswapRouter(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_13(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_4'])
 _uniswapRouter = router
_uniswapRouter_7(address) := router_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_13)
```
#### AgentFactoryV4.setVault(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_7(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_4'])
 _vault = newVault
_vault_13(address) := newVault_1(address)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_7)
```
#### AgentFactoryV3.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
 WITHDRAW_ROLE = keccak256(bytes)(WITHDRAW_ROLE)
 BONDING_ROLE = keccak256(bytes)(BONDING_ROLE)
 _requireNotPaused()
INTERNAL_CALL, PausableUpgradeable._requireNotPaused()()
 _requirePaused()
INTERNAL_CALL, PausableUpgradeable._requirePaused()()
 $ = _getInitializableStorage()
TMP_12542(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_12542'])(Initializable.InitializableStorage) := TMP_12542(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_5156(bool) -> $_1 (-> ['TMP_12542'])._initializing
TMP_12543 = UnaryType.BANG REF_5156 
isTopLevelCall_1(bool) := TMP_12543(bool)
 initialized = $._initialized
REF_5157(uint64) -> $_1 (-> ['TMP_12542'])._initialized
initialized_1(uint64) := REF_5157(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_12544(bool) = initialized_1 == 0
TMP_12545(bool) = TMP_12544 && isTopLevelCall_1
initialSetup_1(bool) := TMP_12545(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_12546(bool) = initialized_1 == 1
TMP_12547 = CONVERT this to address
TMP_12548(bytes) = SOLIDITY_CALL code(address)(TMP_12547)
REF_5158 -> LENGTH TMP_12548
TMP_12549(bool) = REF_5158 == 0
TMP_12550(bool) = TMP_12546 && TMP_12549
construction_1(bool) := TMP_12550(bool)
 ! initialSetup && ! construction
TMP_12551 = UnaryType.BANG initialSetup_1 
TMP_12552 = UnaryType.BANG construction_1 
TMP_12553(bool) = TMP_12551 && TMP_12552
CONDITION TMP_12553
 revert InvalidInitialization()()
TMP_12554(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_5159(uint64) -> $_1 (-> ['TMP_12542'])._initialized
$_2 (-> ['TMP_12542'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_12542'])"])
REF_5159(uint64) (->$_2 (-> ['TMP_12542'])) := 1(uint256)
TMP_12542(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12542'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_5160(bool) -> $_2 (-> ['TMP_12542'])._initializing
$_3 (-> ['TMP_12542'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12542'])"])
REF_5160(bool) (->$_3 (-> ['TMP_12542'])) := True(bool)
TMP_12542(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12542'])"])
$_4 (-> ['TMP_12542'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12542'])", "$_2 (-> ['TMP_12542'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_5161(bool) -> $_4 (-> ['TMP_12542'])._initializing
$_5 (-> ['TMP_12542'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_12542'])"])
REF_5161(bool) (->$_5 (-> ['TMP_12542'])) := False(bool)
TMP_12542(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_12542'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_12556(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_12556'])(Initializable.InitializableStorage) := TMP_12556(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_5162(bool) -> $_1 (-> ['TMP_12556'])._initializing
REF_5163(uint64) -> $_1 (-> ['TMP_12556'])._initialized
TMP_12557(bool) = REF_5163 >= version_1
TMP_12558(bool) = REF_5162 || TMP_12557
CONDITION TMP_12558
 revert InvalidInitialization()()
TMP_12559(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_5164(uint64) -> $_1 (-> ['TMP_12556'])._initialized
$_2 (-> ['TMP_12556'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_12556'])"])
REF_5164(uint64) (->$_2 (-> ['TMP_12556'])) := version_1(uint64)
TMP_12556(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12556'])"])
 $._initializing = true
REF_5165(bool) -> $_2 (-> ['TMP_12556'])._initializing
$_3 (-> ['TMP_12556'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_12556'])"])
REF_5165(bool) (->$_3 (-> ['TMP_12556'])) := True(bool)
TMP_12556(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12556'])"])
 $._initializing = false
REF_5166(bool) -> $_3 (-> ['TMP_12556'])._initializing
$_4 (-> ['TMP_12556'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_12556'])"])
REF_5166(bool) (->$_4 (-> ['TMP_12556'])) := False(bool)
TMP_12556(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_12556'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
role_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_11', 'DEFAULT_ADMIN_ROLE_17', 'TMP_12373', 'BONDING_ROLE_2', 'DEFAULT_ADMIN_ROLE_23', 'TMP_12370', 'DEFAULT_ADMIN_ROLE_27', 'DEFAULT_ADMIN_ROLE_13', 'DEFAULT_ADMIN_ROLE_5', 'DEFAULT_ADMIN_ROLE_15', 'DEFAULT_ADMIN_ROLE_19', 'TMP_12375', 'DEFAULT_ADMIN_ROLE_9', 'DEFAULT_ADMIN_ROLE_7', 'DEFAULT_ADMIN_ROLE_25', 'BONDING_ROLE_4', 'DEFAULT_ADMIN_ROLE_21', 'TMP_12368'])
 _checkRole(role)
INTERNAL_CALL, AccessControl._checkRole(bytes32)(role_1)
gov_1(address) := phi(['gov_0'])
 require(bool,string)(msg.sender == gov,Only DAO can execute proposal)
TMP_12563(bool) = msg.sender == gov_1
TMP_12564(None) = SOLIDITY_CALL require(bool,string)(TMP_12563,Only DAO can execute proposal)
locked_1(bool) := phi(['locked_0', 'locked_3'])
 require(bool,string)(! locked,cannot reenter)
TMP_12565 = UnaryType.BANG locked_1 
TMP_12566(None) = SOLIDITY_CALL require(bool,string)(TMP_12565,cannot reenter)
 locked = true
locked_2(bool) := True(bool)
 locked = false
locked_3(bool) := False(bool)
```
#### AgentFactoryV4.totalAgents() [PUBLIC]
```slithir
allTokens_5(address[]) := phi(['allTokens_5', 'allTokens_0', 'allTokens_4'])
 allTokens.length
REF_5275 -> LENGTH allTokens_5
RETURN REF_5275
```
#### AgentFactoryV4.unpause() [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_25(bytes32) := phi(['DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_10', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_12', 'DEFAULT_ADMIN_ROLE_14', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_6', 'DEFAULT_ADMIN_ROLE_8', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_4'])
 _unpause()
INTERNAL_CALL, PausableUpgradeable._unpause()()
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_25)
```
#### AgentFactoryV4.withdraw(uint256) [PUBLIC]
```slithir
assetToken_7(address) := phi(['assetToken_24', 'assetToken_0', 'assetToken_1', 'assetToken_9', 'assetToken_15', 'assetToken_17', 'assetToken_18', 'assetToken_28', 'assetToken_6', 'assetToken_12'])
WITHDRAW_ROLE_1(bytes32) := phi(['WITHDRAW_ROLE_6', 'WITHDRAW_ROLE_0', 'WITHDRAW_ROLE_3', 'WITHDRAW_ROLE_9'])
_applications_3(mapping(uint256 => AgentFactoryV4.Application)) := phi(['_applications_13', '_applications_0', '_applications_2', '_applications_10', '_applications_6', '_applications_1', '_applications_15', '_applications_12'])
_applicationToken_1(mapping(uint256 => address)) := phi(['_applicationToken_0', '_applicationToken_9', '_applicationToken_6', '_applicationToken_4', '_applicationToken_5', '_applicationToken_3'])
 application = _applications[id]
REF_5198(AgentFactoryV4.Application) -> _applications_4[id_1]
application_1 (-> ['_applications'])(AgentFactoryV4.Application) := REF_5198(AgentFactoryV4.Application)
 require(bool,string)(msg.sender == application.proposer || hasRole(WITHDRAW_ROLE,msg.sender),Not proposer)
REF_5199(address) -> application_1 (-> ['_applications']).proposer
TMP_12659(bool) = msg.sender == REF_5199
TMP_12660(bool) = INTERNAL_CALL, AccessControl.hasRole(bytes32,address)(WITHDRAW_ROLE_2,msg.sender)
TMP_12661(bool) = TMP_12659 || TMP_12660
TMP_12662(None) = SOLIDITY_CALL require(bool,string)(TMP_12661,Not proposer)
 require(bool,string)(application.status == ApplicationStatus.Active,Application is not active)
REF_5200(AgentFactoryV4.ApplicationStatus) -> application_1 (-> ['_applications']).status
REF_5201(AgentFactoryV4.ApplicationStatus) -> ApplicationStatus.Active
TMP_12663(bool) = REF_5200 == REF_5201
TMP_12664(None) = SOLIDITY_CALL require(bool,string)(TMP_12663,Application is not active)
 require(bool,string)(block.number > application.proposalEndBlock,Application is not matured yet)
REF_5202(uint256) -> application_1 (-> ['_applications']).proposalEndBlock
TMP_12665(bool) = block.number > REF_5202
TMP_12666(None) = SOLIDITY_CALL require(bool,string)(TMP_12665,Application is not matured yet)
 withdrawableAmount = application.withdrawableAmount
REF_5203(uint256) -> application_1 (-> ['_applications']).withdrawableAmount
withdrawableAmount_1(uint256) := REF_5203(uint256)
 application.withdrawableAmount = 0
REF_5204(uint256) -> application_1 (-> ['_applications']).withdrawableAmount
application_2 (-> ['_applications'])(AgentFactoryV4.Application) := phi(["application_1 (-> ['_applications'])"])
REF_5204(uint256) (->application_2 (-> ['_applications'])) := 0(uint256)
_applications_5(mapping(uint256 => AgentFactoryV4.Application)) := phi(["application_2 (-> ['_applications'])"])
 application.status = ApplicationStatus.Withdrawn
REF_5205(AgentFactoryV4.ApplicationStatus) -> application_2 (-> ['_applications']).status
REF_5206(AgentFactoryV4.ApplicationStatus) -> ApplicationStatus.Withdrawn
application_3 (-> ['_applications'])(AgentFactoryV4.Application) := phi(["application_2 (-> ['_applications'])"])
REF_5205(AgentFactoryV4.ApplicationStatus) (->application_3 (-> ['_applications'])) := REF_5206(AgentFactoryV4.ApplicationStatus)
_applications_6(mapping(uint256 => AgentFactoryV4.Application)) := phi(["application_3 (-> ['_applications'])"])
 IERC20(assetToken).safeTransfer(application.proposer,withdrawableAmount)
TMP_12667 = CONVERT assetToken_9 to IERC20
REF_5208(address) -> application_3 (-> ['_applications']).proposer
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_12667', 'REF_5208', 'withdrawableAmount_1'] 
 customToken = _applicationToken[id]
REF_5209(address) -> _applicationToken_3[id_1]
customToken_1(address) := REF_5209(address)
 customToken != address(0)
TMP_12669 = CONVERT 0 to address
TMP_12670(bool) = customToken_1 != TMP_12669
CONDITION TMP_12670
 IERC20(customToken).safeTransfer(application.proposer,IERC20(customToken).balanceOf(address(this)))
TMP_12671 = CONVERT customToken_1 to IERC20
REF_5211(address) -> application_3 (-> ['_applications']).proposer
TMP_12672 = CONVERT customToken_1 to IERC20
TMP_12673 = CONVERT this to address
TMP_12674(uint256) = HIGH_LEVEL_CALL, dest:TMP_12672(IERC20), function:balanceOf, arguments:['TMP_12673']  
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_12671', 'REF_5211', 'TMP_12674'] 
 _tokenApplication[customToken] = 0
REF_5213(uint256) -> _tokenApplication_0[customToken_1]
_tokenApplication_1(mapping(address => uint256)) := phi(['_tokenApplication_0'])
REF_5213(uint256) (->_tokenApplication_1) := 0(uint256)
 _applicationToken[id] = address(0)
REF_5214(address) -> _applicationToken_3[id_1]
TMP_12676 = CONVERT 0 to address
_applicationToken_4(mapping(uint256 => address)) := phi(['_applicationToken_3'])
REF_5214(address) (->_applicationToken_4) := TMP_12676(address)
 noReentrant()
MODIFIER_CALL, AgentFactoryV4.noReentrant()()
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
#### IUniswapV2Factory.createPair(address,address) [EXTERNAL]
```slithir

```
#### IUniswapV2Factory.getPair(address,address) [EXTERNAL]
```slithir

```
#### IERC20.approve(address,uint256) [EXTERNAL]
```slithir

```
#### IERC20.balanceOf(address) [EXTERNAL]
```slithir

```
#### SafeERC20.forceApprove(IERC20,address,uint256) [INTERNAL]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1'])
spender_1(address) := phi(['spender_1', 'spender_1'])
value_1(uint256) := phi(['TMP_5419', 'TMP_5425'])
 approvalCall = abi.encodeCall(token.approve,(spender,value))
REF_2050(approve) -> token_1.approve
TMP_5427(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2050,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78002500>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff780023e0>])
approvalCall_1(bytes) := TMP_5427(bytes)
 ! _callOptionalReturnBool(token,approvalCall)
TMP_5428(bool) = INTERNAL_CALL, SafeERC20._callOptionalReturnBool(IERC20,bytes)(token_1,approvalCall_1)
TMP_5429 = UnaryType.BANG TMP_5428 
CONDITION TMP_5429
 _callOptionalReturn(token,abi.encodeCall(token.approve,(spender,0)))
REF_2052(approve) -> token_1.approve
TMP_5430(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2052,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78002500>, 0])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5430)
 _callOptionalReturn(token,approvalCall)
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,approvalCall_1)
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
