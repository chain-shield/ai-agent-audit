

### Storage layout (Distributor) 

```text
initialized bool
launchpad address
totalPendingRewards mapping(address => uint256)

```


#### Distributor._decreaseTotalPending(address,uint256) [INTERNAL]
```slithir
asset_1(address) := phi(['base_1', 'quote_1'])
amount_1(uint256) := phi(['quoteAmount_1', 'baseAmount_1'])
totalPendingRewards_5(mapping(address => uint256)) := phi(['totalPendingRewards_2', 'totalPendingRewards_4', 'totalPendingRewards_6', 'totalPendingRewards_0'])
 currTotal = totalPendingRewards[asset]
REF_1071(uint256) -> totalPendingRewards_5[asset_1]
currTotal_1(uint256) := REF_1071(uint256)
 currTotal < amount
TMP_2386(bool) = currTotal_1 < amount_1
CONDITION TMP_2386
 revert ClaimAmountExceedsTotalPendingRewards()()
TMP_2387(None) = SOLIDITY_CALL revert ClaimAmountExceedsTotalPendingRewards()()
 totalPendingRewards[asset] -= amount
REF_1072(uint256) -> totalPendingRewards_5[asset_1]
totalPendingRewards_6(mapping(address => uint256)) := phi(['totalPendingRewards_5'])
REF_1072(-> totalPendingRewards_6) = REF_1072 - amount_1
 TotalPendingRewardsDecreased(asset,amount)
Emit TotalPendingRewardsDecreased(asset_1,amount_1)
```
#### Distributor._distributeAssets(address,uint256,address,uint256) [INTERNAL]
```slithir
base_1(address) := phi(['launchAsset_1', 'launchAsset_1', 'launchAsset_1'])
baseAmount_1(uint256) := phi(['baseAmount_1', 'baseAmount_1', 'baseAmount_1'])
quote_1(address) := phi(['REF_1061', 'REF_1067', 'REF_1064'])
quoteAmount_1(uint256) := phi(['quoteAmount_1', 'quoteAmount_1', 'quoteAmount_1'])
 baseAmount > 0
TMP_2379(bool) = baseAmount_1 > 0
CONDITION TMP_2379
 _decreaseTotalPending(base,baseAmount)
INTERNAL_CALL, Distributor._decreaseTotalPending(address,uint256)(base_1,baseAmount_1)
 base.safeTransfer(msg.sender,baseAmount)
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransfer(address,address,uint256), arguments:['base_1', 'msg.sender', 'baseAmount_1'] 
 quoteAmount > 0
TMP_2382(bool) = quoteAmount_1 > 0
CONDITION TMP_2382
 _decreaseTotalPending(quote,quoteAmount)
INTERNAL_CALL, Distributor._decreaseTotalPending(address,uint256)(quote_1,quoteAmount_1)
 quote.safeTransfer(msg.sender,quoteAmount)
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransfer(address,address,uint256), arguments:['quote_1', 'msg.sender', 'quoteAmount_1']
```
#### Distributor._increaseTotalPending(address,uint256) [INTERNAL]
```slithir
asset_1(address) := phi(['quoteAsset_3', 'launchAsset_3'])
amount_1(uint256) := phi(['launchAssetAmount_3', 'quoteAssetAmount_3'])
totalPendingRewards_3(mapping(address => uint256)) := phi(['totalPendingRewards_2', 'totalPendingRewards_4', 'totalPendingRewards_6', 'totalPendingRewards_0'])
 totalPendingRewards[asset] += amount
REF_1070(uint256) -> totalPendingRewards_3[asset_1]
totalPendingRewards_4(mapping(address => uint256)) := phi(['totalPendingRewards_3'])
REF_1070(-> totalPendingRewards_4) = REF_1070 + amount_1
 TotalPendingRewardsIncreased(asset,amount)
Emit TotalPendingRewardsIncreased(asset_1,amount_1)
```
#### Distributor.addRewards(address,address,uint128,uint128) [EXTERNAL]
```slithir
 launchAsset = token0
launchAsset_1(address) := token0_1(address)
 quoteAsset = token1
quoteAsset_1(address) := token1_1(address)
 launchAssetAmount = amount0
launchAssetAmount_1(uint128) := amount0_1(uint128)
 quoteAssetAmount = amount1
quoteAssetAmount_1(uint128) := amount1_1(uint128)
 rs = RewardsTrackerStorage.getRewardPool(token0)
TMP_2348(RewardPoolData) = LIBRARY_CALL, dest:RewardsTrackerStorage, function:RewardsTrackerStorage.getRewardPool(address), arguments:['token0_1'] 
rs_1 (-> ['TMP_2348'])(RewardPoolData) := TMP_2348(RewardPoolData)
 rs.quoteAsset == address(0)
REF_1051(address) -> rs_1 (-> ['TMP_2348']).quoteAsset
TMP_2349 = CONVERT 0 to address
TMP_2350(bool) = REF_1051 == TMP_2349
CONDITION TMP_2350
 rs = RewardsTrackerStorage.getRewardPool(token1)
TMP_2351(RewardPoolData) = LIBRARY_CALL, dest:RewardsTrackerStorage, function:RewardsTrackerStorage.getRewardPool(address), arguments:['token1_1'] 
rs_2 (-> ['TMP_2351'])(RewardPoolData) := TMP_2351(RewardPoolData)
 rs.quoteAsset == address(0)
REF_1053(address) -> rs_2 (-> ['TMP_2351']).quoteAsset
TMP_2352 = CONVERT 0 to address
TMP_2353(bool) = REF_1053 == TMP_2352
CONDITION TMP_2353
 revert RewardsDoNotExist()()
TMP_2354(None) = SOLIDITY_CALL revert RewardsDoNotExist()()
 (launchAsset,quoteAsset,launchAssetAmount,quoteAssetAmount) = (token1,token0,amount1,amount0)
launchAsset_2(address) := token1_1(address)
quoteAsset_2(address) := token0_1(address)
launchAssetAmount_2(uint128) := amount1_1(uint128)
quoteAssetAmount_2(uint128) := amount0_1(uint128)
launchAsset_3(address) := phi(['launchAsset_1', 'launchAsset_2'])
quoteAsset_3(address) := phi(['quoteAsset_1', 'quoteAsset_2'])
launchAssetAmount_3(uint128) := phi(['launchAssetAmount_2', 'launchAssetAmount_1'])
quoteAssetAmount_3(uint128) := phi(['quoteAssetAmount_1', 'quoteAssetAmount_2'])
rs_3 (-> ['TMP_2348', 'TMP_2351'])(RewardPoolData) := phi(["rs_1 (-> ['TMP_2348'])", "rs_2 (-> ['TMP_2351'])"])
 rs.totalShares == 0
REF_1054(uint96) -> rs_3 (-> ['TMP_2348', 'TMP_2351']).totalShares
TMP_2355(bool) = REF_1054 == 0
CONDITION TMP_2355
 revert NoSharesToIncentivize()()
TMP_2356(None) = SOLIDITY_CALL revert NoSharesToIncentivize()()
 launchAssetAmount > 0
TMP_2357(bool) = launchAssetAmount_3 > 0
CONDITION TMP_2357
 rs.addBaseRewards(launchAsset,launchAssetAmount)
LIBRARY_CALL, dest:RewardsTrackerLib, function:RewardsTrackerLib.addBaseRewards(RewardPoolData,address,uint128), arguments:["rs_3 (-> ['TMP_2348', 'TMP_2351'])", 'launchAsset_3', 'launchAssetAmount_3'] 
 _increaseTotalPending(launchAsset,launchAssetAmount)
INTERNAL_CALL, Distributor._increaseTotalPending(address,uint256)(launchAsset_3,launchAssetAmount_3)
 launchAsset.safeTransferFrom(msg.sender,address(this),uint256(launchAssetAmount))
TMP_2360 = CONVERT this to address
TMP_2361 = CONVERT launchAssetAmount_3 to uint256
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransferFrom(address,address,address,uint256), arguments:['launchAsset_3', 'msg.sender', 'TMP_2360', 'TMP_2361'] 
 quoteAssetAmount > 0
TMP_2363(bool) = quoteAssetAmount_3 > 0
CONDITION TMP_2363
 rs.addQuoteRewards(launchAsset,quoteAsset,quoteAssetAmount)
LIBRARY_CALL, dest:RewardsTrackerLib, function:RewardsTrackerLib.addQuoteRewards(RewardPoolData,address,address,uint128), arguments:["rs_3 (-> ['TMP_2348', 'TMP_2351'])", 'launchAsset_3', 'quoteAsset_3', 'quoteAssetAmount_3'] 
 _increaseTotalPending(quoteAsset,quoteAssetAmount)
INTERNAL_CALL, Distributor._increaseTotalPending(address,uint256)(quoteAsset_3,quoteAssetAmount_3)
 quoteAsset.safeTransferFrom(msg.sender,address(this),uint256(quoteAssetAmount))
TMP_2366 = CONVERT this to address
TMP_2367 = CONVERT quoteAssetAmount_3 to uint256
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransferFrom(address,address,address,uint256), arguments:['quoteAsset_3', 'msg.sender', 'TMP_2366', 'TMP_2367']
```
#### Distributor.claimRewards(address) [EXTERNAL]
```slithir
 rs = RewardsTrackerStorage.getRewardPool(launchAsset)
TMP_2377(RewardPoolData) = LIBRARY_CALL, dest:RewardsTrackerStorage, function:RewardsTrackerStorage.getRewardPool(address), arguments:['launchAsset_1'] 
rs_1 (-> ['TMP_2377'])(RewardPoolData) := TMP_2377(RewardPoolData)
 (baseAmount,quoteAmount) = rs.claim(msg.sender)
TUPLE_19(uint256,uint256) = LIBRARY_CALL, dest:RewardsTrackerLib, function:RewardsTrackerLib.claim(RewardPoolData,address), arguments:["rs_1 (-> ['TMP_2377'])", 'msg.sender'] 
baseAmount_1(uint256)= UNPACK TUPLE_19 index: 0 
quoteAmount_1(uint256)= UNPACK TUPLE_19 index: 1 
 _distributeAssets(launchAsset,baseAmount,rs.quoteAsset,quoteAmount)
REF_1067(address) -> rs_1 (-> ['TMP_2377']).quoteAsset
INTERNAL_CALL, Distributor._distributeAssets(address,uint256,address,uint256)(launchAsset_1,baseAmount_1,REF_1067,quoteAmount_1)
 (baseAmount,quoteAmount)
RETURN baseAmount_1,quoteAmount_1
```
#### Distributor.constructor() [PUBLIC]
```slithir
 _initializeOwner(msg.sender)
INTERNAL_CALL, Ownable._initializeOwner(address)(msg.sender)
```
#### Distributor.createRewardsPair(address,address) [EXTERNAL]
```slithir
 rs = RewardsTrackerStorage.getRewardPool(launchAsset)
TMP_2338(RewardPoolData) = LIBRARY_CALL, dest:RewardsTrackerStorage, function:RewardsTrackerStorage.getRewardPool(address), arguments:['launchAsset_1'] 
rs_1 (-> ['TMP_2338'])(RewardPoolData) := TMP_2338(RewardPoolData)
 rsq = RewardsTrackerStorage.getRewardPool(quoteAsset)
TMP_2339(RewardPoolData) = LIBRARY_CALL, dest:RewardsTrackerStorage, function:RewardsTrackerStorage.getRewardPool(address), arguments:['quoteAsset_1'] 
rsq_1 (-> ['TMP_2339'])(RewardPoolData) := TMP_2339(RewardPoolData)
 rs.quoteAsset != address(0) || rsq.quoteAsset != address(0)
REF_1047(address) -> rs_1 (-> ['TMP_2338']).quoteAsset
TMP_2340 = CONVERT 0 to address
TMP_2341(bool) = REF_1047 != TMP_2340
REF_1048(address) -> rsq_1 (-> ['TMP_2339']).quoteAsset
TMP_2342 = CONVERT 0 to address
TMP_2343(bool) = REF_1048 != TMP_2342
TMP_2344(bool) = TMP_2341 || TMP_2343
CONDITION TMP_2344
 revert RewardsExist()()
TMP_2345(None) = SOLIDITY_CALL revert RewardsExist()()
 rs.initializePair(launchAsset,quoteAsset)
LIBRARY_CALL, dest:RewardsTrackerLib, function:RewardsTrackerLib.initializePair(RewardPoolData,address,address), arguments:["rs_1 (-> ['TMP_2338'])", 'launchAsset_1', 'quoteAsset_1'] 
 onlyLaunchpad()
MODIFIER_CALL, Distributor.onlyLaunchpad()()
```
#### Distributor.decreaseStake(address,address,uint96) [EXTERNAL]
```slithir
 rs = RewardsTrackerStorage.getRewardPool(launchAsset)
TMP_2373(RewardPoolData) = LIBRARY_CALL, dest:RewardsTrackerStorage, function:RewardsTrackerStorage.getRewardPool(address), arguments:['launchAsset_1'] 
rs_1 (-> ['TMP_2373'])(RewardPoolData) := TMP_2373(RewardPoolData)
 (baseAmount,quoteAmount) = rs.unstake(account,uint96(shares))
TMP_2374 = CONVERT shares_1 to uint96
TUPLE_18(uint256,uint256) = LIBRARY_CALL, dest:RewardsTrackerLib, function:RewardsTrackerLib.unstake(RewardPoolData,address,uint96), arguments:["rs_1 (-> ['TMP_2373'])", 'account_1', 'TMP_2374'] 
baseAmount_1(uint256)= UNPACK TUPLE_18 index: 0 
quoteAmount_1(uint256)= UNPACK TUPLE_18 index: 1 
 _distributeAssets(launchAsset,baseAmount,rs.quoteAsset,quoteAmount)
REF_1064(address) -> rs_1 (-> ['TMP_2373']).quoteAsset
INTERNAL_CALL, Distributor._distributeAssets(address,uint256,address,uint256)(launchAsset_1,baseAmount_1,REF_1064,quoteAmount_1)
 onlyLaunchpad()
MODIFIER_CALL, Distributor.onlyLaunchpad()()
 (baseAmount,quoteAmount)
RETURN baseAmount_1,quoteAmount_1
```
#### Distributor.endRewards(IGTELaunchpadV2Pair) [EXTERNAL]
```slithir
 pair.endRewardsAccrual()
HIGH_LEVEL_CALL, dest:pair_1(IGTELaunchpadV2Pair), function:endRewardsAccrual, arguments:[]  
 onlyLaunchpad()
MODIFIER_CALL, Distributor.onlyLaunchpad()()
```
#### Distributor.getPendingRewards(address,address) [EXTERNAL]
```slithir
 rs = RewardsTrackerStorage.getRewardPool(launchAsset)
TMP_2335(RewardPoolData) = LIBRARY_CALL, dest:RewardsTrackerStorage, function:RewardsTrackerStorage.getRewardPool(address), arguments:['launchAsset_1'] 
rs_1 (-> ['TMP_2335'])(RewardPoolData) := TMP_2335(RewardPoolData)
 rs.getPendingRewards(account)
TUPLE_16(uint256,uint256) = LIBRARY_CALL, dest:RewardsTrackerLib, function:RewardsTrackerLib.getPendingRewards(RewardPoolData,address), arguments:["rs_1 (-> ['TMP_2335'])", 'account_1'] 
RETURN TUPLE_16
 (pendingBase,pendingQuote)
```
#### Distributor.getRewardsPoolData(address) [EXTERNAL]
```slithir
 RewardsTrackerStorage.getRewardPool(launchAsset).getRewardsPoolData()
TMP_2325(RewardPoolData) = LIBRARY_CALL, dest:RewardsTrackerStorage, function:RewardsTrackerStorage.getRewardPool(address), arguments:['launchAsset_1'] 
TMP_2326(RewardPoolDataMemory) = LIBRARY_CALL, dest:RewardsTrackerLib, function:RewardsTrackerLib.getRewardsPoolData(RewardPoolData), arguments:['TMP_2325'] 
RETURN TMP_2326
```
#### Distributor.getUserData(address,address) [EXTERNAL]
```slithir
 RewardsTrackerStorage.getRewardPool(launchAsset).getUserData(account)
TMP_2327(RewardPoolData) = LIBRARY_CALL, dest:RewardsTrackerStorage, function:RewardsTrackerStorage.getRewardPool(address), arguments:['launchAsset_1'] 
TMP_2328(UserRewardData) = LIBRARY_CALL, dest:RewardsTrackerLib, function:RewardsTrackerLib.getUserData(RewardPoolData,address), arguments:['TMP_2327', 'account_1'] 
RETURN TMP_2328
```
#### Distributor.getUserDataForTokens(address[],address) [EXTERNAL]
```slithir
 data = new UserRewardData[](launchAssets.length)
REF_1036 -> LENGTH launchAssets_1
TMP_2330(UserRewardData[])  = new UserRewardData[](REF_1036)
data_1(UserRewardData[]) = ['TMP_2330(UserRewardData[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < launchAssets.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_1037 -> LENGTH launchAssets_1
TMP_2331(bool) = i_2 < REF_1037
CONDITION TMP_2331
 data[i] = RewardsTrackerStorage.getRewardPool(launchAssets[i]).getUserData(account)
REF_1038(UserRewardData) -> data_1[i_2]
REF_1040(address) -> launchAssets_1[i_2]
TMP_2332(RewardPoolData) = LIBRARY_CALL, dest:RewardsTrackerStorage, function:RewardsTrackerStorage.getRewardPool(address), arguments:['REF_1040'] 
TMP_2333(UserRewardData) = LIBRARY_CALL, dest:RewardsTrackerLib, function:RewardsTrackerLib.getUserData(RewardPoolData,address), arguments:['TMP_2332', 'account_1'] 
data_2(UserRewardData[]) := phi(['data_1'])
REF_1038(UserRewardData) (->data_2) := TMP_2333(UserRewardData)
 i ++
TMP_2334(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 data
RETURN data_1
```
#### Distributor.increaseStake(address,address,uint96) [EXTERNAL]
```slithir
 rs = RewardsTrackerStorage.getRewardPool(launchAsset)
TMP_2369(RewardPoolData) = LIBRARY_CALL, dest:RewardsTrackerStorage, function:RewardsTrackerStorage.getRewardPool(address), arguments:['launchAsset_1'] 
rs_1 (-> ['TMP_2369'])(RewardPoolData) := TMP_2369(RewardPoolData)
 (baseAmount,quoteAmount) = rs.stake(account,uint96(shares))
TMP_2370 = CONVERT shares_1 to uint96
TUPLE_17(uint256,uint256) = LIBRARY_CALL, dest:RewardsTrackerLib, function:RewardsTrackerLib.stake(RewardPoolData,address,uint96), arguments:["rs_1 (-> ['TMP_2369'])", 'account_1', 'TMP_2370'] 
baseAmount_1(uint256)= UNPACK TUPLE_17 index: 0 
quoteAmount_1(uint256)= UNPACK TUPLE_17 index: 1 
 _distributeAssets(launchAsset,baseAmount,rs.quoteAsset,quoteAmount)
REF_1061(address) -> rs_1 (-> ['TMP_2369']).quoteAsset
INTERNAL_CALL, Distributor._distributeAssets(address,uint256,address,uint256)(launchAsset_1,baseAmount_1,REF_1061,quoteAmount_1)
 onlyLaunchpad()
MODIFIER_CALL, Distributor.onlyLaunchpad()()
 (baseAmount,quoteAmount)
RETURN baseAmount_1,quoteAmount_1
```
#### Distributor.initialize(address) [PUBLIC][OWNER]
```slithir
initialized_1(bool) := phi(['initialized_3', 'initialized_0'])
 initialized
CONDITION initialized_2
 revert Initialized()()
TMP_2316(None) = SOLIDITY_CALL revert Initialized()()
 launchpad = _launchpad
launchpad_1(address) := _launchpad_1(address)
 initialized = true
initialized_3(bool) := True(bool)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### Distributor.skimExcessRewards(address,uint256) [EXTERNAL]
```slithir
ADMIN_ROLE_1(uint256) := phi(['ADMIN_ROLE_2', 'ADMIN_ROLE_0'])
totalPendingRewards_1(mapping(address => uint256)) := phi(['totalPendingRewards_2', 'totalPendingRewards_4', 'totalPendingRewards_6', 'totalPendingRewards_0'])
 amount > asset.balanceOf(address(this)) - totalPendingRewards[asset]
