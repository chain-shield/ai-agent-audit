
#### PaymentReference.announcedWithdrawal(uint256) [INTERNAL]
```slithir
MAX_ID_3(uint256) := phi(['MAX_ID_0'])
ANNOUNCED_WITHDRAWAL_1(uint256) := phi(['ANNOUNCED_WITHDRAWAL_0'])
 assert(bool)(_id <= MAX_ID)
TMP_5353(bool) = _id_1 <= MAX_ID_3
TMP_5354(None) = SOLIDITY_CALL assert(bool)(TMP_5353)
 bytes32(_id | ANNOUNCED_WITHDRAWAL)
TMP_5355(uint256) = _id_1 | ANNOUNCED_WITHDRAWAL_1
TMP_5356 = CONVERT TMP_5355 to bytes32
RETURN TMP_5356
```
#### PaymentReference.decodeId(bytes32) [INTERNAL]
```slithir
LOW_BITS_MASK_2(uint256) := phi(['LOW_BITS_MASK_0'])
 uint256(_reference) & LOW_BITS_MASK
TMP_5380 = CONVERT _reference_1 to uint256
TMP_5381(uint256) = TMP_5380 & LOW_BITS_MASK_2
RETURN TMP_5381
```
#### PaymentReference.isValid(bytes32,uint256) [INTERNAL]
```slithir
TYPE_MASK_1(uint256) := phi(['TYPE_MASK_0'])
LOW_BITS_MASK_1(uint256) := phi(['LOW_BITS_MASK_0'])
 refType = uint256(_reference) & TYPE_MASK
TMP_5373 = CONVERT _reference_1 to uint256
TMP_5374(uint256) = TMP_5373 & TYPE_MASK_1
refType_1(uint256) := TMP_5374(uint256)
 refLowBits = uint256(_reference) & LOW_BITS_MASK
TMP_5375 = CONVERT _reference_1 to uint256
TMP_5376(uint256) = TMP_5375 & LOW_BITS_MASK_1
refLowBits_1(uint256) := TMP_5376(uint256)
 refType == _type && refLowBits != 0
TMP_5377(bool) = refType_1 == _type_1
TMP_5378(bool) = refLowBits_1 != 0
TMP_5379(bool) = TMP_5377 && TMP_5378
RETURN TMP_5379
```
#### PaymentReference.minting(uint256) [INTERNAL]
```slithir
MAX_ID_1(uint256) := phi(['MAX_ID_0'])
MINTING_1(uint256) := phi(['MINTING_0'])
 assert(bool)(_id <= MAX_ID)
TMP_5345(bool) = _id_1 <= MAX_ID_1
TMP_5346(None) = SOLIDITY_CALL assert(bool)(TMP_5345)
 bytes32(_id | MINTING)
TMP_5347(uint256) = _id_1 | MINTING_1
TMP_5348 = CONVERT TMP_5347 to bytes32
RETURN TMP_5348
```
#### PaymentReference.randomizedIdSkip() [INTERNAL]
```slithir
ID_RANDOMIZATION_1(uint256) := phi(['ID_RANDOMIZATION_0'])
 uint64(block.number % ID_RANDOMIZATION + 1)
TMP_5382(uint256) = block.number % ID_RANDOMIZATION_1
TMP_5383(uint256) = TMP_5382 (c)+ 1
TMP_5384 = CONVERT TMP_5383 to uint64
RETURN TMP_5384
```
#### PaymentReference.redemption(uint256) [INTERNAL]
```slithir
MAX_ID_2(uint256) := phi(['MAX_ID_0'])
REDEMPTION_1(uint256) := phi(['REDEMPTION_0'])
 assert(bool)(_id <= MAX_ID)
TMP_5349(bool) = _id_1 <= MAX_ID_2
TMP_5350(None) = SOLIDITY_CALL assert(bool)(TMP_5349)
 bytes32(_id | REDEMPTION)
TMP_5351(uint256) = _id_1 | REDEMPTION_1
TMP_5352 = CONVERT TMP_5351 to bytes32
RETURN TMP_5352
```
#### PaymentReference.redemptionFromCoreVault(uint256) [INTERNAL]
```slithir
MAX_ID_5(uint256) := phi(['MAX_ID_0'])
REDEMPTION_FROM_CORE_VAULT_1(uint256) := phi(['REDEMPTION_FROM_CORE_VAULT_0'])
 assert(bool)(_id <= MAX_ID)
TMP_5361(bool) = _id_1 <= MAX_ID_5
TMP_5362(None) = SOLIDITY_CALL assert(bool)(TMP_5361)
 bytes32(_id | REDEMPTION_FROM_CORE_VAULT)
TMP_5363(uint256) = _id_1 | REDEMPTION_FROM_CORE_VAULT_1
TMP_5364 = CONVERT TMP_5363 to bytes32
RETURN TMP_5364
```
#### PaymentReference.returnFromCoreVault(uint256) [INTERNAL]
```slithir
MAX_ID_4(uint256) := phi(['MAX_ID_0'])
RETURN_FROM_CORE_VAULT_1(uint256) := phi(['RETURN_FROM_CORE_VAULT_0'])
 assert(bool)(_id <= MAX_ID)
TMP_5357(bool) = _id_1 <= MAX_ID_4
TMP_5358(None) = SOLIDITY_CALL assert(bool)(TMP_5357)
 bytes32(_id | RETURN_FROM_CORE_VAULT)
TMP_5359(uint256) = _id_1 | RETURN_FROM_CORE_VAULT_1
TMP_5360 = CONVERT TMP_5359 to bytes32
RETURN TMP_5360
```
#### PaymentReference.selfMint(address) [INTERNAL]
```slithir
SELF_MINT_1(uint256) := phi(['SELF_MINT_0'])
 bytes32(uint256(uint160(_agentVault)) | SELF_MINT)
TMP_5369 = CONVERT _agentVault_1 to uint160
TMP_5370 = CONVERT TMP_5369 to uint256
TMP_5371(uint256) = TMP_5370 | SELF_MINT_1
TMP_5372 = CONVERT TMP_5371 to bytes32
RETURN TMP_5372
```

#### PaymentReference.topup(address) [INTERNAL]
```slithir
TOPUP_1(uint256) := phi(['TOPUP_0'])
 bytes32(uint256(uint160(_agentVault)) | TOPUP)
TMP_5365 = CONVERT _agentVault_1 to uint160
TMP_5366 = CONVERT TMP_5365 to uint256
TMP_5367(uint256) = TMP_5366 | TOPUP_1
TMP_5368 = CONVERT TMP_5367 to bytes32
RETURN TMP_5368
```
