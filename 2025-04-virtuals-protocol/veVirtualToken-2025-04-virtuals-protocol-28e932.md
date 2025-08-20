
#### veVirtualToken._update(address,address,uint256) [INTERNAL]
```slithir
from_1(address) := phi(['REF_4261', 'account_1', 'TMP_10423', 'from_1'])
to_1(address) := phi(['to_1', 'REF_4262', 'TMP_10429', 'account_1'])
value_1(uint256) := phi(['value_1', 'value_1', 'REF_4263', 'value_1'])
 super._update(from,to,value)
INTERNAL_CALL, ERC20Votes._update(address,address,uint256)(from_1,to_1,value_1)
```
#### veVirtualToken.approve(address,uint256) [PUBLIC]
```slithir
 revert(string)(Approve not supported)
TMP_10473(None) = SOLIDITY_CALL revert(string)(Approve not supported)
```
#### veVirtualToken.constructor(address) [PUBLIC]
```slithir
 ERC20(Virtual Protocol Voting,veVIRTUAL)
INTERNAL_CALL, ERC20.constructor(string,string)(Virtual Protocol Voting,veVIRTUAL)
 ERC20Permit(Virtual Protocol Voting)
INTERNAL_CALL, ERC20Permit.constructor(string)(Virtual Protocol Voting)
 Ownable(initialOwner)
INTERNAL_CALL, Ownable.constructor(address)(initialOwner_1)
```
#### IERC20Permit.nonces(address) [PUBLIC]
```slithir

```
#### veVirtualToken.oracleTransfer(address[],address[],uint256[]) [EXTERNAL][OWNER]
```slithir
 require(bool,string)(froms.length == tos.length && tos.length == values.length,Invalid input)
REF_4256 -> LENGTH froms_1
REF_4257 -> LENGTH tos_1
TMP_10465(bool) = REF_4256 == REF_4257
REF_4258 -> LENGTH tos_1
REF_4259 -> LENGTH values_1
TMP_10466(bool) = REF_4258 == REF_4259
TMP_10467(bool) = TMP_10465 && TMP_10466
TMP_10468(None) = SOLIDITY_CALL require(bool,string)(TMP_10467,Invalid input)
 i = 0
i_1(uint256) := 0(uint256)
 i < froms.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_4260 -> LENGTH froms_1
TMP_10469(bool) = i_2 < REF_4260
CONDITION TMP_10469
 _update(froms[i],tos[i],values[i])
REF_4261(address) -> froms_1[i_2]
REF_4262(address) -> tos_1[i_2]
REF_4263(uint256) -> values_1[i_2]
INTERNAL_CALL, veVirtualToken._update(address,address,uint256)(REF_4261,REF_4262,REF_4263)
 i ++
TMP_10471(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 true
RETURN True
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### veVirtualToken.transfer(address,uint256) [PUBLIC]
```slithir
 revert(string)(Transfer not supported)
TMP_10474(None) = SOLIDITY_CALL revert(string)(Transfer not supported)
```
#### veVirtualToken.transferFrom(address,address,uint256) [PUBLIC]
```slithir
 revert(string)(Transfer not supported)
TMP_10475(None) = SOLIDITY_CALL revert(string)(Transfer not supported)
```