TMP_2318 = CONVERT this to address
TMP_2319(uint256) = LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.balanceOf(address,address), arguments:['asset_1', 'TMP_2318'] 
REF_1030(uint256) -> totalPendingRewards_2[asset_1]
TMP_2320(uint256) = TMP_2319 (c)- REF_1030
TMP_2321(bool) = amount_1 > TMP_2320
CONDITION TMP_2321
 revert SkimOverflow()()
TMP_2322(None) = SOLIDITY_CALL revert SkimOverflow()()
 asset.safeTransfer(msg.sender,amount)
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransfer(address,address,uint256), arguments:['asset_1', 'msg.sender', 'amount_1'] 
 onlyOwnerOrRoles(ADMIN_ROLE)
MODIFIER_CALL, OwnableRoles.onlyOwnerOrRoles(uint256)(ADMIN_ROLE_1)
```

#### SafeTransferLib.safeTransfer(address,address,uint256) [INTERNAL]
```slithir
 mstore(uint256,uint256)(0x14,to)
TMP_15324(None) = SOLIDITY_CALL mstore(uint256,uint256)(20,to_1)
 mstore(uint256,uint256)(0x34,amount)
TMP_15325(None) = SOLIDITY_CALL mstore(uint256,uint256)(52,amount_1)
 mstore(uint256,uint256)(0x00,0xa9059cbb000000000000000000000000)
