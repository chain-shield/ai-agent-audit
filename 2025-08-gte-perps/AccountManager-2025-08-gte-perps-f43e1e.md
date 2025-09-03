





































#### EventNonceLib.inc() [INTERNAL]
```slithir
 ds = getEventNonceStorage()
TMP_9860(EventNonceStorage) = INTERNAL_CALL, EventNonceLib.getEventNonceStorage()()
ds_1 (-> ['TMP_9860'])(EventNonceStorage) := TMP_9860(EventNonceStorage)
 ++ ds.eventNonce
REF_5582(uint256) -> ds_1 (-> ['TMP_9860']).eventNonce
ds_2 (-> ['TMP_9860'])(EventNonceStorage) := phi(["ds_1 (-> ['TMP_9860'])"])
REF_5582(-> ds_2 (-> ['TMP_9860'])) = REF_5582 (c)+ 1
RETURN REF_5582
TMP_9860(EventNonceStorage) := phi(["ds_2 (-> ['TMP_9860'])"])
```
#### AccountManagerStorageLib.getAccountManagerStorage() [INTERNAL]
```slithir
ACCOUNT_MANAGER_STORAGE_POSITION_1(bytes32) := phi(['ACCOUNT_MANAGER_STORAGE_POSITION_0'])
 position = ACCOUNT_MANAGER_STORAGE_POSITION
position_1(bytes32) := ACCOUNT_MANAGER_STORAGE_POSITION_1(bytes32)
 self = position
self_1 (-> ['position'])(AccountManagerStorage) := position_1(bytes32)
 self
RETURN self_1 (-> ['position'])
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
#### FeeDataStorageLib.getFeeDataStorage() [INTERNAL]
```slithir
FEE_DATA_STORAGE_POSITION_1(bytes32) := phi(['FEE_DATA_STORAGE_POSITION_0'])
 position = FEE_DATA_STORAGE_POSITION
position_1(bytes32) := FEE_DATA_STORAGE_POSITION_1(bytes32)
 self = position
self_1 (-> ['position'])(FeeData) := position_1(bytes32)
 self
RETURN self_1 (-> ['position'])
```
#### FeeDataLib.claimFees(FeeData,address) [INTERNAL]
```slithir
 fees = self.unclaimedFees[token]
REF_919(mapping(address => uint256)) -> self_1 (-> []).unclaimedFees
REF_920(uint256) -> REF_919[token_1]
fees_1(uint256) := REF_920(uint256)
 delete self.unclaimedFees[token]
REF_921(mapping(address => uint256)) -> self_1 (-> []).unclaimedFees
REF_922(uint256) -> REF_921[token_1]
REF_921 = delete REF_922 
 FeesClaimed(EventNonceLib.inc(),token,fees)
