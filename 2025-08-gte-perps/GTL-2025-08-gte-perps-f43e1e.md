




### Storage layout (GTL) 

```text
_subaccounts EnumerableSetLib.Uint256Set
_withdrawalQueue uint256[]
_withdrawalCounter uint256
_queuedWithdrawal mapping(uint256 => GTL.Withdrawal)
_queuedShares mapping(address => uint256)

```

#### GTL._afterTokenTransfer(address,address,uint256) [INTERNAL]
```slithir
from_1(address) := phi(['msg.sender', 'from_1', 'from_1', 'TMP_4495', 'from_1'])
_queuedShares_15(mapping(address => uint256)) := phi(['_queuedShares_5', '_queuedShares_14', '_queuedShares_0', '_queuedShares_10', '_queuedShares_3', '_queuedShares_16'])
 balanceOf(from) < _queuedShares[from]
TMP_4682(uint256) = INTERNAL_CALL, ERC20.balanceOf(address)(from_1)
REF_1473(uint256) -> _queuedShares_16[from_1]
TMP_4683(bool) = TMP_4682 < REF_1473
CONDITION TMP_4683
 revert InsufficientBalance()()
TMP_4684(None) = SOLIDITY_CALL revert InsufficientBalance()()
```
#### GTL._assertAdmin() [INTERNAL]
```slithir
ADMIN_ROLE_12(uint256) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_11', 'ADMIN_ROLE_14', 'ADMIN_ROLE_3', 'ADMIN_ROLE_9', 'ADMIN_ROLE_6'])
 ! hasAllRoles(msg.sender,ADMIN_ROLE) && msg.sender != owner()
TMP_4699(bool) = INTERNAL_CALL, OwnableRoles.hasAllRoles(address,uint256)(msg.sender,ADMIN_ROLE_12)
TMP_4700 = UnaryType.BANG TMP_4699 
TMP_4701(address) = INTERNAL_CALL, Ownable.owner()()
TMP_4702(bool) = msg.sender != TMP_4701
TMP_4703(bool) = TMP_4700 && TMP_4702
CONDITION TMP_4703
 revert NotAdmin()()
TMP_4704(None) = SOLIDITY_CALL revert NotAdmin()()
```
#### GTL._convertToAssets(uint256,uint256) [PUBLIC]
```slithir
shares_1(uint256) := phi(['REF_1442'])
allocatedAssets_1(uint256) := phi(['allocatedAssets_1'])
usdc_16(address) := phi(['usdc_15', 'usdc_9', 'usdc_1', 'usdc_3', 'usdc_17', 'usdc_0'])
 shares.fullMulDiv(usdc.balanceOf(address(this)) + allocatedAssets + 1,totalSupply() + 1)
TMP_4692 = CONVERT this to address
TMP_4693(uint256) = LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.balanceOf(address,address), arguments:['usdc_16', 'TMP_4692'] 
TMP_4694(uint256) = TMP_4693 (c)+ allocatedAssets_1
TMP_4695(uint256) = TMP_4694 (c)+ 1
TMP_4696(uint256) = INTERNAL_CALL, ERC20.totalSupply()()
TMP_4697(uint256) = TMP_4696 (c)+ 1
TMP_4698(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['shares_1', 'TMP_4695', 'TMP_4697'] 
RETURN TMP_4698
 assets
```
#### GTL._dequeue(uint256) [INTERNAL]
```slithir
id_1(uint256) := phi(['id_1'])
_withdrawalQueue_13(uint256[]) := phi(['_withdrawalQueue_0', '_withdrawalQueue_9', '_withdrawalQueue_14', '_withdrawalQueue_4', '_withdrawalQueue_16'])
 withdrawalQueue = _withdrawalQueue
withdrawalQueue_1(uint256[]) := _withdrawalQueue_13(uint256[])
 length = withdrawalQueue.length
REF_1474 -> LENGTH withdrawalQueue_1
length_1(uint256) := REF_1474(uint256)
 newQueue = new uint256[](length - 1)
TMP_4686(uint256) = length_1 (c)- 1
TMP_4687(uint256[])  = new uint256[](TMP_4686)
newQueue_1(uint256[]) = ['TMP_4687(uint256[])']
 i < length
i_1(uint256) := phi(['i_2', 'i_0'])
TMP_4688(bool) = i_1 < length_1
CONDITION TMP_4688
 withdrawalQueue[i] != id
REF_1475(uint256) -> withdrawalQueue_1[i_1]
TMP_4689(bool) = REF_1475 != id_1
CONDITION TMP_4689
 newQueue[idx ++] = withdrawalQueue[i]
TMP_4690(uint256) := idx_0(uint256)
idx_1(uint256) = idx_0 (c)+ 1
REF_1476(uint256) -> newQueue_1[TMP_4690]
REF_1477(uint256) -> withdrawalQueue_1[i_1]
newQueue_2(uint256[]) := phi(['newQueue_1'])
REF_1476(uint256) (->newQueue_2) := REF_1477(uint256)
idx_2(uint256) := phi(['idx_0', 'idx_1'])
 ++ i
i_2(uint256) = i_1 (c)+ 1
 _withdrawalQueue = newQueue
_withdrawalQueue_14(uint256[]) := newQueue_1(uint256[])
```
#### GTL._dequeueBatch(uint256) [INTERNAL]
```slithir
num_1(uint256) := phi(['num_1'])
_withdrawalQueue_15(uint256[]) := phi(['_withdrawalQueue_0', '_withdrawalQueue_9', '_withdrawalQueue_14', '_withdrawalQueue_4', '_withdrawalQueue_16'])
 withdrawalQueue = _withdrawalQueue
withdrawalQueue_1(uint256[]) := _withdrawalQueue_15(uint256[])
 _withdrawalQueue = withdrawalQueue.slice(num,withdrawalQueue.length)
REF_1479 -> LENGTH withdrawalQueue_1
TMP_4691(uint256[]) = LIBRARY_CALL, dest:DynamicArrayLib, function:DynamicArrayLib.slice(uint256[],uint256,uint256), arguments:['withdrawalQueue_1', 'num_1', 'REF_1479'] 
_withdrawalQueue_16(uint256[]) = ['TMP_4691(uint256[])']
```
#### GTL.addSubaccount(uint256) [EXTERNAL]
```slithir
_subaccounts_1(EnumerableSetLib.Uint256Set) := phi(['_subaccounts_4', '_subaccounts_2', '_subaccounts_0'])
 _subaccounts.add(subaccount)
TMP_4653(bool) = LIBRARY_CALL, dest:EnumerableSetLib, function:EnumerableSetLib.add(EnumerableSetLib.Uint256Set,uint256), arguments:['_subaccounts_2', 'subaccount_1'] 
 onlyPerpManager()
MODIFIER_CALL, GTL.onlyPerpManager()()
```
#### GTL.approveOperator(address) [EXTERNAL][OWNER]
```slithir
perpManager_4(address) := phi(['perpManager_10', 'perpManager_16', 'perpManager_14', 'perpManager_1', 'perpManager_7', 'perpManager_3', 'perpManager_12', 'perpManager_0'])
ADMIN_ROLE_7(uint256) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_11', 'ADMIN_ROLE_14', 'ADMIN_ROLE_3', 'ADMIN_ROLE_9', 'ADMIN_ROLE_6'])
 ! hasAllRoles(operator,ADMIN_ROLE)
TMP_4638(bool) = INTERNAL_CALL, OwnableRoles.hasAllRoles(address,uint256)(operator_1,ADMIN_ROLE_8)
TMP_4639 = UnaryType.BANG TMP_4638 
CONDITION TMP_4639
 revert InvalidOperator()()
TMP_4640(None) = SOLIDITY_CALL revert InvalidOperator()()
 IOperatorPanel(perpManager).approveOperator({account:address(this),operator:operator,roles:1 << uint256(PerpsOperatorRoles.ADMIN)})
TMP_4641 = CONVERT perpManager_6 to IOperatorPanel
TMP_4642 = CONVERT this to address
REF_1454(PerpsOperatorRoles) -> PerpsOperatorRoles.ADMIN
TMP_4643 = CONVERT REF_1454 to uint256
TMP_4644(uint256) = 1 << TMP_4643
HIGH_LEVEL_CALL, dest:TMP_4641(IOperatorPanel), function:approveOperator, arguments:['TMP_4642', 'operator_1', 'TMP_4644']  
perpManager_7(address) := phi(['perpManager_10', 'perpManager_16', 'perpManager_6', 'perpManager_14', 'perpManager_1', 'perpManager_7', 'perpManager_12', 'perpManager_3'])
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### GTL.asset() [PUBLIC]
```slithir
usdc_4(address) := phi(['usdc_15', 'usdc_9', 'usdc_1', 'usdc_3', 'usdc_17', 'usdc_0'])
 usdc