TMP_15326(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,224668671643508016486903311432943665152)
 success_safeTransfer_asm_0 = call(uint256,uint256,uint256,uint256,uint256,uint256,uint256)(gas()(),token,0,0x10,0x44,0x00,0x20)
TMP_15327(uint256) = SOLIDITY_CALL gas()()
TMP_15328(uint256) = SOLIDITY_CALL call(uint256,uint256,uint256,uint256,uint256,uint256,uint256)(TMP_15327,token_1,0,16,68,0,32)
success_safeTransfer_asm_0_1(uint256) := TMP_15328(uint256)
 ! mload(uint256)(0x00) == 1 & success_safeTransfer_asm_0
TMP_15329(uint256) = SOLIDITY_CALL mload(uint256)(0)
TMP_15330(bool) = TMP_15329 == 1
TMP_15331(bool) = TMP_15330 & success_safeTransfer_asm_0_1
TMP_15332 = UnaryType.BANG TMP_15331 
CONDITION TMP_15332
 ! ! extcodesize(uint256)(token) | returndatasize()() < success_safeTransfer_asm_0
REF_5786 -> CODESIZE token_1
TMP_15333 = UnaryType.BANG REF_5786 
TMP_15334(uint256) = SOLIDITY_CALL returndatasize()(token_1)
TMP_15335(uint256) = TMP_15333 | TMP_15334
TMP_15336(bool) = TMP_15335 < success_safeTransfer_asm_0_1
TMP_15337 = UnaryType.BANG TMP_15336 
CONDITION TMP_15337
 mstore(uint256,uint256)(0x00,0x90b8ec18)