TMP_1918(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit FeesClaimed(TMP_1918,token_1,fees_1)
 fees
RETURN fees_1
```
#### PackedFeeRatesLib.packFeeRates(uint16[]) [INTERNAL]
```slithir
 fees.length > 15
REF_5310 -> LENGTH fees_1
TMP_9520(bool) = REF_5310 > 15
CONDITION TMP_9520
 revert TooManyFeeTiers()()
TMP_9521(None) = SOLIDITY_CALL revert TooManyFeeTiers()()
 i < fees.length
packedValue_1(uint256) := phi(['packedValue_0', 'packedValue_2'])
i_1(uint256) := phi(['i_0', 'i_2'])
REF_5311 -> LENGTH fees_1
TMP_9522(bool) = i_1 < REF_5311
CONDITION TMP_9522
 packedValue = packedValue | (uint256(fees[i]) << (i * 16))
REF_5312(uint16) -> fees_1[i_1]
TMP_9523 = CONVERT REF_5312 to uint256
TMP_9524(uint256) = i_1 (c)* 16
TMP_9525(uint256) = TMP_9523 << TMP_9524
TMP_9526(uint256) = packedValue_1 | TMP_9525
packedValue_2(uint256) := TMP_9526(uint256)
 i ++
TMP_9527(uint256) := i_1(uint256)
i_2(uint256) = i_1 (c)+ 1
 PackedFeeRates.wrap(packedValue)
TMP_9528 = CONVERT packedValue_1 to PackedFeeRates
RETURN TMP_9528
```
#### SafeTransferLib.safeTransferFrom(address,address,address,uint256) [INTERNAL]
```slithir
 m_safeTransferFrom_asm_0 = mload(uint256)(0x40)
TMP_15255(uint256) = SOLIDITY_CALL mload(uint256)(64)
m_safeTransferFrom_asm_0_1(uint256) := TMP_15255(uint256)
 mstore(uint256,uint256)(0x60,amount)
TMP_15256(None) = SOLIDITY_CALL mstore(uint256,uint256)(96,amount_1)
 mstore(uint256,uint256)(0x40,to)
TMP_15257(None) = SOLIDITY_CALL mstore(uint256,uint256)(64,to_1)
 mstore(uint256,uint256)(0x2c,from << 96)
TMP_15258(address) = from_1 << 96
TMP_15259(None) = SOLIDITY_CALL mstore(uint256,uint256)(44,TMP_15258)
 mstore(uint256,uint256)(0x0c,0x23b872dd000000000000000000000000)
TMP_15260(None) = SOLIDITY_CALL mstore(uint256,uint256)(12,47480692178561195778129796594248187904)
 success_safeTransferFrom_asm_0 = call(uint256,uint256,uint256,uint256,uint256,uint256,uint256)(gas()(),token,0,0x1c,0x64,0x00,0x20)
TMP_15261(uint256) = SOLIDITY_CALL gas()()
TMP_15262(uint256) = SOLIDITY_CALL call(uint256,uint256,uint256,uint256,uint256,uint256,uint256)(TMP_15261,token_1,0,28,100,0,32)
success_safeTransferFrom_asm_0_1(uint256) := TMP_15262(uint256)
 ! mload(uint256)(0x00) == 1 & success_safeTransferFrom_asm_0
TMP_15263(uint256) = SOLIDITY_CALL mload(uint256)(0)
TMP_15264(bool) = TMP_15263 == 1
TMP_15265(bool) = TMP_15264 & success_safeTransferFrom_asm_0_1
TMP_15266 = UnaryType.BANG TMP_15265 
CONDITION TMP_15266
 ! ! extcodesize(uint256)(token) | returndatasize()() < success_safeTransferFrom_asm_0
REF_5783 -> CODESIZE token_1
TMP_15267 = UnaryType.BANG REF_5783 
TMP_15268(uint256) = SOLIDITY_CALL returndatasize()(token_1)
TMP_15269(uint256) = TMP_15267 | TMP_15268
TMP_15270(bool) = TMP_15269 < success_safeTransferFrom_asm_0_1
TMP_15271 = UnaryType.BANG TMP_15270 
CONDITION TMP_15271
 mstore(uint256,uint256)(0x00,0x7939f424)
TMP_15272(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,2033841188)
 revert(uint256,uint256)(0x1c,0x04)
TMP_15273(None) = SOLIDITY_CALL revert(uint256,uint256)(28,4)
 mstore(uint256,uint256)(0x60,0)
TMP_15274(None) = SOLIDITY_CALL mstore(uint256,uint256)(96,0)
 mstore(uint256,uint256)(0x40,m_safeTransferFrom_asm_0)
TMP_15275(None) = SOLIDITY_CALL mstore(uint256,uint256)(64,m_safeTransferFrom_asm_0_1)
```

#### EventNonceLib.getCurrentNonce() [INTERNAL]
```slithir
 ds = getEventNonceStorage()
TMP_9861(EventNonceStorage) = INTERNAL_CALL, EventNonceLib.getEventNonceStorage()()
ds_1 (-> ['TMP_9861'])(EventNonceStorage) := TMP_9861(EventNonceStorage)
 ds.eventNonce
REF_5583(uint256) -> ds_1 (-> ['TMP_9861']).eventNonce
RETURN REF_5583
```
#### FeeDataLib.getAccountFeeTier(FeeData,address) [INTERNAL]
```slithir
 self.accountFeeTier[account]
REF_909(mapping(address => FeeTiers)) -> self_1 (-> []).accountFeeTier
REF_910(FeeTiers) -> REF_909[account_1]
RETURN REF_910
 tier
```
#### PackedFeeRatesLib.getFeeAt(PackedFeeRates,uint256) [INTERNAL]
```slithir
U16_PER_WORD_2(uint256) := phi(['U16_PER_WORD_0'])
 index >= 15
TMP_1890(bool) = index_1 >= 15
CONDITION TMP_1890
 revert FeeTierIndexOutOfBounds()()
TMP_1891(None) = SOLIDITY_CALL revert FeeTierIndexOutOfBounds()()
 shiftBits = index * U16_PER_WORD
TMP_1892(uint256) = index_1 (c)* U16_PER_WORD_2
shiftBits_1(uint256) := TMP_1892(uint256)
 uint16((PackedFeeRates.unwrap(fees) >> shiftBits) & 0xFFFF)
TMP_1893 = CONVERT fees_1 to uint256
TMP_1894(uint256) = TMP_1893 >> shiftBits_1
TMP_1895(uint256) = TMP_1894 & 65535
TMP_1896 = CONVERT TMP_1895 to uint16
RETURN TMP_1896
```
#### FeeDataLib.setAccountFeeTier(FeeData,address,FeeTiers) [INTERNAL]
```slithir
 self.accountFeeTier[account] = feeTier
REF_911(mapping(address => FeeTiers)) -> self_1 (-> []).accountFeeTier
REF_912(FeeTiers) -> REF_911[account_1]
self_2 (-> [])(FeeData) := phi(['self_1 (-> [])'])
REF_912(FeeTiers) (->self_2 (-> [])) := feeTier_1(FeeTiers)
 AccountFeeTierUpdated(EventNonceLib.inc(),account,feeTier)
TMP_1914(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit AccountFeeTierUpdated(TMP_1914,account_1,feeTier_1)
```
#### FeeDataLib.accrueFee(FeeData,address,uint256) [INTERNAL]
```slithir
 self.totalFees[token] += amount
REF_914(mapping(address => uint256)) -> self_1 (-> []).totalFees
REF_915(uint256) -> REF_914[token_1]
self_2 (-> [])(FeeData) := phi(['self_1 (-> [])'])
REF_915(-> self_2 (-> [])) = REF_915 (c)+ amount_1
 self.unclaimedFees[token] += amount
REF_916(mapping(address => uint256)) -> self_2 (-> []).unclaimedFees
REF_917(uint256) -> REF_916[token_1]
self_3 (-> [])(FeeData) := phi(['self_2 (-> [])'])
REF_917(-> self_3 (-> [])) = REF_917 (c)+ amount_1
 FeesAccrued(EventNonceLib.inc(),token,amount)
TMP_1916(uint256) = LIBRARY_CALL, dest:EventNonceLib, function:EventNonceLib.inc(), arguments:[] 
Emit FeesAccrued(TMP_1916,token_1,amount_1)
```
#### FeeDataLib.getMakerFee(FeeData,PackedFeeRates,address,uint256) [INTERNAL]
```slithir
FEE_SCALING_2(uint256) := phi(['FEE_SCALING_0'])
 amount == 0
TMP_1910(bool) = amount_1 == 0
CONDITION TMP_1910
 0
RETURN 0
 feeRate = makerRates.getFeeAt(uint256(self.accountFeeTier[account]))
REF_906(mapping(address => FeeTiers)) -> self_1 (-> []).accountFeeTier
REF_907(FeeTiers) -> REF_906[account_1]
TMP_1911 = CONVERT REF_907 to uint256
TMP_1912(uint16) = LIBRARY_CALL, dest:PackedFeeRatesLib, function:PackedFeeRatesLib.getFeeAt(PackedFeeRates,uint256), arguments:['makerRates_1', 'TMP_1911'] 
feeRate_1(uint16) := TMP_1912(uint16)
 amount.fullMulDiv(feeRate,FEE_SCALING)
TMP_1913(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['amount_1', 'feeRate_1', 'FEE_SCALING_2'] 
RETURN TMP_1913
```
#### FeeDataLib.getTakerFee(FeeData,PackedFeeRates,address,uint256) [INTERNAL]
```slithir
FEE_SCALING_1(uint256) := phi(['FEE_SCALING_0'])
 amount == 0
TMP_1906(bool) = amount_1 == 0
CONDITION TMP_1906
 0
RETURN 0
 feeRate = takerRates.getFeeAt(uint256(self.accountFeeTier[account]))
REF_902(mapping(address => FeeTiers)) -> self_1 (-> []).accountFeeTier
REF_903(FeeTiers) -> REF_902[account_1]
TMP_1907 = CONVERT REF_903 to uint256
TMP_1908(uint16) = LIBRARY_CALL, dest:PackedFeeRatesLib, function:PackedFeeRatesLib.getFeeAt(PackedFeeRates,uint256), arguments:['takerRates_1', 'TMP_1907'] 
feeRate_1(uint16) := TMP_1908(uint16)
 amount.fullMulDiv(feeRate,FEE_SCALING)
TMP_1909(uint256) = LIBRARY_CALL, dest:FixedPointMathLib, function:FixedPointMathLib.fullMulDiv(uint256,uint256,uint256), arguments:['amount_1', 'feeRate_1', 'FEE_SCALING_1'] 
RETURN TMP_1909
```
#### EventNonceLib.getEventNonceStorage() [INTERNAL]
```slithir
EVENT_NONCE_STORAGE_POSITION_1(bytes32) := phi(['EVENT_NONCE_STORAGE_POSITION_0'])
 position = EVENT_NONCE_STORAGE_POSITION
position_1(bytes32) := EVENT_NONCE_STORAGE_POSITION_1(bytes32)
 ds = position
ds_1 (-> ['position'])(EventNonceStorage) := position_1(bytes32)
 ds
RETURN ds_1 (-> ['position'])
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
