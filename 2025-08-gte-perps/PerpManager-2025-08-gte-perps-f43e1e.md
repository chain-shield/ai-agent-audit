


















#### PerpManager._getCollateral(uint256,uint256,uint256) [PRIVATE]
```slithir
 collateral = baseAmount.fullMulDiv(price,1e18).fullMulDiv(1e18,leverage)
TMP_6048(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['baseAmount_1', 'price_1', '1000000000000000000'] 
TMP_6049(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['TMP_6048', '1000000000000000000', 'leverage_1'] 
collateral_1(uint256) := TMP_6049(uint256)
 collateral
RETURN collateral_1
```
#### PerpManager.addMargin(address,uint256,uint256) [EXTERNAL]
```slithir
 clearingHouse = StorageLib.loadClearingHouse()
TMP_5950(ClearingHouse) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadClearingHouse(), arguments:[] 
clearingHouse_1 (-> ['TMP_5950'])(ClearingHouse) := TMP_5950(ClearingHouse)
 (cache.assets,cache.positions) = clearingHouse.getAccount(account,subaccount)
REF_2513(DynamicArrayLib.DynamicArray) -> cache_0.assets
REF_2514(Position[]) -> cache_0.positions
TUPLE_57(DynamicArrayLib.DynamicArray,Position[]) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.getAccount(ClearingHouse,address,uint256), arguments:["clearingHouse_1 (-> ['TMP_5950'])", 'account_1', 'subaccount_1'] 
REF_2513(DynamicArrayLib.DynamicArray)= UNPACK TUPLE_57 index: 0 
REF_2514(Position[])= UNPACK TUPLE_57 index: 1 
 amount == 0
TMP_5951(bool) = amount_1 == 0
CONDITION TMP_5951
 revert InvalidDeposit()()
TMP_5952(None) = SOLIDITY_CALL revert InvalidDeposit()()
 cache.positions.length == 0
REF_2516(Position[]) -> cache_0.positions
REF_2517 -> LENGTH REF_2516
TMP_5953(bool) = REF_2517 == 0
CONDITION TMP_5953
 revert InvalidDeposit()()
TMP_5954(None) = SOLIDITY_CALL revert InvalidDeposit()()
 cache.fundingPayment = ClearingHouseLib.realizeFundingPayment(cache.assets,cache.positions)
REF_2518(int256) -> cache_0.fundingPayment
REF_2520(DynamicArrayLib.DynamicArray) -> cache_0.assets
REF_2521(Position[]) -> cache_0.positions
TMP_5955(int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.realizeFundingPayment(DynamicArrayLib.DynamicArray,Position[]), arguments:['REF_2520', 'REF_2521'] 
cache_1(PerpManager.__MarginUpdateCache__) := phi(['cache_0'])
REF_2518(int256) (->cache_1) := TMP_5955(int256)
 remainingMargin = StorageLib.loadCollateralManager().settleMarginUpdate({account:account,subaccount:subaccount,marginDelta:amount.toInt256(),fundingPayment:cache.fundingPayment})
TMP_5956(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
TMP_5957(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['amount_1'] 
REF_2525(int256) -> cache_1.fundingPayment
TMP_5958(int256) = LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.settleMarginUpdate(CollateralManager,address,uint256,int256,int256), arguments:['TMP_5956', 'account_1', 'subaccount_1', 'TMP_5957', 'REF_2525'] 
remainingMargin_1(int256) := TMP_5958(int256)
 clearingHouse.assertNotLiquidatable({assets:cache.assets,positions:cache.positions,margin:remainingMargin})
REF_2527(DynamicArrayLib.DynamicArray) -> cache_1.assets
REF_2528(Position[]) -> cache_1.positions
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.assertNotLiquidatable(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256), arguments:["clearingHouse_1 (-> ['TMP_5950'])", 'REF_2527', 'REF_2528', 'remainingMargin_1'] 
 clearingHouse.setPositions({tradedAsset:,account:account,subaccount:subaccount,assets:cache.assets,positions:cache.positions})
REF_2530(DynamicArrayLib.DynamicArray) -> cache_1.assets
REF_2531(Position[]) -> cache_1.positions
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.setPositions(ClearingHouse,bytes32,address,uint256,DynamicArrayLib.DynamicArray,Position[]), arguments:["clearingHouse_1 (-> ['TMP_5950'])", '', 'account_1', 'subaccount_1', 'REF_2530', 'REF_2531'] 
 MarginAdded(account,subaccount,amount,remainingMargin,StorageLib.incNonce())
TMP_5961(uint256) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.incNonce(), arguments:[] 
Emit MarginAdded(account_1,subaccount_1,amount_1,remainingMargin_1,TMP_5961)
 onlySenderOrOperator(account,PerpsOperatorRoles.DEPOSIT_MARGIN)
REF_2533(PerpsOperatorRoles) -> PerpsOperatorRoles.DEPOSIT_MARGIN
MODIFIER_CALL, PerpManager.onlySenderOrOperator(address,PerpsOperatorRoles)(account_1,REF_2533)
```
#### PerpManager.amendLimitOrder(address,AmendLimitOrderArgs) [EXTERNAL]
```slithir
 clearingHouse = StorageLib.loadClearingHouse()
TMP_6017(ClearingHouse) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadClearingHouse(), arguments:[] 
clearingHouse_1 (-> ['TMP_6017'])(ClearingHouse) := TMP_6017(ClearingHouse)
 collateralDelta = clearingHouse.market[args.asset].amendLimitOrder(account,args,BookType.STANDARD)
REF_2635(mapping(bytes32 => Market)) -> clearingHouse_1 (-> ['TMP_6017']).market
REF_2636(bytes32) -> args_1.asset
REF_2637(Market) -> REF_2635[REF_2636]
REF_2639(BookType) -> BookType.STANDARD
TMP_6018(int256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.amendLimitOrder(Market,address,AmendLimitOrderArgs,BookType), arguments:['REF_2637', 'account_1', 'args_1', 'REF_2639'] 
collateralDelta_1(int256) := TMP_6018(int256)
 StorageLib.loadCollateralManager().handleCollateralDelta({account:account,collateralDelta:collateralDelta})
TMP_6019(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.handleCollateralDelta(CollateralManager,address,int256), arguments:['TMP_6019', 'account_1', 'collateralDelta_1'] 
 onlySenderOrOperator(account,PerpsOperatorRoles.PLACE_ORDER)
REF_2642(PerpsOperatorRoles) -> PerpsOperatorRoles.PLACE_ORDER
MODIFIER_CALL, PerpManager.onlySenderOrOperator(address,PerpsOperatorRoles)(account_1,REF_2642)
 onlyActiveProtocol()
MODIFIER_CALL, PerpManager.onlyActiveProtocol()()
 collateralDelta
RETURN collateralDelta_1
```
#### PerpManager.amendLimitOrderBackstop(address,AmendLimitOrderArgs) [EXTERNAL]
```slithir
 clearingHouse = StorageLib.loadClearingHouse()
TMP_6030(ClearingHouse) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadClearingHouse(), arguments:[] 
clearingHouse_1 (-> ['TMP_6030'])(ClearingHouse) := TMP_6030(ClearingHouse)
 collateralDelta = clearingHouse.market[args.asset].amendLimitOrder(account,args,BookType.BACKSTOP)
REF_2650(mapping(bytes32 => Market)) -> clearingHouse_1 (-> ['TMP_6030']).market
REF_2651(bytes32) -> args_1.asset
REF_2652(Market) -> REF_2650[REF_2651]
REF_2654(BookType) -> BookType.BACKSTOP
TMP_6031(int256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.amendLimitOrder(Market,address,AmendLimitOrderArgs,BookType), arguments:['REF_2652', 'account_1', 'args_1', 'REF_2654'] 
collateralDelta_1(int256) := TMP_6031(int256)
 StorageLib.loadCollateralManager().handleCollateralDelta({account:account,collateralDelta:collateralDelta})
TMP_6032(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.handleCollateralDelta(CollateralManager,address,int256), arguments:['TMP_6032', 'account_1', 'collateralDelta_1'] 
 onlySenderOrOperator(account,PerpsOperatorRoles.PLACE_ORDER)
REF_2657(PerpsOperatorRoles) -> PerpsOperatorRoles.PLACE_ORDER
MODIFIER_CALL, PerpManager.onlySenderOrOperator(address,PerpsOperatorRoles)(account_1,REF_2657)
 onlyActiveProtocol()
MODIFIER_CALL, PerpManager.onlyActiveProtocol()()
 collateralDelta
RETURN collateralDelta_1
```
#### PerpManager.cancelConditionalOrders(address,uint256[]) [EXTERNAL]
```slithir
 clearingHouse = StorageLib.loadClearingHouse()
TMP_6043(ClearingHouse) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadClearingHouse(), arguments:[] 
clearingHouse_1 (-> ['TMP_6043'])(ClearingHouse) := TMP_6043(ClearingHouse)
 i < nonces.length
clearingHouse_2 (-> ['TMP_6043'])(ClearingHouse) := phi(["clearingHouse_3 (-> ['TMP_6043'])", "clearingHouse_1 (-> ['TMP_6043'])"])
i_1(uint256) := phi(['i_2', 'i_0'])
REF_2665 -> LENGTH nonces_1
TMP_6044(bool) = i_1 < REF_2665
CONDITION TMP_6044
 clearingHouse.nonceUsed[account][nonces[i]] = true
REF_2666(mapping(address => mapping(uint256 => bool))) -> clearingHouse_2 (-> ['TMP_6043']).nonceUsed
REF_2667(mapping(uint256 => bool)) -> REF_2666[account_1]
REF_2668(uint256) -> nonces_1[i_1]
REF_2669(bool) -> REF_2667[REF_2668]
clearingHouse_3 (-> ['TMP_6043'])(ClearingHouse) := phi(["clearingHouse_2 (-> ['TMP_6043'])"])
REF_2669(bool) (->clearingHouse_3 (-> ['TMP_6043'])) := True(bool)
TMP_6043(ClearingHouse) := phi(["clearingHouse_3 (-> ['TMP_6043'])"])
 i ++
TMP_6045(uint256) := i_1(uint256)
i_2(uint256) = i_1 (c)+ 1
 onlySenderOrOperator(account,PerpsOperatorRoles.PLACE_ORDER)
REF_2670(PerpsOperatorRoles) -> PerpsOperatorRoles.PLACE_ORDER
MODIFIER_CALL, PerpManager.onlySenderOrOperator(address,PerpsOperatorRoles)(account_1,REF_2670)
 onlyActiveProtocol()
MODIFIER_CALL, PerpManager.onlyActiveProtocol()()
```
#### PerpManager.cancelLimitOrders(bytes32,address,uint256,uint256[]) [EXTERNAL]
```slithir
 refund = CLOBLib.cancel(asset,account,subaccount,orderIds,BookType.STANDARD)
REF_2644(BookType) -> BookType.STANDARD
TMP_6023(uint256) = LIBRARY_CALL, dest:CLOBLib, function:CLOBLib.cancel(bytes32,address,uint256,uint256[],BookType), arguments:['asset_1', 'account_1', 'subaccount_1', 'orderIds_1', 'REF_2644'] 
refund_1(uint256) := TMP_6023(uint256)
 StorageLib.loadCollateralManager().handleCollateralDelta({account:account,collateralDelta:- refund.toInt256()})
TMP_6024(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
TMP_6025(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['refund_1'] 
TMP_6026(int256) = 0 (c)- TMP_6025
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.handleCollateralDelta(CollateralManager,address,int256), arguments:['TMP_6024', 'account_1', 'TMP_6026'] 
 onlySenderOrOperator(account,PerpsOperatorRoles.PLACE_ORDER)
REF_2648(PerpsOperatorRoles) -> PerpsOperatorRoles.PLACE_ORDER
MODIFIER_CALL, PerpManager.onlySenderOrOperator(address,PerpsOperatorRoles)(account_1,REF_2648)
 onlyActiveProtocol()
MODIFIER_CALL, PerpManager.onlyActiveProtocol()()
 refund
RETURN refund_1
```
#### PerpManager.cancelLimitOrdersBackstop(bytes32,address,uint256,uint256[]) [EXTERNAL]
```slithir
 refund = CLOBLib.cancel(asset,account,subaccount,orderIds,BookType.BACKSTOP)
REF_2659(BookType) -> BookType.BACKSTOP
TMP_6036(uint256) = LIBRARY_CALL, dest:CLOBLib, function:CLOBLib.cancel(bytes32,address,uint256,uint256[],BookType), arguments:['asset_1', 'account_1', 'subaccount_1', 'orderIds_1', 'REF_2659'] 
refund_1(uint256) := TMP_6036(uint256)
 StorageLib.loadCollateralManager().handleCollateralDelta({account:account,collateralDelta:- refund.toInt256()})
TMP_6037(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
TMP_6038(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['refund_1'] 
TMP_6039(int256) = 0 (c)- TMP_6038
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.handleCollateralDelta(CollateralManager,address,int256), arguments:['TMP_6037', 'account_1', 'TMP_6039'] 
 onlySenderOrOperator(account,PerpsOperatorRoles.PLACE_ORDER)
REF_2663(PerpsOperatorRoles) -> PerpsOperatorRoles.PLACE_ORDER
MODIFIER_CALL, PerpManager.onlySenderOrOperator(address,PerpsOperatorRoles)(account_1,REF_2663)
 onlyActiveProtocol()
MODIFIER_CALL, PerpManager.onlyActiveProtocol()()
 refund
RETURN refund_1
```
#### PerpManager.constructor(address,address) [PUBLIC]
```slithir
 accountManager = IAccountManager(_accountManager)
TMP_5929 = CONVERT _accountManager_1 to IAccountManager
accountManager_1(IAccountManager) := TMP_5929(IAccountManager)
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
 OperatorPanel(_operatorHub)
INTERNAL_CALL, OperatorPanel.constructor(address)(_operatorHub_1)
```
#### PerpManager.deposit(address,uint256) [EXTERNAL]
```slithir
 StorageLib.loadCollateralManager().depositFreeCollateral(account,account,amount)
TMP_5932(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.depositFreeCollateral(CollateralManager,address,address,uint256), arguments:['TMP_5932', 'account_1', 'account_1', 'amount_1'] 
 onlySenderOrOperator(account,PerpsOperatorRoles.DEPOSIT_ACCOUNT)
REF_2500(PerpsOperatorRoles) -> PerpsOperatorRoles.DEPOSIT_ACCOUNT
MODIFIER_CALL, PerpManager.onlySenderOrOperator(address,PerpsOperatorRoles)(account_1,REF_2500)
```
#### PerpManager.depositFromSpot(address,uint256) [EXTERNAL]
```slithir
accountManager_2(IAccountManager) := phi(['accountManager_4', 'accountManager_0', 'accountManager_1'])
 accountManager.withdrawToPerps(account,amount)
HIGH_LEVEL_CALL, dest:accountManager_3(IAccountManager), function:withdrawToPerps, arguments:['account_1', 'amount_1']  
accountManager_4(IAccountManager) := phi(['accountManager_4', 'accountManager_3', 'accountManager_1'])
 StorageLib.loadCollateralManager().depositFromSpot(account,amount)
TMP_5941(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.depositFromSpot(CollateralManager,address,uint256), arguments:['TMP_5941', 'account_1', 'amount_1'] 
 onlySenderOrOperator(account,PerpsOperatorRoles.SPOT_TO_PERP_DEPOSIT)
REF_2509(PerpsOperatorRoles) -> PerpsOperatorRoles.SPOT_TO_PERP_DEPOSIT
MODIFIER_CALL, PerpManager.onlySenderOrOperator(address,PerpsOperatorRoles)(account_1,REF_2509)
```
#### PerpManager.depositTo(address,uint256) [EXTERNAL]
```slithir
 StorageLib.loadCollateralManager().depositFreeCollateral({from:msg.sender,to:account,amount:amount})
TMP_5938(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.depositFreeCollateral(CollateralManager,address,address,uint256), arguments:['TMP_5938', 'msg.sender', 'account_1', 'amount_1']
```
#### PerpManager.placeOrder(address,PlaceOrderArgs) [EXTERNAL]
```slithir
 StorageLib.loadClearingHouse().placeOrder(account,args,BookType.STANDARD)
TMP_6007(ClearingHouse) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadClearingHouse(), arguments:[] 
REF_2626(BookType) -> BookType.STANDARD
TMP_6008(PlaceOrderResult) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.placeOrder(ClearingHouse,address,PlaceOrderArgs,BookType), arguments:['TMP_6007', 'account_1', 'args_1', 'REF_2626'] 
RETURN TMP_6008
 onlySenderOrOperator(account,PerpsOperatorRoles.PLACE_ORDER)
REF_2627(PerpsOperatorRoles) -> PerpsOperatorRoles.PLACE_ORDER
MODIFIER_CALL, PerpManager.onlySenderOrOperator(address,PerpsOperatorRoles)(account_1,REF_2627)
 onlyActiveProtocol()
MODIFIER_CALL, PerpManager.onlyActiveProtocol()()
 result
```
#### PerpManager.postLimitOrderBackstop(address,PlaceOrderArgs) [EXTERNAL]
```slithir
 args.tif != TiF.MOC
REF_2628(TiF) -> args_1.tif
REF_2629(TiF) -> TiF.MOC
TMP_6011(bool) = REF_2628 != REF_2629
CONDITION TMP_6011
 revert InvalidBackstopLimitOrder()()
TMP_6012(None) = SOLIDITY_CALL revert InvalidBackstopLimitOrder()()
 StorageLib.loadClearingHouse().placeOrder(account,args,BookType.BACKSTOP)
TMP_6013(ClearingHouse) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadClearingHouse(), arguments:[] 
REF_2632(BookType) -> BookType.BACKSTOP
TMP_6014(PlaceOrderResult) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.placeOrder(ClearingHouse,address,PlaceOrderArgs,BookType), arguments:['TMP_6013', 'account_1', 'args_1', 'REF_2632'] 
RETURN TMP_6014
 onlySenderOrOperator(account,PerpsOperatorRoles.PLACE_ORDER)
REF_2633(PerpsOperatorRoles) -> PerpsOperatorRoles.PLACE_ORDER
MODIFIER_CALL, PerpManager.onlySenderOrOperator(address,PerpsOperatorRoles)(account_1,REF_2633)
 onlyActiveProtocol()
MODIFIER_CALL, PerpManager.onlyActiveProtocol()()
 result
```
#### PerpManager.removeMargin(address,uint256,uint256) [EXTERNAL]
```slithir
 clearingHouse = StorageLib.loadClearingHouse()
TMP_5964(ClearingHouse) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadClearingHouse(), arguments:[] 
clearingHouse_1 (-> ['TMP_5964'])(ClearingHouse) := TMP_5964(ClearingHouse)
 (cache.assets,cache.positions) = clearingHouse.getAccount(account,subaccount)
REF_2535(DynamicArrayLib.DynamicArray) -> cache_0.assets
REF_2536(Position[]) -> cache_0.positions
TUPLE_58(DynamicArrayLib.DynamicArray,Position[]) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.getAccount(ClearingHouse,address,uint256), arguments:["clearingHouse_1 (-> ['TMP_5964'])", 'account_1', 'subaccount_1'] 
REF_2535(DynamicArrayLib.DynamicArray)= UNPACK TUPLE_58 index: 0 
REF_2536(Position[])= UNPACK TUPLE_58 index: 1 
 amount == 0
TMP_5965(bool) = amount_1 == 0
CONDITION TMP_5965
 revert InvalidWithdraw()()
TMP_5966(None) = SOLIDITY_CALL revert InvalidWithdraw()()
 cache.positions.length == 0
REF_2538(Position[]) -> cache_0.positions
REF_2539 -> LENGTH REF_2538
TMP_5967(bool) = REF_2539 == 0
CONDITION TMP_5967
 revert InvalidWithdraw()()
TMP_5968(None) = SOLIDITY_CALL revert InvalidWithdraw()()
 cache.fundingPayment = ClearingHouseLib.realizeFundingPayment(cache.assets,cache.positions)
REF_2540(int256) -> cache_0.fundingPayment
REF_2542(DynamicArrayLib.DynamicArray) -> cache_0.assets
REF_2543(Position[]) -> cache_0.positions
TMP_5969(int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.realizeFundingPayment(DynamicArrayLib.DynamicArray,Position[]), arguments:['REF_2542', 'REF_2543'] 
cache_1(PerpManager.__MarginUpdateCache__) := phi(['cache_0'])
REF_2540(int256) (->cache_1) := TMP_5969(int256)
 remainingMargin = StorageLib.loadCollateralManager().settleMarginUpdate({account:account,subaccount:subaccount,marginDelta:- amount.toInt256(),fundingPayment:cache.fundingPayment})
TMP_5970(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
TMP_5971(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['amount_1'] 
TMP_5972(int256) = 0 (c)- TMP_5971
REF_2547(int256) -> cache_1.fundingPayment
TMP_5973(int256) = LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.settleMarginUpdate(CollateralManager,address,uint256,int256,int256), arguments:['TMP_5970', 'account_1', 'subaccount_1', 'TMP_5972', 'REF_2547'] 
remainingMargin_1(int256) := TMP_5973(int256)
 clearingHouse.assertPostWithdrawalMarginRequired({assets:cache.assets,positions:cache.positions,margin:remainingMargin})
REF_2549(DynamicArrayLib.DynamicArray) -> cache_1.assets
REF_2550(Position[]) -> cache_1.positions
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.assertPostWithdrawalMarginRequired(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256), arguments:["clearingHouse_1 (-> ['TMP_5964'])", 'REF_2549', 'REF_2550', 'remainingMargin_1'] 
 clearingHouse.setPositions({tradedAsset:,account:account,subaccount:subaccount,assets:cache.assets,positions:cache.positions})
REF_2552(DynamicArrayLib.DynamicArray) -> cache_1.assets
REF_2553(Position[]) -> cache_1.positions
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.setPositions(ClearingHouse,bytes32,address,uint256,DynamicArrayLib.DynamicArray,Position[]), arguments:["clearingHouse_1 (-> ['TMP_5964'])", '', 'account_1', 'subaccount_1', 'REF_2552', 'REF_2553'] 
 MarginRemoved(account,subaccount,amount,remainingMargin,StorageLib.incNonce())
TMP_5976(uint256) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.incNonce(), arguments:[] 
Emit MarginRemoved(account_1,subaccount_1,amount_1,remainingMargin_1,TMP_5976)
 onlySenderOrOperator(account,PerpsOperatorRoles.WITHDRAW_MARGIN)
REF_2555(PerpsOperatorRoles) -> PerpsOperatorRoles.WITHDRAW_MARGIN
MODIFIER_CALL, PerpManager.onlySenderOrOperator(address,PerpsOperatorRoles)(account_1,REF_2555)
```
#### PerpManager.setPositionLeverage(bytes32,address,uint256,uint256) [EXTERNAL]
```slithir
 clearingHouse = StorageLib.loadClearingHouse()
TMP_5979(ClearingHouse) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadClearingHouse(), arguments:[] 
clearingHouse_1 (-> ['TMP_5979'])(ClearingHouse) := TMP_5979(ClearingHouse)
 market = clearingHouse.market[asset]
REF_2557(mapping(bytes32 => Market)) -> clearingHouse_1 (-> ['TMP_5979']).market
REF_2558(Market) -> REF_2557[asset_1]
market_1 (-> ['clearingHouse'])(Market) := REF_2558(Market)
 MarketLib.assertActive(asset)
LIBRARY_CALL, dest:MarketLib, function:MarketLib.assertActive(bytes32), arguments:['asset_1'] 
 MarketLib.assertMaxLeverage(asset,newLeverage)
LIBRARY_CALL, dest:MarketLib, function:MarketLib.assertMaxLeverage(bytes32,uint256), arguments:['asset_1', 'newLeverage_1'] 
 cache.currentLeverage = market.getPositionLeverage(account,subaccount)
REF_2561(uint256) -> cache_0.currentLeverage
TMP_5982(uint256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getPositionLeverage(Market,address,uint256), arguments:["market_1 (-> ['clearingHouse'])", 'account_1', 'subaccount_1'] 
cache_1(PerpManager.__UpdateLeverageCache__) := phi(['cache_0'])
REF_2561(uint256) (->cache_1) := TMP_5982(uint256)
 cache.orderbookNotional = market.orderbookNotional[account][subaccount]
REF_2563(uint256) -> cache_1.orderbookNotional
REF_2564(mapping(address => mapping(uint256 => uint256))) -> market_1 (-> ['clearingHouse']).orderbookNotional
REF_2565(mapping(uint256 => uint256)) -> REF_2564[account_1]
REF_2566(uint256) -> REF_2565[subaccount_1]
cache_2(PerpManager.__UpdateLeverageCache__) := phi(['cache_1'])
REF_2563(uint256) (->cache_2) := REF_2566(uint256)
 cache.newOrderbookMargin = cache.orderbookNotional.fullMulDiv(1e18,newLeverage)
REF_2567(uint256) -> cache_2.newOrderbookMargin
REF_2568(uint256) -> cache_2.orderbookNotional
TMP_5983(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_2568', '1000000000000000000', 'newLeverage_1'] 
cache_3(PerpManager.__UpdateLeverageCache__) := phi(['cache_2'])
REF_2567(uint256) (->cache_3) := TMP_5983(uint256)
 cache.currentOrderbookMargin = cache.orderbookNotional.fullMulDiv(1e18,cache.currentLeverage)
REF_2570(uint256) -> cache_3.currentOrderbookMargin
REF_2571(uint256) -> cache_3.orderbookNotional
REF_2573(uint256) -> cache_3.currentLeverage
TMP_5984(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_2571', '1000000000000000000', 'REF_2573'] 
cache_4(PerpManager.__UpdateLeverageCache__) := phi(['cache_3'])
REF_2570(uint256) (->cache_4) := TMP_5984(uint256)
 cache.collateralDeltaFromBook = cache.newOrderbookMargin.toInt256() - cache.currentOrderbookMargin.toInt256()
REF_2574(int256) -> cache_4.collateralDeltaFromBook
REF_2575(uint256) -> cache_4.newOrderbookMargin
TMP_5985(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_2575'] 
REF_2577(uint256) -> cache_4.currentOrderbookMargin
TMP_5986(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_2577'] 
TMP_5987(int256) = TMP_5985 (c)- TMP_5986
cache_5(PerpManager.__UpdateLeverageCache__) := phi(['cache_4'])
REF_2574(int256) (->cache_5) := TMP_5987(int256)
 market.position[account][subaccount].leverage = newLeverage
REF_2579(mapping(address => mapping(uint256 => Position))) -> market_1 (-> ['clearingHouse']).position
REF_2580(mapping(uint256 => Position)) -> REF_2579[account_1]
REF_2581(Position) -> REF_2580[subaccount_1]
REF_2582(uint256) -> REF_2581.leverage
market_2 (-> ['clearingHouse'])(Market) := phi(["market_1 (-> ['clearingHouse'])"])
REF_2582(uint256) (->market_2 (-> ['clearingHouse'])) := newLeverage_1(uint256)
clearingHouse_2 (-> ['clearingHouse'])(ClearingHouse) := phi(["market_2 (-> ['clearingHouse'])"])
 market.position[account][subaccount].amount == 0
REF_2583(mapping(address => mapping(uint256 => Position))) -> market_2 (-> ['clearingHouse']).position
REF_2584(mapping(uint256 => Position)) -> REF_2583[account_1]
REF_2585(Position) -> REF_2584[subaccount_1]
REF_2586(uint256) -> REF_2585.amount
TMP_5988(bool) = REF_2586 == 0
CONDITION TMP_5988
 StorageLib.loadCollateralManager().handleCollateralDelta({account:account,collateralDelta:cache.collateralDeltaFromBook})
TMP_5989(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
REF_2589(int256) -> cache_5.collateralDeltaFromBook
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.handleCollateralDelta(CollateralManager,address,int256), arguments:['TMP_5989', 'account_1', 'REF_2589'] 
 margin = StorageLib.loadCollateralManager().getMarginBalance(account,subaccount)
TMP_5991(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
TMP_5992(int256) = LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.getMarginBalance(CollateralManager,address,uint256), arguments:['TMP_5991', 'account_1', 'subaccount_1'] 
margin_1(int256) := TMP_5992(int256)
 PositionLeverageSet(asset,account,subaccount,newLeverage,cache.collateralDeltaFromBook,margin,StorageLib.incNonce())
REF_2592(int256) -> cache_5.collateralDeltaFromBook
TMP_5993(uint256) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.incNonce(), arguments:[] 
Emit PositionLeverageSet(asset_1,account_1,subaccount_1,newLeverage_1,REF_2592,margin_1,TMP_5993)
 cache.collateralDeltaFromBook
REF_2594(int256) -> cache_5.collateralDeltaFromBook
RETURN REF_2594
 (cache.assets,cache.positions) = clearingHouse.getAccount(account,subaccount)
REF_2595(DynamicArrayLib.DynamicArray) -> cache_5.assets
REF_2596(Position[]) -> cache_5.positions
TUPLE_59(DynamicArrayLib.DynamicArray,Position[]) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.getAccount(ClearingHouse,address,uint256), arguments:["clearingHouse_1 (-> ['TMP_5979'])", 'account_1', 'subaccount_1'] 
REF_2595(DynamicArrayLib.DynamicArray)= UNPACK TUPLE_59 index: 0 
REF_2596(Position[])= UNPACK TUPLE_59 index: 1 
 cache.fundingPayment = ClearingHouseLib.realizeFundingPayment(cache.assets,cache.positions)
REF_2598(int256) -> cache_5.fundingPayment
REF_2600(DynamicArrayLib.DynamicArray) -> cache_5.assets
REF_2601(Position[]) -> cache_5.positions
TMP_5995(int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.realizeFundingPayment(DynamicArrayLib.DynamicArray,Position[]), arguments:['REF_2600', 'REF_2601'] 
cache_6(PerpManager.__UpdateLeverageCache__) := phi(['cache_5'])
REF_2598(int256) (->cache_6) := TMP_5995(int256)
 cache.newMargin = clearingHouse.getIntendedMargin(cache.assets,cache.positions)
REF_2602(uint256) -> cache_6.newMargin
REF_2604(DynamicArrayLib.DynamicArray) -> cache_6.assets
REF_2605(Position[]) -> cache_6.positions
TMP_5996(uint256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.getIntendedMargin(ClearingHouse,DynamicArrayLib.DynamicArray,Position[]), arguments:["clearingHouse_1 (-> ['TMP_5979'])", 'REF_2604', 'REF_2605'] 
cache_7(PerpManager.__UpdateLeverageCache__) := phi(['cache_6'])
REF_2602(uint256) (->cache_7) := TMP_5996(uint256)
 clearingHouse.assertOpenMarginRequired({assets:cache.assets,positions:cache.positions,margin:cache.newMargin.toInt256()})
REF_2607(DynamicArrayLib.DynamicArray) -> cache_7.assets
REF_2608(Position[]) -> cache_7.positions
REF_2609(uint256) -> cache_7.newMargin
TMP_5997(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_2609'] 
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.assertOpenMarginRequired(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256), arguments:["clearingHouse_1 (-> ['TMP_5979'])", 'REF_2607', 'REF_2608', 'TMP_5997'] 
 clearingHouse.setPositions({tradedAsset:,account:account,subaccount:subaccount,assets:cache.assets,positions:cache.positions})
REF_2612(DynamicArrayLib.DynamicArray) -> cache_7.assets
REF_2613(Position[]) -> cache_7.positions
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.setPositions(ClearingHouse,bytes32,address,uint256,DynamicArrayLib.DynamicArray,Position[]), arguments:["clearingHouse_1 (-> ['TMP_5979'])", '', 'account_1', 'subaccount_1', 'REF_2612', 'REF_2613'] 
 collateralDelta = StorageLib.loadCollateralManager().settleNewLeverage({account:account,subaccount:subaccount,collateralDeltaFromBook:cache.collateralDeltaFromBook,newMargin:cache.newMargin.toInt256(),fundingPayment:cache.fundingPayment})
TMP_6000(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
REF_2616(int256) -> cache_7.collateralDeltaFromBook
REF_2617(uint256) -> cache_7.newMargin
TMP_6001(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_2617'] 
REF_2619(int256) -> cache_7.fundingPayment
TMP_6002(int256) = LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.settleNewLeverage(CollateralManager,address,uint256,int256,int256,int256), arguments:['TMP_6000', 'account_1', 'subaccount_1', 'REF_2616', 'TMP_6001', 'REF_2619'] 
collateralDelta_1(int256) := TMP_6002(int256)
 PositionLeverageSet(asset,account,subaccount,newLeverage,collateralDelta,cache.newMargin.toInt256(),StorageLib.incNonce())
REF_2620(uint256) -> cache_7.newMargin
TMP_6003(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_2620'] 
TMP_6004(uint256) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.incNonce(), arguments:[] 
Emit PositionLeverageSet(asset_1,account_1,subaccount_1,newLeverage_1,collateralDelta_1,TMP_6003,TMP_6004)
 onlySenderOrOperator(account,PerpsOperatorRoles.SET_LEVERAGE)
REF_2623(PerpsOperatorRoles) -> PerpsOperatorRoles.SET_LEVERAGE
MODIFIER_CALL, PerpManager.onlySenderOrOperator(address,PerpsOperatorRoles)(account_1,REF_2623)
 collateralDelta
RETURN collateralDelta_1
```
#### GTL.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 _PERMIT2 = 0x000000000022D473030F116dDEE9F6B43aC78BA3
 _DEFAULT_UNDERLYING_DECIMALS = 18
 _DEFAULT_DECIMALS_OFFSET = 0
 _OWNER_SLOT = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927
 _ROLE_0 = 1 << 0
 _ROLE_1 = 1 << 1
 _ROLE_2 = 1 << 2
 _ROLE_3 = 1 << 3
 _ROLE_4 = 1 << 4
 _ROLE_5 = 1 << 5
 _ROLE_6 = 1 << 6
 _ROLE_7 = 1 << 7
 _ROLE_8 = 1 << 8
 _ROLE_9 = 1 << 9
 _ROLE_10 = 1 << 10
 _ROLE_11 = 1 << 11
 _ROLE_12 = 1 << 12
 _ROLE_13 = 1 << 13
 _ROLE_14 = 1 << 14
 _ROLE_15 = 1 << 15
 _ROLE_16 = 1 << 16
 _ROLE_17 = 1 << 17
 _ROLE_18 = 1 << 18
 _ROLE_19 = 1 << 19
 _ROLE_20 = 1 << 20
 _ROLE_21 = 1 << 21
 _ROLE_22 = 1 << 22
 _ROLE_23 = 1 << 23
 _ROLE_24 = 1 << 24
 _ROLE_25 = 1 << 25
 _ROLE_26 = 1 << 26
 _ROLE_27 = 1 << 27
 _ROLE_28 = 1 << 28
 _ROLE_29 = 1 << 29
 _ROLE_30 = 1 << 30
 _ROLE_31 = 1 << 31
 _ROLE_32 = 1 << 32
 _ROLE_33 = 1 << 33
 _ROLE_34 = 1 << 34
 _ROLE_35 = 1 << 35
 _ROLE_36 = 1 << 36
 _ROLE_37 = 1 << 37
 _ROLE_38 = 1 << 38
 _ROLE_39 = 1 << 39
 _ROLE_40 = 1 << 40
 _ROLE_41 = 1 << 41
 _ROLE_42 = 1 << 42
 _ROLE_43 = 1 << 43
 _ROLE_44 = 1 << 44
 _ROLE_45 = 1 << 45
 _ROLE_46 = 1 << 46
 _ROLE_47 = 1 << 47
 _ROLE_48 = 1 << 48
 _ROLE_49 = 1 << 49
 _ROLE_50 = 1 << 50
 _ROLE_51 = 1 << 51
 _ROLE_52 = 1 << 52
 _ROLE_53 = 1 << 53
 _ROLE_54 = 1 << 54
 _ROLE_55 = 1 << 55
 _ROLE_56 = 1 << 56
 _ROLE_57 = 1 << 57
 _ROLE_58 = 1 << 58
 _ROLE_59 = 1 << 59
 _ROLE_60 = 1 << 60
 _ROLE_61 = 1 << 61
 _ROLE_62 = 1 << 62
 _ROLE_63 = 1 << 63
 _ROLE_64 = 1 << 64
 _ROLE_65 = 1 << 65
 _ROLE_66 = 1 << 66
 _ROLE_67 = 1 << 67
 _ROLE_68 = 1 << 68
 _ROLE_69 = 1 << 69
 _ROLE_70 = 1 << 70
 _ROLE_71 = 1 << 71
 _ROLE_72 = 1 << 72
 _ROLE_73 = 1 << 73
 _ROLE_74 = 1 << 74
 _ROLE_75 = 1 << 75
 _ROLE_76 = 1 << 76
 _ROLE_77 = 1 << 77
 _ROLE_78 = 1 << 78
 _ROLE_79 = 1 << 79
 _ROLE_80 = 1 << 80
 _ROLE_81 = 1 << 81
 _ROLE_82 = 1 << 82
 _ROLE_83 = 1 << 83
 _ROLE_84 = 1 << 84
 _ROLE_85 = 1 << 85
 _ROLE_86 = 1 << 86
 _ROLE_87 = 1 << 87
 _ROLE_88 = 1 << 88
 _ROLE_89 = 1 << 89
 _ROLE_90 = 1 << 90
 _ROLE_91 = 1 << 91
 _ROLE_92 = 1 << 92
 _ROLE_93 = 1 << 93
 _ROLE_94 = 1 << 94
 _ROLE_95 = 1 << 95
 _ROLE_96 = 1 << 96
 _ROLE_97 = 1 << 97
 _ROLE_98 = 1 << 98
 _ROLE_99 = 1 << 99
 _ROLE_100 = 1 << 100
 _ROLE_101 = 1 << 101
 _ROLE_102 = 1 << 102
 _ROLE_103 = 1 << 103
 _ROLE_104 = 1 << 104
 _ROLE_105 = 1 << 105
 _ROLE_106 = 1 << 106
 _ROLE_107 = 1 << 107
 _ROLE_108 = 1 << 108
 _ROLE_109 = 1 << 109
 _ROLE_110 = 1 << 110
 _ROLE_111 = 1 << 111
 _ROLE_112 = 1 << 112
 _ROLE_113 = 1 << 113
 _ROLE_114 = 1 << 114
 _ROLE_115 = 1 << 115
 _ROLE_116 = 1 << 116
 _ROLE_117 = 1 << 117
 _ROLE_118 = 1 << 118
 _ROLE_119 = 1 << 119
 _ROLE_120 = 1 << 120
 _ROLE_121 = 1 << 121
 _ROLE_122 = 1 << 122
 _ROLE_123 = 1 << 123
 _ROLE_124 = 1 << 124
 _ROLE_125 = 1 << 125
 _ROLE_126 = 1 << 126
 _ROLE_127 = 1 << 127
 _ROLE_128 = 1 << 128
 _ROLE_129 = 1 << 129
 _ROLE_130 = 1 << 130
 _ROLE_131 = 1 << 131
 _ROLE_132 = 1 << 132
 _ROLE_133 = 1 << 133
 _ROLE_134 = 1 << 134
 _ROLE_135 = 1 << 135
 _ROLE_136 = 1 << 136
 _ROLE_137 = 1 << 137
 _ROLE_138 = 1 << 138
 _ROLE_139 = 1 << 139
 _ROLE_140 = 1 << 140
 _ROLE_141 = 1 << 141
 _ROLE_142 = 1 << 142
 _ROLE_143 = 1 << 143
 _ROLE_144 = 1 << 144
 _ROLE_145 = 1 << 145
 _ROLE_146 = 1 << 146
 _ROLE_147 = 1 << 147
 _ROLE_148 = 1 << 148
 _ROLE_149 = 1 << 149
 _ROLE_150 = 1 << 150
 _ROLE_151 = 1 << 151
 _ROLE_152 = 1 << 152
 _ROLE_153 = 1 << 153
 _ROLE_154 = 1 << 154
 _ROLE_155 = 1 << 155
 _ROLE_156 = 1 << 156
 _ROLE_157 = 1 << 157
 _ROLE_158 = 1 << 158
 _ROLE_159 = 1 << 159
 _ROLE_160 = 1 << 160
 _ROLE_161 = 1 << 161
 _ROLE_162 = 1 << 162
 _ROLE_163 = 1 << 163
 _ROLE_164 = 1 << 164
 _ROLE_165 = 1 << 165
 _ROLE_166 = 1 << 166
 _ROLE_167 = 1 << 167
 _ROLE_168 = 1 << 168
 _ROLE_169 = 1 << 169
 _ROLE_170 = 1 << 170
 _ROLE_171 = 1 << 171
 _ROLE_172 = 1 << 172
 _ROLE_173 = 1 << 173
 _ROLE_174 = 1 << 174
 _ROLE_175 = 1 << 175
 _ROLE_176 = 1 << 176
 _ROLE_177 = 1 << 177
 _ROLE_178 = 1 << 178
 _ROLE_179 = 1 << 179
 _ROLE_180 = 1 << 180
 _ROLE_181 = 1 << 181
 _ROLE_182 = 1 << 182
 _ROLE_183 = 1 << 183
 _ROLE_184 = 1 << 184
 _ROLE_185 = 1 << 185
 _ROLE_186 = 1 << 186
 _ROLE_187 = 1 << 187
 _ROLE_188 = 1 << 188
 _ROLE_189 = 1 << 189
 _ROLE_190 = 1 << 190
 _ROLE_191 = 1 << 191
 _ROLE_192 = 1 << 192
 _ROLE_193 = 1 << 193
 _ROLE_194 = 1 << 194
 _ROLE_195 = 1 << 195
 _ROLE_196 = 1 << 196
 _ROLE_197 = 1 << 197
 _ROLE_198 = 1 << 198
 _ROLE_199 = 1 << 199
 _ROLE_200 = 1 << 200
 _ROLE_201 = 1 << 201
 _ROLE_202 = 1 << 202
 _ROLE_203 = 1 << 203
 _ROLE_204 = 1 << 204
 _ROLE_205 = 1 << 205
 _ROLE_206 = 1 << 206
 _ROLE_207 = 1 << 207
 _ROLE_208 = 1 << 208
 _ROLE_209 = 1 << 209
 _ROLE_210 = 1 << 210
 _ROLE_211 = 1 << 211
 _ROLE_212 = 1 << 212
 _ROLE_213 = 1 << 213
 _ROLE_214 = 1 << 214
 _ROLE_215 = 1 << 215
 _ROLE_216 = 1 << 216
 _ROLE_217 = 1 << 217
 _ROLE_218 = 1 << 218
 _ROLE_219 = 1 << 219
 _ROLE_220 = 1 << 220
 _ROLE_221 = 1 << 221
 _ROLE_222 = 1 << 222
 _ROLE_223 = 1 << 223
 _ROLE_224 = 1 << 224
 _ROLE_225 = 1 << 225
 _ROLE_226 = 1 << 226
 _ROLE_227 = 1 << 227
 _ROLE_228 = 1 << 228
 _ROLE_229 = 1 << 229
 _ROLE_230 = 1 << 230
 _ROLE_231 = 1 << 231
 _ROLE_232 = 1 << 232
 _ROLE_233 = 1 << 233
 _ROLE_234 = 1 << 234
 _ROLE_235 = 1 << 235
 _ROLE_236 = 1 << 236
 _ROLE_237 = 1 << 237
 _ROLE_238 = 1 << 238
 _ROLE_239 = 1 << 239
 _ROLE_240 = 1 << 240
 _ROLE_241 = 1 << 241
 _ROLE_242 = 1 << 242
 _ROLE_243 = 1 << 243
 _ROLE_244 = 1 << 244
 _ROLE_245 = 1 << 245
 _ROLE_246 = 1 << 246
 _ROLE_247 = 1 << 247
 _ROLE_248 = 1 << 248
 _ROLE_249 = 1 << 249
 _ROLE_250 = 1 << 250
 _ROLE_251 = 1 << 251
 _ROLE_252 = 1 << 252
 _ROLE_253 = 1 << 253
 _ROLE_254 = 1 << 254
 _ROLE_255 = 1 << 255
 ABI_VERSION = 1
 ADMIN_ROLE = _ROLE_0
 _checkRoles(roles)
