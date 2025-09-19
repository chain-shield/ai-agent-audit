



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
















### Storage layout (FAsset) 

```text
assetName string
assetSymbol string
cleanupBlockNumberManager address
assetManager address
__terminatedAt uint64
_name string
_symbol string
_decimals uint8
_deployer address
_initialized bool
_version uint16

```



### Storage layout (CoreVaultManager) 

```text
assetManager address
chainId bytes32
custodianAddress string
coreVaultAddressHash bytes32
coreVaultAddress string
nextSequenceNumber uint256
fdcVerification IFdcVerification
confirmedPayments mapping(bytes32 => bool)
preimageHashes EnumerableSet.Bytes32Set
escrows ICoreVaultManager.Escrow[]
preimageHashToEscrowIndex mapping(bytes32 => uint256)
nextUnusedPreimageHashIndex uint256
nextUnprocessedEscrowIndex uint256
nextTransferRequestId uint256
cancelableTransferRequests uint256[]
nonCancelableTransferRequests uint256[]
transferRequestById mapping(uint256 => ICoreVaultManager.TransferRequest)
allowedDestinationAddresses string[]
allowedDestinationAddressIndex mapping(string => uint256)
triggeringAccounts EnumerableSet.AddressSet
emergencyPauseSenders EnumerableSet.AddressSet
escrowEndTimeSeconds uint128
escrowAmount uint128
minimalAmount uint128
fee uint128
availableFunds uint128
escrowedFunds uint128
cancelableTransferRequestsAmount uint128
nonCancelableTransferRequestsAmount uint128
paused bool

```
#### CoreVaultClientFacet.cancelReturnFromCoreVault(address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_2548(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2548'])(Agent.State) := TMP_2548(Agent.State)
 state = CoreVaultClient.getState()