RETURN usdc_4
```
#### GTL.cancelWithdrawal(uint256) [EXTERNAL]
```slithir
_queuedWithdrawal_2(mapping(uint256 => GTL.Withdrawal)) := phi(['_queuedWithdrawal_0', '_queuedWithdrawal_8', '_queuedWithdrawal_1', '_queuedWithdrawal_12', '_queuedWithdrawal_3'])
_queuedShares_4(mapping(address => uint256)) := phi(['_queuedShares_5', '_queuedShares_14', '_queuedShares_0', '_queuedShares_10', '_queuedShares_3', '_queuedShares_16'])
 _queuedWithdrawal[id].account != msg.sender
REF_1433(GTL.Withdrawal) -> _queuedWithdrawal_2[id_1]
REF_1434(address) -> REF_1433.account
TMP_4614(bool) = REF_1434 != msg.sender
CONDITION TMP_4614
 revert NotPerpManager()()
TMP_4615(None) = SOLIDITY_CALL revert NotPerpManager()()
 _queuedShares[msg.sender] -= _queuedWithdrawal[id].shares
REF_1435(uint256) -> _queuedShares_4[msg.sender]
REF_1436(GTL.Withdrawal) -> _queuedWithdrawal_2[id_1]
REF_1437(uint256) -> REF_1436.shares
_queuedShares_5(mapping(address => uint256)) := phi(['_queuedShares_4'])
REF_1435(-> _queuedShares_5) = REF_1435 (c)- REF_1437
 delete _queuedWithdrawal[id]
