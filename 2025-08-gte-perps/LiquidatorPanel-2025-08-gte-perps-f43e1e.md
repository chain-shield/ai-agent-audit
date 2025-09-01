


















#### LiquidatorPanel._balanceBadDebt(uint256,uint256) [INTERNAL]
```slithir
 fee > badDebt
TMP_7641(bool) = fee_1 > badDebt_1
CONDITION TMP_7641
 (fee - badDebt,0)
TMP_7642(uint256) = fee_1 (c)- badDebt_1
RETURN TMP_7642,0
 (0,badDebt - fee)
TMP_7643(uint256) = badDebt_1 (c)- fee_1
RETURN 0,TMP_7643
```
#### LiquidatorPanel._deleverage(ClearingHouse,LiquidatorPanel.__DeleverageParams__) [INTERNAL]
```slithir
clearingHouse_1 (-> ['TMP_7502'])(ClearingHouse) := phi(["clearingHouse_1 (-> ['TMP_7502'])"])
params_1(LiquidatorPanel.__DeleverageParams__) := phi(['TMP_7550', 'TMP_7552'])
 result = params.positions[params.positionIdx].processTrade({side:side,quoteTraded:params.quoteTraded,baseTraded:params.baseTraded})
REF_3319(Position[]) -> params_1.positions
REF_3320(uint256) -> params_1.positionIdx
REF_3321(Position) -> REF_3319[REF_3320]
REF_3323(uint256) -> params_1.quoteTraded
REF_3324(uint256) -> params_1.baseTraded
TMP_7554(PositionUpdateResult) = LIBRARY_CALL, dest:PositionLib, function:PositionLib.processTrade(Position,Side,uint256,uint256), arguments:['REF_3321', 'side_3', 'REF_3323', 'REF_3324'] 
result_1(PositionUpdateResult) := TMP_7554(PositionUpdateResult)
 params.margin += result.rpnl
REF_3325(int256) -> params_1.margin
REF_3326(int256) -> result_1.rpnl
params_2(LiquidatorPanel.__DeleverageParams__) := phi(['params_1'])
REF_3325(-> params_2) = REF_3325 (c)+ REF_3326
 (params.margin,result.marginDelta) = clearingHouse.rebalanceClose({assets:params.assets,positions:params.positions,margin:params.margin,marginDelta:0})
REF_3327(int256) -> params_2.margin
REF_3328(int256) -> result_1.marginDelta
REF_3330(DynamicArrayLib.DynamicArray) -> params_2.assets
REF_3331(Position[]) -> params_2.positions
REF_3332(int256) -> params_2.margin
TUPLE_65(int256,int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.rebalanceClose(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256,int256), arguments:["clearingHouse_1 (-> ['TMP_7502'])", 'REF_3330', 'REF_3331', 'REF_3332', '0'] 
REF_3327(int256)= UNPACK TUPLE_65 index: 0 
REF_3328(int256)= UNPACK TUPLE_65 index: 1 
 fullClose = params.positions[params.positionIdx].amount == 0 && params.positions.length == 1
REF_3333(Position[]) -> params_2.positions
REF_3334(uint256) -> params_2.positionIdx
REF_3335(Position) -> REF_3333[REF_3334]
REF_3336(uint256) -> REF_3335.amount
TMP_7555(bool) = REF_3336 == 0
REF_3337(Position[]) -> params_2.positions
REF_3338 -> LENGTH REF_3337
TMP_7556(bool) = REF_3338 == 1
TMP_7557(bool) = TMP_7555 && TMP_7556
fullClose_1(bool) := TMP_7557(bool)
 fullClose && params.margin < 0
REF_3339(int256) -> params_2.margin
TMP_7558(bool) = REF_3339 < 0
TMP_7559(bool) = fullClose_1 && TMP_7558
CONDITION TMP_7559
 badDebt = params.margin.abs()
REF_3340(int256) -> params_2.margin
TMP_7560(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['REF_3340'] 
badDebt_1(uint256) := TMP_7560(uint256)
 StorageLib.loadInsuranceFund().claim(badDebt)
TMP_7561(InsuranceFund) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadInsuranceFund(), arguments:[] 
LIBRARY_CALL, dest:InsuranceFundLib, function:InsuranceFundLib.claim(InsuranceFund,uint256), arguments:['TMP_7561', 'badDebt_1'] 
 delete params.margin
REF_3344(int256) -> params_2.margin
params_3 = delete REF_3344 
params_4(LiquidatorPanel.__DeleverageParams__) := phi(['params_3', 'params_2'])
badDebt_2(uint256) := phi(['badDebt_1', 'badDebt_0'])
 _emitLiquidationEvent({asset:params.asset,account:params.account,subaccount:params.subaccount,side:side,quoteTraded:params.quoteTraded,baseTraded:params.baseTraded,rpnl:result.rpnl,margin:params.margin,fee:- badDebt.toInt256(),liquidationType:params.deleverageType})
REF_3345(bytes32) -> params_4.asset
REF_3346(address) -> params_4.account
REF_3347(uint256) -> params_4.subaccount
REF_3348(uint256) -> params_4.quoteTraded
REF_3349(uint256) -> params_4.baseTraded
REF_3350(int256) -> result_1.rpnl
REF_3351(int256) -> params_4.margin
TMP_7563(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['badDebt_2'] 
TMP_7564(int256) = 0 (c)- TMP_7563
REF_3353(LiquidatorPanel.LiquidationType) -> params_4.deleverageType
INTERNAL_CALL, LiquidatorPanel._emitLiquidationEvent(bytes32,address,uint256,Side,uint256,uint256,int256,int256,int256,LiquidatorPanel.LiquidationType)(REF_3345,REF_3346,REF_3347,side_3,REF_3348,REF_3349,REF_3350,REF_3351,TMP_7564,REF_3353)
 StorageLib.loadCollateralManager().settleFill(params.account,params.subaccount,params.margin,result.marginDelta)
TMP_7566(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
REF_3356(address) -> params_4.account
REF_3357(uint256) -> params_4.subaccount
REF_3358(int256) -> params_4.margin
REF_3359(int256) -> result_1.marginDelta
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.settleFill(CollateralManager,address,uint256,int256,int256), arguments:['TMP_7566', 'REF_3356', 'REF_3357', 'REF_3358', 'REF_3359'] 
 clearingHouse.updateAccount({account:params.account,subaccount:params.subaccount,assets:params.assets,positions:params.positions,tradedAsset:params.asset,positionIdx:params.positionIdx,oiDelta:result.oiDelta,sideClose:result.sideClose})
REF_3361(address) -> params_4.account
REF_3362(uint256) -> params_4.subaccount
REF_3363(DynamicArrayLib.DynamicArray) -> params_4.assets
REF_3364(Position[]) -> params_4.positions
REF_3365(bytes32) -> params_4.asset
REF_3366(uint256) -> params_4.positionIdx
REF_3367(OIDelta) -> result_1.oiDelta
REF_3368(bool) -> result_1.sideClose
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.updateAccount(ClearingHouse,address,uint256,DynamicArrayLib.DynamicArray,Position[],bytes32,uint256,OIDelta,bool), arguments:["clearingHouse_1 (-> ['TMP_7502'])", 'REF_3361', 'REF_3362', 'REF_3363', 'REF_3364', 'REF_3365', 'REF_3366', 'REF_3367', 'REF_3368'] 
 params.positions[params.positionIdx].isLong
REF_3369(Position[]) -> params_1.positions
REF_3370(uint256) -> params_1.positionIdx
REF_3371(Position) -> REF_3369[REF_3370]
REF_3372(bool) -> REF_3371.isLong
CONDITION REF_3372
 side = Side.SELL
REF_3373(Side) -> Side.SELL
side_1(Side) := REF_3373(Side)
 side = Side.BUY
REF_3374(Side) -> Side.BUY
side_2(Side) := REF_3374(Side)
side_3(Side) := phi(['side_1', 'side_2'])
```
#### LiquidatorPanel._deleveragePair(ClearingHouse,bytes32,DeleveragePair) [INTERNAL]
```slithir
clearingHouse_1 (-> ['TMP_7502'])(ClearingHouse) := phi(["clearingHouse_1 (-> ['TMP_7502'])"])
asset_1(bytes32) := phi(['asset_1'])
pair_1(DeleveragePair) := phi(['REF_3146'])
 (cache.makerAssets,cache.makerPositions,cache.makerMargin) = clearingHouse.getAccountAndMargin(pair.maker.account,pair.maker.subaccount)
REF_3247(DynamicArrayLib.DynamicArray) -> cache_0.makerAssets
REF_3248(Position[]) -> cache_0.makerPositions
REF_3249(int256) -> cache_0.makerMargin
REF_3251(Account) -> pair_1.maker
REF_3252(address) -> REF_3251.account
REF_3253(Account) -> pair_1.maker
REF_3254(uint256) -> REF_3253.subaccount
TUPLE_63(DynamicArrayLib.DynamicArray,Position[],int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.getAccountAndMargin(ClearingHouse,address,uint256), arguments:["clearingHouse_1 (-> ['TMP_7502'])", 'REF_3252', 'REF_3254'] 
REF_3247(DynamicArrayLib.DynamicArray)= UNPACK TUPLE_63 index: 0 
REF_3248(Position[])= UNPACK TUPLE_63 index: 1 
REF_3249(int256)= UNPACK TUPLE_63 index: 2 
 (cache.takerAssets,cache.takerPositions,cache.takerMargin) = clearingHouse.getAccountAndMargin(pair.taker.account,pair.taker.subaccount)
REF_3255(DynamicArrayLib.DynamicArray) -> cache_0.takerAssets
REF_3256(Position[]) -> cache_0.takerPositions
REF_3257(int256) -> cache_0.takerMargin
REF_3259(Account) -> pair_1.taker
REF_3260(address) -> REF_3259.account
REF_3261(Account) -> pair_1.taker
REF_3262(uint256) -> REF_3261.subaccount
TUPLE_64(DynamicArrayLib.DynamicArray,Position[],int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.getAccountAndMargin(ClearingHouse,address,uint256), arguments:["clearingHouse_1 (-> ['TMP_7502'])", 'REF_3260', 'REF_3262'] 
REF_3255(DynamicArrayLib.DynamicArray)= UNPACK TUPLE_64 index: 0 
REF_3256(Position[])= UNPACK TUPLE_64 index: 1 
REF_3257(int256)= UNPACK TUPLE_64 index: 2 
 cache.makerMargin -= ClearingHouseLib.realizeFundingPayment(cache.makerAssets,cache.makerPositions)
REF_3263(int256) -> cache_0.makerMargin
REF_3265(DynamicArrayLib.DynamicArray) -> cache_0.makerAssets
REF_3266(Position[]) -> cache_0.makerPositions
TMP_7540(int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.realizeFundingPayment(DynamicArrayLib.DynamicArray,Position[]), arguments:['REF_3265', 'REF_3266'] 
cache_1(LiquidatorPanel.__DeleveragePairCache__) := phi(['cache_0'])
REF_3263(-> cache_1) = REF_3263 (c)- TMP_7540
 cache.takerMargin -= ClearingHouseLib.realizeFundingPayment(cache.takerAssets,cache.takerPositions)
REF_3267(int256) -> cache_1.takerMargin
REF_3269(DynamicArrayLib.DynamicArray) -> cache_1.takerAssets
REF_3270(Position[]) -> cache_1.takerPositions
TMP_7541(int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.realizeFundingPayment(DynamicArrayLib.DynamicArray,Position[]), arguments:['REF_3269', 'REF_3270'] 
cache_2(LiquidatorPanel.__DeleveragePairCache__) := phi(['cache_1'])
REF_3267(-> cache_2) = REF_3267 (c)- TMP_7541
 makerPositionIdx = cache.makerAssets.indexOf(asset)
REF_3271(DynamicArrayLib.DynamicArray) -> cache_2.makerAssets
TMP_7542(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.indexOf(DynamicArrayLib.DynamicArray,bytes32), arguments:['REF_3271', 'asset_1'] 
makerPositionIdx_1(uint256) := TMP_7542(uint256)
 takerPositionIdx = cache.takerAssets.indexOf(asset)
REF_3273(DynamicArrayLib.DynamicArray) -> cache_2.takerAssets
TMP_7543(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.indexOf(DynamicArrayLib.DynamicArray,bytes32), arguments:['REF_3273', 'asset_1'] 
takerPositionIdx_1(uint256) := TMP_7543(uint256)
 _validateDeleveragePair(clearingHouse,__DeleverageValidationParams__({asset:asset,makerAssets:cache.makerAssets,takerAssets:cache.takerAssets,makerPositions:cache.makerPositions,takerPositions:cache.takerPositions,makerPositionIdx:makerPositionIdx,takerPositionIdx:takerPositionIdx,makerMargin:cache.makerMargin,takerMargin:cache.takerMargin}))
REF_3275(DynamicArrayLib.DynamicArray) -> cache_2.makerAssets
REF_3276(DynamicArrayLib.DynamicArray) -> cache_2.takerAssets
REF_3277(Position[]) -> cache_2.makerPositions
REF_3278(Position[]) -> cache_2.takerPositions
REF_3279(int256) -> cache_2.makerMargin
REF_3280(int256) -> cache_2.takerMargin
TMP_7544(LiquidatorPanel.__DeleverageValidationParams__) = new __DeleverageValidationParams__(asset_1,REF_3275,REF_3276,REF_3277,REF_3278,makerPositionIdx_1,takerPositionIdx_1,REF_3279,REF_3280)
INTERNAL_CALL, LiquidatorPanel._validateDeleveragePair(ClearingHouse,LiquidatorPanel.__DeleverageValidationParams__)(clearingHouse_1 (-> ['TMP_7502']),TMP_7544)
 cache.baseAmount = cache.makerPositions[makerPositionIdx].amount.min(cache.takerPositions[takerPositionIdx].amount)
REF_3281(uint256) -> cache_2.baseAmount
REF_3282(Position[]) -> cache_2.makerPositions
REF_3283(Position) -> REF_3282[makerPositionIdx_1]
REF_3284(uint256) -> REF_3283.amount
REF_3286(Position[]) -> cache_2.takerPositions
REF_3287(Position) -> REF_3286[takerPositionIdx_1]
REF_3288(uint256) -> REF_3287.amount
TMP_7546(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.min(uint256,uint256), arguments:['REF_3284', 'REF_3288'] 
cache_3(LiquidatorPanel.__DeleveragePairCache__) := phi(['cache_2'])
REF_3281(uint256) (->cache_3) := TMP_7546(uint256)
 bankruptcyPrice = _getBankruptcyPrice({position:cache.makerPositions[makerPositionIdx],closeSize:cache.baseAmount,proratedMargin:clearingHouse.getProratedMargin(cache.makerAssets,cache.makerPositions,asset,cache.makerMargin)})
REF_3289(Position[]) -> cache_3.makerPositions
REF_3290(Position) -> REF_3289[makerPositionIdx_1]
REF_3291(uint256) -> cache_3.baseAmount
REF_3293(DynamicArrayLib.DynamicArray) -> cache_3.makerAssets
REF_3294(Position[]) -> cache_3.makerPositions
REF_3295(int256) -> cache_3.makerMargin
TMP_7547(int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.getProratedMargin(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],bytes32,int256), arguments:["clearingHouse_1 (-> ['TMP_7502'])", 'REF_3293', 'REF_3294', 'asset_1', 'REF_3295'] 
TMP_7548(uint256) = INTERNAL_CALL, LiquidatorPanel._getBankruptcyPrice(Position,uint256,int256)(REF_3290,REF_3291,TMP_7547)
bankruptcyPrice_1(uint256) := TMP_7548(uint256)
 cache.quoteAmount = cache.baseAmount.fullMulDiv(bankruptcyPrice,1e18)
REF_3296(uint256) -> cache_3.quoteAmount
REF_3297(uint256) -> cache_3.baseAmount
TMP_7549(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_3297', 'bankruptcyPrice_1', '1000000000000000000'] 
cache_4(LiquidatorPanel.__DeleveragePairCache__) := phi(['cache_3'])
REF_3296(uint256) (->cache_4) := TMP_7549(uint256)
 _deleverage(clearingHouse,__DeleverageParams__({account:pair.maker.account,subaccount:pair.maker.subaccount,asset:asset,assets:cache.makerAssets,positions:cache.makerPositions,positionIdx:makerPositionIdx,baseTraded:cache.baseAmount,quoteTraded:cache.quoteAmount,margin:cache.makerMargin,deleverageType:LiquidationType.DELEVERAGE_MAKER}))
REF_3299(Account) -> pair_1.maker
REF_3300(address) -> REF_3299.account
REF_3301(Account) -> pair_1.maker
REF_3302(uint256) -> REF_3301.subaccount
REF_3303(DynamicArrayLib.DynamicArray) -> cache_4.makerAssets
REF_3304(Position[]) -> cache_4.makerPositions
REF_3305(uint256) -> cache_4.baseAmount
REF_3306(uint256) -> cache_4.quoteAmount
REF_3307(int256) -> cache_4.makerMargin
REF_3308(LiquidatorPanel.LiquidationType) -> LiquidationType.DELEVERAGE_MAKER
TMP_7550(LiquidatorPanel.__DeleverageParams__) = new __DeleverageParams__(REF_3300,REF_3302,asset_1,REF_3303,REF_3304,makerPositionIdx_1,REF_3305,REF_3306,REF_3307,REF_3308)
INTERNAL_CALL, LiquidatorPanel._deleverage(ClearingHouse,LiquidatorPanel.__DeleverageParams__)(clearingHouse_1 (-> ['TMP_7502']),TMP_7550)
 _deleverage(clearingHouse,__DeleverageParams__({account:pair.taker.account,subaccount:pair.taker.subaccount,asset:asset,assets:cache.takerAssets,positions:cache.takerPositions,positionIdx:takerPositionIdx,baseTraded:cache.baseAmount,quoteTraded:cache.quoteAmount,margin:cache.takerMargin,deleverageType:LiquidationType.DELEVERAGE_TAKER}))
REF_3309(Account) -> pair_1.taker
REF_3310(address) -> REF_3309.account
REF_3311(Account) -> pair_1.taker
REF_3312(uint256) -> REF_3311.subaccount
REF_3313(DynamicArrayLib.DynamicArray) -> cache_4.takerAssets
REF_3314(Position[]) -> cache_4.takerPositions
REF_3315(uint256) -> cache_4.baseAmount
REF_3316(uint256) -> cache_4.quoteAmount
REF_3317(int256) -> cache_4.takerMargin
REF_3318(LiquidatorPanel.LiquidationType) -> LiquidationType.DELEVERAGE_TAKER
TMP_7552(LiquidatorPanel.__DeleverageParams__) = new __DeleverageParams__(REF_3310,REF_3312,asset_1,REF_3313,REF_3314,takerPositionIdx_1,REF_3315,REF_3316,REF_3317,REF_3318)
INTERNAL_CALL, LiquidatorPanel._deleverage(ClearingHouse,LiquidatorPanel.__DeleverageParams__)(clearingHouse_1 (-> ['TMP_7502']),TMP_7552)
```
#### LiquidatorPanel._delistClose(ClearingHouse,Market,bytes32,Account) [INTERNAL]
```slithir
clearingHouse_1 (-> ['TMP_7507'])(ClearingHouse) := phi(["clearingHouse_1 (-> ['TMP_7507'])"])
market_1 (-> ['clearingHouse'])(Market) := phi(["market_1 (-> ['clearingHouse'])"])
asset_1(bytes32) := phi(['asset_1'])
account_1(Account) := phi(['REF_3154'])
 (cache.assets,cache.positions,cache.margin) = clearingHouse.getAccountAndMargin(account.account,account.subaccount)
REF_3155(DynamicArrayLib.DynamicArray) -> cache_0.assets
REF_3156(Position[]) -> cache_0.positions
REF_3157(int256) -> cache_0.margin
REF_3159(address) -> account_1.account
REF_3160(uint256) -> account_1.subaccount
TUPLE_61(DynamicArrayLib.DynamicArray,Position[],int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.getAccountAndMargin(ClearingHouse,address,uint256), arguments:["clearingHouse_1 (-> ['TMP_7507'])", 'REF_3159', 'REF_3160'] 
REF_3155(DynamicArrayLib.DynamicArray)= UNPACK TUPLE_61 index: 0 
REF_3156(Position[])= UNPACK TUPLE_61 index: 1 
REF_3157(int256)= UNPACK TUPLE_61 index: 2 
 positionIdx = cache.assets.indexOf(asset)
REF_3161(DynamicArrayLib.DynamicArray) -> cache_0.assets
TMP_7515(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.indexOf(DynamicArrayLib.DynamicArray,bytes32), arguments:['REF_3161', 'asset_1'] 
positionIdx_1(uint256) := TMP_7515(uint256)
 positionIdx == type()(uint256).max
TMP_7517(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_7518(bool) = positionIdx_1 == TMP_7517
CONDITION TMP_7518
 cache.margin -= ClearingHouseLib.realizeFundingPayment(cache.assets,cache.positions)
REF_3163(int256) -> cache_0.margin
REF_3165(DynamicArrayLib.DynamicArray) -> cache_0.assets
REF_3166(Position[]) -> cache_0.positions
TMP_7519(int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.realizeFundingPayment(DynamicArrayLib.DynamicArray,Position[]), arguments:['REF_3165', 'REF_3166'] 
cache_1(LiquidatorPanel.__DelistCache__) := phi(['cache_0'])
REF_3163(-> cache_1) = REF_3163 (c)- TMP_7519
 cache.baseTraded = cache.positions[positionIdx].amount
REF_3167(uint256) -> cache_4.baseTraded
REF_3168(Position[]) -> cache_4.positions
REF_3169(Position) -> REF_3168[positionIdx_1]
REF_3170(uint256) -> REF_3169.amount
cache_5(LiquidatorPanel.__DelistCache__) := phi(['cache_4'])
REF_3167(uint256) (->cache_5) := REF_3170(uint256)
 cache.quoteTraded = cache.baseTraded.fullMulDiv(market.markPrice,1e18)
REF_3171(uint256) -> cache_5.quoteTraded
REF_3172(uint256) -> cache_5.baseTraded
REF_3174(uint256) -> market_1 (-> ['clearingHouse']).markPrice
TMP_7520(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_3172', 'REF_3174', '1000000000000000000'] 
cache_6(LiquidatorPanel.__DelistCache__) := phi(['cache_5'])
REF_3171(uint256) (->cache_6) := TMP_7520(uint256)
 cache.positionResult = cache.positions[positionIdx].processTrade({side:cache.side,quoteTraded:cache.quoteTraded,baseTraded:cache.baseTraded})
REF_3175(PositionUpdateResult) -> cache_6.positionResult
REF_3176(Position[]) -> cache_6.positions
REF_3177(Position) -> REF_3176[positionIdx_1]
REF_3179(Side) -> cache_6.side
REF_3180(uint256) -> cache_6.quoteTraded
REF_3181(uint256) -> cache_6.baseTraded
TMP_7521(PositionUpdateResult) = LIBRARY_CALL, dest:PositionLib, function:PositionLib.processTrade(Position,Side,uint256,uint256), arguments:['REF_3177', 'REF_3179', 'REF_3180', 'REF_3181'] 
cache_7(LiquidatorPanel.__DelistCache__) := phi(['cache_6'])
REF_3175(PositionUpdateResult) (->cache_7) := TMP_7521(PositionUpdateResult)
 cache.fee = StorageLib.loadFeeManager().getTakerFee(account.account,cache.quoteTraded).toInt256()
REF_3182(int256) -> cache_7.fee
TMP_7522(FeeManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadFeeManager(), arguments:[] 
REF_3185(address) -> account_1.account
REF_3186(uint256) -> cache_7.quoteTraded
TMP_7523(uint256) = LIBRARY_CALL, dest:FeeManagerLib, function:FeeManagerLib.getTakerFee(FeeManager,address,uint256), arguments:['TMP_7522', 'REF_3185', 'REF_3186'] 
TMP_7524(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['TMP_7523'] 
cache_8(LiquidatorPanel.__DelistCache__) := phi(['cache_7'])
REF_3182(int256) (->cache_8) := TMP_7524(int256)
 cache.margin += cache.positionResult.rpnl - cache.fee
REF_3188(int256) -> cache_8.margin
REF_3189(PositionUpdateResult) -> cache_8.positionResult
REF_3190(int256) -> REF_3189.rpnl
REF_3191(int256) -> cache_8.fee
TMP_7525(int256) = REF_3190 (c)- REF_3191
cache_9(LiquidatorPanel.__DelistCache__) := phi(['cache_8'])
REF_3188(-> cache_9) = REF_3188 (c)+ TMP_7525
 (cache.margin,cache.positionResult.marginDelta) = clearingHouse.rebalanceClose({assets:cache.assets,positions:cache.positions,margin:cache.margin,marginDelta:cache.positionResult.marginDelta})
REF_3192(int256) -> cache_9.margin
REF_3193(PositionUpdateResult) -> cache_9.positionResult
REF_3194(int256) -> REF_3193.marginDelta
REF_3196(DynamicArrayLib.DynamicArray) -> cache_9.assets
REF_3197(Position[]) -> cache_9.positions
REF_3198(int256) -> cache_9.margin
REF_3199(PositionUpdateResult) -> cache_9.positionResult
REF_3200(int256) -> REF_3199.marginDelta
TUPLE_62(int256,int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.rebalanceClose(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256,int256), arguments:["clearingHouse_1 (-> ['TMP_7507'])", 'REF_3196', 'REF_3197', 'REF_3198', 'REF_3200'] 
REF_3192(int256)= UNPACK TUPLE_62 index: 0 
REF_3194(int256)= UNPACK TUPLE_62 index: 1 
 cache.margin < 0 && cache.positions.length == 1
REF_3201(int256) -> cache_9.margin
TMP_7526(bool) = REF_3201 < 0
REF_3202(Position[]) -> cache_9.positions
REF_3203 -> LENGTH REF_3202
TMP_7527(bool) = REF_3203 == 1
TMP_7528(bool) = TMP_7526 && TMP_7527
CONDITION TMP_7528
 cache.fee += cache.margin
REF_3204(int256) -> cache_9.fee
REF_3205(int256) -> cache_9.margin
cache_10(LiquidatorPanel.__DelistCache__) := phi(['cache_9'])
REF_3204(-> cache_10) = REF_3204 (c)+ REF_3205
 delete cache.margin
REF_3206(int256) -> cache_10.margin
cache_11 = delete REF_3206 
cache_12(LiquidatorPanel.__DelistCache__) := phi(['cache_9', 'cache_11'])
 _emitLiquidationEvent({asset:asset,account:account.account,subaccount:account.subaccount,side:cache.side,quoteTraded:cache.quoteTraded,baseTraded:cache.baseTraded,rpnl:cache.positionResult.rpnl,margin:cache.margin,fee:cache.fee,liquidationType:LiquidationType.DELIST})
REF_3207(address) -> account_1.account
REF_3208(uint256) -> account_1.subaccount
REF_3209(Side) -> cache_12.side
REF_3210(uint256) -> cache_12.quoteTraded
REF_3211(uint256) -> cache_12.baseTraded
REF_3212(PositionUpdateResult) -> cache_12.positionResult
REF_3213(int256) -> REF_3212.rpnl
REF_3214(int256) -> cache_12.margin
REF_3215(int256) -> cache_12.fee
REF_3216(LiquidatorPanel.LiquidationType) -> LiquidationType.DELIST
INTERNAL_CALL, LiquidatorPanel._emitLiquidationEvent(bytes32,address,uint256,Side,uint256,uint256,int256,int256,int256,LiquidatorPanel.LiquidationType)(asset_1,REF_3207,REF_3208,REF_3209,REF_3210,REF_3211,REF_3213,REF_3214,REF_3215,REF_3216)
 cache.fee > 0
REF_3217(int256) -> cache_12.fee
TMP_7530(bool) = REF_3217 > 0
CONDITION TMP_7530
 StorageLib.loadInsuranceFund().pay(cache.fee.abs())
TMP_7531(InsuranceFund) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadInsuranceFund(), arguments:[] 
REF_3220(int256) -> cache_12.fee
TMP_7532(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['REF_3220'] 
LIBRARY_CALL, dest:InsuranceFundLib, function:InsuranceFundLib.pay(InsuranceFund,uint256), arguments:['TMP_7531', 'TMP_7532'] 
 StorageLib.loadInsuranceFund().claim(cache.fee.abs())
TMP_7534(InsuranceFund) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadInsuranceFund(), arguments:[] 
REF_3224(int256) -> cache_12.fee
TMP_7535(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['REF_3224'] 
LIBRARY_CALL, dest:InsuranceFundLib, function:InsuranceFundLib.claim(InsuranceFund,uint256), arguments:['TMP_7534', 'TMP_7535'] 
 StorageLib.loadCollateralManager().settleFill(account.account,account.subaccount,cache.margin,cache.positionResult.marginDelta)
TMP_7537(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
REF_3228(address) -> account_1.account
REF_3229(uint256) -> account_1.subaccount
REF_3230(int256) -> cache_12.margin
REF_3231(PositionUpdateResult) -> cache_12.positionResult
REF_3232(int256) -> REF_3231.marginDelta
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.settleFill(CollateralManager,address,uint256,int256,int256), arguments:['TMP_7537', 'REF_3228', 'REF_3229', 'REF_3230', 'REF_3232'] 
 clearingHouse.updateAccount({account:account.account,subaccount:account.subaccount,assets:cache.assets,positions:cache.positions,tradedAsset:asset,positionIdx:positionIdx,oiDelta:cache.positionResult.oiDelta,sideClose:true})
REF_3234(address) -> account_1.account
REF_3235(uint256) -> account_1.subaccount
REF_3236(DynamicArrayLib.DynamicArray) -> cache_12.assets
REF_3237(Position[]) -> cache_12.positions
REF_3238(PositionUpdateResult) -> cache_12.positionResult
REF_3239(OIDelta) -> REF_3238.oiDelta
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.updateAccount(ClearingHouse,address,uint256,DynamicArrayLib.DynamicArray,Position[],bytes32,uint256,OIDelta,bool), arguments:["clearingHouse_1 (-> ['TMP_7507'])", 'REF_3234', 'REF_3235', 'REF_3236', 'REF_3237', 'asset_1', 'positionIdx_1', 'REF_3239', 'True'] 
 cache.positions[positionIdx].isLong
REF_3240(Position[]) -> cache_1.positions
REF_3241(Position) -> REF_3240[positionIdx_1]
REF_3242(bool) -> REF_3241.isLong
CONDITION REF_3242
 cache.side = Side.SELL
REF_3243(Side) -> cache_1.side
REF_3244(Side) -> Side.SELL
cache_2(LiquidatorPanel.__DelistCache__) := phi(['cache_1'])
REF_3243(Side) (->cache_2) := REF_3244(Side)
 cache.side = Side.BUY
REF_3245(Side) -> cache_1.side
REF_3246(Side) -> Side.BUY
cache_3(LiquidatorPanel.__DelistCache__) := phi(['cache_1'])
REF_3245(Side) (->cache_3) := REF_3246(Side)
cache_4(LiquidatorPanel.__DelistCache__) := phi(['cache_2', 'cache_3'])
```
#### LiquidatorPanel._emitLiquidationEvent(bytes32,address,uint256,Side,uint256,uint256,int256,int256,int256,LiquidatorPanel.LiquidationType) [INTERNAL]
```slithir
asset_1(bytes32) := phi(['asset_1', 'asset_1', 'asset_1', 'REF_3345'])
account_1(address) := phi(['account_1', 'REF_3346', 'REF_3207', 'account_1'])
subaccount_1(uint256) := phi(['subaccount_1', 'REF_3347', 'subaccount_1', 'REF_3208'])
side_1(Side) := phi(['side_3', 'REF_3118', 'REF_3209', 'REF_3075'])
quoteTraded_1(uint256) := phi(['REF_3348', 'REF_3210', 'REF_3077', 'REF_3120'])
baseTraded_1(uint256) := phi(['REF_3211', 'REF_3122', 'REF_3349', 'REF_3079'])
rpnl_1(int256) := phi(['REF_3350', 'REF_3213', 'REF_3124', 'REF_3081'])
margin_1(int256) := phi(['REF_3351', 'REF_3214', 'REF_3125', 'REF_3082'])
fee_1(int256) := phi(['REF_3215', 'fee_3', 'fee_5', 'TMP_7564'])
liquidationType_1(LiquidatorPanel.LiquidationType) := phi(['REF_3126', 'REF_3083', 'REF_3353', 'REF_3216'])
 Liquidation({asset:asset,account:account,subaccount:subaccount,baseDelta:baseDelta,quoteDelta:quoteDelta,rpnl:rpnl,margin:margin,fee:fee,liquidationType:liquidationType,nonce:StorageLib.incNonce()})
TMP_7631(uint256) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.incNonce(), arguments:[] 
Emit Liquidation(asset_1,account_1,subaccount_1,baseDelta_3,quoteDelta_3,rpnl_1,margin_1,fee_1,liquidationType_1,TMP_7631)
 side == Side.BUY
REF_3503(Side) -> Side.BUY
TMP_7633(bool) = side_1 == REF_3503
CONDITION TMP_7633
 quoteDelta = - quoteTraded.toInt256()
TMP_7634(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['quoteTraded_1'] 
TMP_7635(uint256) = 0 (c)- TMP_7634
quoteDelta_1(int256) := TMP_7635(uint256)
 quoteDelta = quoteTraded.toInt256()
TMP_7636(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['quoteTraded_1'] 
quoteDelta_2(int256) := TMP_7636(int256)
quoteDelta_3(int256) := phi(['quoteDelta_1', 'quoteDelta_2'])
 side == Side.BUY
REF_3506(Side) -> Side.BUY
TMP_7637(bool) = side_1 == REF_3506
CONDITION TMP_7637
 baseDelta = baseTraded.toInt256()
TMP_7638(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['baseTraded_1'] 
baseDelta_1(int256) := TMP_7638(int256)
 baseDelta = - baseTraded.toInt256()
TMP_7639(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['baseTraded_1'] 
TMP_7640(uint256) = 0 (c)- TMP_7639
baseDelta_2(int256) := TMP_7640(uint256)
baseDelta_3(int256) := phi(['baseDelta_1', 'baseDelta_2'])
```
#### LiquidatorPanel._getBankruptcyPrice(Position,uint256,int256) [INTERNAL]
```slithir
position_1(Position) := phi(['REF_3290'])
closeSize_1(uint256) := phi(['REF_3291'])
proratedMargin_1(int256) := phi(['TMP_7547'])
 proratedMargin > 0
TMP_7569(bool) = proratedMargin_1 > 0
CONDITION TMP_7569
 proratedMargin = proratedMargin.abs().fullMulDiv(closeSize,position.amount).toInt256()
TMP_7570(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['proratedMargin_1'] 
REF_3377(uint256) -> position_1.amount
TMP_7571(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['TMP_7570', 'closeSize_1', 'REF_3377'] 
TMP_7572(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['TMP_7571'] 
proratedMargin_2(int256) := TMP_7572(int256)
 proratedMargin = - proratedMargin.abs().fullMulDiv(closeSize,position.amount).toInt256()
TMP_7573(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['proratedMargin_1'] 
REF_3381(uint256) -> position_1.amount
TMP_7574(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['TMP_7573', 'closeSize_1', 'REF_3381'] 
TMP_7575(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['TMP_7574'] 
TMP_7576(uint256) = 0 (c)- TMP_7575
proratedMargin_3(int256) := TMP_7576(uint256)
proratedMargin_4(int256) := phi(['proratedMargin_2', 'proratedMargin_3'])
 openNotional = position.openNotional.fullMulDiv(closeSize,position.amount)
REF_3383(uint256) -> position_1.openNotional
REF_3385(uint256) -> position_1.amount
TMP_7577(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_3383', 'closeSize_1', 'REF_3385'] 
openNotional_1(uint256) := TMP_7577(uint256)
 position.isLong
REF_3386(bool) -> position_1.isLong
CONDITION REF_3386
 numerator = openNotional.toInt256() - proratedMargin
TMP_7578(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['openNotional_1'] 
TMP_7579(int256) = TMP_7578 (c)- proratedMargin_4
numerator_1(int256) := TMP_7579(int256)
 numerator = openNotional.toInt256() + proratedMargin
TMP_7580(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['openNotional_1'] 
TMP_7581(int256) = TMP_7580 (c)+ proratedMargin_4
numerator_2(int256) := TMP_7581(int256)
numerator_3(int256) := phi(['numerator_1', 'numerator_2'])
 numerator < 0
TMP_7582(bool) = numerator_3 < 0
CONDITION TMP_7582
 0
RETURN 0
 bankruptcyPrice = numerator.toUint256().fullMulDiv(1e18,closeSize)
TMP_7583(uint256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toUint256(int256), arguments:['numerator_3'] 
TMP_7584(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['TMP_7583', '1000000000000000000', 'closeSize_1'] 
bankruptcyPrice_1(uint256) := TMP_7584(uint256)
 bankruptcyPrice
RETURN bankruptcyPrice_1
```
#### LiquidatorPanel._liquidate(ClearingHouse,bytes32,address,uint256,BookType) [INTERNAL]
```slithir
clearingHouse_1 (-> ['TMP_7479', 'TMP_7451'])(ClearingHouse) := phi(["clearingHouse_1 (-> ['TMP_7479'])", "clearingHouse_1 (-> ['TMP_7451'])"])
asset_1(bytes32) := phi(['asset_1', 'asset_1'])
account_1(address) := phi(['account_1', 'account_1'])
subaccount_1(uint256) := phi(['subaccount_1', 'subaccount_1'])
bookType_1(BookType) := phi(['REF_3104', 'REF_3037'])
 (cache.assets,cache.positions,cache.margin,cache.positionIdx) = _setupAccountAndValidateLiquidation({clearingHouse:clearingHouse,account:account,subaccount:subaccount,asset:asset,bookType:bookType})
REF_3414(DynamicArrayLib.DynamicArray) -> cache_0.assets
REF_3415(Position[]) -> cache_0.positions
REF_3416(int256) -> cache_0.margin
REF_3417(uint256) -> cache_0.positionIdx
TUPLE_67(DynamicArrayLib.DynamicArray,Position[],int256,uint256) = INTERNAL_CALL, LiquidatorPanel._setupAccountAndValidateLiquidation(ClearingHouse,address,uint256,bytes32,BookType)(clearingHouse_1 (-> ['TMP_7479', 'TMP_7451']),account_1,subaccount_1,asset_1,bookType_1)
REF_3414(DynamicArrayLib.DynamicArray)= UNPACK TUPLE_67 index: 0 
REF_3415(Position[])= UNPACK TUPLE_67 index: 1 
REF_3416(int256)= UNPACK TUPLE_67 index: 2 
REF_3417(uint256)= UNPACK TUPLE_67 index: 3 
 cache.fillResult = clearingHouse.market[asset].liquidate({account:account,subaccount:subaccount,side:cache.side,amount:cache.positions[cache.positionIdx].amount,bookType:bookType})
REF_3418(PlaceOrderResult) -> cache_3.fillResult
REF_3419(mapping(bytes32 => Market)) -> clearingHouse_1 (-> ['TMP_7479', 'TMP_7451']).market
REF_3420(Market) -> REF_3419[asset_1]
REF_3422(Side) -> cache_3.side
REF_3423(Position[]) -> cache_3.positions
REF_3424(uint256) -> cache_3.positionIdx
REF_3425(Position) -> REF_3423[REF_3424]
REF_3426(uint256) -> REF_3425.amount
TMP_7605(PlaceOrderResult) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.liquidate(Market,address,uint256,Side,uint256,BookType), arguments:['REF_3420', 'account_1', 'subaccount_1', 'REF_3422', 'REF_3426', 'bookType_1'] 
cache_4(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_3'])
REF_3418(PlaceOrderResult) (->cache_4) := TMP_7605(PlaceOrderResult)
 bookType == BookType.BACKSTOP
REF_3427(BookType) -> BookType.BACKSTOP
TMP_7606(bool) = bookType_1 == REF_3427
CONDITION TMP_7606
 proratedMargin = clearingHouse.getProratedMargin(cache.assets,cache.positions,asset,cache.margin)
REF_3429(DynamicArrayLib.DynamicArray) -> cache_4.assets
REF_3430(Position[]) -> cache_4.positions
REF_3431(int256) -> cache_4.margin
TMP_7607(int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.getProratedMargin(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],bytes32,int256), arguments:["clearingHouse_1 (-> ['TMP_7479', 'TMP_7451'])", 'REF_3429', 'REF_3430', 'asset_1', 'REF_3431'] 
proratedMargin_1(int256) := TMP_7607(int256)
 cache.maintenanceOrProratedMargin > 0 && cache.positions[cache.positionIdx].amount > cache.fillResult.baseTraded
REF_3432(uint256) -> cache_7.maintenanceOrProratedMargin
TMP_7608(bool) = REF_3432 > 0
REF_3433(Position[]) -> cache_7.positions
REF_3434(uint256) -> cache_7.positionIdx
REF_3435(Position) -> REF_3433[REF_3434]
REF_3436(uint256) -> REF_3435.amount
REF_3437(PlaceOrderResult) -> cache_7.fillResult
REF_3438(uint256) -> REF_3437.baseTraded
TMP_7609(bool) = REF_3436 > REF_3438
TMP_7610(bool) = TMP_7608 && TMP_7609
CONDITION TMP_7610
 cache.maintenanceOrProratedMargin = cache.maintenanceOrProratedMargin.fullMulDiv(cache.fillResult.baseTraded,cache.positions[cache.positionIdx].amount)
REF_3439(uint256) -> cache_7.maintenanceOrProratedMargin
REF_3440(uint256) -> cache_7.maintenanceOrProratedMargin
REF_3442(PlaceOrderResult) -> cache_7.fillResult
REF_3443(uint256) -> REF_3442.baseTraded
REF_3444(Position[]) -> cache_7.positions
REF_3445(uint256) -> cache_7.positionIdx
REF_3446(Position) -> REF_3444[REF_3445]
REF_3447(uint256) -> REF_3446.amount
TMP_7611(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_3440', 'REF_3443', 'REF_3447'] 
cache_8(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_7'])
REF_3439(uint256) (->cache_8) := TMP_7611(uint256)
cache_9(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_4', 'cache_8'])
 cache.maintenanceOrProratedMargin = clearingHouse.market[asset].getMaintenanceMargin(cache.positions[cache.positionIdx].amount)
REF_3448(uint256) -> cache_4.maintenanceOrProratedMargin
REF_3449(mapping(bytes32 => Market)) -> clearingHouse_1 (-> ['TMP_7479', 'TMP_7451']).market
REF_3450(Market) -> REF_3449[asset_1]
REF_3452(Position[]) -> cache_4.positions
REF_3453(uint256) -> cache_4.positionIdx
REF_3454(Position) -> REF_3452[REF_3453]
REF_3455(uint256) -> REF_3454.amount
TMP_7612(uint256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getMaintenanceMargin(Market,uint256), arguments:['REF_3450', 'REF_3455'] 
cache_10(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_4'])
REF_3448(uint256) (->cache_10) := TMP_7612(uint256)
cache_11(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_4', 'cache_10'])
 cache.positionResult = cache.positions[cache.positionIdx].processTrade({side:cache.side,quoteTraded:cache.fillResult.quoteTraded,baseTraded:cache.fillResult.baseTraded})
REF_3456(PositionUpdateResult) -> cache_11.positionResult
REF_3457(Position[]) -> cache_11.positions
REF_3458(uint256) -> cache_11.positionIdx
REF_3459(Position) -> REF_3457[REF_3458]
REF_3461(Side) -> cache_11.side
REF_3462(PlaceOrderResult) -> cache_11.fillResult
REF_3463(uint256) -> REF_3462.quoteTraded
REF_3464(PlaceOrderResult) -> cache_11.fillResult
REF_3465(uint256) -> REF_3464.baseTraded
TMP_7613(PositionUpdateResult) = LIBRARY_CALL, dest:PositionLib, function:PositionLib.processTrade(Position,Side,uint256,uint256), arguments:['REF_3459', 'REF_3461', 'REF_3463', 'REF_3465'] 
cache_12(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_11'])
REF_3456(PositionUpdateResult) (->cache_12) := TMP_7613(PositionUpdateResult)
 cache.positions[cache.positionIdx].isLong
REF_3466(Position[]) -> cache_0.positions
REF_3467(uint256) -> cache_0.positionIdx
REF_3468(Position) -> REF_3466[REF_3467]
REF_3469(bool) -> REF_3468.isLong
CONDITION REF_3469
 cache.side = Side.SELL
REF_3470(Side) -> cache_0.side
REF_3471(Side) -> Side.SELL
cache_1(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_0'])
REF_3470(Side) (->cache_1) := REF_3471(Side)
 cache.side = Side.BUY
REF_3472(Side) -> cache_0.side
REF_3473(Side) -> Side.BUY
cache_2(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_0'])
REF_3472(Side) (->cache_2) := REF_3473(Side)
cache_3(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_1', 'cache_2'])
 proratedMargin < 0
TMP_7614(bool) = proratedMargin_1 < 0
CONDITION TMP_7614
 cache.maintenanceOrProratedMargin = 0
REF_3474(uint256) -> cache_4.maintenanceOrProratedMargin
cache_6(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_4'])
REF_3474(uint256) (->cache_6) := 0(uint256)
 cache.maintenanceOrProratedMargin = proratedMargin.toUint256()
REF_3475(uint256) -> cache_4.maintenanceOrProratedMargin
TMP_7615(uint256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toUint256(int256), arguments:['proratedMargin_1'] 
cache_5(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_4'])
REF_3475(uint256) (->cache_5) := TMP_7615(uint256)
cache_7(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_5', 'cache_6'])
 cache
RETURN cache_12
```
#### LiquidatorPanel._settleBackstopLiquidation(ClearingHouse,bytes32,uint256) [INTERNAL]
```slithir
clearingHouse_1 (-> ['TMP_7479'])(ClearingHouse) := phi(["clearingHouse_1 (-> ['TMP_7479'])"])
asset_1(bytes32) := phi(['asset_1'])
margin_1(uint256) := phi(['TMP_7483'])
 data = BackstopLiquidatorDataLib.getLiquidatorDataAndClearStorage()
TMP_7616(LiquidatorData[]) = LIBRARY_CALL, dest:BackstopLiquidatorDataLib, function:BackstopLiquidatorDataLib.getLiquidatorDataAndClearStorage(), arguments:[] 
data_1(LiquidatorData[]) = ['TMP_7616(LiquidatorData[])']
 margin == 0
TMP_7617(bool) = margin_1 == 0
CONDITION TMP_7617
 0
RETURN 0
 liquidationFee = margin.fullMulDiv(StorageLib.loadMarketSettings(asset).liquidationFeeRate,1e18)
TMP_7618(MarketSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarketSettings(bytes32), arguments:['asset_1'] 
REF_3480(uint256) -> TMP_7618.liquidationFeeRate
TMP_7619(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['margin_1', 'REF_3480', '1000000000000000000'] 
liquidationFee_1(uint256) := TMP_7619(uint256)
 margin -= liquidationFee
margin_2(uint256) = margin_1 (c)- liquidationFee_1
 points = new uint256[](data.length)
REF_3481 -> LENGTH data_1
TMP_7621(uint256[])  = new uint256[](REF_3481)
points_1(uint256[]) = ['TMP_7621(uint256[])']
 i < data.length
totalPoints_1(uint256) := phi(['totalPoints_2', 'totalPoints_0'])
totalVolume_1(uint256) := phi(['totalVolume_2', 'totalVolume_0'])
i_1(uint256) := phi(['i_0', 'i_2'])
REF_3482 -> LENGTH data_1
TMP_7622(bool) = i_1 < REF_3482
CONDITION TMP_7622
 points[i] = clearingHouse.liquidatorPoints[data[i].liquidator]
REF_3483(uint256) -> points_1[i_1]
REF_3484(mapping(address => uint256)) -> clearingHouse_1 (-> ['TMP_7479']).liquidatorPoints
REF_3485(LiquidatorData) -> data_1[i_1]
REF_3486(address) -> REF_3485.liquidator
REF_3487(uint256) -> REF_3484[REF_3486]
points_2(uint256[]) := phi(['points_1'])
REF_3483(uint256) (->points_2) := REF_3487(uint256)
 totalPoints += points[i]
REF_3488(uint256) -> points_2[i_1]
totalPoints_2(uint256) = totalPoints_1 (c)+ REF_3488
 totalVolume += data[i].volume
REF_3489(LiquidatorData) -> data_1[i_1]
REF_3490(uint256) -> REF_3489.volume
totalVolume_2(uint256) = totalVolume_1 (c)+ REF_3490
 ++ i
i_2(uint256) = i_1 (c)+ 1
 i_scope_0 < data.length
i_scope_0_1(uint256) := phi(['i_scope_0_0', 'i_scope_0_2'])
REF_3491 -> LENGTH data_1
TMP_7623(bool) = i_scope_0_1 < REF_3491
CONDITION TMP_7623
 pointShare = points[i_scope_0].fullMulDiv(1e18,totalPoints)
REF_3492(uint256) -> points_1[i_scope_0_1]
TMP_7624(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_3492', '1000000000000000000', 'totalPoints_1'] 
pointShare_1(uint256) := TMP_7624(uint256)
 volumeShare = data[i_scope_0].volume.fullMulDiv(1e18,totalVolume)
REF_3494(LiquidatorData) -> data_1[i_scope_0_1]
REF_3495(uint256) -> REF_3494.volume
TMP_7625(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_3495', '1000000000000000000', 'totalVolume_1'] 
volumeShare_1(uint256) := TMP_7625(uint256)
 rate = (pointShare + volumeShare) / 2
TMP_7626(uint256) = pointShare_1 (c)+ volumeShare_1
TMP_7627(uint256) = TMP_7626 (c)/ 2
rate_1(uint256) := TMP_7627(uint256)
 fee = margin.fullMulDiv(rate,1e18)
TMP_7628(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['margin_2', 'rate_1', '1000000000000000000'] 
fee_1(uint256) := TMP_7628(uint256)
 StorageLib.loadCollateralManager().creditAccount(data[i_scope_0].liquidator,fee)
TMP_7629(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
REF_3500(LiquidatorData) -> data_1[i_scope_0_1]
REF_3501(address) -> REF_3500.liquidator
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.creditAccount(CollateralManager,address,uint256), arguments:['TMP_7629', 'REF_3501', 'fee_1'] 
 ++ i_scope_0
i_scope_0_2(uint256) = i_scope_0_1 (c)+ 1
 liquidationFee
RETURN liquidationFee_1
```
#### LiquidatorPanel._setupAccountAndValidateLiquidation(ClearingHouse,address,uint256,bytes32,BookType) [INTERNAL]
```slithir
clearingHouse_1 (-> [])(ClearingHouse) := phi(["clearingHouse_1 (-> ['TMP_7479', 'TMP_7451'])"])
account_1(address) := phi(['account_1'])
subaccount_1(uint256) := phi(['subaccount_1'])
asset_1(bytes32) := phi(['asset_1'])
bookType_1(BookType) := phi(['bookType_1'])
 (assets,positions,margin) = clearingHouse.getAccountAndMargin(account,subaccount)
TUPLE_66(DynamicArrayLib.DynamicArray,Position[],int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.getAccountAndMargin(ClearingHouse,address,uint256), arguments:['clearingHouse_1 (-> [])', 'account_1', 'subaccount_1'] 
assets_1(DynamicArrayLib.DynamicArray)= UNPACK TUPLE_66 index: 0 
positions_1(Position[])= UNPACK TUPLE_66 index: 1 
margin_1(int256)= UNPACK TUPLE_66 index: 2 
 positionIdx = assets.indexOf(asset)
TMP_7598(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.indexOf(DynamicArrayLib.DynamicArray,bytes32), arguments:['assets_1', 'asset_1'] 
positionIdx_1(uint256) := TMP_7598(uint256)
 positionIdx == type()(uint256).max
TMP_7600(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_7601(bool) = positionIdx_1 == TMP_7600
CONDITION TMP_7601
 revert InvalidLiquidation()()
TMP_7602(None) = SOLIDITY_CALL revert InvalidLiquidation()()
 margin -= ClearingHouseLib.realizeFundingPayment(assets,positions)
TMP_7603(int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.realizeFundingPayment(DynamicArrayLib.DynamicArray,Position[]), arguments:['assets_1', 'positions_1'] 
margin_2(int256) = margin_1 (c)- TMP_7603
 clearingHouse.assertLiquidatable(assets,positions,margin,bookType)
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.assertLiquidatable(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256,BookType), arguments:['clearingHouse_1 (-> [])', 'assets_1', 'positions_1', 'margin_2', 'bookType_1'] 
 (assets,positions,margin,positionIdx)
RETURN assets_1,positions_1,margin_2,positionIdx_1
```
#### LiquidatorPanel._validateDeleveragePair(ClearingHouse,LiquidatorPanel.__DeleverageValidationParams__) [INTERNAL]
```slithir
clearingHouse_1 (-> ['TMP_7502'])(ClearingHouse) := phi(["clearingHouse_1 (-> ['TMP_7502'])"])
params_1(LiquidatorPanel.__DeleverageValidationParams__) := phi(['TMP_7544'])
 params.makerPositionIdx.max(params.takerPositionIdx) == type()(uint256).max
REF_3391(uint256) -> params_1.makerPositionIdx
REF_3393(uint256) -> params_1.takerPositionIdx
TMP_7585(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.max(uint256,uint256), arguments:['REF_3391', 'REF_3393'] 
TMP_7587(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_7588(bool) = TMP_7585 == TMP_7587
CONDITION TMP_7588
 revert InvalidDeleveragePair()()
TMP_7589(None) = SOLIDITY_CALL revert InvalidDeleveragePair()()
 ! clearingHouse.hasBadDebt(params.makerAssets,params.makerPositions,params.makerMargin)
REF_3395(DynamicArrayLib.DynamicArray) -> params_1.makerAssets
REF_3396(Position[]) -> params_1.makerPositions
REF_3397(int256) -> params_1.makerMargin
TMP_7590(bool) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.hasBadDebt(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256), arguments:["clearingHouse_1 (-> ['TMP_7502'])", 'REF_3395', 'REF_3396', 'REF_3397'] 
TMP_7591 = UnaryType.BANG TMP_7590 
CONDITION TMP_7591
 revert InvalidDeleveragePair()()
TMP_7592(None) = SOLIDITY_CALL revert InvalidDeleveragePair()()
 ! clearingHouse.isOpenMarginRequirementMet(params.takerAssets,params.takerPositions,params.takerMargin)
REF_3399(DynamicArrayLib.DynamicArray) -> params_1.takerAssets
REF_3400(Position[]) -> params_1.takerPositions
REF_3401(int256) -> params_1.takerMargin
TMP_7593(bool) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.isOpenMarginRequirementMet(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256), arguments:["clearingHouse_1 (-> ['TMP_7502'])", 'REF_3399', 'REF_3400', 'REF_3401'] 
TMP_7594 = UnaryType.BANG TMP_7593 
CONDITION TMP_7594
 revert InvalidDeleveragePair()()
TMP_7595(None) = SOLIDITY_CALL revert InvalidDeleveragePair()()
 params.makerPositions[params.makerPositionIdx].isLong == params.takerPositions[params.takerPositionIdx].isLong
REF_3402(Position[]) -> params_1.makerPositions
REF_3403(uint256) -> params_1.makerPositionIdx
REF_3404(Position) -> REF_3402[REF_3403]
REF_3405(bool) -> REF_3404.isLong
REF_3406(Position[]) -> params_1.takerPositions
REF_3407(uint256) -> params_1.takerPositionIdx
REF_3408(Position) -> REF_3406[REF_3407]
REF_3409(bool) -> REF_3408.isLong
TMP_7596(bool) = REF_3405 == REF_3409
CONDITION TMP_7596
 revert InvalidDeleveragePair()()
TMP_7597(None) = SOLIDITY_CALL revert InvalidDeleveragePair()()
```
#### LiquidatorPanel.backstopLiquidate(bytes32,address,uint256) [EXTERNAL]
```slithir
 clearingHouse = StorageLib.loadClearingHouse()
TMP_7479(ClearingHouse) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadClearingHouse(), arguments:[] 
clearingHouse_1 (-> ['TMP_7479'])(ClearingHouse) := TMP_7479(ClearingHouse)
 cache = _liquidate({clearingHouse:clearingHouse,asset:asset,account:account,subaccount:subaccount,bookType:BookType.BACKSTOP})
REF_3104(BookType) -> BookType.BACKSTOP
TMP_7480(LiquidatorPanel.__InternalLiquidateCache__) = INTERNAL_CALL, LiquidatorPanel._liquidate(ClearingHouse,bytes32,address,uint256,BookType)(clearingHouse_1 (-> ['TMP_7479']),asset_1,account_1,subaccount_1,REF_3104)
cache_1(LiquidatorPanel.__InternalLiquidateCache__) := TMP_7480(LiquidatorPanel.__InternalLiquidateCache__)
 proratedMargin = cache.maintenanceOrProratedMargin.toInt256()
REF_3105(uint256) -> cache_1.maintenanceOrProratedMargin
TMP_7481(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_3105'] 
proratedMargin_1(int256) := TMP_7481(int256)
 cache.margin -= proratedMargin
REF_3107(int256) -> cache_1.margin
cache_2(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_1'])
REF_3107(-> cache_2) = REF_3107 (c)- proratedMargin_1
 proratedMargin += cache.positionResult.rpnl
REF_3108(PositionUpdateResult) -> cache_2.positionResult
REF_3109(int256) -> REF_3108.rpnl
proratedMargin_2(int256) = proratedMargin_1 (c)+ REF_3109
 proratedMargin < 0
TMP_7482(bool) = proratedMargin_2 < 0
CONDITION TMP_7482
 cache.margin += proratedMargin
REF_3110(int256) -> cache_2.margin
cache_3(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_2'])
REF_3110(-> cache_3) = REF_3110 (c)+ proratedMargin_2
 delete proratedMargin
proratedMargin_3 = delete proratedMargin_2 
cache_4(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_3', 'cache_2'])
proratedMargin_4(int256) := phi(['proratedMargin_2', 'proratedMargin_3'])
 fee = _settleBackstopLiquidation(clearingHouse,asset,proratedMargin.toUint256()).toInt256()
TMP_7483(uint256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toUint256(int256), arguments:['proratedMargin_4'] 
TMP_7484(uint256) = INTERNAL_CALL, LiquidatorPanel._settleBackstopLiquidation(ClearingHouse,bytes32,uint256)(clearingHouse_1 (-> ['TMP_7479']),asset_1,TMP_7483)
TMP_7485(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['TMP_7484'] 
fee_1(int256) := TMP_7485(int256)
 cache.margin < 0 && cache.positions.length == 1
REF_3113(int256) -> cache_4.margin
TMP_7486(bool) = REF_3113 < 0
REF_3114(Position[]) -> cache_4.positions
REF_3115 -> LENGTH REF_3114
TMP_7487(bool) = REF_3115 == 1
TMP_7488(bool) = TMP_7486 && TMP_7487
CONDITION TMP_7488
 fee += cache.margin
REF_3116(int256) -> cache_4.margin
fee_2(int256) = fee_1 (c)+ REF_3116
 delete cache.margin
REF_3117(int256) -> cache_4.margin
cache_5 = delete REF_3117 
cache_6(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_5', 'cache_2'])
fee_3(int256) := phi(['fee_2', 'fee_1'])
 _emitLiquidationEvent({asset:asset,account:account,subaccount:subaccount,side:cache.side,quoteTraded:cache.fillResult.quoteTraded,baseTraded:cache.fillResult.baseTraded,rpnl:cache.positionResult.rpnl,margin:cache.margin,fee:fee,liquidationType:LiquidationType.BACKSTOP_LIQUIDATEE})
REF_3118(Side) -> cache_6.side
REF_3119(PlaceOrderResult) -> cache_6.fillResult
REF_3120(uint256) -> REF_3119.quoteTraded
REF_3121(PlaceOrderResult) -> cache_6.fillResult
REF_3122(uint256) -> REF_3121.baseTraded
REF_3123(PositionUpdateResult) -> cache_6.positionResult
REF_3124(int256) -> REF_3123.rpnl
REF_3125(int256) -> cache_6.margin
REF_3126(LiquidatorPanel.LiquidationType) -> LiquidationType.BACKSTOP_LIQUIDATEE
INTERNAL_CALL, LiquidatorPanel._emitLiquidationEvent(bytes32,address,uint256,Side,uint256,uint256,int256,int256,int256,LiquidatorPanel.LiquidationType)(asset_1,account_1,subaccount_1,REF_3118,REF_3120,REF_3122,REF_3124,REF_3125,fee_3,REF_3126)
 fee > 0
TMP_7490(bool) = fee_3 > 0
CONDITION TMP_7490
 StorageLib.loadInsuranceFund().pay(fee.abs())
TMP_7491(InsuranceFund) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadInsuranceFund(), arguments:[] 
TMP_7492(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['fee_3'] 
LIBRARY_CALL, dest:InsuranceFundLib, function:InsuranceFundLib.pay(InsuranceFund,uint256), arguments:['TMP_7491', 'TMP_7492'] 
 StorageLib.loadInsuranceFund().claim(fee.abs())
TMP_7494(InsuranceFund) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadInsuranceFund(), arguments:[] 
TMP_7495(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['fee_3'] 
LIBRARY_CALL, dest:InsuranceFundLib, function:InsuranceFundLib.claim(InsuranceFund,uint256), arguments:['TMP_7494', 'TMP_7495'] 
 StorageLib.loadCollateralManager().settleFill(account,subaccount,cache.margin,0)
TMP_7497(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
REF_3135(int256) -> cache_6.margin
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.settleFill(CollateralManager,address,uint256,int256,int256), arguments:['TMP_7497', 'account_1', 'subaccount_1', 'REF_3135', '0'] 
 clearingHouse.updateAccount({account:account,subaccount:subaccount,assets:cache.assets,positions:cache.positions,tradedAsset:asset,positionIdx:cache.positionIdx,oiDelta:cache.positionResult.oiDelta,sideClose:cache.positionResult.sideClose})
REF_3137(DynamicArrayLib.DynamicArray) -> cache_6.assets
REF_3138(Position[]) -> cache_6.positions
REF_3139(uint256) -> cache_6.positionIdx
REF_3140(PositionUpdateResult) -> cache_6.positionResult
REF_3141(OIDelta) -> REF_3140.oiDelta
REF_3142(PositionUpdateResult) -> cache_6.positionResult
REF_3143(bool) -> REF_3142.sideClose
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.updateAccount(ClearingHouse,address,uint256,DynamicArrayLib.DynamicArray,Position[],bytes32,uint256,OIDelta,bool), arguments:["clearingHouse_1 (-> ['TMP_7479'])", 'account_1', 'subaccount_1', 'REF_3137', 'REF_3138', 'asset_1', 'REF_3139', 'REF_3141', 'REF_3143'] 
 onlyBackstopLiquidator()
MODIFIER_CALL, LiquidatorPanel.onlyBackstopLiquidator()()
 onlyActiveProtocol()
MODIFIER_CALL, LiquidatorPanel.onlyActiveProtocol()()
```
#### LiquidatorPanel.deleverage(bytes32,DeleveragePair[]) [EXTERNAL]
```slithir
 clearingHouse = StorageLib.loadClearingHouse()
TMP_7502(ClearingHouse) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadClearingHouse(), arguments:[] 
clearingHouse_1 (-> ['TMP_7502'])(ClearingHouse) := TMP_7502(ClearingHouse)
 i < pairs.length
i_1(uint256) := phi(['i_0', 'i_2'])
REF_3145 -> LENGTH pairs_1
TMP_7503(bool) = i_1 < REF_3145
CONDITION TMP_7503
 _deleveragePair(clearingHouse,asset,pairs[i])
REF_3146(DeleveragePair) -> pairs_1[i_1]
INTERNAL_CALL, LiquidatorPanel._deleveragePair(ClearingHouse,bytes32,DeleveragePair)(clearingHouse_1 (-> ['TMP_7502']),asset_1,REF_3146)
 ++ i
i_2(uint256) = i_1 (c)+ 1
 onlyLiquidator()
MODIFIER_CALL, LiquidatorPanel.onlyLiquidator()()
 onlyActiveProtocol()
MODIFIER_CALL, LiquidatorPanel.onlyActiveProtocol()()
```
#### LiquidatorPanel.delistClose(bytes32,Account[]) [EXTERNAL]
```slithir
 clearingHouse = StorageLib.loadClearingHouse()
TMP_7507(ClearingHouse) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadClearingHouse(), arguments:[] 
clearingHouse_1 (-> ['TMP_7507'])(ClearingHouse) := TMP_7507(ClearingHouse)
 market = clearingHouse.market[asset]
REF_3148(mapping(bytes32 => Market)) -> clearingHouse_1 (-> ['TMP_7507']).market
REF_3149(Market) -> REF_3148[asset_1]
market_1 (-> ['clearingHouse'])(Market) := REF_3149(Market)
 StorageLib.loadMarketSettings(asset).status != Status.DELISTED
TMP_7508(MarketSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarketSettings(bytes32), arguments:['asset_1'] 
REF_3151(Status) -> TMP_7508.status
REF_3152(Status) -> Status.DELISTED
TMP_7509(bool) = REF_3151 != REF_3152
CONDITION TMP_7509
 revert MarketNotDelisted()()
TMP_7510(None) = SOLIDITY_CALL revert MarketNotDelisted()()
 i < accounts.length
i_1(uint256) := phi(['i_0', 'i_2'])
REF_3153 -> LENGTH accounts_1
TMP_7511(bool) = i_1 < REF_3153
CONDITION TMP_7511
 _delistClose(clearingHouse,market,asset,accounts[i])
REF_3154(Account) -> accounts_1[i_1]
INTERNAL_CALL, LiquidatorPanel._delistClose(ClearingHouse,Market,bytes32,Account)(clearingHouse_1 (-> ['TMP_7507']),market_1 (-> ['clearingHouse']),asset_1,REF_3154)
 ++ i
i_2(uint256) = i_1 (c)+ 1
 onlyLiquidator()
MODIFIER_CALL, LiquidatorPanel.onlyLiquidator()()
 onlyActiveProtocol()
MODIFIER_CALL, LiquidatorPanel.onlyActiveProtocol()()
```
#### LiquidatorPanel.liquidate(bytes32,address,uint256) [EXTERNAL]
```slithir
 clearingHouse = StorageLib.loadClearingHouse()
TMP_7451(ClearingHouse) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadClearingHouse(), arguments:[] 
clearingHouse_1 (-> ['TMP_7451'])(ClearingHouse) := TMP_7451(ClearingHouse)
 cache = _liquidate({clearingHouse:clearingHouse,asset:asset,account:account,subaccount:subaccount,bookType:BookType.STANDARD})
REF_3037(BookType) -> BookType.STANDARD
TMP_7452(LiquidatorPanel.__InternalLiquidateCache__) = INTERNAL_CALL, LiquidatorPanel._liquidate(ClearingHouse,bytes32,address,uint256,BookType)(clearingHouse_1 (-> ['TMP_7451']),asset_1,account_1,subaccount_1,REF_3037)
cache_1(LiquidatorPanel.__InternalLiquidateCache__) := TMP_7452(LiquidatorPanel.__InternalLiquidateCache__)
 fee = StorageLib.loadMarketSettings(asset).liquidationFeeRate.fullMulDiv(cache.fillResult.quoteTraded,1e18).toInt256()
TMP_7453(MarketSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarketSettings(bytes32), arguments:['asset_1'] 
REF_3039(uint256) -> TMP_7453.liquidationFeeRate
REF_3041(PlaceOrderResult) -> cache_1.fillResult
REF_3042(uint256) -> REF_3041.quoteTraded
TMP_7454(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_3039', 'REF_3042', '1000000000000000000'] 
TMP_7455(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['TMP_7454'] 
fee_1(int256) := TMP_7455(int256)
 cache.margin += cache.positionResult.rpnl - fee
REF_3044(int256) -> cache_1.margin
REF_3045(PositionUpdateResult) -> cache_1.positionResult
REF_3046(int256) -> REF_3045.rpnl
TMP_7456(int256) = REF_3046 (c)- fee_1
cache_2(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_1'])
REF_3044(-> cache_2) = REF_3044 (c)+ TMP_7456
 (cache.margin,cache.positionResult.marginDelta) = clearingHouse.rebalanceClose({assets:cache.assets,positions:cache.positions,margin:cache.margin,marginDelta:cache.positionResult.marginDelta})
REF_3047(int256) -> cache_2.margin
REF_3048(PositionUpdateResult) -> cache_2.positionResult
REF_3049(int256) -> REF_3048.marginDelta
REF_3051(DynamicArrayLib.DynamicArray) -> cache_2.assets
REF_3052(Position[]) -> cache_2.positions
REF_3053(int256) -> cache_2.margin
REF_3054(PositionUpdateResult) -> cache_2.positionResult
REF_3055(int256) -> REF_3054.marginDelta
TUPLE_60(int256,int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.rebalanceClose(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256,int256), arguments:["clearingHouse_1 (-> ['TMP_7451'])", 'REF_3051', 'REF_3052', 'REF_3053', 'REF_3055'] 
REF_3047(int256)= UNPACK TUPLE_60 index: 0 
REF_3049(int256)= UNPACK TUPLE_60 index: 1 
 fullClose = cache.positions[cache.positionIdx].amount == 0 && cache.positions.length == 1
REF_3056(Position[]) -> cache_2.positions
REF_3057(uint256) -> cache_2.positionIdx
REF_3058(Position) -> REF_3056[REF_3057]
REF_3059(uint256) -> REF_3058.amount
TMP_7457(bool) = REF_3059 == 0
REF_3060(Position[]) -> cache_2.positions
REF_3061 -> LENGTH REF_3060
TMP_7458(bool) = REF_3061 == 1
TMP_7459(bool) = TMP_7457 && TMP_7458
fullClose_1(bool) := TMP_7459(bool)
 fullClose && cache.positionResult.marginDelta < 0
REF_3062(PositionUpdateResult) -> cache_2.positionResult
REF_3063(int256) -> REF_3062.marginDelta
TMP_7460(bool) = REF_3063 < 0
TMP_7461(bool) = fullClose_1 && TMP_7460
CONDITION TMP_7461
 cache.positionResult.marginDelta.abs() < cache.maintenanceOrProratedMargin
REF_3064(PositionUpdateResult) -> cache_2.positionResult
REF_3065(int256) -> REF_3064.marginDelta
TMP_7462(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['REF_3065'] 
REF_3067(uint256) -> cache_2.maintenanceOrProratedMargin
TMP_7463(bool) = TMP_7462 < REF_3067
CONDITION TMP_7463
 fee -= cache.positionResult.marginDelta
REF_3068(PositionUpdateResult) -> cache_2.positionResult
REF_3069(int256) -> REF_3068.marginDelta
fee_4(int256) = fee_1 (c)- REF_3069
 delete cache.positionResult.marginDelta
REF_3070(PositionUpdateResult) -> cache_2.positionResult
REF_3071(int256) -> REF_3070.marginDelta
REF_3070 = delete REF_3071 
cache_5(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_2', 'cache_2'])
fee_5(int256) := phi(['fee_1', 'fee_4'])
 fullClose && cache.margin < 0
REF_3072(int256) -> cache_2.margin
TMP_7464(bool) = REF_3072 < 0
TMP_7465(bool) = fullClose_1 && TMP_7464
CONDITION TMP_7465
 fee += cache.margin
REF_3073(int256) -> cache_2.margin
fee_2(int256) = fee_1 (c)+ REF_3073
 delete cache.margin
REF_3074(int256) -> cache_2.margin
cache_3 = delete REF_3074 
cache_4(LiquidatorPanel.__InternalLiquidateCache__) := phi(['cache_3', 'cache_2'])
fee_3(int256) := phi(['fee_1', 'fee_2'])
 _emitLiquidationEvent({asset:asset,account:account,subaccount:subaccount,side:cache.side,quoteTraded:cache.fillResult.quoteTraded,baseTraded:cache.fillResult.baseTraded,rpnl:cache.positionResult.rpnl,margin:cache.margin,fee:fee,liquidationType:LiquidationType.LIQUIDATEE})
REF_3075(Side) -> cache_5.side
REF_3076(PlaceOrderResult) -> cache_5.fillResult
REF_3077(uint256) -> REF_3076.quoteTraded
REF_3078(PlaceOrderResult) -> cache_5.fillResult
REF_3079(uint256) -> REF_3078.baseTraded
REF_3080(PositionUpdateResult) -> cache_5.positionResult
REF_3081(int256) -> REF_3080.rpnl
REF_3082(int256) -> cache_5.margin
REF_3083(LiquidatorPanel.LiquidationType) -> LiquidationType.LIQUIDATEE
INTERNAL_CALL, LiquidatorPanel._emitLiquidationEvent(bytes32,address,uint256,Side,uint256,uint256,int256,int256,int256,LiquidatorPanel.LiquidationType)(asset_1,account_1,subaccount_1,REF_3075,REF_3077,REF_3079,REF_3081,REF_3082,fee_5,REF_3083)
 fee > 0
TMP_7467(bool) = fee_5 > 0
CONDITION TMP_7467
 StorageLib.loadInsuranceFund().pay(fee.abs())
TMP_7468(InsuranceFund) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadInsuranceFund(), arguments:[] 
TMP_7469(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['fee_5'] 
LIBRARY_CALL, dest:InsuranceFundLib, function:InsuranceFundLib.pay(InsuranceFund,uint256), arguments:['TMP_7468', 'TMP_7469'] 
 StorageLib.loadInsuranceFund().claim(fee.abs())
TMP_7471(InsuranceFund) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadInsuranceFund(), arguments:[] 
TMP_7472(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['fee_5'] 
LIBRARY_CALL, dest:InsuranceFundLib, function:InsuranceFundLib.claim(InsuranceFund,uint256), arguments:['TMP_7471', 'TMP_7472'] 
 StorageLib.loadCollateralManager().settleFill(account,subaccount,cache.margin,cache.positionResult.marginDelta)
TMP_7474(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
REF_3092(int256) -> cache_5.margin
REF_3093(PositionUpdateResult) -> cache_5.positionResult
REF_3094(int256) -> REF_3093.marginDelta
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.settleFill(CollateralManager,address,uint256,int256,int256), arguments:['TMP_7474', 'account_1', 'subaccount_1', 'REF_3092', 'REF_3094'] 
 clearingHouse.updateAccount({account:account,subaccount:subaccount,assets:cache.assets,positions:cache.positions,tradedAsset:asset,positionIdx:cache.positionIdx,oiDelta:cache.positionResult.oiDelta,sideClose:cache.positionResult.sideClose})
REF_3096(DynamicArrayLib.DynamicArray) -> cache_5.assets
REF_3097(Position[]) -> cache_5.positions
REF_3098(uint256) -> cache_5.positionIdx
REF_3099(PositionUpdateResult) -> cache_5.positionResult
REF_3100(OIDelta) -> REF_3099.oiDelta
REF_3101(PositionUpdateResult) -> cache_5.positionResult
REF_3102(bool) -> REF_3101.sideClose
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.updateAccount(ClearingHouse,address,uint256,DynamicArrayLib.DynamicArray,Position[],bytes32,uint256,OIDelta,bool), arguments:["clearingHouse_1 (-> ['TMP_7451'])", 'account_1', 'subaccount_1', 'REF_3096', 'REF_3097', 'asset_1', 'REF_3098', 'REF_3100', 'REF_3102'] 
 onlyLiquidator()
MODIFIER_CALL, LiquidatorPanel.onlyLiquidator()()
 onlyActiveProtocol()
MODIFIER_CALL, LiquidatorPanel.onlyActiveProtocol()()
```
#### ClearingHouseLib.rebalanceClose(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256,int256) [INTERNAL]
```slithir
 (intendedMargin,upnl) = _getIntendedMarginAndUpnl(self,assets,positions)
TUPLE_95(uint256,int256) = INTERNAL_CALL, ClearingHouseLib._getIntendedMarginAndUpnl(ClearingHouse,DynamicArrayLib.DynamicArray,Position[])(self_1 (-> []),assets_1,positions_1)
intendedMargin_1(uint256)= UNPACK TUPLE_95 index: 0 
upnl_1(int256)= UNPACK TUPLE_95 index: 1 
 intendedMargin == 0
TMP_8994(bool) = intendedMargin_1 == 0
CONDITION TMP_8994
 margin < 0
TMP_8995(bool) = margin_1 < 0
CONDITION TMP_8995
 (margin,0)
RETURN margin_1,0
 (0,- margin)
TMP_8996(int256) = 0 (c)- margin_1
RETURN 0,TMP_8996
 equity = margin + upnl
TMP_8997(int256) = margin_1 (c)+ upnl_1
equity_1(int256) := TMP_8997(int256)
 finalMarginDelta = marginDelta.max((intendedMargin.toInt256() - equity).min(0))
TMP_8998(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['intendedMargin_1'] 
TMP_8999(int256) = TMP_8998 (c)- equity_1
TMP_9000(int256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.min(int256,int256), arguments:['TMP_8999', '0'] 
TMP_9001(int256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.max(int256,int256), arguments:['marginDelta_1', 'TMP_9000'] 
finalMarginDelta_1(int256) := TMP_9001(int256)
 finalMargin = margin + finalMarginDelta
TMP_9002(int256) = margin_1 (c)+ finalMarginDelta_1
finalMargin_1(int256) := TMP_9002(int256)
 (finalMargin,finalMarginDelta)
RETURN finalMargin_1,finalMarginDelta_1
```
#### ClearingHouseLib.updateAccount(ClearingHouse,address,uint256,DynamicArrayLib.DynamicArray,Position[],bytes32,uint256,OIDelta,bool) [INTERNAL]
```slithir
 self.setPositions(tradedAsset,account,subaccount,assets,positions)
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.setPositions(ClearingHouse,bytes32,address,uint256,DynamicArrayLib.DynamicArray,Position[]), arguments:['self_1 (-> [])', 'tradedAsset_1', 'account_1', 'subaccount_1', 'assets_1', 'positions_1'] 
 positions[positionIdx].amount == 0
REF_4577(Position) -> positions_1[positionIdx_1]
REF_4578(uint256) -> REF_4577.amount
TMP_9004(bool) = REF_4578 == 0
CONDITION TMP_9004
 _movePop(assets,tradedAsset)
INTERNAL_CALL, ClearingHouseLib._movePop(DynamicArrayLib.DynamicArray,bytes32)(assets_1,tradedAsset_1)
 self.setAssets(account,subaccount,assets.length(),tradedAsset)
TMP_9006(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.length(DynamicArrayLib.DynamicArray), arguments:['assets_1'] 
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.setAssets(ClearingHouse,address,uint256,uint256,bytes32), arguments:['self_1 (-> [])', 'account_1', 'subaccount_1', 'TMP_9006', 'tradedAsset_1'] 
 MarketLib.updateOI(tradedAsset,oiDelta)
LIBRARY_CALL, dest:MarketLib, function:MarketLib.updateOI(bytes32,OIDelta), arguments:['tradedAsset_1', 'oiDelta_1'] 
 sideClose
CONDITION sideClose_1
 self.market[tradedAsset].cancelCloseOrders(account,subaccount)
REF_4582(mapping(bytes32 => Market)) -> self_1 (-> []).market
REF_4583(Market) -> REF_4582[tradedAsset_1]
LIBRARY_CALL, dest:MarketLib, function:MarketLib.cancelCloseOrders(Market,address,uint256), arguments:['REF_4583', 'account_1', 'subaccount_1']
```
#### CollateralManagerLib.settleFill(CollateralManager,address,uint256,int256,int256) [INTERNAL]
```slithir
 self.margin[account][subaccount] = margin
REF_4828(mapping(address => mapping(uint256 => int256))) -> self_1 (-> []).margin
REF_4829(mapping(uint256 => int256)) -> REF_4828[account_1]
REF_4830(int256) -> REF_4829[subaccount_1]
self_2 (-> [])(CollateralManager) := phi(['self_1 (-> [])'])
REF_4830(int256) (->self_2 (-> [])) := margin_1(int256)
 self.handleCollateralDelta(account,marginDelta)
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.handleCollateralDelta(CollateralManager,address,int256), arguments:['self_2 (-> [])', 'account_1', 'marginDelta_1']
```
#### InsuranceFundLib.claim(InsuranceFund,uint256) [INTERNAL]
```slithir
 amount != 0
TMP_9257(bool) = amount_1 != 0
CONDITION TMP_9257
 self.balance < amount
TMP_9258(bool) = self.balance < amount_1
CONDITION TMP_9258
 revert InsufficientInsuranceFundBalance()()
TMP_9259(None) = SOLIDITY_CALL revert InsufficientInsuranceFundBalance()()
 currentBalance_claim_asm_0 = sload(uint256)(self)
TMP_9260(uint256) = SOLIDITY_CALL sload(uint256)(self_1 (-> []))
currentBalance_claim_asm_0_1(uint256) := TMP_9260(uint256)
 newBalance_claim_asm_0 = currentBalance_claim_asm_0 - amount
TMP_9261(uint256) = currentBalance_claim_asm_0_1 - amount_1
newBalance_claim_asm_0_1(uint256) := TMP_9261(uint256)
 sstore(uint256,uint256)(self,newBalance_claim_asm_0)
TMP_9262(None) = SOLIDITY_CALL sstore(uint256,uint256)(self_1 (-> []),newBalance_claim_asm_0_1)
```
#### PositionLib.processTrade(Position,Side,uint256,uint256) [INTERNAL]
```slithir
 openLong = side == Side.BUY && (self.isLong || self.amount == 0)
REF_5313(Side) -> Side.BUY
TMP_9536(bool) = side_1 == REF_5313
REF_5314(bool) -> self_1.isLong
REF_5315(uint256) -> self_1.amount
TMP_9537(bool) = REF_5315 == 0
TMP_9538(bool) = REF_5314 || TMP_9537
TMP_9539(bool) = TMP_9536 && TMP_9538
openLong_1(bool) := TMP_9539(bool)
 openShort = side == Side.SELL && (! self.isLong || self.amount == 0)
REF_5316(Side) -> Side.SELL
TMP_9540(bool) = side_1 == REF_5316
REF_5317(bool) -> self_1.isLong
TMP_9541 = UnaryType.BANG REF_5317 
REF_5318(uint256) -> self_1.amount
TMP_9542(bool) = REF_5318 == 0
TMP_9543(bool) = TMP_9541 || TMP_9542
TMP_9544(bool) = TMP_9540 && TMP_9543
openShort_1(bool) := TMP_9544(bool)
 openLong || openShort
TMP_9545(bool) = openLong_1 || openShort_1
CONDITION TMP_9545
 result.marginDelta = _open(self,side,quoteTraded,baseTraded)
REF_5319(int256) -> result_0.marginDelta
TMP_9546(int256) = INTERNAL_CALL, PositionLib._open(Position,Side,uint256,uint256)(self_1,side_1,quoteTraded_1,baseTraded_1)
result_2(PositionUpdateResult) := phi(['result_0'])
REF_5319(int256) (->result_2) := TMP_9546(int256)
 side == Side.BUY
REF_5320(Side) -> Side.BUY
TMP_9547(bool) = side_1 == REF_5320
CONDITION TMP_9547
 result.oiDelta.long += baseTraded.toInt256()
REF_5321(OIDelta) -> result_2.oiDelta
REF_5322(int256) -> REF_5321.long
TMP_9548(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['baseTraded_1'] 
result_3(PositionUpdateResult) := phi(['result_2'])
REF_5322(-> result_3) = REF_5322 (c)+ TMP_9548
 result.oiDelta.short += baseTraded.toInt256()
REF_5324(OIDelta) -> result_2.oiDelta
REF_5325(int256) -> REF_5324.short
TMP_9549(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['baseTraded_1'] 
result_4(PositionUpdateResult) := phi(['result_2'])
REF_5325(-> result_4) = REF_5325 (c)+ TMP_9549
result_5(PositionUpdateResult) := phi(['result_3', 'result_4'])
 result = _close(self,side,quoteTraded,baseTraded)
TMP_9550(PositionUpdateResult) = INTERNAL_CALL, PositionLib._close(Position,Side,uint256,uint256)(self_1,side_1,quoteTraded_1,baseTraded_1)
result_1(PositionUpdateResult) := TMP_9550(PositionUpdateResult)
result_6(PositionUpdateResult) := phi(['result_1', 'result_2'])
 result
RETURN result_6
```
#### StorageLib.loadCollateralManager() [INTERNAL]
```slithir
COLLATERAL_MANAGER_SLOT_1(bytes32) := phi(['COLLATERAL_MANAGER_SLOT_0'])
 slot = COLLATERAL_MANAGER_SLOT
slot_1(bytes32) := COLLATERAL_MANAGER_SLOT_1(bytes32)
 collateralManager = slot
collateralManager_1 (-> ['slot'])(CollateralManager) := slot_1(bytes32)
 collateralManager
RETURN collateralManager_1 (-> ['slot'])
```
#### StorageLib.loadInsuranceFund() [INTERNAL]
```slithir
INSURANCE_FUND_SLOT_1(bytes32) := phi(['INSURANCE_FUND_SLOT_0'])
 slot = INSURANCE_FUND_SLOT
slot_1(bytes32) := INSURANCE_FUND_SLOT_1(bytes32)
 insuranceFund = slot
insuranceFund_1 (-> ['slot'])(InsuranceFund) := slot_1(bytes32)
 insuranceFund
RETURN insuranceFund_1 (-> ['slot'])
```
#### FixedPointMathLib.abs(int256) [INTERNAL]
```slithir
 z = (uint256(x) + uint256(x >> 255)) ^ uint256(x >> 255)
TMP_13884 = CONVERT x_1 to uint256
TMP_13885(int256) = x_1 >> 255
TMP_13886 = CONVERT TMP_13885 to uint256
TMP_13887(uint256) = TMP_13884 + TMP_13886
TMP_13888(int256) = x_1 >> 255
TMP_13889 = CONVERT TMP_13888 to uint256
TMP_13890(uint256) = TMP_13887 ^ TMP_13889
z_1(uint256) := TMP_13890(uint256)
 z
RETURN z_1
```
#### SafeCastLib.toInt256(uint256) [INTERNAL]
```slithir
 int256(x) >= 0
TMP_15164 = CONVERT x_1 to int256
TMP_15165(bool) = TMP_15164 >= 0
CONDITION TMP_15165
 int256(x)
TMP_15166 = CONVERT x_1 to int256
RETURN TMP_15166
 _revertOverflow()
INTERNAL_CALL, SafeCastLib._revertOverflow()()
```
#### ClearingHouseLib.getAccountAndMargin(ClearingHouse,address,uint256) [INTERNAL]
```slithir
 (assets,positions) = self.getAccount(account,subaccount)
TUPLE_96(DynamicArrayLib.DynamicArray,Position[]) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.getAccount(ClearingHouse,address,uint256), arguments:['self_1 (-> [])', 'account_1', 'subaccount_1'] 
assets_1(DynamicArrayLib.DynamicArray)= UNPACK TUPLE_96 index: 0 
positions_1(Position[])= UNPACK TUPLE_96 index: 1 
 margin = StorageLib.loadCollateralManager().getMarginBalance(account,subaccount)
TMP_9015(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
TMP_9016(int256) = LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.getMarginBalance(CollateralManager,address,uint256), arguments:['TMP_9015', 'account_1', 'subaccount_1'] 
margin_1(int256) := TMP_9016(int256)
 (assets,positions,margin)
RETURN assets_1,positions_1,margin_1
```
#### ClearingHouseLib.getProratedMargin(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],bytes32,int256) [INTERNAL]
```slithir
 length = assets.length()
TMP_9025(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.length(DynamicArrayLib.DynamicArray), arguments:['assets_1'] 
length_1(uint256) := TMP_9025(uint256)
 i < length
totalNotional_1(uint256) := phi(['totalNotional_0', 'totalNotional_2'])
i_1(uint256) := phi(['i_0', 'i_2'])
TMP_9026(bool) = i_1 < length_1
CONDITION TMP_9026
 notional = self.market[assets.getBytes32(i)].getNotionalValue(positions[i])
REF_4611(mapping(bytes32 => Market)) -> self_1 (-> []).market
TMP_9027(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
REF_4613(Market) -> REF_4611[TMP_9027]
REF_4615(Position) -> positions_1[i_1]
TMP_9028(uint256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getNotionalValue(Market,Position), arguments:['REF_4613', 'REF_4615'] 
notional_1(uint256) := TMP_9028(uint256)
 totalNotional += notional
totalNotional_2(uint256) = totalNotional_1 (c)+ notional_1
 assets.getBytes32(i) == asset
TMP_9029(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
TMP_9030(bool) = TMP_9029 == asset_1
CONDITION TMP_9030
 assetNotional = notional
assetNotional_1(uint256) := notional_1(uint256)
 ++ i
i_2(uint256) = i_1 (c)+ 1
 _prorateMargin(margin,assetNotional,totalNotional)
TMP_9031(int256) = INTERNAL_CALL, ClearingHouseLib._prorateMargin(int256,uint256,uint256)(margin_1,assetNotional_0,totalNotional_1)
RETURN TMP_9031
 proratedMargin
```
#### ClearingHouseLib.realizeFundingPayment(DynamicArrayLib.DynamicArray,Position[]) [INTERNAL]
```slithir
assets_1(DynamicArrayLib.DynamicArray) := phi(['REF_4702', 'REF_4444'])
positions_1(Position[]) := phi(['REF_4703', 'REF_4445'])
 length = assets.length()
TMP_9032(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.length(DynamicArrayLib.DynamicArray), arguments:['assets_1'] 
length_1(uint256) := TMP_9032(uint256)
 i < length
fundingPayment_1(int256) := phi(['fundingPayment_2', 'fundingPayment_0'])
i_1(uint256) := phi(['i_0', 'i_2'])
TMP_9033(bool) = i_1 < length_1
CONDITION TMP_9033
 fundingPayment += MarketLib.realizeFundingPayment(assets.getBytes32(i),positions[i])
TMP_9034(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
REF_4620(Position) -> positions_1[i_1]
TMP_9035(int256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.realizeFundingPayment(bytes32,Position), arguments:['TMP_9034', 'REF_4620'] 
fundingPayment_2(int256) = fundingPayment_1 (c)+ TMP_9035
 ++ i
i_2(uint256) = i_1 (c)+ 1
 fundingPayment
RETURN fundingPayment_1
```
#### DynamicArrayLib.indexOf(DynamicArrayLib.DynamicArray,bytes32) [INTERNAL]
```slithir
 indexOf(a.data,uint256(needle),0)
REF_5767(uint256[]) -> a_1.data
TMP_12426 = CONVERT needle_1 to uint256
TMP_12427(uint256) = INTERNAL_CALL, DynamicArrayLib.indexOf(uint256[],uint256,uint256)(REF_5767,TMP_12426,0)
RETURN TMP_12427
```
#### FixedPointMathLib.fullMulDiv(uint256,uint256,uint256) [INTERNAL]
```slithir
x_1(uint256) := phi(['TMP_13961', 'TMP_13966', 'TMP_13980', 'TMP_13990', 'x_1'])
y_1(uint256) := phi(['y_1', 'TMP_13967', 'TMP_13962', 'TMP_13982', 'TMP_13992'])
d_1(uint256) := phi(['TMP_13984', 'TMP_13994', 'TMP_13968', 'TMP_13963', 'd_1'])
 z = x * y
TMP_13384(uint256) = x_1 * y_1
z_1(uint256) := TMP_13384(uint256)
 1
z_2(uint256) := phi(['z_5', 'z_1'])
CONDITION 1
 ! ! x | z / x == y * d
TMP_13385 = UnaryType.BANG x_1 
TMP_13386(uint256) = z_2 / x_1
TMP_13387(bool) = TMP_13386 == y_1
TMP_13388(uint256) = TMP_13385 | TMP_13387
TMP_13389(uint256) = TMP_13388 * d_1
TMP_13390 = UnaryType.BANG TMP_13389 
CONDITION TMP_13390
d_3(uint256) := phi(['d_2', 'd_1'])
z_4(uint256) := phi(['z_3', 'z_1'])
 mm_fullMulDiv_asm_0 = mulmod(uint256,uint256,uint256)(x,y,~ 0)
TMP_13391 = UnaryType.TILD 0 
TMP_13392(uint256) = SOLIDITY_CALL mulmod(uint256,uint256,uint256)(x_1,y_1,TMP_13391)
mm_fullMulDiv_asm_0_1(uint256) := TMP_13392(uint256)
 p1_fullMulDiv_asm_0 = mm_fullMulDiv_asm_0 - z + mm_fullMulDiv_asm_0 < z
TMP_13393(bool) = mm_fullMulDiv_asm_0_1 < z_2
TMP_13394(uint256) = z_2 + TMP_13393
TMP_13395(uint256) = mm_fullMulDiv_asm_0_1 - TMP_13394
p1_fullMulDiv_asm_0_1(uint256) := TMP_13395(uint256)
 r_fullMulDiv_asm_0 = mulmod(uint256,uint256,uint256)(x,y,d)
TMP_13396(uint256) = SOLIDITY_CALL mulmod(uint256,uint256,uint256)(x_1,y_1,d_1)
r_fullMulDiv_asm_0_1(uint256) := TMP_13396(uint256)
 t_fullMulDiv_asm_0 = d & 0 - d
TMP_13397(uint256) = 0 - d_1
TMP_13398(uint256) = d_1 & TMP_13397
t_fullMulDiv_asm_0_1(uint256) := TMP_13398(uint256)
 ! d > p1_fullMulDiv_asm_0
TMP_13399(bool) = d_1 > p1_fullMulDiv_asm_0_1
TMP_13400 = UnaryType.BANG TMP_13399 
CONDITION TMP_13400
 mstore(uint256,uint256)(0x00,0xae47f702)
TMP_13401(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,2923951874)
 revert(uint256,uint256)(0x1c,0x04)
TMP_13402(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 d = d / t_fullMulDiv_asm_0
TMP_13403(uint256) = d_1 / t_fullMulDiv_asm_0_1
d_2(uint256) := TMP_13403(uint256)
 inv_fullMulDiv_asm_0 = 2 ^ 3 * d
TMP_13404(uint256) = 3 * d_2
TMP_13405(uint256) = 2 ^ TMP_13404
inv_fullMulDiv_asm_0_1(uint256) := TMP_13405(uint256)
 inv_fullMulDiv_asm_0 = inv_fullMulDiv_asm_0 * 2 - d * inv_fullMulDiv_asm_0
TMP_13406(uint256) = d_2 * inv_fullMulDiv_asm_0_1
TMP_13407(uint256) = 2 - TMP_13406
TMP_13408(uint256) = inv_fullMulDiv_asm_0_1 * TMP_13407
inv_fullMulDiv_asm_0_2(uint256) := TMP_13408(uint256)
 inv_fullMulDiv_asm_0 = inv_fullMulDiv_asm_0 * 2 - d * inv_fullMulDiv_asm_0
TMP_13409(uint256) = d_2 * inv_fullMulDiv_asm_0_2
TMP_13410(uint256) = 2 - TMP_13409
TMP_13411(uint256) = inv_fullMulDiv_asm_0_2 * TMP_13410
inv_fullMulDiv_asm_0_3(uint256) := TMP_13411(uint256)
 inv_fullMulDiv_asm_0 = inv_fullMulDiv_asm_0 * 2 - d * inv_fullMulDiv_asm_0
TMP_13412(uint256) = d_2 * inv_fullMulDiv_asm_0_3
TMP_13413(uint256) = 2 - TMP_13412
TMP_13414(uint256) = inv_fullMulDiv_asm_0_3 * TMP_13413
inv_fullMulDiv_asm_0_4(uint256) := TMP_13414(uint256)
 inv_fullMulDiv_asm_0 = inv_fullMulDiv_asm_0 * 2 - d * inv_fullMulDiv_asm_0
TMP_13415(uint256) = d_2 * inv_fullMulDiv_asm_0_4
TMP_13416(uint256) = 2 - TMP_13415
TMP_13417(uint256) = inv_fullMulDiv_asm_0_4 * TMP_13416
inv_fullMulDiv_asm_0_5(uint256) := TMP_13417(uint256)
 inv_fullMulDiv_asm_0 = inv_fullMulDiv_asm_0 * 2 - d * inv_fullMulDiv_asm_0
TMP_13418(uint256) = d_2 * inv_fullMulDiv_asm_0_5
TMP_13419(uint256) = 2 - TMP_13418
TMP_13420(uint256) = inv_fullMulDiv_asm_0_5 * TMP_13419
inv_fullMulDiv_asm_0_6(uint256) := TMP_13420(uint256)
 z = p1_fullMulDiv_asm_0 - r_fullMulDiv_asm_0 > z * 0 - t_fullMulDiv_asm_0 / t_fullMulDiv_asm_0 + 1 | z - r_fullMulDiv_asm_0 / t_fullMulDiv_asm_0 * 2 - d * inv_fullMulDiv_asm_0 * inv_fullMulDiv_asm_0
TMP_13421(bool) = r_fullMulDiv_asm_0_1 > z_2
TMP_13422(uint256) = p1_fullMulDiv_asm_0_1 - TMP_13421
TMP_13423(uint256) = 0 - t_fullMulDiv_asm_0_1
TMP_13424(uint256) = TMP_13423 / t_fullMulDiv_asm_0_1
TMP_13425(uint256) = TMP_13424 + 1
TMP_13426(uint256) = TMP_13422 * TMP_13425
TMP_13427(uint256) = z_2 - r_fullMulDiv_asm_0_1
TMP_13428(uint256) = TMP_13427 / t_fullMulDiv_asm_0_1
TMP_13429(uint256) = TMP_13426 | TMP_13428
TMP_13430(uint256) = d_2 * inv_fullMulDiv_asm_0_6
TMP_13431(uint256) = 2 - TMP_13430
TMP_13432(uint256) = TMP_13431 * inv_fullMulDiv_asm_0_6
TMP_13433(uint256) = TMP_13429 * TMP_13432
z_3(uint256) := TMP_13433(uint256)
 z = z / d
TMP_13434(uint256) = z_4 / d_3
z_5(uint256) := TMP_13434(uint256)
 z
RETURN z_2
```
#### FixedPointMathLib.min(int256,int256) [INTERNAL]
```slithir
 z = x ^ x ^ y * y <' x
TMP_13913(int256) = x_1 ^ y_1
TMP_13915 = CONVERT y_1 to int256
TMP_13916 = CONVERT x_1 to int256
TMP_13917(bool) = TMP_13915 < TMP_13916
TMP_13914 = CONVERT TMP_13917 to uint256
TMP_13918(int256) = TMP_13913 * TMP_13914
TMP_13919(int256) = x_1 ^ TMP_13918
z_1(int256) := TMP_13919(int256)
 z
RETURN z_1
```
#### FeeManagerLib.getTakerFee(FeeManager,address,uint256) [INTERNAL]
```slithir
FEE_SCALING_1(uint256) := phi(['FEE_SCALING_0'])
 amount == 0
TMP_9197(bool) = amount_1 == 0
CONDITION TMP_9197
 0
RETURN 0
 feeRate = self.getTakerFeeRate(account)
TMP_9198(uint16) = LIBRARY_CALL, dest:FeeManagerLib, function:FeeManagerLib.getTakerFeeRate(FeeManager,address), arguments:['self_1 (-> [])', 'account_1'] 
feeRate_1(uint16) := TMP_9198(uint16)
 amount.fullMulDiv(feeRate,FEE_SCALING)
TMP_9199(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['amount_1', 'feeRate_1', 'FEE_SCALING_1'] 
RETURN TMP_9199
```
#### InsuranceFundLib.pay(InsuranceFund,uint256) [INTERNAL]
```slithir
 amount != 0
TMP_9253(bool) = amount_1 != 0
CONDITION TMP_9253
 currentBalance_pay_asm_0 = sload(uint256)(self)
TMP_9254(uint256) = SOLIDITY_CALL sload(uint256)(self_1 (-> []))
currentBalance_pay_asm_0_1(uint256) := TMP_9254(uint256)
 newBalance_pay_asm_0 = currentBalance_pay_asm_0 + amount
TMP_9255(uint256) = currentBalance_pay_asm_0_1 + amount_1
newBalance_pay_asm_0_1(uint256) := TMP_9255(uint256)
 sstore(uint256,uint256)(self,newBalance_pay_asm_0)
TMP_9256(None) = SOLIDITY_CALL sstore(uint256,uint256)(self_1 (-> []),newBalance_pay_asm_0_1)
```
#### StorageLib.loadFeeManager() [INTERNAL]
```slithir
FEE_MANAGER_SLOT_1(bytes32) := phi(['FEE_MANAGER_SLOT_0'])
 slot = FEE_MANAGER_SLOT
slot_1(bytes32) := FEE_MANAGER_SLOT_1(bytes32)
 feeManager = slot
feeManager_1 (-> ['slot'])(FeeManager) := slot_1(bytes32)
 feeManager
RETURN feeManager_1 (-> ['slot'])
```
#### StorageLib.incNonce() [INTERNAL]
```slithir
EVENT_NONCE_SLOT_1(bytes32) := phi(['EVENT_NONCE_SLOT_0'])
 slot = EVENT_NONCE_SLOT
slot_1(bytes32) := EVENT_NONCE_SLOT_1(bytes32)
 n = sload(uint256)(slot) + 1
TMP_9691(uint256) = SOLIDITY_CALL sload(uint256)(slot_1)
TMP_9692(uint256) = TMP_9691 + 1
n_1(uint256) := TMP_9692(uint256)
 sstore(uint256,uint256)(slot,n)
TMP_9693(None) = SOLIDITY_CALL sstore(uint256,uint256)(slot_1,n_1)
 n
RETURN n_1
```
#### SafeCastLib.toUint256(int256) [INTERNAL]
```slithir
 x >= 0
TMP_15168(bool) = x_1 >= 0
CONDITION TMP_15168
 uint256(x)
TMP_15169 = CONVERT x_1 to uint256
RETURN TMP_15169
 _revertOverflow()
INTERNAL_CALL, SafeCastLib._revertOverflow()()
```
#### MarketLib.getMaintenanceMargin(Market,uint256) [INTERNAL]
```slithir
 positionAmount.fullMulDiv(self.markPrice,1e18).fullMulDiv(StorageLib.loadMarketSettings(self.asset).maintenanceMarginRatio,1e18)
REF_5091(uint256) -> self_1 (-> []).markPrice
TMP_9362(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['positionAmount_1', 'REF_5091', '1000000000000000000'] 
REF_5094(bytes32) -> self_1 (-> []).asset
TMP_9363(MarketSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarketSettings(bytes32), arguments:['REF_5094'] 
REF_5095(uint256) -> TMP_9363.maintenanceMarginRatio
TMP_9364(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['TMP_9362', 'REF_5095', '1000000000000000000'] 
RETURN TMP_9364
```
#### MarketLib.liquidate(Market,address,uint256,Side,uint256,BookType) [INTERNAL]
```slithir
 bookType == BookType.STANDARD
REF_4969(BookType) -> BookType.STANDARD
TMP_9291(bool) = bookType_1 == REF_4969
CONDITION TMP_9291
 amount = _getLiquidationAmount(self,amount)
TMP_9292(uint256) = INTERNAL_CALL, MarketLib._getLiquidationAmount(Market,uint256)(self_1 (-> []),amount_1)
amount_2(uint256) := TMP_9292(uint256)
amount_3(uint256) := phi(['amount_2', 'amount_1'])
 result = CLOBLib.placeOrder(account,PlaceOrderArgs({subaccount:subaccount,asset:self.asset,side:side,limitPrice:0,amount:amount,baseDenominated:true,tif:TiF.IOC,expiryTime:0,clientOrderId:0,reduceOnly:true}),bookType)
REF_4971(bytes32) -> self_1 (-> []).asset
REF_4972(TiF) -> TiF.IOC
TMP_9293(PlaceOrderArgs) = new PlaceOrderArgs(subaccount_1,REF_4971,side_1,0,amount_3,True,REF_4972,0,0,True)
TMP_9294(PlaceOrderResult) = LIBRARY_CALL, dest:CLOBLib, function:CLOBLib.placeOrder(address,PlaceOrderArgs,BookType), arguments:['account_1', 'TMP_9293', 'bookType_1'] 
result_1(PlaceOrderResult) := TMP_9294(PlaceOrderResult)
 result
RETURN result_1
```
#### BackstopLiquidatorDataLib.getLiquidatorDataAndClearStorage() [INTERNAL]
```slithir
 liquidators = _getLiquidatorsAndClear()
TMP_8455(address[]) = INTERNAL_CALL, BackstopLiquidatorDataLib._getLiquidatorsAndClear()()
liquidators_1(address[]) = ['TMP_8455(address[])']
 length = liquidators.length
REF_3694 -> LENGTH liquidators_1
length_1(uint256) := REF_3694(uint256)
 liquidatorData = new LiquidatorData[](length)
TMP_8457(LiquidatorData[])  = new LiquidatorData[](length_1)
liquidatorData_1(LiquidatorData[]) = ['TMP_8457(LiquidatorData[])']
 i < length
i_1(uint256) := phi(['i_0', 'i_2'])
TMP_8458(bool) = i_1 < length_1
CONDITION TMP_8458
 volume = _getVolumeAndClear(liquidators[i])
REF_3695(address) -> liquidators_1[i_1]
TMP_8459(uint256) = INTERNAL_CALL, BackstopLiquidatorDataLib._getVolumeAndClear(address)(REF_3695)
volume_1(uint256) := TMP_8459(uint256)
 liquidatorData[i] = LiquidatorData({liquidator:liquidators[i],volume:volume})
REF_3696(LiquidatorData) -> liquidatorData_1[i_1]
REF_3697(address) -> liquidators_1[i_1]
TMP_8460(LiquidatorData) = new LiquidatorData(REF_3697,volume_1)
liquidatorData_2(LiquidatorData[]) := phi(['liquidatorData_1'])
REF_3696(LiquidatorData) (->liquidatorData_2) := TMP_8460(LiquidatorData)
 i ++
TMP_8461(uint256) := i_1(uint256)
i_2(uint256) = i_1 (c)+ 1
 liquidatorData
RETURN liquidatorData_1
```
#### CollateralManagerLib.creditAccount(CollateralManager,address,uint256) [INTERNAL]
```slithir
 self.freeCollateral[account] += amount
REF_4832(mapping(address => uint256)) -> self_1 (-> []).freeCollateral
REF_4833(uint256) -> REF_4832[account_1]
self_2 (-> [])(CollateralManager) := phi(['self_1 (-> [])'])
REF_4833(-> self_2 (-> [])) = REF_4833 (c)+ amount_1
```
#### StorageLib.loadMarketSettings(bytes32) [INTERNAL]
```slithir
MARKET_SETTINGS_SLOT_1(bytes32) := phi(['MARKET_SETTINGS_SLOT_0'])
 slot = keccak256(bytes)(abi.encode(asset,MARKET_SETTINGS_SLOT))
TMP_9643(bytes) = SOLIDITY_CALL abi.encode()(asset_1,MARKET_SETTINGS_SLOT_1)
TMP_9644(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_9643)
slot_1(bytes32) := TMP_9644(bytes32)
 marketSettings = slot
marketSettings_1 (-> ['slot'])(MarketSettings) := slot_1(bytes32)
 marketSettings
RETURN marketSettings_1 (-> ['slot'])
```
#### ClearingHouseLib.assertLiquidatable(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256,BookType) [INTERNAL]
```slithir
 ! self.isLiquidatable(assets,positions,margin,bookType)
TMP_9149(bool) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.isLiquidatable(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256,BookType), arguments:['self_1 (-> [])', 'assets_1', 'positions_1', 'margin_1', 'bookType_1'] 
TMP_9150 = UnaryType.BANG TMP_9149 
CONDITION TMP_9150
 revert NotLiquidatable()()
TMP_9151(None) = SOLIDITY_CALL revert NotLiquidatable()()
```
#### ClearingHouseLib.hasBadDebt(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256) [INTERNAL]
```slithir
 i < positions.length
upnl_1(int256) := phi(['upnl_0', 'upnl_2'])
i_1(uint256) := phi(['i_2', 'i_0'])
REF_4668 -> LENGTH positions_1
TMP_9063(bool) = i_1 < REF_4668
CONDITION TMP_9063
 upnl += self.market[assets.getBytes32(i)].getUpnl(positions[i])
REF_4669(mapping(bytes32 => Market)) -> self_1 (-> []).market
TMP_9064(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
REF_4671(Market) -> REF_4669[TMP_9064]
REF_4673(Position) -> positions_1[i_1]
TMP_9065(int256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getUpnl(Market,Position), arguments:['REF_4671', 'REF_4673'] 
upnl_2(int256) = upnl_1 (c)+ TMP_9065
 ++ i
i_2(uint256) = i_1 (c)+ 1
 margin + upnl < 0
TMP_9066(int256) = margin_1 (c)+ upnl_1
TMP_9067(bool) = TMP_9066 < 0
RETURN TMP_9067
 badDebt
```
#### ClearingHouseLib.isOpenMarginRequirementMet(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256) [INTERNAL]
```slithir
 i < positions.length
minOpenMargin_1(uint256) := phi(['minOpenMargin_0', 'minOpenMargin_2'])
upnl_1(int256) := phi(['upnl_2', 'upnl_0'])
i_1(uint256) := phi(['i_0', 'i_2'])
REF_4655 -> LENGTH positions_1
TMP_9055(bool) = i_1 < REF_4655
CONDITION TMP_9055
 minOpenMargin += self.market[assets.getBytes32(i)].getMinOpenMargin(positions[i].amount)
REF_4656(mapping(bytes32 => Market)) -> self_1 (-> []).market
TMP_9056(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
REF_4658(Market) -> REF_4656[TMP_9056]
REF_4660(Position) -> positions_1[i_1]
REF_4661(uint256) -> REF_4660.amount
TMP_9057(uint256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getMinOpenMargin(Market,uint256), arguments:['REF_4658', 'REF_4661'] 
minOpenMargin_2(uint256) = minOpenMargin_1 (c)+ TMP_9057
 upnl += self.market[assets.getBytes32(i)].getUpnl(positions[i])
REF_4662(mapping(bytes32 => Market)) -> self_1 (-> []).market
TMP_9058(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
REF_4664(Market) -> REF_4662[TMP_9058]
REF_4666(Position) -> positions_1[i_1]
TMP_9059(int256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getUpnl(Market,Position), arguments:['REF_4664', 'REF_4666'] 
upnl_2(int256) = upnl_1 (c)+ TMP_9059
 ++ i
i_2(uint256) = i_1 (c)+ 1
 margin + upnl >= minOpenMargin.toInt256()
TMP_9060(int256) = margin_1 (c)+ upnl_1
TMP_9061(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['minOpenMargin_1'] 
TMP_9062(bool) = TMP_9060 >= TMP_9061
RETURN TMP_9062
 met
```
#### FixedPointMathLib.max(int256,int256) [INTERNAL]
```slithir
 z = x ^ x ^ y * y >' x
TMP_13924(int256) = x_1 ^ y_1
TMP_13926 = CONVERT y_1 to int256
TMP_13927 = CONVERT x_1 to int256
TMP_13928(bool) = TMP_13926 > TMP_13927
TMP_13925 = CONVERT TMP_13928 to uint256
TMP_13929(int256) = TMP_13924 * TMP_13925
TMP_13930(int256) = x_1 ^ TMP_13929
z_1(int256) := TMP_13930(int256)
 z
RETURN z_1
```
#### StorageLib.loadClearingHouse() [INTERNAL]
```slithir
CLEARING_HOUSE_SLOT_1(bytes32) := phi(['CLEARING_HOUSE_SLOT_0'])
 slot = CLEARING_HOUSE_SLOT
slot_1(bytes32) := CLEARING_HOUSE_SLOT_1(bytes32)
 ch = slot
ch_1 (-> ['slot'])(ClearingHouse) := slot_1(bytes32)
 ch
RETURN ch_1 (-> ['slot'])
```
#### ClearingHouseLib._getIntendedMarginAndUpnl(ClearingHouse,DynamicArrayLib.DynamicArray,Position[]) [INTERNAL]
```slithir
self_1 (-> [])(ClearingHouse) := phi(['self_1 (-> [])', 'self_1 (-> [])', 'self_1 (-> [])'])
assets_1(DynamicArrayLib.DynamicArray) := phi(['assets_1', 'assets_1', 'assets_1'])
positions_1(Position[]) := phi(['positions_1', 'positions_1', 'positions_1'])
 length = assets.length()
TMP_9106(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.length(DynamicArrayLib.DynamicArray), arguments:['assets_1'] 
length_1(uint256) := TMP_9106(uint256)
 i < length
totalIntendedMargin_1(uint256) := phi(['totalIntendedMargin_0', 'totalIntendedMargin_2'])
totalUpnl_1(int256) := phi(['totalUpnl_2', 'totalUpnl_0'])
i_1(uint256) := phi(['i_0', 'i_2'])
TMP_9107(bool) = i_1 < length_1
CONDITION TMP_9107
 (intendedMargin,upnl) = self.market[assets.getBytes32(i)].getIntendedMarginAndUpnl(positions[i])
REF_4766(mapping(bytes32 => Market)) -> self_1 (-> []).market
TMP_9108(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
REF_4768(Market) -> REF_4766[TMP_9108]
REF_4770(Position) -> positions_1[i_1]
TUPLE_100(uint256,int256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getIntendedMarginAndUpnl(Market,Position), arguments:['REF_4768', 'REF_4770'] 
intendedMargin_1(uint256)= UNPACK TUPLE_100 index: 0 
upnl_1(int256)= UNPACK TUPLE_100 index: 1 
 totalIntendedMargin += intendedMargin
totalIntendedMargin_2(uint256) = totalIntendedMargin_1 (c)+ intendedMargin_1
 totalUpnl += upnl
totalUpnl_2(int256) = totalUpnl_1 (c)+ upnl_1
 ++ i
i_2(uint256) = i_1 (c)+ 1
 (totalIntendedMargin,totalUpnl)
RETURN totalIntendedMargin_1,totalUpnl_1
```
#### ClearingHouseLib._movePop(DynamicArrayLib.DynamicArray,bytes32) [PRIVATE]
```slithir
array_1(DynamicArrayLib.DynamicArray) := phi(['assets_1'])
asset_1(bytes32) := phi(['tradedAsset_1'])
 index = array.indexOf(asset)
TMP_9141(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.indexOf(DynamicArrayLib.DynamicArray,bytes32), arguments:['array_1', 'asset_1'] 
index_1(uint256) := TMP_9141(uint256)
 index == type()(uint256).max
TMP_9143(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_9144(bool) = index_1 == TMP_9143
CONDITION TMP_9144
 array.set(index,asset)
TMP_9145(DynamicArrayLib.DynamicArray) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.set(DynamicArrayLib.DynamicArray,uint256,bytes32), arguments:['array_1', 'index_1', 'asset_1'] 
 array.pop()
TMP_9146(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.pop(DynamicArrayLib.DynamicArray), arguments:['array_1']
```
#### ClearingHouseLib.setAssets(ClearingHouse,address,uint256,uint256,bytes32) [INTERNAL]
```slithir
 oldLength = self.assets[account][subaccount].length()
REF_4535(mapping(address => mapping(uint256 => EnumerableSetLib.Bytes32Set))) -> self_1 (-> []).assets
REF_4536(mapping(uint256 => EnumerableSetLib.Bytes32Set)) -> REF_4535[account_1]
REF_4537(EnumerableSetLib.Bytes32Set) -> REF_4536[subaccount_1]
TMP_8968(uint256) = LIBRARY_CALL, dest:EnumerableSetLib, function:EnumerableSetLib.length(EnumerableSetLib.Bytes32Set), arguments:['REF_4537'] 
oldLength_1(uint256) := TMP_8968(uint256)
 oldLength == newLength
TMP_8969(bool) = oldLength_1 == newLength_1
CONDITION TMP_8969
 oldLength < newLength
TMP_8970(bool) = oldLength_1 < newLength_1
CONDITION TMP_8970
 self.assets[account][subaccount].add(asset)
REF_4539(mapping(address => mapping(uint256 => EnumerableSetLib.Bytes32Set))) -> self_1 (-> []).assets
REF_4540(mapping(uint256 => EnumerableSetLib.Bytes32Set)) -> REF_4539[account_1]
REF_4541(EnumerableSetLib.Bytes32Set) -> REF_4540[subaccount_1]
TMP_8971(bool) = LIBRARY_CALL, dest:EnumerableSetLib, function:EnumerableSetLib.add(EnumerableSetLib.Bytes32Set,bytes32), arguments:['REF_4541', 'asset_1'] 
 self.assets[account][subaccount].remove(asset)
REF_4543(mapping(address => mapping(uint256 => EnumerableSetLib.Bytes32Set))) -> self_1 (-> []).assets
REF_4544(mapping(uint256 => EnumerableSetLib.Bytes32Set)) -> REF_4543[account_1]
REF_4545(EnumerableSetLib.Bytes32Set) -> REF_4544[subaccount_1]
TMP_8972(bool) = LIBRARY_CALL, dest:EnumerableSetLib, function:EnumerableSetLib.remove(EnumerableSetLib.Bytes32Set,bytes32), arguments:['REF_4545', 'asset_1'] 
 account == Constants.GTL
REF_4547(address) -> Constants.GTL
TMP_8973(bool) = account_1 == REF_4547
CONDITION TMP_8973
 oldLength == 0
TMP_8974(bool) = oldLength_1 == 0
CONDITION TMP_8974
 IGTL(Constants.GTL).addSubaccount(subaccount)
REF_4548(address) -> Constants.GTL
TMP_8975 = CONVERT REF_4548 to IGTL
HIGH_LEVEL_CALL, dest:TMP_8975(IGTL), function:addSubaccount, arguments:['subaccount_1']  
 newLength == 0
TMP_8977(bool) = newLength_1 == 0
CONDITION TMP_8977
 IGTL(Constants.GTL).removeSubaccount(subaccount)
REF_4550(address) -> Constants.GTL
TMP_8978 = CONVERT REF_4550 to IGTL
HIGH_LEVEL_CALL, dest:TMP_8978(IGTL), function:removeSubaccount, arguments:['subaccount_1']
```
#### ClearingHouseLib.setPositions(ClearingHouse,bytes32,address,uint256,DynamicArrayLib.DynamicArray,Position[]) [INTERNAL]
```slithir
 length = assets.length()
TMP_8980(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.length(DynamicArrayLib.DynamicArray), arguments:['assets_1'] 
length_1(uint256) := TMP_8980(uint256)
 i < length
i_1(uint256) := phi(['i_2', 'i_0'])
TMP_8981(bool) = i_1 < length_1
CONDITION TMP_8981
 assets.getBytes32(i) == tradedAsset
TMP_8982(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
TMP_8983(bool) = TMP_8982 == tradedAsset_1
CONDITION TMP_8983
 self.market[assets.getBytes32(i)].setPosition(account,subaccount,positions[i])
REF_4554(mapping(bytes32 => Market)) -> self_1 (-> []).market
TMP_8984(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
REF_4556(Market) -> REF_4554[TMP_8984]
REF_4558(Position) -> positions_1[i_1]
LIBRARY_CALL, dest:MarketLib, function:MarketLib.setPosition(Market,address,uint256,Position), arguments:['REF_4556', 'account_1', 'subaccount_1', 'REF_4558'] 
 self.market[assets.getBytes32(i)].position[account][subaccount].lastCumulativeFunding = positions[i].lastCumulativeFunding
REF_4559(mapping(bytes32 => Market)) -> self_1 (-> []).market
TMP_8986(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
REF_4561(Market) -> REF_4559[TMP_8986]
REF_4562(mapping(address => mapping(uint256 => Position))) -> REF_4561.position
REF_4563(mapping(uint256 => Position)) -> REF_4562[account_1]
REF_4564(Position) -> REF_4563[subaccount_1]
REF_4565(int256) -> REF_4564.lastCumulativeFunding
REF_4566(Position) -> positions_1[i_1]
REF_4567(int256) -> REF_4566.lastCumulativeFunding
self_2 (-> [])(ClearingHouse) := phi(['self_1 (-> [])'])
REF_4565(int256) (->self_2 (-> [])) := REF_4567(int256)
self_3 (-> [])(ClearingHouse) := phi(['self_1 (-> [])', 'self_2 (-> [])'])
 ++ i
i_2(uint256) = i_1 (c)+ 1
```
#### MarketLib.cancelCloseOrders(Market,address,uint256) [INTERNAL]
```slithir
 _cancelReduceOnlyOrdersStandard(self,account,subaccount)
INTERNAL_CALL, MarketLib._cancelReduceOnlyOrdersStandard(Market,address,uint256)(self_1 (-> []),account_1,subaccount_1)
 _cancelReduceOnlyOrdersBackstop(self,account,subaccount)
INTERNAL_CALL, MarketLib._cancelReduceOnlyOrdersBackstop(Market,address,uint256)(self_1 (-> []),account_1,subaccount_1)
```
#### MarketLib.updateOI(bytes32,OIDelta) [INTERNAL]
```slithir
 metadata = StorageLib.loadMarketMetadata(asset)
TMP_9319(MarketMetadata) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarketMetadata(bytes32), arguments:['asset_1'] 
metadata_1 (-> ['TMP_9319'])(MarketMetadata) := TMP_9319(MarketMetadata)
 oiDelta.long > 0
REF_5002(int256) -> oiDelta_1.long
TMP_9320(bool) = REF_5002 > 0
CONDITION TMP_9320
 metadata.longOI += oiDelta.long.abs()
REF_5003(uint256) -> metadata_1 (-> ['TMP_9319']).longOI
REF_5004(int256) -> oiDelta_1.long
TMP_9321(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['REF_5004'] 
metadata_2 (-> ['TMP_9319'])(MarketMetadata) := phi(["metadata_1 (-> ['TMP_9319'])"])
REF_5003(-> metadata_2 (-> ['TMP_9319'])) = REF_5003 (c)+ TMP_9321
TMP_9319(MarketMetadata) := phi(["metadata_2 (-> ['TMP_9319'])"])
 oiDelta.long < 0
REF_5006(int256) -> oiDelta_1.long
TMP_9322(bool) = REF_5006 < 0
CONDITION TMP_9322
 metadata.longOI -= oiDelta.long.abs()
REF_5007(uint256) -> metadata_1 (-> ['TMP_9319']).longOI
REF_5008(int256) -> oiDelta_1.long
TMP_9323(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['REF_5008'] 
metadata_3 (-> ['TMP_9319'])(MarketMetadata) := phi(["metadata_1 (-> ['TMP_9319'])"])
REF_5007(-> metadata_3 (-> ['TMP_9319'])) = REF_5007 (c)- TMP_9323
TMP_9319(MarketMetadata) := phi(["metadata_3 (-> ['TMP_9319'])"])
metadata_4 (-> ['TMP_9319'])(MarketMetadata) := phi(["metadata_1 (-> ['TMP_9319'])", "metadata_3 (-> ['TMP_9319'])"])
metadata_5 (-> ['TMP_9319'])(MarketMetadata) := phi(["metadata_2 (-> ['TMP_9319'])", "metadata_1 (-> ['TMP_9319'])"])
 oiDelta.short > 0
REF_5010(int256) -> oiDelta_1.short
TMP_9324(bool) = REF_5010 > 0
CONDITION TMP_9324
 metadata.shortOI += oiDelta.short.abs()
REF_5011(uint256) -> metadata_5 (-> ['TMP_9319']).shortOI
REF_5012(int256) -> oiDelta_1.short
TMP_9325(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['REF_5012'] 
metadata_6 (-> ['TMP_9319'])(MarketMetadata) := phi(["metadata_5 (-> ['TMP_9319'])"])
REF_5011(-> metadata_6 (-> ['TMP_9319'])) = REF_5011 (c)+ TMP_9325
TMP_9319(MarketMetadata) := phi(["metadata_6 (-> ['TMP_9319'])"])
 oiDelta.short < 0
REF_5014(int256) -> oiDelta_1.short
TMP_9326(bool) = REF_5014 < 0
CONDITION TMP_9326
 metadata.shortOI -= oiDelta.short.abs()
REF_5015(uint256) -> metadata_5 (-> ['TMP_9319']).shortOI
REF_5016(int256) -> oiDelta_1.short
TMP_9327(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['REF_5016'] 
metadata_7 (-> ['TMP_9319'])(MarketMetadata) := phi(["metadata_5 (-> ['TMP_9319'])"])
REF_5015(-> metadata_7 (-> ['TMP_9319'])) = REF_5015 (c)- TMP_9327
TMP_9319(MarketMetadata) := phi(["metadata_7 (-> ['TMP_9319'])"])
```
#### DynamicArrayLib.length(DynamicArrayLib.DynamicArray) [INTERNAL]
```slithir
 a.data.length
REF_5750(uint256[]) -> a_1.data
REF_5751 -> LENGTH REF_5750
RETURN REF_5751
```
#### CollateralManagerLib.handleCollateralDelta(CollateralManager,address,int256) [INTERNAL]
```slithir
 collateralDelta > 0
TMP_9185(bool) = collateralDelta_1 > 0
CONDITION TMP_9185
 self.debitAccount(account,collateralDelta.abs())
TMP_9186(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['collateralDelta_1'] 
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.debitAccount(CollateralManager,address,uint256), arguments:['self_1 (-> [])', 'account_1', 'TMP_9186'] 
 collateralDelta < 0
TMP_9188(bool) = collateralDelta_1 < 0
CONDITION TMP_9188
 self.creditAccount(account,collateralDelta.abs())
TMP_9189(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['collateralDelta_1'] 
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.creditAccount(CollateralManager,address,uint256), arguments:['self_1 (-> [])', 'account_1', 'TMP_9189']
```
#### PositionLib._close(Position,Side,uint256,uint256) [PRIVATE]
```slithir
self_1(Position) := phi(['self_1'])
side_1(Side) := phi(['side_1'])
quoteTraded_1(uint256) := phi(['quoteTraded_1'])
baseTraded_1(uint256) := phi(['baseTraded_1'])
 cache.closeSize = self.amount.min(baseTraded)
REF_5345(uint256) -> cache_0.closeSize
REF_5346(uint256) -> self_1.amount
TMP_9561(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.min(uint256,uint256), arguments:['REF_5346', 'baseTraded_1'] 
cache_1(PositionLib.__CloseCache__) := phi(['cache_0'])
REF_5345(uint256) (->cache_1) := TMP_9561(uint256)
 cache.closedOpenNotional = self.openNotional.fullMulDiv(cache.closeSize,self.amount)
REF_5348(uint256) -> cache_1.closedOpenNotional
REF_5349(uint256) -> self_1.openNotional
REF_5351(uint256) -> cache_1.closeSize
REF_5352(uint256) -> self_1.amount
TMP_9562(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_5349', 'REF_5351', 'REF_5352'] 
cache_2(PositionLib.__CloseCache__) := phi(['cache_1'])
REF_5348(uint256) (->cache_2) := TMP_9562(uint256)
 cache.currentNotional = quoteTraded.fullMulDiv(cache.closeSize,baseTraded)
REF_5353(uint256) -> cache_2.currentNotional
REF_5355(uint256) -> cache_2.closeSize
TMP_9563(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['quoteTraded_1', 'REF_5355', 'baseTraded_1'] 
cache_3(PositionLib.__CloseCache__) := phi(['cache_2'])
REF_5353(uint256) (->cache_3) := TMP_9563(uint256)
 result.rpnl = _pnl(self.isLong,cache.closedOpenNotional,cache.currentNotional)
REF_5356(int256) -> result_0.rpnl
REF_5357(bool) -> self_1.isLong
REF_5358(uint256) -> cache_3.closedOpenNotional
REF_5359(uint256) -> cache_3.currentNotional
TMP_9564(int256) = INTERNAL_CALL, PositionLib._pnl(bool,uint256,uint256)(REF_5357,REF_5358,REF_5359)
result_1(PositionUpdateResult) := phi(['result_0'])
REF_5356(int256) (->result_1) := TMP_9564(int256)
 result.marginDelta = - cache.closedOpenNotional.fullMulDiv(1e18,self.leverage).toInt256()
REF_5360(int256) -> result_1.marginDelta
REF_5361(uint256) -> cache_3.closedOpenNotional
REF_5363(uint256) -> self_1.leverage
TMP_9565(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_5361', '1000000000000000000', 'REF_5363'] 
TMP_9566(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['TMP_9565'] 
TMP_9567(uint256) = 0 (c)- TMP_9566
result_2(PositionUpdateResult) := phi(['result_1'])
REF_5360(int256) (->result_2) := TMP_9567(uint256)
 self.openNotional -= cache.closedOpenNotional
REF_5365(uint256) -> self_1.openNotional
REF_5366(uint256) -> cache_3.closedOpenNotional
self_2(Position) := phi(['self_1'])
REF_5365(-> self_2) = REF_5365 (c)- REF_5366
 self.amount -= cache.closeSize
REF_5367(uint256) -> self_2.amount
REF_5368(uint256) -> cache_3.closeSize
self_3(Position) := phi(['self_2'])
REF_5367(-> self_3) = REF_5367 (c)- REF_5368
 quoteTraded -= cache.currentNotional
REF_5369(uint256) -> cache_3.currentNotional
quoteTraded_2(uint256) = quoteTraded_1 (c)- REF_5369
 baseTraded -= cache.closeSize
REF_5370(uint256) -> cache_3.closeSize
baseTraded_2(uint256) = baseTraded_1 (c)- REF_5370
 self.isLong
REF_5371(bool) -> self_3.isLong
CONDITION REF_5371
 result.oiDelta.long = - cache.closeSize.toInt256()
REF_5372(OIDelta) -> result_2.oiDelta
REF_5373(int256) -> REF_5372.long
REF_5374(uint256) -> cache_3.closeSize
TMP_9568(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_5374'] 
TMP_9569(uint256) = 0 (c)- TMP_9568
result_3(PositionUpdateResult) := phi(['result_2'])
REF_5373(int256) (->result_3) := TMP_9569(uint256)
 result.oiDelta.short = - cache.closeSize.toInt256()
REF_5376(OIDelta) -> result_2.oiDelta
REF_5377(int256) -> REF_5376.short
REF_5378(uint256) -> cache_3.closeSize
TMP_9570(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_5378'] 
TMP_9571(uint256) = 0 (c)- TMP_9570
result_4(PositionUpdateResult) := phi(['result_2'])
REF_5377(int256) (->result_4) := TMP_9571(uint256)
result_5(PositionUpdateResult) := phi(['result_3', 'result_4'])
 result.sideClose = self.amount == 0
REF_5380(bool) -> result_5.sideClose
REF_5381(uint256) -> self_3.amount
TMP_9572(bool) = REF_5381 == 0
result_6(PositionUpdateResult) := phi(['result_5'])
REF_5380(bool) (->result_6) := TMP_9572(bool)
CONDITION REF_5380
 baseTraded > 0
TMP_9573(bool) = baseTraded_2 > 0
CONDITION TMP_9573
 result.marginDelta = _open(self,side,quoteTraded,baseTraded)
REF_5382(int256) -> result_6.marginDelta
TMP_9574(int256) = INTERNAL_CALL, PositionLib._open(Position,Side,uint256,uint256)(self_3,side_1,quoteTraded_2,baseTraded_2)
result_7(PositionUpdateResult) := phi(['result_6'])
REF_5382(int256) (->result_7) := TMP_9574(int256)
 self.isLong
REF_5383(bool) -> self_3.isLong
CONDITION REF_5383
 result.oiDelta.long += baseTraded.toInt256()
REF_5384(OIDelta) -> result_7.oiDelta
REF_5385(int256) -> REF_5384.long
TMP_9575(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['baseTraded_2'] 
result_8(PositionUpdateResult) := phi(['result_7'])
REF_5385(-> result_8) = REF_5385 (c)+ TMP_9575
 result.oiDelta.short += baseTraded.toInt256()
REF_5387(OIDelta) -> result_7.oiDelta
REF_5388(int256) -> REF_5387.short
TMP_9576(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['baseTraded_2'] 
result_9(PositionUpdateResult) := phi(['result_7'])
REF_5388(-> result_9) = REF_5388 (c)+ TMP_9576
result_10(PositionUpdateResult) := phi(['result_8', 'result_9'])
 delete self.lastCumulativeFunding
REF_5390(int256) -> self_3.lastCumulativeFunding
self_4 = delete REF_5390 
 delete self.isLong
REF_5391(bool) -> self_4.isLong
self_5 = delete REF_5391 
result_11(PositionUpdateResult) := phi(['result_7', 'result_6'])
 result
RETURN result_11
```
#### PositionLib._open(Position,Side,uint256,uint256) [PRIVATE]
```slithir
self_1(Position) := phi(['self_1', 'self_3'])
side_1(Side) := phi(['side_1', 'side_1'])
quoteTraded_1(uint256) := phi(['quoteTraded_2', 'quoteTraded_1'])
baseTraded_1(uint256) := phi(['baseTraded_2', 'baseTraded_1'])
 self.leverage == 0
REF_5336(uint256) -> self_1.leverage
TMP_9557(bool) = REF_5336 == 0
CONDITION TMP_9557
 self.leverage = 1e18
REF_5337(uint256) -> self_1.leverage
self_2(Position) := phi(['self_1'])
REF_5337(uint256) (->self_2) := 1000000000000000000(uint256)
self_3(Position) := phi(['self_2', 'self_1'])
 self.isLong = side == Side.BUY
REF_5338(bool) -> self_3.isLong
REF_5339(Side) -> Side.BUY
TMP_9558(bool) = side_1 == REF_5339
self_4(Position) := phi(['self_3'])
REF_5338(bool) (->self_4) := TMP_9558(bool)
 self.amount += baseTraded
REF_5340(uint256) -> self_4.amount
self_5(Position) := phi(['self_4'])
REF_5340(-> self_5) = REF_5340 (c)+ baseTraded_1
 self.openNotional += quoteTraded
REF_5341(uint256) -> self_5.openNotional
self_6(Position) := phi(['self_5'])
REF_5341(-> self_6) = REF_5341 (c)+ quoteTraded_1
 marginDelta = quoteTraded.fullMulDiv(1e18,self.leverage).toInt256()
REF_5343(uint256) -> self_6.leverage
TMP_9559(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['quoteTraded_1', '1000000000000000000', 'REF_5343'] 
TMP_9560(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['TMP_9559'] 
marginDelta_1(int256) := TMP_9560(int256)
 marginDelta
RETURN marginDelta_1
```