TMP_2549(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2549'])(CoreVaultClient.State) := TMP_2549(CoreVaultClient.State)
 requestId = agent.activeReturnFromCoreVaultId
REF_1423(uint64) -> agent_1 (-> ['TMP_2548']).activeReturnFromCoreVaultId
requestId_1(uint256) := REF_1423(uint64)
 require(bool,error)(requestId != 0,revert NoActiveReturnRequest()())
TMP_2550(bool) = requestId_1 != 0
TMP_2551(None) = SOLIDITY_CALL revert NoActiveReturnRequest()()
TMP_2552(None) = SOLIDITY_CALL require(bool,error)(TMP_2550,TMP_2551)
 state.coreVaultManager.cancelTransferRequestFromCoreVault(agent.underlyingAddressString)
REF_1424(IICoreVaultManager) -> state_1 (-> ['TMP_2549']).coreVaultManager
REF_1426(string) -> agent_1 (-> ['TMP_2548']).underlyingAddressString
HIGH_LEVEL_CALL, dest:REF_1424(IICoreVaultManager), function:cancelTransferRequestFromCoreVault, arguments:['REF_1426']  
 CoreVaultClient.deleteReturnFromCoreVaultRequest(agent)
LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.deleteReturnFromCoreVaultRequest(Agent.State), arguments:["agent_1 (-> ['TMP_2548'])"] 
 ReturnFromCoreVaultCancelled(_agentVault,requestId)
Emit ReturnFromCoreVaultCancelled(_agentVault_1,requestId_1)
 onlyEnabled()
MODIFIER_CALL, CoreVaultClientFacet.onlyEnabled()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
```
#### CoreVaultClientFacet.confirmReturnFromCoreVault(IPayment.Proof,address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_2559(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2559'])(Agent.State) := TMP_2559(Agent.State)
 state = CoreVaultClient.getState()
TMP_2560(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2560'])(CoreVaultClient.State) := TMP_2560(CoreVaultClient.State)
 TransactionAttestation.verifyPaymentSuccess(_payment)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyPaymentSuccess(IPayment.Proof), arguments:['_payment_1'] 
 requestId = agent.activeReturnFromCoreVaultId
REF_1431(uint64) -> agent_1 (-> ['TMP_2559']).activeReturnFromCoreVaultId
requestId_1(uint64) := REF_1431(uint64)
 require(bool,error)(requestId != 0,revert NoActiveReturnRequest()())
TMP_2562(bool) = requestId_1 != 0
TMP_2563(None) = SOLIDITY_CALL revert NoActiveReturnRequest()()
TMP_2564(None) = SOLIDITY_CALL require(bool,error)(TMP_2562,TMP_2563)
 require(bool,error)(_payment.data.responseBody.sourceAddressHash == state.coreVaultManager.coreVaultAddressHash(),revert PaymentNotFromCoreVault()())
REF_1432(IPayment.Response) -> _payment_1.data
REF_1433(IPayment.ResponseBody) -> REF_1432.responseBody
REF_1434(bytes32) -> REF_1433.sourceAddressHash
REF_1435(IICoreVaultManager) -> state_1 (-> ['TMP_2560']).coreVaultManager
TMP_2565(bytes32) = HIGH_LEVEL_CALL, dest:REF_1435(IICoreVaultManager), function:coreVaultAddressHash, arguments:[]  
TMP_2566(bool) = REF_1434 == TMP_2565
TMP_2567(None) = SOLIDITY_CALL revert PaymentNotFromCoreVault()()
TMP_2568(None) = SOLIDITY_CALL require(bool,error)(TMP_2566,TMP_2567)
 require(bool,error)(_payment.data.responseBody.receivingAddressHash == agent.underlyingAddressHash,revert PaymentNotToAgentsAddress()())
REF_1437(IPayment.Response) -> _payment_1.data
REF_1438(IPayment.ResponseBody) -> REF_1437.responseBody
REF_1439(bytes32) -> REF_1438.receivingAddressHash
REF_1440(bytes32) -> agent_1 (-> ['TMP_2559']).underlyingAddressHash
TMP_2569(bool) = REF_1439 == REF_1440
TMP_2570(None) = SOLIDITY_CALL revert PaymentNotToAgentsAddress()()
TMP_2571(None) = SOLIDITY_CALL require(bool,error)(TMP_2569,TMP_2570)
 require(bool,error)(_payment.data.responseBody.standardPaymentReference == PaymentReference.returnFromCoreVault(requestId),revert InvalidPaymentReference()())
REF_1441(IPayment.Response) -> _payment_1.data
REF_1442(IPayment.ResponseBody) -> REF_1441.responseBody
REF_1443(bytes32) -> REF_1442.standardPaymentReference
TMP_2572(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.returnFromCoreVault(uint256), arguments:['requestId_1'] 
TMP_2573(bool) = REF_1443 == TMP_2572
TMP_2574(None) = SOLIDITY_CALL revert InvalidPaymentReference()()
TMP_2575(None) = SOLIDITY_CALL require(bool,error)(TMP_2573,TMP_2574)
 AssetManagerState.get().paymentConfirmations.confirmIncomingPayment(_payment)
TMP_2576(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
REF_1446(PaymentConfirmations.State) -> TMP_2576.paymentConfirmations
LIBRARY_CALL, dest:PaymentConfirmations, function:PaymentConfirmations.confirmIncomingPayment(PaymentConfirmations.State,IPayment.Proof), arguments:['REF_1446', '_payment_1'] 
 receivedAmountUBA = _payment.data.responseBody.receivedAmount.toUint256()
REF_1448(IPayment.Response) -> _payment_1.data
REF_1449(IPayment.ResponseBody) -> REF_1448.responseBody
REF_1450(int256) -> REF_1449.receivedAmount
TMP_2578(uint256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint256(int256), arguments:['REF_1450'] 
receivedAmountUBA_1(uint256) := TMP_2578(uint256)
 receivedAmountAMG = Conversion.convertUBAToAmg(receivedAmountUBA)
TMP_2579(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['receivedAmountUBA_1'] 
receivedAmountAMG_1(uint64) := TMP_2579(uint64)
 remintedAMG = SafeMath64.min64(agent.returnFromCoreVaultReservedAMG,receivedAmountAMG)
REF_1454(uint64) -> agent_1 (-> ['TMP_2559']).returnFromCoreVaultReservedAMG
TMP_2580(uint64) = LIBRARY_CALL, dest:SafeMath64, function:SafeMath64.min64(uint64,uint64), arguments:['REF_1454', 'receivedAmountAMG_1'] 
remintedAMG_1(uint64) := TMP_2580(uint64)
 AgentBacking.createNewMinting(agent,remintedAMG)
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.createNewMinting(Agent.State,uint64), arguments:["agent_1 (-> ['TMP_2559'])", 'remintedAMG_1'] 
 UnderlyingBalance.increaseBalance(agent,receivedAmountUBA)
LIBRARY_CALL, dest:UnderlyingBalance, function:UnderlyingBalance.increaseBalance(Agent.State,uint256), arguments:["agent_1 (-> ['TMP_2559'])", 'receivedAmountUBA_1'] 
 UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(_payment)
LIBRARY_CALL, dest:UnderlyingBlockUpdater, function:UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(IPayment.Proof), arguments:['_payment_1'] 
 CoreVaultClient.deleteReturnFromCoreVaultRequest(agent)
LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.deleteReturnFromCoreVaultRequest(Agent.State), arguments:["agent_1 (-> ['TMP_2559'])"] 
 remintedUBA = Conversion.convertAmgToUBA(remintedAMG)
TMP_2585(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['remintedAMG_1'] 
remintedUBA_1(uint256) := TMP_2585(uint256)
 ReturnFromCoreVaultConfirmed(_agentVault,requestId,receivedAmountUBA,remintedUBA)
Emit ReturnFromCoreVaultConfirmed(_agentVault_1,requestId_1,receivedAmountUBA_1,remintedUBA_1)
 onlyEnabled()
MODIFIER_CALL, CoreVaultClientFacet.onlyEnabled()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
```
#### CoreVaultClientFacet.constructor() [PUBLIC]
```slithir
 state = CoreVaultClient.getState()
TMP_2481(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2481'])(CoreVaultClient.State) := TMP_2481(CoreVaultClient.State)
 state.initialized = true
REF_1380(bool) -> state_1 (-> ['TMP_2481']).initialized
state_2 (-> ['TMP_2481'])(CoreVaultClient.State) := phi(["state_1 (-> ['TMP_2481'])"])
REF_1380(bool) (->state_2 (-> ['TMP_2481'])) := True(bool)
TMP_2481(CoreVaultClient.State) := phi(["state_2 (-> ['TMP_2481'])"])
```
#### ICoreVaultClient.coreVaultAvailableAmount() [EXTERNAL]
```slithir

```
#### CoreVaultClientFacet.maximumTransferToCoreVault(address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_2611(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2611'])(Agent.State) := TMP_2611(Agent.State)
 (_maximumTransferAMG,_minimumLeftAmountAMG) = CoreVaultClient.maximumTransferToCoreVaultAMG(agent)
TUPLE_27(uint256,uint256) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.maximumTransferToCoreVaultAMG(Agent.State), arguments:["agent_1 (-> ['TMP_2611'])"] 
_maximumTransferAMG_1(uint256)= UNPACK TUPLE_27 index: 0 
_minimumLeftAmountAMG_1(uint256)= UNPACK TUPLE_27 index: 1 
 _maximumTransferUBA = Conversion.convertAmgToUBA(_maximumTransferAMG.toUint64())
TMP_2612(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_maximumTransferAMG_1'] 
TMP_2613(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['TMP_2612'] 
_maximumTransferUBA_1(uint256) := TMP_2613(uint256)
 _minimumLeftAmountUBA = Conversion.convertAmgToUBA(_minimumLeftAmountAMG.toUint64())
TMP_2614(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_minimumLeftAmountAMG_1'] 
TMP_2615(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['TMP_2614'] 
_minimumLeftAmountUBA_1(uint256) := TMP_2615(uint256)
 (_maximumTransferUBA,_minimumLeftAmountUBA)
RETURN _maximumTransferUBA_1,_minimumLeftAmountUBA_1
```
#### CoreVaultClientFacet.redeemFromCoreVault(uint256,string) [EXTERNAL]
```slithir
 state = CoreVaultClient.getState()
TMP_2590(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2590'])(CoreVaultClient.State) := TMP_2590(CoreVaultClient.State)
 availableLots = CoreVaultClient.coreVaultAmountLots()
TMP_2591(uint256) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.coreVaultAmountLots(), arguments:[] 
availableLots_1(uint256) := TMP_2591(uint256)
 require(bool,error)(_lots <= availableLots,revert NotEnoughAvailableOnCoreVault()())
TMP_2592(bool) = _lots_1 <= availableLots_1
TMP_2593(None) = SOLIDITY_CALL revert NotEnoughAvailableOnCoreVault()()
TMP_2594(None) = SOLIDITY_CALL require(bool,error)(TMP_2592,TMP_2593)
 minimumRedeemLots = Math.min(state.minimumRedeemLots,availableLots)
REF_1463(uint64) -> state_1 (-> ['TMP_2590']).minimumRedeemLots
TMP_2595(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['REF_1463', 'availableLots_1'] 
minimumRedeemLots_1(uint256) := TMP_2595(uint256)
 require(bool,error)(_lots >= minimumRedeemLots,revert RequestedAmountTooSmall()())
TMP_2596(bool) = _lots_1 >= minimumRedeemLots_1
TMP_2597(None) = SOLIDITY_CALL revert RequestedAmountTooSmall()()
TMP_2598(None) = SOLIDITY_CALL require(bool,error)(TMP_2596,TMP_2597)
 redeemedUBA = Conversion.convertLotsToUBA(_lots)
TMP_2599(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertLotsToUBA(uint256), arguments:['_lots_1'] 
redeemedUBA_1(uint256) := TMP_2599(uint256)
 Redemptions.burnFAssets(msg.sender,redeemedUBA)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.burnFAssets(address,uint256), arguments:['msg.sender', 'redeemedUBA_1'] 
 redemptionFeeUBA = redeemedUBA.mulBips(state.redemptionFeeBIPS)
REF_1467(uint16) -> state_1 (-> ['TMP_2590']).redemptionFeeBIPS
TMP_2601(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['redeemedUBA_1', 'REF_1467'] 
redemptionFeeUBA_1(uint256) := TMP_2601(uint256)
 paymentUBA = (redeemedUBA - redemptionFeeUBA).toUint128()
TMP_2602(uint256) = redeemedUBA_1 (c)- redemptionFeeUBA_1
TMP_2603(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['TMP_2602'] 
paymentUBA_1(uint128) := TMP_2603(uint128)
 state.newRedemptionFromCoreVaultId += PaymentReference.randomizedIdSkip()
REF_1469(uint64) -> state_1 (-> ['TMP_2590']).newRedemptionFromCoreVaultId
TMP_2604(uint64) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.randomizedIdSkip(), arguments:[] 
state_2 (-> ['TMP_2590'])(CoreVaultClient.State) := phi(["state_1 (-> ['TMP_2590'])"])
REF_1469(-> state_2 (-> ['TMP_2590'])) = REF_1469 (c)+ TMP_2604
TMP_2590(CoreVaultClient.State) := phi(["state_2 (-> ['TMP_2590'])"])
 paymentReference = PaymentReference.redemptionFromCoreVault(state.newRedemptionFromCoreVaultId)
REF_1472(uint64) -> state_2 (-> ['TMP_2590']).newRedemptionFromCoreVaultId
TMP_2605(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.redemptionFromCoreVault(uint256), arguments:['REF_1472'] 
paymentReference_1(bytes32) := TMP_2605(bytes32)
 paymentReference = state.coreVaultManager.requestTransferFromCoreVault(_redeemerUnderlyingAddress,paymentReference,paymentUBA,false)
REF_1473(IICoreVaultManager) -> state_2 (-> ['TMP_2590']).coreVaultManager
TMP_2606(bytes32) = HIGH_LEVEL_CALL, dest:REF_1473(IICoreVaultManager), function:requestTransferFromCoreVault, arguments:['_redeemerUnderlyingAddress_1', 'paymentReference_1', 'paymentUBA_1', 'False']  
paymentReference_2(bytes32) := TMP_2606(bytes32)
 CoreVaultRedemptionRequested(msg.sender,_redeemerUnderlyingAddress,paymentReference,redeemedUBA,redemptionFeeUBA)
Emit CoreVaultRedemptionRequested(msg.sender,_redeemerUnderlyingAddress_1,paymentReference_2,redeemedUBA_1,redemptionFeeUBA_1)
 onlyEnabled()
MODIFIER_CALL, CoreVaultClientFacet.onlyEnabled()()
 notEmergencyPaused()
MODIFIER_CALL, AssetManagerBase.notEmergencyPaused()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### CoreVaultClientFacet.requestReturnFromCoreVault(address,uint256) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_2515(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2515'])(Agent.State) := TMP_2515(Agent.State)
 state = CoreVaultClient.getState()
TMP_2516(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2516'])(CoreVaultClient.State) := TMP_2516(CoreVaultClient.State)
 require(bool,error)(agent.activeReturnFromCoreVaultId == 0,revert ReturnFromCoreVaultAlreadyRequested()())
REF_1401(uint64) -> agent_1 (-> ['TMP_2515']).activeReturnFromCoreVaultId
TMP_2517(bool) = REF_1401 == 0
TMP_2518(None) = SOLIDITY_CALL revert ReturnFromCoreVaultAlreadyRequested()()
TMP_2519(None) = SOLIDITY_CALL require(bool,error)(TMP_2517,TMP_2518)
 collateralData = AgentCollateral.combinedData(agent)
TMP_2520(Collateral.CombinedData) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.combinedData(Agent.State), arguments:["agent_1 (-> ['TMP_2515'])"] 
collateralData_1(Collateral.CombinedData) := TMP_2520(Collateral.CombinedData)
 require(bool,error)(_lots > 0,revert CannotReturnZeroLots()())
TMP_2521(bool) = _lots_1 > 0
TMP_2522(None) = SOLIDITY_CALL revert CannotReturnZeroLots()()
TMP_2523(None) = SOLIDITY_CALL require(bool,error)(TMP_2521,TMP_2522)
 require(bool,error)(agent.status == Agent.Status.NORMAL,revert InvalidAgentStatus()())
REF_1403(Agent.Status) -> agent_1 (-> ['TMP_2515']).status
REF_1404(Agent.Status) -> Status.NORMAL
TMP_2524(bool) = REF_1403 == REF_1404
TMP_2525(None) = SOLIDITY_CALL revert InvalidAgentStatus()()
TMP_2526(None) = SOLIDITY_CALL require(bool,error)(TMP_2524,TMP_2525)
 require(bool,error)(collateralData.freeCollateralLotsOptionalFee(agent,false) >= _lots,revert NotEnoughFreeCollateral()())
TMP_2527(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.freeCollateralLotsOptionalFee(Collateral.CombinedData,Agent.State,bool), arguments:['collateralData_1', "agent_1 (-> ['TMP_2515'])", 'False'] 
TMP_2528(bool) = TMP_2527 >= _lots_1
TMP_2529(None) = SOLIDITY_CALL revert NotEnoughFreeCollateral()()
TMP_2530(None) = SOLIDITY_CALL require(bool,error)(TMP_2528,TMP_2529)
 availableLots = CoreVaultClient.coreVaultAmountLots()
TMP_2531(uint256) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.coreVaultAmountLots(), arguments:[] 
availableLots_1(uint256) := TMP_2531(uint256)
 require(bool,error)(_lots <= availableLots,revert NotEnoughAvailableOnCoreVault()())
TMP_2532(bool) = _lots_1 <= availableLots_1
TMP_2533(None) = SOLIDITY_CALL revert NotEnoughAvailableOnCoreVault()()
TMP_2534(None) = SOLIDITY_CALL require(bool,error)(TMP_2532,TMP_2533)
 state.newTransferFromCoreVaultId += PaymentReference.randomizedIdSkip()
REF_1407(uint64) -> state_1 (-> ['TMP_2516']).newTransferFromCoreVaultId
TMP_2535(uint64) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.randomizedIdSkip(), arguments:[] 
state_2 (-> ['TMP_2516'])(CoreVaultClient.State) := phi(["state_1 (-> ['TMP_2516'])"])
REF_1407(-> state_2 (-> ['TMP_2516'])) = REF_1407 (c)+ TMP_2535
TMP_2516(CoreVaultClient.State) := phi(["state_2 (-> ['TMP_2516'])"])
 requestId = state.newTransferFromCoreVaultId
REF_1409(uint64) -> state_2 (-> ['TMP_2516']).newTransferFromCoreVaultId
requestId_1(uint64) := REF_1409(uint64)
 agent.activeReturnFromCoreVaultId = requestId
REF_1410(uint64) -> agent_1 (-> ['TMP_2515']).activeReturnFromCoreVaultId
agent_2 (-> ['TMP_2515'])(Agent.State) := phi(["agent_1 (-> ['TMP_2515'])"])
REF_1410(uint64) (->agent_2 (-> ['TMP_2515'])) := requestId_1(uint64)
TMP_2515(Agent.State) := phi(["agent_2 (-> ['TMP_2515'])"])
 assert(bool)(agent.returnFromCoreVaultReservedAMG == 0)
REF_1411(uint64) -> agent_2 (-> ['TMP_2515']).returnFromCoreVaultReservedAMG
TMP_2536(bool) = REF_1411 == 0
TMP_2537(None) = SOLIDITY_CALL assert(bool)(TMP_2536)
 amountAMG = Conversion.convertLotsToAMG(_lots)
TMP_2538(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertLotsToAMG(uint256), arguments:['_lots_1'] 
amountAMG_1(uint64) := TMP_2538(uint64)
 agent.returnFromCoreVaultReservedAMG = amountAMG
REF_1413(uint64) -> agent_2 (-> ['TMP_2515']).returnFromCoreVaultReservedAMG
agent_3 (-> ['TMP_2515'])(Agent.State) := phi(["agent_2 (-> ['TMP_2515'])"])
REF_1413(uint64) (->agent_3 (-> ['TMP_2515'])) := amountAMG_1(uint64)
TMP_2515(Agent.State) := phi(["agent_3 (-> ['TMP_2515'])"])
 agent.reservedAMG += amountAMG
REF_1414(uint64) -> agent_3 (-> ['TMP_2515']).reservedAMG
agent_4 (-> ['TMP_2515'])(Agent.State) := phi(["agent_3 (-> ['TMP_2515'])"])
REF_1414(-> agent_4 (-> ['TMP_2515'])) = REF_1414 (c)+ amountAMG_1
TMP_2515(Agent.State) := phi(["agent_4 (-> ['TMP_2515'])"])
 paymentReference = PaymentReference.returnFromCoreVault(requestId)
TMP_2539(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.returnFromCoreVault(uint256), arguments:['requestId_1'] 
paymentReference_1(bytes32) := TMP_2539(bytes32)
 amountUBA = Conversion.convertAmgToUBA(amountAMG).toUint128()
TMP_2540(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['amountAMG_1'] 
TMP_2541(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['TMP_2540'] 
amountUBA_1(uint128) := TMP_2541(uint128)
 state.coreVaultManager.requestTransferFromCoreVault(agent.underlyingAddressString,paymentReference,amountUBA,true)
REF_1418(IICoreVaultManager) -> state_2 (-> ['TMP_2516']).coreVaultManager
REF_1420(string) -> agent_4 (-> ['TMP_2515']).underlyingAddressString
TMP_2542(bytes32) = HIGH_LEVEL_CALL, dest:REF_1418(IICoreVaultManager), function:requestTransferFromCoreVault, arguments:['REF_1420', 'paymentReference_1', 'amountUBA_1', 'True']  
 ReturnFromCoreVaultRequested(_agentVault,requestId,paymentReference,amountUBA)
Emit ReturnFromCoreVaultRequested(_agentVault_1,requestId_1,paymentReference_1,amountUBA_1)
 onlyEnabled()
MODIFIER_CALL, CoreVaultClientFacet.onlyEnabled()()
 notEmergencyPaused()
MODIFIER_CALL, AssetManagerBase.notEmergencyPaused()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
```
#### CoreVaultClientFacet.transferToCoreVault(address,uint256) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_2482(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_2482'])(Agent.State) := TMP_2482(Agent.State)
 state = CoreVaultClient.getState()
TMP_2483(CoreVaultClient.State) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.getState(), arguments:[] 
state_1 (-> ['TMP_2483'])(CoreVaultClient.State) := TMP_2483(CoreVaultClient.State)
 require(bool,error)(agent.status != Agent.Status.FULL_LIQUIDATION,revert InvalidAgentStatus()())
REF_1383(Agent.Status) -> agent_1 (-> ['TMP_2482']).status
REF_1384(Agent.Status) -> Status.FULL_LIQUIDATION
TMP_2484(bool) = REF_1383 != REF_1384
TMP_2485(None) = SOLIDITY_CALL revert InvalidAgentStatus()()
TMP_2486(None) = SOLIDITY_CALL require(bool,error)(TMP_2484,TMP_2485)
 require(bool,error)(_amountUBA > 0,revert ZeroTransferNotAllowed()())
TMP_2487(bool) = _amountUBA_1 > 0
TMP_2488(None) = SOLIDITY_CALL revert ZeroTransferNotAllowed()()
TMP_2489(None) = SOLIDITY_CALL require(bool,error)(TMP_2487,TMP_2488)
 require(bool,error)(_amountUBA.toInt256() <= agent.underlyingBalanceUBA,revert NotEnoughUnderlying()())
TMP_2490(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['_amountUBA_1'] 
REF_1386(int128) -> agent_1 (-> ['TMP_2482']).underlyingBalanceUBA
TMP_2491(bool) = TMP_2490 <= REF_1386
TMP_2492(None) = SOLIDITY_CALL revert NotEnoughUnderlying()()
TMP_2493(None) = SOLIDITY_CALL require(bool,error)(TMP_2491,TMP_2492)
 require(bool,error)(agent.activeTransferToCoreVault == 0,revert TransferAlreadyActive()())
REF_1387(uint64) -> agent_1 (-> ['TMP_2482']).activeTransferToCoreVault
TMP_2494(bool) = REF_1387 == 0
TMP_2495(None) = SOLIDITY_CALL revert TransferAlreadyActive()()
TMP_2496(None) = SOLIDITY_CALL require(bool,error)(TMP_2494,TMP_2495)
 amountAMG = Conversion.convertUBAToAmg(_amountUBA)
TMP_2497(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['_amountUBA_1'] 
amountAMG_1(uint64) := TMP_2497(uint64)
 (transferredAMG,None) = Redemptions.closeTickets(agent,amountAMG,false)
TUPLE_25(uint64,uint256) = LIBRARY_CALL, dest:Redemptions, function:Redemptions.closeTickets(Agent.State,uint64,bool), arguments:["agent_1 (-> ['TMP_2482'])", 'amountAMG_1', 'False'] 
transferredAMG_1(uint64)= UNPACK TUPLE_25 index: 0 
 require(bool,error)(transferredAMG > 0,revert NothingMinted()())
TMP_2498(bool) = transferredAMG_1 > 0
TMP_2499(None) = SOLIDITY_CALL revert NothingMinted()()
TMP_2500(None) = SOLIDITY_CALL require(bool,error)(TMP_2498,TMP_2499)
 (maximumTransferAMG,None) = CoreVaultClient.maximumTransferToCoreVaultAMG(agent)
TUPLE_26(uint256,uint256) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.maximumTransferToCoreVaultAMG(Agent.State), arguments:["agent_1 (-> ['TMP_2482'])"] 
maximumTransferAMG_1(uint256)= UNPACK TUPLE_26 index: 0 
 require(bool,error)(transferredAMG <= maximumTransferAMG,revert TooLittleMintingLeftAfterTransfer()())
TMP_2501(bool) = transferredAMG_1 <= maximumTransferAMG_1
TMP_2502(None) = SOLIDITY_CALL revert TooLittleMintingLeftAfterTransfer()()
TMP_2503(None) = SOLIDITY_CALL require(bool,error)(TMP_2501,TMP_2502)
 underlyingAddress = state.coreVaultManager.coreVaultAddress()
REF_1391(IICoreVaultManager) -> state_1 (-> ['TMP_2483']).coreVaultManager
TMP_2504(string) = HIGH_LEVEL_CALL, dest:REF_1391(IICoreVaultManager), function:coreVaultAddress, arguments:[]  
underlyingAddress_1(string) := TMP_2504(string)
 redemptionRequestId = RedemptionRequests.createRedemptionRequest(RedemptionRequests.AgentRedemptionData(_agentVault,transferredAMG),state.nativeAddress,underlyingAddress,false,address(address(0)),0,state.transferTimeExtensionSeconds,true)
TMP_2505(RedemptionRequests.AgentRedemptionData) = new AgentRedemptionData(_agentVault_1,transferredAMG_1)
REF_1395(address) -> state_1 (-> ['TMP_2483']).nativeAddress
TMP_2506 = CONVERT 0 to address
TMP_2507 = CONVERT TMP_2506 to address
REF_1396(uint64) -> state_1 (-> ['TMP_2483']).transferTimeExtensionSeconds
TMP_2508(uint64) = LIBRARY_CALL, dest:RedemptionRequests, function:RedemptionRequests.createRedemptionRequest(RedemptionRequests.AgentRedemptionData,address,string,bool,address,uint64,uint64,bool), arguments:['TMP_2505', 'REF_1395', 'underlyingAddress_1', 'False', 'TMP_2507', '0', 'REF_1396', 'True'] 
redemptionRequestId_1(uint64) := TMP_2508(uint64)
 agent.activeTransferToCoreVault = redemptionRequestId
REF_1397(uint64) -> agent_1 (-> ['TMP_2482']).activeTransferToCoreVault
agent_2 (-> ['TMP_2482'])(Agent.State) := phi(["agent_1 (-> ['TMP_2482'])"])
REF_1397(uint64) (->agent_2 (-> ['TMP_2482'])) := redemptionRequestId_1(uint64)
TMP_2482(Agent.State) := phi(["agent_2 (-> ['TMP_2482'])"])
 transferredUBA = Conversion.convertAmgToUBA(transferredAMG)
TMP_2509(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['transferredAMG_1'] 
transferredUBA_1(uint256) := TMP_2509(uint256)
 TransferToCoreVaultStarted(_agentVault,redemptionRequestId,transferredUBA)
Emit TransferToCoreVaultStarted(_agentVault_1,redemptionRequestId_1,transferredUBA_1)
 onlyEnabled()
MODIFIER_CALL, CoreVaultClientFacet.onlyEnabled()()
 notEmergencyPaused()
MODIFIER_CALL, AssetManagerBase.notEmergencyPaused()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
```
#### CoreVaultClient.deleteReturnFromCoreVaultRequest(Agent.State) [INTERNAL]
```slithir
 assert(bool)(_agent.activeReturnFromCoreVaultId != 0 && _agent.returnFromCoreVaultReservedAMG != 0)
REF_3190(uint64) -> _agent_1 (-> []).activeReturnFromCoreVaultId
TMP_4688(bool) = REF_3190 != 0
REF_3191(uint64) -> _agent_1 (-> []).returnFromCoreVaultReservedAMG
TMP_4689(bool) = REF_3191 != 0
TMP_4690(bool) = TMP_4688 && TMP_4689
TMP_4691(None) = SOLIDITY_CALL assert(bool)(TMP_4690)
 _agent.reservedAMG -= _agent.returnFromCoreVaultReservedAMG
REF_3192(uint64) -> _agent_1 (-> []).reservedAMG
REF_3193(uint64) -> _agent_1 (-> []).returnFromCoreVaultReservedAMG
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_3192(-> _agent_2 (-> [])) = REF_3192 (c)- REF_3193
 _agent.activeReturnFromCoreVaultId = 0
REF_3194(uint64) -> _agent_2 (-> []).activeReturnFromCoreVaultId
_agent_3 (-> [])(Agent.State) := phi(['_agent_2 (-> [])'])
REF_3194(uint64) (->_agent_3 (-> [])) := 0(uint256)
 _agent.returnFromCoreVaultReservedAMG = 0
REF_3195(uint64) -> _agent_3 (-> []).returnFromCoreVaultReservedAMG
_agent_4 (-> [])(Agent.State) := phi(['_agent_3 (-> [])'])
REF_3195(uint64) (->_agent_4 (-> [])) := 0(uint256)
```
#### CoreVaultClient.getState() [INTERNAL]
```slithir
STATE_POSITION_1(bytes32) := phi(['STATE_POSITION_0'])
 position = STATE_POSITION
position_1(bytes32) := STATE_POSITION_1(bytes32)
 _state = position
_state_1 (-> ['position'])(CoreVaultClient.State) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
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
#### CoreVaultManager.cancelTransferRequestFromCoreVault(string) [EXTERNAL]
```slithir
cancelableTransferRequests_6(uint256[]) := phi(['cancelableTransferRequests_17', 'cancelableTransferRequests_13', 'cancelableTransferRequests_18', 'cancelableTransferRequests_0', 'cancelableTransferRequests_9', 'cancelableTransferRequests_3', 'cancelableTransferRequests_5'])
transferRequestById_6(mapping(uint256 => ICoreVaultManager.TransferRequest)) := phi(['transferRequestById_3', 'transferRequestById_8', 'transferRequestById_15', 'transferRequestById_12', 'transferRequestById_0', 'transferRequestById_11', 'transferRequestById_13', 'transferRequestById_14', 'transferRequestById_4', 'transferRequestById_5'])
cancelableTransferRequestsAmount_5(uint128) := phi(['cancelableTransferRequestsAmount_11', 'cancelableTransferRequestsAmount_4', 'cancelableTransferRequestsAmount_0', 'cancelableTransferRequestsAmount_7', 'cancelableTransferRequestsAmount_3'])
 destinationAddressHash = keccak256(bytes)(bytes(_destinationAddress))
TMP_6892 = CONVERT _destinationAddress_1 to bytes
TMP_6893(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_6892)
destinationAddressHash_1(bytes32) := TMP_6893(bytes32)
 index = 0
index_1(uint256) := 0(uint256)
 index < cancelableTransferRequests.length
index_2(uint256) := phi(['index_5', 'index_1'])
REF_4491 -> LENGTH cancelableTransferRequests_7
TMP_6894(bool) = index_2 < REF_4491
CONDITION TMP_6894
 destAddress = transferRequestById[cancelableTransferRequests[index]].destinationAddress
REF_4492(uint256) -> cancelableTransferRequests_7[index_2]
REF_4493(ICoreVaultManager.TransferRequest) -> transferRequestById_7[REF_4492]
REF_4494(string) -> REF_4493.destinationAddress
destAddress_1(string) := REF_4494(string)
 keccak256(bytes)(bytes(destAddress)) == destinationAddressHash
TMP_6895 = CONVERT destAddress_1 to bytes
TMP_6896(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_6895)
TMP_6897(bool) = TMP_6896 == destinationAddressHash_1
CONDITION TMP_6897
 index ++
TMP_6898(uint256) := index_2(uint256)
index_5(uint256) = index_2 (c)+ 1
 require(bool,error)(index < cancelableTransferRequests.length,revert NotFound()())
REF_4495 -> LENGTH cancelableTransferRequests_7
TMP_6899(bool) = index_2 < REF_4495
TMP_6900(None) = SOLIDITY_CALL revert NotFound()()
TMP_6901(None) = SOLIDITY_CALL require(bool,error)(TMP_6899,TMP_6900)
 transferRequestId = cancelableTransferRequests[index]
REF_4496(uint256) -> cancelableTransferRequests_7[index_2]
transferRequestId_1(uint256) := REF_4496(uint256)
 req = transferRequestById[transferRequestId]
REF_4497(ICoreVaultManager.TransferRequest) -> transferRequestById_7[transferRequestId_1]
req_1 (-> ['transferRequestById'])(ICoreVaultManager.TransferRequest) := REF_4497(ICoreVaultManager.TransferRequest)
 amount = req.amount
REF_4498(uint128) -> req_1 (-> ['transferRequestById']).amount
amount_1(uint128) := REF_4498(uint128)
 cancelableTransferRequestsAmount -= amount
cancelableTransferRequestsAmount_7(uint128) = cancelableTransferRequestsAmount_6 (c)- amount_1
 TransferRequestCanceled(_destinationAddress,req.paymentReference,amount)
REF_4499(bytes32) -> req_1 (-> ['transferRequestById']).paymentReference
Emit TransferRequestCanceled(_destinationAddress_1,REF_4499,amount_1)
 index < cancelableTransferRequests.length - 1
index_3(uint256) := phi(['index_4', 'index_1'])
REF_4500 -> LENGTH cancelableTransferRequests_7
TMP_6903(uint256) = REF_4500 (c)- 1
TMP_6904(bool) = index_3 < TMP_6903
CONDITION TMP_6904
 cancelableTransferRequests[index] = cancelableTransferRequests[index + 1]
REF_4501(uint256) -> cancelableTransferRequests_7[index_3]
TMP_6905(uint256) = index_3 (c)+ 1
REF_4502(uint256) -> cancelableTransferRequests_7[TMP_6905]
cancelableTransferRequests_10(uint256[]) := phi(['cancelableTransferRequests_7'])
REF_4501(uint256) (->cancelableTransferRequests_10) := REF_4502(uint256)
 index ++
TMP_6906(uint256) := index_3(uint256)
index_4(uint256) = index_3 (c)+ 1
 cancelableTransferRequests.pop()
REF_4504 -> LENGTH cancelableTransferRequests_7
TMP_6908(uint256) = REF_4504 (c)- 1
REF_4505(uint256) -> cancelableTransferRequests_7[TMP_6908]
cancelableTransferRequests_8 = delete REF_4505 
REF_4506 -> LENGTH cancelableTransferRequests_8
cancelableTransferRequests_9(uint256[]) := phi(['cancelableTransferRequests_8'])
REF_4506(uint256) (->cancelableTransferRequests_9) := TMP_6908(uint256)
 delete transferRequestById[transferRequestId]
REF_4507(ICoreVaultManager.TransferRequest) -> transferRequestById_7[transferRequestId_1]
transferRequestById_8 = delete REF_4507 
 onlyAssetManager()
MODIFIER_CALL, CoreVaultManager.onlyAssetManager()()
```
#### AgentBacking.createNewMinting(Agent.State,uint64) [INTERNAL]
```slithir
 _agent.mintedAMG += _valueAMG
REF_2789(uint64) -> _agent_1 (-> []).mintedAMG
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2789(-> _agent_2 (-> [])) = REF_2789 (c)+ _valueAMG_1
 settings = Globals.getSettings()
TMP_4273(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4273'])(AssetManagerSettings.Data) := TMP_4273(AssetManagerSettings.Data)
 valueWithDustAMG = _agent.dustAMG + _valueAMG
REF_2791(uint64) -> _agent_2 (-> []).dustAMG
TMP_4274(uint64) = REF_2791 (c)+ _valueAMG_1
valueWithDustAMG_1(uint64) := TMP_4274(uint64)
 newDustAMG = valueWithDustAMG % settings.lotSizeAMG
REF_2792(uint64) -> settings_1 (-> ['TMP_4273']).lotSizeAMG
TMP_4275(uint64) = valueWithDustAMG_1 % REF_2792
newDustAMG_1(uint64) := TMP_4275(uint64)
 ticketValueAMG = valueWithDustAMG - newDustAMG
TMP_4276(uint64) = valueWithDustAMG_1 (c)- newDustAMG_1
ticketValueAMG_1(uint64) := TMP_4276(uint64)
 ticketValueAMG > 0
TMP_4277(bool) = ticketValueAMG_1 > 0
CONDITION TMP_4277
 createRedemptionTicket(_agent,ticketValueAMG)
INTERNAL_CALL, AgentBacking.createRedemptionTicket(Agent.State,uint64)(_agent_2 (-> []),ticketValueAMG_1)
 changeDust(_agent,newDustAMG)
INTERNAL_CALL, AgentBacking.changeDust(Agent.State,uint64)(_agent_2 (-> []),newDustAMG_1)
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
#### TransactionAttestation.verifyPaymentSuccess(IPayment.Proof) [INTERNAL]
```slithir
PAYMENT_SUCCESS_1(uint8) := phi(['PAYMENT_SUCCESS_0'])
 require(bool,error)(_proof.data.responseBody.status == PAYMENT_SUCCESS,revert PaymentFailed()())
REF_3666(IPayment.Response) -> _proof_1.data
REF_3667(IPayment.ResponseBody) -> REF_3666.responseBody
REF_3668(uint8) -> REF_3667.status
TMP_5233(bool) = REF_3668 == PAYMENT_SUCCESS_1
TMP_5234(None) = SOLIDITY_CALL revert PaymentFailed()()
TMP_5235(None) = SOLIDITY_CALL require(bool,error)(TMP_5233,TMP_5234)
 verifyPayment(_proof)
INTERNAL_CALL, TransactionAttestation.verifyPayment(IPayment.Proof)(_proof_1)
```
#### UnderlyingBalance.increaseBalance(Agent.State,uint256) [INTERNAL]
```slithir
 _agent.underlyingBalanceUBA += _balanceIncrease.toInt256().toInt128()
REF_3709(int128) -> _agent_1 (-> []).underlyingBalanceUBA
TMP_5287(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['_balanceIncrease_1'] 
TMP_5288(int128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt128(int256), arguments:['TMP_5287'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_3709(-> _agent_2 (-> [])) = REF_3709 (c)+ TMP_5288
 IAssetManagerEvents.UnderlyingBalanceChanged(_agent.vaultAddress(),_agent.underlyingBalanceUBA)
TMP_5289(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_2 (-> [])'] 
REF_3714(int128) -> _agent_2 (-> []).underlyingBalanceUBA
Emit UnderlyingBalanceChanged(TMP_5289,REF_3714)
```
#### UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(IPayment.Proof) [INTERNAL]
```slithir
 updateCurrentBlock(_proof.data.responseBody.blockNumber,_proof.data.responseBody.blockTimestamp,1)
REF_3729(IPayment.Response) -> _proof_1.data
REF_3730(IPayment.ResponseBody) -> REF_3729.responseBody
REF_3731(uint64) -> REF_3730.blockNumber
REF_3732(IPayment.Response) -> _proof_1.data
REF_3733(IPayment.ResponseBody) -> REF_3732.responseBody
REF_3734(uint64) -> REF_3733.blockTimestamp
INTERNAL_CALL, UnderlyingBlockUpdater.updateCurrentBlock(uint64,uint64,uint64)(REF_3731,REF_3734,1)
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
#### PaymentConfirmations.confirmIncomingPayment(PaymentConfirmations.State,IPayment.Proof) [INTERNAL]
```slithir
 _recordPaymentVerification(_state,_payment.data.requestBody.transactionId)
REF_3753(IPayment.Response) -> _payment_1.data
REF_3754(IPayment.RequestBody) -> REF_3753.requestBody
REF_3755(bytes32) -> REF_3754.transactionId
INTERNAL_CALL, PaymentConfirmations._recordPaymentVerification(PaymentConfirmations.State,bytes32)(_state_1 (-> []),REF_3755)
```
#### PaymentReference.returnFromCoreVault(uint256) [INTERNAL]
```slithir
MAX_ID_4(uint256) := phi(['MAX_ID_0'])
RETURN_FROM_CORE_VAULT_1(uint256) := phi(['RETURN_FROM_CORE_VAULT_0'])
 assert(bool)(_id <= MAX_ID)
TMP_5357(bool) = _id_1 <= MAX_ID_4
TMP_5358(None) = SOLIDITY_CALL assert(bool)(TMP_5357)
 bytes32(_id | RETURN_FROM_CORE_VAULT)
TMP_5359(uint256) = _id_1 | RETURN_FROM_CORE_VAULT_1
TMP_5360 = CONVERT TMP_5359 to bytes32
RETURN TMP_5360
```
#### SafeMath64.min64(uint64,uint64) [INTERNAL]
```slithir
 a <= b
TMP_10505(bool) = a_1 <= b_1
CONDITION TMP_10505
 a
RETURN a_1
 b
RETURN b_1
```
#### SafeCast.toUint256(int256) [INTERNAL]
```slithir
 require(bool,string)(value >= 0,SafeCast: value must be positive)
TMP_786(bool) = value_1 >= 0
TMP_787(None) = SOLIDITY_CALL require(bool,string)(TMP_786,SafeCast: value must be positive)
 uint256(value)
TMP_788 = CONVERT value_1 to uint256
RETURN TMP_788
```
#### CoreVaultClient.coreVaultAvailableAmount() [INTERNAL]
```slithir
 state = getState()
TMP_4694(CoreVaultClient.State) = INTERNAL_CALL, CoreVaultClient.getState()()
state_1 (-> ['TMP_4694'])(CoreVaultClient.State) := TMP_4694(CoreVaultClient.State)
 availableFunds = state.coreVaultManager.availableFunds()
REF_3198(IICoreVaultManager) -> state_1 (-> ['TMP_4694']).coreVaultManager
TMP_4695(uint128) = HIGH_LEVEL_CALL, dest:REF_3198(IICoreVaultManager), function:availableFunds, arguments:[]  
availableFunds_1(uint256) := TMP_4695(uint128)
 escrowedFunds = state.coreVaultManager.escrowedFunds()
REF_3200(IICoreVaultManager) -> state_1 (-> ['TMP_4694']).coreVaultManager
TMP_4696(uint128) = HIGH_LEVEL_CALL, dest:REF_3200(IICoreVaultManager), function:escrowedFunds, arguments:[]  
escrowedFunds_1(uint256) := TMP_4696(uint128)
 requestedAmountWithFee = state.coreVaultManager.totalRequestAmountWithFee() + coreVaultUnderlyingPaymentFee()
REF_3202(IICoreVaultManager) -> state_1 (-> ['TMP_4694']).coreVaultManager
TMP_4697(uint256) = HIGH_LEVEL_CALL, dest:REF_3202(IICoreVaultManager), function:totalRequestAmountWithFee, arguments:[]  
TMP_4698(uint256) = INTERNAL_CALL, CoreVaultClient.coreVaultUnderlyingPaymentFee()()
TMP_4699(uint256) = TMP_4697 (c)+ TMP_4698
requestedAmountWithFee_1(uint256) := TMP_4699(uint256)
 _immediatelyAvailableUBA = MathUtils.subOrZero(availableFunds,requestedAmountWithFee)
TMP_4700(uint256) = LIBRARY_CALL, dest:MathUtils, function:MathUtils.subOrZero(uint256,uint256), arguments:['availableFunds_1', 'requestedAmountWithFee_1'] 
_immediatelyAvailableUBA_1(uint256) := TMP_4700(uint256)
 _totalAvailableUBA = MathUtils.subOrZero(availableFunds + escrowedFunds,requestedAmountWithFee)
TMP_4701(uint256) = availableFunds_1 (c)+ escrowedFunds_1
TMP_4702(uint256) = LIBRARY_CALL, dest:MathUtils, function:MathUtils.subOrZero(uint256,uint256), arguments:['TMP_4701', 'requestedAmountWithFee_1'] 
_totalAvailableUBA_1(uint256) := TMP_4702(uint256)
 (_immediatelyAvailableUBA,_totalAvailableUBA)
RETURN _immediatelyAvailableUBA_1,_totalAvailableUBA_1
```
#### CoreVaultClient.maximumTransferToCoreVaultAMG(Agent.State) [INTERNAL]
```slithir
 _minimumLeftAmountAMG = _minimumRemainingAfterTransferAMG(_agent)
TMP_4692(uint256) = INTERNAL_CALL, CoreVaultClient._minimumRemainingAfterTransferAMG(Agent.State)(_agent_1 (-> []))
_minimumLeftAmountAMG_1(uint256) := TMP_4692(uint256)
 _maximumTransferAMG = MathUtils.subOrZero(_agent.mintedAMG,_minimumLeftAmountAMG)
REF_3197(uint64) -> _agent_1 (-> []).mintedAMG
TMP_4693(uint256) = LIBRARY_CALL, dest:MathUtils, function:MathUtils.subOrZero(uint256,uint256), arguments:['REF_3197', '_minimumLeftAmountAMG_1'] 
_maximumTransferAMG_1(uint256) := TMP_4693(uint256)
 (_maximumTransferAMG,_minimumLeftAmountAMG)
RETURN _maximumTransferAMG_1,_minimumLeftAmountAMG_1
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
#### Conversion.convertLotsToUBA(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4649(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4649'])(AssetManagerSettings.Data) := TMP_4649(AssetManagerSettings.Data)
 _lots * settings.lotSizeAMG * settings.assetMintingGranularityUBA
REF_3155(uint64) -> settings_1 (-> ['TMP_4649']).lotSizeAMG
TMP_4650(uint256) = _lots_1 (c)* REF_3155
REF_3156(uint64) -> settings_1 (-> ['TMP_4649']).assetMintingGranularityUBA
TMP_4651(uint256) = TMP_4650 (c)* REF_3156
RETURN TMP_4651
```
#### CoreVaultClient.coreVaultAmountLots() [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4703(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4703'])(AssetManagerSettings.Data) := TMP_4703(AssetManagerSettings.Data)
 (None,totalAmountUBA) = coreVaultAvailableAmount()
TUPLE_52(uint256,uint256) = INTERNAL_CALL, CoreVaultClient.coreVaultAvailableAmount()()
totalAmountUBA_1(uint256)= UNPACK TUPLE_52 index: 1 
 Conversion.convertUBAToAmg(totalAmountUBA) / settings.lotSizeAMG
TMP_4704(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['totalAmountUBA_1'] 
REF_3208(uint64) -> settings_1 (-> ['TMP_4703']).lotSizeAMG
TMP_4705(uint64) = TMP_4704 (c)/ REF_3208
RETURN TMP_4705
```
#### Redemptions.burnFAssets(address,uint256) [INTERNAL]
```slithir
 Globals.getFAsset().burn(_owner,_amountUBA)
TMP_5022(IIFAsset) = LIBRARY_CALL, dest:Globals, function:Globals.getFAsset(), arguments:[] 
HIGH_LEVEL_CALL, dest:TMP_5022(IIFAsset), function:burn, arguments:['_owner_1', '_amountUBA_1']
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
#### PaymentReference.redemptionFromCoreVault(uint256) [INTERNAL]
```slithir
MAX_ID_5(uint256) := phi(['MAX_ID_0'])
REDEMPTION_FROM_CORE_VAULT_1(uint256) := phi(['REDEMPTION_FROM_CORE_VAULT_0'])
 assert(bool)(_id <= MAX_ID)
TMP_5361(bool) = _id_1 <= MAX_ID_5
TMP_5362(None) = SOLIDITY_CALL assert(bool)(TMP_5361)
 bytes32(_id | REDEMPTION_FROM_CORE_VAULT)
TMP_5363(uint256) = _id_1 | REDEMPTION_FROM_CORE_VAULT_1
TMP_5364 = CONVERT TMP_5363 to bytes32
RETURN TMP_5364
```
#### CoreVaultManager.requestTransferFromCoreVault(string,bytes32,uint128,bool) [EXTERNAL]
```slithir
nextTransferRequestId_1(uint256) := phi(['nextTransferRequestId_5', 'nextTransferRequestId_0', 'nextTransferRequestId_4'])
cancelableTransferRequests_1(uint256[]) := phi(['cancelableTransferRequests_17', 'cancelableTransferRequests_13', 'cancelableTransferRequests_18', 'cancelableTransferRequests_0', 'cancelableTransferRequests_9', 'cancelableTransferRequests_3', 'cancelableTransferRequests_5'])
nonCancelableTransferRequests_1(uint256[]) := phi(['nonCancelableTransferRequests_5', 'nonCancelableTransferRequests_8', 'nonCancelableTransferRequests_13', 'nonCancelableTransferRequests_0', 'nonCancelableTransferRequests_12', 'nonCancelableTransferRequests_11', 'nonCancelableTransferRequests_3', 'nonCancelableTransferRequests_10'])
transferRequestById_1(mapping(uint256 => ICoreVaultManager.TransferRequest)) := phi(['transferRequestById_3', 'transferRequestById_8', 'transferRequestById_15', 'transferRequestById_12', 'transferRequestById_0', 'transferRequestById_11', 'transferRequestById_13', 'transferRequestById_14', 'transferRequestById_4', 'transferRequestById_5'])
allowedDestinationAddressIndex_1(mapping(string => uint256)) := phi(['allowedDestinationAddressIndex_11', 'allowedDestinationAddressIndex_3', 'allowedDestinationAddressIndex_5', 'allowedDestinationAddressIndex_0', 'allowedDestinationAddressIndex_8'])
availableFunds_4(uint128) := phi(['availableFunds_7', 'availableFunds_12', 'availableFunds_2', 'availableFunds_17', 'availableFunds_0', 'availableFunds_3', 'availableFunds_15', 'availableFunds_11'])
escrowedFunds_1(uint128) := phi(['escrowedFunds_4', 'escrowedFunds_0', 'escrowedFunds_7', 'escrowedFunds_13', 'escrowedFunds_8', 'escrowedFunds_11'])
cancelableTransferRequestsAmount_1(uint128) := phi(['cancelableTransferRequestsAmount_11', 'cancelableTransferRequestsAmount_4', 'cancelableTransferRequestsAmount_0', 'cancelableTransferRequestsAmount_7', 'cancelableTransferRequestsAmount_3'])
nonCancelableTransferRequestsAmount_1(uint128) := phi(['nonCancelableTransferRequestsAmount_3', 'nonCancelableTransferRequestsAmount_4', 'nonCancelableTransferRequestsAmount_8', 'nonCancelableTransferRequestsAmount_0'])
 require(bool,error)(_amount > 0,revert AmountZero()())
TMP_6855(bool) = _amount_1 > 0
TMP_6856(None) = SOLIDITY_CALL revert AmountZero()()
TMP_6857(None) = SOLIDITY_CALL require(bool,error)(TMP_6855,TMP_6856)
 require(bool,error)(allowedDestinationAddressIndex[_destinationAddress] != 0,revert DestinationNotAllowed()())
REF_4472(uint256) -> allowedDestinationAddressIndex_3[_destinationAddress_1]
TMP_6858(bool) = REF_4472 != 0
TMP_6859(None) = SOLIDITY_CALL revert DestinationNotAllowed()()
TMP_6860(None) = SOLIDITY_CALL require(bool,error)(TMP_6858,TMP_6859)
 destinationAddressHash = keccak256(bytes)(bytes(_destinationAddress))
TMP_6861 = CONVERT _destinationAddress_1 to bytes
TMP_6862(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_6861)
destinationAddressHash_1(bytes32) := TMP_6862(bytes32)
 newTransferRequest = false
newTransferRequest_1(bool) := False(bool)
 _cancelable
CONDITION _cancelable_1
 i = 0
i_1(uint256) := 0(uint256)
 i < cancelableTransferRequests.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4473 -> LENGTH cancelableTransferRequests_3
TMP_6863(bool) = i_2 < REF_4473
CONDITION TMP_6863
 req = transferRequestById[cancelableTransferRequests[i]]
REF_4474(uint256) -> cancelableTransferRequests_3[i_2]
REF_4475(ICoreVaultManager.TransferRequest) -> transferRequestById_3[REF_4474]
req_1 (-> ['transferRequestById'])(ICoreVaultManager.TransferRequest) := REF_4475(ICoreVaultManager.TransferRequest)
 require(bool,error)(keccak256(bytes)(bytes(req.destinationAddress)) != destinationAddressHash,revert RequestExists()())
REF_4476(string) -> req_1 (-> ['transferRequestById']).destinationAddress
TMP_6864 = CONVERT REF_4476 to bytes
TMP_6865(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_6864)
TMP_6866(bool) = TMP_6865 != destinationAddressHash_1
TMP_6867(None) = SOLIDITY_CALL revert RequestExists()()
TMP_6868(None) = SOLIDITY_CALL require(bool,error)(TMP_6866,TMP_6867)
 i ++
TMP_6869(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 cancelableTransferRequestsAmount += _amount
cancelableTransferRequestsAmount_4(uint128) = cancelableTransferRequestsAmount_3 (c)+ _amount_1
 cancelableTransferRequests.push(nextTransferRequestId)
REF_4478 -> LENGTH cancelableTransferRequests_3
TMP_6871(uint256) := REF_4478(uint256)
TMP_6872(uint256) = TMP_6871 (c)+ 1
cancelableTransferRequests_4(uint256[]) := phi(['cancelableTransferRequests_3'])
REF_4478(uint256) (->cancelableTransferRequests_4) := TMP_6872(uint256)
REF_4479(uint256) -> cancelableTransferRequests_4[TMP_6871]
cancelableTransferRequests_5(uint256[]) := phi(['cancelableTransferRequests_4'])
REF_4479(uint256) (->cancelableTransferRequests_5) := nextTransferRequestId_3(uint256)
 newTransferRequest = true
newTransferRequest_2(bool) := True(bool)
 index = 0
index_1(uint256) := 0(uint256)
 index < nonCancelableTransferRequests.length
index_2(uint256) := phi(['index_3', 'index_1'])
REF_4480 -> LENGTH nonCancelableTransferRequests_3
TMP_6873(bool) = index_2 < REF_4480
CONDITION TMP_6873
 req_scope_0 = transferRequestById[nonCancelableTransferRequests[index]]
REF_4481(uint256) -> nonCancelableTransferRequests_3[index_2]
REF_4482(ICoreVaultManager.TransferRequest) -> transferRequestById_3[REF_4481]
req_scope_0_1 (-> ['transferRequestById'])(ICoreVaultManager.TransferRequest) := REF_4482(ICoreVaultManager.TransferRequest)
 keccak256(bytes)(bytes(req_scope_0.destinationAddress)) == destinationAddressHash
REF_4483(string) -> req_scope_0_1 (-> ['transferRequestById']).destinationAddress
TMP_6874 = CONVERT REF_4483 to bytes
TMP_6875(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_6874)
TMP_6876(bool) = TMP_6875 == destinationAddressHash_1
CONDITION TMP_6876
 req_scope_0.amount += _amount
REF_4484(uint128) -> req_scope_0_1 (-> ['transferRequestById']).amount
req_scope_0_2 (-> ['transferRequestById'])(ICoreVaultManager.TransferRequest) := phi(["req_scope_0_1 (-> ['transferRequestById'])"])
REF_4484(-> req_scope_0_2 (-> ['transferRequestById'])) = REF_4484 (c)+ _amount_1
transferRequestById_5(mapping(uint256 => ICoreVaultManager.TransferRequest)) := phi(["req_scope_0_2 (-> ['transferRequestById'])"])
 _paymentReference = req_scope_0.paymentReference
REF_4485(bytes32) -> req_scope_0_2 (-> ['transferRequestById']).paymentReference
_paymentReference_2(bytes32) := REF_4485(bytes32)
 index ++
TMP_6877(uint256) := index_2(uint256)
index_3(uint256) = index_2 (c)+ 1
_paymentReference_3(bytes32) := phi(['_paymentReference_1', '_paymentReference_2'])
 nonCancelableTransferRequestsAmount += _amount
nonCancelableTransferRequestsAmount_4(uint128) = nonCancelableTransferRequestsAmount_3 (c)+ _amount_1
 index == nonCancelableTransferRequests.length
REF_4486 -> LENGTH nonCancelableTransferRequests_3
TMP_6878(bool) = index_2 == REF_4486
CONDITION TMP_6878
 nonCancelableTransferRequests.push(nextTransferRequestId)
REF_4488 -> LENGTH nonCancelableTransferRequests_3
TMP_6880(uint256) := REF_4488(uint256)
TMP_6881(uint256) = TMP_6880 (c)+ 1
nonCancelableTransferRequests_4(uint256[]) := phi(['nonCancelableTransferRequests_3'])
REF_4488(uint256) (->nonCancelableTransferRequests_4) := TMP_6881(uint256)
REF_4489(uint256) -> nonCancelableTransferRequests_4[TMP_6880]
nonCancelableTransferRequests_5(uint256[]) := phi(['nonCancelableTransferRequests_4'])
REF_4489(uint256) (->nonCancelableTransferRequests_5) := nextTransferRequestId_3(uint256)
 newTransferRequest = true
newTransferRequest_3(bool) := True(bool)
newTransferRequest_4(bool) := phi(['newTransferRequest_3', 'newTransferRequest_1'])
newTransferRequest_5(bool) := phi(['newTransferRequest_1', 'newTransferRequest_2'])
 requestsAmount = totalRequestAmountWithFee()
TMP_6882(uint256) = INTERNAL_CALL, CoreVaultManager.totalRequestAmountWithFee()()
requestsAmount_1(uint256) := TMP_6882(uint256)
 require(bool,error)(requestsAmount <= availableFunds + escrowedFunds,revert InsufficientFunds()())
TMP_6883(uint128) = availableFunds_7 (c)+ escrowedFunds_4
TMP_6884(bool) = requestsAmount_1 <= TMP_6883
TMP_6885(None) = SOLIDITY_CALL revert InsufficientFunds()()
TMP_6886(None) = SOLIDITY_CALL require(bool,error)(TMP_6884,TMP_6885)
 newTransferRequest
CONDITION newTransferRequest_5
 transferRequestById[nextTransferRequestId ++] = TransferRequest({destinationAddress:_destinationAddress,paymentReference:_paymentReference,amount:_amount})
TMP_6887(uint256) := nextTransferRequestId_4(uint256)
nextTransferRequestId_5(uint256) = nextTransferRequestId_4 (c)+ 1
REF_4490(ICoreVaultManager.TransferRequest) -> transferRequestById_3[TMP_6887]
TMP_6888(ICoreVaultManager.TransferRequest) = new TransferRequest(_destinationAddress_1,_paymentReference_3,_amount_1)
transferRequestById_4(mapping(uint256 => ICoreVaultManager.TransferRequest)) := phi(['transferRequestById_3'])
REF_4490(ICoreVaultManager.TransferRequest) (->transferRequestById_4) := TMP_6888(ICoreVaultManager.TransferRequest)
 TransferRequested(_destinationAddress,_paymentReference,_amount,_cancelable)
Emit TransferRequested(_destinationAddress_1,_paymentReference_3,_amount_1,_cancelable_1)
 _paymentReference
RETURN _paymentReference_3
 onlyAssetManager()
MODIFIER_CALL, CoreVaultManager.onlyAssetManager()()
 notPaused()
MODIFIER_CALL, CoreVaultManager.notPaused()()
```
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
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
#### RedemptionRequests.createRedemptionRequest(RedemptionRequests.AgentRedemptionData,address,string,bool,address,uint64,uint64,bool) [INTERNAL]
```slithir
 require(bool,error)(_executorFeeNatGWei == 0 || _executor != address(0),revert ExecutorFeeWithoutExecutor()())
TMP_4926(bool) = _executorFeeNatGWei_1 == 0
TMP_4927 = CONVERT 0 to address
TMP_4928(bool) = _executor_1 != TMP_4927
TMP_4929(bool) = TMP_4926 || TMP_4928
TMP_4930(None) = SOLIDITY_CALL revert ExecutorFeeWithoutExecutor()()
TMP_4931(None) = SOLIDITY_CALL require(bool,error)(TMP_4929,TMP_4930)
 state = AssetManagerState.get()
TMP_4932(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4932'])(AssetManagerState.State) := TMP_4932(AssetManagerState.State)
 agent = Agent.get(_data.agentVault)
REF_3453(address) -> _data_1.agentVault
TMP_4933(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_3453'] 
agent_1 (-> ['TMP_4933'])(Agent.State) := TMP_4933(Agent.State)
 require(bool,error)(bytes(_redeemerUnderlyingAddressString).length < 128,revert UnderlyingAddressTooLong()())
TMP_4934 = CONVERT _redeemerUnderlyingAddressString_1 to bytes
REF_3454 -> LENGTH TMP_4934
TMP_4935(bool) = REF_3454 < 128
TMP_4936(None) = SOLIDITY_CALL revert UnderlyingAddressTooLong()()
TMP_4937(None) = SOLIDITY_CALL require(bool,error)(TMP_4935,TMP_4936)
 underlyingAddressHash = keccak256(bytes)(bytes(_redeemerUnderlyingAddressString))
TMP_4938 = CONVERT _redeemerUnderlyingAddressString_1 to bytes
TMP_4939(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_4938)
underlyingAddressHash_1(bytes32) := TMP_4939(bytes32)
 require(bool,error)(underlyingAddressHash != agent.underlyingAddressHash,revert CannotRedeemToAgentsAddress()())
REF_3455(bytes32) -> agent_1 (-> ['TMP_4933']).underlyingAddressHash
TMP_4940(bool) = underlyingAddressHash_1 != REF_3455
TMP_4941(None) = SOLIDITY_CALL revert CannotRedeemToAgentsAddress()()
TMP_4942(None) = SOLIDITY_CALL require(bool,error)(TMP_4940,TMP_4941)
 redeemedValueUBA = Conversion.convertAmgToUBA(_data.valueAMG).toUint128()
REF_3457(uint64) -> _data_1.valueAMG
TMP_4943(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_3457'] 
TMP_4944(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['TMP_4943'] 
redeemedValueUBA_1(uint128) := TMP_4944(uint128)
 _requestId = _newRequestId(_poolSelfClose)
TMP_4945(uint64) = INTERNAL_CALL, RedemptionRequests._newRequestId(bool)(_poolSelfClose_1)
_requestId_1(uint64) := TMP_4945(uint64)
 request.redeemerUnderlyingAddressHash = underlyingAddressHash
REF_3459(bytes32) -> request_0.redeemerUnderlyingAddressHash
request_1(Redemption.Request) := phi(['request_0'])
REF_3459(bytes32) (->request_1) := underlyingAddressHash_1(bytes32)
 request.underlyingValueUBA = redeemedValueUBA
REF_3460(uint128) -> request_1.underlyingValueUBA
request_2(Redemption.Request) := phi(['request_1'])
REF_3460(uint128) (->request_2) := redeemedValueUBA_1(uint128)
 request.firstUnderlyingBlock = state.currentUnderlyingBlock
REF_3461(uint64) -> request_2.firstUnderlyingBlock
REF_3462(uint64) -> state_1 (-> ['TMP_4932']).currentUnderlyingBlock
request_3(Redemption.Request) := phi(['request_2'])
REF_3461(uint64) (->request_3) := REF_3462(uint64)
 (request.lastUnderlyingBlock,request.lastUnderlyingTimestamp) = _lastPaymentBlock(_data.agentVault,_additionalPaymentTime)
REF_3463(uint64) -> request_3.lastUnderlyingBlock
REF_3464(uint64) -> request_3.lastUnderlyingTimestamp
REF_3465(address) -> _data_1.agentVault
TUPLE_63(uint64,uint64) = INTERNAL_CALL, RedemptionRequests._lastPaymentBlock(address,uint64)(REF_3465,_additionalPaymentTime_1)
REF_3463(uint64)= UNPACK TUPLE_63 index: 0 
REF_3464(uint64)= UNPACK TUPLE_63 index: 1 
 request.timestamp = block.timestamp.toUint64()
REF_3466(uint64) -> request_3.timestamp
TMP_4946(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.timestamp'] 
request_4(Redemption.Request) := phi(['request_3'])
REF_3466(uint64) (->request_4) := TMP_4946(uint64)
 request.redeemer = _redeemer
REF_3468(address) -> request_7.redeemer
request_8(Redemption.Request) := phi(['request_7'])
REF_3468(address) (->request_8) := _redeemer_1(address)
 request.agentVault = _data.agentVault
REF_3469(address) -> request_8.agentVault
REF_3470(address) -> _data_1.agentVault
request_9(Redemption.Request) := phi(['request_8'])
REF_3469(address) (->request_9) := REF_3470(address)
 request.valueAMG = _data.valueAMG
REF_3471(uint64) -> request_9.valueAMG
REF_3472(uint64) -> _data_1.valueAMG
request_10(Redemption.Request) := phi(['request_9'])
REF_3471(uint64) (->request_10) := REF_3472(uint64)
 request.status = Redemption.Status.ACTIVE
REF_3473(Redemption.Status) -> request_10.status
REF_3474(Redemption.Status) -> Status.ACTIVE
request_11(Redemption.Request) := phi(['request_10'])
REF_3473(Redemption.Status) (->request_11) := REF_3474(Redemption.Status)
 request.poolSelfClose = _poolSelfClose
REF_3475(bool) -> request_11.poolSelfClose
request_12(Redemption.Request) := phi(['request_11'])
REF_3475(bool) (->request_12) := _poolSelfClose_1(bool)
 request.executor = _executor
REF_3476(address) -> request_12.executor
request_13(Redemption.Request) := phi(['request_12'])
REF_3476(address) (->request_13) := _executor_1(address)
 request.executorFeeNatGWei = _executorFeeNatGWei
REF_3477(uint64) -> request_13.executorFeeNatGWei
request_14(Redemption.Request) := phi(['request_13'])
REF_3477(uint64) (->request_14) := _executorFeeNatGWei_1(uint64)
 request.redeemerUnderlyingAddressString = _redeemerUnderlyingAddressString
REF_3478(string) -> request_14.redeemerUnderlyingAddressString
request_15(Redemption.Request) := phi(['request_14'])
REF_3478(string) (->request_15) := _redeemerUnderlyingAddressString_1(string)
 request.transferToCoreVault = _transferToCoreVault
REF_3479(bool) -> request_15.transferToCoreVault
request_16(Redemption.Request) := phi(['request_15'])
REF_3479(bool) (->request_16) := _transferToCoreVault_1(bool)
 request.poolFeeShareBIPS = agent.redemptionPoolFeeShareBIPS
REF_3480(uint16) -> request_16.poolFeeShareBIPS
REF_3481(uint16) -> agent_1 (-> ['TMP_4933']).redemptionPoolFeeShareBIPS
request_17(Redemption.Request) := phi(['request_16'])
REF_3480(uint16) (->request_17) := REF_3481(uint16)
 state.redemptionRequests[_requestId] = request
REF_3482(mapping(uint256 => Redemption.Request)) -> state_1 (-> ['TMP_4932']).redemptionRequests
REF_3483(Redemption.Request) -> REF_3482[_requestId_1]
state_2 (-> ['TMP_4932'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_4932'])"])
REF_3483(Redemption.Request) (->state_2 (-> ['TMP_4932'])) := request_17(Redemption.Request)
TMP_4932(AssetManagerState.State) := phi(["state_2 (-> ['TMP_4932'])"])
 AgentBacking.startRedeemingAssets(agent,_data.valueAMG,_poolSelfClose)
REF_3485(uint64) -> _data_1.valueAMG
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.startRedeemingAssets(Agent.State,uint64,bool), arguments:["agent_1 (-> ['TMP_4933'])", 'REF_3485', '_poolSelfClose_1'] 
 _emitRedemptionRequestedEvent(request,_requestId,_redeemerUnderlyingAddressString)
INTERNAL_CALL, RedemptionRequests._emitRedemptionRequestedEvent(Redemption.Request,uint64,string)(request_17,_requestId_1,_redeemerUnderlyingAddressString_1)
 _transferToCoreVault
CONDITION _transferToCoreVault_1
 request.underlyingFeeUBA = 0
REF_3486(uint128) -> request_4.underlyingFeeUBA
request_6(Redemption.Request) := phi(['request_4'])
REF_3486(uint128) (->request_6) := 0(uint256)
 request.underlyingFeeUBA = uint256(redeemedValueUBA).mulBips(Globals.getSettings().redemptionFeeBIPS).toUint128()
REF_3487(uint128) -> request_4.underlyingFeeUBA
TMP_4949 = CONVERT redeemedValueUBA_1 to uint256
TMP_4950(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_3490(uint16) -> TMP_4950.redemptionFeeBIPS
TMP_4951(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_4949', 'REF_3490'] 
TMP_4952(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['TMP_4951'] 
request_5(Redemption.Request) := phi(['request_4'])
REF_3487(uint128) (->request_5) := TMP_4952(uint128)
request_7(Redemption.Request) := phi(['request_5', 'request_6'])
 _requestId
RETURN _requestId_1
```
#### Redemptions.closeTickets(Agent.State,uint64,bool) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4984(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4984'])(AssetManagerState.State) := TMP_4984(AssetManagerState.State)
 maxRedeemedTickets = Globals.getSettings().maxRedeemedTickets
TMP_4985(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_3521(uint16) -> TMP_4985.maxRedeemedTickets
maxRedeemedTickets_1(uint256) := REF_3521(uint16)
 lotSize = Globals.getSettings().lotSizeAMG
TMP_4986(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_3523(uint64) -> TMP_4986.lotSizeAMG
lotSize_1(uint64) := REF_3523(uint64)
 i = 0
i_1(uint256) := 0(uint256)
 i < maxRedeemedTickets && _closedAMG < _amountAMG
_closedAMG_1(uint64) := phi(['_closedAMG_0', '_closedAMG_2'])
i_2(uint256) := phi(['i_3', 'i_1'])
TMP_4987(bool) = i_2 < maxRedeemedTickets_1
TMP_4988(bool) = _closedAMG_1 < _amountAMG_1
TMP_4989(bool) = TMP_4987 && TMP_4988
CONDITION TMP_4989
 ticketId = state.redemptionQueue.agents[_agent.vaultAddress()].firstTicketId
REF_3524(RedemptionQueue.State) -> state_1 (-> ['TMP_4984']).redemptionQueue
REF_3525(mapping(address => RedemptionQueue.AgentQueue)) -> REF_3524.agents
TMP_4990(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
REF_3527(RedemptionQueue.AgentQueue) -> REF_3525[TMP_4990]
REF_3528(uint64) -> REF_3527.firstTicketId
ticketId_1(uint64) := REF_3528(uint64)
 ticketId == 0
TMP_4991(bool) = ticketId_1 == 0
CONDITION TMP_4991
 ticket = state.redemptionQueue.getTicket(ticketId)
REF_3529(RedemptionQueue.State) -> state_1 (-> ['TMP_4984']).redemptionQueue
TMP_4992(RedemptionQueue.Ticket) = LIBRARY_CALL, dest:RedemptionQueue, function:RedemptionQueue.getTicket(RedemptionQueue.State,uint64), arguments:['REF_3529', 'ticketId_1'] 
ticket_1 (-> ['TMP_4992'])(RedemptionQueue.Ticket) := TMP_4992(RedemptionQueue.Ticket)
 maxTicketRedeemAMG = ticket.valueAMG + _agent.dustAMG
REF_3531(uint64) -> ticket_1 (-> ['TMP_4992']).valueAMG
REF_3532(uint64) -> _agent_1 (-> []).dustAMG
TMP_4993(uint64) = REF_3531 (c)+ REF_3532
maxTicketRedeemAMG_1(uint64) := TMP_4993(uint64)
 maxTicketRedeemAMG -= maxTicketRedeemAMG % lotSize
TMP_4994(uint64) = maxTicketRedeemAMG_1 % lotSize_1
maxTicketRedeemAMG_2(uint64) = maxTicketRedeemAMG_1 (c)- TMP_4994
 ticketRedeemAMG = SafeMath64.min64(_amountAMG - _closedAMG,maxTicketRedeemAMG)
TMP_4995(uint64) = _amountAMG_1 (c)- _closedAMG_1
TMP_4996(uint64) = LIBRARY_CALL, dest:SafeMath64, function:SafeMath64.min64(uint64,uint64), arguments:['TMP_4995', 'maxTicketRedeemAMG_2'] 
ticketRedeemAMG_1(uint64) := TMP_4996(uint64)
 removeFromTicket(ticketId,ticketRedeemAMG)
INTERNAL_CALL, Redemptions.removeFromTicket(uint64,uint64)(ticketId_1,ticketRedeemAMG_1)
 _closedAMG += ticketRedeemAMG
_closedAMG_2(uint64) = _closedAMG_1 (c)+ ticketRedeemAMG_1
 i ++
TMP_4998(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 closeDustAMG = SafeMath64.min64(_amountAMG - _closedAMG,_agent.dustAMG)
TMP_4999(uint64) = _amountAMG_1 (c)- _closedAMG_1
REF_3535(uint64) -> _agent_1 (-> []).dustAMG
TMP_5000(uint64) = LIBRARY_CALL, dest:SafeMath64, function:SafeMath64.min64(uint64,uint64), arguments:['TMP_4999', 'REF_3535'] 
closeDustAMG_1(uint64) := TMP_5000(uint64)
 closeDustAMG > 0
TMP_5001(bool) = closeDustAMG_1 > 0
CONDITION TMP_5001
 _closedAMG += closeDustAMG
_closedAMG_3(uint64) = _closedAMG_1 (c)+ closeDustAMG_1
 AgentBacking.decreaseDust(_agent,closeDustAMG)
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.decreaseDust(Agent.State,uint64), arguments:['_agent_1 (-> [])', 'closeDustAMG_1'] 
_closedAMG_4(uint64) := phi(['_closedAMG_0', '_closedAMG_3'])
 _immediatelyReleaseMinted
CONDITION _immediatelyReleaseMinted_1
 AgentBacking.releaseMintedAssets(_agent,_closedAMG)
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.releaseMintedAssets(Agent.State,uint64), arguments:['_agent_1 (-> [])', '_closedAMG_4'] 
 _closedUBA = Conversion.convertAmgToUBA(_closedAMG)
TMP_5004(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['_closedAMG_4'] 
_closedUBA_1(uint256) := TMP_5004(uint256)
 (_closedAMG,_closedUBA)
RETURN _closedAMG_4,_closedUBA_1
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
#### AgentBacking.changeDust(Agent.State,uint64) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_2 (-> [])'])
_newDustAMG_1(uint64) := phi(['newDustAMG_1', 'newDustAMG_1'])
 _agent.dustAMG == _newDustAMG
REF_2808(uint64) -> _agent_1 (-> []).dustAMG
TMP_4290(bool) = REF_2808 == _newDustAMG_1
CONDITION TMP_4290
 _agent.dustAMG = _newDustAMG
REF_2809(uint64) -> _agent_1 (-> []).dustAMG
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2809(uint64) (->_agent_2 (-> [])) := _newDustAMG_1(uint64)
 dustUBA = Conversion.convertAmgToUBA(_newDustAMG)
TMP_4291(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['_newDustAMG_1'] 
dustUBA_1(uint256) := TMP_4291(uint256)
 IAssetManagerEvents.DustChanged(_agent.vaultAddress(),dustUBA)
TMP_4292(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_2 (-> [])'] 
Emit DustChanged(TMP_4292,dustUBA_1)
```
#### AgentBacking.createRedemptionTicket(Agent.State,uint64) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_2 (-> [])'])
_ticketValueAMG_1(uint64) := phi(['ticketValueAMG_1'])
 state = AssetManagerState.get()
TMP_4280(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4280'])(AssetManagerState.State) := TMP_4280(AssetManagerState.State)
 _ticketValueAMG == 0
TMP_4281(bool) = _ticketValueAMG_1 == 0
CONDITION TMP_4281
 vaultAddress = _agent.vaultAddress()
TMP_4282(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
vaultAddress_1(address) := TMP_4282(address)
 lastTicketId = state.redemptionQueue.lastTicketId
REF_2795(RedemptionQueue.State) -> state_1 (-> ['TMP_4280']).redemptionQueue
REF_2796(uint64) -> REF_2795.lastTicketId
lastTicketId_1(uint64) := REF_2796(uint64)
 lastTicket = state.redemptionQueue.getTicket(lastTicketId)
REF_2797(RedemptionQueue.State) -> state_1 (-> ['TMP_4280']).redemptionQueue
TMP_4283(RedemptionQueue.Ticket) = LIBRARY_CALL, dest:RedemptionQueue, function:RedemptionQueue.getTicket(RedemptionQueue.State,uint64), arguments:['REF_2797', 'lastTicketId_1'] 
lastTicket_1 (-> ['TMP_4283'])(RedemptionQueue.Ticket) := TMP_4283(RedemptionQueue.Ticket)
 lastTicket.agentVault == vaultAddress
REF_2799(address) -> lastTicket_1 (-> ['TMP_4283']).agentVault
TMP_4284(bool) = REF_2799 == vaultAddress_1
CONDITION TMP_4284
 lastTicket.valueAMG += _ticketValueAMG
REF_2800(uint64) -> lastTicket_1 (-> ['TMP_4283']).valueAMG
lastTicket_2 (-> ['TMP_4283'])(RedemptionQueue.Ticket) := phi(["lastTicket_1 (-> ['TMP_4283'])"])
REF_2800(-> lastTicket_2 (-> ['TMP_4283'])) = REF_2800 (c)+ _ticketValueAMG_1
TMP_4283(RedemptionQueue.Ticket) := phi(["lastTicket_2 (-> ['TMP_4283'])"])
 ticketValueUBA = Conversion.convertAmgToUBA(lastTicket.valueAMG)
REF_2802(uint64) -> lastTicket_2 (-> ['TMP_4283']).valueAMG
TMP_4285(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_2802'] 
ticketValueUBA_1(uint256) := TMP_4285(uint256)
 IAssetManagerEvents.RedemptionTicketUpdated(vaultAddress,lastTicketId,ticketValueUBA)
Emit RedemptionTicketUpdated(vaultAddress_1,lastTicketId_1,ticketValueUBA_1)
 ticketId = state.redemptionQueue.createRedemptionTicket(vaultAddress,_ticketValueAMG)
REF_2804(RedemptionQueue.State) -> state_1 (-> ['TMP_4280']).redemptionQueue
TMP_4287(uint64) = LIBRARY_CALL, dest:RedemptionQueue, function:RedemptionQueue.createRedemptionTicket(RedemptionQueue.State,address,uint64), arguments:['REF_2804', 'vaultAddress_1', '_ticketValueAMG_1'] 
ticketId_1(uint64) := TMP_4287(uint64)
 ticketValueUBA_scope_0 = Conversion.convertAmgToUBA(_ticketValueAMG)
TMP_4288(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['_ticketValueAMG_1'] 
ticketValueUBA_scope_0_1(uint256) := TMP_4288(uint256)
 IAssetManagerEvents.RedemptionTicketCreated(vaultAddress,ticketId,ticketValueUBA_scope_0)
Emit RedemptionTicketCreated(vaultAddress_1,ticketId_1,ticketValueUBA_scope_0_1)
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
#### TransactionAttestation.verifyPayment(IPayment.Proof) [INTERNAL]
```slithir
_proof_1(IPayment.Proof) := phi(['_proof_1'])
 _settings = Globals.getSettings()
TMP_5237(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5237'])(AssetManagerSettings.Data) := TMP_5237(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3670(address) -> _settings_1 (-> ['TMP_5237']).fdcVerification
TMP_5238 = CONVERT REF_3670 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5238(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3671(IPayment.Response) -> _proof_1.data
REF_3672(bytes32) -> REF_3671.sourceId
REF_3673(bytes32) -> _settings_1 (-> ['TMP_5237']).chainId
TMP_5239(bool) = REF_3672 == REF_3673
TMP_5240(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5241(None) = SOLIDITY_CALL require(bool,error)(TMP_5239,TMP_5240)
 require(bool,error)(fdcVerification.verifyPayment(_proof),revert LegalPaymentNotProven()())
TMP_5242(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyPayment, arguments:['_proof_1']  
TMP_5243(None) = SOLIDITY_CALL revert LegalPaymentNotProven()()
TMP_5244(None) = SOLIDITY_CALL require(bool,error)(TMP_5242,TMP_5243)
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
#### SafeCast.toInt128(int256) [INTERNAL]
```slithir
 downcasted = int128(value)
TMP_834 = CONVERT value_1 to int128
downcasted_1(int128) := TMP_834(int128)
 require(bool,string)(downcasted == value,SafeCast: value doesn't fit in 128 bits)
TMP_835(bool) = downcasted_1 == value_1
TMP_836(None) = SOLIDITY_CALL require(bool,string)(TMP_835,SafeCast: value doesn't fit in 128 bits)
 downcasted
RETURN downcasted_1
```
#### UnderlyingBlockUpdater.updateCurrentBlock(IConfirmedBlockHeightExists.Proof) [INTERNAL]
```slithir
 TransactionAttestation.verifyConfirmedBlockHeightExists(_proof)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyConfirmedBlockHeightExists(IConfirmedBlockHeightExists.Proof), arguments:['_proof_1'] 
 updateCurrentBlock(_proof.data.requestBody.blockNumber,_proof.data.responseBody.blockTimestamp,_proof.data.responseBody.numberOfConfirmations)
REF_3720(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_3721(IConfirmedBlockHeightExists.RequestBody) -> REF_3720.requestBody
REF_3722(uint64) -> REF_3721.blockNumber
REF_3723(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_3724(IConfirmedBlockHeightExists.ResponseBody) -> REF_3723.responseBody
REF_3725(uint64) -> REF_3724.blockTimestamp
REF_3726(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_3727(IConfirmedBlockHeightExists.ResponseBody) -> REF_3726.responseBody
REF_3728(uint64) -> REF_3727.numberOfConfirmations
INTERNAL_CALL, UnderlyingBlockUpdater.updateCurrentBlock(uint64,uint64,uint64)(REF_3722,REF_3725,REF_3728)
```

#### CoreVaultClient.coreVaultUnderlyingPaymentFee() [INTERNAL]
```slithir
 state = getState()
TMP_4706(CoreVaultClient.State) = INTERNAL_CALL, CoreVaultClient.getState()()
state_1 (-> ['TMP_4706'])(CoreVaultClient.State) := TMP_4706(CoreVaultClient.State)
 (None,None,None,fee) = state.coreVaultManager.getSettings()
REF_3209(IICoreVaultManager) -> state_1 (-> ['TMP_4706']).coreVaultManager
TUPLE_53(uint128,uint128,uint128,uint128) = HIGH_LEVEL_CALL, dest:REF_3209(IICoreVaultManager), function:getSettings, arguments:[]  
fee_1(uint256)= UNPACK TUPLE_53 index: 3 
 fee
RETURN fee_1
```
#### MathUtils.subOrZero(uint256,uint256) [INTERNAL]
```slithir
 _a > _b
TMP_10425(bool) = _a_1 > _b_1
CONDITION TMP_10425
 _a - _b
TMP_10426(uint256) = _a_1 (c)- _b_1
RETURN TMP_10426
 0
RETURN 0
```
#### CoreVaultClient._minimumRemainingAfterTransferAMG(Agent.State) [PRIVATE]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 cd = AgentCollateral.combinedData(_agent)
TMP_4719(Collateral.CombinedData) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.combinedData(Agent.State), arguments:['_agent_1 (-> [])'] 
cd_1(Collateral.CombinedData) := TMP_4719(Collateral.CombinedData)
 resultWRTVault = _minimumRemainingAfterTransferForCollateralAMG(_agent,cd.agentCollateral)
REF_3216(Collateral.Data) -> cd_1.agentCollateral
TMP_4720(uint256) = INTERNAL_CALL, CoreVaultClient._minimumRemainingAfterTransferForCollateralAMG(Agent.State,Collateral.Data)(_agent_1 (-> []),REF_3216)
resultWRTVault_1(uint256) := TMP_4720(uint256)
 resultWRTPool = _minimumRemainingAfterTransferForCollateralAMG(_agent,cd.poolCollateral)
REF_3217(Collateral.Data) -> cd_1.poolCollateral
TMP_4721(uint256) = INTERNAL_CALL, CoreVaultClient._minimumRemainingAfterTransferForCollateralAMG(Agent.State,Collateral.Data)(_agent_1 (-> []),REF_3217)
resultWRTPool_1(uint256) := TMP_4721(uint256)
 resultWRTAgentPT = _minimumRemainingAfterTransferForCollateralAMG(_agent,cd.agentPoolTokens)
REF_3218(Collateral.Data) -> cd_1.agentPoolTokens
TMP_4722(uint256) = INTERNAL_CALL, CoreVaultClient._minimumRemainingAfterTransferForCollateralAMG(Agent.State,Collateral.Data)(_agent_1 (-> []),REF_3218)
resultWRTAgentPT_1(uint256) := TMP_4722(uint256)
 Math.min(resultWRTVault,Math.min(resultWRTPool,resultWRTAgentPT))
TMP_4723(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['resultWRTPool_1', 'resultWRTAgentPT_1'] 
TMP_4724(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['resultWRTVault_1', 'TMP_4723'] 
RETURN TMP_4724
```
#### Globals.getFAsset() [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4735(AssetManagerSettings.Data) = INTERNAL_CALL, Globals.getSettings()()
settings_1 (-> ['TMP_4735'])(AssetManagerSettings.Data) := TMP_4735(AssetManagerSettings.Data)
 IIFAsset(settings.fAsset)
REF_3240(address) -> settings_1 (-> ['TMP_4735']).fAsset
TMP_4736 = CONVERT REF_3240 to IIFAsset
RETURN TMP_4736
```
#### FAsset.burn(address,uint256) [EXTERNAL]
```slithir
 _burn(_owner,_amount)
INTERNAL_CALL, ERC20._burn(address,uint256)(_owner_1,_amount_1)
 onlyAssetManager()
MODIFIER_CALL, FAsset.onlyAssetManager()()
```
#### CoreVaultManager.totalRequestAmountWithFee() [PUBLIC]
```slithir
cancelableTransferRequests_18(uint256[]) := phi(['cancelableTransferRequests_17', 'cancelableTransferRequests_13', 'cancelableTransferRequests_18', 'cancelableTransferRequests_0', 'cancelableTransferRequests_9', 'cancelableTransferRequests_3', 'cancelableTransferRequests_5'])
nonCancelableTransferRequests_13(uint256[]) := phi(['nonCancelableTransferRequests_5', 'nonCancelableTransferRequests_8', 'nonCancelableTransferRequests_13', 'nonCancelableTransferRequests_0', 'nonCancelableTransferRequests_12', 'nonCancelableTransferRequests_11', 'nonCancelableTransferRequests_3', 'nonCancelableTransferRequests_10'])
fee_6(uint128) := phi(['fee_0', 'fee_4', 'fee_3'])
cancelableTransferRequestsAmount_12(uint128) := phi(['cancelableTransferRequestsAmount_11', 'cancelableTransferRequestsAmount_4', 'cancelableTransferRequestsAmount_0', 'cancelableTransferRequestsAmount_7', 'cancelableTransferRequestsAmount_3'])
nonCancelableTransferRequestsAmount_9(uint128) := phi(['nonCancelableTransferRequestsAmount_3', 'nonCancelableTransferRequestsAmount_4', 'nonCancelableTransferRequestsAmount_8', 'nonCancelableTransferRequestsAmount_0'])
 nonCancelableTransferRequestsAmount + cancelableTransferRequestsAmount + (cancelableTransferRequests.length + nonCancelableTransferRequests.length) * fee
TMP_7099(uint128) = nonCancelableTransferRequestsAmount_9 (c)+ cancelableTransferRequestsAmount_12
REF_4636 -> LENGTH cancelableTransferRequests_18
REF_4637 -> LENGTH nonCancelableTransferRequests_13
TMP_7100(uint256) = REF_4636 (c)+ REF_4637
TMP_7101(uint256) = TMP_7100 (c)* fee_6
TMP_7102(uint128) = TMP_7099 (c)+ TMP_7101
RETURN TMP_7102
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
#### AgentBacking.startRedeemingAssets(Agent.State,uint64,bool) [INTERNAL]
```slithir
 _agent.redeemingAMG += _valueAMG
REF_2783(uint64) -> _agent_1 (-> []).redeemingAMG
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2783(-> _agent_2 (-> [])) = REF_2783 (c)+ _valueAMG_1
 ! _poolSelfCloseRedemption
TMP_4268 = UnaryType.BANG _poolSelfCloseRedemption_1 
CONDITION TMP_4268
 _agent.poolRedeemingAMG += _valueAMG
REF_2784(uint64) -> _agent_2 (-> []).poolRedeemingAMG
_agent_3 (-> [])(Agent.State) := phi(['_agent_2 (-> [])'])
REF_2784(-> _agent_3 (-> [])) = REF_2784 (c)+ _valueAMG_1
_agent_4 (-> [])(Agent.State) := phi(['_agent_3 (-> [])', '_agent_2 (-> [])'])
 releaseMintedAssets(_agent,_valueAMG)
INTERNAL_CALL, AgentBacking.releaseMintedAssets(Agent.State,uint64)(_agent_4 (-> []),_valueAMG_1)
```
#### RedemptionRequests._emitRedemptionRequestedEvent(Redemption.Request,uint64,string) [PRIVATE]
```slithir
_request_1(Redemption.Request) := phi(['request_17'])
_requestId_1(uint64) := phi(['_requestId_1'])
_redeemerUnderlyingAddressString_1(string) := phi(['_redeemerUnderlyingAddressString_1'])
 IAssetManagerEvents.RedemptionRequested(_request.agentVault,_request.redeemer,_requestId,_redeemerUnderlyingAddressString,_request.underlyingValueUBA,_request.underlyingFeeUBA,_request.firstUnderlyingBlock,_request.lastUnderlyingBlock,_request.lastUnderlyingTimestamp,PaymentReference.redemption(_requestId),_request.executor,_request.executorFeeNatGWei * Conversion.GWEI)
REF_3493(address) -> _request_1.agentVault
REF_3494(address) -> _request_1.redeemer
REF_3495(uint128) -> _request_1.underlyingValueUBA
REF_3496(uint128) -> _request_1.underlyingFeeUBA
REF_3497(uint64) -> _request_1.firstUnderlyingBlock
REF_3498(uint64) -> _request_1.lastUnderlyingBlock
REF_3499(uint64) -> _request_1.lastUnderlyingTimestamp
TMP_4953(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.redemption(uint256), arguments:['_requestId_1'] 
REF_3501(address) -> _request_1.executor
REF_3502(uint64) -> _request_1.executorFeeNatGWei
REF_3503(uint256) -> Conversion.GWEI
TMP_4954(uint64) = REF_3502 (c)* REF_3503
Emit RedemptionRequested(REF_3493,REF_3494,_requestId_1,_redeemerUnderlyingAddressString_1,REF_3495,REF_3496,REF_3497,REF_3498,REF_3499,TMP_4953,REF_3501,TMP_4954)
```

#### RedemptionRequests._newRequestId(bool) [PRIVATE]
```slithir
_poolSelfClose_1(bool) := phi(['_poolSelfClose_1'])
 state = AssetManagerState.get()
TMP_4956(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4956'])(AssetManagerState.State) := TMP_4956(AssetManagerState.State)
 nextRequestId = state.newRedemptionRequestId + PaymentReference.randomizedIdSkip()
REF_3505(uint64) -> state_1 (-> ['TMP_4956']).newRedemptionRequestId
TMP_4957(uint64) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.randomizedIdSkip(), arguments:[] 
TMP_4958(uint64) = REF_3505 (c)+ TMP_4957
nextRequestId_1(uint64) := TMP_4958(uint64)
 state.newRedemptionRequestId = requestId
REF_3507(uint64) -> state_1 (-> ['TMP_4956']).newRedemptionRequestId
state_2 (-> ['TMP_4956'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_4956'])"])
REF_3507(uint64) (->state_2 (-> ['TMP_4956'])) := requestId_3(uint64)
TMP_4956(AssetManagerState.State) := phi(["state_2 (-> ['TMP_4956'])"])
 requestId
RETURN requestId_3
 _poolSelfClose
CONDITION _poolSelfClose_1
 requestId = ((nextRequestId + 1) & ~ uint64(1)) | 1
TMP_4959(uint64) = nextRequestId_1 (c)+ 1
TMP_4960 = CONVERT 1 to uint64
TMP_4961 = UnaryType.TILD TMP_4960 
TMP_4962(uint64) = TMP_4959 & TMP_4961
TMP_4963(uint64) = TMP_4962 | 1
requestId_2(uint64) := TMP_4963(uint64)
 requestId = ((nextRequestId + 1) & ~ uint64(1)) | 0
TMP_4964(uint64) = nextRequestId_1 (c)+ 1
TMP_4965 = CONVERT 1 to uint64
TMP_4966 = UnaryType.TILD TMP_4965 
TMP_4967(uint64) = TMP_4964 & TMP_4966
TMP_4968(uint64) = TMP_4967 | 0
requestId_1(uint64) := TMP_4968(uint64)
requestId_3(uint64) := phi(['requestId_1', 'requestId_2'])
```

#### AgentBacking.releaseMintedAssets(Agent.State,uint64) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_4 (-> [])'])
_valueAMG_1(uint64) := phi(['_valueAMG_1'])
 _agent.mintedAMG = _agent.mintedAMG - _valueAMG
REF_2781(uint64) -> _agent_1 (-> []).mintedAMG
REF_2782(uint64) -> _agent_1 (-> []).mintedAMG
TMP_4267(uint64) = REF_2782 (c)- _valueAMG_1
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2781(uint64) (->_agent_2 (-> [])) := TMP_4267(uint64)
```
#### Redemptions.removeFromTicket(uint64,uint64) [INTERNAL]
```slithir
_redemptionTicketId_1(uint64) := phi(['ticketId_1'])
_redeemedAMG_1(uint64) := phi(['ticketRedeemAMG_1'])
 redemptionQueue = AssetManagerState.get().redemptionQueue
TMP_5005(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
REF_3540(RedemptionQueue.State) -> TMP_5005.redemptionQueue
redemptionQueue_1 (-> ['TMP_5005'])(RedemptionQueue.State) := REF_3540(RedemptionQueue.State)
 ticket = redemptionQueue.getTicket(_redemptionTicketId)
TMP_5006(RedemptionQueue.Ticket) = LIBRARY_CALL, dest:RedemptionQueue, function:RedemptionQueue.getTicket(RedemptionQueue.State,uint64), arguments:["redemptionQueue_1 (-> ['TMP_5005'])", '_redemptionTicketId_1'] 
ticket_1 (-> ['TMP_5006'])(RedemptionQueue.Ticket) := TMP_5006(RedemptionQueue.Ticket)
 agent = Agent.get(ticket.agentVault)
REF_3543(address) -> ticket_1 (-> ['TMP_5006']).agentVault
TMP_5007(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_3543'] 
agent_1 (-> ['TMP_5007'])(Agent.State) := TMP_5007(Agent.State)
 lotSize = Globals.getSettings().lotSizeAMG
TMP_5008(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_3545(uint64) -> TMP_5008.lotSizeAMG
lotSize_1(uint64) := REF_3545(uint64)
 remainingAMG = ticket.valueAMG + agent.dustAMG - _redeemedAMG
REF_3546(uint64) -> ticket_1 (-> ['TMP_5006']).valueAMG
REF_3547(uint64) -> agent_1 (-> ['TMP_5007']).dustAMG
TMP_5009(uint64) = REF_3546 (c)+ REF_3547
TMP_5010(uint64) = TMP_5009 (c)- _redeemedAMG_1
remainingAMG_1(uint64) := TMP_5010(uint64)
 remainingAMGDust = remainingAMG % lotSize
TMP_5011(uint64) = remainingAMG_1 % lotSize_1
remainingAMGDust_1(uint64) := TMP_5011(uint64)
 remainingAMGLots = remainingAMG - remainingAMGDust
TMP_5012(uint64) = remainingAMG_1 (c)- remainingAMGDust_1
remainingAMGLots_1(uint64) := TMP_5012(uint64)
 remainingAMGLots == 0
TMP_5013(bool) = remainingAMGLots_1 == 0
CONDITION TMP_5013
 redemptionQueue.deleteRedemptionTicket(_redemptionTicketId)
LIBRARY_CALL, dest:RedemptionQueue, function:RedemptionQueue.deleteRedemptionTicket(RedemptionQueue.State,uint64), arguments:["redemptionQueue_1 (-> ['TMP_5005'])", '_redemptionTicketId_1'] 
 IAssetManagerEvents.RedemptionTicketDeleted(agent.vaultAddress(),_redemptionTicketId)
TMP_5015(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:["agent_1 (-> ['TMP_5007'])"] 
Emit RedemptionTicketDeleted(TMP_5015,_redemptionTicketId_1)
 remainingAMGLots != ticket.valueAMG
REF_3551(uint64) -> ticket_1 (-> ['TMP_5006']).valueAMG
TMP_5017(bool) = remainingAMGLots_1 != REF_3551
CONDITION TMP_5017
 ticket.valueAMG = remainingAMGLots
REF_3552(uint64) -> ticket_1 (-> ['TMP_5006']).valueAMG
ticket_2 (-> ['TMP_5006'])(RedemptionQueue.Ticket) := phi(["ticket_1 (-> ['TMP_5006'])"])
REF_3552(uint64) (->ticket_2 (-> ['TMP_5006'])) := remainingAMGLots_1(uint64)
TMP_5006(RedemptionQueue.Ticket) := phi(["ticket_2 (-> ['TMP_5006'])"])
 remainingUBA = Conversion.convertAmgToUBA(remainingAMGLots)
TMP_5018(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['remainingAMGLots_1'] 
remainingUBA_1(uint256) := TMP_5018(uint256)
 IAssetManagerEvents.RedemptionTicketUpdated(agent.vaultAddress(),_redemptionTicketId,remainingUBA)
TMP_5019(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:["agent_1 (-> ['TMP_5007'])"] 
Emit RedemptionTicketUpdated(TMP_5019,_redemptionTicketId_1,remainingUBA_1)
 AgentBacking.changeDust(agent,remainingAMGDust)
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.changeDust(Agent.State,uint64), arguments:["agent_1 (-> ['TMP_5007'])", 'remainingAMGDust_1']
```

#### RedemptionQueue.createRedemptionTicket(RedemptionQueue.State,address,uint64) [INTERNAL]
```slithir
 agent = _state.agents[_agentVault]
REF_3775(mapping(address => RedemptionQueue.AgentQueue)) -> _state_1 (-> []).agents
REF_3776(RedemptionQueue.AgentQueue) -> REF_3775[_agentVault_1]
agent_1 (-> ['_state'])(RedemptionQueue.AgentQueue) := REF_3776(RedemptionQueue.AgentQueue)
 ticketId = ++ _state.newTicketId
REF_3777(uint64) -> _state_1 (-> []).newTicketId
_state_2 (-> [])(RedemptionQueue.State) := phi(['_state_1 (-> [])'])
REF_3777(-> _state_2 (-> [])) = REF_3777 (c)+ 1
ticketId_1(uint64) := REF_3777(uint64)
 _state.tickets[ticketId] = Ticket({agentVault:_agentVault,valueAMG:_valueAMG,prev:_state.lastTicketId,next:0,prevForAgent:agent.lastTicketId,nextForAgent:0})
REF_3778(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_2 (-> []).tickets
REF_3779(RedemptionQueue.Ticket) -> REF_3778[ticketId_1]
REF_3780(uint64) -> _state_2 (-> []).lastTicketId
REF_3781(uint64) -> agent_1 (-> ['_state']).lastTicketId
TMP_5399(RedemptionQueue.Ticket) = new Ticket(_agentVault_1,_valueAMG_1,REF_3780,0,REF_3781,0)
_state_3 (-> [])(RedemptionQueue.State) := phi(['_state_2 (-> [])'])
REF_3779(RedemptionQueue.Ticket) (->_state_3 (-> [])) := TMP_5399(RedemptionQueue.Ticket)
 _state.firstTicketId == 0
REF_3782(uint64) -> _state_3 (-> []).firstTicketId
TMP_5400(bool) = REF_3782 == 0
CONDITION TMP_5400
 assert(bool)(_state.lastTicketId == 0)
REF_3783(uint64) -> _state_3 (-> []).lastTicketId
TMP_5401(bool) = REF_3783 == 0
TMP_5402(None) = SOLIDITY_CALL assert(bool)(TMP_5401)
 _state.firstTicketId = ticketId
REF_3784(uint64) -> _state_3 (-> []).firstTicketId
_state_4 (-> [])(RedemptionQueue.State) := phi(['_state_3 (-> [])'])
REF_3784(uint64) (->_state_4 (-> [])) := ticketId_1(uint64)
 assert(bool)(_state.lastTicketId != 0)
REF_3785(uint64) -> _state_3 (-> []).lastTicketId
TMP_5403(bool) = REF_3785 != 0
TMP_5404(None) = SOLIDITY_CALL assert(bool)(TMP_5403)
 _state.tickets[_state.lastTicketId].next = ticketId
REF_3786(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_3 (-> []).tickets
REF_3787(uint64) -> _state_3 (-> []).lastTicketId
REF_3788(RedemptionQueue.Ticket) -> REF_3786[REF_3787]
REF_3789(uint64) -> REF_3788.next
_state_5 (-> [])(RedemptionQueue.State) := phi(['_state_3 (-> [])'])
REF_3789(uint64) (->_state_5 (-> [])) := ticketId_1(uint64)
_state_6 (-> [])(RedemptionQueue.State) := phi(['_state_4 (-> [])', '_state_5 (-> [])'])
 _state.lastTicketId = ticketId
REF_3790(uint64) -> _state_6 (-> []).lastTicketId
_state_7 (-> [])(RedemptionQueue.State) := phi(['_state_6 (-> [])'])
REF_3790(uint64) (->_state_7 (-> [])) := ticketId_1(uint64)
 agent.firstTicketId == 0
REF_3791(uint64) -> agent_1 (-> ['_state']).firstTicketId
TMP_5405(bool) = REF_3791 == 0
CONDITION TMP_5405
 assert(bool)(agent.lastTicketId == 0)
REF_3792(uint64) -> agent_1 (-> ['_state']).lastTicketId
TMP_5406(bool) = REF_3792 == 0
TMP_5407(None) = SOLIDITY_CALL assert(bool)(TMP_5406)
 agent.firstTicketId = ticketId
REF_3793(uint64) -> agent_1 (-> ['_state']).firstTicketId
agent_2 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_1 (-> ['_state'])"])
REF_3793(uint64) (->agent_2 (-> ['_state'])) := ticketId_1(uint64)
_state_9 (-> ['_state'])(RedemptionQueue.State) := phi(["agent_2 (-> ['_state'])"])
 assert(bool)(agent.lastTicketId != 0)
REF_3794(uint64) -> agent_1 (-> ['_state']).lastTicketId
TMP_5408(bool) = REF_3794 != 0
TMP_5409(None) = SOLIDITY_CALL assert(bool)(TMP_5408)
 _state.tickets[agent.lastTicketId].nextForAgent = ticketId
REF_3795(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_7 (-> []).tickets
REF_3796(uint64) -> agent_1 (-> ['_state']).lastTicketId
REF_3797(RedemptionQueue.Ticket) -> REF_3795[REF_3796]
REF_3798(uint64) -> REF_3797.nextForAgent
_state_8 (-> [])(RedemptionQueue.State) := phi(['_state_7 (-> [])'])
REF_3798(uint64) (->_state_8 (-> [])) := ticketId_1(uint64)
agent_3 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_2 (-> ['_state'])", "agent_1 (-> ['_state'])"])
 agent.lastTicketId = ticketId
REF_3799(uint64) -> agent_3 (-> ['_state']).lastTicketId
agent_4 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_3 (-> ['_state'])"])
REF_3799(uint64) (->agent_4 (-> ['_state'])) := ticketId_1(uint64)
_state_10 (-> ['_state'])(RedemptionQueue.State) := phi(["agent_4 (-> ['_state'])"])
 ticketId
RETURN ticketId_1
```
#### TransactionAttestation.verifyConfirmedBlockHeightExists(IConfirmedBlockHeightExists.Proof) [INTERNAL]
```slithir
 _settings = Globals.getSettings()
TMP_5253(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5253'])(AssetManagerSettings.Data) := TMP_5253(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3682(address) -> _settings_1 (-> ['TMP_5253']).fdcVerification
TMP_5254 = CONVERT REF_3682 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5254(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3683(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_3684(bytes32) -> REF_3683.sourceId
REF_3685(bytes32) -> _settings_1 (-> ['TMP_5253']).chainId
TMP_5255(bool) = REF_3684 == REF_3685
TMP_5256(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5257(None) = SOLIDITY_CALL require(bool,error)(TMP_5255,TMP_5256)
 require(bool,error)(fdcVerification.verifyConfirmedBlockHeightExists(_proof),revert BlockHeightNotProven()())
TMP_5258(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyConfirmedBlockHeightExists, arguments:['_proof_1']  
TMP_5259(None) = SOLIDITY_CALL revert BlockHeightNotProven()()
TMP_5260(None) = SOLIDITY_CALL require(bool,error)(TMP_5258,TMP_5259)
```
#### CoreVaultClient._minimumRemainingAfterTransferForCollateralAMG(Agent.State,Collateral.Data) [PRIVATE]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_data_1(Collateral.Data) := phi(['REF_3216', 'REF_3218', 'REF_3217'])
 state = getState()
TMP_4725(CoreVaultClient.State) = INTERNAL_CALL, CoreVaultClient.getState()()
state_1 (-> ['TMP_4725'])(CoreVaultClient.State) := TMP_4725(CoreVaultClient.State)
 (None,systemMinCrBIPS) = AgentCollateral.mintingMinCollateralRatio(_agent,_data.kind)
REF_3222(Collateral.Kind) -> _data_1.kind
TUPLE_54(uint256,uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.mintingMinCollateralRatio(Agent.State,Collateral.Kind), arguments:['_agent_1 (-> [])', 'REF_3222'] 
systemMinCrBIPS_1(uint256)= UNPACK TUPLE_54 index: 1 
 collateralEquivAMG = Conversion.convertTokenWeiToAMG(_data.fullCollateral,_data.amgToTokenWeiPrice)
REF_3224(uint256) -> _data_1.fullCollateral
REF_3225(uint256) -> _data_1.amgToTokenWeiPrice
TMP_4726(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertTokenWeiToAMG(uint256,uint256), arguments:['REF_3224', 'REF_3225'] 
collateralEquivAMG_1(uint256) := TMP_4726(uint256)
 maxSupportedAMG = collateralEquivAMG.mulDiv(SafePct.MAX_BIPS,systemMinCrBIPS)
REF_3227(uint256) -> SafePct.MAX_BIPS
TMP_4727(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['collateralEquivAMG_1', 'REF_3227', 'systemMinCrBIPS_1'] 
maxSupportedAMG_1(uint256) := TMP_4727(uint256)
 maxSupportedAMG.mulBips(state.minimumAmountLeftBIPS)
REF_3229(uint16) -> state_1 (-> ['TMP_4725']).minimumAmountLeftBIPS
TMP_4728(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['maxSupportedAMG_1', 'REF_3229'] 
RETURN TMP_4728
```
#### ERC20._burn(address,uint256) [INTERNAL]
```slithir
_balances_9(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
_totalSupply_5(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 require(bool,string)(account != address(0),ERC20: burn from the zero address)
TMP_216 = CONVERT 0 to address
TMP_217(bool) = account_1 != TMP_216
TMP_218(None) = SOLIDITY_CALL require(bool,string)(TMP_217,ERC20: burn from the zero address)
 _beforeTokenTransfer(account,address(0),amount)
TMP_219 = CONVERT 0 to address
INTERNAL_CALL, ERC20._beforeTokenTransfer(address,address,uint256)(account_1,TMP_219,amount_1)
 accountBalance = _balances[account]
REF_80(uint256) -> _balances_10[account_1]
accountBalance_1(uint256) := REF_80(uint256)
 require(bool,string)(accountBalance >= amount,ERC20: burn amount exceeds balance)
TMP_221(bool) = accountBalance_1 >= amount_1
TMP_222(None) = SOLIDITY_CALL require(bool,string)(TMP_221,ERC20: burn amount exceeds balance)
 _balances[account] = accountBalance - amount
REF_81(uint256) -> _balances_10[account_1]
TMP_223(uint256) = accountBalance_1 - amount_1
_balances_11(mapping(address => uint256)) := phi(['_balances_10'])
REF_81(uint256) (->_balances_11) := TMP_223(uint256)
 _totalSupply -= amount
_totalSupply_7(uint256) = _totalSupply_6 - amount_1
 Transfer(account,address(0),amount)
TMP_224 = CONVERT 0 to address
Emit Transfer(account_1,TMP_224,amount_1)
 _afterTokenTransfer(account,address(0),amount)
TMP_226 = CONVERT 0 to address
INTERNAL_CALL, ERC20._afterTokenTransfer(address,address,uint256)(account_1,TMP_226,amount_1)
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
#### Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data) [INTERNAL]
```slithir
_token_1 (-> [])(CollateralTypeInt.Data) := phi(['_toToken_1 (-> [])', '_fromToken_1 (-> [])'])
 (_price,None,None) = currentAmgPriceInTokenWeiWithTs(_token,false)
TUPLE_44(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool)(_token_1 (-> []),False)
_price_1(uint256)= UNPACK TUPLE_44 index: 0 
 _price
RETURN _price_1
```
#### ERC20.balanceOf(address) [PUBLIC]
```slithir
_balances_1(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
 _balances[account]
REF_73(uint256) -> _balances_1[account_1]
RETURN REF_73
```
#### ERC20.totalSupply() [PUBLIC]
```slithir
_totalSupply_1(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 _totalSupply
RETURN _totalSupply_1
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
#### AgentCollateral.freeCollateralWei(Collateral.Data,Agent.State) [INTERNAL]
```slithir
_data_1(Collateral.Data) := phi(['_data_1'])
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 lockedCollateral = lockedCollateralWei(_data,_agent)
TMP_4332(uint256) = INTERNAL_CALL, AgentCollateral.lockedCollateralWei(Collateral.Data,Agent.State)(_data_1,_agent_1 (-> []))
lockedCollateral_1(uint256) := TMP_4332(uint256)
 MathUtils.subOrZero(_data.fullCollateral,lockedCollateral)
REF_2848(uint256) -> _data_1.fullCollateral
TMP_4333(uint256) = LIBRARY_CALL, dest:MathUtils, function:MathUtils.subOrZero(uint256,uint256), arguments:['REF_2848', 'lockedCollateral_1'] 
RETURN TMP_4333
```
#### AgentCollateral.mintingLotCollateralWei(Collateral.Data,Agent.State,bool) [INTERNAL]
```slithir
_data_1(Collateral.Data) := phi(['_data_1'])
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_chargePoolFee_1(bool) := phi(['_chargePoolFee_1'])
 settings = Globals.getSettings()
TMP_4346(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4346'])(AssetManagerSettings.Data) := TMP_4346(AssetManagerSettings.Data)
 collateralRequiredToMintAmount(_data,_agent,settings.lotSizeAMG,_chargePoolFee)
REF_2868(uint64) -> settings_1 (-> ['TMP_4346']).lotSizeAMG
TMP_4347(uint256) = INTERNAL_CALL, AgentCollateral.collateralRequiredToMintAmount(Collateral.Data,Agent.State,uint256,bool)(_data_1,_agent_1 (-> []),REF_2868,_chargePoolFee_1)
RETURN TMP_4347
```
#### PaymentReference.redemption(uint256) [INTERNAL]
```slithir
MAX_ID_2(uint256) := phi(['MAX_ID_0'])
REDEMPTION_1(uint256) := phi(['REDEMPTION_0'])
 assert(bool)(_id <= MAX_ID)
TMP_5349(bool) = _id_1 <= MAX_ID_2
TMP_5350(None) = SOLIDITY_CALL assert(bool)(TMP_5349)
 bytes32(_id | REDEMPTION)
TMP_5351(uint256) = _id_1 | REDEMPTION_1
TMP_5352 = CONVERT TMP_5351 to bytes32
RETURN TMP_5352
```
#### RedemptionTimeExtension.extendTimeForRedemption(address) [INTERNAL]
```slithir
 state = getState()
TMP_5433(RedemptionTimeExtension.State) = INTERNAL_CALL, RedemptionTimeExtension.getState()()
state_1 (-> ['TMP_5433'])(RedemptionTimeExtension.State) := TMP_5433(RedemptionTimeExtension.State)
 agentData = state.agents[_agentVault]
REF_3850(mapping(address => RedemptionTimeExtension.AgentTimeExtensionData)) -> state_1 (-> ['TMP_5433']).agents
REF_3851(RedemptionTimeExtension.AgentTimeExtensionData) -> REF_3850[_agentVault_1]
agentData_1 (-> ['state'])(RedemptionTimeExtension.AgentTimeExtensionData) := REF_3851(RedemptionTimeExtension.AgentTimeExtensionData)
 timestamp = block.timestamp.toUint64()
TMP_5434(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.timestamp'] 
timestamp_1(uint64) := TMP_5434(uint64)
 accumulatedTimestamp = agentData.extendedTimestamp + state.redemptionPaymentExtensionSeconds
REF_3853(uint64) -> agentData_1 (-> ['state']).extendedTimestamp
REF_3854(uint64) -> state_1 (-> ['TMP_5433']).redemptionPaymentExtensionSeconds
TMP_5435(uint64) = REF_3853 (c)+ REF_3854
accumulatedTimestamp_1(uint64) := TMP_5435(uint64)
 agentData.extendedTimestamp = SafeMath64.max64(accumulatedTimestamp,timestamp)
REF_3855(uint64) -> agentData_1 (-> ['state']).extendedTimestamp
TMP_5436(uint64) = LIBRARY_CALL, dest:SafeMath64, function:SafeMath64.max64(uint64,uint64), arguments:['accumulatedTimestamp_1', 'timestamp_1'] 
agentData_2 (-> ['state'])(RedemptionTimeExtension.AgentTimeExtensionData) := phi(["agentData_1 (-> ['state'])"])
REF_3855(uint64) (->agentData_2 (-> ['state'])) := TMP_5436(uint64)
state_2 (-> ['state'])(RedemptionTimeExtension.State) := phi(["agentData_2 (-> ['state'])"])
 agentData.extendedTimestamp - timestamp
REF_3857(uint64) -> agentData_2 (-> ['state']).extendedTimestamp
TMP_5437(uint64) = REF_3857 (c)- timestamp_1
RETURN TMP_5437
```
#### RedemptionQueue.deleteRedemptionTicket(RedemptionQueue.State,uint64) [INTERNAL]
```slithir
 ticket = _state.tickets[_ticketId]
REF_3800(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_1 (-> []).tickets
REF_3801(RedemptionQueue.Ticket) -> REF_3800[_ticketId_1]
ticket_1 (-> ['_state'])(RedemptionQueue.Ticket) := REF_3801(RedemptionQueue.Ticket)
 assert(bool)(ticket.agentVault != address(0))
REF_3802(address) -> ticket_1 (-> ['_state']).agentVault
TMP_5410 = CONVERT 0 to address
TMP_5411(bool) = REF_3802 != TMP_5410
TMP_5412(None) = SOLIDITY_CALL assert(bool)(TMP_5411)
 agent = _state.agents[ticket.agentVault]
REF_3803(mapping(address => RedemptionQueue.AgentQueue)) -> _state_1 (-> []).agents
REF_3804(address) -> ticket_1 (-> ['_state']).agentVault
REF_3805(RedemptionQueue.AgentQueue) -> REF_3803[REF_3804]
agent_1 (-> ['_state'])(RedemptionQueue.AgentQueue) := REF_3805(RedemptionQueue.AgentQueue)
 ticket.prev == 0
REF_3806(uint64) -> ticket_1 (-> ['_state']).prev
TMP_5413(bool) = REF_3806 == 0
CONDITION TMP_5413
 assert(bool)(_ticketId == _state.firstTicketId)
REF_3807(uint64) -> _state_1 (-> []).firstTicketId
TMP_5414(bool) = _ticketId_1 == REF_3807
TMP_5415(None) = SOLIDITY_CALL assert(bool)(TMP_5414)
 _state.firstTicketId = ticket.next
REF_3808(uint64) -> _state_1 (-> []).firstTicketId
REF_3809(uint64) -> ticket_1 (-> ['_state']).next
_state_3 (-> [])(RedemptionQueue.State) := phi(['_state_1 (-> [])'])
REF_3808(uint64) (->_state_3 (-> [])) := REF_3809(uint64)
 assert(bool)(_ticketId != _state.firstTicketId)
REF_3810(uint64) -> _state_1 (-> []).firstTicketId
TMP_5416(bool) = _ticketId_1 != REF_3810
TMP_5417(None) = SOLIDITY_CALL assert(bool)(TMP_5416)
 _state.tickets[ticket.prev].next = ticket.next
REF_3811(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_1 (-> []).tickets
REF_3812(uint64) -> ticket_1 (-> ['_state']).prev
REF_3813(RedemptionQueue.Ticket) -> REF_3811[REF_3812]
REF_3814(uint64) -> REF_3813.next
REF_3815(uint64) -> ticket_1 (-> ['_state']).next
_state_2 (-> [])(RedemptionQueue.State) := phi(['_state_1 (-> [])'])
REF_3814(uint64) (->_state_2 (-> [])) := REF_3815(uint64)
_state_4 (-> [])(RedemptionQueue.State) := phi(['_state_3 (-> [])', '_state_2 (-> [])'])
 ticket.next == 0
REF_3816(uint64) -> ticket_1 (-> ['_state']).next
TMP_5418(bool) = REF_3816 == 0
CONDITION TMP_5418
 assert(bool)(_ticketId == _state.lastTicketId)
REF_3817(uint64) -> _state_4 (-> []).lastTicketId
TMP_5419(bool) = _ticketId_1 == REF_3817
TMP_5420(None) = SOLIDITY_CALL assert(bool)(TMP_5419)
 _state.lastTicketId = ticket.prev
REF_3818(uint64) -> _state_4 (-> []).lastTicketId
REF_3819(uint64) -> ticket_1 (-> ['_state']).prev
_state_6 (-> [])(RedemptionQueue.State) := phi(['_state_4 (-> [])'])
REF_3818(uint64) (->_state_6 (-> [])) := REF_3819(uint64)
 assert(bool)(_ticketId != _state.lastTicketId)
REF_3820(uint64) -> _state_4 (-> []).lastTicketId
TMP_5421(bool) = _ticketId_1 != REF_3820
TMP_5422(None) = SOLIDITY_CALL assert(bool)(TMP_5421)
 _state.tickets[ticket.next].prev = ticket.prev
REF_3821(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_4 (-> []).tickets
REF_3822(uint64) -> ticket_1 (-> ['_state']).next
REF_3823(RedemptionQueue.Ticket) -> REF_3821[REF_3822]
REF_3824(uint64) -> REF_3823.prev
REF_3825(uint64) -> ticket_1 (-> ['_state']).prev
_state_5 (-> [])(RedemptionQueue.State) := phi(['_state_4 (-> [])'])
REF_3824(uint64) (->_state_5 (-> [])) := REF_3825(uint64)
_state_7 (-> [])(RedemptionQueue.State) := phi(['_state_5 (-> [])', '_state_6 (-> [])'])
 ticket.prevForAgent == 0
REF_3826(uint64) -> ticket_1 (-> ['_state']).prevForAgent
TMP_5423(bool) = REF_3826 == 0
CONDITION TMP_5423
 assert(bool)(_ticketId == agent.firstTicketId)
REF_3827(uint64) -> agent_1 (-> ['_state']).firstTicketId
TMP_5424(bool) = _ticketId_1 == REF_3827
TMP_5425(None) = SOLIDITY_CALL assert(bool)(TMP_5424)
 agent.firstTicketId = ticket.nextForAgent
REF_3828(uint64) -> agent_1 (-> ['_state']).firstTicketId
REF_3829(uint64) -> ticket_1 (-> ['_state']).nextForAgent
agent_2 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_1 (-> ['_state'])"])
REF_3828(uint64) (->agent_2 (-> ['_state'])) := REF_3829(uint64)
_state_12 (-> ['_state'])(RedemptionQueue.State) := phi(["agent_2 (-> ['_state'])"])
 assert(bool)(_ticketId != agent.firstTicketId)
REF_3830(uint64) -> agent_1 (-> ['_state']).firstTicketId
TMP_5426(bool) = _ticketId_1 != REF_3830
TMP_5427(None) = SOLIDITY_CALL assert(bool)(TMP_5426)
 _state.tickets[ticket.prevForAgent].nextForAgent = ticket.nextForAgent
REF_3831(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_7 (-> []).tickets
REF_3832(uint64) -> ticket_1 (-> ['_state']).prevForAgent
REF_3833(RedemptionQueue.Ticket) -> REF_3831[REF_3832]
REF_3834(uint64) -> REF_3833.nextForAgent
REF_3835(uint64) -> ticket_1 (-> ['_state']).nextForAgent
_state_8 (-> [])(RedemptionQueue.State) := phi(['_state_7 (-> [])'])
REF_3834(uint64) (->_state_8 (-> [])) := REF_3835(uint64)
_state_9 (-> [])(RedemptionQueue.State) := phi(['_state_8 (-> [])', '_state_1 (-> [])'])
agent_3 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_1 (-> ['_state'])", "agent_2 (-> ['_state'])"])
 ticket.nextForAgent == 0
REF_3836(uint64) -> ticket_1 (-> ['_state']).nextForAgent
TMP_5428(bool) = REF_3836 == 0
CONDITION TMP_5428
 assert(bool)(_ticketId == agent.lastTicketId)
REF_3837(uint64) -> agent_3 (-> ['_state']).lastTicketId
TMP_5429(bool) = _ticketId_1 == REF_3837
TMP_5430(None) = SOLIDITY_CALL assert(bool)(TMP_5429)
 agent.lastTicketId = ticket.prevForAgent
REF_3838(uint64) -> agent_3 (-> ['_state']).lastTicketId
REF_3839(uint64) -> ticket_1 (-> ['_state']).prevForAgent
agent_4 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_3 (-> ['_state'])"])
REF_3838(uint64) (->agent_4 (-> ['_state'])) := REF_3839(uint64)
_state_13 (-> ['_state'])(RedemptionQueue.State) := phi(["agent_4 (-> ['_state'])"])
 assert(bool)(_ticketId != agent.lastTicketId)
REF_3840(uint64) -> agent_3 (-> ['_state']).lastTicketId
TMP_5431(bool) = _ticketId_1 != REF_3840
TMP_5432(None) = SOLIDITY_CALL assert(bool)(TMP_5431)
 _state.tickets[ticket.nextForAgent].prevForAgent = ticket.prevForAgent
REF_3841(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_9 (-> []).tickets
REF_3842(uint64) -> ticket_1 (-> ['_state']).nextForAgent
REF_3843(RedemptionQueue.Ticket) -> REF_3841[REF_3842]
REF_3844(uint64) -> REF_3843.prevForAgent
REF_3845(uint64) -> ticket_1 (-> ['_state']).prevForAgent
_state_10 (-> [])(RedemptionQueue.State) := phi(['_state_9 (-> [])'])
REF_3844(uint64) (->_state_10 (-> [])) := REF_3845(uint64)
_state_11 (-> [])(RedemptionQueue.State) := phi(['_state_10 (-> [])', '_state_1 (-> [])'])
 delete _state.tickets[_ticketId]
REF_3846(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_11 (-> []).tickets
REF_3847(RedemptionQueue.Ticket) -> REF_3846[_ticketId_1]
REF_3846 = delete REF_3847
```