REF_1438(GTL.Withdrawal) -> _queuedWithdrawal_2[id_1]
_queuedWithdrawal_3 = delete REF_1438 
 _dequeue(id)
INTERNAL_CALL, GTL._dequeue(uint256)(id_1)
 WithdrawalCanceled(id)
Emit WithdrawalCanceled(id_1)
```
#### GTL.constructor(address,address) [PUBLIC]
```slithir
 usdc = _usdc
usdc_1(address) := _usdc_1(address)
 perpManager = _perpManager
perpManager_1(address) := _perpManager_1(address)
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### GTL.disapproveOperator(address) [EXTERNAL][OWNER]
```slithir
perpManager_8(address) := phi(['perpManager_10', 'perpManager_16', 'perpManager_14', 'perpManager_1', 'perpManager_7', 'perpManager_3', 'perpManager_12', 'perpManager_0'])
 IOperatorPanel(perpManager).disapproveOperator({account:address(this),operator:operator,roles:1 << uint256(PerpsOperatorRoles.ADMIN)})
TMP_4647 = CONVERT perpManager_9 to IOperatorPanel
TMP_4648 = CONVERT this to address
REF_1456(PerpsOperatorRoles) -> PerpsOperatorRoles.ADMIN
TMP_4649 = CONVERT REF_1456 to uint256
TMP_4650(uint256) = 1 << TMP_4649
HIGH_LEVEL_CALL, dest:TMP_4647(IOperatorPanel), function:disapproveOperator, arguments:['TMP_4648', 'operator_1', 'TMP_4650']  
perpManager_10(address) := phi(['perpManager_10', 'perpManager_16', 'perpManager_14', 'perpManager_1', 'perpManager_7', 'perpManager_12', 'perpManager_3', 'perpManager_9'])
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### GTL.freeCollateralBalance() [PUBLIC]
```slithir
perpManager_15(address) := phi(['perpManager_10', 'perpManager_16', 'perpManager_14', 'perpManager_1', 'perpManager_7', 'perpManager_3', 'perpManager_12', 'perpManager_0'])
 IViewPort(perpManager).getFreeCollateralBalance(address(this))