#### ClearingHouseLib.getAccount(ClearingHouse,address,uint256) [INTERNAL]
```slithir
 assets = self.assets[account][subaccount].values().wrap()
REF_4590(mapping(address => mapping(uint256 => EnumerableSetLib.Bytes32Set))) -> self_1 (-> []).assets
REF_4591(mapping(uint256 => EnumerableSetLib.Bytes32Set)) -> REF_4590[account_1]
REF_4592(EnumerableSetLib.Bytes32Set) -> REF_4591[subaccount_1]
TMP_9012(bytes32[]) = LIBRARY_CALL, dest:EnumerableSetLib, function:EnumerableSetLib.values(EnumerableSetLib.Bytes32Set), arguments:['REF_4592'] 
TMP_9013(DynamicArrayLib.DynamicArray) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.wrap(bytes32[]), arguments:['TMP_9012'] 
assets_1(DynamicArrayLib.DynamicArray) := TMP_9013(DynamicArrayLib.DynamicArray)
 positions = _getPositions(self,assets,account,subaccount,false)
TMP_9014(Position[]) = INTERNAL_CALL, ClearingHouseLib._getPositions(ClearingHouse,DynamicArrayLib.DynamicArray,address,uint256,bool)(self_1 (-> []),assets_1,account_1,subaccount_1,False)
positions_1(Position[]) = ['TMP_9014(Position[])']
 (assets,positions)
RETURN assets_1,positions_1
```
#### CollateralManagerLib.getMarginBalance(CollateralManager,address,uint256) [INTERNAL]
```slithir
 self.margin[account][subaccount]
REF_4844(mapping(address => mapping(uint256 => int256))) -> self_1 (-> []).margin
REF_4845(mapping(uint256 => int256)) -> REF_4844[account_1]
REF_4846(int256) -> REF_4845[subaccount_1]
RETURN REF_4846
```
#### ClearingHouseLib._prorateMargin(int256,uint256,uint256) [INTERNAL]
```slithir
margin_1(int256) := phi(['margin_1'])
assetNotional_1(uint256) := phi(['assetNotional_0'])
totalNotional_1(uint256) := phi(['totalNotional_1'])
 totalNotional == 0
TMP_9073(bool) = totalNotional_1 == 0
CONDITION TMP_9073
 0
RETURN 0
 proratedMargin = margin.abs().fullMulDiv(assetNotional,totalNotional).toInt256()
TMP_9074(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['margin_1'] 
TMP_9075(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['TMP_9074', 'assetNotional_1', 'totalNotional_1'] 
TMP_9076(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['TMP_9075'] 
proratedMargin_1(int256) := TMP_9076(int256)
 margin < 0
TMP_9077(bool) = margin_1 < 0
CONDITION TMP_9077
 proratedMargin = - proratedMargin
TMP_9078(int256) = 0 (c)- proratedMargin_1
proratedMargin_2(int256) := TMP_9078(int256)
proratedMargin_3(int256) := phi(['proratedMargin_2', 'proratedMargin_1'])
 proratedMargin
RETURN proratedMargin_3
```
#### MarketLib.getNotionalValue(Market,Position) [INTERNAL]
```slithir
 position.amount.fullMulDiv(self.markPrice,1e18)
REF_5053(uint256) -> position_1.amount
REF_5055(uint256) -> self_1 (-> []).markPrice
TMP_9347(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_5053', 'REF_5055', '1000000000000000000'] 
RETURN TMP_9347
 notional
```
#### DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256) [INTERNAL]
```slithir
 result = mload(uint256)(mload(uint256)(a) + 0x20 + i << 5)
TMP_12366(uint256) = SOLIDITY_CALL mload(uint256)(a_1)
TMP_12367(uint256) = TMP_12366 + 32
TMP_12368(uint256) = i_1 << 5
TMP_12369(uint256) = TMP_12367 + TMP_12368
TMP_12370(uint256) = SOLIDITY_CALL mload(uint256)(TMP_12369)
result_1(bytes32) := TMP_12370(uint256)
 result
RETURN result_1
```
#### MarketLib.realizeFundingPayment(bytes32,Position) [INTERNAL]
```slithir
 position.realizeFundingPayment(StorageLib.loadFundingRateEngine(asset).getCumulativeFunding())
TMP_9316(FundingRateEngine) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadFundingRateEngine(bytes32), arguments:['asset_1'] 
TMP_9317(int256) = LIBRARY_CALL, dest:FundingLib, function:FundingLib.getCumulativeFunding(FundingRateEngine), arguments:['TMP_9316'] 
TMP_9318(int256) = LIBRARY_CALL, dest:PositionLib, function:PositionLib.realizeFundingPayment(Position,int256), arguments:['position_1', 'TMP_9317'] 
RETURN TMP_9318
 fundingPayment
```
#### FeeManagerLib.getTakerFeeRate(FeeManager,address) [INTERNAL]
```slithir
 self.takerFeeRates.getFeeAt(uint256(self.accountFeeTier[account]))
REF_4858(PackedFeeRates) -> self_1 (-> []).takerFeeRates
REF_4860(mapping(address => FeeTier)) -> self_1 (-> []).accountFeeTier
REF_4861(FeeTier) -> REF_4860[account_1]
TMP_9203 = CONVERT REF_4861 to uint256
TMP_9204(uint16) = LIBRARY_CALL, dest:PackedFeeRatesLib, function:PackedFeeRatesLib.getFeeAt(PackedFeeRates,uint256), arguments:['REF_4858', 'TMP_9203'] 
RETURN TMP_9204
 feeRate
```
#### CLOBLib.placeOrder(address,PlaceOrderArgs,BookType) [INTERNAL]
```slithir
 ds = _getStorage(args.asset,bookType)
REF_4000(bytes32) -> args_1.asset
TMP_8669(Book) = INTERNAL_CALL, CLOBLib._getStorage(bytes32,BookType)(REF_4000,bookType_1)
ds_1 (-> ['TMP_8669'])(Book) := TMP_8669(Book)
 uint8(args.tif) <= 1 && args.limitPrice == 0
REF_4001(TiF) -> args_1.tif
TMP_8670 = CONVERT REF_4001 to uint8
TMP_8671(bool) = TMP_8670 <= 1
REF_4002(uint256) -> args_1.limitPrice
TMP_8672(bool) = REF_4002 == 0
TMP_8673(bool) = TMP_8671 && TMP_8672
CONDITION TMP_8673
 revert InvalidMakerPrice()()
TMP_8674(None) = SOLIDITY_CALL revert InvalidMakerPrice()()
 args.amount == 0
REF_4003(uint256) -> args_1.amount
TMP_8675(bool) = REF_4003 == 0
CONDITION TMP_8675
 revert ZeroAmount()()
TMP_8676(None) = SOLIDITY_CALL revert ZeroAmount()()
 ds.assertPriceInBounds(args.limitPrice)
REF_4005(uint256) -> args_1.limitPrice
LIBRARY_CALL, dest:BookLib, function:BookLib.assertPriceInBounds(Book,uint256), arguments:["ds_1 (-> ['TMP_8669'])", 'REF_4005'] 
 args.expiryTime.isExpired()
REF_4006(uint32) -> args_1.expiryTime
TMP_8678(bool) = LIBRARY_CALL, dest:OrderLib, function:OrderLib.isExpired(uint256), arguments:['REF_4006'] 
CONDITION TMP_8678
 revert OrderAlreadyExpired()()
TMP_8679(None) = SOLIDITY_CALL revert OrderAlreadyExpired()()
 orderId = ds.toOrderId(account,args.clientOrderId)
REF_4009(uint96) -> args_1.clientOrderId
TMP_8680(uint256) = LIBRARY_CALL, dest:BookLib, function:BookLib.toOrderId(Book,address,uint96), arguments:["ds_1 (-> ['TMP_8669'])", 'account_1', 'REF_4009'] 
orderId_1(uint256) := TMP_8680(uint256)
 newOrder = args.toOrder(orderId,account)
TMP_8681(Order) = LIBRARY_CALL, dest:OrderLib, function:OrderLib.toOrder(PlaceOrderArgs,uint256,address), arguments:['args_1', 'orderId_1', 'account_1'] 
newOrder_1(Order) := TMP_8681(Order)
 args.side == Side.BUY
REF_4011(Side) -> args_1.side
REF_4012(Side) -> Side.BUY
TMP_8682(bool) = REF_4011 == REF_4012
CONDITION TMP_8682
 result = _processBuyOrder(ds,newOrder,args)
TMP_8683(PlaceOrderResult) = INTERNAL_CALL, CLOBLib._processBuyOrder(Book,Order,PlaceOrderArgs)(ds_1 (-> ['TMP_8669']),newOrder_1,args_1)
result_1(PlaceOrderResult) := TMP_8683(PlaceOrderResult)
 result = _processSellOrder(ds,newOrder,args)
TMP_8684(PlaceOrderResult) = INTERNAL_CALL, CLOBLib._processSellOrder(Book,Order,PlaceOrderArgs)(ds_1 (-> ['TMP_8669']),newOrder_1,args_1)
result_2(PlaceOrderResult) := TMP_8684(PlaceOrderResult)
result_3(PlaceOrderResult) := phi(['result_1', 'result_2'])
 result.baseTraded + result.quoteTraded + result.basePosted == 0
REF_4013(uint256) -> result_3.baseTraded
REF_4014(uint256) -> result_3.quoteTraded
TMP_8685(uint256) = REF_4013 (c)+ REF_4014
REF_4015(uint256) -> result_3.basePosted
TMP_8686(uint256) = TMP_8685 (c)+ REF_4015
TMP_8687(bool) = TMP_8686 == 0
CONDITION TMP_8687
 revert ZeroOrder()()
TMP_8688(None) = SOLIDITY_CALL revert ZeroOrder()()
 ! args.reduceOnly && result.basePosted > 0
REF_4016(bool) -> args_1.reduceOnly
TMP_8689 = UnaryType.BANG REF_4016 
REF_4017(uint256) -> result_3.basePosted
TMP_8690(bool) = REF_4017 > 0
TMP_8691(bool) = TMP_8689 && TMP_8690
CONDITION TMP_8691
 _updateOrderbookNotional(args.asset,account,args.subaccount,result.basePosted.fullMulDiv(newOrder.price,1e18).toInt256())
REF_4018(bytes32) -> args_1.asset
REF_4019(uint256) -> args_1.subaccount
REF_4020(uint256) -> result_3.basePosted
REF_4022(uint256) -> newOrder_1.price
TMP_8692(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_4020', 'REF_4022', '1000000000000000000'] 
TMP_8693(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['TMP_8692'] 
INTERNAL_CALL, CLOBLib._updateOrderbookNotional(bytes32,address,uint256,int256)(REF_4018,account_1,REF_4019,TMP_8693)
 _emitOrderProcessed(account,args,result,bookType)
INTERNAL_CALL, CLOBLib._emitOrderProcessed(address,PlaceOrderArgs,PlaceOrderResult,BookType)(account_1,args_1,result_3,bookType_1)
 result
RETURN result_3
```
#### MarketLib._getLiquidationAmount(Market,uint256) [INTERNAL]
```slithir
self_1 (-> [])(Market) := phi(['self_1 (-> [])'])
positionAmount_1(uint256) := phi(['amount_1'])
 positionAmount.fullMulDiv(self.markPrice,1e18) < StorageLib.loadMarketSettings(self.asset).partialLiquidationThreshold
REF_5212(uint256) -> self_1 (-> []).markPrice
TMP_9458(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['positionAmount_1', 'REF_5212', '1000000000000000000'] 
REF_5214(bytes32) -> self_1 (-> []).asset
TMP_9459(MarketSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarketSettings(bytes32), arguments:['REF_5214'] 
REF_5215(uint256) -> TMP_9459.partialLiquidationThreshold
TMP_9460(bool) = TMP_9458 < REF_5215
CONDITION TMP_9460
 positionAmount
RETURN positionAmount_1
 positionAmount.fullMulDiv(StorageLib.loadMarketSettings(self.asset).partialLiquidationRate,1e18)
REF_5218(bytes32) -> self_1 (-> []).asset
TMP_9461(MarketSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarketSettings(bytes32), arguments:['REF_5218'] 
REF_5219(uint256) -> TMP_9461.partialLiquidationRate
TMP_9462(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['positionAmount_1', 'REF_5219', '1000000000000000000'] 
RETURN TMP_9462
```
#### BackstopLiquidatorDataLib._getLiquidatorsAndClear() [INTERNAL]
```slithir
TRANSIENT_LIQUIDATORS_SLOT_2(bytes32) := phi(['TRANSIENT_LIQUIDATORS_SLOT_0'])
 slot = TRANSIENT_LIQUIDATORS_SLOT
slot_1(bytes32) := TRANSIENT_LIQUIDATORS_SLOT_2(bytes32)
 len__getLiquidatorsAndClear_asm_0 = tload(uint256)(slot)
TMP_8469(uint256) = SOLIDITY_CALL tload(uint256)(slot_1)
len__getLiquidatorsAndClear_asm_0_1(uint256) := TMP_8469(uint256)
 liquidators = mload(uint256)(0x40)
TMP_8470(uint256) = SOLIDITY_CALL mload(uint256)(64)
liquidators_1(address[]) = ['TMP_8470(uint256)']
 mstore(uint256,uint256)(liquidators,len__getLiquidatorsAndClear_asm_0)
TMP_8471(None) = SOLIDITY_CALL mstore(uint256,uint256)(liquidators_1,len__getLiquidatorsAndClear_asm_0_1)
 mstore(uint256,uint256)(0x00,slot)
TMP_8472(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,slot_1)
 dataSlot__getLiquidatorsAndClear_asm_0 = keccak256(uint256,uint256)(0x00,0x20)
TMP_8473(uint256) = SOLIDITY_CALL keccak256(uint256,uint256)(0,32)
dataSlot__getLiquidatorsAndClear_asm_0_1(uint256) := TMP_8473(uint256)
 memPointer__getLiquidatorsAndClear_asm_0 = liquidators + 0x20
TMP_8474(address[]) = liquidators_1 + 32
memPointer__getLiquidatorsAndClear_asm_0_1(uint256) := TMP_8474(address[])
 i__getLiquidatorsAndClear_asm_0 = 0
i__getLiquidatorsAndClear_asm_0_1(uint256) := 0(uint256)
 i__getLiquidatorsAndClear_asm_0 < len__getLiquidatorsAndClear_asm_0
i__getLiquidatorsAndClear_asm_0_2(uint256) := phi(['i__getLiquidatorsAndClear_asm_0_1', 'i__getLiquidatorsAndClear_asm_0_3'])
TMP_8475(bool) = i__getLiquidatorsAndClear_asm_0_2 < len__getLiquidatorsAndClear_asm_0_1
CONDITION TMP_8475
 mstore(uint256,uint256)(memPointer__getLiquidatorsAndClear_asm_0 + i__getLiquidatorsAndClear_asm_0 * 0x20,tload(uint256)(dataSlot__getLiquidatorsAndClear_asm_0 + i__getLiquidatorsAndClear_asm_0))
TMP_8476(uint256) = i__getLiquidatorsAndClear_asm_0_2 * 32
TMP_8477(uint256) = memPointer__getLiquidatorsAndClear_asm_0_1 + TMP_8476
TMP_8478(uint256) = dataSlot__getLiquidatorsAndClear_asm_0_1 + i__getLiquidatorsAndClear_asm_0_2
TMP_8479(uint256) = SOLIDITY_CALL tload(uint256)(TMP_8478)
TMP_8480(None) = SOLIDITY_CALL mstore(uint256,uint256)(TMP_8477,TMP_8479)
 tstore(uint256,uint256)(dataSlot__getLiquidatorsAndClear_asm_0 + i__getLiquidatorsAndClear_asm_0,0)
TMP_8481(uint256) = dataSlot__getLiquidatorsAndClear_asm_0_1 + i__getLiquidatorsAndClear_asm_0_2
TMP_8482(None) = SOLIDITY_CALL tstore(uint256,uint256)(TMP_8481,0)
 i__getLiquidatorsAndClear_asm_0 = i__getLiquidatorsAndClear_asm_0 + 1
TMP_8483(uint256) = i__getLiquidatorsAndClear_asm_0_2 + 1
i__getLiquidatorsAndClear_asm_0_3(uint256) := TMP_8483(uint256)
 mstore(uint256,uint256)(0x40,memPointer__getLiquidatorsAndClear_asm_0 + len__getLiquidatorsAndClear_asm_0 * 0x20)
TMP_8484(uint256) = len__getLiquidatorsAndClear_asm_0_1 * 32
TMP_8485(uint256) = memPointer__getLiquidatorsAndClear_asm_0_1 + TMP_8484
TMP_8486(None) = SOLIDITY_CALL mstore(uint256,uint256)(64,TMP_8485)
 tstore(uint256,uint256)(slot,0)
TMP_8487(None) = SOLIDITY_CALL tstore(uint256,uint256)(slot_1,0)
 liquidators
RETURN liquidators_1
```
#### BackstopLiquidatorDataLib._getVolumeAndClear(address) [INTERNAL]
```slithir
liquidator_1(address) := phi(['REF_3695'])
TRANSIENT_VOLUME_SLOT_2(bytes32) := phi(['TRANSIENT_VOLUME_SLOT_0'])
 slot = keccak256(bytes)(abi.encode(TRANSIENT_VOLUME_SLOT,liquidator))
TMP_8488(bytes) = SOLIDITY_CALL abi.encode()(TRANSIENT_VOLUME_SLOT_2,liquidator_1)
TMP_8489(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_8488)
slot_1(bytes32) := TMP_8489(bytes32)
 volume = tload(uint256)(slot + 1)
TMP_8490(bytes32) = slot_1 + 1
TMP_8491(uint256) = SOLIDITY_CALL tload(uint256)(TMP_8490)
volume_1(uint256) := TMP_8491(uint256)
 tstore(uint256,uint256)(slot,0)
TMP_8492(None) = SOLIDITY_CALL tstore(uint256,uint256)(slot_1,0)
 tstore(uint256,uint256)(slot + 1,0)
TMP_8493(bytes32) = slot_1 + 1
TMP_8494(None) = SOLIDITY_CALL tstore(uint256,uint256)(TMP_8493,0)
 volume
RETURN volume_1
```
#### ClearingHouseLib.isLiquidatable(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256,BookType) [INTERNAL]
```slithir
 i < assets.length()
cache_1(ClearingHouseLib.__LiquidatableCheckCache__) := phi(['cache_3', 'cache_0'])
i_1(uint256) := phi(['i_0', 'i_2'])
TMP_9046(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.length(DynamicArrayLib.DynamicArray), arguments:['assets_1'] 
TMP_9047(bool) = i_1 < TMP_9046
CONDITION TMP_9047
 (cache.upnl,cache.minMargin) = self.market[assets.getBytes32(i)].getUpnlAndMinMargin(positions[i],bookType)
REF_4640(int256) -> cache_1.upnl
REF_4641(uint256) -> cache_1.minMargin
REF_4642(mapping(bytes32 => Market)) -> self_1 (-> []).market
TMP_9048(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
REF_4644(Market) -> REF_4642[TMP_9048]
REF_4646(Position) -> positions_1[i_1]
TUPLE_98(int256,uint256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getUpnlAndMinMargin(Market,Position,BookType), arguments:['REF_4644', 'REF_4646', 'bookType_1'] 
REF_4640(int256)= UNPACK TUPLE_98 index: 0 
REF_4641(uint256)= UNPACK TUPLE_98 index: 1 
 cache.totalUpnl += cache.upnl
REF_4647(int256) -> cache_1.totalUpnl
REF_4648(int256) -> cache_1.upnl
cache_2(ClearingHouseLib.__LiquidatableCheckCache__) := phi(['cache_1'])
REF_4647(-> cache_2) = REF_4647 (c)+ REF_4648
 cache.totalMinMargin += cache.minMargin
REF_4649(uint256) -> cache_2.totalMinMargin
REF_4650(uint256) -> cache_2.minMargin
cache_3(ClearingHouseLib.__LiquidatableCheckCache__) := phi(['cache_2'])
REF_4649(-> cache_3) = REF_4649 (c)+ REF_4650
 ++ i
i_2(uint256) = i_1 (c)+ 1
 cache.totalMinMargin == 0 && margin < 0
REF_4651(uint256) -> cache_1.totalMinMargin
TMP_9049(bool) = REF_4651 == 0
TMP_9050(bool) = margin_1 < 0
TMP_9051(bool) = TMP_9049 && TMP_9050
CONDITION TMP_9051
 true
RETURN True
 (margin + cache.totalUpnl) < cache.totalMinMargin.toInt256()
REF_4652(int256) -> cache_1.totalUpnl
TMP_9052(int256) = margin_1 (c)+ REF_4652
REF_4653(uint256) -> cache_1.totalMinMargin
TMP_9053(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_4653'] 
TMP_9054(bool) = TMP_9052 < TMP_9053
RETURN TMP_9054
 liquidatable
```
#### MarketLib.getUpnl(Market,Position) [INTERNAL]
```slithir
 currentNotional = position.amount.fullMulDiv(self.markPrice,1e18)
REF_5076(uint256) -> position_1.amount
REF_5078(uint256) -> self_1 (-> []).markPrice
TMP_9358(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_5076', 'REF_5078', '1000000000000000000'] 
currentNotional_1(uint256) := TMP_9358(uint256)
 _calcUpnl(position.isLong,position.openNotional,currentNotional)
REF_5079(bool) -> position_1.isLong
REF_5080(uint256) -> position_1.openNotional
TMP_9359(int256) = INTERNAL_CALL, MarketLib._calcUpnl(bool,uint256,uint256)(REF_5079,REF_5080,currentNotional_1)
RETURN TMP_9359
 upnl
```
#### MarketLib.getMinOpenMargin(Market,uint256) [INTERNAL]
```slithir
 positionNotional = positionAmount.fullMulDiv(self.markPrice,1e18)
REF_5063(uint256) -> self_1 (-> []).markPrice
TMP_9351(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['positionAmount_1', 'REF_5063', '1000000000000000000'] 
positionNotional_1(uint256) := TMP_9351(uint256)
 minOpenMargin = positionNotional.fullMulDiv(1e18,StorageLib.loadMarketSettings(self.asset).maxOpenLeverage)
REF_5066(bytes32) -> self_1 (-> []).asset
TMP_9352(MarketSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarketSettings(bytes32), arguments:['REF_5066'] 
REF_5067(uint256) -> TMP_9352.maxOpenLeverage
TMP_9353(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['positionNotional_1', '1000000000000000000', 'REF_5067'] 
minOpenMargin_1(uint256) := TMP_9353(uint256)
 minOpenMargin
RETURN minOpenMargin_1
```
#### MarketLib.getIntendedMarginAndUpnl(Market,Position) [INTERNAL]
```slithir
 position.amount == 0
REF_5068(uint256) -> position_1.amount
TMP_9354(bool) = REF_5068 == 0
CONDITION TMP_9354
 (0,0)
RETURN 0,0
 currentNotional = position.amount.fullMulDiv(self.markPrice,1e18)
REF_5069(uint256) -> position_1.amount
REF_5071(uint256) -> self_1 (-> []).markPrice
TMP_9355(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_5069', 'REF_5071', '1000000000000000000'] 
currentNotional_1(uint256) := TMP_9355(uint256)
 upnl = _calcUpnl(position.isLong,position.openNotional,currentNotional)
REF_5072(bool) -> position_1.isLong
REF_5073(uint256) -> position_1.openNotional
TMP_9356(int256) = INTERNAL_CALL, MarketLib._calcUpnl(bool,uint256,uint256)(REF_5072,REF_5073,currentNotional_1)
upnl_1(int256) := TMP_9356(int256)
 intendedMargin = currentNotional.fullMulDiv(1e18,position.leverage)
REF_5075(uint256) -> position_1.leverage
TMP_9357(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['currentNotional_1', '1000000000000000000', 'REF_5075'] 
intendedMargin_1(uint256) := TMP_9357(uint256)
 (intendedMargin,upnl)
RETURN intendedMargin_1,upnl_1
```
#### DynamicArrayLib.pop(DynamicArrayLib.DynamicArray) [INTERNAL]
```slithir
 o_pop_asm_0 = mload(uint256)(a)
TMP_12301(uint256) = SOLIDITY_CALL mload(uint256)(a_1)
o_pop_asm_0_1(uint256) := TMP_12301(uint256)
 n_pop_asm_0 = mload(uint256)(o_pop_asm_0)
TMP_12302(uint256) = SOLIDITY_CALL mload(uint256)(o_pop_asm_0_1)
n_pop_asm_0_1(uint256) := TMP_12302(uint256)
 result = mload(uint256)(o_pop_asm_0 + n_pop_asm_0 << 5)
TMP_12303(uint256) = n_pop_asm_0_1 << 5
TMP_12304(uint256) = o_pop_asm_0_1 + TMP_12303
TMP_12305(uint256) = SOLIDITY_CALL mload(uint256)(TMP_12304)
result_1(uint256) := TMP_12305(uint256)
 mstore(uint256,uint256)(o_pop_asm_0,n_pop_asm_0 - ! ! n_pop_asm_0)
TMP_12306 = UnaryType.BANG n_pop_asm_0_1 
TMP_12307 = UnaryType.BANG TMP_12306 
TMP_12308(uint256) = n_pop_asm_0_1 - TMP_12307
TMP_12309(None) = SOLIDITY_CALL mstore(uint256,uint256)(o_pop_asm_0_1,TMP_12308)
 result
RETURN result_1
```
#### DynamicArrayLib.set(DynamicArrayLib.DynamicArray,uint256,bytes32) [INTERNAL]
```slithir
 _deallocate(result)
INTERNAL_CALL, DynamicArrayLib._deallocate(DynamicArrayLib.DynamicArray)(result_0)
 result = a
result_1(DynamicArrayLib.DynamicArray) := a_1(DynamicArrayLib.DynamicArray)
 mstore(uint256,uint256)(mload(uint256)(result) + 0x20 + i << 5,data)
TMP_12394(uint256) = SOLIDITY_CALL mload(uint256)(result_1)
TMP_12395(uint256) = TMP_12394 + 32
TMP_12396(uint256) = i_1 << 5
TMP_12397(uint256) = TMP_12395 + TMP_12396
TMP_12398(None) = SOLIDITY_CALL mstore(uint256,uint256)(TMP_12397,data_1)
 result
RETURN result_1
```
#### IGTL.addSubaccount(uint256) [EXTERNAL]
```slithir

```

