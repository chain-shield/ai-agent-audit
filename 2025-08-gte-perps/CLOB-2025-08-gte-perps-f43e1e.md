











#### CLOB.__CLOB_init(MarketConfig,MarketSettings,address) [INTERNAL]
```slithir
marketConfig_1(MarketConfig) := phi(['marketConfig_1'])
marketSettings_1(MarketSettings) := phi(['marketSettings_1'])
initialOwner_1(address) := phi(['initialOwner_1'])
 __Ownable_init(initialOwner)
INTERNAL_CALL, OwnableUpgradeable.__Ownable_init(address)(initialOwner_1)
 CLOBStorageLib.init(_getStorage(),marketConfig,marketSettings)
TMP_1061(Book) = INTERNAL_CALL, CLOB._getStorage()()
LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.init(Book,MarketConfig,MarketSettings), arguments:['TMP_1061', 'marketConfig_1', 'marketSettings_1']
```
#### CLOB._boundMakerToLotSize(Book,Order,uint256) [INTERNAL]
```slithir
ds_1 (-> [])(Book) := phi(["ds_1 (-> ['TMP_780', 'TMP_780'])"])
order_1 (-> [])(Order) := phi(["makerOrder_1 (-> ['ds', 'ds'])"])
lotSize_1(uint256) := phi(['lotSize_1'])
 remainder = order.amount % lotSize
REF_442(uint256) -> order_1 (-> []).amount
TMP_969(uint256) = REF_442 % lotSize_1
remainder_1(uint256) := TMP_969(uint256)
 remainder == 0
TMP_970(bool) = remainder_1 == 0
CONDITION TMP_970
 remainder == order.amount
REF_443(uint256) -> order_1 (-> []).amount
TMP_971(bool) = remainder_1 == REF_443
CONDITION TMP_971
 order.side == Side.BUY
REF_444(Side) -> order_1 (-> []).side
REF_445(Side) -> Side.BUY
TMP_972(bool) = REF_444 == REF_445
CONDITION TMP_972
 _removeExpiredBid(ds,order)
INTERNAL_CALL, CLOB._removeExpiredBid(Book,Order)(ds_1 (-> []),order_1 (-> []))
 _removeExpiredAsk(ds,order)
INTERNAL_CALL, CLOB._removeExpiredAsk(Book,Order)(ds_1 (-> []),order_1 (-> []))
 order.side == Side.BUY
REF_446(Side) -> order_1 (-> []).side
REF_447(Side) -> Side.BUY
TMP_975(bool) = REF_446 == REF_447
CONDITION TMP_975
 quoteTokenAmount = ds.getQuoteTokenAmount(order.price,remainder)
REF_449(uint256) -> order_1 (-> []).price
TMP_976(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getQuoteTokenAmount(Book,uint256,uint256), arguments:['ds_1 (-> [])', 'REF_449', 'remainder_1'] 
quoteTokenAmount_1(uint256) := TMP_976(uint256)
 TransientMakerData.addQuoteToken(order.owner,quoteTokenAmount)
REF_451(address) -> order_1 (-> []).owner
LIBRARY_CALL, dest:TransientMakerData, function:TransientMakerData.addQuoteToken(address,uint256), arguments:['REF_451', 'quoteTokenAmount_1'] 
 ds.metadata().quoteTokenOpenInterest -= quoteTokenAmount
TMP_978(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:['ds_1 (-> [])'] 
REF_453(uint256) -> TMP_978.quoteTokenOpenInterest
REF_453(-> TMP_978) = REF_453 (c)- quoteTokenAmount_1
 TransientMakerData.addBaseToken(order.owner,remainder)
REF_455(address) -> order_1 (-> []).owner
LIBRARY_CALL, dest:TransientMakerData, function:TransientMakerData.addBaseToken(address,uint256), arguments:['REF_455', 'remainder_1'] 
 ds.metadata().baseTokenOpenInterest -= remainder
TMP_980(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:['ds_1 (-> [])'] 
REF_457(uint256) -> TMP_980.baseTokenOpenInterest
REF_457(-> TMP_980) = REF_457 (c)- remainder_1
 order.amount -= remainder
REF_458(uint256) -> order_1 (-> []).amount
order_2 (-> [])(Order) := phi(['order_1 (-> [])'])
REF_458(-> order_2 (-> [])) = REF_458 (c)- remainder_1
```
#### CLOB._executeAmendAmount(Book,Order,uint256,uint32) [INTERNAL]
```slithir
ds_1 (-> ['TMP_790'])(Book) := phi(["ds_1 (-> ['TMP_790'])"])
order_1 (-> ['ds'])(Order) := phi(["order_1 (-> ['ds'])"])
amount_1(uint256) := phi(['REF_354'])
cancelTimestamp_1(uint32) := phi(['TMP_916'])
 order.side == Side.BUY
REF_388(Side) -> order_1 (-> ['ds']).side
REF_389(Side) -> Side.BUY
TMP_932(bool) = REF_388 == REF_389
CONDITION TMP_932
 oldAmountInQuote = ds.getQuoteTokenAmount(order.price,order.amount).toInt256()
REF_391(uint256) -> order_1 (-> ['ds']).price
REF_392(uint256) -> order_1 (-> ['ds']).amount
TMP_933(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getQuoteTokenAmount(Book,uint256,uint256), arguments:["ds_1 (-> ['TMP_790'])", 'REF_391', 'REF_392'] 
TMP_934(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['TMP_933'] 
oldAmountInQuote_1(int256) := TMP_934(int256)
 newAmountInQuote = ds.getQuoteTokenAmount(order.price,amount).toInt256()
REF_395(uint256) -> order_1 (-> ['ds']).price
TMP_935(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getQuoteTokenAmount(Book,uint256,uint256), arguments:["ds_1 (-> ['TMP_790'])", 'REF_395', 'amount_1'] 
TMP_936(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['TMP_935'] 
newAmountInQuote_1(int256) := TMP_936(int256)
 quoteTokenDelta = oldAmountInQuote - newAmountInQuote
TMP_937(int256) = oldAmountInQuote_1 (c)- newAmountInQuote_1
quoteTokenDelta_1(int256) := TMP_937(int256)
 ds.metadata().quoteTokenOpenInterest = uint256(ds.metadata().quoteTokenOpenInterest.toInt256() - quoteTokenDelta)
TMP_938(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:["ds_1 (-> ['TMP_790'])"] 
REF_398(uint256) -> TMP_938.quoteTokenOpenInterest
TMP_939(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:["ds_1 (-> ['TMP_790'])"] 
REF_400(uint256) -> TMP_939.quoteTokenOpenInterest
TMP_940(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_400'] 
TMP_941(int256) = TMP_940 (c)- quoteTokenDelta_1
TMP_942 = CONVERT TMP_941 to uint256
REF_398(uint256) (->TMP_938) := TMP_942(uint256)
 baseTokenDelta = order.amount.toInt256() - amount.toInt256()
REF_402(uint256) -> order_1 (-> ['ds']).amount
TMP_943(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_402'] 
TMP_944(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['amount_1'] 
TMP_945(int256) = TMP_943 (c)- TMP_944
baseTokenDelta_1(int256) := TMP_945(int256)
 ds.metadata().baseTokenOpenInterest = uint256(ds.metadata().baseTokenOpenInterest.toInt256() - baseTokenDelta)
TMP_946(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:["ds_1 (-> ['TMP_790'])"] 
REF_406(uint256) -> TMP_946.baseTokenOpenInterest
TMP_947(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:["ds_1 (-> ['TMP_790'])"] 
REF_408(uint256) -> TMP_947.baseTokenOpenInterest
TMP_948(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_408'] 
TMP_949(int256) = TMP_948 (c)- baseTokenDelta_1
TMP_950 = CONVERT TMP_949 to uint256
REF_406(uint256) (->TMP_946) := TMP_950(uint256)
quoteTokenDelta_2(int256) := phi(['quoteTokenDelta_1', 'quoteTokenDelta_0'])
baseTokenDelta_2(int256) := phi(['baseTokenDelta_0', 'baseTokenDelta_1'])
 order.amount = amount
REF_410(uint256) -> order_1 (-> ['ds']).amount
order_2 (-> ['ds'])(Order) := phi(["order_1 (-> ['ds'])"])
REF_410(uint256) (->order_2 (-> ['ds'])) := amount_1(uint256)
 order.cancelTimestamp = cancelTimestamp
REF_411(uint32) -> order_2 (-> ['ds']).cancelTimestamp
order_3 (-> ['ds'])(Order) := phi(["order_2 (-> ['ds'])"])
REF_411(uint32) (->order_3 (-> ['ds'])) := cancelTimestamp_1(uint32)
 (quoteTokenDelta,baseTokenDelta)
RETURN quoteTokenDelta_2,baseTokenDelta_2
```
#### CLOB._executeAmendNewOrder(Book,Order,ICLOB.AmendArgs) [INTERNAL]
```slithir
ds_1 (-> ['TMP_790'])(Book) := phi(["ds_1 (-> ['TMP_790'])"])
order_1 (-> ['ds'])(Order) := phi(["order_1 (-> ['ds'])"])
args_1(ICLOB.AmendArgs) := phi(['args_1'])
 newOrder.owner = order.owner
REF_361(address) -> newOrder_0.owner
REF_362(address) -> order_1 (-> ['ds']).owner
newOrder_1(Order) := phi(['newOrder_0'])
REF_361(address) (->newOrder_1) := REF_362(address)
 newOrder.id = order.id
REF_363(OrderId) -> newOrder_1.id
REF_364(OrderId) -> order_1 (-> ['ds']).id
newOrder_2(Order) := phi(['newOrder_1'])
REF_363(OrderId) (->newOrder_2) := REF_364(OrderId)
 newOrder.side = args.side
REF_365(Side) -> newOrder_2.side
REF_366(Side) -> args_1.side
newOrder_3(Order) := phi(['newOrder_2'])
REF_365(Side) (->newOrder_3) := REF_366(Side)
 newOrder.price = args.price
REF_367(uint256) -> newOrder_3.price
REF_368(uint256) -> args_1.price
newOrder_4(Order) := phi(['newOrder_3'])
REF_367(uint256) (->newOrder_4) := REF_368(uint256)
 newOrder.amount = args.amountInBase
REF_369(uint256) -> newOrder_4.amount
REF_370(uint256) -> args_1.amountInBase
newOrder_5(Order) := phi(['newOrder_4'])
REF_369(uint256) (->newOrder_5) := REF_370(uint256)
 newOrder.cancelTimestamp = uint32(args.cancelTimestamp)
REF_371(uint32) -> newOrder_5.cancelTimestamp
REF_372(uint32) -> args_1.cancelTimestamp
TMP_923 = CONVERT REF_372 to uint32
newOrder_6(Order) := phi(['newOrder_5'])
REF_371(uint32) (->newOrder_6) := TMP_923(uint32)
 order.side == Side.BUY
REF_373(Side) -> order_1 (-> ['ds']).side
REF_374(Side) -> Side.BUY
TMP_924(bool) = REF_373 == REF_374
CONDITION TMP_924
 quoteTokenDelta = ds.getQuoteTokenAmount(order.price,order.amount).toInt256()
REF_376(uint256) -> order_1 (-> ['ds']).price
REF_377(uint256) -> order_1 (-> ['ds']).amount
TMP_925(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getQuoteTokenAmount(Book,uint256,uint256), arguments:["ds_1 (-> ['TMP_790'])", 'REF_376', 'REF_377'] 
TMP_926(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['TMP_925'] 
quoteTokenDelta_1(int256) := TMP_926(int256)
 baseTokenDelta = order.amount.toInt256()
REF_379(uint256) -> order_1 (-> ['ds']).amount
TMP_927(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['REF_379'] 
baseTokenDelta_1(int256) := TMP_927(int256)
quoteTokenDelta_2(int256) := phi(['quoteTokenDelta_1', 'quoteTokenDelta_0'])
baseTokenDelta_2(int256) := phi(['baseTokenDelta_0', 'baseTokenDelta_1'])
 ds.removeOrderFromBook(order)
LIBRARY_CALL, dest:BookLib, function:BookLib.removeOrderFromBook(Book,Order), arguments:["ds_1 (-> ['TMP_790'])", "order_1 (-> ['ds'])"] 
 args.side == Side.BUY
REF_382(Side) -> args_1.side
REF_383(Side) -> Side.BUY
TMP_929(bool) = REF_382 == REF_383
CONDITION TMP_929
 (postAmount,None,None) = _executeBid(ds,newOrder,ICLOB.TiF.MOC,true)
REF_384(ICLOB.TiF) -> TiF.MOC
TUPLE_12(uint256,uint256,uint256) = INTERNAL_CALL, CLOB._executeBid(Book,Order,ICLOB.TiF,bool)(ds_1 (-> ['TMP_790']),newOrder_6,REF_384,True)
postAmount_1(uint256)= UNPACK TUPLE_12 index: 0 
 quoteTokenDelta -= postAmount.toInt256()
TMP_930(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['postAmount_1'] 
quoteTokenDelta_3(int256) = quoteTokenDelta_2 (c)- TMP_930
 (postAmount,None,None) = _executeAsk(ds,newOrder,ICLOB.TiF.MOC,true)
REF_386(ICLOB.TiF) -> TiF.MOC
TUPLE_13(uint256,uint256,uint256) = INTERNAL_CALL, CLOB._executeAsk(Book,Order,ICLOB.TiF,bool)(ds_1 (-> ['TMP_790']),newOrder_6,REF_386,True)
postAmount_2(uint256)= UNPACK TUPLE_13 index: 0 
 baseTokenDelta -= postAmount.toInt256()
TMP_931(int256) = LIBRARY_CALL, dest:SafeCastLib, function:SafeCastLib.toInt256(uint256), arguments:['postAmount_2'] 
baseTokenDelta_3(int256) = baseTokenDelta_2 (c)- TMP_931
quoteTokenDelta_4(int256) := phi(['quoteTokenDelta_0', 'quoteTokenDelta_3'])
baseTokenDelta_4(int256) := phi(['baseTokenDelta_3', 'baseTokenDelta_0'])
 (quoteTokenDelta,baseTokenDelta)
RETURN quoteTokenDelta_4,baseTokenDelta_4
```
#### CLOB._executeAsk(Book,Order,ICLOB.TiF,bool) [INTERNAL]
```slithir
ds_1 (-> ['TMP_780'])(Book) := phi(["ds_1 (-> ['TMP_780'])", "ds_1 (-> ['TMP_790'])"])
newOrder_1(Order) := phi(['newOrder_1', 'newOrder_6'])
tif_1(ICLOB.TiF) := phi(['REF_264', 'REF_386'])
baseDenominated_1(bool) := phi(['REF_265'])
factory_5(ICLOBManager) := phi(['factory_1', 'factory_7', 'factory_0', 'factory_4'])
maxNumOrdersPerSide_5(uint256) := phi(['maxNumOrdersPerSide_7', 'maxNumOrdersPerSide_1', 'maxNumOrdersPerSide_0', 'maxNumOrdersPerSide_4'])
 ds.getBestBidPrice() >= newOrder.price
TMP_875(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getBestBidPrice(Book), arguments:["ds_1 (-> ['TMP_780'])"] 
REF_311(uint256) -> newOrder_1.price
TMP_876(bool) = TMP_875 >= REF_311
CONDITION TMP_876
 tif == ICLOB.TiF.MOC
REF_312(ICLOB.TiF) -> TiF.MOC
TMP_877(bool) = tif_1 == REF_312
CONDITION TMP_877
 revert PostOnlyOrderWouldFill()()
TMP_878(None) = SOLIDITY_CALL revert PostOnlyOrderWouldFill()()
 (totalQuoteReceived,totalBaseSent) = _matchIncomingAsk(ds,newOrder,baseDenominated)
TUPLE_8(uint256,uint256) = INTERNAL_CALL, CLOB._matchIncomingAsk(Book,Order,bool)(ds_1 (-> ['TMP_780']),newOrder_1,baseDenominated_1)
totalQuoteReceived_1(uint256)= UNPACK TUPLE_8 index: 0 
totalBaseSent_1(uint256)= UNPACK TUPLE_8 index: 1 
totalQuoteReceived_2(uint256) := phi(['totalQuoteReceived_0', 'totalQuoteReceived_1'])
totalBaseSent_2(uint256) := phi(['totalBaseSent_0', 'totalBaseSent_1'])
 tif == ICLOB.TiF.FOK && newOrder.amount > 0
REF_313(ICLOB.TiF) -> TiF.FOK
TMP_879(bool) = tif_1 == REF_313
REF_314(uint256) -> newOrder_1.amount
TMP_880(bool) = REF_314 > 0
TMP_881(bool) = TMP_879 && TMP_880
CONDITION TMP_881
 revert FOKOrderNotFilled()()
TMP_882(None) = SOLIDITY_CALL revert FOKOrderNotFilled()()
 isTake = false
isTake_1(bool) := False(bool)
 (isTake,newOrder.amount) = _getTakeOrPostAmount(ds,tif,newOrder.amount,totalQuoteReceived | totalBaseSent > 0,baseDenominated,newOrder.price)
REF_315(uint256) -> newOrder_1.amount
REF_316(uint256) -> newOrder_1.amount
TMP_883(uint256) = totalQuoteReceived_2 | totalBaseSent_2
TMP_884(bool) = TMP_883 > 0
REF_317(uint256) -> newOrder_1.price
TUPLE_9(bool,uint256) = INTERNAL_CALL, CLOB._getTakeOrPostAmount(Book,ICLOB.TiF,uint256,bool,bool,uint256)(ds_1 (-> ['TMP_780']),tif_1,REF_316,TMP_884,baseDenominated_1,REF_317)
isTake_2(bool)= UNPACK TUPLE_9 index: 0 
REF_315(uint256)= UNPACK TUPLE_9 index: 1 
 isTake
CONDITION isTake_2
 (0,totalQuoteReceived,totalBaseSent)
RETURN 0,totalQuoteReceived_2,totalBaseSent_2
 ds.assertLimitPriceInBounds(newOrder.price)
REF_319(uint256) -> newOrder_1.price
LIBRARY_CALL, dest:BookLib, function:BookLib.assertLimitPriceInBounds(Book,uint256), arguments:["ds_1 (-> ['TMP_780'])", 'REF_319'] 
 ds.incrementLimitsPlaced(address(factory),msg.sender)
TMP_886 = CONVERT factory_7 to address
LIBRARY_CALL, dest:BookLib, function:BookLib.incrementLimitsPlaced(Book,address,address), arguments:["ds_1 (-> ['TMP_780'])", 'TMP_886', 'msg.sender'] 
 ds.metadata().numAsks == maxNumOrdersPerSide
TMP_888(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:["ds_1 (-> ['TMP_780'])"] 
REF_322(uint256) -> TMP_888.numAsks
TMP_889(bool) = REF_322 == maxNumOrdersPerSide_7
CONDITION TMP_889
 maxAskPrice = ds.getWorstAskPrice()
TMP_890(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getWorstAskPrice(Book), arguments:["ds_1 (-> ['TMP_780'])"] 
maxAskPrice_1(uint256) := TMP_890(uint256)
 newOrder.price >= maxAskPrice
REF_324(uint256) -> newOrder_1.price
TMP_891(bool) = REF_324 >= maxAskPrice_1
CONDITION TMP_891
 revert MaxOrdersInBookPostNotCompetitive()()
TMP_892(None) = SOLIDITY_CALL revert MaxOrdersInBookPostNotCompetitive()()
 _removeNonCompetitiveOrder(ds,ds.orders[ds.askLimits[maxAskPrice].tailOrder])
REF_325(mapping(OrderId => Order)) -> ds_1 (-> ['TMP_780']).orders
REF_326(mapping(uint256 => Limit)) -> ds_1 (-> ['TMP_780']).askLimits
REF_327(Limit) -> REF_326[maxAskPrice_1]
REF_328(OrderId) -> REF_327.tailOrder
REF_329(Order) -> REF_325[REF_328]
INTERNAL_CALL, CLOB._removeNonCompetitiveOrder(Book,Order)(ds_1 (-> ['TMP_780']),REF_329)
 ds.addOrderToBook(newOrder)
LIBRARY_CALL, dest:BookLib, function:BookLib.addOrderToBook(Book,Order), arguments:["ds_1 (-> ['TMP_780'])", 'newOrder_1'] 
 postAmount = newOrder.amount
REF_331(uint256) -> newOrder_1.amount
postAmount_1(uint256) := REF_331(uint256)
 (postAmount,totalQuoteReceived,totalBaseSent)
RETURN postAmount_1,totalQuoteReceived_2,totalBaseSent_2
 (postAmount,totalQuoteReceived,totalBaseSent)
```
#### CLOB._executeBid(Book,Order,ICLOB.TiF,bool) [INTERNAL]
```slithir
ds_1 (-> ['TMP_780'])(Book) := phi(["ds_1 (-> ['TMP_780'])", "ds_1 (-> ['TMP_790'])"])
newOrder_1(Order) := phi(['newOrder_6', 'newOrder_1'])
tif_1(ICLOB.TiF) := phi(['REF_384', 'REF_242'])
baseDenominated_1(bool) := phi(['REF_243'])
factory_2(ICLOBManager) := phi(['factory_1', 'factory_7', 'factory_0', 'factory_4'])
maxNumOrdersPerSide_2(uint256) := phi(['maxNumOrdersPerSide_7', 'maxNumOrdersPerSide_1', 'maxNumOrdersPerSide_0', 'maxNumOrdersPerSide_4'])
 ds.getBestAskPrice() <= newOrder.price
TMP_854(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getBestAskPrice(Book), arguments:["ds_1 (-> ['TMP_780'])"] 
REF_287(uint256) -> newOrder_1.price
TMP_855(bool) = TMP_854 <= REF_287
CONDITION TMP_855
 tif == ICLOB.TiF.MOC
REF_288(ICLOB.TiF) -> TiF.MOC
TMP_856(bool) = tif_1 == REF_288
CONDITION TMP_856
 revert PostOnlyOrderWouldFill()()
TMP_857(None) = SOLIDITY_CALL revert PostOnlyOrderWouldFill()()
 (totalQuoteSent,totalBaseReceived) = _matchIncomingBid(ds,newOrder,baseDenominated)
TUPLE_6(uint256,uint256) = INTERNAL_CALL, CLOB._matchIncomingBid(Book,Order,bool)(ds_1 (-> ['TMP_780']),newOrder_1,baseDenominated_1)
totalQuoteSent_1(uint256)= UNPACK TUPLE_6 index: 0 
totalBaseReceived_1(uint256)= UNPACK TUPLE_6 index: 1 
totalQuoteSent_2(uint256) := phi(['totalQuoteSent_1', 'totalQuoteSent_0'])
totalBaseReceived_2(uint256) := phi(['totalBaseReceived_1', 'totalBaseReceived_0'])
 tif == ICLOB.TiF.FOK && newOrder.amount > 0
REF_289(ICLOB.TiF) -> TiF.FOK
TMP_858(bool) = tif_1 == REF_289
REF_290(uint256) -> newOrder_1.amount
TMP_859(bool) = REF_290 > 0
TMP_860(bool) = TMP_858 && TMP_859
CONDITION TMP_860
 revert FOKOrderNotFilled()()
TMP_861(None) = SOLIDITY_CALL revert FOKOrderNotFilled()()
 isTake = false
isTake_1(bool) := False(bool)
 (isTake,newOrder.amount) = _getTakeOrPostAmount(ds,tif,newOrder.amount,totalQuoteSent | totalBaseReceived > 0,baseDenominated,newOrder.price)
REF_291(uint256) -> newOrder_1.amount
REF_292(uint256) -> newOrder_1.amount
TMP_862(uint256) = totalQuoteSent_2 | totalBaseReceived_2
TMP_863(bool) = TMP_862 > 0
REF_293(uint256) -> newOrder_1.price
TUPLE_7(bool,uint256) = INTERNAL_CALL, CLOB._getTakeOrPostAmount(Book,ICLOB.TiF,uint256,bool,bool,uint256)(ds_1 (-> ['TMP_780']),tif_1,REF_292,TMP_863,baseDenominated_1,REF_293)
isTake_2(bool)= UNPACK TUPLE_7 index: 0 
REF_291(uint256)= UNPACK TUPLE_7 index: 1 
 isTake
CONDITION isTake_2
 (0,totalQuoteSent,totalBaseReceived)
RETURN 0,totalQuoteSent_2,totalBaseReceived_2
 ds.assertLimitPriceInBounds(newOrder.price)
REF_295(uint256) -> newOrder_1.price
LIBRARY_CALL, dest:BookLib, function:BookLib.assertLimitPriceInBounds(Book,uint256), arguments:["ds_1 (-> ['TMP_780'])", 'REF_295'] 
 ds.incrementLimitsPlaced(address(factory),msg.sender)
TMP_865 = CONVERT factory_4 to address
LIBRARY_CALL, dest:BookLib, function:BookLib.incrementLimitsPlaced(Book,address,address), arguments:["ds_1 (-> ['TMP_780'])", 'TMP_865', 'msg.sender'] 
 ds.metadata().numBids == maxNumOrdersPerSide
TMP_867(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:["ds_1 (-> ['TMP_780'])"] 
REF_298(uint256) -> TMP_867.numBids
TMP_868(bool) = REF_298 == maxNumOrdersPerSide_4
CONDITION TMP_868
 minBidPrice = ds.getWorstBidPrice()
TMP_869(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getWorstBidPrice(Book), arguments:["ds_1 (-> ['TMP_780'])"] 
minBidPrice_1(uint256) := TMP_869(uint256)
 newOrder.price <= minBidPrice
REF_300(uint256) -> newOrder_1.price
TMP_870(bool) = REF_300 <= minBidPrice_1
CONDITION TMP_870
 revert MaxOrdersInBookPostNotCompetitive()()
TMP_871(None) = SOLIDITY_CALL revert MaxOrdersInBookPostNotCompetitive()()
 _removeNonCompetitiveOrder(ds,ds.orders[ds.bidLimits[minBidPrice].tailOrder])
REF_301(mapping(OrderId => Order)) -> ds_1 (-> ['TMP_780']).orders
REF_302(mapping(uint256 => Limit)) -> ds_1 (-> ['TMP_780']).bidLimits
REF_303(Limit) -> REF_302[minBidPrice_1]
REF_304(OrderId) -> REF_303.tailOrder
REF_305(Order) -> REF_301[REF_304]
INTERNAL_CALL, CLOB._removeNonCompetitiveOrder(Book,Order)(ds_1 (-> ['TMP_780']),REF_305)
 ds.addOrderToBook(newOrder)
LIBRARY_CALL, dest:BookLib, function:BookLib.addOrderToBook(Book,Order), arguments:["ds_1 (-> ['TMP_780'])", 'newOrder_1'] 
 postAmount = ds.getQuoteTokenAmount(newOrder.price,newOrder.amount)
REF_308(uint256) -> newOrder_1.price
REF_309(uint256) -> newOrder_1.amount
TMP_874(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getQuoteTokenAmount(Book,uint256,uint256), arguments:["ds_1 (-> ['TMP_780'])", 'REF_308', 'REF_309'] 
postAmount_1(uint256) := TMP_874(uint256)
 (postAmount,totalQuoteSent,totalBaseReceived)
RETURN postAmount_1,totalQuoteSent_2,totalBaseReceived_2
 (postAmount,totalQuoteSent,totalBaseReceived)
```
#### CLOB._executeCancel(Book,address,ICLOB.CancelArgs) [INTERNAL]
```slithir
ds_1 (-> ['TMP_802'])(Book) := phi(["ds_1 (-> ['TMP_802'])"])
account_1(address) := phi(['account_1'])
args_1(ICLOB.CancelArgs) := phi(['args_1'])
 numOrders = args.orderIds.length
REF_534(uint256[]) -> args_1.orderIds
REF_535 -> LENGTH REF_534
numOrders_1(uint256) := REF_535(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < numOrders
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_1021(bool) = i_2 < numOrders_1
CONDITION TMP_1021
 orderId = args.orderIds[i]
REF_536(uint256[]) -> args_1.orderIds
REF_537(uint256) -> REF_536[i_2]
orderId_1(uint256) := REF_537(uint256)
 order = ds.orders[orderId.toOrderId()]
REF_538(mapping(OrderId => Order)) -> ds_1 (-> ['TMP_802']).orders
TMP_1022(OrderId) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.toOrderId(uint256), arguments:['orderId_1'] 
REF_540(Order) -> REF_538[TMP_1022]
order_1 (-> ['ds'])(Order) := REF_540(Order)
 order.isNull()
TMP_1023(bool) = LIBRARY_CALL, dest:OrderLib, function:OrderLib.isNull(Order), arguments:["order_1 (-> ['ds'])"] 
CONDITION TMP_1023
 CancelFailed(EventNonceLib.inc(),orderId,account)
TMP_1024(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit CancelFailed(TMP_1024,orderId_1,account_1)
 order.owner != account
REF_543(address) -> order_1 (-> ['ds']).owner
TMP_1026(bool) = REF_543 != account_1
CONDITION TMP_1026
 revert CancelUnauthorized()()
TMP_1027(None) = SOLIDITY_CALL revert CancelUnauthorized()()
 quoteTokenRefunded = 0
quoteTokenRefunded_1(uint256) := 0(uint256)
 baseTokenRefunded = 0
baseTokenRefunded_1(uint256) := 0(uint256)
 order.side == Side.BUY
REF_544(Side) -> order_1 (-> ['ds']).side
REF_545(Side) -> Side.BUY
TMP_1028(bool) = REF_544 == REF_545
CONDITION TMP_1028
 quoteTokenRefunded = ds.getQuoteTokenAmount(order.price,order.amount)
REF_547(uint256) -> order_1 (-> ['ds']).price
REF_548(uint256) -> order_1 (-> ['ds']).amount
TMP_1029(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getQuoteTokenAmount(Book,uint256,uint256), arguments:["ds_1 (-> ['TMP_802'])", 'REF_547', 'REF_548'] 
quoteTokenRefunded_2(uint256) := TMP_1029(uint256)
 totalQuoteTokenRefunded += quoteTokenRefunded
totalQuoteTokenRefunded_1(uint256) = totalQuoteTokenRefunded_0 (c)+ quoteTokenRefunded_2
 baseTokenRefunded = order.amount
REF_549(uint256) -> order_1 (-> ['ds']).amount
baseTokenRefunded_2(uint256) := REF_549(uint256)
 totalBaseTokenRefunded += baseTokenRefunded
totalBaseTokenRefunded_1(uint256) = totalBaseTokenRefunded_0 (c)+ baseTokenRefunded_2
totalQuoteTokenRefunded_2(uint256) := phi(['totalQuoteTokenRefunded_0', 'totalQuoteTokenRefunded_1'])
totalBaseTokenRefunded_2(uint256) := phi(['totalBaseTokenRefunded_0', 'totalBaseTokenRefunded_1'])
quoteTokenRefunded_3(uint256) := phi(['quoteTokenRefunded_2', 'quoteTokenRefunded_1'])
baseTokenRefunded_3(uint256) := phi(['baseTokenRefunded_1', 'baseTokenRefunded_2'])
 ds.removeOrderFromBook(order)
LIBRARY_CALL, dest:BookLib, function:BookLib.removeOrderFromBook(Book,Order), arguments:["ds_1 (-> ['TMP_802'])", "order_1 (-> ['ds'])"] 
 eventNonce = EventNonceLib.inc()
TMP_1031(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
eventNonce_1(uint256) := TMP_1031(uint256)
 OrderCanceled(eventNonce,orderId,account,quoteTokenRefunded,baseTokenRefunded,CancelType.USER)
REF_552(ICLOB.CancelType) -> CancelType.USER
Emit OrderCanceled(eventNonce_1,orderId_1,account_1,quoteTokenRefunded_3,baseTokenRefunded_3,REF_552)
 i ++
TMP_1033(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 (totalQuoteTokenRefunded,totalBaseTokenRefunded)
RETURN totalQuoteTokenRefunded_0,totalBaseTokenRefunded_0
```
#### CLOB._getStorage() [INTERNAL]
```slithir
 CLOBStorageLib._getCLOBStorage()
TMP_1063(Book) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib._getCLOBStorage(), arguments:[] 
RETURN TMP_1063
```
#### CLOB._getTakeOrPostAmount(Book,ICLOB.TiF,uint256,bool,bool,uint256) [INTERNAL]
```slithir
ds_1 (-> ['TMP_780', 'TMP_780'])(Book) := phi(["ds_1 (-> ['TMP_780'])", "ds_1 (-> ['TMP_780'])"])
tif_1(ICLOB.TiF) := phi(['tif_1', 'tif_1'])
remainingAmount_1(uint256) := phi(['REF_292', 'REF_316'])
matchOccurred_1(bool) := phi(['TMP_884', 'TMP_863'])
baseDenominated_1(bool) := phi(['baseDenominated_1', 'baseDenominated_1'])
limitPrice_1(uint256) := phi(['REF_293', 'REF_317'])
 tif == ICLOB.TiF.FOK || tif == ICLOB.TiF.IOC
REF_332(ICLOB.TiF) -> TiF.FOK
TMP_895(bool) = tif_1 == REF_332
REF_333(ICLOB.TiF) -> TiF.IOC
TMP_896(bool) = tif_1 == REF_333
TMP_897(bool) = TMP_895 || TMP_896
CONDITION TMP_897
 (true,0)
RETURN True,0
 remainingAmount < ds.settings().minLimitOrderAmountInBase
TMP_898(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])"] 
REF_335(uint256) -> TMP_898.minLimitOrderAmountInBase
TMP_899(bool) = remainingAmount_4 < REF_335
CONDITION TMP_899
 tif == ICLOB.TiF.GTC && matchOccurred
REF_336(ICLOB.TiF) -> TiF.GTC
TMP_900(bool) = tif_1 == REF_336
TMP_901(bool) = TMP_900 && matchOccurred_1
CONDITION TMP_901
 (true,0)
RETURN True,0
 BookLib.LimitOrderAmountInvalid()
TMP_902(None) = SOLIDITY_CALL revert LimitOrderAmountInvalid()()
 (false,remainingAmount)
RETURN False,remainingAmount_4
 baseDenominated
CONDITION baseDenominated_1
 remainingAmount = ds.boundToLots(remainingAmount)
TMP_903(uint256) = LIBRARY_CALL, dest:BookLib, function:BookLib.boundToLots(Book,uint256), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])", 'remainingAmount_1'] 
remainingAmount_2(uint256) := TMP_903(uint256)
 remainingAmount = ds.boundToLots(ds.getBaseTokenAmount(remainingAmount,limitPrice))
TMP_904(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getBaseTokenAmount(Book,uint256,uint256), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])", 'remainingAmount_1', 'limitPrice_1'] 
TMP_905(uint256) = LIBRARY_CALL, dest:BookLib, function:BookLib.boundToLots(Book,uint256), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])", 'TMP_904'] 
remainingAmount_3(uint256) := TMP_905(uint256)
remainingAmount_4(uint256) := phi(['remainingAmount_2', 'remainingAmount_3'])
 (isTake,postAmount)
```
#### CLOB._matchIncomingAsk(Book,Order,bool) [INTERNAL]
```slithir
ds_1 (-> ['TMP_780'])(Book) := phi(["ds_1 (-> ['TMP_780'])"])
incomingOrder_1(Order) := phi(['newOrder_1'])
amountIsBase_1(bool) := phi(['baseDenominated_1'])
 bestBidPrice = ds.getBestBidPrice()
TMP_960(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getBestBidPrice(Book), arguments:["ds_1 (-> ['TMP_780'])"] 
bestBidPrice_1(uint256) := TMP_960(uint256)
 bestBidPrice >= incomingOrder.price && incomingOrder.amount > 0
bestBidPrice_2(uint256) := phi(['bestBidPrice_3', 'bestBidPrice_1'])
REF_428(uint256) -> incomingOrder_1.price
TMP_961(bool) = bestBidPrice_2 >= REF_428
REF_429(uint256) -> incomingOrder_1.amount
TMP_962(bool) = REF_429 > 0
TMP_963(bool) = TMP_961 && TMP_962
CONDITION TMP_963
 limit = ds.bidLimits[bestBidPrice]
REF_430(mapping(uint256 => Limit)) -> ds_1 (-> ['TMP_780']).bidLimits
REF_431(Limit) -> REF_430[bestBidPrice_2]
limit_1 (-> ['ds'])(Limit) := REF_431(Limit)
 bestBidOrder = ds.orders[limit.headOrder]
REF_432(mapping(OrderId => Order)) -> ds_1 (-> ['TMP_780']).orders
REF_433(OrderId) -> limit_1 (-> ['ds']).headOrder
REF_434(Order) -> REF_432[REF_433]
bestBidOrder_1 (-> ['ds'])(Order) := REF_434(Order)
 bestBidOrder.isExpired()
TMP_964(bool) = LIBRARY_CALL, dest:OrderLib, function:OrderLib.isExpired(Order), arguments:["bestBidOrder_1 (-> ['ds'])"] 
CONDITION TMP_964
 _removeExpiredBid(ds,bestBidOrder)
INTERNAL_CALL, CLOB._removeExpiredBid(Book,Order)(ds_1 (-> ['TMP_780']),bestBidOrder_1 (-> ['ds']))
 bestBidPrice = ds.getBestBidPrice()
TMP_966(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getBestBidPrice(Book), arguments:["ds_1 (-> ['TMP_780'])"] 
bestBidPrice_4(uint256) := TMP_966(uint256)
 currMatch = _matchIncomingOrder(ds,bestBidOrder,incomingOrder,bestBidPrice,amountIsBase)
TMP_967(CLOB.__MatchData__) = INTERNAL_CALL, CLOB._matchIncomingOrder(Book,Order,Order,uint256,bool)(ds_1 (-> ['TMP_780']),bestBidOrder_1 (-> ['ds']),incomingOrder_1,bestBidPrice_2,amountIsBase_1)
currMatch_1(CLOB.__MatchData__) := TMP_967(CLOB.__MatchData__)
 incomingOrder.amount -= currMatch.matchedAmount
REF_437(uint256) -> incomingOrder_1.amount
REF_438(uint256) -> currMatch_1.matchedAmount
incomingOrder_3(Order) := phi(['incomingOrder_1'])
REF_437(-> incomingOrder_3) = REF_437 (c)- REF_438
 totalQuoteReceived += currMatch.quoteDelta
REF_439(uint256) -> currMatch_1.quoteDelta
totalQuoteReceived_2(uint256) = totalQuoteReceived_0 (c)+ REF_439
 totalBaseSent += currMatch.baseDelta
REF_440(uint256) -> currMatch_1.baseDelta
totalBaseSent_2(uint256) = totalBaseSent_0 (c)+ REF_440
 bestBidPrice = ds.getBestBidPrice()
incomingOrder_2(Order) := phi(['incomingOrder_1', 'incomingOrder_3'])
totalQuoteReceived_1(uint256) := phi(['totalQuoteReceived_2', 'totalQuoteReceived_0'])
totalBaseSent_1(uint256) := phi(['totalBaseSent_0', 'totalBaseSent_2'])
TMP_968(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getBestBidPrice(Book), arguments:["ds_1 (-> ['TMP_780'])"] 
bestBidPrice_3(uint256) := TMP_968(uint256)
 (totalQuoteReceived,totalBaseSent)
RETURN totalQuoteReceived_0,totalBaseSent_0
```
#### CLOB._matchIncomingBid(Book,Order,bool) [INTERNAL]
```slithir
ds_1 (-> ['TMP_780'])(Book) := phi(["ds_1 (-> ['TMP_780'])"])
incomingOrder_1(Order) := phi(['newOrder_1'])
amountIsBase_1(bool) := phi(['baseDenominated_1'])
 bestAskPrice = ds.getBestAskPrice()
TMP_951(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getBestAskPrice(Book), arguments:["ds_1 (-> ['TMP_780'])"] 
bestAskPrice_1(uint256) := TMP_951(uint256)
 bestAskPrice <= incomingOrder.price && incomingOrder.amount > 0
bestAskPrice_2(uint256) := phi(['bestAskPrice_3', 'bestAskPrice_1'])
REF_413(uint256) -> incomingOrder_1.price
TMP_952(bool) = bestAskPrice_2 <= REF_413
REF_414(uint256) -> incomingOrder_1.amount
TMP_953(bool) = REF_414 > 0
TMP_954(bool) = TMP_952 && TMP_953
CONDITION TMP_954
 limit = ds.askLimits[bestAskPrice]
REF_415(mapping(uint256 => Limit)) -> ds_1 (-> ['TMP_780']).askLimits
REF_416(Limit) -> REF_415[bestAskPrice_2]
limit_1 (-> ['ds'])(Limit) := REF_416(Limit)
 bestAskOrder = ds.orders[limit.headOrder]
REF_417(mapping(OrderId => Order)) -> ds_1 (-> ['TMP_780']).orders
REF_418(OrderId) -> limit_1 (-> ['ds']).headOrder
REF_419(Order) -> REF_417[REF_418]
bestAskOrder_1 (-> ['ds'])(Order) := REF_419(Order)
 bestAskOrder.isExpired()
TMP_955(bool) = LIBRARY_CALL, dest:OrderLib, function:OrderLib.isExpired(Order), arguments:["bestAskOrder_1 (-> ['ds'])"] 
CONDITION TMP_955
 _removeExpiredAsk(ds,bestAskOrder)
INTERNAL_CALL, CLOB._removeExpiredAsk(Book,Order)(ds_1 (-> ['TMP_780']),bestAskOrder_1 (-> ['ds']))
 bestAskPrice = ds.getBestAskPrice()
TMP_957(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getBestAskPrice(Book), arguments:["ds_1 (-> ['TMP_780'])"] 
bestAskPrice_4(uint256) := TMP_957(uint256)
 currMatch = _matchIncomingOrder(ds,bestAskOrder,incomingOrder,bestAskPrice,amountIsBase)
TMP_958(CLOB.__MatchData__) = INTERNAL_CALL, CLOB._matchIncomingOrder(Book,Order,Order,uint256,bool)(ds_1 (-> ['TMP_780']),bestAskOrder_1 (-> ['ds']),incomingOrder_1,bestAskPrice_2,amountIsBase_1)
currMatch_1(CLOB.__MatchData__) := TMP_958(CLOB.__MatchData__)
 incomingOrder.amount -= currMatch.matchedAmount
REF_422(uint256) -> incomingOrder_1.amount
REF_423(uint256) -> currMatch_1.matchedAmount
incomingOrder_3(Order) := phi(['incomingOrder_1'])
REF_422(-> incomingOrder_3) = REF_422 (c)- REF_423
 totalQuoteSent += currMatch.quoteDelta
REF_424(uint256) -> currMatch_1.quoteDelta
totalQuoteSent_2(uint256) = totalQuoteSent_0 (c)+ REF_424
 totalBaseReceived += currMatch.baseDelta
REF_425(uint256) -> currMatch_1.baseDelta
totalBaseReceived_2(uint256) = totalBaseReceived_0 (c)+ REF_425
 bestAskPrice = ds.getBestAskPrice()
incomingOrder_2(Order) := phi(['incomingOrder_3', 'incomingOrder_1'])
totalQuoteSent_1(uint256) := phi(['totalQuoteSent_2', 'totalQuoteSent_0'])
totalBaseReceived_1(uint256) := phi(['totalBaseReceived_2', 'totalBaseReceived_0'])
TMP_959(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getBestAskPrice(Book), arguments:["ds_1 (-> ['TMP_780'])"] 
bestAskPrice_3(uint256) := TMP_959(uint256)
 (totalQuoteSent,totalBaseReceived)
RETURN totalQuoteSent_0,totalBaseReceived_0
```
#### CLOB._matchIncomingOrder(Book,Order,Order,uint256,bool) [INTERNAL]
```slithir
ds_1 (-> ['TMP_780', 'TMP_780'])(Book) := phi(["ds_1 (-> ['TMP_780'])", "ds_1 (-> ['TMP_780'])"])
makerOrder_1 (-> ['ds', 'ds'])(Order) := phi(["bestBidOrder_1 (-> ['ds'])", "bestAskOrder_1 (-> ['ds'])"])
takerOrder_1(Order) := phi(['incomingOrder_1', 'incomingOrder_1'])
matchedPrice_1(uint256) := phi(['bestAskPrice_2', 'bestBidPrice_2'])
amountIsBase_1(bool) := phi(['amountIsBase_1', 'amountIsBase_1'])
 lotSize = ds.settings().lotSizeInBase
TMP_981(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])"] 
REF_460(uint256) -> TMP_981.lotSizeInBase
lotSize_1(uint256) := REF_460(uint256)
 _boundMakerToLotSize(ds,makerOrder,lotSize)
INTERNAL_CALL, CLOB._boundMakerToLotSize(Book,Order,uint256)(ds_1 (-> ['TMP_780', 'TMP_780']),makerOrder_1 (-> ['ds', 'ds']),lotSize_1)
 matchedBase = makerOrder.amount
REF_461(uint256) -> makerOrder_1 (-> ['ds', 'ds']).amount
matchedBase_1(uint256) := REF_461(uint256)
 amountIsBase
CONDITION amountIsBase_1
 matchData.baseDelta = (matchedBase.min(takerOrder.amount) / lotSize) * lotSize
REF_462(uint256) -> matchData_0.baseDelta
REF_464(uint256) -> takerOrder_1.amount
TMP_983(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.min(uint256,uint256), arguments:['matchedBase_1', 'REF_464'] 
TMP_984(uint256) = TMP_983 (c)/ lotSize_1
TMP_985(uint256) = TMP_984 (c)* lotSize_1
matchData_1(CLOB.__MatchData__) := phi(['matchData_0'])
REF_462(uint256) (->matchData_1) := TMP_985(uint256)
 matchData.quoteDelta = ds.getQuoteTokenAmount(matchedPrice,matchData.baseDelta)
REF_465(uint256) -> matchData_1.quoteDelta
REF_467(uint256) -> matchData_1.baseDelta
TMP_986(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getQuoteTokenAmount(Book,uint256,uint256), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])", 'matchedPrice_1', 'REF_467'] 
matchData_2(CLOB.__MatchData__) := phi(['matchData_1'])
REF_465(uint256) (->matchData_2) := TMP_986(uint256)
 matchData.baseDelta = (matchedBase.min(ds.getBaseTokenAmount(matchedPrice,takerOrder.amount)) / lotSize) * lotSize
REF_468(uint256) -> matchData_0.baseDelta
REF_471(uint256) -> takerOrder_1.amount
TMP_987(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getBaseTokenAmount(Book,uint256,uint256), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])", 'matchedPrice_1', 'REF_471'] 
TMP_988(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.min(uint256,uint256), arguments:['matchedBase_1', 'TMP_987'] 
TMP_989(uint256) = TMP_988 (c)/ lotSize_1
TMP_990(uint256) = TMP_989 (c)* lotSize_1
matchData_6(CLOB.__MatchData__) := phi(['matchData_0'])
REF_468(uint256) (->matchData_6) := TMP_990(uint256)
 matchData.quoteDelta = ds.getQuoteTokenAmount(matchedPrice,matchData.baseDelta)
REF_472(uint256) -> matchData_6.quoteDelta
REF_474(uint256) -> matchData_6.baseDelta
TMP_991(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getQuoteTokenAmount(Book,uint256,uint256), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])", 'matchedPrice_1', 'REF_474'] 
matchData_7(CLOB.__MatchData__) := phi(['matchData_6'])
REF_472(uint256) (->matchData_7) := TMP_991(uint256)
matchData_11(CLOB.__MatchData__) := phi(['matchData_7', 'matchData_2'])
 matchData.baseDelta == 0
REF_475(uint256) -> matchData_11.baseDelta
TMP_992(bool) = REF_475 == 0
CONDITION TMP_992
 matchData
RETURN matchData_11
 orderRemoved = matchData.baseDelta == matchedBase
REF_476(uint256) -> matchData_11.baseDelta
TMP_993(bool) = REF_476 == matchedBase_1
orderRemoved_1(bool) := TMP_993(bool)
 takerOrder.side == Side.BUY
REF_477(Side) -> takerOrder_1.side
REF_478(Side) -> Side.BUY
TMP_994(bool) = REF_477 == REF_478
CONDITION TMP_994
 TransientMakerData.addQuoteToken(makerOrder.owner,matchData.quoteDelta)
REF_480(address) -> makerOrder_1 (-> ['ds', 'ds']).owner
REF_481(uint256) -> matchData_11.quoteDelta
LIBRARY_CALL, dest:TransientMakerData, function:TransientMakerData.addQuoteToken(address,uint256), arguments:['REF_480', 'REF_481'] 
 ! orderRemoved
TMP_996 = UnaryType.BANG orderRemoved_1 
CONDITION TMP_996
 ds.metadata().baseTokenOpenInterest -= matchData.baseDelta
TMP_997(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])"] 
REF_483(uint256) -> TMP_997.baseTokenOpenInterest
REF_484(uint256) -> matchData_11.baseDelta
REF_483(-> TMP_997) = REF_483 (c)- REF_484
 TransientMakerData.addBaseToken(makerOrder.owner,matchData.baseDelta)
REF_486(address) -> makerOrder_1 (-> ['ds', 'ds']).owner
REF_487(uint256) -> matchData_11.baseDelta
LIBRARY_CALL, dest:TransientMakerData, function:TransientMakerData.addBaseToken(address,uint256), arguments:['REF_486', 'REF_487'] 
 ! orderRemoved
TMP_999 = UnaryType.BANG orderRemoved_1 
CONDITION TMP_999
 ds.metadata().quoteTokenOpenInterest -= matchData.quoteDelta
TMP_1000(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])"] 
REF_489(uint256) -> TMP_1000.quoteTokenOpenInterest
REF_490(uint256) -> matchData_11.quoteDelta
REF_489(-> TMP_1000) = REF_489 (c)- REF_490
 orderRemoved
CONDITION orderRemoved_1
 ds.removeOrderFromBook(makerOrder)
LIBRARY_CALL, dest:BookLib, function:BookLib.removeOrderFromBook(Book,Order), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])", "makerOrder_1 (-> ['ds', 'ds'])"] 
 makerOrder.amount -= matchData.baseDelta
REF_492(uint256) -> makerOrder_1 (-> ['ds', 'ds']).amount
REF_493(uint256) -> matchData_11.baseDelta
makerOrder_2 (-> ['ds', 'ds'])(Order) := phi(["makerOrder_1 (-> ['ds', 'ds'])"])
REF_492(-> makerOrder_2 (-> ['ds', 'ds'])) = REF_492 (c)- REF_493
 matchData.baseDelta != matchedBase
REF_494(uint256) -> matchData_2.baseDelta
TMP_1002(bool) = REF_494 != matchedBase_1
CONDITION TMP_1002
 matchData.matchedAmount = takerOrder.amount
REF_495(uint256) -> matchData_2.matchedAmount
REF_496(uint256) -> takerOrder_1.amount
matchData_3(CLOB.__MatchData__) := phi(['matchData_2'])
REF_495(uint256) (->matchData_3) := REF_496(uint256)
 matchData.matchedAmount = matchData.baseDelta
REF_497(uint256) -> matchData_2.matchedAmount
REF_498(uint256) -> matchData_2.baseDelta
matchData_4(CLOB.__MatchData__) := phi(['matchData_2'])
REF_497(uint256) (->matchData_4) := REF_498(uint256)
matchData_5(CLOB.__MatchData__) := phi(['matchData_3', 'matchData_4'])
 matchData.baseDelta != matchedBase
REF_499(uint256) -> matchData_7.baseDelta
TMP_1003(bool) = REF_499 != matchedBase_1
CONDITION TMP_1003
 matchData.matchedAmount = takerOrder.amount
REF_500(uint256) -> matchData_7.matchedAmount
REF_501(uint256) -> takerOrder_1.amount
matchData_8(CLOB.__MatchData__) := phi(['matchData_7'])
REF_500(uint256) (->matchData_8) := REF_501(uint256)
 matchData.matchedAmount = matchData.quoteDelta
REF_502(uint256) -> matchData_7.matchedAmount
REF_503(uint256) -> matchData_7.quoteDelta
matchData_9(CLOB.__MatchData__) := phi(['matchData_7'])
REF_502(uint256) (->matchData_9) := REF_503(uint256)
matchData_10(CLOB.__MatchData__) := phi(['matchData_8', 'matchData_9'])
 matchData
RETURN matchData_11
```
#### CLOB._processAmend(Book,Order,ICLOB.AmendArgs) [INTERNAL]
```slithir
ds_1 (-> ['TMP_790'])(Book) := phi(["ds_1 (-> ['TMP_790'])"])
order_1 (-> ['ds'])(Order) := phi(["order_1 (-> ['ds'])"])
args_1(ICLOB.AmendArgs) := phi(['args_1'])
 preAmend = order
preAmend_1(Order) := order_1 (-> ['ds'])(Order)
 maker = preAmend.owner
REF_340(address) -> preAmend_1.owner
maker_1(address) := REF_340(address)
 args.cancelTimestamp.isExpired() || args.amountInBase < ds.settings().minLimitOrderAmountInBase
REF_341(uint32) -> args_1.cancelTimestamp
TMP_906(bool) = LIBRARY_CALL, dest:OrderLib, function:OrderLib.isExpired(uint256), arguments:['REF_341'] 
REF_343(uint256) -> args_1.amountInBase
TMP_907(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:["ds_1 (-> ['TMP_790'])"] 
REF_345(uint256) -> TMP_907.minLimitOrderAmountInBase
TMP_908(bool) = REF_343 < REF_345
TMP_909(bool) = TMP_906 || TMP_908
CONDITION TMP_909
 revert AmendInvalid()()
TMP_910(None) = SOLIDITY_CALL revert AmendInvalid()()
 ds.assertLotSizeCompliant(args.amountInBase)
REF_347(uint256) -> args_1.amountInBase
LIBRARY_CALL, dest:BookLib, function:BookLib.assertLotSizeCompliant(Book,uint256), arguments:["ds_1 (-> ['TMP_790'])", 'REF_347'] 
 order.side != args.side || order.price != args.price
REF_348(Side) -> order_1 (-> ['ds']).side
REF_349(Side) -> args_1.side
TMP_912(bool) = REF_348 != REF_349
REF_350(uint256) -> order_1 (-> ['ds']).price
REF_351(uint256) -> args_1.price
TMP_913(bool) = REF_350 != REF_351
TMP_914(bool) = TMP_912 || TMP_913
CONDITION TMP_914
 (quoteTokenDelta,baseTokenDelta) = _executeAmendNewOrder(ds,order,args)
TUPLE_10(int256,int256) = INTERNAL_CALL, CLOB._executeAmendNewOrder(Book,Order,ICLOB.AmendArgs)(ds_1 (-> ['TMP_790']),order_1 (-> ['ds']),args_1)
quoteTokenDelta_3(int256)= UNPACK TUPLE_10 index: 0 
baseTokenDelta_3(int256)= UNPACK TUPLE_10 index: 1 
 order.amount != args.amountInBase
REF_352(uint256) -> order_1 (-> ['ds']).amount
REF_353(uint256) -> args_1.amountInBase
TMP_915(bool) = REF_352 != REF_353
CONDITION TMP_915
 (quoteTokenDelta,baseTokenDelta) = _executeAmendAmount(ds,order,args.amountInBase,uint32(args.cancelTimestamp))
REF_354(uint256) -> args_1.amountInBase
REF_355(uint32) -> args_1.cancelTimestamp
TMP_916 = CONVERT REF_355 to uint32
TUPLE_11(int256,int256) = INTERNAL_CALL, CLOB._executeAmendAmount(Book,Order,uint256,uint32)(ds_1 (-> ['TMP_790']),order_1 (-> ['ds']),REF_354,TMP_916)
quoteTokenDelta_1(int256)= UNPACK TUPLE_11 index: 0 
baseTokenDelta_1(int256)= UNPACK TUPLE_11 index: 1 
 args.cancelTimestamp != order.cancelTimestamp
REF_356(uint32) -> args_1.cancelTimestamp
REF_357(uint32) -> order_1 (-> ['ds']).cancelTimestamp
TMP_917(bool) = REF_356 != REF_357
CONDITION TMP_917
 order.cancelTimestamp = uint32(args.cancelTimestamp)
REF_358(uint32) -> order_1 (-> ['ds']).cancelTimestamp
REF_359(uint32) -> args_1.cancelTimestamp
TMP_918 = CONVERT REF_359 to uint32
order_2 (-> ['ds'])(Order) := phi(["order_1 (-> ['ds'])"])
REF_358(uint32) (->order_2 (-> ['ds'])) := TMP_918(uint32)
 revert ZeroAmend()()
TMP_919(None) = SOLIDITY_CALL revert ZeroAmend()()
quoteTokenDelta_2(int256) := phi(['quoteTokenDelta_0', 'quoteTokenDelta_1'])
baseTokenDelta_2(int256) := phi(['baseTokenDelta_0', 'baseTokenDelta_1'])
quoteTokenDelta_4(int256) := phi(['quoteTokenDelta_0', 'quoteTokenDelta_3'])
baseTokenDelta_4(int256) := phi(['baseTokenDelta_0', 'baseTokenDelta_3'])
 OrderAmended(EventNonceLib.inc(),preAmend,args,quoteTokenDelta,baseTokenDelta)
TMP_920(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit OrderAmended(TMP_920,preAmend_1,args_1,quoteTokenDelta_4,baseTokenDelta_4)
 _settleAmend(ds,maker,quoteTokenDelta,baseTokenDelta)
INTERNAL_CALL, CLOB._settleAmend(Book,address,int256,int256)(ds_1 (-> ['TMP_790']),maker_1,quoteTokenDelta_4,baseTokenDelta_4)
 (quoteTokenDelta,baseTokenDelta)
RETURN quoteTokenDelta_4,baseTokenDelta_4
```
#### CLOB._processAsk(Book,address,Order,ICLOB.PlaceOrderArgs) [INTERNAL]
```slithir
ds_1 (-> ['TMP_780'])(Book) := phi(["ds_1 (-> ['TMP_780'])"])
account_1(address) := phi(['account_1'])
newOrder_1(Order) := phi(['newOrder_1'])
args_1(ICLOB.PlaceOrderArgs) := phi(['args_1'])
 (postAmount,totalQuoteReceived,totalBaseSent) = _executeAsk(ds,newOrder,args.tif,args.baseDenominated)
REF_264(ICLOB.TiF) -> args_1.tif
REF_265(bool) -> args_1.baseDenominated
TUPLE_5(uint256,uint256,uint256) = INTERNAL_CALL, CLOB._executeAsk(Book,Order,ICLOB.TiF,bool)(ds_1 (-> ['TMP_780']),newOrder_1,REF_264,REF_265)
postAmount_1(uint256)= UNPACK TUPLE_5 index: 0 
totalQuoteReceived_1(uint256)= UNPACK TUPLE_5 index: 1 
totalBaseSent_1(uint256)= UNPACK TUPLE_5 index: 2 
 postAmount + totalQuoteReceived + totalBaseSent == 0
TMP_833(uint256) = postAmount_1 (c)+ totalQuoteReceived_1
TMP_834(uint256) = TMP_833 (c)+ totalBaseSent_1
TMP_835(bool) = TMP_834 == 0
CONDITION TMP_835
 revert ZeroOrder()()
TMP_836(None) = SOLIDITY_CALL revert ZeroOrder()()
 totalBaseSent != totalQuoteReceived && (totalBaseSent == 0 || totalQuoteReceived == 0)
TMP_837(bool) = totalBaseSent_1 != totalQuoteReceived_1
TMP_838(bool) = totalBaseSent_1 == 0
TMP_839(bool) = totalQuoteReceived_1 == 0
TMP_840(bool) = TMP_838 || TMP_839
TMP_841(bool) = TMP_837 && TMP_840
CONDITION TMP_841
 revert ZeroCostTrade()()
TMP_842(None) = SOLIDITY_CALL revert ZeroCostTrade()()
 takerFee = _settleIncomingOrder(ds,account,Side.SELL,totalQuoteReceived,totalBaseSent + postAmount)
REF_266(Side) -> Side.SELL
TMP_843(uint256) = totalBaseSent_1 (c)+ postAmount_1
TMP_844(uint256) = INTERNAL_CALL, CLOB._settleIncomingOrder(Book,address,Side,uint256,uint256)(ds_1 (-> ['TMP_780']),account_1,REF_266,totalQuoteReceived_1,TMP_843)
takerFee_1(uint256) := TMP_844(uint256)
 res.account = account
REF_267(address) -> res_0.account
res_1(ICLOB.PlaceOrderResult) := phi(['res_0'])
REF_267(address) (->res_1) := account_1(address)
 res.orderId = newOrder.id.unwrap()
REF_268(uint256) -> res_1.orderId
REF_269(OrderId) -> newOrder_1.id
TMP_845(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_269'] 
res_2(ICLOB.PlaceOrderResult) := phi(['res_1'])
REF_268(uint256) (->res_2) := TMP_845(uint256)
 res.quoteTokenAmountTraded = int256(totalQuoteReceived)
REF_271(int256) -> res_2.quoteTokenAmountTraded
TMP_846 = CONVERT totalQuoteReceived_1 to int256
res_3(ICLOB.PlaceOrderResult) := phi(['res_2'])
REF_271(int256) (->res_3) := TMP_846(int256)
 res.baseTokenAmountTraded = - int256(totalBaseSent)
REF_272(int256) -> res_3.baseTokenAmountTraded
TMP_847 = CONVERT totalBaseSent_1 to int256
TMP_848(int256) = 0 (c)- TMP_847
res_4(ICLOB.PlaceOrderResult) := phi(['res_3'])
REF_272(int256) (->res_4) := TMP_848(int256)
 res.takerFee = takerFee
REF_273(uint256) -> res_4.takerFee
res_5(ICLOB.PlaceOrderResult) := phi(['res_4'])
REF_273(uint256) (->res_5) := takerFee_1(uint256)
 uint8(args.tif) <= 1
REF_274(ICLOB.TiF) -> args_1.tif
TMP_849 = CONVERT REF_274 to uint8
TMP_850(bool) = TMP_849 <= 1
CONDITION TMP_850
 res.basePosted = newOrder.amount
REF_275(uint256) -> res_5.basePosted
REF_276(uint256) -> newOrder_1.amount
res_6(ICLOB.PlaceOrderResult) := phi(['res_5'])
REF_275(uint256) (->res_6) := REF_276(uint256)
res_7(ICLOB.PlaceOrderResult) := phi(['res_5', 'res_6'])
 res.wasMarketOrder = (args.limitPrice == 0)
REF_277(bool) -> res_7.wasMarketOrder
REF_278(uint256) -> args_1.limitPrice
TMP_851(bool) = REF_278 == 0
res_8(ICLOB.PlaceOrderResult) := phi(['res_7'])
REF_277(bool) (->res_8) := TMP_851(bool)
 OrderProcessed({eventNonce:EventNonceLib.inc(),account:account,orderId:res.orderId,tif:args.tif,limitPrice:args.limitPrice,basePosted:res.basePosted,quoteDelta:res.quoteTokenAmountTraded,baseDelta:res.baseTokenAmountTraded,takerFee:takerFee})
TMP_852(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
REF_280(uint256) -> res_8.orderId
REF_281(ICLOB.TiF) -> args_1.tif
REF_282(uint256) -> args_1.limitPrice
REF_283(uint256) -> res_8.basePosted
REF_284(int256) -> res_8.quoteTokenAmountTraded
REF_285(int256) -> res_8.baseTokenAmountTraded
Emit OrderProcessed(TMP_852,account_1,REF_280,REF_281,REF_282,REF_283,REF_284,REF_285,takerFee_1)
 res
RETURN res_8
```
#### CLOB._processBid(Book,address,Order,ICLOB.PlaceOrderArgs) [INTERNAL]
```slithir
ds_1 (-> ['TMP_780'])(Book) := phi(["ds_1 (-> ['TMP_780'])"])
account_1(address) := phi(['account_1'])
newOrder_1(Order) := phi(['newOrder_1'])
args_1(ICLOB.PlaceOrderArgs) := phi(['args_1'])
 (postAmount,totalQuoteSent,totalBaseReceived) = _executeBid(ds,newOrder,args.tif,args.baseDenominated)
REF_242(ICLOB.TiF) -> args_1.tif
REF_243(bool) -> args_1.baseDenominated
TUPLE_4(uint256,uint256,uint256) = INTERNAL_CALL, CLOB._executeBid(Book,Order,ICLOB.TiF,bool)(ds_1 (-> ['TMP_780']),newOrder_1,REF_242,REF_243)
postAmount_1(uint256)= UNPACK TUPLE_4 index: 0 
totalQuoteSent_1(uint256)= UNPACK TUPLE_4 index: 1 
totalBaseReceived_1(uint256)= UNPACK TUPLE_4 index: 2 
 postAmount + totalQuoteSent + totalBaseReceived == 0
TMP_810(uint256) = postAmount_1 (c)+ totalQuoteSent_1
TMP_811(uint256) = TMP_810 (c)+ totalBaseReceived_1
TMP_812(bool) = TMP_811 == 0
CONDITION TMP_812
 revert ZeroOrder()()
TMP_813(None) = SOLIDITY_CALL revert ZeroOrder()()
 totalBaseReceived != totalQuoteSent && (totalBaseReceived == 0 || totalQuoteSent == 0)
TMP_814(bool) = totalBaseReceived_1 != totalQuoteSent_1
TMP_815(bool) = totalBaseReceived_1 == 0
TMP_816(bool) = totalQuoteSent_1 == 0
TMP_817(bool) = TMP_815 || TMP_816
TMP_818(bool) = TMP_814 && TMP_817
CONDITION TMP_818
 revert ZeroCostTrade()()
TMP_819(None) = SOLIDITY_CALL revert ZeroCostTrade()()
 takerFee = _settleIncomingOrder(ds,account,Side.BUY,totalQuoteSent + postAmount,totalBaseReceived)
REF_244(Side) -> Side.BUY
TMP_820(uint256) = totalQuoteSent_1 (c)+ postAmount_1
TMP_821(uint256) = INTERNAL_CALL, CLOB._settleIncomingOrder(Book,address,Side,uint256,uint256)(ds_1 (-> ['TMP_780']),account_1,REF_244,TMP_820,totalBaseReceived_1)
takerFee_1(uint256) := TMP_821(uint256)
 res.account = account
REF_245(address) -> res_0.account
res_1(ICLOB.PlaceOrderResult) := phi(['res_0'])
REF_245(address) (->res_1) := account_1(address)
 res.orderId = newOrder.id.unwrap()
REF_246(uint256) -> res_1.orderId
REF_247(OrderId) -> newOrder_1.id
TMP_822(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_247'] 
res_2(ICLOB.PlaceOrderResult) := phi(['res_1'])
REF_246(uint256) (->res_2) := TMP_822(uint256)
 res.quoteTokenAmountTraded = - int256(totalQuoteSent)
REF_249(int256) -> res_2.quoteTokenAmountTraded
TMP_823 = CONVERT totalQuoteSent_1 to int256
TMP_824(int256) = 0 (c)- TMP_823
res_3(ICLOB.PlaceOrderResult) := phi(['res_2'])
REF_249(int256) (->res_3) := TMP_824(int256)
 res.baseTokenAmountTraded = int256(totalBaseReceived)
REF_250(int256) -> res_3.baseTokenAmountTraded
TMP_825 = CONVERT totalBaseReceived_1 to int256
res_4(ICLOB.PlaceOrderResult) := phi(['res_3'])
REF_250(int256) (->res_4) := TMP_825(int256)
 res.takerFee = takerFee
REF_251(uint256) -> res_4.takerFee
res_5(ICLOB.PlaceOrderResult) := phi(['res_4'])
REF_251(uint256) (->res_5) := takerFee_1(uint256)
 uint8(args.tif) <= 1
REF_252(ICLOB.TiF) -> args_1.tif
TMP_826 = CONVERT REF_252 to uint8
TMP_827(bool) = TMP_826 <= 1
CONDITION TMP_827
 res.basePosted = newOrder.amount
REF_253(uint256) -> res_5.basePosted
REF_254(uint256) -> newOrder_1.amount
res_6(ICLOB.PlaceOrderResult) := phi(['res_5'])
REF_253(uint256) (->res_6) := REF_254(uint256)
res_7(ICLOB.PlaceOrderResult) := phi(['res_6', 'res_5'])
 res.wasMarketOrder = (args.limitPrice == type()(uint256).max)
REF_255(bool) -> res_7.wasMarketOrder
REF_256(uint256) -> args_1.limitPrice
TMP_829(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_830(bool) = REF_256 == TMP_829
res_8(ICLOB.PlaceOrderResult) := phi(['res_7'])
REF_255(bool) (->res_8) := TMP_830(bool)
 OrderProcessed({eventNonce:EventNonceLib.inc(),account:account,orderId:res.orderId,tif:args.tif,limitPrice:args.limitPrice,basePosted:res.basePosted,quoteDelta:res.quoteTokenAmountTraded,baseDelta:res.baseTokenAmountTraded,takerFee:takerFee})
TMP_831(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
REF_258(uint256) -> res_8.orderId
REF_259(ICLOB.TiF) -> args_1.tif
REF_260(uint256) -> args_1.limitPrice
REF_261(uint256) -> res_8.basePosted
REF_262(int256) -> res_8.quoteTokenAmountTraded
REF_263(int256) -> res_8.baseTokenAmountTraded
Emit OrderProcessed(TMP_831,account_1,REF_258,REF_259,REF_260,REF_261,REF_262,REF_263,takerFee_1)
 res
RETURN res_8
```
#### CLOB._removeExpiredAsk(Book,Order) [INTERNAL]
```slithir
ds_1 (-> ['TMP_766', 'TMP_780'])(Book) := phi(["ds_1 (-> ['TMP_766'])", "ds_1 (-> ['TMP_780'])", 'ds_1 (-> [])'])
order_1 (-> ['ds', 'ds'])(Order) := phi(["o_1 (-> ['ds'])", 'order_1 (-> [])', "bestAskOrder_1 (-> ['ds'])"])
 baseTokenAmount = order.amount
REF_504(uint256) -> order_1 (-> ['ds', 'ds']).amount
baseTokenAmount_1(uint256) := REF_504(uint256)
 TransientMakerData.addBaseToken(order.owner,baseTokenAmount)
REF_506(address) -> order_1 (-> ['ds', 'ds']).owner
LIBRARY_CALL, dest:TransientMakerData, function:TransientMakerData.addBaseToken(address,uint256), arguments:['REF_506', 'baseTokenAmount_1'] 
 ds.removeOrderFromBook(order)
LIBRARY_CALL, dest:BookLib, function:BookLib.removeOrderFromBook(Book,Order), arguments:["ds_1 (-> ['TMP_766', 'TMP_780'])", "order_1 (-> ['ds', 'ds'])"]
```
#### CLOB._removeExpiredBid(Book,Order) [INTERNAL]
```slithir
ds_1 (-> ['TMP_766', 'TMP_780'])(Book) := phi(["ds_1 (-> ['TMP_766'])", 'ds_1 (-> [])', "ds_1 (-> ['TMP_780'])"])
order_1 (-> ['ds', 'ds'])(Order) := phi(["o_1 (-> ['ds'])", 'order_1 (-> [])', "bestBidOrder_1 (-> ['ds'])"])
 quoteTokenAmount = ds.getQuoteTokenAmount(order.price,order.amount)
REF_509(uint256) -> order_1 (-> ['ds', 'ds']).price
REF_510(uint256) -> order_1 (-> ['ds', 'ds']).amount
TMP_1006(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getQuoteTokenAmount(Book,uint256,uint256), arguments:["ds_1 (-> ['TMP_766', 'TMP_780'])", 'REF_509', 'REF_510'] 
quoteTokenAmount_1(uint256) := TMP_1006(uint256)
 TransientMakerData.addQuoteToken(order.owner,quoteTokenAmount)
REF_512(address) -> order_1 (-> ['ds', 'ds']).owner
LIBRARY_CALL, dest:TransientMakerData, function:TransientMakerData.addQuoteToken(address,uint256), arguments:['REF_512', 'quoteTokenAmount_1'] 
 ds.removeOrderFromBook(order)
LIBRARY_CALL, dest:BookLib, function:BookLib.removeOrderFromBook(Book,Order), arguments:["ds_1 (-> ['TMP_766', 'TMP_780'])", "order_1 (-> ['ds', 'ds'])"]
```
#### CLOB._removeNonCompetitiveOrder(Book,Order) [INTERNAL]
```slithir
ds_1 (-> ['TMP_780', 'TMP_780'])(Book) := phi(["ds_1 (-> ['TMP_780'])", "ds_1 (-> ['TMP_780'])"])
order_1 (-> [])(Order) := phi(['REF_329', 'REF_305'])
accountManager_8(IAccountManager) := phi(['accountManager_12', 'accountManager_5', 'accountManager_1', 'accountManager_10', 'accountManager_17', 'accountManager_16', 'accountManager_9', 'accountManager_15', 'accountManager_6', 'accountManager_14', 'accountManager_7', 'accountManager_0'])
 order.side == Side.BUY
REF_514(Side) -> order_1 (-> []).side
REF_515(Side) -> Side.BUY
TMP_1009(bool) = REF_514 == REF_515
CONDITION TMP_1009
 quoteRefunded = ds.getQuoteTokenAmount(order.price,order.amount)
REF_517(uint256) -> order_1 (-> []).price
REF_518(uint256) -> order_1 (-> []).amount
TMP_1010(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getQuoteTokenAmount(Book,uint256,uint256), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])", 'REF_517', 'REF_518'] 
quoteRefunded_1(uint256) := TMP_1010(uint256)
 accountManager.creditAccountNoEvent(order.owner,address(ds.config().quoteToken),quoteRefunded)
REF_520(address) -> order_1 (-> []).owner
TMP_1011(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])"] 
REF_522(address) -> TMP_1011.quoteToken
TMP_1012 = CONVERT REF_522 to address
HIGH_LEVEL_CALL, dest:accountManager_8(IAccountManager), function:creditAccountNoEvent, arguments:['REF_520', 'TMP_1012', 'quoteRefunded_1']  
accountManager_10(IAccountManager) := phi(['accountManager_12', 'accountManager_5', 'accountManager_1', 'accountManager_10', 'accountManager_17', 'accountManager_16', 'accountManager_9', 'accountManager_15', 'accountManager_6', 'accountManager_14', 'accountManager_7'])
 baseRefunded = order.amount
REF_523(uint256) -> order_1 (-> []).amount
baseRefunded_1(uint256) := REF_523(uint256)
 accountManager.creditAccountNoEvent(order.owner,address(ds.config().baseToken),baseRefunded)
REF_525(address) -> order_1 (-> []).owner
TMP_1014(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])"] 
REF_527(address) -> TMP_1014.baseToken
TMP_1015 = CONVERT REF_527 to address
HIGH_LEVEL_CALL, dest:accountManager_8(IAccountManager), function:creditAccountNoEvent, arguments:['REF_525', 'TMP_1015', 'baseRefunded_1']  
accountManager_9(IAccountManager) := phi(['accountManager_12', 'accountManager_8', 'accountManager_5', 'accountManager_1', 'accountManager_10', 'accountManager_17', 'accountManager_16', 'accountManager_9', 'accountManager_15', 'accountManager_6', 'accountManager_14', 'accountManager_7'])
quoteRefunded_2(uint256) := phi(['quoteRefunded_0', 'quoteRefunded_1'])
baseRefunded_2(uint256) := phi(['baseRefunded_1', 'baseRefunded_0'])
 OrderCanceled(EventNonceLib.inc(),order.id.unwrap(),order.owner,quoteRefunded,baseRefunded,CancelType.NON_COMPETITIVE)
TMP_1017(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
REF_529(OrderId) -> order_1 (-> []).id
TMP_1018(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_529'] 
REF_531(address) -> order_1 (-> []).owner
REF_532(ICLOB.CancelType) -> CancelType.NON_COMPETITIVE
Emit OrderCanceled(TMP_1017,TMP_1018,REF_531,quoteRefunded_2,baseRefunded_2,REF_532)
 ds.removeOrderFromBook(order)
LIBRARY_CALL, dest:BookLib, function:BookLib.removeOrderFromBook(Book,Order), arguments:["ds_1 (-> ['TMP_780', 'TMP_780'])", 'order_1 (-> [])']
```
#### CLOB._settleAmend(Book,address,int256,int256) [INTERNAL]
```slithir
ds_1 (-> ['TMP_790'])(Book) := phi(["ds_1 (-> ['TMP_790'])"])
maker_1(address) := phi(['maker_1'])
quoteTokenDelta_1(int256) := phi(['quoteTokenDelta_4'])
baseTokenDelta_1(int256) := phi(['baseTokenDelta_4'])
accountManager_13(IAccountManager) := phi(['accountManager_12', 'accountManager_5', 'accountManager_1', 'accountManager_10', 'accountManager_17', 'accountManager_16', 'accountManager_9', 'accountManager_15', 'accountManager_6', 'accountManager_14', 'accountManager_7', 'accountManager_0'])
 quoteTokenDelta > 0
TMP_1038(bool) = quoteTokenDelta_1 > 0
CONDITION TMP_1038
 accountManager.creditAccount(maker,address(ds.config().quoteToken),uint256(quoteTokenDelta))
TMP_1039(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:["ds_1 (-> ['TMP_790'])"] 
REF_568(address) -> TMP_1039.quoteToken
TMP_1040 = CONVERT REF_568 to address
TMP_1041 = CONVERT quoteTokenDelta_1 to uint256
HIGH_LEVEL_CALL, dest:accountManager_13(IAccountManager), function:creditAccount, arguments:['maker_1', 'TMP_1040', 'TMP_1041']  
accountManager_14(IAccountManager) := phi(['accountManager_12', 'accountManager_5', 'accountManager_1', 'accountManager_10', 'accountManager_17', 'accountManager_16', 'accountManager_9', 'accountManager_15', 'accountManager_6', 'accountManager_14', 'accountManager_7', 'accountManager_13'])
 quoteTokenDelta < 0
TMP_1043(bool) = quoteTokenDelta_1 < 0
CONDITION TMP_1043
 accountManager.debitAccount(maker,address(ds.config().quoteToken),uint256(- quoteTokenDelta))
TMP_1044(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:["ds_1 (-> ['TMP_790'])"] 
REF_571(address) -> TMP_1044.quoteToken
TMP_1045 = CONVERT REF_571 to address
TMP_1046(int256) = 0 (c)- quoteTokenDelta_1
TMP_1047 = CONVERT TMP_1046 to uint256
HIGH_LEVEL_CALL, dest:accountManager_13(IAccountManager), function:debitAccount, arguments:['maker_1', 'TMP_1045', 'TMP_1047']  
accountManager_15(IAccountManager) := phi(['accountManager_12', 'accountManager_5', 'accountManager_1', 'accountManager_10', 'accountManager_17', 'accountManager_16', 'accountManager_9', 'accountManager_15', 'accountManager_6', 'accountManager_14', 'accountManager_7'])
 baseTokenDelta > 0
TMP_1049(bool) = baseTokenDelta_1 > 0
CONDITION TMP_1049
 accountManager.creditAccount(maker,address(ds.config().baseToken),uint256(baseTokenDelta))
TMP_1050(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:["ds_1 (-> ['TMP_790'])"] 
REF_574(address) -> TMP_1050.baseToken
TMP_1051 = CONVERT REF_574 to address
TMP_1052 = CONVERT baseTokenDelta_1 to uint256
HIGH_LEVEL_CALL, dest:accountManager_15(IAccountManager), function:creditAccount, arguments:['maker_1', 'TMP_1051', 'TMP_1052']  
accountManager_17(IAccountManager) := phi(['accountManager_12', 'accountManager_5', 'accountManager_1', 'accountManager_10', 'accountManager_17', 'accountManager_16', 'accountManager_9', 'accountManager_15', 'accountManager_6', 'accountManager_14', 'accountManager_7'])
 baseTokenDelta < 0
TMP_1054(bool) = baseTokenDelta_1 < 0
CONDITION TMP_1054
 accountManager.debitAccount(maker,address(ds.config().baseToken),uint256(- baseTokenDelta))
TMP_1055(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:["ds_1 (-> ['TMP_790'])"] 
REF_577(address) -> TMP_1055.baseToken
TMP_1056 = CONVERT REF_577 to address
TMP_1057(int256) = 0 (c)- baseTokenDelta_1
TMP_1058 = CONVERT TMP_1057 to uint256
HIGH_LEVEL_CALL, dest:accountManager_15(IAccountManager), function:debitAccount, arguments:['maker_1', 'TMP_1056', 'TMP_1058']  
accountManager_16(IAccountManager) := phi(['accountManager_12', 'accountManager_5', 'accountManager_1', 'accountManager_10', 'accountManager_17', 'accountManager_16', 'accountManager_9', 'accountManager_15', 'accountManager_6', 'accountManager_14', 'accountManager_7'])
```
#### CLOB._settleIncomingOrder(Book,address,Side,uint256,uint256) [INTERNAL]
```slithir
ds_1 (-> ['TMP_780', 'TMP_780', 'TMP_766'])(Book) := phi(["ds_1 (-> ['TMP_780'])", "ds_1 (-> ['TMP_780'])", "ds_1 (-> ['TMP_766'])"])
account_1(address) := phi(['account_1', 'account_1', 'TMP_773'])
side_1(Side) := phi(['REF_266', 'virtualTakerSide_3', 'REF_244'])
quoteTokenAmount_1(uint256) := phi(['totalQuoteReceived_1', 'TMP_820'])
baseTokenAmount_1(uint256) := phi(['TMP_843', 'totalBaseReceived_1'])
accountManager_11(IAccountManager) := phi(['accountManager_12', 'accountManager_5', 'accountManager_1', 'accountManager_10', 'accountManager_17', 'accountManager_16', 'accountManager_9', 'accountManager_15', 'accountManager_6', 'accountManager_14', 'accountManager_7', 'accountManager_0'])
 (settleParams.quoteToken,settleParams.baseToken) = (ds.config().quoteToken,ds.config().baseToken)
REF_553(address) -> settleParams_0.quoteToken
REF_554(address) -> settleParams_0.baseToken
TMP_1034(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:["ds_1 (-> ['TMP_780', 'TMP_780', 'TMP_766'])"] 
REF_556(address) -> TMP_1034.quoteToken
TMP_1035(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:["ds_1 (-> ['TMP_780', 'TMP_780', 'TMP_766'])"] 
REF_558(address) -> TMP_1035.baseToken
settleParams_1(ICLOB.SettleParams) := phi(['settleParams_0'])
REF_553(address) (->settleParams_1) := REF_556(address)
settleParams_2(ICLOB.SettleParams) := phi(['settleParams_0'])
REF_554(address) (->settleParams_2) := REF_558(address)
 settleParams.taker = account
REF_559(address) -> settleParams_2.taker
settleParams_3(ICLOB.SettleParams) := phi(['settleParams_2'])
REF_559(address) (->settleParams_3) := account_1(address)
 settleParams.side = side
REF_560(Side) -> settleParams_3.side
settleParams_4(ICLOB.SettleParams) := phi(['settleParams_3'])
REF_560(Side) (->settleParams_4) := side_1(Side)
 settleParams.takerQuoteAmount = quoteTokenAmount
REF_561(uint256) -> settleParams_4.takerQuoteAmount
settleParams_5(ICLOB.SettleParams) := phi(['settleParams_4'])
REF_561(uint256) (->settleParams_5) := quoteTokenAmount_1(uint256)
 settleParams.takerBaseAmount = baseTokenAmount
REF_562(uint256) -> settleParams_5.takerBaseAmount
settleParams_6(ICLOB.SettleParams) := phi(['settleParams_5'])
REF_562(uint256) (->settleParams_6) := baseTokenAmount_1(uint256)
 settleParams.makerCredits = TransientMakerData.getMakerCreditsAndClearStorage()
REF_563(MakerCredit[]) -> settleParams_6.makerCredits
TMP_1036(MakerCredit[]) = LIBRARY_CALL, dest:TransientMakerData, function:TransientMakerData.getMakerCreditsAndClearStorage(), arguments:[] 
settleParams_7(ICLOB.SettleParams) := phi(['settleParams_6'])
REF_563(MakerCredit[]) (->settleParams_7) := TMP_1036(MakerCredit[])
 accountManager.settleIncomingOrder(settleParams)
TMP_1037(uint256) = HIGH_LEVEL_CALL, dest:accountManager_11(IAccountManager), function:settleIncomingOrder, arguments:['settleParams_7']  
accountManager_12(IAccountManager) := phi(['accountManager_12', 'accountManager_5', 'accountManager_1', 'accountManager_10', 'accountManager_17', 'accountManager_16', 'accountManager_9', 'accountManager_15', 'accountManager_6', 'accountManager_14', 'accountManager_7', 'accountManager_11'])
RETURN TMP_1037
 takerFee
```
#### CLOB.adminCancelExpiredOrders(OrderId[],Side) [EXTERNAL]
```slithir
 removed = new bool[](ids.length)
REF_200 -> LENGTH ids_1
TMP_765(bool[])  = new bool[](REF_200)
removed_1(bool[]) = ['TMP_765(bool[])']
 ds = _getStorage()
TMP_766(Book) = INTERNAL_CALL, CLOB._getStorage()()
ds_1 (-> ['TMP_766'])(Book) := TMP_766(Book)
 i = 0
i_1(uint256) := 0(uint256)
 i < ids.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_201 -> LENGTH ids_1
TMP_767(bool) = i_2 < REF_201
CONDITION TMP_767
 o = ds.orders[ids[i]]
REF_202(mapping(OrderId => Order)) -> ds_1 (-> ['TMP_766']).orders
REF_203(OrderId) -> ids_1[i_2]
REF_204(Order) -> REF_202[REF_203]
o_1 (-> ['ds'])(Order) := REF_204(Order)
 ! o.isExpired() || o.side != side
TMP_768(bool) = LIBRARY_CALL, dest:OrderLib, function:OrderLib.isExpired(Order), arguments:["o_1 (-> ['ds'])"] 
TMP_769 = UnaryType.BANG TMP_768 
REF_206(Side) -> o_1 (-> ['ds']).side
TMP_770(bool) = REF_206 != side_1
TMP_771(bool) = TMP_769 || TMP_770
CONDITION TMP_771
 removed[i] = true
REF_207(bool) -> removed_1[i_2]
removed_2(bool[]) := phi(['removed_1'])
REF_207(bool) (->removed_2) := True(bool)
 i ++
removed_3(bool[]) := phi(['removed_1', 'removed_2'])
TMP_772(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 _settleIncomingOrder({ds:ds,account:address(0),side:virtualTakerSide,quoteTokenAmount:0,baseTokenAmount:0})
TMP_773 = CONVERT 0 to address
TMP_774(uint256) = INTERNAL_CALL, CLOB._settleIncomingOrder(Book,address,Side,uint256,uint256)(ds_1 (-> ['TMP_766']),TMP_773,virtualTakerSide_3,0,0)
 removed
RETURN removed_1
 onlyManager()
MODIFIER_CALL, CLOB.onlyManager()()
 side == Side.BUY
REF_208(Side) -> Side.BUY
TMP_776(bool) = side_1 == REF_208
CONDITION TMP_776
 _removeExpiredBid(ds,o)
INTERNAL_CALL, CLOB._removeExpiredBid(Book,Order)(ds_1 (-> ['TMP_766']),o_1 (-> ['ds']))
 _removeExpiredAsk(ds,o)
INTERNAL_CALL, CLOB._removeExpiredAsk(Book,Order)(ds_1 (-> ['TMP_766']),o_1 (-> ['ds']))
 side == Side.BUY
REF_209(Side) -> Side.BUY
TMP_779(bool) = side_1 == REF_209
CONDITION TMP_779
 virtualTakerSide = Side.SELL
REF_210(Side) -> Side.SELL
virtualTakerSide_2(Side) := REF_210(Side)
 virtualTakerSide = Side.BUY
REF_211(Side) -> Side.BUY
virtualTakerSide_1(Side) := REF_211(Side)
virtualTakerSide_3(Side) := phi(['virtualTakerSide_1', 'virtualTakerSide_2'])
```
#### CLOB.amend(address,ICLOB.AmendArgs) [EXTERNAL]
```slithir
 ds = _getStorage()
TMP_790(Book) = INTERNAL_CALL, CLOB._getStorage()()
ds_1 (-> ['TMP_790'])(Book) := TMP_790(Book)
 order = ds.orders[args.orderId.toOrderId()]
REF_221(mapping(OrderId => Order)) -> ds_1 (-> ['TMP_790']).orders
REF_222(uint256) -> args_1.orderId
TMP_791(OrderId) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.toOrderId(uint256), arguments:['REF_222'] 
REF_224(Order) -> REF_221[TMP_791]
order_1 (-> ['ds'])(Order) := REF_224(Order)
 order.id.unwrap() == 0
REF_225(OrderId) -> order_1 (-> ['ds']).id
TMP_792(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_225'] 
TMP_793(bool) = TMP_792 == 0
CONDITION TMP_793
 OrderLib.OrderNotFound()
TMP_794(None) = SOLIDITY_CALL revert OrderNotFound()()
 order.owner != account
REF_227(address) -> order_1 (-> ['ds']).owner
TMP_795(bool) = REF_227 != account_1
CONDITION TMP_795
 revert AmendUnauthorized()()
TMP_796(None) = SOLIDITY_CALL revert AmendUnauthorized()()
 ds.assertLimitPriceInBounds(args.price)
REF_229(uint256) -> args_1.price
LIBRARY_CALL, dest:BookLib, function:BookLib.assertLimitPriceInBounds(Book,uint256), arguments:["ds_1 (-> ['TMP_790'])", 'REF_229'] 
 ds.assertMakeAmountInBounds(args.amountInBase)
REF_231(uint256) -> args_1.amountInBase
LIBRARY_CALL, dest:BookLib, function:BookLib.assertMakeAmountInBounds(Book,uint256), arguments:["ds_1 (-> ['TMP_790'])", 'REF_231'] 
 args.cancelTimestamp.isExpired()
REF_232(uint32) -> args_1.cancelTimestamp
TMP_799(bool) = LIBRARY_CALL, dest:OrderLib, function:OrderLib.isExpired(uint256), arguments:['REF_232'] 
CONDITION TMP_799
 revert AmendInvalid()()
TMP_800(None) = SOLIDITY_CALL revert AmendInvalid()()
 (quoteDelta,baseDelta) = _processAmend(ds,order,args)
TUPLE_2(int256,int256) = INTERNAL_CALL, CLOB._processAmend(Book,Order,ICLOB.AmendArgs)(ds_1 (-> ['TMP_790']),order_1 (-> ['ds']),args_1)
quoteDelta_1(int256)= UNPACK TUPLE_2 index: 0 
baseDelta_1(int256)= UNPACK TUPLE_2 index: 1 
 onlySenderOrOperator(account,SpotOperatorRoles.PLACE_ORDER)
REF_234(SpotOperatorRoles) -> SpotOperatorRoles.PLACE_ORDER
MODIFIER_CALL, CLOB.onlySenderOrOperator(address,SpotOperatorRoles)(account_1,REF_234)
 (quoteDelta,baseDelta)
RETURN quoteDelta_1,baseDelta_1
```
#### CLOB.cancel(address,ICLOB.CancelArgs) [EXTERNAL]
```slithir
accountManager_2(IAccountManager) := phi(['accountManager_12', 'accountManager_5', 'accountManager_1', 'accountManager_10', 'accountManager_17', 'accountManager_16', 'accountManager_9', 'accountManager_15', 'accountManager_6', 'accountManager_14', 'accountManager_7', 'accountManager_0'])
 ds = _getStorage()
TMP_802(Book) = INTERNAL_CALL, CLOB._getStorage()()
ds_1 (-> ['TMP_802'])(Book) := TMP_802(Book)
 quoteToken = ds.config().quoteToken
TMP_803(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:["ds_1 (-> ['TMP_802'])"] 
REF_236(address) -> TMP_803.quoteToken
quoteToken_1(address) := REF_236(address)
 baseToken = ds.config().baseToken
TMP_804(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:["ds_1 (-> ['TMP_802'])"] 
REF_238(address) -> TMP_804.baseToken
baseToken_1(address) := REF_238(address)
 (totalQuoteTokenRefunded,totalBaseTokenRefunded) = _executeCancel(ds,account,args)
TUPLE_3(uint256,uint256) = INTERNAL_CALL, CLOB._executeCancel(Book,address,ICLOB.CancelArgs)(ds_1 (-> ['TMP_802']),account_1,args_1)
totalQuoteTokenRefunded_1(uint256)= UNPACK TUPLE_3 index: 0 
totalBaseTokenRefunded_1(uint256)= UNPACK TUPLE_3 index: 1 
 totalBaseTokenRefunded > 0
TMP_805(bool) = totalBaseTokenRefunded_1 > 0
CONDITION TMP_805
 accountManager.creditAccount(account,baseToken,totalBaseTokenRefunded)
HIGH_LEVEL_CALL, dest:accountManager_5(IAccountManager), function:creditAccount, arguments:['account_1', 'baseToken_1', 'totalBaseTokenRefunded_1']  
accountManager_6(IAccountManager) := phi(['accountManager_12', 'accountManager_5', 'accountManager_1', 'accountManager_10', 'accountManager_17', 'accountManager_16', 'accountManager_9', 'accountManager_15', 'accountManager_6', 'accountManager_14', 'accountManager_7'])
 totalQuoteTokenRefunded > 0
TMP_807(bool) = totalQuoteTokenRefunded_1 > 0
CONDITION TMP_807
 accountManager.creditAccount(account,quoteToken,totalQuoteTokenRefunded)
HIGH_LEVEL_CALL, dest:accountManager_6(IAccountManager), function:creditAccount, arguments:['account_1', 'quoteToken_1', 'totalQuoteTokenRefunded_1']  
accountManager_7(IAccountManager) := phi(['accountManager_12', 'accountManager_5', 'accountManager_1', 'accountManager_10', 'accountManager_17', 'accountManager_16', 'accountManager_9', 'accountManager_15', 'accountManager_6', 'accountManager_14', 'accountManager_7'])
 (totalQuoteTokenRefunded,totalBaseTokenRefunded)
RETURN totalQuoteTokenRefunded_1,totalBaseTokenRefunded_1
 onlySenderOrOperator(account,SpotOperatorRoles.PLACE_ORDER)
REF_241(SpotOperatorRoles) -> SpotOperatorRoles.PLACE_ORDER
MODIFIER_CALL, CLOB.onlySenderOrOperator(address,SpotOperatorRoles)(account_1,REF_241)
```
#### CLOB.constructor(address,address,address,uint256) [PUBLIC]
```slithir
 factory = ICLOBManager(_factory)
TMP_698 = CONVERT _factory_1 to ICLOBManager
factory_1(ICLOBManager) := TMP_698(ICLOBManager)
 gteRouter = _gteRouter
gteRouter_1(address) := _gteRouter_1(address)
 operator = IOperatorPanel(_accountManager)
TMP_699 = CONVERT _accountManager_1 to IOperatorPanel
operator_1(IOperatorPanel) := TMP_699(IOperatorPanel)
 accountManager = IAccountManager(_accountManager)
TMP_700 = CONVERT _accountManager_1 to IAccountManager
accountManager_1(IAccountManager) := TMP_700(IAccountManager)
 maxNumOrdersPerSide = _maxNumOrdersPerSide
maxNumOrdersPerSide_1(uint256) := _maxNumOrdersPerSide_1(uint256)
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### CLOB.getBaseQuanta() [EXTERNAL]
```slithir
 _getStorage().getBaseQuanta()
