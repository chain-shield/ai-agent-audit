
#### SafeMath64.max64(uint64,uint64) [INTERNAL]
```slithir
 a >= b
TMP_10504(bool) = a_1 >= b_1
CONDITION TMP_10504
 a
RETURN a_1
 b
RETURN b_1
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

#### SafeMath64.toInt64(uint256) [INTERNAL]
```slithir
MAX_INT64_1(int256) := phi(['MAX_INT64_0'])
 require(bool,error)(a <= uint256(MAX_INT64),revert ConversionOverflow()())
TMP_10498 = CONVERT MAX_INT64_1 to uint256
TMP_10499(bool) = a_1 <= TMP_10498
TMP_10500(None) = SOLIDITY_CALL revert ConversionOverflow()()
TMP_10501(None) = SOLIDITY_CALL require(bool,error)(TMP_10499,TMP_10500)
 int64(int256(a))
TMP_10502 = CONVERT a_1 to int256
TMP_10503 = CONVERT TMP_10502 to int64
RETURN TMP_10503
```
#### SafeMath64.toUint64(int256) [INTERNAL]
```slithir
MAX_UINT64_1(uint256) := phi(['MAX_UINT64_0'])
 require(bool,error)(a >= 0,revert NegativeValue()())
TMP_10489(bool) = a_1 >= 0
TMP_10490(None) = SOLIDITY_CALL revert NegativeValue()()
TMP_10491(None) = SOLIDITY_CALL require(bool,error)(TMP_10489,TMP_10490)
 require(bool,error)(a <= int256(MAX_UINT64),revert ConversionOverflow()())
TMP_10492 = CONVERT MAX_UINT64_1 to int256
TMP_10493(bool) = a_1 <= TMP_10492
TMP_10494(None) = SOLIDITY_CALL revert ConversionOverflow()()
TMP_10495(None) = SOLIDITY_CALL require(bool,error)(TMP_10493,TMP_10494)
 uint64(uint256(a))
TMP_10496 = CONVERT a_1 to uint256
TMP_10497 = CONVERT TMP_10496 to uint64
RETURN TMP_10497
```
