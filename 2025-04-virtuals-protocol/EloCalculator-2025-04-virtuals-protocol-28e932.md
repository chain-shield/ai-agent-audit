
### Storage layout (EloCalculator) 

```text
k uint256

```

#### EloCalculator._roundUp(uint256,uint256) [INTERNAL]
```slithir
numerator_1(uint256) := phi(['change_1'])
 (numerator + denominator - 1) / denominator
TMP_14306(uint256) = numerator_1 (c)+ denominator_1
TMP_14307(uint256) = TMP_14306 (c)- 1
TMP_14308(uint256) = TMP_14307 (c)/ denominator_1
RETURN TMP_14308
```
#### EloCalculator.battleElo(uint256,uint8[]) [PUBLIC]
```slithir
k_2(uint256) := phi(['k_0', 'k_5', 'k_4', 'k_1'])
 eloA = 1000
eloA_1(uint256) := 1000(uint256)
 eloB = 1000
eloB_1(uint256) := 1000(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < battles.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_5818 -> LENGTH battles_1
TMP_14309(bool) = i_2 < REF_5818
CONDITION TMP_14309
 result = mapBattleResultToGameResult(battles[i])
REF_5819(uint8) -> battles_1[i_2]
TMP_14310(uint256) = INTERNAL_CALL, EloCalculator.mapBattleResultToGameResult(uint8)(REF_5819)
result_1(uint256) := TMP_14310(uint256)
 (change,negative) = Elo.ratingChange(eloB,eloA,result,k)
TUPLE_141(uint256,bool) = LIBRARY_CALL, dest:Elo, function:Elo.ratingChange(uint256,uint256,uint256,uint256), arguments:['eloB_1', 'eloA_1', 'result_1', 'k_3'] 
change_1(uint256)= UNPACK TUPLE_141 index: 0 
negative_1(bool)= UNPACK TUPLE_141 index: 1 
 change = _roundUp(change,100)
TMP_14311(uint256) = INTERNAL_CALL, EloCalculator._roundUp(uint256,uint256)(change_1,100)
change_2(uint256) := TMP_14311(uint256)
 negative
CONDITION negative_1
 eloA -= change
eloA_3(uint256) = eloA_1 (c)- change_2
 eloB += change
eloB_3(uint256) = eloB_1 (c)+ change_2
 eloA += change
eloA_2(uint256) = eloA_1 (c)+ change_2
 eloB -= change
eloB_2(uint256) = eloB_1 (c)- change_2
eloA_4(uint256) := phi(['eloA_2', 'eloA_3'])
eloB_4(uint256) := phi(['eloB_3', 'eloB_2'])
 i ++
TMP_14312(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 currentRating + eloA - 1000
TMP_14313(uint256) = currentRating_1 (c)+ eloA_1
TMP_14314(uint256) = TMP_14313 (c)- 1000
RETURN TMP_14314
```
#### EloCalculator.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### EloCalculator.initialize(address) [PUBLIC]
```slithir
 __Ownable_init(initialOwner)
INTERNAL_CALL, OwnableUpgradeable.__Ownable_init(address)(initialOwner_1)
 k = 30
k_1(uint256) := 30(uint256)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### EloCalculator.mapBattleResultToGameResult(uint8) [INTERNAL]
```slithir
result_1(uint8) := phi(['REF_5819'])
 result == 1
TMP_14303(bool) = result_1 == 1
CONDITION TMP_14303
 100
RETURN 100
 result == 2
TMP_14304(bool) = result_1 == 2
CONDITION TMP_14304
 50
RETURN 50
 result == 3
TMP_14305(bool) = result_1 == 3
CONDITION TMP_14305
 50
RETURN 50
 0
RETURN 0
```


#### Elo.sixteenthRoot(uint256) [INTERNAL]
```slithir
x_1(uint256) := phi(['_powered_1'])
 FixedPointMathLib.sqrt(FixedPointMathLib.sqrt(FixedPointMathLib.sqrt(FixedPointMathLib.sqrt(x))))