TMP_4677 = CONVERT perpManager_15 to IViewPort
TMP_4678 = CONVERT this to address
TMP_4679(uint256) = HIGH_LEVEL_CALL, dest:TMP_4677(IViewPort), function:getFreeCollateralBalance, arguments:['TMP_4678']  
perpManager_16(address) := phi(['perpManager_15', 'perpManager_10', 'perpManager_16', 'perpManager_14', 'perpManager_1', 'perpManager_7', 'perpManager_12', 'perpManager_3'])
RETURN TMP_4679
```
#### GTL.getQueuedShares(address) [EXTERNAL]
```slithir
_queuedShares_14(mapping(address => uint256)) := phi(['_queuedShares_5', '_queuedShares_14', '_queuedShares_0', '_queuedShares_10', '_queuedShares_3', '_queuedShares_16'])
 _queuedShares[account]
REF_1472(uint256) -> _queuedShares_14[account_1]
RETURN REF_1472
```
#### GTL.getQueuedWithdrawal(uint256) [EXTERNAL]
```slithir
_queuedWithdrawal_12(mapping(uint256 => GTL.Withdrawal)) := phi(['_queuedWithdrawal_0', '_queuedWithdrawal_8', '_queuedWithdrawal_1', '_queuedWithdrawal_12', '_queuedWithdrawal_3'])
 _queuedWithdrawal[id]
REF_1471(GTL.Withdrawal) -> _queuedWithdrawal_12[id_1]
RETURN REF_1471
```
#### GTL.getSubaccounts() [EXTERNAL]
```slithir
_subaccounts_7(EnumerableSetLib.Uint256Set) := phi(['_subaccounts_4', '_subaccounts_2', '_subaccounts_0'])
 _subaccounts.values()
TMP_4680(uint256[]) = LIBRARY_CALL, dest:EnumerableSetLib, function:EnumerableSetLib.values(EnumerableSetLib.Uint256Set), arguments:['_subaccounts_7'] 
RETURN TMP_4680
```
#### GTL.getWithdrawalQueue() [EXTERNAL]
```slithir
_withdrawalQueue_12(uint256[]) := phi(['_withdrawalQueue_0', '_withdrawalQueue_9', '_withdrawalQueue_14', '_withdrawalQueue_4', '_withdrawalQueue_16'])
 _withdrawalQueue
RETURN _withdrawalQueue_12
```
#### GTL.grantAdminRole(address) [EXTERNAL][OWNER]
```slithir
ADMIN_ROLE_1(uint256) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_11', 'ADMIN_ROLE_14', 'ADMIN_ROLE_3', 'ADMIN_ROLE_9', 'ADMIN_ROLE_6'])
 _grantRoles(account,ADMIN_ROLE)
INTERNAL_CALL, OwnableRoles._grantRoles(address,uint256)(account_1,ADMIN_ROLE_2)
 AdminRoleGranted(account)
Emit AdminRoleGranted(account_1)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### GTL.hasAdminRole(address) [EXTERNAL]
```slithir
ADMIN_ROLE_10(uint256) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_11', 'ADMIN_ROLE_14', 'ADMIN_ROLE_3', 'ADMIN_ROLE_9', 'ADMIN_ROLE_6'])
 hasAllRoles(account,ADMIN_ROLE)
TMP_4681(bool) = INTERNAL_CALL, OwnableRoles.hasAllRoles(address,uint256)(account_1,ADMIN_ROLE_10)
RETURN TMP_4681
```
#### GTL.initialize(address) [EXTERNAL]
```slithir
usdc_2(address) := phi(['usdc_15', 'usdc_9', 'usdc_1', 'usdc_3', 'usdc_17', 'usdc_0'])
perpManager_2(address) := phi(['perpManager_10', 'perpManager_16', 'perpManager_14', 'perpManager_1', 'perpManager_7', 'perpManager_3', 'perpManager_12', 'perpManager_0'])
 usdc.safeApprove(perpManager,type()(uint256).max)
TMP_4599(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeApprove(address,address,uint256), arguments:['usdc_3', 'perpManager_3', 'TMP_4599'] 
 _initializeOwner(_owner)
INTERNAL_CALL, Ownable._initializeOwner(address)(_owner_1)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### GTL.maxRedeem(address) [PUBLIC]
```slithir
 0
