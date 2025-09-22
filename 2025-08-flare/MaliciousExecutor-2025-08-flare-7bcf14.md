### Storage layout (MaliciousExecutor) 

```text
tempProof IReferencedPaymentNonexistence.Proof
tempRequestId uint256
hit uint256
trigger uint256

```

#### MaliciousExecutor.constructor(address) [PUBLIC]
```slithir
 diamond = _diamond
diamond_1(address) := _diamond_1(address)
```
#### MaliciousExecutor.defaulting(IReferencedPaymentNonexistence.Proof,uint256,uint256) [EXTERNAL]
```slithir
diamond_2(address) := phi(['diamond_1', 'diamond_5', 'diamond_3', 'diamond_0'])
 tempProof = _proof
tempProof_1(IReferencedPaymentNonexistence.Proof) := _proof_1(IReferencedPaymentNonexistence.Proof)
 tempRequestId = _redemptionRequestId
tempRequestId_1(uint256) := _redemptionRequestId_1(uint256)
 trigger = _trigger
trigger_1(uint256) := _trigger_1(uint256)
 IAssetManager(diamond).redemptionPaymentDefault(_proof,_redemptionRequestId)
TMP_5488 = CONVERT diamond_2 to IAssetManager
HIGH_LEVEL_CALL, dest:TMP_5488(IAssetManager), function:redemptionPaymentDefault, arguments:['_proof_1', '_redemptionRequestId_1']  
diamond_3(address) := phi(['diamond_1', 'diamond_2', 'diamond_3', 'diamond_5'])
```
#### MaliciousExecutor.fallback() [EXTERNAL]
```slithir
diamond_4(address) := phi(['diamond_1', 'diamond_5', 'diamond_3', 'diamond_0'])
tempProof_2(IReferencedPaymentNonexistence.Proof) := phi(['tempProof_1', 'tempProof_3', 'tempProof_0'])
tempRequestId_2(uint256) := phi(['tempRequestId_0', 'tempRequestId_3', 'tempRequestId_1'])
hit_1(uint256) := phi(['hit_0', 'hit_3'])
trigger_2(uint256) := phi(['trigger_0', 'trigger_1'])
 hit == 0 && trigger == 1
TMP_5492(bool) = hit_1 == 0
TMP_5493(bool) = trigger_2 == 1
TMP_5494(bool) = TMP_5492 && TMP_5493
CONDITION TMP_5494
 hit = 1
hit_2(uint256) := 1(uint256)
 IAssetManager(diamond).redemptionPaymentDefault(tempProof,tempRequestId)
TMP_5495 = CONVERT diamond_4 to IAssetManager
HIGH_LEVEL_CALL, dest:TMP_5495(IAssetManager), function:redemptionPaymentDefault, arguments:['tempProof_2', 'tempRequestId_2']  
diamond_5(address) := phi(['diamond_1', 'diamond_4', 'diamond_3', 'diamond_5'])
tempProof_3(IReferencedPaymentNonexistence.Proof) := phi(['tempProof_2', 'tempProof_1', 'tempProof_3'])
tempRequestId_3(uint256) := phi(['tempRequestId_3', 'tempRequestId_1', 'tempRequestId_2'])
 hit = 0
hit_3(uint256) := 0(uint256)
```
#### MaliciousExecutor.howMuchIsMyNativeBalance() [EXTERNAL]
```slithir
 address(this).balance
TMP_5490 = CONVERT this to address
TMP_5491(uint256) = SOLIDITY_CALL balance(address)(TMP_5490)
RETURN TMP_5491
```
#### MaliciousDistributionToDelegators.slitherConstructorVariables() [INTERNAL]
```slithir
 amount = 0
```
#### IAssetManager.redemptionPaymentDefault(IReferencedPaymentNonexistence.Proof,uint256) [EXTERNAL]
```slithir

```