TMP_15338(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,2428038168)
 revert(uint256,uint256)(0x1c,0x04)
TMP_15339(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 mstore(uint256,uint256)(0x34,0)
TMP_15340(None) = SOLIDITY_CALL mstore(uint256,uint256)(52,0)
```
#### RewardsTrackerLib.addBaseRewards(RewardPoolData,address,uint128) [INTERNAL]
```slithir
 self.pendingBaseRewards += amount
REF_1251(uint128) -> self_1 (-> []).pendingBaseRewards
self_2 (-> [])(RewardPoolData) := phi(['self_1 (-> [])'])
REF_1251(-> self_2 (-> [])) = REF_1251 (c)+ amount_1
 BaseRewardsAdded(baseAsset,amount)
Emit BaseRewardsAdded(baseAsset_1,amount_1)
```
#### RewardsTrackerLib.addQuoteRewards(RewardPoolData,address,address,uint128) [INTERNAL]
```slithir
 self.pendingQuoteRewards += amount
REF_1252(uint128) -> self_1 (-> []).pendingQuoteRewards
self_2 (-> [])(RewardPoolData) := phi(['self_1 (-> [])'])
REF_1252(-> self_2 (-> [])) = REF_1252 (c)+ amount_1
 QuoteRewardsAdded(baseAsset,quoteAsset,amount)
Emit QuoteRewardsAdded(baseAsset_1,quoteAsset_1,amount_1)
```
#### RewardsTrackerStorage.getRewardPool(address) [INTERNAL]
```slithir
 slot = rewardPoolSlot(baseAsset)
TMP_3550(bytes32) = INTERNAL_CALL, RewardsTrackerStorage.rewardPoolSlot(address)(baseAsset_1)
slot_1(bytes32) := TMP_3550(bytes32)
 p = slot
p_1 (-> ['slot'])(RewardPoolData) := slot_1(bytes32)
 p
RETURN p_1 (-> ['slot'])
```
#### SafeTransferLib.safeTransferFrom(address,address,address,uint256) [INTERNAL]
```slithir
 m_safeTransferFrom_asm_0 = mload(uint256)(0x40)
TMP_15255(uint256) = SOLIDITY_CALL mload(uint256)(64)
m_safeTransferFrom_asm_0_1(uint256) := TMP_15255(uint256)
 mstore(uint256,uint256)(0x60,amount)
TMP_15256(None) = SOLIDITY_CALL mstore(uint256,uint256)(96,amount_1)
 mstore(uint256,uint256)(0x40,to)
TMP_15257(None) = SOLIDITY_CALL mstore(uint256,uint256)(64,to_1)
 mstore(uint256,uint256)(0x2c,from << 96)
TMP_15258(address) = from_1 << 96
TMP_15259(None) = SOLIDITY_CALL mstore(uint256,uint256)(44,TMP_15258)
 mstore(uint256,uint256)(0x0c,0x23b872dd000000000000000000000000)
TMP_15260(None) = SOLIDITY_CALL mstore(uint256,uint256)(12,47480692178561195778129796594248187904)
 success_safeTransferFrom_asm_0 = call(uint256,uint256,uint256,uint256,uint256,uint256,uint256)(gas()(),token,0,0x1c,0x64,0x00,0x20)
TMP_15261(uint256) = SOLIDITY_CALL gas()()
TMP_15262(uint256) = SOLIDITY_CALL call(uint256,uint256,uint256,uint256,uint256,uint256,uint256)(TMP_15261,token_1,0,28,100,0,32)
success_safeTransferFrom_asm_0_1(uint256) := TMP_15262(uint256)
 ! mload(uint256)(0x00) == 1 & success_safeTransferFrom_asm_0
TMP_15263(uint256) = SOLIDITY_CALL mload(uint256)(0)
TMP_15264(bool) = TMP_15263 == 1
TMP_15265(bool) = TMP_15264 & success_safeTransferFrom_asm_0_1
TMP_15266 = UnaryType.BANG TMP_15265 
CONDITION TMP_15266
 ! ! extcodesize(uint256)(token) | returndatasize()() < success_safeTransferFrom_asm_0
REF_5783 -> CODESIZE token_1
TMP_15267 = UnaryType.BANG REF_5783 
TMP_15268(uint256) = SOLIDITY_CALL returndatasize()(token_1)
TMP_15269(uint256) = TMP_15267 | TMP_15268
TMP_15270(bool) = TMP_15269 < success_safeTransferFrom_asm_0_1
TMP_15271 = UnaryType.BANG TMP_15270 
CONDITION TMP_15271
 mstore(uint256,uint256)(0x00,0x7939f424)
TMP_15272(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,2033841188)
 revert(uint256,uint256)(0x1c,0x04)
TMP_15273(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 mstore(uint256,uint256)(0x60,0)
TMP_15274(None) = SOLIDITY_CALL mstore(uint256,uint256)(96,0)
 mstore(uint256,uint256)(0x40,m_safeTransferFrom_asm_0)
TMP_15275(None) = SOLIDITY_CALL mstore(uint256,uint256)(64,m_safeTransferFrom_asm_0_1)
```
#### RewardsTrackerLib.claim(RewardPoolData,address) [INTERNAL]
```slithir
 userData = self.userRewards[user]
REF_1273(mapping(address => UserRewardData)) -> self_1 (-> []).userRewards
REF_1274(UserRewardData) -> REF_1273[user_1]
userData_1 (-> ['self'])(UserRewardData) := REF_1274(UserRewardData)
 shares = uint256(userData.shares)
REF_1275(uint96) -> userData_1 (-> ['self']).shares
TMP_3516 = CONVERT REF_1275 to uint256
shares_1(uint256) := TMP_3516(uint256)
 shares == 0
TMP_3517(bool) = shares_1 == 0
CONDITION TMP_3517
 revert ZeroShareClaim()()
TMP_3518(None) = SOLIDITY_CALL revert ZeroShareClaim()()
 (accBaseRewardsPerShare,accQuoteRewardsPerShare) = self.update()
TUPLE_30(uint256,uint256) = LIBRARY_CALL, dest:RewardsTrackerLib, function:RewardsTrackerLib.update(RewardPoolData), arguments:['self_1 (-> [])'] 
accBaseRewardsPerShare_1(uint256)= UNPACK TUPLE_30 index: 0 
accQuoteRewardsPerShare_1(uint256)= UNPACK TUPLE_30 index: 1 
 totalAccBaseRewards = totalAccRewards(shares,accBaseRewardsPerShare)
TMP_3519(uint256) = INTERNAL_CALL, RewardsTrackerLib.totalAccRewards(uint256,uint256)(shares_1,accBaseRewardsPerShare_1)
totalAccBaseRewards_1(uint256) := TMP_3519(uint256)
 totalAccQuoteRewards = totalAccRewards(shares,accQuoteRewardsPerShare)
TMP_3520(uint256) = INTERNAL_CALL, RewardsTrackerLib.totalAccRewards(uint256,uint256)(shares_1,accQuoteRewardsPerShare_1)
totalAccQuoteRewards_1(uint256) := TMP_3520(uint256)
 baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt)
REF_1277(uint96) -> userData_1 (-> ['self']).baseRewardDebt
TMP_3521 = CONVERT REF_1277 to uint128
TMP_3522(uint256) = totalAccBaseRewards_1 (c)- TMP_3521
baseAmount_1(uint256) := TMP_3522(uint256)
 quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt)