#### EnumerableSetLib.add(EnumerableSetLib.Uint8Set,uint8) [INTERNAL]
```slithir
 result = sload(uint256)(set)
TMP_12669(uint256) = SOLIDITY_CALL sload(uint256)(set_1 (-> []))
result_1(bool) := TMP_12669(uint256)
 mask_add_asm_0 = 1 << 0xff & value
TMP_12670(uint256) = 255 & value_1
TMP_12671(uint256) = 1 << TMP_12670
mask_add_asm_0_1(uint256) := TMP_12671(uint256)
 sstore(uint256,uint256)(set,result | mask_add_asm_0)
TMP_12672(bool) = result_1 | mask_add_asm_0_1
TMP_12673(None) = SOLIDITY_CALL sstore(uint256,uint256)(set_1 (-> []),TMP_12672)
 result = ! result & mask_add_asm_0
TMP_12674(bool) = result_1 & mask_add_asm_0_1
TMP_12675 = UnaryType.BANG TMP_12674 
result_2(bool) := TMP_12675(bool)
 result
RETURN result_2
```
#### EnumerableSetLib.length(EnumerableSetLib.Uint8Set) [INTERNAL]
```slithir
 packed_length_asm_0 = sload(uint256)(set)
TMP_12489(uint256) = SOLIDITY_CALL sload(uint256)(set_1 (-> []))
packed_length_asm_0_1(uint256) := TMP_12489(uint256)
 packed_length_asm_0
result_1(uint256) := phi(['result_0', 'result_2'])
packed_length_asm_0_2(uint256) := phi(['packed_length_asm_0_3', 'packed_length_asm_0_1'])
CONDITION packed_length_asm_0_2
 packed_length_asm_0 = packed_length_asm_0 ^ packed_length_asm_0 & 1 + ~ packed_length_asm_0
TMP_12490 = UnaryType.TILD packed_length_asm_0_2 
TMP_12491(uint256) = 1 + TMP_12490
TMP_12492(uint256) = packed_length_asm_0_2 & TMP_12491
TMP_12493(uint256) = packed_length_asm_0_2 ^ TMP_12492
packed_length_asm_0_3(uint256) := TMP_12493(uint256)
 result = 1 + result
TMP_12494(uint256) = 1 + result_1
result_2(uint256) := TMP_12494(uint256)
 result
RETURN result_1
```
#### EnumerableSetLib.remove(EnumerableSetLib.Uint8Set,uint8) [INTERNAL]
```slithir
 result = sload(uint256)(set)
TMP_12809(uint256) = SOLIDITY_CALL sload(uint256)(set_1 (-> []))
result_1(bool) := TMP_12809(uint256)
 mask_remove_asm_0 = 1 << 0xff & value
TMP_12810(uint256) = 255 & value_1
TMP_12811(uint256) = 1 << TMP_12810
mask_remove_asm_0_1(uint256) := TMP_12811(uint256)
 sstore(uint256,uint256)(set,result & ~ mask_remove_asm_0)
TMP_12812 = UnaryType.TILD mask_remove_asm_0_1 
TMP_12813(bool) = result_1 & TMP_12812
TMP_12814(None) = SOLIDITY_CALL sstore(uint256,uint256)(set_1 (-> []),TMP_12813)
 result = ! ! result & mask_remove_asm_0
TMP_12815(bool) = result_1 & mask_remove_asm_0_1
TMP_12816 = UnaryType.BANG TMP_12815 
TMP_12817 = UnaryType.BANG TMP_12816 
result_2(bool) := TMP_12817(bool)
 result
RETURN result_2
```
#### MarketLib.setPosition(Market,address,uint256,Position) [INTERNAL]
```slithir
 self.position[account][subaccount] = position
REF_5018(mapping(address => mapping(uint256 => Position))) -> self_1 (-> []).position
REF_5019(mapping(uint256 => Position)) -> REF_5018[account_1]
REF_5020(Position) -> REF_5019[subaccount_1]
self_2 (-> [])(Market) := phi(['self_1 (-> [])'])
REF_5020(Position) (->self_2 (-> [])) := position_1(Position)
```
#### MarketLib._cancelReduceOnlyOrdersBackstop(Market,address,uint256) [PRIVATE]
```slithir
self_1 (-> [])(Market) := phi(['self_1 (-> [])'])
account_1(address) := phi(['account_1'])
subaccount_1(uint256) := phi(['subaccount_1'])
 orderIds = self.reduceOnlyOrdersBackstopBook[account][subaccount]
REF_5230(mapping(address => mapping(uint256 => uint256[]))) -> self_1 (-> []).reduceOnlyOrdersBackstopBook
REF_5231(mapping(uint256 => uint256[])) -> REF_5230[account_1]
REF_5232(uint256[]) -> REF_5231[subaccount_1]
orderIds_1(uint256[]) = ['REF_5232(uint256[])']
 orderIds.length == 0
REF_5233 -> LENGTH orderIds_1
TMP_9465(bool) = REF_5233 == 0
CONDITION TMP_9465
 CLOBLib.cancel(self.asset,account,subaccount,orderIds,BookType.BACKSTOP)
REF_5235(bytes32) -> self_1 (-> []).asset
REF_5236(BookType) -> BookType.BACKSTOP
TMP_9466(uint256) = LIBRARY_CALL, dest:CLOBLib, function:CLOBLib.cancel(bytes32,address,uint256,uint256[],BookType), arguments:['REF_5235', 'account_1', 'subaccount_1', 'orderIds_1', 'REF_5236'] 
 delete self.reduceOnlyOrdersBackstopBook[account][subaccount]
REF_5237(mapping(address => mapping(uint256 => uint256[]))) -> self_1 (-> []).reduceOnlyOrdersBackstopBook
REF_5238(mapping(uint256 => uint256[])) -> REF_5237[account_1]
REF_5239(uint256[]) -> REF_5238[subaccount_1]
REF_5238 = delete REF_5239
```
#### MarketLib._cancelReduceOnlyOrdersStandard(Market,address,uint256) [PRIVATE]
```slithir
self_1 (-> [])(Market) := phi(['self_1 (-> [])'])
account_1(address) := phi(['account_1'])
subaccount_1(uint256) := phi(['subaccount_1'])
 orderIds = self.reduceOnlyOrders[account][subaccount]
REF_5220(mapping(address => mapping(uint256 => uint256[]))) -> self_1 (-> []).reduceOnlyOrders
REF_5221(mapping(uint256 => uint256[])) -> REF_5220[account_1]
REF_5222(uint256[]) -> REF_5221[subaccount_1]
orderIds_1(uint256[]) = ['REF_5222(uint256[])']
 orderIds.length == 0
REF_5223 -> LENGTH orderIds_1
TMP_9463(bool) = REF_5223 == 0
CONDITION TMP_9463
 CLOBLib.cancel(self.asset,account,subaccount,orderIds,BookType.STANDARD)
REF_5225(bytes32) -> self_1 (-> []).asset
REF_5226(BookType) -> BookType.STANDARD
TMP_9464(uint256) = LIBRARY_CALL, dest:CLOBLib, function:CLOBLib.cancel(bytes32,address,uint256,uint256[],BookType), arguments:['REF_5225', 'account_1', 'subaccount_1', 'orderIds_1', 'REF_5226'] 
 delete self.reduceOnlyOrders[account][subaccount]
REF_5227(mapping(address => mapping(uint256 => uint256[]))) -> self_1 (-> []).reduceOnlyOrders
REF_5228(mapping(uint256 => uint256[])) -> REF_5227[account_1]
REF_5229(uint256[]) -> REF_5228[subaccount_1]
REF_5228 = delete REF_5229
```
#### StorageLib.loadMarketMetadata(bytes32) [INTERNAL]
```slithir
MARKET_METADATA_SLOT_1(bytes32) := phi(['MARKET_METADATA_SLOT_0'])
 slot = keccak256(bytes)(abi.encode(asset,MARKET_METADATA_SLOT))
TMP_9645(bytes) = SOLIDITY_CALL abi.encode()(asset_1,MARKET_METADATA_SLOT_1)
TMP_9646(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_9645)
slot_1(bytes32) := TMP_9646(bytes32)
 marketMetadata = slot
marketMetadata_1 (-> ['slot'])(MarketMetadata) := slot_1(bytes32)
 marketMetadata
RETURN marketMetadata_1 (-> ['slot'])
```
#### CollateralManagerLib.debitAccount(CollateralManager,address,uint256) [INTERNAL]
```slithir
 self.freeCollateral[account] < amount
REF_4834(mapping(address => uint256)) -> self_1 (-> []).freeCollateral
REF_4835(uint256) -> REF_4834[account_1]
TMP_9183(bool) = REF_4835 < amount_1
CONDITION TMP_9183
 revert InsufficientBalance()()
TMP_9184(None) = SOLIDITY_CALL revert InsufficientBalance()()
 self.freeCollateral[account] -= amount
REF_4836(mapping(address => uint256)) -> self_1 (-> []).freeCollateral
REF_4837(uint256) -> REF_4836[account_1]
self_2 (-> [])(CollateralManager) := phi(['self_1 (-> [])'])
REF_4837(-> self_2 (-> [])) = REF_4837 (c)- amount_1
```

