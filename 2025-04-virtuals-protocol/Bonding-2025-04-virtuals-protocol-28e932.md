### Storage layout (FERC20) 

```text
_totalSupply uint256
_name string
_symbol string
maxTx uint256
_maxTxAmount uint256
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
isExcludedFromMaxTx mapping(address => bool)

```
### Storage layout (Bonding) 

```text
_feeTo address
factory FFactory
router FRouter
initialSupply uint256
fee uint256
assetRate uint256
gradThreshold uint256
maxTx uint256
agentFactory address
_deployParams Bonding.DeployParams
profile mapping(address => Bonding.Profile)
profiles address[]
tokenInfo mapping(address => Bonding.Token)
tokenInfos address[]

```




### Storage layout (FFactory) 

```text
_pair mapping(address => mapping(address => address))
pairs address[]
router address
taxVault address
buyTax uint256
sellTax uint256

```


### Storage layout (FRouter) 

```text
factory FFactory
assetToken address
taxManager address

```
### Storage layout (FPair) 

```text
router address
tokenA address
tokenB address
_pool FPair.Pool

```
#### Bonding._approval(address,address,uint256) [INTERNAL]
```slithir
_spender_1(address) := phi(['TMP_8043'])
_token_1(address) := phi(['TMP_8044'])
amount_1(uint256) := phi(['supply_1'])
 IERC20(_token).forceApprove(_spender,amount)
TMP_8011 = CONVERT _token_1 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.forceApprove(IERC20,address,uint256), arguments:['TMP_8011', '_spender_1', 'amount_1'] 
 true
RETURN True
```
#### Bonding._checkIfProfileExists(address) [INTERNAL]
```slithir
_user_1(address) := phi(['account_1', 'creator_1'])
profile_2(mapping(address => Bonding.Profile)) := phi(['profile_16', 'profile_2', 'profile_4', 'profile_15', 'profile_1', 'profile_0', 'profile_14'])
 profile[_user].user == _user
REF_3138(Bonding.Profile) -> profile_2[_user_1]
REF_3139(address) -> REF_3138.user
TMP_8010(bool) = REF_3139 == _user_1
RETURN TMP_8010
```
#### Bonding._createUserProfile(address) [INTERNAL]
```slithir
_user_1(address) := phi(['creator_1'])
profiles_1(address[]) := phi(['profiles_3', 'profiles_0'])
 _profile = Profile({user:_user,tokens:_tokens})
TMP_8006(Bonding.Profile) = new Profile(_user_1,_tokens_0)
_profile_1(Bonding.Profile) := TMP_8006(Bonding.Profile)
 profile[_user] = _profile
REF_3134(Bonding.Profile) -> profile_0[_user_1]
profile_1(mapping(address => Bonding.Profile)) := phi(['profile_0'])
REF_3134(Bonding.Profile) (->profile_1) := _profile_1(Bonding.Profile)
 profiles.push(_user)
REF_3136 -> LENGTH profiles_1
TMP_8008(uint256) := REF_3136(uint256)
TMP_8009(uint256) = TMP_8008 (c)+ 1
profiles_2(address[]) := phi(['profiles_1'])
REF_3136(uint256) (->profiles_2) := TMP_8009(uint256)
REF_3137(address) -> profiles_2[TMP_8008]
profiles_3(address[]) := phi(['profiles_2'])
REF_3137(address) (->profiles_3) := _user_1(address)
 true
RETURN True
```
#### Bonding._openTradingOnUniswap(address) [PRIVATE]
```slithir
tokenAddress_1(address) := phi(['tokenAddress_1'])
factory_14(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_13', 'factory_7', 'factory_0', 'factory_10'])
router_23(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_17', 'router_0'])
agentFactory_2(address) := phi(['agentFactory_0', 'agentFactory_12', 'agentFactory_1'])
_deployParams_2(Bonding.DeployParams) := phi(['_deployParams_1', '_deployParams_9', '_deployParams_0'])
tokenInfo_28(mapping(address => Bonding.Token)) := phi(['tokenInfo_1', 'tokenInfo_27', 'tokenInfo_14', 'tokenInfo_31', 'tokenInfo_32', 'tokenInfo_0'])
 token_ = FERC20(tokenAddress)
TMP_8125 = CONVERT tokenAddress_1 to FERC20
token__1(FERC20) := TMP_8125(FERC20)
 _token = tokenInfo[tokenAddress]
REF_3267(Bonding.Token) -> tokenInfo_28[tokenAddress_1]
_token_1 (-> ['tokenInfo'])(Bonding.Token) := REF_3267(Bonding.Token)
 require(bool,string)(_token.trading && ! _token.tradingOnUniswap,trading is already open)
REF_3268(bool) -> _token_1 (-> ['tokenInfo']).trading
REF_3269(bool) -> _token_1 (-> ['tokenInfo']).tradingOnUniswap
TMP_8126 = UnaryType.BANG REF_3269 
TMP_8127(bool) = REF_3268 && TMP_8126
TMP_8128(None) = SOLIDITY_CALL require(bool,string)(TMP_8127,trading is already open)
 _token.trading = false
REF_3270(bool) -> _token_1 (-> ['tokenInfo']).trading
_token_2 (-> ['tokenInfo'])(Bonding.Token) := phi(["_token_1 (-> ['tokenInfo'])"])
REF_3270(bool) (->_token_2 (-> ['tokenInfo'])) := False(bool)
tokenInfo_29(mapping(address => Bonding.Token)) := phi(["_token_2 (-> ['tokenInfo'])"])
 _token.tradingOnUniswap = true
REF_3271(bool) -> _token_2 (-> ['tokenInfo']).tradingOnUniswap
_token_3 (-> ['tokenInfo'])(Bonding.Token) := phi(["_token_2 (-> ['tokenInfo'])"])
REF_3271(bool) (->_token_3 (-> ['tokenInfo'])) := True(bool)
tokenInfo_30(mapping(address => Bonding.Token)) := phi(["_token_3 (-> ['tokenInfo'])"])
 pairAddress = factory.getPair(tokenAddress,router.assetToken())
TMP_8129(address) = HIGH_LEVEL_CALL, dest:router_23(FRouter), function:assetToken, arguments:[]  
factory_15(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_13', 'factory_7', 'factory_10', 'factory_14'])
router_24(FRouter) := phi(['router_23', 'router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_17'])
agentFactory_3(address) := phi(['agentFactory_2', 'agentFactory_12', 'agentFactory_1'])
_deployParams_3(Bonding.DeployParams) := phi(['_deployParams_1', '_deployParams_9', '_deployParams_2'])
TMP_8130(address) = HIGH_LEVEL_CALL, dest:factory_15(FFactory), function:getPair, arguments:['tokenAddress_1', 'TMP_8129']  
factory_16(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_15', 'factory_13', 'factory_7', 'factory_10'])
router_25(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_24', 'router_35', 'router_1', 'router_17'])
agentFactory_4(address) := phi(['agentFactory_3', 'agentFactory_12', 'agentFactory_1'])
_deployParams_4(Bonding.DeployParams) := phi(['_deployParams_1', '_deployParams_9', '_deployParams_3'])
pairAddress_1(address) := TMP_8130(address)
 pair = IFPair(pairAddress)
TMP_8131 = CONVERT pairAddress_1 to IFPair
pair_1(IFPair) := TMP_8131(IFPair)
 assetBalance = pair.assetBalance()
TMP_8132(uint256) = HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:assetBalance, arguments:[]  
router_26(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_25', 'router_35', 'router_1', 'router_17'])
agentFactory_5(address) := phi(['agentFactory_12', 'agentFactory_1', 'agentFactory_4'])
_deployParams_5(Bonding.DeployParams) := phi(['_deployParams_1', '_deployParams_9', '_deployParams_4'])
assetBalance_1(uint256) := TMP_8132(uint256)
 tokenBalance = pair.balance()
TMP_8133(uint256) = HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:balance, arguments:[]  
router_27(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_26', 'router_17'])
agentFactory_6(address) := phi(['agentFactory_12', 'agentFactory_1', 'agentFactory_5'])
_deployParams_6(Bonding.DeployParams) := phi(['_deployParams_1', '_deployParams_9', '_deployParams_5'])
tokenBalance_1(uint256) := TMP_8133(uint256)
 router.graduate(tokenAddress)
HIGH_LEVEL_CALL, dest:router_27(FRouter), function:graduate, arguments:['tokenAddress_1']  
router_28(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_27', 'router_17'])
agentFactory_7(address) := phi(['agentFactory_6', 'agentFactory_12', 'agentFactory_1'])
_deployParams_7(Bonding.DeployParams) := phi(['_deployParams_1', '_deployParams_9', '_deployParams_6'])
 IERC20(router.assetToken()).forceApprove(agentFactory,assetBalance)
TMP_8135(address) = HIGH_LEVEL_CALL, dest:router_28(FRouter), function:assetToken, arguments:[]  
router_29(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_28', 'router_17'])
agentFactory_8(address) := phi(['agentFactory_12', 'agentFactory_1', 'agentFactory_7'])
_deployParams_8(Bonding.DeployParams) := phi(['_deployParams_1', '_deployParams_9', '_deployParams_7'])
TMP_8136 = CONVERT TMP_8135 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.forceApprove(IERC20,address,uint256), arguments:['TMP_8136', 'agentFactory_8', 'assetBalance_1'] 
 id = IAgentFactoryV3(agentFactory).initFromBondingCurve(string.concat(_token.data._name, by Virtuals),_token.data.ticker,_token.cores,_deployParams.tbaSalt,_deployParams.tbaImplementation,_deployParams.daoVotingPeriod,_deployParams.daoThreshold,assetBalance,_token.creator)
TMP_8138 = CONVERT agentFactory_8 to IAgentFactoryV3
REF_3281(Bonding.Data) -> _token_3 (-> ['tokenInfo']).data
REF_3282(string) -> REF_3281._name
TMP_8139(string) = SOLIDITY_CALL string.concat()(REF_3282, by Virtuals)
REF_3283(Bonding.Data) -> _token_3 (-> ['tokenInfo']).data
REF_3284(string) -> REF_3283.ticker
REF_3285(uint8[]) -> _token_3 (-> ['tokenInfo']).cores
REF_3286(bytes32) -> _deployParams_8.tbaSalt
REF_3287(address) -> _deployParams_8.tbaImplementation
REF_3288(uint32) -> _deployParams_8.daoVotingPeriod
REF_3289(uint256) -> _deployParams_8.daoThreshold
REF_3290(address) -> _token_3 (-> ['tokenInfo']).creator
TMP_8140(uint256) = HIGH_LEVEL_CALL, dest:TMP_8138(IAgentFactoryV3), function:initFromBondingCurve, arguments:['TMP_8139', 'REF_3284', 'REF_3285', 'REF_3286', 'REF_3287', 'REF_3288', 'REF_3289', 'assetBalance_1', 'REF_3290']  
router_30(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_29', 'router_17'])
agentFactory_9(address) := phi(['agentFactory_8', 'agentFactory_12', 'agentFactory_1'])
_deployParams_9(Bonding.DeployParams) := phi(['_deployParams_1', '_deployParams_9', '_deployParams_8'])
id_1(uint256) := TMP_8140(uint256)
 agentToken = IAgentFactoryV3(agentFactory).executeBondingCurveApplication(id,_token.data.supply / (10 ** token_.decimals()),tokenBalance / (10 ** token_.decimals()),pairAddress)
TMP_8141 = CONVERT agentFactory_9 to IAgentFactoryV3
REF_3292(Bonding.Data) -> _token_3 (-> ['tokenInfo']).data
REF_3293(uint256) -> REF_3292.supply
TMP_8142(uint8) = HIGH_LEVEL_CALL, dest:token__1(FERC20), function:decimals, arguments:[]  
router_31(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_17', 'router_30'])
agentFactory_10(address) := phi(['agentFactory_9', 'agentFactory_12', 'agentFactory_1'])
TMP_8143(uint256) = 10 (c)** TMP_8142
TMP_8144(uint256) = REF_3293 (c)/ TMP_8143
TMP_8145(uint8) = HIGH_LEVEL_CALL, dest:token__1(FERC20), function:decimals, arguments:[]  
router_32(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_31', 'router_1', 'router_17'])
agentFactory_11(address) := phi(['agentFactory_10', 'agentFactory_12', 'agentFactory_1'])
TMP_8146(uint256) = 10 (c)** TMP_8145
TMP_8147(uint256) = tokenBalance_1 (c)/ TMP_8146
TMP_8148(address) = HIGH_LEVEL_CALL, dest:TMP_8141(IAgentFactoryV3), function:executeBondingCurveApplication, arguments:['id_1', 'TMP_8144', 'TMP_8147', 'pairAddress_1']  
router_33(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_32', 'router_1', 'router_17'])
agentFactory_12(address) := phi(['agentFactory_12', 'agentFactory_1', 'agentFactory_11'])
agentToken_1(address) := TMP_8148(address)
 _token.agentToken = agentToken
REF_3296(address) -> _token_3 (-> ['tokenInfo']).agentToken
_token_4 (-> ['tokenInfo'])(Bonding.Token) := phi(["_token_3 (-> ['tokenInfo'])"])
REF_3296(address) (->_token_4 (-> ['tokenInfo'])) := agentToken_1(address)
tokenInfo_31(mapping(address => Bonding.Token)) := phi(["_token_4 (-> ['tokenInfo'])"])
 router.approval(pairAddress,agentToken,address(this),IERC20(agentToken).balanceOf(pairAddress))
TMP_8149 = CONVERT this to address
TMP_8150 = CONVERT agentToken_1 to IERC20
TMP_8151(uint256) = HIGH_LEVEL_CALL, dest:TMP_8150(IERC20), function:balanceOf, arguments:['pairAddress_1']  
router_34(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_33', 'router_35', 'router_1', 'router_17'])
HIGH_LEVEL_CALL, dest:router_34(FRouter), function:approval, arguments:['pairAddress_1', 'agentToken_1', 'TMP_8149', 'TMP_8151']  
router_35(FRouter) := phi(['router_22', 'router_38', 'router_34', 'router_12', 'router_35', 'router_1', 'router_17'])
 token_.burnFrom(pairAddress,tokenBalance)
HIGH_LEVEL_CALL, dest:token__1(FERC20), function:burnFrom, arguments:['pairAddress_1', 'tokenBalance_1']  
 Graduated(tokenAddress,agentToken)
Emit Graduated(tokenAddress_1,agentToken_1)
```
#### Bonding.buy(uint256,address) [PUBLIC]
```slithir
factory_11(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_13', 'factory_7', 'factory_0', 'factory_10'])
router_18(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_17', 'router_0'])
gradThreshold_3(uint256) := phi(['gradThreshold_7', 'gradThreshold_2', 'gradThreshold_0', 'gradThreshold_1'])
tokenInfo_15(mapping(address => Bonding.Token)) := phi(['tokenInfo_1', 'tokenInfo_27', 'tokenInfo_14', 'tokenInfo_31', 'tokenInfo_32', 'tokenInfo_0'])
 require(bool,string)(tokenInfo[tokenAddress].trading,Token not trading)
REF_3220(Bonding.Token) -> tokenInfo_15[tokenAddress_1]
REF_3221(bool) -> REF_3220.trading
TMP_8106(None) = SOLIDITY_CALL require(bool,string)(REF_3221,Token not trading)
 pairAddress = factory.getPair(tokenAddress,router.assetToken())
TMP_8107(address) = HIGH_LEVEL_CALL, dest:router_18(FRouter), function:assetToken, arguments:[]  
factory_12(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_11', 'factory_13', 'factory_7', 'factory_10'])
router_19(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_18', 'router_17'])
gradThreshold_4(uint256) := phi(['gradThreshold_7', 'gradThreshold_2', 'gradThreshold_3', 'gradThreshold_1'])
tokenInfo_16(mapping(address => Bonding.Token)) := phi(['tokenInfo_1', 'tokenInfo_27', 'tokenInfo_15', 'tokenInfo_14', 'tokenInfo_31', 'tokenInfo_32'])
TMP_8108(address) = HIGH_LEVEL_CALL, dest:factory_12(FFactory), function:getPair, arguments:['tokenAddress_1', 'TMP_8107']  
factory_13(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_12', 'factory_13', 'factory_7', 'factory_10'])
router_20(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_17', 'router_19'])
gradThreshold_5(uint256) := phi(['gradThreshold_7', 'gradThreshold_2', 'gradThreshold_4', 'gradThreshold_1'])
tokenInfo_17(mapping(address => Bonding.Token)) := phi(['tokenInfo_1', 'tokenInfo_27', 'tokenInfo_16', 'tokenInfo_14', 'tokenInfo_31', 'tokenInfo_32'])
pairAddress_1(address) := TMP_8108(address)
 pair = IFPair(pairAddress)
TMP_8109 = CONVERT pairAddress_1 to IFPair
pair_1(IFPair) := TMP_8109(IFPair)
 (reserveA,reserveB) = pair.getReserves()
TUPLE_89(uint256,uint256) = HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:getReserves, arguments:[]  
router_21(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_17', 'router_20'])
gradThreshold_6(uint256) := phi(['gradThreshold_7', 'gradThreshold_2', 'gradThreshold_5', 'gradThreshold_1'])
tokenInfo_18(mapping(address => Bonding.Token)) := phi(['tokenInfo_1', 'tokenInfo_27', 'tokenInfo_17', 'tokenInfo_14', 'tokenInfo_31', 'tokenInfo_32'])
reserveA_1(uint256)= UNPACK TUPLE_89 index: 0 
reserveB_1(uint256)= UNPACK TUPLE_89 index: 1 
 (amount1In,amount0Out) = router.buy(amountIn,tokenAddress,msg.sender)
TUPLE_90(uint256,uint256) = HIGH_LEVEL_CALL, dest:router_21(FRouter), function:buy, arguments:['amountIn_1', 'tokenAddress_1', 'msg.sender']  
router_22(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_21', 'router_17'])
gradThreshold_7(uint256) := phi(['gradThreshold_7', 'gradThreshold_2', 'gradThreshold_6', 'gradThreshold_1'])
tokenInfo_19(mapping(address => Bonding.Token)) := phi(['tokenInfo_1', 'tokenInfo_27', 'tokenInfo_18', 'tokenInfo_14', 'tokenInfo_31', 'tokenInfo_32'])
amount1In_1(uint256)= UNPACK TUPLE_90 index: 0 
amount0Out_1(uint256)= UNPACK TUPLE_90 index: 1 
 newReserveA = reserveA - amount0Out
TMP_8110(uint256) = reserveA_1 (c)- amount0Out_1
newReserveA_1(uint256) := TMP_8110(uint256)
 newReserveB = reserveB + amount1In
TMP_8111(uint256) = reserveB_1 (c)+ amount1In_1
newReserveB_1(uint256) := TMP_8111(uint256)
 duration = block.timestamp - tokenInfo[tokenAddress].data.lastUpdated
REF_3226(Bonding.Token) -> tokenInfo_19[tokenAddress_1]
REF_3227(Bonding.Data) -> REF_3226.data
REF_3228(uint256) -> REF_3227.lastUpdated
TMP_8112(uint256) = block.timestamp (c)- REF_3228
duration_1(uint256) := TMP_8112(uint256)
 liquidity = newReserveB * 2
TMP_8113(uint256) = newReserveB_1 (c)* 2
liquidity_1(uint256) := TMP_8113(uint256)
 mCap = (tokenInfo[tokenAddress].data.supply * newReserveB) / newReserveA
REF_3229(Bonding.Token) -> tokenInfo_19[tokenAddress_1]
REF_3230(Bonding.Data) -> REF_3229.data
REF_3231(uint256) -> REF_3230.supply
TMP_8114(uint256) = REF_3231 (c)* newReserveB_1
TMP_8115(uint256) = TMP_8114 (c)/ newReserveA_1
mCap_1(uint256) := TMP_8115(uint256)
 price = newReserveA / newReserveB
TMP_8116(uint256) = newReserveA_1 (c)/ newReserveB_1
price_1(uint256) := TMP_8116(uint256)
 tokenInfo[tokenAddress].data.price = price
REF_3232(Bonding.Token) -> tokenInfo_19[tokenAddress_1]
REF_3233(Bonding.Data) -> REF_3232.data
REF_3234(uint256) -> REF_3233.price
tokenInfo_20(mapping(address => Bonding.Token)) := phi(['tokenInfo_19'])
REF_3234(uint256) (->tokenInfo_20) := price_1(uint256)
 tokenInfo[tokenAddress].data.marketCap = mCap
REF_3235(Bonding.Token) -> tokenInfo_20[tokenAddress_1]
REF_3236(Bonding.Data) -> REF_3235.data
REF_3237(uint256) -> REF_3236.marketCap
tokenInfo_21(mapping(address => Bonding.Token)) := phi(['tokenInfo_20'])
REF_3237(uint256) (->tokenInfo_21) := mCap_1(uint256)
 tokenInfo[tokenAddress].data.liquidity = liquidity
REF_3238(Bonding.Token) -> tokenInfo_21[tokenAddress_1]
REF_3239(Bonding.Data) -> REF_3238.data
REF_3240(uint256) -> REF_3239.liquidity
tokenInfo_22(mapping(address => Bonding.Token)) := phi(['tokenInfo_21'])
REF_3240(uint256) (->tokenInfo_22) := liquidity_1(uint256)
 tokenInfo[tokenAddress].data.volume = tokenInfo[tokenAddress].data.volume + amount1In
REF_3241(Bonding.Token) -> tokenInfo_22[tokenAddress_1]
REF_3242(Bonding.Data) -> REF_3241.data
REF_3243(uint256) -> REF_3242.volume
REF_3244(Bonding.Token) -> tokenInfo_22[tokenAddress_1]
REF_3245(Bonding.Data) -> REF_3244.data
REF_3246(uint256) -> REF_3245.volume
TMP_8117(uint256) = REF_3246 (c)+ amount1In_1
tokenInfo_23(mapping(address => Bonding.Token)) := phi(['tokenInfo_22'])
REF_3243(uint256) (->tokenInfo_23) := TMP_8117(uint256)
 tokenInfo[tokenAddress].data.volume24H = volume
REF_3247(Bonding.Token) -> tokenInfo_23[tokenAddress_1]
REF_3248(Bonding.Data) -> REF_3247.data
REF_3249(uint256) -> REF_3248.volume24H
tokenInfo_24(mapping(address => Bonding.Token)) := phi(['tokenInfo_23'])
REF_3249(uint256) (->tokenInfo_24) := volume_3(uint256)
 tokenInfo[tokenAddress].data.prevPrice = _price
REF_3250(Bonding.Token) -> tokenInfo_24[tokenAddress_1]
REF_3251(Bonding.Data) -> REF_3250.data
REF_3252(uint256) -> REF_3251.prevPrice
tokenInfo_25(mapping(address => Bonding.Token)) := phi(['tokenInfo_24'])
REF_3252(uint256) (->tokenInfo_25) := _price_3(uint256)
 duration > 86400
TMP_8118(bool) = duration_1 > 86400
CONDITION TMP_8118
 tokenInfo[tokenAddress].data.lastUpdated = block.timestamp
REF_3253(Bonding.Token) -> tokenInfo_25[tokenAddress_1]
REF_3254(Bonding.Data) -> REF_3253.data
REF_3255(uint256) -> REF_3254.lastUpdated
tokenInfo_26(mapping(address => Bonding.Token)) := phi(['tokenInfo_25'])
REF_3255(uint256) (->tokenInfo_26) := block.timestamp(uint256)
tokenInfo_27(mapping(address => Bonding.Token)) := phi(['tokenInfo_26', 'tokenInfo_25'])
 newReserveA <= gradThreshold && tokenInfo[tokenAddress].trading
TMP_8119(bool) = newReserveA_1 <= gradThreshold_7
REF_3256(Bonding.Token) -> tokenInfo_27[tokenAddress_1]
REF_3257(bool) -> REF_3256.trading
TMP_8120(bool) = TMP_8119 && REF_3257
CONDITION TMP_8120
 _openTradingOnUniswap(tokenAddress)
INTERNAL_CALL, Bonding._openTradingOnUniswap(address)(tokenAddress_1)
 true
RETURN True
 duration > 86400
TMP_8122(bool) = duration_1 > 86400
CONDITION TMP_8122
 volume = amount1In
volume_1(uint256) := amount1In_1(uint256)
 volume = tokenInfo[tokenAddress].data.volume24H + amount1In
REF_3258(Bonding.Token) -> tokenInfo_19[tokenAddress_1]
REF_3259(Bonding.Data) -> REF_3258.data
REF_3260(uint256) -> REF_3259.volume24H
TMP_8123(uint256) = REF_3260 (c)+ amount1In_1
volume_2(uint256) := TMP_8123(uint256)
volume_3(uint256) := phi(['volume_1', 'volume_2'])
 duration > 86400
TMP_8124(bool) = duration_1 > 86400
CONDITION TMP_8124
 _price = tokenInfo[tokenAddress].data.price
REF_3261(Bonding.Token) -> tokenInfo_19[tokenAddress_1]
REF_3262(Bonding.Data) -> REF_3261.data
REF_3263(uint256) -> REF_3262.price
_price_1(uint256) := REF_3263(uint256)
 _price = tokenInfo[tokenAddress].data.prevPrice
REF_3264(Bonding.Token) -> tokenInfo_19[tokenAddress_1]
REF_3265(Bonding.Data) -> REF_3264.data
REF_3266(uint256) -> REF_3265.prevPrice
_price_2(uint256) := REF_3266(uint256)
_price_3(uint256) := phi(['_price_1', '_price_2'])
```
#### Bonding.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### Bonding.getUserTokens(address) [PUBLIC]
```slithir
profile_3(mapping(address => Bonding.Profile)) := phi(['profile_16', 'profile_2', 'profile_4', 'profile_15', 'profile_1', 'profile_0', 'profile_14'])
 require(bool,string)(_checkIfProfileExists(account),User Profile dose not exist.)
TMP_8021(bool) = INTERNAL_CALL, Bonding._checkIfProfileExists(address)(account_1)
profile_4(mapping(address => Bonding.Profile)) := phi(['profile_2'])
TMP_8022(None) = SOLIDITY_CALL require(bool,string)(TMP_8021,User Profile dose not exist.)
 _profile = profile[account]
REF_3141(Bonding.Profile) -> profile_4[account_1]
_profile_1(Bonding.Profile) := REF_3141(Bonding.Profile)
 _profile.tokens
REF_3142(address[]) -> _profile_1.tokens
RETURN REF_3142
```
#### Bonding.initialize(address,address,address,uint256,uint256,uint256,uint256,address,uint256) [EXTERNAL]
```slithir
 __Ownable_init(msg.sender)
INTERNAL_CALL, OwnableUpgradeable.__Ownable_init(address)(msg.sender)
 __ReentrancyGuard_init()
INTERNAL_CALL, ReentrancyGuardUpgradeable.__ReentrancyGuard_init()()
 factory = FFactory(factory_)
TMP_8001 = CONVERT factory__1 to FFactory
factory_1(FFactory) := TMP_8001(FFactory)
 router = FRouter(router_)
TMP_8002 = CONVERT router__1 to FRouter
router_1(FRouter) := TMP_8002(FRouter)
 _feeTo = feeTo_
_feeTo_1(address) := feeTo__1(address)
 fee = (fee_ * 1000000000000000000) / 1000
TMP_8003(uint256) = fee__1 (c)* 1000000000000000000
TMP_8004(uint256) = TMP_8003 (c)/ 1000
fee_1(uint256) := TMP_8004(uint256)
 initialSupply = initialSupply_
initialSupply_1(uint256) := initialSupply__1(uint256)
 assetRate = assetRate_
assetRate_1(uint256) := assetRate__1(uint256)
 maxTx = maxTx_
maxTx_1(uint256) := maxTx__1(uint256)
 agentFactory = agentFactory_
agentFactory_1(address) := agentFactory__1(address)
 gradThreshold = gradThreshold_
gradThreshold_1(uint256) := gradThreshold__1(uint256)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### Bonding.launch(string,string,uint8[],string,string,string[4],uint256) [PUBLIC]
```slithir
 launchFor(_name,_ticker,cores,desc,img,urls,purchaseAmount,msg.sender)