REF_1278(uint96) -> userData_1 (-> ['self']).quoteRewardDebt
TMP_3523 = CONVERT REF_1278 to uint128
TMP_3524(uint256) = totalAccQuoteRewards_1 (c)- TMP_3523
quoteAmount_1(uint256) := TMP_3524(uint256)
 userData.baseRewardDebt = uint96(totalAccBaseRewards)
REF_1279(uint96) -> userData_1 (-> ['self']).baseRewardDebt
TMP_3525 = CONVERT totalAccBaseRewards_1 to uint96
userData_2 (-> ['self'])(UserRewardData) := phi(["userData_1 (-> ['self'])"])
REF_1279(uint96) (->userData_2 (-> ['self'])) := TMP_3525(uint96)
self_2 (-> ['self'])(RewardPoolData) := phi(["userData_2 (-> ['self'])"])
 userData.quoteRewardDebt = uint96(totalAccQuoteRewards)
REF_1280(uint96) -> userData_2 (-> ['self']).quoteRewardDebt
TMP_3526 = CONVERT totalAccQuoteRewards_1 to uint96
userData_3 (-> ['self'])(UserRewardData) := phi(["userData_2 (-> ['self'])"])
REF_1280(uint96) (->userData_3 (-> ['self'])) := TMP_3526(uint96)
self_3 (-> ['self'])(RewardPoolData) := phi(["userData_3 (-> ['self'])"])
 (baseAmount,quoteAmount)
RETURN baseAmount_1,quoteAmount_1
```
#### RewardsTrackerLib.initializePair(RewardPoolData,address,address) [INTERNAL]
```slithir
 self.quoteAsset = quoteAsset
REF_1250(address) -> self_1 (-> []).quoteAsset
self_2 (-> [])(RewardPoolData) := phi(['self_1 (-> [])'])
REF_1250(address) (->self_2 (-> [])) := quoteAsset_1(address)
 PairRewardsInitialized(baseAsset,quoteAsset)
Emit PairRewardsInitialized(baseAsset_1,quoteAsset_1)
```
#### RewardsTrackerLib.unstake(RewardPoolData,address,uint96) [INTERNAL]
```slithir
 (accBaseRewardsPerShare,accQuoteRewardsPerShare) = self.update()
TUPLE_29(uint256,uint256) = LIBRARY_CALL, dest:RewardsTrackerLib, function:RewardsTrackerLib.update(RewardPoolData), arguments:['self_1 (-> [])'] 
accBaseRewardsPerShare_1(uint256)= UNPACK TUPLE_29 index: 0 
accQuoteRewardsPerShare_1(uint256)= UNPACK TUPLE_29 index: 1 
 userData = self.userRewards[user]
REF_1264(mapping(address => UserRewardData)) -> self_1 (-> []).userRewards
REF_1265(UserRewardData) -> REF_1264[user_1]
userData_1 (-> ['self'])(UserRewardData) := REF_1265(UserRewardData)
 removeShares == 0
TMP_3501(bool) = removeShares_1 == 0
CONDITION TMP_3501
 revert ZeroShareStake()()
TMP_3502(None) = SOLIDITY_CALL revert ZeroShareStake()()
 existingShares = uint256(userData.shares)
REF_1266(uint96) -> userData_1 (-> ['self']).shares
TMP_3503 = CONVERT REF_1266 to uint256
existingShares_1(uint256) := TMP_3503(uint256)
 existingShares < removeShares
TMP_3504(bool) = existingShares_1 < removeShares_1
CONDITION TMP_3504
 revert InsufficientShares()()
TMP_3505(None) = SOLIDITY_CALL revert InsufficientShares()()
 baseAmount = totalAccRewards(existingShares,accBaseRewardsPerShare) - userData.baseRewardDebt