RETURN 0
```
#### GTL.maxWithdraw(address) [PUBLIC]
```slithir
 0
RETURN 0
```
#### GTL.name() [PUBLIC]
```slithir
 GTE Liquidity Pool
RETURN GTE Liquidity Pool
```
#### GTL.orderbookCollateral() [PUBLIC]
```slithir
perpManager_13(address) := phi(['perpManager_10', 'perpManager_16', 'perpManager_14', 'perpManager_1', 'perpManager_7', 'perpManager_3', 'perpManager_12', 'perpManager_0'])
_subaccounts_6(EnumerableSetLib.Uint256Set) := phi(['_subaccounts_4', '_subaccounts_2', '_subaccounts_0'])
 subaccounts = _subaccounts.values()
TMP_4672(uint256[]) = LIBRARY_CALL, dest:EnumerableSetLib, function:EnumerableSetLib.values(EnumerableSetLib.Uint256Set), arguments:['_subaccounts_6'] 
subaccounts_1(uint256[]) = ['TMP_4672(uint256[])']
 i < subaccounts.length
collateral_1(uint256) := phi(['collateral_0', 'collateral_2'])
i_1(uint256) := phi(['i_0', 'i_2'])
REF_1466 -> LENGTH subaccounts_1
TMP_4673(bool) = i_1 < REF_1466
CONDITION TMP_4673
 collateral += IViewPort(perpManager).getOrderbookCollateral(address(this),subaccounts[i])
TMP_4674 = CONVERT perpManager_13 to IViewPort
TMP_4675 = CONVERT this to address
REF_1468(uint256) -> subaccounts_1[i_1]
TMP_4676(uint256) = HIGH_LEVEL_CALL, dest:TMP_4674(IViewPort), function:getOrderbookCollateral, arguments:['TMP_4675', 'REF_1468']  
perpManager_14(address) := phi(['perpManager_10', 'perpManager_16', 'perpManager_14', 'perpManager_13', 'perpManager_1', 'perpManager_7', 'perpManager_12', 'perpManager_3'])
collateral_2(uint256) = collateral_1 (c)+ TMP_4676
 ++ i
i_2(uint256) = i_1 (c)+ 1
 collateral
RETURN collateral_1
```
#### GTL.previewWithdraw(uint256) [PUBLIC]
```slithir
 0
RETURN 0
 assets
```
#### GTL.processWithdrawals(uint256) [EXTERNAL]
```slithir
usdc_5(address) := phi(['usdc_15', 'usdc_9', 'usdc_1', 'usdc_3', 'usdc_17', 'usdc_0'])
_withdrawalQueue_5(uint256[]) := phi(['_withdrawalQueue_0', '_withdrawalQueue_9', '_withdrawalQueue_14', '_withdrawalQueue_4', '_withdrawalQueue_16'])
_queuedWithdrawal_4(mapping(uint256 => GTL.Withdrawal)) := phi(['_queuedWithdrawal_0', '_queuedWithdrawal_8', '_queuedWithdrawal_1', '_queuedWithdrawal_12', '_queuedWithdrawal_3'])
_queuedShares_6(mapping(address => uint256)) := phi(['_queuedShares_5', '_queuedShares_14', '_queuedShares_0', '_queuedShares_10', '_queuedShares_3', '_queuedShares_16'])
 num > _withdrawalQueue.length
REF_1439 -> LENGTH _withdrawalQueue_6
TMP_4618(bool) = num_1 > REF_1439
CONDITION TMP_4618
 revert InsufficientWithdrawalsQueued()()
TMP_4619(None) = SOLIDITY_CALL revert InsufficientWithdrawalsQueued()()
 allocatedAssets = orderbookCollateral() + freeCollateralBalance() + totalAccountValue()