INTERNAL_CALL, OwnableRoles._checkRoles(uint256)(roles_1)
 _checkOwnerOrRoles(roles)
INTERNAL_CALL, OwnableRoles._checkOwnerOrRoles(uint256)(roles_1)
 _checkRolesOrOwner(roles)
INTERNAL_CALL, OwnableRoles._checkRolesOrOwner(uint256)(roles_1)
 _checkOwner()
INTERNAL_CALL, Ownable._checkOwner()()
_INTIALIZED_EVENT_SIGNATURE_3(bytes32) := phi(['_INTIALIZED_EVENT_SIGNATURE_0', '_INTIALIZED_EVENT_SIGNATURE_4', '_INTIALIZED_EVENT_SIGNATURE_2', '_INTIALIZED_EVENT_SIGNATURE_6'])
 s = _initializableSlot()
TMP_4965(bytes32) = INTERNAL_CALL, Initializable._initializableSlot()()
s_1(bytes32) := TMP_4965(bytes32)
 i_initializer_asm_0 = sload(uint256)(s)
TMP_4966(uint256) = SOLIDITY_CALL sload(uint256)(s_1)
i_initializer_asm_0_1(uint256) := TMP_4966(uint256)
 sstore(uint256,uint256)(s,3)
TMP_4967(None) = SOLIDITY_CALL sstore(uint256,uint256)(s_1,3)
 i_initializer_asm_0