TMP_3506(uint256) = INTERNAL_CALL, RewardsTrackerLib.totalAccRewards(uint256,uint256)(existingShares_1,accBaseRewardsPerShare_1)
REF_1267(uint96) -> userData_1 (-> ['self']).baseRewardDebt
TMP_3507(uint256) = TMP_3506 (c)- REF_1267
baseAmount_1(uint256) := TMP_3507(uint256)
 quoteAmount = totalAccRewards(existingShares,accQuoteRewardsPerShare) - userData.quoteRewardDebt
TMP_3508(uint256) = INTERNAL_CALL, RewardsTrackerLib.totalAccRewards(uint256,uint256)(existingShares_1,accQuoteRewardsPerShare_1)
REF_1268(uint96) -> userData_1 (-> ['self']).quoteRewardDebt
TMP_3509(uint256) = TMP_3508 (c)- REF_1268
quoteAmount_1(uint256) := TMP_3509(uint256)
 userData.shares -= removeShares
REF_1269(uint96) -> userData_1 (-> ['self']).shares
userData_2 (-> ['self'])(UserRewardData) := phi(["userData_1 (-> ['self'])"])
REF_1269(-> userData_2 (-> ['self'])) = REF_1269 (c)- removeShares_1
self_3 (-> ['self'])(RewardPoolData) := phi(["userData_2 (-> ['self'])"])
 self.totalShares -= removeShares
REF_1270(uint96) -> self_1 (-> []).totalShares
self_2 (-> [])(RewardPoolData) := phi(['self_1 (-> [])'])
REF_1270(-> self_2 (-> [])) = REF_1270 (c)- removeShares_1
 userData.baseRewardDebt = uint96(totalAccRewards(existingShares - removeShares,accBaseRewardsPerShare))
REF_1271(uint96) -> userData_2 (-> ['self']).baseRewardDebt
TMP_3510(uint256) = existingShares_1 (c)- removeShares_1
TMP_3511(uint256) = INTERNAL_CALL, RewardsTrackerLib.totalAccRewards(uint256,uint256)(TMP_3510,accBaseRewardsPerShare_1)
TMP_3512 = CONVERT TMP_3511 to uint96
userData_3 (-> ['self'])(UserRewardData) := phi(["userData_2 (-> ['self'])"])
REF_1271(uint96) (->userData_3 (-> ['self'])) := TMP_3512(uint96)
self_4 (-> ['self'])(RewardPoolData) := phi(["userData_3 (-> ['self'])"])
 userData.quoteRewardDebt = uint96(totalAccRewards(existingShares - removeShares,accQuoteRewardsPerShare))
REF_1272(uint96) -> userData_3 (-> ['self']).quoteRewardDebt
TMP_3513(uint256) = existingShares_1 (c)- removeShares_1
TMP_3514(uint256) = INTERNAL_CALL, RewardsTrackerLib.totalAccRewards(uint256,uint256)(TMP_3513,accQuoteRewardsPerShare_1)
TMP_3515 = CONVERT TMP_3514 to uint96
userData_4 (-> ['self'])(UserRewardData) := phi(["userData_3 (-> ['self'])"])
REF_1272(uint96) (->userData_4 (-> ['self'])) := TMP_3515(uint96)
self_5 (-> ['self'])(RewardPoolData) := phi(["userData_4 (-> ['self'])"])
 (baseAmount,quoteAmount)
RETURN baseAmount_1,quoteAmount_1
```

#### RewardsTrackerLib.getPendingRewards(RewardPoolData,address) [INTERNAL]
```slithir
 (accBaseRewardsPerShare,accQuoteRewardsPerShare) = getAccRewardsPerShare(self)
TUPLE_31(uint256,uint256) = INTERNAL_CALL, RewardsTrackerLib.getAccRewardsPerShare(RewardPoolData)(self_1 (-> []))
accBaseRewardsPerShare_1(uint256)= UNPACK TUPLE_31 index: 0 
accQuoteRewardsPerShare_1(uint256)= UNPACK TUPLE_31 index: 1 
 userData = self.userRewards[user]
REF_1281(mapping(address => UserRewardData)) -> self_1 (-> []).userRewards
REF_1282(UserRewardData) -> REF_1281[user_1]
userData_1 (-> ['self'])(UserRewardData) := REF_1282(UserRewardData)
 shares = uint256(userData.shares)
REF_1283(uint96) -> userData_1 (-> ['self']).shares
TMP_3527 = CONVERT REF_1283 to uint256
shares_1(uint256) := TMP_3527(uint256)
 shares == 0
TMP_3528(bool) = shares_1 == 0
CONDITION TMP_3528
 (0,0)
RETURN 0,0
 baseAmount = totalAccRewards(shares,accBaseRewardsPerShare) - uint128(userData.baseRewardDebt)
TMP_3529(uint256) = INTERNAL_CALL, RewardsTrackerLib.totalAccRewards(uint256,uint256)(shares_1,accBaseRewardsPerShare_1)
REF_1284(uint96) -> userData_1 (-> ['self']).baseRewardDebt
TMP_3530 = CONVERT REF_1284 to uint128
TMP_3531(uint256) = TMP_3529 (c)- TMP_3530
baseAmount_1(uint256) := TMP_3531(uint256)
 quoteAmount = totalAccRewards(shares,accQuoteRewardsPerShare) - uint128(userData.quoteRewardDebt)
TMP_3532(uint256) = INTERNAL_CALL, RewardsTrackerLib.totalAccRewards(uint256,uint256)(shares_1,accQuoteRewardsPerShare_1)
REF_1285(uint96) -> userData_1 (-> ['self']).quoteRewardDebt
TMP_3533 = CONVERT REF_1285 to uint128
TMP_3534(uint256) = TMP_3532 (c)- TMP_3533
quoteAmount_1(uint256) := TMP_3534(uint256)
 (baseAmount,quoteAmount)
RETURN baseAmount_1,quoteAmount_1
```
#### RewardsTrackerLib.getRewardsPoolData(RewardPoolData) [INTERNAL]
```slithir
 pm = RewardPoolDataMemory({quoteAsset:self.quoteAsset,totalShares:self.totalShares,pendingBaseRewards:self.pendingBaseRewards,pendingQuoteRewards:self.pendingQuoteRewards,accBaseRewardPerShare:self.accBaseRewardPerShare,accQuoteRewardPerShare:self.accQuoteRewardPerShare})
REF_1244(address) -> self_1 (-> []).quoteAsset
REF_1245(uint96) -> self_1 (-> []).totalShares
REF_1246(uint128) -> self_1 (-> []).pendingBaseRewards
REF_1247(uint128) -> self_1 (-> []).pendingQuoteRewards
REF_1248(uint256) -> self_1 (-> []).accBaseRewardPerShare
REF_1249(uint256) -> self_1 (-> []).accQuoteRewardPerShare
TMP_3483(RewardPoolDataMemory) = new RewardPoolDataMemory(REF_1245,REF_1244,REF_1246,REF_1247,REF_1248,REF_1249)
pm_1(RewardPoolDataMemory) := TMP_3483(RewardPoolDataMemory)
 pm
RETURN pm_1
```
#### RewardsTrackerLib.getUserData(RewardPoolData,address) [INTERNAL]
```slithir
 self.userRewards[account]
