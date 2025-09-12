
#### MathUtils.mixedLTE(uint256,int256) [INTERNAL]
```slithir
 _b >= 0 && _a <= uint256(_b)
TMP_10429(bool) = _b_1 >= 0
TMP_10430 = CONVERT _b_1 to uint256
TMP_10431(bool) = _a_1 <= TMP_10430
TMP_10432(bool) = TMP_10429 && TMP_10431
RETURN TMP_10432
```
#### MathUtils.positivePart(int256) [INTERNAL]
```slithir
 _x >= 0
TMP_10427(bool) = _x_1 >= 0
CONDITION TMP_10427
 uint256(_x)
TMP_10428 = CONVERT _x_1 to uint256
RETURN TMP_10428
 0
RETURN 0
```
#### MathUtils.roundUp(uint256,uint256) [INTERNAL]
```slithir
 remainder = x % rounding
TMP_10421(uint256) = x_1 % rounding_1
remainder_1(uint256) := TMP_10421(uint256)
 remainder == 0
TMP_10422(bool) = remainder_1 == 0
CONDITION TMP_10422
 x
RETURN x_1
 x - remainder + rounding
TMP_10423(uint256) = x_1 (c)- remainder_1
TMP_10424(uint256) = TMP_10423 (c)+ rounding_1
RETURN TMP_10424
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
