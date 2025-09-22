

#### Transfers.depositWNat(IWNat,address,uint256) [INTERNAL]
```slithir
 _amount > 0
TMP_10540(bool) = _amount_1 > 0
CONDITION TMP_10540
 _wNat.depositTo{value: _amount}(_recipient)
HIGH_LEVEL_CALL, dest:_wNat_1(IWNat), function:depositTo, arguments:['_recipient_1'] value:_amount_1
```
#### SafePct.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 MAX_BIPS = 10_000
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
#### IWNat.depositTo(address) [EXTERNAL]
```slithir

```
