








#### GTERouter._accountDepositInternal(address,uint256) [INTERNAL]
```slithir
token_1(address) := phi(['tokenOut_1'])
amount_1(uint256) := phi(['amountOut_1'])
acctManager_16(IAccountManager) := phi(['acctManager_17', 'acctManager_0', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_13', 'acctManager_11', 'acctManager_8'])
 token.safeApprove(address(acctManager),amount)
TMP_9776 = CONVERT acctManager_16 to address
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeApprove(address,address,uint256), arguments:['token_1', 'TMP_9776', 'amount_1'] 
 acctManager.depositFromRouter(msg.sender,token,amount)
HIGH_LEVEL_CALL, dest:acctManager_16(IAccountManager), function:depositFromRouter, arguments:['msg.sender', 'token_1', 'amount_1']  
acctManager_17(IAccountManager) := phi(['acctManager_17', 'acctManager_16', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_13', 'acctManager_11', 'acctManager_8'])
```
#### GTERouter._assertValidCLOB(address) [INTERNAL]
```slithir
clob_1(address) := phi(['TMP_9816'])
clobAdminPanel_2(ICLOBManager) := phi(['clobAdminPanel_0', 'clobAdminPanel_1', 'clobAdminPanel_5', 'clobAdminPanel_3'])
 ! clobAdminPanel.isMarket(address(clob))
TMP_9731 = CONVERT clob_1 to address
TMP_9732(bool) = HIGH_LEVEL_CALL, dest:clobAdminPanel_2(ICLOBManager), function:isMarket, arguments:['TMP_9731']  
clobAdminPanel_3(ICLOBManager) := phi(['clobAdminPanel_5', 'clobAdminPanel_1', 'clobAdminPanel_2', 'clobAdminPanel_3'])
TMP_9733 = UnaryType.BANG TMP_9732 
CONDITION TMP_9733
 revert InvalidCLOBAddress()()
TMP_9734(None) = SOLIDITY_CALL revert InvalidCLOBAddress()()
```
#### GTERouter._executeAllHops(address,uint256,bytes[]) [INTERNAL]
```slithir
tokenIn_1(address) := phi(['tokenIn_1'])
amountIn_1(uint256) := phi(['amountIn_1'])
hops_1(bytes[]) := phi(['hops_1'])
 route = __RouteMetadata__({nextTokenIn:tokenIn,prevAmountOut:amountIn,prevHopType:HopType.NULL,nextHopType:hops[0].getHopType()})
REF_5483(GTERouter.HopType) -> HopType.NULL
REF_5484(bytes) -> hops_1[0]
TMP_9735(GTERouter.HopType) = LIBRARY_CALL, dest:HopLib, function:HopLib.getHopType(bytes), arguments:['REF_5484'] 
TMP_9736(GTERouter.__RouteMetadata__) = new __RouteMetadata__(tokenIn_1,amountIn_1,REF_5483,TMP_9735)
route_1(GTERouter.__RouteMetadata__) := TMP_9736(GTERouter.__RouteMetadata__)
 i = 0
i_1(uint256) := 0(uint256)
 i < hops.length
route_2(GTERouter.__RouteMetadata__) := phi(['route_8', 'route_1'])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_5486 -> LENGTH hops_1
TMP_9737(bool) = i_2 < REF_5486
CONDITION TMP_9737
 currHopType = route.nextHopType
REF_5487(GTERouter.HopType) -> route_2.nextHopType
currHopType_1(GTERouter.HopType) := REF_5487(GTERouter.HopType)
 currHopType == HopType.CLOB_FILL
REF_5488(GTERouter.HopType) -> HopType.CLOB_FILL
TMP_9738(bool) = currHopType_1 == REF_5488
CONDITION TMP_9738
 (route.prevAmountOut,route.nextTokenIn) = _executeClobPostFillOrder(route,hops[i])
REF_5489(uint256) -> route_5.prevAmountOut
REF_5490(address) -> route_5.nextTokenIn
REF_5491(bytes) -> hops_1[i_2]
TUPLE_111(uint256,address) = INTERNAL_CALL, GTERouter._executeClobPostFillOrder(GTERouter.__RouteMetadata__,bytes)(route_5,REF_5491)
REF_5489(uint256)= UNPACK TUPLE_111 index: 0 
REF_5490(address)= UNPACK TUPLE_111 index: 1 
 currHopType == HopType.UNI_V2_SWAP
REF_5492(GTERouter.HopType) -> HopType.UNI_V2_SWAP
TMP_9739(bool) = currHopType_1 == REF_5492
CONDITION TMP_9739
 (route.prevAmountOut,route.nextTokenIn) = _executeUniV2SwapExactTokensForTokens(route,hops[i])
REF_5493(uint256) -> route_5.prevAmountOut
REF_5494(address) -> route_5.nextTokenIn
REF_5495(bytes) -> hops_1[i_2]
TUPLE_112(uint256,address) = INTERNAL_CALL, GTERouter._executeUniV2SwapExactTokensForTokens(GTERouter.__RouteMetadata__,bytes)(route_5,REF_5495)
REF_5493(uint256)= UNPACK TUPLE_112 index: 0 
REF_5494(address)= UNPACK TUPLE_112 index: 1 
 revert InvalidHopType()()
TMP_9740(None) = SOLIDITY_CALL revert InvalidHopType()()
route_6(GTERouter.__RouteMetadata__) := phi(['route_1', 'route_5'])
route_7(GTERouter.__RouteMetadata__) := phi(['route_5', 'route_1'])
 route.prevHopType = currHopType
REF_5496(GTERouter.HopType) -> route_7.prevHopType
route_8(GTERouter.__RouteMetadata__) := phi(['route_7'])
REF_5496(GTERouter.HopType) (->route_8) := currHopType_1(GTERouter.HopType)
 i ++
TMP_9741(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 (route.prevAmountOut,route.nextTokenIn)
REF_5497(uint256) -> route_2.prevAmountOut
REF_5498(address) -> route_2.nextTokenIn
RETURN REF_5497,REF_5498
 (i == hops.length - 1)
REF_5499 -> LENGTH hops_1
TMP_9742(uint256) = REF_5499 (c)- 1
TMP_9743(bool) = i_2 == TMP_9742
CONDITION TMP_9743
 route.nextHopType = HopType.NULL
REF_5500(GTERouter.HopType) -> route_2.nextHopType
REF_5501(GTERouter.HopType) -> HopType.NULL
route_3(GTERouter.__RouteMetadata__) := phi(['route_2'])
REF_5500(GTERouter.HopType) (->route_3) := REF_5501(GTERouter.HopType)
 route.nextHopType = hops[i + 1].getHopType()
REF_5502(GTERouter.HopType) -> route_2.nextHopType
TMP_9744(uint256) = i_2 (c)+ 1
REF_5503(bytes) -> hops_1[TMP_9744]
TMP_9745(GTERouter.HopType) = LIBRARY_CALL, dest:HopLib, function:HopLib.getHopType(bytes), arguments:['REF_5503'] 
route_4(GTERouter.__RouteMetadata__) := phi(['route_2'])
REF_5502(GTERouter.HopType) (->route_4) := TMP_9745(GTERouter.HopType)
route_5(GTERouter.__RouteMetadata__) := phi(['route_3', 'route_4'])
 (finalAmountOut,finalTokenOut)
```
#### GTERouter._executeClobPostFillOrder(GTERouter.__RouteMetadata__,bytes) [INTERNAL]
```slithir
route_1(GTERouter.__RouteMetadata__) := phi(['route_5'])
hop_1(bytes) := phi(['REF_5491'])
clobAdminPanel_4(ICLOBManager) := phi(['clobAdminPanel_0', 'clobAdminPanel_1', 'clobAdminPanel_5', 'clobAdminPanel_3'])
 tokenOut = abi.decode(hop,(ClobHopArgs)).tokenOut
TMP_9746(GTERouter.ClobHopArgs) = SOLIDITY_CALL abi.decode()(hop_1,ClobHopArgs)
REF_5506(address) -> TMP_9746.tokenOut
tokenOut_1(address) := REF_5506(address)
 market = clobAdminPanel.getMarketAddress(route.nextTokenIn,tokenOut)
REF_5508(address) -> route_1.nextTokenIn
TMP_9747(address) = HIGH_LEVEL_CALL, dest:clobAdminPanel_4(ICLOBManager), function:getMarketAddress, arguments:['REF_5508', 'tokenOut_1']  
clobAdminPanel_5(ICLOBManager) := phi(['clobAdminPanel_5', 'clobAdminPanel_1', 'clobAdminPanel_4', 'clobAdminPanel_3'])
market_1(address) := TMP_9747(address)
 market == address(0)
TMP_9748 = CONVERT 0 to address
TMP_9749(bool) = market_1 == TMP_9748
CONDITION TMP_9749
 revert CLOBDoesNotExist()()
TMP_9750(None) = SOLIDITY_CALL revert CLOBDoesNotExist()()
 fillArgs.limitPrice = 0
REF_5509(uint256) -> fillArgs_3.limitPrice
fillArgs_4(ICLOB.PlaceOrderArgs) := phi(['fillArgs_3'])
REF_5509(uint256) (->fillArgs_4) := 0(uint256)
 fillArgs.clientOrderId = 0
REF_5510(uint96) -> fillArgs_4.clientOrderId
fillArgs_5(ICLOB.PlaceOrderArgs) := phi(['fillArgs_4'])
REF_5510(uint96) (->fillArgs_5) := 0(uint256)
 fillArgs.baseDenominated = fillArgs.side == Side.SELL
REF_5511(bool) -> fillArgs_5.baseDenominated
REF_5512(Side) -> fillArgs_5.side
REF_5513(Side) -> Side.SELL
TMP_9751(bool) = REF_5512 == REF_5513
fillArgs_6(ICLOB.PlaceOrderArgs) := phi(['fillArgs_5'])
REF_5511(bool) (->fillArgs_6) := TMP_9751(bool)
 fillArgs.tif = ICLOB.TiF.FOK
REF_5514(ICLOB.TiF) -> fillArgs_6.tif
REF_5515(ICLOB.TiF) -> TiF.FOK
fillArgs_7(ICLOB.PlaceOrderArgs) := phi(['fillArgs_6'])
REF_5514(ICLOB.TiF) (->fillArgs_7) := REF_5515(ICLOB.TiF)
 fillArgs.amount = route.prevAmountOut
REF_5516(uint256) -> fillArgs_7.amount
REF_5517(uint256) -> route_1.prevAmountOut
fillArgs_8(ICLOB.PlaceOrderArgs) := phi(['fillArgs_7'])
REF_5516(uint256) (->fillArgs_8) := REF_5517(uint256)
 fillArgs.expiryTime = 0
REF_5518(uint32) -> fillArgs_8.expiryTime
fillArgs_9(ICLOB.PlaceOrderArgs) := phi(['fillArgs_8'])
REF_5518(uint32) (->fillArgs_9) := 0(uint256)
 result = ICLOB(market).placeOrder(msg.sender,fillArgs)
TMP_9752 = CONVERT market_1 to ICLOB
TMP_9753(ICLOB.PlaceOrderResult) = HIGH_LEVEL_CALL, dest:TMP_9752(ICLOB), function:placeOrder, arguments:['msg.sender', 'fillArgs_9']  
result_1(ICLOB.PlaceOrderResult) := TMP_9753(ICLOB.PlaceOrderResult)
 (amountOut,tokenOut)
RETURN amountOut_3,tokenOut_1
 ICLOB(market).getQuoteToken() == route.nextTokenIn
TMP_9754 = CONVERT market_1 to ICLOB
TMP_9755(address) = HIGH_LEVEL_CALL, dest:TMP_9754(ICLOB), function:getQuoteToken, arguments:[]  
REF_5521(address) -> route_1.nextTokenIn
TMP_9756(bool) = TMP_9755 == REF_5521
CONDITION TMP_9756
 fillArgs.side = Side.BUY
REF_5522(Side) -> fillArgs_0.side
REF_5523(Side) -> Side.BUY
fillArgs_1(ICLOB.PlaceOrderArgs) := phi(['fillArgs_0'])
REF_5522(Side) (->fillArgs_1) := REF_5523(Side)
 fillArgs.side = Side.SELL
REF_5524(Side) -> fillArgs_0.side
REF_5525(Side) -> Side.SELL
fillArgs_2(ICLOB.PlaceOrderArgs) := phi(['fillArgs_0'])
REF_5524(Side) (->fillArgs_2) := REF_5525(Side)
fillArgs_3(ICLOB.PlaceOrderArgs) := phi(['fillArgs_1', 'fillArgs_2'])
 fillArgs.side == Side.BUY
REF_5526(Side) -> fillArgs_9.side
REF_5527(Side) -> Side.BUY
TMP_9757(bool) = REF_5526 == REF_5527
CONDITION TMP_9757
 amountOut = uint256(result.baseTokenAmountTraded) - result.takerFee
REF_5528(int256) -> result_1.baseTokenAmountTraded
TMP_9758 = CONVERT REF_5528 to uint256
REF_5529(uint256) -> result_1.takerFee
TMP_9759(uint256) = TMP_9758 (c)- REF_5529
amountOut_1(uint256) := TMP_9759(uint256)
 amountOut = uint256(result.quoteTokenAmountTraded) - result.takerFee
REF_5530(int256) -> result_1.quoteTokenAmountTraded
TMP_9760 = CONVERT REF_5530 to uint256
REF_5531(uint256) -> result_1.takerFee
TMP_9761(uint256) = TMP_9760 (c)- REF_5531
amountOut_2(uint256) := TMP_9761(uint256)
amountOut_3(uint256) := phi(['amountOut_1', 'amountOut_2'])
 (amountOut,tokenOut)
```
#### GTERouter._executeUniV2SwapExactTokensForTokens(GTERouter.__RouteMetadata__,bytes) [INTERNAL]
```slithir
route_1(GTERouter.__RouteMetadata__) := phi(['route_5'])
hop_1(bytes) := phi(['REF_5495'])
acctManager_14(IAccountManager) := phi(['acctManager_17', 'acctManager_0', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_13', 'acctManager_11', 'acctManager_8'])
uniV2Router_2(IUniswapV2Router01) := phi(['uniV2Router_1', 'uniV2Router_4', 'uniV2Router_0'])
 args = abi.decode(hop,(UniV2HopArgs))
TMP_9762(GTERouter.UniV2HopArgs) = SOLIDITY_CALL abi.decode()(hop_1,UniV2HopArgs)
args_1(GTERouter.UniV2HopArgs) := TMP_9762(GTERouter.UniV2HopArgs)
 path = args.path
REF_5533(address[]) -> args_1.path
path_1(address[]) = ['REF_5533(address[])']
 path[0] != route.nextTokenIn
REF_5534(address) -> path_1[0]
REF_5535(address) -> route_1.nextTokenIn
TMP_9763(bool) = REF_5534 != REF_5535
CONDITION TMP_9763
 revert InvalidTokenRoute()()
TMP_9764(None) = SOLIDITY_CALL revert InvalidTokenRoute()()
 route.prevHopType != HopType.UNI_V2_SWAP
REF_5536(GTERouter.HopType) -> route_1.prevHopType
REF_5537(GTERouter.HopType) -> HopType.UNI_V2_SWAP
TMP_9765(bool) = REF_5536 != REF_5537
CONDITION TMP_9765
 acctManager.withdrawToRouter(msg.sender,route.nextTokenIn,route.prevAmountOut)
REF_5539(address) -> route_1.nextTokenIn
REF_5540(uint256) -> route_1.prevAmountOut
HIGH_LEVEL_CALL, dest:acctManager_14(IAccountManager), function:withdrawToRouter, arguments:['msg.sender', 'REF_5539', 'REF_5540']  
acctManager_15(IAccountManager) := phi(['acctManager_17', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_14', 'acctManager_13', 'acctManager_11', 'acctManager_8'])
uniV2Router_3(IUniswapV2Router01) := phi(['uniV2Router_1', 'uniV2Router_2', 'uniV2Router_4'])
 path[0].safeApprove(address(uniV2Router),route.prevAmountOut)
REF_5541(address) -> path_1[0]
TMP_9767 = CONVERT uniV2Router_3 to address
REF_5543(uint256) -> route_1.prevAmountOut
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeApprove(address,address,uint256), arguments:['REF_5541', 'TMP_9767', 'REF_5543'] 
 amounts = uniV2Router.swapExactTokensForTokens(route.prevAmountOut,0,path,address(this),block.timestamp)
REF_5545(uint256) -> route_1.prevAmountOut
TMP_9769 = CONVERT this to address
TMP_9770(uint256[]) = HIGH_LEVEL_CALL, dest:uniV2Router_3(IUniswapV2Router01), function:swapExactTokensForTokens, arguments:['REF_5545', '0', 'path_1', 'TMP_9769', 'block.timestamp']  
uniV2Router_4(IUniswapV2Router01) := phi(['uniV2Router_1', 'uniV2Router_3', 'uniV2Router_4'])
amounts_1(uint256[]) = ['TMP_9770(uint256[])']
 tokenOut = path[path.length - 1]
REF_5546 -> LENGTH path_1
TMP_9771(uint256) = REF_5546 (c)- 1
REF_5547(address) -> path_1[TMP_9771]
tokenOut_1(address) := REF_5547(address)
 amountOut = amounts[amounts.length - 1]
REF_5548 -> LENGTH amounts_1
TMP_9772(uint256) = REF_5548 (c)- 1
REF_5549(uint256) -> amounts_1[TMP_9772]
amountOut_1(uint256) := REF_5549(uint256)
 route.nextHopType != HopType.UNI_V2_SWAP
REF_5550(GTERouter.HopType) -> route_1.nextHopType
REF_5551(GTERouter.HopType) -> HopType.UNI_V2_SWAP
TMP_9773(bool) = REF_5550 != REF_5551
CONDITION TMP_9773
 _accountDepositInternal(tokenOut,amountOut)
INTERNAL_CALL, GTERouter._accountDepositInternal(address,uint256)(tokenOut_1,amountOut_1)
 (amounts[amounts.length - 1],tokenOut)
REF_5552 -> LENGTH amounts_1
TMP_9775(uint256) = REF_5552 (c)- 1
REF_5553(uint256) -> amounts_1[TMP_9775]
RETURN REF_5553,tokenOut_1
 (amountOut,tokenOut)
```
#### GTERouter.clobAmend(ICLOB,ICLOB.AmendArgs) [EXTERNAL]
```slithir
 clob.amend(msg.sender,args)
TUPLE_107(int256,int256) = HIGH_LEVEL_CALL, dest:clob_1(ICLOB), function:amend, arguments:['msg.sender', 'args_1']  
RETURN TUPLE_107
 isMarket(clob)
MODIFIER_CALL, GTERouter.isMarket(ICLOB)(clob_1)
 (quoteDelta,baseDelta)
```
#### GTERouter.clobCancel(ICLOB,ICLOB.CancelArgs) [EXTERNAL]
```slithir
 clob.cancel(msg.sender,args)
TUPLE_106(uint256,uint256) = HIGH_LEVEL_CALL, dest:clob_1(ICLOB), function:cancel, arguments:['msg.sender', 'args_1']  
RETURN TUPLE_106
 isMarket(clob)
MODIFIER_CALL, GTERouter.isMarket(ICLOB)(clob_1)
 (quoteRefunded,baseRefunded)
```
#### GTERouter.clobPlaceOrder(ICLOB,ICLOB.PlaceOrderArgs) [EXTERNAL]
```slithir
 clob.placeOrder(msg.sender,args)
TMP_9722(ICLOB.PlaceOrderResult) = HIGH_LEVEL_CALL, dest:clob_1(ICLOB), function:placeOrder, arguments:['msg.sender', 'args_1']  
RETURN TMP_9722
 isMarket(clob)
MODIFIER_CALL, GTERouter.isMarket(ICLOB)(clob_1)
```
#### GTERouter.constructor(address,address,address,address,address,address) [PUBLIC]
```slithir
 weth = WETH(weth_)
TMP_9695 = CONVERT weth__1 to WETH
weth_1(WETH) := TMP_9695(WETH)
 launchpad = ILaunchpad(launchpad_)
TMP_9696 = CONVERT launchpad__1 to ILaunchpad
launchpad_1(ILaunchpad) := TMP_9696(ILaunchpad)
 acctManager = IAccountManager(accountManager_)
TMP_9697 = CONVERT accountManager__1 to IAccountManager
acctManager_1(IAccountManager) := TMP_9697(IAccountManager)
 clobAdminPanel = ICLOBManager(clobManager_)
TMP_9698 = CONVERT clobManager__1 to ICLOBManager
clobAdminPanel_1(ICLOBManager) := TMP_9698(ICLOBManager)
 permit2 = IAllowanceTransfer(permit2_)
TMP_9699 = CONVERT permit2__1 to IAllowanceTransfer
permit2_1(IAllowanceTransfer) := TMP_9699(IAllowanceTransfer)
 uniV2Router = IUniswapV2Router01(uniV2Router_)
TMP_9700 = CONVERT uniV2Router__1 to IUniswapV2Router01
uniV2Router_1(IUniswapV2Router01) := TMP_9700(IUniswapV2Router01)
```
#### GTERouter.executeRoute(address,uint256,uint256,uint256,bytes[]) [EXTERNAL]
```slithir
 (finalAmountOut,finalTokenOut) = _executeAllHops(tokenIn,amountIn,hops)
TUPLE_110(uint256,address) = INTERNAL_CALL, GTERouter._executeAllHops(address,uint256,bytes[])(tokenIn_1,amountIn_1,hops_1)
finalAmountOut_1(uint256)= UNPACK TUPLE_110 index: 0 
finalTokenOut_1(address)= UNPACK TUPLE_110 index: 1 
 finalAmountOut < amountOutMin
TMP_9727(bool) = finalAmountOut_1 < amountOutMin_1
CONDITION TMP_9727
 revert SlippageToleranceExceeded()()
TMP_9728(None) = SOLIDITY_CALL revert SlippageToleranceExceeded()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardTransient.nonReentrant()()
 inTime(deadline)
MODIFIER_CALL, GTERouter.inTime(uint256)(deadline_1)
 (finalAmountOut,finalTokenOut)
RETURN finalAmountOut_1,finalTokenOut_1
```
#### GTERouter.launchpadBuy(address,uint256,address,uint256) [EXTERNAL]
```slithir
launchpad_5(ILaunchpad) := phi(['launchpad_0', 'launchpad_4', 'launchpad_1', 'launchpad_7'])
 launchpad.buy(ILaunchpad.BuyData({account:msg.sender,token:launchToken,recipient:msg.sender,amountOutBase:amountOutBase,maxAmountInQuote:worstAmountInQuote}))
TMP_9725(ILaunchpad.BuyData) = new BuyData(msg.sender,launchToken_1,msg.sender,amountOutBase_1,worstAmountInQuote_1)
TUPLE_109(uint256,uint256) = HIGH_LEVEL_CALL, dest:launchpad_6(ILaunchpad), function:buy, arguments:['TMP_9725']  
launchpad_7(ILaunchpad) := phi(['launchpad_4', 'launchpad_7', 'launchpad_1', 'launchpad_6'])
RETURN TUPLE_109
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardTransient.nonReentrant()()
 (baseBought,quoteSpent)
```
#### GTERouter.launchpadSell(address,uint256,uint256) [EXTERNAL]
```slithir
launchpad_2(ILaunchpad) := phi(['launchpad_0', 'launchpad_4', 'launchpad_1', 'launchpad_7'])
 launchpad.sell({account:msg.sender,token:launchToken,recipient:msg.sender,amountInBase:amountInBase,minAmountOutQuote:worstAmountOutQuote})
TUPLE_108(uint256,uint256) = HIGH_LEVEL_CALL, dest:launchpad_3(ILaunchpad), function:sell, arguments:['msg.sender', 'launchToken_1', 'msg.sender', 'amountInBase_1', 'worstAmountOutQuote_1']  
launchpad_4(ILaunchpad) := phi(['launchpad_4', 'launchpad_7', 'launchpad_1', 'launchpad_3'])
RETURN TUPLE_108
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardTransient.nonReentrant()()
 (baseSpent,quoteBought)
```
#### StorageLib.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 CLEARING_HOUSE_SLOT = 0x82401ef06211501256a876d252aaf61e7132ccc51e18716e2d709a0d4272e700
 INSURANCE_FUND_SLOT = 0xbfd5935e9ce192860479583c8f68d8f0281e1b205c9b51903c37fd1663caf700
 COLLATERAL_MANAGER_SLOT = 0x61b9ccef1e220863792471c905db5592dea4de72f361956c0ad957095e951f00
 FEE_MANAGER_SLOT = 0x342baed097735cb285ac1652589d9be5e07986ffa1048894c329a3e87d336000
 MARKET_SETTINGS_SLOT = 0xabab056a6b37dca48028a49dc141d38e864363077235e1ceedd891a9da3d5700
 MARKET_METADATA_SLOT = 0x924d635e09fb0ed4d506fa4757253ad18d1012b6778f35eaca050f36795c0e00
 FUNDING_RATE_ENGINE_SLOT = 0x617f70bdcfb1b30f7368b905448126d45e4211d49d45d0b890adb64417867a00
 FUNDING_RATE_SETTINGS_SLOT = 0x2d9df79ce2a04bace979c8e7822d5d58e0eba86f9b6d650a53d036070e79e300
 PERP_CLOB_SLOT = 0xa57a5c98162987d0c55c599afa286778f3124669c2f7ee0229f5fa9d51839700
 BOOK_CONFIG_SLOT = 0x9664b91c31ceff59d9f1ffab6c8af23eb35df7e5770fbcfcb63ce9d0c5f3d600
 BOOK_SETTINGS_SLOT = 0xfd97e8e280d3f806a8f248b702ebf7f7d42962451433a0b02180a11a7d773b00
 BOOK_METADATA_SLOT = 0x96ac35e14db2dbf70714b88de0d8321e86e7af2b879d88c55718ded69e62bf00
 EVENT_NONCE_SLOT = 0x00f57b92438c2add21322de9585c2e64b6631becda92262d6e63a910f44abd00