TMP_10510(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.sqrt(uint256), arguments:['x_1'] 
TMP_10511(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.sqrt(uint256), arguments:['TMP_10510'] 
TMP_10512(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.sqrt(uint256), arguments:['TMP_10511'] 
TMP_10513(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.sqrt(uint256), arguments:['TMP_10512'] 
RETURN TMP_10513
```
#### FixedPointMathLib.rpow(uint256,uint256,uint256) [INTERNAL]
```slithir
 switch_expr_2878_2570_119_rpow_asm_0 = x
switch_expr_2878_2570_119_rpow_asm_0_1(uint256) := x_1(uint256)
 switch_expr_2878_2570_119_rpow_asm_0 == 0
TMP_10559(bool) = switch_expr_2878_2570_119_rpow_asm_0_1 == 0
CONDITION TMP_10559
 switch_expr_2924_222_119_rpow_asm_0 = n
switch_expr_2924_222_119_rpow_asm_0_1(uint256) := n_1(uint256)
 switch_expr_2924_222_119_rpow_asm_0 == 0
TMP_10560(bool) = switch_expr_2924_222_119_rpow_asm_0_1 == 0
CONDITION TMP_10560
z_8(uint256) := phi(['z_6', 'z_7'])
 z = scalar
z_6(uint256) := scalar_1(uint256)
 z = 0
z_7(uint256) := 0(uint256)
 switch_expr_3199_284_119_rpow_asm_0 = n % 2
TMP_10561(uint256) = n_1 % 2
switch_expr_3199_284_119_rpow_asm_0_1(uint256) := TMP_10561(uint256)
 switch_expr_3199_284_119_rpow_asm_0 == 0
TMP_10562(bool) = switch_expr_3199_284_119_rpow_asm_0_1 == 0
CONDITION TMP_10562
z_3(uint256) := phi(['z_1', 'z_2'])
 z = scalar
z_1(uint256) := scalar_1(uint256)
 z = x
z_2(uint256) := x_1(uint256)
 half_rpow_asm_0 = scalar >> 1
TMP_10563(uint256) = scalar_1 >> 1
half_rpow_asm_0_1(uint256) := TMP_10563(uint256)
 n = n >> 1
TMP_10564(uint256) = n_1 >> 1
n_2(uint256) := TMP_10564(uint256)
 n
x_2(uint256) := phi(['x_1', 'x_3'])
n_3(uint256) := phi(['n_4', 'n_2'])
CONDITION n_3
 x >> 128
TMP_10565(uint256) = x_2 >> 128
CONDITION TMP_10565
 revert(uint256,uint256)(0,0)
TMP_10566(None) = SOLIDITY_CALL revert(uint256,uint256)(0,0)
 xx_rpow_asm_0 = x * x
TMP_10567(uint256) = x_2 * x_2
xx_rpow_asm_0_1(uint256) := TMP_10567(uint256)
 xxRound_rpow_asm_0 = xx_rpow_asm_0 + half_rpow_asm_0
TMP_10568(uint256) = xx_rpow_asm_0_1 + half_rpow_asm_0_1
xxRound_rpow_asm_0_1(uint256) := TMP_10568(uint256)
 xxRound_rpow_asm_0 < xx_rpow_asm_0
TMP_10569(bool) = xxRound_rpow_asm_0_1 < xx_rpow_asm_0_1
CONDITION TMP_10569
 revert(uint256,uint256)(0,0)
TMP_10570(None) = SOLIDITY_CALL revert(uint256,uint256)(0,0)
 x = xxRound_rpow_asm_0 / scalar
TMP_10571(uint256) = xxRound_rpow_asm_0_1 / scalar_1
x_3(uint256) := TMP_10571(uint256)
 n % 2
TMP_10572(uint256) = n_3 % 2
CONDITION TMP_10572
z_5(uint256) := phi(['z_0', 'z_4'])
 zx_rpow_asm_0 = z * x
TMP_10573(uint256) = z_3 * x_3
zx_rpow_asm_0_1(uint256) := TMP_10573(uint256)
 ! zx_rpow_asm_0 / x == z
TMP_10574(uint256) = zx_rpow_asm_0_1 / x_3
TMP_10575(bool) = TMP_10574 == z_3
TMP_10576 = UnaryType.BANG TMP_10575 
CONDITION TMP_10576
 ! ! x
TMP_10577 = UnaryType.BANG x_3 
TMP_10578 = UnaryType.BANG TMP_10577 
CONDITION TMP_10578
 revert(uint256,uint256)(0,0)
TMP_10579(None) = SOLIDITY_CALL revert(uint256,uint256)(0,0)
 zxRound_rpow_asm_0 = zx_rpow_asm_0 + half_rpow_asm_0
TMP_10580(uint256) = zx_rpow_asm_0_1 + half_rpow_asm_0_1
zxRound_rpow_asm_0_1(uint256) := TMP_10580(uint256)
 zxRound_rpow_asm_0 < zx_rpow_asm_0
TMP_10581(bool) = zxRound_rpow_asm_0_1 < zx_rpow_asm_0_1
CONDITION TMP_10581
 revert(uint256,uint256)(0,0)
TMP_10582(None) = SOLIDITY_CALL revert(uint256,uint256)(0,0)
 z = zxRound_rpow_asm_0 / scalar
TMP_10583(uint256) = zxRound_rpow_asm_0_1 / scalar_1
z_4(uint256) := TMP_10583(uint256)
 n = n >> 1
TMP_10584(uint256) = n_3 >> 1
n_4(uint256) := TMP_10584(uint256)
 z
RETURN z_8
```
#### FixedPointMathLib.sqrt(uint256) [INTERNAL]
```slithir
 y_sqrt_asm_0 = x
y_sqrt_asm_0_1(uint256) := x_1(uint256)
 z = 181
z_1(uint256) := 181(uint256)
 ! y_sqrt_asm_0 < 0x10000000000000000000000000000000000
TMP_10585(bool) = y_sqrt_asm_0_1 < 87112285931760246646623899502532662132736
TMP_10586 = UnaryType.BANG TMP_10585 
CONDITION TMP_10586
z_3(uint256) := phi(['z_1', 'z_2'])
y_sqrt_asm_0_3(uint256) := phi(['y_sqrt_asm_0_2', 'y_sqrt_asm_0_1'])
 y_sqrt_asm_0 = y_sqrt_asm_0 >> 128
TMP_10587(uint256) = y_sqrt_asm_0_1 >> 128
y_sqrt_asm_0_2(uint256) := TMP_10587(uint256)
 z = z << 64
TMP_10588(uint256) = z_1 << 64
z_2(uint256) := TMP_10588(uint256)
 ! y_sqrt_asm_0 < 0x1000000000000000000
TMP_10589(bool) = y_sqrt_asm_0_3 < 4722366482869645213696
TMP_10590 = UnaryType.BANG TMP_10589 
CONDITION TMP_10590
z_5(uint256) := phi(['z_1', 'z_4'])
y_sqrt_asm_0_5(uint256) := phi(['y_sqrt_asm_0_4', 'y_sqrt_asm_0_1'])
 y_sqrt_asm_0 = y_sqrt_asm_0 >> 64
TMP_10591(uint256) = y_sqrt_asm_0_3 >> 64
y_sqrt_asm_0_4(uint256) := TMP_10591(uint256)
 z = z << 32
TMP_10592(uint256) = z_3 << 32
z_4(uint256) := TMP_10592(uint256)
 ! y_sqrt_asm_0 < 0x10000000000
TMP_10593(bool) = y_sqrt_asm_0_5 < 1099511627776
TMP_10594 = UnaryType.BANG TMP_10593 
CONDITION TMP_10594
z_7(uint256) := phi(['z_1', 'z_6'])
y_sqrt_asm_0_7(uint256) := phi(['y_sqrt_asm_0_6', 'y_sqrt_asm_0_1'])
 y_sqrt_asm_0 = y_sqrt_asm_0 >> 32
TMP_10595(uint256) = y_sqrt_asm_0_5 >> 32
y_sqrt_asm_0_6(uint256) := TMP_10595(uint256)
 z = z << 16
TMP_10596(uint256) = z_5 << 16
z_6(uint256) := TMP_10596(uint256)
 ! y_sqrt_asm_0 < 0x1000000
TMP_10597(bool) = y_sqrt_asm_0_7 < 16777216
TMP_10598 = UnaryType.BANG TMP_10597 
CONDITION TMP_10598
z_9(uint256) := phi(['z_1', 'z_8'])
y_sqrt_asm_0_9(uint256) := phi(['y_sqrt_asm_0_8', 'y_sqrt_asm_0_1'])
 y_sqrt_asm_0 = y_sqrt_asm_0 >> 16
TMP_10599(uint256) = y_sqrt_asm_0_7 >> 16
y_sqrt_asm_0_8(uint256) := TMP_10599(uint256)
 z = z << 8
TMP_10600(uint256) = z_7 << 8
z_8(uint256) := TMP_10600(uint256)
 z = z * y_sqrt_asm_0 + 65536 >> 18
TMP_10601(uint256) = y_sqrt_asm_0_9 + 65536
TMP_10602(uint256) = z_9 * TMP_10601
TMP_10603(uint256) = TMP_10602 >> 18
z_10(uint256) := TMP_10603(uint256)
 z = z + x / z >> 1
TMP_10604(uint256) = x_1 / z_10
TMP_10605(uint256) = z_10 + TMP_10604
TMP_10606(uint256) = TMP_10605 >> 1
z_11(uint256) := TMP_10606(uint256)
 z = z + x / z >> 1
TMP_10607(uint256) = x_1 / z_11
TMP_10608(uint256) = z_11 + TMP_10607
TMP_10609(uint256) = TMP_10608 >> 1
z_12(uint256) := TMP_10609(uint256)
 z = z + x / z >> 1
TMP_10610(uint256) = x_1 / z_12
TMP_10611(uint256) = z_12 + TMP_10610
TMP_10612(uint256) = TMP_10611 >> 1
z_13(uint256) := TMP_10612(uint256)
 z = z + x / z >> 1
TMP_10613(uint256) = x_1 / z_13
TMP_10614(uint256) = z_13 + TMP_10613
TMP_10615(uint256) = TMP_10614 >> 1
z_14(uint256) := TMP_10615(uint256)
 z = z + x / z >> 1
TMP_10616(uint256) = x_1 / z_14
TMP_10617(uint256) = z_14 + TMP_10616
TMP_10618(uint256) = TMP_10617 >> 1
z_15(uint256) := TMP_10618(uint256)
 z = z + x / z >> 1
TMP_10619(uint256) = x_1 / z_15
TMP_10620(uint256) = z_15 + TMP_10619
TMP_10621(uint256) = TMP_10620 >> 1
z_16(uint256) := TMP_10621(uint256)
 z = z + x / z >> 1
TMP_10622(uint256) = x_1 / z_16
TMP_10623(uint256) = z_16 + TMP_10622
TMP_10624(uint256) = TMP_10623 >> 1
z_17(uint256) := TMP_10624(uint256)
 z = z - x / z < z
TMP_10625(uint256) = x_1 / z_17
TMP_10626(bool) = TMP_10625 < z_17
TMP_10627(uint256) = z_17 - TMP_10626
z_18(uint256) := TMP_10627(uint256)
 z
RETURN z_18
```