TMP_4620(uint256) = INTERNAL_CALL, GTL.orderbookCollateral()()
TMP_4621(uint256) = INTERNAL_CALL, GTL.freeCollateralBalance()()
TMP_4622(uint256) = TMP_4620 (c)+ TMP_4621
TMP_4623(uint256) = INTERNAL_CALL, GTL.totalAccountValue()()
TMP_4624(uint256) = TMP_4622 (c)+ TMP_4623
allocatedAssets_1(uint256) := TMP_4624(uint256)
 i < num
i_1(uint256) := phi(['i_0', 'i_2'])
TMP_4625(bool) = i_1 < num_1
CONDITION TMP_4625
 id = _withdrawalQueue[i]
REF_1440(uint256) -> _withdrawalQueue_9[i_1]
id_1(uint256) := REF_1440(uint256)
 withdrawal = _queuedWithdrawal[id]
REF_1441(GTL.Withdrawal) -> _queuedWithdrawal_8[id_1]
withdrawal_1(GTL.Withdrawal) := REF_1441(GTL.Withdrawal)
 assets = _convertToAssets({shares:withdrawal.shares,allocatedAssets:allocatedAssets})
REF_1442(uint256) -> withdrawal_1.shares
TMP_4626(uint256) = INTERNAL_CALL, GTL._convertToAssets(uint256,uint256)(REF_1442,allocatedAssets_1)
usdc_10(address) := phi(['usdc_17'])
assets_1(uint256) := TMP_4626(uint256)
 delete _queuedWithdrawal[id]
REF_1443(GTL.Withdrawal) -> _queuedWithdrawal_9[id_1]
_queuedWithdrawal_10 = delete REF_1443 
 _queuedShares[withdrawal.account] -= withdrawal.shares
REF_1444(address) -> withdrawal_1.account
REF_1445(uint256) -> _queuedShares_11[REF_1444]
REF_1446(uint256) -> withdrawal_1.shares
_queuedShares_12(mapping(address => uint256)) := phi(['_queuedShares_11'])
REF_1445(-> _queuedShares_12) = REF_1445 (c)- REF_1446
 _burn(withdrawal.account,withdrawal.shares)
REF_1447(address) -> withdrawal_1.account
REF_1448(uint256) -> withdrawal_1.shares
INTERNAL_CALL, ERC20._burn(address,uint256)(REF_1447,REF_1448)
 usdc.safeTransfer(withdrawal.account,assets)
REF_1450(address) -> withdrawal_1.account
LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.safeTransfer(address,address,uint256), arguments:['usdc_11', 'REF_1450', 'assets_1'] 
 WithdrawalProcessed(id,withdrawal.account,withdrawal.shares,assets)
REF_1451(address) -> withdrawal_1.account
REF_1452(uint256) -> withdrawal_1.shares
Emit WithdrawalProcessed(id_1,REF_1451,REF_1452,assets_1)
 ++ i
i_2(uint256) = i_1 (c)+ 1
 _dequeueBatch(num)
INTERNAL_CALL, GTL._dequeueBatch(uint256)(num_1)
 onlyAdmin()
MODIFIER_CALL, GTL.onlyAdmin()()
```
#### GTL.queueWithdrawal(uint256) [EXTERNAL]
```slithir
_withdrawalQueue_1(uint256[]) := phi(['_withdrawalQueue_0', '_withdrawalQueue_9', '_withdrawalQueue_14', '_withdrawalQueue_4', '_withdrawalQueue_16'])
_withdrawalCounter_1(uint256) := phi(['_withdrawalCounter_0', '_withdrawalCounter_3'])
_queuedShares_1(mapping(address => uint256)) := phi(['_queuedShares_5', '_queuedShares_14', '_queuedShares_0', '_queuedShares_10', '_queuedShares_3', '_queuedShares_16'])
 shares == 0
TMP_4603(bool) = shares_1 == 0
CONDITION TMP_4603
 revert InsufficientWithdrawal()()
TMP_4604(None) = SOLIDITY_CALL revert InsufficientWithdrawal()()
 _queuedShares[msg.sender] + shares > balanceOf(msg.sender)
