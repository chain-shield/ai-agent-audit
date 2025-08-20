### Storage layout (LPRefund) 

```text
taxToken address
refunds mapping(bytes32 => uint256)

```



#### LPRefund.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### LPRefund.initialize(address,address) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_4', 'DEFAULT_ADMIN_ROLE_0'])
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_9', 'ADMIN_ROLE_5', 'ADMIN_ROLE_7'])
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 _grantRole(DEFAULT_ADMIN_ROLE,defaultAdmin_)
TMP_11148(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_3,defaultAdmin__1)
 _grantRole(ADMIN_ROLE,defaultAdmin_)
TMP_11149(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(ADMIN_ROLE_4,defaultAdmin__1)
 taxToken = taxToken_
taxToken_1(address) := taxToken__1(address)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### LPRefund.manualRefund(bytes32,address,uint256) [PUBLIC]
```slithir
ADMIN_ROLE_8(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_9', 'ADMIN_ROLE_5', 'ADMIN_ROLE_7'])
taxToken_4(address) := phi(['taxToken_0', 'taxToken_5', 'taxToken_3', 'taxToken_1'])
refunds_4(mapping(bytes32 => uint256)) := phi(['refunds_6', 'refunds_2', 'refunds_0'])
 refunds[txhash] += amount
REF_4564(uint256) -> refunds_5[txhash_1]
refunds_6(mapping(bytes32 => uint256)) := phi(['refunds_5'])
REF_4564(-> refunds_6) = REF_4564 (c)+ amount_1
 IERC20(taxToken).safeTransfer(recipient,amount)
TMP_11168 = CONVERT taxToken_5 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_11168', 'recipient_1', 'amount_1'] 
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_8)
```
#### LPRefund.refund(address,bytes32[],uint256[]) [PUBLIC]
```slithir
EXECUTOR_ROLE_1(bytes32) := phi(['EXECUTOR_ROLE_0', 'EXECUTOR_ROLE_2'])
taxToken_2(address) := phi(['taxToken_0', 'taxToken_5', 'taxToken_3', 'taxToken_1'])
refunds_1(mapping(bytes32 => uint256)) := phi(['refunds_6', 'refunds_2', 'refunds_0'])
 require(bool,string)(txhashes.length == amounts.length,Unmatched inputs)
REF_4556 -> LENGTH txhashes_1
REF_4557 -> LENGTH amounts_1
TMP_11158(bool) = REF_4556 == REF_4557
TMP_11159(None) = SOLIDITY_CALL require(bool,string)(TMP_11158,Unmatched inputs)
 total = 0
total_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < txhashes.length
total_2(uint256) := phi(['total_1', 'total_3'])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4558 -> LENGTH txhashes_1
TMP_11160(bool) = i_2 < REF_4558
CONDITION TMP_11160
 txhash = txhashes[i]
REF_4559(bytes32) -> txhashes_1[i_2]
txhash_1(bytes32) := REF_4559(bytes32)
 amount = amounts[i]
REF_4560(uint256) -> amounts_1[i_2]
amount_1(uint256) := REF_4560(uint256)
 refunds[txhash] > 0
REF_4561(uint256) -> refunds_2[txhash_1]
TMP_11161(bool) = REF_4561 > 0
CONDITION TMP_11161
 revert TxHashExists(bytes32)(txhash)
TMP_11162(None) = SOLIDITY_CALL revert TxHashExists(bytes32)(txhash_1)
 refunds[txhash] = amount
REF_4562(uint256) -> refunds_2[txhash_1]
refunds_3(mapping(bytes32 => uint256)) := phi(['refunds_2'])
REF_4562(uint256) (->refunds_3) := amount_1(uint256)
 total += amount
total_3(uint256) = total_2 (c)+ amount_1
 TaxRefunded(txhash,recipient,amount)
Emit TaxRefunded(txhash_1,recipient_1,amount_1)
 i ++
TMP_11164(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 IERC20(taxToken).safeTransfer(recipient,total)
TMP_11165 = CONVERT taxToken_3 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_11165', 'recipient_1', 'total_2'] 
 onlyRole(EXECUTOR_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(EXECUTOR_ROLE_1)
```

#### LPRefund.withdraw(address) [EXTERNAL]
```slithir
ADMIN_ROLE_6(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_9', 'ADMIN_ROLE_5', 'ADMIN_ROLE_7'])
 IERC20(token).safeTransfer(_msgSender(),IERC20(token).balanceOf(address(this)))
TMP_11151 = CONVERT token_1 to IERC20
TMP_11152(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
TMP_11153 = CONVERT token_1 to IERC20
TMP_11154 = CONVERT this to address
TMP_11155(uint256) = HIGH_LEVEL_CALL, dest:TMP_11153(IERC20), function:balanceOf, arguments:['TMP_11154']  
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_11151', 'TMP_11152', 'TMP_11155'] 
 onlyRole(ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(ADMIN_ROLE_6)
```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_2044(transfer) -> token_1.transfer
TMP_5413(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2044,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000550>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001210>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5413)
```
#### IERC20.balanceOf(address) [EXTERNAL]
```slithir

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