REF_1242(mapping(address => UserRewardData)) -> self_1 (-> []).userRewards
REF_1243(UserRewardData) -> REF_1242[account_1]
RETURN REF_1243
```
#### RewardsTrackerLib.stake(RewardPoolData,address,uint96) [INTERNAL]
```slithir
 newShares == 0
TMP_3487(bool) = newShares_1 == 0
CONDITION TMP_3487
 revert ZeroShareStake()()
TMP_3488(None) = SOLIDITY_CALL revert ZeroShareStake()()
 (accBaseRewardsPerShare,accQuoteRewardsPerShare) = self.update()
TUPLE_28(uint256,uint256) = LIBRARY_CALL, dest:RewardsTrackerLib, function:RewardsTrackerLib.update(RewardPoolData), arguments:['self_1 (-> [])'] 
accBaseRewardsPerShare_1(uint256)= UNPACK TUPLE_28 index: 0 
accQuoteRewardsPerShare_1(uint256)= UNPACK TUPLE_28 index: 1 
 userData = self.userRewards[user]
REF_1254(mapping(address => UserRewardData)) -> self_1 (-> []).userRewards
REF_1255(UserRewardData) -> REF_1254[user_1]
userData_1 (-> ['self'])(UserRewardData) := REF_1255(UserRewardData)
 existingShares = uint96(userData.shares)
REF_1256(uint96) -> userData_1 (-> ['self']).shares
TMP_3489 = CONVERT REF_1256 to uint96
existingShares_1(uint256) := TMP_3489(uint96)
 existingShares > 0
TMP_3490(bool) = existingShares_1 > 0
CONDITION TMP_3490
 baseAmount = totalAccRewards(existingShares,accBaseRewardsPerShare) - userData.baseRewardDebt
TMP_3491(uint256) = INTERNAL_CALL, RewardsTrackerLib.totalAccRewards(uint256,uint256)(existingShares_1,accBaseRewardsPerShare_1)
REF_1257(uint96) -> userData_1 (-> ['self']).baseRewardDebt
TMP_3492(uint256) = TMP_3491 (c)- REF_1257
baseAmount_1(uint256) := TMP_3492(uint256)
 quoteAmount = totalAccRewards(existingShares,accQuoteRewardsPerShare) - userData.quoteRewardDebt
TMP_3493(uint256) = INTERNAL_CALL, RewardsTrackerLib.totalAccRewards(uint256,uint256)(existingShares_1,accQuoteRewardsPerShare_1)
REF_1258(uint96) -> userData_1 (-> ['self']).quoteRewardDebt
TMP_3494(uint256) = TMP_3493 (c)- REF_1258
quoteAmount_1(uint256) := TMP_3494(uint256)
baseAmount_2(uint256) := phi(['baseAmount_1', 'baseAmount_0'])
quoteAmount_2(uint256) := phi(['quoteAmount_0', 'quoteAmount_1'])
 userData.shares += newShares
REF_1259(uint96) -> userData_1 (-> ['self']).shares
userData_2 (-> ['self'])(UserRewardData) := phi(["userData_1 (-> ['self'])"])
REF_1259(-> userData_2 (-> ['self'])) = REF_1259 (c)+ newShares_1
self_3 (-> ['self'])(RewardPoolData) := phi(["userData_2 (-> ['self'])"])
 self.totalShares += newShares
REF_1260(uint96) -> self_1 (-> []).totalShares
self_2 (-> [])(RewardPoolData) := phi(['self_1 (-> [])'])
REF_1260(-> self_2 (-> [])) = REF_1260 (c)+ newShares_1
 userData.baseRewardDebt = uint96(totalAccRewards(existingShares + newShares,accBaseRewardsPerShare))
REF_1261(uint96) -> userData_2 (-> ['self']).baseRewardDebt
TMP_3495(uint256) = existingShares_1 (c)+ newShares_1
TMP_3496(uint256) = INTERNAL_CALL, RewardsTrackerLib.totalAccRewards(uint256,uint256)(TMP_3495,accBaseRewardsPerShare_1)
TMP_3497 = CONVERT TMP_3496 to uint96
userData_3 (-> ['self'])(UserRewardData) := phi(["userData_2 (-> ['self'])"])
REF_1261(uint96) (->userData_3 (-> ['self'])) := TMP_3497(uint96)
self_4 (-> ['self'])(RewardPoolData) := phi(["userData_3 (-> ['self'])"])
 userData.quoteRewardDebt = uint96(totalAccRewards(existingShares + newShares,accQuoteRewardsPerShare))
REF_1262(uint96) -> userData_3 (-> ['self']).quoteRewardDebt
TMP_3498(uint256) = existingShares_1 (c)+ newShares_1
TMP_3499(uint256) = INTERNAL_CALL, RewardsTrackerLib.totalAccRewards(uint256,uint256)(TMP_3498,accQuoteRewardsPerShare_1)
TMP_3500 = CONVERT TMP_3499 to uint96
userData_4 (-> ['self'])(UserRewardData) := phi(["userData_3 (-> ['self'])"])
REF_1262(uint96) (->userData_4 (-> ['self'])) := TMP_3500(uint96)
self_5 (-> ['self'])(RewardPoolData) := phi(["userData_4 (-> ['self'])"])
 (baseAmount,quoteAmount)
RETURN baseAmount_2,quoteAmount_2
```
#### SafeTransferLib.balanceOf(address,address) [INTERNAL]
```slithir
 mstore(uint256,uint256)(0x14,account)
TMP_15421(None) = SOLIDITY_CALL mstore(uint256,uint256)(20,account_1)
 mstore(uint256,uint256)(0x00,0x70a08231000000000000000000000000)
TMP_15422(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,149706943620704588101898925390394556416)
 amount = mload(uint256)(0x20) * returndatasize()() > 0x1f & staticcall(uint256,uint256,uint256,uint256,uint256,uint256)(gas()(),token,0x10,0x24,0x20,0x20)
TMP_15423(uint256) = SOLIDITY_CALL mload(uint256)(32)
TMP_15424(uint256) = SOLIDITY_CALL returndatasize()()
TMP_15425(bool) = TMP_15424 > 31
TMP_15426(uint256) = SOLIDITY_CALL gas()()
TMP_15427(uint256) = SOLIDITY_CALL staticcall(uint256,uint256,uint256,uint256,uint256,uint256)(TMP_15426,token_1,16,36,32,32)
TMP_15428(bool) = TMP_15425 & TMP_15427
TMP_15429(uint256) = TMP_15423 * TMP_15428
amount_1(uint256) := TMP_15429(uint256)
 amount
RETURN amount_1
```
#### RewardsTrackerStorage.rewardPoolSlot(address) [PRIVATE]
```slithir
baseAsset_1(address) := phi(['baseAsset_1'])
LAUNCH_ASSET_TO_REWARDS_SLOT_1(bytes32) := phi(['LAUNCH_ASSET_TO_REWARDS_SLOT_0'])
 keccak256(bytes)(abi.encodePacked(baseAsset,LAUNCH_ASSET_TO_REWARDS_SLOT))