REF_1427(uint256) -> _queuedShares_1[msg.sender]
TMP_4605(uint256) = REF_1427 (c)+ shares_1
TMP_4606(uint256) = INTERNAL_CALL, ERC20.balanceOf(address)(msg.sender)
TMP_4607(bool) = TMP_4605 > TMP_4606
CONDITION TMP_4607
 revert InsufficientBalance()()
TMP_4608(None) = SOLIDITY_CALL revert InsufficientBalance()()
 id = ++ _withdrawalCounter
_withdrawalCounter_3(uint256) = _withdrawalCounter_2 (c)+ 1
id_1(uint256) := _withdrawalCounter_3(uint256)
 _queuedShares[msg.sender] += shares
REF_1428(uint256) -> _queuedShares_2[msg.sender]
_queuedShares_3(mapping(address => uint256)) := phi(['_queuedShares_2'])
REF_1428(-> _queuedShares_3) = REF_1428 (c)+ shares_1
 _queuedWithdrawal[id] = Withdrawal(msg.sender,shares)
REF_1429(GTL.Withdrawal) -> _queuedWithdrawal_0[id_1]
TMP_4609(GTL.Withdrawal) = new Withdrawal(msg.sender,shares_1)
_queuedWithdrawal_1(mapping(uint256 => GTL.Withdrawal)) := phi(['_queuedWithdrawal_0'])
REF_1429(GTL.Withdrawal) (->_queuedWithdrawal_1) := TMP_4609(GTL.Withdrawal)
 _withdrawalQueue.push(id)
REF_1431 -> LENGTH _withdrawalQueue_2
TMP_4611(uint256) := REF_1431(uint256)
TMP_4612(uint256) = TMP_4611 (c)+ 1
_withdrawalQueue_3(uint256[]) := phi(['_withdrawalQueue_2'])
REF_1431(uint256) (->_withdrawalQueue_3) := TMP_4612(uint256)
REF_1432(uint256) -> _withdrawalQueue_3[TMP_4611]
_withdrawalQueue_4(uint256[]) := phi(['_withdrawalQueue_3'])
REF_1432(uint256) (->_withdrawalQueue_4) := id_1(uint256)
 WithdrawalQueued(id,msg.sender,shares)
Emit WithdrawalQueued(id_1,msg.sender,shares_1)
 id
RETURN id_1
```
#### GTL.removeSubaccount(uint256) [EXTERNAL]
```slithir
_subaccounts_3(EnumerableSetLib.Uint256Set) := phi(['_subaccounts_4', '_subaccounts_2', '_subaccounts_0'])
 _subaccounts.remove(subaccount)
TMP_4655(bool) = LIBRARY_CALL, dest:EnumerableSetLib, function:EnumerableSetLib.remove(EnumerableSetLib.Uint256Set,uint256), arguments:['_subaccounts_4', 'subaccount_1'] 
 onlyPerpManager()
MODIFIER_CALL, GTL.onlyPerpManager()()
```
#### GTL.revokeAdminRole(address) [EXTERNAL][OWNER]
```slithir
ADMIN_ROLE_4(uint256) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_11', 'ADMIN_ROLE_14', 'ADMIN_ROLE_3', 'ADMIN_ROLE_9', 'ADMIN_ROLE_6'])
 _removeRoles(account,ADMIN_ROLE)
INTERNAL_CALL, OwnableRoles._removeRoles(address,uint256)(account_1,ADMIN_ROLE_5)
 AdminRoleRevoked(account)
Emit AdminRoleRevoked(account_1)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```

#### GTL.symbol() [PUBLIC]
```slithir
 GTL
RETURN GTL
```
#### GTL.totalAccountValue() [PUBLIC]
```slithir
perpManager_11(address) := phi(['perpManager_10', 'perpManager_16', 'perpManager_14', 'perpManager_1', 'perpManager_7', 'perpManager_3', 'perpManager_12', 'perpManager_0'])
_subaccounts_5(EnumerableSetLib.Uint256Set) := phi(['_subaccounts_4', '_subaccounts_2', '_subaccounts_0'])
 subaccounts = _subaccounts.values()
