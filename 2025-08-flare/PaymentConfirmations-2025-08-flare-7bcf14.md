

#### PaymentConfirmations.confirmIncomingPayment(PaymentConfirmations.State,IPayment.Proof) [INTERNAL]
```slithir
 _recordPaymentVerification(_state,_payment.data.requestBody.transactionId)
REF_3753(IPayment.Response) -> _payment_1.data
REF_3754(IPayment.RequestBody) -> REF_3753.requestBody
REF_3755(bytes32) -> REF_3754.transactionId
INTERNAL_CALL, PaymentConfirmations._recordPaymentVerification(PaymentConfirmations.State,bytes32)(_state_1 (-> []),REF_3755)
```
#### PaymentConfirmations.confirmSourceDecreasingTransaction(PaymentConfirmations.State,IPayment.Proof) [INTERNAL]
```slithir
 txKey = transactionKey(_payment.data.responseBody.sourceAddressHash,_payment.data.requestBody.transactionId)
REF_3756(IPayment.Response) -> _payment_1.data
REF_3757(IPayment.ResponseBody) -> REF_3756.responseBody
REF_3758(bytes32) -> REF_3757.sourceAddressHash
REF_3759(IPayment.Response) -> _payment_1.data
REF_3760(IPayment.RequestBody) -> REF_3759.requestBody
REF_3761(bytes32) -> REF_3760.transactionId
TMP_5336(bytes32) = INTERNAL_CALL, PaymentConfirmations.transactionKey(bytes32,bytes32)(REF_3758,REF_3761)
txKey_1(bytes32) := TMP_5336(bytes32)
 _recordPaymentVerification(_state,txKey)
INTERNAL_CALL, PaymentConfirmations._recordPaymentVerification(PaymentConfirmations.State,bytes32)(_state_1 (-> []),txKey_1)
```
#### PaymentConfirmations.transactionConfirmed(PaymentConfirmations.State,IBalanceDecreasingTransaction.Proof) [INTERNAL]
```slithir
 txKey = transactionKey(_transaction.data.responseBody.sourceAddressHash,_transaction.data.requestBody.transactionId)
REF_3762(IBalanceDecreasingTransaction.Response) -> _transaction_1.data
REF_3763(IBalanceDecreasingTransaction.ResponseBody) -> REF_3762.responseBody
REF_3764(bytes32) -> REF_3763.sourceAddressHash
REF_3765(IBalanceDecreasingTransaction.Response) -> _transaction_1.data
REF_3766(IBalanceDecreasingTransaction.RequestBody) -> REF_3765.requestBody
REF_3767(bytes32) -> REF_3766.transactionId
TMP_5338(bytes32) = INTERNAL_CALL, PaymentConfirmations.transactionKey(bytes32,bytes32)(REF_3764,REF_3767)
txKey_1(bytes32) := TMP_5338(bytes32)
 _state.verifiedPayments[txKey] != 0
REF_3768(mapping(bytes32 => bytes32)) -> _state_1 (-> []).verifiedPayments
REF_3769(bytes32) -> REF_3768[txKey_1]
TMP_5339(bool) = REF_3769 != 0
RETURN TMP_5339
```
#### PaymentConfirmations.transactionKey(bytes32,bytes32) [INTERNAL]
```slithir
_underlyingSourceAddressHash_1(bytes32) := phi(['REF_3758', 'REF_3764'])
_transactionHash_1(bytes32) := phi(['REF_3761', 'REF_3767'])
 keccak256(bytes)(abi.encode(_underlyingSourceAddressHash,_transactionHash))
TMP_5340(bytes) = SOLIDITY_CALL abi.encode()(_underlyingSourceAddressHash_1,_transactionHash_1)
TMP_5341(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_5340)
RETURN TMP_5341
```