```
#### GTERouter.spotDeposit(address,uint256,bool) [EXTERNAL]
```slithir
acctManager_2(IAccountManager) := phi(['acctManager_17', 'acctManager_0', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_13', 'acctManager_11', 'acctManager_8'])
 fromRouter
CONDITION fromRouter_1
 token.safeTransferFrom(msg.sender,address(this),amount)
TMP_9701 = CONVERT this to address
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransferFrom(address,address,address,uint256), arguments:['token_1', 'msg.sender', 'TMP_9701', 'amount_1'] 
 token.safeApprove(address(acctManager),amount)
TMP_9703 = CONVERT acctManager_2 to address
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeApprove(address,address,uint256), arguments:['token_1', 'TMP_9703', 'amount_1'] 
 acctManager.depositFromRouter(msg.sender,token,amount)
HIGH_LEVEL_CALL, dest:acctManager_2(IAccountManager), function:depositFromRouter, arguments:['msg.sender', 'token_1', 'amount_1']  
acctManager_3(IAccountManager) := phi(['acctManager_17', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_13', 'acctManager_2', 'acctManager_11', 'acctManager_8'])
 acctManager.deposit(msg.sender,token,amount)
HIGH_LEVEL_CALL, dest:acctManager_2(IAccountManager), function:deposit, arguments:['msg.sender', 'token_1', 'amount_1']  
acctManager_4(IAccountManager) := phi(['acctManager_17', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_13', 'acctManager_11', 'acctManager_8'])
```
#### GTERouter.spotDepositPermit2(address,uint160,IAllowanceTransfer.PermitSingle,bytes) [EXTERNAL]
```slithir
acctManager_5(IAccountManager) := phi(['acctManager_17', 'acctManager_0', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_13', 'acctManager_11', 'acctManager_8'])
permit2_2(IAllowanceTransfer) := phi(['permit2_1', 'permit2_4', 'permit2_0'])
 permit2.permit(msg.sender,permitSingle,signature)
HIGH_LEVEL_CALL, dest:permit2_2(IAllowanceTransfer), function:permit, arguments:['msg.sender', 'permitSingle_1', 'signature_1']  
acctManager_6(IAccountManager) := phi(['acctManager_17', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_5', 'acctManager_13', 'acctManager_11', 'acctManager_8'])
permit2_3(IAllowanceTransfer) := phi(['permit2_1', 'permit2_4', 'permit2_2'])
 permit2.transferFrom(msg.sender,address(this),amount,token)
TMP_9708 = CONVERT this to address
HIGH_LEVEL_CALL, dest:permit2_3(IAllowanceTransfer), function:transferFrom, arguments:['msg.sender', 'TMP_9708', 'amount_1', 'token_1']  
acctManager_7(IAccountManager) := phi(['acctManager_17', 'acctManager_1', 'acctManager_15', 'acctManager_6', 'acctManager_4', 'acctManager_3', 'acctManager_13', 'acctManager_11', 'acctManager_8'])
permit2_4(IAllowanceTransfer) := phi(['permit2_1', 'permit2_4', 'permit2_3'])
 token.safeApprove(address(acctManager),amount)
TMP_9710 = CONVERT acctManager_7 to address
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeApprove(address,address,uint256), arguments:['token_1', 'TMP_9710', 'amount_1'] 
 acctManager.depositFromRouter(msg.sender,token,amount)
HIGH_LEVEL_CALL, dest:acctManager_7(IAccountManager), function:depositFromRouter, arguments:['msg.sender', 'token_1', 'amount_1']  
acctManager_8(IAccountManager) := phi(['acctManager_17', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_13', 'acctManager_11', 'acctManager_8', 'acctManager_7'])
```
#### GTERouter.spotWithdraw(address,uint256) [EXTERNAL]
```slithir
acctManager_12(IAccountManager) := phi(['acctManager_17', 'acctManager_0', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_13', 'acctManager_11', 'acctManager_8'])
 acctManager.withdraw(msg.sender,token,amount)
HIGH_LEVEL_CALL, dest:acctManager_12(IAccountManager), function:withdraw, arguments:['msg.sender', 'token_1', 'amount_1']  
acctManager_13(IAccountManager) := phi(['acctManager_17', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_12', 'acctManager_13', 'acctManager_11', 'acctManager_8'])
```
#### GTERouter.wrapSpotDeposit() [EXTERNAL]
```slithir
weth_2(WETH) := phi(['weth_4', 'weth_0', 'weth_1'])
acctManager_9(IAccountManager) := phi(['acctManager_17', 'acctManager_0', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_13', 'acctManager_11', 'acctManager_8'])
 weth.deposit{value: msg.value}()
HIGH_LEVEL_CALL, dest:weth_2(WETH), function:deposit, arguments:[] value:msg.value 
weth_3(WETH) := phi(['weth_4', 'weth_1', 'weth_2'])
acctManager_10(IAccountManager) := phi(['acctManager_17', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_13', 'acctManager_11', 'acctManager_8', 'acctManager_9'])
 address(weth).safeApprove(address(acctManager),msg.value)
TMP_9714 = CONVERT weth_3 to address
TMP_9715 = CONVERT acctManager_10 to address
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeApprove(address,address,uint256), arguments:['TMP_9714', 'TMP_9715', 'msg.value'] 
 acctManager.depositFromRouter(msg.sender,address(weth),msg.value)
TMP_9717 = CONVERT weth_3 to address
HIGH_LEVEL_CALL, dest:acctManager_10(IAccountManager), function:depositFromRouter, arguments:['msg.sender', 'TMP_9717', 'msg.value']  
weth_4(WETH) := phi(['weth_4', 'weth_3', 'weth_1'])
acctManager_11(IAccountManager) := phi(['acctManager_17', 'acctManager_1', 'acctManager_15', 'acctManager_4', 'acctManager_3', 'acctManager_10', 'acctManager_13', 'acctManager_11', 'acctManager_8'])
```
#### IAccountManager.depositFromRouter(address,address,uint256) [EXTERNAL]
```slithir

```
#### SafeTransferLib.safeApprove(address,address,uint256) [INTERNAL]
```slithir
 mstore(uint256,uint256)(0x14,to)
TMP_15369(None) = SOLIDITY_CALL mstore(uint256,uint256)(20,to_1)
 mstore(uint256,uint256)(0x34,amount)
TMP_15370(None) = SOLIDITY_CALL mstore(uint256,uint256)(52,amount_1)
 mstore(uint256,uint256)(0x00,0x095ea7b3000000000000000000000000)
TMP_15371(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,12454529211011416535493358632801665024)
 success_safeApprove_asm_0 = call(uint256,uint256,uint256,uint256,uint256,uint256,uint256)(gas()(),token,0,0x10,0x44,0x00,0x20)
TMP_15372(uint256) = SOLIDITY_CALL gas()()
TMP_15373(uint256) = SOLIDITY_CALL call(uint256,uint256,uint256,uint256,uint256,uint256,uint256)(TMP_15372,token_1,0,16,68,0,32)
success_safeApprove_asm_0_1(uint256) := TMP_15373(uint256)
 ! mload(uint256)(0x00) == 1 & success_safeApprove_asm_0
TMP_15374(uint256) = SOLIDITY_CALL mload(uint256)(0)
TMP_15375(bool) = TMP_15374 == 1
TMP_15376(bool) = TMP_15375 & success_safeApprove_asm_0_1
TMP_15377 = UnaryType.BANG TMP_15376 
CONDITION TMP_15377
 ! ! extcodesize(uint256)(token) | returndatasize()() < success_safeApprove_asm_0
REF_5788 -> CODESIZE token_1
TMP_15378 = UnaryType.BANG REF_5788 
TMP_15379(uint256) = SOLIDITY_CALL returndatasize()(token_1)
TMP_15380(uint256) = TMP_15378 | TMP_15379
TMP_15381(bool) = TMP_15380 < success_safeApprove_asm_0_1
TMP_15382 = UnaryType.BANG TMP_15381 
CONDITION TMP_15382
 mstore(uint256,uint256)(0x00,0x3e3f8f73)
TMP_15383(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,1044352883)
 revert(uint256,uint256)(0x1c,0x04)
TMP_15384(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 mstore(uint256,uint256)(0x34,0)
TMP_15385(None) = SOLIDITY_CALL mstore(uint256,uint256)(52,0)
```
#### ICLOBManager.isMarket(address) [EXTERNAL]
```slithir

```

#### ICLOB.getQuoteToken() [EXTERNAL]
```slithir

```
#### ICLOB.placeOrder(address,ICLOB.PlaceOrderArgs) [EXTERNAL]
```slithir

```
#### ICLOBManager.getMarketAddress(address,address) [EXTERNAL]
```slithir

```
#### IAccountManager.withdrawToRouter(address,address,uint256) [EXTERNAL]
```slithir

```
#### ICLOB.amend(address,ICLOB.AmendArgs) [EXTERNAL]
```slithir

```
#### ICLOB.cancel(address,ICLOB.CancelArgs) [EXTERNAL]
```slithir

```
#### ILaunchpad.buy(ILaunchpad.BuyData) [EXTERNAL]
```slithir

```
#### ILaunchpad.sell(address,address,address,uint256,uint256) [EXTERNAL]
```slithir

```
#### IAccountManager.deposit(address,address,uint256) [EXTERNAL]
```slithir

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
#### IAllowanceTransfer.permit(address,IAllowanceTransfer.PermitBatch,bytes) [EXTERNAL]
```slithir

```
#### IAllowanceTransfer.transferFrom(IAllowanceTransfer.AllowanceTransferDetails[]) [EXTERNAL]
```slithir

```
#### IAccountManager.withdraw(address,address,uint256) [EXTERNAL]
```slithir

```
#### WETH.deposit() [PUBLIC]
```slithir
 _mint(msg.sender,msg.value)
INTERNAL_CALL, ERC20._mint(address,uint256)(msg.sender,msg.value)
```