#### ClearingHouseLib._getPositions(ClearingHouse,DynamicArrayLib.DynamicArray,address,uint256,bool) [INTERNAL]
```slithir
self_1 (-> [])(ClearingHouse) := phi(['self_1 (-> [])', 'self_1 (-> [])', 'self_1 (-> [])'])
assets_1(DynamicArrayLib.DynamicArray) := phi(['REF_4698', 'assets_1', 'REF_4439'])
account_1(address) := phi(['REF_4699', 'account_1', 'REF_4440'])
subaccount_1(uint256) := phi(['subaccount_1', 'REF_4441', 'REF_4700'])
newPosition_1(bool) := phi(['REF_4442', 'isNewPosition_1'])
 length = assets.length()
TMP_9109(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.length(DynamicArrayLib.DynamicArray), arguments:['assets_1'] 
length_1(uint256) := TMP_9109(uint256)
 length == 0
TMP_9110(bool) = length_1 == 0
CONDITION TMP_9110
 positions
RETURN positions_0
 positions = new Position[](length)
TMP_9112(Position[])  = new Position[](length_1)
positions_1(Position[]) = ['TMP_9112(Position[])']
 i < length - 1
i_1(uint256) := phi(['i_0', 'i_2'])
TMP_9113(uint256) = length_1 (c)- 1
TMP_9114(bool) = i_1 < TMP_9113
CONDITION TMP_9114
 positions[i] = self.market[assets.getBytes32(i)].getPosition(account,subaccount)
REF_4772(Position) -> positions_1[i_1]
REF_4773(mapping(bytes32 => Market)) -> self_1 (-> []).market
TMP_9115(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
REF_4775(Market) -> REF_4773[TMP_9115]
TMP_9116(Position) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getPosition(Market,address,uint256), arguments:['REF_4775', 'account_1', 'subaccount_1'] 
positions_5(Position[]) := phi(['positions_1'])
REF_4772(Position) (->positions_5) := TMP_9116(Position)
 ++ i
i_2(uint256) = i_1 (c)+ 1
 newPosition
CONDITION newPosition_1
 positions[length - 1].leverage = self.market[assets.getBytes32(length - 1)].getPositionLeverage(account,subaccount)
TMP_9117(uint256) = length_1 (c)- 1
REF_4777(Position) -> positions_1[TMP_9117]
REF_4778(uint256) -> REF_4777.leverage
REF_4779(mapping(bytes32 => Market)) -> self_1 (-> []).market
TMP_9118(uint256) = length_1 (c)- 1
TMP_9119(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'TMP_9118'] 
REF_4781(Market) -> REF_4779[TMP_9119]
TMP_9120(uint256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getPositionLeverage(Market,address,uint256), arguments:['REF_4781', 'account_1', 'subaccount_1'] 
positions_3(Position[]) := phi(['positions_1'])
REF_4778(uint256) (->positions_3) := TMP_9120(uint256)
 positions[length - 1] = self.market[assets.getBytes32(length - 1)].getPosition(account,subaccount)
TMP_9121(uint256) = length_1 (c)- 1
REF_4783(Position) -> positions_1[TMP_9121]
REF_4784(mapping(bytes32 => Market)) -> self_1 (-> []).market
TMP_9122(uint256) = length_1 (c)- 1
TMP_9123(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'TMP_9122'] 
REF_4786(Market) -> REF_4784[TMP_9123]
TMP_9124(Position) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getPosition(Market,address,uint256), arguments:['REF_4786', 'account_1', 'subaccount_1'] 
positions_2(Position[]) := phi(['positions_1'])
REF_4783(Position) (->positions_2) := TMP_9124(Position)
positions_4(Position[]) := phi(['positions_3', 'positions_2'])
 positions
RETURN positions_4
```
#### DynamicArrayLib.wrap(bytes32[]) [INTERNAL]
```slithir
 mstore(uint256,uint256)(result,a)
TMP_12127(None) = SOLIDITY_CALL mstore(uint256,uint256)(result_0,a_1)
 result
RETURN result_0
```
#### EnumerableSetLib.values(EnumerableSetLib.Uint8Set) [INTERNAL]
```slithir
 result = mload(uint256)(0x40)
TMP_12908(uint256) = SOLIDITY_CALL mload(uint256)(64)
result_1(uint8[]) = ['TMP_12908(uint256)']
 ptr_values_asm_0 = result + 0x20
TMP_12909(uint8[]) = result_1 + 32
ptr_values_asm_0_1(uint256) := TMP_12909(uint8[])
 o_values_asm_0 = 0
o_values_asm_0_1(uint256) := 0(uint256)
 packed_values_asm_0 = sload(uint256)(set)
TMP_12910(uint256) = SOLIDITY_CALL sload(uint256)(set_1 (-> []))
packed_values_asm_0_1(uint256) := TMP_12910(uint256)
 packed_values_asm_0
ptr_values_asm_0_2(uint256) := phi(['ptr_values_asm_0_1', 'ptr_values_asm_0_3'])
o_values_asm_0_2(uint256) := phi(['o_values_asm_0_1', 'o_values_asm_0_5'])
packed_values_asm_0_2(uint256) := phi(['packed_values_asm_0_1', 'packed_values_asm_0_5'])
CONDITION packed_values_asm_0_2
 ! packed_values_asm_0 & 0xffff
TMP_12911(uint256) = packed_values_asm_0_2 & 65535
TMP_12912 = UnaryType.BANG TMP_12911 
CONDITION TMP_12912
o_values_asm_0_4(uint256) := phi(['o_values_asm_0_1', 'o_values_asm_0_3'])
packed_values_asm_0_4(uint256) := phi(['packed_values_asm_0_3', 'packed_values_asm_0_1'])
 o_values_asm_0 = o_values_asm_0 + 16
TMP_12913(uint256) = o_values_asm_0_2 + 16
o_values_asm_0_3(uint256) := TMP_12913(uint256)
 packed_values_asm_0 = packed_values_asm_0 >> 16
TMP_12914(uint256) = packed_values_asm_0_2 >> 16
packed_values_asm_0_3(uint256) := TMP_12914(uint256)
 mstore(uint256,uint256)(ptr_values_asm_0,o_values_asm_0)
TMP_12915(None) = SOLIDITY_CALL mstore(uint256,uint256)(ptr_values_asm_0_2,o_values_asm_0_4)
 ptr_values_asm_0 = ptr_values_asm_0 + packed_values_asm_0 & 1 << 5
TMP_12916(uint256) = packed_values_asm_0_4 & 1
TMP_12917(uint256) = TMP_12916 << 5
TMP_12918(uint256) = ptr_values_asm_0_2 + TMP_12917
ptr_values_asm_0_3(uint256) := TMP_12918(uint256)
 o_values_asm_0 = o_values_asm_0 + 1
TMP_12919(uint256) = o_values_asm_0_4 + 1
o_values_asm_0_5(uint256) := TMP_12919(uint256)
 packed_values_asm_0 = packed_values_asm_0 >> 1
TMP_12920(uint256) = packed_values_asm_0_4 >> 1
packed_values_asm_0_5(uint256) := TMP_12920(uint256)
 mstore(uint256,uint256)(result,ptr_values_asm_0 - result + 0x20 >> 5)
TMP_12921(uint8[]) = result_1 + 32
TMP_12922(uint256) = ptr_values_asm_0_2 - TMP_12921
TMP_12923(uint256) = TMP_12922 >> 5
TMP_12924(None) = SOLIDITY_CALL mstore(uint256,uint256)(result_1,TMP_12923)
 mstore(uint256,uint256)(0x40,ptr_values_asm_0)
TMP_12925(None) = SOLIDITY_CALL mstore(uint256,uint256)(64,ptr_values_asm_0_2)
 result
RETURN result_1
```
#### FundingLib.getCumulativeFunding(FundingRateEngine) [INTERNAL]
```slithir
 self.cumulativeFundingIndex
REF_4899(int256) -> self_1 (-> []).cumulativeFundingIndex
RETURN REF_4899
```
#### PositionLib.realizeFundingPayment(Position,int256) [INTERNAL]
```slithir
 self.lastCumulativeFunding == cumulativeFunding
REF_5327(int256) -> self_1.lastCumulativeFunding
TMP_9551(bool) = REF_5327 == cumulativeFunding_1
CONDITION TMP_9551
 0
RETURN 0
 self.lastCumulativeFunding = cumulativeFunding
REF_5328(int256) -> self_1.lastCumulativeFunding
self_2(Position) := phi(['self_1'])
REF_5328(int256) (->self_2) := cumulativeFunding_1(int256)
 self.isLong
REF_5329(bool) -> self_1.isLong
CONDITION REF_5329
 fundingPayment = _getFundingPayment({amount:self.amount.toInt256(),lastCumulativePremiumFunding:self.lastCumulativeFunding,cumulativePremiumFunding:cumulativeFunding})
REF_5330(uint256) -> self_1.amount
TMP_9552(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_5330'] 
REF_5332(int256) -> self_1.lastCumulativeFunding
TMP_9553(int256) = INTERNAL_CALL, PositionLib._getFundingPayment(int256,int256,int256)(TMP_9552,REF_5332,cumulativeFunding_1)
fundingPayment_2(int256) := TMP_9553(int256)
 fundingPayment = _getFundingPayment({amount:- self.amount.toInt256(),lastCumulativePremiumFunding:self.lastCumulativeFunding,cumulativePremiumFunding:cumulativeFunding})
REF_5333(uint256) -> self_1.amount
TMP_9554(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_5333'] 
TMP_9555(int256) = 0 (c)- TMP_9554
REF_5335(int256) -> self_1.lastCumulativeFunding
TMP_9556(int256) = INTERNAL_CALL, PositionLib._getFundingPayment(int256,int256,int256)(TMP_9555,REF_5335,cumulativeFunding_1)
fundingPayment_1(int256) := TMP_9556(int256)
fundingPayment_3(int256) := phi(['fundingPayment_1', 'fundingPayment_2'])
 fundingPayment
RETURN fundingPayment_3
```
#### StorageLib.loadFundingRateEngine(bytes32) [INTERNAL]
```slithir
FUNDING_RATE_ENGINE_SLOT_1(bytes32) := phi(['FUNDING_RATE_ENGINE_SLOT_0'])
 slot = keccak256(bytes)(abi.encode(asset,FUNDING_RATE_ENGINE_SLOT))
TMP_9647(bytes) = SOLIDITY_CALL abi.encode()(asset_1,FUNDING_RATE_ENGINE_SLOT_1)
TMP_9648(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_9647)
slot_1(bytes32) := TMP_9648(bytes32)
 fundingRateEngine = slot
fundingRateEngine_1 (-> ['slot'])(FundingRateEngine) := slot_1(bytes32)
 fundingRateEngine
RETURN fundingRateEngine_1 (-> ['slot'])
```
#### PackedFeeRatesLib.getFeeAt(PackedFeeRates,uint256) [INTERNAL]
```slithir
U16_PER_WORD_2(uint256) := phi(['U16_PER_WORD_0'])
 index >= 15
TMP_1890(bool) = index_1 >= 15
CONDITION TMP_1890
 revert FeeTierIndexOutOfBounds()()
TMP_1891(None) = SOLIDITY_CALL revert FeeTierIndexOutOfBounds()()
 shiftBits = index * U16_PER_WORD
TMP_1892(uint256) = index_1 (c)* U16_PER_WORD_2
shiftBits_1(uint256) := TMP_1892(uint256)
 uint16((PackedFeeRates.unwrap(fees) >> shiftBits) & 0xFFFF)
TMP_1893 = CONVERT fees_1 to uint256
TMP_1894(uint256) = TMP_1893 >> shiftBits_1
TMP_1895(uint256) = TMP_1894 & 65535
TMP_1896 = CONVERT TMP_1895 to uint16
RETURN TMP_1896
```
#### BookLib.assertPriceInBounds(Book,uint256) [INTERNAL]
```slithir
 price % StorageLib.loadBookSettings(self.config.asset).tickSize != 0
REF_3706(BookConfig) -> self_1 (-> []).config
REF_3707(bytes32) -> REF_3706.asset
TMP_8503(BookSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadBookSettings(bytes32), arguments:['REF_3707'] 
REF_3708(uint256) -> TMP_8503.tickSize
TMP_8504(uint256) = price_1 % REF_3708
TMP_8505(bool) = TMP_8504 != 0
CONDITION TMP_8505
 revert OrderPriceOutOfBounds()()
TMP_8506(None) = SOLIDITY_CALL revert OrderPriceOutOfBounds()()
```
#### BookLib.toOrderId(Book,address,uint96) [INTERNAL]
```slithir
 clientOrderId == 0
TMP_8578(bool) = clientOrderId_1 == 0
CONDITION TMP_8578
 self.incrementOrderId()
TMP_8579(uint256) = LIBRARY_CALL, dest:BookLib, function:BookLib.incrementOrderId(Book), arguments:['self_1 (-> [])'] 
RETURN TMP_8579
 orderId = OrderIdLib.getOrderId(account,clientOrderId)
TMP_8580(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.getOrderId(address,uint96), arguments:['account_1', 'clientOrderId_1'] 
orderId_1(uint256) := TMP_8580(uint256)
 self.assertUnusedOrderId(orderId)
LIBRARY_CALL, dest:BookLib, function:BookLib.assertUnusedOrderId(Book,uint256), arguments:['self_1 (-> [])', 'orderId_1'] 
 orderId
RETURN orderId_1
```
#### CLOBLib._emitOrderProcessed(address,PlaceOrderArgs,PlaceOrderResult,BookType) [INTERNAL]
```slithir
account_1(address) := phi(['account_1'])
args_1(PlaceOrderArgs) := phi(['args_1'])
result_1(PlaceOrderResult) := phi(['result_3'])
bookType_1(BookType) := phi(['bookType_1'])
 OrderProcessed({asset:args.asset,account:account,subaccount:args.subaccount,orderId:result.orderId,amountSubmitted:args.amount,baseDenominated:args.baseDenominated,tif:args.tif,expiryTime:args.expiryTime,limitPrice:args.limitPrice,side:args.side,reduceOnly:args.reduceOnly,basePosted:result.basePosted,quoteTraded:result.quoteTraded,baseTraded:result.baseTraded,bookType:bookType,nonce:StorageLib.incNonce()})
REF_4380(bytes32) -> args_1.asset
REF_4381(uint256) -> args_1.subaccount
REF_4382(uint256) -> result_1.orderId
REF_4383(uint256) -> args_1.amount
REF_4384(bool) -> args_1.baseDenominated
REF_4385(TiF) -> args_1.tif
REF_4386(uint32) -> args_1.expiryTime
REF_4387(uint256) -> args_1.limitPrice
REF_4388(Side) -> args_1.side
REF_4389(bool) -> args_1.reduceOnly
REF_4390(uint256) -> result_1.basePosted
REF_4391(uint256) -> result_1.quoteTraded
REF_4392(uint256) -> result_1.baseTraded
TMP_8909(uint256) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.incNonce(), arguments:[] 
Emit OrderProcessed(REF_4380,account_1,REF_4381,REF_4382,REF_4383,REF_4384,REF_4385,REF_4386,REF_4387,REF_4388,REF_4389,REF_4390,REF_4391,REF_4392,bookType_1,TMP_8909)
```
#### CLOBLib._getStorage(bytes32,BookType) [INTERNAL]
```slithir
asset_1(bytes32) := phi(['REF_4000', 'asset_1', 'asset_1', 'REF_4024'])
bookType_1(BookType) := phi(['bookType_1', 'REF_3992', 'bookType_1', 'REF_3975', 'bookType_1'])
 StorageLib.loadBook(asset,bookType)
TMP_8924(Book) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadBook(bytes32,BookType), arguments:['asset_1', 'bookType_1'] 
RETURN TMP_8924
```
#### CLOBLib._processBuyOrder(Book,Order,PlaceOrderArgs) [INTERNAL]
```slithir
ds_1 (-> ['TMP_8669'])(Book) := phi(["ds_1 (-> ['TMP_8669'])"])
newOrder_1(Order) := phi(['newOrder_1'])
args_1(PlaceOrderArgs) := phi(['args_1'])
 result.orderId = newOrder.id.unwrap()
REF_4045(uint256) -> result_0.orderId
REF_4046(OrderId) -> newOrder_1.id
TMP_8712(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_4046'] 
result_1(PlaceOrderResult) := phi(['result_0'])
REF_4045(uint256) (->result_1) := TMP_8712(uint256)
 (result.quoteTraded,result.baseTraded) = _executeBuyOrder(ds,newOrder,args.tif,args.baseDenominated)
REF_4048(uint256) -> result_1.quoteTraded
REF_4049(uint256) -> result_1.baseTraded
REF_4050(TiF) -> args_1.tif
REF_4051(bool) -> args_1.baseDenominated
TUPLE_83(uint256,uint256) = INTERNAL_CALL, CLOBLib._executeBuyOrder(Book,Order,TiF,bool)(ds_1 (-> ['TMP_8669']),newOrder_1,REF_4050,REF_4051)
REF_4048(uint256)= UNPACK TUPLE_83 index: 0 
REF_4049(uint256)= UNPACK TUPLE_83 index: 1 
 uint8(args.tif) <= 1
REF_4052(TiF) -> args_1.tif
TMP_8713 = CONVERT REF_4052 to uint8
TMP_8714(bool) = TMP_8713 <= 1
CONDITION TMP_8714
 result.basePosted = newOrder.amount
REF_4053(uint256) -> result_1.basePosted
REF_4054(uint256) -> newOrder_1.amount
result_2(PlaceOrderResult) := phi(['result_1'])
REF_4053(uint256) (->result_2) := REF_4054(uint256)
result_3(PlaceOrderResult) := phi(['result_2', 'result_1'])
 result
RETURN result_3
```
#### CLOBLib._processSellOrder(Book,Order,PlaceOrderArgs) [INTERNAL]
```slithir
ds_1 (-> ['TMP_8669'])(Book) := phi(["ds_1 (-> ['TMP_8669'])"])
newOrder_1(Order) := phi(['newOrder_1'])
args_1(PlaceOrderArgs) := phi(['args_1'])
 result.orderId = newOrder.id.unwrap()
REF_4055(uint256) -> result_0.orderId
REF_4056(OrderId) -> newOrder_1.id
TMP_8715(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_4056'] 
result_1(PlaceOrderResult) := phi(['result_0'])
REF_4055(uint256) (->result_1) := TMP_8715(uint256)
 (result.quoteTraded,result.baseTraded) = _executeSellOrder(ds,newOrder,args.tif,args.baseDenominated)
REF_4058(uint256) -> result_1.quoteTraded
REF_4059(uint256) -> result_1.baseTraded
REF_4060(TiF) -> args_1.tif
REF_4061(bool) -> args_1.baseDenominated
TUPLE_84(uint256,uint256) = INTERNAL_CALL, CLOBLib._executeSellOrder(Book,Order,TiF,bool)(ds_1 (-> ['TMP_8669']),newOrder_1,REF_4060,REF_4061)
REF_4058(uint256)= UNPACK TUPLE_84 index: 0 
REF_4059(uint256)= UNPACK TUPLE_84 index: 1 
 uint8(args.tif) <= 1
REF_4062(TiF) -> args_1.tif
TMP_8716 = CONVERT REF_4062 to uint8
TMP_8717(bool) = TMP_8716 <= 1
CONDITION TMP_8717
 result.basePosted = newOrder.amount
REF_4063(uint256) -> result_1.basePosted
REF_4064(uint256) -> newOrder_1.amount
result_2(PlaceOrderResult) := phi(['result_1'])
REF_4063(uint256) (->result_2) := REF_4064(uint256)
result_3(PlaceOrderResult) := phi(['result_2', 'result_1'])
 result
RETURN result_3
```
#### CLOBLib._updateOrderbookNotional(bytes32,address,uint256,int256) [PRIVATE]
```slithir
asset_1(bytes32) := phi(['REF_4018', 'REF_4038', 'asset_1', 'asset_1', 'REF_4230'])
account_1(address) := phi(['account_1', 'account_1', 'owner_1', 'matchedOwner_1', 'account_1'])
subaccount_1(uint256) := phi(['REF_4231', 'REF_4019', 'subaccount_1', 'REF_4039', 'subaccount_1'])
amount_1(int256) := phi(['TMP_8693', 'TMP_8846', 'TMP_8776', 'TMP_8829', 'notionalDelta_1'])
 StorageLib.loadMarket(asset).updateOrderbookNotional(account,subaccount,amount)
TMP_8911(Market) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarket(bytes32), arguments:['asset_1'] 
LIBRARY_CALL, dest:MarketLib, function:MarketLib.updateOrderbookNotional(Market,address,uint256,int256), arguments:['TMP_8911', 'account_1', 'subaccount_1', 'amount_1']
```
#### OrderLib.isExpired(uint256) [INTERNAL]
```slithir
 expiryTime != NULL_TIMESTAMP && expiryTime < block.timestamp
TMP_9513(bool) = expiryTime_1 != NULL_TIMESTAMP
TMP_9514(bool) = expiryTime_1 < block.timestamp
TMP_9515(bool) = TMP_9513 && TMP_9514
RETURN TMP_9515
```
#### OrderLib.toOrder(AmendLimitOrderArgs,Order) [INTERNAL]
```slithir
 newOrder.owner = currentOrder.owner
REF_5289(address) -> newOrder_0.owner
REF_5290(address) -> currentOrder_1 (-> []).owner
newOrder_1(Order) := phi(['newOrder_0'])
REF_5289(address) (->newOrder_1) := REF_5290(address)
 newOrder.id = currentOrder.id
REF_5291(OrderId) -> newOrder_1.id
REF_5292(OrderId) -> currentOrder_1 (-> []).id
newOrder_2(Order) := phi(['newOrder_1'])
REF_5291(OrderId) (->newOrder_2) := REF_5292(OrderId)
 newOrder.side = args.side
REF_5293(Side) -> newOrder_2.side
REF_5294(Side) -> args_1.side
newOrder_3(Order) := phi(['newOrder_2'])
REF_5293(Side) (->newOrder_3) := REF_5294(Side)
 newOrder.price = args.price
REF_5295(uint256) -> newOrder_3.price
REF_5296(uint256) -> args_1.price
newOrder_4(Order) := phi(['newOrder_3'])
REF_5295(uint256) (->newOrder_4) := REF_5296(uint256)
 newOrder.amount = args.baseAmount
REF_5297(uint256) -> newOrder_4.amount
REF_5298(uint256) -> args_1.baseAmount
newOrder_5(Order) := phi(['newOrder_4'])
REF_5297(uint256) (->newOrder_5) := REF_5298(uint256)
 newOrder.reduceOnly = args.reduceOnly
REF_5299(bool) -> newOrder_5.reduceOnly
REF_5300(bool) -> args_1.reduceOnly
newOrder_6(Order) := phi(['newOrder_5'])
REF_5299(bool) (->newOrder_6) := REF_5300(bool)
 newOrder.subaccount = currentOrder.subaccount
REF_5301(uint256) -> newOrder_6.subaccount
REF_5302(uint256) -> currentOrder_1 (-> []).subaccount
newOrder_7(Order) := phi(['newOrder_6'])
REF_5301(uint256) (->newOrder_7) := REF_5302(uint256)
 newOrder.expiryTime = args.expiryTime
REF_5303(uint32) -> newOrder_7.expiryTime
REF_5304(uint32) -> args_1.expiryTime
newOrder_8(Order) := phi(['newOrder_7'])
REF_5303(uint32) (->newOrder_8) := REF_5304(uint32)
 newOrder
RETURN newOrder_8
```
#### ClearingHouseLib.getFundingPayment(ClearingHouse,address,uint256) [INTERNAL]
```slithir
 assets = self.assets[account][subaccount].values()
REF_4627(mapping(address => mapping(uint256 => EnumerableSetLib.Bytes32Set))) -> self_1 (-> []).assets
REF_4628(mapping(uint256 => EnumerableSetLib.Bytes32Set)) -> REF_4627[account_1]
REF_4629(EnumerableSetLib.Bytes32Set) -> REF_4628[subaccount_1]
TMP_9040(bytes32[]) = LIBRARY_CALL, dest:EnumerableSetLib, function:EnumerableSetLib.values(EnumerableSetLib.Bytes32Set), arguments:['REF_4629'] 
assets_1(bytes32[]) = ['TMP_9040(bytes32[])']
 i < assets.length
fundingPayment_1(int256) := phi(['fundingPayment_0', 'fundingPayment_2'])
i_1(uint256) := phi(['i_2', 'i_0'])
REF_4631 -> LENGTH assets_1
TMP_9041(bool) = i_1 < REF_4631
CONDITION TMP_9041
 fundingPayment += self.market[assets[i]].getFundingPayment(account,subaccount)
REF_4632(mapping(bytes32 => Market)) -> self_1 (-> []).market
REF_4633(bytes32) -> assets_1[i_1]
REF_4634(Market) -> REF_4632[REF_4633]
TMP_9042(int256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getFundingPayment(Market,address,uint256), arguments:['REF_4634', 'account_1', 'subaccount_1'] 
fundingPayment_2(int256) = fundingPayment_1 (c)+ TMP_9042
 ++ i
i_2(uint256) = i_1 (c)+ 1
 fundingPayment
RETURN fundingPayment_1
```
#### MarketLib.getUpnlAndMinMargin(Market,Position,BookType) [INTERNAL]
```slithir
 position.amount == 0
REF_5045(uint256) -> position_1.amount
TMP_9342(bool) = REF_5045 == 0
CONDITION TMP_9342
 (0,0)
RETURN 0,0
 currentNotional = position.amount.fullMulDiv(self.markPrice,1e18)
REF_5046(uint256) -> position_1.amount
REF_5048(uint256) -> self_1 (-> []).markPrice
TMP_9343(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_5046', 'REF_5048', '1000000000000000000'] 
currentNotional_1(uint256) := TMP_9343(uint256)
 upnl = _calcUpnl(position.isLong,position.openNotional,currentNotional)
REF_5049(bool) -> position_1.isLong
REF_5050(uint256) -> position_1.openNotional
TMP_9344(int256) = INTERNAL_CALL, MarketLib._calcUpnl(bool,uint256,uint256)(REF_5049,REF_5050,currentNotional_1)
upnl_1(int256) := TMP_9344(int256)
 minMargin = currentNotional.fullMulDiv(self.getMinMarginRatio(bookType),1e18)
TMP_9345(uint256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getMinMarginRatio(Market,BookType), arguments:['self_1 (-> [])', 'bookType_1'] 
TMP_9346(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['currentNotional_1', 'TMP_9345', '1000000000000000000'] 
minMargin_1(uint256) := TMP_9346(uint256)
 (upnl,minMargin)
RETURN upnl_1,minMargin_1
```
#### MarketLib._calcUpnl(bool,uint256,uint256) [PRIVATE]
```slithir
isLong_1(bool) := phi(['REF_5043', 'REF_5049', 'REF_5072', 'REF_5079'])
openNotional_1(uint256) := phi(['REF_5044', 'REF_5050', 'REF_5073', 'REF_5080'])
currentNotional_1(uint256) := phi(['currentNotional_1', 'currentNotional_1', 'currentNotional_1', 'currentNotional_1'])
 isLong
CONDITION isLong_1
 upnl = currentNotional.toInt256() - openNotional.toInt256()
TMP_9467(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['currentNotional_1'] 
TMP_9468(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['openNotional_1'] 
TMP_9469(int256) = TMP_9467 (c)- TMP_9468
upnl_2(int256) := TMP_9469(int256)
 upnl = openNotional.toInt256() - currentNotional.toInt256()
TMP_9470(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['openNotional_1'] 
TMP_9471(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['currentNotional_1'] 
TMP_9472(int256) = TMP_9470 (c)- TMP_9471
upnl_1(int256) := TMP_9472(int256)
upnl_3(int256) := phi(['upnl_1', 'upnl_2'])
 upnl
RETURN upnl_3
```