TMP_3548(bytes) = SOLIDITY_CALL abi.encodePacked()(baseAsset_1,LAUNCH_ASSET_TO_REWARDS_SLOT_1)
TMP_3549(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_3548)
RETURN TMP_3549
```
#### RewardsTrackerLib.totalAccRewards(uint256,uint256) [INTERNAL]
```slithir
shares_1(uint256) := phi(['existingShares_1', 'existingShares_1', 'TMP_3510', 'TMP_3495', 'TMP_3513', 'shares_1', 'shares_1', 'TMP_3498'])
accRewardsPerShare_1(uint256) := phi(['accBaseRewardsPerShare_1', 'accQuoteRewardsPerShare_1', 'accBaseRewardsPerShare_1', 'accQuoteRewardsPerShare_1', 'accBaseRewardsPerShare_1', 'accBaseRewardsPerShare_1', 'accQuoteRewardsPerShare_1', 'accQuoteRewardsPerShare_1'])
PRECISION_FACTOR_1(uint128) := phi(['PRECISION_FACTOR_0'])
 (shares * accRewardsPerShare) / PRECISION_FACTOR
TMP_3535(uint256) = shares_1 (c)* accRewardsPerShare_1
TMP_3536(uint256) = TMP_3535 (c)/ PRECISION_FACTOR_1
RETURN TMP_3536
```
#### RewardsTrackerLib.update(RewardPoolData) [INTERNAL]
```slithir
 (newAccBaseRewardsPerShare,newAccQuoteRewardsPerShare) = getAccRewardsPerShare(self)
TUPLE_32(uint256,uint256) = INTERNAL_CALL, RewardsTrackerLib.getAccRewardsPerShare(RewardPoolData)(self_1 (-> []))
newAccBaseRewardsPerShare_1(uint256)= UNPACK TUPLE_32 index: 0 
newAccQuoteRewardsPerShare_1(uint256)= UNPACK TUPLE_32 index: 1 
 self.pendingBaseRewards > 0
REF_1286(uint128) -> self_1 (-> []).pendingBaseRewards
TMP_3537(bool) = REF_1286 > 0
CONDITION TMP_3537
 self.accBaseRewardPerShare = newAccBaseRewardsPerShare
REF_1287(uint256) -> self_1 (-> []).accBaseRewardPerShare
self_2 (-> [])(RewardPoolData) := phi(['self_1 (-> [])'])
REF_1287(uint256) (->self_2 (-> [])) := newAccBaseRewardsPerShare_1(uint256)
 delete self.pendingBaseRewards
REF_1288(uint128) -> self_2 (-> []).pendingBaseRewards
self_3 (-> []) = delete REF_1288 
self_4 (-> [])(RewardPoolData) := phi(['self_1 (-> [])', 'self_3 (-> [])'])
 self.pendingQuoteRewards > 0
REF_1289(uint128) -> self_4 (-> []).pendingQuoteRewards
TMP_3538(bool) = REF_1289 > 0
CONDITION TMP_3538
 self.accQuoteRewardPerShare = newAccQuoteRewardsPerShare
REF_1290(uint256) -> self_4 (-> []).accQuoteRewardPerShare
self_5 (-> [])(RewardPoolData) := phi(['self_4 (-> [])'])
REF_1290(uint256) (->self_5 (-> [])) := newAccQuoteRewardsPerShare_1(uint256)
 delete self.pendingQuoteRewards
REF_1291(uint128) -> self_5 (-> []).pendingQuoteRewards
self_6 (-> []) = delete REF_1291 
 (newAccBaseRewardsPerShare,newAccQuoteRewardsPerShare)
RETURN newAccBaseRewardsPerShare_1,newAccQuoteRewardsPerShare_1
```
#### RewardsTrackerLib.getAccRewardsPerShare(RewardPoolData) [INTERNAL]
```slithir
self_1 (-> [])(RewardPoolData) := phi(['self_1 (-> [])', 'self_1 (-> [])'])
PRECISION_FACTOR_2(uint128) := phi(['PRECISION_FACTOR_0'])
 totalShares = self.totalShares
REF_1292(uint96) -> self_1 (-> []).totalShares
totalShares_1(uint96) := REF_1292(uint96)
 totalShares == 0
TMP_3539(bool) = totalShares_1 == 0
CONDITION TMP_3539
 (self.accBaseRewardPerShare,self.accQuoteRewardPerShare)
REF_1293(uint256) -> self_1 (-> []).accBaseRewardPerShare
REF_1294(uint256) -> self_1 (-> []).accQuoteRewardPerShare
RETURN REF_1293,REF_1294
 accBaseRewardsPerShare = self.accBaseRewardPerShare
REF_1295(uint256) -> self_1 (-> []).accBaseRewardPerShare
accBaseRewardsPerShare_1(uint256) := REF_1295(uint256)
 accQuoteRewardsPerShare = self.accQuoteRewardPerShare
REF_1296(uint256) -> self_1 (-> []).accQuoteRewardPerShare
accQuoteRewardsPerShare_1(uint256) := REF_1296(uint256)
 self.pendingBaseRewards > 0
REF_1297(uint128) -> self_1 (-> []).pendingBaseRewards
TMP_3540(bool) = REF_1297 > 0
CONDITION TMP_3540
 accBaseRewardsPerShare += ((self.pendingBaseRewards * PRECISION_FACTOR) / uint128(totalShares))
REF_1298(uint128) -> self_1 (-> []).pendingBaseRewards
TMP_3541(uint128) = REF_1298 (c)* PRECISION_FACTOR_2
TMP_3542 = CONVERT totalShares_1 to uint128
TMP_3543(uint128) = TMP_3541 (c)/ TMP_3542
accBaseRewardsPerShare_2(uint256) = accBaseRewardsPerShare_1 (c)+ TMP_3543
accBaseRewardsPerShare_3(uint256) := phi(['accBaseRewardsPerShare_2', 'accBaseRewardsPerShare_1'])
 self.pendingQuoteRewards > 0
REF_1299(uint128) -> self_1 (-> []).pendingQuoteRewards
TMP_3544(bool) = REF_1299 > 0
CONDITION TMP_3544
 accQuoteRewardsPerShare += ((self.pendingQuoteRewards * PRECISION_FACTOR) / uint128(totalShares))
REF_1300(uint128) -> self_1 (-> []).pendingQuoteRewards
TMP_3545(uint128) = REF_1300 (c)* PRECISION_FACTOR_2
TMP_3546 = CONVERT totalShares_1 to uint128
TMP_3547(uint128) = TMP_3545 (c)/ TMP_3546
accQuoteRewardsPerShare_2(uint256) = accQuoteRewardsPerShare_1 (c)+ TMP_3547
accQuoteRewardsPerShare_3(uint256) := phi(['accQuoteRewardsPerShare_2', 'accQuoteRewardsPerShare_1'])
 (accBaseRewardsPerShare,accQuoteRewardsPerShare)
RETURN accBaseRewardsPerShare_3,accQuoteRewardsPerShare_3
```