CONDITION i_initializer_asm_0_1
s_3(bytes32) := phi(['s_2', 's_1'])
 ! extcodesize(uint256)(address()()) < i_initializer_asm_0 >> 1 == 1
TMP_4968 = CONVERT this to address
REF_1482 -> CODESIZE TMP_4968
TMP_4969(uint256) = i_initializer_asm_0_1 >> 1
TMP_4970(bool) = TMP_4969 == 1
TMP_4971(bool) = REF_1482 < TMP_4970
TMP_4972 = UnaryType.BANG TMP_4971 
CONDITION TMP_4972
 mstore(uint256,uint256)(0x00,0xf92ee8a9)
TMP_4973(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,4180601001)
 revert(uint256,uint256)(0x1c,0x04)
TMP_4974(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 s = s << i_initializer_asm_0 << 255
TMP_4975(uint256) = i_initializer_asm_0_1 << 255
TMP_4976(bytes32) = s_1 << TMP_4975
s_2(bytes32) := TMP_4976(bytes32)
 s
CONDITION s_3
 sstore(uint256,uint256)(s,2)
TMP_4977(None) = SOLIDITY_CALL sstore(uint256,uint256)(s_3,2)
 mstore(uint256,uint256)(0x20,1)
TMP_4978(None) = SOLIDITY_CALL mstore(uint256,uint256)(32,1)
 log1(uint256,uint256,uint256)(0x20,0x20,_INTIALIZED_EVENT_SIGNATURE)
TMP_4979(None) = SOLIDITY_CALL log1(uint256,uint256,uint256)(32,32,_INTIALIZED_EVENT_SIGNATURE_4)
_INTIALIZED_EVENT_SIGNATURE_5(bytes32) := phi(['_INTIALIZED_EVENT_SIGNATURE_0', '_INTIALIZED_EVENT_SIGNATURE_4', '_INTIALIZED_EVENT_SIGNATURE_2', '_INTIALIZED_EVENT_SIGNATURE_6'])
 s = _initializableSlot()
TMP_4980(bytes32) = INTERNAL_CALL, Initializable._initializableSlot()()
s_1(bytes32) := TMP_4980(bytes32)
 version = version & 0xffffffffffffffff
TMP_4981(uint64) = version_1 & 18446744073709551615
version_2(uint64) := TMP_4981(uint64)
 i_reinitializer_asm_0 = sload(uint256)(s)
TMP_4982(uint256) = SOLIDITY_CALL sload(uint256)(s_1)
i_reinitializer_asm_0_1(uint256) := TMP_4982(uint256)
 ! i_reinitializer_asm_0 & 1 < i_reinitializer_asm_0 >> 1 < version
TMP_4983(uint256) = i_reinitializer_asm_0_1 & 1
TMP_4984(uint256) = i_reinitializer_asm_0_1 >> 1
TMP_4985(bool) = TMP_4984 < version_2
TMP_4986(bool) = TMP_4983 < TMP_4985
TMP_4987 = UnaryType.BANG TMP_4986 
CONDITION TMP_4987
 mstore(uint256,uint256)(0x00,0xf92ee8a9)
TMP_4988(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,4180601001)
 revert(uint256,uint256)(0x1c,0x04)
TMP_4989(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 sstore(uint256,uint256)(s,1 | version << 1)
TMP_4990(uint64) = version_2 << 1
TMP_4991(uint256) = 1 | TMP_4990
TMP_4992(None) = SOLIDITY_CALL sstore(uint256,uint256)(s_1,TMP_4991)
 sstore(uint256,uint256)(s,version << 1)
TMP_4993(uint64) = version_2 << 1
TMP_4994(None) = SOLIDITY_CALL sstore(uint256,uint256)(s_1,TMP_4993)
 mstore(uint256,uint256)(0x20,version)
TMP_4995(None) = SOLIDITY_CALL mstore(uint256,uint256)(32,version_2)
 log1(uint256,uint256,uint256)(0x20,0x20,_INTIALIZED_EVENT_SIGNATURE)
TMP_4996(None) = SOLIDITY_CALL log1(uint256,uint256,uint256)(32,32,_INTIALIZED_EVENT_SIGNATURE_6)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
perpManager_17(address) := phi(['perpManager_10', 'perpManager_16', 'perpManager_14', 'perpManager_1', 'perpManager_7', 'perpManager_3', 'perpManager_12', 'perpManager_0'])
 msg.sender != perpManager
TMP_4998(bool) = msg.sender != perpManager_17
CONDITION TMP_4998
 revert NotPerpManager()()
TMP_4999(None) = SOLIDITY_CALL revert NotPerpManager()()
 _assertAdmin()
INTERNAL_CALL, GTL._assertAdmin()()
```
#### PerpManager.withdraw(address,uint256) [EXTERNAL]
```slithir
 StorageLib.loadCollateralManager().withdrawFreeCollateral(account,amount)
TMP_5935(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.withdrawFreeCollateral(CollateralManager,address,uint256), arguments:['TMP_5935', 'account_1', 'amount_1'] 
 onlySenderOrOperator(account,PerpsOperatorRoles.WITHDRAW_ACCOUNT)
REF_2503(PerpsOperatorRoles) -> PerpsOperatorRoles.WITHDRAW_ACCOUNT
MODIFIER_CALL, PerpManager.onlySenderOrOperator(address,PerpsOperatorRoles)(account_1,REF_2503)
```
#### PerpManager.withdrawToSpot(address,uint256) [EXTERNAL]
```slithir
accountManager_5(IAccountManager) := phi(['accountManager_4', 'accountManager_0', 'accountManager_1'])
 msg.sender != address(accountManager)
TMP_5944 = CONVERT accountManager_5 to address
TMP_5945(bool) = msg.sender != TMP_5944
CONDITION TMP_5945
 revert NotAccountManager()()
TMP_5946(None) = SOLIDITY_CALL revert NotAccountManager()()
 StorageLib.loadCollateralManager().withdrawToSpot(account,amount,address(accountManager))
TMP_5947(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
TMP_5948 = CONVERT accountManager_5 to address
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.withdrawToSpot(CollateralManager,address,uint256,address), arguments:['TMP_5947', 'account_1', 'amount_1', 'TMP_5948']
```
#### FixedPointMathLib.fullMulDiv(uint256,uint256,uint256) [INTERNAL]
```slithir
x_1(uint256) := phi(['x_1', 'TMP_13961', 'TMP_13966', 'TMP_13980', 'TMP_13990'])
y_1(uint256) := phi(['TMP_13982', 'TMP_13992', 'y_1', 'TMP_13967', 'TMP_13962'])
d_1(uint256) := phi(['d_1', 'TMP_13984', 'TMP_13994', 'TMP_13968', 'TMP_13963'])
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
#### ClearingHouseLib.assertNotLiquidatable(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256) [INTERNAL]
```slithir
 self.isLiquidatable(assets,positions,margin,BookType.STANDARD)
REF_4804(BookType) -> BookType.STANDARD
TMP_9147(bool) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.isLiquidatable(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256,BookType), arguments:['self_1 (-> [])', 'assets_1', 'positions_1', 'margin_1', 'REF_4804'] 
CONDITION TMP_9147
 revert Liquidatable()()
TMP_9148(None) = SOLIDITY_CALL revert Liquidatable()()
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
#### ClearingHouseLib.realizeFundingPayment(DynamicArrayLib.DynamicArray,Position[]) [INTERNAL]
```slithir
assets_1(DynamicArrayLib.DynamicArray) := phi(['REF_4702', 'REF_4444'])
positions_1(Position[]) := phi(['REF_4445', 'REF_4703'])
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
#### CollateralManagerLib.settleMarginUpdate(CollateralManager,address,uint256,int256,int256) [INTERNAL]
```slithir
 remainingMargin = self.margin[account][subaccount] += marginDelta - fundingPayment
REF_4817(mapping(address => mapping(uint256 => int256))) -> self_1 (-> []).margin
REF_4818(mapping(uint256 => int256)) -> REF_4817[account_1]
REF_4819(int256) -> REF_4818[subaccount_1]
TMP_9176(int256) = marginDelta_1 (c)- fundingPayment_1
self_2 (-> [])(CollateralManager) := phi(['self_1 (-> [])'])
REF_4819(-> self_2 (-> [])) = REF_4819 (c)+ TMP_9176
remainingMargin_1(int256) := REF_4819(int256)
 self.handleCollateralDelta(account,marginDelta)
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.handleCollateralDelta(CollateralManager,address,int256), arguments:['self_2 (-> [])', 'account_1', 'marginDelta_1'] 
 remainingMargin
RETURN remainingMargin_1
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
#### MarketLib.amendLimitOrder(Market,address,AmendLimitOrderArgs,BookType) [INTERNAL]
```slithir
 args.reduceOnly
REF_4963(bool) -> args_1.reduceOnly
CONDITION REF_4963
 _validateReduceOnlyOrder(self,account,args.subaccount,args.baseAmount,args.side,true)
REF_4964(uint256) -> args_1.subaccount
REF_4965(uint256) -> args_1.baseAmount
REF_4966(Side) -> args_1.side
INTERNAL_CALL, MarketLib._validateReduceOnlyOrder(Market,address,uint256,uint256,Side,bool)(self_1 (-> []),account_1,REF_4964,REF_4965,REF_4966,True)
 CLOBLib.amend(account,args,bookType)
TMP_9289(int256) = LIBRARY_CALL, dest:CLOBLib, function:CLOBLib.amend(address,AmendLimitOrderArgs,BookType), arguments:['account_1', 'args_1', 'bookType_1'] 
RETURN TMP_9289
 onlyActiveMarket(args.asset)
REF_4968(bytes32) -> args_1.asset
MODIFIER_CALL, MarketLib.onlyActiveMarket(bytes32)(REF_4968)
 collateralDelta
```
#### CLOBLib.cancel(bytes32,address,uint256,uint256[],BookType) [INTERNAL]
```slithir
 ds = _getStorage(asset,bookType)
TMP_8710(Book) = INTERNAL_CALL, CLOBLib._getStorage(bytes32,BookType)(asset_1,bookType_1)
ds_1 (-> ['TMP_8710'])(Book) := TMP_8710(Book)
 collateralRefunded = _executeCancel(ds,account,subaccount,orderIds)
TMP_8711(uint256) = INTERNAL_CALL, CLOBLib._executeCancel(Book,address,uint256,uint256[])(ds_1 (-> ['TMP_8710']),account_1,subaccount_1,orderIds_1)
collateralRefunded_1(uint256) := TMP_8711(uint256)
 collateralRefunded
RETURN collateralRefunded_1
```
#### CollateralManagerLib.depositFreeCollateral(CollateralManager,address,address,uint256) [INTERNAL]
```slithir
USDC_1(address) := phi(['USDC_0'])
 USDC.safeTransferFrom(from,address(this),amount)
TMP_9164 = CONVERT this to address
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransferFrom(address,address,address,uint256), arguments:['USDC_1', 'from_1', 'TMP_9164', 'amount_1'] 
 self.creditAccount(to,amount)
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.creditAccount(CollateralManager,address,uint256), arguments:['self_1 (-> [])', 'to_1', 'amount_1'] 
 Deposit(to,amount)
Emit Deposit(to_1,amount_1)
```
#### IAccountManager.withdrawToPerps(address,uint256) [EXTERNAL]
```slithir

```
#### CollateralManagerLib.depositFromSpot(CollateralManager,address,uint256) [INTERNAL]
```slithir
 self.creditAccount(account,amount)
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.creditAccount(CollateralManager,address,uint256), arguments:['self_1 (-> [])', 'account_1', 'amount_1'] 
 Deposit(account,amount)
Emit Deposit(account_1,amount_1)
```
#### ClearingHouseLib.placeOrder(ClearingHouse,address,PlaceOrderArgs,BookType) [INTERNAL]
```slithir
 market = self.market[args.asset]
REF_4405(mapping(bytes32 => Market)) -> self_1 (-> []).market
REF_4406(bytes32) -> args_1.asset
REF_4407(Market) -> REF_4405[REF_4406]
market_1 (-> ['self'])(Market) := REF_4407(Market)
 orderResult = market.placeOrder(account,args,bookType)
TMP_8925(PlaceOrderResult) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.placeOrder(Market,address,PlaceOrderArgs,BookType), arguments:["market_1 (-> ['self'])", 'account_1', 'args_1', 'bookType_1'] 
orderResult_1(PlaceOrderResult) := TMP_8925(PlaceOrderResult)
 orderResult.basePosted > 0 && ! args.reduceOnly
REF_4409(uint256) -> orderResult_1.basePosted
TMP_8926(bool) = REF_4409 > 0
REF_4410(bool) -> args_1.reduceOnly
TMP_8927 = UnaryType.BANG REF_4410 
TMP_8928(bool) = TMP_8926 && TMP_8927
CONDITION TMP_8928
 collateralPosted = _getCollateral(orderResult.basePosted,args.limitPrice,market.getPositionLeverage(account,args.subaccount))
REF_4411(uint256) -> orderResult_1.basePosted
REF_4412(uint256) -> args_1.limitPrice
REF_4414(uint256) -> args_1.subaccount
TMP_8929(uint256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getPositionLeverage(Market,address,uint256), arguments:["market_1 (-> ['self'])", 'account_1', 'REF_4414'] 
TMP_8930(uint256) = INTERNAL_CALL, ClearingHouseLib._getCollateral(uint256,uint256,uint256)(REF_4411,REF_4412,TMP_8929)
collateralPosted_1(uint256) := TMP_8930(uint256)
collateralPosted_2(uint256) := phi(['collateralPosted_1', 'collateralPosted_0'])
 orderResult.baseTraded == 0
REF_4415(uint256) -> orderResult_1.baseTraded
TMP_8931(bool) = REF_4415 == 0
CONDITION TMP_8931
 StorageLib.loadCollateralManager().handleCollateralDelta({account:account,collateralDelta:collateralPosted.toInt256()})
TMP_8932(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
TMP_8933(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['collateralPosted_2'] 
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.handleCollateralDelta(CollateralManager,address,int256), arguments:['TMP_8932', 'account_1', 'TMP_8933'] 
 orderResult
RETURN orderResult_1
 _processTakerFill(self,__FillParams__({asset:args.asset,account:account,subaccount:args.subaccount,side:args.side,quoteAmount:orderResult.quoteTraded,baseAmount:orderResult.baseTraded,collateralPosted:collateralPosted}))
REF_4419(bytes32) -> args_1.asset
REF_4420(uint256) -> args_1.subaccount
REF_4421(Side) -> args_1.side
REF_4422(uint256) -> orderResult_1.quoteTraded
REF_4423(uint256) -> orderResult_1.baseTraded
TMP_8935(ClearingHouseLib.__FillParams__) = new __FillParams__(REF_4419,account_1,REF_4420,REF_4421,REF_4422,REF_4423,collateralPosted_2)
INTERNAL_CALL, ClearingHouseLib._processTakerFill(ClearingHouse,ClearingHouseLib.__FillParams__)(self_1 (-> []),TMP_8935)
 orderResult
RETURN orderResult_1
```
#### ClearingHouseLib.assertPostWithdrawalMarginRequired(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256) [INTERNAL]
```slithir
 margin < 0
TMP_9152(bool) = margin_1 < 0
CONDITION TMP_9152
 revert MarginRequirementUnmet()()
TMP_9153(None) = SOLIDITY_CALL revert MarginRequirementUnmet()()
 (intendedMargin,upnl) = _getIntendedMarginAndUpnl(self,assets,positions)
TUPLE_101(uint256,int256) = INTERNAL_CALL, ClearingHouseLib._getIntendedMarginAndUpnl(ClearingHouse,DynamicArrayLib.DynamicArray,Position[])(self_1 (-> []),assets_1,positions_1)
intendedMargin_1(uint256)= UNPACK TUPLE_101 index: 0 
upnl_1(int256)= UNPACK TUPLE_101 index: 1 
 totalNotional = self.getNotionalAccountValue(assets,positions)
TMP_9154(uint256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.getNotionalAccountValue(ClearingHouse,DynamicArrayLib.DynamicArray,Position[]), arguments:['self_1 (-> [])', 'assets_1', 'positions_1'] 
totalNotional_1(uint256) := TMP_9154(uint256)
 intendedMargin = intendedMargin.max(totalNotional / 10)
TMP_9155(uint256) = totalNotional_1 (c)/ 10
TMP_9156(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.max(uint256,uint256), arguments:['intendedMargin_1', 'TMP_9155'] 
intendedMargin_2(uint256) := TMP_9156(uint256)
 margin + upnl < intendedMargin.toInt256()
TMP_9157(int256) = margin_1 (c)+ upnl_1
TMP_9158(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['intendedMargin_2'] 
TMP_9159(bool) = TMP_9157 < TMP_9158
CONDITION TMP_9159
 revert MarginRequirementUnmet()()
TMP_9160(None) = SOLIDITY_CALL revert MarginRequirementUnmet()()
```

#### ClearingHouseLib.getIntendedMargin(ClearingHouse,DynamicArrayLib.DynamicArray,Position[]) [INTERNAL]
```slithir
 length = assets.length()
TMP_9021(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.length(DynamicArrayLib.DynamicArray), arguments:['assets_1'] 
length_1(uint256) := TMP_9021(uint256)
 i < length
intendedMargin_1(uint256) := phi(['intendedMargin_0', 'intendedMargin_2'])
i_1(uint256) := phi(['i_0', 'i_2'])
TMP_9022(bool) = i_1 < length_1
CONDITION TMP_9022
 intendedMargin += self.market[assets.getBytes32(i)].getIntendedMargin(positions[i])
REF_4605(mapping(bytes32 => Market)) -> self_1 (-> []).market
TMP_9023(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
REF_4607(Market) -> REF_4605[TMP_9023]
REF_4609(Position) -> positions_1[i_1]
TMP_9024(uint256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getIntendedMargin(Market,Position), arguments:['REF_4607', 'REF_4609'] 
intendedMargin_2(uint256) = intendedMargin_1 (c)+ TMP_9024
 ++ i
i_2(uint256) = i_1 (c)+ 1
 intendedMargin
RETURN intendedMargin_1
```
#### CollateralManagerLib.getMarginBalance(CollateralManager,address,uint256) [INTERNAL]
```slithir
 self.margin[account][subaccount]
REF_4844(mapping(address => mapping(uint256 => int256))) -> self_1 (-> []).margin
REF_4845(mapping(uint256 => int256)) -> REF_4844[account_1]
REF_4846(int256) -> REF_4845[subaccount_1]
RETURN REF_4846
```
#### CollateralManagerLib.settleNewLeverage(CollateralManager,address,uint256,int256,int256,int256) [INTERNAL]
```slithir
 currentMargin = self.margin[account][subaccount] - fundingPayment
REF_4821(mapping(address => mapping(uint256 => int256))) -> self_1 (-> []).margin
REF_4822(mapping(uint256 => int256)) -> REF_4821[account_1]
REF_4823(int256) -> REF_4822[subaccount_1]
TMP_9178(int256) = REF_4823 (c)- fundingPayment_1
currentMargin_1(int256) := TMP_9178(int256)
 collateralDeltaFromPosition = newMargin - currentMargin
TMP_9179(int256) = newMargin_1 (c)- currentMargin_1
collateralDeltaFromPosition_1(int256) := TMP_9179(int256)
 collateralDelta = collateralDeltaFromPosition + collateralDeltaFromBook
TMP_9180(int256) = collateralDeltaFromPosition_1 (c)+ collateralDeltaFromBook_1
collateralDelta_1(int256) := TMP_9180(int256)
 self.handleCollateralDelta(account,collateralDelta)
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.handleCollateralDelta(CollateralManager,address,int256), arguments:['self_1 (-> [])', 'account_1', 'collateralDelta_1'] 
 self.margin[account][subaccount] = newMargin
REF_4825(mapping(address => mapping(uint256 => int256))) -> self_1 (-> []).margin
REF_4826(mapping(uint256 => int256)) -> REF_4825[account_1]
REF_4827(int256) -> REF_4826[subaccount_1]
self_2 (-> [])(CollateralManager) := phi(['self_1 (-> [])'])
REF_4827(int256) (->self_2 (-> [])) := newMargin_1(int256)
 collateralDelta
RETURN collateralDelta_1
```
#### MarketLib.assertActive(bytes32) [INTERNAL]
```slithir
asset_1(bytes32) := phi(['asset_1'])
 StorageLib.loadMarketSettings(asset).status != Status.ACTIVE
TMP_9430(MarketSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarketSettings(bytes32), arguments:['asset_1'] 
REF_5167(Status) -> TMP_9430.status
REF_5168(Status) -> Status.ACTIVE
TMP_9431(bool) = REF_5167 != REF_5168
CONDITION TMP_9431
 revert MarketInactive()()
TMP_9432(None) = SOLIDITY_CALL revert MarketInactive()()
```
#### MarketLib.assertMaxLeverage(bytes32,uint256) [INTERNAL]
```slithir
 leverage < 1e18
TMP_9425(bool) = leverage_1 < 1000000000000000000
CONDITION TMP_9425
 revert LeverageInvalid()()
TMP_9426(None) = SOLIDITY_CALL revert LeverageInvalid()()
 leverage > StorageLib.loadMarketSettings(asset).maxOpenLeverage
TMP_9427(MarketSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarketSettings(bytes32), arguments:['asset_1'] 
REF_5165(uint256) -> TMP_9427.maxOpenLeverage
TMP_9428(bool) = leverage_1 > REF_5165
CONDITION TMP_9428
 revert MaxLeverageExceeded()()
TMP_9429(None) = SOLIDITY_CALL revert MaxLeverageExceeded()()
```
#### MarketLib.getPositionLeverage(Market,address,uint256) [INTERNAL]
```slithir
 leverage = self.position[account][subaccount].leverage
REF_5086(mapping(address => mapping(uint256 => Position))) -> self_1 (-> []).position
REF_5087(mapping(uint256 => Position)) -> REF_5086[account_1]
REF_5088(Position) -> REF_5087[subaccount_1]
REF_5089(uint256) -> REF_5088.leverage
leverage_1(uint256) := REF_5089(uint256)
 leverage == 0
TMP_9361(bool) = leverage_1 == 0
CONDITION TMP_9361
 leverage = 1e18
leverage_2(uint256) := 1000000000000000000(uint256)
leverage_3(uint256) := phi(['leverage_2', 'leverage_1'])
 leverage
RETURN leverage_3
```
#### CollateralManagerLib.withdrawFreeCollateral(CollateralManager,address,uint256) [INTERNAL]
```slithir
USDC_2(address) := phi(['USDC_0'])
 self.debitAccount(account,amount)
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.debitAccount(CollateralManager,address,uint256), arguments:['self_1 (-> [])', 'account_1', 'amount_1'] 
 USDC.safeTransfer(account,amount)
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransfer(address,address,uint256), arguments:['USDC_2', 'account_1', 'amount_1'] 
 Withdraw(account,amount)
Emit Withdraw(account_1,amount_1)
```
#### CollateralManagerLib.withdrawToSpot(CollateralManager,address,uint256,address) [INTERNAL]
```slithir
USDC_3(address) := phi(['USDC_0'])
 self.debitAccount(account,amount)
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.debitAccount(CollateralManager,address,uint256), arguments:['self_1 (-> [])', 'account_1', 'amount_1'] 
 USDC.safeTransfer(accountManager,amount)
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransfer(address,address,uint256), arguments:['USDC_3', 'accountManager_1', 'amount_1'] 
 Withdraw(account,amount)
Emit Withdraw(account_1,amount_1)
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
#### ClearingHouseLib._getPositions(ClearingHouse,DynamicArrayLib.DynamicArray,address,uint256,bool) [INTERNAL]
```slithir
self_1 (-> [])(ClearingHouse) := phi(['self_1 (-> [])', 'self_1 (-> [])', 'self_1 (-> [])'])
assets_1(DynamicArrayLib.DynamicArray) := phi(['assets_1', 'REF_4439', 'REF_4698'])
account_1(address) := phi(['REF_4440', 'REF_4699', 'account_1'])
subaccount_1(uint256) := phi(['subaccount_1', 'REF_4700', 'REF_4441'])
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
#### MarketLib.realizeFundingPayment(bytes32,Position) [INTERNAL]
```slithir
 position.realizeFundingPayment(StorageLib.loadFundingRateEngine(asset).getCumulativeFunding())
TMP_9316(FundingRateEngine) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadFundingRateEngine(bytes32), arguments:['asset_1'] 
TMP_9317(int256) = LIBRARY_CALL, dest:FundingLib, function:FundingLib.getCumulativeFunding(FundingRateEngine), arguments:['TMP_9316'] 
TMP_9318(int256) = LIBRARY_CALL, dest:PositionLib, function:PositionLib.realizeFundingPayment(Position,int256), arguments:['position_1', 'TMP_9317'] 
RETURN TMP_9318
 fundingPayment
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
#### DynamicArrayLib.length(DynamicArrayLib.DynamicArray) [INTERNAL]
```slithir
 a.data.length
REF_5750(uint256[]) -> a_1.data
REF_5751 -> LENGTH REF_5750
RETURN REF_5751
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

#### CollateralManagerLib.creditAccount(CollateralManager,address,uint256) [INTERNAL]
```slithir
 self.freeCollateral[account] += amount
REF_4832(mapping(address => uint256)) -> self_1 (-> []).freeCollateral
REF_4833(uint256) -> REF_4832[account_1]
self_2 (-> [])(CollateralManager) := phi(['self_1 (-> [])'])
REF_4833(-> self_2 (-> [])) = REF_4833 (c)+ amount_1
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
#### CLOBLib.amend(address,AmendLimitOrderArgs,BookType) [INTERNAL]
```slithir
 ds = _getStorage(args.asset,bookType)
REF_4024(bytes32) -> args_1.asset
TMP_8696(Book) = INTERNAL_CALL, CLOBLib._getStorage(bytes32,BookType)(REF_4024,bookType_1)
ds_1 (-> ['TMP_8696'])(Book) := TMP_8696(Book)
 order = ds.orders[args.orderId.wrap()]
REF_4025(mapping(OrderId => Order)) -> ds_1 (-> ['TMP_8696']).orders
REF_4026(uint256) -> args_1.orderId
TMP_8697(OrderId) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.wrap(uint256), arguments:['REF_4026'] 
REF_4028(Order) -> REF_4025[TMP_8697]
order_1 (-> ['ds'])(Order) := REF_4028(Order)
 order.id.unwrap() == 0
REF_4029(OrderId) -> order_1 (-> ['ds']).id
TMP_8698(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_4029'] 
TMP_8699(bool) = TMP_8698 == 0
CONDITION TMP_8699
 OrderLib.OrderNotFound()
TMP_8700(None) = SOLIDITY_CALL revert OrderNotFound()()
 order.owner != account
REF_4031(address) -> order_1 (-> ['ds']).owner
TMP_8701(bool) = REF_4031 != account_1
CONDITION TMP_8701
 revert UnauthorizedAmend()()
TMP_8702(None) = SOLIDITY_CALL revert UnauthorizedAmend()()
 order.subaccount != args.subaccount
REF_4032(uint256) -> order_1 (-> ['ds']).subaccount
REF_4033(uint256) -> args_1.subaccount
TMP_8703(bool) = REF_4032 != REF_4033
CONDITION TMP_8703
 revert IncorrectSubaccount()()
TMP_8704(None) = SOLIDITY_CALL revert IncorrectSubaccount()()
 ds.assertLimitPriceInBounds(args.price)
REF_4035(uint256) -> args_1.price
LIBRARY_CALL, dest:BookLib, function:BookLib.assertLimitPriceInBounds(Book,uint256), arguments:["ds_1 (-> ['TMP_8696'])", 'REF_4035'] 
 ds.assertLimitOrderAmountInBounds(args.baseAmount)
REF_4037(uint256) -> args_1.baseAmount
LIBRARY_CALL, dest:BookLib, function:BookLib.assertLimitOrderAmountInBounds(Book,uint256), arguments:["ds_1 (-> ['TMP_8696'])", 'REF_4037'] 
 (notionalDelta,collateralDelta) = _processAmend(ds,order,args)
TUPLE_82(int256,int256) = INTERNAL_CALL, CLOBLib._processAmend(Book,Order,AmendLimitOrderArgs)(ds_1 (-> ['TMP_8696']),order_1 (-> ['ds']),args_1)
notionalDelta_1(int256)= UNPACK TUPLE_82 index: 0 
collateralDelta_1(int256)= UNPACK TUPLE_82 index: 1 
 _updateOrderbookNotional(args.asset,account,args.subaccount,notionalDelta)
REF_4038(bytes32) -> args_1.asset
REF_4039(uint256) -> args_1.subaccount
INTERNAL_CALL, CLOBLib._updateOrderbookNotional(bytes32,address,uint256,int256)(REF_4038,account_1,REF_4039,notionalDelta_1)
 OrderAmended(args.asset,args.orderId,order,collateralDelta,ds.config.bookType,StorageLib.incNonce())
REF_4040(bytes32) -> args_1.asset
REF_4041(uint256) -> args_1.orderId
REF_4042(BookConfig) -> ds_1 (-> ['TMP_8696']).config
REF_4043(BookType) -> REF_4042.bookType
TMP_8708(uint256) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.incNonce(), arguments:[] 
Emit OrderAmended(REF_4040,REF_4041,order_1 (-> ['ds']),collateralDelta_1,REF_4043,TMP_8708)
 collateralDelta
RETURN collateralDelta_1
```
#### MarketLib._validateReduceOnlyOrder(Market,address,uint256,uint256,Side,bool) [INTERNAL]
```slithir
self_1 (-> [])(Market) := phi(['self_1 (-> [])', 'self_1 (-> [])'])
account_1(address) := phi(['account_1', 'account_1'])
subaccount_1(uint256) := phi(['REF_4964', 'REF_4957'])
orderAmount_1(uint256) := phi(['REF_4965', 'REF_4958'])
side_1(Side) := phi(['REF_4959', 'REF_4966'])
baseDenominated_1(bool) := phi(['REF_4960'])
 ! baseDenominated
TMP_9433 = UnaryType.BANG baseDenominated_1 
CONDITION TMP_9433
 revert InvalidReduceOnlyDenomination()()
TMP_9434(None) = SOLIDITY_CALL revert InvalidReduceOnlyDenomination()()
 position = self.position[account][subaccount]
REF_5169(mapping(address => mapping(uint256 => Position))) -> self_1 (-> []).position
REF_5170(mapping(uint256 => Position)) -> REF_5169[account_1]
REF_5171(Position) -> REF_5170[subaccount_1]
position_1 (-> ['self'])(Position) := REF_5171(Position)
 position.amount < orderAmount
REF_5172(uint256) -> position_1 (-> ['self']).amount
TMP_9435(bool) = REF_5172 < orderAmount_1
CONDITION TMP_9435
 revert NotReduceOnly()()
TMP_9436(None) = SOLIDITY_CALL revert NotReduceOnly()()
 side == Side.BUY && position.isLong
REF_5173(Side) -> Side.BUY
TMP_9437(bool) = side_1 == REF_5173
REF_5174(bool) -> position_1 (-> ['self']).isLong
TMP_9438(bool) = TMP_9437 && REF_5174
CONDITION TMP_9438
 revert NotReduceOnly()()
TMP_9439(None) = SOLIDITY_CALL revert NotReduceOnly()()
 side == Side.SELL && ! position.isLong
REF_5175(Side) -> Side.SELL
TMP_9440(bool) = side_1 == REF_5175
REF_5176(bool) -> position_1 (-> ['self']).isLong
TMP_9441 = UnaryType.BANG REF_5176 
TMP_9442(bool) = TMP_9440 && TMP_9441
CONDITION TMP_9442
 revert NotReduceOnly()()
TMP_9443(None) = SOLIDITY_CALL revert NotReduceOnly()()
```
#### CLOBLib._executeCancel(Book,address,uint256,uint256[]) [INTERNAL]
```slithir
ds_1 (-> ['TMP_8710'])(Book) := phi(["ds_1 (-> ['TMP_8710'])"])
account_1(address) := phi(['account_1'])
subaccount_1(uint256) := phi(['subaccount_1'])
orderIds_1(uint256[]) := phi(['orderIds_1'])
 asset = ds.config.asset
REF_4237(BookConfig) -> ds_1 (-> ['TMP_8710']).config
REF_4238(bytes32) -> REF_4237.asset
asset_1(bytes32) := REF_4238(bytes32)
 bookType = ds.config.bookType
REF_4239(BookConfig) -> ds_1 (-> ['TMP_8710']).config
REF_4240(BookType) -> REF_4239.bookType
bookType_1(BookType) := REF_4240(BookType)
 numOrders = orderIds.length
REF_4241 -> LENGTH orderIds_1
numOrders_1(uint256) := REF_4241(uint256)
 i < numOrders
i_1(uint256) := phi(['i_0', 'i_2'])
TMP_8832(bool) = i_1 < numOrders_1
CONDITION TMP_8832
 orderId = orderIds[i]
REF_4242(uint256) -> orderIds_1[i_1]
orderId_1(uint256) := REF_4242(uint256)
 order = ds.orders[orderId.wrap()]
REF_4243(mapping(OrderId => Order)) -> ds_1 (-> ['TMP_8710']).orders
TMP_8833(OrderId) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.wrap(uint256), arguments:['orderId_1'] 
REF_4245(Order) -> REF_4243[TMP_8833]
order_1 (-> ['ds'])(Order) := REF_4245(Order)
 order.isNull()
TMP_8834(bool) = LIBRARY_CALL, dest:OrderLib, function:OrderLib.isNull(Order), arguments:["order_1 (-> ['ds'])"] 
CONDITION TMP_8834
 CancelFailed(asset,orderId,account,bookType,StorageLib.incNonce())
TMP_8835(uint256) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.incNonce(), arguments:[] 
Emit CancelFailed(asset_1,orderId_1,account_1,bookType_1,TMP_8835)
 order.owner != account
REF_4248(address) -> order_1 (-> ['ds']).owner
TMP_8837(bool) = REF_4248 != account_1
CONDITION TMP_8837
 revert UnauthorizedCancel()()
TMP_8838(None) = SOLIDITY_CALL revert UnauthorizedCancel()()
 order.subaccount != subaccount
REF_4249(uint256) -> order_1 (-> ['ds']).subaccount
TMP_8839(bool) = REF_4249 != subaccount_1
CONDITION TMP_8839
 revert IncorrectSubaccount()()
TMP_8840(None) = SOLIDITY_CALL revert IncorrectSubaccount()()
 ! order.reduceOnly
REF_4250(bool) -> order_1 (-> ['ds']).reduceOnly
TMP_8841 = UnaryType.BANG REF_4250 
CONDITION TMP_8841
 quoteAmount = order.amount.fullMulDiv(order.price,1e18)
REF_4251(uint256) -> order_1 (-> ['ds']).amount
REF_4253(uint256) -> order_1 (-> ['ds']).price
TMP_8842(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_4251', 'REF_4253', '1000000000000000000'] 
quoteAmount_1(uint256) := TMP_8842(uint256)
 collateralRefunded = quoteAmount.fullMulDiv(1e18,_getLeverage(asset,account,subaccount))
TMP_8843(uint256) = INTERNAL_CALL, CLOBLib._getLeverage(bytes32,address,uint256)(asset_1,account_1,subaccount_1)
TMP_8844(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['quoteAmount_1', '1000000000000000000', 'TMP_8843'] 
collateralRefunded_1(uint256) := TMP_8844(uint256)
 _updateOrderbookNotional(asset,account,subaccount,- quoteAmount.toInt256())
TMP_8845(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['quoteAmount_1'] 
TMP_8846(int256) = 0 (c)- TMP_8845
INTERNAL_CALL, CLOBLib._updateOrderbookNotional(bytes32,address,uint256,int256)(asset_1,account_1,subaccount_1,TMP_8846)
collateralRefunded_2(uint256) := phi(['collateralRefunded_1', 'collateralRefunded_0'])
 OrderCanceled(asset,orderId,account,subaccount,collateralRefunded,bookType,StorageLib.incNonce())
TMP_8848(uint256) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.incNonce(), arguments:[] 
Emit OrderCanceled(asset_1,orderId_1,account_1,subaccount_1,collateralRefunded_2,bookType_1,TMP_8848)
 totalCollateralRefunded += collateralRefunded
totalCollateralRefunded_1(uint256) = totalCollateralRefunded_0 (c)+ collateralRefunded_2
 ds.removeOrderFromBook(order)
LIBRARY_CALL, dest:BookLib, function:BookLib.removeOrderFromBook(Book,Order), arguments:["ds_1 (-> ['TMP_8710'])", "order_1 (-> ['ds'])"] 
 ++ i
totalCollateralRefunded_2(uint256) := phi(['totalCollateralRefunded_0', 'totalCollateralRefunded_1'])
i_2(uint256) = i_1 (c)+ 1
 totalCollateralRefunded
RETURN totalCollateralRefunded_0
```
#### CLOBLib._getStorage(bytes32,BookType) [INTERNAL]
```slithir
asset_1(bytes32) := phi(['asset_1', 'REF_4024', 'REF_4000', 'asset_1'])
bookType_1(BookType) := phi(['REF_3975', 'bookType_1', 'bookType_1', 'bookType_1', 'REF_3992'])
 StorageLib.loadBook(asset,bookType)
TMP_8924(Book) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadBook(bytes32,BookType), arguments:['asset_1', 'bookType_1'] 
RETURN TMP_8924
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
#### ClearingHouseLib._getCollateral(uint256,uint256,uint256) [PRIVATE]
```slithir
baseAmount_1(uint256) := phi(['REF_4411'])
price_1(uint256) := phi(['REF_4412'])
leverage_1(uint256) := phi(['TMP_8929'])
 collateral = baseAmount.fullMulDiv(price,1e18).fullMulDiv(1e18,leverage)
TMP_9068(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['baseAmount_1', 'price_1', '1000000000000000000'] 
TMP_9069(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['TMP_9068', '1000000000000000000', 'leverage_1'] 
collateral_1(uint256) := TMP_9069(uint256)
 collateral
RETURN collateral_1
```
#### ClearingHouseLib._processTakerFill(ClearingHouse,ClearingHouseLib.__FillParams__) [INTERNAL]
```slithir
self_1 (-> [])(ClearingHouse) := phi(['self_1 (-> [])'])
params_1(ClearingHouseLib.__FillParams__) := phi(['TMP_8935'])
 cache.assets = self.assets[params.account][params.subaccount].values().wrap()
REF_4681(DynamicArrayLib.DynamicArray) -> cache_0.assets
REF_4682(mapping(address => mapping(uint256 => EnumerableSetLib.Bytes32Set))) -> self_1 (-> []).assets
REF_4683(address) -> params_1.account
REF_4684(mapping(uint256 => EnumerableSetLib.Bytes32Set)) -> REF_4682[REF_4683]
REF_4685(uint256) -> params_1.subaccount
REF_4686(EnumerableSetLib.Bytes32Set) -> REF_4684[REF_4685]
TMP_9079(bytes32[]) = LIBRARY_CALL, dest:EnumerableSetLib, function:EnumerableSetLib.values(EnumerableSetLib.Bytes32Set), arguments:['REF_4686'] 
TMP_9080(DynamicArrayLib.DynamicArray) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.wrap(bytes32[]), arguments:['TMP_9079'] 
cache_1(ClearingHouseLib.__ProcessTakerFillCache__) := phi(['cache_0'])
REF_4681(DynamicArrayLib.DynamicArray) (->cache_1) := TMP_9080(DynamicArrayLib.DynamicArray)
 isNewPosition = ! cache.assets.contains(params.asset)
REF_4689(DynamicArrayLib.DynamicArray) -> cache_1.assets
REF_4691(bytes32) -> params_1.asset
TMP_9081(bool) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.contains(DynamicArrayLib.DynamicArray,bytes32), arguments:['REF_4689', 'REF_4691'] 
TMP_9082 = UnaryType.BANG TMP_9081 
isNewPosition_1(bool) := TMP_9082(bool)
 isNewPosition
CONDITION isNewPosition_1
 ! _assetCanBeAddedToAccount(cache.assets,params.asset)
REF_4692(DynamicArrayLib.DynamicArray) -> cache_1.assets
REF_4693(bytes32) -> params_1.asset
TMP_9083(bool) = INTERNAL_CALL, ClearingHouseLib._assetCanBeAddedToAccount(DynamicArrayLib.DynamicArray,bytes32)(REF_4692,REF_4693)
TMP_9084 = UnaryType.BANG TMP_9083 
CONDITION TMP_9084
 revert CrossMarginIsDisabled()()
TMP_9085(None) = SOLIDITY_CALL revert CrossMarginIsDisabled()()
 cache.assets.p(params.asset)
REF_4694(DynamicArrayLib.DynamicArray) -> cache_1.assets
REF_4696(bytes32) -> params_1.asset
TMP_9086(DynamicArrayLib.DynamicArray) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.p(DynamicArrayLib.DynamicArray,bytes32), arguments:['REF_4694', 'REF_4696'] 
 cache.positions = _getPositions(self,cache.assets,params.account,params.subaccount,isNewPosition)
REF_4697(Position[]) -> cache_1.positions
REF_4698(DynamicArrayLib.DynamicArray) -> cache_1.assets
REF_4699(address) -> params_1.account
REF_4700(uint256) -> params_1.subaccount
TMP_9087(Position[]) = INTERNAL_CALL, ClearingHouseLib._getPositions(ClearingHouse,DynamicArrayLib.DynamicArray,address,uint256,bool)(self_1 (-> []),REF_4698,REF_4699,REF_4700,isNewPosition_1)
cache_2(ClearingHouseLib.__ProcessTakerFillCache__) := phi(['cache_1'])
REF_4697(Position[]) (->cache_2) := TMP_9087(Position[])
 cache.fundingPayment = realizeFundingPayment(cache.assets,cache.positions)
REF_4701(int256) -> cache_2.fundingPayment
REF_4702(DynamicArrayLib.DynamicArray) -> cache_2.assets
REF_4703(Position[]) -> cache_2.positions
TMP_9088(int256) = INTERNAL_CALL, ClearingHouseLib.realizeFundingPayment(DynamicArrayLib.DynamicArray,Position[])(REF_4702,REF_4703)
cache_3(ClearingHouseLib.__ProcessTakerFillCache__) := phi(['cache_2'])
REF_4701(int256) (->cache_3) := TMP_9088(int256)
 positionIdx = cache.assets.indexOf(params.asset)
REF_4704(DynamicArrayLib.DynamicArray) -> cache_3.assets
REF_4706(bytes32) -> params_1.asset
TMP_9089(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.indexOf(DynamicArrayLib.DynamicArray,bytes32), arguments:['REF_4704', 'REF_4706'] 
positionIdx_1(uint256) := TMP_9089(uint256)
 cache.positionResult = cache.positions[positionIdx].processTrade({side:params.side,quoteTraded:params.quoteAmount,baseTraded:params.baseAmount})
REF_4707(PositionUpdateResult) -> cache_3.positionResult
REF_4708(Position[]) -> cache_3.positions
REF_4709(Position) -> REF_4708[positionIdx_1]
REF_4711(Side) -> params_1.side
REF_4712(uint256) -> params_1.quoteAmount
REF_4713(uint256) -> params_1.baseAmount
TMP_9090(PositionUpdateResult) = LIBRARY_CALL, dest:PositionLib, function:PositionLib.processTrade(Position,Side,uint256,uint256), arguments:['REF_4709', 'REF_4711', 'REF_4712', 'REF_4713'] 
cache_4(ClearingHouseLib.__ProcessTakerFillCache__) := phi(['cache_3'])
REF_4707(PositionUpdateResult) (->cache_4) := TMP_9090(PositionUpdateResult)
 cache.takerFee = StorageLib.loadFeeManager().getTakerFee(params.account,params.quoteAmount)
REF_4714(uint256) -> cache_4.takerFee
TMP_9091(FeeManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadFeeManager(), arguments:[] 
REF_4717(address) -> params_1.account
REF_4718(uint256) -> params_1.quoteAmount
TMP_9092(uint256) = LIBRARY_CALL, dest:FeeManagerLib, function:FeeManagerLib.getTakerFee(FeeManager,address,uint256), arguments:['TMP_9091', 'REF_4717', 'REF_4718'] 
cache_5(ClearingHouseLib.__ProcessTakerFillCache__) := phi(['cache_4'])
REF_4714(uint256) (->cache_5) := TMP_9092(uint256)
 cache.margin = StorageLib.loadCollateralManager().getMarginBalance(params.account,params.subaccount)
REF_4719(int256) -> cache_5.margin
TMP_9093(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
REF_4722(address) -> params_1.account
REF_4723(uint256) -> params_1.subaccount
TMP_9094(int256) = LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.getMarginBalance(CollateralManager,address,uint256), arguments:['TMP_9093', 'REF_4722', 'REF_4723'] 
cache_6(ClearingHouseLib.__ProcessTakerFillCache__) := phi(['cache_5'])
REF_4719(int256) (->cache_6) := TMP_9094(int256)
 cache.margin += cache.positionResult.rpnl - cache.fundingPayment - cache.takerFee.toInt256()
REF_4724(int256) -> cache_6.margin
REF_4725(PositionUpdateResult) -> cache_6.positionResult
REF_4726(int256) -> REF_4725.rpnl
REF_4727(int256) -> cache_6.fundingPayment
TMP_9095(int256) = REF_4726 (c)- REF_4727
REF_4728(uint256) -> cache_6.takerFee
TMP_9096(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_4728'] 
TMP_9097(int256) = TMP_9095 (c)- TMP_9096
cache_7(ClearingHouseLib.__ProcessTakerFillCache__) := phi(['cache_6'])
REF_4724(-> cache_7) = REF_4724 (c)+ TMP_9097
 (cache.margin,cache.positionResult.marginDelta) = self.rebalanceAccount({assets:cache.assets,positions:cache.positions,margin:cache.margin,marginDelta:cache.positionResult.marginDelta})
REF_4730(int256) -> cache_7.margin
REF_4731(PositionUpdateResult) -> cache_7.positionResult
REF_4732(int256) -> REF_4731.marginDelta
REF_4734(DynamicArrayLib.DynamicArray) -> cache_7.assets
REF_4735(Position[]) -> cache_7.positions
REF_4736(int256) -> cache_7.margin
REF_4737(PositionUpdateResult) -> cache_7.positionResult
REF_4738(int256) -> REF_4737.marginDelta
TUPLE_99(int256,int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.rebalanceAccount(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256,int256), arguments:['self_1 (-> [])', 'REF_4734', 'REF_4735', 'REF_4736', 'REF_4738'] 
REF_4730(int256)= UNPACK TUPLE_99 index: 0 
REF_4732(int256)= UNPACK TUPLE_99 index: 1 
 self.assertNotLiquidatable(cache.assets,cache.positions,cache.margin)
REF_4740(DynamicArrayLib.DynamicArray) -> cache_7.assets
REF_4741(Position[]) -> cache_7.positions
REF_4742(int256) -> cache_7.margin
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.assertNotLiquidatable(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256), arguments:['self_1 (-> [])', 'REF_4740', 'REF_4741', 'REF_4742'] 
 StorageLib.loadInsuranceFund().pay(cache.takerFee)
TMP_9099(InsuranceFund) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadInsuranceFund(), arguments:[] 
REF_4745(uint256) -> cache_7.takerFee
LIBRARY_CALL, dest:InsuranceFundLib, function:InsuranceFundLib.pay(InsuranceFund,uint256), arguments:['TMP_9099', 'REF_4745'] 
 StorageLib.loadCollateralManager().settleFill(params.account,params.subaccount,cache.margin,cache.positionResult.marginDelta + params.collateralPosted.toInt256())
TMP_9101(CollateralManager) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadCollateralManager(), arguments:[] 
REF_4748(address) -> params_1.account
REF_4749(uint256) -> params_1.subaccount
REF_4750(int256) -> cache_7.margin
REF_4751(PositionUpdateResult) -> cache_7.positionResult
REF_4752(int256) -> REF_4751.marginDelta
REF_4753(uint256) -> params_1.collateralPosted
TMP_9102(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_4753'] 
TMP_9103(int256) = REF_4752 (c)+ TMP_9102
LIBRARY_CALL, dest:CollateralManagerLib, function:CollateralManagerLib.settleFill(CollateralManager,address,uint256,int256,int256), arguments:['TMP_9101', 'REF_4748', 'REF_4749', 'REF_4750', 'TMP_9103'] 
 self.updateAccount({account:params.account,subaccount:params.subaccount,assets:cache.assets,positions:cache.positions,tradedAsset:params.asset,positionIdx:positionIdx,oiDelta:cache.positionResult.oiDelta,sideClose:cache.positionResult.sideClose})
REF_4756(address) -> params_1.account
REF_4757(uint256) -> params_1.subaccount
REF_4758(DynamicArrayLib.DynamicArray) -> cache_7.assets
REF_4759(Position[]) -> cache_7.positions
REF_4760(bytes32) -> params_1.asset
REF_4761(PositionUpdateResult) -> cache_7.positionResult
REF_4762(OIDelta) -> REF_4761.oiDelta
REF_4763(PositionUpdateResult) -> cache_7.positionResult
REF_4764(bool) -> REF_4763.sideClose
LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.updateAccount(ClearingHouse,address,uint256,DynamicArrayLib.DynamicArray,Position[],bytes32,uint256,OIDelta,bool), arguments:['self_1 (-> [])', 'REF_4756', 'REF_4757', 'REF_4758', 'REF_4759', 'REF_4760', 'positionIdx_1', 'REF_4762', 'REF_4764']
```
#### MarketLib.placeOrder(Market,address,PlaceOrderArgs,BookType) [INTERNAL]
```slithir
 bookType == BookType.BACKSTOP && args.tif != TiF.MOC
REF_4953(BookType) -> BookType.BACKSTOP
TMP_9281(bool) = bookType_1 == REF_4953
REF_4954(TiF) -> args_1.tif
REF_4955(TiF) -> TiF.MOC
TMP_9282(bool) = REF_4954 != REF_4955
TMP_9283(bool) = TMP_9281 && TMP_9282
CONDITION TMP_9283
 revert InvalidBackstopOrder()()
TMP_9284(None) = SOLIDITY_CALL revert InvalidBackstopOrder()()
 args.reduceOnly
REF_4956(bool) -> args_1.reduceOnly
CONDITION REF_4956
 _validateReduceOnlyOrder({self:self,account:account,subaccount:args.subaccount,orderAmount:args.amount,side:args.side,baseDenominated:args.baseDenominated})
REF_4957(uint256) -> args_1.subaccount
REF_4958(uint256) -> args_1.amount
REF_4959(Side) -> args_1.side
REF_4960(bool) -> args_1.baseDenominated
INTERNAL_CALL, MarketLib._validateReduceOnlyOrder(Market,address,uint256,uint256,Side,bool)(self_1 (-> []),account_1,REF_4957,REF_4958,REF_4959,REF_4960)
 CLOBLib.placeOrder(account,args,bookType)
TMP_9286(PlaceOrderResult) = LIBRARY_CALL, dest:CLOBLib, function:CLOBLib.placeOrder(address,PlaceOrderArgs,BookType), arguments:['account_1', 'args_1', 'bookType_1'] 
RETURN TMP_9286
 onlyActiveMarket(args.asset)
REF_4962(bytes32) -> args_1.asset
MODIFIER_CALL, MarketLib.onlyActiveMarket(bytes32)(REF_4962)
 result
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
#### ClearingHouseLib.getNotionalAccountValue(ClearingHouse,DynamicArrayLib.DynamicArray,Position[]) [INTERNAL]
```slithir
 length = assets.length()
TMP_9036(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.length(DynamicArrayLib.DynamicArray), arguments:['assets_1'] 
length_1(uint256) := TMP_9036(uint256)
 i < length
totalNotional_1(uint256) := phi(['totalNotional_0', 'totalNotional_2'])
i_1(uint256) := phi(['i_2', 'i_0'])
TMP_9037(bool) = i_1 < length_1
CONDITION TMP_9037
 totalNotional += self.market[assets.getBytes32(i)].getNotionalValue(positions[i])
REF_4622(mapping(bytes32 => Market)) -> self_1 (-> []).market
TMP_9038(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
REF_4624(Market) -> REF_4622[TMP_9038]
REF_4626(Position) -> positions_1[i_1]
TMP_9039(uint256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getNotionalValue(Market,Position), arguments:['REF_4624', 'REF_4626'] 
totalNotional_2(uint256) = totalNotional_1 (c)+ TMP_9039
 ++ i
i_2(uint256) = i_1 (c)+ 1
 totalNotional
RETURN totalNotional_1
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
#### MarketLib.getIntendedMargin(Market,Position) [INTERNAL]
```slithir
 position.amount == 0
REF_5056(uint256) -> position_1.amount
TMP_9348(bool) = REF_5056 == 0
CONDITION TMP_9348
 0
RETURN 0
 currentNotional = position.amount.fullMulDiv(self.markPrice,1e18)
REF_5057(uint256) -> position_1.amount
REF_5059(uint256) -> self_1 (-> []).markPrice
TMP_9349(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_5057', 'REF_5059', '1000000000000000000'] 
currentNotional_1(uint256) := TMP_9349(uint256)
 intendedMargin = currentNotional.fullMulDiv(1e18,position.leverage)
REF_5061(uint256) -> position_1.leverage
TMP_9350(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['currentNotional_1', '1000000000000000000', 'REF_5061'] 
intendedMargin_1(uint256) := TMP_9350(uint256)
 intendedMargin
RETURN intendedMargin_1
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
#### MarketLib.getPosition(Market,address,uint256) [INTERNAL]
```slithir
 position = self.position[account][subaccount]
REF_5081(mapping(address => mapping(uint256 => Position))) -> self_1 (-> []).position
REF_5082(mapping(uint256 => Position)) -> REF_5081[account_1]
REF_5083(Position) -> REF_5082[subaccount_1]
position_1(Position) := REF_5083(Position)
 position.leverage == 0
REF_5084(uint256) -> position_1.leverage
TMP_9360(bool) = REF_5084 == 0
CONDITION TMP_9360
 position.leverage = 1e18
REF_5085(uint256) -> position_1.leverage
position_2(Position) := phi(['position_1'])
REF_5085(uint256) (->position_2) := 1000000000000000000(uint256)
position_3(Position) := phi(['position_2', 'position_1'])
 position
RETURN position_3
```
#### EnumerableSetLib._rootSlot(EnumerableSetLib.Bytes32Set) [PRIVATE]
```slithir
s_1 (-> [])(EnumerableSetLib.Bytes32Set) := phi(['set_1 (-> [])', 'set_1 (-> [])', 'set_1 (-> [])', 'set_1 (-> [])', 'set_1 (-> [])', 'set_1 (-> [])'])
_ENUMERABLE_WORD_SET_SLOT_SEED_1(uint256) := phi(['_ENUMERABLE_WORD_SET_SLOT_SEED_0'])
 mstore(uint256,uint256)(0x04,_ENUMERABLE_WORD_SET_SLOT_SEED)
TMP_12990(None) = SOLIDITY_CALL mstore(uint256,uint256)(4,_ENUMERABLE_WORD_SET_SLOT_SEED_1)
 mstore(uint256,uint256)(0x00,s)
TMP_12991(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,s_1 (-> []))
 r = keccak256(uint256,uint256)(0x00,0x24)
TMP_12992(uint256) = SOLIDITY_CALL keccak256(uint256,uint256)(0,36)
r_1(bytes32) := TMP_12992(uint256)
 r
RETURN r_1
```
#### EnumerableSetLib._toBytes32Set(EnumerableSetLib.Int256Set) [PRIVATE]
```slithir
s_1 (-> [])(EnumerableSetLib.Int256Set) := phi(['set_1 (-> [])', 'set_1 (-> [])', 'set_1 (-> [])', 'set_1 (-> [])', 'set_1 (-> [])', 'set_1 (-> [])'])
 c = s
c_1 (-> ['s'])(EnumerableSetLib.Bytes32Set) := s_1 (-> [])(EnumerableSetLib.Int256Set)
 c
RETURN c_1 (-> ['s'])
```
#### EnumerableSetLib._toInts(bytes32[]) [PRIVATE]
```slithir
a_1(bytes32[]) := phi(['TMP_12906'])
 c = a
c_1(int256[]) := a_1(bytes32[])
 c
RETURN c_1
```
#### EnumerableSetLib._toUints(bytes32[]) [PRIVATE]
```slithir
a_1(bytes32[]) := phi(['TMP_12903'])
 c = a
c_1(uint256[]) := a_1(bytes32[])
 c
RETURN c_1
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
#### BookLib.assertLimitOrderAmountInBounds(Book,uint256) [INTERNAL]
```slithir
 orderAmountInBase < StorageLib.loadBookSettings(self.config.asset).minLimitOrderAmountInBase
REF_3712(BookConfig) -> self_1 (-> []).config
REF_3713(bytes32) -> REF_3712.asset
TMP_8510(BookSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadBookSettings(bytes32), arguments:['REF_3713'] 
REF_3714(uint256) -> TMP_8510.minLimitOrderAmountInBase
TMP_8511(bool) = orderAmountInBase_1 < REF_3714
CONDITION TMP_8511
 revert LimitOrderAmountOutOfBounds()()
TMP_8512(None) = SOLIDITY_CALL revert LimitOrderAmountOutOfBounds()()
 orderAmountInBase % self.config.lotSize != 0
REF_3715(BookConfig) -> self_1 (-> []).config
REF_3716(uint256) -> REF_3715.lotSize
TMP_8513(uint256) = orderAmountInBase_1 % REF_3716
TMP_8514(bool) = TMP_8513 != 0
CONDITION TMP_8514
 revert LimitOrderAmountNotOnLotSize()()
TMP_8515(None) = SOLIDITY_CALL revert LimitOrderAmountNotOnLotSize()()
```
#### BookLib.assertLimitPriceInBounds(Book,uint256) [INTERNAL]
```slithir
 tickSize = StorageLib.loadBookSettings(self.config.asset).tickSize
REF_3702(BookConfig) -> self_1 (-> []).config
REF_3703(bytes32) -> REF_3702.asset
TMP_8497(BookSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadBookSettings(bytes32), arguments:['REF_3703'] 
REF_3704(uint256) -> TMP_8497.tickSize
tickSize_1(uint256) := REF_3704(uint256)
 price == 0 || price % tickSize != 0
TMP_8498(bool) = price_1 == 0
TMP_8499(uint256) = price_1 % tickSize_1
TMP_8500(bool) = TMP_8499 != 0
TMP_8501(bool) = TMP_8498 || TMP_8500
CONDITION TMP_8501
 revert LimitPriceOutOfBounds()()
TMP_8502(None) = SOLIDITY_CALL revert LimitPriceOutOfBounds()()
```
#### CLOBLib._processAmend(Book,Order,AmendLimitOrderArgs) [INTERNAL]
```slithir
ds_1 (-> ['TMP_8696'])(Book) := phi(["ds_1 (-> ['TMP_8696'])"])
order_1 (-> ['ds'])(Order) := phi(["order_1 (-> ['ds'])"])
args_1(AmendLimitOrderArgs) := phi(['args_1'])
 args.expiryTime.isExpired() || args.baseAmount < StorageLib.loadBookSettings(ds.config.asset).minLimitOrderAmountInBase
REF_4278(uint32) -> args_1.expiryTime
TMP_8856(bool) = LIBRARY_CALL, dest:OrderLib, function:OrderLib.isExpired(uint256), arguments:['REF_4278'] 
REF_4280(uint256) -> args_1.baseAmount
REF_4282(BookConfig) -> ds_1 (-> ['TMP_8696']).config
REF_4283(bytes32) -> REF_4282.asset
TMP_8857(BookSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadBookSettings(bytes32), arguments:['REF_4283'] 
REF_4284(uint256) -> TMP_8857.minLimitOrderAmountInBase
TMP_8858(bool) = REF_4280 < REF_4284
TMP_8859(bool) = TMP_8856 || TMP_8858
CONDITION TMP_8859
 revert InvalidAmend()()
TMP_8860(None) = SOLIDITY_CALL revert InvalidAmend()()
 order.side != args.side || order.price != args.price
REF_4285(Side) -> order_1 (-> ['ds']).side
REF_4286(Side) -> args_1.side
TMP_8861(bool) = REF_4285 != REF_4286
REF_4287(uint256) -> order_1 (-> ['ds']).price
REF_4288(uint256) -> args_1.price
TMP_8862(bool) = REF_4287 != REF_4288
TMP_8863(bool) = TMP_8861 || TMP_8862
CONDITION TMP_8863
 _executeAmendNewOrder(ds,order,args)
TUPLE_87(int256,int256) = INTERNAL_CALL, CLOBLib._executeAmendNewOrder(Book,Order,AmendLimitOrderArgs)(ds_1 (-> ['TMP_8696']),order_1 (-> ['ds']),args_1)
RETURN TUPLE_87
 _executeAmendAmount(ds,order,args)
TUPLE_88(int256,int256) = INTERNAL_CALL, CLOBLib._executeAmendAmount(Book,Order,AmendLimitOrderArgs)(ds_1 (-> ['TMP_8696']),order_1 (-> ['ds']),args_1)
RETURN TUPLE_88
 (notionalDelta,collateralDelta)
RETURN notionalDelta_0,collateralDelta_0
```
#### CLOBLib._updateOrderbookNotional(bytes32,address,uint256,int256) [PRIVATE]
```slithir
asset_1(bytes32) := phi(['asset_1', 'asset_1', 'REF_4230', 'REF_4018', 'REF_4038'])
account_1(address) := phi(['matchedOwner_1', 'account_1', 'account_1', 'account_1', 'owner_1'])
subaccount_1(uint256) := phi(['REF_4231', 'REF_4019', 'subaccount_1', 'REF_4039', 'subaccount_1'])
amount_1(int256) := phi(['TMP_8829', 'notionalDelta_1', 'TMP_8693', 'TMP_8846', 'TMP_8776'])
 StorageLib.loadMarket(asset).updateOrderbookNotional(account,subaccount,amount)
TMP_8911(Market) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarket(bytes32), arguments:['asset_1'] 
LIBRARY_CALL, dest:MarketLib, function:MarketLib.updateOrderbookNotional(Market,address,uint256,int256), arguments:['TMP_8911', 'account_1', 'subaccount_1', 'amount_1']
```
#### OrderIdLib.unwrap(OrderId) [INTERNAL]
```slithir
 uint256(OrderId.unwrap(id))
TMP_1924 = CONVERT id_1 to uint256
TMP_1925 = CONVERT TMP_1924 to uint256
RETURN TMP_1925
```
#### OrderIdLib.wrap(uint256) [INTERNAL]
```slithir
 OrderId.wrap(id)
TMP_9502 = CONVERT id_1 to OrderId
RETURN TMP_9502
```
#### BookLib.removeOrderFromBook(Book,Order) [INTERNAL]
```slithir
 order.reduceOnly
REF_3835(bool) -> order_1.reduceOnly
CONDITION REF_3835
 StorageLib.loadMarket(self.config.asset).unlinkReduceOnlyOrder(order.owner,order.subaccount,order.id.unwrap(),self.config.bookType)
REF_3837(BookConfig) -> self_1 (-> []).config
REF_3838(bytes32) -> REF_3837.asset
TMP_8604(Market) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarket(bytes32), arguments:['REF_3838'] 
REF_3840(address) -> order_1.owner
REF_3841(uint256) -> order_1.subaccount
REF_3842(OrderId) -> order_1.id
TMP_8605(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_3842'] 
REF_3844(BookConfig) -> self_1 (-> []).config
REF_3845(BookType) -> REF_3844.bookType
LIBRARY_CALL, dest:MarketLib, function:MarketLib.unlinkReduceOnlyOrder(Market,address,uint256,uint256,BookType), arguments:['TMP_8604', 'REF_3840', 'REF_3841', 'TMP_8605', 'REF_3845'] 
 _updateLimitRemoveOrder(self,order)
INTERNAL_CALL, BookLib._updateLimitRemoveOrder(Book,Order)(self_1 (-> []),order_1)
 _updateBookRemoveOrder(self,order)
INTERNAL_CALL, BookLib._updateBookRemoveOrder(Book,Order)(self_1 (-> []),order_1)
```
#### CLOBLib._getLeverage(bytes32,address,uint256) [INTERNAL]
```slithir
asset_1(bytes32) := phi(['asset_1', 'REF_4290', 'asset_1', 'REF_4317'])
account_1(address) := phi(['account_1', 'owner_1', 'REF_4318', 'REF_4291'])
subaccount_1(uint256) := phi(['REF_4319', 'subaccount_1', 'REF_4292', 'subaccount_1'])
 StorageLib.loadMarket(asset).getPositionLeverage(account,subaccount)
TMP_8913(Market) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarket(bytes32), arguments:['asset_1'] 
TMP_8914(uint256) = LIBRARY_CALL, dest:MarketLib, function:MarketLib.getPositionLeverage(Market,address,uint256), arguments:['TMP_8913', 'account_1', 'subaccount_1'] 
RETURN TMP_8914
```
#### OrderLib.isNull(Order) [INTERNAL]
```slithir
 self.id.unwrap() == NULL_ORDER_ID
REF_5307(OrderId) -> self_1.id
TMP_9516(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_5307'] 
TMP_9517(bool) = TMP_9516 == NULL_ORDER_ID
RETURN TMP_9517
```
#### StorageLib.loadBook(bytes32,BookType) [INTERNAL]
```slithir
PERP_CLOB_SLOT_3(bytes32) := phi(['PERP_CLOB_SLOT_0'])
 assetSlot = keccak256(bytes)(abi.encode(uint256(keccak256(bytes)(abi.encode(asset))) - 1)) & ~ bytes32(uint256(0xff))
TMP_9675(bytes) = SOLIDITY_CALL abi.encode()(asset_1)
TMP_9676(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_9675)
TMP_9677 = CONVERT TMP_9676 to uint256
TMP_9678(uint256) = TMP_9677 (c)- 1
TMP_9679(bytes) = SOLIDITY_CALL abi.encode()(TMP_9678)
TMP_9680(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_9679)
TMP_9681 = CONVERT 255 to uint256
TMP_9682 = CONVERT TMP_9681 to bytes32
TMP_9683 = UnaryType.TILD TMP_9682 
TMP_9684(bytes32) = TMP_9680 & TMP_9683
assetSlot_1(bytes32) := TMP_9684(bytes32)
 slot = keccak256(bytes)(abi.encode(bookType,assetSlot,PERP_CLOB_SLOT))
TMP_9685(bytes) = SOLIDITY_CALL abi.encode()(bookType_1,assetSlot_1,PERP_CLOB_SLOT_3)
TMP_9686(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_9685)
slot_1(bytes32) := TMP_9686(bytes32)
 ds = slot
ds_1 (-> ['slot'])(Book) := slot_1(bytes32)
 ds
RETURN ds_1 (-> ['slot'])
```
#### ClearingHouseLib._assetCanBeAddedToAccount(DynamicArrayLib.DynamicArray,bytes32) [PRIVATE]
```slithir
assets_1(DynamicArrayLib.DynamicArray) := phi(['REF_4433', 'REF_4692'])
asset_1(bytes32) := phi(['REF_4693', 'REF_4434'])
 numPositions = assets.length()
TMP_9125(uint256) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.length(DynamicArrayLib.DynamicArray), arguments:['assets_1'] 
numPositions_1(uint256) := TMP_9125(uint256)
 numPositions == 0
TMP_9126(bool) = numPositions_1 == 0
CONDITION TMP_9126
 true
RETURN True
 assets.contains(asset)
TMP_9127(bool) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.contains(DynamicArrayLib.DynamicArray,bytes32), arguments:['assets_1', 'asset_1'] 
CONDITION TMP_9127
 true
RETURN True
 ! StorageLib.loadMarketSettings(asset).crossMarginEnabled
TMP_9128(MarketSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarketSettings(bytes32), arguments:['asset_1'] 
REF_4791(bool) -> TMP_9128.crossMarginEnabled
TMP_9129 = UnaryType.BANG REF_4791 
CONDITION TMP_9129
 false
RETURN False
 i < numPositions
i_1(uint256) := phi(['i_2', 'i_0'])
TMP_9130(bool) = i_1 < numPositions_1
CONDITION TMP_9130
 ! StorageLib.loadMarketSettings(assets.getBytes32(i)).crossMarginEnabled
TMP_9131(bytes32) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.getBytes32(DynamicArrayLib.DynamicArray,uint256), arguments:['assets_1', 'i_1'] 
TMP_9132(MarketSettings) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarketSettings(bytes32), arguments:['TMP_9131'] 
REF_4794(bool) -> TMP_9132.crossMarginEnabled
TMP_9133 = UnaryType.BANG REF_4794 
CONDITION TMP_9133
 false
RETURN False
 ++ i
i_2(uint256) = i_1 (c)+ 1
 true
RETURN True
 canBeAdded
```
#### ClearingHouseLib.rebalanceAccount(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256,int256) [INTERNAL]
```slithir
 marginDelta >= 0
TMP_8987(bool) = marginDelta_1 >= 0
CONDITION TMP_8987
 self.rebalanceOpen({assets:assets,positions:positions,margin:margin,marginDelta:marginDelta})
TUPLE_92(int256,int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.rebalanceOpen(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256,int256), arguments:['self_1 (-> [])', 'assets_1', 'positions_1', 'margin_1', 'marginDelta_1'] 
RETURN TUPLE_92
 self.rebalanceClose({assets:assets,positions:positions,margin:margin,marginDelta:marginDelta})
TUPLE_93(int256,int256) = LIBRARY_CALL, dest:ClearingHouseLib, function:ClearingHouseLib.rebalanceClose(ClearingHouse,DynamicArrayLib.DynamicArray,Position[],int256,int256), arguments:['self_1 (-> [])', 'assets_1', 'positions_1', 'margin_1', 'marginDelta_1'] 
RETURN TUPLE_93
 (finalMargin,finalMarginDelta)
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
#### DynamicArrayLib.contains(DynamicArrayLib.DynamicArray,bytes32) [INTERNAL]
```slithir
 ~ indexOf(a.data,uint256(needle),0) != 0
REF_5761(uint256[]) -> a_1.data
TMP_12414 = CONVERT needle_1 to uint256
TMP_12415(uint256) = INTERNAL_CALL, DynamicArrayLib.indexOf(uint256[],uint256,uint256)(REF_5761,TMP_12414,0)
TMP_12416 = UnaryType.TILD TMP_12415 
TMP_12417(bool) = TMP_12416 != 0
RETURN TMP_12417
```
#### DynamicArrayLib.indexOf(DynamicArrayLib.DynamicArray,bytes32) [INTERNAL]
```slithir
 indexOf(a.data,uint256(needle),0)
REF_5767(uint256[]) -> a_1.data
TMP_12426 = CONVERT needle_1 to uint256
TMP_12427(uint256) = INTERNAL_CALL, DynamicArrayLib.indexOf(uint256[],uint256,uint256)(REF_5767,TMP_12426,0)
RETURN TMP_12427
```
#### DynamicArrayLib.p(bytes32) [INTERNAL]
```slithir
 p(result,uint256(data))
TMP_12299 = CONVERT data_1 to uint256
TMP_12300(DynamicArrayLib.DynamicArray) = INTERNAL_CALL, DynamicArrayLib.p(DynamicArrayLib.DynamicArray,uint256)(result_0,TMP_12299)
 result
RETURN result_0
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
#### MarketLib.getNotionalValue(Market,Position) [INTERNAL]
```slithir
 position.amount.fullMulDiv(self.markPrice,1e18)
REF_5053(uint256) -> position_1.amount
REF_5055(uint256) -> self_1 (-> []).markPrice
TMP_9347(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_5053', 'REF_5055', '1000000000000000000'] 
RETURN TMP_9347
 notional
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