TUPLE_84(address,address,uint256) = INTERNAL_CALL, Bonding.launchFor(string,string,uint8[],string,string,string[4],uint256,address)(_name_1,_ticker_1,cores_1,desc_1,img_1,urls_1,purchaseAmount_1,msg.sender)
RETURN TUPLE_84
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### Bonding.launchFor(string,string,uint8[],string,string,string[4],uint256,address) [PUBLIC]
```slithir
_name_1(string) := phi(['_name_1'])
_ticker_1(string) := phi(['_ticker_1'])
cores_1(uint8[]) := phi(['cores_1'])
desc_1(string) := phi(['desc_1'])
img_1(string) := phi(['img_1'])
urls_1(string[4]) := phi(['urls_1'])
purchaseAmount_1(uint256) := phi(['purchaseAmount_1'])
creator_1(address) := phi(['msg.sender'])
_feeTo_3(address) := phi(['_feeTo_0', '_feeTo_1', '_feeTo_2', '_feeTo_6'])
factory_2(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_13', 'factory_7', 'factory_0', 'factory_10'])
router_2(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_17', 'router_0'])
initialSupply_3(uint256) := phi(['initialSupply_2', 'initialSupply_1', 'initialSupply_6', 'initialSupply_0'])
fee_3(uint256) := phi(['fee_1', 'fee_0', 'fee_2', 'fee_6'])
K_1(uint256) := phi(['K_7', 'K_0'])
assetRate_3(uint256) := phi(['assetRate_0', 'assetRate_1', 'assetRate_9', 'assetRate_2'])
maxTx_3(uint256) := phi(['maxTx_6', 'maxTx_2', 'maxTx_1', 'maxTx_0'])
profile_5(mapping(address => Bonding.Profile)) := phi(['profile_16', 'profile_2', 'profile_4', 'profile_15', 'profile_1', 'profile_0', 'profile_14'])
tokenInfos_1(address[]) := phi(['tokenInfos_11', 'tokenInfos_0'])
 require(bool,string)(purchaseAmount > fee,Purchase amount must be greater than fee)
TMP_8024(bool) = purchaseAmount_1 > fee_4
TMP_8025(None) = SOLIDITY_CALL require(bool,string)(TMP_8024,Purchase amount must be greater than fee)
 assetToken = router.assetToken()
TMP_8026(address) = HIGH_LEVEL_CALL, dest:router_3(FRouter), function:assetToken, arguments:[]  
_feeTo_5(address) := phi(['_feeTo_4', '_feeTo_1', '_feeTo_2', '_feeTo_6'])
factory_4(FFactory) := phi(['factory_19', 'factory_3', 'factory_16', 'factory_1', 'factory_13', 'factory_7', 'factory_10'])
router_4(FRouter) := phi(['router_22', 'router_38', 'router_3', 'router_12', 'router_35', 'router_1', 'router_17'])
initialSupply_5(uint256) := phi(['initialSupply_6', 'initialSupply_1', 'initialSupply_4', 'initialSupply_2'])
fee_5(uint256) := phi(['fee_1', 'fee_4', 'fee_2', 'fee_6'])
K_3(uint256) := phi(['K_2', 'K_7'])
assetRate_5(uint256) := phi(['assetRate_1', 'assetRate_9', 'assetRate_4', 'assetRate_2'])
maxTx_5(uint256) := phi(['maxTx_6', 'maxTx_2', 'maxTx_1', 'maxTx_4'])
profile_7(mapping(address => Bonding.Profile)) := phi(['profile_6', 'profile_16', 'profile_2', 'profile_4', 'profile_15', 'profile_1', 'profile_14'])
tokenInfos_3(address[]) := phi(['tokenInfos_2', 'tokenInfos_11'])
assetToken_1(address) := TMP_8026(address)
 require(bool,string)(IERC20(assetToken).balanceOf(msg.sender) >= purchaseAmount,Insufficient amount)
TMP_8027 = CONVERT assetToken_1 to IERC20
TMP_8028(uint256) = HIGH_LEVEL_CALL, dest:TMP_8027(IERC20), function:balanceOf, arguments:['msg.sender']  
_feeTo_6(address) := phi(['_feeTo_5', '_feeTo_1', '_feeTo_2', '_feeTo_6'])
factory_5(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_13', 'factory_7', 'factory_10', 'factory_4'])
router_5(FRouter) := phi(['router_4', 'router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_17'])
initialSupply_6(uint256) := phi(['initialSupply_6', 'initialSupply_5', 'initialSupply_1', 'initialSupply_2'])
fee_6(uint256) := phi(['fee_1', 'fee_2', 'fee_5', 'fee_6'])
K_4(uint256) := phi(['K_3', 'K_7'])
assetRate_6(uint256) := phi(['assetRate_5', 'assetRate_1', 'assetRate_9', 'assetRate_2'])
maxTx_6(uint256) := phi(['maxTx_6', 'maxTx_2', 'maxTx_1', 'maxTx_5'])
profile_8(mapping(address => Bonding.Profile)) := phi(['profile_7', 'profile_16', 'profile_2', 'profile_4', 'profile_15', 'profile_1', 'profile_14'])
tokenInfos_4(address[]) := phi(['tokenInfos_11', 'tokenInfos_3'])
TMP_8029(bool) = TMP_8028 >= purchaseAmount_1
TMP_8030(None) = SOLIDITY_CALL require(bool,string)(TMP_8029,Insufficient amount)
 initialPurchase = (purchaseAmount - fee)
TMP_8031(uint256) = purchaseAmount_1 (c)- fee_6
initialPurchase_1(uint256) := TMP_8031(uint256)
 IERC20(assetToken).safeTransferFrom(msg.sender,_feeTo,fee)
TMP_8032 = CONVERT assetToken_1 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_8032', 'msg.sender', '_feeTo_6', 'fee_6'] 
 IERC20(assetToken).safeTransferFrom(msg.sender,address(this),initialPurchase)
TMP_8034 = CONVERT assetToken_1 to IERC20
TMP_8035 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_8034', 'msg.sender', 'TMP_8035', 'initialPurchase_1'] 
 token = new FERC20(string.concat(fun ,_name),_ticker,initialSupply,maxTx)
TMP_8038(string) = SOLIDITY_CALL string.concat()(fun ,_name_1)
TMP_8039(FERC20) = new FERC20(TMP_8038,_ticker_1,initialSupply_6,maxTx_6) 
token_1(FERC20) := TMP_8039(FERC20)
 supply = token.totalSupply()
TMP_8040(uint256) = HIGH_LEVEL_CALL, dest:token_1(FERC20), function:totalSupply, arguments:[]  
factory_6(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_5', 'factory_13', 'factory_7', 'factory_10'])
router_6(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_5', 'router_1', 'router_17'])
K_5(uint256) := phi(['K_4', 'K_7'])
assetRate_7(uint256) := phi(['assetRate_6', 'assetRate_1', 'assetRate_9', 'assetRate_2'])
profile_9(mapping(address => Bonding.Profile)) := phi(['profile_16', 'profile_2', 'profile_4', 'profile_15', 'profile_1', 'profile_8', 'profile_14'])
tokenInfos_5(address[]) := phi(['tokenInfos_11', 'tokenInfos_4'])
supply_1(uint256) := TMP_8040(uint256)
 _pair = factory.createPair(address(token),assetToken)
TMP_8041 = CONVERT token_1 to address
TMP_8042(address) = HIGH_LEVEL_CALL, dest:factory_6(FFactory), function:createPair, arguments:['TMP_8041', 'assetToken_1']  
factory_7(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_13', 'factory_7', 'factory_10', 'factory_6'])
router_7(FRouter) := phi(['router_22', 'router_6', 'router_38', 'router_12', 'router_35', 'router_1', 'router_17'])
K_6(uint256) := phi(['K_5', 'K_7'])
assetRate_8(uint256) := phi(['assetRate_7', 'assetRate_1', 'assetRate_9', 'assetRate_2'])
profile_10(mapping(address => Bonding.Profile)) := phi(['profile_16', 'profile_2', 'profile_4', 'profile_15', 'profile_1', 'profile_9', 'profile_14'])
tokenInfos_6(address[]) := phi(['tokenInfos_11', 'tokenInfos_5'])
_pair_1(address) := TMP_8042(address)
 approved = _approval(address(router),address(token),supply)
TMP_8043 = CONVERT router_7 to address
TMP_8044 = CONVERT token_1 to address
TMP_8045(bool) = INTERNAL_CALL, Bonding._approval(address,address,uint256)(TMP_8043,TMP_8044,supply_1)
approved_1(bool) := TMP_8045(bool)
 require(bool)(approved)
TMP_8046(None) = SOLIDITY_CALL require(bool)(approved_1)
 k = ((K * 10000) / assetRate)
TMP_8047(uint256) = K_7 (c)* 10000
TMP_8048(uint256) = TMP_8047 (c)/ assetRate_9
k_1(uint256) := TMP_8048(uint256)
 liquidity = (((k * 10000000000000000000000) / supply) * 1000000000000000000) / 10000
TMP_8049(uint256) = k_1 (c)* 10000000000000000000000
TMP_8050(uint256) = TMP_8049 (c)/ supply_1
TMP_8051(uint256) = TMP_8050 (c)* 1000000000000000000
TMP_8052(uint256) = TMP_8051 (c)/ 10000
liquidity_1(uint256) := TMP_8052(uint256)
 router.addInitialLiquidity(address(token),supply,liquidity)
TMP_8053 = CONVERT token_1 to address
TUPLE_85(uint256,uint256) = HIGH_LEVEL_CALL, dest:router_8(FRouter), function:addInitialLiquidity, arguments:['TMP_8053', 'supply_1', 'liquidity_1']  
router_9(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_8', 'router_17'])
profile_12(mapping(address => Bonding.Profile)) := phi(['profile_16', 'profile_11', 'profile_2', 'profile_4', 'profile_15', 'profile_1', 'profile_14'])
tokenInfos_8(address[]) := phi(['tokenInfos_11', 'tokenInfos_7'])
 _data = Data({token:address(token),name:string.concat(fun ,_name),_name:_name,ticker:_ticker,supply:supply,price:supply / liquidity,marketCap:liquidity,liquidity:liquidity * 2,volume:0,volume24H:0,prevPrice:supply / liquidity,lastUpdated:block.timestamp})
TMP_8054 = CONVERT token_1 to address
TMP_8055(string) = SOLIDITY_CALL string.concat()(fun ,_name_1)
TMP_8056(uint256) = supply_1 (c)/ liquidity_1
TMP_8057(uint256) = liquidity_1 (c)* 2
TMP_8058(uint256) = supply_1 (c)/ liquidity_1
TMP_8059(Bonding.Data) = new Data(TMP_8054,TMP_8055,_name_1,_ticker_1,supply_1,TMP_8056,liquidity_1,TMP_8057,0,0,TMP_8058,block.timestamp)
_data_1(Bonding.Data) := TMP_8059(Bonding.Data)
 tmpToken = Token({creator:creator,token:address(token),agentToken:address(0),pair:_pair,data:_data,description:desc,cores:cores,image:img,twitter:urls[0],telegram:urls[1],youtube:urls[2],website:urls[3],trading:true,tradingOnUniswap:false})
TMP_8060 = CONVERT token_1 to address
TMP_8061 = CONVERT 0 to address
REF_3152(string) -> urls_1[0]
REF_3153(string) -> urls_1[1]
REF_3154(string) -> urls_1[2]
REF_3155(string) -> urls_1[3]
TMP_8062(Bonding.Token) = new Token(creator_1,TMP_8060,_pair_1,TMP_8061,_data_1,desc_1,cores_1,img_1,REF_3152,REF_3153,REF_3154,REF_3155,True,False)
tmpToken_1(Bonding.Token) := TMP_8062(Bonding.Token)
 tokenInfo[address(token)] = tmpToken
TMP_8063 = CONVERT token_1 to address
REF_3156(Bonding.Token) -> tokenInfo_0[TMP_8063]
tokenInfo_1(mapping(address => Bonding.Token)) := phi(['tokenInfo_0'])
REF_3156(Bonding.Token) (->tokenInfo_1) := tmpToken_1(Bonding.Token)
 tokenInfos.push(address(token))
TMP_8064 = CONVERT token_1 to address
REF_3158 -> LENGTH tokenInfos_8
TMP_8066(uint256) := REF_3158(uint256)
TMP_8067(uint256) = TMP_8066 (c)+ 1
tokenInfos_9(address[]) := phi(['tokenInfos_8'])
REF_3158(uint256) (->tokenInfos_9) := TMP_8067(uint256)
REF_3159(address) -> tokenInfos_9[TMP_8066]
tokenInfos_10(address[]) := phi(['tokenInfos_9'])
REF_3159(address) (->tokenInfos_10) := TMP_8064(address)
 exists = _checkIfProfileExists(creator)
TMP_8068(bool) = INTERNAL_CALL, Bonding._checkIfProfileExists(address)(creator_1)
profile_13(mapping(address => Bonding.Profile)) := phi(['profile_2'])
exists_1(bool) := TMP_8068(bool)
 exists
CONDITION exists_1
 _profile = profile[creator]
REF_3160(Bonding.Profile) -> profile_13[creator_1]
_profile_1 (-> ['profile'])(Bonding.Profile) := REF_3160(Bonding.Profile)
 _profile.tokens.push(address(token))
REF_3161(address[]) -> _profile_1 (-> ['profile']).tokens
TMP_8069 = CONVERT token_1 to address
REF_3163 -> LENGTH REF_3161
TMP_8071(uint256) := REF_3163(uint256)
TMP_8072(uint256) = TMP_8071 (c)+ 1
_profile_2 (-> ['profile'])(Bonding.Profile) := phi(["_profile_1 (-> ['profile'])"])
REF_3163(uint256) (->_profile_3 (-> ['profile'])) := TMP_8072(uint256)
REF_3164(address) -> REF_3161[TMP_8071]
_profile_3 (-> ['profile'])(Bonding.Profile) := phi(["_profile_2 (-> ['profile'])"])
REF_3164(address) (->_profile_3 (-> ['profile'])) := TMP_8069(address)
profile_16(mapping(address => Bonding.Profile)) := phi(["_profile_3 (-> ['profile'])"])
 created = _createUserProfile(creator)
TMP_8073(bool) = INTERNAL_CALL, Bonding._createUserProfile(address)(creator_1)
profile_14(mapping(address => Bonding.Profile)) := phi(['profile_1'])
created_1(bool) := TMP_8073(bool)
 created
CONDITION created_1
 _profile_scope_0 = profile[creator]
REF_3165(Bonding.Profile) -> profile_14[creator_1]
_profile_scope_0_1 (-> ['profile'])(Bonding.Profile) := REF_3165(Bonding.Profile)
 _profile_scope_0.tokens.push(address(token))
REF_3166(address[]) -> _profile_scope_0_1 (-> ['profile']).tokens
TMP_8074 = CONVERT token_1 to address
REF_3168 -> LENGTH REF_3166
TMP_8076(uint256) := REF_3168(uint256)
TMP_8077(uint256) = TMP_8076 (c)+ 1
_profile_scope_0_2 (-> ['profile'])(Bonding.Profile) := phi(["_profile_scope_0_1 (-> ['profile'])"])
REF_3168(uint256) (->_profile_scope_0_3 (-> ['profile'])) := TMP_8077(uint256)
REF_3169(address) -> REF_3166[TMP_8076]
_profile_scope_0_3 (-> ['profile'])(Bonding.Profile) := phi(["_profile_scope_0_2 (-> ['profile'])"])
REF_3169(address) (->_profile_scope_0_3 (-> ['profile'])) := TMP_8074(address)
profile_15(mapping(address => Bonding.Profile)) := phi(["_profile_scope_0_3 (-> ['profile'])"])
 n = tokenInfos.length
REF_3170 -> LENGTH tokenInfos_11
n_1(uint256) := REF_3170(uint256)
 Launched(address(token),_pair,n)
TMP_8078 = CONVERT token_1 to address
Emit Launched(TMP_8078,_pair_1,n_1)
 IERC20(assetToken).forceApprove(address(router),initialPurchase)
TMP_8080 = CONVERT assetToken_1 to IERC20
TMP_8081 = CONVERT router_10 to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.forceApprove(IERC20,address,uint256), arguments:['TMP_8080', 'TMP_8081', 'initialPurchase_1'] 
 router.buy(initialPurchase,address(token),address(this))
TMP_8083 = CONVERT token_1 to address
TMP_8084 = CONVERT this to address
TUPLE_86(uint256,uint256) = HIGH_LEVEL_CALL, dest:router_10(FRouter), function:buy, arguments:['initialPurchase_1', 'TMP_8083', 'TMP_8084']  
router_12(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_11', 'router_17'])
 token.transfer(msg.sender,token.balanceOf(address(this)))
TMP_8085 = CONVERT this to address
TMP_8086(uint256) = HIGH_LEVEL_CALL, dest:token_1(FERC20), function:balanceOf, arguments:['TMP_8085']  
TMP_8087(bool) = HIGH_LEVEL_CALL, dest:token_1(FERC20), function:transfer, arguments:['msg.sender', 'TMP_8086']  
 (address(token),_pair,n)
TMP_8088 = CONVERT token_1 to address
RETURN TMP_8088,_pair_1,n_1
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### Bonding.sell(uint256,address) [PUBLIC]
```slithir
factory_8(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_13', 'factory_7', 'factory_0', 'factory_10'])
router_13(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_17', 'router_0'])
tokenInfo_2(mapping(address => Bonding.Token)) := phi(['tokenInfo_1', 'tokenInfo_27', 'tokenInfo_14', 'tokenInfo_31', 'tokenInfo_32', 'tokenInfo_0'])
 require(bool,string)(tokenInfo[tokenAddress].trading,Token not trading)
REF_3175(Bonding.Token) -> tokenInfo_2[tokenAddress_1]
REF_3176(bool) -> REF_3175.trading
TMP_8090(None) = SOLIDITY_CALL require(bool,string)(REF_3176,Token not trading)
 pairAddress = factory.getPair(tokenAddress,router.assetToken())
TMP_8091(address) = HIGH_LEVEL_CALL, dest:router_13(FRouter), function:assetToken, arguments:[]  
factory_9(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_8', 'factory_13', 'factory_7', 'factory_10'])
router_14(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_13', 'router_1', 'router_17'])
tokenInfo_3(mapping(address => Bonding.Token)) := phi(['tokenInfo_1', 'tokenInfo_27', 'tokenInfo_2', 'tokenInfo_14', 'tokenInfo_31', 'tokenInfo_32'])
TMP_8092(address) = HIGH_LEVEL_CALL, dest:factory_9(FFactory), function:getPair, arguments:['tokenAddress_1', 'TMP_8091']  
factory_10(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_13', 'factory_9', 'factory_7', 'factory_10'])
router_15(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_17', 'router_14'])
tokenInfo_4(mapping(address => Bonding.Token)) := phi(['tokenInfo_1', 'tokenInfo_27', 'tokenInfo_3', 'tokenInfo_14', 'tokenInfo_31', 'tokenInfo_32'])
pairAddress_1(address) := TMP_8092(address)
 pair = IFPair(pairAddress)
TMP_8093 = CONVERT pairAddress_1 to IFPair
pair_1(IFPair) := TMP_8093(IFPair)
 (reserveA,reserveB) = pair.getReserves()
TUPLE_87(uint256,uint256) = HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:getReserves, arguments:[]  
router_16(FRouter) := phi(['router_15', 'router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_17'])
tokenInfo_5(mapping(address => Bonding.Token)) := phi(['tokenInfo_1', 'tokenInfo_27', 'tokenInfo_4', 'tokenInfo_14', 'tokenInfo_31', 'tokenInfo_32'])
reserveA_1(uint256)= UNPACK TUPLE_87 index: 0 
reserveB_1(uint256)= UNPACK TUPLE_87 index: 1 
 (amount0In,amount1Out) = router.sell(amountIn,tokenAddress,msg.sender)
TUPLE_88(uint256,uint256) = HIGH_LEVEL_CALL, dest:router_16(FRouter), function:sell, arguments:['amountIn_1', 'tokenAddress_1', 'msg.sender']  
router_17(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_16', 'router_17'])
tokenInfo_6(mapping(address => Bonding.Token)) := phi(['tokenInfo_1', 'tokenInfo_27', 'tokenInfo_14', 'tokenInfo_31', 'tokenInfo_5', 'tokenInfo_32'])
amount0In_1(uint256)= UNPACK TUPLE_88 index: 0 
amount1Out_1(uint256)= UNPACK TUPLE_88 index: 1 
 newReserveA = reserveA + amount0In
TMP_8094(uint256) = reserveA_1 (c)+ amount0In_1
newReserveA_1(uint256) := TMP_8094(uint256)
 newReserveB = reserveB - amount1Out
TMP_8095(uint256) = reserveB_1 (c)- amount1Out_1
newReserveB_1(uint256) := TMP_8095(uint256)
 duration = block.timestamp - tokenInfo[tokenAddress].data.lastUpdated
REF_3181(Bonding.Token) -> tokenInfo_6[tokenAddress_1]
REF_3182(Bonding.Data) -> REF_3181.data
REF_3183(uint256) -> REF_3182.lastUpdated
TMP_8096(uint256) = block.timestamp (c)- REF_3183
duration_1(uint256) := TMP_8096(uint256)
 liquidity = newReserveB * 2
TMP_8097(uint256) = newReserveB_1 (c)* 2
liquidity_1(uint256) := TMP_8097(uint256)
 mCap = (tokenInfo[tokenAddress].data.supply * newReserveB) / newReserveA
REF_3184(Bonding.Token) -> tokenInfo_6[tokenAddress_1]
REF_3185(Bonding.Data) -> REF_3184.data
REF_3186(uint256) -> REF_3185.supply
TMP_8098(uint256) = REF_3186 (c)* newReserveB_1
TMP_8099(uint256) = TMP_8098 (c)/ newReserveA_1
mCap_1(uint256) := TMP_8099(uint256)
 price = newReserveA / newReserveB
TMP_8100(uint256) = newReserveA_1 (c)/ newReserveB_1
price_1(uint256) := TMP_8100(uint256)
 tokenInfo[tokenAddress].data.price = price
REF_3187(Bonding.Token) -> tokenInfo_6[tokenAddress_1]
REF_3188(Bonding.Data) -> REF_3187.data
REF_3189(uint256) -> REF_3188.price
tokenInfo_7(mapping(address => Bonding.Token)) := phi(['tokenInfo_6'])
REF_3189(uint256) (->tokenInfo_7) := price_1(uint256)
 tokenInfo[tokenAddress].data.marketCap = mCap
REF_3190(Bonding.Token) -> tokenInfo_7[tokenAddress_1]
REF_3191(Bonding.Data) -> REF_3190.data
REF_3192(uint256) -> REF_3191.marketCap
tokenInfo_8(mapping(address => Bonding.Token)) := phi(['tokenInfo_7'])
REF_3192(uint256) (->tokenInfo_8) := mCap_1(uint256)
 tokenInfo[tokenAddress].data.liquidity = liquidity
REF_3193(Bonding.Token) -> tokenInfo_8[tokenAddress_1]
REF_3194(Bonding.Data) -> REF_3193.data
REF_3195(uint256) -> REF_3194.liquidity
tokenInfo_9(mapping(address => Bonding.Token)) := phi(['tokenInfo_8'])
REF_3195(uint256) (->tokenInfo_9) := liquidity_1(uint256)
 tokenInfo[tokenAddress].data.volume = tokenInfo[tokenAddress].data.volume + amount1Out
REF_3196(Bonding.Token) -> tokenInfo_9[tokenAddress_1]
REF_3197(Bonding.Data) -> REF_3196.data
REF_3198(uint256) -> REF_3197.volume
REF_3199(Bonding.Token) -> tokenInfo_9[tokenAddress_1]
REF_3200(Bonding.Data) -> REF_3199.data
REF_3201(uint256) -> REF_3200.volume
TMP_8101(uint256) = REF_3201 (c)+ amount1Out_1
tokenInfo_10(mapping(address => Bonding.Token)) := phi(['tokenInfo_9'])
REF_3198(uint256) (->tokenInfo_10) := TMP_8101(uint256)
 tokenInfo[tokenAddress].data.volume24H = volume
REF_3202(Bonding.Token) -> tokenInfo_10[tokenAddress_1]
REF_3203(Bonding.Data) -> REF_3202.data
REF_3204(uint256) -> REF_3203.volume24H
tokenInfo_11(mapping(address => Bonding.Token)) := phi(['tokenInfo_10'])
REF_3204(uint256) (->tokenInfo_11) := volume_3(uint256)
 tokenInfo[tokenAddress].data.prevPrice = prevPrice
REF_3205(Bonding.Token) -> tokenInfo_11[tokenAddress_1]
REF_3206(Bonding.Data) -> REF_3205.data
REF_3207(uint256) -> REF_3206.prevPrice
tokenInfo_12(mapping(address => Bonding.Token)) := phi(['tokenInfo_11'])
REF_3207(uint256) (->tokenInfo_12) := prevPrice_3(uint256)
 duration > 86400
TMP_8102(bool) = duration_1 > 86400
CONDITION TMP_8102
 tokenInfo[tokenAddress].data.lastUpdated = block.timestamp
REF_3208(Bonding.Token) -> tokenInfo_12[tokenAddress_1]
REF_3209(Bonding.Data) -> REF_3208.data
REF_3210(uint256) -> REF_3209.lastUpdated
tokenInfo_13(mapping(address => Bonding.Token)) := phi(['tokenInfo_12'])
REF_3210(uint256) (->tokenInfo_13) := block.timestamp(uint256)
tokenInfo_14(mapping(address => Bonding.Token)) := phi(['tokenInfo_13', 'tokenInfo_12'])
 true
RETURN True
 duration > 86400
TMP_8103(bool) = duration_1 > 86400
CONDITION TMP_8103
 volume = amount1Out
volume_1(uint256) := amount1Out_1(uint256)
 volume = tokenInfo[tokenAddress].data.volume24H + amount1Out
REF_3211(Bonding.Token) -> tokenInfo_6[tokenAddress_1]
REF_3212(Bonding.Data) -> REF_3211.data
REF_3213(uint256) -> REF_3212.volume24H
TMP_8104(uint256) = REF_3213 (c)+ amount1Out_1
volume_2(uint256) := TMP_8104(uint256)
volume_3(uint256) := phi(['volume_1', 'volume_2'])
 duration > 86400
TMP_8105(bool) = duration_1 > 86400
CONDITION TMP_8105
 prevPrice = tokenInfo[tokenAddress].data.price
REF_3214(Bonding.Token) -> tokenInfo_6[tokenAddress_1]
REF_3215(Bonding.Data) -> REF_3214.data
REF_3216(uint256) -> REF_3215.price
prevPrice_1(uint256) := REF_3216(uint256)
 prevPrice = tokenInfo[tokenAddress].data.prevPrice
REF_3217(Bonding.Token) -> tokenInfo_6[tokenAddress_1]
REF_3218(Bonding.Data) -> REF_3217.data
REF_3219(uint256) -> REF_3218.prevPrice
prevPrice_2(uint256) := REF_3219(uint256)
prevPrice_3(uint256) := phi(['prevPrice_1', 'prevPrice_2'])
```
#### Bonding.setAssetRate(uint256) [PUBLIC][OWNER]
```slithir
 require(bool,string)(newRate > 0,Rate err)
TMP_8017(bool) = newRate_1 > 0
TMP_8018(None) = SOLIDITY_CALL require(bool,string)(TMP_8017,Rate err)
 assetRate = newRate
assetRate_2(uint256) := newRate_1(uint256)
 onlyOwner()
MODIFIER_CALL, OwnableUpgradeable.onlyOwner()()
```
#### Bonding.setDeployParams(Bonding.DeployParams) [PUBLIC][OWNER]
```slithir
 _deployParams = params
_deployParams_1(Bonding.DeployParams) := params_1(Bonding.DeployParams)
 onlyOwner()
MODIFIER_CALL, OwnableUpgradeable.onlyOwner()()
```
#### Bonding.setFee(uint256,address) [PUBLIC][OWNER]
```slithir
 fee = newFee
fee_2(uint256) := newFee_1(uint256)
 _feeTo = newFeeTo
_feeTo_2(address) := newFeeTo_1(address)
 onlyOwner()
MODIFIER_CALL, OwnableUpgradeable.onlyOwner()()
```
#### Bonding.setGradThreshold(uint256) [PUBLIC][OWNER]
```slithir
 gradThreshold = newThreshold
gradThreshold_2(uint256) := newThreshold_1(uint256)
 onlyOwner()
MODIFIER_CALL, OwnableUpgradeable.onlyOwner()()
```
#### Bonding.setInitialSupply(uint256) [PUBLIC][OWNER]
```slithir
 initialSupply = newSupply
initialSupply_2(uint256) := newSupply_1(uint256)
 onlyOwner()
MODIFIER_CALL, OwnableUpgradeable.onlyOwner()()
```
#### Bonding.setMaxTx(uint256) [PUBLIC][OWNER]
```slithir
 maxTx = maxTx_
maxTx_2(uint256) := maxTx__1(uint256)
 onlyOwner()
MODIFIER_CALL, OwnableUpgradeable.onlyOwner()()
```

#### Bonding.unwrapToken(address,address[]) [PUBLIC]
```slithir
factory_17(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_13', 'factory_7', 'factory_0', 'factory_10'])
router_36(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_17', 'router_0'])
tokenInfo_32(mapping(address => Bonding.Token)) := phi(['tokenInfo_1', 'tokenInfo_27', 'tokenInfo_14', 'tokenInfo_31', 'tokenInfo_32', 'tokenInfo_0'])
 info = tokenInfo[srcTokenAddress]
REF_3300(Bonding.Token) -> tokenInfo_32[srcTokenAddress_1]
info_1(Bonding.Token) := REF_3300(Bonding.Token)
 require(bool,string)(info.tradingOnUniswap,Token is not graduated yet)
REF_3301(bool) -> info_1.tradingOnUniswap
TMP_8155(None) = SOLIDITY_CALL require(bool,string)(REF_3301,Token is not graduated yet)
 token = FERC20(srcTokenAddress)
TMP_8156 = CONVERT srcTokenAddress_1 to FERC20
token_1(FERC20) := TMP_8156(FERC20)
 agentToken = IERC20(info.agentToken)
REF_3302(address) -> info_1.agentToken
TMP_8157 = CONVERT REF_3302 to IERC20
agentToken_1(IERC20) := TMP_8157(IERC20)
 pairAddress = factory.getPair(srcTokenAddress,router.assetToken())
TMP_8158(address) = HIGH_LEVEL_CALL, dest:router_36(FRouter), function:assetToken, arguments:[]  
factory_18(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_13', 'factory_17', 'factory_7', 'factory_10'])
router_37(FRouter) := phi(['router_22', 'router_36', 'router_38', 'router_12', 'router_35', 'router_1', 'router_17'])
TMP_8159(address) = HIGH_LEVEL_CALL, dest:factory_18(FFactory), function:getPair, arguments:['srcTokenAddress_1', 'TMP_8158']  
factory_19(FFactory) := phi(['factory_19', 'factory_16', 'factory_1', 'factory_18', 'factory_13', 'factory_7', 'factory_10'])
router_38(FRouter) := phi(['router_22', 'router_38', 'router_12', 'router_35', 'router_1', 'router_37', 'router_17'])
pairAddress_1(address) := TMP_8159(address)
 i = 0
i_1(uint256) := 0(uint256)
 i < accounts.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3305 -> LENGTH accounts_1
TMP_8160(bool) = i_2 < REF_3305
CONDITION TMP_8160
 acc = accounts[i]
REF_3306(address) -> accounts_1[i_2]
acc_1(address) := REF_3306(address)
 balance = token.balanceOf(acc)
TMP_8161(uint256) = HIGH_LEVEL_CALL, dest:token_1(FERC20), function:balanceOf, arguments:['acc_1']  
balance_1(uint256) := TMP_8161(uint256)
 balance > 0
TMP_8162(bool) = balance_1 > 0
CONDITION TMP_8162
 token.burnFrom(acc,balance)
HIGH_LEVEL_CALL, dest:token_1(FERC20), function:burnFrom, arguments:['acc_1', 'balance_1']  
 agentToken.transferFrom(pairAddress,acc,balance)
TMP_8164(bool) = HIGH_LEVEL_CALL, dest:agentToken_1(IERC20), function:transferFrom, arguments:['pairAddress_1', 'acc_1', 'balance_1']  
 i ++
TMP_8165(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
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
#### IERC20.balanceOf(address) [EXTERNAL]
```slithir

```
#### FERC20.burnFrom(address,uint256) [PUBLIC][OWNER]
```slithir
_balances_8(mapping(address => uint256)) := phi(['_balances_7', '_balances_1', '_balances_0', '_balances_5', '_balances_2', '_balances_10'])
 require(bool,string)(user != address(0),Invalid address)
TMP_8262 = CONVERT 0 to address
TMP_8263(bool) = user_1 != TMP_8262
TMP_8264(None) = SOLIDITY_CALL require(bool,string)(TMP_8263,Invalid address)
 _balances[user] = _balances[user] - amount
REF_3339(uint256) -> _balances_9[user_1]
REF_3340(uint256) -> _balances_9[user_1]
TMP_8265(uint256) = REF_3340 (c)- amount_1
_balances_10(mapping(address => uint256)) := phi(['_balances_9'])
REF_3339(uint256) (->_balances_10) := TMP_8265(uint256)
 Transfer(user,address(0),amount)
TMP_8266 = CONVERT 0 to address
Emit Transfer(user_1,TMP_8266,amount_1)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### FERC20.decimals() [PUBLIC]
```slithir
_decimals_3(uint8) := phi(['_decimals_2', '_decimals_0'])
 _decimals
RETURN _decimals_3
```
#### FFactory.getPair(address,address) [PUBLIC]
```slithir
_pair_3(mapping(address => mapping(address => address))) := phi(['_pair_0', '_pair_3', '_pair_2'])
 _pair[tokenA][tokenB]
REF_3379(mapping(address => address)) -> _pair_3[tokenA_1]
REF_3380(address) -> REF_3379[tokenB_1]
RETURN REF_3380
```
#### FRouter.approval(address,address,address,uint256) [PUBLIC]
```slithir
EXECUTOR_ROLE_10(bytes32) := phi(['EXECUTOR_ROLE_0', 'EXECUTOR_ROLE_2', 'EXECUTOR_ROLE_7', 'EXECUTOR_ROLE_9', 'EXECUTOR_ROLE_11', 'EXECUTOR_ROLE_5'])
 require(bool,string)(spender != address(0),Zero addresses are not allowed.)
TMP_8621 = CONVERT 0 to address
TMP_8622(bool) = spender_1 != TMP_8621
TMP_8623(None) = SOLIDITY_CALL require(bool,string)(TMP_8622,Zero addresses are not allowed.)
 IFPair(pair).approval(spender,asset,amount)
TMP_8624 = CONVERT pair_1 to IFPair
TMP_8625(bool) = HIGH_LEVEL_CALL, dest:TMP_8624(IFPair), function:approval, arguments:['spender_1', 'asset_1', 'amount_1']  
 onlyRole(EXECUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(EXECUTOR_ROLE_10)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### FRouter.graduate(address) [PUBLIC]
```slithir
EXECUTOR_ROLE_8(bytes32) := phi(['EXECUTOR_ROLE_0', 'EXECUTOR_ROLE_2', 'EXECUTOR_ROLE_7', 'EXECUTOR_ROLE_9', 'EXECUTOR_ROLE_11', 'EXECUTOR_ROLE_5'])
factory_20(FFactory) := phi(['factory_6', 'factory_23', 'factory_0', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_20(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_0', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
 require(bool,string)(tokenAddress != address(0),Zero addresses are not allowed.)
TMP_8611 = CONVERT 0 to address
TMP_8612(bool) = tokenAddress_1 != TMP_8611
TMP_8613(None) = SOLIDITY_CALL require(bool,string)(TMP_8612,Zero addresses are not allowed.)
 pair = factory.getPair(tokenAddress,assetToken)
TMP_8614(address) = HIGH_LEVEL_CALL, dest:factory_22(FFactory), function:getPair, arguments:['tokenAddress_1', 'assetToken_22']  
factory_23(FFactory) := phi(['factory_6', 'factory_23', 'factory_1', 'factory_3', 'factory_22', 'factory_19', 'factory_13'])
assetToken_23(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_1', 'assetToken_22', 'assetToken_12', 'assetToken_5'])
pair_1(address) := TMP_8614(address)
 assetBalance = IFPair(pair).assetBalance()
TMP_8615 = CONVERT pair_1 to IFPair
TMP_8616(uint256) = HIGH_LEVEL_CALL, dest:TMP_8615(IFPair), function:assetBalance, arguments:[]  
assetBalance_1(uint256) := TMP_8616(uint256)
 FPair(pair).transferAsset(msg.sender,assetBalance)
TMP_8617 = CONVERT pair_1 to FPair
HIGH_LEVEL_CALL, dest:TMP_8617(FPair), function:transferAsset, arguments:['msg.sender', 'assetBalance_1']  
 onlyRole(EXECUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(EXECUTOR_ROLE_8)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### IFPair.assetBalance() [EXTERNAL]
```slithir

```
#### IFPair.balance() [EXTERNAL]
```slithir

```

#### IAgentFactoryV3.initFromBondingCurve(string,string,uint8[],bytes32,address,uint32,uint256,uint256,address) [EXTERNAL]
```slithir

```
#### FRouter.buy(uint256,address,address) [PUBLIC]
```slithir
EXECUTOR_ROLE_6(bytes32) := phi(['EXECUTOR_ROLE_0', 'EXECUTOR_ROLE_2', 'EXECUTOR_ROLE_7', 'EXECUTOR_ROLE_9', 'EXECUTOR_ROLE_11', 'EXECUTOR_ROLE_5'])
factory_14(FFactory) := phi(['factory_6', 'factory_23', 'factory_0', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_13(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_0', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
taxManager_12(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_0', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
 require(bool,string)(tokenAddress != address(0),Zero addresses are not allowed.)
TMP_8584 = CONVERT 0 to address
TMP_8585(bool) = tokenAddress_1 != TMP_8584
TMP_8586(None) = SOLIDITY_CALL require(bool,string)(TMP_8585,Zero addresses are not allowed.)
 require(bool,string)(to != address(0),Zero addresses are not allowed.)
TMP_8587 = CONVERT 0 to address
TMP_8588(bool) = to_1 != TMP_8587
TMP_8589(None) = SOLIDITY_CALL require(bool,string)(TMP_8588,Zero addresses are not allowed.)
 require(bool,string)(amountIn > 0,amountIn must be greater than 0)
TMP_8590(bool) = amountIn_1 > 0
TMP_8591(None) = SOLIDITY_CALL require(bool,string)(TMP_8590,amountIn must be greater than 0)
 pair = factory.getPair(tokenAddress,assetToken)
TMP_8592(address) = HIGH_LEVEL_CALL, dest:factory_16(FFactory), function:getPair, arguments:['tokenAddress_1', 'assetToken_15']  
factory_17(FFactory) := phi(['factory_6', 'factory_23', 'factory_1', 'factory_3', 'factory_16', 'factory_19', 'factory_13'])
assetToken_16(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_1', 'assetToken_15', 'assetToken_12', 'assetToken_5'])
taxManager_15(address) := phi(['taxManager_22', 'taxManager_14', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
pair_1(address) := TMP_8592(address)
 fee = factory.buyTax()
TMP_8593(uint256) = HIGH_LEVEL_CALL, dest:factory_17(FFactory), function:buyTax, arguments:[]  
factory_18(FFactory) := phi(['factory_6', 'factory_17', 'factory_23', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_17(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_16', 'assetToken_23', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
taxManager_16(address) := phi(['taxManager_22', 'taxManager_15', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
fee_1(uint256) := TMP_8593(uint256)
 txFee = (fee * amountIn) / 100
TMP_8594(uint256) = fee_1 (c)* amountIn_1
TMP_8595(uint256) = TMP_8594 (c)/ 100
txFee_1(uint256) := TMP_8595(uint256)
 feeTo = factory.taxVault()
TMP_8596(address) = HIGH_LEVEL_CALL, dest:factory_18(FFactory), function:taxVault, arguments:[]  
factory_19(FFactory) := phi(['factory_6', 'factory_18', 'factory_23', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_18(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_17', 'assetToken_23', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
taxManager_17(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_16', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
feeTo_1(address) := TMP_8596(address)
 amount = amountIn - txFee
TMP_8597(uint256) = amountIn_1 (c)- txFee_1
amount_1(uint256) := TMP_8597(uint256)
 IERC20(assetToken).safeTransferFrom(to,pair,amount)
TMP_8598 = CONVERT assetToken_18 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_8598', 'to_1', 'pair_1', 'amount_1'] 
 IERC20(assetToken).safeTransferFrom(to,feeTo,txFee)
TMP_8600 = CONVERT assetToken_18 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_8600', 'to_1', 'feeTo_1', 'txFee_1'] 
 amountOut = getAmountsOut(tokenAddress,assetToken,amount)
TMP_8602(uint256) = INTERNAL_CALL, FRouter.getAmountsOut(address,address,uint256)(tokenAddress_1,assetToken_18,amount_1)
assetToken_19(address) := phi(['assetToken_5'])
amountOut_1(uint256) := TMP_8602(uint256)
 IFPair(pair).transferTo(to,amountOut)
TMP_8603 = CONVERT pair_1 to IFPair
HIGH_LEVEL_CALL, dest:TMP_8603(IFPair), function:transferTo, arguments:['to_1', 'amountOut_1']  
taxManager_19(address) := phi(['taxManager_22', 'taxManager_18', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
 IFPair(pair).swap(0,amountOut,amount,0)
TMP_8605 = CONVERT pair_1 to IFPair
TMP_8606(bool) = HIGH_LEVEL_CALL, dest:TMP_8605(IFPair), function:swap, arguments:['0', 'amountOut_1', 'amount_1', '0']  
taxManager_20(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21', 'taxManager_19'])
 feeTo == taxManager
TMP_8607(bool) = feeTo_1 == taxManager_20
CONDITION TMP_8607
 IBondingTax(taxManager).swapForAsset()
TMP_8608 = CONVERT taxManager_20 to IBondingTax
TUPLE_93(bool,uint256) = HIGH_LEVEL_CALL, dest:TMP_8608(IBondingTax), function:swapForAsset, arguments:[]  
taxManager_21(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
 (amount,amountOut)
RETURN amount_1,amountOut_1
 onlyRole(EXECUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(EXECUTOR_ROLE_6)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### IFPair.getReserves() [EXTERNAL]
```slithir

```
#### SafeERC20.safeTransferFrom(IERC20,address,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transferFrom,(from,to,value)))
REF_2046(transferFrom) -> token_1.transferFrom
TMP_5415(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2046,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000160>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001150>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001c90>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5415)
```
#### FERC20.balanceOf(address) [PUBLIC]
```slithir
_balances_2(mapping(address => uint256)) := phi(['_balances_7', '_balances_1', '_balances_0', '_balances_5', '_balances_2', '_balances_10'])
 _balances[account]
REF_3324(uint256) -> _balances_2[account_1]
RETURN REF_3324
```
#### FERC20.totalSupply() [PUBLIC]
```slithir
_totalSupply_6(uint256) := phi(['_totalSupply_0', '_totalSupply_5'])
 _totalSupply
RETURN _totalSupply_6
```
#### FERC20.transfer(address,uint256) [PUBLIC]
```slithir
 _transfer(_msgSender(),recipient,amount)
TMP_8219(address) = INTERNAL_CALL, Context._msgSender()()
INTERNAL_CALL, FERC20._transfer(address,address,uint256)(TMP_8219,recipient_1,amount_1)
 true
RETURN True
```
#### FFactory.createPair(address,address) [EXTERNAL]
```slithir
CREATOR_ROLE_1(bytes32) := phi(['CREATOR_ROLE_2', 'CREATOR_ROLE_0'])
 pair = _createPair(tokenA,tokenB)
TMP_8365(address) = INTERNAL_CALL, FFactory._createPair(address,address)(tokenA_1,tokenB_1)
pair_1(address) := TMP_8365(address)
 pair
RETURN pair_1
 onlyRole(CREATOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(CREATOR_ROLE_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### FRouter.addInitialLiquidity(address,uint256,uint256) [PUBLIC]
```slithir
EXECUTOR_ROLE_1(bytes32) := phi(['EXECUTOR_ROLE_0', 'EXECUTOR_ROLE_2', 'EXECUTOR_ROLE_7', 'EXECUTOR_ROLE_9', 'EXECUTOR_ROLE_11', 'EXECUTOR_ROLE_5'])
factory_4(FFactory) := phi(['factory_6', 'factory_23', 'factory_0', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_6(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_0', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
 require(bool,string)(token_ != address(0),Zero addresses are not allowed.)
TMP_8551 = CONVERT 0 to address
TMP_8552(bool) = token__1 != TMP_8551
TMP_8553(None) = SOLIDITY_CALL require(bool,string)(TMP_8552,Zero addresses are not allowed.)
 pairAddress = factory.getPair(token_,assetToken)
TMP_8554(address) = HIGH_LEVEL_CALL, dest:factory_5(FFactory), function:getPair, arguments:['token__1', 'assetToken_7']  
factory_6(FFactory) := phi(['factory_6', 'factory_23', 'factory_1', 'factory_3', 'factory_5', 'factory_19', 'factory_13'])
assetToken_8(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_1', 'assetToken_7', 'assetToken_12', 'assetToken_5'])
pairAddress_1(address) := TMP_8554(address)
 pair = IFPair(pairAddress)
TMP_8555 = CONVERT pairAddress_1 to IFPair
pair_1(IFPair) := TMP_8555(IFPair)
 token = IERC20(token_)
TMP_8556 = CONVERT token__1 to IERC20
token_1(IERC20) := TMP_8556(IERC20)
 token.safeTransferFrom(msg.sender,pairAddress,amountToken_)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['token_1', 'msg.sender', 'pairAddress_1', 'amountToken__1'] 
 pair.mint(amountToken_,amountAsset_)
TMP_8558(bool) = HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:mint, arguments:['amountToken__1', 'amountAsset__1']  
 (amountToken_,amountAsset_)
RETURN amountToken__1,amountAsset__1
 onlyRole(EXECUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(EXECUTOR_ROLE_1)
```
#### FRouter.sell(uint256,address,address) [PUBLIC]
```slithir
EXECUTOR_ROLE_3(bytes32) := phi(['EXECUTOR_ROLE_0', 'EXECUTOR_ROLE_2', 'EXECUTOR_ROLE_7', 'EXECUTOR_ROLE_9', 'EXECUTOR_ROLE_11', 'EXECUTOR_ROLE_5'])
factory_7(FFactory) := phi(['factory_6', 'factory_23', 'factory_0', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_9(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_0', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
taxManager_1(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_0', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
 require(bool,string)(tokenAddress != address(0),Zero addresses are not allowed.)
TMP_8560 = CONVERT 0 to address
TMP_8561(bool) = tokenAddress_1 != TMP_8560
TMP_8562(None) = SOLIDITY_CALL require(bool,string)(TMP_8561,Zero addresses are not allowed.)
 require(bool,string)(to != address(0),Zero addresses are not allowed.)
TMP_8563 = CONVERT 0 to address
TMP_8564(bool) = to_1 != TMP_8563
TMP_8565(None) = SOLIDITY_CALL require(bool,string)(TMP_8564,Zero addresses are not allowed.)
 pairAddress = factory.getPair(tokenAddress,assetToken)
TMP_8566(address) = HIGH_LEVEL_CALL, dest:factory_9(FFactory), function:getPair, arguments:['tokenAddress_1', 'assetToken_11']  
factory_10(FFactory) := phi(['factory_6', 'factory_23', 'factory_9', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_12(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_11', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
taxManager_4(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_3', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
pairAddress_1(address) := TMP_8566(address)
 pair = IFPair(pairAddress)
TMP_8567 = CONVERT pairAddress_1 to IFPair
pair_1(IFPair) := TMP_8567(IFPair)
 token = IERC20(tokenAddress)
TMP_8568 = CONVERT tokenAddress_1 to IERC20
token_1(IERC20) := TMP_8568(IERC20)
 amountOut = getAmountsOut(tokenAddress,address(0),amountIn)
TMP_8569 = CONVERT 0 to address
TMP_8570(uint256) = INTERNAL_CALL, FRouter.getAmountsOut(address,address,uint256)(tokenAddress_1,TMP_8569,amountIn_1)
factory_11(FFactory) := phi(['factory_3'])
amountOut_1(uint256) := TMP_8570(uint256)
 token.safeTransferFrom(to,pairAddress,amountIn)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['token_1', 'to_1', 'pairAddress_1', 'amountIn_1'] 
 fee = factory.sellTax()
TMP_8572(uint256) = HIGH_LEVEL_CALL, dest:factory_11(FFactory), function:sellTax, arguments:[]  
factory_12(FFactory) := phi(['factory_6', 'factory_23', 'factory_1', 'factory_3', 'factory_19', 'factory_13', 'factory_11'])
taxManager_6(address) := phi(['taxManager_22', 'taxManager_5', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
fee_1(uint256) := TMP_8572(uint256)
 txFee = (fee * amountOut) / 100
TMP_8573(uint256) = fee_1 (c)* amountOut_1
TMP_8574(uint256) = TMP_8573 (c)/ 100
txFee_1(uint256) := TMP_8574(uint256)
 amount = amountOut - txFee
TMP_8575(uint256) = amountOut_1 (c)- txFee_1
amount_1(uint256) := TMP_8575(uint256)
 feeTo = factory.taxVault()
TMP_8576(address) = HIGH_LEVEL_CALL, dest:factory_12(FFactory), function:taxVault, arguments:[]  
factory_13(FFactory) := phi(['factory_6', 'factory_23', 'factory_1', 'factory_3', 'factory_19', 'factory_13', 'factory_12'])
taxManager_7(address) := phi(['taxManager_22', 'taxManager_6', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
feeTo_1(address) := TMP_8576(address)
 pair.transferAsset(to,amount)
HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:transferAsset, arguments:['to_1', 'amount_1']  
taxManager_8(address) := phi(['taxManager_22', 'taxManager_7', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
 pair.transferAsset(feeTo,txFee)
HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:transferAsset, arguments:['feeTo_1', 'txFee_1']  
taxManager_9(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_20', 'taxManager_8', 'taxManager_11', 'taxManager_21'])
 pair.swap(amountIn,0,0,amountOut)
TMP_8579(bool) = HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:swap, arguments:['amountIn_1', '0', '0', 'amountOut_1']  
taxManager_10(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21', 'taxManager_9'])
 feeTo == taxManager
TMP_8580(bool) = feeTo_1 == taxManager_10
CONDITION TMP_8580
 IBondingTax(taxManager).swapForAsset()
TMP_8581 = CONVERT taxManager_10 to IBondingTax
TUPLE_92(bool,uint256) = HIGH_LEVEL_CALL, dest:TMP_8581(IBondingTax), function:swapForAsset, arguments:[]  
taxManager_11(address) := phi(['taxManager_22', 'taxManager_10', 'taxManager_20', 'taxManager_11', 'taxManager_21'])
 (amountIn,amountOut)
RETURN amountIn_1,amountOut_1
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
 onlyRole(EXECUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(EXECUTOR_ROLE_4)
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


#### FPair.transferAsset(address,uint256) [PUBLIC]
```slithir
tokenB_2(address) := phi(['tokenB_3', 'tokenB_5', 'tokenB_1', 'tokenB_0'])
 require(bool,string)(recipient != address(0),Zero addresses are not allowed.)
TMP_8432 = CONVERT 0 to address
TMP_8433(bool) = recipient_1 != TMP_8432
TMP_8434(None) = SOLIDITY_CALL require(bool,string)(TMP_8433,Zero addresses are not allowed.)
 IERC20(tokenB).safeTransfer(recipient,amount)
TMP_8435 = CONVERT tokenB_3 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_8435', 'recipient_1', 'amount_1'] 
 onlyRouter()
MODIFIER_CALL, FPair.onlyRouter()()
```
#### FRouter.getAmountsOut(address,address,uint256) [PUBLIC]
```slithir
token_1(address) := phi(['tokenAddress_1', 'tokenAddress_1'])
assetToken__1(address) := phi(['assetToken_18', 'TMP_8569'])
amountIn_1(uint256) := phi(['amountIn_1', 'amount_1'])
factory_2(FFactory) := phi(['factory_6', 'factory_23', 'factory_0', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_2(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_0', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
 require(bool,string)(token != address(0),Zero addresses are not allowed.)
TMP_8538 = CONVERT 0 to address
TMP_8539(bool) = token_1 != TMP_8538
TMP_8540(None) = SOLIDITY_CALL require(bool,string)(TMP_8539,Zero addresses are not allowed.)
 pairAddress = factory.getPair(token,assetToken)
TMP_8541(address) = HIGH_LEVEL_CALL, dest:factory_2(FFactory), function:getPair, arguments:['token_1', 'assetToken_2']  
factory_3(FFactory) := phi(['factory_6', 'factory_23', 'factory_2', 'factory_1', 'factory_3', 'factory_19', 'factory_13'])
assetToken_3(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_2', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
pairAddress_1(address) := TMP_8541(address)
 pair = IFPair(pairAddress)
TMP_8542 = CONVERT pairAddress_1 to IFPair
pair_1(IFPair) := TMP_8542(IFPair)
 (reserveA,reserveB) = pair.getReserves()
TUPLE_91(uint256,uint256) = HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:getReserves, arguments:[]  
assetToken_4(address) := phi(['assetToken_19', 'assetToken_8', 'assetToken_23', 'assetToken_1', 'assetToken_3', 'assetToken_12', 'assetToken_5'])
reserveA_1(uint256)= UNPACK TUPLE_91 index: 0 
reserveB_1(uint256)= UNPACK TUPLE_91 index: 1 
 k = pair.kLast()
TMP_8543(uint256) = HIGH_LEVEL_CALL, dest:pair_1(IFPair), function:kLast, arguments:[]  
assetToken_5(address) := phi(['assetToken_19', 'assetToken_4', 'assetToken_8', 'assetToken_23', 'assetToken_1', 'assetToken_12', 'assetToken_5'])
k_1(uint256) := TMP_8543(uint256)
 assetToken_ == assetToken
TMP_8544(bool) = assetToken__1 == assetToken_5
CONDITION TMP_8544
 newReserveB = reserveB + amountIn
TMP_8545(uint256) = reserveB_1 (c)+ amountIn_1
newReserveB_1(uint256) := TMP_8545(uint256)
 newReserveA = k / newReserveB
TMP_8546(uint256) = k_1 (c)/ newReserveB_1
newReserveA_1(uint256) := TMP_8546(uint256)
 amountOut = reserveA - newReserveA
TMP_8547(uint256) = reserveA_1 (c)- newReserveA_1
amountOut_1(uint256) := TMP_8547(uint256)
 newReserveA_scope_0 = reserveA + amountIn
TMP_8548(uint256) = reserveA_1 (c)+ amountIn_1
newReserveA_scope_0_1(uint256) := TMP_8548(uint256)
 newReserveB_scope_1 = k / newReserveA_scope_0
TMP_8549(uint256) = k_1 (c)/ newReserveA_scope_0_1
newReserveB_scope_1_1(uint256) := TMP_8549(uint256)
 amountOut = reserveB - newReserveB_scope_1
TMP_8550(uint256) = reserveB_1 (c)- newReserveB_scope_1_1
amountOut_2(uint256) := TMP_8550(uint256)
amountOut_3(uint256) := phi(['amountOut_1', 'amountOut_2'])
 amountOut
RETURN amountOut_3
 _amountOut
```
#### IFPair.swap(uint256,uint256,uint256,uint256) [EXTERNAL]
```slithir

```
#### IFPair.transferTo(address,uint256) [EXTERNAL]
```slithir

```

#### FERC20._transfer(address,address,uint256) [PRIVATE]
```slithir
from_1(address) := phi(['TMP_8219', 'sender_1'])
to_1(address) := phi(['recipient_1', 'recipient_1'])
amount_1(uint256) := phi(['amount_1', 'amount_1'])
_maxTxAmount_1(uint256) := phi(['_maxTxAmount_0', '_maxTxAmount_2'])
_balances_3(mapping(address => uint256)) := phi(['_balances_7', '_balances_1', '_balances_0', '_balances_5', '_balances_2', '_balances_10'])
isExcludedFromMaxTx_3(mapping(address => bool)) := phi(['isExcludedFromMaxTx_2', 'isExcludedFromMaxTx_0', 'isExcludedFromMaxTx_3', 'isExcludedFromMaxTx_4'])
 require(bool,string)(from != address(0),ERC20: transfer from the zero address)
TMP_8235 = CONVERT 0 to address
TMP_8236(bool) = from_1 != TMP_8235
TMP_8237(None) = SOLIDITY_CALL require(bool,string)(TMP_8236,ERC20: transfer from the zero address)
 require(bool,string)(to != address(0),ERC20: transfer to the zero address)
TMP_8238 = CONVERT 0 to address
TMP_8239(bool) = to_1 != TMP_8238
TMP_8240(None) = SOLIDITY_CALL require(bool,string)(TMP_8239,ERC20: transfer to the zero address)
 require(bool,string)(amount > 0,Transfer amount must be greater than zero)
TMP_8241(bool) = amount_1 > 0
TMP_8242(None) = SOLIDITY_CALL require(bool,string)(TMP_8241,Transfer amount must be greater than zero)
 ! isExcludedFromMaxTx[from]
REF_3331(bool) -> isExcludedFromMaxTx_3[from_1]
TMP_8243 = UnaryType.BANG REF_3331 
CONDITION TMP_8243
 require(bool,string)(amount <= _maxTxAmount,Exceeds MaxTx)
TMP_8244(bool) = amount_1 <= _maxTxAmount_1
TMP_8245(None) = SOLIDITY_CALL require(bool,string)(TMP_8244,Exceeds MaxTx)
 _balances[from] = _balances[from] - amount
REF_3332(uint256) -> _balances_3[from_1]
REF_3333(uint256) -> _balances_3[from_1]
TMP_8246(uint256) = REF_3333 (c)- amount_1
_balances_4(mapping(address => uint256)) := phi(['_balances_3'])
REF_3332(uint256) (->_balances_4) := TMP_8246(uint256)
 _balances[to] = _balances[to] + amount
REF_3334(uint256) -> _balances_4[to_1]
REF_3335(uint256) -> _balances_4[to_1]
TMP_8247(uint256) = REF_3335 (c)+ amount_1
_balances_5(mapping(address => uint256)) := phi(['_balances_4'])
REF_3334(uint256) (->_balances_5) := TMP_8247(uint256)
 Transfer(from,to,amount)
Emit Transfer(from_1,to_1,amount_1)
```
#### FFactory._createPair(address,address) [INTERNAL]
```slithir
tokenA_1(address) := phi(['tokenA_1'])
tokenB_1(address) := phi(['tokenB_1'])
pairs_1(address[]) := phi(['pairs_3', 'pairs_0', 'pairs_4'])
router_1(address) := phi(['router_2', 'router_0'])
 require(bool,string)(tokenA != address(0),Zero addresses are not allowed.)
TMP_8345 = CONVERT 0 to address
TMP_8346(bool) = tokenA_1 != TMP_8345
TMP_8347(None) = SOLIDITY_CALL require(bool,string)(TMP_8346,Zero addresses are not allowed.)
 require(bool,string)(tokenB != address(0),Zero addresses are not allowed.)
TMP_8348 = CONVERT 0 to address
TMP_8349(bool) = tokenB_1 != TMP_8348
TMP_8350(None) = SOLIDITY_CALL require(bool,string)(TMP_8349,Zero addresses are not allowed.)
 require(bool,string)(router != address(0),No router)
TMP_8351 = CONVERT 0 to address
TMP_8352(bool) = router_1 != TMP_8351
TMP_8353(None) = SOLIDITY_CALL require(bool,string)(TMP_8352,No router)
 pair_ = new FPair(router,tokenA,tokenB)
TMP_8355(FPair) = new FPair(router_1,tokenA_1,tokenB_1) 
pair__1(FPair) := TMP_8355(FPair)
 _pair[tokenA][tokenB] = address(pair_)
REF_3371(mapping(address => address)) -> _pair_0[tokenA_1]
REF_3372(address) -> REF_3371[tokenB_1]
TMP_8356 = CONVERT pair__1 to address
_pair_1(mapping(address => mapping(address => address))) := phi(['_pair_0'])
REF_3372(address) (->_pair_1) := TMP_8356(address)
 _pair[tokenB][tokenA] = address(pair_)
REF_3373(mapping(address => address)) -> _pair_1[tokenB_1]
REF_3374(address) -> REF_3373[tokenA_1]
TMP_8357 = CONVERT pair__1 to address
_pair_2(mapping(address => mapping(address => address))) := phi(['_pair_1'])
REF_3374(address) (->_pair_2) := TMP_8357(address)
 pairs.push(address(pair_))
TMP_8358 = CONVERT pair__1 to address
REF_3376 -> LENGTH pairs_1
TMP_8360(uint256) := REF_3376(uint256)
TMP_8361(uint256) = TMP_8360 (c)+ 1
pairs_2(address[]) := phi(['pairs_1'])
REF_3376(uint256) (->pairs_2) := TMP_8361(uint256)
REF_3377(address) -> pairs_2[TMP_8360]
pairs_3(address[]) := phi(['pairs_2'])
REF_3377(address) (->pairs_3) := TMP_8358(address)
 n = pairs.length
REF_3378 -> LENGTH pairs_3
n_1(uint256) := REF_3378(uint256)
 PairCreated(tokenA,tokenB,address(pair_),n)
TMP_8362 = CONVERT pair__1 to address
Emit PairCreated(tokenA_1,tokenB_1,TMP_8362,n_1)
 address(pair_)
TMP_8364 = CONVERT pair__1 to address
RETURN TMP_8364
```
#### IFPair.mint(uint256,uint256) [EXTERNAL]
```slithir

```
#### IFPair.transferAsset(address,uint256) [EXTERNAL]
```slithir

```
#### Address.functionCall(address,bytes) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0)
TMP_5457(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256)(target_1,data_1,0)
RETURN TMP_5457
```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_2044(transfer) -> token_1.transfer
TMP_5413(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2044,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000550>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001210>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5413)
```
#### IFPair.kLast() [EXTERNAL]
```slithir

```
