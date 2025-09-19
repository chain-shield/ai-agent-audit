

### Storage layout (AgentOwnerRegistry) 

```text
manager address
whitelist mapping(address => bool)
workToMgmtAddress mapping(address => address)
mgmtToWorkAddress mapping(address => address)
agentName mapping(address => string)
agentDescription mapping(address => string)
agentIconUrl mapping(address => string)
agentTouUrl mapping(address => string)

```

### Storage layout (ERC20) 

```text
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string

```











#### CollateralReservationsFacet._currentPoolFeeAMG(Agent.State,uint64) [PRIVATE]
```slithir
_agent_1 (-> ['TMP_2327'])(Agent.State) := phi(["agent_1 (-> ['TMP_2327'])"])
_valueAMG_1(uint64) := phi(['valueAMG_1'])
 underlyingValueUBA = Conversion.convertAmgToUBA(_valueAMG)
TMP_2394(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['_valueAMG_1'] 
underlyingValueUBA_1(uint256) := TMP_2394(uint256)
 poolFeeUBA = Minting.calculateCurrentPoolFeeUBA(_agent,underlyingValueUBA)
TMP_2395(uint256) = LIBRARY_CALL, dest:Minting, function:Minting.calculateCurrentPoolFeeUBA(Agent.State,uint256), arguments:["_agent_1 (-> ['TMP_2327'])", 'underlyingValueUBA_1'] 
poolFeeUBA_1(uint256) := TMP_2395(uint256)
 Conversion.convertUBAToAmg(poolFeeUBA)
TMP_2396(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['poolFeeUBA_1'] 
RETURN TMP_2396
```
#### CollateralReservationsFacet._emitCollateralReservationEvent(Agent.State,CollateralReservation.Data,uint256) [PRIVATE]
```slithir
_agent_1 (-> ['TMP_2327'])(Agent.State) := phi(["agent_1 (-> ['TMP_2327'])"])
_cr_1(CollateralReservation.Data) := phi(['cr_13'])
_crtId_1(uint256) := phi(['crtId_1'])
 IAssetManagerEvents.CollateralReserved(_agent.vaultAddress(),_cr.minter,_crtId,Conversion.convertAmgToUBA(_cr.valueAMG),_cr.underlyingFeeUBA,_cr.firstUnderlyingBlock,_cr.lastUnderlyingBlock,_cr.lastUnderlyingTimestamp,_agent.underlyingAddressString,PaymentReference.minting(_crtId),_cr.executor,_cr.executorFeeNatGWei * Conversion.GWEI)
TMP_2389(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:["_agent_1 (-> ['TMP_2327'])"] 
REF_1312(address) -> _cr_1.minter
REF_1314(uint64) -> _cr_1.valueAMG
TMP_2390(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_1314'] 
REF_1315(uint128) -> _cr_1.underlyingFeeUBA
REF_1316(uint64) -> _cr_1.firstUnderlyingBlock
REF_1317(uint64) -> _cr_1.lastUnderlyingBlock
REF_1318(uint64) -> _cr_1.lastUnderlyingTimestamp
REF_1319(string) -> _agent_1 (-> ['TMP_2327']).underlyingAddressString
TMP_2391(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.minting(uint256), arguments:['_crtId_1'] 
REF_1321(address) -> _cr_1.executor
REF_1322(uint64) -> _cr_1.executorFeeNatGWei
REF_1323(uint256) -> Conversion.GWEI
TMP_2392(uint64) = REF_1322 (c)* REF_1323
Emit CollateralReserved(TMP_2389,REF_1312,_crtId_1,TMP_2390,REF_1315,REF_1316,REF_1317,REF_1318,REF_1319,TMP_2391,REF_1321,TMP_2392)
```
#### CollateralReservationsFacet._lastPaymentBlock() [PRIVATE]
```slithir
 state = AssetManagerState.get()
TMP_2397(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2397'])(AssetManagerState.State) := TMP_2397(AssetManagerState.State)
 settings = Globals.getSettings()
TMP_2398(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_2398'])(AssetManagerSettings.Data) := TMP_2398(AssetManagerSettings.Data)
 timeshift = block.timestamp.toUint64() - state.currentUnderlyingBlockUpdatedAt
TMP_2399(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.timestamp'] 
REF_1330(uint64) -> state_1 (-> ['TMP_2397']).currentUnderlyingBlockUpdatedAt
TMP_2400(uint64) = TMP_2399 (c)- REF_1330
timeshift_1(uint64) := TMP_2400(uint64)
 blockshift = (uint256(timeshift) * 1000 / settings.averageBlockTimeMS).toUint64()
TMP_2401 = CONVERT timeshift_1 to uint256
TMP_2402(uint256) = TMP_2401 (c)* 1000
REF_1331(uint32) -> settings_1 (-> ['TMP_2398']).averageBlockTimeMS
TMP_2403(uint256) = TMP_2402 (c)/ REF_1331
TMP_2404(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_2403'] 
blockshift_1(uint64) := TMP_2404(uint64)
 _lastUnderlyingBlock = state.currentUnderlyingBlock + blockshift + settings.underlyingBlocksForPayment
REF_1333(uint64) -> state_1 (-> ['TMP_2397']).currentUnderlyingBlock
TMP_2405(uint64) = REF_1333 (c)+ blockshift_1
REF_1334(uint64) -> settings_1 (-> ['TMP_2398']).underlyingBlocksForPayment
TMP_2406(uint64) = TMP_2405 (c)+ REF_1334
_lastUnderlyingBlock_1(uint64) := TMP_2406(uint64)
 _lastUnderlyingTimestamp = state.currentUnderlyingBlockTimestamp + timeshift + settings.underlyingSecondsForPayment
REF_1335(uint64) -> state_1 (-> ['TMP_2397']).currentUnderlyingBlockTimestamp
TMP_2407(uint64) = REF_1335 (c)+ timeshift_1
REF_1336(uint64) -> settings_1 (-> ['TMP_2398']).underlyingSecondsForPayment
TMP_2408(uint64) = TMP_2407 (c)+ REF_1336
_lastUnderlyingTimestamp_1(uint64) := TMP_2408(uint64)
 (_lastUnderlyingBlock,_lastUnderlyingTimestamp)
RETURN _lastUnderlyingBlock_1,_lastUnderlyingTimestamp_1
```

#### CollateralReservationsFacet._reserveCollateral(Agent.State,uint64) [PRIVATE]
```slithir
_agent_1 (-> ['TMP_2327'])(Agent.State) := phi(["agent_1 (-> ['TMP_2327'])"])
_reservationAMG_1(uint64) := phi(['TMP_2354'])
 state = AssetManagerState.get()
TMP_2387(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2387'])(AssetManagerState.State) := TMP_2387(AssetManagerState.State)
 Minting.checkMintingCap(_reservationAMG)
LIBRARY_CALL, dest:Minting, function:Minting.checkMintingCap(uint64), arguments:['_reservationAMG_1'] 
 _agent.reservedAMG += _reservationAMG
REF_1308(uint64) -> _agent_1 (-> ['TMP_2327']).reservedAMG
_agent_2 (-> ['TMP_2327'])(Agent.State) := phi(["_agent_1 (-> ['TMP_2327'])"])
REF_1308(-> _agent_2 (-> ['TMP_2327'])) = REF_1308 (c)+ _reservationAMG_1
 state.totalReservedCollateralAMG += _reservationAMG
REF_1309(uint64) -> state_1 (-> ['TMP_2387']).totalReservedCollateralAMG
state_2 (-> ['TMP_2387'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_2387'])"])
REF_1309(-> state_2 (-> ['TMP_2387'])) = REF_1309 (c)+ _reservationAMG_1
TMP_2387(AssetManagerState.State) := phi(["state_2 (-> ['TMP_2387'])"])
```
#### CollateralReservationsFacet.collateralReservationFee(uint256) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_2383(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2383'])(AssetManagerState.State) := TMP_2383(AssetManagerState.State)
 amgToTokenWeiPrice = Conversion.currentAmgPriceInTokenWei(state.poolCollateralIndex)