TMP_4665(uint256[]) = LIBRARY_CALL, dest:EnumerableSetLib, function:EnumerableSetLib.values(EnumerableSetLib.Uint256Set), arguments:['_subaccounts_5'] 
subaccounts_1(uint256[]) = ['TMP_4665(uint256[])']
 i < subaccounts.length
i_1(uint256) := phi(['i_2', 'i_0'])
REF_1461 -> LENGTH subaccounts_1
TMP_4666(bool) = i_1 < REF_1461
CONDITION TMP_4666
 subaccountValue = IViewPort(perpManager).getAccountValue(address(this),subaccounts[i])
TMP_4667 = CONVERT perpManager_11 to IViewPort
TMP_4668 = CONVERT this to address
REF_1463(uint256) -> subaccounts_1[i_1]
TMP_4669(int256) = HIGH_LEVEL_CALL, dest:TMP_4667(IViewPort), function:getAccountValue, arguments:['TMP_4668', 'REF_1463']  
perpManager_12(address) := phi(['perpManager_10', 'perpManager_16', 'perpManager_14', 'perpManager_1', 'perpManager_7', 'perpManager_12', 'perpManager_3', 'perpManager_11'])
subaccountValue_1(int256) := TMP_4669(int256)
 subaccountValue > 0
TMP_4670(bool) = subaccountValue_1 > 0
CONDITION TMP_4670
 accountValue += subaccountValue.abs()
TMP_4671(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.abs(int256), arguments:['subaccountValue_1'] 
accountValue_1(uint256) = accountValue_0 (c)+ TMP_4671
accountValue_2(uint256) := phi(['accountValue_1', 'accountValue_0'])
 ++ i
i_2(uint256) = i_1 (c)+ 1
 accountValue
RETURN accountValue_0
```
#### GTL.totalAssets() [PUBLIC]
```slithir
usdc_12(address) := phi(['usdc_15', 'usdc_9', 'usdc_1', 'usdc_3', 'usdc_17', 'usdc_0'])
 usdc.balanceOf(address(this)) + orderbookCollateral() + freeCollateralBalance() + totalAccountValue()
TMP_4657 = CONVERT this to address
TMP_4658(uint256) = LIBRARY_CALL, dest:SafeTransferLib, function:SafeTransferLib.balanceOf(address,address), arguments:['usdc_12', 'TMP_4657'] 
TMP_4659(uint256) = INTERNAL_CALL, GTL.orderbookCollateral()()
TMP_4660(uint256) = TMP_4658 (c)+ TMP_4659
TMP_4661(uint256) = INTERNAL_CALL, GTL.freeCollateralBalance()()
TMP_4662(uint256) = TMP_4660 (c)+ TMP_4661
TMP_4663(uint256) = INTERNAL_CALL, GTL.totalAccountValue()()
TMP_4664(uint256) = TMP_4662 (c)+ TMP_4663
RETURN TMP_4664
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
#### DynamicArrayLib.slice(DynamicArrayLib.DynamicArray,uint256) [INTERNAL]
```slithir
 result.data = slice(a.data,start,type()(uint256).max)
REF_5757(uint256[]) -> result_0.data
REF_5758(uint256[]) -> a_1.data
TMP_12405(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_12406(uint256[]) = INTERNAL_CALL, DynamicArrayLib.slice(uint256[],uint256,uint256)(REF_5758,start_1,TMP_12405)
result_1(DynamicArrayLib.DynamicArray) := phi(['result_0'])
REF_5757(uint256[]) (->result_1) := TMP_12406(uint256[])
 result
RETURN result_1
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
#### IOperatorPanel.approveOperator(address,address,uint256) [EXTERNAL]
```slithir

```
#### IOperatorPanel.disapproveOperator(address,address,uint256) [EXTERNAL]
```slithir

```
#### IViewPort.getFreeCollateralBalance(address) [EXTERNAL]
```slithir

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
#### IViewPort.getOrderbookCollateral(address,uint256) [EXTERNAL]
```slithir

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
#### IViewPort.getAccountValue(address,uint256) [EXTERNAL]
```slithir

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
