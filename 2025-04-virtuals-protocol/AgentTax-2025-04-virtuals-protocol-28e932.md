### Storage layout (AgentTax) 

```text
assetToken address
taxToken address
router IRouter
treasury address
feeRate uint16
minSwapThreshold uint256
maxSwapThreshold uint256
agentNft IAgentNft
_agentTba mapping(uint256 => address)
taxHistory mapping(bytes32 => AgentTax.TaxHistory)
agentTaxAmounts mapping(uint256 => AgentTax.TaxAmounts)
_agentRecipients mapping(uint256 => AgentTax.TaxRecipient)
creatorFeeRate uint16
tbaBonus ITBABonus

```






#### AgentTax._getTaxRecipient(uint256) [INTERNAL]
```slithir
agentId_1(uint256) := phi(['agentId_1'])
agentNft_2(IAgentNft) := phi(['agentNft_1', 'agentNft_3', 'agentNft_5', 'agentNft_6', 'agentNft_0'])
_agentRecipients_1(mapping(uint256 => AgentTax.TaxRecipient)) := phi(['_agentRecipients_1', '_agentRecipients_0', '_agentRecipients_6', '_agentRecipients_3'])
 recipient = _agentRecipients[agentId]
REF_4428(AgentTax.TaxRecipient) -> _agentRecipients_1[agentId_1]
recipient_1 (-> ['_agentRecipients'])(AgentTax.TaxRecipient) := REF_4428(AgentTax.TaxRecipient)
 recipient.tba == address(0)
REF_4429(address) -> recipient_1 (-> ['_agentRecipients']).tba
TMP_10868 = CONVERT 0 to address
TMP_10869(bool) = REF_4429 == TMP_10868
CONDITION TMP_10869
 info = agentNft.virtualInfo(agentId)
TMP_10870(IAgentNft.VirtualInfo) = HIGH_LEVEL_CALL, dest:agentNft_2(IAgentNft), function:virtualInfo, arguments:['agentId_1']  
agentNft_3(IAgentNft) := phi(['agentNft_1', 'agentNft_3', 'agentNft_5', 'agentNft_6', 'agentNft_2'])
info_1(IAgentNft.VirtualInfo) := TMP_10870(IAgentNft.VirtualInfo)
 recipient.tba = info.tba
REF_4431(address) -> recipient_1 (-> ['_agentRecipients']).tba
REF_4432(address) -> info_1.tba
recipient_2 (-> ['_agentRecipients'])(AgentTax.TaxRecipient) := phi(["recipient_1 (-> ['_agentRecipients'])"])
REF_4431(address) (->recipient_2 (-> ['_agentRecipients'])) := REF_4432(address)
_agentRecipients_2(mapping(uint256 => AgentTax.TaxRecipient)) := phi(["recipient_2 (-> ['_agentRecipients'])"])
 recipient.creator = info.founder
REF_4433(address) -> recipient_2 (-> ['_agentRecipients']).creator
REF_4434(address) -> info_1.founder
recipient_3 (-> ['_agentRecipients'])(AgentTax.TaxRecipient) := phi(["recipient_2 (-> ['_agentRecipients'])"])
REF_4433(address) (->recipient_3 (-> ['_agentRecipients'])) := REF_4434(address)
_agentRecipients_3(mapping(uint256 => AgentTax.TaxRecipient)) := phi(["recipient_3 (-> ['_agentRecipients'])"])
recipient_4 (-> ['_agentRecipients'])(AgentTax.TaxRecipient) := phi(["recipient_1 (-> ['_agentRecipients'])", "recipient_3 (-> ['_agentRecipients'])"])
 recipient
RETURN recipient_4 (-> ['_agentRecipients'])
```
#### AgentTax._swapForAsset(uint256,uint256,uint256) [INTERNAL]
```slithir
agentId_1(uint256) := phi(['agentId_1', 'agentId_1'])
minOutput_1(uint256) := phi(['minOutput_1', 'minOutput_1'])
maxOverride_1(uint256) := phi(['maxSwapThreshold_6', 'maxOverride_1'])
DENOM_3(uint256) := phi(['DENOM_2', 'DENOM_5', 'DENOM_7', 'DENOM_9', 'DENOM_0'])
assetToken_5(address) := phi(['assetToken_0', 'assetToken_10', 'assetToken_1', 'assetToken_9', 'assetToken_4', 'assetToken_7'])
taxToken_4(address) := phi(['taxToken_0', 'taxToken_6', 'taxToken_3', 'taxToken_1'])
router_5(IRouter) := phi(['router_4', 'router_0', 'router_1', 'router_9', 'router_7'])
treasury_8(address) := phi(['treasury_10', 'treasury_7', 'treasury_4', 'treasury_0', 'treasury_1', 'treasury_13', 'treasury_12'])
feeRate_5(uint16) := phi(['feeRate_9', 'feeRate_7', 'feeRate_4', 'feeRate_1', 'feeRate_0'])
minSwapThreshold_5(uint256) := phi(['minSwapThreshold_1', 'minSwapThreshold_4', 'minSwapThreshold_7', 'minSwapThreshold_0'])
agentTaxAmounts_4(mapping(uint256 => AgentTax.TaxAmounts)) := phi(['agentTaxAmounts_4', 'agentTaxAmounts_7', 'agentTaxAmounts_3', 'agentTaxAmounts_5', 'agentTaxAmounts_0'])
tbaBonus_1(ITBABonus) := phi(['tbaBonus_7', 'tbaBonus_0', 'tbaBonus_5', 'tbaBonus_3', 'tbaBonus_6'])
 agentAmounts = agentTaxAmounts[agentId]
REF_4435(AgentTax.TaxAmounts) -> agentTaxAmounts_4[agentId_1]
agentAmounts_1 (-> ['agentTaxAmounts'])(AgentTax.TaxAmounts) := REF_4435(AgentTax.TaxAmounts)
 amountToSwap = agentAmounts.amountCollected - agentAmounts.amountSwapped
REF_4436(uint256) -> agentAmounts_1 (-> ['agentTaxAmounts']).amountCollected
REF_4437(uint256) -> agentAmounts_1 (-> ['agentTaxAmounts']).amountSwapped
TMP_10871(uint256) = REF_4436 (c)- REF_4437
amountToSwap_1(uint256) := TMP_10871(uint256)
 balance = IERC20(taxToken).balanceOf(address(this))
TMP_10872 = CONVERT taxToken_4 to IERC20
TMP_10873 = CONVERT this to address
TMP_10874(uint256) = HIGH_LEVEL_CALL, dest:TMP_10872(IERC20), function:balanceOf, arguments:['TMP_10873']  
DENOM_4(uint256) := phi(['DENOM_2', 'DENOM_5', 'DENOM_7', 'DENOM_9', 'DENOM_3'])
assetToken_6(address) := phi(['assetToken_10', 'assetToken_1', 'assetToken_9', 'assetToken_4', 'assetToken_7', 'assetToken_5'])
taxToken_5(address) := phi(['taxToken_4', 'taxToken_6', 'taxToken_3', 'taxToken_1'])
router_6(IRouter) := phi(['router_4', 'router_1', 'router_9', 'router_5', 'router_7'])
treasury_9(address) := phi(['treasury_10', 'treasury_7', 'treasury_4', 'treasury_1', 'treasury_13', 'treasury_8', 'treasury_12'])
feeRate_6(uint16) := phi(['feeRate_9', 'feeRate_5', 'feeRate_7', 'feeRate_4', 'feeRate_1'])
minSwapThreshold_6(uint256) := phi(['minSwapThreshold_1', 'minSwapThreshold_5', 'minSwapThreshold_4', 'minSwapThreshold_7'])
tbaBonus_2(ITBABonus) := phi(['tbaBonus_7', 'tbaBonus_5', 'tbaBonus_3', 'tbaBonus_1', 'tbaBonus_6'])
balance_1(uint256) := TMP_10874(uint256)
 require(bool,string)(balance >= amountToSwap,Insufficient balance)
TMP_10875(bool) = balance_1 >= amountToSwap_1
TMP_10876(None) = SOLIDITY_CALL require(bool,string)(TMP_10875,Insufficient balance)
 taxRecipient = _getTaxRecipient(agentId)
TMP_10877(AgentTax.TaxRecipient) = INTERNAL_CALL, AgentTax._getTaxRecipient(uint256)(agentId_1)
taxRecipient_1(AgentTax.TaxRecipient) := TMP_10877(AgentTax.TaxRecipient)
 require(bool,string)(taxRecipient.tba != address(0),Agent does not have TBA)
REF_4439(address) -> taxRecipient_1.tba
TMP_10878 = CONVERT 0 to address
TMP_10879(bool) = REF_4439 != TMP_10878
TMP_10880(None) = SOLIDITY_CALL require(bool,string)(TMP_10879,Agent does not have TBA)
 amountToSwap < minSwapThreshold
TMP_10881(bool) = amountToSwap_1 < minSwapThreshold_7
CONDITION TMP_10881
 (false,0)
RETURN False,0
 amountToSwap > maxOverride
TMP_10882(bool) = amountToSwap_1 > maxOverride_1
CONDITION TMP_10882
 amountToSwap = maxOverride
amountToSwap_2(uint256) := maxOverride_1(uint256)
amountToSwap_3(uint256) := phi(['amountToSwap_1', 'amountToSwap_2'])
 path = new address[](2)
TMP_10884(address[])  = new address[](2)
path_1(address[]) = ['TMP_10884(address[])']
 path[0] = taxToken
REF_4440(address) -> path_1[0]
path_2(address[]) := phi(['path_1'])
REF_4440(address) (->path_2) := taxToken_6(address)
 path[1] = assetToken
REF_4441(address) -> path_2[1]
path_3(address[]) := phi(['path_2'])
REF_4441(address) (->path_3) := assetToken_7(address)
 amountsOut = router.getAmountsOut(amountToSwap,path)
TMP_10885(uint256[]) = HIGH_LEVEL_CALL, dest:router_7(IRouter), function:getAmountsOut, arguments:['amountToSwap_3', 'path_3']  
DENOM_6(uint256) := phi(['DENOM_5', 'DENOM_2', 'DENOM_9', 'DENOM_7'])
assetToken_8(address) := phi(['assetToken_10', 'assetToken_1', 'assetToken_9', 'assetToken_4', 'assetToken_7'])
router_8(IRouter) := phi(['router_4', 'router_9', 'router_7', 'router_1'])
treasury_11(address) := phi(['treasury_10', 'treasury_7', 'treasury_4', 'treasury_1', 'treasury_13', 'treasury_12'])
feeRate_8(uint16) := phi(['feeRate_4', 'feeRate_9', 'feeRate_1', 'feeRate_7'])
tbaBonus_4(ITBABonus) := phi(['tbaBonus_5', 'tbaBonus_6', 'tbaBonus_3', 'tbaBonus_7'])
amountsOut_1(uint256[]) = ['TMP_10885(uint256[])']
 require(bool,string)(amountsOut.length > 1,Failed to fetch token price)
REF_4443 -> LENGTH amountsOut_1
TMP_10886(bool) = REF_4443 > 1
TMP_10887(None) = SOLIDITY_CALL require(bool,string)(TMP_10886,Failed to fetch token price)
 amounts = router.swapExactTokensForTokens(amountToSwap,minOutput,path,address(this),block.timestamp + 300)
TMP_10888 = CONVERT this to address
TMP_10889(uint256) = block.timestamp (c)+ 300
TMP_10890(uint256[]) = HIGH_LEVEL_CALL, dest:router_8(IRouter), function:swapExactTokensForTokens, arguments:['amountToSwap_3', 'minOutput_1', 'path_3', 'TMP_10888', 'TMP_10889']  
DENOM_7(uint256) := phi(['DENOM_2', 'DENOM_5', 'DENOM_7', 'DENOM_9', 'DENOM_6'])
assetToken_9(address) := phi(['assetToken_8', 'assetToken_10', 'assetToken_1', 'assetToken_9', 'assetToken_4', 'assetToken_7'])
router_9(IRouter) := phi(['router_4', 'router_1', 'router_8', 'router_9', 'router_7'])
treasury_12(address) := phi(['treasury_10', 'treasury_7', 'treasury_4', 'treasury_1', 'treasury_11', 'treasury_13', 'treasury_12'])
feeRate_9(uint16) := phi(['feeRate_9', 'feeRate_7', 'feeRate_4', 'feeRate_1', 'feeRate_8'])
tbaBonus_5(ITBABonus) := phi(['tbaBonus_7', 'tbaBonus_5', 'tbaBonus_3', 'tbaBonus_6', 'tbaBonus_4'])
amounts_1(uint256[]) = ['TMP_10890(uint256[])']
 assetReceived = amounts[1]
REF_4445(uint256) -> amounts_1[1]
assetReceived_1(uint256) := REF_4445(uint256)
 SwapExecuted(agentId,amountToSwap,assetReceived)
Emit SwapExecuted(agentId_1,amountToSwap_3,assetReceived_1)
 feeAmount = (assetReceived * feeRate) / DENOM
TMP_10892(uint256) = assetReceived_1 (c)* feeRate_9
TMP_10893(uint256) = TMP_10892 (c)/ DENOM_7
feeAmount_1(uint256) := TMP_10893(uint256)
 creatorFee = assetReceived - feeAmount
TMP_10894(uint256) = assetReceived_1 (c)- feeAmount_1
creatorFee_1(uint256) := TMP_10894(uint256)
 creatorFee > 0
TMP_10895(bool) = creatorFee_1 > 0
CONDITION TMP_10895
 IERC20(assetToken).safeTransfer(taxRecipient.creator,creatorFee)
TMP_10896 = CONVERT assetToken_9 to IERC20
REF_4447(address) -> taxRecipient_1.creator
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_10896', 'REF_4447', 'creatorFee_1'] 
 address(tbaBonus) != address(0)
TMP_10898 = CONVERT tbaBonus_5 to address
TMP_10899 = CONVERT 0 to address
TMP_10900(bool) = TMP_10898 != TMP_10899
CONDITION TMP_10900
 tbaBonus.distributeBonus(agentId,taxRecipient.creator,creatorFee)
REF_4449(address) -> taxRecipient_1.creator
HIGH_LEVEL_CALL, dest:tbaBonus_5(ITBABonus), function:distributeBonus, arguments:['agentId_1', 'REF_4449', 'creatorFee_1']  
assetToken_10(address) := phi(['assetToken_10', 'assetToken_1', 'assetToken_9', 'assetToken_4', 'assetToken_7'])
treasury_13(address) := phi(['treasury_10', 'treasury_7', 'treasury_4', 'treasury_1', 'treasury_13', 'treasury_12'])
tbaBonus_6(ITBABonus) := phi(['tbaBonus_5', 'tbaBonus_6', 'tbaBonus_3', 'tbaBonus_7'])
 feeAmount > 0
TMP_10902(bool) = feeAmount_1 > 0
CONDITION TMP_10902
 IERC20(assetToken).safeTransfer(treasury,feeAmount)
TMP_10903 = CONVERT assetToken_10 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_10903', 'treasury_13', 'feeAmount_1'] 
 agentAmounts.amountSwapped += amountToSwap
REF_4451(uint256) -> agentAmounts_1 (-> ['agentTaxAmounts']).amountSwapped
agentAmounts_2 (-> ['agentTaxAmounts'])(AgentTax.TaxAmounts) := phi(["agentAmounts_1 (-> ['agentTaxAmounts'])"])
REF_4451(-> agentAmounts_2 (-> ['agentTaxAmounts'])) = REF_4451 (c)+ amountToSwap_3
agentTaxAmounts_5(mapping(uint256 => AgentTax.TaxAmounts)) := phi(["agentAmounts_2 (-> ['agentTaxAmounts'])"])
 (true,amounts[1])
REF_4452(uint256) -> amounts_1[1]
RETURN True,REF_4452
 SwapFailed(agentId,amountToSwap)
Emit SwapFailed(agentId_1,amountToSwap_3)
 (false,0)
RETURN False,0
```
#### AgentTax.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### AgentTax.dcaSell(uint256[],uint256,uint256) [PUBLIC]
```slithir
EXECUTOR_ROLE_3(bytes32) := phi(['EXECUTOR_ROLE_2', 'EXECUTOR_ROLE_0', 'EXECUTOR_ROLE_4'])
DENOM_8(uint256) := phi(['DENOM_2', 'DENOM_5', 'DENOM_7', 'DENOM_9', 'DENOM_0'])
agentTaxAmounts_6(mapping(uint256 => AgentTax.TaxAmounts)) := phi(['agentTaxAmounts_4', 'agentTaxAmounts_7', 'agentTaxAmounts_3', 'agentTaxAmounts_5', 'agentTaxAmounts_0'])
 require(bool,string)(slippage <= DENOM,Invalid slippage)
TMP_10915(bool) = slippage_1 <= DENOM_9
TMP_10916(None) = SOLIDITY_CALL require(bool,string)(TMP_10915,Invalid slippage)
 i = 0
i_1(uint256) := 0(uint256)
 i < agentIds.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_4463 -> LENGTH agentIds_1
TMP_10917(bool) = i_2 < REF_4463
CONDITION TMP_10917
 agentId = agentIds[i]
REF_4464(uint256) -> agentIds_1[i_2]
agentId_1(uint256) := REF_4464(uint256)
 agentAmounts = agentTaxAmounts[agentId]
REF_4465(AgentTax.TaxAmounts) -> agentTaxAmounts_7[agentId_1]
agentAmounts_1(AgentTax.TaxAmounts) := REF_4465(AgentTax.TaxAmounts)
 amountToSwap = agentAmounts.amountCollected - agentAmounts.amountSwapped
REF_4466(uint256) -> agentAmounts_1.amountCollected
REF_4467(uint256) -> agentAmounts_1.amountSwapped
TMP_10918(uint256) = REF_4466 (c)- REF_4467
amountToSwap_1(uint256) := TMP_10918(uint256)
 amountToSwap > maxOverride
TMP_10919(bool) = amountToSwap_1 > maxOverride_1
CONDITION TMP_10919
 amountToSwap = maxOverride
amountToSwap_2(uint256) := maxOverride_1(uint256)
amountToSwap_3(uint256) := phi(['amountToSwap_2', 'amountToSwap_1'])
 minOutput = ((amountToSwap * (DENOM - slippage)) / DENOM)
TMP_10920(uint256) = DENOM_9 (c)- slippage_1
TMP_10921(uint256) = amountToSwap_3 (c)* TMP_10920
TMP_10922(uint256) = TMP_10921 (c)/ DENOM_9
minOutput_1(uint256) := TMP_10922(uint256)
 _swapForAsset(agentId,minOutput,maxOverride)
TUPLE_115(bool,uint256) = INTERNAL_CALL, AgentTax._swapForAsset(uint256,uint256,uint256)(agentId_1,minOutput_1,maxOverride_1)
DENOM_10(uint256) := phi(['DENOM_7', 'DENOM_4'])
agentTaxAmounts_8(mapping(uint256 => AgentTax.TaxAmounts)) := phi(['agentTaxAmounts_4', 'agentTaxAmounts_5'])
 i ++
TMP_10923(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 onlyRole(EXECUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(EXECUTOR_ROLE_3)
```
#### AgentTax.handleAgentTaxes(uint256,bytes32[],uint256[],uint256) [PUBLIC]
```slithir
EXECUTOR_ROLE_1(bytes32) := phi(['EXECUTOR_ROLE_2', 'EXECUTOR_ROLE_0', 'EXECUTOR_ROLE_4'])
maxSwapThreshold_5(uint256) := phi(['maxSwapThreshold_1', 'maxSwapThreshold_0', 'maxSwapThreshold_4', 'maxSwapThreshold_7'])
taxHistory_1(mapping(bytes32 => AgentTax.TaxHistory)) := phi(['taxHistory_0', 'taxHistory_2'])
agentTaxAmounts_1(mapping(uint256 => AgentTax.TaxAmounts)) := phi(['agentTaxAmounts_4', 'agentTaxAmounts_7', 'agentTaxAmounts_3', 'agentTaxAmounts_5', 'agentTaxAmounts_0'])
 require(bool,string)(txhashes.length == amounts.length,Unmatched inputs)
REF_4416 -> LENGTH txhashes_1
REF_4417 -> LENGTH amounts_1
TMP_10859(bool) = REF_4416 == REF_4417
TMP_10860(None) = SOLIDITY_CALL require(bool,string)(TMP_10859,Unmatched inputs)
 agentAmounts = agentTaxAmounts[agentId]
REF_4418(AgentTax.TaxAmounts) -> agentTaxAmounts_2[agentId_1]
agentAmounts_1 (-> ['agentTaxAmounts'])(AgentTax.TaxAmounts) := REF_4418(AgentTax.TaxAmounts)
 totalAmount = 0
totalAmount_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < txhashes.length
totalAmount_2(uint256) := phi(['totalAmount_1', 'totalAmount_3'])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4419 -> LENGTH txhashes_1
TMP_10861(bool) = i_2 < REF_4419
CONDITION TMP_10861
 txhash = txhashes[i]
REF_4420(bytes32) -> txhashes_1[i_2]
txhash_1(bytes32) := REF_4420(bytes32)
 taxHistory[txhash].agentId > 0
REF_4421(AgentTax.TaxHistory) -> taxHistory_2[txhash_1]
REF_4422(uint256) -> REF_4421.agentId
TMP_10862(bool) = REF_4422 > 0
CONDITION TMP_10862
 revert TxHashExists(bytes32)(txhash)
TMP_10863(None) = SOLIDITY_CALL revert TxHashExists(bytes32)(txhash_1)
 taxHistory[txhash] = TaxHistory(agentId,amounts[i])
REF_4423(AgentTax.TaxHistory) -> taxHistory_2[txhash_1]
REF_4424(uint256) -> amounts_1[i_2]
TMP_10864(AgentTax.TaxHistory) = new TaxHistory(agentId_1,REF_4424)
taxHistory_3(mapping(bytes32 => AgentTax.TaxHistory)) := phi(['taxHistory_2'])
REF_4423(AgentTax.TaxHistory) (->taxHistory_3) := TMP_10864(AgentTax.TaxHistory)
 totalAmount += amounts[i]
REF_4425(uint256) -> amounts_1[i_2]
totalAmount_3(uint256) = totalAmount_2 (c)+ REF_4425
 TaxCollected(txhash,agentId,amounts[i])
REF_4426(uint256) -> amounts_1[i_2]
Emit TaxCollected(txhash_1,agentId_1,REF_4426)
 i ++
TMP_10866(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 agentAmounts.amountCollected += totalAmount
REF_4427(uint256) -> agentAmounts_1 (-> ['agentTaxAmounts']).amountCollected
agentAmounts_2 (-> ['agentTaxAmounts'])(AgentTax.TaxAmounts) := phi(["agentAmounts_1 (-> ['agentTaxAmounts'])"])
REF_4427(-> agentAmounts_2 (-> ['agentTaxAmounts'])) = REF_4427 (c)+ totalAmount_2
agentTaxAmounts_3(mapping(uint256 => AgentTax.TaxAmounts)) := phi(["agentAmounts_2 (-> ['agentTaxAmounts'])"])
 _swapForAsset(agentId,minOutput,maxSwapThreshold)
TUPLE_114(bool,uint256) = INTERNAL_CALL, AgentTax._swapForAsset(uint256,uint256,uint256)(agentId_1,minOutput_1,maxSwapThreshold_6)
 onlyRole(EXECUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(EXECUTOR_ROLE_1)
```
#### AgentTax.initialize(address,address,address,address,address,uint256,uint256,address) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_5', 'DEFAULT_ADMIN_ROLE_0'])
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10', 'ADMIN_ROLE_4', 'ADMIN_ROLE_16', 'ADMIN_ROLE_12', 'ADMIN_ROLE_6'])
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 require(bool,string)(assetToken_ != taxToken_,Asset token cannot be same as tax token)
TMP_10821(bool) = assetToken__1 != taxToken__1
TMP_10822(None) = SOLIDITY_CALL require(bool,string)(TMP_10821,Asset token cannot be same as tax token)
 _grantRole(ADMIN_ROLE,defaultAdmin_)