REF_1304(uint16) -> state_1 (-> ['TMP_2383']).poolCollateralIndex
TMP_2384(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.currentAmgPriceInTokenWei(uint256), arguments:['REF_1304'] 
amgToTokenWeiPrice_1(uint256) := TMP_2384(uint256)
 _reservationFee(amgToTokenWeiPrice,Conversion.convertLotsToAMG(_lots))
TMP_2385(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertLotsToAMG(uint256), arguments:['_lots_1'] 
TMP_2386(uint256) = INTERNAL_CALL, CollateralReservationsFacet._reservationFee(uint256,uint64)(amgToTokenWeiPrice_1,TMP_2385)
RETURN TMP_2386
 _reservationFeeNATWei
```
#### CollateralReservationsFacet.reserveCollateral(address,uint256,uint256,address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_2327(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2327'])(Agent.State) := TMP_2327(Agent.State)
 Agents.requireWhitelistedAgentVaultOwner(agent)
LIBRARY_CALL, dest:Agents, function:Agents.requireWhitelistedAgentVaultOwner(Agent.State), arguments:["agent_1 (-> ['TMP_2327'])"] 
 collateralData = AgentCollateral.combinedData(agent)
TMP_2329(Collateral.CombinedData) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.combinedData(Agent.State), arguments:["agent_1 (-> ['TMP_2327'])"] 
collateralData_1(Collateral.CombinedData) := TMP_2329(Collateral.CombinedData)
 state = AssetManagerState.get()
TMP_2330(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_2330'])(AssetManagerState.State) := TMP_2330(AssetManagerState.State)
 require(bool,error)(state.mintingPausedAt == 0,revert MintingPaused()())
REF_1261(uint64) -> state_1 (-> ['TMP_2330']).mintingPausedAt
TMP_2331(bool) = REF_1261 == 0
TMP_2332(None) = SOLIDITY_CALL revert MintingPaused()()
TMP_2333(None) = SOLIDITY_CALL require(bool,error)(TMP_2331,TMP_2332)
 require(bool,error)(agent.availableAgentsPos != 0 || agent.alwaysAllowedMinters.contains(msg.sender),revert AgentNotInMintQueue()())
REF_1262(uint32) -> agent_1 (-> ['TMP_2327']).availableAgentsPos
TMP_2334(bool) = REF_1262 != 0
REF_1263(EnumerableSet.AddressSet) -> agent_1 (-> ['TMP_2327']).alwaysAllowedMinters
TMP_2335(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.contains(EnumerableSet.AddressSet,address), arguments:['REF_1263', 'msg.sender'] 
TMP_2336(bool) = TMP_2334 || TMP_2335
TMP_2337(None) = SOLIDITY_CALL revert AgentNotInMintQueue()()
TMP_2338(None) = SOLIDITY_CALL require(bool,error)(TMP_2336,TMP_2337)
 require(bool,error)(_lots > 0,revert CannotMintZeroLots()())
TMP_2339(bool) = _lots_1 > 0
TMP_2340(None) = SOLIDITY_CALL revert CannotMintZeroLots()()
TMP_2341(None) = SOLIDITY_CALL require(bool,error)(TMP_2339,TMP_2340)
 require(bool,error)(agent.status == Agent.Status.NORMAL,revert InvalidAgentStatus()())
REF_1265(Agent.Status) -> agent_1 (-> ['TMP_2327']).status
REF_1266(Agent.Status) -> Status.NORMAL
TMP_2342(bool) = REF_1265 == REF_1266
TMP_2343(None) = SOLIDITY_CALL revert InvalidAgentStatus()()
TMP_2344(None) = SOLIDITY_CALL require(bool,error)(TMP_2342,TMP_2343)
 require(bool,error)(collateralData.freeCollateralLots(agent) >= _lots,revert NotEnoughFreeCollateral()())
TMP_2345(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.freeCollateralLots(Collateral.CombinedData,Agent.State), arguments:['collateralData_1', "agent_1 (-> ['TMP_2327'])"] 
TMP_2346(bool) = TMP_2345 >= _lots_1
TMP_2347(None) = SOLIDITY_CALL revert NotEnoughFreeCollateral()()
TMP_2348(None) = SOLIDITY_CALL require(bool,error)(TMP_2346,TMP_2347)
 require(bool,error)(_maxMintingFeeBIPS >= agent.feeBIPS,revert AgentsFeeTooHigh()())
REF_1268(uint16) -> agent_1 (-> ['TMP_2327']).feeBIPS
TMP_2349(bool) = _maxMintingFeeBIPS_1 >= REF_1268
TMP_2350(None) = SOLIDITY_CALL revert AgentsFeeTooHigh()()
TMP_2351(None) = SOLIDITY_CALL require(bool,error)(TMP_2349,TMP_2350)
 valueAMG = Conversion.convertLotsToAMG(_lots)
TMP_2352(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertLotsToAMG(uint256), arguments:['_lots_1'] 
valueAMG_1(uint64) := TMP_2352(uint64)
 _reserveCollateral(agent,valueAMG + _currentPoolFeeAMG(agent,valueAMG))
TMP_2353(uint64) = INTERNAL_CALL, CollateralReservationsFacet._currentPoolFeeAMG(Agent.State,uint64)(agent_1 (-> ['TMP_2327']),valueAMG_1)
TMP_2354(uint64) = valueAMG_1 (c)+ TMP_2353
INTERNAL_CALL, CollateralReservationsFacet._reserveCollateral(Agent.State,uint64)(agent_1 (-> ['TMP_2327']),TMP_2354)
 require(bool,error)(msg.value >= reservationFee,revert InappropriateFeeAmount()())
TMP_2356(bool) = msg.value >= reservationFee_3
TMP_2357(None) = SOLIDITY_CALL revert InappropriateFeeAmount()()
TMP_2358(None) = SOLIDITY_CALL require(bool,error)(TMP_2356,TMP_2357)
 state.newCrtId += PaymentReference.randomizedIdSkip()
REF_1270(uint64) -> state_1 (-> ['TMP_2330']).newCrtId
TMP_2359(uint64) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.randomizedIdSkip(), arguments:[] 
state_2 (-> ['TMP_2330'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_2330'])"])
REF_1270(-> state_2 (-> ['TMP_2330'])) = REF_1270 (c)+ TMP_2359
TMP_2330(AssetManagerState.State) := phi(["state_2 (-> ['TMP_2330'])"])
 crtId = state.newCrtId
REF_1272(uint64) -> state_2 (-> ['TMP_2330']).newCrtId
crtId_1(uint256) := REF_1272(uint64)
 cr.valueAMG = valueAMG
REF_1273(uint64) -> cr_0.valueAMG
cr_1(CollateralReservation.Data) := phi(['cr_0'])
REF_1273(uint64) (->cr_1) := valueAMG_1(uint64)
 cr.underlyingFeeUBA = Conversion.convertAmgToUBA(valueAMG).mulBips(agent.feeBIPS).toUint128()
REF_1274(uint128) -> cr_1.underlyingFeeUBA
TMP_2360(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['valueAMG_1'] 
REF_1277(uint16) -> agent_1 (-> ['TMP_2327']).feeBIPS
TMP_2361(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_2360', 'REF_1277'] 
TMP_2362(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['TMP_2361'] 
cr_2(CollateralReservation.Data) := phi(['cr_1'])
REF_1274(uint128) (->cr_2) := TMP_2362(uint128)
 cr.reservationFeeNatWei = reservationFee.toUint128()
REF_1279(uint128) -> cr_2.reservationFeeNatWei
TMP_2363(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['reservationFee_3'] 
cr_3(CollateralReservation.Data) := phi(['cr_2'])
REF_1279(uint128) (->cr_3) := TMP_2363(uint128)
 cr.poolFeeShareBIPS = agent.poolFeeShareBIPS + 1
REF_1281(uint16) -> cr_3.poolFeeShareBIPS
REF_1282(uint16) -> agent_1 (-> ['TMP_2327']).poolFeeShareBIPS
TMP_2364(uint16) = REF_1282 (c)+ 1
cr_4(CollateralReservation.Data) := phi(['cr_3'])
REF_1281(uint16) (->cr_4) := TMP_2364(uint16)
 cr.agentVault = _agentVault
REF_1283(address) -> cr_4.agentVault
cr_5(CollateralReservation.Data) := phi(['cr_4'])
REF_1283(address) (->cr_5) := _agentVault_1(address)
 cr.minter = msg.sender
REF_1284(address) -> cr_5.minter
cr_6(CollateralReservation.Data) := phi(['cr_5'])
REF_1284(address) (->cr_6) := msg.sender(address)
 _executor != address(0)
TMP_2365 = CONVERT 0 to address
TMP_2366(bool) = _executor_1 != TMP_2365
CONDITION TMP_2366
 cr.executor = _executor
REF_1285(address) -> cr_6.executor
cr_7(CollateralReservation.Data) := phi(['cr_6'])
REF_1285(address) (->cr_7) := _executor_1(address)
 cr.executorFeeNatGWei = ((msg.value - reservationFee) / Conversion.GWEI).toUint64()
REF_1286(uint64) -> cr_7.executorFeeNatGWei
TMP_2367(uint256) = msg.value (c)- reservationFee_3
REF_1287(uint256) -> Conversion.GWEI
TMP_2368(uint256) = TMP_2367 (c)/ REF_1287
TMP_2369(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_2368'] 
cr_8(CollateralReservation.Data) := phi(['cr_7'])
REF_1286(uint64) (->cr_8) := TMP_2369(uint64)
cr_9(CollateralReservation.Data) := phi(['cr_8', 'cr_6'])
 (lastUnderlyingBlock,lastUnderlyingTimestamp) = _lastPaymentBlock()
TUPLE_24(uint64,uint64) = INTERNAL_CALL, CollateralReservationsFacet._lastPaymentBlock()()
lastUnderlyingBlock_1(uint64)= UNPACK TUPLE_24 index: 0 
lastUnderlyingTimestamp_1(uint64)= UNPACK TUPLE_24 index: 1 
 cr.firstUnderlyingBlock = state.currentUnderlyingBlock
REF_1289(uint64) -> cr_9.firstUnderlyingBlock
REF_1290(uint64) -> state_2 (-> ['TMP_2330']).currentUnderlyingBlock
cr_10(CollateralReservation.Data) := phi(['cr_9'])
REF_1289(uint64) (->cr_10) := REF_1290(uint64)
 cr.lastUnderlyingBlock = lastUnderlyingBlock
REF_1291(uint64) -> cr_10.lastUnderlyingBlock
cr_11(CollateralReservation.Data) := phi(['cr_10'])
REF_1291(uint64) (->cr_11) := lastUnderlyingBlock_1(uint64)
 cr.lastUnderlyingTimestamp = lastUnderlyingTimestamp
REF_1292(uint64) -> cr_11.lastUnderlyingTimestamp
cr_12(CollateralReservation.Data) := phi(['cr_11'])
REF_1292(uint64) (->cr_12) := lastUnderlyingTimestamp_1(uint64)
 cr.status = CollateralReservation.Status.ACTIVE
REF_1293(CollateralReservation.Status) -> cr_12.status
REF_1294(CollateralReservation.Status) -> Status.ACTIVE
cr_13(CollateralReservation.Data) := phi(['cr_12'])
REF_1293(CollateralReservation.Status) (->cr_13) := REF_1294(CollateralReservation.Status)
 state.crts[crtId] = cr
REF_1295(mapping(uint256 => CollateralReservation.Data)) -> state_2 (-> ['TMP_2330']).crts
REF_1296(CollateralReservation.Data) -> REF_1295[crtId_1]
state_3 (-> ['TMP_2330'])(AssetManagerState.State) := phi(["state_2 (-> ['TMP_2330'])"])
REF_1296(CollateralReservation.Data) (->state_3 (-> ['TMP_2330'])) := cr_13(CollateralReservation.Data)
TMP_2330(AssetManagerState.State) := phi(["state_3 (-> ['TMP_2330'])"])
 _emitCollateralReservationEvent(agent,cr,crtId)
INTERNAL_CALL, CollateralReservationsFacet._emitCollateralReservationEvent(Agent.State,CollateralReservation.Data,uint256)(agent_1 (-> ['TMP_2327']),cr_13,crtId_1)
 cr.executor == address(0) && msg.value > reservationFee
REF_1297(address) -> cr_13.executor
TMP_2371 = CONVERT 0 to address
TMP_2372(bool) = REF_1297 == TMP_2371
TMP_2373(bool) = msg.value > reservationFee_3
TMP_2374(bool) = TMP_2372 && TMP_2373
CONDITION TMP_2374
 Transfers.transferNAT(address(msg.sender),msg.value - reservationFee)
TMP_2375 = CONVERT msg.sender to address
TMP_2376(uint256) = msg.value (c)- reservationFee_3
LIBRARY_CALL, dest:Transfers, function:Transfers.transferNAT(address,uint256), arguments:['TMP_2375', 'TMP_2376'] 
 crtId
RETURN crtId_1
 onlyAttached()
MODIFIER_CALL, AssetManagerBase.onlyAttached()()
 notEmergencyPaused()
MODIFIER_CALL, AssetManagerBase.notEmergencyPaused()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 agent.availableAgentsPos != 0
REF_1299(uint32) -> agent_1 (-> ['TMP_2327']).availableAgentsPos
TMP_2381(bool) = REF_1299 != 0
CONDITION TMP_2381
 reservationFee = _reservationFee(collateralData.poolCollateral.amgToTokenWeiPrice,valueAMG)
REF_1300(Collateral.Data) -> collateralData_1.poolCollateral
REF_1301(uint256) -> REF_1300.amgToTokenWeiPrice
TMP_2382(uint256) = INTERNAL_CALL, CollateralReservationsFacet._reservationFee(uint256,uint64)(REF_1301,valueAMG_1)
reservationFee_1(uint256) := TMP_2382(uint256)
 reservationFee = 0
reservationFee_2(uint256) := 0(uint256)
reservationFee_3(uint256) := phi(['reservationFee_1', 'reservationFee_2'])
 _collateralReservationId
```
#### Conversion.convertAmgToUBA(uint64) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4637(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4637'])(AssetManagerSettings.Data) := TMP_4637(AssetManagerSettings.Data)
 uint256(_valueAMG) * settings.assetMintingGranularityUBA
TMP_4638 = CONVERT _valueAMG_1 to uint256
REF_3145(uint64) -> settings_1 (-> ['TMP_4637']).assetMintingGranularityUBA
TMP_4639(uint256) = TMP_4638 (c)* REF_3145
RETURN TMP_4639
```
#### Conversion.convertUBAToAmg(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4640(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4640'])(AssetManagerSettings.Data) := TMP_4640(AssetManagerSettings.Data)
 SafeCast.toUint64(_valueUBA / settings.assetMintingGranularityUBA)
REF_3148(uint64) -> settings_1 (-> ['TMP_4640']).assetMintingGranularityUBA
TMP_4641(uint256) = _valueUBA_1 (c)/ REF_3148
TMP_4642(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_4641'] 
RETURN TMP_4642
```
#### Minting.calculateCurrentPoolFeeUBA(Agent.State,uint256) [INTERNAL]
```slithir
 mintingFeeUBA = _mintingValueUBA.mulBips(_agent.feeBIPS)
REF_3359(uint16) -> _agent_1 (-> []).feeBIPS
TMP_4852(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['_mintingValueUBA_1', 'REF_3359'] 
mintingFeeUBA_1(uint256) := TMP_4852(uint256)
 _calculatePoolFeeUBA(mintingFeeUBA,_agent.poolFeeShareBIPS)
REF_3360(uint16) -> _agent_1 (-> []).poolFeeShareBIPS
TMP_4853(uint256) = INTERNAL_CALL, Minting._calculatePoolFeeUBA(uint256,uint16)(mintingFeeUBA_1,REF_3360)
RETURN TMP_4853
```
#### Agent.vaultAddress(Agent.State) [INTERNAL]
```slithir
AGENTS_POSITION_2(bytes32) := phi(['AGENTS_POSITION_0'])
 position = _agent
position_1(bytes32) := _agent_1 (-> [])(Agent.State)
 address(uint160((uint256(position) ^ uint256(AGENTS_POSITION)) >> 64))
TMP_5325 = CONVERT position_1 to uint256
TMP_5326 = CONVERT AGENTS_POSITION_2 to uint256
TMP_5327(uint256) = TMP_5325 ^ TMP_5326
TMP_5328(uint256) = TMP_5327 >> 64
TMP_5329 = CONVERT TMP_5328 to uint160
TMP_5330 = CONVERT TMP_5329 to address
RETURN TMP_5330
```
#### PaymentReference.minting(uint256) [INTERNAL]
```slithir
MAX_ID_1(uint256) := phi(['MAX_ID_0'])
MINTING_1(uint256) := phi(['MINTING_0'])
 assert(bool)(_id <= MAX_ID)
TMP_5345(bool) = _id_1 <= MAX_ID_1
TMP_5346(None) = SOLIDITY_CALL assert(bool)(TMP_5345)
 bytes32(_id | MINTING)
TMP_5347(uint256) = _id_1 | MINTING_1
TMP_5348 = CONVERT TMP_5347 to bytes32
RETURN TMP_5348
```
#### Globals.getSettings() [INTERNAL]
```slithir
ASSET_MANAGER_SETTINGS_POSITION_1(bytes32) := phi(['ASSET_MANAGER_SETTINGS_POSITION_0'])
 position = ASSET_MANAGER_SETTINGS_POSITION
position_1(bytes32) := ASSET_MANAGER_SETTINGS_POSITION_1(bytes32)
 _settings = position
_settings_1 (-> ['position'])(AssetManagerSettings.Data) := position_1(bytes32)
 _settings
RETURN _settings_1 (-> ['position'])
```
#### AssetManagerState.get() [INTERNAL]
```slithir
STATE_POSITION_1(bytes32) := phi(['STATE_POSITION_0'])
 position = STATE_POSITION
position_1(bytes32) := STATE_POSITION_1(bytes32)
 _state = position
_state_1 (-> ['position'])(AssetManagerState.State) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
```
#### SafeCast.toUint64(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint64).max,SafeCast: value doesn't fit in 64 bits)
TMP_747(uint64) := 18446744073709551615(uint64)
TMP_748(bool) = value_1 <= TMP_747
TMP_749(None) = SOLIDITY_CALL require(bool,string)(TMP_748,SafeCast: value doesn't fit in 64 bits)
 uint64(value)
TMP_750 = CONVERT value_1 to uint64
RETURN TMP_750
```
#### Conversion.convertAmgToTokenWei(uint256,uint256) [INTERNAL]
```slithir
AMG_TOKEN_WEI_PRICE_SCALE_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_0'])
 _valueAMG.mulDiv(_amgToTokenWeiPrice,AMG_TOKEN_WEI_PRICE_SCALE)
TMP_4674(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_valueAMG_1', '_amgToTokenWeiPrice_1', 'AMG_TOKEN_WEI_PRICE_SCALE_1'] 
RETURN TMP_4674
```
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
```
#### Minting.checkMintingCap(uint64) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4838(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4838'])(AssetManagerState.State) := TMP_4838(AssetManagerState.State)
 settings = Globals.getSettings()
TMP_4839(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4839'])(AssetManagerSettings.Data) := TMP_4839(AssetManagerSettings.Data)
 mintingCapAMG = settings.mintingCapAMG
REF_3350(uint64) -> settings_1 (-> ['TMP_4839']).mintingCapAMG
mintingCapAMG_1(uint256) := REF_3350(uint64)
 mintingCapAMG == 0
TMP_4840(bool) = mintingCapAMG_1 == 0
CONDITION TMP_4840
 totalMintedUBA = IERC20(settings.fAsset).totalSupply()
REF_3351(address) -> settings_1 (-> ['TMP_4839']).fAsset
TMP_4841 = CONVERT REF_3351 to IERC20
TMP_4842(uint256) = HIGH_LEVEL_CALL, dest:TMP_4841(IERC20), function:totalSupply, arguments:[]  
totalMintedUBA_1(uint256) := TMP_4842(uint256)
 totalAMG = state.totalReservedCollateralAMG + Conversion.convertUBAToAmg(totalMintedUBA)
REF_3353(uint64) -> state_1 (-> ['TMP_4838']).totalReservedCollateralAMG
TMP_4843(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['totalMintedUBA_1'] 
TMP_4844(uint64) = REF_3353 (c)+ TMP_4843
totalAMG_1(uint256) := TMP_4844(uint64)
 require(bool,error)(totalAMG + _increaseAMG <= mintingCapAMG,revert MintingCapExceeded()())
TMP_4845(uint256) = totalAMG_1 (c)+ _increaseAMG_1
TMP_4846(bool) = TMP_4845 <= mintingCapAMG_1
TMP_4847(None) = SOLIDITY_CALL revert MintingCapExceeded()()
TMP_4848(None) = SOLIDITY_CALL require(bool,error)(TMP_4846,TMP_4847)
```
#### Conversion.convertLotsToAMG(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4646(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4646'])(AssetManagerSettings.Data) := TMP_4646(AssetManagerSettings.Data)
 SafeCast.toUint64(_lots * settings.lotSizeAMG)
REF_3153(uint64) -> settings_1 (-> ['TMP_4646']).lotSizeAMG
TMP_4647(uint256) = _lots_1 (c)* REF_3153
TMP_4648(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_4647'] 
RETURN TMP_4648
```
#### Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data) [INTERNAL]
```slithir
_token_1 (-> [])(CollateralTypeInt.Data) := phi(['_toToken_1 (-> [])', '_fromToken_1 (-> [])'])
 (_price,None,None) = currentAmgPriceInTokenWeiWithTs(_token,false)
TUPLE_44(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool)(_token_1 (-> []),False)
_price_1(uint256)= UNPACK TUPLE_44 index: 0 
 _price
RETURN _price_1
```
#### AgentCollateral.combinedData(Agent.State) [INTERNAL]
```slithir
 poolCollateral = poolCollateralData(_agent)
TMP_4296(Collateral.Data) = INTERNAL_CALL, AgentCollateral.poolCollateralData(Agent.State)(_agent_1 (-> []))
poolCollateral_1(Collateral.Data) := TMP_4296(Collateral.Data)
 Collateral.CombinedData({agentCollateral:agentVaultCollateralData(_agent),poolCollateral:poolCollateral,agentPoolTokens:agentsPoolTokensCollateralData(_agent,poolCollateral)})
TMP_4297(Collateral.Data) = INTERNAL_CALL, AgentCollateral.agentVaultCollateralData(Agent.State)(_agent_1 (-> []))
TMP_4298(Collateral.Data) = INTERNAL_CALL, AgentCollateral.agentsPoolTokensCollateralData(Agent.State,Collateral.Data)(_agent_1 (-> []),poolCollateral_1)
TMP_4299(Collateral.CombinedData) = new CombinedData(TMP_4297,poolCollateral_1,TMP_4298)
RETURN TMP_4299
```
#### AgentCollateral.freeCollateralLots(Collateral.CombinedData,Agent.State) [INTERNAL]
```slithir
 freeCollateralLotsOptionalFee(_data,_agent,true)
TMP_4322(uint256) = INTERNAL_CALL, AgentCollateral.freeCollateralLotsOptionalFee(Collateral.CombinedData,Agent.State,bool)(_data_1,_agent_1 (-> []),True)
RETURN TMP_4322
 _lots
```
#### Agents.requireWhitelistedAgentVaultOwner(Agent.State) [INTERNAL]
```slithir
 requireWhitelisted(_agent.ownerManagementAddress)
REF_2996(address) -> _agent_1 (-> []).ownerManagementAddress
INTERNAL_CALL, Agents.requireWhitelisted(address)(REF_2996)
```
#### Agent.get(address) [INTERNAL]
```slithir
 agent = getWithoutCheck(_address)
TMP_5309(Agent.State) = INTERNAL_CALL, Agent.getWithoutCheck(address)(_address_1)
agent_1 (-> ['TMP_5309'])(Agent.State) := TMP_5309(Agent.State)
 status = agent.status
REF_3748(Agent.Status) -> agent_1 (-> ['TMP_5309']).status
status_1(Agent.Status) := REF_3748(Agent.Status)
 require(bool,error)(status != Agent.Status.EMPTY && status != Agent.Status.DESTROYED,revert InvalidAgentVaultAddress()())
REF_3749(Agent.Status) -> Status.EMPTY
TMP_5310(bool) = status_1 != REF_3749
REF_3750(Agent.Status) -> Status.DESTROYED
TMP_5311(bool) = status_1 != REF_3750
TMP_5312(bool) = TMP_5310 && TMP_5311
TMP_5313(None) = SOLIDITY_CALL revert InvalidAgentVaultAddress()()
TMP_5314(None) = SOLIDITY_CALL require(bool,error)(TMP_5312,TMP_5313)
 agent
RETURN agent_1 (-> ['TMP_5309'])
```
#### PaymentReference.randomizedIdSkip() [INTERNAL]
```slithir
ID_RANDOMIZATION_1(uint256) := phi(['ID_RANDOMIZATION_0'])
 uint64(block.number % ID_RANDOMIZATION + 1)
TMP_5382(uint256) = block.number % ID_RANDOMIZATION_1
TMP_5383(uint256) = TMP_5382 (c)+ 1
TMP_5384 = CONVERT TMP_5383 to uint64
RETURN TMP_5384
```
#### Transfers.transferNAT(address,uint256) [INTERNAL]
```slithir
TRANSFER_GAS_ALLOWANCE_1(uint256) := phi(['TRANSFER_GAS_ALLOWANCE_0', 'TRANSFER_GAS_ALLOWANCE_3', 'TRANSFER_GAS_ALLOWANCE_2'])
 _amount > 0
TMP_10536(bool) = _amount_1 > 0
CONDITION TMP_10536
 (success,None) = _recipient.call{gas: TRANSFER_GAS_ALLOWANCE,value: _amount}()
TUPLE_91(bool,bytes) = LOW_LEVEL_CALL, dest:_recipient_1, function:call, arguments:[''] value:_amount_1 gas:TRANSFER_GAS_ALLOWANCE_2
TRANSFER_GAS_ALLOWANCE_3(uint256) := phi(['TRANSFER_GAS_ALLOWANCE_3', 'TRANSFER_GAS_ALLOWANCE_2'])
success_1(bool)= UNPACK TUPLE_91 index: 0 
 require(bool,error)(success,revert TransferFailed()())
TMP_10537(None) = SOLIDITY_CALL revert TransferFailed()()
TMP_10538(None) = SOLIDITY_CALL require(bool,error)(success_1,TMP_10537)
 requireReentrancyGuard()
MODIFIER_CALL, Transfers.requireReentrancyGuard()()
```
#### SafeCast.toUint128(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint128).max,SafeCast: value doesn't fit in 128 bits)
TMP_707(uint128) := 340282366920938463463374607431768211455(uint128)
TMP_708(bool) = value_1 <= TMP_707
TMP_709(None) = SOLIDITY_CALL require(bool,string)(TMP_708,SafeCast: value doesn't fit in 128 bits)
 uint128(value)
TMP_710 = CONVERT value_1 to uint128
RETURN TMP_710
```
#### EnumerableSet.contains(EnumerableSet.UintSet,uint256) [INTERNAL]
```slithir
 _contains(set._inner,bytes32(value))
REF_234(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_944 = CONVERT value_1 to bytes32
TMP_945(bool) = INTERNAL_CALL, EnumerableSet._contains(EnumerableSet.Set,bytes32)(REF_234,TMP_944)
RETURN TMP_945
```

#### SafePct.mulDiv(uint256,uint256,uint256) [INTERNAL]
```slithir
x_1(uint256) := phi(['x_1', 'x_1'])
y_1(uint256) := phi(['y_1', 'y_1'])
z_1(uint256) := phi(['z_1', 'MAX_BIPS_1'])
 require(bool,error)(z > 0,revert DivisionByZero()())
TMP_10510(bool) = z_1 > 0
TMP_10511(None) = SOLIDITY_CALL revert DivisionByZero()()
TMP_10512(None) = SOLIDITY_CALL require(bool,error)(TMP_10510,TMP_10511)
 x == 0
TMP_10513(bool) = x_1 == 0
CONDITION TMP_10513
 0
RETURN 0
 xy = x * y
TMP_10514(uint256) = x_1 * y_1
xy_1(uint256) := TMP_10514(uint256)
 xy / x == y
TMP_10515(uint256) = xy_1 / x_1
TMP_10516(bool) = TMP_10515 == y_1
CONDITION TMP_10516
 xy / z
TMP_10517(uint256) = xy_1 / z_1
RETURN TMP_10517
 a = x / z
TMP_10518(uint256) = x_1 (c)/ z_1
a_1(uint256) := TMP_10518(uint256)
 b = x % z
TMP_10519(uint256) = x_1 % z_1
b_1(uint256) := TMP_10519(uint256)
 c = y / z
TMP_10520(uint256) = y_1 (c)/ z_1
c_1(uint256) := TMP_10520(uint256)
 d = y % z
TMP_10521(uint256) = y_1 % z_1
d_1(uint256) := TMP_10521(uint256)
 (a * c * z) + (a * d) + (b * c) + (b * d / z)
TMP_10522(uint256) = a_1 (c)* c_1
TMP_10523(uint256) = TMP_10522 (c)* z_1
TMP_10524(uint256) = a_1 (c)* d_1
TMP_10525(uint256) = TMP_10523 (c)+ TMP_10524
TMP_10526(uint256) = b_1 (c)* c_1
TMP_10527(uint256) = TMP_10525 (c)+ TMP_10526
TMP_10528(uint256) = b_1 (c)* d_1
TMP_10529(uint256) = TMP_10528 (c)/ z_1
TMP_10530(uint256) = TMP_10527 (c)+ TMP_10529
RETURN TMP_10530
```
#### ERC20.totalSupply() [PUBLIC]
```slithir
_totalSupply_1(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 _totalSupply
RETURN _totalSupply_1
```
#### Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool) [INTERNAL]
```slithir
_token_1 (-> [])(CollateralTypeInt.Data) := phi(['_token_1 (-> [])', 'REF_3140', '_token_1 (-> [])'])
 (assetPrice,assetTs,assetFtsoDec) = readFtsoPrice(_token.assetFtsoSymbol,_fromTrustedProviders)
REF_3163(string) -> _token_1 (-> []).assetFtsoSymbol
TUPLE_48(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.readFtsoPrice(string,bool)(REF_3163,_fromTrustedProviders_1)
assetPrice_1(uint256)= UNPACK TUPLE_48 index: 0 
assetTs_1(uint256)= UNPACK TUPLE_48 index: 1 
assetFtsoDec_1(uint256)= UNPACK TUPLE_48 index: 2 
 _token.directPricePair
REF_3164(bool) -> _token_1 (-> []).directPricePair
CONDITION REF_3164
 price = calcAmgToTokenWeiPrice(_token.decimals,1,0,assetPrice,assetFtsoDec)
REF_3165(uint8) -> _token_1 (-> []).decimals
TMP_4661(uint256) = INTERNAL_CALL, Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256)(REF_3165,1,0,assetPrice_1,assetFtsoDec_1)
price_1(uint256) := TMP_4661(uint256)
 (price,assetTs,assetTs)
RETURN price_1,assetTs_1,assetTs_1
 (tokenPrice,tokenTs,tokenFtsoDec) = readFtsoPrice(_token.tokenFtsoSymbol,_fromTrustedProviders)
REF_3166(string) -> _token_1 (-> []).tokenFtsoSymbol
TUPLE_49(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.readFtsoPrice(string,bool)(REF_3166,_fromTrustedProviders_1)
tokenPrice_1(uint256)= UNPACK TUPLE_49 index: 0 
tokenTs_1(uint256)= UNPACK TUPLE_49 index: 1 
tokenFtsoDec_1(uint256)= UNPACK TUPLE_49 index: 2 
 price_scope_0 = calcAmgToTokenWeiPrice(_token.decimals,tokenPrice,tokenFtsoDec,assetPrice,assetFtsoDec)
REF_3167(uint8) -> _token_1 (-> []).decimals
TMP_4662(uint256) = INTERNAL_CALL, Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256)(REF_3167,tokenPrice_1,tokenFtsoDec_1,assetPrice_1,assetFtsoDec_1)
price_scope_0_1(uint256) := TMP_4662(uint256)
 (price_scope_0,assetTs,tokenTs)
RETURN price_scope_0_1,assetTs_1,tokenTs_1
```
#### AgentCollateral.agentVaultCollateralData(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
 collateral = _agent.getVaultCollateral()
TMP_4306(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4306'])(CollateralTypeInt.Data) := TMP_4306(CollateralTypeInt.Data)
 Collateral.Data({kind:Collateral.Kind.VAULT,fullCollateral:collateral.token.balanceOf(_agent.vaultAddress()),amgToTokenWeiPrice:Conversion.currentAmgPriceInTokenWei(collateral)})
REF_2819(Collateral.Kind) -> Kind.VAULT
REF_2820(IERC20) -> collateral_1 (-> ['TMP_4306']).token
TMP_4307(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4308(uint256) = HIGH_LEVEL_CALL, dest:REF_2820(IERC20), function:balanceOf, arguments:['TMP_4307']  
TMP_4309(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data), arguments:["collateral_1 (-> ['TMP_4306'])"] 
TMP_4310(Collateral.Data) = new Data(REF_2819,TMP_4308,TMP_4309)
RETURN TMP_4310
```
#### AgentCollateral.agentsPoolTokensCollateralData(Agent.State,Collateral.Data) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
_poolCollateral_1(Collateral.Data) := phi(['poolCollateral_1', 'TMP_4304'])
 poolToken = _agent.collateralPool.poolToken()
REF_2830(IICollateralPool) -> _agent_1 (-> []).collateralPool
TMP_4315(ICollateralPoolToken) = HIGH_LEVEL_CALL, dest:REF_2830(IICollateralPool), function:poolToken, arguments:[]  
poolToken_1(IERC20) := TMP_4315(ICollateralPoolToken)
 agentPoolTokens = poolToken.balanceOf(_agent.vaultAddress())
TMP_4316(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4317(uint256) = HIGH_LEVEL_CALL, dest:poolToken_1(IERC20), function:balanceOf, arguments:['TMP_4316']  
agentPoolTokens_1(uint256) := TMP_4317(uint256)
 totalPoolTokens = poolToken.totalSupply()
TMP_4318(uint256) = HIGH_LEVEL_CALL, dest:poolToken_1(IERC20), function:totalSupply, arguments:[]  
totalPoolTokens_1(uint256) := TMP_4318(uint256)
 Collateral.Data({kind:Collateral.Kind.AGENT_POOL,fullCollateral:agentPoolTokens,amgToTokenWeiPrice:amgToPoolTokenWeiPrice})
REF_2836(Collateral.Kind) -> Kind.AGENT_POOL
TMP_4319(Collateral.Data) = new Data(REF_2836,agentPoolTokens_1,amgToPoolTokenWeiPrice_3)
RETURN TMP_4319
 _poolCollateral.fullCollateral != 0
REF_2837(uint256) -> _poolCollateral_1.fullCollateral
TMP_4320(bool) = REF_2837 != 0
CONDITION TMP_4320
 amgToPoolTokenWeiPrice = _poolCollateral.amgToTokenWeiPrice.mulDiv(totalPoolTokens,_poolCollateral.fullCollateral)
REF_2838(uint256) -> _poolCollateral_1.amgToTokenWeiPrice
REF_2840(uint256) -> _poolCollateral_1.fullCollateral
TMP_4321(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['REF_2838', 'totalPoolTokens_1', 'REF_2840'] 
amgToPoolTokenWeiPrice_1(uint256) := TMP_4321(uint256)
 amgToPoolTokenWeiPrice = _poolCollateral.amgToTokenWeiPrice
REF_2841(uint256) -> _poolCollateral_1.amgToTokenWeiPrice
amgToPoolTokenWeiPrice_2(uint256) := REF_2841(uint256)
amgToPoolTokenWeiPrice_3(uint256) := phi(['amgToPoolTokenWeiPrice_1', 'amgToPoolTokenWeiPrice_2'])
```
#### AgentCollateral.poolCollateralData(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
 collateral = _agent.getPoolCollateral()
TMP_4311(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getPoolCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4311'])(CollateralTypeInt.Data) := TMP_4311(CollateralTypeInt.Data)
 Collateral.Data({kind:Collateral.Kind.POOL,fullCollateral:_agent.collateralPool.totalCollateral(),amgToTokenWeiPrice:Conversion.currentAmgPriceInTokenWei(collateral)})
REF_2826(Collateral.Kind) -> Kind.POOL
REF_2827(IICollateralPool) -> _agent_1 (-> []).collateralPool
TMP_4312(uint256) = HIGH_LEVEL_CALL, dest:REF_2827(IICollateralPool), function:totalCollateral, arguments:[]  
TMP_4313(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data), arguments:["collateral_1 (-> ['TMP_4311'])"] 
TMP_4314(Collateral.Data) = new Data(REF_2826,TMP_4312,TMP_4313)
RETURN TMP_4314
```
#### AgentCollateral.freeCollateralLotsOptionalFee(Collateral.CombinedData,Agent.State,bool) [INTERNAL]
```slithir
_data_1(Collateral.CombinedData) := phi(['_data_1'])
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 agentLots = freeSingleCollateralLots(_data.agentCollateral,_agent,_chargePoolFee)
REF_2842(Collateral.Data) -> _data_1.agentCollateral
TMP_4323(uint256) = INTERNAL_CALL, AgentCollateral.freeSingleCollateralLots(Collateral.Data,Agent.State,bool)(REF_2842,_agent_1 (-> []),_chargePoolFee_1)
agentLots_1(uint256) := TMP_4323(uint256)
 poolLots = freeSingleCollateralLots(_data.poolCollateral,_agent,_chargePoolFee)
REF_2843(Collateral.Data) -> _data_1.poolCollateral
TMP_4324(uint256) = INTERNAL_CALL, AgentCollateral.freeSingleCollateralLots(Collateral.Data,Agent.State,bool)(REF_2843,_agent_1 (-> []),_chargePoolFee_1)
poolLots_1(uint256) := TMP_4324(uint256)
 agentPoolTokenLots = freeSingleCollateralLots(_data.agentPoolTokens,_agent,_chargePoolFee)
REF_2844(Collateral.Data) -> _data_1.agentPoolTokens
TMP_4325(uint256) = INTERNAL_CALL, AgentCollateral.freeSingleCollateralLots(Collateral.Data,Agent.State,bool)(REF_2844,_agent_1 (-> []),_chargePoolFee_1)
agentPoolTokenLots_1(uint256) := TMP_4325(uint256)
 Math.min(agentLots,Math.min(poolLots,agentPoolTokenLots))
TMP_4326(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['poolLots_1', 'agentPoolTokenLots_1'] 
TMP_4327(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['agentLots_1', 'TMP_4326'] 
RETURN TMP_4327
 _lots
```
#### Agents.requireWhitelisted(address) [INTERNAL]
```slithir
_ownerManagementAddress_1(address) := phi(['REF_2996'])
 require(bool,error)(Globals.getAgentOwnerRegistry().isWhitelisted(_ownerManagementAddress),revert AgentNotWhitelisted()())
TMP_4488(IAgentOwnerRegistry) = LIBRARY_CALL, dest:Globals, function:Globals.getAgentOwnerRegistry(), arguments:[] 
TMP_4489(bool) = HIGH_LEVEL_CALL, dest:TMP_4488(IAgentOwnerRegistry), function:isWhitelisted, arguments:['_ownerManagementAddress_1']  
TMP_4490(None) = SOLIDITY_CALL revert AgentNotWhitelisted()()
TMP_4491(None) = SOLIDITY_CALL require(bool,error)(TMP_4489,TMP_4490)
```
#### Agent.getWithoutCheck(address) [INTERNAL]
```slithir
_address_1(address) := phi(['_address_1', '_address_1'])
AGENTS_POSITION_1(bytes32) := phi(['AGENTS_POSITION_0'])
 position = bytes32(uint256(AGENTS_POSITION) ^ (uint256(uint160(_address)) << 64))
TMP_5319 = CONVERT AGENTS_POSITION_1 to uint256
TMP_5320 = CONVERT _address_1 to uint160
TMP_5321 = CONVERT TMP_5320 to uint256
TMP_5322(uint256) = TMP_5321 << 64
TMP_5323(uint256) = TMP_5319 ^ TMP_5322
TMP_5324 = CONVERT TMP_5323 to bytes32
position_1(bytes32) := TMP_5324(bytes32)
 _agent = position
_agent_1 (-> ['position'])(Agent.State) := position_1(bytes32)
 _agent
RETURN _agent_1 (-> ['position'])
```
#### EnumerableSet._contains(EnumerableSet.Set,bytes32) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_234', 'set_1 (-> [])', 'REF_228', 'REF_222'])
value_1(bytes32) := phi(['value_1', 'value_1', 'TMP_944', 'TMP_932'])
 set._indexes[value] != 0
REF_213(mapping(bytes32 => uint256)) -> set_1 (-> [])._indexes
REF_214(uint256) -> REF_213[value_1]
TMP_915(bool) = REF_214 != 0
RETURN TMP_915
```
#### Conversion.roundUBAToAmg(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4643(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4643'])(AssetManagerSettings.Data) := TMP_4643(AssetManagerSettings.Data)
 _valueUBA - (_valueUBA % settings.assetMintingGranularityUBA)
REF_3150(uint64) -> settings_1 (-> ['TMP_4643']).assetMintingGranularityUBA
TMP_4644(uint256) = _valueUBA_1 % REF_3150
TMP_4645(uint256) = _valueUBA_1 (c)- TMP_4644
RETURN TMP_4645
```
#### Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256) [INTERNAL]
```slithir
_tokenDecimals_1(uint256) := phi(['REF_3165', 'REF_3167'])
_tokenPrice_1(uint256) := phi(['tokenPrice_1'])
_tokenFtsoDecimals_1(uint256) := phi(['tokenFtsoDec_1'])
_assetPrice_1(uint256) := phi(['assetPrice_1'])
_assetFtsoDecimals_1(uint256) := phi(['assetFtsoDec_1'])
AMG_TOKEN_WEI_PRICE_SCALE_EXP_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_EXP_0'])
 settings = Globals.getSettings()
TMP_4665(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4665'])(AssetManagerSettings.Data) := TMP_4665(AssetManagerSettings.Data)
 expPlus = _tokenDecimals + _tokenFtsoDecimals + AMG_TOKEN_WEI_PRICE_SCALE_EXP
TMP_4666(uint256) = _tokenDecimals_1 (c)+ _tokenFtsoDecimals_1
TMP_4667(uint256) = TMP_4666 (c)+ AMG_TOKEN_WEI_PRICE_SCALE_EXP_1
expPlus_1(uint256) := TMP_4667(uint256)
 expMinus = settings.assetMintingDecimals + _assetFtsoDecimals
REF_3173(uint8) -> settings_1 (-> ['TMP_4665']).assetMintingDecimals
TMP_4668(uint8) = REF_3173 (c)+ _assetFtsoDecimals_1
expMinus_1(uint256) := TMP_4668(uint8)
 assert(bool)(expPlus >= expMinus)
TMP_4669(bool) = expPlus_1 >= expMinus_1
TMP_4670(None) = SOLIDITY_CALL assert(bool)(TMP_4669)
 _assetPrice.mulDiv(10 ** (expPlus - expMinus),_tokenPrice)
TMP_4671(uint256) = expPlus_1 (c)- expMinus_1
TMP_4672(uint256) = 10 (c)** TMP_4671
TMP_4673(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_assetPrice_1', 'TMP_4672', '_tokenPrice_1'] 
RETURN TMP_4673
```
#### Conversion.readFtsoPrice(string,bool) [INTERNAL]
```slithir
_symbol_1(string) := phi(['REF_3163', 'REF_3166', 'REF_3160'])
_fromTrustedProviders_1(bool) := phi(['_fromTrustedProviders_1'])
 settings = Globals.getSettings()
TMP_4663(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4663'])(AssetManagerSettings.Data) := TMP_4663(AssetManagerSettings.Data)
 priceReader = IPriceReader(settings.priceReader)
REF_3169(address) -> settings_1 (-> ['TMP_4663']).priceReader
TMP_4664 = CONVERT REF_3169 to IPriceReader
priceReader_1(IPriceReader) := TMP_4664(IPriceReader)
 _fromTrustedProviders
CONDITION _fromTrustedProviders_1
 priceReader.getPriceFromTrustedProviders(_symbol)
TUPLE_50(uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:priceReader_1(IPriceReader), function:getPriceFromTrustedProviders, arguments:['_symbol_1']  
RETURN TUPLE_50
 priceReader.getPrice(_symbol)
TUPLE_51(uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:priceReader_1(IPriceReader), function:getPrice, arguments:['_symbol_1']  
RETURN TUPLE_51
```
#### Agents.getVaultCollateral(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 state = AssetManagerState.get()
TMP_4510(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4510'])(AssetManagerState.State) := TMP_4510(AssetManagerState.State)
 state.collateralTokens[_agent.vaultCollateralIndex]
REF_3005(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4510']).collateralTokens
REF_3006(uint16) -> _agent_1 (-> []).vaultCollateralIndex
REF_3007(CollateralTypeInt.Data) -> REF_3005[REF_3006]
RETURN REF_3007
```
#### ERC20.balanceOf(address) [PUBLIC]
```slithir
_balances_1(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
 _balances[account]
REF_73(uint256) -> _balances_1[account_1]
RETURN REF_73
```
#### Agents.getPoolCollateral(Agent.State) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4516(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4516'])(AssetManagerState.State) := TMP_4516(AssetManagerState.State)
 state.collateralTokens[_agent.poolCollateralIndex]
REF_3015(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4516']).collateralTokens
REF_3016(uint16) -> _agent_1 (-> []).poolCollateralIndex
REF_3017(CollateralTypeInt.Data) -> REF_3015[REF_3016]
RETURN REF_3017
```
#### AgentCollateral.freeSingleCollateralLots(Collateral.Data,Agent.State,bool) [INTERNAL]
```slithir
_data_1(Collateral.Data) := phi(['REF_2842', 'REF_2844', 'REF_2843'])
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_chargePoolFee_1(bool) := phi(['_chargePoolFee_1'])
 collateralWei = freeCollateralWei(_data,_agent)
TMP_4328(uint256) = INTERNAL_CALL, AgentCollateral.freeCollateralWei(Collateral.Data,Agent.State)(_data_1,_agent_1 (-> []))
collateralWei_1(uint256) := TMP_4328(uint256)
 lotWei = mintingLotCollateralWei(_data,_agent,_chargePoolFee)
TMP_4329(uint256) = INTERNAL_CALL, AgentCollateral.mintingLotCollateralWei(Collateral.Data,Agent.State,bool)(_data_1,_agent_1 (-> []),_chargePoolFee_1)
lotWei_1(uint256) := TMP_4329(uint256)
 lotWei != 0
TMP_4330(bool) = lotWei_1 != 0
CONDITION TMP_4330
 collateralWei / lotWei
TMP_4331(uint256) = collateralWei_1 (c)/ lotWei_1
RETURN TMP_4331
 0
RETURN 0
```
#### Math.min(uint256,uint256) [INTERNAL]
```slithir
a_1(uint256) := phi(['result_8'])
b_1(uint256) := phi(['TMP_554'])
 a < b
TMP_477(bool) = a_1 < b_1
CONDITION TMP_477
 a
RETURN a_1
 b
RETURN b_1
```
#### Globals.getAgentOwnerRegistry() [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4737(AssetManagerSettings.Data) = INTERNAL_CALL, Globals.getSettings()()
settings_1 (-> ['TMP_4737'])(AssetManagerSettings.Data) := TMP_4737(AssetManagerSettings.Data)
 IAgentOwnerRegistry(settings.agentOwnerRegistry)
REF_3242(address) -> settings_1 (-> ['TMP_4737']).agentOwnerRegistry
TMP_4738 = CONVERT REF_3242 to IAgentOwnerRegistry
RETURN TMP_4738
```
#### AgentOwnerRegistry.isWhitelisted(address) [PUBLIC]
```slithir
_address_1(address) := phi(['msg.sender'])
whitelist_1(mapping(address => bool)) := phi(['whitelist_2', 'whitelist_1', 'whitelist_4', 'whitelist_5', 'whitelist_0', 'whitelist_3'])
 whitelist[_address]
REF_313(bool) -> whitelist_1[_address_1]
RETURN REF_313
```