TMP_750(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_751(uint256) = LIBRARY_CALL, dest:BookLib, function:BookLib.getBaseQuanta(Book), arguments:['TMP_750'] 
RETURN TMP_751
```
#### CLOB.getBaseToken() [EXTERNAL]
```slithir
 _getStorage().config().baseToken
TMP_704(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_705(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:['TMP_704'] 
REF_148(address) -> TMP_705.baseToken
RETURN REF_148
```
#### CLOB.getBaseTokenAmount(uint256,uint256) [EXTERNAL]
```slithir
 _getStorage().getBaseTokenAmount(price,quoteAmount)
TMP_708(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_709(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getBaseTokenAmount(Book,uint256,uint256), arguments:['TMP_708', 'price_1', 'quoteAmount_1'] 
RETURN TMP_709
```
#### CLOB.getEventNonce() [EXTERNAL]
```slithir
 EventNonceLib.getCurrentNonce()
TMP_746(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.getCurrentNonce(), arguments:[] 
RETURN TMP_746
```
#### CLOB.getLimit(uint256,Side) [EXTERNAL]
```slithir
 _getStorage().getLimit(price,side)
TMP_730(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_731(Limit) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getLimit(Book,uint256,Side), arguments:['TMP_730', 'price_1', 'side_1'] 
RETURN TMP_731
```
#### CLOB.getLotSizeInBase() [EXTERNAL]
```slithir
 _getStorage().settings().lotSizeInBase
TMP_718(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_719(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:['TMP_718'] 
REF_158(uint256) -> TMP_719.lotSizeInBase
RETURN REF_158
```
#### CLOB.getMarketConfig() [EXTERNAL]
```slithir
 _getStorage().config()
TMP_712(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_713(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:['TMP_712'] 
RETURN TMP_713
```
#### CLOB.getMarketSettings() [EXTERNAL]
```slithir
 _getStorage().settings()
TMP_714(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_715(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:['TMP_714'] 
RETURN TMP_715
```
#### CLOB.getNextBiggestPrice(uint256,Side) [EXTERNAL]
```slithir
 _getStorage().getNextBiggestPrice(price,side)
TMP_739(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_740(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getNextBiggestPrice(Book,uint256,Side), arguments:['TMP_739', 'price_1', 'side_1'] 
RETURN TMP_740
```
#### CLOB.getNextOrderId() [EXTERNAL]
```slithir
 (_getStorage().metadata().orderIdCounter + 1)
TMP_743(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_744(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:['TMP_743'] 
REF_178(uint96) -> TMP_744.orderIdCounter
TMP_745(uint96) = REF_178 (c)+ 1
RETURN TMP_745
```
#### CLOB.getNextOrders(uint256,uint256) [EXTERNAL]
```slithir
 _getStorage().getNextOrders(startOrderId.toOrderId(),numOrders)
TMP_736(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_737(OrderId) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.toOrderId(uint256), arguments:['startOrderId_1'] 
TMP_738(Order[]) = LIBRARY_CALL, dest:BookLib, function:BookLib.getNextOrders(Book,OrderId,uint256), arguments:['TMP_736', 'TMP_737', 'numOrders_1'] 
RETURN TMP_738
```
#### CLOB.getNextSmallestPrice(uint256,Side) [EXTERNAL]
```slithir
 _getStorage().getNextSmallestPrice(price,side)
TMP_741(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_742(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getNextSmallestPrice(Book,uint256,Side), arguments:['TMP_741', 'price_1', 'side_1'] 
RETURN TMP_742
```
#### CLOB.getNumAsks() [EXTERNAL]
```slithir
 _getStorage().metadata().numAsks
TMP_734(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_735(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:['TMP_734'] 
REF_172(uint256) -> TMP_735.numAsks
RETURN REF_172
```
#### CLOB.getNumBids() [EXTERNAL]
```slithir
 _getStorage().metadata().numBids
TMP_732(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_733(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:['TMP_732'] 
REF_170(uint256) -> TMP_733.numBids
RETURN REF_170
```
#### CLOB.getOpenInterest() [EXTERNAL]
```slithir
 (_getStorage().metadata().quoteTokenOpenInterest,_getStorage().metadata().baseTokenOpenInterest)
TMP_720(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_721(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:['TMP_720'] 
REF_160(uint256) -> TMP_721.quoteTokenOpenInterest
TMP_722(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_723(MarketMetadata) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.metadata(Book), arguments:['TMP_722'] 
REF_162(uint256) -> TMP_723.baseTokenOpenInterest
RETURN REF_160,REF_162
 (quoteOi,baseOi)
```
#### CLOB.getOrder(uint256) [EXTERNAL]
```slithir
 _getStorage().orders[orderId.toOrderId()]
TMP_724(Book) = INTERNAL_CALL, CLOB._getStorage()()
REF_163(mapping(OrderId => Order)) -> TMP_724.orders
TMP_725(OrderId) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.toOrderId(uint256), arguments:['orderId_1'] 
REF_165(Order) -> REF_163[TMP_725]
RETURN REF_165
```
#### CLOB.getOrdersPaginated(OrderId,uint256) [EXTERNAL]
```slithir
 ds = _getStorage()
TMP_749(Book) = INTERNAL_CALL, CLOB._getStorage()()
ds_1 (-> ['TMP_749'])(Book) := TMP_749(Book)
 nextOrder = ds.orders[startOrderId]
REF_192(mapping(OrderId => Order)) -> ds_1 (-> ['TMP_749']).orders
REF_193(Order) -> REF_192[startOrderId_1]
nextOrder_1(Order) := REF_193(Order)
 ds.getOrdersPaginated(nextOrder,pageSize)
TUPLE_1(Order[],Order) = LIBRARY_CALL, dest:BookLib, function:BookLib.getOrdersPaginated(Book,Order,uint256), arguments:["ds_1 (-> ['TMP_749'])", 'nextOrder_1', 'pageSize_1'] 
RETURN TUPLE_1
 (result,nextOrder)
```
#### CLOB.getQuoteToken() [EXTERNAL]
```slithir
 _getStorage().config().quoteToken
TMP_706(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_707(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:['TMP_706'] 
REF_150(address) -> TMP_707.quoteToken
RETURN REF_150
```
#### CLOB.getQuoteTokenAmount(uint256,uint256) [EXTERNAL]
```slithir
 _getStorage().getQuoteTokenAmount(price,baseAmount)
TMP_710(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_711(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getQuoteTokenAmount(Book,uint256,uint256), arguments:['TMP_710', 'price_1', 'baseAmount_1'] 
RETURN TMP_711
```
#### CLOB.getTOB() [EXTERNAL]
```slithir
 (_getStorage().getBestBidPrice(),_getStorage().getBestAskPrice())
TMP_726(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_727(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getBestBidPrice(Book), arguments:['TMP_726'] 
TMP_728(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_729(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getBestAskPrice(Book), arguments:['TMP_728'] 
RETURN TMP_727,TMP_729
 (maxBid,minAsk)
```
#### CLOB.getTickSize() [EXTERNAL]
```slithir
 _getStorage().settings().tickSize
TMP_716(Book) = INTERNAL_CALL, CLOB._getStorage()()
TMP_717(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:['TMP_716'] 
REF_156(uint256) -> TMP_717.tickSize
RETURN REF_156
```
#### CLOB.initialize(MarketConfig,MarketSettings,address) [EXTERNAL]
```slithir
 __CLOB_init(marketConfig,marketSettings,initialOwner)
INTERNAL_CALL, CLOB.__CLOB_init(MarketConfig,MarketSettings,address)(marketConfig_1,marketSettings_1,initialOwner_1)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### CLOB.placeOrder(address,ICLOB.PlaceOrderArgs) [EXTERNAL]
```slithir
 ds = _getStorage()
TMP_780(Book) = INTERNAL_CALL, CLOB._getStorage()()
ds_1 (-> ['TMP_780'])(Book) := TMP_780(Book)
 orderId = ds.incrementOrderId()
TMP_781(uint256) = LIBRARY_CALL, dest:BookLib, function:BookLib.incrementOrderId(Book), arguments:["ds_1 (-> ['TMP_780'])"] 
orderId_1(uint256) := TMP_781(uint256)
 args.clientOrderId > 0
REF_213(uint96) -> args_1.clientOrderId
TMP_782(bool) = REF_213 > 0
CONDITION TMP_782
 orderId = account.getClientOrderId(args.clientOrderId)
REF_215(uint96) -> args_1.clientOrderId
TMP_783(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.getClientOrderId(address,uint96), arguments:['account_1', 'REF_215'] 
orderId_2(uint256) := TMP_783(uint256)
 ds.assertUnusedOrderId(orderId)
LIBRARY_CALL, dest:BookLib, function:BookLib.assertUnusedOrderId(Book,uint256), arguments:["ds_1 (-> ['TMP_780'])", 'orderId_2'] 
orderId_3(uint256) := phi(['orderId_2', 'orderId_1'])
 newOrder = args.toOrderChecked(orderId,account)
TMP_785(Order) = LIBRARY_CALL, dest:OrderLib, function:OrderLib.toOrderChecked(ICLOB.PlaceOrderArgs,uint256,address), arguments:['args_1', 'orderId_3', 'account_1'] 
newOrder_1(Order) := TMP_785(Order)
 args.side == Side.BUY
REF_218(Side) -> args_1.side
REF_219(Side) -> Side.BUY
TMP_786(bool) = REF_218 == REF_219
CONDITION TMP_786
 _processBid(ds,account,newOrder,args)
TMP_787(ICLOB.PlaceOrderResult) = INTERNAL_CALL, CLOB._processBid(Book,address,Order,ICLOB.PlaceOrderArgs)(ds_1 (-> ['TMP_780']),account_1,newOrder_1,args_1)
RETURN TMP_787
 _processAsk(ds,account,newOrder,args)
TMP_788(ICLOB.PlaceOrderResult) = INTERNAL_CALL, CLOB._processAsk(Book,address,Order,ICLOB.PlaceOrderArgs)(ds_1 (-> ['TMP_780']),account_1,newOrder_1,args_1)
RETURN TMP_788
 onlySenderOrOperator(account,SpotOperatorRoles.PLACE_ORDER)
REF_220(SpotOperatorRoles) -> SpotOperatorRoles.PLACE_ORDER
MODIFIER_CALL, CLOB.onlySenderOrOperator(address,SpotOperatorRoles)(account_1,REF_220)
```
#### CLOB.setLotSizeInBase(uint256) [EXTERNAL]
```slithir
 _getStorage().setLotSizeInBase(newLotSizeInBase)
TMP_761(Book) = INTERNAL_CALL, CLOB._getStorage()()
LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.setLotSizeInBase(Book,uint256), arguments:['TMP_761', 'newLotSizeInBase_1'] 
 onlyManager()
MODIFIER_CALL, CLOB.onlyManager()()
```
#### CLOB.setMaxLimitsPerTx(uint8) [EXTERNAL]
```slithir
 _getStorage().setMaxLimitsPerTx(newMaxLimits)
TMP_752(Book) = INTERNAL_CALL, CLOB._getStorage()()
LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.setMaxLimitsPerTx(Book,uint8), arguments:['TMP_752', 'newMaxLimits_1'] 
 onlyManager()
MODIFIER_CALL, CLOB.onlyManager()()
```
#### CLOB.setMinLimitOrderAmountInBase(uint256) [EXTERNAL]
```slithir
 _getStorage().setMinLimitOrderAmountInBase(newMinLimitOrderAmountInBase)
TMP_758(Book) = INTERNAL_CALL, CLOB._getStorage()()
LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.setMinLimitOrderAmountInBase(Book,uint256), arguments:['TMP_758', 'newMinLimitOrderAmountInBase_1'] 
 onlyManager()
MODIFIER_CALL, CLOB.onlyManager()()
```
#### CLOB.setTickSize(uint256) [EXTERNAL]
```slithir
 _getStorage().setTickSize(tickSize)
TMP_755(Book) = INTERNAL_CALL, CLOB._getStorage()()
LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.setTickSize(Book,uint256), arguments:['TMP_755', 'tickSize_1'] 
 onlyManager()
MODIFIER_CALL, CLOB.onlyManager()()
```

#### CLOBStorageLib.init(Book,MarketConfig,MarketSettings) [INTERNAL]
```slithir
 cs = self.config()
TMP_1843(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:['self_1 (-> [])'] 
cs_1 (-> ['TMP_1843'])(MarketConfig) := TMP_1843(MarketConfig)
 ss = self.settings()
TMP_1844(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:['self_1 (-> [])'] 
ss_1 (-> ['TMP_1844'])(MarketSettings) := TMP_1844(MarketSettings)
 cs.quoteToken = marketConfig.quoteToken
REF_875(address) -> cs_1 (-> ['TMP_1843']).quoteToken
REF_876(address) -> marketConfig_1.quoteToken
cs_2 (-> ['TMP_1843'])(MarketConfig) := phi(["cs_1 (-> ['TMP_1843'])"])
REF_875(address) (->cs_2 (-> ['TMP_1843'])) := REF_876(address)
TMP_1843(MarketConfig) := phi(["cs_2 (-> ['TMP_1843'])"])
 cs.baseToken = marketConfig.baseToken
REF_877(address) -> cs_2 (-> ['TMP_1843']).baseToken
REF_878(address) -> marketConfig_1.baseToken
cs_3 (-> ['TMP_1843'])(MarketConfig) := phi(["cs_2 (-> ['TMP_1843'])"])
REF_877(address) (->cs_3 (-> ['TMP_1843'])) := REF_878(address)
TMP_1843(MarketConfig) := phi(["cs_3 (-> ['TMP_1843'])"])
 cs.quoteSize = marketConfig.quoteSize
REF_879(uint256) -> cs_3 (-> ['TMP_1843']).quoteSize
REF_880(uint256) -> marketConfig_1.quoteSize
cs_4 (-> ['TMP_1843'])(MarketConfig) := phi(["cs_3 (-> ['TMP_1843'])"])
REF_879(uint256) (->cs_4 (-> ['TMP_1843'])) := REF_880(uint256)
TMP_1843(MarketConfig) := phi(["cs_4 (-> ['TMP_1843'])"])
 cs.baseSize = marketConfig.baseSize
REF_881(uint256) -> cs_4 (-> ['TMP_1843']).baseSize
REF_882(uint256) -> marketConfig_1.baseSize
cs_5 (-> ['TMP_1843'])(MarketConfig) := phi(["cs_4 (-> ['TMP_1843'])"])
REF_881(uint256) (->cs_5 (-> ['TMP_1843'])) := REF_882(uint256)
TMP_1843(MarketConfig) := phi(["cs_5 (-> ['TMP_1843'])"])
 ss.status = marketSettings.status
REF_883(bool) -> ss_1 (-> ['TMP_1844']).status
REF_884(bool) -> marketSettings_1.status
ss_2 (-> ['TMP_1844'])(MarketSettings) := phi(["ss_1 (-> ['TMP_1844'])"])
REF_883(bool) (->ss_2 (-> ['TMP_1844'])) := REF_884(bool)
TMP_1844(MarketSettings) := phi(["ss_2 (-> ['TMP_1844'])"])
 ss.maxLimitsPerTx = marketSettings.maxLimitsPerTx
REF_885(uint8) -> ss_2 (-> ['TMP_1844']).maxLimitsPerTx
REF_886(uint8) -> marketSettings_1.maxLimitsPerTx
ss_3 (-> ['TMP_1844'])(MarketSettings) := phi(["ss_2 (-> ['TMP_1844'])"])
REF_885(uint8) (->ss_3 (-> ['TMP_1844'])) := REF_886(uint8)
TMP_1844(MarketSettings) := phi(["ss_3 (-> ['TMP_1844'])"])
 ss.minLimitOrderAmountInBase = marketSettings.minLimitOrderAmountInBase
REF_887(uint256) -> ss_3 (-> ['TMP_1844']).minLimitOrderAmountInBase
REF_888(uint256) -> marketSettings_1.minLimitOrderAmountInBase
ss_4 (-> ['TMP_1844'])(MarketSettings) := phi(["ss_3 (-> ['TMP_1844'])"])
REF_887(uint256) (->ss_4 (-> ['TMP_1844'])) := REF_888(uint256)
TMP_1844(MarketSettings) := phi(["ss_4 (-> ['TMP_1844'])"])
 ss.tickSize = marketSettings.tickSize
REF_889(uint256) -> ss_4 (-> ['TMP_1844']).tickSize
REF_890(uint256) -> marketSettings_1.tickSize
ss_5 (-> ['TMP_1844'])(MarketSettings) := phi(["ss_4 (-> ['TMP_1844'])"])
REF_889(uint256) (->ss_5 (-> ['TMP_1844'])) := REF_890(uint256)
TMP_1844(MarketSettings) := phi(["ss_5 (-> ['TMP_1844'])"])
 ss.lotSizeInBase = marketSettings.lotSizeInBase
REF_891(uint256) -> ss_5 (-> ['TMP_1844']).lotSizeInBase
REF_892(uint256) -> marketSettings_1.lotSizeInBase
ss_6 (-> ['TMP_1844'])(MarketSettings) := phi(["ss_5 (-> ['TMP_1844'])"])
REF_891(uint256) (->ss_6 (-> ['TMP_1844'])) := REF_892(uint256)
TMP_1844(MarketSettings) := phi(["ss_6 (-> ['TMP_1844'])"])
```
#### CLOBStorageLib.getQuoteTokenAmount(Book,uint256,uint256) [INTERNAL]
```slithir
 baseAmount * price / self.config().baseSize
TMP_1814(uint256) = baseAmount_1 (c)* price_1
TMP_1815(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:['self_1 (-> [])'] 
REF_854(uint256) -> TMP_1815.baseSize
TMP_1816(uint256) = TMP_1814 (c)/ REF_854
RETURN TMP_1816
 quoteAmount
```
#### CLOBStorageLib.metadata(Book) [INTERNAL]
```slithir
 _getMarketMetadataStorage()
TMP_1799(MarketMetadata) = INTERNAL_CALL, CLOBStorageLib._getMarketMetadataStorage()()
RETURN TMP_1799
```
#### TransientMakerData.addBaseToken(address,uint256) [INTERNAL]
```slithir
TRANSIENT_CREDITS_POSITION_2(bytes32) := phi(['TRANSIENT_CREDITS_POSITION_0'])
 slot = keccak256(bytes)(abi.encode(TRANSIENT_CREDITS_POSITION,maker))
TMP_2021(bytes) = SOLIDITY_CALL abi.encode()(TRANSIENT_CREDITS_POSITION_2,maker_1)
TMP_2022(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_2021)
slot_1(bytes32) := TMP_2022(bytes32)
 err = revert ArithmeticOverflow().selector
REF_982(bytes4) (->None) := 3833512052(bytes4)
err_1(bytes4) := REF_982(bytes4)
 exists = ! ! tload(uint256)(slot)
TMP_2023(uint256) = SOLIDITY_CALL tload(uint256)(slot_1)
TMP_2024 = UnaryType.BANG TMP_2023 
TMP_2025 = UnaryType.BANG TMP_2024 
exists_1(bool) := TMP_2025(uint256)
 ! exists
TMP_2026 = UnaryType.BANG exists_1 
CONDITION TMP_2026
 tstore(uint256,uint256)(slot,1)
TMP_2027(None) = SOLIDITY_CALL tstore(uint256,uint256)(slot_1,1)
 balSlot_addBaseToken_asm_0 = slot + 2
TMP_2028(bytes32) = slot_1 + 2
balSlot_addBaseToken_asm_0_1(uint256) := TMP_2028(bytes32)
 oldVal_addBaseToken_asm_0 = tload(uint256)(balSlot_addBaseToken_asm_0)
TMP_2029(uint256) = SOLIDITY_CALL tload(uint256)(balSlot_addBaseToken_asm_0_1)
oldVal_addBaseToken_asm_0_1(uint256) := TMP_2029(uint256)
 newVal_addBaseToken_asm_0 = oldVal_addBaseToken_asm_0 + baseAmount
TMP_2030(uint256) = oldVal_addBaseToken_asm_0_1 + baseAmount_1
newVal_addBaseToken_asm_0_1(uint256) := TMP_2030(uint256)
 newVal_addBaseToken_asm_0 < oldVal_addBaseToken_asm_0
TMP_2031(bool) = newVal_addBaseToken_asm_0_1 < oldVal_addBaseToken_asm_0_1
CONDITION TMP_2031
 mstore(uint256,uint256)(0x00,err)
TMP_2032(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,err_1)
 revert(uint256,uint256)(0x00,0x04)
TMP_2033(None) = SOLIDITY_CALL revert(uint256,uint256)(0,4)
 tstore(uint256,uint256)(balSlot_addBaseToken_asm_0,newVal_addBaseToken_asm_0)
TMP_2034(None) = SOLIDITY_CALL tstore(uint256,uint256)(balSlot_addBaseToken_asm_0_1,newVal_addBaseToken_asm_0_1)
 ! exists
TMP_2035 = UnaryType.BANG exists_1 
CONDITION TMP_2035
 _addMaker(maker)
INTERNAL_CALL, TransientMakerData._addMaker(address)(maker_1)
```
#### TransientMakerData.addQuoteToken(address,uint256) [INTERNAL]
```slithir
TRANSIENT_CREDITS_POSITION_1(bytes32) := phi(['TRANSIENT_CREDITS_POSITION_0'])
 slot = keccak256(bytes)(abi.encode(TRANSIENT_CREDITS_POSITION,maker))
TMP_2005(bytes) = SOLIDITY_CALL abi.encode()(TRANSIENT_CREDITS_POSITION_1,maker_1)
TMP_2006(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_2005)
slot_1(bytes32) := TMP_2006(bytes32)
 err = revert ArithmeticOverflow().selector
REF_980(bytes4) (->None) := 3833512052(bytes4)
err_1(bytes4) := REF_980(bytes4)
 exists = ! ! tload(uint256)(slot)
TMP_2007(uint256) = SOLIDITY_CALL tload(uint256)(slot_1)
TMP_2008 = UnaryType.BANG TMP_2007 
TMP_2009 = UnaryType.BANG TMP_2008 
exists_1(bool) := TMP_2009(uint256)
 ! exists
TMP_2010 = UnaryType.BANG exists_1 
CONDITION TMP_2010
 tstore(uint256,uint256)(slot,1)
TMP_2011(None) = SOLIDITY_CALL tstore(uint256,uint256)(slot_1,1)
 balSlot_addQuoteToken_asm_0 = slot + 1
TMP_2012(bytes32) = slot_1 + 1
balSlot_addQuoteToken_asm_0_1(uint256) := TMP_2012(bytes32)
 oldVal_addQuoteToken_asm_0 = tload(uint256)(balSlot_addQuoteToken_asm_0)
TMP_2013(uint256) = SOLIDITY_CALL tload(uint256)(balSlot_addQuoteToken_asm_0_1)
oldVal_addQuoteToken_asm_0_1(uint256) := TMP_2013(uint256)
 newVal_addQuoteToken_asm_0 = oldVal_addQuoteToken_asm_0 + quoteAmount
TMP_2014(uint256) = oldVal_addQuoteToken_asm_0_1 + quoteAmount_1
newVal_addQuoteToken_asm_0_1(uint256) := TMP_2014(uint256)
 newVal_addQuoteToken_asm_0 < oldVal_addQuoteToken_asm_0
TMP_2015(bool) = newVal_addQuoteToken_asm_0_1 < oldVal_addQuoteToken_asm_0_1
CONDITION TMP_2015
 mstore(uint256,uint256)(0x00,err)
TMP_2016(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,err_1)
 revert(uint256,uint256)(0x00,0x04)
TMP_2017(None) = SOLIDITY_CALL revert(uint256,uint256)(0,4)
 tstore(uint256,uint256)(balSlot_addQuoteToken_asm_0,newVal_addQuoteToken_asm_0)
TMP_2018(None) = SOLIDITY_CALL tstore(uint256,uint256)(balSlot_addQuoteToken_asm_0_1,newVal_addQuoteToken_asm_0_1)
 ! exists
TMP_2019 = UnaryType.BANG exists_1 
CONDITION TMP_2019
 _addMaker(maker)
INTERNAL_CALL, TransientMakerData._addMaker(address)(maker_1)
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
#### BookLib.addOrderToBook(Book,Order) [INTERNAL]
```slithir
 order.reduceOnly
REF_3821(bool) -> order_1.reduceOnly
CONDITION REF_3821
 StorageLib.loadMarket(self.config.asset).linkReduceOnlyOrder(order.owner,order.subaccount,order.id.unwrap(),self.config.bookType)
REF_3823(BookConfig) -> self_1 (-> []).config
REF_3824(bytes32) -> REF_3823.asset
TMP_8599(Market) = LIBRARY_CALL, dest:StorageLib, function:StorageLib.loadMarket(bytes32), arguments:['REF_3824'] 
REF_3826(address) -> order_1.owner
REF_3827(uint256) -> order_1.subaccount
REF_3828(OrderId) -> order_1.id
TMP_8600(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_3828'] 
REF_3830(BookConfig) -> self_1 (-> []).config
REF_3831(BookType) -> REF_3830.bookType
LIBRARY_CALL, dest:MarketLib, function:MarketLib.linkReduceOnlyOrder(Market,address,uint256,uint256,BookType), arguments:['TMP_8599', 'REF_3826', 'REF_3827', 'TMP_8600', 'REF_3831'] 
 limit = _updateBookPostOrder(self,order)
TMP_8602(Limit) = INTERNAL_CALL, BookLib._updateBookPostOrder(Book,Order)(self_1 (-> []),order_1)
limit_1 (-> ['TMP_8602'])(Limit) := TMP_8602(Limit)
 _updateLimitPostOrder(self,limit,order)
INTERNAL_CALL, BookLib._updateLimitPostOrder(Book,Limit,Order)(self_1 (-> []),limit_1 (-> ['TMP_8602']),order_1)
 self.orders[order.id] = order
REF_3832(mapping(OrderId => Order)) -> self_1 (-> []).orders
REF_3833(OrderId) -> order_1.id
REF_3834(Order) -> REF_3832[REF_3833]
self_2 (-> [])(Book) := phi(['self_1 (-> [])'])
REF_3834(Order) (->self_2 (-> [])) := order_1(Order)
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
#### CLOBStorageLib.getBestBidPrice(Book) [INTERNAL]
```slithir
 self.bidTree.maximum()
REF_828(RedBlackTree) -> self_1 (-> []).bidTree
TMP_1800(uint256) = LIBRARY_CALL, dest:BookRedBlackTreeLib, function:BookRedBlackTreeLib.maximum(RedBlackTree), arguments:['REF_828'] 
RETURN TMP_1800
```
#### CLOBStorageLib.getWorstAskPrice(Book) [INTERNAL]
```slithir
 self.askTree.maximum()
REF_834(RedBlackTree) -> self_1 (-> []).askTree
TMP_1803(uint256) = LIBRARY_CALL, dest:BookRedBlackTreeLib, function:BookRedBlackTreeLib.maximum(RedBlackTree), arguments:['REF_834'] 
RETURN TMP_1803
```
#### CLOBStorageLib.getBestAskPrice(Book) [INTERNAL]
```slithir
 self.askTree.minimum()
REF_830(RedBlackTree) -> self_1 (-> []).askTree
TMP_1801(uint256) = LIBRARY_CALL, dest:BookRedBlackTreeLib, function:BookRedBlackTreeLib.minimum(RedBlackTree), arguments:['REF_830'] 
RETURN TMP_1801
```
#### CLOBStorageLib.getWorstBidPrice(Book) [INTERNAL]
```slithir
 self.bidTree.minimum()
REF_832(RedBlackTree) -> self_1 (-> []).bidTree
TMP_1802(uint256) = LIBRARY_CALL, dest:BookRedBlackTreeLib, function:BookRedBlackTreeLib.minimum(RedBlackTree), arguments:['REF_832'] 
RETURN TMP_1802
```
#### EventNonceLib.inc() [INTERNAL]
```slithir
 ds = getEventNonceStorage()
TMP_9860(EventNonceStorage) = INTERNAL_CALL, EventNonceLib.getEventNonceStorage()()
ds_1 (-> ['TMP_9860'])(EventNonceStorage) := TMP_9860(EventNonceStorage)
 ++ ds.eventNonce
REF_5582(uint256) -> ds_1 (-> ['TMP_9860']).eventNonce
ds_2 (-> ['TMP_9860'])(EventNonceStorage) := phi(["ds_1 (-> ['TMP_9860'])"])
REF_5582(-> ds_2 (-> ['TMP_9860'])) = REF_5582 (c)+ 1
RETURN REF_5582
TMP_9860(EventNonceStorage) := phi(["ds_2 (-> ['TMP_9860'])"])
```
#### OrderIdLib.toOrderId(uint256) [INTERNAL]
```slithir
 OrderId.wrap(id)
TMP_1923 = CONVERT id_1 to OrderId
RETURN TMP_1923
```
#### OrderLib.isNull(Order) [INTERNAL]
```slithir
 self.id.unwrap() == NULL_ORDER_ID
REF_5307(OrderId) -> self_1.id
TMP_9516(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_5307'] 
TMP_9517(bool) = TMP_9516 == NULL_ORDER_ID
RETURN TMP_9517
```
#### CLOBStorageLib._getCLOBStorage() [INTERNAL]
```slithir
CLOB_STORAGE_POSITION_1(bytes32) := phi(['CLOB_STORAGE_POSITION_0'])
 slot = CLOB_STORAGE_POSITION
slot_1(bytes32) := CLOB_STORAGE_POSITION_1(bytes32)
 self = slot
self_1 (-> ['slot'])(Book) := slot_1(bytes32)
 self
RETURN self_1 (-> ['slot'])
```
#### BookLib.boundToLots(Book,uint256) [INTERNAL]
```slithir
 lotSize = self.config.lotSize
REF_3758(BookConfig) -> self_1 (-> []).config
REF_3759(uint256) -> REF_3758.lotSize
lotSize_1(uint256) := REF_3759(uint256)
 baseAmount / lotSize * lotSize
TMP_8540(uint256) = baseAmount_1 (c)/ lotSize_1
TMP_8541(uint256) = TMP_8540 (c)* lotSize_1
RETURN TMP_8541
```
#### CLOBStorageLib.getBaseTokenAmount(Book,uint256,uint256) [INTERNAL]
```slithir
 quoteAmount * self.config().baseSize / price
TMP_1811(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:['self_1 (-> [])'] 
REF_852(uint256) -> TMP_1811.baseSize
TMP_1812(uint256) = quoteAmount_1 (c)* REF_852
TMP_1813(uint256) = TMP_1812 (c)/ price_1
RETURN TMP_1813
```
#### CLOBStorageLib.settings(Book) [INTERNAL]
```slithir
 _getMarketSettingsStorage()
TMP_1797(MarketSettings) = INTERNAL_CALL, CLOBStorageLib._getMarketSettingsStorage()()
RETURN TMP_1797
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
#### BookLib.assertLotSizeCompliant(Book,uint256) [INTERNAL]
```slithir
 amount % self.settings().lotSizeInBase > 0
TMP_1670(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:['self_1 (-> [])'] 
REF_655(uint256) -> TMP_1670.lotSizeInBase
TMP_1671(uint256) = amount_1 % REF_655
TMP_1672(bool) = TMP_1671 > 0
CONDITION TMP_1672
 revert LotSizeInvalid()()
TMP_1673(None) = SOLIDITY_CALL revert LotSizeInvalid()()
```
#### OrderIdLib.unwrap(OrderId) [INTERNAL]
```slithir
 uint256(OrderId.unwrap(id))
TMP_1924 = CONVERT id_1 to uint256
TMP_1925 = CONVERT TMP_1924 to uint256
RETURN TMP_1925
```
#### IAccountManager.creditAccountNoEvent(address,address,uint256) [EXTERNAL]
```slithir

```
#### CLOBStorageLib.config(Book) [INTERNAL]
```slithir
 _getMarketConfigStorage()
TMP_1798(MarketConfig) = INTERNAL_CALL, CLOBStorageLib._getMarketConfigStorage()()
RETURN TMP_1798
```
#### IAccountManager.creditAccount(address,address,uint256) [EXTERNAL]
```slithir

```

#### IAccountManager.settleIncomingOrder(ICLOB.SettleParams) [EXTERNAL]
```slithir

```
#### TransientMakerData.getMakerCreditsAndClearStorage() [INTERNAL]
```slithir
 makers = _getMakersAndClear()
TMP_2037(address[]) = INTERNAL_CALL, TransientMakerData._getMakersAndClear()()
makers_1(address[]) = ['TMP_2037(address[])']
 length = makers.length
REF_983 -> LENGTH makers_1
length_1(uint256) := REF_983(uint256)
 makerCredits = new MakerCredit[](length)
TMP_2039(MakerCredit[])  = new MakerCredit[](length_1)
makerCredits_1(MakerCredit[]) = ['TMP_2039(MakerCredit[])']
 i < length
i_1(uint256) := phi(['i_0', 'i_2'])
TMP_2040(bool) = i_1 < length_1
CONDITION TMP_2040
 (quoteAmount,baseAmount) = _getBalancesAndClear(makers[i])
REF_984(address) -> makers_1[i_1]
TUPLE_14(uint256,uint256) = INTERNAL_CALL, TransientMakerData._getBalancesAndClear(address)(REF_984)
quoteAmount_1(uint256)= UNPACK TUPLE_14 index: 0 
baseAmount_1(uint256)= UNPACK TUPLE_14 index: 1 
 makerCredits[i] = MakerCredit({maker:makers[i],quoteAmount:quoteAmount,baseAmount:baseAmount})
REF_985(MakerCredit) -> makerCredits_1[i_1]
REF_986(address) -> makers_1[i_1]
TMP_2041(MakerCredit) = new MakerCredit(REF_986,quoteAmount_1,baseAmount_1)
makerCredits_2(MakerCredit[]) := phi(['makerCredits_1'])
REF_985(MakerCredit) (->makerCredits_2) := TMP_2041(MakerCredit)
 i ++
TMP_2042(uint256) := i_1(uint256)
i_2(uint256) = i_1 (c)+ 1
 makerCredits
RETURN makerCredits_1
```
#### BookLib.assertMakeAmountInBounds(Book,uint256) [INTERNAL]
```slithir
 orderAmountInBase < self.settings().minLimitOrderAmountInBase
TMP_1674(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:['self_1 (-> [])'] 
REF_657(uint256) -> TMP_1674.minLimitOrderAmountInBase
TMP_1675(bool) = orderAmountInBase_1 < REF_657
CONDITION TMP_1675
 revert LimitOrderAmountInvalid()()
TMP_1676(None) = SOLIDITY_CALL revert LimitOrderAmountInvalid()()
 orderAmountInBase % self.settings().lotSizeInBase != 0
TMP_1677(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:['self_1 (-> [])'] 
REF_659(uint256) -> TMP_1677.lotSizeInBase
TMP_1678(uint256) = orderAmountInBase_1 % REF_659
TMP_1679(bool) = TMP_1678 != 0
CONDITION TMP_1679
 revert LotSizeInvalid()()
TMP_1680(None) = SOLIDITY_CALL revert LotSizeInvalid()()
```
#### BookLib.getBaseQuanta(Book) [INTERNAL]
```slithir
 marketSettings = self.settings()
TMP_1741(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:['self_1 (-> [])'] 
marketSettings_1 (-> ['TMP_1741'])(MarketSettings) := TMP_1741(MarketSettings)
 marketSettings.lotSizeInBase.fullMulDiv(marketSettings.tickSize,self.config().baseSize)
REF_718(uint256) -> marketSettings_1 (-> ['TMP_1741']).lotSizeInBase
REF_720(uint256) -> marketSettings_1 (-> ['TMP_1741']).tickSize
TMP_1742(MarketConfig) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.config(Book), arguments:['self_1 (-> [])'] 
REF_722(uint256) -> TMP_1742.baseSize
TMP_1743(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_718', 'REF_720', 'REF_722'] 
RETURN TMP_1743
```
#### EventNonceLib.getCurrentNonce() [INTERNAL]
```slithir
 ds = getEventNonceStorage()
TMP_9861(EventNonceStorage) = INTERNAL_CALL, EventNonceLib.getEventNonceStorage()()
ds_1 (-> ['TMP_9861'])(EventNonceStorage) := TMP_9861(EventNonceStorage)
 ds.eventNonce
REF_5583(uint256) -> ds_1 (-> ['TMP_9861']).eventNonce
RETURN REF_5583
```
#### CLOBStorageLib.getLimit(Book,uint256,Side) [INTERNAL]
```slithir
 side == Side.BUY
REF_836(Side) -> Side.BUY
TMP_1804(bool) = side_1 == REF_836
CONDITION TMP_1804
 self.bidLimits[price]
REF_837(mapping(uint256 => Limit)) -> self_1 (-> []).bidLimits
REF_838(Limit) -> REF_837[price_1]
RETURN REF_838
 self.askLimits[price]
REF_839(mapping(uint256 => Limit)) -> self_1 (-> []).askLimits
REF_840(Limit) -> REF_839[price_1]
RETURN REF_840
```
#### CLOBStorageLib.getNextBiggestPrice(Book,uint256,Side) [INTERNAL]
```slithir
 side == Side.BUY
REF_841(Side) -> Side.BUY
TMP_1805(bool) = side_1 == REF_841
CONDITION TMP_1805
 self.bidTree.getNextBiggest(price)
REF_842(RedBlackTree) -> self_1 (-> []).bidTree
TMP_1806(uint256) = LIBRARY_CALL, dest:BookRedBlackTreeLib, function:BookRedBlackTreeLib.getNextBiggest(RedBlackTree,uint256), arguments:['REF_842', 'price_1'] 
RETURN TMP_1806
 self.askTree.getNextBiggest(price)
REF_844(RedBlackTree) -> self_1 (-> []).askTree
TMP_1807(uint256) = LIBRARY_CALL, dest:BookRedBlackTreeLib, function:BookRedBlackTreeLib.getNextBiggest(RedBlackTree,uint256), arguments:['REF_844', 'price_1'] 
RETURN TMP_1807
```
#### BookLib.getNextOrders(Book,OrderId,uint256) [INTERNAL]
```slithir
 currentOrder = self.orders[startOrderId]
REF_3785(mapping(OrderId => Order)) -> self_1 (-> []).orders
REF_3786(Order) -> REF_3785[startOrderId_1]
currentOrder_1 (-> ['self'])(Order) := REF_3786(Order)
 currentOrder.assertExists()
LIBRARY_CALL, dest:OrderLib, function:OrderLib.assertExists(Order), arguments:["currentOrder_1 (-> ['self'])"] 
 count = 0
count_1(uint256) := 0(uint256)
 orders = new Order[](numOrders)
TMP_8567(Order[])  = new Order[](numOrders_1)
orders_1(Order[]) = ['TMP_8567(Order[])']
 count < numOrders && ! currentOrder.isNull()
count_2(uint256) := phi(['count_1', 'count_3'])
TMP_8568(bool) = count_2 < numOrders_1
TMP_8569(bool) = LIBRARY_CALL, dest:OrderLib, function:OrderLib.isNull(Order), arguments:["currentOrder_1 (-> ['self'])"] 
TMP_8570 = UnaryType.BANG TMP_8569 
TMP_8571(bool) = TMP_8568 && TMP_8570
CONDITION TMP_8571
 orders[count] = currentOrder
REF_3789(Order) -> orders_1[count_2]
orders_2(Order[]) := phi(['orders_1'])
REF_3789(Order) (->orders_2) := currentOrder_1 (-> ['self'])(Order)
 count ++
TMP_8572(uint256) := count_2(uint256)
count_3(uint256) = count_2 (c)+ 1
 currentOrder.nextOrderId.unwrap() != 0
REF_3790(OrderId) -> currentOrder_1 (-> ['self']).nextOrderId
TMP_8573(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_3790'] 
TMP_8574(bool) = TMP_8573 != 0
CONDITION TMP_8574
 currentOrder = self.orders[currentOrder.nextOrderId]
REF_3792(mapping(OrderId => Order)) -> self_1 (-> []).orders
REF_3793(OrderId) -> currentOrder_1 (-> ['self']).nextOrderId
REF_3794(Order) -> REF_3792[REF_3793]
currentOrder_2 (-> ['self'])(Order) := REF_3794(Order)
 nextPrice = self.getNextBiggestPrice(currentOrder.price,currentOrder.side)
REF_3796(uint256) -> currentOrder_1 (-> ['self']).price
REF_3797(Side) -> currentOrder_1 (-> ['self']).side
TMP_8575(uint256) = LIBRARY_CALL, dest:BookLib, function:BookLib.getNextBiggestPrice(Book,uint256,Side), arguments:['self_1 (-> [])', 'REF_3796', 'REF_3797'] 
nextPrice_1(uint256) := TMP_8575(uint256)
 nextPrice == 0
TMP_8576(bool) = nextPrice_1 == 0
CONDITION TMP_8576
 nextLimit = self.getLimit(nextPrice,currentOrder.side)
REF_3799(Side) -> currentOrder_1 (-> ['self']).side
TMP_8577(Limit) = LIBRARY_CALL, dest:BookLib, function:BookLib.getLimit(Book,uint256,Side), arguments:['self_1 (-> [])', 'nextPrice_1', 'REF_3799'] 
nextLimit_1 (-> ['TMP_8577'])(Limit) := TMP_8577(Limit)
 currentOrder = self.orders[nextLimit.headOrder]
REF_3800(mapping(OrderId => Order)) -> self_1 (-> []).orders
REF_3801(OrderId) -> nextLimit_1 (-> ['TMP_8577']).headOrder
REF_3802(Order) -> REF_3800[REF_3801]
currentOrder_3 (-> ['self'])(Order) := REF_3802(Order)
currentOrder_4 (-> ['self'])(Order) := phi(["currentOrder_1 (-> ['self'])", "currentOrder_2 (-> ['self'])", "currentOrder_3 (-> ['self'])"])
 orders
RETURN orders_2
```
#### CLOBStorageLib.getNextSmallestPrice(Book,uint256,Side) [INTERNAL]
```slithir
 side == Side.BUY
REF_846(Side) -> Side.BUY
TMP_1808(bool) = side_1 == REF_846
CONDITION TMP_1808
 self.bidTree.getNextSmallest(price)
REF_847(RedBlackTree) -> self_1 (-> []).bidTree
TMP_1809(uint256) = LIBRARY_CALL, dest:BookRedBlackTreeLib, function:BookRedBlackTreeLib.getNextSmallest(RedBlackTree,uint256), arguments:['REF_847', 'price_1'] 
RETURN TMP_1809
 self.askTree.getNextSmallest(price)
REF_849(RedBlackTree) -> self_1 (-> []).askTree
TMP_1810(uint256) = LIBRARY_CALL, dest:BookRedBlackTreeLib, function:BookRedBlackTreeLib.getNextSmallest(RedBlackTree,uint256), arguments:['REF_849', 'price_1'] 
RETURN TMP_1810
```
#### BookLib.getOrdersPaginated(Book,Order,uint256) [INTERNAL]
```slithir
 orders = new Order[](pageSize)
TMP_1727(Order[])  = new Order[](pageSize_1)
orders_1(Order[]) = ['TMP_1727(Order[])']
 nextOrder = startOrder
nextOrder_1(Order) := startOrder_1(Order)
 counter < pageSize
orders_2(Order[]) := phi(['orders_3', 'orders_1'])
counter_1(uint256) := phi(['counter_0', 'counter_2'])
TMP_1728(bool) = counter_1 < pageSize_1
CONDITION TMP_1728
 nextOrder.id.unwrap() == 0
REF_691(OrderId) -> nextOrder_1.id
TMP_1729(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_691'] 
TMP_1730(bool) = TMP_1729 == 0
CONDITION TMP_1730
 orders[counter] = nextOrder
REF_693(Order) -> orders_2[counter_1]
orders_3(Order[]) := phi(['orders_2'])
REF_693(Order) (->orders_3) := nextOrder_1(Order)
 nextOrder.nextOrderId.unwrap() == 0
REF_694(OrderId) -> nextOrder_1.nextOrderId
TMP_1731(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_694'] 
TMP_1732(bool) = TMP_1731 == 0
CONDITION TMP_1732
 nextOrder = ds.orders[nextOrder.nextOrderId]
REF_696(mapping(OrderId => Order)) -> ds_1 (-> []).orders
REF_697(OrderId) -> nextOrder_1.nextOrderId
REF_698(Order) -> REF_696[REF_697]
nextOrder_2(Order) := REF_698(Order)
nextOrder_6(Order) := phi(['nextOrder_2', 'nextOrder_1'])
 counter ++
TMP_1733(uint256) := counter_1(uint256)
counter_2(uint256) = counter_1 (c)+ 1
 result = orders
result_1(Order[]) := orders_2(Order[])
 mstore(uint256,uint256)(counter < mload(uint256)(result) * result,counter)
TMP_1734(uint256) = SOLIDITY_CALL mload(uint256)(result_1)
TMP_1735(bool) = counter_1 < TMP_1734
TMP_1736(bool) = TMP_1735 * result_1
TMP_1737(None) = SOLIDITY_CALL mstore(uint256,uint256)(TMP_1736,counter_1)
 (result,nextOrder)
RETURN result_1,nextOrder_1
 nextOrder.side == Side.BUY
REF_699(Side) -> nextOrder_1.side
REF_700(Side) -> Side.BUY
TMP_1738(bool) = REF_699 == REF_700
CONDITION TMP_1738
 nextOrder = ds.orders[ds.bidLimits[ds.getNextSmallestPrice(nextOrder.price,Side.BUY)].headOrder]
REF_701(mapping(OrderId => Order)) -> ds_1 (-> []).orders
REF_702(mapping(uint256 => Limit)) -> ds_1 (-> []).bidLimits
REF_704(uint256) -> nextOrder_1.price
REF_705(Side) -> Side.BUY
TMP_1739(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getNextSmallestPrice(Book,uint256,Side), arguments:['ds_1 (-> [])', 'REF_704', 'REF_705'] 
REF_706(Limit) -> REF_702[TMP_1739]
REF_707(OrderId) -> REF_706.headOrder
REF_708(Order) -> REF_701[REF_707]
nextOrder_3(Order) := REF_708(Order)
 nextOrder = ds.orders[ds.askLimits[ds.getNextBiggestPrice(nextOrder.price,Side.SELL)].headOrder]
REF_709(mapping(OrderId => Order)) -> ds_1 (-> []).orders
REF_710(mapping(uint256 => Limit)) -> ds_1 (-> []).askLimits
REF_712(uint256) -> nextOrder_1.price
REF_713(Side) -> Side.SELL
TMP_1740(uint256) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.getNextBiggestPrice(Book,uint256,Side), arguments:['ds_1 (-> [])', 'REF_712', 'REF_713'] 
REF_714(Limit) -> REF_710[TMP_1740]
REF_715(OrderId) -> REF_714.headOrder
REF_716(Order) -> REF_709[REF_715]
nextOrder_4(Order) := REF_716(Order)
nextOrder_5(Order) := phi(['nextOrder_3', 'nextOrder_4'])
 (result,nextOrder)
```
#### BookLib.assertUnusedOrderId(Book,uint256) [INTERNAL]
```slithir
 self.orders[orderId.wrap()].owner != address(0)
REF_3717(mapping(OrderId => Order)) -> self_1 (-> []).orders
TMP_8516(OrderId) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.wrap(uint256), arguments:['orderId_1'] 
REF_3719(Order) -> REF_3717[TMP_8516]
REF_3720(address) -> REF_3719.owner
TMP_8517 = CONVERT 0 to address
TMP_8518(bool) = REF_3720 != TMP_8517
CONDITION TMP_8518
 revert OrderIdInUse()()
TMP_8519(None) = SOLIDITY_CALL revert OrderIdInUse()()
```
#### BookLib.incrementOrderId(Book) [INTERNAL]
```slithir
 ++ self.metadata.orderIdCounter
REF_3806(BookMetadata) -> self_1 (-> []).metadata
REF_3807(uint96) -> REF_3806.orderIdCounter
self_2 (-> [])(Book) := phi(['self_1 (-> [])'])
REF_3807(-> self_2 (-> [])) = REF_3807 (c)+ 1
RETURN REF_3807
```
#### OrderIdLib.getClientOrderId(address,uint96) [INTERNAL]
```slithir
 uint256(bytes32(abi.encodePacked(account,id)))
TMP_1920(bytes) = SOLIDITY_CALL abi.encodePacked()(account_1,id_1)
TMP_1921 = CONVERT TMP_1920 to bytes32
TMP_1922 = CONVERT TMP_1921 to uint256
RETURN TMP_1922
```
#### OrderLib.toOrderChecked(ICLOB.PlaceOrderArgs,uint256,address) [INTERNAL]
```slithir
 args.limitPrice == 0 && uint8(args.tif) < 2
REF_926(uint256) -> args_1.limitPrice
TMP_1928(bool) = REF_926 == 0
REF_927(ICLOB.TiF) -> args_1.tif
TMP_1929 = CONVERT REF_927 to uint8
TMP_1930(bool) = TMP_1929 < 2
TMP_1931(bool) = TMP_1928 && TMP_1930
CONDITION TMP_1931
 revert MarketOrderCannotMake()()
TMP_1932(None) = SOLIDITY_CALL revert MarketOrderCannotMake()()
 uint8(args.tif) <= 1 && args.expiryTime > 0 && args.expiryTime < block.timestamp
REF_928(ICLOB.TiF) -> args_1.tif
TMP_1933 = CONVERT REF_928 to uint8
TMP_1934(bool) = TMP_1933 <= 1
REF_929(uint32) -> args_1.expiryTime
TMP_1935(bool) = REF_929 > 0
TMP_1936(bool) = TMP_1934 && TMP_1935
REF_930(uint32) -> args_1.expiryTime
TMP_1937(bool) = REF_930 < block.timestamp
TMP_1938(bool) = TMP_1936 && TMP_1937
CONDITION TMP_1938
 revert MakerOrderExpired()()
TMP_1939(None) = SOLIDITY_CALL revert MakerOrderExpired()()
 args.expiryTime > 0 && uint8(args.tif) > 1
REF_931(uint32) -> args_1.expiryTime
TMP_1940(bool) = REF_931 > 0
REF_932(ICLOB.TiF) -> args_1.tif
TMP_1941 = CONVERT REF_932 to uint8
TMP_1942(bool) = TMP_1941 > 1
TMP_1943(bool) = TMP_1940 && TMP_1942
CONDITION TMP_1943
 revert TakerOrdersCannotExpire()()
TMP_1944(None) = SOLIDITY_CALL revert TakerOrdersCannotExpire()()
 args.tif == ICLOB.TiF.MOC && ! args.baseDenominated
REF_933(ICLOB.TiF) -> args_1.tif
REF_934(ICLOB.TiF) -> TiF.MOC
TMP_1945(bool) = REF_933 == REF_934
REF_935(bool) -> args_1.baseDenominated
TMP_1946 = UnaryType.BANG REF_935 
TMP_1947(bool) = TMP_1945 && TMP_1946
CONDITION TMP_1947
 revert PostOnlyOrderMustBeBaseDenominated()()
TMP_1948(None) = SOLIDITY_CALL revert PostOnlyOrderMustBeBaseDenominated()()
 args.limitPrice > 0
REF_936(uint256) -> args_1.limitPrice
TMP_1949(bool) = REF_936 > 0
CONDITION TMP_1949
 order.price = args.limitPrice
REF_937(uint256) -> order_0.price
REF_938(uint256) -> args_1.limitPrice
order_1(Order) := phi(['order_0'])
REF_937(uint256) (->order_1) := REF_938(uint256)
order_5(Order) := phi(['order_1', 'order_0'])
 order.id = orderId.toOrderId()
REF_939(OrderId) -> order_5.id
TMP_1950(OrderId) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.toOrderId(uint256), arguments:['orderId_1'] 
order_6(Order) := phi(['order_5'])
REF_939(OrderId) (->order_6) := TMP_1950(OrderId)
 order.side = args.side
REF_941(Side) -> order_6.side
REF_942(Side) -> args_1.side
order_7(Order) := phi(['order_6'])
REF_941(Side) (->order_7) := REF_942(Side)
 order.owner = owner
REF_943(address) -> order_7.owner
order_8(Order) := phi(['order_7'])
REF_943(address) (->order_8) := owner_1(address)
 order.amount = args.amount
REF_944(uint256) -> order_8.amount
REF_945(uint256) -> args_1.amount
order_9(Order) := phi(['order_8'])
REF_944(uint256) (->order_9) := REF_945(uint256)
 order.cancelTimestamp = args.expiryTime
REF_946(uint32) -> order_9.cancelTimestamp
REF_947(uint32) -> args_1.expiryTime
order_10(Order) := phi(['order_9'])
REF_946(uint32) (->order_10) := REF_947(uint32)
 args.side == Side.BUY
REF_948(Side) -> args_1.side
REF_949(Side) -> Side.BUY
TMP_1951(bool) = REF_948 == REF_949
CONDITION TMP_1951
 order.price = type()(uint256).max
REF_950(uint256) -> order_0.price
TMP_1953(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
order_2(Order) := phi(['order_0'])
REF_950(uint256) (->order_2) := TMP_1953(uint256)
 order.price = 0
REF_951(uint256) -> order_0.price
order_3(Order) := phi(['order_0'])
REF_951(uint256) (->order_3) := 0(uint256)
order_4(Order) := phi(['order_2', 'order_3'])
 order
RETURN order_10
```
#### CLOBStorageLib.setLotSizeInBase(Book,uint256) [INTERNAL]
```slithir
 self.settings().lotSizeInBase = newLotSizeInBase
TMP_1834(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:['self_1 (-> [])'] 
REF_868(uint256) -> TMP_1834.lotSizeInBase
REF_868(uint256) (->TMP_1834) := newLotSizeInBase_1(uint256)
 self.settings().minLimitOrderAmountInBase < newLotSizeInBase
TMP_1835(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:['self_1 (-> [])'] 
REF_870(uint256) -> TMP_1835.minLimitOrderAmountInBase
TMP_1836(bool) = REF_870 < newLotSizeInBase_1
CONDITION TMP_1836
 revert NewLotSizeInvalid()()
TMP_1837(None) = SOLIDITY_CALL revert NewLotSizeInvalid()()
 self.getBaseQuanta() == 0
TMP_1838(uint256) = LIBRARY_CALL, dest:BookLib, function:BookLib.getBaseQuanta(Book), arguments:['self_1 (-> [])'] 
TMP_1839(bool) = TMP_1838 == 0
CONDITION TMP_1839
 revert NewLotSizeInvalid()()
TMP_1840(None) = SOLIDITY_CALL revert NewLotSizeInvalid()()
 LotSizeInBaseUpdated(EventNonceLib.inc(),newLotSizeInBase)
TMP_1841(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit LotSizeInBaseUpdated(TMP_1841,newLotSizeInBase_1)
```
#### CLOBStorageLib.setMaxLimitsPerTx(Book,uint8) [INTERNAL]
```slithir
 newMaxLimits == 0
TMP_1817(bool) = newMaxLimits_1 == 0
CONDITION TMP_1817
 revert NewMaxLimitsPerTxInvalid()()
TMP_1818(None) = SOLIDITY_CALL revert NewMaxLimitsPerTxInvalid()()
 self.settings().maxLimitsPerTx = newMaxLimits
TMP_1819(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:['self_1 (-> [])'] 
REF_856(uint8) -> TMP_1819.maxLimitsPerTx
REF_856(uint8) (->TMP_1819) := newMaxLimits_1(uint8)
 MaxLimitOrdersPerTxUpdated(EventNonceLib.inc(),newMaxLimits)
TMP_1820(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit MaxLimitOrdersPerTxUpdated(TMP_1820,newMaxLimits_1)
```
#### CLOBStorageLib.setMinLimitOrderAmountInBase(Book,uint256) [INTERNAL]
```slithir
 newMinLimitOrderAmountInBase < self.settings().lotSizeInBase
TMP_1828(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:['self_1 (-> [])'] 
REF_863(uint256) -> TMP_1828.lotSizeInBase
TMP_1829(bool) = newMinLimitOrderAmountInBase_1 < REF_863
CONDITION TMP_1829
 revert NewMinLimitOrderAmountInvalid()()
TMP_1830(None) = SOLIDITY_CALL revert NewMinLimitOrderAmountInvalid()()
 self.settings().minLimitOrderAmountInBase = newMinLimitOrderAmountInBase
TMP_1831(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:['self_1 (-> [])'] 
REF_865(uint256) -> TMP_1831.minLimitOrderAmountInBase
REF_865(uint256) (->TMP_1831) := newMinLimitOrderAmountInBase_1(uint256)
 MinLimitOrderAmountInBaseUpdated(EventNonceLib.inc(),newMinLimitOrderAmountInBase)
TMP_1832(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit MinLimitOrderAmountInBaseUpdated(TMP_1832,newMinLimitOrderAmountInBase_1)
```
#### CLOBStorageLib.setTickSize(Book,uint256) [INTERNAL]
```slithir
 self.settings().tickSize = newTickSize
TMP_1822(MarketSettings) = LIBRARY_CALL, dest:CLOBStorageLib, function:CLOBStorageLib.settings(Book), arguments:['self_1 (-> [])'] 
REF_859(uint256) -> TMP_1822.tickSize
REF_859(uint256) (->TMP_1822) := newTickSize_1(uint256)
 self.getBaseQuanta() == 0
TMP_1823(uint256) = LIBRARY_CALL, dest:BookLib, function:BookLib.getBaseQuanta(Book), arguments:['self_1 (-> [])'] 
TMP_1824(bool) = TMP_1823 == 0
CONDITION TMP_1824
 revert NewTickSizeInvalid()()
TMP_1825(None) = SOLIDITY_CALL revert NewTickSizeInvalid()()
 TickSizeUpdated(EventNonceLib.inc(),newTickSize)
TMP_1826(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit TickSizeUpdated(TMP_1826,newTickSize_1)
```
#### CLOBStorageLib._getMarketMetadataStorage() [INTERNAL]
```slithir
MARKET_METADATA_STORAGE_POSITION_1(bytes32) := phi(['MARKET_METADATA_STORAGE_POSITION_0'])
 slot = MARKET_METADATA_STORAGE_POSITION
slot_1(bytes32) := MARKET_METADATA_STORAGE_POSITION_1(bytes32)
 self = slot
self_1 (-> ['slot'])(MarketMetadata) := slot_1(bytes32)
 self
RETURN self_1 (-> ['slot'])
```
#### TransientMakerData._addMaker(address) [INTERNAL]
```slithir
maker_1(address) := phi(['maker_1', 'maker_1'])
TRANSIENT_MAKERS_POSITION_1(bytes32) := phi(['TRANSIENT_MAKERS_POSITION_0'])
 slot = TRANSIENT_MAKERS_POSITION
slot_1(bytes32) := TRANSIENT_MAKERS_POSITION_1(bytes32)
 len__addMaker_asm_0 = tload(uint256)(slot)
TMP_2043(uint256) = SOLIDITY_CALL tload(uint256)(slot_1)
len__addMaker_asm_0_1(uint256) := TMP_2043(uint256)
 mstore(uint256,uint256)(0x00,slot)
TMP_2044(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,slot_1)
 dataSlot__addMaker_asm_0 = keccak256(uint256,uint256)(0x00,0x20)
TMP_2045(uint256) = SOLIDITY_CALL keccak256(uint256,uint256)(0,32)
dataSlot__addMaker_asm_0_1(uint256) := TMP_2045(uint256)
 tstore(uint256,uint256)(dataSlot__addMaker_asm_0 + len__addMaker_asm_0,maker)
TMP_2046(uint256) = dataSlot__addMaker_asm_0_1 + len__addMaker_asm_0_1
TMP_2047(None) = SOLIDITY_CALL tstore(uint256,uint256)(TMP_2046,maker_1)
 tstore(uint256,uint256)(slot,len__addMaker_asm_0 + 1)
TMP_2048(uint256) = len__addMaker_asm_0_1 + 1
TMP_2049(None) = SOLIDITY_CALL tstore(uint256,uint256)(slot_1,TMP_2048)
```

#### BookLib._updateBookRemoveOrder(Book,Order) [PRIVATE]
```slithir
self_1 (-> [])(Book) := phi(['self_1 (-> [])'])
order_1(Order) := phi(['order_1'])
 order.side == Side.BUY
REF_3890(Side) -> order_1.side
REF_3891(Side) -> Side.BUY
TMP_8620(bool) = REF_3890 == REF_3891
CONDITION TMP_8620
 self.metadata.numBids --
REF_3892(BookMetadata) -> self_1 (-> []).metadata
REF_3893(uint256) -> REF_3892.numBids
TMP_8621(uint256) := REF_3893(uint256)
self_2 (-> [])(Book) := phi(['self_1 (-> [])'])
REF_3893(-> self_2 (-> [])) = REF_3893 (c)- 1
 self.metadata.quoteOI -= order.amount.fullMulDiv(order.price,1e18)
REF_3894(BookMetadata) -> self_2 (-> []).metadata
REF_3895(uint256) -> REF_3894.quoteOI
REF_3896(uint256) -> order_1.amount
REF_3898(uint256) -> order_1.price
TMP_8622(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_3896', 'REF_3898', '1000000000000000000'] 
self_3 (-> [])(Book) := phi(['self_2 (-> [])'])
REF_3895(-> self_3 (-> [])) = REF_3895 (c)- TMP_8622
 self.metadata.numAsks --
REF_3899(BookMetadata) -> self_1 (-> []).metadata
REF_3900(uint256) -> REF_3899.numAsks
TMP_8623(uint256) := REF_3900(uint256)
self_4 (-> [])(Book) := phi(['self_1 (-> [])'])
REF_3900(-> self_4 (-> [])) = REF_3900 (c)- 1
 self.metadata.baseOI -= order.amount
REF_3901(BookMetadata) -> self_4 (-> []).metadata
REF_3902(uint256) -> REF_3901.baseOI
REF_3903(uint256) -> order_1.amount
self_5 (-> [])(Book) := phi(['self_4 (-> [])'])
REF_3902(-> self_5 (-> [])) = REF_3902 (c)- REF_3903
self_6 (-> [])(Book) := phi(['self_5 (-> [])', 'self_3 (-> [])'])
 delete self.orders[order.id]
REF_3904(mapping(OrderId => Order)) -> self_6 (-> []).orders
REF_3905(OrderId) -> order_1.id
REF_3906(Order) -> REF_3904[REF_3905]
REF_3904 = delete REF_3906
```
#### BookLib._updateLimitRemoveOrder(Book,Order) [PRIVATE]
```slithir
self_1 (-> [])(Book) := phi(['self_1 (-> [])'])
order_1(Order) := phi(['order_1'])
 limit.numOrders == 1
REF_3907(uint64) -> limit_3 (-> ['self']).numOrders
TMP_8624(bool) = REF_3907 == 1
CONDITION TMP_8624
 order.side == Side.BUY
REF_3908(Side) -> order_1.side
REF_3909(Side) -> Side.BUY
TMP_8625(bool) = REF_3908 == REF_3909
CONDITION TMP_8625
 delete self.bidLimits[order.price]
REF_3910(mapping(uint256 => Limit)) -> self_1 (-> []).bidLimits
REF_3911(uint256) -> order_1.price
REF_3912(Limit) -> REF_3910[REF_3911]
REF_3910 = delete REF_3912 
 self.bidTree.remove(order.price)
REF_3913(RedBlackTree) -> self_1 (-> []).bidTree
REF_3915(uint256) -> order_1.price
LIBRARY_CALL, dest:BookRedBlackTreeLib, function:BookRedBlackTreeLib.remove(RedBlackTree,uint256), arguments:['REF_3913', 'REF_3915'] 
 delete self.askLimits[order.price]
REF_3916(mapping(uint256 => Limit)) -> self_1 (-> []).askLimits
REF_3917(uint256) -> order_1.price
REF_3918(Limit) -> REF_3916[REF_3917]
REF_3916 = delete REF_3918 
 self.askTree.remove(order.price)
REF_3919(RedBlackTree) -> self_1 (-> []).askTree
REF_3921(uint256) -> order_1.price
LIBRARY_CALL, dest:BookRedBlackTreeLib, function:BookRedBlackTreeLib.remove(RedBlackTree,uint256), arguments:['REF_3919', 'REF_3921'] 
 limit.numOrders --
REF_3922(uint64) -> limit_3 (-> ['self']).numOrders
TMP_8628(uint64) := REF_3922(uint64)
limit_4 (-> ['self'])(Limit) := phi(["limit_3 (-> ['self'])"])
REF_3922(-> limit_4 (-> ['self'])) = REF_3922 (c)- 1
self_5 (-> ['self'])(Book) := phi(["limit_4 (-> ['self'])"])
 order.prevOrderId.unwrap() != 0
REF_3923(OrderId) -> order_1.prevOrderId
TMP_8629(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_3923'] 
TMP_8630(bool) = TMP_8629 != 0
CONDITION TMP_8630
 self.orders[order.prevOrderId].nextOrderId = order.nextOrderId
REF_3925(mapping(OrderId => Order)) -> self_1 (-> []).orders
REF_3926(OrderId) -> order_1.prevOrderId
REF_3927(Order) -> REF_3925[REF_3926]
REF_3928(OrderId) -> REF_3927.nextOrderId
REF_3929(OrderId) -> order_1.nextOrderId
self_2 (-> [])(Book) := phi(['self_1 (-> [])'])
REF_3928(OrderId) (->self_2 (-> [])) := REF_3929(OrderId)
 limit.headOrder = order.nextOrderId
REF_3930(OrderId) -> limit_4 (-> ['self']).headOrder
REF_3931(OrderId) -> order_1.nextOrderId
limit_5 (-> ['self'])(Limit) := phi(["limit_4 (-> ['self'])"])
REF_3930(OrderId) (->limit_5 (-> ['self'])) := REF_3931(OrderId)
self_7 (-> ['self'])(Book) := phi(["limit_5 (-> ['self'])"])
self_3 (-> [])(Book) := phi(['self_1 (-> [])', 'self_2 (-> [])'])
limit_6 (-> ['self'])(Limit) := phi(["limit_4 (-> ['self'])", "limit_5 (-> ['self'])"])
 order.nextOrderId.unwrap() != 0
REF_3932(OrderId) -> order_1.nextOrderId
TMP_8631(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_3932'] 
TMP_8632(bool) = TMP_8631 != 0
CONDITION TMP_8632
 self.orders[order.nextOrderId].prevOrderId = order.prevOrderId
REF_3934(mapping(OrderId => Order)) -> self_3 (-> []).orders
REF_3935(OrderId) -> order_1.nextOrderId
REF_3936(Order) -> REF_3934[REF_3935]
REF_3937(OrderId) -> REF_3936.prevOrderId
REF_3938(OrderId) -> order_1.prevOrderId
self_4 (-> [])(Book) := phi(['self_3 (-> [])'])
REF_3937(OrderId) (->self_4 (-> [])) := REF_3938(OrderId)
 limit.tailOrder = order.prevOrderId
REF_3939(OrderId) -> limit_6 (-> ['self']).tailOrder
REF_3940(OrderId) -> order_1.prevOrderId
limit_7 (-> ['self'])(Limit) := phi(["limit_6 (-> ['self'])"])
REF_3939(OrderId) (->limit_7 (-> ['self'])) := REF_3940(OrderId)
self_6 (-> ['self'])(Book) := phi(["limit_7 (-> ['self'])"])
 order.side == Side.BUY
REF_3941(Side) -> order_1.side
REF_3942(Side) -> Side.BUY
TMP_8633(bool) = REF_3941 == REF_3942
CONDITION TMP_8633
 limit = self.bidLimits[order.price]
REF_3943(mapping(uint256 => Limit)) -> self_1 (-> []).bidLimits
REF_3944(uint256) -> order_1.price
REF_3945(Limit) -> REF_3943[REF_3944]
limit_2 (-> ['self'])(Limit) := REF_3945(Limit)
 limit = self.askLimits[order.price]
REF_3946(mapping(uint256 => Limit)) -> self_1 (-> []).askLimits
REF_3947(uint256) -> order_1.price
REF_3948(Limit) -> REF_3946[REF_3947]
limit_1 (-> ['self'])(Limit) := REF_3948(Limit)
limit_3 (-> ['self'])(Limit) := phi(["limit_1 (-> ['self'])", "limit_2 (-> ['self'])"])
```
#### BookLib._updateBookPostOrder(Book,Order) [PRIVATE]
```slithir
self_1 (-> [])(Book) := phi(['self_1 (-> [])'])
order_1(Order) := phi(['order_1'])
 order.side == Side.BUY
REF_3846(Side) -> order_1.side
REF_3847(Side) -> Side.BUY
TMP_8609(bool) = REF_3846 == REF_3847
CONDITION TMP_8609
 limit = self.bidLimits[order.price]
REF_3848(mapping(uint256 => Limit)) -> self_1 (-> []).bidLimits
REF_3849(uint256) -> order_1.price
REF_3850(Limit) -> REF_3848[REF_3849]
limit_2 (-> ['self'])(Limit) := REF_3850(Limit)
 limit.numOrders == 0
REF_3851(uint64) -> limit_2 (-> ['self']).numOrders
TMP_8610(bool) = REF_3851 == 0
CONDITION TMP_8610
 self.bidTree.insert(order.price)
REF_3852(RedBlackTree) -> self_1 (-> []).bidTree
REF_3854(uint256) -> order_1.price
LIBRARY_CALL, dest:BookRedBlackTreeLib, function:BookRedBlackTreeLib.insert(RedBlackTree,uint256), arguments:['REF_3852', 'REF_3854'] 
 self.metadata.numBids ++
REF_3855(BookMetadata) -> self_1 (-> []).metadata
REF_3856(uint256) -> REF_3855.numBids
TMP_8612(uint256) := REF_3856(uint256)
self_4 (-> [])(Book) := phi(['self_1 (-> [])'])
REF_3856(-> self_4 (-> [])) = REF_3856 (c)+ 1
 self.metadata.quoteOI += order.amount.fullMulDiv(order.price,1e18)
REF_3857(BookMetadata) -> self_4 (-> []).metadata
REF_3858(uint256) -> REF_3857.quoteOI
REF_3859(uint256) -> order_1.amount
REF_3861(uint256) -> order_1.price
TMP_8613(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['REF_3859', 'REF_3861', '1000000000000000000'] 
self_5 (-> [])(Book) := phi(['self_4 (-> [])'])
REF_3858(-> self_5 (-> [])) = REF_3858 (c)+ TMP_8613
 limit = self.askLimits[order.price]
REF_3862(mapping(uint256 => Limit)) -> self_1 (-> []).askLimits
REF_3863(uint256) -> order_1.price
REF_3864(Limit) -> REF_3862[REF_3863]
limit_1 (-> ['self'])(Limit) := REF_3864(Limit)
 limit.numOrders == 0
REF_3865(uint64) -> limit_1 (-> ['self']).numOrders
TMP_8614(bool) = REF_3865 == 0
CONDITION TMP_8614
 self.askTree.insert(order.price)
REF_3866(RedBlackTree) -> self_1 (-> []).askTree
REF_3868(uint256) -> order_1.price
LIBRARY_CALL, dest:BookRedBlackTreeLib, function:BookRedBlackTreeLib.insert(RedBlackTree,uint256), arguments:['REF_3866', 'REF_3868'] 
 self.metadata.numAsks ++
REF_3869(BookMetadata) -> self_1 (-> []).metadata
REF_3870(uint256) -> REF_3869.numAsks
TMP_8616(uint256) := REF_3870(uint256)
self_2 (-> [])(Book) := phi(['self_1 (-> [])'])
REF_3870(-> self_2 (-> [])) = REF_3870 (c)+ 1
 self.metadata.baseOI += order.amount
REF_3871(BookMetadata) -> self_2 (-> []).metadata
REF_3872(uint256) -> REF_3871.baseOI
REF_3873(uint256) -> order_1.amount
self_3 (-> [])(Book) := phi(['self_2 (-> [])'])
REF_3872(-> self_3 (-> [])) = REF_3872 (c)+ REF_3873
limit_3 (-> ['self'])(Limit) := phi(["limit_1 (-> ['self'])", "limit_2 (-> ['self'])"])
 limit
RETURN limit_3 (-> ['self'])
```
#### BookLib._updateLimitPostOrder(Book,Limit,Order) [PRIVATE]
```slithir
self_1 (-> [])(Book) := phi(['self_1 (-> [])'])
limit_1 (-> ['TMP_8602'])(Limit) := phi(["limit_1 (-> ['TMP_8602'])"])
order_1(Order) := phi(['order_1'])
 limit.numOrders ++
REF_3874(uint64) -> limit_1 (-> ['TMP_8602']).numOrders
TMP_8617(uint64) := REF_3874(uint64)
limit_2 (-> ['TMP_8602'])(Limit) := phi(["limit_1 (-> ['TMP_8602'])"])
REF_3874(-> limit_2 (-> ['TMP_8602'])) = REF_3874 (c)+ 1
 limit.headOrder.unwrap() == 0
REF_3875(OrderId) -> limit_2 (-> ['TMP_8602']).headOrder
TMP_8618(uint256) = LIBRARY_CALL, dest:OrderIdLib, function:OrderIdLib.unwrap(OrderId), arguments:['REF_3875'] 
TMP_8619(bool) = TMP_8618 == 0
CONDITION TMP_8619
 limit.headOrder = order.id
REF_3877(OrderId) -> limit_2 (-> ['TMP_8602']).headOrder
REF_3878(OrderId) -> order_1.id
limit_3 (-> ['TMP_8602'])(Limit) := phi(["limit_2 (-> ['TMP_8602'])"])
REF_3877(OrderId) (->limit_3 (-> ['TMP_8602'])) := REF_3878(OrderId)
 limit.tailOrder = order.id
REF_3879(OrderId) -> limit_3 (-> ['TMP_8602']).tailOrder
REF_3880(OrderId) -> order_1.id
limit_4 (-> ['TMP_8602'])(Limit) := phi(["limit_3 (-> ['TMP_8602'])"])
REF_3879(OrderId) (->limit_4 (-> ['TMP_8602'])) := REF_3880(OrderId)
 tailOrder = self.orders[limit.tailOrder]
REF_3881(mapping(OrderId => Order)) -> self_1 (-> []).orders
REF_3882(OrderId) -> limit_2 (-> ['TMP_8602']).tailOrder
REF_3883(Order) -> REF_3881[REF_3882]
tailOrder_1 (-> ['self'])(Order) := REF_3883(Order)
 tailOrder.nextOrderId = order.id
REF_3884(OrderId) -> tailOrder_1 (-> ['self']).nextOrderId
REF_3885(OrderId) -> order_1.id
tailOrder_2 (-> ['self'])(Order) := phi(["tailOrder_1 (-> ['self'])"])
REF_3884(OrderId) (->tailOrder_2 (-> ['self'])) := REF_3885(OrderId)
self_2 (-> ['self'])(Book) := phi(["tailOrder_2 (-> ['self'])"])
 order.prevOrderId = tailOrder.id
REF_3886(OrderId) -> order_1.prevOrderId
REF_3887(OrderId) -> tailOrder_2 (-> ['self']).id
order_2(Order) := phi(['order_1'])
REF_3886(OrderId) (->order_2) := REF_3887(OrderId)
 limit.tailOrder = order.id
REF_3888(OrderId) -> limit_2 (-> ['TMP_8602']).tailOrder
REF_3889(OrderId) -> order_2.id
limit_5 (-> ['TMP_8602'])(Limit) := phi(["limit_2 (-> ['TMP_8602'])"])
REF_3888(OrderId) (->limit_5 (-> ['TMP_8602'])) := REF_3889(OrderId)
```
#### BookRedBlackTreeLib.maximum(RedBlackTree) [INTERNAL]
```slithir
 result = RedBlackTreeLib.last(tree.tree)
REF_963(RedBlackTreeLib.Tree) -> tree_1 (-> []).tree
TMP_1971(bytes32) = LIBRARY_CALL, dest:RedBlackTreeLib, function:RedBlackTreeLib.last(RedBlackTreeLib.Tree), arguments:['REF_963'] 
result_1(bytes32) := TMP_1971(bytes32)
 result == bytes32(0)
TMP_1972 = CONVERT 0 to bytes32
TMP_1973(bool) = result_1 == TMP_1972
CONDITION TMP_1973
 type()(uint256).min
TMP_1975(uint256) := 0(uint256)
RETURN TMP_1975
 RedBlackTreeLib.value(result)
RETURN TMP_1976
```
#### BookRedBlackTreeLib.minimum(RedBlackTree) [INTERNAL]
```slithir
 result = RedBlackTreeLib.first(tree.tree)
REF_960(RedBlackTreeLib.Tree) -> tree_1 (-> []).tree
TMP_1965(bytes32) = LIBRARY_CALL, dest:RedBlackTreeLib, function:RedBlackTreeLib.first(RedBlackTreeLib.Tree), arguments:['REF_960'] 
result_1(bytes32) := TMP_1965(bytes32)
 result == bytes32(0)
TMP_1966 = CONVERT 0 to bytes32
TMP_1967(bool) = result_1 == TMP_1966
CONDITION TMP_1967
 type()(uint256).max
TMP_1969(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
RETURN TMP_1969
 RedBlackTreeLib.value(result)
RETURN TMP_1970
```
#### EventNonceLib.getEventNonceStorage() [INTERNAL]
```slithir
EVENT_NONCE_STORAGE_POSITION_1(bytes32) := phi(['EVENT_NONCE_STORAGE_POSITION_0'])
 position = EVENT_NONCE_STORAGE_POSITION
position_1(bytes32) := EVENT_NONCE_STORAGE_POSITION_1(bytes32)
 ds = position
ds_1 (-> ['position'])(EventNonceStorage) := position_1(bytes32)
 ds
RETURN ds_1 (-> ['position'])
```
#### CLOBStorageLib._getMarketSettingsStorage() [INTERNAL]
```slithir
MARKET_SETTINGS_STORAGE_POSITION_1(bytes32) := phi(['MARKET_SETTINGS_STORAGE_POSITION_0'])
 slot = MARKET_SETTINGS_STORAGE_POSITION
slot_1(bytes32) := MARKET_SETTINGS_STORAGE_POSITION_1(bytes32)
 self = slot
self_1 (-> ['slot'])(MarketSettings) := slot_1(bytes32)
 self
RETURN self_1 (-> ['slot'])
```
#### CLOBStorageLib._getMarketConfigStorage() [INTERNAL]
```slithir
MARKET_CONFIG_STORAGE_POSITION_1(bytes32) := phi(['MARKET_CONFIG_STORAGE_POSITION_0'])
 slot = MARKET_CONFIG_STORAGE_POSITION
slot_1(bytes32) := MARKET_CONFIG_STORAGE_POSITION_1(bytes32)
 self = slot
self_1 (-> ['slot'])(MarketConfig) := slot_1(bytes32)
 self
RETURN self_1 (-> ['slot'])
```
#### TransientMakerData._getBalancesAndClear(address) [INTERNAL]
```slithir
maker_1(address) := phi(['REF_984'])
TRANSIENT_CREDITS_POSITION_3(bytes32) := phi(['TRANSIENT_CREDITS_POSITION_0'])
 slot = keccak256(bytes)(abi.encode(TRANSIENT_CREDITS_POSITION,maker))
TMP_2069(bytes) = SOLIDITY_CALL abi.encode()(TRANSIENT_CREDITS_POSITION_3,maker_1)
TMP_2070(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_2069)
slot_1(bytes32) := TMP_2070(bytes32)
 quote__getBalancesAndClear_asm_0 = slot + 1
TMP_2071(bytes32) = slot_1 + 1
quote__getBalancesAndClear_asm_0_1(uint256) := TMP_2071(bytes32)
 base__getBalancesAndClear_asm_0 = slot + 2
TMP_2072(bytes32) = slot_1 + 2
base__getBalancesAndClear_asm_0_1(uint256) := TMP_2072(bytes32)
 instant__getBalancesAndClear_asm_0 = 0
instant__getBalancesAndClear_asm_0_1(uint256) := 0(uint256)
 account__getBalancesAndClear_asm_0 = 1
account__getBalancesAndClear_asm_0_1(uint256) := 1(uint256)
 quoteAmount = tload(uint256)(quote__getBalancesAndClear_asm_0)
TMP_2073(uint256) = SOLIDITY_CALL tload(uint256)(quote__getBalancesAndClear_asm_0_1)
quoteAmount_1(uint256) := TMP_2073(uint256)
 baseAmount = tload(uint256)(base__getBalancesAndClear_asm_0)
TMP_2074(uint256) = SOLIDITY_CALL tload(uint256)(base__getBalancesAndClear_asm_0_1)
baseAmount_1(uint256) := TMP_2074(uint256)
 tstore(uint256,uint256)(slot,0)
TMP_2075(None) = SOLIDITY_CALL tstore(uint256,uint256)(slot_1,0)
 tstore(uint256,uint256)(quote__getBalancesAndClear_asm_0,0)
TMP_2076(None) = SOLIDITY_CALL tstore(uint256,uint256)(quote__getBalancesAndClear_asm_0_1,0)
 tstore(uint256,uint256)(base__getBalancesAndClear_asm_0,0)
TMP_2077(None) = SOLIDITY_CALL tstore(uint256,uint256)(base__getBalancesAndClear_asm_0_1,0)
 (quoteAmount,baseAmount)
RETURN quoteAmount_1,baseAmount_1
```
#### TransientMakerData._getMakersAndClear() [INTERNAL]
```slithir
TRANSIENT_MAKERS_POSITION_2(bytes32) := phi(['TRANSIENT_MAKERS_POSITION_0'])
 slot = TRANSIENT_MAKERS_POSITION
slot_1(bytes32) := TRANSIENT_MAKERS_POSITION_2(bytes32)
 len__getMakersAndClear_asm_0 = tload(uint256)(slot)
TMP_2050(uint256) = SOLIDITY_CALL tload(uint256)(slot_1)
len__getMakersAndClear_asm_0_1(uint256) := TMP_2050(uint256)
 makers = mload(uint256)(0x40)
TMP_2051(uint256) = SOLIDITY_CALL mload(uint256)(64)
makers_1(address[]) = ['TMP_2051(uint256)']
 mstore(uint256,uint256)(makers,len__getMakersAndClear_asm_0)
TMP_2052(None) = SOLIDITY_CALL mstore(uint256,uint256)(makers_1,len__getMakersAndClear_asm_0_1)
 mstore(uint256,uint256)(0x00,slot)
TMP_2053(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,slot_1)
 dataSlot__getMakersAndClear_asm_0 = keccak256(uint256,uint256)(0x00,0x20)
TMP_2054(uint256) = SOLIDITY_CALL keccak256(uint256,uint256)(0,32)
dataSlot__getMakersAndClear_asm_0_1(uint256) := TMP_2054(uint256)
 memPointer__getMakersAndClear_asm_0 = makers + 0x20
TMP_2055(address[]) = makers_1 + 32
memPointer__getMakersAndClear_asm_0_1(uint256) := TMP_2055(address[])
 i__getMakersAndClear_asm_0 = 0
i__getMakersAndClear_asm_0_1(uint256) := 0(uint256)
 i__getMakersAndClear_asm_0 < len__getMakersAndClear_asm_0
i__getMakersAndClear_asm_0_2(uint256) := phi(['i__getMakersAndClear_asm_0_1', 'i__getMakersAndClear_asm_0_3'])
TMP_2056(bool) = i__getMakersAndClear_asm_0_2 < len__getMakersAndClear_asm_0_1
CONDITION TMP_2056
 mstore(uint256,uint256)(memPointer__getMakersAndClear_asm_0 + i__getMakersAndClear_asm_0 * 0x20,tload(uint256)(dataSlot__getMakersAndClear_asm_0 + i__getMakersAndClear_asm_0))
TMP_2057(uint256) = i__getMakersAndClear_asm_0_2 * 32
TMP_2058(uint256) = memPointer__getMakersAndClear_asm_0_1 + TMP_2057
TMP_2059(uint256) = dataSlot__getMakersAndClear_asm_0_1 + i__getMakersAndClear_asm_0_2
TMP_2060(uint256) = SOLIDITY_CALL tload(uint256)(TMP_2059)
TMP_2061(None) = SOLIDITY_CALL mstore(uint256,uint256)(TMP_2058,TMP_2060)
 tstore(uint256,uint256)(dataSlot__getMakersAndClear_asm_0 + i__getMakersAndClear_asm_0,0)
TMP_2062(uint256) = dataSlot__getMakersAndClear_asm_0_1 + i__getMakersAndClear_asm_0_2
TMP_2063(None) = SOLIDITY_CALL tstore(uint256,uint256)(TMP_2062,0)
 i__getMakersAndClear_asm_0 = i__getMakersAndClear_asm_0 + 1
TMP_2064(uint256) = i__getMakersAndClear_asm_0_2 + 1
i__getMakersAndClear_asm_0_3(uint256) := TMP_2064(uint256)
 mstore(uint256,uint256)(0x40,memPointer__getMakersAndClear_asm_0 + len__getMakersAndClear_asm_0 * 0x20)
TMP_2065(uint256) = len__getMakersAndClear_asm_0_1 * 32
TMP_2066(uint256) = memPointer__getMakersAndClear_asm_0_1 + TMP_2065
TMP_2067(None) = SOLIDITY_CALL mstore(uint256,uint256)(64,TMP_2066)
 tstore(uint256,uint256)(slot,0)
TMP_2068(None) = SOLIDITY_CALL tstore(uint256,uint256)(slot_1,0)
 makers
RETURN makers_1
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
#### BookRedBlackTreeLib.getNextBiggest(RedBlackTree,uint256) [INTERNAL]
```slithir
 nodeKey == tree.maximum()
TMP_1978(uint256) = LIBRARY_CALL, dest:BookRedBlackTreeLib, function:BookRedBlackTreeLib.maximum(RedBlackTree), arguments:['tree_1 (-> [])'] 
TMP_1979(bool) = nodeKey_1 == TMP_1978
CONDITION TMP_1979
 MAX
RETURN MAX
 nodeKey == uint256(type()(uint256).max)
TMP_1981(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_1982 = CONVERT TMP_1981 to uint256
TMP_1983(bool) = nodeKey_1 == TMP_1982
CONDITION TMP_1983
 revert NodeKeyInvalid()()
TMP_1984(None) = SOLIDITY_CALL revert NodeKeyInvalid()()
 result = RedBlackTreeLib.nearestAfter(tree.tree,nodeKey + 1)
REF_969(RedBlackTreeLib.Tree) -> tree_1 (-> []).tree
TMP_1985(uint256) = nodeKey_1 (c)+ 1
TMP_1986(bytes32) = LIBRARY_CALL, dest:RedBlackTreeLib, function:RedBlackTreeLib.nearestAfter(RedBlackTreeLib.Tree,uint256), arguments:['REF_969', 'TMP_1985'] 
result_1(bytes32) := TMP_1986(bytes32)
 RedBlackTreeLib.value(result)
RETURN TMP_1987
```

#### BookRedBlackTreeLib.getNextSmallest(RedBlackTree,uint256) [INTERNAL]
```slithir
 nodeKey == tree.minimum()
TMP_1988(uint256) = LIBRARY_CALL, dest:BookRedBlackTreeLib, function:BookRedBlackTreeLib.minimum(RedBlackTree), arguments:['tree_1 (-> [])'] 
TMP_1989(bool) = nodeKey_1 == TMP_1988
CONDITION TMP_1989
 MIN
RETURN MIN
 nodeKey == 0
TMP_1990(bool) = nodeKey_1 == 0
CONDITION TMP_1990
 revert NodeKeyInvalid()()
TMP_1991(None) = SOLIDITY_CALL revert NodeKeyInvalid()()
 result = RedBlackTreeLib.nearestBefore(tree.tree,nodeKey - 1)
REF_973(RedBlackTreeLib.Tree) -> tree_1 (-> []).tree
TMP_1992(uint256) = nodeKey_1 (c)- 1
TMP_1993(bytes32) = LIBRARY_CALL, dest:RedBlackTreeLib, function:RedBlackTreeLib.nearestBefore(RedBlackTreeLib.Tree,uint256), arguments:['REF_973', 'TMP_1992'] 
result_1(bytes32) := TMP_1993(bytes32)
 RedBlackTreeLib.value(result)
RETURN TMP_1994
```


#### BookRedBlackTreeLib.insert(RedBlackTree,uint256) [INTERNAL]
```slithir
 RedBlackTreeLib.insert(tree.tree,nodeKey)
REF_976(RedBlackTreeLib.Tree) -> tree_1 (-> []).tree
LIBRARY_CALL, dest:RedBlackTreeLib, function:RedBlackTreeLib.insert(RedBlackTreeLib.Tree,uint256), arguments:['REF_976', 'nodeKey_1']
```
#### RedBlackTreeLib.last(RedBlackTreeLib.Tree) [INTERNAL]
```slithir
_BITPOS_RIGHT_1(uint256) := phi(['_BITPOS_RIGHT_4', '_BITPOS_RIGHT_12', '_BITPOS_RIGHT_8', '_BITPOS_RIGHT_0', '_BITPOS_RIGHT_13', '_BITPOS_RIGHT_2', '_BITPOS_RIGHT_11', '_BITPOS_RIGHT_9', '_BITPOS_RIGHT_6', '_BITPOS_RIGHT_10'])
 result = _end(tree,_BITPOS_RIGHT)
TMP_14158(bytes32) = INTERNAL_CALL, RedBlackTreeLib._end(RedBlackTreeLib.Tree,uint256)(tree_1 (-> []),_BITPOS_RIGHT_1)
result_1(bytes32) := TMP_14158(bytes32)
 result
RETURN result_1
```
#### RedBlackTreeLib.first(RedBlackTreeLib.Tree) [INTERNAL]
```slithir
_BITPOS_LEFT_1(uint256) := phi(['_BITPOS_LEFT_2', '_BITPOS_LEFT_7', '_BITPOS_LEFT_0', '_BITPOS_LEFT_9', '_BITPOS_LEFT_6', '_BITPOS_LEFT_4', '_BITPOS_LEFT_8'])
 result = _end(tree,_BITPOS_LEFT)
TMP_14157(bytes32) = INTERNAL_CALL, RedBlackTreeLib._end(RedBlackTreeLib.Tree,uint256)(tree_1 (-> []),_BITPOS_LEFT_1)
result_1(bytes32) := TMP_14157(bytes32)
 result
RETURN result_1
```
#### RedBlackTreeLib.nearestAfter(RedBlackTreeLib.Tree,uint256) [INTERNAL]
```slithir
 (nodes,cursor,key) = _find(tree,x)
TUPLE_121(uint256,uint256,uint256) = INTERNAL_CALL, RedBlackTreeLib._find(RedBlackTreeLib.Tree,uint256)(tree_1 (-> []),x_1)
nodes_1(uint256)= UNPACK TUPLE_121 index: 0 
cursor_1(uint256)= UNPACK TUPLE_121 index: 1 
key_1(uint256)= UNPACK TUPLE_121 index: 2 
 cursor == uint256(0)
TMP_14128 = CONVERT 0 to uint256
TMP_14129(bool) = cursor_1 == TMP_14128
CONDITION TMP_14129
 result
RETURN result_0
 key != uint256(0)
TMP_14130 = CONVERT 0 to uint256
TMP_14131(bool) = key_1 != TMP_14130
CONDITION TMP_14131
 _pack(nodes,key)
TMP_14132(bytes32) = INTERNAL_CALL, RedBlackTreeLib._pack(uint256,uint256)(nodes_1,key_1)
RETURN TMP_14132
 a = _pack(nodes,cursor)
TMP_14133(bytes32) = INTERNAL_CALL, RedBlackTreeLib._pack(uint256,uint256)(nodes_1,cursor_1)
a_1(bytes32) := TMP_14133(bytes32)
 value(a) > x
TMP_14134(uint256) = INTERNAL_CALL, RedBlackTreeLib.value(bytes32)(a_1)
TMP_14135(bool) = TMP_14134 > x_1
CONDITION TMP_14135
 a
RETURN a_1
 next(a)
TMP_14136(bytes32) = INTERNAL_CALL, RedBlackTreeLib.next(bytes32)(a_1)
RETURN TMP_14136
 result
```
#### RedBlackTreeLib.nearestBefore(RedBlackTreeLib.Tree,uint256) [INTERNAL]
```slithir
 (nodes,cursor,key) = _find(tree,x)
TUPLE_120(uint256,uint256,uint256) = INTERNAL_CALL, RedBlackTreeLib._find(RedBlackTreeLib.Tree,uint256)(tree_1 (-> []),x_1)
nodes_1(uint256)= UNPACK TUPLE_120 index: 0 
cursor_1(uint256)= UNPACK TUPLE_120 index: 1 
key_1(uint256)= UNPACK TUPLE_120 index: 2 
 cursor == uint256(0)
TMP_14119 = CONVERT 0 to uint256
TMP_14120(bool) = cursor_1 == TMP_14119
CONDITION TMP_14120
 result
RETURN result_0
 key != uint256(0)
TMP_14121 = CONVERT 0 to uint256
TMP_14122(bool) = key_1 != TMP_14121
CONDITION TMP_14122
 _pack(nodes,key)
TMP_14123(bytes32) = INTERNAL_CALL, RedBlackTreeLib._pack(uint256,uint256)(nodes_1,key_1)
RETURN TMP_14123
 a = _pack(nodes,cursor)
TMP_14124(bytes32) = INTERNAL_CALL, RedBlackTreeLib._pack(uint256,uint256)(nodes_1,cursor_1)
a_1(bytes32) := TMP_14124(bytes32)
 value(a) < x
TMP_14125(uint256) = INTERNAL_CALL, RedBlackTreeLib.value(bytes32)(a_1)
TMP_14126(bool) = TMP_14125 < x_1
CONDITION TMP_14126
 a
RETURN a_1
 prev(a)
TMP_14127(bytes32) = INTERNAL_CALL, RedBlackTreeLib.prev(bytes32)(a_1)
RETURN TMP_14127
 result
```