TMP_10823(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(ADMIN_ROLE_3,defaultAdmin__1)
 _grantRole(DEFAULT_ADMIN_ROLE,defaultAdmin_)
TMP_10824(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_4,defaultAdmin__1)
 assetToken = assetToken_
assetToken_1(address) := assetToken__1(address)
 taxToken = taxToken_
taxToken_1(address) := taxToken__1(address)
 router = IRouter(router_)
TMP_10825 = CONVERT router__1 to IRouter
router_1(IRouter) := TMP_10825(IRouter)
 treasury = treasury_
treasury_1(address) := treasury__1(address)
 minSwapThreshold = minSwapThreshold_
minSwapThreshold_1(uint256) := minSwapThreshold__1(uint256)
 maxSwapThreshold = maxSwapThreshold_
maxSwapThreshold_1(uint256) := maxSwapThreshold__1(uint256)
 IERC20(taxToken).forceApprove(router_,type()(uint256).max)
TMP_10826 = CONVERT taxToken_1 to IERC20
TMP_10828(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.forceApprove(IERC20,address,uint256), arguments:['TMP_10826', 'router__1', 'TMP_10828'] 
 agentNft = IAgentNft(nft_)
TMP_10830 = CONVERT nft__1 to IAgentNft
agentNft_1(IAgentNft) := TMP_10830(IAgentNft)
 feeRate = 100
feeRate_1(uint16) := 100(uint256)
 creatorFeeRate = 3000
creatorFeeRate_1(uint16) := 3000(uint256)
 SwapParamsUpdated2(address(0),router_,address(0),assetToken_,0,feeRate,0,creatorFeeRate)
TMP_10831 = CONVERT 0 to address
TMP_10832 = CONVERT 0 to address
Emit SwapParamsUpdated2(TMP_10831,router__1,TMP_10832,assetToken__1,0,feeRate_1,0,creatorFeeRate_1)
 SwapThresholdUpdated(0,minSwapThreshold_,0,maxSwapThreshold_)
Emit SwapThresholdUpdated(0,minSwapThreshold__1,0,maxSwapThreshold__1)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```

#### AgentTax.updateCreator(uint256,address) [PUBLIC]
```slithir
ADMIN_ROLE_13(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10', 'ADMIN_ROLE_4', 'ADMIN_ROLE_16', 'ADMIN_ROLE_12', 'ADMIN_ROLE_6'])
agentNft_4(IAgentNft) := phi(['agentNft_1', 'agentNft_3', 'agentNft_5', 'agentNft_6', 'agentNft_0'])
_agentRecipients_4(mapping(uint256 => AgentTax.TaxRecipient)) := phi(['_agentRecipients_1', '_agentRecipients_0', '_agentRecipients_6', '_agentRecipients_3'])
 sender = _msgSender()
TMP_10906(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
sender_1(address) := TMP_10906(address)
 recipient = _agentRecipients[agentId]
REF_4453(AgentTax.TaxRecipient) -> _agentRecipients_5[agentId_1]
recipient_1 (-> ['_agentRecipients'])(AgentTax.TaxRecipient) := REF_4453(AgentTax.TaxRecipient)
 recipient.tba == address(0)
REF_4454(address) -> recipient_1 (-> ['_agentRecipients']).tba
TMP_10907 = CONVERT 0 to address
TMP_10908(bool) = REF_4454 == TMP_10907
CONDITION TMP_10908
 info = agentNft.virtualInfo(agentId)
TMP_10909(IAgentNft.VirtualInfo) = HIGH_LEVEL_CALL, dest:agentNft_5(IAgentNft), function:virtualInfo, arguments:['agentId_1']  
ADMIN_ROLE_15(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10', 'ADMIN_ROLE_14', 'ADMIN_ROLE_4', 'ADMIN_ROLE_16', 'ADMIN_ROLE_12', 'ADMIN_ROLE_6'])
agentNft_6(IAgentNft) := phi(['agentNft_6', 'agentNft_1', 'agentNft_3', 'agentNft_5'])
info_1(IAgentNft.VirtualInfo) := TMP_10909(IAgentNft.VirtualInfo)
 recipient.tba = info.tba
REF_4456(address) -> recipient_1 (-> ['_agentRecipients']).tba
REF_4457(address) -> info_1.tba
recipient_2 (-> ['_agentRecipients'])(AgentTax.TaxRecipient) := phi(["recipient_1 (-> ['_agentRecipients'])"])
REF_4456(address) (->recipient_2 (-> ['_agentRecipients'])) := REF_4457(address)
_agentRecipients_7(mapping(uint256 => AgentTax.TaxRecipient)) := phi(["recipient_2 (-> ['_agentRecipients'])"])
 recipient.creator = info.founder
REF_4458(address) -> recipient_2 (-> ['_agentRecipients']).creator
REF_4459(address) -> info_1.founder
recipient_3 (-> ['_agentRecipients'])(AgentTax.TaxRecipient) := phi(["recipient_2 (-> ['_agentRecipients'])"])
REF_4458(address) (->recipient_3 (-> ['_agentRecipients'])) := REF_4459(address)
_agentRecipients_8(mapping(uint256 => AgentTax.TaxRecipient)) := phi(["recipient_3 (-> ['_agentRecipients'])"])
recipient_4 (-> ['_agentRecipients'])(AgentTax.TaxRecipient) := phi(["recipient_3 (-> ['_agentRecipients'])", "recipient_1 (-> ['_agentRecipients'])"])
 oldCreator = recipient.creator
REF_4460(address) -> recipient_4 (-> ['_agentRecipients']).creator
oldCreator_1(address) := REF_4460(address)
 require(bool,string)(sender == recipient.creator || hasRole(ADMIN_ROLE,sender),Only creator can update)
REF_4461(address) -> recipient_4 (-> ['_agentRecipients']).creator
TMP_10910(bool) = sender_1 == REF_4461
TMP_10911(bool) = INTERNAL_CALL, AccessControlUpgradeable.hasRole(bytes32,address)(ADMIN_ROLE_15,sender_1)
TMP_10912(bool) = TMP_10910 || TMP_10911
TMP_10913(None) = SOLIDITY_CALL require(bool,string)(TMP_10912,Only creator can update)
 recipient.creator = creator
REF_4462(address) -> recipient_4 (-> ['_agentRecipients']).creator
recipient_5 (-> ['_agentRecipients'])(AgentTax.TaxRecipient) := phi(["recipient_4 (-> ['_agentRecipients'])"])
REF_4462(address) (->recipient_5 (-> ['_agentRecipients'])) := creator_1(address)
_agentRecipients_6(mapping(uint256 => AgentTax.TaxRecipient)) := phi(["recipient_5 (-> ['_agentRecipients'])"])
 CreatorUpdated(agentId,oldCreator,creator)
Emit CreatorUpdated(agentId_1,oldCreator_1,creator_1)
```
#### AgentTax.updateSwapParams(address,address,uint16,uint16) [PUBLIC]
```slithir
ADMIN_ROLE_5(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10', 'ADMIN_ROLE_4', 'ADMIN_ROLE_16', 'ADMIN_ROLE_12', 'ADMIN_ROLE_6'])
DENOM_1(uint256) := phi(['DENOM_2', 'DENOM_5', 'DENOM_7', 'DENOM_9', 'DENOM_0'])
assetToken_2(address) := phi(['assetToken_0', 'assetToken_10', 'assetToken_1', 'assetToken_9', 'assetToken_4', 'assetToken_7'])
taxToken_2(address) := phi(['taxToken_0', 'taxToken_6', 'taxToken_3', 'taxToken_1'])
router_2(IRouter) := phi(['router_4', 'router_0', 'router_1', 'router_9', 'router_7'])
feeRate_2(uint16) := phi(['feeRate_9', 'feeRate_7', 'feeRate_4', 'feeRate_1', 'feeRate_0'])
creatorFeeRate_2(uint16) := phi(['creatorFeeRate_4', 'creatorFeeRate_0', 'creatorFeeRate_1'])
 require(bool,string)((feeRate_ + creatorFeeRate_) == DENOM,Invalid fee rates)
TMP_10836(uint16) = feeRate__1 (c)+ creatorFeeRate__1
TMP_10837(bool) = TMP_10836 == DENOM_2
TMP_10838(None) = SOLIDITY_CALL require(bool,string)(TMP_10837,Invalid fee rates)
 oldRouter = address(router)
TMP_10839 = CONVERT router_3 to address
oldRouter_1(address) := TMP_10839(address)
 oldAsset = assetToken
oldAsset_1(address) := assetToken_3(address)
 oldFee = feeRate
oldFee_1(uint16) := feeRate_3(uint16)
 oldCreatorFee = creatorFeeRate
oldCreatorFee_1(uint16) := creatorFeeRate_3(uint16)
 assetToken = assetToken_
assetToken_4(address) := assetToken__1(address)
 router = IRouter(router_)
TMP_10840 = CONVERT router__1 to IRouter
router_4(IRouter) := TMP_10840(IRouter)
 feeRate = feeRate_
feeRate_4(uint16) := feeRate__1(uint16)
 creatorFeeRate = creatorFeeRate_
creatorFeeRate_4(uint16) := creatorFeeRate__1(uint16)
 IERC20(taxToken).forceApprove(oldRouter,0)
TMP_10841 = CONVERT taxToken_3 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.forceApprove(IERC20,address,uint256), arguments:['TMP_10841', 'oldRouter_1', '0'] 
 IERC20(taxToken).forceApprove(router_,type()(uint256).max)
TMP_10843 = CONVERT taxToken_3 to IERC20
TMP_10845(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.forceApprove(IERC20,address,uint256), arguments:['TMP_10843', 'router__1', 'TMP_10845'] 
 SwapParamsUpdated2(oldRouter,router_,oldAsset,assetToken_,oldFee,feeRate_,oldCreatorFee,creatorFeeRate)
Emit SwapParamsUpdated2(oldRouter_1,router__1,oldAsset_1,assetToken__1,oldFee_1,feeRate__1,oldCreatorFee_1,creatorFeeRate_4)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_5)
```
#### AgentTax.updateSwapThresholds(uint256,uint256) [PUBLIC]
```slithir
ADMIN_ROLE_7(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10', 'ADMIN_ROLE_4', 'ADMIN_ROLE_16', 'ADMIN_ROLE_12', 'ADMIN_ROLE_6'])
minSwapThreshold_2(uint256) := phi(['minSwapThreshold_1', 'minSwapThreshold_4', 'minSwapThreshold_7', 'minSwapThreshold_0'])
maxSwapThreshold_2(uint256) := phi(['maxSwapThreshold_1', 'maxSwapThreshold_0', 'maxSwapThreshold_4', 'maxSwapThreshold_7'])
 oldMin = minSwapThreshold
oldMin_1(uint256) := minSwapThreshold_3(uint256)
 oldMax = maxSwapThreshold
oldMax_1(uint256) := maxSwapThreshold_3(uint256)
 minSwapThreshold = minSwapThreshold_
minSwapThreshold_4(uint256) := minSwapThreshold__1(uint256)
 maxSwapThreshold = maxSwapThreshold_
maxSwapThreshold_4(uint256) := maxSwapThreshold__1(uint256)
 SwapThresholdUpdated(oldMin,minSwapThreshold_,oldMax,maxSwapThreshold_)
Emit SwapThresholdUpdated(oldMin_1,minSwapThreshold__1,oldMax_1,maxSwapThreshold__1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_7)
```
#### AgentTax.updateTbaBonus(address) [PUBLIC]
```slithir
ADMIN_ROLE_17(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10', 'ADMIN_ROLE_4', 'ADMIN_ROLE_16', 'ADMIN_ROLE_12', 'ADMIN_ROLE_6'])
 tbaBonus = ITBABonus(tbaBonus_)
TMP_10925 = CONVERT tbaBonus__1 to ITBABonus
tbaBonus_7(ITBABonus) := TMP_10925(ITBABonus)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_17)
```
#### AgentTax.updateTreasury(address) [PUBLIC]
```slithir
ADMIN_ROLE_9(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10', 'ADMIN_ROLE_4', 'ADMIN_ROLE_16', 'ADMIN_ROLE_12', 'ADMIN_ROLE_6'])
treasury_2(address) := phi(['treasury_10', 'treasury_7', 'treasury_4', 'treasury_0', 'treasury_1', 'treasury_13', 'treasury_12'])
 oldTreasury = treasury
oldTreasury_1(address) := treasury_3(address)
 treasury = treasury_
treasury_4(address) := treasury__1(address)
 TreasuryUpdated(oldTreasury,treasury_)
Emit TreasuryUpdated(oldTreasury_1,treasury__1)
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_9)
```
#### AgentTax.withdraw(address) [EXTERNAL]
```slithir
ADMIN_ROLE_11(bytes32) := phi(['ADMIN_ROLE_8', 'ADMIN_ROLE_0', 'ADMIN_ROLE_18', 'ADMIN_ROLE_10', 'ADMIN_ROLE_4', 'ADMIN_ROLE_16', 'ADMIN_ROLE_12', 'ADMIN_ROLE_6'])
treasury_5(address) := phi(['treasury_10', 'treasury_7', 'treasury_4', 'treasury_0', 'treasury_1', 'treasury_13', 'treasury_12'])
 IERC20(token).safeTransfer(treasury,IERC20(token).balanceOf(address(this)))
TMP_10853 = CONVERT token_1 to IERC20
TMP_10854 = CONVERT token_1 to IERC20
TMP_10855 = CONVERT this to address
TMP_10856(uint256) = HIGH_LEVEL_CALL, dest:TMP_10854(IERC20), function:balanceOf, arguments:['TMP_10855']  
treasury_7(address) := phi(['treasury_10', 'treasury_6', 'treasury_7', 'treasury_4', 'treasury_1', 'treasury_13', 'treasury_12'])
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_10853', 'treasury_7', 'TMP_10856'] 
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_11)
```
#### IAgentNft.virtualInfo(uint256) [EXTERNAL]
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
#### AeroAdaptor.getAmountsOut(uint256,address[]) [EXTERNAL]
```slithir
router_4(address) := phi(['router_1', 'router_3', 'router_5', 'router_0'])
tokenIn_3(address) := phi(['tokenIn_1', 'tokenIn_0'])
tokenOut_3(address) := phi(['tokenOut_1', 'tokenOut_0'])
factory_3(address) := phi(['factory_1', 'factory_0'])
 routes = new IAeroRouter.Route[](1)
TMP_10755(IAeroRouter.Route[])  = new IAeroRouter.Route[](1)
routes_1(IAeroRouter.Route[]) = ['TMP_10755(IAeroRouter.Route[])']
 routes[0] = IAeroRouter.Route(tokenIn,tokenOut,false,factory)
REF_4383(IAeroRouter.Route) -> routes_1[0]
TMP_10756(IAeroRouter.Route) = new Route(tokenIn_3,tokenOut_3,False,factory_3)
routes_2(IAeroRouter.Route[]) := phi(['routes_1'])
REF_4383(IAeroRouter.Route) (->routes_2) := TMP_10756(IAeroRouter.Route)
 IAeroRouter(router).getAmountsOut(amountIn,routes)
TMP_10757 = CONVERT router_4 to IAeroRouter
TMP_10758(uint256[]) = HIGH_LEVEL_CALL, dest:TMP_10757(IAeroRouter), function:getAmountsOut, arguments:['amountIn_1', 'routes_2']  
router_5(address) := phi(['router_1', 'router_3', 'router_5', 'router_4'])
RETURN TMP_10758
 amounts
```
#### IRouter.swapExactTokensForTokens(uint256,uint256,address[],address,uint256) [EXTERNAL]
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
